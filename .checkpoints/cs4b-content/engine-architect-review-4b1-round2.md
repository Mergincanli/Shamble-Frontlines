# engine-architect, review of change-set 4b-1 "Content", round 2

Head reviewed: code head `d36e416` on `cs/4b-content` (worktree `.gate/lane1-takeover`); round 1 read `ef2edec`.
Brief: `.checkpoints/cs4b-content/review-brief-4b1-round2.md` (+ round 1's "Rules for you"). Reads ONLY
`git diff ef2edec d36e416 -- crates packages`. Notes only; no source edits; target folder
`.gate/engine-architect-4b1-target`; scratch copies only under `.gate/engine-architect-4b1-scratch/`.

## Plan
1. Confirm the tree: git status, git log ef2edec..HEAD, diff --stat of crates packages.
2. EA-1: read the new `the_content_crate_keeps_its_rules` in `crates/sim/tests/rules_scan.rs`; judge placement (sim's
   tests scanning `../content/src` by path) and whether it holds what sheet step 2 planned (reread-sheet.md:156,
   :75-76, test-list.md:50-51). Not repeat rust-reviewer's round-2 gaps.
3. EA-2 / EA-5: `crates/content/src/map.rs` diff: `Form::Ramp { rises_toward: Toward }`, `group` dropped. Check the
   file format is unchanged (reader keys, writer if any, proofs/vectors), and how ts-rs 12.0.1 would render it at
   step 9 (registry source in ~/.cargo if present, offline).
4. EA-3: `crates/devtools/tests/dependency_rules.rs` comment on step 9's dev-dependency; check the rule's code.
5. EA-4, EA-7, EA-8: wording in `crates/sim/src/time.rs`, `crates/content/src/time.rs`, `crates/content/src/error.rs`.
6. EA-6: what the step-8 facing line should state (map.rs's doc of `at` as pinned now).
7. New: `LIMIT_CASES` / `limit_case(at)` in `crates/devtools/src/content_proofs.rs` and `random_id`'s device-name
   rule: boundary / layering (wasm.rs exports, who calls them, duplication of is_id's rule).
8. Runs (own target, --locked, offline): content + devtools lib + dependency_rules + time_unit + sim rules_scan;
   clippy if cheap; players' Wasm unchanged if cheap.

## Progress
- [ ] 1 tree
- [ ] 2 EA-1
- [ ] 3 EA-2 / EA-5
- [ ] 4 EA-3
- [ ] 5 EA-4 / EA-7 / EA-8
- [ ] 6 EA-6
- [ ] 7 new code
- [ ] 8 runs

## Findings (as made; severity | where | what | evidence | fix)
- [x] 1 tree: HEAD 995bca9; `git diff --stat d36e416 HEAD -- crates packages Cargo.toml Cargo.lock rust-toolchain.toml
  .github` empty (commits on top are docs and notes); fix pass = 19 files (git diff --stat ef2edec d36e416).
- [runs, own target, --locked --offline] `cargo test -p sim --test rules_scan`: 8 passed (the_content_crate_keeps_its_rules,
  9 bites, 6 controls among them). `cargo test -p content -p devtools --lib --test dependency_rules --test time_unit`:
  content 35, devtools lib 15, dependency_rules 2, time_unit 4: all passed.
- CI: toolchain.yml:158 `cargo test --workspace --locked` and :162 `--release -p sim ...` run rules_scan, so the content
  scan runs in CI although it lives in sim's tests; `cargo test -p content` alone does not run it.
- [x] 3 EA-2 CLOSED, by run: scratch copy `.gate/engine-architect-4b1-scratch/ts` (workspace copied; probe
  `crates/protocol/tests/map_ts_probe.rs`, protocol already has ts-rs 12.0.1 as dev-dependency, lock unchanged; map.rs's
  types copied by hand with attributes only: `#[ts(tag = "type", rename_all = "lowercase")]` on Form, `#[ts(flatten)]`
  on Shape.form, per-variant renames on Toward, rename_all lowercase on Ground). Printed:
  `type Shape = { id: string, min: [number, number, number], max: [number, number, number], } & ({ "type": "box" } | { "type": "ramp", rises_toward: Toward, });`
  `type Toward = "+x" | "-x" | "+z" | "-z";` `type Ground = "street" | "grass";` Region/Spawn/Bounds plain tuples.
  Round 1's shape renders `{ "type": "ramp" } & Toward` (object & string union, no rises_toward key). So step 9 needs
  attributes only, no shape change. Note: MapDocument has no `format`/`version` fields, so the page's type has no header
  keys (harmless: the loader checks them; the page reads after the Worker's ready).
- [x] EA-5 CLOSED: `group` gone from Shape, reader (map.rs:168), the test TEXT, the generator's shape_text
  (content_proofs.rs:365-366) and Map::shapes; grep `group` over crates/content, crates/devtools: 0. File format: the
  Ramp change leaves it unchanged (same keys `type`, `rises_toward`, same match arms); dropping `group` does change
  version 1 (a shape with `group` is now UnknownKey; without it no longer MissingKey) and that is the intended fix: no
  content file exists and 4b-1 is unmerged, so version 1 is first fixed at merge; no version step owed. Verdict digests
  that moved (content-verdicts.txt): the map and bundle classes, "an id that is no id" (devices), "more values than a
  document holds" and the fuzz's first inputs; movement-only classes unchanged: consistent with group + SA-3 + the fold
  threshold (not traced case by case: test-engineer's).
- [x] 2 EA-1 CLOSED (placement and plan; RR's round-2 gaps in the panic rule not repeated). Plan: sheet step 2
  (reread-sheet.md:156 "the scan (crates/sim/tests/rules_scan.rs): its hashed-map, clock and unwrap rules also read
  crates/content/src") and test-list.md:50-51 name this very file; built: every rule of breaches_of but floats
  (hashed, clocks, per-target, #[path]/include!) + panic calls outside #[cfg(test)], 9 bites, 6 controls, a sanity
  assert on four file names, run in CI (toolchain.yml:158, :162). Placement right: the test of the upper crate reads
  the lower crate's text, the same way sim -> content will point at step 6; no Cargo edge, so content's
  depends-on-nothing rule stands; content's own tests could not share the tokenizer without a dev-dependency (refused)
  and devtools would need the tokenizer moved out of a test file (outside 4b-1). Coupling (path ../content/src, four
  file names) fails loudly by its own assert. Interim, like the sim half: its end is CI clippy for the panic half;
  the hashed/clock half for content must pass to G2's lints when they land (PM-0004-23 names "the sim crate" only):
  a note for 4b-2's step 6/7 plan.
- F1 Low | crates/sim/tests/rules_scan.rs:1-17 | the module doc still says the scan reads `crates/sim/src` and gives
  only the sim's interim end; the second scope and its end ("until a lint step does") are only at :504-507. Fix: one
  sentence in the module doc naming crates/content/src, its rule set (all but floats, plus panics outside cfg(test))
  and its end. Can wait.
- F2 Low | crates/sim/tests/rules_scan.rs:601-614 | "the guard ... is still the crate's first words" is held by
  `lib.contains(name)`: the names, not the deny. By run: scratch copy with `deny(` -> `allow(` in content/src/lib.rs:15
  (.gate/engine-architect-4b1-scratch/ts), `cargo test -p sim --test rules_scan the_content_crate_keeps_its_rules`:
  ok (1 passed). Since CI runs no clippy, the guard's only future effect is the CI clippy step RR-2 asks for, which an
  allow would void silently. Fix: assert the token run `cfg_attr ( not ( test ) , deny (` and each name inside it
  (the file's tokens() already exist), or let the CI clippy step's own planted unwrap prove it. Can wait (with RR-2's
  follow-up).
- [x] 4 EA-3 CLOSED: dependency_rules.rs:8-11 says the rule is stricter than "no normal dependency", that 4b-2's type
  step adds exactly one dev-dependency (the type writer; the orphan rule's reason, rightly), and that the change edits
  the rule in the same commit after security-auditor's answer. Nit only: it could name the line
  (`ts-rs = { workspace = true }`, the workspace's `=12.0.1`, default features off, Cargo.toml:13); a dev-dependency
  stays out of sim's tree, which is why it is admissible. The dotted-key and build.rs checks are RR's (closed there).
- [x] 5 EA-4 CLOSED: sim/src/time.rs:1-5 names the replacement (`pub use content::time::UNITS_PER_SECOND` at 4b-2,
  with the collision library = sheet step 6, whose manifest batch gives sim its `content` line, reread-sheet.md:164),
  the tie's first check (time_unit.rs:22) going and the const assertion staying. EA-7 CLOSED: content/time.rs:6-8
  ("a content file keeps a duration ... in seconds; whoever starts a match converts it here, once ... Nothing converts
  while a match runs") now agrees with movement.rs:5-6 ("once, when a match starts"); no stale "when its file is read"
  left (grep). EA-8 CLOSED: error.rs:207-211 "a place of 0 and 0 is no place: with no field the whole document, with a
  field that field" matches MissingKey, IdNotFileName and the map rules (map.rs:217-219).
- ADR 0005 CD-2 (adr 0005:285) says "never `flatten` where unknown keys must fail": that is serde's deserializer
  flatten; the step-9 `#[ts(flatten)]` is ts-rs's, type text only, and this crate reads with its own reader, so no
  clash; worth one line in 4b-2's step 9 plan so nobody reads it as a breach.
- [x] 6 EA-6: `at` pinned (map.rs:83 "the point on the ground between the feet, the soldier's own origin": fine; no
  sim position exists yet to contradict it, grep crates/sim/src). The deferral line IS already in the tree at d36e416:
  map.rs:56-57 "Which way 0 points and which way it turns is pinned in the change that writes the first map file
  (4b-2's data step), before any spawn is written" (the brief says it is not yet: it is; do not add a second one).
  What step 8's pinning line should state, in the map's own axes (y up, ground x/z): (a) the zero as an axis: 0 faces
  +z; (b) the sense: degrees grow from +z toward +x (counter-clockwise seen from above, the right-hand rule about +y),
  so the facing is (sin t, 0, cos t) and 90 faces +x. That is the legacy game's own convention, every soldier turned by
  `rotation.y = Math.atan2(dir.x, dir.z)` (legacy/shamble-game.js:871, :2758, :2916, :2920, :2924), i.e. three.js's
  rotation.y with no offset; (c) what faces: the soldier's body forward at its first tick; (d) how it reaches the
  simulation with no std transcendental (ADR 0004 part 7; PM-0004-23 plants `deg.to_radians()` and `x.sin()` as
  breaches): either the page alone turns it (presentation) until 4c needs it, or 4c converts once at match start
  through libm on step 7's allow-list with PM-0004-22's vectors, or the file holds facing to a grid (whole multiples
  of 45 or 90, exact by table/sqrt): a grid is free only before the first file, tightening later is a version step
  (CD-4); (e) 4c's test: a spawn at 90 faces +x.
- [x] 7 new code. LIMIT_CASES/limit_case(at) (content_proofs.rs:1169-1234): no boundary change: the Wasm export
  `content_limits()` still calls `limits_check()` (wasm.rs:92-93), one call, a string out; devtools-only, nothing shipped
  imports it. The reason is sound: Wasm memory only grows, so building all cases first measures the list, one at a time
  measures max(case) + loader (as the native counting allocator does per case). random_id's device rule
  (content_proofs.rs:238-253): an oracle kept apart from the code it judges is right; drift fails loudly both ways (a
  name the loader adds -> a "valid" case refused; a name it drops -> "an id that is no id" gets no refusal); no layering
  concern.
- F3 Low | content_proofs.rs:1169-1234, tests/content_fuzz.rs:103-127 | a case and its name are tied only by position
  across two public items: guarded for a missing case (limits_check :1238-1240) and an extra case (content_fuzz
  asserts limit_case(len) is None, native only), but a reordered or renamed list mislabels the printed per-case bytes,
  peak and time (content_fuzz.rs:116, the numbers the report quotes) with nothing failing. Fix: `limit_case(at) ->
  Option<(&'static str, Case)>` with the name in its own match arm (the list becomes a count). Can wait.
- F4 Low | crates/sim/tests/rules_scan.rs:551-554 (content_breaches -> breaches_of's `covered`, :272, :297-303) | the
  sheet's section C says "no hashed map anywhere in the crate (the scan's rule is extended to it)"
  (reread-sheet.md:75-76), but content inherits the sim's part 9 escape: by run, a copy of the scan
  (.gate/engine-architect-4b1-scratch/ts, probe test) gives [] for `#[allow(clippy::disallowed_types, reason = "lookup
  only")] type Seen = std::collections::HashSet<String>;` and for a struct with a HashMap field under the same
  attribute, and one finding without the attribute. No clippy.toml exists (ls), so that allow names an unconfigured
  lint: for content only the scan holds the rule, and its message invites the escape. Today content has 0 hashed maps
  and 0 escapes (grep). Fix: content_breaches ignores the escape (a parameter of breaches_of) and one bite with the
  escape. Can wait.
- [runs] players' Wasm, raw: `cargo build --release --target wasm32-unknown-unknown -p sim-wasm` at d36e416: 79,068
  bytes, sha256 a56b0e8b603f7a5de92fa98820d0b340681495d64970785829a7f319555bcebe, the same as round 1's at ef2edec and
  without the time module: the fix pass changes no byte of it (raw cargo output, not the shipped file).
- F5 Low | crates/sim/tests/rules_scan.rs:517-545 (in_tests) with sheet step 9 (reread-sheet.md:167: "ts_export.rs
  (test-only) with content's dev-dependency ts-rs") | the scan judges file by file and knows test code only by a
  `#[cfg(test)]` item inside the same file; protocol's pattern puts `#[cfg(test)]` on the `mod ts_export;` line of
  lib.rs (protocol/src/lib.rs:22-23), and an inner `#![cfg(test)]` is not matched by MARK either. By run (scratch
  probe): protocol's ts_export.rs through content_breaches gives 20 findings (first `ts_export.rs:43: panic`), and 20
  with `#![cfg(test)]` on top. So step 9's planned file fails the scan on arrival (fails closed, no hole; clippy's
  guard has no such problem: cfg_attr(not(test))). Fix: in_tests treats a file that begins with `#![cfg(test)]` as
  test code (and step 9 writes that line), or the scan reads the parent's `#[cfg(test)] mod x;`. Can wait: a note for
  4b-2's step 9 plan.

## Progress (final)
- [x] 1 tree  - [x] 2 EA-1  - [x] 3 EA-2 / EA-5  - [x] 4 EA-3  - [x] 5 EA-4 / EA-7 / EA-8  - [x] 6 EA-6
- [x] 7 new code  - [x] 8 runs (rules_scan 8, content 35, devtools lib 15, dependency_rules 2, time_unit 4; ts-rs probe;
  two scan probes and one planted guard in the scratch copy only; players' Wasm hash)

## Verdicts per round-1 finding
EA-1 closed · EA-2 closed (by run) · EA-3 closed (nit: name the line) · EA-4 closed · EA-5 closed · EA-6 open by
design (STATUS follow-up; the deferral sentence is already at map.rs:56-57; content of step 8's line above) · EA-7
closed · EA-8 closed.

## New in round 2 (all Low, all can wait)
F1 module doc (rules_scan.rs:1-17) · F2 guard names not the deny (:601-614, by run) · F3 limit case and name tied by
position (content_proofs.rs:1169-1234) · F4 content inherits part 9's escape against "no hashed map anywhere" (by
run) · F5 file-level test modules unseen by in_tests, collides with step 9's ts_export.rs (by run).

## Not checked
rust-reviewer's scan gaps (not repeated); json.rs string(), number.rs ties, files.rs, corpus-texts.mjs (RR, SA); the
Node and browser runs (not run here); the plants script (not run: it edits source); the shipped file after
wasm-bindgen (raw cargo output only); the verdict digests' moves case by case (test-engineer).
Scratch: .gate/engine-architect-4b1-scratch/ts (a copy of the workspace; probes in crates/protocol/tests/map_ts_probe.rs
and appended to crates/sim/tests/rules_scan.rs, copy only).

## Status: DONE (2026-11-26). Round 2: 7 of 8 closed, EA-6 a follow-up by design; 5 new Lows, none blocks.
