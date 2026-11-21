//! L1 through its public interface, as the room, the clients and the probe use it (plan §6.4: unit tests plus
//! property tests on fixed seeds). Every number checked here is part 13's (ADR 0001) or the model's (npt-l1's docs).

use npt_l1::{FateLog, L1Net, Profile, ReorderStep};
use npt_link::{Channel, Link, LinkState, SendResult};

const MS: u64 = 1_000_000;
const SEC: u64 = 1_000_000_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Got {
    seq: u64,
    sent_ns: u64,
    arrival_ns: u64,
    polled_ns: u64,
}

impl Got {
    fn delay(&self) -> u64 {
        self.arrival_ns - self.sent_ns
    }
}

fn payload(seq: u64, sent_ns: u64) -> [u8; 16] {
    let mut b = [0u8; 16];
    b[..8].copy_from_slice(&seq.to_le_bytes());
    b[8..].copy_from_slice(&sent_ns.to_le_bytes());
    b
}

fn got(bytes: &[u8], arrival_ns: u64, polled_ns: u64) -> Got {
    let seq = u64::from_le_bytes(bytes[..8].try_into().unwrap());
    let sent_ns = u64::from_le_bytes(bytes[8..16].try_into().unwrap());
    Got {
        seq,
        sent_ns,
        arrival_ns,
        polled_ns,
    }
}

/// What one exchange delivered: each direction's datagrams in the order `poll_recv` handed them out.
struct Run {
    up: Vec<Got>,
    down: Vec<Got>,
    sent: u64,
    net: L1Net,
}

/// `n` datagrams each way, one every `spacing_ns` from `spacing_ns` on, both ends polled before sending at every step,
/// then steps until everything has arrived (a datagram is never resent, so 4 s is ample).
fn exchange(net: L1Net, profile: Profile, n: u64, spacing_ns: u64) -> Run {
    let drain_until = n * spacing_ns + 4 * SEC;
    let mut client = net.connect(profile, 1200, 0);
    let mut listener = net.listener();
    let mut server: Option<Box<dyn Link>> = None;
    let (mut up, mut down) = (Vec::new(), Vec::new());
    let mut t = spacing_ns;
    let mut k = 0;
    while t <= drain_until {
        if server.is_none() {
            server = listener.poll_accept(t).map(|(l, _)| l);
        }
        let s = server.as_mut().expect("accepted at the first step");
        while let Some(r) = s.poll_recv(t) {
            assert_eq!(r.channel, Channel::Datagram);
            up.push(got(&r.bytes, r.arrival_ns, t));
        }
        while let Some(r) = client.poll_recv(t) {
            down.push(got(&r.bytes, r.arrival_ns, t));
        }
        if k < n {
            assert_eq!(client.send_unreliable(&payload(k, t), t), SendResult::Sent);
            assert_eq!(s.send_unreliable(&payload(k, t), t), SendResult::Sent);
            k += 1;
        }
        t += spacing_ns;
    }
    Run {
        up,
        down,
        sent: n,
        net,
    }
}

fn canonical(log: &FateLog) -> Vec<u8> {
    let mut out = Vec::new();
    log.for_each_canonical(|b| out.extend_from_slice(b));
    out
}

fn log_bytes(net: &L1Net) -> Vec<u8> {
    net.with_fate_log(canonical)
}

/// RFC 4737's reordered datagrams, in delivery order: those arriving after a later-sent one.
fn reordered(delivered: &[Got]) -> u64 {
    let mut highest: Option<u64> = None;
    let mut n = 0;
    for g in delivered {
        match highest {
            Some(h) if g.seq < h => n += 1,
            _ => highest = Some(g.seq),
        }
    }
    n
}

fn nearest_rank(sorted: &[u64], permille: u64) -> u64 {
    let n = sorted.len() as u64;
    let rank = (permille * n).div_ceil(1000).max(1);
    sorted[(rank - 1) as usize]
}

