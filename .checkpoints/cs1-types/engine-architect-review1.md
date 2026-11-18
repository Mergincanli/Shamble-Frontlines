# engine-architect, round 1 review of cs1-types (foundation step 2.4, "Types")

Head reviewed: code of commit 82142fa (branch claude/wonderful-hamilton-jdmlnf); base main 22c6afa.
Memory: no engine-architect memory file exists (.claude/agent-memory-local/ holds security-auditor only).

## Plan
1. Read the diff (Cargo.toml, crates/protocol, crates/client-endpoint, packages/types), the approved plan
   (plan.md over plan-draft.md), my plan notes, the checkpoint's build notes, rust-reviewer's answer.
2. Read the drafts: ADR 0002 part 3 (lines ~137-217), part 5 (~284-360), migration step 4, PM-0002-15..19, -43, -63;
   ENGINE_OVERVIEW (block, generated types); build plan card 1 and Part 7; part-15 file (lanes).
3. Check 1: beyond-card / missing; public symbols and tests vs plan-draft section 4 + plan item 5(g).
4. Check 2: BLOCK_LAYOUT as the one description; what CS2's writer and the page need; hash coverage.
5. Check 3: the three deviations.
6. Check 4: packages/types shape, exports, naming scale, checkLayoutHash's home, ts-rs output style.
7. Check 5: ADR departure (no TS_RS_EXPORT_DIR/.cargo/config.toml, no #[ts(export)]); pointer text;
   R5-LAYOUT-OQ4, R5-TSLINT-OQ3 answers.
8. Check 6: cargo tree --locked; imports in packages/types.
9. Check 7: PM-0002-15/-16/-17/-19/-63 met vs shown.
10. Answer, at most 45 lines, via SubagentHandback.

## Findings (a line each, as found)

- (info, check 6) cargo tree --locked --offline: client-endpoint has no edge of any kind (-e normal,build,dev --target
  all); protocol has no normal or build edge on any target; ts-rs is reached only as protocol's dev-dependency (-i
  ts-rs -e all --target all); wasm32 normal edges: the six crates alone. packages/types imports only its own files
  (index.ts, layout-check.ts, generated/protocol siblings) and vitest in tests; its package.json has no dependencies.
- (info) tsc 6.0.3 by path: .gate/cs1-types/tsconfig.json exit 0, tsconfig.shipped.json exit 0 (lists the 9 generated
  files, layout-check.ts, index.ts). Both configs carry A2:421-426's flags plus noEmit, allowImportingTsExtensions,
  module preserve, target/lib es2023, types []; tests config adds skipLibCheck.
- (info) symbols: everything in plan-draft s4 and plan 5(g) is built under its planned name; added public items are
  ElementType::name(), ElementType::size() and the derives (Debug, Clone, Copy, PartialEq, Eq) on the three table
  types; added private: fnv1a, FNV consts, SECTION_TAG/FIELD_TAG (deviation 1), test helpers (field, section,
  name_is_plain, broken_rule, GOOD, BAD, BASE, SECOND, CHANGED), write_generated x2, new_scratch_folder, files_in,
  GENERATED_FOLDER/FILE_NAME. Removed as planned: stub_builds, PACKAGE. Missing: none. Renamed: none.
