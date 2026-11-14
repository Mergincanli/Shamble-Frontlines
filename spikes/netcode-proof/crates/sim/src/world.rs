//! `World`, the stand-in sim's one implementation of the seam (u4-build-plan.md §5 U4d, §4.2; plan E1).
//!
//! **One step, in phases** (each a function below, in this order):
//! 0. the tick's spawns, teleports and attribute changes ([`TickInput`]);
//! 1. the movers: the wall stand-in falls or rises, one door toggles each second, the vehicles drive their loops; their
//!    records are written, and the soldiers' collision movers are read back from those records' grid integers, so the
//!    authority collides against exactly what a client decodes ([`World::load_movers`]);
//! 2. the commands, in the canonical order: per command, the soldier's move (one tick of it), the own-copy snap, its
//!    aim (the clamp) and its weapon (fire, reload, throw);
//! 3. the present hit volumes, from the soldiers' records after the moves ([`World::hit_volumes_from`]);
//! 4. the shots: each against the lent ring blended at its [`Rewind`], or at the present;
//! 5. the projectiles: blasts at their fuse, the stand-ins kept live (or spawned at the scene's rate);
//! 6. S1's effect rows: each row's end checked, a row that ended rolled again;
//! 7. respawns;
//! 8. the records, the volumes (from those records, [`World::hit_volumes_from`]) and the portals of the tick.
//!
//! **The records** (the advisor's W1-S1 and W1-U4D): a soldier's record is the catalogue's remote row, and its owner's
//! [`EntityKind::OwnState`] record holds the rest of what decides its next ticks; a vehicle part's record is its
//! transform relative to its hull, so a rigid part's record changes only when its health does, and every world box of
//! a part is its hull's record composed with its own by one function; an effect row's record holds the tick it ends,
//! so it changes only when the row starts or ends.
//!
//! **Rules the tests hold** (D-1 to D-10; the carries of u4s-carries.md):
//! - The state reads, of a [`Command`], `slot`, `seq`, `buttons`, `presses`, `move_xy` and `aim`, nothing else; the
//!   order assert reads `tick` (carry 7, 8).
//! - **A command that holds or presses fire and has no Rewind fires at the present**: its shot meets the soldiers and
//!   the cover as they are after this tick's moves. A refused Rewind (a tick the lent ring does not hold, or a blend
//!   toward one: [`Rewind`]'s rule) is counted and the command then runs exactly as with `None`: one rule for both
//!   (carry 10).
//! - **Refused input leaves no trace in the state** (the advisor's condition 1): a fire press in the cooldown or with
//!   no ammunition, a reload press the weapon refuses, a throw into a full pool and a Rewind outside the ring are
//!   counted in [`InputReports`], each listed there as a [`crate::Refusal`] (slot, sequence, kind) for the tick, and
//!   change nothing hashed; a clamped aim point is reported there and the shot uses the clamped point. The world's one
//!   piece of per-slot input bookkeeping is the press counters of the last command it ran for that slot: a press edge
//!   is a counter that differs from them, and a refused press's edge is consumed there and changes nothing hashed. They
//!   stay outside the hash and the dump (a twin's counters differ by design) and are a function of the command stream
//!   alone. A refused press's twin is the same stream with that press removed: that button's counter one lower from
//!   the refused command on, every other field equal (the advisor's W1-U4D ruling).
//! - Integers come in widened before any arithmetic (`move_xy` to i32, `aim` to f64), so no recorded command trips an
//!   overflow check (carry 15); every float the state holds is finite, asserted each tick in debug builds (D-4).
//! - `HitVolumes::tick` is the world's own tick counter, never `TickInput::tick` (carry 15).
//! - The own-copy snap (the advisor's Q23 ruling) ends every command's move, in `step` and in [`World::predict`] alike;
//!   each soldier's records carry the values before it in `raw` and the wire grid's integers of them in `grid` (carry
//!   9): position, yaw and pitch in its soldier record, velocity in its own-state record.
//! - **The clamp's bound** (the advisor's W1-U4D TUNE-2): an aim point that came over the wire lies inside the grid's
//!   ranges, so for a fire origin at or below the grid's top, 40.95 m (feet at or below 39.55 m, 3,955 cm), the
//!   point at the reach lies between the origin and the aim point and so inside those ranges
//!   too: the clamp puts it at the reach (within one cell's half-diagonal) and a clamped point sent again is not moved.
//!   Above it a reach point can leave the grid's z range, saturates there, and is not claimed to lie at the reach
//!   (measured: the second clamp first moves a sent-back point for feet at about 40.57 m, the test review's probe A).
//!   No scene places a fire origin near it: every spawn point, plate and greybox surface is below 10 m (a test).

use core::f32::consts::{FRAC_PI_2, TAU};

use crate::greybox::{Aabb, Greybox};
use crate::grid::{Config, Field, PITCH, POSITION_XY, POSITION_Z, VELOCITY, YAW};
use crate::hash::{Finite, Fnv, Sink, StateDump};
use crate::history::HitHistory;
use crate::rng::{STREAMS, SimRng, Stream};
use crate::scene::{LARGE_PARTS, SMALL_PARTS, Scene};
use crate::seam::{
    Attribute, ClampReport, Command, CoverVariant, EntityKind, EntityRecord, FIRE_INTERVAL_MAX_US,
    HitVolumes, InputReports, MOVE_SPEED_MAX_CM_S, PortalStates, RELOAD_MAX_MS, Refusal,
    RefusalKind, Rewind, SimCore, SimEvent, TickInput, UNITS_PER_TICK, VolumeBox,
};
use crate::{TICK_HZ, ticks_for_ms, trajectory_at};

/// The clamp's bound (the module text, TUNE-2): the highest feet, cm, whose fire origin ([`CHEST`] above them) is at
/// or below the position grid's top, 4,095 cm. Its test holds every scene far below it.
#[cfg(test)]
const FEET_Z_BOUND_CM: i32 = POSITION_Z.max() - CHEST_CM;
/// One tick, in seconds.
const DT: f32 = 1.0 / TICK_HZ as f32;

/// A soldier's half width (x and y) and height, metres, for collision.
const RADIUS: f32 = 0.4;
const HEIGHT: f32 = 1.8;
/// The fire origin's height above the soldier's feet, cm and metres.
const CHEST_CM: i32 = 140;
const CHEST: f32 = CHEST_CM as f32 / 100.0;
/// A soldier's hit box, half extents in cm (80 × 80 × 180 cm), and a plate's (100 × 2 × 100 cm: TC's thin targets).
const SOLDIER_HALF: [u16; 3] = [40, 40, 90];
const PLATE_HALF: [u16; 3] = [50, 1, 50];
/// A vehicle part: a 1 m cube whose centre is 1 m above its hull's origin (its record's relative z).
const PART_HALF_CM: i32 = 50;
const PART_Z_CM: i32 = 100;
const PART_HEALTH: u16 = 200;
/// The most vehicles a scene has: a part's record names its hull in [`EntityKind::VehiclePart`]'s 6-bit parent field.
const MAX_VEHICLES: usize = 64;
/// The fields of a record [`World`] reads back from records (each kind's [`EntityKind::layout`] order): a soldier's
/// 8 flag bits; a part's hull; a door's state; the wall's standing.
const SOLDIER_FLAGS: usize = 5;
const PART_PARENT: usize = 5;
const DOOR_OPEN: usize = 0;
const WALL_STANDING: usize = 1;
/// A box blocks a move only past this overlap, metres, so a soldier pushed to touch it slides along it.
const SKIN: f32 = 0.001;
/// A box the soldier starts more than this far inside, metres, never blocks it (a door that closed on it, the wall
/// that rose round it): it walks out.
const INSIDE: f32 = 0.01;
/// The clamp's slack, metres: one grid cell's half-diagonal, √3/2 cm = 0.866 cm, rounded up (§4.2).
const CLAMP_SLACK: f64 = 0.0087;
/// A grenade: its throw speed and lift (m/s), its blast's radius (m) and damage; its fuse is the scene's.
const THROW_SPEED: f32 = 14.0;
const THROW_LIFT: f32 = 4.0;
const BLAST_RADIUS: f32 = 4.0;
const BLAST_DAMAGE: u16 = 50;
/// Projectile types (the record's type field).
const STAND_IN: u8 = 0;
const GRENADE: u8 = 1;
/// The wall stand-in's health, and how long it stays down once destroyed (ms).
const WALL_HEALTH: u16 = 500;
const WALL_DOWN_MS: u32 = 10_000;

/// The soldier record's flag bits (its 8 flag bits for stance and action states, which change rarely: ADR
/// 0001:309-313). A shot is no flag: it is the tick's [`SimEvent::Shot`] (the advisor's W1-U4D N-M3).
const FLAG_CROUCH: u8 = 1 << 0;
const FLAG_SPRINT: u8 = 1 << 1;
const FLAG_RELOADING: u8 = 1 << 2;
const FLAG_DEAD: u8 = 1 << 3;

/// The sim's own float helpers (ADR 0004's allow-list replaces std's min and max).
fn fmin(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}

fn fmax(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}

/// A grid integer of a position field as metres: the wire grid's `from_grid` (scale 100).
fn cm(k: i32) -> f32 {
    Config::default().from_grid(POSITION_XY, k)
}

fn sub(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn length(d: [f32; 3]) -> f32 {
    (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt()
}

/// An integer brought into a field's range.
fn fit(field: Field, k: i32) -> i32 {
    k.clamp(field.min(), field.max())
}

/// A segment of a ray cast ([`World::ray_cast`]): from `origin` to `to`, metres, passing through the soldier `skip`
/// (the shooter's own box).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    /// Where it starts.
    pub origin: [f32; 3],
    /// Where it ends.
    pub to: [f32; 3],
    /// A soldier the ray passes through: the one casting it.
    pub skip: Option<u8>,
}

/// What a ray met first, and where: `at` is the fraction of the way from its origin to its end, 0 to 1.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RayHit {
    /// Nothing before its end, or a ray that is not finite.
    Miss,
    /// The static world: the map's edge, a building's wall, a crate.
    World {
        /// The fraction of the way.
        at: f32,
    },
    /// A cover item: [`HitVolumes::cover`]'s index (a vehicle part, then a door, then the wall stand-in).
    Cover {
        /// Its index in the movers' `cover`.
        index: u16,
        /// The fraction of the way.
        at: f32,
    },
    /// A soldier, by slot.
    Soldier {
        /// The soldier's slot.
        slot: u8,
        /// The fraction of the way.
        at: f32,
    },
}

/// A box in metres, for the ray cast.
#[derive(Clone, Copy, Debug, Default)]
struct FBox {
    min: [f32; 3],
    max: [f32; 3],
    live: bool,
}

impl FBox {
    fn of(b: &VolumeBox) -> FBox {
        let c = b.center.map(cm);
        let h = b.half.map(|k| f32::from(k) / 100.0);
        FBox {
            min: [c[0] - h[0], c[1] - h[1], c[2] - h[2]],
            max: [c[0] + h[0], c[1] + h[1], c[2] + h[2]],
            live: b.live,
        }
    }

    /// `a` blended toward `b` by `f` (0 to 1), as the client's Interp draws it: centre a + (b − a) × f, a's size.
    fn blend(a: &VolumeBox, b: &VolumeBox, f: f32) -> FBox {
        let (ca, cb) = (a.center.map(cm), b.center.map(cm));
        let c = [
            ca[0] + (cb[0] - ca[0]) * f,
            ca[1] + (cb[1] - ca[1]) * f,
            ca[2] + (cb[2] - ca[2]) * f,
        ];
        let h = a.half.map(|k| f32::from(k) / 100.0);
        FBox {
            min: [c[0] - h[0], c[1] - h[1], c[2] - h[2]],
            max: [c[0] + h[0], c[1] + h[1], c[2] + h[2]],
            live: a.live,
        }
    }
}

/// Where the segment from `o` along `d` (its whole length) first enters the box, as a fraction 0 to 1; `None` if it
/// misses (the slab test; a segment starting inside enters at 0).
fn enters(o: [f32; 3], d: [f32; 3], min: [f32; 3], max: [f32; 3]) -> Option<f32> {
    let (mut lo, mut hi) = (0.0f32, 1.0f32);
    for a in 0..3 {
        if d[a] == 0.0 {
            if o[a] < min[a] || o[a] > max[a] {
                return None;
            }
        } else {
            let (t1, t2) = ((min[a] - o[a]) / d[a], (max[a] - o[a]) / d[a]);
            let (t1, t2) = if t1 < t2 { (t1, t2) } else { (t2, t1) };
            lo = fmax(lo, t1);
            hi = fmin(hi, t2);
            if lo > hi {
                return None;
            }
        }
    }
    Some(lo)
}

