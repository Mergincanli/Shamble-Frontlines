// The maths routines the players' WebAssembly may hold beside plain instructions (build-checks.mjs, libmProblems):
// functions of the `libm` crate, of the toolchain's `compiler_builtins` (`math`, `float`), and C-named ones.
// Every other one stops the build. An entry joins this list in the change that first links its routine, with the
// reason its result is the same bits on every target; a fused multiply-add and the functions of BANNED_MATHS are
// refused whatever this list says.
//
// The first two entries are brought by the content loader, which the players' constructor links from change-set
// 4b-2's step 9 on (rust-reviewer's decision of 2026-11-27): each runs when a bundle loads, never per tick. The other
// 21 are brought by the simulation's own two routines (crates/sim/src/fmath.rs: libm's exp and sincos), which the
// players' constructor calls from change-set 4c's C4 on, while a match is set up (the rules' easing and push shares,
// and a facing for each spawn), never per tick. They are exactly the paths the build check printed for that build, in
// both twins (the release twin's four, which the dev-profile twin prints too, and the dev-profile twin's other 17:
// the routines the release twin inlines, and their helpers), each its own entry, none a covering prefix beyond what the
// build printed (the advisor agent's condition of 2026-11-28): three are module paths (support::env,
// support::float_traits, support::int_traits), because the check reads a trait's method by its module alone, and so
// cover every function of those modules; six entries sit below another and are listed because the build printed them
// (rem_pio2::rem_pio2::medium, rem_pio2_large::rem_pio2_large::floor and the four support::float_traits::Float::*).
// libm 0.2.16's exp and sincos and what they call, no other; a libm version other than the pinned one brings this list
// back to review.
// The static collision world links none (its tree's build and its rays are additions, subtractions, multiplications,
// divisions and comparisons; rust-reviewer's read of 2026-11-27). Each entry names a routine by its path as the build
// check prints it, and covers the steps below it: { routine: "libm::math::sqrt", why: "..." } covers
// libm::math::sqrt::sqrtf; a C-named one is "C::round".
//
// What the 21 share. libm is pinned at 0.2.16 (Cargo.toml, `libm = "=0.2.16"`, and Cargo.lock), plain Rust with its
// default features: on our two targets (WebAssembly, and x86-64 for the server and the native tests) every routine
// below takes libm's own Rust path, no routine of the platform and no target-specific branch (floor's and the
// reduction's other branches are for targets we do not build: wasm32 with the unstable intrinsics, 32-bit x86 without
// SSE2), with the one exception that rem_pio2_large's entry names. Their arithmetic is f64 additions, subtractions,
// multiplications, divisions, comparisons, integer steps and bit reads; a fused multiply-add and a vector instruction
// are refused by this check whatever this list says, and WebAssembly's f64 arithmetic rounds to nearest on every
// engine. Their results are pinned, not trusted: crates/devtools/vectors/libm-expected.txt holds one digest of exp's
// bits on 33,646 inputs and one of sincos's on 101,017 (the inputs: crates/devtools/vectors/make_libm_vectors.py), and
// the same digests come out natively (crates/devtools/tests/libm_vectors.rs), in Node
// (packages/sim-worker/boundary/rules.test.mjs) and in Chromium and Firefox (packages/e2e/tests/rules/wasm-rules.spec.ts).

