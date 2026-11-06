// U3 scout probe (throwaway; rust-reviewer, 2026-11-06): timer lateness for ImpairSocket's release mechanism.
// Built from .gate/netcode-proof-test/timer-probe/Cargo.toml (tokio =1.54.1, libc =0.2.191, the workspace's pins),
// offline. Not part of the spike workspace.
//
// Usage: timer-probe <group> <n>   group: sleepers | spinners
//   sleepers: (a) tokio sleep_until on a 4-worker multi-thread runtime with the I/O driver on (enable_all),
//             (b) std::thread::sleep on a dedicated thread (default timer slack),
//             (b1) the same with this thread's timer slack set to 1 ns (prctl PR_SET_TIMERSLACK)
//   spinners: (c) std::thread::sleep to 100 us before the due time, then a spin on Instant::now(),
//             (c2) the same with a 1 ms lead (added: the VM's sleep lateness can exceed 100 us),
//             (d) (c) on a dedicated release thread, which then wakes a task on the multi-thread runtime through
//                 tokio::sync::Notify; lateness measured inside the task after it resumes (the quinn poll_recv path)
// Lateness = Instant::now() at wake - due instant, in ns. Each wait's due = Instant::now() + delay at the wait's start.
// CPU = getrusage(RUSAGE_SELF) user+sys delta over the cell's wall time (cores).
//   stream <rate> <lead_us> <slack_ns> <seconds>: the ImpairSocket shape. A producer thread pushes due times (Poisson
//             arrivals at <rate>/s, each due 25-75 ms later: P2's one-way 50 ms +- 25 ms span, wider than P2's +- 10 ms
//             on purpose) into a Mutex<BinaryHeap>; one release thread sleeps (timer slack <slack_ns>) to
//             due - <lead_us>, spins to due, releases every due entry; lateness per release, and the release
//             thread's own CPU (getrusage RUSAGE_THREAD) over the run.

use std::sync::Arc;
use std::sync::mpsc;
use std::time::{Duration, Instant};

const DELAYS_US: [u64; 6] = [100, 500, 1_000, 2_000, 10_000, 50_000];
const LEAD: Duration = Duration::from_micros(100);
const LEAD_1MS: Duration = Duration::from_micros(1_000);

fn cpu_ns() -> u128 {
    // SAFETY: getrusage writes into a zeroed, correctly sized struct; probe code only.
    let mut ru: libc::rusage = unsafe { std::mem::zeroed() };
    unsafe { libc::getrusage(libc::RUSAGE_SELF, &mut ru) };
    let t = |tv: libc::timeval| tv.tv_sec as u128 * 1_000_000_000 + tv.tv_usec as u128 * 1_000;
    t(ru.ru_utime) + t(ru.ru_stime)
}

fn report(mech: &str, delay_us: u64, mut late: Vec<i64>, wall: Duration, cpu: u128) {
    late.sort_unstable();
    let n = late.len();
    let q = |p: f64| -> f64 {
        let idx = ((p * n as f64).ceil() as usize).clamp(1, n) - 1;
        late[idx] as f64 / 1_000.0
    };
    let min = late[0] as f64 / 1_000.0;
    let max = late[n - 1] as f64 / 1_000.0;
    let over250 = late.iter().filter(|&&x| x > 250_000).count();
    let cpu_frac = cpu as f64 / wall.as_nanos() as f64;
    println!(
        "{mech}\tdelay_ms={:.1}\tn={n}\tmin_us={min:.1}\tp50_us={:.1}\tp99_us={:.1}\tp99.9_us={:.1}\tmax_us={max:.1}\tover_250us={over250}\tcpu_cores={cpu_frac:.3}",
        delay_us as f64 / 1_000.0,
        q(0.50),
        q(0.99),
        q(0.999)
    );
}

fn spin_until(due: Instant) {
    spin_until_lead(due, LEAD)
}

fn spin_until_lead(due: Instant, lead: Duration) {
    if let Some(coarse) = due.checked_sub(lead) {
        let now = Instant::now();
        if coarse > now {
            std::thread::sleep(coarse - now);
        }
    }
    while Instant::now() < due {
        std::hint::spin_loop();
    }
}

fn late_ns(due: Instant) -> i64 {
    let now = Instant::now();
    if now >= due { (now - due).as_nanos() as i64 } else { -((due - now).as_nanos() as i64) }
}

fn runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_multi_thread().worker_threads(4).enable_all().build().unwrap()
}

