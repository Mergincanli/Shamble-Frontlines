//! npt-bots: the netcode proof test's headless clients (plan E12-E14): 30 inputs per second, the brawl, the
//! target-course shooter with a page-like frame loop, the cheats, the hostile clients, and the load orchestrator's
//! calibration and roll-out across the CAX21s. Built in units U4 and U6.
//!
//! U4r (u4-build-plan.md §5 U4r; u4r-brief.md 2.2) builds the library's [`Behaviour`], clock-injected and
//! transport-free (U4-SERVER): what one client's page would give its endpoint, from a seed, the client's slot and what
//! the host tells it, never from a clock or a socket. Two kinds: the brawl (the stress and estimate scenes' 48 soldiers
//! fighting in 3 x 16, every one holding fire, so the scene's weapon interval sets the shot rate PM-0001-2 checks:
//! §4.13) and the open-loop scripts (commands from the slot, the seed and the count of commands alone, standing in for
//! the cheats' command patterns in U4r's contract test: U4R-Q4 (a)). The target-course shooter is U4h's, the cheats
//! and the never-acker U4f's.
//!
//! **When a command is given** (u4r-brief.md 2.2; carry 74 as U4R-Q14 (b1) amended it): at most one per client step,
//! and none until the endpoint holds a snapshot (before it the stamp would be tick 0), nor at a step whose newest
//! snapshot tick did not move since the last command (a second command would be stamped for the tick the first holds,
//! and the room counts it late: room/src/input.rs:107-113). A skipped tick runs option B's repeat. Each step first hands
//! the endpoint what arrived (`input(None, received, now)`), then, only when the newest tick moved,
//! `input(Some(command), &[], now)` (a call with nothing received only stamps). **The press rule** (U4R-Q14, A4): each
//! press counter moves by at most one edge per command given, never per step (the room refuses more edges than the ticks
//! a command covers: room/src/input.rs:91-99); a press wanted at a step without a command waits for the next command.
//! **The view time** (2.2): a client without `Interp` (U4g's) shows its newest snapshot, so each command claims
//! `view_tick` = the endpoint's newest tick and `view_fraction` = 0, which the room's ring holds at every lead of 1 or
//! more.

use npt_client::ClientEndpoint;
use npt_link::Channel;
use npt_sim::{Command, Scene};

/// Move vectors 22.5 degrees apart round the circle, each inside the room's x² + y² ≤ 127² check: a soldier walks
/// through them, one step every [`TURN_EVERY`] commands.
const MOVES: [[i8; 2]; 16] = [
    [127, 0],
    [117, 49],
    [89, 89],
    [49, 117],
    [0, 127],
    [-49, 117],
    [-89, 89],
    [-117, 49],
    [-127, 0],
    [-117, -49],
    [-89, -89],
    [-49, -117],
    [0, -127],
    [49, -117],
    [89, -89],
    [117, -49],
];
/// Commands between two turns of the move vector: a count of commands given, with no time or speed in it (a cadence in
/// time would need a rate-derived constant from npt-sim: carry 39; advisor-u4r-r1, FP-7 item 1).
const TURN_EVERY: u64 = 4;
/// Commands a brawler keeps its target before it picks again: a count of commands given, with no time in it.
const RETARGET_EVERY: u64 = 30;
/// How far above a target's feet the brawl aims, cm (inside a soldier's box).
const AIM_ABOVE_FEET_CM: i32 = 100;
/// Where an aim with no target in view goes: a seeded point within this many cm of the map's centre, in x and y.
const BLIND_SPREAD_CM: u64 = 1_000;
/// The clamp script's points lie this far out in x and y, cm: beyond any weapon's reach from the fight, inside the
/// position grid's range (±32,767 cm).
const FAR_CM: i32 = 32_000;

/// What a behaviour does with each command it gives.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    /// Fire held, a walk round a small circle, aimed at an enemy in view (picked by the seed), else at a seeded point.
    Brawl,
    /// Open loop: fire held and the fire press counter moved by one on every second command (scripted press edges).
    Presses,
    /// Open loop: fire held, aimed at a seeded point far beyond the weapon's reach (the sim clamps it: §4.2).
    Clamp,
    /// Open loop: fire held, the walk, aimed at a seeded point near the centre.
    Steady,
}

/// One client's behaviour (the module text): the brawl or an open-loop script, seeded per slot.
#[derive(Clone, Debug)]
pub struct Behaviour {
    kind: Kind,
    team: Option<u8>,
    teams: Vec<Option<u8>>,
    rng: u64,
    phase: u64,
    /// The seeded point an aim with no target goes to.
    blind: [i32; 3],
    target: Option<u8>,
    /// A command at every step after the first snapshot (D13's test-only setting): never a judged one.
    every_step: bool,
    presses: [u8; 4],
    last_newest: Option<u32>,
    given: u64,
    skipped: u64,
}

