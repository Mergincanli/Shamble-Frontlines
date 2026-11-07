//! S13's tests on simulated time (advisor-u3q.md §(1)): (a) a single loss under 30 Hz traffic, (b) a tail loss on a
//! quiet stream, (c) a lost resend, (d) the doubling and its reset, (e) the 1 s burst swept over 1,000 phases (the
//! docs PR's number), (f) the stall count against the fate draws; with S13's bites (the old constant in place of RACK,
//! no doubling, loss off), plus determinism and the write budget. Then one focused test per kernel rule of v2 and v3,
//! each with a bite (a planted defect, `Variant`, that the test catches): Karn's rule, the echo's sample, the echo's
//! source (rcv_wup), the RTO's marking and window, the old-duplicate discard, D-SACK at both ends, the undo by
//! timestamps, F-RTO, PRR and the window at Recovery's exit, the probe's own reduction, Cubic and slow start, the
//! Disorder state, and the handshake's rules.

use super::*;

const MS: u64 = 1_000_000;
const SEC: u64 = 1_000_000_000;
const TICK: u64 = SEC / 30;
/// A snapshot as the model counts it: 1,024 B of payload, a 4 B WebSocket header and TLS 1.3's 22 B (S0).
const SNAP: u64 = 1_050;
/// A client input: 60 B of payload and the same overheads.
const INPUT: u64 = 86;

fn quiet(mut p: Profile) -> Profile {
    p.loss_ppm = 0;
    p
}

/// One end writing one message every tick from `next` to `stop`.
struct Feed {
    end: usize,
    size: u64,
    next: u64,
    stop: u64,
}

/// Steps the model event by event to `until`, writing the feeds' messages on their ticks.
fn run(m: &mut TcpModel, feeds: &mut [Feed], until: u64) {
    loop {
        let write = feeds
            .iter()
            .filter(|f| f.next <= f.stop)
            .map(|f| f.next)
            .min();
        let t = match (write, m.next_event()) {
            (Some(a), Some(b)) => a.min(b),
            (Some(a), None) => a,
            (None, Some(b)) => b,
            (None, None) => break,
        };
        if t > until {
            break;
        }
        m.advance(t);
        for f in feeds.iter_mut() {
            if f.next == t && f.next <= f.stop {
                m.write(f.end, f.size, t);
                f.next += TICK;
            }
        }
    }
    m.advance(until);
}

/// The send times of one segment's transmissions, in order.
fn sends_of(m: &TcpModel, end: usize, start: u64) -> Vec<u64> {
    m.sent_log
        .iter()
        .filter(|&&(e, s, _, _, _)| e == end && s == start)
        .map(|&(_, _, _, t, _)| t)
        .collect()
}

fn settled(p: &Profile, seed: u64, variant: Variant) -> TcpModel {
    settled_with(p, seed, variant, TcpConfig::default())
}

fn settled_with(p: &Profile, seed: u64, variant: Variant, cfg: TcpConfig) -> TcpModel {
    let mut m = TcpModel::with_config(p, seed, 0, cfg).with_variant(variant, p);
    let mut feeds = [Feed {
        end: 0,
        size: SNAP,
        next: TICK,
        stop: 3 * SEC,
    }];
    run(&mut m, &mut feeds, 5 * SEC);
    m
}

/// (a): the 31st snapshot's first transmission lost; returns the stall and the counters.
fn single_loss(seed: u64, variant: Variant) -> (u64, Counters) {
    let p = quiet(Profile::p2());
    let mut m = TcpModel::new(&p, seed, 0).with_variant(variant, &p);
    m.forced.push((0, 30 * SNAP, 1));
    let mut feeds = [Feed {
        end: 0,
        size: SNAP,
        next: TICK,
        stop: 3 * SEC,
    }];
    run(&mut m, &mut feeds, 6 * SEC);
    assert_eq!(
        m.delivered(1),
        m.ends[0].snd.write_seq,
        "every byte arrives in order"
    );
    let c = m.counters(0);
    (c.stall_ns_max, c)
}

#[test]
fn a_a_single_loss_is_repaired_by_rack_one_round_trip_and_one_spacing_later() {
    for seed in 1..=5 {
        let (stall, c) = single_loss(seed, Variant::Faithful);
        assert_eq!(
            (c.first_lost, c.stalls, c.repairs_rack),
            (1, 1, 1),
            "seed {seed}: {c:?}"
        );
        assert_eq!(c.repairs_probe + c.repairs_timeout, 0, "seed {seed}: {c:?}");
        // The spacing, the next segment's trip and its acknowledgement's, then the resend's, less the lost one's:
        // 33.3 + [40, 60] x 3 - [40, 60] ms.
        assert!(
            (93 * MS..=174 * MS).contains(&stall),
            "seed {seed}: stall {} ms",
            stall / MS
        );
    }
}

#[test]
fn bite_the_old_constant_in_place_of_rack_fails_a() {
    let (stall, c) = single_loss(1, Variant::ConstantRepair);
    assert_eq!(c.repairs_rack, 0);
    assert!(
        !(93 * MS..=174 * MS).contains(&stall),
        "the constant repairs at {} ms",
        stall / MS
    );
}

#[test]
fn b_a_tail_loss_on_a_quiet_stream_waits_for_the_probe_capped_by_the_rto() {
    for seed in 1..=3 {
        let p = quiet(Profile::p2());
        let mut m = settled(&p, seed, Variant::Faithful);
        let start = m.ends[0].snd.write_seq;
        m.forced.push((0, start, 1));
        let srtt = m.ends[0].snd.rtt.srtt_us() * US;
        let rto = m.ends[0].snd.rto_ns();
        m.write(0, SNAP, 5 * SEC);
        run(&mut m, &mut [], 10 * SEC);
        let c = m.counters(0);
        assert_eq!(
            (c.repairs_probe, c.repairs_rack, c.repairs_timeout),
            (1, 0, 0),
            "seed {seed}: {c:?}"
        );
        let probe = (2 * srtt + 200 * MS).min(rto);
        let sends = sends_of(&m, 0, start);
        assert_eq!(sends.len(), 2);
        assert_eq!(
            sends[1] - sends[0],
            probe,
            "seed {seed}: the probe at min(2 x srtt + 200 ms, the RTO)"
        );
        assert!(
            probe < 2 * srtt + 200 * MS,
            "seed {seed}: capped by the RTO ({} ms)",
            rto / MS
        );
    }
}

#[test]
fn c_a_lost_resend_goes_to_the_rto_one_full_timeout_after_the_probe() {
    let p = quiet(Profile::p2());
    let mut m = settled(&p, 2, Variant::Faithful);
    let start = m.ends[0].snd.write_seq;
    m.forced.extend([(0, start, 1), (0, start, 2)]);
    let rto = m.ends[0].snd.rto_ns();
    m.write(0, SNAP, 5 * SEC);
    run(&mut m, &mut [], 12 * SEC);
    let c = m.counters(0);
    assert_eq!((c.repairs_probe, c.repairs_timeout), (1, 1), "{c:?}");
    let sends = sends_of(&m, 0, start);
    assert_eq!(sends.len(), 3);
    assert_eq!(sends[2] - sends[1], rto, "the RTO restarts from the probe");
    assert!(
        (250 * MS..=350 * MS).contains(&rto),
        "about srtt + 200 ms at P2: {} ms",
        rto / MS
    );
}