fn sleepers(n: usize) {
    // (a) tokio
    let rt = runtime();
    for &d in &DELAYS_US {
        let delay = Duration::from_micros(d);
        let (c0, w0) = (cpu_ns(), Instant::now());
        let late = rt.block_on(async move {
            tokio::spawn(async move {
                let mut v = Vec::with_capacity(n);
                for _ in 0..n {
                    let due = Instant::now() + delay;
                    tokio::time::sleep_until(tokio::time::Instant::from_std(due)).await;
                    v.push(late_ns(due));
                }
                v
            })
            .await
            .unwrap()
        });
        report("a_tokio_sleep_until", d, late, w0.elapsed(), cpu_ns() - c0);
    }
    drop(rt);
    // (b) and (b1) std::thread::sleep on a dedicated thread
    for (mech, slack) in [("b_thread_sleep_slack_default", None), ("b1_thread_sleep_slack_1ns", Some(1u64))] {
        std::thread::spawn(move || {
            if let Some(s) = slack {
                // SAFETY: prctl with PR_SET_TIMERSLACK takes an integer argument; probe code only.
                let r = unsafe { libc::prctl(libc::PR_SET_TIMERSLACK, s as libc::c_ulong, 0, 0, 0) };
                assert_eq!(r, 0);
            }
            // SAFETY: PR_GET_TIMERSLACK takes no pointer arguments; probe code only.
            let slack_now = unsafe { libc::prctl(libc::PR_GET_TIMERSLACK, 0, 0, 0, 0) };
            println!("# {mech}: PR_GET_TIMERSLACK = {slack_now} ns");
            for &d in &DELAYS_US {
                let delay = Duration::from_micros(d);
                let (c0, w0) = (cpu_ns(), Instant::now());
                let mut v = Vec::with_capacity(n);
                for _ in 0..n {
                    let due = Instant::now() + delay;
                    std::thread::sleep(due - Instant::now());
                    v.push(late_ns(due));
                }
                report(mech, d, v, w0.elapsed(), cpu_ns() - c0);
            }
        })
        .join()
        .unwrap();
    }
}

fn spinners(n: usize) {
    // (c) sleep to due - 100 us, then spin; (c2) the same with a 1 ms lead
    for (mech, lead) in [("c_sleep_then_spin_100us", LEAD), ("c2_sleep_then_spin_1ms", LEAD_1MS)] {
        std::thread::spawn(move || {
            for &d in &DELAYS_US {
                let delay = Duration::from_micros(d);
                let (c0, w0) = (cpu_ns(), Instant::now());
                let mut v = Vec::with_capacity(n);
                for _ in 0..n {
                    let due = Instant::now() + delay;
                    spin_until_lead(due, lead);
                    v.push(late_ns(due));
                }
                report(mech, d, v, w0.elapsed(), cpu_ns() - c0);
            }
        })
        .join()
        .unwrap();
    }
    // (d) release thread (c) + Notify -> task on the multi-thread runtime
    let rt = runtime();
    let notify = Arc::new(tokio::sync::Notify::new());
    let (tx, rx) = mpsc::channel::<Option<Instant>>();
    let n2 = notify.clone();
    let rel = std::thread::spawn(move || {
        while let Ok(Some(due)) = rx.recv() {
            spin_until(due);
            n2.notify_one();
        }
    });
    for &d in &DELAYS_US {
        let delay = Duration::from_micros(d);
        let (c0, w0) = (cpu_ns(), Instant::now());
        let (tx2, nt) = (tx.clone(), notify.clone());
        let late = rt.block_on(async move {
            tokio::spawn(async move {
                let mut v = Vec::with_capacity(n);
                for _ in 0..n {
                    let due = Instant::now() + delay;
                    let fut = nt.notified();
                    tx2.send(Some(due)).unwrap();
                    fut.await;
                    v.push(late_ns(due));
                }
                v
            })
            .await
            .unwrap()
        });
        report("d_release_thread_spin_then_notify_task", d, late, w0.elapsed(), cpu_ns() - c0);
    }
    tx.send(None).unwrap();
    rel.join().unwrap();
}

fn thread_cpu_ns() -> u128 {
    // SAFETY: as cpu_ns, for the calling thread only.
    let mut ru: libc::rusage = unsafe { std::mem::zeroed() };
    unsafe { libc::getrusage(libc::RUSAGE_THREAD, &mut ru) };
    let t = |tv: libc::timeval| tv.tv_sec as u128 * 1_000_000_000 + tv.tv_usec as u128 * 1_000;
    t(ru.ru_utime) + t(ru.ru_stime)
}

