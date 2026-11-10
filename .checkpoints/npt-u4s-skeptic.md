# Checkpoint: npt-u4s-skeptic
- **Goal:** a skeptic check of U4s's merged code (spike commit f080551 against its parent 5f61f8f: the seam types, the
  wire grid, the tick constants and the manifests; 19 files, +1,979 −13 outside `.checkpoints/`, confirmed with
  `git diff --stat`); notes only, no build, no test run, no pull request · plan: item 3 of `night-2026-11-10.md` at
  2dc1f31, approved in issue #16's Queue under Mustafa's rule of 2026-11-09
- **Branch:** cs/npt-u4s-skeptic · **Base:** origin/main 44446a5
- **Spike read in:** `.gate/npt-spike` (read-only worktree, detached at df8975b; never edited). netcode-architect
  confirmed crates/sim is unchanged from f080551 to df8975b (empty git diff), so line numbers hold at both.
- **Last update:** 2026-11-10 06:49 Amsterdam
- **State: DONE.** Both notes files and this summary are committed and pushed. The day session folds them into PR #40
  (plan rule 2: no pull request, delete nothing).

## Summary (findings by severity, each with its proof)
**No Critical, no High.** One Medium, eleven distinct Lows (netcode T1 and rust F-L1 are the same finding). U4s has had
its two review rounds, so by the plan's rule 5 every finding below is a follow-up for the unit's report; none opens a
third round. Step 3 (the night main session's refutation of each Critical or High) had nothing to refute; the
Medium was still checked by reading.

**Medium**
- **S1 (netcode-architect): one field layout per kind serves two catalogue rows.** The soldier layout carries velocity
  (`crates/sim/src/seam.rs:114-116`) that the remote-soldier row leaves out (`docs/design/netcode-proof-messages.md:137`),
  and has no ammo or fire timers, which the owner-only own-state row needs (`:136`). The doc at `seam.rs:139-141` calls
  the lists "the catalogue's spatial rows", which is false for velocity. Carry 1 has U4a build every wire table from
  this layout. Scenario (the agent's estimate, read from wip/u4a.patch and wip/u4d.patch): each remote soldier record
  about +54 % bits (about 294 B more per full 48-soldier snapshot), velocity re-sent on every turn, so the spike's bytes
  and staleness are not the catalogue's: a risk of a false FAIL, never a false pass; and the owner's client cannot be
  reset to the server's ammo or fire timers. **Checked by the night main session at 06:46 by reading those four places:
  holds.** Becomes High if U4g's Predictor or -41's untraced-corrections check needs the weapon timers. Proof: read the
  lines; needs a run (day) after U4a and U4d merge: a npt-proto test that the remote soldier group's whole-record width
  equals the catalogue row's (`cargo test -p npt-proto`). Fix: an audience per layout entry (owner only or everyone), or a
  separate own-state layout, before U4a's table lands. **For the day session to weigh before U4a merges.**

**Low**
- **S2 (netcode-architect):** nothing ties `TickInput::tick` to the World's counter (`seam.rs:67-71`, `:80-81`,
  `:201-204`). A World built with a first tick other than the room's `tick_start` (63,736, plan §4.14:443) misses every
  history lookup (`history.rs:72-82`): lag compensation silently off, traced only by `rewind_outside`. Proof: a
  `debug_assert_eq!(input.tick, self.next_tick)` in U4d's `World::step`; a U4e test at `tick_start` 63,736 with one
  rewound hit. Not in the carries.
- **T1 = F-L1 (both):** `LOOP_CATCHUP = TICK_HZ / 4` is 0 for 1-3 Hz, which the 1-1,000 assert allows
  (`ticks.rs:18-21`, `:42-47`): such a server never runs a tick. Fix: assert `LOOP_CATCHUP >= 1` or narrow to 4..=1000.
- **G1 (netcode-architect, a design note for the ADR):** the per-tick 1 cm own-copy snap (`grid.rs:186-190`) makes
  speed depend on direction and tick rate (about 6 % faster diagonals than straight moves at 60 Hz; under 0.15 m/s per
  axis at 30 Hz never moves). Prediction stays exact. Needs a run (day): a World test stepping (127, 0), (90, 90) and
  (3, 0) for one second at 30 and 60 Hz.
- **G2 (netcode-architect):** a projectile's start tick is 32 bits per add (`seam.rs:108-109`, `:119`) where the
  catalogue re-bases it to the tick it becomes relevant (`messages.md:139`). Bytes only.
- **F-L2 (rust-reviewer):** `HitHistory`'s derived `Clone` (`history.rs:10`) gives capacity = length; a cloned
  part-full ring reallocates. Performance only, if anything clones it.
- **F-L3 (rust-reviewer):** npt-sim's float ban list (`crates/sim/clippy.toml:16-58`) may not ban
  `f32/f64::algebraic_*` (reassociation allowed) if they are stable on the pinned 1.99.1. Needs a run (day): a scratch
  `let _ = 1.0f32.algebraic_add(2.0);` in npt-sim, `cargo clippy -p npt-sim -- -D warnings`; refusal expected.
- **F-L4 (rust-reviewer):** `grid.rs:11-12`'s doc says every integer of every field round-trips "(the tests run all of
  them)", but s2 (`tests/seam.rs:123-151`) runs the five float fields only; `Field::integer` fields such as `TICK` lose
  integers above 2^24 through `to_grid`/`from_grid` (f32). A generic decoder calling `from_grid` on every field gets a
  wrong projectile start tick after 6.5 days at 30 Hz. Proof: `cargo test -p npt-sim --test seam` with
  `to_grid(TICK, from_grid(TICK, 16_777_217)) == 16_777_217` (fails). Fix: a debug assert or a narrower doc.
