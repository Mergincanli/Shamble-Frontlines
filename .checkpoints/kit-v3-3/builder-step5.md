# Task: kit-v3-3 builder, plan rev2 section 8 step 5 (read-only Actions tools for pr-advisor, owner/repo check, C7)
## Plan (blocks)
B1 read plan rev2 sections 0,4,5,8 + advisor conditions + step3/4 notes
B2 schema check: ToolSearch actions_get/actions_list method enums vs ACTIONS_READ
B3 code edit in routine-guard.mjs
B4 tests added
B5 test run + counts
B6 mutations in .gate/kit-v3-3/
B7 answer
## Notes
- B1 done: scope = plan 4.1 ACTIONS_READ (actions_get: get_workflow, get_workflow_run, get_workflow_job, get_workflow_run_usage; actions_list: list_workflows, list_workflow_runs, list_workflow_jobs, list_workflow_run_artifacts) after GITHUB_READ, check in mcp() before the general read path: role + name in ACTIONS_READ -> owner/repo REPO else deny "routine (<role>): GitHub reads only on <REPO>"; night -> deny "not on the night shift's GitHub list"; pr-advisor -> null only listed method else deny naming methods. Tests group "Actions reads (item 3)" + mutation rows (method check, owner/repo check). Step 4 notes: existing CASES assert !!d===refused; MUTANTS run over CASES. Next B2 schema check.
- B2 done (schemas loaded with ToolSearch, neither tool called): actions_get method enum = get_workflow, get_workflow_run, get_workflow_job, download_workflow_run_artifact, get_workflow_run_usage, get_workflow_run_logs_url (6); actions_list enum = list_workflows, list_workflow_runs, list_workflow_jobs, list_workflow_run_artifacts (4). ACTIONS_READ = actions_get enum minus download_workflow_run_artifact and get_workflow_run_logs_url, plus actions_list enum whole: MATCHES plan 4.1 (no name outside the enums, no enum read left out except the two on purpose). Both schemas require method, owner, repo. actions_run_trigger enum = run_workflow, rerun_workflow_run, rerun_failed_jobs, cancel_workflow_run, delete_workflow_run_logs (plan's test list lacks rerun_failed_jobs: I add it). Next B3 read mcp() site.
- B3 done (edit, routine-guard.mjs): ACTIONS_READ + actionsRead() after GITHUB_READ; mcp() dispatch line before the general read path, role only (day path untouched). owner/repo compared separately, case-folded (stricter than the joined string). Line refs in B7. Next B4 tests.
- B4 written: CASES +32 Actions rows (after the rev-list section), MUTANTS +6, one reason test. 0 removed lines in either test file. Next B5 run.
- B5 done: routine-guard 317/317 (284 + 32 CASES + 1 reason test; MUTANTS rows run inside the existing mutation test), settings 124/124 (unchanged), combined 441/441. Next B6 scratch mutations.
- B6 done (script .checkpoints/kit-v3-3/mut-step5.mjs; copies .gate/kit-v3-3/mut-step5-0..7/, git-ignored):
  guard before step 5 + new tests: 306/317; fails the 8 listed-method null cases, owner-case null case, the reason test, the mutation test (new MUTANTS targets absent). The deny cases pass on the old guard (it refused every actions_* for a role via githubWrite).
  Actions method check off (plan row): 308/317; fails download_workflow_run_artifact, get_workflow_run_logs_url, unknown method x2, no method x2, list method on actions_get, reason test, mutation test.
  Actions owner/repo check off (plan row): 310/317; fails another repo x2, same owner other repo, owner holding the slash, no owner/repo, reason test, mutation test.
  extra: night refusal off 313/317 (night actions_get/actions_list); logs URL added to list 315/317; artifact download added 314/317; dispatch for day sessions too 314/317 (day download, day another repo).
  finding: mutant "owner/repo as one joined string" survives 317/317: an EQUIVALENT mutant (REPO has one slash, so the joined compare and the separate compare agree on every string input; no distinguishing case exists). The separate compare is a readability choice, not extra strength; not added to MUTANTS.
  In-suite MUTANTS: 6 new rows, all caught (mutation test passes).
- line refs (worktree, uncommitted): routine-guard.mjs comment :816-818, ACTIONS_READ :819-822, actionsRead :823-833 (owner/repo deny :824-827, night deny :828, method check :829-832), mcp() dispatch :842 (before the general read path, role only). routine-guard.test.mjs CASES Actions section :293-343 (32 rows), MUTANTS +6 :508-513, reason test :515-526. settings.test.mjs untouched. guard.mjs untouched.
- B7: answer written from these notes.
DONE
