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
- F1 (REQUIRED-CHECK answered): `toolchain` is NOT required on main. Unauthenticated API 2026-11-12: /branches/main
  protected=true, classic protection enabled=false, contexts []; /rules/branches/main: ruleset 25069670 "Shamble Ruleset 1"
  (active, ~DEFAULT_BRANCH, updated 2026-10-17) requires gate + smoke only (integration 15368). Classic /protection 404
  (needs admin). bypass_actors shows [] to an anonymous caller (may be redacted; not concluded). Also the guard's merge
  check (routine-guard.mjs:1486, ['gate','smoke']), CLAUDE.md:130 merge rule, CLAUDE.md:119 + advisor.md:273 rollback
  trigger, STATUS.md:22 name only gate+smoke: a red toolchain blocks nothing mechanically until 2.3 (advisor.md:323's
  "failing ... checks" is the only text that covers it).
- Source (read 2026-11-12): rustup CHANGELOG (raw.githubusercontent.com/rust-lang/rustup/main/CHANGELOG.md): 1.28.0
  (2025-03-04) removed implicit install, "run `rustup toolchain install` with no arguments"; 1.28.1 (2025-03-05)
  "Automatic install is enabled by default but can be opted out by setting RUSTUP_AUTO_INSTALL ... to 0"; 1.28.2 adds
  `rustup set auto-install disable` (lower priority than the env var). The workflow's :21 comment is right.
- Checked: all 14 planted red runs completed as failure at the claimed step (none cancelled); all 8 reverts exact
  (git diff <plant>^ <revert> empty); fb17451..66d9888 empty outside .checkpoints. But 4 revert pushes (2ee2bff, 5c64404,
  b60a996, 00cee0b) had toolchain AND gate push runs cancelled and no pull_request run; 2d9766f's gate push run cancelled.
  Branch total: 82 runs; gate 37 success / 5 cancelled; toolchain 22 success / 14 failure (planted) / 4 cancelled.
- M2 log (job 130098974545): without the file, `rustup toolchain install` = "using existing install for stable ...
  active because: it's the default toolchain"; then `pin=$(sed ...)` exits 2 under bash -e before the script's own message.
