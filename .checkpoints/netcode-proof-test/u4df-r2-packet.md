# Review packet: U4d-F round 2, on what changed (the fix pass: FX-1, the exact reckoning test, and FX-2, scene.rs's doc words)

_Cut by `review_packet.py` from `u4df-r2-packet.json`. It holds sources only: code diffs, the approved plan's lines, the advisor's rulings, the last round's findings and Mustafa's words, each copied by line range. It holds no conclusion of the builder or the main session (no builder notes, brief, checkpoint card or summary). You stay free to read any file and run any test._

## Heads

- Round 1's files: `.gate/u4df-r1`, a worktree at dad9566 with round 1's snapshot (`.checkpoints/netcode-proof-test/wip/u4df.patch` as committed at 9dd037a) applied; its five crates/sim files' sha256 equal round 1's (u4df-review-test.md [1]).
- U4d-F's copy after the fix pass: `.claude/worktrees/wf_9ce88fa3-452-1` (branch u4df-build at dad9566; the change is uncommitted there).
- The diffs below compare round 1's files with the fix pass's; the tests folder's diff is expected to be empty.

## What changed (the diffs)

### npt-sim's sources (crates/sim/src), round 1 against the fix pass
_`git diff --no-index .gate/u4df-r1/spikes/netcode-proof/crates/sim/src .claude/worktrees/wf_9ce88fa3-452-1/spikes/netcode-proof/crates/sim/src`:  2 files changed, 164 insertions(+), 4 deletions(-)._

````diff
diff --git a/.gate/u4df-r1/spikes/netcode-proof/crates/sim/src/replay.rs b/.claude/worktrees/wf_9ce88fa3-452-1/spikes/netcode-proof/crates/sim/src/replay.rs
index b45d5c9..660ee18 100644
--- a/.gate/u4df-r1/spikes/netcode-proof/crates/sim/src/replay.rs
+++ b/.claude/worktrees/wf_9ce88fa3-452-1/spikes/netcode-proof/crates/sim/src/replay.rs
@@ -768,3 +768,158 @@ impl Replayer {
         Step::Stepped(tick)
     }
 }
