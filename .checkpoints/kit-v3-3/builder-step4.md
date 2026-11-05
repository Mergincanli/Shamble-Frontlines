TASK: kit-v3-3 builder, plan rev2 section 8 step 4: day ask (AF-MECH, AF-AGENT keyed on agent_id), main-folder checks (WT-DAY, WT-UPDATE), protectedHit refactor, gh() vcwd, guard.mjs ask with C5 fail-closed print (A37, A26). Not switchTargetCheck.

Plan (blocks, in order):
B1 read plan sections 0, 4 (4.1, 4.2), 5, 8 + advisor C1-C12 + E10 ruling
B2 read routine-guard.mjs relevant sites, guard.mjs, test files structure
B3 baseline test counts
B4 edits routine-guard.mjs (protectedHit refactor, gh vcwd, WT-DAY, WT-UPDATE, AF-MECH, AF-AGENT)
B5 edits guard.mjs (ask print, C5)
B6 tests added
B7 test run after
B8 mutation rows in .gate/kit-v3-3/
B9 answer

Notes:
- B1 done: scope = 4.1 ask const + header :9-11, kitPath, dayEdit (decide :272), dayShellWrites (bash day path), mainFolderMove (git + gh), gh() vcwd, protectedHit refactor of protectedInBranch :554-560; 4.2 print ask shape + C5 + header :8-9 (E10-3); tests group "day ask-first" A1-A30,A33,A35,A36 (+A22b, A23-A26), A37, group "main-folder moves" T1-T14,T17-T19,T22-T25; settings S1-S5; mutation rows: day ask, agent deny (agent_type must fail A22b), kitPath order (projectRel first must fail A9b), stale-main (name exemption must fail T18), guard fail-closed (must fail A37). NOT mine: Actions, secret check, refusal texts :451/:453 + python assert (not in step 4 list; flag), WT-NIGHT. Next B2.
- B2 done: sites read. routine-guard.mjs: header :9-11, deny :257, decide :259-291 (`if (!role) return null;` :278), bash :408-444 (day path: unwrap :417, cd :421, git/gh :438-439), git :487-553 (end `return null` :552), gitTop :556, protectedInBranch :560-566, gh :568-596. guard.mjs print :23-34, :40. Tests: CASES assert `!!d === refused` (:297-302); MUTANTS :410-456 run over CASES with fakeIo (project /repo, rev-parse --show-toplevel -> /repo for every cwd).
- FINDING (C6 conflict, blocks part of the step): two EXISTING cases contradict the plan as written:
  (1) routine-guard.test.mjs:130 ['day: edit CLAUDE.md', DAY, Edit /repo/CLAUDE.md, false]: dayEdit (plan 4.1, A2) makes decide() return { ask } -> !!d true -> the case fails (and the unmutated-rules check :463 with it). No design keeps it without gaming the fake io: it is the exact opposite of A1/A2.
  (2) routine-guard.test.mjs:110 ['merge in a day session', DAY, git merge origin/main, false]: cwd /repo = the project = the main folder; WT-UPDATE "git merge other than --ff-only origin/main asks" -> fails.
  Per the brief: not changed; reported. Building the rest (stop on blockers, move on).