fn mean(xs: &[u64]) -> f64 {
    xs.iter().map(|&x| x as f64).sum::<f64>() / xs.len() as f64
}

#[test]
fn the_same_seed_gives_the_same_fate_log_for_every_profile_and_another_seed_does_not() {
    let profiles = [
        Profile::p0(),
        Profile::p1(),
        Profile::p2(),
        Profile::p3(),
        Profile::p4(),
        Profile::b1(&[3 * SEC]),
    ];
    for p in profiles {
        let a = exchange(L1Net::new(7), p.clone(), 6_000, MS);
        let b = exchange(L1Net::new(7), p.clone(), 6_000, MS);
        let (la, lb) = (log_bytes(&a.net), log_bytes(&b.net));
        assert_eq!(la, lb, "{}: same seed, same fate log", p.name);
        assert_eq!(a.up, b.up, "{}", p.name);
        assert_eq!(a.down, b.down, "{}", p.name);
        let c = exchange(L1Net::new(8), p.clone(), 6_000, MS);
        if p.name == "P0" {
            // P0 draws nothing that changes a fate: no delay, no loss, and nothing to swap at 1 ms spacing.
            assert_eq!(log_bytes(&c.net), la, "P0 is the same on every seed");
        } else {
            assert_ne!(
                log_bytes(&c.net),
                la,
                "{}: another seed, another log",
                p.name
            );
        }
    }
}

#[test]
fn fates_belong_to_their_flow_so_other_traffic_never_moves_them() {
    let p = Profile::p3();
    let alone = exchange(L1Net::new(11), p.clone(), 3_000, MS);
    // The same link 0 with a second link and reliable messages on link 0 interleaved.
    let net = L1Net::new(11);
    let mut c0 = net.connect(p.clone(), 1200, 0);
    let mut c1 = net.connect(Profile::p1(), 1200, 0);
    let mut listener = net.listener();
    let mut s0: Option<Box<dyn Link>> = None;
    // Link 1's server end is kept open, so its client's sends really enter the net.
    let mut s1: Option<Box<dyn Link>> = None;
    let mut up0 = Vec::new();
    let mut t = MS;
    let mut k = 0u64;
    while t <= 3_000 * MS + 4 * SEC {
        if s0.is_none() {
            s0 = listener.poll_accept(t).map(|(l, _)| l);
            s1 = listener.poll_accept(t).map(|(l, _)| l);
        }
        assert_eq!(s1.as_ref().map(|l| l.state()), Some(LinkState::Open));
        let s = s0.as_mut().unwrap();
        while let Some(r) = s.poll_recv(t) {
            if r.channel == Channel::Datagram {
                up0.push(got(&r.bytes, r.arrival_ns, t));
            }
        }
        if k < 3_000 {
            c1.send_unreliable(b"other link", t);
            if k.is_multiple_of(3) {
                c0.send_reliable(Channel::Control, b"a control", t);
            }
            c0.send_unreliable(&payload(k, t), t);
            k += 1;
        }
        t += MS;
    }
    let key = |g: &Got| (g.seq, g.sent_ns, g.arrival_ns);
    assert_eq!(
        up0.iter().map(key).collect::<Vec<_>>(),
        alone.up.iter().map(key).collect::<Vec<_>>(),
        "link 0's datagram fates do not depend on any other flow"
    );
}

