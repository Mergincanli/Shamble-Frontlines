# Review packet: U4d-F round 1 (the estimate scene fix: N2-M1's reckoning and N2-M2's vehicles, before U4d2 pins the recordings)

_Cut by `review_packet.py` from `u4df-packet.json`. It holds sources only: code diffs, the approved plan's lines, the advisor's rulings, the last round's findings and Mustafa's words, each copied by line range. It holds no conclusion of the builder or the main session (no builder notes, brief, checkpoint card or summary). You stay free to read any file and run any test._

## Heads

- Base: dad9566 on the spike branch, where U4d-F's build started; its spikes/netcode-proof/crates/sim equals the branch head's (U4d's merge 39f4290; nothing in crates/sim has changed since).
- U4d-F's copy: `.claude/worktrees/wf_9ce88fa3-452-1` (branch u4df-build; the change is uncommitted there).
- The diffs below compare the base copy `.gate/u4df-prefix` (a worktree at dad9566) with U4d-F's copy.

## What changed (the diffs)

### npt-sim's sources (crates/sim/src)
_`git diff --no-index .gate/u4df-prefix/spikes/netcode-proof/crates/sim/src .claude/worktrees/wf_9ce88fa3-452-1/spikes/netcode-proof/crates/sim/src`:  3 files changed, 274 insertions(+), 79 deletions(-)._

````diff
diff --git a/.gate/u4df-prefix/spikes/netcode-proof/crates/sim/src/replay.rs b/.claude/worktrees/wf_9ce88fa3-452-1/spikes/netcode-proof/crates/sim/src/replay.rs
index 3c6a293..b45d5c9 100644
--- a/.gate/u4df-prefix/spikes/netcode-proof/crates/sim/src/replay.rs
+++ b/.claude/worktrees/wf_9ce88fa3-452-1/spikes/netcode-proof/crates/sim/src/replay.rs
@@ -19,8 +19,10 @@ use rand_core::{Rng, SeedableRng};
 
 use crate::scene::{LARGE_PARTS, SMALL_PARTS, Scene};
 use crate::seam::{
-    Attribute, AttributeChange, Command, CoverVariant, Placement, Rewind, SimCore, TickInput,
+    Attribute, AttributeChange, Command, CoverVariant, MOVE_SPEED_MAX_CM_S, Placement, Rewind,
+    SimCore, TickInput,
 };
+use crate::world::move_velocity;
 use crate::{HitHistory, StateDump, TICK_HZ, World, ticks_for_ms};
 
 const MAGIC: u32 = u32::from_le_bytes(*b"NPTR");
@@ -281,8 +283,9 @@ fn write_tick(out: &mut Vec<u8>, t: &TickInput) {
 }
 
 impl Recording {
-    /// The synthetic generator's version: a recordings manifest names it beside each recording's seed (§4.15).
-    pub const GENERATOR: u32 = 1;
+    /// The synthetic generator's version: a recordings manifest names it beside each recording's seed (§4.15). 2 from
+    /// U4d-F: the dead reckoning follows every input the generator emits (the advisor's U4d close ruling, B1.2 step 3).
+    pub const GENERATOR: u32 = 2;
 
     /// The recording's bytes (the module's layout).
     ///
@@ -349,8 +352,18 @@ impl Recording {
     /// and then, and reload on the courses; a rare aim point lies beyond the weapon's reach (the clamp) and a rare
     /// Rewind outside the
     /// ring (the refusal). Each firing command's Rewind names ticks the ring holds, and its view time is that Rewind's.
-    /// Every few hundred ticks a soldier is spawned at its home, teleported there, or its move speed set. The commands
-    /// are written in the canonical order (carry 8).
+    /// Every few hundred ticks a soldier is spawned at its home, teleported there, or its move speed set; in ES (its
+    /// `aim_one_in` above 1) each soldier is also teleported home once every 60 s. The commands are written in the
+    /// canonical order (carry 8).
+    ///
+    /// The dead reckoning follows every input the generator emits, as the World applies it (the advisor's U4d close
+    /// ruling, B1.2 step 3). A tick's spawn, teleport and move speed are drawn before its commands, since the World
+    /// applies them first: a spawn or teleport puts the estimate at home, and a move speed is held as the World's
+    /// `apply_inputs` brings it into range. Each command the World runs moves the estimate one tick by the World's own
+    /// move rule (`move_velocity`: crouch, sprint, the vector's scale) at that speed, in the order the World runs them,
+    /// so a held-back command moves it when it is sent, beside the next, and a lost one never does. What the World does
+    /// that no input says (a move blocked by a solid, a door, the wall or a vehicle part; a death and its respawn; the
+    /// position's snap to the wire grid) the reckoning cannot see: the recording stays open-loop (§4.15).
     pub fn synthetic(
         scene: &Scene,
         seed: u64,
@@ -374,6 +387,9 @@ impl Recording {
             buttons: u16,
             turn_at: u32,
             est: [f32; 2],
+            /// The move speed the World holds for the slot, cm/s: the scene's, then each MoveSpeed the generator
+            /// emits for it, brought into range as `apply_inputs` brings it.
+            speed_cm: u32,
             held_back: Option<(Command, Option<Rewind>)>,
         }
         let homes = scene.homes;
@@ -382,10 +398,16 @@ impl Recording {
         let mut clients: Vec<Client> = (0..slots)
             .map(|s| Client {
                 est: home_of(s).map(|k| k as f32 / 100.0),
+                speed_cm: scene.move_speed,
                 ..Client::default()
             })
             .collect();
-        let speed = scene.move_speed as f32 / 100.0;
+        // One tick's move of a command the World runs, before collision: the World's own rule at the speed it holds.
+        let dt = 1.0 / TICK_HZ as f32;
+        let one_tick = |speed_cm: u32, buttons: u16, move_xy: [i8; 2]| -> [f32; 2] {
+            let v = move_velocity(speed_cm, buttons, move_xy, 1.0).0;
+            [v[0] * dt, v[1] * dt]
+        };
         let reach = scene.weapon.reach_cm as i32;
         let mut out = Vec::with_capacity(ticks as usize);
         for e in 0..ticks {
@@ -394,11 +416,64 @@ impl Recording {
                 tick,
                 ..TickInput::default()
             };
+            // Phase 0's inputs first: the World applies a tick's spawns, teleports and attribute changes before its
+            // commands, so the reckoning takes them before this tick's moves.
+            if e % 311 == 157 {
+                let slot = draw(u32::from(slots)) as u8;
+                if !scene.is_plate(slot) {
+                    let value = 400 + draw(400);
+                    input.attributes.push(AttributeChange {
+                        slot,
+                        attribute: Attribute::MoveSpeed,
+                        value,
+                    });
+                    clients[usize::from(slot)].speed_cm = value.min(MOVE_SPEED_MAX_CM_S);
+                }
+            }
+            if e % 457 == 300 || e % 997 == 500 {
+                let slot = draw(u32::from(slots)) as u8;
+                if !scene.is_plate(slot) {
+                    let home = home_of(slot);
+                    let p = Placement {
+                        slot,
+                        position: [home[0], home[1], 0],
+                    };
+                    clients[usize::from(slot)].est = home.map(|k| k as f32 / 100.0);
+                    if e % 457 == 300 {
+                        input.teleports.push(p);
+                    } else {
+                        input.spawns.push(p);
+                    }
+                }
+            }
+            // ES only (its `aim_one_in` above 1), the bounded re-home (the advisor's U4d close ruling, B1.2 step 4's
+            // 3b): once every 60 s each soldier is teleported to its home, slot k on the ticks where e mod 1,800 =
+            // floor(k × 1,800 / slots), and its estimate with it, so what the reckoning cannot see (a blocked move,
+            // above all) builds up for at most 60 s. It draws nothing.
+            if scene.aim_one_in > 1 {
+                let period = ticks_for_ms(60_000);
+                for slot in (0..slots).filter(|&k| !scene.is_plate(k)) {
+                    if u32::from(slot) * period / u32::from(slots) == e % period {
+                        let home = home_of(slot);
+                        input.teleports.push(Placement {
+                            slot,
+                            position: [home[0], home[1], 0],
+                        });
+                        clients[usize::from(slot)].est = home.map(|k| k as f32 / 100.0);
+                    }
+                }
+            }
             for slot in 0..slots {
                 if scene.is_plate(slot) {
                     continue;
                 }
                 let c = &mut clients[usize::from(slot)];
+                // A command held back last tick runs first this tick, beside this tick's: its own buttons and vector
+                // at the speed the World holds now.
+                if let Some((back, _)) = &c.held_back {
+                    let d = one_tick(c.speed_cm, back.buttons, back.move_xy);
+                    c.est = [c.est[0] + d[0], c.est[1] + d[1]];
+                }
                 // A new heading every 0.5 to 2 s: home when it has wandered more than 8 m, else at random.
                 if e >= c.turn_at {
                     c.turn_at = e + ticks_for_ms(500 + draw(1_500));
@@ -428,8 +503,10 @@ impl Recording {
                         _ => {}
                     }
                 }
-                c.est[0] += f32::from(c.mv[0]) / 127.0 * speed / TICK_HZ as f32;
-                c.est[1] += f32::from(c.mv[1]) / 127.0 * speed / TICK_HZ as f32;
+                // Where this command's move puts the soldier when the World runs it (the estimate moves there only
+                // when it is sent).
+                let d = one_tick(c.speed_cm, c.buttons, c.mv);
+                let after = [c.est[0] + d[0], c.est[1] + d[1]];
                 if draw(40) == 0 {
                     c.presses[Command::PRESS_FIRE] = c.presses[Command::PRESS_FIRE].wrapping_add(1);
                 }
@@ -455,12 +532,13 @@ impl Recording {
                         50 + draw(150) as i32,
                     ]
                 };
-                // ES only (its `aim_one_in` above 1; the other scenes draw nothing more, so their recordings are as
-                // before): on all but one command in `aim_one_in`, none that throws, the aim is 40 m above the shooter's
-                // own reckoning, so the shot rises almost straight up and meets only a soldier whose box holds the
-                // shooter's chest (ES's 10 gameplay events a second: T-M1).
+                // ES only (its `aim_one_in` above 1; no other scene draws for it): on all but one command in
+                // `aim_one_in`, none that throws, the aim is 40 m above where this command's move puts the shooter by
+                // its own reckoning, so the shot rises almost straight up and meets only a soldier whose box holds the
+                // shooter's chest (ES's 10 gameplay events a second over the judged window and each half of it: T-M1,
+                // set on that window by the advisor's U4d close ruling, B1.2 step 5).
                 let aim = if scene.aim_one_in > 1 && !throws && draw(scene.aim_one_in) != 0 {
-                    [(c.est[0] * 100.0) as i32, (c.est[1] * 100.0) as i32, 4_000]
+                    [(after[0] * 100.0) as i32, (after[1] * 100.0) as i32, 4_000]
                 } else {
                     aim
                 };
@@ -498,40 +576,19 @@ impl Recording {
                     view_tick: rewind.map_or(tick.wrapping_sub(3), |r| r.tick_a),
                     view_fraction: rewind.map_or(0, |r| r.fraction),
                 };
+                // Sent now (the estimate moves), held back (it moves when sent, next tick) or lost (it never moves).
                 if let Some(back) = c.held_back.take() {
                     input.commands.push(back);
                     input.commands.push((cmd, rewind));
+                    c.est = after;
                 } else {
                     match draw(64) {
                         0 => c.held_back = Some((cmd, rewind)),
                         1 => {}
-                        _ => input.commands.push((cmd, rewind)),
-                    }
-                }
-            }
-            if e % 311 == 157 {
-                let slot = draw(u32::from(slots)) as u8;
-                if !scene.is_plate(slot) {
-                    input.attributes.push(AttributeChange {
-                        slot,
-                        attribute: Attribute::MoveSpeed,
-                        value: 400 + draw(400),
-                    });
-                }
-            }
-            if e % 457 == 300 || e % 997 == 500 {
-                let slot = draw(u32::from(slots)) as u8;
-                if !scene.is_plate(slot) {
-                    let home = home_of(slot);
-                    let p = Placement {
-                        slot,
-                        position: [home[0], home[1], 0],
-                    };
-                    clients[usize::from(slot)].est = home.map(|k| k as f32 / 100.0);
-                    if e % 457 == 300 {
-                        input.teleports.push(p);
-                    } else {
-                        input.spawns.push(p);
+                        _ => {
+                            input.commands.push((cmd, rewind));
+                            c.est = after;
+                        }
                     }
                 }
             }
diff --git a/.gate/u4df-prefix/spikes/netcode-proof/crates/sim/src/scene.rs b/.claude/worktrees/wf_9ce88fa3-452-1/spikes/netcode-proof/crates/sim/src/scene.rs
index 524d6b3..7481231 100644
--- a/.gate/u4df-prefix/spikes/netcode-proof/crates/sim/src/scene.rs
+++ b/.claude/worktrees/wf_9ce88fa3-452-1/spikes/netcode-proof/crates/sim/src/scene.rs
@@ -75,9 +75,13 @@ const ES_WEAPON: Weapon = Weapon {
     ..SS_WEAPON
 };
 /// ES's recording aims among the other teams on one command in this many ([`Scene`]'s `aim_one_in`; the advisor's
-/// W1-U4D T-M1). Measured over ES's 9,000-tick recording at 1 in 11: 9.85, 10.26 and 9.67 gameplay events a second for
-/// seeds 2, 1 and 3 (shot hits about 4.4, blast hits about 5.3, kills about 0.2); 1 in 10 gives 10.39 for seed 2.
-const ES_AIM_ONE_IN: u32 = 11;
+/// W1-U4D T-M1, set on the judged window by its U4d close ruling, B1.2 step 5). Measured over the window, the last
+/// 18,000 ticks (10 minutes) of a 21,600-tick ES recording from tick 1,000,000, after its 2-minute warm-up: 9.878,
+/// 9.743 and 10.098 gameplay events a second for seeds 1, 2 and 3 (shot hits about 2.7, blast hits about 6.8, kills
+/// about 0.35), each 300 s half between 9.580 and 10.220 (tests/world.rs, t_m1 and its three-seed form). Of the shares
+/// tried (11, 16, 26, 34, 36, 38, 40, 48), 40 and 48 put every window and half in 9.5-10.5, and 40 keeps the larger
+/// smallest margin to the band's edges (0.080 a second; 48's is 0.036).
+const ES_AIM_ONE_IN: u32 = 40;
 /// An S30 (a 30-minute run) and 5 minutes more, µs.
 const S30_PLUS_5_US: u64 = 35 * 60 * 1_000_000;
 // SS and ES script no reloads (TUNE-1), so a soldier firing at its weapon's interval must not run dry inside an S30
@@ -100,18 +104,26 @@ pub struct Scene {
     /// Big vehicles, and how many of them are very large units of `LARGE_PARTS` parts (the rest have `SMALL_PARTS`).
     pub(crate) vehicles: u8,
     pub(crate) large_vehicles: u8,
+    /// How many big vehicles loop inside the fight, the first this many (the advisor's U4d close ruling, B1.3): round
+    /// the soldiers' cluster's centre, 5 to 14 m out, every hull within 1,400 cm of it on every tick; vehicle v of the
+    /// rest loops 40 + 3 v m out (SS's from 52 m). SS's 4 very large units; all 15 of ES's (ADR 0001:1475-1477: every
+    /// vehicle in view).
+    pub(crate) fight_vehicles: u8,
     /// Stand-in projectiles the sim keeps live (the stress scene's 300).
     pub(crate) live_projectiles: u16,
     /// Stand-in projectiles spawned per second, in thousandths (ES's 0.44 per player per second: 21,120).
     pub(crate) spawn_millihertz: u32,
     /// The stand-in projectiles' fuse range, ms.
     pub(crate) fuse_ms: [u32; 2],
-    /// A thrown grenade's fuse, ms: 2 s, and ES's 1.515 s (every ES projectile fuses after 1.515 s: ADR 0001:1478).
+    /// A thrown grenade's fuse, ms: 2 s, and ES's 1.515 s (ADR 0001:1478-1479's 32 live projectiles at 21.12 spawns a
+    /// second give each a life of 32 / 21.12 = 1.515 s, so every ES projectile fuses after it).
     pub(crate) grenade_fuse_ms: u32,
     /// Its synthetic recordings aim among the other teams' soldiers on one command in this many (and on every command
     /// that throws), and into the air above the shooter on the rest, so those shots rise almost straight up and meet
     /// only a soldier whose box holds the shooter's chest (soldiers pass through each other): 1 in every scene but ES,
-    /// where the share sets its 10 gameplay events a second (ADR 0001:1478; the advisor's W1-U4D T-M1).
+    /// where the share sets its 10 gameplay events a second over the judged window, the last 10 minutes of a 12-minute
+    /// recording, and over each 300 s half of it (ADR 0001:1478 and :1600; the advisor's W1-U4D T-M1 and U4d close
+    /// ruling, B1.2). Above 1 it also turns on the generator's re-home of each soldier every 60 s (B1.2 step 4's 3b).
     pub(crate) aim_one_in: u32,
     /// The projectile pool's fixed capacity (stand-ins and grenades).
     pub(crate) projectile_capacity: u16,
@@ -155,6 +167,7 @@ impl Scene {
             plates_from: 48,
             vehicles: 15,
             large_vehicles: 4,
+            fight_vehicles: 4,
             live_projectiles: 300,
             spawn_millihertz: 0,
             fuse_ms: [2_000, 6_000],
@@ -171,16 +184,19 @@ impl Scene {
         }
     }
 
-    /// ES, R2's estimate scene (ADR 0001:1474-1479): 48 soldiers and 15 big vehicles in view, 1.5 shots per player per
-    /// second (the weapon's interval 666,667 µs), 32 live projectiles at 0.44 spawns per player per second (21.12 a
-    /// second, each fusing after 1.515 s: 21.12 × 1.515 = 32.0; a thrown grenade is one of them, and the stand-ins
-    /// fill the rest), 10 gameplay events (hits and kills) a second from its recording's aim (one command in 11 aims
-    /// among the other teams), no effect rows (the advisor's W1-U4D T-M1).
+    /// ES, R2's estimate scene (ADR 0001:1474-1479): 48 soldiers and 15 big vehicles in view (all 15 loop inside the
+    /// fight: the advisor's U4d close ruling, B1.3), 1.5 shots per player per second (the weapon's interval 666,667
+    /// µs), 32 live projectiles at 0.44 spawns per player per second (21.12 a second, each fusing after 1.515 s: 21.12
+    /// × 1.515 = 32.0; a thrown grenade is one of them, and the stand-ins fill the rest), 10 gameplay events (hits and
+    /// kills) a second over the judged window (the last 10 minutes of a 12-minute recording, each 300 s half of it
+    /// too: B1.2) from its recording's aim (one command in 40 aims among the other teams) and its re-home of each
+    /// soldier every 60 s, no effect rows (the advisor's W1-U4D T-M1).
     pub fn estimate() -> Scene {
         Scene {
             id: 2,
             name: "ES",
             large_vehicles: 0,
+            fight_vehicles: 15,
             live_projectiles: 0,
             spawn_millihertz: 21_120,
             fuse_ms: [1_515, 1_515],
@@ -204,6 +220,7 @@ impl Scene {
             plates_from: 1,
             vehicles: 0,
             large_vehicles: 0,
+            fight_vehicles: 0,
             live_projectiles: 0,
             spawn_millihertz: 0,
             fuse_ms: [2_000, 2_000],
diff --git a/.gate/u4df-prefix/spikes/netcode-proof/crates/sim/src/world.rs b/.claude/worktrees/wf_9ce88fa3-452-1/spikes/netcode-proof/crates/sim/src/world.rs
index 80900f9..d3bddd8 100644
--- a/.gate/u4df-prefix/spikes/netcode-proof/crates/sim/src/world.rs
+++ b/.claude/worktrees/wf_9ce88fa3-452-1/spikes/netcode-proof/crates/sim/src/world.rs
@@ -466,6 +466,44 @@ fn part_box(c: [i32; 3]) -> ([f32; 3], [f32; 3]) {
     )
 }
 
+/// One command's move before collision: the move rule's one home (the advisor's U4d close ruling, B1.2 step 1), read
+/// by [`World`]'s commands and by the synthetic generator's dead reckoning (`Recording::synthetic`, B1.2 step 3, with
+/// `gain` 1). From the soldier's move speed (cm/s), the command's buttons and its move vector: crouching
+/// halves the speed; sprinting takes it × 3 / 2, only when not crouching and not holding fire; a vector longer than 127
+/// is scaled to 127. `gain` scales the speed (m/s) before the vector does: 1, but 1.01 under
+/// PM-0001-41/server-speed-1pct's bite (the authority only, bites builds). Returns the velocity (m/s, x and y; one
+/// tick's move is it times one tick), then whether the soldier crouches and whether it sprints (its record's flags).
+pub(crate) fn move_velocity(
+    move_speed: u32,
+    buttons: u16,
+    move_xy: [i8; 2],
+    gain: f32,
+) -> ([f32; 2], bool, bool) {
+    let crouch = buttons & Command::CROUCH != 0;
+    let fire_held = buttons & Command::FIRE != 0;
+    let sprint = buttons & Command::SPRINT != 0 && !crouch && !fire_held;
+    let speed_cm = if sprint {
+        move_speed.saturating_mul(3) / 2
+    } else if crouch {
+        move_speed / 2
+    } else {
+        move_speed
+    };
+    let speed = speed_cm as f32 / 100.0 * gain;
+    let (mx, my) = (i32::from(move_xy[0]), i32::from(move_xy[1]));
+    let len2 = mx * mx + my * my;
+    let scale = if len2 > 127 * 127 {
+        127.0 / (len2 as f32).sqrt()
+    } else {
+        1.0
+    };
+    let v = [
+        mx as f32 / 127.0 * scale * speed,
+        my as f32 / 127.0 * scale * speed,
+    ];
+    (v, crouch, sprint)
+}
+
 /// The parts of one scene's world that `records` names, for [`compose_volumes`]: how many soldiers, hulls and parts.
 #[derive(Clone, Copy)]
 struct Counts {
@@ -754,11 +792,14 @@ impl World {
                 });
             }
             let period = ticks_for_ms(60_000 + 5_000 * u32::from(v));
-            // In the fight (the advisor's W1-U4D N-M4; §4.13, F113): the very large units loop round the soldiers'
-            // cluster's centre, 5 to 14 m out, so their parts (within 3.71 m of the hull) stay inside the cluster and
-            // its reach; the others loop elsewhere, 52 m out and beyond, past the buildings.
-            let radius = if v < scene.large_vehicles {
-                5.0 + 3.0 * f32::from(v)
+            // In the fight (the advisor's W1-U4D N-M4 and U4d close ruling B1.3; §4.13, F113): the scene's first
+            // `fight_vehicles` hulls loop round the soldiers' cluster's centre, 5 to 14 m out (5 + 9 v / (n - 1) m for
+            // n of them: SS's 4 very large units at 5, 8, 11 and 14 m, ES's 15 every 9 / 14 m), so their parts (within
+            // 3.71 m of the hull) stay inside the cluster and its reach; the others loop elsewhere, 40 + 3 v m out
+            // (SS's from 52 m), past the buildings.
+            let fight = scene.fight_vehicles;
+            let radius = if v < fight {
+                5.0 + 9.0 * f32::from(v) / f32::from(fight.saturating_sub(1).max(1))
             } else {
                 40.0 + 3.0 * f32::from(v)
             };
@@ -1290,36 +1331,17 @@ impl World {
             return false;
         }
         // The move: one tick at the soldier's speed, the vector widened before any arithmetic (carry 15).
-        let crouch = c.buttons & Command::CROUCH != 0;
         let fire_held = c.buttons & Command::FIRE != 0;
-        let sprint = c.buttons & Command::SPRINT != 0 && !crouch && !fire_held;
-        let speed_cm = if sprint {
-            s.move_speed.saturating_mul(3) / 2
-        } else if crouch {
-            s.move_speed / 2
-        } else {
-            s.move_speed
-        };
         #[allow(
             unused_mut,
             reason = "PM-0001-41/server-speed-1pct scales it in bites builds"
         )]
-        let mut speed = speed_cm as f32 / 100.0;
+        let mut gain = 1.0;
         #[cfg(feature = "bites")]
         if authority && self.bitten(Bite::Speed) {
-            speed *= 1.01;
+            gain = 1.01;
         }
-        let (mx, my) = (i32::from(c.move_xy[0]), i32::from(c.move_xy[1]));
-        let len2 = mx * mx + my * my;
-        let scale = if len2 > 127 * 127 {
-            127.0 / (len2 as f32).sqrt()
-        } else {
-            1.0
-        };
-        let v = [
-            mx as f32 / 127.0 * scale * speed,
-            my as f32 / 127.0 * scale * speed,
-        ];
+        let (v, crouch, sprint) = move_velocity(s.move_speed, c.buttons, c.move_xy, gain);
         let (pos, blocked) = self.collide(s.pos, [v[0] * DT, v[1] * DT]);
         let vel = [
             if blocked[0] { 0.0 } else { v[0] },
@@ -2141,11 +2163,110 @@ impl SimCore for World {
 
 #[cfg(test)]
 mod tests {
-    //! The tests that read `World`'s private parts: the blend's boxes (T-M3), the movers' sizes (R-M1) and the clamp's
-    //! bound (TUNE-2).
+    //! The tests that read `World`'s private parts: the blend's boxes (T-M3), the movers' sizes (R-M1), the clamp's
+    //! bound (TUNE-2), and T2-L1's two invariants (hull ∘ part's frame, and phase 3's boxes after the moves).
 
     use super::*;
     use crate::replay::Recording;
+    use crate::seam::{AttributeChange, Placement};
+
+    #[test]
+    fn part_center_turns_a_parts_offset_by_its_hulls_yaw() {
+        // T2-L1 (the test review's round 2; the advisor's U4d close ruling, B1.2 step 0): hull ∘ part's frame
+        // convention on its own, not only through D-7's pin. At hull yaw 90 degrees a part 120 cm along the hull's own
+        // +x sits 120 cm to the world's +y of the hull, and one 120 cm along its +y sits 120 cm to the world's -x, 100
+        // cm up in both, within 1 cm: x' = x cos - y sin, y' = x sin + y cos (the old place_vehicles formula). A
+        // rotation by -yaw (plant O1) or none (O2) fails it.
+        let wire = Config::default();
+        let (hx, hy) = (1_000, -500);
+        let hull = [hx, hy, 0, wire.to_grid(YAW, FRAC_PI_2)];
+        for (offset, want) in [
+            ([120, 0, 100], [hx, hy + 120, 100]),
+            ([0, 120, 100], [hx - 120, hy, 100]),
+        ] {
+            let got = part_center(hull, offset);
+            assert!(
+                (0..3).all(|k| (got[k] - want[k]).abs() <= 1),
+                "offset {offset:?} at yaw 90 degrees: {got:?}, not {want:?}"
+            );
+        }
+    }
+
+    #[test]
+    fn a_shot_with_no_rewind_meets_a_soldier_where_this_ticks_move_put_it() {
+        // T2-L1 (the advisor's U4d close ruling, B1.2 step 0): phase 3 refreshes the soldiers' records after the moves,
+        // so a shot with no Rewind meets each soldier where this tick's command moved it. In TC, one tick: the shooter
+        // (slot 0) and the target (slot 1, 5 m north of it) are teleported into the open, the target's move speed set
+        // to the most the attribute holds (100 m/s), and its command runs it 3.33 m east, out of its old box (80 cm
+        // wide); the shooter, holding fire with no Rewind, hits it at its new chest and misses at its old one. Phase 3
+        // without the refresh (plant O3: the soldiers' records from before this tick's teleports and moves) misses it
+        // at its new chest.
+        let scene = Scene::targets();
+        let (shooter, target) = ([-6_000, -1_000, 0], [-6_000, -500, 0]);
+        let shoot = |aim: [i32; 3]| {
+            let mut world = World::new(&scene, 1, 0);
+            let history = HitHistory::for_ms(1_000).expect("1 s of ticks");
+            let tick = world.next_tick();
+            let command = |slot: u8, buttons: u16, move_xy: [i8; 2], aim: [i32; 3]| {
+                let c = Command {
+                    slot,
+                    tick,
+                    seq: 1,
+                    buttons,
+                    presses: [0; 4],
+                    move_xy,
+                    aim,
+                    view_tick: tick,
+                    view_fraction: 0,
+                };
+                (c, None)
+            };
+            let input = TickInput {
+                tick,
+                commands: vec![
+                    command(0, Command::FIRE, [0, 0], aim),
+                    command(1, 0, [127, 0], [-6_000, 2_000, 140]),
+                ],
+                teleports: vec![
+                    Placement {
+                        slot: 0,
+                        position: shooter,
+                    },
+                    Placement {
+                        slot: 1,
+                        position: target,
+                    },
+                ],
+                attributes: vec![AttributeChange {
+                    slot: 1,
+                    attribute: Attribute::MoveSpeed,
+                    value: MOVE_SPEED_MAX_CM_S,
+                }],
+                ..TickInput::default()
+            };
+            let events = world.step(&input, &history);
+            let hit = events
+                .iter()
+                .any(|e| matches!(e, SimEvent::Hit { victim: 1, .. }));
+            let shots = events
+                .iter()
+                .filter(|e| matches!(e, SimEvent::Shot { slot: 0, .. }))
+                .count();
+            (hit, shots, world.soldiers[1].pos)
+        };
+        let old = [target[0], target[1], 140];
+        let (hit_old, shots, at) = shoot(old);
+        assert_eq!(shots, 1, "the shooter fires once");
+        assert!(
+            at[0] - cm(target[0]) > 3.3 && (at[1] - cm(target[1])).abs() < 0.01,
+            "the target ran 3.33 m east: {at:?}"
+        );
+        let new = [libm::roundf(at[0] * 100.0) as i32, target[1], 140];
+        let (hit_new, shots, _) = shoot(new);
+        assert_eq!(shots, 1, "the shooter fires once");
+        assert!(hit_new, "a shot at the target's new chest {new:?} hits it");
+        assert!(!hit_old, "a shot at its old chest {old:?} misses it");
+    }
 
     /// A box's centre and half extents as the reference reads them, m, in f64 from its integer centimetres.
     fn metres(b: &VolumeBox) -> ([f64; 3], [f64; 3]) {
````

### npt-sim's tests (crates/sim/tests)
_`git diff --no-index .gate/u4df-prefix/spikes/netcode-proof/crates/sim/tests .claude/worktrees/wf_9ce88fa3-452-1/spikes/netcode-proof/crates/sim/tests`:  2 files changed, 229 insertions(+), 67 deletions(-)._

````diff
diff --git a/.gate/u4df-prefix/spikes/netcode-proof/crates/sim/tests/replay.rs b/.claude/worktrees/wf_9ce88fa3-452-1/spikes/netcode-proof/crates/sim/tests/replay.rs
index ca3beb6..6538019 100644
--- a/.gate/u4df-prefix/spikes/netcode-proof/crates/sim/tests/replay.rs
+++ b/.claude/worktrees/wf_9ce88fa3-452-1/spikes/netcode-proof/crates/sim/tests/replay.rs
@@ -319,7 +319,7 @@ fn d7_the_hash_and_dump_vectors_are_pinned() {
     // here for this build; their cross-target half (x86-64 against wasm32) is U4d2's.
     let pinned: [(Scene, usize, u64); 4] = [
         (Scene::stress(), 6_488, 6_786_077_935_583_382_297),
-        (Scene::estimate(), 1_944, 8_780_045_697_152_956_090),
+        (Scene::estimate(), 1_944, 8_237_017_275_248_454_161),
         (Scene::course(), 260, 12_984_630_011_248_297_851),
         (Scene::targets(), 340, 5_080_305_515_375_423_141),
     ];
@@ -353,7 +353,7 @@ fn d7_the_hash_and_dump_vectors_are_pinned() {
         [
             (1, 6_772, 10_412_709_864_748_183_085),
             (30, 6_628, 8_845_330_375_396_163_272),
-            (300, 6_612, 923_332_457_071_349_262),
+            (300, 6_612, 3_337_416_615_378_533_976),
         ]
     );
     // The dump's first names: the world's tick, its 64-bit tick count (two words), then soldier 0's fields.
@@ -369,7 +369,7 @@ fn d7_the_hash_and_dump_vectors_are_pinned() {
             "soldier[0].generation"
         ]
     );
-    // The three synthetic recordings' bytes (generator version 1): length and FNV-1a-64, for the recordings manifest
+    // The three synthetic recordings' bytes (generator version 2): length and FNV-1a-64, for the recordings manifest
     // (u4-build-plan.md §4.15: regenerated at check time, each file's digest committed).
     let digests: Vec<(usize, u64)> = recordings()
         .iter()
@@ -378,13 +378,13 @@ fn d7_the_hash_and_dump_vectors_are_pinned() {
             (b.len(), fnv(&b))
         })
         .collect();
-    assert_eq!(Recording::GENERATOR, 1);
+    assert_eq!(Recording::GENERATOR, 2);
     assert_eq!(
         digests,
         [
-            (16_697_066, 2_695_722_420_930_191_497),
-            (16_697_144, 7_136_420_146_508_984_890),
-            (798_759, 3_177_023_422_367_409_324),
+            (16_694_921, 35_974_458_070_191_737),
+            (16_700_849, 15_077_317_756_489_230_056),
+            (799_693, 9_126_256_229_328_065_459),
         ]
     );
 }
diff --git a/.gate/u4df-prefix/spikes/netcode-proof/crates/sim/tests/world.rs b/.claude/worktrees/wf_9ce88fa3-452-1/spikes/netcode-proof/crates/sim/tests/world.rs
index 4a63ea1..4d8a0ce 100644
--- a/.gate/u4df-prefix/spikes/netcode-proof/crates/sim/tests/world.rs
+++ b/.claude/worktrees/wf_9ce88fa3-452-1/spikes/netcode-proof/crates/sim/tests/world.rs
@@ -1001,71 +1001,233 @@ fn r_m1_a_record_of_an_entity_the_world_has_not_changes_nothing() {
     assert!(v.soldiers.iter().chain(&v.cover).all(|b| !b.live), "{v:?}");
 }
 
-#[test]
-fn t_m1_the_estimate_scene_meets_pm_0001_2s_es_bands() {
-    // The advisor's W1-U4D T-M1 (the test review's M1; PM-0001-2's ES checks, marks.json; ADR 0001:1474-1479): over
-    // ES's synthetic recording (seed 2, 9,000 ticks: tests/replay.rs's), every tick has 48 soldiers and 15 vehicles,
-    // and the run's means lie in PM-0001-2's bands exactly: 72 shots a second (68.4-75.6), 10 gameplay events (hits and
-    // kills) a second (9.5-10.5), 32 live projectiles a tick (30.4-33.6) and 21.12 spawns a second (20.064-22.176).
-    // Thrown grenades count toward both projectile bands; the stand-in spawner fills the rest, and every projectile
-    // fuses after 1.515 s. The events come from the recording's aim (one command in 11 among the other teams, the rest
-    // into the air) and from the blasts.
-    let scene = Scene::estimate();
-    let ticks = 9_000u32;
-    let seconds = u64::from(ticks / TICK_HZ);
-    let rec = Recording::synthetic(&scene, 2, 1_000_000, ticks, SPAN_MS);
-    let mut world = World::new(&scene, 2, 1_000_000);
-    let mut history = ring();
-    let (mut shots, mut events, mut live, mut spawns, mut thrown) = (0u64, 0u64, 0u64, 0u64, 0u64);
-    let (mut min_soldiers, mut min_vehicles) = (usize::MAX, usize::MAX);
-    let mut before = std::collections::BTreeSet::new();
-    for t in &rec.ticks {
-        let e = room_tick(&mut world, &mut history, t);
-        shots += e
-            .iter()
-            .filter(|e| matches!(e, SimEvent::Shot { .. }))
-            .count() as u64;
-        events += e
-            .iter()
-            .filter(|e| matches!(e, SimEvent::Hit { .. } | SimEvent::Kill { .. }))
-            .count() as u64;
-        let projectiles = records_of(&world, EntityKind::Projectile);
-        live += projectiles.len() as u64;
-        let now: std::collections::BTreeSet<(u16, u16)> = projectiles
-            .iter()
-            .map(|p| (p.index, p.generation))
-            .collect();
-        for p in &projectiles {
-            if !before.contains(&(p.index, p.generation)) {
-                spawns += 1;
-                thrown += u64::from(p.grid[7] == 1);
+/// One judged ES run (the advisor's U4d close ruling, B1.2 steps 4 and 6; PM-0001-2's ES checks, marks.json; ADR
+/// 0001:1474-1479): `seed`'s synthetic ES recording, 21,600 ticks (12 minutes) from tick 1,000,000, counted per 60 s.
+/// The window PM-0001-2's bands judge is its last 18,000 ticks (`ES_WINDOW`): 10 minutes, PM-0001-12's ES run length
+/// (ADR 0001:1600), after a 2-minute warm-up (PM-0001-63's, :1634).
+struct EsRun {
+    seed: u64,
+    /// Each minute's counts: shots, gameplay events (hits and kills), live projectiles summed over its ticks, spawns,
+    /// and the spawns that were thrown grenades.
+    minutes: Vec<[u64; 5]>,
+    /// The fewest soldiers and the fewest vehicles on any tick.
+    min_soldiers: usize,
+    min_vehicles: usize,
+    /// The farthest any hull came from the cluster's centre on any tick, cm: in D-6's form (whole centimetres, rounded
+    /// down), and exactly.
+    hull_out: i64,
+    hull_out_exact: f64,
+}
+
+/// An [`EsRun`]'s judged window, minutes 2 to 11 (ticks 3,600-21,599), and its two 300 s halves.
+const ES_WINDOW: std::ops::Range<usize> = 2..12;
+const ES_HALVES: [std::ops::Range<usize>; 2] = [2..7, 7..12];
+
+/// Thousandths as a decimal.
+fn milli(n: u64) -> String {
+    format!("{}.{:03}", n / 1_000, n % 1_000)
+}
+
+impl EsRun {
+    fn new(seed: u64) -> EsRun {
+        let scene = Scene::estimate();
+        let minute = ticks_for_ms(60_000) as usize;
+        let rec = Recording::synthetic(&scene, seed, 1_000_000, 12 * minute as u32, SPAN_MS);
+        let mut world = World::new(&scene, seed, 1_000_000);
+        let mut history = ring();
+        let mut run = EsRun {
+            seed,
+            minutes: vec![[0; 5]; 12],
+            min_soldiers: usize::MAX,
+            min_vehicles: usize::MAX,
+            hull_out: 0,
+            hull_out_exact: 0.0,
+        };
+        let mut before = std::collections::BTreeSet::new();
+        let mut out2 = 0i64;
+        for (k, t) in rec.ticks.iter().enumerate() {
+            let e = room_tick(&mut world, &mut history, t);
+            let m = &mut run.minutes[k / minute];
+            m[0] += e
+                .iter()
+                .filter(|e| matches!(e, SimEvent::Shot { .. }))
+                .count() as u64;
+            m[1] += e
+                .iter()
+                .filter(|e| matches!(e, SimEvent::Hit { .. } | SimEvent::Kill { .. }))
+                .count() as u64;
+            let projectiles = records_of(&world, EntityKind::Projectile);
+            m[2] += projectiles.len() as u64;
+            let now: std::collections::BTreeSet<(u16, u16)> = projectiles
+                .iter()
+                .map(|p| (p.index, p.generation))
+                .collect();
+            for p in &projectiles {
+                if !before.contains(&(p.index, p.generation)) {
+                    m[3] += 1;
+                    m[4] += u64::from(p.grid[7] == 1);
+                }
+            }
+            before = now;
+            run.min_soldiers = run
+                .min_soldiers
+                .min(records_of(&world, EntityKind::Soldier).len());
+            let hulls = records_of(&world, EntityKind::Vehicle);
+            run.min_vehicles = run.min_vehicles.min(hulls.len());
+            for v in &hulls {
+                let (x, y) = (i64::from(v.grid[0]), i64::from(v.grid[1]));
+                out2 = out2.max(x * x + y * y);
             }
         }
-        before = now;
-        min_soldiers = min_soldiers.min(records_of(&world, EntityKind::Soldier).len());
-        min_vehicles = min_vehicles.min(records_of(&world, EntityKind::Vehicle).len());
+        // The farthest squared distance, then its root: rounding down is monotonic, so this is D-6's form's maximum.
+        run.hull_out_exact = (out2 as f64).sqrt();
+        run.hull_out = run.hull_out_exact as i64;
+        run
+    }
+
+    /// Minutes `r`'s means, thousandths: shots, gameplay events and spawns a second, live projectiles a tick.
+    fn rates(&self, r: std::ops::Range<usize>) -> [u64; 4] {
+        let secs = r.len() as u64 * 60;
+        let mut s = [0u64; 5];
+        for m in &self.minutes[r] {
+            for (a, x) in s.iter_mut().zip(m) {
+                *a += x;
+            }
+        }
+        [
+            s[0] * 1_000 / secs,
+            s[1] * 1_000 / secs,
+            s[2] * 1_000 / (secs * u64::from(TICK_HZ)),
+            s[3] * 1_000 / secs,
+        ]
+    }
+
+    /// Minutes `r`'s four means as the report gives them.
+    fn line(&self, r: std::ops::Range<usize>) -> String {
+        let thrown: u64 = self.minutes[r.clone()].iter().map(|m| m[4]).sum();
+        let [shots, events, live, spawns] = self.rates(r);
+        format!(
+            "shots {} a second, gameplay events {} a second, live projectiles {} a tick, spawns {} a second ({thrown} \
+             thrown)",
+            milli(shots),
+            milli(events),
+            milli(live),
+            milli(spawns)
+        )
     }
-    let per_s = |n: u64| n * 1_000 / seconds;
-    let (shots_m, events_m, spawns_m) = (per_s(shots), per_s(events), per_s(spawns));
-    let live_m = live * 1_000 / u64::from(ticks);
+
+    /// PM-0001-2's ES checks over the window, with the ruling's two scene facts (48 soldiers and 15 vehicles on every
+    /// tick; every hull within 1,400 cm of the cluster's centre, B1.3, in D-6's form: the hulls' records lie on the
+    /// 1 cm wire grid, so a hull on the 14 m loop reads up to 1,400.6 cm exactly) and its drift check (each 300 s
+    /// half's gameplay events in band too); it panics at the first it misses, naming it.
+    fn check(&self) {
+        let seed = self.seed;
+        assert!(
+            self.min_soldiers >= 48 && self.min_vehicles >= 15,
+            "seed {seed}: {} soldiers and {} vehicles on the fewest tick",
+            self.min_soldiers,
+            self.min_vehicles
+        );
+        assert!(
+            self.hull_out <= 1_400,
+            "seed {seed}, the hull bound: a hull {} cm from the cluster's centre ({:.1} cm exactly)",
+            self.hull_out,
+            self.hull_out_exact
+        );
+        let [shots, events, live, spawns] = self.rates(ES_WINDOW);
+        assert!(
+            (68_400..=75_600).contains(&shots),
+            "seed {seed}: shots {shots} thousandths a second over the window"
+        );
+        assert!(
+            (30_400..=33_600).contains(&live),
+            "seed {seed}: live projectiles {live} thousandths a tick over the window"
+        );
+        assert!(
+            (20_064..=22_176).contains(&spawns),
+            "seed {seed}: spawns {spawns} thousandths a second over the window"
+        );
+        let halves = ES_HALVES.map(|h| self.rates(h)[1]);
+        assert!(
+            [events, halves[0], halves[1]]
+                .iter()
+                .all(|e| (9_500..=10_500).contains(e)),
+            "seed {seed}, gameplay events and the drift check: {events} thousandths a second over the window, {} \
+             and {} over its 300 s halves (each in 9,500-10,500)",
+            halves[0],
+            halves[1]
+        );
+    }
+}
+
+#[test]
+fn t_m1_the_estimate_scene_meets_pm_0001_2s_es_bands() {
+    // The advisor's W1-U4D T-M1 (the test review's M1), judged as the U4d close ruling sets it (B1.2 steps 4 and 6;
+    // N2-M1, N2-M2): over seed 2's window (`EsRun`: the last 10 minutes of 12), every tick has 48 soldiers and 15
+    // vehicles and every hull lies within 1,400 cm of the cluster's centre (B1.3), and the window's means lie in
+    // PM-0001-2's bands exactly: 72 shots a second (68.4-75.6), 10 gameplay events (hits and kills) a second
+    // (9.5-10.5), 32 live projectiles a tick (30.4-33.6) and 21.12 spawns a second (20.064-22.176); against drift, each
+    // 300 s half's gameplay events lie in 9.5-10.5 too. Thrown grenades count toward both projectile bands; the
+    // stand-in spawner fills the rest, and every projectile fuses after 1.515 s. The events come from the recording's
+    // aim (one command in 40 among the other teams, the rest into the air) and from the blasts. Seeds 1 and 3: the
+    // ignored test below.
+    let run = EsRun::new(2);
+    let halves = ES_HALVES.map(|h| milli(run.rates(h)[1]));
     println!(
-        "T-M1: ES over {seconds} s: shots {shots_m}, gameplay events {events_m}, spawns {spawns_m} ({thrown} \
-         thrown) thousandths a second; live projectiles {live_m} thousandths a tick"
-    );
-    assert!(
-        min_soldiers >= 48 && min_vehicles >= 15,
-        "{min_soldiers} soldiers, {min_vehicles} vehicles"
-    );
-    assert!((68_400..=75_600).contains(&shots_m), "shots: {shots_m}");
-    assert!(
-        (9_500..=10_500).contains(&events_m),
-        "gameplay events: {events_m}"
+        "T-M1: ES seed 2 over the window (600 s after 120 s): {}; gameplay events {} and {} a second over its halves; \
+         hulls at most {:.1} cm out",
+        run.line(ES_WINDOW),
+        halves[0],
+        halves[1],
+        run.hull_out_exact
     );
-    assert!(
-        (30_400..=33_600).contains(&live_m),
-        "live projectiles: {live_m}"
-    );
-    assert!((20_064..=22_176).contains(&spawns_m), "spawns: {spawns_m}");
+    run.check();
+}
+
+#[test]
+#[ignore = "T-M1's three-seed form (U4d-F, B1.2 step 6), run once by name for the report: about 30 s"]
+fn t_m1_seeds_1_2_and_3_meet_the_es_bands_and_report_them() {
+    // B1.2 step 4's measurement as a test: seeds 1, 2 and 3, each judged by t_m1's checks (`EsRun::check`), and per
+    // seed the report's lines: the window's four means, its two halves, the twelve 60 s gameplay-event rates over all
+    // 12 minutes, T-M1's 300 s from tick 0 and X2's form (ticks 0-17,999; X2 gave 7.578 for seed 2 at 1 in 11, before
+    // U4d-F), those two reported, never used to set anything. Every seed is reported before any is judged.
+    let runs = [1, 2, 3].map(EsRun::new);
+    for run in &runs {
+        let seed = run.seed;
+        println!(
+            "ES seed {seed}, the window (600 s after 120 s): {}",
+            run.line(ES_WINDOW)
+        );
+        for (k, h) in ES_HALVES.into_iter().enumerate() {
+            println!(
+                "ES seed {seed}, half {} ({} s): {}",
+                k + 1,
+                h.len() * 60,
+                run.line(h)
+            );
+        }
+        let minutes: Vec<String> = (0..12).map(|m| milli(run.rates(m..m + 1)[1])).collect();
+        println!(
+            "ES seed {seed}, gameplay events a second per 60 s, minutes 0-11: {}",
+            minutes.join(", ")
+        );
+        println!(
+            "ES seed {seed}, T-M1's 300 s from tick 0: {}",
+            run.line(0..5)
+        );
+        println!(
+            "ES seed {seed}, X2's form (600 s from tick 0): {}",
+            run.line(0..10)
+        );
+        println!(
+            "ES seed {seed}, every tick: at least {} soldiers and {} vehicles; hulls at most {} cm out in D-6's form \
+             ({:.1} cm exactly)",
+            run.min_soldiers, run.min_vehicles, run.hull_out, run.hull_out_exact
+        );
+    }
+    for run in &runs {
+        run.check();
+    }
 }
 
 /// The course's one soldier spawned at `feet` (cm), and the fire origin a still command fires from, m.
````

## What it is judged against

### The approved plan: the advisor's U4d close ruling, B1.1 to B1.3 (advisor-u4d-close.md)
_`.checkpoints/netcode-proof-test/advisor-u4d-close.md` lines 23-181 (sha256 cbc7e1c9210b), as they are:_

````text
**B1.1 (A) the estimate scene's knobs: APPROVED as stand-in tuning (as TUNE-1 was), with conditions.**
- `grenade_fuse_ms` = 1,515 (scene.rs:110, read by the World): not new tuning in substance. It is ADR 0001:1478-1479's
  two ES rates by Little's law (32 live projectiles at 21.12 spawns a second give each a life of 32 / 21.12 =
  1.515 s), applied to
  thrown grenades so that every ES projectile has one fuse (`ticks_for_ms` rounds it up to 46 ticks, 1.533 s; T-M1
  measured 32.300 live, in band). It does change every bot-driven ES run's grenades (T2-L3), and that is intended: ES
  is R2's estimate, where a projectile lives about 1.5 s. Conditions:
  - scene.rs:109's doc says how the number follows (the engine-architect's wording point: ":1478" names a number that
    line does not state). Words: "A thrown grenade's fuse, ms: 2 s, and ES's 1.515 s (ADR 0001:1478-1479's 32 live
    projectiles at 21.12 spawns a second give each a life of 32 / 21.12 = 1.515 s, so every ES projectile fuses after
    it)." It rides U4d-F (B1.2).
  - U4r's contract-test ES run shows the live-projectile band under the bots' own throws (carry 59, B1.2).
- `aim_one_in` = 11 (scene.rs:80, :115; read only by `Recording::synthetic`, replay.rs:462; pub(crate); drawn only
  when above 1, so no other scene's draw stream changes): **the knob is approved; its value is not final.** 11 was set
  on T-M1's 300 s from tick 0, which N2-M1 shows is the wrong window. U4d-F sets it on the judged window (B1.2 step 5).
  If no whole 1-in-N puts every seed in band, the knob may become a finer share (for example in thousandths), keeping
  the rule that only ES draws for it. scene.rs:114 and :177's "10 gameplay events a second" then name the window it
  is measured over.

**B1.2 N2-M1's fix form (carry 52): the generator stays open-loop, and its dead reckoning follows every input the
generator itself emits. A bounded re-home is added only if the measurement still shows a drift. `aim_one_in` is set on
the judged window. The main session lands it as U4d-F, together with N2-M2's scene fix (B1.3), before U4d2 pins
anything.**

Why this form:
- **Closed loop (the generator steps a World to read the true positions): refused.** §4.15 makes the recordings
  open-loop commands that U4d2 regenerates from the manifest (u4-build-plan.md:493-494). A generator that ran the World
  would tie every recording's bytes to the World's float path: any World change would re-pin every recording, and a
  cross-target difference in the World would show up as a different recording, not as U4d2's first mismatching tick.
- **Re-home alone: not as the fix.** It masks the error and leaves the model wrong. It multiplies the recording's
  teleports (48 a minute against about 6 today), each an input that forces a correction, so it changes what the
  recording exercises. And a teleport puts the soldier on the exact home point, where the into-the-air shots meet
  overlapping boxes: an event source of its own, tied to the period.
- **Reckoning what the generator emits fixes the cause for the main terms** (my reading of replay.rs:402-432 against
  world.rs:1276-1320 and :1112-1121):
  - ES holds fire on every command (`always_fire`, replay.rs:368), so sprint never applies in ES (world.rs:1295);
  - crouch halves the speed on about one heading in 10 (replay.rs:425-429);
  - a scripted MoveSpeed of 400-800 cm/s replaces 600 for one slot every 311 ticks and stays (replay.rs:512-521);
  - one command in 64 is dropped while the estimate still moves (replay.rs:501-509).
  Crouch alone is a random walk of about 3.75 m per crouched heading (6 m/s × about 1.25 s × 0.5): tens of metres
  over 10 minutes. That is my estimate, not a measurement. Deaths reset the true position near home (a respawn at a
  team spawn point, world.rs:1712-1727), so they bound the error rather than grow it. Blocked moves (greybox solids,
  closed doors, the wall, vehicle parts: world.rs:1192-1239) the generator cannot see; the measurement decides whether
  they need the re-home.

**U4d-F, in order.** Each step runs the gate and npt-sim's suites. A pin changes only in the step that changes it, so
every re-pin has one named cause (T2-L1's lesson).
0. **T2-L1's two unit tests, first** (in world.rs's module). U4d-F re-pins D-7, which is the exact scenario T2-L1
   names, so its invariants get their own tests before any pin moves:
   - (a) `part_center` at hull yaw 90° maps offset (+120, 0, 100) cm to hull + (0, +120, 100) cm within 1 cm (the old
     place_vehicles formula's sign);
   - (b) in one step, a soldier that moved out of its old box is hit at its new place by a shot with no Rewind (or:
     phase 3's soldier boxes equal `hit_volumes_from` of the post-move records).
   The test-engineer's plants O1, O2 and O3 (u4d-round2-test.md [5]) each fail one of them by name. No pin changes.
1. **One home for the move rule.** A pub(crate) function in world.rs gives one command's per-tick move from the
   soldier's move speed, its buttons (crouch halves the speed; sprint × 3 / 2 only when not crouching and not holding
   fire: world.rs:1293-1301) and its move vector (the scale above 127). `run_command` and the generator both call it.
   This step is behaviour-neutral: every test and every D-7 pin stays unchanged.
2. **N2-M2: ES's vehicles** (B1.3). Only ES's D-7 pins change; SS's, CO's and TC's stay unchanged.
3. **The reckoning, in every scene** (one generator, one model). The estimate moves by step 1's function, with:
   - the generator's own buttons;
   - its own record of each slot's move speed: the scripted MoveSpeed it emits, clamped as `apply_inputs` clamps it,
     from the tick `apply_inputs` applies it, which is before that tick's commands (world.rs:1112-1121);
   - no move for a command it drops (a held-back command moves it once, when the World runs it);
   - teleports and spawns resetting it, as now.
   `Recording::GENERATOR` goes from 1 to 2 (replay.rs:285). D-7's recordings and after-tick states are re-pinned in
   every scene. D-6 (SS) and the CO and TC tests pass with their assertions unchanged.
4. **The measurement (the acceptance), before `aim_one_in` is set:**
   - **The recordings:** for seeds 1, 2 and 3, an ES recording of 21,600 ticks (12 minutes) from the fixed
     generator. The window is its last 18,000 ticks: 10 minutes, PM-0001-12's ES run length (ADR 0001:1600), after a
     2-minute warm-up (the shortest warm-up ADR 0001 states for a run: PM-0001-63, :1634). A stationary generator
     makes the warm-up's length immaterial; that is what this step shows.
   - **Over the window, each seed:** shots 68.4-75.6 a second, gameplay events 9.5-10.5 a second, live projectiles
     30.4-33.6, spawns 20.064-22.176 a second; 48 soldiers and 15 vehicles on every tick; every ES hull within
     1,400 cm of the centre (B1.3).
   - **Against drift:** each seed's gameplay-event rate over each 300 s half of the window is also in 9.5-10.5.
   - **Reported per seed:** the four rates over the window; the two halves; the twelve 60 s event rates over all 12
     minutes; T-M1's 300 s from tick 0 beside them (reported, never used to set anything); and X2's form (ticks
     0-17,999, seed 2) beside X2's 7.578.
   - **3b, only if the drift check fails after step 3:** the bounded re-home, in ES only. Each soldier is placed at its
     home by a recorded teleport once every 60 s (slot k on the ticks where e mod 1,800 = floor(k × 1,800 / 48)), and
     its estimate is reset with it; no new draws. Then measure again.
5. **`aim_one_in`** (or its finer form, B1.1) is set from step 4's three seeds, and step 4's checks pass at the value
   chosen.
6. **Tests.** t_m1 becomes the window test for seed 2 in the measured suite: 21,600 ticks, about 11 s by X2's 9 s for
   18,000. The three-seed form is an `#[ignore]` test, run once by name for the report. Plants, each failing by name:
   - the old reckoning restored (scene speed only) fails t_m1's drift check;
   - one ES hull at 40 m fails the hull bound.
7. **Docs:** scene.rs:77-79's measured numbers, :114 and :177 name the window; the grenade-fuse doc (B1.1);
   replay.rs:458-461's comment.
- **If no setting** in steps 4-5 puts every seed's window and both halves in band, U4d-F stops and brings the numbers
  to me. The 9.5 floor does not move (Mustafa, 2026-11-11): the scene is what changes, never the band.

**Who lands it, and how it is reviewed:**
- **Who:** the main session, itself or through one builder in a worktree from the spike head. crates/sim has no
  wave-2 owner (u4-build-plan.md, Lanes). It merges before U4d2's brief is final. U4d2 bases on that commit and pins
  nothing generated before it.
- **U4e** need not wait: it edits no crates/sim file and pins no recording. Any ES number in U4e's report comes from
  U4d-F's scene.
- **Its own review budget:** U4d-F is its own change, not a round 3 of U4d. netcode-architect and test-engineer
  (Opus 5.5, xhigh) review U4d-F's diff; rust-reviewer only if step 1 changes a World line beyond moving the speed rule
  into the function.
- **Evidence:** check.mjs's counted run with the new counts, the gate, and step 4's table. Risk tier 1 (spike code,
  inside PR #40). Nothing for Mustafa's eyes.
- **This block is U4d-F's approved plan,** for these files and symbols:
  - crates/sim/src/world.rs: the move function; ES's loop placement in `World::new`; T2-L1's two tests;
  - src/scene.rs: the in-fight hull knob, `ES_AIM_ONE_IN`, the docs;
  - src/replay.rs: `Recording::synthetic`, `Recording::GENERATOR`;
  - tests/world.rs: t_m1 and the ignored three-seed test;
  - tests/replay.rs: D-7's pins.
  Anything beyond them comes back to me first. The main session's checkpoint points to this block as the plan.
  SEC-U4d-R2-1's accessor does not ride U4d-F: Mustafa deferred it to U4e (carry 51).

**Carry 52 (u4s-carries.md, §U4d2): faithful to Mustafa's words.**
- His quote in the label is verbatim (history.md:284-285).
- The body says "fix" of the estimate scene and keeps the 9.5 floor, as he did. "Or hash" is the carry's own word, not
  his: it covers ES's hashes (N2-M2), and it is stricter than his words, never looser.
- Text change: the label lines (his words) stay as they are. In the body, replace the span from "(per 50 s window:" to
  the carry's closing quote with the block below (no word of his is in the span; rewrap within the entry):
```
(per 50 s window: 13.4, 11.0, 10.1, 9.4, 9.7, 8.7 a second at 1 in 10; the test-engineer's probe X2, seed 2 at 1 in
    11: 7.578 a second over 600 s). The fix and the re-calibration of `aim_one_in` are measured on the judged window (a
    10-minute measured window after warm-up, ADR 0001:1600; 3 seeds), not on T-M1's 300 s from tick 0. PM-0001-2's
    floor of 9.5 events a second does not move. The form, the measurement and the owner are the advisor's U4d close
    ruling (advisor-u4d-close.md, B1.2 and B1.3): the main session's change U4d-F lands the fix, with ES's vehicles
    inside the fight (N2-M2), before U4d2's brief is final. U4d2 pins only recordings written by U4d-F's
    `Recording::GENERATOR` or later, and ES's hashes only from U4d-F's scene; its report names U4d-F's commit."
```

**New carry for U4r, from N2-M1's other half (A2-L3 = T2-M1's carry):** the judged ES runs are U4r's, driven by
npt-bots (§5 U4r; marks.json's ES runs). No U4 check shows that the brawl meets ES's bands before PM-0001-12 counts,
and §4.12's contract test resolves PM-0001-2 on SS suites only. u4s-carries.md §U4r gains this entry (copy the
block's lines as they are):
```
59. **A2-L3 and T2-M1, ES under the bots (u4d-round2-arch.md, u4d-round2-test.md; the advisor's U4d close ruling):**
    "U4r's contract test also runs a short ES suite of the brawl `Behaviour` (48 clients) and resolves PM-0001-2's ES
    checks on it: shots, gameplay events, live projectiles and spawns in their bands (W1-U4D T-M1's numbers) and the
    share of snapshots whose client set holds all 47 other soldiers and all 15 vehicles. A band or share missed goes
    to the advisor before any ES run of PM-0001-12 counts. The ES runs themselves stay under PM-0001-2's counter
    script, run by run (ADR 0001:1480)."
```

**B1.3 N2-M2, ES's vehicles: fix the scene now, in U4d-F step 2, before any ES pin.**
- **The rule:** ES's 15 vehicles loop inside the fight, every hull within 1,400 cm of the cluster's centre on every
  tick (the bound D-6 holds SS's very large units to). A pub(crate) scene knob counts the hulls that loop in the
  fight (SS 4, its very large units; ES 15). SS's placement stays as it is (scene.rs:183 and world.rs:760-764 today
  put all of ES's at 40 + 3v m).
- **Why not "U4e's report shows its region covers them":**
  - The region's size is not set yet: neither plan fixes it, and ADR 0001 leaves interest's values as data set from
    R2-INT-REC1's marks when interest management is built (R2-INT-OQ7, :1957).
  - ES's premise is every vehicle in every client's set on at least 99 % of snapshots (ADR 0001:1475-1477). If it
    held only under one region setting, PM-0001-12's ES runs would depend on a tuning value.
  - With the hulls inside 14 m, no hull is much farther from any soldier than the soldiers are from each other (homes
    10 m out and 17.3 m apart; each soldier wanders about 8 m). So a region that holds all 47 other soldiers holds the
    vehicles too.
- **Order:** before `aim_one_in` is set. In the fight the hulls are cover (a part hit is no gameplay event:
  world.rs:1572-1590) and they block moves (world.rs:1226-1235), and both move the event rate.
- **The share** stays PM-0001-2's check on every ES run, and U4r's contract test shows it (carry 59).

````

### Round 2's netcode findings U4d-F answers (N2-M1, N2-M2)
_`.checkpoints/netcode-proof-test/u4d-round2-netcode.md` lines 1-90 (sha256 80fa6739d84c), as they are:_

````text
# U4d round 2 + W1-S1 (npt-sim fix pass + npt-proto half): netcode-architect (read only)

Copy: /home/user/Shamble-Frontlines/.gate/u4d-s1 (detached 21a8597, uncommitted). Round 1 state: .gate/u4d-prefix.
Rulings: advisor-u4-wave1.md W1-S1 (:45-111), W1-U4D (:113-304), W1-PUB (:353-436).

## Plan
1. Read fixer's record (u4d-build.md "Fix pass, round 1") and s1-proto.md.
2. diff -ru u4d-prefix sim vs u4d-s1 sim; git diff 21a8597 -- spikes/netcode-proof/crates/proto.
3. Check H1 (parts relative to hull; one compose function; offsets fit; tests), N-M1 (magazine), N-M3 (FLAG_FIRED),
   N-M4 (in the fight), N-M5 (effect end tick), N-M2 code half (Refusal/RefusalKind/refusals per command, cleared each
   step, outside hash and dump; D-10 asserts).
4. W1-S1: owner-only kind fields + ranges, one per soldier, SOLDIER = catalogue remote row; proto GROUPS own-state rows,
   section rule, enforcement in encoder+decoder, tests (3 checks), -57 bite.
5. Fixer's points A, B, C; downstream U4e/U4g/U4h impact (own-state record size/churn).
6. Findings with file:line, verdict.

## Progress
- [1] Read u4d-build.md "Fix pass, round 1" (:135-374) and s1-proto.md whole. Sim diff saved (scratchpad simfix.diff,
  3,103 lines); read src diff (history, lib, replay, scene, seam, world). Open checks: part damage at health 0 (churn
  over an S30?), cluster centre = origin?, FLAG_DEAD vs alive, throw authority-only, wait's range, phase-1 records.
  (My slip: line [1] was first appended by a shell heredoc to this absolute path, not a file tool; notes only. The
  Edit tool is not available to me here, so this file is rewritten whole by Write after each block.)
- [2] Read sim tests diff (D-3 catch-up, h1 x2, n_m5, a_m1, r_m1, t_m1, w1_s1, D-5/D-10 refusals, carry 9/14, D-6
  in-fight, D-8 bound) and world.rs run_movers/run_command/throw/refuse. Checked: part health wraps to 200 (never
  dies: churn steady over an S30, world.rs:1579); cluster centre = origin (homes 10 m out, scene.rs:136); FLAG_DEAD
  set iff dead (world.rs:1612, revive :1142); throw and refuse authority-only (:1381-1418); wait in [1-U, cost]
  (:1371, :1401); phase 1 writes records then load_movers_into (:1169-1171); refusals cleared per step (:2080).
- [3] Proto diff read (snapshot.rs Section/carries, put/get_delta, put/get_record, both GROUPS rows, the 3-check
  test; wire.rs w1_s1 28/14; bites -57 own_state; common section_kinds). Sound. Update codes changed fields at full
  width (snapshot.rs:289-312). Plan text §4.2/§4.10 already carries W1-U4D's words (u4-build-plan.md:369).
- [4] (B) weighed: marks' default window is `measured` (marks/README.md:100); PM-0001-12 judges "ES at P2 for 10
  minutes" (ADR 0001:1600) with pre:PM-0001-2; the fixer's 50 s windows (13.4 ... 8.7 at 1 in 10) put a measured
  10-min window after warm-up near 8.8-9.3/s at 1 in 11 < 9.5. Generator (replay.rs:402-432) reckons scene speed only,
  no crouch, no dropped command, no scripted speed, no death; est reset only by its own teleports/spawns (~ every
  8.4 min per slot, replay.rs:522-530). Also ES "all 15 vehicles in view" (ADR 0001:1475-1477) vs ES's vehicles: ES
  sets large_vehicles 0 (scene.rs:183), so all 15 loop at 40 + 3v m = 40-82 m (world.rs:760-764). Own-state size vs
  R2's 16 B own-state assumption (r2-netcode.md:1756): ~10-14 B delta; fine.

## Round-1 findings (mine): CLOSED / NOT CLOSED
- H1 CLOSED: PART = offset x, y, z, yaw relative to hull + health + parent (seam.rs PART doc; world.rs write_records
  part block, Val::F(cm(offset)), yaw 0); one composition part_center (world.rs:446) used by compose_volumes (:483,
  phases 3 and 8, hit_volumes_from :882, load_movers) and load_movers_into (:2031); offsets asserted in World::new;
  tests h1 x2 (tests/world.rs:~760, :800): 2,850 part records changed = 2,850 parts hit over 600 SS ticks (4.75 a tick,
  was 123.5); Q6 plant fails both.
- N-M1 CLOSED: SS magazine 32,000 (scene.rs:63-70), const assert both scenes (:167-170), D-6 lines; Q25 fails build.
- N-M3 CLOSED: FLAG_FIRED gone, flags dense (world.rs:116-120); D-3 catch-up every 9th step, >40 such ticks; Q1 fails.
- N-M4 CLOSED for SS (the ruling's scope): large hulls 5-14 m round the centre, others 52-82 m, stand-ins ±12 m
  (world.rs:760-764, spawn_stand_in); D-6 asserts; Q4, Q5. U4r carry present (u4s-carries.md:193). ES: see N2-M2.
- N-M5 CLOSED: effect record = kind, end tick (TICK), magnitude; n_m5 test; 7.74 of 384 a tick; Q7.
- N-M2 code half CLOSED: Refusal/RefusalKind (seam.rs:498-520), InputReports::refusals (:531), refuse() authority
  only, cleared each step (world.rs:2080), write_state excludes reports; D-5/D-10 assert records; Q3, Q13, Q14, Q28,
  Q29. Module doc words = §4.2's replacement text.
- My round-1 Lows: ES events source folded into T-M1 (closed for its 300 s window; see N2-M1); 1 MB = 10^6 (T-M2)
  CLOSED; clamp on non-firing commands, PARENT redundant, held-back Rewind leaving the ring, non-finite ray (carry 45):
  NOT CLOSED, named follow-ups (u4d-build.md:367-372); carry 13 counting half: to U5 (C13).

## W1-S1
- Sim half MET: OwnState last in ALL (seam.rs:204, :297), OWN_STATE 8 fields (:252) with ranges holding every
  reachable value (wait in [1-U, interval x TICK_HZ]; reload <= RELOAD_MAX; attributes brought into range on entry;
  scene starts asserted; record() debug_assert; w1_s1 test :1810); one record per slot, generation the soldier's
  (world.rs:1767); SOLDIER = 8 fields = catalogue :137 remote row (seam.rs:249); reloading stays a flag; FIELDS 12.
- Proto half MET: OwnState in both own-state rows (snapshot.rs:492, :648); carries() (:175) = owner rows for Own,
  other rows for Entities; encoder OutOfRange before writing, decoder Malformed at the kind (:222-:245, :266-:340);
  3-check test (:726: 92-bit row, 129 bits on the wire, P7 shows 48 bits); w1_s1 wire test 28 encodes + 14 decodes
  refused (tests/wire.rs:188); -57 sends OwnState (tests/bites.rs:104); P1-P8.
- Together: OwnState and Effect can no longer travel in the entity section or inside a remote soldier record. Which
  slot's own-state records go to client c is the room's (U4e) selection, which the codec cannot see: N2-L1.

## New findings
- N2-M1 MEDIUM (fixer's point B): ES's event rate falls over a run; T-M1 (tests/world.rs:1005-1014) measures 300 s
  from tick 0, not the judged window (PM-0001-12's 10-min ES runs, measured after warm-up). Measured 10-min mean likely
  ~8.5-9.3 < 9.5: PM-0001-2 fails on ES, PM-0001-12's ES part INVALID. Same drift may spread SS's cluster over an S30
  (everyone-in-view, the precondition of -17/-18/-20/-21): unmeasured. Not before merge; fix before U4d2 pins the
  recordings; report wording now.
- N2-M2 MEDIUM: ES's 15 vehicles loop 40-82 m out (scene.rs:183 large_vehicles 0 -> world.rs:760-764 else branch)
  against ES's "all 15 vehicles in every client's set on >= 99 %" (ADR 0001:1475-1477; SERIES.md:314). Pre-existing
  (round 1: 40 + 3v m for all), missed in my round 1, untouched by the fix. Not before merge; before U4d2 pins ES.
- N2-L1 LOW: owner routing is the room's; U4e carry with a test (own section names only c's slot; effects index/8).
- N2-L2 LOW: ES sky aim (replay.rs:458-466) keeps pitch (and a standing soldier's yaw) still on most ticks vs R2's
  "yaw 16, pitch half the time" (r2-netcode.md:1751): ES's downstream slightly lighter than the estimate it stands for.
- N2-L3 LOW: downstream notes: U4g reset lacks respawn_left (world.rs:274; predict ignores it: :1289); U4h must blend
  composed boxes or interpolate hull yaw wrap-aware (:1185); U4e: own-state record 160 bits, changes every tick in SS.
- N2-L4 LOW: text: "(9 fields)" slip (8 is right); Weapon, Scene::weapon, Scene::health into §4.1/W1-PUB's list.
- (A): grenade_fuse_ms 1,515 = ADR 0001:1478's ES fuse applied to grenades (SS/CO/TC 2,000): not new tuning in
  substance. aim_one_in = 11: new stand-in tuning; acceptable as a knob, value to be re-calibrated with N2-M1.

## Verdict
APPROVE WITH FINDINGS: no Critical or High; every round-1 finding of mine the fix claims is closed; W1-S1 both halves
met; two Mediums and four Lows as follow-ups with carries (no round 3).
DONE
````

### Round 2's test findings U4d-F answers (T2-M1, T2-L1 and its plants O1 to O3, probe X2)
_`.checkpoints/netcode-proof-test/u4d-round2-test.md` lines 1-197 (sha256 daf84f53c470), as they are:_

````text
# U4d round 2 (combined with W1-S1's npt-proto half): test-engineer

Copy: /home/user/Shamble-Frontlines/.gate/u4d-s1 (detached at 21a8597; nothing committed). Round 1 (builder's state):
.gate/u4d-prefix. Fix record: u4d-build.md "Fix pass, round 1"; proto half: s1-proto.md. Rulings: advisor-u4-wave1.md
W1-S1 :45-111, W1-U4D :113-304, W1-PUB :353-436. Round 1 mine: u4d-review-test.md (M1-M5, L1-L6, N1-N3).
Other round 2 notes (read-only): u4d-round2-{netcode,security,rust,arch}.md. Memory: none (no agent-memory folder).
Rules: CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target, CARGO_NET_OFFLINE=true; `cd <copy> || exit 1`
alone on its own line in a subshell; never background a chain with cd; idle wait (pgrep cargo/rustc) + touch all crates
files + 2 s toucher inside the same command for counted runs; provenance pre-run (Compiling npt-sim / npt-proto with
this copy's path); df > 2 GB; plants one at a time, restore, sha256 check; commit nothing.
Expected counts: measured 351, bites 388, self 52 of 52.
DEVIATION (stated): this run has no Edit tool (the harness refused it: "No such tool available: Edit"), as in round 1.
So: notes are rewritten whole with Write; plants are applied by a scratch script (scratchpad r2t-plant.py) that
replaces exactly one unique match per site (asserts count == 1), saves a byte copy first, restores from it and checks
sha256 against r2t-ref.sha256 after each plant. Same method as round 1's u4d-test-bites.py. Runner: scratchpad
r2t-run.sh (idle 8 s, df, touch, 2 s toucher inside the script, output to stdout only). Plants Q1, Q10a2, Q12 and X3
were added to the scratch script by small python edits of that scratchpad file (outside the repo).
MY SLIP: block [4] below was first appended to this file by a shell heredoc (`cat >> ... <<EOF`), not by Write; notes
only, no other file; this whole file was then rewritten with Write.

## Plan
1. sha256 of every file in crates/sim and crates/proto; compare with u4d-build.md's list and s1-ref.sha256.
2. Read the diff (prefix vs copy for sim; 21a8597 vs copy for proto) and the other round 2 notes.
3. Round-1 findings M1-M5, L1-L6, N1-N3: CLOSED / NOT CLOSED with proof (test + plant).
4. Re-plant (one at a time, named test fails, restore, sha256): Q2, Q6, Q11a, Q13, Q19; P1, P3, P7; plus my own
   plants where a test's power is in doubt (O1 hull-part rotation sign, O2 rotation ignored, O3 phase 3 stale soldier
   records, O4 decode cap undone, O5 carry 13's derived clone_from); probes X1 (H1 count over 54,000 SS ticks), X2 (ES
   bands over 18,000 ticks = 10 min).
5. Carries 9, 12, 13, 14, 36 against code and tests.
6. The fixer's open points A (ES knobs), B (ES drift), C (SOLDIER 8 vs 9; new public items).
7. Settle by test/plant any other reviewer's finding a run can settle.
8. check.mjs once at the end (fresh touch, idle, 2 s toucher): every step's counts.
9. Final sha256 = step 1's.

## Progress
- [1] sha256 of all 34 files under crates/sim (17) and crates/proto (17): scratchpad r2t-ref.sha256. crates/sim: 19 of
  20 f-ref.sha256 entries OK (u4d-build.md :305-315's list plus the unchanged files and workspace files); crates/sim
  also `diff -r` identical to U4d's worktree wf_f54246ff-e0d-1. The one mismatch is check.mjs (f-ref 6d1ed188, here
  7eddc285), which equals 21a8597's (U4a's merged check.mjs; `git diff 21a8597` clean): expected. crates/proto: all 7
  s1-ref.sha256 entries OK (snapshot 48ca1183, error f52dbb94, common b23b687b, wire 69ef15ec, bites 96d07cc9, sim
  seam 0c41b406, quant 0d41e906). Workspace Cargo.toml 75b6c160 and Cargo.lock 3da1e15f = 21a8597. git status: 10 M,
  9 ??. df 11 GB free.
- [2] Read: sim diff prefix -> copy (scratchpad r2t-sim.diff, 3,103 lines: history, lib, replay, scene, seam, world,
  tests replay/seam/world), proto src + tests diff vs 21a8597 (r2t-proto-src.diff, r2t-proto-tests.diff), the four
  other round-2 notes (netcode, security, rust, arch: all APPROVE WITH FINDINGS, no Critical/High). Doubts to settle
  by plant: (a) hull o part composition has no semantic test (h1 checks "box moved", a_m1 compares the function with
  itself): O1, O2; (b) phase 3's present volumes now come from refreshed soldier records: which test sees a stale
  refresh? O3; (c) rust L1 = my L5 (decode cap) has no test: O4; (d) h1's "changes only when health does" is measured
  over 600 ticks only, and its comparison includes the record's area (not on the wire: proto between() reads grid
  only, snapshot.rs:86-115): X1 over an S30; (e) fixer's point B: X2 over ES's 10-minute run length.
- [3] Provenance pre-run 11:36:36-11:36:40 UTC: `cargo test --locked --no-run -p npt-sim -p npt-proto` printed
  "Compiling npt-sim (.gate/u4d-s1/.../crates/sim)" and "Compiling npt-proto (.gate/u4d-s1/.../crates/proto)"; idle,
  11 GB free.
- [4] Re-plants (r2t-plant.py, one at a time, the named test run alone from this copy, restored, `check` ALL-OK (34
  files) after each):
  - Q2 (predict leaves the own-state record stale): d3 FAILED at tests/world.rs:246 "own-state record" (client grid
    [0,0,0,30,0,...] vs authority [600,0,0,30,-999999,...]: velocity and fire timer stale). = fixer.
  - Q6 (part x = hull's world x + offset): both h1 tests FAILED (:779 "tick 1: the parts' records"; :837). = fixer.
  - Q11a (a throw takes no spawn): t_m1 FAILED at :1064, live 37,726 thousandths (spawns 24,660). = fixer.
  - Q13 (refusals not cleared per step): d10 cooldown, ammunition, reload, rewind FAILED (:1499, :1521, :1547,
    :1637); clamp passed (it has no refusals). = fixer.
  - Q19 (cap back to 2^20): the replayer test FAILED "a span above 1 MB: Done" (tests/replay.rs:577); carry14 FAILED
    "one ms past the edge" (:1964), run separately (cargo stops at the first failing binary). = fixer.
  - P1 (get_delta's check off): w1_s1 FAILED "Vehicle moved out of the entity section" (tests/wire.rs:297). = s1.
  - P3 (put_delta's check off): w1_s1 FAILED "Vehicle own true: Add(...)" (tests/wire.rs:225). = s1.
  - P7 (SOLDIER [Field; 11] + 3 VELOCITY): the GROUPS unit test FAILED at snapshot.rs:747, widths [16,16,12,16,10,8,
    10,4,16,16,16] vs the row's 8. = s1. (npt-sim, npt-link, npt-proto recompiled from this copy.)
  - Q12 (my round-1 E6: blend by / 255), 11:52:45: the blend unit test FAILED at world.rs:2173 "the soldier, fraction
    1, axis 0: [-127.396, -126.596] against the reference centre -127 ± 0.4" (3.9 mm); both D-9 tests still pass
    (the hit-or-miss check is the second test, as ruled). = fixer.
  - Q1 (my M4 = N-M3, P11's form: a fired flag bit 1 << 4 set on a shot, cleared per tick in step, per command in
    predict; 3 sites), 12:00:39: d3 FAILED at :245 "CO step 4: record" (flags 0 vs 16; step 4 = the first catch-up
    tick). = fixer.
  - Q10a2 (my M5, probe C's form: load_movers grows doors_closed to a decoded index), 12:01:01: the module test FAILED
    at world.rs:2322 "CO: no mover grew"; r_m1 passes (a grown door vector changes no volume or state). = fixer's Q10a'.
- [5] My own plants (whole npt-sim measured suite, --no-fail-fast; restored, ALL-OK after each):
  - O1 (part_center rotates by -yaw: signs of the s terms flipped, world.rs:452-453), 11:39:46-11:40:34: ONLY d7
    FAILED, at tests/replay.rs:351 (SS's state after 1/30/300 ticks pin). lib 3, seam 8, world 30 pass: h1 x2, a_m1,
    r_m1, D-1, D-3, D-6 all pass. D-2 (in replay) passes too (5 of 6 passed).
  - O2 (part_center ignores the hull's yaw, c = 1, s = 0), 11:41:01-11:41:50: ONLY d7 FAILED (:351), the rest pass.
  - O3 (phase 3 does not refresh the soldiers' records: the present volumes are the soldiers' boxes before this
    tick's moves, world.rs:1476-1478), 11:42:06-11:42:56: ONLY d7 FAILED (:351), the rest pass.
  - O4 (decode reserves by the header's count again, replay.rs:323), 11:43:27-11:44:18: NOTHING fails (47 pass).
  - O5 (carry 13, `*self = source.clone()`): carry13 FAILED "tick 2: soldiers' buffer" (:1927). = builder's P2.
  Reading: D-7's pins were re-pinned by this same fix pass (u4d-build.md F4) after the composition and phase-3 code
  were written, so a pin is no evidence that the frame convention or the phase-3 refresh is right, only that nothing
  changed since F4. By reading, both are right now: part_center (world.rs:446-458) is the old place_vehicles formula
  (x' = ox c - oy s, y' = ox s + oy c; the prefix's place_vehicles) on the hull record's grid yaw; present_volumes
  refreshes records[slot] from soldier_record (pre-snap raw -> grid = the snapped position's grid, carry 9), which is
  the old build_volumes' to_grid(s.pos) for the snapped s.pos.
- [6] Probes (a test's length raised, restored like a plant):
  - X1 h1 count over 54,000 SS ticks (an S30), 11:45:27-11:48:29: PASS. Records changed a tick: Soldier 47.73, Vehicle
    14.99, VehiclePart 5.62, Projectile 2.66, Door 0.03, Wall 0.40, Effect 7.40, OwnState 47.49 (126.3 a tick, was
    574 in round 1's probe K); parts hit 303,949 = part records changed 303,949 (no area-driven change in 30 min).
  - X2 t_m1 over 18,000 ES ticks (600 s, PM-0001-12's ES run length), 11:48:46-11:48:55: FAILED "gameplay events:
    7578" (thousandths a second; band 9,500-10,500). shots 70.783, spawns 21.120 (2,167 thrown), live 32.342: in band.
    Over the first 300 s the test's 9.853; so the second 300 s average (2 x 7.578 - 9.853) = 5.30 a second.
  - X3 D-6 over 1,800 s of SS (an S30), 11:50:10-11:51:59: PASS. 1,234,196 shots, 685.664 a second; 1,800 portal
    changes, 1.00 a second; 489 kills (0.272 a second; 100 s gives 28, 0.28); every per-tick minimum held (48
    soldiers, teams >= 16, >= 15 vehicles of which >= 4 large, >= 300 projectiles, 384 effect rows); large hulls <=
    1,400 cm out, others >= 5,199 cm, stand-ins start <= 1,200 cm. Magazine 32,000 lasts 2,194 s at one soldier's rate.
- [7] Carries and reading: carry 9 (raw pre-snap, grid of it; now per field over Soldier and OwnState records,
  tests/world.rs:1741-1802; the fixer's Q16 fails it; -41 quant's unit proof unchanged, tests/world.rs:2219-2237):
  MET. Carry 12 (layouts within FIELDS; s6 pins all 8 kinds by Field identity, tests/seam.rs:477-557; the fixer's
  Q22): MET, with W1-S1's change (velocity and weapon state out of SOLDIER, the ruling's lift); VELOCITY unchanged
  (grid.rs: +4 doc lines only vs 21a8597). Carry 13: storage identity MET (O5 fails it), counting half to U5 (C13;
  u4s-carries.md entry 48). Carry 14: MET at 10^6 B (220 ticks, 7,333 ms; Q19 fails both tests). Carry 36: MET
  (exhaustive match, tests/seam.rs:70-80; the fixer's Q23). ES's synthetic recording is 9,000 ticks by plan (§4.15,
  RQ5 "3 recordings x 5 min"; tests/replay.rs:20-28); ES's judged runs are 10 min each, every ES run checked by
  PM-0001-2's counter script within ±5 % (ADR 0001:1474-1480, :1600), driven by U4r's runner (plan §5 U4r :742:
  npt-bots' brawl and open-loop scripts), not by this generator today.
- [8] check.mjs, the one counted run, 11:54:59-11:59:37 UTC, by absolute path, cwd this copy's spikes/netcode-proof,
  shared target, offline, idle 12 s before, 2 s toucher, no other cargo at the end: all 10 steps PASS. fmt 0.4 s;
  clippy 8.8 s (measured and bites); clippy-wasm32 1.9 s (both); doc 33.4 s (both builds, host and wasm32, -D
  warnings); test 227.7 s: measured 351 passed, 0 failed, 0 skips; bites 388 passed, 0 failed, 0 skips (= expected);
  deps 0.7 s (108 crate files, 1 cargo config); refs 0.5 s (625 tracked files); marks 2.5 s (81 marks, drafts
  626df678); self 1.5 s (52 of 52); redact 0.8 s (10 of 10; 182 items, 0 findings). df 11 GB after.
- [9] Final: r2t-plant.py check ALL-OK (34 files = step 1's sha256, after Q1 and Q10a2 too); no r2t-backup left; no
  PLANT text in crates (grep); git status the same 10 M + 9 ?? as at the start; HEAD 21a8597; check.mjs 7eddc285,
  workspace Cargo.toml 75b6c160, Cargo.lock 3da1e15f unchanged. Nothing committed; nothing edited beyond restored
  plants.

## Round-1 findings (mine): CLOSED / NOT CLOSED
- M1 (ES misses PM-0001-2 bands; no ES test): CLOSED as ruled, on ES's 9,000-tick synthetic recording: t_m1
  (tests/world.rs:1005-1070) asserts the four bands and 48/15 every tick: 70.776 shots, 9.853 events, 21.120 spawns a
  second, 32.300 live; Q11a fails it (and the fixer's Q11b, Q11c). But see T2-M1: the event band holds for that
  window only.
- M2 (1 MB = 2^20): CLOSED (history.rs:7, 10^6 B; carry14 220 / 7,333; replay span 7,333 / 7,334; Q19 fails both).
- M3 (D-9 blind to a wrong blend): CLOSED (world.rs's blend unit test, fractions 0, 1, 127, 128, 255 against an f64
  reference; Q12 fails it 3.9 mm off at fraction 1).
- M4 (catch-up tick, FLAG_FIRED): CLOSED (FLAG_FIRED dropped, flags dense, world.rs:116-120; D-3 has a catch-up tick
  every 9th step, catch_ups > 40 asserted, and compares the own-state record; Q1 fails "CO step 4: record").
- M5 (load_movers grows pools): CLOSED (Movers sized at World::new, world.rs:782-795; load_movers_into guards every
  index, :2031-2058; the module test and r_m1; Q10a2 fails the module test).
- L1 (D-4 misses a NaN the snap launders): CLOSED (non_finite reads `pre`, world.rs:2005-2023; the fixer's Q24).
- L2 (std-sin unit test cannot fail): NOT CLOSED, not claimed (u4d-build.md "Left for", test L2): follow-up.
- L3 (clamp idempotence above about 40.57 m): CLOSED by the TUNE-2 ruling (bound in World's and the module's doc; D-8's
  half at the bound; the in-module scene test; the fixer's Q26, Q27).
- L4 (Replayer panics on a non-canonical recording): CLOSED (replay.rs:698-702; tests/replay.rs:608-637; the fixer's
  Q20).
- L5 (decode reserves 2^20 x 104 B from a 60 B header): CLOSED in code (replay.rs:320-323), NOT by any test: O4
  passes all 47 tests (T2-L2).
- L6 (SS magazine runs dry; D-6's 100 s cannot see it): CLOSED (32,000 rounds, const assert scene.rs:167-170; X3:
  685.664 shots a second and every per-tick minimum over a whole S30).
- N1 (d10 clamp shots not asserted), N2 (D-2 pipes without timeout): NOT CLOSED, not claimed, listed follow-ups.
- N3 (carry 13 storage identity): ruled C13, met in part; O5 shows the test bites; counting half to U5.

## New findings (round 2)
- T2-M1 MEDIUM, not before merge (with netcode N2-M1 and arch A2-L3, now measured): ES's synthetic recording meets
  PM-0001-2's 10 gameplay events a second only over the 300 s t_m1 reads. X2 (t_m1 at 18,000 ticks): 7.578 a second
  over 600 s, so about 5.30 in the second 300 s; shots, spawns and live projectiles stay in band. Cause (the fixer's
  point B): the generator's dead reckoning (replay.rs:431-432) ignores crouch, sprint and scripted speeds, so the aim
  among the other teams (replay.rs:458-466, one command in 11) drifts off its targets. `aim_one_in` = 11
  (scene.rs:80) was tuned on that same window, and scene.rs:114, :177 state "10 gameplay events a second" as a scene
  property. Scenario: U4d2 or U4r lengthens or windows an ES recording from this generator (PM-0001-12 judges ES runs
  of 10 minutes, each checked by PM-0001-2 within ±5 %: ADR 0001:1474-1480, :1600) and reads 7.6 events a second,
  24 % low, so PM-0001-2 fails and PM-0001-12's ES part is invalid. Reach today: none judged (the plan pins the
  synthetic ES recording at 9,000 ticks; judged ES runs come from U4r's runner). Cheapest proof: t_m1 at 18,000 ticks
  (X2). Fix: either t_m1 asserts each 100 s window in band over the length any consumer uses, or the generator's
  reckoning follows crouch, sprint and scripted speeds at the next GENERATOR bump (re-pinning D-7's ES); the doc
  lines say "over its 9,000-tick recording"; a U4r carry: PM-0001-2's ES checks shown on a 10-minute ES run of
  whatever drives ES before PM-0001-12's marks count.
- T2-L1 LOW, not before merge: two new invariants are held only by D-7's after-tick pin (tests/replay.rs:351), which
  this fix pass re-pinned after writing them: hull o part's frame convention (part_center, world.rs:446-458: O1 rotate
  by -yaw and O2 ignore yaw pass every semantic test, h1, a_m1, r_m1, D-1, D-2, D-3, D-6 included) and phase 3's
  refresh of the soldiers' records before the shots (present_volumes, world.rs:1475-1480: O3 passes the same). Both
  are right by reading today. Scenario: a later edit (e.g. write_records' order, arch A2-L4) breaks one and re-pins D-7
  with its own legitimate change; shots without a Rewind then meet last tick's boxes, or parts' boxes sit mirrored
  about the hull, and every semantic test stays green. Cheapest proof: two unit tests in world.rs's module:
  part_center at hull yaw 90 degrees maps offset (+120, 0, 100) to hull + (0, +120, 100) cm within 1 cm; and in one
  step a soldier moved out of its old box is hit at its new place by a no-Rewind shot (or: phase 3's soldier boxes
  equal hit_volumes_from of the post-move records).
- T2-L2 LOW, not before merge: rust L1 = my L5 is closed in code only (replay.rs:320-323): O4 (the 2^20 reserve
  restored) passes all 47 npt-sim tests; an allocation is unobservable without a counting allocator. Fix: one line in
  U5's carry 48 (u4s-carries.md): decoding a 60 B header that claims 2^20 ticks allocates at most room x
  size_of::<TickInput>().
- T2-L3 LOW (the fixer's point A, for the advisor): `aim_one_in` = 11 and `grenade_fuse_ms` = 1,515 are new stand-in
  tuning. aim_one_in is read only by the generator and its value is valid for the 300 s window (T2-M1);
  grenade_fuse_ms is read by World (world.rs:1466), so it also changes every bot-driven ES run's grenades (46 ticks,
  1.533 s, vs 2 s). Both need the advisor's nod as TUNE-1's did; the report states X2's number beside them.

## Fixer's open points
- (A): T2-L3. (B): T2-M1, Medium, measured. (C): SOLDIER's 8 fields are the ruling's 8 names (advisor-u4-wave1.md:79),
  the catalogue's row (:137) and the GROUPS test's 92-bit row; "(9 fields)" is a count slip. Weapon, Scene::weapon,
  Scene::health: public beyond W1-PUB's list, traced to TUNE-1's header condition, only caller D-6
  (tests/world.rs:618-628): text for §4.1 (as arch A2-L2, rust R2-L4). Carry 13: met in part, ruled (C13); O5 bites.
- For U4e's budget and TUNE-1's report: X1's 126.3 records changed a tick over an S30 (OwnState 47.49 of 48:
  owner-only, one per client); SS's kill rate 0.272 a second over an S30 (X3), not probe F's 0.42 (before this pass).

## Verdict
APPROVE WITH FINDINGS. Every round-1 finding of mine that the fix claims is CLOSED: M1-M5 with a plant I re-ran here
(Q11a, Q19, Q12, Q1, Q10a2); L1, L3, L4 by the fixer's plants (Q24, Q26-Q27, Q20), read, not re-run by me; L6 by
measurement (X3); L5 in code only (T2-L2); N3 by the C13 ruling (O5 shows its test bites). W1-S1's both halves proven:
Q2 (D-3's own-state record), P1, P3, P7 re-planted here; the fixer's Q16, Q17, Q18, Q22, Q23 read, not re-run; the
GROUPS row's 129 bits on the wire. check.mjs in this copy: 351 / 388 / 52 of 52, all 10 steps. No Critical or High,
so no round 3 from this lane. T2-M1 (Medium) and T2-L1 to T2-L3 (Low) are follow-ups.
DONE (2026-11-11 12:04 UTC)
````

### Mustafa's words on the drift, 2026-11-11 (his words)
_`.checkpoints/netcode-proof-test/history.md` lines 338-339 (sha256 1c1f401478f4), as they are:_

````text
  - The event-rate drift in the estimate scene: fix the scene before U4d2 pins the recordings. The 9.5 floor
    (PM-0001-2) does not move.
````

### §4.2 of u4-build-plan.md (cited by §5 U4d)
_`.checkpoints/netcode-proof-test/u4-build-plan.md` lines 205-249 (sha256 d928358b3ccb), as they are:_

````text
### 4.2 The seam
- `SimCore`:
  - `step(&mut self, input: &TickInput, history: &HitHistory) -> &[SimEvent]`;
  - `state_hash(&self) -> u64`;
  - `entity_records(&self) -> &[EntityRecord]`: per entity its id, kind, area id, grid integers, and the pre-grid
    values as `u32` bits (for -57's log; the room copies bits and never computes with them);
  - `hit_volumes(&self) -> &HitVolumes`; `portal_states(&self) -> &PortalStates` (each portal open or closed: A2);
  - `reports(&self) -> &InputReports`: the tick's clamp reports and the running refusal counters per kind, outside
    `state_hash` and outside the hashed event list; the room reads them through this accessor alone and logs them
    (the advisor's condition 1);
  - spawn, teleport and scripted attribute changes go in through `TickInput`.
  - One implementation: `World`. The static graph is npt-sim's plain-data `Areas` and `Portals` (plan.md:567), so the
    room's `AreaFlood` re-floods when `portal_states` changes, with no float and no sim internals.
- **`Rewind`** rides with each command that has fire held or a fire edge: `tick_a: u32`, `fraction: u8` (the blend
  from tick_a toward tick_a + 1; U4a's field table fixes the bits, net F116 proposes 8), `cover: CoverVariant`
  (R-10's two variants). Integers only. A Rewind naming a tick the lent ring does not hold is refused and counted by
  the sim, in `reports()` and never in the hash (the advisor's condition 1); the debug assert that a correct room never
  sends one sits in the room's `rewind_at`, not in the sim, so that condition 1's unit test can feed the sim one (the
  advisor's condition 1).
- **`HitHistory`** is npt-sim's fixed ring of `HitVolumes` per tick (capacity set at construction from U4h's test cap,
  in ms, through §4.3's functions). The room owns its instance and pushes `world.hit_volumes()` after each `step`; the
  replay driver does the same, so a recording needs no volumes. The client's predicting World carries none.
- **The clamp** (Q25): inside the shot phase, on the aim point in integer cm, from the in-step fire origin: a point
  beyond the weapon's reach + 0.87 cm (one grid cell's half-diagonal) moves along its own ray to exactly the reach and
  is snapped to the grid; a clamp report `{ slot, seq, point }` in `reports()` records it, never a `SimEvent` and never
  in the hash (the advisor's condition 1). A point within reach + 0.87 cm is never clamped again, so a clamped point
  sent back is stable (the twin's rule, §4.10), for every fire origin below the bound World's doc states (TUNE-2).
- **Two entry points** over the same phase functions (ADR 0004:166-178): the authority `step` and `World::predict` (one
  slot and its vehicle, against the movers `World::load_movers` loaded from the last decoded snapshot, drawing from no
  random stream). Both end each tick with the own-copy snap through `grid` (Q23).
- `World::ray_cast(&self, ray, movers: &HitVolumes) -> RayHit`: the static world from self, the movers supplied (the
  client's interpolated set for AimCast, the blended rewound set for a server shot: ADR 0001:258-260, :358-362; A11).
- Pools have fixed capacity; a full pool refuses and counts. The tick's event list is state and hashed
  (ADR 0004:544-561).
- **Refused input leaves no trace in the hash (the advisor's condition 1).** ADR 0004:544-556 hashes the tick's event
  list, so the clamp reports and every refusal counter (the cooldown, ammunition and reload edges, Rewinds outside the
  ring, a full pool) stay outside `state_hash` and outside that list (the advisor's U4s ruling, condition 1 (b)): they
  live in `reports()`, and the room logs them. The sim's one piece of per-slot input bookkeeping is the press counters
  of the last command it ran for that slot: a press edge is a counter that differs from them, and a refused press's
  edge is consumed there and changes nothing hashed. They stay outside `state_hash` and `StateDump` (a twin's counters
  differ by design); they are a function of the command stream alone, and the predicting World restores them from the
  client's own command at the snapshot's last-run sequence (U4g). `reports()` also lists each refusal per command
  (`{ slot, seq, kind }`, beside the clamp reports), so a twin removes exactly the refused parts. D-10 runs past the
  next accepted press. (The advisor's W1-U4D ruling: it replaces condition 1's 'what the server accepted, not what
  the client claimed' for the press counters.)
````

### §4.13 of u4-build-plan.md (cited by §5 U4d)
_`.checkpoints/netcode-proof-test/u4-build-plan.md` lines 427-451 (sha256 d928358b3ccb), as they are:_

````text
### 4.13 Scenes (data, recorded in each header)
- **SS, the stress scene,** with net F113's numbers: 48 soldiers in 3 x 16 in one cluster in the centre field; 15 big
  vehicles (the 4 of at least 20 parts in the fight); 300 live projectiles as trajectory records; 384 effect rows (8 per
  soldier) in the state and the hash; doors giving one portal change per second near the fight; one destructible-wall
  stand-in; the stand-in weapon's interval in integer sub-tick units for 700 shots per second with deaths counted;
  DM_Tri's extent (LEGACY_INVENTORY.md:153); the 4 very large units loop inside the fight and the other 11 elsewhere,
  and the stand-in projectiles start inside it; parts are recorded relative to their hull; effect rows record their
  end tick; a hit takes 1 of 1,000 health and SS and ES script no reloads, with a magazine that covers an S30 plus 5
  minutes at one soldier's rate (the advisor's W1-U4D ruling). A soldier counts in `soldiers_count` whether alive or
  awaiting respawn (one per slot), so deaths never drop it below 48.
- **The shot band under each precondition suite (arithmetic, before any run):** 700 / (48 x 30) = 0.486 shots per
  command-tick.
  - B1 under option A (nothing runs during a burst: OQ-0001-6 (a), ADR 0001:1156-1162): each 1 s burst loses about 22
    of its 30 ticks of commands (the redundancy cap recovers 8), about 0.73 s of shots; 10 bursts in 600 s give a mean of
    about 700 x (1 - 7.3 / 600) = 691.5 ≥ 665. Under option B the server repeats the last command on a miss, so about 700.
  - -21's forged suite: 11 forging clients, one kind each, 1,000 forged commands each; if every refusal costs that
    tick's shot, about 11 x 1,000 x 0.486 = 5,350 shots over 600 s, about 8.9 a second: a mean of about 691.
  - Any one client silent for a whole run: 700 x 47 / 48 = 685.4 ≥ 665. No suite can raise shots above 735: the
    command cap and the sim's cooldown bound each soldier's rate.
  U4r's contract test shows the band per suite on its short runs; a suite outside it goes to the advisor before its
  marks count.
- **S1's stand-ins on L1 (A17, Q26):** the 1 ms burn and the 2.80 ms top-up are off on L1 (no L1 mark reads time: ADR
  0001:1466-1467), recorded in `params.stand_ins`; the 384 effect rows stay, since they are state.
- ES, CO and TC as ADR 0001:1470-1482 gives them; TC's static targets are thin plates (at most 2 cm thick) on the
  ground, at 2.5 m and at 4 m (Q17; net F109).
````

### §4.14 of u4-build-plan.md (cited by §5 U4d)
_`.checkpoints/netcode-proof-test/u4-build-plan.md` lines 453-476 (sha256 d928358b3ccb), as they are:_

````text
### 4.14 Bites that need a site or a run setting (T4, T6, risk 3)
Each is planted where its code lives, shown to change state by a unit test in that sub-unit, and run judged in the
sub-unit that moves its mark.

| Bite | Its site | The check it fails | Run setting | Unit proof |
|---|---|---|---|---|
| -17/tick16, -58/tick16 | proto's header writer and the client's tick rebuild (the 16-bit value taken as the tick) | decoded hash: post-wrap frames logged under the wrong tick (an orphan pair) or as the sentinel | -17: `tick_start` = 63,736; -58: `ticks_below_wrap` = 30 | a frame at tick 65,537 hashes differently or is refused |
| -17/client-rounding | npt-client's call into `grid` (floor in place of nearest); the id lives in npt-client (`npt_client::BITES`, `grid_with_bite`), the grid holding only the defect (the advisor's U4s ruling, condition 1 (a)) | decoded hash | any -17 profile | the floor variant differs on about half of all position values |
| -18/no-fallback | the encoder's baseline choice, never falling back to the defaults | `burst_heal` (the fresh delta) | B1, ring ages 0-31 | a gap evicts B (on Loopback `npt_sim::ACK_RING_TICKS` tick periods, 1 s plus two tick periods at 30 Hz, since there a 1 s gap leaves B at the ring's edge: the advisor's U4E-Q14 ruling); `burst_eviction` = 1 |
| -21/not-older-than | the room's tick check (≥ in place of >) | fast clock's excess commands | the fast-clock suite, option A (marks.json -21 notes) | an equal-tick command runs |
| -21/no-catchup-cap | the room's per-tick command cap | forged: unrejected | the forged suite, option A, two datagrams in one tick | 16 commands run in one tick |
| -41/server-speed-1pct | the authority step's run speed x 1.01, not `predict`'s (Q24) | untraced corrections | CO, every profile | one moving tick differs |
| -41/server-quant-differs | the authority step's own-copy snap with another rounding than `grid`'s (Q23); the id lives at npt-sim's own-copy snap side (`OWN_COPY_SNAP_BITES`, `own_copy_snap_with_bite`), the grid holding only the defect (the advisor's U4s ruling, condition 1 (a)) | untraced corrections | CO | the snapped copy differs from the sent one |
| -45/buffer-one-short | Interp's slot count | L1 P2: extrapolated share < 1 % | P2; size = the clean P2 runs' logged largest delay in intervals + 2 - 1; the delay never clamped to the buffer | extrapolation on a fixed P2 arrival trace |
| -48/no-buffer-wait | the estimate without the input-buffer wait | view-hit share | where the buffer settles at 2 ticks or more (P3, option B, read from -20's logged depth; below 2, the advisor is asked before a fixed minimum is set) | honest view times replaced |
| -58/lt-cmd-seq | the room's sequence-forward compare | 0 commands refused | sequence 2 below its wrap | post-wrap commands refused |
| -58/lt-press | the room's press-edge compare | 0 presses lost or doubled | presses scripted through the wrap | an edge lost |
| -58/lt-tick-offset | the client's rebuild of the header's 16-bit 'last command run' sequence, where its replay starts (a plain `<` in place of its wrap-aware compare): U4e builds the rebuild, U4g's Predictor starts its replay there (the advisor's W1-N2 ruling; U4E-Q2) | 0 untraced corrections | command sequences crossing the wrap (as -58/lt-cmd-seq: a few steps below it) | U4e: the rebuilt sequence names the wrong command; U4g: the replay starts at the wrong command |
| -58/lt-event-seq | the client's event-number rebuild | decoded hash (event numbers are hashed: §4.6) | event numbers 3 below the wrap; SS's hits flow | a post-wrap event rebuilt wrong |
| -58/lt-ack | the room's rebuild of the 16-bit ack: a wrong rebuild names a snapshot never sent, and the check "the acknowledgement names a sent snapshot" refuses that datagram's commands | 0 commands refused | acks crossing the wrap | a post-wrap ack refused |
| -62/cues-from-replay | CueLog written from replays | plays at most once | suite `cues`: the replay skip off, a replay on every snapshot (as PM-0001-44: ADR 0001:1650) | a replayed shot plays twice |
| -62/server-echo | the room sending the owner its own cosmetic shot events | own cosmetic sent = 0 | `cues` | one event sent |
| -22/unbounded | the event queue's cap | headroom min = 0 | a full S30 of simulated time (marks.json -22) | headroom below 0 |
| PM-0004-21 hash-map | a bites-only HashMap of at least 32 keys whose whole iteration order folds into a state value | D-2 and U4d2: first mismatching tick = T | two processes; each logs its order digest | the two digests differ; equal digests mean "not exercised", never caught or missed |
````

### §4.15 of u4-build-plan.md (cited by §5 U4d)
_`.checkpoints/netcode-proof-test/u4-build-plan.md` lines 478-504 (sha256 d928358b3ccb), as they are:_

````text
### 4.15 Recordings (A1, A12, T9; Q4)
- **Fields:** a header (scene, seed, TICK_HZ, pool sizes, HitHistory's capacity (nit b)); then per tick a
  `TickInput`: per soldier one `Command` (slot 1 B, tick 4, sequence 2, buttons 2, press counters 4, move 2 (the advisor's U4s ruling, condition 1 (a)), aim in cm 12, view tick 4 and fraction 1) and its
  `Rewind` (present 1, tick_a 4, fraction 1, cover 1); spawns, teleports and scripted attribute changes when they occur.
  Fixed-width little-endian, no `usize` (plan.md:527-529).
- **Size:** 39 B per command (move is 2 B), so about 1,872 B a tick of commands for 48 soldiers before spawns and the
  tick's own header (the advisor's U4s ruling, condition 1 (a)); about 16.8 MB per 9,000 ticks and 50.5 MB for three,
  commands only (my arithmetic; U4d states the measured size). Fed to Wasm as 32-bit words: about 4.2 million calls
  each (the advisor's F1 ruling).
- **Both uses:** the sim-only replay steps npt-sim with each `TickInput` and the ring the driver rebuilds; -30's
  pre-check (a recorded npt-lab test, never judged: the advisor's condition 2) takes the SS P0 recording below (the
  advisor's condition 3) and re-encodes its commands as `Input` (stamped ticks, view times, sequences and press
  counters from the recording; the ack from what that leg's replay client decodes live) and lets the room compute its
  own Rewinds.
- **One driver:** `npt_sim::replay::Replayer` (feed words, step, per-tick hash and dump). npt-lab's `hash` subcommand
  runs it natively; npt-hash-wasm's exports wrap it; D-2 calls it in a test.
- **Sources:** U4d's three synthetic recordings (open-loop commands, Rewinds drawn inside the ring) from a seeded
  generator in npt-sim's library (`Recording::synthetic`), since U4d2's step regenerates them from the manifest,
  needing no room; U4r's three runner recordings (`scenario --record`: SS P2 seed 1, ES
  P3 seed 2, TC B1 seed 3), which hold the room's real Rewinds; and a fourth runner recording, **SS P0 seed 4**
  (`scenario --record`), the -30 pre-check's source (the advisor's condition 3). Replayed at P0, the other three's view
  times sit about one RTT older than P0's estimate, so the room would replace them on both legs, each from its own RTT;
  a recording made at P0 keeps them inside the tolerance. The determinism step keeps its three; the fourth serves the
  pre-check only.
- **Where they live:** regenerated at check time under .gate/ from a committed recordings manifest (scene, seed,
  script, generator version) with each file's SHA-256; a different hash fails the step and names the recording.
  Never committed as files.
````

## The carries this packet holds

_Rule: every entry of `.checkpoints/netcode-proof-test/u4s-carries.md` whose text matches `U4d-F|N2-M1|N2-M2|aim_one_in|estimate scene`._

### Carry 52
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 368-378 (sha256 287b15889b33), as they are:_

````text
52. **N2-M1, Medium (u4d-round2-netcode.md; Mustafa, 2026-11-11: "The event-rate drift in the estimate scene: fix the
    scene before U4d2 pins the recordings. The 9.5 floor (PM-0001-2) does not move."):** "Before U4d2 pins any
    recording or hash, the estimate scene's event-rate drift is fixed: the recording's dead reckoning (replay.rs's
    generator) ignores crouch, dropped commands, scripted move speeds, blocked moves and deaths, so the event rate
    falls over a run (per 50 s window: 13.4, 11.0, 10.1, 9.4, 9.7, 8.7 a second at 1 in 10; the test-engineer's probe
    X2, seed 2 at 1 in 11: 7.578 a second over 600 s). The fix and the re-calibration of `aim_one_in` are measured on
    the judged window (a 10-minute measured window after warm-up, ADR 0001:1600; 3 seeds), not on T-M1's 300 s from
    tick 0. PM-0001-2's floor of 9.5 events a second does not move. The form, the measurement and the owner are the
    advisor's U4d close ruling (advisor-u4d-close.md, B1.2 and B1.3): the main session's change U4d-F lands the fix,
    with ES's vehicles inside the fight (N2-M2), before U4d2's brief is final. U4d2 pins only recordings written by
    U4d-F's `Recording::GENERATOR` or later, and ES's hashes only from U4d-F's scene; its report names U4d-F's commit."
````

## The carries this packet left out

Ids: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66.
