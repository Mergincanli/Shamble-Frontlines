# Advisor: MAIN-68 check of rev 3, ruled on main's tip b9aad01 (the take-in procedure, and the values for that tip)
2026-11-26, 19:5x-20:1x UTC window. Advisor agent (the one that checked rev 1), day session, read only, effort xhigh.
I act by main's advisor.md (read whole in this run).
- Plan: main-68-plan.md rev 3, sha256 a6ff0bc46fcfa5552209e9094fc0f4ed5176d5807515ea86eb2e26578104274a (as the request
  names it).
- At the start: HEAD 07cb835 (equal to the remote branch by ls-remote); local origin/main b9aad01 (= ls-remote). I made
  no fetch.
- Rev 1's facts (advisor-main-68.md section 3) stand for #70's half. I read #73's, #74's and #76's halves in rev 3, and
  spot-checked them by git now that the objects are local. #77's and #75's halves I read from git, as I read #70's.

## 1. Verdict: APPROVE WITH CONDITIONS C68B-1 to C68B-6, for main's tip b9aad01954be666340a7ec5c6e9fc9353b11abf1
**The departure.** This verdict covers b9aad01, a tip rev 3 never saw. It departs from rev 3's own "Order" line ("if
main moved, this plan is brought up first, then checked"), and I rule that line replaced (C68B-1):
- Three revisions have each been overtaken. Each wrote its numbers, then waited up to five hours for a check window,
  and main merged in between: nine merges today.
- From now on the check reads main's tip itself and binds that tip's values, and the main session starts step 1 as
  soon as the verdict is in. The gap shrinks from hours to minutes.
- No rule of his is loosened: every code, CI, rule or manifest move is still checked by the advisor before it is taken
  in. This check is the check of #77 and #75.

**The tip's facts hold** (section 4). They are:
- main's six first-parent merges since 07d673b, their 87 paths and two size lines;
- the trial merge on HEAD 07cb835 (two conflict blocks, one each, of the shape rev 3 names);
- the kit and CI facts;
- refs, untracked files and disk;
- main's green CI on b9aad01.

**Rev 3 is wrong for b9aad01 in five places.** Each is fixed by the values below (C68B-2), applied word for word:
- the tip and its CI ids;
- every count (47 → 87);
- DEP-HOLD's line: "no Cargo.toml, Cargo.lock …" is now false;
- C6's rows and line numbers;
- step 7's baseline.

**Severity and tier.** ADV-M68B-1, High: rev 3's facts no longer hold for the tip it would merge. C4 would stop it
safely. Nothing is Critical. Risk tier 1 for PR #40 after the merge.

**Applied word for word, these conditions need no new review.** Rev 4 is rev 3 with them applied by Edit. Step 7's read
of the pushed head stays.

## 2. Conditions (paste-ready)
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

## 3. Rulings (the request's items 3 and 4)
**Which tip.** b9aad01954be666340a7ec5c6e9fc9353b11abf1, by the values in C68B-2.

**P-2: a move of main after b9aad01, found at step 1's fetch.**
- **(a) Re-derived at step 1 without a new check, with one line to him: a docs-only move.** All of these must hold,
  each command in its own call:
  - **Each merge, read alone.** Every first-parent merge M in `git log --first-parent --format='%H %s'
    b9aad01..origin/main`, read alone (`<M>^1 <M>`), passes the LIGHT-TAKEIN recipe's entry checks as I conditioned
    them (advisor-light-takein.md): E1, E1b (L-5), E2 with L-7, E3 with L-1, and E4.

    In plain words: docs .md files only, plain files; no CLAUDE.md, CLAUDE.local.md, AGENTS.md or `.claude/` path; no
    design or look doc; no `| PM-` row.
  - **CI.** Main's push runs on the new tip (gate with gate and smoke, and toolchain) completed with success. If they
    are still running, wait.
  - **The trial merge.** `git merge-tree --write-tree --name-only H0 origin/main` exits 1 with exactly the two CONFLICT
    lines (docs/STATUS.md, docs/STATUS_ARCHIVE.md). Its auto-merge lines name only DECISIONS.md, REPO_MAP.md,
    STATUS.md and STATUS_ARCHIVE.md, and each of the two files has one block.
  - **C6.** C68B-4's preconditions hold.

  Then:
  - re-derive by the same commands every value in C68B-2 that does not depend on H0, with refs = N0 + A - D;
  - write them into "The numbers" by Edit under "re-derived on <sha7> (advisor-main-68b.md P-2 (a))";
  - send him one line naming the new pull requests, in LIGHT-TAKEIN's form;
  - start step 1 again from its first line.

  This is my reading of his LIGHT-TAKEIN (history.md:8053). Each new merge is inside his limits on its own (docs .md
  only, CI green). It adds no conflict this check has not seen: the archive's block is #73's and #76's, ruled here. And
  step 7's read follows, as his word keeps it. If he reads his limits otherwise, a docs-only move stops for a check like
  any other.
