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
  comments are in std/core .rmeta (strings dump in scratch/std-rmeta-strings.txt:4793-4799): 1.97.0's f32 div_euclid doc
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
- 07:52 own test (scratch/euclid_test.rs, built rustc 1.97.0 -O, x86_64-unknown-linux-gnu; raw output euclid_test.raw;
  exact checker euclid_ref.py with Fractions; summary euclid_ref.out). FIX lines: f32 a=40400001 b=3f800001 a/b=3
  div_euclid=3 rem_euclid=1 2*b+1==a:true q*b+r==a:false; f64 a=4008000000000001 b=3ff0000000000001: the same (3, 1).
  Mirrors (exact -3, -2, 3): got -4, -3, 4 (all off by one); rem_euclid right in all four. Sweep: f32 200,000 pairs
  (|a| in [1, 2^13), |b| in [1, 2)): div_euclid wrong in 8 (+4 mirrors = 12/200,004, 0.006%), rem_euclid wrong in 0;
  f64 200,000 pairs: 0 wrong for both (rarer at 53 bits; the fixed f64 case still fails). Upheld on (2).
