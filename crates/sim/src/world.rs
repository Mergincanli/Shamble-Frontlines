//! The world: the whole state of one match, and the step that advances it (ADR 0004 parts 1, 3, 4, 8 and 9). It holds
//! the tick counter, the one player's record, the soldier table and the match seed. The step reads no clock and
//! takes no time: one call is one tick.

use crate::aim::AimCm;
use crate::rules::Rules;
use crate::soldiers::{GROUNDED, MAX_SOLDIERS, Soldier, Soldiers, SpawnPoint};
use crate::state_hash::Fnv1a64;
use crate::vec::V3;

/// Ticks per second: the one build-time constant, the same on the server and the client (ADR 0004 part 4; TICK-30).
pub const TICK_HZ: u32 = 30;

/// The length of a match seed in bytes.
pub const SEED_BYTES: usize = 32;

/// The buttons a player's command has room for.
pub const BUTTONS: usize = 16;

/// The one player's slot and team, in one home: read by `World::new`, which puts his soldier there, and by the room's
/// join answer (netcode-architect's block 9.2, condition 1).
pub const PLAYER_SLOT: u16 = 0;
pub const PLAYER_TEAM: u8 = 0;

/// The longest move a command may hold: the wire's pair of `i8`, of length at most 127, which is a whole step of the
/// wish (a room refuses a longer one: netcode-architect's block 9.3).
pub const MOVE_REACH: i32 = 127;

/// Whether a move pair is within `MOVE_REACH`: x squared plus y squared at most 127 squared, in whole numbers (the
/// squares of -128 fit an `i32`).
#[must_use]
pub const fn move_in_reach(x: i8, y: i8) -> bool {
    let (x, y) = (x as i32, y as i32);
    x * x + y * y <= MOVE_REACH * MOVE_REACH
}

/// The length of one soldier's row in the canonical bytes (rust-reviewer's block 11.2).
pub const ROW_BYTES: usize = 60;

/// The length of the shared state's canonical bytes: the tick (4), the player's record (2 and 16), then the 48 rows
/// of the soldier table. It is the part a client holds too; what follows it is the authority's alone.
pub const SHARED_STATE_BYTES: usize = 4 + 2 + BUTTONS + MAX_SOLDIERS * ROW_BYTES;

/// The length of the whole state's canonical bytes: the shared state, then the seed.
#[cfg(test)]
const STATE_BYTES: usize = SHARED_STATE_BYTES + SEED_BYTES;

/// What the authority starts a match from. There is no `Debug`: the seed is never printed or logged (ADR 0004 part
/// 8).
pub struct MatchSetup {
    /// From whoever starts the match; the simulation draws none itself.
    pub seed: [u8; SEED_BYTES],
    /// The tick the match starts at: 0 in play, and near a counter's wrap in the tests of the wraps.
    pub start_tick: u32,
}

/// What the simulation reads of one player's command: the buttons held and a press counter per button, each
/// wrapping; the move held, as the wire carries it (a pair of `i8`, of length at most `MOVE_REACH`); and the aim in
/// whole centimetres, or `AimCm::NONE`. A command's view time joins when a system reads it.
///
/// The counters are running totals, so one command can bring many presses of one button at once: an honest client
/// after an outage of 255 ticks brings up to 255. A system that acts on presses applies at most what its own rule
/// allows per tick (one jump, one shot per cooldown) and never one action per press it finds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerCommand {
    pub buttons: u16,
    pub presses: [u8; BUTTONS],
    pub move_x: i8,
    pub move_y: i8,
    pub aim: AimCm,
}

impl PlayerCommand {
    /// Nothing held, no press yet, no move and no aim.
    pub const NONE: Self = Self {
        buttons: 0,
        presses: [0; BUTTONS],
        move_x: 0,
        move_y: 0,
        aim: AimCm::NONE,
    };
}

/// What the world keeps of the one player's commands: the newest one's buttons and press counters (engine-architect's
/// B1). His move and his aim live in his soldier's row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerRecord {
    pub buttons: u16,
    pub presses: [u8; BUTTONS],
}

impl PlayerRecord {
    /// Nothing held and no press yet: the record before the first command.
    pub const NONE: Self = Self {
        buttons: 0,
        presses: [0; BUTTONS],
    };
}

