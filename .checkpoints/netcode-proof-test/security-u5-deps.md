Task: security-auditor DEP-HOLD dependency read for U5's early start (netcode proof test, PR #40, lane claude/netcode-proof-test-plan-hcky5g), Block 0 manifests and locks, libfuzzer-sys 0.4.13, arbitrary 1.5.0, cargo-fuzz 0.13.2, cargo-deny 0.22.0, cargo-audit 0.23.0, nightly-2026-11-01.

## Plan (blocks, in order)
- B0: context: advisor-u5-early.md condition 3 (:219-226), N3/N4 (:28-32); u5-brief.md Block 0 (:101-120); u4s-carries.md entry 107 (U5-Q7 term 3); u5-deps-facts.md.
- B1: item 1, verify the 8 sha256 hashes; read every manifest and both locks; diff against main's versions; added lines, what they pull in, anything beyond Block 0; root Cargo.toml comment.
- B2: item 2, libfuzzer-sys 0.4.13: build.rs (env vars, network, writes outside OUT_DIR, spawns), fuzz_target! expansion (unsafe, no_mangle, extern "C"), license, own deps.
- B3: item 3, arbitrary 1.5.0: unsafe, build script, license, deps.
- B4: item 4, tools: cargo-fuzz 0.13.2, cargo-deny 0.22.0, cargo-audit 0.23.0 (.crate Cargo.lock, build scripts, native code, run-time network, license).
- B5: item 5, nightly-2026-11-01 --profile minimal.
- B6: item 6, RustSec advisories (advisory-db) for the five crates at those versions; what could not be checked.
- B7: verdicts, conditions, dependency-check.md lines; DONE.

(No Edit tool in this agent: the notes are rewritten whole with Write at each block.)

