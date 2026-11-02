# rust-reviewer: follow-up d on ADR 0004 (verifier fixcheck-d0 row 9), from the doc on disk (2026-11-02, about 21:05 Amsterdam)

Skeptic check: verifier-fixcheck-d0-scratch/excerpts.md (X9) holds the quotes, from doc.rust-lang.org std 1.99.1 f32 and f64 pages fetched 20:34 (audit.tsv rows 4-5: 200). div_euclid carries "§Precision The result of this operation is guaranteed to be the rounded infinite-precision result."; the rem_euclid excerpt is abridged (txt 475-489, heading elided) but has the same words; abs: "This function always returns the precise result." The operator-expr excerpt gives no precision for the float `%`. This matches the std source as I know it, so the finding holds. My call: rem_euclid and div_euclid stay off until PM-0004-22 confirms them, as row 9 proposes; no sim code needs them yet, and the allow-list admits on evidence.

### FIX fixcheck-d0-9 (part 7: abs and copysign, their sources)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
   `mul_add`, which std specifies exactly (R1-DET-F10, r1-simulation-core.md, Findings, lines 655–660); `abs` and
   `copysign` (proposed: sign-bit operations, exact by definition); `as` casts (R1-DET-F15, lines 684–692); and the
AFTER:
   `mul_add`, which std specifies exactly (R1-DET-F10, r1-simulation-core.md, Findings, lines 655–660); `abs` (std
   1.99.1's `f32` page: "This function always returns the precise result") and `copysign` (proposed: a sign-bit
   operation; std warns only of a NaN sign, which this part's no-NaN rule excludes); `as` casts (R1-DET-F15, lines
   684–692); and the
END

### FIX fixcheck-d0-9 (part 7: rem_euclid, div_euclid and the float %)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
   bound); a degree value becomes radians by one multiply with a pinned constant. `rem_euclid`, `div_euclid` and the
   float `%` operator are not on the list: one joins only once PM-0004-22's unit vectors show it bit-identical on every
   target (proposed: no source read here states their precision; review, not the lint, catches the operator).
AFTER:
   bound); a degree value becomes radians by one multiply with a pinned constant. `rem_euclid`, `div_euclid` and the
   float `%` operator are not on the list. std 1.99.1 guarantees `rem_euclid` and `div_euclid` "the rounded
   infinite-precision result" (its `f32` and `f64` pages, read by the verifier on 2026-11-02), so they may join once
   PM-0004-22's unit vectors confirm it on every target; no source read states the float `%`'s precision (the Rust
   Reference defines it only by truncating division and the dividend's sign), and review, not the lint, catches the
   operator.
END
