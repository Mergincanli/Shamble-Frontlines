//! The seam between the room and the sim (u4-build-plan.md §4.2; plan §4.6; the advisor's Q1 and Q2 rulings): one
//! trait, [`SimCore`], with one implementation ([`crate::World`]), and the plain-data types that cross it.
//!
//! npt-room is generic over `SimCore` and owns the [`HitHistory`] it lends to each step. Everything the room builds or
//! reads here is integers (npt-room is float-free: u4-build-plan.md §4.11), and a recording holds the input field by
//! field (§4.15): the commands with their Rewinds, and the spawns, teleports and scripted attribute changes that §4.2
//! also routes through [`TickInput`]. [`HitVolumes`] holds integer boxes the room copies into its ring.

use crate::TICK_HZ;
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

/// The button bits of [`Command::buttons`] and the indices of [`Command::presses`]: the one home the sim, the client
/// and the bots read (the netcode review's F10). A held button is its bit; a pressable button's presses are counted in
/// its counter, which the sender advances by one per press, wrapping, so a resent command neither loses nor doubles one
/// (ADR 0001:226).
impl Command {
    /// Held: the trigger. While held and the weapon is ready, the stand-in weapon fires automatically.
    pub const FIRE: u16 = 1 << 0;
    /// Held: crouching, half the move speed.
    pub const CROUCH: u16 = 1 << 1;
    /// Held: sprinting, one and a half times the move speed (not while crouching or firing).
    pub const SPRINT: u16 = 1 << 2;
    /// Every bit the sim reads; the others are ignored.
    pub const BUTTONS: u16 = Command::FIRE | Command::CROUCH | Command::SPRINT;

    /// The fire button's press counter: a press fires at once if the weapon is ready, else it is refused and counted
    /// (the cooldown, ammunition and reload edges: u4-build-plan.md §4.2).
    pub const PRESS_FIRE: usize = 0;
    /// The reload button's press counter: a press starts a reload unless the magazine is full or a reload is running
    /// (then refused and counted).
    pub const PRESS_RELOAD: usize = 1;
    /// The throw button's press counter: a press throws a grenade, a trajectory record, unless the projectile pool is
    /// full (then refused and counted).
    pub const PRESS_THROW: usize = 2;
    /// The melee button's press counter: named for the catalogue's action starts (fire, reload, throw, melee); the
    /// stand-in sim has no melee and reads no press from it.
    pub const PRESS_MELEE: usize = 3;
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
///
/// **Integers only, every field** (the advisor's U4s ruling, NF-D4): every value that enters the sim from outside is
/// finite by type, so the sim refuses and counts nothing as non-finite. If a float field ever joins this type, a
/// non-finite counter and its refusal test come back with it.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TickInput {
    /// The tick this input steps: the world's next tick. A label for the room and the recording: `World` counts its
    /// own ticks and never reads it.
    pub tick: u32,
    /// The accepted commands, each with the [`Rewind`] the room computed for it (present when the command holds or
    /// presses fire), in **the canonical order**: slot ascending, then, within one slot, command tick strictly
    /// ascending, compared wrap-aware (`b` comes after `a` when `b.wrapping_sub(a) as i32 > 0`). The room sorts into
    /// it, a recording holds it and the sim runs the commands in it, so legs that received the same commands in
    /// different arrival orders step alike (-30's per-tick hash comparison). U4d's `World` debug-asserts it.
    pub commands: Vec<(Command, Option<Rewind>)>,
    /// Soldiers spawned this tick, before any command runs: alive at the position, at full health and ammunition.
    pub spawns: Vec<Placement>,
    /// Soldiers moved this tick, before any command runs, keeping their health and weapon (a dead soldier is not moved).
    pub teleports: Vec<Placement>,
    /// Scripted attribute changes this tick, before any command runs (PM-0001-60's own-state changes).
    pub attributes: Vec<AttributeChange>,
}

/// A soldier put at a point by [`TickInput::spawns`] or [`TickInput::teleports`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Placement {
    /// The soldier's slot; a slot the scene does not have is ignored.
    pub slot: u8,
    /// Its feet, in the position grid's integers (cm): [`crate::grid::POSITION_XY`] for x and y,
    /// [`crate::grid::POSITION_Z`] for z, each brought into its field's range.
    pub position: [i32; 3],
}

