# Advisor: disk deadlock before the counted check of 2bd44a5 (2026-11-15)

## Plan
1. Read main's advisor.md (done: git show origin/main:.claude/agents/advisor.md).
2. Confirm the state: HEAD 2bd44a5, unpushed; free space; sizes of what holds space.
3. Read the rule on 2bd44a5: .claude/CHECKPOINTS.md section 11, kept-logs.mjs (the `cache` subcommand: what it deletes, its guards), main-run.sh (the 4 GB floor).
4. Read his words: KL-YES (history.md:2131-2146), KL-MERGE (history.md:2163-2175), LEFTOVER-DELETE.
5. Read my REFS-KIT head-check notes (step order) and u4r-disk-plan.md section 5 / W1-TARGET item 3.
6. Weigh options 1-3; rule; say what is recorded and reported.

## Notes
- 20:58 UTC: HEAD 2bd44a5, branch ahead 82 of its remote; df free 2,866 MiB (matches the brief).
- Rule on 2bd44a5: kept-logs.mjs and CHECKPOINTS.md equal origin/main 53272a1 (git diff empty). §11: cache only by `kept-logs.mjs cache`, main session, main folder, never while a run set or build runs; free before/after to the card and the report. Script's cache(): refuses tracked files, a worktree inside, anything newer than 07:20 UTC; prints free before/after.
- Read-only checks now: find -newermt '2026-11-15 07:20 UTC' in .gate/u3c-review-target = 0 files; no worktree in it; pgrep cargo, rustc, npt-lab, npt-judge, loader.mjs, check.mjs, main-run all empty.
- His words (history.md 2131-2175): KL-YES/KL-CHANGE-YES "Nothing real is packed or deleted before that pull request is on main and this branch has taken main in"; KL-MERGE "After the merge this branch takes main in. Only then may anything real be packed. Report what was packed, with the free space before and after." Both conditions hold at 2bd44a5 (PR #53 on main; the branch holds main). The remote branch does not hold it yet: pushing first makes the condition true on both readings.
- My own earlier conditions (refs-clash order step 3; refs-kit head conditions): "No kept-logs list or pack before that merged head is checked". The reason was list's commit under spikes/ (the refs exception proven on the real tree first) and pack needing the list pushed. `cache` touches no spikes/ path and commits nothing: not covered by that reason. The cache removal before the check is my change of order, said openly in the report.
- main-run.sh :38-47: floor = 4*1024^3 bytes (4,096 MiB) on /home/user; then `find "$S/crates" -type f -exec touch {} +` (:48). So the workspace crates rebuild every counted run; only dependencies are warm. The brief's "the cache is warm" holds for dependencies only.
- Numbers: after cache, free about 2,866 + 4,051 = 6,917 MiB (2.8 GB above the floor). The 20:34 run grew the shared target by about 4.76 GB from near-cold; a run on the filled target rebuilds workspace crates in place.
- (2) rejected: the shared target is check.mjs's live warm cache (u4r-disk-plan.md §5: "A clean there would be refilled ... by the next check.mjs run"); a part-clean frees space the run takes straight back, leaves free near today's 2.9 GB for the next run, and slows every later run. W1-TARGET item 3 is for a cache nothing still uses.
- (3) rejected: outside main-run.sh there is no lane lock, no floor, no crate touch and no diff-hash line, so it is not the counted run my head check asked for (914798a's counted run was proven by that hash); with 2.9 GB free and workspace crates rebuilding, a full disk mid-run could hit git objects and the card.
- 2bd44a5 tree: 1,273 tracked files (914798a: 1,268); the merge changes no spikes/ or crates/ path (git diff --name-only 98ffb08 2bd44a5: .claude/CHECKPOINTS.md, the two kept-logs scripts, docs only). So the counted run's refs line must read 1,273 files with the §4.9 line, and its diff hash must be the empty diff.
- Ruling: (1), with the push of the merge first (by the card's checkpoint save) and a stop on any refusal.
