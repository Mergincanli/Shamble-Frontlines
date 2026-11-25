# Advisor: NB2-MERGE plan check (Builder 2's branch into PR #40's branch)
2026-11-25, 03:5x-04:5x UTC. Read only, effort xhigh. Acting by main's advisor.md (`git show
origin/main:.claude/agents/advisor.md`, read whole). Brief: nb2-merge-request.txt, sha256 df3f2576…2acb6 (checked: equal).
Plan: nb2-merge-plan.md, sha256 2b7fb042…fe479 (checked: equal). HEAD at launch bae62327fc89 (= origin's netcode
branch); T = 0e621dae6e5b009a4cf00b6401b46d771b2215a3. Form: main-65-plan.md and advisor-main-65.md. Notes of every
check: section 7.
(One slip, read only: my first Bash call was compound: `cd … && sha256sum … && git rev-parse HEAD && git branch
--show-current && git status --short | head`. Every later call was one simple command.)

## 1. Verdict: APPROVE WITH CONDITIONS NB2-C1 to NB2-C12
The merge is his word (history.md:7909, item 4). Every fact the plan states holds when I re-run it. His condition holds
at T: each of the five commits after 72651a3c touches only `.checkpoints/netcode-judge.md` or
`.checkpoints/netcode-judge/`. The guard's pathspec over e2c0e896..T prints nothing. The trial merge is clean on
today's head. SERIES.md's two sides are about 490 lines apart. The -47 logs are where the plan says.
At the tip, PM-0001-47 reads MOVED on both calls, with exit 0 (section 4).
Four things need fixing before it runs:
- The order: U7's 0b check is due the hour U6c merges (NB2-C1).
- The gap's length: step 4 will likely be cold, not warm (NB2-C2).
- Step 2 always asks him: the guard asks on every merge into the main folder, and a pending ask freezes the session
  (NB2-C3).
- What a stop after the merge commit does (Q2: NB2-C4).
No Critical or High. Risk tier 1: spike code and notes; no kit, CI, gate or gitignored path; every file mode 100644.

## 2. Conditions (each with where it is met)
- **NB2-C1 (Medium): U7's Block 0b binding check goes first.**
  - It "runs the hour U6c merges" (advisor-u7-brief.md:219-220). It is commands, not an agent (advisor-main-65.md:
    159-160). It runs cargo check, deny and audit, and reads check.mjs's counts and Cargo.lock: the lane and the main
    folder.
  - So it is R1's path, due at U6c's M9, not "in a reset's first 30 minutes" as plan :23-24 files it. NB2's step 1
    starts only after 0b is done and on the card (or the card says why it is not due).
  - 0b edits two manifests and the lock in the main folder and ends with its own save (u7-brief.md:245-251, :362), so
    H0 will hold it. Two bars follow:
    - Before 1g, `git status --short --untracked-files=all` lists nothing outside `.checkpoints/`.
    - The main-run.sh headers of 1i, step 4 and step 5 (main-run.sh:42, the diff and untracked hashes) read the same
      clean values. A dirty tree: NB2 waits.
  - The "+47" bars then rest on 0b's own check counts, which its item (10) holds equal to M5's.
  - Met: before step 1; the plan's Order text; 1i, 4, 5.
- **NB2-C2 (Medium): the gap is measured, not assumed.**
  - Card :148 expects U6c's near-cold M5 to fill about 7.8 GB, so 1f will likely land under 8,192 MiB. Then the
    part-clean runs, and step 4 is cold. The 45-minute projection (U6b's warm 23.5 min) and the Risks line "mostly
    warm" then do not hold.
  - After 1f, and after any clean, project the end from:
    - U6c's M5 measured time if a clean ran (cold), else 23.5 min;
    - plus about 10 min for the judge's build and the two calls;
    - plus the wait for his answer (NB2-C3).
  - If that end passes the next R1-path step's due time, stop before 1g with nothing changed, and wait for the next
    gap.
  - Met: after 1f; Risks text.
- **NB2-C3 (Medium): step 2 asks, every time, and only he can answer it.**
  - The guard's main-folder rule (routine-guard.mjs:1342-1346, :1360-1362, this branch = main's kit) asks for any merge
    but `--ff-only origin/main` or a one-word `git merge origin/main`.
  - A call that waits for him holds the session and the lane (his stall rule, history.md:2187).
  - So:
    - (a) After 1i, name the exact step-2 command in the chat (history.md:2187), set the board thread to needs-musti
      with a due label naming nb2-merge-plan.md (his item 7), and end the turn.
    - (b) Send step 2 only after his reply in the chat, so the prompt appears while he is there. His yes at that
      prompt, with his item 4, is the go for that command.
    - (c) If he says no, or does not reply before an R1-path step is due: nothing is merged. The saver restarts, R1's
      step runs, and NB2 starts again at 1a in the next gap. H0 is pushed, so there is nothing to undo.
    - (d) Plan :89-91 reads "This call asks (the guard's main-folder rule)", not "may ask … the permission mode
      decides".
  - Met: steps 1i-2; Risks; For Mustafa.
- **NB2-C4 (Medium): a stop after the merge commit exists (my Q2 ruling, section 3).**
  - (a) Step 5 reads anything but MOVED, or exits non-zero: option (i).
    - His three lines go out at once, in his words.
    - No save, commit, push or revert; notes by Edit only (M65-1's form).
    - R1's path waits for his answer or the advisor's ruling at the next window, whichever comes first.
  - (b) A stop in step 3, 4 or 6: option (i) by default (the advisor rules at the next window). Option (ii) only when
    an R1-path step must use the lane or the main folder before that ruling:
    - `git revert -m 1 --no-edit HEAD` in its own call, named in the chat first.
    - Then `git rev-parse HEAD^` = M (the merge), and `git diff --quiet H0 HEAD` exits 0.
    - One card save pushes M, the revert R and the card. CI is green on it, read as advisor-main-65.md §4 (a).
    - Builder 2 is told at once to hold: it must not take the netcode head in until the merge is redone. A head
      holding R would delete its own judge work from its branch.
    - The card names M, R and the stop's log. The redo is `git revert R`, never `git merge T` again (git counts T as
      merged), plan first, after the ruling.
    - If the revert asks and is refused, or its tree is not H0's: option (i).
  - (c) The stop line "nothing is pushed" reads "nothing of the merge reaches the pushed tree; only NB2-C4 (b)'s pair
    may be pushed".
  - (d) 1h's resume line adds: "If the container was lost, the unpushed merge is gone and HEAD is H0; step 2 is not
    redone before the ruling."
  - Met: Stop rules; 1h.
- **NB2-C5 (Low): the part-clean's form, in steps 1f, 4 and 5.**
  - The form is the card's Tools line (netcode-proof-test.md:193-195; the same as advisor-u6b-m6.md:162-163):
    `env CARGO_NET_OFFLINE=true flock -x <M>/.gate/shared-target/.lane.lock cargo clean --manifest-path
    <M>/spikes/netcode-proof/Cargo.toml --target-dir <M>/.gate/shared-target --profile dev`. Then `test` the lock,
    CACHEDIR.TAG and that debug/ is gone, then df. Run it with no lane running (pgrep cargo and rustc empty).
  - Not u4r-disk-plan.md §5: its folder, .gate/netcode-proof-test/target, has no debug/ left.
  - Its real cost: R1's next whole check runs cold. Put that on the card's disk line.
  - Met: 1f, 4, 5.
- **NB2-C6 (Low): step 5's bar, exactly.**
  - Each call exits 0 and prints one line: section 4's line, with "3 run(s)" for measured and "2 run(s)" for bites.
  - Each out file's PM-0001-47 entry reads "verdict": "MOVED" with moved.to "Slice 1 change-set 7 (Lag)".
  - Exit 1 (a malformed log or marks file) is a stop under NB2-C4 (a).
  - Give each call `env CARGO_INCREMENTAL=0` as step 4 does: less disk, no result changes.
  - Met: step 5.
- **NB2-C7 (Low): bind steps 3 and 4 tighter.**
  - (a) After 1h's save, run `git merge-tree --write-tree --name-only H0 T`: one line, tree X. Step 3 adds `git
    rev-parse HEAD^{tree}` = X. git merge and merge-tree use the same merge code; a difference stops, and the advisor
    rules.
  - (b) Step 3's kit line uses the guard's full pathspec (main-61-plan.md:37-40) plus `tools`, over H0..HEAD: empty.
  - (c) Step 4's bars:
    - redact = 1i's items + 10, 0 findings (redact walks the spike folder on disk: redact-check.mjs:277-288,
      :378-385; the merge adds 10 fixture files there);
    - refs = N0 + 32 (`git ls-files`, check.mjs:2625-2631);
    - self = 1i's and M5's.
  - Met: 1h, 3, 4.
- **NB2-C8 (Low): if Builder 2's tip moved.**
  - If T at 1a is not 0e621dae but 1b holds and `git diff --quiet 72651a3c T -- spikes/` exits 0, this approval
    stands, and step 3's counts come from 1c at T.
  - Any spike difference: stop.
  - Met: step 1.
- **NB2-C9 (Low): Risks gains the cross-reads.**
  - SERIES.md is read by lab/src/contract.rs and by three of U6c's tests: bots/tests/load.rs:287,
    server/src/probe_tests.rs:286 and net/src/l2/socket_tests.rs:394, each checking the file contains a name.
  - marks.json is read by room/tests/bites.rs:31, server/tests/schedule.rs:355 and check.test.mjs:1776.
  - The trial merge cannot show how these meet; step 4 does. Builder 2 drops no SERIES.md row name, and the two sides
    add no name in common, so the name checks cannot newly fail.
  - Met: Risks text.
- **NB2-C10 (Medium): one reviewer's read of 72651a3c's last judge lines, then the report's own line under his
  2026-11-15 rule (history.md:2127).**
  - His rule asks for "a reviewer's read before its unit closes". 72651a3c's last 67 lines after the re-check's head
    67892d39 (figures.rs's no-tail guard, two engine tests, README and SERIES text) had the advisor agent's read and
    his main Advisor's check, not a reviewer agent's. His 24 November item 4 ("have your advisor agent read it")
    covers 67892d39 only. Whether the advisor agent's read counts is his to say; I do not settle it by approving.
  - So: a short read-only test-engineer read (or rust-reviewer) of `git diff 67892d39 72651a3c --
    spikes/netcode-proof/crates/judge spikes/netcode-proof/marks`, in step 9's window, before NB2-MERGE's report.
  - The merge need not wait: the rule's deadline is the unit's close, and the merge changes no byte of those lines. A
    finding above Low gets a forward fix, plan first, as the advisor rules.
  - The report's line then names what covered what:
  - part a: b22c30cd, read by test-engineer and rust-reviewer; its fix pass 0e0f6547, by the advisor agent;
  - part b: a3476e6c, read by both reviewers. The fix pass 7c3855c7 with 67892d39 (marks.json's five lines; his 24
    November item 4: "have your advisor agent read it. No new review round for it."): both reviewers' re-check on
    67892d39, and the advisor agent;
  - 72651a3c's last 67 lines: the advisor agent's APPROVE, his main Advisor's check, and this condition's reviewer
    read. The test-engineer's re-check had proposed that guard;
  - this merge changes no judge or marks.json byte (step 3's empty diff against T).
  - Met: step 9's window; the report.
- **NB2-C11 (Low): -57 is not run in this merge** (section 3). The report's "not verified" list names it, with the
  reason. Met: the report.
- **NB2-C12 (Low): text.**
  - "33 table rows" is 31 rows plus the table's header and separator.
  - "self 78 of 78 at both" means at 0e0f6547 and 72651a3c (Builder 2's card gives no self count at the base).
  - The Record says Builder 2's hand-off reads "Before the merge: the re-judge"; his item 4 ("After the merge")
    governs.
  - `git merge --abort` is out because the guard asks for it (advisor-main-54.md F26), not because of the deny list
    (`git reset --merge*` does not match it). The conclusion stands.
  - PR #40's close also deletes `.checkpoints/netcode-judge*` (another slug), after Builder 2's saved messages of 24 and
    25 November are carried where his words are kept (the 24th into DECISIONS.md, his item 5).
  - Met: the plan's text; PR #40's close.

## 3. The plan's questions, and the two pre-rulings
- **Q1, the main folder: yes**, as one `git merge --no-ff` call, through the guard's ask answered by him (NB2-C3).
  - CLAUDE.md's Never line names `merge` as a way of switching the main folder, and its "Still allowed" list does not
    name this one. I do not waive that line.
  - What carries it: his item 4 orders this merge into this branch, which lives in the main folder. The guard asks him
    for this exact command. And the line's reason does not arise:
    - T changes no `.claude/`, settings, hook or agent file (the pathspec is empty);
    - this branch's kit equals main's (`git diff --stat HEAD origin/main -- .claude CLAUDE.md` is empty).
  - A worktree is not better. Merging there needs a second cargo cache (a cold build, about 8 GB) or the shared one.
    Then bringing the main folder up to the merged head asks him again: `--ff-only` to anything but origin/main asks
    (routine-guard.mjs:1343-1346).
  - Kit follow-up, after the freeze, like ADV-M54-8: the "Still allowed" list names no merge of another lane's branch
    that changes no kit file.
- **Q2, an unpushed merge commit after a stop: (i) by default, and always for step 5; (ii) only as NB2-C4 (b) says.**
  - Step 5 is his "stop and tell me in three lines": nothing is undone before he hears.
  - For steps 3, 4 and 6, (ii) keeps "R1's path goes first" when an R1 step cannot wait. Two hazards the plan did not
    name are now covered: Builder 2 taking in a reverted head, and a redo by `git merge`.
- **Step 5's disk fallback: pre-ruled, yes.**
  - Read the space as main-run.sh does: `df -B1 --output=avail /home/user`. Under 4,294,967,296 B, run NB2-C5's
    part-clean, then measure again.
  - That costs the judge's rebuild here (about 212 MiB: card :148) and a cold start for R1's next whole check.
  - The part-clean takes the lane lock itself, outside main-run.sh, so it runs under main-run.sh's floor
    (main-run.sh:38-47 exits 3 before any command).
  - Still under 4 GiB after it: stop; the advisor rules. Record both df lines on the card.
- **-57's re-judge: no, not in this merge.** Its line is not runnable as worded, and it is outside his item 4:
  - The re-plan's real-data line (replan-marks.md:207) also names "-57's folder still PASS". Part a's plan narrowed it
    to -47 (plan-a.md:73), and its ruling (5) handed on only -47's command. Nobody ran or dropped -57.
  - His item 4 names only -47, with its own stop.
  - At the tip -57 has no moved piece (marks.json:4074-4170). Every kept set reads it NOT-RUN at the mark level: each
    holds one profile, and only its own part reads PASS (results/u4r/*/P2|P3/runs.txt). So no kept folder can read
    "PASS".
  - The kept sets are 291-741 MB, with no measured judge peak at the tip's judge.
  - It becomes a Low follow-up in the part-level form: one kept P3 set (291 MB), `--only PM-0001-57`, "L1 P3: PASS" in
    the out file. Its home is PR #40's close verification, or an earlier unit that judges a kept set anyway.

## 4. The re-judge (check 3): MOVED on both calls, exit 0
- **Why MOVED:**
  - At the tip, PM-0001-47 carries a mark-level `moved` (marks.json:9250-9254).
  - judge_mark returns MOVED for it before any part, check or bite is judged (eval.rs:1737-1746), so the headline order
    (eval.rs:1838-1849: FAIL, INVALID, NOT-RUN, then MOVED) never applies to it.
  - That is why both halves read MOVED. Before, measured/ alone read NOT-RUN (no bite runs) and bites/ alone NOT-RUN
    (0 of 3 valid runs) (runs.txt:9, :12).
- **What could stop it first:** only `read_logs` (exit 1 on a malformed log; main.rs's exit codes).
  - Builder 2 left read_logs unchanged.
  - npt-log changed since b57221e only by additions (Machine::read_local, cpu_list, a test; +111 -0). Neither branch
    touches it, so the logs written at b57221e parse as they did.
- **The two lines, exactly** (`{:<12} {:<9} {first}`, lib.rs:50-62; `moved to {to}: {why} ({by})`, marks.rs:90-92).
  Each is one line on stdout; the breaks below are for width only, so compare the text, not the breaks:
  - measured: `PM-0001-47   MOVED     moved to Slice 1 change-set 7 (Lag): the clock estimate is the real client's; its
    L1 pass here (a4cac39) stays as a record (NPT-TRIM, 2026-11-21 (the re-plan, approved 2026-11-22)); 3 run(s) here
    list it: counted, not judged`
  - bites: the same with `2 run(s)`.
  - The run counts come from the logs' headers: run = the folder's name, marks ["PM-0001-47"]. measured/ holds 3 runs,
    bites/ holds 2.
- **Exit code:** 0 on both (main.rs: 0 whenever the verdicts are written, whatever they say).

## 5. Findings (CLAUDE.md's scale)
| Id | Severity | What | Fix |
|---|---|---|---|
| ADV-NB2-1 | Medium | U7's 0b check is due "the hour U6c merges" and uses the lane and the main folder; the plan files it as window-bound, so NB2 could take that hour (his "R1's path goes first"). | NB2-C1 |
| ADV-NB2-2 | Medium | The 45-minute gap assumes a warm step 4; a part-clean at 1f (likely after U6c's M5) makes it cold. | NB2-C2 |
| ADV-NB2-3 | Medium | Step 2 "may ask … the permission mode decides": the guard asks every time, and a pending ask holds the session and the lane until he answers. | NB2-C3 |
| ADV-NB2-4 | Medium | The stop rules: "nothing is pushed" contradicts Q2 (ii); (ii) has a Builder 2 hazard and a redo trap; step 5's "stop" was not kept apart from a revert. | NB2-C4 |
| ADV-NB2-5 | Low | The part-clean pointer (u4r-disk-plan.md §5) names a folder with no dev profile left. | NB2-C5 |
| ADV-NB2-6 | Low | Step 5's bar records the exit codes but does not require exit 0, and does not give the exact lines. | NB2-C6 |
| ADV-NB2-7 | Low | Step 3 uses a short kit pathspec; no tree anchor; redact's item count is not bound. | NB2-C7 |
| ADV-NB2-8 | Low | No ruling for a moved tip. | NB2-C8 |
| ADV-NB2-9 | Low | Risks miss the SERIES.md and marks.json cross-reads in other crates' tests. | NB2-C9 |
| ADV-NB2-10 | Medium | 72651a3c's last 67 judge lines were read by the advisor agent and checked by his main Advisor, not by a reviewer agent; his 2026-11-15 rule asks for a reviewer's read, and his 24 November item 4 covers 67892d39 only. | NB2-C10: a reviewer read in step 9's window; the report line |
| ADV-NB2-11 | Low | -57's half of the re-plan's real-data line: never run nor dropped since plan-a.md:73; not runnable as worded. | NB2-C11; follow-up |
| ADV-NB2-12 | Low | Text: 33 rows; "self at both"; Builder 2's "before the merge"; the `--abort` reason; PR #40's close and the other slug. | NB2-C12 |
| ADV-NB2-13 | Low (kit) | CLAUDE.md's "Still allowed" list names no merge of another lane's branch that changes no kit file. | Kit follow-up after the freeze, with ADV-M54-8 |

## 6. What I did not check
- I ran no cargo, test, gate, light run or merge (the brief is read only). The trial merge is on today's head bae6232.
  The post-U6c head does not exist yet: 1e and NB2-C7 (a) cover it.
- How the two sides meet in other crates' tests is read from the code, not built (step 4's job).
- The SERIES.md name overlap was read by eye from both -U0 diffs, not by the plan's `comm` pipe.
- M4b's answer 2 (its timing-test and disk fallbacks) is running beside me; not read.
- Whether Claude Code's auto mode could answer a hook's ask without him: not settled. NB2-C3 makes the answer his
  either way.
- GitHub's checks on 72651a3c: taken from Builder 2's card (step 28, run ids 45508565066 and 45508565097) and his main
  Advisor's check, not read through GitHub.
- U6c's M5 numbers (the base of the +47 bars) do not exist yet.

## 7. Notes (one line per check, kept as written)
- His words: history.md:7909 read; heading line 7902 "Mustafa, 2026-11-25 00:07 UTC, in this chat … (word for word)". The
  plan's four quoted lines match item 4. His 2026-11-15 rule: history.md:2127 "every change to marks.json or to the judge
  gets a reviewer's read before its unit closes, and your report to me names it in its own line with what covered it."
- T object present (`git cat-file -t` commit). 72651a3c is an ancestor (exit 0). `rev-list --count` 5. The five, linear,
  one parent each: f54422b, ff6c84d, 9325873 (each M .checkpoints/netcode-judge.md only), 71b32bf (M netcode-judge.md, A
  netcode-judge/mustafa-2026-11-25.md, M netcode-judge/save-message.mjs), 0e621da (M netcode-judge.md). `diff --raw`
  72651a3c..T: 3 files, all 100644; `--stat` 3 files +96/-60. HIS CONDITION HOLDS at T.
- Merge base HEAD (bae6232) and T: e2c0e896dedd5c09c2536e2fbe9323353756fe99. origin's netcode branch = bae6232 (= HEAD).
- e2c0e896..T: `--name-status --no-renames` 46 paths: 22 notes (all A), 24 spike (fixtures/README.md M, 10 fixture logs
  A, src 7 M, tests 3 M, marks README/SERIES/marks.json M) = 32 A, 14 M, 0 D/R. `--shortstat` 46, +6,951/-208; spike part
  24, +4,712/-208. `--raw`: every path 100644 (no symlink, submodule or executable bit).
- Guard pathspec (main-61-plan.md:37-40) over e2c0e896..T: empty. Extra pathspec (tools, **/Cargo.toml, Cargo.lock,
  package*.json, rust-toolchain*, .cargo/**, build.rs): empty. `git check-ignore --no-index` on 36 of the paths (all 22
  notes, the 11 fixtures/README, eval.rs, marks.json, SERIES.md): exit 1, none ignored.
- This branch e2c0e896..HEAD under crates/judge, marks, .checkpoints/netcode-judge*: empty (74 commits). Working tree
  (U6c, uncommitted) in the same places: SERIES.md +51 only. Full `git status --untracked-files=all`: 12 M + 12 new spike
  files = U6c's 24, plus the card; none under crates/judge, marks.json, marks/README.md, netcode-judge*. Only shared path:
  SERIES.md.
- SERIES.md: Builder 2's hunks at base 262-264, 267-272, 278-279, 282-285, 287-288, 290, 293-294, 297, 402, 407, 418-419
  (+421); U6c's one insert after base 908 (new 909-959, 51 lines; 31 data rows + header + separator = the plan's "33").
  Both on base blob 665ab3f. ~490 lines apart: no overlapping or adjacent hunk. Builder 2's 25 row names (21 metric rows,
  the "(19)"->"(21)" heading, relevant_count, admit/session_end, iface, room_end) share none with U6c's probe_*,
  header.params.*, steal_cpus, udp_*, snmp*, cpu_* names (read by eye from both -U0 diffs; I ran no comm pipe).
- Trial merge `git merge-tree --write-tree --name-only HEAD T` at bae6232: one line, tree 8012ac0db8cd…, exit 0, no
  conflict. (Plan's cbb0d520 was at 5e6711e; three notes-only commits since: a different tree id is expected.)
- 5e6711e..bae6232: the card, nb2-merge-plan.md, nb2-merge-request.txt only (launch note holds).
- marks.json: working tree sha256 c1935a5b… (= e2c0e896's blob f9aab0d); T's blob bd93d3d = 72651a3c's; sha256 of
  `git show T:…marks.json` c3396db2…f1b7a. Base marks.rs has no `moved` and `deny_unknown_fields` on every raw struct
  (:14,:22,:56,…): the old judge refuses the new file. All 81 drafts are in docs/adr/0001/0004/0006; this branch changed
  no docs/adr file, check.mjs, Cargo.toml or Cargo.lock since e2c0e896 (docs/ changes are DECISIONS, REPO_MAP, ROADMAP,
  STATUS, STATUS_ARCHIVE, reports: MAIN-65's).
- U7/U8 patches: u8-build.patch 43 headers, u7-t-build-t5i.patch 23; a grep of every wip/*.patch header for marks/,
  crates/judge, netcode-judge finds only older SERIES.md hunks (u4f2, u6b, u6c-build, u6c-fix); none in u7/u8. Holds.
- Builder 2's card at T read whole: counts 807/904 (base) -> 854/951 (72651a3c), self 78/78, redact 11/11 0 findings,
  132 judge tests; hand-off block says "Before the merge: the re-judge of the -47 set" (his item 4 says after: his word
  governs). plan-a.md:73 and ruling (5) narrowed the re-plan's real-data line to -47 only; "-57's folder still PASS"
  (replan-marks.md:207) was never run nor deferred by name.
- eval.rs at T: judge_mark :1737-1746 returns MOVED for a mark-level moved before parts, checks, bites; the headline
  order :1838-1849 applies only past it. main.rs: exit 0 whenever verdicts are written; 1 on malformed marks or logs.
  lib.rs: read_logs unchanged by Builder 2; summary_line `{:<12} {:<9} {first}` with control chars escaped.
  marks.rs:90-92 `moved to {to}: {why} ({by})`. marks.json:9243-9254 PM-0001-47 moved {to "Slice 1 change-set 7 (Lag)",
  why "the clock estimate is the real client's; its L1 pass here (a4cac39) stays as a record", by "NPT-TRIM, 2026-11-21
  (the re-plan, approved 2026-11-22)"}.
- crates/log b57221e..HEAD: +111/-0 in src/lib.rs only (Machine::read_local, cpu_list, a test): no record field
  changed, so the reader parses the b57221e logs as before. Neither U6c nor Builder 2 touches crates/log.
- -47 folder: measured/l1-47-s1..s3 and bites/bite-newest-sample, bite-tick-quantised, each client+lab+room .jsonl;
  du -sm 51. Headers: run = folder name, marks ["PM-0001-47"], role measured / bite, build b57221e release. So serving
  = 3 runs (measured), 2 (bites). Old verdicts: NOT-RUN both (by design of the split). runs.txt:4-12 two calls, peaks
  372 / 349 MiB projected.
- refs step (check.mjs:2625-2631) counts `git ls-files` (all tracked): N0 + 32 right. referenceProblems :1855 skips
  `.checkpoints/` wholesale: Builder 2's notes cannot fail refs. redact walks the spike folder on disk (redact-check.mjs
  :277-288, :378-385): items rise by the 10 fixtures.
- Cross-reads: SERIES.md is read by lab/contract.rs and tests (U6c's bots/tests/load.rs:287, server/src/probe_tests.rs
  :286, net/src/l2/socket_tests.rs:394: containment checks); marks.json by room/tests/bites.rs:31,
  server/tests/schedule.rs:355, check.test.mjs:1776. Builder 2 removes no SERIES.md row name (every - row returns with
  its first cell); neither side adds a name the other adds. Step 4 covers the rest.
- Guard (HEAD = main's kit, `git diff --stat HEAD origin/main -- .claude CLAUDE.md` empty): mainFolderMove
  routine-guard.mjs:1342-1346 — any merge but `--ff-only origin/main` or a one-word `origin/main` -> "only origin/main
  merges into the main folder" -> ask (:1360-1362; a subagent: deny). Step 2 WILL ask. settings.json allow: `git
  merge-base *` only; deny `git reset --hard*`, `--merge*` etc.; the guard has no `revert` rule.
- His rule 2026-11-16 21:50 (history.md:2187): after a 17:09-20:36 stall on "a call that waited for my approval": simple
  forms; "When a call has to ask me, name it in your message before you send it."
- checkpoint.mjs:196 "nothing to save" returns 0 (no push) when the card is unchanged; :108-113 onlyCheckpointsAhead
  is tree-to-tree vs the remote: the merge makes it code-ahead, no [skip ci]. Claim holds.
- Disk: `df -m` /home/user 10,246 MiB now. `.gate/netcode-proof-test/target` has no debug folder (u4r-disk-plan.md §5's
  clean is spent). The live part-clean: card :193-195 (flock on shared-target's lane lock, cargo clean --profile dev
  --target-dir shared-target, then tests and df) = advisor-u6b-m6.md:162-163. It runs outside main-run.sh, so it works
  under main-run.sh's 4 GiB floor (main-run.sh:38-47 exits 3 before any command).
- U7 0b: advisor-u7-brief.md:219-220 "Block 0b's binding check runs the hour U6c merges"; its items run cargo check,
  deny, audit, check.mjs counts (lane + main folder). advisor-main-65.md:159-160: "a list of commands, not an agent".
- -57: kept raw sets u4r-P2-8ee453c22882 741 MB, u4r-P2-fc411116c1f1 623, u4r-P3-d66e23cd78f2 298,
  u4r-P3-fd4612050cd6 291; P0's not kept. No folder named by the re-plan line.
- Coverage of the last changes: test-engineer-b.md's re-check reads head 67892d39 (7c3855c7 + marks.json's 5 lines);
  72651a3c = 67892d3 + 67 lines (figures.rs 20, engine.rs +48, README 3, SERIES 4; no marks.json), read by the advisor
  agent (card step 28) and his main Advisor. mustafa-2026-11-24.md item 4: "Put 1 and 3 in one small commit and have
  your advisor agent read it. No new review round for it."
- -57 at T: no `moved` on the mark, its parts, checks or bites (marks.json:4074-4170). Kept sets' runs.txt: mark NOT-RUN
  ("L1 P0: 0 of 3 valid runs") in every set; part L1 P2 / L1 P3 PASS in its own set.

## Decisions needed
- **NB2-PROMPT: will you answer the merge's permission prompt yourself?**
  - The question: the merge command into the netcode branch always stops and asks you. The kit's guard asks for every
    merge into the main folder except main's own. The netcode chat names the command first, then waits for your
    reply before sending it.
  - Why it matters: while that prompt waits, the netcode chat and R1's path are frozen.
  - The options:
    - (a) Yes: reply when you are at your screen, then approve the prompt.
    - (b) No: the merge does not run in the main folder. A worktree route also ends in a prompt and costs a cold build
      of about 8 GB.
  - My recommendation: (a).

## For Mustafa
1. When the netcode chat names the merge command (`git merge --no-ff … 0e621dae…`) and says it will ask you, reply in
   the chat when you are at your screen. Then approve the permission prompt that appears, or say no: then nothing is
   merged.
2. Nothing to check in Brave: nothing a player sees, hears or feels changes. If the -47 re-judge reads anything but
   MOVED, you get three lines.

DONE