/// Whether a soldier at `p` overlaps the box past `eps` in x and y (and at all in z).
fn overlaps(p: [f32; 3], min: [f32; 3], max: [f32; 3], eps: f32) -> bool {
    p[0] + RADIUS > min[0] + eps
        && p[0] - RADIUS < max[0] - eps
        && p[1] + RADIUS > min[1] + eps
        && p[1] - RADIUS < max[1] - eps
        && p[2] + HEIGHT > min[2]
        && p[2] < max[2]
}

/// Whether `commands` is in the canonical order ([`TickInput::commands`]): slot ascending, then command tick strictly
/// ascending within a slot, wrap-aware.
pub(crate) fn canonical(commands: &[(Command, Option<Rewind>)]) -> bool {
    commands.windows(2).all(|w| {
        let (a, b) = (&w[0].0, &w[1].0);
        a.slot < b.slot || (a.slot == b.slot && (b.tick.wrapping_sub(a.tick) as i32) > 0)
    })
}

/// Whether the lent ring holds what a Rewind names: `tick_a`, and `tick_a + 1` when it blends toward it.
fn held(history: &HitHistory, r: &Rewind) -> bool {
    history.get(r.tick_a).is_some()
        && (r.fraction == 0 || history.get(r.tick_a.wrapping_add(1)).is_some())
}

#[derive(Clone, Copy, Debug)]
struct Soldier {
    alive: bool,
    /// Ticks until a dead soldier respawns.
    respawn_left: u32,
    generation: u16,
    health: u16,
    pos: [f32; 3],
    vel: [f32; 3],
    yaw: f32,
    pitch: f32,
    flags: u8,
    /// `Attribute::MoveSpeed`, cm/s.
    move_speed: u32,
    /// `Attribute::FireInterval`, µs.
    fire_interval: u32,
    ammo: u16,
    /// Weapon clock units until the next automatic shot (at most 0 is ready).
    wait: i64,
    /// Weapon clock units of reload left (0: not reloading).
    reload_left: i64,
}

/// One soldier's float fields before the own-copy snap: the records' `raw` (not state).
#[derive(Clone, Copy, Debug, Default)]
struct PreSnap {
    pos: [f32; 3],
    vel: [f32; 3],
    yaw: f32,
    pitch: f32,
}

impl PreSnap {
    fn of(s: &Soldier) -> PreSnap {
        PreSnap {
            pos: s.pos,
            vel: s.vel,
            yaw: s.yaw,
            pitch: s.pitch,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Effect {
    kind: u8,
    /// The tick it ends: at the step that runs this tick (or later, wrap-aware) it is rolled again.
    end: u32,
    magnitude: u8,
}

#[derive(Clone, Debug)]
struct Vehicle {
    /// Its loop round the map's centre: radius (m), period and phase (ticks).
    radius: f32,
    period: u32,
    phase: u32,
    /// Its parts' farthest reach from its centre, m (the collision's broad phase).
    reach: f32,
    pos: [f32; 3],
    yaw: f32,
    parts: core::ops::Range<usize>,
}

#[derive(Clone, Copy, Debug)]
struct Part {
    vehicle: u8,
    /// Where it sits on its hull, in the hull's frame (x and y before the hull's yaw, z above the hull's origin), in
    /// the position grid's centimetres: its record's relative transform (the advisor's W1-U4D H1). A rigid part's yaw
    /// relative to its hull is 0.
    offset_cm: [i32; 3],
    health: u16,
}

#[derive(Clone, Copy, Debug, Default)]
struct Projectile {
    live: bool,
    kind: u8,
    start_tick: u32,
    p0: [f32; 3],
    v0: [f32; 3],
    fuse: u16,
    owner: u8,
    seq: u16,
    generation: u16,
}

#[derive(Clone, Copy, Debug)]
struct Wall {
    health: u16,
    standing: bool,
    /// Destroyed this tick: it goes down at the next tick's movers, so this tick's moves saw it standing.
    falling: bool,
    down_left: u32,
}

/// The soldiers' collision movers, read from records' grid integers ([`World::load_movers`]). Sized once from the
/// scene and never grown: a record of an entity the world has not is skipped (the rust review's M1).
#[derive(Clone, Debug, Default)]
struct Movers {
    /// Each hull's last record integers: x, y, z and yaw.
    hulls: Vec<[i32; 4]>,
    /// Each part's last record integers: its x, y and z relative to its hull, then its hull's index.
    parts: Vec<[i32; 4]>,
    /// Each part's world box, m: its hull composed with it ([`part_center`]).
    boxes: Vec<([f32; 3], [f32; 3])>,
    /// Each hull's centre, m (x, y): the collision's broad phase.
    vehicles: Vec<[f32; 2]>,
    doors_closed: Vec<bool>,
    wall_standing: bool,
}

/// A shot this tick's commands fired, for the shot phase.
#[derive(Clone, Copy, Debug)]
struct Fired {
    slot: u8,
    seq: u16,
    origin: [f32; 3],
    point: [i32; 3],
    rewind: Option<Rewind>,
}

/// A field's value in a record: a float (raw bits, the wire grid's integer) or an integer.
#[derive(Clone, Copy)]
enum Val {
    F(f32),
    I(i32),
}

fn record(
    kind: EntityKind,
    index: u16,
    generation: u16,
    area: u16,
    values: &[Val],
) -> EntityRecord {
    let layout = kind.layout();
    debug_assert_eq!(
        layout.len(),
        values.len(),
        "{kind:?}'s record fills its layout"
    );
    let mut r = EntityRecord {
        kind,
        index,
        generation,
        area,
        grid: [0; EntityRecord::FIELDS],
        raw: [0; EntityRecord::FIELDS],
    };
    let wire = Config::default();
    for (i, (field, v)) in layout.iter().zip(values).enumerate() {
        match *v {
            Val::F(x) => {
                r.raw[i] = x.to_bits();
                r.grid[i] = wire.to_grid(*field, x);
            }
            Val::I(k) => {
                // An integer field holds every value the world can reach (the rust review's N4; W1-S1).
                debug_assert!(
                    (field.min()..=field.max()).contains(&k),
                    "{kind:?} field {i}: {k} outside {}..={}",
                    field.min(),
                    field.max()
                );
                r.grid[i] = k;
            }
        }
    }
    r
}

/// A part's world centre in the position grid's centimetres: its hull's record integers (`hull`: x, y, z and yaw)
/// composed with its own (`part`: x, y and z relative to the hull, in the hull's frame). Hull ∘ part's one home (the
/// advisor's W1-U4D H1 and A-M1): every world box of a part, the authority's collision and volumes, a client's movers
/// and AimCast's volumes, is this function of the records' grid integers.
fn part_center(hull: [i32; 4], part: [i32; 3]) -> [i32; 3] {
    let wire = Config::default();
    let yaw = wire.from_grid(YAW, hull[3]);
    let (c, s) = (libm::cosf(yaw), libm::sinf(yaw));
    let (ox, oy) = (cm(part[0]), cm(part[1]));
    [
        wire.to_grid(POSITION_XY, cm(hull[0]) + ox * c - oy * s),
        wire.to_grid(POSITION_XY, cm(hull[1]) + ox * s + oy * c),
        fit(POSITION_Z, hull[2].saturating_add(part[2])),
    ]
}

/// A part's box, m, around its world centre `c` (cm).
fn part_box(c: [i32; 3]) -> ([f32; 3], [f32; 3]) {
    let wire = Config::default();
    let at = [cm(c[0]), cm(c[1]), wire.from_grid(POSITION_Z, c[2])];
    let h = PART_HALF_CM as f32 / 100.0;
    (
        [at[0] - h, at[1] - h, at[2] - h],
        [at[0] + h, at[1] + h, at[2] + h],
    )
}

/// One command's move before collision: the move rule's one home (the advisor's U4d close ruling, B1.2 step 1), read
/// by [`World`]'s commands and by the synthetic generator's dead reckoning (`Recording::synthetic`, B1.2 step 3, with
/// `gain` 1). From the soldier's move speed (cm/s), the command's buttons and its move vector: crouching
/// halves the speed; sprinting takes it × 3 / 2, only when not crouching and not holding fire; a vector longer than 127
/// is scaled to 127. `gain` scales the speed (m/s) before the vector does: 1, but 1.01 under
/// PM-0001-41/server-speed-1pct's bite (the authority only, bites builds). Returns the velocity (m/s, x and y; one
/// tick's move is it times one tick), then whether the soldier crouches and whether it sprints (its record's flags).
pub(crate) fn move_velocity(
    move_speed: u32,
    buttons: u16,
    move_xy: [i8; 2],
    gain: f32,
) -> ([f32; 2], bool, bool) {
    let crouch = buttons & Command::CROUCH != 0;
    let fire_held = buttons & Command::FIRE != 0;
    let sprint = buttons & Command::SPRINT != 0 && !crouch && !fire_held;
    let speed_cm = if sprint {
        move_speed.saturating_mul(3) / 2
    } else if crouch {
        move_speed / 2
    } else {
        move_speed
    };
    let speed = speed_cm as f32 / 100.0 * gain;
    let (mx, my) = (i32::from(move_xy[0]), i32::from(move_xy[1]));
    let len2 = mx * mx + my * my;
    let scale = if len2 > 127 * 127 {
        127.0 / (len2 as f32).sqrt()
    } else {
        1.0
    };
    let v = [
        mx as f32 / 127.0 * scale * speed,
        my as f32 / 127.0 * scale * speed,
    ];
    (v, crouch, sprint)
}

/// The parts of one scene's world that `records` names, for [`compose_volumes`]: how many soldiers, hulls and parts.
#[derive(Clone, Copy)]
struct Counts {
    slots: usize,
    vehicles: usize,
    parts: usize,
}

/// The hit volumes of `records` into `into`, its storage reused ([`World::hit_volumes_from`]'s body): each soldier's
/// box from its record (feet on the grid, live unless its flags say dead, TC's plates thin), then the cover in the
/// world's order: each part composed with its hull's record ([`part_center`]), each door (live while its record says
/// closed), the wall stand-in (live while its record says standing). An entity with no record gives a box that is not
/// live; a record of an entity the world has not (an index past its count, a part naming a hull it has not, or whose
/// hull has no record) is skipped.
fn compose_volumes(
    scene: &Scene,
    greybox: &Greybox,
    n: Counts,
    records: &[EntityRecord],
    tick: u32,
    into: &mut HitVolumes,
) {
    into.tick = tick;
    into.soldiers.clear();
    into.soldiers.resize(n.slots, VolumeBox::default());
    into.cover.clear();
    into.cover.resize(n.parts, VolumeBox::default());
    for b in greybox.doors.iter().chain([&greybox.wall]) {
        into.cover.push(VolumeBox {
            center: [0, 1, 2].map(|a| (b.min_cm[a] + b.max_cm[a]) / 2),
            half: [0, 1, 2].map(|a| ((b.max_cm[a] - b.min_cm[a]) / 2) as u16),
            live: false,
        });
    }
    let doors = greybox.doors.len();
    let mut hulls = [None::<[i32; 4]>; MAX_VEHICLES];
    for r in records {
        let i = usize::from(r.index);
        let g = &r.grid;
        match r.kind {
            EntityKind::Soldier if i < n.slots => {
                let half = if scene.is_plate(r.index as u8) {
                    PLATE_HALF
                } else {
                    SOLDIER_HALF
                };
                into.soldiers[i] = VolumeBox {
                    center: [g[0], g[1], g[2].saturating_add(i32::from(half[2]))],
                    half,
                    live: g[SOLDIER_FLAGS] & i32::from(FLAG_DEAD) == 0,
                };
            }
            EntityKind::Vehicle if i < n.vehicles => hulls[i] = Some([g[0], g[1], g[2], g[3]]),
            EntityKind::Door if i < doors => into.cover[n.parts + i].live = g[DOOR_OPEN] == 0,
            EntityKind::Wall if i == 0 => into.cover[n.parts + doors].live = g[WALL_STANDING] == 1,
            _ => {}
        }
    }
    for r in records {
        let i = usize::from(r.index);
        if r.kind != EntityKind::VehiclePart || i >= n.parts {
            continue;
        }
        let g = &r.grid;
        let Some(Some(hull)) = usize::try_from(g[PART_PARENT])
            .ok()
            .and_then(|p| hulls.get(p))
        else {
            continue;
        };
        into.cover[i] = VolumeBox {
            center: part_center(*hull, [g[0], g[1], g[2]]),
            half: [PART_HALF_CM as u16; 3],
            live: true,
        };
    }
}

/// The planted defects `World` carries (bites builds only), each chosen per instance (u4-build-plan.md §4.11).
#[cfg(feature = "bites")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Bite {
    /// PM-0004-21's std `HashMap` iteration: at the arming tick a map of 64 keys is iterated once and the whole order
    /// folded into a state word.
    HashMap,
    /// PM-0004-21's std `sin`: the vehicles' loops take `f32::sin` in place of `libm::sinf` from the arming tick.
    StdSin,
    /// PM-0001-41/server-speed-1pct: the authority step's move speed × 1.01; `predict`'s is not (Q24).
    Speed,
    /// PM-0001-41/server-quant-differs: the authority step's own-copy snap on the grid `own_copy_snap_with_bite` gives.
    Quant,
}

#[cfg(feature = "bites")]
const BITE_IDS: [(&str, Bite); 4] = [
    ("PM-0004-21/hash-map", Bite::HashMap),
    ("PM-0004-21/std-sin", Bite::StdSin),
    ("PM-0001-41/server-speed-1pct", Bite::Speed),
    (crate::OWN_COPY_SNAP_BITES[0], Bite::Quant),
];

/// PM-0004-21/hash-map's planted iteration: a std `HashMap` of `keys` keys, iterated once, its whole order folded by
/// FNV-1a-64 (T5: a first-key outcome would repeat with p ≈ 1/32; the whole order of 64 keys does not).
#[cfg(feature = "bites")]
#[allow(
    clippy::disallowed_types,
    clippy::iter_over_hash_type,
    reason = "PM-0004-21/hash-map's planted defect (bites builds only): one std HashMap iteration whose order is the \
              process's random seed's"
)]
fn hash_map_order(keys: u32) -> u64 {
    let map: std::collections::HashMap<u32, u32> = (0..keys).map(|k| (k, k)).collect();
    let mut h = Fnv::new();
    for k in map.keys() {
        h.bytes(&k.to_le_bytes());
    }
    h.0
}