/// (d): the head's first four transmissions lost. Returns the gaps between its sends, and the backoff once the episode
/// is over and again after a new message's acknowledgement.
fn doubling(variant: Variant, timestamps: bool) -> (Vec<u64>, u32, u32) {
    let p = quiet(Profile::p2());
    let cfg = TcpConfig {
        timestamps,
        ..TcpConfig::default()
    };
    let mut m = settled_with(&p, 3, variant, cfg);
    let start = m.ends[0].snd.write_seq;
    m.forced.extend((1..=4).map(|a| (0, start, a)));
    m.write(0, SNAP, 5 * SEC);
    run(&mut m, &mut [], 12 * SEC);
    let sends = sends_of(&m, 0, start);
    let gaps: Vec<u64> = sends.windows(2).map(|w| w[1] - w[0]).collect();
    let after_episode = m.ends[0].snd.backoff;
    // A new message after the episode: its acknowledgement is a valid sample (never resent), which resets the backoff.
    m.write(0, SNAP, 12 * SEC);
    run(&mut m, &mut [], 14 * SEC);
    (gaps, after_episode, m.ends[0].snd.backoff)
}

#[test]
fn d_the_rto_doubles_on_each_expiry_and_a_valid_sample_resets_it() {
    for timestamps in [false, true] {
        let (gaps, after_episode, after_new) = doubling(Variant::Faithful, timestamps);
        assert_eq!(gaps.len(), 4, "the probe, then three timeouts: {gaps:?}");
        assert_eq!(gaps[2], 2 * gaps[1], "{gaps:?}");
        assert_eq!(gaps[3], 2 * gaps[2], "{gaps:?}");
        if timestamps {
            // The resent head's acknowledgement echoes its own timestamp: tcp_ack_update_rtt takes that sample
            // (tcp_input.c:3155-3157) and resets the backoff at once (:3172).
            assert_eq!(after_episode, 0, "the echo's sample resets the backoff");
        } else {
            // Without timestamps the resent head gives no sample (Karn), so the backoff stays until the new message.
            assert_eq!(after_episode, 3, "no sample from a resend");
            assert_eq!(after_new, 0, "the next valid RTT sample resets the backoff");
        }
    }
}

#[test]
fn bite_no_doubling_fails_d() {
    let (gaps, _, _) = doubling(Variant::NoDoubling, false);
    assert_ne!(gaps[3], 2 * gaps[2], "{gaps:?}");
}

/// One phase of (e): a B1 burst at `burst_ns`, the room writing one snapshot a tick (the newest only, once the write
/// budget is full) and the client one input a tick. Returns, from the link's return, the time to the in-order delivery
/// of the first snapshot written at or after it, and to the first byte after it.
fn burst_phase(seed: u64, burst_ns: u64, variant: Variant) -> (u64, u64) {
    let (f, b, _) = burst_phase_model(seed, burst_ns, variant, false);
    (f, b)
}

/// The same, also returning the connection at the phase's end (the diagnostic reads its log).
fn burst_phase_model(
    seed: u64,
    burst_ns: u64,
    variant: Variant,
    trace: bool,
) -> (u64, u64, TcpModel) {
    let p = Profile::b1(&[burst_ns]);
    let mut m = TcpModel::new(&p, seed, 0).with_variant(variant, &p);
    let back = burst_ns + SEC;
    let until = back + 10 * SEC;
    let input_phase = (seed * 7_919) % TICK;
    let (mut tick, mut input) = (TICK, TICK + input_phase);
    let mut pending = false;
    let mut fresh_end: Option<u64> = None;
    let mut at_back: Option<u64> = None;
    let (mut first_fresh, mut first_byte) = (None, None);
    loop {
        let t = [Some(tick), Some(input), m.next_event()]
            .into_iter()
            .flatten()
            .min()
            .unwrap_or(until);
        if t > until {
            break;
        }
        if trace && m.trace.is_none() && t + 1_300 * MS >= back {
            m.trace = Some(back);
        }
        m.advance(t);
        if t >= back && at_back.is_none() {
            at_back = Some(m.delivered(1));
        }
        if t == tick {
            pending = true;
            tick += TICK;
        }
        if t == input {
            m.write(1, INPUT, t);
            input += TICK;
        }
        if pending && m.free_budget(0) >= SNAP {
            m.write(0, SNAP, t);
            pending = false;
            if t >= back && fresh_end.is_none() {
                fresh_end = Some(m.ends[0].snd.write_seq);
            }
        }
        if let (Some(b), None) = (at_back, first_byte)
            && m.delivered(1) > b
        {
            first_byte = Some(t - back);
        }
        if let (Some(e), None) = (fresh_end, first_fresh)
            && m.delivered(1) >= e
        {
            first_fresh = Some(t - back);
        }
        if first_fresh.is_some() && first_byte.is_some() {
            break;
        }
    }
    (
        first_fresh.expect("a fresh snapshot arrives within 10 s"),
        first_byte.expect("a byte arrives within 10 s"),
        m,
    )
}

/// 1,000 phases: the burst starts 5 s to 7 s into the run (spread by the golden ratio), each with its own seed.
fn sweep(variant: Variant, phases: u64) -> (Vec<u64>, Vec<u64>) {
    let (mut fresh, mut byte) = (Vec::new(), Vec::new());
    for i in 0..phases {
        let burst = 5 * SEC + ((i * 618_033_989) % SEC) * 2;
        let (f, b) = burst_phase(1_000 + i, burst, variant);
        fresh.push(f);
        byte.push(b);
    }
    fresh.sort_unstable();
    byte.sort_unstable();
    (fresh, byte)
}

fn rank(sorted: &[u64], permille: usize) -> u64 {
    sorted[(permille * sorted.len()).div_ceil(1000).max(1) - 1]
}

#[test]
fn e_after_a_1_s_burst_at_p2_the_first_fresh_snapshot_arrives_within_the_swept_maximum() {
    let (fresh, byte) = sweep(Variant::Faithful, 1_000);
    let max = *fresh.last().expect("1,000 phases");
    let limit = (max * 5 / 4).div_ceil(100 * MS) * 100 * MS;
    let over = |x: u64| fresh.iter().filter(|&&f| f > x).count();
    println!(
        "S13 (e), 1,000 phases at P2 with a 1 s burst: first fresh snapshot after the return p50 {} ns, p99 {} ns, \
         max {} ns ({} over 1 s, {} over 1.5 s, {} over 2 s); first byte p50 {} ns, max {} ns; the docs PR's limit \
         (max x 1.25, rounded up to 0.1 s): {} ms",
        rank(&fresh, 500),
        rank(&fresh, 990),
        max,
        over(SEC),
        over(1_500 * MS),
        over(2 * SEC),
        rank(&byte, 500),
        byte.last().copied().unwrap_or(0),
        limit / MS
    );
    assert!(
        rank(&fresh, 500) > 500 * MS,
        "the stale backlog drains first: a fresh snapshot is never prompt"
    );
    // Pinned: the docs PR's number comes from these, so any change to the model fails here until it is re-derived.
    assert_eq!(
        (rank(&fresh, 500), rank(&fresh, 990), max),
        PINNED_E,
        "S13 (e) moved: re-derive the docs PR's limit"
    );
}

