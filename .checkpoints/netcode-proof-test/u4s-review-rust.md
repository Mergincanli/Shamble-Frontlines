# U4s build proof: rust-reviewer

Worktree: /home/user/Shamble-Frontlines/.claude/worktrees/wf_aa2cc979-004-1 (uncommitted; base c8abc857)
Plan: u4-build-plan.md §4.1-§4.4, §4.11, §5 U4s (:492-508), §7.1; plan.md §4.5 (determinism), §4.8 (pins).
Rules: CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target on every cargo call; touch crates before
each build; df -h /home/user before each build (stop < 2 GB); --locked offline; git only in the copy (no -C/-c);
Write/Edit only; commit nothing; bites in place, one at a time, sha256 restored.

## Plan
1. Record sha256 of every file I will plant in (baseline); git status of the copy.
2. Run 1: touch, df, `node spikes/netcode-proof/check.mjs` unchanged; record the nine step lines and counts.
3. Bites (one at a time, in place; run; record failure by name; restore; sha256 check):
   B1 grid floor-rounding (s2), B2 ring kept one short (s4), B3.. check.mjs grep self-test bites (44, 45, 37 wiring).
   For cargo bites: the narrowest run that shows the named test failing (cargo test -p npt-sim --features bites ...).
4. Review the Rust: unsafe, determinism (floats, libm, hash maps, const fns), overflow, allocation, wasm32;
   libm =0.2.16 pin, features, Cargo.lock (no new package), cargo tree -d.
5. Final: touch, df, check.mjs again; git status/diff hash equal to step 1.
6. Findings + verdict. DONE.

## Progress
(started 2026-11-09)
