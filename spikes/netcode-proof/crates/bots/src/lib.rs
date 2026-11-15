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
//! and the never-acker U4f's. The brawl's walk is on a leash (BRAWL-1): past 14 m from the cluster centre, by its own
//! soldier's position as the endpoint decoded it (`ClientEndpoint::own_soldier`), it steers back toward the centre.
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
/// The brawl's leash, outer radius, cm from the cluster centre (the origin; BRAWL-1, the advisor's plan review
/// condition 1): past it the walk vector becomes [`toward_centre`] until the soldier is back inside [`LEASH_IN_CM`]. A
/// distance in cm against positions in cm, with no rate in it (carry 39). Its margin is the door-entry margin: a
/// soldier's centre enters a building at 1,800 cm at the nearest door (building 0's, x 1,800 with |y| < 60: npt-sim
/// greybox.rs:83-136), 4 m beyond it as geometry. A soldier overshoots the outer radius, because the leash reads a
/// decoded position some steps old and a skipped tick re-runs the last command: at BRAWL-1's review the sim-only
/// test's skip pattern reached 1,720 cm and B1's bursts 1,832 cm, past the door entry (it prints the largest distance
/// per pattern; BRAWL-1 round 1, NA-B1-2).
const LEASH_OUT_CM: i64 = 1_400;
/// The leash's inner radius, cm: back inside it, the open-loop walk again (the hysteresis between the two radii keeps
/// it from flipping at each command). Above the wall stand-in's contact reach (its north face at 1,220 cm plus a
/// soldier's 40 cm radius: 1,260 cm; greybox.rs:156, world.rs:75-77), so no fixed box lies between the two radii (the
/// crates reach at most 990 cm). The spawn grid's corners lie at 1,414 cm (world.rs:703-722; scene.rs:153), just past
/// the outer radius, so a soldier spawned there steers in at once.
const LEASH_IN_CM: i64 = 1_300;
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
    /// Fire held, a walk round a small circle on the leash (BRAWL-1: toward the centre past [`LEASH_OUT_CM`], by the
    /// position the endpoint decoded), aimed at an enemy in view (picked by the seed), else at a seeded point.
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
    /// The leash's state (the brawl only): steering toward the centre, from past [`LEASH_OUT_CM`] until inside
    /// [`LEASH_IN_CM`].
    steering: bool,
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

