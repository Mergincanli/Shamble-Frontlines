# The advisor's check of the pushed head after U4f's merge, 2026-11-19

Seat: a day session's advisor (general agent on main's `.claude/agents/advisor.md`, read whole with `git show
origin/main:`). Read only: this file is my one write in the repository. I built, ran and timed no Rust code. Lane:
`claude/netcode-proof-test-plan-hcky5g` (PR #40, draft). Merge commit 25d316e; pushed head c26ff6a; the lane's head at
my check is 138260c (one more card save, `[skip ci]`).

## 1. Verdict
**APPROVE WITH FINDINGS. The merge is right. The 17 pre-runs may start. Nothing goes to Mustafa.**
- **The merge is byte-exact.** I took b25b96a's spikes/ and docs/design with `git archive` and applied the snapshot
  (sha256 286aac60…, the same blob 79ba35c4 at 50e1d01 and at 25d316e^). `diff -r` against 25d316e's spikes/ and
  docs/design is empty (318 files). The 29 files are also byte-equal to builder 4's worktree (`cmp`, 29 of 29). The
  9 new files are mode 100644, and no file changes mode.
- **The commit holds nothing else.** Outside the 29 files it only deletes wip/u4f-build.patch. c26ff6a and 138260c
  touch only `.checkpoints/`. The spike tree f7ed90ad… is the same at 25d316e, c26ff6a and 138260c.
- **M1-M9, C1-C3 and D10 hold as recorded.** M8's "one commit" became three commits. That is accepted (note N1).
- **CI is green on c26ff6a.** Six check runs succeeded: gate, smoke and toolchain, each on push and on pull_request.
  The lane changes no `.github/`, gate or kit file against main, so that green stands.
- **The plan edits match the draft exactly.** I applied u4f-merge-edits.md's 12 old-to-new blocks to fca16a9^'s
  three files in memory. The result equals fca16a9's plan.md, u4-build-plan.md and u4s-carries.md. Each old text
  occurred exactly once. The five carries are the ruling's words. No mark, judge file, SERIES.md row or word of his
  changed.
- **Risk tier 1** (spike code under spikes/, plus docs/design). The merge into the lane needs no go from him (his
  SIZE-U4F (a), as for U4r: history.md:4659-4662). PR #40 keeps its own go.

## 2. Findings (CLAUDE.md's scale)
- **ADV-U4FMH-1, Medium: the card breaks his 200-line cap again.**
  - His rule: TEST-FIRST rule 3, history.md:2265, "The resume card goes back to a handoff: at most 200 lines."
  - The card is 231 lines at 138260c. It was 198 at bed9fa5 (ADV-M61H-1's trim), then 202 at 50e1d01, 209, 214, 218,
    219, 224, 226, 229 and 231 (`wc -l` at each save).
  - The growth is the Running work block: builder 4's record and the merge's step record (:179-214).
  - Stale lines in it:
    - "**Next (M9):** this save pushes it; CI on it", although CI already ran;
    - "Earlier: builder 4 RUNNING from 14:57 UTC";
    - "After the pass: … then 5.3's 21 pre-runs" (17 until U4f2);
    - "Disk 11,6xx MiB free (18:27 UTC)" in Last update (5,318 MiB at 19:29 UTC, `df -BM`).
  - Fix at the next card save. Move builder 4's and the merge's records to history.md by script, as
    card_trim_1119g.py did. Leave one done line ("U4f MERGED at 25d316e, pushed c26ff6a, CI green; this check:
    advisor-u4f-merge-head.md"). Refresh the stale lines, and bring the card to 200 lines or fewer.
  - It does not block the pre-runs: the card is a record, not an input to any run.
- **ADV-U4FMH-2, Low: `Room::cmd_timing()`'s route (item 5). My ruling: U4f2's lane. It stays public until then, with
  its reason recorded.**
  - The facts:
    - it is public (room/src/room.rs:621) and nothing calls it, tests included (my grep over spikes/netcode-proof);
    - A5's rule (advisor-u4e-r1.md:158-167) asks every public item for a consumer or a reason;
    - the reason recorded today is 2.3's setting, in u4-build-plan.md §4.1's new npt-room row (edit 6a) and in the
      report.
  - **Why not pub(crate) now:**
    - check.mjs's clippy runs with `-D warnings` (check.mjs:2015-2017), so an unused pub(crate) method fails as dead
      code. "pub(crate) now" really means "remove it now".
    - Any code change re-opens the merged tree. M5 (about 19 minutes), the gate and a new head check would run again
      before the pre-runs, and no behaviour would change.
    - The header's `params.cmd_timing` and the room's option already come from one setting (scenario.rs:1165 and
      :1903-1907), so nothing reads a wrong value meanwhile.
  - **Why not kept for good:** A5 made getters like it crate-private (`ClientSlot::skipped`, `Decoder::limits`), so
    "the setting's getter" alone is a weak reason.
  - **In U4f2:** its lane opens npt-room (C-U4F2-3) and the lab (the forged suite) anyway.
    - My lean: give the getter its consumer. The lab would write the header's `params.cmd_timing` from the live room
      (`server.room.cmd_timing()`), as FP7 did for `lead_bound_ms` (SEC-U4F-2's principle), and f6's header assert
      (scenario.rs:3649) would pin it.
    - Otherwise, remove it.
    - Either way, update §4.1's npt-room row in U4f2's merge.
  - Write it into U4f2's brief as a carry by number, beside C-U4F2-2 and C-U4F2-3. The main session picks the next
    free number.
  - Record it under "Choices taken without him" (inside the plan, undoable, no pass mark) and name it in the next
    day's message. It is not a security item: it is a read-only getter of a `Copy` enum.
- **ADV-U4FMH-3, Low (record): gate.txt names the wrong head.**
  - gate.txt says M7 ran "on the lane head 7432e34". But 7432e34 (18:47:33 UTC) is the save whose card already reports
    "M7 gate --no-smoke PASS". So either the gate ran on de3e9e4, or the card recorded the PASS before the run.
  - Nothing rests on which: the gate reads legacy/ only, and no commit from b25b96a to 138260c touches legacy/.
  - My own run on 138260c prints the same 11 lines (core 138 / game 714 / editor 353, +0 -0, 51 guarded refs).
  - Fix: one clause when gate.txt's line next moves (or in U4's report): "on de3e9e4's checkout, before the save
    7432e34 recorded it". No push for it.

**Notes (no finding):**
- **N1, M8's one commit became three, accepted.**
  - What went where:
    - fca16a9 (before the merge commit): the plan edits;
    - 25d316e: the 29 files and `git rm` of the snapshot;
    - c26ff6a: u4f-build.md's merge section and the report's corrections. The report's draft was already in the tree
      at 1a673e5.
  - This is the kit's own order: "Commit the step's work first, then … commit the checkpoint"
    (.claude/CHECKPOINTS.md:55-56).
  - 25d316e was pushed only together with c26ff6a: no Actions run names 25d316e. So no pushed head lacks the papers.
    The report's M8 line says all this.
  - The report's opening "in one commit" holds for the code.
- **N2, third-party check suites.** The cursor and claude check suites on c26ff6a are "queued" with 0 runs. They were
  the same on 631edf3 (advisor-main-61-head.md:123). This is noise, not a pending check.
- **N3, what CI covers.** The toolchain job builds the root workspace (`crates/*`), not spikes/netcode-proof, which is
  its own workspace. The spike's proof is M5's check.mjs run, which I tied to the commit (section 3). 138260c has no
  run of its own (card only, `[skip ci]`). PR #40's ready head needs its own run, as the card already says.

## 3. The checks, one by one
**1. The merge.** Shown in section 1. Also:
- fca16a9 equals b25b96a and 07289a4 in spikes/netcode-proof and docs/design (`git diff --quiet`).
- `git log 549ed77..fca16a9 -- spikes docs .claude .github CLAUDE.md` is empty.
- `git log 25d316e..138260c -- . ':!.checkpoints'` is empty.

**2. M1-M9, C1-C3, D10.**

| Condition | As recorded | My evidence |
|---|---|---|
| M1 | snapshot 286aac60… | `sha256sum` of 50e1d01's blob: 286aac60561ba684…, 394,400 bytes; the same blob at 25d316e^ |
| M2 | lane equals 07289a4; main unmoved | fca16a9 = 07289a4 = b25b96a (spikes/netcode-proof, docs/design); `git ls-remote` main f5030a3, an ancestor of 25d316e |
| M3, M4 | apply, cmp, hashes | my own `git apply --check` and apply on b25b96a's archive, then `diff -r` against 25d316e: empty; `cmp` against builder 4's worktree: 29 same. The runner's hashes rebuilt from the commits: `git diff --diff-filter=M d4f3f9a 25d316e -- .` gives **2f07abc2…**; the 9 new files' `sha256sum` list gives **1a767d5b…**. Both equal m5-check.log:3 |
| C1 | part-clean first | clean-shared.log:2 shows free 4,360 (under 4,600), HEAD d4f3f9a, diff hash e3b0c442… (empty: before the apply), lock idle 0 s, exit 0 |
| M5 | 11 steps, 634 / 731 | m5-check.log :447-458 read: every PASS line; test 865.9 s, 634 and 731, 0 failed, 0 skipped; wasm 27,000 ticks, 0 mismatching; refs 1,569; marks 81; self 78 of 78; redact 10 of 10. The only tree difference between M5's checkout and 25d316e is under `.checkpoints/`, which the refs step skips (check.mjs:1855) |
| C2 | at least 3,600,000 ms | the card says 7,200,000; the timeout is not on disk; the run ended at exit 0 in 18 min 56 s |
| M6 | nothing failed | no FAIL line in m5-check.log |
| M7 | gate --no-smoke; CI | my `node tools/gate.mjs --no-smoke` on 138260c: GATE PASS, the same lines as gate.txt; CI as in section 1 |
| D10 | empty | `git diff --stat 5b29296b 25d316e` and `5b29296b 138260c` on marks.json, crates/judge and marks/README.md: empty both |
| DEP-HOLD | no manifests | no Cargo.toml, Cargo.lock, package*.json, .npmrc, deny.toml or rust-toolchain among the commit's files |
| M8 | the one commit | N1 |
| M9 | push, saver, CI, card | push and CI above; `pgrep` shows `checkpoint.mjs watch netcode-proof-test` with both trailers; the card updated, but ADV-U4FMH-1 |
| C3 | the report's lines | all six present: the bytes (394,400 and 380,363) beside the character counts; ADV-U4FP-2's slips; 6,425 added, the pass +212, its diff 330 in and 121 out; +82 and +99; 46 negatives by row (my sum of the row list: 46); FP7's and FP2's notes; ADV-U4FP-3 as a U4f2 carry |

**3. CI on c26ff6a.**
- Push runs: gate 44596744717 (jobs gate and smoke) and toolchain 44596744078.
- Pull_request runs: gate 44596758828 and toolchain 44596758527.
- Every job concluded success, and so did every step ("Install Chromium and its system libraries" skipped, since the
  browser came from the cache).
- Check runs: 6 of 6 success, completed 19:17:46-19:19:00 UTC.

**4. The papers.**
- **The plan edits.** Shown in section 1. Also:
  - The removed lines are only plan text: the U4f entry, the Lanes line, and the U5, U6 and U7 rows, each kept as the
    prefix of its new text.
  - "Mustafa's go" in the U7 row stays.
  - Carry cites: C-U5-1 :138-140, C-U6-17 :141-143, C-U6-18 :144-145, C-U7-3 :146-148, C-U7-4 :149-154.
  - The LEAD-BOUND cite history.md:6791-6806 holds.
  - The cap figures (1,045 x 1.5 to 1,600; carry 50's bound 4,328) equal cap.txt:44 and room/src/events.rs:38-44.
  - marks.json:5227 calls the cap "a design constant", not a bar.
- **U4f2's stop past 3,000: it holds.**
  - His SIZE-CHECK says "'Well past its plan' means more than twice the plan" (history.md:2049). The doubling for a
    size letter applies only "where the plan gives only a size letter".
  - U4f2's entry gives a number, about 1,500. That number is the one in the option he chose: "(about 1,500 lines,
    with its own size line and review)" (advisor-u4f-stop.md:36-38; his (a), history.md:6896-6903). Twice it is 3,000.
  - So past 3,000 is his rule applied, not a new stop. Because the figure comes from his (a), U4f2's brief may confirm
    it but not raise it. If the brief's own estimate comes out above about 1,500, that goes to him before the build,
    rather than moving the stop.
  - The drafting note's "the advisor's check of U4f2's brief confirms or corrects it" (u4f-merge-edits.md, edit 2) is
    read that way. The applied plan entry itself is right.
- **The report: spot-checks, every one holds.**
  - The counts: 6,425 added and 310 removed; 3,609 into 20 files and 2,816 in 9 new files; the component table's merged
    column (1,521, 1,985, 720, 1,514, 48, 20, 453 and 164, summing to 6,425). Checked by numstat.
  - Room and events: `refusals_dropped` at room.rs:637; `cmd_timing` at :621; `EventQueue::CAP` 1,600 at
    events.rs:44.
  - Bots: `forger` and `take_forged` at bots lib.rs:310 and :322; f15's interval case at :1170.
  - New tests: `q9_a_…` at input.rs:568; `ea_u4f_6_…` at scenario.rs:4243.
  - The header write: FP7 at scenario.rs:1179-1186.
  - The rest of the cites: the 186-character line at client lib.rs:360; f17 at commands.rs:579; f31 at clock.rs:337;
    `canonical` at room.rs:444; the line ranges of EA-U4F-3's five inert lines (world.rs:1429-1457); `CHECK_BITES`
    behind its cfg (world.rs:945-951); `with_quant`'s consumer at loopback.rs:259; `with_lead_bound_ms`'s single
    test caller at commands.rs:818; the ClientSlot getters in logs.rs:613-621 and :703.
  - SERIES.md: the 15 new rows at :376-390, and 20 lines added, 0 removed against 5b29296b.
  - c-u4f-r2.mjs's 17 rows, split 6 for -20 and -47 and 11 for -21.
  - A1's two readers both say SEND WITH EDITS.

**5. `Room::cmd_timing()`.** ADV-U4FMH-2.

**6. Readiness for 5.3's 17 pre-runs and what follows: ready.**
- **R1, from the merge commit.**
  - At my check, no tracked change and nothing untracked under crates/ (`git status`), and no cargo running.
  - The script checks those itself (`committed()`), then names HEAD: a save after 25d316e, not 25d316e itself.
  - The record rule holds by the spike tree, f7ed90ad7fd08d39…, which equals 25d316e's. U4r's record run
    r2-e59ece9ecafc also ran from a save after its merge.
  - Name both 25d316e and that tree in 5.3's report line.
  - `node --check` passes on c-u4f-r2.mjs, c-u4f-runs.mjs and c-u4f-cap.mjs.
- **R2, disk.**
  - 5,318 MiB free at 19:29 UTC: 1,222 above the 4,096 floor. The shared target holds 6,323 MiB and
    .gate/netcode-proof-test/target 776.
  - The pre-runs project 17 x 12 MB (the script asks for about 4,292 MiB before its builds), plus the release builds'
    growth. They fit.
- **R3, reading the rows.**
  - A row that reads other than its `expect` comes to me before its set (5.3's last paragraph).
  - The -21 rows are an early read, not -21's record (r2.txt's head line, FP8).
  - The fast-clock rows read `ping_limited` 0 by design (S1), not "above 0" as u4f-brief.md 5.3 still says. The report
    names this.
- **R4, the part-clean at A6's place is needed, not optional.**
  - After the pre-runs, the sets' builds and logs would leave too thin a margin over the floor without it (5.5 item 2).
  - Then take D2's reading against the sets still to run (5.5; R26). With -21's 49 runs moved to U4f2, -47, -20 and
    -22 need about 700 MiB plus their builds, so KL-U4F should not be needed. If D2 shows the room short, KL-U4F
    goes to him as 5.5 item 3 says.
- **R5, the sets.**
  - One per call, in R2's order: 47, then 20, then 22.
  - As c-u4f-runs.mjs:3-8 says: `flock -x .gate/shared-target/.lane.lock`, with CARGO_TARGET_DIR unset, from the
    repository root.
  - -20's six 10-minute P3 runs are read for honest overflow resyncs before -22's set starts (advisor-u4f-stop.md:126-134).
  - The set reports carry NC-U4F-5 and C-OQ6-SEC-1 (-20), and N1 and the load (-22).
- **R6, the saver.** b02f5nmep's 110-minute window ends about 21:07 UTC: restart it then, inside the run window.
- **For Mustafa: nothing.** No pass mark, judge, marks.json, money, main, stop or run day is touched. The next day's
  message names ADV-U4FMH-2's route among the choices taken without him.

## 4. What I ran and read
**Read:**
- main's advisor.md, whole;
- the card, whole;
- advisor-u4f-r1.md :60-155 and :300-407;
- advisor-u4f-pass.md, whole;
- advisor-u4f-stop.md :20-140;
- u4f-merge-edits.md and apply_u4f_merge_edits.py, whole;
- u4f-report.md, whole, and its c26ff6a diff;
- u4f-build.md's merge section;
- history.md :2040-2080, :2263-2266, :4654-4663, :6789-6808 and :6815-6820, :6890-6905;
- advisor-u4e-r1.md :150-172;
- advisor-main-61-head.md's findings table;
- u4f-brief.md 5.3 (greps) and 5.5 :1080-1119;
- clean-shared.log, m5-check.log and gate.txt, whole;
- cap.txt :21-44;
- in the merged tree: room.rs :585-640; scenario.rs :1158-1186 and :1898-1912; c-u4f-r2.mjs, whole; c-u4f-runs.mjs
  :1-71; check.mjs :1852-1866 and :2625-2631; marks.json:5227;
- toolchain.yml.

**Ran (read only in the repository):**
- `git fetch`, `git log`, `git show`, `git diff` (name-status, numstat, raw, quiet, stat), `git rev-parse`,
  `git merge-base`, `git ls-remote`;
- `git archive` of b25b96a and 25d316e into my scratchpad; `git apply --check`, then `git apply` there; `diff -r`;
- `cmp` of the 29 files against builder 4's worktree;
- the runner's two hash pipelines, rebuilt from the commits;
- an in-memory Python check (`python3 -I`, no file written) of the 12 plan edits, and of the five carries against the
  ruling;
- `node tools/gate.mjs --no-smoke`;
- `node --check` of the three run scripts;
- `gh api` for check runs, check suites, workflow runs, jobs and PR #40;
- `df -BM`, `du -sm`, `pgrep`, greps.

**My scratch copies** (my scratchpad, outside the repository):
- two `git archive | tar` extracts;
- one `git show … > u4f-build.patch` copy of the snapshot. That is a shell redirect, against the standing write line's
  letter. It touched no repository file.

**Not checked:**
- I ran no cargo, check.mjs, judge, smoke or pre-run. Every count is read from the logs, which I tied to the merged
  content by their two hashes.
- C2's timeout value is not on disk.
- I did not read builder 4's transcript, the seats' notes, or the report's sections beyond the spot-checks named.

## Decisions needed
None.

## For Mustafa
Nothing.
