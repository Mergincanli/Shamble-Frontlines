# Advisor: U4d-F round 1 (the estimate scene fix), 2026-11-12

Seat: the advisor agent, called from the day session's workflow (claude/netcode-proof-test-plan-hcky5g, head 1148c45;
PR #40). Role read whole from main (`git show origin/main:.claude/agents/advisor.md`, 334 lines). I rule; the main
session applies every text change and builds the fix pass. I write only this file, with the Write tool, block by block.

**Verdict: APPROVE WITH CONDITIONS.** Both Mediums are accepted at their labels. NF-M1: 40 stays, and later
re-measures follow a new rule, RM. TF-M1: its test lands before the merge, in a fix pass, then round 2 on what changed.
The seven Lows are routed. Both of the main session's readings are confirmed, with two corrections of fact. The merge
follows round 2 on M1-M9. Nothing goes to Mustafa: his floor does not move and no rule of his changes.

## Plan
1. Read the inputs whole: the brief (done), both reviews (done), the builder's record (done), my U4d close ruling
   B1.1-B1.3 (done). Then: the card's "Running" (item 4), the build brief's TE2-5 clause and B2.3 (item 4), U4j's
   M1-M9 (item 5), carry 59's text (NF-L4), the reviewers' briefs' write rule (item 6).
2. Spot-check at the copy: the diff's file list, scene.rs's 40 and its doc, t_m1's assertions, the reckoning's lines in
   `synthetic`, 3b's switch; and the snapshot (wip/u4df.patch at 9dd037a) against the copy's diff.
3. Spot-check the logs behind the rulings: nc-probe.log and nc-long.log (NF-M1), te-plant-p1.log and te-plant-p3.log
   (TF-M1), end-check4.log / end-check5.log / te-check.log (the counts).
4. Rule items 1-6 in order, one block each, then the texts, the two lists and "What I checked, and what I did not".

## Progress
- [x] 1 read  - [x] 2 spot-checked (facts cited in each block)  - [x] 3 logs  - [x] R1  - [x] R2  - [x] R3  - [x] R4
- [x] R5  - [x] R6  - [x] the texts (T1-T4)  - [x] the end sections. DONE.

## Rulings

### R1. NF-M1 (Medium) with TF-L2 (Low): 40 stays, the acceptance stands as met, the report says what the level and the halves are, and a later re-measure judges by RM. Nothing here needs Mustafa's word. APPROVE.

**The finding holds.** I re-derived the netcode-architect's 36-minute figures from nc-long.log's own per-minute counts
(seeds 1-9, 23:26:56-23:27:26 UTC, exit 0): the nine seeds' mean per 300 s block from minute 2 is 10.096, 9.692, 9.941,
9.901, 9.799, 9.857 and 9.887 (minutes 32-35); the slope over minutes 2-35 is -0.0032 a second per minute around a mean
of 9.882; all 27 ten-minute windows (minutes 2-11, 12-21, 22-31) are in band, the lowest 9.537; 6 of the 54 single
300 s blocks are out of band; and seeds 1-3's minutes 2-11 equal the official windows (9.878, 9.743, 10.098). The
test-engineer's run through the test's own `EsRun` (te-probe-seeds4-9.log) gives seed 6's second half 9.446, the same
figure. The label stays Medium.

