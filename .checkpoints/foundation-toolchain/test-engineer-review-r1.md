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

### Block 4 DONE (unchanged-since claims)
- workflow fb17451..4d78ad6: exactly the K1b line added. Pins/locks/crates/packages/.gitignore 566363f..4d78ad6: only
  the two overflow tests' lines (70f1c31). gate.yml equals main. M1a-M4 era: no workflow/pin change between fb17451 and 2d9766f.

### Block 2b DONE (all 40 toolchain runs on the branch; 82 runs total)
- 30 runs reached the cut: 30/30 "the network is cut ... (curl exit 7)", 30/30 uid 1001 gid/groups nogroup only,
  30/30 only `lo`. 0 real "still reachable"/"inconclusive" outputs (matches are the echoed script only).
- Every failure is a planted commit (7 x push + PR = 14); 5 toolchain + 5 gate runs cancelled by the next push
  (concurrency cancel-in-progress: reverts 2ee2bff, 5c64404, b60a996, 00cee0b had no completed run; 66d9888, M4's
  revert, green 42893708734/42893718519 with a tree equal to fb17451's for every toolchain file). run_attempt 1 for
  all 82: nothing re-run into green. No unexplained red: no flake.

### Block 5 (local re-runs in /home/user/Shamble-Frontlines/.gate/te-r1/, git archive of 4d78ad6) - partly DONE
- head: cargo test --workspace --locked 6/6; cargo test --release -p sim -p protocol --locked 2/2 (same test-binary
  hashes as CI); wasm32 build of the five crates OK.
- control nosim (sim's release block removed, protocol's kept): CI's exact K1b line -> sim "did not panic as expected
  at crates/sim/src/lib.rs:10:8", exit 101 (on the head's stricter test, after 70f1c31). So the K1b line does catch
  sim's block; CI never showed it because M5 removed both and protocol's binary runs first.
- control noproto: protocol FAILED, exit 101. bench profile: head exit 0; nosim exit 101 (bench inherits the override).
- L1 on all six crates (unsafe {} in each, cargo build --workspace --keep-going): 6 x "usage of an `unsafe` block", exit 101.
- gate: node tools/gate.mjs origin/main --no-smoke -> GATE PASS, symbols +0 -0 (E6 holds). Smoke job on the head
  131510222694: 8/8 and 5/5, tools installed with npm ci --prefix tools (159 packages) (E4c, E5 hold).
- E11/E7 recount: lock v3, 75 `packages` entries = root + 5 workspace dirs + 5 links + 64 registry (all sha512);
  hasInstallScript only fsevents; root devDependencies exact (3); the five packages have no dependencies.
- E1 pedantic clippy (cargo clippy --workspace --all-targets --locked -- -W clippy::pedantic): exit 0, clean.
- E8: repo-map-check "repo map OK: 120 entries cover 107 required paths", exit 0. E10: the three kit test files on
  the branch: 1261 pass, 0 fail (= 1107 + 127 + 27). E4a: git grep finds only a legacy variable `spikes`; no .cargo/.
- E4b reproduced (head export + `git archive 1efd46f spikes/netcode-proof`): cargo metadata --no-deps --offline from
  the top -> 6 crates; from the spike -> its own root, 16 npt-* crates.
- E9 reproduced with main's guard (protectedRel/mergeAllowed from the main folder at 2aa5e9c): 42 files, 3 protected
  (advisor.md, toolchain.yml, CLAUDE.md), 5 mergeAllowed, 34 neither.
- Kit texts: main's CLAUDE.md/advisor.md and fb17451's workflow + k-texts.mjs EDITS == head files, sha256 EQUAL (3/3).
- Code unchanged 70f1c31..4d78ad6 (crates, packages, pins, locks, .github, .gitignore).

### Block 6 DONE (branches that CI never saw red; scripts in /home/user/Shamble-Frontlines/.gate/te-r1/branches/)
- drive.py runs toolchain.yml:46-53 (pin check) and :76-81 (probe) verbatim with fake rustup/rustc/curl:
  11/11 pin cases as designed (S1 stable, S2 1.99, S3 RUSTUP_TOOLCHAIN, S4 directory override, S5 default, S6 1.99.0,
  S7 1.99.10 prefix trap, S8 two lines, S9 CRLF -> exit 1; S10 missing file -> exit 2 (sed); S0 control -> 0);
  6/6 probe cases (rc 0 -> exit 1 "still reachable"; 7, 28 -> pass; 6, 35, 127 -> exit 1 "inconclusive").
- Never seen red in CI: the probe's rc=0 branch (no run without --net), the pin check's branches other than a
  missing file, a failing Playwright spec, a wasm32-only compile error. The report claims none of them proven.

## Findings so far (severity; to be finalised)
- TE-1 Low: "green on every push" / "green on every head" (As built :58-60, E4c :106, E5 :108, K1-GO :206) is looser
  than the data: 4 revert pushes (2ee2bff, 5c64404, b60a996, 00cee0b) have no completed toolchain, gate or smoke run
  (cancelled by the next push); M4's gate push run cancelled (PR twin green). Substance holds (trees equal to green
  fb17451/66d9888; no unplanned red).
- TE-2 Low: "The job takes about 20 seconds" (E2 :96): 16 green jobs 19-45 s, median ~24 s; "69 entries" (E11 :114):
  the lock's `packages` has 75 (root + 5 workspace folders + 5 links + 64 registry).
- TE-3 Low: sim's release overflow block is CI-unproven (M5 removed both, protocol's binary fails first); my local
  sim-only control on the head's stricter test: exit 101 with K1b's exact line. A one-commit M5b (sim's block only)
  or 2.3's --prove closes it; the report states the gap honestly.
- TE-4 Low: four checks never seen red in CI (list above); locally simulated, each bites; carry one mutation each
  into 2.3's --prove (PM-0002-52 needs one mutation per check id).
