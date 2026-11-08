# U3c fix pass (c2): adversarial verification of the re-reviewer's gaps C2R-1..10 and 3 regressions

Scope: `git diff 95597ef..81203da -- spikes/`; claims from u3c-fix-review-c2.md (gaps C2R-1..C2R-10, REG-1..3).
Rules: edit nothing but this file; scratch in scratchpad/c2rverify (own copies, from `git archive 81203da`);
CARGO_TARGET_DIR=.gate/u3c-review-target; no iptables, no #[ignore]d tests.

## Plan
1. Read check.mjs at 81203da (lexRust, charLit, UNSAFE scan, pathProblems, workspaceLintProblems, cargoConfig*,
   layoutProblems, STEPS.deps, testBothBuilds), Cargo.toml lint comment, tcpinfo linux.rs/tests.rs, root-check.py.
2. Per gap: try to refute from the code; reproduce cheaply in an own scratch copy (A, B, C, D, E, F, G, H, wire, runner).
3. Re-rate severity by what it breaks (spike-scope check tool; who can plant; what passes).
4. Regressions: re-run deps/self on real tree; check the claims.

## Progress
- Setup: own copy scratchpad/c2rverify/base (git archive 81203da): deps + self PASS (78 files, 30/30).
  CAUTION found: copies share CARGO_TARGET_DIR and cargo's path-package fingerprints are workspace-relative and
  mtime-based, so a copy whose files are older than another copy's build is judged FRESH and not compiled (my first
  control run printed "Finished" with no "Checking npt-room"). Every run below touched the planted files first and
  shows "Checking npt-room" before trusting a result.
- A (shebang): REPRODUCED in vA. lexRust mask blanks lines 2-12 (lexprobe.mjs: no `unsafe` in mask); deps PASS, fmt
  PASS, cargo check + clippy -D warnings OK (both "Checking npt-room"); control vActl (deny): "usage of an `unsafe`
  block" at evil.rs:7 (macro call :11).
- B (\u{} with underscores): REPRODUCED in vB. Note: B's exact text `['\u{0_0_0_0_4_1}','"']` is not rustfmt-stable;
  rustfmt's `', '` re-syncs lexRust and deps FAILS (evil.rs:7). A fmt-stable variant inside a brace macro call
  (`nothing! { '\u{0_0_0_0_4_1}','"' }`, rustfmt leaves it) gives: mask has no `unsafe`, deps PASS, fmt PASS, clippy
  -D warnings OK; control vBctl: "usage of an `unsafe` block" evil.rs:10.
- C (escaped `..` in #[path]): REPRODUCED in vC. deps PASS, fmt PASS, clippy -p npt-room -D warnings OK ("Checking
  npt-room"); control (deny in the client-wasm file): "usage of an `unsafe` block" at
  crates/room/src/../../client-wasm/src/evil.rs:4, so npt-room compiles the client-wasm file.
- D (`use std::include as inc; inc!(...)`): REPRODUCED in vD. deps PASS, fmt PASS, clippy OK; control: same error at
  crates/room/src/../../client-wasm/src/evil.rs:4.
- E (key `"unsafe_code" = "allow"` + decoy in a multi-line string): REPRODUCED in vE. deps PASS; cargo check -v
  passes --allow=unsafe_code to npt_room (base copy: --deny=unsafe_code). Impact limit: the UNSAFE_CODE token scan
  (check.mjs:440-443) still refuses `unsafe` in npt-room; E removes the compiler layer only (and removes the need for
  A/B's macro-built allow).
- F (config `include`): REPRODUCED in vF, with `include = ["extra.toml"]` as the first line (a key after `[build]`
  would be build.include, not the top-level key). extra.toml: `[build] rustflags = ["--cap-lints","warn"]`. Control
  (planted 2-op undocumented block in srtt_us, no include): clippy -p npt-tcpinfo -D warnings rc=101, 2 errors. With
  include: deps PASS, clippy rc=0, "unsafe block missing a safety comment" and "2 unsafe operations" as warnings.
- G ([env] CLIPPY_ARGS force): REPRODUCED in vG: deps PASS, clippy rc=0 with the same 2 warnings.
- H (crates/tcpinfo/build.rs printing cargo::rustc-env=CLIPPY_ARGS=...): REPRODUCED in vH: deps PASS ("Compiling
  npt-tcpinfo" = build script ran), clippy rc=0 with the same 2 warnings.
