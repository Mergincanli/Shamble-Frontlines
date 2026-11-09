//! The seam between the room and the sim (u4-build-plan.md §4.2; plan §4.6; the advisor's Q1 and Q2 rulings): one
//! trait, [`SimCore`], with one implementation (`World`, built in U4d), and the plain-data types that cross it.
//!
//! npt-room is generic over `SimCore` and owns the [`HitHistory`] it lends to each step. Everything the room builds or
//! reads here is integers (npt-room is float-free: u4-build-plan.md §4.11), and a recording holds the input field by
//! field (§4.15). U4d's `World` adds what only it decides: the volumes inside [`HitVolumes`], and the spawns,
//! teleports and scripted attribute changes that §4.2 also routes through [`TickInput`].

use crate::grid::{Field, PITCH, POSITION_XY, POSITION_Z, VELOCITY, YAW};
use crate::history::HitHistory;

/// One soldier's command for one tick, as the room accepted it: the decoded `Input` command after the room's checks
/// (plan §4.4), and what a recording holds for it (u4-build-plan.md §4.15). Integers only.
///
/// **What the sim reads:** `slot`, `seq`, `buttons`, `presses`, `move_xy` and `aim`, and nothing else. `tick`,
/// `view_tick` and `view_fraction` are what the client claimed: they belong to the room (its strictly-newer tick check
/// and `rewind_at`) and to the recording (-30's pre-check re-encodes them as `Input`: u4-build-plan.md §4.15). For lag
/// compensation the sim uses the command's [`Rewind`], the view time the room accepted, never the claimed one (the
/// advisor's condition 1: the sim keeps what the server accepted), so two commands that differ only in these three
/// fields step to equal state hashes (U4d's test).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Command {
    /// The soldier's slot.
    pub slot: u8,
    /// The tick the client stamped the command for: the room's and the recording's, never read by the sim.
    pub tick: u32,
    /// The command's sequence number (16 bits, compared wrap-aware: PM-0001-58).
    pub seq: u16,
    /// The buttons held, one bit each.
    pub buttons: u16,
    /// Each pressable button's running press counter (wrapping: PM-0001-58); the sim takes press edges from them.
    pub presses: [u8; 4],
    /// The move vector, each component in -127..=127 (127 is full speed); the room has checked x² + y² ≤ 127².
    pub move_xy: [i8; 2],
    /// The aim point in the position grid's integers (cm): [`crate::grid::POSITION_XY`] for x and y,
    /// [`crate::grid::POSITION_Z`] for z. The sim clamps a point beyond the weapon's reach (§4.2).
    pub aim: [i32; 3],
    /// The client's view time as the command claims it: the tick of the older snapshot it drew ...
    pub view_tick: u32,
    /// ... and the blend from that tick toward the next, in 256ths. The room turns the pair into a [`Rewind`] (or
    /// replaces it by its estimate); the sim never reads either.
    pub view_fraction: u8,
}

/// R-10's two cover variants (OQ-0001-9; plan.md:1382): the state a lag-compensated shot sees cover items in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CoverVariant {
    /// Doors, the wall stand-in and vehicle parts rewound with the victim, to the shooter's view time.
    Rewound,
    /// Cover items as they are at the tick the shot runs.
    Present,
}

/// How a lag-compensated shot sees the past (u4-build-plan.md §4.2; the advisor's Q2 ruling): the room's `rewind_at`
/// turns a command's accepted view time into these integers, and the sim blends the two history ticks linearly, as the
/// client's Interp drew them. It rides with each command that holds fire or presses it.
///
/// **A tick the lent ring does not hold:** the ring lacks `tick_a`, or `fraction` is above 0 and the ring lacks
/// `tick_a + 1` (wrapping), the tick the blend reaches toward; at `fraction` 0 only `tick_a` is read. Such a Rewind is
/// refused and counted by the sim, in [`InputReports::rewind_outside`] and never in the hash.
///
/// **A refused Rewind behaves as `None`:** the command steps exactly as the same command with no Rewind, on that tick
/// and after (D-10's twin is the forged command with only the refused part removed, so the two must give equal state
/// hashes). What a command that holds or presses fire does without a Rewind is `World`'s rule, U4d's; it is one rule
/// for both.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rewind {
    /// The older of the two history ticks the view time falls between.
    pub tick_a: u32,
    /// The blend from `tick_a` toward `tick_a + 1`, in 256ths (0 is `tick_a` itself).
    pub fraction: u8,
    /// Which state the shot's cover items take.
    pub cover: CoverVariant,
}

