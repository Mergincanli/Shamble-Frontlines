# rust-reviewer, round 2 review of cs1-types (foundation step 2.4 "Types")

Code = 65a7b13 (branch claude/wonderful-hamilton-jdmlnf, head 6c3a24a); round 1 reviewed 82142fa; base main 22c6afa.
Round 1: APPROVE with six Lows L-a..L-f (rust-reviewer-review1.md).

## Plan
1. Read `git diff 82142fa..65a7b13 -- crates` and `-- packages/types/src/generated`; fix-pass-1.md.
2. Round-1 Lows L-a..L-f: closed or not, one line each, with file:line.
3. New code: repository_root (CARGO_MANIFEST_DIR run time vs compile time: cargo test variants, env -i, symlinks,
   trailing slash, Windows), write_generated (canonicalize, unlinked, create_dir, delete loop, writes), entries_of +
   generated_layout_is_current checks, new_scratch_folder, the doc-comment check vs ts-rs 12.0.1 format_docs,
   layout.rs (destructuring in const fn, const assert LAYOUT_HASH != 0, fixtures, known answers recomputed in Python).
4. Probes only in a scratch copy with its own target folder (.gate/rr-cs1-r2 or refresh .gate/rr-cs1), never the main target.
5. Commands (all --locked): fmt --check; clippy native and wasm32 -D warnings; pedantic on protocol + client-endpoint;
   test --workspace; test --release -p protocol -p client-endpoint; wasm32 build of 5 crates; cargo tree -p protocol
   -e normal; cargo doc -D warnings; -W missing_docs on both crates.
6. Findings one line each as found; DONE at the end; handback <= 35 lines.

## Findings (one line each, as found)

## Commands run (exit codes)
- Main folder (own target; code = 65a7b13, HEAD d34cfc5 only adds docs): fmt --check 0; clippy native -D warnings 0;
  clippy wasm32 -D warnings 0; pedantic protocol+client-endpoint native 0 and wasm32 0, no warnings; test --workspace 0
  (ce 5, protocol 2, others stubs); test --release -p protocol -p client-endpoint 0 (5 + 2); wasm32 build of 5 crates 0;
  cargo tree -p protocol -e normal (host, wasm32, all) = protocol alone, 0; -i ts-rs = dev-dep of protocol only;
  cargo doc -D warnings 0 (target-dir .gate/rr-cs1-r2/target-doc-main); cargo rustc --lib -W missing_docs both crates
  0 warnings (target-dir .gate/rr-cs1-r2/target-md-main). Scratch copy .gate/rr-cs1-r2 = git archive 65a7b13, own target.
- (info, checked right) Known answers recomputed by an independent Python FNV-1a over the documented stream
  (.gate/rr-cs1-r2/probe/fnv.py): BASE 0x9adbecc7, empty 0x811c9dc5, TAG_PAIR 0x4265359c / 0xbd51cd2f (untagged streams
  byte-identical, both 0x60d6dbf4), block 0x2df78c30, BASE + 12 CHANGED all distinct; RFC 9923 vectors match.
- (info) Cargo book, "Environment variables Cargo sets for crates": "this applies for running binaries with cargo run
  and cargo test as well" -> CARGO_MANIFEST_DIR is set for the test process whatever the parent env (CI's env -i ok).
  Same binary name in the main target and the scratch target (client_endpoint-bdbcb2c4df73f18c): -C metadata of a
  workspace member is path-independent, which is how round 1's cross-tree reuse happened.
- L2-1 (Low, follow-up) layout_ts.rs:44-53, ts_export.rs:17-26: one tree reached by two path spellings across
  invocations fails an honest run. Probe: link .gate/rr-link-r2 -> rr-cs1-r2; cargo test from the real path, then
  `cargo test -p client-endpoint --manifest-path <link>/Cargo.toml`: no rebuild (fresh), panic at layout_ts.rs:47,
  left ".../rr-link-r2/crates/client-endpoint" right ".../rr-cs1-r2/crates/client-endpoint", exit 101; message blames
  "another copy". cd into the link: cargo uses the physical cwd -> passes (exit 0). Fails closed, never a wrong pass.
  Fix: compare fs::canonicalize of both (same physical tree passes, another copy still stops), or name the case + cargo
  clean -p in the message.
- (info, checked right) Write mode through a symlinked repository path, one invocation (fresh build, --manifest-path
  via the link, SHAMBLE_WRITE_TYPES=1 in the scratch copy): exit 0, all 9 files rewritten after the start, identical
  to the main tree. The canonicalized root makes an honest linked checkout pass.
