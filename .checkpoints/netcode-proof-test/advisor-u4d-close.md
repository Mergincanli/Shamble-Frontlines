# Advisor: U4d's close (wave 1, Part B), 2026-11-11

Seat: the advisor agent, called from the day session (claude/netcode-proof-test-plan-hcky5g, head 59e03b7, U4d merged
at 39f4290). Role read word for word from main (`git show origin/main:.claude/agents/advisor.md`). I rule; the main
session applies every text change. I write only this file.

## Plan
1. Read the inputs: u4d-report.md (done), u4d-build.md ("Left for the main session or the advisor"), the five round-2
   notes, u4s-carries.md §U4d2 and §U4e (carries 51, 52), history.md "Decisions taken" (Mustafa's 2026-11-11 words),
   advisor-u4-wave1.md (W1-S1, W1-U4D, W1-PUB), u4-build-plan.md §4.1 and §4.2, advisor-u4.md:86,
   w1-plan-edits-check.md (F3-F11), the skeptic file at 558fc0e, na1-output-20261111.txt, plan.md R-31 (:1423) and
   C-U8-6.
2. Spot-check the code at the head where a ruling depends on it (scene.rs knobs, the generator's dead reckoning, ES's
   vehicle loops, SOLDIER's field count, the pub items).
3. Rule block by block: B1 (U4d's open points A, N2-M1, N2-M2, C, public items), B2 (U4e carries), B3 (plan-text
   Lows, F10 first), B4 (NA1, NA5, TE2-TE4, NA4), B5 (Mustafa).
4. End with "What I checked, and what I did not".

## Rulings

### B1. U4d's open points

**B1.1 (A) the estimate scene's knobs: APPROVED as stand-in tuning (as TUNE-1 was), with conditions.**
- `grenade_fuse_ms` = 1,515 (scene.rs:110, read by the World): not new tuning in substance. It is ADR 0001:1478-1479's
  two ES rates by Little's law (32 live projectiles at 21.12 spawns a second live 32 / 21.12 = 1.515 s), applied to
  thrown grenades so that every ES projectile has one fuse (`ticks_for_ms` rounds it up to 46 ticks, 1.533 s; T-M1
  measured 32.300 live, in band). It does change every bot-driven ES run's grenades (T2-L3), and that is intended: ES
  is R2's estimate, where a projectile lives about 1.5 s. Conditions:
  - scene.rs:109's doc says how the number follows (the engine-architect's wording point: ":1478" names a number that
    line does not state). Words: "A thrown grenade's fuse, ms: 2 s, and ES's 1.515 s (ADR 0001:1478-1479's 32 live
    projectiles at 21.12 spawns a second live 32 / 21.12 = 1.515 s, so every ES projectile fuses after it)." It rides
    U4d-F (B1.2).
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
- Text change: replace from "(per 50 s window:" to the carry's end with: "(per 50 s window: 13.4, 11.0, 10.1, 9.4,
  9.7, 8.7 a second at 1 in 10; the test-engineer's probe X2, seed 2 at 1 in 11: 7.578 a second over 600 s). The fix
  and the re-calibration of `aim_one_in` are measured on the judged window (a 10-minute measured window after
  warm-up, ADR 0001:1600; 3 seeds), not on T-M1's 300 s from tick 0. PM-0001-2's floor of 9.5 events a second does
  not move. The form, the measurement and the owner are the advisor's U4d close ruling (advisor-u4d-close.md, B1.2
  and B1.3): the main session's change U4d-F lands the fix, with ES's vehicles inside the fight (N2-M2), before
  U4d2's brief is final. U4d2 pins only recordings written by U4d-F's `Recording::GENERATOR` or later, and ES's
  hashes only from U4d-F's scene; its report names U4d-F's commit."

