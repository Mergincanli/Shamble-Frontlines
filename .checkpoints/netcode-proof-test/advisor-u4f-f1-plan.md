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
    edges change state: each edge throws while the pool has room (world.rs:1500-1505), so a 2-edge throw is two
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
