//! The scenes, as data (u4-build-plan.md §4.13; ADR 0001:1470-1482): SS the stress scene, ES R2's estimate scene, CO
//! the correction course and TC the target course. Each is recorded in a run's header by its id.

use crate::TICK_HZ;

/// S1's CPU stand-ins as data (ADR 0004's S1; PM-0001-2: "384 effect rows with the effect step's 1 ms ceiling burned
/// per tick, and the remainder topped up to 2.80 ms at 30 Hz and 0.30 ms at 60 Hz"). The sim has no clock, so it burns
/// nothing itself: the server's tick loop reads these and burns them (U6). Off on L1 (u4-build-plan.md §4.13, Q26),
/// recorded in `params.stand_ins`; the effect rows are state and stay in every scene that has them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StandIns {
    /// The effect step's ceiling, burned each tick, in ns (1 ms in the stress scene).
    pub burn_ns: u32,
    /// The rest of the sim step's share, topped up to each tick, in ns (2.80 ms at 30 Hz, 0.30 ms at 60 Hz).
    pub top_up_ns: u32,
}

impl StandIns {
    /// No CPU stand-in: what L1 runs with, and every scene but the stress scene.
    pub const OFF: StandIns = StandIns {
        burn_ns: 0,
        top_up_ns: 0,
    };

    /// S1's numbers at the build's tick rate.
    pub const fn s1() -> StandIns {
        StandIns {
            burn_ns: 1_000_000,
            top_up_ns: top_up_ns_at(TICK_HZ),
        }
    }
}

/// S1's top-up at `tick_hz`: 2.80 ms at 30 Hz and 0.30 ms at 60 Hz, the two rates S1 states; 0 at any other rate.
const fn top_up_ns_at(tick_hz: u32) -> u32 {
    match tick_hz {
        30 => 2_800_000,
        60 => 300_000,
        _ => 0,
    }
}

/// The stand-in weapon (one weapon, id 0), as plain data: what a run's header records of it with the soldiers'
/// health ([`Scene::weapon`], [`Scene::health`]; the advisor's W1-U4D TUNE-1).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Weapon {
    /// The interval between automatic shots, µs: the soldier's starting `Attribute::FireInterval`.
    pub interval_us: u32,
    /// Health points a hit takes.
    pub damage: u16,
    /// How far a shot reaches, cm.
    pub reach_cm: u32,
    /// Rounds per magazine.
    pub magazine: u16,
    /// A reload's length, ms.
    pub reload_ms: u32,
    /// The spread's half width, in millionths of the distance to the aim point (micro-radians).
    pub spread_urad: u32,
}

/// The stress scene's weapon: 700 shots a second with every soldier firing (48 × 1,000,000 / 68,571 µs = 700.0), a
/// hit taking 1 of 1,000 health, and a magazine that covers an S30 plus 5 minutes at one soldier's rate.
const SS_WEAPON: Weapon = Weapon {
    interval_us: 68_571,
    damage: 1,
    reach_cm: 6_000,
    magazine: 32_000,
    reload_ms: 2_000,
    spread_urad: 5_000,
};
/// The estimate scene's weapon: 1.5 shots per player per second (72 a second), the stress weapon's magazine.
const ES_WEAPON: Weapon = Weapon {
    interval_us: 666_667,
    damage: 10,
    ..SS_WEAPON
};
/// ES's recording aims among the other teams on one command in this many ([`Scene`]'s `aim_one_in`; the advisor's
/// W1-U4D T-M1). Measured over ES's 9,000-tick recording at 1 in 11: 9.85, 10.26 and 9.67 gameplay events a second for
/// seeds 2, 1 and 3 (shot hits about 4.4, blast hits about 5.3, kills about 0.2); 1 in 10 gives 10.39 for seed 2.
const ES_AIM_ONE_IN: u32 = 11;
/// An S30 (a 30-minute run) and 5 minutes more, µs.
const S30_PLUS_5_US: u64 = 35 * 60 * 1_000_000;
// SS and ES script no reloads (TUNE-1), so a soldier firing at its weapon's interval must not run dry inside an S30
// (the advisor's W1-U4D N-M1): magazine × interval covers 35 minutes. SS: 35 × 60 × 700 / 48 = 30,625 rounds at
// 68,571 µs, and its magazine holds 32,000; ES: 3,150 rounds at 666,667 µs.
const _: () = assert!(
    SS_WEAPON.magazine as u64 * SS_WEAPON.interval_us as u64 >= S30_PLUS_5_US
        && ES_WEAPON.magazine as u64 * ES_WEAPON.interval_us as u64 >= S30_PLUS_5_US
);

