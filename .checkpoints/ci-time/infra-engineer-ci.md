# infra-engineer: CI review of af127c21 (toolchain.yml timeout 15 -> 25)

## Plan
1. Diff is one line (done: `git show af127c21 -- .github` = line 38 only; second file is the advisor notes).
2. Validity: job-level `timeout-minutes` max (GitHub docs: 360 min on GitHub-hosted).
3. Read recent toolchain runs (5c6738a): total, cut step's time, the two Playwright stops.
4. Interactions: concurrency/cancel-in-progress, required checks, preview.yml workflow_run.
5. Cost: public repo, standard runner.
6. Verdict.

## Notes
- Diff: only line 38 in toolchain.yml changes (15 -> 25). Commit also edits .checkpoints/ci-time/advisor-plan.md (notes, not CI).
- Limits (docs.github.com/en/actions/reference/limits, read 2026-11-27): "Each job in a workflow can run for up to 6
  hours of execution time"; a job that hits it is "terminated and fails". Workflow syntax page (read 2026-11-27):
  step timeout-minutes "Maximum: 360 for both GitHub-hosted and self-hosted runners", positive integer. The job-level
  section's own text was NOT read (WebFetch excerpts missed it); proof that GitHub accepts 25: both af127c21 runs
  (46001050475 push, 46001088665 pull_request) started their job (12:43:39 and 12:43:56 UTC), i.e. the file parsed.
- Cost (docs.github.com/en/billing/concepts/product-billing/github-actions, read 2026-11-27): "The use of standard
  GitHub-hosted runners is free: In public repositories"; larger runners are billed. ubuntu-24.04 is standard.
  Visibility read from GitHub REST 2026-11-27: `public`. Cost: 0 USD per run and per month.
- Runs read with gh (2026-11-27 ~12:45-12:55 UTC), job start -> end, cut step:
  - 45918878273 push 5c6738a: 12m59s; cut step 12m20s; before the cut 35s. Chrome start 8m39s after job start,
    "19 passed (3.8m)"; Firefox "5 passed (27.0s)".
  - 45917465670 push ad88341 (14 min earlier, same branch): 9m26s; Chrome start 5m40s after job start, "19 passed
    (3.4m)"; Firefox 15.9s. Release build 25.4s vs 41.5s on 5c6738a. So of 5c6738a's +3.5 min, ~3.0 min is before the
    browsers (compile/tests, likely runner speed plus code), ~0.4 min Chrome, ~0.2 min Firefox.
  - 45999904686 push 19034b7: 8m02s. 45991394502 push 6417fe2: 7m51s.
  - 45999915993 pull_request 19034b7: plain-files check 0s, upload 3s (the only steps push runs skip).
  - af127c21 runs: still in progress at 12:55 UTC; result not read.
- Worst case with both Playwright stops at their 5-minute cap (5c6738a timings): 8m39s + 5m + ~3s + 5m + ~3s upload +
  ~5s post = ~18m55s. At 15 the job limit would cut a hung or slow Firefox run before Playwright's own stop, so line
  201's "(so before the job)" was not true at 15; at 25 it is, with ~6 min margin for build growth.
- Next ceiling: Chrome suite 3.4-3.8 of 5.0 min (68-76%) of its global-timeout with the same 19 tests (~12 s each);
  about 6 more tests of that length reach it, and 25 buys no headroom there. Follow-up, not this diff.
- Concurrency: group toolchain-<workflow>-<ref>, cancel-in-progress true: a newer push cancels the older run on the same
  ref; the timeout does not change that. push and pull_request runs have different refs, so each push to a PR branch
  runs twice (seen on af127c21). Pre-existing; free; doubles runner-slot use.
- REQUIRED-CHECK, read from GitHub REST 2026-11-27 (`rules/branches/main`, ruleset 25069670): required_status_checks =
  gate, smoke only. toolchain is not required: a timeout red does not block a merge mechanically. No interaction.
- preview.yml: not in this branch, not in origin/main at b9aad01 (local ref): no workflow_run consumer today.
- Rollback: revert the one line; no state, secret or provider change. Exposure: none new (a stuck run holds a free
  runner up to 10 min longer).
- Process note: several earlier gh/git calls in this review had `cd ... &&`, a for loop or `R=$(gh ...)` in front,
  against the rule "git and gh always start their own command". Later calls were bare.

## Verdict
APPROVE. L1 (Low, follow-up, decision): the Chrome 5-minute stop is the next limit (68-76% used). L2 (Low,
pre-existing): push + pull_request double runs. L3 (Low): af127c21's own runs not yet read to the end.
