// The maths routines the players' WebAssembly may hold beside plain instructions (build-checks.mjs, libmProblems):
// functions of the `libm` crate, of the toolchain's `compiler_builtins` (`math`, `float`), and C-named ones.
// Every other one stops the build. An entry joins this list in the change that first links its routine, with the
// reason its result is the same bits on every target; a fused multiply-add and the functions of BANNED_MATHS are
// refused whatever this list says.
//
// Empty today: the simulation calls no maths routine. Each entry: { routine: "libm::math::sqrtf", why: "..." }.

/** @type {{ routine: string, why: string }[]} */
export const MATHS_ALLOWED = [];