+
+#[cfg(test)]
+mod tests {
+    //! The reckoning's exact guard (the advisor's U4d-F ruling, R2's FX-1; the test review's TF-M1): ES's sky aims
+    //! against the integration of the recording's own inputs by the World's own move rule. It steps no World.
+
+    use super::*;
+
+    /// The seeds whose ES recordings the test reads: seed 2 alone puts each of its six counts above 0.
+    const SEEDS: [u64; 1] = [2];
+
+    /// What each count of checked sky aims holds (the ruling's six; each must be above 0).
+    const COUNTS: [&str; 6] = [
+        "sky aims checked",
+        "crouched",
+        "held back",
+        "of a slot a MoveSpeed re-sped",
+        "the first after a lost command",
+        "the first after a held-back command carried on a tick that re-homed or re-sped its slot",
+    ];
+
+    /// One slot's reckoning from the recording alone: where its inputs put the soldier (m; `None` before the slot's
+    /// first spawn or teleport), the move speed the World holds (cm/s), and whether a MoveSpeed has set it.
+    #[derive(Clone, Copy)]
+    struct Reckoned {
+        est: Option<[f32; 2]>,
+        speed_cm: u32,
+        re_sped: bool,
+    }
+
+    /// One tick's move of `c` at `speed_cm`, m: the World's own rule (`crate::world::move_velocity`) times a tick.
+    fn one_tick(speed_cm: u32, c: &Command) -> [f32; 2] {
+        let v = crate::world::move_velocity(speed_cm, c.buttons, c.move_xy, 1.0).0;
+        let dt = 1.0 / TICK_HZ as f32;
+        [v[0] * dt, v[1] * dt]
+    }
+
+    #[test]
+    fn the_sky_aims_reckon_every_input_the_recording_emits() {
+        // TF-M1 (the test review's round 1; the advisor's U4d-F ruling, R2, FX-1): B1.2 step 3's reckoning, exact and
+        // apart from the World's chaos. Over the ES recording t_m1 replays (seed 2, 21,600 ticks from tick 1,000,000,
+        // a 2,000 ms ring), every sky aim (height 4,000 cm) equals, to the centimetre, where the slot's own recorded
+        // inputs put the shooter after that command's move, integrated from the recording alone, never from the
+        // generator's state. Per slot: the speed is the scene's, then each MoveSpeed brought into range as
+        // `apply_inputs` brings it; the estimate is each spawn's or teleport's own position, and the slot's checks
+        // start at its first (3b re-homes every slot within the first 60 s); each tick, phase 0 first, then the
+        // slot's commands in their recorded order, each moving the estimate one tick by the World's own move rule at
+        // that tick's speed. A held-back command (its tick one less than the tick that carries it) aims from its own
+        // tick's state after phase 0 and moves on the tick that carries it; a lost one is not in the recording, so it
+        // never moves the estimate, and the slot's next command shows it as a gap in `seq`. Plants Q1 (the old
+        // reckoning), Q2 (crouch ignored), Q3 (a held-back command's move dropped), Q4 (a MoveSpeed not held) and Q5
+        // (a lost command moving the estimate) each fail it.
+        let scene = Scene::estimate();
+        let ticks = 21_600;
+        let n = usize::from(scene.slots);
+        let mut counts = [0u32; 6];
+        for seed in SEEDS {
+            let rec = Recording::synthetic(&scene, seed, 1_000_000, ticks, 2_000);
+            let start = Reckoned {
+                est: None,
+                speed_cm: scene.move_speed,
+                re_sped: false,
+            };
+            let mut now = vec![start; n];
+            // Each slot's state after this tick's phase 0, kept for the next tick: a held-back command's own tick.
+            let mut own = vec![start; n];
+            let mut seqs: Vec<Option<u16>> = vec![None; n];
+            // Since the slot's last checked sky aim and its last re-home: a lost command; a held-back command carried
+            // on a tick whose phase 0 re-homed or re-sped the slot.
+            let mut lost = vec![false; n];
+            let mut carried = vec![false; n];
+            let mut touched = vec![false; n];
+            for t in &rec.ticks {
+                touched.fill(false);
+                for a in &t.attributes {
+                    if a.attribute == Attribute::MoveSpeed {
+                        let k = usize::from(a.slot);
+                        now[k].speed_cm = a.value.min(MOVE_SPEED_MAX_CM_S);
+                        now[k].re_sped = true;
+                        touched[k] = true;
+                    }
+                }
+                for p in t.spawns.iter().chain(&t.teleports) {
+                    let k = usize::from(p.slot);
+                    let [x, y, _] = p.position;
+                    now[k].est = Some([x as f32 / 100.0, y as f32 / 100.0]);
+                    touched[k] = true;
+                    lost[k] = false;
+                    carried[k] = false;
+                }
+                let before = std::mem::replace(&mut own, now.clone());
+                for (c, _) in &t.commands {
+                    let k = usize::from(c.slot);
+                    if seqs[k].is_some_and(|q| c.seq != q.wrapping_add(1)) {
+                        lost[k] = true;
+                    }
+                    seqs[k] = Some(c.seq);
+                    let held = c.tick == t.tick.wrapping_sub(1);
+                    assert!(
+                        held || c.tick == t.tick,
+                        "seed {seed}, slot {}, tick {}: a command of tick {}",
+                        c.slot,
+                        t.tick,
+                        c.tick
+                    );
+                    // The aim: from its own tick's state after phase 0 (the last tick's, for a held-back command).
+                    let from = if held { before[k] } else { now[k] };
+                    if c.aim[2] == 4_000
+                        && let Some(e) = from.est
+                    {
+                        let d = one_tick(from.speed_cm, c);
+                        let after = [e[0] + d[0], e[1] + d[1]];
+                        let want = [(after[0] * 100.0) as i32, (after[1] * 100.0) as i32];
+                        assert_eq!(
+                            [c.aim[0], c.aim[1]],
+                            want,
+                            "seed {seed}, slot {}, tick {} (carried on {}), seq {}: the sky aim (left) against the \
+                             recording's own inputs' reckoning (right)",
+                            c.slot,
+                            c.tick,
+                            t.tick,
+                            c.seq
+                        );
+                        counts[0] += 1;
+                        counts[1] += u32::from(c.buttons & Command::CROUCH != 0);
+                        counts[2] += u32::from(held);
+                        counts[3] += u32::from(from.re_sped);
+                        counts[4] += u32::from(std::mem::take(&mut lost[k]));
+                        counts[5] += u32::from(std::mem::take(&mut carried[k]));
+                    }
+                    // The move: on the tick that carries the command, at this tick's speed.
+                    let d = one_tick(now[k].speed_cm, c);
+                    if let Some(e) = &mut now[k].est {
+                        *e = [e[0] + d[0], e[1] + d[1]];
+                    }
+                    if held && touched[k] && now[k].est.is_some() {
+                        carried[k] = true;
+                    }
+                }
+            }
+        }
+        let line: Vec<String> = COUNTS
+            .iter()
+            .zip(counts)
+            .map(|(what, n)| format!("{what} {n}"))
+            .collect();
+        println!(
+            "FX-1: ES seeds {SEEDS:?}, {ticks} ticks each from tick 1000000: {}",
+            line.join("; ")
+        );
+        for (what, n) in COUNTS.iter().zip(counts) {
+            assert!(n > 0, "not vacuous: {what}: 0");
+        }
+    }
+}
diff --git a/.gate/u4df-r1/spikes/netcode-proof/crates/sim/src/scene.rs b/.claude/worktrees/wf_9ce88fa3-452-1/spikes/netcode-proof/crates/sim/src/scene.rs
index 7481231..4ed34ba 100644
--- a/.gate/u4df-r1/spikes/netcode-proof/crates/sim/src/scene.rs
+++ b/.claude/worktrees/wf_9ce88fa3-452-1/spikes/netcode-proof/crates/sim/src/scene.rs
@@ -80,7 +80,10 @@ const ES_WEAPON: Weapon = Weapon {
 /// 9.743 and 10.098 gameplay events a second for seeds 1, 2 and 3 (shot hits about 2.7, blast hits about 6.8, kills
 /// about 0.35), each 300 s half between 9.580 and 10.220 (tests/world.rs, t_m1 and its three-seed form). Of the shares
 /// tried (11, 16, 26, 34, 36, 38, 40, 48), 40 and 48 put every window and half in 9.5-10.5, and 40 keeps the larger
-/// smallest margin to the band's edges (0.080 a second; 48's is 0.036).
+/// smallest margin to the band's edges (0.080 a second; 48's is 0.036). Over seeds 1-9 and 36 minutes the rate is flat
+/// at about 9.88 a second, and a 300 s half's noise is 0.2-0.36 a second (U4d-F's review), so 40 is one passing set of
+/// halves, not a margin: a later change that moves ES is re-measured by the advisor's U4d-F ruling (RM), never re-tuned
+/// on three seeds.
 const ES_AIM_ONE_IN: u32 = 40;
 /// An S30 (a 30-minute run) and 5 minutes more, µs.
 const S30_PLUS_5_US: u64 = 35 * 60 * 1_000_000;
@@ -121,9 +124,11 @@ pub struct Scene {
     /// Its synthetic recordings aim among the other teams' soldiers on one command in this many (and on every command
     /// that throws), and into the air above the shooter on the rest, so those shots rise almost straight up and meet
     /// only a soldier whose box holds the shooter's chest (soldiers pass through each other): 1 in every scene but ES,
-    /// where the share sets its 10 gameplay events a second over the judged window, the last 10 minutes of a 12-minute
-    /// recording, and over each 300 s half of it (ADR 0001:1478 and :1600; the advisor's W1-U4D T-M1 and U4d close
-    /// ruling, B1.2). Above 1 it also turns on the generator's re-home of each soldier every 60 s (B1.2 step 4's 3b).
+    /// where the share tunes its gameplay events toward 10 a second over the judged window, the last 10 minutes of a
+    /// 12-minute recording, and over each 300 s half of it (ADR 0001:1478 and :1600; the advisor's W1-U4D T-M1 and U4d
+    /// close ruling, B1.2); it moves only the aimed hits, about 0.7 of ES's 9.9 a second at 40 (grenade blasts and sky
+    /// shots meeting a soldier on the shooter's chest make the rest). Above 1 it also turns on the generator's re-home
+    /// of each soldier every 60 s (B1.2 step 4's 3b).
     pub(crate) aim_one_in: u32,
     /// The projectile pool's fixed capacity (stand-ins and grenades).
     pub(crate) projectile_capacity: u16,
````

### npt-sim's tests (crates/sim/tests), round 1 against the fix pass
_`git diff --no-index .gate/u4df-r1/spikes/netcode-proof/crates/sim/tests .claude/worktrees/wf_9ce88fa3-452-1/spikes/netcode-proof/crates/sim/tests`: no change._

````diff

````

## What it is judged against

### The approved plan for the fix pass and round 2: the advisor's U4d-F ruling, R2 (advisor-u4df.md)
_`.checkpoints/netcode-proof-test/advisor-u4df.md` lines 89-180 (sha256 842ec092a4b4), as they are:_

````text
### R2. TF-M1 (Medium): APPROVED, and it lands before the merge, in U4d-F's fix pass (FX-1), with round 2 on what changed. One doc fix rides it (FX-2). APPROVE.

**The finding holds.** The counted suite tells the reckoning (N2-M1's fix, the reason U4d-F exists) from the old one
only through t_m1 on one seed, and only by a half: the old reckoning planted back in the base's own lines fails t_m1 at
9.240 on its first half with the window in band at 9.846 (te-plant-p1.log); crouch ignored alone fails it at 9.370 on
its second half, window 9.866 (te-plant-p3.log). A last-ulp change in the estimate moves a half by about 1.1 a second
(the builder's P1 form against the test-engineer's). The three-seed test catches both on every seed, but it is
`#[ignore]` and check.mjs counts passes only. What holds the reckoning for certain today is D-7's recording digest,
which the next legitimate change to `synthetic` re-pins for its own cause: T2-L1's lesson, the same one that put
B1.2's step 0 first.

**Why before the merge, not later:**
- Mustafa's words are "fix the scene before U4d2 pins the recordings". U4d2 bases on U4d-F's commit and pins its
  recordings next; the fix should arrive with its own exact guard, before any later change can re-pin D-7 over a
  half-broken reckoning.
- It is cheap: one deterministic test that steps no World, in one file, changing no recording, no pin and no band.
  Round 2 reads only it and FX-2.
- It is what lets R1 keep t_m1's half check as a tripwire rather than as a guard: the reckoning's guard becomes exact
  and independent of the World's chaos.
- The cost is a fix pass and one short round, inside U4d-F's own budget (round 1, one fix pass, round 2 on what
  changed). U4e's L3 waits for the merge either way.

**FX-1, the test (form and place; approved beyond B1.2's symbol list):**
- **Place:** src/replay.rs, a new `#[cfg(test)] mod tests` at the end of the file, holding one private `#[test]`,
  `the_sky_aims_reckon_every_input_the_recording_emits`, and any private helper it needs. A unit test, because it
  calls the World's own `crate::world::move_velocity` (pub(crate)) and reads crate-private items
  (`MOVE_SPEED_MAX_CM_S`, `TICK_HZ`, `Scene`'s pub(crate) fields); an integration test in tests/ could do it only by
  making `move_velocity` public or by copying the move rule, and both are refused (no public item; the rule keeps one
  home, B1.2 step 1). It runs in both builds.
- **The recording:** ES (`Scene::estimate()`), seed 2, 21,600 ticks from tick 1,000,000, span 2,000 ms: the recording
  t_m1 replays.
- **What it integrates, per slot, from the recording's own inputs alone (never from the generator's state):**
  - the speed: the scene's move speed, then each MoveSpeed attribute change for that slot, taken as
    `.min(MOVE_SPEED_MAX_CM_S)`, from its own tick on;
  - the estimate: set at each spawn or teleport of the slot to that Placement's own position (cm / 100, as f32); the
    slot's checks start at its first spawn or teleport (3b re-homes every slot within the first 60 s);
  - each tick, phase 0 first (attribute changes, spawns, teleports), then the slot's commands in their recorded order;
    each command moves the estimate once, by `move_velocity(speed, buttons, move_xy, 1.0).0` times
    `1.0 / TICK_HZ as f32`, at that tick's speed, in the generator's float order (estimate + move, per axis);
  - a held-back command (its `tick` field one less than the tick that carries it) has its aim checked against the
    slot's estimate and speed on its own tick, after that tick's phase 0, and moves the estimate on the tick that
    carries it, at that tick's speed (the World runs it then);
  - a lost command is not in the recording, so it never moves the estimate; the slot's next command shows it as a gap
    in `seq`.
- **The check:** for every command whose aim's height is 4,000 (the sky aim), its x and y equal
  `(after * 100.0) as i32` exactly, `after` being the estimate plus that command's own one-tick move as above. The
  first mismatch panics, naming the slot, the tick, the command's seq and both values.
- **Not vacuous:** it counts, prints, and asserts each above 0: the sky aims checked; the crouched ones; the held-back
  ones; those of a slot whose speed a MoveSpeed changed; the first sky aim after a lost command; and a held-back
  command carried on a tick whose phase 0 reset or re-sped its slot. If any count is 0 on seed 2, seed 1 and then seed
  3 are added until each is above 0, and the test's comment names the seeds.
- **Plants, each by Edit in the generator, restored by Edit, sha256 checked, the whole measured suite with
  `--no-fail-fast`; each must fail this test by name** (other tests failing beside it, such as t_m1, d7 or a_m1, are
  recorded, not required):
  - Q1, the old reckoning: the test-engineer's P1 form (u4df-review-test.md [5], its two Edits);
  - Q2, crouch ignored: P3's form ([11]);
  - Q3, a held-back command's move dropped: the held-back block's two lines in `synthetic` removed;
  - Q4, a MoveSpeed not recorded: the line that sets `speed_cm` from the emitted value removed;
  - Q5, a lost command moving the estimate: `c.est = after;` added in the lost arm (`1 => {}`).
- **Run time:** it steps no World, so it should take a second or two; if it takes more than 5 s in either build, it
  uses the recording's first 7,200 ticks and says so in its comment (the counts still above 0).
- **Counts:** +1 in each build. The copy's check.mjs then gives measured 368, bites 415, self 52.

**FX-2, scene.rs docs (NF-L2's wording point and NF-M1's, riding the fix pass; doc comments only):**
- `ES_AIM_ONE_IN`'s doc: after "(0.080 a second; 48's is 0.036)." add the sentence in T3 (a).
- `aim_one_in`'s doc: "sets its 10 gameplay events a second" becomes "tunes its gameplay events toward 10 a second",
  and after "U4d close ruling, B1.2)" insert the clause in T3 (b), before the full stop.
- Rewrap to at most 120 characters; no other word changes. rustdoc runs on it in check.mjs's doc step.

**Who and how:**
- **The fix pass:** the main session, itself or through one fresh builder (general-purpose, Opus, xhigh) in the same
  copy, `.claude/worktrees/wf_9ce88fa3-452-1`, which first reads u4df-build.md and `git diff` there. Every cargo and
  check.mjs run through u4df-run.sh, as in run A. Record: u4df-build.md, a new section "Fix pass, round 1" (each FX,
  each plant's log and diff, the sha256 of the five files at its end).
- **Its evidence:** fmt and clippy, host and wasm32, both builds; the suites in both builds with D-7 and every pin
  unchanged; the three-seed test once by name, its 21 "ES seed" lines equal to end-three-seeds.log's (FX-1 and FX-2
  change no recording); Q1-Q5 each failing FX-1's test by name; check.mjs twice, counted, 368 / 415 / 52.
- **Round 2, on what changed only** (CLAUDE.md's round: both of U4d-F's reviewers, Opus at xhigh, from a packet cut by
  review_packet.py holding the fix pass's diff against the round-1 files (wip/u4df.patch at 9dd037a), this block and
  T3's FX-2 words, and no conclusion of the builder or the main session):
  - netcode-architect, read only: does FX-1 integrate exactly what the World does with the recording's inputs (phase 0
    before the commands; a held-back command aimed on its own tick and moved on the tick that carries it; a lost one
    never), are its counts the right ones, and do FX-2's words say what is true;
  - test-engineer (general-purpose, with Edit for plants): Q1-Q5 each fail FX-1's test by name; FX-1 passes in both
    builds; world.rs, tests/world.rs and tests/replay.rs are byte-identical to round 1's (their sha256 as in
    u4df-review-test.md [1]); replay.rs and scene.rs differ from round 1's only by FX-1 and FX-2; check.mjs once,
    counted, 368 / 415 / 52; the copy byte-identical to its own start hashes at the end.
- **After round 2:** if both seats return APPROVE, or APPROVE WITH FINDINGS with Lows only, the merge goes ahead on
  M1-M9 (R5) without another ruling, and the Lows become follow-ups listed in the report. A Medium or above, or a
  REVISE, comes to me first; a third round only for an open Critical or High.

R2 written 2026-11-12.
````

### FX-2's words: the advisor's U4d-F ruling, T3 (advisor-u4df.md)
_`.checkpoints/netcode-proof-test/advisor-u4df.md` lines 449-457 (sha256 842ec092a4b4), as they are:_

````text
### T3. FX-2's doc words (scene.rs; one line each here: the fix pass applies them by Edit at the doc's indent and rewraps to 120)
**(a)** appended to `ES_AIM_ONE_IN`'s doc, after "(0.080 a second; 48's is 0.036).":
```
Over seeds 1-9 and 36 minutes the rate is flat at about 9.88 a second, and a 300 s half's noise is 0.2-0.36 a second (U4d-F's review), so 40 is one passing set of halves, not a margin: a later change that moves ES is re-measured by the advisor's U4d-F ruling (RM), never re-tuned on three seeds.
```
**(b)** inserted in `aim_one_in`'s doc after "U4d close ruling, B1.2)", before the full stop:
```
; it moves only the aimed hits, about 0.7 of ES's 9.9 a second at 40 (grenade blasts and sky shots meeting a soldier on the shooter's chest make the rest)
```
````