/** @type {{ routine: string, why: string }[]} */
export const MATHS_ALLOWED = [
  {
    routine: "C::round",
    why: "f64::round, rounding to a whole number with halves away from zero. Called when a bundle loads, never per tick: the map's grid check (crates/content/src/map.rs, on_grid: a coordinate times its steps per metre, rounded, must divide back to the same double), once per box corner; and from change-set 4c's C4 also while a match is set up: the static world's bounds in whole centimetres (crates/sim/src/collision.rs, face_cm: on_grid once for each of the bounds' six faces, and, for a face off the grid, which only a document built in code has, face_cm's own rounding once more), never per tick. The operation is exact (IEEE 754 roundToIntegralTiesToAway): every double that is not a NaN has one answer, the sign of zero included, so any correct routine gives the same bits, natively, in every browser and on a server. The toolchain's routine here is one f64.add of x and 0.5 - 2^-54 with x's sign (f64.copysign), then a cut to a whole number by integer bit masks: no multiply, no fused multiply-add, no vector instruction, and WebAssembly's f64.add rounds to nearest on every engine. A NaN's bits are not promised; no NaN reaches it (on_grid's value is a finite parsed number, its scale 100 or 1,000; face_cm's is a face of bounds that StaticWorld::build has first held finite as an f32, refusing the map otherwise). rust-reviewer, 2026-11-27: the twin's own body in Node 22 (V8) and native x86_64 f64::round gave the same bits on 4,006,064 inputs, none off an exact reference.",
  },
  {
    routine: "C::__multi3",
    why: "The toolchain's 128-bit integer multiply (the low 128 bits of a product), which WebAssembly has no instruction for. Called by the loader's number reading and writing, when a bundle loads, never per tick: core's f64 from text (dec2flt, Eisel-Lemire's 64 x 64-bit products), core's shortest and exact texts of an f64 (flt2dec, Grisu: crates/content/src/number.rs, number_text, and the refusal sentences of crates/content/src/error.rs), core's text of a u128, and the tie rule for number texts (number.rs, even_of_a_tie: 5 to the decimals times the odd mantissa, checked). Integer arithmetic: the product modulo 2^128 is one number on every target, and Rust defines checked_mul's overflow, so the bits are the same natively, in every browser and on a server. The routine here is 64-bit integer multiplies, additions, shifts and compares only, no float. rust-reviewer, 2026-11-27: the twin's own body in Node 22 (V8) matched BigInt's product on 900,144 operand pairs.",
  },
  // libm's exp and what it calls (the header above says what all 21 share).
  {
    routine: "libm::math::exp::exp",
    why: "libm's exp (musl's algorithm): a reduction by ln 2, a polynomial and a scaling by a power of two. Called only while a match is set up: crates/sim/src/fmath.rs exp_of, from share_per_tick, from Rules::take in sim-wasm's Sim::create (the easing's and the push's shares a tick), never per tick. Pinned by exp's 33,646 vectors on every target.",
  },
  {
    routine: "libm::math::scalbn::scalbn",
    why: "libm's scalbn (x times 2 to a whole power), called by exp for its last step and by rem_pio2_large: exact in its own steps (multiplications by powers of two, which round only at the subnormal edge, the same on every target) and bit reads. Reached only through exp (and the unreachable rem_pio2_large); pinned by exp's vectors, whose inputs reach its subnormal edge.",
  },
  {
    routine: "libm::math::generic::scalbn::scalbn",
    why: "The generic body of libm's scalbn above (the dev-profile twin keeps it as its own function): the same steps, the same reach, the same pin.",
  },
  // libm's sincos and what it calls.
  {
    routine: "libm::math::sincos::sincos",
    why: "libm's sincos (musl's): for |x| up to pi/4 the two kernels below, else a reduction by pi/2 (rem_pio2) and the kernels. Called only while a match is set up: crates/sim/src/fmath.rs sincos_of, from facing_of, from spawn_points in sim-wasm's Sim::create, once for each of a map's spawn points (at most 16), never per tick. facing_of takes the quarter turns off first, so the argument lies in [0, pi/2). Pinned by sincos's 101,017 vectors on every target.",
  },
  {
    routine: "libm::math::k_sin::k_sin",
    why: "sincos's sine kernel: a polynomial on the reduced argument, plain f64 arithmetic. The release twin inlines it into sincos; reached only through sincos and pinned by its vectors.",
  },
  {
    routine: "libm::math::k_cos::k_cos",
    why: "sincos's cosine kernel: a polynomial on the reduced argument, plain f64 arithmetic. The release twin inlines it into sincos; reached only through sincos and pinned by its vectors.",
  },
  {
    routine: "libm::math::rem_pio2::rem_pio2",
    why: "sincos's reduction by pi/2 for |x| above pi/4: up to 3 pi/4, which holds every argument facing_of makes, one subtraction of pi/2 in two parts, except near pi/2 itself (the medium path below); plain f64 arithmetic and bit reads (its x87 branch is for 32-bit x86 without SSE2, which we do not build). Reached only through sincos; pinned by its vectors, which hold the reduction's thresholds and their neighbours.",
  },
  {
    routine: "libm::math::rem_pio2::rem_pio2::medium",
    why: "rem_pio2's path for an argument whose high word is 0x3ff921fb (just under pi/2: a heading's rest above about 89.9999991 degrees, rust-reviewer's block 17, B17-8): up to three rounds of subtraction, plain f64 arithmetic. Pinned by sincos's vectors, whose 1,000 seeded rests on (89.9999991, 90) degrees and edges take this path.",
  },
  {
    routine: "libm::math::rem_pio2_large::rem_pio2_large",
    why: "Linked, unreachable on facing_of's [0, 360) domain: rem_pio2 calls it only for |x| of at least 2^20 pi/2, and facing_of's argument lies below pi/2 (content refuses any other heading, crates/content/src/map.rs's FACING range). It is the one target-dependent code of this list: its table of the bits of 2/pi holds 66 words on a 32-bit target (WebAssembly) and 690 on a 64-bit one (the server), and a debug assertion runs on 64-bit targets only, so it is not the same code on every target. No input of the game reaches it; a later caller that could would bring it back to review.",
  },
  {
    routine: "libm::math::rem_pio2_large::rem_pio2_large::floor",
    why: "rem_pio2_large's own wrapper of libm's floor (below), inside the unreachable routine above: linked, never run on facing_of's domain.",
  },
  {
    routine: "libm::math::floor::floor",
    why: "libm's floor, called only by rem_pio2_large (unreachable, above). Exact by definition (the largest whole number not above x): one answer for every double; on our targets it takes the generic body below, not an intrinsic.",
  },
  {
    routine: "libm::math::generic::floor::floor",
    why: "The generic body of libm's floor: integer masks on the bits, exact. Reached only through floor, and so only through the unreachable rem_pio2_large.",
  },
  {
    routine: "libm::math::generic::floor::floor_status",
    why: "The generic floor's own step that also says whether the result was inexact: integer masks on the bits, exact. Reached only through floor, as above.",
  },
  // Helpers both families share; the release twin inlines every one.
  {
    routine: "libm::math::get_high_word",
    why: "The high 32 bits of a double, by its bits (an integer shift): exact, no arithmetic. Called by sincos; pinned with it.",
  },
  {
    routine: "libm::math::support::env",
    why: "libm's carrier of a result and its floating-point status (FpResult's new and ok), used by the generic floor: it moves values and status words, no arithmetic.",
  },
  {
    routine: "libm::math::support::float_traits",
    why: "libm's float trait for f64 (is_sign_negative, to_bits, from_bits as the build names them): bit reads and bit casts, exact, no arithmetic. Used by the generic floor.",
  },
  {
    routine: "libm::math::support::float_traits::Float::ex",
    why: "The float trait's biased exponent of a double, by its bits (a shift and a mask): exact. Used by exp_unbiased below, for the generic floor.",
  },
  {
    routine: "libm::math::support::float_traits::Float::exp_unbiased",
    why: "The float trait's unbiased exponent of a double: the biased one less the bias, integer steps, exact. Used by the generic floor.",
  },
  {
    routine: "libm::math::support::float_traits::Float::from_parts",
    why: "The float trait's double from a sign, an exponent and a mantissa, by bits (shifts and ors): exact. Used by the generic scalbn.",
  },
  {
    routine: "libm::math::support::float_traits::Float::is_sign_positive",
    why: "The float trait's sign test of a double, by its top bit: exact. Used by the generic floor.",
  },
  {
    routine: "libm::math::support::int_traits",
    why: "libm's integer traits (cast_from, cast, signed and unsigned between i32, u32, i64 and u64, as the build names them): integer conversions, the same on every target. Used by the generic scalbn (cast_from, cast), and by the float trait and the generic floor (signed, unsigned).",
  },
];
