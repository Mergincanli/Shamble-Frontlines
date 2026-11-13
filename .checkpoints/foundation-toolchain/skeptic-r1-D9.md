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
- C5 the cancelled run's job logs (actions/jobs/117435551501/logs and /117435551384/logs, HTTP 200, in
  .gate/skeptic-D9/logs): gate printed "GATE PASS (5a smoke skipped: --no-smoke)" at 15:43:26Z against base 5ffe679;
  smoke printed "[5a] smoke PASS index.html 8/8 ... GATE PASS (smoke only...)" at 15:43:59Z. So on 04c086c the checks
  passed in the log while the check runs read `cancelled`: what branch protection, the PR page and `gh run list`
  show is "cancelled", not green. Reading a cancelled job's log can rescue a verdict only when the job ran to its end.
- C6 docs.github.com, read 2026-11-12 23:59 UTC through docs.github.com/api/article/body (exact markdown, .gate/skeptic-D9/docs-dl):
  "Control the concurrency of workflows and jobs" :11 "at most one running job or workflow in a concurrency group at
  any time ... By default, any existing `pending` job or workflow in the same concurrency group will be canceled and
  the new queued job or workflow will take its place"; :13 cancel-in-progress true "also cancel[s] any currently
  running job or workflow in the same concurrency group", and may be an expression; :15-20 a `queue` property:
  `single` (default) or `max` (up to 100 pending), and "The combination of `queue: max` and `cancel-in-progress: true`
  is not allowed"; :127-133 the fallback example `group: ${{ github.head_ref || github.run_id }}` "cancels in-progress
  jobs or runs on `pull_request` events only"; :156 the release/ expression the finding quotes. The finding's quote
  ("any currently running job or workflow in the same concurrency group") and its example hold.
  "Workflow cancellation reference" :7 "the server re-evaluates `if` conditions for all currently running jobs ...
  `if: always()` would evaluate to true and the job continues to run"; nothing on a job not yet started. Holds as
  quoted. "Evaluate expressions" always: "returns `true`, even when canceled", with the WARNING to use
  `if: ${{ !cancelled() }}` to "run a job or step regardless of its success or failure".
- C7 the cited docs in the repo, at 4d78ad6 (only .checkpoints/ differ in the worktree): ADR 0002 part 9 starts :562;
  the G1 bullet starts :593 and its aggregate job (`if: always()`, "fails unless every job it needs succeeded") is
  :595-596; "concurrency cancel" is :605-606 (Hygiene, "as today's CI"). advisor.md:273-276 at the head (272-275 on
  main): "Pinned: the red main commit's first parent is green (its CI, or the full gate run on it in a worktree); the
  re-run is that same commit's run, once (a day session, with an ask, or Mustafa)". A10 = report Appendix A10 /
  DECISIONS.md:123-139, Mustafa 2026-11-12: "Any other change to the CI job asks me again." gate.yml is under
  `.github/` (protected). All citations hold; "no edit in this PR" is right.
- C8 the kit already knows main runs get cancelled: advisor.md:308 at the head (307 on main), "Check that CI ran to
  the end on the revert's main commit; re-run it once if it was cancelled." The pinning step (273-276) has its own
  fallback (the full gate in a worktree). So the cost the finding names is real but already has a written workaround.
- C9 reach today and in 2.3 (gaps between consecutive main push runs, 2026-10-14 to 2026-11-12, 45 gaps): under
  60 s: 1; under 120 s: 2; under 300 s: 3; under 600 s: 5; under 900 s: 6. With today's ~48 s median gate run, one
  cancel in 46 runs, as seen. 2.3's one workflow (Rust native, wasm32, Playwright, determinism jobs) runs minutes, so
  the same merge rhythm would put 3 to 6 of 45 merges inside the window: this is why it is a 2.3 note. The toolchain
  job is not required and the Revert rule names only `gate` and `smoke`, so toolchain.yml's own cancel has no reach
  on that rule today.
