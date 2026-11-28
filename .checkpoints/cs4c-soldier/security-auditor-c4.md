# security-auditor: C4 read (fb2030ed, parent 3ac717b4) against F2, ruling 8 (a)-(d), F9

Read only: notes here, scratch under .gate/. No checkout in the worktree.

## Plan
1. maths-allowed.mjs at fb2030ed: 21 entries vs maths-refusals-c4.txt; each why vs libm 0.2.16 source (rem_pio2_large,
   the header's "own Rust path" claim, C::round's second caller face_cm). Rule on the 6 parent-covered entries.
2. build-checks.test.mjs departure: exact carve-out pair; planted twin; condition-1 block; anything lifted wrongly.
2b. support::env / float_traits / int_traits: what else lives there (non-exact methods, fma); version pin note;
   rem_pio2_large vs F2.
3. room on_datagram: refused_move, refused_aim, NONE tie, JOIN_SLOT, no panic / no per-tick alloc, `handed` cfg(test),
   any_bytes fuzz (run it).
4. collision.rs bounds: WorldError::Bounds, face_cm, aim_box, holds_cm, world past reach.
5. sim-wasm local_constants, CreateError messages (no seed byte), constructor check order.
6. devtools replay.rs Lines: seed never printed.
7. git show --stat: dependencies / dynamic text / storage unchanged.
8. Verdict.

## Notes
- [x] 1a list read at fb2030ed (maths-allowed.mjs): 23 entries = C::round, C::__multi3 + 21 libm paths = the
  refusals file's 21 unique paths (maths-refusals-c4.txt: release 4, dev 29, 0 with the whole list, none outside
  libm::math). F2's four (exp, sincos, rem_pio2, rem_pio2_large) are all there; the 17 more are dev-twin-only paths
  (inlined in release). No `libm::math` or other covering prefix; every entry an exact path the build printed.
- [x] 1b header claim "own Rust path on wasm32 and x86-64" vs libm 0.2.16 source: exp.rs:86-88 select_implementation
  only use_arch_required x86_no_sse; floor.rs:15-18 use_arch wasm32 + intrinsics_enabled (unstable-intrinsics, refused
  by the feature pin) and x86 without sse2; rem_pio2.rs:56 x87 only; sincos/k_sin/k_cos/scalbn/generic floor/scalbn: no
  arch cfg. rem_pio2_large.rs:29/:41 IPIO2 66 vs 690 by pointer width, :237-238 debug_assert on 64-bit only, :229 the
  inline(never) for x86_no_sse. Claim HOLDS, with rem_pio2_large the one named exception.
- [x] 1c rem_pio2_large reach: rem_pio2.rs (from :155) calls it only for ix >= 0x413921fb (|x| >= 2^20 pi/2), NaN/inf
  returns first. facing_of (fmath.rs:67-86) passes (deg - turned)*DEG_TO_RAD, in [0, pi/2) for deg in [0, 360); content's
  FACING [0, 360) (map.rs:66). Callers at fb2030ed: soldiers.rs:211 (spawn.facing_deg), :397 (test consts), devtools
  libm_ops.rs (vectors). For a NaN deg: falls to quarter 3, NaN -> sincos returns at ix >= 0x7ff00000 (sincos.rs:37-41).
  Even a huge deg (content-refused) gives rest = deg - 270 large -> rem_pio2_large on server; on f64 inputs only the
  first ~47 IPIO2 words are read, which are equal in both tables, so results would match anyway (by reading, not run).
  The why's text (linked, unreachable, 66 vs 690, the debug assertion, "a later caller ... back to review") = ruling
  8(c). Weighed against F2: F2 expected it listed with exactly this domain argument. MET.
- [x] 1d the six parent-covered entries (rem_pio2::rem_pio2::medium, rem_pio2_large::rem_pio2_large::floor, the four
  float_traits::Float::*): under the segment rule (build-checks.mjs libmProblems `routine === entry ||
  startsWith(entry::)`) each is a no-op: removing one lifts nothing and refuses nothing. Ruling: ACCEPTABLE, no finding.
  They widen nothing (a child of a listed parent), the header names them as such, and the test's 23-path pin holds the
  list exact, so they document what the build printed (ruling 8(a)'s "exactly the paths the refusals print") rather
  than grant anything. Risk only of a reader's misreading (removing a child does not refuse it); the header says so.
- [x] 1e whys checked vs source: rem_pio2's (|x| <= 3pi/4 one subtraction in two parts, rem_pio2.rs ~:96-108; medium on
  (ix & 0xfffff) == 0x921fb) right; medium's "high word 0x3ff921fb, just under pi/2" right for [pi/4, pi/2); floor called
  only by rem_pio2_large (rem_pio2_large.rs:230-231, :301) right; get_high_word (mod.rs:368-370) a shift, right;
  floor's "generic body on our targets" right (floor.rs:15-18).