- (verified) ts-rs-macros 12.0.1 src/lib.rs:319-327: with #[ts(repr(enum))] the decl is format!("enum {} {{ {} }}")
  -> a TS enum declaration, which erasableSyntaxOnly refuses (TS1294, the plant's 17b). Default output: type aliases,
  Rust enums as unions. -> R5-TSLINT-OQ3's answer for ts-rs needs "never repr(enum)" (Low, report wording).
- (verified) exhaustive struct patterns in a const fn compile on rustc 1.99.1 ed. 2024 (scratchpad probe d.rs,
  const assert holds) -> layout_hash/render_typescript can destructure Section/Field so a new member fails to compile
  until both handle it (Low, now: same files as rust-reviewer's fixes).
- F-PM15 (Medium, report wording): PM-0002-15 (A2:1287) Where=CI, How="G4 step; the two planted defects", 1 of 1
  each; BP:208 "met" needs every clause "in CI". In CI: only clause 1 (export tests in toolchain run 169). Defects and
  control ran here (plant-results.txt), not in CI -> "shown"; met in full with G4 --prove (PM-0002-43, 2.3 after 1).
  BP:454 says "met at 1" (my draft s2:16 repeated it); inconsistent with BP:458 (PM-19's runs here = shown). The report
  says what ran where and names the difference from BP:454. No decision of his changes.
- F-ALIGN (Low): layout.ts states `align` (layout_ts.rs:22-26) but the hash does not cover it (layout.rs:73-96); its
  rule lives only in test code (layout_ts.rs:22-23, layout.rs:139-140). Not in A2:348's list (offsets, strides,
  element types); no reader (the page needs offsets and types; the writer places sections). Drop it from layout.ts now;
  Section::align() enters with the writer (cs2), Rust-only. If ever printed again, hash it.
- F-NAMES (Low): `export const <section>` (layout_ts.rs:26) breaks on reserved words (rust-reviewer L-a), can collide
  with another top-level export of index.ts, and needs a hand line per section in index.ts:12 and index.test.ts:6.
  One object keyed by section name, `export const BLOCK_LAYOUT = { header: {...} } as const` (the Rust name), avoids all
  three, keeps section order, and needs no reserved-word list. Now: before cs2's page imports names.
- F-SHORT (Low): checkLayoutHash refuses `byteLength < header.stride` before it reads the hash (layout-check.ts:15-17).
  The hash's place is pinned forever (layout.rs:49-51, layout_rules_hold); once counts grow the header (cs4) a block
  of an older layout reads as "too-short" instead of "layout-hash". Read the hash once offset+4 bytes exist, compare,
  then require header.stride. Now or by cs4.
- F-FORMAT (Low, cross-lane): 27 lines with trailing spaces in 7 ts-rs files (grep " $"), unformatted ts-rs layout;
  2.3's CS-C brings Prettier/ESLint and BP:966-967 plans "one mechanical commit" of formatting: it must ignore
  packages/types/src/generated/** (tsc keeps it), else the export tests fail after a format pass. Report follow-up.
- F-NOREADER (Low, the drafts; Mustafa only if a draft changes): no TS code reads the message types in the decided
  design (A2:374-378 H2: the page sees only the block; codec and join exchange are Rust in the Worker: BP:317-320).
  D3 (A2:350-352; R1-BND-REC4 "low-rate messages") and the card ask for them; keep (they prove the ts-rs pipeline
  before content types, cs4; nothing shipped). Drafts' next revision: name their reader or narrow D3.
- F-DOC (Low nit): protocol/src/lib.rs:2-3 and messages.rs:2-3: "the codec enters with the protocol's first messages"
  while the module now holds "the first message types": name change-set 3 / the loopback exchange.
- cs2 note: packages/types/package.json has no "exports"/"types" entry: the first import by package name (cs2) needs
  one; its form (TS source vs built .d.ts for tsc -b references) is agreed with 2.3's CS-C tsconfigs.
- F-STALE (High, same root as test-engineer's F-HIGH-1; my part: reproduced live + the boundary's design): at
  ~13:20 UTC `cargo test --workspace --locked --offline` in the main folder ran client_endpoint-bdbcb2c4df73f18c built
  13:04:04 from security-auditor's removed worktree (backtrace: .gate/cs1-types/sa-plant/crates/client-endpoint/src/
  layout_ts.rs:68), no rebuild ("Finished"), and failed "packages/types/src/generated/block cannot be read". Cause:
  env!("CARGO_MANIFEST_DIR") is baked at compile time (layout_ts.rs:59, ts_export.rs:77) and cargo judges a binary built
  in another worktree of the same workspace fresh when the target dir is shared (plant.sh:13 CARGO_TARGET_DIR=$root/
  target). The export tests then check another tree: a vacuous pass (runs20) or a false failure (mine). Fix: (1) every
  worktree its own CARGO_TARGET_DIR; rebuild the main target before any evidence run (it holds sa-plant binaries now);
  (2) hardening: read the folder from std::env::var_os("CARGO_MANIFEST_DIR") at run time (cargo docs, environment
  variables: "Note that this applies for running binaries with cargo run and cargo test as well"), so a binary looks
  in the checkout cargo runs in. Stale code stays a risk, so (1) is the must. No tree file changed by my run.
- (read) test-engineer's notes: F-HIGH-1 is the same root as my F-STALE (it also cites the cargo env-var page and
  suggests failing fast when run-time and compile-time CARGO_MANIFEST_DIR differ: I endorse that form). My additions:
  the third occurrence (security-auditor's sa-plant run, shared target, its R2 line) and the live state: the main
  target holds 13:04 binaries from the removed sa-plant worktree, so the main folder's next cargo test is wrong until
  rebuilt (own target per worktree; or touch the two crates' sources).
- Deviations: (1) tags: right; my draft's stream was ambiguous (rust-reviewer's pair); keep + its L-f pair test.
  (2) in-memory: right; ts-rs has no in-memory export_all (export_to_string covers one type, not its dependencies'
  files), ours is a pure function; both compare names and bytes of the whole folder; keep. (3) helper + stride rule:
  right; the stride rule is the multi-row half of A2:311's condition (row k's u32 at start + k*stride stays aligned);
  the section-start half is the writer's (cs2/cs4); keep.
- rust-reviewer's notes: F1 by-name handle -> cs2 (writer is the first user; const fn lookup panicking at compile time;
  never positional). align() -> cs2, Rust-only (drop `align` from layout.ts now: F-ALIGN). Padding between sections ->
  cs4's plan (first entity section; cs2's block is header only, BP:300): header states each section's row count and
  byte offset as table fields, written from Section::align(); the page binds at stated offsets; no placement rule in TS.
- PM words: -15 clause 1 met in CI, defects+control shown -> "shown" overall (F-PM15); -16 shown in two halves (planted
  (4) in the plant worktree, rebuilt there; Vitest refusal), method needs cs2's writer; -17 shown with tsc -p and
  scratch configs (re-ran: exit 0 both); -19 NOT shown as recorded (F-HIGH-1), shown after a re-run on its own target;
  -63 shown in part (two of three generators; CI carries -15 clause 1 and -16's Vitest half).
- OQ answers: R5-LAYOUT-OQ4 designed out (one writer per folder), the main session's 20 runs must be redone
  (test-engineer's valid cross-check: one set 81bd1a49...); reopens with #[ts(export)]. R5-TSLINT-OQ3: yes for ts-rs's
  default output and ours; never #[ts(repr(enum))] (emits a TS enum); wasm-bindgen's .d.ts at cs2.
- Verdict: REVISE (F-STALE High, shared root with test-engineer F-HIGH-1; F-PM15 Medium for the report). Lows:
  F-ALIGN, F-NAMES, destructuring, F-SHORT, repr(enum) wording, F-FORMAT, F-NOREADER (drafts), F-DOC; cs2 note on
  packages/types "exports". Code unchanged since 82142fa at 13:26 UTC (git diff --stat 82142fa HEAD, code paths).
- Not checked by me: which binary ran in each planted-defect run (test-engineer's); CI logs (relied on the checkpoint
  for run 169); clippy, fmt, wasm32 build, Vitest (records and other reviewers); Firefox/WebKit.
DONE 2026-11-18 13:27 UTC