/// PM-0004-21/std-sin's planted call (bites builds only).
#[cfg(feature = "bites")]
#[allow(
    clippy::disallowed_methods,
    reason = "PM-0004-21/std-sin's planted defect (bites builds only): std's sin, which is the target's libm, not the \
              pinned libm crate"
)]
fn std_sin(x: f32) -> f32 {
    x.sin()
}

#[cfg(feature = "bites")]
#[derive(Clone, Debug)]
struct BiteState {
    bite: Bite,
    arm_tick: u32,
    armed: bool,
    /// The own-copy snap's grid under PM-0001-41/server-quant-differs.
    snap: Config,
    /// PM-0004-21/hash-map's folded order, once armed: a state word.
    fold: Option<u64>,
}

/// The stand-in sim (u4-build-plan.md §5 U4d): soldiers driven by commands, the stress scene's vehicles, projectiles,
/// effect rows, doors and wall, behind [`SimCore`]. One per room (the authority) and one per predicting client, built
/// from the same [`Scene`] and seed; its phases and rules are this module's.
///
/// **The clamp's bound** (the advisor's W1-U4D TUNE-2): for a fire origin at or below the position grid's top,
/// 40.95 m (a soldier's feet at or below 39.55 m), the clamp moves an aim point beyond the weapon's reach to the reach
/// (within one cell's half-diagonal) and leaves a clamped point sent again where it is, for every aim point inside the
/// grid's range, as every aim from the wire is. Above that bound a reach point can leave the grid's z range, saturates
/// there, and is not claimed to lie at the reach (the test review's probe A measured the second clamp first moving a
/// sent-back point for feet at about 40.57 m). No scene places a fire origin near it: every spawn point, plate and
/// greybox surface is at or below 10 m, which a test holds.
#[derive(Clone, Debug)]
pub struct World {
    scene: Scene,
    greybox: Greybox,
    /// Each team's free respawn points, cm.
    spawn_points: Vec<Vec<[i32; 3]>>,
    /// The tick the next step runs.
    next_tick: u32,
    /// Ticks run.
    elapsed: u64,
    soldiers: Vec<Soldier>,
    /// The last press counters each slot's commands claimed: outside the hash and the dump.
    claimed: Vec<[u8; 4]>,
    pre: Vec<PreSnap>,
    effects: Vec<Effect>,
    vehicles: Vec<Vehicle>,
    parts: Vec<Part>,
    projectiles: Vec<Projectile>,
    doors: Vec<bool>,
    wall: Wall,
    /// The estimate scene's spawn-rate accumulator, thousandths of a spawn × TICK_HZ: below 0 after grenades took
    /// spawns the stand-ins have not yet given up.
    spawn_acc: i64,
    rngs: [SimRng; 4],
    events: Vec<SimEvent>,
    reports: InputReports,
    records: Vec<EntityRecord>,
    volumes: HitVolumes,
    portals: PortalStates,
    movers: Movers,
    fired: Vec<Fired>,
    /// The blended volumes of the last Rewind a shot used, and that Rewind.
    blend_soldiers: Vec<FBox>,
    blend_cover: Vec<FBox>,
    blend_key: Option<(u32, u8, CoverVariant)>,
    #[cfg(feature = "bites")]
    bite: Option<BiteState>,
}

impl World {
    /// A world of `scene` from the host's `seed`, whose first step runs tick `first_tick`.
    pub fn new(scene: &Scene, seed: u64, first_tick: u32) -> World {
        let greybox = Greybox::dm_tri();
        let mut spawn_points = Vec::new();
        for home in scene.homes.iter().take(usize::from(scene.teams)) {
            let mut points = Vec::new();
            for i in -2..=2 {
                for j in -2..=2 {
                    let p = [home[0] + 150 * i, home[1] + 150 * j, 0];
                    let at = p.map(cm);
                    let free = greybox
                        .solids
                        .iter()
                        .chain(&greybox.doors)
                        .chain([&greybox.wall])
                        .all(|b| !overlaps(at, b.min, b.max, -0.1));
                    if free {
                        points.push(p);
                    }
                }
            }
            assert!(!points.is_empty(), "every team has a free spawn point");
            spawn_points.push(points);
        }
        // A scene's starting values are inside the ranges the own-state record's fields hold (W1-S1), and a part names
        // its hull in a 6-bit field.
        let w = scene.weapon;
        assert!(
            scene.move_speed <= MOVE_SPEED_MAX_CM_S
                && (1..=FIRE_INTERVAL_MAX_US).contains(&w.interval_us)
                && w.reload_ms <= RELOAD_MAX_MS
                && usize::from(scene.vehicles) <= MAX_VEHICLES,
            "a scene's soldiers start inside their own-state record's ranges"
        );
        let mut soldiers = Vec::new();
        let per_team = usize::from(scene.slots / scene.teams).max(1);
        for slot in 0..scene.slots {
            let at = if scene.is_plate(slot) {
                scene.plate_position(slot)
            } else {
                let team = usize::from(scene.team(slot).unwrap_or(0));
                let points = &spawn_points[team];
                points[(usize::from(slot) % per_team) % points.len()]
            };
            soldiers.push(Soldier {
                alive: true,
                respawn_left: 0,
                generation: 0,
                health: scene.health,
                pos: at.map(cm),
                vel: [0.0; 3],
                yaw: 0.0,
                pitch: 0.0,
                flags: 0,
                move_speed: scene.move_speed,
                fire_interval: scene.weapon.interval_us,
                ammo: scene.weapon.magazine,
                wait: 0,
                reload_left: 0,
            });
        }
        let mut vehicles = Vec::new();
        let mut parts = Vec::new();
        for v in 0..scene.vehicles {
            let n = if v < scene.large_vehicles {
                LARGE_PARTS
            } else {
                SMALL_PARTS
            };
            let first = parts.len();
            let mut reach = 0.0f32;
            for k in 0..n as i32 {
                // On a 1.2 m grid across the hull, in cm: 5 × 4 for a very large unit, 2 × 2 for the others.
                let offset_cm = if n == LARGE_PARTS {
                    [(k % 5) * 120 - 240, (k / 5) * 120 - 180, PART_Z_CM]
                } else {
                    [(k % 2) * 120 - 60, (k / 2) * 120 - 60, PART_Z_CM]
                };
                // Every relative offset fits its field (POSITION_Z has no negative values: H1).
                assert!(
                    [POSITION_XY, POSITION_XY, POSITION_Z]
                        .iter()
                        .zip(offset_cm)
                        .all(|(f, k)| fit(*f, k) == k),
                    "a part's offset fits its record's fields"
                );
                let [ox, oy] = [cm(offset_cm[0]), cm(offset_cm[1])];
                reach = fmax(reach, (ox * ox + oy * oy).sqrt() + 0.71);
                parts.push(Part {
                    vehicle: v,
                    offset_cm,
                    health: PART_HEALTH,
                });
            }
            let period = ticks_for_ms(60_000 + 5_000 * u32::from(v));
            // In the fight (the advisor's W1-U4D N-M4 and U4d close ruling B1.3; §4.13, F113): the scene's first
            // `fight_vehicles` hulls loop round the soldiers' cluster's centre, 5 to 14 m out (5 + 9 v / (n - 1) m for
            // n of them: SS's 4 very large units at 5, 8, 11 and 14 m, ES's 15 every 9 / 14 m), so their parts (within
            // 3.71 m of the hull) stay inside the cluster and its reach; the others loop elsewhere, 40 + 3 v m out
            // (SS's from 52 m), past the buildings.
            let fight = scene.fight_vehicles;
            let radius = if v < fight {
                5.0 + 9.0 * f32::from(v) / f32::from(fight.saturating_sub(1).max(1))
            } else {
                40.0 + 3.0 * f32::from(v)
            };
            vehicles.push(Vehicle {
                radius,
                period,
                phase: period / u32::from(scene.vehicles) * u32::from(v),
                reach,
                pos: [0.0; 3],
                yaw: 0.0,
                parts: first..parts.len(),
            });
        }
        assert!(
            soldiers.len() <= HitVolumes::MAX_SOLDIERS
                && parts.len() + greybox.doors.len() < HitVolumes::MAX_COVER,
            "a scene's volumes fit HitVolumes' maxima"
        );
        let doors: Vec<bool> = (0..greybox.doors.len()).map(|i| i % 2 == 0).collect();
        let slots = usize::from(scene.slots);
        let movers = Movers {
            hulls: vec![[0; 4]; vehicles.len()],
            parts: parts
                .iter()
                .map(|p| {
                    let [x, y, z] = p.offset_cm;
                    [x, y, z, i32::from(p.vehicle)]
                })
                .collect(),
            boxes: vec![([0.0; 3], [0.0; 3]); parts.len()],
            vehicles: vec![[0.0; 2]; vehicles.len()],
            doors_closed: vec![false; doors.len()],
            wall_standing: false,
        };
        let mut world = World {
            greybox,
            spawn_points,
            next_tick: first_tick,
            elapsed: 0,
            pre: soldiers.iter().map(PreSnap::of).collect(),
            soldiers,
            claimed: vec![[0; 4]; slots],
            // Each row ends at the first step: it is rolled there.
            effects: vec![
                Effect {
                    end: first_tick,
                    ..Effect::default()
                };
                slots * usize::from(scene.effect_rows)
            ],
            vehicles,
            parts,
            projectiles: vec![Projectile::default(); usize::from(scene.projectile_capacity)],
            portals: PortalStates {
                open: doors.clone(),
            },
            doors,
            wall: Wall {
                health: WALL_HEALTH,
                standing: true,
                falling: false,
                down_left: 0,
            },
            spawn_acc: 0,
            rngs: STREAMS.map(|s| SimRng::new(seed, s)),
            events: Vec::with_capacity(512),
            reports: InputReports::default(),
            records: Vec::new(),
            volumes: HitVolumes {
                tick: first_tick.wrapping_sub(1),
                soldiers: Vec::with_capacity(HitVolumes::MAX_SOLDIERS),
                cover: Vec::with_capacity(HitVolumes::MAX_COVER),
            },
            movers,
            fired: Vec::with_capacity(256),
            blend_soldiers: Vec::with_capacity(HitVolumes::MAX_SOLDIERS),
            blend_cover: Vec::with_capacity(HitVolumes::MAX_COVER),
            blend_key: None,
            scene: scene.clone(),
            #[cfg(feature = "bites")]
            bite: None,
        };
        // The state before the first tick: the movers where the first step puts them, and every read-out.
        world.place_vehicles();
        world.write_records();
        load_movers_into(&mut world.movers, &world.records);
        world.compose_own_volumes(first_tick.wrapping_sub(1));
        world
    }

