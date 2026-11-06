// U3 verifier probe (throwaway, 2026-11-06): judge PM-0001-3's P1 jitter-end checks directly on a release thread,
// instead of judging the release lateness's p99.9. Built from .gate/netcode-proof-test/u3-verify-timing-probe/
// (tokio =1.54.1, libc =0.2.191, offline). Not part of the spike workspace.
//
// Usage: probe m3 <rate_per_dir> <seconds> <lead_us> <slack_ns> <fifo_prio> <send 0|1> <inbound 0|1>
//   A producer thread emits, every 1/rate s, one "up" and one "down" packet (two directions sharing one release
//   thread, like one ImpairSocket carrying the -3 probe both ways). Each packet: arrival = Instant::now() at push,
//   delay = 25 ms + U(-5, +5) ms (P1's per-direction one-way and half jitter), due = arrival + delay.
//   The release thread waits on a Condvar (absolute futex deadline) to due - lead, spins to due, pops every due
//   entry, optionally sends a 64 B UDP datagram on loopback for each (send = 1), and records the delivered delay
//   (release instant - arrival). With inbound = 1 the "up" entries are handed to a task on a 4-worker tokio runtime
//   (Waker-based, like poll_recv) and their delivered delay is taken inside the task.
//   Output per direction: delivered-delay quantiles and PM-0001-3's P1 bands (marks.json:814-880), lateness
//   quantiles, the share above 250 us and E[(lateness - 0.25 ms)+].
// Usage: probe hiccup <seconds>   one thread spinning on Instant::now(), gaps > 20 us.

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Condvar, Mutex};
use std::task::{Context, Poll, Waker};
use std::time::{Duration, Instant};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Ent {
    due: Instant,
    seq: u64,
    arrival: Instant,
    up: bool,
}

fn thread_cpu_ns() -> u128 {
    // SAFETY: getrusage writes into a zeroed, correctly sized struct (probe only).
    let mut ru: libc::rusage = unsafe { std::mem::zeroed() };
    unsafe { libc::getrusage(libc::RUSAGE_THREAD, &mut ru) };
    let t = |tv: libc::timeval| tv.tv_sec as u128 * 1_000_000_000 + tv.tv_usec as u128 * 1_000;
    t(ru.ru_utime) + t(ru.ru_stime)
}

fn q(v: &[i64], p: f64) -> f64 {
    let n = v.len();
    let idx = ((p * n as f64).ceil() as usize).clamp(1, n) - 1;
    v[idx] as f64
}

fn summarize(label: &str, delays: &mut Vec<i64>, lates: &mut Vec<i64>) {
    delays.sort_unstable();
    lates.sort_unstable();
    let n = delays.len();
    let ms = |x: f64| x / 1e6;
    let mean = delays.iter().map(|&x| x as f64).sum::<f64>() / n as f64;
    let (p001, p90, p999) = (q(delays, 0.001), q(delays, 0.90), q(delays, 0.999));
    let ok = |c: bool| if c { "PASS" } else { "FAIL" };
    let mean_ok = (23.75e6..=26.25e6).contains(&mean);
    let lo_ok = (19.75e6..=20.25e6).contains(&p001);
    let hi_ok = (29.75e6..=30.25e6).contains(&p999);
    let p90_ok = (28.75e6..=29.25e6).contains(&p90);
    let excess: f64 = lates.iter().map(|&l| ((l - 250_000).max(0)) as f64).sum::<f64>() / n as f64;
    let over = lates.iter().filter(|&&l| l > 250_000).count();
    println!(
        "{label}\tn={n}\tmean_ms={:.4}({})\tp001_ms={:.4}({})\tp90_ms={:.4}({})\tp999_ms={:.4}({})\tmax_ms={:.3}",
        ms(mean), ok(mean_ok), ms(p001), ok(lo_ok), ms(p90), ok(p90_ok), ms(p999), ok(hi_ok), ms(delays[n - 1] as f64)
    );
    println!(
        "{label}\tlate_us p50={:.1} p99={:.1} p99.9={:.1} max={:.1}\tover_250us={:.3}%\tE[(L-0.25ms)+]={:.2}us (P1 bar ~10 us)",
        q(lates, 0.5) / 1e3, q(lates, 0.99) / 1e3, q(lates, 0.999) / 1e3, lates[n - 1] as f64 / 1e3,
        100.0 * over as f64 / n as f64, excess / 1e3
    );
}

struct Inbound {
    q: Mutex<(Vec<Ent>, Option<Waker>, bool)>,
}

struct RecvFut<'a> {
    ib: &'a Inbound,
}

