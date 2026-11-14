#!/usr/bin/env node
// U4r's determinism run on its two runner recordings (the advisor's U4r build ruling, point 1, route (c);
// u4-build-plan.md :800-802): check.mjs's `wasm` step, imported (never a copy), on crates/lab/det-recordings.json with
// the bites half off, in its own folder under .gate/ (never the step's WASM_STEP_DIR).
//   node spikes/netcode-proof/ops/runs/det-u4.mjs
// - Its clean half: each runner entry regenerated inside `npt-lab hash` by its `npt-lab scenario --record` settings and
//   checked by length and SHA-256 (carry 82), native leg 1 against native leg 2, Wasm leg 1 against native leg 1, and
//   h2's vectors. The bites half proves the comparer catches a planted divergence; check.mjs shows that on SS-s1 at
//   every run, on the same code, so it is off here (the ruling's point 1).
// - The step's rules hold here as they are wasmStep's own: `npt-lab hash` and its compare must exit 0 (carry 83 (b));
//   every call but the cargo builds shares one 300 s deadline from the first of them, and a call stopped by it takes
//   every process it started (carry 84).
// - It runs check.mjs's `deps` step first, then the clean half (advisor-u4r-r1, FP-6 item 10: INF-U4R-5). It prints as
//   check.mjs's main prints its steps (PASS or FAIL, each step's seconds and its summary: for the clean half each pair,
//   h2 and each call's seconds; on a FAIL its last 40 lines and the file under .gate/ that keeps its whole output) and
//   exits 0 only when both pass. A FAIL is never re-run into a PASS (u4r-brief.md section 4's no-retry rule for the wasm step).
// - Run it as check.mjs is run: under the lane lock, with the session's CARGO_TARGET_DIR (the step's cargo builds read
//   their artifacts' paths from cargo's JSON messages, so either cache serves).

import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { FAILED_STEP_DIR, STEPS, main, wasmStep } from "../../check.mjs";

const SPIKE = join(dirname(fileURLToPath(import.meta.url)), "..", "..");
const ROOT = join(SPIKE, "..", "..");
/** The manifest it reads: U4r's two runner recordings, in recordings.json's schema (the ruling's point 1 (2)). */
const DET_MANIFEST = "crates/lab/det-recordings.json";
/** Its own folder under .gate/ (gitignored), emptied at each run as the step empties its own. */
const DET_DIR = join(ROOT, ".gate", "netcode-proof-test", "det-u4");

// check.mjs's deps step first (advisor-u4r-r1, FP-6 item 10: INF-U4R-5), so this exits 0 only when the builds pass the
// checks check.mjs would make of them and the clean half passes.
const steps = { deps: STEPS.deps, "det-u4": () => wasmStep(undefined, undefined, undefined, DET_DIR, { manifest: DET_MANIFEST, bitesHalf: false }) };
try {
  process.exitCode = main(process.argv.slice(2), steps, console.log, FAILED_STEP_DIR);
} catch (e) {
  console.error(`det-u4: ${e.message}`);
  process.exitCode = 2;
}