/// A move vector from (x, y) toward the origin at about full command length, inside the room's x² + y² ≤ 127² check
/// (so the sim never scales it: world.rs:494-498): each part is 127 times its share of ⌈|(x, y)|⌉, truncated toward 0,
/// so the vector is never longer than 127 and, at the leash's radii, longer than 125. Integers only. Its inputs are
/// decoded grid positions (16 bits on the wire: |x|, |y| at most 32,768, npt-proto quant.rs:197-215), so x² + y² stays
/// below 2^31 and the i64 arithmetic cannot overflow; it is not for other inputs.
fn toward_centre(x: i32, y: i32) -> [i8; 2] {
    let (x, y) = (i64::from(x), i64::from(y));
    let d2 = x * x + y * y;
    let r = d2.isqrt();
    let len = if r * r < d2 { r + 1 } else { r }.max(1);
    let part = |v: i64| i8::try_from(-v * 127 / len).unwrap_or(0);
    [part(x), part(y)]
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
            steering: false,
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
        let command = self.next_command(newest, view, endpoint.own_soldier());
        endpoint.input(Some(command), &[], now_ns);
        true
    }

    /// The decision function (crate-private; `step` calls it, and so do this crate's tests): the next command from
    /// `newest`, `view` and `own`, the own soldier as the endpoint decoded it (`ClientEndpoint::own_soldier`: its grid
    /// position and dead bit), counted as given.
    fn next_command(
        &mut self,
        newest: u32,
        view: &[(u8, [i32; 3])],
        own: Option<([i32; 3], bool)>,
    ) -> Command {
        let command = self.command(newest, view, own);
        self.last_newest = Some(newest);
        self.given += 1;
        command
    }

    /// The brawl's walk on the leash (BRAWL-1, Fix (a)): `walk`, the open-loop vector, unless the own soldier is past
    /// [`LEASH_OUT_CM`] from the centre, where it becomes [`toward_centre`] until the soldier is back inside
    /// [`LEASH_IN_CM`]. It reads only the position the client decoded (x and y in toward_centre's domain), never the
    /// server's state, so a re-run command or a step cut short is corrected once the decoded position shows it
    /// (lead - 1 steps later, plus the ticks between the newest and the newest complete tick). `None` (no complete
    /// tick yet) or a dead soldier (a respawn puts it at home) keeps the walk and the leash's state.
    fn leash(&mut self, own: Option<([i32; 3], bool)>, walk: [i8; 2]) -> [i8; 2] {
        let Some(([x, y, _], false)) = own else {
            return walk;
        };
        let d2 = i64::from(x).pow(2) + i64::from(y).pow(2);
        if d2 > LEASH_OUT_CM.pow(2) {
            self.steering = true;
        } else if d2 < LEASH_IN_CM.pow(2) {
            self.steering = false;
        }
        if self.steering {
            toward_centre(x, y)
        } else {
            walk
        }
    }

    /// The next command (its slot, tick and sequence are the endpoint's to stamp), claiming `newest` as its view.
    fn command(
        &mut self,
        newest: u32,
        view: &[(u8, [i32; 3])],
        own: Option<([i32; 3], bool)>,
    ) -> Command {
        let k = self.given;
        if self.kind == Kind::Presses && k % 2 == 1 {
            self.presses[Command::PRESS_FIRE] = self.presses[Command::PRESS_FIRE].wrapping_add(1);
        }
        let aim = match self.kind {
            Kind::Brawl => self.brawl_aim(k, view),
            _ => self.blind,
        };
        let walk = MOVES[((k / TURN_EVERY + self.phase) % MOVES.len() as u64) as usize];
        let walk = match self.kind {
            Kind::Brawl => self.leash(own, walk),
            _ => walk,
        };
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

/// BRAWL-1's tests (Fix (c); the advisor's plan review, conditions 2, 3, 4 and 13): in this crate, through the
/// crate-private decision function; only this module names npt-sim's World.
#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use npt_sim::{
        EntityKind, HitHistory, Placement, SimCore, StandIns, TickInput, World, ticks_for_ms,
    };

    use super::*;

    const _: () = assert!(
        LEASH_IN_CM >= 1_300 && LEASH_OUT_CM >= LEASH_IN_CM + 100 && LEASH_OUT_CM <= 1_500,
        "the advisor's plan review, condition 1's bounds"
    );

    /// A live soldier at (x, y), as the endpoint would give it.
    fn at(x: i32, y: i32) -> Option<([i32; 3], bool)> {
        Some(([x, y, 150], false))
    }

    fn len2(m: [i8; 2]) -> i64 {
        i64::from(m[0]).pow(2) + i64::from(m[1]).pow(2)
    }

    /// The steering unit test: the decision function with set positions, beside a twin of the same seed fed no
    /// position (today's open-loop walk, command for command). Inside LEASH_IN_CM it walks; between the radii it keeps
    /// what it did (the hysteresis); past LEASH_OUT_CM it steers to the centre and keeps steering under blocked steps
    /// (the position does not move) and skipped ones (the room re-ran the last command: the next position is further
    /// out) until it is inside LEASH_IN_CM; None and dead keep the walk and the leash's state; on the outer radius it
    /// is not past it; the open-loop scripts never steer. No sideways part is kept (condition 1 lets it go: no fixed
    /// box lies between the radii). Every leash vector holds x² + y² ≤ 127², above 125² from LEASH_IN_CM out, and
    /// points toward the centre. The blocked, kept-state and outer-radius checks each fall on a command whose
    /// open-loop walk differs from the steer, so each can fail (BRAWL-1 round 1, TE-B1-3).
    #[test]
    fn brawl1_the_leash_steers_past_its_outer_radius_until_inside_its_inner_one() {
        let scene = Scene::stress();
        let mut b = Behaviour::brawl(&scene, 5, 1);
        let mut twin = Behaviour::brawl(&scene, 5, 1);
        let mut give = |own: Option<([i32; 3], bool)>| {
            let walk = twin.next_command(100, &[], None).move_xy;
            (b.next_command(100, &[], own).move_xy, walk)
        };
        for (own, why) in [
            (at(0, 0), "the centre"),
            (at(500, -300), "inside"),
            (at(1_350, 0), "between the radii, never past the outer"),
        ] {
            let (got, walk) = give(own);
            assert_eq!(got, walk, "{why}: the walk");
        }
        let steer = [-127, 0];
        assert_eq!(give(at(1_450, 0)).0, steer, "past the outer radius");
        let blocked = toward_centre(1_450, 200);
        for _ in 0..3 {
            let (got, walk) = give(at(1_450, 200));
            assert_ne!(
                walk, blocked,
                "the twin's walk differs, so the check can fail"
            );
            assert_eq!(got, blocked, "blocked: the position does not move");
        }
        assert_eq!(
            give(at(1_480, 30)).0,
            toward_centre(1_480, 30),
            "skipped: further out"
        );
        assert_eq!(
            give(at(1_350, 0)).0,
            steer,
            "between the radii after it steered"
        );
        assert_eq!(
            give(at(1_301, 0)).0,
            steer,
            "still outside the inner radius"
        );
        let (got, walk) = give(Some(([1_450, 0, 150], true)));
        assert_eq!(got, walk, "dead: the walk");
        let (got, walk) = give(None);
        assert_eq!(got, walk, "no complete tick: the walk");
        let (got, walk) = give(at(1_350, 0));
        assert_ne!(
            walk, steer,
            "the twin's walk differs, so the check can fail"
        );
        assert_eq!(got, steer, "None and dead keep the leash's state");
        let (got, walk) = give(at(1_299, 0));
        assert_eq!(got, walk, "inside the inner radius: the walk again");
        let (got, walk) = give(at(1_350, 0));
        assert_eq!(got, walk, "between the radii, not past the outer since");
        let (got, walk) = give(at(1_400, 0));
        assert_ne!(
            walk, steer,
            "the twin's walk differs, so the check can fail"
        );
        assert_eq!(got, walk, "on the outer radius: not past it");
        let (got, walk) = give(at(1_401, 0));
        assert_ne!(
            walk, steer,
            "the twin's walk differs, so the check can fail"
        );
        assert_eq!(got, steer, "1 cm past the outer radius");
        assert_eq!(
            give(at(1_100, -1_100)).0,
            [-89, 89],
            "a diagonal past the outer radius"
        );
        let mut steady = Behaviour::script(&scene, 5, 1, "steady").expect("a script");
        let mut open = Behaviour::script(&scene, 5, 1, "steady").expect("a script");
        assert_eq!(
            steady.next_command(100, &[], at(5_000, 0)).move_xy,
            open.next_command(100, &[], None).move_xy,
            "an open-loop script never steers"
        );
        let edges = [
            (32_767, 32_767),
            (-32_768, -32_768),
            (32_767, 0),
            (0, -32_768),
            (1_300, 0),
        ];
        let sweep = (-3_000..=3_000i32)
            .step_by(37)
            .flat_map(|x| (-3_000..=3_000i32).step_by(41).map(move |y| (x, y)));
        for (x, y) in sweep.chain(edges) {
            let d2 = i64::from(x).pow(2) + i64::from(y).pow(2);
            if d2 < LEASH_IN_CM.pow(2) {
                continue;
            }
            let m = toward_centre(x, y);
            assert!(
                len2(m) <= 127 * 127 && len2(m) > 125 * 125,
                "({x}, {y}): {m:?}"
            );
            assert!(
                i64::from(m[0]) * i64::from(x) + i64::from(m[1]) * i64::from(y) < 0,
                "({x}, {y}): {m:?} toward the centre"
            );
        }
    }

    /// The judged runs' first tick (the run scripts' `--tick-start 63736`).
    const FIRST: u32 = 63_736;
    /// The feed's delay, in World steps: each slot decides on its soldier as it was this many steps before, the
    /// largest lead a run takes (`--lead`'s cap, 8), as a step count with no time in it.
    const DELAY: usize = 8;

    /// How the sim-only test feeds the World (each a slot's command per tick).
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Pattern {
        /// (i), barred: a command every tick.
        EveryTick,
        /// (ii), barred: a seeded skip pattern; a skipped tick re-runs the slot's last command unchanged, as the room's
        /// `InputBuffer::take` does. Per slot, each tick that is not skipped or right after a streak starts a streak
        /// with chance 1/8, of 1 to 6 ticks (uniform): about 30 % of ticks skipped, the longest streak 6 (the run
        /// prints both as measured).
        Skips,
        /// (iii), printed only: B1's bursts. Every slot re-runs its last command for `ticks_for_ms(1_000)` ticks at
        /// ten burst starts, the first 30 s into the measured phase (after the 10 s warm-up) and then every 60 s (the
        /// run scripts' B1 settings).
        Bursts,
    }

    /// One run's record: soldier-ticks in a building and the first; the largest distance from the centre (its square in
    /// cm², slot, tick, x, y); skipped ticks, chances and the longest streak; the soldier-ticks seen dead; each soldier
    /// named as stuck (slot, tick, x, y).
    #[derive(Debug, Default)]
    struct Record {
        in_building: u64,
        first_in: Option<(u32, u8, u16)>,
        far: (i64, u8, u32, i32, i32),
        skipped: u64,
        chances: u64,
        streak: u32,
        dead: u64,
        stuck: Vec<(u8, u32, i32, i32)>,
    }

    /// One Soldier record as the test reads it: (slot, grid position, dead bit, area).
    type Seen = (u8, [i32; 3], bool, u16);

    /// Each Soldier record (the dead bit: flags, the layout's sixth field, bit 3: npt-sim world.rs:92, :120), with the
    /// dead bit tied to health (the seventh field) on every record: npt-sim sets both at a death and clears both at a
    /// revive (world.rs:1630-1634, :1176-1188). This pins the literals npt-client copies (client lib.rs:250-251,
    /// SOLDIER_FLAGS and SOLDIER_DEAD) against npt-sim's real deaths.
    fn soldiers(world: &World) -> Vec<Seen> {
        SimCore::entity_records(world)
            .iter()
            .filter(|r| r.kind == EntityKind::Soldier)
            .map(|r| {
                let dead = r.grid[5] & (1 << 3) != 0;
                assert_eq!(
                    dead,
                    r.grid[6] == 0,
                    "slot {}: flags (field 5) bit 3 is set exactly when health (field 6) is 0",
                    r.index
                );
                (
                    r.index as u8,
                    [r.grid[0], r.grid[1], r.grid[2]],
                    dead,
                    r.area,
                )
            })
            .collect()
    }

    /// The sim-only test's one run (no network): npt-sim's World from the judged runs' first tick, SS's 48 brawlers
    /// driven through the decision function for `ms` of ticks under `pattern`. The position fed in is the World's own
    /// record, `DELAY` steps old, standing in for the decoded position (with no network they are equal: this tests
    /// the steering, not the decoding). It records every Soldier record with `area != 0`, the largest distance from
    /// the centre, and names a soldier whose position stays inside a 100 cm square for more than 300 commands given.
    fn sim_run(seed: u64, ms: u32, pattern: Pattern) -> Record {
        let scene = Scene::stress().with_stand_ins(StandIns::OFF);
        let mut world = World::new(&scene, seed, FIRST);
        let history = HitHistory::for_ms(2_000).expect("a ring");
        let n = usize::from(scene.slots());
        let mut bots: Vec<Behaviour> = (0..scene.slots())
            .map(|s| Behaviour::brawl(&scene, s, seed))
            .collect();
        let mut last: Vec<Option<Command>> = vec![None; n];
        let mut seq = vec![0u16; n];
        let mut rng = seed ^ 0x736b_6970_7300_0000;
        let (mut left, mut rest, mut run) = (vec![0u32; n], vec![false; n], vec![0u32; n]);
        // Per slot: the square's corners, the commands given inside it, and whether it was named.
        let mut square = vec![([0i32; 2], [0i32; 2], 0u64, false); n];
        let bursts: Vec<(u32, u32)> = (0..10)
            .map(|i| {
                let s = ticks_for_ms(10_000 + 30_000 + 60_000 * i);
                (s, s + ticks_for_ms(1_000))
            })
            .collect();
        let mut past: VecDeque<Vec<Seen>> = VecDeque::new();
        let mut r = Record::default();
        for k in 0..ticks_for_ms(ms) {
            let t = FIRST.wrapping_add(k);
            let now = soldiers(&world);
            assert_eq!(now.len(), n, "tick {t}: one Soldier record per slot");
            for &(slot, p, dead, area) in &now {
                let s = usize::from(slot);
                if dead {
                    r.dead += 1;
                }
                if area != 0 {
                    r.in_building += 1;
                    r.first_in.get_or_insert((t, slot, area));
                }
                let d2 = i64::from(p[0]).pow(2) + i64::from(p[1]).pow(2);
                if d2 > r.far.0 {
                    r.far = (d2, slot, t, p[0], p[1]);
                }
                let (lo, hi, given, named) = &mut square[s];
                let (nlo, nhi) = (
                    [lo[0].min(p[0]), lo[1].min(p[1])],
                    [hi[0].max(p[0]), hi[1].max(p[1])],
                );
                if *given == 0 || nhi[0] - nlo[0] > 100 || nhi[1] - nlo[1] > 100 {
                    square[s] = ([p[0], p[1]], [p[0], p[1]], 0, false);
                } else {
                    (*lo, *hi) = (nlo, nhi);
                    if *given > 300 && !*named {
                        *named = true;
                        r.stuck.push((slot, t, p[0], p[1]));
                    }
                }
            }
            past.push_back(now);
            if past.len() > DELAY + 1 {
                past.pop_front();
            }
            let fed = &past[0];
            let view: Vec<(u8, [i32; 3])> = fed.iter().map(|x| (x.0, x.1)).collect();
            let burst = bursts.iter().any(|&(a, b)| (a..b).contains(&k));
            let mut commands = Vec::with_capacity(n);
            for s in 0..n {
                let skip = last[s].is_some()
                    && match pattern {
                        Pattern::EveryTick => false,
                        Pattern::Bursts => burst,
                        Pattern::Skips => {
                            r.chances += 1;
                            if left[s] > 0 {
                                left[s] -= 1;
                                rest[s] = left[s] == 0;
                                true
                            } else if rest[s] {
                                rest[s] = false;
                                false
                            } else if below(next(&mut rng), 8) == 0 {
                                left[s] = below(next(&mut rng), 6) as u32;
                                rest[s] = left[s] == 0;
                                true
                            } else {
                                false
                            }
                        }
                    };
                run[s] = if skip { run[s] + 1 } else { 0 };
                r.streak = r.streak.max(run[s]);
                let c = match last[s] {
                    Some(c) if skip => {
                        r.skipped += 1;
                        c
                    }
                    _ => {
                        let own = fed
                            .iter()
                            .find(|x| usize::from(x.0) == s)
                            .map(|x| (x.1, x.2));
                        let newest = t.wrapping_sub(DELAY as u32);
                        let mut c = bots[s].next_command(newest, &view, own);
                        (c.slot, c.tick, c.seq) = (s as u8, t, seq[s]);
                        seq[s] = seq[s].wrapping_add(1);
                        square[s].2 += 1;
                        last[s] = Some(c);
                        c
                    }
                };
                commands.push((c, None));
            }
            let input = TickInput {
                tick: t,
                commands,
                ..TickInput::default()
            };
            world.step(&input, &history);
        }
        r
    }

    /// One run, printed: its pattern, seed and figures (the stuck print names each soldier). The skip chances and the
    /// rate per 1000 are the skip pattern's alone (ADV-PT-L1); every pattern prints its skipped ticks and longest
    /// streak.
    fn printed(seed: u64, ms: u32, pattern: Pattern) -> Record {
        let r = sim_run(seed, ms, pattern);
        let skips = match pattern {
            Pattern::EveryTick => format!("skipped {}", r.skipped),
            Pattern::Skips => format!(
                "skipped {} of {} chances ({} per 1000)",
                r.skipped,
                r.chances,
                (r.skipped * 1_000).checked_div(r.chances).unwrap_or(0)
            ),
            Pattern::Bursts => format!("skipped {} in its bursts", r.skipped),
        };
        println!(
            "brawl1 sim seed {seed} {pattern:?} ({} ticks): in a building {} soldier-ticks (first {:?}); largest \
             distance from the centre {} cm (slot {}, tick {}, at {}, {}); {skips}, longest streak {}; dead {} \
             soldier-ticks; stuck (slot, tick, x, y): {:?}",
            ticks_for_ms(ms),
            r.in_building,
            r.first_in,
            r.far.0.isqrt(),
            r.far.1,
            r.far.2,
            r.far.3,
            r.far.4,
            r.streak,
            r.dead,
            r.stuck
        );
        r
    }

    /// The sim-only test's short copy (condition 13): seed 1, `ticks_for_ms(120_000)`, both barred patterns; no Soldier
    /// record with `area != 0` on any tick; and soldiers die in each pattern, so `soldiers()`' dead-bit tie runs on
    /// real deaths (BRAWL-1 round 1, TE-B1-6).
    #[test]
    fn brawl1_sim_short_no_soldier_enters_a_building() {
        for pattern in [Pattern::EveryTick, Pattern::Skips] {
            let r = printed(1, 120_000, pattern);
            assert_eq!(r.in_building, 0, "{pattern:?}: first {:?}", r.first_in);
            assert!(r.dead > 0, "the dead-bit tie is exercised");
        }
    }

    /// TE-B1-2's npt-sim half (BRAWL-1 round 1): a soldier teleported to building 0's centre (2,200, 0: npt-sim
    /// greybox.rs:92; its footprint is half-open, x 1,800 to 2,600) reads area 1 in its Soldier record, and every other
    /// soldier area 0, so the area the sim-only test bars is one npt-sim sets. A teleport moves only a live soldier
    /// (world.rs:1147-1151), and every soldier starts alive (:745).
    #[test]
    fn brawl1_a_soldier_teleported_into_building_0_reads_area_1() {
        let scene = Scene::stress().with_stand_ins(StandIns::OFF);
        let mut world = World::new(&scene, 1, FIRST);
        let history = HitHistory::for_ms(2_000).expect("a ring");
        let input = TickInput {
            tick: FIRST,
            teleports: vec![Placement {
                slot: 0,
                position: [2_200, 0, 0],
            }],
            ..TickInput::default()
        };
        world.step(&input, &history);
        let now = soldiers(&world);
        assert_eq!(
            now.len(),
            usize::from(scene.slots()),
            "one Soldier record per slot"
        );
        for (slot, _, _, area) in now {
            let want = if slot == 0 { 1 } else { 0 };
            assert_eq!(area, want, "slot {slot}: area 1 in building 0, else 0");
        }
    }

    /// The sim-only test, full (conditions 3 and 13): seeds 1 to 3, a judged run's `ticks_for_ms(610_000)`, the two
    /// barred patterns (no Soldier record with `area != 0` on any tick) and B1's bursts (printed only, never barred).
    #[test]
    #[ignore = "the full 610 s run, 3 seeds x 3 patterns (about 165,000 World steps): run by name in release for the \
                record (the advisor's plan review, condition 13)"]
    fn brawl1_sim_full_no_soldier_enters_a_building_seeds_1_to_3() {
        let mut barred = Vec::new();
        for seed in 1..=3 {
            for pattern in [Pattern::EveryTick, Pattern::Skips, Pattern::Bursts] {
                let r = printed(seed, 610_000, pattern);
                if pattern != Pattern::Bursts && r.in_building > 0 {
                    barred.push((seed, pattern, r.in_building, r.first_in));
                }
            }
        }
        assert!(barred.is_empty(), "in a building: {barred:?}");
    }
}
