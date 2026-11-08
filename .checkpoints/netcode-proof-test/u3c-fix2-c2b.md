# U3c last fix round, part C2b: check.mjs, the lint tables, npt-tcpinfo nits, provenance()

Agent: workflow fix agent C2b, worktree .claude/worktrees/wf_ac5e4575-9da-2 at b2e879b (spike branch
claude/netcode-proof-test-plan-hcky5g). Inputs: u3c-fix-review-result.txt (C2R-1..10, REG-2, REG-3 with verdicts),
u3c-fix-review-c2.md, advisor-u3c-plan.md "Side rulings: U3c's last fix round" (binding), u3c-plan.md rev 2.
Rules: cargo +1.99.1 with CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/u3v3-model-target only; no iptables/tc,
no #[ignore]d evidence tests; commit nothing; no new public symbol; npt-net untouched (C1b owns crates/net, SERIES.md).

## Plan
1. Backstop trial first (it decides the comments): workspace `unsafe_code = "forbid"`; npt-tcpinfo and npt-client-wasm
   get complete own [lints.rust] + [lints.clippy] tables (tcpinfo unsafe_code "deny", client-wasm "allow"); every
   other member keeps `workspace = true`. Run clippy (both builds, host + wasm32) and test (both builds). Watch for
   E0453 from a dependency macro's allow(unsafe_code) and from npt-net's `#![deny(unsafe_code)]` (crates/net is not
   mine to edit). Prove the backstop in a scratch copy: a planted `#[allow(unsafe_code)]` in a leaf crate -> E0453.
   Keep only if check.mjs passes 9/9 on both builds; otherwise drop and record why.
2. check.mjs lint tables: one mask-based TOML lint-table reader (headers and keys from tomlScan's mask, values from
   `clean`; a quoted key in a lint table fails closed). workspaceLintProblems requires "forbid" if the backstop lands;
   a new check: each exception's [lints.rust]/[lints.clippy] equal the workspace's except unsafe_code (every key),
   with its own unsafe_code level; every other member inherits (existing rule).
3. (b) cheap refusals in cargoConfigProblems, each with a test, none refusing this container's env
   (CARGO_HTTP_CAINFO, RUSTUP_HOME, RUST_BACKTRACE): C2R-8 RUSTC, CARGO_BUILD_RUSTC, RUSTC_WORKSPACE_WRAPPER,
   CARGO_BUILD_RUSTC_WORKSPACE_WRAPPER, config build.rustc / build.rustc-workspace-wrapper (RUSTC_WRAPPER and
   build.rustc-wrapper stay allowed: sccache); C2R-4 a config `include` key; C2R-7 a config `runner` key and
   CARGO_TARGET_<triple>_RUNNER.
4. (a) C2R-5: export depsStep(here, env, metadata) used as STEPS.deps itself (STEPS exported); an end-to-end test on a
   fixture folder with one plant per check (dependency, workspace lint, lint tables, layout, link, cargo config,
   unsafe), asserting STEPS.deps === depsStep and each check's message present; clean fixture passes.
5. (a) C2R-6/REG-2: rewrite check.mjs comments, DEPS_CHECKED and the workspace Cargo.toml lint comment to what is
   checked; one line "the scan guards against accidental relaxation; review is the wall against a deliberate one";
   (d) known limits named there.
6. (c) C2R-9: the MPTCP test reports a skip through an unbuffered stderr line `SKIPPED <test>: <why>` (bypasses
   libtest's capture), with a unit test of the skip path on a failing socket constructor; check.mjs's test summary
   counts SKIPPED lines (a check.test.mjs test of the counter).
7. (c) C2R-10: uncommitted() drops only untracked (`??`) own outputs; a committed Python test of provenance() in a
   scratch git repo (new own log clean; edited committed log reported; source file and stray .bak reported), run by
   check.test.mjs (self step).
8. REG-3: srtt_us doc states the cost (3 extra SOL_SOCKET getsockopt calls, 5 syscalls... per call; the per-tick gate
   calls it once per connection; no cache).
9. Bites: revert each fix in a scratch copy (or in place with restore) and record the failing test.
10. Final: check.mjs 9/9 twice; clippy -D warnings with and without features; cargo fmt --check.

## Progress
- Item 1 (backstop trial): workspace forbid + own [lints.rust]/[lints.clippy] in tcpinfo (deny) and client-wasm
  (allow). check.mjs fmt, clippy (both builds), clippy-wasm32 PASS; test PASS (measured 251, bites 265, 0 failed);
  deps FAIL only on the old manifest rule (expected, item 2). No dependency macro tripped E0453 (tokio, futures-util,
  serde expansions all build). Proof, scratch copy scratchpad/c2b/netcode-proof: planted `#[allow(unsafe_code)]` +
  unsafe block in npt-link -> E0453 at crates/link/src/lib.rs:329 (rustc -v: --forbid=unsafe_code); attack A
  (shebang + macro-built allow) in npt-room -> E0453 at evil.rs:11; attack D (renamed include! of a client-wasm file
  holding an allow) -> E0453 at crates/room/src/../../client-wasm/src/evil.rs:1, while `check.mjs --only deps` sees
  neither A nor D. Control (same copy, workspace back to deny, -v shows --deny=unsafe_code): npt-link and npt-room
  compile, exit 0. Backstop kept pending the final 9/9.
- Items 2-8 written: check.mjs (lintTables on tomlScan's mask; workspaceLintProblems requires forbid;
  exceptionLintProblems; cargoConfigProblems + COMPILER_ENV/KEYS, include, runner; cargoConfigFiles(start, env);
  cargoCounts counts SKIPPED lines; depsStep exported and STEPS.deps === depsStep; DEPS_CHECKED + known-limits
  comment; lexRust/pathProblems known limits), Cargo.toml lint comment, both exception manifests' comments,
  tcpinfo lib.rs doc, linux.rs srtt_us Cost paragraph, tests.rs (report_skip, mptcp_case, the no-MPTCP unit test),
  siocoutq-root-check.py uncommitted() (drops only untracked own logs) + test_siocoutq_root_check.py (5 tests).
  self 35/35, deps PASS. Skip proof (scratch, socket() forced to fail): cargo test output carries
  "SKIPPED an_mptcp_socket_is_not_tcp_for_either_reading: ..." and cargoCounts reads "10 tests passed, 0 failed,
  1 reported a skip (an_mptcp_socket_is_not_tcp_for_either_reading)" (scratchpad/c2b/skip-run.txt).
- Item 9 (bites): scratchpad/c2b/bites.mjs on the scratch copy (files synced from the worktree, each mutation
  restored): 20 check.mjs mutations and 2 Python mutations, all caught (output scratchpad/c2b/bites-out.txt);
  control passes. Rust: report_skip reverted to eprintln -> tests::a_kernel_without_mptcp_reports_the_mptcp_case_skipped
  FAILED; restored.
