# engine-architect, round 2 review of cs1-types (foundation step 2.4, "Types")

Head reviewed: code of commit 65a7b13 (branch claude/wonderful-hamilton-jdmlnf; checkpoint head 6c3a24a); round 1
reviewed 82142fa; base main 22c6afa. Memory: no engine-architect memory file (.claude/agent-memory-local/ holds
security-auditor only). Round 1 notes: engine-architect-review1.md (REVISE).

## Plan
1. Read fix-pass-1.md, checkpoint step 9, `git diff 82142fa..65a7b13 -- . ':!.checkpoints'`.
2. Check 1: repository_root guard in layout_ts.rs and ts_export.rs; cargo clean record; plant.sh own target; runs20
   freshness rule. Judge the guard for later runners (cs2 Wasm tests, gate v2 regeneration, IDE, nextest).
3. Check 2: generated/block/layout.ts, index.ts, layout-check.ts; hash coverage vs the file's values; destructuring.
4. Check 3: public symbols + tests at 65a7b13 vs plan-draft s4 + plan 5(g); the four deviations.
5. Check 4: report sentences (PM-0002-15/-16/-17/-19/-63; O2; OQ4/OQ3; follow-ups cs2, cs4, 2.3, drafts).
6. Check 5: anything the fix pass broke at the boundary (cargo tree, tsc by path, tests on own target).
7. Answer at most 45 lines via SubagentHandback.

## Findings (a line each, as found)
- (read) fix-pass-1.md, checkpoint step 9, the diff 82142fa..65a7b13 (16 files, +371 -150; Cargo.toml/.lock unchanged).
  Working tree code == 65a7b13 (git diff --quiet 65a7b13 -- crates packages Cargo.*). Later commits: checkpoints and
  d34cfc5 (DECISIONS, out of scope).
- (verified) a third implementation of the hash stream (scratchpad fnv_check.py, python -I): real table 0x2df78c30
  (= layout.ts:4), BASE 0x9adbecc7, empty 0x811c9dc5, TAG_PAIR 0x4265359c / 0xbd51cd2f, and the pair's streams are
  equal without the tag bytes, different with them: the known answers in layout.rs:273-279 hold.
- (read) plant-results.txt: 30 runs, 0 not as wanted, worktree at 65a7b13 with CARGO_TARGET_DIR plant-target
  (plant.sh:15); 6a/6b ran the client_endpoint binary with a foreign/absent CARGO_MANIFEST_DIR: guard message, exit 101.
  Same artifact name client_endpoint-bdbcb2c4df73f18c as round 1's stale sa-plant binary: the artifact name does not
  depend on the tree's path, which is why a shared target reused it. Protocol's guard: no planted run (same text).
- (read) runs20-results.txt: commit 65a7b13, 1 worktree, 20 runs, all 9 files newer than each run's start, one set
  07cd492f...4bc8, git: no difference. The record now holds (the guard would also fail a foreign binary).
- (verified) my own build, main folder, CARGO_TARGET_DIR=.gate/cs1-types/ea2-target (not shared), cargo test
  --workspace --locked --offline at 13:47 UTC: client-endpoint 5 passed, protocol 2 passed, the rest as before.
  Artifact again client_endpoint-bdbcb2c4df73f18c (third target folder, same name). Protocol's guard tried by me:
  its binary with CARGO_MANIFEST_DIR=/somewhere/else and with it unset: both exit 101 with the guard's message (left
  None / right Some(".../crates/protocol")); no scratch folder made (the guard runs first).
- (verified) main target after the clean: 0 files older than 13:33 (oldest 13:37:24, CACHEDIR.TAG); its
  client_endpoint and protocol test binaries carry only the main folder's paths (strings). cargo clean: confirmed.
- CHECK 1 CLOSED (High 1). Guard design: right. It fails closed with the cause named, in test code only, before any
  file is read or written; my round-1 hardening (use the run-time dir) would still have run stale code; the guard
  stops both. Later runners: cargo test (CI, gate v2's regeneration, rust-analyzer's runnables) pass it; cargo-nextest
  sets CARGO_MANIFEST_DIR at run time "matching the behavior of cargo test" (nexte.st/docs/configuration/env-vars/),
  so it passes, except --workspace-remap ("set to the remapped manifest directory": the guard stops it, by design);
  cs2's Wasm tests: the build plan's are Vitest loading built sim-wasm (BP:939) and wasm-bindgen-test (A2:494), whose
  guide names #[wasm_bindgen_test] for wasm32-unknown-unknown and #[test] for other targets
  (wasm-bindgen.github.io/wasm-bindgen/wasm-bindgen-test/usage.html, no version); on that target "std::fs always
  return errors" (doc.rust-lang.org/rustc/platform-support/wasm32-unknown-unknown.html), so the export tests are
  native-only whatever the guard does; clippy --target wasm32 --all-targets already compiles them (step 9: exit 0).
  A debugger that starts the binary itself must set CARGO_MANIFEST_DIR (the message says to run through cargo test).
