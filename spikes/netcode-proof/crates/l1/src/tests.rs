//! npt-l1's unit tests: the helpers, and the link model's L-c tests (u4-build-plan.md §5 U4c; the advisor's C1' and C2),
//! which read the model's state and its test-only trace. Each L-c test is a check that returns its first failure as an
//! `Err` naming it, run on clean nets here and, in bites builds, on each planted defect, which must fail it.

use super::*;

/// 30 Hz.
const TICK: u64 = SEC / 30;

macro_rules! ensure {
    ($cond:expr, $($arg:tt)+) => {
        if !$cond {
            return Err(format!($($arg)+));
        }
    };
}

fn lossless(p: Profile) -> Profile {
    Profile { loss_ppm: 0, ..p }
}

/// What one end handed out: (channel, bytes, arrival).
type Got = (Channel, Vec<u8>, u64);

/// A net with its links' two ends, each polled at every call to `poll`.
struct Rig {
    net: L1Net,
    /// `ends[link]`: [client, server].
    ends: Vec<[Box<dyn Link>; 2]>,
    /// `got[link][end]`.
    got: Vec<[Vec<Got>; 2]>,
    /// `at[link][end]`: the instant of the poll that handed out each of `got[link][end]`.
    at: Vec<[Vec<u64>; 2]>,
}

impl Rig {
    /// Connects one link per profile at 0 and accepts each at 1 ns.
    fn new(net: L1Net, profiles: &[Profile]) -> Rig {
        let clients: Vec<Box<dyn Link>> = profiles
            .iter()
            .map(|p| net.connect(p.clone(), 1200, 0))
            .collect();
        let mut listener = net.listener();
        let ends: Vec<[Box<dyn Link>; 2]> = clients
            .into_iter()
            .map(|c| {
                let (s, _) = listener
                    .poll_accept(1)
                    .expect("accepted from the next instant on");
                [c, s]
            })
            .collect();
        let got = ends.iter().map(|_| [Vec::new(), Vec::new()]).collect();
        let at = ends.iter().map(|_| [Vec::new(), Vec::new()]).collect();
        Rig { net, ends, got, at }
    }

    fn poll(&mut self, t: u64) {
        let links = self.ends.iter_mut().zip(&mut self.got).zip(&mut self.at);
        for ((ends, got), at) in links {
            for ((end, got), at) in ends.iter_mut().zip(got.iter_mut()).zip(at.iter_mut()) {
                while let Some(r) = end.poll_recv(t) {
                    got.push((r.channel, r.bytes, r.arrival_ns));
                    at.push(t);
                }
            }
        }
    }

    fn send(&mut self, link: usize, end: usize, bytes: &[u8], t: u64) {
        assert_eq!(
            self.ends[link][end].send_unreliable(bytes, t),
            SendResult::Sent
        );
    }

    fn send_on(&mut self, link: usize, end: usize, channel: Channel, bytes: &[u8], t: u64) {
        assert_eq!(
            self.ends[link][end].send_reliable(channel, bytes, t),
            SendResult::Sent
        );
    }

    /// A datagram each way on `link`: 100 B up, 400 B down.
    fn tick(&mut self, link: usize, t: u64) {
        self.send(link, CLIENT, &[1; 100], t);
        self.send(link, SERVER, &[2; 400], t);
    }

    fn entries(&self) -> Vec<Fate> {
        self.net.net.borrow().log.entries.clone()
    }

    fn est(&self, link: usize, dir: Dir) -> Rtt {
        self.net.net.borrow().links[link].rtt[dir as usize]
    }

    fn pto_count(&self, link: usize, dir: Dir) -> u32 {
        self.net.net.borrow().links[link].pto_count[dir as usize]
    }

    fn timer(&self, link: usize, dir: Dir) -> Option<(u64, u64)> {
        self.net.net.borrow().links[link].spaces[dir as usize][0].timer
    }

    fn ptos(&self, link: u32, dir: Dir) -> Vec<Pto> {
        let net = self.net.net.borrow();
        let all = net.trace.ptos.iter();
        all.filter(|p| p.link == link && p.dir == dir)
            .copied()
            .collect()
    }

    fn declared(&self, link: u32, dir: Dir) -> Vec<Declared> {
        let net = self.net.net.borrow();
        let all = net.trace.declared.iter();
        all.filter(|d| d.link == link && d.dir == dir)
            .copied()
            .collect()
    }

    fn samples(&self, link: u32, dir: Dir) -> Vec<Sample> {
        let net = self.net.net.borrow();
        let all = net.trace.samples.iter();
        all.filter(|s| s.link == link && s.dir == dir)
            .copied()
            .collect()
    }
}

fn piece(channel: Channel, message: u64, part: u32) -> Piece {
    Piece {
        channel,
        message,
        part,
    }
}

/// Plants the loss of a piece's `send`-th send (0: its first) on `link`, client to server.
fn lose_up(net: &L1Net, link: u32, p: Piece, send: u32) {
    net.force(
        link,
        Forced::Piece {
            dir: Dir::Up,
            piece: p,
            send,
        },
    );
}

/// The fate-log entries, with their indexes, of one stream message's pieces, in send order.
fn sends_of(
    log: &[Fate],
    link: u32,
    dir: Dir,
    channel: Channel,
    message: u64,
) -> Vec<(usize, Fate)> {
    log.iter()
        .enumerate()
        .filter(|(_, e)| {
            e.link == link
                && e.dir == dir
                && matches!(e.carries, Carries::Stream { channel: c, message: m, .. } if c == channel && m == message)
        })
        .map(|(i, e)| (i, e.clone()))
        .collect()
}

fn pn_of(e: &Fate) -> u64 {
    match e.carries {
        Carries::Stream { pn, .. } => pn,
        _ => u64::MAX,
    }
}

fn arrival_of(e: &Fate) -> Option<u64> {
    match e.outcome {
        Outcome::Delivered { arrival_ns } => Some(arrival_ns),
        _ => None,
    }
}

fn ms(ns: u64) -> f64 {
    ns as f64 / 1e6
}

#[test]
fn below_maps_the_whole_range_evenly_at_its_ends() {
    assert_eq!(below(0, 10), 0);
    assert_eq!(below(u64::MAX, 10), 9);
    assert_eq!(below(1 << 63, 10), 5);
    assert_eq!(below(u64::MAX, 1), 0);
}

#[test]
fn the_named_profiles_are_part_13s() {
    let rows = [
        (Profile::p0(), "P0", 0, 0, 0),
        (Profile::p1(), "P1", 50, 10, 10_000),
        (Profile::p2(), "P2", 100, 20, 20_000),
        (Profile::p3(), "P3", 150, 30, 50_000),
        (Profile::p4(), "P4", 200, 40, 50_000),
        (Profile::b1(&[5 * SEC]), "B1", 100, 20, 20_000),
    ];
    for (p, name, rtt_ms, jitter_ms, ppm) in rows {
        assert_eq!(
            (p.name, p.rtt_ns, p.jitter_ns, p.loss_ppm),
            (name, rtt_ms * MS, jitter_ms * MS, ppm)
        );
        assert_eq!(
            p.one_way_ns(),
            rtt_ms * MS / 2,
            "{name}: half the latency each way"
        );
        assert_eq!(
            p.half_jitter_ns(),
            jitter_ms * MS / 2,
            "{name}: half the jitter each way"
        );
        assert_eq!(
            p.reorder,
            ReorderStep::PART_13,
            "{name}: every run also reordering"
        );
    }
    let b1 = Profile::b1(&[5 * SEC]);
    assert_eq!(b1.bursts_ns, vec![(5 * SEC, SEC)]);
    assert!(!b1.in_burst(5 * SEC - 1) && b1.in_burst(5 * SEC) && b1.in_burst(6 * SEC - 1));
    assert!(!b1.in_burst(6 * SEC));
    assert_eq!(Profile::by_name("P3"), Some(Profile::p3()));
    assert_eq!(Profile::by_name("B1"), None, "B1 needs its burst times");
}

#[test]
fn the_rtt_estimator_follows_rfc_9002_from_its_starting_state() {
    let mut r = Rtt::START;
    assert_eq!(r.smoothed, None);
    // RFC 9002 §6.2.2 and §5.3: 333 ms and 166.5 ms before any sample, so the first PTO is 333 + 666 + 25 ms and the
    // time threshold 9/8 x 333 ms.
    assert_eq!((r.smoothed_ns(), r.var), (333 * MS, 166_500_000));
    assert_eq!(r.pto_ns(), 1_024 * MS);
    assert_eq!(r.loss_delay_ns(), 374_625_000);
    r.sample(100);
    assert_eq!((r.smoothed, r.var, r.latest), (Some(100), 50, 100));
    r.sample(200);
    // var = (3 * 50 + |100 - 200|) / 4 = 62; smoothed = (7 * 100 + 200) / 8 = 112.
    assert_eq!((r.smoothed, r.var, r.latest), (Some(112), 62, 200));
    // kGranularity floors both: a zero estimate gives a 26 ms PTO and a 1 ms time threshold.
    let mut z = Rtt::START;
    z.sample(0);
    assert_eq!((z.pto_ns(), z.loss_delay_ns()), (26 * MS, MS));
}