/// splitmix64's step: the bots' one generator, seeded per slot (no clock, no hash map).
fn next(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

/// A uniform integer in `[0, n)` from one draw, by multiply and shift.
fn below(word: u64, n: u64) -> u64 {
    ((u128::from(word) * u128::from(n)) >> 64) as u64
}

impl Behaviour {
    fn new(kind: Kind, scene: &Scene, slot: u8, seed: u64) -> Behaviour {
        let mut rng = seed ^ (u64::from(slot) << 56) ^ 0x6e70_742d_626f_7473;
        let phase = below(next(&mut rng), MOVES.len() as u64);
        let spread =
            |r: &mut u64| below(next(r), 2 * BLIND_SPREAD_CM + 1) as i32 - BLIND_SPREAD_CM as i32;
        let blind = match kind {
            Kind::Clamp => {
                let sx = if next(&mut rng) & 1 == 0 { 1 } else { -1 };
                let sy = if next(&mut rng) & 1 == 0 { 1 } else { -1 };
                [sx * FAR_CM, sy * FAR_CM, AIM_ABOVE_FEET_CM]
            }
            _ => [spread(&mut rng), spread(&mut rng), AIM_ABOVE_FEET_CM],
        };
        Behaviour {
            kind,
            team: scene.team(slot),
            teams: (0..scene.slots()).map(|s| scene.team(s)).collect(),
            rng,
            phase,
            blind,
            target: None,
            every_step: false,
            presses: [0; 4],
            last_newest: None,
            given: 0,
            skipped: 0,
        }
    }

    /// The brawl of `slot` in `scene`, from the run's `seed`.
    pub fn brawl(scene: &Scene, slot: u8, seed: u64) -> Behaviour {
        Behaviour::new(Kind::Brawl, scene, slot, seed)
    }

    /// The open-loop script named `script` (`presses`, `clamp` or `steady`) of `slot` in `scene`, from `seed`;
    /// `None` for another name.
    pub fn script(scene: &Scene, slot: u8, seed: u64, script: &str) -> Option<Behaviour> {
        let kind = match script {
            "presses" => Kind::Presses,
            "clamp" => Kind::Clamp,
            "steady" => Kind::Steady,
            _ => return None,
        };
        Some(Behaviour::new(kind, scene, slot, seed))
    }

    /// The same behaviour giving a command at every step once the endpoint holds a snapshot, whether or not its newest
    /// tick moved: D13's test-only setting (u4r-brief.md section 11), which shows the duplicate stamps U4R-Q14 (b)
    /// counts; never a judged run's.
    pub fn every_step(mut self) -> Behaviour {
        self.every_step = true;
        self
    }

    /// The commands given so far.
    pub fn given(&self) -> u64 {
        self.given
    }

    /// The steps since the endpoint's first snapshot at which no command was given (its newest tick had not moved).
    pub fn skipped(&self) -> u64 {
        self.skipped
    }

    /// One client step at `now_ns` on the host's clock (u4r-brief.md 2.2; the module text): `received` goes to the
    /// endpoint first, then a command if one is due. `view` is what the host says this client sees at its newest tick:
    /// (slot, position in the position grid's cm) of other soldiers; the open-loop scripts never read it. Returns
    /// whether a command was given. A host that computes a view must feed the endpoint itself and pass `&[]` (the view
    /// is of the newest tick the endpoint holds after what it was fed: advisor-u4r-r1, FP-7 item 4).
    pub fn step(
        &mut self,
        endpoint: &mut ClientEndpoint,
        received: &[(Channel, Vec<u8>, u64)],
        now_ns: u64,
        view: &[(u8, [i32; 3])],
    ) -> bool {
        endpoint.input(None, received, now_ns);
        let (Some(newest), Some(_)) = (endpoint.newest_tick(), endpoint.slot()) else {
            return false;
        };
        if !self.every_step && self.last_newest == Some(newest) {
            self.skipped += 1;
            return false;
        }
        let command = self.command(newest, view);
        endpoint.input(Some(command), &[], now_ns);
        self.last_newest = Some(newest);
        self.given += 1;
        true
    }

    /// The next command (its slot, tick and sequence are the endpoint's to stamp), claiming `newest` as its view.
    fn command(&mut self, newest: u32, view: &[(u8, [i32; 3])]) -> Command {
        let k = self.given;
        if self.kind == Kind::Presses && k % 2 == 1 {
            self.presses[Command::PRESS_FIRE] = self.presses[Command::PRESS_FIRE].wrapping_add(1);
        }
        let aim = match self.kind {
            Kind::Brawl => self.brawl_aim(k, view),
            _ => self.blind,
        };
        let walk = MOVES[((k / TURN_EVERY + self.phase) % MOVES.len() as u64) as usize];
        Command {
            buttons: Command::FIRE,
            presses: self.presses,
            move_xy: if self.kind == Kind::Clamp {
                [0, 0]
            } else {
                walk
            },
            aim,
            view_tick: newest,
            view_fraction: 0,
            ..Command::default()
        }
    }

    /// The brawl's aim: its target's position in view, raised into its box, re-picked by the seed every
    /// [`RETARGET_EVERY`] commands or when the target is out of view; else the seeded blind point.
    fn brawl_aim(&mut self, k: u64, view: &[(u8, [i32; 3])]) -> [i32; 3] {
        let enemy = |s: &u8| {
            let team = self.teams.get(usize::from(*s)).copied().flatten();
            team.is_some() && team != self.team
        };
        let in_view = |t: u8| view.iter().any(|(s, _)| *s == t);
        if k.is_multiple_of(RETARGET_EVERY) || !self.target.is_some_and(in_view) {
            let enemies: Vec<u8> = view.iter().map(|(s, _)| *s).filter(enemy).collect();
            self.target = (!enemies.is_empty())
                .then(|| enemies[below(next(&mut self.rng), enemies.len() as u64) as usize]);
        }
        match self.target.and_then(|t| view.iter().find(|(s, _)| *s == t)) {
            Some((_, p)) => [p[0], p[1], p[2].saturating_add(AIM_ABOVE_FEET_CM)],
            None => self.blind,
        }
    }
}
