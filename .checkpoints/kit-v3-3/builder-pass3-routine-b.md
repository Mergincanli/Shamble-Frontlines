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
- B6 done: new tests pass; real git diagnostics in .gate/kit-v3-3/pass3/rb-p35.txt (allowlist off lets 22 rows through; -- 6; -i 1; -o 1; pathspec 1; P3-6 flag 4 rows). Fix during: P3-6 hiding needs the worktree's .gitmodules (git reads ignore from the worktree): switch rows start on cs/gm (adds only .gitmodules). Next B7 suites.
- B7 part: suites 1026/1026 = routine-guard 872 + settings 127 + checkpoint 27 (27.8 s). Next: movers harness.
- B7 part: movers harness builder-pass3b-movers.mjs/.out (flat copy .gate/kit-v3-3/pass3/movers-b; old = 01b158c with its diff calls given the P3-6 flag so fakes answer): 2019 decisions, looser 0; existing tables: only side A's 8 reason moves, none from side B; PASS3B rows: 38 null->deny or reason moves vs old (S2d, --amend reason only), all stricter. Next: main's file.
- B7 part: main's file (12d7961) flat copy .gate/kit-v3-3/pass3/moh-b (new guard + guard.mjs): 180/192, the same 12 as 01b158c (moh) with identical messages (diffed): 3 pre-existing (#23, #42, #192), 8 rev-parse scaffold (#50,#52,#61,#62,#63,#64,#74,#163), 1 SD-2/P3-6 key (#122 'night: commit a staged rules file': main's fake answers 'diff --cached --name-only'). No new failure; no commit-allowlist mover in main's file. Latent probe (latent-new/latent-old, unmutated-wrong cases skipped): no main MUTANTS target text broken by side B; survivors differ from 01b158c only by side A's 'unquoted expansion (git add *)'. Next: widths, gate.
- B7 done: widths: no added guard line over 120 (vs 01b158c); gate --no-smoke GATE PASS (base 12d7961; [1] 5/5 [2] PASS [3] +0 -0 core 138/game 714/editor 353 [4] 51 guarded 0 dead [5a] SKIP [5b]-[5d] PASS; .gate/kit-v3-3/pass3/rb/gate.txt). Next B8 line refs + answer.

## Answer (B8)
Guard .claude/hooks/routine-guard.mjs (not committed):
- P3-5: comment :904-905, NIGHT_COMMIT_FLAG :906, NIGHT_COMMIT_ONLY :907-908 (ruling's text word for word), nightCommitPlain :909-923 (flags, bundles -[aqs]+, --message=, -m/--message/-[aqs]*m with the message attached or the next word; a message required: stricter than the ruling's list, the text's own form), the refusal :949 (night only, right after the SD-1 alone check, before gitArgsCheck). Commit check :1000-1013: comment :1001-1003; the named list and its join removed; hit over the lists only :1010-1012. The -a test (old regex, :1005) kept: a superset of the parsed -a (a message word such as -m -am also reads the unstaged list): longer list, never shorter.
- P3-6: DIFF_NAMES :1034 gains --ignore-submodules=none (comment :1032-1033); switchTargetCheck :1094 now lists with ...DIFF_NAMES (was its own literal list), so the flag is at every caller: night switch, protectedHit (night push, checkpoint save/watch, day main folder), the commit's lists.
Tests .claude/hooks/routine-guard.test.mjs:
- Scaffold (named, A3's own): NAMES_Z :31-32 (+flag; fakeIo keys :63/:66/:67 and dayIo :654 follow it); dayIo :647 slice(0,7)/args[7], :655-656 slice(0,7)/args[8]/length 9/args[7]; WT-NIGHT SHA test :1830-1833 indices 6/7/8 (was 5/6/7); NIGHT_SWITCH_MUTANTS :1846-1847 'the diff gets the commit...' and :1852 '-z is passed' target texts; real-git flags loop :1952-1957 (from ...DIFF_NAMES, the copy spells the list out); real pass-2 mutants :2185-2191 'SD-2 -z', 'SD-2 --no-renames', 'SD-2 --no-relative' texts. Main's file untouched.
- New block :2808-3075: COMMIT_ONLY :2813, DIRTY/DOCS/shIn, PASS3B_COMMIT_CASES :2817 (56 rows: 43 deny, 13 null) + runner :2880 'routine (fix pass 3, P3-5) ...'; 'fix pass 3 (P3-5): the night commit refusal, word for word' :2887; P3B targets :2892, P3B_MUTANTS :2902 (11) + 'mutation runs (fix pass 3, P3-5): ...' :2915; 'real git (fix pass 3, P3-5, P3-6): the night commit allowlist; submodules never hidden' :2945 (36 rows, 6 real mutants).
Moved: no existing case moved (movers harness: CASES, DAY_CASES, ROUTINE_FIX_CASES, NIGHT_SWITCH_CASES, PASS2 old vs new: only side A's 8 reason moves). Versus 01b158c on the new rows: S2d '-- CLAUDE.md' and '--amend' deny reason moves to the allowlist text; 38 null -> deny; 0 looser over 2019 decisions.
Counts: 1026/1026 = routine-guard 872 + settings 127 + checkpoint 27 (was 967 after side A; 864 at 01b158c), 27.8 s.
Mutation: P3B_MUTANTS 11/11 killed (in suite, kind+reason); real: allowlist removed lets 22 rows through, '--' and after 6, -i 1, -o 1, --pathspec-from-file 1, P3-6 flag dropped 4 (night switch, night push, day switch, night commit of a staged gitlink): all killed; all earlier tables still pass (scaffold-edited ones included). RB: the allowlist-off copy lets S2a and 15 more path rows through and real git commits CLAUDE.md (N3: .claude/settings.json); every null row's real commit equals the guard's lists.
P3-6 finding: git reads `ignore` from the worktree's .gitmodules, so the hiding needs that file in the worktree (night push on the branch, a staged commit, or a switch from a branch holding it); the switch rows start on cs/gm.
Main's file: 180/192, the same 12 as 01b158c with identical messages; #122 is the SD-2/P3-6 key; no new failure.
Gate --no-smoke PASS.
DONE