    /// The counts [`compose_volumes`] reads this world's records against.
    fn counts(&self) -> Counts {
        Counts {
            slots: self.soldiers.len(),
            vehicles: self.vehicles.len(),
            parts: self.parts.len(),
        }
    }

    /// The authority's own volumes, from its own records (phase 3 and phase 8; [`World::hit_volumes_from`]).
    fn compose_own_volumes(&mut self, tick: u32) {
        let n = self.counts();
        compose_volumes(
            &self.scene,
            &self.greybox,
            n,
            &self.records,
            tick,
            &mut self.volumes,
        );
    }

    /// The hit volumes `records` describe, at `tick`, into `into` (its vectors' storage reused): the one function from
    /// records to [`HitVolumes`] (the advisor's W1-U4D A-M1). The authority's step builds its own volumes with it from
    /// its own records (phase 3 and phase 8), a client from the records it decoded, and AimCast from its interpolated
    /// ones, so the boxes' sizes and a part composed with its hull stay in npt-sim. Each soldier's box comes from its
    /// record (live unless the record says dead); the cover keeps this world's order, each vehicle part (its hull's
    /// record composed with its own), then each door (live while closed), then the wall stand-in (live while
    /// standing). An entity with no record in `records` gives a box that is not live; a record of an entity this world
    /// has not (an index past its count, or a part naming a hull it has not or whose hull has no record) is skipped.
    pub fn hit_volumes_from(&self, records: &[EntityRecord], tick: u32, into: &mut HitVolumes) {
        compose_volumes(
            &self.scene,
            &self.greybox,
            self.counts(),
            records,
            tick,
            into,
        );
    }