#[test]
fn the_canonical_form_is_the_tag_then_one_fixed_form_per_kind_of_entry() {
    let log = FateLog {
        entries: vec![
            Fate {
                link: 1,
                dir: Dir::Down,
                carries: Carries::Stream {
                    channel: Channel::Control,
                    pn: 7,
                    message: 8,
                    part: 2,
                    parts: 3,
                },
                seq: 2,
                sent_ns: 3,
                len: 4,
                outcome: Outcome::Delivered { arrival_ns: 5 },
                overtaken: false,
                retransmits: 6,
            },
            Fate {
                link: 0,
                dir: Dir::Up,
                carries: Carries::Datagram,
                seq: 0,
                sent_ns: 9,
                len: 1,
                outcome: Outcome::Burst,
                overtaken: true,
                retransmits: 0,
            },
            Fate {
                link: 2,
                dir: Dir::Up,
                carries: Carries::Ack {
                    acked: 11,
                    largest: 12,
                },
                seq: 10,
                sent_ns: 13,
                len: 0,
                outcome: Outcome::Lost,
                overtaken: false,
                retransmits: 0,
            },
        ],
    };
    let mut calls = Vec::new();
    log.for_each_canonical(|b| calls.push(b.to_vec()));
    assert_eq!(calls.len(), 4, "the tag, then one call per entry");
    assert_eq!(calls[0], b"npt-l1 fate log v2\n");
    let (s, d, a) = (&calls[1], &calls[2], &calls[3]);
    assert_eq!((s.len(), d.len(), a.len()), (64, 40, 56));
    assert_eq!(&s[0..8], &[1, 0, 0, 0, 1, 1, 0, 0]);
    assert_eq!(&s[8..16], &2u64.to_le_bytes());
    assert_eq!(&s[24..32], &5u64.to_le_bytes());
    assert_eq!(&s[32..40], &[4, 0, 0, 0, 6, 0, 0, 0]);
    assert_eq!(&s[40..48], &7u64.to_le_bytes(), "the packet number");
    assert_eq!(&s[48..56], &8u64.to_le_bytes(), "the message");
    assert_eq!(&s[56..64], &[2, 0, 0, 0, 3, 0, 0, 0], "piece 2 of 3");
    assert_eq!(
        &d[0..8],
        &[0, 0, 0, 0, 0, 0, 2, 1],
        "a datagram lost in a burst"
    );
    assert_eq!(&d[16..24], &9u64.to_le_bytes());
    assert_eq!(
        &d[24..32],
        &0u64.to_le_bytes(),
        "no arrival for a lost packet"
    );
    assert_eq!(
        &a[0..8],
        &[2, 0, 0, 0, 0, 3, 1, 0],
        "an acknowledgement, lost"
    );
    assert_eq!(&a[8..16], &10u64.to_le_bytes());
    assert_eq!(
        &a[40..56],
        &[11, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0]
    );
}

#[test]
fn reorder_counts_follow_rfc_4737_and_count_only_datagrams() {
    let mk = |seq: u64, arrival_ns: u64| Fate {
        link: 0,
        dir: Dir::Up,
        carries: Carries::Datagram,
        seq,
        sent_ns: seq,
        len: 1,
        outcome: Outcome::Delivered { arrival_ns },
        overtaken: false,
        retransmits: 0,
    };
    let ack = Fate {
        carries: Carries::Ack {
            acked: 0,
            largest: 0,
        },
        ..mk(0, 5)
    };
    // Arrival order 0, 2, 1, 3, 5, 4: seq 1 and seq 4 arrive after a later-sent datagram.
    let log = FateLog {
        entries: vec![
            mk(0, 10),
            mk(1, 31),
            mk(2, 30),
            ack,
            mk(3, 40),
            mk(4, 61),
            mk(5, 60),
        ],
    };
    assert_eq!(log.reorder_counts(), [(6, 2), (0, 0)]);
}

/// L-c1 (C1' (b), (f)): P2's delays with no loss but the planted ones, 30 Hz datagrams both ways, and from 3 s a control
/// message every half second right after the client's datagram, its first packet lost. Each is declared lost by the loss
/// rules (a later packet's acknowledgement, the time threshold), never by the PTO, and resent then; the next datagram is
/// one tick later and a round trip is 80 to 120 ms, so the declaration falls 113.3 to 153.3 ms after the send (a later
/// packet is acknowledged by then and the time threshold, 9/8 of an estimate of at most 120 ms, has passed), and the
/// resend's arrival 40 to 60 ms after that.
fn lc1(net: L1Net) -> Result<(), String> {
    const N: u64 = 12;
    let first = 90;
    for j in 0..N {
        lose_up(&net, 0, piece(Channel::Control, j, 0), 0);
    }
    let mut rig = Rig::new(net, &[lossless(Profile::p2())]);
    let mut sent = Vec::new();
    let last = first + 15 * N + 30;
    for k in 0..last {
        let t = MS + k * TICK;
        rig.poll(t);
        rig.tick(0, t);
        if k >= first && (k - first).is_multiple_of(15) && (sent.len() as u64) < N {
            let j = sent.len() as u64;
            rig.send_on(0, CLIENT, Channel::Control, &j.to_le_bytes(), t);
            sent.push(t);
        }
    }
    rig.poll(MS + last * TICK + 2 * SEC);
    let ptos = rig.ptos(0, Dir::Up);
    ensure!(
        ptos.is_empty(),
        "L-c1: {} probe timeouts fired (the first at {:.3} ms): the loss rules should repair each loss first",
        ptos.len(),
        ms(ptos[0].at_ns)
    );
    let log = rig.entries();
    let declared = rig.declared(0, Dir::Up);
    let got: Vec<&Got> = rig.got[0][SERVER]
        .iter()
        .filter(|g| g.0 == Channel::Control)
        .collect();
    ensure!(
        got.len() == N as usize,
        "L-c1: {} of {N} control messages handed out",
        got.len()
    );
    for (j, &s) in sent.iter().enumerate() {
        let sends = sends_of(&log, 0, Dir::Up, Channel::Control, j as u64);
        ensure!(
            sends.len() == 2,
            "L-c1: message {j}: {} sends, not its lost first and one resend",
            sends.len()
        );
        let (first, resend) = (&sends[0].1, &sends[1].1);
        ensure!(
            first.outcome == Outcome::Lost && first.sent_ns == s,
            "L-c1: message {j}: its first send is not the planted loss"
        );
        let d = resend.sent_ns;
        let Some(x) = declared.iter().find(|x| x.pn == pn_of(first)) else {
            return Err(format!(
                "L-c1: message {j}: resent at {:.3} ms but never declared lost",
                ms(d)
            ));
        };
        ensure!(
            x.at_ns == d && (x.by_count || d >= s + x.delay_ns),
            "L-c1: message {j}: declared lost at {:.3} ms (threshold {:.3} ms), resent at {:.3} ms",
            ms(x.at_ns - s),
            ms(x.delay_ns),
            ms(d - s)
        );
        ensure!(
            (113_333_333..=153_333_334).contains(&(d - s)),
            "L-c1: message {j}: declared lost {:.3} ms after its send, outside [113.3, 153.3] ms",
            ms(d - s)
        );
        let Some(a) = arrival_of(resend) else {
            return Err(format!("L-c1: message {j}: its resend was lost"));
        };
        ensure!(
            (40 * MS..=60 * MS).contains(&(a - d)),
            "L-c1: message {j}: its resend took {:.3} ms",
            ms(a - d)
        );
        ensure!(
            got[j].1 == (j as u64).to_le_bytes() && got[j].2 == a,
            "L-c1: message {j}: handed out at {:.3} ms, not at its resend's arrival {:.3} ms",
            ms(got[j].2),
            ms(a)
        );
    }
    Ok(())
}

#[test]
fn lc1_a_lost_control_message_under_30_hz_datagrams_at_p2_is_declared_lost_and_resent() {
    for seed in [1, 2, 3] {
        lc1(L1Net::new(seed)).unwrap();
    }
}