#[test]
fn each_direction_carries_half_the_latency_a_uniform_half_jitter_and_its_own_loss() {
    // P2 without the reorder step, 200,000 datagrams each way: exact bounds, the mean, the uniform's p90 and loss.
    for seed in [1u64, 2, 3] {
        let mut p = Profile::p2();
        p.reorder = ReorderStep::OFF;
        let run = exchange(L1Net::new(seed), p, 200_000, MS / 2);
        for (name, got) in [("up", &run.up), ("down", &run.down)] {
            let mut d: Vec<u64> = got.iter().map(Got::delay).collect();
            d.sort_unstable();
            assert!(
                d[0] >= 40 * MS && *d.last().unwrap() <= 60 * MS,
                "{name}: inside 50 +- 10 ms"
            );
            let m = mean(&d);
            assert!(
                (m - 50e6).abs() < 0.1e6,
                "{name}: mean {m} ns, half of 100 ms"
            );
            let p90 = nearest_rank(&d, 900) as f64;
            assert!(
                (p90 - 58e6).abs() < 0.2e6,
                "{name}: p90 {p90}, the uniform's d + 0.8a"
            );
            let p001 = nearest_rank(&d, 1) as f64;
            let p999 = nearest_rank(&d, 999) as f64;
            assert!(
                p001 < 40.1e6 && p999 > 59.9e6,
                "{name}: the range is filled"
            );
            let lost = run.sent - got.len() as u64;
            assert!(
                (3_600..=4_400).contains(&lost),
                "{name}: {lost} of 200,000 lost, 2 % +- 10 %"
            );
        }
        // Each direction draws its own loss: both directions lose the same index about p * p of the time.
        let up: std::collections::BTreeSet<u64> = run.up.iter().map(|g| g.seq).collect();
        let down: std::collections::BTreeSet<u64> = run.down.iter().map(|g| g.seq).collect();
        let both = (0..run.sent)
            .filter(|s| !up.contains(s) && !down.contains(s))
            .count();
        assert!(
            (30..=140).contains(&both),
            "{both} indices lost both ways (expected about 80)"
        );
    }
}

#[test]
fn every_packet_sent_inside_a_burst_is_lost_both_ways() {
    let run = exchange(L1Net::new(4), Profile::b1(&[2 * SEC]), 5_000, MS);
    for (name, got) in [("up", &run.up), ("down", &run.down)] {
        assert!(
            got.iter()
                .all(|g| g.sent_ns < 2 * SEC || g.sent_ns >= 3 * SEC),
            "{name}: nothing sent inside the burst arrives"
        );
        let outside = got.len() as u64;
        // 4,000 sent outside the burst (1 ms spacing over 5 s, minus the 1,000 inside it), 2 % of them lost.
        assert!(
            (3_880..=3_960).contains(&outside),
            "{name}: {outside} delivered"
        );
    }
}

#[test]
fn the_reorder_step_swaps_about_1_percent_of_30_hz_datagrams_at_p2_and_keeps_the_floor() {
    let spacing = SEC / 30;
    for seed in [5u64, 6, 7] {
        let run = exchange(L1Net::new(seed), Profile::p2(), 60_000, spacing);
        for (name, got) in [("up", &run.up), ("down", &run.down)] {
            let n = reordered(got);
            let share = n as f64 / got.len() as f64;
            assert!(
                (0.007..0.013).contains(&share),
                "{name}: reorder share {share}"
            );
            assert!(
                got.iter().all(|g| g.delay() >= 40 * MS),
                "{name}: no delay below the floor"
            );
            // An overtaken datagram arrives 1 ns after its successor: at most one spacing plus the range late.
            assert!(
                got.iter().all(|g| g.delay() <= 60 * MS + spacing + 1),
                "{name}: holds are bounded"
            );
        }
        let counts = run.net.with_fate_log(FateLog::reorder_counts);
        assert_eq!(
            counts[0],
            (run.up.len() as u64, reordered(&run.up)),
            "the log counts what poll_recv handed out"
        );
        assert_eq!(counts[1], (run.down.len() as u64, reordered(&run.down)));
    }
    // Without the step, P2's jitter cannot reorder 30 Hz traffic (part 13: ±10 ms one way against 33 ms spacing).
    let mut off = Profile::p2();
    off.reorder = ReorderStep::OFF;
    let run = exchange(L1Net::new(5), off, 20_000, spacing);
    assert_eq!((reordered(&run.up), reordered(&run.down)), (0, 0));
}

