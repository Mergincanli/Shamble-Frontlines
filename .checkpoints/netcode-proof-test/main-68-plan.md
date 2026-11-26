# MAIN-68: take main b9aad01 (#70, #73, #74, #76, #77, #75) into the lane: the plan, rev 4 (rev 3 as checked)
Rev 4 (2026-11-26 20:1x UTC) is rev 3 (sha256 a6ff0bc46fcfa5552209e9094fc0f4ed5176d5807515ea86eb2e26578104274a, on
2671595) with C68B-1 to C68B-6 applied word for word: advisor-main-68b.md, APPROVE WITH CONDITIONS for main's tip
b9aad01954be666340a7ec5c6e9fc9353b11abf1 (20:0x UTC). No new check; step 7's read of the pushed head stays. Where a rev 3
line below differs from "As checked", "As checked" governs.
Rev 3 brought up 2026-11-26 18:4x-18:5x UTC: main moved after rev 2 by one docs-only merge, #76 (MAIN-WALL's record,
18:00:25 UTC), so rev 2's "Order" applies ("if main moved, this plan is brought up first, then checked"). Rev 2 (16:5x-17:2x
UTC, on 3b949c1, sha256 20d27df93c27d79bffa29c8e401ebc8c54763ed08430a5def4534638d2e13832) brought rev 1 up by R68-1 to
R68-8 (advisor-main-68.md). Rev 1 (12:5x UTC, on 2483e9a, sha256 e0bb8c75…2eca24) was REVISE: main moved after it was
written. Its check: advisor-main-68b.md.

## As checked: C68B-1 to C68B-6 (advisor-main-68b.md section 2), word for word; P-2 and the Q answers in its section 3
- **C68B-1 (Medium): the procedure.** Replace rev 3's "Order" first bullet with:

  "This plan's check binds the tip it reads (advisor-main-68b.md: b9aad01). Step 1 starts as soon as the check's values
  are in this plan, in the same window. At step 1, after `git fetch origin` in its own call, C4 reads origin/main:
  - equal to b9aad01954be666340a7ec5c6e9fc9353b11abf1: go on.
  - moved: classify the new first-parent merges by P-2 (advisor-main-68b.md section 3). Under P-2 (a), re-derive and
    tell him in one line. Under P-2 (b), stop before step 2 for a new check, which reads only the new merges and binds
    the new tip.

  In step 2, before the merge and in its own call: `git rev-parse origin/main` prints the bound tip; otherwise stop
  before step 2."
