# U4d2 review round 1: security-auditor (read only)

Seat: the advisor's U4d2 brief check, A12 (the third unsafe exception; check.mjs's unsafe and rustflags scans).
Read copy: /home/user/Shamble-Frontlines/.gate/u4d2-r1 (51520b4 + snapshot). Builder's copy:
/home/user/Shamble-Frontlines/.claude/worktrees/u4d2-build. Logs: /home/user/Shamble-Frontlines/.gate/u4d2/r1-sec-*.log.
This agent has no Edit tool: the notes are rewritten whole with Write each time.

## Plan
1. Threat model (below), then read the packet: heads, the diffs for hash-wasm, lab's hash.rs, check.mjs; the
   judged-against sections (§5 U4d2, §4.2, §4.5, conditions 7 and 8, Q3, the U4d2 brief ruling A1-A15, Q1-Q12, L1-L5).
2. The unsafe scan: read check.mjs's scan code; list the bypasses it misses (macro bodies, `unsafe extern`,
   `#[unsafe(export_name)]`, `#[unsafe(link_section)]`, `cfg_attr`, multi-line attributes, raw idents, strings and
   comments, `include!`, build.rs, paths outside the scanned folders). Prove them with a scratch probe script in this
   notes folder that imports the builder's check.mjs and feeds unsafeProblems in-memory sources (no edit of any copy),
   run through the runner.
3. The signature scan (Q7) and the loader's refusals (moduleProblems, exportProblems), the chunked streaming and the
   lengths the loader trusts.
4. The build-flag check (A8; condition 7 (a); Q4): rustflags sources, parsing, fail-closed, the read-back.
5. The features comparison (PM-0004-74; Q6; P1's plant).
6. Supply chain and process: Cargo.lock, child processes, the wasm-step scratch folder and stale files.
7. Findings with severity; the coverage table; the verdict.

## Threat model
- T1 a change slips `unsafe` past the scan into a build (any crate in the workspace, not only hash-wasm).
- T2 a build flag that changes float results (fast-math-like `-C` flags, target-feature, `-Z` options, a linker
  arg) reaches the native or wasm build without the check seeing it.
- T3 a dependency feature differs between the native and the wasm build of npt-sim (or a dependency of it).
- T4 a check passes for the wrong reason: a stale file in the scratch folder, a comparison of a thing with itself, a
  parse that defaults to "clean" on an error, a test that asserts a constant.
- T5 the loader trusts a hostile .wasm (imports, extra exports, memory export) or a hostile recording length.
- T6 process: child processes through a shell, paths from the environment, writes outside .gate/.
- Out of scope here: the server's network input (no network code in U4d2).

## Read so far (block 1, done)
- hash-wasm: Cargo.toml (deny + clippy keys, cdylib, bites fwd, npt-sim only), src/lib.rs (7 exports, thread_local
  state, no unsafe block), loader.mjs (export list incl. `memory`, empty imports, 64 KiB chunks).
- check.mjs diff whole: hashWasmProblems/exportAfter (:497-600), rustflagSources/buildFlagProblems/readBackCheck
  (:1443-1553), simFeatureCheck (:1556-1653), wasmStep (:2010-2363); existing lexRust, rustAttributes, pathProblems,
  cargoConfigProblems, cargoConfigFiles, run.
- Environment: only CARGO_HTTP_CAINFO, RUSTUP_HOME, RUST_BACKTRACE set (names only); the one cargo config read is
  spikes/netcode-proof/.cargo/config.toml ([build] target-dir only); ~/.cargo has no config. rust-toolchain.toml 1.99.1.
- libm 0.2.16's build script: cfgs from features, target and OPT_LEVEL only (ENSURE_NO_PANIC needs a dep not present).
- Judged against: §5 U4d2, §4.2, §4.5, conditions 7 and 8, Q3, the brief check's A1-A10, A15, Q4-Q7 read. Q7 chose the
  source scan (a) over the loader's type section (b).

## Candidate findings to prove (block 2, in progress)
- C1 signature scan reads the token `u32`, not the type: `use core::primitive::f64 as u32;` or `type u32 = f64;`.
- C2 build.rs + `use std::include as inc;` (pathProblems' known limit C2R-2) now reaches hash-wasm, where the placed
  allows cover each export's body: unsafe from OUT_DIR inside an export body, seen by no scan; is build.rs refused?
- C3 the placed allow covers the export's whole body (closures, matches), so rustc's deny does not back the scan there;
  a lexer desync (C2R-1's shebang or `\u{...}` limit) hides an `unsafe` block in a body. Hardening: one-call bodies.
- C4 `build.target` / CARGO_BUILD_TARGET not read: the native builds pass no `--target`, the read-back and the tree
  read x86_64-unknown-linux-gnu.
- C5 `-C llvm-args` and target-cpu names other than `native` are read by no name rule (the read-back catches CPU
  features; llvm-args are invisible to `--print cfg`).
- C6 pathProblems' doc comment still says "into npt-tcpinfo, only review stops it" (not hash-wasm).

## Findings (a line after each)
