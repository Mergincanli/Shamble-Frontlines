# U4s build notes (seam types, the wire grid, the tick constants, the manifests, check.mjs's two greps)

Builder: a workflow agent in worktree /home/user/Shamble-Frontlines/.claude/worktrees/wf_aa2cc979-004-1, base
c8abc857d9bc5e7fb3a848ddcf430ab9c4b65c31 (fast-forwarded). Plan: u4-build-plan.md rev 2 §5 U4s, §4.1-§4.4, §4.11;
the advisor's verdict advisor-u4.md (conditions 1, 10 (a), 10 (b)). Build rules §7.1: CARGO_TARGET_DIR =
/home/user/Shamble-Frontlines/.gate/shared-target, touch crates/ before each build, counts as deltas, bites planted in
place one at a time with sha256 restore checks, no network, commit nothing.

## Plan
1. Manifests (the only wave-1 manifest change, plus condition 10 (a) and (b)):
   - workspace: `libm = "=0.2.16"` (already in Cargo.lock through fastbloom; default features kept, so npt-sim's libm
     resolves the same features as the server build's);
   - npt-sim: libm, rand_chacha, rand_core; `bites = []`;
   - npt-proto: npt-sim, npt-link; `bites = ["npt-sim/bites"]`;
   - npt-room: npt-sim; `bites = ["npt-sim/bites", "npt-proto/bites"]` (wave 2);
   - npt-client: npt-link; `bites = ["npt-sim/bites", "npt-proto/bites"]` (wave 2);
   - npt-server: `bites = ["npt-sim/bites", "npt-proto/bites", "npt-room/bites"]`; dev dependency npt-client (U4e);
   - npt-lab: `bites` forwards to npt-l1, npt-sim, npt-proto, npt-room, npt-server, npt-client.
   - Cargo.lock by `cargo check --offline` (minimal update), diff checked: only those edges.
2. crates/sim/src: `ticks.rs` (TICK_HZ, the `_at` const fns, ticks_for_ms, LOOP_CATCHUP, FULL_STATE_ACK_TICKS,
   ACK_RING_TICKS, SETTINGS_STEP_TICKS), `grid.rs` (Field, the five field constants, Config with to_grid, from_grid,
   snap; bites-only Config::BITES and with_bite), `seam.rs` (SimCore, Command, Rewind, CoverVariant, TickInput,
   EntityRecord, EntityKind, HitVolumes, PortalStates, SimEvent, InputReports, ClampReport), `history.rs` (HitHistory).
3. crates/sim/tests/seam.rs: s1-s5 (+ one bites-build test of the grid's bite ids).
4. check.mjs: tickConstantProblems and roomFloatProblems in the deps step (DEPS_CHECKED updated); check.test.mjs: a
   self-test bite for each, and a plant each in the deps-step fixture test.
5. Build and test under §7.1; bites: grid floor-rounding (s2), the ring kept one short (s4), planted in place by a
   scratch script with sha256 restore; plus each grep planted once in the real tree (deps step FAIL, then restored).
6. Final: check.mjs 9/9 twice; clippy both builds host and wasm32; fmt; cargo tree -d unchanged.

## Progress
- Baseline: `cargo tree -d` (--all-features --target all): getrandom 0.2.17/0.4.3, syn 2/3, synstructure 0.13/0.14,
  windows-link 0.100/0.2, windows-sys 0.52/0.61. Cargo.lock base sha256 d0158c8a...6397.
- Item 1 done: manifests edited; Cargo.lock updated by `cargo metadata --offline` (no fetch): +12 lines, exactly the new
  edges (client +npt-link; proto +npt-link +npt-sim; room +npt-sim; server +npt-client (dev); sim +libm +rand_chacha
  +rand_core).
- Item 2-3 done: crates/sim/src/{lib,ticks,grid,seam,history}.rs, crates/sim/tests/seam.rs. npt-sim alone: measured
  6 passed, bites 7 passed; clippy -D warnings host both builds, wasm32 measured: clean; fmt applied to npt-sim, clean.
- Item 4 done: check.mjs tickConstantProblems + roomFloatProblems (exported, in depsStep, DEPS_CHECKED updated);
  check.test.mjs: two tests (44, 45) and the deps fixture's two plants. self 45 of 45; deps PASS on the real tree
  (83 crate files).
- Item 5 done: u4s-bites.py, 9 bites, all as expected, every file restored with its sha256 equal:
  grid floor-rounding -> s2's two tests FAILED (POSITION_XY: -32767 came back from -3.2767e2); ring one short -> s4
  FAILED (capacity 64: 64 held after 64 pushes); check bites: a call counts as derived -> test 44 not ok; TICK_HZ
  anywhere first -> 44 not ok; float literals missed -> 45 not ok; each grep unhooked from depsStep -> 37 not ok;
  real tree: `pub const SCALE: f32 = 0.5;` in room -> deps FAIL; `LOOP_CATCHUP = 7` in ticks.rs -> deps FAIL (s1 would
  pass: the value is right, which is why the grep exists).
- Shakedown full check.mjs: 9/9 PASS; test 273 / 288 (267 + 6, 281 + 7); self 45 of 45 (43 + 2). (A doc-only edit to
  grid.rs's module comment landed during it, so it is not one of the two final runs.)
- `--list` on npt-sim: measured names s1, s2 x2, s3, s4, s5 (6); bites adds
  the_grids_bites_are_listed_chosen_per_value_and_each_moves_values_off_the_wire_grid (7).
- Item 6 done: check.mjs twice in a row (touch before each): both 9/9 PASS; test 273 measured / 288 bites, self 45 of
  45, deps 83 crate files, refs 528 tracked files, marks 81, redact 10 of 10 and 156 items. clippy -D warnings host
  and wasm32, measured and bites (check.mjs's clippy and clippy-wasm32 steps): PASS. `cargo +1.99.1 fmt --all --check`:
  clean. `cargo tree -d` (both forms): identical to the baseline. Cargo.lock: +12 lines, sha256 3cb230ca...f3f0. No
  target folder or .gate inside the copy. df: 13 GB free at the end.
- Process notes: one fix to this notes folder's u4s-bites.py was written by a python heredoc, and the baseline tree
  capture went to the scratchpad by a shell redirect; no project source was written by the shell.

DONE (2026-11-09). Nothing committed.