/// L-c2 (C1' (c), (f)): P2's delays with no loss but the planted one; 4 s of 30 Hz datagrams both ways settle the
/// estimate, then the link is quiet; at 5 s one control message, its first packet lost, and nothing else. It is resent
/// at its PTO, smoothed RTT + max(4 x variance, 1 ms) + 25 ms after its send: about 155-160 ms (advisor-u3q.md:170).
/// The estimate is an average of samples 80 to 120 ms (two uniform ±10 ms one-way delays), so the PTO is near 100 +
/// 4 x 6.9 + 25 = 152.6 ms and spreads with the variance's 1/4-weight average: 60 seeds gave 138.3 to 175.6 ms, mean
/// 152.7 (measured at U4c's build). Each seed's lies in [130, 180] ms. Returns that PTO.
fn lc2(net: L1Net) -> Result<u64, String> {
    lose_up(&net, 0, piece(Channel::Control, 0, 0), 0);
    let mut rig = Rig::new(net, &[lossless(Profile::p2())]);
    for k in 0..120 {
        let t = MS + k * TICK;
        rig.poll(t);
        rig.tick(0, t);
    }
    let s = 5 * SEC;
    rig.poll(s);
    let pto = rig.est(0, Dir::Up).pto_ns();
    rig.send_on(0, CLIENT, Channel::Control, b"quiet", s);
    rig.poll(s + 3 * SEC);
    let ptos = rig.ptos(0, Dir::Up);
    ensure!(
        ptos.len() == 1 && ptos[0].at_ns == s + pto && ptos[0].count == 1,
        "L-c2: probe timeouts at {:?} ms, not one at the PTO, {:.3} ms",
        ptos.iter().map(|p| ms(p.at_ns - s)).collect::<Vec<_>>(),
        ms(pto)
    );
    ensure!(
        (130 * MS..=180 * MS).contains(&pto),
        "L-c2: a PTO of {:.3} ms at P2, outside [130, 180] ms",
        ms(pto)
    );
    let log = rig.entries();
    let sends = sends_of(&log, 0, Dir::Up, Channel::Control, 0);
    ensure!(
        sends.len() == 2 && sends[1].1.sent_ns == s + pto,
        "L-c2: {} sends, the resend not at the PTO",
        sends.len()
    );
    let a = arrival_of(&sends[1].1);
    let control: Vec<&Got> = rig.got[0][SERVER]
        .iter()
        .filter(|g| g.0 == Channel::Control)
        .collect();
    ensure!(
        a.is_some() && control == [&(Channel::Control, b"quiet".to_vec(), a.unwrap_or(0))],
        "L-c2: the message was not handed out once, at its resend's arrival"
    );
    Ok(pto)
}

#[test]
fn lc2_a_lost_message_on_a_quiet_link_at_p2_is_resent_at_its_pto() {
    let mut sum = 0;
    for seed in 1..=10 {
        let pto = lc2(L1Net::new(seed)).unwrap();
        println!("L-c2 seed {seed}: PTO {:.3} ms", ms(pto));
        sum += pto;
    }
    let mean = sum / 10;
    assert!(
        (145 * MS..=160 * MS).contains(&mean),
        "L-c2: a mean PTO of {:.3} ms over seeds 1 to 10 at P2, not about 152.6 ms",
        ms(mean)
    );
}

/// L-c3 (C1' (c), (d), (f)): B1's delays with no loss but its 1 s burst at 6 s; 30 Hz datagrams both ways until 5 s
/// settle the estimate, then quiet. Link 0: a control message at 5.98 s, which arrives inside the burst, so its
/// acknowledgement is lost; link 1: one at 6.1 s, inside the burst. On each the PTO doubles at every expiry while no
/// acknowledgement comes (P, 2P, 4P, ...), every probe inside the burst is lost, and the first one after it is
/// delivered, at its backed-off PTO; its acknowledgement resets the count.
fn lc3(net: L1Net) -> Result<(), String> {
    let p = lossless(Profile::b1(&[6 * SEC]));
    let mut rig = Rig::new(net, &[p.clone(), p]);
    for k in 0..150 {
        let t = MS + k * TICK;
        rig.poll(t);
        rig.tick(0, t);
        rig.tick(1, t);
    }
    rig.poll(5_900 * MS);
    let base = [rig.est(0, Dir::Up).pto_ns(), rig.est(1, Dir::Up).pto_ns()];
    let starts = [5_980 * MS, 6_100 * MS];
    rig.send_on(0, CLIENT, Channel::Control, b"before", starts[0]);
    rig.poll(starts[1]);
    rig.send_on(1, CLIENT, Channel::Control, b"inside", starts[1]);
    rig.poll(12 * SEC);
    let log = rig.entries();
    let mut handed = [0u64; 2];
    for link in 0..2usize {
        let sends = sends_of(&log, link as u32, Dir::Up, Channel::Control, 0);
        let ptos = rig.ptos(link as u32, Dir::Up);
        ensure!(
            !ptos.is_empty(),
            "L-c3: link {link}: no probe timeout fired"
        );
        ensure!(
            sends.len() == ptos.len() + 1,
            "L-c3: link {link}: {} sends for {} probe timeouts",
            sends.len(),
            ptos.len()
        );
        let mut due = starts[link] + base[link];
        for (i, pto) in ptos.iter().enumerate() {
            ensure!(
                pto.at_ns == due && pto.count == i as u32 + 1,
                "L-c3: link {link}: probe timeout {} at {:.3} ms, not at {:.3} ms (the PTO, {:.3} ms, doubles at each \
                 expiry)",
                i + 1,
                ms(pto.at_ns),
                ms(due),
                ms(base[link])
            );
            let probe = &sends[i + 1].1;
            let in_burst = (6 * SEC..7 * SEC).contains(&pto.at_ns);
            ensure!(
                probe.sent_ns == pto.at_ns && arrival_of(probe).is_some() != in_burst,
                "L-c3: link {link}: probe {} at {:.3} ms: inside the burst it is lost, after it delivered",
                i + 1,
                ms(probe.sent_ns)
            );
            due = pto.at_ns + (base[link] << (i + 1));
        }
        let last = &sends[ptos.len()].1;
        ensure!(
            ptos.len() >= 2 && last.sent_ns >= 7 * SEC,
            "L-c3: link {link}: the first probe after the burst is not a backed-off one"
        );
        ensure!(
            rig.pto_count(link, Dir::Up) == 0,
            "L-c3: link {link}: the probe's acknowledgement did not reset the count"
        );
        handed[link] = arrival_of(last).unwrap_or(0);
    }
    // Link 0's message itself arrived inside the burst, and only its acknowledgement was lost.
    let first0 = sends_of(&log, 0, Dir::Up, Channel::Control, 0)[0].1.clone();
    let a0 = arrival_of(&first0).unwrap_or(0);
    let ack0 = log.iter().find(|e| {
        e.link == 0
            && e.dir == Dir::Down
            && matches!(e.carries, Carries::Ack { acked, .. } if acked == pn_of(&first0))
    });
    ensure!(
        (6 * SEC..7 * SEC).contains(&a0) && ack0.is_some_and(|e| e.outcome == Outcome::Burst),
        "L-c3: link 0: its message did not arrive inside the burst with its acknowledgement lost"
    );
    let control = |link: usize| -> Vec<(Vec<u8>, u64)> {
        rig.got[link][SERVER]
            .iter()
            .filter(|g| g.0 == Channel::Control)
            .map(|g| (g.1.clone(), g.2))
            .collect()
    };
    let (c0, c1) = (control(0), control(1));
    ensure!(
        c0 == [(b"before".to_vec(), a0)] && c1 == [(b"inside".to_vec(), handed[1])],
        "L-c3: link 0's message not handed out once at its arrival {:.3} ms, or link 1's not once at its delivered \
         probe's {:.3} ms: {:?} ms, {:?} ms",
        ms(a0),
        ms(handed[1]),
        c0.iter().map(|g| ms(g.1)).collect::<Vec<_>>(),
        c1.iter().map(|g| ms(g.1)).collect::<Vec<_>>()
    );
    Ok(())
}

#[test]
fn lc3_a_1_s_burst_at_p2_doubles_the_pto_while_acknowledgements_are_lost() {
    for seed in [1, 2, 3] {
        lc3(L1Net::new(seed)).unwrap();
    }
}

