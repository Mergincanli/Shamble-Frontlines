# Step 1's build order (4c-2, the soldier's movement in the simulation), revision 2

Written by the main session, 2026-11-28, from rust-reviewer's block 11 as blocks 12 to 15 left it (`rust-reviewer-read.md`:
11.1 to 11.10), engine-architect's block 7 (the room owns the match's constants), netcode-architect's block 9,
security-auditor's third answer and the advisor agent's two verdicts (`advisor-steps1to7.md`: Q4's nine conditions;
`advisor-delta-step1.md`: D1 to D3 with G1 to G12). **Revision 2 takes the delta verdict's G1, G2, G3 and G10**
(revision 1 is in git history). rust-reviewer's block 16 writes the per-commit table of 11.6's symbols and 11.7's rows;
where it and this file differ, its table holds and this file is brought to it.

**Rules for every commit** (G3 and the standing ones): the gate, the full local checks (`local-checks.sh`, clippy with
`-D warnings` among them) **with no new `allow`**, the push, CI green before the next code commit is pushed (notes
pushes held while a code head's toolchain run is going), the count after it. A field or a `pub(crate)` helper enters
in the commit that first reads it, or is `pub` inside a `pub mod`. Every commit that changes the hashed state re-makes
world.rs's known answers and the first recording in that same commit (Q4 condition 4). **No commit puts a move from
the wire into the hashed row without the room having checked it** (G2's rule).

**Before C3:** engine-architect's and rust-reviewer's one-line yes on G1 (the room borrows the spawn table for
`World::new` and keeps nothing; no `#[allow(dead_code)]`). C1 and C2 may start now.

## C1. The soldier table and the aim's type (no state change, no dependency)
- `crates/sim/src/ids.rs` (`SoldierId`), `soldiers.rs` (`Soldier`, its flags, `Soldiers`, `spawn`, `despawn`,
  `resolve`, `resolve_mut`, `iter`; `MAX_SOLDIERS`), `aim.rs` (`AimCm`, `NONE`), `vec.rs`'s V3 helpers; `lib.rs`. Each
  `pub` in a `pub mod` (G3), or left to the commit that first reads it.
- Tests: 11.7 row 1 (PM-0004-15's 10,000 cases on a 64-slot table) with its two plants.
- `World` untouched: no hash moves.

## C2. Content's thirteen new fields (no state change; the frozen bundle re-frozen)
- `crates/content/src/movement.rs`: FIELDS 15 to 28 with each one's version rule (4B-FIRST-DEFAULT goes to the advisor
  agent first if any field takes a default), `Movement`, the schema, the generated types (written by the export test),
  the test text; `content/movement/soldier.json` (the 13 values of NO-SOURCE (b), each marked "not from his Export";
  `body_height_m` from his Export's measured block); `content/source/testlines-table.json`'s rows with a `field` and
  no `export`, which `testlines.mjs --check` takes, and its planted row.
- **The table's thirteen rows** (game-designer's block 10, `game-designer-read.md`: N38 to N40, the rows as JSON):
  the twelve sandbox numbers each as a row with `field`, `value`, `sandbox` (its file:line at be608cad), `unit`,
  `what` and `old_game` (the old game has none of them, `shamble-game.js:2532-2541`), which the converter learns to
  take (a row with no `export` must carry a finite `value` and its `sandbox` line; its planted row claims an export
  it does not have); the body height as an ordinary row from his Export's `measured.soldier_height_m` (1.8), which
  the converter also learns to read (today it reads only `values.<group>.<name>`, `testlines.mjs:136-139`:
  game-designer's Medium flag). The unit word for a fraction: "share" (new) or one of the table's own.
- The first recording's frozen bundle re-frozen (`make_recordings --freeze first-loop`) and the recordings re-made:
  **their `.hashes` files byte-identical** [SUPERSEDED, advisor-delta-c2c3.md item 6: the 1,800 tick lines and the
  vector file byte for byte; the `.hashes` file's first line names the `.rec`, whose header names the bundle] (G5: no
  state changed), their headers naming the new bundle; `recordings.rs` green.
- **The version rule** (the advisor agent's VERSION-RULE ruling, `advisor-version-rule.md`, conditions 1 to 9): the
  movement kind goes to `VERSION = 2` ("the one version the typed read takes"), `TEST_TEXT` at version 2 with all 28
  fields; `read.rs`'s `header` takes the newest version only (`whole_in(node, &path, newest, newest)`, its doc saying
  older versions are migrated before the typed read, ADR 0005's CD-4), maps staying at 1; `bundle.rs:167` the same
  exact rule; the page's `content.ts:236` checks `version !== kind.newest`; `testlines.mjs:42`'s HEADER at version 2
  with a test tying it to `movement.rs`'s VERSION; every literal version 1 of the movement kind moved to 2 (its F16
  list: `testlines.test.mjs`, `content.test.ts`, `report.test.ts` with its mixed case kept a mix, `skeleton.spec.ts`);
  the schema's const 2. Tests and plants: version 2 read and 1, 3, 0, 1.5 and -1 refused as `WrongVersion { newest:
  2 }` (plant: the lower bound of 1 put back); the page refusing a movement file at version 1 (plant: the old range);
  the converter's tie (plant: HEADER's version 1). 4c-2's Brave checklist expects "shamble-movement 2" on the readout;
  4c-2's report lists the departure from CD-4 (no 1-to-2 migration step) for his call; 4B-FIRST-DEFAULT stays open.

## C3. The dependency batch, exp and sincos, the rules and the spawns, called only by sim's tests (no state change)
- The batch `deps-4c-v2.diff` applied, its five digests checked first (`dependency-check.md`; security-auditor's
  third answer: head plus the patch rebuilds exactly fc436ec3, b7c6ee5d, c7432b03, 79b829a9 and 22e2c5bf).
- `fmath.rs` (`share_per_tick` with `libm::exp`, `facing_of` with `libm::sincos` [SUPERSEDED, advisor-delta-c2c3.md
  item 1: the allowances sit on `exp_of` and `sincos_of`, which the two call, with the raw-name rule; rust-reviewer's
  block 17, Q1], `DEG_TO_RAD`), `rules.rs`
  (`Rules::take` with its refusals and `HEADROOM_FEET` = `SandboxTest`, his (b) until his word), `soldiers.rs`'s
  `SpawnPoint` and `spawn_points` (ranked by the id's byte order); in this commit only sim's tests call them (G10)
  [SUPERSEDED, advisor-delta-c2c3.md item 7: in C3 nothing in the players' build reaches exp or sincos; the
  vectors' test-only module calls `exp_of` and `sincos_of` by design].
- In the same commit (Q4 condition 2 as the delta verdict amends it): F1's tightened `dependency_rules.rs`; F2's
  `layering.rs` with security-auditor's narrow marking, shared in `crates/sim/tests/support/marking.rs` with content's
  panic scan (its F5, fixed here; how devtools' `layering.rs` includes the file: rust-reviewer's block 16, G8); the
  stricter libm scan in `rules_scan.rs`; the exp and sincos vectors (PM-0004-22). **The maths list's exact entries wait
  for C4** (G10: the players' build first calls exp and sincos there).
- **layering.rs by security-auditor's third answer** (`security-auditor-deps-4c-v2.md`, "Third answer" and its
  amendment): test-only is a file starting `#![cfg(test)]`, or a top-level `#[cfg(test)] [#[..]] mod NAME {..}`; a
  `#[cfg(test)]` on a fn, use or impl is refused (a false red on the safe side); raw names stay distinct from keywords;
  both crates' `src/` walked recursively, both `lib.rs` found. Its plants, all red, run inside layering.rs against its
  own marking: T-G's line in a copy of a room file and of an endpoint file; `cfg(any(test, ..))`, `cfg(not(test))`,
  `cfg_attr(test, ..)`; `extern crate content as c;`, `use ::content as c;`, `use r#content::..`; the four leaks of its
  trace (a `#[cfg(test)] r#type` field before a `content` field, a `#[cfg(test)] r#fn` parameter, a cfg on a const
  generic, a macro that drops the attribute); a top-level `#[cfg(test)] use` or `fn`; `'{'`, `"{"`, `r#"{"#` and
  `/* { */` inside a test module followed by T-G's line; `#[path]`, `include!`, `include_bytes!`; the manifest's
  `c = { package = "content", .. }`, `[dependencies.content]` and a target-specific table; sim's
  `pub use content::map::MapDocument`. Its controls, passing: the real tree, a top-level `#[cfg(test)] mod tests`
  naming content, a `#![cfg(test)]` file, `content_hash`. Block 4's manifest half and sim half
  (`engine-architect-read.md:551-559`) stand; point 15 replaces only the `src/` half.
- Its F6 (Low, accepted): the client endpoint's dev-dependency on `content` stays unused, with a comment ahead of its
  tests, until step 2, whose commit must use it (or the endpoint's two lines come out in a new batch back to it).
- security-auditor reads this commit with the shared marking file in it (the advisor agent's F9).
- Tests: 11.7 rows 2 to 5 and 32 (T-G) with their plants.

## C4. The wiring and the soldier in the shared state, with the room's whole half of the move and aim (the pins re-made)
- **The wiring** (G10): sim-wasm's `local_constants` (B5 (c): the bundle's one movement document) and `CreateError`'s
  two variants; the room's `rules`, `statics` and `query_counts`, each as it gains a reader; **the spawn table borrowed
  by `Room::new` and `LocalMatch::new` for `World::new` and kept by neither** (G1, its owners' yes first);
  `LocalMatch::new` takes the constants and loses `world`; the callers and test callers; **the maths list's exact
  entries for what the players' build now prints** (expected `libm::math::exp`, `libm::math::sincos`,
  `libm::math::rem_pio2`, `libm::math::rem_pio2_large`; G10) [SUPERSEDED, advisor-delta-c2c3.md item 8: exactly the
  paths the real build refuses, for both builds (the dev twin about 14, rust-reviewer's B17-6), each a whole path;
  anything outside the exp and sincos families reported, not added; security-auditor reads the real list].
- **The soldier:** `World { tick, player: PlayerRecord, soldiers, seed }`; `World::new(setup, &Rules, &[SpawnPoint])`
  (slot 0 at the lowest-id spawn, `PLAYER_SLOT` and `PLAYER_TEAM`; B4 with netcode-architect's conditions; grounded at
  spawn); `PlayerCommand` gains the move and the aim; the fold with NONE (the record rule, "no aim yet"); a slot with
  no soldier takes the record and touches no row; `visit_shared` writes the 48 rows, 2,902 shared bytes;
  `StateSink::f32`. He stands: C5 brings the movement.
- **The room's whole half** (G2; netcode-architect's 9.3 a to h): the fill copying move and aim from the wire;
  `MOVE_REACH` and `move_in_reach`; `refused_move` and `refused_aim` in `RoomCounters` and `refused()`; the check in
  `on_datagram`; the doc; the tie of the two NONEs; protocol's sentence; any_bytes (T2-7); **`bounds_cm`, `holds_cm`
  and `build`'s derivation of both bounds** (moved here from C5: `refused_aim` needs them).
- Pins: world.rs's known answers by the second implementation; the first recording re-made.
- Tests: rows 22 (the restore, T-F), 25, 27 (T-A), 28 (T-B), 31 (T-E), 33 (B5), W-1, W-2, W-4, W-3's stand-and-join
  half, T2-6 but its direct-feed case, T2-7, with their plants.

## C5. The movement (the hash moves again: the pins re-made)
- `collision.rs`'s `bounds_m`, floor and candidates; `body.rs` (the one sweep and every rule of 11.3, `QueryCounts`);
  `movement.rs` (`step_soldier`'s phases 2a to 2i, `push_pass`); `authority_step` in 11.1's order; the wish's cap of a
  longer pair (row 38) with T2-6's direct-feed case.
- Tests: rows 6 to 21, 23 (with G9's tick-1 jump from the air pinned or noted in 11.1), 24, 26 (J1 to J5), 29 (T-C),
  30 (T-D), W-3's moving half, Q-1, T-B's route reaching the box's top (G4), with their plants; test 21 (the oracle
  against parry3d) green: step 1 counts as done only with it (F3, Q4 condition 7) and with PM-0004-20's and -35's new
  wording drafted (`pm-wording-4c2.md`, F2, with G6 applied).
- Pins re-made again.

## After C5
- The plants' whole run on step 1 (a `plants-4c2.py` like 4c-1's), the count, the card; then step 2 (its symbol table
  first, netcode-architect's, F5).
