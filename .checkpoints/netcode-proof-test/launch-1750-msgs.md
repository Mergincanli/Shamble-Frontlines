# The three-seat window after NB2-ONEWAY's push (2026-11-27 17:5x UTC; the advisor tool's ruling of 16:3x)
Status: T and S SENT 17:5x; B after CI on b81061cf reads all success. At most three agents; each saves every finished
block at once; a cut resumes from its own file.

## T: test-engineer (new seat), two blocks, into review-um4-nb2b-test.md
Goal: is UM-4 proven (block 1), and is Builder 2's fix pass proven (block 2, NB2B-8)? Read only.
- Write your plan at the top of /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/review-um4-nb2b-test.md
  first; save each finished block to it at once; last line "Status: COMPLETE <UTC time>". Edit no other file. Run no
  cargo and no check.mjs (the lane lock is the main session's); `node --test spikes/netcode-proof/check.test.mjs` from
  the repository root is allowed.
- Block 1, UM-4 (the `ops` step): `git show 8fb32bc8 -- spikes/netcode-proof/check.mjs
  spikes/netcode-proof/check.test.mjs` (opsStep, OPS_MISSING, OPS_BROWSER_MODULES, five new tests); the plan
  .checkpoints/netcode-proof-test/um4-plan.md (rev 0.3 and its done lines) and its check advisor-um4-plan.md (UM4-1 to
  UM4-8); the logs /home/user/Shamble-Frontlines/.gate/um4/absent.log (the missing-install FAIL),
  /home/user/Shamble-Frontlines/.gate/um4/ops-pass.log and /home/user/Shamble-Frontlines/.gate/um4/counted.log (the
  cold run, 13 of 13). Ask: does each failure branch have a test that fails by name if its guard is removed? Does any
  test lean on the real ops/browser/node_modules folder (STEPS.ops reaches opsStep without an injected `present`)? Does
  the glob select what u8-brief.md:524 asks? Does the wrappers' last-line parse hold if stdout ends with other output?
  Known: CI never runs check.mjs (no workflow names spikes/), so the local run is the proof: say whether that suffices.
- Block 2, NB2B-8: `git diff 1a637408 c56afd07 -- spikes/netcode-proof/crates/judge spikes/netcode-proof/marks`
  (Builder 2's fix pass, merged here at cb600ede): is each change proven by a test (tests/engine.rs), and do
  marks/README.md 2.3 and marks.json's PM-0001-3 notes match eval.rs and marks.rs? NB2-C10's M1 (figures.rs:722-724,
  review-nb2-c10.md) is not in this diff: it stays Builder 2's forward fix; do not re-report it.
- Severity on the repository's one scale (Critical, High, Medium, Low), each finding with file:line and its evidence.
- Return five lines: the verdict per block (APPROVE, APPROVE WITH FINDINGS or REVISE), counts by severity, the file.

## S: security-auditor (new seat), a short read of UM-4, into review-um4-security.md
- Write /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/review-um4-security.md, plan first, each block saved
  at once, last line "Status: COMPLETE <UTC time>". Read only: no install, never npm audit, no network, no cargo.
- (1) The step's two commands as opsStep runs them (spikes/netcode-proof/check.mjs, `git show 8fb32bc8`):
  `process.execPath --test "ops/**/*.test.mjs"` and `bash ops/tests/wrappers.test.sh`, in the spike's folder. Can a
  file under ops/ or an environment value make the check run something it did not intend (glob, shell, PATH lookup of
  bash)? (2) Step 3's install record: .checkpoints/netcode-proof-test/dependency-check.md's UM-4 section (`npm ci
  --ignore-scripts --no-audit --no-fund --prefix ops/browser`, npm_config_ignore_scripts=true, both package files'
  sha256 unchanged, 167 packages) against its own clearances (security-r1-deps.md and SEC-U7D-3 in the same notes).
  (3) OPS_MISSING's line: does it tell an operator to run anything unsafe?
- Return five lines: the verdict, counts by severity (the repository's one scale), the file.

## B: the advisor a3de649cd25e474df (resumed by SendMessage), after CI on b81061cf reads all success
- The pushed-head read of UM-4 (787eb984; CI 4 of 4 success: gate 46034034986, 46034043528; toolchain 46034034877,
  46034043538) and of NB2-ONEWAY (merge cb600edee64b74a7e3bf82bd85791b61058d28bd, pushed in b81061cf; the CI run ids
  filled at sending), as it read F1 and MAIN-69 (advisor-f1-main69-head.md). Evidence: um4-plan.md's and
  nb2b-merge-plan.md's done lines, the logs under /home/user/Shamble-Frontlines/.gate/um4/ and
  /home/user/Shamble-Frontlines/.gate/nb2b-merge/ (check.log, judge-tests.log, rejudge-47-*.log and .json, gate.txt,
  light-base.log). Into /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/advisor-um4-nb2b-head.md, last line
  "Status: COMPLETE <UTC time>".
