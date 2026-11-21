# Advisor plan review: U4f-F1 (u4f-f1-plan.md rev 1), 2026-11-19, from main's advisor.md

Read only. Sources: `.checkpoints/netcode-proof-test/u4f-f1-plan.md` (rev 1, 55 lines), my -20 ruling
(`advisor-u4f-20.md`: R20-Q1 :114-140, ADV-U4F20-1 :162), sim/src/world.rs, room/src/room.rs, room/src/input.rs,
ADR 0001 part 5, the tests and pins that read refusal counts, SERIES.md, history.md:2234-2282 (TEST-FIRST).

## Plan
1. R20-Q1 against the plan's fix shape: world.rs `run_command` and every `edge` use; room.rs `run_one`; input.rs:285-300;
   ADR 0001:225-237.
2. 1-edge path unchanged; what T2 must show; which tests or pins read the sim's refusal counts.
3. Lane, size, reviewers, builder setup, the re-run and its expected verdicts.
4. Rule U4FF1-Q1 to Q3.
5. Anything for Mustafa (pass mark, judge, marks.json, money, main, the plan's stops).
6. Verdict, findings, rulings, conditions, the line to Mustafa.

## Block 1: the fix's shape against R20-Q1 (the code, read; nothing run)
- **The plan's cites hold:** world.rs:1340-1346 (the one-edge loop and `claimed`), room.rs:1330-1349 (`run_one`),
  input.rs:285-300 (kind 5), ADR 0001:226, :234-237 (checked: the ADR is main's copy), SERIES.md:42, room/Cargo.toml:21
  (`npt-sim.workspace = true`). The code tree is one since U4f's merge (`git diff --quiet` 25d316e..2dae645..HEAD and
  350677f..HEAD on spikes/netcode-proof, results excluded: exit 0), so 2dae645's counts are 25d316e's 634 and 731.
- **Its shape is R20-Q1's, not loosened or widened:** every edge of an advance, each through today's checks, a refused
  one counted and traceless; the room's lost count from one rule shared with the sim; the comment corrected; T1 is my
  option-B room test (plus doubled 0); T2 is my sim test; -20's L1 set re-run whole on the fix; 350677f's folder kept;
  -22 not re-run (advisor-u4f-20.md:123-131). No mark, judge, marks.json, README or SERIES.md row (plan :4-5, :14, :22-23).
- **Gaps in the shape** (each a finding below):
  - **Four press counters, not three checks.** `edge` is used at world.rs:1419 (reload), :1443, :1452, :1457, :1466
    (fire, bites included) and **:1484 (throw)**; the plan's range :1408-1466 and its "(cooldown, ammunition, reload)"
    leave the throw out, and melee (seam.rs:69-71) the sim reads from no one. The throw is the one press whose extra
    edges change state: each edge throws while the pool has room (world.rs:1502-1507), so a 2-edge throw is two
    grenades, not a refusal. Not state-neutral, and correct by ADR 0001:236-237 (each edge meets the server's own state;
    the stand-in's throw has no cooldown). ADV-U4FF1-1.
  - **The fire edge after a shot.** Today a command fires at most once (`fired`, one bool: world.rs:1466; returned to
    `predict`, :1052). The weapon's clock advances once (:1407-1417). With an interval below one tick a re-read of
    `ready` after the shot is true again: `Attribute::FireInterval` clamps to 1 µs (world.rs:1171-1172), the shot adds
    interval x TICK_HZ units against 1,000,000 a tick (seam.rs:167; world.rs:1409-1411). So a per-edge loop that
    re-reads `ready` would fire twice in one command, which no code path does today. Every scene's weapon interval is
    above one tick (scene.rs:64, :73, :237: 68,571, 666,667 and 100,000 µs against 33,333 µs at 30 Hz), so no run meets
    it; tests that set the interval do (sim/tests/world.rs:466-470, :1987-1995). The plan names neither the clock nor
    this. ADV-U4FF1-2.
  - **The order.** Today's code order is: the clock, the reload edge, the fire edge, the throw (authority only). The
    plan's "in order (cooldown, ammunition, reload)" names check kinds, not that order. Part of ADV-U4FF1-2.
  - **The one rule's contract.** If the `pub fn` returns only the advance, the room's lost is 0 by its own code. Then the
    plant "the old one-edge rule" has to be made in two places, and the room cannot see a sim that skips the fn.
    ADV-U4FF1-4.
  - **A session's first command.** Kind 5 runs only against an accepted command (input.rs:287 `if let Some(prev) =
    &self.accepted`); a new session starts with none (InputBuffer::new, room.rs:829). The sim's `claimed`
    (world.rs:678-679) and the room's `run_presses` (room.rs:507-509, "which outlive a session") carry over. So a
    session's first command carries an advance up to 255 per button that no check bounds. The old rule took 1 edge;
    the new takes it all: state-neutral for fire and reload, but the throw fills the pool in one tick. No run in -20 or
    -22 meets it (-20: 48 joins, no kick, overflow resyncs 0 on all 7: runs.txt:23-29; -22's bots press nothing:
    advisor-u4f-20.md:110-111). ADV-U4FF1-3.
- **The loop's bound inside a session:** an accepted advance against the last accepted command is at most the ticks
  it covers and below 128 (`after8`: proto/src/wrap.rs:43-47). Against the last *run* command (the sim's reference),
  it is the sum of accepted advances since then, at most the ticks since then. 255 only on a first command (above) or
  with PM-0001-21/no-press-edge bitten (input.rs:293). That is no allocation risk beyond the refusal list.

## Block 2: the 1-edge path, T2, and who reads refusal counts
- **A 1-edge advance runs as now** if the loop runs each button's edges at today's place, and its first pass is today's
  code: the same `ready`, the same bite overrides (:1440-1455), the same kind test (:1457-1464), the same `fired`. Then
  the twins' pins hold: every twin test feeds 1-edge streams (d5 :485, d10 :1651-1763, through `course_stream` :1605,
  which advances each listed counter by one at its step). So do the lab's run twin (r1, lab/src/scenario.rs:2585: two
  processes, equal fate logs and per-tick hashes) and npt-l1's fate-log tests (the link's packets, independent of the
  sim's press rule: l1/src/tests.rs). The bite list and its pins do not move (world.rs:949; room/tests/bites.rs:9).
- **Where multi-edge advances occur today** (the plan's "every honest command today but the six" is narrower than the
  code; ADV-U4FF1-8):
  - -20 B's six ticks;
  - bite-redundancy-1's 3,614 lost edges (B; advisor-u4f-20.md:24-25);
  - d4's first command (sim/tests/world.rs:426: counters 255 from claimed 0, so 255 edges per button);
  - the synthetic recordings' lost commands when the next command presses the same button (sim/src/replay.rs:347-348,
    :511-523; w1_s1 :1972 replays one, with fire intervals of 1 µs set on slot 0);
  - PM-0001-21/no-press-edge's room test (room/tests/commands.rs:1017-1023);
  - a session's first command (Block 1).
  None of them asserts a refusal count or a state pin that the fix moves. d4 asserts grid ranges only (:475-481); w1_s1
  asserts that the record equals the state; the bite test asserts that the command ran (`shows`, :923-937).
- **Readers of the sim's refusal counts** (+1 per extra edge): d5 (:531-545), the four d10 twins, room's no-cooldown
  bite test (commands.rs:871-895: Cooldown refusals over 1-edge commands, presses = seq, :878) and refuse-not-clamp
  (:900-). All use 1-edge streams: none moves. The lab reads only `reports().clamps` (scenario.rs:1374-1390), and the
  room exposes refusals only through `reports()` (room.rs:762, :1045). So no sim refusal reaches the wire, a log
  series or a -20 check. -20's checks read on-time, missing and the three press series (advisor-u4f-20.md:26-29). By
  reading, not run: the end checks confirm it.
- **What T2 must show for the state-neutral claim** (my Block 3 said "expected, not proven": advisor-u4f-20.md:75-78;
  the plan's T2 compares one state hash, ADV-U4FF1-9).
  The template is there: the d10 twins (`course_stream` + `twins`, :558, which asserts equal state hashes **and** equal
  event lists on every tick, and returns each side's refusals). A 2-edge advance is `course_stream`'s press list with
  one step named twice, e.g. `&[(0, 10), (0, 10)]`. Its twin is the same list with that step once: the W1-U4D twin
  form, "that button's counter one lower from the refused command on" (world.rs:35-38). The cases, each over a stream
  that runs on past the 2-edge step:
  - (a) fire held (the brawl bots' form: bots/src/lib.rs:658-662), the weapon ready: one shot, the second edge refused
    as Cooldown;
  - (b) fire not held, ready: the same;
  - (c) in the cooldown: both refused (forged 2, twin 1);
  - (d) on the last round: the shot, then Ammunition;
  - (e) reload: one starts, the second refused as Reload;
  - each with its refusal list exact (slot, seq, kind) and its counts +1;
  - (f) throw, apart from the twin form: two projectiles with the command's seq where the pool has room, one and a
    PoolFull where it has one slot.
  A plant that removes the second edge's refusal (the old rule) fails the count half. A second plant, the weapon's
  clock advanced per edge or a second shot allowed, fails the twin half by name, so the state half is shown able to
  fail. The run-scale half is the re-run (Block 3).

## Block 3: lane, size, reviewers, builder, the re-run
- **Lane:** crates/sim and crates/room and their tests, nothing else (plan :14). U4f2's lane keeps crates/sim shut and
  opens npt-room only for C-U4F2-3 and C-U4F2-8 (u4f2-brief.md:130-132), so the two lanes do not overlap while U4f2
  waits. No marks.json, judge, marks/README.md, SERIES.md row, ops/runs script or lab code changes. SERIES.md:42 ("press
  edges lost this tick (against the press counters)") keeps its meaning: the count stays a true count of the edges the
  sim does not take (room.rs:352-356's doc changes with it: ADV-U4FF1-7).
- **Size:** S, about 60-150, the stop past 300 (twice the figure: SIZE-CHECK). My conditions add T2 cases, but
  each is a d10-style twin of about 10 lines on the existing helpers, so the build stays inside 300. Past 300 it
  stops as the plan says; the figure does not grow.
- **Reviewers:** rust-reviewer, netcode-architect, test-engineer fit the change. The change is also the server's
  handling of client input: the authority now takes more of a client's claimed edges, and a session's first command is
  unbounded (ADV-U4FF1-3). CLAUDE.md step 5 names security-auditor for "server", and U4f's input work had it
  (u4f-report.md:64). Add it, scoped to that reach. Four reviewers in batches of at most three is still one round
  (CLAUDE.md, Review budget). ADV-U4FF1-6.
- **Builder:** one, Opus 5.5 at xhigh (netcode: CLAUDE.md, Models and effort), in an isolated worktree on the lane
  head, notes in its copy of u4f-f1-build.md, snapshots by worktree_patch.py, the shared target under the lane lock, the
  standing write sentence word for word; the main session merges into the lane branch (gate --no-smoke, check.mjs, CI).
  Nothing goes to main. One agent, so §4's three hold. Fine as written.
- **The re-run and its expected verdicts.** The plan says "option A as before, option B '0 presses lost' 0 on all three,
  the bite shown". That is right but loose, and the runs can prove the state-neutral claim at full scale for free:
  - L1 is deterministic by seed (r1: lab/src/scenario.rs:2582-2604, equal fate logs and per-tick hashes across two
    processes).
  - Sim refusals never reach the wire or a log (Block 2).
  - -20's bots hold fire and press only fire (bots/src/lib.rs:639-646, :658-662).
  - So with the fix state-neutral, each run reproduces 350677f's:
    - every summary line equal to runs.txt:23-29 but its wall time (fate log digest, client hashes, shots, projectiles
      and audit included);
    - every measured room log's per-tick `h` equal, tick by tick, to the kept raw logs
      (.gate/netcode-proof-test/runs/u4f-20-350677f0aef2/measured/; bites/ for the bite run);
    - `tick.v.press_lost_count` 0 everywhere, the six ticks included (advisor-u4f-20.md:42-44).
  - Verdicts:
    - PM-0001-2's "SS runs" PASS, 6 of 6 valid;
    - PM-0001-20 option A PASS 3/3, every figure as at 350677f;
    - option B PASS 3/3: on-time, missing and presses as at 350677f (on-time and missing equal to A's per seed;
      presses 432,000 each), lost 0, doubled 0;
    - L2 parts NOT-RUN;
    - the bite PM-0001-20/redundancy-1 shown by the 99.9 % (on time 0.894544, as marks.json:4794-4795 defines it), its
      lost 3,614 going to 0 by design (losing commands no longer loses presses);
    - the reorder report row still "not available" (ADV-U4F20-2, his; the report states the event's figures by hand).
  - Any other difference stops the record: it goes back to the advisor before the verdict counts. ADV-U4FF1-5.

## Block 4: what touches Mustafa's (TEST-FIRST, history.md:2234-2282)
- **Pass marks:** none changes. "0 presses lost" keeps its bar and its series. After the fix its count is still what
  the sim does not take, and the plant shows it can fail. The fix makes ADR 0001:226 ("a resent command can neither
  lose nor double a press") true of the stand-in; it does not relax the check. The bite's "fails the 99.9 %"
  (marks.json:4795) still holds; its lost figure falling to 0 is the fix, not a change to the bite.
- **Judge, marks.json, README, SERIES rows:** untouched (Block 3). **Money:** none (local L1, about 41 min). **Main:**
  nothing. **The plan's stops:** the size stop (300) stands. -20's re-run is the planned set my R20-Q2 placed inside the
  plan (advisor-u4f-20.md:141-145; 350677f's set ran without him).
- **Rule 2's four hold:** inside plan rev 3.3, undoable, no pass mark, no money and nothing on main. So it is no
  ESCALATE: recorded with its reason and named in the next day's message.
- **Later, not now:** if U4f2's ruling on a session's first command (ADV-U4FF1-3) needs a new forged kind or check in
  marks.json, that is his then.

## Rulings
- **U4FF1-Q1: yes, in npt-sim's seam, and npt-room calls it.** One `pub fn` in sim/src/seam.rs beside `Command`'s
  press indices (seam.rs:46-72: "the one home the sim, the client and the bots read"). The seam is the room-sim type
  module (seam.rs:1-7), and npt-room depends on npt-sim for it (room/Cargo.toml:18-21). It is integer-only (npt-room is
  float-free), its doc cites ADR 0001:226, :234-237, and its contract is condition 2's. A second copy anywhere is the
  drift the rule exists to stop.
- **U4FF1-Q2: confirmed, no change.** `claimed` takes the command's counters (world.rs:1346) before the dead-or-plate
  return (:1348-1350): a dead or plate soldier's edges are taken by the rule, have no effect and count no refusal, as
  today. The room counts them seen with lost 0; today a 2-edge advance there counted lost 1. A slot beyond the soldiers
  returns before `claimed` (:1336-1338), also as today.
- **U4FF1-Q3: U4f-F1 first, as its own step, before U4f2.**
  - U4f2 waits on his U4F2-SIZE answer; F1 does not (card :177-183).
  - U4f2's lane keeps crates/sim shut (u4f2-brief.md:132), and its size is already near his stop (about 1,700-2,250
    against 3,000: advisor-u4f2-brief.md:149-156). Folding F1 in would widen both.
  - -21's judged runs then run on the fixed sim (R20-Q1, advisor-u4f-20.md:132-133).
  - Before U4f2 builds, its brief rebases on F1's merge: its base and its count baseline become F1's merge and F1's end
    counts, not 25d316e's 634 and 731 (u4f2-brief.md:15, :128). It also takes ADV-U4FF1-3's carry.

## Findings
| Id | Severity | Where | What goes wrong | Fix |
|---|---|---|---|---|
| ADV-U4FF1-1 | Medium | u4f-f1-plan.md:15-18, :31; world.rs:1484, :1502-1507; seam.rs:69-71 | The plan names three checks (cooldown, ammunition, reload) and "reload or ability"; the throw edge (:1484) and melee are not named. The throw is the one press whose extra edges change state (two grenades) | Condition 1: all four counters named with their rule; throw not state-neutral by design, shown by T2 (f) |
| ADV-U4FF1-2 | Medium | u4f-f1-plan.md:15-18; world.rs:1407-1417, :1466, :1171-1172; seam.rs:167 | The rule for a fire edge after the command's shot, the weapon clock (once, not per edge) and the button order are unnamed. A re-read of `ready` fires twice in one command when the interval is below one tick (FireInterval clamps to 1 µs), which no code path does today | Condition 1: one clock tick per command; reload, fire, throw at today's places; at most one shot per command as today, further fire edges refused (Ammunition if empty, else Cooldown) |
| ADV-U4FF1-3 | Medium | room/src/input.rs:287; room.rs:507-509, :829; world.rs:678-679 | A session's first command is not kind-5 checked, and the counters it is measured against outlive the session. Its advance (up to 255 per button) is taken whole under the new rule: state-neutral for fire and reload, a pool of grenades in one tick for throw (the old rule took 1). No -20 or -22 run meets it | Condition 7: named in the plan and the build notes; kind 5 unchanged in F1; carried into U4f2's brief for -21 (netcode-architect, security-auditor there); a marks.json need is his |
| ADV-U4FF1-4 | Medium | u4f-f1-plan.md:19-23, :27-28 | The shared fn's contract is open: if it returns only the advance, the room's lost is 0 by its own code, the old-rule plant needs two edits, and the room cannot see a sim that skips the fn | Conditions 2 and 3: the fn gives the edges taken; the sim loops that count; the room's lost = advance - taken; T1 also asserts the sim's refusal for the second edge; the plant edits the fn only |
| ADV-U4FF1-5 | Medium | u4f-f1-plan.md:36-38 | The re-run's expectation is loose, though L1's determinism (scenario.rs:2582-2604) and the wire-free refusals let it prove the state-neutral claim at full scale | Condition 5: Block 3's expected figures; summary lines (but wall time) and per-tick `h` compared with 350677f's kept logs; any other difference stops for the advisor |
| ADV-U4FF1-6 | Medium | u4f-f1-plan.md:43-44 | The reviewers miss security-auditor on a change to how the server takes client input (CLAUDE.md step 5: "server"; U4f had it: u4f-report.md:64), with an unbounded first-command reach | Condition 6: add it to the one round, scoped to the hostile-input reach |
| ADV-U4FF1-7 | Low | world.rs:33-38, :1340-1341; room.rs:352-356, :1326-1329 | The plan corrects only world.rs:1340-1341; three more docs state the one-edge rule | Condition 8: correct all four |
| ADV-U4FF1-8 | Low | u4f-f1-plan.md:24 | "every honest command today but the six" is narrower than the code's multi-edge cases (Block 2's list) | Condition 9: "a 1-edge advance runs as now", with the list and "none pinned" |
| ADV-U4FF1-9 | Medium | u4f-f1-plan.md:29-32 | T2 compares one state hash from one state; my -20 ruling asked for the claim to be proven (advisor-u4f-20.md:75-78, :127-128) | Condition 4: the d10 twin form (equal hashes and events every tick), cases (a)-(f), exact refusal lists, two plants |

## Conditions
1. **The rule per counter, in today's order** (ADV-U4FF1-1, -2), per command:
   - the weapon's clock advances once (world.rs:1407-1417);
   - then the reload edges, then the fire edges, then the throw edges (authority only), each group at today's place;
   - **reload:** each edge through :1419-1428's check against the state the edge before it left;
   - **fire:** the command fires at most one shot, as today; each further fire edge is refused and counted,
     Ammunition if the magazine is empty, else Cooldown (:1457-1464's kind test);
   - **throw:** each edge throws or is refused as PoolFull (two grenades from one command is the rule: not
     state-neutral);
   - **melee:** taken, no effect, as today.
   A 1-edge advance runs today's code path exactly, bites included (World::BITES and its pins unchanged). What a bite
   does on an extra edge is written in the build notes.
2. **The one rule** (Q1, ADV-U4FF1-4): one integer-only `pub fn` in sim/src/seam.rs beside `Command`'s press indices.
   It gives the edges the sim takes from one counter's advance (last, now, as u8; the fix: `now.wrapping_sub(last)`).
   `run_command` loops exactly that count per button. `run_one` counts seen += advance, lost += advance - taken, and
   doubled += taken on a repeat.
3. **T1** as planned (option B, two late drops, a 2-edge command: seen 2, lost 0, doubled 0), plus the sim's side: the
   second edge's refusal in `reports().refusals` with that command's seq. Its plant edits the shared fn only
   (taken = min(advance, 1)) and fails T1 by name.
4. **T2** (ADV-U4FF1-9) in the d10 twin form (`course_stream` with one step named twice, against its twin; `twins`
   asserts equal state hashes and event lists every tick):
   - cases (a) fire held, ready; (b) not held, ready; (c) in the cooldown; (d) on the last round; (e) reload; each with
     its refusal list exact and its count +1;
   - (f) throw, apart: two projectiles with the command's seq, or one and a PoolFull with one slot left;
   - two plants: the old rule (the count half fails), and a second shot or a per-edge clock (the twin half fails),
     each by name.
5. **The re-run** (ADV-U4FF1-5): expected as Block 3 states. Its report compares each run's summary line (but wall
   time), the per-tick `h` of every room log, and `press_lost_count` with 350677f's kept logs. Any other difference
   stops for the advisor before the verdict counts. 350677f's folder stays.
6. **Reviewers** (ADV-U4FF1-6): security-auditor joins the one round, scoped to the loop's bounds, a session's first
   command and the path with no-press-edge bitten. The round is still one (batches of at most three).
7. **A session's first command** (ADV-U4FF1-3): the plan and the build notes name it and its F1 behaviour. F1 does not
   change kind 5. It goes into U4f2's brief as a named item for -21, decided there.
8. **Docs** (ADV-U4FF1-7): correct world.rs:33-38 and :1340-1341, and room.rs:352-356 and :1326-1329.
9. **Wording** (ADV-U4FF1-8): "a 1-edge advance runs as now", with Block 2's list of today's multi-edge cases and "none
   of them is pinned".
10. **U4f2's brief rebases on F1's merge** (Q3): its base and its count baseline (u4f2-brief.md:15, :128), and it takes
    condition 7's item.
The main session writes these into the plan (rev 1.1) or the builder's brief; no new plan review unless one cannot be
met. Lane, size and stop, builder setup and the round budget stand as planned.

## Verdict
**APPROVE WITH CONDITIONS (1-10).** The fix is R20-Q1's shape, inside the lane, with no pass mark, judge, marks.json,
money or main in it. The conditions name what the plan left open (all four counters, the clock and the one-shot rule,
the shared fn's contract, a session's first command) and make T1, T2 and the re-run prove the state-neutral claim
rather than assume it.

**To Mustafa:** nothing to decide. One line in the next day's message (TEST-FIRST rule 2): "U4f-F1: the stand-in sim
now takes every press edge, through its own checks; -20's L1 set re-runs on it (no money); the advisor approved the
plan with conditions; the edge check skips a session's first command, so that case goes to U4f2."
