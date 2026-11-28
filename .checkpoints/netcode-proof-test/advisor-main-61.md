# Advisor: MAIN-61 plan review (take main f5030a3 into PR #40's branch)
2026-11-19, 15:20-15:35 UTC. Read only. Acting by main's advisor.md (`git show origin/main:.claude/agents/advisor.md`,
335 lines, read whole). Reviewed: `.checkpoints/netcode-proof-test/main-61-plan.md` (117 lines, sha256
6c2defd9c8bdb8f9…, equal to its copy in b846828), against MAIN-60's approved form (main-60-plan.md "As approved"
:3-12), my conditions C1-C6 for it (advisor-main-60.md:14-70) and my head check (advisor-main-60-head.md).

## Verdict: APPROVE WITH CONDITIONS C1-C6
The take-in is due under his MAIN-52 rule (history.md:2055-2057, "Take main in when main changes code, CI or rules";
the reading :2082-2086; kept by TEST-FIRST at :2270) and is the precondition my U4f ruling set (advisor-u4f-r1.md:327-328,
M2). Main moved by one merge, #61 (f5030a3's first parent is b763ba3): code in crates/ and packages/, the root manifests
and lockfiles, .gitignore, docs. Every fact the plan states holds when I re-run it (F1-F12 below). Nothing in it
touches a pass mark, the judge, marks.json, money or main: all 81 marks' rows hash to marks.json's values on HEAD,
origin/main and merge-tree's tree (F9). The plan's conflict resolution is right (F6). But three things MAIN-60's form
had, or that the running builder now needs, are missing: C4's three whole-tree checks and the base run's bar (C1); a
notes allowance that covers the card itself, and an index check before the commit that cannot be amended (C2); and a
way to run the light runs while builder 4 holds the lane lock (C3). The conditions below are exact. Applied word for
word, they need no new plan review. Step 7 (my check of the pushed head) stays as planned.

## Conditions (each can be met by number)
- **C1 (ADV-M61-1, Medium): MAIN-60's C4 kept whole.**
  (a) Step 1's base run meets the same bar as step 5d, except N0 and M0: exit 0; "check: all 4 steps pass"; the
  header's HEAD is H0, and its diff and untracked hashes are e3b0c442…; the marks line reads "81 marks read and
  checked …; rows in this branch's docs/adr/: all unchanged"; self 78 of 78; redact 10 of 10 and 0 findings. A FAIL
  on the base run stops before step 2: restart the saver, and the advisor rules.
  (b) Step 5d's marks bar is that same marks line, not "marks 81" alone.
  (c) Step 4 adds these checks after the plan's own, each in its own call:
  - `git diff --quiet origin/main HEAD -- . ':(exclude)spikes/' ':(exclude).checkpoints/' ':(exclude)docs/'` exits 0
    (code, manifests, .gitignore, kit, CI, legacy and tools all equal main's; the plan's two path lists miss
    .gitignore, legacy/ and every other root file);
  - `git diff --name-only origin/main HEAD -- docs/` prints exactly six names: docs/DECISIONS.md, docs/REPO_MAP.md,
    docs/STATUS.md, docs/STATUS_ARCHIVE.md, docs/design/netcode-proof-messages.md, docs/design/ws-reconnect.md;
  - `git diff --name-only H0 HEAD` prints exactly the 78 names that `git diff --name-only b763ba3 origin/main` prints,
    and `git diff --shortstat H0 HEAD` prints "78 files changed, 5621 insertions(+), 106 deletions(-)", as
    `git diff --shortstat b763ba3 origin/main` does.

  I ran all three on merge-tree's tree (695d0a9; for the third, with STATUS.md resolved as step 3 says): exit 0, the
  six names, the same 78 names, and 77 files +5,610/-95 outside STATUS.md plus its 11/11 (F5, F7).
