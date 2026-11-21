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

### B2 (item 2: libfuzzer-sys 0.4.13) DONE
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
- Sources for SEC-U5D-1 (read 2026-11-21): (1) rustc book, allowed-by-default lints, unsafe-code: "The `unsafe_code`
  lint catches usage of `unsafe` code and other potentially unsound constructs like `no_mangle`, `export_name`, and
  `link_section`." https://doc.rust-lang.org/rustc/lints/listing/allowed-by-default.html (high). (2) rustc source,
  rustc_lint_defs/src/builtin.rs:5671-5708 (nightly docs): `pub UNSAFE_CODE, Allow, "...", @eval_always = true`, no
  report_in_external_macro. https://doc.rust-lang.org/nightly/nightly-rustc/src/rustc_lint_defs/builtin.rs.html
  (high). (3) rustc_middle/src/lint.rs:441-476 (emit_lint_base_impl): a lint whose primary span is in an external
  macro returns without emitting unless future-incompatible (non-edition), report_in_external_macro or find_attr; the
  level (deny/forbid) is not consulted. https://doc.rust-lang.org/nightly/nightly-rustc/src/rustc_middle/lint.rs.html
  (high). (4) Lint struct doc: report_in_external_macro "`true` if this lint is reported even inside expansions of
  external macros." https://doc.rust-lang.org/nightly/nightly-rustc/rustc_lint_defs/struct.Lint.html (high).
  That the no_mangle check reports at the attribute's span (inside the expansion) and that the 2018-edition tokens pass
  edition 2024's unsafe-attribute rule: medium confidence (reasoned from rustc's span rules, not run: no build allowed
  here); Block 3's `cargo check` of a target shows both.
