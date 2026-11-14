# U4d2 review round 1: security-auditor (read only)

Seat: the advisor's U4d2 brief check, A12 (the third unsafe exception; check.mjs's unsafe and rustflags scans).
Read copy: /home/user/Shamble-Frontlines/.gate/u4d2-r1 (51520b4 + snapshot). Builder's copy:
/home/user/Shamble-Frontlines/.claude/worktrees/u4d2-build (run in only through u4d2-run.sh; never edited). Logs:
/home/user/Shamble-Frontlines/.gate/u4d2/r1-sec-*.log. Scratch outputs (rustc builds): .gate/u4d2/sec-probe/.
This agent has no Edit tool: the notes are rewritten whole with Write each time.

## Plan
1. Threat model (below), then read the packet: heads, the diffs for hash-wasm, lab's hash.rs, check.mjs; the
   judged-against sections (§5 U4d2, §4.2, §4.5, conditions 7 and 8, Q3, the U4d2 brief ruling A1-A15, Q1-Q12, L1-L5).
2. The unsafe scan: read check.mjs's scan code; list the bypasses it misses. Prove them with scratch probe scripts in
   this notes folder that import the builder's check.mjs and loader.mjs and feed them in-memory sources (no edit of any
   copy), run through the runner.
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
- Out of scope here: the server's network input (no network code in U4d2). Every attack below needs a committed change
  or a changed environment; the scans guard against accidents, review against a deliberate change (check.mjs's own
  words above DEPS_CHECKED).

## Block 1: read (done)
- hash-wasm: Cargo.toml (deny + clippy keys, cdylib, bites fwd, npt-sim only), src/lib.rs (7 exports, thread_local
  state, no unsafe block), loader.mjs (export list incl. `memory`, empty imports, 64 KiB chunks).
- check.mjs diff whole: hashWasmProblems/exportAfter (:497-600), rustflagSources/buildFlagProblems/readBackCheck
  (:1443-1553), simFeatureCheck (:1556-1653), wasmStep (:2010-2363); existing lexRust, rustAttributes, pathProblems,
  cargoConfigProblems, cargoConfigFiles, layoutProblems (build scripts allowed if .rs in the crate folder), run.
- Environment: only CARGO_HTTP_CAINFO, RUSTUP_HOME, RUST_BACKTRACE set (names only); the one cargo config read is
  spikes/netcode-proof/.cargo/config.toml ([build] target-dir only); ~/.cargo has no config. rust-toolchain.toml 1.99.1.
- libm 0.2.16's build script: cfgs from features, target and OPT_LEVEL only (ENSURE_NO_PANIC needs a dep not present).
- npt-lab hash.rs: legs are current_exe() children (no PATH lookup), lock step over pipes; manifest names unvalidated
  (joined into paths), the file is committed and reviewed.
- Judged against: §5 U4d2, §4.2, §4.5, conditions 7 and 8, Q3, the brief check's A1-A10, A15, Q4-Q7 read. Q7 chose the
  source scan (a) over the loader's type section (b).
- The builder's plants read (p1-p4 logs): P1 features caught (two pairs named), P2 +fma by name and read-back, haswell
  and bleeding-edge by the read-back, P3 stray unsafe at lib.rs:212 (a new fn: rustc refused it too, exit 101), P4
  extra export refused by the scan and by the loader.