- **C2 (ADV-M61-2, Medium): the notes allowance, and the index before the commit.**
  (a) The card, `.checkpoints/netcode-proof-test.md`, is not under `.checkpoints/netcode-proof-test/`, yet steps 2 to 6
  update it by Edit. Wherever the plan allows notes (step 1's "nothing outside .checkpoints/netcode-proof-test/", step
  2's "beyond unstaged .checkpoints/netcode-proof-test/ lines", the Facts' last bullet), read it as: `git status --short`
  lines whose first column is a space (` M`, ` D`) or that read `??`, for a path under `.checkpoints/netcode-proof-test/`
  or for the card itself. Such a line with a letter in the first column, or any line for another path beyond main's
  78, stops.
  (b) Between step 3's `git add docs/STATUS.md` and step 4's commit, each alone:
  `git diff --name-only --diff-filter=U` prints nothing; `git diff --cached --shortstat H0` prints "78 files changed,
  5621 insertions(+), 106 deletions(-)"; `git diff --cached --name-only H0 -- .checkpoints/ spikes/` prints nothing.
  Any difference stops before the commit; the merge stays in progress, as the stop rules say.
  (c) From step 1's TaskStop to the push, nothing is staged but docs/STATUS.md, and nothing is committed or pushed by
  hand. That covers three things that arrive between turns: the claude.ai web session's Stop-hook request to commit and
  push uncommitted files (ignore it; step 6's card save takes the notes); the 2-hour check-in
  (trig_01EKqNqwZrQTVJ6caAyRrug5; next about 16:34 UTC; the card by Edit only, no save); and a handback from builder 4
  (read it; no save, snapshot, `git apply` or check run on it until the push).
  (d) If a git command fails because `.git/index.lock` exists (builder 4 or another git call in the main folder), git
  changed nothing: when `pgrep -x git` prints nothing, run it once more. A second failure stops.
- **C3 (ADV-M61-3, Medium): the light runs beside builder 4.** main-run.sh waits with no limit for the lane lock
  (`flock -x 9`, :28), which builder 4's runner holds shared or exclusive for each of its runs (u4f-run.sh:53), then up
  to 60 minutes for every cargo and rustc to end (:30-37). A foreground call ends at 10 minutes.
  (a) Run both light runs (step 1's and step 5d's) with run_in_background and timeout 7200000. Wait for the
  completion notice, then read the whole log.
  (b) Exit 3 (under 4 GB free), 4 ("STOP: still busy after 60 min") or 5 (the lock or CACHEDIR.TAG) means no check
  ran. It is not a check FAIL. At step 1: restart the saver (run_in_background, both trailers), and start step 1 again
  later from its first line. At step 5d: the merge stays committed and unpushed; record it on the card by Edit, name it
  in the chat, and run 5d once more. A second such exit, or a run that ends with no "--- end … exit" line, stops, and
  the advisor rules.
  (c) Any other non-zero exit, or a bar line that differs, is a FAIL under the stop rules.
- **C4 (ADV-M61-4, Low): fetch and read main's CI before the merge, not after it.** In step 1, before the pre-checks:
  `git fetch origin` in its own call. Then origin/main must be f5030a33185b…; if main has moved, stop before step 2
  (restart the saver), and the advisor rules. Add to the pre-checks: CI on f5030a3 is green, six check runs (gate,
  smoke, toolchain, two of each), all completed and successful. I read them at 15:2x UTC: all six success, push runs
  on main gate 44551896122 and toolchain 44551894194, their check runs 15:11:02-15:13:16 UTC (F11). Step 6 then reads no CI before the
  push. As written, step 6 reads main's CI after the merge is committed, the state the guard gives no way back from.
- **C5 (ADV-M61-5, Low): MAIN-60's C2 in this take-in's words.**
  (a) The card's resume line, by Edit, before TaskStop, word for word: "MAIN-61 approved (advisor-main-61.md), running
  steps 1-7. If `git rev-parse -q --verify MERGE_HEAD` prints f5030a33185b1ea1b15329174e82823f0215f993, the merge is in
  progress: no save, no `git merge --abort`, no reset. If `git diff --name-only --diff-filter=U` names docs/STATUS.md,
  resume at step 3; otherwise resume at step 3's checks before the commit (C2 (b)), then step 4. If HEAD^2 is f5030a3
  and HEAD is not on the remote yet, resume at step 5. Builder 4's notes and this card may show unstaged: leave them;
  step 6's card save takes them. Anything else: the advisor rules."
  (b) After TaskStop: the saver's output ends with "watch stopped: terminated", or the reflog's last save is the
  "(terminated)" autosave; and `pgrep -af "checkpoint.mjs watch"` shows no saver process. The card names which proof
  was used (ADV-M60H-4).
  (c) The foreground save runs at most once, and only when HEAD and the remote branch differ, or the resume line is not
  yet in a pushed commit. Builder 4's notes written after the saver's last save are not "left unsaved": C2 (a) allows
  them. Then the pre-checks from the beginning. H0 is named in the chat (MAIN-60's C1).
- **C6 (ADV-M61-6, Low): fixes to the plan's text, with an "As approved" header as at MAIN-60.**
  - The Facts' HEAD line: 424f2a2 when the plan was written, b846828 (the plan's own commit) and 36b0983 at this
    review; every commit since b25b96a touches .checkpoints/ only. Give the times.
  - Add the marks fact (MAIN-60's F15): main adds 6 lines to ADR 0002, which holds no mark, and changes no mark's row;
    the 81 rows (63 in ADR 0001, 1 in 0004, 17 in 0006) hash to marks.json's values on HEAD, origin/main and
    merge-tree's tree.
  - The conflict fact: main "changed ten lines in five places", not "five lines elsewhere" (11 removed and 11 added,
    the #60-to-#61 row among them).
  - Step 3: replace "Main's side of every hunk is taken, except the conflict block" with "Only the conflict block
    (merged lines 28-34, from `<<<<<<< HEAD` to `>>>>>>> origin/main`) is edited; every other line stays as git merged
    it."
  - Step 5b's expected line: "repo map OK: 170 entries cover 110 required paths" (F8).
  - The Risks: the lock wait has no limit; the 60 minutes is the idle wait after it (C3). The saver stays off from step
    1 to the push, so builder 4's notes are on disk only for that time, as its code in its worktree already is.
  - After the push: if gate, smoke or toolchain is red on the pushed head, the lane takes no further step on that head
    (no U4f merge) until the advisor rules.
  - The record: under "Choices taken without him", "main f5030a3 taken in now, while builder 4 runs, plan first
    (MAIN-61)", named in that day's message, as MAIN-60 was. The card save's message cites MAIN-52 (history.md:2055-2057)
    and says why the merge commit keeps git's own message without attribution lines (ADV-M60H-5).

## Findings (CLAUDE.md's scale)
| Id | Severity | What | Fix |
|---|---|---|---|
| ADV-M61-1 | Medium | The plan says it builds in MAIN-60's C1-C6, but C4's three whole-tree checks are gone; step 4's path lists miss .gitignore (which main changes), legacy/ and the other root files. The base run's bar is cut to "exit 0", and step 5d's marks bar to "marks 81". | C1 |
| ADV-M61-2 | Medium | The notes allowance names `.checkpoints/netcode-proof-test/` only, while the session updates the card, `.checkpoints/netcode-proof-test.md`, by Edit from step 2 on. Read literally, step 2 stops the take-in on the session's own edit, with the merge in progress. And nothing checks the index before a commit that can never be amended or reset. | C2 |
| ADV-M61-3 | Medium | The light runs wait with no limit for builder 4's lock, then up to 60 minutes for cargo to go idle, while a foreground call ends at 10 minutes. The plan does not say how to run them, or what a run that never started (exit 3, 4, 5) means. At 5d that would stop the take-in with the merge committed and unpushed. | C3 |
| ADV-M61-4 | Low | Main's CI is read at step 6, after the merge is committed; no fetch before the pre-checks. CI is green on f5030a3 (F11), so today this costs nothing. | C4 |
| ADV-M61-5 | Low | MAIN-60's C2 is cited but not written out: the resume line's text (now f5030a3, plus the builder's notes), the proof that the saver ended, and a bound on the foreground save while builder 4 keeps writing notes. | C5 |
| ADV-M61-6 | Low | Stale or loose text: the HEAD line; "five lines elsewhere"; "at most 60 minutes"; step 3's "main's side of every hunk"; the marks fact is missing; no record line, no red-CI line. | C6 |
| ADV-M60-7 | Low (open) | STATUS's #40 row still says "main 53272a1 merged in" (this branch's row, kept word for word, which is right for a merge). | PR #40's closing docs pass, as ruled |
| ADV-M54-8 | Low (open) | CLAUDE.md's Never line still lists no plain `git merge origin/main`, which the guard allows. | The kit follow-up after the freeze, as ruled |

## What I checked (read-only git, file reads and GitHub reads; git merge-tree wrote objects only)
- F1 `git fetch -q origin`, then `git ls-remote`: main f5030a33185b1ea1b15329174e82823f0215f993; the branch
  36b09836ec19…, equal to HEAD. Merge base b763ba3e5f34…. f5030a3's parents: b763ba3 and 4cf30ce (one merge, #61).
  HEAD moved during this review: 424f2a2, then b846828 (15:15:40, the plan) and 36b0983 (15:16:20, the card trim).
  `git diff --name-only b25b96a HEAD` outside .checkpoints/: nothing.
- F2 `git diff --name-status b763ba3 origin/main`: 78 files, 53 A, 21 M, 3 D (packages/client/src/index.ts, its test,
  packages/sim-worker/src/index.test.ts), 1 R100 (packages/e2e/tests/stub.spec.ts to tests/node/); +5,621/-106. The 21
  M include docs/STATUS.md, so step 2's "M 20" is right. None of the 53 added paths exists on disk now, so git has no
  untracked file to refuse over.
- F3 `git diff --stat b763ba3 origin/main -- .claude/ CLAUDE.md AGENTS.md .github/ tools/ spikes/ docs/design/`: empty.
  The guard's full pathspec on `origin/main...HEAD` prints nothing; on `HEAD...origin/main`, nothing too (main changed
  no kit file since b763ba3, and no nested CLAUDE.md, AGENTS.md, .npmrc or .mcp.json arrived). `.claude/hooks/` equals
  main's.
- F4 The guard (routine-guard.mjs:1308-1363, mainFolderMove): for a day session, `git merge origin/main` alone passes
  when `protectedHit(origin/main...HEAD)` is null. That compares commits only. The working tree is not read, so an
  unstaged notes file changes nothing in the guard's answer. Git itself merges over unstaged changes to paths the merge
  does not touch, and main has no .checkpoints/ file (`git ls-tree -r origin/main -- .checkpoints/`: 0). `git add` has
  role-only checks (:1133-1138). git config: commit.gpgsign true, no merge, rerere, status or diff settings, no hook
  outside the samples.
- F5 `git merge-tree --write-tree --name-only HEAD origin/main`: tree 695d0a9, one conflict, docs/STATUS.md (content);
  DECISIONS, REPO_MAP and STATUS_ARCHIVE auto-merge. The patch-id of the merged file against main equals the branch's
  own patch-id against the base for DECISIONS, STATUS_ARCHIVE and both design docs. REPO_MAP has the same +/- lines
  with moved context, each in its table. So no word of Mustafa's in DECISIONS.md is changed by the merge.
- F6 The conflict block is at merged lines 28-34. HEAD's side: #40 (the 2026-11-15 text, "main 53272a1 merged in (its
  checkpoint, 2026-11-15)") and #60. Main's side: #40 (byte-equal to the base's, 2026-11-14) and #61. The plan's
  resolution (line 29, then line 33) diffs against origin/main's STATUS.md in one line only, the #40 row. Line 29
  equals H0's #40 row and line 33 main's #61 row, byte for byte. Main has no #60 row. H0 to the resolved file changes
  exactly main's 11/11 lines. The file has 120 lines. No other line in docs/, crates/ or packages/ starts like a
  conflict marker, so step 3's grep can hit only real markers.
- F7 The three whole-tree checks (C1 (c)) on 695d0a9: exit 0; the six docs; `git diff --name-only HEAD 695d0a9`
  equals b763ba3..origin/main's 78 names. Outside STATUS.md, both numstats are 77 files +5,610/-95.
- F8 Tracked files: 1,514 at HEAD and 1,564 in 695d0a9 (+50 = 53 - 3). check.mjs's refs counts `git ls-files` (the
  index, :2626), so N0 + 50 holds while nothing else is staged. repo-map-check's rule run on 695d0a9's tree: none
  missing, none stale, 170 entries, 110 required paths.
