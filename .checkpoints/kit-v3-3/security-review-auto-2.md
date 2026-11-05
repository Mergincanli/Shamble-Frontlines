Automated security review of commit 2e82216 (build steps 4b, 6, 7), 2026-11-05 ~02:20 UTC: two findings, both in the
day-session code of .claude/hooks/routine-guard.mjs. For step 4c, each with a test.

5. MEDIUM, CDPATH set in the same command line. dayCd treats a bare relative cd target as lost only when the guard's
   own environment has CDPATH (`io.env.CDPATH`). A line that sets CDPATH itself is not covered: a `CDPATH=...` prefix
   on the cd or pushd word, or `export CDPATH=...` / `declare CDPATH` / a bare `CDPATH=...` earlier in the line. A bare
   relative target could then land in the main folder while the guard thinks it went elsewhere. Fix: treat any
   CDPATH assignment anywhere in the line like an inherited CDPATH (a line-level flag dayCd checks), or treat every
   bare relative target (not ./ or ../) as lost.
6. MEDIUM, a branch name versus a tag. mainFolderMove resolves a switch target with commitOf(pos[0]), the general
   rev-parse, while `git switch <name>` moves to the branch refs/heads/<name>. A tag (or other ref) with the same
   name resolves first in rev-parse's order, so the guard compares the tag's commit while git switches to the
   branch. Fix: for a switch without --detach, resolve refs/heads/<name> explicitly
   (`rev-parse --verify --end-of-options refs/heads/<name>^{commit}`); if the short name is ambiguous (it also resolves
   as a tag or another ref, or `rev-parse --symbolic-full-name` disagrees), ask. Use the general resolve only for
   --detach.
DONE