    /// The bite ids `World` plants (bites builds only): PM-0004-21's two injections, -41's speed bite, and the
    /// own-copy snap's ([`crate::OWN_COPY_SNAP_BITES`]).
    #[cfg(feature = "bites")]
    pub const BITES: [&'static str; 4] =
        [BITE_IDS[0].0, BITE_IDS[1].0, BITE_IDS[2].0, BITE_IDS[3].0];

    /// A world with one planted defect by its id (bites builds only), armed from the step that runs `arm_tick` on
    /// (at or after `first_tick`); `None` for an id it does not plant. PM-0001-41/server-quant-differs takes its
    /// own-copy snap's grid from [`crate::own_copy_snap_with_bite`] (carry 11), and the records keep the wire grid.
    #[cfg(feature = "bites")]
    pub fn with_bite(
        scene: &Scene,
        seed: u64,
        first_tick: u32,
        bite: &str,
        arm_tick: u32,
    ) -> Option<World> {
        let (_, b) = *BITE_IDS.iter().find(|(id, _)| *id == bite)?;
        let snap = if b == Bite::Quant {
            crate::own_copy_snap_with_bite(bite)?
        } else {
            Config::default()
        };
        let mut world = World::new(scene, seed, first_tick);
        world.bite = Some(BiteState {
            bite: b,
            arm_tick,
            armed: false,
            snap,
            fold: None,
        });
        Some(world)
    }

    /// PM-0004-21/hash-map's folded iteration order once armed (bites builds only): each process logs it, and two
    /// equal digests mean the bite was not exercised (u4-build-plan.md §4.14).
    #[cfg(feature = "bites")]
    pub fn bite_order_digest(&self) -> Option<u64> {
        self.bite.as_ref().and_then(|b| b.fold)
    }

    #[cfg(feature = "bites")]
    fn bitten(&self, bite: Bite) -> bool {
        self.bite
            .as_ref()
            .is_some_and(|b| b.armed && b.bite == bite)
    }

    /// The grid the authority's own-copy snap uses: the wire grid, or under PM-0001-41/server-quant-differs its own.
    fn own_snap(&self) -> Config {
        #[cfg(feature = "bites")]
        if let Some(b) = &self.bite
            && b.armed
            && b.bite == Bite::Quant
        {
            return b.snap;
        }
        Config::default()
    }

    fn sinf(&self, x: f32) -> f32 {
        #[cfg(feature = "bites")]
        if self.bitten(Bite::StdSin) {
            return std_sin(x);
        }
        libm::sinf(x)
    }

    /// The scene it runs.
    pub fn scene(&self) -> &Scene {
        &self.scene
    }

    /// The static world (its areas and portals are the room's static graph: u4-build-plan.md §4.2).
    pub fn greybox(&self) -> &Greybox {
        &self.greybox
    }

    /// The tick its next step runs.
    pub fn next_tick(&self) -> u32 {
        self.next_tick
    }

    /// Its state as the hash reads it ([`StateDump`]).
    pub fn dump(&self) -> StateDump {
        let mut d = StateDump::new();
        self.write_state(&mut d);
        d
    }

    /// The prediction entry point (ADR 0004:166-178; u4-build-plan.md §4.2): runs `command` for its soldier alone, as
    /// one tick, through the same phase functions as [`SimCore::step`] (the move, the own-copy snap on the wire grid,
    /// the aim and the weapon), against the movers [`World::load_movers`] last loaded. It draws from no random stream,
    /// resolves no shot, spawns nothing and counts nothing (a predicted throw is the authority's to accept), and
    /// refreshes that soldier's two records (its soldier record and its own-state record). Returns whether the command
    /// fired. A stand-in soldier has no vehicle, so the "one slot and its vehicle" of §4.2 is the slot alone. Two
    /// commands of one tick (a catch-up tick) are two calls, and predict the authority's tick bit for bit (D-3).
    pub fn predict(&mut self, command: &Command) -> bool {
        let slot = usize::from(command.slot);
        if slot >= self.soldiers.len() {
            return false;
        }
        let s = &mut self.soldiers[slot];
        s.vel = [0.0; 3];
        self.pre[slot] = PreSnap::of(s);
        let fired = self.run_command(command, None, false);
        for i in 0..self.records.len() {
            let r = &self.records[i];
            if usize::from(r.index) != slot {
                continue;
            }
            match r.kind {
                EntityKind::Soldier => self.records[i] = self.soldier_record(slot),
                EntityKind::OwnState => self.records[i] = self.own_state_record(slot),
                _ => {}
            }
        }
        fired
    }

    /// Loads the movers the soldiers collide with from records (a client: its last decoded snapshot's; the authority:
    /// its own, each tick): each hull's transform and each part's transform relative to its hull, each door open or
    /// closed and the wall standing or down, from the records' grid integers; each part's box is then its hull
    /// composed with it (the one composition: [`World::hit_volumes_from`]'s). A record of another kind, or one this
    /// world has no entity for (an index past the scene's count, a part naming a hull it has not), is skipped, and no
    /// mover is ever added from a record; a mover with no record keeps its last value. Its present hit volumes
    /// ([`SimCore::hit_volumes`]) then become [`World::hit_volumes_from`] of `records` (the advisor's W1-U4D A-M1: the
    /// function's caller on decoded records), labelled with this world's own tick counter, the tick it last stepped
    /// (carry 15).
    pub fn load_movers(&mut self, records: &[EntityRecord]) {
        load_movers_into(&mut self.movers, records);
        let (n, tick) = (self.counts(), self.next_tick.wrapping_sub(1));
        compose_volumes(
            &self.scene,
            &self.greybox,
            n,
            records,
            tick,
            &mut self.volumes,
        );
    }

    /// The first thing the segment `ray` meets: the static world from this world, the soldiers and cover from
    /// `movers` (a client's interpolated set for AimCast; for a server shot the blended, rewound set: ADR
    /// 0001:258-260, :358-362; A11). It passes through `ray.skip`'s box and every box that is not `live`; a ray with a
    /// non-finite end is a [`RayHit::Miss`]. It takes `&self` and changes nothing.
    pub fn ray_cast(&self, ray: &Ray, movers: &HitVolumes) -> RayHit {
        self.cast(
            ray,
            movers.soldiers.iter().map(FBox::of),
            movers.cover.iter().map(FBox::of),
        )
    }

    fn cast(
        &self,
        ray: &Ray,
        soldiers: impl Iterator<Item = FBox>,
        cover: impl Iterator<Item = FBox>,
    ) -> RayHit {
        if !ray.origin.iter().chain(&ray.to).all(|x| x.is_finite()) {
            return RayHit::Miss;
        }
        let d = sub(ray.to, ray.origin);
        let mut best = RayHit::Miss;
        let mut best_at = 2.0f32;
        for b in &self.greybox.solids {
            if let Some(at) = enters(ray.origin, d, b.min, b.max)
                && at < best_at
            {
                (best, best_at) = (RayHit::World { at }, at);
            }
        }
        for (i, b) in cover.enumerate() {
            if b.live
                && let Some(at) = enters(ray.origin, d, b.min, b.max)
                && at < best_at
            {
                (best, best_at) = (
                    RayHit::Cover {
                        index: i as u16,
                        at,
                    },
                    at,
                );
            }
        }
        for (i, b) in soldiers.enumerate() {
            if b.live
                && ray.skip != Some(i as u8)
                && let Some(at) = enters(ray.origin, d, b.min, b.max)
                && at < best_at
            {
                (best, best_at) = (RayHit::Soldier { slot: i as u8, at }, at);
            }
        }
        best
    }

    // Phase 0: the tick's spawns, teleports and attribute changes.
    fn apply_inputs(&mut self, input: &TickInput) {
        let slots = self.soldiers.len();
        for p in &input.spawns {
            let slot = usize::from(p.slot);
            if slot < slots && !self.scene.is_plate(p.slot) {
                let at = self.placement(p.position);
                let s = &mut self.soldiers[slot];
                if !s.alive {
                    s.generation = s.generation.wrapping_add(1);
                }
                self.revive(slot, at);
            }
        }
        for p in &input.teleports {
            let slot = usize::from(p.slot);
            if slot < slots && !self.scene.is_plate(p.slot) && self.soldiers[slot].alive {
                self.soldiers[slot].pos = self.placement(p.position);
            }
        }
        for a in &input.attributes {
            if let Some(s) = self.soldiers.get_mut(usize::from(a.slot)) {
                // Brought into the attribute's range ([`Attribute`]), which the own-state record's field holds.
                match a.attribute {
                    Attribute::MoveSpeed => s.move_speed = a.value.min(MOVE_SPEED_MAX_CM_S),
                    Attribute::FireInterval => {
                        s.fire_interval = a.value.clamp(1, FIRE_INTERVAL_MAX_US);
                    }
                }
            }
        }
    }

    /// A placement's point, brought into the grid's ranges, as metres.
    fn placement(&self, p: [i32; 3]) -> [f32; 3] {
        [
            cm(fit(POSITION_XY, p[0])),
            cm(fit(POSITION_XY, p[1])),
            cm(fit(POSITION_Z, p[2])),
        ]
    }

    /// Brings a soldier back alive at `at`, at full health and ammunition.
    fn revive(&mut self, slot: usize, at: [f32; 3]) {
        let s = &mut self.soldiers[slot];
        s.alive = true;
        s.respawn_left = 0;
        s.health = self.scene.health;
        s.pos = at;
        s.vel = [0.0; 3];
        s.flags = 0;
        s.ammo = self.scene.weapon.magazine;
        s.wait = 0;
        s.reload_left = 0;
        self.pre[slot] = PreSnap::of(s);
    }

    // Phase 1: the movers.
    fn run_movers(&mut self) {
        if self.wall.falling {
            self.wall.falling = false;
            self.wall.standing = false;
            self.wall.down_left = ticks_for_ms(WALL_DOWN_MS).max(1);
        } else if !self.wall.standing {
            self.wall.down_left -= 1;
            if self.wall.down_left == 0 {
                self.wall.standing = true;
                self.wall.health = WALL_HEALTH;
            }
        }
        // One portal change a second: door (second mod doors) toggles at each second's first tick.
        let hz = u64::from(TICK_HZ);
        if !self.doors.is_empty() && self.elapsed.is_multiple_of(hz) {
            let i = ((self.elapsed / hz) % self.doors.len() as u64) as usize;
            self.doors[i] = !self.doors[i];
        }
        self.portals.open.clone_from(&self.doors);
        self.place_vehicles();
        self.write_records();
        load_movers_into(&mut self.movers, &self.records);
    }

    /// Each vehicle on its loop at this tick (its parts ride on it: their records are relative to it).
    fn place_vehicles(&mut self) {
        for v in 0..self.vehicles.len() {
            let (radius, period, phase) = {
                let h = &self.vehicles[v];
                (h.radius, h.period, h.phase)
            };
            let along = ((self.elapsed + u64::from(phase)) % u64::from(period)) as f32;
            let theta = TAU * along / period as f32;
            let (c, s) = (libm::cosf(theta), self.sinf(theta));
            self.vehicles[v].pos = [radius * c, radius * s, 0.0];
            self.vehicles[v].yaw = theta + FRAC_PI_2;
        }
    }

    /// Where a soldier at `pos` moving by `delta` (m, x and y) ends: each axis in turn, stopped at the face of the
    /// first box in its way (the static world, closed doors, the standing wall, the vehicle parts); a box it starts
    /// inside does not block it. Also returns which axes were stopped.
    fn collide(&self, pos: [f32; 3], delta: [f32; 2]) -> ([f32; 3], [bool; 2]) {
        let mut p = pos;
        let mut blocked = [false; 2];
        for axis in 0..2 {
            if delta[axis] == 0.0 {
                continue;
            }
            let start = p;
            let mut q = p;
            q[axis] = p[axis] + delta[axis];
            let want = q;
            let mut test = |min: [f32; 3], max: [f32; 3]| {
                if !overlaps(start, min, max, INSIDE) && overlaps(q, min, max, SKIN) {
                    q[axis] = if delta[axis] > 0.0 {
                        min[axis] - RADIUS
                    } else {
                        max[axis] + RADIUS
                    };
                    blocked[axis] = true;
                }
            };
            for b in &self.greybox.solids {
                test(b.min, b.max);
            }
            for (b, closed) in self.greybox.doors.iter().zip(&self.movers.doors_closed) {
                if *closed {
                    test(b.min, b.max);
                }
            }
            if self.movers.wall_standing {
                let Aabb { min, max, .. } = self.greybox.wall;
                test(min, max);
            }
            let step = delta[axis].abs();
            for (v, centre) in self.vehicles.iter().zip(&self.movers.vehicles) {
                let (dx, dy) = (want[0] - centre[0], want[1] - centre[1]);
                let near = v.reach + RADIUS + step;
                if dx * dx + dy * dy > near * near {
                    continue;
                }
                for &(min, max) in self.movers.boxes.get(v.parts.clone()).unwrap_or(&[]) {
                    test(min, max);
                }
            }
            p = q;
        }
        (p, blocked)
    }

    /// The aim point a command's shot uses, from the fire origin `origin` (u4-build-plan.md §4.2, Q25): a point beyond
    /// the weapon's reach + 0.87 cm moves along its own ray to exactly the reach and is snapped to the wire grid,
    /// reported (by the authority) and never refused or counted; any other point is the command's own.
    fn clamp(&mut self, c: &Command, origin: [f32; 3], authority: bool) -> [i32; 3] {
        let reach = f64::from(self.scene.weapon.reach_cm) / 100.0;
        let o = origin.map(f64::from);
        let d = [
            f64::from(c.aim[0]) / 100.0 - o[0],
            f64::from(c.aim[1]) / 100.0 - o[1],
            f64::from(c.aim[2]) / 100.0 - o[2],
        ];
        let dist = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
        if dist <= reach + CLAMP_SLACK {
            return c.aim;
        }
        let k = reach / dist;
        let wire = Config::default();
        let point = [
            wire.to_grid(POSITION_XY, (o[0] + d[0] * k) as f32),
            wire.to_grid(POSITION_XY, (o[1] + d[1] * k) as f32),
            wire.to_grid(POSITION_Z, (o[2] + d[2] * k) as f32),
        ];
        if authority {
            self.reports.clamps.push(ClampReport {
                slot: c.slot,
                seq: c.seq,
                point,
            });
        }
        point
    }

    /// One command for its soldier, as one tick (phase 2; [`World::predict`]): the press edges, the move, the own-copy
    /// snap, the aim and the weapon. `authority` is the step: it counts refusals, reports clamps, fires its shots into
    /// the shot phase, throws, and (under -41's bites) moves and snaps otherwise. Returns whether it fired.
    fn run_command(&mut self, c: &Command, rewind: Option<Rewind>, authority: bool) -> bool {
        let slot = usize::from(c.slot);
        if slot >= self.soldiers.len() {
            return false;
        }
        // The press edges: a counter that changed is one press (the room allows one per button per tick); the counter
        // seen is the client's claim, kept outside the state.
        let mut edge = [false; 4];
        for (k, e) in edge.iter_mut().enumerate() {
            *e = c.presses[k] != self.claimed[slot][k];
        }
        self.claimed[slot] = c.presses;
        let mut s = self.soldiers[slot];
        if !s.alive || self.scene.is_plate(c.slot) {
            return false;
        }
        // The move: one tick at the soldier's speed, the vector widened before any arithmetic (carry 15).
        let fire_held = c.buttons & Command::FIRE != 0;
        #[allow(
            unused_mut,
            reason = "PM-0001-41/server-speed-1pct scales it in bites builds"
        )]
        let mut gain = 1.0;
        #[cfg(feature = "bites")]
        if authority && self.bitten(Bite::Speed) {
            gain = 1.01;
        }
        let (v, crouch, sprint) = move_velocity(s.move_speed, c.buttons, c.move_xy, gain);
        let (pos, blocked) = self.collide(s.pos, [v[0] * DT, v[1] * DT]);
        let vel = [
            if blocked[0] { 0.0 } else { v[0] },
            if blocked[1] { 0.0 } else { v[1] },
            0.0,
        ];
        // The own-copy snap (Q23): the wire grid, or the authority's under PM-0001-41/server-quant-differs.
        let snap = if authority {
            self.own_snap()
        } else {
            Config::default()
        };
        let pos_snapped = [
            snap.snap(POSITION_XY, pos[0]),
            snap.snap(POSITION_XY, pos[1]),
            snap.snap(POSITION_Z, pos[2]),
        ];
        // The aim, from the in-step fire origin.
        let origin = [pos_snapped[0], pos_snapped[1], pos_snapped[2] + CHEST];
        let point = self.clamp(c, origin, authority);
        let d = [
            (f64::from(point[0]) / 100.0 - f64::from(origin[0])) as f32,
            (f64::from(point[1]) / 100.0 - f64::from(origin[1])) as f32,
            (f64::from(point[2]) / 100.0 - f64::from(origin[2])) as f32,
        ];
        let yaw = libm::atan2f(d[1], d[0]);
        let pitch = libm::atan2f(d[2], (d[0] * d[0] + d[1] * d[1]).sqrt());
        self.pre[slot] = PreSnap {
            pos,
            vel,
            yaw,
            pitch,
        };
        s.pos = pos_snapped;
        s.vel = vel.map(|x| snap.snap(VELOCITY, x));
        s.yaw = snap.snap(YAW, yaw);
        s.pitch = snap.snap(PITCH, pitch);
        s.flags &= !(FLAG_CROUCH | FLAG_SPRINT | FLAG_RELOADING);
        if crouch {
            s.flags |= FLAG_CROUCH;
        }
        if sprint {
            s.flags |= FLAG_SPRINT;
        }
        // The weapon: its clock advances one tick (carrying at most one tick's remainder, so a held trigger fires at
        // exactly one shot per interval and an idle weapon bursts no faster), then the reload, then the trigger.
        let w = self.scene.weapon;
        let cost = i64::from(s.fire_interval.max(1)) * i64::from(TICK_HZ);
        s.wait = (s.wait - UNITS_PER_TICK).max(1 - UNITS_PER_TICK);
        if s.reload_left > 0 {
            s.reload_left -= UNITS_PER_TICK;
            if s.reload_left <= 0 {
                s.reload_left = 0;
                s.ammo = w.magazine;
            }
        }
        if edge[Command::PRESS_RELOAD] {
            if s.reload_left > 0 || s.ammo >= w.magazine {
                if authority {
                    self.reports.reload += 1;
                    self.refuse(c, RefusalKind::Reload);
                }
            } else {
                s.reload_left = i64::from(w.reload_ms) * 1_000 * i64::from(TICK_HZ);
            }
        }
        let ready = s.reload_left == 0 && s.ammo > 0 && s.wait <= 0;
        if edge[Command::PRESS_FIRE] && !ready && authority {
            if s.ammo == 0 {
                self.reports.ammunition += 1;
                self.refuse(c, RefusalKind::Ammunition);
            } else {
                self.reports.cooldown += 1;
                self.refuse(c, RefusalKind::Cooldown);
            }
        }
        let fired = ready && (fire_held || edge[Command::PRESS_FIRE]);
        if fired {
            s.wait += cost;
            s.ammo -= 1;
            if authority {
                self.fired.push(Fired {
                    slot: c.slot,
                    seq: c.seq,
                    origin,
                    point,
                    rewind,
                });
            }
        }
        if s.reload_left > 0 {
            s.flags |= FLAG_RELOADING;
        }
        self.soldiers[slot] = s;
        if edge[Command::PRESS_THROW] && authority {
            self.throw(c, origin, d);
        }
        fired
    }

    /// One refused part of `c`, for the tick's [`InputReports::refusals`] (the authority's only).
    fn refuse(&mut self, c: &Command, kind: RefusalKind) {
        self.reports.refusals.push(Refusal {
            slot: c.slot,
            seq: c.seq,
            kind,
        });
    }

    /// A grenade from `origin` toward the aim (`d` from the origin to it), on the grid, or a refusal when the pool is
    /// full. In a scene that spawns its stand-ins at a rate (ES), a grenade is one of the rate's spawns: it takes one
    /// from the spawner, which fills the rest (the advisor's W1-U4D T-M1).
    fn throw(&mut self, c: &Command, origin: [f32; 3], d: [f32; 3]) {
        let Some(i) = self.projectiles.iter().position(|p| !p.live) else {
            self.reports.pool_full += 1;
            self.refuse(c, RefusalKind::PoolFull);
            return;
        };
        if self.scene.spawn_millihertz > 0 {
            self.spawn_acc -= 1_000 * i64::from(TICK_HZ);
        }
        let flat = (d[0] * d[0] + d[1] * d[1]).sqrt();
        let (ux, uy) = if flat > 0.0 {
            (d[0] / flat, d[1] / flat)
        } else {
            (1.0, 0.0)
        };
        let wire = Config::default();
        let p = &mut self.projectiles[i];
        *p = Projectile {
            live: true,
            kind: GRENADE,
            start_tick: self.next_tick,
            p0: [
                wire.snap(POSITION_XY, origin[0]),
                wire.snap(POSITION_XY, origin[1]),
                wire.snap(POSITION_Z, origin[2]),
            ],
            v0: [
                wire.snap(VELOCITY, ux * THROW_SPEED),
                wire.snap(VELOCITY, uy * THROW_SPEED),
                wire.snap(VELOCITY, THROW_LIFT),
            ],
            fuse: ticks_for_ms(self.scene.grenade_fuse_ms) as u16,
            owner: c.slot,
            seq: c.seq,
            generation: p.generation.wrapping_add(1),
        };
    }

    // Phase 3: the present volumes, after this tick's moves: the soldiers' records refreshed (the first `slots` records
    // are theirs, in slot order), then the volumes of the records (the cover's are phase 1's: nothing moved them since).
    fn present_volumes(&mut self, tick: u32) {
        for slot in 0..self.soldiers.len() {
            self.records[slot] = self.soldier_record(slot);
        }
        self.compose_own_volumes(tick);
    }

    /// The blended volumes of `r` (accepted: the ring holds what it names) in `blend_soldiers` and `blend_cover`, kept
    /// while the next shot names the same Rewind.
    fn blend(&mut self, history: &HitHistory, r: &Rewind) {
        let key = (r.tick_a, r.fraction, r.cover);
        if self.blend_key == Some(key) {
            return;
        }
        let Some(a) = history.get(r.tick_a) else {
            return;
        };
        let b = if r.fraction > 0 {
            history.get(r.tick_a.wrapping_add(1)).unwrap_or(a)
        } else {
            a
        };
        let f = f32::from(r.fraction) / 256.0;
        self.blend_soldiers.clear();
        for (i, va) in a.soldiers.iter().enumerate() {
            let vb = b.soldiers.get(i).unwrap_or(va);
            let mut fb = FBox::blend(va, vb, f);
            // A soldier dead now takes no shot, whatever it was then.
            fb.live &= self.soldiers.get(i).is_some_and(|s| s.alive);
            self.blend_soldiers.push(fb);
        }
        self.blend_cover.clear();
        match r.cover {
            CoverVariant::Rewound => {
                for (i, va) in a.cover.iter().enumerate() {
                    let vb = b.cover.get(i).unwrap_or(va);
                    self.blend_cover.push(FBox::blend(va, vb, f));
                }
            }
            CoverVariant::Present => {
                self.blend_cover
                    .extend(self.volumes.cover.iter().map(FBox::of));
            }
        }
        self.blend_key = Some(key);
    }

    // Phase 4: the shots.
    fn run_shots(&mut self, history: &HitHistory) {
        let w = self.scene.weapon;
        let reach = w.reach_cm as f32 / 100.0;
        for i in 0..self.fired.len() {
            let f = self.fired[i];
            self.events.push(SimEvent::Shot {
                slot: f.slot,
                seq: f.seq,
            });
            // The spread, from the authority-only stream: up to spread_urad × the distance, per axis.
            let target = f.point.map(|k| (f64::from(k) / 100.0) as f32);
            let width =
                w.spread_urad as f32 / 1_000_000.0 * length(sub(target, f.origin)) / 1_000.0;
            let r = &mut self.rngs[Stream::Spread as usize - 1];
            let off = [0; 3].map(|_| r.between(-1_000, 1_000) as f32 * width);
            let d = sub(
                [target[0] + off[0], target[1] + off[1], target[2] + off[2]],
                f.origin,
            );
            let dist = length(d);
            if dist < 0.01 {
                continue;
            }
            let k = reach / dist;
            let ray = Ray {
                origin: f.origin,
                to: [
                    f.origin[0] + d[0] * k,
                    f.origin[1] + d[1] * k,
                    f.origin[2] + d[2] * k,
                ],
                skip: Some(f.slot),
            };
            let hit = match &f.rewind {
                Some(r) => {
                    self.blend(history, r);
                    self.cast(
                        &ray,
                        self.blend_soldiers.iter().copied(),
                        self.blend_cover.iter().copied(),
                    )
                }
                None => self.ray_cast(&ray, &self.volumes),
            };
            self.land(f.slot, f.seq, hit, w.damage);
        }
    }

    /// What a shot that met `hit` does.
    fn land(&mut self, shooter: u8, seq: u16, hit: RayHit, damage: u16) {
        match hit {
            RayHit::Soldier { slot, .. } => self.damage(shooter, seq, slot, damage),
            RayHit::Cover { index, .. } => {
                let i = usize::from(index);
                if let Some(p) = self.parts.get_mut(i) {
                    // A part worn out is mended at once (a stand-in: its health keeps changing under fire).
                    p.health = p.health.checked_sub(damage).unwrap_or(PART_HEALTH);
                } else if i == self.parts.len() + self.doors.len() && self.wall.standing {
                    self.wall.health = self.wall.health.saturating_sub(damage);
                    if self.wall.health == 0 {
                        self.wall.falling = true;
                    }
                }
            }
            RayHit::Miss | RayHit::World { .. } => {}
        }
    }

    /// `victim` takes `damage` from `shooter`'s command `seq`: a Hit, and a Kill when its health runs out. TC's plates
    /// take the hit and keep their health.
    fn damage(&mut self, shooter: u8, seq: u16, victim: u8, damage: u16) {
        let v = usize::from(victim);
        if !self.soldiers[v].alive {
            return;
        }
        self.events.push(SimEvent::Hit {
            shooter,
            seq,
            victim,
            damage,
        });
        if self.scene.is_plate(victim) {
            return;
        }
        let s = &mut self.soldiers[v];
        s.health = s.health.saturating_sub(damage);
        if s.health == 0 {
            s.alive = false;
            s.respawn_left = ticks_for_ms(self.scene.respawn_ms).max(1);
            s.flags = FLAG_DEAD;
            s.vel = [0.0; 3];
            // Its record shows the stop too, so the record's grid stays the wire grid of the snapped copy (carry 9).
            self.pre[v].vel = [0.0; 3];
            self.events.push(SimEvent::Kill {
                killer: shooter,
                victim,
            });
            self.volumes.soldiers[v].live = false;
            self.blend_key = None;
        }
    }

    // Phase 5: the projectiles.
    fn run_projectiles(&mut self, tick: u32) {
        for i in 0..self.projectiles.len() {
            let p = self.projectiles[i];
            if p.live && tick.wrapping_sub(p.start_tick) >= u32::from(p.fuse) {
                self.projectiles[i].live = false;
                if p.kind == GRENADE {
                    let at = trajectory_at(p.p0, p.v0, u32::from(p.fuse));
                    for v in 0..self.soldiers.len() {
                        let s = &self.soldiers[v];
                        let d = sub([s.pos[0], s.pos[1], s.pos[2] + CHEST], at);
                        if s.alive && length(d) <= BLAST_RADIUS {
                            self.damage(p.owner, p.seq, v as u8, BLAST_DAMAGE);
                        }
                    }
                }
            }
        }
        let live = self
            .projectiles
            .iter()
            .filter(|p| p.live && p.kind == STAND_IN)
            .count();
        for _ in live..usize::from(self.scene.live_projectiles) {
            self.spawn_stand_in(tick);
        }
        if self.scene.spawn_millihertz > 0 {
            // The scene's rate of spawns, grenades included: a grenade thrown took its spawn from the accumulator
            // (`throw`), so the stand-ins fill the rest (T-M1).
            let whole = 1_000 * i64::from(TICK_HZ);
            self.spawn_acc += i64::from(self.scene.spawn_millihertz);
            while self.spawn_acc >= whole {
                self.spawn_acc -= whole;
                self.spawn_stand_in(tick);
            }
        }
    }

    /// One stand-in projectile, its start and fuse drawn from the projectiles' stream, on the grid; none if the pool
    /// is full (the scene keeps room for the stand-ins it asks for). It starts inside the fight (the advisor's W1-U4D
    /// N-M4: "300 live projectiles in the fight"): within 12 m of the cluster's centre in x and y, up to 2 m up, at up
    /// to 3 m/s across and 2 to 12 m/s up, so it lands within about 8 m of its start (at most 2.6 s in the air).
    fn spawn_stand_in(&mut self, tick: u32) {
        let Some(i) = self.projectiles.iter().position(|p| !p.live) else {
            return;
        };
        let [lo, hi] = self.scene.fuse_ms;
        let r = &mut self.rngs[Stream::Projectiles as usize - 1];
        let p0 = [
            r.between(-1_200, 1_200),
            r.between(-1_200, 1_200),
            r.between(0, 200),
        ];
        let v0 = [
            r.between(-300, 300),
            r.between(-300, 300),
            r.between(200, 1_200),
        ];
        let fuse = ticks_for_ms(r.between(lo as i32, hi as i32) as u32);
        let wire = Config::default();
        let p = &mut self.projectiles[i];
        *p = Projectile {
            live: true,
            kind: STAND_IN,
            start_tick: tick,
            p0: [cm(p0[0]), cm(p0[1]), wire.from_grid(POSITION_Z, p0[2])],
            v0: v0.map(|k| wire.from_grid(VELOCITY, k)),
            fuse: fuse.min(u32::from(u16::MAX)) as u16,
            owner: 0,
            seq: 0,
            generation: p.generation.wrapping_add(1),
        };
    }

    // Phase 6: S1's effect rows: each row's end checked every tick (S1's per-row work), a row at or past its end rolled
    // again from the effects' stream, to end 0.5 to 3 s on. A row's record changes only when it is rolled (N-M5).
    fn run_effects(&mut self, tick: u32) {
        let r = &mut self.rngs[Stream::Effects as usize - 1];
        for e in &mut self.effects {
            if tick.wrapping_sub(e.end) as i32 >= 0 {
                e.kind = r.below(8) as u8;
                e.end = tick.wrapping_add(ticks_for_ms(r.between(500, 3_000) as u32));
                e.magnitude = r.below(256) as u8;
            }
        }
    }

    // Phase 7: respawns, at a free point of the soldier's team drawn from the respawn stream.
    fn run_respawns(&mut self) {
        for slot in 0..self.soldiers.len() {
            let s = &mut self.soldiers[slot];
            if s.alive {
                continue;
            }
            s.respawn_left = s.respawn_left.saturating_sub(1);
            if s.respawn_left > 0 {
                continue;
            }
            s.generation = s.generation.wrapping_add(1);
            let team = usize::from(self.scene.team(slot as u8).unwrap_or(0));
            let points = &self.spawn_points[team];
            let r = &mut self.rngs[Stream::Respawn as usize - 1];
            let at = points[r.below(points.len() as u32) as usize].map(cm);
            self.revive(slot, at);
        }
    }

    /// The area of a soldier's feet.
    fn soldier_area(&self, slot: usize) -> u16 {
        let s = &self.soldiers[slot];
        let wire = Config::default();
        self.greybox.area_of(
            wire.to_grid(POSITION_XY, s.pos[0]),
            wire.to_grid(POSITION_XY, s.pos[1]),
        )
    }

    /// Slot `slot`'s soldier record: the catalogue's remote row (W1-S1).
    fn soldier_record(&self, slot: usize) -> EntityRecord {
        let s = &self.soldiers[slot];
        let pre = &self.pre[slot];
        record(
            EntityKind::Soldier,
            slot as u16,
            s.generation,
            self.soldier_area(slot),
            &[
                Val::F(pre.pos[0]),
                Val::F(pre.pos[1]),
                Val::F(pre.pos[2]),
                Val::F(pre.yaw),
                Val::F(pre.pitch),
                Val::I(i32::from(s.flags)),
                Val::I(i32::from(s.health)),
                Val::I(0),
            ],
        )
    }

    /// Slot `slot`'s own-state record (owner only: W1-S1): the velocity before the own-copy snap (its `raw`, the wire
    /// grid's integers of it in `grid`), then the weapon's ammunition, fire timer and reload left, and the move-speed
    /// and fire-interval attributes, each exactly as the state holds it.
    fn own_state_record(&self, slot: usize) -> EntityRecord {
        let s = &self.soldiers[slot];
        let pre = &self.pre[slot];
        record(
            EntityKind::OwnState,
            slot as u16,
            s.generation,
            self.soldier_area(slot),
            &[
                Val::F(pre.vel[0]),
                Val::F(pre.vel[1]),
                Val::F(pre.vel[2]),
                Val::I(i32::from(s.ammo)),
                Val::I(s.wait as i32),
                Val::I(s.reload_left as i32),
                Val::I(s.move_speed as i32),
                Val::I(s.fire_interval as i32),
            ],
        )
    }

    /// Every entity's record, in kind order ([`EntityKind::ALL`]'s): soldiers, vehicles, parts, live projectiles,
    /// doors, the wall, effect rows, the soldiers' own states.
    fn write_records(&mut self) {
        let wire = Config::default();
        let mut records = core::mem::take(&mut self.records);
        records.clear();
        for slot in 0..self.soldiers.len() {
            records.push(self.soldier_record(slot));
        }
        let area = |p: [f32; 3]| {
            self.greybox.area_of(
                wire.to_grid(POSITION_XY, p[0]),
                wire.to_grid(POSITION_XY, p[1]),
            )
        };
        for (i, v) in self.vehicles.iter().enumerate() {
            records.push(record(
                EntityKind::Vehicle,
                i as u16,
                0,
                area(v.pos),
                &[
                    Val::F(v.pos[0]),
                    Val::F(v.pos[1]),
                    Val::F(v.pos[2]),
                    Val::F(v.yaw),
                ],
            ));
        }
        // A part's record: its transform relative to its hull (on the cm grid, so `raw` is exactly its grid integer's
        // value) and its hull's area, since it travels with its hull (H1).
        for (i, p) in self.parts.iter().enumerate() {
            let hull = &self.vehicles[usize::from(p.vehicle)];
            records.push(record(
                EntityKind::VehiclePart,
                i as u16,
                0,
                area(hull.pos),
                &[
                    Val::F(cm(p.offset_cm[0])),
                    Val::F(cm(p.offset_cm[1])),
                    Val::F(wire.from_grid(POSITION_Z, p.offset_cm[2])),
                    Val::F(0.0),
                    Val::I(i32::from(p.health)),
                    Val::I(i32::from(p.vehicle)),
                ],
            ));
        }
        for (i, p) in self.projectiles.iter().enumerate() {
            if !p.live {
                continue;
            }
            let now = trajectory_at(p.p0, p.v0, self.next_tick.wrapping_sub(p.start_tick));
            records.push(record(
                EntityKind::Projectile,
                i as u16,
                p.generation,
                area(now),
                &[
                    Val::I(p.start_tick as i32),
                    Val::F(p.p0[0]),
                    Val::F(p.p0[1]),
                    Val::F(p.p0[2]),
                    Val::F(p.v0[0]),
                    Val::F(p.v0[1]),
                    Val::F(p.v0[2]),
                    Val::I(i32::from(p.kind)),
                    Val::I(i32::from(p.fuse)),
                ],
            ));
        }
        for (i, open) in self.doors.iter().enumerate() {
            records.push(record(
                EntityKind::Door,
                i as u16,
                0,
                i as u16 + 1,
                &[Val::I(i32::from(*open))],
            ));
        }
        let wall = self.greybox.wall;
        records.push(record(
            EntityKind::Wall,
            0,
            0,
            self.greybox.area_of(wall.min_cm[0], wall.min_cm[1]),
            &[
                Val::I(i32::from(self.wall.health)),
                Val::I(i32::from(self.wall.standing)),
            ],
        ));
        let rows = usize::from(self.scene.effect_rows).max(1);
        for (i, e) in self.effects.iter().enumerate() {
            let owner = &self.soldiers[i / rows];
            records.push(record(
                EntityKind::Effect,
                i as u16,
                owner.generation,
                area(owner.pos),
                &[
                    Val::I(i32::from(e.kind)),
                    Val::I(e.end as i32),
                    Val::I(i32::from(e.magnitude)),
                ],
            ));
        }
        for slot in 0..self.soldiers.len() {
            records.push(self.own_state_record(slot));
        }
        self.records = records;
    }

    /// The state, as the hash reads it: every field that decides a later tick, in a fixed order, as 32-bit words.
    /// Left out: the press counters of the last command run per slot (input bookkeeping, a function of the command
    /// stream alone), the reports, what each tick rebuilds from the state (the records, the volumes, the movers, the
    /// blend), and what the scene fixes (a part's place on its hull).
    fn write_state(&self, s: &mut impl Sink) {
        s.word("world", 0, "next_tick", self.next_tick);
        s.wide("world", 0, "elapsed", self.elapsed);
        for (i, x) in self.soldiers.iter().enumerate() {
            let i = i as u32;
            s.word("soldier", i, "alive", u32::from(x.alive));
            s.word("soldier", i, "respawn_left", x.respawn_left);
            s.word("soldier", i, "generation", u32::from(x.generation));
            s.word("soldier", i, "health", u32::from(x.health));
            s.float("soldier", i, "pos.x", x.pos[0]);
            s.float("soldier", i, "pos.y", x.pos[1]);
            s.float("soldier", i, "pos.z", x.pos[2]);
            s.float("soldier", i, "vel.x", x.vel[0]);
            s.float("soldier", i, "vel.y", x.vel[1]);
            s.float("soldier", i, "vel.z", x.vel[2]);
            s.float("soldier", i, "yaw", x.yaw);
            s.float("soldier", i, "pitch", x.pitch);
            s.word("soldier", i, "flags", u32::from(x.flags));
            s.word("soldier", i, "move_speed", x.move_speed);
            s.word("soldier", i, "fire_interval", x.fire_interval);
            s.word("soldier", i, "ammo", u32::from(x.ammo));
            s.wide("soldier", i, "wait", x.wait as u64);
            s.wide("soldier", i, "reload_left", x.reload_left as u64);
        }
        for (i, e) in self.effects.iter().enumerate() {
            let i = i as u32;
            s.word("effect", i, "kind", u32::from(e.kind));
            s.word("effect", i, "end", e.end);
            s.word("effect", i, "magnitude", u32::from(e.magnitude));
        }
        for (i, v) in self.vehicles.iter().enumerate() {
            let i = i as u32;
            s.float("vehicle", i, "pos.x", v.pos[0]);
            s.float("vehicle", i, "pos.y", v.pos[1]);
            s.float("vehicle", i, "pos.z", v.pos[2]);
            s.float("vehicle", i, "yaw", v.yaw);
        }
        for (i, p) in self.parts.iter().enumerate() {
            s.word("part", i as u32, "health", u32::from(p.health));
        }
        for (i, p) in self.projectiles.iter().enumerate() {
            let i = i as u32;
            s.word("projectile", i, "live", u32::from(p.live));
            s.word("projectile", i, "kind", u32::from(p.kind));
            s.word("projectile", i, "start_tick", p.start_tick);
            s.float("projectile", i, "p0.x", p.p0[0]);
            s.float("projectile", i, "p0.y", p.p0[1]);
            s.float("projectile", i, "p0.z", p.p0[2]);
            s.float("projectile", i, "v0.x", p.v0[0]);
            s.float("projectile", i, "v0.y", p.v0[1]);
            s.float("projectile", i, "v0.z", p.v0[2]);
            s.word("projectile", i, "fuse", u32::from(p.fuse));
            s.word("projectile", i, "owner", u32::from(p.owner));
            s.word("projectile", i, "seq", u32::from(p.seq));
            s.word("projectile", i, "generation", u32::from(p.generation));
        }
        for (i, open) in self.doors.iter().enumerate() {
            s.word("door", i as u32, "open", u32::from(*open));
        }
        s.word("wall", 0, "health", u32::from(self.wall.health));
        s.word("wall", 0, "standing", u32::from(self.wall.standing));
        s.word("wall", 0, "falling", u32::from(self.wall.falling));
        s.word("wall", 0, "down_left", self.wall.down_left);
        s.wide("spawner", 0, "acc", self.spawn_acc as u64);
        for (i, r) in self.rngs.iter().enumerate() {
            let at = r.position();
            s.wide("rng", i as u32, "position.lo", at as u64);
            s.wide("rng", i as u32, "position.hi", (at >> 64) as u64);
        }
        s.word("events", 0, "count", self.events.len() as u32);
        for (i, e) in self.events.iter().enumerate() {
            let i = i as u32;
            let (kind, a, b, c) = match *e {
                SimEvent::Shot { slot, seq } => (1, u32::from(slot), u32::from(seq), 0),
                SimEvent::Hit {
                    shooter,
                    seq,
                    victim,
                    damage,
                } => (
                    2,
                    u32::from(shooter) | u32::from(victim) << 8,
                    u32::from(seq),
                    u32::from(damage),
                ),
                SimEvent::Kill { killer, victim } => (3, u32::from(killer), u32::from(victim), 0),
            };
            s.word("event", i, "kind", kind);
            s.word("event", i, "a", a);
            s.word("event", i, "b", b);
            s.word("event", i, "c", c);
        }
        #[cfg(feature = "bites")]
        if let Some(fold) = self.bite.as_ref().and_then(|b| b.fold) {
            s.wide("bite", 0, "hash_map_order", fold);
        }
    }

    /// The first non-finite float of the state, by name (D-4), and then of each soldier's values before the own-copy
    /// snap (the records' `raw`), where a NaN the snap would turn into 0 is caught at the tick it arose (the test
    /// review's L1).
    fn non_finite(&self) -> Option<String> {
        let mut f = Finite(None);
        self.write_state(&mut f);
        for (i, p) in self.pre.iter().enumerate() {
            let i = i as u32;
            let values = [
                ("pos.x", p.pos[0]),
                ("pos.y", p.pos[1]),
                ("pos.z", p.pos[2]),
                ("vel.x", p.vel[0]),
                ("vel.y", p.vel[1]),
                ("vel.z", p.vel[2]),
                ("yaw", p.yaw),
                ("pitch", p.pitch),
            ];
            for (name, x) in values {
                f.float("pre", i, name, x);
            }
        }
        f.0
    }
}