- F9 Marks: an inline node script hashed each of the 81 marks' rows via `git show` (check.mjs:1880-1909's rule) on HEAD,
  origin/main and 695d0a9: 0 differ. Main changes docs/adr/0002 (+6 lines; no mark there). marks/ and crates/judge:
  unchanged by main.
- F10 DEP-HOLD: `git diff --stat b763ba3 HEAD -- Cargo.toml Cargo.lock package.json package-lock.json crates packages
  .gitignore` is empty, so main's manifests and lockfiles arrive equal to main's, and checkpoint.mjs:115-141 holds none
  of them, as long as refs/remotes/origin/main is still f5030a3 at the save. refs: outside spikes/, docs/, .checkpoints/
  and *.md, only .claude/scripts/kept-logs.mjs and its test name `spikes/`, on main as at b763ba3. checkpoint.test.mjs
  on main: 33 test() calls; .claude/scripts/ unchanged since b763ba3.
- F11 CI on f5030a3 (gh api): six check runs, all completed and successful (toolchain, gate and smoke, two of each,
  15:11:02-15:13:16 UTC). On main the push runs are gate 44551896122 and toolchain 44551894194, and the PR branch has
  its own push runs. PR #40: head 36b0983, draft, open, mergeable false, mergeable_state "dirty".
