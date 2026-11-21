# Advisor round ruling: U4f2's review round 1 (2026-11-20), from main's advisor.md

Read only. Role: main's `.claude/agents/advisor.md` (`git show origin/main:.claude/agents/advisor.md`, read whole).
Diff: wip/u4f2-build.patch (base a289def) and the builder's worktree
(.claude/worktrees/agent-aa62d31b6fa5b9807, read by absolute path). Nothing run but git/grep/sed/wc/sha256sum.

## Plan
1. Confirm the diff (patch hash) and read the code at each High and Medium (world.rs dead return and revive; forged.rs
   kind-7 rule; bots forge_step, FIRE_REST, RELOAD_ITEMS, forge_input's twin arm; scenario.rs lag client, f16 pair, f20;
   logs.rs lagswitch excess; marks.json -21 checks; SERIES.md:158).
2. F-B (NC-U4F2-1 = TE-U4F2-1, High): the cause in code; route (a) inside the plan or his under TEST-FIRST; other
   routes the rules allow; the proof owed.
3. Every other finding: F-A, F-E, FIRE_REST, the shared pair, F-C, the pre-F-D hand runs, the 1,207 s margin, every Low.
4. The builder's departures (the five the panel brief names, `HOLD_STEPS` included).
5. The fix pass: FP-n with acceptance checks and plants; size against 3,000 (2,077 now); who builds; runs owed against
   the disk (about 10 GB free, 4 GiB floor, about 6.3 GB per check.mjs run).
6. The round decision.
7. What goes to Mustafa, in which message; the paste-ready question if any; "Choices taken without him".
8. Tables: ruling per finding, FP-n, round decision, the Mustafa line.
Each block is added below as it finishes.

## Block 0: what I checked (DONE)
- **The diff:** wip/u4f2-build.patch sha256 e9a0ae3e3f48af18ea7a947fdf9aba849061fa32fbd91876a948c46f3046406b (my
  `sha256sum`), the one the panel brief names (its line 11 now prints all 64 hex). The builder's record is the same file
  in both copies (main folder and worktree: sha256 7d7ff18c…). Free now: 9.5G (`df -h /home/user`).