/// Reads the movers from records' grid integers ([`World::load_movers`]): each record of an entity the movers hold
/// updates that entity's integers, any other is skipped and nothing is added (the movers are sized from the scene at
/// `World::new`); then each part's box is its hull composed with it ([`part_center`]).
fn load_movers_into(movers: &mut Movers, records: &[EntityRecord]) {
    let vehicles = movers.hulls.len();
    for r in records {
        let i = usize::from(r.index);
        let g = &r.grid;
        match r.kind {
            EntityKind::Vehicle if i < vehicles => movers.hulls[i] = [g[0], g[1], g[2], g[3]],
            EntityKind::VehiclePart
                if i < movers.parts.len()
                    && usize::try_from(g[PART_PARENT]).is_ok_and(|p| p < vehicles) =>
            {
                movers.parts[i] = [g[0], g[1], g[2], g[PART_PARENT]];
            }
            EntityKind::Door if i < movers.doors_closed.len() => {
                movers.doors_closed[i] = g[DOOR_OPEN] == 0;
            }
            EntityKind::Wall if i == 0 => movers.wall_standing = g[WALL_STANDING] == 1,
            _ => {}
        }
    }
    for (v, h) in movers.vehicles.iter_mut().zip(&movers.hulls) {
        *v = [cm(h[0]), cm(h[1])];
    }
    for (b, p) in movers.boxes.iter_mut().zip(&movers.parts) {
        let hull = movers.hulls[p[3] as usize];
        *b = part_box(part_center(hull, [p[0], p[1], p[2]]));
    }
}