- F12 What main's code does to CI here. .github/ is unchanged since b763ba3, so toolchain.yml is the same file as at
  MAIN-60. It now also builds and tests what #61 added: `cargo test --workspace` on the root workspace (members
  `crates/*`, sim-wasm with wasm-bindgen =0.2.130), the wasm32 build, `npm ci` and `npm test --workspaces` (workspaces
  `packages/*`, vite 8.3.7 now in the root devDependencies), and strict tsc on packages/types. The spike is outside both
  workspaces: it has its own Cargo workspace and no npm manifest. Main adds no .cargo/ config, .npmrc or toolchain file
  that would reach it. So this branch's toolchain job should run what main's did, which was green in 45 to 52 s. Smoke
  and gate read legacy/ and tools/, which equal main's.
- F13 Running now: the saver (PID 12066, from 14:57:29 UTC, both `--trailer` lines); no cargo or rustc at 15:2x; free
  9,722 MiB. Builder 4's worktree is `.claude/worktrees/agent-a5b1297a320917dfb` (u4f-fix1 at b25b96a, locked).
  main-run.sh:27-37 and u4f-run.sh:51-62 share `.gate/shared-target/.lane.lock`. M2 already holds on 695d0a9:
  `git diff --quiet 07289a4 695d0a9 -- spikes/netcode-proof docs/design` exits 0. Since 0f12d9e, nothing outside
  .checkpoints/ has changed on the branch. spikes/netcode-proof tracks 307 files.