fn stream(rate: f64, lead_us: u64, slack_ns: u64, secs: u64) {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    use std::sync::{Condvar, Mutex};
    let shared = Arc::new((Mutex::new((BinaryHeap::<Reverse<Instant>>::new(), false)), Condvar::new()));
    let lead = Duration::from_micros(lead_us);
    let s2 = shared.clone();
    let rel = std::thread::spawn(move || {
        // SAFETY: PR_SET_TIMERSLACK takes an integer argument; probe code only.
        assert_eq!(unsafe { libc::prctl(libc::PR_SET_TIMERSLACK, slack_ns as libc::c_ulong, 0, 0, 0) }, 0);
        let (c0, w0) = (thread_cpu_ns(), Instant::now());
        let mut late: Vec<i64> = Vec::with_capacity((rate * secs as f64 * 1.2) as usize);
        let (m, cv) = &*s2;
        loop {
            let mut g = m.lock().unwrap();
            let head = loop {
                if let Some(Reverse(h)) = g.0.peek() { break Some(*h); }
                if g.1 { break None; }
                g = cv.wait(g).unwrap();
            };
            let Some(due) = head else { break };
            let now = Instant::now();
            if due > now + lead {
                // sleep until due - lead, or until a new earlier entry arrives
                let (g2, _) = cv.wait_timeout(g, due - lead - now).unwrap();
                drop(g2);
                continue;
            }
            drop(g);
            while Instant::now() < due { std::hint::spin_loop(); }
            let mut g = m.lock().unwrap();
            let now = Instant::now();
            while let Some(Reverse(h)) = g.0.peek() {
                if *h > now { break; }
                late.push(late_ns(*h));
                g.0.pop();
            }
        }
        let cpu = thread_cpu_ns() - c0;
        report(&format!("stream_rate{rate}_lead{lead_us}us_slack{slack_ns}ns_release_thread"), 0, late, w0.elapsed(), cpu);
    });
    // producer: Poisson arrivals, due = arrival + 25..75 ms, from a fixed xorshift seed
    let mut x: u64 = 0x9E3779B97F4A7C15;
    let mut rnd = || { x ^= x << 13; x ^= x >> 7; x ^= x << 17; (x >> 11) as f64 / (1u64 << 53) as f64 };
    let end = Instant::now() + Duration::from_secs(secs);
    let mut t = Instant::now();
    let (m, cv) = &*shared;
    while t < end {
        let gap = -(1.0 - rnd()).ln() / rate;
        t += Duration::from_secs_f64(gap);
        let now = Instant::now();
        if t > now { std::thread::sleep(t - now); }
        let due = Instant::now() + Duration::from_micros(25_000 + (rnd() * 50_000.0) as u64);
        let mut g = m.lock().unwrap();
        let earlier = g.0.peek().is_none_or(|Reverse(h)| due < *h);
        g.0.push(Reverse(due));
        drop(g);
        if earlier { cv.notify_one(); }
    }
    std::thread::sleep(Duration::from_millis(100));
    { let mut g = m.lock().unwrap(); g.1 = true; }
    cv.notify_one();
    rel.join().unwrap();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n: usize = args.get(2).and_then(|s| s.parse::<f64>().ok()).map(|v| v as usize).unwrap_or(10_000);
    let mut res = libc::timespec { tv_sec: 0, tv_nsec: 0 };
    // SAFETY: clock_getres writes into a valid timespec; probe code only.
    unsafe { libc::clock_getres(libc::CLOCK_MONOTONIC, &mut res) };
    println!("# clock_getres(CLOCK_MONOTONIC) = {} s {} ns", res.tv_sec, res.tv_nsec);
    println!(
        "# process timerslack_ns = {}",
        std::fs::read_to_string("/proc/self/timerslack_ns").unwrap_or_default().trim()
    );
    match args.get(1).map(String::as_str) {
        Some("sleepers") => sleepers(n),
        Some("spinners") => spinners(n),
        Some("stream") => {
            let a = |i: usize, d: f64| args.get(i).and_then(|s| s.parse::<f64>().ok()).unwrap_or(d);
            stream(a(2, 3000.0), a(3, 100.0) as u64, a(4, 1.0) as u64, a(5, 60.0) as u64)
        }
        _ => eprintln!("usage: timer-probe sleepers|spinners [n]"),
    }
}