impl SimCore for World {
    fn step(&mut self, input: &TickInput, history: &HitHistory) -> &[SimEvent] {
        debug_assert!(
            canonical(&input.commands),
            "TickInput::commands is in the canonical order: slot ascending, then command tick strictly ascending \
             within a slot, wrap-aware"
        );
        let tick = self.next_tick;
        #[cfg(feature = "bites")]
        if let Some(b) = &mut self.bite
            && !b.armed
            && tick == b.arm_tick
        {
            b.armed = true;
            if b.bite == Bite::HashMap {
                b.fold = Some(hash_map_order(64));
            }
        }
        self.events.clear();
        self.reports.clamps.clear();
        self.reports.refusals.clear();
        self.fired.clear();
        self.blend_key = None;
        for s in &mut self.soldiers {
            s.vel = [0.0; 3];
        }
        self.apply_inputs(input);
        self.run_movers();
        for (pre, s) in self.pre.iter_mut().zip(&self.soldiers) {
            *pre = PreSnap::of(s);
        }
        for (c, rewind) in &input.commands {
            let rewind = match rewind {
                Some(r) if !held(history, r) => {
                    self.reports.rewind_outside += 1;
                    self.refuse(c, RefusalKind::RewindOutside);
                    None
                }
                other => *other,
            };
            self.run_command(c, rewind, true);
        }
        self.present_volumes(tick);
        self.run_shots(history);
        self.run_projectiles(tick);
        self.run_effects(tick);
        self.run_respawns();
        self.write_records();
        self.compose_own_volumes(tick);
        self.next_tick = tick.wrapping_add(1);
        self.elapsed += 1;
        debug_assert!(
            self.non_finite().is_none(),
            "a non-finite value in the state after tick {tick}: {}",
            self.non_finite().unwrap_or_default()
        );
        &self.events
    }

