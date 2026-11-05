Task: A3 (kit-v3-3) step 8 fix round, builder part 1: the cd tracking redesign R-a..R-e (SA-1, auto 7, auto 8, A-1).

Plan (blocks):
B1 read ruling + reviews (SA review, verifier, skeptic SA-1 + probe, auto-3) + plan-rev2 (relevant parts)
B2 read routine-guard.mjs cd tracking code (parser, unwrap, dayCd, lost(), lineSetsCdpath, VAR_SETTERS) and existing tests on cd
B3 implement R-a..R-e in routine-guard.mjs
B4 add tests to routine-guard.test.mjs (Ff1-Ff3, skeptic 7, auto 7/8, failed cd, ||, if, timeout, command -p, quiet controls)
B5 run suites; list movers (existing cases whose expected result moves)
B6 mutation rows R-a..R-e under .gate/kit-v3-3/, run
B7 write answer, DONE

Notes:
- B1 ruling read (advisor-fix-round.md): R-a..R-e as quoted in task.
- B1 done (reviews, skeptic probe, auto-3 read). B2 done: code read routine-guard.mjs parseShell :124-215 (no op recorded; || and && endCmd :192-193), unwrap :218-242, bash() :470-519, dayShellWrites :568-599, dayCd :604-624, lineSetsCdpath+VAR_SETTERS :631-640; tests: dayIo :575 (exists true except CLAUDE.md), DAY_CASES :637-805, DAY_MUTANTS :875-933 (S-4/F5 rows target code R-b deletes).
- design decisions (stricter than ruling, safe side): cd prefix allowed only if/{/!/time/builtin/command(-p/--); then/else/elif/do/while/until prefix -> unknown (loop or branch may skip or repeat the cd); command -v/-V cd -> unknown (does not run cd); a cd after || -> unknown and ends the lead; pushd to an unknown target -> stack unknown.
- Next B3 implement.
- B3 done (code edited; line numbers final in B7). First run: 393/401; movers (all A3's own DAY_CASES, no main case): S-4 cd -P (null->ask, R-b option), S-4 relative cd .gate/x (null->ask, R-b bare name), F5 ./ after export (null->ask, R-c), F5 absolute after export (null->ask, R-c), F5 printf then cd (null->ask, R-c/R-b), F5 cd docs then write (null->ask, R-a/R-b), F5 cd .claude then kit write (ask stays, reason now "after a cd the guard cannot follow"); DAY_MUTANTS rows for deleted code fail (expected).
- Next B4 tests.
- B4 in progress: dayIo gained missing option + isDir (test :576, :626); movers renamed+re-expected; ~75 new DAY_CASES (Ff1-3, SK4-7, R-a..R-e, A7/A7b/A8/A8b, QC1-4); DAY_MUTANTS: S-4/F5 rows on deleted code replaced by 28 R-a..R-e rows; dead stack-null line removed from the unfollowed branch.
- B4 done. B5 done: node --test routine-guard + settings + checkpoint = 620/620 (466 + 127 + 27; was 401 + 127 + 27 = 555). Main's test file (12d7961) on the new guard: 189 pass, 3 fail, the same 3 as on the pre-fix guard 442fbf7 (merge in a day session, day: edit CLAUDE.md, mutation runs: pre-existing R1/N-2/MUTANTS), so no main case moves. 442fbf7's test file on the new guard: fails exactly the 7 movers + the day mutation run. New test file on the pre-fix guard: 42 of the day/real cases fail (all of Ff1-3, SK4-7, A7b, A8, A8b, the real-folder test), QC1-4 pass on both.
- B6 done: .checkpoints/kit-v3-3/mut-fix1.mjs -> mut-fix1.out (copies .gate/kit-v3-3/fix1-mut/m0..m27, control): 28/28 killed, control 212/212; in-suite DAY_MUTANTS (28 new rows replacing 15 S-4/F5 rows) 0 survivors.
- finding: harness bug fixed: String.replace treats $` in the replacement as a pattern; harness uses a function replacement (in-suite rows have no such $ pattern).

## Answer (B7)
Changes, .claude/hooks/routine-guard.mjs:
- :15-17 io doc names isDir?; :20 imports accessSync, constants, statSync.
- :138-145 parseShell records sep (the operator before each command); :200 endCmd('&&'), :201 endCmd('||').
- :234 unwrap strips command's -p, -v, -V, -- (unwrap is day-only; routines use unwrapRole, unchanged).
- :484-504 bash(): dirs {stack, known, lead}; dayShellWrites gets null while unknown (:493); dayFolder (:498) then vcwd = projectDir while unknown (:499) for the main-folder check; cd/pushd/popd skip the rest (:504). lineSetsCdpath call deleted.
- :607-613 dayShellWrites R-a: relative target while unknown -> ask (agent deny), reason ends "(write to an absolute path, or put the cd first in the line)"; absolute targets as before (:614).
- :621-672 dayFolder replaces dayCd; UNKNOWN_FIX :633, CD_REACH :634. Deleted: dayCd, lineSetsCdpath, VAR_SETTERS, the io.env.CDPATH branch (0 hits left).
- :1058-1059 realIo isDir (X_OK and a directory).
Stricter than the ruling (safe side): then/else/elif/do/while/until before a cd -> unknown; command -v/-V cd -> unknown (does not run); a cd after || -> unknown; pushd to an unknown target -> stack unknown; a pure VAR= command ends the leading run.
Tests, .claude/hooks/routine-guard.test.mjs: dayIo missing option :576 and isDir :625 (scaffold; default = every folder there, as exists answered before); movers :772-776, :791-802; 64 new DAY_CASES :814-896; DAY_MUTANTS :1000-1030; real-folder test :1162-1213.
DONE
