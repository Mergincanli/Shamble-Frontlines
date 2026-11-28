// Throwaway probe for the U3 verifier (netcode-proof-test). std only; never shipped.
// Measures a dedicated thread's release lateness (wake time - due time) with Condvar::wait_timeout, in modes:
//   default slack (50 us), slack 1 ns written to /proc/<tid>/timerslack_ns by the thread itself, and each with a
//   100 us spin tail. Two schedules: "sparse" (one due time 1-3 ms ahead, as the scout's bench) and "stream"
//   (10,000 due times 100 us apart plus uniform +-50 us jitter, sorted, as a 10 kpps release queue would see).
// Run under `chrt -f 50` from outside to test SCHED_FIFO without unsafe code.
use std::sync::{Condvar, Mutex};
use std::time::{Duration, Instant};

fn tid() -> Option<String> {
    let l = std::fs::read_link("/proc/thread-self").ok()?;
    l.file_name().map(|s| s.to_string_lossy().into_owned())
}

fn set_slack(ns: u64) -> String {
    let Some(t) = tid() else { return "no tid".into() };
    let p = format!("/proc/{t}/timerslack_ns");
    let w = std::fs::write(&p, ns.to_string());
    let r = std::fs::read_to_string(&p).ok();
    format!("write {ns} -> {:?}, read {:?}", w.map_err(|e| e.to_string()), r.map(|s| s.trim().to_string()))
}

fn wait_until(m: &Mutex<()>, cv: &Condvar, due: Instant, spin: Duration) {
    let mut g = m.lock().unwrap();
    loop {
        let now = Instant::now();
        if now >= due {
            return;
        }
        let left = due - now;
        if left <= spin {
            drop(g);
            while Instant::now() < due {
                std::hint::spin_loop();
            }
            return;
        }
        g = cv.wait_timeout(g, left - spin).unwrap().0;
    }
}

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.0 >> 33
    }
}

fn stats(name: &str, v: &mut [u64]) {
    v.sort_unstable();
    let n = v.len();
    let q = |p: f64| v[(((n - 1) as f64) * p).round() as usize] as f64 / 1000.0;
    let over = v.iter().filter(|&&x| x > 250_000).count();
    println!(
        "{name}: n={n} p50={:.1}us p90={:.1}us p99={:.1}us p99.9={:.1}us max={:.1}us share>250us={:.3}%",
        q(0.5), q(0.9), q(0.99), q(0.999), q(1.0), 100.0 * over as f64 / n as f64
    );
}

fn run(slack: Option<u64>, spin_us: u64, label: &str) {
    let label = label.to_string();
    std::thread::Builder::new()
        .name("verify-clock".into())
        .spawn(move || {
            let s = match slack {
                Some(ns) => set_slack(ns),
                None => "default".into(),
            };
            let m = Mutex::new(());
            let cv = Condvar::new();
            let spin = Duration::from_micros(spin_us);
            // sparse
            let mut sp = Vec::with_capacity(2000);
            for i in 0..2000u64 {
                let due = Instant::now() + Duration::from_micros(1_000 + (i * 37) % 2000);
                wait_until(&m, &cv, due, spin);
                sp.push(Instant::now().saturating_duration_since(due).as_nanos() as u64);
            }
            // stream: 10,000 due times, 100 us apart, +-50 us uniform jitter, sorted
            let mut rng = Lcg(0x5eed);
            let base = Instant::now() + Duration::from_millis(5);
            let mut dues: Vec<Instant> = (0..10_000u64)
                .map(|i| base + Duration::from_micros(100 * i + 50) - Duration::from_micros(50) + Duration::from_nanos(rng.next() % 100_000))
                .collect();
            dues.sort_unstable();
            let mut st = Vec::with_capacity(dues.len());
            for d in &dues {
                wait_until(&m, &cv, *d, spin);
                st.push(Instant::now().saturating_duration_since(*d).as_nanos() as u64);
            }
            println!("[{label}] slack: {s}; spin tail {spin_us} us");
            stats(&format!("[{label}] sparse"), &mut sp);
            stats(&format!("[{label}] stream 10k/s"), &mut st);
        })
        .unwrap()
        .join()
        .unwrap();
}

fn main() {
    let sched = std::fs::read_to_string("/proc/thread-self/sched").ok().and_then(|s| s.lines().find(|l| l.starts_with("policy")).map(|l| l.to_string()));
    println!("main thread policy line: {sched:?}; /proc/self/timerslack_ns = {:?}", std::fs::read_to_string("/proc/self/timerslack_ns").ok().map(|s| s.trim().to_string()));
    run(None, 0, "default");
    run(Some(1), 0, "slack1");
    run(None, 100, "default+spin100");
    run(Some(1), 100, "slack1+spin100");
    println!("VERIFY PROBE DONE");
}
