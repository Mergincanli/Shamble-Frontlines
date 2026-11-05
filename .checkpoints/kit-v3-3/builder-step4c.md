TASK: kit-v3-3 builder, step 4c: security review auto-2 findings 5 (CDPATH in the line) and 6 (switch branch vs tag).

Plan (blocks, in order):
C1 read code sites (dayCd, parseShell assignments, mainFolderMove switch, commitOf) + test structure (dayIo, DAY_CASES, DAY_MUTANTS, real-git test)
C2 baseline test counts
C3 finding 5 fix (decide stricter option) + reasoning
C4 finding 6 fix
C5 tests (DAY_CASES rows, dayIo scaffold additions if needed, real-git test for 6)
C6 test run
C7 mutation rows (in-suite DAY_MUTANTS) + scratch-copy mutations under .gate/kit-v3-3/
C8 line refs + answer

Notes:
- C1 done: sites (head 546d5a7): parseShell :119-218, bash() day cd hook :488-491 (dirs :476), dayCd :607-627 (CDPATH :624), commitOf :777-782, switchWords :788-801, mainFolderMove switch :807-828. Tests: dayIo :575-606, DAY_CASES to :755, DAY_MUTANTS :824-866 (rows 864/865 target dayCd :624 text: keep it intact), real-git 4b test :895.
- finding: option "every bare relative target lost" breaks existing case 'S-4 a relative cd into a worktree is followed' (cd .gate/x && git switch cs/other -> null) and would also weaken the write check (a lost vcwd = project folder: `cd .claude && echo > settings.json` would resolve to /p/settings.json, no ask). Decision: line-level flag (option a), made broad: the line names CDPATH in its raw text or any parsed word, or runs a command that can set a variable by an indirect name (eval, source, ., export, declare, typeset, local, readonly, read, mapfile, readarray, printf -v).
- probe (.gate/kit-v3-3/p4c-probe, git 2.43): tag dup at commit one, branch dup at commit two: `rev-parse --verify dup^{commit}` = tag's commit (warning ambiguous); refs/heads/dup^{commit} = branch's; `rev-parse --verify --symbolic-full-name --end-of-options dup` -> error ambiguous, exit 0, empty stdout; for main -> refs/heads/main; for a SHA -> empty exit 0; unknown -> exit 128. Without --verify, --end-of-options is echoed: use --verify.
- design 6: branchCommit(name): symbolic-full-name must equal refs/heads/<name>, then commitOf(refs/heads/<name>); switchWords gets detach (exact --detach / -d); --no-d* -> odd (cannot tell, asks).
- C2 done: baseline routine-guard 375/375, settings 127/127, checkpoint 27/27 = 529/529.
- Next C3.
- C3 done: finding 5: dirs.cdpath = lineSetsCdpath(command, cmds) in bash() (day only); dayCd new line before the existing CDPATH line (existing text kept: rows 864/865); VAR_SETTERS + lineSetsCdpath before dayEdit. Raw-text check /CDPATH|\$\{!/ (bash probe: `: ${!V:=/tmp}` and `declare -n r=$V; r=/tmp` both set CDPATH); per-word CDPATH check dropped (raw covers; a quoted name only works through a setter, which flags anyway).
- C4 done: finding 6: branchCommit() before switchWords; switchWords detach (--detach/-d) and --no-d* -> odd; switch: detach ? commitOf : branchCommit; failed reason adds "(as the branch refs/heads/<name>, a name no tag or other ref shares)" unless detach. Existing tests then: T16d, T19 fail (dayIo answers no symbolic-full-name: scaffold), mutation test fails (same cause).
- Next C5: dayIo scaffold `tags` option + symbolic-full-name + refs/heads answers; cases.
- C5 done: dayIo scaffold (tags option; symbolic-full-name + refs/heads + tag answers; added lines only, one destructuring line gained `tags = []`); DAY_CASES +26 rows (F5 x19, F6 x7); DAY_MUTANTS +13 rows; real-git test "real git (step 4c, finding 6)..." appended. Process slip: that test was first appended with a shell heredoc (cat >>), not the Edit tool; later fix by Edit. Content only appended.
- Next C6 test run.
- correction to C5: DAY_CASES +25 rows (F5 x18, F6 x7), not 26.
- C6 done: routine-guard 401/401 (375 + 25 rows + 1 real-git test), settings 127/127, checkpoint 27/27; the combined command 555/555.
- C7 done: in-suite DAY_MUTANTS +13 rows (46 total), 0 survivors. Scratch copies (.checkpoints/kit-v3-3/mut-step4c.mjs -> mut-step4c.out; copies .gate/kit-v3-3/mut-step4c-0..6), routine-guard.test.mjs:
  [0] unmutated 401/401 · [1] base guard 546d5a7 + new tests 381/401: the 20 failures are exactly the new rows/tests (13 F5 asks, 5 F6 ask/deny rows, the day mutation test, the real-git 4c test); every older test passes on base
  [2] F5 dayCd line off 387/401 (13 F5 asks + mutation test) · [3] F5 flag never set 387/401 (same) · [4] F6 general resolve 395/401 (4 F6 rows, mutation test, real-git) · [5] F6 ambiguity check off 396/401 (3 rows, mutation test, real-git) · [6] both off 382/401
- real-git + real-bash probe (.checkpoints/kit-v3-3/p4c-probe.mjs -> p4c-probe.out; repo .gate/kit-v3-3/p4c-probe2, git 2.43.0; origin/main by remote add + fetch): origin/main f6f9005 = tag dup; branch dup 0fd2e31 (changes .claude/settings.json); short name dup reads f6f9005. Base guard: `git switch dup` goes on (the finding reproduced); step 4c: asks. `--detach dup` and `switch main` go on in both. git switch dup lands on 0fd2e31 (the branch). Bash: `CDPATH=<p>/elsewhere cd w2` and `export CDPATH=...; cd w2` land in elsewhere/w2, `cd ./w2` in here/w2.
- line refs: routine-guard.mjs :477 (flag set), :625-626 (dayCd line flag), :630-646 (VAR_SETTERS, lineSetsCdpath, comment), :807-814 (branchCommit), :815-829 (switchWords detach, --no-d odd), :837, :853-857 (switch resolve + reason). routine-guard.test.mjs :576 (tags option, the one changed line), :578-582, :597-609 (dayIo answers), :773-804 (DAY_CASES), :916-929 (DAY_MUTANTS), :1011-1050 (real-git test).
- residuals (for security-auditor): (1) a cd run through a child-process wrapper (`env cd x`, `nohup cd x`, `timeout 5 cd x`) does not move the shell, but unwrap strips the wrapper and dayCd follows it as a move (pre-existing, not finding 5/6). (2) lineSetsCdpath is a list: a setter outside it (a function defined in the shell profile, BASH_ENV) is not seen; profile-level CDPATH stays out of the guard's env (noted in 4b). (3) `git switch <name>` with no local branch (git's remote-tracking guess) asks, as before 4c. (4) the flag is line-level: a setter after the cd also loses it (stricter, never looser).
- C8 done. DONE