/// S13 (e)'s p50, p99 and maximum, in ns, at the commit that measured them (the docs PR cites it). Model v3: the
/// window shrinks at each repaired loss before the burst (cwnd 4 to 6 at its start), the undo and F-RTO act after it,
/// and the 64 KB stale backlog drains at that window (v2 pinned 736,507,852 / 932,850,818 / 3,170,535,056).
const PINNED_E: (u64, u64, u64) = (1_125_455_214, 2_891_749_084, 5_497_575_887);

/// A diagnostic (run by hand with --ignored --nocapture): the worst phases and what their transmissions did.
#[test]
#[ignore]
fn e_diagnose_the_worst_phases() {
    let mut all: Vec<(u64, u64, u64)> = (0..1_000u64)
        .map(|i| {
            let burst = 5 * SEC + ((i * 618_033_989) % SEC) * 2;
            (
                burst_phase(1_000 + i, burst, Variant::Faithful).0,
                1_000 + i,
                burst,
            )
        })
        .collect();
    all.sort_unstable();
    let over = |x: u64| all.iter().filter(|a| a.0 > x).count();
    println!(
        "phases over 1 s: {}, over 1.5 s: {}, over 2 s: {}",
        over(SEC),
        over(1_500 * MS),
        over(2 * SEC)
    );
    for &(rec, seed, burst) in all.iter().rev().take(3) {
        let (_, _, m) = burst_phase_model(seed, burst, Variant::Faithful, false);
        println!(
            "seed {seed}, burst at {} ms, recovery {} ms after the return, {:?}",
            burst / MS,
            rec / MS,
            m.counters(0)
        );
        for &(e, s, a, t, lost) in m
            .sent_log
            .iter()
            .filter(|x| x.0 == 0 && x.3 + 200 * MS >= burst)
        {
            println!(
                "  end {e} seq {s} attempt {a} at {:+} ms from the return, lost {lost}",
                (t as i64 - (burst + SEC) as i64) / MS as i64
            );
        }
    }
}

#[test]
fn bite_no_doubling_moves_e() {
    let (faithful, _) = sweep(Variant::Faithful, 100);
    let (flat, _) = sweep(Variant::NoDoubling, 100);
    assert_ne!(
        faithful.last(),
        flat.last(),
        "the recovery depends on the backoff"
    );
}

fn stall_run(p: &Profile, seed: u64) -> (Counters, u64) {
    let mut m = TcpModel::new(p, seed, 0);
    let mut feeds = [Feed {
        end: 0,
        size: SNAP,
        next: TICK,
        stop: 20_000 * TICK,
    }];
    run(&mut m, &mut feeds, 20_000 * TICK + 20 * SEC);
    assert_eq!(
        m.delivered(1),
        m.ends[0].snd.write_seq,
        "every byte arrives in order"
    );
    (m.counters(0), m.counters(0).segments)
}

/// `segments` first transmissions at 2 %: the band is 4 sigma around 2 % of them (400 and 19.8 at 20,000).
fn in_band(stalls: u64, segments: u64) -> bool {
    let n = segments as f64;
    let (mean, sigma) = (0.02 * n, (n * 0.02 * 0.98).sqrt());
    (stalls as f64 - mean).abs() <= 4.0 * sigma
}

#[test]
fn f_the_stall_count_equals_the_fate_draws_and_sits_in_the_binomial_band_at_p2() {
    for seed in 1..=3 {
        let (c, segments) = stall_run(&Profile::p2(), seed);
        // Re-derived for v3: each repaired loss ends Recovery at cwnd = ssthresh (tcp_end_cwnd_reduction), so a write
        // the window holds back joins the next one in the unsent tail and leaves in full segments, as tcp_sendmsg
        // appends to the tail skb: a few hundred of the 20,000 snapshots share segments (19,799 to 19,866 here).
        assert!(
            (19_000..=20_000).contains(&segments),
            "about one segment per snapshot: {segments}"
        );
        assert_eq!(
            c.stalls, c.first_lost,
            "seed {seed}: every first-transmission loss stalls, nothing else does"
        );
        assert!(
            in_band(c.stalls, segments),
            "seed {seed}: {} stalls in {segments} segments",
            c.stalls
        );
        assert_eq!(
            c.repairs_rack + c.repairs_probe + c.repairs_timeout,
            c.lost_profile,
            "seed {seed}: every lost transmission is repaired once"
        );
    }
}

#[test]
fn bite_loss_off_fails_f() {
    let (c, segments) = stall_run(&quiet(Profile::p2()), 1);
    assert_eq!(c.stalls, 0);
    assert!(!in_band(c.stalls, segments));
}

#[test]
fn the_same_seed_gives_the_same_connection_and_another_seed_does_not() {
    let p = Profile::p3();
    let go = |seed: u64| {
        let mut m = TcpModel::new(&p, seed, 0);
        let mut feeds = [
            Feed {
                end: 0,
                size: SNAP,
                next: TICK,
                stop: 20 * SEC,
            },
            Feed {
                end: 1,
                size: INPUT,
                next: TICK + 5 * MS,
                stop: 20 * SEC,
            },
        ];
        run(&mut m, &mut feeds, 30 * SEC);
        (m.counters(0), m.counters(1), m.sent_log.clone())
    };
    assert_eq!(go(4), go(4));
    assert_ne!(go(4).2, go(5).2);
}

#[test]
fn the_write_budget_holds_64_kib_of_unsent_and_unacknowledged_bytes() {
    let p = Profile::b1(&[2 * SEC]);
    let mut m = TcpModel::new(&p, 1, 0);
    // Inside the burst nothing is acknowledged, so writes stop at the budget.
    let mut taken = 0;
    for k in 0..200u64 {
        let t = 2 * SEC + k * 5 * MS;
        taken += m.write(0, SNAP, t);
        assert!(m.free_budget(0) <= WRITE_BUDGET);
    }
    assert_eq!(
        taken, WRITE_BUDGET,
        "it takes exactly the budget, then nothing"
    );
    assert_eq!(m.write(0, SNAP, 3 * SEC - 1), 0);
}

