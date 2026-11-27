# rust-reviewer: the names content brings into the players' file (4b-2, step 9, cut C, its first step)

Asked by Builder V6 (the main session), 2026-11-27. The question is yours to answer; no answer is proposed here.

## What changed
Step 9's cut C makes the players' constructor load the content bundle: `Sim::create(seed, mode, pad_bytes, bundle)`
in `crates/sim-wasm/src/lib.rs` calls `content::load(bundle)` (the content crate, `crates/content/src/`). So the
content crate is linked into the players' file for the first time. The page's build walks that file with the
built-file check (`packages/client/scripts/build-checks.mjs`: `twinProblems`, `productionProblems`, `libmProblems`,
`bannedMathsProblems`, `fusedMultiplyAddProblems`, `PLAYERS_IMPORTS`) against the allow-list
`packages/client/scripts/maths-allowed.mjs` (empty today). Builder V4's notes saw, in the test-only module that
already links content, `f64::round` arriving as `C::round` and a 128-bit multiply as `C::__multi3`
(`.checkpoints/cs4b-content.md`, the entry "Step 10, the core", and `rust-reviewer-step10.md`); step 7's notes found
`f32::round` arriving as the C name `roundf` alone, with no `compiler_builtins` path beside it.

## The build's own lines
(filled in from the wasm-only build's output when it is run: `node packages/client/scripts/build.mjs --wasm-only`)

## What is asked
For each name the check refuses: an entry in `maths-allowed.mjs` with its reason (what calls it, why its result is
the same bit for bit on every target the game runs on: native, Chromium, Firefox, a server), or a change of the
code so the name does not arrive (which code, in which crate; if it is the content crate, 4b-1's proofs run again
before anything is built on it). His standing words that bind it: MUL-ADD (no fused multiply-add in the players'
file); FEATURE-COMPARE (a)'s conditions (the built-file check keeps looking for SIMD and fused multiply-add in the
players' file). If anything other than maths names fires (the exact imports list `PLAYERS_IMPORTS`, a machine path,
the boundary's shape), say so and stop: the imports list is security-auditor's control.