- [x] 1f RUN (reads only): the session's c4-maths-refusals.mjs on the twins as they stand now (rebuilt 18:25, sha
  release 6fd915ed..., dev a024fe70..., not the 17:56 pair the file names): release 4, dev 29, 21 unique paths, 0
  listed-not-refused, 0 refused-not-listed, 0 outside libm::math, 0 with the whole list. Reproduced.
  My probe .gate/sa-c4/probe.mjs: release names holding `4libm` = 4 (exp, sincos, rem_pio2, rem_pio2_large); dev = 83,
  of which ~50 are core generics instantiated BY libm (name ends `..._4libm`, no step after it: pathsInside reads no
  path, so not maths routines: right).
- [x] 2b(i) module entries. libm 0.2.16 support/: big, env, feature_detect, float_traits, hex_float, int_traits
  (+ int_traits/narrowing_div), macros, modular. The three entries lift env, float_traits, int_traits (and
  int_traits::narrowing_div) only; probe: support::big, support::modular, a `float_traitsx` sibling refused.
  What else they lift (source read): float_traits.rs Float::canonicalize (:196-201, `self * 1.0`), signum, abs/copysign
  (generic fabs/copysign, bit ops), normalize (int), DFloat::narrow / HFloat::widen (:413-433, f64<->f32 `as`), and
  Float::fma (:288-298, calls libm's fma): fma's name holds the step `fma`, refused by fusedMultiplyAddProblems
  whatever the list (probe: trait fma name -> fused 1). int_traits.rs: integer ops, CastInto (:436-454, int `as`
  int, debug_assert in dev), cast_into_float (:456-480, int `as` f32/f64: correctly rounded, one answer per target),
  narrowing_div (integer divide; unsafe unreachable_unchecked on its precondition). i128 -> float would call
  compiler_builtins::float (refused: probe cbFloat). No non-exact routine is lifted beyond NaN payloads (canonicalize's
  multiply, narrow's demote: Wasm's NaN bits are not promised, the same as every f64 op of the sim; no NaN reaches them
  on the vectors' domain). Finding L-1 (Low): the float_traits and int_traits whys describe only what was printed
  ("bit reads and bit casts ... no arithmetic"; "integer conversions") while each lifts more (a multiply by one, f64<->f32
  conversions, int->float casts, a narrowing divide); the header says modules cover every function, the whys should too.
  Bounded by the =0.2.16 pin (Cargo.toml:28, Cargo.lock:170-171) and the header's "a libm version other than the pinned
  one brings this list back to review".
- [x] 2 test departure (build-checks.test.mjs @fb2030ed, diff +58): carve-out `own` = exactly the Map of
  "libm::math::exp::exp" -> exp, "libm::math::sincos::sincos" -> sincos; expected exactly [banned(name), banned(name)]
  for those two strings, [] for every other entry: any other entry with a banned segment fails (e.g. exp::exp::x,
  C::exp, sin::sin). Planted twin [OURS, CORE, exp, sincos, sin, cos, expf] -> toEqual exactly five banned lines (so
  no libm-rule line double-counts them, :785). Condition-1 block present (8 siblings refused under the whole list;
  theTwo [] under the list, 2 with the two entries filtered out, banned []); no-duplicate, five bare prefixes refused,
  23-path pin. Rule code: bannedMathsProblems (bc :735-737) reads no list; libmProblems never emits C::<banned> or
  C::<fused> (:785); fused rule reads no list (:715-720). My probe: plain exp/sincos/sin/cos/expf -> libm [] and banned
  1 each; plain fma -> fused 1; a trait `<f64 as Float>::fma` name -> fused 1 though float_traits is listed. So no list
  can lift a plain C banned function or a fused one. Nothing lifted that should not be. MET.
- [x] 3 room on_datagram @fb2030ed (lib.rs:231-295): order lead -> move_in_reach -> aim (NONE skips bounds, else
  holds_cm) -> queue. move_in_reach (sim world.rs:27-35) i32 squares, -128^2 fits, const fn, no overflow. holds_cm
  (collision.rs:334-339) i32 compares, None box -> false. NONE x i16::MIN = -32,768 < -AIM_REACH_CM (-32,767): never
  held by any box (aim_box clamps x to the reach). (-32,768, 1, 0) != NONE -> asks the bounds -> refused (any_bytes lie 8;
  unit case aim(i16::MIN,1,0) false). Compile-time NONE tie lib.rs:35-40 (x, y, z each). JOIN_SLOT lib.rs:49-53 from
  PLAYER_SLOT.to_le_bytes(), const assert high == 0. refused() adds both new counters (wrapping); count() wrapping_add.
  newest_received still rises before the checks (pre-existing; echo only, :360). No new allocation (no Vec in the path);
  the crate's deny list (:8-16) unchanged. `handed`: field :128-129 and init :156-157 and write :348-351 each under
  #[cfg(test)]; integration tests build the lib without cfg(test): not in the shipped build. Snapshot still carries own
  buttons/presses only (:356-366): no soldier row on the wire, so no other row's aim or move is exposed. MET.
- [x] 3 RUN: `cargo test -p room --locked --offline` @fb2030ed worktree: unit 22 passed, any_bytes 1 passed (1.04 s).
  any_bytes read: honest_move loops to move_in_reach; honest_aim NONE 1 in 4 else within bounds faces included; lies
  6 (move_x -128: -128^2 > 127^2 whatever y), 7 (1 cm past one of five faces), 8 (i16::MIN, 1, 0); per-message identity
  (refused_move, refused_aim) deltas == the lie for intact messages after the join; refused() == sum of the seven;
  every count > 100 over the run. MET.
- [x] 4 collision.rs @fb2030ed: build (:262-281) refuses Bounds when an f32-narrowed corner is not finite or an axis
  has low >= high, BEFORE aim_box; so face_cm only sees doubles finite within f32 range. face_cm (:126-140): on_grid
  (map.rs:250-253, `steps as i64` saturating cast), else round, `as i64` saturating, saturating_add/sub: no panic,
  no overflow on any finite double. aim_box (:146-161): i64 max/min with the reach, low > high -> None, `as i32` only
  after the clamp (exact). holds_cm (:334-339) i32 compares; None -> false. Tests: Bounds for NaN, +-inf, +-1e39 on each
  face, flat, reversed, one f32 apart, message pinned (:880-907); faces held / 1 cm past refused, NONE and
  (i16::MIN, 10, 0) never held even on the widest map (:836-877); world past the reach: None, no aim held (:874-876),
  and room's on_a_world_past_the_aim_s_reach.. (room lib.rs:994). RUN: `cargo test -p sim --locked --offline`: 59 +
  1 + 9 + 2 + 2 passed. Input: map documents are content (our bundle, server-side) or built in code; no network
  input reaches build. MET.
- [x] 5 sim-wasm lib.rs @fb2030ed: local_constants (:104-115) map -> exactly one movement (Soldier{found}) -> world ->
  rules -> spawn table; create (:137-158) seed, mode, content, local_constants, pad. CreateError::Soldier and ::Rules
  messages (:75-91) are built from a usize and RulesError::message only: no seed byte by type (the seed never enters
  local_constants). Order test (:395-445) covers seed, mode, content, map, soldier (0 and 2 documents), padding; the
  world's and rules' places are by reading (the test says so). Messages reach the Worker as JsError text (data, not
  markup); no innerHTML/eval/new Function in packages/ outside tests and scripts (git grep). MET.
- [x] 6 devtools replay.rs Lines (:170-218): rows prefixed `row <slot>`, authority() resets the prefix, f32 as value
  plus `(bits xxxxxxxx)`; the seed reaches only `secret` -> "(32 bytes, never shown)" (world.rs visit: sink.secret,
  visit_shared binds `seed: _`); test :440-484 checks no seed byte as letters or numbers. Devtools not shipped. MET.
- [x] 7 stat: 28 files; no Cargo.toml, Cargo.lock, package.json, lockfile, deny.toml, .github, html, or storage file.
  Generated types/schema: comments only. client-endpoint sends AimPoint::NONE. content's on_grid made pub (one rule).
  Shared state (world.rs:40-42, "the part a client holds too") now holds every soldier's held move and aim; the
  snapshot does not carry it today (room lib.rs:356-366). When snapshots carry rows, interest management (ADR 0001)
  decides what each client gets: a note for netcode, not a finding on this commit.
- [x] 1f' provenance RUN. HEAD moved to 04873cd8 (two notes commits); `git diff --stat fb2030ed HEAD`: only
  .checkpoints files, so the code is fb2030ed's. Built both twins myself into .gate/sa-c4/target with build.mjs's own
  call (build.mjs:189-192: `cargo build --locked [--release] --target wasm32-unknown-unknown -p sim-wasm --target-dir`,
  CARGO_ENCODED_RUSTFLAGS = the remap flag of build.mjs:94, --offline added): release 318,021 bytes sha 6fd915ed...,
  dev 19,609,567 bytes sha a024fe70... = byte for byte the 18:25 twins in target/. .gate/sa-c4/twins.mjs on them: loader
  only -> release 4, dev 29; 21 unique; listed-not-refused [], refused-not-listed []; whole list 0 and 0. The
  evidence file's 17:56 pair (e03a8eb9..., 88ee1952...) is an earlier build; its result equals mine.
- [x] 1d' RUN: the list with the six parent-covered entries removed (17 entries) -> 0 refusals on both twins of
  fb2030ed. So the six grant nothing: documentation of what the build printed. Ruling stands: acceptable.
- [x] 1e' remaining whys. exp.rs:68, :155 scalbn only as exp's last step (k != 0): right. generic/floor.rs:10-67 uses
  FpResult/Status (env), to_bits/from_bits/exp_unbiased/is_sign_negative/is_sign_positive (float_traits),
  unsigned (int_traits): right. generic/scalbn.rs:36-119 from_parts (float_traits) and casts: right. k_sin.rs:47,
  k_cos.rs:55: plain polynomials, no fma/mul_add/cfg: right.
  L-2 (Low, by computation): medium's why says "a heading's rest above about 89.9999991 degrees" (from B17-8). High word
  0x3ff921fb begins at 0x3ff921fb00000000 = 1.570796012878418 rad = 89.99998201 degrees (python3 struct + degrees), so
  medium's band is (89.999982, 90), about 20 times wider than written. Coverage holds: make_libm_vectors.py:102 puts
  0x3FF921FB00000000 and the double below it in the inputs, and :105's 1,000 seeded rests lie inside the band; only the
  number is wrong (also in the generator's docstring :29-30, which describes the seeded band, true as a subset). Fix:
  the why's number to about 89.999982 (or "the high word 0x3ff921fb, from 89.999982 degrees"); the generator's text only
  if the band is meant to be the whole path (changing :105 re-records libm-inputs.bin, which comes back to me under
  L5's third condition).
- [x] 3' callee read. world.rs:241-257 World::new: spawns.first() else no soldier; Soldiers::spawn (soldiers.rs:102-117)
  get_mut by slot (Option); resolve_mut Option. authority_step (:287-306): loop over at most CATCH_UP_CAP commands,
  occupied_mut (soldiers.rs, get_mut + filter), no allocation; the one panic is the tick at u32::MAX (pre-existing,
  documented; start_tick is the server's, not the client's). spawn_points (soldiers.rs:202-215) allocates once at
  setup; facing_of on content-checked facings. local_match.rs: the room now owns the static world; static_world()
  gone. queue.rs +1 (`..PlayerCommand::NONE` in a test). RUN `cargo test -p sim-wasm --locked --offline`: 19 + 1
  (panic_hook) + 1 (step_alloc) passed.

## Not checked
- The vitest file build-checks.test.mjs itself was not run (my probes ran the same rules from the commit's
  build-checks.mjs and maths-allowed.mjs with node; the condition-1 block's assertions by reading plus probe).
- devtools tests (replay Lines, libm vectors, hash_cost, aim_reach) not run: item 6 by reading only.
- The IPIO2 "results would match anyway on f64 inputs" remark (1c) is by reading, not run; not part of the verdict's
  support (the verdict rests on unreachability on [0, 360) and the vectors).
- No clippy, gate, browser or Node boundary run; CI's run on this head not read.
- My agent memory was neither read nor updated (the brief limits my writes to this file and .gate/ scratch).

## Verdict: APPROVE, with two Lows
F2's four expected entries present; the 17 more are exactly the dev twin's own paths (reproduced on twins I built from
fb2030ed). Ruling 8 (a) to (c) MET; (d) this read. Test departure: no list can lift a plain C banned or fused
function; the carve-out is exactly libm's own exp::exp and sincos::sincos. Room, bounds, sim-wasm, devtools: MET, run.
- L-1 (Low): the support::float_traits and support::int_traits whys describe only what the build printed while the
  module entries lift every function there (canonicalize's multiply by one, DFloat::narrow / HFloat::widen, signum,
  abs/copysign, normalize; int -> float casts, narrowing_div; Float::fma lifted by path but refused by the fused rule).
  None is non-deterministic beyond NaN bits. Fix: name them in the two whys.
- L-2 (Low): medium's band is (89.999982, 90) degrees, not (89.9999991, 90). Fix: the why's number.
Per ruling 8(d) a fix goes in its own commit before C5; neither needs a push of its own; if deferred instead, name them
as STATUS follow-ups (an open security finding keeps the PR needs-mustafa until fixed or explicitly deferred).

DONE