**Why 40 stays:**
- The acceptance as I ruled it (B1.2 step 4) is met on seeds 1, 2 and 3, and three independent runs agree on every
  figure (the builder's s6-three-seeds.log and end-three-seeds.log, the netcode-architect's nc-suite.log, the
  test-engineer's te-base-suite.log). The rule for choosing the share (the largest smallest margin) was written down
  before the sweep chose, and no band moved. Nothing in the acceptance changed after the fact.
- What the finding shows is that my per-seed half check is noisier at this level than I assumed when I wrote it: a
  300 s half's noise is about 0.2-0.36 a second, so at a level of 9.88 one half in about 9 misses by noise, and three
  seeds' six halves all pass only about half the time. That makes 40 one passing set of halves, not a margin. It does
  not make 40 wrong: the windows, the quantity PM-0001-2 judges run by run ("on every ES run", ADR 0001:1480), are all
  in band on seeds 1-9 (9.693 to 10.171; standard deviation 0.166 across the nine).
- A re-tune now would buy little and cost a re-pin. The share moves only the aimed hits (about 0.7 of ES's 9.9 events
  a second at 40; blasts and sky shots meeting the shooter's chest make the rest), so 36 or 38 would lift the level by
  about 0.1 at most, would be chosen on the same noise, and would re-pin ES's recording and widen round 2.

**Does any of it change the acceptance in a way that needs Mustafa's word? No.**
- His words bind two things: the scene is fixed before U4d2 pins the recordings, and PM-0001-2's 9.5 floor does not
  move. The drift is fixed: before U4d-F the rate fell from about 11.5 to about 4.3 a second within 12 minutes in every
  seed (measurement A, s4-probe-11.log); after it, the rate is flat over 36 minutes and nine seeds. The floor is
  unchanged in every check, now and in RM: every judged window lies in 9.5-10.5.
- The per-seed half check is mine (B1.2 step 4, "Against drift"), not his and not PM-0001-2's. RM below replaces it,
  for later re-measures only, by a drift check on the nine seeds' mean. That changes my rule, not his floor: RM keeps
  the floor run by run on every seed's window and widens it from 3 seeds to 9.
- t_m1, the counted test (seed 2), keeps its half check exactly as built. For given code it is deterministic, and it
  passes today (halves 9.880 and 9.606, window 9.743). Its job from now on is the tripwire that sends a later change
  that moves ES to RM; it is no longer the reckoning's guard (R2, FX-1).

**TF-L2, my erratum.** B1.2 step 4 says "A stationary generator makes the warm-up's length immaterial; that is what
this step shows." The step did not show it: inside the judged window the second half is lower than the first in 8 of
9 seeds (mean -0.404 a second; the nine seeds' halves average 10.096 and 9.692), the same in every seed, and the dip
does not recur after minute 12. The netcode-architect reads it as the hulls' loops, which run on the World's elapsed
ticks from zero in every run, so every run's minutes 2-11 see the same arrangement; I accept that reading as likely,
not proven. What the step did show is that the decay is gone. The report says so (T1), beside NF-M1's numbers.

**How a later re-measure judges drift: RM (carry 67; its words in T2).** In short:
- **When:** before the merge of any change that moves ES's synthetic recording (D-7's ES digest) or the World's
  outcome on it (t_m1's printed line), and at once whenever t_m1 fails.
- **Runs:** seeds 1-9, each a 36-minute ES recording (64,800 ticks from tick 1,000,000), counted as `EsRun` counts.
- **The floor, run by run:** each seed's window (minutes 2-11) meets all four bands, with 48 soldiers and 15 vehicles
  on every tick and every hull within 1,400 cm.
- **Drift:** the nine seeds' mean gameplay-event rate over each 300 s block from minute 2 to minute 31 lies in
  9.5-10.5 (today: 10.096, 9.692, 9.941, 9.901, 9.799 and 9.857, so today's scene passes RM). The slope over minutes
  2-35 is reported beside it.
- **Reported, not judged:** every seed's two halves.
- **The share** is re-set only when the floor or the drift check fails at the current value, then to the value whose
  nine-seed mean window lies nearest 10.0 with both passing, never on three seeds' margins; the numbers come to me
  before D-7 or t_m1 is re-pinned for it.
- **t_m1 failing while RM passes:** the session changes neither the share nor t_m1 and brings RM's numbers to me; I
  rule on t_m1's form then.

R1 written 2026-11-12.

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

### R3. The seven Lows: each one's route (labels stand; none triggers a push of its own)
Three ride the push FX-1 needs anyway (as doc words, FX-2, or as the report's words); the rest are carries, each
written once in u4s-carries.md (T2). Carries 67-72 are new; 1-66 exist (66 is the U4e brief ruling's F2-ROUTE).
- **NF-L1** (ES's alive soldiers change yaw on 48 % of ticks and pitch on 18 %, against R2's yaw on every moving update
  and pitch half the time, docs/research/r2-netcode.md:1751; verified at that line): **a carry, two halves.** Carry 68
  (§U4e) and carry 69 (§U4r): no ES byte or bandwidth figure is taken from the synthetic ES recording as ES's load,
  and any figure taken from it states the two shares beside R2's. U4e is the half that matters: my U4d close ruling
  lets U4e report ES numbers from U4d-F's scene, and U4e has no bots. Reach today: none (the netcode-architect's [6]).
- **NF-L2** (what 3b changes in what the recording exercises): **the report's words and one carry.** T1's "What the
  events are now" bullet gives the numbers (51.9 teleports a minute against 3.9; about 69 % blasts, 21 % sky-shot
  overlaps, 7 % aimed hits, 3.5 % kills; the team-phased minute, averaged out exactly by whole-minute windows). The
  overstatement in scene.rs's `aim_one_in` doc is corrected by FX-2 (b). Carry 70 (§U4g): a test or figure that
  predicts on the synthetic ES recording counts its teleport corrections apart (U4g is the prediction unit).
- **NF-L3** (3b's switch is `aim_one_in > 1`): **a follow-up, carry 71, with no knob now.** The builder stayed inside
  B1.2's symbols; scene.rs's doc already says the share above 1 turns the re-home on, and t_m1 would fail at once if
  ES's share went to 1 (a higher rate). A knob today would serve no second scene: a feature "for later". The carry
  splits the switch in the change that would first need it.
- **NF-L4** (the in-fight hulls cut ES's events by about a quarter at the old reckoning and share: 9.853 to 7.250 over
  300 s; s2-suites.log's t_m1 line, which the test-engineer confirmed): **carry 59's text gains one sentence** (T2),
  so U4r's contract test expects the effect and a missed ES band is not read as a bot defect before I have seen it.
- **TF-L1** (a_m1's non-vacuity rests on about two deaths, 58 dead soldier-ticks, in SS seed 5's 1,200 ticks; both
  generator-only plants took them away): **a follow-up, carry 72.** It is a pre-existing test outside B1.2's symbols,
  and U4d-F's real code passes it in every run; the carry makes the next change that moves SS's recording fix it first.
- **TF-L2** (B1.2 step 4's stationarity claim): **my erratum (R1) and the report's words** (T1's "halves" bullet).
- **TF-L3** (the builder's "P1 shows as a level, not a decay" holds for its own form only): **the report's words**
  (T1's "reckoning" bullet gives P1's three seeds from the test-engineer's [6], not one form's one seed). The builder's
  record stays as written: it is its claim, and the report is where the reading is corrected.

R3 written 2026-11-12.

### R4. The main session's two readings
**R4.1 TE2-5: CONFIRMED, with two corrections of fact and one brief wording for next time.**
- **The reading holds:** every npt-sim failure in run A had a cause the plan names, and none was retried into green.
  - t_m1, the old 300 s test at the old share, failed in step 2 (7.250, the hulls), step 3 (8.566, the reckoning) and
    step 4 (12.716, 3b), each time with the same values in both builds; it went green only through the planned steps
    (step 5's share, then step 6's rebuild). B1.3's "both move the event rate" foresaw it.
  - d7 failed in steps 2, 3, 4 and 5 on exactly the pin each step re-pins (B1.2: "a pin changes only in the step that
    changes it"), and passed after each re-pin.
  - The step 4-5 builder named the stop clause and left the ruling to the main session (u4df-build.md:355-357).
- **Correction 1:** t_m1 failed in steps 2 to 4, not 2 to 5. At step 5's first run it passed (seed 2's 300 s at 40:
  10.440); step 5's only failure was d7's named re-pin (s5-first-measured.log).
- **Correction 2:** the separate "re-run at once" happened in steps 2, 4 and 5 (s2-rerun-*, s4-3b-rerun-*,
  s5-rerun-*). In step 3 the first runs were already the whole crate with `--no-fail-fast` in each build, their
  output saved and both failures named in the notes (s3-first-measured.log, s3-first-bites.log), but with no
  separate re-run and no TE2-5 tag. The rule's purpose held (each failure named, from the whole output, never retried
  into green); the record should not claim a step-3 re-run.
- **Stricter than B2.3:** yes. B2.3 stops a unit's merge on an unexplained failure in its own crate; the build brief
  stopped a step group on any npt-sim failure, which a change whose plan expects failures between its steps cannot
  meet. Not stopping was right. Next time the builder brief says so in advance (T4, item 2), so a builder does not
  have to choose between its brief and its plan.

**R4.2 The rust-reviewer: not triggered. CONFIRMED.**
- I read the world.rs diff: `move_velocity` (world.rs:476-505 in the copy) holds the old run_command lines as they were
  (crouch, fire held, sprint only when neither, the speed in cm/s, the scale above 127, the velocity); run_command
  keeps `fire_held` for the trigger and calls the function; nothing else in the World changed in step 1.
- The bite: the old `let mut speed = speed_cm as f32 / 100.0; speed *= 1.01;` and the new
  `speed_cm as f32 / 100.0 * gain` with `gain = 1.01` are the same two f32 operations in the same order; in the
  measured build `* 1.0` is exact for every finite f32. Bit-identical, as the netcode-architect's [11] reads it.
- The evidence agrees: s1-suites.log passes in both builds with no test file touched, D-7's SS after-tick pins at 1, 30
  and 300 among them, and the speed bite's unit proofs.
- FX-1 adds no World line, so the fix pass does not trigger it either.

R4 written 2026-11-12.

### R5. The merge into the spike branch: after round 2, on conditions M1 to M9 (U4j's form)
Checked now, at the spike head 1148c45: nothing under spikes/netcode-proof/crates/sim changed since dad9566
(`git diff --quiet dad9566 HEAD -- spikes/netcode-proof/crates/sim`: true); the only spikes/ changes since are U4j's
(crates/judge, crates/log, marks/) and L-c8's results files; no other crate reads `synthetic`, `GENERATOR`,
`aim_one_in`, `fight_vehicles` or `move_velocity` (grep over crates/); round 1's snapshot wip/u4df.patch (committed at
9dd037a, sha256 755cd562…) is byte-identical to the copy's `git diff -- spikes/` today, and `git apply --check` is
clean on the spike head. npt-sim's dependants are npt-proto, npt-room, npt-server, npt-client and npt-lab; npt-lab and
npt-server also link U4j's npt-log, so the merged tree pairs U4d-F's npt-sim with U4j's crates for the first time
(M5 and M6 cover it).

The conditions (the main session; each result goes in u4df-build.md's new section "Merge into the spike branch"):
- **M1, the copy is the one reviewed.** Before the snapshot: the copy's HEAD is still dad9566bdb90…, `git status` in
  it shows the five crates/sim files modified and its record untracked, nothing else; and `sha256sum -c` of round 2's
  start hashes, run from the copy's root, gives all OK. Round 2's test-engineer saves that hash file under .gate/u4df/
  (not in its scratchpad), in `sha256sum` form with paths from the copy's root. Any other result stops the merge and
  comes to me.
- **M2, the base still fits.** `git diff --quiet dad9566 HEAD -- spikes/netcode-proof/crates/sim` on the spike
  branch's head is true. If false, stop: the patch's base no longer matches.
- **M3, the snapshot and the apply.** The saver stopped first. `python3 -I
  .checkpoints/netcode-proof-test/worktree_patch.py .claude/worktrees/wf_9ce88fa3-452-1
  .checkpoints/netcode-proof-test/wip/u4df.patch u4df-` lists exactly the five files, all under
  spikes/netcode-proof/crates/sim/ (it also copies the copy's u4df-build.md into the main folder: that is the record's
  route). Then `git apply --check`, then `git apply`, in the main folder.
- **M4, the merged bytes are the reviewed bytes.** From the main folder's root, `sha256sum -c` of the five spikes/
  lines of round 2's start hashes gives 5 OK. A patch that applies but differs stops the merge.
- **M5, check.mjs counted on the merged tree**, through main-run.sh (the lane lock held exclusively, idle at the start,
  the 2 s touch loop, the shared target, offline), its log under .gate/u4df-merge/, with the pre-run builds
  `cargo test --locked --workspace --no-run` and the same with `--all-features` showing "Compiling npt-sim" from the
  main folder's path. All 10 steps pass, with measured 399 (396 + 3), bites 446 (443 + 3), self 52 of 52, marks 81
  read and checked with the ADR rows unchanged, redact 0 findings. The +3: step 0's two module tests and FX-1. In the
  same main-run.sh call, the three-seed test once by name (`--ignored --exact --nocapture`), its 21 "ES seed" lines
  equal to end-three-seeds.log's, and FX-1's test by name with `--nocapture`, its counts printed. Any other count or
  line is explained in the notes before the commit.
- **M6, a failing test is never retried into green unnamed** (TE2-5's rule, B2.3): a failing build's crate is re-run at
  once through main-run.sh with `--no-fail-fast`, its whole output under .gate/u4df-merge/, the failing test's name
  and panic in the notes. A failure in npt-sim or in a crate that links it (npt-proto, npt-room, npt-server,
  npt-client, npt-lab) stops the merge and comes to me; any other failure is recorded under TE2-5, and the merge goes
  ahead only on a clean counted run, the earlier failure named in the notes.
- **M7, the gate:** `node tools/gate.mjs` before the commit; if smoke (5a) cannot start this container's Chromium, as
  at U4j's merge, `--no-smoke` with both logs kept and the reason in the notes. CI's gate and smoke must be green on
  the pushed head.
- **M8, one commit,** "U4d-F: the estimate scene fix (npt-sim), merged after review round 2": the five spikes/ files
  exactly as applied; `git rm` of wip/u4df.patch; and under .checkpoints/ only: u4df-build.md (with "Fix pass, round
  1" and "Merge into the spike branch", whose readings record R4.1's two corrections), u4df-report.md (new, beside
  u4d-report.md, folded later by U4's report; T1 is its measurement section), u4d-report.md's pointer line (T1's
  second block), u4s-carries.md's carries 67-72 and carry 59's sentence (T2), and the card. The texts are copied from
  this file's code blocks by script, never retyped: each old text found exactly once, each new text found in place
  after, no changed prose line over 120 characters. The commit's spikes/ part is the reviewed bytes and nothing else.
- **M9, after the push:** the saver restarted; the card updated (U4d-F merged at <sha>; base counts 399 / 446 / 52 for
  the next lanes, U4e's `<base>` and U4d2's included; U4d-F's carries 67-72 listed); then U4e's L3 (the carry-51
  commit to scene.rs) on the merged head, in its own commit; U4d2's brief bases on U4d-F's commit and names it
  (carry 52); and the card's "Rules in force" gains T4.

R5 written 2026-11-12.

### R6. Process: the reviewers' shell writes. A brief wording, no new rule. APPROVE as recorded.
- **What happened:** each reviewer first wrote part of its notes with a python one-off (the netcode-architect blocks
  [5]-[8], the test-engineer blocks [11]-[12]), then rewrote its notes whole with Write, and says so at the top of its
  notes. The standing rule (the card's "Rules in force", Standing: "write files with Write and Edit, never cd plus a
  redirect") was broken twice; neither seat hid it.
- **Harm: none found.** Only each seat's own notes file under .checkpoints/netcode-proof-test/ was written that way,
  the folder its role allows; the test-engineer's end check shows the copy byte-identical at the end (its 183-file
  hash list, [14]); I re-hashed the five changed files and they equal the builder's final list; the main folder's
  `git status` shows only this file new. The netcode-architect's probe crates live in its scratchpad, outside every
  copy, by its notes ([9]).
- **Why the rule matters anyway:** a file tool's write is what the guard and the permission check read; a shell write
  goes around them, and a python write can clobber a file the agent did not mean to touch.
- **What the briefs said:** the build brief carried the rule in words ("Write files with the Write and Edit tools only
  (no sed, no shell redirect, no python writes; ...)"); the review brief said only "you have no Edit tool, so rewrite
  the notes file whole with Write each time", which tells how, not what is forbidden, and the test-engineer had Edit
  and slipped too.
- **The ruling:** no rule beyond the standing one. Every agent brief from now on, reviewers' included, carries the
  build brief's sentence word for word (T4, item 1). An agent that slips says so at the top of its notes, as both did,
  and the main session checks `git status` and the copy's hashes after the run, as it did here. Severity: Low
  (process); no push of its own.

R6 written 2026-11-12.

## The texts (each copied by script from its code block; never retyped)

### T1. The report's measurement section, and u4d-report.md's pointer
**(1) u4df-report.md** (new, beside u4d-report.md; the main session writes its other sections in u4d-report.md's
form: who built it, the rounds, the rulings, the checks on the merged tree, the carries met and added by number, the
follow-ups, Decisions needed, For Mustafa). This section, as it is:
```
## The estimate scene after U4d-F (the advisor's U4d-F ruling: NF-M1, NF-L2, NF-L4, TF-L2, TF-L3)
- **The acceptance (the advisor's U4d close ruling, B1.2 step 4) is met as ruled.** Seeds 1, 2 and 3, the last 18,000
  of 21,600 ticks: gameplay events 9.878, 9.743 and 10.098 a second, each 300 s half between 9.580 and 10.220, shots,
  live projectiles and spawns in their bands, 48 soldiers and 15 vehicles on every tick, every hull within 1,400 cm
  (1,400.6 cm exactly on the 1 cm grid). `aim_one_in` is 40, by the rule written before the sweep (the largest
  smallest margin to the band's edges: 0.080 a second); 34, 36 and 38 each put one half above 10.5, and 48 passes
  with 0.036. The 9.5 floor did not move.
- **The drift is fixed.** After the reckoning alone (step 3) the rate still fell in every seed, from about 11.5 to
  about 4.3 a second over 12 minutes (measurement A), so the ruling's bounded re-home (3b) was added. After U4d-F,
  over 36-minute runs of seeds 1-9 (the netcode-architect's probe), the nine seeds' mean over each 300 s block from
  minute 2 is 10.096, 9.692, 9.941, 9.901, 9.799, 9.857 and 9.887 a second, the slope over minutes 2-35 is -0.003 a
  second per minute, and all 27 ten-minute windows are in band (the lowest 9.537).
- **The level sits a little below the band's centre:** about 9.88-9.89 a second. Out of sample, seeds 4-9's windows
  are all in band (9.693 to 10.171), and one of their 12 halves is out: seed 6's second, 9.446.
- **The halves inside the window are not flat.** This corrects the ruling's "that is what this step shows" (B1.2 step
  4): the second half is lower than the first in 8 of 9 seeds (by 0.404 a second on average; the halves average
  10.096 and 9.692), the same in every seed, and the dip does not recur after minute 12; the netcode-architect reads
  it as the hulls' loops, which start from the same point in every run. A single 300 s half's noise is 0.2-0.36 a
  second, so at this level the per-seed half check misses by noise about one half in 9, and three seeds' six halves
  all pass only about half the time: 40 is one passing set of halves, not a margin. Later re-measures follow the
  advisor's rule RM (carry 67): nine seeds, 36-minute runs, every seed's window in band, drift judged on the nine
  seeds' mean per 300 s block, single halves reported and not judged; the share is re-set only when that fails.
  Today's scene passes RM.
- **What the events are now:** about 69 % grenade blasts (6.6-7.0 a second), 21 % sky shots meeting a soldier on the
  shooter's chest (about 2.0), 7 % aimed hits (about 0.7, the only part the share moves) and 3.5 % kills (0.34-0.36);
  42 % of hits land on a teammate, as before. 3b makes 51.9 teleports a minute (3.9 before). Each team is re-homed
  inside one 20 s third of every minute, so its hits peak in that third and dip about 13 % before the next; every
  judged window is whole minutes, which averages it out exactly. The teleports force no correction anywhere today
  (synthetic recordings are replayed sim-only); a predicting client fed this recording would meet about 52 forced
  corrections a minute (carry 70). ES's alive soldiers change yaw on about 48 % of ticks and pitch on 18 %, against
  R2's moving-delta assumption, so no ES byte figure is taken from this recording (carries 68 and 69). The 15 hulls
  now loop inside the fight, where they are cover and block moves: at the old reckoning and share they cut the rate
  from 9.853 to 7.250 a second over T-M1's 300 s (step 2); carry 59 now names it for U4r.
- **What the reckoning is worth, and what guards it.** The old reckoning planted back (the test-engineer's P1, in the
  base's own lines, 3b and 40 kept) gives windows 9.358, 9.846 and 9.235 for seeds 1-3 (mean 9.480, against 9.906 as
  built): under 3b the reckoning is worth about 0.43 events a second, and t_m1 (seed 2) catches it only on one half
  (9.240), with the window in band. The builder's reading "P1 shows as a level, not a decay" held for its own plant
  form on seed 2 alone: a last-ulp difference in the estimate moves a half by about 1.1 a second. The reckoning's
  exact guard is the fix pass's test (TF-M1, FX-1), which each of the plants Q1-Q5 fails by name; t_m1 stays the
  tripwire that sends a later change to RM.
```
**(2) u4d-report.md:** at the end of its section "## The merge condition (N2-M1, the netcode-architect's round 2)",
after the "Routing" bullet, this bullet:
```
- **Fixed by U4d-F** (the commit "U4d-F: the estimate scene fix …"; u4df-report.md): N2-M1's drift and N2-M2's
  vehicles. Over the judged window (the last 10 minutes of 12), seeds 1-3 give 9.878, 9.743 and 10.098 gameplay
  events a second at `aim_one_in` 40, and the rate is flat over 36 minutes on nine seeds.
```

### T2. The carries (u4s-carries.md)
**(1) Carry 59's sentence (NF-L4).** In entry 59, the old text `script, run by run (ADR 0001:1480)."` (found once, at
:273) becomes the text below (the quote now ends after the new sentences):
```
script, run by run (ADR 0001:1480). Since U4d-F all 15 of ES's hulls loop inside the fight (the advisor's U4d close
    ruling, B1.3), where they are cover and block moves: on the synthetic recording, at its old reckoning and share,
    they cut ES's gameplay events from 9.853 to 7.250 a second over 300 s (U4d-F's step 2). The brawl meets the same
    hulls. U4r's report gives that figure beside its ES suite's event rate, and a missed event band is not read as a
    bot defect before the advisor has seen both."
```
**(2) §U4e, after entry 61 (its last):**
```
68. **NF-L1, ES's bytes and the synthetic recording (u4df-review-netcode.md; the advisor's U4d-F ruling, R3):** "U4e
    takes no ES snapshot byte or bandwidth figure from `Recording::synthetic`'s ES recording as a measure of ES's load.
    Since U4d-F that recording's alive soldiers change yaw on about 48 % of soldier-ticks and pitch on about 18 % (its
    sky aims sit nearly still while the soldier moves), against R2's moving-delta assumption of yaw in every moving
    update and pitch half the time (docs/research/r2-netcode.md:1751), so its bytes read lighter than R2's estimate.
    Any ES byte figure U4e reports from that recording states those two shares beside R2's."
```
**(3) §U4r, after entry 63 (its last):**
```
69. **NF-L1's U4r half (u4df-review-netcode.md; the advisor's U4d-F ruling, R3):** "U4r takes no ES byte or bandwidth
    figure from `Recording::synthetic`'s ES recording as ES's load: its judged ES runs are the bots' (carry 59). Any ES
    byte figure it reports from that recording states the recording's churn beside R2's moving-delta assumption: yaw
    changing on about 48 % of alive soldier-ticks and pitch on about 18 % (U4d-F's scene), against yaw in every moving
    update and pitch half the time (docs/research/r2-netcode.md:1751)."
```
**(4) §U4g, after entry 47 (its last):**
```
70. **NF-L2, the synthetic ES recording's teleports (u4df-review-netcode.md; the advisor's U4d-F ruling, R3):** "Since
    U4d-F, ES's synthetic recording teleports every soldier home once a minute (3b's re-home: about 52 teleports a
    minute with the random ones, against about 4 before). A predicting client fed that recording meets each one as a
    forced correction that no network caused, about 52 a minute. Any U4g test or figure that predicts on it counts
    teleport corrections apart from the others and reports both."
```
**(5) A new section, after entry 64 (the end of "## The next change to crates/judge (whoever makes it; its first
step)") and before "## U4f (clock and commands)", with a blank line before and after:**
```
## Any later change to npt-sim's synthetic recordings, or to the World's outcome on them (whoever makes it)
67. **NF-M1 and TF-L2, the ES re-measure RM (u4df-review-netcode.md, u4df-review-test.md; the advisor's U4d-F
    ruling, R1):** "Before the merge of any change that moves the estimate scene's synthetic recording (D-7's ES
    digest) or the World's outcome on it (t_m1's printed line differs from the base's), and at once whenever t_m1
    fails, the change's owner runs RM and its report gives the numbers. (1) Runs: seeds 1 to 9, each a 36-minute ES
    recording (64,800 ticks from tick 1,000,000), replayed and counted as tests/world.rs's `EsRun` counts (an
    `#[ignore]` test run by name, or a scratch crate on npt-sim's public API outside every copy). (2) The floor, run by
    run: each seed's window (minutes 2-11) meets PM-0001-2's four ES bands (shots 68.4-75.6 a second, gameplay events
    9.5-10.5, live projectiles 30.4-33.6 a tick, spawns 20.064-22.176 a second), with 48 soldiers and 15 vehicles on
    every tick and every hull within 1,400 cm of the cluster's centre. (3) Drift: the nine seeds' mean gameplay-event
    rate over each 300 s block from minute 2 to minute 31 (six blocks) lies in 9.5-10.5; the least-squares slope of the
    nine seeds' mean per-minute rate over minutes 2-35 is reported beside it. (4) Every seed's two 300 s halves of its
    window are reported, not judged: a single half's noise is 0.2-0.36 a second at ES's level. (5) `aim_one_in` is
    re-set only if (2) or (3) fails at the current value, then to the value whose nine-seed mean window lies nearest
    10.0 with (2) and (3) passing at it, never on three seeds' in-sample margins; the numbers come to the advisor
    before D-7 or t_m1 is re-pinned for it. (6) If t_m1 fails while (2) and (3) pass, neither the share nor t_m1
    changes: the numbers come to the advisor, who rules on t_m1's form. The band and its 9.5 floor do not move
    (Mustafa, 2026-11-11; carry 52 quotes him). At U4d-F's merge the scene passes RM: windows 9.693-10.171 on seeds
    1-9; blocks 10.096, 9.692, 9.941, 9.901, 9.799 and 9.857 (u4df-review-netcode.md [10]; .gate/u4df/nc-long.log)."
71. **NF-L3, 3b's switch (u4df-review-netcode.md; the advisor's U4d-F ruling, R3):** "The generator's re-home (3b) is
    switched on by a scene's `aim_one_in` above 1 (replay.rs's `synthetic`; scene.rs's `aim_one_in` doc says so), and
    only ES sets it above 1. A change that would let another scene set `aim_one_in` above 1, or set ES's to 1, first
    gives the re-home its own pub(crate) scene knob (its period, 0 for none), in that same change and with the
    advisor's approval of its symbols. Until then no knob is added."
72. **TF-L1, a_m1's non-vacuity (u4df-review-test.md; the advisor's U4d-F ruling, R3):** "a_m1
    (tests/world.rs:940) is not vacuous only because SS seed 5's 1,200 ticks hold about two deaths (58 dead
    soldier-ticks: the test-engineer's U4d-F probe), and two generator-only plants took them away. Before the merge of
    any change that moves SS's synthetic recording, a_m1 is made independent of the recording's draws: it scripts its
    deaths through its own inputs, or runs long enough for at least 10, and asserts that count by name."
```

### T3. FX-2's doc words (scene.rs; one line each here: the fix pass applies them by Edit at the doc's indent and rewraps to 120)
**(a)** appended to `ES_AIM_ONE_IN`'s doc, after "(0.080 a second; 48's is 0.036).":
```
Over seeds 1-9 and 36 minutes the rate is flat at about 9.88 a second, and a 300 s half's noise is 0.2-0.36 a second (U4d-F's review), so 40 is one passing set of halves, not a margin: a later change that moves ES is re-measured by the advisor's U4d-F ruling (RM), never re-tuned on three seeds.
```
**(b)** inserted in `aim_one_in`'s doc after "U4d close ruling, B1.2)", before the full stop:
```
; it moves only the aimed hits, about 0.7 of ES's 9.9 a second at 40 (grenade blasts and sky shots meeting a soldier on the shooter's chest make the rest)
```

### T4. The card's "Rules in force", a new bullet after the U4j round-2 ruling's
```
- **The advisor's U4d-F ruling (advisor-u4df.md, R1, R4.1 and R6):**
  1. Every agent brief, builders' and reviewers' alike, carries this sentence word for word: "Write files with the
     Write and Edit tools only (no sed, no shell redirect, no python writes; never `cargo fmt` writing files: apply
     rustfmt's diff by Edit); an agent with no Edit tool rewrites its notes file whole with Write."
  2. A builder brief's TE2-5 stop clause reads, with the unit's crate named: "a failure in <crate>, or in a test that
     reads <crate>'s files, stops your step group (report it), unless your step's plan names it as expected (a pin
     your step re-pins, or a test a later step rebuilds): then record it under TE2-5 with its cause and the step that
     ends it, and go on." B2.3's merge stop is unchanged.
  3. RM (carry 67) for any change that moves ES's synthetic recording or the World's outcome on it.
```

## Before the merge (in this order)
1. **The fix pass** (R2): FX-1, the test `the_sky_aims_reckon_every_input_the_recording_emits` in src/replay.rs's new
   test module, with plants Q1-Q5 each failing it by name; FX-2, T3's doc words in scene.rs. Its evidence: lint in
   both builds and targets, the suites with every pin unchanged, the three-seed test's 21 lines unchanged, check.mjs
   twice counted in the copy at 368 / 415 / 52. Recorded in u4df-build.md, "Fix pass, round 1". The builder's brief
   carries T4's two sentences.
2. **Round 2, on what changed** (R2): netcode-architect (read only) and test-engineer (with Edit), Opus at xhigh, from a
   packet cut by review_packet.py; both briefs carry T4 item 1; the test-engineer saves its start hashes under
   .gate/u4df/ and runs check.mjs once counted (368 / 415 / 52).
3. **Round 2's outcome:** APPROVE, or APPROVE WITH FINDINGS with Lows only, lets the merge go ahead; a Medium or above,
   or a REVISE, comes to me first.
4. **M1-M8** (R5): the copy is the one reviewed; the base still fits; the snapshot and apply; the merged bytes equal
   the reviewed bytes; check.mjs counted in the main folder at measured 399, bites 446, self 52, with the three-seed
   test's lines and FX-1's counts; TE2-5's rule; the gate; one commit with T1 and T2 applied by script and R4.1's
   corrections in the record.
5. **M9 after the push:** the saver restarted; the card (base 399 / 446 / 52; carries 67-72; T4 in "Rules in force");
   then U4e's L3 on the merged head; U4d2's brief names U4d-F's commit.

## Decisions needed
None. Nothing in this ruling moves PM-0001-2's floor, changes a word of Mustafa's, or changes a rule of his: RM
replaces only my own per-seed half check, and only for later re-measures.

## For Mustafa
Nothing.

## What I checked, and what I did not
**Checked myself (reading, git and sha256; no build or run):**
- My role file from main, whole; the brief; both reviews and the builder's record, whole; my U4d close ruling
  B1.1-B1.6; advisor-u4j-r2.md B1 (M1-M9) and B2.3; the build and review workflow scripts' rule sentences; the card's
  "Running" and "Rules in force"; u4s-carries.md's sections and entries 51-66; Mustafa's words at history.md:338-339
  (equal to the packet's quote); the packet's plan sections §4.13-§4.15 and its carry 52.
- The copy: HEAD dad9566, `git status` (the five files and the record), the five files' sha256 equal to the builder's
  final list and the test-engineer's [1]; its `git diff -- spikes/` byte-identical to wip/u4df.patch (sha256
  755cd562…, also at HEAD's committed version; equal patch-ids); `git apply --check` clean in the main folder at
  1148c45; crates/sim unchanged from dad9566 to the spike head; no other crate uses the changed symbols; npt-sim's
  dependants.
- The code read: scene.rs's `ES_AIM_ONE_IN` and its doc, `fight_vehicles`, `grenade_fuse_ms` and `aim_one_in` docs;
  tests/world.rs's `EsRun`, t_m1 and the three-seed test (every band literal as the base's); replay.rs's `synthetic`
  whole (the phase-0 order, the held-back move, `after`, the sky aim, the send / hold / lose arms, 3b's loop);
  world.rs's `move_velocity` and run_command's call, and the bite's two lines old and new.
- The logs: nc-long.log (I re-derived the block means, the slope, the 27 windows and the 6 of 54); te-plant-p1.log,
  te-plant-p3.log and te-probe-seeds4-9.log (their panic lines); end-check4.log, end-check5.log and te-check.log (all
  10 steps PASS, 367 / 414 / 52); s2-, s3-, s4- and s5- first and re-run logs (R4.1's two corrections).
- ADR 0001:1478 and :1480 (ES's rates, "on every ES run") and :1600 (PM-0001-12's ES runs: 10 minutes, 3 runs);
  docs/research/r2-netcode.md:1751 (the moving delta's "yaw 16, pitch half the time").

**Not checked, and why:**
- I ran nothing that builds or tests (the brief's rule). Every run figure is the builder's or a reviewer's, from their
  logs; I re-derived only nc-long.log's arithmetic.
- The netcode-architect's churn shares (48 % and 18 %), its event split (sky, aimed, blast, kills), the 51.9
  teleports a minute and the team-phased hit table come from its probe (nc-probe.log), which I did not re-derive; the
  test-engineer confirmed 51.9 by arithmetic only.
- The reading of the in-window dip as the hulls' loops is likely, not proven; nothing in this ruling depends on it.
- FX-1's run time and its six counts are my estimates until it runs (I expect about ten held-back commands carried on
  a re-homed or re-sped tick in seed 2's 21,600 ticks); its fall-backs (more seeds, or 7,200 ticks) are written in.
- The packet's diff sections I read through the copy itself (byte-identical to the patch the packet was cut from);
  its round-2 notes sections I read at U4d's close, not again now. The test-engineer's scratchpad hash lists I did
  not open.

Ruling complete 2026-11-12.