/// One scripted change of a soldier's attribute (u4-build-plan.md §5 U4g: the own-state section's move-speed and
/// fire-interval attributes).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AttributeChange {
    /// The soldier's slot; a slot the scene does not have is ignored.
    pub slot: u8,
    /// Which attribute.
    pub attribute: Attribute,
    /// Its new value, in the attribute's unit, brought into the attribute's range (each [`Attribute`]'s), so the
    /// own-state record's field holds every value a soldier can reach (the advisor's W1-S1).
    pub value: u32,
}

/// A soldier attribute a script may set.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Attribute {
    /// The walking speed in cm/s (crouching halves it, sprinting multiplies it by 1.5); above 10,000 (100 m/s) is
    /// taken as 10,000.
    MoveSpeed,
    /// The weapon's interval between automatic shots in µs; 0 is taken as 1, and above 10,000,000 (10 s) as
    /// 10,000,000.
    FireInterval,
}

/// The weapon clock's units in one tick (`World`'s fire timer and reload): one µs is `TICK_HZ` units, so an interval of
/// `i` µs costs `i × TICK_HZ` units at any tick rate (u4-build-plan.md §4.13: "the stand-in weapon's interval in
/// integer sub-tick units").
pub(crate) const UNITS_PER_TICK: i64 = 1_000_000;
/// The ranges `World` keeps a soldier's attributes and its weapon's reload in ([`Attribute`]; the advisor's W1-S1: the
/// own-state record's fields hold every value the world can reach). A scene's starting values are inside them
/// (`World::new` asserts it), and a scripted change is brought into them. The move speed's top, cm/s ...
pub(crate) const MOVE_SPEED_MAX_CM_S: u32 = 10_000;
/// ... the fire interval's, µs (its bottom is 1) ...
pub(crate) const FIRE_INTERVAL_MAX_US: u32 = 10_000_000;
/// ... and the longest reload a scene's weapon may have, ms.
pub(crate) const RELOAD_MAX_MS: u32 = 10_000;
// Both clocks' largest values fit an i32 at this tick rate: the fire timer's (one interval) and the reload's.
const _: () = assert!(
    FIRE_INTERVAL_MAX_US as i64 * TICK_HZ as i64 <= i32::MAX as i64
        && RELOAD_MAX_MS as i64 * 1_000 * TICK_HZ as i64 <= i32::MAX as i64
);

/// What kind of entity a record describes; an entity's id is its kind, index and generation (ADR 0001 part 6).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum EntityKind {
    /// A soldier, one per slot whether alive or awaiting respawn; its index is the slot.
    Soldier,
    /// A big vehicle's hull (the stress scene has 15: §4.13); its parts are their own records.
    Vehicle,
    /// One part of a vehicle, as its own record (the stress scene's 4 large vehicles have at least 20: §4.13).
    VehiclePart,
    /// A trajectory record (type, start tick, start position, velocity, fuse: ADR 0001 part 6).
    Projectile,
    /// A door of the greybox, open or closed: each door is a portal between two areas (u4-build-plan.md §4.2, A2).
    Door,
    /// The destructible-wall stand-in (R-10).
    Wall,
    /// One of S1's effect rows (8 per soldier, §4.13), in its owner's own-state section.
    Effect,
    /// One soldier's owner-only state (the advisor's W1-S1): every value that decides that soldier's next ticks and
    /// is not in its [`EntityKind::Soldier`] record (its velocity, its weapon's ammunition, fire timer and reload, its
    /// move-speed and fire-interval attributes). Its index is the slot and its generation the soldier's; it travels
    /// only in its owner's own-state section, never to another client (the catalogue's remote-soldier row has none of
    /// it).
    OwnState,
}

