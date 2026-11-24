# security-auditor: cs3b dependency batch (manifests + lockfile), read-only review

Question: may this one batch of manifest and lockfile changes be committed and built?
Worktree: /home/user/Shamble-Frontlines/.gate/ci-wasm-wt, branch cs/3b-loop-closes, HEAD 2ab665c5 (base main 28254a73).
Batch: uncommitted Cargo.toml, Cargo.lock, crates/{room,client-endpoint,sim-wasm,devtools}/Cargo.toml.

## Plan
1. Measure the six sha256 values; compare with the caller's.
2. Lockfile diff vs HEAD: only path-dep edges between our own crates; no registry package added/removed/changed;
   no version/source/checksum line touched (script the comparison: parse both lockfiles).
3. Edges vs ADR 0002 part 3 crate table (who may depend on whom); no cycle; room <-/-> client-endpoint;
   sim depends on nothing of ours; protocol without transport/async and with no new dependency.
4. What newly reaches the players' Wasm (sim-wasm cdylib): room + protocol; protocol's ts-rs is dev-only
   (cargo tree -e normal for wasm32 target, --locked --offline); same for devtools.
5. build-checks.mjs runtimeCrates / simFeatureList and build.mjs: can these edges trip or weaken either check.
6. Root Cargo.toml profile: overflow checks on for workspace crates in release; named overrides for outside
   crates; no new outside crate -> no new override needed.
7. The root comment change: true as written? meets 3A-MANIFEST-COMMENT (STATUS_ARCHIVE ~1435)?
8. Anything else misleading in the six files.

Method note: my scratch scripts live outside the repository, in the session scratchpad
(/tmp/claude-0/-home-user-Shamble-Frontlines/f3c2a423-4254-5158-8151-83cb434f56ec/scratchpad/scripts/:
sa-lockdiff.mjs, sa-checkcrates.mjs), and a copy of HEAD's Cargo files + crates/ (git archive HEAD) in
scratchpad/head-copy for the before/after runs. Only read-only cargo calls: cargo tree / cargo metadata with
--locked --offline. No cargo config file in the worktree, its parents or ~/.cargo (checked), so no [source]
replacement or [patch] from outside shapes these outputs. (A first copy of sa-lockdiff.mjs was written into this
notes folder by mistake and removed at once; this file is my only write in the repository.)

