# MAIN-68: take main 2671595 (#70, #73, #74, #76) into the lane: the plan, rev 3
Rev 3 brought up 2026-11-26 18:4x-18:5x UTC: main moved after rev 2 by one docs-only merge, #76 (MAIN-WALL's record,
18:00:25 UTC), so rev 2's "Order" applies ("if main moved, this plan is brought up first, then checked"). Rev 2 (16:5x-17:2x
UTC, on 3b949c1, sha256 20d27df93c27d79bffa29c8e401ebc8c54763ed08430a5def4534638d2e13832) brought rev 1 up by R68-1 to
R68-8 (advisor-main-68.md). Rev 1 (12:5x UTC, on 2483e9a, sha256 e0bb8c75…2eca24) was REVISE: main moved after it was
written. Its check: advisor-main-68b.md.

## What rev 3 changes (everything else is rev 2's text)
- Main's tip 2671595fc823750a83c58f5b5b122128eb500387 in C4, C5 (a), step 4's parents and step 6. Its CI: gate run
  45872417625, toolchain run 45872417457 (job 140090951266, step 7's baseline).
- #76 in "Why", the merge messages and "On record": 8 docs paths (7 M, 1 A), no kit, CI, code, spike, ADR or design path.
- The counts: 47 paths (A 24, M 23; were 45, A 23, M 22): main-wall.md added and TECHNICAL.md modified; the shortstats; main's
  STATUS.md at 120 lines (was 125); refs at 5d = N0 + 24; step 2's staged 45 (A 24, M 21).
- C6 re-derived on the new trial tree. STATUS.md: #76 moved the ci-preview-handover and guard-wrappers rows that rev 2
  kept to main's archive (its STATUS.md:6), and added MAIN-WALL's row. STATUS_ARCHIVE.md: main added a MAIN-WALL bullet
  above the guard-wrappers one. C2 (b)'s 45-file line measured on the trial tree (Facts).
- Builder 2's merge landed first (48efc9c, pushed 23e9e92), as "Order" allowed: H0 is past it.
- New Q5 (the wall bullet's screenshot line).

## Why (R68-2)
MAIN-52 calls for it. Main moved from 07d673b (MAIN-67's) to 2671595 by four merges, and each of the first three alone
calls for a take-in:
- #70 changes code (crates/, packages/);
- #73 changes rules (two kit files: the day guard and its tests);
- #74 changes CI (.github/workflows/toolchain.yml) and code outside spikes/ (packages/e2e/playwright.config.ts);
- #76 changes docs .md files only (MAIN-WALL's record of main's wall and his decisions).

His word of 12:21 UTC (history.md:8068-8088), "#70 ... merges today and needs a full take-in", names #70 only. #73, #74
and #76 come in with main's tip, because step 2's `git merge origin/main`, the only form the guard passes in the main
folder, takes the tip (as #67 came in at MAIN-66). None of the first three fits his LIGHT-TAKEIN limits. The merge
messages, word for word:
- #70 (2483e9a, 12:22:00 UTC): "Merged on Mustafa's go of 2026-11-26 in the builder chat, for the head the advisor agent
  approved, after his Advisor's merge check; expected head 420b16b35225c41f1978b177822f2e24386a2f95; gate, smoke and
  toolchain green on it".
- #73 (4a62873, 14:56:33 UTC): "guard-wrappers (SAME-GAP (a)): nothing in front of git or gh gets through the day guard
  unread. Merged on Mustafa's go of 2026-11-26 15:45 Amsterdam for head 7bff799f34119fd97588bda0edfc25c46d0f6a9e, after
  the advisor agent's APPROVE of that head and his Advisor's merge check. R4-1 (Critical) stays open by his R4-END (1)."
- #74 (3b949c1, 15:24:22 UTC): "ci-preview-handover: the tested build handed to the preview workflow (with its check),
  TOKEN-WORDS, and the replay test in GitHub's Firefox. Merged at exactly e8340ebfc16859c3ae741db7bea2d75ef59fe6ea on
  Mustafa's tap on the decision card in his main Advisor thread (2026-11-26, 15:24 UTC), after the merge check."
- #76 (2671595, 18:00:25 UTC): "main-wall (MAIN-WALL): the record of main's wall, his decisions and the report. Merged on
  Mustafa's go ("Merge #76") for head 8dd8a6ffe630e96dee9ed0200d278ad505d7d38d, after the advisor agent's APPROVE of that
  head; gate and smoke green. The first day-session merge under the wall (T10's day-session half)."

Form: MAIN-66's plan (main-66-plan.md, as approved with M66-1 to M66-5, advisor-main-66.md) as MAIN-67 applied it
(main-67-plan.md with M67-1 to M67-3), steps 1-7 whole. Only what differs is written here, and every MAIN-66 or MAIN-67
value is replaced by its line below. #73 makes it MAIN-66's case (kit: M66-2's naming and refusal branch, step 5e); #74
makes it MAIN-67's case too (CI: M67-3's step-7 comparison). His interim rule holds in every command: git and gh always
start their own command, with no other program in front of them.

## Facts (read 18:4x-18:5x UTC, read only; HEAD then 590246f3dc85f4b89fa3c8204e24f6bb7113d767)
- `git ls-remote origin refs/heads/main` (18:4x) read 2671595; `git fetch origin main` (18:4x): origin/main
  2671595fc823750a83c58f5b5b122128eb500387. First-parent 07d673b..2671595: #70, #73, #74, #76. The merge base with this
  branch: 07d673b.
- `git diff --name-status 07d673b origin/main`: 47 paths, A 24, M 23. `--no-renames --diff-filter=DRCTUXB` prints
  nothing; `--raw` shows every file as 100644 (no link, no submodule; first-loop.rec included; #76's eight read again).
  - Whole: "47 files changed, 7144 insertions(+), 242 deletions(-)". Without docs/STATUS.md: "46 files changed, 7100
    insertions(+), 198 deletions(-)". Without docs/STATUS.md and docs/STATUS_ARCHIVE.md: "45 files changed, 6699
    insertions(+), 196 deletions(-)".
  - Docs numstat: DECISIONS 570/0, REPO_MAP 38/16, ROADMAP 106/52, STATUS 44/44, STATUS_ARCHIVE 401/2, TECHNICAL 9/0,
    UNITS 10/0. Main's STATUS.md has 120 lines (`git grep -c ""`).
  - The 24 added: crates/devtools recordings/ (first-loop.hashes, .rec, .vector.txt, manifest.txt), src/bin/
    (make_recordings.rs, replay.rs), src/ (compare, files, recording, replay, scripts, sha256), tests/ (hash_cost,
    recordings, two_instances, two_processes); docs/reports/ (2026-11-25-cs4a-recording.md,
    2026-11-26-ci-preview-handover.md, 2026-11-26-guard-wrappers.md, 2026-11-26-main-wall.md);
    packages/e2e/tests/replay/determinism.spec.ts; packages/sim-worker boundary/replay.test.mjs, src/recording.test.ts,
    src/recording.ts.
  - #76 alone (3b949c1..2671595): DECISIONS, REPO_MAP, ROADMAP, STATUS, STATUS_ARCHIVE, TECHNICAL and
    reports/2026-11-26-guard-wrappers.md modified, reports/2026-11-26-main-wall.md added; "8 files changed, 282
    insertions(+), 62 deletions(-)". `git diff --name-only 3b949c1 origin/main -- .claude/ .github/ tools/ CLAUDE.md
    CLAUDE.local.md AGENTS.md .gitignore spikes/ docs/adr/ docs/design/ crates/ packages/` prints nothing.
- **Kit and CI (R68-3).** Main changes .claude/hooks/routine-guard.mjs and routine-guard.test.mjs (#73) and
  .github/workflows/toolchain.yml (#74). It changes no other `.claude/` file (settings.json and guard.mjs among them), no
  other `.github/` file, and nothing in tools/, CLAUDE.md, CLAUDE.local.md, AGENTS.md or .gitignore.
  - The guard's pathspec both ways (read again 18:5x): `origin/main...HEAD` prints nothing; `HEAD...origin/main` prints
    exactly those three.
  - The merge rule at the tip (routine-guard.mjs:1391-1441, by `git grep`; #76 changes no `.claude/` file, so it is
    #73's): ":1428 only origin/main merges into the main folder"; ":1430 a plain merge takes main's kit files exactly only
    when the branch changed none since the merge base (R2)". #73's hunks against HEAD's file (`git diff -U0 HEAD
    origin/main`) sit at :301-359 (NONE, WRAP_OPTS, GIT_HELPER, unwrap()), :790-815 (bash(): rule U and xargs) and
    :850-855 (GIT_GLOBAL, NON_RUNNERS). None is in the merge rule. So `git merge origin/main` in its own call passes, and
    after it .claude/ and .github/ equal main's.
  - The guard after the merge, as MAIN-66's guard line: from the first call after step 2 writes main's
    routine-guard.mjs, #73's rules check this session's calls (the merge call itself, the old ones). Rule U: a program
    the guard does not strip, with git or gh among its later words, asks in a session and is refused for an agent. xargs
    asks unless a read-only program follows it. timeout, xvfb-run, command, time and exec read their options from
    WRAP_OPTS. Whether an agent's calls from its worktree resolve to the main folder's copy is untested (ADV-M66-7).
- **#74's change.**
  - toolchain.yml: comment and step-name wording, and two new steps after the cut, both
    `if: github.event_name == 'pull_request' && github.event.pull_request.head.repo.full_name == github.repository`:
    "Check that the tested build holds only plain files and folders" (it prints "<n> files, <bytes> bytes") and "Hand
    the tested build to the preview workflow" (actions/upload-artifact at cf430e0, v7.0.2: artifact `preview-site` from
    .gate/dist-production, kept 7 days).
  - Main has no .github/workflows/preview.yml (`git ls-tree` lists gate.yml and toolchain.yml), so no preview starts.
  - playwright.config.ts: the firefox project now runs tests/rules/ and tests/replay/ (his FIREFOX-REPLAY).
  - On main's push runs both new steps were skipped (3b949c1's and 2671595's), so main gives no baseline for them.
- **Main's CI on 2671595**, push runs, attempt 1, all success: gate run 45872417625 (gate job 140090952092, smoke job
  140090951826; 18:00:31-18:01:18 UTC) and toolchain run 45872417457 (job 140090951266, 18:00:31-18:06:30; its
  build-and-test step 18:01:02-18:06:27; the two new steps skipped). Rev 2's on 3b949c1: gate 45848772772, toolchain
  45848772877 (job 140009301248), all success.
- **The trial merge.** `git merge-tree --write-tree --name-only HEAD origin/main` (HEAD 590246f): exit 1, tree
  cf3bde597e237940d3611b11d215d0a0deb6d5bd. Two CONFLICT lines, docs/STATUS.md and docs/STATUS_ARCHIVE.md (content);
  auto-merging DECISIONS.md, REPO_MAP.md, STATUS.md and STATUS_ARCHIVE.md.
  - STATUS.md's block (markers :34, :37, :40, by `git grep -n` on the tree). Ours: :35 (#40, this branch's row, its
    2026-11-15 text) and :36 (ci-after-3a's "none yet" row, main's 07d673b text). Theirs: :38 (main's #40, its
    2026-11-14 text) and :39 (MAIN-WALL's "none yet" row, #76's). Main moved ci-after-3a's row to its archive, and #76
    moved the ci-preview-handover and guard-wrappers rows there too (main's STATUS.md:6, word for word: "#73 and #74
    merged, their rows, "Waiting on" lines and follow-ups bullets moved to the archive by script"; `git grep` finds
    neither name in a table row). Main's file lists MAIN-WALL as "none yet", though #76 merged: main's text, taken as it
    is (as ADV-M68-4).
  - STATUS_ARCHIVE.md's block (markers :2216, :2226, :2252), right under "## Open bugs and risks" (:2215). Ours:
    :2217-2225, the "netcode proof test" bullet (9 lines, this branch's own: part of its 18/5 since 07d673b). Theirs:
    :2227-2251, main's new "MAIN-WALL" bullet (:2227-2240, 14 lines, #76's), then the "guard-wrappers" bullet
    (:2241-2251, 11 lines, #73's; its first line now carries #76's correction note). At 07d673b the heading was followed
    by the "CI, the smoke job's Chromium step" bullet (now :2253), so both sides added at the same place. Main's tip holds
    no copy of ours.
  - The rest of the trial tree equals main outside spikes/, .checkpoints/ and docs/ (`git diff --stat` prints nothing)
    and HEAD in spikes/ and docs/design/ (nothing). Against main in docs/ it differs in the usual six names. DECISIONS
    24/1, REPO_MAP 17/0, STATUS 1/1 and STATUS_ARCHIVE 18/5: this branch's own (`git diff --numstat 07d673b HEAD`).
  - Measured, not predicted: `git diff --shortstat 590246f cf3bde597e237940d3611b11d215d0a0deb6d5bd -- .
    ':(exclude)docs/STATUS.md' ':(exclude)docs/STATUS_ARCHIVE.md'` prints "45 files changed, 6699 insertions(+), 196
    deletions(-)", and its numstat gives DECISIONS 570/0 and REPO_MAP 38/16, main's own: the two files both sides changed
    auto-merge to main's counts, so C2 (b)'s 45-file line holds.
- refs: `git grep -a -l "spikes/" origin/main -- . ':(exclude)spikes/' ':(exclude)docs/' ':(exclude).checkpoints/'
  ':(exclude)*.md'` names kept-logs.mjs and its test only. Untracked: `git ls-files --others --exclude-standard -- crates
  packages docs .claude .github` prints nothing. Free disk 7,708 MiB at 18:5x UTC (13,733 before NB2's full check run,
  18:05-18:29); the light runs need 4,096 (main-66-plan.md:120-121).
- DEP-HOLD: no Cargo.toml, Cargo.lock, package.json or package-lock.json among the 47. Nothing under spikes/, docs/adr/
  or docs/design/ (no `| PM-` row moves).

## On record (R68-5)
- R4-1 (Critical) is open on main by his R4-END (1). #73's report: "one more spelling hides git, gh or xargs from the
  guard's reading, for a session and a subagent … open on main too"; its fix is the next guard change-set; "His interim
  rule stays until R4-1's fix merges" (also main's STATUS_ARCHIVE guard-wrappers bullet). #76's DECISIONS.md records his
  SPEED-ORDER (b) of 17:37:10 UTC (the review tracks before the guard change-set) and repeats that "his interim rule on
  git and gh stands until R4-1's fix merges".
- ADV-R4-2: 91 swept lines are looser than the guard before #73. Six pass where timeout gets a lone dash and then runs
  nothing; 85 ask in a session instead of being refused; subagents are still refused.
- #74: security-auditor's one read found nothing that stops it; its SA-1 Medium is the accepted CUT-ESCAPE (code that
  ran inside the cut can still change later steps). infra-engineer: APPROVE. (Main's STATUS row for it.)
- #76: main's wall is on since about 17:15 UTC by his click. Main's STATUS row for it, word for word: "risk tier 2;
  GitHub settings by his clicks, no code or workflow changed; measured on a public throwaway repository, deleted by his
  click after 17:27 UTC [assumed: his "done" of 17:31:41 UTC]; the plan: the advisor agent APPROVE, security-auditor
  REVISE (text only, all applied); the results: both APPROVE". Its TECHNICAL.md §8c: the wall ruleset targets "the
  default branch", one rule, "Restrict updates", bypassed only by the "Repository admin" role, beside "Shamble Ruleset 1"
  (pull request, gate and smoke). So it binds main only: this branch took seven pushes after 17:15 UTC (42c0d67 at 17:38
  to 0498241 at 18:52, 23e9e92's merge among them), and the merge here is a local commit on this branch.
- No security read for the take-in. These reasons carry: main's files come in byte for byte; after the merge, PR #40's
  diff against main holds no kit or CI file; #73, #74 and #76 merged on his go and tap with these findings open by his
  own words. Two of MAIN-66's reasons do not: #73's last security round was REVISE, not APPROVE; and #73's rules do not
  only add asks or denies (ADV-R4-2).

## The commands of steps 2-6 under #73's rules (R68-4)
Each in its own call. None has a program in front of git or gh, and no xargs runs at the session level.
- Step 2: `git merge origin/main`.
- Step 3: Edit on docs/STATUS.md and docs/STATUS_ARCHIVE.md; `git grep -n -E "^(<<<<<<<|=======|>>>>>>>)" --
  docs/STATUS.md docs/STATUS_ARCHIVE.md`; `wc -l docs/STATUS.md`; `git add docs/STATUS.md docs/STATUS_ARCHIVE.md`; the
  `git diff --cached` checks below.
- Step 4: `git commit --no-edit`.
- Step 5: `node tools/gate.mjs --no-smoke`; `node .claude/scripts/repo-map-check.mjs`; `node --test
  .claude/scripts/checkpoint.test.mjs`; `bash .checkpoints/netcode-proof-test/main-run.sh <log> node check.mjs --only
  refs,marks,self,redact` (main-run.sh's own xargs at :42 and its git calls run inside the script, which the guard does
  not read: #73's report, Risks 2, INPUT-RUN); `node --test .claude/hooks/routine-guard.test.mjs`.
- Step 6: the card save (`node .claude/scripts/checkpoint.mjs save netcode-proof-test -m "<msg>"` with the two
  trailers; no argument word is exactly git or gh); the saver's restart; the CI reads by the GitHub MCP tools.

If one asks or is refused under #73's rules, the step stops, the merge stays as it is, and the advisor rules. No other
form is tried.

## Steps (MAIN-66's steps 1-7 as MAIN-67 applied them; only what differs)
1. As written: the resume line by Edit (C5 (a), replacing card lines); the saver stopped (TaskStop and C5 (b)'s proof,
   or M64-3's if it ended on its own); H0 named in the chat; `git fetch origin` in its own call, then C4; the anchor T
   (below); the base light run into .gate/main-68/light-base.log (run_in_background, timeout 7200000), giving N0, M0 and
   the self count.
2. Named in the chat first (M66-2): `git merge origin/main`. It writes into .claude/hooks/ and .github/workflows/.
   - Refused before git runs: prove nothing changed (no MERGE_HEAD, HEAD = H0, status shows only notes lines), try no
     other form, wording or worktree, restart the saver, and tell him the one command with options (a) approve it in a
     prompt or (b) wait, my lean (a). A prompt: he answers it.
   - Expected: exit 1, "CONFLICT (content): Merge conflict in docs/STATUS.md" and "CONFLICT (content): Merge conflict in
     docs/STATUS_ARCHIVE.md", no other CONFLICT line. `git status --short` shows `UU docs/STATUS.md`, `UU
     docs/STATUS_ARCHIVE.md`, and main's other 45 paths staged (A 24, M 21).
   - From the next call on, #73's rules check this session's calls.
3. Resolve by Edit, each kept line word for word from its side (C6):
   - STATUS.md: this branch's #40 row, then main's MAIN-WALL row. The ci-after-3a row and main's #40 row go.
   - STATUS_ARCHIVE.md: this branch's netcode proof test bullet (9 lines), then main's MAIN-WALL bullet (14 lines) and
     guard-wrappers bullet (11 lines), as main has them. Only the three marker lines go.
   - Then the marker grep (nothing) and `wc -l docs/STATUS.md`: exactly 120, before `git add docs/STATUS.md
     docs/STATUS_ARCHIVE.md`.
   - After the add: `git diff --cached --numstat origin/main -- docs/STATUS.md` prints "1 1" (the line out is main's
     #40, its 2026-11-14 text; the line in, H0's #40 row).
   - `git diff --cached --numstat origin/main -- docs/STATUS_ARCHIVE.md` is expected at "18 5" (H0's own `git diff
     --numstat 07d673b H0 -- docs/STATUS_ARCHIVE.md`). That is a prediction; the content check governs: `git diff
     --cached -U0 origin/main -- docs/STATUS_ARCHIVE.md` shows only H0's own changes, the bullet's nine lines word for
     word among them.
4. `git commit --no-edit`: two parents, H0 and 2671595, git's own message (the card save carries the attribution lines).
5. a-c as written: GATE PASS (`--no-smoke`); "repo map OK", its two numbers recorded; the checkpoint tests pass, 0 fail,
   the count recorded (33 at MAIN-66; main changes no .claude/scripts/ file).
   - d: the light run (run_in_background): every count equals the base run's word for word, except HEAD, and refs =
     N0 + 24.
   - e: `node --test .claude/hooks/routine-guard.test.mjs` (main's file now), in M66-3's form (in the background,
     timeout 1800000; its summary lines by Write into .gate/main-68/step5-outputs.txt). The bar: 0 fail, the count
     recorded. Expected 1,551 (#73's report, "Evidence": "1247 on main → 1437 → 1490 → 1499 → 1551/1551"; main's STATUS
     row "guard tests 1551/1551"; #76 changes no .claude/ file). Another total with 0 fail, while `git diff --quiet
     origin/main HEAD -- .claude/` exits 0, is recorded and goes on (M65-2); any fail stops.
6. As written: main's CI on 2671595 read again; the card save pushes (`wc -l` on the card first: 200 or fewer); CI on
   the pushed head, push and pull request: gate (gate, smoke) and toolchain green.
   - Toolchain now also runs #70's tests (devtools' recording, replay, two-instance and two-process tests; sim-worker's
     recording and replay boundary tests; the e2e replay spec in Chromium and, by #74, in Firefox).
   - #74's two steps have their own bar, since main skipped them: on PR #40's pull-request toolchain run both succeed,
     the check's printed line ("<n> files, <bytes> bytes") is recorded and the artifact `preview-site` is listed on that
     run; on the push run both are skipped. No preview starts (main has no preview.yml). A red in either stops for the
     advisor.
   - The saver restarts here, both trailers.
7. The advisor's check of the pushed head by SHA (MAIN-65 step 7's form): new agent work, in the next reset's first 30
   minutes unless it rides an agent already running. Its toolchain baseline: main's run 45872417457, job 140090951266,
   compared line for line as M67-3 gives it (the `test result:` lines, the vitest `Test Files` lines, and every
   Playwright "passed" line, Chromium and Firefox); #74's two steps by step 6's bar.

## The numbers (in place of MAIN-66's and MAIN-67's)
- C1 (a), (b), 5d: the base run gives every count (refs N0, the full marks line, self S0 of S0, redact R0 of R0 with M0
  items, 0 findings); 5d equals them word for word, except HEAD, and refs = N0 + 24.
- C1 (c):
  - `git diff --name-only H0 HEAD` equals `git diff --name-only 07d673b origin/main` (47 names).
  - Without docs/STATUS.md and docs/STATUS_ARCHIVE.md, both pairs' shortstat reads "45 files changed, 6699
    insertions(+), 196 deletions(-)".
  - The whole line is expected at "47 files changed, 7144 insertions(+), 242 deletions(-)" (STATUS.md's 44/44 and
    STATUS_ARCHIVE.md's 401/2 as main's: the swapped #40 row and H0's bullet are the same on both sides). If a line
    differs, step 3's content checks govern.
  - docs/ against main: the usual six names (DECISIONS, REPO_MAP, STATUS, STATUS_ARCHIVE, design/netcode-proof-messages,
    design/ws-reconnect).
- C2 (a): "beyond main's 47". The allowance as MAIN-66's: a ` M`, ` D` or `??` line under .checkpoints/netcode-proof-test/
  or the card. Expected: the card, this plan, advisor-main-68b.md, history.md (if he writes), msg-2026-11-27.md,
  u7-merge-plan.md, the window's verdict files.
- C2 (b): after `git add docs/STATUS.md docs/STATUS_ARCHIVE.md`, each alone:
  - `git diff --name-only --diff-filter=U` prints nothing;
  - `git diff --cached --shortstat H0 -- . ':(exclude)docs/STATUS.md' ':(exclude)docs/STATUS_ARCHIVE.md'` prints the
    45-file line (measured on the trial tree: Facts);
  - `git diff --cached --name-only H0 -- .checkpoints/ spikes/` prints nothing;
  - the anchor: `git diff --cached --quiet T -- . ':(exclude).checkpoints/' ':(exclude)docs/STATUS.md'
    ':(exclude)docs/STATUS_ARCHIVE.md'` exits 0 (after the commit, `HEAD` in place of `--cached`).
- The anchor T (M67-2's form): at step 1, after H0 is fixed and after `git fetch origin`, alone: `git merge-tree
  --write-tree --name-only H0 origin/main`. The bar: exit 1, two CONFLICT lines (docs/STATUS.md, docs/STATUS_ARCHIVE.md),
  and the auto-merge lines for DECISIONS.md, REPO_MAP.md, STATUS.md and STATUS_ARCHIVE.md; anything else stops before
  step 2. T goes into this section by Edit before step 2 (cf3bde59 on HEAD 590246f at this revision; not bound).
- C2 (c), C3: no builder runs on the lane. U8-F1's builder, if resumed, works in its own worktree and runs no cargo; its
  hand-back is only read until the push.
- C4: after `git fetch origin` in its own call, origin/main must be 2671595fc823750a83c58f5b5b122128eb500387. Main's CI
  is its own push runs: gate 45872417625 (gate 140090952092, smoke 140090951826) and toolchain 45872417457
  (140090951266), all completed with success. A further move stops before step 2 (advisor-main-65.md:113-114), unless
  the check's answer to Q3 lets it be re-derived.
- C5 (a), the resume line, word for word: "MAIN-68 approved (advisor-main-68b.md), running steps 1-7. If `git rev-parse
  -q --verify MERGE_HEAD` prints 2671595fc823750a83c58f5b5b122128eb500387, the merge is in progress: no save, no `git
  merge --abort`, no reset. If `git diff --name-only --diff-filter=U` names docs/STATUS.md or docs/STATUS_ARCHIVE.md,
  resume at step 3; otherwise resume at step 3's checks before the commit (C2 (b)), then step 4. If HEAD^2 is 2671595
  and HEAD is not on the remote yet, resume at step 5. advisor-main-68b.md and this card may show unstaged: leave them;
  step 6's card save takes them. U8-F1's builder works in its own worktree: nothing of it shows here. Anything else: the
  advisor rules."
- C5 (b): the saver running then is stopped by TaskStop at step 1, with its proof; it restarts at step 6.
- C6:
  - STATUS.md's block is merged lines 34-40 (`<<<<<<< HEAD` :34, `=======` :37, `>>>>>>> origin/main` :40). Keep :35
    (ours #40, the 2026-11-15 text) and :39 (main's MAIN-WALL row), in that order. Drop :36 (ci-after-3a's row) and :38
    (main's #40, the 2026-11-14 text).
  - STATUS_ARCHIVE.md's block is merged lines 2216-2252 (`<<<<<<< HEAD` :2216, `=======` :2226, `>>>>>>> origin/main`
    :2252). Keep :2217-2225 (ours, the netcode proof test bullet), then :2227-2251 (main's MAIN-WALL and guard-wrappers
    bullets). Drop only the three marker lines.
  - The line numbers are re-derived on step 1's tree T (from H0) if they move; the rows and bullets stay as named.
- Step 7's toolchain baseline: run 45872417457, job 140090951266 (M67-3's list), and step 6's bar for #74's two steps.
- M65-2's fallbacks stand with these values: refs at 5d = H0's tree count + 24, with `git diff --name-only
  --diff-filter=A H0 HEAD` listing main's 24 added names; the staged tally `git diff --cached --name-status H0` = `git
  diff --name-status 07d673b origin/main` (47 paths, A 24, M 23); STATUS.md's 120 with no tolerance.

## Order with the rest of the day
- This plan's check (advisor-main-68b.md) is new agent work: in the 19:50 UTC window, by resuming the advisor that
  checked rev 1 (it holds #70's half), reading only #73's, #74's and #76's halves and the new numbers. Just before its
  launch: `git ls-remote origin refs/heads/main`; if main moved, this plan is brought up first, then checked.
- Steps 1-6 after the check, when no other merge runs. Builder 2's merge landed first (48efc9c, 18:02 UTC; pushed
  23e9e92), so H0 is past it, and this plan's anchor and counts are re-derived from H0 at step 1. U7's merge comes after
  this one (U7-M-1); its check.mjs whole run needs the part-clean first (8,192 MiB; 7,708 free at 18:5x).
- Step 7 in the window after step 6, unless it rides an agent then running.
- The record, under "Choices taken without him": "main 2671595 taken in (#70, #73, #74 and #76 in one merge: his word of
  12:21 UTC for #70; #73, #74 and #76 with main's tip, by the only merge form the guard passes), plan first (MAIN-68)",
  named in msg-2026-11-27.md with all four pull requests.

## Questions for the checker
- Q1: STATUS_ARCHIVE.md's block: both sides added under "## Open bugs and risks". Is ours then main's (git's order)
  right, or main's first?
- Q2: #74's two pull-request steps on PR #40 check and upload the toolchain job's player build (.gate/dist-production).
  Does anything under spikes/ run in or feed that job, so that the uploaded build could hold anything but main's code?
  My reading: the spike is outside the root workspace (its own Cargo.toml, no path dependency up) and outside packages/.
  Anything to add?
- Q3: if main moves again before step 1, which values may be re-derived at step 1 by the same commands, with one line
  to him, without a new check (MAIN-66's Q4 form)? My lean: a docs-only move (no code, CI, rule, manifest, spikes/ or
  docs/adr/ file) whose trial merge shows no conflict outside the same two blocks; anything else stops for you. (#76 is
  that case; rev 3 brings it in by hand instead, since the check had not run.)
- Q4: may step 7 ride U7's head read, or Builder 2's NB2-C10 reviewer read, in the window after step 6?
- Q5: main's new MAIN-WALL archive bullet says "A day session asks him for a screenshot of both rulesets at the start of
  any kit, CI or gate change-set (the plan's risk 4)." MAIN-68 brings in main's kit and CI files byte for byte and
  changes none of its own; after it PR #40's diff against main holds no kit or CI file. My reading: it is not such a
  change-set, so no screenshot is asked of him. Right?

## Change type, tier, size
A merge of main's reviewed content (47 files, +7,144 / -242), with two conflict blocks resolved by hand; nothing else is
edited by hand.
- #70: five reviewers in two rounds, the verifier PASS, the advisor agent's APPROVE of its head 420b16b; merged on his go
  after his Advisor's merge check.
- #73: risk tier 2 on main; four security-auditor rounds, the last REVISE, R4-1 left open by his R4-END (1); the advisor
  agent's APPROVE of head 7bff799f; merged on his go.
- #74: risk tier 2 on main; security-auditor (SA-1 Medium, the accepted CUT-ESCAPE) and infra-engineer APPROVE; merged at
  e8340eb on his tap, after the merge check.
- #76: docs only (main's STATUS row: risk tier 2, his clicks on GitHub's settings); the advisor agent's APPROVE of head
  8dd8a6f; merged on his go ("Merge #76"), gate and smoke green.

Tier 1 for PR #40 after the merge (no kit or CI file differs from main's). Reviewers: the advisor (this plan, then the
pushed head). Mustafa: nothing to check in Brave.
