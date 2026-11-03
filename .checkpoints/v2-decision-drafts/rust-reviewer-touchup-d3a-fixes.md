# rust-reviewer touchup d3a: fixes (D3A-1)

### FIX D3A-1
FILE: docs/adr/0004-simulation-core.md
BEFORE:
   684–692); and the
   exact non-arithmetic methods the rules themselves use, such as `is_finite`, `is_nan`, `to_bits` and `from_bits`.
   Every other std `f32` and `f64` method is banned, and ADR 0002's lint is this list's complement. Among them are
   `powi`, `hypot`, `cbrt`, `to_degrees` and `to_radians`, which std 1.99.1 marks non-deterministic as it does the
   transcendental functions (R1-DET-F9, lines 648–654; rust-reviewer read the note on all five at the step-10
   review), and `min`, `max` and `clamp`, which may return either zero for equal signed zeros, while `clamp` panics on
   bad bounds (R1-DET-F14, lines 679–683). In their place the sim has three helpers of its own, written as plain
   comparisons (`if a < b { a } else { b }` and the like): on equal inputs, signed zeros included, each returns one
   defined argument, the same on every target, and none panics (a clamp whose bounds are the wrong way round returns a
   bound); a degree value becomes radians by one multiply with a pinned constant. `rem_euclid`, `div_euclid` and the
   float `%` operator are not on the list. std 1.99.1 guarantees `rem_euclid` and `div_euclid` "the rounded
   infinite-precision result" (its `f32` and `f64` pages, read by the verifier on 2026-11-02), so they may join once
   PM-0004-22's unit vectors confirm it on every target; no source read states the float `%`'s precision (the Rust
   Reference defines it only by truncating division and the dividend's sign), and review, not the lint, catches the
   operator.
   Transcendental maths only
AFTER:
   684–692); and the exact non-arithmetic methods the rules themselves use, such as `is_finite`, `is_nan`, `to_bits` and
   `from_bits`. Every other std `f32` and `f64` method is banned, and ADR 0002's lint is this list's complement. Among
   them are `powi`, `hypot`, `cbrt`, `to_degrees` and `to_radians`, which std 1.99.1 marks non-deterministic as it does
   the transcendental functions (R1-DET-F9, lines 648–654; rust-reviewer read the note on all five at the step-10
   review), and `min`, `max` and `clamp`, which may return either zero for equal signed zeros, while `clamp` panics on
   bad bounds (R1-DET-F14, lines 679–683). In their place the sim has three helpers of its own, written as plain
   comparisons (`if a < b { a } else { b }` and the like): on equal inputs, signed zeros included, each returns one
   defined argument, the same on every target, and none panics (a clamp whose bounds are the wrong way round returns a
   bound); a degree value becomes radians by one multiply with a pinned constant. `rem_euclid`, `div_euclid` and the
   float `%` operator are not on the list. std's pages promise both methods "the rounded infinite-precision result"
   (1.99.1's, read by the verifier on 2026-11-02, and 1.97.0's), yet 1.97.0's `div_euclid` gives 3, not 2, for
   (3 + 2^-22) ÷ (1 + 2^-23) in `f32`, as does its `f64` analogue (verifiers fixcheck-d3a and skeptic-6, 2026-11-03). A
   shared error passes a same-bits check, all determinism needs; correct rounding is a separate claim, so both may join
   only once fixed inputs, those cases and their sign mirrors included, equal an exact reference on every target
   (PM-0004-22, extended); no source read states `%`'s precision (the Rust Reference defines it only by truncating
   division and the dividend's sign), and review, not the lint, catches the operator. Transcendental maths only
END

