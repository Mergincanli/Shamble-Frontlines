//! U4f's endpoint traces (u4f-brief.md 2.1, 2.2, 2.10): the real `ClientEndpoint` with ClockSync and the controller
//! against a model room on a model link, on injected clocks. The room ticks at 30 Hz on its own time (tick k due at
//! ⌊k × 10^9 / 30⌋ ns), answers each `Ping` in the instant it arrives (hold 0) with its next tick and its time, sends a
//! snapshot each tick acknowledging the newest sequence it received, and records when each stamped command first
//! arrived. The client's host clock is the room's plus a seeded offset; it steps at its own whole tick periods, so one
//! run's pings meet the room's grid at one phase (SC-1). The link follows npt-l1's profiles (l1/src/lib.rs:177-202:
//! each direction half the round trip, ± half its jitter, uniform, the loss each way, B1's 1 s burst) with part 13's
//! 1 % adjacent-pair swaps (:146). Every generator is splitmix64, seeded in each test.

use npt_client::ClientEndpoint;
use npt_link::Channel;
use npt_proto::{Direction, JoinAccepted, Message, Pong, Snapshot, decode, encode};
use npt_sim::Command;

const HZ: u32 = 30;
const MS: u64 = 1_000_000;
/// The lab's controller setting: the lead bound's 12 ticks less 3, and at most 9 (option B's buffer): npt-client's
/// `HONEST_MAX_LEAD` (C3).
const MAX_LEAD: u32 = npt_client::HONEST_MAX_LEAD;
/// The room's lead bound at 30 Hz (LEAD-BOUND: 400 ms).
const BOUND: i64 = 12;
/// The room's time at the join, ns.
const START: u64 = 20_000 * MS;

fn due(k: u32) -> u64 {
    u64::from(k) * 1_000_000_000 / u64::from(HZ)
}

/// The room's next tick to run at its time `s` (the I/O step runs before a tick due at the same instant).
fn next_at(s: u64) -> u32 {
    (u128::from(s) * u128::from(HZ)).div_ceil(1_000_000_000) as u32
}