/// A diagnostic (by hand): fixes.rs's "fix 1 alone" phase (the adaptive newest-only budget, no reconnect) mirrored
/// step for step for DIAG_SEED (fixes.rs's phase_at: seed 1,000 (k + 1) + i), traced from 1.3 s before the return.
#[test]
#[ignore]
fn diag_fix1_phase() {
    let seed: u64 = std::env::var("DIAG_SEED")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(35_518);
    let (k, i) = (seed / 1_000 - 1, seed % 1_000);
    let burst = 5 * SEC + (((i + 1_000 * k) * 618_033_989) % SEC) * 2;
    let p = Profile::b1(&[burst]);
    let back = burst + SEC;
    let mut m = TcpModel::with_config(&p, seed, 0, TcpConfig::default());
    let (mut tick, mut input) = (TICK, TICK + (seed * 7_919) % TICK);
    let mut fresh_end = None;
    loop {
        let mut t = tick.min(input);
        if let Some(e) = m.next_event() {
            t = t.min(e);
        }
        if t > back + 12 * SEC {
            eprintln!("no fresh message by back + 12 s; {:?}", m.counters(0));
            break;
        }
        if m.trace.is_none() && t + 1_300 * MS >= back {
            m.trace = Some(back);
        }
        m.advance(t);
        if t == tick {
            let srtt = m.srtt_ns(0).unwrap_or(100 * MS);
            let limit = ((3 * srtt).div_ceil(2 * TICK) + 1).max(3) * SNAP;
            if m.free_budget(0) >= SNAP && m.outstanding(0) + SNAP <= limit {
                m.write(0, SNAP, t);
                if t >= back && fresh_end.is_none() {
                    fresh_end = Some(m.ends[0].snd.write_seq);
                }
            }
            tick += TICK;
        }
        if t == input {
            m.write(1, 88, t);
            input += TICK;
        }
        if fresh_end.is_some_and(|e| m.delivered(1) >= e) {
            eprintln!("fresh at {} ms after the return", (t - back) / MS);
            break;
        }
    }
}

/// A diagnostic (by hand): DIAG_IDX picks the phase at that rank (per mille) of the sweep; its trace goes to stderr.
#[test]
#[ignore]
fn diag_trace_phase() {
    let permille: usize = std::env::var("DIAG_IDX")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(500);
    let n: u64 = std::env::var("DIAG_N")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1_000);
    let mut all: Vec<(u64, u64, u64)> = (0..n)
        .map(|i| {
            let burst = 5 * SEC + ((i * 618_033_989) % SEC) * 2;
            (
                burst_phase(1_000 + i, burst, Variant::Faithful).0,
                1_000 + i,
                burst,
            )
        })
        .collect();
    all.sort_unstable();
    let (rec, seed, burst) = all[(permille * all.len() / 1000).min(all.len() - 1)];
    eprintln!(
        "seed {seed} burst {} ms recovery {} ms",
        burst / MS,
        rec / MS
    );
    let (_, _, m) = burst_phase_model(seed, burst, Variant::Faithful, true);
    eprintln!("{:?}", m.counters(0));
}

// ---- One test per kernel rule, each with its bite where one is cheap ----

fn sender(v: Variant) -> Sender {
    let mut s = Sender::new(&TcpConfig::default());
    s.variant = v;
    s
}

fn seg(start: u64, end: u64, xmit_ns: u64) -> Seg {
    Seg {
        start,
        end,
        xmit_ns,
        first_arrival_ns: 0,
        sacked: false,
        lost: false,
        sretrans: false,
        ever: false,
        tsorted: true,
        attempts: 1,
        cause: None,
        received: false,
    }
}

/// A sender in the Open state with `n` outstanding segments of 1,000 B, the i-th sent at `t0` + i ms.
fn loaded(v: Variant, n: u64, t0: u64) -> Sender {
    let mut s = sender(v);
    for i in 0..n {
        s.segs
            .push_back(seg(i * 1_000, (i + 1) * 1_000, t0 + i * MS));
    }
    s.snd_nxt = n * 1_000;
    s.write_seq = n * 1_000;
    s
}

fn fresh_sack_state(s: &Sender) -> SackState {
    SackState {
        reord: s.snd_nxt,
        first_sackt: None,
        last_sackt: None,
        sack_delivered: 0,
    }
}

/// tcp_retransmit_skb's marks for the head resent at `now` (tcp_output.c:3407-3430), as an RTO leaves them.
fn head_resent(s: &mut Sender, now: u64) {
    let h = &mut s.segs[0];
    h.sretrans = true;
    h.ever = true;
    h.xmit_ns = now;
    s.retrans_stamp = Some(ts_of(now));
    s.undo_retrans = 1;
    s.total_retrans = 1;
}

/// A cumulative ACK over a resent segment: the sample it gives (has one, backoff), and its flag.
fn over_resent(v: Variant) -> (bool, u32, bool, u64) {
    let mut s = loaded(v, 2, 400 * MS);
    s.segs[0].ever = true;
    s.segs[0].xmit_ns = 500 * MS;
    s.backoff = 2;
    s.snd_una = 2_000;
    let st = fresh_sack_state(&s);
    let (flag, delivered) = s.clean_rtx_queue(0, &st, SEC);
    (
        s.rtt.has(),
        s.backoff,
        flag & FLAG_RETRANS_DATA_ACKED != 0,
        delivered,
    )
}

/// Karn's rule as tcp_clean_rtx_queue applies it (tcp_input.c:3319-3327, :3390): an ACK that delivers in order any
/// segment ever resent (FLAG_RETRANS_DATA_ACKED) gives no sequence sample; otherwise the oldest segment it delivers
/// gives the sample (first_ackt) and the newest the minimum RTT (last_ackt).
#[test]
fn karn_no_sample_beside_a_resent_segment_and_the_oldest_fresh_one_gives_it() {
    assert_eq!(over_resent(Variant::Faithful), (false, 2, true, 2));
    let mut s = loaded(Variant::Faithful, 2, 200 * MS);
    s.segs[1].xmit_ns = 600 * MS;
    s.snd_una = 2_000;
    let st = fresh_sack_state(&s);
    s.clean_rtx_queue(0, &st, SEC);
    assert_eq!((s.rtt.srtt_us(), s.rtt.min_rtt), (800_000, Some(400_000)));
}

#[test]
fn bite_karn_off_samples_beside_a_resent_segment() {
    let (has, backoff, _, _) = over_resent(Variant::NoKarn);
    assert!(has && backoff == 0);
}

/// The echo's sample when nothing else gives one: (srtt in us, backoff).
fn echo_sample(v: Variant, delivers: bool) -> (u64, u32) {
    let mut s = loaded(v, 1, 0);
    s.segs[0].ever = true;
    s.segs[0].xmit_ns = 500 * MS;
    s.backoff = 3;
    if delivers {
        s.snd_una = 1_000;
    }
    s.rcv_tsecr = Some(700);
    let st = fresh_sack_state(&s);
    s.clean_rtx_queue(0, &st, SEC);
    (s.rtt.srtt_us(), s.backoff)
}

/// tcp_ack_update_rtt (tcp_input.c:3146-3157): with no sequence or SACK sample, the echoed timestamp gives one (whole
/// milliseconds, tcp_rtt_tsopt_us), but only when the ACK delivers data in order (FLAG_ACKED); it resets the backoff.
#[test]
fn the_echo_gives_the_sample_only_when_data_is_delivered_in_order() {
    assert_eq!(echo_sample(Variant::Faithful, true), (300_000, 0));
    assert_eq!(echo_sample(Variant::Faithful, false), (0, 3));
}

#[test]
fn bite_no_echo_sample() {
    assert_eq!(echo_sample(Variant::NoTsecrFallback, true), (0, 3));
}