- **(b) Anything else stops before step 2 for a new check.** That means a merge touching any of:
  - code (crates/, packages/), CI (.github/), the kit, tools/ or legacy/;
  - a manifest or lockfile, spikes/ or docs/design/;
  - a design or look doc, or a `| PM-` row.

  It also means a third CONFLICT file, a second block, another auto-merge, or main's CI not green.

  **That includes a reviewed code pull request merged on his go, like #75.** His words send code to the full take-in:
  - 12:21 UTC: "#70 … merges today and needs a full take-in";
  - 20:48 UTC: "#71 … and later #70 … Neither fits the lighter recipe";
  - LIGHT-TAKEIN: "Anything outside falls back to the full plan".

  A code move also changes what the check must read: DEP-HOLD's files (#75), the toolchain baseline and the job's time
  (9 min 20 s of 15 now), the tests PR #40's CI runs, and refs. Loosening this is his to decide (TAKEIN-CODE, Decisions
  needed).

  The new check reads only the new merges and re-derives the values, as this one did. It runs in a window's first 30
  minutes, or rides an advisor agent already running, and binds the new tip. Step 1 then starts at once.
- **(c) A move after step 1's fetch changes nothing.** Step 2 merges the fetched origin/main (C68B-1's rev-parse), and
  no other fetch comes before step 6 (advisor-main-66.md Q4).

