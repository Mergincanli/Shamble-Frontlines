# Advisor: MAIN-65 pushed-head check (pushed 82bd694; merge 39a0d04; branch head 610e412 at the end)
2026-11-25, 03:5x-04:1x UTC. Read only, effort xhigh. Acting by main's advisor.md (`git show
origin/main:.claude/agents/advisor.md`, read whole). Brief: main-65-step7-request.txt (sha256 b03fc992…1847ca, matches).
Measured against main-65-plan.md, advisor-main-65.md (M65-1 to M65-5, its section 3 and section 4 (d)), and the form
of advisor-main-64-head.md. Kept short (NPT-TRIM). Notes of every check: section 7.

## 1. Verdict: APPROVE WITH FINDINGS (heads checked: pushed 82bd69428512e679b789797d674ef04bd4743715; now 610e41242e8a…)
The take-in matches the plan. M65-1, M65-2, M65-3 and M65-5 are met. M65-4 is met apart from one record detail
(ADV-M65H-1, Low). Merge 39a0d04 has parents H0 473f5c1 and main 28254a7 (merge
base 1ec6f6c). Its change from H0 is main's own, name for name (64 paths, 23 added, 41 modified). Outside spikes/,
.checkpoints/ and docs/, the pushed tree equals main's. spikes/ and docs/design/ equal H0's. STATUS.md has 120 lines and
differs from main in the #40 row alone. No save ran between the merge and the card save: the remote went from H0
straight to 82bd694. CI is green on 82bd694: 6 of 6 check runs (gate, smoke, toolchain; push and pull request).
Toolchain's summary lines equal main's. My own reruns pass: gate, repo map 186/110, checkpoint tests 33/33.
Every commit since touches .checkpoints/ only. **The lane may go on from this head.**
One new Low, a record detail (ADV-M65H-1): it holds nothing. No Critical, High or Medium. Risk tier 1.

## 2. Findings (CLAUDE.md's scale)
| Id | Severity | What | Fix |
|---|---|---|---|
| ADV-M65H-1 | Low | M65-4 asked for the bar lines word for word "on the card's done line and in the day's message". The card's done line (:157) holds every bar number in short form ("repo map OK 186/110", "checkpoint tests 33/33", "refs 1848 = 1825 + 23"…); the literal lines are in .gate/main-65/step5-outputs.txt and the two logs. The day message's addendum (msg-2026-11-25.md:88-89) gives the results in plain words only ("The gate, the repo map, the checkpoint tests and the light run all pass; CI … is green"). | None needed. That message is sent; the card, step5-outputs.txt and this file hold the numbers. Tomorrow's message may point to the card's line in one clause. No push for it. |
| ADV-M65-5 | Low (on record) | Main's own 3a row still reads "none yet … no pull request yet" (82bd694:docs/STATUS.md:25), although #65 has merged. It is main's text, taken word for word, which is right for a merge. | Main's next docs pass |
| ADV-M60-7 | Low (open) | The #40 row says "main 53272a1 merged in" (82bd694:docs/STATUS.md:24): the branch's own row, kept word for word. | PR #40's closing docs pass, as ruled |
| ADV-M54-8 | Low (open) | CLAUDE.md's Never line lists no plain `git merge origin/main`. | The kit follow-up after the freeze, as ruled |

Section 4 (e) of my plan check stops the lane only for a step-7 finding above Low. None is above Low.

## 3. The conditions, each with its evidence (section 7 has the commands)
- **M65-1 (no save from step 2 to step 6's card save): met.**
  - The branch's own line from H0 to the pushed head is the merge and the card save only (`git rev-list --first-parent`).
  - The remote-tracking reflog moves 473f5c1 (23:20:51) to 82bd694 (23:25:37), never to 39a0d04 alone. GitHub's own
    push record for the branch (the activity API) agrees: push 473f5c1 to 82bd694 at 23:25:37. Each push starts where
    the last one ended, so there was no force push.
- **M65-2 (this take-in's numbers): met.**
  - Names: the merge's 64 equal main's 64. The whole line is "64 files changed, 4677 insertions(+), 317 deletions(-)".
    Without STATUS.md, both pairs read the 63-file line.
  - The anchor (tree 9475d37) exits 0. STATUS.md: "1 1" against main, 120 lines, and the #40 row equals H0's byte for
    byte: `git diff -U0 H0 39a0d04 -- docs/STATUS.md` changes no line that begins with `| #40`.
  - Trees: 1,825 files at H0 and 1,848 at the merge (refs = N0 + 23).
  - The light runs: equal word for word except HEAD, times and free disk.
  - Step 5: gate PASS, repo map 186/110, checkpoint tests 33/33.
  - The resume line at H0 (card :157) is C5 (a)'s text word for word. Main's push runs 45503161434 and 45503161333:
    success, attempt 1, on 28254a7.