/// The timestamp a receiver echoes after two segments sent at 0 and 10 ms (it sent nothing before: rcv_wup = 0).
fn echoed(v: Variant) -> Option<u64> {
    let p = quiet(Profile::p2());
    let mut m = TcpModel::new(&p, 1, 0).with_variant(v, &p);
    m.write(0, MSS, 0);
    m.write(0, MSS, 10 * MS);
    // The second arrival (by 70 ms) makes end 1 acknowledge at once; that ACK reaches end 0 at 90 ms at the soonest.
    m.advance(85 * MS);
    m.paths[1].queue.back().and_then(|pk| pk.ack.tsecr)
}

/// tcp_replace_ts_recent (tcp_input.c:3746-3758, called at :3904): only a segment starting at or below rcv_wup sets
/// the timestamp to echo, so a delayed ACK echoes the first of the segments it covers (RFC 7323 4.3).
#[test]
fn the_echo_comes_from_the_last_arrival_at_or_below_rcv_wup() {
    assert_eq!(echoed(Variant::Faithful), Some(0));
}

#[test]
fn bite_ts_recent_from_every_arrival() {
    assert_eq!(echoed(Variant::TsRecentAlways), Some(10));
}

/// An RTO at 1 s over three segments, the last two sent within RACK's RTT (200 ms): (lost marks, cwnd, ssthresh).
fn rto_marks(v: Variant) -> (Vec<bool>, u64, u64, State) {
    let mut s = loaded(v, 3, 0);
    s.segs[0].xmit_ns = 980 * MS;
    s.segs[1].xmit_ns = 900 * MS;
    s.segs[2].xmit_ns = 950 * MS;
    s.rack.set = true;
    s.rack.rtt_us = 200_000;
    s.enter_loss(SEC);
    (
        s.segs.iter().map(|g| g.lost).collect(),
        s.cwnd,
        s.ssthresh,
        s.state,
    )
}

/// tcp_timeout_mark_lost (tcp_input.c:2143-2171): every segment lost but those sent within RACK's RTT, the head always;
/// tcp_enter_loss (:2174-2218): ssthresh = Cubic's max(0.7 cwnd, 2), cwnd = what is still in flight + 1.
#[test]
fn the_rto_spares_recent_segments_but_not_the_head_and_sets_cwnd_to_in_flight_plus_one() {
    assert_eq!(
        rto_marks(Variant::Faithful),
        (vec![true, false, false], 3, 7, State::Loss)
    );
}

#[test]
fn bite_the_rto_marks_everything() {
    assert_eq!(rto_marks(Variant::RtoMarksAll).0, vec![true, true, true]);
}

#[test]
fn bite_the_rto_sets_cwnd_to_one() {
    assert_eq!(rto_marks(Variant::RtoCwndOne).1, 1);
}

/// The client's data D (sent at 5 ms) is acknowledged only inside the server's resend of X, an old duplicate at the
/// client (X and Y arrived and the client sent since, so its rcv_wup is past X). Returns, at 2.08 s, the server's
/// discards, the client's snd_una, and at 2.3 s the D-SACKs the server accepted.
fn old_duplicate(v: Variant) -> (u64, u64, u64) {
    let p = quiet(Profile::p2());
    let mut m = TcpModel::new(&p, 1, 0).with_variant(v, &p);
    // The client's transmissions die from 10 ms to 2 s, the server's from 80 ms to 1.9 s: X and Y's ACKs, the server's
    // ACK of D, the server's probe (Y at 1 s) and the client's probe of D all die; the server's RTO resends X at 2 s.
    m.blackouts.push((1, 10 * MS, 2 * SEC));
    m.blackouts.push((0, 80 * MS, 1_900 * MS));
    m.write(0, 2 * MSS, 0);
    m.write(1, MSS, 5 * MS);
    m.advance(2_080 * MS);
    let at_2080 = (m.counters(0).discards_old, m.ends[1].snd.snd_una);
    m.advance(2_300 * MS);
    (at_2080.0, at_2080.1, m.counters(0).dsacks)
}

/// tcp_validate_incoming (tcp_input.c:5857-5900) and tcp_sequence (:4424-4434): a segment ending before rcv_wup is
/// discarded whole, its ACK field never read; tcp_send_dupack (:4618-4637) answers with a D-SACK for it.
#[test]
fn an_old_duplicate_is_discarded_unread_and_answered_with_a_dsack() {
    assert_eq!(old_duplicate(Variant::Faithful), (1, 0, 1));
}

#[test]
fn bite_an_old_duplicate_processed_acknowledges_d() {
    let (discards, snd_una, _) = old_duplicate(Variant::OldSeqProcessed);
    assert_eq!((discards, snd_una), (0, MSS));
}

fn receiver(no_dsack: bool) -> Receiver {
    Receiver {
        max_blocks: 3,
        no_dsack,
        ..Receiver::default()
    }
}

/// The receiver's D-SACK (tcp_data_queue :5146-5265, tcp_data_queue_ofo :4924-5069, tcp_dsack_set :4565-4583,
/// tcp_options_write tcp_output.c:709-726): one block, first in the option, sent once, for a duplicate below rcv_nxt
/// or one already held out of order; and the SACK blocks as Linux keeps them (tcp_sack_new_ofo_skb :4696-4743,
/// tcp_sack_remove :4745-4772): most recently changed first, three beside the timestamp option.
#[test]
fn the_receiver_sends_one_dsack_first_and_its_blocks_most_recent_first() {
    let mut r = receiver(false);
    assert!(matches!(r.data_queue(0, 1_000), Arrival::InOrder));
    assert!(matches!(r.data_queue(2_000, 3_000), Arrival::OutOfOrder));
    assert!(matches!(r.data_queue(0, 1_000), Arrival::Duplicate));
    assert_eq!(r.ack().sacks, vec![(0, 1_000), (2_000, 3_000)]);
    assert_eq!(r.ack().sacks, vec![(2_000, 3_000)], "the D-SACK goes once");
    assert!(matches!(r.data_queue(2_000, 3_000), Arrival::OutOfOrder));
    assert_eq!(
        r.ack().sacks,
        vec![(2_000, 3_000), (2_000, 3_000)],
        "a duplicate held out of order: inside the next block"
    );
    for k in [4, 6, 8] {
        r.data_queue(k * 1_000, k * 1_000 + 1_000);
    }
    assert_eq!(
        r.ack().sacks,
        vec![(8_000, 9_000), (6_000, 7_000), (4_000, 5_000)]
    );
    r.data_queue(1_000, 2_000);
    assert_eq!(r.rcv_nxt, 3_000, "the hole filled, the held range released");
    assert_eq!(r.sacks.len(), 3, "blocks at or below rcv_nxt dropped");
}

#[test]
fn bite_no_dsack() {
    let mut r = receiver(true);
    r.data_queue(0, 1_000);
    r.data_queue(0, 1_000);
    assert!(r.ack().sacks.is_empty());
}

