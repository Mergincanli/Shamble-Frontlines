//! S13's tests on simulated time (advisor-u3q.md §(1)): (a) a single loss under 30 Hz traffic, (b) a tail loss on a
//! quiet stream, (c) a lost resend, (d) the doubling and its reset, (e) the 1 s burst swept over 1,000 phases (the
//! docs PR's number), (f) the stall count against the fate draws; with S13's bites (the old constant in place of RACK,
//! no doubling, loss off), plus determinism and the write budget.

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
    let mut m = TcpModel::new(p, seed, 0).with_variant(variant, p);
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

fn doubling(variant: Variant) -> (Vec<u64>, u32) {
    let p = quiet(Profile::p2());
    let mut m = settled(&p, 3, variant);
    let start = m.ends[0].snd.write_seq;
    m.forced.extend((1..=4).map(|a| (0, start, a)));
    m.write(0, SNAP, 5 * SEC);
    run(&mut m, &mut [], 12 * SEC);
    let sends = sends_of(&m, 0, start);
    let gaps: Vec<u64> = sends.windows(2).map(|w| w[1] - w[0]).collect();
    // A new message after the episode: its acknowledgement is a valid sample, which resets the backoff.
    m.write(0, SNAP, 12 * SEC);
    run(&mut m, &mut [], 14 * SEC);
    (gaps, m.ends[0].snd.backoff)
}

#[test]
fn d_the_rto_doubles_on_each_expiry_and_a_valid_sample_resets_it() {
    let (gaps, backoff) = doubling(Variant::Faithful);
    assert_eq!(gaps.len(), 4, "the probe, then three timeouts: {gaps:?}");
    assert_eq!(gaps[2], 2 * gaps[1], "{gaps:?}");
    assert_eq!(gaps[3], 2 * gaps[2], "{gaps:?}");
    assert_eq!(backoff, 0, "the next valid RTT sample resets the backoff");
}

#[test]
fn bite_no_doubling_fails_d() {
    let (gaps, _) = doubling(Variant::NoDoubling);
    assert_ne!(gaps[3], 2 * gaps[2], "{gaps:?}");
}

/// One phase of (e): a B1 burst at `burst_ns`, the room writing one snapshot a tick (the newest only, once the write
/// budget is full) and the client one input a tick. Returns, from the link's return, the time to the in-order delivery
/// of the first snapshot written at or after it, and to the first byte after it.
fn burst_phase(seed: u64, burst_ns: u64, variant: Variant) -> (u64, u64) {
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

/// S13 (e)'s p50, p99 and maximum, in ns, at the commit that measured them (the docs PR cites it).
const PINNED_E: (u64, u64, u64) = (708_081_049, 914_405_971, 2_516_541_273);

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
        let p = Profile::b1(&[burst]);
        let mut m = TcpModel::new(&p, seed, 0);
        let mut feeds = [
            Feed {
                end: 0,
                size: SNAP,
                next: TICK,
                stop: burst + 4 * SEC,
            },
            Feed {
                end: 1,
                size: INPUT,
                next: TICK + (seed * 7_919) % TICK,
                stop: burst + 4 * SEC,
            },
        ];
        run(&mut m, &mut feeds, burst + 5 * SEC);
        println!(
            "seed {seed}, burst at {} ms, recovery {} ms, {:?}",
            burst / MS,
            rec / MS,
            m.counters(0)
        );
        for &(e, s, a, t, lost) in m
            .sent_log
            .iter()
            .filter(|x| x.0 == 0 && x.2 > 1 && x.3 >= burst)
        {
            println!(
                "  end {e} seq {s} attempt {a} at +{} ms lost {lost}",
                (t - burst) / MS
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

/// 20,000 segments at 2 %: 400 expected, sigma about 19.8; the band is 4 sigma.
fn in_band(stalls: u64) -> bool {
    (321..=479).contains(&stalls)
}

#[test]
fn f_the_stall_count_equals_the_fate_draws_and_sits_in_the_binomial_band_at_p2() {
    for seed in 1..=3 {
        let (c, segments) = stall_run(&Profile::p2(), seed);
        assert_eq!(segments, 20_000, "one segment per snapshot");
        assert_eq!(
            c.stalls, c.first_lost,
            "seed {seed}: every first-transmission loss stalls, nothing else does"
        );
        assert!(in_band(c.stalls), "seed {seed}: {} stalls", c.stalls);
        assert_eq!(
            c.repairs_rack + c.repairs_probe + c.repairs_timeout,
            c.lost_profile,
            "seed {seed}: every lost transmission is repaired once"
        );
    }
}

#[test]
fn bite_loss_off_fails_f() {
    let (c, _) = stall_run(&quiet(Profile::p2()), 1);
    assert_eq!(c.stalls, 0);
    assert!(!in_band(c.stalls));
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
