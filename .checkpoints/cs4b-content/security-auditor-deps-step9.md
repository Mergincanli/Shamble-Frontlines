# security-auditor: dependency check of 4b-2 step 9 (ts-rs as content's dev-dependency)

Brief: `.checkpoints/cs4b-content/deps-step9-brief.md`. Worktree `.gate/lane1-takeover`, branch `cs/4b-world`, head
54e89d2. Batch: `.gate/cs4b/batch-step9/` (two files), diff `deps-step9.diff`, digests `deps-step9.sha256`. No build, no
network, git read-only (one git command per shell call). No agent memory folder in this environment (checked again):
earlier decisions come from `security-auditor-deps-4b1.md` and `security-auditor-deps-4b2.md`.

## Plan
1. Digests and scope: `sha256sum -c` in the batch folder; HEAD = 54e89d2; the tree's Cargo.lock and
   crates/content/Cargo.toml equal HEAD's (batch not in the tree); re-diff the batch against `git show HEAD:<path>`
   myself and compare with deps-step9.diff; file hygiene (regular, ASCII, LF, no symlink).
2. Lockfile: parse HEAD's and the batch's lockfile; every entry equal except content's `dependencies`; ts-rs 12.0.1 and
   its tree (ts-rs-macros, thiserror, syn ...) already locked; no source/checksum/version moves.
3. Q2 players' builds: read build.mjs / build-checks.mjs (the three tree commands, simFeatureList,
   simFeatureDifferences, TEST_ONLY_FEATURES) and CI; in a copy at `.gate/security-auditor-step9-scratch/ws` (batch's two
   files copied in, digests checked) run `cargo tree --offline --locked`: players' Wasm and test-only module lists,
   `-i ts-rs` on wasm32 and host without dev edges, native list; compare entry for entry with
   crates/devtools/vectors/sim-feature-lists.json; lock digest unchanged after every call. Reason about cargo's unit
   graph (dev-deps only for the declaring package's test units) and resolver-2 feature decoupling (has_dev_units).
4. Q2 exception 2: does content's dev edge widen where thiserror's std is on (e.g. `cargo test -p sim`, now that the
   type writer hangs under a crate in sim's normal tree)? Check with cargo tree from sim with dev edges.
5. Q3 derive: cfg_attr(test, ...) strips before expansion; read protocol's pattern (helper attributes inside cfg_attr
   too); content's lib.rs (no_std? forbid(unsafe_code), panic lints); grep ts-rs-macros 12.0.1 cached source for unsafe,
   std paths, env/file reads at expansion time.
6. Q4 missing pieces: step 9's plan (checkpoint / plan doc); bundle parsing without a dependency; packages/types
   package.json exports for generated/content; ts-rs features content's types need (import-esm, serde-compat, ...);
   .cargo/config env; dependency_rules.rs edit holds build.rs/target tables.
7. Answer (<= 15 lines).

## Progress
- [ ] 1 digests and scope
- [ ] 2 lockfile
- [ ] 3 players' builds
- [ ] 4 exception 2
- [ ] 5 derive
- [ ] 6 missing pieces
- [ ] 7 answer

## Findings (one line each, as made)
- [x] 1 digests and scope. HEAD is 0505b88, one notes commit over the brief's 54e89d2: `git diff --stat 54e89d2 HEAD`
  lists only .checkpoints files (no manifest, no lockfile). `sha256sum -c deps-step9.sha256` in the batch folder: two OK
  (56bd2d52...9c47 Cargo.lock, 620a400b...9b8c crates/content/Cargo.toml). The tree's Cargo.lock (ad0f7680..., my
  step-6 approval), crates/content/Cargo.toml (1c20979e..., my 4b-1 approval) and root Cargo.toml (a854d5e3...) equal
  HEAD's blobs (`git hash-object` = `git ls-tree`); `git status`: only two untracked notes files. Batch not in the tree.
  My own `diff -u` tree-vs-batch gives exactly the two hunks of deps-step9.diff; `patch -p0 -i deps-step9.diff` on
  copies of the head's files (`.gate/security-auditor-step9-scratch/patched/`) gives both digests OK and no third
  file. Batch files: regular, root-owned, 644, pure ASCII, LF, no BOM, no CR, newline at end.