- **M65-3 (the card at 200 lines or fewer): met.** The card has 200 lines at H0 and at 82bd694 (counted against the
  empty tree), and 200 in the main folder now (`wc -l`), with one MAIN-65 done line.
- **M65-4 (text): met, apart from ADV-M65H-1.**
  - Risk tier 1 is named (the plan's "As approved").
  - step5-outputs.txt records `wc -l` 120 before the add and "1 1" after it.
  - .gate/main-65/ is kept (light-base.log, light.log, step5-outputs.txt).
  - Hand saves followed step 6: 23:28 and 23:33, then each step to 00:50, and 04:02 once agents ran from 03:58.
- **M65-5 (U6c's order): met.**
  - U6c's merge started at 00:20 (e83d9fb), after CI on 82bd694 had completed with success (to 23:28:34).
  - The pull-request runs on 82bd694 prove the conflict had cleared (section 4 (a)), and PR #40 now reads mergeable
    true. No mergeable reading of 00:20 is on the card.
  - The three agents started inside the 03:50 reset's window (03:58).
- **Section 4 (d), this check's terms: met.** I read H0, the merge and the card save by SHA. Every later commit touches
  only .checkpoints/, and none touches spikes/ or docs/design/. The three reruns ran in the main folder, as the
  precondition allows. Toolchain matches main's job 138831415745.

## 4. What I checked myself (one line each)
- `node tools/gate.mjs --no-smoke` in the main folder: GATE PASS (base 28254a7; symbols +0 -0; 5a skipped).
- `node .claude/scripts/repo-map-check.mjs`: "repo map OK: 186 entries cover 110 required paths".
- `node --test .claude/scripts/checkpoint.test.mjs`: 33 tests, 33 pass, 0 fail, 0 skipped.
- Merge 39a0d04 by SHA: its parents, git's own message, "# Conflicts: docs/STATUS.md", no attribution lines, and a
  signature. 82bd694 is the card only, with both trailers.
- The whole tree, spikes/, docs/ (six names), the kit pathspec, the manifests and .gitignore, file modes (no link, no
  submodule), and no conflict marker.
- CI on 82bd694: 6 check runs, all success; the 4 runs read by id; toolchain's 38 + 5 + 1 + 1 summary lines equal
  main's, push and pull request.
- origin/main is still 28254a7 (ls-remote), so no new take-in is due. PR #40: draft, mergeable true, base 28254a7.

## 5. What I did not check
- **Steps recorded only in the chat:** the merge's own output and exit code, the Edits of STATUS.md, the staged tally
  before the commit, and whether any call asked. The committed results match each one.
- **The no-save window** is proved by pushes, from this folder's remote-tracking reflog and GitHub's push record. A
  local commit that was never pushed would not show in either. The branch's first-parent line rules out such a commit
  between the merge and the card save.
- **Not run (the brief says read only):** no light run, check.mjs, cargo or smoke run; smoke is CI's (green, both
  events). spikes/ is unchanged in every commit since H0.
- **Not checked:** the signature's key. I read no CI artifacts.
- **How the exits were read:** only the item 4 precondition printed its exit code. The anchor, the whole-tree check,
  spikes/ and docs/design/ against H0, and the working tree's docs/ were read as no output with no error from
  `git diff --quiet`. Output backs the whole-tree claim: every one of the 365 lines of `git diff --raw --no-renames
  28254a7 82bd694` outside .checkpoints/ is under docs/ (six names) or spikes/.
- **The brief's form:** two of my Bash calls were not one simple command. The first ran sha256sum and a git chain
  together, and the precondition added `; echo "exit $?"`. Both only read, and their output stands, so nothing needs
  redoing.
- **Not mine:** U6c's applied, uncommitted spikes/ files (12 modified, 12 new): they change no commit, and they were in
  the working tree during my reruns. The gate reads legacy/ only. repo-map-check.mjs:22-33 requires only `spikes/` for
  them. The checkpoint tests build their own repositories.

## 6. Decisions needed / For Mustafa
**Decisions needed:** none. The take-in is under his standing MAIN-52 rule. It touches no pass mark, judge,
marks.json, money or main.
**For Mustafa:** nothing.

## 7. Notes (one line per check, kept as written)
- **Item 4 reruns DONE at 04:03:41 UTC (U6c's counted run may start).** Precondition `git diff --quiet 39a0d04 HEAD --
  . ':(exclude)spikes/' ':(exclude).checkpoints/' ':(exclude)docs/design/'`: exit 0, so all three ran in the main
  folder, each alone. `node tools/gate.mjs --no-smoke`: GATE PASS (base 28254a7; 1-4, 5b-5d PASS; symbols +0 -0; 5a
  skipped). `node .claude/scripts/repo-map-check.mjs`: "repo map OK: 186 entries cover 110 required paths" (exit 0).
  `node --test .claude/scripts/checkpoint.test.mjs`: 33 tests, 33 pass, 0 fail, 0 skipped. U6c's untracked spikes/
  files cannot move the map's count: repo-map-check.mjs:22-33 requires only `spikes/` at depth 1 for them.
- Item 2: HEAD moved to 610e412 during this check (a save, 04:02:43 push; it took this file's plan).
  `git rev-list --parents 39a0d04..HEAD`: a straight line of 10, one parent each: 82bd694, daf0897, 7e95ab7, 9923ba2,
  e83d9fb, 5e6711e, 836b4bc, ed3b636, bae6232, 610e412. `git show --stat` of each: .checkpoints/ only (the card,
  history.md, msg-2026-11-25.md, the step-7 and M4b requests, nb2-merge-plan.md and its request, the three advisor
  files). `git diff --name-only 39a0d04 HEAD`: 10 names, all under .checkpoints/netcode-proof-test. No spikes/ or
  docs/design/ commit yet (U6c's files are uncommitted).
- Item 1, the commits. `git rev-list --first-parent --parents 473f5c1..82bd694`: 82bd694 (parent 39a0d04) and 39a0d04
  (parents 473f5c1 and 28254a7) only; the plain range also lists main's 52 commits 1ec6f6c..28254a7 through the second
  parent. Times: H0 23:20:49, merge 23:23:20, card save 23:25:35 UTC. The remote-tracking reflog: 473f5c1 (23:20:51)
  then 82bd694 (23:25:37), never 39a0d04 alone: no save or push between step 2 and step 6 (M65-1 met).
  `git cat-file -p 39a0d04`: git's own message "Merge remote-tracking branch 'origin/main' into
  claude/netcode-proof-test-plan-hcky5g" with "# Conflicts: docs/STATUS.md", no attribution lines, an SSH signature
  (ADV-M60H-5's form). 82bd694: the card only (14 lines, +7/-7), both trailers, no [skip ci], unsigned (a save).
- Item 1, section 3's numbers by SHA (H0 473f5c1, merge 39a0d04, pushed 82bd694, main 28254a7, base 1ec6f6c):
  - `git diff --name-status H0 39a0d04` = `git diff --name-status 1ec6f6c 28254a7`, line for line: 64 paths, 23 A,
    41 M, no D or R.
  - shortstat H0..39a0d04 "64 files changed, 4677 insertions(+), 317 deletions(-)" (the expected whole line); without
    docs/STATUS.md both pairs "63 files changed, 4644 insertions(+), 284 deletions(-)"; STATUS.md numstat 33/33.
  - The anchor `git diff --quiet 9475d37 39a0d04 -- . ':(exclude).checkpoints/' ':(exclude)docs/STATUS.md'`: exit 0.
  - STATUS.md: `git diff 28254a7 39a0d04` is one line out (main's #40 row, the 2026-11-14 text) and one in (H0's #40
    row, the 2026-11-15 text, the same text as `git diff 1ec6f6c H0`'s + line); 120 lines (`wc -l`, the working
    tree's docs/ equal to 39a0d04's: `git diff --quiet 39a0d04 -- docs/` exit 0). The "none yet" 3a row is main's.
  - Whole tree: `git diff --quiet 28254a7 82bd694 -- . ':(exclude)spikes/' ':(exclude).checkpoints/' ':(exclude)docs/'`
    exit 0; docs/ differs in the six names (DECISIONS, REPO_MAP, STATUS, STATUS_ARCHIVE, design/netcode-proof-messages,
    design/ws-reconnect); `git diff --quiet H0 82bd694 -- spikes/ docs/design/` exit 0.
  - Kit: the guard's full pathspec (main-61-plan.md:37-40) on 28254a7..82bd694 prints nothing; manifests, lockfiles
    and .gitignore equal main's (`git diff --stat` empty). `git diff --raw --no-renames 28254a7 82bd694` outside
    .checkpoints/: 365 lines, one 100755 (spikes/netcode-proof/results/u3c-kernel-check/run.sh, old), no 120000 or
    160000, no deletion. No conflict marker outside .checkpoints/ (`git grep` for `<<<<<<<`/`>>>>>>>`: none).
  - Trees: H0 1,825 files, 39a0d04 1,848 (diff against the empty tree) = N0 + 23.
- Item 1, the record. light-base.log: lock "idle after 0 s", HEAD 473f5c1, both hashes e3b0c442…, refs 1825, marks
  81 (drafts 626df678…, rows unchanged), self 78/78, redact 11/11 with 359 items and 0 findings, exit 0. light.log:
  HEAD 39a0d04, the same hashes, refs 1848, marks, self and redact word for word the base's, exit 0.
  step5-outputs.txt: GATE PASS (base 28254a7, +0 -0), repo map 186/110, checkpoint tests 33/33, and step 3-4's checks.
  The card at 82bd694: 200 lines (M65-3 met), the MAIN-65 record line; today's card :157 is the one done line, with
  CI's four runs.
- Item 3, CI on 82bd694 (gh api, by SHA): 6 check runs, all completed with success, 23:25:42-23:28:33 UTC: push run
  45509875991 (gate 138854521336, smoke 138854522059), push run 45509875989 (toolchain 138854521153), PR run
  45509881420 (gate 138854540757, smoke 138854541190), PR run 45509881180 (toolchain 138854539712). Each run by id:
  event push or pull_request as named, branch claude/netcode-proof-test-plan-hcky5g, head_sha 82bd694, attempt 1,
  completed, success. PR #40 now: open, draft, mergeable true, "blocked", base 28254a7, head 610e412 (notes-only since).
- Item 3, toolchain against main's run 45503161333, job 138831415745 (job logs, read as data): main's, the push job's
  and the PR job's each hold 38 `test result: ok.` lines with the same passed counts in the same order (15, 0, 3, 2, 2,
  1, 15, 5, 1, 9, six 0s; the same 16 again; then 2, 15, 5, 1, 0, 0), all "0 failed; 0 ignored; 0 measured; 0 filtered
  out"; 5 `Test Files` lines (7/70, 1/1, 7/42, 2/7, 2/17), the "1 passed" line and Playwright's "14 passed". No
  FAILED, failed, ignored, filtered, skipped, flaky or ##[error] line in any of the three. (MAIN-64's baseline had 28
  and "12 passed": 3a's new tests are counted on both sides here.)
- Section 3 rest: `git merge-base H0 28254a7` = 1ec6f6c. ls-remote at 04:0x: main 28254a7 (unmoved), branch 610e412.
  Main's runs 45503161434 (gate) and 45503161333 (toolchain): push, main, 28254a7, completed, success, attempt 1.
- C5 (a): H0's card :157 "**RESUME (MAIN-65):**" + C5 (a)'s text word for word; the card has 200 lines at H0 and at
  82bd694 (numstat against the empty tree).
- M65-4: msg-2026-11-25.md's addendum (:88-89) names the results without the bar lines (ADV-M65H-1); the card's :157
  has them. Saves after 82bd694 (reflog): 23:28:59, 23:33:08, 00:12:36, 00:20:02, 00:23:26, 00:39:38, 00:47:48,
  00:50:20, 04:02:43.
- M65-5: e83d9fb (00:20) starts U6c's merge, after CI's last run ended 23:28:34; no mergeable reading on the card at
  e83d9fb (`git grep -i mergeable`: none); the pull-request runs on 82bd694 stand for it (section 4 (a)).
- `git status --short` (whole): only .checkpoints/ (this file, advisor-u6c-m4b.md) and spikes/ (12 M, 12 ??), as the
  brief says.
- GitHub's push record (`gh api repos/{owner}/{repo}/activity?ref=refs/heads/claude/netcode-proof-test-plan-hcky5g`):
  ... 462cbbb->473f5c1 23:20:51, 473f5c1->82bd694 23:25:37, 82bd694->daf0897 23:28:59, ... bae6232->610e412 04:02:42;
  every push's "before" is the last one's "after" (no force push); 39a0d04 was never a pushed tip.
- Final re-reads (04:1x): `git rev-parse HEAD` 610e41242e8aef03e96e061ab2a7fe5584a13ce4 (unmoved since 04:02);
  `git diff -U0 H0 39a0d04 -- docs/STATUS.md`: hunks at :5, :13, :18-19, :25 (#64 and "#63, #62" out, "none yet" in),
  :28-36, :39-42, :47, :98-108, :116-118; none touches the #40 row (:24). Card `wc -l` 200. The raw listing
  (365 lines outside .checkpoints/): 365 under docs/ or spikes/, docs/ = the six names.

DONE
