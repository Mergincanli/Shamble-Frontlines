# U3c fix C2: check.mjs's unsafe_code rule and npt-tcpinfo (S1, F-D, F-E, F-N, F8, F-J, S5 part)

Worktree /home/user/Shamble-Frontlines/.claude/worktrees/wf_3ea724fc-a63-2, at spike head 95597ef (fast-forwarded).
Inputs: u3c-review-result.txt (four reviews + verifier), u3c-review-{rust,netcode,security,test}.md, u3c-plan.md rev 2
([C1]-[C16]), advisor-u3c-plan.md. Not touched: crates/net, marks/SERIES.md (fix agent C1). Commit nothing.
Cargo: `cargo +1.99.1`, CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/u3v3-model-target for every command.

## Plan
1. Probes (scratchpad, plain rustc/clippy-driver to a scratch .rmeta, no target dir): which relaxings rustc 1.99.1
   accepts that the scan misses (comment in the list, macro-built, upper-case lint name, `#[path = concat!(..)]`,
   `warnings` / `clippy::restriction` groups against the denied lints).
2. S1 in check.mjs (each route planted in check.test.mjs and failing):
   - the relaxing regex also runs on the comment-blanked code (comment inside the list), case-insensitive;
   - outside the two exception crates, any `unsafe` keyword in code (comments and literals blanked) is a problem: it
     catches the unsafe code whatever relaxed the lint (macro-built allow, RUSTFLAGS, config, cap-lints);
   - in npt-tcpinfo, every `unsafe` keyword lies inside the body of srtt_us or unacked_and_unsent in linux.rs, and
     `unsafe_code` is named in code only by the two placed allows;
   - npt-tcpinfo: no relaxing of clippy::undocumented_unsafe_blocks / multiple_unsafe_ops_per_block (or their groups),
     and lib.rs keeps the crate-level deny of both;
   - rustflags: RUSTFLAGS, CARGO_ENCODED_RUSTFLAGS, CARGO_BUILD_RUSTFLAGS, CARGO_TARGET_*_RUSTFLAGS and every cargo
     config file cargo reads from the spike folder (ancestors + CARGO_HOME) carry no lint-lowering flag (-A, -W,
     --allow, --warn, --force-warn, --cap-lints), no -Z and no @argfile.
3. F-D: the root manifest's [workspace.lints.rust] unsafe_code is "deny" or "forbid" (string or { level }); a test.
4. F-E: from cargo metadata, every workspace member's target src_path (lib, bin, example, test, bench, build script)
   lies inside its own package folder; no symbolic link under crates/; tests.
5. S5 (c): PLACED_ALLOW needs a non-empty reason; every workspace member's manifest is crates/<dir>/Cargo.toml (the
   scan reads crates/ only); tests. (a), (b), (d) are U6 carries: listed below for the main session.
6. F-N: npt-tcpinfo joins the wasm32 clippy step (measured and bites builds); a bite (stub wiring broken -> the step
   fails), restored.
7. F8: srtt_us reads SO_DOMAIN / SO_TYPE / SO_PROTOCOL first, as unacked_and_unsent does (its own SAFETY'd block, in
   the allowed function); a shared safe helper decides; docs corrected; an MPTCP test (connected MPTCP socket ->
   InvalidInput from both) with a bite (check removed -> the test fails), restored.
8. F-J: siocoutq-root-check.py's provenance excludes its own outputs (siocoutq-root-check-*.txt) and lists the dirty
   paths; checked by importing the function against a planted output and a planted other file.
9. Comments that describe the checks (workspace Cargo.toml lint comment, DEPS_CHECKED, docs) say what is checked.
10. End: check.mjs 9/9 twice in a row; clippy -D warnings with and without features; cargo fmt --check.