## Block 2: probes (done)
- r1-sec-scan.log (sec-probe-scan.mjs, the copy's own unsafeProblems on in-memory variants; exit 0):
  V0 real files 0 problems; V1 `unsafe` block inside begin's body CAUGHT (lib.rs:101); V2b export inside a module
  with `use core::primitive::f64 as u32;` 0 problems (passes); V3 `type u32 = f64;` 0 (passes); V4 build.rs writing
  `\x75nsafe` code + `use std::include as inc; inc!(concat!(env!("OUT_DIR"), "/gen.rs"))` in begin's body 0 (passes);
  V4b `include!` by name with a computed path CAUGHT; V5 a shebang line with one quote alone: 0 (no hiding crafted);
  V6 cfg_attr(unsafe(no_mangle)) CAUGHT; V7 multi-line `#[unsafe(\n no_mangle\n)]` accepted (lawful); V8 macro_rules
  unsafe CAUGHT; V9 no_mangle static CAUGHT; V10 a second `step` in a module CAUGHT (count 2); V11 `-> !` CAUGHT; V12
  multi-line f32 CAUGHT; V13 core::primitive::u32 refused (fails closed). Name rule: `-C llvm-args=...`,
  `--cfg feature="bites"`, `-C overflow-checks=off`, `-C target-cpu=haswell`, `-Ctarget-feature=+avx512f` pass the name
  rule (haswell and avx512f are the read-back's); `-Ctarget_feature=+fma`, `--codegen=target-feature=+sse2,+fma`
  refused. rustflagSources ignores CARGO_BUILD_TARGET.
- r1-sec-rustc.log (exit 1 by design, `-k`): rustc 1.99.1; sec-probe.rs (crate-level deny) compiled for wasm32 with
  an `unsafe` block inside an allowed export's body; its Wasm types (sec-probe-wasm.mjs): `begin (i32, i32) -> (i32)`,
  `feed (f64) -> (f64)` though the source reads `word: u32`. The builder's own two .wasm files: all seven exports i32
  only, `feed (i32) -> ()`, `memory` (the invariant holds today). `-C llvm-args=-enable-unsafe-fp-math` refused by
  rustc ("Unknown command line argument"); `-fp-contract=fast` accepted; `--cfg feature="bites"` shows in --print cfg.
- r1-sec-fp.log: `-enable-no-nans-fp-math` and `-enable-no-signed-zeros-fp-math` unknown to rustc 1.99.1;
  `-enable-no-trapping-fp-math` accepted, same assembly as without; baseline x86-64 `a*b+c` is mulsd + addsd. So no
  fast-math LLVM option reachable through llvm-args was found; the only accepted ones do nothing without FMA.
- r1-sec-imports.log (sec-probe-imports.mjs, the copy's loader `instantiate`): `env.now` refused; a module importing
  `constructor.keys`, `__proto__.valueOf` or `constructor.constructor` (JS's Function) INSTANTIATED with
  moduleProblems [] (the import object `{}` inherits Object.prototype); `step()` through `constructor.constructor`
  returned 0.
- r1-sec-tf.log, r1-sec-tf2.log: under `#![deny(unsafe_code)]` and no rustflag, a safe
  `#[target_feature(enable = "relaxed-simd")]` fn calling `core::arch::wasm32::f32x4_relaxed_madd`, called from safe
  code, compiles for wasm32 and emits `f32x4.relaxed_madd` (tf.s:14); linked as a cdylib its `target_features` custom
  section lists `+relaxed-simd +simd128` (sec-probe-custom.mjs); the builder's two npt-hash-wasm modules list neither
  (bulk-memory, bulk-memory-opt, call-indirect-overlong, multivalue, mutable-globals, nontrapping-fptoint,
  reference-types, sign-ext). Node instantiated it with `{}`.

## Findings (a line after each)
- SEC-U4D2-1 Medium (before merge): the loader's "a module that imports anything fails" (loader.mjs:9-10, :72-85) is
  false: `new WebAssembly.Instance(module, {})` (:80) resolves imports named after Object.prototype's properties
  (`constructor.constructor` = Function, `constructor.keys`, `__proto__.valueOf`), and such a module instantiates with
  moduleProblems [] (r1-sec-imports.log); check.test.mjs:2011-2014's env.now case passes because `env` is not on
  Object.prototype, not by a rule. Reach is small (a Rust import needs an `unsafe extern` block the scan refuses, and
  nothing reachable gives a clock or randomness), but the stated guarantee and its test are wrong. Fix: moduleProblems
  refuses every `WebAssembly.Module.imports(module)` entry by name before instantiating; the import object is
  `Object.create(null)`; a self-test with a `constructor.constructor` import.
- SEC-U4D2-2 Medium (before merge): no test or plant shows the scan's one sole-layer case, an `unsafe` block inside a
  listed export's body: the export's placed allow lets rustc accept it (sec-probe.rs, r1-sec-rustc.log, exit 0). The
  stray-unsafe bite (check.test.mjs:1916; the live plant p3-unsafe.log, lib.rs:212) sits in a new `fn stray()`, where
  rustc's deny refuses it too (p3's `cargo build` exit 101), so it passes whether or not the scan reads export bodies.
  The code is right today (V1 caught at lib.rs:101); a change to tcpinfo's fnBodies pattern (unsafe allowed inside the
  listed functions, check.mjs:463-470) would pass every test. Fix: one self-test assertion (`unsafe {}` inside begin's
  body → hwUnsafe at that line), and the plan's stray-unsafe plant placed in an export body, with `cargo build` exit 0
  shown beside the deps FAIL.
- SEC-U4D2-3 Low: the signature scan (check.mjs:575-597, EXPORT_INTEGERS :212) reads the token `u32`, not the type: a
  module-local `use core::primitive::f64 as u32;` (V2b) or `type u32 = f64;` (V3) passes it, and Wasm then types the
  export `feed (f64) -> (f64)` (r1-sec-rustc.log). Q7 chose the source scan; this limit is not written beside it. Fix:
  state it in the comment, or (Q7 (b), about 60 lines: sec-probe-wasm.mjs) the loader reads each export's type from the
  module's type section and refuses anything but i32.
- SEC-U4D2-4 Low: C2R-2's known limit now reaches npt-hash-wasm, whose placed allows cover the export bodies: a
  build.rs (layoutProblems allows one, check.mjs:1240-1247) writing code from escapes, pulled into begin's body by a
  renamed `include!` (`use std::include as inc;`), passes the scan (V4) and, inside the allow, rustc. pathProblems' doc
  (check.mjs:367-370) still says "into npt-tcpinfo, only review stops it". Fix (cheap): refuse a custom-build target in
  the two deny exceptions, and any `include` path in them other than the `include!(...)`/`include_str!`/`include_bytes!`
  call forms; name npt-hash-wasm in the comment.
- SEC-U4D2-5 Low: the build-flag rule (check.mjs:1466-1509) is a deny-list of four names; `--cfg feature="bites"`
  (planted defects compiled into a measured build, or one target's feature cfg that `cargo tree` cannot see, so the
  features comparison passes), `-C overflow-checks=off` (overrides plan §4.5's overflow-checks rule) and `-C llvm-args`
  pass the name rule and are invisible to `--print cfg`'s target_feature lines (r1-sec-scan.log, r1-sec-rustc.log). No
  fast-math LLVM option was found reachable (r1-sec-fp.log). Fix: refuse every non-empty rustflags source (the spike's
  builds use none: e1-check.log's "given no rustflags"); that one rule also covers every CPU name.
- SEC-U4D2-6 Low: the native release builds (check.mjs:2032-2033) pass no `--target`, so a `build.target` config key
  or CARGO_BUILD_TARGET (neither read: rustflagSources and cargoConfigProblems) builds npt-lab for another installed
  target while the read-back and the features tree read x86_64-unknown-linux-gnu. Here only wasm32 is installed, so it
  fails closed today. Fix: `--target x86_64-unknown-linux-gnu` on both npt-lab builds (builtArtifact reads cargo's
  JSON, so the path follows).
- SEC-U4D2-7 Medium (not before merge; the advisor's call: the fix pass or a carry beside C-U7-1): the build-flag check
  reads rustflags only; a source-level `#[target_feature(enable = "simd128" | "relaxed-simd")]` on a safe function,
  callable from safe code on wasm32, puts SIMD and relaxed (implementation-defined) float ops into the Wasm leg with no
  rustflag and no `unsafe` (r1-sec-tf.log: `f32x4.relaxed_madd` under `#![deny(unsafe_code)]`), and no scan, ban or
  read-back sees it (npt-sim's clippy.toml bans no `core::arch`). The linked module records it: its `target_features`
  custom section lists `+relaxed-simd +simd128` (r1-sec-tf2.log), absent from both npt-hash-wasm builds today. Fix
  (about 10 lines): moduleProblems refuses a module whose `target_features` section lists simd128 or relaxed-simd, or
  that has no such section (fail closed), with a hand-built self-test module; optionally the deps scan refuses
  `target_feature` attributes and `core::arch`/`std::arch` paths in the crates the Wasm leg compiles. Pre-existing for
  npt-sim since U4s; U4d2's check is the one that claims "simd128 and relaxed-simd off".
- SEC-U4D2-8 Low: moduleProblems' kind branch (loader.mjs:53: a listed name exported as another kind, such as `begin`
  as a global or `memory` as a function) has no self-test; check.test.mjs's wasmModule builds functions and one memory
  only. Fix: one case each.

## Block 3: coverage (done)
| Attack path | Where it is checked | Gap |
|---|---|---|
| `unsafe` block, fn, impl, trait in hash-wasm | hashWasmProblems :558-574; rustc deny outside the export items | inside export bodies the scan alone, untested (SEC-U4D2-2) |
| `unsafe extern` block | the scan (test :1940) | none |
| `#[unsafe(export_name)]`, `#[unsafe(link_section)]` | the scan (tests :1941-1944) | none |
| cfg_attr around unsafe or the allow | the scan (V6; test :1933) | none |
| attribute over several lines | NO_MANGLE `\s*`, rustAttributes | none (V7 lawful) |
| macro_rules body | the scan reads it as code (V8) | none |
| raw identifier `r#unsafe` | not the keyword (UNSAFE_CODE's lookbehind) | none |
| strings, comments, lexer desync | lexRust; C2R-1 shebang and `\u{}` limits | documented; review only in hash-wasm |
| include!, #[path], build.rs | pathProblems, layoutProblems | renamed include! + build.rs (SEC-U4D2-4) |
| path outside scanned folders, symlinks | layoutProblems, crateFiles | none beyond C2R-2 |
| export not on the list | scan + loader moduleProblems (test :2005; P4) | none |
| export of another kind | moduleProblems :53 | untested (SEC-U4D2-8) |
| imports | empty import object | Object.prototype names resolve (SEC-U4D2-1) |
| export signatures | the source scan :575-597 | name shadowing (SEC-U4D2-3); artifacts i32 today |
| recording length, chunks | 64 KiB reads, readSync lengths | none; dump_len trusted (own module, no timeout: note) |
| env rustflags (4 names) | rustflagSources `/RUSTFLAGS$/` | none |
| cargo configs ([build], [target.*], cfg tables, CARGO_HOME, upward) | cargoConfigFiles, tomlValues any table | none; `--config` not passed |
| unreadable value, failed read-back | cargoConfigProblems; readBackCheck problems; non-vacuity | fails closed |
| flag forms (-C, -Cx, --codegen[=], `_`) | buildFlagProblems | --cfg, overflow-checks, llvm-args (SEC-U4D2-5) |
| implied features (CPU names, avx512f) | rustc --print cfg read-back | none (P2 haswell, bleeding-edge) |
| source-level target features | nothing | SEC-U4D2-7 |
| build.target / CARGO_BUILD_TARGET | nothing | SEC-U4D2-6 (fails closed here) |
| RUSTC_WRAPPER, build-script output, linker wrapper | documented limits above DEPS_CHECKED | as documented |
| features differ between builds | simFeatureCheck, three builds, transitive, non-vacuity | --cfg feature unseen (SEC-U4D2-5) |
| Cargo.lock, new dependencies | diff: npt-hash-wasm → npt-sim only; no npm change | none |
| child processes | spawnSync without shell; process.execPath; legs current_exe | none (cargo, rustc from PATH as before) |
| stale scratch files | WASM_IO.reset rmSync + mkdir each run; exists() sees this run only | none; artifacts copied from the shared target's uplift path: only W1-TARGET's lock stops another copy's build swapping it (note) |
| manifest names in paths | joined unvalidated (hash.rs:393) | committed file, review (note) |

## Verdict
REVISE: two Mediums before the merge (SEC-U4D2-1, SEC-U4D2-2: each a few lines and one self-test), one Medium for the
advisor to route (SEC-U4D2-7), five Lows (SEC-U4D2-3 to -6, -8). No High or Critical. The third exception's code holds
today: every probe of the scan's documented rules was caught, both built modules export the seven i32 functions and
`memory` only, import nothing and list no SIMD feature. TE2-5: no unplanted failure met.