fn mix(s: &mut u64) -> u64 {
    *s = s.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *s;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

fn below(s: &mut u64, n: u64) -> u64 {
    ((u128::from(mix(s)) * u128::from(n)) >> 64) as u64
}

#[derive(Clone, Copy, Debug)]
struct Profile {
    name: &'static str,
    one_way: u64,
    half_jitter: u64,
    loss_ppm: u64,
    /// B1: a 1 s burst of total loss from this room time.
    burst: Option<u64>,
}

const fn profile(
    name: &'static str,
    one_way_ms: u64,
    half_jitter_ms: u64,
    loss_ppm: u64,
) -> Profile {
    Profile {
        name,
        one_way: one_way_ms * MS,
        half_jitter: half_jitter_ms * MS,
        loss_ppm,
        burst: None,
    }
}

const P0: Profile = profile("P0", 0, 0, 0);
const P1: Profile = profile("P1", 25, 5, 10_000);
const P2: Profile = profile("P2", 50, 10, 20_000);
const P3: Profile = profile("P3", 75, 15, 50_000);
const P4: Profile = profile("P4", 100, 20, 50_000);
const B1: Profile = Profile {
    burst: Some(START + 40_000 * MS),
    ..profile("B1", 50, 10, 20_000)
};

/// One direction of the link: datagrams in flight as (arrival in room time, channel, bytes).
#[derive(Default)]
struct Way {
    flight: Vec<(u64, Channel, Vec<u8>)>,
    last: Option<usize>,
}

impl Way {
    fn send(&mut self, p: &Profile, rng: &mut u64, at: u64, channel: Channel, bytes: Vec<u8>) {
        let burst = p.burst.is_some_and(|b| at >= b && at < b + 1_000 * MS);
        if channel == Channel::Datagram && (burst || below(rng, 1_000_000) < p.loss_ppm) {
            return;
        }
        let jitter = below(rng, 2 * p.half_jitter + 1);
        let arrive = (at + p.one_way + jitter)
            .saturating_sub(p.half_jitter)
            .max(at + 1_000);
        self.flight.push((arrive, channel, bytes));
        let i = self.flight.len() - 1;
        // Part 13's step: 1 % of adjacent pairs swapped.
        if let Some(j) = self.last
            && below(rng, 1_000) < 10
            && self.flight[j].0 < arrive
        {
            self.flight[i].0 = self.flight[j].0;
            self.flight[j].0 = arrive;
        }
        self.last = Some(i);
    }

    /// Every datagram arrived by room time `t`, in arrival order.
    fn take(&mut self, t: u64) -> Vec<(u64, Channel, Vec<u8>)> {
        let mut got: Vec<(u64, Channel, Vec<u8>)> = Vec::new();
        let mut i = 0;
        while i < self.flight.len() {
            if self.flight[i].0 <= t {
                got.push(self.flight.swap_remove(i));
            } else {
                i += 1;
            }
        }
        self.last = None;
        got.sort_by_key(|g| g.0);
        got
    }
}

/// One run's settings.
#[derive(Clone, Copy)]
struct Opts {
    p: Profile,
    seed: u64,
    secs: u64,
    /// Stamps and ticks are measured from this many s.
    warm: u64,
    /// The host clock's offset from the room's, ns.
    offset: i64,
    /// The client's first step, ns into the room's tick grid.
    phase: u64,
    /// Read the clock's error at the first step at or after this long after the first sample, ns.
    clock_at: Option<u64>,
    /// At this room time, the profile becomes the second (a lead step-down).
    switch: Option<(u64, Profile)>,
    /// At this host time, the offset falls by this much (the room's time jumps ahead: a ClockSync reset).
    jump: Option<(u64, u64)>,
    /// At this host time, no input or step for this long (a host stall); then the host catches up its steps.
    stall: Option<(u64, u64)>,
    /// The lead and the stamps checked from the first step at or after this host time plus this many steps.
    check_from: Option<(u64, u32)>,
    /// The room's first snapshot this long after the join (its pongs go back at once).
    first_snapshot: u64,
}

fn opts(p: Profile, seed: u64, secs: u64) -> Opts {
    Opts {
        p,
        seed,
        secs,
        warm: 0,
        offset: 0,
        phase: 0,
        clock_at: None,
        switch: None,
        jump: None,
        stall: None,
        check_from: None,
        first_snapshot: 0,
    }
}

/// What a run measured.
#[derive(Debug, Default)]
struct Measured {
    /// Over the ticks after the warm-up: ticks, those whose stamped command arrived before they ran, and those with
    /// none arrived by then.
    ticks: u64,
    on_time: u64,
    missing: u64,
    pings: u64,
    offset_err_ns: Option<i64>,
    /// Each command's stamp strictly above its sequence's predecessor's, and at most the bound past the room's next
    /// tick at its first arrival.
    rise_ok: bool,
    bound_ok: bool,
    most_ahead: i64,
    /// The most commands one `Input` carried; the join's tick and the first stamp the room saw.
    most_per_input: usize,
    join_tick: u32,
    first_stamp: Option<u32>,
    /// Steps before the first snapshot at which a command was due (0 expected).
    due_before_snapshot: u64,
    /// From `check_from` on: the least lead set, the stamps first arrived, and those that arrived after their tick ran.
    lead_min_after: Option<i64>,
    arrived_after: u64,
    late_after: u64,
}

/// One run (the module text); the endpoint `with` adds to the controller's, returned with what it measured.
fn run(o: Opts, with: impl FnOnce(ClientEndpoint) -> ClientEndpoint) -> (Measured, ClientEndpoint) {
    let mut rng = o.seed;
    let mut p = o.p;
    let mut offset = o.offset;
    let mut ep = with(ClientEndpoint::new(npt_proto::Join::default(), HZ).with_clock(MAX_LEAD));
    let (mut up, mut down) = (Way::default(), Way::default());
    // 20 s in, so the host's time stays positive at offsets down to -10 s.
    let start = START + o.phase;
    let join_tick = next_at(start);
    let mut m = Measured {
        rise_ok: true,
        bound_ok: true,
        join_tick,
        ..Measured::default()
    };
    let mut accepted = Vec::new();
    let ja = JoinAccepted {
        slot: 0,
        tick: join_tick,
        ..JoinAccepted::default()
    };
    encode(&Message::JoinAccepted(ja), &mut accepted).expect("encodes");
    down.flight.push((start, Channel::Control, accepted));
    let end = start + o.secs * 1_000 * MS;
    let mut next_tick = join_tick;
    let mut newest_seq: Option<u16> = None;
    let mut arrived: Vec<Option<u64>> = Vec::new();
    // Each sequence's stamp and first arrival; the commands given (the next one's sequence), and the first checked.
    let mut by_seq: Vec<Option<(u32, u64)>> = vec![None; 1 << 16];
    let (mut given, mut mark, mut since, mut stalled) = (0u16, None, 0u32, false);
    let mut first_sample: Option<u64> = None;
    let mut host = (start as i64 + offset) as u64;
    loop {
        if let Some((at, fall)) = o.jump
            && host >= at
            && offset == o.offset
        {
            offset -= fall as i64;
        }
        if let Some((at, len)) = o.stall
            && host >= at
            && !stalled
        {
            stalled = true;
            host += len;
        }
        let now = (host as i64 - offset) as u64;
        if now > end {
            break;
        }
        if let Some((at, q)) = o.switch
            && now >= at
        {
            p = q;
        }
        // The room's events up to now: arrivals first at each instant, then the tick.
        loop {
            let next_up = up.flight.iter().map(|f| f.0).min();
            let tick_at = due(next_tick);
            match next_up {
                Some(a) if a <= tick_at && a <= now => {
                    for (a, _, bytes) in up.take(a) {
                        match decode(Direction::ClientToRoom, Channel::Datagram, &bytes) {
                            Ok(Message::Ping(pi)) => {
                                m.pings += 1;
                                let pong = Pong {
                                    id: pi.id,
                                    echo_ns: pi.sent_ns,
                                    tick: next_at(a),
                                    hold_us: 0,
                                    server_ns: a,
                                };
                                let mut out = Vec::new();
                                encode(&Message::Pong(pong), &mut out).expect("encodes");
                                down.send(&p, &mut rng, a, Channel::Datagram, out);
                            }
                            Ok(Message::Input(i)) => {
                                m.most_per_input = m.most_per_input.max(i.commands.len());
                                for c in i.commands.iter().rev() {
                                    m.first_stamp.get_or_insert(c.tick);
                                    let k = c.tick.wrapping_sub(join_tick) as usize;
                                    if k < 1 << 22 {
                                        if arrived.len() <= k {
                                            arrived.resize(k + 1, None);
                                        }
                                        arrived[k].get_or_insert(a);
                                    }
                                    if by_seq[usize::from(c.seq)].is_none() {
                                        by_seq[usize::from(c.seq)] = Some((c.tick, a));
                                        let ahead =
                                            i64::from(c.tick.wrapping_sub(next_at(a)) as i32);
                                        m.most_ahead = m.most_ahead.max(ahead);
                                        m.bound_ok &= ahead <= BOUND;
                                        let prev = by_seq[usize::from(c.seq.wrapping_sub(1))];
                                        if let Some((q, _)) = prev {
                                            m.rise_ok &= (c.tick.wrapping_sub(q) as i32) > 0;
                                        }
                                    }
                                    if newest_seq.is_none_or(|n| (c.seq.wrapping_sub(n) as i16) > 0)
                                    {
                                        newest_seq = Some(c.seq);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ if tick_at <= now => {
                    if tick_at >= start + o.first_snapshot {
                        let s = Snapshot {
                            tick: next_tick,
                            groups: 1,
                            newest_received: newest_seq,
                            ..Snapshot::default()
                        };
                        let mut out = Vec::new();
                        encode(&Message::Snapshot(s), &mut out).expect("encodes");
                        down.send(&p, &mut rng, tick_at, Channel::Datagram, out);
                    }
                    next_tick += 1;
                }
                _ => break,
            }
        }
        let received: Vec<(Channel, Vec<u8>, u64)> = down
            .take(now)
            .into_iter()
            .map(|(a, c, b)| (c, b, (a as i64 + offset) as u64))
            .collect();
        ep.input(None, &received, host);
        if ep.newest_tick().is_none() {
            m.due_before_snapshot += u64::from(ep.commands_due());
        }
        if let Some((t, n)) = o.check_from
            && host >= t
            && mark.is_none()
        {
            mark = (since == n).then_some(given);
            since += 1;
        }
        if mark.is_some() {
            let lead = ep.lead_now().unwrap_or(i64::MIN);
            m.lead_min_after = Some(m.lead_min_after.map_or(lead, |l| l.min(lead)));
        }
        for _ in 0..ep.commands_due() {
            ep.input(Some(Command::default()), &[], host);
            given = given.wrapping_add(1);
        }
        if first_sample.is_none() && !ep.clock().samples().is_empty() {
            first_sample = Some(host);
        }
        if let (Some(f), Some(after), None) = (first_sample, o.clock_at, m.offset_err_ns)
            && host >= f + after
        {
            m.offset_err_ns = ep.clock().offset_ns().map(|e| e - offset);
        }
        let out = ep.step();
        for d in out.datagrams {
            up.send(&p, &mut rng, now, Channel::Datagram, d);
        }
        // After a stall the steps due lie in the past: the host makes them at once.
        host = out.next_due_ns.max(host);
    }
    for (tick, a) in (mark.unwrap_or(given)..given).filter_map(|s| by_seq[usize::from(s)]) {
        m.arrived_after += 1;
        m.late_after += u64::from(a > due(tick));
    }
    let from = next_at(start + o.warm * 1_000 * MS).wrapping_sub(join_tick) as usize;
    let to = next_tick.wrapping_sub(join_tick) as usize;
    for k in from..to.saturating_sub(10) {
        m.ticks += 1;
        match arrived.get(k).copied().flatten() {
            Some(a) => m.on_time += u64::from(a <= due(join_tick + k as u32)),
            None => m.missing += 1,
        }
    }
    (m, ep)
}

/// f1 (-47; U4F-Q6 (b), D4's parameters): 1,000 seeded connects at P2 with its 2 % loss, each at one phase (its first
/// step a seeded share of a tick into the room's grid) and a seeded true offset within ±10 s: the p99 of |estimate -
/// true| at the first step at or after 2 s from the first sample is at most 4.0 ms (the bar's 5 ms less 1 ms for what
/// D4 did not model; a lowest-RTT pick sat at 4.856 ms there). Seed 0x5eed_00f1, 1,000 cases.
#[test]
fn f1_clocksync_p99_error_at_2_s_on_p2_is_at_most_4_ms_over_1_000_connects() {
    let mut s: u64 = 0x5eed_00f1;
    let mut errs: Vec<u64> = (0..1_000)
        .map(|_| {
            let mut o = opts(P2, mix(&mut s), 3);
            o.offset = below(&mut s, 20_000 * MS) as i64 - 10_000 * MS as i64;
            o.phase = below(&mut s, due(1));
            o.clock_at = Some(2_000 * MS);
            let (m, _) = run(o, |e| e);
            m.offset_err_ns.expect("an estimate at 2 s").unsigned_abs()
        })
        .collect();
    errs.sort_unstable();
    let p99 = errs[989];
    println!("f1: p99 |err| {p99} ns, max {}", errs[999]);
    assert!(p99 <= 4 * MS, "{p99}");
}

/// f4 (neg): the endpoint's pings never exceed the room's limit of 10 a second (catalogue :91; f31 pins the interval at
/// 60 Hz), so an honest client's `ping_limited` stays 0. Seed 4. Its negative half (a faster rate is limited and
/// counted by the pump) is U4e's e9 (server/tests/pump.rs:193-212).
#[test]
fn f4_the_endpoints_ping_rate_never_exceeds_the_rooms_limit() {
    let (m, _) = run(opts(P0, 4, 10), |e| e);
    assert!((98..=101).contains(&m.pings), "{} pings in 10 s", m.pings);
}

/// f5 (carry 74; TE-15; TE-U4F-1): the controller's settled lead per profile on a seeded 60 s trace: at least 1, at
/// most the lead bound less the margin (9), reported beside U4R-Q14's fixed leads (P0 2, P1 3, P2 and B1 5, P3 7, P4
/// 9); on every profile at most 1 % of stamped ticks late (the command arrived after its tick ran), counted apart from
/// missing (none arrived); and on the five without a burst at most 1 % late or missing (B1's missing ticks are its
/// 1 s of total loss; P3's own bar is f36's). Seed 5.
#[test]
fn f5_the_controllers_settled_lead_per_profile_is_bounded_and_late_at_most_1_percent() {
    for (p, u4r) in [(P0, 2), (P1, 3), (P2, 5), (B1, 5), (P3, 7), (P4, 9)] {
        let mut o = opts(p, 5, 60);
        o.warm = 10;
        o.offset = 123 * MS as i64;
        o.phase = 7 * MS;
        let (m, ep) = run(o, |e| e);
        let lead = ep.lead_now().expect("a lead");
        let ticks = m.ticks.max(1) as f64;
        let late = (m.ticks - m.on_time - m.missing) as f64 / ticks;
        let (not_on_time, skipped) = (1.0 - m.on_time as f64 / ticks, ep.stamps_skipped());
        println!(
            "f5 {}: settled lead {lead} (U4R-Q14's {u4r}), late {late:.5}, missing {}, not on time {not_on_time:.5}, most ahead {}, skipped {skipped}",
            p.name, m.missing, m.most_ahead
        );
        assert!((1..=9).contains(&lead), "{}: {lead}", p.name);
        assert!(late <= 0.01, "{}: {late}", p.name);
        assert!(p.burst.is_some() || not_on_time <= 0.01, "{}", p.name);
    }
}

/// f32 (TE-15, SA-8): honest stamps rise strictly and stay within the lead bound: a seeded property over P0-P4 and B1
/// traces of 3 s at seeded offsets and phases, read at the model room (each command's stamp above its predecessor's,
/// at most 12 past the room's next tick when it arrived). 1,000 cases (seed 0x5eed_0f32).
#[test]
fn f32_honest_stamps_rise_strictly_and_stay_within_the_lead_bound() {
    let mut s: u64 = 0x5eed_0f32;
    for case in 0..1_000u64 {
        let mut o = opts(
            [P0, P1, P2, P3, P4, B1][(case % 6) as usize],
            mix(&mut s),
            3,
        );
        o.offset = below(&mut s, 2_000 * MS) as i64 - 1_000 * MS as i64;
        o.phase = below(&mut s, due(1));
        let (m, ep) = run(o, |e| e);
        assert!(m.rise_ok && m.bound_ok, "case {case} ({}): {m:?}", o.p.name);
        assert!(ep.stamps_skipped() == 0, "case {case}");
    }
}

/// f32 (ii): through a lead step-down (P3 to P0 at 20 s) and a ClockSync reset (the room's time jumps 12 s ahead of the
/// host's at 40 s: each new sample moves the 2 s window's mean by more than 500 ms), the stamps still rise strictly and
/// stay within the bound, and the resets are counted. NC-U4F-1: the goal past the redundancy cap moves the target at
/// once (stamps skipped, counted), so from 2 s plus 10 steps after the jump the lead is at least 1 and every stamp
/// arrives before its tick runs (P0: no loss); and the same after a host stall (P0, no input or step for 1 s at 10 s)
/// from its third step on. Seed 32, both runs.
#[test]
fn f32_through_a_lead_step_down_and_a_clock_reset_stamps_rise_and_stay_within_the_bound() {
    let jump = START + 40_000 * MS;
    let mut o = opts(P3, 32, 60);
    o.switch = Some((START + 20_000 * MS, P0));
    o.jump = Some((jump, 12_000 * MS));
    o.check_from = Some((jump + 2_000 * MS, 10));
    let (m, ep) = run(o, |e| e);
    let stall = START + 10_000 * MS;
    let mut s = opts(P0, 32, 20);
    s.stall = Some((stall, 1_000 * MS));
    s.check_from = Some((stall + 1_000 * MS, 2));
    let (ms, es) = run(s, |e| e);
    println!(
        "f32 (ii): most ahead {}, resets {}, skipped {}, least lead {:?}, late {} of {}; stall: skipped {}, late {} of {}",
        m.most_ahead,
        ep.clock().resets(),
        ep.stamps_skipped(),
        m.lead_min_after,
        m.late_after,
        m.arrived_after,
        es.stamps_skipped(),
        ms.late_after,
        ms.arrived_after
    );
    assert!(m.rise_ok && m.bound_ok, "{m:?}");
    assert!(ep.clock().resets() >= 1);
    assert!(ep.stamps_skipped() > 0);
    assert!(m.lead_min_after >= Some(1), "{m:?}");
    assert!(m.arrived_after > 0 && m.late_after == 0, "{m:?}");
    assert!(ms.rise_ok && ms.bound_ok && es.stamps_skipped() > 0);
    assert!(ms.arrived_after > 0 && ms.late_after == 0, "{ms:?}");
}

/// f32 (iii) (TE31-4 = SA31-2): a join at room tick > 0 whose pongs arrive before its first snapshot (the room's
/// snapshots start 1 s after the join): no command is due and none stamped before the snapshot, and the first stamp is
/// the controller's target, above the join's tick and not counted up from it (at least the room's tick then). Seed 33.
#[test]
fn f32_no_command_is_due_before_the_first_snapshot_and_the_first_stamp_is_the_target() {
    let mut o = opts(P2, 33, 4);
    o.first_snapshot = 1_000 * MS;
    let (m, _) = run(o, |e| e);
    assert_eq!(m.due_before_snapshot, 0);
    let first = m.first_stamp.expect("a command");
    assert!(
        first > m.join_tick + 30,
        "{first} against the join's {}",
        m.join_tick
    );
}

/// f36 (i) (TE3-1; TE31-10, TE31-1): -20's two shares on a seeded P3 trace of 3,400 s (102,000 ticks), with its 5 % loss
/// each way and part 13's 1 % adjacent swaps, the endpoint's pings, controller and redundancy as built: at least 99.9 %
/// of stamped ticks had their command arrive before they ran, at most 0.1 % had none, and no stamp skipped. The
/// shares are by stamp, so one trace serves both options (the room's option changes what runs, not what arrived).
/// Seed 36.
#[test]
fn f36_p3_on_time_at_least_99_9_percent_and_missing_at_most_0_1_percent() {
    let mut o = opts(P3, 36, 3_400);
    o.warm = 10;
    o.offset = -2_345 * MS as i64;
    o.phase = 11 * MS;
    let (m, ep) = run(o, |e| e);
    let on_time = m.on_time as f64 / m.ticks as f64;
    let missing = m.missing as f64 / m.ticks as f64;
    println!(
        "f36: {} ticks, on time {on_time:.5}, missing {missing:.5}, lead {:?}, skipped {}",
        m.ticks,
        ep.lead_now(),
        ep.stamps_skipped()
    );
    assert!(m.ticks >= 100_000);
    assert!(on_time >= 0.999, "{on_time}");
    assert!(missing <= 0.001, "{missing}");
    assert_eq!(ep.stamps_skipped(), 0);
}

/// The endpoint's -47 and -20 bites (bites builds only), each shown against the clean endpoint.
#[cfg(feature = "bites")]
mod bites {
    use super::*;

    fn bitten(id: &'static str) -> impl FnOnce(ClientEndpoint) -> ClientEndpoint {
        move |e| e.with_bite(id).expect("a client bite")
    }

    /// PM-0001-47/newest-sample: the estimate is the newest raw sample (and differs from it on the clean endpoint).
    /// Seed 47.
    #[test]
    fn pm_0001_47_newest_sample_makes_the_estimate_the_newest_sample() {
        let mut equal = Vec::new();
        for bite in [false, true] {
            let mut o = opts(P2, 47, 3);
            o.offset = 777 * MS as i64;
            let (_, ep) = if bite {
                run(o, bitten("PM-0001-47/newest-sample"))
            } else {
                run(o, |e| e)
            };
            let newest = ep.clock().samples().back().map(|s| s.offset_ns);
            equal.push(newest.is_some() && ep.clock().offset_ns() == newest);
        }
        assert_eq!(equal, [false, true]);
    }

    /// PM-0001-47/tick-quantised: each sample is timed at the step that reads it: its arrival time is a step time.
    /// Seed 48.
    #[test]
    fn pm_0001_47_tick_quantised_moves_each_samples_time_to_its_step() {
        let mut on_steps = Vec::new();
        for bite in [false, true] {
            let mut o = opts(P2, 48, 3);
            o.phase = 5 * MS;
            let (_, ep) = if bite {
                run(o, bitten("PM-0001-47/tick-quantised"))
            } else {
                run(o, |e| e)
            };
            let base = START + o.phase;
            let all =
                ep.clock().samples().iter().all(|s| {
                    (0..200u64).any(|k| base + k * 1_000_000_000 / u64::from(HZ) == s.at_ns)
                });
            on_steps.push(all);
        }
        assert_eq!(on_steps, [false, true]);
    }

    /// PM-0001-20/redundancy-1: an `Input` holds one command (at P3 the clean endpoint's carry several). Seed 20.
    #[test]
    fn pm_0001_20_redundancy_1_makes_an_input_hold_one_command() {
        let mut most = Vec::new();
        for bite in [false, true] {
            let o = opts(P3, 20, 10);
            let (m, _) = if bite {
                run(o, bitten("PM-0001-20/redundancy-1"))
            } else {
                run(o, |e| e)
            };
            most.push(m.most_per_input);
        }
        assert!(most[0] > 1 && most[1] == 1, "{most:?}");
    }

    /// f30 (plan :399-401: bites per instance), the client half: a bitten and a clean endpoint on parallel threads each
    /// keep their own behaviour. Seed 30, both.
    #[test]
    fn f30_a_bitten_and_a_clean_endpoint_on_parallel_threads_each_keep_their_own() {
        let (clean, bit) = std::thread::scope(|sc| {
            let a = sc.spawn(|| run(opts(P3, 30, 6), |e| e).0.most_per_input);
            let b = sc.spawn(|| {
                run(opts(P3, 30, 6), bitten("PM-0001-20/redundancy-1"))
                    .0
                    .most_per_input
            });
            (a.join().unwrap_or(0), b.join().unwrap_or(0))
        });
        assert!(clean > 1 && bit == 1, "{clean} {bit}");
    }
}
