# Change-set 1, Types (foundation step 2.4): plan draft
By engine-architect, 2026-11-18, from main 22c6afa, for the main session (it owns the plan). Keys: BP = the build plan,
docs/reports/2026-11-17-build-plan-slice-1.md; P15 = its part 15 file; A1, A2, A4, A5 = ADR 0001, 0002, 0004, 0005;
TY = .github/workflows/toolchain.yml; S1 to S13 = sources summarised in engine-architect-plan.md beside this file.

## 1. Goal
The page and the simulation share one description of their data, generated from Rust and committed in
`packages/types`; a test fails when the two drift, and the page can refuse a block of an unknown layout (BP:283-296).

## 2. Done-when and pass marks
Step 2.4's done-when (A2:1493-1494): "the marks of the spike's item 4 hold in CI" (item 4: A2:1166-1170). "Met": every
clause holds; "shown": it passed here (BP:208-211). The gate and CI edits that follow (BP:977-978: G4, the 20 runs in
CI, a type check) are asked of Mustafa with their exact text, drafted in the report (BP:25), outside this change-set.
- PM-0002-15 (A2:1287): "regenerating every generated type (the ts-rs output, the block's layout and its hash) gives 0
  changed bytes against the committed files; a field renamed in an exported Rust type without regenerating fails; a
  hand edit to a generated file fails". Met here (BP:454): the export tests run in today's `cargo test` (TY:82-83), the
  defects run once. Its gate form, G4: 2.3 after 1.
- PM-0002-16 (A2:1288): "a Rust layout change without regeneration fails at build or at the page's startup (the page
  refuses a block whose layout hash differs), proven by a committed test". Shown: the export test fails on a stale
  layout, `checkLayoutHash` refuses in Vitest (TY:86). Met 2.3 after 2: `sim-wasm` with a field added (BP:455).
- PM-0002-17 (A2:1289): "0 errors from `tsc -b` on `packages/types` with the base tsconfig (`strict`,
  `erasableSyntaxOnly` included)"; both bites required; an ambient `enum` recorded. Shown: tsc by path from a scratch
  config, the bites, `declare enum` recorded (BP:456, BP:936). Met with 2.3's base config.
- PM-0002-18 (A2:1290): "CI fails when the wasm-bindgen CLI and the library version in `Cargo.lock` differ": cs2.
- PM-0002-19 (A2:1291): "two generator runs on one commit give byte-identical files; 20 runs give 20 identical sets".
  Shown: 20 runs, hashed (BP:458). Met 2.3 after 1.
- PM-0002-63 (A2:1337): "PM-0002-15 to PM-0002-19 hold in the foundation's CI with the three generators of
  `packages/types`": two of three here, met 2.3 after 2 (BP:459). PM-0002-43 (A2:1317, G4's bites) is gate work.
Answered here (BP:765): R5-LAYOUT-OQ4 (PM-0002-19, section 4) and R5-TSLINT-OQ3 for ts-rs and our generator (O4).