/// L-c4 (C1' (a), (e), (f)). Link 0, P2's delays with no loss but the planted one, 30 Hz datagrams both ways: at 2 s the
/// client sends three control messages in one instant, 100 B, 3,000 B (three packets: 1,200, 1,200 and 600 B) whose
/// middle packet is lost, and 500 B. The big message waits for that resend, and the last waits behind it: in order
/// across messages. Link 1, P3 (5 %), 30 Hz datagrams both ways: 600 messages of 3,000 B down its full-state stream,
/// one every other tick. Each packet draws its own fate, so about 1 - 0.95^3 - 0.05^3 = 14.25 % of the messages have
/// pieces both lost and delivered; every message is handed out once, in order.
fn lc4(net: L1Net) -> Result<(), String> {
    lose_up(&net, 0, piece(Channel::Control, 1, 1), 0);
    let mut rig = Rig::new(net, &[lossless(Profile::p2()), Profile::p3()]);
    let big: Vec<u8> = (0..3_000u32).map(|i| (i % 251) as u8).collect();
    let msgs = [vec![7u8; 100], big, vec![9u8; 500]];
    const FULL: u64 = 600;
    let last = 2 * FULL + 90;
    for k in 0..last {
        let t = MS + k * TICK;
        rig.poll(t);
        rig.tick(0, t);
        rig.tick(1, t);
        if k == 60 {
            for m in &msgs {
                rig.send_on(0, CLIENT, Channel::Control, m, t);
            }
        }
        if k.is_multiple_of(2) && k / 2 < FULL {
            let mut b = vec![0u8; 3_000];
            b[..8].copy_from_slice(&(k / 2).to_le_bytes());
            rig.send_on(1, SERVER, Channel::FullState, &b, t);
        }
    }
    rig.poll(MS + last * TICK + 5 * SEC);
    let log = rig.entries();
    // Link 0.
    let sends = sends_of(&log, 0, Dir::Up, Channel::Control, 1);
    let firsts: Vec<&Fate> = sends
        .iter()
        .map(|s| &s.1)
        .filter(|e| e.retransmits == 0)
        .collect();
    let shape: Vec<(u32, u32)> = firsts
        .iter()
        .map(|e| match e.carries {
            Carries::Stream { part, parts, .. } => (part, parts),
            _ => (u32::MAX, 0),
        })
        .collect();
    ensure!(
        firsts.iter().map(|e| e.len).collect::<Vec<_>>() == [1_200, 1_200, 600]
            && shape == [(0, 3), (1, 3), (2, 3)],
        "L-c4: the 3,000 B message did not go as three packets of 1,200, 1,200 and 600 B"
    );
    ensure!(
        firsts[1].outcome == Outcome::Lost
            && arrival_of(firsts[0]).is_some()
            && arrival_of(firsts[2]).is_some(),
        "L-c4: its middle packet alone was not lost (each packet draws its own fate)"
    );
    // Its middle piece is resent; its other pieces may be too, spuriously: the jitter reorders the instant's packets, and
    // a later one's acknowledgement can declare an earlier one lost by the packet threshold, as in QUIC.
    let middle: Vec<&Fate> = sends
        .iter()
        .map(|s| &s.1)
        .filter(|e| matches!(e.carries, Carries::Stream { part: 1, .. }))
        .collect();
    let ready = piece_arrivals(&log, 0, Dir::Up, Channel::Control);
    let pieces = |m: u64| ready.get(&m).map(|x| x.1.clone()).unwrap_or_default();
    let big = pieces(1);
    let r = big.get(&1).copied().unwrap_or(u64::MAX);
    ensure!(
        middle.len() >= 2 && middle[1].retransmits == 1 && r < u64::MAX,
        "L-c4: the middle packet was not resent and delivered ({} sends)",
        middle.len()
    );
    let got: Vec<(Vec<u8>, u64)> = rig.got[0][SERVER]
        .iter()
        .filter(|g| g.0 == Channel::Control)
        .map(|g| (g.1.clone(), g.2))
        .collect();
    ensure!(
        got.len() == 3 && (0..3).all(|i| got[i].0 == msgs[i]),
        "L-c4: the three messages were not handed out once each, in order"
    );
    let due = handouts(&ready, 3);
    ensure!(
        got.iter().map(|g| g.1).collect::<Vec<_>>() == due,
        "L-c4: handed out at {:?} ms, not at {:?} ms (each message once all its pieces have arrived and every earlier one \
         is out)",
        got.iter().map(|g| ms(g.1)).collect::<Vec<_>>(),
        due.iter().map(|&x| ms(x)).collect::<Vec<_>>()
    );
    ensure!(
        due[1] == r && big.iter().all(|(&p, &a)| p == 1 || a < r) && due[2] >= r,
        "L-c4: the big message did not wait for its middle packet's resend ({:.3} ms), or the last not behind it: {:?} \
         ms",
        ms(r),
        due.iter().map(|&x| ms(x)).collect::<Vec<_>>()
    );
    let pns: Vec<u64> = (0..3)
        .flat_map(|m| sends_of(&log, 0, Dir::Up, Channel::Control, m))
        .filter(|s| s.1.retransmits == 0)
        .map(|s| pn_of(&s.1))
        .collect();
    ensure!(
        pns.windows(2).all(|w| w[0] < w[1]),
        "L-c4: the pieces' packet numbers {pns:?} do not follow their send order"
    );
    // Link 1.
    let mut mixed = 0u64;
    for m in 0..FULL {
        let lost: Vec<bool> = sends_of(&log, 1, Dir::Down, Channel::FullState, m)
            .iter()
            .filter(|s| s.1.retransmits == 0)
            .map(|s| arrival_of(&s.1).is_none())
            .collect();
        ensure!(
            lost.len() == 3,
            "L-c4: full-state message {m} went as {} packets",
            lost.len()
        );
        if lost.iter().any(|&x| x) && lost.iter().any(|&x| !x) {
            mixed += 1;
        }
    }
    let share = mixed as f64 / FULL as f64;
    ensure!(
        (0.08..=0.21).contains(&share),
        "L-c4: {mixed} of {FULL} messages ({share:.4}) had pieces both lost and delivered, not about 0.1425"
    );
    let full: Vec<&Got> = rig.got[1][CLIENT]
        .iter()
        .filter(|g| g.0 == Channel::FullState)
        .collect();
    ensure!(
        full.len() == FULL as usize
            && full
                .iter()
                .enumerate()
                .all(|(i, g)| g.1.len() == 3_000 && g.1[..8] == (i as u64).to_le_bytes()),
        "L-c4: the full-state messages were not handed out once each, whole and in order"
    );
    let due = handouts(
        &piece_arrivals(&log, 1, Dir::Down, Channel::FullState),
        FULL,
    );
    let at: Vec<u64> = full.iter().map(|g| g.2).collect();
    ensure!(
        at == due,
        "L-c4: a full-state message was not handed out when the in-order rule says (first difference at message {:?})",
        at.iter().zip(&due).position(|(a, b)| a != b)
    );
    Ok(())
}

/// Each message of one stream, from the log alone: its piece count and each piece's earliest arrival over its sends
/// (`u64::MAX` for a piece never delivered).
fn piece_arrivals(
    log: &[Fate],
    link: u32,
    dir: Dir,
    channel: Channel,
) -> BTreeMap<u64, (u32, BTreeMap<u32, u64>)> {
    let mut out: BTreeMap<u64, (u32, BTreeMap<u32, u64>)> = BTreeMap::new();
    for e in log.iter().filter(|e| e.link == link && e.dir == dir) {
        if let Carries::Stream {
            channel: c,
            message,
            part,
            parts,
            ..
        } = e.carries
            && c == channel
        {
            let m = out.entry(message).or_insert((parts, BTreeMap::new()));
            let a = m.1.entry(part).or_insert(u64::MAX);
            *a = (*a).min(arrival_of(e).unwrap_or(u64::MAX));
        }
    }
    out
}

/// When the in-order rule hands out each of a stream's first `n` messages: once all its pieces have arrived and every
/// earlier message is out (`u64::MAX`: never).
fn handouts(pieces: &BTreeMap<u64, (u32, BTreeMap<u32, u64>)>, n: u64) -> Vec<u64> {
    let mut out = Vec::new();
    let mut prev = 0;
    for m in 0..n {
        let ready = match pieces.get(&m) {
            Some((parts, p)) if p.len() == *parts as usize => {
                p.values().copied().max().unwrap_or(u64::MAX)
            }
            _ => u64::MAX,
        };
        prev = ready.max(prev);
        out.push(prev);
    }
    out
}

#[test]
fn lc4_a_3000_b_message_goes_as_3_packets_and_waits_for_the_lost_ones_resend_in_order() {
    for seed in [1, 2, 3] {
        lc4(L1Net::new(seed)).unwrap();
    }
}

/// Drives `rig` as an event-driven host does: it calls the net only at `next_event_ns` and at the script's own instants
/// (`script`, ascending, each once), up to `horizon`; at each, first the script's sends (`act`), then a poll of every
/// end. Returns the instants it called, and how often `next_event_ns` then named one not after the call.
fn drive(
    rig: &mut Rig,
    script: &[u64],
    horizon: u64,
    mut act: impl FnMut(&mut Rig, u64),
) -> (std::collections::BTreeSet<u64>, u64) {
    let mut calls = std::collections::BTreeSet::new();
    let mut stuck = 0;
    let mut own = script.iter().copied().peekable();
    let Some(mut t) = own.peek().copied() else {
        return (calls, 0);
    };
    loop {
        calls.insert(t);
        if own.peek() == Some(&t) {
            own.next();
            act(rig, t);
        }
        rig.poll(t);
        let mut event = rig.net.next_event_ns();
        if event.is_some_and(|x| x <= t) {
            stuck += 1;
            event = Some(t + 1);
        }
        match [event, own.peek().copied()].into_iter().flatten().min() {
            Some(x) if x <= horizon => t = x,
            _ => break,
        }
    }
    (calls, stuck)
}