- E4c verified by CI logs: gate job "added 9 packages" on main (job 131211586861, run 43226692719 on 2aa5e9c) and on the
  head (131510254606); smoke "added 159 packages" both (131211586912 / 131510255220), Playwright cache key
  playwright-chromium-Linux-1.56.1 both, smoke 8/8 + 5/5 both. npm's own localPrefix rule: --prefix wins; a workspace
  root only claims a child listed in its workspaces (packages/*), never tools/.
- E5 verified: git diff origin/main 4d78ad6 -- .github/workflows/gate.yml = 0 lines; tools/ unchanged; no root .npmrc.
- E6 re-run: `node tools/gate.mjs origin/main --no-smoke` in the worktree: GATE PASS, symbols +0 -0, base 2aa5e9c.
- Timing (22 green toolchain jobs on the branch, API): 19-45 s, median 25 s; rustup install step 8-14 s (median 10 s,
  downloads 6 components every run, no cache); cut step 4-6 s. Report E2 says "about 20 seconds" (the minimum). Nit.
- Head runs: toolchain push 43312017234 (job 131510223737, 26 s) and PR 43312027194 (job 131510257155, 30 s): log
  matches the report's E2 description (rustup "overridden by" the file, pin equal, control connected 140.82.114.4,
  uid 1001 gid nogroup, only lo, curl (7), cargo 6/6, release 2/2, wasm32 build, npm ci --offline 46 pkgs, 5/5, tsc 6.0.3).

## Resumed after the restart (2026-11-12, second context)
- Worktree HEAD 3f48773 (saver commits); `git diff 4d78ad6 HEAD -- . ':!.checkpoints'` empty: the reviewed head holds.
- gate.yml at the head: same triggers (push + pull_request), same concurrency (`cancel-in-progress: true` on every ref,
  main included), same action SHAs as toolchain.yml (checkout 3d3c42e5 v7.0.1, setup-node 82076278 v7.0.0).
- ADR 0002 read: part 2 (:157-182: "that root lockfile is the one CI's npm cache keys on (cache-dependency-path)"),
  part 9 (:562-655: G0-G13, option G1 aggregate check, caching, "concurrency cancel"), migration 2.2 and 2.3
  (:1449-1465: 2.3 done-when includes a fork PR with no secret, a cancelled job turning the check red, his branch
  protection), PM-0002-37 (:1303, frequency "1 run"), PM-0002-57 (:1323).
- STATUS_ARCHIVE "From foundation 2.2" (:551-640) read: no item carries the network cut (or K1b's release test) into
  2.3's workflow; CARGO-NET-OFFLINE assumes the cut persists.
- Head log (job 131510223737): image ubuntu-24.04 20261111.370.1, runner 2.338.2; rustup prints no self-update line;
  "it's active because: overridden by '.../rust-toolchain.toml'".
- Source (read 2026-11-12): runner-images Ubuntu2404-Readme.md at tag ubuntu24/20261111.370 (the head job's image):
  Rustup 1.29.3, Rust 1.99.1, Cargo 1.99.1 (the image's own stable equals the pin today, so `rustc` equal to the pin
  alone proves nothing: the workflow's :50 `case *rust-toolchain.toml*` check is what tells them apart), Node.js
  22.23.4, sudo 1.9.15p5. install-rust.sh at that tag: rustup with stable, minimal profile, rustfmt + clippy; no
  auto-self-update setting.
- Source (read 2026-11-12): rustup CHANGELOG 1.29.0 (2026-03-05): pr#4566 "add missing self-update in `rustup toolchain
  install`"; pr#4522 `no_update` flag; "Empty environment variables are now treated as unset" (pr#4422). The head log
  shows no self-update line (rustup 1.29.3 may have been current). Candidate Low: `--no-self-update` (dtolnay/rust-toolchain
  passes it) so the image's rustup is the one that runs.
- Source (read 2026-11-12): docs.github.com "Events that trigger workflows": fork PRs: "The `GITHUB_TOKEN` has read-only
  permissions in pull requests from forked repositories"; "secrets are not passed to the runner when a workflow is
  triggered from a forked repository"; GITHUB_REF refs/pull/N/merge, GITHUB_SHA the merge commit; "Workflows will not
  run on `pull_request` activity if the pull request has a merge conflict"; first-time contributors may need approval.
- Source (read 2026-11-12): docs.github.com "Managing GitHub Actions settings for a repository": "By default, all
  first-time contributors require approval to run workflows" (options: new to GitHub / first-time / all external);
  "Require actions to be pinned to a full-length commit SHA" exists as a setting.
- Source (read 2026-11-12): docs.github.com "GitHub Actions billing": standard hosted runners free "In public
  repositories"; Linux 2-core x64 $0.006/min (SKU actions_linux). Repo is public (API: private False), forks_count 0.
- Source (read 2026-11-12): docs.github.com "GitHub-hosted runners reference": "The Linux and macOS virtual machines both
  run using passwordless `sudo`"; each runner (but single-CPU) "is a new virtual machine"; x64 Linux public: 4 CPU,
  16 GB RAM, 14 GB SSD. The report's Risks claim (passwordless sudo documented) holds.
- Actions settings unreadable without a token: /actions/permissions, /fork-pr-contributor-approval, /workflow,
  /selected-actions all HTTP 403 (2026-11-12). Fork approval setting and the SHA-pin requirement: not read.
- Re-checked the cancelled revert runs (API, 2026-11-12): 2ee2bff, 5c64404, b60a996, 00cee0b: only push runs, both
  workflows cancelled (each revert committed 2 s before the next plant, pushed separately); 2d9766f: gate push
  cancelled, gate pull_request 42893520512 success; 66d9888 (M4 revert): all 4 runs success. So E4c/E5's "green on
  every push" is not literal (4 pushes never had a completed gate run); the substance (gate.yml unchanged, every
  completed gate/smoke run green, the head green) holds. Candidate Low.
- ADR part 4 vs Cargo.toml: release overrides for sim + protocol present (the CI release test covers them); part 2 pins
  as built (channel 1.99.1, minimal, clippy, rustfmt, wasm32 target; engines node 22.x; exact devDependencies).