## 3. Files touched (about 505 hand-written lines, counted as BP:235-238 says)
| Path | New or changed | What it holds | Lines |
|---|---|---|---|
| `Cargo.toml` | changed | `ts-rs = { version = "=12.0.1", default-features = false }`, workspace table | 1 |
| `crates/protocol/Cargo.toml` | changed | `[dev-dependencies]` `ts-rs = { workspace = true }` | 2 |
| `Cargo.lock` | generated | ts-rs, ts-rs-macros, thiserror(-impl), proc-macro2, quote, syn, unicode-ident | - |
| `crates/protocol/src/lib.rs`, `messages.rs` | changed, new | modules; the eight types of section 4 | 125 |
| `crates/protocol/src/ts_export.rs` | new, test only | the export test and its write mode | 60 |
| `crates/client-endpoint/src/lib.rs`, `layout.rs` | changed, new | modules; the table, its hash, their tests | 133 |
| `crates/client-endpoint/src/layout_ts.rs` | new, test only | the generator and its export test | 80 |
| `packages/types/src/generated/` | generated | `protocol/` (eight files by ts-rs), `block/layout.ts` (ours) | - |
| `packages/types/src/layout-check.ts`, `.test.ts` | new | `checkLayoutHash` and its Vitest tests | 85 |
| `packages/types/src/index.ts`, `index.test.ts` | changed | the exports; the stub `PACKAGE` and its test go | 20 |
ts-rs 12.0.1 (newest on docs.rs today; MIT; MSRV 1.88.0, under our 1.99.1: S1, S4) brings ts-rs-macros =12.0.1 and
thiserror 2 (S4), proc-macro2, quote, syn (S5); the rest of the lock list is inferred, then read back. Picked by
A2:350-352; let in by A2:160-161 and Cargo.toml:11 (workspace table), A2:215-216 (crates.io), A2:353-356
(dev-dependency, `cfg_attr`). No `serde-compat` (it pulls termcolor: S5); `=` makes a bump its own change. Security
review: BP:221-222, P15:73. Tier 1: root `Cargo.toml` is unguarded today (BP:219). Unchanged: root `package.json`, the
npm lockfile, every tsconfig, rustfmt, clippy file (2.3's: BP:964-969, P15:135-137), `.github/`, the other crates.

## 4. The exact symbols
**`protocol`** (A2:188): `pub` structs deriving `Debug, Clone, PartialEq, Eq`, plus `#[cfg_attr(test,
derive(ts_rs::TS))]`; no serde (the codec is cs3's); Rust field names kept in TypeScript. "Prov." = open in the drafts:
provisional; cs3's codec fixes wire widths; the proof test may move them (BP:1257-1258). Ticks `u32` (A4:185, A4:155).
- `Join` (A1:685): `token: Vec<u8>` (format open, lobby at Slice 3; prov.); `protocol_version: u16` (A1:446-447; prov.);
  `content_hash: [u8; 32]` (SHA-256: A5:431-436).
- `JoinAccepted` (A1:686): `slot`, `team`, `squad: u8` (prov.; u8 holds 48+); `mode_id: String` (ids are strings:
  A5:275); `mode_hash: [u8; 32]` (named nowhere else: prov., O1); `content_hash: [u8; 32]`; `server_tick: u32`;
  `tick_rate_hz`, `snapshot_rate_hz: u8` (30 Hz, A1:125; 20 Hz under pressure, A1:450-451; prov.).
- `Input` (A1:682): `acks: Vec<SnapshotAck>` ("newest snapshot decoded (ack, per entity group)"), with `SnapshotAck {
  group: u8, tick: u32 }` (A1:293-294; prov.); `commands: Vec<Command>` (up to the cap, data, proposed 8: A1:220-221).
- `Command` (A1:225-227): `tick: u32`; `sequence: u16` (16-bit or narrower, A1:1607; wraps, A1:234); `move_x`,
  `move_y: i8` (length at most 1, A1:234; prov.); `buttons: u16` (bitfield; buttons come with cs4 and cs8: prov.);
  `presses: [u8; 16]` (a counter per button, A1:1607's width rule; prov.); `aim: AimPoint`; `view_time: ViewTime`.
- `AimPoint` (A1:226-227, "at most 80 bits"): `x: i16`, `y: u16`, `z: i16` in cm, E3's step and ranges (A1:280), as
  PM-0002-77 proposes (A2:1286); y up as in three.js, fixed by no draft read (prov.). `ViewTime` (A1:227):
  `tick: u32`, `fraction: u16` in 1/65,536 of a tick (prov.: the sim's time unit is set at cs4, A4:191-196, BP:768).
- `Snapshot` (A1:683), header only (sections are soldier and event data: cs4, cs8): `server_tick: u32` (A1:318-319);
  `baseline_tick: Option<u32>` (none: coded against defaults, A1:277-278; the 16-bit wire offset is cs3's);
  `newest_command_received`, `last_processed_command: Option<u16>` (sequences, A1:228-230); `group: u8` (A1:293-294).
Out: `JoinRejected` (not on the card), ids, sections, events. In TS (S7): `number`, tuples, `Array`, `T | null`; no u64.
**The layout table** (`client-endpoint/src/layout.rs`; shipped, as cs2's writer uses it: A2:309-310): `pub enum
ElementType { U8, U32 }` (a variant enters with its first use: U32 the header, U8 the rules test); `pub struct Field {
name: &'static str, ty: ElementType, offset: u32 }`; `pub struct Section { name: &'static str, stride: u32, fields:
&'static [Field] }` (fields `pub`); `pub const BLOCK_LAYOUT: &[Section]`: one section, `header`, stride 8, `tick` u32 at
0, `layout_hash` u32 at 4 (A2:288); `pub const LAYOUT_HASH: u32 = layout_hash(BLOCK_LAYOUT);`. Offsets are written in
the table and checked by a test. Each entity section adds its u32 count to the header when it enters; none here (O2).
**The hash**: `pub const fn layout_hash(sections: &[Section]) -> u32`, FNV-1a 32-bit (RFC 9923: basis 0x811C9DC5, prime
0x01000193, xor then wrapping multiply: S13) over one byte stream: per section its name's UTF-8, 0x00, stride as u32
little-endian; per field its name, 0x00, its type's name ("u8", "u32"), 0x00, offset as u32 little-endian. Fixed widths,
no `usize`, no `std::hash::Hasher` (A4:343-349); it covers every value the generated file states. Why: drift detection,
not security (RFC 9923 section 6); exact in a JS number; no dependency; compile time. TypeScript never recomputes it:
cs2's writer puts `LAYOUT_HASH.to_le_bytes()` at its offset (A2:310-311); the page compares `getUint32(offset, true)`
with the generated constant, printed from the same Rust const.
**The generator** (`layout_ts.rs`, test only): `fn render_typescript(sections: &[Section], hash: u32) -> String`, pure.
It writes `generated/block/layout.ts`: a "generated, do not edit" line naming its source, `export const LAYOUT_HASH =
0x%08x;`, and per section one `as const` object: stride, align, each field's offset and type (`"u32"`).
**The export tests**, one per crate, unit tests in `src/` (`cfg_attr(test)` derives exist only in the crate's own test
build: S11, S12), write into `std::env::temp_dir()/shamble-types-<crate>-<pid>`; protocol builds
`ts_rs::Config::new().with_out_dir(dir).with_import_extension(Some("ts"))` and calls `export_all(&cfg)` on `Join`,
`JoinAccepted`, `Input`, `Snapshot` (S2, S3). File names and bytes are compared with the committed folder; a difference
fails, naming the file. With `SHAMBLE_WRITE_TYPES=1` a test clears its committed folder and writes there (ts-rs
overwrites only what it writes, S6). No `#[ts(export)]`, `TS_RS_EXPORT_DIR` or `.cargo/config.toml` (BP:294-295,
BP:955-959): `set_var` is unsafe in edition 2024 (S10), `unsafe_code` denied (Cargo.toml:16). One thread per crate
writes, under ts-rs's lock (S6): R5-LAYOUT-OQ4's race cannot occur, and PM-0002-19's runs show it.
**`packages/types`**: `src/index.ts` exports the eight message types (`export type`), `LAYOUT_HASH`, `header`,
`checkLayoutHash` and `type LayoutCheck`; `PACKAGE` goes (only its stub test used it). `src/layout-check.ts`:
`export function checkLayoutHash(block: ArrayBuffer): LayoutCheck`, `LayoutCheck = { ok: true } | { ok: false; reason:
"too-short"; byteLength: number } | { ok: false; reason: "layout-hash"; expected: number; found: number }` (readonly).
It reads the u32 at the generated offset, little-endian, never throws; the page (cs2) refuses the block and says why.

## 5. The tests
- `generated_types_are_current` (protocol), `generated_layout_is_current` (client-endpoint): scratch export equals the
  committed folder. `layout_rules_hold`: header first; `tick` at 0, `layout_hash` at 4, pinned (read before any other
  offset); fields aligned to their size, inside the stride, no overlap. `hash_covers_every_value`: on a fixture table
  (U8, U32), one changed name, type, offset or stride, or an added field, changes the hash. `fnv1a_matches_rfc_9923`:
  "" gives 0x811c9dc5, "a" 0xe40c292c (from S13). `typescript_carries_the_hash`. Kept: `overflow_checks_are_on`.
- Vitest: an equal hash passes; one bit off fails with both values; a buffer shorter than the header fails; the right
  hash written big-endian fails (proves little-endian reading); `index.ts` exports what section 4 lists.
- Planted defects, once, in a throwaway worktree (`git worktree add .gate/cs1-types/plant HEAD`), never committed, each
  with its command, exit code and failing message in the report. (1) `Snapshot::group` renamed, no regeneration:
  `cargo test -p protocol --locked` fails, naming `Snapshot.ts`. (2) One character changed in `Join.ts`: it fails,
  naming `Join.ts`. (3) Control: the rename, `SHAMBLE_WRITE_TYPES=1 cargo test -p protocol --locked`, then the plain
  run, tsc and Vitest pass. (4) A header field added, no regeneration: `cargo test -p client-endpoint --locked` fails;
  the report prints old and new `LAYOUT_HASH` (the page would refuse; the Wasm form is cs2's, BP:455).
- PM-0002-17's bites: a class with a parameter property, and an `enum`, each in a scratch file, each fails tsc; the
  result for `declare enum` is recorded (R5-TSLINT-OQ3).
- PM-0002-19: 20 runs of `SHAMBLE_WRITE_TYPES=1 cargo test --workspace --locked` (default threads), sha256 of every
  generated file after each: one distinct set, and `git diff --exit-code packages/types/src/generated` is empty.
- Today's CI, no workflow edit, runs the export, rule and hash tests (TY:82; protocol's in release, TY:83), the wasm32
  build without dev-dependencies, so without ts-rs (TY:84), and Vitest in `packages/types` (TY:86). Not run: a tsc
  type check (TY:87 prints the version), clippy, fmt, the 20 runs, the planted defects, G4.

## 6. Checks by hand before the report (BP:923-948, the rows "from 1"; tools by path, BP:925)
```
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo clippy --workspace --all-targets --locked --target wasm32-unknown-unknown -- -D warnings
cargo test --workspace --locked && cargo test --release -p sim -p protocol --locked
cargo build --locked --target wasm32-unknown-unknown -p sim -p protocol -p room -p client-endpoint -p sim-wasm
cargo tree -p protocol -e normal --target wasm32-unknown-unknown   # no ts-rs; with -e dev it shows
cargo clippy -p protocol --all-targets --locked -- -W clippy::pedantic -D warnings   # A2:440, before 2.3 sets it
npm ci --ignore-scripts --no-audit --no-fund && node node_modules/typescript/bin/tsc -p .gate/cs1-types/tsconfig.json
cd packages/types && node ../../node_modules/vitest/vitest.mjs run   # the bin path vitest's package.json names
node tools/gate.mjs
test ! -e .cargo/config.toml && git grep -n spikes -- ':!*.md'   # BP:955-959; and no "type" in root package.json
```
First read the "Lane requests" issue (P15:164-168). Scratch tsconfig, not committed (BP:966-967), text in the report:
A2:421-426's flags plus `noEmit`, `allowImportingTsExtensions` (stubs import `./index.ts`), `module: "preserve"`,
`target`, `lib` `es2023`, `types: []`. CI green on the head (BP:945). Not shown (BP:947-948): Firefox, WebKit, Safari.

## 7. Risks, ranked
1. Provisional widths move (proof test: OQ-0001-6, -7, R2-XU-OQ4; cs3's codec). Way out: regenerate, a small diff.
2. ts-rs output trips a strict flag, or `with_import_extension` wants ".ts", not "ts" (not verified). Shows: the first
   tsc run. Way out: the other form, or one field `#[cfg_attr(test, ts(type = "..."))]` with its reason.
3. 2.3's later tsconfig differs from the scratch one. Shows: when it lands. Way out: one mechanical commit (BP:966-968).
4. A derive leaks into a shipped build. Shows: `cargo tree -e normal`, TY:84. Way out: `cfg_attr` only; nesting another
   crate's types needs A2:353-356's feature form, which cs1 avoids (protocol nests only its own types).
5. wasm32 clippy `--all-targets` fails on the export tests: `cfg(all(test, not(target_arch = "wasm32")))` on them.
6. Write mode left on hides drift: CI never sets it, and G4 (2.3 after 1) regenerates and diffs on its own.
7. No file is shared with lane 2 (P15:90-95); lockfile conflicts are solved by regenerating (BP:1249-1250).
Size: stop line 1,200 (BP:235-238). Passed only by a field ts-rs cannot express, a general layout language or a shared
helper crate, none planned. Out first: such a field (an open point instead), then the fixture test's breadth.

## 8. Open points
- O1 (advisor agent; netcode-architect confirms at cs3's plan): every "prov." width and name, `mode_hash` above all
  (A1:686 alone names it). Recommendation: keep them, each marked provisional in its doc line.
- O2 (advisor agent): the header's counts. Recommendation: none until the first entity section (cs4) brings one.
- O3 (advisor agent): the startup check in `packages/types`, not `client`. Recommendation: `types` (P15:97-98).
- O4 (advisor agent): R5-TSLINT-OQ3 is answered for ts-rs and our generator only; wasm-bindgen's `.d.ts` comes in cs2
  (BP:292). Recommendation: the report records the split; cs2 answers the rest.
- O5 (advisor agent): the card names three reviewers (BP:296); a new dependency also gets security-auditor (BP:221-222,
  P15:73, CLAUDE.md protocol step 5). Recommendation: add it.

## 9. Reviewers and evidence
Reviewers at effort max (BP:278): engine-architect, rust-reviewer, test-engineer (BP:296), and security-auditor (O5).
Evidence (CLAUDE.md step 4, logic): tests on real code, the planted defects' output, the 20 runs' hashes, tsc's output,
section 6's checks, CI on the head; no Brave check (BP:247). The report adds the zip-route test archive, never committed
(BP:27, BP:992-996), the CI edits drafted for Mustafa (BP:25) and the answers of BP:765; docs-keeper: P15:24.