/// One scene: what a world starts from and how it runs (u4-build-plan.md §4.13).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Scene {
    id: u8,
    name: &'static str,
    pub(crate) slots: u8,
    pub(crate) teams: u8,
    /// TC's static targets: the slots from this one on are thin plates that never move or die (§4.13, Q17).
    pub(crate) plates_from: u8,
    /// Big vehicles, and how many of them are very large units of `LARGE_PARTS` parts (the rest have `SMALL_PARTS`).
    pub(crate) vehicles: u8,
    pub(crate) large_vehicles: u8,
    /// Stand-in projectiles the sim keeps live (the stress scene's 300).
    pub(crate) live_projectiles: u16,
    /// Stand-in projectiles spawned per second, in thousandths (ES's 0.44 per player per second: 21,120).
    pub(crate) spawn_millihertz: u32,
    /// The stand-in projectiles' fuse range, ms.
    pub(crate) fuse_ms: [u32; 2],
    /// A thrown grenade's fuse, ms: 2 s, and ES's 1.515 s (every ES projectile fuses after 1.515 s: ADR 0001:1478).
    pub(crate) grenade_fuse_ms: u32,
    /// Its synthetic recordings aim among the other teams' soldiers on one command in this many (and on every command
    /// that throws), and into the air above the shooter on the rest, so those shots rise almost straight up and meet
    /// only a soldier whose box holds the shooter's chest (soldiers pass through each other): 1 in every scene but ES,
    /// where the share sets its 10 gameplay events a second (ADR 0001:1478; the advisor's W1-U4D T-M1).
    pub(crate) aim_one_in: u32,
    /// The projectile pool's fixed capacity (stand-ins and grenades).
    pub(crate) projectile_capacity: u16,
    /// Effect rows per soldier (S1's 8 in the stress scene).
    pub(crate) effect_rows: u8,
    pub(crate) weapon: Weapon,
    /// The soldiers' starting walking speed, cm/s.
    pub(crate) move_speed: u32,
    pub(crate) health: u16,
    /// How long a dead soldier waits to respawn, ms.
    pub(crate) respawn_ms: u32,
    /// Each team's home in the centre field, cm: its soldiers start and respawn on a grid round it.
    pub(crate) homes: [[i32; 2]; 3],
    stand_ins: StandIns,
}

/// Parts of a very large unit (the stress scene's 4 have at least 20: PM-0001-2).
pub(crate) const LARGE_PARTS: usize = 20;
/// Parts of any other big vehicle.
pub(crate) const SMALL_PARTS: usize = 4;

/// The stress scene's and the estimate scene's three team homes, 10 m out from the centre, 120 degrees apart.
const CLUSTER: [[i32; 2]; 3] = [[1_000, 0], [-500, 866], [-500, -866]];

impl Scene {
    /// SS, the stress scene (u4-build-plan.md §4.13; net F113; PM-0001-2): 48 soldiers in 3 × 16 in one cluster in the
    /// centre field; 15 big vehicles, the 4 of 20 parts looping inside the fight and the other 11 elsewhere; 300 live
    /// stand-in projectiles as trajectory records, started inside the fight; 384 effect rows (8 per soldier); the
    /// doors' one portal change a second; the wall stand-in; the weapon's interval set for 700 shots per second with
    /// every soldier firing (48 × 30 ticks a second × 1,000,000 / (68,571 µs × 30) = 700.0), a hit taking 1 of 1,000
    /// health so that deaths, which pause a soldier's fire for the 1 s respawn, stay rare (D-6 measures the mean with
    /// them counted), and a magazine of 32,000 rounds that covers an S30 plus 5 minutes at one soldier's rate (30,625
    /// shots), so its recordings script no reload (the advisor's W1-U4D TUNE-1 and N-M1). S1's CPU stand-ins at the
    /// build's rate.
    pub fn stress() -> Scene {
        Scene {
            id: 1,
            name: "SS",
            slots: 48,
            teams: 3,
            plates_from: 48,
            vehicles: 15,
            large_vehicles: 4,
            live_projectiles: 300,
            spawn_millihertz: 0,
            fuse_ms: [2_000, 6_000],
            grenade_fuse_ms: 2_000,
            aim_one_in: 1,
            projectile_capacity: 320,
            effect_rows: 8,
            weapon: SS_WEAPON,
            move_speed: 600,
            health: 1_000,
            respawn_ms: 1_000,
            homes: CLUSTER,
            stand_ins: StandIns::s1(),
        }
    }