/// Everything one step takes from outside the sim (u4-build-plan.md §4.2, §4.15). The room fills one per tick and the
/// replay driver reads one per tick from a recording.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TickInput {
    /// The tick this input steps: the world's next tick.
    pub tick: u32,
    /// The accepted commands, each with the [`Rewind`] the room computed for it (present when the command holds or
    /// presses fire), in **the canonical order**: slot ascending, then, within one slot, command tick strictly
    /// ascending, compared wrap-aware (`b` comes after `a` when `b.wrapping_sub(a) as i32 > 0`). The room sorts into
    /// it, a recording holds it and the sim runs the commands in it, so legs that received the same commands in
    /// different arrival orders step alike (-30's per-tick hash comparison). U4d's `World` debug-asserts it.
    pub commands: Vec<(Command, Option<Rewind>)>,
}

/// What kind of entity a record describes; an entity's id is its kind, index and generation (ADR 0001 part 6).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum EntityKind {
    Soldier,
    Vehicle,
    /// One part of a vehicle, as its own record (the stress scene's 4 large vehicles have at least 20: §4.13).
    VehiclePart,
    /// A trajectory record (type, start tick, start position, velocity, fuse: ADR 0001 part 6).
    Projectile,
    Door,
    /// The destructible-wall stand-in (R-10).
    Wall,
    /// One of S1's effect rows (8 per soldier, §4.13), in its owner's own-state section.
    Effect,
}

/// Eight flag bits, for stance and actions (the catalogue's remote-soldier row).
const FLAGS: Field = Field::integer(0, 255);
/// A tick by its u32 bits, held as i32 (a projectile's start tick).
const TICK: Field = Field::integer(i32::MIN, i32::MAX);
/// A door's state: 0 closed, 1 open.
const OPEN: Field = Field::integer(0, 1);
/// A position: x, y, z.
const XYZ: [Field; 3] = [POSITION_XY, POSITION_XY, POSITION_Z];
const SOLDIER: [Field; 9] = [
    XYZ[0], XYZ[1], XYZ[2], YAW, PITCH, VELOCITY, VELOCITY, VELOCITY, FLAGS,
];
/// A transform on the ground: x, y, z and yaw (a hull, a part).
const TRANSFORM: [Field; 4] = [XYZ[0], XYZ[1], XYZ[2], YAW];
const PROJECTILE: [Field; 7] = [TICK, XYZ[0], XYZ[1], XYZ[2], VELOCITY, VELOCITY, VELOCITY];
const DOOR: [Field; 1] = [OPEN];

impl EntityKind {
    /// Every kind, in declaration order: npt-proto builds its field-group table for each.
    pub const ALL: [EntityKind; 7] = [
        EntityKind::Soldier,
        EntityKind::Vehicle,
        EntityKind::VehiclePart,
        EntityKind::Projectile,
        EntityKind::Door,
        EntityKind::Wall,
        EntityKind::Effect,
    ];

    /// The kind's field layout: what each entry of an [`EntityRecord`]'s `grid` and `raw` holds, in order. The one
    /// home of that order (the arch review's F2): U4d's `World` fills records in it, and npt-proto's field-group table
    /// and its `EntityRecord` ↔ wire mapping are built from this function, never from a copy, so a change here needs
    /// no edit in npt-proto.
    ///
    /// U4s's lists are the catalogue's spatial rows (docs/design/netcode-proof-messages.md §3): a soldier's position,
    /// yaw, pitch, velocity and 8 flag bits; a hull's and a part's transform; a projectile's start tick, start position
    /// and velocity; a door's state. U4d completes them (health, weapon id and state, a projectile's type and fuse, the
    /// wall's and the effect rows' fields) with the grid's fields and [`Field::integer`], within
    /// [`EntityRecord::FIELDS`] (a const assert below holds every layout to it).
    pub const fn layout(self) -> &'static [Field] {
        match self {
            EntityKind::Soldier => &SOLDIER,
            EntityKind::Vehicle | EntityKind::VehiclePart => &TRANSFORM,
            EntityKind::Projectile => &PROJECTILE,
            EntityKind::Door => &DOOR,
            EntityKind::Wall | EntityKind::Effect => &[],
        }
    }
}

/// One entity's snapshot fields, read out after a step (u4-build-plan.md §4.2): the encoder codes them, npt-proto
/// maps them to and from the wire (one home, §4.4), and -57's log takes the pre-grid values. The room copies `raw` and
/// never computes with it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EntityRecord {
    pub kind: EntityKind,
    /// The entity's index among its kind (a soldier's slot, a pool index).
    pub index: u16,
    /// Its generation: a change is a removal and an addition (ADR 0001 part 6).
    pub generation: u16,
    /// The area it is in (the static graph's numbering), for interest.
    pub area: u16,
    /// Its fields as integers, in its kind's [`EntityKind::layout`] order; entries past the layout 0. A float field's
    /// entry is the **wire** grid's `Config::default().to_grid(field, value)` of the value `raw` holds, computed apart
    /// from the own-copy snap and its own [`crate::grid::Config`], so a snap that rounds otherwise
    /// (PM-0001-41/server-quant-differs) shows as this entry differing from the snapped copy; an integer field's entry
    /// is the integer itself, a tick by its u32 bits.
    pub grid: [i32; EntityRecord::FIELDS],
    /// The same fields' values **before the own-copy snap**, each f32 by its bits (`to_bits`), for -57's log, which
    /// pairs them with what the client decoded (u4-build-plan.md §4.6, T20); 0 for a field that is not a float.
    pub raw: [u32; EntityRecord::FIELDS],
}

