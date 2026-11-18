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
