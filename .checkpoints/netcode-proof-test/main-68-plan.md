# MAIN-68: take main 3b949c1 (#70, #73, #74) into the lane: the plan, rev 2
Rev 2 brought up 2026-11-26 16:5x-17:2x UTC (R68-1 to R68-8, advisor-main-68.md). Rev 1 (12:5x UTC, on 2483e9a, sha256
e0bb8c75…2eca24) was REVISE: main moved after it was written. Its check: advisor-main-68b.md.

## Why (R68-2)
MAIN-52 calls for it. Main moved from 07d673b (MAIN-67's) to 3b949c1 by three merges, and each alone calls for a take-in:
- #70 changes code (crates/, packages/);
- #73 changes rules (two kit files: the day guard and its tests);
- #74 changes CI (.github/workflows/toolchain.yml) and code outside spikes/ (packages/e2e/playwright.config.ts).

His word of 12:21 UTC (history.md:8068-8088), "#70 ... merges today and needs a full take-in", names #70 only. #73 and
#74 come in with main's tip, because step 2's `git merge origin/main`, the only form the guard passes in the main folder,
takes the tip (as #67 came in at MAIN-66). None fits his LIGHT-TAKEIN limits. The merge messages, word for word:
- #70 (2483e9a, 12:22:00 UTC): "Merged on Mustafa's go of 2026-11-26 in the builder chat, for the head the advisor agent
  approved, after his Advisor's merge check; expected head 420b16b35225c41f1978b177822f2e24386a2f95; gate, smoke and
  toolchain green on it".
- #73 (4a62873, 14:56:33 UTC): "guard-wrappers (SAME-GAP (a)): nothing in front of git or gh gets through the day guard
  unread. Merged on Mustafa's go of 2026-11-26 15:45 Amsterdam for head 7bff799f34119fd97588bda0edfc25c46d0f6a9e, after
  the advisor agent's APPROVE of that head and his Advisor's merge check. R4-1 (Critical) stays open by his R4-END (1)."
- #74 (3b949c1, 15:24:22 UTC): "ci-preview-handover: the tested build handed to the preview workflow (with its check),
  TOKEN-WORDS, and the replay test in GitHub's Firefox. Merged at exactly e8340ebfc16859c3ae741db7bea2d75ef59fe6ea on
  Mustafa's tap on the decision card in his main Advisor thread (2026-11-26, 15:24 UTC), after the merge check."

Form: MAIN-66's plan (main-66-plan.md, as approved with M66-1 to M66-5, advisor-main-66.md) as MAIN-67 applied it
(main-67-plan.md with M67-1 to M67-3), steps 1-7 whole. Only what differs is written here, and every MAIN-66 or MAIN-67
value is replaced by its line below. #73 makes it MAIN-66's case (kit: M66-2's naming and refusal branch, step 5e); #74
makes it MAIN-67's case too (CI: M67-3's step-7 comparison). His interim rule holds in every command: git and gh always
start their own command, with no other program in front of them.

## Facts (read 16:5x-17:1x UTC, read only; HEAD then 06ed6c568d2423bb604b683d5e29810534f09f42)
- `git fetch origin main` (16:5x): origin/main 3b949c1a9f9c99d6d54570a0f2805c3d65661d8b (C's ls-remote at 15:2x read
  4a62873; #74 merged after). First-parent 07d673b..3b949c1: #70, #73, #74. The merge base with this branch: 07d673b.
- `git diff --name-status 07d673b origin/main`: 45 paths, A 23, M 22. `--no-renames --diff-filter=DRCTUXB` prints
  nothing; `--raw` shows every file as 100644 (no link, no submodule; first-loop.rec included).
  - Whole: "45 files changed, 6900 insertions(+), 218 deletions(-)". Without docs/STATUS.md: "44 files changed, 6852
    insertions(+), 175 deletions(-)". Without docs/STATUS.md and docs/STATUS_ARCHIVE.md: "43 files changed, 6525
    insertions(+), 173 deletions(-)".
  - Docs numstat: DECISIONS 519/0, REPO_MAP 37/16, ROADMAP 74/29, STATUS 48/43, STATUS_ARCHIVE 327/2, UNITS 10/0.
    Main's STATUS.md has 125 lines (`git grep -c ""`).
  - The 23 added: crates/devtools recordings/ (first-loop.hashes, .rec, .vector.txt, manifest.txt), src/bin/
    (make_recordings.rs, replay.rs), src/ (compare, files, recording, replay, scripts, sha256), tests/ (hash_cost,
    recordings, two_instances, two_processes); docs/reports/ (2026-11-25-cs4a-recording.md,
    2026-11-26-ci-preview-handover.md, 2026-11-26-guard-wrappers.md); packages/e2e/tests/replay/determinism.spec.ts;
    packages/sim-worker boundary/replay.test.mjs, src/recording.test.ts, src/recording.ts.
- **Kit and CI (R68-3).** Main changes .claude/hooks/routine-guard.mjs and routine-guard.test.mjs (#73) and
  .github/workflows/toolchain.yml (#74). It changes no other `.claude/` file (settings.json and guard.mjs among them), no
  other `.github/` file, and nothing in tools/, CLAUDE.md, CLAUDE.local.md, AGENTS.md or .gitignore.
  - The guard's pathspec both ways: `origin/main...HEAD` prints nothing; `HEAD...origin/main` prints exactly those three.
  - The merge rule at the tip (routine-guard.mjs:1391-1441, by `git grep`): ":1428 only origin/main merges into the main
    folder"; ":1430 a plain merge takes main's kit files exactly only when the branch changed none since the merge base
    (R2)". #73's hunks against HEAD's file (`git diff -U0 HEAD origin/main`) sit at :301-359 (NONE, WRAP_OPTS,
    GIT_HELPER, unwrap()), :790-815 (bash(): rule U and xargs) and :850-855 (GIT_GLOBAL, NON_RUNNERS). None is in the
    merge rule. So `git merge origin/main` in its own call passes, and after it .claude/ and .github/ equal main's.
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
  - On main's push run both new steps were skipped, so main gives no baseline for them.
- **Main's CI on 3b949c1**, push runs, attempt 1, all success: gate run 45848772772 (gate job 140009303166, smoke job
  140009302600; 15:24:27-15:25:09 UTC) and toolchain run 45848772877 (job 140009301248, 15:24:28-15:29:48; its
  build-and-test step 15:24:56-15:29:46; the two new steps skipped). R68-8: rev 1 called toolchain 45818982052 (on
  2483e9a) "in progress at 12:3x"; it completed at 12:28:06 UTC (job 139905981337, success).
- **The trial merge.** `git merge-tree --write-tree --name-only HEAD origin/main` (HEAD 06ed6c5): exit 1, tree
  485cd4d31c8f7c9165407c005002e6c5ee0141e8. Two CONFLICT lines, docs/STATUS.md and docs/STATUS_ARCHIVE.md (content);
  auto-merging DECISIONS.md, REPO_MAP.md, STATUS.md and STATUS_ARCHIVE.md.
  - STATUS.md's block (markers :32, :35, :39, by `git grep -n` on the tree). Ours: :33 (#40, this branch's row, its
    2026-11-15 text) and :34 (ci-after-3a's "none yet" row, main's 07d673b text). Theirs: :36 (main's #40, its
    2026-11-14 text), :37 (ci-preview-handover's "none yet" row, #74's) and :38 (guard-wrappers' "none yet" row, #73's).
    Main moved ci-after-3a's row to its archive. Main's file still lists #73 and #74 as "none yet", though both merged:
    main's text, taken as it is (ADV-M68-4).
  - STATUS_ARCHIVE.md's block (markers :2157, :2167, :2179), right under "## Open bugs and risks". Ours: :2158-2166, the
    "netcode proof test" bullet (9 lines, this branch's own: part of its 18/5 since 07d673b). Theirs: :2168-2178, main's
    new "guard-wrappers" bullet (11 lines, #73's). At 07d673b the heading was followed by the "CI, the smoke job's
    Chromium step" bullet, so both sides added a bullet at the same place. Main's tip holds no copy of ours.
  - The rest of the trial tree equals main outside spikes/, .checkpoints/ and docs/ (`git diff --stat` prints nothing)
    and HEAD in spikes/ and docs/design/ (nothing). DECISIONS 24/1 and REPO_MAP 17/0 against main: this branch's own
    (`git diff --numstat 07d673b HEAD`).
- refs: `git grep -a -l "spikes/" origin/main -- . ':(exclude)spikes/' ':(exclude)docs/' ':(exclude).checkpoints/'
  ':(exclude)*.md'` names kept-logs.mjs and its test only. Untracked: `git ls-files --others --exclude-standard -- crates
  packages docs .claude .github` prints nothing. Free disk 13,922 MiB at 17:0x UTC.
- DEP-HOLD: no Cargo.toml, Cargo.lock, package.json or package-lock.json among the 45. Nothing under spikes/, docs/adr/
  or docs/design/ (no `| PM-` row moves).

## On record (R68-5)
- R4-1 (Critical) is open on main by his R4-END (1). #73's report: "one more spelling hides git, gh or xargs from the
  guard's reading, for a session and a subagent … open on main too"; its fix is the next guard change-set; "His interim
  rule stays until R4-1's fix merges" (also main's STATUS_ARCHIVE guard-wrappers bullet).
- ADV-R4-2: 91 swept lines are looser than the guard before #73. Six pass where timeout gets a lone dash and then runs
  nothing; 85 ask in a session instead of being refused; subagents are still refused.
- #74: security-auditor's one read found nothing that stops it; its SA-1 Medium is the accepted CUT-ESCAPE (code that
  ran inside the cut can still change later steps). infra-engineer: APPROVE. (Main's STATUS row for it.)
- No security read for the take-in. These reasons carry: main's files come in byte for byte; after the merge, PR #40's
  diff against main holds no kit or CI file; #73 and #74 merged on his go and tap with these findings open by his own
  words. Two of MAIN-66's reasons do not: #73's last security round was REVISE, not APPROVE; and #73's rules do not only
  add asks or denies (ADV-R4-2).

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
     docs/STATUS_ARCHIVE.md`, and main's other 43 paths staged (A 23, M 20).
   - From the next call on, #73's rules check this session's calls.
3. Resolve by Edit, each kept line word for word from its side (C6):
   - STATUS.md: this branch's #40 row, then main's ci-preview-handover row, then main's guard-wrappers row. The
     ci-after-3a row and main's #40 row go.
   - STATUS_ARCHIVE.md: this branch's netcode proof test bullet (9 lines), then main's guard-wrappers bullet (11 lines).
     Only the three marker lines go.
   - Then the marker grep (nothing) and `wc -l docs/STATUS.md`: exactly 125, before `git add docs/STATUS.md
     docs/STATUS_ARCHIVE.md`.
   - After the add: `git diff --cached --numstat origin/main -- docs/STATUS.md` prints "1 1" (the line out is main's
     #40, its 2026-11-14 text; the line in, H0's #40 row).
   - `git diff --cached --numstat origin/main -- docs/STATUS_ARCHIVE.md` is expected at "18 5" (H0's own `git diff
     --numstat 07d673b H0 -- docs/STATUS_ARCHIVE.md`). That is a prediction; the content check governs: `git diff
     --cached -U0 origin/main -- docs/STATUS_ARCHIVE.md` shows only H0's own changes, the bullet's nine lines word for
     word among them.
4. `git commit --no-edit`: two parents, H0 and 3b949c1, git's own message (the card save carries the attribution lines).
5. a-c as written: GATE PASS (`--no-smoke`); "repo map OK", its two numbers recorded; the checkpoint tests pass, 0 fail,
   the count recorded (33 at MAIN-66; main changes no .claude/scripts/ file).
   - d: the light run (run_in_background): every count equals the base run's word for word, except HEAD, and refs =
     N0 + 23.
   - e: `node --test .claude/hooks/routine-guard.test.mjs` (main's file now), in M66-3's form (in the background,
     timeout 1800000; its summary lines by Write into .gate/main-68/step5-outputs.txt). The bar: 0 fail, the count
     recorded. Expected 1,551 (#73's report, "Evidence": "1247 on main → 1437 → 1490 → 1499 → 1551/1551"; main's STATUS
     row "guard tests 1551/1551"). Another total with 0 fail, while `git diff --quiet origin/main HEAD -- .claude/`
     exits 0, is recorded and goes on (M65-2); any fail stops.
6. As written: main's CI on 3b949c1 read again; the card save pushes (`wc -l` on the card first: 200 or fewer); CI on
   the pushed head, push and pull request: gate (gate, smoke) and toolchain green.
   - Toolchain now also runs #70's tests (devtools' recording, replay, two-instance and two-process tests; sim-worker's
     recording and replay boundary tests; the e2e replay spec in Chromium and, by #74, in Firefox).
   - #74's two steps have their own bar, since main skipped them: on PR #40's pull-request toolchain run both succeed,
     the check's printed line ("<n> files, <bytes> bytes") is recorded and the artifact `preview-site` is listed on that
     run; on the push run both are skipped. No preview starts (main has no preview.yml). A red in either stops for the
     advisor.
   - The saver restarts here, both trailers.
7. The advisor's check of the pushed head by SHA (MAIN-65 step 7's form): new agent work, in the next reset's first 30
   minutes unless it rides an agent already running. Its toolchain baseline: main's run 45848772877, job 140009301248,
   compared line for line as M67-3 gives it (the `test result:` lines, the vitest `Test Files` lines, and every
   Playwright "passed" line, Chromium and Firefox); #74's two steps by step 6's bar.

## The numbers (in place of MAIN-66's and MAIN-67's)
- C1 (a), (b), 5d: the base run gives every count (refs N0, the full marks line, self S0 of S0, redact R0 of R0 with M0
  items, 0 findings); 5d equals them word for word, except HEAD, and refs = N0 + 23.
- C1 (c):
  - `git diff --name-only H0 HEAD` equals `git diff --name-only 07d673b origin/main` (45 names).
  - Without docs/STATUS.md and docs/STATUS_ARCHIVE.md, both pairs' shortstat reads "43 files changed, 6525
    insertions(+), 173 deletions(-)".
  - The whole line is expected at "45 files changed, 6900 insertions(+), 218 deletions(-)" (STATUS.md's 48/43 and
    STATUS_ARCHIVE.md's 327/2 as main's: the swapped #40 row and H0's bullet are the same on both sides). If a line
    differs, step 3's content checks govern.
  - docs/ against main: the usual six names (DECISIONS, REPO_MAP, STATUS, STATUS_ARCHIVE, design/netcode-proof-messages,
    design/ws-reconnect).
- C2 (a): "beyond main's 45". The allowance as MAIN-66's: a ` M`, ` D` or `??` line under .checkpoints/netcode-proof-test/
  or the card. Expected: the card, this plan, advisor-main-68b.md, history.md (if he writes), msg-2026-11-27.md,
  u7-merge-plan.md (if written), the window's verdict files.
- C2 (b): after `git add docs/STATUS.md docs/STATUS_ARCHIVE.md`, each alone:
  - `git diff --name-only --diff-filter=U` prints nothing;
  - `git diff --cached --shortstat H0 -- . ':(exclude)docs/STATUS.md' ':(exclude)docs/STATUS_ARCHIVE.md'` prints the
    43-file line;
  - `git diff --cached --name-only H0 -- .checkpoints/ spikes/` prints nothing;
  - the anchor: `git diff --cached --quiet T -- . ':(exclude).checkpoints/' ':(exclude)docs/STATUS.md'
    ':(exclude)docs/STATUS_ARCHIVE.md'` exits 0 (after the commit, `HEAD` in place of `--cached`).
- The anchor T (M67-2's form): at step 1, after H0 is fixed and after `git fetch origin`, alone: `git merge-tree
  --write-tree --name-only H0 origin/main`. The bar: exit 1, two CONFLICT lines (docs/STATUS.md, docs/STATUS_ARCHIVE.md),
  and the auto-merge lines for DECISIONS.md, REPO_MAP.md, STATUS.md and STATUS_ARCHIVE.md; anything else stops before
  step 2. T goes into this section by Edit before step 2 (485cd4d3 on HEAD 06ed6c5 at this revision; not bound).
- C2 (c), C3: no builder runs on the lane. U8-F1's builder, if resumed, works in its own worktree and runs no cargo; its
  hand-back is only read until the push.
- C4: after `git fetch origin` in its own call, origin/main must be 3b949c1a9f9c99d6d54570a0f2805c3d65661d8b. Main's CI
  is its own push runs: gate 45848772772 (gate 140009303166, smoke 140009302600) and toolchain 45848772877
  (140009301248), all completed with success. A further move stops before step 2 (advisor-main-65.md:113-114), unless
  the check's answer to Q3 lets it be re-derived.
- C5 (a), the resume line, word for word: "MAIN-68 approved (advisor-main-68b.md), running steps 1-7. If `git rev-parse
  -q --verify MERGE_HEAD` prints 3b949c1a9f9c99d6d54570a0f2805c3d65661d8b, the merge is in progress: no save, no `git
  merge --abort`, no reset. If `git diff --name-only --diff-filter=U` names docs/STATUS.md or docs/STATUS_ARCHIVE.md,
  resume at step 3; otherwise resume at step 3's checks before the commit (C2 (b)), then step 4. If HEAD^2 is 3b949c1
  and HEAD is not on the remote yet, resume at step 5. advisor-main-68b.md and this card may show unstaged: leave them;
  step 6's card save takes them. U8-F1's builder works in its own worktree: nothing of it shows here. Anything else: the
  advisor rules."
- C5 (b): the saver running then is stopped by TaskStop at step 1, with its proof; it restarts at step 6.
- C6:
  - STATUS.md's block is merged lines 32-39 (`<<<<<<< HEAD` :32, `=======` :35, `>>>>>>> origin/main` :39). Keep :33
    (ours #40, the 2026-11-15 text), :37 (main's ci-preview-handover row) and :38 (main's guard-wrappers row), in that
    order. Drop :34 (ci-after-3a's row) and :36 (main's #40, the 2026-11-14 text).
  - STATUS_ARCHIVE.md's block is merged lines 2157-2179 (`<<<<<<< HEAD` :2157, `=======` :2167, `>>>>>>> origin/main`
    :2179). Keep :2158-2166 (ours, the netcode proof test bullet), then :2168-2178 (main's guard-wrappers bullet). Drop
    only the three marker lines.
  - The line numbers are re-derived on step 1's tree T (from H0) if they move; the rows and bullets stay as named.
- Step 7's toolchain baseline: run 45848772877, job 140009301248 (M67-3's list), and step 6's bar for #74's two steps.
- M65-2's fallbacks stand with these values: refs at 5d = H0's tree count + 23, with `git diff --name-only
  --diff-filter=A H0 HEAD` listing main's 23 added names; the staged tally `git diff --cached --name-status H0` = `git
  diff --name-status 07d673b origin/main` (45 paths, A 23, M 22); STATUS.md's 125 with no tolerance.

## Order with the rest of the day
- This plan's check (advisor-main-68b.md) is new agent work: in the 19:50 UTC window, by resuming the advisor that
  checked rev 1 (it holds #70's half), reading only #73's and #74's halves and the new numbers. Just before its launch:
  `git ls-remote origin refs/heads/main`; if main moved, this plan is brought up first, then checked.
- Steps 1-6 after the check, when no other merge runs. Builder 2's merge goes first whenever he is at the prompt. Its
  bars come from its own base run on its own H0, and this plan's anchor and counts are re-derived from H0 at step 1 if
  it lands first. U7's merge comes after this one (U7-M-1). A "ready" during steps 2-6 gets "wait about 40 minutes".
- Step 7 in the window after step 6, unless it rides an agent then running.
- The record, under "Choices taken without him": "main 3b949c1 taken in (#70, #73 and #74 in one merge: his word of
  12:21 UTC for #70; #73 and #74 with main's tip, by the only merge form the guard passes), plan first (MAIN-68)", named
  in msg-2026-11-27.md with all three pull requests.

## Questions for the checker
- Q1: STATUS_ARCHIVE.md's block: both sides added a bullet under "## Open bugs and risks". Is ours then main's (git's
  order) right, or main's first?
- Q2: #74's two pull-request steps on PR #40 check and upload the toolchain job's player build (.gate/dist-production).
  Does anything under spikes/ run in or feed that job, so that the uploaded build could hold anything but main's code?
  My reading: the spike is outside the root workspace (its own Cargo.toml, no path dependency up) and outside packages/.
  Anything to add?
- Q3: if main moves again before step 1, which values may be re-derived at step 1 by the same commands, with one line
  to him, without a new check (MAIN-66's Q4 form)? My lean: a docs-only move (no code, CI, rule, manifest, spikes/ or
  docs/adr/ file) whose trial merge shows no conflict outside the same two blocks; anything else stops for you.
- Q4: may step 7 ride U7's head read, or Builder 2's NB2-C10 reviewer read, in the window after step 6?

## Change type, tier, size
A merge of main's reviewed content (45 files, +6,900 / -218), with two conflict blocks resolved by hand; nothing else is
edited by hand.
- #70: five reviewers in two rounds, the verifier PASS, the advisor agent's APPROVE of its head 420b16b; merged on his go
  after his Advisor's merge check.
- #73: risk tier 2 on main; four security-auditor rounds, the last REVISE, R4-1 left open by his R4-END (1); the advisor
  agent's APPROVE of head 7bff799f; merged on his go.
- #74: risk tier 2 on main; security-auditor (SA-1 Medium, the accepted CUT-ESCAPE) and infra-engineer APPROVE; merged at
  e8340eb on his tap, after the merge check.

Tier 1 for PR #40 after the merge (no kit or CI file differs from main's). Reviewers: the advisor (this plan, then the
pushed head). Mustafa: nothing to check in Brave.
