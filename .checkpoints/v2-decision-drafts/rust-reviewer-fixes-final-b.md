# rust-reviewer: follow-up b on ADR 0004 (RR-1, the ADR 0004 half), at b815c85

Author: rust-reviewer, 2026-11-02. Every BEFORE is copied by script from `git show b815c85:docs/adr/0004-simulation-core.md` (whole lines, non-overlapping, in file order).

### FIX RR-1 (part 7: the float allow-list and the min, max and clamp helpers)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
   R4-PHYS-REC5), while time, counts and money are integers (parts 4 and 17); only `+ − × ÷`,
   `sqrt`, `floor`, `ceil`, `round`, `trunc`, explicit `mul_add` and `as` casts from core; transcendental maths only
   through the pure-Rust `libm` crate, pinned; no std `sin`, `cos`, `atan2`, `powf` or `exp`, no `algebraic_*`, no
AFTER:
   R4-PHYS-REC5), while time, counts and money are integers (parts 4 and 17). The float allow-list: `+ − × ÷` and
   negation; the comparisons and `total_cmp`; `sqrt`, `floor`, `ceil`, `round`, `trunc`, `fract` and explicit
   `mul_add`, which std specifies exactly (R1-DET-F10, r1-simulation-core.md, Findings, lines 655–660); `abs` and
   `copysign` (proposed: sign-bit operations, exact by definition); `as` casts (R1-DET-F15, lines 684–692); and the
   exact non-arithmetic methods the rules themselves use, such as `is_finite`, `is_nan`, `to_bits` and `from_bits`.
   Every other std `f32` and `f64` method is banned, and ADR 0002's lint is this list's complement. Among them are
   `powi`, `hypot`, `cbrt`, `to_degrees` and `to_radians`, which std 1.99.1 marks non-deterministic as it does the
   transcendental functions (R1-DET-F9, lines 648–654; rust-reviewer read the note on all five at the step-10
   review), and `min`, `max` and `clamp`, which may return either zero for equal signed zeros, while `clamp` panics on
   bad bounds (R1-DET-F14, lines 679–683). In their place the sim has three helpers of its own, written as plain
   comparisons (`if a < b { a } else { b }` and the like): on equal inputs, signed zeros included, each returns one
   defined argument, the same on every target, and none panics (a clamp whose bounds are the wrong way round returns a
   bound); a degree value becomes radians by one multiply with a pinned constant. `rem_euclid`, `div_euclid` and the
   float `%` operator are not on the list: one joins only once PM-0004-22's unit vectors show it bit-identical on every
   target (proposed: no source read here states their precision; review, not the lint, catches the operator).
   Transcendental maths only
   through the pure-Rust `libm` crate, pinned; no std `sin`, `cos`, `atan2`, `powf` or `exp`, no `algebraic_*`, no
END

### FIX RR-1 (part 7: the Why line)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
   - Why: basic operations, `sqrt` and `mul_add` are exactly specified on every target while std's transcendental
     functions are not; NaN bits and relaxed SIMD differ by engine; with server authority a divergence costs a
     correction, not a desync.
AFTER:
   - Why: basic operations, `sqrt` and `mul_add` are exactly specified on every target while std's transcendental
     functions are not, and neither are `powi`, `hypot`, `cbrt`, `to_degrees` and `to_radians`; std's `min`, `max` and
     `clamp` leave signed zero open, which an allow-list and three helpers close where a deny-list of transcendental
     names would not; NaN bits and relaxed SIMD differ by engine; with server authority a divergence costs a
     correction, not a desync.
END

### FIX RR-1 (part 20: the clamp hazard removed by construction)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
    reach an `unwrap`, an unchecked index or a `clamp` with unchecked bounds (`f32::clamp` panics on bad bounds,
    R1-DET-F14). On the server a panic in the tick ends that room's match and its process (each room is its own
AFTER:
    reach an `unwrap` or an unchecked index, and std's `clamp`, which panics on bad bounds (R1-DET-F14), is never
    called: part 7's clamp helper returns a bound instead. On the server a panic in the tick ends that room's match and
    its process (each room is its own
END

