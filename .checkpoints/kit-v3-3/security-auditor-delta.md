# Task: kit-v3-3 A3 security-auditor delta review (442fbf7..ed3ea8c: fix round 2acfbbc+22f2480, step 9 ed3ea8c)

## Plan (blocks)
B1 read context: security-auditor-review.md, advisor-fix-round.md, builder-fix1/2/step9, mustafa-words, auto-4 (9,10,11)
B2 read the delta diff (hooks/scripts code)
B3 fix round checks + probes (cd tracking, SA-2 routine half, SA-2b, F-b, SA-5, SA-6, SA-8, routines no looser)
B4 step 9 night switch probes (forged refs, renames, unusual names, options, resolution order, pr-advisor/day unchanged)
B5 weigh auto 9/10/11 and F9-1; named limit proposal
B6 write findings; DONE

## Notes
- B1 done: read SA review (SA-1..SA-10), advisor ruling (R-a..R-e, SA-2 1-6, F-b, SA-5, SA-6, SA-8; L1,L4,L9,L12-L15), builder fix1 (dayFolder, stricter), fix2 (originMainCheck :904, newBranchNames, fetch ':'/'+', symbolic-ref HEAD only, remote reads, SA-5 all positional, SA-6 ls-files, SA-8 line check, F-b clone/worktree), step9 (switchTargetCheck, F9-1 confirmed on real git), Mustafa WT-NIGHT (a), PC-SETUP Windows PowerShell Auto Enterprise; auto-4: 9 ls-files index only, 10 assembled var names, 11 clone --revision. Next B2 diff.
- B2 done: diff read (guard :107-127 kitDir, :144-153 sep, :243 unwrap command, :495-506 bash, :554-616 SA-8/F-b, :618-635 sed/perl, :656-663 R-a, :679-720 dayFolder, :764 interp, :826-840 SA-2 block, :843-844 night call, :874 protectedInBranch, :887-928 switchTargetCheck, :964-1000 originMainCheck/newBranchNames). Candidates to probe in B3/B4:
  C1 `cd <missing> && cd <abs wt>; git switch x` : && after an unknown cd re-knows the folder (bash skips the 2nd cd). dayFolder :705 only '||' unknown.
  C2 `! cd <main> && cd ./.gate/wt; git switch x`: '!' in CD_REACH inverts status, && skips 2nd cd.
  C3 night line `git branch -f cs/ok <evil> && git switch cs/ok` or `git fetch origin cs/x && git switch cs/x`: switch check reads refs at hook time, earlier commands in the same line move them (TOCTOU in one line).
  C4 perl bundles `-wpi`, `-lpi` miss in-place (/^-p?i/, pre-existing, block touched by SA-5).
  C5 switchTargetCheck uses protectedRel (no fold): case-insensitive FS only (night runs Linux) nit.
  C6 F9-1 rename-away of the guard file: hook fails -> non-blocking -> guard off; check mainFolderMove use of protectedHit.
- B3/B4 part 1 (code read :236-305 lists, :775-862 git(), :864-881 protectedHit, :938-1000, :1008-1052 mainFolderMove):
  C3 refined: branch -f refused (GIT_SHORT_DANGER), but `git fetch origin cs/x && git switch cs/x` and `git branch cs/n origin/cs/evil && git switch cs/n` (origin/cs/n clean) remain; the ordinary handoff line. To probe.
  C6 refined: F9-1 reaches the DAY main-folder check: mainFolderMove :1032 protectedHit (plain name-only) for git switch <branch>; a branch renaming .claude/hooks/routine-guard.mjs away prints only the new name -> no ask -> guard file missing. To probe + check guard.mjs import failure behaviour.
