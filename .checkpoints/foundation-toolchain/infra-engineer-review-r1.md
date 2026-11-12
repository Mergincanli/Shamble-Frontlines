# infra-engineer, review round 1 of foundation 2.2 (PR #48), head 4d78ad61e652f2e35ddf51c49557e725420248c8

Effort: max (Opus 5.5). Worktree: /home/user/Shamble-Frontlines/.gate/foundation-toolchain-wt (read only, but this file).
No memory folder exists for infra-engineer (checked .claude/agent-memory* in both folders, 2026-11-12).

## Plan
1. Read the report draft, the workflow (toolchain.yml), gate.yml, ADR 0002 parts 2, 3, 4, 7, 9 and migration step 2.2. (done: report, workflow, gate.yml)
2. CI as infrastructure: job shape and steps; triggers (push + pull_request: double runs); concurrency group and
   cancel-in-progress (what it does to planted-failure evidence: were any planted runs cancelled?); timeout; runner image.
3. rustup on ubuntu-24.04: RUSTUP_AUTO_INSTALL semantics (rustup docs/changelog), `rustup toolchain install` with no
   args reading rust-toolchain.toml, the pin check's regexp/case logic, RUSTUP_TOOLCHAIN check, what M2 shows.
4. The cut: sudo unshare, setpriv, env -i, what still inherits (fds, mounts), the probe logic (exit 7/28).
5. Timing per run from the API (run/job durations), cost (public repo => free minutes?), double runs per push.
6. Fork PRs: what a fork's pull_request runs (read-only token, no secrets, sudo on a hosted runner), approval settings.
7. Required check: try the API without a token (rulesets, branch protection); say plainly if unreadable.
8. E4c: the legacy toolkit's install with the root package.json present (npm ci --prefix tools), workspaces resolution
   up the tree; E5: gate.yml equal to main's; gate/smoke jobs green on the head.
9. What 2.3's gate v2 must take over (list for the report, not re-reporting known findings).
10. Run `node tools/gate.mjs origin/main --no-smoke` in the worktree; read CI logs for the head.

## Findings (one line each, as found)