/// Whether a host driven by `drive` was called for everything: each item handed out in the instant `due` says, and
/// every internal event up to `horizon` (each arrival and acknowledgement in the fate log, each PTO, each loss declared)
/// at an instant it called. `name` starts each failure.
fn woken(
    rig: &Rig,
    calls: &std::collections::BTreeSet<u64>,
    horizon: u64,
    name: &str,
    due: impl Fn(&Got) -> u64,
) -> Result<(), String> {
    for (link, (got, at)) in rig.got.iter().zip(&rig.at).enumerate() {
        for (g, &a) in got.iter().zip(at).flat_map(|(g, a)| g.iter().zip(a)) {
            let d = due(g);
            ensure!(
                a == d,
                "{name}: link {link}: an item that arrived at {:.6} ms was handed out at {:.6} ms, not in its instant, \
                 {:.6} ms",
                ms(g.2),
                ms(a),
                ms(d)
            );
        }
    }
    let net = rig.net.net.borrow();
    let arrivals = net.log.entries.iter().filter_map(arrival_of);
    let timers = net.trace.ptos.iter().map(|p| p.at_ns);
    let declared = net.trace.declared.iter().map(|d| d.at_ns);
    let skipped: std::collections::BTreeSet<u64> = arrivals
        .filter(|&x| x <= horizon)
        .chain(timers)
        .chain(declared)
        .filter(|x| !calls.contains(x))
        .collect();
    ensure!(
        skipped.is_empty(),
        "{name}: the host was never called at {} instants with an internal event, the first {:.6} ms",
        skipped.len(),
        skipped.first().map_or(0.0, |&x| ms(x))
    );
    Ok(())
}

/// What one run of L-c5's script gave.
struct Lc5 {
    /// The canonical fate log.
    log: Vec<u8>,
    /// What each end received.
    got: Vec<[Vec<Got>; 2]>,
    /// PTOs fired and losses declared.
    ptos: usize,
    declared: usize,
    /// The event-driven host: how often `next_event_ns` named an instant not after its call, and `woken`'s verdict.
    stuck: u64,
    woken: Result<(), String>,
}

/// L-c5's script: link 0 B1 (P2 and a burst at 2 s), link 1 P3; until 6 s, sends at whole milliseconds only: datagrams
/// both ways on both links every 33 ms (none on link 0 from 2 s to 3.5 s, so its PTOs fire), link 0's client a 50 B
/// control message every 100 ms and its server a 2,500 B full state every 500 ms, link 1's client a 3,000 B control
/// message every 250 ms and its server a 10 B one every 70 ms; then polls until 10 s. With `every_ms` it polls every
/// millisecond; else it is an event-driven host (`drive`): called only at `next_event_ns` and at its own sends.
fn lc5_run(net: L1Net, every_ms: bool) -> Lc5 {
    let mut rig = Rig::new(net, &[Profile::b1(&[2 * SEC]), Profile::p3()]);
    let sends_at = |m: u64| {
        (1..=6_000).contains(&m)
            && (m % 33 == 1 || m % 100 == 7 || m % 500 == 11 || m % 250 == 3 || m % 70 == 5)
    };
    let sends = |rig: &mut Rig, m: u64| {
        let t = m * MS;
        if !sends_at(m) {
            return;
        }
        if m % 33 == 1 {
            if !(2_000..3_500).contains(&m) {
                rig.tick(0, t);
            }
            rig.tick(1, t);
        }
        if m % 100 == 7 {
            rig.send_on(0, CLIENT, Channel::Control, &[3; 50], t);
        }
        if m % 500 == 11 {
            rig.send_on(0, SERVER, Channel::FullState, &[4; 2_500], t);
        }
        if m % 250 == 3 {
            rig.send_on(1, CLIENT, Channel::Control, &[5; 3_000], t);
        }
        if m % 70 == 5 {
            rig.send_on(1, SERVER, Channel::Control, &[6; 10], t);
        }
    };
    let end = 10 * SEC;
    let (mut stuck, mut woke) = (0, Ok(()));
    if every_ms {
        for m in 1..=10_000 {
            rig.poll(m * MS);
            sends(&mut rig, m);
        }
    } else {
        let script: Vec<u64> = (1..=6_000)
            .filter(|&m| sends_at(m))
            .map(|m| m * MS)
            .collect();
        let (calls, s) = drive(&mut rig, &script, end, |rig, t| sends(rig, t / MS));
        stuck = s;
        // A last poll, as the every-millisecond run's: it finds nothing when next_event_ns named everything.
        rig.poll(end);
        // Every delay here is at least 40 ms, so each item is due in the instant it arrived.
        woke = woken(&rig, &calls, end, "L-c5", |g| g.2);
    }
    let mut log = Vec::new();
    rig.net
        .with_fate_log(|l| l.for_each_canonical(|b| log.extend_from_slice(b)));
    let net = rig.net.net.borrow();
    let (ptos, declared) = (net.trace.ptos.len(), net.trace.declared.len());
    drop(net);
    Lc5 {
        log,
        got: std::mem::take(&mut rig.got),
        ptos,
        declared,
        stuck,
        woken: woke,
    }
}

/// L-c5 (the event-driven model): the same sends polled every millisecond and driven only at `next_event_ns` (and the
/// sends) give equal fate logs and equal arrivals, and the driven host is called at every internal event and hands each
/// item out in the instant it arrived.
fn lc5(make: &dyn Fn() -> L1Net) -> Result<(), String> {
    let a = lc5_run(make(), true);
    let b = lc5_run(make(), false);
    ensure!(
        a.ptos > 0 && a.declared > 0,
        "L-c5: the script fired {} probe timeouts and declared {} losses: it must exercise both",
        a.ptos,
        a.declared
    );
    ensure!(
        b.stuck == 0,
        "L-c5: next_event_ns named a past instant {} times",
        b.stuck
    );
    ensure!(
        a.log == b.log,
        "L-c5: the fate logs differ ({} and {} bytes) between polling every millisecond and at next_event_ns",
        a.log.len(),
        b.log.len()
    );
    ensure!(
        a.got == b.got,
        "L-c5: the ends received differently between polling every millisecond and at next_event_ns"
    );
    b.woken
}

#[test]
fn lc5_polling_every_millisecond_or_only_at_next_event_ns_gives_the_same_log_and_arrivals() {
    for seed in [1, 2, 3] {
        lc5(&|| L1Net::new(seed)).unwrap();
    }
}

/// One payload of the next_event_ns test: its own number and its send instant, so each hand-out's due instant is known.
fn stamp(n: u8, t: u64) -> Vec<u8> {
    let mut b = vec![n];
    b.extend_from_slice(&t.to_le_bytes());
    b
}

/// A stamped payload's send instant.
fn stamped(bytes: &[u8]) -> u64 {
    let mut t = [0u8; 8];
    t.copy_from_slice(&bytes[1..9]);
    u64::from_le_bytes(t)
}

/// A link with a fixed 100 ms round trip: no jitter, no loss, and the reorder step at `permille`.
fn fixed(permille: u32) -> Profile {
    Profile {
        name: "F",
        rtt_ns: 100 * MS,
        jitter_ns: 0,
        loss_ppm: 0,
        bursts_ns: Vec::new(),
        reorder: ReorderStep { permille },
    }
}