/// Eight flag bits, for stance and actions (the catalogue's remote-soldier row): `World`'s `FLAG_*` bits.
const FLAGS: Field = Field::integer(0, 255);
/// A tick by its u32 bits, held as i32 (a projectile's start tick).
const TICK: Field = Field::integer(i32::MIN, i32::MAX);
/// A door's state: 0 closed, 1 open.
const OPEN: Field = Field::integer(0, 1);
/// Health points (a soldier's, a vehicle part's, the wall's).
const HEALTH: Field = Field::integer(0, 1_023);
/// A weapon id (the stand-in has one weapon, 0).
const WEAPON: Field = Field::integer(0, 15);
/// A vehicle part's hull: the index of its vehicle.
const PARENT: Field = Field::integer(0, 63);
/// A projectile's type: 0 the stress scene's stand-in (inert), 1 a thrown grenade.
const PROJECTILE_TYPE: Field = Field::integer(0, 3);
/// A projectile's fuse: the ticks from its start tick to its blast.
const FUSE: Field = Field::integer(0, 65_535);
/// The wall stand-in's state: 0 down, 1 standing.
const STANDING: Field = Field::integer(0, 1);
/// An effect row's kind (0 to 7).
const EFFECT_KIND: Field = Field::integer(0, 7);
/// An effect row's magnitude.
const EFFECT_MAGNITUDE: Field = Field::integer(0, 255);
/// A soldier's ammunition: the rounds in its magazine.
const AMMO: Field = Field::integer(0, u16::MAX as i32);
/// The weapon's fire timer, in weapon clock units ([`UNITS_PER_TICK`] a tick): at most 0 is ready; it runs from one
/// tick's carry below 0 up to one whole interval.
const FIRE_TIMER: Field = Field::integer(
    1 - UNITS_PER_TICK as i32,
    (FIRE_INTERVAL_MAX_US as i64 * TICK_HZ as i64) as i32,
);
/// The reload left, in weapon clock units (0: not reloading).
const RELOAD_LEFT: Field =
    Field::integer(0, (RELOAD_MAX_MS as i64 * 1_000 * TICK_HZ as i64) as i32);
/// The move-speed attribute, cm/s.
const MOVE_SPEED: Field = Field::integer(0, MOVE_SPEED_MAX_CM_S as i32);
/// The fire-interval attribute, µs.
const FIRE_INTERVAL: Field = Field::integer(1, FIRE_INTERVAL_MAX_US as i32);
/// A position: x, y, z.
const XYZ: [Field; 3] = [POSITION_XY, POSITION_XY, POSITION_Z];
/// The catalogue's remote-soldier row (docs/design/netcode-proof-messages.md §3; ADR 0001:309-311; the advisor's
/// W1-S1): U4s's position, yaw, pitch and 8 flag bits, then U4d's health and weapon id. Velocity and the weapon's state
/// are the owner's alone ([`OWN_STATE`]); "reloading" is a flag bit.
const SOLDIER: [Field; 8] = [XYZ[0], XYZ[1], XYZ[2], YAW, PITCH, FLAGS, HEALTH, WEAPON];
/// A soldier's owner-only state (the advisor's W1-S1): its velocity (x, y, z), its ammunition, its fire timer, its
/// reload left, and its move-speed and fire-interval attributes (PM-0001-60's two rows).
const OWN_STATE: [Field; 8] = [
    VELOCITY,
    VELOCITY,
    VELOCITY,
    AMMO,
    FIRE_TIMER,
    RELOAD_LEFT,
    MOVE_SPEED,
    FIRE_INTERVAL,
];
/// A transform on the ground: x, y, z and yaw (a hull).
const TRANSFORM: [Field; 4] = [XYZ[0], XYZ[1], XYZ[2], YAW];
/// A part: its transform relative to its hull, in the hull's frame (offset x, y and z, and yaw), then its health and
/// its hull ("hull and turret transforms and health per part; parts travel with their parent": the catalogue's
/// vehicles row; the advisor's W1-U4D H1). A rigid part's record changes only when its health does.
const PART: [Field; 6] = [XYZ[0], XYZ[1], XYZ[2], YAW, HEALTH, PARENT];
/// U4s's start tick, start position and velocity, then U4d's type and fuse (ADR 0001 part 6's trajectory record).
const PROJECTILE: [Field; 9] = [
    TICK,
    XYZ[0],
    XYZ[1],
    XYZ[2],
    VELOCITY,
    VELOCITY,
    VELOCITY,
    PROJECTILE_TYPE,
    FUSE,
];
const DOOR: [Field; 1] = [OPEN];
/// The wall stand-in: its health and whether it stands.
const WALL: [Field; 2] = [HEALTH, STANDING];
/// An effect row: kind, the tick it ends (its u32 bits), magnitude (its owner is its index / 8). It changes only when
/// the row starts or ends (the advisor's W1-U4D N-M5).
const EFFECT: [Field; 3] = [EFFECT_KIND, TICK, EFFECT_MAGNITUDE];

impl EntityKind {
    /// Every kind, in declaration order: npt-proto builds its field-group table for each.
    pub const ALL: [EntityKind; 8] = [
        EntityKind::Soldier,
        EntityKind::Vehicle,
        EntityKind::VehiclePart,
        EntityKind::Projectile,
        EntityKind::Door,
        EntityKind::Wall,
        EntityKind::Effect,
        EntityKind::OwnState,
    ];