#[test]
fn where_the_spacing_is_wider_than_the_delay_the_step_finds_nothing_to_swap() {
    // P1 at 30 Hz: 25 +- 5 ms one way against 33 ms spacing; a swap would push a datagram past its profile.
    let run = exchange(L1Net::new(9), Profile::p1(), 20_000, SEC / 30);
    assert_eq!((reordered(&run.up), reordered(&run.down)), (0, 0));
    assert!(run.up.iter().all(|g| g.delay() <= 30 * MS));
}

#[test]
fn a_packet_is_never_delivered_in_the_instant_it_was_sent() {
    let net = L1Net::new(1);
    let mut c = net.connect(Profile::p0(), 1200, 0);
    let mut listener = net.listener();
    assert!(
        listener.poll_accept(0).is_none(),
        "not in the instant of the connect"
    );
    let (mut s, _) = listener.poll_accept(1).expect("from the next instant on");
    c.send_unreliable(b"p0", 100);
    assert!(
        s.poll_recv(100).is_none(),
        "P0's zero delay still waits for the next instant"
    );
    let r = s.poll_recv(101).expect("delivered at the next instant");
    assert_eq!((r.bytes.as_slice(), r.arrival_ns), (&b"p0"[..], 100));
}

#[test]
fn delivery_follows_arrival_order_and_stamps_the_arrival_never_the_poll() {
    for seed in [21u64, 22] {
        let run = exchange(L1Net::new(seed), Profile::p4(), 20_000, MS / 4);
        for got in [&run.up, &run.down] {
            assert!(got.windows(2).all(|w| w[0].arrival_ns <= w[1].arrival_ns));
            assert!(
                got.iter()
                    .all(|g| g.arrival_ns <= g.polled_ns && g.sent_ns < g.polled_ns)
            );
            assert!(
                reordered(got) > 0,
                "P4's jitter reorders datagrams 0.25 ms apart"
            );
        }
    }
}

#[test]
fn streams_lose_nothing_keep_their_order_and_wait_out_a_burst() {
    let p = Profile::b1(&[2 * SEC]);
    let net = L1Net::new(3);
    let mut c = net.connect(p, 1200, 0);
    let mut listener = net.listener();
    let mut s: Option<Box<dyn Link>> = None;
    let mut control = Vec::new();
    let mut full = Vec::new();
    let mut t = MS;
    let mut k = 0u64;
    while t <= 8 * SEC {
        if s.is_none() {
            s = listener.poll_accept(t).map(|(l, _)| l);
        }
        let srv = s.as_mut().unwrap();
        while let Some(r) = srv.poll_recv(t) {
            let g = got(&r.bytes, r.arrival_ns, t);
            match r.channel {
                Channel::Control => control.push(g),
                Channel::FullState => full.push(g),
                Channel::Datagram => unreachable!(),
            }
        }
        if k < 4_000 {
            c.send_reliable(Channel::Control, &payload(k, t), t);
            if k.is_multiple_of(4) {
                c.send_reliable(Channel::FullState, &payload(k, t), t);
            }
            k += 1;
        }
        t += MS;
    }
    assert_eq!(control.len(), 4_000, "a stream loses nothing");
    assert_eq!(full.len(), 1_000);
    for stream in [&control, &full] {
        assert!(stream.windows(2).all(|w| w[0].seq < w[1].seq), "in order");
    }
    // A message sent inside the burst [2 s, 3 s) waits for its first resend outside it. With a packet sent every
    // millisecond the PTO never fires (each send restarts it): the losses found before the burst are resent by about
    // 2.1 s (the last acknowledgements sent before 2 s arrive by 2.06 s, and the time threshold is at most 9/8 x 120 ms
    // after a send), and nothing is resent from 2.2 s until the burst ends. The first acknowledgements after it, from
    // about 3.1 s, declare its packets lost by the loss rules (the unit tests pin the packet and the time threshold),
    // and their resends arrive about 50 ms later.
    for stream in [&control, &full] {
        for g in stream
            .iter()
            .filter(|g| (2 * SEC..3 * SEC).contains(&g.sent_ns))
        {
            assert!(
                (3 * SEC + 40 * MS..3 * SEC + 600 * MS).contains(&g.arrival_ns),
                "seq {} arrived at {}",
                g.seq,
                g.arrival_ns
            );
        }
    }
    let inside = net.with_fate_log(|log| {
        let mut sends = Vec::new();
        log.for_each_canonical(|b| {
            // A stream packet's entry (64 bytes): its resends field, then its send time.
            let sent = || u64::from_le_bytes(b[16..24].try_into().unwrap());
            if b.len() == 64
                && u32::from_le_bytes(b[36..40].try_into().unwrap()) > 0
                && (2_200 * MS..3 * SEC).contains(&sent())
            {
                sends.push(sent());
            }
        });
        sends
    });
    assert!(
        inside.is_empty(),
        "{} stream packets resent inside the burst after 2.2 s, the first at {} ns: a PTO fired",
        inside.len(),
        inside.first().copied().unwrap_or(0)
    );
    // Outside the burst, P2's 2 % loss makes some messages wait for a resend, past the one-way ceiling of 60 ms, and
    // hold back those behind them.
    let late = control
        .iter()
        .filter(|g| g.sent_ns < 2 * SEC && g.delay() > 60 * MS)
        .count();
    assert!(late > 0, "some messages were resent");
    let resent = net.with_fate_log(|log| {
        let mut n = 0u64;
        log.for_each_canonical(|b| {
            // A stream packet's entry (64 bytes) whose resends field is above 0.
            if b.len() == 64 && u32::from_le_bytes(b[36..40].try_into().unwrap()) > 0 {
                n += 1;
            }
        });
        n
    });
    assert!(
        resent > 1_000,
        "{resent} resends: the burst's packets and 2 % of the rest"
    );
}