## Findings (a line per finding, as I go)
- F0 checksums (sha256sum, measured at the start and again at the end, unchanged):
  Cargo.toml 593297a811501a67fc87273b795da4d423fa1bb0086d30d988d96418958489a8
  Cargo.lock a73653ea4f8a14f0849d23cf43ecae308240d1fb96b48bb5656609e6157562b8
  crates/room/Cargo.toml 83aeccc17e2b120af4814a3e589c71b0c213ad5bd1d57185fb8bcc566cbe14ae
  crates/client-endpoint/Cargo.toml ccf21ea7c637b78fa3ee4277dc52fd69bf9830d2c58677ca262f7ebf78601c49
  crates/sim-wasm/Cargo.toml 5105930087245dff087bffdbbe68ee53cf10582169a8470e41e6c14e7f0f50f5
  crates/devtools/Cargo.toml 006a3b7ae0a60cc65b1bc6b7dff2b529309f21bb27612f5fa903c76fc4a25b8e
  All six equal the caller's. HEAD 2ab665c5 equals main 28254a73 on Cargo.toml, Cargo.lock and crates/ (git diff
  --quiet); merge-base = origin/main. protocol and sim manifests, rust-toolchain.toml, package.json,
  package-lock.json and packages/ unchanged vs HEAD. All six files: ASCII, LF, git diff --check clean, no -diff attribute.
  Note: crates/room/Cargo.toml and crates/client-endpoint/Cargo.toml are not new files (git status: M); they existed
  with no [dependencies] table, and the batch adds one to each (the caller's wording only).
  CONCURRENT CHANGE seen during the review (outside the batch, not reviewed): crates/protocol/src/{lib,messages,
  ts_export}.rs modified; codec.rs, loopback.rs, transport.rs, wire.rs, wrap.rs and tests/decoder_inputs.rs new
  (untracked), appearing while I worked. They touch no manifest. ts_rs uses there are cfg(test)/cfg_attr(test) only.
  tests/decoder_inputs.rs:16 has #![allow(unsafe_code)] for a counting GlobalAlloc with SAFETY comments, the same
  pattern as crates/sim/tests/state_hash_alloc.rs:10, :35-37: an integration test, never in a library; for step 1's
  code review.
- F1 lockfile (scripted, sa-lockdiff.mjs; git show HEAD:Cargo.lock vs working tree): header equal (version = 4);
  22 packages before and after; none added or removed; only 4 path packages change dependency lists:
  client-endpoint +[protocol, sim], devtools +[client-endpoint, protocol, room], room +[protocol, sim],
  sim-wasm +[protocol, room]; nothing removed. Registry packages changed: 0; the 16 (name@version|source checksum)
  entries identical; checksum lines identical (16); one source only: registry+https://github.com/rust-lang/crates.io-index.
  Cargo.lock:20-23, 28-35, 65-68, 83-89. `cargo tree --locked` succeeds, so the lockfile matches the manifests
  (CI's --locked commands will not want a re-lock; CI's cargo fetch --locked fetches the same 16). PASS.
- F2 edges vs ADR 0002 part 3 (docs/adr/0002-code-structure-build-toolchain.md:184-195, rules :207-222):
  room -> sim, protocol (allowed :189); client-endpoint -> sim, protocol (allowed :190); sim-wasm -> client-endpoint,
  room, sim, protocol (allowed :191); devtools -> anything (:194). sim -> [] (rule :186: content only; no content crate
  yet). protocol: no new dependency (still only dev-dependency ts-rs, crates/protocol/Cargo.toml:9-10, untouched).
  room and client-endpoint do not depend on each other; their normal closure is {protocol, sim}: no wasm-bindgen,
  transport or async runtime (rule :207-210). Graph is a DAG: sim, protocol < room, client-endpoint < sim-wasm < devtools
  (cargo refuses cyclic normal deps; tree and metadata resolve). cargo metadata: every new edge is kind normal, path,
  default features, no `features =`, not optional, no target cfg; no crate has build.rs, [features], [target],
  [patch], [replace], links or [build-dependencies]. PASS.
- F3 players' Wasm (cargo tree -p sim-wasm --target wasm32-unknown-unknown, -e normal and -e all --target all):
  newly in the graph: protocol and room, both ours (crates/*). No registry crate enters; ts-rs appears nowhere in
  sim-wasm's or devtools' trees; `cargo tree --workspace --target all -e normal -i ts-rs` prints nothing; with -e all
  ts-rs is only under protocol's [dev-dependencies]. Cargo Book (Specifying Dependencies, Development dependencies):
  dev-dependencies are used only for tests/examples/benches and "are not propagated to other packages which depend
  on this package". protocol's ts_rs use is cfg(test)-gated, and cfg(test) is never set when protocol is compiled as
  a dependency. Code: room and protocol cannot add exports to the players' file by themselves: no no_mangle,
  export_name, link_section, wasm_bindgen or unsafe in protocol/room/client-endpoint/sim library sources (grep, twice),
  all six crates take [lints] workspace = true with unsafe_code = "deny" (Cargo.toml:20-21; the rustc lint covers
  no_mangle/export_name/link_section), and neither crate can use #[wasm_bindgen] (no dependency on it). Their code
  reaches the file only where sim-wasm calls it (cs3b steps 3-4: the code review's matter). PASS.
- F4 build checks (sa-checkcrates.mjs: the exact cargo tree arguments of build.mjs:95-113 plus --offline and
  --manifest-path, fed to the real functions of packages/client/scripts/build-checks.mjs; nothing built):
  BATCH: runtimeCrates = [cfg-if, unicode-ident, wasm-bindgen, wasm-bindgen-shared]; missingOverflowOverrides = [];
  simFeatureList = ["sim feature \"default\""] for the players' Wasm, the test-only module and the native tests;
  simFeatureDifferences = []. HEAD COPY: identical values. So the edges neither trip nor change either check.
  Weakening: none. runtimeCrates counts as "ours" only sources under <root>/crates/ (build-checks.mjs:240-247);
  protocol and room are there and are workspace members (Cargo.toml:3 members = ["crates/*"]), so
  [profile.release] overflow-checks = true (Cargo.toml:30-31) covers them, not "*" (Cargo.toml:33-34, Cargo Book:
  "*" = all dependencies but not workspace members). simFeatureList reads sim's features across the whole tree
  (build-checks.mjs:288-307); more parents of sim cannot hide a feature. Every tree line parsed (the function
  throws otherwise). devtools' own flat list: the same 4 runtime crates, all overridden. PASS.
- F5 profile: release overflow-checks on for members (Cargo.toml:30-31); "*" off for non-members (:33-34); named on
  for cfg-if, thiserror, ts-rs, unicode-ident, wasm-bindgen, wasm-bindgen-shared (:39-55). Non-proc-macro registry
  crates in the whole native normal+dev graph: exactly those six (sa-checkcrates.mjs). The batch adds no outside
  crate and does not touch the profile section (diff: lines 14-16 only), so no new override line is needed. PASS.
- F6 root comment (Cargo.toml:14-16): true. wasm-bindgen is a direct dependency of sim-wasm (crates/sim-wasm/
  Cargo.toml:17) and devtools (crates/devtools/Cargo.toml:21) and of no other crate (cargo metadata; Cargo.lock:34,
  :88); devtools' test-only module links sim-wasm's panic hook (crates/devtools/src/wasm.rs:2, :9-10) and uses
  #[wasm_bindgen] itself (wasm.rs:7, :16, :24, :31, :38). Meets 3A-MANIFEST-COMMENT (docs/STATUS_ARCHIVE.md:1435-1437):
  "in sim-wasm only" is gone and "(a later cargo-deny rule on wasm-bindgen must allow both)" carries the devtools
  half. Wording nit L1 (Low, not a security defect): "The bindings the Worker loads ...: in sim-wasm, and in devtools
  for the test-only WebAssembly module" can be read as the Worker loading the test-only module; it never does
  (build.mjs:11-16), and wasmProblems refuses it in a built page. Possible wording: "wasm-bindgen: in sim-wasm, whose
  bindings the Worker loads (ADR 0002 parts 2 and 5), and in devtools, for the test-only WebAssembly module, which
  links sim-wasm's panic hook and is never shipped (a later cargo-deny rule on wasm-bindgen must allow both)."
  Not a condition; at the session's choice (a change brings the batch back with new checksums).
- F7 L2 (Low, not a security defect): crates/devtools/Cargo.toml:18-19 gives sim-wasm a second reason, "the local
  match's own step, which the replay runner and the link simulator's tests run unchanged": that code does not exist
  yet (sim-wasm/src has lib.rs and panic.rs; the plan's steps 4, 7 and 8 add it), and the plan's ladder
  (cs3-plan.md:138-145, C4) may move L1 and the first recording to a 3c. Then this comment and devtools' three new
  edges (client-endpoint, protocol, room; needed by steps 7-8) would say more than 3b holds, and fixing them is a
  manifest change under the hold. Not a condition now; noted so a ladder move checks this manifest too.
  (Smaller, notes only: client-endpoint's comment "The simulation's constants" fits 3b; when prediction arrives it
  runs sim's step through the same edge and no manifest change will prompt the comment.)
- F8 L3 (Low, pointer for the later cargo-deny change, no change here): every path edge reports req "*" (cargo
  metadata), 4 at main and 13 with this batch; with ADR 0002's `wildcards` at deny (G5 row, :579), deny.toml will need
  `allow-wildcard-paths = true` (valid because every crate is publish = false: Cargo.toml:9, metadata publish=[]),
  or every path edge fails. From my reading of the cargo-deny book (bans, allow-wildcard-paths); not re-read today
  (medium-high confidence). The same change takes the wasm-bindgen rule with wrappers sim-wasm and devtools.
- Threat model note (no finding): with room in sim-wasm the local server's match logic, and in local play its seed,
  sit in the players' Worker (by design: SLICE-ORDER condition 1, ADR 0002:189-191). The online server must never
  accept anything the client's room computes, and the bindings must not hand the seed to the page (ADR 0004 part 8;
  my memory). Both are code-review items for steps 3-4, not this batch.

## Verdict
The dependency change may be committed and built: yes. No security finding. Lows L1-L3 are wording and a pointer
(none a security defect, none a condition). The yes covers exactly the six sha256 values above; any edit to any of
the six files (the Lows included) brings the batch back.

## Memory line owed (the caller made this file my only write; for the session to allow later)
- 2026-11-24, cs3b dependency batch (six files, sha256 above): yes, no conditions. Path edges only (room,
  client-endpoint -> protocol, sim; sim-wasm +protocol, room; devtools +client-endpoint, protocol, room); registry
  set unchanged (16); build checks unchanged (runtime crates cfg-if, unicode-ident, wasm-bindgen,
  wasm-bindgen-shared; sim feature list "default" in all three builds). Lows: root comment wording, devtools comment
  vs the ladder, deny.toml needs allow-wildcard-paths.

DONE