    /// ES, R2's estimate scene (ADR 0001:1474-1479): 48 soldiers and 15 big vehicles in view, 1.5 shots per player per
    /// second (the weapon's interval 666,667 µs), 32 live projectiles at 0.44 spawns per player per second (21.12 a
    /// second, each fusing after 1.515 s: 21.12 × 1.515 = 32.0; a thrown grenade is one of them, and the stand-ins
    /// fill the rest), 10 gameplay events (hits and kills) a second from its recording's aim (one command in 11 aims
    /// among the other teams), no effect rows (the advisor's W1-U4D T-M1).
    pub fn estimate() -> Scene {
        Scene {
            id: 2,
            name: "ES",
            large_vehicles: 0,
            live_projectiles: 0,
            spawn_millihertz: 21_120,
            fuse_ms: [1_515, 1_515],
            grenade_fuse_ms: 1_515,
            aim_one_in: ES_AIM_ONE_IN,
            projectile_capacity: 64,
            effect_rows: 0,
            weapon: ES_WEAPON,
            stand_ins: StandIns::OFF,
            ..Scene::stress()
        }
    }

    /// CO, the correction course (R2-SYNC-REC3 mark 1): one soldier and no other player, among the greybox's doors.
    pub fn course() -> Scene {
        Scene {
            id: 3,
            name: "CO",
            slots: 1,
            teams: 1,
            plates_from: 1,
            vehicles: 0,
            large_vehicles: 0,
            live_projectiles: 0,
            spawn_millihertz: 0,
            fuse_ms: [2_000, 2_000],
            grenade_fuse_ms: 2_000,
            aim_one_in: 1,
            projectile_capacity: 16,
            effect_rows: 0,
            weapon: Weapon {
                interval_us: 100_000,
                damage: 10,
                reach_cm: 6_000,
                magazine: 30,
                reload_ms: 2_000,
                spread_urad: 5_000,
            },
            move_speed: 600,
            health: 100,
            respawn_ms: 1_000,
            homes: [[0, -1_500], [0, -1_500], [0, -1_500]],
            stand_ins: StandIns::OFF,
        }
    }

    /// TC, the target course (ADR 0001:1480-1482; Q17): the shooter (slot 0), the target soldier (slot 1, sprinting at
    /// 9 m/s: 600 cm/s × 1.5), and three static targets (slots 2 to 4): thin plates 2 cm thick, on the ground, at 2.5 m
    /// and at 4 m, which never move or die.
    pub fn targets() -> Scene {
        Scene {
            id: 4,
            name: "TC",
            slots: 5,
            teams: 2,
            plates_from: 2,
            homes: [[-6_000, -1_000], [-6_000, 1_000], [-6_000, 1_000]],
            ..Scene::course()
        }
    }

    /// The scene of a header's id: 1 SS, 2 ES, 3 CO, 4 TC.
    pub fn by_id(id: u8) -> Option<Scene> {
        [
            Scene::stress(),
            Scene::estimate(),
            Scene::course(),
            Scene::targets(),
        ]
        .into_iter()
        .find(|s| s.id == id)
    }

    /// Its id in a recording's header.
    pub fn id(&self) -> u8 {
        self.id
    }

    /// Its name in a run's header: SS, ES, CO or TC.
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Its soldier slots, 0 to `slots() - 1`.
    pub fn slots(&self) -> u8 {
        self.slots
    }

    /// The team of `slot`, or `None` for a slot the scene does not have.
    pub fn team(&self, slot: u8) -> Option<u8> {
        if slot >= self.slots {
            None
        } else if self.plates_from < self.slots {
            // TC: the shooter against the rest.
            Some(u8::from(slot != 0))
        } else {
            Some(slot / (self.slots / self.teams))
        }
    }

    /// S1's CPU stand-ins this scene runs with.
    pub fn stand_ins(&self) -> StandIns {
        self.stand_ins
    }

    /// Its weapon (interval, damage, reach, magazine, reload, spread), which a run's header records (TUNE-1).
    pub fn weapon(&self) -> Weapon {
        self.weapon
    }

    /// Its soldiers' health, which a run's header records beside the weapon (TUNE-1).
    pub fn health(&self) -> u16 {
        self.health
    }

    /// The same scene with other CPU stand-ins (`StandIns::OFF` on L1: §4.13). The state is the same either way.
    pub fn with_stand_ins(self, stand_ins: StandIns) -> Scene {
        Scene { stand_ins, ..self }
    }

    /// Whether `slot` is one of TC's plates.
    pub(crate) fn is_plate(&self, slot: u8) -> bool {
        slot >= self.plates_from && slot < self.slots
    }

    /// Where a plate stands: TC's three, 3 m apart, 30 m north of the shooter, their bottoms at 0, 2.5 m and 4 m.
    pub(crate) fn plate_position(&self, slot: u8) -> [i32; 3] {
        let k = i32::from(slot - self.plates_from);
        let base = [0, 250, 400];
        [-6_300 + 300 * k, 2_000, base[(k as usize).min(2)]]
    }
}
