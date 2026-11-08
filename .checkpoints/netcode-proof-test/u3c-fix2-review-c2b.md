# test-engineer review: U3c last fix round, C2b (b5e4043)

Started 2026-11-08 (resumed after a usage limit; no earlier notes for this review existed). Branch
claude/netcode-proof-test-plan-hcky5g in the main folder, read only. Scratch copy under the session scratchpad
(c2b-review/), never the main tree. CARGO_TARGET_DIR for anything in the main folder: .gate/u3c-review-target.
No iptables.

Brief: advisor-u3c-plan.md "Side rulings: U3c's last fix round" (2): (a) C2R-5 self-test + honest comments
(C2R-6 / REG-2); (b) refusals RUSTC, CARGO_BUILD_RUSTC, workspace wrapper, config rustc / rustc-workspace-wrapper,
include, runners, each with a test, none refusing this container's env; forbid backstop + exceptionLintProblems,
kept only if 9/9; (c) C2R-9, C2R-10; (d) known limits documented. Fix notes: u3c-fix2-c2b.md.

## Plan
1. Read C2R-1..10 / REG-2 / REG-3 text (u3c-fix-review-result.txt :200-330) so each item's bar is known.
2. Read `git show b5e4043` (Cargo.toml, both exception manifests, check.mjs, check.test.mjs, tcpinfo, python).
3. Per item: locate the test; revert the fix in a scratch copy; run the test; record bite or no bite.
4. False positives: this container's env (env | grep CARGO/RUST), ~/.cargo/config.toml, a sccache-style setup,
   lowercase/alternate spellings of env names, CARGO_TARGET_<triple>_LINKER (must stay allowed?).