- CHECK 2. layout.ts (generated/block/layout.ts:4-15): LAYOUT_HASH, then BLOCK_LAYOUT = { header: { stride: 8,
  fields: { tick {0,u32}, layout_hash {4,u32} } } } as const. Values it states: section names and order, strides,
  field names and order, offsets, type names; the hash stream (layout.rs:86-115) covers exactly these (order by
  position, count implied), plus structure bytes (tags, 0x00), and nothing the file leaves out (align gone). Names are
  [a-z][a-z0-9_]* so never integer-like (key order kept) and never __proto__. Destructuring: layout.rs:92-96 and :103,
  layout_ts.rs:24-30 and :33, no `..`: what I meant; ElementType's matches (layout.rs:18-32) have no wildcard either.
  Shape: right for the page (literal types, property access fails tsc on a missing name; no hand line per section in
  index.ts:12); the writer (cs2) reads the Rust const, not this file. checkLayoutHash(ArrayBuffer) fits B2 (A2:287-288,
  a pooled ArrayBuffer per tick starting at 0); B3 would need an offset (later, measured alternative).
- (Low, cs4 follow-up) index.test.ts:8 pins Object.keys(BLOCK_LAYOUT) to ["header"]: a hand edit per section is back,
  in the test; what the page needs is "header first". Change when the first entity section enters.
- (agree, not taken) TE's Rust literal pin of LAYOUT_HASH: the value is pinned once, in the committed layout.ts, by a
  byte comparison; the function by known answers from another implementation (I re-derived them). A literal adds a
  second hand edit and catches nothing more. EA 6 deferral with layout-check.ts:20-21: accepted (unreachable today).
- (verified) run 4's new hash 0x5d0e9efb (plant-results.txt:133) = my script for stride 12 + soldier_count u32 at 8.
- CHECK 3 (symbols at 65a7b13 vs draft s4 + plan 5(g)): nothing missing but `align` (deviation 4, on purpose).
  Added public: ElementType::name(), ::size(); derives on ElementType/Field/Section. Added private/test: SECTION_TAG,
  FIELD_TAG (dev. 1), const _ non-zero assert, fnv1a + FNV consts, repository_root x2, entries_of, helpers/fixtures
  (field, section, name_is_plain, broken_rule, GOOD, BAD 11, FIRST, SECOND, BASE, CHANGED 12, TAG_PAIR), Vitest "a
  block of zero bytes fails". Renamed: TS header -> BLOCK_LAYOUT (dev. 4); Vitest "the package exports the eight
  message types" -> "values of the eight message types build (...)"; write_generated(folder) -> (root) (private).
  Removed as planned: stub_builds (client-endpoint), PACKAGE; package.json description changed. Rust tests 5 + 2,
  Vitest 7. (Low, report) the four deviations leave out the public name()/size(): one line "added" for the verifier.
- CHECK 5: nothing broken. cargo tree --locked --offline: client-endpoint no edge of any kind; protocol no normal or
  build edge; ts-rs only as protocol's dev-dependency (unchanged; Cargo.toml/.lock not in the diff). tsc 6.0.3 by path:
  both scratch configs exit 0 (shipped lists 9 generated + layout-check.ts + index.ts). Vitest 5.1.0 by path: 7 passed.
  clippy --workspace --all-targets --target wasm32-unknown-unknown -D warnings: clean (my target). CI: toolchain 170
  (6c3a24a) and 171 (ac08108) success, gate 873 (ac08108) success; ac08108's crates/packages/Cargo/.github/tools ==
  65a7b13 (empty diff).
- (Low, the drafts; sharpens round 1's F-NOREADER) ADR 0002 names a TypeScript reader of the message types only in
  its test plan: "round-trips every message type between the Rust encoders and the TypeScript decoders" (A2:495,
  A2:1533-1534), which H2 (A2:374-375: the page "sees only the block") and the page's own input post (A2:318-323)
  leave without a place in shipped code. Next revision: name the reader, or narrow D3 (A2:350-352) and that row.
- (info) ts-rs output now has 34 trailing-space lines in 8 files (round 1: 27 in 7); ours 0: F-FORMAT stands.
- (Low, cs4 follow-up) the folder rules (repository_root, write_generated's link refusal) are copied in two export
  tests (layout_ts.rs:44-84, ts_export.rs:17-104), and the stray-file rule for generated/ lives only in
  client-endpoint's (layout_ts.rs:124-135): content's export test (third generator) should share them, not copy again.
- (Low, 2.3 follow-up, same root as High 1) the gate v2's G4 and any worktree run need a target folder of their own,
  or the export tests stop at the guard (correctly).
- Round-1 closure: High 1 closed; Medium 2 open until the report carries the PM-0002-15 sentence (no code); Lows 3, 4
  (residue: index.test.ts:8), 5, 10 closed; 6 deferred to cs4 with layout-check.ts:20-21 (accepted); 7, 8, 9, 11 are
  report or follow-up lines (sentences in my answer).
- Verdict: APPROVE (no finding at Medium or above on 65a7b13's code; Medium 2 is report wording; all new findings Low:
  report lines or follow-ups, none triggers a push).
- Not checked: the planted runs and the 20 runs themselves (read their logs; I ran my own build, protocol's guard, tsc,
  Vitest, wasm32 clippy, cargo tree); other reviewers' round-2 notes; rust-analyzer, nextest and wasm-bindgen-test in
  practice (docs only); Windows/macOS; Firefox/WebKit.
DONE 2026-11-18 13:58 UTC
