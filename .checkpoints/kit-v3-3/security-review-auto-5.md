Automated security review of commit ed3ea8c (step 9, the night switch check), 2026-11-05 ~11:25 UTC: two HIGH
findings in switchTargetCheck (.claude/hooks/routine-guard.mjs), a routine path. For the fix pass.

12. HIGH, time of check versus time of use within one command line. The guard resolves the switch target's commit
   before the line runs. A night line like `git branch -f cs/ok <commit> && git switch cs/ok`, or any other command in
   the same line that moves refs (fetch, branch, worktree, commit, push, reset), changes what git switches to after the
   guard checked it. Fix: for the night shift, refuse a git switch that is not the only command in its line (or that
   shares a line with any command that can move refs). The guard cannot rewrite the command to pin the SHA.
13. HIGH, ref resolution through git's general lookup. commitOf(`refs/heads/${name}`) and
   commitOf(`refs/remotes/origin/${name}`) go through rev-parse, which can read a name differently from an exact branch.
   Fix: resolve only exact full refs (`git show-ref --verify --hash refs/heads/<name>`, the same for
   refs/remotes/origin/<name>), and refuse a name that resolves only through git's guessing. Also: newBranchNames must
   check the new name in `git branch -m/-M/-c/-C <old> <new>` (a rename or copy to origin/... or main reopens SA-2).
DONE
