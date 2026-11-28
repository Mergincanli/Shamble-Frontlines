//! The soldier table (ADR 0004 parts 1 and 2; PM-0004-15): a fixed array of rows, one per slot, so it never
//! allocates. A row holds one soldier's whole state, the part the shared state will carry for him (change-set 4c's
//! step 1). Every slot is reached through `get` and `get_mut`, so a slot from anywhere, a command's included, never
//! panics.

use content::map::{MapDocument, Spawn};

use crate::aim::AimCm;
use crate::collision::narrow;
use crate::fmath::facing_of;
use crate::ids::SoldierId;
use crate::vec::V3;

/// The soldiers one match holds: the block's 48 rows (the build plan's 6.2).
pub const MAX_SOLDIERS: usize = 48;

/// A row's flag: a soldier holds the slot.
pub const OCCUPIED: u8 = 1 << 0;
/// A row's flag: he is alive.
pub const ALIVE: u8 = 1 << 1;
/// A row's flag: he stands on something.
pub const GROUNDED: u8 = 1 << 2;
/// A row's flag: he sprints.
pub const SPRINTING: u8 = 1 << 3;
/// A row's flag: his stamina ran out, and he may not sprint until it is back at its share.
pub const SPRINT_LOCKED: u8 = 1 << 4;

/// One soldier's row, in the order the shared state will write it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Soldier {
    /// The slot's generation: raised by one at each spawn into the slot and kept at the despawn, so an id given out
    /// before names a generation that is gone. It wraps after 65,536 spawns into one slot, and an id that old then
    /// resolves again (accepted: a respawn every 3 s for 54 hours).
    pub generation: u16,
    /// The flag bits above.
    pub flags: u8,
    pub team: u8,
    /// The feet, in metres.
    pub position: V3,
    /// In metres a second: the eased move across the map and the vertical speed.
    pub velocity: V3,
    /// In whole units.
    pub stamina: u64,
    /// The direction he faces across the map, (x, z), of length 1.
    pub facing: [f32; 2],
    /// The move the last command held, as the command carries it.
    pub held_move: [i8; 2],
    /// The aim the last command with a real aim held: `AimCm::NONE` until one does.
    pub aim_cm: AimCm,
    /// What is left of the pull onto a step, across the map: (0, 0) when none runs.
    pub pull: [f32; 2],
}

impl Soldier {
    /// An empty row: no soldier, and every field zero.
    pub const EMPTY: Self = Self {
        generation: 0,
        flags: 0,
        team: 0,
        position: V3::new(0.0, 0.0, 0.0),
        velocity: V3::new(0.0, 0.0, 0.0),
        stamina: 0,
        facing: [0.0; 2],
        held_move: [0; 2],
        aim_cm: AimCm { x: 0, y: 0, z: 0 },
        pull: [0.0; 2],
    };
}

/// Whether `row` holds the soldier `id` names.
const fn holds(row: &Soldier, id: SoldierId) -> bool {
    row.flags & OCCUPIED != 0 && row.generation == id.generation
}

/// The table: `N` rows, the row at index `i` being slot `i`. The match's is `Soldiers<MAX_SOLDIERS>`; a test may
/// build another (PM-0004-15's pool of 64).
#[derive(Debug, Clone, PartialEq)]
pub struct Soldiers<const N: usize> {
    slots: [Soldier; N],
}

impl<const N: usize> Soldiers<N> {
    /// Every slot empty.
    #[must_use]
    pub const fn new() -> Self {
        const {
            assert!(
                N < u16::MAX as usize,
                "a slot is a u16, and u16::MAX names none"
            )
        };
        Self {
            slots: [Soldier::EMPTY; N],
        }
    }

