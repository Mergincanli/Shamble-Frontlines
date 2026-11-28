# Review packet: BRAWL-1 round 1 (the bots' leash toward the cluster centre, from each client's own decoded position: npt-bots' crate-private decision function and its tests; npt-client's own-soldier read item and its test; npt-lab's in-building figure; the homed items: Part 1's refactors (the Sim alias, the shared constructor, the wake step), N2-1's fourth check, parse()'s bounds, the manifest's entry-ticks cap and the no-log flag; the three re-pinned manifest entries; marks/SERIES.md's new row)

_Cut by `review_packet.py` from `brawl1-packet.json`. It holds sources only: code diffs, the approved plan's lines, the advisor's rulings, the last round's findings and Mustafa's words, each copied by line range. It holds no conclusion of the builder or the main session (no builder notes, brief, checkpoint card or summary). You stay free to read any file and run any test._

## Heads

- The build's `<base>`: 352a245 on the spike branch claude/netcode-proof-test-plan-hcky5g (spike tree abad0635). Its spikes/ equals the branch head's: in the main folder, `git diff --stat 352a245 HEAD -- spikes/` and `git status --porcelain -- spikes/` are both empty.
- Counts at `<base>`: the last counted check.mjs run on these crates is `.gate/u4r-merge/c62h-check.log` (HEAD a36babd; `git diff --stat a36babd 352a245 -- spikes/netcode-proof/crates spikes/netcode-proof/check.mjs spikes/netcode-proof/check.test.mjs` is empty). Its lines: all 11 steps pass; measured build 531 tests passed, bites build 609; self 77 of 77.
- The read copy is the builder's worktree `/home/user/Shamble-Frontlines/.claude/worktrees/brawl1` (detached at 352a245; no builder runs there now). The change is uncommitted there: 10 modified files under spikes/netcode-proof, none new. Change nothing in it.
- The same change as a patch: `.checkpoints/netcode-proof-test/wip/brawl1.patch` (10 files, 1,482 lines; sha256 ee2973aa8ad8c453b5720099c27b37cb9980820da74f33ad9c78ddd7ad591dc7; committed at 0e2bf9a). It equals the read copy's `git diff HEAD`, re-cut and compared by cmp when this packet was cut.
- The diffs below compare the main folder's copy of each file (equal to `<base>`'s) with the read copy's.
- The build's end checks, run in the read copy through the lane's runner `.checkpoints/netcode-proof-test/u4r-run.sh` (sha256 6efb9157…): `.gate/u4r/brawl1/final-check.log` (check.mjs), `.gate/u4r/brawl1/final-det.log` (det-u4.mjs at the new pins) and `.gate/u4r/brawl1/final-regen-tests.log`. The build's other logs are in the same folder: Part 1's p1-*.log, Part 2's p2-*.log and the re-pin's repin*.log.

## What changed (the diffs)

### npt-bots' library (crates/bots/src/lib.rs: the decision function, the leash and their #[cfg(test)] module)
_`git diff --no-index spikes/netcode-proof/crates/bots/src/lib.rs .claude/worktrees/brawl1/spikes/netcode-proof/crates/bots/src/lib.rs`:  1 file changed, 421 insertions(+), 5 deletions(-)._

````diff
diff --git a/spikes/netcode-proof/crates/bots/src/lib.rs b/.claude/worktrees/brawl1/spikes/netcode-proof/crates/bots/src/lib.rs
index 1e4aa54..cadcebe 100644
--- a/spikes/netcode-proof/crates/bots/src/lib.rs
+++ b/.claude/worktrees/brawl1/spikes/netcode-proof/crates/bots/src/lib.rs
@@ -8,7 +8,8 @@
 //! fighting in 3 x 16, every one holding fire, so the scene's weapon interval sets the shot rate PM-0001-2 checks:
 //! §4.13) and the open-loop scripts (commands from the slot, the seed and the count of commands alone, standing in for
 //! the cheats' command patterns in U4r's contract test: U4R-Q4 (a)). The target-course shooter is U4h's, the cheats
-//! and the never-acker U4f's.
+//! and the never-acker U4f's. The brawl's walk is on a leash (BRAWL-1): past 14 m from the cluster centre, by its own
+//! soldier's position as the endpoint decoded it (`ClientEndpoint::own_soldier`), it steers back toward the centre.
 //!
 //! **When a command is given** (u4r-brief.md 2.2; carry 74 as U4R-Q14 (b1) amended it): at most one per client step,
 //! and none until the endpoint holds a snapshot (before it the stamp would be tick 0), nor at a step whose newest