/// The mixed run of L-c8's unit half: 3,000 steps of 1 ms, a datagram each way per step, a control message up every
/// 3rd step and a 1,500 B full state down every 5th, then 4 s of polls.
fn mixed(profile: Profile, seed: u64) -> L1Net {
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
        let srv = s.as_mut().expect("accepted at the first step");
        while srv.poll_recv(t).is_some() {}
        while c.poll_recv(t).is_some() {}
        if k < 3_000 {
            if k.is_multiple_of(3) {
                c.send_reliable(Channel::Control, b"a control", t);
            }
            c.send_unreliable(&payload(k, t), t);
            if k.is_multiple_of(5) {
                srv.send_reliable(Channel::FullState, &big, t);
            }
            srv.send_unreliable(&payload(k, t), t);
            k += 1;
        }
        t += MS;
    }
    net
}

/// FNV-1a-64 of v1's tag (`npt-l1 fate log v1` and a newline) followed by the log's datagram entries in its canonical
/// form (each 40 bytes, flow code 0), and how many of each kind of entry the log holds: (datagram, stream, ack).
fn v1_datagram_fnv(log: &FateLog) -> (u64, [u64; 3]) {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    let mut eat = |b: &[u8]| {
        for &x in b {
            h ^= u64::from(x);
            h = h.wrapping_mul(0x0000_0100_0000_01b3);
        }
    };
    eat(b"npt-l1 fate log v1\n");
    let mut counts = [0u64; 3];
    let mut tag = true;
    log.for_each_canonical(|b| {
        if tag {
            assert_eq!(b, b"npt-l1 fate log v2\n");
            tag = false;
            return;
        }
        match (b.len(), b[5]) {
            (40, 0) => {
                counts[0] += 1;
                eat(b);
            }
            (64, 1 | 2) => counts[1] += 1,
            (56, 3) => counts[2] += 1,
            other => panic!("an entry of {} bytes with flow code {}", other.0, other.1),
        }
    });
    (h, counts)
}