/// `next_event_ns`'s own check (round 1's M1): a host called only at `next_event_ns` and at its own sends (`drive`)
/// misses nothing. Link 0, a fixed 100 ms round trip: 30 Hz datagrams both ways until 1 s settle the estimate, then
/// quiet; at 2 s a control message, lost, and nothing else, so only its PTO resends it; at 3 s another, lost, and a
/// datagram 1 ms later, whose acknowledgement leaves it to the time threshold's timer, 3 s + 9/8 x 100 ms. Link 1, P0:
/// what arrives in the instant it was sent is due 1 ns later (a datagram up at 10 ms; at 20 ms a control message up and a
/// datagram down). Link 2, a fixed 100 ms round trip with the reorder step on every datagram: one up at 30 ms, held to its
/// own arrival; two at 500 ms and 510 ms, the first overtaken. Every item is handed out once, in its due instant
/// (its arrival, or 1 ns after its send), before `next_event_ns` says nothing is left; the host is called at every
/// internal event; and `next_event_ns` never names an instant at or before the call that asked it.
fn host_at_next_event(net: L1Net) -> Result<(), String> {
    lose_up(&net, 0, piece(Channel::Control, 0, 0), 0);
    lose_up(&net, 0, piece(Channel::Control, 1, 0), 0);
    let mut rig = Rig::new(net, &[fixed(0), Profile::p0(), fixed(1_000)]);
    // (instant, link, sending end, a control message rather than a datagram)
    let mut acts: Vec<(u64, usize, usize, bool)> = Vec::new();
    for k in 0..30 {
        let t = MS + k * TICK;
        acts.extend([(t, 0, CLIENT, false), (t, 0, SERVER, false)]);
    }
    acts.extend([
        (10 * MS, 1, CLIENT, false),
        (20 * MS, 1, CLIENT, true),
        (20 * MS, 1, SERVER, false),
        (30 * MS, 2, CLIENT, false),
        (500 * MS, 2, CLIENT, false),
        (510 * MS, 2, CLIENT, false),
        (2 * SEC, 0, CLIENT, true),
        (3 * SEC, 0, CLIENT, true),
        (3 * SEC + MS, 0, CLIENT, false),
    ]);
    acts.sort_by_key(|a| a.0);
    let mut script: Vec<u64> = acts.iter().map(|a| a.0).collect();
    script.dedup();
    // (link, receiving end, payload)
    let mut sent: Vec<(usize, usize, Vec<u8>)> = Vec::new();
    let horizon = 5 * SEC;
    let (calls, stuck) = drive(&mut rig, &script, horizon, |rig, t| {
        for (n, &(at, link, end, control)) in acts.iter().enumerate() {
            if at != t {
                continue;
            }
            let b = stamp(n as u8, t);
            if control {
                rig.send_on(link, end, Channel::Control, &b, t);
            } else {
                rig.send(link, end, &b, t);
            }
            sent.push((link, 1 - end, b));
        }
    });
    ensure!(
        stuck == 0,
        "next_event_ns: it named an instant at or before the call that asked it, {stuck} times"
    );
    for link in 0..3 {
        for end in [CLIENT, SERVER] {
            let mut want: Vec<&Vec<u8>> = sent
                .iter()
                .filter(|s| s.0 == link && s.1 == end)
                .map(|s| &s.2)
                .collect();
            let mut have: Vec<&Vec<u8>> = rig.got[link][end].iter().map(|g| &g.1).collect();
            want.sort();
            have.sort();
            ensure!(
                have == want,
                "next_event_ns: link {link}, end {end}: {} of {} items handed out when the host stopped at {:.3} ms \
                 (next_event_ns said nothing was left)",
                have.len(),
                want.len(),
                calls.last().map_or(0.0, |&x| ms(x))
            );
        }
    }
    let before = rig.entries().len();
    rig.poll(10 * SEC);
    ensure!(
        rig.entries().len() == before
            && rig.got.iter().flatten().map(Vec::len).sum::<usize>() == sent.len(),
        "next_event_ns: a poll after the host stopped found more to do"
    );
    woken(&rig, &calls, horizon, "next_event_ns", |g| {
        g.2.max(stamped(&g.1) + 1)
    })?;
    // The run reached each path: link 0's PTO and its time threshold, link 1's same-instant arrivals, link 2's holds.
    let ptos = rig.ptos(0, Dir::Up);
    let by_time = rig
        .declared(0, Dir::Up)
        .iter()
        .any(|d| !d.by_count && d.at_ns == 3 * SEC + 112_500_000);
    let log = rig.entries();
    let overtaken = log.iter().filter(|e| e.link == 2 && e.overtaken).count();
    let held_alone = log
        .iter()
        .any(|e| e.link == 2 && e.sent_ns == 30 * MS && arrival_of(e) == Some(80 * MS));
    ensure!(
        ptos.len() == 1
            && (2 * SEC..3 * SEC).contains(&ptos[0].at_ns)
            && by_time
            && rig.at[1][SERVER].contains(&(10 * MS + 1))
            && overtaken == 1
            && held_alone,
        "next_event_ns: the run missed a path (PTOs {:?} ms, time threshold {by_time}, overtaken {overtaken}, held \
         alone {held_alone})",
        ptos.iter().map(|p| ms(p.at_ns)).collect::<Vec<_>>()
    );
    Ok(())
}

#[test]
fn a_host_called_only_at_next_event_ns_and_its_sends_misses_nothing() {
    host_at_next_event(L1Net::new(1)).unwrap();
}

/// RFC 9002 §6.1.1's kPacketThreshold, exactly (round 1's M2). Link 0, a fixed 100 ms round trip, quiet: from 1 s five
/// one-packet control messages 1 ms apart, the first lost. The second's, third's and fourth's acknowledgements arrive at
/// 1.101, 1.102 and 1.103 s; the first is declared lost by count at the third later packet's, 1.103 s exactly: not at
/// the second's (a threshold of 2), and not by the time threshold at 1 s + 9/8 x 100 ms = 1.1125 s (no count rule).
fn packet_threshold(net: L1Net) -> Result<(), String> {
    lose_up(&net, 0, piece(Channel::Control, 0, 0), 0);
    let mut rig = Rig::new(net, &[fixed(0)]);
    for j in 0..5u64 {
        let t = SEC + j * MS;
        rig.poll(t);
        rig.send_on(0, CLIENT, Channel::Control, &j.to_le_bytes(), t);
    }
    rig.poll(3 * SEC);
    let log = rig.entries();
    let first = sends_of(&log, 0, Dir::Up, Channel::Control, 0);
    let p = pn_of(&first[0].1);
    let third = log
        .iter()
        .find(|e| {
            e.link == 0
                && e.dir == Dir::Down
                && matches!(e.carries, Carries::Ack { acked, .. } if acked == p + 3)
        })
        .and_then(arrival_of);
    ensure!(
        third == Some(SEC + 103 * MS),
        "packet threshold: the third later packet's acknowledgement arrived at {:?} ms, not 1,103 ms",
        third.map(ms)
    );
    let declared = rig.declared(0, Dir::Up);
    ensure!(
        declared.len() == 1
            && declared[0].pn == p
            && declared[0].by_count
            && Some(declared[0].at_ns) == third,
        "packet threshold: losses declared at {:?} ms (by count {:?}), not the first message's alone, by count, at the \
         third later packet's acknowledgement, 1,103 ms",
        declared.iter().map(|d| ms(d.at_ns)).collect::<Vec<_>>(),
        declared.iter().map(|d| d.by_count).collect::<Vec<_>>()
    );
    ensure!(
        first.len() == 2 && first[1].1.sent_ns == SEC + 103 * MS,
        "packet threshold: the first message was not resent at its declaration, 1,103 ms"
    );
    Ok(())
}

#[test]
fn the_packet_threshold_declares_a_loss_at_the_third_later_packets_acknowledgement() {
    packet_threshold(L1Net::new(1)).unwrap();
}

/// RFC 9002 §6.1.2's time threshold takes the latest RTT where it is above the smoothed one (an RTT step up that the
/// smoothed estimate has not caught up with; round 1's L1). Link 0, a 100 ms round trip with ±20 ms of jitter (50 ±
/// 10 ms each way) and no loss but the planted ones: 30 Hz datagrams both ways until 2 s settle the estimate, then
/// quiet. Ten trials, one a second from 3 s: a control message, lost, then a datagram in the same instant, whose
/// acknowledgement gives the one sample r before the loss rules run. The message is declared lost by the time threshold
/// exactly 9/8 x max(smoothed, r) after its send, the smoothed estimate taken after r: in the trials where r is above
/// it (each seed has some), 9/8 x r. Returns the trials' steps up and down.
fn time_threshold(net: L1Net) -> Result<(u32, u32), String> {
    const TRIALS: u64 = 10;
    for i in 0..TRIALS {
        lose_up(&net, 0, piece(Channel::Control, i, 0), 0);
    }
    let p = Profile {
        name: "J",
        rtt_ns: 100 * MS,
        jitter_ns: 40 * MS,
        loss_ppm: 0,
        bursts_ns: Vec::new(),
        reorder: ReorderStep::OFF,
    };
    let mut rig = Rig::new(net, &[p]);
    for k in 0..60 {
        let t = MS + k * TICK;
        rig.poll(t);
        rig.tick(0, t);
    }
    let (mut ups, mut downs) = (0, 0);
    for i in 0..TRIALS {
        let s = 3 * SEC + i * SEC;
        rig.poll(s);
        let mut est = rig.est(0, Dir::Up);
        rig.send_on(0, CLIENT, Channel::Control, b"lost", s);
        rig.send(0, CLIENT, b"later", s);
        rig.poll(s + SEC - MS);
        let log = rig.entries();
        let arrived = |e: &Fate| {
            e.link == 0 && e.dir == Dir::Up && e.sent_ns == s && e.carries == Carries::Datagram
        };
        let Some(a) = log.iter().find(|e| arrived(e)).and_then(arrival_of) else {
            return Err(format!(
                "time threshold: trial {i}: the datagram did not arrive"
            ));
        };
        let ack = log
            .iter()
            .find(|e| {
                e.link == 0
                    && e.dir == Dir::Down
                    && matches!(e.carries, Carries::Ack { .. })
                    && e.sent_ns == a
            })
            .and_then(arrival_of);
        let Some(r) = ack.map(|x| x - s) else {
            return Err(format!(
                "time threshold: trial {i}: no acknowledgement came back"
            ));
        };
        est.sample(r);
        let smoothed = est.smoothed.unwrap_or(INITIAL_RTT_NS);
        if r > smoothed {
            ups += 1;
        } else {
            downs += 1;
        }
        let want = (smoothed.max(r) * 9 / 8).max(GRANULARITY_NS);
        let sends = sends_of(&log, 0, Dir::Up, Channel::Control, i);
        let pn = sends.first().map_or(u64::MAX, |x| pn_of(&x.1));
        let d = rig.declared(0, Dir::Up).into_iter().find(|d| d.pn == pn);
        ensure!(
            d.is_some_and(|d| !d.by_count && d.at_ns == s + want && d.delay_ns == want)
                && sends.get(1).is_some_and(|x| x.1.sent_ns == s + want),
            "time threshold: trial {i}: declared lost {:?} ms after its send, not 9/8 x max({:.3}, {:.3}) = {:.3} ms",
            d.map(|d| ms(d.at_ns - s)),
            ms(smoothed),
            ms(r),
            ms(want)
        );
    }
    ensure!(
        ups > 0 && downs > 0 && rig.ptos(0, Dir::Up).is_empty(),
        "time threshold: the trials gave {ups} steps up and {downs} down (both needed), PTOs {}",
        rig.ptos(0, Dir::Up).len()
    );
    Ok((ups, downs))
}

