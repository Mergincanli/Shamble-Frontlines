TASK: kit-v3-3 builder, step 4b: advisor step-4 ruling R1-R4 + automated security review S-1..S-4 of f22c4df.

Plan (blocks, in order):
B1 read rulings + builder-step4 + security review (done at start)
B2 read routine-guard.mjs sites + test files structure (+ plan 4.1 dayEdit, 5 A-tests text)
B3 baseline test counts
B4 R1 dayEdit + decide wiring (routine-guard.mjs)
B5 R2 merge guard; R4b agent refusal on unresolvable target
B6 S-1/S-2 ref resolution by SHA; S-3 darwin fold; S-4 pushd/popd/untracked cd
B7 R3 settings.json allow + settings.test NEEDED
B8 tests: R1 (:129-130, :590 comment, new case, Edit forms, S2/S3), R2 T5c-e, R4 A20a-c, fix1 real-git, S-1..S-4 tests
B9 test run
B10 mutation rows in .gate/kit-v3-3/
B11 answer

Notes:
- B1 done. Next B2.
- B2 done: sites read (routine-guard.mjs at f22c4df: decide :336, dayShellWrites :565-590 (unresolvable :578-582), bash cd :484-491, mainFolderMove :741-779, merge :766-768, isWin :64; tests: fakeIo :30-76, :110, :129-130, dayIo :559-583, DAY_CASES :589-660 (:590 comment), A26 :682, DAY_MUTANTS :730-749; settings.test S2-S5 :201-217, NEEDED :35-45).
- finding: DAY_MUTANTS rows (step-4 scaffold) target code text that S-1/S-2/S-3 must change: 'switch: no name exempt', 'switch -c: the start point', "switch -c: HEAD's protected files", 'merge forms', 'win32 folds case'. Those rows get their target text updated (same mutation, new text); named in the answer.
- finding: dayIo answers diff only for `origin/main <ref> --` by ref name; S-1 passes a SHA and S-2 a fully qualified ref: dayIo scaffold gets rev-parse --verify answers (fake SHAs per name) and maps SHA back; no DAY_CASES row changes.
- finding: fakeIo answers every unknown git call {0,''}; S-2's commit compare with a SHA-shape check would flip :110 (C6). Decision: fakeIo scaffold gets ONE added line answering `rev-parse --verify --end-of-options (refs/remotes/)origin/main^{commit}` with SHA, as real git does; no case row changed. Named in the answer.
- decision S-4: day path only (routines keep their cd block; pushd stays refused in routines): cd/pushd/popd tracked with a line-local stack; untracked forms (cd -, $/glob/~user target, CDPATH set + bare relative, +N/-N, unknown options, popd/pushd with empty stack, any cd in a line with subshell parens, pipes, & or $(...)) -> vcwd = project folder.
- decision S-2 residual: routine protectedInBranch ['origin/main...HEAD'] and interpreter/npm `diff --quiet origin/main` keep the short name (C6, R4): a routine-created local branch origin/main would shadow them. Note for security-auditor.
- B2 done. Next B3 baseline.
- B3 done: baseline routine-guard 317/317, settings 124/124, checkpoint 25/25 = 466/466. Real git 2.43 probe (.gate/kit-v3-3/probe1): `rev-parse --verify --end-of-options origin/main^{commit}` prints the SHA; with `--output=x^{commit}` "fatal: Needed a single revision" exit 128, no file; a local branch origin/main shadows (warning "refname is ambiguous", rev-parse origin/main gives the local branch's SHA, refs/remotes/origin/main the remote's). Note: the session's own guard refuses `git update-ref` (GIT_NEVER), so probes build origin/main by `git remote add origin <self> && git fetch`.
- Next B4.
- B4 done: dayEdit added after dayShellWrites; decide `if (!role) return dayEdit(tool, ti, cwd, io, input);`; header :9-11 reworded. (line refs recorded at the end)
- B5 part done: R4b agent deny on unresolvable target inserted in dayShellWrites. R2 merge guard goes in with S-2 (one rewrite of the merge branch).
- design S-1: commitOf(ref): a word starting with '-' -> null without git; else `rev-parse --verify --end-of-options <ref>^{commit}`, SHA-shaped output only. switch passes only the SHA to diff. Layers: SHA-only (table-caught via dayIo emulating diff --output = code 0, empty, as git), '-' check (caught by a call-recorder test), end-of-options (belt).
- design S-2: ORIGIN_MAIN = 'refs/remotes/origin/main'; isOriginMain(ref) = commitOf(ref) === commitOf(ORIGIN_MAIN); merge form = [t] or ['--ff-only', t], t must resolve to that commit; switch -c start point the same; diffs use ORIGIN_MAIN.
- B5 done (R2 merge guard + S-2 in the merge branch; R4b).
- B6 done: S-1/S-2 (ORIGIN_MAIN, commitOf, isOriginMain; switch passes SHA; switch -c start by commit; merge by commit + R2 three-dot guard), S-3 (foldsCase win32+darwin), S-4 (parseShell `forks` flag; bash() day cd/pushd/popd via dayCd before the routine cd block; routines unchanged).
- Next B7 R3 settings.
- B7 done: settings.json allow `Bash(git rev-list *)` after merge-base; settings.test NEEDED.push(rev-list --count); settings.test new test "S2-S3 the real guard, file tools" (Edit/Write/MultiEdit/NotebookEdit; game file null); the S2-S5 comment line changed (comment only: "the Edit-tool form waits" -> "is the next test"). settings 127/127.
- Next B8 routine-guard tests.
- B8 part 1 done: test :129-130 per R1 + 'day: edit a game file'; :590 comment; fakeIo one rev-parse line; createHash import; dayIo scaffold (shaOf, threeDot, shadow, calls, --output emulation, SHA diffs). All existing cases pass; only the day mutation test fails (stale targets). Next: DAY_MUTANTS targets + new rows + new tests.
- B8 part 2 done: DAY_MUTANTS 5 rows retargeted (win32 folds case, switch: no name exempt, switch -c: the start point, switch -c: HEAD's protected files, merge forms; same mutation, new code text), 17 rows added; DAY_CASES +57 rows (R1 A-forms, S-3, R2 T5c-g, R4 A20a-c, S-1, S-2, S-4); helper ded; dayIo cdpath option. routine-guard 372/372. Next: recorder test (S-1) + real-git test (fix 1, S-1, S-2).
- B8 done: tests "S-1 no guard-run git command gets a word of the command that starts with '-'" (call recorder) and "real git (step 4b): `--` after the ref (fix 1), no file written (S-1), origin/main by commit (S-2)" appended; imports mkdirSync, realpathSync, realIo.
- B9 done: routine-guard 374/374 (317 + 57: 1 CASES row, 54 DAY_CASES rows, 2 tests), settings 127/127 (124 + 3: NEEDED rev-list x2 tests, S2-S3 file tools), checkpoint 25/25; combined 526/526.
- B10 done (script .checkpoints/kit-v3-3/mut-step4b.mjs, output mut-step4b.out; copies .gate/kit-v3-3/mut-step4b-0..17): every mutant caught. Copies' settings S2-S5 fails in every copy incl. unmutated (copy root is not a repo root: its `git switch cs/x` S4 is not in a main folder), so it is the baseline, not a catch.
  [1] base f22c4df guard + new tests 324/374 (new R1/R2/R4/S-1..S-4 cases fail; T1/T2/T10/T12/T18/T24/T27 fail there too because dayIo now answers the SHA form only: a scaffold artifact).
  [2] R1 file-tool ask off 358/374 (A1, A2, A3, A9-A10, A22b, A25, A36, S-3 Edit, :130) · [3] dayEdit unwired 357/374 · [4] both asks off 334/374
  [5] R2 plain merge unguarded 370/374: T5c, T5d, T5e + day mutation test · [6] R3 allow removed: settings 125/127 "needed, matches an allow: git rev-list --count origin/main..HEAD"
  [7] R4 agent ask 371/374: A20a, A20b · [8] S-1 word not SHA 367/374: S-1 rows, T16d, T19, recorder, real-git · [9] S-1 '-' check off 373/374: recorder test · [10] '-' check and --end-of-options both off 360/374
  [11] S-2 merge by name 369/374 (S-2 rows, real-git) · [12] switch -c by name 371/374 · [13] short-name resolve 368/374 · [14] S-3 darwin off 370/374 (3 darwin rows)
  [15] pushd/popd not followed 368/374 · [16] forked not lost 370/374 (subshell, substitution, pipe) · [17] untracked target not lost 369/374
  In-suite DAY_MUTANTS: 33 rows, 0 survivors.
- S-1 real-git probe (.checkpoints/kit-v3-3/s1-probe.mjs; repos .gate/kit-v3-3/s1-probe-*): base f22c4df guard: decision null, file written TRUE; step 4b guard: ask, file written false.
- Next B11 line refs + answer.
- B11 line refs (worktree, uncommitted): routine-guard.mjs header :9-11; foldsCase :65-67 (+ :73); parseShell forks :122-123, :126, :195, :216; decide :341; bash() :472, :476, :490-493; dayShellWrites R4b :589-590; dayCd :603-627; dayEdit :628-637; ORIGIN_MAIN/commitOf/isOriginMain :774-787; switch -c :811, :816; switch target SHA :822-826; merge + R2 :829-837. protectedInBranch :763 unchanged.
  routine-guard.test.mjs: imports :12, :16, :18; fakeIo :60-61; :133-134 (R1); dayIo header :556-565, body :569-602 (calls :581); ded :607-608; :614 comment; DAY_CASES additions :684-748; DAY_MUTANTS retargeted :828-836, added :840-859; S-1 recorder :885; real-git :895.
  settings.json :30; settings.test.mjs :46, :204 (comment), :219-235.
- residual (for security-auditor): routines keep `origin/main` by short name in protectedInBranch :763-764 (`origin/main...HEAD`), interpreter :~640 and npmCheck :~463 (`diff --quiet origin/main --`): a local branch or tag named origin/main (night shift can create branches) would shadow the remote ref there. Also S-4 limits: CDPATH from the shell profile is not in the guard's env; cd inside if/while bodies or after a failed cd (`cd x || ...`) is read as having run.
- residual: dayIo scaffold change makes some step-4 T cases fail against the f22c4df guard (they answer the SHA form only).
- B11 done. DONE
