# rust-reviewer: review round 2 of change-set 4b-1 "Content" (code head d36e416, notes commit 495204a; round 1 read ef2edec)

Brief: .checkpoints/cs4b-content/review-brief-4b1-round2.md (+ round 1's "Rules for you"). Diff read:
`git diff ef2edec d36e416 -- crates packages`. Target folder: .gate/rust-reviewer-4b1-target (every cargo call,
--locked). Scratch: .gate/rust-reviewer-4b1-scratch/ (copies only) and scratch text beside this file.

## Plan
1. Read the whole diff ef2edec..d36e416 (crates, packages); list the files touched.
2. RR-1: number.rs even_of_a_tie read-back; unit cases; re-run my 31.75 M double sweep (scratch from round 1) against
   the fixed crate; check the doc's wording ("agree on every finite number"); corpus generator's ties/powers.
3. SA-1: json.rs string(): pre-scan vs read agreement (escapes, refusals, overflow of the length), linear cost;
   with_capacity bound by the input; fifth limit case.
4. SA-3: is_id device names (case, extensions, COM0-9/LPT0-9, superscripts, CONIN$...), what the id grammar allows.
5. files.rs: read_plain one handle + take; write_plain name check; recordings scan.
6. rules_scan.rs: in_tests, content_breaches, plants and controls; probe the token scan with my own cases in a copy.
7. content_proofs.rs: LIMIT_CASES / limit_case(at), random_id's device rule, echo oracle with the quote.
8. dependency_rules.rs: dotted keys before the first table, build.rs assert.
9. RR-4/RR-5/nits: SAFETY note, fold comment/threshold, "slowest by the clock", saturating_mul.
10. Runs: fmt --check, clippy -D warnings (workspace all targets; wasm32 libs), cargo test -p content, -p devtools,
    -p sim --test rules_scan; release fuzz if time.
11. Final: per RR finding closed/open; new findings; not checked.

## Findings (a line each, as found)
- [diff read] 19 files changed (git diff --stat ef2edec d36e416 -- crates packages). Content: error.rs (echo quote,
  record doc), json.rs (is_id devices, string() pre-scan), map.rs (Ramp{rises_toward}, group gone, ranges test),
  number.rs (read-back), time.rs (wording). devtools: content_proofs.rs, files.rs, time_report.rs, tests (fuzz,
  proofs, dependency_rules, recordings). sim: time.rs wording, tests/rules_scan.rs (content scan). Corpus 20,360 lines.
- [RR-1 reasoning] read-back `format!("{even}e-{decimals}").parse::<f64>() == Ok(value)` (number.rs:108): std's
  dec2flt is correctly rounded and pure software (fast path = exact IEEE mul/div), same on wasm32 and x86_64. Off a
  power of two the rounding interval is symmetric (both midpoints in or out together by mantissa parity), so the
  read-back can only change the answer at a power of two; at a power of two in the tie range (2^-25..2^-2) it now
  keeps std's upper text when the lower one is the double below. Even ending in 0 (lower=..9) can only be invalid
  (else a shorter text exists): now refused by the read-back too (before: would have been written). decimals >= 0
  whenever it reaches the format (places >= 1). Sweep re-run against the fixed crate: running (scratch2/).
- [SA-1 string() reasoning] pre-scan (json.rs:377-387) and read loop (:389-414) share the escape structure (a
  backslash takes the next byte): on a read that succeeds both stop at the same quote, and length == decoded length
  exactly. On a read that fails (forbidden byte, refused escape, unterminated) the pre-scan over-counts up to the
  next unescaped quote or the end of input: one transient allocation <= the remaining input, dropped with the error,
  and the parse ends at that error, so at most once per parse. Work: successful strings scanned twice (2 passes over
  their bytes), plus at most one failing scan to the end: <= 2x input, linear. No overflow (length <= slice length;
  saturating_add; with_capacity <= isize::MAX since the input exists).
