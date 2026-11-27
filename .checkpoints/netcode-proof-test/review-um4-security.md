# UM-4 security read (seat S, launch-1750-msgs.md "S"), security-auditor, 2026-11-27
Commit read: 8fb32bc8 (the `ops` step in spikes/netcode-proof/check.mjs), on branch
claude/netcode-proof-test-plan-hcky5g at b81061cf. Read only: no install, no npm audit, no network, no cargo; this is
the only file written.

## Plan
- Block 1, the two commands: `run()` (check.mjs:1951-1964: spawnSync, no shell, env = process.env), opsStep's two calls
  (check.mjs:2683-2706). Questions: (a) Node's glob `ops/**/*.test.mjs`: what does it select today, does it reach
  ops/browser/node_modules (third-party code run as a test), does it follow symlinks, can a planted file under ops/
  run as a test; (b) `bash` found by PATH, BASH_ENV/ENV, NODE_OPTIONS and other inherited environment; (c) what
  wrappers.test.sh and the ops Node tests themselves run (sudo, systemctl, network, browser/driver downloads,
  SEC-R1D-3); (d) can output forge the pass (the last-line parse, "# tests N").
- Block 2, the install record: dependency-check.md's UM-4 section against security-r1-deps.md (webdriverio 10.2.0
  standalone only; SEC-R1D-2, -3, -6) and SEC-U7D-3; the lockfile's integrity and resolved hosts (read only).
- Block 3, OPS_MISSING (check.mjs:2692-2693): working directory of `--prefix ops/browser`, what the line leaves out.
- Findings with severity, file:line and evidence; the verdict last.

