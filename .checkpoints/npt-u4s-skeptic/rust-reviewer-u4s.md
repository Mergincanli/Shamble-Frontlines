Task: npt-u4s-skeptic — rust-reviewer skeptic check of U4s (f080551 vs 5f61f8f), night 2026-11-10

Started 06:37 Amsterdam; findings due 07:05; done 06:47. Notes only. Line numbers are at df8975b
(/home/user/Shamble-Frontlines/.gate/npt-spike/spikes/netcode-proof/...).

## Plan (blocks, in order)
B1. Skim prior reviews (arch, netcode, rust, fix, fix2) + carries list — titles only.
B2. Determinism + HitHistory: history.rs, ticks.rs (const asserts), seam.rs float use.
B3. Grid: grid.rs tie rule + edge snap.
B4. wasm32 hazards (usize/u64 casts) across the new files.
B5. Manifests + Cargo.lock edges.
B6. check.mjs new steps + check.test.mjs coverage.
B7. u4s-report "What was built"/"Proof" vs plan §5 U4s, §4.2, §4.3.
B8. Answer.

## Notes
- B1 done (prior rust 1-7, carries 1-45 skimmed: 1 MB cap = carry 14/43; HitVolumes.tick source + move_xy = carry 15;
  HitVolumes clone_from = carry 13; libm arch comment = carry 29; tick grep literal-arg = carry 6/18).
- B2 done (history.rs, ticks.rs, seam.rs, lib.rs, clippy.toml, tests s1-s4b):
  - HitHistory push/get/evict: correct; get() checks tick equality so a gap or duplicate push (release) gives a
    miss, never another tick's volumes; wrap test crosses u32::MAX. HOLDS. Huge span → carry 14.
  - F-L1 LOW ticks.rs:18-21 + :42-44: assert admits TICK_HZ 1..=3 where LOOP_CATCHUP = 0 (plan §4.3:255-257: at most
    `catchup` per wake, the excess dropped, so a server that never runs a tick) and ACK_RING_TICKS = 1. Scenario needs
    TICK_HZ set to 1-3 (plan: 30/60), so Low. Fix: assert loop_catchup_at(TICK_HZ) >= 1 (or range 4..=1000).
  - F-L2 LOW history.rs:10: derived Clone gives a Vec with capacity == len; a clone of a not-yet-full ring
    reallocates on later pushes (Vec growth, up to ~2x capacity). Only if anything clones it; perf, no wrong result.
  - Determinism: grid uses only f64 *, /, round, floor, as-casts (all exact or IEEE-correctly-rounded; casts saturate,
    NaN→0); no contraction (Rust never fuses); scale consts const-evaluated. HOLDS.
  - F-L3 LOW (needs a run, day) clippy.toml:16-58: ban list predates U4s but U4s adds libm; check whether
    f32/f64::algebraic_add/sub/mul/div/rem are stable on the pinned 1.99.1 (they permit reassociation/contraction,
    so target-dependent results) and absent from the ban list. Command: a scratch line
    `let _ = 1.0f32.algebraic_add(2.0);` in npt-sim, then `cargo clippy -p npt-sim -- -D warnings` (expect refusal).
    `%` on floats is not on the allow-list text but is exact (fmod), so harmless.
  - Const asserts ticks.rs:83-98 self-referential by design (catch a literal only when TICK_HZ changes);
    seam.rs:185-194 layout-fits assert sound. HOLDS.
- B3 done (grid.rs + s2/s3):
  - Tie rule: f64 product exact for scale 100 (≤29 significant bits), round() is ties-away, s3 tests ±; HOLDS.
  - Edges: saturating cast + clamp; yaw rem_euclid over 65,536 with π → -32,768; s2 covers ends ±1 and half steps.
    HOLDS. +inf yaw → -1, -inf yaw → 0 (each fixed; doc "one fixed yaw integer" slightly loose; nit, not listed).
  - F-L4 LOW grid.rs:11-12 + seam.rs:107-109: module doc says every integer of a field round-trips "(the tests run
    all of them)", but s2 (tests/seam.rs:123-151) runs only the five float fields; Field::integer fields (TICK =
    i32::MIN..=i32::MAX, in PROJECTILE's layout) go through to_grid/from_grid unrefused and lose integers above 2^24
    via f32. Scenario: a generic decoder (U4a/U4e) calling from_grid over every layout field gets a wrong projectile
    start tick after 2^24 ticks (6.5 days at 30 Hz). Fix: debug_assert kind != Integer in to_grid/from_grid, or
    narrow the doc. Proof: `cargo test -p npt-sim --test seam` with a line asserting
    `Config::default().to_grid(TICK, Config::default().from_grid(TICK, 16_777_217)) == 16_777_217` (fails).
  - F-L5 LOW grid.rs:99, :151-158: a range field saturates silently in the own-copy snap (no ClampReport, no debug
    assert): a sim value past ±327.67 m (x, y), below 0 m (z) or past ±327.67 m/s (velocity: PROJECTILE uses
    VELOCITY) is moved by the snap each tick. Deterministic (server and client agree), so not a hash split; a
    gameplay/design limit (netcode lane owns the grid design). Proof at U4d: a debug_assert that the snap's input is
    within the field's half-step range.