- [x] 2 lockfile. The tree-vs-batch diff is one hunk of 3 added lines (content's `dependencies = ["ts-rs"]`); every
  other line is byte-identical, so no package is added, removed or moved, no source/checksum changes, format stays 4.
  Already locked and unchanged: ts-rs 12.0.1 (756050...31d8; deps thiserror, ts-rs-macros), ts-rs-macros 12.0.1
  (38d90e...52aa; proc-macro2, quote, syn 2.0.119), thiserror 2.0.21 / thiserror-impl 2.0.21. Cached manifests: ts-rs
  `build = false`, asks `thiserror = "2"` with default features (= std); default features off at the root, so
  serde-compat and termcolor stay off (termcolor is not in the lock). ts-rs-macros: proc-macro, `build = false`.
- [x] 5 derive (Q3). content/src/lib.rs:12 `#![forbid(unsafe_code)]`, :13-24 the panic/arithmetic lints under
  `cfg_attr(not(test), deny(...))`; no `no_std` (the derive's std paths compile). Protocol's pattern: messages.rs:31..171
  `#[cfg_attr(test, derive(ts_rs::TS))]`, no `#[ts(...)]` helper attribute anywhere, export by ts_export.rs (a
  `#[cfg(test)] mod`, lib.rs:22-23) through `Config::new()` (ts-rs lib.rs:607 = defaults, no env; only `from_env`,
  :618-641, reads TS_RS_*). ts-rs-macros 12.0.1 (2926 lines): no `unsafe`, no env/fs/process/net/clock at expansion;
  output (lib.rs:95-114) is an `impl TS` of string/path functions, plus a `#[test]` only under `#[ts(export)]`
  (lib.rs:41-43). ts-rs runtime: no `unsafe`. Non-test builds: `cfg_attr(test, ...)` is removed before expansion, and
  `test` is set only for the crate's own unit-test target, so the lib as sim/sim-wasm/devtools/integration tests
  compile it holds nothing of ts-rs; a bare `derive(ts_rs::TS)` or a bare `#[ts(...)]` would fail to compile (ts_rs is
  not in a non-test build's extern prelude): fail-closed, never silent code. Unsafe in the output would be refused by
  forbid (fail-closed too). Answer: no code in any non-test build.
  Correction to the line above: a bare `#[ts(...)]` fails because, without the derive, `ts` is an unknown attribute
  (not because of the extern prelude). Step 9's helper attributes, if any, go inside `cfg_attr(test, ts(...))`.
- [x] 3 players' builds (Q2), resolved in `.gate/security-auditor-step9-scratch/ws` (batch, digests OK) and `ws-head`
  (head, ad0f7680/1c20979e), cargo 1.99.1, `--offline --locked`, RUSTUP_AUTO_INSTALL=0, CARGO_NET_OFFLINE=true, no
  cargo config; both lock digests unchanged after every call. Without dev units ts-rs is not even in the targeted
  resolve: `cargo tree -p sim-wasm --target wasm32 -e normal,build -i ts-rs` and the same for `-p devtools` -> "package
  ID specification `ts-rs` did not match any packages"; `--workspace -e normal,build -i ts-rs` on host and on wasm32 ->
  "nothing to print" (`--target all` cannot run offline: serde's never-built archive is not cached). With dev edges,
  `-i ts-rs --workspace` shows content and protocol as its only askers. Full feature trees (normal+build, proc macros
  included, --no-dedupe), head vs batch, paths normalised: identical md5 for the players' Wasm (14750 lines), the
  test-only module (19905), CI's wasm32 build line toolchain.yml:167 (29139), CI's release test line :162 (55687) and
  `cargo test -p sim` (5120). build.mjs's three lists via build-checks.mjs's own functions
  (`security-auditor-deps-4b2-lists.mjs`, build.mjs:113-131 commands): 71/71/76 on both copies, each equal entry for
  entry to crates/devtools/vectors/sim-feature-lists.json; compare with the exceptions: []; without them exactly the
  same five entries (safe_arch x3, thiserror default/std); no crate lacks its overflow block. Mechanism: cargo links a
  dev-dependency only into the declaring package's own test/example/bench units, and with no test unit in the
  command the resolver leaves dev-dependencies out (both builds players get are `cargo build`, build.mjs:145, :175).
  Answer: no build players get, and not the simulation's normal tree.
- [x] 4 exception 2 (Q2). `cargo test --workspace` tree (toolchain.yml:158-159), head vs batch: the same 131 distinct
  package/feature entries (md5 c98a4d27 both); the only diff is the new `ts-rs (*)` edge under content (dedupe marks).
  thiserror "default"/"std" were already on there through protocol's identical line; ts-rs =12.0.1 with no features
  from both askers, so unification cannot differ. `-p sim` with dev edges does not reach content's dev edge (no ts-rs,
  no thiserror std, head and batch alike), so the narrower test runs keep their features; CI's release line has
  protocol and is byte-identical. New only: `cargo test -p content` compiles ts-rs + thiserror std, a build with no
  sim in it; a hand-run `cargo test -p content -p sim` would hold thiserror std in sim's tree (native, not modelled,
  build-checks.mjs:282-283 says so). Exception 2 (build-checks.mjs:321-327) is pinned to entries, not askers: its
  basis is unchanged. The "known clash" (build-checks.mjs:285-288: a FEATURE only dev-dependencies turn on, reaching
  sim's tree) does not fire: the batch adds no feature (content has no [features]); cfg(test) + dev edge instead.
- [x] Sources tied to the lock (`security-auditor-deps-4b2-src.py`, python -I, reads only): ts-rs 12.0.1 (89 files),
  ts-rs-macros 12.0.1 (21), thiserror 2.0.21 (105), thiserror-impl 2.0.21 (17): unpacked = archive, no missing or
  extra file; each archive's sha256 = the batch lock's checksum. ts-rs: no build script, 0 unsafe; ts-rs-macros: proc
  macro, 0 unsafe; thiserror's build.rs and thiserror-impl's one `unsafe` token are not new (thiserror is in every
  build since step 6; the proc macro already runs). Lock necessity: head lock + batch manifest (`ws-mixed`) ->
  `cargo tree --locked` refuses ("cannot update the lock file ... --locked"); batch lock resolves with no rewrite.
- [x] 6 missing pieces (Q4). No second batch foreseen. Content's kinds (map.rs:73-136, movement.rs:38-54,
  bundle.rs:35-37) use String, Vec, [f64; 2|3], f64, u32 and plain/struct-variant enums: ts-rs 12.0.1 covers them
  with default features off (engine-architect ran it on the map's types, card line 430); the `.ts` import extension
  is runtime `Config` (protocol: ts_export.rs:110-112), not the old import-esm feature; no serde, no third-party type,
  so no `*-impl`/serde-compat/format feature. packages/types/package.json exports only "." -> re-exports go in
  src/index.ts (code). sim, sim-wasm (`content` line, its comment names the constructor) and devtools already depend
  on content; bytes cross wasm-bindgen as &[u8] with no js-sys. CI's write-again check covers all of
  packages/types/src/generated (toolchain.yml:159-161). Must ride in the same commit (not a manifest): the rule edit
  (with the batch alone `the_content_crate_depends_on_nothing` fails, dependency_rules.rs:56-60). Not manifests, for
  step 9's plan: helper attributes inside cfg_attr; `Config::new()` not `from_env()` (no TS_RS_* steering); its own
  folder generated/content and scratch name; the card's open note that the content scan does not take a file-level
  `#![cfg(test)]` as test code (card line 436/462). Also: Grid.thinnest is i64 -> `bigint` by default if exported.

## Findings
- SA-STEP9-1 Low (the rule edit's commit, not the batch): `the_rule_reads_what_it_should_refuse`
  (dependency_rules.rs:155-169) compares its plants with ["package", "lints"] and plants exactly
  `[dev-dependencies]\nts-rs = { workspace = true }`, the line that becomes allowed. Left as is it still passes,
  but proves nothing about the new rule. Fix: compare with the new list, and plant a second dev-dependency and a
  ts-rs line with a feature (`features = ["format"]`), each seen through `table(.., "dev-dependencies")`.
- SA-STEP9-2 Low (wording, same commit): build-checks.mjs:285-288 says the step that first nests generated types in
  sim's tree decides which side gives way; step 9 adds no feature (cfg(test) + dev edge), so the compare is
  untouched. Say so there, so no later change adds a `ts` feature to content on ADR 0002 part 5's old plan.
- No Critical, High or Medium.

## Verdict
APPROVE on the two digests of deps-step9.sha256: Cargo.lock
56bd2d52483a46eb0843b465e21a8d3492e3df1b9263b46bf9c7cdb61f3b9c47, crates/content/Cargo.toml
620a400b14da6030dce2fda1f336358f3edc13cc0d92f802bdfd260416738b9c. Committed with the rule edit; after the commit,
check the committed blobs against both digests; no cargo call without --locked between copy-in and commit.
Not checked: RustSec/OSV (no network; the session's OSV note lists none for ts-rs's tree: not re-verified);
heapless 0.8.0's nine advisories (unchanged by this batch; its own request); nothing compiled (the derive on
content's real types untried); `--target all` (offline: serde's archive not cached); non-Linux hosts.
- [x] 7 answer
DONE
- Correction: step 1's short form "620a400b...9b8c" is a typo; the digest ends ...738b9c (full text in the Verdict).
- Progress: all seven plan steps are done (the [x] lines above supersede the unticked Progress list).