### Round 1's finding FX-1 answers: TF-M1 (the test-engineer)
_`.checkpoints/netcode-proof-test/u4df-review-test.md` lines 154-169 (sha256 90d92f743e5e), as they are:_

````text
- TF-M1 MEDIUM, not before merge (for the advisor; a new test is outside B1.2's symbols): the counted suite guards
  B1.2 step 3's reckoning, N2-M1's fix, only by t_m1 on one seed, which tells a broken reckoning from the right one by
  less than one 300 s half's noise. Where: tests/world.rs:1150-1160 (the events and drift assertion, t_m1 = seed 2
  alone at :1174); replay.rs:405-410, :470-476, :506-507 (the reckoning). Evidence: P1 (the old reckoning) fails t_m1
  by 0.260 on its first half, window in band (9.846); P3 (crouch ignored only) fails it by 0.130 on its second half,
  window 9.866; a half's sd across seeds is about 0.19-0.26 and a last-ulp float change swings it by 1.1 ([5]). The
  three-seed test fails P1 and P3 on every seed, but it is #[ignore] and check.mjs counts `passed` only. Under 3b the
  reckoning is worth about 0.43 events a second (P1 mean 9.480 against 9.906, [6]). What holds the reckoning for
  certain is D-7's recording digest: T2-L1's lesson again (an invariant held only by a pin that a later legitimate
  change re-pins). Scenario: a later edit to `synthetic` (a GENERATOR 3, a new input kind) half-breaks the reckoning
  and re-pins D-7 for its own named cause; t_m1 passes with a probability of roughly a quarter to a half and the
  regression lands. Cheapest proof: te-plant-p1.log, te-plant-p3.log. Fix: one deterministic test of the reckoning,
  independent of the World's chaos: over ES's synthetic recording, each sky aim's x, y equals the integration of the
  commands the recording itself sends for that slot (move_velocity at the MoveSpeed it emitted, clamped; a held-back
  command moved when sent; a lost one never; teleports and spawns resetting), truncated to cm, exactly; P1 and P3
  each fail it on the first crouched or lost command.
