TASK: builder, A3 kit-v3-3 fix pass 3, day side (parseShell's five day rules + sub() comment; P3-7 clone targets; C-PLAIN rerun; no-looser diff; movers; main's file; gate with smoke) in routine-guard.mjs and its test.

## Plan (blocks)
B1 read ruling (advisor-pass3.md) + routine-a/-b notes - done
B2 read findings (security-auditor-pass3, verifier-pass3, skeptics), probe lines H8/R4/A1/D5, builder-pass2-day/-routine
B3 read guard code: parseShell, sub(), clone branch; test file structure (day tables, mutants, real-git/bash tests)
B4 implement parseShell five rules + sub() comment
B5 implement P3-7 clone
B6 tests: parseShell day rows (+H8, R4 real bash) + mutants; P3-7 rows + mutant
B7 suites; C-PLAIN; no-looser diff vs 01b158c; movers across fix pass 3
B8 main's file flat copy; widths; gate with smoke
B9 answer

## Notes
- B1 done. Next B2.
- B2 done: probe lines: H8 day `git status # '\ngit switch -q cs/evil # '` (sap3-probe.mjs:53); A1 `git status  # what's changed\ngit switch -q cs/evil` (skp3-p31.mjs:46); D5 `git status \\\n# '\ngit switch -q cs/evil # '` (sap3-probe2.mjs:49); R4 day `git status >&CLAUDE.md` (adv3-probe4.mjs:48). Real-bash style: bash --norc --noprofile -c in scratch repo with origin.git, cs/ok, cs/evil (settings {"a":2}). P3-7: writtenBy clone made = pos.map(cloneDir). Next B3 code.
