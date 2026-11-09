//! U4c scratch: `probe <P2|B1|P4> <seed>` and `pins`. See Cargo.toml.

use npt_l1::{FateLog, L1Net, Profile};
use npt_link::{Channel, Link};
use sha2::{Digest, Sha256};

const MS: u64 = 1_000_000;
const SEC: u64 = 1_000_000_000;
const V1_TAG: &[u8] = b"npt-l1 fate log v1\n";

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

/// (SHA-256 of the whole canonical form, SHA-256 of the v1 tag + the datagram entries).
fn shas(log: &FateLog) -> (String, String, usize, usize) {
    let mut all = Sha256::new();
    let mut dg = Sha256::new();
    dg.update(V1_TAG);
    let (mut first, mut entries, mut datagrams) = (true, 0usize, 0usize);
    log.for_each_canonical(|b| {
        all.update(b);
        if first {
            first = false;
            return;
        }
        entries += 1;
        if b.len() == 40 && b[5] == 0 {
            datagrams += 1;
            dg.update(b);
        }
    });
    (hex(&all.finalize()), hex(&dg.finalize()), entries, datagrams)
}

fn fnv_datagrams(log: &FateLog) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    let mut eat = |b: &[u8]| {
        for &x in b {
            h ^= u64::from(x);
            h = h.wrapping_mul(0x0000_0100_0000_01b3);
        }
    };
    eat(V1_TAG);
    let mut first = true;
    log.for_each_canonical(|b| {
        if first {
            first = false;
            return;
        }
        if b.len() == 40 && b[5] == 0 {
            eat(b);
        }
    });
    h
}

fn in_burst(p: &Profile, t: u64) -> bool {
    p.bursts_ns.iter().any(|&(s, l)| t >= s && t - s < l)
}

/// probe.rs run()'s send loop: 10,000 per second each way until 1,000,000 are sent outside bursts; 40 B each.
fn probe(profile: Profile, seed: u64) {
    let interval = SEC / 10_000;
    let send_time = |i: u64| interval / 2 + i * interval;
    let (mut total, mut outside) = (0u64, 0u64);
    while outside < 1_000_000 {
        if !in_burst(&profile, send_time(total)) {
            outside += 1;
        }
        total += 1;
    }
    let net = L1Net::new(seed);
    let mut client = net.connect(profile.clone(), 1200, 0);
    let mut listener = net.listener();
    let mut server: Option<Box<dyn Link>> = None;
    let end = send_time(total - 1) + 2 * SEC;
    let mut t = send_time(0);
    let mut i = 0u64;
    let payload = [0u8; 40];
    while t <= end {
        if server.is_none() {
            server = listener.poll_accept(t).map(|(l, _)| l);
        }
        let s = server.as_mut().expect("accepted");
        while s.poll_recv(t).is_some() {}
        while client.poll_recv(t).is_some() {}
        if i < total {
            client.send_unreliable(&payload, t);
            s.send_unreliable(&payload, t);
            i += 1;
        }
        t += interval;
    }
    drop(server);
    let (all, dg, entries, datagrams) = net.with_fate_log(shas);
    println!(
        "{} seed {seed}: sent {total} each way; entries {entries}, datagram entries {datagrams}\n  whole log    {all}\n  v1 tag + datagram entries {dg}",
        profile.name
    );
}

/// The unit pin's scenario: 3,000 steps of 1 ms, a datagram each way per step, a control message up every 3rd step
/// and a 1,500 B full-state message down every 5th, then 4 s of polls.
fn mixed(profile: Profile, seed: u64) -> u64 {
    let net = L1Net::new(seed);
    let mut c = net.connect(profile, 1200, 0);
    let mut listener = net.listener();
    let mut s: Option<Box<dyn Link>> = None;
    let mut t = MS;
    let mut k = 0u64;
    let big = [7u8; 1500];
    while t <= 3_000 * MS + 4 * SEC {
        if s.is_none() {
            s = listener.poll_accept(t).map(|(l, _)| l);
        }
        let srv = s.as_mut().expect("accepted");
        while srv.poll_recv(t).is_some() {}
        while c.poll_recv(t).is_some() {}
        if k < 3_000 {
            let mut p = [0u8; 16];
            p[..8].copy_from_slice(&k.to_le_bytes());
            p[8..].copy_from_slice(&t.to_le_bytes());
            if k.is_multiple_of(3) {
                c.send_reliable(Channel::Control, b"a control", t);
            }
            c.send_unreliable(&p, t);
            if k.is_multiple_of(5) {
                srv.send_reliable(Channel::FullState, &big, t);
            }
            srv.send_unreliable(&p, t);
            k += 1;
        }
        t += MS;
    }
    net.with_fate_log(fnv_datagrams)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("probe") => {
            let seed: u64 = args[3].parse().expect("seed");
            let p = match args[2].as_str() {
                "B1" => Profile::b1(&[50 * SEC]),
                n => Profile::by_name(n).expect("profile"),
            };
            probe(p, seed);
        }
        Some("pins") => {
            for (p, seed) in [
                (Profile::p2(), 1u64),
                (Profile::b1(&[SEC]), 1),
                (Profile::p4(), 1),
            ] {
                println!("{} seed {seed}: {:#018x}", p.name, mixed(p.clone(), seed));
            }
        }
        _ => eprintln!("usage: probe <P2|B1|P4> <seed> | pins"),
    }
}
