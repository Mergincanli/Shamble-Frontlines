# Step 1's build order (4c-2, the soldier's movement in the simulation), DRAFT

Written by the main session, 2026-11-28, from rust-reviewer's block 11 as blocks 12 to 14 left it (`rust-reviewer-read.md`:
11.1 to 11.10), engine-architect's block 7 (the room owns the match's constants), the advisor agent's verdict on steps
1 to 7 (`advisor-steps1to7.md`: Q4's nine conditions) and the sheet's section I. Each commit below is one unit: the
gate, the full local checks (`local-checks.sh`), the push, CI green before the next code commit is pushed (notes
pushes held while a code head's toolchain run is going), and the count after it. Every commit that changes the
hashed state re-makes world.rs's known answers and the first recording in that same commit (Q4 condition 4).

Waiting before commit 1: netcode-architect's block 9 (B4: when and where the first soldier is made; which step
carries the wire's move and aim), game-designer's block 9 (whether the first spawn is a rule of his), security-auditor
on layering.rs's non-test reading (its F2). The order may change on their answers.

## C1. The soldier table and the aim's type (no state change, no dependency)
- `crates/sim/src/ids.rs` (`SoldierId`), `soldiers.rs` (`Soldier`, its flags, `Soldiers`, `spawn`, `despawn`,
  `resolve`, `resolve_mut`, `iter`; `MAX_SOLDIERS`), `aim.rs` (`AimCm`, `NONE`), `vec.rs`'s V3 helpers; `lib.rs`.
- Tests: 11.7 row 1 (PM-0004-15's 10,000 cases on a 64-slot table) with its two plants.
- `World` untouched: no hash moves.

## C2. Content's thirteen new fields (no state change; the frozen bundle re-frozen)
- `crates/content/src/movement.rs`: FIELDS 15 to 28 with each one's version rule (4B-FIRST-DEFAULT goes to the advisor
  agent first if any field takes a default), `Movement`, the schema, the generated types (written by the export test),
  the test text; `content/movement/soldier.json` (the 13 values of NO-SOURCE (b), each marked "not from his Export";
  `body_height_m` from his Export's measured block); `content/source/testlines-table.json`'s rows with a `field` and
  no `export`, which `testlines.mjs --check` takes, and its planted row.
- The first recording's frozen bundle re-frozen (`make_recordings --freeze first-loop`) and the recordings re-made:
  their state hashes are unchanged (no state changed), their headers name the new bundle; `recordings.rs` green.

## C3. The dependency batch, exp and sincos, the rules and the spawns (no state change)
- The batch `deps-4c-v2.diff` applied, its five digests checked first (`dependency-check.md`).
- `fmath.rs` (`share_per_tick` with `libm::exp`, `facing_of` with `libm::sincos`, `DEG_TO_RAD`), `rules.rs`
  (`Rules::take` with its refusals and `HEADROOM_FEET` = `SandboxTest`, his (b) until his word), `soldiers.rs`'s
  `SpawnPoint` and `spawn_points` (ranked by id).
- The room holds `rules`, `statics`, `spawns`, `query_counts` (engine-architect's block 7); `Room::new` takes them;
  sim-wasm's `local_constants` (B5 (c): the bundle's one movement document) and `CreateError`'s new variants;
  `LocalMatch::new` takes the three and loses `world`; the callers and test callers.
- In the same commit (Q4 condition 2): F1's tightened `dependency_rules.rs`, F2's `layering.rs`, the stricter libm
  scan in `rules_scan.rs`, the exact `maths-allowed.mjs` entries for what the players' build prints, the exp and
  sincos vectors (PM-0004-22).
- **layering.rs by security-auditor's third answer (REVISE on the wording, the batch's approval standing;
  `security-auditor-deps-4c-v2.md`, "Third answer" and its amendment):** not rules_scan.rs's test-only marking, which
  lets `content` through (raw names read as keywords; a closing `)` not ending the marked span), but a narrow one:
  test-only is a file starting `#![cfg(test)]`, or a top-level `#[cfg(test)] [#[..]] mod NAME {..}`; a `#[cfg(test)]`
  on a fn, use or impl is refused (a false red on the safe side); raw names stay distinct from keywords. Both crates'
  `src/` walked recursively, both `lib.rs` found. Its plants, all red, run inside layering.rs against its own marking:
  T-G's line in a copy of a room file and of an endpoint file; `cfg(any(test, ..))`, `cfg(not(test))`,
  `cfg_attr(test, ..)`; `extern crate content as c;`, `use ::content as c;`, `use r#content::..`; the four leaks of its
  trace (a `#[cfg(test)] r#type` field before a `content` field, a `#[cfg(test)] r#fn` parameter, a cfg on a const
  generic, a macro that drops the attribute); a top-level `#[cfg(test)] use` or `fn`; `'{'`, `"{"`, `r#"{"#` and
  `/* { */` inside a test module followed by T-G's line; `#[path]`, `include!`, `include_bytes!`; the manifest's
  `c = { package = "content", .. }`, `[dependencies.content]` and a target-specific table; sim's
  `pub use content::map::MapDocument`. Its controls, passing: the real tree, a top-level `#[cfg(test)] mod tests`
  naming content, a `#![cfg(test)]` file, `content_hash`. Block 4's manifest half and sim half
  (`engine-architect-read.md:551-559`) stand; point 15 replaces only the `src/` half.
- Its F6 (Low, accepted): the client endpoint's dev-dependency on `content` stays unused, with a comment ahead of its
  tests, until step 2, whose commit must use it (or the endpoint's two lines come out in a new batch back to it). Its
  F5 (Low, for rust-reviewer): the same leak in content's existing panic scan in `rules_scan.rs`; rust-reviewer says
  whether the narrow marking is shared there in this commit or it becomes a STATUS follow-up.
- Tests: 11.7 rows 2 to 5, 32 (T-G) and 33 (B5) with their plants.

## C4. The soldier in the shared state (the hash moves: the pins re-made)
- `World { tick, player: PlayerRecord, soldiers, seed }`; `World::new(setup, &Rules, &[SpawnPoint])` (slot 0 at the
  lowest-id spawn: B4, its owners' call); `PlayerCommand` gains the move and the aim; the fold with NONE (the record
  rule, "no aim yet"); `visit_shared` writes the 48 rows, 2,902 shared bytes; `StateSink::f32`.
- The soldier is made and folds its commands; he does not move yet (C5 brings the movement).
- Pins: world.rs's known answers by the second implementation; the first recording re-made.
- Tests: rows 22 (the restore, T-F), 25, 27 (T-A), 28 (T-B), 31 (T-E) with their plants.

## C5. The movement (the hash moves again: the pins re-made)
- `collision.rs`'s bounds, floor and candidates; `body.rs` (the one sweep and every rule of 11.3, `QueryCounts`);
  `movement.rs` (`step_soldier`'s phases 2a to 2i, `push_pass`); `authority_step` in 11.1's order.
- Tests: rows 6 to 21, 23, 24, 26 (J1 to J5), 29 (T-C), 30 (T-D), with their plants; test 21 (the oracle against
  parry3d) green: step 1 counts as done only with it (F3, Q4 condition 7) and with PM-0004-20's and -35's new wording
  drafted (`pm-wording-4c2.md`, F2).
- Pins re-made again.

## After C5
- The plants' whole run on step 1 (a `plants-4c2.py` like 4c-1's), the count, the card; then step 2 (its symbol table
  first, netcode-architect's, F5).
