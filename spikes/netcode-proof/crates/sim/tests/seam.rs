//! U4s's tests (u4-build-plan.md §5 U4s): s1 the tick constants at 30 and 60 Hz, s2 the wire grid each way, s3 its tie
//! rule, s4 the hit history's ring (and s4b its refusal of a zero span), s5 the seam types are `Send`, s6 each entity
//! kind's field layout; and, in bites builds, the grid's one planted defect and the own-copy snap's bite id.

use npt_sim::grid::{self, Config, Field, PITCH, POSITION_XY, POSITION_Z, VELOCITY, YAW};
use npt_sim::{
    ACK_RING_TICKS, ClampReport, Command, CoverVariant, EntityKind, EntityRecord,
    FULL_STATE_ACK_TICKS, HitHistory, HitVolumes, InputReports, LOOP_CATCHUP, PortalStates, Rewind,
    SETTINGS_STEP_TICKS, SimCore, SimEvent, TICK_HZ, TickInput, ack_ring_ticks_at,
    full_state_ack_ticks_at, loop_catchup_at, settings_step_ticks_at, ticks_for_ms,
    ticks_for_ms_at,
};

/// A derived constant's name, its value, its function of the tick rate, and the function's values at 30 and 60 Hz.
type Derived = (&'static str, u32, fn(u32) -> u32, u32, u32);

/// Each derived constant, its function of the tick rate, and that function's pinned values at 30 and at 60 Hz
/// (u4-build-plan.md §4.3). check.mjs's deps step holds each constant's initializer to `<name>_at(TICK_HZ)`, and
/// ticks.rs's const asserts hold each constant equal to its function at whatever TICK_HZ is, so the function's value at
/// 60 Hz is the constant the 60 Hz build gets.
const DERIVED: [Derived; 4] = [
    ("LOOP_CATCHUP", LOOP_CATCHUP, loop_catchup_at, 7, 15),
    (
        "FULL_STATE_ACK_TICKS",
        FULL_STATE_ACK_TICKS,
        full_state_ack_ticks_at,
        60,
        120,
    ),
    ("ACK_RING_TICKS", ACK_RING_TICKS, ack_ring_ticks_at, 32, 64),
    (
        "SETTINGS_STEP_TICKS",
        SETTINGS_STEP_TICKS,
        settings_step_ticks_at,
        30,
        60,
    ),
];

const FIELDS: [(&str, Field); 5] = [
    ("POSITION_XY", grid::POSITION_XY),
    ("POSITION_Z", grid::POSITION_Z),
    ("YAW", grid::YAW),
    ("PITCH", grid::PITCH),
    ("VELOCITY", grid::VELOCITY),
];

/// Each field's scale in grid steps per sim unit, as u4-build-plan.md §4.4 states the grid: 1 cm (and 1 cm/s), yaw's
/// 16 bits over the turn, pitch's ±511 over ±90 degrees. s2 checks `from_grid` against it for every integer.
fn scale(f: Field) -> f64 {
    if f == grid::YAW {
        65_536.0 / core::f64::consts::TAU
    } else if f == grid::PITCH {
        511.0 / core::f64::consts::FRAC_PI_2
    } else {
        100.0
    }
}

/// One tick's hit volumes, as the tests push them (U4d's World gives them their volumes).
fn volumes(tick: u32) -> HitVolumes {
    HitVolumes { tick }
}

/// One unit in the last place of a finite f64, by its bits (std's next_up is outside the sim's float allow-list).
fn ulp(s: f64) -> f64 {
    let a = s.abs();
    f64::from_bits(a.to_bits() + 1) - a
}

/// The grid's error at `x`, in steps: the integer the grid gives less the scaled value, around the turn for yaw.
fn error_in_steps(f: Field, x: f32, k: i32) -> f64 {
    let s = f64::from(x) * scale(f);
    let d = f64::from(k) - s;
    if f.wraps() {
        let turn = f64::from(f.max()) - f64::from(f.min()) + 1.0;
        d - turn * (d / turn).round()
    } else {
        d
    }
}

#[test]
fn s1_every_tick_constant_is_its_function_of_the_tick_rate_at_30_and_60_hz() {
    // Each derived constant is its function at TICK_HZ, and the function is pinned at both rates (u4-build-plan.md
    // §4.3): catch-up 7 and 15, the FullState bound 60 and 120, the AckRing 32 and 64, the ClientSettings step 30 and 60.
    for (name, constant, at, at_30, at_60) in DERIVED {
        assert_eq!(constant, at(TICK_HZ), "{name} is its function at TICK_HZ");
        assert_eq!(at(30), at_30, "{name} at 30 Hz");
        assert_eq!(at(60), at_60, "{name} at 60 Hz");
    }
    // The FullState bound is 2 s at every rate, and the AckRing stays small at the highest rate TICK_HZ may take.
    for hz in [1, 30, 60, 1_000] {
        assert_eq!(full_state_ack_ticks_at(hz), ticks_for_ms_at(hz, 2_000));
    }
    assert_eq!(ack_ring_ticks_at(1_000), 1_066);
    // ticks_for_ms rounds up, at both rates.
    for (hz, ms, ticks) in [
        (30, 0, 0),
        (30, 1, 1),
        (30, 33, 1),
        (30, 34, 2),
        (30, 1_000, 30),
        (60, 16, 1),
        (60, 17, 2),
        (60, 1_000, 60),
    ] {
        assert_eq!(ticks_for_ms_at(hz, ms), ticks, "{ms} ms at {hz} Hz");
    }
    assert_eq!(
        ticks_for_ms_at(1_000, u32::MAX),
        u32::MAX,
        "no overflow at the largest rate"
    );
    // The build's rate is 30, and ticks_for_ms is ticks_for_ms_at at it.
    assert_eq!(TICK_HZ, 30);
    for ms in [0, 1, 33, 34, 2_000, 100_000] {
        assert_eq!(ticks_for_ms(ms), ticks_for_ms_at(TICK_HZ, ms));
    }
}

#[test]
fn s2_from_then_to_returns_every_integer_of_every_field() {
    let g = Config::default();
    let mut seen = 0u64;
    for (name, f) in FIELDS {
        assert!(f.min() < f.max());
        let span = i64::from(f.max()) - i64::from(f.min()) + 1;
        assert!(
            span <= 1 << f.bits(),
            "{name}: {span} integers in {} bits",
            f.bits()
        );
        for k in f.min()..=f.max() {
            let x = g.from_grid(f, k);
            assert_eq!(
                x.to_bits(),
                ((f64::from(k) / scale(f)) as f32).to_bits(),
                "{name}: {k}'s value"
            );
            assert_eq!(g.to_grid(f, x), k, "{name}: {k} came back from {x:e}");
            assert_eq!(
                g.snap(f, x).to_bits(),
                x.to_bits(),
                "{name}: a grid value snaps to itself"
            );
            seen += 1;
        }
    }
    // Every integer of every field: 65,536 + 4,096 + 65,536 + 1,023 + 65,536.
    assert_eq!(seen, 201_727);
}

#[test]
fn s2_to_errs_by_at_most_half_a_step_plus_one_ulp_over_the_half_steps_and_the_range_ends() {
    let g = Config::default();
    let mut points = 0u64;
    for (name, f) in FIELDS {
        let sc = scale(f);
        // Every integer of the narrow fields, every 7th of the wide ones, and always both ends and their neighbours.
        let stride = if f.bits() > 12 { 7 } else { 1 };
        let mut ks: Vec<i32> = (f.min()..=f.max()).step_by(stride).collect();
        ks.extend([f.min(), f.min() + 1, f.max() - 1, f.max()]);
        for k in ks {
            // The grid point, the quarter steps and the half steps on both sides, each as the nearest f32 and its two
            // neighbours.
            for quarter in -3..=3 {
                let at = (f64::from(k) + f64::from(quarter) / 4.0) / sc;
                let x = at as f32;
                for x in [
                    x,
                    f32::from_bits(x.to_bits().wrapping_add(1)),
                    f32::from_bits(x.to_bits().wrapping_sub(1)),
                ] {
                    if !x.is_finite() {
                        continue;
                    }
                    let s = f64::from(x) * sc;
                    // Past the range's half steps the grid saturates: not an error bound.
                    if !f.wraps() && (s < f64::from(f.min()) - 0.5 || s > f64::from(f.max()) + 0.5)
                    {
                        continue;
                    }
                    let got = g.to_grid(f, x);
                    let e = error_in_steps(f, x, got);
                    assert!(
                        e.abs() <= 0.5 + ulp(s),
                        "{name}: {x:e} (scaled {s}) went to {got}, {e} steps off"
                    );
                    points += 1;
                }
            }
        }
    }
    assert!(points > 300_000, "{points} points");
    // Past the ends: a range field saturates, yaw wraps, NaN is 0.
    assert_eq!(g.to_grid(grid::POSITION_Z, -1.0), 0);
    assert_eq!(g.to_grid(grid::POSITION_Z, 100.0), 4_095);
    assert_eq!(g.to_grid(grid::POSITION_XY, f32::INFINITY), 32_767);
    assert_eq!(g.to_grid(grid::POSITION_XY, f32::NEG_INFINITY), -32_768);
    assert_eq!(g.to_grid(grid::PITCH, 2.0), 511);
    assert_eq!(g.to_grid(grid::VELOCITY, f32::NAN), 0);
    assert_eq!(
        g.to_grid(grid::YAW, core::f32::consts::PI),
        -32_768,
        "π is -π around the turn"
    );
    assert_eq!(g.to_grid(grid::YAW, -core::f32::consts::PI), -32_768);
    assert_eq!(
        g.to_grid(grid::YAW, 3.0 * core::f32::consts::FRAC_PI_2),
        -16_384
    );
}

#[test]
fn s3_a_tie_rounds_away_from_zero() {
    let g = Config::default();
    // Metres whose centimetre value is exactly k + 0.5 (dyadic, so the f32 and the f64 product are exact): ties away
    // from zero, not to even (12.5 → 13, 62.5 → 63) and not upward (-12.5 → -13).
    for (x, k) in [
        (0.125, 13),
        (-0.125, -13),
        (0.375, 38),
        (-0.375, -38),
        (0.625, 63),
        (-0.625, -63),
        (1.125, 113),
        (-1.125, -113),
    ] {
        for f in [grid::POSITION_XY, grid::VELOCITY] {
            assert_eq!(g.to_grid(f, x), k, "{x}");
        }
        if x > 0.0 {
            assert_eq!(g.to_grid(grid::POSITION_Z, x), k, "{x}");
        }
    }
    // Yaw's and pitch's scales are not dyadic, so no f32 lands exactly on their half steps: the same rounding serves
    // them (s2 bounds their error).
}

#[test]
fn s4_the_hit_history_keeps_exactly_its_capacity_and_evicts_the_oldest() {
    // Spans in ms (the ring is built from one: u4-build-plan.md §4.9), rounded up to whole ticks: at 30 Hz 64, 32, 2, 1
    // and 1 ticks.
    let spans = [2_133, 1_066, 66, 33, 1];
    assert_eq!(spans.map(ticks_for_ms), [64, 32, 2, 1, 1]);
    for span_ms in spans {
        let capacity = ticks_for_ms(span_ms);
        let mut h = HitHistory::for_ms(span_ms).expect("a span above 0 ms holds a tick");
        assert_eq!(h.capacity(), capacity);
        assert!(h.is_empty());
        // Ticks that cross the u32 wrap.
        let start = u32::MAX - 40;
        for i in 0..capacity + 50 {
            let tick = start.wrapping_add(i);
            h.push(&volumes(tick));
            let held = (i + 1).min(capacity);
            assert_eq!(
                h.len(),
                held,
                "capacity {capacity}: {held} held after {} pushes",
                i + 1
            );
            let oldest = tick.wrapping_sub(held - 1);
            for t in 0..held {
                let t = oldest.wrapping_add(t);
                assert_eq!(
                    h.get(t).map(|v| v.tick),
                    Some(t),
                    "capacity {capacity}: tick {t} held"
                );
            }
            if i >= capacity {
                assert!(
                    h.get(oldest.wrapping_sub(1)).is_none(),
                    "capacity {capacity}: the oldest evicted"
                );
            }
            assert!(h.get(tick.wrapping_add(1)).is_none(), "no future tick");
            assert!(!h.is_empty());
        }
        assert_eq!(h.capacity(), capacity);
    }
}

#[test]
fn s4b_a_hit_history_of_zero_ms_is_refused() {
    // A span of 0 ms holds no tick, so the ring refuses it rather than build one that could never hold its newest tick.
    assert!(HitHistory::for_ms(0).is_none(), "0 ms is refused");
    // Any span above it rounds up to at least one tick.
    let one = HitHistory::for_ms(1).expect("1 ms holds a tick");
    assert_eq!(one.capacity(), 1);
}

/// A stand-in implementation: the seam is a trait a sim can implement with these types, and a room generic over it.
struct Stub {
    events: Vec<SimEvent>,
    records: Vec<EntityRecord>,
    volumes: HitVolumes,
    portals: PortalStates,
    reports: InputReports,
}

/// Whether the lent ring holds what a Rewind names (seam.rs's rule): `tick_a`, and `tick_a + 1` when it blends toward it.
fn held(history: &HitHistory, r: &Rewind) -> bool {
    history.get(r.tick_a).is_some()
        && (r.fraction == 0 || history.get(r.tick_a.wrapping_add(1)).is_some())
}

impl SimCore for Stub {
    fn step(&mut self, input: &TickInput, history: &HitHistory) -> &[SimEvent] {
        self.events.clear();
        for (c, rewind) in &input.commands {
            if rewind.is_some_and(|r| held(history, &r)) {
                self.events.push(SimEvent::Shot {
                    slot: c.slot,
                    seq: c.seq,
                });
            }
        }
        self.volumes.tick = input.tick;
        &self.events
    }

    fn state_hash(&self) -> u64 {
        u64::from(self.volumes.tick)
    }

    fn entity_records(&self) -> &[EntityRecord] {
        &self.records
    }

    fn hit_volumes(&self) -> &HitVolumes {
        &self.volumes
    }

    fn portal_states(&self) -> &PortalStates {
        &self.portals
    }

    fn reports(&self) -> &InputReports {
        &self.reports
    }
}

/// What a room does each tick: step, then push the new volumes into its ring.
fn room_tick<S: SimCore + Send>(sim: &mut S, history: &mut HitHistory, input: &TickInput) -> usize {
    let n = sim.step(input, history).len();
    history.push(sim.hit_volumes());
    n
}

#[test]
fn s5_the_seam_types_are_send() {
    fn send<T: Send>() {}
    send::<Command>();
    send::<CoverVariant>();
    send::<Rewind>();
    send::<TickInput>();
    send::<EntityKind>();
    send::<EntityRecord>();
    send::<HitVolumes>();
    send::<HitHistory>();
    send::<PortalStates>();
    send::<SimEvent>();
    send::<ClampReport>();
    send::<InputReports>();
    send::<Field>();
    send::<Config>();
    send::<Stub>();
    // And the trait carries a tick: a shot whose Rewind the lent ring holds runs; one outside it does not (a tick the
    // ring lacks, or a blend toward a tick it lacks), and a refused Rewind is as none.
    let mut sim = Stub {
        events: Vec::new(),
        records: vec![EntityRecord {
            kind: EntityKind::Soldier,
            index: 0,
            generation: 0,
            area: 0,
            grid: [0; EntityRecord::FIELDS],
            raw: [0; EntityRecord::FIELDS],
        }],
        volumes: HitVolumes::default(),
        portals: PortalStates::default(),
        reports: InputReports::default(),
    };
    let mut history = HitHistory::for_ms(1_000).expect("1 s of ticks");
    history.push(&volumes(0));
    let fire = |tick_a, fraction| Rewind {
        tick_a,
        fraction,
        cover: CoverVariant::Rewound,
    };
    let input = TickInput {
        tick: 1,
        commands: vec![
            (
                Command {
                    slot: 1,
                    seq: 7,
                    ..Command::default()
                },
                Some(fire(0, 0)),
            ),
            (
                Command {
                    slot: 2,
                    seq: 9,
                    ..Command::default()
                },
                Some(fire(99, 0)),
            ),
            (
                Command {
                    slot: 3,
                    ..Command::default()
                },
                None,
            ),
            (
                Command {
                    slot: 4,
                    seq: 2,
                    ..Command::default()
                },
                Some(fire(0, 128)),
            ),
        ],
    };
    assert_eq!(room_tick(&mut sim, &mut history, &input), 1);
    assert_eq!(sim.events, [SimEvent::Shot { slot: 1, seq: 7 }]);
    assert_eq!(history.get(1).map(|v| v.tick), Some(1));
    assert_eq!(sim.state_hash(), 1);
    assert_eq!(sim.entity_records().len(), 1);
    assert!(sim.portal_states().open.is_empty() && sim.reports().clamps.is_empty());
}

#[test]
fn s6_every_kinds_layout_fits_a_record_and_each_field_fits_its_bits() {
    assert_eq!(EntityKind::ALL.len(), 7);
    for (i, kind) in EntityKind::ALL.into_iter().enumerate() {
        // In declaration order, each kind once.
        assert!(EntityKind::ALL[..i].iter().all(|k| *k < kind), "{kind:?}");
        let layout = kind.layout();
        assert!(layout.len() <= EntityRecord::FIELDS, "{kind:?}");
        for f in layout {
            let span = i64::from(f.max()) - i64::from(f.min()) + 1;
            assert!(
                span <= 1 << f.bits() && span > 1 << (f.bits() - 1),
                "{kind:?}: {span} integers in {} bits, the fewest",
                f.bits()
            );
        }
    }
    // The catalogue's rows (docs/design/netcode-proof-messages.md §3), U4d to complete: a soldier's position, yaw,
    // pitch, velocity and 8 flag bits; a projectile's start tick (32 bits), start position and velocity.
    let flags = Field::integer(0, 255);
    assert_eq!(
        EntityKind::Soldier.layout(),
        [
            POSITION_XY,
            POSITION_XY,
            POSITION_Z,
            YAW,
            PITCH,
            VELOCITY,
            VELOCITY,
            VELOCITY,
            flags
        ]
    );
    let tick = Field::integer(i32::MIN, i32::MAX);
    assert_eq!(
        EntityKind::Projectile.layout(),
        [
            tick,
            POSITION_XY,
            POSITION_XY,
            POSITION_Z,
            VELOCITY,
            VELOCITY,
            VELOCITY
        ]
    );
    // An integer field takes the fewest bits that hold its range, never wraps, and is its own kind: never equal to a
    // grid field of the same integers.
    for (min, max, bits) in [
        (0, 1, 1),
        (0, 255, 8),
        (0, 256, 9),
        (-1, 0, 1),
        (-511, 511, 10),
        (i32::MIN, i32::MAX, 32),
    ] {
        let f = Field::integer(min, max);
        assert_eq!(
            (f.min(), f.max(), f.bits(), f.wraps()),
            (min, max, bits, false),
            "{min}..={max}"
        );
    }
    assert_ne!(Field::integer(-32_768, 32_767), POSITION_XY);
    assert_ne!(POSITION_XY, VELOCITY, "one range, two kinds");
}

/// The share of position values from 0 to 20 m, in sixteenths of a centimetre, on which `a` and `b` differ.
#[cfg(feature = "bites")]
fn differing_share(a: impl Fn(f32) -> f32, b: impl Fn(f32) -> f32) -> f64 {
    let (mut differ, mut all) = (0u32, 0u32);
    for k in 0..2_000 {
        for sixteenth in 0..16 {
            let x = ((f64::from(k) + f64::from(sixteenth) / 16.0) / 100.0) as f32;
            differ += u32::from(a(x).to_bits() != b(x).to_bits());
            all += 1;
        }
    }
    f64::from(differ) / f64::from(all)
}

#[cfg(feature = "bites")]
#[test]
fn the_grids_one_defect_rounds_down_and_moves_about_half_of_all_positions_off_the_wire_grid() {
    let (wire, floor) = (Config::default(), Config::floor_rounding());
    assert_ne!(floor, wire);
    // Down, not to the nearest: 12.5 cm and 12.9 cm to 12, -12.5 cm to -13; a grid value stays itself.
    assert_eq!(floor.to_grid(POSITION_XY, 0.125), 12);
    assert_eq!(floor.to_grid(POSITION_XY, 0.129), 12);
    assert_eq!(floor.to_grid(POSITION_XY, -0.125), -13);
    // Each field's ends still hold, and from_grid is the wire grid's.
    for (name, f) in FIELDS {
        for k in [f.min(), f.min() + 1, 0.max(f.min()), f.max() - 1, f.max()] {
            let x = wire.from_grid(f, k);
            assert_eq!(floor.from_grid(f, k).to_bits(), x.to_bits(), "{name}: {k}");
            let got = floor.to_grid(f, x);
            assert!((f.min()..=f.max()).contains(&got), "{name}: {k} → {got}");
        }
        assert_eq!(
            floor.to_grid(f, f32::INFINITY),
            wire.to_grid(f, f32::INFINITY)
        );
    }
    // About half of all positions off the wire grid (the share -17's and -41's unit proofs rest on).
    let share = differing_share(
        |x| floor.from_grid(POSITION_XY, floor.to_grid(POSITION_XY, x)),
        |x| wire.from_grid(POSITION_XY, wire.to_grid(POSITION_XY, x)),
    );
    assert!((0.45..=0.6).contains(&share), "{share}");
}

#[cfg(feature = "bites")]
#[test]
fn pm_0001_41_server_quant_differs_is_planted_at_the_own_copy_snap_alone() {
    let id = "PM-0001-41/server-quant-differs";
    assert_eq!(npt_sim::OWN_COPY_SNAP_BITES, [id]);
    let own = npt_sim::own_copy_snap_with_bite(id).expect("the own-copy snap plants -41's");
    assert_eq!(own, Config::floor_rounding());
    // No id of another site: the client's (-17), the encoder's (-57), the authority step's other (-41's speed bite,
    // U4d's World), an unknown one.
    for other in [
        "PM-0001-17/client-rounding",
        "PM-0001-57/pos-4cm",
        "PM-0001-57/yaw-12bit",
        "PM-0001-57/pitch-8bit",
        "PM-0001-41/server-speed-1pct",
        "PM-0001-41/no-such-bite",
    ] {
        assert!(npt_sim::own_copy_snap_with_bite(other).is_none(), "{other}");
    }
    // u4-build-plan.md §4.14's unit proof: the snapped copy differs from the one sent. The record carries the wire
    // grid's integer (EntityRecord's rule), the bitten snap keeps its own: about half of all positions differ.
    let wire = Config::default();
    let share = differing_share(
        |x| own.snap(POSITION_XY, x),
        |x| wire.from_grid(POSITION_XY, wire.to_grid(POSITION_XY, x)),
    );
    assert!((0.45..=0.6).contains(&share), "{share}");
}