impl EntityRecord {
    /// Fields per record: U4d's full soldier (position, yaw, pitch, velocity, health, flags, weapon and weapon state)
    /// fills 12.
    pub const FIELDS: usize = 12;
}

// Every kind's layout fits a record.
const _: () = {
    let mut i = 0;
    while i < EntityKind::ALL.len() {
        assert!(
            EntityKind::ALL[i].layout().len() <= EntityRecord::FIELDS,
            "every kind's layout fits EntityRecord::FIELDS"
        );
        i += 1;
    }
};

/// One tick's hit volumes for lag compensation (u4-build-plan.md §4.2, §4.9), as the world reads them out after a
/// step: the room pushes them into its [`HitHistory`], and the replay driver does the same. U4s fixes their tick; the
/// volumes themselves (soldiers, vehicle parts and R-10's cover items) are `World`'s, added in U4d with the hitscan,
/// as plain data the room copies and never reads.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HitVolumes {
    /// The tick the volumes are from.
    pub tick: u32,
}

/// Each portal of the static graph, open or closed (u4-build-plan.md §4.2, A2): the room's AreaFlood re-floods when
/// these change, with no float and no sim internals.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PortalStates {
    /// One entry per portal, in the static graph's order: true when open.
    pub open: Vec<bool>,
}

/// One event of a step, in the tick's event list (u4-build-plan.md §4.2): the list is state and in the hash
/// (ADR 0004:544-561). The room numbers and queues them per client (§4.7). Clamp reports and refusals are never events
/// ([`InputReports`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SimEvent {
    /// Cosmetic: `slot` fired the shot of its command `seq`; sent once, never to the shooter's own client
    /// (ADR 0001 part 6).
    Shot { slot: u8, seq: u16 },
    /// Gameplay: the shot of `shooter`'s command `seq` hit soldier `victim` for `damage`; resent until acknowledged,
    /// and the shooter's hit confirmation.
    Hit {
        shooter: u8,
        seq: u16,
        victim: u8,
        damage: u16,
    },
    /// Gameplay: `killer` killed `victim`.
    Kill { killer: u8, victim: u8 },
}

/// A point beyond the weapon's reach, moved along its own ray to exactly the reach and snapped to the grid
/// (u4-build-plan.md §4.2, Q25): never refused, never counted as a refusal, never an event, never in the hash.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClampReport {
    pub slot: u8,
    pub seq: u16,
    /// The point the shot used, in the position grid's integers (cm).
    pub point: [i32; 3],
}

/// What the sim did with refused or clamped input, outside `state_hash` and outside the hashed event list (the
/// advisor's condition 1): the room reads it through [`SimCore::reports`] alone and logs it. Refused input leaves no
/// trace in the state.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct InputReports {
    /// This tick's clamp reports, in the order the shots ran.
    pub clamps: Vec<ClampReport>,
    /// Running refusal counters since the world began, one per kind: a fire edge in the weapon's cooldown ...
    pub cooldown: u64,
    /// ... a fire edge with no ammunition ...
    pub ammunition: u64,
    /// ... a reload edge the weapon's state refuses ...
    pub reload: u64,
    /// ... a non-finite command component: always 0 while [`Command`] holds integers only, since no integer is
    /// non-finite; kept because u4-build-plan.md §4.2 lists the kind and D-4 tests it (whether to drop both is the
    /// advisor's question) ...
    pub non_finite: u64,
    /// ... a [`Rewind`] naming a tick the lent ring does not hold ...
    pub rewind_outside: u64,
    /// ... and an action refused because its pool was full (pools have fixed capacity: §4.2).
    pub pool_full: u64,
}

/// The seam: the one trait the room drives the sim through (u4-build-plan.md §4.2). One implementation, `World`.
pub trait SimCore {
    /// Runs one tick on `input`, the room's ring of past hit volumes lent for its lag-compensated shots; returns the
    /// tick's event list.
    fn step(&mut self, input: &TickInput, history: &HitHistory) -> &[SimEvent];

    /// The state's canonical hash (plan §4.5): its event list included; [`InputReports`] never.
    fn state_hash(&self) -> u64;

    /// Every entity's snapshot fields after the last step.
    fn entity_records(&self) -> &[EntityRecord];

    /// The hit volumes after the last step, for the room's [`HitHistory`].
    fn hit_volumes(&self) -> &HitVolumes;

    /// Each portal open or closed after the last step.
    fn portal_states(&self) -> &PortalStates;

    /// The last step's clamp reports and the running refusal counters.
    fn reports(&self) -> &InputReports;
}
