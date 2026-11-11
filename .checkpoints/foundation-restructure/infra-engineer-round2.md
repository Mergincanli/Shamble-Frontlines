Task: infra-engineer ROUND 2 review of foundation 2.1a (PR #50), diff 3c98ad5...76544aa, focus CI + F1 fix.

Plan (blocks):
B1 read tools/gate.mjs diff + surrounding base-resolution code
B2 read .github/workflows/gate.yml: triggers, base ref for push vs pull_request, required checks
B3 read evidence-f1 files (decoy, move directions)
B4 CI outage question (smoke deps / mirror) + 0607b45 cover
B5 write findings, DONE

Notes:
- B1 done. gate.mjs:243-249 (76544aa): check only when base && base.prefix !== BUILD_PREFIX; reads the WORKING TREE (readdirSync of ROOT/base.prefix), names /i; die = exit 2; runs before `if (smokeOnly)` but base is null there, so smoke job untouched. Cost: one existsSync + one readdirSync, only on a move. Failure modes: (a) if base.prefix names a FILE in head (e.g. a file called `legacy`), readdirSync throws ENOTDIR -> uncaught crash exit 1, still red (fail closed) - Low; (b) local untracked strays trip it (CI checkout clean) - info, clear message.
- B2 done. gate.yml unchanged (.github tree 67ccb30 at 3f29cb8 and 76544aa). PR run: BASE=origin/$BASE_REF (main), working tree = refs/pull/N/merge -> on 2.1b base prefix '' vs head 'legacy/' -> check runs on every synchronize. Push runs after first: BEFORE=previous tip (legacy/) -> no check. First push of a new branch: before=zeros -> merge-base origin/main -> check runs. Push to main after merge: BEFORE=old main tip -> check runs (post-merge net, also catches main-moved race).
- Q1 evidence (public REST, unauthenticated, read 2026-11-11): commits/<sha>/check-runs with default filter=latest returns one gate+smoke per check suite, BOTH suites (push and pull_request) on the head SHA: 3c98ad5 total 4, 76544aa total 4, 3f29cb8 latest 4 / all 8 (latest = newest attempt per suite). routine-guard.mjs:1483-1491 mergeCheck reads that list (default filter) and requires EVERY gate/smoke run completed and success/skipped/neutral with >=1 success -> a red PR-run gate blocks even if the push-run gate is green.
- Revert PR: new branch first push (merge-base = legacy main) + PR run (base legacy main) + push to main all run the check; head keeps 2.1a's gate. Nothing else needed, provided 2.1b is reverted before 2.1a (round 1, plan.md:368-369).
- Q2 evidence: jobs API: failing step on 43054418953 (both attempts), 43054429019 attempt 1, 43060970457 = "Install Chromium's system libraries (browser restored from the cache)" (playwright install-deps = apt), ~10 min each before failing; Smoke step skipped. Live deps per run: apt mirror (every smoke run, cache hit or not), npm registry (every run of both jobs; setup-node has no cache), Playwright CDN (cache miss only); three.js served locally (smoke.mjs:142,173). So "every smoke run depends on a live package mirror" is right; 2.3 should name all three and add a step timeout on install-deps.
- Cover: non-doc/non-checkpoint trees of 3f29cb8, 0607b45, 76544aa identical (git diff --stat empty; tools f23190c; .github 67ccb30) -> 0607b45 is a valid cover; moot anyway: 76544aa itself has gate+smoke success in both suites (completed 17:19:53-17:20:35).
- FINDING R2-1 Low: gate.mjs:247 readdirSync on a non-directory crashes (exit 1, stack) instead of a die(); fail closed.
- FINDING R2-3 Medium->follow-up for 2.3: CI live-mirror dependency (apt every smoke run, npm every run, no step timeout).
- FINDING R2-4 Low/info: ruleset's own handling of duplicate-name runs unverified; the guard's mergeCheck verified to see both suites. 2.1b: merge with its PR run on the current main, since pull_request runs don't re-fire when main moves and the ruleset is not strict; main's push run is the net.
- B3/B4/B5 done.
- B6 (after advisor tool): guard.mjs:42 imports routine-guard decide; routine-guard.mjs:1425 runs mergeCheck for EVERY session's github merge_pull_request (role null = day session) -> all-runs-green holds for day-session MCP merges too. Ruleset 25069670 on main (public REST /repos/<repo>/rules/branches/main, read 2026-11-11): required_status_checks gate + smoke, integration_id 15368 (GitHub Actions), strict_required_status_checks_policy false; pull_request rule; non_fast_forward; deletion. Classic protection off. How the ruleset treats two same-name runs is not documented on an official page I could reach -> unverified.
- R2-2 corrected and dropped: 43054429019 attempt 2 ran 16:26:46-16:42:01 = ~15 min = smoke timeout-minutes 15 -> cancelled by the JOB TIMEOUT (likely), not concurrency (8d8c422's runs started 16:42:30). A failed attempt burns 10-15 min. STATUS.md:101 "failed twice on #50's 3f29cb8" is accurate; the checkpoint's "failed again" is checkpoint-only (deleted before ready). No docs finding.
- e3-compare.txt first line read: 31 of 31 PASS, identical to 2026-11-10. The two raw .json files not opened (their .exit files: 0 and 0).
- Not verified: the DNS cause (only the failing step seen, no log access); no decoy run in the revert direction (code reading only; symmetric join(ROOT, 'legacy/')); ruleset's duplicate-name handling; a merge through gh on the shell (out of scope).
- VERDICT APPROVE.
DONE