- **C68B-2 (High): the values for b9aad01.** They replace rev 3's "What rev 3 changes", the Facts' counts and "The
  numbers". Each one equals my reading in section 4.
  - **C4.** origin/main = b9aad01954be666340a7ec5c6e9fc9353b11abf1. Main's push runs, attempt 1, all success:
    - gate 45879443239 (gate 140114837556, smoke 140114837338);
    - toolchain 45879443258 (job 140114848934).

    #77's runs on 47bb79e were cancelled by #75's push; only the tip's runs count.
  - **First-parent 07d673b..b9aad01:** #70 2483e9a, #73 4a62873, #74 3b949c1, #76 2671595, #77 47bb79e, #75 b9aad01.
    The merge base is 07d673b.
  - **C1 (c).** `git diff --name-only H0 HEAD` equals `git diff --name-only 07d673b origin/main`: 87 names, A 60, M 27,
    no D or R, all 100644.
    - Whole: "87 files changed, 51544 insertions(+), 271 deletions(-)".
    - Without docs/STATUS.md and docs/STATUS_ARCHIVE.md: "85 files changed, 50926 insertions(+), 223 deletions(-)" (both
      pairs).
    - STATUS.md 46/46, STATUS_ARCHIVE.md 572/2.
    - docs/ against main: the usual six names.
  - **C1 (a), (b), 5d:** refs = N0 + 60.
  - **Step 2:** `UU docs/STATUS.md`, `UU docs/STATUS_ARCHIVE.md`, and main's other 85 paths staged (A 60, M 25).
  - **C2 (a):** "beyond main's 87".
  - **C2 (b):** the 85-file line, then the anchor as rev 3 gives it. T is from H0 (6497e29d on HEAD 07cb835; not
    bound). T's bar adds "and one `<<<<<<<` line in each of the two files (`git grep -c -E '^<<<<<<<' T --
    docs/STATUS.md docs/STATUS_ARCHIVE.md`)".
  - **C5 (a):** rev 3's resume line, with b9aad01954be666340a7ec5c6e9fc9353b11abf1 in place of 2671595's full SHA, and
    "HEAD^2 is b9aad01" in place of "HEAD^2 is 2671595".
  - **5a-5c and 5e:** as rev 3. #75 and #77 change no .claude/, tools/ or .github/ file, so 5e expects 1,551 and the
    checkpoint tests 33.
  - **Step 6's toolchain on PR #40** also runs #75's tests:
    - the content crate's own tests;
    - devtools' content proofs, fuzz, time unit, aim reach and dependency rules;
    - sim's rules scan;
    - the client scripts' two tests;
    - the e2e content spec under tests/rules/ (Chromium, and Firefox by #74);
    - the sim-worker content and plain-file boundary tests.

    On main this job took 18:53:35-19:03:18, its build-and-test step 9 min 20 s of the job's 15.
  - **Step 7's baseline:** job 140114848934, by M67-3's list. #74's two steps were skipped on main (its steps 11-12),
    so rev 3's own bar for them stands.
  - **M65-2's fallbacks:** refs = H0's tree count + 60, with `--diff-filter=A H0 HEAD` naming main's 60. The staged
    tally equals `git diff --name-status 07d673b origin/main` (87, A 60, M 27). STATUS.md's line count is main's (120
    today), with no tolerance.
  - If a value that does not depend on H0 reads otherwise at step 1 on b9aad01, stop before step 2; the advisor rules.
    The values that depend on H0 are T, its line numbers, N0, M0, S0 and R0.
- **C68B-3 (Medium): DEP-HOLD** (the request's item 1). Replace the Facts' DEP-HOLD line with:

  "DEP-HOLD: main changes three dependency files, Cargo.lock (5/2), crates/content/Cargo.toml (new, 13 lines) and
  crates/devtools/Cargo.toml (3/2), all #75's. This branch changes none of them (`git diff --name-only 07d673b H0 --
  Cargo.toml Cargo.lock crates packages package.json package-lock.json` prints nothing), so after the merge each equals
  main's byte for byte. DEP-HOLD's own exception covers that: 'A manifest or lockfile is not held when its content in
  the push equals its content on main (origin/main as last fetched)' (.claude/CHECKPOINTS.md §3;
  checkpoint.mjs:23-24, :135). That is MAIN-66's precedent (#68's manifests, main-66-plan.md:95-99). No
  dependency-check.md line is written for main's files. No registry package enters: Cargo.lock's diff adds only the
  path crate `content` (no `source` or `checksum` line) and changes devtools' dependency list. Before step 6's save, in
  its own call, `git diff --quiet origin/main HEAD -- Cargo.lock Cargo.toml crates packages package.json
  package-lock.json` exits 0. If the save still reports a held file (exit 1, 'NOT pushed'), stop: no record is written,
  and the advisor rules. That can happen if origin/main moved locally after step 1."
- **C68B-4 (Medium): C6 as a rule with derived bars** (the request's item 2), in place of rev 3's C6 and step 3:
  - **Preconditions, read at step 1 on T (each alone):**
    - `git diff --numstat 07d673b H0 -- docs/STATUS.md` prints "1 1", and its two lines are #40 rows;
    - `git diff --name-only -G'^[|] #40 [|]' 07d673b origin/main -- docs/STATUS.md` prints nothing (main's own #40 row
      unchanged);
    - each of the two files has one block in T (C68B-2's count);
    - in the archive's block, ours' side is H0's netcode proof test bullet (its lines are + lines of `git diff -U0
      07d673b H0 -- docs/STATUS_ARCHIVE.md`), and main's side holds only main's + lines since 07d673b.

    A miss stops before step 2.
  - **STATUS.md, the rule:** the block becomes main's side, with main's #40 row replaced by H0's #40 row byte for byte;
    nothing else changes. (Today that means: keep T's :35, then :39 and :40, the MAIN-WALL and #75 rows; drop :36,
    ci-after-3a's row, which main moved, and :38, main's #40.)
  - **STATUS_ARCHIVE.md, the rule:** the block becomes main's side as main has it, then ours' side (H0's bullet) byte
    for byte; only the three marker lines go. That is main's first: see Q1. (Today: T's :2396-2420, then
    :2386-2394.)
  - **The bars, after `git add docs/STATUS.md docs/STATUS_ARCHIVE.md`, each alone:**
    - the marker grep prints nothing;
    - `wc -l docs/STATUS.md` equals `git grep -c "" origin/main -- docs/STATUS.md` (120 today);
    - `git diff --cached --numstat origin/main -- docs/STATUS.md` prints "1 1", and `git diff --cached -U0 origin/main
      -- docs/STATUS.md` shows main's #40 row out and H0's in;
    - `git diff --cached --numstat origin/main -- docs/STATUS_ARCHIVE.md` equals `git diff --numstat 07d673b H0 --
      docs/STATUS_ARCHIVE.md` (18 5 today);
    - `wc -l docs/STATUS_ARCHIVE.md` equals main's count plus H0's added minus its removed (2720 + 13 = 2733 today);
    - `git diff --cached -U0 origin/main -- docs/STATUS_ARCHIVE.md` holds exactly the + and - lines of `git diff -U0
      07d673b H0 -- docs/STATUS_ARCHIVE.md`, positions aside.

    A miss stops; the merge stays; the advisor rules.
- **C68B-5 (Low): the reason, the record and his line.**
  - "Why" adds:
    - #77 (47bb79e, 18:52:17 UTC): docs only, merged by the PR Advisor routine (its message);
    - #75 (b9aad01, 18:53:13 UTC): code, manifests and docs. It comes in with main's tip, by the only merge form the
      guard passes.
  - On record for #75:
    - its merge message holds only its title;
    - main's STATUS row still reads "#75 (draft) … his go for the head the advisor agent approves";
    - its pull request body names the advisor agent's APPROVE of head 2542b1a and says "A merge happens only on
      Mustafa's go in the builder chat, for the head above";
    - GitHub shows it merged at 18:53:15 UTC by Mergincanli with head 2542b1a.

    His go is lane 1's record, not on file here. The take-in takes main as it is.
  - The record line names all six: "main b9aad01 taken in (#70, #73, #74, #76, #77 and #75 in one merge: his word of
    12:21 UTC for #70; the rest with main's tip, by the only merge form the guard passes), plan first (MAIN-68)". His
    one line in msg-2026-11-27.md names the same six.
- **C68B-6 (Low): rev 3's Q answers bind as section 3 gives them:** Q1's order, in C68B-4; Q2's addition; Q4's limit;
  Q5.
- The values file main-68-rev4-values.md (the main session's own reading of b9aad01, 19:5x-20:0x UTC) agrees with every
  value both name; C68B-2 governs.

## Why (R68-2)
MAIN-52 calls for it. Main moved from 07d673b (MAIN-67's) to 2671595 by four merges, and each of the first three alone
calls for a take-in:
- #70 changes code (crates/, packages/);
- #73 changes rules (two kit files: the day guard and its tests);
- #74 changes CI (.github/workflows/toolchain.yml) and code outside spikes/ (packages/e2e/playwright.config.ts);
- #76 changes docs .md files only (MAIN-WALL's record of main's wall and his decisions);
- #77 (47bb79e, 18:52:17 UTC): docs only, merged by the PR Advisor routine (its message);
- #75 (b9aad01, 18:53:13 UTC): code, manifests and docs. It comes in with main's tip, by the only merge form the guard
  passes. (C68B-5; on record for #75: C68B-5 in "As checked".)

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
- DEP-HOLD: main changes three dependency files, Cargo.lock (5/2), crates/content/Cargo.toml (new, 13 lines) and
  crates/devtools/Cargo.toml (3/2), all #75's. This branch changes none of them (`git diff --name-only 07d673b H0 --
  Cargo.toml Cargo.lock crates packages package.json package-lock.json` prints nothing), so after the merge each equals
  main's byte for byte. DEP-HOLD's own exception covers that: 'A manifest or lockfile is not held when its content in
  the push equals its content on main (origin/main as last fetched)' (.claude/CHECKPOINTS.md §3;
  checkpoint.mjs:23-24, :135). That is MAIN-66's precedent (#68's manifests, main-66-plan.md:95-99). No
  dependency-check.md line is written for main's files. No registry package enters: Cargo.lock's diff adds only the
  path crate `content` (no `source` or `checksum` line) and changes devtools' dependency list. Before step 6's save, in
  its own call, `git diff --quiet origin/main HEAD -- Cargo.lock Cargo.toml crates packages package.json
  package-lock.json` exits 0. If the save still reports a held file (exit 1, 'NOT pushed'), stop: no record is written,
  and the advisor rules. That can happen if origin/main moved locally after step 1. (C68B-3.) Nothing under spikes/ or
  docs/design/; docs/adr/README.md 8/6 (#75), no `| PM-` row moves.

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
   - Before the merge, in its own call (C68B-1): `git rev-parse origin/main` prints
     b9aad01954be666340a7ec5c6e9fc9353b11abf1; otherwise stop before step 2.
   - Refused before git runs: prove nothing changed (no MERGE_HEAD, HEAD = H0, status shows only notes lines), try no
     other form, wording or worktree, restart the saver, and tell him the one command with options (a) approve it in a
     prompt or (b) wait, my lean (a). A prompt: he answers it.
   - Expected: exit 1, "CONFLICT (content): Merge conflict in docs/STATUS.md" and "CONFLICT (content): Merge conflict in
     docs/STATUS_ARCHIVE.md", no other CONFLICT line. `git status --short` shows `UU docs/STATUS.md`, `UU
     docs/STATUS_ARCHIVE.md`, and main's other 85 paths staged (A 60, M 25) (C68B-2).
   - From the next call on, #73's rules check this session's calls.
3. By C68B-4 ("As checked"), in place of rev 3's step 3: by Edit, STATUS.md's block becomes main's side with main's #40
   row replaced by H0's #40 row byte for byte; STATUS_ARCHIVE.md's block becomes main's side as main has it, then H0's
   bullet byte for byte; only the three marker lines go in each. Then `git add docs/STATUS.md docs/STATUS_ARCHIVE.md`
   and C68B-4's bars, each alone. Today's lines on T (re-derived at step 1): STATUS.md keep :35, then :39 and :40, drop
   :36 and :38; STATUS_ARCHIVE.md :2396-2420, then :2386-2394.
4. `git commit --no-edit`: two parents, H0 and b9aad01, git's own message (the card save carries the attribution lines).
5. a-c as written: GATE PASS (`--no-smoke`); "repo map OK", its two numbers recorded; the checkpoint tests pass, 0 fail,
   the count recorded (33 at MAIN-66; main changes no .claude/scripts/ file).
   - d: the light run (run_in_background): every count equals the base run's word for word, except HEAD, and refs =
     N0 + 60 (C68B-2).
   - e: `node --test .claude/hooks/routine-guard.test.mjs` (main's file now), in M66-3's form (in the background,
     timeout 1800000; its summary lines by Write into .gate/main-68/step5-outputs.txt). The bar: 0 fail, the count
     recorded. Expected 1,551 (#73's report, "Evidence": "1247 on main → 1437 → 1490 → 1499 → 1551/1551"; main's STATUS
     row "guard tests 1551/1551"; #76 changes no .claude/ file). Another total with 0 fail, while `git diff --quiet
     origin/main HEAD -- .claude/` exits 0, is recorded and goes on (M65-2); any fail stops.
6. As written: main's CI on b9aad01 read again; before the save, in its own call, C68B-3's `git diff --quiet origin/main
   HEAD -- Cargo.lock Cargo.toml crates packages package.json package-lock.json` exits 0; the card save pushes (`wc -l`
   on the card first: 200 or fewer); CI on the pushed head, push and pull request: gate (gate, smoke) and toolchain
   green, toolchain with #75's tests too (C68B-2's list).
   - Toolchain now also runs #70's tests (devtools' recording, replay, two-instance and two-process tests; sim-worker's
     recording and replay boundary tests; the e2e replay spec in Chromium and, by #74, in Firefox).
   - #74's two steps have their own bar, since main skipped them: on PR #40's pull-request toolchain run both succeed,
     the check's printed line ("<n> files, <bytes> bytes") is recorded and the artifact `preview-site` is listed on that
     run; on the push run both are skipped. No preview starts (main has no preview.yml). A red in either stops for the
     advisor.
   - The saver restarts here, both trailers.
7. The advisor's check of the pushed head by SHA (MAIN-65 step 7's form): new agent work, in the next reset's first 30
   minutes unless it rides an agent already running. Its toolchain baseline: main's run 45879443258, job 140114848934,
   compared line for line as M67-3 gives it (the `test result:` lines, the vitest `Test Files` lines, and every
   Playwright "passed" line, Chromium and Firefox); #74's two steps by step 6's bar.

## The numbers (in place of MAIN-66's and MAIN-67's; for b9aad01 C68B-2 governs every value here, and rev 3's 2671595 values left below are the record only)
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
- C4: after `git fetch origin` in its own call, origin/main must be b9aad01954be666340a7ec5c6e9fc9353b11abf1. Main's CI
  is its own push runs, attempt 1, all success: gate 45879443239 (gate 140114837556, smoke 140114837338) and toolchain
  45879443258 (job 140114848934); #77's runs on 47bb79e were cancelled by #75's push, and only the tip's runs count. A
  further move: C68B-1 and P-2 (advisor-main-68b.md section 3). (C68B-2.)
- C5 (a), the resume line, word for word: "MAIN-68 approved (advisor-main-68b.md), running steps 1-7. If `git rev-parse
  -q --verify MERGE_HEAD` prints b9aad01954be666340a7ec5c6e9fc9353b11abf1, the merge is in progress: no save, no `git
  merge --abort`, no reset. If `git diff --name-only --diff-filter=U` names docs/STATUS.md or docs/STATUS_ARCHIVE.md,
  resume at step 3; otherwise resume at step 3's checks before the commit (C2 (b)), then step 4. If HEAD^2 is b9aad01
  and HEAD is not on the remote yet, resume at step 5. advisor-main-68b.md and this card may show unstaged: leave them;
  step 6's card save takes them. U8-F1's builder works in its own worktree: nothing of it shows here. Anything else: the
  advisor rules."
- C5 (b): the saver running then is stopped by TaskStop at step 1, with its proof; it restarts at step 6.
- C6: by C68B-4 ("As checked"), in place of rev 3's C6: its preconditions read at step 1 on T, its rule for each file,
  its bars after the add.
- Step 7's toolchain baseline: run 45872417457, job 140090951266 (M67-3's list), and step 6's bar for #74's two steps.
- M65-2's fallbacks stand with these values: refs at 5d = H0's tree count + 24, with `git diff --name-only
  --diff-filter=A H0 HEAD` listing main's 24 added names; the staged tally `git diff --cached --name-status H0` = `git
  diff --name-status 07d673b origin/main` (47 paths, A 24, M 23); STATUS.md's 120 with no tolerance.

## Order with the rest of the day
- This plan's check binds the tip it reads (advisor-main-68b.md: b9aad01). Step 1 starts as soon as the check's values
  are in this plan, in the same window. At step 1, after `git fetch origin` in its own call, C4 reads origin/main:
  - equal to b9aad01954be666340a7ec5c6e9fc9353b11abf1: go on.
  - moved: classify the new first-parent merges by P-2 (advisor-main-68b.md section 3). Under P-2 (a), re-derive and
    tell him in one line. Under P-2 (b), stop before step 2 for a new check, which reads only the new merges and binds
    the new tip.

  In step 2, before the merge and in its own call: `git rev-parse origin/main` prints the bound tip; otherwise stop
  before step 2. (C68B-1.)
- Steps 1-6 after the check, when no other merge runs. Builder 2's merge landed first (48efc9c, 18:02 UTC; pushed
  23e9e92), so H0 is past it, and this plan's anchor and counts are re-derived from H0 at step 1. U7's merge comes after
  this one (U7-M-1); its check.mjs whole run needs the part-clean first (8,192 MiB; 7,708 free at 18:5x).
- Step 7 in the window after step 6, unless it rides an agent then running.
- The record, under "Choices taken without him": "main b9aad01 taken in (#70, #73, #74, #76, #77 and #75 in one merge:
  his word of 12:21 UTC for #70; the rest with main's tip, by the only merge form the guard passes), plan first
  (MAIN-68)". His one line in msg-2026-11-27.md names the same six. (C68B-5.)

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
A merge of main's reviewed content (87 files, +51,544 / -271: six pull requests, #70, #73, #74, #76, #77 and #75), with
two conflict blocks resolved by hand; nothing else is edited by hand. (Rev 3's 47 files and four pull requests were for
2671595.)
- #77: docs only, risk tier 0; merged by the PR Advisor routine after its APPROVE of head 4e8a87e.
- #75: code, manifests and docs, risk tier 1 by main's STATUS row; reviewed in two rounds with no Critical or High;
  merged at 18:53:15 UTC with head 2542b1a, the advisor agent's approved head (C68B-5 on record).
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
