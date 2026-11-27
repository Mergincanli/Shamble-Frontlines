# MAIN-69: take main b8f02b0 (#78 ci-time) into the lane: the plan (rev 0.2, as checked)
Status: AS CHECKED 14:19 UTC: APPROVE WITH CONDITIONS M69-1 to M69-6, all Low, for main's tip
b8f02b0149cc928964e54e194cd9c4d68f52bd94 (advisor-main-69.md, last line "Status: COMPLETE 2026-11-27T14:19Z"; it checked
rev 0.1, sha256 3c54354c…dbb). The conditions stand below word for word and govern wherever the steps differ. The
answers: Q1 yes (the light run, no whole check.mjs run); Q2 yes (no guard tests); Q3 keep the plain `git merge
origin/main`, the checks after git's own commit (`--no-commit` would make the guard ask him: routine-guard.mjs:1425-1428
passes only the plain and `--ff-only` forms); Q4 yes, step 7 may ride B's or NB's seat (both advisor agents), never D;
Q5 no screenshot. On record: ADV-M69-0 (C's earlier `--quiet` reads re-read by `--name-only`, all hold) and ADV-M69-7
(the light recipe's `--quiet` bars become `--name-only` when its conditions are next applied).
Rev 0.1 was the main session's draft (14:1x UTC), checked by C a2b7636a57dff4206. A delta on main-68-plan.md rev 4: its
steps, forms and stop rules hold except where this file differs.

## As checked: M69-1 to M69-6 (advisor-main-69.md, "Conditions (paste-ready)"), word for word
- **M69-1 (Low): the whole-tree bars, at step 5 after the content checks, each alone, in printing forms:**
  - `git diff --name-only origin/main HEAD -- . ':(exclude)spikes/' ':(exclude).checkpoints/' ':(exclude)docs/'`
    prints nothing;
  - `git diff --name-only origin/main HEAD -- docs/` prints the usual six names (DECISIONS.md, REPO_MAP.md, STATUS.md,
    STATUS_ARCHIVE.md, design/netcode-proof-messages.md, design/ws-reconnect.md);
  - `git diff --name-only H0 HEAD -- spikes/ docs/design/ .checkpoints/` prints nothing.

  These are MAIN-66's C1 (c). They prove "PR #40's diff against main holds no .github/ path".
- **M69-2 (Low): step 7's baseline.** It is main's toolchain run 46011355903, job 140547396571, compared line for line as
  M67-3 gives it (the `test result:` lines, the vitest `Test Files` and `Tests` lines, every Playwright "passed" line).
  Its lines should also equal job 140114848934's: #78 changes only the time limit. #74's two pull-request steps keep rev
  4's step-6 bar. Record the toolchain job's run time beside the 25-minute limit.
- **M69-3 (Low): a move of main found at step 1.** P-2 and TAKEIN-CODE (b) apply with "the same conflicts" read as
  none: step 1's anchor bar (one tree line, no CONFLICT line) must still hold. A move that brings any conflict stops
  before step 2 for a check.
  - Under TAKEIN-CODE (b), "reviewed and merged on main" is read from the merge's message or report: it names the
    advisor agent's approval of the merged head and his go. Without that, the move stops before step 2.
  - Each move taken in by (b) or by P-2 (a) gets one line to him naming its pull requests. The advisor's read of the
    pushed head stays.
- **M69-4 (Low): the resume line (C5 (a)), word for word.**

  "MAIN-69 approved (advisor-main-69.md), running steps 1-7 for main b8f02b0149cc928964e54e194cd9c4d68f52bd94; usage at
  step 1: <figure>, <go or no-go>. If `ls -d /home/user/Shamble-Frontlines/.git/MERGE_HEAD` lists the file, a merge
  stopped in progress: no save, no `git merge --abort`, no reset; the advisor rules. If `git rev-parse HEAD^2` prints
  b8f02b0149cc928964e54e194cd9c4d68f52bd94 and `git ls-remote origin refs/heads/claude/netcode-proof-test-plan-hcky5g`
  prints another SHA than `git rev-parse HEAD`, resume at step 5. advisor-main-69.md and this card may show unstaged:
  leave them; step 6's card save takes them. Anything else: the advisor rules."
- **M69-5 (Low): no `--quiet` bar.** Every bar this take-in inherits from rev 4 in a `--quiet` form runs as
  `--name-only` and must print nothing. This means step 6's DEP-HOLD line (C68B-3), in its own call: `git diff
  --name-only origin/main HEAD -- Cargo.lock Cargo.toml crates packages package.json package-lock.json` prints nothing.
  Its control is step 5's six-path line.
- **M69-6 (Low): "Why" cites his words.** history.md:2055-2057, word for word there: "Take main in when main changes
  code, CI or rules". The main session's reading at :2082-2086 may stand beside them.
- Added by the main session after the check (the advisor tool, 14:1x): both workflows cancel a running run on a new push
  to the same ref (gate.yml:15-17, toolchain.yml:23-25), so step 6's save waits until the four CI runs of the follow-up
  F1 (operator-rewordings.md, its last section), if pushed first, read completed; F1's commit is named under H0.

## Why
- His words (MAIN-52, history.md:2055-2057, word for word there): "Take main in when main changes code, CI or rules".
  The main session's reading at :2082-2086 stands beside them. #78 changes .github/workflows/toolchain.yml, a CI file.
- Neither lighter form fits. TAKEIN-CODE (b) (his 13:55 UTC message, history.md:8168; the conditions of
  msg-2026-11-27.md:71-77) is for a code move between a checked plan and its merge, and its list leaves CI out. The
  light recipe covers "only a move of main whose new merges touch only docs .md files" (light-takein-recipe.md:9-10).
  So the full form: plan first, its check.
- UM-4's step 0 pins main (um4-plan.md:66: "a code, CI or rules move stops: MAIN-52"), so MAIN-69 comes before UM-4,
  and before NB2-ONEWAY's merge (nb2b-merge-plan.md, "Since the draft").

## Facts (read 14:0x-14:1x UTC, read only, printing forms; HEAD then ccbd534)
- `git fetch origin main` (14:0x): "b9aad01..b8f02b0 main -> origin/main". `git log --oneline --first-parent
  b9aad01..b8f02b0`: one line, "b8f02b0 Merge pull request #78: ci-time, the toolchain job's time limit 15 to 25
  minutes". Its parents: b9aad01954be666340a7ec5c6e9fc9353b11abf1 and 312a763d2894a743c64bc186607c3301484c0757 (#78's
  head). Its message (a record of the CI chat, not his word here): "On Mustafa's go of 2026-11-27, 14:45 Amsterdam,
  typed in the CI Builder's chat, for head 312a763d…, merged with that id as the expected head. The advisor agent
  approved that head; gate, smoke and both toolchain runs were green on it".
- `git merge-base HEAD b8f02b0`: b9aad01954be666340a7ec5c6e9fc9353b11abf1 (main as MAIN-68 took it in).
- `git diff --raw --no-renames b9aad01 b8f02b0`: six paths, every new mode 100644 (no link, submodule or mode change):
  - M .github/workflows/toolchain.yml (4f2e88d to b19cadf), `--numstat` 1/1: line 38 `timeout-minutes: 15` becomes
    `timeout-minutes: 25`, nothing else (the diff read 14:0x);
  - M docs/DECISIONS.md 54/0 (his CI-TIME (a) and FULL-PATHS entries of 11-27 and his #74 entry of 11-26, copied by the
    CI chat);
  - M docs/REPO_MAP.md 2/1; M docs/ROADMAP.md 5/0; M docs/STATUS.md 2/2;
  - A docs/reports/2026-11-27-ci-time.md 70/0.
  Total: 6 files, +134/-4.
- None of the six is a manifest, a lockfile, or under crates/, packages/, spikes/ or .claude/, and none is a CLAUDE.md
  or AGENTS.md: no DEP-HOLD, no guard file, no design or look doc.
- `git merge-tree --write-tree --name-only HEAD b8f02b0` (at HEAD d765947, 14:0x): one line, tree
  b231cc144e65d5fbd8a667052e59a64ccc23b6f7, no CONFLICT line. docs/STATUS.md merges by itself (this branch's #40 row
  and main's #78 lines do not touch).
- Main's CI on b8f02b0, its push runs, attempt 1, both success: gate 46011351316 (13:55:51-13:57:01) and toolchain
  46011355903 (13:55:53-14:03:27).
- The refs step (spikes/netcode-proof/check.mjs:2695-2700) counts `git ls-files` and, by referenceProblems
  (check.mjs:1880-1894), reads only files outside spikes/, docs/ and .checkpoints/ that are not .md, for "spikes/". Of
  the six paths it reads only toolchain.yml, where #78 changes one number. So step 5d's refs = N0 + 1 (one added
  path). The marks, self and redact steps read none of the six.

## Steps (main-68-plan.md rev 4's steps 1-7; only what differs; each command its own call, printing forms)
1. As MAIN-68's, with these values:
   - The resume line by Edit, C5 (a)'s form for b8f02b0149cc928964e54e194cd9c4d68f52bd94.
   - Usage (his 13:55 item 1, history.md:8167): the session's rate-limit event read (utilization, resetsAt). Steps
     1-6 start only if the usage left covers them; the figure and the go or no-go on the resume line.
   - The saver stopped by TaskStop, then `pgrep -c -f checkpoint.mjs` printing 0.
   - H0 named in the chat. `git fetch origin` in its own call, then C4: origin/main must be
     b8f02b0149cc928964e54e194cd9c4d68f52bd94, its CI as in Facts. A further move: classified by P-2
     (advisor-main-68b.md section 3) as widened by TAKEIN-CODE (b); anything outside both stops for a new check.
   - The anchor T, alone: `git merge-tree --write-tree --name-only H0 origin/main`. The bar: exit 0, one tree line, no
     CONFLICT line; anything else stops before step 2. T goes into this file by Edit before step 2.
   - `mkdir -p /home/user/Shamble-Frontlines/.gate/main-69`, then the base light run: `bash
     .checkpoints/netcode-proof-test/main-run.sh /home/user/Shamble-Frontlines/.gate/main-69/light-base.log node
     check.mjs --only refs,marks,self,redact` (run_in_background, timeout 7200000), giving N0, M0 and the self count.
2. Named in the chat first: `git merge origin/main`. It writes one line into .github/workflows/toolchain.yml and five
   docs paths.
   - Before it, in its own call: `git rev-parse origin/main` prints b8f02b0149cc928964e54e194cd9c4d68f52bd94;
     otherwise stop before step 2.
   - Refused before git runs: as MAIN-68's step 2 (prove nothing changed, try no other form, restart the saver, tell
     him the one command).
   - Expected: exit 0 and git's own merge commit at once (no conflict; no editor opens, as the tool has no terminal),
     with git's summary "6 files changed, 134 insertions(+), 4 deletions(-)".
3-4. Moot: no conflict to resolve, and git commits in step 2. If step 2 stops at a conflict instead: stop there,
   resolve nothing, no `git merge --abort`, no reset; the advisor rules (C5 (a)'s merge-in-progress line).
5. a-c as MAIN-68's: GATE PASS (`node tools/gate.mjs --no-smoke`); "repo map OK" with its two numbers (#78 changes
   REPO_MAP.md); the checkpoint tests pass, 0 fail, the count recorded (#78 changes no .claude/scripts/ file). Then the
   content checks, each alone:
   - `git rev-parse HEAD^1 HEAD^2` prints H0, then b8f02b0149cc928964e54e194cd9c4d68f52bd94.
   - `git diff --name-only H0 HEAD` prints exactly the six paths of Facts; `git diff --stat H0 HEAD`'s last line equals
     `git diff --stat b9aad01 b8f02b0`'s, "6 files changed, 134 insertions(+), 4 deletions(-)".
   - `git diff --name-only T HEAD` prints nothing (the anchor; its control is the previous line printing six).
   - d: the light run (run_in_background) into /home/user/Shamble-Frontlines/.gate/main-69/light.log: every count
     equals the base run's word for word, except HEAD, and refs = N0 + 1.
   - e (MAIN-68's guard tests): not run, as #78 changes no .claude/ file (Q2).
6. As MAIN-68's: main's CI on b8f02b0 read again; `wc -l` on the card first (200 or fewer); the card save pushes the
   merge; CI on the pushed head, push and pull request: gate (gate, smoke) and toolchain green, the toolchain job now
   with a 25-minute limit (its run time recorded); the saver restarts, both trailers.
7. The advisor's check of the pushed head by SHA (MAIN-65 step 7's form), with no window wait (his 13:55 item 1): C's
   seat, or it rides an agent then running.
- One line to him (MAIN-52's record, for the chat and the day message): "main b8f02b0 taken in (#78 ci-time: the
  toolchain job's time limit 15 to 25 minutes, and docs), plan first (MAIN-69)."

## Order with the rest of the day
- 14:09: D, B and NB run (launch-1550-msgs.md). C takes the first seat that frees.
- D's keep-first and its worktree remove (main-session git on .git/worktrees) run before MAIN-69's step 1 or after its
  step 6, never between them.
- After C's verdict, its conditions applied by Edit: steps 1-6 when no other lane step runs.
- Then UM-4: its step 0 then reads origin/main b8f02b0, taken in by MAIN-69 (um4-plan.md:66's value re-read by Edit and
  named on its resume line). Then NB2-ONEWAY's merge. NB's answer to nb2b-merge-plan.md's Q4 may swap these two.

## Questions for the checker
- Q1: is step 5 enough as the gate, the repo map check, the checkpoint tests and the light run, with no whole check.mjs
  run, since no code, manifest, lockfile, spikes/ or .claude/ path changes?
- Q2: may step 5e (main's guard tests) be left out, as #78 changes no .claude/ file?
- Q3: with a clean merge, git commits in step 2 and the content checks come after the commit (the plain form, MAIN-68's
  step 2 command). Right? Or should step 2 be `git merge --no-commit origin/main`, so the checks run before the commit,
  and is that form inside main's guard?
- Q4: may step 7 ride B's or NB's seat if one is resumed later, or must it be C?
- Q5: main's MAIN-WALL bullet asks him for a screenshot of both rulesets at the start of any kit, CI or gate
  change-set. MAIN-68's Q5 read a take-in as not such a change-set. The same here?

## Change type, tier, size
A merge of main's reviewed content: 6 files, +134/-4, one pull request (#78, risk tier 2 on main; by its merge message
and docs/DECISIONS.md the advisor agent approved its head 312a763d, with security-auditor and infra-engineer, and it
merged on his go), with no conflict and nothing edited by hand. After the merge PR #40's diff against main holds no
.github/ path, so PR #40 stays tier 1. Reviewers: the advisor (this plan, then the pushed head). No Brave check.