````

### Round 1's finding FX-2 (a) answers: NF-M1 (the netcode-architect)
_`.checkpoints/netcode-proof-test/u4df-review-netcode.md` lines 114-126 (sha256 f89a6a359e05), as they are:_

````text
- NF-M1 MEDIUM, not before merge (for the advisor): the acceptance's drift check fires on noise at ES's stationary
  level, and 40 sits below the band's centre. Where: scene.rs:78-84 (40 and its measured doc), tests/world.rs:
  1150-1160 (each 300 s half in 9.5-10.5). Evidence: [9]-[10]: the generator is stationary (36 min, 9 seeds, slope
  -0.003 a second per minute) at 9.88; a 300 s half's sd ~0.36 puts 6 of 54 halves out of band by noise; out of
  sample seed 6 fails (9.446); 34, 36, 38 failed one in-sample half each, so 40 is a realisation the sweep selected on
  the seeds that judge it. Scenario: a later legitimate change to ES's draw stream or the World's ES outcome (U4g/U4h
  editing the shot path, a generator v3) re-runs t_m1 (seed 2: about 0.8 to pass) and step 4's three seeds (about 0.5);
  it fails with no drift and no defect, and the session re-tunes `aim_one_in` on noise again, re-pinning ES's
  recording each time. Reach today: none judged (U4d2 pins seed 2's 300 s recording, 10.440). Cheapest proof: my
  probes (nc-probe.log, nc-long.log). Fix (the advisor's call): keep 40 (the acceptance is met); the report states the
  stationary level (9.88, 9 seeds x 36 min), seeds 4-9's numbers and the half check's ~11 % false-alarm rate per half;
  a later re-measure judges drift by a paired-halves or slope test over 9 or more seeds (or longer windows) and sets
  the share by the stationary level nearest 10.0, never by in-sample margins.
````

### Round 1's finding FX-2 (b) answers: NF-L2 (the netcode-architect)
_`.checkpoints/netcode-proof-test/u4df-review-netcode.md` lines 135-144 (sha256 f89a6a359e05), as they are:_

````text
- NF-L2 LOW, not before merge (what 3b changes in what the recording exercises; report wording): replay.rs:449-465.
  Teleports 51.9 a minute in the window (base 3.9); sky-shot hits on point-blank overlaps 2.0 a second (base 0.75), so
  ES's events are now ~69 % grenade blasts, ~21 % sky-shot overlaps, ~7 % aimed hits, ~3.5 % kills (42 % same-team, as
  before), and `aim_one_in` moves only the aimed ~0.7 a second of 9.9 (scene.rs:124's "the share sets its 10 gameplay
  events" says more); the ruling's period-tied source is real and team-phased (team = slot / 16 makes the formula
  re-home each team inside one 20 s third: its hits peak there and dip ~13 % before its next; diag / off 1.11), and
  every judged window is whole minutes, so it averages out exactly; the teleports force no correction in any consumer
  today (synthetic recordings are replayed sim-only), but a predicting-client test fed ES's synthetic recording would
  meet 52 forced corrections a minute. Fix: the report names these numbers; any later consumer that predicts on the
  synthetic ES recording counts teleport corrections apart.
````

## The carries this packet holds

_Rule: every entry of `.checkpoints/netcode-proof-test/u4s-carries.md` whose text matches `U4d-F`._

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