5. Backstop defeat: can the exception crates' own lint tables be changed in a way exceptionLintProblems accepts but
   that weakens the backstop (e.g. a priority key, a lint group, `unsafe_op_in_unsafe_fn`, a [lints] workspace=true
   alongside own tables, a [lints.rustdoc] table, a cfg'd target table)? Also: a non-exception crate dropping
   `workspace = true`, crate-level `#![allow(unsafe_code)]` under forbid, RUSTFLAGS --cap-lints / -A unsafe_code.
6. Reproduce check.mjs 9/9 in a scratch copy.
7. Verdict + table.

## Findings (as I go)
- Step 1-2 read: C2R-1..10 bars (u3c-fix-review-result.txt :200-330); manifests, check.mjs :400-850.
- Lead L1 (to test): lintTables (check.mjs:504-559) silently skips any header that is not `[<prefix>lints]` or
  `[<prefix>lints.<tool>]`, and lines under it unless they start `lints.`/`lints =`. A sub-table header such as
  `[workspace.lints.rust.unsafe-code]` + `level = "allow"` is read by nobody: workspaceLintProblems' scan (:573-577)
  looks for `\bunsafe_code\b`, which a hyphen spelling avoids. Whether cargo/rustc honour `unsafe-code` and whether a
  later command-line --allow relaxes an earlier --forbid: test with cargo +1.99.1.
- Lead L2: non-exception manifests are checked by a raw-line regex `\bunsafe_code\b` (:462-466), hyphen spelling again.
- L1 result (mini workspace, scratchpad c2b-review/mini, cargo +1.99.1): workspace `unsafe_code = "forbid"` plus a
  hyphen key `unsafe-code = "allow"` (inline, inline with priority 1, or a `[workspace.lints.rust.unsafe-code]`
  sub-table with level allow, priority 1): cargo passes `--forbid=unsafe_code --allow=unsafe-code` (warns
  "deprecated" and "duplicate lint"), and rustc STILL errors "usage of an unsafe block ... requested on the command
  line with -F unsafe-code", exit 101. A later command-line allow does not lift a command-line forbid. The backstop
  holds against this spelling. Next: same trick against a DENY level (tcpinfo's) and against clippy lints.
- Main-folder reproduction (read-only, CARGO_TARGET_DIR=.gate/u3c-review-target, 20:11-20:14 UTC): check.mjs 9/9,
  test 252 measured / 266 bites, 0 failed, 0 skips; deps "78 crate files and 1 cargo config file(s)"; self 35/35;
  2 m 44 s. Tree unchanged afterwards (git status: only these notes).
- L1b (mini, deny level): `unsafe_code = "deny"` + `[lints.rust.unsafe-code] level=allow priority=1` in a crate's own
  tables: cargo passes --deny=unsafe_code --allow=unsafe-code, an unsafe block with no #[allow] COMPILES (exit 0).
  A non-exception crate cannot do it: cargo refuses `[lints] workspace = true` beside any own lint table ("cannot
  override workspace.lints"), exit 101.
- L1c (mini2, clippy): own `[lints.clippy] iter_over_hash_type = "deny"` + `[lints.clippy.iter-over-hash-type]
  level=allow priority=1`: `cargo clippy -- -D warnings` exits 0 on a HashMap for-loop (plain: exit 101, "requested
  with -D clippy::iter-over-hash-type"). So a hyphen sub-table relaxes any deny-level lint. Next: does check.mjs's
  exceptionLintProblems accept it in the real manifests (scratch clone)?
- FINDING F1 (low; exceptionLintProblems' "key for key" claim is false): in the scratch clone at b5e4043, adding to
  crates/tcpinfo/Cargo.toml `[lints.rust.unsafe-code]` + `[lints.clippy.iter-over-hash-type]` (level "allow",
  priority 1 each) leaves `check.mjs --only deps,self` PASS (self 35/35). With a HashMap for-loop planted in
  tcpinfo/src/lib.rs, `cargo +1.99.1 clippy -p npt-tcpinfo --all-targets -- -D warnings` exits 0 (rustc gets
  --deny=clippy::iter_over_hash_type then --allow=clippy::iter-over-hash-type, and --deny=unsafe_code then
  --allow=unsafe-code); control with the original manifest exits 101 ("iteration over unordered hash-based type").
  Root cause: lintTables (check.mjs:513-529) skips a `[lints.<tool>.<lint>]` sub-table header and its lines without a
  problem (contrary to its doc :499-503, "so it never passes"), and nothing normalizes `-` to `_` in lint names
  (cargo 1.99.1 only warns "deprecated ... duplicate lint"). Does NOT defeat the forbid backstop for the inheriting
  crates (L1: rustc keeps a command-line forbid), and in tcpinfo the token scan still refuses `unsafe` outside the
  two fns, so its unsafe reach equals the documented lexer limits. What it defeats: the exceptions' clippy lints
  (iter_over_hash_type bites there; disallowed_* are vacuous there, no clippy.toml), and the claims at
  check.mjs:583-588, Cargo.toml:73-79 comment, tcpinfo Cargo.toml:13, tcpinfo lib.rs doc, DEPS_CHECKED.
  Fix: in lintTables, report any header that starts `<prefix>lints.` and is not exactly a tool table, and report a
  lint key holding `-` (or compare with `-` read as `_`); one test each.
- Revert battery (scratchpad c2b-review/bites.mjs, output bites-out.txt; scratch clone at b5e4043, each file restored,
  control self 0 failures before and after): 32 of 33 CAUGHT. Caught: all 7 depsStep wirings, STEPS.deps as a
  wrapper, depsStep ignoring its env (a), the 4 COMPILER_ENV names, both COMPILER_KEYS, include as key and as
  [[include]] header, runner env and config, forbid accepting deny, the forbid line scan, exception key compare,
  extra key, level, tcpinfo level allow, quoted-key refusal, reading lint tables from `clean` (C2R-3), non-exception
  inherit; cargoCounts' SKIPPED counter; the 3 provenance reverts (python test and self both fail). MISSED:
  depsStep calling cargoConfigFiles(here, {}) (the given CARGO_HOME ignored, ~/.cargo read instead): no test sees it
  (this container has no ~/.cargo/config.toml). Nit F3.
- C2R-9 Rust side (scratchpad c2b-review/rust-bites.mjs, out rust-bites-out.txt; target .gate/c2b-review-scratch-target):
  control 10 passed, 0 skips. socket() forced to fail in the real test: status 0, the SKIPPED line reaches cargo's
  output past libtest's capture, cargoCounts reads "10 tests passed, 0 failed, 1 reported a skip (an_mptcp_...)"
  (end-to-end claim reproduced). report_skip reverted to eprint!: CAUGHT (a_kernel_without_mptcp_... FAILED).
  The real test handed io::sink() instead of io::stderr(): NOT caught (status 0; on an MPTCP host the skip path
  never runs, and the unit test writes to a Vec). Nit F4.
- False positives: main-folder env passes deps (CARGO_HTTP_CAINFO, RUSTUP_HOME, RUST_BACKTRACE); `rustup run 1.99.1`
  adds CARGO_HOME, RUSTUP_TOOLCHAIN, RUST_RECURSION_COUNT: cargoConfigProblems(process.env) = [] (checked). CI does
  not run check.mjs (.github/workflows/gate.yml, tools/gate.mjs: no match). Future only: the runner refusal covers
  every triple, so wasm-bindgen-test's usual `[target.wasm32-unknown-unknown] runner` would be refused though the
  test step runs host tests only; no wasm test is planned (plan.md: no match). Note, not a finding.
- FINDING F2 (low, latent, destructive): test_siocoutq_root_check.py's git() (:19-25) inherits the environment. With
  GIT_DIR set to a throwaway repo (scratchpad c2b-review/victim), `python3 -I -B test_siocoutq_root_check.py` made
  a commit "fixture" on the victim's branch whose tree drops the victim's keep.txt and adds the fixture files
  (git log --stat: keep.txt 1 -, 4 files changed); 5 tests then fail/error. Trigger: any run of check.mjs's self
  step with GIT_DIR / GIT_INDEX_FILE / GIT_WORK_TREE exported (a git hook, `git rebase --exec`). Not triggered today
  (nothing runs check.mjs from a hook). Fix: pass env with every GIT_* variable removed (and core.hooksPath and
  commit.gpgsign pinned) to the fixture's git calls; test it by running the file with GIT_DIR set to a scratch repo.
- Source: git-scm.com/docs/githooks (read 2026-11-08): "Environment variables, such as GIT_DIR, GIT_WORK_TREE, etc.,
  are exported so that Git commands run by the hook can correctly locate the repository. If your hook needs to invoke
  Git commands in a foreign repository ... it should clear these environment variables", example
  `unset $(git rev-parse --local-env-vars)`. GIT_INDEX_FILE not named there. Sure: high (official doc). Supports F2's
  trigger and its fix.
- Backstop reproduced (scratchpad c2b-review/backstop.sh, outputs backstop-*.txt): planted #[allow(unsafe_code)] +
  unsafe block in scratch npt-link: under the committed forbid, cargo check -v shows --forbid=unsafe_code and
  "error[E0453]: allow(unsafe_code) incompatible with previous forbid", exit 101; workspace reverted to deny
  (control): --deny=unsafe_code, exit 0; deps FAILS on the deny manifest (Cargo.toml:84 ... and "no ... forbid").
- FINDING F5 (low/nit, unlisted limit): RUSTC_BOOTSTRAP=1 in the environment + workspace Cargo.toml
  `cargo-features = ["profile-rustflags"]` and `[profile.dev] rustflags = ["--cap-lints", "warn"]`: deps PASSES
  (cargoConfigProblems reads config files only, never manifests; RUSTC_BOOTSTRAP is not refused), and cargo check of
  scratch npt-link with the planted allow + unsafe exits 0 with no warning (rustc gets --forbid=unsafe_code and
  --cap-lints warn). Without RUSTC_BOOTSTRAP cargo refuses the manifest and deps FAILS (cargo metadata). Mini
  workspace: same, forbid becomes a warning. Not in DEPS_CHECKED's known limits (check.mjs:965-972). The same
  manifest change under a nightly channel in rust-toolchain.toml is untested here (no nightly installed). Cheap fix:
  refuse RUSTC_BOOTSTRAP in env and a `cargo-features` or `rustflags` key in any member or workspace manifest.
- Carries not tracked in the main checkpoint (grep .checkpoints/netcode-proof-test.md: no "BPF", no "1.8 (a)"):
  C2R-9 (2) the BPF-hook line in linux.rs's SOL_SOCKET SAFETY comments (not done; grep -i bpf crates/tcpinfo/src:
  none), C2R-9 (4) plan.md:627 "two unsafe blocks", u3c-plan.md:275 "Keep `[lints] workspace = true`" (superseded by
  the ruling). Only u3c-fix2-c2b.md names them. Nit F6.

## Verdict: REVISE (small). Nothing functional broke; 9/9 reproduced; 3 cheap fixes and 3 nits.

| Item | Claim | Proof (reverted in scratch) | Gap |
|---|---|---|---|
| (a) C2R-5 | deps step calls every check | check.test.mjs:830 fails on each of 7 unhooked checks, STEPS.deps as wrapper, env ignored (9/9) | CARGO_HOME param not tested (F3) |
| (a) C2R-6/REG-2 | comments say what is checked | read; "review is the wall" wording honest; limits at check.mjs:959-972 | "key for key" / "the workspace's otherwise" false (F1) |
| (b) RUSTC, CARGO_BUILD_RUSTC, both workspace wrappers | refused | check.test.mjs:748 fails on each of 4 removals | none |
| (b) config rustc, rustc-workspace-wrapper | refused | :748 fails on each removal | none |
| (b) config include (key, [[include]]) | refused | :748 (and :830 for the key) fail | none |
| (b) runner env + config | refused | :748 (+ :830 for env) fail | wasm32 runner would be refused too (future note) |
| (b) this container's env | not refused | main run deps PASS; rustup run env -> [] | none |
| (b) forbid backstop | other crates cannot allow unsafe | E0453 reproduced; deny control compiles; check.test.mjs:529 fails if deny accepted | RUSTC_BOOTSTRAP + manifest profile rustflags caps it, deps PASS, unlisted (F5) |
| (b) exceptionLintProblems | exception tables = workspace's but unsafe_code | :563 fails on key compare, extra key, level, tcpinfo allow | sub-table + hyphen name accepted, relaxes clippy deny (F1) |
| (c) C2R-9 skip visible | SKIPPED line counted | end to end in scratch; report_skip->eprint caught (tests.rs unit test); counter caught (:875) | real test's stderr wiring untested (F4); BPF line not done (F6) |
| (c) C2R-10 | only untracked own logs hidden | 3 reverts each fail python test + self (:887) | test's git inherits GIT_DIR: commits into another repo (F2) |
| (d) known limits | documented | check.mjs:959-972 lists [env], build script, RUSTC_WRAPPER, toolchain path, lexer, macros | F5 missing |