    /// Spawns a soldier into `slot` at `at`, facing `facing`: the slot's generation goes up by one, wrapping; the row
    /// is occupied and alive, and its other fields take their spawn values (no speed, no stamina, team 0, no move
    /// held, no aim yet, no pull), for the caller to set what its rules give. None when `slot` is past the table or
    /// a soldier holds it.
    #[must_use]
    pub fn spawn(&mut self, slot: u16, at: V3, facing: [f32; 2]) -> Option<SoldierId> {
        let row = self.slots.get_mut(usize::from(slot))?;
        if row.flags & OCCUPIED != 0 {
            return None;
        }
        let generation = row.generation.wrapping_add(1);
        *row = Soldier {
            generation,
            flags: OCCUPIED | ALIVE,
            position: at,
            facing,
            aim_cm: AimCm::NONE,
            ..Soldier::EMPTY
        };
        Some(SoldierId { slot, generation })
    }

    /// Despawns the soldier `id` names: his row is emptied but for its generation, which stays, so `id` and every
    /// copy of it resolve to nothing. False when `id` names no soldier (its slot past the table or empty, or a later
    /// generation there).
    pub fn despawn(&mut self, id: SoldierId) -> bool {
        let Some(row) = self.resolve_mut(id) else {
            return false;
        };
        *row = Soldier {
            generation: row.generation,
            ..Soldier::EMPTY
        };
        true
    }

    /// The soldier `id` names, or none.
    #[must_use]
    pub fn resolve(&self, id: SoldierId) -> Option<&Soldier> {
        self.slots
            .get(usize::from(id.slot))
            .filter(|row| holds(row, id))
    }

    /// The soldier `id` names, to change, or none.
    pub fn resolve_mut(&mut self, id: SoldierId) -> Option<&mut Soldier> {
        self.slots
            .get_mut(usize::from(id.slot))
            .filter(|row| holds(row, id))
    }

    /// The soldier in `slot`, to change, or none (an empty slot, or one past the table): the one player's fold reaches
    /// his row by his slot.
    pub fn occupied_mut(&mut self, slot: u16) -> Option<&mut Soldier> {
        self.slots
            .get_mut(usize::from(slot))
            .filter(|row| row.flags & OCCUPIED != 0)
    }

    /// Every row in slot order, an empty one too: the rows the shared state holds.
    #[must_use]
    pub const fn rows(&self) -> &[Soldier; N] {
        &self.slots
    }

    /// A table of these rows, as a restore from the shared bytes rebuilds one (world.rs's restore test, T-F).
    #[cfg(test)]
    pub(crate) const fn from_rows(slots: [Soldier; N]) -> Self {
        Self { slots }
    }

    /// The soldiers there are, in slot order, each with his id.
    pub fn iter(&self) -> impl Iterator<Item = (SoldierId, &Soldier)> {
        self.slots
            .iter()
            .zip(0..=u16::MAX)
            .filter(|(row, _)| row.flags & OCCUPIED != 0)
            .map(|(row, slot)| {
                let id = SoldierId {
                    slot,
                    generation: row.generation,
                };
                (id, row)
            })
    }
}

impl<const N: usize> Default for Soldiers<N> {
    fn default() -> Self {
        Self::new()
    }
}

/// Where a soldier can start: a map's spawn as the simulation holds it, its point narrowed once (as a shape's corners
/// are) and its heading as a direction (x, z).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpawnPoint {
    pub at: V3,
    pub facing: [f32; 2],
}

