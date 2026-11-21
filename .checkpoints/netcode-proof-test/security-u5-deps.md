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
