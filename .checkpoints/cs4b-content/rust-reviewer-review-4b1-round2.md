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
- [RR-1 CLOSED, by run] round 1's sweep (rust-reviewer-scratch-4b1.rs, unchanged) rebuilt with rustc 1.99.1
  opt-level 3, overflow checks on, against d36e416's crates/content/src (.gate/rust-reviewer-4b1-target/scratch2/):
  31,750,551 doubles written; round trip parse_number(number_text(x)) == x: 0 failures (round 1: +-2^-24 failed);
  tie window 9,913,103 checked against ECMAScript's rule worked out apart from the crate, 318,235 real ties: 0
  disagreements. Node 22 V8 String(x) over all 31,750,551 (rust-reviewer-scratch-4b1.mjs): differ 0 (round 1: 2).
  Unit rows by bits for +-2^-24, 2^-25 and both neighbours of 2^-24 are in number.rs:299-301.
- [RR-1 doc] even_of_a_tie's doc (number.rs:74-81) and comment (:104-107) are true as worded: note 2 cited for the
  even rule; "both neighbours must be texts of this very double" then "the even one is taken only if it reads back"
  (the code checks the even one only, which is the one that matters: the other is std's, which always reads back).
  The module doc (number.rs:4-5) still says "the one text ECMAScript's Number::toString gives": loose, since step 5
  leaves the last digit open (the function's own doc says so); Low wording, can wait.
- [RR-1 corpus, Low] corpus-texts.mjs powersOfTwo() writes positive powers only (`value > 0`): the corpus has
  3e70000000000000 and neighbours (lines 17189-17191) and no be70000000000000. The brief's "every power of two with
  its two neighbours, both signs" is not true of the corpus. The writer is sign-symmetric by construction (abs, then
  a "-"), number.rs:299 has -2^-24 by bits, my sweep has both signs: wording, can wait.
- [SA-3 is_id] json.rs:77-93: con/prn/aux/nul + com0-9/lpt0-9, lower case (ids are lower case only), whole id only
  (an id is the stem: `<folder>/<id>.json`, so "con.json" is the case that matters). Superscript ports (com¹...) and
  conin$/conout$/clock$ cannot be ids (ASCII [a-z0-9_-] only). Right. Note: is_id also judges ids that are no file
  names (shape, spawn, region ids, read.rs:172): a shape "aux" is refused with "no device name of Windows": a small
  over-reach, harmless before any content exists; can wait / design call.
- [RR-4 files.rs] read_plain (files.rs:107-125): name check, path lstat checks, ONE open, fstat on the handle
  (is_file), read through take(MAX+1), cap judged on the bytes read: type and cap now tied to the bytes (my round-1
  fix, as asked). Residual: the "no link" check is still on the path (lstat at :110, then open follows a link swapped
  in between); closing it needs O_NOFOLLOW or a dev/ino compare (MetadataExt). Anyone who can race it already runs
  code in that test run: not a finding beyond a note. write_plain: same name check (:133). recordings.rs:228-234 scan
  counts `File::open(` and `fs::read(` only: a new `fs::read_to_string(`/`OpenOptions` in files.rs would pass it
  (the doc says it is a text scan); Low, can wait.
- [RR-3 dependency_rules.rs] CLOSED: a key line before the first table is pushed as a table (:23-25), so the tables
  assert fails; build.rs asserted absent by lstat (:67-72). Remaining ways in: none I can find (a dotted key inside
  [package] is package.x, not a dependency; [[bin]] reads as "bin"; spaced headers fail closed).
- [runs] cargo fmt --all -- --check: clean. cargo clippy --workspace --all-targets --locked --offline -- -D warnings
  (own target): clean. Same, --target wasm32-unknown-unknown -p content -p devtools -p sim --lib: clean.
- [rules_scan probes, by run] a COPY of crates/sim/tests/rules_scan.rs (.gate/rust-reviewer-4b1-scratch/scan_probe.rs)
  with 22 probes through content_breaches (rustc --test, CARGO_MANIFEST_DIR set; its own 8 tests pass too). MISSED
  (production panics not named): `#[cfg(test)]` on a match arm or a struct-expression field whose value is no block
  swallows the rest of the enclosing braces (in_tests ends at the first `;`/`}` at depth 0); path calls
  `Option::unwrap(y)`, `Result::expect(r, ..)`, `.map(Option::unwrap)`; `unwrap_err`, `expect_err`; `assert!`,
  `assert_eq!`, `debug_assert!`. WRONGLY FLAGGED (fails closed): a test item whose signature holds `[T; N]` (the `;`
  ends the region: depth counts braces only), `#[cfg(all(test, ..))]`. Right: r#unwrap, cfg(not(test)),
  cfg_attr(test, ..), a statement, a use list, an impl block, a where clause, a brace in an attribute string.
- [guard vs scan, by run] clippy-driver 1.99.1 on a scratch file with content's deny list
  (.gate/rust-reviewer-4b1-scratch/guard_probe.rs): the guard DOES catch `Option::unwrap(y)`, `Result::expect(r, ..)`,
  `r.unwrap_err()`, `r.expect_err(..)`; it does not catch `.map(Option::unwrap)`, `assert!`, `assert_eq!`,
  `debug_assert!`, `unimplemented!` (not in the list). So the scan's doc (rules_scan.rs:551-554, "holds the part of
  it that text can show") overclaims: four text-visible forms the guard holds pass the scan. No assert!/path-form
  unwrap in content's non-test code today (awk over each file up to its first #[cfg(test)]), so adding them is free.
  CI (toolchain.yml:158, :162) runs `cargo test --workspace` (rules_scan, content units, debug fuzz) and the release
  fuzz (-p devtools), no clippy: RR-2's CI step stays a recorded follow-up.
- [runs] cargo test -p content --locked (debug): 35 passed. cargo test -p sim --test rules_scan --locked: 8 passed.
- [SA-1 string(), by run] rust-reviewer-scratch-4b1-r2.rs (counting allocator, opt-level 3) against d36e416's
  content: a taken string's capacity equals its length (1,000,000; 524,289; 5 with two escapes): no slack. A string
  refused early (a forbidden byte or a refused escape first, then 1 MB, then the quote): peak 1,000,011 bytes held
  for a 1,000,010-byte input: the pre-scan counted the whole megabyte, the read stopped at its first byte. So the two
  disagree on a refused string: one transient allocation <= the remaining input (never more than 1x the input, once
  per parse), where the old code held almost nothing there. Bounded, so no finding above Low: the pre-scan could stop
  where the read would refuse (a byte outside 0x20-0x7e, an escape other than \" and \n) and then agree exactly.
- [runs] cargo test -p devtools --locked (debug, own target, 4 test threads): every binary passed (content_fuzz 3 in
  11.5 s, content_proofs 5 in 24.1 s, dependency_rules 2, recordings 5, time_unit 4, aim_reach 1, two_* 1+1, lib 15).
  Release fuzz (`cargo test --release -p devtools --test content_fuzz -- --nocapture --test-threads 1`): 1,000,000
  inputs, no panic, 35.7 s; histogram 1867/0/2131/10/1508/3306/384/601/190 (all above SHARES). Limit cases: 8 x 1 MB
  bundle 8,006,189 B in, 9,011,083 B held (1.13x; round 1 9,443,515, 1.18x), 68.3 ms; fifteen texts 7,865,384 B in,
  7,870,046 B held (1.0006x), 56.5 ms; 16,383 keys 4.7 ms / 1.54 MB; one key 16,383 times 2.6 ms; 100,000 keys 2.6 ms.
- [speed A/B, by run] the pre-scan's cost: round 1's `speed` scratch built twice at opt-level 3 (rustc 1.99.1,
  x86_64, this 4-core machine), once against d36e416's content and once against a COPY whose string() has
  `String::new()` instead of the pre-scan (.gate/rust-reviewer-4b1-scratch/content-plain-string), 3 alternating
  rounds x 7 runs, medians: strings bundle 8.0 MB 67.6/71.5/67.8 ms vs 57.3/51.4/48.7; numbers bundle 6.3 MB
  60.1/58.0/55.9 vs 47.4/48.5/48.3; keys bundle 5.1 MB 40.5/39.7/35.4 vs 31.1/31.4/30.3; one 1 MB string file
  6.4/6.4/6.5 vs 5.6/5.2/5.4; numbers and keys files: equal within noise. So +1.2 to 2.1 ns per string byte, +20-30%
  at the bundle stage, linear: a once-per-match load, no finding (numbers for the report).
- [content_proofs, read] LIMIT_CASES/limit_case(at) (content_proofs.rs:1167-1241): every call draws the same
  `distinct(&mut rng, 8)` first, so case 3's files are as before; each case is made, judged and dropped before the
  next (limits_check's find_map closure; content_fuzz's loop); limit_case(len) is None is asserted. 15 x 524,289
  spaces = 2^19 + 1 each, under the 8 MiB cap, so doubling would hold 2^20 a text (15.7 MB > bytes + TREE_BYTES =
  12.06 MB): the case can fail. random_id's device rule (:241-252) is written apart from is_id and equals it on
  [a-z0-9_-]; `push('x')` makes a valid non-device id (<= 5 bytes). echo() (:177-188) now escapes `"` as Echo::of
  does. The fold threshold 262,144 (:1111): every input above it is spaces or a bundle of one megabyte of spaces
  (classes "a file past the byte cap" :743-750, "a bundle past a byte cap" :1029-1043): comment true.
- [wasm runs] module .gate/rules-module built 16:32:55, no crate file newer (find -newer): it is d36e416's code.
  `node node_modules/vitest/vitest.mjs run --root packages/sim-worker --dir packages/sim-worker/boundary
  --silent=false --reporter=verbose content`: 7 passed; verdicts release 5,443 ms, dev 43,049 ms (digests equal the
  native ones); limits release 214 ms, dev 3,074 ms (asserted under 5 s / 60 s); module memory 49,479,680 B release,
  49,545,216 B dev (47.2 MiB; ceiling 64 MiB).
- [corpus ties, MEDIUM, by run] rust-reviewer-scratch-4b1-r2-ties.rs / -ties2.rs (a line is a real tie when its
  canonical significant digits differ from std's shortest digits, i.e. the even rule changed them): ef2edec's corpus
  64 real ties (58 in its 1,000-line ties section, at 2/3/4 binary places: 40/13/5); d36e416's corpus 44 (36 in the
  2,000-line section, at 2/3/4 places: 25/10/1; places 5 to 25: 0 of ~1,400 lines), plus 2^-25 among the powers.
  Why: ties() draws whole in [2^52, 2^53) and divides by 2^(1..25), so every value lies in [2^27, 2^52), where a
  17-digit text leaves room for a tie only at about 2 to 4 binary places; widening `% 8` to `% 25` spread the draws
  thin and cut the tie coverage by a third. corpus-texts.mjs:75-76's doc and the card's step 27 ("2,000 ties of 1 to
  25 binary places") do not hold. The result is still proven (unit rows by bits, the powers of two, the reviewer's
  sweep: 318,235 ties over q 1..25), but the committed cross-engine tie evidence shrank. Fix: draw x = M * 2^-q with
  M odd below 10^19 / 5^q, q 1..25 (the sweep's window), or keep `% 8` and add such a family; state measured counts.
- [verdict per round-1 finding] RR-1 closed (sweep above). RR-2 open as recorded follow-up (CI step, CI Builder;
  scan holds part, gaps listed). RR-3 closed. RR-4 closed. RR-5 + nits closed (SAFETY says both; "three heavy ones";
  fold 262,144 with a true comment; "the slowest by the clock"; saturating_mul; even_of_a_tie cites note 2), the
  module doc number.rs:4-5 still loose (Low).
- [new, ranked] Medium corpus-texts.mjs:75-81 ties (coverage 64 -> 44 real ties; 0 at 5-25 places; card/doc claim
  does not hold) + :96 positives only ("both signs" untrue). Low rules_scan.rs:506-559 (misses guard-held text forms,
  match-arm/field swallow, assert! held by nothing; overclaiming doc). Low json.rs:377-388 (refused string: transient
  alloc <= rest of input). Low number.rs:4-5 wording. Low recordings.rs:228-234 scan blind to read_to_string/OpenOptions.
  Notes: read_plain link check still on the path (race needs code execution anyway); is_id also refuses device names
  for shape/spawn/region ids.
## Not checked
Chromium/Firefox runs (main session's); the plants script (edits source; not run); AArch64; plain-file.ts and the
TypeScript readers (security-auditor's); corpus-texts.mjs not run (read, and its output counted); Windows behaviour of
device names with endings (not tested on Windows).
DONE
