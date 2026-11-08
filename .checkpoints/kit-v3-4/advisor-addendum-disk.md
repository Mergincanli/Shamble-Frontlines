# Advisor verdict: A4 addendum, disk hygiene as CHECKPOINTS.md §10 (2026-11-08)
**REVISE** (two exact text changes; then approved without another round). Placement as §10, his quote and the no-rule-change line are right: §1-§9 untouched (main's file = kb1/checkpoints-md.txt, diff adds only lines 209-218).
Checked on main's guard (ran .claude/hooks/guard.mjs per seat): `git worktree remove <path>` day: allowed (settings allow `git worktree *`), night-shift: allowed, pr-advisor: refused ("only reads"). `--force` refused in every seat (routine-guard.mjs:1023, settings deny :214-216). `cargo` refused in both routines (R-PROG, :554); day: no allow entry, so it prompts. `rm -rf` denied (settings :247; routines R-PROG).
Problem: CHECKPOINTS.md line 7 says its rules hold for every session, PR Advisor included, so §10 as drafted promises steps the guard refuses. Second, "dropped on purpose" adds a path his words don't hold (the only ways are --force, rm -rf or git clean: all refused).
1. Insert after the "His words" quote, as the first bullet:
   `- Who runs it: day sessions. The night shift may remove merged copies but cannot run cargo; the PR Advisor runs neither (the guard refuses both). A routine that finds a copy to clean notes it for a day session.`
   (If the last sentence reads as a new duty, drop it; the first two sentences are the condition.)
2. Replace "Git\n  refuses a copy that still has unsaved work, and that copy stays until its work is saved or dropped on purpose." with:
   `Git refuses a copy that still has unsaved work; that copy stays until its work is saved.`
Note (no change required): `target/` is not in .gitignore, so a build folder inside a copy counts as unsaved work and blocks the remove; the bullet order (clean first, then remove) already handles it.
Accepted as is: "so each copy does not rebuild the same crates" (a reason, not a rule; under "To consider").
Evidence and review: E1 quotes both files word for word; E2 row for §10 reads "guidance" (no allow/deny/ask change; settings.json and the guard untouched); the section joins the A4 review round (security-auditor, verifier, infra-engineer), and his go comes in the kit sitting as the fifth prompt.