    /// The kind's field layout: what each entry of an [`EntityRecord`]'s `grid` and `raw` holds, in order. The one
    /// home of that order (the arch review's F2): U4d's `World` fills records in it, and npt-proto's field-group table
    /// and its `EntityRecord` ↔ wire mapping are built from this function, never from a copy, so a change here needs
    /// no edit in npt-proto.
    ///
    /// The lists are the catalogue's spatial rows (docs/design/netcode-proof-messages.md §3), U4s's fields first and
    /// U4d's after them (carry 12), each a grid field or a [`Field::integer`], within [`EntityRecord::FIELDS`] (a const
    /// assert below holds every layout to it):
    /// - a soldier (the catalogue's remote row): position (x, y, z), yaw, pitch, 8 flag bits, then health and weapon
    ///   id;
    /// - a hull: its transform (x, y, z, yaw);
    /// - a part: its transform relative to its hull, in the hull's frame (offset x, y, z, and yaw), then its health and
    ///   its hull's index;
    /// - a projectile: start tick (its u32 bits), start position, velocity, then its type (0 the stand-in, 1 a
    ///   grenade) and its fuse in ticks;
    /// - a door: open (1) or closed (0);
    /// - the wall stand-in: its health, and standing (1) or down (0);
    /// - an effect row: its kind, the tick it ends (its u32 bits) and its magnitude;
    /// - a soldier's own state (owner only): velocity (x, y, z), ammunition, fire timer and reload left (weapon clock
    ///   units), move speed (cm/s) and fire interval (µs).
    pub const fn layout(self) -> &'static [Field] {
        match self {
            EntityKind::Soldier => &SOLDIER,
            EntityKind::Vehicle => &TRANSFORM,
            EntityKind::VehiclePart => &PART,
            EntityKind::Projectile => &PROJECTILE,
            EntityKind::Door => &DOOR,
            EntityKind::Wall => &WALL,
            EntityKind::Effect => &EFFECT,
            EntityKind::OwnState => &OWN_STATE,
        }
    }
}

/// One entity's snapshot fields, read out after a step (u4-build-plan.md §4.2): the encoder codes them, npt-proto
/// maps them to and from the wire (one home, §4.4), and -57's log takes the pre-grid values. The room copies `raw` and
/// never computes with it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EntityRecord {
    /// What the entity is, which names its fields ([`EntityKind::layout`]).
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
    /// Fields per record: 12, which every kind's layout fits (the longest, a projectile's, has 9).
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
/// step: the room pushes them into its [`HitHistory`], and the replay driver does the same. Plain integer data the room
/// copies and never reads: boxes in the position grid's centimetres, built from the entities' records by one function,
/// [`crate::World::hit_volumes_from`] (a soldier's box from its record's position; a part's from its hull's record
/// composed with its own), so a client builds the same volumes from the records it decoded (`World::ray_cast`'s
/// movers; the advisor's W1-U4D A-M1).
///
/// Its `Clone` is written by hand: `clone_from` copies field by field into the existing vectors (the rust review's
/// item 2: a derived `clone_from` replaces them, which allocates), so a push into a full [`HitHistory`] reuses the
/// evicted entry's storage.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct HitVolumes {
    /// The tick the volumes are from: `World`'s own tick counter (carry 15).
    pub tick: u32,
    /// Each soldier's box, by slot: the victims a shot can hit, always taken at the shooter's view time.
    pub soldiers: Vec<VolumeBox>,
    /// R-10's cover items: each vehicle part, then each door, then the wall stand-in; a shot takes them at its view
    /// time or at the present, by its [`CoverVariant`].
    pub cover: Vec<VolumeBox>,
}

impl HitVolumes {
    /// The most soldier boxes one tick holds: slots are a `u8`, and a scene has at most 64.
    pub const MAX_SOLDIERS: usize = 64;
    /// The most cover boxes one tick holds: 128 vehicle parts, 16 doors and the wall, rounded up.
    pub const MAX_COVER: usize = 160;
    /// One tick's bytes at most, the same on every target: the struct at its 64-bit size (the tick, then two vectors
    /// of three 8-byte words: 56 bytes; 28 on wasm32), and both vectors full of 20-byte boxes. [`HitHistory::for_ms`]
    /// refuses a ring that would hold more than 1 MB of them (10^6 B, as PM-0001-7 judges it: u4-build-plan.md §4.9;
    /// carry 14), so a recording's ring span replays alike natively and in Wasm.
    pub const MAX_BYTES: usize =
        56 + (HitVolumes::MAX_SOLDIERS + HitVolumes::MAX_COVER) * core::mem::size_of::<VolumeBox>();
}

