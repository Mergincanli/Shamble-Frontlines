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
