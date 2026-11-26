# rust-reviewer: review round 1 of change-set 4b-1 "Content" (head 7cc46a1 = ef2edec + brief; branch cs/4b-content)

Brief: .checkpoints/cs4b-content/review-brief-4b1.md. Target folder: .gate/rust-reviewer-4b1-target (every cargo call,
--locked). Scratch: .gate/rust-reviewer-4b1-scratch/ (copies only) and scratch text beside this file.

## Plan
1. Read crates/content/src: lib.rs (lint guard), error.rs, json.rs, number.rs, write.rs, read.rs, bundle.rs, map.rs,
   time.rs, movement.rs. Panics, overflow, `as` casts, recursion bounds, slices, float->int; work/memory bound by
   input length and caps; float ops used and their exactness on both targets; no hash map, no clock.
2. number.rs in depth: RFC 8259 grammar, ECMAScript Number::toString, even_of_a_tie reasoning and arithmetic
   (subnormals, powers of two); any other way std's shortest text can differ from ECMAScript's.
3. devtools: content_proofs.rs (is each class's verdict independent of the loader?), time_report.rs, files.rs,
   wasm.rs, bin/time_report.rs; tests content_proofs, content_fuzz (allocator SAFETY, threads, shares), aim_reach,
   time_unit, recordings, two_instances, two_processes; sim/src/time.rs.
4. Speed: cost per byte at the 8 MiB cap, duplicate-key search, canonical compare; anything quadratic.
5. Runs: cargo clippy -D warnings (workspace, all targets), cargo test -p content, -p devtools (debug), release fuzz;
   scratch tests: 3 ties + 3 near-ties through number_text; corner 0.1+0.2; deep nesting; bundle whose file text holds
   the bundle's closing lines.
6. Departures 1, 2, 3, 6, 7, 8, 10, 13: a verdict each.
7. Final: verdict + findings (severity | file:line | what | evidence | fix), not checked.

## Findings (a line each, as found)
- [json.rs read] recursion bounded (containers refused at depth > 32 before entry, json.rs:204); nodes capped 16,384
  before each value (:199-202); numbers refused unread past 25 bytes (:404); path string bounded (32 levels x <= 66 B).
  Duplicate keys: index sort per object, O(m log m) (:308-323). One `as` (:407, constant 25). No panic path seen.
- [number.rs, PROVISIONAL, to confirm by run] even_of_a_tie (number.rs:81-103) checks only that the double sits exactly
  halfway (decimals == places-1); it never checks that the even neighbour lies inside the rounding interval. For a power
  of two that interval is lopsided (half as wide below). 2^-24 = 5.9604644775390625e-8: 16-digit neighbours ...062e-8 and
  ...063e-8 are 5e-24 away; upper margin 2^-77 = 6.6e-24 (valid), lower margin 2^-78 = 3.3e-24 (NOT valid). So std gives
  ...063 (right), ECMAScript gives ...063, the code flips to ...062, which reads back as a different double. By my
  algebra ties exist only for x = M*2^P with -25 <= P <= -2; lopsided only at fraction 0; only E = -24 has the upper
  inside and the lower outside (E = -25: both inside, flip right). So exactly +-2^-24.
