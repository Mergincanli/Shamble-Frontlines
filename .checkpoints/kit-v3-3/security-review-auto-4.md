Automated security review of commit 2acfbbc (the step 8 fix round), 2026-11-05 ~11:10 UTC: three findings, all in
the day-session code of .claude/hooks/routine-guard.mjs. For the fix pass after the delta review.

9. MEDIUM, SA-6's folder check reads git's index only. A folder target asks when `git ls-files -z -- <dir>` names a
   kit path, so untracked kit files, a nested repository, or a worktree inside the target (for example a worktree
   under .gate/ with its own .claude/) are missed: `rm -rf .gate` gets no ask. Fix: also list worktrees
   (`git worktree list --porcelain`) and ask when any worktree path is inside the target; ask when the target holds a
   .git entry below it; or walk the folder for protected basenames (CLAUDE.md, .claude, .mcp.json, .npmrc) to a
   bounded depth and count, asking when the bound is reached.
10. MEDIUM, a variable name the guard cannot read. SA-8's check matches GIT_DIR, GIT_WORK_TREE, GIT_COMMON_DIR and
   GIT_INDEX_FILE in the raw text or in a word with quotes removed, but a name built from an expansion (for example
   `export "${n}IR=..."`, `declare -n`, `printf -v`, read, mapfile, eval, source, a substitution in an assignment word)
   passes. Fix: ask on a day line with a git command that can also set a variable whose name the guard cannot read.
11. MEDIUM, git clone's value-option list is incomplete. CLONE_VALUE lacks --revision (and any future option), so
   writtenBy can take the wrong word as the clone's folder. Fix: fail closed. Ask when git clone has any option outside
   a known set, or check the cloneDir of every positional word; add --revision.
The pattern again: each parser rule has a differential. Prefer rules that fail toward the ask when unsure.
DONE