**New carry for U4r, from N2-M1's other half (A2-L3 = T2-M1's carry):** the judged ES runs are U4r's, driven by
npt-bots (§5 U4r; marks.json's ES runs). No U4 check shows that the brawl meets ES's bands before PM-0001-12 counts,
and §4.12's contract test resolves PM-0001-2 on SS suites only. u4s-carries.md §U4r gains:
- "59. **A2-L3 and T2-M1, ES under the bots (u4d-round2-arch.md, u4d-round2-test.md; the advisor's U4d close ruling):**
  "U4r's contract test also runs a short ES suite of the brawl `Behaviour` (48 clients) and resolves PM-0001-2's ES
  checks on it: shots, gameplay events, live projectiles and spawns in their bands (W1-U4D T-M1's numbers) and the
  share of snapshots whose client set holds all 47 other soldiers and all 15 vehicles. A band or share missed goes to
  the advisor before any ES run of PM-0001-12 counts. The ES runs themselves stay under PM-0001-2's counter script,
  run by run (ADR 0001:1480)."

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
  world.rs:1571-1590) and they block moves (world.rs:1226-1235), and both move the event rate.
- **The share** stays PM-0001-2's check on every ES run, and U4r's contract test shows it (carry 59).

**B1.4 (C) W1-S1's "(9 fields)": a slip, mine.**
- The list it heads names 8 fields, the catalogue's remote row (:137) has 8, and SOLDIER as built has 8 (seam.rs:249;
  the GROUPS test's 92-bit row). Nothing built depends on the 9.
- The record: the main session appends this erratum at the end of advisor-u4-wave1.md, so that every line cite into
  that file holds (F1's precedent):
  "**Errata (the advisor, 2026-11-11; advisor-u4d-close.md, B1.4 and B3):** W1-S1's '(9 fields)' (:79) reads '(8
  fields)': the list it heads names 8, as do the catalogue's remote row and SOLDIER as built. W1-U4D's quote of
  condition 1 (:224) reads 'what the server accepted, not what the client claimed' (advisor-u4.md:86). W1-PLAN's cite
  'advisor-u3q.md:143-144' (:525) reads ':142-143'. W1-PUB's '§4.7's items' (:423) reads 'plan §4.7's items'. No ruling
  changes."

**B1.5 The public items `Weapon`, `Scene::weapon`, `Scene::health` (R2-L4, A2-L2, N2-L4): approved.**
- They are TUNE-1's condition: each run's header records the weapon and the health (advisor-u4-wave1.md:252-253). That
  is a planned consumer, not "for later". Today their only reader is D-6 (tests/world.rs:618-628).
- The header writers are U4r's runner (npt-lab's L1 headers) and U6's npt-server (carry 60, B3; plan.md's U6 row,
  B3). TUNE-1's header condition had no home in any carry or plan row until now.
- **u4-build-plan.md §4.1, the npt-sim row at :184** (edit inside the row, so no line moves). Three insertions:
  1. In "the plain-data types `Placement`, … `RayHit`, `replay::Step`", after "`replay::Step`" insert
     ", `Weapon` (TUNE-1)".
  2. Replace "`Scene::by_id`, `id`, `name`, `slots`, `team`, `stand_ins`, `with_stand_ins`)" with "`Scene::by_id`,
     `id`, `name`, `slots`, `team`, `stand_ins`, `with_stand_ins`, `weapon`, `health`); W1-PUB's other approved items:
     `StandIns::OFF` and `s1`, `HitVolumes::MAX_SOLDIERS`, `MAX_COVER` and `MAX_BYTES`, carry 17's button and press
     constants, `Greybox::dm_tri`, and, bites builds only, `World::BITES`, `World::with_bite` and
     `World::bite_order_digest`" (the second half is F11, B3).
  3. In the Why cell, after "the advisor's W1-S1, W1-U4D and W1-PUB rulings" insert "; TUNE-1: `Weapon`,
     `Scene::weapon` and `Scene::health` are the facts each run's header records (U4r's npt-lab headers and U6's
     npt-server header: carry 60)".
- **The row at :185:** replace "`Recording`, `StateDump`, `replay::Replayer`" with "`Recording` (with `GENERATOR`,
  `encode`, `decode` and `synthetic`), `StateDump`, `replay::Replayer` and its methods". In its Why cell, after "A12's
  one driver" insert "; W1-PUB: `synthetic` is in the library because U4d2 regenerates the recordings (§4.15)" (F11).
- U4d's report (or U4's, when it folds it) lists the three TUNE-1 items as approved here.

**B1.6 Found in passing: TUNE-1's report condition is not met.** TUNE-1 asks that "U4d's report states the measured
kill rate ... as a property of the load" (advisor-u4-wave1.md:254-255). u4d-report.md has no kill rate (grep: no
"kill"). The numbers exist: X3, 489 kills over an S30, 0.272 a second; D-6's 100 s, 28 kills, 0.28 a second
(u4d-round2-test.md [6]). The main session adds this line to u4d-report.md under "Other findings, by route":
"- **TUNE-1's measured load:** SS's kill rate is 0.272 a second over an S30 (the test-engineer's probe X3: 489 kills in
  1,800 s; D-6's 100 s give 28, 0.28 a second; probe F's 0.42 was before the fix pass), so respawns, and with them
  generation changes, are rare in SS. N-M1's magazine of 32,000 rounds lasts 2,194 s at one soldier's rate, so 'no
  reloads' holds for a whole S30 (X3)."

B1 written 2026-11-11.

### B2. The U4e carries from round 2 (u4s-carries.md §U4e; new entries appended in this order)