**Rev 3's questions:**
- **Q1, the archive's order: main's bullets first, then this branch's.** That changes rev 3, and the request's
  proposed rule.
  - Both orders pass the same bars: numstat 18 5, the -U0 lines, and the line count.
  - Ours first keeps our bullet directly under "## Open bugs and risks". Main's docs passes add new bullets at exactly
    that place (#73 did, then #76), so every later one would clash with ours.
  - A second conflict file puts a move outside his "the one conflict is STATUS.md's table". So ours first would put
    every later move outside LIGHT-TAKEIN.
  - Main's first leaves our bullet between main's last new bullet and the CI bullet, the line it sat above at 07d673b.
    The next take-in's base is then main's file, and main's next bullet under the heading merges cleanly.
- **Q2: nothing under spikes/ runs in or feeds the toolchain job.**
  - npm workspaces are `packages/*` only (main's package.json).
  - The cargo members are `crates/*` only; the spike has its own `[workspace]` and no path dependency up.
  - Nothing under `.gate/` is tracked.
  - toolchain.yml, build.mjs and playwright.config.ts name no spikes/ or .checkpoints/ path.
  - The refs step proves that nothing outside spikes/ references the spike, but kept-logs.
  - One addition: the pull-request run builds PR #40's head merged with main's tip at that moment. If main moves after
    step 6, the uploaded build holds that newer main code: still main's only. A red in #74's two steps stops for the
    advisor, as rev 3 says.
- **Q4: step 7 may ride U7's head read** (an advisor agent), on advisor-main-66.md Q1 (d)'s terms, with three agents
  at most.
  - It may not ride NB2-C10's reviewer read: that is a reviewer agent (launch-1450-msgs.md:6), and step 7 is the
    advisor's check.
  - It never waits for U7's head.
- **Q5: right, no screenshot.**
  - The wall bullet's rule (main's STATUS_ARCHIVE.md:2386-2388) is: "A day session asks him for a screenshot of both
    rulesets at the start of any kit, CI or gate change-set". Its reason: the wall does not protect itself.
  - MAIN-68 changes no kit, CI or gate file against main. After it, PR #40's diff against main holds none. And nothing
    a merge on this branch does can change a ruleset.
  - It is a take-in, not such a change-set.

## 4. What I checked (read only; HEAD 07cb835, origin/main b9aad01 = ls-remote)
- **The merges.** First-parent 07d673b..b9aad01: six merges, as C68B-2 lists them. #77's message: docs only, merged
  by the PR Advisor after its APPROVE of 4e8a87e. #75's message: its title only.
- **#77 and #75 together (2671595..b9aad01).** 57 paths, "57 files changed, 44456 insertions(+), 85 deletions(-)",
  all 100644 (`--raw`). That is A 36 and M 21, as the request lists them:
  - crates/content: Cargo.toml and 10 sources;
  - devtools: Cargo.toml, three sources, a bin, five tests and seven vector files;
  - sim: lib.rs, time.rs and tests/rules_scan.rs;
  - packages: four client scripts, two e2e specs, three sim-worker boundary files;
  - Cargo.lock 5/2;
  - docs: DECISIONS 175/0, REPO_MAP 28/12, ROADMAP 26/11, STATUS 17/17, STATUS_ARCHIVE 171/0, TECHNICAL 1/1,
    adr/README 8/6, two reports.

  Nothing under .claude, .github, tools, legacy, spikes or docs/design. No .gitignore, CLAUDE.md, CLAUDE.local.md,
  AGENTS.md, package.json, package-lock.json, .npmrc or rust-toolchain.toml.
- **The manifests' diff** (read whole): the `content` crate has no dependency; devtools drops client-endpoint and
  room, adds content; Cargo.lock adds the local package only.
- **The whole move (07d673b..b9aad01).** 87 paths, A 60, M 27, with C68B-2's size lines.
  - `-G'^[|] PM-'` under docs/: nothing.
  - The refs grep (`-a -l`): kept-logs.mjs and its test only.
  - Untracked files in main's paths: none.
- **The trial merge, `git merge-tree --write-tree --name-only HEAD origin/main`:** exit 1, tree 6497e29d. Two
  CONFLICT lines (STATUS.md, STATUS_ARCHIVE.md); auto-merging DECISIONS, REPO_MAP, STATUS, STATUS_ARCHIVE.
  - STATUS.md: markers :34, :37, :41; the rows as C68B-4 names them.
  - STATUS_ARCHIVE.md: markers :2385, :2395, :2421. Ours :2386-2394 (the 9-line netcode bullet); theirs :2396-2420
    (MAIN-WALL 14 lines, guard-wrappers 11).
  - `-U1` against main shows only the two blocks and the branch's own archive hunks.
  - The tree's docs against main: DECISIONS 24/1, REPO_MAP 17/0, STATUS 5/0 and STATUS_ARCHIVE 21/5 (the branch's own
    lines plus markers), and the two design files.
  - Outside spikes/, .checkpoints/ and docs/ it equals main (exit 0); spikes/ and docs/design/ equal HEAD's (exit 0).
  - HEAD..tree without the two STATUS files and .checkpoints/: the 85-file line.
- **The branch's own side.** `git diff --numstat 07d673b HEAD`: STATUS 1/1, STATUS_ARCHIVE 18/5, DECISIONS 24/1,
  REPO_MAP 17/0. The -G #40 check: names STATUS.md on 07d673b..HEAD, and nothing on 07d673b..origin/main. Main's
  STATUS.md has 120 lines and STATUS_ARCHIVE.md 2,720.
- **The kit.** HEAD...origin/main: exactly routine-guard.mjs, its test and toolchain.yml. origin/main...HEAD over the
  kit pathspec: nothing. The merge rule is at main's routine-guard.mjs:1428 and :1430, as rev 3 quotes it. The branch
  changes no root code, manifest or kit path since 07d673b. Since rev 3's HEAD 590246f, it changed nothing outside
  .checkpoints/.