@@ -51,6 +52,18 @@ const MOVES: [[i8; 2]; 16] = [
 const TURN_EVERY: u64 = 4;
 /// Commands a brawler keeps its target before it picks again: a count of commands given, with no time in it.
 const RETARGET_EVERY: u64 = 30;
+/// The brawl's leash, outer radius, cm from the cluster centre (the origin; BRAWL-1, the advisor's plan review
+/// condition 1): past it the walk vector becomes [`toward_centre`] until the soldier is back inside [`LEASH_IN_CM`]. A
+/// distance in cm against positions in cm, with no rate in it (carry 39). Its margin is the door-entry margin: a
+/// soldier's centre enters a building at 1,800 cm at the nearest door (building 0's, x 1,800 with |y| < 60: npt-sim
+/// greybox.rs:83-136), 4 m beyond it.
+const LEASH_OUT_CM: i64 = 1_400;
+/// The leash's inner radius, cm: back inside it, the open-loop walk again (the hysteresis between the two radii keeps
+/// it from flipping at each command). Above the wall stand-in's contact reach (its north face at 1,220 cm plus a
+/// soldier's 40 cm radius: 1,260 cm; greybox.rs:156, world.rs:75-77), so no fixed box lies between the two radii (the
+/// crates reach at most 990 cm). The spawn grid's corners lie at 1,414 cm (world.rs:703-722; scene.rs:153), just past
+/// the outer radius, so a soldier spawned there steers in at once.
+const LEASH_IN_CM: i64 = 1_300;
 /// How far above a target's feet the brawl aims, cm (inside a soldier's box).
 const AIM_ABOVE_FEET_CM: i32 = 100;
 /// Where an aim with no target in view goes: a seeded point within this many cm of the map's centre, in x and y.
@@ -62,7 +75,8 @@ const FAR_CM: i32 = 32_000;
 /// What a behaviour does with each command it gives.
 #[derive(Clone, Copy, Debug, PartialEq, Eq)]
 enum Kind {
-    /// Fire held, a walk round a small circle, aimed at an enemy in view (picked by the seed), else at a seeded point.
+    /// Fire held, a walk round a small circle on the leash (BRAWL-1: toward the centre past [`LEASH_OUT_CM`], by the
+    /// position the endpoint decoded), aimed at an enemy in view (picked by the seed), else at a seeded point.
     Brawl,
     /// Open loop: fire held and the fire press counter moved by one on every second command (scripted press edges).
     Presses,
@@ -89,6 +103,9 @@ pub struct Behaviour {
     last_newest: Option<u32>,
     given: u64,
     skipped: u64,
+    /// The leash's state (the brawl only): steering toward the centre, from past [`LEASH_OUT_CM`] until inside
+    /// [`LEASH_IN_CM`].
+    steering: bool,
 }
 
 /// splitmix64's step: the bots' one generator, seeded per slot (no clock, no hash map).
@@ -105,6 +122,18 @@ fn below(word: u64, n: u64) -> u64 {
     ((u128::from(word) * u128::from(n)) >> 64) as u64
 }
 
+/// A move vector from (x, y) toward the origin at about full command length, inside the room's x² + y² ≤ 127² check
+/// (so the sim never scales it: world.rs:494-498): each part is 127 times its share of ⌈|(x, y)|⌉, truncated toward 0,
+/// so the vector is never longer than 127 and, at the leash's radii, longer than 125. Integers only.
+fn toward_centre(x: i32, y: i32) -> [i8; 2] {
+    let (x, y) = (i64::from(x), i64::from(y));
+    let d2 = x * x + y * y;
+    let r = d2.isqrt();
+    let len = if r * r < d2 { r + 1 } else { r }.max(1);
+    let part = |v: i64| i8::try_from(-v * 127 / len).unwrap_or(0);
+    [part(x), part(y)]
+}
+
 impl Behaviour {
     fn new(kind: Kind, scene: &Scene, slot: u8, seed: u64) -> Behaviour {
         let mut rng = seed ^ (u64::from(slot) << 56) ^ 0x6e70_742d_626f_7473;
@@ -132,6 +161,7 @@ impl Behaviour {
             last_newest: None,
             given: 0,
             skipped: 0,
+            steering: false,
         }
     }
 
@@ -190,15 +220,55 @@ impl Behaviour {
             self.skipped += 1;
             return false;
         }
-        let command = self.command(newest, view);
+        let command = self.next_command(newest, view, endpoint.own_soldier());
         endpoint.input(Some(command), &[], now_ns);
+        true
+    }
+
+    /// The decision function (crate-private; `step` calls it, and so do this crate's tests): the next command from
+    /// `newest`, `view` and `own`, the own soldier as the endpoint decoded it (`ClientEndpoint::own_soldier`: its grid
+    /// position and dead bit), counted as given.
+    fn next_command(
+        &mut self,
+        newest: u32,
+        view: &[(u8, [i32; 3])],
+        own: Option<([i32; 3], bool)>,
+    ) -> Command {
+        let command = self.command(newest, view, own);
         self.last_newest = Some(newest);
         self.given += 1;
-        true
+        command
+    }
+
+    /// The brawl's walk on the leash (BRAWL-1, Fix (a)): `walk`, the open-loop vector, unless the own soldier is past
+    /// [`LEASH_OUT_CM`] from the centre, where it becomes [`toward_centre`] until the soldier is back inside
+    /// [`LEASH_IN_CM`]. It reads only the position the client decoded, never the server's state, so a re-run command or
+    /// a step cut short is corrected at the next command. `None` (no complete tick yet) or a dead soldier (a respawn
+    /// puts it at home) keeps the walk and the leash's state.
+    fn leash(&mut self, own: Option<([i32; 3], bool)>, walk: [i8; 2]) -> [i8; 2] {
+        let Some(([x, y, _], false)) = own else {
+            return walk;
+        };
+        let d2 = i64::from(x).pow(2) + i64::from(y).pow(2);
+        if d2 > LEASH_OUT_CM.pow(2) {
+            self.steering = true;
+        } else if d2 < LEASH_IN_CM.pow(2) {
+            self.steering = false;
+        }
+        if self.steering {
+            toward_centre(x, y)
+        } else {
+            walk
+        }
     }
 
     /// The next command (its slot, tick and sequence are the endpoint's to stamp), claiming `newest` as its view.
-    fn command(&mut self, newest: u32, view: &[(u8, [i32; 3])]) -> Command {
+    fn command(
+        &mut self,
+        newest: u32,
+        view: &[(u8, [i32; 3])],
+        own: Option<([i32; 3], bool)>,
+    ) -> Command {
         let k = self.given;
         if self.kind == Kind::Presses && k % 2 == 1 {
             self.presses[Command::PRESS_FIRE] = self.presses[Command::PRESS_FIRE].wrapping_add(1);
@@ -208,6 +278,10 @@ impl Behaviour {
             _ => self.blind,
         };
         let walk = MOVES[((k / TURN_EVERY + self.phase) % MOVES.len() as u64) as usize];
+        let walk = match self.kind {
+            Kind::Brawl => self.leash(own, walk),
+            _ => walk,
+        };
         Command {
             buttons: Command::FIRE,
             presses: self.presses,
@@ -242,3 +316,345 @@ impl Behaviour {
         }
     }
 }
+
+/// BRAWL-1's tests (Fix (c); the advisor's plan review, conditions 2, 3, 4 and 13): in this crate, through the
+/// crate-private decision function; only this module names npt-sim's World.
+#[cfg(test)]
+mod tests {
+    use std::collections::VecDeque;
+
+    use npt_sim::{EntityKind, HitHistory, SimCore, StandIns, TickInput, World, ticks_for_ms};
+
+    use super::*;
+
+    /// A live soldier at (x, y), as the endpoint would give it.
+    fn at(x: i32, y: i32) -> Option<([i32; 3], bool)> {
+        Some(([x, y, 150], false))
+    }
+
+    fn len2(m: [i8; 2]) -> i64 {
+        i64::from(m[0]).pow(2) + i64::from(m[1]).pow(2)
+    }
+
+    /// The steering unit test: the decision function with set positions, beside a twin of the same seed fed no
+    /// position (today's open-loop walk, command for command). Inside LEASH_IN_CM it walks; between the radii it keeps
+    /// what it did (the hysteresis); past LEASH_OUT_CM it steers to the centre and keeps steering under blocked steps
+    /// (the position does not move) and skipped ones (the room re-ran the last command: the next position is further
+    /// out) until it is inside LEASH_IN_CM; None and dead keep the walk; the open-loop scripts never steer. No sideways
+    /// part is kept (condition 1 lets it go: no fixed box lies between the radii). Every leash vector holds x² + y² ≤
+    /// 127², above 125² from LEASH_IN_CM out, and points toward the centre.
+    #[test]
+    fn brawl1_the_leash_steers_past_its_outer_radius_until_inside_its_inner_one() {
+        let scene = Scene::stress();
+        let mut b = Behaviour::brawl(&scene, 5, 1);
+        let mut twin = Behaviour::brawl(&scene, 5, 1);
+        let mut give = |own: Option<([i32; 3], bool)>| {
+            let walk = twin.next_command(100, &[], None).move_xy;
+            (b.next_command(100, &[], own).move_xy, walk)
+        };
+        for (own, why) in [
+            (at(0, 0), "the centre"),
+            (at(500, -300), "inside"),
+            (at(1_350, 0), "between the radii, never past the outer"),
+        ] {
+            let (got, walk) = give(own);
+            assert_eq!(got, walk, "{why}: the walk");
+        }
+        let steer = [-127, 0];
+        assert_eq!(give(at(1_450, 0)).0, steer, "past the outer radius");
+        for _ in 0..3 {
+            assert_eq!(
+                give(at(1_450, 0)).0,
+                steer,
+                "blocked: the position does not move"
+            );
+        }
+        assert_eq!(
+            give(at(1_480, 30)).0,
+            toward_centre(1_480, 30),
+            "skipped: further out"
+        );
+        assert_eq!(
+            give(at(1_350, 0)).0,
+            steer,
+            "between the radii after it steered"
+        );
+        assert_eq!(
+            give(at(1_301, 0)).0,
+            steer,
+            "still outside the inner radius"
+        );
+        let (got, walk) = give(Some(([1_450, 0, 150], true)));
+        assert_eq!(got, walk, "dead: the walk");
+        let (got, walk) = give(None);
+        assert_eq!(got, walk, "no complete tick: the walk");
+        let (got, walk) = give(at(1_299, 0));
+        assert_eq!(got, walk, "inside the inner radius: the walk again");
+        let (got, walk) = give(at(1_350, 0));
+        assert_eq!(got, walk, "between the radii, not past the outer since");
+        assert_eq!(
+            give(at(1_100, -1_100)).0,
+            [-89, 89],
+            "a diagonal past the outer radius"
+        );
+        let mut steady = Behaviour::script(&scene, 5, 1, "steady").expect("a script");
+        let mut open = Behaviour::script(&scene, 5, 1, "steady").expect("a script");
+        assert_eq!(
+            steady.next_command(100, &[], at(5_000, 0)).move_xy,
+            open.next_command(100, &[], None).move_xy,
+            "an open-loop script never steers"
+        );
+        let edges = [
+            (32_767, 32_767),
+            (-32_768, -32_768),
+            (32_767, 0),
+            (0, -32_768),
+            (1_300, 0),
+        ];
+        let sweep = (-3_000..=3_000i32)
+            .step_by(37)
+            .flat_map(|x| (-3_000..=3_000i32).step_by(41).map(move |y| (x, y)));
+        for (x, y) in sweep.chain(edges) {
+            let d2 = i64::from(x).pow(2) + i64::from(y).pow(2);
+            if d2 < LEASH_IN_CM.pow(2) {
+                continue;
+            }
+            let m = toward_centre(x, y);
+            assert!(
+                len2(m) <= 127 * 127 && len2(m) > 125 * 125,
+                "({x}, {y}): {m:?}"
+            );
+            assert!(
+                i64::from(m[0]) * i64::from(x) + i64::from(m[1]) * i64::from(y) < 0,
+                "({x}, {y}): {m:?} toward the centre"
+            );
+        }
+    }
+
+    /// The judged runs' first tick (the run scripts' `--tick-start 63736`).
+    const FIRST: u32 = 63_736;
+    /// The feed's delay, in World steps: each slot decides on its soldier as it was this many steps before, the
+    /// largest lead a run takes (`--lead`'s cap, 8), as a step count with no time in it.
+    const DELAY: usize = 8;
+
+    /// How the sim-only test feeds the World (each a slot's command per tick).
+    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
+    enum Pattern {
+        /// (i), barred: a command every tick.
+        EveryTick,
+        /// (ii), barred: a seeded skip pattern; a skipped tick re-runs the slot's last command unchanged, as the room's
+        /// `InputBuffer::take` does. Per slot, each tick that is not skipped or right after a streak starts a streak
+        /// with chance 1/8, of 1 to 6 ticks (uniform): about 30 % of ticks skipped, the longest streak 6 (the run
+        /// prints both as measured).
+        Skips,
+        /// (iii), printed only: B1's bursts. Every slot re-runs its last command for `ticks_for_ms(1_000)` ticks at
+        /// ten burst starts, the first 30 s into the measured phase (after the 10 s warm-up) and then every 60 s (the
+        /// run scripts' B1 settings).
+        Bursts,
+    }
+
+    /// One run's record: soldier-ticks in a building and the first; the largest distance from the centre (its square in
+    /// cm², slot, tick, x, y); skipped ticks, chances and the longest streak; each soldier named as stuck (slot, tick,
+    /// x, y).
+    #[derive(Debug, Default)]
+    struct Record {
+        in_building: u64,
+        first_in: Option<(u32, u8, u16)>,
+        far: (i64, u8, u32, i32, i32),
+        skipped: u64,
+        chances: u64,
+        streak: u32,
+        stuck: Vec<(u8, u32, i32, i32)>,
+    }
+
+    /// One Soldier record as the test reads it: (slot, grid position, dead bit, area).
+    type Seen = (u8, [i32; 3], bool, u16);
+
+    /// Each Soldier record (the dead bit: flags, the layout's sixth field, bit 3: npt-sim world.rs:92, :120).
+    fn soldiers(world: &World) -> Vec<Seen> {
+        SimCore::entity_records(world)
+            .iter()
+            .filter(|r| r.kind == EntityKind::Soldier)
+            .map(|r| {
+                let dead = r.grid[5] & (1 << 3) != 0;
+                (
+                    r.index as u8,
+                    [r.grid[0], r.grid[1], r.grid[2]],
+                    dead,
+                    r.area,
+                )
+            })
+            .collect()
+    }
+
+    /// The sim-only test's one run (no network): npt-sim's World from the judged runs' first tick, SS's 48 brawlers
+    /// driven through the decision function for `ms` of ticks under `pattern`. The position fed in is the World's own
+    /// record, `DELAY` steps old, standing in for the decoded position (with no network they are equal: this tests
+    /// the steering, not the decoding). It records every Soldier record with `area != 0`, the largest distance from
+    /// the centre, and names a soldier whose position stays inside a 100 cm square for more than 300 commands given.
+    fn sim_run(seed: u64, ms: u32, pattern: Pattern) -> Record {
+        let scene = Scene::stress().with_stand_ins(StandIns::OFF);
+        let mut world = World::new(&scene, seed, FIRST);
+        let history = HitHistory::for_ms(2_000).expect("a ring");
+        let n = usize::from(scene.slots());
+        let mut bots: Vec<Behaviour> = (0..scene.slots())
+            .map(|s| Behaviour::brawl(&scene, s, seed))
+            .collect();
+        let mut last: Vec<Option<Command>> = vec![None; n];
+        let mut seq = vec![0u16; n];
+        let mut rng = seed ^ 0x736b_6970_7300_0000;
+        let (mut left, mut rest, mut run) = (vec![0u32; n], vec![false; n], vec![0u32; n]);
+        // Per slot: the square's corners, the commands given inside it, and whether it was named.
+        let mut square = vec![([0i32; 2], [0i32; 2], 0u64, false); n];
+        let bursts: Vec<(u32, u32)> = (0..10)
+            .map(|i| {
+                let s = ticks_for_ms(10_000 + 30_000 + 60_000 * i);
+                (s, s + ticks_for_ms(1_000))
+            })
+            .collect();
+        let mut past: VecDeque<Vec<Seen>> = VecDeque::new();
+        let mut r = Record::default();
+        for k in 0..ticks_for_ms(ms) {
+            let t = FIRST.wrapping_add(k);
+            let now = soldiers(&world);
+            for &(slot, p, _, area) in &now {
+                let s = usize::from(slot);
+                if area != 0 {
+                    r.in_building += 1;
+                    r.first_in.get_or_insert((t, slot, area));
+                }
+                let d2 = i64::from(p[0]).pow(2) + i64::from(p[1]).pow(2);
+                if d2 > r.far.0 {
+                    r.far = (d2, slot, t, p[0], p[1]);
+                }
+                let (lo, hi, given, named) = &mut square[s];
+                let (nlo, nhi) = (
+                    [lo[0].min(p[0]), lo[1].min(p[1])],
+                    [hi[0].max(p[0]), hi[1].max(p[1])],
+                );
+                if *given == 0 || nhi[0] - nlo[0] > 100 || nhi[1] - nlo[1] > 100 {
+                    square[s] = ([p[0], p[1]], [p[0], p[1]], 0, false);
+                } else {
+                    (*lo, *hi) = (nlo, nhi);
+                    if *given > 300 && !*named {
+                        *named = true;
+                        r.stuck.push((slot, t, p[0], p[1]));
+                    }
+                }
+            }
+            past.push_back(now);
+            if past.len() > DELAY + 1 {
+                past.pop_front();
+            }
+            let fed = &past[0];
+            let view: Vec<(u8, [i32; 3])> = fed.iter().map(|x| (x.0, x.1)).collect();
+            let burst = bursts.iter().any(|&(a, b)| (a..b).contains(&k));
+            let mut commands = Vec::with_capacity(n);
+            for s in 0..n {
+                let skip = last[s].is_some()
+                    && match pattern {
+                        Pattern::EveryTick => false,
+                        Pattern::Bursts => burst,
+                        Pattern::Skips => {
+                            r.chances += 1;
+                            if left[s] > 0 {
+                                left[s] -= 1;
+                                rest[s] = left[s] == 0;
+                                true
+                            } else if rest[s] {
+                                rest[s] = false;
+                                false
+                            } else if below(next(&mut rng), 8) == 0 {
+                                left[s] = below(next(&mut rng), 6) as u32;
+                                rest[s] = left[s] == 0;
+                                true
+                            } else {
+                                false
+                            }
+                        }
+                    };
+                run[s] = if skip { run[s] + 1 } else { 0 };
+                r.streak = r.streak.max(run[s]);
+                let c = match last[s] {
+                    Some(c) if skip => {
+                        r.skipped += 1;
+                        c
+                    }
+                    _ => {
+                        let own = fed
+                            .iter()
+                            .find(|x| usize::from(x.0) == s)
+                            .map(|x| (x.1, x.2));
+                        let newest = t.wrapping_sub(DELAY as u32);
+                        let mut c = bots[s].next_command(newest, &view, own);
+                        (c.slot, c.tick, c.seq) = (s as u8, t, seq[s]);
+                        seq[s] = seq[s].wrapping_add(1);
+                        square[s].2 += 1;
+                        last[s] = Some(c);
+                        c
+                    }
+                };
+                commands.push((c, None));
+            }
+            let input = TickInput {
+                tick: t,
+                commands,
+                ..TickInput::default()
+            };
+            world.step(&input, &history);
+        }
+        r
+    }
+
+    /// One run, printed: its pattern, seed and figures (the stuck print names each soldier).
+    fn printed(seed: u64, ms: u32, pattern: Pattern) -> Record {
+        let r = sim_run(seed, ms, pattern);
+        let rate = (r.skipped * 1_000).checked_div(r.chances).unwrap_or(0);
+        println!(
+            "brawl1 sim seed {seed} {pattern:?} ({} ticks): in a building {} soldier-ticks (first {:?}); largest \
+             distance from the centre {} cm (slot {}, tick {}, at {}, {}); skipped {} of {} chances ({rate} per 1000), \
+             longest streak {}; stuck (slot, tick, x, y): {:?}",
+            ticks_for_ms(ms),
+            r.in_building,
+            r.first_in,
+            r.far.0.isqrt(),
+            r.far.1,
+            r.far.2,
+            r.far.3,
+            r.far.4,
+            r.skipped,
+            r.chances,
+            r.streak,
+            r.stuck
+        );
+        r
+    }
+
+    /// The sim-only test's short copy (condition 13): seed 1, `ticks_for_ms(120_000)`, both barred patterns; no Soldier
+    /// record with `area != 0` on any tick.
+    #[test]
+    fn brawl1_sim_short_no_soldier_enters_a_building() {
+        for pattern in [Pattern::EveryTick, Pattern::Skips] {
+            let r = printed(1, 120_000, pattern);
+            assert_eq!(r.in_building, 0, "{pattern:?}: first {:?}", r.first_in);
+        }
+    }
+
+    /// The sim-only test, full (conditions 3 and 13): seeds 1 to 3, a judged run's `ticks_for_ms(610_000)`, the two
+    /// barred patterns (no Soldier record with `area != 0` on any tick) and B1's bursts (printed only, never barred).
+    #[test]
+    #[ignore = "the full 610 s run, 3 seeds x 3 patterns (about 165,000 World steps): run by name in release for the \
+                record (the advisor's plan review, condition 13)"]
+    fn brawl1_sim_full_no_soldier_enters_a_building_seeds_1_to_3() {
+        let mut barred = Vec::new();
+        for seed in 1..=3 {
+            for pattern in [Pattern::EveryTick, Pattern::Skips, Pattern::Bursts] {
+                let r = printed(seed, 610_000, pattern);
+                if pattern != Pattern::Bursts && r.in_building > 0 {
+                    barred.push((seed, pattern, r.in_building, r.first_in));
+                }
+            }
+        }
+        assert!(barred.is_empty(), "in a building: {barred:?}");
+    }
+}
````

### npt-client's library (crates/client/src/lib.rs: the own-soldier read item)
_`git diff --no-index spikes/netcode-proof/crates/client/src/lib.rs .claude/worktrees/brawl1/spikes/netcode-proof/crates/client/src/lib.rs`:  1 file changed, 31 insertions(+)._

````diff
diff --git a/spikes/netcode-proof/crates/client/src/lib.rs b/.claude/worktrees/brawl1/spikes/netcode-proof/crates/client/src/lib.rs
index 882f93d..a79a0bb 100644
--- a/spikes/netcode-proof/crates/client/src/lib.rs
+++ b/.claude/worktrees/brawl1/spikes/netcode-proof/crates/client/src/lib.rs
@@ -242,6 +242,10 @@ pub struct ClientEndpoint {
     out: StepOut,
 }
 
+/// A Soldier record's flags field, the sixth of its layout (npt-sim seam.rs's `SOLDIER`: x, y, z, yaw, pitch, flags,
+/// health, weapon), and its dead bit (npt-sim world.rs's `FLAG_DEAD`, bit 3): both private there.
+const SOLDIER_FLAGS: usize = 5;
+const SOLDIER_DEAD: i32 = 1 << 3;
 /// Pings the endpoint holds to answer at its next step, at most (the room sends a few a second).
 const PINGS_HELD: usize = 8;
 /// Bad headers' ticks the endpoint remembers, so each gets the sentinel once (the groups of one tick arrive together).
@@ -337,6 +341,33 @@ impl ClientEndpoint {
         self.newest
     }
 
+    /// The client's own decoded state of its own soldier (BRAWL-1; the advisor's plan review, condition 5): its grid
+    /// position [x, y, z] (1 cm steps, as decoded: no float) and whether its flags say dead (bit 3), by value, from the
+    /// newest tick that is complete and filed with a hash, ticks compared by the endpoint's wrap rule. `None` before
+    /// the slot or the first such tick, or when that tick's own section holds no record of the client's soldier. A
+    /// newer tick that is incomplete, refused or the sentinel changes nothing. Never the room's state: only what this
+    /// client decoded.
+    pub fn own_soldier(&self) -> Option<([i32; 3], bool)> {
+        let slot = u16::from(self.slot?);
+        let tick = self
+            .ring
+            .iter()
+            .filter(|t| t.used && t.complete() && matches!(t.filed, Filed::Hash(_)))
+            .reduce(|a, b| {
+                if self.wrap.after32("tick", a.tick, b.tick) {
+                    b
+                } else {
+                    a
+                }
+            })?;
+        let own = &tick.frame.own;
+        let i = own
+            .binary_search_by_key(&(EntityKind::Soldier, slot), key)
+            .ok()?;
+        let g = own[i].rec.grid;
+        Some(([g[0], g[1], g[2]], g[SOLDIER_FLAGS] & SOLDIER_DEAD != 0))
+    }
+
     /// The command the header's last-run sequence names, rebuilt to the client's own 32-bit command counter (U4E-Q2
     /// (c): where U4g's replay starts).
     pub fn last_run_command(&self) -> Option<u32> {
````

### npt-client's endpoint tests (crates/client/tests/endpoint.rs)
_`git diff --no-index spikes/netcode-proof/crates/client/tests/endpoint.rs .claude/worktrees/brawl1/spikes/netcode-proof/crates/client/tests/endpoint.rs`:  1 file changed, 60 insertions(+)._

````diff
diff --git a/spikes/netcode-proof/crates/client/tests/endpoint.rs b/.claude/worktrees/brawl1/spikes/netcode-proof/crates/client/tests/endpoint.rs
index 137d655..7adb1dd 100644
--- a/spikes/netcode-proof/crates/client/tests/endpoint.rs
+++ b/.claude/worktrees/brawl1/spikes/netcode-proof/crates/client/tests/endpoint.rs
@@ -380,6 +380,66 @@ fn pm_0001_58_event_numbers_are_rebuilt_across_the_16_bit_wrap() {
     assert_eq!(wraps, ["event_seq"]);
 }
 
+/// A one-group snapshot of tick `t` (against the defaults) whose own section holds `own`.
+fn own_only(t: u32, groups: u8, own: Vec<EntityRecord>) -> Snapshot {
+    Snapshot {
+        tick: t,
+        groups,
+        own: own.into_iter().map(EntityDelta::Add).collect(),
+        ..Snapshot::default()
+    }
+}
+
+/// BRAWL-1's read item (the advisor's plan review, condition 5): `own_soldier` is `None` before the first complete
+/// tick; after a complete tick it is the own record's x, y, z and dead bit (flags bit 3); a newer incomplete, refused or
+/// sentinel tick changes nothing; a newer complete tick moves it; a complete tick with no own record gives `None`.
+#[test]
+fn brawl1_own_soldier_is_the_newest_complete_filed_tick_s_own_record() {
+    let mut e = endpoint();
+    assert_eq!(e.own_soldier(), None, "nothing received");
+    let accepted = bytes(&Message::JoinAccepted(JoinAccepted {
+        slot: 0,
+        tick: 100,
+        ..JoinAccepted::default()
+    }));
+    feed(&mut e, vec![(Channel::Control, accepted, 0)]);
+    assert_eq!(e.own_soldier(), None, "joined, no snapshot");
+    let mut dead = soldier(0, 1_234);
+    dead.grid[5] = 1 << 3;
+    let out = feed(&mut e, vec![datagram(own_only(100, 2, vec![dead]))]);
+    assert!(out.hashes.is_empty());
+    assert_eq!(e.own_soldier(), None, "one group of two: incomplete");
+    let out = feed(&mut e, vec![datagram(own_only(101, 1, vec![dead]))]);
+    assert_eq!(out.hashes.len(), 1, "tick 101 complete and filed");
+    let first = Some(([1_234, -1_234, 150], true));
+    assert_eq!(e.own_soldier(), first);
+    // Newer ticks that are incomplete (102), refused for a baseline it does not hold (103: the sentinel) and the
+    // sentinel by a corrupt datagram (104): no change.
+    feed(
+        &mut e,
+        vec![datagram(own_only(102, 2, vec![soldier(0, 9)]))],
+    );
+    let out = feed(&mut e, vec![datagram(moved(103, 12, 0, 1, 150))]);
+    assert_eq!(out.hashes, [(103, 0)]);
+    let mut b = bytes(&Message::Snapshot(own_only(104, 1, vec![soldier(0, 9)])));
+    b.push(0);
+    let out = feed(&mut e, vec![(Channel::Datagram, b, 0)]);
+    assert_eq!(out.hashes, [(104, 0)]);
+    assert_eq!(e.own_soldier(), first, "no change from ticks 102 to 104");
+    let out = feed(
+        &mut e,
+        vec![datagram(own_only(105, 1, vec![soldier(0, -700)]))],
+    );
+    assert_eq!(out.hashes.len(), 1);
+    assert_eq!(e.own_soldier(), Some(([-700, 700, 150], false)), "tick 105");
+    let out = feed(
+        &mut e,
+        vec![datagram(own_only(106, 1, vec![soldier(1, 5)]))],
+    );
+    assert_eq!(out.hashes.len(), 1);
+    assert_eq!(e.own_soldier(), None, "tick 106: no record of soldier 0");
+}
+
 /// npt-client's planted defects (bites builds only), each in wave 1's two parts: a bites-only twin asserting the clean
 /// test's check fails on the bitten endpoint.
 #[cfg(feature = "bites")]
````

### npt-lab: scenario.rs
_`git diff --no-index spikes/netcode-proof/crates/lab/src/scenario.rs .claude/worktrees/brawl1/spikes/netcode-proof/crates/lab/src/scenario.rs`:  1 file changed, 225 insertions(+), 68 deletions(-)._

````diff
diff --git a/spikes/netcode-proof/crates/lab/src/scenario.rs b/.claude/worktrees/brawl1/spikes/netcode-proof/crates/lab/src/scenario.rs
index 4865e70..3841812 100644
--- a/spikes/netcode-proof/crates/lab/src/scenario.rs
+++ b/.claude/worktrees/brawl1/spikes/netcode-proof/crates/lab/src/scenario.rs
@@ -33,7 +33,7 @@ use std::path::PathBuf;
 use npt_bots::Behaviour;
 use npt_client::ClientEndpoint;
 use npt_l1::{L1Net, Profile};
-use npt_link::{Channel, Link, LinkState};
+use npt_link::{Channel, Link, LinkState, Listener};
 use npt_log::{Ended, EventRecord, Phase};
 use npt_proto::{Direction, Join, Kick, Message, Quant};
 use npt_room::Room;
@@ -46,6 +46,10 @@ use crate::hash::Failure;
 use crate::logs::{Logs, Meta};
 use crate::mirror::{Mirror, Records, SetStats};
 
+/// The simulation the lab's rooms run (advisor-u4r-r1, EA-U4R-2; U4-SEAM: npt-lab names `npt_sim::World` once, here;
+/// hash.rs's bite list and dump checks name npt-sim's World as the tool they check, not a room's).
+pub(crate) type Sim = World;
+
 /// The protocol version and content hash the lab's room and clients share.
 pub(crate) const VERSION: u16 = 1;
 pub(crate) const CONTENT: [u8; 32] = *b"npt-lab scenario content hash v1";
@@ -209,7 +213,7 @@ impl Bites {
         let proto = npt_proto::BITES.contains(&id);
         let b = Bites {
             id: Some(id.to_owned()),
-            world: World::BITES.contains(&id),
+            world: Sim::BITES.contains(&id),
             room: npt_room::BITES.contains(&id) || proto,
             client: npt_client::BITES.contains(&id) || proto,
             decoder: proto,
@@ -265,19 +269,19 @@ impl Bites {
         Quant::default()
     }
 
-    fn world(&self, scene: &Scene, seed: u64, first_tick: u32, arm_tick: u32) -> World {
+    fn world(&self, scene: &Scene, seed: u64, first_tick: u32, arm_tick: u32) -> Sim {
         #[cfg(feature = "bites")]
         if self.world
             && let Some(id) = self.id.as_deref()
         {
-            return World::with_bite(scene, seed, first_tick, id, arm_tick)
+            return Sim::with_bite(scene, seed, first_tick, id, arm_tick)
                 .expect("Bites::of checked that this instance plants it");
         }
         let _ = arm_tick;
-        World::new(scene, seed, first_tick)
+        Sim::new(scene, seed, first_tick)
     }
 
-    fn room(&self, room: Room<World>) -> Room<World> {
+    fn room(&self, room: Room<Sim>) -> Room<Sim> {
         #[cfg(feature = "bites")]
         if self.room
             && let Some(id) = self.id.as_deref()
@@ -340,7 +344,7 @@ pub(crate) struct Session {
 
 /// The server half (the room behind one decoder per session), the same on L1 and on Loopback.
 pub(crate) struct Server {
-    pub(crate) room: Room<World>,
+    pub(crate) room: Room<Sim>,
     pub(crate) sessions: Vec<Option<Session>>,
     quant: Quant,
     rtt: RttSource,
@@ -360,7 +364,37 @@ pub(crate) struct Server {
 }
 
 impl Server {
-    pub(crate) fn new(room: Room<World>, quant: Quant, rtt: RttSource) -> Server {
+    /// The lab's one server (advisor-u4r-r1, EA-U4R-2), for `run` and the pre-check's `run_leg`: a room on a new
+    /// [`Sim`] of `scene` (the caller's, stand-ins off), `seed` and `first_tick`, with every slot's seeded token,
+    /// `clients` client slots, the lab's content hash and `event_start`, behind one decoder per session on `bites`'
+    /// `Quant`;
+    /// `bites` plants its World bite (armed at `arm_tick`), its room bite and its pump bite. `run_leg` passes
+    /// `Bites::default()` and event start 0, then sets its own decoder bite.
+    pub(crate) fn build(
+        scene: &Scene,
+        seed: u64,
+        first_tick: u32,
+        clients: usize,
+        event_start: u32,
+        arm_tick: u32,
+        bites: &Bites,
+    ) -> Server {
+        let world = bites.world(scene, seed, first_tick, arm_tick);
+        let greybox = world.greybox().clone();
+        let tokens = (0..scene.slots())
+            .map(|slot| credentials(seed, slot).0)
+            .collect();
+        let room = Room::new(world, scene, &greybox, tokens, clients, npt_sim::TICK_HZ)
+            .with_hashes(CONTENT, [0; 32])
+            .with_event_start(event_start);
+        let mut server = Server::new(bites.room(room), bites.quant(), RttSource::Link);
+        if bites.pump {
+            server.decoder_bite = bites.id.clone();
+        }
+        server
+    }
+
+    pub(crate) fn new(room: Room<Sim>, quant: Quant, rtt: RttSource) -> Server {
         Server {
             room,
             sessions: Vec::new(),
@@ -551,6 +585,40 @@ impl Server {
     }
 }
 
+/// What one instant's RQ8 steps 1 and 2 hand a loop, with step 3's due count (advisor-u4r-r1, EA-U4R-1, the narrow
+/// form): the admissions and the ended sessions since the last take, each for the caller's own sinks (the runner's
+/// `Logs::admit` and `Logs::gone`; the pre-check's maps and its outcome's `ended`), and the ticks due now.
+pub(crate) struct Wake {
+    pub(crate) admitted: Vec<(u8, u64, u32)>,
+    pub(crate) ended: Vec<(u8, &'static str)>,
+    pub(crate) to_run: u32,
+}
+
+/// RQ8's steps 1 and 2 at `now`, shared by [`run`] and the pre-check's `run_leg` (advisor-u4r-r1, EA-U4R-1, the narrow
+/// form): every new link's server end the listener hands out accepted (its `poll_accept` is the instant's first net
+/// call, which applies the packets due), the server's I/O step, and its admissions and ended sessions taken; then step
+/// 3's due count from `schedule`, which moves on. Each caller keeps its own per-tick body.
+pub(crate) fn wake(
+    server: &mut Server,
+    listener: &mut dyn Listener,
+    schedule: &mut Schedule,
+    now: u64,
+) -> Wake {
+    while let Some((link, _)) = listener.poll_accept(now) {
+        server.accept(link, now);
+    }
+    server.io(now);
+    let admitted = std::mem::take(&mut server.admitted);
+    let ended = std::mem::take(&mut server.ended);
+    let due = due_ticks(now, schedule);
+    *schedule = due.next;
+    Wake {
+        admitted,
+        ended,
+        to_run: due.to_run,
+    }
+}
+
 /// One run's settings (the `scenario` subcommand's options).
 #[derive(Clone, Debug)]
 pub(crate) struct Settings {
@@ -743,27 +811,16 @@ pub(crate) fn run(s: &Settings) -> Result<Summary, Failure> {
     let fail = Failure::Failed;
     let scene = s.scene.clone().with_stand_ins(StandIns::OFF);
     let n = s.clients.min(scene.slots());
-    let world = s.bites.world(&scene, s.seed, s.tick_start, s.arm_tick);
-    let greybox = world.greybox().clone();
-    let tokens = (0..scene.slots())
-        .map(|slot| credentials(s.seed, slot).0)
-        .collect();
-    let room = Room::new(
-        world,
+    let mut server = Server::build(
         &scene,
-        &greybox,
-        tokens,
+        s.seed,
+        s.tick_start,
         usize::from(n),
-        npt_sim::TICK_HZ,
-    )
-    .with_hashes(CONTENT, [0; 32])
-    .with_event_start(s.event_start);
-    let room = s.bites.room(room);
+        s.event_start,
+        s.arm_tick,
+        &s.bites,
+    );
     let quant = s.bites.quant();
-    let mut server = Server::new(room, quant, RttSource::Link);
-    if s.bites.pump {
-        server.decoder_bite = s.bites.id.clone();
-    }
     let mut schedule = s.bites.schedule();
     let net = s.bites.net(s.seed);
     let mut listener = net.listener();
@@ -879,13 +936,9 @@ pub(crate) fn run(s: &Settings) -> Result<Summary, Failure> {
     let mut stop = false;
     while !stop {
         let now = clock.now();
-        // (1) packets due, and each new link's server end.
-        while let Some((link, _)) = listener.poll_accept(now) {
-            server.accept(link, now);
-        }
-        // (2) the server's I/O step.
-        server.io(now);
-        for (slot, session, tick) in std::mem::take(&mut server.admitted) {
+        // (1) packets due, and each new link's server end; (2) the server's I/O step; then (3)'s due count.
+        let woke = wake(&mut server, listener.as_mut(), &mut schedule, now);
+        for (slot, session, tick) in woke.admitted {
             // Carry 75's bound is checked on logged runs only: `events_stamped` counts what `room_tick` returns, which
             // a no-log run (--logs 0, or `npt-lab hash`'s regeneration) does not call, so there it stays 0
             // (advisor-u4r-r1, FP-7 item 6: RR3-L2 (b)).
@@ -897,13 +950,11 @@ pub(crate) fn run(s: &Settings) -> Result<Summary, Failure> {
             }
             logs.admit(slot, session, tick, phase(k)).map_err(fail)?;
         }
-        for (slot, _) in std::mem::take(&mut server.ended) {
+        for (slot, _) in woke.ended {
             logs.gone(slot);
         }
         // (3) the server tick, if due.
-        let due = due_ticks(now, &schedule);
-        schedule = due.next;
-        for _ in 0..due.to_run {
+        for _ in 0..woke.to_run {
             let ph = phase(k);
             let t = server.room.last_tick().wrapping_add(1);
             server.room.tick();
@@ -1376,9 +1427,18 @@ pub(crate) fn parse(argv: &[String]) -> Result<Settings, String> {
     let measured_end = warmup_ticks
         .checked_add(measured_ticks)
         .ok_or("--warmup-s and --measured-s: their ticks' sum overflows 64 bits".to_owned())?;
-    measured_end
+    let last_tick = measured_end
         .checked_add(after_max_ticks)
         .ok_or("--after-max-s: the run's last tick overflows 64 bits".to_owned())?;
+    // advisor-u4r-r2, RR4-L2, after the 64-bit checks (which keep their messages): the run's last tick, counted from
+    // its first (the warm-up, measured and after phases' ticks), fits 32 bits, on every profile.
+    if last_tick > u64::from(u32::MAX) {
+        return Err(format!(
+            "--warmup-s, --measured-s and --after-max-s: the run's last tick, {last_tick} ticks from its first, is \
+             above {} (32 bits)",
+            u32::MAX
+        ));
+    }
     let behaviour = take("behaviour").unwrap_or_else(|| "brawl".to_owned());
     if !["brawl", "presses", "clamp", "steady"].contains(&behaviour.as_str()) {
         return Err(format!(
@@ -1439,9 +1499,12 @@ pub(crate) fn parse(argv: &[String]) -> Result<Settings, String> {
                 .to_owned(),
         );
     }
-    // The last burst's tick, checked before `burst_starts` sums it (FP-3: RR3-L3), which bounds every sum there.
+    // Where bursts are used (B1 with at least one: `burst_starts` runs only there), what is bounded: the last burst's
+    // tick boundary counted from the run's first tick, ((bursts - 1) x every_s + first_s) x TICK_HZ + the warm-up's
+    // ticks, first in 64 bits, which bounds every sum `burst_starts` makes (FP-3: RR3-L3); then (advisor-u4r-r2,
+    // RR4-L2) that tick within 32 bits, --bursts at most 1,000, and --burst-every-s above 0 with more than one burst.
     if profile_name == "B1" && bursts > 0 {
-        (bursts - 1)
+        let last_burst = (bursts - 1)
             .checked_mul(burst_every_s)
             .and_then(|x| x.checked_add(burst_first_s))
             .and_then(|x| x.checked_mul(hz))
@@ -1450,6 +1513,21 @@ pub(crate) fn parse(argv: &[String]) -> Result<Settings, String> {
                 "--bursts {bursts}, --burst-first-s {burst_first_s} and --burst-every-s {burst_every_s}: the last \
                  burst's tick overflows 64 bits"
             ))?;
+        if last_burst > u64::from(u32::MAX) {
+            return Err(format!(
+                "--bursts {bursts}, --burst-first-s {burst_first_s} and --burst-every-s {burst_every_s}: the last \
+                 burst's tick, {last_burst} ticks from the run's first, is above {} (32 bits)",
+                u32::MAX
+            ));
+        }
+        if bursts > 1_000 {
+            return Err(format!("--bursts {bursts}: above 1000"));
+        }
+        if bursts > 1 && burst_every_s == 0 {
+            return Err(format!(
+                "--burst-every-s 0 with --bursts {bursts}: more than one burst needs an interval above 0 s"
+            ));
+        }
     }
     let profile = match profile_name.as_str() {
         "B1" => Profile::b1(&burst_starts(
@@ -1546,7 +1624,7 @@ pub(crate) fn summary_line(s: &Settings, o: &Summary) -> String {
          client hashes {} (sentinels {}); measured incomplete {}; decided lost at the stop {}; shots per measured \
          second {}; carry 49: measured client sets {}, holding every very large unit {}, projectiles per set {}; \
          audit {} owner, {} spatial checked against the radius, {} spatial not checked, {} breaches; fate log {}; \
-         connect order digest {:016x}",
+         connect order digest {:016x}; in a building: {}",
         s.meta.run,
         s.scene.name(),
         s.profile.name,
@@ -1595,6 +1673,20 @@ pub(crate) fn summary_line(s: &Settings, o: &Summary) -> String {
         o.audit.breaches_count,
         &o.fate_log_sha256[..16.min(o.fate_log_sha256.len())],
         o.order_digest,
+        // BRAWL-1's figure: measured slot-ticks with feet in a building area, and their slots.
+        logged(format!(
+            "{} slot-ticks (slots {})",
+            c.in_building_measured,
+            if c.in_building_slots.is_empty() {
+                "none".to_owned()
+            } else {
+                c.in_building_slots
+                    .iter()
+                    .map(u8::to_string)
+                    .collect::<Vec<_>>()
+                    .join(", ")
+            }
+        )),
     )
 }
 
@@ -2018,19 +2110,7 @@ pub(crate) mod tests {
 
     fn lab_room(n: u8) -> Server {
         let scene = Scene::stress().with_stand_ins(StandIns::OFF);
-        let world = World::new(&scene, 1, 0);
-        let greybox = world.greybox().clone();
-        let tokens = (0..scene.slots()).map(|s| credentials(1, s).0).collect();
-        let room = Room::new(
-            world,
-            &scene,
-            &greybox,
-            tokens,
-            usize::from(n),
-            npt_sim::TICK_HZ,
-        )
-        .with_hashes(CONTENT, [0; 32]);
-        Server::new(room, Quant::default(), RttSource::Link)
+        Server::build(&scene, 1, 0, usize::from(n), 0, 0, &Bites::default())
     }
 
     /// r8 (carry 76): a second endpoint joins with slot 0's token and bind while the first is live; the first is sent
@@ -2145,8 +2225,9 @@ pub(crate) mod tests {
         );
     }
 
-    /// FP-1's three checks of a session that ended closed on `slot`: the room's slot freed, no owner, and `(slot,
-    /// "closed")` in `ended`; each that fails, by name.
+    /// FP-1's four checks of a session that ended closed on `slot`: the room's slot freed, no owner, `(slot, "closed")`
+    /// in `ended`, and the roster's slot not live (advisor-u4r-r2, N2-1 = TE-R2-1: a rejoin then is no replacement);
+    /// each that fails, by name.
     fn closed_checks(server: &Server, slot: u8) -> Vec<String> {
         let mut failed = Vec::new();
         if server.room.client(slot).is_some() {
@@ -2160,13 +2241,20 @@ pub(crate) mod tests {
         if !server.ended.contains(&(slot, "closed")) {
             failed.push(format!("ended holds no ({slot}, \"closed\")"));
         }
+        if server.room.roster().is_live(slot) {
+            failed.push(format!(
+                "the roster still holds slot {slot}: a rejoin would be a replacement"
+            ));
+        }
         failed
     }
 
     /// Carry 76's boundary (advisor-u4r-r1, FP-1: NC-U4R-1 = SEC-U4R-1): a session whose decoder closes, (a) on a
     /// control-stream frame it refuses as malformed, or whose link closes, (b) its client end dropped, ends, and the
     /// room's slot is freed (`Room::leave`: "a closed link"); after (a) a new endpoint with that slot's token and bind is
-    /// admitted, and not as a replacement. Each case's checks are collected, so a failure names its case.
+    /// admitted. It is not a replacement because the roster freed the slot (`closed_checks`' roster check, read before
+    /// the rejoin: `Room::join` reports a replacement only for a slot the roster holds live). Each case's checks are
+    /// collected, so a failure names its case.
     #[test]
     fn fp1_a_closed_decoder_or_link_ends_the_session_and_frees_the_room_s_slot() {
         // (a) A frame the pump refuses with an error other than WrongChannel or WrongDirection: `close_malformed`.
@@ -2206,10 +2294,6 @@ pub(crate) mod tests {
             Some(&again),
             "(a): the new endpoint is admitted"
         );
-        assert!(
-            !a.ended.contains(&(0, "replaced")),
-            "(a): the new endpoint is no replacement"
-        );
     }
 
     /// The reliable messages' channel (the module text): exactly one stream channel takes each, never a fall-back.
@@ -2418,16 +2502,89 @@ pub(crate) mod tests {
     }
 
     /// advisor-u4r-r1, FP-3 (SEC-U4R-3, RR3-L3): a length whose ticks overflow 64 bits is a usage error naming its
-    /// option, never a wrapped length.
+    /// option, never a wrapped length. advisor-u4r-r2, TE-R2-2: each 64-bit refusal's exact text (the last burst's
+    /// tick on a B1 line, the warm-up and measured sum, the after phase); RR4-L2: a run's last tick and the last
+    /// burst's tick above u32::MAX, more than 1,000 bursts and an interval of 0 s between bursts, each a usage error
+    /// naming its options, and each bound's own value accepted.
     #[test]
     fn fp3_a_length_whose_ticks_overflow_is_a_usage_error_naming_its_option() {
-        let line = format!(
-            "--run t --set t --date 2026-11-14 --commit {COMMIT} --out x --scene SS --seed 1 --profile P0 \
-             --measured-s 1000000000000000000"
-        );
-        let argv: Vec<String> = line.split_whitespace().map(str::to_owned).collect();
-        let e = parse(&argv).expect_err("--measured-s 10^18 x 30 overflows");
+        let parsed = |profile: &str, extra: &str| {
+            let line = format!(
+                "--run t --set t --date 2026-11-14 --commit {COMMIT} --out x --scene SS --seed 1 --profile {profile} \
+                 {extra}"
+            );
+            let argv: Vec<String> = line.split_whitespace().map(str::to_owned).collect();
+            parse(&argv)
+        };
+        let e = parsed("P0", "--measured-s 1000000000000000000")
+            .expect_err("--measured-s 10^18 x 30 overflows");
         assert!(e.starts_with("--measured-s 1000000000000000000: "), "{e}");
+        // TE-R2-2 (1), on a B1 line: the 64-bit check comes before RR4-L2's bounds.
+        let e = parsed(
+            "B1",
+            "--bursts 2 --burst-first-s 0 --burst-every-s 1000000000000000000",
+        )
+        .expect_err("the last burst's tick overflows");
+        assert!(
+            e.starts_with(
+                "--bursts 2, --burst-first-s 0 and --burst-every-s 1000000000000000000: the last burst's tick \
+                 overflows 64 bits"
+            ),
+            "{e}"
+        );
+        // TE-R2-2 (2): each product fits, their sum does not.
+        let e = parsed(
+            "P0",
+            "--warmup-s 310000000000000000 --measured-s 310000000000000000",
+        )
+        .expect_err("the sum overflows");
+        assert_eq!(
+            e,
+            "--warmup-s and --measured-s: their ticks' sum overflows 64 bits"
+        );
+        // TE-R2-2 (3), in full digits (`num` parses u64): the sum fits, the after phase's addition does not.
+        let e = parsed(
+            "P0",
+            "--warmup-s 300000000000000000 --measured-s 300000000000000000 --after-max-s 100000000000000000",
+        )
+        .expect_err("the run's last tick overflows");
+        assert_eq!(e, "--after-max-s: the run's last tick overflows 64 bits");
+        // RR4-L2's ruled case: the last burst's tick fits 64 bits, not 32.
+        let e = parsed("B1", "--bursts 2 --burst-every-s 600000000000")
+            .expect_err("the last burst's tick above 32 bits");
+        assert!(e.contains("--burst-every-s 600000000000"), "{e}");
+        assert_eq!(
+            e,
+            "--bursts 2, --burst-first-s 30 and --burst-every-s 600000000000: the last burst's tick, 18000000001200 \
+             ticks from the run's first, is above 4294967295 (32 bits)"
+        );
+        // RR4-L2: more than 1,000 bursts; 1,000 accepted.
+        let e = parsed("B1", "--bursts 1001").expect_err("1,001 bursts");
+        assert_eq!(e, "--bursts 1001: above 1000");
+        assert!(parsed("B1", "--bursts 1000").is_ok(), "1,000 bursts");
+        // RR4-L2: an interval of 0 s with more than one burst; one burst at 0 s accepted.
+        let e = parsed("B1", "--bursts 2 --burst-every-s 0").expect_err("two bursts 0 s apart");
+        assert_eq!(
+            e,
+            "--burst-every-s 0 with --bursts 2: more than one burst needs an interval above 0 s"
+        );
+        assert!(
+            parsed("B1", "--bursts 1 --burst-every-s 0").is_ok(),
+            "one burst"
+        );
+        // RR4-L2: a run's last tick past 32 bits that fits 64 (the defaults' 300 + 1,800 ticks, then the after phase);
+        // u32::MAX itself or less accepted.
+        let e =
+            parsed("P0", "--after-max-s 143165507").expect_err("the run's last tick above 32 bits");
+        assert_eq!(
+            e,
+            "--warmup-s, --measured-s and --after-max-s: the run's last tick, 4294967310 ticks from its first, is \
+             above 4294967295 (32 bits)"
+        );
+        assert!(
+            parsed("P0", "--after-max-s 143165506").is_ok(),
+            "4,294,967,280 ticks"
+        );
     }
 
     /// advisor-u4r-r1, FP-6 items 1 and 2 (NC-U4R-2; EA-U4R-3 = RR3-L8; bites builds only): `scenario` refuses the
````

### npt-lab: logs.rs
_`git diff --no-index spikes/netcode-proof/crates/lab/src/logs.rs .claude/worktrees/brawl1/spikes/netcode-proof/crates/lab/src/logs.rs`:  1 file changed, 75 insertions(+), 7 deletions(-)._

````diff
diff --git a/spikes/netcode-proof/crates/lab/src/logs.rs b/.claude/worktrees/brawl1/spikes/netcode-proof/crates/lab/src/logs.rs
index a104ea4..724dd4f 100644
--- a/spikes/netcode-proof/crates/lab/src/logs.rs
+++ b/.claude/worktrees/brawl1/spikes/netcode-proof/crates/lab/src/logs.rs
@@ -1,6 +1,7 @@
 //! The scenario's three logs (u4-build-plan.md §4.12 :410-415; marks/README.md §1 and §4; u4r-brief.md 2.1): on L1
 //! one npt-lab process writes one log per process value the checks select on, each with its own header: `room` (the
-//! room's per-tick World hash `h`, `hs.server_hash`, PM-0001-2's series, the `admit` events, `burst_eviction`), `client`
+//! room's per-tick World hash `h`, `hs.server_hash`, PM-0001-2's series, BRAWL-1's `in_building_slot_area`, the
+//! `admit` events, `burst_eviction`), `client`
 //! (every client's `hs.decoded_hash` and `tick.s.snap_incomplete`, one entry per slot) and `lab` (-57's errors,
 //! `burst_heal`, `counter_wrap`, the run's own records). Every tick and second record carries all its counters, 0
 //! included; every log ends with its footer.
@@ -21,9 +22,10 @@ use std::io::BufWriter;
 
 use npt_log::{Ended, EventRecord, Header, Phase, SCHEMA, SecondRecord, TickRecord, Writer};
 use npt_room::Room;
-use npt_sim::{EntityKind, World};
+use npt_sim::EntityKind;
 
 use crate::mirror::{Errors, Records, SetStats};
+use crate::scenario::Sim;
 
 /// The fields every log of a run shares, and its params.
 #[derive(Clone, Debug)]
@@ -94,7 +96,7 @@ fn io(e: std::io::Error) -> String {
 /// and kills). `at` caches the section's index (the sections before it are fixed-size pools). The advisor's U4r build
 /// ruling, point 6, pins it against `SimCore::step`'s returned events (the test below).
 pub(crate) fn dump_events(
-    world: &World,
+    world: &Sim,
     at: &mut Option<usize>,
 ) -> Result<(usize, i64, i64), String> {
     let dump = world.dump();
@@ -142,6 +144,9 @@ pub(crate) struct Logs {
     room: Out,
     client: Out,
     lab: Out,
+    /// Whether the logs have a folder (`open`'s `dir`): a run with none keeps no client resolution (advisor-u4r-r1,
+    /// RR3-L2 (a): `filed` then keeps only what `started` and the stop read).
+    logging: bool,
     n: usize,
     first_tick: u32,
     phases: BTreeMap<u64, Phase>,
@@ -182,6 +187,9 @@ pub(crate) struct Counts {
     pub(crate) large_measured: u64,
     pub(crate) projectiles_measured: u64,
     pub(crate) sets_measured: u64,
+    /// BRAWL-1's figure: measured slot-ticks with feet in a building area, and the slots that had any.
+    pub(crate) in_building_measured: u64,
+    pub(crate) in_building_slots: std::collections::BTreeSet<u8>,
 }
 
 impl Logs {
@@ -199,6 +207,7 @@ impl Logs {
             room: open(dir, "room", meta)?,
             client: open(dir, "client", meta)?,
             lab: open(dir, "lab", meta)?,
+            logging: dir.is_some(),
             n,
             first_tick,
             phases: BTreeMap::new(),
@@ -321,12 +330,28 @@ impl Logs {
         as_lost
     }
 
-    /// What one client step filed: (tick, hash) pairs (0 the sentinel) and the runs of ticks it resolved lost.
+    /// What one client step filed: (tick, hash) pairs (0 the sentinel) and the runs of ticks it resolved lost. A run
+    /// with no log folder only resolves each tick (its first and last resolved ticks, which `started` and the stop
+    /// read) and keeps no per-tick resolution (advisor-u4r-r1, RR3-L2 (a)): nothing writes there, and with no room
+    /// tick logged no tick is ever written, so neither the conflict count nor the whole-as-lost bite can act.
     pub(crate) fn filed(&mut self, slot: u8, hashes: &[(u32, u64)], lost: &[(u32, u32)]) {
         let s = usize::from(slot);
         if s >= self.n {
             return;
         }
+        if !self.logging {
+            for &(t, _) in hashes {
+                let w = self.wide(t);
+                self.resolved(s, w);
+            }
+            for &(first, count) in lost {
+                for k in 0..count {
+                    let w = self.wide(first.wrapping_add(k));
+                    self.resolved(s, w);
+                }
+            }
+            return;
+        }
         for &(t, h) in hashes {
             let w = self.wide(t);
             if self.written.is_some_and(|m| w <= m) {
@@ -469,7 +494,7 @@ impl Logs {
     /// tick. Returns the events the World stamped at the tick.
     pub(crate) fn room_tick(
         &mut self,
-        room: &Room<World>,
+        room: &Room<Sim>,
         t: u32,
         ph: Phase,
         sets: &[SetStats],
@@ -556,6 +581,22 @@ impl Logs {
             .map(|s| i64::from(sets.get(s).is_some_and(|x| x.everyone)))
             .collect();
         rec.s.insert("everyone_in_view".into(), flags);
+        // BRAWL-1's figure (Fix (b)): each soldier whose feet are in a building area (its record's `area`, the sim's
+        // own: building i is area i + 1), as (slot, area) pairs, on every logged tick with one, in any phase.
+        let in_building: Vec<i64> = records
+            .iter()
+            .filter(|r| r.kind == EntityKind::Soldier && r.area != 0)
+            .flat_map(|r| [i64::from(r.index), i64::from(r.area)])
+            .collect();
+        if !in_building.is_empty() {
+            if ph == Phase::Measured {
+                self.counts.in_building_measured += in_building.len() as u64 / 2;
+                self.counts
+                    .in_building_slots
+                    .extend(in_building.iter().step_by(2).map(|&s| s as u8));
+            }
+            rec.s.insert("in_building_slot_area".into(), in_building);
+        }
         rec.s.insert(
             "snap_datagram_bytes".into(),
             datagrams.iter().flatten().map(|b| *b as i64).collect(),
@@ -592,7 +633,7 @@ impl Logs {
     /// list and records (the dump's `events` section: npt-sim's canonical state), portal changes from the doors.
     fn room_second(
         &mut self,
-        world: &World,
+        world: &Sim,
         t: u32,
         ph: Phase,
         records: &[npt_sim::EntityRecord],
@@ -743,7 +784,7 @@ mod tests {
     #[test]
     fn p6_the_dump_reader_counts_the_shots_hits_and_kills_simcore_step_returns() {
         let scene = Scene::stress().with_stand_ins(StandIns::OFF);
-        let mut world = World::new(&scene, 3, 0);
+        let mut world = Sim::new(&scene, 3, 0);
         let history = HitHistory::for_ms(2_000).expect("a ring");
         let mut at = None;
         let (mut shots, mut hits, mut kills) = (0i64, 0i64, 0i64);
@@ -800,4 +841,31 @@ mod tests {
         );
         assert!(shots > 0 && hits > 0 && kills > 0, "{shots} {hits} {kills}");
     }
+
+    /// advisor-u4r-r1, RR3-L2 (a): a run with no log folder keeps no per-tick client resolution. After an admission
+    /// and one client's filings over 9,000 ticks (hashes, sentinels and lost runs), `res` is empty, the slot has
+    /// started, and its first and last resolved ticks are the filings' first and last.
+    #[test]
+    fn rr3_l2_a_a_run_with_no_log_folder_keeps_no_per_tick_resolution() {
+        let meta =
+            crate::scenario::tests::settings(std::path::Path::new("-"), "--profile P0 --seed 1")
+                .meta;
+        let first = 63_736u32;
+        let mut logs = Logs::open(None, &meta, first, false).expect("three sinks");
+        logs.admit(3, 0, first, Phase::Warmup)
+            .expect("the admission");
+        assert!(!logs.started(3), "admitted, nothing resolved yet");
+        for k in 0..9_000u32 {
+            let t = first.wrapping_add(k);
+            match k % 10 {
+                0 => logs.filed(3, &[], &[(t, 1)]),
+                1 => logs.filed(3, &[(t, 0)], &[]),
+                _ => logs.filed(3, &[(t, u64::from(k) + 1)], &[]),
+            }
+        }
+        assert!(logs.res.is_empty(), "{} ticks kept", logs.res.len());
+        assert!(logs.started(3), "admitted and resolved");
+        let w = u64::from(first);
+        assert_eq!((logs.first[3], logs.last[3]), (Some(w), Some(w + 8_999)));
+    }
 }
````

### npt-lab: precheck.rs
_`git diff --no-index spikes/netcode-proof/crates/lab/src/precheck.rs .claude/worktrees/brawl1/spikes/netcode-proof/crates/lab/src/precheck.rs`:  1 file changed, 19 insertions(+), 33 deletions(-)._

````diff
diff --git a/spikes/netcode-proof/crates/lab/src/precheck.rs b/.claude/worktrees/brawl1/spikes/netcode-proof/crates/lab/src/precheck.rs
index 8daf08e..09a4439 100644
--- a/spikes/netcode-proof/crates/lab/src/precheck.rs
+++ b/.claude/worktrees/brawl1/spikes/netcode-proof/crates/lab/src/precheck.rs
@@ -43,15 +43,12 @@ use npt_client::ClientEndpoint;
 use npt_l1::{L1Net, Profile};
 use npt_lab::{HashCompare, TickHash};
 use npt_link::{Channel, Link};
-use npt_proto::{Direction, Input, Limits, Message, Quant};
-use npt_room::Room;
-use npt_server::{RttSource, Schedule, due_ticks};
+use npt_proto::{Direction, Input, Limits, Message};
+use npt_server::Schedule;
 use npt_sim::replay::Recording;
-use npt_sim::{Command, Scene, StandIns, World};
+use npt_sim::{Command, Scene, StandIns};
 
-use crate::scenario::{
-    Bites, CONTENT, MAX_DATAGRAM, Server, check_placements, credentials, join_of,
-};
+use crate::scenario::{Bites, MAX_DATAGRAM, Server, SimClock, check_placements, join_of, wake};
 
 /// The pre-check's suite name (7.2).
 pub(crate) const SUITE: &str = "precheck-30";
@@ -293,21 +290,15 @@ pub(crate) fn run_leg(
         .ok_or(format!("no scene {}", rec.scene))?
         .with_stand_ins(StandIns::OFF);
     let cmds = commands_of(rec)?;
-    let world = World::new(&scene, rec.seed, rec.first_tick);
-    let greybox = world.greybox().clone();
-    let tokens = (0..scene.slots())
-        .map(|slot| credentials(rec.seed, slot).0)
-        .collect();
-    let room = Room::new(
-        world,
+    let mut server = Server::build(
         &scene,
-        &greybox,
-        tokens,
+        rec.seed,
+        rec.first_tick,
         cmds.len(),
-        npt_sim::TICK_HZ,
-    )
-    .with_hashes(CONTENT, [0; 32]);
-    let mut server = Server::new(room, Quant::default(), RttSource::Link);
+        0,
+        rec.first_tick,
+        &Bites::default(),
+    );
     if let Some(id) = bite {
         let b = Bites::of(id)?;
         if b.pump {
@@ -369,24 +360,19 @@ pub(crate) fn run_leg(
     let mut first_take: BTreeMap<u8, u32> = BTreeMap::new();
     let t0 = rec.ticks.first().map_or(rec.first_tick, |t| t.tick);
     let mut last_seq: BTreeMap<u8, u16> = BTreeMap::new();
-    let mut now = 0u64;
+    let mut clock = SimClock::default();
     let mut idx = 0usize;
     while idx < rec.ticks.len() {
-        // (1) packets due, and each new link's server end (L1).
-        while let Some((link, _)) = listener.poll_accept(now) {
-            server.accept(link, now);
-        }
-        // (2) the server's I/O step.
-        server.io(now);
-        for (slot, _, tick) in std::mem::take(&mut server.admitted) {
+        let now = clock.now();
+        // (1) packets due, and each new link's server end (L1); (2) the server's I/O step; then (3)'s due count.
+        let woke = wake(&mut server, listener.as_mut(), &mut schedule, now);
+        for (slot, _, tick) in woke.admitted {
             admitted.entry(slot).or_insert(tick);
         }
-        o.ended.append(&mut server.ended);
+        o.ended.extend(woke.ended);
         // (3) the server tick, if due.
-        let due = due_ticks(now, &schedule);
-        schedule = due.next;
         let mut ran = None;
-        for _ in 0..due.to_run {
+        for _ in 0..woke.to_run {
             let Some(ti) = rec.ticks.get(idx) else { break };
             check_placements(&ti.spawns, &ti.teleports)?;
             o.scripted += ti.spawns.len() + ti.teleports.len() + ti.attributes.len();
@@ -472,7 +458,7 @@ pub(crate) fn run_leg(
         if let Some(e) = net.next_event_ns() {
             next = next.min(e.max(now + 1));
         }
-        now = next.max(now + 1);
+        clock.advance_to(next.max(now + 1));
     }
     for c in &clients {
         let k = server
````

### npt-lab: hash.rs
_`git diff --no-index spikes/netcode-proof/crates/lab/src/hash.rs .claude/worktrees/brawl1/spikes/netcode-proof/crates/lab/src/hash.rs`:  1 file changed, 35 insertions(+), 7 deletions(-)._

````diff
diff --git a/spikes/netcode-proof/crates/lab/src/hash.rs b/.claude/worktrees/brawl1/spikes/netcode-proof/crates/lab/src/hash.rs
index 7212591..85e70a0 100644
--- a/spikes/netcode-proof/crates/lab/src/hash.rs
+++ b/.claude/worktrees/brawl1/spikes/netcode-proof/crates/lab/src/hash.rs
@@ -232,6 +232,11 @@ const TICK_HEADER_BYTES: u64 = 12;
 /// The most bytes an entry's recording may hold: 64 MiB, four times today's largest entry (SS-P0-s4, 16,950,444 B); a
 /// larger recording comes to the advisor first (advisor-u4r-r1, FP-3: RR3-L1).
 const MAX_ENTRY_BYTES: u64 = 64 * 1024 * 1024;
+/// The most ticks an entry's recording may hold: four times today's largest entry (9,000), the same rule as the bytes
+/// cap; a longer recording comes to the advisor first (advisor-u4r-r2, RR4-L1). Checked before the runner checks, so a
+/// runner entry above it is refused by this rule, not by its ticks' own check. A manifest's size bound, a literal as
+/// MAX_ENTRY_BYTES is, so not named `*_TICKS`: check.mjs reads that name as a rate-derived tick count (TICK_NAMED).
+const MAX_ENTRY_TICK_COUNT: u32 = 36_000;
 /// The profiles a runner entry may name (`npt-lab scenario --profile`'s), each checked by name before its settings
 /// reach `scenario::record` (advisor-u4r-r1, FP-3: RR3-M1 = SEC-U4R-2).
 const RUNNER_PROFILES: [&str; 6] = ["P0", "P1", "P2", "P3", "P4", "B1"];
@@ -242,9 +247,10 @@ fn name_ok(name: &str) -> bool {
     !name.is_empty() && name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
 }
 
-/// The manifest's entries, in its order. Each entry's name, its `ticks` against its `bytes` and its `bytes` against 64
-/// MiB are checked here, before anything is generated (carry 82), and a runner entry's profile by name, its `ticks`
-/// against its warm-up and measured seconds, and its `span_ms` against the room's (advisor-u4r-r1, FP-3).
+/// The manifest's entries, in its order. Each entry's name, its `ticks` against its `bytes`, its `bytes` against 64
+/// MiB and its `ticks` against 36,000 (advisor-u4r-r2, RR4-L1) are checked here, before anything is generated (carry
+/// 82), and a runner entry's profile by name, its `ticks` against its warm-up and measured seconds, and its `span_ms`
+/// against the room's (advisor-u4r-r1, FP-3).
 pub(crate) fn manifest(text: &str) -> Result<Vec<Entry>, String> {
     let v: Value =
         serde_json::from_str(text).map_err(|e| format!("the manifest is not JSON: {e}"))?;
@@ -329,6 +335,13 @@ pub(crate) fn manifest(text: &str) -> Result<Vec<Entry>, String> {
                 entry.name, entry.bytes
             ));
         }
+        if entry.ticks > MAX_ENTRY_TICK_COUNT {
+            return Err(format!(
+                "manifest entry {}: ticks {} is above {MAX_ENTRY_TICK_COUNT} (four times today's largest entry): a \
+                 longer recording comes to the advisor first",
+                entry.name, entry.ticks
+            ));
+        }
         if let Kind::Runner {
             profile,
             warmup_s,
@@ -1545,12 +1558,14 @@ mod tests {
             let e = manifest(&one(bad, 9_000, k)).expect_err(bad);
             assert!(e.contains("is not one or more of"), "{bad:?}: {e}");
         }
-        let at = 799_693 / 12;
+        // At the rule's boundary below RR4-L1's 36,000-tick cap (advisor-u4r-r2): 360,000 bytes hold at most 30,000.
+        let small = |text: String| text.replace("\"bytes\": 799693", "\"bytes\": 360000");
+        let at = 360_000 / 12;
         assert!(
-            manifest(&one("TC-s3", at, k)).is_ok(),
+            manifest(&small(one("TC-s3", at, k))).is_ok(),
             "ticks at bytes / 12"
         );
-        let e = manifest(&one("TC-s3", at + 1, k)).expect_err("ticks above bytes / 12");
+        let e = manifest(&small(one("TC-s3", at + 1, k))).expect_err("ticks above bytes / 12");
         assert!(e.contains("cannot fit"), "{e}");
         assert!(manifest(&one("TC-s3", 9_000, "")).is_err(), "no kind");
         assert!(
@@ -1589,7 +1604,8 @@ mod tests {
 
     /// advisor-u4r-r1, FP-3 (RR3-M1 = SEC-U4R-2, SEC-U4R-3, RR3-L1): before anything is generated, a runner entry's
     /// profile is checked by name, its ticks against (warmup_s + measured_s) x TICK_HZ and its span_ms against the room's,
-    /// and every entry's bytes against 64 MiB; each refusal names the entry and the value. The committed manifests read.
+    /// and every entry's bytes against 64 MiB and (advisor-u4r-r2, RR4-L1) its ticks against 36,000; each refusal names
+    /// the entry and the value. The committed manifests read.
     #[test]
     fn r10_fp3_a_runner_entry_s_profile_ticks_and_span_and_any_entry_s_bytes_are_checked() {
         let text = "{\"recordings\": [{\"name\": \"SS-P2-s1\", \"kind\": \"runner\", \"profile\": \"P2\", \
@@ -1635,6 +1651,18 @@ mod tests {
             manifest(&text.replace("\"bytes\": 999999", "\"bytes\": 67108864")).is_ok(),
             "bytes at 64 MiB"
         );
+        // advisor-u4r-r2, RR4-L1: 36,001 ticks refused by name, by this rule before the runner's ticks check (999,999 /
+        // 12 = 83,333 ticks fit the bytes); a synthetic entry at 36,000 read (799,693 / 12 = 66,641 fit).
+        let e = refused("\"ticks\": 9000", "\"ticks\": 36001", "ticks 36,001");
+        assert_eq!(
+            e,
+            "manifest entry SS-P2-s1: ticks 36001 is above 36000 (four times today's largest entry): a longer \
+             recording comes to the advisor first"
+        );
+        assert!(
+            manifest(&one("TC-s3", 36_000, "\"kind\": \"synthetic\",")).is_ok(),
+            "a synthetic entry at 36,000 ticks"
+        );
         for (name, committed) in [
             ("recordings.json", include_str!("../recordings.json")),
             (
````

### npt-lab's runner recordings manifest for det-u4.mjs (crates/lab/det-recordings.json)
_`git diff --no-index spikes/netcode-proof/crates/lab/det-recordings.json .claude/worktrees/brawl1/spikes/netcode-proof/crates/lab/det-recordings.json`:  1 file changed, 2 insertions(+), 2 deletions(-)._

````diff
diff --git a/spikes/netcode-proof/crates/lab/det-recordings.json b/.claude/worktrees/brawl1/spikes/netcode-proof/crates/lab/det-recordings.json
index e6cd904..7d4961a 100644
--- a/spikes/netcode-proof/crates/lab/det-recordings.json
+++ b/.claude/worktrees/brawl1/spikes/netcode-proof/crates/lab/det-recordings.json
@@ -1,7 +1,7 @@
 {
   "about": "U4r's two runner recordings (u4-build-plan.md §4.15 :494-501; u4r-brief.md 2.1 (ii); carry 82's schema, recordings.json's), in a manifest of their own by the advisor's U4r build ruling, point 1 (advisor-u4r-build1, route (c)): ops/runs/det-u4.mjs runs check.mjs's `wasm` step's clean half on this file (regenerate and SHA-check, native leg 1 against leg 2, Wasm leg 1 against native leg 1), while check.mjs's step keeps recordings.json's three synthetic recordings. Each entry is regenerated by `npt-lab scenario --record` at its scene, seed, first tick, profile, clients, warmup_s and measured_s (the brawl on L1, generator 0), read by `npt-lab hash`'s same reader. Scene ids: 1 SS, 2 ES, 3 CO, 4 TC (Scene::by_id).",
   "recordings": [
-    {"name": "SS-P2-s1", "kind": "runner", "profile": "P2", "clients": 48, "warmup_s": 10, "measured_s": 290, "scene": 1, "seed": 1, "first_tick": 0, "ticks": 9000, "span_ms": 2000, "generator": 0, "bytes": 16937652, "sha256": "5b79abb2a2cba55b06a8916b235aea1766960867f09a038aed146805a29a32e3"},
-    {"name": "ES-P3-s2", "kind": "runner", "profile": "P3", "clients": 48, "warmup_s": 10, "measured_s": 290, "scene": 2, "seed": 2, "first_tick": 1000000, "ticks": 9000, "span_ms": 2000, "generator": 0, "bytes": 16929501, "sha256": "fad8f67ded2a220e42d7e4d42a3d3f2f76e980d4c55abb4fbcd01c550045953b"}
+    {"name": "SS-P2-s1", "kind": "runner", "profile": "P2", "clients": 48, "warmup_s": 10, "measured_s": 290, "scene": 1, "seed": 1, "first_tick": 0, "ticks": 9000, "span_ms": 2000, "generator": 0, "bytes": 16937652, "sha256": "1ec25de7c9ce8ea869ed9a8b343746ba131593e1de5c65bc9c1c626ce01cc76d"},
+    {"name": "ES-P3-s2", "kind": "runner", "profile": "P3", "clients": 48, "warmup_s": 10, "measured_s": 290, "scene": 2, "seed": 2, "first_tick": 1000000, "ticks": 9000, "span_ms": 2000, "generator": 0, "bytes": 16929501, "sha256": "6abf7643982b5970a630c97daa4d0815ceb2022958b18e18116397dcad9c893e"}
   ]
 }
````

### npt-lab's pre-check recordings manifest (crates/lab/precheck-recordings.json)
_`git diff --no-index spikes/netcode-proof/crates/lab/precheck-recordings.json .claude/worktrees/brawl1/spikes/netcode-proof/crates/lab/precheck-recordings.json`:  1 file changed, 1 insertion(+), 1 deletion(-)._

````diff
diff --git a/spikes/netcode-proof/crates/lab/precheck-recordings.json b/.claude/worktrees/brawl1/spikes/netcode-proof/crates/lab/precheck-recordings.json
index 9c4b6c5..3853618 100644
--- a/spikes/netcode-proof/crates/lab/precheck-recordings.json
+++ b/.claude/worktrees/brawl1/spikes/netcode-proof/crates/lab/precheck-recordings.json
@@ -1,6 +1,6 @@
 {
   "about": "The -30 pre-check's recording (u4-build-plan.md §4.15 :500-501: SS P0 seed 4 serves the pre-check only; u4r-brief.md 2.1 (i)): a runner recording in recordings.json's schema (carry 82), read by `npt-lab hash`'s same reader and regenerated by the pre-check's script before it replays it: `npt-lab scenario --record` at these settings (the brawl on L1, 48 clients, 10 s warm-up and 290 s measured: 9,000 ticks), its byte length and SHA-256 checked here. check.mjs's `wasm` step never reads this file.",
   "recordings": [
-    {"name": "SS-P0-s4", "kind": "runner", "profile": "P0", "clients": 48, "warmup_s": 10, "measured_s": 290, "scene": 1, "seed": 4, "first_tick": 0, "ticks": 9000, "span_ms": 2000, "generator": 0, "bytes": 16950444, "sha256": "6933b56a619197c719439ff0c16eed3b8927645c3a0bb66910568c6ccf83bab3"}
+    {"name": "SS-P0-s4", "kind": "runner", "profile": "P0", "clients": 48, "warmup_s": 10, "measured_s": 290, "scene": 1, "seed": 4, "first_tick": 0, "ticks": 9000, "span_ms": 2000, "generator": 0, "bytes": 16950444, "sha256": "1f00d2d5f70a2c7b6836ede611b5f1c281319a5312060ead91e016da3c0b1b42"}
   ]
 }
````

### marks/SERIES.md
_`git diff --no-index spikes/netcode-proof/marks/SERIES.md .claude/worktrees/brawl1/spikes/netcode-proof/marks/SERIES.md`:  1 file changed, 6 insertions(+)._

````diff
diff --git a/spikes/netcode-proof/marks/SERIES.md b/.claude/worktrees/brawl1/spikes/netcode-proof/marks/SERIES.md
index 9672bd2..e7ae680 100644
--- a/spikes/netcode-proof/marks/SERIES.md
+++ b/.claude/worktrees/brawl1/spikes/netcode-proof/marks/SERIES.md
@@ -361,6 +361,12 @@ are written once at the run's end, after the measured phase. No mark reads them
 | `event.run_lengths.warmup_ticks_count`, `.measured_ticks_count`, `.after_ticks_count`, `.order_digest`, `.after_capped` | count (ticks); 16 hex digits; flag (true or false) | npt-lab | the run's three phases in ticks; the digest of the order its clients connected in (r1's bite); true when a bite run's after phase reached its cap with measured ticks unresolved, so the stop decided them lost (carry 73's tick16 past the wrap; a measured run fails there instead) | none (recorded) |
 | `event.recording.ticks_count`, `.bytes`, `.sha256`, `.span_ms` | count; bytes; hex string; ms | npt-lab | a `--record` run's recording: its ticks, size, SHA-256 and the room's hit-volume span (carry 82) | none (recorded) |
 
+### Names added with BRAWL-1 (npt-lab `scenario`'s room log; advisor-u4r-sets-r2-full.md, BRAWL-1 Fix (b))
+
+| Name | Unit | Written by (crate) | Meaning | Marks |
+|---|---|---|---|---|
+| `tick.s.in_building_slot_area` | pairs (slot, area) | npt-lab (from npt-sim's Soldier records' `area`) | the room log, on every logged tick (any phase) where a soldier's feet are in a building area: each such soldier's slot, then its area (building i is area i + 1: the greybox's area rule, half-open on each footprint), in slot order; absent on a tick with none. The run's summary line prints the measured total as "in a building: N slot-ticks (slots ...)" | none (recorded) |
+
 ## Series read only by figures (18)
 
 | Name | Unit | Written by (crate) | Meaning | Figures |
````

## What it is judged against

### The approved plan: sections 1 to 7 (goal, the steering, the figure, the evidence, the recordings that move, the homed items, who builds and the order) (brawl1-plan.md)
_`.checkpoints/netcode-proof-test/brawl1-plan.md` lines 10-236 (sha256 67c16373d744), as they are:_

````text
## 1. Goal

- **The cause, as you ruled it:** the brawl's walk is open loop. A re-run command, or a step that collide() cuts short,
  leaves an offset that nothing corrects. Soldiers then drift into buildings at every profile, and while a door is shut
  the room rightly drops them. That voids -17's L1 parts and fails -2's SS part.
- **The fix:** each brawler steers by its own soldier's decoded position.
- **The proof:**
  - a sim-only test with no soldier in a building area on any tick;
  - two 10-minute runs with everyone in view at or above 0.99;
  - then the re-run sets P3, P2 and P0 (with its six bites) at the fix's commit, before U4g's judged B1 runs.
- -2's SS part passing everywhere is the fix's proof. -17's parts and bites count from that commit.

## 2. The steering (Fix (a))

**Where the position comes from: the client's decoded snapshot.** The brawl's `view` today is the lab mirror's frame,
which is built from the room's own records (mirror.rs:107-150; brawl1-facts A2). That is server state, so it is
barred. The client's decoded state is private (R16: npt-client exposes no decoded state). So:

- **npt-client gains one pub read item.** It gives the own soldier's decoded x, y and z (1 cm grid integers) and its
  dead flag (flags bit 3), from the newest complete, filed tick, as an `Option`.
  - It returns None before the first complete tick, and when the newest complete tick has no own section.
  - It never reads a tick whose header moved on without a complete frame (client/src/lib.rs:147-164, :682-760).
  - This is the route your U4r build ruling 1, point 7 names for U6 (an npt-client pub read item). Q2 asks whether it
    may come now.
- **npt-bots:** step() already holds `&mut ClientEndpoint`. It reads the item and passes the result to one decision
  function, a new pub `next_command`. step() and the sim-only test both call it.
  - Behaviour::command stays private behind it.
  - npt-bots names no World or SimCore: its use of npt-sim stays Command and Scene.

**The rule: a leash toward the cluster centre, in cm, with no rate-derived constant (FP-7 item 1; carry 39).**

| Point | Value | Where |
|---|---|---|
| Team homes | 10 m from the centre | scene.rs:143-154 |
| Crates | 6 and 9 m | greybox.rs |
| Wall stand-in | y 11.8-12.2 m, x -3..3 m | greybox.rs |
| Fight vehicles' bands | about 1.3-17.7 m | world.rs:772-810 |
| Nearest building footprint corner | 16.6 m (buildings 1, 2, 4 and 5); 18 m (buildings 0 and 3) | greybox.rs:83-163 |

- **Proposed:**
  - **R_OUT = 1,400 cm.** Past it, the brawler's walk vector becomes a vector toward the origin, scaled to full
    command length.
  - **R_IN = 1,200 cm.** It walks the open loop again once back inside. The hysteresis stops it flipping each step.
  - **A sideways part.** When the steer is straight along one axis, a deterministic sideways part, never 0 after the
    1 cm snap (|m| > 3 of 127), lets it slide off a box face. That is A4's pin: a goal straight behind a crate or the
    wall stand-in.
  - **None** (no complete tick yet) keeps today's open-loop walk. **Dead** (bit 3) keeps it too: a respawn puts the
    soldier at home.
  - The aim and fire stay as today.
- **Why the centre and not each team's home:**
  - The homes are pub(crate) in npt-sim with no accessor, so a home leash would open crates/sim, and EA-U4R-4 with it.
  - The origin needs nothing from npt-sim.
  - A 14 m leash keeps every soldier at least 2.6 m from any footprint, with homes, crates and the wall inside it.
- **The constants:** R_OUT and R_IN are distances in cm against positions in cm, so they are not rate-derived (A8).
  The walk's TURN_EVERY and RETARGET_EVERY stay pure command counts.
- **The builder may tune** R_OUT, R_IN and the sideways part, within the sim-only test's bar. Any tuned value lands in
  the plan's record.

## 3. The figure (Fix (b))

- **What:** per measured tick, every slot whose feet are in a building area, with the area. "Every" because P2's s2 had
  two (FP-P2).
- **Server side, in Logs::room_tick** (lab/src/logs.rs:470-578). It already iterates the Soldier records of
  `room.sim().entity_records()`. Each record's pub `area` is the sim's own feet area: greybox.area_of, half-open on each
  footprint; area i+1 is building i (world.rs:1753-1772). The test is `area != 0`.
- **Its record:**
  - a new name in room.jsonl's tick record, written on ticks where the list is not empty (the builder names it);
  - a measured total in Counts;
  - the run's summary line prints it as "in a building: N slot-ticks (slots …)". summary_line prints neither it nor
    everyone-in-view today.
- **A recording only:** no judge change and no marks.json change. The new name gets a marks/SERIES.md row ("none
  (recorded)"): the contract's check (b) fails on a written name with no row (contract.rs:64-112). The row is the main
  session's Edit, under "Names added with U4r" or a new BRAWL-1 heading.
- **Why server side:** the wire carries area 0 (proto snapshot.rs:150-160), so a client-side test would need a copy of
  area_of's rule, and two definitions could drift. This is the lab's record, not the bot's steering, so reading the
  server's state is right here.

## 4. The evidence (Fix (c))

1. **A unit test of the steering**, in npt-bots' tests/ (bots/tests/behaviour.rs has b1-b5; none tests the walk's
   closure). next_command with set positions must show:
   - inside R_IN, the walk is as today;
   - past R_OUT, the steer points to the origin;
   - the hysteresis between them;
   - the sideways part on each axis-aligned case;
   - None and dead keep the walk;
   - under a skip pattern (the last command re-run) and blocked steps (positions that do not move), the steer stays
     toward the origin until inside R_IN.
2. **The sim-only test** (no network):
   - **Setup:** npt-sim's World plus next_command; SS; 48 clients; a judged run's full 18,300 ticks; seeds 1 to 3.
   - **Two patterns:**
     - a command every tick;
     - a seeded skip pattern, where a skipped tick re-runs the last command unchanged, as InputBuffer::take does
       (room/src/input.rs:127-151).
   - **The position fed in:** World's own record, standing in for the decoded position. With no network they are equal,
     and the test checks steering, not decoding.
   - **Bar:** no Soldier record with `area != 0` on any tick, for every seed and both patterns.
   - **Printed, not barred:** each run's largest distance from the centre, and any soldier that has not moved for more
     than 300 commanded steps, named.
   - **Home and cost:** it lives in npt-bots' tests/, which needs no Cargo.toml change: npt-bots already depends on
     npt-sim and npt-client (A6). The cost is unmeasured: about 109,800 SS World steps per build, and check.mjs runs both
     builds. So, Q4:
     - (a) the full test is `#[ignore]` and run by name, in release, for the record, with its log kept. A short
       non-ignored copy (seed 1, 3,600 ticks, both patterns) guards the rule in every check.mjs run.
     - (b) the full test runs in check.mjs.
3. **Two 10-minute runs** at the fix's commit: SS P0 seed 1 and SS P3 seed 1. Each is the judged sets' measured-run
   command, with `--out .gate/brawl1-evidence/<run>`. Each is then judged with `--only PM-0001-2,PM-0001-17,PM-0001-57`
   into scratch, under the lane lock, with peak.py.
   - **Bar:** each run's everyone-in-view check (PM-0001-2's everyone_in_view_min_share) at or above 0.99, and the
     building figure's total 0.
   - **Also read:** each run's bytes, against U4r-F3's rates (P0 603.7 MB, P3 183.3 MB). A run above its rate comes to
     you before the re-run sets.
   - **Wall time:** about 250 s at P0 and 335 s at P3 (A9).
   - These runs are evidence: never counted, never in results/.
   - They are not a mode of c-u4r-runs.mjs, so the commands are run by hand, and the command lines are written into the
     evidence log.

## 5. The recordings that move (a re-pin ruling, Q3)

Any brawl change moves three runner recordings (A7):

| Recording | File | Old pin |
|---|---|---|
| SS-P0-s4 | precheck-recordings.json | 6933b56a, 16,950,444 B |
| SS-P2-s1 | det-recordings.json | 5b79abb2, 16,937,652 B |
| ES-P3-s2 | det-recordings.json | fad8f67d, 16,929,501 B |

- **Your rule:** "a SHA-256 mismatch comes to me whole" (INF-U4R-1).
- **What this plan asks (Q3):** that you rule the re-pin here, for exactly these three entries.
  - The builder regenerates each by its own route (`npt-lab hash` prints the new bytes and SHA-256). It records each new
    pin beside the old one in the build notes and the report.
  - det-u4.mjs and `npt-lab hash --manifest crates/lab/precheck-recordings.json` then pass at the new pins.
  - The three synthetic entries in recordings.json do not move.
  - Any other mismatch comes to you whole.
- **det-u4.mjs's time:** its non-cargo time was 227.2 s against the 240 s line (INF-R2-2), and its regeneration is
  about 195 s per call. The steering adds a few integer operations per command. If the sum crosses 240 s, it stops and
  comes to you, never retried.
- **The -30 pre-check set** (runs.txt at d666aeb) re-records SS-P0-s4 and checks it against the manifest. Once SS-P0-s4
  is re-pinned, it is re-made at U4h, as carry 40 and carry 88 already say, not in this lane.
- **c-u4r-r2.mjs's expect texts** hold brawl figures: b1 "everyone in view 0.273333", es "50.05", tick16 "720". They are
  restated at r2's next run. No r2 run is planned.

## 6. The items homed in this lane (brawl1-facts B)

**Part 1: they move no pin.** EA-U4R-1 and EA-U4R-2 are checked at the old pins before the brawl changes. det-u4.mjs and
the precheck regeneration must be byte-equal at 6933b56a, 5b79abb2 and fad8f67d, because their proof of no behaviour
change holds only at the old pins.

- **EA-U4R-2** (`pub(crate) type Sim = npt_sim::World`; one server constructor used by run and run_leg):
  - The constructor takes scene, seed, first tick, clients, event start, arm tick and bites. run needs event_start and
    arm_tick (scenario.rs:746, :760); run_leg passes Bites::default() and event start 0, then sets decoder_bite as today.
  - logs.rs and precheck.rs name Sim.
  - The printed "World::new(scene, ..)" strings (precheck.rs:691, :705) stay word for word, so runs.txt stays
    comparable.
  - lab_room (test-only) uses the constructor if it fits.
  - About 60-90 lines.
- **EA-U4R-1** (one shared loop for RQ8's steps 1-3), in the narrow form:
  - one shared function for steps 1-2 (accept; server.io with admitted and ended) plus the due count, with each caller
    keeping its own per-tick body;
  - run_leg uses SimClock, as the reviewer asked;
  - admitted and ended go to each caller's own sinks: the runner's logs.admit and logs.gone, and the pre-check's maps
    and o.ended.
  - About 40-70 lines. The one-loop driver (150-250 lines) is not proposed.
- **N2-1 with TE-R2-1:**
  - closed_checks gains the ruled fourth check with the ruled message; its doc says four.
  - fp1's doc rests the "not a replacement" claim on the roster.
  - The no-op assert on `ended` (scenario.rs:2209-2212) is dropped.
  - **One correction to the ruled proof (Q5).** Under FP-1's leave plant (room.leave deleted), owner.remove and
    ended.push still run, so two of the four checks fail by name, not four: "the room still holds slot 0: its slot was
    not freed" and "the roster still holds slot 0: a rejoin would be a replacement".
  - A second plant (roster.leave at room.rs:691 deleted) leaves the new check as the only one to fail.
  - Each plant is restored by Edit, with its sha256 checked.
- **TE-R2-2 and RR4-L2** (one change, parse()):
  - **Order:** the 64-bit checks come first and keep their messages, so TE-R2-2's ruled prefix holds. Then RR4-L2's
    caps:
    - a run's last tick (measured_end + after_max_ticks) above u64::from(u32::MAX);
    - the last burst's tick above the same;
    - --bursts above 1,000;
    - --burst-every-s 0 with more than one burst.
  - **Where the caps apply:** the burst caps wherever bursts are used (B1, as today's guard). The last-tick cap applies
    on every profile.
  - **Messages:** each names its options; the builder words them, and they go in the record.
  - **fp3's cases:**
    - TE-R2-2's three cases. Case 1 is on a B1 base line, case 2 equals the ruled message exactly, and case 3 pins the
      64-bit after-max refusal (`--warmup-s 3e17 --measured-s 3e17 --after-max-s 1e17`).
    - RR4-L2's ruled case (`--profile B1 --bursts 2 --burst-every-s 600000000000`, naming --burst-every-s).
    - One case each for the bursts cap, every-s 0, and an --after-max-s that fits in 64 bits but passes the u32 cap.
  - The comment at scenario.rs:1442 says exactly what is bounded.
  - About 45-70 lines.
- **RR4-L1:**
  - MAX_ENTRY_TICKS = 36,000, refused right after the bytes cap and before the runner checks, so the new message, not
    the runner's, is the one that fires. Its message is the ruled one.
  - r10_fp3 gains a case refusing 36,001 by name, and a synthetic entry at 36,000 that passes.
  - About 12-18 lines.
- **RR3-L2 (a):**
  - A Logs flag is set from open's `dir.is_some()`. On a no-log run, filed() resolves only and never grows `res`.
    `last` stays (finish reads it).
  - A new logs.rs unit test: Logs::open(None, ..), admit, filed over many ticks, then `res` is empty and started() is
    true.
  - About 25-35 lines.

**Part 2: BRAWL-1 itself** (sections 2 to 4), then the re-pins (section 5).

**Not in this lane:** EA-U4R-4, because the leash needs no npt-sim change; T-6's contract half, because contract.rs is
not edited; JDG-ONLY-1, because there is no judge change; and the check.mjs follow-up.

## 7. Who builds, and the order

- **One builder copy:** general-purpose, opus, effort xhigh (netcode), in an isolated worktree, building in
  .gate/shared-target (Mustafa's rule). Its brief comes from this plan once approved. Q6 asks whether the brief comes to
  you first, as U4r's did.
- **The order inside the copy:**
  1. **Part 1,** checked with check.mjs plus det-u4.mjs and the precheck regeneration byte-equal at the old pins. These
     results go in the build notes before Part 2 starts.
  2. **Part 2.**
  3. **The re-pins,** with check.mjs and det-u4.mjs passing at the new pins.
  - A stop rule as U4r's copies had: at its context limit, a copy writes its state and the next copy continues from its
    notes and the diff.
- **SERIES.md's row:** the main session's Edit, before the merge.
- **One review round on the lane's final diff:** test-engineer, netcode-architect and rust-reviewer, together as one
  round. Below High, a finding becomes a follow-up (the budget).
- **The merge into the spike branch:** CI, then your review of the head.
- **After the merge:**
  1. (B) and (C) on the shared target (section 8);
  2. the two 10-minute runs (4.3);
  3. the re-run sets (section 9), each set's results committed before the next.

````

### The approved plan: sections 11 and 12 (change type, tier, reviewers; planned size) (brawl1-plan.md)
_`.checkpoints/netcode-proof-test/brawl1-plan.md` lines 288-306 (sha256 67c16373d744), as they are:_

````text
## 11. Change type, tier, reviewers

- **Risk tier 1:** spike code and tests. No game code, no kit or gate file, no judge or marks.json change; marks/SERIES.md
  gains one row (recorded only).
- **Types and their evidence:**
  - netcode and sim behaviour: deterministic tests and the 10-minute runs;
  - the refactors: byte-equal regeneration at the old pins.
- **Reviewers:** test-engineer, netcode-architect and rust-reviewer (your ruling).

## 12. Planned size (SIZE-CHECK)

| Part | Lines changed (estimate) |
|---|---|
| Part 1: EA-U4R-2 60-90, EA-U4R-1 40-70, N2-1 8-12, TE-R2-2 with RR4-L2 45-70, RR4-L1 12-18, RR3-L2 (a) 25-35 | about 190-295 |
| Part 2: the npt-client read with its test 30-50; next_command, the leash and the unit test 80-120; the sim-only test 80-120; the figure in logs.rs with Counts and the summary 30-50; SERIES.md 2; the manifests 6; docs and comments 20-40 | about 250-390 |

- **The whole lane:** about 450-700 lines. The card carries 700 as the plan.
- **"Well past"** is more than twice that: more than about 1,400.

````

### The advisor's plan review: APPROVE WITH CONDITIONS 1-18, the choices, the findings and the size (advisor-brawl1-plan-full.md)
_`.checkpoints/netcode-proof-test/advisor-brawl1-plan-full.md` lines 1-109 (sha256 a7e68b90096b), as they are:_

````text
# Advisor: the plan review of BRAWL-1's lane (workflow wf_533068a9-b2b), 2026-11-14

Written by script, word for word, from the workflow's journal (advisor-brawl1-plan's StructuredOutput result). The plan: brawl1-plan.md. Its notes: advisor-brawl1-plan.md. Nothing below is the main session's.

## Verdict: APPROVE WITH CONDITIONS

## Conditions

1. 1. The leash's bounds (Q1): R_IN at least 1,300 cm. That is above the wall stand-in's contact reach: its north face at 1,220 cm plus the soldier's 40 cm radius gives 1,260 cm (greybox.rs:156; world.rs:75-77). So no fixed box lies in the steering band (the crates reach at most 990 cm). R_OUT at least R_IN + 100 cm and at most 1,500 cm, which keeps 3 m to the nearest door entry: a soldier's centre at 1,800 cm, building 0's door at x 1,800 with |y| < 60 (greybox.rs:83-136). The plan's record replaces '2.6 m from any footprint' with this door-entry margin, and notes that the spawn grid's corners lie at 1,414 cm (world.rs:703-722; scene.rs:153). The sideways part may stay or go. The leash applies to Kind::Brawl only, so the open-loop scripts are unchanged. Every leash vector has x^2 + y^2 <= 127^2 (the unit test asserts it), so the sim never takes its scale path (world.rs:494-498). Tuning stays inside these bounds, and any tuned value is recorded.
2. 2. The stuck print: the sim-only test names any soldier whose position stays inside a 100 cm square for more than 300 commands given, with its slot, tick and position. This replaces 'has not moved', which a soldier rocking by 1 cm at a box face passes. Any soldier the full test names comes to me before the 10-minute runs.
3. 3. The sim-only test (Fix (c)): (i) Both barred patterns feed each slot its position from 8 steps earlier: the largest lead a run takes (--lead's cap), as a step count with no time claim. (ii) The skip pattern's rate and longest streak are stated in the test's doc and printed. (iii) A third pattern is printed only, never barred. It models B1: every slot re-runs its last command for npt_sim::ticks_for_ms(1_000) ticks at ten burst starts, the first at 30 s and then every 60 s (the run scripts' B1 settings). Its result (any area != 0, the largest distance from the centre) goes into the build notes and into U4g's brief. (iv) Every length comes from npt_sim::ticks_for_ms (610,000 ms for the full run, 120,000 ms for the short copy), never a literal tick count (carry 39). (v) The World starts at the judged runs' first tick: World::new(scene, seed, 63_736).
4. 4. npt-bots gains no pub item. The decision function is crate-private and called by step(). The steering unit test and the sim-only test live in a #[cfg(test)] module in npt-bots' src/: npt-sim is already a normal dependency, so Cargo.toml does not change. npt-bots' library code names no World or SimCore; only that test module does.
5. 5. The npt-client read item (Q2): one read-only pub method on ClientEndpoint (&self). It returns, by value, an Option of the own soldier's decoded grid x, y (and z) and its dead bit (flags bit 3), taken from the newest tick that is complete (Tick::complete) and filed Filed::Hash, with ticks compared by the endpoint's wrap rule. No reference into the ring, no float, no new dependency. Its doc says it reads the client's own decoded state. Its npt-client test shows: None before the first complete tick; after a complete tick, Some equal to the own record's x, y, z and dead bit; no change from a newer incomplete, refused or sentinel tick. The aim stays on the mirror view: build ruling 1 point 7 and its U6 follow-up stand.
6. 6. Part 1's proof at the old pins. check.mjs, det-u4.mjs and `npt-lab hash --manifest crates/lab/precheck-recordings.json`, byte-equal at 6933b56a, 5b79abb2 and fad8f67d, prove only scenario::run's half, because they regenerate through `scenario --record`. run_leg's half needs two more proofs. (i) The clean pre-check pair: record SS-P0-s4 at its entry's settings (sha256 6933b56a), then run `npt-lab precheck --manifest crates/lab/precheck-recordings.json --recording <file> --commit <sha>` with no bite on the measured build. Its output lines must equal the '## pair clean' section of results/u4r-precheck-30/faf42aeb034c/runs.txt line for line, except the build line (commit and binary sha256). The crates have not changed since faf42aeb. This costs about 320 s and 17 MB. (ii) The precheck tests' --nocapture output (r6, the late-command bite, wallclock), captured before Part 1 and after, must diff identical. Any difference comes to me whole. Section 6 of the plan says which proof covers which half.
7. 7. The re-pin (Q3) is ruled for exactly three entries: SS-P0-s4 (precheck-recordings.json), and SS-P2-s1 and ES-P3-s2 (det-recordings.json). It comes only after condition 6's results and Part 2's tests pass and are in the build notes. The manifests' diff changes only those three entries' `bytes` and `sha256`; every other field and both 'about' texts stay. Each new pin must be reproduced at the merge step's counted run (condition 9). The build notes and the report give each new pin beside the old one. Any other mismatch, any mismatch at the old pins, and a new pin that is not reproduced each come to me whole. No retry.
8. 8. det-u4.mjs: every call has `uptime` first in the same main-run.sh call and runs while no other lane builds or runs. A non-cargo sum over 240 s stops the step and comes to me whole with the uptime, never retried (INF-R2-2).
9. 9. The plan names the merge step in U4r's M-list form. First, the merged bytes must equal the reviewed bytes (cmp and the tree fingerprint). Then one counted main-run.sh call on the merged tree: uptime; check.mjs with all 11 steps; det-u4.mjs; the precheck manifest regeneration at the new pin (build ruling 1: det-u4 at every merge that touches the scenario path). Then the gate (--no-smoke, with the reason, if Chromium cannot start), CI green, and my review of the pushed head.
10. 10. Fix (b): every name the change writes gets a marks/SERIES.md row marked 'none (recorded)'. That means the tick name, plus any lab end-event field that carries the measured total. The list is written on every logged tick where it is not empty, in any phase. The Counts total counts measured ticks, and the summary line prints it. No judge change and no marks.json change.
11. 11. The reviewers: test-engineer, netcode-architect, rust-reviewer and engine-architect. engine-architect reviews the npt-client pub item and the form of EA-U4R-1 and EA-U4R-2. It is one round on the final diff, run in two batches of at most 3, each seat at effort xhigh (netcode).
12. 12. TE-R2-2's case 3 is written in full digits, because num() parses u64 (scenario.rs:1327-1332) and '3e17' would give a different usage error: --warmup-s 300000000000000000 --measured-s 300000000000000000 --after-max-s 100000000000000000. The case pins '--after-max-s: the run's last tick overflows 64 bits'.
13. 13. Q4: the full test is #[ignore = "<reason>"] (the precedent is net l2 socket_tests.rs:512). It is run by name, in release, at the head that goes to review, and its log is kept under .gate/. The short copy (seed 1, ticks_for_ms(120_000), both barred patterns) runs in every check.mjs. The build notes record the short copy's time in each build and the full test's release time. If the short copy takes more than 60 s in either build, it is shortened and the change recorded.
14. 14. The 10-minute evidence runs: their command lines go into the evidence log, and their set and run names mark them as evidence, never u4r-*. Each run's -17 mismatches and incomplete count are recorded as facts, not as a verdict. If either run misses its bar (everyone_in_view_min_share under 0.99, or a building total above 0), or its bytes exceed its U4r-F3 rate, the re-run sets do not start, and it comes to me whole.
15. 15. The expected lines for the re-run sets, with my words clarified: for -57, a difference in a verdict, or in whether a bite is shown, comes to me whole. Its values are recorded beside U4r's, and the report names the largest relative change. The rest stays as the plan has it: TICK16-WORDING's line, the building figure at 0 on every run, and each set's raw-log bytes against its projection.
16. 16. Disk: the build-phase figure uses the measured full shared target, 8.2 GiB (u4r-disk-plan.md:12), so about 4.7 GiB is free after the refill. The plan's 'about 9 GiB' would give 3.9 GiB, and its '4.2' was my own unit mix-up. Brief rule 6 applies: 4 GB free before any wasm32 or release build, 2 GB before any other build. A floor stop is reported and never cleaned around. The kept-logs draft's ratio is corrected to the measured 3.55 times, about 2.1 GiB of archives for the 7.4 GiB.
17. 17. Q6: the builder's brief comes to me first, for a conformance check only: every section of the plan and every condition here is carried, and nothing new is decided.
18. 18. The report records my two corrections. ADV-BP-1: N2-1's 'all four checks fail' was wrong; under the leave plant two fail (Q5). ADV-BP-2: my sets-r2 figure of 'about 4.2 GiB free after the refill' mixed units.

## The choices (the plan's section 13)

- **Q1, the leash** (a) Toward the cluster centre, with the bounds of condition 1: R_IN at least 1,300 cm; R_OUT from R_IN + 100 cm up to 1,500 cm. The proposed R_IN of 1,200 cm is refused. **Why:** The centre needs nothing from npt-sim, so EA-U4R-4 stays shut. Building entry is only through a door gap, at a soldier's centre at least 1,800 cm out (greybox.rs:83-136), so the margin is 3 m or more. But at R_IN 1,200 cm the wall stand-in (contact reach 1,260 cm) sits inside the steering band. A soldier steering in from the north hits its north face and stays in steer mode. The origin steer's x part points to x = 0, so it slides toward the middle and rocks at about |x| 35 cm, where the 1 cm snap erases the steer (A4). A sideways part used only on an exact axis pushes it out, and the steer pulls it back. With R_IN above 1,260 cm, no fixed box lies in the band, and the crates (990 cm at most) are never met while steering.
- **Q2, the position's source (and whether an npt-client pub read item may come now)** (a) Yes, it may come now, in condition 5's form. Option (b) is barred. **Why:** Verified: the brawl's view is mirror.view (scenario.rs:1004-1008, mirror.rs:227-244), and the mirror's Records are built from the room's own EntityRecords (mirror.rs:110-138). That is server state, which Fix (a) bars. R16 is a facts row ('npt-client exposes no decoded record'), not a rule. Build ruling 1 point 7 named 'an npt-client read item (R16)' as the route once bots need the client's own view, and BRAWL-1 now needs it for movement. It is a new pub item, so engine-architect joins the round (condition 11). npt-bots gains none (condition 4).
- **Q3, the re-pin** (a) Ruled here, for exactly SS-P0-s4, SS-P2-s1 and ES-P3-s2, under conditions 7 to 9. **Why:** Build ruling 1 says a unit on the scenario path re-pins and never retries. INF-U4R-1 says a re-pin needs a ruling. This is that ruling. It is bound to the old-pin proofs coming first, to a manifest diff that touches only bytes and sha256, and to the new pins being reproduced at the merge's counted run. Anything else comes to me whole.
- **Q4, the sim-only test's cost** (a) The full test is #[ignore]d and run by name in release for the record; a short copy runs in every check.mjs (condition 13). **Why:** In the dev profile npt-sim is not optimised (Cargo.toml opts only rand_chacha and ppv-lite86), and check.mjs runs both builds. There is a precedent: net l2 socket_tests.rs:512. An ignored test is not a skip, because cargoCounts reads passed and failed, and 'SKIPPED' is a test's own print (check.mjs:1937). My ruling asks for the full test once, with its log, at the fix's commit.
- **Q5, N2-1's proof under the leave plant** (a) Two of the four checks fail by name, plus the second plant, under which the roster check fails alone. **Why:** Verified at scenario.rs:496-503. With room.leave (:499) deleted, owner.remove (:500) and ended.push (:502) still run, so only the room check and the roster check fail. With roster.leave (room.rs:691) deleted, clients[i] is still set to None, so only the new check fails. My r2 words 'all four' were wrong: correction ADV-BP-1, mine.
- **Q6, the builder's brief** (a) It comes to me first, for a conformance check only (condition 17). **Why:** This ruling adds exact conditions (bounds, proofs, re-pin limits, test design). A brief that drops one would cost runs. The check decides nothing new.
- **Q7, KEPT-LOGS** (a) It goes to Mustafa now. The lane does not wait for his answer. My recommendation to him is option (c), archive then remove, with the old 4 GB build cache named in the same rule. **Why:** The approved cleans hold this lane, but only just: about 4.7 GiB is free in the build phase against the 4 GB rule. U4g, right after, does not fit at all: about 6.6 GiB free after the re-run sets against a refill of about 8.2 to 9 GiB. The rule is a kit change (risk tier 2, its own PR with security-auditor, his go in a day session), so it needs lead time. A narrow kit rule is his own LEFTOVER-DELETE route. I measured 3.55 times compression (one P3 run, zlib-6, in memory), not 5 to 10 times.
- **Q8, Part 1's place** (a) In this lane, as ruled, and proved at the old pins first, with condition 6's added proofs. **Why:** The old pins exist now, and the crates have not changed since faf42aeb, so a byte-level 'before' exists today: the clean pair's runs.txt. Moving the refactors to U4h would only move the same proof and add another lane's overhead. Part 1 is about 190-295 lines with roughly an hour of proof runs. That is acceptable on the critical path.

## Findings

### BP-1 (Medium)

- **Where:** brawl1-plan.md section 6 Part 1 and section 7 step 1; precheck.rs:285-480 (run_leg); hash.rs:413-480 and det-u4.mjs:1-37 (regeneration via scenario::run only)
- **Wrong:** The plan proves EA-U4R-1 and EA-U4R-2 by det-u4.mjs and the precheck regeneration, byte-equal at the old pins. Both regenerate through `scenario --record`, so they never run run_leg. The SimClock change and the shared constructor in run_leg would be proven only by the precheck unit tests, whose outputs are not byte-compared. Half the refactor's claimed evidence does not hold.
- **Fix:** Condition 6: the clean pre-check pair at the old pin, diffed line for line against results/u4r-precheck-30/faf42aeb034c/runs.txt's '## pair clean' section, plus the precheck tests' --nocapture output diffed before and after Part 1. Any difference comes to me whole.

### BP-2 (Medium)

- **Where:** brawl1-plan.md section 2 (R_IN 1,200 cm, the sideways part) and 4.2 (the stuck print); greybox.rs:156 (the wall stand-in); world.rs:1230-1285, :1343-1361
- **Wrong:** The wall stand-in's contact reach (1,260 cm) lies inside the steering band [1,200, ...]. A soldier that steers in from the north is funnelled along the wall's north face to about |x| 35 cm, where the 1 cm snap erases the steer. The sideways part, used only on an exact axis, pushes it out and the steer pulls it back: a rocking pin. The 'has not moved in 300 steps' print cannot see it. The wall falls when shot (500 health), so it likely heals, and no verdict is wrong, but the plan's claim that the sideways part answers A4's pins is not true here.
- **Fix:** Conditions 1 and 2: R_IN at least 1,300 cm, R_OUT from R_IN + 100 up to 1,500 cm; the stuck print by a 100 cm square over 300 commands; any soldier named comes to me.

### BP-3 (Medium)

- **Where:** brawl1-plan.md 4.2 (the sim-only test); l1 lib.rs:197-201 (B1's 1 s bursts); scenario.rs:1829-1836 (P3 lead 7)
- **Wrong:** The test feeds each soldier its current position, while the real loop acts on a position a full lead old (up to 7 to 8 steps, about 1.6 m). Nothing records how the leash behaves under B1's 1 s bursts of total loss: about 30 or more re-run steps, 6 m or more, against a 3-4 m entry margin. U4g's judged B1 runs could repeat BRAWL-1 unseen. The test also names literal tick counts (18,300 and 3,600), which carry 39 bars outside npt-sim.
- **Fix:** Condition 3: an 8-step delayed feed in both barred patterns; the skip pattern's rate and longest streak stated; a printed-only B1 pattern recorded for U4g's brief; lengths from npt_sim::ticks_for_ms; first tick 63,736.

### BP-4 (Medium)

- **Where:** brawl1-plan.md section 7 ('The merge into the spike branch: CI, then your review of the head'); advisor-u4r-build1-full.md:14
- **Wrong:** The plan names no counted run on the merged tree. Build ruling 1 requires det-u4.mjs at every merge that touches the scenario path, and the new pins must be reproduced outside the builder's copy.
- **Fix:** Condition 9: the M-list form (merged bytes equal the reviewed bytes; one counted main-run.sh call with uptime, check.mjs, det-u4.mjs and the precheck regeneration at the new pin; the gate; CI; my review of the pushed head).

### BP-5 (Low)

- **Where:** brawl1-plan.md sections 2, 7 and 11 (the reviewers; pub next_command); advisor-u4r-r1-full.md:40
- **Wrong:** The lane adds a pub item to npt-client and a pub next_command to npt-bots, and carries engine-architect's own EA-U4R-1 and EA-U4R-2. My round-1 rule is that engine-architect rejoins on a new pub item or seam change, but it is not named. A pub next_command also opens a second public route to commands, against EA-U4R-5's single route (U6), when a crate-private function would do.
- **Fix:** Conditions 4 and 11: npt-bots gains no pub item (the tests sit in-crate), and engine-architect joins the one round.

### BP-6 (Low)

- **Where:** brawl1-plan.md section 6, TE-R2-2 case 3; scenario.rs:1327-1332
- **Wrong:** Case 3 is written as '--warmup-s 3e17 ...'. num() parses u64, so that text would hit 'is not a whole number', not the after-max refusal.
- **Fix:** Condition 12: full digits.

### BP-7 (Low)

- **Where:** brawl1-plan.md section 9 ('-57 ... its values recorded beside U4r's (any difference comes to you)'), from advisor-u4r-sets-r2-full.md BRAWL-1 (d)
- **Wrong:** Every -57 value moves with any brawl change, so read literally, every set would stop for me. My words meant the verdicts.
- **Fix:** Condition 15: a verdict difference, or a change in whether a bite is shown, comes to me; values are recorded side by side with the largest change named.

### BP-8 (Low)

- **Where:** brawl1-plan.md section 8 and kept-logs-draft.md section 2 ('about 9 GiB ... about 4.2 GiB'); u4r-disk-plan.md:12
- **Wrong:** 13,225 MiB minus 9 GiB is 3.9 GiB, under the floor, not 4.2. The measured full shared target is 8.2 GiB, which gives about 4.7 GiB. The 4.2 is my own sets-r2 unit mix-up (ADV-BP-2).
- **Fix:** Condition 16.

### BP-9 (Low)

- **Where:** kept-logs-draft.md section 4 (c) ('about 5 to 10 times smaller ... about 1 GiB')
- **Wrong:** Measured in memory with zlib-6 on P3 s2's three logs (181.4 MB to 51.2 MB): 3.55 times. So about 2.1 GiB of archives, not 1 GiB.
- **Fix:** Condition 16; the figure in the question to Mustafa uses 3.55 times.

### BP-10 (Low)

- **Where:** brawl1-plan.md section 3 ('The new name gets a marks/SERIES.md row'); contract.rs:66-110
- **Wrong:** If the measured total is written to a lab end event, that event field is a second written name, and the contract's check (b) needs a row for it too. The plan names one row.
- **Fix:** Condition 10: a row for every written name.

### ADV-BP-1 (Low)

- **Where:** advisor-u4r-r2-full.md N2-1 ('Under the leave plant (room.leave deleted), all four checks fail by name in both cases')
- **Wrong:** My error. owner.remove and ended.push still run under that plant, so only two checks fail.
- **Fix:** Q5 (a); condition 18 records it in the report.

## Size

About 700 lines planned, as the card carries (the plan's 450-700; my conditions add roughly 30-60 lines of test and doc, for about 450-750). 'Well past' under SIZE-CHECK means more than about 1,400 lines; past that the builder stops, and it goes to Mustafa in three lines.
````

### The advisor's sets ruling, Q1: -17 L1 P0 INVALID and BRAWL-1's scope (advisor-u4r-sets-r2-full.md)
_`.checkpoints/netcode-proof-test/advisor-u4r-sets-r2-full.md` lines 7-24 (sha256 7b6aa38c26cb), as they are:_

````text
### Q1. -17 L1 P0 INVALID: the cause, what happens to -17's L1 P0 part and its three bites, whether BRAWL-1's scope must widen, what goes in U4r's report, anything to Mustafa

**Ruling:** It is the same defect as BRAWL-1, but with a second trigger that my sets ruling missed. It is not a netcode fault. The brawl's walk is open loop: it closes only if every step moves its full length. At P3 and P2, re-run commands break that. At P0 every step has its own command (878,352 = 48 x 18,299; 0 steps without one; late 0). But collide() stops a step at the first box in its way: crates, the wall stand-in, closed doors, and the parts of the four fight vehicles that loop through the cluster. Each cut step leaves a net offset that nothing pulls back, so a soldier slowly random-walks. In s1, slot 21 (team 1) entered a building 458 s into the measured window.
(1) -17's L1 P0 part stays INVALID: recorded, never counted, never re-run at 051995db9cea (condition 5). s2 and s3 are valid: 0 mismatches of 878,352 compared and 0 incomplete in each. They are recorded as facts, not a verdict, because the part needs 3 valid runs.
(2) Its three bites (tick16, client-rounding, whole-as-lost) read invalid by pre:PM-0001-2 at the same 0.861889 (seed 1, the same drift): recorded, never counted, neither shown nor missed. Their summary lines show that each bite bit as designed (whole-as-lost measured incomplete 48; tick16 720 and 48 resyncs). The report states that as mechanics, not as a judged result.
(3) The part and all three bites move to BRAWL-1's re-run sets at the fix's commit, together with -17's L1 P3 and L1 P2 parts.
(4) Counted and recorded at P0: -57's L1 P0 PASS and its three shown bites count, because -57 has no validity rule. -2's SS part at P0 is recorded as FAIL (s1 only).
(5) BRAWL-1's scope widens (finding BRAWL-1). The cause is the open-loop walk plus any step that does not move its full length, at every profile. The fix must steer by the soldier's own position. My option 'or a walk that closes despite re-runs' is withdrawn. The evidence adds a sim-only full-length test and a P0 seed-1 run.
(6) What U4r's report says about P0 is listed under Q3.
(7) Mustafa gets a heads-up only, with nothing to decide. Moving a part to a later unit is mine (sets ruling Q1 (5)), the 99 % bar stays, and the verdict failed closed.

**Why:** Checked myself from the logs and the code.
(a) Not loss, timing or decoding. P0 has no loss. s1's summary: commands 878,352, 0 steps without one, late 0, overflow resyncs 0, measured incomplete 0. My print-only hash read of s1's measured ticks finds 0 of 864,000 client-ticks differ, the same as s2. That read decides nothing.
(b) One soldier behind one door. The onset at 77,476 is elapsed 13,740 from the first tick 63,736. elapsed starts at 0 and adds 1 per step (world.rs:841, :2132). (13,740 / 30) mod 6 = 2, so this is door 2's 77th toggle. Door 2 starts open (world.rs:819), so it shuts on the onset tick. The others' count is 47 in door 2's open half and 0 in its shut half (phases 60-240). They are re-added within one 30-tick bin, because nothing is lost. Slot 21 holds on 2,074 of 4,560 ticks after the onset. Door 2 belongs to building 2 at (-11, 19.05) m. Its south door is about 8.7 m from team 1's home at (-5, 8.66) m, the nearest door to that home (building 1's is about 17 m away). Slot 21 is in team 1 (slot / 16, scene.rs:302). No log carries positions (room v/s/hs, lab s, client s), so 'building 2' is inferred, not read.
(c) It is deterministic. client-rounding (the same sim) is identical. tick16 starts at 47,251, yet its onset (60,991) is also at elapsed 13,740, with the same slot and the same 0.861889. s2 and s3 have no onset.
(d) The mechanism in the code. The walk is MOVES[(given / 4 + phase) mod 16] (bots/src/lib.rs:31-51, :210), and the 16 vectors sum to 0. In free space the moves are symmetric on the 1 cm grid (move_velocity, world.rs:476-505: +18 / -18 cm), so the loop closes only when no step is cut. collide() (world.rs:1233-1285, applied per command at :1345) stops each axis at the first box face. The boxes are the greybox solids (crates at 6 and 9 m out, e.g. [-600, 600] and [0, 900] beside team 1's home), the closed doors, the standing wall (x -3..3 m, y 11.8-12.2 m) and the vehicle parts. The four fight vehicles loop at 5, 8, 11 and 14 m (world.rs:796-806), through all three homes 10 m out. Soldiers do not collide with each other, and a box a soldier starts inside never blocks it, so nothing pushes a soldier back. Deaths are rare (a hit takes 1 of 1,000 health). A respawn puts the soldier back at a home point (world.rs:1734-1750), which only re-anchors it.
(e) My sets ruling's Why (d) said 'At P0 ... the circle closes'. That held for the command count over r2's 30 s, not for the position over 10 minutes. My step 7 anticipated this case and held it for a ruling, so nothing counted wrongly. The error is mine (ADV-SR2-1).
(f) -17's parts carry validity ['pre:PM-0001-2'] with min 3; -57's carry none (marks.json). Both are as ruled for P3.
````

### The advisor's sets ruling, finding BRAWL-1 with its Fixes (a) to (d) (advisor-u4r-sets-r2-full.md)
_`.checkpoints/netcode-proof-test/advisor-u4r-sets-r2-full.md` lines 96-107 (sha256 7b6aa38c26cb), as they are:_

````text
### BRAWL-1 (Medium)

- **Where:** spikes/netcode-proof/crates/bots/src/lib.rs:29-51 and :206-223 (the brawl's walk, open loop); crates/sim/src/world.rs:1233-1285 (collide) and :1330-1350 (each command's move); crates/room/src/input.rs take() (a tick without a command re-runs the last one)
- **Wrong:** The walk closes only if every step moves its full length. A re-run command (P2, P3, B1) or a blocked step leaves a net offset that nothing corrects. Steps are blocked by crates, the wall stand-in, closed doors and the fight vehicles' parts, at every profile, P0 included. Over 10 minutes, soldiers reach the buildings, and the room correctly drops them while a door is shut. This voids -17's L1 P0, P2 and P3 parts and its three P0 bites, and fails -2's SS part at every profile. It fails closed (INVALID, never a wrong pass), so it stays Medium, but it now holds every -17 L1 verdict and sits on U4's critical path.
- **Fix:** In the next npt-lab lane (with N2-1), plan first, with my plan review. Reviewers: test-engineer, netcode-architect, rust-reviewer.
(a) Each brawler steers by its own soldier's position, as the host tells it from the decoded snapshot, never from the server's state. For example, a leash toward its team's home or the cluster centre, in cm, with no rate-derived constant (FP-7 item 1). The option 'a walk that closes despite re-runs' is withdrawn.
(b) A run figure that names every soldier whose feet are in a building area, per measured tick (P2's s2 had two).
(c) Evidence:
- a unit test of the steering under skipped and blocked steps;
- a sim-only test with no network (npt-sim's World plus Behaviour, a judged run's full 18,300 ticks, seeds 1 to 3, both with a command every tick and with a seeded skip pattern), showing no soldier in a building area on any tick;
- then one 10-minute SS P0 seed-1 run and one SS P3 run, each with everyone in view at or above 0.99.
(d) Then the re-run sets P3, P2 and P0 (with its six bites) at the fix's commit, before U4g's judged B1 runs. -2's SS part passing everywhere is the fix's proof. -17's parts and bites count from that commit. -57's results there are recorded beside U4r's, and any difference comes to me.
````

### U4r's round-2 ruling: the main session's next steps (condition 18 homes N2-1, TE-R2-2, RR4-L1, RR4-L2 and round 1's EA-U4R-1/-2 and RR3-L2 (a) here) (advisor-u4r-r2-full.md)
_`.checkpoints/netcode-proof-test/advisor-u4r-r2-full.md` lines 5-24 (sha256 6dcc0a81a961), as they are:_

````text
## The main session's next steps, in order

1. 1. Record the ruling: write it word for word by script (advisor-u4r-r2-full.md), and put its For Mustafa (nothing to decide; two heads-ups) and its follow-up list on the card at once. The card's line 'no judged or pre-check set before INF-R2-1 is ruled' now reads: no c-u4r-runs.mjs set before U4r-F1 is committed. No round 3: there is no Critical or High.
2. 2. M1, the bytes reviewed: wip/u4r.patch's sha256 is 21ca46be23704131... and unchanged since 450af1a (`git diff --stat 450af1a HEAD` on it is empty). .gate/u4r-snap2's spikes/ equals copy 6's worktree (diff -rq empty). The builder's tree fingerprint, run from the copy's root, gives 63713cf344482a9b.... Anything else stops the merge and comes to the advisor.
3. 3. M2, the base: on the branch head, `git diff --quiet 2fbec77 HEAD -- spikes/` exits 0 (it does at a849eac), and every commit since 2fbec77 touches only .checkpoints/. Main is still 4460ef5 (docs only), so it is not merged in (his MAIN-52). If main has meanwhile changed code, CI or rules, merge main in first and re-check M2.
4. 4. M3, the apply: save and push the card, then stop the saver. Run `git apply --check`, then `git apply`, of wip/u4r.patch in the main folder. `git status` shows exactly the 26 files under spikes/netcode-proof (13 modified, 13 new) beside .checkpoints/.
5. 5. M4, the merged bytes are the reviewed bytes: each of the 26 files is cmp-equal to .gate/u4r-snap2's copy, and the fingerprint run from the main folder's root gives 63713cf3.... A patch that applies but differs stops the merge.
6. 6. M5, the counted run on the merged tree: one main-run.sh call (shared target, lane lock exclusive, offline), in the background with a limit of at least 1,800,000 ms, its log under .gate/u4r-merge/. In order: `uptime`; section 4 rule 3's six builds, whose provenance lines must name /home/user/Shamble-Frontlines/spikes/netcode-proof/crates/...; `node check.mjs`; `node ops/runs/det-u4.mjs`; `npt-lab hash --manifest crates/lab/precheck-recordings.json` as regen-precheck.log ran it. Expected: 'check: all 11 steps pass'; the test step at 531 and 609, 0 failed and 0 skips; self 77 of 77; marks 81 with the ADR rows unchanged; redact 0 findings; the deps line as c6-end2.log:173 (no attribute holds a metavariable; npt-sim's features one set); the wasm line as c6-end2.log:174 (27000 ticks and 0 mismatching for both pairs, h2 equal, hash-map caught at SS-s1 tick 1234, std-sin at SS-s1 tick 2 vehicle[8].pos.y), under 300 s and labelled with uptime; det-u4 with deps PASS and det-u4 PASS, 18000 ticks and 0 mismatching for both comparisons, and a non-cargo sum of at most 240 s; SS-P0-s4 regenerated with sha256 6933b56a... as the manifest's, legs equal. A SHA-256 mismatch, a FAIL, or a non-cargo sum over 240 s stops the merge and comes to the advisor whole, with the uptime: no re-pin, no retry.
7. 7. M6, TE2-5: a failing test is never retried into green unnamed. Its crate is re-run at once through main-run.sh with --no-fail-fast, and its panic and FX-10's file are kept under .gate/u4r-merge/. A failure in npt-lab, npt-bots, npt-log, npt-server, check.mjs or check.test.mjs, or in a test that reads U4r's files, stops the merge and comes to the advisor. Any other failure (TE-M3's npt-net test among them) is recorded under TE2-5, and the merge goes ahead only on a whole counted run with all 11 steps passing. TE2-5's rule ends at this merge only if this test step has no failure (TE-Q2); otherwise it continues.
8. 8. M7, the gate: `node tools/gate.mjs --no-smoke` (brief section 5 item 6), or the full gate if Chromium starts; keep both logs if smoke cannot start, with the reason. CI's gate, smoke and toolchain jobs must be green on the pushed head.
9. 9. M8, one commit: 'U4r: the L1 runner, the -30 pre-check and the run scripts, merged after review round 2'. It carries: the 26 files exactly as applied (nothing else under spikes/; no marks.json change); `git rm` of wip/u4r.patch; u4r-build.md with a new section 'Merge into the spike branch' (M1-M7's results); and u4r-report.md, new, in u4d2-report.md's form. The report covers: who built it (copies 1-6); what the four build rulings and both rounds settled (round 1: six seats, 50 findings; round 2: five seats, 1 Medium and 12 Low); the two deviations accepted here; the checks on the merged tree; the carries met, by number; the public items (EA-U4R-1's line, the contract and precheck subcommands); the timing with its labels (u4r-build.md:1916-1923 carried over, plus INF-R2-2's det-u4 base and the merge-step figure with uptime); SIZE-CHECK at 8,362 lines against M's about 3,000, under his SIZE-U4R (a); every follow-up with its home (round 1's FP-9 item 4 list plus this ruling's); the open security findings SEC-U4R-R2-1 (and SEC-U4R-R2-2, message only) for PR #40's report; the judged runs as pending, in step 17's order; Decisions needed; For Mustafa. The commit also carries the card. The scratch helper test is never committed.
10. 10. M9, after the push: restart the saver; CI green; update the card (U4r merged at <sha>, 531/609/77, TE2-5's state, the follow-ups, the open security finding). Keep copy 6's worktree until F1's evidence is in (its scratch helper test is F1's starting point), then remove U4r's copies per CHECKPOINTS.md §10 (the builder worktrees, .gate/u4r-snap1, .gate/u4r-snap2). Every run log stays (LEFTOVER-DELETE).
11. 11. The r2 record run from the merge commit (ruling 3 point 6 (4)): `node ops/runs/c-u4r-r2.mjs --date <today>` through main-run.sh with the lock exclusive, under INF-R2-1's interim rule. Commit and push the card right before; stop the saver; run no agent that writes in the main folder; make no commit and no tracked edit in the main folder until r2.txt is written; then save and restart the saver. b1, es and b1-nofallback are expected to fail as ruled: they are recorded and never re-run. Any 'stopped at' or 'exit by a signal' line comes to the advisor whole.
12. 12. Carry 62 (h), from r2 (ruling 4 point 3 and T-7). If every clean short L1 P0 suite's measured-phase sum of tick.s.snap_incomplete is 0: the main session's own commit adds the plain check and PM-0001-17/whole-as-lost to -17's L1 P0 part in marks.json, plus the readings-PR list line (plan §11: his go before R5). Then run `npt-judge --only PM-0001-17` with that marks.json on r2's whole-as-lost and tick16 folders: both bites must read Shown, never INVALID. If any sum is not 0, the cause comes to the advisor first, before any -17 run. A bite run's sum is reported with its cause and is never that condition.
13. 13. U4r-F1 (INF-R2-1 with INF-R2-3 and INF-R2-4, as this ruling words them; the two scripts only, no check.mjs change): plan first, then the advisor's plan review, the main session's build, evidence (node --check on both scripts; the scratch helper test extended; the npt-lab command lines unchanged), infra-engineer's review as its one round, the advisor's review of its head, and the commit, before any c-u4r-runs.mjs set. Its size goes on the card.
14. 14. D4: measure the runner's ticks per second in the main folder's own cache (CARGO_TARGET_DIR unset) with r5's command, before any set, and redo the set's wall-time estimate.
15. 15. The disk plan, in writing, before the first c-u4r-runs.mjs set (ruling 1 point 3). Free space is 4,681 MiB now. Still to come: kept logs of about 162 (r2) + 16 (pre-check) + 360 (P3) + 545 (B1) + 629 (P2) + 3,020 (P0) MiB, plus the 4 GiB floor and the main folder's release builds: about 4.1 GiB short. Free space only by the approved part-cleans (W1-TARGET item 3 as amended: .gate/shared-target 8.2G, the main folder's cache 6.8G), between lanes with no lane running, and count any lane that runs during the sets (U4f's copy and builds). If the P0 set still cannot fit, the smaller sets may go first, and P0 goes to Mustafa under Decisions needed (his LEFTOVER-DELETE route). Never delete a log.
16. 16. The pre-check set: `node spikes/netcode-proof/ops/runs/c-u4r-runs.mjs precheck --date <day>` in the main folder, CARGO_TARGET_DIR unset, after F1 and D4. It stops before the pairs if the recording's run refused a rewind (carry 40). The late-command pair counts as expected only on the whole line 'result: VOID (L1 late_count 1)'; anything not as expected comes to the advisor. Commit results/u4r-precheck-30/<commit12>/runs.txt.
17. 17. The judged sets, one at a time, smallest first: P3, B1, P2, then P0 last (it has the most disk, it is the only set over 30 minutes, and it needs F1 and the disk plan). /usr/bin/time is not installed here, so read peak memory with python3's resource.getrusage(RUSAGE_CHILDREN).ru_maxrss around each script call (Linux keeps the reaped tree's maximum) and say so. Ruling 1 point 3 (4) is amended, in the advisor's words, because the script judges a set at its end: the judge's wall time and peak memory come from the earlier sets and from one measured run's kept logs judged apart into a scratch path. P0's judge memory is projected from its bytes, and a projection over 10 GiB (the machine has 15 GiB) comes to the advisor before the P0 set. RR3-L6's read_log share is reported from the same measurement. B1's -17 and -18 verdicts are INVALID: recorded, never counted. Every run flagged for overflow resyncs comes to the advisor before it counts (P0's tick16 bite run is expected to be flagged, about 48: ruling 4 point 1). After each set, commit results/u4r/<commit12>/<profile>/, then fill the report's judged-runs section and the card. U4's report folds U4r's.
18. 18. The other follow-ups wait for their homes, with nothing to do until then: the check.mjs follow-up (SEC-U4R-R2-1, SEC-U4R-R2-2, TE-R2-3, INF-R2-5, INF-U4R-10 (2)) at check.mjs's next change, before PR #40 is marked ready at the latest; and the next npt-lab lane (N2-1 = TE-R2-1, TE-R2-2, RR4-L2, and RR4-L1 before U4h's carry-87 entry at the latest), with round 1's EA-U4R-1/-2 and RR3-L2 (a).
````

### U4r's round-2 ruling: TE-R2-1 (advisor-u4r-r2-full.md)
_`.checkpoints/netcode-proof-test/advisor-u4r-r2-full.md` lines 64-69 (sha256 6dcc0a81a961), as they are:_

````text
### TE-R2-1 (Low; follow-up)

- **What happens:** The same finding as N2-1, fixed once by N2-1's follow-up: the roster check added to closed_checks, and the no-op 'no replacement' assert dropped or reworded.
- **Where it goes:** N2-1's follow-up (the next npt-lab lane).
- **Until then:** As N2-1: nothing.
- **Why:** Confirmed. It is N2-1, raised independently by the test seat.
````

### U4r's round-2 ruling: N2-1 (advisor-u4r-r2-full.md)
_`.checkpoints/netcode-proof-test/advisor-u4r-r2-full.md` lines 71-76 (sha256 6dcc0a81a961), as they are:_

````text
### N2-1 (Low; follow-up)

- **What happens:** closed_checks (scenario.rs:2150-2164) gains a fourth named check: `if server.room.roster().is_live(slot) { failed.push(format!("the roster still holds slot {slot}: a rejoin would be a replacement")); }`. The 'no replacement' assert on `ended` (:2209-2212) is dropped, or its doc says it guards only the lab's own push. Under the leave plant (room.leave deleted), all four checks fail by name in both cases.
- **Where it goes:** The next npt-lab lane (with round 1's EA-U4R-1/-2 and RR3-L2 (a)), with TE-R2-2, RR4-L1 and RR4-L2.
- **Until then:** Nothing: the leave plant is caught by the existing closed checks.
- **Why:** Confirmed. drop_slot (scenario.rs:408-422) pushes (slot, 'replaced') only when it finds a live old session, and FP-1 has already set that session to None (:496). So the assert cannot fail, and the roster half of Room::leave (room.rs:690-695; Room::roster :568, Roster::is_live roster.rs:80) is checked by nothing. The no-op assert is my ruling's own test design (FP-1 TEST). The boundary property itself, that the slot is freed, is proven: the plant fails by name in both cases (c6-plant-fp1.log). The builder disclosed the gap (u4r-build.md:2089-2090). Hence Low.
````

### U4r's round-2 ruling: TE-R2-2 (advisor-u4r-r2-full.md)
_`.checkpoints/netcode-proof-test/advisor-u4r-r2-full.md` lines 78-83 (sha256 6dcc0a81a961), as they are:_

````text
### TE-R2-2 (Low; follow-up)

- **What happens:** fp3's parse test gains the missing cases, each asserting the exact usage error. First, `--profile B1 --bursts 2 --burst-first-s 0 --burst-every-s 1000000000000000000`: the error starts with '--bursts 2, --burst-first-s 0 and --burst-every-s 1000000000000000000: the last burst's tick overflows 64 bits'. Second, `--warmup-s 310000000000000000 --measured-s 310000000000000000`: the error equals '--warmup-s and --measured-s: their ticks' sum overflows 64 bits'. Third, an --after-max-s case for the last refusal. Made in the same change as RR4-L2, so the cases test the final messages.
- **Where it goes:** The next npt-lab lane, together with RR4-L2.
- **Until then:** Nothing: the scripts pass fixed small lengths.
- **Why:** Confirmed. fp3 (scenario.rs:2420-2432) parses only the --measured-s product. The sum (:1376-1378), after-max (:1379-1381) and last-burst (:1443-1454) refusals are reachable and untested, and release npt-lab has no overflow checks (Cargo.toml:116-120). Operator input only.
````

### U4r's round-2 ruling: RR4-L1 (advisor-u4r-r2-full.md)
_`.checkpoints/netcode-proof-test/advisor-u4r-r2-full.md` lines 92-97 (sha256 6dcc0a81a961), as they are:_

````text
### RR4-L1 (Low; follow-up)

- **What happens:** hash.rs gets `const MAX_ENTRY_TICKS: u32 = 36_000;` (four times today's 9,000, the same rule as the 64 MiB cap). manifest() refuses any entry above it, beside the bytes cap, with: 'manifest entry {name}: ticks {ticks} is above 36000 (four times today's largest entry): a longer recording comes to the advisor first'. Add one r10_fp3 case: 36,001 refused by name, and the three committed manifests still read.
- **Where it goes:** The next npt-lab lane, and at the latest before U4h adds its runner entry to det-recordings.json (carry 87).
- **Until then:** No manifest entry is added outside a reviewed unit; the advisor checks the ticks in any manifest diff.
- **Why:** Confirmed. ticks <= bytes/12 with bytes <= 64 MiB allows 5,592,405 ticks (hash.rs:319-331), while real recordings take about 1,855-1,883 B per tick. encode reserves ticks x 1,900 B (replay.rs:296), and a runner entry runs (w+m) x 30 ticks before check() compares anything. It is reachable only by a committed manifest edit, in a dev and CI tool.
````

### U4r's round-2 ruling: RR4-L2 (advisor-u4r-r2-full.md)
_`.checkpoints/netcode-proof-test/advisor-u4r-r2-full.md` lines 99-104 (sha256 6dcc0a81a961), as they are:_

````text
### RR4-L2 (Low; follow-up)

- **What happens:** parse refuses these as usage errors that name their options: a run's last tick (measured_end + after_max_ticks) above u64::from(u32::MAX), and the last burst's tick above the same; --bursts above 1,000; and --burst-every-s 0 with more than one burst. The comment at scenario.rs:1442 says exactly what is bounded. Add one parse case: `--profile B1 --bursts 2 --burst-every-s 600000000000`, refused, naming --burst-every-s.
- **Where it goes:** The next npt-lab lane, together with TE-R2-2.
- **Until then:** Nothing: the run scripts pass fixed small settings (B1: 10 bursts, every 60 s).
- **Why:** Confirmed. due_ns truncates with `as u64` above tick 553,402,322,211 (scenario.rs:131). burst_starts adds `boundary + below(..)` in plain u64 (:1306-1307), and the burst count is unbounded when every_s is 0. :862 casts to u32. Operator command-line input only.
````

### U4r's round-1 ruling: EA-U4R-1 (advisor-u4r-r1-full.md)
_`.checkpoints/netcode-proof-test/advisor-u4r-r1-full.md` lines 166-170 (sha256 7b552e8c9d6f), as they are:_

````text
### EA-U4R-1 (Low; fix now)

- **Builder:** FP-7 item 2
- **Where it goes:** A STATUS follow-up for the next npt-lab lane, before the -30 pre-check's re-run after U4h (carry 40): one shared loop for RQ8's steps 1-3.
- **Why:** Confirmed. There is no L1Scenario item. precheck.rs:350 runs a plain u64 `now`, while its doc (:3) says SimClock, and lib.rs:3-5 places items where they are not. The doc corrections and the report's public-items line are made now. Sharing one loop would refactor the pre-check's run order, and its evidence would then have to be made again, so that part is kept out of this fix pass.
````

### U4r's round-1 ruling: EA-U4R-2 (advisor-u4r-r1-full.md)
_`.checkpoints/netcode-proof-test/advisor-u4r-r1-full.md` lines 172-175 (sha256 7b552e8c9d6f), as they are:_

````text
### EA-U4R-2 (Low; follow-up)

- **Where it goes:** A STATUS follow-up for the same next npt-lab lane as EA-U4R-1, before the pre-check's re-run after U4h: `pub(crate) type Sim = npt_sim::World` and one server constructor used by run and run_leg.
- **Why:** Confirmed. npt_sim::World is named in logs.rs:97, :455 and :578 and in precheck.rs:277, and the room is built in two places, against U4-SEAM (u4-build-plan.md:162-163). Refactoring both construction paths inside a fix pass risks behaviour, and no current result differs. The report names the deviation and its three sites now (FP-9).
````

### U4r's round-1 ruling: EA-U4R-5 (the single route to commands) (advisor-u4r-r1-full.md)
_`.checkpoints/netcode-proof-test/advisor-u4r-r1-full.md` lines 188-192 (sha256 7b552e8c9d6f), as they are:_

````text
### EA-U4R-5 (Low; fix now)

- **Builder:** FP-7 item 4
- **Where it goes:** The U6 brief's bots'-view follow-up (ruling 1 point 7): one route for feeding the endpoint.
- **Why:** Confirmed. The only real host feeds the endpoint itself and passes &[] (scenario.rs:962-968), while the tests use the `received` route (behaviour.rs:92-165). A doc line now; keeping a single route changes npt-bots' public Behaviour, which waits.
````

### U4r's round-1 ruling: RR3-L2 (advisor-u4r-r1-full.md)
_`.checkpoints/netcode-proof-test/advisor-u4r-r1-full.md` lines 226-230 (sha256 7b552e8c9d6f), as they are:_

````text
### RR3-L2 (Low; fix now)

- **Builder:** FP-7 item 6 (part (b))
- **Where it goes:** Part (a): a STATUS follow-up for the next npt-lab lane. On a no-log run `filed` keeps only what `started` needs.
- **Why:** Confirmed by reading. (a) On a no-log run, `filed` grows `res` for the whole run, about 7 MB for a 9,000-tick regeneration: bounded, so it waits. (b) events_stamped is counted only when logs are written (scenario.rs:928-932), so carry 75's bound goes unchecked on --logs 0 and on record() runs. A doc line now says so.
````

### U4r's build ruling 1, point 1 (the determinism route; det-u4.mjs at merges on the scenario path) (advisor-u4r-build1-full.md)
_`.checkpoints/netcode-proof-test/advisor-u4r-build1-full.md` lines 10-15 (sha256 476121ab65e6), as they are:_

````text
### Point 1 (severity: Medium; to Mustafa: no)

- **Decision:** Route (c), the plan's own (u4-build-plan.md :800-802: "in check.mjs if the step stays within 5 minutes, else through ops/runs/det-u4.mjs ... on the advisor's ok"), with my ok. The two runner entries go into a manifest of their own, crates/lab/det-recordings.json, in recordings.json's schema. ops/runs/det-u4.mjs runs the wasm step's clean half on it: regenerate and SHA-check, native leg 1 against leg 2, Wasm leg 1 against native leg 1, plus h2's vectors. It reuses check.mjs's wasmStep by import, never a copy. check.mjs's step keeps the three synthetic recordings. (a) RR-L4 was weighed first and does not fix this: it acts on the Wasm leg only and saves at most about 11.6 s + W, while regeneration takes about 195 s per legs call. Not taken. (b) is not taken: under (c) it saves nothing, and hash.rs `legs` stays as merged (brief 2.6). (d) is taken only inside (c): on its own it drops the plan's evidence. Machine and margin: the 300 s is judged on this container, the only machine that runs check.mjs (CI's gate.yml runs tools/gate.mjs and toolchain.yml runs the root cargo tests; neither runs spikes/; 4 vCPU Xeon @ 2.30 GHz, 16 GB). The figure is the step's printed seconds with builds included (M9's basis), labelled warm or cold with uptime, and cold binds. A recording set stays in check.mjs only while that cold figure is at most 200 s (two-thirds of 300 s). det-u4.mjs's non-cargo calls must stay at most 240 s inside carry 84's shared 300 s deadline; above 240 s it comes to me, with RR-L4 weighed first then. Words this changes: carry 82's "U4r's runner recordings join crates/lab/recordings.json" now reads "join crates/lab/det-recordings.json, in recordings.json's schema". Those are the reviewers' EA-8 and RR-L5 words as ruled in the advisor's U4d2 round-1 ruling; the advisor amends them here, and the rule (kind key, dispatch, name and ticks checks) is unchanged. Carry 87 (my U4r brief check's words) now reads: "adds it to crates/lab/det-recordings.json under carry 82's schema, and states det-u4.mjs's time with it, its non-cargo calls against carry 84's shared 300 s deadline (over 240 s it comes to the advisor, RR-L4's remedy weighed first)". Plan :800-802's trigger list widens to every crate on the scenario's path. The brief's 2.1 (i), section 5 item 4 and return list item 9 read as this ruling says.
- **Builder:** Point 1, the determinism route: (c), with the advisor's ok. (1) crates/lab/recordings.json stays as you have it: the three synthetic entries, each with "kind": "synthetic", and nothing else. (2) New file crates/lab/det-recordings.json (this ruling opens it). It uses recordings.json's schema and is read by the same `manifest`. It holds the two runner entries exactly as your notes :225-230 give them, SS-P2-s1 then ES-P3-s2, with an `about` text naming this ruling. hash.rs's h5 also asserts it: two entries, both kind runner, generator 0. (3) hash.rs `legs` stays as merged: no regeneration limited to `--only`'s entries. (4) check.mjs (this ruling opens this much beyond carries 83-85): `wasmStep` takes two options, the manifest path (default RECORDINGS_MANIFEST) and whether its bites half runs (default true). With the defaults it runs and prints exactly what it does now. With the bites half off it starts no bites-build call and says the half did not run by that option. Add one check.test.mjs case with the fake runner: a non-default manifest reaches every `npt-lab hash` call's `--manifest` and the loader's recordings, and the bites half off starts no bites-build call. Carries 83 (b) to (d) and 84 stay yours in `wasmStep`, as section 4's lane says, so det-u4.mjs inherits the exit-code rule, the one 300 s deadline shared from the first non-cargo call, and a stopped call taking every process it started. (5) New ops/runs/det-u4.mjs: it imports `wasmStep` and calls it with crates/lab/det-recordings.json, the bites half off, and its own folder under .gate/ (never the step's). It prints its lines and summary as check.mjs's main prints the step's, and exits 0 only on PASS. Section 4's no-retry rule for the wasm step binds it. (6) Run it once through the runner with -x (df first; the 4 GB floor), logging to .gate/u4r/det-u4.log. Its PASS line, every call's seconds and the two Wasm legs' seconds (W) go into your notes and report. If its calls other than cargo builds sum above 240 s, stop and report before going on. (7) Section 5 item 4 and return list item 9 now read: check.mjs's wasm step seconds on the three synthetic recordings (warm or cold, with uptime), det-u4.mjs's time beside them, and this ruling as the route.
- **Main session:** Sends the builder text and records the ruling on the card. In its own commit it amends carry 82's location words and carry 87's words in u4s-carries.md, each with a pointer to this ruling. At its next plan edit it amends u4-build-plan.md :800-802: det-u4.mjs runs at every merge that touches npt-sim, npt-proto, npt-room or npt-hash-wasm (the plan's list) or any crate on the scenario's path: npt-server, npt-client, npt-l1, npt-link, npt-bots, npt-lab (brief 2.1 (iii)). It adds det-u4.mjs to U4r's merge step (main-run.sh -x, after check.mjs) and to every later merge on that list. A SHA-256 failure there means that unit re-pins, never a retry. The review panel (section 12 item 7): infra-engineer reads det-u4.mjs and wasmStep's two options (carry 84's seat), and engine-architect counts the check.mjs changes (2.7).
- **Why:** As merged, the three legs calls (measured, hash-map, std-sin: check.mjs:2319, :2374, :2432) each regenerate every manifest entry (hash.rs:836-857). The file times show about 195.2 s per call for the two runner entries (142.6 + 52.6). So the step would take about 621 s + W warm and 650 s + W cold (fact check). The builder's "~170 s" was not a measured figure. Today's per-call limit would stop nothing (the largest call is 201.0 s), but carry 84 (a)'s shared deadline would stop the step at the hash-map legs call. (b) would bring it to about 259 s + W cold. That leaves about 40 - W s, and under section 4's no-retry rule a stopped call stops the step group. Every check.mjs run would also grow by about 200 s under the exclusive lock. U4h's TC B1 recording (carry 87) would push it over anyway. And every change on the scenario path would fail check.mjs's SHA until re-pinned. (d) alone drops "U4d2's step green on them" (plan :800-802; §4.15 :500-501). The clean half is enough: the bites half proves the comparer catches a planted divergence, and check.mjs shows that on SS-s1 every run, on the same code. Not for Mustafa: the plan gives this route "on the advisor's ok". No word or decision of his changes. Risk tier 1 (spike code and checkpoint text; check.mjs is the spike's own check, not a gate file).
````

### U4r's build ruling 1, point 7 (the brawl's view; an npt-client read item, R16) (advisor-u4r-build1-full.md)
_`.checkpoints/netcode-proof-test/advisor-u4r-build1-full.md` lines 52-57 (sha256 476121ab65e6), as they are:_

````text
### Point 7 (severity: Low; to Mustafa: no)

- **Decision:** Accepted as built. The brawl aims from the room-side frame of that client's newest tick: the mirror, fed with the room's datagrams before the link. The report states this. npt-client exposes no decoded record (R16), so a client-side view would need a seam change. The mirror view is deterministic and independent of the link's fates, so the twin test and the recordings hold. No U4r mark judges the brawl's aim, and the open-loop scripts read no view. A follow-up goes to U6's brief, where bots run apart from the room and so need the client's own view.
- **Builder:** Point 7: accepted as built; no change. The report's words: "the brawl aims from the room-side frame of that client's newest tick (the mirror, fed before the link: scenario.rs:916-921, mirror.rs:213-225), which may hold entities from datagrams the client lost or refused, or a baseline it never completed; no U4r mark judges the brawl's aim; the open-loop scripts read no view."
- **Main session:** Adds a card follow-up for U6's brief (npt-bots' npt-net glue, plan :897-898). There the bots run beside no room-side mirror, so the brawl's view must come from the client's own decoded state: an npt-client read item (R16) or U4g's Interp.
- **Why:** ClientEndpoint's pub API returns no frame or records (client/src/lib.rs:253-415). The mirror is fed before server.send, so before L1's fates (scenario.rs:830-852; mirror.rs:1-4). Only Kind::Brawl reads the view (bots lib.rs:204-207). The judged figures (-17, -57, -18, PM-0001-2's bands) do not judge aim fidelity. The shot rate comes from fire held on every command, and D13 shows about 698 shots per measured second at P2 lead 5, inside the 665-735 band.
````

### The proof test's plan: §4.2, narrow public surfaces (plan.md)
_`.checkpoints/netcode-proof-test/plan.md` lines 454-454 (sha256 3ca4e12a39c9), as they are:_

````text
- **Narrow public surfaces:** each crate's public items are those in §4.7; everything else is `pub(crate)`.
````

### marks.json: PM-0001-17 (spikes/netcode-proof/marks/marks.json)
_`spikes/netcode-proof/marks/marks.json` lines 3844-4021 (sha256 9fb5be578b27), as they are:_

````text
    {
      "id": "PM-0001-17",
      "title": "Every decoded snapshot is the server's state",
      "draft": {
        "file": "docs/adr/0001-multiplayer-netcode.md",
        "row_sha256": "3075f6384860068622268fc3efff47db2b8a29624d063882d72beab37eedffea"
      },
      "verdict": "pass",
      "where": [
        "C",
        "R3"
      ],
      "parts": [
        {
          "name": "L1 P0",
          "select": {
            "layer": "L1",
            "scene": "SS",
            "profile": "P0",
            "clients": 48
          },
          "min": 3,
          "validity": [
            "pre:PM-0001-2"
          ]
        },
        {
          "name": "L1 P2",
          "select": {
            "layer": "L1",
            "scene": "SS",
            "profile": "P2",
            "clients": 48
          },
          "min": 3,
          "validity": [
            "pre:PM-0001-2"
          ]
        },
        {
          "name": "L1 P3",
          "select": {
            "layer": "L1",
            "scene": "SS",
            "profile": "P3",
            "clients": 48
          },
          "min": 3,
          "validity": [
            "pre:PM-0001-2"
          ]
        },
        {
          "name": "L1 B1",
          "select": {
            "layer": "L1",
            "scene": "SS",
            "profile": "B1",
            "clients": 48
          },
          "min": 3,
          "validity": [
            "pre:PM-0001-2"
          ]
        },
        {
          "name": "L2 P0",
          "select": {
            "layer": "L2",
            "profile": "P0",
            "machine.role": "LG"
          },
          "min": 1
        },
        {
          "name": "L2 P2",
          "select": {
            "layer": "L2",
            "profile": "P2",
            "machine.role": "LG"
          },
          "min": 1
        },
        {
          "name": "L2 P3",
          "select": {
            "layer": "L2",
            "profile": "P3",
            "machine.role": "LG"
          },
          "min": 1
        },
        {
          "name": "L2 B1",
          "select": {
            "layer": "L2",
            "profile": "B1",
            "machine.role": "LG"
          },
          "min": 1
        },
        {
          "name": "every S30",
          "select": {
            "params.soak": "S30"
          },
          "min": 3
        }
      ],
      "validity": [],
      "checks": [
        {
          "name": "100 % of decoded snapshots rebuild the server's quantised state (0 mismatches; on L2 under the Arm rule)",
          "figure": "decoded_hash_mismatch_count",
          "op": "==",
          "value": 0
        },
        {
          "name": "setup: the S30 starts 1,800 ticks below 65,536, so it crosses the 16-bit wrap",
          "series": "header.params.tick_start",
          "stat": "max",
          "op": "==",
          "value": 63736,
          "parts": [
            "every S30"
          ],
          "setup": true
        },
        {
          "name": "L1 P0 loses no group (no client-tick left incomplete)",
          "series": "tick.s.snap_incomplete",
          "stat": "sum",
          "window": "measured",
          "op": "==",
          "value": 0,
          "parts": [
            "L1 P0"
          ]
        }
      ],
      "report": [
        {
          "name": "aarch64-only mismatches (a cross-architecture finding; decides nothing)",
          "figure": "cross_arch_mismatch_count"
        },
        {
          "name": "decoded snapshots compared",
          "figure": "decoded_hash_compared_count"
        },
        {
          "name": "client-ticks left incomplete (a group lost, none refused: not compared, u4-build-plan.md §4.6)",
          "series": "tick.s.snap_incomplete",
          "stat": "sum",
          "window": "all"
        }
      ],
      "bites": [
        {
          "id": "PM-0001-17/tick16",
          "what": "a 16-bit tick in the snapshot header (fails at the wrap: the bite run must cross it)"
        },
        {
          "id": "PM-0001-17/client-rounding",
          "what": "a client quantiser that rounds differently from the server's"
        },
        {
          "id": "PM-0001-17/whole-as-lost",
          "what": "a client writer that files a tick it decoded whole as lost: null and incomplete 1"
        }
      ],
      "stand_in": false,
      "arm_rule": true,
      "readings": [
        "R-5",
        "R-8"
      ],
      "notes": "L1: 48 headless clients, 3 seeds per profile, 10 minutes each under P0, P2, P3 and B1, in the stress scene (its L1 parts select scene SS and 48 clients and carry pre:PM-0001-2: u4-build-plan.md §4.12, the advisor's Q22 ruling); L2: LG's 48 clients, 1 run per profile (R3, VS4; their scene is the R run plans'). The server logs each client's quantised hash per tick (`hs.server_hash`), the client its decoded hash under the tick it rebuilt (`hs.decoded_hash`: the sentinel 0 for a tick with a group it refused, null for a tick it could not complete, counted in `tick.s.snap_incomplete`: §4.6, the advisor's Q11 ruling); each LG client logs every datagram and stream byte it receives (kept on LG). Arm rule: an L2 mismatch counts only once those bytes, decoded again on x86-64 (npt-lab, in this container or on RS: R-8; `hs.replay_hash`, process replay), miss the server's hash too; a mismatch only on aarch64 is reported. The figure fails, never passes, on 0 compared snapshots; a client hash at a tick the server never logged counts as a mismatch."
    },
````

## The carries this packet holds

_Rule: every entry of `.checkpoints/netcode-proof-test/u4s-carries.md` whose text matches `^(18|39|74|75|82|87)\. \*\*`._

### Carry 18
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 129-132 (sha256 c74e52553bd8), as they are:_

````text
18. **this pass (the tick grep):** "A new tick-count constant in crates/sim/src joins check.mjs's `TICK_DERIVED` as
    `<name in lower case>_at(TICK_HZ)`, its function pinned at 30 and 60 Hz in s1's `DERIVED` table and tied by a const
    assert in ticks.rs; the deps step refuses any other form (no mechanical check ties s1's table to `TICK_DERIVED`:
    review keeps the two lists equal)."
````

### Carry 39
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 169-179 (sha256 c74e52553bd8), as they are:_

````text
39. **rust fix review nit (u4s-fix-review-rust.md:114-115; the advisor's U4s ruling, condition 2):** "A tick count
    needed outside npt-sim goes into npt-sim as `<name>_at` plus its constant (carry 18's route), and the crate uses it
    by path. Why: check.mjs:627 lets an initializer that merely names `TICK_HZ` through, and nothing pins such a value
    at 60 Hz." (Route, the advisor's F2 ruling, 2026-11-09: the plan names no new tick count for U4e; its counts are
    `FULL_STATE_ACK_TICKS` and `ACK_RING_TICKS`, in npt-sim since U4s, and its deadlines and limits run in ns on the
    injected now. The Input bucket's refill (2 × TICK_HZ a second) and the window's per-tick budget (256 kbps) follow
    carry 6's pattern: the type takes the rate as an argument, production callers pass `npt_sim::TICK_HZ`, a test pins
    each at 30 and 60 Hz, and no const or static outside npt-sim is derived from TICK_HZ. If U4e finds it needs a new
    tick count or a named constant derived from the rate, it writes no local form and records the need in its report;
    the main session adds it by carry 18's route in its own commit once U4d2 has merged, and the site and U4e's merge
    follow it.)
````

### Carry 74
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 338-349 (sha256 c74e52553bd8), as they are:_

````text
74. **NC-M6, the endpoint's lead (u4e-review-netcode.md, Block h; the advisor's U4e round-1 ruling, R3):** "U4e's
    `ClientEndpoint` stamps each command `newest + lead`, with a lead of 1 by default (crates/client/src/lib.rs), and
    U4f's lead controller comes after U4r's runs. On a link whose one-way delay is above about half a tick period every
    command would reach the room after its tick and be dropped as late, `last_run` would stay `None`, and option B
    would have nothing to repeat: -17 would compare a world of soldiers standing still. U4r sets each endpoint's lead
    with `ClientEndpoint::with_lead` from the run's profile (the round trip plus the jitter, in ticks rounded up, at
    least 1, plus 1: `npt_sim::ticks_for_ms(rtt_ms + jitter_ms).max(1) + 1`, so P0 2, P1 3, P2 and B1 5, P3 7, within
    `CMD_CATCHUP` = 8; its bots give a command only at a client step whose newest snapshot tick moved since their last
    command, and move each press counter by at most one edge per command; the advisor's U4r brief check, U4R-Q14),
    records it with `cmd_timing` in the header, and its contract test reports on each short run the commands that
    arrived late (the room's `late` count) beside the commands sent; a run with more than 1 % of its commands late comes
    to the advisor before its marks count."
````

### Carry 75
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 350-358 (sha256 c74e52553bd8), as they are:_

````text
75. **NC-M5 and A3, the client's event-number base (u4e-review-netcode.md, Block h; u4e-review-arch.md, A3; the test
    seat's probe Q-10; the advisor's U4e round-1 ruling, R3):** "U4e's client rebuilds each event's 16-bit number near
    its own start (`ClientEndpoint::with_starts`, 0 by default), and the wire carries no event base (`JoinAccepted` and
    `FullState` have none), so a client whose start is 32,768 or more from the room's counter rebuilds every event
    number wrong, and the hash reads them (Q-10). U4r's lab gives each endpoint, through `with_starts`, the event start
    it gave the room (`Room::with_event_start`) and its command start, and admits every client before the room has
    stamped 32,768 events past that start (about 30 s of SS's events); a later admission comes to the advisor first
    (the room exposes no event counter). The contract test shows `decoded_hash_mismatch_count` 0 on a run whose event
    start is far from 0 (-58/lt-event-seq's, 3 below the wrap, among them). The wire field is U6's (C-U6-13)."
````

### Carry 82
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 377-386 (sha256 c74e52553bd8), as they are:_

````text
82. **EA-8 with RR-L5, the recordings manifest (u4d2-review-arch.md, EA-8; u4d2-review-rust.md, RR-L5; the advisor's
    U4d2 round-1 ruling, Blocks 4 and 5):** "U4r's runner recordings join crates/lab/recordings.json (§4.15), whose
    entries today hold only the synthetic generator's inputs, read by hash.rs's `manifest` and generated by its
    `regenerate_all`: U4r changes that schema (a kind or script key, and the dispatch both functions need), and its
    brief says so. With that change, the reader checks each entry's name against `[A-Za-z0-9-]+` and bounds `ticks` by
    `bytes / 12` before it generates anything (today a bad name reaches a file path and the `h <recording> <tick>
    <hash>` line, and a large `ticks` reaches `Vec::with_capacity` before the length and SHA-256 check)."
    **Amended (the advisor's U4r build ruling, point 1; advisor-u4r-build1-full.md):** "U4r's runner recordings join
    crates/lab/recordings.json" now reads "join crates/lab/det-recordings.json, in recordings.json's schema"; the rule
    (kind key, dispatch, name and ticks checks) is unchanged.
````

### Carry 87
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 572-578 (sha256 c74e52553bd8), as they are:_

````text
87. **U4R-Q9, the TC B1 seed 3 recording (the advisor's U4r brief check):** U4h records TC B1 seed 3 with `scenario
    --record` and its TC shooter, adds it to crates/lab/recordings.json under carry 82's schema, and states the `wasm`
    step's time with it against the 5-minute budget.
    **Amended (the advisor's U4r build ruling, point 1; advisor-u4r-build1-full.md):** from "adds it to" on, it now
    reads "adds it to crates/lab/det-recordings.json under carry 82's schema, and states det-u4.mjs's time with it, its
    non-cargo calls against carry 84's shared 300 s deadline (over 240 s it comes to the advisor, RR-L4's remedy
    weighed first)".
````

## The carries this packet left out

Ids: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 76, 77, 78, 79, 80, 81, 83, 84, 85, 86, 88, 89.