- B3/B4 probe run 1 (sad-probe.mjs -> sad-probe.out, real git 2.43, head guard):
  CONFIRMED C3 (major, step 9): night `git fetch -q origin cs/x && git switch -q cs/x` null, real lands on settings {"a":3}; `git branch cs/n origin/cs/evil && git switch -q cs/n` null, real lands on {"a":2}. Day `git fetch && git switch --detach origin/cs/x` null too.
  CONFIRMED C1 (`cd <missing> && cd <abs wt>; git switch -c ...` null, real main folder switched) and C2 (`! cd <main> && cd ./.gate/wt; git switch ...` null, real main folder switched).
  CONFIRMED C4 perl -wpi/-lpi/-0pi null (real perl -wpi edits); auto-11 --revision null; --bran deny (abbrev rule). auto-9 rm -rf .gate null, rm -rf .gate/wt ask.
  F9-1: night push of a branch renaming the guard away null (confirmed); day C6b ask was from a deleted local branch (can't compare) -> rerun with local branch.
- probe run 2: CONFIRMED C6/F9-1 day path: `git switch cs/ren` (local branch renaming .claude/hooks/routine-guard.mjs to docs/) from the main folder -> null; after it the guard file is gone and guard.mjs's bootstrap exits 0 for a day session (settings.json:272, guard.mjs:1-17) -> guard off. Control cs/evil asks. Night switch cs/ren deny (step 9 -z/--no-renames). Night push of cs/ren null.
  CONFIRMED C1c: `cd <missing> && cd <non-repo abs>; echo x > CLAUDE.md` null, real overwrote the main CLAUDE.md.
- probe2 (sad-probe2.mjs -> .out): night and pr-advisor writes into .git refs/packed-refs/config (Bash and Write) all DENY: SA-2 holds there. Routine-looser check: writtenBy, kitDir, unwrap, dayFolder are day-only (callers :509-:662); parseShell sep adds a field only; every routine change is an added refusal (:826-844, :482, :764, :874). protectedRel :32-43 read.
- B3 done. Next: run suites at head, then B5/B6.
- suites at head: 768/768. probe3 (sad-probe3.mjs -> .out): SA-1 forms, R-b bare, R-e source, SA-5 (--in-pla=, -s -i, -i'' -es, --expr), SA-8 (export, env, declare -x), F-b (worktree add .claude/wt, --lock --reason -b, clone into .claude), SA-6 (rm -rf docs nested CLAUDE.md, rm -rf ancestor, mv docs) all ASK; controls null. Clone into a NEW tools/x null (nested CLAUDE.md brought by the clone: same class as L11 worktrees, accepted).
- candidate C7: day globs `sed -i s/a/b/ *.md` at the project root edits CLAUDE.md; check whether a stated limit covers globs.
- C7 withdrawn: globs and braces in day write targets already ask (probe3 C7 rows). C4 perl bundles are named in L2 (STATUS.md:682-684) -> nit (cheap fix lets L2 drop it). C1 contradicts STATUS.md:715-716 ("a cd after a failed cd" listed as closed).
- B4 done (step 9): role gate :842 night only (pr-advisor and day unchanged); options strict (abbrev/-qd/-f/-C/--orphan/'-' refused); plain name refs/heads then refs/remotes/origin (:918) matches git switch DWIM; -c start/--detach by rev-parse as git; @{-1} plain fails closed; -z/--no-renames/--no-relative (:924); forged origin/main via originMainCheck; SA-3 independent. Hole: C3 (in-line TOCTOU). Nit: protectedRel not folded (:927), Linux night only.
- B5 done (weighing): auto-9 fix (worktree ancestors; untracked/nested repos limit); auto-10 named limit L16; auto-11 fix (every positional + --revision); F9-1 fix now (day guard-off rename, routine push/commit).

## Final (B6)
Verdict FAIL: two majors in the delta's scope (SD-1 new in step 9; SD-2 = F9-1 on the day and routine paths), both cheap.
SD-1 major :842/:893-928 + bash() :502-520 one-line TOCTOU: `git fetch -q origin cs/x && git switch -q cs/x` and `git branch cs/n origin/cs/evil && git switch -q cs/n` null, real lands on kit-changed commit (sad-probe.out C3b, C3c). Fix: night switch refused unless every earlier command in the line is cd/pushd/popd; NIGHT_SHIFT.md:27 says fetch in its own call first; rows C3b, C3c + mutant.
SD-2 major F9-1 :870, :845-846: day `git switch cs/ren` (renames .claude/hooks/routine-guard.mjs to docs/) null (C6b) -> guard file gone, bootstrap settings.json:272 exits 0 for day -> guard off (consequence by code reading, not run end to end); night push null. Fix: -z --no-renames --no-relative, split NUL, in protectedHit and the commit check; lift C6 for these lines; rows rename/non-ASCII/diff.relative day + night push + mutants.
SD-3 minor day C3d :1031-1032: `git fetch -q origin cs/x && git switch --detach origin/cs/x` null. Fix: day main-folder switch/merge after an earlier git command in the line asks unless the target word is origin/main.
SD-4 minor dayFolder :695, CD_REACH :682: C1 `cd <missing> && cd <abs wt>; git switch -c ...` null (real main switched), C1c main CLAUDE.md overwritten no ask; C2 `! cd <main> && cd ./.gate/wt; git switch` null (real). Fix: cd with sep '&&' while !dirs.known stays unknown; drop '!' from CD_REACH; rows C1, C1c, C2.
SD-5 minor auto-11 :556-558, :603-616: clone --revision misparse null. Fix: every positional word and cloneDir of each are targets; add --revision.
SD-6 minor auto-9 kitDir :113-127: rm -rf .gate null. Fix: ancestor of any `git worktree list --porcelain -z` path asks, git failing asks; untracked kit files / nested non-worktree repos -> stated limit.
SD-7 nit :619 perl -wpi/-lpi/-0pi null (L2 names it). Fix /^-[^-]*i/ for perl, drop clause from L2.
SD-8 nit auto-10 -> named limit L16 deliberately disguised shell text (text proposed).
SD-9 nit :927 protectedRel unfolded; night on Linux; accept or protectedFold.
Right: R-a..R-e hold on probe3 + skeptic rows; SA-2 routine half and SA-2b hold (probe2 .git writes deny; fetch tag shadow caught by originMainCheck by construction); F-b, SA-5, SA-6, SA-8 hold (probe3); routines no looser (all added refusals; changed helpers day-only); step 9 options/names/resolution order right; pr-advisor and day unchanged by step 9; texts CLAUDE.md:251-252, NIGHT_SHIFT.md:27-31, CHECKPOINTS.md:192-194 match Mustafa's (a).
DONE