- decision: dayEdit (Edit/Write/NotebookEdit/MultiEdit ask) NOT wired/built; A1-A6, A8, A25, A36 (Edit forms) and S2/S3 (Edit forms) not added; shell variants added instead where they test the same rule (kitPath, agent deny, win32 fold).
- decision: merge clause built as "ask unless exactly `git merge --ff-only origin/main` OR exactly `git merge origin/main`": the plain form is a deviation (keeps :110), flagged for the advisor; tighten = drop it and change :110 to true.
- B3 (baseline) next.
- B3 done: baseline routine-guard 209/209 + settings 123/123 = 332/332. Next B4 edits routine-guard.mjs.
- B4 done (routine-guard.mjs): header, kitPath/kitDir/gitPlace/protectedFold helpers, ask const, bash() day path (dayShellWrites + ask carry), writtenBy + dayShellWrites, git() input param + mainFolderMove call, protectedHit + protectedInBranch refactor, switchWords + mainFolderMove, gh() vcwd/input + pr checkout/co. Lines rewrapped to <=120 (script .checkpoints/kit-v3-3/wrap-step4.py). Existing 332/332 still pass. Line refs recorded at the end (B7). Next B5 guard.mjs.
- finding: the fake io's `rev-parse --git-common-dir` answer for existing CASES is '' (code 0); kitPath then falls back to the project-relative check, so existing cases resolve as today.
- B5 done (guard.mjs): header :8-10 (E10-3 wording), comment :16-17 (C5), print() third arg + ask shape :25-37, :42-43 ask/deny dispatch. Existing 332/332 pass. Next B6 tests.
- finding (fixed in my code): `git switch -` was read as an option (switchWords skipped '-'); now a positional -> asks (T26).
- finding (fixed in my code): `git diff --name-only origin/main <ref>` reads <ref> as a PATH when no such ref exists and a folder of that name does (git switch docs with only origin/docs) -> empty diff -> fail open. mainFolderMove now passes `--` after the ref (git then fails on a non-ref -> ask). The routine call protectedInBranch keeps ['origin/main...HEAD'] unchanged (C6).
- B6 done: routine-guard.test.mjs section 5 appended (dayIo, DAY_CASES 68 rows, T17, A23, A24, A26, A37 + launcherWith + guard mutation test, DAY_MUTANTS 16 rows + day mutation test); settings.test.mjs one test S2-S5 (shell form). No existing test line removed or changed (git diff: 0 '-' lines).
- B7 done: after: routine-guard 284/284 (209 + 75), settings 124/124 (123 + 1), combined 408/408. Next B8 scratch mutations.
- B8 done (script .checkpoints/kit-v3-3/mut-step4.mjs; copies in .gate/kit-v3-3/mut-step4-0..7/, git-ignored):
  base HEAD guard (before steps 3-4) + new tests: 224/284 (the step-3 cases, and every new day ask/deny case fail; the null cases pass).
  day ask off: 260/284; fails A7s, A9s, A9b-s, A9c-s, A10s, A12-A17, A19, A22b-s, A27-A30b, A36s + day mutation test.
  agent deny keyed on agent_type (plan row): 281/284; fails A22s and A22b-s (as required) + day mutation test.
  kitPath projectRel first (plan row): 280/284; fails A9b-s (as required), A9c-s, A36s + day mutation test.
  stale main name exemption (plan row): 282/284; fails T18 (as required) + day mutation test.
  guard.mjs fail-closed off (plan row): 282/284; fails A37 (as required) + the in-suite guard mutation test.
  extra: main folder check off 281/284 (T8, T22); no `--` after the ref 277/284 (T1, T2, T10, T12, T18, T27: through the fake io, which answers only the `--` form; the path-reading hole itself is not shown with real git).
  In-suite: DAY_MUTANTS 16 rows, all caught (0 survivors).
- line refs at the end (worktree files, uncommitted): routine-guard.mjs header :9-17; day helpers :63-113 (isWin/fold/CANON/protectedFold/relIn :64-75, gitPlace :77, kitPath :94, kitDir :108); ask :314-315; bash() day path :470, :476-480, :502-507; writtenBy :516; dayShellWrites :565; git() input param :633, mainFolderMove call :698; protectedHit :709, protectedInBranch :716; WT-DAY comment :722, sameDir, switchWords :727, mainFolderMove :741; gh() vcwd/input :781, pr checkout/co :808-809. guard.mjs header :8-11, C5 comment :17-18, print :26-37 (ask shape :34-35), dispatch :44-45. routine-guard.test.mjs section 5 :483-703 (dayIo :490, DAY_CASES :520, T17 :599, A23 :602, A24 :610, A26 :613, A37 :628-641, launcherWith :642, guard mutation :652, DAY_MUTANTS :661, day mutation test :681). settings.test.mjs :201-217 (S2-S5).
- B9: answer written from these notes.
DONE