/// L-c8's unit half (u4-build-plan.md §5 U4c; Q15's fate log v2): a datagram's entry keeps v1's 40-byte form, and its
/// fate does not move with the stream packets and acknowledgements around it, so v1's tag followed by a v2 log's
/// datagram entries are the bytes v1's tag and datagram entries were for the same run. For the probe, whose v1 log holds
/// only datagrams, that is v1's whole log, the hash L-c8 compares. The pins are those bytes' FNV-1a-64 for this mixed
/// run on the U2 code (097f897, before U4c), for P2, B1 with a burst at 1 s, and P4, seed 1. (The judged half, the
/// probe's SHA-256 against U2's committed 913db8e9, feabf2f0 and b8a04f2f, runs with -3's re-run after U4j.)
#[test]
fn v1s_tag_and_the_v2_logs_datagram_entries_hash_as_v1s_log_did() {
    for (p, pin) in [
        (Profile::p2(), 0xadfb_ebe4_bb1a_bf64_u64),
        (Profile::b1(&[SEC]), 0x4374_9a62_922e_9b35),
        (Profile::p4(), 0x246e_7a24_1a2a_0a18),
    ] {
        let name = p.name;
        let (h, [datagrams, stream, acks]) = mixed(p, 1).with_fate_log(v1_datagram_fnv);
        assert_eq!(datagrams, 6_000, "{name}: every datagram has its entry");
        assert!(
            stream > 1_600 && acks > datagrams,
            "{name}: {stream} stream packets and {acks} acknowledgements logged beside them"
        );
        assert_eq!(
            h, pin,
            "{name}: the datagram entries are not v1's ({h:#018x})"
        );
    }
}

/// Q15: an acknowledgement is lost only inside a burst. In the mixed run under B1 (its 2 % loss, and a burst at 1 s),
/// the loss outside the burst takes packets of the other flows but no acknowledgement: in the v2 log no
/// acknowledgement's entry is lost (kind 1), and one is lost in a burst (kind 2) exactly when it was sent inside
/// [1 s, 2 s).
#[test]
fn acknowledgements_are_lost_only_inside_bursts() {
    let net = mixed(Profile::b1(&[SEC]), 1);
    // (packets lost outside the burst, acknowledgements lost in it, acknowledgements delivered)
    let mut n = (0u64, 0u64, 0u64);
    let mut wrong = Vec::new();
    net.with_fate_log(|log| {
        log.for_each_canonical(|b| {
            if b.len() < 40 {
                return;
            }
            let (code, kind) = (b[5], b[6]);
            let sent = u64::from_le_bytes(b[16..24].try_into().unwrap());
            let inside = (SEC..2 * SEC).contains(&sent);
            if code == 3 {
                match (kind, inside) {
                    (0, false) => n.2 += 1,
                    (2, true) => n.1 += 1,
                    _ => wrong.push((kind, sent)),
                }
            } else if kind == 1 {
                n.0 += 1;
            }
        });
    });
    assert!(
        wrong.is_empty(),
        "{} acknowledgements with the wrong fate, the first (kind, sent ns) {:?}",
        wrong.len(),
        wrong.first()
    );
    assert!(
        n.0 > 50 && n.1 > 50 && n.2 > 3_000,
        "the run did not lose packets outside the burst and acknowledgements in it: {n:?}"
    );
    println!("lost outside the burst, acknowledgements lost in it, delivered: {n:?}");
}