/// tcp_check_dsack (tcp_input.c:1282-1323), tcp_dsack_seen (:1022-1057) and tcp_sacktag_one's duplicate path
/// (:1386-1394): a D-SACK below the cumulative point for a resend counts it off undo_retrans and is delivered; a
/// D-SACK for more than was ever resent is dubious and ignored.
#[test]
fn a_dsack_for_a_resend_counts_it_off_undo_retrans_and_a_dubious_one_is_ignored() {
    let tag = |total_retrans: u64| {
        let mut s = loaded(Variant::Faithful, 2, 0);
        s.state = State::Loss;
        s.undo_marker = Some(0);
        s.undo_retrans = 1;
        s.total_retrans = total_retrans;
        let h = &mut s.segs[0];
        h.lost = true;
        h.sretrans = true;
        h.ever = true;
        s.snd_una = 1_000;
        let ack = Ack {
            cum: 1_000,
            sacks: vec![(0, 1_000)],
            tsecr: None,
        };
        let mut st = fresh_sack_state(&s);
        let flag = s.sacktag(&ack, 0, &mut st, SEC);
        (
            flag & FLAG_DSACKING_ACK != 0,
            s.undo_retrans,
            s.dsacks,
            st.sack_delivered,
        )
    };
    assert_eq!(tag(1), (true, 0, 1, 1));
    assert_eq!(tag(0), (false, 1, 0, 0));
}

/// An RTO at 1 s over three segments, the head resent then; then the head's original is acknowledged with the echo
/// `tsecr` (ms). Returns (state, cwnd, ssthresh, lost, Loss undos).
fn eifel(v: Variant, tsecr: u64) -> (State, u64, u64, u64, u64) {
    let mut s = loaded(v, 3, 0);
    s.enter_loss(SEC);
    head_resent(&mut s, SEC);
    let now = SEC + 100 * MS;
    s.snd_una = 1_000;
    s.rcv_tsecr = Some(tsecr);
    let st = fresh_sack_state(&s);
    let (mut flag, _) = s.clean_rtx_queue(0, &st, now);
    flag |= FLAG_SND_UNA_ADVANCED;
    let mut rexmit = Rexmit::None;
    s.fastretrans_alert(0, 0, &mut flag, &mut rexmit, now);
    (s.state, s.cwnd, s.ssthresh, s.lost_out(), s.undos_loss)
}

/// The undo by timestamps (Eifel, RFC 3522): tcp_process_loss (tcp_input.c:2873-2881) calls tcp_try_undo_loss
/// (:2616-2639) on an ACK that advances; tcp_may_undo (:2550-2553) holds through tcp_packet_delayed (:2457-2461), an
/// echo older than the first resend (retrans_stamp); tcp_undo_cwnd_reduction (:2521-2548) restores cwnd and ssthresh
/// and forgets the loss marks, and the state goes back to Open.
#[test]
fn an_echo_older_than_the_first_resend_undoes_the_timeout() {
    assert_eq!(
        eifel(Variant::Faithful, 50),
        (State::Open, 10, INFINITE_SSTHRESH, 0, 1)
    );
    assert_eq!(
        eifel(Variant::Faithful, 1_050),
        (State::Loss, 1, 7, 2, 0),
        "an echo of the resend proves nothing"
    );
}

#[test]
fn bite_no_undo_keeps_a_spurious_timeout() {
    assert_eq!(eifel(Variant::NoUndo, 50), (State::Loss, 1, 7, 2, 0));
}

/// F-RTO's step 3.b: after an RTO at 1 s over two segments (head resent), a SACK of the second, sent before the timeout
/// and never resent. Returns (state, cwnd, F-RTO undos).
fn frto_3b(v: Variant) -> (State, u64, u64) {
    let mut s = loaded(v, 2, 0);
    s.enter_loss(SEC);
    head_resent(&mut s, SEC);
    let now = SEC + 100 * MS;
    let ack = Ack {
        cum: 0,
        sacks: vec![(1_000, 2_000)],
        tsecr: Some(1_050),
    };
    s.rcv_tsecr = ack.tsecr;
    let mut st = fresh_sack_state(&s);
    let mut flag = s.sacktag(&ack, 0, &mut st, now);
    flag |= s.clean_rtx_queue(0, &st, now).0;
    let mut rexmit = Rexmit::None;
    s.fastretrans_alert(0, 0, &mut flag, &mut rexmit, now);
    (s.state, s.cwnd, s.undos_frto)
}

/// F-RTO (tcp_frto = 2): armed by tcp_enter_loss (tcp_input.c:2211-2217); tcp_process_loss's step 3.b (:2883-2890):
/// data never resent (FLAG_ORIG_SACK_ACKED, from tcp_sacktag_one :1418-1428) proves the timeout spurious, so the loss
/// is undone (tcp_try_undo_loss with frto_undo); Disorder follows, the head's resend still out.
#[test]
fn frto_a_never_resent_segment_sacked_after_the_timeout_undoes_it() {
    assert_eq!(frto_3b(Variant::Faithful), (State::Disorder, 10, 1));
}

#[test]
fn bite_no_frto_keeps_the_timeout() {
    assert_eq!(frto_3b(Variant::NoFrto), (State::Loss, 1, 0));
}

/// F-RTO's steps 2.b and 3.a (tcp_process_loss :2892-2905, tcp_xmit_recovery :3806-3820): the first ACK that advances
/// sends new data first when there is some; an ACK that then SACKs it shows the loss was real.
#[test]
fn frto_sends_new_data_first_then_a_sack_of_it_ends_frto() {
    let mut s = loaded(Variant::Faithful, 3, 0);
    s.write_seq = 5_000;
    s.enter_loss(SEC);
    head_resent(&mut s, SEC);
    let now = SEC + 100 * MS;
    s.snd_una = 1_000;
    s.rcv_tsecr = Some(1_060);
    let st = fresh_sack_state(&s);
    let (mut flag, _) = s.clean_rtx_queue(0, &st, now);
    flag |= FLAG_SND_UNA_ADVANCED;
    let mut rexmit = Rexmit::None;
    s.fastretrans_alert(0, 0, &mut flag, &mut rexmit, now);
    assert_eq!((rexmit, s.frto, s.high_seq), (Rexmit::New, true, 3_000));
    // The new segment went out past high_seq; the next ACK SACKs it (step 3.a).
    s.segs.push_back(seg(3_000, 4_000, now));
    s.snd_nxt = 4_000;
    let later = now + 100 * MS;
    let ack = Ack {
        cum: 1_000,
        sacks: vec![(3_000, 4_000)],
        tsecr: Some(1_100),
    };
    s.rcv_tsecr = ack.tsecr;
    let mut st = fresh_sack_state(&s);
    let mut flag = s.sacktag(&ack, 1_000, &mut st, later);
    flag |= s.clean_rtx_queue(1_000, &st, later).0;
    let mut rexmit = Rexmit::None;
    s.fastretrans_alert(1_000, 0, &mut flag, &mut rexmit, later);
    assert_eq!(
        (s.frto, s.state, rexmit),
        (false, State::Loss, Rexmit::Lost)
    );
}

