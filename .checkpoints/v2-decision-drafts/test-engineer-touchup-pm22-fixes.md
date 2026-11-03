# test-engineer touch-up PM-0004-22 (D3A-1 extension): FIX/NOTE blocks

### FIX PM22-EUCLID
FILE: docs/adr/0004-simulation-core.md
BEFORE:
| PM-0004-22 | The maths library is one implementation everywhere (S7; part 7; decides R1-DET-OQ8 and R1-DET-OQ10) | for each `libm` function the sim uses, and for `sqrt`: at least 10,000 fixed inputs (a committed table of edge cases and seeded values: proposed) give bit-identical outputs (`to_bits`) on native x86-64 and in Wasm in Node and in each browser engine; `sqrt` also equals a correctly rounded reference; part 7's `min`, `max` and `clamp` helpers give the same bits on every target for equal signed zeros, (+0, −0) and (−0, +0), and a clamp with its bounds the wrong way round returns a bound with no panic (part 7: one defined argument on equal inputs); reported: each function's largest error in units in the last place against a higher-precision reference (R1-DET-OQ8's "enough" read as PM-0004-30's 1 mm apex and PM-0004-54's 1 cm cursor marks passing with `libm`: proposed), and `libm`'s cost against std's, native and in Wasm | CI; MAC (Safari); RS and ML (the cost) | unit vectors in `cargo test` and wasm-bindgen-test (R5-TEST-REC1) | 20 identical verdicts. Bites: one output moved by one unit in the last place on one target | R1-DET-REC2 (lines 2165–2168); R1-DET-OQ8 (lines 2704–2705); R1-DET-OQ10 (lines 2710–2712) |
AFTER:
| PM-0004-22 | The maths library is one implementation everywhere (S7; part 7; decides R1-DET-OQ8 and R1-DET-OQ10) | for each `libm` function the sim uses, and for `sqrt`: at least 10,000 fixed inputs (a committed table of edge cases and seeded values: proposed) give bit-identical outputs (`to_bits`) on native x86-64 and in Wasm in Node and in each browser engine; `sqrt` also equals a correctly rounded reference; if part 7's `rem_euclid` or `div_euclid` is to join the allow-list (neither is on it, so this row's pass today does not wait on them), the method gets the same check, at least 10,000 fixed inputs with (3 + 2^-22, 1 + 2^-23) in `f32`, (3 + 2^-51, 1 + 2^-52) in `f64` and their sign mirrors among them, and its outputs also equal by value (`==`, which leaves a zero's sign to `to_bits`) a correctly rounded Euclidean reference, the exact quotient and remainder by integer or rational arithmetic rounded once to the type: for (a, b) and its mirrors (−a, b), (a, −b) and (−a, −b), the quotients 2, −3, −2 and 3 and the remainders 1, 2^-23, 1 and 2^-23 in `f32` and 1, 2^-52, 1 and 2^-52 in `f64` (verifiers fixcheck-d3a and skeptic-6 and the Rust and test reviews, 2026-11-03); part 7's `min`, `max` and `clamp` helpers give the same bits on every target for equal signed zeros, (+0, −0) and (−0, +0), and a clamp with its bounds the wrong way round returns a bound with no panic (part 7: one defined argument on equal inputs); reported: each function's largest error in units in the last place against a higher-precision reference (R1-DET-OQ8's "enough" read as PM-0004-30's 1 mm apex and PM-0004-54's 1 cm cursor marks passing with `libm`: proposed), and `libm`'s cost against std's, native and in Wasm | CI; MAC (Safari); RS and ML (the cost) | unit vectors in `cargo test` and wasm-bindgen-test (R5-TEST-REC1) | 20 identical verdicts. Bites: one output moved by one unit in the last place on one target; and, for the Euclidean clause when it runs, errors every target shares: a `div_euclid` planted as rustc 1.97.0's steps (`trunc(a / b)`, corrected by one only when `a % b < 0`: skeptic-6's disassembly, 2026-11-03), which gives 3 for (3 + 2^-22, 1 + 2^-23) where the exact quotient is 2, and one `rem_euclid` output moved by one unit in the last place on every target alike | R1-DET-REC2 (lines 2165–2168); R1-DET-OQ8 (lines 2704–2705); R1-DET-OQ10 (lines 2710–2712) |
END