impl Future for RecvFut<'_> {
    type Output = (Vec<Ent>, bool);
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut g = self.ib.q.lock().unwrap();
        if !g.0.is_empty() || g.2 {
            let v = std::mem::take(&mut g.0);
            let done = g.2;
            Poll::Ready((v, done))
        } else {
            g.1 = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn m3(rate: f64, secs: f64, lead_us: u64, slack_ns: u64, fifo: i32, send: bool, inbound: bool) {
    let lead = Duration::from_micros(lead_us);
    let shared = Arc::new((Mutex::new((BinaryHeap::<Reverse<Ent>>::new(), false)), Condvar::new()));
    let ib = Arc::new(Inbound { q: Mutex::new((Vec::new(), None, false)) });
    let rt = if inbound {
        Some(tokio::runtime::Builder::new_multi_thread().worker_threads(4).enable_all().build().unwrap())
    } else {
        None
    };
    let task = rt.as_ref().map(|rt| {
        let ib2 = ib.clone();
        rt.spawn(async move {
            let mut d = Vec::new();
            let mut l = Vec::new();
            loop {
                let (v, done) = RecvFut { ib: &ib2 }.await;
                let now = Instant::now();
                for e in v {
                    d.push((now - e.arrival).as_nanos() as i64);
                    l.push(if now >= e.due { (now - e.due).as_nanos() as i64 } else { -((e.due - now).as_nanos() as i64) });
                }
                if done {
                    break;
                }
            }
            (d, l)
        })
    });
    let s2 = shared.clone();
    let ib3 = ib.clone();
    let rel = std::thread::Builder::new().name("rel".into()).spawn(move || {
        // SAFETY: integer arguments only (probe).
        assert_eq!(unsafe { libc::prctl(libc::PR_SET_TIMERSLACK, slack_ns as libc::c_ulong, 0, 0, 0) }, 0);
        if fifo > 0 {
            let p = libc::sched_param { sched_priority: fifo };
            // SAFETY: valid pointer to a local (probe).
            let r = unsafe { libc::sched_setscheduler(0, libc::SCHED_FIFO, &p) };
            println!("# SCHED_FIFO {fifo}: {r}");
        }
        let sock = if send {
            let s = std::net::UdpSocket::bind("127.0.0.1:0").unwrap();
            let sink = std::net::UdpSocket::bind("127.0.0.1:0").unwrap();
            s.connect(sink.local_addr().unwrap()).unwrap();
            Some((s, sink))
        } else {
            None
        };
        let payload = [0u8; 64];
        let (c0, w0) = (thread_cpu_ns(), Instant::now());
        let mut out: [(Vec<i64>, Vec<i64>); 2] = [(Vec::new(), Vec::new()), (Vec::new(), Vec::new())];
        let mut send_err = 0u64;
        let (m, cv) = &*s2;
        let mut batch: Vec<Ent> = Vec::with_capacity(64);
        loop {
            let mut g = m.lock().unwrap();
            let head = loop {
                if let Some(Reverse(h)) = g.0.peek() {
                    break Some(h.due);
                }
                if g.1 {
                    break None;
                }
                g = cv.wait(g).unwrap();
            };
            let Some(due) = head else { break };
            let now = Instant::now();
            if due > now + lead {
                let (g2, _) = cv.wait_timeout(g, due - lead - now).unwrap();
                drop(g2);
                continue;
            }
            drop(g);
            while Instant::now() < due {
                std::hint::spin_loop();
            }
            let mut g = m.lock().unwrap();
            let now = Instant::now();
            while let Some(Reverse(h)) = g.0.peek() {
                if h.due > now {
                    break;
                }
                batch.push(*h);
                g.0.pop();
            }
            drop(g);
            let mut handed = false;
            for e in batch.drain(..) {
                if inbound && e.up {
                    ib3.q.lock().unwrap().0.push(e);
                    handed = true;
                    continue;
                }
                if let Some((s, _)) = &sock {
                    if s.send(&payload).is_err() {
                        send_err += 1;
                    }
                }
                let t = Instant::now();
                let k = if e.up { 0 } else { 1 };
                out[k].0.push((t - e.arrival).as_nanos() as i64);
                out[k].1.push(if t >= e.due { (t - e.due).as_nanos() as i64 } else { -((e.due - t).as_nanos() as i64) });
            }
            if handed {
                let w = ib3.q.lock().unwrap().1.take();
                if let Some(w) = w {
                    w.wake();
                }
            }
        }
        {
            let mut g = ib3.q.lock().unwrap();
            g.2 = true;
            if let Some(w) = g.1.take() {
                w.wake();
            }
        }
        let cpu = thread_cpu_ns() - c0;
        println!("# release thread cpu_cores={:.3} send_err={send_err}", cpu as f64 / w0.elapsed().as_nanos() as f64);
        out
    }).unwrap();
    // producer: fixed-rate ticks on an absolute schedule; one up and one down packet per tick
    let mut x: u64 = 0x2545F4914F6CDD1D;
    let mut rnd = move || {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        (x >> 11) as f64 / (1u64 << 53) as f64
    };
    // SAFETY: integer arguments only (probe): the producer's sleeps use 1 ns slack so its rate holds.
    unsafe { libc::prctl(libc::PR_SET_TIMERSLACK, 1 as libc::c_ulong, 0, 0, 0) };
    let period = Duration::from_secs_f64(1.0 / rate);
    let start = Instant::now();
    let end = start + Duration::from_secs_f64(secs);
    let mut next = start;
    let mut seq = 0u64;
    let (m, cv) = &*shared;
    while next < end {
        let now = Instant::now();
        if next > now {
            std::thread::sleep(next - now);
        }
        let arrival = Instant::now();
        let mut g = m.lock().unwrap();
        let mut earlier = false;
        for up in [true, false] {
            let d = Duration::from_nanos((20_000_000.0 + rnd() * 10_000_000.0) as u64);
            let e = Ent { due: arrival + d, seq, arrival, up };
            seq += 1;
            earlier |= g.0.peek().is_none_or(|Reverse(h)| e.due < h.due);
            g.0.push(Reverse(e));
        }
        drop(g);
        if earlier {
            cv.notify_one();
        }
        next += period;
    }
    std::thread::sleep(Duration::from_millis(100));
    {
        let mut g = m.lock().unwrap();
        g.1 = true;
    }
    cv.notify_one();
    let mut out = rel.join().unwrap();
    let wall = start.elapsed().as_secs_f64();
    println!("# produced {seq} packets in {wall:.1} s");
    if let (Some(rt), Some(t)) = (rt.as_ref(), task) {
        let (mut d, mut l) = rt.block_on(t).unwrap();
        summarize("up_inbound_task", &mut d, &mut l);
    } else {
        let (d, l) = &mut out[0];
        summarize("up_release_thread", d, l);
    }
    let (d, l) = &mut out[1];
    summarize("down_release_thread", d, l);
}

fn hiccup(secs: f64) {
    let end = Instant::now() + Duration::from_secs_f64(secs);
    let mut gaps: Vec<i64> = Vec::new();
    let mut prev = Instant::now();
    let w0 = prev;
    while prev < end {
        let now = Instant::now();
        let g = (now - prev).as_nanos() as i64;
        if g > 20_000 {
            gaps.push(g);
        }
        prev = now;
    }
    let wall = w0.elapsed().as_secs_f64();
    let lost: i64 = gaps.iter().sum();
    let c = |t: i64| gaps.iter().filter(|&&g| g > t).count();
    // predicted E[(L - 0.25 ms)+] for a release thread paused by the same gaps: sum (G - 0.25 ms)^2 / (2T)
    let pred_us: f64 = gaps.iter().map(|&g| { let e = ((g - 250_000).max(0)) as f64; e * e }).sum::<f64>() / (2.0 * wall * 1e9) / 1e3;
    println!("hiccup_pred\tE[(L-0.25ms)+]_predicted={pred_us:.2}us (P1 bar ~10 us)");
    println!(
        "hiccup\t{wall:.1}s\tgaps>20us={}\t>250us={} ({:.1}/s)\t>1ms={}\t>10ms={}\tlost={:.3}%\tmax_us={:.1}",
        gaps.len(), c(250_000), c(250_000) as f64 / wall, c(1_000_000), c(10_000_000),
        100.0 * lost as f64 / (wall * 1e9), gaps.iter().max().copied().unwrap_or(0) as f64 / 1e3
    );
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let f = |i: usize, d: f64| a.get(i).and_then(|s| s.parse::<f64>().ok()).unwrap_or(d);
    println!("# args: {:?}", &a[1..]);
    match a.get(1).map(String::as_str) {
        Some("m3") => m3(f(2, 10_000.0), f(3, 30.0), f(4, 0.0) as u64, f(5, 1.0) as u64, f(6, 0.0) as i32, f(7, 0.0) > 0.0, f(8, 0.0) > 0.0),
        Some("hiccup") => hiccup(f(2, 10.0)),
        _ => eprintln!("usage: probe m3 <rate> <secs> <lead_us> <slack_ns> <fifo> <send> <inbound> | hiccup <secs>"),
    }
}