#[test]
fn next_event_ns_names_the_next_arrival_then_its_acknowledgement_then_nothing() {
    let net = L1Net::new(5);
    assert_eq!(net.next_event_ns(), None, "nothing pending on a new net");
    let mut c = net.connect(Profile::p2(), 1200, 0);
    let mut listener = net.listener();
    let (mut s, _) = listener.poll_accept(1).unwrap();
    assert_eq!(net.next_event_ns(), None);
    c.send_unreliable(b"one", 10 * MS);
    let arrival = net.next_event_ns().expect("the datagram's arrival");
    assert!((50 * MS..=70 * MS).contains(&arrival), "{arrival}");
    assert!(s.poll_recv(arrival - 1).is_none(), "nothing before it");
    let r = s.poll_recv(arrival).expect("handed out at its arrival");
    assert_eq!(r.arrival_ns, arrival);
    let ack = net.next_event_ns().expect("its acknowledgement's arrival");
    assert!(
        (arrival + 40 * MS..=arrival + 60 * MS).contains(&ack),
        "{ack}"
    );
    assert_eq!(
        c.rtt_ns(),
        None,
        "no sample before the acknowledgement arrives"
    );
    assert!(c.poll_recv(ack).is_none());
    assert_eq!(
        c.rtt_ns(),
        Some(ack - 10 * MS),
        "the first sample sets the estimate"
    );
    assert_eq!(
        net.next_event_ns(),
        None,
        "a datagram arms no probe timer, so nothing is left"
    );
}

#[test]
fn a_datagram_refused_as_too_large_takes_no_index() {
    let plain = exchange(L1Net::new(13), Profile::p2(), 500, MS);
    let net = L1Net::new(13);
    let mut c = net.connect(Profile::p2(), 1200, 0);
    let mut listener = net.listener();
    let mut s: Option<Box<dyn Link>> = None;
    let mut up = Vec::new();
    let mut t = MS;
    let mut k = 0u64;
    while t <= 500 * MS + 2 * SEC {
        if s.is_none() {
            s = listener.poll_accept(t).map(|(l, _)| l);
        }
        let srv = s.as_mut().unwrap();
        while let Some(r) = srv.poll_recv(t) {
            up.push(got(&r.bytes, r.arrival_ns, t));
        }
        if k < 500 {
            if k.is_multiple_of(7) {
                assert_eq!(c.send_unreliable(&[0u8; 1201], t), SendResult::TooLarge);
            }
            c.send_unreliable(&payload(k, t), t);
            k += 1;
        }
        t += MS;
    }
    let key = |g: &Got| (g.seq, g.arrival_ns);
    assert_eq!(
        up.iter().map(key).collect::<Vec<_>>(),
        plain.up.iter().map(key).collect::<Vec<_>>()
    );
}

#[test]
fn dropping_an_end_closes_the_link_and_draws_nothing_more() {
    let net = L1Net::new(2);
    let mut c = net.connect(Profile::p2(), 1200, 0);
    let mut listener = net.listener();
    let (s, addr) = listener.poll_accept(1).unwrap();
    assert_eq!(c.state(), LinkState::Open);
    assert!(addr.ip().is_ipv4());
    c.send_unreliable(b"before", 10);
    drop(s);
    assert_eq!(c.state(), LinkState::Closed);
    let before = net.with_fate_log(FateLog::len);
    assert_eq!(c.send_unreliable(b"after", 20), SendResult::Sent);
    assert_eq!(
        net.with_fate_log(FateLog::len),
        before,
        "nothing enters a closed link"
    );
    assert_eq!(c.max_datagram_size(), 1200);
    assert_eq!(c.unsent_unreliable(), 0);
}

#[test]
fn the_listener_yields_each_link_once_in_connect_order_with_its_own_address() {
    let net = L1Net::new(2);
    let _a = net.connect(Profile::p1(), 1200, 5);
    let _b = net.connect(Profile::p2(), 1200, 5);
    let mut listener = net.listener();
    assert!(listener.poll_accept(5).is_none());
    let (_, first) = listener.poll_accept(6).unwrap();
    let (_, second) = listener.poll_accept(6).unwrap();
    assert!(listener.poll_accept(7).is_none());
    assert_ne!(first, second);
}