- libfuzzer-sys's own dependencies: arbitrary 1.5.0 (B3) and the build-side cc 1.11.1, jobserver 0.1.35, libc, shlex,
  find-msvc-tools: all already in the workspace lock at the same checksums (cc already builds ring's C here).
- Verdict item 2: CLEARED WITH CONDITIONS: CUSTOM_LIBFUZZER_PATH / CUSTOM_LIBFUZZER_STD_CXX unset for every fuzz
  build; RUST_LIBFUZZER_DEBUG_PATH and libFuzzer's artifact/corpus output kept under .gate/ or the fuzz crate's
  gitignored dirs (never the public notes folder); fuzz_target! only (SEC-U5D-3); SEC-U5D-1 to the advisor before Block 3.

### B3 (item 3: arbitrary 1.5.0) DONE
- Manifest: edition 2021, rust-version 1.71.0, license "MIT OR Apache-2.0", `build = false` (no build script; no
  build.rs in the package), one optional dependency derive_arbitrary ~1.5.0 behind feature `derive` (OFF: not in the
  fuzz lock, and libfuzzer-sys's `arbitrary-derive` feature is not requested), dev-dep exhaustigen (not resolved). So
  no proc-macro runs at compile time.
- unsafe: 5 sites, all inside its own crate with SAFETY comments: CString::from_vec_unchecked after removing NULs
  (src/foreign/alloc/ffi/c_str.rs:16), str::from_utf8_unchecked on a valid_up_to() prefix (src/foreign/core/str.rs:25),
  array building over MaybeUninit with a drop guard (src/foreign/core/array.rs:32, :53, :59). No #[macro_export], no
  no_mangle/export_name/extern "C"/link_section, no std::fs/std::env/std::process; std::net only for Arbitrary impls of
  address types (src/foreign/std/net.rs:3), no socket use. Nothing expands into the caller's crate.
- Verdict item 3: CLEARED.

### B4 (item 4: the tools) DONE
- Integrity: each .crate in the registry cache hashes (sha256sum) to the crates.io index's cksum (index cache parsed
  by scratchpad idx.py), none yanked: cargo-fuzz 0.13.2 5acfd019...00e2, cargo-deny 0.22.0 8372c38e...1446,
  cargo-audit 0.23.0 956f85ae...6446 (also libfuzzer-sys a9fd2f41..., arbitrary 3bc62ac9..., not yanked). All five
  are the newest version in today's index cache (15:09 UTC).
- cargo-fuzz 0.13.2 (MIT OR Apache-2.0; edition 2021; `build = false`; vcs 984c861c8dfe...). Published Cargo.lock: v4,
  101 packages, all crates.io with checksums, 10 also in the workspace lock (same checksum), 91 new; no cc/cmake/-sys
  crate: no native C at install; build scripts are the rustc-probe kind (proc-macro2, serde/serde_core/serde_json,
  libc, rustix, anyhow, thiserror, num-traits/autocfg, ...); proc macros clap_derive, serde_derive, thiserror-impl.
  No HTTP client in the graph: no network at run time. Its own source has no unsafe. Run time (src/project.rs): spawns
  `cargo build/run --manifest-path fuzz/Cargo.toml --target <host> --release --config profile.release.debug=...`
  (:137-175) with RUSTFLAGS = its sancov flags + `--cfg fuzzing` + (if not release) -Cdebug-assertions, then the
  caller's RUSTFLAGS appended (:176-308); `rustc --print sysroot` (:1097-1098), `cargo metadata --no-deps` (:1142),
  the target binaries, llvm-profdata/llvm-cov for coverage. Default sanitizer = address (src/options.rs:82-84:
  -Zsanitizer=address on nightly; runtime ships in rust-std). `cargo fuzz init/add` write templates and APPEND a
  [[bin]] to fuzz/Cargo.toml (project.rs:120-134).
- FINDING SEC-U5D-4 (Medium): cargo-fuzz 0.13.2 never passes --locked (project.rs:137-175: no locked option in
  BuildOptions, src/options.rs:55-135), and cargo has no config or env key for --locked (cargo config reference: only
  net.offline / CARGO_NET_OFFLINE, which is --offline; read 2026-11-21, https://doc.rust-lang.org/cargo/reference/config.html,
  high). So fuzz/Cargo.toml:2-3's "with --locked" does not hold for `cargo fuzz build/run`: after any fuzz manifest
  edit, cargo re-resolves fuzz/Cargo.lock silently (offline from the registry cache under the runner, online
  outside it) and builds it before any read; DEP-HOLD's script catches the lock only at push. Exploit sketch: a
  manifest edit (or `cargo fuzz add`) during Block 3+ builds an unread dependency into the fuzz binaries. Fix: the
  runner, before every cargo-fuzz call, asserts fuzz/Cargo.toml's and fuzz/Cargo.lock's sha256 equal
  dependency-check.md's lines and that `cargo +1.99.1 tree --locked --offline --manifest-path fuzz/Cargo.toml` exits 0,
  and re-hashes the lock after; or builds the targets with plain `cargo +nightly-2026-11-01 build --locked` and
  cargo-fuzz's flags. Never `cargo fuzz init` / `cargo fuzz add` (they write the manifest).
- Also for cargo-fuzz: never --build-std / --careful / --sanitizer=memory (they pass -Zbuild-std: std's own crates.io
  dependencies, outside both locks and this read, plus rust-src, which the minimal profile lacks); --target-dir under
  .gate/ or the shared target (default would be fuzz/target inside the repo); corpus and artifacts outside the public
  notes folder. Info: cargo-fuzz's `--cfg fuzzing` reaches every crate; in the fuzz graph quinn-proto 0.11.19 and
  tokio 1.54.1 hold cfg(fuzzing) code, the spike's crates none (grep: 0); not on the listed targets' paths
  (test-engineer's call; `--no-cfg-fuzzing` exists).
- Verdict cargo-fuzz 0.13.2: CLEARED WITH CONDITIONS (SEC-U5D-4's runner check; the flags above).
- cargo-deny 0.22.0 (MIT OR Apache-2.0; edition 2024, rust-version 1.95.0; `build = false`; bin src/cargo-deny/main.rs;
  scripts/ not built). Published Cargo.lock: v4, 188 packages, all crates.io with checksums, 88 in the workspace lock
  (same checksum), 100 new. Native at install: ring 0.17.14 (a direct dependency: C + asm via cc 1.11.2; the same ring
  version the workspace locks) and zstd-sys 2.1.3+zstd.1.5.7 (via spdx's detection features -> zstd: compiles zstd C
  with cc, may probe pkg-config). ureq/rustls/webpki-roots are dev-dependencies only (not built by install); tame-index
  default-features off (no HTTP). Own unsafe: from_utf8_unchecked and an mmap (src/bans/*.rs, src/advisories/helpers/
  db.rs:332, src/lib.rs:76): its own crate, a tool. Run time: spawns `cargo metadata` (src/cargo-deny/common.rs:449)
  and the `git` CLI for the advisory DB (src/git.rs:69,:138,:165): `git clone --depth=1 --branch <b>` of
  https://github.com/RustSec/advisory-db (src/advisories/helpers/db.rs:11) or, if present, `git -C <db> reset --hard`
  + `fetch --depth=1` + `reset --hard FETCH_HEAD` in the DB folder; `--disable-fetch` skips it. Our deny.toml sets no
  db-path/db-urls (defaults under CARGO_HOME). Condition: never point db-path at a folder in the repository or a
  worktree (the tool runs `git reset --hard` there).
- Verdict cargo-deny 0.22.0: CLEARED (for Block 8), with the db-path condition.
- cargo-audit 0.23.0 (Apache-2.0 OR MIT; edition 2024, rust-version 1.88; `build = false`; default feature
  binary-scanning: object, rustc-demangle, syn 3). Published Cargo.lock: v4, 363 packages, all crates.io with
  checksums, 119 in the workspace lock (same checksum), 244 new: the largest surface here. Native at install:
  aws-lc-sys 0.45.0 (AWS-LC C + asm; by aws-lc-rs 1.18.1 <- rustls 0.23.46's default provider, quinn-proto 0.11.18,
  rustls-webpki 0.103.16; its builder uses cc on x86_64 Linux, cmake only as fallback, pregenerated bindings; probes
  `nasm -version`, `bindgen --version`; docs.rs source of builder/main.rs read 2026-11-21, helper modules not read:
  medium) and ring 0.17.14. Network graph: rustsec 0.34.0 -> gix 0.90 (git over HTTPS via reqwest 0.13.5 + rustls +
  rustls-platform-verifier) and tame-index 0.26.3 (crates.io sparse index via reqwest); quinn 0.11.12 in the graph
  through reqwest. Run time: fetches the advisory DB by git (default on; `--no-fetch`, src/commands/audit.rs:118-124;
  `--db <path>`, :88; `--url`, :145; staleness check unless `--stale`, :126-128) and, for the yanked check, updates
  the crates.io index (src/auditor.rs:77-96; `--no-yanked` skips, :114-116). No secrets sent (a User-Agent naming the
  tool). Condition: install only for Block 8 (condition 3 as ruled), one install at a time with its build directory
  under .gate/ or removed after (N4: AWS-LC's build is the largest; same disk as the repo).
- Verdict cargo-audit 0.23.0: CLEARED (for Block 8), pending B6's advisory check of its lock (aws-lc-sys 0.45.0,
  rustls-webpki 0.103.16, quinn-proto 0.11.18 are older than or equal to the workspace's).

### B5 (item 5: nightly-2026-11-01, minimal profile) DONE
- What it installs: rustup's minimal profile = rustc, rust-std, cargo ("as few components as possible to get a working
  compiler (`rustc`, `rust-std`, and `cargo`)": https://rust-lang.github.io/rustup/concepts/profiles.html, read
  2026-11-21, high). For the host x86_64-unknown-linux-gnu: rustc 1.101.0-nightly (f9ceb4a4d 2026-10-31), its cargo,
  rust-std (which carries the ASan runtime cargo-fuzz's default sanitizer links). No clippy, rustfmt, rust-src or
  wasm32 target (fmt and clippy of the fuzz crate stay on stable 1.99.1, U5-Q7 term 2). Date mapping recorded at U0:
  u0-probes.md:9.
- Trust: the official distribution (static.rust-lang.org) over HTTPS, as 1.99.1 and stable already on this machine
  (rustup 1.29.3). rustup "performs all downloads over HTTPS, but does not yet validate signatures of downloads"
  (https://rust-lang.github.io/rustup/security.html, read 2026-11-21, high); it checks each component against the
  channel manifest's sha256 (rustup's behaviour, not stated on that page: medium). Accepted risk, the same as every
  toolchain the spike already uses (Low, no new finding).
- Footprint: ~/.rustup, the same disk as the repository (N4: 9,309,110,272 B free at 15:2x UTC by `df -B1`).
- Conditions: `rustup toolchain install nightly-2026-11-01 --profile minimal` exactly; no `rustup default` or
  override change, rust-toolchain.toml (the repo's and the spike's) untouched; used only as `+nightly-2026-11-01` for
  cargo-fuzz's builds and runs.
- Verdict item 5: CLEARED WITH CONDITIONS (as above).

### B6 (item 6: RustSec advisories) DONE
- Source: RustSec advisory-db, shallow clone into the scratchpad (data only), HEAD 55e1c3dfbd834237a05bbbf7b50ba3b5be27b662
  (2026-11-21T10:28:03+01:00), 1,028 crate folders, 1,536 crate advisories; https://github.com/rustsec/advisory-db
  (high: the database cargo-audit and cargo-deny read). Matched by scratchpad advmatch.py (RustSec's rule: affected =
  matches no `patched` and no `unaffected` requirement; withdrawn skipped); spot-checked against 5 advisories by hand
  (rustls-webpki RUSTSEC-2026-0382, aws-lc-sys RUSTSEC-2026-0048, zstd-sys RUSTSEC-2026-0410, quinn-proto
  RUSTSEC-2026-0343, pin-utils RUSTSEC-2026-0441): matcher agrees.
- The five crates themselves: no advisory folder for libfuzzer-sys, arbitrary, cargo-fuzz, cargo-deny or cargo-audit
  (rustsec's only advisory, RUSTSEC-2020-0051, is not 0.34's). The rust/ folder holds no 2026 toolchain advisory.
- fuzz/Cargo.lock (130 packages): 0 matches. Workspace Cargo.lock (243): 1, pin-utils 0.1.1 RUSTSEC-2026-0441
  (unmaintained, 2026-10-01), through loom 0.7.2 (a cfg(loom) graph edge); already on main, not part of this change,
  not in the fuzz lock. Heads-up for Block 8: deny.toml's `ignore = []` and `[graph] all-features = true` will report it.
- Tool locks (what `cargo install --locked` builds):
  - cargo-fuzz (101): anyhow 1.0.102 RUSTSEC-2026-0190 (unsound: Error::downcast_mut after context; patched >=1.0.103);
    crossbeam-epoch 0.9.18 RUSTSEC-2026-0204 (fmt::Pointer of a null Atomic/Shared derefs it; patched >=0.9.20);
    rustix 1.1.4 RUSTSEC-2026-0486 (unsound: xattr functions with a zero-length buffer; patched >=1.1.6).
  - cargo-deny (188): 0 matches.
  - cargo-audit (363): heapless 0.8.0 RUSTSEC-2026-0574..0582 (9 unsound, 2026-11-18, via faster-hex 0.10.1 <- gix;
    patched >=0.9.4); quinn-proto 0.11.18 RUSTSEC-2026-0523 (MAX_STREAM_DATA opens streams past the limit: server
    side DoS) and RUSTSEC-2026-0524 (long Retry token panics/exhausts a QUIC client), patched >=0.11.19; quinn-udp
    0.5.15 RUSTSEC-2026-0538 (receive spin on Apple/BSD/illumos targets only, not Linux); rustix 1.1.5
    RUSTSEC-2026-0486.
- FINDING SEC-U5D-5 (Low): the tools' published locks carry these advisories, and `--locked` installs exactly them
  (no newer release exists today to move to; installing without --locked would build unread code instead). None is
  reachable in our use: the tools run locally on our own inputs; anyhow's downcast_mut, crossbeam-epoch's Pointer fmt,
  rustix's xattr and heapless's listed APIs are not the tools' paths (not traced call by call: medium); quinn-udp's bug
  is not on Linux; quinn-proto is only reached through reqwest's HTTP/3, which reqwest enables only by its `http3`
  feature, and http3 also pulls h3 and h3-quinn, which are absent from cargo-audit's lock (docs.rs reqwest 0.13.5
  feature list, read 2026-11-21: "http3: dep:h3, dep:h3-quinn, dep:quinn, rustls"; why quinn is in that lock without
  h3 I could not settle without a download: medium). Accepted for developer tools that never ship; cargo-audit's own
  HTTPS goes to github.com and index.crates.io only.
- Not checked: GitHub Security Advisories / OSV beyond RustSec; advisories against the C sources (libFuzzer from LLVM,
  AWS-LC, zstd, ring's BoringSSL-derived code) that are not filed in RustSec; the tool locks' code beyond the native
  builds and the packages named above (91 + 100 + 244 new packages, read only by lock entry, checksum and advisory).

### B7 (answer) DONE
- Item 1, the 8 manifests and locks: CLEARED as hashed (root comment acceptable as final; SEC-U5D-2 Low, follow-up).
- Item 2, libfuzzer-sys 0.4.13: CLEARED WITH CONDITIONS (CUSTOM_LIBFUZZER_* unset; outputs kept out of the notes
  folder; fuzz_target! only, SEC-U5D-3). SEC-U5D-1 (Medium): fuzz_target! adds two #[no_mangle] extern "C" fns per
  target past the silent forbid: a fifth exception to the advisor before Block 3 (U5-Q7 term 3). Recommendation for
  his ruling: record it as a dependency-macro exception (no #[allow] is writable inside the macro), with the invariant
  "exactly LLVMFuzzerInitialize and rust_fuzzer_test_input per bin, from libfuzzer-sys 0.4.13, nothing else unsafe",
  and as its test check.mjs pinning libfuzzer-sys's fuzz-lock entry (0.4.13, checksum a9fd2f41...) and refusing other
  libfuzzer_sys macros, so a new expansion cannot arrive unread.
- Item 3, arbitrary 1.5.0: CLEARED.
- Item 4: cargo-fuzz 0.13.2 CLEARED WITH CONDITIONS (SEC-U5D-4 Medium: the runner's lock check around every cargo-fuzz
  call, or plain cargo --locked; no init/add; no build-std/careful/memory; target dir and outputs outside the repo's
  tracked tree); cargo-deny 0.22.0 CLEARED for Block 8 (db-path never in the repo); cargo-audit 0.23.0 CLEARED for
  Block 8 (SEC-U5D-5 Low accepted; DB fetch a networked step; build dir under .gate/ or removed, N4).
- Item 5, the nightly: CLEARED WITH CONDITIONS (minimal profile exactly; no default/override change).
- Item 6: no advisory on the five crates or the fuzz lock; tool-lock advisories as SEC-U5D-5; limits listed in B6.
- Conditions on the save: none that change content. On the installs: `cargo install --locked --version 0.13.2
  cargo-fuzz` (exact, the read's lock), then `cargo fuzz --version` = 0.13.2; deny and audit likewise at Block 8 only.
- dependency-check.md lines (read and cleared, paths from the repository root as the lane will hold them):
  3bf4d76008175aa7e779d2c1f504b09c5979a5e0e49ddcded8380b7045383b04  spikes/netcode-proof/Cargo.toml
  31c2c7ea9fdf04f78391b0e6d90165f14b74483c87f235c5daf4ad5f8993a7a8  spikes/netcode-proof/Cargo.lock
  fb5324f1ac53251ef03accb89a9182d62506eb98d074aed6165c00cc49efebf1  spikes/netcode-proof/crates/alloc/Cargo.toml
  b5385fcdaa5dcc2344c41177b5afec923cea9d2004512245fc6d776a2f0c329a  spikes/netcode-proof/crates/sim/Cargo.toml
  670914131fb08661aafdf8299bea21c18a15e2e7a33f754c295b1bbd61ea1b12  spikes/netcode-proof/crates/proto/Cargo.toml
  afd0952b15865881f763b1cba9d0feac715d2cf104dc4b2c6d13d3ab87f7b265  spikes/netcode-proof/crates/net/Cargo.toml
  7606244c186ebb95c486c2ef8c100740383cdcdf81f9e985b9495c1e328a369d  spikes/netcode-proof/fuzz/Cargo.toml
  3117b9305e1ebbf8ed4d802ab3689a6f4296067275e66d84fd8d7b9e66bbf1fd  spikes/netcode-proof/fuzz/Cargo.lock

DONE

### B8 (S2: the read of SEC-U5D-4's guard, u5-fuzz-guard.sh) DONE
- Read: .checkpoints/netcode-proof-test/u5-fuzz-guard.sh, sha256 2edc7b13512f053d004822a68705ba2c922fd161596d51a8726247463fb88563
  (109 lines, read whole; matches the coordinator's hash); the runner u5-run.sh (e69b0fa7..., 117 lines: cds to the
  copy's spikes/netcode-proof, exports CARGO_TARGET_DIR=$M/.gate/shared-target and CARGO_NET_OFFLINE=true, flock -x/-s);
  the ruling advisor-u5-early.md:316-338 and S1-S4 :348-355; dependency-check.md (c5b68489...). Against cargo-fuzz
  0.13.2's own option code (the .crate in the registry cache: src/options.rs, options/build.rs, options/run.rs,
  project.rs). Nothing run.
- What holds (as ruled): both fuzz hashes read from dependency-check.md by anchored exact-path lines, exactly one each,
  from the main folder's file, never the copy's (:41-44; a missing, doubled or CRLF line refuses); CUSTOM_LIBFUZZER_PATH,
  CUSTOM_LIBFUZZER_STD_CXX, RUSTFLAGS, CARGO_ENCODED_RUSTFLAGS unset (:93-96; RUSTFLAGS is right to refuse, cargo-fuzz
  appends it to its own, project.rs:304-308); CARGO_TARGET_DIR = the shared target (:97; it outranks the spike's
  .cargo/config.toml target-dir); subcommand build or run only (:74-78); hash check, then `cargo +1.99.1 tree --locked
  --offline` (cannot write the lock), then the call, then the hashes again with exit 65 and the void message
  (:98-107). Order sound: every refusal comes before the call. With no --fuzz-dir, cargo-fuzz finds the project by
  walking up from cwd to the first non-fuzz Cargo.toml ($S) and builds $S/fuzz/Cargo.toml (project.rs find_package,
  :999-1001): the file the guard hashes. Everything after `--` is libFuzzer's (clap `last(true)`, options/run.rs; for
  `build` a token after `--` is only a positional target name), so not parsing it is right for the lock.
- GUARD-1 (Medium): `--fuzz-dir` is not refused. Both subcommands flatten FuzzDirWrapper (options.rs:267-270
  `#[arg(long)] fuzz_dir`; options/build.rs, options/run.rs: `FuzzProject::new(self.fuzz_dir_wrapper.fuzz_dir)`), and
  the manifest built is `<fuzz_dir>/Cargo.toml` (project.rs:999-1001). Exploit sketch: `... build --fuzz-dir /x/fuzz`
  builds another manifest and lock (unread, re-resolved without --locked) while the guard's pre and post hashes both
  look at the unchanged $F and pass. It defeats the fix itself. Fix: refuse `--fuzz-dir` in every form.
- GUARD-2 (Medium): dependency features are not refused, and the pre-check proves the lock only for default features.
  `--features npt-net/wt-wtransport` (or `npt-net/wt-wtq`, `libfuzzer-sys/arbitrary-derive`) turns on an optional
  dependency the fuzz lock does not hold; wtransport 0.7.2 and web-transport-quinn 0.13.6 sit in the registry cache, so
  even offline the build re-resolves the lock and runs those packages' build scripts and proc macros before the post
  check voids the outputs: "build nothing that holds the change" is broken for that call. Fix: allow `--features` only
  as `bites` (refuse any value with `/`), or run the tree check with the call's own feature arguments.
- GUARD-3 (Low; fails closed today): the refused-option scan reads only long and separate forms. It misses `-c`
  (careful's short, options.rs:97 `#[arg(short, long = "careful")]`), grouped shorts (`-Oc`, `-Os memory`), attached
  values (`-smemory`, `-s=memory`, `-Z=build-std`, `-vZbuild-std`); careful and the memory sanitizer imply -Zbuild-std
  (project.rs:171-175). Today -Zbuild-std fails: no rust-src in the minimal profile and CARGO_NET_OFFLINE under the
  runner (it would be Medium the day rust-src is added). Fix for GUARD-1..3 at once: an allow-list before `--` (the
  tokens U5 uses: -O/--release, -D/--dev, -a/--debug-assertions, -v, --features bites, -s/--sanitizer address|none in
  separate or `=` form, --codegen-units N, -j/--jobs N for run, a target name among the 17 bins, corpus paths under
  .gate/), refusing every other token that begins with `-`, and every `-Z`.
- GUARD-4 (Low): the guard trusts its caller for two things it could check in one line each: CARGO_NET_OFFLINE=true
  (only the runner sets it; called directly, a re-resolve could download) and the cargo-fuzz binary's identity (all
  this reasoning is 0.13.2's; `cargo +nightly-2026-11-01 fuzz --version` = 0.13.2). Also the tree check runs on 1.99.1
  cargo while nightly cargo resolves at the build: add `cargo +nightly-2026-11-01 tree --locked --offline` too (a
  nightly-only lock rewrite now shows only after the build, voided). And a cargo config `[env]` table could set
  CUSTOM_LIBFUZZER_PATH past the env check (the spike's .cargo/config.toml holds only build.target-dir today; an
  operator route, memory: Low).
- GUARD-5 (Low): outputs. `cargo fuzz run` creates fuzz/corpus/<t> and fuzz/artifacts/<t>/ in the copy and passes
  `-artifact_prefix=` there (project.rs:1017-1038, :343-347); `git check-ignore` matches neither path, so corpus and
  crash inputs land as untracked files beside the manifest, against dependency-check.md:6 ("fuzz outputs under
  .gate/", S4). Fix: corpus positionals under .gate/u5/ and a later `-artifact_prefix=<.gate path>` after `--` (the
  empty dirs still appear), or the guard fails when files exist under $F/corpus or $F/artifacts; snapshots never take
  them.
- Plant test (:55-71): it cannot leave the manifest changed in the normal flow: clean start checked (:56), backup made
  by cp -p before the change (:57-58; a failure refuses before any change), the restore is mv on the same filesystem
  (.gate/u5 and the worktree are both on /dev/vda: a rename), then both hashes re-checked (:63), and the refusal it
  shows is the same hashes_match the real path uses (:98). GUARD-6 (Low): no trap, so a kill between :59 and :62
  (milliseconds), or a failed mv, leaves the extra byte, and the FAILED line does not name the backup file. Fix: a
  trap that moves $backup back on EXIT/INT/TERM, set right after the cp, and $backup printed in the failure line. (A
  concurrent guard call during the plant only refuses: fail safe. A check-to-call race needs a concurrent edit that is
  reverted before the post-check: Low, accepted.)
- Verdict: the guard meets the ruling's checks, order, void path and plant, but its option scan does not meet my fix
  yet: GUARD-1 lets a call build a manifest the guard never hashed, and GUARD-2 builds unread packages before the
  post-check. Both Medium: fix before the first cargo-fuzz call (the allow-list closes GUARD-1..3), new sha256 on the
  card, then a short re-read of the diff. GUARD-4..6 Low: fold in now or carry as follow-ups.

DONE

### B9 (the re-read of guard rev 2, u5-fuzz-guard.sh 768f6055...) DONE
- Read whole: .checkpoints/netcode-proof-test/u5-fuzz-guard.sh rev 2, sha256
  768f6055fbbc1b412b85cf3bae0356ae2360adb2e46e18e628cd22e6533942ae (192 lines; matches the coordinator's hash). Checked
  again against cargo-fuzz 0.13.2's source in the registry cache (src/main.rs, src/options.rs, options/build.rs,
  options/run.rs, src/project.rs) and cargo's config reference. Nothing run. Environment names seen (names only):
  CARGO_HTTP_CAINFO, RUSTUP_HOME, RUST_BACKTRACE; the only .cargo folder on the cwd's ancestry is the spike's
  ($S/.cargo/config.toml: build.target-dir only).
- Closed:
  - GUARD-1: --fuzz-dir is refused (the dash allow-list, :114-130). With no --fuzz-dir the project is
    find_package()+"fuzz" from cwd $S (project.rs:1079-1084) = the hashed $F; no cargo-fuzz environment variable picks
    the folder (its env reads are RUSTFLAGS, ASAN_OPTIONS, TSAN_OPTIONS, RUSTC, RUSTC_BOOTSTRAP only); the target dir is
    None without coverage (project.rs:352-371), so CARGO_TARGET_DIR applies. Subcommand aliases b/r (main.rs) are
    refused: only the literal build or run passes.
  - GUARD-2: features only `bites` (:123-129), and both tree checks run with the call's features (:177-182). bites
    turns on npt-proto/bites -> npt-sim/bites, plain features with no optional dependency (B1). The build therefore uses
    the cleared lock with the cleared features.
  - GUARD-3: -c, grouped shorts, attached values, -Z and --target-dir all start with a dash and are not exact
    allow-list tokens, so they are refused. Value consumption agrees with clap: the guard takes the next token after -s,
    --sanitizer and --features and refuses anything but address/none/bites, and clap rejects a dash-led value anyway.
  - GUARD-4: CARGO_NET_OFFLINE=true checked (:167); version checked (:174-175); tree on 1.99.1 and the nightly; configs
    scanned for CUSTOM_LIBFUZZER (:168-173). GUARD-5: run needs corpora and -artifact_prefix under .gate/ with no ".."
    (:145-159). GUARD-6: traps right after the cp (:76-77), cleared after the mv (:83); a signal after the mv only runs
    a failed, silent mv; the backup is named on failure (:84). Only a SIGKILL in that millisecond window is left.
- GUARD-7 (Low under the recorded threat model; the one GUARD-1-kind route left): the guard checks the environment
  and cargo configs by a deny-list, so routes that point cargo at another lock or other sources stay open, and every
  check then passes. (a) `resolver.lockfile-path` is stable since cargo 1.97, environment variable
  CARGO_RESOLVER_LOCKFILE_PATH (cargo config reference, read 2026-11-21: "Specifies the path to the lockfile to use
  when resolving dependencies", "MSRV: Requires 1.97+", high). Set, both tree checks and the build use that other lock
  while the guard hashes $F/Cargo.lock. (b) A config `paths` override or a `[source]` replacement builds other source
  code under an unchanged lock. (c) `[unstable] build-std` or CARGO_UNSTABLE_* turns on build-std without a flag (fails
  closed today). (d) CARGO_HOME points at another config and registry; RUSTC sets another compiler. The config scan
  reads 4 folders, but cargo reads every .cargo/ from cwd up to / plus $CARGO_HOME, and it looks only for
  CUSTOM_LIBFUZZER. Why Low: each route needs a deliberate environment variable or config file, and none is set on
  this machine today (memory: environment and config routes are the operator's machine). Recommended before first use
  anyway, since it is the guard's own subject and costs a few lines: an environment allow-list (refuse any CARGO_* or
  RUSTC* name except CARGO_TARGET_DIR, CARGO_NET_OFFLINE, CARGO_HTTP_CAINFO), and a config check walking every
  directory from $S to / plus ${CARGO_HOME:-$HOME/.cargo}: no config file may exist but $S/.cargo/config.toml, and that
  one must equal its known sha256 or hold only build.target-dir.
- Parsing edge cases (all Low; none builds something other than the cleared manifest and lock):
  - E1: after `--` the guard checks only -artifact_prefix and -exact_artifact_path. A token there with no leading dash
    is a libFuzzer input, and cargo-fuzz places it before the guard's validated corpora (`-- -artifact_prefix=... <args>
    <corpora>`: project.rs:343-347, :512-530), so it becomes libFuzzer's first corpus folder, the one it writes into,
    anywhere: GUARD-5 bypassed for outputs. Other libFuzzer flags write files elsewhere (-features_dir=,
    -mutation_graph_file=, -merge_control_file=; -jobs/-fork write fuzz-N.log into cwd $S) or run a named program
    (-collect_data_flow=). Fix: after `--` accept only `-name=value` tokens from a short list (-max_total_time, -runs,
    -seed, -max_len, -timeout, -rss_limit_mb, -malloc_limit_mb, -print_final_stats, -use_value_profile, plus the two
    checked path flags), and refuse any token with no leading dash.
  - E2: for build, tokens after `--` are not checked. clap makes the first one Build's positional target when none came
    before `--`, which skips the [[bin]] check, and cargo then fails on `--bin <odd name>`. Fix: refuse `--` for build.
  - E3: an empty token "" is neither a dash token nor a recorded target (the `-z` test stays true), while clap takes it
    as the target; cargo then fails on `--bin ""`. Fix: refuse empty tokens.
  - E4: the version test is a substring (`*"cargo-fuzz 0.13.2"*` also matches 0.13.20) and self-reported. Fix: exact
    equality; optionally the installed binary's sha256 on the card, compared.
  - E5 (carried): the fuzz build also compiles the path crates (alloc, client, link, net, proto, room, sim). A change
    to one of their manifests that needs no lock change (a `build =` key, say) passes `tree --locked` and gets built.
    The workspace's own runner builds have the same exposure (DEP-HOLD's push check and check.mjs back it). Cheap fix:
    check every line of dependency-check.md before the call, not only the two fuzz ones.
  - Symlinks under .gate/ named as a corpus or prefix could point back into the repository: deliberate, Low.
- Verdict: READY FOR FIRST USE. GUARD-1 and GUARD-2 are closed and no Medium or higher is open. GUARD-7 and E1-E5 are
  Low: GUARD-7 and E1 are worth folding in before the plant and first call (a few lines each, then a new sha256 and a
  diff-only re-read); otherwise they carry as follow-ups.

DONE

### B10 (the diff-only re-read of guard rev 3, u5-fuzz-guard.sh 8466a42e...) DONE
- Read: .checkpoints/netcode-proof-test/u5-fuzz-guard.sh rev 3, sha256
  8466a42e7bd87a50adba90479fb484ef280f42397037d74d56b2d6e772ac2386 (260 lines, read whole, not just the diff);
  u5-fuzz-guard-rev3.diff (234 lines; its body equals `diff -u` of git 18c7625's rev 2 (768f6055...) against rev 3:
  same sha256 64ba4ba0... after the two header lines); u5-guard-rev3-tests.txt (40 of 40 PASS: A1-A29 refusals, B1-B11
  parser cases; no positive end-to-end, since the nightly is not installed yet). Checked against cargo-fuzz 0.13.2's
  source (src/project.rs, src/main.rs, src/options*.rs), libFuzzer's FuzzerFlags.def and FuzzerDriver.cpp (libfuzzer-sys
  0.4.13 in the registry), cargo's config reference and rustup's environment-variable page. Read-only: no cargo, rustup
  or cargo-fuzz command, no environment listing. The spike's .cargo/config.toml hashes to SPIKE_CFG_SHA 9ff1aa86... in
  both the main folder and the copy; /root/.cargo holds no config; /root/.rustup/toolchains holds 1.99.1 and stable
  (the nightly to come).
- (1) `env -i` (:47-48, used at :243, :248, :250, :253). Nothing the build needs is lost:
  - cargo-fuzz's own environment reads (RUSTFLAGS, ASAN_OPTIONS, TSAN_OPTIONS, RUSTC, RUSTC_BOOTSTRAP) fall back to its
    defaults: RUSTFLAGS = its own flags only; ASAN_OPTIONS = `detect_odr_violation=0` (project.rs:304-320); `rustc`
    from PATH.
  - The nested `cargo` that cargo-fuzz spawns gets the nightly through rustup's proxy environment.
    RUSTUP_TOOLCHAIN_SOURCE is "Set by rustup to tell proxied tools how `RUSTUP_TOOLCHAIN` was determined"
    (https://rust-lang.github.io/rustup/environment-variables.html, read 2026-11-21; medium that RUSTUP_TOOLCHAIN
    itself is passed on). If it were not, the toolchain file's 1.99.1 would refuse the nightly-only sanitizer flags:
    fail closed, and the same as rev 2. rust-toolchain.toml and rustup directory overrides lose to `+toolchain`.
  - libfuzzer-sys's build script and cc: no CC/CXX/CFLAGS/AR set, so cc uses `c++` and `ar` from PATH (/usr/bin). No
    cc, c++, gcc, g++, clang, clang++, ar, ld, cargo or rustc sits in /usr/local/bin, which comes first. cargo itself
    hands build scripts OUT_DIR, TARGET and its jobserver.
  - TMPDIR unset means /tmp (same disk as before, inside the runner's 4 GB check).
  - Cosmetic losses only: RUST_BACKTRACE and an ASan symbolizer path.
  - Closed by env -i: CARGO_RESOLVER_LOCKFILE_PATH, CARGO_HOME, CARGO_UNSTABLE_*, every other CARGO_* config key,
    RUSTC/RUSTC_WRAPPER, RUSTUP_TOOLCHAIN, CC/CXX/CFLAGS, LD_PRELOAD, RUST_LIBFUZZER_DEBUG_PATH.
  - Routes left: PATH's directories, the registry's extracted sources and the installed binaries. These are the
    operator's machine (memory: Low); no new finding.
- (2) The config walk (:218-240) is exactly what cargo reads here:
  - Cargo probes the current directory and every parent, then $CARGO_HOME/config.toml, and reads `.cargo/config`
    without the extension too. Source: cargo config reference, "Hierarchical structure", read 2026-11-21, high.
  - `cd "$S"` (:218) now comes before the version check, both trees and the call, so all four read the same configs.
  - cargo-fuzz sets no current_dir on its cargo commands (project.rs: `env::current_dir` reads only, no
    `.current_dir(`), so its child cargo and the target inherit $S.
  - `pwd -P` is getcwd, which is what cargo uses.
  - The docs say nothing about --manifest-path. If cargo ever probed from the manifest's folder instead, fuzz/.cargo
    (:236-240) plus the same ancestry covers it.
  - CARGO_HOME is pinned to /root/.cargo and checked.
  - A config `include` could only come through the pinned file, which holds none.
  - Environment config keys are gone under env -i except the three set.
- (3) -detect_leaks=0|1 (:204) is safe:
  - It is an int flag (FuzzerFlags.def:164-165, default 1) that writes no file and starts no process. With 1, a leak
    becomes a leak- artifact under the checked -artifact_prefix; 0 only narrows what is found (the header records
    it, u5-brief.md:180; test-engineer's call).
  - libFuzzer matches a flag by its exact name followed by '=' (FuzzerDriver.cpp:120-126, :145-175), so no allowed
    name can reach another flag (-timeout= is not -timeout_exitcode=).
  - Bare tokens and `--` after `--` are refused (:205; tests A6, A15), so no libFuzzer input path can be slipped in.
- (4) Regressions against rev 2:
  - None in the checks. The allow-list, the target, corpus and prefix checks are unchanged. The trees now run from $S
    with the build's configs.
  - The plant test is the same logic: the refusal now counts only if fuzz/Cargo.toml is among the differing files
    (:120-122), and all listed files are checked before (:113) and after (:127). Its traps are unchanged.
  - Two new behaviours count as regressions: R3-1 and R3-2 below.
- R3-1 (Medium; functional, not a security hole): env -i also strips NPT_BITE. U5-Q6 term 2 says "The runner sets it,
  or clears it, explicitly for every run" (advisor-u5-brief.md:192-196; u5-brief.md:192-195; fuzz/Cargo.toml:38-41).
  - Effect: a bite run through the guard runs the bites build with no bite chosen. It fails visibly: the -34 run "must
    end with panics at least 1". It is still a wasted run, and the guard cannot express the brief's "sets it".
  - Fix: a guard option such as `--bite <id>`, allowed only with `run --features bites` and checked against the bite-id
    form, added as `NPT_BITE=<id>` to the env -i list for the fuzz call only and printed in the GUARD PASSED line. An
    NPT_BITE in the runner's environment without bites is refused.
  - Builds and non-bite runs do not need it (clearing NPT_BITE is right for them), so it blocks the first bite run, not
    first use.
- R3-2 (Low; availability, fails closed): the strict parser (:76-85) refuses DEP-HOLD's own `deleted  <path>` form
  (main .claude/CHECKPOINTS.md:64-66) and any listed path outside spikes/netcode-proof/ (test B11). A later release
  file holding either would stop every cargo-fuzz call.
  - Fix: accept `deleted  spikes/netcode-proof/<path>` (and require that file to be absent), and skip lines outside the
    spike instead of refusing them. Skipping is safe, since the fuzz build reads nothing outside the spike.
- R3-3 (Low; E5's remainder, carried): manifests in the fuzz graph that dependency-check.md does not list
  (crates/client, crates/room, crates/link Cargo.toml, unchanged from main today) are still unchecked. A change that
  needs no lock change passes `tree --locked`.
  - Fix: every Cargo.toml and Cargo.lock under $S that the file does not list must equal HEAD's (tracked and unchanged).
  - The workspace's own runner builds have the same exposure.
- R3-4 (Low; resource nit): the numeric libFuzzer values accept 0 and 10 digits (:202-203).
  - `-rss_limit_mb=0` and `-timeout=0` switch libFuzzer's memory and time guards off.
  - Int flags wrap past 2^31, because MyStol's long is cast to int (FuzzerDriver.cpp:129-143, :160-165): for example,
    -rss_limit_mb=4294967296 becomes 0.
  - So a run can eat the shared machine's memory (N4).
  - Fix (optional): 1 to 9 digits, and no 0 for rss_limit_mb or timeout; or leave it to the brief.
- Carried, unchanged: the cargo-fuzz binary's sha256 is recorded on the card, not compared (E4's remainder, Low).
  Symlinks under .gate/ (Low).
- Verdict: READY FOR FIRST USE, for Block 3's builds and non-bite runs. GUARD-7 and E1-E5 are closed as asked, and no
  security finding of Medium or higher is open. R3-1 (Medium, functional) must be fixed before the first bite run.
  R3-2 to R3-4 are Low: fold them in with R3-1 or carry them as follow-ups. A changed guard needs a new sha256 on the
  card and a diff-only re-read.

DONE

### B11 (the diff-only re-read of guard rev 4, u5-fuzz-guard.sh 7413834d...) DONE
- Read: .checkpoints/netcode-proof-test/u5-fuzz-guard.sh rev 4, sha256
  7413834d3a8d8d4e664631c1463cb66d5b85b0243876e082c7c90471e1d9edb4 (349 lines, read whole). u5-fuzz-guard-rev4.diff (313
  lines): its body equals `diff -u` of git d10fd11's rev 3 (8466a42e...) against rev 4 (same sha256 f42c15a1... after
  the two header lines). u5-guard-rev4-tests.txt: 58 of 58 PASS (A refusals on the real guard; B parser cases; C valid
  cases on a scratch copy stopped before the call: C1 shows "10 path crates; 5 listed; 5 held").
- Not run by me: the guard, cargo, rustup or cargo-fuzz. The git reads I made from the main folder: no partial clone
  (extensions.partialclone unset), no refs/replace, no .git/info/grafts, the copy's .git file points at
  $M/.git/worktrees/agent-a60a432d4564f4cc7, and there is no nested .git in the copy's spikes/ or spikes/netcode-proof/.
  HEAD:spikes/netcode-proof/crates/sim/Cargo.toml:4-10 inherits `version.workspace`, edition, rust-version, publish
  and `[lints] workspace = true` from the spike's root manifest.
- R3-1 to R3-4 are closed:
  - R3-1: `--bite` (:208-215) takes one id that must match `^PM-[0-9]{4}-[0-9]{1,3}/[a-z0-9][a-z0-9-]{0,63}$` (no
    space, newline or shell character; bash's `$` is end of string), only with run and `--features bites` (:244-247).
    It is never put into `pass`. NPT_BITE is added to the fuzz call's env -i alone (:335-338) and printed in the pass
    line (:337). `pass` (:187-242) is the original argument list minus `--bite <id>`, in order: every accepted branch
    appends its tokens, every other branch refuses. After `--`, a `--bite` token reaches the libFuzzer allow-list and is
    refused. Without --bite, NPT_BITE is unset, which is U5-Q6's "clears it".
  - R3-2: `deleted  <path>` lines (:94-96) are read, and hashes_match requires that spike file to be absent
    (:130-134). Lines outside the spike are skipped and counted (:97-100). A spike path must be plain with no ".."
    (:101-102). A deleted line for a fuzz file counts toward n_toml/n_lock, but the present file then has no hash line
    and is refused: fail closed (tests B11-B16).
  - R3-3: the graph check (:309-328, then :344-347 after the call).
  - R3-4: 1 to 9 digits, and -timeout and -rss_limit_mb from 1 (:263-267); 9 digits stay below 2^31, so no int wrap.
- (1) The tree parse is sound:
  - `{p}` prints a package as "name vX" plus " (source)" when the source is not crates.io. A path source prints its
    absolute directory; cargo builds path-dependency source ids from the lexically normalised joined path, and C1's
    output shows the copy's crates/* paths.
  - The regex `\(/[^)]*\)` (:314) takes only groups that start with "(/". "(proc-macro)", "(*)", a git "(https…#rev)"
    or "(registry …)" are left out; the fuzz lock holds only crates.io and path sources (B1).
  - `--prefix none` changes only the indentation, and dedup "(*)" lines still carry the path group, so `sort -u` sees
    every path crate.
  - `-e normal,build` matches what cargo-fuzz compiles (bins only, no dev targets). The default `--target` is the host,
    the same triple cargo-fuzz passes. `-e` is a display filter only, so `--locked` still checks the whole resolve.
  - Fail-closed edges: a path outside the copy (:318), a ".." (:319), a ")" inside a path (it truncates the group, the
    HEAD lookup then fails, :323), and a crate that is not in HEAD (:323). The fuzz root itself is the listed
    fuzz/Cargo.toml.
- (2) The baseline is sound for its purpose. HEAD's blob plus "HEAD is an ancestor of the pushed lane ref" means the
  manifest content was pushed, so it went through the lane's reads and DEP-HOLD's check (unless pushed by hand, which
  the rule forbids).
  - `git hash-object --no-filters` hashes the raw file without writing an object (no -w), with no clean filter. A
    CRLF-converted file would refuse: strict, fails closed.
  - `rev-parse --verify --quiet HEAD:<path>` reads only the tree (the path is root-relative, correct from cwd $S).
  - `merge-base --is-ancestor` reads commits only.
  - None of the three runs a hook, a filter, a pager (output is captured, not a tty) or fsmonitor (no index refresh).
    Aliases cannot shadow builtins. No network: no remote operation, and the repo is not a partial clone.
  - Config that is still read: the repository's (shared with every worktree) and the global/system files under HOME=/root.
    Nothing in them changes what these three commands compute, except object replacement below.
- R4-1 (Low; deliberate local routes, none present today): what git reads can still be redirected.
  - refs/replace/* objects (honoured by default; env -i also strips GIT_NO_REPLACE_OBJECTS) and a .git/info/grafts file
    change what `HEAD:<path>` and `merge-base` see.
  - A nested `.git` planted under spikes/ or spikes/netcode-proof/ would capture repository discovery from $S.
  - The remote-tracking ref is a local ref, so `git update-ref` moves it without a push.
  - Each one needs a deliberate local act (memory: Low). Fix (cheap): add GIT_NO_REPLACE_OBJECTS=1 to GE (:61), and
    check that `git rev-parse --show-toplevel` equals the copy's physical path and that `--git-common-dir` equals
    $M/.git before the graph check.
- R4-2 (Low; a future release file, not today): the spike's root manifest is an input of the fuzz build, because the
  path crates inherit from it (`version.workspace = true`, `[lints] workspace = true`, `<dep>.workspace = true`:
  HEAD:spikes/netcode-proof/crates/sim/Cargo.toml:4-10). It is not a path crate, so the graph check (:317-328) never
  holds it.
  - Today it is hashed by its dependency-check.md line. A later release file that lists only the fuzz pair would leave
    it neither hashed nor held.
  - A version or new-dependency change there would still fail `tree --locked`. A feature change on an already-locked
    dependency in `[workspace.dependencies]` would not.
  - Fix: when spikes/netcode-proof/Cargo.toml is not listed, hold it to HEAD like the unlisted path crates (and so
    anything that holds `[workspace.dependencies]` for a path crate).
- (3) Regressions against rev 3: none.
  - The same allow-list, corpus, prefix and libFuzzer checks; env -i; the config walk; the pinned spike config.
  - The plant test is unchanged (hash lines only, both forms; traps as before).
  - The new order (:305-334) still runs every check before the call (:338).
  - The 1.99.1 tree now carries `--prefix none --format '{p}' -e normal,build` (display only) and is captured; the
    nightly tree is unchanged.
  - Held manifests are re-checked after the call with exit 65 (:344-347).
  - New, availability only: the guard refuses when the copy's HEAD is not on the pushed lane ref (:312-313). It is
    expected, and fails closed.
- Out of the guard's scope, no finding: source files such as an auto-discovered build.rs in a path crate are code, not
  dependencies (check.mjs refuses build scripts at the end checks).
- Verdict: READY FOR FIRST USE, bite runs included. R3-1 to R3-4 are closed, and no finding of Medium or higher is
  open. R4-1 and R4-2 are Low: fold them in (a few lines, then a new sha256 and a diff-only re-read) or carry them as
  follow-ups.

DONE