## Notes (findings and block completions)
- B0 done: rule (main CHECKPOINTS.md:60-69), condition 3 (advisor-u5-early.md:219-226: nightly by rustup minimal and
  cargo-fuzz by `cargo install --locked --version` only after dependency-check.md; deny/audit read now, installed at
  Block 8; a version not cleared goes to the advisor), N3 (runner offline), N4 (/root, /tmp share the repo's disk);
  Block 0 (u5-brief.md:101-120); U5-Q7 term 3 (u4s-carries.md:801-807: any unsafe or no_mangle in the expansion named;
  a fifth exception to the advisor before Block 3). Facts: u5-deps-facts.md (8 manifest/lock hashes, fetch output, lock
  compare 127 kept / 3 added / 116 dropped / 0 changed). Next: B1.

### B1 (item 1: the manifests and locks) DONE
- Hashes: `sha256sum` in W (HEAD bd88fa756c6c, the lane head) matches the facts file for all 8 manifest/lock files and
  the lib.rs placeholder (e64d0d9e...). `git status` in W: M on the root Cargo.toml, Cargo.lock, crates/net, proto, sim
  Cargo.toml; new crates/alloc/ and fuzz/ only (plus the builder's two notes). crates/room and crates/client untouched
  (C-U5-1 and the client entry deferred: matches advisor-u5-early.md condition 1).
- Root Cargo.toml (git diff vs lane head): + "crates/alloc" member (:24); + `exclude = ["fuzz"]` with comment (:26-28);
  + `npt-alloc = { path = "crates/alloc" }` (:49-50); lints comment rewritten (:84-109): "Four crates", cite ADR
  0002:269 (verified: W/docs/adr/0002-code-structure-build-toolchain.md:269 holds "`unsafe_code` is denied in every
  crate; an exception needs a written invariant, a test and an `#[allow]` with its reason"), npt-alloc the fourth at
  deny, the fuzz crate's tables "key for key", check.mjs's added duties. No lint level, profile, dependency version or
  external crate changed. No `cargo-features`, `rustflags`, `build`, `[patch]`, `[replace]` or source key added.
- Cargo.lock diff: + `[[package]] npt-alloc 0.0.0` (path, no source/checksum); + "npt-alloc" in npt-proto's and
  npt-sim's dependency lists. Nothing else (diff read whole: 3 hunks).
- crates/alloc/Cargo.toml: workspace package keys; own [lints]: rust unsafe_code deny; clippy disallowed_types,
  disallowed_methods, iter_over_hash_type deny = the workspace's tables key for key but unsafe_code (as npt-tcpinfo's,
  crates/tcpinfo/Cargo.toml:17-23). No [dependencies], no build script (no build.rs in crates/alloc/), no `build` or
  `include` key. src/lib.rs: 2 doc-comment lines, no code.
- crates/sim/Cargo.toml:24-28: new [dev-dependencies] npt-alloc.workspace (dev only: never ships).
- crates/proto/Cargo.toml:29-32: npt-alloc.workspace appended inside its existing [dev-dependencies] (:23).
- crates/net/Cargo.toml: `fuzz = []` feature (:16-18), no dependency.
- fuzz/Cargo.toml: npt-fuzz 0.0.0, publish=false, edition 2024, own `[workspace] members=["."]`, `[package.metadata]
  cargo-fuzz = true`; [lints] rust unsafe_code forbid + the three clippy denies (= workspace key for key, :29-35);
  `bites = ["npt-proto/bites"]`; deps: `libfuzzer-sys = "0.4"` (:46), 7 path crates (npt-net with features ["fuzz"]),
  rand_chacha =0.10.0 / rand_core =0.10.1 default-features off (the workspace's pins); 17 [[bin]] targets under
  fuzz_targets/ (files not yet written); release profile overflow-checks for npt-sim, npt-proto (as the workspace).
  No build key, no cargo-features, no rustflags, no [patch]/[replace], no git or other-registry source.
- fuzz/Cargo.lock vs workspace lock (my own compare, scratchpad lockcmp.py, python -I tomllib): lock v4 both; ws 243,
  fuzz 130; added exactly arbitrary 1.5.0 (crates.io, checksum 3bc62ac9...), libfuzzer-sys 0.4.13 (crates.io,
  a9fd2f41..., deps arbitrary + cc), npt-fuzz (path); 116 dropped (outside the fuzz graph); 0 kept packages with a
  changed source or checksum; no new version of a name the workspace holds; sources: crates.io + path only.
  Dependency-list differences on kept packages are feature/graph pruning only (aws-lc-rs, wtransport,
  web-transport-quinn, futures-*, valuable, tracing-attributes dropped; npt-proto/npt-sim lose their dev deps, as a
  non-member path crate's dev deps are not resolved; "windows-link" name qualification). cc 1.11.1 (libfuzzer-sys's
  build dependency, with "parallel" -> jobserver 0.1.35, libc) is the workspace's own entry, same checksum
  (4ff2a812...). So the only new code is arbitrary + libfuzzer-sys.
- .crate files in the registry cache hash to the lock checksums (sha256sum: arbitrary 3bc62ac9..., libfuzzer-sys
  a9fd2f41...): the sources read are the locked ones.
- Beyond Block 0? Everything the brief lists is there; nothing outside its list but (i) fuzz/Cargo.toml's rand_chacha
  and rand_core (the seed generator; the workspace's exact pins, already in both locks: no new package) and its
  release-profile overflow-checks (mirrors the workspace), (ii) crates/alloc/src/lib.rs (a 2-line placeholder the
  resolve needs; not a manifest). Both in scope of U5's plan; no slipped dependency.
- FINDING SEC-U5D-2 (Low): fuzz/Cargo.toml:46 `libfuzzer-sys = "0.4"` is a caret range where every external crate the
  spike names is an exact `=` pin (root Cargo.toml:51 "External crates, exact versions"; fuzz/Cargo.toml:58-59 pin
  rand exactly). The lock binds 0.4.13 for every --locked build, and cargo never moves a satisfying lock entry without
  `cargo update`; the gap is only a re-resolve after a lost or regenerated lock, which then would pick a newer,
  unread libfuzzer-sys (and a new expansion). Fix: `=0.4.13`, at the next manifest change (with its read); and/or
  check.mjs pins libfuzzer-sys's lock entry (version + checksum) so a new expansion cannot arrive unread.
- Root comment "written as final": acceptable to save as is. Why: condition 5 keeps U5's content out of lane commits
  until its merge, so the comment ships only with the code and check.mjs duties it describes, and writing it once
  spares a second read. Conditions: (a) at U5's review, check.mjs must do exactly what :95-106 say for npt-alloc and
  the fuzz crate, or the comment changes (a manifest change: a second, comment-only read); (b) the "Four crates" count
  depends on the advisor's ruling on SEC-U5D-1 below (if he counts fuzz_target!'s no_mangle as a fifth exception the
  sentence changes, again a comment-only re-read).

### B2 (item 2: libfuzzer-sys 0.4.13) IN PROGRESS
- Manifest (registry src Cargo.toml): edition 2018; license "(MIT OR Apache-2.0) AND NCSA"; deps arbitrary "1";
  build-dep cc 1.0.83 features ["parallel"]; features default = ["link_libfuzzer"]; dev-deps flate2, rand (not
  resolved for a dependent). .cargo_vcs_info sha1 719e4efb9b8857ebaa782ae59376c8cbb78fed0f.
- build.rs (46 lines, read whole): with link_libfuzzer (default) and CUSTOM_LIBFUZZER_PATH unset: read_dir("libfuzzer")
  in its own package dir, every top-level *.cpp (26 files) into cc::Build, cpp(true), std c++17, frame pointers,
  warnings off, compile("libfuzzer.a") -> OUT_DIR. Env read by the script: CUSTOM_LIBFUZZER_PATH (if set: links that
  static lib instead, from its directory; rerun-if-changed on it) and CUSTOM_LIBFUZZER_STD_CXX (the C++ runtime to
  link; default stdc++). cc itself reads the usual compiler env (CC/CXX, CFLAGS/CXXFLAGS, AR, TARGET/HOST, OPT_LEVEL,
  DEBUG, OUT_DIR, CARGO_ENCODED_RUSTFLAGS, CRATE_CC_NO_DEFAULTS ...). No network; no file write outside OUT_DIR (cc's
  objects and archive); spawns only the C++ compiler and archiver (cc's probes included), via jobserver ("parallel").
  Both CUSTOM_ vars are operator-environment routes (memory: Low; the runner should leave them unset).
- The C++ compiled: libFuzzer from LLVM, all 26 .cpp headed "Apache-2.0 WITH LLVM-exception" (grep: 26/26); the crate's
  license field says NCSA, the old LLVM licence (a metadata mismatch, both permissive: info, publish=false spike).
  libFuzzer run-time behaviour: writes crash-/leak-/timeout- artifacts and corpus files; spawns `/bin/sh` commands
  (ExecuteCommand via system() on Linux) only for -fork, -jobs/-workers, -merge with a control file, -minimize_crash and
  the data-flow trace (FuzzerDriver.cpp:245,:401,:450,:470; FuzzerFork.cpp:275,:306; FuzzerMerge.cpp:510;
  FuzzerDataFlowTrace.cpp:273,:281); FuzzerInterceptors.cpp (Linux) wraps memcmp/strncmp etc. via dlsym(RTLD_NEXT).
  All standard cargo-fuzz behaviour; flag-driven, operator-controlled.
- fuzz_target! (src/lib.rs:226-373, read whole). Every form ends in one of two arms (:247-297 for &[u8], :311-372 for
  an Arbitrary type), each `const _: () = { ... }` putting INTO THE CALLER'S CRATE:
  1. `#[no_mangle] pub extern "C" fn LLVMFuzzerInitialize(_argc: *const isize, _argv: *const *const *const u8) -> isize`
     (:251-258 / :315-322): calls libfuzzer_sys::initialize (sets a panic hook that aborts, :82-97), runs `init`.
  2. `#[no_mangle] pub extern "C" fn rust_fuzzer_test_input(bytes: &[u8]) -> i32` (:260-278 / :324-365): if env
     RUST_LIBFUZZER_DEBUG_PATH is set (read once, :76-79) it creates that file and writes the input's Debug form
     (`cargo fuzz fmt`'s route), else runs the body; Arbitrary arm: size_hint early exit, arbitrary_take_rest.
  3. `#[inline(never)] fn __libfuzzer_sys_run(...)` holding the body (:292-295 / :367-370): plain safe fn.
  NO `unsafe` block, NO `unsafe fn`, NO `extern` block, NO export_name/link_section/#[used] in fuzz_target!'s
  expansion. The unsafe stays in libfuzzer-sys's own crate: `extern "C" { fn rust_fuzzer_test_input(&[u8]) -> i32;
  fn LLVMFuzzerMutate(..) }` (:48-54, improper_ctypes allowed: both sides Rust, same compiler), `#[export_name =
  "LLVMFuzzerTestOneInput"] pub unsafe fn test_input_wrap` (:58-73: from_raw_parts on libFuzzer's buffer, inside
  catch_unwind, abort on panic), `fuzzer_mutate`'s unsafe call (:601-607, asserts around it).
- FINDING SEC-U5D-1 (Medium, U5-Q7 term 3's named item): fuzz_target! puts TWO `#[no_mangle]` `pub extern "C"` fns
  per target (LLVMFuzzerInitialize, rust_fuzzer_test_input) into npt-fuzz. rustc's unsafe_code lint counts
  `no_mangle` as unsafe code, but a lint raised in another crate's macro_rules expansion is dropped unless the lint is
  report_in_external_macro (unsafe_code is not), so fuzz/Cargo.toml's forbid is silent there (the bare `#[no_mangle]`
  is also accepted in an edition 2024 crate because the tokens carry libfuzzer-sys's 2018 edition). Per U5-Q7 term 3
  a fifth exception comes to the advisor before Block 3. Exploit sketch: none against the game (fuzz binaries never
  ship); the risk is the gate's claim ("unsafe_code is forbid" for the fuzz crate) being wider than the truth.
- FINDING SEC-U5D-3 (Low): libfuzzer-sys's other macros, fuzz_mutator! (:520-567) and fuzz_crossover! (:706-752),
  expand `#[export_name] pub unsafe fn` with from_raw_parts(_mut) on raw pointers INTO THE CALLER'S CRATE, also silent
  under the forbid. Not needed by U5's targets. Fix: U5 uses fuzz_target! only; check.mjs's fuzz/ scan (U5-Q7 term 1)
  refuses `fuzz_mutator!`/`fuzz_crossover!` (and any other libfuzzer_sys macro) by name.