#[test]
fn the_time_threshold_takes_the_latest_rtt_when_it_is_above_the_smoothed_one() {
    for seed in [1, 2, 3] {
        let (ups, downs) = time_threshold(L1Net::new(seed)).unwrap();
        println!("time threshold seed {seed}: {ups} steps up, {downs} down");
    }
}

/// L-c6 (the event order): at one instant, internal events come before application sends, by (time, fate-log index).
/// Link 0, P2's delays with no loss but the planted one: datagrams from 1 s to 3 s settle the estimate, then quiet
/// (their acknowledgements are all back by 3.1 s, so nothing moves the timer); at 4 s a control message, lost; at its
/// PTO's instant exactly the client sends another. Link 1, P0: at 10 ms the client
/// sends a datagram, which arrives in that instant, and the server end sends one in the same instant. Links 2 and 3, a
/// fixed 100 ms round trip: at 20 ms link 3's client sends, then link 2's; both datagrams arrive at 70 ms, and the next
/// call (at 1 s) applies both. The probe goes before the application's message (a lower fate-log index and packet
/// number); link 1's acknowledgement before the server's datagram; link 3's acknowledgement before link 2's (fate-log
/// order, not link order).
fn lc6(net: L1Net) -> Result<(), String> {
    lose_up(&net, 0, piece(Channel::Control, 0, 0), 0);
    let p0 = Profile::p0();
    let mut rig = Rig::new(net, &[lossless(Profile::p2()), p0, fixed(0), fixed(0)]);
    rig.send(1, CLIENT, b"A", 10 * MS);
    rig.send(1, SERVER, b"B", 10 * MS);
    rig.send(3, CLIENT, b"Y", 20 * MS);
    rig.send(2, CLIENT, b"X", 20 * MS);
    rig.poll(20 * MS);
    for k in 0..60 {
        let t = SEC + k * TICK;
        rig.poll(t);
        rig.tick(0, t);
    }
    let s = 4 * SEC;
    rig.poll(s);
    rig.send_on(0, CLIENT, Channel::Control, b"M", s);
    let pto = rig.est(0, Dir::Up).pto_ns();
    let Some((at, _)) = rig.timer(0, Dir::Up) else {
        return Err("L-c6: no timer armed for the lost message".into());
    };
    ensure!(
        at == s + pto,
        "L-c6: the timer is armed for {:.3} ms, not the PTO {:.3} ms",
        ms(at - s),
        ms(pto)
    );
    rig.poll(at - 1);
    ensure!(
        rig.timer(0, Dir::Up).is_some_and(|x| x.0 == at),
        "L-c6: the timer moved from {:.3} ms before its instant",
        ms(at)
    );
    rig.send_on(0, CLIENT, Channel::Control, b"N", at);
    rig.poll(at + 2 * SEC);
    let log = rig.entries();
    let m = sends_of(&log, 0, Dir::Up, Channel::Control, 0);
    let n = sends_of(&log, 0, Dir::Up, Channel::Control, 1);
    ensure!(
        m.len() >= 2 && m[1].1.sent_ns == at,
        "L-c6: the PTO due at {:.3} ms did not resend message 0 in its instant",
        ms(at)
    );
    ensure!(
        !n.is_empty() && m[1].0 < n[0].0 && pn_of(&m[1].1) < pn_of(&n[0].1),
        "L-c6: at {:.3} ms the application's send went before the PTO's resend",
        ms(at)
    );
    let kinds = |link: u32| -> Vec<(u8, Dir, u64)> {
        log.iter()
            .filter(|e| e.link == link)
            .map(|e| {
                let code = match e.carries {
                    Carries::Datagram => 0,
                    Carries::Stream { .. } => 1,
                    Carries::Ack { .. } => ACK_CODE,
                };
                (code, e.dir, e.sent_ns)
            })
            .collect()
    };
    let l1 = kinds(1);
    ensure!(
        l1.len() >= 3
            && l1[..3]
                == [
                    (0, Dir::Up, 10 * MS),
                    (ACK_CODE, Dir::Down, 10 * MS),
                    (0, Dir::Down, 10 * MS)
                ],
        "L-c6: link 1's entries at 10 ms are {:?}: the acknowledgement of the arrival goes before the server's send",
        &l1[..l1.len().min(3)]
    );
    let ack_index = |link: u32| {
        log.iter().position(|e| {
            e.link == link && matches!(e.carries, Carries::Ack { .. }) && e.sent_ns == 70 * MS
        })
    };
    let (i2, i3) = (ack_index(2), ack_index(3));
    ensure!(
        i2.is_some() && i3.is_some() && i3 < i2,
        "L-c6: the two arrivals at 70 ms were not acknowledged in fate-log order, link 3's first (links 2 and 3 at \
         {i2:?}, {i3:?})"
    );
    Ok(())
}

#[test]
fn lc6_at_one_instant_internal_events_come_before_application_sends() {
    for seed in [1, 2, 3] {
        lc6(L1Net::new(seed)).unwrap();
    }
}

/// L-c7 (C2). Link 0, P4's delays with no loss: a datagram up every millisecond for 4 s, nothing down, so packets are
/// overtaken and acknowledgements reordered all the time. Read from the fate log alone: an RTT sample comes exactly from
/// each acknowledgement of a packet that was the newest to arrive, when no later-sent acknowledgement reached the sender
/// before it, valued its arrival minus its packet's send. Link 1, a 100 ms round trip with no jitter or loss: 30 Hz
/// datagrams both ways until 3 s, then quiet; at 4 s a control message M1, lost, and at 4.01 s M2, held behind it: M2
/// samples one round trip, 100 ms, at 4.11 s; M1 is declared lost at 4.1125 s (9/8 x 100 ms) and both are handed out
/// at its resend's arrival, 4.1625 s.
fn lc7(net: L1Net) -> Result<(), String> {
    lose_up(&net, 1, piece(Channel::Control, 0, 0), 0);
    let zero = Profile {
        name: "Z",
        rtt_ns: 100 * MS,
        jitter_ns: 0,
        loss_ppm: 0,
        bursts_ns: Vec::new(),
        reorder: ReorderStep::OFF,
    };
    let mut rig = Rig::new(net, &[lossless(Profile::p4()), zero]);
    for m in 1..=5_000u64 {
        let t = m * MS;
        rig.poll(t);
        if m <= 4_000 {
            rig.send(0, CLIENT, &[0; 20], t);
        }
        if m % 33 == 1 && m < 3_000 {
            rig.tick(1, t);
        }
        if m == 4_000 {
            rig.send_on(1, CLIENT, Channel::Control, b"M1", t);
        }
        if m == 4_010 {
            rig.send_on(1, CLIENT, Channel::Control, b"M2", t);
        }
    }
    rig.poll(8 * SEC);
    let log = rig.entries();
    // Link 0, from the log: the up datagrams in arrival order; which were the newest to arrive.
    let mut up: Vec<(u64, usize, u64, u64)> = log
        .iter()
        .enumerate()
        .filter(|(_, e)| e.link == 0 && e.dir == Dir::Up && e.carries == Carries::Datagram)
        .filter_map(|(i, e)| arrival_of(e).map(|a| (a, i, e.seq, e.sent_ns)))
        .collect();
    let sent: BTreeMap<u64, u64> = up.iter().map(|x| (x.2, x.3)).collect();
    up.sort_unstable();
    let mut newest = std::collections::BTreeSet::new();
    let mut highest: Option<u64> = None;
    for x in &up {
        if highest.is_none_or(|h| x.2 > h) {
            highest = Some(x.2);
            newest.insert(x.2);
        }
    }
    // Their acknowledgements, in send order (each packet's number is its datagram index: link 0 sends nothing else up).
    let acks: Vec<(u64, u64)> = log
        .iter()
        .filter(|e| e.link == 0 && e.dir == Dir::Down)
        .filter_map(|e| match e.carries {
            Carries::Ack { acked, .. } => arrival_of(e).map(|a| (acked, a)),
            _ => None,
        })
        .collect();
    let mut later_min = vec![u64::MAX; acks.len() + 1];
    for k in (0..acks.len()).rev() {
        later_min[k] = later_min[k + 1].min(acks[k].1);
    }
    let expected: Vec<(u64, u64, u64)> = acks
        .iter()
        .enumerate()
        .filter(|(k, (pn, a))| newest.contains(pn) && *a <= later_min[k + 1])
        .map(|(_, &(pn, a))| (pn, a, a - sent[&pn]))
        .collect();
    let samples: Vec<(u64, u64, u64)> = rig
        .samples(0, Dir::Up)
        .iter()
        .map(|s| (s.pn, s.at_ns, s.sample_ns))
        .collect();
    ensure!(
        newest.len() < up.len() / 2 && expected.len() < newest.len() && !expected.is_empty(),
        "L-c7: the run did not overtake packets and reorder acknowledgements ({} newest of {}, {} samples due)",
        newest.len(),
        up.len(),
        expected.len()
    );
    ensure!(
        samples == expected,
        "L-c7: link 0 took {} RTT samples where the newest-packet rule gives {} (first difference at {:?})",
        samples.len(),
        expected.len(),
        samples.iter().zip(&expected).position(|(a, b)| a != b)
    );
    // Link 1.
    let s = 4 * SEC;
    let m2 = sends_of(&log, 1, Dir::Up, Channel::Control, 1);
    let late: Vec<Sample> = rig
        .samples(1, Dir::Up)
        .into_iter()
        .filter(|x| x.at_ns >= s)
        .collect();
    ensure!(
        late.iter().all(|x| x.sample_ns <= 100 * MS)
            && late.iter().any(|x| x.at_ns == s + 110 * MS
                && x.sample_ns == 100 * MS
                && Some(x.pn) == m2.first().map(|e| pn_of(&e.1))),
        "L-c7: link 1's samples after 4 s are {:?} ms: M2, held behind M1, samples one round trip, 100 ms, at 4.11 s",
        late.iter()
            .map(|x| (ms(x.at_ns), ms(x.sample_ns)))
            .collect::<Vec<_>>()
    );
    let handed = s + 162_500_000;
    ensure!(
        rig.got[1][SERVER]
            .iter()
            .filter(|g| g.0 == Channel::Control)
            .map(|g| (g.1.clone(), g.2))
            .collect::<Vec<_>>()
            == [(b"M1".to_vec(), handed), (b"M2".to_vec(), handed)],
        "L-c7: M1 and M2 were not handed out in order at M1's resend's arrival, 4.1625 s"
    );
    Ok(())
}

