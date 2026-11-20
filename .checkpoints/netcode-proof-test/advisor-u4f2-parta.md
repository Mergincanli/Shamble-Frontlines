# Advisor ruling: U4f2's fix pass part A (2026-11-20), F-G, the early runs, and what part A's report changes

Read only (this file is my one write). Role: main's `.claude/agents/advisor.md`, as in advisor-u4f2-r1.md (left as it
is). Sources: the coordinator's message; history.md:7402-7410 (his words); the build record's "Block 9: the fix pass";
.gate/u4f/u4f2-fp*.log; the worktree (absolute paths only); wip/u4f2-build.patch (sha256 23c5f788…).

## Plan
1. Read his words (history.md:7402-7410), Block 9, the fp logs (T-K7's interval-3 form at tick 257, plant 9's B
   result, the size and times), and the code: bots `forge_step` and `fire_rest_at`/`forger(7, …)`, the twin path.
2. F-G: confirm the mechanism in the code; rule keep 3 (a limit) or require `every` above the rest.
3. FP-5's plant 9 under B: does the result hold, what F-E now means, and anything owed.
4. Early runs: whether R4 and R5 may run on part A's tree and count; what re-runs after FP-2.
5. Size against my 2,700 stop with FP-2 to come; the test time; the disk; the slips.
6. Tables and the short return.
Each block below as it finishes.

