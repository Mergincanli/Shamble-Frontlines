# Seat T (test-engineer): UM-4 and NB2B-8 proof review

Brief: launch-1750-msgs.md section T. Read only. Branch claude/netcode-proof-test-plan-hcky5g, head b81061cf.

## Plan
1. Block 1 (UM-4, commit 8fb32bc8):
   a. `git show 8fb32bc8 -- spikes/netcode-proof/check.mjs spikes/netcode-proof/check.test.mjs`: opsStep, OPS_MISSING,
      OPS_BROWSER_MODULES, the five new tests.
   b. um4-plan.md rev 0.3 + done lines; advisor-um4-plan.md UM4-1..UM4-8.
   c. Logs .gate/um4/absent.log, ops-pass.log, counted.log (+ light-base.log, commit-msg.txt).
   d. Questions: each failure branch has a test that fails by name if its guard is removed; any test leaning on the real
      ops/browser/node_modules (STEPS.ops reaching opsStep without an injected `present`); the glob vs u8-brief.md:524;
      the wrappers' last-line parse when stdout ends with other output; CI never runs check.mjs: does the local run
      suffice.
   e. Run `node --test spikes/netcode-proof/check.test.mjs` from the repo root (allowed); mutation by reading only
      (no edits to shipped files; any scratch copy lives in the scratchpad).
2. Block 2 (NB2B-8): `git diff 1a637408 c56afd07 -- spikes/netcode-proof/crates/judge spikes/netcode-proof/marks`:
   each change proven by a test in tests/engine.rs; marks/README.md 2.3 and marks.json PM-0001-3 notes vs eval.rs and
   marks.rs. NB2-C10 M1 (figures.rs:722-724) excluded.
3. Verdicts, severity counts, first action. Last line "Status: COMPLETE <UTC time>".

No cargo, no check.mjs.

## Notes (one line per finding as it lands)
- 18:00Z draft of block 1 saved; extra checks (last-line mutant, skip grep, nb2b check.log) still running.
- Mutation harness: scratchpad mutate-ops.mjs (copies check.mjs, check.test.mjs and crates/hash-wasm/loader.mjs to a
  scratch folder, one exact once-only replacement per mutant, runs `node --test --test-name-pattern="the ops step"`).
  No repository file touched. Results: M0 control 5/5; M1 (drop check.mjs:2696, node exit guard) SURVIVES 5/5; M2 (drop
  :2698, zero count) killed by :2033; M3 (drop :2700, wrappers exit guard) SURVIVES 5/5; M4 (`w.out` for `w.stdout ||
  w.out`, :2702) SURVIVES; M5 (drop N > 0, :2704) killed by :2043; M6 (drop :2694, present) killed by :2052; M7
  (STEPS.ops passes `() => true`, :2740) SURVIVES; M8 (OPS_BROWSER_MODULES = ops/browser, :2684) SURVIVES; M9 (last-line
  regex accepts any failed count, :2703) killed by :2043. W1 (outside the diff: drop webStep's node exit guard) SURVIVES.
- Node glob (scratch globexp.sh, dummy files, Node 22.23.4): `node --test "ops/**/*.test.mjs"` ran ops/top and ops/a/deep
  only; it did not enter ops/node_modules, ops/a/node_modules/pkg or ops/.hidden.
- Counted run tied to the commit: `git diff 65b2f296 8fb32bc8 -- .` from spikes/netcode-proof hashes to 09c26ef2…b858,
  the value absent.log, ops-pass.log and counted.log print; untracked e3b0c442… = sha256 of empty input.
- `node --test spikes/netcode-proof/check.test.mjs` at b81061cf: 85 tests, 85 pass, 0 fail, 0 skipped.

## Block 1: UM-4 (in progress)
