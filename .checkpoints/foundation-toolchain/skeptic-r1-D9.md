# Skeptic, review round 1, D9 (= IE-5, Low): head 4d78ad6

Plan: (1) re-read the concurrency blocks of toolchain.yml and gate.yml at 4d78ad6 and on origin/main; (2) re-query
main's push runs through the REST API (count, conclusions, the cancelled one and its neighbour) and look for GitHub's
own cancel annotation on the cancelled run's jobs, so the cause is read, not inferred from timing; (3) re-read the
cited docs (docs.github.com concurrency page, workflow cancellation reference, the always() expression notes) in my own
folder .gate/skeptic-D9; (4) re-read ADR 0002 :593 and :605, advisor.md:273-276 (Revert) and A10, and the guard's
rule on re-runs; (5) judge holds and severity on CLAUDE.md's scale; (6) test the proposed fix against GitHub's
pending-run rule (one running and one pending per group) and name what it would break or miss. Nothing in the
worktree changes but this file.

## Checks
- C1 the config at 4d78ad6 (git show): toolchain.yml:16-18 `group: toolchain-${{ github.workflow }}-${{ github.ref }}`,
  `cancel-in-progress: true`; gate.yml:15-17 the same with `gate-`; origin/main (2aa5e9c) has only gate.yml, same
  block. Unconditional, keyed by ref: every push to main shares one group per workflow. Commits after 4d78ad6 touch
  only .checkpoints/ (`git diff --name-only 4d78ad6 HEAD`, nothing outside). Holds.
- C2 main's push runs (REST API, read 2026-11-12, files in .gate/skeptic-D9/api): total_count 46; 45 gate success,
  1 gate cancelled: run 39046286643 on 04c086c (merge of PR #21), created 15:43:17Z 2026-10-17, run_attempt 1 (never
  re-run). df1e6d3 (merge of PR #20, first parent 04c086c) got run 39046292180 at 15:43:22Z, 5 s later, success. It is
  the only gap under 60 s among consecutive main runs (next: 63 s, 120 s; median run 48 s). The evidence holds.
- C3 the cause is READ, not inferred: both jobs' check-run annotations (check-runs/117435551501 and /117435551384
  /annotations) say "Canceling since a higher priority waiting request for gate-gate-refs/heads/main exists". So the
  finding's "the API gives no reason for a cancel" is wrong in its own favour: the API does give it.
- C4 a detail the finding does not state: in that cancelled run every step of both jobs ended `success` (gate's Gate
  step 15:43:25-26Z, smoke's Smoke step 15:43:44-59Z), yet each job's and check run's conclusion is `cancelled`. So
  04c086c shows no green gate or smoke on main, although both checks did pass (on the job logs' reading, see C5).
