TASK: builder, A3 kit-v3-3 fix pass 3, routine side B (P3-5 night commit allowlist, P3-6 DIFF_NAMES --ignore-submodules=none) in routine-guard.mjs and its test.

## Plan (blocks)
B1 read ruling (advisor-pass3.md) + side A notes - done
B2 read findings for P3-5/P3-6: security-auditor-pass3, verifier-pass3 (N3), skeptic P3-5, probe scripts cited
B3 read builder-pass2-routine/day notes (style), guard commit check + DIFF_NAMES + test fakes keys, real-git tests
B4 implement P3-5 allowlist in guard
B5 implement P3-6 flag; update fake-git keys in A3 test (named)
B6 tests: commit allowlist rows (real git) + mutants; P3-6 rows (real git) + mutant
B7 run suites; main's file flat copy; movers check; gate --no-smoke
B8 answer

## Notes
- B1 done. Next B2.
- B2 done: P3-5 = SA sap3-probe7 (S2a-S2h pathspec forms), verifier N1-N5b (N3 '-- settings.json' from .claude/); P3-6 = sap3-sub/sub2 (.gitmodules ignore=all hides gitlink; flag lists it). Next B3.
- B3 done: guard: commit check :979-993 (lists, -a regex :983, named :988), allowlist goes after NIGHT_ALONE :924-928 (before gitArgsCheck), DIFF_NAMES :1012, switchTargetCheck own literal diff list :1072 (not DIFF_NAMES) -> make it ...DIFF_NAMES so the flag is at every caller. Test scaffold to follow: NAMES_Z :31 (fakeIo keys :62/:65/:66, dayIo :646/:653/:654 slice(0,6)/args[6]/args[7]/length 8), WT-NIGHT SHA test :1829-1832 indices, NIGHT_SWITCH_MUTANTS :1845, :1850 texts, real-git flags loop :1947-1950 text, real pass2 muts :2181-2186 DIFF_NAMES const text. Keep substrings of PASS2 mutants :2066-2070 in the rewrite. Keep old -a regex (superset of parsed -a: never looser). Require a message (the text's form; mutant).
- B4 done: guard: NIGHT_COMMIT_FLAG/NIGHT_COMMIT_ONLY/nightCommitPlain after NIGHT_ALONE; deny line after the NIGHT_ALONE check (before gitArgsCheck); commit check: named + join removed, comment. B5 guard part done: DIFF_NAMES + '--ignore-submodules=none', switchTargetCheck uses ...DIFF_NAMES. Next: run suite, test scaffold edits.
- B5 scaffold edits done (NAMES_Z, dayIo slice/index, SHA test indices, NIGHT_SWITCH_MUTANTS 2 texts, flags loop, real pass2 SD-2 3 mutant texts). Running suite.
- B5 done: suite 813/813 after scaffold edits. B6 design: PASS3B_COMMIT_CASES (fake, kind+reason) + word-for-word + PASS3B mutants (allowlist removed, night only, -- and after accepted, -i/-o break, pathspec-from-file accepted, bundle o x2, message required, message word skipped); real git test: commit rows (RB S2a via allowlist-off copy + real commit holds CLAUDE.md; null rows: real commit holds exactly the listed sets), P3-6 rows (cs/sub gitlink + .gitmodules ignore=all; .gitmodules not protected), real mutants. Next: write tests.
- B6 part: test block appended at file end (PASS3B_COMMIT_CASES, word-for-word, P3B/P3B_MUTANTS + runner, real git P3-5/P3-6 test). Running.
