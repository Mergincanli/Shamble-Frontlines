# MAIN-67: take main 07d673b (#71, ci-after-3a) into the lane: the plan, written 2026-11-26 00:0x UTC
MAIN-52 calls for it: #71 changes CI (.github/workflows/toolchain.yml) and code outside spikes/ (packages/e2e), so it is
not his LIGHT-TAKEIN recipe's case (his 20:48 UTC word, history.md's last entry: a full plan). Form: MAIN-66's plan
(main-66-plan.md, as approved with M66-1 to M66-5, advisor-main-66.md), steps 1-7 whole; only the facts and numbers
below differ, and every MAIN-66 value is replaced by its line here. His interim rule holds in every command: git and gh
always start their own command, with no other program in front of them.

## Facts (read 2026-11-25 23:59 to 2026-11-26 00:0x UTC, read only)
- `git fetch origin main`: c1da1bd..07d673b0ee509d36ce1b976ed8dfae002c7d7b4a, one first-parent merge: #71 (merged
  20:49 UTC on his tap, from his main Advisor thread). The merge base with this branch is c1da1bd (MAIN-66's main).
- What it changes (`git diff --name-status c1da1bd origin/main`): 8 paths, A 1 (docs/reports/2026-11-25-ci-after-3a.md),
  M 7 (.github/workflows/toolchain.yml, docs/DECISIONS.md, docs/REPO_MAP.md, docs/ROADMAP.md, docs/STATUS.md,
  docs/STATUS_ARCHIVE.md, packages/e2e/playwright.config.ts). Whole: 8 files, +555/-29; without docs/STATUS.md: "7 files
  changed, 543 insertions(+), 17 deletions(-)". Nothing under spikes/, .claude/, a CLAUDE.md or a manifest (DEP-HOLD
  free); no `| PM-` row in docs/adr/ (no docs/adr/ path at all).
- Main's CI on 07d673b, push runs: gate 45698566018 and toolchain 45698565943, both completed, success.
- `git merge-tree --write-tree --name-only HEAD origin/main` (HEAD then 1219f8a): exit 1, one CONFLICT, docs/STATUS.md;
  DECISIONS.md, REPO_MAP.md and STATUS_ARCHIVE.md merge on their own; tree d19a7ce8 (not bound: re-derived from H0).
- The conflict is the "In flight" block, merged lines 24-31 (`<<<<<<< HEAD` :24, `=======` :28, `>>>>>>> origin/main`
  :31): ours :25 (#40, this branch's row), :26 (#67), :27 (#69); main's :29 (#40, main's older text), :30 (the new
  ci-after-3a row). Main moved #67 and #69 to its archive ("the merged #67 and #69 rows moved to the archive's 'In
  flight' by script"). Main's new STATUS.md has 120 lines.
- The kit: #71 touches no .claude/ file, so the guard's rules stay main's c1da1bd ones (MAIN-66); .github/'s one file
  comes in equal to main's, so after the merge PR #40's diff against main touches no kit file (tier 1 stays).
- The lane: U7's builder (a9873cdba07a3219e) runs on 9177fb5 in its worktree with u7-run.sh; steps 1 and 5d wait for
  its lane lock with no limit (main-run.sh:28-37), as MAIN-66's did.

## The numbers (in place of MAIN-66's)
- C1 (a), (b), 5d: the base run gives every count; 5d equals them word for word, except HEAD, and refs = N0 + 1.
- C1 (c): `git diff --name-only H0 HEAD` equals `git diff --name-only c1da1bd origin/main` (8 names); without
  docs/STATUS.md both pairs read "7 files changed, 543 insertions(+), 17 deletions(-)"; the whole line is expected at
  "8 files changed, 555 insertions(+), 29 deletions(-)" (STATUS.md's 12 / 12 as main's: the swapped #40 row is the same
  on both sides).
- C2 (a): "beyond main's 8"; the allowance as MAIN-66's: a ` M`, ` D` or `??` line under .checkpoints/netcode-proof-test/
  or the card. Expected: the card, this plan, advisor-main-67.md, advisor-main-66-head.md, history.md (if he writes).
- C2 (b): `git diff --cached --shortstat H0 -- . ':(exclude)docs/STATUS.md'` prints the 7-file line; the anchor is
  re-derived at step 1 from H0 (M66-1 (a)'s form) and binds as there.
- C2 (c), C3: "builder 4" is U7's builder on 9177fb5 and u7-run.sh; its hand-back is only read until the push.
- C4: after `git fetch origin` in its own call, origin/main must be 07d673b0ee509d36ce1b976ed8dfae002c7d7b4a; main's CI
  as in the Facts. A further move stops before step 2 (advisor-main-65.md:113-114), unless it fits his LIGHT-TAKEIN
  recipe once that recipe is checked (not yet: it is written after this take-in).
- C5 (a): MAIN-66's resume line with "MAIN-67", advisor-main-67.md and 07d673b0ee509d36ce1b976ed8dfae002c7d7b4a; U7's
  builder works in its own worktree.
- C5 (b): the saver b2zyx9eey stopped by TaskStop at step 1 with its proof; it restarts at step 6.
- C6: keep :25 (ours #40) and :30 (main's ci-after-3a row), in that order; drop :26 (#67), :27 (#69) and :29 (main's
  #40). Then the marker grep and `wc -l docs/STATUS.md`: exactly 120, before `git add docs/STATUS.md`; after the add,
  `git diff --cached --numstat origin/main -- docs/STATUS.md` prints "1 1".
- Step 4: two parents, H0 and 07d673b, git's own message.
- Step 5: a-c and e as MAIN-66's (5e: main's routine-guard tests, any fail stops; #71 changed no guard file, so the
  count is 1,247 again). 5d: the light run as above.
- Step 6: CI on the pushed head: gate (gate, smoke) and toolchain green, push and pull request. Toolchain now runs
  #71's additions (every crate's release tests, the vector generators' `--check`, devtools' wasm32 build line, the rule
  tests in Firefox); none of them reads spikes/.
- Step 7: the advisor's check of the pushed head by SHA, a new agent's work: the next reset's first 30 minutes (04:50
  UTC) unless it rides an agent already running. Its toolchain baseline: main's run 45698565943 (job read at step 1).
- M65-2's fallbacks stand with these values: refs at 5d = H0's tree count + 1, `git diff --name-only --diff-filter=A
  H0 HEAD` listing main's one added name; the staged tally `git diff --cached --name-status H0` = `git diff
  --name-status c1da1bd origin/main` (8 paths, A 1, M 7); STATUS.md's 120 with no tolerance.

## Order with the rest of the night
After this plan's check: steps 1-6 run when the light runs can get the lane (U7's builder holds it through its room
tests and end check). U8's merge (u8-merge-plan.md) follows MAIN-67's push, M0 reading main again.

## Questions for the checker
- Q1: anything in #71 (a CI workflow change, a Playwright config) that needs more than MAIN-66's steps, given that
  nothing of it touches spikes/?
- Q2: may step 7 ride U8's head read in the same window, as MAIN-66's step 7 was meant to?

## Change type, tier, size
A merge of main's reviewed content: no new code of ours; tier 1 after the merge (no kit file differs from main's). The
check: one advisor read of this plan, then step 7's read of the pushed head.
