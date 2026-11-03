# verifier-skeptic-6 — skeptic on finding D3A-1 (fixcheck-d3 unit a)

Branch: claude/laughing-lovelace-ox61se. Started 2026-11-03 07:30 Amsterdam (before 08:30: no web fetch).
Target: D3A-1 (docs/adr/0004-simulation-core.md:272-274, div_euclid correctness claim).
Scratch: .checkpoints/v2-decision-drafts/verifier-skeptic-6-scratch/

## Notes
- 07:30 created file. Next: read the finding's file and test, the ADR lines, PM-0004-22.
- 07:33 read finding (d3a verdicts.md:96-122, 182-193), its test (.rs/.out), d0 excerpts.md:118-127, ADR 0004:257-277 and
  PM-0004-22 (0004:1127). Observation: PM-0004-22 names only "each libm function the sim uses, and sqrt"; it does not
  mention rem_euclid/div_euclid at all (grep "euclid" over docs/: only 0004:271-272). Next: arithmetic by hand, own test.
- 07:38 toolchain: rustc 1.97.0 (2d8144b78 2026-07-07); installed targets: x86_64-unknown-linux-gnu only (no wasm32,
  so no Wasm run possible without installing: not done); no rust-src component (lib/rustlib/src absent). The doc
  comments are in std/core .rmeta (strings dump in scratch/std-rmeta-strings-4790-4800.txt (lines 4790-4800 of the full dump, deleted for size)): 1.97.0's f32 div_euclid doc
  already carries "infinite-precision result." after its description. Next: decode the rmeta bytes to see the full
  Precision lines (symbols are deduplicated by offset), then my own test.