- C10 the "red or no verdict" step, as a two-way reading: the cancel reference (:7) keeps a RUNNING `always()` job
  running; if the aggregate job is already running, its needs are done and its verdict is real; if needs are still
  running, they are cancelled (no `if` = success(), false once cancelled) and the not-yet-started aggregate either runs
  (always() is "true, even when canceled") and fails on `cancelled` results (red), or is cancelled itself (no
  verdict). C4 adds a third shape: needs whose steps all passed but whose result reads `cancelled`, which an aggregate
  that "fails unless every job it needs succeeded" turns red. Not green in every branch: the step holds.
- C11 "on purpose": plan.md:270-272, "`cancel-in-progress: true` (as `gate.yml`; so a planted failure is left red
  before its revert is pushed: Part 7)", and :486-490. Holds. "(with an ask)": rests on advisor.md:276's own words
  ("a day session, with an ask, or Mustafa"); routine-guard.mjs (1517 lines) has no rerun rule (grep rerun|re-run|
  gh run: nothing), so the guard itself was not shown to ask. Not verified beyond the kit's text.
- C12 the fix, bullet 1, read from the docs (C6 :11, :17), NOT observed (I cannot push; C2 has no three-in-a-row
  case): with `cancel-in-progress: ${{ github.ref != 'refs/heads/main' }}` and the group unchanged, main keeps ONE
  group per workflow with the default `queue: single`. Merges A, B, C inside one run: A runs, B waits pending, C
  arrives and "any existing `pending` ... will be canceled and the new queued job or workflow will take its place":
  B ends cancelled with no job started. So it fixes the two-merge case seen on 04c086c, not three or more; and as
  written main's runs go one at a time (B's verdict waits for A's whole run). Its words and its example differ:
  "Cancel only pull-request runs" vs an expression that is true for every non-main ref, branch pushes included
  (refs/heads/claude/..., refs/pull/N/merge): it cancels everywhere but main. Nothing it changes breaks a check
  name or the required checks.
- C13 the constraint a complete fix must meet: main's runs must not share one group (the group carries the commit
  on main), or the group must hold more than one pending run. Candidates for 2.3 to prove, not tested here: (a)
  `group: <name>-${{ github.workflow }}-${{ github.ref == 'refs/heads/main' && github.sha || github.ref }}` with
  `cancel-in-progress: true` kept (branch and PR runs cancel as today; main runs never touch each other and run in
  parallel); (b) the docs' fallback `${{ github.head_ref || github.run_id }}` (:127-133) needs the workflow prefix
  (:137) and stops cancelling branch push runs; (c) `queue: max` is a validation error beside `cancel-in-progress:
  true` (:20) and the docs do not say whether either may be an expression beside the other: do not lean on it.
- C14 the fix, bullet 2 (planted failures in --prove's throwaway worktrees, or wait for each run, a revert's
  included): supported by skeptic-r1-D3.md check 3 (not re-queried): the four reverts 2ee2bff, 5c64404, b60a996,
  00cee0b each had both push runs cancelled 2 to 4 s later by the next plant's push. The 2.2 plan waited for each
  plant's red but not for each revert's run, so "a revert's included" names the real gap. Holds.
- C15 corrections to IE-5's text if carried into the report (not new findings): (1) "The cause is inferred from that
  timing; the API gives no reason for a cancel" -> GitHub's annotation on both jobs: "Canceling since a higher priority
  waiting request for gate-gate-refs/heads/main exists" (C3); (2) "left without a completed check" -> without a green
  check: the check runs are completed/cancelled, and in that run both jobs printed GATE PASS (C4, C5); (3) ADR :593 is
  the bullet's start, `if: always()` is :595.

## Verdict
Holds, Low. Every fact it rests on re-checks (config C1, the cancelled main run C2, its cause read from GitHub's own
annotation C3, docs C6, ADR and Revert-rule cites C7, A10, the plan's deliberate copy C11). Low, not Medium: at this
head `toolchain` is not a required check, the Revert rule names only `gate` and `smoke`, the pinning step already has
a written fallback (a full gate run in a worktree) and the kit re-runs a cancelled revert commit (C8); the PR is asked
to change nothing. C9's larger reach in 2.3 is a projection (2.3's run time is unknown) and does not lift it. Fix:
right in direction and right to defer (A10; gate.yml protected), but bullet 1 is incomplete (the default single
pending slot still cancels the middle of three close merges, C12), its words and example differ, and it serializes
main; the 2.3 entry should state the constraint (C13). Bullet 2 holds (C14).