/// A map's spawns as the match's spawn table, built once when a match starts (allocating, never in a tick): ranked by
/// id in byte order, as the shapes are (`collision.rs`), so "street_10" comes before "street_2". The first is where
/// the first soldier stands: a build default until the barracks, not a rule of his (game-designer's N37).
#[must_use]
pub fn spawn_points(map: &MapDocument) -> Vec<SpawnPoint> {
    let mut spawns: Vec<&Spawn> = map.spawns.iter().collect();
    spawns.sort_by(|a, b| a.id.as_bytes().cmp(b.id.as_bytes()));
    spawns
        .into_iter()
        .map(|spawn| {
            let [x, y, z] = narrow(spawn.at);
            SpawnPoint {
                at: V3::new(x, y, z),
                facing: facing_of(spawn.facing_deg),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ChaCha8, SEED_BYTES};

    /// Where a test spawns the soldier of `slot`: a point no other slot's soldier has, so a row read back names the
    /// slot it was spawned into.
    fn spot(slot: u16) -> V3 {
        V3::new(f32::from(slot), 0.5, -2.0)
    }

    const EAST: [f32; 2] = [1.0, 0.0];

    /// PM-0004-15: 10,000 seeded cases, each of 1 to 1,000 spawns and despawns drawn from the simulation's own
    /// generator on a table of 64 slots (the mark's pool, smaller than a case, so slots are reused). After every case
    /// each id despawned resolves to nothing, each id still held resolves to the soldier spawned with it, and the table
    /// holds exactly those. A spawn is refused only where a soldier holds the slot or the slot is past the table, and
    /// a despawn succeeds once per id.
    #[test]
    fn a_stale_id_never_resolves() {
        let mut stream = ChaCha8::new(&[0x15; SEED_BYTES], 0x0004_0015);
        let (mut held, mut gone) = (Vec::new(), Vec::new());
        let mut failures = 0_u32;
        for _ in 0..10_000 {
            let mut table = Soldiers::<64>::new();
            held.clear();
            gone.clear();
            for _ in 0..=stream.next_u32() % 1_000 {
                let draw = stream.next_u32();
                if draw.is_multiple_of(2) || held.is_empty() {
                    // Slots 64 to 69 lie past the table.
                    let slot = u16::try_from((draw / 2) % 70).expect("a slot under 70");
                    match table.spawn(slot, spot(slot), EAST) {
                        Some(id) => held.push(id),
                        None => {
                            assert!(slot >= 64 || held.iter().any(|id: &SoldierId| id.slot == slot))
                        }
                    }
                } else {
                    let index =
                        usize::try_from(draw / 2).expect("a u32 fits a usize here") % held.len();
                    let id = held.swap_remove(index);
                    assert!(table.despawn(id), "a held id despawns");
                    assert!(!table.despawn(id), "an id despawns once");
                    gone.push(id);
                }
            }
            let stale = gone
                .iter()
                .filter(|id| table.resolve(**id).is_some())
                .count();
            let lost = held
                .iter()
                .filter(|id| {
                    table.resolve(**id).map(|soldier| soldier.position) != Some(spot(id.slot))
                })
                .count();
            if stale + lost > 0 || table.iter().count() != held.len() {
                failures += 1;
            }
        }
        assert_eq!(failures, 0);
    }

    #[test]
    fn a_spawn_is_refused_on_a_held_slot_and_past_the_table() {
        let mut table = Soldiers::<4>::new();
        let first = table.spawn(2, spot(2), EAST).expect("an empty slot");
        assert_eq!(
            first,
            SoldierId {
                slot: 2,
                generation: 1
            }
        );
        assert_eq!(table.spawn(2, spot(2), EAST), None);
        assert_eq!(table.spawn(4, spot(4), EAST), None);
        assert_eq!(table.spawn(u16::MAX, spot(0), EAST), None);
        assert_eq!(table.resolve(SoldierId::NONE), None);
        let soldier = table.resolve(first).expect("held");
        assert_eq!(soldier.flags, OCCUPIED | ALIVE);
        assert_eq!((soldier.position, soldier.facing), (spot(2), EAST));
        assert_eq!(soldier.aim_cm, AimCm::NONE);
        assert_eq!(
            (soldier.stamina, soldier.team, soldier.held_move),
            (0, 0, [0, 0])
        );
    }

    #[test]
    fn a_despawn_empties_the_row_but_keeps_its_generation() {
        let mut table = Soldiers::<2>::new();
        let first = table.spawn(1, spot(1), EAST).expect("an empty slot");
        table.resolve_mut(first).expect("held").stamina = 7;
        assert!(table.despawn(first));
        assert_eq!(table.resolve(first), None);
        let again = table.spawn(1, spot(1), EAST).expect("empty again");
        assert_eq!(again.generation, 2);
        assert_eq!(
            table.resolve(first),
            None,
            "the first id names a generation that is gone"
        );
        assert_eq!(table.resolve(again).map(|soldier| soldier.stamina), Some(0));
    }

    #[test]
    fn the_soldiers_come_in_slot_order() {
        let mut table = Soldiers::<8>::new();
        for slot in [5, 1, 7, 3] {
            let _ = table.spawn(slot, spot(slot), EAST);
        }
        let order: Vec<u16> = table.iter().map(|(id, _)| id.slot).collect();
        assert_eq!(order, [1, 3, 5, 7]);
        assert_eq!(Soldiers::<8>::default(), Soldiers::<8>::new());
    }

    /// The generation's wrap, as `Soldier::generation` states it: after 65,536 spawns into one slot the next spawn
    /// gives the first spawn's id again, and that old id resolves (ABA, accepted).
    #[test]
    fn a_slot_spawned_into_65536_times_gives_its_first_id_again() {
        let mut table = Soldiers::<1>::new();
        let first = table.spawn(0, spot(0), EAST).expect("an empty slot");
        assert!(table.despawn(first));
        for _ in 1..65_536 {
            let id = table.spawn(0, spot(0), EAST).expect("an empty slot");
            assert!(table.despawn(id));
        }
        let again = table.spawn(0, spot(0), EAST).expect("an empty slot");
        assert_eq!(again, first);
        assert!(table.resolve(first).is_some());
    }

    /// A map built in code with the spawns it is given, in that order (it never passed the loader).
    fn map_with(spawns: &[(&str, [f64; 3], f64)]) -> MapDocument {
        MapDocument {
            id: "spawns".to_owned(),
            bounds: content::map::Bounds {
                min: [-50.0, 0.0, -50.0],
                max: [50.0, 20.0, 50.0],
            },
            spawns: spawns
                .iter()
                .map(|(id, at, facing_deg)| Spawn {
                    id: (*id).to_owned(),
                    at: *at,
                    facing_deg: *facing_deg,
                })
                .collect(),
            shapes: Vec::new(),
            regions: Vec::new(),
        }
    }

    /// The spawn table is ranked by id in byte order, whatever the file's order (W-2's order half, block 15): "c",
    /// "a", "b" give "a" first; "street_10" comes before "street_2" (no number-aware order). Each point is narrowed as
    /// a shape's corner is (a minus zero loses its sign), and each heading is `facing_of`'s.
    #[test]
    fn the_spawn_table_is_ranked_by_id_in_byte_order() {
        let ids = |map: &MapDocument| -> Vec<[u32; 3]> {
            spawn_points(map)
                .iter()
                .map(|point| point.at.to_array().map(f32::to_bits))
                .collect()
        };
        let map = map_with(&[
            ("c", [3.0, 0.0, 0.0], 0.0),
            ("a", [1.0, 0.0, 0.0], 90.0),
            ("b", [2.0, 0.0, 0.0], 180.0),
        ]);
        let order: Vec<f32> = spawn_points(&map)
            .iter()
            .map(|point| point.at.to_array()[0])
            .collect();
        assert_eq!(order, [1.0, 2.0, 3.0]);
        assert_eq!(
            spawn_points(&map)
                .iter()
                .map(|point| point.facing)
                .collect::<Vec<_>>(),
            [facing_of(90.0), facing_of(180.0), facing_of(0.0)]
        );
        let streets = map_with(&[
            ("street_2", [2.0, 0.0, 0.0], 0.0),
            ("street_10", [10.0, 0.0, 0.0], 0.0),
        ]);
        let first = spawn_points(&streets)[0].at.to_array()[0];
        assert_eq!(first, 10.0);
        // A point is narrowed once, and a minus zero (which only a map built in code can hold) loses its sign.
        let signed = map_with(&[("z", [-0.0, 0.1, -0.0], 0.0)]);
        assert_eq!(ids(&signed), [[0, 0.1_f32.to_bits(), 0]]);
        assert!(spawn_points(&map_with(&[])).is_empty());
    }
}