### NOTE PM22-EVIDENCE
Owner's check of rust-reviewer's NOTE D3A-1-PM22 (rust-reviewer-touchup-d3a-fixes.md) against part 7 as it stands
(0004:257-277, commit 7cc6942), skeptic-6's verdict (verifier-skeptic-6-d3a-1.md) and rr_euclid.out. No fetch; nothing
installed; rustc 1.97.0 (2d8144b78 2026-07-07), x86_64-unknown-linux-gnu, -O. The NOTE holds; FIX PM22-EUCLID is its
extension with four changes, each with its evidence below.
1. Own recomputation (test-engineer-touchup-pm22-scratch/te_euclid_ref.py, .out): exact integers on the inputs scaled
   by 2^23 (A = 3*2^23 + 2, B = 2^23 + 1) and 2^52, cross-checked with Fractions. 2B = A - 2^23 and 3B = A + 1 > A, so
   the Euclidean quotient of (a, b) is 2 with remainder 1. For (a, b), (-a, b), (a, -b), (-a, -b): quotients 2, -3, -2,
   3; remainders 1, 2^-23, 1, 2^-23 (f64: 1, 2^-52, 1, 2^-52). An IEEE emulation of rustc 1.97.0's steps (trunc(a / b),
   corrected by one when fmod < 0: skeptic-6's de32.disasm) gives 3, -4, -3, 4 and the right remainders. All 8 agree
   with rr_euclid.out, with skeptic-6's euclid_test.fixed-cases.out (mirror bits c0800000 = -4, 34000000 = 2^-23) and,
   for (a, b), with fixcheck-d3a's div_euclid_check.out. te_rem_round.out re-runs the pair on rustc: 3 and 1.
2. "rounded once to the type" (added): an exact Euclidean remainder need not fit the type. te_rem_round.out: rustc
   1.97.0's `(-2^-30f32).rem_euclid(1.0)` = 1.0 (bits 3f800000), which is |b|, the correct rounding of 1 - 2^-30 (std's
   promise is "the rounded infinite-precision result"). A reference left unrounded would fail a correct method: a test
   failing for the test's own fault. skeptic-6's sweep already compared "the correctly rounded exact remainder".
3. "equal by value (`==`, which leaves a zero's sign to `to_bits`)" (added): te_signed_zero.out: rustc 1.97.0 gives
   `0.5f32.div_euclid(-1.0)` = -0.0 (bits 80000000) and `(-1f32).rem_euclid(1.0)` = -0.0, where the exact value is the
   integer 0, which has no sign. A bit comparison against a reference built from integers would fail there; the sign of
   the zero is the cross-target `to_bits` clause's to check, which the methods also get ("the same check, at least
   10,000 fixed inputs": same bits on every target, then the reference).
4. Bites (reworded from the NOTE): the table's Bites rule (0004:1091-1093) wants a planted defect that makes the mark
   fail on the spike's own build. The pinned toolchain is ADR 0002's (1.99.1 when read), whose `div_euclid` code is
   unverified (no fetch), so a bare "rustc 1.97.0's div_euclid" may not be on the build to fail. The plant is therefore
   a copy of rustc 1.97.0's steps (skeptic-6's disassembly), which bites whichever way 1.99.1 turns out; and since
   1.97.0's `rem_euclid` is right (8 of 8 here, 0 of 200,004 in skeptic-6's sweep), the `rem_euclid` arm gets its own
   plant (one output moved by one ulp on every target alike: proposed, as the rule allows). Both are errors every
   target shares, which the existing bite (one ulp on one target) does not cover.
5. Today's run: the clause opens "if part 7's `rem_euclid` or `div_euclid` is to join the allow-list (neither is on it,
   so this row's pass today does not wait on them)"; the Bites addition says "when it runs". The `sqrt`, `libm` and
   helper clauses are word for word unchanged.
6. Part 7 needs no change: its "equal an exact reference" reads as the row defines it (the exact value, rounded once).
   If rust-reviewer wants the words to match exactly, "an exact reference" could become "a correctly rounded
   reference"; not proposed here (part 7 is rust-reviewer's, and its 16 lines are fitted).
7. Checks (scratch tree laid out as the repository, test-engineer-touchup-pm22-scratch/docs and /repo_after/docs):
   apply_fixes.py --dry-run: ok, 1 line -> 1, ADR 0004 stays 1,410 lines; the row has 7 cells (8 pipes) before and
   after, no straight quote added; glossary_check_drafts.py (its two paths pointed at the scratch trees, gl_base.py and
   gl_after.py), with and without --skip-records: exact hits 0 before and after, ADR 0004 in context 98 both, the hit
   lists identical; cite_check.py and draft_pointers.py: output identical before and after. No table edge touched.
8. Not verified: a Wasm run (no wasm32 target installed; nothing installed) and rustc 1.99.1's `div_euclid` code (no
   fetch). The clause is written so it holds either way.
END
