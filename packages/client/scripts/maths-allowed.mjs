// The maths routines the players' WebAssembly may hold beside plain instructions (build-checks.mjs, libmProblems):
// functions of the `libm` crate, of the toolchain's `compiler_builtins` (`math`, `float`), and C-named ones.
// Every other one stops the build. An entry joins this list in the change that first links its routine, with the
// reason its result is the same bits on every target; a fused multiply-add and the functions of BANNED_MATHS are
// refused whatever this list says.
//
// Empty today: the simulation calls no maths routine, and the static collision world links none (its tree's build
// and its rays are additions, subtractions, multiplications, divisions and comparisons; rust-reviewer's read of
// 2026-11-27). Each entry names a routine by its path as the build check prints it, and covers the steps below it:
// { routine: "libm::math::sqrt", why: "..." } covers libm::math::sqrt::sqrtf; a C-named one is "C::round".

/** @type {{ routine: string, why: string }[]} */
export const MATHS_ALLOWED = [];