/// PRR (RFC 6937) as tcp_cwnd_reduction computes it (tcp_input.c:2663-2687) after tcp_init_cwnd_reduction
/// (:2649-2661): above ssthresh, ceil(ssthresh x delivered / prior_cwnd) less what was sent; at or below it, packet
/// conservation, plus one on an advancing ACK with no new loss, at most up to ssthresh.
#[test]
fn prr_spreads_the_reduction_then_conserves_packets() {
    let mut s = loaded(Variant::Faithful, 10, 0);
    s.init_cwnd_reduction();
    s.state = State::Recovery;
    assert_eq!((s.prior_cwnd, s.ssthresh), (10, 7));
    s.segs[0].lost = true;
    s.segs[1].sacked = true;
    s.cwnd_reduction(1, 1, 0);
    assert_eq!(s.cwnd, 9, "8 in flight + ceil(7 x 1 / 10)");
    s.segs[0].sretrans = true;
    s.prr_out = 1;
    s.segs[2].sacked = true;
    s.segs[3].sacked = true;
    s.cwnd_reduction(2, 0, 0);
    assert_eq!(s.cwnd, 7, "7 in flight, at ssthresh: nothing more");
    for _ in 0..4 {
        s.segs.pop_front();
    }
    s.cwnd_reduction(1, 0, FLAG_SND_UNA_ADVANCED);
    assert_eq!(s.cwnd, 7, "6 in flight: one more, up to ssthresh");
}

/// A single loss on a settled quiet stream: the window afterwards (cwnd, ssthresh, state, RACK repairs).
fn after_one_loss(v: Variant) -> (u64, u64, State, u64) {
    let p = quiet(Profile::p2());
    let mut m = TcpModel::new(&p, 1, 0).with_variant(v, &p);
    m.forced.push((0, 30 * SNAP, 1));
    let mut feeds = [Feed {
        end: 0,
        size: SNAP,
        next: TICK,
        stop: 3 * SEC,
    }];
    run(&mut m, &mut feeds, 6 * SEC);
    let s = &m.ends[0].snd;
    (s.cwnd, s.ssthresh, s.state, m.counters(0).repairs_rack)
}

/// tcp_fastretrans_alert's Recovery exit (tcp_input.c:3036-3043) and tcp_end_cwnd_reduction (:2689-2703): a recovery
/// not undone ends with cwnd = ssthresh = max(0.7 x 10, 2); an app-limited flow never grows it back (S2).
#[test]
fn recovery_ends_with_cwnd_at_ssthresh() {
    assert_eq!(after_one_loss(Variant::Faithful), (7, 7, State::Open, 1));
}

/// Recovery entered at cwnd 10 (ssthresh 7), PRR having left cwnd at 4, then every segment acknowledged: the exit's
/// (state, cwnd), with the recovery undone (every resend D-SACKed: undo_retrans 0) or not.
fn recovery_exit(v: Variant, undone: bool) -> (State, u64) {
    let mut s = loaded(v, 2, 0);
    s.enter_recovery(SEC);
    s.cwnd = 4;
    if undone {
        s.undo_retrans = 0;
    }
    s.snd_una = 2_000;
    s.segs.clear();
    let mut flag = FLAG_SND_UNA_ADVANCED | FLAG_DATA_ACKED;
    let mut rexmit = Rexmit::None;
    s.fastretrans_alert(0, 0, &mut flag, &mut rexmit, SEC + 100 * MS);
    (s.state, s.cwnd)
}

#[test]
fn recovery_exit_sets_ssthresh_unless_undone() {
    assert_eq!(recovery_exit(Variant::Faithful, false), (State::Open, 7));
    assert_eq!(recovery_exit(Variant::Faithful, true), (State::Open, 10));
}

#[test]
fn bite_recovery_exit_keeps_cwnd() {
    assert_eq!(recovery_exit(Variant::NoExitCwnd, false), (State::Open, 4));
}

/// tcp_process_tlp_ack (tcp_input.c:3764-3792): a probe that resent data, then an ACK past tlp_high_seq, shows a loss
/// the probe repaired: CWR, at once cwnd = ssthresh; a probe of new data, or a D-SACK of the probe, reduces nothing.
#[test]
fn a_resent_probe_that_repaired_a_loss_cuts_cwnd_once_passed() {
    let tlp = |retrans: bool, ack: u64, flag: u32| {
        let mut s = loaded(Variant::Faithful, 4, 0);
        s.tlp_high_seq = Some(3_000);
        s.tlp_retrans = retrans;
        s.process_tlp_ack(ack, flag);
        (s.cwnd, s.ssthresh, s.tlp_high_seq, s.state)
    };
    assert_eq!(tlp(true, 4_000, 0), (7, 7, None, State::Open));
    assert_eq!(
        tlp(false, 4_000, 0),
        (10, INFINITE_SSTHRESH, None, State::Open)
    );
    assert_eq!(
        tlp(true, 3_000, FLAG_DSACK_TLP),
        (10, INFINITE_SSTHRESH, None, State::Open)
    );
    assert_eq!(
        tlp(true, 3_000, FLAG_SND_UNA_ADVANCED),
        (10, INFINITE_SSTHRESH, Some(3_000), State::Open)
    );
}

/// Twenty ACKs of one segment each, 1 ms apart, at cwnd = ssthresh = 10.
fn cubic_steps(v: Variant, limited: bool) -> u64 {
    let mut s = sender(v);
    s.cwnd = 10;
    s.ssthresh = 10;
    s.is_cwnd_limited = limited;
    for k in 0..20 {
        s.cong_avoid(1, SEC + k * MS);
    }
    s.cwnd
}

/// tcp_slow_start (tcp_cong.c:458-466) and cubictcp_cong_avoid (tcp_cubic.c:324-339): slow start to ssthresh with the
/// rest carried into congestion avoidance; Cubic's first epoch at most one segment per 20 delivered (5 % per RTT,
/// bictcp_update :296-297); nothing while the flow is not cwnd-limited (tcp_is_cwnd_limited).
#[test]
fn cubic_grows_from_ssthresh_only_while_cwnd_limited() {
    let mut s = sender(Variant::Faithful);
    s.cwnd = 2;
    s.ssthresh = 5;
    s.max_packets_out = 2;
    s.cong_avoid(4, SEC);
    assert_eq!(
        (s.cwnd, s.cwnd_cnt),
        (5, 1),
        "slow start to ssthresh, one ACK left over"
    );
    assert_eq!(cubic_steps(Variant::Faithful, true), 11);
    assert_eq!(
        cubic_steps(Variant::Faithful, false),
        10,
        "app-limited: no growth"
    );
}

#[test]
fn bite_no_cubic_growth_above_ssthresh() {
    assert_eq!(cubic_steps(Variant::NoCubic, true), 10);
}

/// Cubic's curve in the kernel's fixed point (bictcp_update :214-322, cubic_root :167-209): after a reduction from
/// Wmax = 20 to 14 (cubictcp_recalc_ssthresh :341-356), K = cbrt((20 - 14) x 0.1 s / 0.4) = 2.47 s (2,525 in
/// 1/1,024 s); cwnd stays at or below Wmax until about K and passes it after.
#[test]
fn cubic_climbs_back_to_wmax_at_k_and_passes_it_after() {
    for a in [64u64, 1_000, 123_456, 16_090_414_062, 1 << 40] {
        let exact = (a as f64).cbrt();
        let got = cubic_root(a) as f64;
        // An integer root: at most one unit of truncation beside the table's ~0.2 % error.
        assert!(
            (got - exact).abs() < 1.0 + 0.005 * exact,
            "cbrt({a}) = {got}, not {exact}"
        );
    }
    let mut s = sender(Variant::Faithful);
    s.cwnd = 20;
    s.ssthresh = s.cubic_ssthresh();
    assert_eq!((s.ssthresh, s.cubic.last_max_cwnd), (14, 20));
    s.cwnd = 14;
    s.is_cwnd_limited = true;
    let k = cubic_root(CUBE_FACTOR * 6);
    assert!((2_510..=2_540).contains(&k), "K = {k}");
    let mut at = Vec::new();
    for step in 0..=400u64 {
        // An ACK every 10 ms delivering one segment.
        s.cong_avoid(1, SEC + step * 10 * MS);
        at.push(s.cwnd);
    }
    let before_k = at[200];
    let after_k = at[400];
    assert!((17..=20).contains(&before_k), "at 2 s: {before_k}");
    assert!(after_k > 20, "at 4 s: {after_k}");
}

