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
  - Network cut (:135-213): unchanged. Inside the cut: no network, no token in env (env -i). But the cut is no
    sandbox (CUT-ESCAPE, accepted, memory): cut code can plant into later steps (GITHUB_ENV/BASH_ENV/_actions/lingering
    processes), and those post-cut steps have the network and can stall up to the job limit, so the extra 10 min are
    also open to a hijacked post-cut step. Same accepted class (contents: read, no secrets, no id-token): the memory's
    bar for a High is not met.
  - Source (read 2026-11-27): docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows:
    push GITHUB_SHA "Tip commit pushed to the ref"; pull_request GITHUB_SHA "Last merge commit on the GITHUB_REF
    branch", and pull_request_target runs "in the context of the default branch ... rather than in the context of the
    merge commit, as the pull_request event does". So the workflow file (and its timeout-minutes) comes from the
    pushed/merge commit: a hostile author sets their own limit; main's 25 binds only honest or stuck runs. The page
    does not say "workflow file" in so many words. Confidence: medium-high. Also: "a maintainer with write access may
    need to approve running workflows" on fork PRs.
  - Upload after the cut (:222-243, same-repo PRs only): on a timeout the job is cancelled and the upload never runs
    (no artifact uploaded). preview.yml is not in this tree (lane 2's): its handling of a cancelled run NOT read. Same
    as at 15. Extra time does not let cut code reach the upload
    in a new way.
  - Playwright stops (:205, :212): two `--global-timeout=300000` (5 min each) stay. Bound: they fire before the job
    limit while the non-browser part stays under 15 min at 25 (under 5 min at 15). The 13-min run of 5c6738a is the
    brief's figure, not measured here (unverified). No weakening.
  - Merge check: a timed-out job still concludes non-success, so a stuck run still blocks; the limit only moves when it
    fails (25 vs 15 min). No way around the check or branch protection.
  - Availability: cancel-in-progress per ref (:23-25); push + pull_request = up to two runs per push; a stuck run holds
    a hosted runner slot up to 25 min instead of 15 (queue delay for other runs). Public repo: standard hosted minutes
    free (advisor checked visibility public), so no tier-3 cost. Who can trigger: collaborators' pushes; fork PRs get a
    read-only token and no secrets (fork-approval setting not checked here). Accepted, no finding.
  - 25 is far under the 360 default/6 h hosted maximum; limit stays explicit.
## Verdict: APPROVE, no findings (no Critical/High/Medium/Low). The .github diff is the one line Mustafa approved.
- Process slip: my second Bash call ran `sha256sum ...; git diff ...` (git not starting its own command). Later git
  calls ran alone. Not verified: fork-approval setting, the 13-min figure, preview.yml, PR #78 remote head = af127c2.
- Memory appended (outside notes): .claude/agent-memory-local/security-auditor/MEMORY.md (CI-TIME decision).