## Progress
- Probes (scratchpad c2probe/, rustc and clippy-driver 1.99.1 to one .rmeta, --deny unsafe_code as cargo's [lints]):
  COMPILES (the lint relaxed): `allow(dead_code, /* x */ unsafe_code)`, the same with a `// x` line comment,
  macro-built `#[allow($l)]` + `lax!(unsafe_code)`, `#[allow(r#unsafe_code)]`, `#[r#allow(unsafe_code)]`,
  `allow(r#dead_code, unsafe_code)` (the old list pattern misses it), `--cap-lints warn` (even with -F unsafe_code),
  `--force-warn unsafe_code` (even with -D warnings), `#[path = $p]` built by macro_rules ($p:expr or :literal).
  Rejected: UNSAFE_CODE / Unsafe_Code (lint names are case-sensitive), `allow(warnings)` / `--allow warnings`,
  `::unsafe_code`, reason first, `#[path = concat!(..)]`, unsafe attributes and global_asm! (unsafe_code covers them).
  clippy (crate-level deny of the two lints): `allow(clippy::restriction)` and `clippy::r#undocumented_unsafe_blocks`
  relax them; `clippy::all`, `warnings`, `blanket_clippy_restriction_lints` do not. Also found (not in the review): a
  cargo alias named clippy or fmt ([alias] in a config, or CARGO_ALIAS_CLIPPY) replaces the subcommand with a warning
  (shown with `clippy = "version"`), so the clippy step would pass without linting. A crafted emoji char literal
  ('😀' is two UTF-16 units) mis-lexes in lexRust and can open a fake string that hides code from the mask.
- Baseline check.mjs on 95597ef in this worktree: all 9 steps pass (clippy 22.4 s, wasm32 4.5 s, test 116.6 s; self
  21/21).
- Progress F8 (fixed): srtt_us reads SO_DOMAIN/SO_TYPE/SO_PROTOCOL in its own SAFETY'd closure (one getsockopt per
  block, inside the allowed function) and refuses a non-TCP socket with InvalidInput; shared safe helpers int_answer
  and require_tcp (private) serve both functions; docs (srtt_us, lib.rs) corrected. Tests: UDP/Unix now InvalidInput
  for srtt_us; new an_mptcp_socket_is_not_tcp_for_either_reading (connected MPTCP socket: both refuse, "protocol 262").
  Bite: check disabled in srtt_us -> the MPTCP test fails with Ok(Some(28)) (TCP_INFO answered from the subflow, F8's
  premise shown) and the non-TCP test fails; restored -> 9/9 pass. clippy -p npt-tcpinfo --all-targets -D warnings
  clean.
- Progress S1 (fixed, check.mjs + check.test.mjs): RELAX_UNSAFE reads the list up to its first `)` and runs on the raw
  text and the mask (comment in the list, `r#` names); any `unsafe` / `global_asm` in code outside the two exception
  crates is a problem (macro-built allow, rustflags, cap-lints, config: the code still shows); in npt-tcpinfo `unsafe`
  only inside the two fn bodies, `unsafe_code` named in code only inside attributes, the two clippy lints (and
  `restriction`) named only in lib.rs's crate-root deny, which must deny both; lexRust's char literal reads one code
  point (u flag); #[path] value must be a plain string literal (macro-built #[path = $p] compiled in rustc);
  cargoConfigProblems: lint-lowering flags (-A -W --allow --warn --force-warn --cap-lints -Z @) in any *RUSTFLAGS env
  var or in a rustflags key of every cargo config cargo reads from the spike folder (ancestors + CARGO_HOME), and an
  alias named clippy or fmt (env CARGO_ALIAS_* or config key). Not added: a `cargo clippy -- -F unsafe_code` pass
  (probe: --cap-lints warn and --force-warn defeat -F; the token scan catches the code itself; a second CLIPPY_ARGS
  would re-check the shared crates on every run). Tests: 9 new tests, 30/30 pass; 25 of 25 mutations of the new rules
  caught (scratchpad c2probe/bites.py); 10 of 10 routes planted in the real tree make `check.mjs --only deps` FAIL
  (c2probe/realplant.py: comment-in-list, macro-built allow, RUSTFLAGS -A, RUSTFLAGS --cap-lints, config rustflags,
  alias clippy, tcpinfo allow(clippy::restriction), root "allow", [lib] path into tcpinfo, empty reason), tree restored
  (git status shows only this fix's files).
- Progress F-D (fixed): workspaceLintProblems: every unsafe_code line of the root Cargo.toml must be "deny"/"forbid"
  (or { level }) in [workspace.lints.rust], and one must exist; test on the real Cargo.toml + 4 relaxed levels, the
  line removed, another table, a dotted key, a multi-line-string decoy. Real plant: root "allow" -> deps FAIL.
- Progress F-E (fixed): layoutProblems from cargo metadata: every member target's src_path (lib, bin, example, test,
  bench, custom-build) inside its package folder and a .rs file; edition 2024 per target (2021 lets #[no_mangle] go
  without `unsafe`: probe compiled a macro-built allow over it); crateFiles reports every symbolic link (refused) and no
  longer skips target/ or dot folders (`mod target;` would compile one unscanned). Tests: 6 outside paths, non-.rs,
  edition 2021, temp-dir walk with links. Real plant: [lib] path = "../tcpinfo/src/linux.rs" -> deps FAIL.
- Progress S5 (c) (fixed): empty or escape-only reason refused (isPlacedAllow; 2 test forms + real plant); every member
  manifest at crates/<folder>/Cargo.toml (root package or another folder refused; test).