#[test]
fn lc7_only_the_newest_packets_acknowledgement_samples_and_a_held_message_samples_one_round_trip() {
    for seed in [1, 2, 3] {
        lc7(L1Net::new(seed)).unwrap();
    }
}

/// L-c9 (RFC 9002's starting state and kGranularity). Link 0, P2: nothing sent before a control message at 1 s, its
/// first packet lost: resent at exactly 1 s + 1,024 ms. Links 1 and 2, P0: 30 Hz datagrams both ways from 1 s to 2 s
/// set the estimate to 0, then quiet. Link 1: a control message at 3 s, lost: resent at its PTO, exactly 26 ms later
/// (kGranularity + max_ack_delay). Link 2: a control message at 3 s, lost, then a datagram 0.5 ms later, acknowledged
/// in its instant: the message is declared lost exactly 1 ms after its send (kGranularity floors the time threshold),
/// and resent then.
fn lc9(net: L1Net) -> Result<(), String> {
    for link in 0..3 {
        lose_up(&net, link, piece(Channel::Control, 0, 0), 0);
    }
    let p0 = Profile::p0();
    let mut rig = Rig::new(net, &[lossless(Profile::p2()), p0.clone(), p0]);
    rig.poll(SEC);
    ensure!(
        rig.ends[0][CLIENT].rtt_ns().is_none(),
        "L-c9: an RTT estimate before any sample"
    );
    rig.send_on(0, CLIENT, Channel::Control, b"first", SEC);
    for k in 0..30 {
        let t = SEC + MS + k * TICK;
        rig.poll(t);
        rig.tick(1, t);
        rig.tick(2, t);
    }
    let s = 3 * SEC;
    rig.poll(s);
    rig.send_on(1, CLIENT, Channel::Control, b"p0", s);
    rig.send_on(2, CLIENT, Channel::Control, b"p0", s);
    rig.poll(s + MS / 2);
    rig.send(2, CLIENT, b"later", s + MS / 2);
    rig.poll(4 * SEC);
    let log = rig.entries();
    let ptos0 = rig.ptos(0, Dir::Up);
    let s0 = sends_of(&log, 0, Dir::Up, Channel::Control, 0);
    ensure!(
        ptos0.first().is_some_and(|p| p.at_ns == SEC + 1_024 * MS)
            && s0.get(1).is_some_and(|e| e.1.sent_ns == SEC + 1_024 * MS),
        "L-c9: link 0's first PTO at {:?} ms after the send, not 1,024 ms",
        ptos0.first().map(|p| ms(p.at_ns - SEC))
    );
    let ptos1 = rig.ptos(1, Dir::Up);
    ensure!(
        rig.est(1, Dir::Up).smoothed == Some(0)
            && ptos1.first().is_some_and(|p| p.at_ns == s + 26 * MS),
        "L-c9: link 1's PTO at {:?} ms with a zero estimate, not 26 ms",
        ptos1.first().map(|p| ms(p.at_ns - s))
    );
    let d2 = rig.declared(2, Dir::Up);
    let s2 = sends_of(&log, 2, Dir::Up, Channel::Control, 0);
    ensure!(
        d2.iter().any(|d| d.pn == pn_of(&s2[0].1)
            && d.at_ns == s + MS
            && !d.by_count
            && d.delay_ns == MS)
            && s2.get(1).is_some_and(|e| e.1.sent_ns == s + MS)
            && rig.ptos(2, Dir::Up).is_empty(),
        "L-c9: link 2's message was not declared lost and resent 1 ms after its send"
    );
    Ok(())
}

#[test]
fn lc9_a_message_lost_before_any_rtt_sample_is_resent_at_1024_ms_and_kgranularity_floors_both_timers()
 {
    lc9(L1Net::new(1)).unwrap();
}

#[cfg(feature = "bites")]
mod bites {
    use super::*;

    fn bitten(id: &str) -> L1Net {
        L1Net::with_model_bite(1, id).expect(id)
    }

    /// The link model's bites are the unit tests' own: npt-lab and the marks reach PM-0001-3's four only.
    #[test]
    fn the_model_bites_are_chosen_only_here() {
        assert_eq!(MODEL_BITE_IDS.len(), 8);
        for (id, _) in MODEL_BITE_IDS {
            assert!(id.starts_with("npt-l1/") && L1Net::with_model_bite(1, id).is_some());
            assert!(L1Net::with_bite(1, id).is_none() && !L1Net::BITES.contains(&id));
        }
        assert!(L1Net::with_model_bite(1, "PM-0001-3/rtt-each-way").is_none());
    }

    /// The check fails on the check the bite is for: its L-c test (`lc`, as `L-cN: `) and words of that check's message
    /// (`check`), not on another check of the same test.
    fn fails<T: std::fmt::Debug>(r: Result<T, String>, lc: &str, check: &str) {
        let e = r.expect_err("the planted defect passed");
        assert!(
            e.starts_with(&format!("{lc}: ")) && e.contains(check),
            "failed on another check than {lc}'s \"{check}\": {e}"
        );
        println!("{e}");
    }

    #[test]
    fn bite_separate_spaces_fails_lc1() {
        let r = lc1(bitten("npt-l1/separate-spaces"));
        fails(r, "L-c1", "probe timeouts fired");
    }

    #[test]
    fn bite_no_pto_doubling_fails_lc3() {
        let r = lc3(bitten("npt-l1/no-pto-doubling"));
        fails(r, "L-c3", "doubles at each expiry");
    }

    #[test]
    fn bite_acks_in_bursts_fails_lc3() {
        let r = lc3(bitten("npt-l1/acks-in-bursts"));
        fails(r, "L-c3", "no probe timeout fired");
    }

    #[test]
    fn bite_one_draw_per_message_fails_lc4() {
        let r = lc4(bitten("npt-l1/one-draw-per-message"));
        fails(r, "L-c4", "its middle packet alone was not lost");
    }

    #[test]
    fn bite_events_at_polls_fails_lc5() {
        let r = lc5(&|| bitten("npt-l1/events-at-polls"));
        fails(r, "L-c5", "the fate logs differ");
    }

    #[test]
    fn bite_sends_before_events_fails_lc6() {
        let r = lc6(bitten("npt-l1/sends-before-events"));
        fails(r, "L-c6", "did not resend message 0 in its instant");
    }

    #[test]
    fn bite_every_packet_sampled_fails_lc7() {
        let r = lc7(bitten("npt-l1/every-packet-sampled"));
        fails(r, "L-c7", "where the newest-packet rule gives");
    }

    #[test]
    fn bite_stream_sample_in_order_fails_lc7() {
        let r = lc7(bitten("npt-l1/stream-sample-in-order"));
        fails(r, "L-c7", "link 1's samples after 4 s");
    }

    /// u4-build-plan.md §4.11 (T10): a bite is chosen per instance, never by a static, so a bitten net and a clean one
    /// run side by side on two threads of one process.
    #[test]
    fn a_bitten_net_and_a_clean_one_run_side_by_side_on_two_threads() {
        std::thread::scope(|s| {
            let clean = s.spawn(|| lc3(L1Net::new(1)));
            let bitten = s.spawn(|| lc3(bitten("npt-l1/no-pto-doubling")));
            assert_eq!(clean.join().expect("clean thread"), Ok(()));
            assert!(bitten.join().expect("bitten thread").is_err());
        });
    }
}