## Block 0: what I checked (DONE)
- Snapshot wip/u4f2-build.patch sha256 23c5f78816f33e480ec6b7d8f0e16cbadec4e66b6a9e5a1aeef5caca6f958abc (my
  `sha256sum`, equal to the message's). The build record equal in both copies (4b5e1b60…); Block 9 read whole
  (u4f2-build.md:268-405).
- His words (history.md:7402-7408, read verbatim): "continue the work, the advisor will be answering the question in 2
  days, until then keep progressing. Things that are very important which needs an answer: you can ask me". The main
  session's reading (:7410) is its own; I read "the question" as F-B-COUNT, as it does.
- Code read in the worktree: bots lib.rs:151-172 (the kind-7 doc, `fire_rest_at`), :575-591 (`forger`: kind 7 taken
  when `every >= r`), :925-967 (`forge_step`: `rest` over k = 1 to the rest, `ahead(k) && (k == 1 || !reload(n + k))`);
  sim world.rs:2063-2067 (the tick's events in the state hash, each `Shot` with its `seq`); lab scenario.rs:1576-1586
  (`Summary::soldiers` pushed on every tick of every lab test run, `#[cfg(test)]`); c-u4f-r2.mjs:52, :61-64 (p1, p7,
  p14 and p17 are all forged-suite rows).

## Block 1: F-G (DONE)
- **The mechanism holds in the code.** At `every` equal to the rest R (3 at 30 Hz), every command n has some k in 1..=R
  with n + k due, so `rest` releases fire on every command, d included (bots lib.rs:933). A fire item whose d-1 ran
  dead and d alive meets a weapon the revive left ready (world.rs:1192-1203): the forged edge alone fires
  (world.rs:1473: `ready && (fire_held || edge)`), the twin's d (no edge, no held fire) does not, so the state differs
  (a shot and its `Shot` event, hashed at world.rs:2063-2067): a twin difference, an effect beyond the valid part, not
  only a missing record. At `every` above R, d is not inside the next item's rest, so it holds fire and the twin's held
  trigger fires the same shot (T-K7's case "fire d - 1 dead", PASS at 12). The builder saw it as T-K7's first form at 3
  (u4f2-fp1-tests.log, the hash at tick 257).
- **Severity: Low.** No judged run reaches it (12, against R = 3 at 30 Hz and 5 at 60 Hz); it gives a false FAIL of
  the twin check, never a false pass. FP-2 cannot cover it (FP-2 leaves the item out of the count; the twin check
  still reads the shot).
- **Ruling: require `every` above the rest (fix now, before the early runs).** An accepted setting whose twin can
  differ by construction is the kind of setting FP-3 already refuses, and the fix is a few lines in an open file. The
  builder rightly did not build it: my FP-3 text said "3 taken", which this ruling replaces:
  - `forger(7, every, _)` takes `every > r` (4 at 30 Hz, 6 at 60 Hz), else `None`; its doc and the kind-7 doc say why
    (d must hold fire, so a revived d fires as the twin's held trigger does); the lab's usage error names it ("kind 7
    needs an interval above its rest (R)").
  - fp3's pins move by one: `forger(7, rest)` `None`, `forger(7, rest + 1)` taken; c_u4f2_1's refused run stays at
    `--forge-every 2` (now refused for this reason too) and one case adds 3 refused and 4 taken at 30 Hz, the parser's
    floor of 2 unchanged (C-U4F2-1).
  - f15's kind-7 case runs at `rest + 1` (4), its rested-command count unchanged.
  - Plant: the condition back to `every >= r`: fp3 fails by name (the `forger(7, rest)` pin).
  - About 5-10 added lines. It rides round 2 (netcode and rust seats).
- **Record line:** U4f2's report names F-G beside SA-U4F2-2 (kind 7's interval floor: above the rest, from the weapon
  and the rate).

## Block 2: FP-5's plant 9 under option B (DONE)
- **The result holds, and it is better than my acceptance expected.** I wrote that plant 9 in its right form would fail
  f16 under option A and pass under B (advisor-u4f2-r1.md, FP-5's row). It failed both (u4f2-fp5-plant9.log; Block 9
  :319-326). The builder's reading of B is right by the code: the state hash covers the tick's events, and a `Shot`
  carries its command's `seq` (world.rs:2063-2067). Under B the forger's tick re-runs the last command (seq d-1), while
  a twin that sends the honest command fires under seq d, so the hashes differ on the ticks with a shot and agree again
  after: the single ticks 50, 62, 74, … at the 12-command cadence.
- **So F-E is closed under both options:** f16 tells "never made" from "made honest" under A (one command more run,
  from tick 13) and under B (the shot's sequence). Round 1's plant 9 passed because the plant made the twin forge, as
  NC-U4F2-4 read. My FP-5 acceptance line ("option B passes; F-E recorded as option B's limit") is superseded: the
  record line reads "F-E was the first plant, not a limit; closed under both options (A by the extra command, B by the
  shot's sequence in the hash)". FP-5 is met. Nothing owed.

## Block 3: the early runs (DONE)
His KEEP-PROGRESSING (history.md:7406) leaves about two days before FP-2 can be built. Runs that do not depend on
FP-2 should use them.
- **What FP-2 touches** (advisor-u4f2-r1.md, FP-2's row): forged.rs (the attribution), scenario.rs's passing of the
  forgers' alive flags into it, and one SERIES.md row. Nothing in the lag-switch, fast-clock, clamp or forger code, and
  nothing that changes a command, a datagram or the sim's state. So the logs of a run with no forger should be equal
  byte for byte before and after FP-2, and every `forged_kind` record of a forged run changes.
- **Order now:** F-G's fix first (block 1), with its tests and plant and a snapshot, so that every early run reads one
  tree. Then:
  - **R0, the test step's new cost** (cheap, about 50 s): `c62hb_p2_short_every_measured_lost_tick_is_explained` (the
    yardstick, 16.73 s alone at u4f2-times-f16.log) and `t6_a_duplicate_admit_…` (the slowest lab test, 31.94 s at
    u4f2-times-rest.log), each alone, measured build. `Summary::soldiers` is now pushed on every tick of every lab test
    run (scenario.rs:1579-1586), and Block 9 did not time it (:398-399). If the two together rise by more than 1 s,
    gather the records only on the runs f16 compares (a setting on `pair`) before R6. R6 still reads the whole step.
  - **R4, FP-4's power: may run now and counts** (no forger in the lag-switch suite).
  - **R5, section 6 item 5 whole: may run now.** Its lines with no forger count for the final tree: set 21's clamp A
    and B (measured and twin), fast-clock A and B, lag-switch A and B, the bites refuse-not-clamp (clamp) and
    not-older-than (fast-clock), and the fast-clock tick-hash proof against hand-a's run. Its forged lines also run now,
    as a smoke that does not count: FP-8's new usage errors (`--suite forged` only at P0 and with no `--lead`) are met
    first by the run scripts' own command lines here.
  - **R2-pre and R3-pre, the 610 s forged pairs and bite-no-lead-bound: also run now, as a pre-read that does not
    count.** The figures that do not depend on FP-2 are read now: twin mismatch under A and B (FP-1's fix at length,
    with real deaths: the one figure no unit test reaches); the shots under A, under B and on the thinnest bite
    (TE-U4F2-6); sent counts, slot 6's re-runs under B, `counted_count` for kinds 8, 9, 11. Kind 7's `unrejected_count`
    is stated as found (dead-window items expected; unproven until FP-2). A twin mismatch above 0, a kind other than 7
    unrejected, or shots under 670 comes to me at once: none of them waits for his answer. Their cost: 5 runs, about 50
    minutes, about 0.33 GB of logs, kept. They run only if free stays above 6.5 GB after R0, R4 and R5.
- **After FP-2 is built, re-run:**
  - **R5's forged lines:** set 21's forged A and B, measured and twin (4); the 11 forged no-* bites; r2's p1, p7, p14
    and p17 (all forged-suite rows, c-u4f-r2.mjs:61-64); and both judge calls in 5.4's form. The measured call may read
    the non-forged lines' logs from the early run.
  - **R2 and R3 whole**, counted (advisor-u4f2-r1.md: block 1's proof, block 4's R2 and R3 figures).
  - **One non-forged line again, as the proof that the early lines still stand:** set 21's lag-switch A line (it runs
    through the most scenario.rs code at P1). Its lab log's fate_log sha256 and its room log's per-tick hashes must
    equal the early run's, and FP-2's diff must show only forged.rs, scenario.rs's attribution wiring and SERIES.md. If
    either fails, R4 and all of R5 re-run.
  - **Anything a round-2 fix touches** re-runs as advisor-u4f2-r1.md's order says (I name the lines).
- **Disk:** 8,368,844,800 B free (the message). The floor is 4 GiB (4,294,967,296 B), so about 4.07 GB of headroom.
  The early runs need about 0.8 GB: the release builds (about 0.3 GB from an empty release target, by the cleans'
  259 MiB), R5's short runs (about 0.1 GB), R2-pre and R3-pre (0.33 GB), and R0 and R4 (small). That leaves about
  7.5 GB. FP-2's debug build and its re-runs (about 0.5 GB of logs) take it to about 6-6.5 GB. Then a part-clean comes
  before each R6 run and before the merge's main-folder check: about three, the main session's, with no lane running
  (P27).

## Block 4: what else part A's report changes (DONE)
- **Size against my 2,700 stop.**
  - Part A ran about 485 added lines (2,077 to 2,562), against the 180-310 I estimated for it. T-K7 alone was about 180.
  - With F-G (about 5-10) and FP-2 (60-100), U4f2 reaches about 2,630-2,670. At FP-2's 1.5x it would reach about
    2,720, before any round-2 fix.
  - My stop at 2,700 was the card's size line (the main session's reading of his (a), history.md:7271), not his
    stop. Its purpose was that I see an overrun before more is built, and I see it now.
  - **Ruling:** my stop moves to **2,800 added, at an FP or fix end** (the builder stops and reports to me). That
    leaves 200 for round 2's fixes under **his stop past 3,000 at a block end, which stands**.
  - The figure he is told changes: see the message, below.
- **Test time:** about 6 s a build, 12 s in all, inside the 30 s I allowed, with two notes:
  - c_u4f2_1 went from 0.00 s to 4.2 s, because the refused run builds the server before slot 6's forger refuses.
    That is inside the budget; no action.
  - `Summary::soldiers`' cost on every other lab test is unmeasured: R0 (block 3).
  - R6 reads the whole step against 1,207 s, as ruled.
- **The slips:**
  - **FP-6's second plant was put in before the first restore was hashed.** The same slip happened in the build
    (plants 3 and 4, u4f2-build.md:166-167). Both times the restores were checked equal afterwards (c7ba5f38…), so no
    file was affected.
    - Ruling: recorded, and from now on one plant at a time: restore, hash, then the next plant. A second plant
      applied before the restore is hashed must be restored and re-run alone.
  - **Logs and two small files were read through the shell** (`grep`, `sed -n`, `head`, `cat`) where the rules ask
    for the Read and Grep tools, and one `git -C` call was refused by the guard. No file was written and nothing
    changed.
    - Ruling: recorded; the builder uses the Read and Grep tools for logs.
  - Neither slip touches evidence: every plant's log and every restore hash stands.
- **FP-3's acceptance**, as amended by block 1:
  - the pins move to `forger(7, rest)` `None` and `forger(7, rest + 1)` taken;
  - 2 and 3 are refused for kind 7 at 30 Hz, 4 is taken;
  - the parser's floor of 2 stays.
- **FP-5's acceptance** as amended by block 2: F-E is closed under both options.
- **Round 2** is unchanged: netcode, test and rust, on the fix pass's diff, after FP-2 and R1-R5. F-G's fix and
  `Summary::soldiers` (or its narrowing, after R0) join its scope.
- **The 2026-11-21 message** (msg-2026-11-21.md:47-48 still reads "about 2,320 to 2,490"):
  - If it has not gone out, its size sentence becomes: "U4f2's size after them: about 2,630 to 2,670 added lines in
    all (2,562 built so far; the counting fix that waits for your answer adds about 60 to 100), above the 1,700 to
    2,250 your (a) was given, and under the stop at 3,000, which stays and is not reached."
  - If it has gone out, the next day's message corrects the figure in one line.
  - F-G and the early runs need no line for him: they are inside the plan, undoable, and touch no pass mark, judge,
    marks.json, existing SERIES.md row, money or main.
  - Nothing here meets his "very important" bar for an ask outside that message.

## Rulings
| Item | Ruling | Why |
|---|---|---|
| F-G (Low) | **fix now: kind 7 requires `every` above its rest** (4 at 30 Hz, 6 at 60 Hz); FP-3's "3 taken" replaced; pins, f15 at rest + 1, a 3/4 case, plant `every >= r` | at `every` = rest, d holds no fire, so a revived d fires by the forged edge alone: a twin difference (world.rs:1473, :2063-2067); no judged reach; FP-2 cannot cover it |
| FP-5's plant 9 under B | **result accepted; F-E closed under both options**; my B expectation corrected | the hash covers each `Shot`'s seq (world.rs:2063-2067): a re-run (d-1) and a fresh honest command (d) differ |
| R4 | **may run now and counts** | no forger in the lag-switch suite; FP-2 touches only the attribution |
| R5 | **may run now**; its non-forged lines and the fast-clock proof count; its forged lines run as a smoke | as R4; FP-8's usage errors meet the scripts' lines now |
| After FP-2 | **re-run** R5's forged lines (forged A/B measured and twin, the 11 forged bites, r2's p1, p7, p14, p17), both judge calls, R2 and R3 whole; plus the lag-switch A line with its fate-log sha256 and room-log hashes equal to the early run, and FP-2's diff only in forged.rs, the attribution's wiring and SERIES.md (else R4 and R5 re-run) | the forged_kind records change with FP-2; nothing else should |
| R0 (new) | yardstick and slowest lab test re-timed now; above +1 s together, `Summary::soldiers` only on f16's pairs | per-tick pushes on every lab test, unmeasured |
| R2-pre, R3-pre (new) | **run now as a pre-read, not counted**, if free stays above 6.5 GB; twin mismatch above 0, a kind other than 7 unrejected, or shots under 670 come to me at once | the twin hash at 600 s with deaths and the shot band are the figures most likely to send work back; two idle days |
| My size stop | **moves from 2,700 to 2,800** at an FP or fix end; his past 3,000 stands | part A ran about 485 against 180-310; FP-2 brings about 2,630-2,670 |
| Slips | **recorded; one plant at a time; Read/Grep for logs** | no file or evidence affected |
| The 11-21 message | size sentence to about 2,630-2,670 if unsent, else corrected next day; nothing else for him | inside the plan, undoable, no pass mark |