- Progress F-N (fixed): WASM_CRATES = LEAF + npt-tcpinfo; the clippy-wasm32 step lints its lib for wasm32 (a target
  that is not Linux, no dependency there) on the measured and bites builds (cargo -v: npt-tcpinfo checked for wasm32).
  Bite: the stub's non-Linux `pub use` renamed -> clippy-wasm32 FAIL (E0432) while host clippy -p npt-tcpinfo passes;
  restored -> PASS. check.test.mjs asserts WASM_CRATES (mutation M16 caught). Not covered (crates/net is C1's and needs
  a non-Unix host target): tcp_kernel.rs's not(unix) handle.
- Progress F-J (fixed): siocoutq-root-check.py's provenance moved into uncommitted() / provenance(): the porcelain
  status of the spike folder (untracked files listed one by one) with an `:(exclude,glob)` pathspec of OWN_OUTPUTS
  (results/u3c-kernel-check/siocoutq-root-check-*.txt), and up to five dirty paths printed. Pass rule untouched (dated
  note beside OWN_OUTPUTS). Check (scratchpad c2probe/fj_check.py, python3 -I -B, imports the script, no iptables, no
  probe): a planted output-named log is left out, a planted other file is reported, and with OWN_OUTPUTS emptied (the
  bite) the log is reported; plants removed; py_compile OK. (An import without -B left a __pycache__ next to the
  script once; removed.)
- Workspace Cargo.toml lint comment rewritten to what the deps step checks (S1's "overclaims"); srtt_us's SOL_SOCKET
  reads named there.
- One more route found and closed (S1 family): a proc macro that builds the `unsafe` keyword from strings
  (scratchpad c2probe/pm/: Ident::new(["un","safe"].concat())) makes the calling crate compile an unsafe block under
  --deny AND --forbid unsafe_code (rustc 1.99.1; another crate's macro output is not linted), with no `unsafe` in its
  source. layoutProblems now refuses a proc-macro member target, and any path package (no registry or git source)
  that is not a workspace member (unscanned local code). Tests + mutations M12e, M12f: 27 of 27 caught.
- Pre-final full run: check.mjs 9/9 (test: 243 + 257 passed; self 30/30).
- Final, on the finished tree (base 95597ef + this fix, uncommitted): `node check.mjs` twice in a row, both "all 9
  steps pass" (test 71.6 s / 75.0 s: measured 243 passed, bites 257 passed, 0 failed; self 30/30; deps 78 crate files,
  1 cargo config); `cargo +1.99.1 clippy --locked --workspace --all-targets -- -D warnings` exit 0, the same with
  --all-features exit 0; `cargo +1.99.1 fmt --all --check` exit 0. Changed files: Cargo.toml (lint comment),
  check.mjs, check.test.mjs, crates/tcpinfo/src/{lib,linux,tests}.rs, results/u3c-kernel-check/siocoutq-root-check.py.
  Not touched: crates/net, marks/SERIES.md. No new Rust public symbol (int_answer, require_tcp are private).

## For the main session (not done here)
- S5 U6 carries (security reviewer; no effect in U3c): (a) build tungstenite's limits and DelayStream's from one
  &L2Config (ws_config takes a bare usize today; only the test pair() ties them); (b) U6's 2 s Join deadline from
  accepted_ns must also cover the TLS accept (tokio-rustls's accept has no timeout and runs below DelayStream);
  (d) snapshot_budget has no upper bound and srtt on the plain path is client-influenced: state the plain path's free
  bound (e.g. SO_SNDBUF), not N.
- S6 plan text: linux.rs now has four unsafe blocks, each one operation with its SAFETY comment: srtt_us's SOL_SOCKET
  getsockopt (in its int_option closure) and its TCP_INFO getsockopt; unacked_and_unsent's SOL_SOCKET getsockopt and
  its SIOCOUTQ ioctl. plan.md §4.2 / §4.7 / §4.8 should name them (S6 counted three before F8).
- Not closed, and why: a different rustc or a rustc wrapper (RUSTC, RUSTC_WRAPPER, RUSTC_WORKSPACE_WRAPPER, build.rustc
  / rustc-wrapper keys) can drop any flag; refusing them would also refuse sccache-style setups, and a source scan
  cannot judge a compiler. Registry proc macros (serde_derive, tokio-macros...) expand into our crates unlinted: they
  are dependencies, U5's audit. No `cargo clippy -- -F unsafe_code` pass was added (cap-lints and force-warn defeat -F;
  the token scan sees the code itself; a second CLIPPY_ARGS would re-check shared crates every run).
- F-N's other half: npt-net's tcp_kernel.rs not(unix) handle stays unbuilt (crates/net is fix C1's; it needs a
  non-Unix host target with ring's C build).
- Scratch kept in the session scratchpad (c2probe/: probes, bites.py, realplant.py, fj_check.py, pm/); nothing under
  .gate/ was created (plain rustc wrote one .rmeta / .so in the scratchpad).
DONE