- **F-L5 (rust-reviewer):** the own-copy snap saturates range fields with no clamp report or debug assert
  (`grid.rs:99`, `:151-158`); hashes still match. Proof at U4d: a debug assert on the snap's input range.
- **F-L6 (rust-reviewer):** npt-server's `bites` feature omits `npt-client/bites` (`crates/server/Cargo.toml:19-23`,
  `:32-35`), so `cargo test -p npt-server --features bites` would not compile a server test naming
  `npt_client::grid_with_bite`. Needs a run (day) once U4e adds such a test (U4e's brief: recheck).
- **F-L7 (rust-reviewer):** check.mjs's tick-constant grep (`check.mjs:627`, `:563`) passes any initializer that names
  `TICK_HZ` or `ticks_for_ms(` whatever literal rides along (`ticks_for_ms(500) + 2`), never reads a nested-array const
  type, and passes a path to a `*_TICKS` defined in npt-net; check.test.mjs plants none of these. Makes the report's
  "no literal `*_TICKS`" (u4s-report.md:34) slightly broad. Proof: plant the first two, `node check.test.mjs`.
- **F-L8 (rust-reviewer):** the room float grep `FLOAT_TOKEN` (`check.mjs:646-647`) misses `1e_3` and a float right
  after `..`. Proof: plant both at check.test.mjs:1424-1435, `node check.test.mjs`.
- **F-L9 (rust-reviewer):** the npt-sim dependency allow-list (`check.mjs:74`, `:122-131`) checks direct dependencies
  but not their features: `rand_core` with `os_rng` would bring getrandom into the sim and pass. Proof: plant
  `e.rand_core = n("getrandom")`, `node check.test.mjs` (passes today). Fix: check the shipped set against the lock, or
  ban `from_os_rng` in clippy.toml.

**Held up** (both lanes): the Rewind edge cases against the history lookup, the canonical command order, what the sim
reads (§4.2, carry 7), the reports outside the hash (ADR 0004:552); the grid's widths and ranges against §4.4 and the
catalogue, its tie rule, NaN and infinity, the 201,727-integer round trip; every §4.3 tick value at 30 and 60 Hz;
determinism (f64 multiply and divide, round, floor and saturating casts only); `HitHistory`'s eviction and refusals
(stored-tick compare, u32 wrap tested); wasm32 casts; the lock (edges only); the bite-id grep for its stated rule; every
§5 U4s "Builds" item present.

**Not covered:** the plan's "Bites" line (the ring kept one short, s4) needs the build's bite log (day); whether libm
0.2.16's build script changes code paths by optimisation level (no registry, no web).

## Steps done
1. Branch set up (the worktree from item 2 reused as it is, plan rule 1) · cd2a3e3 · ok
2. In parallel (Opus 5.5, xhigh), launched 06:37: netcode-architect → `netcode-architect-u4s.md` (DONE 06:46,
   7877e92); rust-reviewer → `rust-reviewer-u4s.md` (DONE 06:47, this commit)
3. Refutation pass on each Critical or High: none raised. S1 (Medium) checked by reading at 06:46: holds
4. This summary · (this commit)

## Current step
none: the item is done.

## Next steps (for the day session)
1. Fold `.checkpoints/npt-u4s-skeptic/` into PR #40's notes; weigh S1 before U4a's field-group table lands.
2. Route the Lows to their units' follow-ups (S2 to U4d and U4e; F-L4, F-L5 to U4d; F-L6 to U4e; F-L7 to F-L9 to the
   main session's check.mjs list; G1 to the ADR notes; T1 to whoever next touches ticks.rs).

## Open findings
- S1 Medium: open, for the day session (above).
- Eleven Lows: open, follow-ups (above).

## Key numbers
- Findings: 0 Critical, 0 High, 1 Medium, 11 distinct Lows (12 raised, T1 = F-L1)

## Decisions taken
- Started at 06:37, before the 06:45 no-new-item time; agents time-boxed to 07:05; both done by 06:47 · night shift ·
  2026-11-10

## Running work
- none: both agents done; the saver (bnnx8ohmi) stopped with TaskStop at 06:47