- F14 The citations check out: MAIN-52 his words at history.md:2055-2057, the reading at :2082-2086; TEST-FIRST at
  :2234-2282, its "Taking main in when main changes code, CI or rules" at :2270; M2 at advisor-u4f-r1.md:327-328 (its
  heading :325); #61's two review rounds (seven reviewers, then six) in main's STATUS row.

Read: the plan; main-60-plan.md; advisor-main-60.md; advisor-main-60-head.md; history.md:2050-2090 and :2230-2284;
advisor-u4f-r1.md:300-345; the card (193 lines); main-run.sh; u4f-run.sh:1-80; check.mjs:1852-1909 and :2625-2648;
routine-guard.mjs:1130-1363; checkpoint.mjs:1-40 and :110-145; repo-map-check.mjs; toolchain.yml; main's root
Cargo.toml, package.json and rust-toolchain.toml.

## What I did not check
- I did not run the merge, the gate, the light runs, checkpoint.test.mjs or repo-map-check in the main folder (the brief
  is read only). Repo-map and the whole-tree checks were run against merge-tree's tree instead.
- I did not check the merge-conflict event of 15:11:38 UTC itself; the PR's state (F11) agrees with it.
- How builder 4's runs are spaced (so how long the lock waits will be), and whether builder 4 runs git in the main
  folder; C3 and C2 (d) cover both.
- Whether the permission flow asks on `git merge origin/main` (the precedents at 3f2fa21 and 0f12d9e ran without a
  prompt); the guard's part is read from the code (F4).
- CI on the pushed head: that is step 7.

## Decisions needed
None. MAIN-52 is his standing rule, and TEST-FIRST keeps it. The one conflict is resolved by the plan's mechanical
rule. No pass mark, judge, marks.json, money, main, stop or run day is touched, and no review beyond mine is due: #61
was reviewed and merged on main; its manifests arrive equal to main's (DEP-HOLD's own exemption); no kit or CI file
moves.

## For Mustafa
Nothing.