### NOTE D3A-1-PM22
For test-engineer (PM-0004-22 is your row; I change no pass mark). Row PM-0004-22 (docs/adr/0004-simulation-core.md:1127),
in its pass mark, after the `sqrt` clause ("`sqrt` also equals a correctly rounded reference;"), add:
"if part 7's `rem_euclid` or `div_euclid` is to join the allow-list, fixed inputs, (3 + 2^-22, 1 + 2^-23) in `f32` and
(3 + 2^-51, 1 + 2^-52) in `f64` with their sign mirrors among them, equal an exact Euclidean reference (quotient and
remainder computed in exact integer or rational arithmetic) on every target;"
and in its "Bites" cell, after "one output moved by one unit in the last place on one target", add:
"; and rustc 1.97.0's `div_euclid` on (3 + 2^-22, 1 + 2^-23), which gives 3 where the exact quotient is 2 on every
target alike".
Why: the row today checks `to_bits` identity across targets and, for `sqrt` only, a correctly rounded reference; its
"largest error in units in the last place" is reported, not gated. rustc 1.97.0's `div_euclid` is `trunc(a / b)`
corrected by one only when `a % b < 0` (skeptic-6's disassembly, verifier-skeptic-6-scratch/de32.disasm), every step
of it exactly rounded IEEE 754 arithmetic, so its wrong quotient is the same bits on every target and a bit-identity
check passes it. Part 7's new condition (FIX D3A-1) names "PM-0004-22, extended": this clause is that extension.
Expected values (exact, by integer arithmetic on the inputs scaled by 2^23 or 2^52; rust-reviewer's run,
rust-reviewer-touchup-d3a-scratch/rr_euclid.out): for (a, b), (-a, b), (a, -b), (-a, -b), the Euclidean quotient is
2, -3, -2, 3 and the remainder 1, 2^-23, 1, 2^-23 (in `f64`, 2^-52); rustc 1.97.0 gives quotients 3, -4, -3, 4 (all
one off) and the right remainders. Line change: none in part 7; the row stays one table line.
END

### NOTE D3A-1-EVIDENCE
Owner's check of D3A-1 and skeptic-6 (no fetch; rustc 1.97.0 (2d8144b78 2026-07-07), x86_64-unknown-linux-gnu, -O;
no wasm32 target installed, so no Wasm run). The finding holds.
1. Own test, rust-reviewer-touchup-d3a-scratch/rr_euclid.rs and .out, against an exact reference in integer arithmetic
   (inputs scaled by 2^23 or 2^52): `div_euclid` is wrong in 8 of 8 cases (the pair and its three sign mirrors, in `f32`
   and in `f64`), `rem_euclid` right in 8 of 8. This agrees with fixcheck-d3a (div_euclid_check.out) and with
   skeptic-6's sweep (f32: 12 of 200,004 pairs wrong for `div_euclid`, 0 for `rem_euclid`; euclid_ref.out).
2. One correction to skeptic-6's proposed words: "returns 3 for (3 + 2^-22) ÷ (1 + 2^-23) ... in `f32` and in `f64`" is
   true only with the `f64` pair scaled: in `f64` the `f32` pair gives the right 2 (rr_f64_same_inputs.out:
   a/b = 2.9999998807907247, div_euclid 2). The failing `f64` pair is (3 + 2^-51, 1 + 2^-52). The FIX therefore says
   "in `f32`, as does its `f64` analogue".
3. std: skeptic-6 decoded rustc 1.97.0's own std metadata (div_euclid_f32_doc_1.97.txt, f64_euclid_docs_1.97.txt): the
   "# Precision ... the rounded infinite-precision result" lines are already there for both methods and both types,
   so 1.99.1's page (the verifier's reading of 2026-11-02) is no evidence of a fix; d3a's "its new §Precision line"
   (verifier-fixcheck-d3a-verdicts.md:112) is wrong in "new" (a verifier record, not draft text: left as is).
   Whether rustc 1.99.1's code differs is unverified; the page to read after 08:30 is the 1.99.1 source of
   `core::num::f32`, fn `div_euclid`. The FIX's condition holds either way, since it is tested on the pinned toolchain.
4. The FIX's shape: lines 262-277 are re-wrapped so the paragraph keeps its 16 lines and ADR 0004 its 1,410 (README's
   line pointers into 0004 stay right; no pointer targets 255-279). Lines 262-271 change no word (the word diff,
   rust-reviewer-touchup-d3a-scratch/wdiff.txt, shows changes only from "std 1.99.1 guarantees" on); the old short
   lines 262 ("684–692); and the"), 276 and 277 are absorbed. In the `%` clause "the float `%`'s precision" becomes
   "`%`'s precision" (the float `%` is named at the sentence's start) to save the line.
5. The decision is unchanged: `rem_euclid`, `div_euclid` and the float `%` stay off the allow-list.
6. A remark for the sim's own code (no draft change): `(a / b).floor()`, which the allow-list permits, rounds the same
   way, so it too gives 3 for the pair; it is deterministic (std specifies `÷` and `floor` exactly), but a grid or
   cell index computed that way can be one off at a cell edge. Where an exact cell matters, the helper should test the
   sign of `q.mul_add(b, -a)` (explicit `mul_add` is on the list; one rounding, so its sign is exact), not `q * b <= a`
   (two roundings), or use integer coordinates. For the spike's review, not this ADR.
7. Glossary check (glossary_check_drafts.py on a scratch copy laid out as the repository, its two paths pointed at it):
   exact hits 0 before and after; ADR 0004 in context 98 before and after, the hit lists identical. No table near.
END