#[test]
fn the_rtt_estimate_is_smoothed_from_acknowledgements_and_lands_near_the_profile() {
    let net = L1Net::new(17);
    let mut c = net.connect(Profile::p2(), 1200, 0);
    let mut listener = net.listener();
    let (mut s, _) = listener.poll_accept(1).unwrap();
    assert_eq!(
        c.rtt_ns(),
        None,
        "no sample before the first acknowledgement"
    );
    let mut t = MS;
    for k in 0..3_000u64 {
        while s.poll_recv(t).is_some() {}
        c.send_unreliable(&payload(k, t), t);
        t += MS;
    }
    let rtt = c.rtt_ns().expect("samples have come back") as f64;
    assert!(
        (80e6..120e6).contains(&rtt),
        "smoothed RTT {rtt} ns against P2's 100 ms"
    );
    assert_eq!(
        s.rtt_ns(),
        None,
        "the server end sent nothing, so it has no sample"
    );
}

#[cfg(feature = "bites")]
mod bites {
    use super::*;

    fn delays(run: &Run) -> (Vec<u64>, Vec<u64>) {
        let mut up: Vec<u64> = run.up.iter().map(Got::delay).collect();
        let mut down: Vec<u64> = run.down.iter().map(Got::delay).collect();
        up.sort_unstable();
        down.sort_unstable();
        (up, down)
    }

    fn p2_off() -> Profile {
        let mut p = Profile::p2();
        p.reorder = ReorderStep::OFF;
        p
    }

    #[test]
    fn every_bite_is_listed_and_an_unknown_id_is_refused() {
        assert_eq!(L1Net::BITES.len(), 4);
        for id in L1Net::BITES {
            assert!(L1Net::with_bite(1, id).is_some(), "{id}");
        }
        assert!(L1Net::with_bite(1, "PM-0001-3/no-such-bite").is_none());
    }

    #[test]
    fn rtt_each_way_doubles_the_one_way_mean() {
        let run = exchange(
            L1Net::with_bite(1, "PM-0001-3/rtt-each-way").unwrap(),
            p2_off(),
            40_000,
            MS / 2,
        );
        let (up, down) = delays(&run);
        assert!((mean(&up) - 100e6).abs() < 0.3e6 && (mean(&down) - 100e6).abs() < 0.3e6);
    }

    #[test]
    fn loss_per_round_trip_halves_each_directions_loss_and_never_loses_both() {
        let run = exchange(
            L1Net::with_bite(1, "PM-0001-3/loss-per-round-trip").unwrap(),
            p2_off(),
            200_000,
            MS / 2,
        );
        for got in [&run.up, &run.down] {
            let lost = run.sent - got.len() as u64;
            assert!(
                (1_700..=2_300).contains(&lost),
                "{lost} lost: about 1 %, not 2 %"
            );
        }
        let up: std::collections::BTreeSet<u64> = run.up.iter().map(|g| g.seq).collect();
        let down: std::collections::BTreeSet<u64> = run.down.iter().map(|g| g.seq).collect();
        assert!((0..run.sent).all(|s| up.contains(&s) || down.contains(&s)));
    }

    #[test]
    fn normal_jitter_keeps_the_range_but_not_the_uniforms_shape() {
        let run = exchange(
            L1Net::with_bite(1, "PM-0001-3/normal-jitter").unwrap(),
            p2_off(),
            200_000,
            MS / 2,
        );
        let (up, _) = delays(&run);
        let p90 = nearest_rank(&up, 900) as f64;
        // A normal with sigma = a / 3.09 has its p90 at d + 1.28 sigma, about 54.1 ms, not the uniform's 58 ms.
        assert!((p90 - 54.15e6).abs() < 0.5e6, "p90 {p90}");
        assert!((mean(&up) - 50e6).abs() < 0.1e6, "the mean is unchanged");
    }

    #[test]
    fn a_clock_seed_gives_another_fate_log_for_the_same_seed() {
        let a = exchange(
            L1Net::with_bite(1, "PM-0001-3/clock-seed").unwrap(),
            Profile::p2(),
            2_000,
            MS,
        );
        let b = exchange(
            L1Net::with_bite(1, "PM-0001-3/clock-seed").unwrap(),
            Profile::p2(),
            2_000,
            MS,
        );
        assert_ne!(log_bytes(&a.net), log_bytes(&b.net));
    }
}