- B4 done (wasm32): u32→usize casts only widen or equal on wasm32; usize→u32 casts (capacity, len) bounded by a u32
  origin; u64 counters fine; `newest + capacity - age` can exceed u32 only when capacity > 2^31, which with_capacity
  already refuses (carry 14). HOLDS.
- B5 done (manifests, lock): lock adds edges only, no new package (libm, rand_chacha, rand_core, npt-link were in);
  bites features chain sim ← proto ← room/client ← server ← lab consistently. libm `arch` = carry 29.
  - F-L6 LOW server/Cargo.toml:19-23 (+ :32-35): npt-server's `bites` omits `npt-client/bites`, while npt-client is its
    dev dependency for U4e's Loopback tests; `cargo test -p npt-server --features bites` builds npt-client without
    bites, so a server test naming npt_client::grid_with_bite / BITES does not compile (only the lab's bites build
    has it). Needs a run (day) once U4e adds such a test; or add "npt-client/bites" (verify with `cargo metadata`).
- B6 done (check.mjs diff + check.test.mjs :140-170, :1300-1437):
  - F-L7 LOW check.mjs:627 (tick grep, outside npt-sim): any initializer that mentions TICK_HZ or `ticks_for_ms(`
    passes whatever literal rides with it: `const GRACE_TICKS: u32 = npt_sim::ticks_for_ms(500) + 2;` (2 ticks of
    slack that halve in time at 60 Hz). Also CONST_ITEM (:563) never matches a nested-array type
    (`const RESEND_TICKS: [[u32; 2]; 1] = [[7, 7]];`: the type part stops at the inner `;`), so the item is not read;
    and a path to any `*_TICKS` name passes, including one in npt-net, which the name rule skips. check.test.mjs has
    no plant for these (:1386-1397). Proof: `node check.test.mjs` after adding the first two plants to the "no other
    tick-count name is a literal" list (today the grep returns [] for both, so the test fails). Documented limits
    cover "another name / a let / a literal argument" (carry 6/18), not these forms.
  - F-L8 LOW check.mjs:646-647 (room float grep FLOAT_TOKEN): misses `1e_3` (valid Rust: the exponent may start with
    `_`) and a float literal right after `..` (`(..2.5).contains(&v)`, `lo..2.5`: the lookbehind (?<![\w.]) rejects
    every start inside the literal). check.test.mjs:1424-1435 plants neither. `const G: u32 = 1e_3 as u32;` compiles
    alone; the `..` form needs a float value from an npt-sim fn (already a documented limit), e.g.
    `(..2.5).contains(&cfg.from_grid(POSITION_XY, k))`. Proof: add both to the plant list, `node check.test.mjs`.
  - F-L9 LOW check.mjs:74, :122-131 + check.test.mjs:168: SIM_DEPENDENCIES holds direct dependencies only, by design;
    the features npt-sim turns on in them are not held: `rand_core = { workspace = true, features = ["os_rng"] }`
    pulls getrandom into the sim and makes `SimRng::from_os_rng()` compile, and the deps step passes (DEPS_CHECKED's
    "npt-sim ships libm, rand_chacha and rand_core only" reads as the whole closure). The -30 per-tick hash
    comparison would catch the use, not the dependency. Fix: refuse any package in npt-sim's shipped closure outside
    the locked set, or ban `from_os_rng` in clippy.toml. Proof: `node check.test.mjs` with a plant
    `e.rand_core = n("getrandom")` (passes the grep today).
  - Bite grep (check.mjs:723-): sound for its stated rule (an id in two crates' src/), raw and byte strings and
    #[cfg(test)] modules covered by tests :1444-1500; a single id in the wrong crate is pinned by the crates' own
    bites tests (client tests/bites.rs, sim seam.rs:552). HOLDS.
  - doc step (check.mjs:1365): measured then --all-features, -D warnings, CARGO_ENCODED_RUSTDOCFLAGS dropped:
    HOLDS for public items. Not seen (nit): private items' doc links (no --document-private-items) and an in-source
    #![allow(rustdoc::...)].
- B7 done (u4s-report.md:17-49 vs plan :497-514, :178-259): every "Builds" item present (manifests match :498-505
  edge for edge; check.mjs greps + self-tests; s1-s5). Two report claims are slightly broad: "no literal `*_TICKS`"
  (u4s-report.md:34; F-L7's forms pass) and s2 "every integer of every field width" (plan :508-509; only the five
  float fields, F-L4). The "Bites" line (ring kept one short, s4) I could not check without the build log: needs the
  day session's bite log, not a run of mine. No Critical, High or Medium found.

DONE
