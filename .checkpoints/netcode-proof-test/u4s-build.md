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

## Fix pass
Builder: a workflow agent, same worktree, still uncommitted on base c8abc85. Inputs: the three reviews
(u4s-review-netcode.md, u4s-review-arch.md, u4s-review-rust.md in the main folder's notes), the fix list of this pass's
brief (11 items), u4-build-plan.md §4.1-§4.4, §4.11, §4.14, §5 (U4s, U4a, Lanes), advisor-u4.md. Same build rules
(§7.1; CARGO_NET_OFFLINE=true; Write/Edit only; bites in place by a scratch script with sha256 restores; commit nothing).
Baseline of this pass: grid.rs 2446152d, history.rs d4351e08, lib.rs 812493d5, seam.rs 6dd94205, ticks.rs b5bbcbf2,
tests/seam.rs 6cba21bd, client lib.rs fc6019ee, client Cargo.toml 8fb1f79d, proto Cargo.toml d5a2d334, Cargo.lock
3cb230ca, check.mjs 74f75b74, check.test.mjs 09d67b79; `cargo tree -d --all-features --target all` sha256 2da539bb,
`-e normal,build` f183b34a.

### Plan (one line per item; the proof each gets)
1. seam.rs Command: the sim reads slot, seq, buttons, presses, move_xy, aim; tick and the view time are the room's and
   the recording's (the sim uses the Rewind). Doc; its test (equal hashes when only those differ) is U4d's: carry.
2. seam.rs TickInput.commands: the canonical order (slot ascending, then command tick wrap-aware within a slot); the
   World's debug assert is U4d's: carry. Doc.
3. seam.rs EntityRecord grid/raw rule; Rewind: a refused Rewind behaves as None; outside the ring = tick_a not held, or
   tick_a + 1 not held when fraction > 0. Doc. Docs build with rustdoc -D warnings (intra-doc links), one link bite.
4. history.rs: `HitHistory::for_ms(span_ms) -> Option<HitHistory>` (ticks_for_ms; None for 0 ms); `new(ticks)` removed.
   s4 by ms; new test s4b refuses 0 ms. Bites: the refusal removed (s4b), the ring one short (s4).
5. grid.rs: only the float-side defect, bites-only `Config::floor_rounding()`; no ids, no BITES. -17's id to npt-client
   (bites-only BITES + grid_with_bite, test in crates/client/tests); -41's id to the own-copy snap side in npt-sim
   (bites-only OWN_COPY_SNAP_BITES + own_copy_snap_with_bite, for U4d's World::with_bite); -57's three to U4a's Quant
   (carry). Tests: each site's id at its site alone. Bites: each site accepting the other's id; the defect a no-op.
6. `EntityKind::ALL`, `EntityKind::layout() -> &'static [Field]` (the catalogue's per-kind rows, U4d completes them),
   `grid::Field::integer(min, max)`; compile-time assert every layout fits EntityRecord::FIELDS; VELOCITY fixed for
   wave 1 in grid.rs's text. Test s6. Bites: a layout past FIELDS (compile error), integer() one bit short (s6).
7. check.mjs dependencyProblems: npt-sim's shipped direct dependencies are libm, rand_chacha, rand_core only;
   check.test.mjs test + the clean graph updated. Bite: the allow-list widened (self-test fails).
8. npt-proto [dev-dependencies] rand_chacha, rand_core (U4a's a1/a2 fixed-seed cases); Cargo.lock by `cargo metadata
   --offline`, diff checked (2 lines).
9. ticks.rs: `full_state_ack_ticks_at`, so each derived constant is `<name>_at(TICK_HZ)`; const asserts: each constant
   == its function at TICK_HZ, TICK_HZ in 1..=1,000. s1 table-driven per constant at TICK_HZ, 30 and 60. Bites:
   R4 (`7 * TICK_HZ / 30`) with TICK_HZ = 60 (compile error) vs TICK_HZ = 60 alone (compiles); TICK_HZ = 0 and 1,001.
10. check.mjs tickConstantProblems widened: in npt-sim each TICK_DERIVED name is exactly `<name>_at(TICK_HZ)` (R4
    refused); outside npt-sim a TICK_DERIVED name is a path to it, never derived again; every other const/static named
    `*_TICKS` or `*CATCHUP*` (CMD_CATCHUP excepted: the redundancy cap, plan §4.6) is never set from a literal; in npt-sim
    such a name must join TICK_DERIVED. Why not "no literal 7 anywhere" literally: stated in the doc comment. Self-test
    test + plants; bites on each new rule.
11. seam.rs non_finite: always 0 while Command is integers only; kept; the drop/D-4 question to the advisor (carries).
Then: u4s-carries.md in the main folder's notes. Final: check.mjs 9/9 twice, clippy host + wasm32 both builds, fmt
--check, cargo tree -d both forms equal to the baseline hashes, the old 9 bites re-run on their new sites.

### Progress
- Items 1-3, 11 (seam.rs docs): written. Item 4 (history.rs for_ms), 5 (grid floor_rounding; lib.rs
  OWN_COPY_SNAP_BITES + own_copy_snap_with_bite; client lib.rs BITES + grid_with_bite; client tests/bites.rs), 6
  (Field::integer, EntityKind::ALL + layout, const assert), 9 (ticks.rs full_state_ack_ticks_at + const asserts; s1
  table), tests s4b, s6, two sim bites tests: written. Item 8: proto dev-deps; Cargo.lock by `cargo metadata --offline`:
  +2 lines on the pre-fix lock (npt-proto's rand_chacha, rand_core), +14 on base. Items 7, 10: check.mjs + check.test.mjs.
- First runs (touch, df 13 GB each): `node --test check.test.mjs` 47 of 47; npt-sim + npt-client measured: seam 8
  passed, client 0; bites: seam 10, client tests/bites.rs 1. clippy -D warnings host (all targets) and wasm32, both
  builds, npt-sim/-client/-proto: one type_complexity error on s1's table type, fixed with a `type Derived` alias, then
  clean. rustdoc -D warnings: two unresolved links to the bites-only `Config::floor_rounding` in measured docs, made
  plain code text; then clean in both builds. fmt: two diffs applied by Edit; then clean.
- Process note: one empty `python3 -I - <<'EOF'` heredoc ran by mistake before a grep; it wrote nothing.
- Shakedown check.mjs (touch, df 13 GB): 9/9 PASS; test 275 measured (267 + 8) / 292 bites (281 + 11); self 47 of 47
  (43 + 4); deps 84 crate files (83 + client/tests/bites.rs); redact 157 items. `--list`: seam measured s1, s2 x2, s3,
  s4, s4b, s5, s6 (8); bites + the_grids_one_defect_..., pm_0001_41_... (10); client bites pm_0001_17_... (1).
- Bites (u4s-fix-bites.py, in place, sha256 restored each, all files equal to their start at the end): 31 runs, all as
  expected. Old nine on their new sites: O1 grid floor (s2 x2 FAILED, "-32767 came back from -3.2767e2"); O2 ring one
  short (s4 FAILED, right 64; s4b too); O3-O7 check self-tests (not ok: the first tick test, the first tick test,
  the float test, the deps-step test, the deps-step test, each alone in its run); O8 real-tree float in
  room (deps FAIL :7 `f32`); O9 LOOP_CATCHUP = 7 (deps FAIL :47, the new in-npt-sim message). Item 9: N1 R4 real tree
  deps FAIL ("from `7 * TICK_HZ / 30`: in npt-sim it is its own function"); N1s R4 at 30 Hz: s1 passes (why the grep);
  N2 R4 + TICK_HZ 60: cargo check E0080 "LOOP_CATCHUP is loop_catchup_at(TICK_HZ)"; N2c TICK_HZ 60 alone builds; N2r
  R4 + 60 with that assert removed builds; N3 / N3b TICK_HZ 0 / 1,001: E0080 "TICK_HZ is 1 to 1,000"; N3r 0 with the
  range assert removed builds. Item 4: N4 refusal removed -> s4b FAILED "0 ms is refused". Item 5: N5 snap takes -17's
  id -> pm_0001_41 FAILED; N6 client takes -41's -> pm_0001_17 FAILED; N7 defect a no-op -> both sim bites tests
  FAILED. Item 6: N8 integer() one bit short -> s6 FAILED "Soldier: 256 integers in 7 bits"; N9 soldier layout of 13
  -> E0080 "every kind's layout fits EntityRecord::FIELDS". Item 3: N10 broken doc link -> rustdoc -D warnings error.
  Item 7: N11 allow-list widened -> its self-test not ok. Item 10: N12 TICK_HZ-naming accepted in npt-sim -> new test not
  ok (R4 plant); N13 re-derivation outside accepted -> test 44 not ok (`npt_sim::TICK_HZ / 4`); N14 name rule dropped
  -> new test not ok (first at RESEND_TICKS: my first expectation named LOOP_CATCHUP_ALIAS, re-run with the right
  text); N14b literal check off -> not ok at LOOP_CATCHUP_ALIAS; N15 real tree `EVENT_RESEND_TICKS = 60` in room -> deps
  FAIL. Item 8: T1 dev edges removed (manifest + lock) -> `cargo tree -d --all-features --target all` sha256 = this
  pass's baseline 2da539bb (so the one change in that tree's text, now edb1202d, is npt-proto's dev edge to
  rand_chacha under syn v2's reverse paths; the duplicate set is unchanged; `-e normal,build` f183b34a unchanged).
- After the last bite: every planted file's sha256 equal to its start (grid ede88bbe, history 1c62e5aa, ticks
  5d852f1a, seam a203150c, lib 073aba27, client lib c4c4797d, room lib fd4db3af (unchanged by the pass), check.mjs
  508cd49f, proto Cargo.toml 21715f7b, Cargo.lock 3da1e15f).
- Final, run 1 (touch, df 13 GB): check.mjs 9/9 PASS: fmt; clippy (host, all targets, both builds) 3.1 s;
  clippy-wasm32 (both builds) 0.5 s; test 79.8 s, measured 275 passed / bites 292 passed, 0 failed, 0 skips; deps PASS
  (84 crate files, 1 cargo config); refs 528 tracked files; marks 81; self 47 of 47; redact 10 of 10, 157 items, 0
  findings. Run 2 (touch, df 13 GB): the same, test 80.2 s, 275 / 292, self 47 of 47. `cargo +1.99.1 fmt --all
  --check` clean. `cargo tree -d -e normal,build` f183b34a (= baseline); `--all-features --target all` edb1202d (T1:
  only item 8's dev edge). No target/ or .gate/ inside the copy. Counts against U3d's base: measured 267 + 8, bites
  281 + 11, self 43 + 4; against the pre-fix build: +2, +4, +2.
- Carries: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4s-carries.md (33 entries by brief, the
  record of what was fixed, and the advisor's question NF-D4 on non_finite and D-4).

DONE (fix pass, 2026-11-09). Nothing committed.