/// HyStart (cubictcp_acked :448-474, hystart_update :386-446) at cwnd 16 and above in slow start: a round whose
/// smallest of 8 RTT samples exceeds delay_min + clamp(delay_min / 8, 4, 16 ms) ends slow start (ssthresh = cwnd).
#[test]
fn hystart_ends_slow_start_on_a_delay_increase() {
    let rise = |later_us: i64| {
        let mut s = sender(Variant::Faithful);
        s.cwnd = 20;
        s.snd_nxt = 10_000;
        s.cubic_acked(Some(100_000), 10 * MS);
        s.snd_una = 5_000;
        for k in 0..9 {
            s.cubic_acked(Some(later_us), (20 + 10 * k) * MS);
        }
        s.ssthresh
    };
    assert_eq!(rise(120_000), 20);
    assert_eq!(
        rise(110_000),
        INFINITE_SSTHRESH,
        "within 12.5 ms of the minimum"
    );
}

/// Whether the probe may be armed in `state`.
fn probe_in(v: Variant, state: State) -> bool {
    let mut s = loaded(v, 2, 0);
    s.rtt.sample(100_000, 100_000, 0, 0, 200_000);
    s.state = state;
    s.schedule_probe(SEC, true)
}

/// tcp_schedule_loss_probe (tcp_output.c:2845-2850): a probe only in Open or CWR; tcp_try_keep_open (tcp_input.c:
/// 2719-2731): Disorder while anything is SACKed, lost or resent, Open (with high_seq moved) once not.
#[test]
fn disorder_arms_no_probe() {
    assert!(probe_in(Variant::Faithful, State::Open));
    assert!(probe_in(Variant::Faithful, State::Cwr));
    assert!(!probe_in(Variant::Faithful, State::Disorder));
    assert!(!probe_in(Variant::Faithful, State::Recovery));
    let mut s = loaded(Variant::Faithful, 2, 0);
    s.segs[1].sacked = true;
    s.try_keep_open();
    assert_eq!(s.state, State::Disorder);
    s.segs[1].sacked = false;
    s.high_seq = 0;
    s.try_keep_open();
    assert_eq!((s.state, s.high_seq), (State::Open, 2_000));
}

#[test]
fn bite_a_probe_in_disorder() {
    assert!(probe_in(Variant::ProbeInDisorder, State::Disorder));
}

/// A connect whose SYN-ACKs all die for 2.5 s: (client cwnd, server cwnd, SYNs sent).
fn handshake_windows(v: Variant, timestamps: bool) -> (u64, u64, u32) {
    let p = quiet(Profile::p2());
    let cfg = TcpConfig {
        timestamps,
        ..TcpConfig::default()
    };
    let mut m = TcpModel::connect(&p, 1, 0, cfg, 0).with_variant(v, &p);
    m.blackouts.push((0, 0, 2_500 * MS));
    m.advance(4 * SEC);
    assert!(m.established(0) && m.established(1));
    let syns = m.setup.map_or(0, |s| s.syn_tries);
    (m.ends[1].snd.cwnd, m.ends[0].snd.cwnd, syns)
}

/// tcp_init_transfer (tcp_input.c:6187-6213): cwnd 1 after more than one handshake resend when an undo marker is left;
/// tcp_try_undo_spurious_syn (:6319-6332): the client clears it when the SYN-ACK echoes its first SYN's timestamp
/// (retrans_stamp, tcp_output.c:4093), so its SYN timeouts were spurious; the server keeps it when the ACK echoes a
/// later SYN-ACK than its first (tcp_minisocks.c:594-606).
#[test]
fn the_handshake_cuts_cwnd_after_resends_unless_the_echo_shows_the_first_syn_arrived() {
    assert_eq!(handshake_windows(Variant::Faithful, true), (10, 1, 4));
    assert_eq!(handshake_windows(Variant::Faithful, false), (1, 1, 4));
}

#[test]
fn bite_no_spurious_syn_undo() {
    assert_eq!(handshake_windows(Variant::NoSpuriousSyn, true).0, 1);
}

/// The client's SYN times (s) while no SYN-ACK arrives, up to 40 s.
fn syn_times(syn_rto_us: u64, syn_linear_timeouts: u32) -> Vec<u64> {
    let p = quiet(Profile::p2());
    let cfg = TcpConfig {
        syn_rto_us,
        syn_linear_timeouts,
        ..TcpConfig::default()
    };
    let mut m = TcpModel::connect(&p, 1, 0, cfg, 0);
    m.blackouts.push((0, 0, 60 * SEC));
    let mut times = vec![0];
    let mut sent = m.counters(1).handshake_packets;
    while let Some(t) = m.next_event() {
        if t > 40 * SEC {
            break;
        }
        m.advance(t);
        let now = m.counters(1).handshake_packets;
        if now > sent {
            times.push(t / SEC);
            sent = now;
        }
    }
    times
}

/// The SYN timer (tcp_retransmit_timer's SYN_SENT branch, tcp_timer.c:640-660): Linux's RTOs are "1, 1, 1, 1, 1, 2,
/// 4, ..." with tcp_syn_linear_timeouts = 4 (ip-sysctl.rst:958-966); TcpConfig sets another client's (a Windows client:
/// 3 s, then doubling at once).
#[test]
fn the_syn_timer_is_linear_for_linux_and_settable_for_other_clients() {
    assert_eq!(
        syn_times(RTO_INIT_US, 4),
        vec![0, 1, 2, 3, 4, 5, 7, 11, 19, 35]
    );
    assert_eq!(syn_times(3 * RTO_INIT_US, 0), vec![0, 3, 9, 21]);
    assert_eq!(syn_times(RTO_INIT_US, 0), vec![0, 1, 3, 7, 15, 31]);
}

/// Two packets sent at the same instant arrive at distinct instants (S1): the kernel processes each arrival at its own
/// clock reading, so RACK never compares sends from separate arrivals at one stamp.
#[test]
fn no_two_packets_on_a_path_arrive_at_the_same_instant() {
    let p = quiet(Profile::p0());
    let mut m = TcpModel::new(&p, 1, 0);
    m.write(0, 3 * MSS, 0);
    let arrivals: Vec<u64> = m.paths[0].queue.iter().map(|pk| pk.arrival_ns).collect();
    assert_eq!(arrivals, vec![1, 2, 3]);
}