// MAX_BYTES's two sizes hold on every target: the struct at most its 64-bit size, a box 20 bytes.
const _: () =
    assert!(core::mem::size_of::<HitVolumes>() <= 56 && core::mem::size_of::<VolumeBox>() == 20);

impl Clone for HitVolumes {
    fn clone(&self) -> HitVolumes {
        HitVolumes {
            tick: self.tick,
            soldiers: self.soldiers.clone(),
            cover: self.cover.clone(),
        }
    }

    /// Field by field: `Vec::clone_from` keeps the vector's storage when it is large enough.
    fn clone_from(&mut self, source: &HitVolumes) {
        self.tick = source.tick;
        self.soldiers.clone_from(&source.soldiers);
        self.cover.clone_from(&source.cover);
    }
}

/// One axis-aligned box of a [`HitVolumes`], in the position grid's centimetres.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VolumeBox {
    /// Its centre (x, y, z).
    pub center: [i32; 3],
    /// Its half extents (x, y, z).
    pub half: [u16; 3],
    /// Whether it is there at that tick: an alive soldier, a closed door, a standing wall, a part (always).
    pub live: bool,
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
    Shot {
        /// The shooter's slot.
        slot: u8,
        /// The sequence number of the command that fired.
        seq: u16,
    },
    /// Gameplay: the shot of `shooter`'s command `seq` hit soldier `victim` for `damage`; resent until acknowledged,
    /// and the shooter's hit confirmation.
    Hit {
        /// The shooter's slot (a grenade's thrower).
        shooter: u8,
        /// The sequence number of the command that fired (or threw).
        seq: u16,
        /// The slot of the soldier hit.
        victim: u8,
        /// The health points taken.
        damage: u16,
    },
    /// Gameplay: `killer` killed `victim`.
    Kill {
        /// The slot whose shot or grenade killed.
        killer: u8,
        /// The slot of the soldier killed.
        victim: u8,
    },
}

/// A point beyond the weapon's reach, moved along its own ray to exactly the reach and snapped to the grid
/// (u4-build-plan.md §4.2, Q25): never refused, never counted as a refusal, never an event, never in the hash.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClampReport {
    /// The slot whose command was clamped.
    pub slot: u8,
    /// The sequence number of that command.
    pub seq: u16,
    /// The point the shot used, in the position grid's integers (cm).
    pub point: [i32; 3],
}

/// One refused part of one command (the advisor's W1-U4D ruling on the press counters): which slot's command, by its
/// sequence number, and what was refused, so a twin removes exactly the refused parts (§4.10; D-10).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Refusal {
    /// The slot whose command it was.
    pub slot: u8,
    /// That command's sequence number.
    pub seq: u16,
    /// What was refused.
    pub kind: RefusalKind,
}

/// What a [`Refusal`] refused: one per running counter of [`InputReports`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RefusalKind {
    /// A fire press in the weapon's cooldown.
    Cooldown,
    /// A fire press with no ammunition.
    Ammunition,
    /// A reload press the weapon's state refuses (the magazine full, or a reload running).
    Reload,
    /// A [`Rewind`] naming a tick the lent ring does not hold (the command then runs as with none).
    RewindOutside,
    /// A throw into a full projectile pool.
    PoolFull,
}

/// What the sim did with refused or clamped input, outside `state_hash` and outside the hashed event list (the
/// advisor's condition 1): the room reads it through [`SimCore::reports`] alone and logs it. Refused input leaves no
/// trace in the state.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct InputReports {
    /// This tick's clamp reports, in the order the shots ran.
    pub clamps: Vec<ClampReport>,
    /// This tick's refusals, one per refused part of a command, in the order the commands ran (cleared each step, as
    /// the clamp reports are; the advisor's W1-U4D ruling): what a twin removes (§4.10).
    pub refusals: Vec<Refusal>,
    /// Running refusal counters since the world began, one per kind: a fire edge in the weapon's cooldown ...
    pub cooldown: u64,
    /// ... a fire edge with no ammunition ...
    pub ammunition: u64,
    /// ... a reload edge the weapon's state refuses ...
    pub reload: u64,
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

    /// The last step's clamp reports and refusals, and the running refusal counters.
    fn reports(&self) -> &InputReports;
}