**Carry 51 (SEC-U4d-R2-1): faithful to Mustafa's words.** The label quotes him verbatim (history.md:286). The body
defers the Low to U4e in writing, and nothing is pushed for it. The cites hold at the head (world.rs:1879-1881,
scene.rs:119, seam.rs:283). One fix, because the route it names is wrong: carry 26's route is "raises a seam change
with the advisor", not "to the main session". Replace "npt-sim is not U4e's lane, so if no such home exists when U4e
starts, U4e raises it to the main session (carry 26's route), which adds it in its own commit." with: "npt-sim is not
U4e's lane, and no such home exists today: the main session adds `Scene::effect_rows()` (pub, the rows per soldier,
8 or 0 in today's scenes; approved as that home by the advisor's U4d close ruling) in its own commit when U4e's build
starts, with its line in u4-build-plan.md §4.1's npt-sim row ("`Scene::effect_rows` (an effect row's owner, the one
public home: carry 51)"). Any other home U4e finds it needs is a seam change for the advisor (carry 26's route),
never a local copy of the 8."
- Reading of his words: the filter and the test are U4e's work, and the accessor lands when U4e starts, as the carry
  already said. It does not ride U4d-F.

**Carry 53, N2-L1 joined with the security-auditor's first carry (same rule, two proofs):**
"53. **N2-L1 and the security-auditor's own-state carry (u4d-round2-netcode.md, N2-L1; u4d-round2-security.md,
'Carries for U4e', first bullet; the advisor's U4d close ruling):** "The codec checks a record's kind against its
section, never its slot against the client: any slot's Soldier, OwnState or Effect record passes in the own-state
section. So the room's encoder fills client c's own-state section with c's slot's records only. In every `Snapshot`
and `FullState` sent to c, the own-state section's Soldier and OwnState records have index = c's slot and that
soldier's generation, and its Effect rows are exactly the rows whose owner is c's slot (carry 51's mapping). A test
shows it for every client of an SS room (48 clients) on each of at least 300 ticks, the ticks around a respawn (a
generation change) included. Catalogue item 5's L1 check reads each own-section record's identity (kind, index,
generation, an Effect row's owner) against the receiving client's slot, not only the section the record sits in. Its
owner-to-all bite takes the route the codec leaves open: other slots' own-state records in every client's own-state
section. Moving them into the entity section ends in `Err(OutOfRange)` at the encoder; that proves the codec, not the
class filter, so it does not count as that bite.""

**Carry 54, the security-auditor's second carry:**
"54. **The encoder's refusals in the room (u4d-round2-security.md, 'Carries for U4e', second bullet; the advisor's
U4d close ruling):** "The room handles `Err(OutOfRange)` from the encoder (a section mix-up, or an integer outside its
field's range) without a panic, `unwrap` or `expect`: that client's snapshot for that tick is skipped, the error is
counted per client and logged (under a name with its SERIES.md row), and the next tick encodes afresh. A test plants
one out-of-range record for one client and shows every other client's snapshot for that tick unchanged.""

**Carry 55, the security-auditor's third carry:**
"55. **Team and Public rows (u4d-round2-security.md, 'Carries for U4e', third bullet; the advisor's U4d close
ruling):** "npt-proto's `carries()` (snapshot.rs:173-179) counts Team and Public rows as the entity section; both are
empty today. A kind placed in a Team row needs the room's class filter (that team only), since the codec cannot tell
teams apart. Any Team or Public row added before U4's end comes to the advisor with its filter and a test.""
- The security-auditor's fourth bullet (the merge-time items) is not a carry: the catalogue's own-state row was done in
  U4d's merge (the checkpoint card, "U4d's merge"), and the U4e brief's rev 3 carries W1-S1's own words
  (advisor-u4-wave1.md:109).

**Carry 56, netcode N2-L3 (c):**
"56. **N2-L3 (c), the own-state section's size (u4d-round2-netcode.md; probe X1, u4d-round2-test.md; the advisor's U4d
close ruling):** "U4e's report states the measured own-state section per snapshot: bytes on the wire (mean, p99 and max
per client), in SS and in ES, with the OwnState record's share of it and how often that record changes. Over an S30
of SS, 47.49 of 48 OwnState records changed each tick (X1): the fire timer and the reload left count down every tick.
R2's estimate allows 16 B per snapshot for the local player's own state (r2-netcode.md:1756). A mean above it is
reported beside that figure, not judged: PM-0001-12 judges the downstream total.""

**Carry 57, U4E-Q6's room half** (F8, B3, gives it its home):
"57. **U4E-Q6, the room's half (advisor-u4-wave1.md:473-479; the advisor's W1-U4E ruling; homed by the advisor's U4d
close ruling):** "The event queue's cap is data, with a provisional value from net F121's arithmetic (1,024 is F121's
example, above the about 400 a B1 burst adds), recorded as provisional wherever the run's header records the room's
settings; U4f measures the rate and records §4.7's value before -22's runs. The room counts each overflow resync per
client, with its tick, in its log, so that every judged -17 and -18 run can report them (carry 58).""

The other round-2 notes for U4e (N2-L3 (a) and (b) are U4g's and U4h's; A2-L1 is U4g's) are not asked of me here.
They stay follow-ups, named in U4d's report, for the U4g and U4h briefs to take.

B2 written 2026-11-11.
