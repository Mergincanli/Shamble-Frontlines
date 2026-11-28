// The maths routines the players' WebAssembly may hold beside plain instructions (build-checks.mjs, libmProblems):
// functions of the `libm` crate, of the toolchain's `compiler_builtins` (`math`, `float`), and C-named ones.
// Every other one stops the build. An entry joins this list in the change that first links its routine, with the
// reason its result is the same bits on every target; a fused multiply-add and the functions of BANNED_MATHS are
// refused whatever this list says.
//
// Two entries, both brought by the content loader, which the players' constructor links from change-set 4b-2's step
// 9 on (rust-reviewer's decision of 2026-11-27): each runs when a bundle loads, never per tick. The simulation's own
// two (crates/sim/src/fmath.rs: libm's exp and sincos, since change-set 4c's C3) are not linked into the players' file
// until its constructor calls them (4c's C4, which brings their entries), and the static collision world links none (its tree's build and its rays are additions,
// subtractions, multiplications, divisions and comparisons; rust-reviewer's read of 2026-11-27). Each entry names a
// routine by its path as the build check prints it, and covers the steps below it:
// { routine: "libm::math::sqrt", why: "..." } covers libm::math::sqrt::sqrtf; a C-named one is "C::round".

/** @type {{ routine: string, why: string }[]} */
export const MATHS_ALLOWED = [
  {
    routine: "C::round",
    why: "f64::round, rounding to a whole number with halves away from zero. Called when a bundle loads, never per tick: the map's grid check (crates/content/src/map.rs, on_grid: a coordinate times its steps per metre, rounded, must divide back to the same double), once per box corner. The operation is exact (IEEE 754 roundToIntegralTiesToAway): every double that is not a NaN has one answer, the sign of zero included, so any correct routine gives the same bits, natively, in every browser and on a server. The toolchain's routine here is one f64.add of x and 0.5 - 2^-54 with x's sign (f64.copysign), then a cut to a whole number by integer bit masks: no multiply, no fused multiply-add, no vector instruction, and WebAssembly's f64.add rounds to nearest on every engine. A NaN's bits are not promised; no NaN reaches it (on_grid's value is a finite parsed number, its scale 100 or 1,000). rust-reviewer, 2026-11-27: the twin's own body in Node 22 (V8) and native x86_64 f64::round gave the same bits on 4,006,064 inputs, none off an exact reference.",
  },
  {
    routine: "C::__multi3",
    why: "The toolchain's 128-bit integer multiply (the low 128 bits of a product), which WebAssembly has no instruction for. Called by the loader's number reading and writing, when a bundle loads, never per tick: core's f64 from text (dec2flt, Eisel-Lemire's 64 x 64-bit products), core's shortest and exact texts of an f64 (flt2dec, Grisu: crates/content/src/number.rs, number_text, and the refusal sentences of crates/content/src/error.rs), core's text of a u128, and the tie rule for number texts (number.rs, even_of_a_tie: 5 to the decimals times the odd mantissa, checked). Integer arithmetic: the product modulo 2^128 is one number on every target, and Rust defines checked_mul's overflow, so the bits are the same natively, in every browser and on a server. The routine here is 64-bit integer multiplies, additions, shifts and compares only, no float. rust-reviewer, 2026-11-27: the twin's own body in Node 22 (V8) matched BigInt's product on 900,144 operand pairs.",
  },
];
