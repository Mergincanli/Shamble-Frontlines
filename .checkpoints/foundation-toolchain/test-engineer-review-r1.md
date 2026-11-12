# test-engineer, review round 1 of foundation 2.2 (PR #48), head 4d78ad61e652f2e35ddf51c49557e725420248c8

Angle: is it proven? Each Outcome and Evidence claim of docs/reports/2026-11-12-foundation-2-2.md against its evidence.
Scratch and builds: /home/user/Shamble-Frontlines/.gate/te-r1/ (my own folder; nothing in the worktree but this file).
Worktree HEAD 4c0dfd5 differs from 4d78ad6 only in .checkpoints/ (checked: git diff --stat 4d78ad6 HEAD).

## Plan (blocks; each marked DONE with its evidence when finished)
1. Read the diff: workflow, Cargo.toml, crates, package.json files, tests; ADR 0002 parts 2, 3, 4, 7, 9 + migration 2.2.
2. CI: run lists for the head 4d78ad6 and every SHA the report names (E2, M1a-M6); per run the failing step.
   Logs read in full: E2 (head), M5, M6; others as needed.
3. Reverts exact: git diff <planted>^ <revert> for M1a, M1b, M2, M3, M4, M5, M6.
4. "Unchanged since" claims: git diff fb17451..4d78ad6 on the workflow; 566363f..4d78ad6 on pins, locks, crates, packages.
5. Local re-runs in a copy (git archive of 4d78ad6): cargo test --workspace --locked; wasm32 build; release tests;
   sim-only and protocol-only removal of the release override (the controls); L1 on every crate; the gate --no-smoke;
   repo-map check.
6. Tests that can't fail / timing / flake review: the probe's logic, the control, the stubs, tsc --version, Playwright.
7. Findings, then the claim/proof/gap table, then the verdict.

## Findings (a line each, as found)

### Block 2 DONE (CI runs and logs; data under /home/user/Shamble-Frontlines/.gate/te-r1/ci/)
- Head 4d78ad6: toolchain push 43312017234 (job 131510223737) and pull_request 43312027194 green; gate 43312016939
  (jobs gate 131510223092, smoke 131510222694) and 43312026745 green. Log read in full: pin check, control, cut
  (uid 1001 gid nogroup, only lo, curl (7)), cargo 6/6, release 2/2, wasm32 build, npm ci --offline 46, vitest 4x1,
  playwright 1 passed, tsc "Version 6.0.3".
- Every E2 run id: toolchain, green, on the SHA the report names (14 of 14). Job durations 19-45 s (median ~24 s).
- M1a 42891819768: red at step 7, devtools planted_network_use FAILED, "failed to lookup address information", exit 101.
- M1b 42892038296: red at step 7, @shamble/net "1 failed", fetch failed EAI_AGAIN.
- M2 42892226674: step 4 fell back to stable ("it's the default toolchain"), step 5 red "sed: can't read
  rust-toolchain.toml", exit 2; steps 6-7 skipped.
- M3 42893284193: red at step 6 cargo fetch --locked, "cannot update the lock file", exit 101; step 7 skipped.
- M4 42893510607: red at step 7, @shamble/types "Missing script: "test"", exit 1.
- M5 43281730739 (job 131404460318): red at step 7 cargo test --release: protocol "did not panic as expected at
  crates/protocol/src/lib.rs:10:8", exit 101; sim's release binary never ran (cargo stops at the first failing binary).
- M6 43310269682 (job 131504204735): red at step 6 npm ci, EUSAGE, "Missing: is-number@7.0.0 from lock file"; step 7 skipped.
- Pull-request twins of every M run also red (42891831298, 42892040651, 42892231842, 42893293995, 42893520814,
  43281743102, 43310283647). Gate runs on M commits green (M4's gate push run 42893510654 cancelled by the next push).

### Block 3 DONE (reverts)
- git diff <planted>^ <revert> empty for all 7 (M1a 1bee16c/2ee2bff, M1b 363c455/5c64404, M2 0d3ec36/b60a996,
  M3 af65262/00cee0b, M4 2d9766f/66d9888, M5 45f1bb5/a5823cf, M6 b764298/d5316d6). Each planted commit touches 1 file.