- **Read whole:** main's advisor.md; TEST-FIRST (history.md:2234-2282); the four seats' files; the panel brief; the
  brief rev 1.2 and my check of it; the build record; advisor-u4f-f1-r1.md (the form); the card; LEAD-BOUND's decision
  with his yes to A1 (history.md:6791-6806); the 2026-11-17 one-round choice (:6818-6819); u4f-brief.md 2.4 and 2.6
  (:410-546) and its scale rule (:667-680); U4F-Q15 (:1611-1625); ADR 0001:1611 (PM-0001-21's row); marks.json -21's
  parts and checks (:4805-4999); SERIES.md:158-159 and U4f2's new rows (:392-398).
- **Code read at each High and Medium** (the worktree, `W` = .../agent-aa62d31b6fa5b9807/spikes/netcode-proof):
  - F-B: W/crates/sim/src/world.rs:1346-1355 (`claimed` set at :1351, then the `!s.alive` return at :1353, before the
    weapon at :1412-1499 where every Reload, Cooldown and Ammunition record is made), :1192-1203 (`revive`: `wait` 0,
    `reload_left` 0, magazine full); W/crates/lab/src/forged.rs:86-91 and :117 (kind 7 rejected only on a sim Cooldown
    or Reload record of its sequence); W/crates/bots/src/lib.rs:147-153 (`RELOAD_ITEMS` 40, `FIRE_REST` 3), :886-918
    (`forge_step`: `rest` releases fire on d-3..d-1 for fire items only; the honest edge at d-1; the forged edge at d
    with fire held), :1046 (`Kind::Steady => [0, 0]`), :1049-1054 (fire held).
  - The run: .gate/u4f/u4f2-hand-long2/l1-21-forged-a-s1/lab.jsonl:52-62: every kind sent 1,523 (kind 10 1,656),
    unrejected 0, but kind 7 (slot 6): 18. Its tree a284ae45… (after F-D; before the doc fix and the rename, neither of
    which changes code that runs).
  - F-E: bots lib.rs:279-282 (the 1|2|4|5|6 arm forges with no twin guard), :909-910 (`skip` and `f.due`), :919-923;
    W/crates/room/src/room.rs:1085-1087 (option A drains, no re-run on an empty tick).
  - F-A: W/crates/lab/src/scenario.rs:1265-1267 (the lag client wraps the run's behaviour, now steady), W/crates/lab/
    src/logs.rs:694-710 (excess = the tick's movement less 8 ticks at full speed), scenario.rs:4224-4225 (f20's `<= 0`);
    bots lib.rs:630-648 (`lane` keeps its `west` in `Fast`).
  - FIRE_REST and SA-U4F2-1: bots lib.rs:147-153, :886-892, :944 (`f.every * u64::from(f.burst_refused())`), :551-552.
  - The shared pair: scenario.rs:4105 (`pair`), :4364-4420 (f16: the panic names the test's kind; the link check
    :4392-4398).
  - The hand runs' trees (TE-U4F2-7): u4f2-hand-a.log:3 diff d211149a… (before F-D), u4f2-end-b.log:3 diff 4a9dde09….
- **Nothing run** but sha256sum, sed -n, df, git show; no cargo, check.mjs, npt-lab or npt-judge.

## Block 1: F-B (NC-U4F2-1 = TE-U4F2-1, High) (DONE)
**The cause holds in the code**, in two halves (the seats agree; the code above confirms each step):
1. **Dead at d.** A dead soldier's command claims its press counters (world.rs:1351) and returns at :1353, before the
   weapon: no record. The kind-7 rule (forged.rs:86-91, :117) needs a Cooldown or Reload record, so the item counts
   unrejected. The press has no effect (claimed, so nothing is banked for the revive: the security seat's block B).
2. **Revived between d-1 and d.** `revive` (:1192-1203) leaves the weapon ready. Fire item, d-1 run dead and d alive:
   the forged press fires; the held trigger of the twin fires the same shot, so no hash difference, but no record.
   Reload item, d-1 the first live command: the magazine is full, so the honest reload edge is refused; fire is held on
   d-1 for a reload item (`rest` covers fire items only, :892), so d-1 fires and leaves the magazine short; d's forged
   reload edge then starts a legal reload with no record, and a state the twin lacks (`reload_left`, FLAG_RELOADING):
   a twin-hash difference (NC point 2; the security seat's block B missed the d-1 case, saw only the d one).
3. **Why only long runs:** C-U4F2-5 (c) stands every steady soldier on fixed blind aims whose rays run to the reach
   (NC point 3); slot 6 dies several times in 610 s, never in f16's 6 s or the hand runs' 15 s. The figure 18 of 1,523
   is unverified as "all dead-window": the proof below settles it.
4. **What it costs as built:** -21's "forged: every forged command rejected and counted" (marks.json:4958-4969, max of
   every kind's `unrejected_count` == 0) fails on every 600 s forged run for a reason in the test; a forged no-* bite
   then reads FAILS whatever it reaches (the 11 forged bites lose their power); the reload half can also fail the twin
   figure (rare). High stands.

**The routes the rules allow, weighed:**
- **(i) Close the reload half by construction, inside the plan (mine): the reload item's honest command d-1 releases
  fire.** At d-1 in the normal case the honest reload edge starts a reload (:1426-1434), so `ready` is false and the
  held trigger fires nothing: releasing it is a no-op there. After a revive (magazine full) nothing fires at d-1, the
  magazine stays full, and d's forged edge meets it: refused, a Reload record (:1427-1431), no state the twin lacks.
  Every case checked: dead d-2 with d-1 first live (refused at d, recorded), dead d-1 with d alive (refused at d,
  recorded), alive throughout (refused at d: reload in progress). It removes the twin hazard whole, so NC's (b)
  (`RELOAD_ITEMS` about 400) is not taken: it only shrank the hazard and cut the reload items per run from about 38 to
  4. This is the forger's construction (2.6's kind-7 row, u4f-brief.md:525: "a fire or reload press edge where the sim
  refuses it"), as F-D's rebuild was: open loop, a function of seed, slot and step, the twin's d-1 equal to the forger's.
- **(ii) The dead half cannot be closed by construction.** A press lands in a cooldown only after a shot, and a shot
  needs a live d-1; an open-loop script cannot know the soldier died. So every remaining route changes what the check
  counts or how kind 7 counts as rejected:
  - **(a) NC's: leave the item out.** An item is left out of kind 7's `sent_count` and `unrejected_count` when its
    forger's soldier was dead, by the server's own World (its Soldier record before the tick's command phase), when
    the item's command d or the command one sequence before it (d-1) ran; counted apart in a new report-only
    `event.forged_kind.dead_left_out_count`. With (i) in, {d-1, d} is exact for fire items (dead only at d-2 or d-3:
    d-1 fires from a fresh weapon and d is refused) and over-covers reload items by a negligible share (dead at d-1
    only: d is refused anyway); one rule for both is simpler to state. An item whose d-1 has no run is not left out (no
    missing run can hide an item). Smaller than NC's d-3..d window, which (i) makes unnecessary.
  - **(b) A sim record for every press a dead soldier's command carries** (a new refusal kind counted under kind 7):
    keeps "rejected and counted" literal, but opens crates/sim (shut in U4f2's lane: u4f2-brief.md:172), widens kind
    7's own-kind rule (his, by C-U4F2-10's own words), and records honest players' presses while dead as refusals.
    About 60-150 lines and a sim review.
  - **(c) Counting a dead-window item as rejected with no record:** calls an uncounted command "counted" (row 159's
    "rejected-and-counted"); weaker than (a), not offered.
  - **Ruled out, as the seats say:** the forger reading its own dead bit (closed loop, against u4-build-plan.md:411-412
    and U4F2-Q1; and the view lags the tick the command runs at); new aims for Steady (Q1 point 1 keeps the aim, carry
    86's shot band); moving the forgers' spawns (the scene's, crates/sim); picking forger slots that happen not to die
    (fitting the setup to a run's result).
- **Is (a) mine or his? His (TEST-FIRST), and the fix pass stops on it for that one item.** Reasons:
  1. It narrows the population of a judged check: ADR 0001:1611's "every forged command of ... part 5's set ...
     rejected and counted", read through marks.json:4959's check of the same words. The text and the 0 stay, but a
     category of forged commands leaves the count. TEST-FIRST keeps "every pass mark and its number" exactly and
     leaves "any change to a pass mark" to him (history.md:2262, :2270).
  2. It changes what an existing SERIES.md row's value is: SERIES.md:158's `sent_count` "forged commands sent" would no
     longer count every forged command sent. His words forbid that without asking: LEAD-BOUND (history.md:6801: "no
     existing SERIES.md row ... If the build needs one of those to change, stop and ask me first") and SIZE-U4F (a),
     carried in u4f2-brief.md:173-175.
  3. The precedent is exact: A1 read "moves" in the same ADR row, leaving dead and respawn ticks out and counting them
     apart; it went to him and he said yes in his own words (history.md:6799). And it would be made after a judged-length
     run failed the check, the pattern his freezes guard against (LEAD-BOUND, history.md:6797: "It is never changed to
     fit a run's result, and a change comes back to me").
  4. NC's own alternative test ("if the advisor reads it as changing what SERIES.md:158's row means, it is his") is met.
- **My lean for him: (a).** It keeps the kind-7 rule exact (a record of its own kind), keeps every refused item's
  evidence, reports what it leaves out beside the check, leaves the 1,000-per-kind check on what remains (about 1,500 a
  run), and keeps the twin check over every item, the left-out ones included (a dead press has no effect; a revived d
  fires as the twin's held trigger does). Like A1, the reading joins the readings PR's list (plan :971-974) and -21's
  report names it in a line of its own with who reviewed it.
- **What waits and what does not:** only FP-2 (the count) waits for his answer; FP-1 ((i)) and every other item go now.
  The 610 s pairs, the end checks and round 2 run on the tree with FP-2 in, so nothing expensive runs twice.

**The proof owed (TE-U4F2-1), in this order:**
1. **A unit-level death test through the real World** (FP-1, then extended by FP-2; at most about 5 s alone in debug):
   a kind-7 forger's commands and its twin's, each fed to a World, with the forger's soldier killed and revived by an
   opposing soldier's fire (or a set-up the test picks) so that items fall with d or d-1 run dead, a reload item has
   d-1 as the first live command, and other items run alive. Asserts: every item whose d-1 and d ran alive carries its
   Cooldown or Reload record; the reload item after the revive carries a Reload record; every tick's state hash equal
   between the two Worlds. Plant: today's reload construction (fire held on d-1) fails it by name. With FP-2: the
   attribution over its records leaves the dead-window items out, counts them apart and reads unrejected 0; plant: the
   window ignored (today's rule) fails it by name. An f33 case on hand-made inputs pins the rule beside it.
2. **The 610 s forged pairs on the final tree** (seed 1, options A and B, measured and twin, judged in 5.4's
   measured-and-twins form): every kind sent at least 1,000; every kind's `unrejected_count` 0 (this is the proof that
   all of the 18 were dead-window items: a live item without a record would still read here); kind 7's
   `dead_left_out_count` reported (about 15-30 expected, no fixed figure); twin mismatch 0 on both; PM-0001-2's shots in
   the band; slot 6's `commands_rerun_count` 0 under B. Any kind unrejected above 0 stops the record and comes to me.
3. **One forged bite at length:** bite-no-lead-bound at 610 s (seed 1, A): kind 1 the only kind unrejected (the bites'
   power restored), and its shots (TE-U4F2-6, block 2).

## Block 2: every other finding (DONE)
- **F-A, NC-U4F2-3 = TE-U4F2-5 (Medium): fix, FP-4 (route (i)).** Confirmed: the lag client wraps the steady script
  (scenario.rs:1265-1267), which now stands, so `lagswitch_excess_move_um` is minus 8 ticks of full speed on every tick
  (logs.rs:699-706) and f20's `<= 0` (scenario.rs:4225) cannot fail. What it costs beyond U4F-Q15 (c): at the judged
  200 ms the check could not fail by the cap (about 6 commands under 8), but a walking client still let it fail on a
  tick that moved more than its commands allow (a command run twice under option A: 12 ticks of movement against 8);
  standing, it fails on nothing, and p19's 400 ms figure and f20 lose their movement half. The fix: the lag-switch
  suite's slot 0 walks the Lane band (bots lib.rs:630-648: full length, a band no box, door, wall or vehicle part
  reaches, from the fast clock's own slot-0 spawn path). Closed loop is allowed there: the suite has no twin, and the
  open-loop rule binds the forged and clamp suites (u4-build-plan.md:411-412). **Against U4F2-Q1 point 1:** that point
  bound C-U4F2-5's route; it does not forbid another finding's fix from moving the Lane's `west` out of `Fast` so a
  non-fast client can use it, provided the fast clock's commands do not change (FP-4's acceptance proves it). The
  route-(ii) record line alone is not taken: a judged check that cannot fail by construction is evidence that does not
  hold. -21's report keeps U4F-Q15 (c)'s 200 ms arithmetic line.
- **F-E, NC-U4F2-4 with TE-U4F2-2 (Medium): fix, FP-5.** The code supports NC's reading: the 1|2|4|5|6 arm of
  `forge_input` has no twin guard (bots lib.rs:279-282) while `f.due` is cleared only through `skip` (:909-910), so a
  plant that just stops the skip makes the twin forge as the forger does, and both runs agree; under option A a twin
  that really sent the honest content would run one command more (room.rs:1085-1087: no re-run on an empty tick) and
  move the weapon clock. So plant 9 as run most likely proved nothing either way; F-E is shown for option B only by
  reading (input.rs:473's re-run equals the standing honest command). FP-5 adds the guard, the bots case that pins
  U4F-Q13 (b)'s twin for those five kinds, and plant 9 re-run in its right form under both options.
- **`FIRE_REST`, NC-U4F2-2 = RR-U4F2-1 (Medium), with SA-U4F2-2 (Low): fix, FP-3.** Confirmed by the arithmetic of
  world.rs:1415-1416 and :1440 (cost = interval x TICK_HZ; the clock falls 10^6 a command, clamped at 1 - 10^6): the
  construction holds only while the interval lies between 2 and the rest in ticks, and the rest must not exceed
  `every`. At 60 Hz (4.11 ticks) and at `--forge-every 2` (30 Hz) fire items go unrecorded: false FAILs, never a false
  pass, but against carry 39 and plan A5/RQ6 for a value the 60 Hz change-set would not touch. The doc's "more than a
  tick" is wrong ("at least two ticks"). RR's form (a const fn of the rate and the weapon, pinned at 30 and 60 Hz, kind
  7 refused below it) is the fix; C-U4F2-1's parser floor of 2 stays as ruled.
- **The shared pair, TE-U4F2-4 (Medium): the departure accepted, and fix FP-7.** Sound (any twin defect fails all 11
  tests of its option: no false pass), closer to the judged runs (all 11 forgers in one run), and the only way under
  the 1,207 s line (22 pairs about +190 s: the build record's first finding). Its cost, a failure that names the test's
  kind and not the culprit (plant10.log:21-25), is cured by FP-7's message (0 s).
- **F-C, TE-U4F2-6 (Medium): no code; runs before the merge (R2, R3).** 677 and 685 at 600 s, 668-669 on the
  no-lead-bound bite at 5-20 s, against PM-0001-2's 665 floor (marks.json:285-297). The pairs under A and B and
  bite-no-lead-bound at 610 s report their shots; a mean under 670 on any of them comes to me before -21's set, with
  the levers inside the plan named (kind 1's k range, `RELOAD_ITEMS`, U4F-Q10's option for that bite). The band is his
  and does not move.
- **TE-U4F2-7 (Medium): fix, in the end checks (R5).** Confirmed: hand-a and hand-b ran on diff d211149a… (before F-D),
  the final tree is 4a9dde09…, and hand-long2 (a284ae45…) is one measured run with no twin. Section 6 item 5 (every
  run-script line by hand, both judge calls in 5.4's form) re-runs whole on the fix pass's final tree.
- **TE-U4F2-8 (Medium): ruled, no code.** The 1,207 s line (u4f-brief.md:678-680: 1.5 times 5b29296b's step, a stop
  for me) reads the builder's two counted check.mjs runs in its worktree, as A3 and B read it; each must be under it.
  The main folder's merge check prints its step time beside, not against the line (equal tests read 931.9 s in the
  worktree and 1,007.5 s in the main folder at 1faf894: a known 8 % setup spread). The fix pass's new tests add at most
  30 s to the worktree's step in all (each one's time alone written, the brief's rule: unit level, or an existing
  pair); past 30 s, or a worktree counted run past 1,207 s, stops for me. U4g re-bases its own line
  (u4g-brief.md:206-207).
- **TE-U4F2-3 (Medium): fix, FP-6.** Confirmed: q4's other-slot record names seq 3, which the forger never forged
  (forged.rs:345-349), so removing either slot filter (:81, :86) passes every test, and a cross-slot record can only
  mask (mark an item rejected).
- **The Lows** (the budget: a Low rides a push needed anyway only where its file is open and it is cheap; else a record
  line or a follow-up):
  - SA-U4F2-1 = TE-U4F2-15 (`--forge-every` unbounded, kind 10's multiply): **fix, FP-8**, with RR's refinement
    (`forger` returns `None` when `every.checked_mul(8)` fails) and a top bound in the parser (at most 1,000,000).
  - SA-U4F2-2: **fixed by FP-3** (2 refused for kind 7 at 30 Hz).
  - SA-U4F2-3 (kinds 8, 9, 11's silent clip: with RR-U4F2-2's lap, one of the attribution's two false-pass paths, none
    reached today): **fix, FP-8** (`counted_count` on
    their `forged_kind` records, a new row in U4f2's SERIES.md subsection, f16 asserting it equals `sent_count` at P0).
  - RR-U4F2-2 (the 16-bit key past one lap, a false pass at intervals dividing 65,536): **fix, FP-8** (a reused
    sequence counts unrejected; the module text names the one-lap limit; one f33 case).
  - RR-U4F2-3 with NC-U4F2-5's usage half (`--suite forged` off P0 or with `--lead`): **fix, FP-8** (usage errors,
    two cases in c_u4f2_1's test). NC-U4F2-5's header param is **not taken**: a record line (the forgers stamp at P0's
    fixed lead of 2, their `lead_now_count` -1 by design; kind 10's burst relies on P0's in-order arrival).
  - RR-U4F2-4 (`RefusalRecord`'s fields): **fix, FP-8** (three doc lines; npt-room is open for C-U4F2-3).
  - NC-U4F2-6 = TE-U4F2-14's doc part (`twin()`'s doc on the skips' timing): **fix, FP-8** (2 lines).
  - TE-U4F2-9 (`refusals_dropped_count` pinned only at 0): **fix, FP-8** (one f33 line with a nonzero input).
  - TE-U4F2-10 (no -21 row's `expect` names "in a building: 0", which u4f2-brief.md:110 asks): **fix, FP-8**.
  - TE-U4F2-11 (set 21's projection 56 MB against 65.2 MB measured): **fix, FP-8** (the measured figure; it sizes the
    set against a tight disk).
  - TE-U4F2-12 (2.5's zeros asserted for slot 9 only): **fix, FP-8** (f16 on the shared pair, 0 s; a nonzero count
    there is a finding for me, not a test to loosen).
  - TE-U4F2-13 (`skip_command`'s controller half): **no code.** No -21 client skips on a controlled endpoint (the
    forgers and twins run fixed-lead endpoints); a record line, and a follow-up by id for the first unit whose runs
    skip on a controlled endpoint (none planned in U4).
  - TE-U4F2-14's record parts: **record lines (FP-9)**: each plant's file:line in the record (the eleven and the new
    ones), the short C-U4F2-5 copy passing (b)'s 1/32 walk that the full test failed (u4f2-b7-walk-fix.log:30, :52).
    The panel brief's hash slip is already fixed (its line 11).
  - NC-U4F2-1 (b) (`RELOAD_ITEMS` about 400): **not taken**, superseded by (i) (block 1).

## Block 3: the builder's departures (DONE; the five the panel brief names, u4f2-panel-brief.md:36-39)
1. **f16 shares one 16-client pair per option (11 forgers + 5 honest), and the clamp pair is shared by f18, f28 and
   f16: accepted**, with FP-7 (block 2, TE-U4F2-4). The budget reason holds (the record's first finding: 22 pairs
   about +190 s against 275 s of room).
2. **The forgers run on fixed-lead endpoints: accepted.** They keep kinds 2 and 5's "previous command accepted"
   (C-U4F2-1, bots lib.rs:549-550), match 2.6's table ("at P0's lead of 2", u4f-brief.md:526) and touch no check
   (-21's read the room's refusals; the controller is -20's). Its record gap is NC-U4F2-5's record line and FP-8's
   usage errors (P0 only, no `--lead`).
3. **C-U4F2-5 took route (c), Steady stands: accepted, as U4F2-Q1 ruled it** ("(b), or (c) if (b) cannot be shown;
   never (a)"). (b) was tried and shown not to hold with figures (u4f2-b7-walk-fix.log: a 1/32 circle still enters),
   the plant (today's walk) entered in every pattern (u4f2-b7-walk-today.log), so the test can show the fix, and the
   full test reads 0 in all 12 (seed, pattern) pairs (u4f2-b7-stand.log); only Steady's `move_xy` changed
   (bots lib.rs:1046; point 1). Its costs, which the ruling could not weigh, are each ruled here: F-B (block 1), F-A
   (FP-4), F-E (FP-5). U4f2's report shows point 1 from `git diff 25d316e -- crates/bots/src/lib.rs` outside the test
   module, naming FP-4's moved `west` as F-A's fix, not C-U4F2-5's.
4. **Kind 7 rebuilt so `PM-0001-21/no-cooldown` reaches it (F-D): accepted.** The first shape's fire edges lay inside
   its own reload, where the bite relaxes nothing (world.rs:1449-1455), so the bite would not have failed at length;
   the rebuild meets 2.6's kind-7 row and the bite reaches it (u4f2-hand2: kind 7 31 of 36 unrejected under it). FP-1
   and FP-3 amend it (the reload item's d-1, the rest from the weapon and rate).
5. **`CATCHUP_HOLD` renamed `HOLD_STEPS` to pass check.mjs:760's name rule, rustfmt's diff applied by Edit:
   accepted.** Both the netcode and rust seats read it as a step count, not a tick count: the burst's 8 accepted
   commands over the episode's 8 steps leave the lead unchanged at one step a tick at any rate; its value comes from
   `REDUNDANCY_CAP` (8), rate-free like `CMD_CATCHUP`, the one name the rule exempts; 6 at 30 Hz and 6 at 60 Hz. The
   old name met the rule only through "CATCHUP" (the kind's name). The other route, an exemption in check.mjs, edits a
   gate file outside the lane and under KIT-FREEZE-2. The carry-39 record (name, definition, 6 and 6, site
   bots lib.rs:156; u4f2-build.md:226-231) is the right form. U4f2's report names it in a line of its own: a name
   changed to pass the gate's rule, with the two seats' reading. `FIRE_REST`, unlike it, is a real rate-derived count
   (FP-3).
- **Not a departure, noted:** the builder's step-0 slip (`cd && …` with `;` and `|`, read-only git and a `merge
  --ff-only`), said at the top of its record as the rules ask: no effect, no action. The bots part ran about 3 times its
  estimate (u4f2-build.md:56-57): the whole unit stayed at 2,077 against the 3,000 stop.

## Block 4: the fix pass: who, order, size, runs and disk (DONE; the list with acceptance checks is the FP table below)
- **Who:** the builder, resumed by SendMessage in its own worktree (.claude/worktrees/agent-aa62d31b6fa5b9807; its tree
  4a9dde09… plus forged.rs 7919200c…, checked first: `git status`, no plant left), Opus 5.5 at xhigh (netcode). Notes
  as a new "Block 9: the fix pass" in its u4f2-build.md; a snapshot to wip/u4f2-build.patch by worktree_patch.py at
  each FP end; each plant by Edit, one `-x -k` runner call, restored by Edit with the file's sha256 checked; runner
  u4f-run.sh, logs .gate/u4f/u4f2-fp<n>-*.log; the shared target under the lane lock; one command per call in the
  simple forms; the brief's write sentence word for word: "Write files with the Write and Edit tools only (no sed, no
  shell redirect, no python writes; never `cargo fmt` writing files: apply rustfmt's diff by Edit); an agent with no
  Edit tool rewrites its notes file whole with Write." It never cleans: at the runner's floor it stops and reports.
  Never `cd` into the worktree from the main session (the card's lesson).
- **Order:**
  1. Part A, now: FP-1, FP-3, FP-5, FP-6, FP-7, FP-4, FP-8, each compiling with its tests passing and its plant failing
     by name before the next; SIZE-CHECK (`git diff --stat a289def -- spikes/` plus forged.rs) after each.
  2. Part B, on his F-B-COUNT answer: FP-2 if (a). If he chooses (b) or (c), the main session brings it to me first
     (its seats, size and runs change); nothing of FP-2 is built before his word.
  3. fmt, both clippy targets and doc (cheap; their logs), then R2-R5 on the tree with FP-2 in.
  4. Round 2 (block 5) on the fix pass's diff, with R1-R5's logs as its evidence.
  5. Round 2's fixes, if any, each with its test and plant; any R2-R5 line whose code they touch re-runs (I name them).
  6. R6, check.mjs twice on the final tree. Then the merge into the lane as planned (the main session's).
  Running R6 after round 2 saves up to two check.mjs runs and their cleans if round 2 asks for changes.
- **Size:** FP-1 about 40-70 added lines, FP-2 60-100, FP-3 20-35, FP-4 30-50, FP-5 20-30, FP-6 8-15, FP-7 10-30, FP-8
  50-80: about **240-410**, so U4f2 ends at about **2,320-2,490** added (2,077 now): above the 1,700 to 2,250 of the
  option he chose (history.md:7266, "(a) Build it whole: about 1,700 to 2,250 added lines ... The stop at 3,000 lines
  stays"), inside the card's size line read from it (history.md:7271, the main session's reading: up to about 2,700 at
  U4f's 1.2), and under the stop past 3,000 at a block end, which stands and is his. Not a stop, so it is told to him
  (block 6), not asked. At the bots part's 3x overrun on FP-1, FP-4 and FP-5 alone it would reach about 2,700. **My
  stop beside his:** past 2,700 added at an FP end, the builder stops and reports to me, before more is built; past
  3,000 his stop applies as written.
- **The runs owed before the merge** (each in the record with its log path; figures as found, none fitted):
  - **R1, per FP (debug):** the FP's tests alone (each new test's wall time, the brief's rule) and its plant.
  - **R2, the 610 s forged pairs** (release measured build; seed 1; options A and B; 10 s warm-up + 600 s; each
    measured run then its twin; judged by `npt-judge --only PM-0001-2,PM-0001-21` in 5.4's measured-and-twins form):
    every kind sent at least 1,000; every kind's `unrejected_count` 0; kind 7's `dead_left_out_count` reported; twin
    mismatch 0 on both options; PM-0001-2 passing with the shots stated; slot 6's `commands_rerun_count` 0 under B;
    `counted_count` equal to `sent_count` for kinds 8, 9, 11. 4 runs, about 40 min, about 0.26 GB of logs (hand-long2:
    65,188,828 B a measured run).
  - **R3, bite-no-lead-bound at 610 s** (release bites build, seed 1, A): kind 1 the only kind unrejected; its shots
    stated (under 670 comes to me before the set). 1 run, about 10 min, about 65 MB.
  - **R4, FP-4's power:** p19's form (400 ms hold, P1, option A, its short length) under PM-0001-21/no-catchup-cap,
    release bites, unjudged: `tick.v.lagswitch_excess_move_um` max above 0 (if not, to me). 1 short run.
  - **R5, section 6 item 5 whole on the final tree** (TE-U4F2-7): every set-21 line and r2's p1, p7, p14, p17 by hand
    at their short settings, both judge calls in 5.4's form, all exit 0, the twin checks PASS on both options; and the
    fast-clock line's room-log per-tick hashes equal to its run in .gate/u4f/u4f2-hand (hand-a's tree), FP-4's proof
    that the fast clock's commands did not move (another cause of a difference named, else to me). About 35 short runs.
  - **R6, check.mjs twice** on the final tree (A3's form, `-x`, all 11 steps, `df` first): measured 702 + n and bites
    799 + n (n the fix pass's new tests), 0 failed, 0 skipped; each test step under 1,207 s in the worktree (block 2,
    TE-U4F2-8); wasm 27,000 ticks, 0 mismatching.
- **The disk:** 9.5G free now (10.1 GB), the runner's floor 4 GiB, about 6.3 GB per check.mjs run from a cleaned
  target and about the same for a debug test build. So the debug runs of part A meet the floor once; part B's debug
  runs with the release builds and R2-R5 (about 0.4 GB of logs, kept) meet it again; each R6 run needs a cleaned
  target. **Expect about four part-cleans** (the build took four), each by the main session between steps with no lane
  running (P27's commands under the lock, its guards), never by the builder, never of a kept-logs folder. R2-R5's logs
  stay under .gate/u4f/ (about 0.4 GB). After the merge, as planned and the main session's: gate `--no-smoke`, check.mjs
  in the main folder (a clean first), CI, my head check, c-u4f-r2.mjs whole (21 rows, about 80 MiB), D2, then -21's
  set (49 runs, about 3.2 GB at the measured 65 MB a run: FP-8's projection), judged in two calls.

## Block 5: the round decision (DONE)
- **Round 2 runs, on what changed.** Round 1 leaves a High open (F-B), and FP-2 changes what a judged check counts:
  both are the 2026-11-17 choice's triggers for a second round (history.md:6818-6819), inside CLAUDE.md's budget (at
  most two rounds; a third only for a Critical or High still open after round 2).
- **Its scope:** the fix pass's diff only (the final fix tree against the e9a0ae3e… snapshot's), with R1-R5's logs as
  evidence, before R6 (block 4's order).
- **Its seats:** netcode-architect (FP-1's construction, FP-2's window against his answer's words, FP-3, FP-4's lane
  and the fast clock unchanged, FP-5), test-engineer (TE-U4F2-1's proof, every new plant, FP-6, FP-7, FP-8's tests,
  R2-R5's figures) and rust-reviewer (FP-3's const fn and its pins, FP-8's arithmetic and bounds, the bots refactor,
  clippy and docs): one batch of three, each at xhigh, read only, the panel brief's rules. security-auditor is not
  re-called: its three Lows are fixed in FP-3 and FP-8, and the pass touches no server, network or storage code
  (npt-room gets doc lines only); I read SA-U4F2-1 to -3's fixes against their acceptance in my head check, and any
  round-2 finding with a security side brings the seat back.
- **After it:** a Low becomes a follow-up (never a push alone); a Medium is fixed or deferred in writing; a High or
  Critical still open after round 2 is the only ground for a third round, and one open after a third stops U4f2 for
  him.
- **Verdict on round 1: REQUEST CHANGES** (FP-1 to FP-9, FP-2 on his F-B-COUNT answer).

## Block 6: what goes to Mustafa (DONE)
- **In the 2026-11-21 Amsterdam message** (after 23:00 UTC on 2026-11-20; at most one a day), whose first line reads
  "stopped on you", beside U4G-Q1 (not a stop) and the choices already named for it:
  1. **F-B-COUNT, the one decision** (paste-ready, below). Only FP-2 and what follows it wait; part A goes on.
  2. **Under "Choices taken without him"** (TEST-FIRST rule 2: inside the plan, undoable, no pass mark, judge,
     marks.json or existing SERIES.md row, no money, nothing on main), paste-ready:
     > U4f2's first review round (four reviewers; one serious finding, below as F-B-COUNT): the fixes taken by the
     > advisor's ruling. Kind 7's cheating bot rebuilt so a reload press next to its return from death is still refused
     > (this also removes a rare false difference from the twin run); its pause before each press taken from the weapon
     > and the tick rate, so it holds at 60 ticks a second too, and intervals the weapon cannot meet refused at start;
     > the lag-switch bot walks a clear lane again, so the lag check's movement can fail; a missing twin test for five
     > kinds, and a dozen smaller fixes and record lines. The builder's five departures accepted: one shared test pair
     > per option, the cheaters on a fixed lead, the honest bots standing still (the advisor's earlier route for keeping
     > them out of buildings), kind 7's rebuild, and a
     > constant renamed (HOLD_STEPS) because the checker read its old name as a tick count, which two reviewers checked
     > it is not. The test step's time limit read on the builder's own two counted runs. A second review round on what
     > the fixes change. U4f2's size after them: about 2,320 to 2,490 added lines in all, above the 1,700 to 2,250 your
     > (a) was given (the review's fixes add about 240 to 410), and under the stop at 3,000, which stays and is not
     > reached. Why: all inside the plan and undoable; no pass mark, judge, marks.json or existing SERIES.md row
     > changes; no money; nothing on main.
- **The question, paste-ready, in plain words:**

> **F-B-COUNT** (stopped on you: U4f2's merge, and so -21's runs, wait for it). How should the forged-command check
> count a cheating press that reaches a dead soldier?
> - What happened: in -21's forged-command test, one cheating bot (kind 7) presses fire or reload at moments the
>   weapon must refuse. The honest bots now stand still and fire at fixed points, so a few times a run they kill it. A
>   press it sends while dead, or one whose set-up press (the command just before it) ran while it was dead, changes
>   nothing, but leaves no refusal record. On the one 10-minute run, 18 of its 1,523 presses were counted "not
>   rejected", so "every forged command rejected and counted" (it must read 0) would fail on every judged run, for a
>   reason in the test, not in the server.
> - Why it is yours: every way out changes what that check counts (a reading of ADR 0001's PM-0001-21 row, like your
>   yes to A1's movement reading in the same row), or how kind 7 counts as rejected.
> - (a) Leave such presses out of kind 7's count: a press is left out when the bot's soldier was dead, by the server's
>   own state, when that command or the one before it ran, and the number left out is reported beside the check in a
>   new report-only field. The check, its 0 and the other ten kinds stay as they are; the 1,000-per-kind check still
>   applies to what is left (about 1,500 a run); the twin check, which compares every tick's state with a run without
>   the forged presses, still covers every press, the left-out ones included.
> - (b) Have the simulation record a refusal for every press a dead soldier's command carries, and count that as kind
>   7's rejection. It keeps the check's words exact, but opens the simulation code this unit keeps closed, adds a
>   refusal kind that also marks honest players pressing while dead, and costs about 60 to 150 lines and a review.
> - (c) Neither: kind 7 then fails -21 on every forged run.
> - My lean, and the advisor's: (a). The reading joins A1's on the readings list, and -21's report names it in a line
>   of its own with who reviewed it.

- **Nothing else is his:** no pass mark, judge, marks.json text, existing SERIES.md row (FP-8's `counted_count` and
  FP-2's `dead_left_out_count` are new rows in U4f2's own subsection), money or main. No security finding of this
  round is left open (SA-U4F2-1 to -3 are fixed in the pass).

## The ruling per finding (each seat's severity kept)
| Id | Sev | Ruling | Why (evidence) |
|---|---|---|---|
| NC-U4F2-1 = TE-U4F2-1 | High | **cause confirmed; FP-1 (the reload half, by construction: mine); FP-2 (the dead half, route (a)) only on his F-B-COUNT answer**; NC's (b) not taken | world.rs:1351-1355, :1192-1203; forged.rs:86-91, :117; bots lib.rs:892 (fire held on a reload item's d-1); lab.jsonl:58 (18 of 1,523); (a) narrows ADR 0001:1611's "every forged command" and SERIES.md:158's `sent_count`: his by TEST-FIRST, LEAD-BOUND's row rule and A1's precedent (Block 1) |
| NC-U4F2-2 = RR-U4F2-1 | Medium | **fix: FP-3** | world.rs:1415-1416, :1440; bots lib.rs:147-153: sound only for an interval of 2 to the rest in ticks; at 60 Hz or `--forge-every 2` fire items go unrecorded; carry 39, plan A5/RQ6 |
| NC-U4F2-3 = TE-U4F2-5 | Medium | **fix: FP-4** (route (i)); Q1 point 1 not in the way | scenario.rs:1265-1267, logs.rs:699-706, scenario.rs:4225: a standing lag client makes the check unable to fail; at 200 ms a walking one still catches a doubled run |
| NC-U4F2-4, TE-U4F2-2 | Medium | **fix: FP-5** | bots lib.rs:279-282 (no twin guard), :909-910; plant9.log:20-34; room.rs:1085-1087 |
| NC-U4F2-5 | Low | **usage half: FP-8** (with RR-U4F2-3); header param not taken: **record line (FP-9)** | scenario.rs:1230, :1988-1995; the suite runs at P0 only |
| NC-U4F2-6 | Low | **fix: FP-8** (doc) | bots lib.rs:562-566 against :909-923 |
| SA-U4F2-1 = TE-U4F2-15 | Low | **fix: FP-8**, with RR's `checked_mul` in `forger` | bots lib.rs:944; scenario.rs:2240-2247; Cargo.toml:116-120 |
| SA-U4F2-2 | Low | **fixed by FP-3** | at 2 the honest press drifts into the cooldown (RR block 1's arithmetic) |
| SA-U4F2-3 | Low | **fix: FP-8** (a false-pass path, unreached today) | forged.rs:116 |
| TE-U4F2-3 | Medium | **fix: FP-6** | forged.rs:81, :86, :345-349 |
| TE-U4F2-4 | Medium | **departure accepted; fix: FP-7** | scenario.rs:4105, :4364-4420; plant10.log:21-25 |
| TE-U4F2-6 | Medium | **no code; R2 and R3 state the shots; under 670 to me before the set** | hand-a 680-688, hand-long 677/685, hand-b 669, p7 668; the band is his |
| TE-U4F2-7 | Medium | **fix: R5** (section 6 item 5 whole on the final tree) | hand-a.log:3 d211149a… against end-b.log:3 4a9dde09… |
| TE-U4F2-8 | Medium | **ruled: the line reads the builder's two counted worktree runs; the pass adds at most 30 s; past either, to me** | u4f-brief.md:678-680; 931.9 s against 1,007.5 s on equal tests |
| TE-U4F2-9 | Low | **fix: FP-8** | forged.rs:136-148 |
| TE-U4F2-10 | Low | **fix: FP-8** | c-u4f-r2.mjs:61-75; u4f2-brief.md:110 |
| TE-U4F2-11 | Low | **fix: FP-8** | c-u4f-runs.mjs:80; hand-long2's 65,188,828 B |
| TE-U4F2-12 | Low | **fix: FP-8** | scenario.rs:4408-4416; u4f-brief.md:487 |
| TE-U4F2-13 | Low | **no code: record line and a follow-up by id** (first unit that skips on a controlled endpoint) | no -21 client skips under the controller |
| TE-U4F2-14 | Low | **doc: FP-8; records: FP-9** | the plants' file:line; the short copy's pass of (b)'s walk; the hash slip already fixed |
| RR-U4F2-2 | Low | **fix: FP-8** | forged.rs:28-34, :66-69 |
| RR-U4F2-3 | Low | **fix: FP-8** | scenario.rs:2240-2266 |
| RR-U4F2-4 | Low | **fix: FP-8** | room.rs:42-46 |

## The fix pass (the builder, in its worktree; FP-9 the builder's record and the main session's report)
| FP | Finding | The change | Acceptance check and plant |
|---|---|---|---|
| FP-1 | NC-U4F2-1 (reload half), TE-U4F2-1 (twin half) | bots `forge_step`: a kind-7 reload item's honest command d-1 releases fire (`buttons &= !FIRE`), the twin's d-1 the same; f15's kind-7 case to the new shape; a World-level death test (T-K7): a kind-7 forger's commands and its twin's, each into a World, the forger's soldier killed and revived (an opposing soldier's fire, or a set-up the test picks) so items fall with d or d-1 run dead, one reload item has d-1 as the first live command, others run alive | T-K7 passes, at most about 5 s alone in debug: every item whose d-1 and d ran alive carries its Cooldown or Reload record; the reload item after the revive carries a Reload record; every tick's state hash equal between the two Worlds; the dead-window items printed. Plant: fire held on a reload item's d-1 (today's) fails T-K7 by name (an unrecorded reload and a hash difference) |
| FP-2 | NC-U4F2-1 (a), TE-U4F2-1 | **only on his (a):** forged.rs: per forger, the alive flag of its soldier (the server's World, before the tick's command phase) at each run of its last few sequences; an item left out of `sent_count` and `unrejected_count` when the soldier was dead at the run of d or of d-1 (an item whose d-1 never ran is not left out); `dead_left_out_count` on every `forged_kind` record (0 for kinds other than 7); the module text; scenario.rs passes the flags; one new SERIES.md row in U4f2's subsection | an f33 case on hand-made inputs: dead at d's run out; dead at d-1's, alive at d's, out; alive at both with no record unrejected; alive with a record rejected; d-1 never ran, not out: passes. T-K7 extended: its attribution reads unrejected 0 with the dead-window items counted apart. Plant: the window ignored (today's rule) fails both by name. R2: every kind's `unrejected_count` 0 under A and B. -21's report: the reading in a line of its own, with who reviewed it; the readings PR's list |
| FP-3 | NC-U4F2-2 = RR-U4F2-1, SA-U4F2-2 | `fire_rest_at(tick_hz, interval_us) -> Option<u64>`: (interval_us x tick_hz).div_ceil(10^6) when interval_us x tick_hz is at least 2,000,000, else `None`; the rest held on `Behaviour` from the scene's weapon and `npt_sim::TICK_HZ`; `forger(7, every, _)` `None` when the rest is `None` or `every` is below it (the lab's usage error names kind 7's rest); the doc reads "at least two ticks" | a test pins SS at 3 (30 Hz) and 5 (60 Hz) and `None` for an interval under 2 ticks; a case (beside c_u4f2_1, or on `forger` itself) shows a forged suite at `--forge-every 2` refused as a usage error and 3 taken (30 Hz), the parser's floor of 2 unchanged; plant: the rest fixed at 3 fails the 60 Hz pin by name; R2's kind 7 at 12 as before |
| FP-4 | NC-U4F2-3 = TE-U4F2-5 | the lag-switch suite's slot 0 on the Lane walk (closed loop, no twin), its lag switch unchanged; the Lane's `west` moved out of `Fast` so a non-fast client can hold it, the fast clock's logic unchanged | f20 asserts `lagswitch_excess_move_um` above its standing value (minus 8 ticks of full speed) on some burst tick and at most 0 on every tick, and the lag client's in-a-building count 0; plant: the lag client back on the standing script fails f20 by name; the fast clock's tests pass unchanged; R5's fast-clock tick hashes equal hand-a's; R4's excess above 0 |
| FP-5 | NC-U4F2-4, TE-U4F2-2 | `if !twin` on `forge_input`'s 1, 2, 4, 5, 6 arm; a bots case on `sides(k, false)` for those kinds (the kind-10 case's pattern, bots lib.rs:1891-1901) | the case passes: the twin sends no forged sequence, its sequences are the forger's less the forged ones, every other command equal; plant 9 in its right form (the twin sends the refused command's honest content, `due` false) fails the case by name, and under it f16 option A kinds 1, 2, 4, 5, 6 fail and option B passes (both logged: F-E recorded as option B's limit) |
| FP-6 | TE-U4F2-3 | in q4 or a new f33 case: an own-kind record on another slot naming a forged sequence (room kind 10 on SLOT + 1 for seq 2; a sim Cooldown on SLOT + 1 for a kind-7 item) leaves the item unrejected | passes; plants: each slot filter removed (forged.rs:81, :86), each fails it by name |
| FP-7 | TE-U4F2-4 | f16's failure names the culprit: on a fate-entry difference its link's slot and kind; on a hash difference the forger slots whose Soldier records differ at the first differing tick | plant 10 re-run (kind 4's room check removed): the failure names kind 4 (slot 3) |
| FP-8 | SA-U4F2-1 = TE-U4F2-15, SA-U4F2-3, RR-U4F2-2, RR-U4F2-3 with NC-U4F2-5's usage half, RR-U4F2-4, NC-U4F2-6, TE-U4F2-9, -10, -11, -12 | the parser's top bound (at most 1,000,000) and `forger` `None` when `every.checked_mul(8)` fails; `counted_count` on kinds 8, 9, 11's records (a new SERIES.md row) with f16's equality at P0; a reused sequence counted unrejected and the one-lap limit in the module text; `--suite forged` off P0 or with `--lead` a usage error; `RefusalRecord`'s field docs; `twin()`'s doc on both skip timings; f33 asserts a nonzero dropped count is carried; "in a building: 0" in every -21 row's `expect`; set 21's projection from the measured 65.2 MB; f16 asserts every forger's refusal counters but its own kind's, and every honest client's, 0 | each new case passes: "18446744073709551615" and "1000001" refused, "1000000" taken; f16's `counted_count` equality (plant: the counter read as sent + 1 fails it by name); a reused-sequence f33 case; P1 and `--lead` refused (two cases); the dropped count (plant: written as 0 fails it by name); f16's zeros (a nonzero count is a finding for me, never a looser test); the docs, rows and projection by reading |
| FP-9 | TE-U4F2-13, -14, NC-U4F2-5, the departures | record lines in the build record and U4f2's report: each plant's file:line (old and new); the short C-U4F2-5 copy's pass of (b)'s walk; the forgers' fixed lead of 2 and P0's in-order assumption; `skip_command`'s controller half untested, with its follow-up; the five departures as ruled (block 3), `HOLD_STEPS` in a line of its own; C-U4F2-9's lines; F-C's figures | the main session reads each line before the merge |
End checks (R1-R6, block 4): every FP's tests and plants; R2's pairs, R3's bite, R4, R5 before round 2; R6 (check.mjs twice, all 11 steps, the counts and each test step under 1,207 s in the worktree) after it.

## The round decision
**Round 2, on what changed**, after part B and R1-R5: netcode-architect, test-engineer and rust-reviewer in one batch of
three, xhigh, read only (round 1 left a High; FP-2 changes what a judged check counts). security-auditor only if a
round-2 finding has a security side. A third round only for a Critical or High still open after round 2. **Verdict on
round 1: REQUEST CHANGES, FP-1 to FP-9**, with FP-2 waiting for his F-B-COUNT answer and nothing else waiting.

## To Mustafa, in one line
**F-B-COUNT** in the 2026-11-21 message, first line "stopped on you" (paste-ready in block 6; lean (a)), plus one entry
under "Choices taken without him" (block 6); only FP-2 and what follows it wait; nothing else is his.