- **CI on b9aad01**, by run and job id (C68B-2). #77's two runs: cancelled.
- **#75's pull request (GitHub):** merged 18:53:15 UTC by Mergincanli, head 2542b1a, base 2671595; its body as
  C68B-5 quotes it; no comments. Main's DECISIONS.md:183-203 records his message of 18:16:35 UTC, in which his
  Advisor's merge check of 2e28217 passed and main had to be taken in first.
- **His words since 13:04 UTC** (history.md:8108-8131): U8F1-SIZE (a), LT-CLEAN (a), "ready", "go" (Builder 2's
  merge). Nothing on take-ins of code.
- **Disk:** 7,696 MiB free at 20:0x UTC (the light runs need 4,096).

## 5. Findings (CLAUDE.md's scale)
| Id | Severity | What | Fix |
|---|---|---|---|
| ADV-M68B-1 | High | Main moved after rev 3 (#77 docs; #75 code, manifests and docs). Rev 3's C4, counts, DEP-HOLD line, C6 rows and line numbers, and step 7 baseline are wrong for b9aad01. | C68B-2, C68B-3, C68B-4 |
| ADV-M68B-2 | Medium | "Brought up first, then checked" cannot converge while checks wait for a window: three revisions overtaken. | C68B-1, P-2 |
| ADV-M68B-3 | Medium | Rev 3's archive order (ours first) keeps our bullet where main's docs passes add theirs, so every later move would clash in a second file and fall outside LIGHT-TAKEIN. | C68B-4 (main's first) |
| ADV-M68B-4 | Low (on record) | #75's merge message and main's STATUS row record no go. Its pull request names the advisor-approved head 2542b1a, and that head was merged. His go is lane 1's record. | C68B-5; one line to him names #75 |
| ADV-M68B-5 | Low (on record) | The toolchain job on b9aad01 took 9 min 20 s of its 15-minute limit. PR #40's runs add #74's two steps. A timeout there is the runner's, and the advisor rules before the next step. | None now |
| ADV-M68B-6 | Low (on record) | Main's STATUS.md lists MAIN-WALL as "none yet" and #75 as "(draft)", both merged. It is main's text, taken as it is (as ADV-M68-4). | Main's next docs pass |
| ADV-M60-7, ADV-M54-8 | Low (open) | As before. | As ruled |

## 6. What I did not check
- I ran no merge, gate, light run or test (read only). The bars' values come from the trial tree.
- I did not read main-68-rev4-values.md. If its numbers differ from C68B-2's, C68B-2 governs, and a value that cannot
  be reproduced stops before step 2.
- The guard tests' 1,551 is #73's report's count, not mine.
- I did not re-run the patch-ids (they need a pipe).

## Decisions needed
- **TAKEIN-CODE: may a reviewed code pull request merged on main (like #75) be taken in at step 1 by re-derived
  numbers, without a new advisor check?**
  - Why it matters: main moved nine times today, and each code move between a check and step 1 costs a new check and a
    window's wait.
  - Options:
    - (a) No. A code move always gets the advisor's short check of the new merges, which binds the new tip; step 1
      follows at once.
    - (b) Yes, if the merge touches no kit, CI, manifest or lockfile, spikes/, design or look doc or mark row, main's CI
      is green and the conflicts are the same; one line to you; the advisor's read of the pushed head still follows.
  - My recommendation: (a). With the check now reading main's tip and step 1 following at once, the gap is minutes.
    Every code move today (#70, #74, #75) also carried something (code, CI, manifests) that changed what the check had
    to read. Until you answer, (a) holds, and nothing waits on it.

## For Mustafa
1. Answer TAKEIN-CODE, (a) or (b), whenever it suits you. Nothing waits on it.
2. The take-in's one line will name #75. Its merge message and main's STATUS row record no go of yours (its pull request
   says it merges only on your go in the builder chat). If it merged without your go, say so.

DONE