- 07:45 rustc 1.97.0's own std carries the same Precision lines (decoded from libstd-ca8536218e109c03.rmeta with
  scratch/rmeta_docs.py; outputs scratch/div_euclid_f32_doc_1.97.txt, rem_euclid_f32_doc_1.97.txt,
  f64_euclid_docs_1.97.txt): for div_euclid and rem_euclid, f32 and f64: "# Precision" / "The result of this operation
  is guaranteed to be the rounded" / "infinite-precision result." So the claim is not new in 1.99.1: 1.97.0 states it
  and (per d3a's test) breaks it for div_euclid. Arithmetic by hand: b = 1 + 2^-23 exact (ulp 2^-23 in [1,2)); 2b + 1 =
  3 + 2^-22 exact (ulp 2^-22 in [2,4)); 3b = 3 + 1.5 * 2^-22 > a, so the Euclidean quotient is 2, r = 1, 0 <= 1 < b;
  a/b exact = 3 - 2^-23/(1 + 2^-23), less than half an ulp (2^-23) below 3, so f32 a / b rounds to 3.0 and trunc gives 3.
  Same for f64 with 2^-52, 2^-51. Upheld so far on (1). Next: own test (fixed case + exact-reference sweep), disassembly.
- 07:52 own test (scratch/euclid_test.rs, built rustc 1.97.0 -O, x86_64-unknown-linux-gnu; raw output not kept (regenerate: rustc -O euclid_test.rs, ./euclid_test > euclid_test.raw); fixed cases in euclid_test.fixed-cases.out;
  exact checker euclid_ref.py with Fractions; summary euclid_ref.out). FIX lines: f32 a=40400001 b=3f800001 a/b=3
  div_euclid=3 rem_euclid=1 2*b+1==a:true q*b+r==a:false; f64 a=4008000000000001 b=3ff0000000000001: the same (3, 1).
  Mirrors (exact -3, -2, 3): got -4, -3, 4 (all off by one); rem_euclid right in all four. Sweep: f32 200,000 pairs
  (|a| in [1, 2^13), |b| in [1, 2)): div_euclid wrong in 8 (+4 mirrors = 12/200,004, 0.006%), rem_euclid wrong in 0;
  f64 200,000 pairs: 0 wrong for both (rarer at 53 bits; the fixed f64 case still fails). Upheld on (2).
- 07:58 implementation (no rust-src; disassembly of the -O build, scratch/de32.disasm, re32.disasm; call targets resolved
  by objdump -R / -d to truncf at 0x58660 and fmodf at 0x57f00): de32 = q = truncf(a / b) [divss then truncf];
  r = fmodf(a, b); if r < 0 then (b > 0 ? q + (-1.0) : q + 1.0) else q. re32 = r = fmodf(a, b); r < 0 ? r + |b| : r.
  So 1.97.0's div_euclid is trunc of the rounded quotient, corrected only by the sign of the exact fmod: when a / b
  rounds up across an integer, nothing corrects it. rem_euclid is one rounding of an exact sum, so it is correctly
  rounded (0 mismatches in the sweep). Node emulation of the same steps under IEEE semantics (scratch/emulate_ieee.js,
  .out): f32 and f64 both give 3, rem 1: every IEEE target running these steps gives the same wrong bits, so a
  cross-target to_bits check passes it (wasm32 target not installed: no Rust-to-Wasm run).
- 07:59 draft check: PM-0004-22 (0004:1127) covers "each `libm` function the sim uses, and for `sqrt`"; it names neither
  rem_euclid nor div_euclid (grep "euclid" over docs/: only 0004:271-272), so "PM-0004-22's unit vectors confirm it"
  points at vectors the row does not define; its pass criteria are to_bits identity across targets and, for sqrt
  only, a correctly rounded reference; the "largest error in ulps" is reported, not gated. ADR 0002:155 pins the exact
  stable Rust (1.99.1 when read), so 1.99.1's code is the one that matters and is unverified (no fetch before 08:30).
  Next: verdict block.

## Verdict on D3A-1: PARTLY (facts upheld; the smallest fix is incomplete)

1. Arithmetic: upheld. In f32, b = 1 + 2^-23 and a = 3 + 2^-22 are exact; 2b + 1 = a exactly; 3b = 3 + 1.5 * 2^-22 > a,
   so the Euclidean quotient is 2 (remainder 1, 0 <= 1 < b). a / b = 3 - 2^-23/(1 + 2^-23) is under half an ulp below
   3, so it rounds to 3.0. f64 with 2^-52 and 2^-51: the same.
2. Test: upheld. scratch/euclid_test.rs on rustc 1.97.0 (2d8144b78 2026-07-07), x86_64-unknown-linux-gnu, -O:
   "FIX f32 a=40400001 b=3f800001 a/b=3 div_euclid=3 rem_euclid=1 2*b+1==a:true q*b+r==a:false"; f64 the same. Sign
   mirrors also wrong (exact -3, -2, 3; got -4, -3, 4). Exact-reference sweep (euclid_ref.py, Fractions): f32
   12/200,004 wrong for div_euclid, 0 for rem_euclid; f64 0/200,000 (the fixed f64 case still fails). Wasm: no wasm32
   target installed, not run; the Node emulation of the same IEEE steps gives 3 (emulate_ieee.out).
3. std: rustc 1.97.0's own std (decoded from its .rmeta, no rust-src) already carries, for div_euclid and rem_euclid in
   f32 and f64: "# Precision" / "The result of this operation is guaranteed to be the rounded" / "infinite-precision
   result." So the guarantee is not new in 1.99.1: 1.97.0 states it and its div_euclid breaks it. Implementation
   (disassembly): q = trunc(a / b), minus or plus 1 only when fmod(a, b) < 0; nothing corrects a quotient that rounds
   up across an integer. rem_euclid = fmod, plus |b| when negative: one rounding, correct. d3a's line "its new
   §Precision line" (verdicts.md:112) is wrong in "new", outside the finding itself.
4. The draft: part 7's "so they may join once PM-0004-22's unit vectors confirm it on every target" does not hold.
   (a) PM-0004-22 (0004:1127) defines vectors only for "each `libm` function the sim uses, and for `sqrt`": none for
   rem_euclid or div_euclid. (b) Its pass criteria are to_bits identity across targets, plus a correctly rounded
   reference for sqrt only (the ulp error is reported, not gated). div_euclid is built from `/`, trunc, fmod and +-1,
   all exact-by-IEEE steps, so its wrong result is the same bits on every target: a bit-identity check passes it.
   Determinism needs the same bits; the quoted guarantee promises correct rounding: the test as written checks the
   first and the sentence claims the second.
   The finding's claims hold. Its fix leaves (a) open: "PM-0004-22's unit vectors, that case among them" names vectors
   PM-0004-22 does not have.

Corrected smallest fix (part 7, 0004:272-274): "std's `f32` and `f64` pages state that `rem_euclid` and `div_euclid`
return "the rounded infinite-precision result" (std 1.99.1's pages, read by the verifier on 2026-11-02; rustc 1.97.0's
std carries the same lines), but rustc 1.97.0's `div_euclid` returns 3 for (3 + 2^-22) / (1 + 2^-23), whose exact
Euclidean quotient is 2, in `f32` and in `f64` (verifiers fixcheck-d3a and skeptic-6), and an error every target shares
passes a bit-identity check; so they may join only once PM-0004-22, extended to them, shows that its fixed inputs (that
case and its sign mirrors among them) equal an exact Euclidean reference on every target". And in PM-0004-22's check
(0004:1127), after the `sqrt` clause: "if part 7's `rem_euclid` or `div_euclid` is to join the list, the same fixed
inputs, (3 + 2^-22, 1 + 2^-23) and its sign mirrors among them, equal an exact Euclidean reference on every target".

Not verified: rustc 1.99.1's div_euclid code (ADR 0002:155 pins 1.99.1; no fetch before 08:30; after it, the place to
read is doc.rust-lang.org/1.99.1/src/core/num/f32.rs.html, fn div_euclid); a real Wasm run (no wasm32 target, nothing
installed). The fix's wording holds whichever way 1.99.1 turns out.

- 08:02 verdict written. DONE
- 08:04 scratch trimmed (binary, full strings dumps and the 400,000-line raw output removed; excerpts kept). DONE