    fn state_hash(&self) -> u64 {
        let mut h = Fnv::new();
        self.write_state(&mut h);
        h.0
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

#[cfg(test)]
mod tests {
    //! The tests that read `World`'s private parts: the blend's boxes (T-M3), the movers' sizes (R-M1), the clamp's
    //! bound (TUNE-2), and T2-L1's two invariants (hull ∘ part's frame, and phase 3's boxes after the moves).

    use super::*;
    use crate::replay::Recording;
    use crate::seam::{AttributeChange, Placement};

    #[test]
    fn part_center_turns_a_parts_offset_by_its_hulls_yaw() {
        // T2-L1 (the test review's round 2; the advisor's U4d close ruling, B1.2 step 0): hull ∘ part's frame
        // convention on its own, not only through D-7's pin. At hull yaw 90 degrees a part 120 cm along the hull's own
        // +x sits 120 cm to the world's +y of the hull, and one 120 cm along its +y sits 120 cm to the world's -x, 100
        // cm up in both, within 1 cm: x' = x cos - y sin, y' = x sin + y cos (the old place_vehicles formula). A
        // rotation by -yaw (plant O1) or none (O2) fails it.
        let wire = Config::default();
        let (hx, hy) = (1_000, -500);
        let hull = [hx, hy, 0, wire.to_grid(YAW, FRAC_PI_2)];
        for (offset, want) in [
            ([120, 0, 100], [hx, hy + 120, 100]),
            ([0, 120, 100], [hx - 120, hy, 100]),
        ] {
            let got = part_center(hull, offset);
            assert!(
                (0..3).all(|k| (got[k] - want[k]).abs() <= 1),
                "offset {offset:?} at yaw 90 degrees: {got:?}, not {want:?}"
            );
        }
    }

    #[test]
    fn a_shot_with_no_rewind_meets_a_soldier_where_this_ticks_move_put_it() {
        // T2-L1 (the advisor's U4d close ruling, B1.2 step 0): phase 3 refreshes the soldiers' records after the moves,
        // so a shot with no Rewind meets each soldier where this tick's command moved it. In TC, one tick: the shooter
        // (slot 0) and the target (slot 1, 5 m north of it) are teleported into the open, the target's move speed set
        // to the most the attribute holds (100 m/s), and its command runs it 3.33 m east, out of its old box (80 cm
        // wide); the shooter, holding fire with no Rewind, hits it at its new chest and misses at its old one. Phase 3
        // without the refresh (plant O3: the soldiers' records from before this tick's teleports and moves) misses it
        // at its new chest.
        let scene = Scene::targets();
        let (shooter, target) = ([-6_000, -1_000, 0], [-6_000, -500, 0]);
        let shoot = |aim: [i32; 3]| {
            let mut world = World::new(&scene, 1, 0);
            let history = HitHistory::for_ms(1_000).expect("1 s of ticks");
            let tick = world.next_tick();
            let command = |slot: u8, buttons: u16, move_xy: [i8; 2], aim: [i32; 3]| {
                let c = Command {
                    slot,
                    tick,
                    seq: 1,
                    buttons,
                    presses: [0; 4],
                    move_xy,
                    aim,
                    view_tick: tick,
                    view_fraction: 0,
                };
                (c, None)
            };
            let input = TickInput {
                tick,
                commands: vec![
                    command(0, Command::FIRE, [0, 0], aim),
                    command(1, 0, [127, 0], [-6_000, 2_000, 140]),
                ],
                teleports: vec![
                    Placement {
                        slot: 0,
                        position: shooter,
                    },
                    Placement {
                        slot: 1,
                        position: target,
                    },
                ],
                attributes: vec![AttributeChange {
                    slot: 1,
                    attribute: Attribute::MoveSpeed,
                    value: MOVE_SPEED_MAX_CM_S,
                }],
                ..TickInput::default()
            };
            let events = world.step(&input, &history);
            let hit = events
                .iter()
                .any(|e| matches!(e, SimEvent::Hit { victim: 1, .. }));
            let shots = events
                .iter()
                .filter(|e| matches!(e, SimEvent::Shot { slot: 0, .. }))
                .count();
            (hit, shots, world.soldiers[1].pos)
        };
        let old = [target[0], target[1], 140];
        let (hit_old, shots, at) = shoot(old);
        assert_eq!(shots, 1, "the shooter fires once");
        assert!(
            at[0] - cm(target[0]) > 3.3 && (at[1] - cm(target[1])).abs() < 0.01,
            "the target ran 3.33 m east: {at:?}"
        );
        let new = [libm::roundf(at[0] * 100.0) as i32, target[1], 140];
        let (hit_new, shots, _) = shoot(new);
        assert_eq!(shots, 1, "the shooter fires once");
        assert!(hit_new, "a shot at the target's new chest {new:?} hits it");
        assert!(!hit_old, "a shot at its old chest {old:?} misses it");
    }

    /// A box's centre and half extents as the reference reads them, m, in f64 from its integer centimetres.
    fn metres(b: &VolumeBox) -> ([f64; 3], [f64; 3]) {
        (
            b.center.map(|k| f64::from(k) / 100.0),
            b.half.map(|k| f64::from(k) / 100.0),
        )
    }

    /// `got` against the reference blend of `a` toward `b` by `fraction` 256ths: centre a + (b − a) × fraction / 256,
    /// a's half extents, within 0.1 mm (f32's rounding of metres below 330 m is under 0.05 mm).
    fn assert_blend(
        got: &FBox,
        a: &VolumeBox,
        b: &VolumeBox,
        fraction: u8,
        live: bool,
        what: &str,
    ) {
        let ((ca, h), (cb, _)) = (metres(a), metres(b));
        let t = f64::from(fraction) / 256.0;
        for k in 0..3 {
            let c = ca[k] + (cb[k] - ca[k]) * t;
            let (lo, hi) = (f64::from(got.min[k]), f64::from(got.max[k]));
            assert!(
                (lo - (c - h[k])).abs() <= 1e-4 && (hi - (c + h[k])).abs() <= 1e-4,
                "{what}, fraction {fraction}, axis {k}: [{lo}, {hi}] against the reference centre {c} ± {}",
                h[k]
            );
        }
        assert_eq!(got.live, live, "{what}, fraction {fraction}: live");
    }

    #[test]
    fn the_blend_puts_each_box_at_its_fraction_of_the_way() {
        // T-M3 (the test review's M3; the advisor's W1-U4D): the blended boxes `World::blend` gives a shot, pinned
        // against a reference computed here, for fractions 0, 1, 127, 128 and 255. The boxes move 256 m (and 25.6 m up)
        // between the two ticks, so one 256th is 1 m: a blend by another divisor (E6's / 255) is 3.9 mm off at fraction
        // 1 and 1 m off at 255, where the hit-or-miss check (tests/world.rs, D-9) sees only past its 6 cm box.
        let mut world = World::new(&Scene::course(), 1, 0);
        let mut history = HitHistory::for_ms(1_000).expect("1 s of ticks");
        let shift = |b: VolumeBox, d: [i32; 3], half: [u16; 3], live: bool| VolumeBox {
            center: [0, 1, 2].map(|k| b.center[k] + d[k]),
            half,
            live,
        };
        let soldier = VolumeBox {
            center: [-12_800, 3_000, 90],
            half: [40, 40, 90],
            live: true,
        };
        let part = VolumeBox {
            center: [1_000, -12_800, 100],
            half: [50, 50, 50],
            live: true,
        };
        let door = VolumeBox {
            center: [-5_000, 7_000, 150],
            half: [15, 100, 150],
            live: false,
        };
        let a = HitVolumes {
            tick: 100,
            soldiers: vec![soldier],
            cover: vec![part, door],
        };
        let b = HitVolumes {
            tick: 101,
            soldiers: vec![shift(
                soldier,
                [25_600, -25_600, 2_560],
                [10, 10, 10],
                false,
            )],
            cover: vec![
                shift(part, [25_600, 25_600, 0], [5, 5, 5], false),
                shift(door, [-25_600, 0, 2_560], [1, 1, 1], true),
            ],
        };
        history.push(&a);
        history.push(&b);
        for fraction in [0u8, 1, 127, 128, 255] {
            for cover in [CoverVariant::Rewound, CoverVariant::Present] {
                world.blend_key = None;
                world.blend(
                    &history,
                    &Rewind {
                        tick_a: 100,
                        fraction,
                        cover,
                    },
                );
                // The victim takes a's size and a's liveness, and is live only while it is alive now (it is).
                assert_eq!(world.blend_soldiers.len(), 1);
                assert_blend(
                    &world.blend_soldiers[0],
                    &a.soldiers[0],
                    &b.soldiers[0],
                    fraction,
                    true,
                    "the soldier",
                );
                match cover {
                    CoverVariant::Rewound => {
                        assert_eq!(world.blend_cover.len(), 2);
                        for i in 0..2 {
                            let what = format!("rewound cover {i}");
                            let live = a.cover[i].live;
                            assert_blend(
                                &world.blend_cover[i],
                                &a.cover[i],
                                &b.cover[i],
                                fraction,
                                live,
                                &what,
                            );
                        }
                    }
                    CoverVariant::Present => {
                        // The cover as it is now, whatever the ring held.
                        assert_eq!(world.blend_cover.len(), world.volumes.cover.len());
                        for (got, now) in world.blend_cover.iter().zip(&world.volumes.cover) {
                            assert_blend(got, now, now, fraction, now.live, "present cover");
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn load_movers_never_grows_a_mover_from_decoded_records() {
        // The rust review's M1 = the test review's M5 (probe C: cover 7 -> 408): `load_movers` skips a record whose
        // index the world has no entity for, as its doc says, and never grows a pool from decoded data. In CO (no
        // vehicle) and SS (15 hulls, 124 parts): records past every count, and a part naming a hull the world has not;
        // the movers keep their sizes and every value a valid record did not set.
        let bogus = |kind, index: u16, values: &[i32]| {
            let mut grid = [0; EntityRecord::FIELDS];
            grid[..values.len()].copy_from_slice(values);
            EntityRecord {
                kind,
                index,
                generation: 0,
                area: 0,
                grid,
                raw: [0; EntityRecord::FIELDS],
            }
        };
        for scene in [Scene::course(), Scene::stress()] {
            let mut world = World::new(&scene, 1, 0);
            let before = world.movers.clone();
            let (vehicles, parts) = (world.vehicles.len() as u16, world.parts.len() as u16);
            let records = [
                bogus(EntityKind::VehiclePart, 400, &[0, -1_500, 100, 0, 200, 0]),
                bogus(EntityKind::VehiclePart, parts, &[1, 2, 3, 0, 200, 0]),
                bogus(EntityKind::VehiclePart, 0, &[1, 2, 3, 0, 200, 63]),
                bogus(EntityKind::Vehicle, vehicles, &[100, 100, 0, 0]),
                bogus(EntityKind::Vehicle, 63, &[100, 100, 0, 0]),
                bogus(EntityKind::Door, 99, &[1]),
                bogus(EntityKind::Wall, 3, &[0, 0]),
            ];
            world.load_movers(&records);
            let m = &world.movers;
            let sizes = |m: &Movers| {
                [
                    m.hulls.len(),
                    m.parts.len(),
                    m.boxes.len(),
                    m.vehicles.len(),
                    m.doors_closed.len(),
                ]
            };
            assert_eq!(sizes(m), sizes(&before), "{}: no mover grew", scene.name());
            assert_eq!(
                (
                    &m.hulls,
                    &m.parts,
                    &m.boxes,
                    &m.vehicles,
                    &m.doors_closed,
                    m.wall_standing
                ),
                (
                    &before.hulls,
                    &before.parts,
                    &before.boxes,
                    &before.vehicles,
                    &before.doors_closed,
                    before.wall_standing
                ),
                "{}: nothing changed",
                scene.name()
            );
        }
    }

    #[test]
    fn no_scene_places_a_fire_origin_near_the_clamp_bound() {
        // TUNE-2 (the advisor's W1-U4D): the clamp puts a far point at the reach for every fire origin at or below the
        // grid's top (feet at or below 3,955 cm: the module text). Every place a scene puts a soldier's feet (its spawn
        // points, its soldiers' starts, TC's plates, its synthetic recording's spawns and teleports) and every greybox
        // surface (the map's edge, the buildings, the crates, the doors, the wall stand-in) is at or below 10 m, 29.55 m
        // under it. A scripted teleport above it is a scenario error the runner refuses (U4r's carry).
        const FAR_BELOW_CM: i32 = 1_000;
        assert_eq!(FEET_Z_BOUND_CM, 3_955);
        const _: () = assert!(FAR_BELOW_CM < FEET_Z_BOUND_CM);
        let wire = Config::default();
        for scene in [
            Scene::stress(),
            Scene::estimate(),
            Scene::course(),
            Scene::targets(),
        ] {
            let name = scene.name();
            let world = World::new(&scene, 1, 0);
            for p in world.spawn_points.iter().flatten() {
                assert!(p[2] <= FAR_BELOW_CM, "{name}: spawn point {p:?}");
            }
            for (slot, s) in world.soldiers.iter().enumerate() {
                let feet = wire.to_grid(POSITION_Z, s.pos[2]);
                let top = if scene.is_plate(slot as u8) {
                    feet + 2 * i32::from(PLATE_HALF[2])
                } else {
                    feet
                };
                assert!(
                    top <= FAR_BELOW_CM,
                    "{name}: slot {slot} at {feet} cm, its top {top} cm"
                );
            }
            let greybox = &world.greybox;
            for b in greybox
                .solids
                .iter()
                .chain(&greybox.doors)
                .chain([&greybox.wall])
            {
                assert!(
                    b.max_cm[2] <= FAR_BELOW_CM,
                    "{name}: a greybox box {:?} to {:?}",
                    b.min_cm,
                    b.max_cm
                );
            }
            let rec = Recording::synthetic(&scene, 1, 0, 3_000, 2_000);
            let placed = rec
                .ticks
                .iter()
                .flat_map(|t| t.spawns.iter().chain(&t.teleports));
            let mut n = 0;
            for p in placed {
                n += 1;
                assert!(
                    p.position[2] <= FAR_BELOW_CM,
                    "{name}: a scripted placement {p:?}"
                );
            }
            assert!(n > 0, "{name}: the recording places soldiers");
        }
    }
}