/// What reads the state field by field, in its one fixed order (ADR 0004 part 9). Every canonical writing of the
/// state goes through it: the bytes the hashes cover are one reader, and a dump that names each field (the replay
/// tools') is another, so a name and its bytes cannot drift apart. The snapshot and the block are no such writing:
/// they keep their own layouts.
pub trait StateSink {
    fn u16(&mut self, name: &'static str, value: u16);
    fn u32(&mut self, name: &'static str, value: u32);
    /// A float, by its bits (IEEE 754 binary32): the bytes never hold more or less than the value.
    fn f32(&mut self, name: &'static str, value: f32);
    /// The fields that follow, until the next call, are the soldier table's row `slot`. Nothing is written for it:
    /// a reader that names the fields may say which row they are.
    fn row(&mut self, slot: u16) {
        let _ = slot;
    }
    /// A field that is a run of bytes of a fixed length, each its own value (the press counters). A run whose length
    /// can vary is never handed over here: it is a count, then rows of a fixed width.
    fn bytes<const N: usize>(&mut self, name: &'static str, value: &[u8; N]);
    /// Everything before this call is the shared state; what follows is the authority's alone.
    fn authority(&mut self);
    /// Bytes that are never printed or logged: the match seed (ADR 0004 part 8). A reader may hash or count them.
    fn secret<const N: usize>(&mut self, name: &'static str, value: &[u8; N]);
}

/// Where the canonical bytes go: into a buffer, or straight into the hash, with no buffer between.
trait Bytes {
    fn put(&mut self, bytes: &[u8]);
}

impl Bytes for Vec<u8> {
    fn put(&mut self, bytes: &[u8]) {
        self.extend_from_slice(bytes);
    }
}

impl Bytes for Fnv1a64 {
    fn put(&mut self, bytes: &[u8]) {
        self.write(bytes);
    }
}

/// The canonical bytes: every integer at a fixed width and little-endian, no `usize`, nothing between the fields.
struct Canonical<'a, B>(&'a mut B);

impl<B: Bytes> StateSink for Canonical<'_, B> {
    // The path form on the integer's own type: the rule scan (tests/rules_scan.rs) reads text and cannot tell
    // `.to_le_bytes()` on a tick from the same call on a float, which part 7 does not allow.
    fn u16(&mut self, _: &'static str, value: u16) {
        self.0.put(&u16::to_le_bytes(value));
    }

    fn u32(&mut self, _: &'static str, value: u32) {
        self.0.put(&u32::to_le_bytes(value));
    }

    fn f32(&mut self, _: &'static str, value: f32) {
        self.0.put(&u32::to_le_bytes(f32::to_bits(value)));
    }

    fn bytes<const N: usize>(&mut self, _: &'static str, value: &[u8; N]) {
        self.0.put(value);
    }

    fn authority(&mut self) {}

    fn secret<const N: usize>(&mut self, _: &'static str, value: &[u8; N]) {
        self.0.put(value);
    }
}

/// One row of the soldier table to `sink`, every field in rust-reviewer's 11.2 order, 60 bytes: the pattern names each
/// field, so one added to `Soldier` later does not compile until it is handed over here.
fn visit_row(row: &Soldier, sink: &mut impl StateSink) {
    let Soldier {
        generation,
        flags,
        team,
        position,
        velocity,
        stamina,
        facing,
        held_move,
        aim_cm,
        pull,
    } = row;
    let point = |sink: &mut _, names: [&'static str; 3], value: &V3| {
        let [x, y, z] = value.to_array();
        StateSink::f32(sink, names[0], x);
        StateSink::f32(sink, names[1], y);
        StateSink::f32(sink, names[2], z);
    };
    let ([facing_x, facing_z], [move_x, move_y], [pull_x, pull_z]) = (*facing, *held_move, *pull);
    sink.u16("soldier.generation", *generation);
    sink.bytes("soldier.flags", &[*flags]);
    sink.bytes("soldier.team", &[*team]);
    point(
        sink,
        [
            "soldier.position.x",
            "soldier.position.y",
            "soldier.position.z",
        ],
        position,
    );
    point(
        sink,
        [
            "soldier.velocity.x",
            "soldier.velocity.y",
            "soldier.velocity.z",
        ],
        velocity,
    );
    sink.bytes("soldier.stamina", &u64::to_le_bytes(*stamina));
    sink.f32("soldier.facing.x", facing_x);
    sink.f32("soldier.facing.z", facing_z);
    // An `i8` and an `i16` go by their two's-complement bits.
    sink.bytes("soldier.held_move", &[move_x as u8, move_y as u8]);
    sink.u16("soldier.aim_cm.x", aim_cm.x as u16);
    sink.u16("soldier.aim_cm.y", aim_cm.y);
    sink.u16("soldier.aim_cm.z", aim_cm.z as u16);
    sink.f32("soldier.pull.x", pull_x);
    sink.f32("soldier.pull.z", pull_z);
}

/// The whole state of one match. There is no `Debug`: the seed is never printed or logged (ADR 0004 part 8). The
/// match's constants (its rules, its static world, its spawn table) are not state: the room owns them and lends them
/// to the step (engine-architect's block 7, F1).
pub struct World {
    tick: u32,
    /// The newest command's buttons and presses, for the one player there is so far.
    player: PlayerRecord,
    soldiers: Soldiers<MAX_SOLDIERS>,
    seed: [u8; SEED_BYTES],
}

impl World {
    /// A new match at the setup's start tick, with nothing held, and the player's soldier in his slot at the spawn
    /// table's first point (the table ranked by id; a build default until the barracks: netcode-architect's block 9.2,
    /// B4 (a)): facing as the spawn says, stamina full, no move held, no aim yet, and standing (the map's spawn is
    /// the point on the ground between his feet; tick 1's standing corrects a mismatch). A table with no point makes
    /// no soldier.
    #[must_use]
    pub fn new(setup: &MatchSetup, rules: &Rules, spawns: &[SpawnPoint]) -> Self {
        let mut soldiers = Soldiers::new();
        if let Some(first) = spawns.first()
            && let Some(id) = soldiers.spawn(PLAYER_SLOT, first.at, first.facing)
            && let Some(row) = soldiers.resolve_mut(id)
        {
            row.flags |= GROUNDED;
            row.team = PLAYER_TEAM;
            row.stamina = rules.stamina_full;
        }
        Self {
            tick: setup.start_tick,
            player: PlayerRecord::NONE,
            soldiers,
            seed: setup.seed,
        }
    }

    /// The number of ticks run so far, counted from the start tick. It is the simulation's only clock: sim time is
    /// `tick / TICK_HZ` seconds.
    #[must_use]
    pub const fn tick(&self) -> u32 {
        self.tick
    }

    /// The player's record: the newest command's buttons and presses.
    #[must_use]
    pub const fn player(&self) -> &PlayerRecord {
        &self.player
    }

    /// The soldier table.
    #[must_use]
    pub const fn soldiers(&self) -> &Soldiers<MAX_SOLDIERS> {
        &self.soldiers
    }

    /// Runs one tick of the authority's phases with the player's commands for it, the oldest first (rust-reviewer's
    /// 11.1, item 1): the newest one's buttons and presses become the record; each command's move becomes his
    /// soldier's held move, and each one with a real aim his aim (`AimCm::NONE` leaves it). With none all three hold,
    /// so a held button stays held. A slot with no soldier takes the record and touches no row.
    ///
    /// # Panics
    /// When the counter is at `u32::MAX`, which is 4.5 years at 30 Hz: it stops loudly and never wraps (ADR 0004 part
    /// 4: counters use explicit operations).
    pub fn authority_step(&mut self, commands: &[PlayerCommand]) {
        for command in commands {
            if let Some(row) = self.soldiers.occupied_mut(PLAYER_SLOT) {
                row.held_move = [command.move_x, command.move_y];
                if command.aim != AimCm::NONE {
                    row.aim_cm = command.aim;
                }
            }
        }
        if let Some(newest) = commands.last() {
            self.player = PlayerRecord {
                buttons: newest.buttons,
                presses: newest.presses,
            };
        }
        self.tick = self
            .tick
            .checked_add(1)
            .expect("the tick counter is at u32::MAX and cannot advance");
    }

    /// Hands the whole state to `sink`, field by field in its one fixed order (ADR 0004 part 9). The shared state
    /// comes first (the tick, then the player's buttons and the 16 press counters, then the soldier table's 48 rows),
    /// then `authority()`, then the authority's own part (the seed, as a secret; the random streams' positions join it
    /// when a system first draws). Each field that joins the state joins here or in `visit_shared`, in the same change
    /// (`tests/state_rule.rs` holds that from the source).
    pub fn visit(&self, sink: &mut impl StateSink) {
        // Every field is named here too: one added to the struct later does not compile until it is handed over
        // here, or left to `visit_shared` in plain sight.
        let Self {
            tick: _,
            player: _,
            soldiers: _,
            seed,
        } = self;
        self.visit_shared(sink);
        sink.authority();
        sink.secret("seed", seed);
    }

    /// Hands the shared state to `sink` and nothing of the authority's part: it never touches the seed. After the
    /// tick and the record come the 48 rows in slot order, each field of each row (an empty row as zeros), floats by
    /// their bits, the signed fields by their two's-complement bits (rust-reviewer's 11.2).
    pub fn visit_shared(&self, sink: &mut impl StateSink) {
        // Every field is named in this pattern: one added to the struct later does not compile until it is handed
        // over here, or left to `visit` in plain sight.
        let Self {
            tick,
            player: PlayerRecord { buttons, presses },
            soldiers,
            seed: _,
        } = self;
        sink.u32("tick", *tick);
        sink.u16("player.buttons", *buttons);
        sink.bytes("player.presses", presses);
        for (slot, row) in (0..=u16::MAX).zip(soldiers.rows()) {
            sink.row(slot);
            visit_row(row, sink);
        }
    }

    /// Writes the canonical bytes of the whole state into `out`, replacing what it held: the shared state's bytes,
    /// then the authority's, so the shared state is a prefix. For this file's tests only: nothing else is handed
    /// the seed's bytes.
    #[cfg(test)]
    pub fn write_state(&self, out: &mut Vec<u8>) {
        out.clear();
        out.reserve(STATE_BYTES);
        self.visit(&mut Canonical(out));
    }

    /// Writes the canonical bytes of the shared state into `out`, replacing what it held. No seed byte is ever
    /// written into `out`, so a caller that keeps or hands on the buffer keeps and hands on no seed.
    pub fn write_shared_state(&self, out: &mut Vec<u8>) {
        out.clear();
        out.reserve(SHARED_STATE_BYTES);
        self.visit_shared(&mut Canonical(out));
    }

    /// The state hash: FNV-1a 64 over the canonical bytes, each field fed straight into it. There is no buffer, so
    /// it takes no memory (counted in `tests/state_hash_alloc.rs`).
    #[must_use]
    pub fn state_hash(&self) -> u64 {
        let mut hash = Fnv1a64::start();
        self.visit(&mut Canonical(&mut hash));
        hash.finish()
    }

    /// The hash of the shared state only, which covers no seed byte (ADR 0004 part 9): the number a replay compares
    /// tick by tick across targets. It is never exchanged with a client: no state hash is (ADR 0001 part 6).
    #[must_use]
    pub fn shared_hash(&self) -> u64 {
        let mut hash = Fnv1a64::start();
        self.visit_shared(&mut Canonical(&mut hash));
        hash.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collision::StaticWorld;
    use crate::soldiers::{ALIVE, OCCUPIED, spawn_points};
    use content::map::{Bounds, MapDocument, Spawn};

    /// The seed 0, 1, ..., 31.
    fn counting_seed(first: u8) -> [u8; SEED_BYTES] {
        let mut seed = [0; SEED_BYTES];
        for (byte, value) in seed.iter_mut().zip(first..) {
            *byte = value;
        }
        seed
    }

    /// The test bundle's rules and spawn table (content's made-up test texts: one spawn, "a", at (0, 0, -5) facing
    /// 90; 8 s of sprint and 4 s of refill).
    fn constants() -> (Rules, Vec<SpawnPoint>) {
        let set = content::load(content::bundle::test_bundle().as_bytes())
            .expect("the test bundle loads");
        let rules = Rules::take(&set.movements()[0]).expect("its movement table plays");
        let spawns = spawn_points(set.map("test-scene").expect("its map"));
        (rules, spawns)
    }

    fn setup(first: u8, start_tick: u32) -> MatchSetup {
        MatchSetup {
            seed: counting_seed(first),
            start_tick,
        }
    }

    fn world(first: u8, start_tick: u32) -> World {
        let (rules, spawns) = constants();
        World::new(&setup(first, start_tick), &rules, &spawns)
    }

    /// A match on a map with no spawn: no soldier.
    fn empty_world(first: u8, start_tick: u32) -> World {
        let (rules, _) = constants();
        World::new(&setup(first, start_tick), &rules, &[])
    }

    /// Buttons 0x8001 and the counters 1, 2, ..., 16; no move and no aim.
    fn pressed() -> PlayerCommand {
        let mut presses = [0; BUTTONS];
        for (counter, value) in presses.iter_mut().zip(1..) {
            *counter = value;
        }
        PlayerCommand {
            buttons: 0x8001,
            presses,
            ..PlayerCommand::NONE
        }
    }

    /// `pressed`, holding the move (127, -3) and the aim (100, 50, -200) cm.
    fn moved() -> PlayerCommand {
        PlayerCommand {
            move_x: 127,
            move_y: -3,
            aim: AimCm {
                x: 100,
                y: 50,
                z: -200,
            },
            ..pressed()
        }
    }

    fn row(world: &World, slot: usize) -> Soldier {
        world.soldiers().rows()[slot]
    }

    #[test]
    fn a_new_world_is_at_its_start_tick_and_each_step_is_one_tick() {
        let mut at_zero = world(0, 0);
        assert_eq!(at_zero.tick(), 0);
        assert_eq!(*at_zero.player(), PlayerRecord::NONE);
        at_zero.authority_step(&[]);
        at_zero.authority_step(&[]);
        assert_eq!(at_zero.tick(), 2);
        // A match can start anywhere, for the tests of the counters' wraps.
        let mut late = world(0, 131_069);
        late.authority_step(&[]);
        assert_eq!(late.tick(), 131_070);
    }

    #[test]
    fn the_newest_command_of_a_tick_becomes_the_record_and_none_holds_it() {
        let mut world = world(0, 0);
        let older = PlayerCommand {
            buttons: 2,
            presses: [9; BUTTONS],
            ..PlayerCommand::NONE
        };
        let record = |command: PlayerCommand| PlayerRecord {
            buttons: command.buttons,
            presses: command.presses,
        };
        world.authority_step(&[older, pressed()]);
        assert_eq!(*world.player(), record(pressed()));
        // No command this tick: what was held stays held, and no counter moves.
        world.authority_step(&[]);
        assert_eq!(*world.player(), record(pressed()));
        world.authority_step(&[older]);
        assert_eq!(*world.player(), record(older));
        assert_eq!(world.tick(), 3);
    }

    /// W-1 (netcode-architect's block 9.2): a fresh world on the test bundle holds his soldier in slot 0 at the table's
    /// first point by bits, facing (1, 0) by bits, stamina full, no move, no aim, standing, the generation a first
    /// spawn gives; the other 47 rows are empty.
    #[test]
    fn a_new_world_holds_his_soldier_at_the_first_spawn_and_nothing_else() {
        let (rules, spawns) = constants();
        let world = World::new(&setup(0, 0), &rules, &spawns);
        let soldier = row(&world, usize::from(PLAYER_SLOT));
        let bits = |v: V3| v.to_array().map(f32::to_bits);
        assert_eq!(bits(soldier.position), bits(spawns[0].at));
        assert_eq!(bits(soldier.position), [0, 0, (-5.0_f32).to_bits()]);
        assert_eq!(soldier.facing.map(f32::to_bits), [1.0_f32.to_bits(), 0]);
        assert_eq!(soldier.flags, OCCUPIED | ALIVE | GROUNDED);
        assert_eq!((soldier.team, soldier.generation), (PLAYER_TEAM, 1));
        assert_eq!(soldier.stamina, rules.stamina_full);
        assert_eq!((soldier.held_move, soldier.aim_cm), ([0, 0], AimCm::NONE));
        assert_eq!(bits(soldier.velocity), [0, 0, 0]);
        assert_eq!(soldier.pull, [0.0, 0.0]);
        for slot in 1..MAX_SOLDIERS {
            assert_eq!(row(&world, slot), Soldier::EMPTY, "row {slot}");
        }
    }

    /// The fold (rust-reviewer's 11.1 item 1; netcode-architect's N-1, the world's half): each command's move goes
    /// into the row, the newest last; a real aim sets the row's aim, NONE leaves it, so [A, NONE] keeps A and [NONE]
    /// alone keeps the earlier one; a tick with no command keeps all three.
    #[test]
    fn the_fold_puts_each_move_and_each_real_aim_in_his_row() {
        let mut world = world(0, 0);
        let aim_a = moved().aim;
        world.authority_step(&[moved(), pressed()]);
        let soldier = row(&world, 0);
        assert_eq!(
            soldier.held_move,
            [0, 0],
            "the newest command's move is the one held"
        );
        assert_eq!(soldier.aim_cm, aim_a, "[A, NONE] keeps A");
        world.authority_step(&[pressed()]);
        assert_eq!(
            row(&world, 0).aim_cm,
            aim_a,
            "[NONE] alone keeps the earlier aim"
        );
        world.authority_step(&[moved()]);
        world.authority_step(&[]);
        let soldier = row(&world, 0);
        assert_eq!(
            (soldier.held_move, soldier.aim_cm),
            ([127, -3], aim_a),
            "no command keeps them"
        );
        let other = AimCm { x: -7, y: 0, z: 9 };
        world.authority_step(&[PlayerCommand {
            aim: other,
            ..moved()
        }]);
        assert_eq!(row(&world, 0).aim_cm, other);
    }

    /// W-4: a map with no spawn makes no soldier; 30 commands with a move and a real aim still give the record, and
    /// every row stays empty (the fold touches no row of an empty slot), with no panic.
    /// W-2's standing half (netcode-architect's 9.2): on a map built in code whose spawns are listed "c", "a", "b", his
    /// soldier stands at "a"; with "street_2" and "street_10", at "street_10": the ids are ranked by their bytes, not
    /// as numbers and not in the file's order.
    #[test]
    fn his_soldier_stands_at_the_first_spawn_by_id_whatever_the_file_s_order() {
        let (rules, _) = constants();
        let spawn = |id: &str, x: f64| Spawn {
            id: id.to_owned(),
            at: [x, 0.0, 1.0],
            facing_deg: 180.0,
        };
        let map = |spawns| MapDocument {
            id: "made".to_owned(),
            bounds: Bounds {
                min: [-10.0, 0.0, -10.0],
                max: [10.0, 5.0, 10.0],
            },
            spawns,
            shapes: Vec::new(),
            regions: Vec::new(),
        };
        let listed = [
            (
                vec![spawn("c", 3.0), spawn("a", 1.0), spawn("b", 2.0)],
                1.0_f32,
            ),
            (vec![spawn("street_2", 2.0), spawn("street_10", 10.0)], 10.0),
        ];
        for (spawns, x) in listed {
            let world = World::new(&setup(0, 0), &rules, &spawn_points(&map(spawns)));
            let his = row(&world, usize::from(PLAYER_SLOT));
            assert_eq!(his.position.to_array(), [x, 0.0, 1.0]);
            assert_eq!(his.facing, [0.0, -1.0]);
            assert_eq!(world.soldiers().iter().count(), 1);
        }
    }

    /// T-B (rust-reviewer's row 28): one bundle loaded and built twice gives the same match. The two loads' rules,
    /// spawn tables and static worlds are equal (the worlds by their tree's bytes and their bounds), and two worlds
    /// started from them and stepped with the same 300 commands give the same hashes at every tick.
    #[test]
    fn one_bundle_loaded_and_built_twice_gives_the_same_match() {
        let build = || {
            let set = content::load(content::bundle::test_bundle().as_bytes())
                .expect("the test bundle loads");
            let map = set.map("test-scene").expect("its map");
            let rules = Rules::take(&set.movements()[0]).expect("its movement table plays");
            let statics = StaticWorld::build(map).expect("the map builds");
            (rules, spawn_points(map), statics)
        };
        let ((rules_a, spawns_a, statics_a), (rules_b, spawns_b, statics_b)) = (build(), build());
        assert_eq!(rules_a, rules_b);
        assert_eq!(format!("{rules_a:?}"), format!("{rules_b:?}"));
        assert_eq!(spawns_a, spawns_b);
        let bits = |spawns: &[SpawnPoint]| {
            let point = |spawn: &SpawnPoint| {
                let facing = spawn.facing.map(f32::to_bits);
                (spawn.at.to_array().map(f32::to_bits), facing)
            };
            spawns.iter().map(point).collect::<Vec<_>>()
        };
        assert_eq!(bits(&spawns_a), bits(&spawns_b));
        assert_eq!(statics_a.tree_bytes(), statics_b.tree_bytes());
        assert_eq!(statics_a.bounds_cm(), statics_b.bounds_cm());
        let mut a = World::new(&setup(5, 0), &rules_a, &spawns_a);
        let mut b = World::new(&setup(5, 0), &rules_b, &spawns_b);
        assert_eq!(a.state_hash(), b.state_hash());
        let mut presses = [0u8; BUTTONS];
        for tick in 0..300u16 {
            let [low, _] = u16::to_le_bytes(tick);
            let button = usize::from(low) % BUTTONS;
            presses[button] = presses[button].wrapping_add(1);
            let aim = if tick % 3 == 0 {
                AimCm::NONE
            } else {
                AimCm {
                    x: i16::from(low) - 100,
                    y: tick,
                    z: 7,
                }
            };
            let command = PlayerCommand {
                buttons: tick,
                presses,
                move_x: (low % 120) as i8,
                move_y: -((low % 90) as i8),
                aim,
            };
            a.authority_step(&[command]);
            b.authority_step(&[command]);
            assert_eq!(
                (a.state_hash(), a.shared_hash()),
                (b.state_hash(), b.shared_hash()),
                "tick {tick}"
            );
        }
        assert_eq!(row(&a, 0), row(&b, 0));
    }

    #[test]
    fn a_map_with_no_spawn_makes_no_soldier_and_the_fold_touches_no_row() {
        let mut world = empty_world(0, 0);
        for _ in 0..30 {
            world.authority_step(&[moved()]);
        }
        assert_eq!(world.player().buttons, moved().buttons);
        assert_eq!(world.player().presses, moved().presses);
        for slot in 0..MAX_SOLDIERS {
            assert_eq!(row(&world, slot), Soldier::EMPTY, "row {slot}");
        }
        let mut bytes = Vec::new();
        world.write_shared_state(&mut bytes);
        assert!(bytes[22..].iter().all(|byte| *byte == 0));
    }

    /// S-1's spawn half (rust-reviewer's block 8): a spawn facing 90 is (1, 0) by bits, and stays so through 30 ticks
    /// with no command and 30 with NONE.
    #[test]
    fn the_spawn_facing_holds_its_bits_through_ticks_without_an_aim() {
        let mut world = world(0, 0);
        for step in 0..60 {
            let commands = [pressed()];
            world.authority_step(if step < 30 { &[] } else { &commands[..] });
            assert_eq!(
                row(&world, 0).facing.map(f32::to_bits),
                [1.0_f32.to_bits(), 0],
                "tick {step}"
            );
        }
    }

    #[test]
    fn state_bytes_have_a_fixed_order_and_width() {
        let mut world = world(0, 0);
        for _ in 0..0x0101 {
            world.authority_step(&[]);
        }
        world.authority_step(&[moved()]);
        // What was in the buffer before does not survive.
        let mut bytes = vec![0xEE; 100];
        world.write_state(&mut bytes);
        assert_eq!(bytes.len(), STATE_BYTES);
        assert_eq!((SHARED_STATE_BYTES, bytes.len()), (2_902, 2_934));
        // The tick 0x0000_0102, little-endian; the buttons 0x8001; the counters.
        assert_eq!(bytes[..4], [0x02, 0x01, 0x00, 0x00]);
        assert_eq!(bytes[4..6], [0x01, 0x80]);
        assert_eq!(bytes[6..22], pressed().presses);
        // Row 0, 60 bytes in 11.2's order: generation 1, flags 7, team 0, the feet (0, 0, -5), no speed, stamina,
        // facing (1, 0), the move (127, -3), the aim (100, 50, -200), no pull.
        let row = &bytes[22..82];
        assert_eq!(row[..4], [0x01, 0x00, 0x07, 0x00]);
        assert_eq!(row[4..16], [0, 0, 0, 0, 0, 0, 0, 0, 0x00, 0x00, 0xA0, 0xC0]);
        assert_eq!(row[16..28], [0; 12]);
        assert_eq!(row[28..36], u64::to_le_bytes(24_000 * 12_000));
        assert_eq!(row[36..44], [0x00, 0x00, 0x80, 0x3F, 0, 0, 0, 0]);
        assert_eq!(row[44..46], [127, 0xFD]);
        assert_eq!(row[46..52], [100, 0, 50, 0, 0x38, 0xFF]);
        assert_eq!(row[52..60], [0; 8]);
        // The other 47 rows are zeros; then the seed as it was given.
        assert!(bytes[82..SHARED_STATE_BYTES].iter().all(|byte| *byte == 0));
        assert_eq!(bytes[SHARED_STATE_BYTES..], counting_seed(0));
    }

    /// A reader that notes each field's name and width, each row's mark, and where the authority's part begins.
    #[derive(Default)]
    struct Names(Vec<(&'static str, usize)>);

    impl StateSink for Names {
        fn u16(&mut self, name: &'static str, _: u16) {
            self.0.push((name, 2));
        }

        fn u32(&mut self, name: &'static str, _: u32) {
            self.0.push((name, 4));
        }

        fn f32(&mut self, name: &'static str, _: f32) {
            self.0.push((name, 4));
        }

        fn row(&mut self, _: u16) {
            self.0.push(("row", 0));
        }

        fn bytes<const N: usize>(&mut self, name: &'static str, _: &[u8; N]) {
            self.0.push((name, N));
        }

        fn authority(&mut self) {
            self.0.push(("authority", 0));
        }

        fn secret<const N: usize>(&mut self, name: &'static str, _: &[u8; N]) {
            self.0.push((name, N));
        }
    }

    /// One row's fields, in 11.2's order, with their widths: 60 bytes.
    #[rustfmt::skip]
    const ROW: [(&str, usize); 19] = [
        ("row", 0), ("soldier.generation", 2), ("soldier.flags", 1), ("soldier.team", 1),
        ("soldier.position.x", 4), ("soldier.position.y", 4), ("soldier.position.z", 4),
        ("soldier.velocity.x", 4), ("soldier.velocity.y", 4), ("soldier.velocity.z", 4), ("soldier.stamina", 8),
        ("soldier.facing.x", 4), ("soldier.facing.z", 4), ("soldier.held_move", 2), ("soldier.aim_cm.x", 2),
        ("soldier.aim_cm.y", 2), ("soldier.aim_cm.z", 2), ("soldier.pull.x", 4), ("soldier.pull.z", 4),
    ];

    /// Row 25 (rust-reviewer's 11.7): every reader gets the same fields in the same order, each row's in 11.2's, and
    /// the widths are the canonical lengths: 2,902 shared bytes, then the seed.
    #[test]
    fn every_reader_of_the_state_gets_the_same_fields_in_the_same_order() {
        let world = world(0, 0);
        let mut names = Names::default();
        world.visit(&mut names);
        let mut fields = vec![("tick", 4), ("player.buttons", 2), ("player.presses", 16)];
        for _ in 0..MAX_SOLDIERS {
            fields.extend(ROW);
        }
        let shared = fields.len();
        fields.extend([("authority", 0), ("seed", 32)]);
        assert_eq!(names.0, fields);
        let width = |fields: &[(&str, usize)]| fields.iter().map(|field| field.1).sum::<usize>();
        assert_eq!(width(&ROW), ROW_BYTES);
        assert_eq!(width(&fields[..shared]), SHARED_STATE_BYTES);
        assert_eq!(width(&fields), STATE_BYTES);
        let mut only_shared = Names::default();
        world.visit_shared(&mut only_shared);
        assert_eq!(only_shared.0, fields[..shared]);
    }

    /// A reader of the shared state that stops the test when it is handed anything of the authority's part.
    struct SharedOnly;

    impl StateSink for SharedOnly {
        fn u16(&mut self, _: &'static str, _: u16) {}

        fn u32(&mut self, _: &'static str, _: u32) {}

        fn f32(&mut self, _: &'static str, _: f32) {}

        fn bytes<const N: usize>(&mut self, _: &'static str, _: &[u8; N]) {}

        fn authority(&mut self) {
            panic!("the shared state's reader was handed the authority's part");
        }

        fn secret<const N: usize>(&mut self, name: &'static str, _: &[u8; N]) {
            panic!("the shared state's reader was handed the secret {name}");
        }
    }

    /// The shared bytes are the whole state's first bytes, and writing them never puts the seed in the buffer. On a
    /// map with no soldier this seed's bytes, 0xA0 to 0xBF, are none of the shared bytes; with his soldier (whose
    /// floats hold any byte) the seed's 32 bytes are nowhere among them.
    #[test]
    fn the_shared_bytes_are_a_prefix_and_hold_no_seed_byte() {
        for mut world in [empty_world(0xA0, 7), world(0xA0, 7)] {
            world.authority_step(&[moved()]);
            world.visit_shared(&mut SharedOnly);
            let (mut whole, mut shared) = (Vec::new(), vec![0xEE; 100]);
            world.write_state(&mut whole);
            world.write_shared_state(&mut shared);
            assert_eq!(shared.len(), SHARED_STATE_BYTES);
            assert_eq!(shared, whole[..SHARED_STATE_BYTES]);
            assert_eq!(whole[SHARED_STATE_BYTES..], counting_seed(0xA0));
            // The hashes are fed field by field with no buffer: they are the function over exactly these bytes.
            let over = crate::state_hash::fnv1a_64;
            assert_eq!(
                (world.shared_hash(), world.state_hash()),
                (over(&shared), over(&whole))
            );
            assert!(
                !shared
                    .windows(SEED_BYTES)
                    .any(|window| window == counting_seed(0xA0))
            );
        }
        let mut empty = empty_world(0xA0, 7);
        empty.authority_step(&[moved()]);
        let mut shared = Vec::new();
        empty.write_shared_state(&mut shared);
        assert!(shared.iter().all(|byte| !(0xA0..=0xBF).contains(byte)));
    }

    /// Known answers from a second implementation (`.checkpoints/cs4c-soldier/world-known-answers-c4.py`, a script
    /// that builds the bytes from 11.2's table and first reproduces the answers this test held before the soldier
    /// table): they hold the byte order and the hash still, on every target the tests run on.
    #[test]
    fn state_hash_matches_known_answers() {
        let mut stepped = world(0, 0);
        assert_eq!(stepped.state_hash(), 0xfe5c_1b37_2be6_b04f);
        for _ in 0..3 {
            stepped.authority_step(&[]);
        }
        assert_eq!(stepped.state_hash(), 0xed75_6338_8a8f_e700);
        assert_eq!(world(1, 0).state_hash(), 0x5e87_f5ee_6b99_ae0f);
        assert_eq!(world(0, 131_069).state_hash(), 0x356d_7029_9b57_4922);
        assert_eq!(world(0, u32::MAX).state_hash(), 0x7514_161d_05d9_e31b);
        let mut played = world(0, 0);
        played.authority_step(&[]);
        played.authority_step(&[pressed()]);
        assert_eq!(played.state_hash(), 0x2d3a_5263_e54c_3740);
        assert_eq!(played.shared_hash(), 0x6ad4_99aa_4def_3e40);
        let mut moving = world(0, 0);
        moving.authority_step(&[moved()]);
        assert_eq!(moving.state_hash(), 0x1464_7681_b7aa_69fc);
        assert_eq!(empty_world(0, 0).state_hash(), 0x4492_32b7_23f1_4ddd);
    }

    /// The shared hash covers the tick, the player's record and the rows and no seed byte: two matches with different
    /// seeds agree on it, and differ on the whole state's hash.
    #[test]
    fn the_shared_hash_covers_no_seed_byte() {
        let (mut a, mut b) = (world(0, 0), world(1, 0));
        assert_eq!(a.shared_hash(), 0xfe9a_6ff7_b575_8ccf);
        for step in 0..50 {
            let commands = [moved()];
            let commands = if step % 2 == 0 { &commands[..] } else { &[] };
            a.authority_step(commands);
            b.authority_step(commands);
            assert_eq!(a.shared_hash(), b.shared_hash());
            assert_ne!(a.state_hash(), b.state_hash());
        }
        // The tick and each part of the record and the row count.
        let base = a.shared_hash();
        a.authority_step(&[]);
        assert_ne!(a.shared_hash(), base);
        let one = |command: PlayerCommand| {
            let mut world = world(0, 9);
            world.authority_step(&[command]);
            world.shared_hash()
        };
        let plain = one(PlayerCommand::NONE);
        let mut presses = [0; BUTTONS];
        presses[BUTTONS - 1] = 1;
        for (what, command) in [
            (
                "a held button",
                PlayerCommand {
                    buttons: 1,
                    ..PlayerCommand::NONE
                },
            ),
            (
                "a press",
                PlayerCommand {
                    presses,
                    ..PlayerCommand::NONE
                },
            ),
            (
                "a move",
                PlayerCommand {
                    move_y: 1,
                    ..PlayerCommand::NONE
                },
            ),
            (
                "an aim",
                PlayerCommand {
                    aim: AimCm { x: 0, y: 0, z: 1 },
                    ..PlayerCommand::NONE
                },
            ),
        ] {
            assert_ne!(one(command), plain, "{what}");
        }
    }

    #[test]
    fn the_same_seed_gives_the_same_hashes_and_every_field_counts() {
        let mut a = world(7, 0);
        let mut b = world(7, 0);
        let mut other_seed = world(8, 0);
        let mut seen = Vec::new();
        for _ in 0..100 {
            let hash = a.state_hash();
            assert_eq!(hash, b.state_hash());
            assert_ne!(hash, other_seed.state_hash());
            assert!(!seen.contains(&hash), "each tick has its own hash");
            seen.push(hash);
            a.authority_step(&[]);
            b.authority_step(&[]);
            other_seed.authority_step(&[]);
        }
    }

    #[test]
    #[should_panic(expected = "the tick counter is at u32::MAX")]
    fn the_tick_counter_stops_loudly_and_never_wraps() {
        world(0, u32::MAX).authority_step(&[]);
    }

    /// A world rebuilt from the shared bytes alone, field by field in `visit_shared`'s order, with the seed from the
    /// setup (netcode-architect's restore, T-F: engine-architect's block 7 item 5.17, no constant taken). A test's
    /// tool, never change-set 7's reset.
    fn read_shared(bytes: &[u8], setup: &MatchSetup) -> World {
        assert_eq!(bytes.len(), SHARED_STATE_BYTES);
        let take = |at: usize, n: usize| &bytes[at..at + n];
        let u16_at = |at: usize| u16::from_le_bytes([bytes[at], bytes[at + 1]]);
        let u32_at = |at: usize| u32::from_le_bytes(take(at, 4).try_into().expect("4 bytes"));
        let f32_at = |at: usize| f32::from_bits(u32_at(at));
        let mut presses = [0; BUTTONS];
        presses.copy_from_slice(take(6, BUTTONS));
        let mut rows = [Soldier::EMPTY; MAX_SOLDIERS];
        for (slot, row) in rows.iter_mut().enumerate() {
            let at = 22 + slot * ROW_BYTES;
            *row = Soldier {
                generation: u16_at(at),
                flags: bytes[at + 2],
                team: bytes[at + 3],
                position: V3::new(f32_at(at + 4), f32_at(at + 8), f32_at(at + 12)),
                velocity: V3::new(f32_at(at + 16), f32_at(at + 20), f32_at(at + 24)),
                stamina: u64::from_le_bytes(take(at + 28, 8).try_into().expect("8 bytes")),
                facing: [f32_at(at + 36), f32_at(at + 40)],
                held_move: [
                    i8::from_le_bytes([bytes[at + 44]]),
                    i8::from_le_bytes([bytes[at + 45]]),
                ],
                aim_cm: AimCm {
                    x: i16::from_le_bytes([bytes[at + 46], bytes[at + 47]]),
                    y: u16_at(at + 48),
                    z: i16::from_le_bytes([bytes[at + 50], bytes[at + 51]]),
                },
                pull: [f32_at(at + 52), f32_at(at + 56)],
            };
        }
        World {
            tick: u32_at(0),
            player: PlayerRecord {
                buttons: u16_at(4),
                presses,
            },
            soldiers: Soldiers::from_rows(rows),
            seed: setup.seed,
        }
    }

    /// T-F, C4's half (netcode-architect's restore; S-3 with NONE in the row): copy A runs 30 ticks holding the move
    /// (127, -3) and a real aim, then 30 with no command; copy B is rebuilt from A's shared bytes after tick 30 and runs
    /// the same 30. Their shared bytes are equal at every tick, and B's row holds the move and the aim: the held move
    /// and the held aim live in the state, not outside it. Run again with NONE as the only aim: the row keeps NONE.
    #[test]
    fn a_world_restored_from_its_shared_bytes_runs_on_as_the_original() {
        for aim in [moved().aim, AimCm::NONE] {
            let command = PlayerCommand { aim, ..moved() };
            let mut a = world(3, 0);
            for _ in 0..30 {
                a.authority_step(&[command]);
            }
            let mut bytes = Vec::new();
            a.write_shared_state(&mut bytes);
            let mut b = read_shared(&bytes, &setup(3, 0));
            assert_eq!(b.state_hash(), a.state_hash());
            let (mut bytes_a, mut bytes_b) = (Vec::new(), Vec::new());
            for tick in 31..=60 {
                a.authority_step(&[]);
                b.authority_step(&[]);
                a.write_shared_state(&mut bytes_a);
                b.write_shared_state(&mut bytes_b);
                assert_eq!(bytes_a, bytes_b, "tick {tick}");
            }
            assert_eq!(row(&b, 0), row(&a, 0));
            assert_eq!((row(&b, 0).held_move, row(&b, 0).aim_cm), ([127, -3], aim));
        }
    }
}
