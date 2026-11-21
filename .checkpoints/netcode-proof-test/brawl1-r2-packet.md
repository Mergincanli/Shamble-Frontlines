# Review packet: BRAWL-1 round 2 (only what changed since snapshot ee2973aa: the fix pass FP-1 to FP-9 of the advisor's ruling on round 1, and FP-0's two runner diffs)

_Cut by `review_packet.py` from `brawl1-r2-packet.json`. It holds sources only: code diffs, the approved plan's lines, the advisor's rulings, the last round's findings and Mustafa's words, each copied by line range. It holds no conclusion of the builder or the main session (no builder notes, brief, checkpoint card or summary). You stay free to read any file and run any test._

## Heads

- The snapshot reviewed in round 1: `.checkpoints/netcode-proof-test/wip/brawl1.patch` sha256 ee2973aa8ad8c453b5720099c27b37cb9980820da74f33ad9c78ddd7ad591dc7 (10 files on 352a245; committed at 0e2bf9a). Its read copy `.gate/brawl1-snap1`: a detached worktree at 352a245 with that patch applied (`git diff HEAD | sha256sum` there gives ee2973aa...).
- The snapshot under review: `.checkpoints/netcode-proof-test/wip/brawl1.patch` sha256 c2561b6ca54c8cde994fcdad5954c63d332df73f2db038c12aba3d71a0daa49f (10 files on 352a245; committed at 888ecbb). Its read copy is the builder's worktree `/home/user/Shamble-Frontlines/.claude/worktrees/brawl1` (detached at 352a245, the change uncommitted; `git diff HEAD | sha256sum` there gives c2561b6c...; no builder runs there now). Change nothing in it.
- The diffs below compare `.gate/brawl1-snap1/spikes/netcode-proof` with the builder's worktree's `spikes/netcode-proof`, file by file: the 6 files that differ (cmp). The fix pass added no file under spikes/ and removed none.
- Counts at snapshot ee2973aa (round 1's head), from `.gate/u4r/brawl1/final-check.log`: check.mjs all 11 steps; measured build 535 tests passed, bites build 613.
- The fix pass's runs, all through the lane's runner `.checkpoints/netcode-proof-test/u4r-run.sh` (sha256 2f754d125b10b6bde06a980f46cb4e078a582ca277ec8828bc48e8aacb064e50, after FP-0) in the builder's worktree, logs under `.gate/u4r/brawl1/`: fp-0-header.log (the first, before any edit), fp-check-1.log, fp-check-2.log, fp-fmt-2.log, the plant runs fp-plant-1.log to fp-plant-4.log, and FP-8's final-fp-check.log, final-fp-det.log and final-fp-regen-tests.log. Each log's header carries the diff hash at that moment (FP-0).
- FP-0's two runner diffs are copied below: the runners are outside the patch, and the main session edited them (each diff is `git diff` of the runner in commit 7402dac).
- Round 1's packet, with the approved plan's lines, the rulings it copies and the carries as round 1 had them: `.checkpoints/netcode-proof-test/brawl1-packet.md`.

## What changed (the diffs)

### spikes/netcode-proof/crates/bots/src/lib.rs
_`git diff --no-index .gate/brawl1-snap1/spikes/netcode-proof/crates/bots/src/lib.rs .claude/worktrees/brawl1/spikes/netcode-proof/crates/bots/src/lib.rs`:  1 file changed, 115 insertions(+), 24 deletions(-)._

````diff
diff --git a/.gate/brawl1-snap1/spikes/netcode-proof/crates/bots/src/lib.rs b/.claude/worktrees/brawl1/spikes/netcode-proof/crates/bots/src/lib.rs
index cadcebe..47f91ef 100644
--- a/.gate/brawl1-snap1/spikes/netcode-proof/crates/bots/src/lib.rs
+++ b/.claude/worktrees/brawl1/spikes/netcode-proof/crates/bots/src/lib.rs
@@ -56,7 +56,10 @@ const RETARGET_EVERY: u64 = 30;
 /// condition 1): past it the walk vector becomes [`toward_centre`] until the soldier is back inside [`LEASH_IN_CM`]. A
 /// distance in cm against positions in cm, with no rate in it (carry 39). Its margin is the door-entry margin: a
 /// soldier's centre enters a building at 1,800 cm at the nearest door (building 0's, x 1,800 with |y| < 60: npt-sim
-/// greybox.rs:83-136), 4 m beyond it.
+/// greybox.rs:83-136), 4 m beyond it as geometry. A soldier overshoots the outer radius, because the leash reads a
+/// decoded position some steps old and a skipped tick re-runs the last command: at BRAWL-1's review the sim-only
+/// test's skip pattern reached 1,720 cm and B1's bursts 1,832 cm, past the door entry (it prints the largest distance
+/// per pattern; BRAWL-1 round 1, NA-B1-2).
 const LEASH_OUT_CM: i64 = 1_400;
 /// The leash's inner radius, cm: back inside it, the open-loop walk again (the hysteresis between the two radii keeps
 /// it from flipping at each command). Above the wall stand-in's contact reach (its north face at 1,220 cm plus a
@@ -124,7 +127,9 @@ fn below(word: u64, n: u64) -> u64 {
 
 /// A move vector from (x, y) toward the origin at about full command length, inside the room's x² + y² ≤ 127² check
 /// (so the sim never scales it: world.rs:494-498): each part is 127 times its share of ⌈|(x, y)|⌉, truncated toward 0,
-/// so the vector is never longer than 127 and, at the leash's radii, longer than 125. Integers only.
+/// so the vector is never longer than 127 and, at the leash's radii, longer than 125. Integers only. Its inputs are
+/// decoded grid positions (16 bits on the wire: |x|, |y| at most 32,768, npt-proto quant.rs:197-215), so x² + y² stays
+/// below 2^31 and the i64 arithmetic cannot overflow; it is not for other inputs.
 fn toward_centre(x: i32, y: i32) -> [i8; 2] {
     let (x, y) = (i64::from(x), i64::from(y));
     let d2 = x * x + y * y;
@@ -242,9 +247,10 @@ impl Behaviour {
 
     /// The brawl's walk on the leash (BRAWL-1, Fix (a)): `walk`, the open-loop vector, unless the own soldier is past
     /// [`LEASH_OUT_CM`] from the centre, where it becomes [`toward_centre`] until the soldier is back inside
-    /// [`LEASH_IN_CM`]. It reads only the position the client decoded, never the server's state, so a re-run command or
-    /// a step cut short is corrected at the next command. `None` (no complete tick yet) or a dead soldier (a respawn
-    /// puts it at home) keeps the walk and the leash's state.
+    /// [`LEASH_IN_CM`]. It reads only the position the client decoded (x and y in toward_centre's domain), never the
+    /// server's state, so a re-run command or a step cut short is corrected once the decoded position shows it
+    /// (lead - 1 steps later, plus the ticks between the newest and the newest complete tick). `None` (no complete
+    /// tick yet) or a dead soldier (a respawn puts it at home) keeps the walk and the leash's state.
     fn leash(&mut self, own: Option<([i32; 3], bool)>, walk: [i8; 2]) -> [i8; 2] {
         let Some(([x, y, _], false)) = own else {
             return walk;
@@ -323,10 +329,17 @@ impl Behaviour {
 mod tests {
     use std::collections::VecDeque;
 
-    use npt_sim::{EntityKind, HitHistory, SimCore, StandIns, TickInput, World, ticks_for_ms};
+    use npt_sim::{
+        EntityKind, HitHistory, Placement, SimCore, StandIns, TickInput, World, ticks_for_ms,
+    };
 
     use super::*;
 
+    const _: () = assert!(
+        LEASH_IN_CM >= 1_300 && LEASH_OUT_CM >= LEASH_IN_CM + 100 && LEASH_OUT_CM <= 1_500,
+        "the advisor's plan review, condition 1's bounds"
+    );
+
     /// A live soldier at (x, y), as the endpoint would give it.
     fn at(x: i32, y: i32) -> Option<([i32; 3], bool)> {
         Some(([x, y, 150], false))
@@ -340,9 +353,11 @@ mod tests {
     /// position (today's open-loop walk, command for command). Inside LEASH_IN_CM it walks; between the radii it keeps
     /// what it did (the hysteresis); past LEASH_OUT_CM it steers to the centre and keeps steering under blocked steps
     /// (the position does not move) and skipped ones (the room re-ran the last command: the next position is further
-    /// out) until it is inside LEASH_IN_CM; None and dead keep the walk; the open-loop scripts never steer. No sideways
-    /// part is kept (condition 1 lets it go: no fixed box lies between the radii). Every leash vector holds x² + y² ≤
-    /// 127², above 125² from LEASH_IN_CM out, and points toward the centre.
+    /// out) until it is inside LEASH_IN_CM; None and dead keep the walk and the leash's state; on the outer radius it
+    /// is not past it; the open-loop scripts never steer. No sideways part is kept (condition 1 lets it go: no fixed
+    /// box lies between the radii). Every leash vector holds x² + y² ≤ 127², above 125² from LEASH_IN_CM out, and
+    /// points toward the centre. The blocked, kept-state and outer-radius checks each fall on a command whose
+    /// open-loop walk differs from the steer, so each can fail (BRAWL-1 round 1, TE-B1-3).
     #[test]
     fn brawl1_the_leash_steers_past_its_outer_radius_until_inside_its_inner_one() {
         let scene = Scene::stress();
@@ -362,12 +377,14 @@ mod tests {
         }
         let steer = [-127, 0];
         assert_eq!(give(at(1_450, 0)).0, steer, "past the outer radius");
+        let blocked = toward_centre(1_450, 200);
         for _ in 0..3 {
-            assert_eq!(
-                give(at(1_450, 0)).0,
-                steer,
-                "blocked: the position does not move"
+            let (got, walk) = give(at(1_450, 200));
+            assert_ne!(
+                walk, blocked,
+                "the twin's walk differs, so the check can fail"
             );
+            assert_eq!(got, blocked, "blocked: the position does not move");
         }
         assert_eq!(
             give(at(1_480, 30)).0,
@@ -388,10 +405,28 @@ mod tests {
         assert_eq!(got, walk, "dead: the walk");
         let (got, walk) = give(None);
         assert_eq!(got, walk, "no complete tick: the walk");
+        let (got, walk) = give(at(1_350, 0));
+        assert_ne!(
+            walk, steer,
+            "the twin's walk differs, so the check can fail"
+        );
+        assert_eq!(got, steer, "None and dead keep the leash's state");
         let (got, walk) = give(at(1_299, 0));
         assert_eq!(got, walk, "inside the inner radius: the walk again");
         let (got, walk) = give(at(1_350, 0));
         assert_eq!(got, walk, "between the radii, not past the outer since");
+        let (got, walk) = give(at(1_400, 0));
+        assert_ne!(
+            walk, steer,
+            "the twin's walk differs, so the check can fail"
+        );
+        assert_eq!(got, walk, "on the outer radius: not past it");
+        let (got, walk) = give(at(1_401, 0));
+        assert_ne!(
+            walk, steer,
+            "the twin's walk differs, so the check can fail"
+        );
+        assert_eq!(got, steer, "1 cm past the outer radius");
         assert_eq!(
             give(at(1_100, -1_100)).0,
             [-89, 89],
@@ -454,8 +489,8 @@ mod tests {
     }
 
     /// One run's record: soldier-ticks in a building and the first; the largest distance from the centre (its square in
-    /// cm², slot, tick, x, y); skipped ticks, chances and the longest streak; each soldier named as stuck (slot, tick,
-    /// x, y).
+    /// cm², slot, tick, x, y); skipped ticks, chances and the longest streak; the soldier-ticks seen dead; each soldier
+    /// named as stuck (slot, tick, x, y).
     #[derive(Debug, Default)]
     struct Record {
         in_building: u64,
@@ -464,19 +499,29 @@ mod tests {
         skipped: u64,
         chances: u64,
         streak: u32,
+        dead: u64,
         stuck: Vec<(u8, u32, i32, i32)>,
     }
 
     /// One Soldier record as the test reads it: (slot, grid position, dead bit, area).
     type Seen = (u8, [i32; 3], bool, u16);
 
-    /// Each Soldier record (the dead bit: flags, the layout's sixth field, bit 3: npt-sim world.rs:92, :120).
+    /// Each Soldier record (the dead bit: flags, the layout's sixth field, bit 3: npt-sim world.rs:92, :120), with the
+    /// dead bit tied to health (the seventh field) on every record: npt-sim sets both at a death and clears both at a
+    /// revive (world.rs:1630-1634, :1176-1188). This pins the literals npt-client copies (client lib.rs:250-251,
+    /// SOLDIER_FLAGS and SOLDIER_DEAD) against npt-sim's real deaths.
     fn soldiers(world: &World) -> Vec<Seen> {
         SimCore::entity_records(world)
             .iter()
             .filter(|r| r.kind == EntityKind::Soldier)
             .map(|r| {
                 let dead = r.grid[5] & (1 << 3) != 0;
+                assert_eq!(
+                    dead,
+                    r.grid[6] == 0,
+                    "slot {}: flags (field 5) bit 3 is set exactly when health (field 6) is 0",
+                    r.index
+                );
                 (
                     r.index as u8,
                     [r.grid[0], r.grid[1], r.grid[2]],
@@ -517,8 +562,12 @@ mod tests {
         for k in 0..ticks_for_ms(ms) {
             let t = FIRST.wrapping_add(k);
             let now = soldiers(&world);
-            for &(slot, p, _, area) in &now {
+            assert_eq!(now.len(), n, "tick {t}: one Soldier record per slot");
+            for &(slot, p, dead, area) in &now {
                 let s = usize::from(slot);
+                if dead {
+                    r.dead += 1;
+                }
                 if area != 0 {
                     r.in_building += 1;
                     r.first_in.get_or_insert((t, slot, area));
@@ -606,14 +655,25 @@ mod tests {
         r
     }
 
-    /// One run, printed: its pattern, seed and figures (the stuck print names each soldier).
+    /// One run, printed: its pattern, seed and figures (the stuck print names each soldier). The skip chances and the
+    /// rate per 1000 are the skip pattern's alone (ADV-PT-L1); every pattern prints its skipped ticks and longest
+    /// streak.
     fn printed(seed: u64, ms: u32, pattern: Pattern) -> Record {
         let r = sim_run(seed, ms, pattern);
-        let rate = (r.skipped * 1_000).checked_div(r.chances).unwrap_or(0);
+        let skips = match pattern {
+            Pattern::EveryTick => format!("skipped {}", r.skipped),
+            Pattern::Skips => format!(
+                "skipped {} of {} chances ({} per 1000)",
+                r.skipped,
+                r.chances,
+                (r.skipped * 1_000).checked_div(r.chances).unwrap_or(0)
+            ),
+            Pattern::Bursts => format!("skipped {} in its bursts", r.skipped),
+        };
         println!(
             "brawl1 sim seed {seed} {pattern:?} ({} ticks): in a building {} soldier-ticks (first {:?}); largest \
-             distance from the centre {} cm (slot {}, tick {}, at {}, {}); skipped {} of {} chances ({rate} per 1000), \
-             longest streak {}; stuck (slot, tick, x, y): {:?}",
+             distance from the centre {} cm (slot {}, tick {}, at {}, {}); {skips}, longest streak {}; dead {} \
+             soldier-ticks; stuck (slot, tick, x, y): {:?}",
             ticks_for_ms(ms),
             r.in_building,
             r.first_in,
@@ -622,21 +682,52 @@ mod tests {
             r.far.2,
             r.far.3,
             r.far.4,
-            r.skipped,
-            r.chances,
             r.streak,
+            r.dead,
             r.stuck
         );
         r
     }
 
     /// The sim-only test's short copy (condition 13): seed 1, `ticks_for_ms(120_000)`, both barred patterns; no Soldier
-    /// record with `area != 0` on any tick.
+    /// record with `area != 0` on any tick; and soldiers die in each pattern, so `soldiers()`' dead-bit tie runs on
+    /// real deaths (BRAWL-1 round 1, TE-B1-6).
     #[test]
     fn brawl1_sim_short_no_soldier_enters_a_building() {
         for pattern in [Pattern::EveryTick, Pattern::Skips] {
             let r = printed(1, 120_000, pattern);
             assert_eq!(r.in_building, 0, "{pattern:?}: first {:?}", r.first_in);
+            assert!(r.dead > 0, "the dead-bit tie is exercised");
+        }
+    }
+
+    /// TE-B1-2's npt-sim half (BRAWL-1 round 1): a soldier teleported to building 0's centre (2,200, 0: npt-sim
+    /// greybox.rs:92; its footprint is half-open, x 1,800 to 2,600) reads area 1 in its Soldier record, and every other
+    /// soldier area 0, so the area the sim-only test bars is one npt-sim sets. A teleport moves only a live soldier
+    /// (world.rs:1147-1151), and every soldier starts alive (:745).
+    #[test]
+    fn brawl1_a_soldier_teleported_into_building_0_reads_area_1() {
+        let scene = Scene::stress().with_stand_ins(StandIns::OFF);
+        let mut world = World::new(&scene, 1, FIRST);
+        let history = HitHistory::for_ms(2_000).expect("a ring");
+        let input = TickInput {
+            tick: FIRST,
+            teleports: vec![Placement {
+                slot: 0,
+                position: [2_200, 0, 0],
+            }],
+            ..TickInput::default()
+        };
+        world.step(&input, &history);
+        let now = soldiers(&world);
+        assert_eq!(
+            now.len(),
+            usize::from(scene.slots()),
+            "one Soldier record per slot"
+        );
+        for (slot, _, _, area) in now {
+            let want = if slot == 0 { 1 } else { 0 };
+            assert_eq!(area, want, "slot {slot}: area 1 in building 0, else 0");
         }
     }
 
````

### spikes/netcode-proof/crates/client/src/lib.rs
_`git diff --no-index .gate/brawl1-snap1/spikes/netcode-proof/crates/client/src/lib.rs .claude/worktrees/brawl1/spikes/netcode-proof/crates/client/src/lib.rs`:  1 file changed, 7 insertions(+), 3 deletions(-)._

````diff
diff --git a/.gate/brawl1-snap1/spikes/netcode-proof/crates/client/src/lib.rs b/.claude/worktrees/brawl1/spikes/netcode-proof/crates/client/src/lib.rs
index a79a0bb..74ed082 100644
--- a/.gate/brawl1-snap1/spikes/netcode-proof/crates/client/src/lib.rs
+++ b/.claude/worktrees/brawl1/spikes/netcode-proof/crates/client/src/lib.rs
@@ -243,7 +243,10 @@ pub struct ClientEndpoint {
 }
 
 /// A Soldier record's flags field, the sixth of its layout (npt-sim seam.rs's `SOLDIER`: x, y, z, yaw, pitch, flags,
-/// health, weapon), and its dead bit (npt-sim world.rs's `FLAG_DEAD`, bit 3): both private there.
+/// health, weapon), and its dead bit (npt-sim world.rs's `FLAG_DEAD`, bit 3): both private there. npt-bots' sim-only
+/// test pins the same two literals against npt-sim's real deaths (its `soldiers()`: the bit set exactly when health is
+/// 0), so a change there fails that test; both copies change together until npt-sim exposes them (BRAWL-1 round 1,
+/// EA-B1-1).
 const SOLDIER_FLAGS: usize = 5;
 const SOLDIER_DEAD: i32 = 1 << 3;
 /// Pings the endpoint holds to answer at its next step, at most (the room sends a few a second).
@@ -345,8 +348,9 @@ impl ClientEndpoint {
     /// position [x, y, z] (1 cm steps, as decoded: no float) and whether its flags say dead (bit 3), by value, from the
     /// newest tick that is complete and filed with a hash, ticks compared by the endpoint's wrap rule. `None` before
     /// the slot or the first such tick, or when that tick's own section holds no record of the client's soldier. A
-    /// newer tick that is incomplete, refused or the sentinel changes nothing. Never the room's state: only what this
-    /// client decoded.
+    /// newer tick that is incomplete, refused or the sentinel changes nothing while the complete tick stays in the
+    /// ring (ACK_RING_TICKS ticks); a newer tick that takes its ring slot drops it, and then the newest complete tick
+    /// still held is read, else `None`. Never the room's state: only what this client decoded.
     pub fn own_soldier(&self) -> Option<([i32; 3], bool)> {
         let slot = u16::from(self.slot?);
         let tick = self
````

### spikes/netcode-proof/crates/client/tests/endpoint.rs
_`git diff --no-index .gate/brawl1-snap1/spikes/netcode-proof/crates/client/tests/endpoint.rs .claude/worktrees/brawl1/spikes/netcode-proof/crates/client/tests/endpoint.rs`:  1 file changed, 48 insertions(+), 1 deletion(-)._

````diff
diff --git a/.gate/brawl1-snap1/spikes/netcode-proof/crates/client/tests/endpoint.rs b/.claude/worktrees/brawl1/spikes/netcode-proof/crates/client/tests/endpoint.rs
index 7adb1dd..5690535 100644
--- a/.gate/brawl1-snap1/spikes/netcode-proof/crates/client/tests/endpoint.rs
+++ b/.claude/worktrees/brawl1/spikes/netcode-proof/crates/client/tests/endpoint.rs
@@ -392,7 +392,9 @@ fn own_only(t: u32, groups: u8, own: Vec<EntityRecord>) -> Snapshot {
 
 /// BRAWL-1's read item (the advisor's plan review, condition 5): `own_soldier` is `None` before the first complete
 /// tick; after a complete tick it is the own record's x, y, z and dead bit (flags bit 3); a newer incomplete, refused or
-/// sentinel tick changes nothing; a newer complete tick moves it; a complete tick with no own record gives `None`.
+/// sentinel tick changes nothing; a newer complete tick moves it; a complete tick that a second, different group 0
+/// turns into the sentinel is not read, and the older complete tick is read again (BRAWL-1 round 1, TE-B1-4 (a)); a
+/// complete tick with no own record gives `None`.
 #[test]
 fn brawl1_own_soldier_is_the_newest_complete_filed_tick_s_own_record() {
     let mut e = endpoint();
@@ -432,6 +434,20 @@ fn brawl1_own_soldier_is_the_newest_complete_filed_tick_s_own_record() {
     );
     assert_eq!(out.hashes.len(), 1);
     assert_eq!(e.own_soldier(), Some(([-700, 700, 150], false)), "tick 105");
+    let out = feed(
+        &mut e,
+        vec![datagram(own_only(105, 1, vec![soldier(0, -701)]))],
+    );
+    assert_eq!(
+        out.hashes,
+        [(105, 0)],
+        "a second, different group 0 turns tick 105 into the sentinel (carry 61 (b))"
+    );
+    assert_eq!(
+        e.own_soldier(),
+        first,
+        "a complete tick turned sentinel is not read: tick 101 again"
+    );
     let out = feed(
         &mut e,
         vec![datagram(own_only(106, 1, vec![soldier(1, 5)]))],
@@ -440,6 +456,37 @@ fn brawl1_own_soldier_is_the_newest_complete_filed_tick_s_own_record() {
     assert_eq!(e.own_soldier(), None, "tick 106: no record of soldier 0");
 }
 
+/// BRAWL-1 round 1 (TE-B1-4 (b), NA-B1-1, RR-B1-5, EA-B1-3 (b)): `own_soldier` compares ticks by the endpoint's wrap
+/// rule. Tick 0 comes after tick u32::MAX; with ACK_RING_TICKS at 32, u32::MAX sits in ring slot 31 and 0 in slot 0,
+/// so a plain maximum and a "last matching ring entry" read would both still return tick u32::MAX's record.
+#[test]
+fn brawl1_own_soldier_compares_ticks_by_the_endpoint_s_wrap_rule() {
+    let mut e = endpoint();
+    let accepted = bytes(&Message::JoinAccepted(JoinAccepted {
+        slot: 0,
+        tick: u32::MAX - 1,
+        ..JoinAccepted::default()
+    }));
+    feed(&mut e, vec![(Channel::Control, accepted, 0)]);
+    let out = feed(
+        &mut e,
+        vec![datagram(own_only(u32::MAX, 1, vec![soldier(0, 11)]))],
+    );
+    assert_eq!(out.hashes.len(), 1, "tick u32::MAX complete and filed");
+    assert_eq!(
+        e.own_soldier(),
+        Some(([11, -11, 150], false)),
+        "tick u32::MAX"
+    );
+    let out = feed(&mut e, vec![datagram(own_only(0, 1, vec![soldier(0, 22)]))]);
+    assert_eq!(out.hashes.len(), 1, "tick 0 complete and filed");
+    assert_eq!(
+        e.own_soldier(),
+        Some(([22, -22, 150], false)),
+        "tick 0 comes after u32::MAX by the endpoint's wrap rule"
+    );
+}
+
 /// npt-client's planted defects (bites builds only), each in wave 1's two parts: a bites-only twin asserting the clean
 /// test's check fails on the bitten endpoint.
 #[cfg(feature = "bites")]
````

### spikes/netcode-proof/crates/lab/src/hash.rs
_`git diff --no-index .gate/brawl1-snap1/spikes/netcode-proof/crates/lab/src/hash.rs .claude/worktrees/brawl1/spikes/netcode-proof/crates/lab/src/hash.rs`:  1 file changed, 4 insertions(+), 3 deletions(-)._

````diff
diff --git a/.gate/brawl1-snap1/spikes/netcode-proof/crates/lab/src/hash.rs b/.claude/worktrees/brawl1/spikes/netcode-proof/crates/lab/src/hash.rs
index 85e70a0..ae12e1b 100644
--- a/.gate/brawl1-snap1/spikes/netcode-proof/crates/lab/src/hash.rs
+++ b/.claude/worktrees/brawl1/spikes/netcode-proof/crates/lab/src/hash.rs
@@ -1558,9 +1558,10 @@ mod tests {
             let e = manifest(&one(bad, 9_000, k)).expect_err(bad);
             assert!(e.contains("is not one or more of"), "{bad:?}: {e}");
         }
-        // At the rule's boundary below RR4-L1's 36,000-tick cap (advisor-u4r-r2): 360,000 bytes hold at most 30,000.
-        let small = |text: String| text.replace("\"bytes\": 799693", "\"bytes\": 360000");
-        let at = 360_000 / 12;
+        // At the rule's boundary below RR4-L1's 36,000-tick cap (advisor-u4r-r2): 359,993 bytes hold at most 29,999
+        // ticks; not a multiple of 12, so the rule's rounding down is checked (ADV-PT-L2).
+        let small = |text: String| text.replace("\"bytes\": 799693", "\"bytes\": 359993");
+        let at = 359_993 / 12;
         assert!(
             manifest(&small(one("TC-s3", at, k))).is_ok(),
             "ticks at bytes / 12"
````

### spikes/netcode-proof/crates/lab/src/logs.rs
_`git diff --no-index .gate/brawl1-snap1/spikes/netcode-proof/crates/lab/src/logs.rs .claude/worktrees/brawl1/spikes/netcode-proof/crates/lab/src/logs.rs`:  1 file changed, 102 insertions(+), 5 deletions(-)._

````diff
diff --git a/.gate/brawl1-snap1/spikes/netcode-proof/crates/lab/src/logs.rs b/.claude/worktrees/brawl1/spikes/netcode-proof/crates/lab/src/logs.rs
index 724dd4f..50fc356 100644
--- a/.gate/brawl1-snap1/spikes/netcode-proof/crates/lab/src/logs.rs
+++ b/.claude/worktrees/brawl1/spikes/netcode-proof/crates/lab/src/logs.rs
@@ -1,10 +1,10 @@
 //! The scenario's three logs (u4-build-plan.md §4.12 :410-415; marks/README.md §1 and §4; u4r-brief.md 2.1): on L1
 //! one npt-lab process writes one log per process value the checks select on, each with its own header: `room` (the
 //! room's per-tick World hash `h`, `hs.server_hash`, PM-0001-2's series, BRAWL-1's `in_building_slot_area`, the
-//! `admit` events, `burst_eviction`), `client`
-//! (every client's `hs.decoded_hash` and `tick.s.snap_incomplete`, one entry per slot) and `lab` (-57's errors,
-//! `burst_heal`, `counter_wrap`, the run's own records). Every tick and second record carries all its counters, 0
-//! included; every log ends with its footer.
+//! `admit` events, `burst_eviction`), `client` (every client's `hs.decoded_hash` and `tick.s.snap_incomplete`, one
+//! entry per slot) and `lab` (-57's errors, `burst_heal`, `counter_wrap`, the run's own records). Every tick and
+//! second record carries all its counters, 0 included (BRAWL-1's `in_building_slot_area`, a list, is written only on
+//! ticks with one: the advisor's plan review, condition 10); every log ends with its footer.
 //!
 //! **The client writer** (carry 62 (a) to (f); carry 73; README §1's `hs` rules): each client's hashes are filed under
 //! the tick it rebuilt (a 32-bit tick, unwrapped forward from the room's first tick, so a wrong rebuild past the 16-bit
@@ -776,7 +776,7 @@ impl Logs {
 #[cfg(test)]
 mod tests {
     use super::*;
-    use npt_sim::{Command, HitHistory, Scene, SimCore, SimEvent, StandIns, TickInput};
+    use npt_sim::{Command, HitHistory, Placement, Scene, SimCore, SimEvent, StandIns, TickInput};
 
     /// The advisor's U4r build ruling, point 6: the dump reader's counts at each tick equal the shots, hits and kills
     /// in `SimCore::step`'s returned events, over a short seeded run of the stress scene in which every soldier fires
@@ -868,4 +868,101 @@ mod tests {
         let w = u64::from(first);
         assert_eq!((logs.first[3], logs.last[3]), (Some(w), Some(w + 8_999)));
     }
+
+    /// BRAWL-1 round 1 (TE-B1-2 = RR-B1-1), the building figure's positive control: soldier 3, teleported to building
+    /// 0's centre (area 1) and given no command, stays there. A warm-up room tick writes its (slot, area) pair and
+    /// counts nothing; the next room tick, measured, counts one slot-tick for slot 3 (two ticks, because npt-log's
+    /// writer refuses a record at or before the last one: log lib.rs:467-474); both tick records hold the pair; the
+    /// summary line prints the count with its slot, and "not counted (--logs 0)" on a no-log run.
+    #[test]
+    fn brawl1_the_building_figure_counts_a_soldier_in_a_building_in_measured_ticks_only() {
+        let scene = Scene::stress().with_stand_ins(StandIns::OFF);
+        let first = 63_736u32;
+        let mut server = crate::scenario::Server::build(
+            &scene,
+            1,
+            first,
+            48,
+            0,
+            0,
+            &crate::scenario::Bites::default(),
+        );
+        server.room.script(
+            &[],
+            &[Placement {
+                slot: 3,
+                position: [2_200, 0, 0],
+            }],
+            &[],
+        );
+        server.room.tick();
+        let dir = crate::scenario::tests::scratch("brawl1-figure");
+        std::fs::create_dir_all(dir.as_path()).expect("the scratch folder");
+        let s = crate::scenario::tests::settings(&dir, "--profile P0 --seed 1");
+        let mut logs = Logs::open(Some(dir.as_path()), &s.meta, first, false).expect("three logs");
+        logs.room_tick(
+            &server.room,
+            server.room.last_tick(),
+            Phase::Warmup,
+            &[],
+            &[],
+            &[],
+        )
+        .expect("the warm-up tick");
+        assert_eq!(logs.counts.in_building_measured, 0, "warm-up: not counted");
+        assert!(logs.counts.in_building_slots.is_empty(), "warm-up: no slot");
+        server.room.tick();
+        logs.room_tick(
+            &server.room,
+            server.room.last_tick(),
+            Phase::Measured,
+            &[],
+            &[],
+            &[],
+        )
+        .expect("the measured tick");
+        assert_eq!(
+            logs.counts.in_building_measured, 1,
+            "measured: one slot-tick"
+        );
+        assert_eq!(
+            logs.counts.in_building_slots,
+            std::collections::BTreeSet::from([3]),
+            "measured: slot 3"
+        );
+        let counts = logs.finish(Ended::Clean, &[]).expect("the footers");
+        let room = crate::scenario::tests::read(&dir, "room");
+        assert_eq!(room.ticks.len(), 2, "the warm-up and the measured tick");
+        for t in &room.ticks {
+            assert_eq!(
+                t.s.get("in_building_slot_area"),
+                Some(&vec![3, 1]),
+                "tick {} ({:?}): written in any phase",
+                t.t,
+                t.ph
+            );
+        }
+        let o = crate::scenario::Summary {
+            counts: counts.clone(),
+            ..crate::scenario::Summary::default()
+        };
+        let line = crate::scenario::summary_line(&s, &o);
+        assert!(
+            line.ends_with("in a building: 1 slot-ticks (slots 3)"),
+            "{line}"
+        );
+        // parse takes --logs 0 only with --record ("a run keeps its logs"); the path is only stored, never written.
+        let quiet = crate::scenario::tests::settings(
+            &dir,
+            &format!(
+                "--profile P0 --seed 1 --logs 0 --record {}",
+                dir.join("brawl1.rec").display()
+            ),
+        );
+        let line = crate::scenario::summary_line(&quiet, &o);
+        assert!(
+            line.ends_with("in a building: not counted (--logs 0)"),
+            "{line}"
+        );
+    }
 }
````

### spikes/netcode-proof/crates/lab/src/scenario.rs
_`git diff --no-index .gate/brawl1-snap1/spikes/netcode-proof/crates/lab/src/scenario.rs .claude/worktrees/brawl1/spikes/netcode-proof/crates/lab/src/scenario.rs`:  1 file changed, 26 insertions(+), 6 deletions(-)._

````diff
diff --git a/.gate/brawl1-snap1/spikes/netcode-proof/crates/lab/src/scenario.rs b/.claude/worktrees/brawl1/spikes/netcode-proof/crates/lab/src/scenario.rs
index 3841812..08efd8f 100644
--- a/.gate/brawl1-snap1/spikes/netcode-proof/crates/lab/src/scenario.rs
+++ b/.claude/worktrees/brawl1/spikes/netcode-proof/crates/lab/src/scenario.rs
@@ -367,8 +367,7 @@ impl Server {
     /// The lab's one server (advisor-u4r-r1, EA-U4R-2), for `run` and the pre-check's `run_leg`: a room on a new
     /// [`Sim`] of `scene` (the caller's, stand-ins off), `seed` and `first_tick`, with every slot's seeded token,
     /// `clients` client slots, the lab's content hash and `event_start`, behind one decoder per session on `bites`'
-    /// `Quant`;
-    /// `bites` plants its World bite (armed at `arm_tick`), its room bite and its pump bite. `run_leg` passes
+    /// `Quant`; `bites` plants its World bite (armed at `arm_tick`), its room bite and its pump bite. `run_leg` passes
     /// `Bites::default()` and event start 0, then sets its own decoder bite.
     pub(crate) fn build(
         scene: &Scene,
@@ -394,7 +393,7 @@ impl Server {
         server
     }
 
-    pub(crate) fn new(room: Room<Sim>, quant: Quant, rtt: RttSource) -> Server {
+    fn new(room: Room<Sim>, quant: Quant, rtt: RttSource) -> Server {
         Server {
             room,
             sessions: Vec::new(),
@@ -1430,8 +1429,9 @@ pub(crate) fn parse(argv: &[String]) -> Result<Settings, String> {
     let last_tick = measured_end
         .checked_add(after_max_ticks)
         .ok_or("--after-max-s: the run's last tick overflows 64 bits".to_owned())?;
-    // advisor-u4r-r2, RR4-L2, after the 64-bit checks (which keep their messages): the run's last tick, counted from
-    // its first (the warm-up, measured and after phases' ticks), fits 32 bits, on every profile.
+    // advisor-u4r-r2, RR4-L2, after the run-length 64-bit checks (which keep their messages): the run's last tick,
+    // counted from its first (the warm-up, measured and after phases' ticks), fits 32 bits, on every profile. The
+    // burst block's own 64-bit check comes below it, then its 32-bit cap.
     if last_tick > u64::from(u32::MAX) {
         return Err(format!(
             "--warmup-s, --measured-s and --after-max-s: the run's last tick, {last_tick} ticks from its first, is \
@@ -2573,7 +2573,7 @@ pub(crate) mod tests {
             "one burst"
         );
         // RR4-L2: a run's last tick past 32 bits that fits 64 (the defaults' 300 + 1,800 ticks, then the after phase);
-        // u32::MAX itself or less accepted.
+        // u32::MAX is odd, so never a whole-second length; the largest below it (4,294,967,280 ticks) accepted.
         let e =
             parsed("P0", "--after-max-s 143165507").expect_err("the run's last tick above 32 bits");
         assert_eq!(
@@ -2585,6 +2585,26 @@ pub(crate) mod tests {
             parsed("P0", "--after-max-s 143165506").is_ok(),
             "4,294,967,280 ticks"
         );
+        // BRAWL-1 round 1, RR-B1-4: the last burst's 32-bit cap at its boundary, (1 x 143,165,566 + 0) x 30 + 300 =
+        // 4,294,967,280 ticks accepted (`burst_starts` has no in-run check), 30 ticks more refused.
+        assert!(
+            parsed(
+                "B1",
+                "--bursts 2 --burst-first-s 0 --burst-every-s 143165566"
+            )
+            .is_ok(),
+            "the last burst at 4,294,967,280 ticks"
+        );
+        let e = parsed(
+            "B1",
+            "--bursts 2 --burst-first-s 0 --burst-every-s 143165567",
+        )
+        .expect_err("the last burst's tick above 32 bits");
+        assert_eq!(
+            e,
+            "--bursts 2, --burst-first-s 0 and --burst-every-s 143165567: the last burst's tick, 4294967310 ticks from \
+             the run's first, is above 4294967295 (32 bits)"
+        );
     }
 
     /// advisor-u4r-r1, FP-6 items 1 and 2 (NC-U4R-2; EA-U4R-3 = RR3-L8; bites builds only): `scenario` refuses the
````

## What it is judged against

### FP-0: u4r-run.sh's change (git diff in commit 7402dac)
_`.checkpoints/netcode-proof-test/u4r-run.sh.te-b1-7.diff` lines 1-12 (sha256 a458c9e45029), as they are:_

````text
diff --git a/.checkpoints/netcode-proof-test/u4r-run.sh b/.checkpoints/netcode-proof-test/u4r-run.sh
index 8678438..8f04204 100644
--- a/.checkpoints/netcode-proof-test/u4r-run.sh
+++ b/.checkpoints/netcode-proof-test/u4r-run.sh
@@ -64,6 +64,7 @@ avail=$(df -B1 --output=avail /home/user | tail -1 | tr -dc 0-9)
 {
   echo "lock $LOCK held ($mode) at $(date -u +%T) UTC; idle after ${i} s"
   echo "pwd $(pwd); free $((avail/1048576)) MiB; HEAD $(git rev-parse --short HEAD); CARGO_TARGET_DIR $CARGO_TARGET_DIR"
+  echo "diff $(git diff HEAD -- . | sha256sum | cut -d' ' -f1); untracked $(git status --porcelain --untracked-files=all -- . | sha256sum | cut -d' ' -f1)"
 } > "$log"
 if [ "$avail" -lt $((4*1024*1024*1024)) ]; then
   echo "STOP: under 4 GB free" >> "$log"
````

### FP-0: main-run.sh's change (git diff in commit 7402dac)
_`.checkpoints/netcode-proof-test/main-run.sh.te-b1-7.diff` lines 1-12 (sha256 910a3e04df49), as they are:_

````text
diff --git a/.checkpoints/netcode-proof-test/main-run.sh b/.checkpoints/netcode-proof-test/main-run.sh
index 9901cf6..d2bb744 100644
--- a/.checkpoints/netcode-proof-test/main-run.sh
+++ b/.checkpoints/netcode-proof-test/main-run.sh
@@ -39,6 +39,7 @@ avail=$(df -B1 --output=avail /home/user | tail -1 | tr -dc 0-9)
 {
   echo "lock $LOCK held exclusively at $(date -u +%T) UTC; idle after ${i} s"
   echo "pwd $(pwd); free $((avail/1048576)) MiB; HEAD $(cd "$M" && git rev-parse --short HEAD)"
+  echo "diff $(git diff HEAD -- . | sha256sum | cut -d' ' -f1); untracked $(git status --porcelain --untracked-files=all -- . | sha256sum | cut -d' ' -f1)"
 } > "$log"
 if [ "$avail" -lt $((4*1024*1024*1024)) ]; then
   echo "STOP: under 4 GB free" >> "$log"
````

### The advisor's ruling on round 1: the fix pass, the pins, round 2, the merge step, U4g's brief, the size and each finding (advisor-brawl1-r1-full.md)
_`.checkpoints/netcode-proof-test/advisor-brawl1-r1-full.md` lines 1-460 (sha256 86bec63bfffb), as they are:_

````text
# Advisor: the ruling on BRAWL-1's review round 1 (workflow wf_59e64b31-fa2), 2026-11-15

Written by script, word for word, from the workflow's journal (advisor-r1's StructuredOutput result). The round: brawl1-review-r1-full.md. Its notes: advisor-brawl1-r1.md. Nothing below is the main session's.

**Snapshot:** wip/brawl1.patch, sha256 ee2973aa8ad8c453b5720099c27b37cb9980820da74f33ad9c78ddd7ad591dc7. I checked that the read copy (/home/user/Shamble-Frontlines/.claude/worktrees/brawl1, HEAD 352a245, 10 files modified, nothing new) gives the same value for `git diff HEAD | sha256sum`. numstat: 875 inserted and 123 deleted, 998 changed lines. I checked every finding against the code in the read copy and against the builder's logs (.gate/u4r/brawl1/). I ran no cargo or check.mjs command. The plants need edits I may not make, and the reviewers had already re-run the tests at this snapshot. I diffed their full-test prints (.gate/u4r/brawl1-review-test/t2-full-release.log, .gate/u4r/brawl1-review-netcode/run1.log) against final-regen-tests.log: they are identical. Disk at my check: 6,566 MiB free. The shared target holds 6,442 MiB: debug 6,008 (npt-* deps 1,026 and incremental 3,171 of it), release 250, doc 15, wasm32 172. My notes: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/advisor-brawl1-r1.md. I wrote no logs, because I ran nothing.

## Summary

All three Mediums are confirmed, and one fix pass closes them. TE-B1-1: no run shows that the sim-only test can fail. So a leash-off plant is run and recorded, and the test now asserts that it sees 48 Soldier records. If the plant does not bite on area, a door-entry distance bar (1,800 cm) is added and must bite. TE-B1-2 and RR-B1-1 are the same gap, and one fix closes both. A logs.rs test teleports a soldier into building 0 and checks three things: the building figure reads 1 in a measured tick; the warm-up tick writes the pair but does not count it; the summary line prints 'in a building: 1 slot-ticks (slots 3)'. A plant shows that this test fails if the figure is cut. A bots test shows that npt-sim gives area 1 there too. Of the 20 Lows, 16 ride this push. All of them are tests or doc words, and none changes behaviour. RR-B1-7 and EA-B1-1 ride only in part; the rest of each becomes a follow-up. Two are recorded as report words, and two need no action because the points ruling already settled them. Of my points ruling's findings: ADV-PT-L1 and ADV-PT-L2 ride, ADV-PT-L3 stays a follow-up, ADV-PT-M1 goes into U4g's brief merged with NA-B1-2 (c) and TE-B1-5, and ADV-BP-3 and ADV-BP-4 go in the report. One correction to a reviewer: TE-B1-5's 'obstacle at 18.3 m / the 14 m vehicle' is wrong. The sim-only test runs with stand-ins off, and no solid lies within 1.6 m of the three maxima. On the recording path no fix changes library behaviour, so the three new pins must stay. The end checks prove it, and the full test's round-1 figures must come out equal. Round 2: test-engineer and rust-reviewer, in one batch. The merge-step clean may come right before the counted run, but only as the approved part-cleans (B) and (C). The folder-level `cargo clean --target-dir .gate/shared-target` is not approved. Expected size after the fix pass: about 1,270 lines, under 1,400. Nothing for Mustafa.

## The fix pass, in order

### Step 1: 1. THE COPY AND ITS RULES (the main session sends this with the steps below)

- **Findings:** TE-B1-1, TE-B1-2, RR-B1-1
- **Words:** A fresh builder copy (copy 2): general-purpose, opus, effort xhigh. It works in the same worktree (/home/user/Shamble-Frontlines/.claude/worktrees/brawl1), with no isolation. Its prompt is copy 1's launch prompt plus a resume part. Before any edit it checks that `git diff HEAD | sha256sum` gives ee2973aa8ad8c453b5720099c27b37cb9980820da74f33ad9c78ddd7ad591dc7. If it does not, the copy stops and tells the main session. It reads brawl1-build.md and the diff first, and adds a new section 'Fix pass, round 1' to brawl1-build.md as it goes. The rules of brawl1-brief.md section 4 hold throughout:
- Every change is made with the Edit tool. rustfmt's diffs are applied by Edit; never `cargo fmt` writing files.
- Every run goes through u4r-run.sh with -x, df first, and 4 GiB free before any release or wasm32 build. A floor stop is reported and never cleaned around.
- Each plant is restored by Edit, and the file's sha256 is then checked equal to its pre-plant value.
- Counts are deltas from 535/613, and each new test is named with its file.
- A check.mjs test-step failure is recorded whole (TE2-5) and never retried.
- Any SHA-256 mismatch in det-u4.mjs or the precheck regeneration: stop. No re-pin and no retry; the log comes to me whole, with the first differing tick.
- Any change to a figure the full sim-only test printed in round 1 (FP-8 (4)): stop, and it comes to me whole.
- SIZE-CHECK: past 1,400 changed lines by numstat against 352a245 (SERIES.md included), stop. It goes to Mustafa in three lines.
- No commit.
The Lows ride this push because the Mediums need it anyway; a Low never triggers a push on its own. At the end the main session cuts a new snapshot by script, and round 2 reviews its diff against ee2973aa.
- **Evidence:** From FP-0 on, each runner log's header carries the diff hash at that moment (a log run before any edit shows ee2973aa...). The notes section in brawl1-build.md. Logs under .gate/u4r/brawl1/ named fp-*.

### Step 2: 2. FP-0 (the main session, before copy 2 starts; this ruling approves the runner edit, outside the change set, as FP-0 in U4r's round 1)

- **Findings:** TE-B1-7
- **Words:** In /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4r-run.sh, inside the header block (:64-67), after the `echo "pwd $(pwd); free ...; HEAD ..."` line (:66), add:
`echo "diff $(git diff HEAD -- . | sha256sum | cut -d' ' -f1); untracked $(git status --porcelain --untracked-files=all -- . | sha256sum | cut -d' ' -f1)"`
In main-run.sh, add the same line after :41. Its working folder is the main folder's spikes/netcode-proof, set at :26.
Rules: Edit tool only. Save each change as a .diff beside its runner (u4r-run.sh.te-b1-7.diff, main-run.sh.te-b1-7.diff), in INF2-4's form. `bash -n` on each runner must print nothing. No other runner line changes.
Why: today every log records only 'HEAD 352a245' while the change is uncommitted. Run from spikes/netcode-proof, `git diff HEAD -- .` equals the whole `git diff HEAD`, because every change lies under that folder. So the line gives the snapshot's own sha256. On a clean committed tree, both hashes are the sha256 of empty input.
- **Evidence:** The two .diff files. Empty `bash -n` output. Copy 2's first log shows the diff line. At M3, the counted run's header should show e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 for both diff and untracked.

### Step 3: 3. FP-1: the sim-only test (bots/src/lib.rs, the #[cfg(test)] module only)

- **Findings:** TE-B1-1, TE-B1-2, TE-B1-6, NA-B1-3, RR-B1-3, EA-B1-1, ADV-PT-L1
- **Words:** (a) TE-B1-1's record check: after `let now = soldiers(&world);` (bots lib.rs:519), add `assert_eq!(now.len(), n, "tick {t}: one Soldier record per slot");`. The World writes one record per soldier slot, dead or alive (npt-sim world.rs:1816-1818).

(b) The dead-bit tie: one fix for TE-B1-6, NA-B1-3, RR-B1-3 and the part of EA-B1-1 that is done now.
- In `soldiers()` (:474-488), after `let dead = r.grid[5] & (1 << 3) != 0;`, add `assert_eq!(dead, r.grid[6] == 0, "slot {}: flags (field 5) bit 3 is set exactly when health (field 6) is 0", r.index);`. npt-sim sets flags = FLAG_DEAD and health 0 together at a death (world.rs:1630-1634) and clears both at a revive (:1176-1188).
- Record (:459-468) gains `dead: u64`: the soldier-ticks seen dead, counted in sim_run's loop over `now` (bind the dead flag at :520). printed() prints it.
- The short copy (:636-640) asserts `r.dead > 0` for each pattern, with the message "the dead-bit tie is exercised". If a pattern at seed 1 shows none, that assert moves to the full test (any seed of that pattern), and the notes say why.
- soldiers()'s doc adds: 'This pins the literals npt-client copies (client lib.rs:247-248, SOLDIER_FLAGS and SOLDIER_DEAD) against npt-sim's real deaths.'
TE-B1-6's separate kill test is not needed: this tie runs on every record of every tick, with the brawl's real deaths.

(c) TE-B1-2's npt-sim half: a new #[test] in the same module, `brawl1_a_soldier_teleported_into_building_0_reads_area_1`.
- It builds `World::new(&Scene::stress().with_stand_ins(StandIns::OFF), 1, FIRST)`.
- It takes one step with `TickInput { tick: FIRST, teleports: vec![Placement { slot: 0, position: [2_200, 0, 0] }], ..TickInput::default() }` and `HitHistory::for_ms(2_000)`.
- It asserts that `soldiers(&world)` gives slot 0 area 1 and every other slot area 0.
The point is building 0's centre (greybox.rs:92). Its footprint is half-open, x 1,800 to 2,600 (:103-104, :176-183). A teleport moves only a live soldier (world.rs:1147-1151), and all soldiers start alive (:745). Import Placement.

(d) ADV-PT-L1: printed() (:610-631) prints the chances and the per-1000 rate only for Pattern::Skips. EveryTick prints 'skipped {n}' alone, and Bursts prints 'skipped {n} in its bursts'. The numbers come from the Record, with no literal. The longest streak stays for all three patterns.
- **Evidence:** fmt and clippy clean in both builds (check.mjs). The new test passes. The short copy passes in both builds; record each build's time (at most 60 s, condition 13) and its dead figure, which must be above 0. The full test's prints: FP-8 (4).

### Step 4: 4. FP-2: the steering unit test

- **Findings:** TE-B1-3
- **Words:** In brawl1_the_leash_steers_past_its_outer_radius_until_inside_its_inner_one (bots lib.rs:346-432). I re-computed the command sequence: slot 5 at seed 1 has walk phase 7.

(a) Replace the blocked loop (:365-371) with:
`let blocked = toward_centre(1_450, 200); for _ in 0..3 { let (got, walk) = give(at(1_450, 200)); assert_ne!(walk, blocked, "the twin's walk differs, so the check can fail"); assert_eq!(got, blocked, "blocked: the position does not move"); }`
toward_centre(1,450, 200) = [-125, -17], which is in no MOVES entry. Today's asserts sit on commands 4 to 6, whose walk is [-127, 0]: the steer itself, so they cannot fail.

(b) Right after the None case (:389-390), insert:
`let (got, walk) = give(at(1_350, 0)); assert_ne!(walk, steer, "the twin's walk differs, so the check can fail"); assert_eq!(got, steer, "None and dead keep the leash's state");`
This is command 12, whose walk is [-89, -89].

(c) Right after the case 'between the radii, not past the outer since' (:393-394), insert:
`let (got, walk) = give(at(1_400, 0)); assert_ne!(walk, steer, "the twin's walk differs, so the check can fail"); assert_eq!(got, walk, "on the outer radius: not past it"); let (got, walk) = give(at(1_401, 0)); assert_ne!(walk, steer, "the twin's walk differs, so the check can fail"); assert_eq!(got, steer, "1 cm past the outer radius");`
These are commands 15 and 16, whose walks are [-89, -89] and [-49, -117]. The diagonal case after them still steers to [-89, 89].

(d) At the top of the tests module: `const _: () = assert!(LEASH_IN_CM >= 1_300 && LEASH_OUT_CM >= LEASH_IN_CM + 100 && LEASH_OUT_CM <= 1_500, "the advisor's plan review, condition 1's bounds");`.
- **Evidence:** The unit test passes. Its count does not change: it is the same test.

### Step 5: 5. FP-3: the lab figure's positive control (logs.rs; TE-B1-2 and RR-B1-1 name the same gap and are fixed once)

- **Findings:** TE-B1-2, RR-B1-1, EA-B1-4
- **Words:** In logs.rs's #[cfg(test)] module (:775 on), add `brawl1_the_building_figure_counts_a_soldier_in_a_building_in_measured_ticks_only`:
1. `let scene = Scene::stress().with_stand_ins(StandIns::OFF); let first = 63_736u32; let mut server = crate::scenario::Server::build(&scene, 1, first, 48, 0, 0, &crate::scenario::Bites::default());`
2. `server.room.script(&[], &[Placement { slot: 3, position: [2_200, 0, 0] }], &[]); server.room.tick();` This puts soldier 3 in building 0 (area 1). With no command it stays there.
3. Take a folder from `crate::scenario::tests::scratch("brawl1-figure")` and make it with `std::fs::create_dir_all`; Logs::open's File::create_new needs the folder to exist (logs.rs:77-88). Then `let s = crate::scenario::tests::settings(&dir, "--profile P0 --seed 1"); let mut logs = Logs::open(Some(&dir), &s.meta, first, false)`.
4. `logs.room_tick(&server.room, server.room.last_tick(), Phase::Warmup, &[], &[], &[])`. Assert that `logs.counts.in_building_measured == 0` and `logs.counts.in_building_slots.is_empty()`.
5. `server.room.tick();` then `logs.room_tick(&server.room, server.room.last_tick(), Phase::Measured, &[], &[], &[])`. Assert that `in_building_measured == 1` and `in_building_slots == BTreeSet::from([3])`. Two room ticks are needed: npt-log's writer refuses a tick record at or before the last one (log lib.rs:467-474). So TE-B1-2's 'the same tick in both phases' would fail on the writer, not on the figure.
6. `let counts = logs.finish(Ended::Clean, &[])`. Read room.jsonl with `crate::scenario::tests::read(&dir, "room")`. Both tick records hold `in_building_slot_area` = [3, 1], the warm-up one too (condition 10: written in any phase).
7. `summary_line(&s, &Summary { counts: counts.clone(), ..Summary::default() })` ends with "in a building: 1 slot-ticks (slots 3)". With `settings(&dir, "--profile P0 --seed 1 --logs 0")`, it ends with "in a building: not counted (--logs 0)".
The test holds the Scratch guard until its last assert (T-5's rule). The figure's code (:584-599) does not change in this pass (RR-B1-7 (1) is deferred).

EA-B1-4: replace logs.rs:6-7's 'Every tick and second record carries all its counters, 0 included; every log ends with its footer.' with 'Every tick and second record carries all its counters, 0 included (BRAWL-1's `in_building_slot_area`, a list, is written only on ticks with one: the advisor's plan review, condition 10); every log ends with its footer.'
- **Evidence:** The new test passes in both builds and is named in check.mjs's counts. Plant P2 (FP-7) makes it fail at its first figure assert.

### Step 6: 6. FP-4: the client read item's tests and docs

- **Findings:** TE-B1-4, NA-B1-1, RR-B1-5, EA-B1-3, RR-B1-3
- **Words:** (a) TE-B1-4 (a): in brawl1_own_soldier_is_the_newest_complete_filed_tick_s_own_record (client/tests/endpoint.rs:397-442), after the tick 105 assert (:435) and before tick 106, add:
`let out = feed(&mut e, vec![datagram(own_only(105, 1, vec![soldier(0, -701)]))]); assert_eq!(out.hashes, [(105, 0)], "a second, different group 0 turns tick 105 into the sentinel (carry 61 (b))"); assert_eq!(e.own_soldier(), first, "a complete tick turned sentinel is not read: tick 101 again");`
The group handler passes a different group to sentinel(), which sets only `filed` (client lib.rs:626-637, :725-731). Tick 101 is then the newest complete tick filed Hash still in the ring. The test's doc line names this case.

(b) One wrap case, for TE-B1-4 (b), NA-B1-1, RR-B1-5 and EA-B1-3 (b): a new #[test] `brawl1_own_soldier_compares_ticks_by_the_endpoint_s_wrap_rule`, in NA-B1-1's form.
- A fresh endpoint; JoinAccepted for slot 0 at tick u32::MAX - 1.
- A complete own_only(u32::MAX, 1, vec![soldier(0, 11)]). Assert own_soldier = Some(([11, -11, 150], false)).
- Then a complete own_only(0, 1, vec![soldier(0, 22)]). Assert out.hashes.len() == 1, and own_soldier = Some(([22, -22, 150], false)) with the message 'tick 0 comes after u32::MAX by the endpoint's wrap rule'.
With ACK_RING_TICKS at 32, tick u32::MAX sits in ring slot 31 and tick 0 in slot 0. So this one case kills both mutants: a plain maximum and a 'last matching ring entry' read both return tick u32::MAX's record. TE-B1-4 (b)'s 127/128 case is then not needed.

(c) EA-B1-3 (a): in own_soldier's doc (client lib.rs:347-348), replace 'A newer tick that is incomplete, refused or the sentinel changes nothing.' with 'A newer tick that is incomplete, refused or the sentinel changes nothing while the complete tick stays in the ring (ACK_RING_TICKS ticks); a newer tick that takes its ring slot drops it, and then the newest complete tick still held is read, else `None`.'

(d) RR-B1-3's doc line: at the end of the doc above SOLDIER_FLAGS (client lib.rs:244-246), add: 'npt-bots' sim-only test pins the same two literals against npt-sim's real deaths (its `soldiers()`: the bit set exactly when health is 0), so a change there fails that test; both copies change together until npt-sim exposes them (BRAWL-1 round 1, EA-B1-1).'
- **Evidence:** Both client tests pass, and the wrap test is named in the counts. Plants P3 and P4 (FP-7) make each test fail at its new assert.

### Step 7: 7. FP-5: doc words in npt-bots' library (no code)

- **Findings:** NA-B1-2, RR-B1-2
- **Words:** (a) NA-B1-2 (a), bots lib.rs:57-59: replace '..., 4 m beyond it.' with '..., 4 m beyond it as geometry. A soldier overshoots the outer radius, because the leash reads a decoded position some steps old and a skipped tick re-runs the last command: at BRAWL-1's review the sim-only test's skip pattern reached 1,720 cm and B1's bursts 1,832 cm, past the door entry (it prints the largest distance per pattern; BRAWL-1 round 1, NA-B1-2).'

(b) NA-B1-2 (b), :245-246: replace 'so a re-run command or a step cut short is corrected at the next command' with 'so a re-run command or a step cut short is corrected once the decoded position shows it (lead - 1 steps later, plus the ticks between the newest and the newest complete tick)'.

(c) RR-B1-2, by its documented alternative, with no code change:
- toward_centre's doc (:125-127) adds: 'Its inputs are decoded grid positions (16 bits on the wire: |x|, |y| at most 32,768, npt-proto quant.rs:197-215), so x² + y² stays below 2^31 and the i64 arithmetic cannot overflow; it is not for other inputs.'
- leash's doc adds: '(x and y in toward_centre's domain)'.
The u64 rewrite is not taken. It would edit library code on the recording path for an input that cannot arrive.
- **Evidence:** fmt clean. The full test's figures are unchanged (FP-8 (4)) and the pins are reproduced (FP-8 (2), (3)). Together these show the library behaves as before.

### Step 8: 8. FP-6: small items in scenario.rs and hash.rs

- **Findings:** EA-B1-2, RR-B1-7, RR-B1-4, ADV-PT-L2
- **Words:** (a) EA-B1-2: at scenario.rs:397, `pub(crate) fn new(room: Room<Sim>, quant: Quant, rtt: RttSource) -> Server {` becomes `fn new(room: Room<Sim>, quant: Quant, rtt: RttSource) -> Server {`. Its one caller is Server::build (:390); I grepped crates/ and check.mjs.

(b) RR-B1-7 (2): rewrap Server::build's doc (:368-372) so that no line holds a lone fragment: '... behind one decoder per session on `bites`' `Quant`; `bites` plants its World bite (armed at `arm_tick`), its room bite and its pump bite. `run_leg` passes ...'. The words do not change.

(c) RR-B1-4. The code order is kept, because a line that breaks both bounds is refused either way.
- Reword :1433-1434 to: '// advisor-u4r-r2, RR4-L2, after the run-length 64-bit checks (which keep their messages): the run's last tick, counted from its first (the warm-up, measured and after phases' ticks), fits 32 bits, on every profile. The burst block's own 64-bit check comes below it, then its 32-bit cap.'
- Reword :2576-2577 to: '// u32::MAX is odd, so never a whole-second length; the largest below it (4,294,967,280 ticks) accepted.'
- After the after-max accepted case (:2584-2587), add two cases. First: `assert!(parsed("B1", "--bursts 2 --burst-first-s 0 --burst-every-s 143165566").is_ok(), "the last burst at 4,294,967,280 ticks");`. Second: `parsed("B1", "--bursts 2 --burst-first-s 0 --burst-every-s 143165567")` is refused with exactly '--bursts 2, --burst-first-s 0 and --burst-every-s 143165567: the last burst's tick, 4294967310 ticks from the run's first, is above 4294967295 (32 bits)'.
The arithmetic: (1 x 143,165,566 + 0) x 30 + 300 = 4,294,967,280, and burst_starts has no in-run check. fp3's doc (:2508) then stays true.

(d) ADV-PT-L2, at hash.rs:1560-1562:
- '360000' becomes '359993' in the replace.
- `let at = 360_000 / 12;` becomes `let at = 359_993 / 12;` (29,999).
- The comment reads: 'At the rule's boundary below RR4-L1's 36,000-tick cap (advisor-u4r-r2): 359,993 bytes hold at most 29,999 ticks; not a multiple of 12, so the rule's rounding down is checked (ADV-PT-L2).'
- **Evidence:** fp3 and r10_carry_82 pass in both builds. Clippy is clean: narrowing Server::new raises no dead-code or private-link warning.

### Step 9: 9. FP-7: the plants

- **Findings:** TE-B1-1, TE-B1-2, RR-B1-1, TE-B1-4, NA-B1-1, RR-B1-5, EA-B1-3
- **Words:** Each plant: the file's sha256 before, the plant by Edit, the sha256 planted, the runs, the restore by Edit, and the sha256 again, equal to before. All runs go through u4r-run.sh -x -k, so every command runs even after one fails.

P1, TE-B1-1 (the leash off): make `return walk;` the first line of `fn leash`'s body (bots lib.rs:249). Run in one call, in final-regen-tests.log's forms (:27, :48, :69):
(1) `cargo test --locked -p npt-bots --lib -- tests::brawl1_sim_short_no_soldier_enters_a_building --exact --nocapture`
(2) the same with `--all-features`
(3) `cargo test --locked --release -p npt-bots --lib -- tests::brawl1_sim_full_no_soldier_enters_a_building_seeds_1_to_3 --ignored --exact --nocapture`
The bite rule: the short copy fails in both builds, and the full test fails with Skips listed for at least one seed. Record EveryTick's figures as they come. If it never bites, the record says so: it is the closed-circle baseline.
If the bite rule does not hold on area: add the door-entry bar below to both barred patterns of both tests, and run the same call under the plant again. The rule must then hold. If it still does not, that item stops and its record comes to me whole; the rest of the pass goes on.
The bar:
- In the tests module: `/// The nearest door entry, cm from the centre: a soldier's centre enters building 0 at x 1,800 (centre x 2,200 minus ROOM 400) with |y| < 60 (npt-sim greybox.rs:64-70, :92-99). const DOOR_ENTRY_CM: i64 = 1_800;`
- In the short copy, after the area assert: `assert!(r.far.0 < DOOR_ENTRY_CM.pow(2), "{pattern:?}: largest distance {} cm, at or past the door entry", r.far.0.isqrt());`
- In the full test, a barred pattern with `r.far.0 >= DOOR_ENTRY_CM.pow(2)` is pushed to `barred` too.
Round 1's largest barred distance is 1,720 cm, so the bar holds with the leash on. This bounded fallback replaces TE-B1-1's open 'lengthen or harden until it does'.

P2, the figure (TE-B1-2 = RR-B1-1): in room_tick's building block (logs.rs:586-588), make the filter `.filter(|_| false)`. Run `cargo test --locked -p npt-lab --bin npt-lab -- logs::tests::<FP-3's test> --exact --nocapture`. It must fail at its first figure assert.

P3, the Hash filter (TE-B1-4 (a)): remove `&& matches!(t.filed, Filed::Hash(_))` from own_soldier (client lib.rs:356).
P4, the wrap rule (TE-B1-4 (b) = NA-B1-1 = RR-B1-5 = EA-B1-3 (b)): replace the `.reduce(...)` (:357-363) with `.max_by_key(|t| t.tick)`.
For each of P3 and P4, run `cargo test --locked -p npt-client --test endpoint -- brawl1 --nocapture`. P3 must fail at 'a complete tick turned sentinel is not read'; P4 at the wrap test's last assert.
- **Evidence:** For each plant, in brawl1-build.md's new section: the file, its three sha256 values, the planted line, the log path (.gate/u4r/brawl1/fp-plant-<n>.log), each command's exit code, and the failing assert's message. For P1 also, per seed and pattern: in_building, first, and the largest distance with its slot, tick, x and y; and whether the door-entry bar was needed. A plant among P2 to P4 that does not bite as ruled stops that item, and its record comes to me whole.

### Step 10: 10. FP-8: the end checks (after every edit above)

- **Findings:** TE-B1-1, TE-B1-2, RR-B1-1
- **Words:** Before its first edit, the copy records the sha256 of crates/lab/det-recordings.json and crates/lab/precheck-recordings.json. On the final code, through u4r-run.sh -x, df first, with each log carrying FP-0's diff line:
(1) check.mjs once: all 11 steps PASS. Counts as deltas from 535/613, each new test named with its file. Expected: +3 in each build (the bots area test, the logs.rs figure test, the client wrap test).
(2) `uptime`, then `node ops/runs/det-u4.mjs`, in one call, with no other lane building or running (condition 8). It must PASS and reproduce SS-P2-s1 1ec25de7c9ce8ea869ed9a8b343746ba131593e1de5c65bc9c1c626ce01cc76d and ES-P3-s2 6abf7643982b5970a630c97daa4d0815ceb2022958b18e18116397dcad9c893e. The non-cargo sum must stay under 240 s; over it, stop, and it comes to me whole.
(3) The precheck regeneration as final-regen-tests.log:18 ran it. It must print 'regenerated SS-P0-s4: 16950444 bytes, sha256 1f00d2d5f70a2c7b6836ede611b5f1c281319a5312060ead91e016da3c0b1b42, as the manifest's'.
(4) The short copy in both builds, with each build's time (at most 60 s), and the full test by name in release. Every figure round 1 printed must equal final-regen-tests.log:41-42, :62-63 and :83-91: in a building, first, the largest distance with its slot, tick, x and y, skipped, the chances and rate for Skips, the longest streak, and stuck. Only the dead figure and the reworded skip text are new. Any other difference means behaviour moved: stop, and it comes to me whole.
(5) Both manifests keep the sha256 recorded before the first edit.
(6) The size, by numstat against 352a245, SERIES.md included.
A det-u4.mjs or regeneration mismatch: stop. No re-pin and no retry; the log comes to me whole, with the first differing tick.
- **Evidence:** final-fp-check.log, final-fp-det.log and final-fp-regen-tests.log under .gate/u4r/brawl1/, each with its diff line. The figures go in the notes and the report.

### Step 11: 11. FP-9: report words, no code (in brawl1-build.md's new section and the copy's report, on disk before round 2)

- **Findings:** TE-B1-5, TE-B1-8, ADV-BP-3, ADV-BP-4, RR-B1-6, EA-B1-5, ADV-PT-M1, NA-B1-2
- **Words:** 1. The plant records P1 to P4, and whether the door-entry bar was added, and why.
2. Pattern (iii): withdraw 'a fixed radius stops it' and the 14 m vehicle. sim_run runs with stand-ins off (bots lib.rs:496). No solid lies within 1.6 m of (1734, -592), (1671, -751) or (1586, -917): building 0's corner (1770, -430) is 166 cm from the first point, the crates lie within 950 cm of the centre, and the wall stand-in is at y 1,180-1,220. The reading for U4g is this ruling's U4g words.
3. TE-B1-8: 'No test drives Behaviour::step with an own soldier past the outer radius. The glue at bots lib.rs:223 is shown only by the moved pins (own_soldier returns Some and changes commands) and by the two 10-minute evidence runs (condition 14).'
4. My corrections, in my words, beside ADV-BP-1 and ADV-BP-2: ADV-BP-3 and ADV-BP-4 as the points ruling words them.
5. RR4-L1's constant: MAX_ENTRY_TICK_COUNT stands (points ruling, Point 1). RR-B1-6 and EA-B1-5 are answered there, and ADV-PT-L3 carries the point to the check.mjs follow-up.
6. The follow-ups with their homes, as listed under 'Each finding': RR-B1-7 (1); EA-B1-1's pub constants, with NA-B1-3's npt-client half; ADV-PT-L3; TE-B1-8's optional glue test.
- **Evidence:** The section is on disk before the main session cuts round 2's snapshot.

## The pins

No ruled fix can move the three re-pinned recordings' bytes: SS-P0-s4 1f00d2d5..., SS-P2-s1 1ec25de7..., ES-P3-s2 6abf7643....

Why: the recordings come from `npt-lab scenario --record`, which writes no logs. The code on that path is npt-bots' library (Behaviour::step, next_command, leash, toward_centre), npt-client's decoding and own_soldier, the lab's run loop, the room and the sim. On that path the fix pass changes only two things:
- doc comments: bots lib.rs:57-59, :125-127 and :245-246; client lib.rs:244-246 and :347-348;
- Server::new's visibility (scenario.rs:397).
Everything else is test code (the #[cfg(test)] modules, client/tests/endpoint.rs, fp3, r10_carry_82), the logs.rs module doc, or comments. room_tick's code does not change in this pass (RR-B1-7 (1) is deferred), and it runs only on logged runs anyway (scenario.rs:1022-1026). So no bot decision and no recorded byte can change.

Proof at the copy's end (FP-8):
- det-u4.mjs (uptime first, condition 8) reproduces SS-P2-s1 1ec25de7c9ce8ea869ed9a8b343746ba131593e1de5c65bc9c1c626ce01cc76d and ES-P3-s2 6abf7643982b5970a630c97daa4d0815ceb2022958b18e18116397dcad9c893e.
- The precheck regeneration reproduces SS-P0-s4 1f00d2d5f70a2c7b6836ede611b5f1c281319a5312060ead91e016da3c0b1b42, 'as the manifest's'.
- Both manifests keep the sha256 they had at the snapshot.
- The full sim-only test's round-1 figures are equal. That test drives next_command directly, so equal figures show the decision function is unchanged.

Proof at the merge's counted run (condition 9, M3): the same det-u4.mjs (uptime first) and the same precheck regeneration, on the merged tree, reproducing the same three pins, plus the full test by name in release.

Any mismatch at either point: stop, and it comes to me whole. No retry and no re-pin (condition 7).

## Round 2

Round 2 is the second and last round, unless a Critical or High is open after it. Two seats, at effort xhigh (a netcode lane, as condition 11 set), in one batch: test-engineer and rust-reviewer.

The subject is only what changed:
- the diff from snapshot ee2973aa to the fix pass's new snapshot, cut by script as round 1's packet was;
- the plant records and logs, P1 to P4;
- FP-8's end-check logs;
- FP-0's two runner diffs;
- the copy's report section.
Each seat confirms its own round-1 findings fixed as ruled, or deferred as ruled, with file:line, and reviews the changed lines. Nothing unchanged is reviewed again.

Who reads what:
- test-engineer:
  - TE-B1-1's 48-record assert, and P1's record (the bite rule, and the door-entry bar if it was added);
  - FP-3's figure test with P2, and the bots area test;
  - FP-2's unit-test edits (TE-B1-3), against my re-computed command sequence;
  - FP-4's two client cases with P3 and P4: its own TE-B1-4, plus NA-B1-1, RR-B1-5 and EA-B1-3 (b);
  - the dead-bit tie and its count (TE-B1-6, with NA-B1-3 and RR-B1-3);
  - RR-B1-4's two fp3 cases, and ADV-PT-L2;
  - FP-8's counts, the full test's equal figures, and the pins.
- rust-reviewer:
  - RR-B1-1, through FP-3's test;
  - RR-B1-2's doc route, and RR-B1-3's tie;
  - RR-B1-4's rewords and cases;
  - RR-B1-5, through the wrap test;
  - RR-B1-7 (2), and EA-B1-2's narrowing;
  - fmt and clippy in both builds;
  - that the library diff on the recording path holds only doc comments and the one visibility change.

netcode-architect sits out. Its three Lows are covered elsewhere: the wrap case is a test (test-engineer reads it); NA-B1-2 (a) and (b) are doc words I give exactly (the main session checks them against this ruling and puts file:line on the card); the tie is test-engineer's. No sim, room, protocol or recording behaviour changes, and FP-8 proves the pins.

engine-architect sits out. No pub item, dependency or seam is added, and EA-B1-2 narrows one. The main session checks the doc words of EA-B1-3 (a), EA-B1-4 and RR-B1-7 (2) itself.

Either seat rejoins if the new snapshot adds a pub item, a dependency or a seam change, or if any library behaviour on the recording path changes. FP-8 makes that last case a stop anyway.

The budget: a round-2 finding below High becomes a STATUS follow-up named in the report; a Low never triggers a push. A Critical or High opens round 3 for that finding alone, and one still open after round 3 stops the change set for Mustafa. Not rounds: the gate after every step, and my review of the pushed head.

## The merge step

Condition 9's M-list. The answer to the clean question: yes, with two corrections.

(1) The form. The folder-level `cargo clean --target-dir .gate/shared-target` is not an approved method while the lane lock's file lives in that folder (W1-TARGET item 3 as amended, advisor-u4d2-m9.md:252-257: 'never by a clean without one of those three'). The approved cleans are the part-cleans:
- (B): the shared target's release, doc and wasm32 roots;
- (C): its dev profile.
Each is `cargo clean --target-dir <tagged root>` with `--release`, `--doc` or `--profile dev` and its test guards, exactly as .gate/u4r-sets/clean-shared-B.log and clean-shared-C.log ran them on 2026-11-14. (B) alone frees only about 440 MiB and (C) about 6 GiB, so both are needed.

(2) The place: right before the counted run, after round 2's ruling and M1, with no lane and no agent running. Round 2's seats run tests in the builder's copy through the shared target, so the clean must wait for them.

Why the clean comes first: a main-folder build re-makes every workspace crate beside the builder copy's artifacts. Today those take about 1.0 GiB of npt-* deps and 3.2 GiB of incremental data. Starting from about 6 GiB free, that would cross the 4 GiB floor mid-run, and the floor is checked only at the call's start.

The steps:
- M1. The merged bytes equal the reviewed bytes, in U4r's form: every changed file under spikes/ at the merge commit is cmp-equal to the round-2 snapshot's, and the tree fingerprint matches. The base also holds: spikes/ at the merge commit's parent equals 352a245's, and any commit in between touches only .checkpoints/.
- M2. pgrep for cargo, rustc, npt-lab, npt-judge and loader.mjs: all empty, and no agent running. Then df -B1; (B) in one main-run.sh call; df -B1; (C) in one main-run.sh call; df -B1. The guards: `test -f` on the lock file and on both CACHEDIR.TAG files, `test ! -e` on each cleaned folder, then `ls -A`. Every figure goes on the card.
- M3. One counted main-run.sh call on the merged tree:
  - `cat /proc/uptime`;
  - check.mjs, all 11 steps, counts equal to the copy's end figures;
  - `uptime` then det-u4.mjs, in the same call (condition 8; a non-cargo sum over 240 s stops it, and it comes to me whole);
  - the precheck regeneration at 1f00d2d5...;
  - the full sim-only test by name in release (condition 13, at the head that goes to my review), its prints equal to the copy's.
  The header's diff and untracked hashes should read the empty-input sha256. The run builds cold, so its timings are labelled cold (INF-U4R-7's labels). Free space is recorded at its start and end. A pin mismatch: stop, and it comes to me whole, with no retry.
- M4. The gate: `node tools/gate.mjs` (with --no-smoke, and the reason, if Chromium cannot start).
- M5. Push; CI green on the pushed head.
- M6. My review of the pushed head.
- M7. Then the plan's section 7 order. The cold counted run refills about 9 GiB, so (B) and (C) run again first, with df before and after; then the two 10-minute evidence runs; then the re-run sets, each set's results committed before the next.

## U4g's brief

These words go into U4g's brief, and the main session records them on the card and in U4r's report for U4g. They merge my ADV-PT-M1 with NA-B1-2 (c), and TE-B1-5's kinematic bound without its obstacle reading. The readings marked 'computed' were not run.

"BRAWL-1's pattern (iii), B1's bursts, in the sim-only test (printed only, never barred):
- Measured: in the full test (release, seeds 1 to 3, 610 s each), no Soldier record had area != 0. The largest distance from the centre was 1,832 cm in every seed, each at a burst's end, at (1734, -592), (1671, -751) and (1586, -917): three angles, one 1 cm band.
- Not a stop by a solid: the test runs with stand-ins off, and no solid lies within 1.6 m of those points. It is the leash's own reach under a 30-tick re-run. Computed, not run: about 1,830 to 1,840 cm. One reading takes the walk's 205 cm circle kept inside the 1,400 cm outer radius, plus 600 cm of re-run. The other takes a start just inside the 1,300 cm inner radius, plus 30 re-runs at about 18 cm. By kinematics the reach is at most about 2,000 cm: a soldier at the outer radius re-running 30 steps of 20 cm straight out.
- That reach passes the 1,800 cm door-entry radius of buildings 0 and 3, whose doors lie on the x axis (|y| < 60 for a soldier's centre). The door entries of buildings 1, 2, 4 and 5 lie at about 1,864 cm, within only the kinematic bound.
- The test is optimistic after a burst. It feeds each slot its World position from 8 steps earlier at once, while a real client may steer on its last complete pre-burst tick until a complete post-burst tick arrives.
- So 'area 0 in three seeds' is neither a bound nor a margin. A B1 judged run can carry a soldier into a door gap. That lowers -2's everyone-in-view share (bar 0.99), which makes -17's parts invalid (their validity is pre:PM-0001-2): BRAWL-1's INVALID again. The building figure on every B1 run is the check. U4g's plan says how a B1 building entry reads in -17, or measures how often it happens, before its judged B1 runs. One way to measure it is a printed door-gap margin per burst, TE-B1-5's option."

## The size

998 changed lines now (875 inserted and 123 deleted by numstat against 352a245, SERIES.md included). The fix pass adds about 270 changed lines, almost all of them tests:
- the bots tests, about 110;
- logs.rs's control test, about 60;
- the client tests and docs, about 50;
- the bots docs, about 15;
- scenario.rs and hash.rs, about 30.
Expected after the fix pass: about 1,270 (1,200 to 1,340); the conditional door-entry bar adds about 10 more. That is under my 1,400 'well past' line, so no SIZE-CHECK note is due. To keep that margin I deferred RR-B1-7 (1), a polish rewrite of production log code of about 30 lines. If numstat passes 1,400 at any point, the builder stops, and it goes to Mustafa in three lines (what grew, why, and the cheaper option).

## For Mustafa

BRAWL-1, the fix that keeps the test bots out of buildings, passed its first review round with no serious problem. The four reviewers found three medium gaps, all in the proof rather than in the code:
- nothing yet shows that the new 'no bot in a building' test can fail;
- nothing shows that the lab's building counter can count above zero.
I checked every finding against the code myself. One fix pass closes these gaps. A deliberate break of the leash shows that the test catches it. A test that puts a soldier inside a building shows that the counter counts it.

Most small findings ride the same pass. They are tests and comment wording, and nothing a bot does changes, so the three pinned recordings must stay the same byte for byte; the checks prove that. One reviewer misread a figure: it blamed a wall that is not there, and I corrected it. The warning behind that figure still holds and goes to the next unit: during a one-second burst of total packet loss, a bot can still walk into a building.

The change will be about 1,270 lines, under the 1,400-line mark where I would stop and ask you. Disk space holds if the build caches are cleared, by the approved method, just before the final counted run.

Decisions needed: none. Every point here is a technical choice inside the approved plan. KEPT-LOGS stays with you as raised at the plan review; nothing here changes it.

For Mustafa: nothing. This is spike code. Nothing a player sees, hears or feels changes, and it is not a merge to main.

## Each finding

### TE-B1-1 (Medium; fix now)

- **Home:** FP-1 (a) and FP-7 P1 (with the conditional door-entry bar); FP-8 (4)
- **Why:** Confirmed. No log runs the leash off; only the N2-1 plants exist. The sim test never checks that 48 Soldier records are present (World writes one per slot: world.rs:1816-1818). TE's 'lengthen or harden until it does' is open-ended. I replace it with a bounded fallback: a 1,800 cm door-entry distance bar, which holds with the leash on (round 1's barred maximum is 1,720 cm) and must bite under the plant.

### TE-B1-2 (Medium; fix now)

- **Home:** FP-3 (with RR-B1-1, fixed once), FP-1 (c), FP-7 P2
- **Why:** Confirmed by grep: no test makes a soldier's area non-zero, and none calls room_tick or summary_line with one. Its fix as worded would fail on npt-log's writer, which refuses a second record for the same tick (log lib.rs:467-474). So FP-3 uses two room ticks, a warm-up one and then a measured one.

### RR-B1-1 (Medium; fix now)

- **Home:** FP-3 (one test with TE-B1-2), FP-7 P2
- **Why:** The same gap as TE-B1-2, confirmed (logs.rs:584-599; scenario.rs:1612-1690). Its Server::build, script and tick route works: Server.room is pub(crate) (scenario.rs:347), and Room::script and Room::tick apply the teleport (room.rs:617, :805). Its room.jsonl and Warmup asserts are kept. Logs::open needs the folder made first (logs.rs:77-88).

### TE-B1-3 (Low; fix now)

- **Home:** FP-2
- **Why:** Confirmed by my own splitmix64: slot 5 at seed 1 has walk phase 7, so the walk at commands 4 to 7 is [-127, 0], the steer itself. The blocked asserts therefore cannot fail. The None/dead state and the outer boundary are untested. I re-sequenced the inserted cases (commands 12, 15, 16) so each check can fail. Test-only; it rides the push the Mediums need.

### TE-B1-4 (Low; fix now)

- **Home:** FP-4 (a) and (b), FP-7 P3 and P4
- **Why:** Confirmed: ticks 103 and 104 never complete, and the ring order is increasing. (a) The complete-then-sentinel case is added as worded. (b) is covered by the single wrap case: u32::MAX sits in ring slot 31 and 0 in slot 0, so that case also kills the ring-order mutant, and the 127/128 case is not needed.

### TE-B1-5 (Low; recorded)

- **Home:** U4g's brief words (this ruling); FP-9 item 2 (the report's correction)
- **Why:** Its conclusion holds: B1 can reach past the door entry, and area 0 is not evidence. Its mechanism is wrong. sim_run runs with stand-ins off (bots lib.rs:496), and no solid lies within 1.6 m of the three maxima (building 0's corner (1770, -430) is 166 cm from (1734, -592)), so there is no 'obstacle at 18.3 m'. Its kinematic bound of about 2,000 cm goes into the brief. No code change; the door-gap print is left as U4g's option.

### TE-B1-6 (Low; fix now)

- **Home:** FP-1 (b): the dead-bit tie in soldiers()
- **Why:** Confirmed: the literals are at client lib.rs:247-248 and bots lib.rs:479, and npt-sim's are private (world.rs:92, :120). Fixed once with NA-B1-3, RR-B1-3 and EA-B1-1. The per-record assert `dead == (health == 0)` runs on every tick of sim_run with real deaths (world.rs:1630-1634, :1176-1188), so no separate kill test is needed.

### TE-B1-7 (Low; fix now)

- **Home:** FP-0, by the main session, outside the change set (u4r-run.sh:66 and main-run.sh:41, with .diff files and bash -n)
- **Why:** Confirmed: the runner headers record only HEAD. From spikes/netcode-proof, `git diff HEAD -- .` equals the whole diff here, so the logged hash equals the snapshot sha256. This makes copy 2's logs and the counted run carry their exact code state.

### TE-B1-8 (Low; recorded)

- **Home:** FP-9 item 3 (report words); an optional glue test at U6's bots'-view item (EA-U4R-5's home)
- **Why:** Confirmed: no test drives Behaviour::step with an own soldier past the outer radius. The moved pins show only that own_soldier returns Some and changes commands. The fix was offered as optional, and the 10-minute runs (condition 14) are the integration proof. The report states this.

### NA-B1-1 (Low; fix now)

- **Home:** FP-4 (b), FP-7 P4
- **Why:** Confirmed: only ticks 100 to 106 are fed. Its wrap case is taken as the single case for TE-B1-4 (b), RR-B1-5 and EA-B1-3 (b). It kills both the plain-maximum and the ring-order mutants.

### NA-B1-2 (Low; fix now)

- **Home:** FP-5 (a) and (b) (doc words); (c) in U4g's brief words (this ruling)
- **Why:** Confirmed against final-regen-tests.log:84-91 (Skips at seed 2: 1,720 cm; Bursts: 1,832 cm in all three seeds) and its reading of the post-burst stale view. Doc-only, with no pin effect. Its (c) is merged into the U4g words with ADV-PT-M1.

### NA-B1-3 (Low; fix now)

- **Home:** FP-1 (b) (the bots tie with the dead count printed); its npt-client half goes to EA-B1-1's follow-up
- **Why:** Confirmed. The same fix as TE-B1-6, RR-B1-3 and EA-B1-1: its assert and the printed count are taken as worded. The client-side tie waits until npt-sim exposes the flag.

### RR-B1-2 (Low; fix now)

- **Home:** FP-5 (c): its documented alternative (domain stated), no code change
- **Why:** Confirmed but unreachable: decoded |x| and |y| are at most 32,768 (quant.rs:197-215), so x² + y² stays below 2^31. The u64 rewrite would edit library code on the recording path for no reachable input, so the doc route is taken.

### RR-B1-3 (Low; fix now)

- **Home:** FP-1 (b) and FP-4 (d) (the doc line on the client's constants)
- **Why:** Confirmed. The same fix as TE-B1-6: its assert, the dead count and the doc line naming the tie are all taken.

### RR-B1-4 (Low; fix now)

- **Home:** FP-6 (c): comments reworded, two fp3 cases added, code order kept
- **Why:** Confirmed: the run's 32-bit cap (scenario.rs:1435) comes before the burst block's 64-bit check (:1507-1515). Both lines are refused either way, so no wrong line is accepted. :2576's comment is false (u32::MAX is odd). I checked its case arithmetic myself (4,294,967,280 accepted, 4,294,967,310 refused). Test and comments only.

### RR-B1-5 (Low; fix now)

- **Home:** FP-4 (b), FP-7 P4
- **Why:** Confirmed. The same gap as NA-B1-1; one wrap case closes it.

### RR-B1-6 (Low; no action)

- **Home:** Answered by my points ruling (Point 1, ADV-BP-4); the check.mjs side is ADV-PT-L3's follow-up
- **Why:** The rename was ruled before this round: MAX_ENTRY_TICK_COUNT stands, because the ruled name fails check.mjs and the cap is a size bound, not a time. ADV-PT-L3 adds the name to NOT_TICK_COUNTS and _TICK_COUNT to TICK_NAMED at check.mjs's next change. Restoring the ruled name was refused there.

### RR-B1-7 (Low; fix now)

- **Home:** (2), the doc rewrap: FP-6 (b). (1), the loop rewrite: a follow-up for the next lane that edits logs.rs's room_tick
- **Why:** Confirmed: the `s as u8` cast after the i64 round trip, and the broken doc line. Soldier indices are below 64, so nothing truncates. (1) is a polish rewrite of production log code that round 2 would have to re-verify. I defer it to keep the size margin and to keep the fix pass free of library behaviour edits. (2) is doc-only.

### EA-B1-1 (Low; fix now)

- **Home:** Now: FP-1 (b) and FP-4 (d) (the tie and the doc line). Later: npt-sim exposes the flags index and the dead bit as pub items, in the next lane that opens crates/sim (beside EA-U4R-4), U6 at the latest
- **Why:** Confirmed (client lib.rs:246 says 'private there'). The bots tie closes today's drift risk without opening npt-sim. The pub items need a lane that edits crates/sim, so that part is a follow-up. The client-side cfg(test) variant is not needed now.

### EA-B1-2 (Low; fix now)

- **Home:** FP-6 (a)
- **Why:** Confirmed by grep: Server::new's only caller is scenario.rs:390, inside build, so making it private changes nothing else. It enforces EA-U4R-2's single constructor.

### EA-B1-3 (Low; fix now)

- **Home:** (a) FP-4 (c), doc words; (b) FP-4 (b) and FP-7 P4
- **Why:** Confirmed: slot_for resets a slot when a newer tick takes it (client lib.rs:564-574), so the doc claims more than the code does. The wrap gap is the same as NA-B1-1's.

### EA-B1-4 (Low; fix now)

- **Home:** FP-3 (EA-B1-4's words at logs.rs:6-7)
- **Why:** Confirmed: the module doc's 'all its counters, 0 included' conflicts with the list written only when non-empty (logs.rs:597-598; SERIES.md's row). Doc-only, in the file FP-3 opens.

### EA-B1-5 (Low; no action)

- **Home:** As RR-B1-6: my points ruling (Point 1); ADV-PT-L3's follow-up
- **Why:** The same item as RR-B1-6, already ruled. Its alternative (rename back and register the name) was refused at Point 1, in favour of ADV-PT-L3's form.

### ADV-PT-M1 (Medium; recorded)

- **Home:** U4g's brief words (this ruling), merged with NA-B1-2 (c) and TE-B1-5's bound
- **Why:** My finding, unchanged in substance: B1's 1,832 cm is the leash's reach, not a stop, and it passes the 1,800 cm door entry. The mechanism is labelled as computed, with the readings side by side. The B1 pattern is printed only by condition 3 (iii), so the diff does not change.

### ADV-PT-L1 (Low; fix now)

- **Home:** FP-1 (d)
- **Why:** It rides the push the Mediums need, as my points ruling said it would. The Bursts line's 'of 0 chances (0 per 1000)' goes into U4g's brief and means nothing.

### ADV-PT-L2 (Low; fix now)

- **Home:** FP-6 (d)
- **Why:** It rides the push the Mediums need. With 359,993 bytes the rule's boundary sits at 29,999 ticks and is not a multiple of 12, so a div_ceil plant is caught again.

### ADV-PT-L3 (Low; follow-up)

- **Home:** The check.mjs follow-up (check.mjs's next change; before PR #40 is marked ready at the latest)
- **Why:** Unchanged since my points ruling. It needs a check.mjs edit, which is outside this lane and would pull in the whole check.mjs follow-up.

### ADV-BP-3 (Low; recorded)

- **Home:** FP-9 item 4 (the report, in my words)
- **Why:** My wording error in condition 6 (ii); nothing to redo. Recorded beside ADV-BP-1 and ADV-BP-2.

### ADV-BP-4 (Low; recorded)

- **Home:** FP-9 item 4 (the report, in my words)
- **Why:** My error: RR4-L1's ruled name fails check.mjs's TICK_NAMED rule. MAX_ENTRY_TICK_COUNT stands.
````

### Round 1's findings, the four seats' results word for word (brawl1-review-r1-full.md)
_`.checkpoints/netcode-proof-test/brawl1-review-r1-full.md` lines 1-224 (sha256 fc979c632a76), as they are:_

````text
# BRAWL-1 review round 1: the four seats' results (workflow wf_bbfeece0-4c7), 2026-11-15

Written by script, word for word, from the workflow's journal. Snapshot reviewed: wip/brawl1.patch ee2973aa...; packet brawl1-packet.md 16eab462.... Each seat's notes: brawl1-review-<seat>.md. Nothing below is the main session's.

## review-test (APPROVE WITH FINDINGS; snapshot ee2973aa8ad8)

**Seat:** test-engineer (BRAWL-1 round 1, condition 11's panel, batch seat; effort xhigh)

**Its snapshot field, whole:** ee2973aa8ad8c453b5720099c27b37cb9980820da74f33ad9c78ddd7ad591dc7 (wip/brawl1.patch; the read copy .claude/worktrees/brawl1 gives the same value for `git diff HEAD | sha256sum`; packet 16eab462c4e0fcbcd8360aad7a951207275927c7f0f6813b794df54c12f4b1a4). No Critical or High finding.

What I checked against the logs, and it holds:
- Proof (a): p1-a-check2.log, all 11 steps, 532 and 609+1=610 tests.
- Proof (b): p1-b-det.log, uptime first, old det pins.
- Proof (c): p1-cd.log, SS-P0-s4 6933b56a "as the manifest's".
- Proof (d): my own diff of p1-cd.log:38-150 against results/u4r-precheck-30/faf42aeb034c/runs.txt:17-125. Only the build line differs.
- Proof (e): my own normalised diff of before-a/b against after-e/e2. Only two kinds of line differ: the wallclock bite's L1 hash, which already differs between the two runs on the unchanged base, and the filtered-out count (+1 test). The builder disclosed this as PROOF-E.
- The two N2-1 plants: p1-plant-leave.log fails both cases on exactly the two ruled names. p1-plant-roster.log fails both on the new check alone. Both plants are restored (scenario.rs:533; room.rs is not in the diff).
- fp3: every ruled text is exact, including condition 12's full digits. r10_fp3: 36,001 is refused with the exact ruled text, ahead of the runner check.
- Re-pin: only the three sha256 values change. final-det.log reproduces both det pins (non-cargo 214.5 s, uptime first). final-regen-tests.log reproduces 1f00d2d5.

My own runs at this snapshot (shared mode, not counted):
- .gate/u4r/brawl1-review-test/t1-tests.log, exit 0: bots unit test and short copy pass, client test passes, 8 lab tests pass.
- .gate/u4r/brawl1-review-test/t2-full-release.log, exit 0: the full test passes in release in 24.23 s, and all nine prints are identical to the builder's final log.

Notes: .checkpoints/netcode-proof-test/brawl1-review-test.md.

### TE-B1-1 (Medium)

- **Where:** spikes/netcode-proof/crates/bots/src/lib.rs:633-640 (brawl1_sim_short_no_soldier_enters_a_building), :644-659 (the full test), :495 (sim_run)
- **Wrong:** Nothing shows that the sim-only test can fail. No run with the leash removed exists. The short copy is the guard condition 13 puts in every check.mjs run. Its EveryTick half almost certainly cannot fail in 120 s even with no leash: with a command every tick, the drift comes only from cut steps. In U4r's real P0 seed-1 run, the first soldier got into a building 458 s into the measured window, and seeds 2 and 3 had none in 10 minutes. The Skips half has not been shown to bite either. The sim loop also never asserts that it sees 48 Soldier records. A future World that spawns soldiers only on join would make in_building 0 trivially. Today only the far and stuck prints show the soldiers exist and move.
- **Evidence:** brawl1-build.md and every log list plants for N2-1 only (p1-plant-leave.log, p1-plant-roster.log). No p2-* log runs a leash-off build. Advisor sets-r2 Q1 (b): onset at 458 s at P0 s1; s2 and s3 had no onset. My run t1-tests.log reproduces 0 soldier-ticks in both short patterns. That is a pass, not a bite.
- **Fix:** Run a plant by Edit, record its sha256, then restore and check the sha256 again. In bots/src/lib.rs:248, make the first line of `fn leash` `return walk;`. Then run through u4r-run.sh -x: `cargo test --locked -p npt-bots --lib -- tests::brawl1_sim_short_no_soldier_enters_a_building --exact --nocapture` in both builds, and `cargo test --locked --release -p npt-bots --lib -- tests::brawl1_sim_full_no_soldier_enters_a_building_seeds_1_to_3 --ignored --exact --nocapture`. Record the result: each barred pattern must read in_building > 0 in at least one seed of the full test, and the Skips pattern in the short copy. If a pattern never bites, say so in the record, and lengthen or harden the short copy until it does. Also add `assert_eq!(now.len(), usize::from(scene.slots()), "tick {t}: every slot's Soldier record");` after `let now = soldiers(&world);` (:519).

### TE-B1-2 (Medium)

- **Where:** spikes/netcode-proof/crates/lab/src/logs.rs:584-599 (the figure in room_tick); lab/src/scenario.rs:1627 and :1673-1686 (the summary line); bots/src/lib.rs:522 (the sim test's predicate)
- **Wrong:** Both new bars count `area != 0` on Soldier records. No test anywhere shows a soldier whose feet are inside a building giving area != 0. No test calls room_tick with such a soldier either, so the figure has never read anything but 0. The same goes for the Counts total, the slot list, the measured-only counting, the any-phase writing and the summary text. The plan makes this figure a stop bar for the two 10-minute runs ('a building total above 0', condition 14) and for every re-run set ('the building figure at 0'). A figure that could only read 0, for example through a filter or index mistake, would pass both bars without anyone noticing. Everyone-in-view is an independent bar and limits the harm, which is why this is Medium and not High.
- **Evidence:** A grep of the read copy for in_building finds only the producer sites and the sim test's `assert_eq!(r.in_building, 0, ...)`. A grep for area tests in crates/sim finds no assertion of a soldier's non-zero area (the only area test is sim/tests/world.rs:890's owner diff). The diff adds no test in logs.rs or scenario.rs for the figure; the only new lab test is rr3_l2_a.
- **Fix:** Add two positive controls in the same change set, before the 10-minute runs. (1) In the bots #[cfg(test)] module: `let mut w = World::new(&scene, 1, FIRST); w.step(&TickInput { tick: FIRST, teleports: vec![Placement { slot: 0, position: [2_200, 0, 0] }], ..TickInput::default() }, &history);` then assert that `soldiers(&w)` gives slot 0 area 1 (building 0's centre, greybox.rs:93; footprint x 1,800-2,600). (2) In logs.rs tests: step a Sim the same way, wrap it in `Room::new(...)` as Server::build does, and call `logs.room_tick(&room, t, Phase::Measured, ...)` once and with `Phase::Warmup` once. Assert `counts.in_building_measured == 1` and `in_building_slots == {0}` after the measured call, and no change after the warm-up call. Assert the summary text `in a building: 1 slot-ticks (slots 0)` through summary_line, or through the same format, if the Summary is easy to build.

### TE-B1-3 (Low)

- **Where:** spikes/netcode-proof/crates/bots/src/lib.rs:365-371 (the three 'blocked' asserts), :387-391 (dead, then None, then 1,299), :363 (`steer`), :60 (LEASH_OUT_CM)
- **Wrong:** The unit test's 'blocked' checks cannot fail. Slot 5 at seed 1 has walk phase 7, so the open-loop walk at commands 4 to 7 is MOVES[8] = [-127, 0]. That is exactly the steer expected at (1450, 0). The three blocked asserts fall on commands 4, 5 and 6, so a leash that dropped steering on a step that did not move would pass them. It would also pass the rest of the test, because (1480, 30) at command 7 sets steering again. This is one of the showings the plan's section 4.1 requires. The test also never checks two things the leash's doc claims. One: that None and dead keep the leash's state; every case after dead and None goes to 1,299, which resets the state anyway. Two: the outer boundary; 1,350 walks and 1,450 steers, so LEASH_OUT_CM could be anything from 1,350 to 1,449 and the test would still pass. Condition 1's bounds are pinned nowhere.
- **Evidence:** I recomputed Behaviour::new's splitmix64 for slot 5, seed 1: phase = 7. Walk by command: k 0-3 [-117,49], k 4-7 [-127,0], k 8-11 [-117,-49]. The call order puts the blocked asserts at k 4, 5, 6. toward_centre(1450, 200) = [-125, -17], which is in no MOVES entry.
- **Fix:** Replace the blocked loop with `let blocked = toward_centre(1_450, 200); for _ in 0..3 { let (got, walk) = give(at(1_450, 200)); assert_ne!(walk, blocked, "the twin's walk differs, so the check can fail"); assert_eq!(got, blocked, "blocked: the position does not move"); }`. Insert `assert_eq!(give(at(1_350, 0)).0, steer, "None and dead keep the leash's state");` right after the None case, with an `assert_ne!` on the twin's walk there too. Add `(at(1_400, 0), walk)` and `(at(1_401, 0), steer)` cases. Add `const _: () = assert!(LEASH_IN_CM >= 1_300 && LEASH_OUT_CM >= LEASH_IN_CM + 100 && LEASH_OUT_CM <= 1_500, "condition 1's bounds");`.

### TE-B1-4 (Low)

- **Where:** spikes/netcode-proof/crates/client/tests/endpoint.rs:397-441 (brawl1_own_soldier_...); client/src/lib.rs:350-369 (own_soldier)
- **Wrong:** Condition 5 says the item reads 'the newest tick that is complete and filed Filed::Hash, with ticks compared by the endpoint's wrap rule'. The test leaves both halves of that unproven. (1) The newer 'refused' and 'sentinel' ticks (103, 104) are never complete (got stays 0), so `complete()` alone already excludes them. A version without the `matches!(t.filed, Filed::Hash(_))` filter would pass, and the case that needs that filter is never built: a tick that is complete but later turned sentinel by a duplicate group with different bytes (lib.rs:725-731, carry 61 (b)). (2) Ticks 100 to 106 sit in ring slots 4 to 10, in increasing order (ring 32, slot t % 32). So taking the last matching ring entry in iteration order, or comparing ticks with a plain `>`, would pass in place of the wrap-rule reduce.
- **Evidence:** ACK_RING_TICKS = 32 at 30 Hz (sim/src/ticks.rs:65-70). The ring is indexed by t % ACK_RING_TICKS (client lib.rs:565). sentinel() sets only `filed` and leaves got alone (lib.rs:626-637). The 103 baseline refusal and the 104 decode failure both return before any group bit is set (lib.rs:740-760).
- **Fix:** Add two cases to the test. (a) After tick 105 is filed, feed a second datagram for tick 105 group 0 with different bytes (as the carry-61 test does). Assert `out.hashes == [(105, 0)]` and `e.own_soldier() == first`, the older tick 101 record. (b) Feed complete ticks 127 (slot 31) and 128 (slot 0), each with a different own record. Assert own_soldier gives tick 128's record, which kills the ring-order mutant.

### TE-B1-5 (Low)

- **Where:** spikes/netcode-proof/crates/bots/src/lib.rs:509-515 and :557 (Pattern::Bursts); brawl1-build.md:338-342 and brawl1-build-report.md:117-121 (the reading for U4g's brief)
- **Wrong:** Pattern (iii)'s 'no area != 0' is recorded for U4g's brief as the B1 result, but it is not evidence that the leash holds under B1. In all three seeds the largest distance is 1,832 cm, at three different angles: (1734,-592), (1671,-751), (1586,-917). That is one radius to the cm, each reached at a burst's end. Something other than the leash stopped the soldier there; the builder's candidate is the 14 m fight vehicle's outer parts, and the builder did not check it. By kinematics alone, a 1 s burst repeats 30 steps of 20 cm (SS move speed 600 cm/s, fire held, so no sprint). That is 600 cm past a starting point as far out as R_OUT 1,400 cm: about 2,000 cm. Every door entry is closer than that: buildings 0 and 3 at 1,800 cm, buildings 1, 2, 4 and 5 at about 1,864 cm (greybox.rs:92-99, ROOM 400). Within condition 1's floor (R_OUT >= 1,400), the leash cannot keep a B1 burst out of a door that lies in the walk's direction. Seeds 1 to 3 simply did not line up with a door. BP-3 named the general risk. What is new is the reading of the printed data.
- **Evidence:** My t2-full-release.log matches final-regen-tests.log: Bursts largest distance 1,832 cm in seeds 1 to 3, at the points above. sqrt(1734²+592²) = 1832.3, sqrt(1671²+751²) = 1832.0, sqrt(1586²+917²) = 1832.0. scene.rs:184 gives move_speed 600. world.rs:476-505 gives full speed at |move| about 127, sprint only without fire.
- **Fix:** No code change is required for the merge. In the build notes and U4g's brief, replace 'outside a building' as the takeaway with: 'pattern (iii)'s maximum is limited by an obstacle at about 18.3 m, not by the leash; by kinematics a 1 s burst can carry a soldier to about 2,000 cm, past every door entry (1,800 / about 1,864 cm), so B1 can repeat BRAWL-1 when a walk points at a door'. If U4g wants a measured figure, add one printed-only line to pattern (iii): each burst's smallest distance from any soldier's centre to any door gap's centre line. Use the six door gaps from greybox.rs:92-99 (the centre of the door side, plus or minus 60 cm), so the margin is read directly and not inferred from the radius.

### TE-B1-6 (Low)

- **Where:** spikes/netcode-proof/crates/client/src/lib.rs:247-248 (SOLDIER_FLAGS = 5, SOLDIER_DEAD = 1 << 3); bots/src/lib.rs:479 (the test's own copy `r.grid[5] & (1 << 3)`); client/tests/endpoint.rs (`dead.grid[5] = 1 << 3`)
- **Wrong:** The dead bit's index and value are copied by hand into three places from npt-sim's private SOLDIER_FLAGS (world.rs:92) and FLAG_DEAD (world.rs:120). Each test writes its own copy of the same literal, so no test would notice if npt-sim's layout or flag bits moved. The leash would then either steer dead soldiers or never steer live ones whose flags carry a set bit 3. Correct today: world.rs:120 FLAG_DEAD = 1 << 3, seam.rs:249 SOLDIER[5] = FLAGS.
- **Evidence:** A grep shows the copies at the lines above. No test kills a soldier and reads its record's flags.
- **Fix:** In the bots #[cfg(test)] module, which already names World, add a test. Step SS (seed 3, as logs.rs p6 does, which reaches kills: 'kills 2' in 600 ticks per my t1-tests.log) until SimCore::step reports a kill. Then assert that the killed slot's Soldier record has `grid[5] & (1 << 3) != 0` and that every live slot's record has it clear. That ties the copy to npt-sim's behaviour. Name the client's constants in that test's doc as the ones it pins.

### TE-B1-7 (Low)

- **Where:** .checkpoints/netcode-proof-test/u4r-run.sh lines 64-67 (the header the runner writes); .gate/u4r/brawl1/p1-*.log and final-*.log
- **Wrong:** Every log records only 'HEAD 352a245', and the whole change is uncommitted. So the logs cannot show that proofs (a) to (e) ran on one code state, or that the full test ran 'at the head that goes to review' (condition 13). Both rest on the builder's notes. My own runs close condition 13's gap for this snapshot, because the full test's nine prints are identical. They cannot close it after the fact for Part 1's intermediate state.
- **Evidence:** p1-a-check2.log, p1-b-det.log, p1-cd.log and p1-after-e2.log each start 'HEAD 352a245' with no tree or diff hash. brawl1-build.md:157-158 says (e) was captured again 'so all five proofs stand on one code state'.
- **Fix:** In the runner's header block, after the HEAD line, add `echo "diff $(git diff HEAD -- . | sha256sum | cut -c1-16); untracked $(git status --porcelain --untracked-files=all -- . | sha256sum | cut -c1-16)"`. This is a main-session edit of the lane's runner, not of this change set. Use it for the merge step's counted run and later lanes. For BRAWL-1, the merge step's M-list (condition 9) already re-runs everything on the merged bytes.

### TE-B1-8 (Low)

- **Where:** spikes/netcode-proof/crates/bots/src/lib.rs:223 (step passes `endpoint.own_soldier()` to next_command)
- **Wrong:** No test covers the glue between the endpoint's decoded state and the leash. The unit test and the sim test call next_command directly, and the client test calls own_soldier. Passing a different value would leave every one of those tests green: the wrong own record (for example, a stale tick's), or a view-derived position. Only the moved pins show that step() passes a Some that changes commands. They could not move if own were always None. The integration proof is the 10-minute runs after the merge.
- **Evidence:** The diff's tests: brawl1_the_leash_... and sim_run call `next_command` (bots lib.rs:353-354, :589), and endpoint.rs:397 calls own_soldier. No test calls `Behaviour::step` with an own soldier past R_OUT.
- **Fix:** Optional, before or with the 10-minute runs: a bots #[cfg(test)] test. Make a ClientEndpoint, feed it JoinAccepted for slot 0 and one complete snapshot whose own section holds soldier 0 at (1450, 0), using the client test's own_only pattern. Call `b.step(&mut endpoint, &[], view, now)`, take the endpoint's next Input datagram from its StepOut and decode it. Assert move_xy == [-127, 0]. Otherwise record in the report that the 10-minute runs are the glue's only proof.

## review-netcode (APPROVE WITH FINDINGS; snapshot ee2973aa8ad8)

**Seat:** netcode-architect (BRAWL-1 round 1, panel per the advisor's condition 11)

**Its snapshot field, whole:** ee2973aa8ad8c453b5720099c27b37cb9980820da74f33ad9c78ddd7ad591dc7 (wip/brawl1.patch; the read copy /home/user/Shamble-Frontlines/.claude/worktrees/brawl1 at 352a245, whose `git diff HEAD` hashes to the same value; packet sha256 16eab462c4e0fcbcd8360aad7a951207275927c7f0f6813b794df54c12f4b1a4). Notes: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/brawl1-review-netcode.md. My run: /home/user/Shamble-Frontlines/.gate/u4r/brawl1-review-netcode/run1.log. On all four focus points I found no Critical, High or Medium. (a) own_soldier reads the newest tick that is used, complete and filed Hash, using the endpoint's wrap compare. The plain-compare bites only affect the counters tick_offset and event_seq, so "tick" compares the same in every build. It reads the decoded integers, so the client-rounding bite does not change it. The own section always holds (Soldier, owner) (room.rs:1064-1078). Under the owner-to-all bite it returns None, so a bot never steers on another soldier's position. (b) The 8-step delay covers P3 (lead 7) with a gap of up to 2 ticks between the newest tick and the newest complete one; the real uncorrected count is lead - 1 + that gap. Pattern (iii) matches B1 within a tick or two (about 31 re-run ticks against its 30). (c) room_tick runs on every logged tick in every phase (scenario.rs:1022-1026). The total counts measured slot-ticks only. The summary prints it through logged(). No end-event field is written, and the one written name has its SERIES.md row. (d) Read side by side, Part 1 changes no behaviour: SimClock starts at 0 and advance_to sets exactly next.max(now+1); wake() runs due_ticks before the admit/gone sinks, and due_ticks depends only on now and the schedule; with_event_start(0) equals Room::new's next_event 0 (room.rs:459, :488-491); Bites::default() gives Quant::default(), the room unchanged and Sim::new. Two diffs I ran myself agree. The clean pair in p1-cd.log equals results/u4r-precheck-30/faf42aeb034c/runs.txt '## pair clean' line for line, apart from the build line. The precheck --nocapture logs before and after Part 1 differ only in build output and timings. Through the runner, at this snapshot, I re-ran the client test, the bots unit and short tests, and the full release sim test: all pass, with the same figures as final-regen-tests.log. One process slip: an early shell command wrote a scratch copy of the reference section to the session scratchpad (/tmp/claude-0/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/scratchpad/ref.txt) by redirect, which breaks my write limits. It is outside the repo, and I redid that diff without writing any file.

### NA-B1-1 (Low)

- **Where:** spikes/netcode-proof/crates/client/tests/endpoint.rs:396-442 (brawl1_own_soldier_is_the_newest_complete_filed_tick_s_own_record); the code it guards: crates/client/src/lib.rs:352-362
- **Wrong:** Condition 5 requires ticks to be compared by the endpoint's wrap rule, and the code does so (`self.wrap.after32("tick", a.tick, b.tick)` inside a reduce). But no test exercises it. Every tick in the test is between 100 and 106, so replacing the reduce with `.max_by_key(|t| t.tick)` would still pass while returning the pre-wrap (stale) position across the 32-bit wrap.
- **Evidence:** In the test, own_only(100..=106, ...) are the only ticks fed (endpoint.rs:409-437). ACK_RING_TICKS is 32 (sim ticks.rs:65-70) and divides 2^32, so ticks u32::MAX (ring slot 31) and 0 (slot 0) can both be held at once. after32 picks 0 there; a plain max picks u32::MAX. Nothing in the patch tests own_soldier across the wrap.
- **Fix:** Add a wrap case, at the end of the brawl1 test or as its own #[test]: `let mut w = endpoint(); let accepted = bytes(&Message::JoinAccepted(JoinAccepted { slot: 0, tick: u32::MAX - 1, ..JoinAccepted::default() })); feed(&mut w, vec![(Channel::Control, accepted, 0)]); feed(&mut w, vec![datagram(own_only(u32::MAX, 1, vec![soldier(0, 11)]))]); let out = feed(&mut w, vec![datagram(own_only(0, 1, vec![soldier(0, 22)]))]); assert_eq!(out.hashes.len(), 1, "tick 0 complete and filed"); assert_eq!(w.own_soldier(), Some(([22, -22, 150], false)), "tick 0 comes after u32::MAX by the endpoint's wrap rule");`. With the max_by_key mutation this case fails (it returns 11).

### NA-B1-2 (Low)

- **Where:** spikes/netcode-proof/crates/bots/src/lib.rs:55-59 (LEASH_OUT_CM's doc: '4 m beyond it'), :243-244 (leash's doc: 'corrected at the next command'), and the pattern (iii) record that goes into the build notes and U4g's brief (condition 3 (iii))
- **Wrong:** The docs give only the geometric margin, and the full test's own figures show far less in practice. (1) The barred Skips pattern overshoots LEASH_OUT_CM by up to 320 cm, leaving 80 cm to the 1,800 cm door entry. The radii are already at the smallest values condition 1 allows, so tuning cannot widen this. (2) Under B1's bursts every seed reaches 1,832 cm, past the door-entry radius. The 0 in a building there comes from the 120 cm door gaps, not from any margin. Simple geometry gives the burst worst case: LEASH_IN_CM plus ticks_for_ms(1_000) re-runs at about 18 cm each is about 1,840 cm. A soldier near angle 0 walking MOVES [127, 0] would reach building 0's door gap (x 1,800, |y| < 60). (3) Pattern (iii) is optimistic right after a burst. It feeds the 8-step-old World position at once. The real client can decide on its last complete pre-burst tick, which stays in the ring because the burst's ticks never overwrite those slots, until its first complete post-burst tick. (4) 'Corrected at the next command' overstates the speed. The correction acts on a decoded position lead - 1 + (newest - newest complete) steps old.
- **Evidence:** run1.log (my run at this snapshot) and final-regen-tests.log:83-91 agree. Skips: 1654 / 1720 (seed 2, slot 36 at -1354, -1061) / 1686 cm. Bursts: 1832 cm in all three seeds, at (1734, -592), (1671, -751) and (1586, -917): each starts from about 1,293-1,299 cm (just inside R_IN) plus 30 re-runs of MOVES [117, -49], about 539 cm. EveryTick: 1575 / 1575 / 1570. In-building 0 everywhere, stuck none. Door entry at 1,800 cm: greybox.rs (building 0 at [2200, 0], ROOM 400, door on the west side). Re-run rule: room input.rs:127-151. Burst both ways: l1 lib.rs:23, :197-201.
- **Fix:** No code or radius change (condition 1 bounds the radii). (a) bots lib.rs:59: replace ', 4 m beyond it.' with ', 4 m beyond it as geometry. Measured by the full sim-only test at this head: the barred patterns overshoot it by up to 320 cm (Skips, seed 2: 1,720 cm, 80 cm inside the door entry), and B1's bursts reach 1,832 cm (about LEASH_IN_CM plus ticks_for_ms(1_000) re-runs at full speed), past it.' (b) bots lib.rs:243-244: replace 'so a re-run command or a step cut short is corrected at the next command' with 'so a re-run command or a step cut short is corrected once the decoded position shows it (lead - 1 steps later, plus the ticks between the newest and the newest complete tick)'. (c) Add to the build notes and U4g's brief, next to pattern (iii)'s figures: pattern (iii)'s 0 in a building is not a margin; its worst case is about 1,840 cm, past the 1,800 cm door entry; after a burst the real client may steer on its pre-burst position, which (iii) does not model; so U4g's judged B1 runs can still void -17 and -2 through a building entry, and the building figure on every B1 run is the check.

### NA-B1-3 (Low)

- **Where:** spikes/netcode-proof/crates/client/src/lib.rs:245-248 (SOLDIER_FLAGS = 5, SOLDIER_DEAD = 1 << 3); crates/bots/src/lib.rs:479 (the test's `r.grid[5] & (1 << 3)`); crates/client/tests/endpoint.rs:407-408 (`dead.grid[5] = 1 << 3`)
- **Wrong:** The dead read is copied as literals from npt-sim, where they are private: SOLDIER_FLAGS at world.rs:92, FLAG_DEAD at world.rs:120. No test ties either copy to npt-sim's real death. The endpoint test sets the bit by hand with the same literal, so it proves only that own_soldier reads index 5, bit 3. If npt-sim's layout or flag changes, both copies drift silently: the leash would steer dead soldiers, or never steer at all, and no test would fail.
- **Evidence:** A real death in npt-sim sets health to 0 and flags to FLAG_DEAD (world.rs:1630-1634). The soldier layout is x, y, z, yaw, pitch, FLAGS, HEALTH, WEAPON (seam.rs:249), and FLAGS is not a pub grid Field, so the client cannot look the index up. Neither new test compares the copied bit with a death npt-sim produced.
- **Fix:** In npt-bots' #[cfg(test)] soldiers() (bots lib.rs:476-490): after `let dead = ...`, add `assert_eq!(dead, r.grid[6] == 0, "slot {}: the dead bit (flags field 5, bit 3) against health 0", r.index);`. Count the soldier records seen dead in Record and print the count in printed(), so the full test shows the tie was exercised (deaths occur in a World run: logs.rs p6 asserts kills > 0). For npt-client's own copy, add a U6 follow-up, made when npt-sim exposes the flag: an npt-client test that feeds the record of a soldier npt-sim killed and asserts own_soldier's dead bit is true.

## review-rust (APPROVE WITH FINDINGS; snapshot ee2973aa8ad8)

**Seat:** rust-reviewer (BRAWL-1 round 1, batch per the advisor's condition 11; notes: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/brawl1-review-rust.md; logs: /home/user/Shamble-Frontlines/.gate/u4r/brawl1-review-rust/{lint,tests,lab-tests}.log)

**Its snapshot field, whole:** Reviewed: wip/brawl1.patch sha256 ee2973aa8ad8c453b5720099c27b37cb9980820da74f33ad9c78ddd7ad591dc7 (sha256sum checked; the read copy /home/user/Shamble-Frontlines/.claude/worktrees/brawl1 at 352a245 has a `git diff HEAD` that is cmp-equal to it). Judged against packet brawl1-packet.md (sha256 16eab462c4e0...), read whole.

My runs, all through u4r-run.sh, all exit 0:
- lint.log (-x): `cargo fmt --all -- --check` passes. `cargo clippy --locked --workspace --all-targets -- -D warnings` passes in the measured build and with --all-features. Every npt-* crate was re-checked in both builds (2 Checking lines each).
- tests.log, npt-bots --lib: 2 passed, 1 ignored, in 11.26 s. The short sim test printed: in a building 0; largest distance 1575 cm (every tick) and 1629 cm (skips); 307 per 1000 ticks skipped; longest streak 6; stuck none.
- tests.log, npt-client endpoint brawl1: 1 passed.
- lab-tests.log, npt-lab bin: fp3, fp1, r10_fp3 and rr3_l2 passed (4 measured); fp1, fp3 and r10_fp3 passed in the bites build (3).
- Read, not re-run: the builder's .gate/u4r/brawl1/final-regen-tests.log has the full test in release at the current test text: 9 runs, in a building 0 in each, 23.52 s.

Checked and holding:
- The steering fits x^2+y^2 <= 127^2. The sim scales a move only above that (world.rs:494-498). Every steering vector is longer than 125 from 1300 cm out. Integers only.
- Decoded x and y are limited to 16 bits on the wire (quant.rs:197-215), so no overflow comes from network input.
- own_soldier reads the decoded integers (rec.grid), never the held f32s, and only from ticks that are complete and filed with a hash. after32 is pure. The own section's soldier index equals the slot (room.rs:1064-1076).
- Condition 4 holds: next_command, leash and toward_centre are private, the tests sit in a #[cfg(test)] module at the end of the file, and only that module names World and SimCore.
- Determinism: nothing new reads a clock or an unseeded generator, and npt-bots' clippy.toml bans HashMap, HashSet and the clocks.
- parse(): every sum in burst_starts is at most the checked last_burst, and the cast at scenario.rs:919 can no longer truncate.
- manifest(): the checks run in the ruled order (hash.rs:323-349).
- The no-log flag equals s.logs (scenario.rs:836-837, :1022). Counts that now stay 0 print 'not counted (--logs 0)'.
- Server::build with Bites::default() changes nothing, and with_event_start(0) is Room::new's default (room.rs:459).

Process slips (mine): an early progress update to my notes file was written by python3, and the patch comparison used a shell redirect into the session scratchpad. Both break the brief's 'Write and Edit tools only' rule. Neither touched the read copy or any tracked file.

### RR-B1-1 (Medium)

- **Where:** spikes/netcode-proof/crates/lab/src/logs.rs:584-599 (the building figure in room_tick, Counts fields :190-192); crates/lab/src/scenario.rs:1676-1689 (the summary text)
- **Wrong:** No test shows that the lab's building figure can be non-zero. If the figure were broken (wrong predicate, block skipped, wrong phase), every run would still read 'in a building: 0 slot-ticks (slots none)'. Condition 14 stops the re-run sets on 'a building total above 0', and condition 15 expects 'the building figure at 0 on every run', so both checks would pass without testing anything. The sim-only test checks the same `area != 0` rule on npt-sim's records, but not the code in logs.rs or the summary line.
- **Evidence:** `grep -rn in_building` over crates/, ops/ and check.mjs: the only readers of in_building_slot_area, in_building_measured and in_building_slots are the writer (logs.rs:586-598) and summary_line (scenario.rs:1679-1683). No test or script reads them, and no test reads the 'in a building:' text. The everyone-in-view bar (>= 0.99) is an indirect guard, which is why this is Medium, not High.
- **Fix:** Add a logs.rs unit test that makes the figure fire. Build `let mut server = Server::build(&Scene::stress().with_stand_ins(StandIns::OFF), 1, 0, 48, 0, 0, &Bites::default());`, then teleport one soldier inside building 0 (centre (2200, 0): greybox.rs:91-92) with `server.room.script(&[], &[Placement { slot: 3, position: [2_200, 0, 0] }], &[]); server.room.tick();`. Open Logs with a folder (scenario's fresh-folder test helper, scenario.rs:1731) and call `logs.room_tick(&server.room, server.room.last_tick(), Phase::Measured, &[], &[], &[])`. Assert `logs.counts.in_building_measured == 1` and `logs.counts.in_building_slots == BTreeSet::from([3])`. Assert that room.jsonl's tick record holds "in_building_slot_area": [3, 1]. Assert that the same call in Phase::Warmup writes the pair but leaves the counts at 0. Optionally assert that summary_line contains 'in a building: 1 slot-ticks (slots 3)'.

### RR-B1-2 (Low)

- **Where:** spikes/netcode-proof/crates/bots/src/lib.rs:128-135 (toward_centre, :130 `x * x + y * y`) and :252 (leash's d2)
- **Wrong:** Both functions take i32 coordinates but compute x^2 + y^2 in i64. At (i32::MIN, i32::MIN) the sum is 2^62 + 2^62 = 2^63, one more than i64::MAX. In the dev profile (overflow checks on), leash panics there. In release (npt-bots has no overflow-checks override; Cargo.toml:116-120 sets them only for npt-sim and npt-proto) the sum wraps to i64::MIN, and toward_centre called directly would then panic in `i64::isqrt`, which panics on a negative input. Neither function states its input range, and the test's edge cases stop at the 16-bit grid.
- **Evidence:** The values come only from ClientEndpoint::own_soldier. POSITION_XY is 16 bits on the wire and Quant::value refuses anything out of range (proto quant.rs:197-215), so decoded |x| and |y| are at most 32,768 and d2 is at most 2^31. Unreachable today, hence Low. The edge cases at lib.rs:262-268 (packet) cover only the ±32,767/-32,768 corners.
- **Fix:** Compute the square in u64 so no i32 input can overflow or reach isqrt with a negative number. In toward_centre: `let d2 = u64::from(x.unsigned_abs()).pow(2) + u64::from(y.unsigned_abs()).pow(2); let r = d2.isqrt(); let len = i64::try_from((if r * r < d2 { r + 1 } else { r }).max(1)).unwrap_or(i64::MAX); let part = |v: i32| i8::try_from(-i64::from(v) * 127 / len).unwrap_or(0); [part(x), part(y)]`. In leash: compute d2 the same way and compare it with `(LEASH_OUT_CM as u64).pow(2)` and `(LEASH_IN_CM as u64).pow(2)`, or make the two constants u64. Add `(i32::MIN, i32::MIN)` to the test's edge cases. The bracket around the `if` also makes the `.max(1)` precedence explicit. Alternative: keep the code and document the domain ('x, y in the position grid's 16-bit range: d2 < 2^32').

### RR-B1-3 (Low)

- **Where:** spikes/netcode-proof/crates/client/src/lib.rs:247-248 (SOLDIER_FLAGS = 5, SOLDIER_DEAD = 1 << 3); crates/bots/src/lib.rs:479 (`r.grid[5] & (1 << 3)`); crates/client/tests/endpoint.rs:408 (`dead.grid[5] = 1 << 3`)
- **Wrong:** The soldier layout's flags index and the dead bit are npt-sim's private constants (world.rs:92 SOLDIER_FLAGS, :120 FLAG_DEAD; seam.rs:249 layout). They are now copied as literals in three places, and nothing ties any copy to npt-sim. The client test writes the same literal it checks, so it cannot catch a drift. If npt-sim's layout or flag bits moved, own_soldier would quietly read the wrong bit. A bit that is always set would switch the leash off in the lab. Only the sim-only test would notice, through its positions, and it uses the same literal.
- **Evidence:** client/src/lib.rs:244-246 says both constants are 'private there'. world.rs:1630-1634 sets `s.flags = FLAG_DEAD` exactly when health reaches 0. No test compares either copy with npt-sim.
- **Fix:** Tie the literals to npt-sim's behaviour without opening npt-sim. In the bots test's `soldiers()` (lib.rs:474-487), after `let dead = ...`, add `assert_eq!(dead, r.grid[6] == 0, "slot {}: flags bit 3 (dead) is set exactly when health (layout field 6) is 0", r.index);`. Count the dead soldier-ticks in Record and print them in `printed`, so a bit that is never set shows up. Add a doc line on the client's SOLDIER_FLAGS and SOLDIER_DEAD naming that test as their tie.

### RR-B1-4 (Low)

- **Where:** spikes/netcode-proof/crates/lab/src/scenario.rs:1433-1441 (the run's last-tick 32-bit check and its comment), :1506-1531 (the burst block), :2576 (fp3's comment)
- **Wrong:** (1) The order: the plan (section 6, TE-R2-2 and RR4-L2) says 'the 64-bit checks come first and keep their messages ... Then RR4-L2's caps'. The run's 32-bit cap (:1435) runs before the last burst's 64-bit check (:1507-1515). A B1 line that trips both (for example `--after-max-s 143165507 --bursts 2 --burst-every-s 1000000000000000000`) gets the 32-bit run message, not the 64-bit burst message. The comment at :1433 ('after the 64-bit checks') is true only for the run-length checks. Both lines are refused either way, so nothing wrong is accepted. (2) fp3's comment at :2576 says 'u32::MAX itself or less accepted', but both 32-bit bounds are multiples of 30 ticks and u32::MAX (4,294,967,295) is odd, so it can never be reached. The accepted value tested is 4,294,967,280. (3) The doc at :2508 says 'each bound's own value accepted', but no accepted case sits near the last burst's 32-bit bound.
- **Evidence:** Code order at scenario.rs:1427-1441, then :1506-1531. 4,294,967,295 = 3 x 5 x 17 x 257 x 65,537, which is not divisible by 2, so not by 30. fp3 (scenario.rs:2510-2590) tests the burst bound only on the refused side (`--bursts 2 --burst-every-s 600000000000`).
- **Fix:** Either move the run's u32 check (:1433-1441) below the burst block's 64-bit check, so every 64-bit check runs first as the plan words it; or keep the code and reword :1433 to 'after the run-length 64-bit checks (which keep their messages); the burst block's own 64-bit check follows, then its 32-bit cap', and note the order in the build record. Reword :2576 to 'the largest whole-second length below u32::MAX (4,294,967,280 ticks) accepted'. Add one accepted case at the burst bound, for example `parsed("B1", "--bursts 2 --burst-first-s 0 --burst-every-s 143165566")`: (1 x 143,165,566 + 0) x 30 + 300 = 4,294,967,280, which must be ok. Add one refused case at the next second, 4,294,967,310, which must be refused with the burst's 32-bit message.

### RR-B1-5 (Low)

- **Where:** spikes/netcode-proof/crates/client/src/lib.rs:350-368 (own_soldier's reduce with `self.wrap.after32("tick", ..)`); crates/client/tests/endpoint.rs:398-442 (brawl1_own_soldier_is_the_newest_complete_filed_tick_s_own_record)
- **Wrong:** Condition 5 and the doc say the newest tick is chosen 'by the endpoint's wrap rule', but the test uses ticks 100-106 only. Replacing the compare with a plain `a.tick < b.tick` still passes it. No run crosses the 32-bit tick wrap today (ops/ and check.mjs use --tick-start 63736, 64651 and 64936), so this is a test gap, not a live defect.
- **Evidence:** In endpoint.rs's brawl1 test every compared pair has b > a with no wrap. `grep tick-start` over ops/ and check.mjs finds only 63736, 64651 and 64936.
- **Fix:** Extend the test, or add a second one: join a fresh endpoint at JoinAccepted tick u32::MAX - 1. Feed a complete one-group own_only(u32::MAX, 1, vec![soldier(0, 100)]), then a complete own_only(0, 1, vec![soldier(0, 200)]), which wraps. Assert `own_soldier() == Some(([200, -200, 150], false))`, the record of tick 0 and not of tick u32::MAX.

### RR-B1-6 (Low)

- **Where:** spikes/netcode-proof/crates/lab/src/hash.rs:236-240 (MAX_ENTRY_TICK_COUNT); check.mjs:760-765 (TICK_NAMED, NOT_TICK_COUNTS)
- **Wrong:** RR4-L1 ruled the exact text `const MAX_ENTRY_TICKS: u32 = 36_000;`. The build names it MAX_ENTRY_TICK_COUNT so that check.mjs's TICK_NAMED grep (`/_TICKS$|CATCHUP/`) does not flag it as a rate-derived tick count. The reason is real: the value is a size bound like MAX_ENTRY_BYTES. But the check has a designed route for a name that matches and is not a tick count: NOT_TICK_COUNTS, each entry with its reason (check.mjs:761). Renaming to get past the grep is a deviation from a ruled name, recorded only in a doc comment.
- **Evidence:** hash.rs:238-239: 'so not named `*_TICKS`: check.mjs reads that name as a rate-derived tick count (TICK_NAMED)'. check.mjs:806 applies the name rule to every crates/ file except crates/net/. The plan's section 6 lists 'the check.mjs follow-up' as not in this lane, so the builder could not add the exception.
- **Fix:** No code change in this lane. The advisor accepts or refuses the rename by name in the round's ruling. At the check.mjs follow-up (SEC-U4R-R2-1 and the others), either add `MAX_ENTRY_TICKS: "a manifest's size bound (RR4-L1): four times today's largest entry, a literal like MAX_ENTRY_BYTES"` to NOT_TICK_COUNTS and restore the ruled name, or record the new name as final. Separately: 36,000 is 'four times' only at 30 Hz. The manifests are 30 Hz-specific anyway (ticks must equal (warmup_s + measured_s) x TICK_HZ), so this matters only if a 60 Hz manifest is ever added.

### RR-B1-7 (Low)

- **Where:** spikes/netcode-proof/crates/lab/src/logs.rs:586-596; crates/lab/src/scenario.rs:369-371
- **Wrong:** (1) The slot set is rebuilt from the flattened i64 pairs with `step_by(2).map(|&s| s as u8)`, a cast that would silently truncate after a round trip through i64. It also ties the count to the pair layout (`len() / 2`). The same function's idiom for a soldier's index is `u8::try_from(r.index).ok()` (logs.rs:516). Soldier indices are below 64 today, so nothing truncates. (2) Server::build's doc breaks a sentence over a lone line ('... on `bites`'' / '`Quant`;' / '`bites` plants ...').
- **Evidence:** logs.rs:593 `in_building.len() as u64 / 2`; logs.rs:596 `.map(|&s| s as u8)`; scenario.rs:369-371.
- **Fix:** Iterate the records once: `for r in records.iter().filter(|r| r.kind == EntityKind::Soldier && r.area != 0) { pairs.extend([i64::from(r.index), i64::from(r.area)]); if ph == Phase::Measured { self.counts.in_building_measured += 1; if let Ok(s) = u8::try_from(r.index) { self.counts.in_building_slots.insert(s); } } }`, with `let mut pairs = Vec::new();`, and insert the record only if pairs is not empty. That is byte-identical output with no cast. Rejoin the doc lines: '... behind one decoder per session on `bites`' `Quant`. `bites` plants its World bite (armed at `arm_tick`), its room bite and its pump bite.'

## review-arch (APPROVE WITH FINDINGS; snapshot ee2973aa8ad8)

**Seat:** engine-architect: BRAWL-1 build review, round 1 (condition 11's panel). I checked the npt-client pub item against condition 5 and plan.md 4.2/4.7, the form of EA-U4R-1 and EA-U4R-2, and npt-bots' boundaries. I looked wider too: who owns the leash state, the direction data flows, lab coupling. Notes: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/brawl1-review-arch.md. I ran nothing of my own and wrote no logs; the evidence is code in the read copy plus the builder's logs under .gate/u4r/brawl1/.

**Its snapshot field, whole:** ee2973aa8ad8c453b5720099c27b37cb9980820da74f33ad9c78ddd7ad591dc7 (wip/brawl1.patch, verified by sha256sum; read copy /home/user/Shamble-Frontlines/.claude/worktrees/brawl1 at 352a245 with the 10 files modified, nothing new; packet brawl1-packet.md sha256 16eab462c4e0fcbcd8360aad7a951207275927c7f0f6813b794df54c12f4b1a4). Checked and holding, no finding: (1) npt-client gains exactly one pub item, ClientEndpoint::own_soldier(&self) -> Option<([i32; 3], bool)> (client/src/lib.rs:350-369). It is &self, returns by value, uses no float, gives no reference into the ring and adds no dependency (client Cargo.toml unchanged). It filters used && complete() && Filed::Hash and reduces by Wrap::after32("tick", ..), the endpoint's own counter name; Wrap is Copy and after32 has no side effects (proto/src/wrap.rs:60-65). frame.own is kept sorted by key in both writers (apply :103-136, full_state :822-823), so the binary search is valid. The room's own section holds (Soldier, owner) (room/src/room.rs:1076). plan.md 4.7 lists npt-client's public surface by type, so a method on ClientEndpoint stays inside it. The anonymous tuple adds no new pub type, and nothing is exported through npt-client-wasm. (2) npt-bots gains no pub item: next_command, leash and toward_centre are private, and steering is a private field. World and SimCore appear only in the #[cfg(test)] module (bots/src/lib.rs:322-326, :474-475, :497); the library itself still uses only Command and Scene from npt-sim (:28). Cargo.toml and Cargo.lock are untouched (git diff --stat is empty). The leash state belongs to each client's Behaviour, and own_soldier is read only after the endpoint has been fed (:215, :223). (3) EA-U4R-1: wake() (scenario.rs:591-623) is the narrow form. It accepts new links, runs the I/O step, takes admitted and ended, and gives the due count; each caller keeps its own per-tick body and sinks. Computing the due count before the admitted loop changes nothing, because due_ticks is a pure function (server/src/schedule.rs:149). run_leg now uses SimClock. The pre-check tests' --nocapture output is the same before and after Part 1 (my diff of .gate/u4r/brawl1/p1-before-a.log against p1-after-e2.log, time lines dropped, is empty). (4) EA-U4R-2: pub(crate) type Sim = World (scenario.rs:51). World is still named only by hash.rs's bite list and dump checks (documented) and by the printed 'World::new(scene, ..)' strings (precheck.rs:677, :691). Server::build (scenario.rs:373-394) now builds every server: run, run_leg and lab_room. with_event_start(0) equals the room's default (room.rs:459, :488-491), Bites::default leaves world, room and quant unchanged (scenario.rs:258-285), and both paths keep RttSource::Link. (5) final-check.log: all 11 steps pass, with 535 and 613 tests, which is 531 and 609 plus the 4 new non-ignored tests. Nothing parses the summary line's new tail (searched ops/ and check.mjs).

### EA-B1-1 (Low)

- **Where:** spikes/netcode-proof/crates/client/src/lib.rs:245-248 and :368; copies at crates/bots/src/lib.rs:479 (test) and crates/client/tests/endpoint.rs:408; originals at crates/sim/src/world.rs:92 (SOLDIER_FLAGS) and :120 (FLAG_DEAD)
- **Wrong:** own_soldier reads the dead bit through copies of two private npt-sim constants (the flags field's index 5 and bit 3), and no test ties those copies to npt-sim. The endpoint test sets the bit with the same literal. The sim-only test reads World records with a third copy of the literal. If npt-sim's Soldier layout or FLAG_DEAD changes, every test still passes while own_soldier misreads the bit. The leash would then steer dead soldiers, or stop steering a live soldier whose bit 3 now means something else. The plan's section 3 names this same risk ('two definitions could drift') as its reason for keeping the building figure on the server side.
- **Evidence:** client/src/lib.rs:246 says it outright: 'npt-sim world.rs's FLAG_DEAD, bit 3: both private there'. endpoint.rs:408 is `dead.grid[5] = 1 << 3;` and bots/src/lib.rs:479 is `let dead = r.grid[5] & (1 << 3) != 0;`. A search of crates/ finds no other user of SOLDIER_DEAD and no test that reads a dead soldier from a real World through the client's constants.
- **Fix:** Follow-up at the next npt-sim change (EA-U4R-4's lane, or U6's bots'-view item): npt-sim exposes the Soldier layout's flags index and dead bit as pub consts (or an EntityRecord::soldier_dead() accessor), and npt-client plus the bots' test use them. Until then, if wanted now: add a #[cfg(test)] test in client/src/lib.rs (npt-sim is already a normal dependency) that copies logs.rs:784-800's p6 focus fire: Scene::stress().with_stand_ins(StandIns::OFF), seed 3, every soldier aiming at slot 16 if s < 16 else slot 0. Step it until SimCore::step returns SimEvent::Kill { victim, .. }. Assert that the victim's Soldier record has grid[SOLDIER_FLAGS] & SOLDIER_DEAD != 0 at that tick, and that no Soldier record has the bit on any earlier tick.

### EA-B1-2 (Low)

- **Where:** spikes/netcode-proof/crates/lab/src/scenario.rs:397 (Server::new), its only caller :390 (inside Server::build, :373-394)
- **Wrong:** EA-U4R-2 asks for one server constructor. Server::build is that constructor, but Server::new is still pub(crate), so precheck.rs (a sibling module) or any later caller can go around build with a room it built itself. That is how the duplication EA-U4R-2 removed came about in the first place.
- **Evidence:** A search of crates/ and check.mjs for 'Server::new' finds only scenario.rs:390, inside build. No doc link names it either, so narrowing it will not trip rustdoc's private-link warning.
- **Fix:** At scenario.rs:397, change `pub(crate) fn new(room: Room<Sim>, quant: Quant, rtt: RttSource) -> Server {` to `fn new(room: Room<Sim>, quant: Quant, rtt: RttSource) -> Server {`. Its one caller is in the same impl, so nothing else changes.

### EA-B1-3 (Low)

- **Where:** spikes/netcode-proof/crates/client/src/lib.rs:344-349 (own_soldier's doc) and :357 (the wrap compare); crates/client/tests/endpoint.rs:397-443
- **Wrong:** (a) The doc says 'A newer tick that is incomplete, refused or the sentinel changes nothing'. That holds only while the complete tick is still in the ring. A newer tick that lands in the same ring slot resets it (slot_for, t % ACK_RING_TICKS, which is 32 at 30 Hz). After 32 ticks that each got a group but never completed, own_soldier falls back to an older complete tick still held, or to None. The behaviour is fine, since the leash keeps walking on None, but the doc claims more than the code does. (b) Condition 5 asks for ticks compared by the endpoint's wrap rule, and the code does use after32. No test proves it, though: the test uses ticks 100 to 106, so a max_by_key(|t| t.tick) version would pass it too.
- **Evidence:** client/src/lib.rs:564-574: `if !self.ring[i].used || self.ring[i].tick != t { self.ring[i] = Tick { tick: t, used: true, ..Tick::default() }; }`. sim/src/ticks.rs:66-70: ACK_RING_TICKS = 32 x tick_hz / 30. endpoint.rs:399-442 feeds ticks 100 to 106 only.
- **Fix:** (a) Doc words: replace 'A newer tick that is incomplete, refused or the sentinel changes nothing.' with 'A newer tick that is incomplete, refused or the sentinel changes nothing while that tick stays in the ring (ACK_RING_TICKS ticks); a newer tick that takes its ring slot drops it, and then the newest complete tick still held is read, else `None`.' (b) Add one case to the endpoint test: JoinAccepted with slot 0 and tick u32::MAX - 1, then a complete own_only(u32::MAX, 1, vec![soldier(0, 9)]), then a complete own_only(0, 1, vec![soldier(0, -700)]), and assert own_soldier() == Some(([-700, 700, 150], false)). The ring visits slot 0 (tick 0) before slot 31 (tick u32::MAX), so a plain maximum would pick tick u32::MAX and fail.

### EA-B1-4 (Low)

- **Where:** spikes/netcode-proof/crates/lab/src/logs.rs:6-7 (module doc); behaviour at logs.rs:586-598
- **Wrong:** The module doc still says 'Every tick and second record carries all its counters, 0 included'. BRAWL-1's in_building_slot_area is written only on ticks that have one, as condition 10 rules and the SERIES.md row says. The doc's blanket rule now has an exception it does not name. marks/README.md:145-147 treats an empty tick series as a gap, so a future mark reading this name needs to know.
- **Evidence:** logs.rs:597-598: `if !in_building.is_empty() { ... rec.s.insert("in_building_slot_area".into(), in_building); }`. SERIES.md:368: 'absent on a tick with none'. logs.rs:6-7 is unchanged apart from the name added at :3.
- **Fix:** logs.rs:6-7: replace 'Every tick and second record carries all its counters, 0 included; every log ends with its footer.' with 'Every tick and second record carries all its counters, 0 included (BRAWL-1's `in_building_slot_area`, a list, is written only on ticks with one: the advisor's plan review, condition 10); every log ends with its footer.'

### EA-B1-5 (Low)

- **Where:** spikes/netcode-proof/crates/lab/src/hash.rs:239 (MAX_ENTRY_TICK_COUNT) and its doc :235-238; check.mjs:760-765
- **Wrong:** RR4-L1's ruling and the approved plan section 6 name the constant MAX_ENTRY_TICKS. The build calls it MAX_ENTRY_TICK_COUNT, so that check.mjs's TICK_NAMED rule (/_TICKS$|CATCHUP/) does not treat it as a rate-derived tick count. The reason holds: the constant is a manifest size bound, not derived from the tick rate. But it departs from the ruled words and gets past the rule by renaming, rather than through the rule's own exemption list (NOT_TICK_COUNTS).
- **Evidence:** hash.rs:238: 'not named `*_TICKS`: check.mjs reads that name as a rate-derived tick count (TICK_NAMED)'. check.mjs:760: `export const TICK_NAMED = /_TICKS$|CATCHUP/;`. check.mjs:761-765: NOT_TICK_COUNTS holds only CMD_CATCHUP.
- **Fix:** No code change in this lane. The report names the deviation and its reason beside RR4-L1. The check.mjs follow-up already homed for PR #40 gains one item: add `MAX_ENTRY_TICKS: "a manifest size bound (RR4-L1), not a rate-derived tick count"` to NOT_TICK_COUNTS, then rename hash.rs's constant back to MAX_ENTRY_TICKS, or keep the current name with that recorded.
````

### The advisor's ruling on the builder's points (advisor-brawl1-points-full.md; its ADV-PT-L1 and ADV-PT-L2 ride the fix pass)
_`.checkpoints/netcode-proof-test/advisor-brawl1-points-full.md` lines 1-107 (sha256 0c1f6f5de46f), as they are:_

````text
# Advisor: the ruling on BRAWL-1's builder points (workflow wf_bbfeece0-4c7), 2026-11-15

Written by script, word for word, from the workflow's journal (advisor-points' StructuredOutput result). The points: brawl1-points.md. Its notes: advisor-brawl1-points.md. Nothing below is the main session's.

**Snapshot:** wip/brawl1.patch sha256 ee2973aa8ad8c453b5720099c27b37cb9980820da74f33ad9c78ddd7ad591dc7. I checked that it equals `git diff` of the read copy (/home/user/Shamble-Frontlines/.claude/worktrees/brawl1, HEAD 352a245c, the change uncommitted), in both plain and --binary form. numstat: 10 files, 875 insertions and 123 deletions, 998 changed lines. I ran no cargo or check.mjs command. Every judgement below rests on the builder's logs under .gate/u4r/brawl1/ and on the source in the read copy. I diffed and grepped those myself. My notes: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/advisor-brawl1-points.md.

## Summary

I ruled on all three points and on the recorded items. None of them changes the diff under review. Point 1 (TICKS-NAME): the constant stays as built, MAX_ENTRY_TICK_COUNT = 36,000. RR4-L1's ruled name was my error, because check.mjs refuses it. Point 2 (PROOF-E): proof (e) holds. Its two differences are the wallclock bite's L1 hash, which already varies between two runs at <base> by the bite's design, and libtest's filtered-out count, which is +1 for RR3-L2 (a)'s new test. Condition 6 (ii)'s 'identical' was my wording, and it could not hold at <base>. Point 3: I accept r10_carry_82's move to a boundary below the cap. Recorded items: I accept the zero-test reruns, the check.mjs fix and rerun, and the size (998 lines, under the 1,400 'well past' line). I also re-checked proof (d): 109 lines against 109, with only the build line's commit and binary sha256 differing. One correction to the record matters for U4g. Pattern (iii)'s 1,832 cm is not a fixed face or a vehicle's reach. The three maxima lie at three different angles in one 1 cm band, which fits the leash's own reach under a 30-tick re-run. That reach is past the 1,800 cm door entry, so 'area 0 in three seeds' does not bound B1. Further findings: one Medium (the pattern (iii) reading, for U4g's brief) and five Lows. Nothing for Mustafa.

## Rulings

### Point 1 (TICKS-NAME): the form of RR4-L1's constant

- **Ruling:** Option (a) stands: hash.rs:239 `const MAX_ENTRY_TICK_COUNT: u32 = 36_000;`, with its doc at :236-238. RR4-L1's ruled name `MAX_ENTRY_TICKS` is amended to MAX_ENTRY_TICK_COUNT. That name was my error, recorded as correction ADV-BP-4. Options (b) and (c) are refused, and so is carry 39's route (d), the constant put in npt-sim.
- **Why:** Facts checked. RR4-L1 (advisor-u4r-r2-full.md:92-97) rules the name MAX_ENTRY_TICKS. check.mjs:760 `TICK_NAMED = /_TICKS$|CATCHUP/` makes any matching const outside npt-sim and not in NOT_TICK_COUNTS (:761-765) a tick count, and :839-846 refuses it as a literal. p1-a-check.log:50 shows that refusal for hash.rs:238. As built, the refusal at :338-343 prints the constant, and its text equals the ruled message. r10_fp3 asserts that text exactly (:1655-1662) and reads a synthetic entry at 36,000 (:1663-1666). Final check: final-check.log, all 11 steps PASS.

Why (a) wins: the cap is a resource bound per tick, not a time. RR4-L1's own reason is encode's reserve of ticks x 1,900 B (replay.rs:296) and a runner entry's work per tick. The runner check already scales the entries with the rate (hash.rs:358-361, `x npt_sim::TICK_HZ`). At 60 Hz today's 300 s entries would be 18,000 ticks, still under 36,000, and the cap should stay 36,000. Option (b), `4 * ticks_for_ms(300_000)`, makes the bound follow the rate (72,000 at 60 Hz) for no reason, and carry 39 bars it (u4s-carries.md:169-172). Route (d), carry 18's (u4s-carries.md:129-132), has the same wrong meaning. It would also put npt-lab's manifest policy into npt-sim, against layering, and change npt-sim's ticks.rs, s1's DERIVED table and check.mjs's TICK_DERIVED. Option (c) changes check.mjs. By round 2's step 18 (advisor-u4r-r2-full.md:24), that pulls the whole check.mjs follow-up (SEC-U4R-R2-1/-2, TE-R2-3, INF-R2-5, INF-U4R-10 (2)) into this lane, which is already 998 lines and has no security-auditor seat.

Precedent: NC-U4R-7 (advisor-u4r-r1-full.md:90-92) kept TURN_EVERY and RETARGET_EVERY as literal counts because they claim no time. The doc at :236-238 claims no time and says openly why the name avoids `*_TICKS`. check.mjs's own doc (:794 comment block, 'Known limit') leaves a count under another name to review, and this is that review. No other const in crates/ ends in _TICK_COUNT (grep). The points file's range check.mjs:785-806 is slightly off: the rule's sentence is at :784-788, the code at :839-846. Not material.
- **Changes the diff under review:** no
- **The change:** None. The report records ADV-BP-4: 'RR4-L1's ruled name MAX_ENTRY_TICKS falls under check.mjs's TICK_NAMED rule and is refused as a literal; I did not check it; the name is MAX_ENTRY_TICK_COUNT.' A Low follow-up goes to the check.mjs follow-up: further finding ADV-PT-L3.

### Point 2 (PROOF-E): proof (e)'s two differences under condition 6 (ii)

- **Ruling:** I accept proof (e) with exactly these two differences. Together with proof (d), it proves run_leg's half of Part 1 at the old pins. The diff stays as it is. Condition 6 (ii)'s 'must diff identical' was my wording, and it could not hold at <base> itself: correction ADV-BP-3. Future captures of these tests mask exactly the L1 leg's hash values in the wallclock bite's comparer and result lines ('theirs ... hash' and the L1 hex). They keep the index, the loopback hash, compared and mismatching, and they explain libtest's 'filtered out' count by the binary's test count. Nothing else is masked.
- **Why:** I diffed the logs myself with the normaliser at brawl1-build.md:52, which leaves out runner, Compiling, Finished, Running and Blocking lines and libtest's seconds. Each log has 67 lines after normalising.
- A against B, both at <base>: only lines 46-47 differ. The L1 hash is 9375924675163200265 / 821df9d0f7ec1b09 against 1388086141469307493 / 134378fdc3ebb265.
- A against e2: the same two lines (now 5648772452115156074 / 4e64789de1560c6a), plus line 20 (52 to 53 filtered out) and line 66 (58 to 59).
- In all four logs, index 3, loopback hash 6328158668484839173 (57d222c7ed590705), compared 120 and mismatching 117 are equal. Every line of r6 and of the late-command bite is identical, including the deterministic bite mismatch at index 60 with both hashes. All four logs end with exit 0.

Why the hash varies: server pump.rs:236-243 (bites builds only) XORs each command's aim[0] with 1 | (SystemTime ns & 0xfe), so the L1 leg's World hash is wall-clock dependent by design.

Why the count moves: the patch adds exactly one #[test] in crates/lab, logs.rs's rr3_l2_a test (patch line 884). That matches +1 in both builds.

Proof (d), re-checked myself: p1-cd.log:38-150 against runs.txt:17-125, 109 lines each. Only the build line differs, and only in commit and binary sha256.

One process note: 'comes to me whole' did not say 'stop', so the builder went on to the re-pin, which condition 7 places after condition 6's results. This ruling cures that, and nothing needs redoing. The wording was mine, so it is part of ADV-BP-3.
- **Changes the diff under review:** no
- **The change:** None. The report records ADV-BP-3: 'Condition 6 (ii)'s identical-diff test could not hold at <base>, because the wallclock bite's L1 hash varies with wall-clock ns (pump.rs:236-243); and 'comes to me whole' did not say stop.'

### Point 3: r10_carry_82's boundary

- **Ruling:** I accept the change as built: the entry's bytes are rewritten to 360,000, so the boundary is 30,000 ticks (accepted) and 30,001 is refused with 'cannot fit'. That is below the 36,000 cap. The test keeps its purpose, the bytes/12 rule on both sides of its boundary, and the cap's own boundary is tested in r10_fp3.
- **Why:** Base: `at = 799_693 / 12` gives 66,641, and the test asserted it reads OK. With the cap at hash.rs:338, checked after bytes/12 at :325, 66,641 is now refused. p1-a-check.log:47 names exactly this failing test. The at+1 case was still caught by 'cannot fit' first, because bytes/12 is checked before the cap.

As built (hash.rs:1560-1568): the `.replace` of '"bytes": 799693' checks itself. If it ever matched nothing, 30,001 ticks would read OK against 799,693 bytes, and expect_err would fail. The cap's own boundary sits in r10_fp3 at :1655-1666: 36,001 is refused with the exact text, and a synthetic entry at 36,000 reads.

The facts text at brawl1-facts-full.md:113-116 named only r10_fp3's numbers and the synthetic 36,000 entry, so it missed this test. check.mjs caught the miss, as it should.

One small loss of strength (Low, ADV-PT-L2): 360,000 divides by 12 exactly, so a ceil plant would now pass, where base's 799,693 caught it.
- **Changes the diff under review:** no
- **The change:** None now. If a push is needed anyway, use 359,993 bytes: 29,999 at the boundary, 30,000 refused with 'cannot fit' (ADV-PT-L2).

### Recorded items: the zero-test runs, proof (a)'s failed first run, the size, pattern (iii)

- **Ruling:** 1. Zero-test runs: accepted. They are selector mistakes, and no failed run was rerun into a pass. 2. p1-a-check then p1-a-check2: accepted. The second run follows a fix of Part 1's own code against its checks. It is not a retry of a result under a no-retry rule. 3. Size: 998 lines is past the plan (about 700; 450-750) but under the 1,400 'well past' line, so there is no stop and no SIZE-CHECK note to Mustafa. It goes on the card. 4. Pattern (iii): recorded as printed only (condition 3 (iii)), with no diff change. The record of what the 1,832 cm means needs correcting for U4g's brief (further finding ADV-PT-M1).
- **Why:** 1. p1-before-1.log ran '--lib' twice: 'running 0 tests', 5 filtered out. p2-full-release.log ran '--exact' without 'tests::': 0 ran, 3 filtered out. The reruns ran the intended tests: p1-before-a, 1 then 3 passed; p2-full-release-2, 1 passed in 23.51 s.
2. p1-a-check.log failed the test step (r10_carry_82) and the deps step (the TICKS name). p1-a-check2.log passed all 11 steps (532/610). The log timestamps are in order: check2 03:08-03:20, then det, then cd, then e2 03:33-03:35, and Part 2 from 03:39. final-check.log passed all 11 steps (535/613).
3. The size is the numstat above; the 1,400 line is my plan review's Size section.
4. final-regen-tests.log prints the Bursts maxima at (1734, -592), (1671, -751) and (1586, -917). These are three different angles, all with isqrt 1832. That is a radius, not a face. My reading, computed and not run: the walk turns 22.5 degrees every 4 commands at 20 cm a command (bots lib.rs:30-52; 600 cm/s at 30 Hz), a circle of radius about 205 cm. A circle kept inside R_OUT (centre about 1,195 cm out) that re-runs one walk command for 30 ticks (600 cm) ends at most about 1,195 + sqrt(205^2 + 600^2), about 1,829 cm out. That matches.

Aside: a Bursts line prints 'skipped 14400 of 0 chances (0 per 1000)', because chances are counted only under Skips (lib.rs:555-560, :610-627). That is ADV-PT-L1.
- **Changes the diff under review:** no
- **The change:** None to the diff. The main session carries ADV-PT-M1's reading into U4g's brief, beside the builder's, labelled as computed.

## Further findings

### ADV-PT-M1 (Medium)

- **Where:** brawl1-build-report.md:117-120 and brawl1-build.md:193-196, :338-342 (pattern (iii), for U4g's brief); bots lib.rs:30-66 (MOVES, TURN_EVERY, LEASH_OUT_CM/LEASH_IN_CM); .gate/u4r/brawl1/final-regen-tests.log (the Bursts lines); greybox.rs:64-70, :92-133 (ROOM 400, DOOR_HALF 100: door entry at 1,800 cm on six axes)
- **Wrong:** The record suggests that 'a fixed radius stops it', with the 14 m vehicle's reach as one candidate, and it reads 'no area != 0 in three seeds' as reassurance. The positions do not fit a stop. The three maxima lie at three different angles in one 1 cm band, which fits the leash's own reach under a 30-tick re-run (about R_OUT - 205 + sqrt(205^2 + 600^2), about 1,829 cm; my computation, not a run). Inside condition 1's bounds (R_OUT at least 1,400) that reach passes the 1,800 cm door-entry radius at every angle, door axes included. The real B1 loop also acts on a stale view after each burst. So a B1 judged run can carry a soldier into a door gap, and area 0 in three seeds is not a bound. If U4g's brief carried the builder's inference, its B1 runs could repeat BRAWL-1's INVALID without anyone expecting it.
- **Fix:** No change to BRAWL-1's diff: B1 is printed only, by my condition 3 (iii). The main session puts this reading in U4g's brief, labelled as computed, beside the builder's figures and positions. U4g's plan then says how a B1 building entry reads in -17, or measures how often it happens, before its judged B1 runs.

### ADV-PT-L1 (Low)

- **Where:** spikes/netcode-proof/crates/bots/src/lib.rs:555-560 (chances counted only under Pattern::Skips) and :610-627 (printed)
- **Wrong:** The printed-only Bursts line reads 'skipped 14400 of 0 chances (0 per 1000)'. The rate and the chances are meaningless for that pattern, and the line goes into U4g's brief.
- **Fix:** Print chances and the rate only for Skips; for Bursts, print 'skipped 14400 (10 bursts x 30 ticks x 48 slots)'. This rides a push that is needed anyway; otherwise it becomes a STATUS follow-up.

### ADV-PT-L2 (Low)

- **Where:** spikes/netcode-proof/crates/lab/src/hash.rs:1560-1568 (r10_carry_82, 360,000 bytes)
- **Wrong:** 360,000 divides by 12 exactly, so the test no longer checks that bytes/12 rounds down: a `div_ceil(12)` plant now passes. Base's 799,693 caught it.
- **Fix:** Use 359,993 bytes (`at` = 29,999; 30,000 refused with 'cannot fit'; still under the 36,000 cap) and update the comment. This rides a push that is needed anyway; otherwise it becomes a STATUS follow-up.

### ADV-PT-L3 (Low)

- **Where:** spikes/netcode-proof/check.mjs:755-765 (TICK_NAMED, NOT_TICK_COUNTS); hash.rs:239
- **Wrong:** After ruling 1, MAX_ENTRY_TICK_COUNT is outside the tick-name rule only because of its name. The check's own register of exceptions (NOT_TICK_COUNTS, 'each with its reason') does not show it. A later duration could take the same `_TICK_COUNT` name as a literal and pass unseen.
- **Fix:** Add this to the check.mjs follow-up (check.mjs's next change; before PR #40 is marked ready at the latest). TICK_NAMED becomes /_TICKS$|_TICK_COUNT$|CATCHUP/, and NOT_TICK_COUNTS gains MAX_ENTRY_TICK_COUNT with the reason: 'a manifest's size bound: encode reserves per tick (replay.rs:296), so it does not follow the rate'. Add one check.test.mjs case: a literal `FOO_TICK_COUNT` is refused, and the registered name passes. Not in this lane.

### ADV-BP-3 (Low)

- **Where:** advisor-brawl1-plan-full.md condition 6 (ii) (mine)
- **Wrong:** Condition 6 (ii) required the before and after outputs to diff identical. The wallclock bite's L1 hash varies between two runs at <base> by design (pump.rs:236-243), so the condition could not hold as written. Its 'comes to me whole' also did not say 'stop'.
- **Fix:** The report records ADV-BP-3 beside ADV-BP-1 and ADV-BP-2, in my words, with the masking rule from ruling 2. Nothing to redo.

### ADV-BP-4 (Low)

- **Where:** advisor-u4r-r2-full.md:94 (RR4-L1, mine)
- **Wrong:** RR4-L1 ruled `const MAX_ENTRY_TICKS: u32 = 36_000;`. check.mjs:760 reads that name as a tick count, and :839-846 refuses it as a literal (p1-a-check.log:50). I did not check it.
- **Fix:** The report records ADV-BP-4. The name is MAX_ENTRY_TICK_COUNT (ruling 1).

## For Mustafa

Decisions needed: none. The three points are technical choices inside the approved plan, and all three are mine to rule. For Mustafa: nothing. This is spike code, with nothing a player sees, hears or feels. The size, 998 lines against about 700 planned, is under the 1,400 'well past' line, so no size note is due. KEPT-LOGS stays with you as raised at the plan review. Nothing here changes it.
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
