# security-auditor: CI-TIME tier-2 read (af127c2 vs main b9aad01), 2026-11-27
## Plan
1. Diff af127c2 vs b9aad01: `.github/` part must be toolchain.yml:38 15 -> 25 only; sha256 after = d1dd16d7...
2. Read toolchain.yml whole: triggers, permissions/token, network cut, upload step after it, Playwright stops,
   cache, concurrency. Weigh what 10 more minutes changes for a stuck or hostile run.
3. Verdict + findings (severity per CLAUDE.md).

## Notes
- Step 1: `git diff --stat b9aad01 af127c2`: 8 files; 7 are `.checkpoints/ci-time/*` (notes, deleted before ready),
  1 is `.github/workflows/toolchain.yml | 2 +-` (hunk @@ -35,7 +35,7: `timeout-minutes: 15` -> `25`). One line.
- `git diff --raw --no-renames b9aad01 af127c2 -- .github`: one entry, `:100644 100644 4f2e88d9 b19cadf2 M
  .github/workflows/toolchain.yml`; changed-line count 2 (one -, one +). gate.yml untouched. sha256 of the worktree
  file = d1dd16d7eb850a8b27118eab51008b2e8ad97fcded27c468fa1d310f080ff11c (matches the brief). Mode unchanged.
- Step 2 (toolchain.yml at af127c2, read whole):
  - Token: workflow `permissions: contents: read` (:20-21); checkout `persist-credentials: false` (:56); GH_TOKEN only
    in the verify-asset step (:96-104), before the cut. No secrets, no id-token. Unchanged by this diff.
  - Source (read 2026-11-27): docs.github.com/en/actions/concepts/security/github_token: "The `GITHUB_TOKEN` expires
    when the job finishes or after its effective maximum lifetime" (hosted: up to 6 h). So the token's life grows by at
    most 10 min, and only on a run that really runs that long; scope contents:read on a public repo = data already
    public. Confidence: high.
  - Network cut (:135-213): unchanged. A stuck/hostile run inside the cut gets 10 more minutes with no network and no
    token in its env (env -i). Lingering processes (CUT-ESCAPE, accepted, memory) can touch later steps whatever the
    limit; the window that matters is the upload step's own runtime, not the job limit. No change in kind.
  - Upload after the cut (:222-243, same-repo PRs only): on a timeout the job is cancelled and the upload never runs
    (no artifact; preview consumer sees no success). Same as at 15. Extra time does not let cut code reach the upload
    in a new way.
  - Playwright stops (:205, :212): two `--global-timeout=300000` (5 min each) stay. At 15 the two stops (10 min) plus
    builds/tests (head 5c6738a took 13 min) could reach the job limit first, so the comment at :201 "(so before the
    job)" was not reliably true; at 25 a hung browser run is stopped by Playwright with a named failure before the job
    is killed. Small positive for diagnosability, no weakening.
  - Merge check: a timed-out job still concludes non-success, so a stuck run still blocks; the limit only moves when it
    fails (25 vs 15 min). No way around the check or branch protection.
  - Availability: cancel-in-progress per ref (:23-25); push + pull_request = up to two runs per push; a stuck run holds
    a hosted runner slot up to 25 min instead of 15 (queue delay for other runs). Public repo: standard hosted minutes
    free (advisor checked visibility public), so no tier-3 cost. Who can trigger: collaborators' pushes; fork PRs get a
    read-only token and no secrets (fork-approval setting not checked here). Accepted, no finding.
  - 25 is far under the 360 default/6 h hosted maximum; limit stays explicit.
## Verdict: APPROVE, no findings (no Critical/High/Medium/Low). The .github diff is the one line Mustafa approved.
