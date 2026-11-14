# U3c last fix round, part C2c: check.mjs lint tables, refusals, test guards, linux.rs comments

Agent: workflow fix agent C2c, worktree .claude/worktrees/wf_5c9f078a-da1-2 at c37dfe1 (spike branch
claude/netcode-proof-test-plan-hcky5g). Inputs: u3c-fix2-review-c2b.md (F1-F6, verdict table), u3c-fix2-c2b.md,
advisor-u3c-plan.md "Side rulings: U3c's last fix round", u3c-fix-review-result.txt C2R-9.
Rules: cargo +1.99.1 with CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/u3c-review-target only; touch every .rs
under spikes/netcode-proof/crates before the first build; df -h /home/user before each build (stop under 1.5 GB);
no iptables/tc, no #[ignore]d evidence tests, no network; commit nothing; no git -C / -c; Write/Edit tools only.

## Plan
1. F1: lintTables reports any `[<prefix>lints.<x>.<y>...]` header (not exactly a tool table), dotted/inline forms it
   does not parse exactly, and any lint key holding `-`; the raw-line unsafe_code regexes read `-` as `_`
   (`unsafe[-_]code`). One test each in check.test.mjs. Fix the claims (exceptionLintProblems "key for key", lintTables
   "so it never passes", workspace Cargo.toml lint comment, tcpinfo Cargo.toml + lib.rs doc, DEPS_CHECKED).
2. F2: test_siocoutq_root_check.py's git(): env without GIT_*, core.hooksPath = empty dir, commit.gpgsign false
   (via GIT_CONFIG_* env or a -c inside the Python argv). Test: run the file with GIT_DIR at a scratch repo, repo
   unchanged (check.test.mjs). siocoutq-root-check.py's own git calls: check whether GIT_* misleads provenance; scrub +
   test if so.
3. F3: test that depsStep passes its env's CARGO_HOME to cargoConfigFiles (scratch CARGO_HOME with a refused config).
4. F4: make the real MPTCP test's stderr wiring fail on io::sink() (self-step check or structure).
5. F5: refuse RUSTC_BOOTSTRAP in env; `cargo-features` key and any `rustflags` key in workspace or member manifests.
   Tests; no false positive on this container's env/manifests. Nightly channel in rust-toolchain.toml -> known limit
   (or refuse it if in reach).
6. F6: one line in linux.rs's SOL_SOCKET SAFETY comments on the cgroup-BPF getsockopt hook (v6.8 net/socket.c:
   2377-2380), and what the next call then does.
7. Note: runner refusal covers every triple (a future wasm32 runner refused) -> known limits list.
8. Bites: revert each fix once, run its test, record how it fails, restore.
9. Final: check.mjs 9/9 twice; clippy -D warnings with/without --all-features; cargo fmt --all --check.

## Progress
- Start: worktree fast-forwarded to c37dfe14 (git log -1 checked). Env names here: CARGO_HTTP_CAINFO, RUSTUP_HOME,
  RUST_BACKTRACE, GIT_SSL_CAINFO, GIT_ASKPASS, GIT_EDITOR, GIT_TERMINAL_PROMPT, GIT_CONFIG_COUNT/KEY_0-2/VALUE_0-2
  (credential.interactive, url.*.insteadOf); no RUSTC_BOOTSTRAP; ~/.cargo has no config.toml. Disk 5.0 GB free.
- Probe (rustc +1.99.1 --emit=metadata into scratchpad c2c/): `#![allow(unsafe-code)]` is a parse error ("expected
  identifier, found keyword `unsafe`"); `#![allow(UNSAFE_CODE)]` is "unknown lint" (warning, deny still errors). So the
  hyphen rule matters in manifests only; the Rust-source scans keep `unsafe_code`.
- Kernel cites re-read (scratchpad u3cbfix-linux-v6.8/net_socket.c): :2377-2380 is the `if (!compat) err =
  BPF_CGROUP_RUN_PROG_GETSOCKOPT(..., max_optlen, err)` after sk_getsockopt/ops->getsockopt; sock_do_ioctl :1222 calls
  ops->ioctl, :1231-1232 returns ENOTTY for a cmd that is no socket ioctl number (0x5411 never reaches dev_ioctl);
  sock_ioctl :1341 default -> sock_do_ioctl; net_core_sock.c sock_ioctl_out :4191-4199 put_user one int, sk_ioctl
  :4207; af_inet.c:1004 / af_inet6.c:591 -> sk_ioctl.
- Item 1 (F1) code + tests written: tomlKeyAt (dotted keys, quoted parts decoded), openBrackets (a value's run-on lines
  skipped), lintTables rewritten (lint header not exactly [<prefix>lints] / [<prefix>lints.<tool>] with bare lowercase
  names -> problem; key not lowercase `_` -> problem; dotted/inline lints anywhere by key path, quoted parts included);
  UNSAFE_CODE_NAME = /\bunsafe[-_]code\b/i in workspaceLintProblems' line scan and the other manifests' line scan.
  check.test.mjs: 4 new tests (sub-table/quoted/array headers; hyphen/caps names; dotted/inline forms; line scans) and
  the `[workspace."lints".rust]` case now expects the header refusal. node --test: 39/39.
- Items 2-5 code + tests written: F2 test_siocoutq_root_check.py git() (env without GIT_*, -c core.hooksPath=<empty
  tmp dir>, -c commit.gpgsign=false), scratch_repo(), new Python test (GIT_DIR/GIT_WORK_TREE/GIT_INDEX_FILE patched to
  another repo: provenance names its own commit); siocoutq-root-check.py git() helper with GIT_* scrubbed for its three
  calls; check.test.mjs runs the Python file with GIT_DIR + GIT_INDEX_FILE at a scratch repo and compares a byte
  snapshot of it. F3: depsStep given CARGO_HOME with a `rustc` config -> exactly that problem. F4: check.test.mjs pins
  the real MPTCP test's body (spaces aside) to `mptcp_case(|| Socket::new(...MPTCP)), &mut std::io::stderr())`. F5:
  RUSTC_BOOTSTRAP refused (any value) in cargoConfigProblems; new manifestKeyProblems (cargo-features, rustflags in any
  form) on Cargo.toml + every crates/**/Cargo.toml, hooked into depsStep (plant added to the C2R-5 fixture); no false
  positive on the 17 real manifests (test). Python 6/6; node --test 41/41.
- F2 extra: the check.test.mjs F2 test also runs the Python file with HOME at a scratch ~/.gitconfig (core.hooksPath to
  a post-commit hook that leaves a mark; commit.gpgsign = true with gpg.program = false): status 0 and no mark.
- Items 6-7 (F6 + note, comments only, no test possible): linux.rs's two SOL_SOCKET SAFETY comments each gain the
  cgroup-BPF lines (socket.c:2377-2380; srtt_us: TCP_INFO then asked of the socket's own family, within `len`, rtt_from
  reads only what was written; unacked_and_unsent: SIOCOUTQ to the socket's own family via sock_ioctl, ENOTTY for a
  family with no answer (socket.c:1222-1232), inet families one int via sk_ioctl (sock.c:4191-4199, :4207-4224), the
  count being that protocol's). Known limits above DEPS_CHECKED: toolchain/nightly not detected; runner refusal covers
  every triple (a future wasm32 runner refused too); [env] can set any variable for rustc. Claims updated: Cargo.toml
  lint comment, tcpinfo Cargo.toml + lib.rs doc, client-wasm Cargo.toml, DEPS_CHECKED, lintTables/exception docs.
  tests.rs: doc line on the pinned body (comment only).
- No false positive (F5): `node envcheck.mjs` plain and under `rustup run 1.99.1`: cargoConfigProblems(process.env) =
  [] (env names CARGO_HTTP_CAINFO, GIT_* x12, RUSTUP_HOME, RUST_BACKTRACE; + CARGO_HOME, RUSTUP_TOOLCHAIN,
  RUST_RECURSION_COUNT). check.mjs --only deps,self: PASS (78 crate files, 1 config), self 41/41.
- Item 8 (bites): scratchpad c2c/bites.mjs, output c2c/bites-out.txt: 15 of 15 CAUGHT (F1 x6, F2 x4, F3, F4, F5 x3),
  control 41/41 before and after, sha256 of every mutated file equal before and after. Each revert and its failing
  test is in bites-out.txt.
- End to end (scratchpad c2c/e2e.mjs, files restored, sha256 checked): the reviewer's F1 plant (two sub-tables with
  level "allow" in crates/tcpinfo/Cargo.toml) -> `check.mjs --only deps` FAIL with both headers named; F5 (RUSTC_BOOTSTRAP=1
  + cargo-features + [profile.dev] rustflags --cap-lints warn; cargo metadata accepts it under the bootstrap) -> FAIL
  with all three; control PASS before and after.
- Item 9 (final), disk 5.0 GB then 4.5 GB free; every crate .rs touched (62 files) before the first build:
  check.mjs run 1 (21:18:10-21:20:06 UTC) 9/9: test measured 258 / bites 272, 0 failed, 0 skips; deps 78 crate files,
  1 config; self 41/41; redact 10/10, 148 items. Run 2 (21:20:19-21:21:35) 9/9, same counts. Counts vs the reviewer's
  run at b5e4043 (252/266): +6/+6 = the 6 tests C1b (f01d573) added (git diff b5e4043 c37dfe14: 6 `#[test]` added, 0
  removed); this change adds no Rust test. Explicit cargo +1.99.1 clippy --workspace --all-targets -D warnings exit 0,
  with --all-features exit 0; cargo +1.99.1 fmt --all --check exit 0.
- Late tidy (after those runs): lintTables now counts a value's open brackets before skipping a refused lint table's
  lines, so a `[...]` line inside an array there is not read as a header either. Then again: bites 15 of 15 CAUGHT
  (control 41/41 before and after); check.mjs 9/9 at 21:23:05-21:24:21 and 21:24:48-21:26:06 UTC (test 258 / 272,
  0 failed, 0 skips, same as before; self 41/41); npt-tcpinfo's three .rs touched, then explicit clippy -D warnings
  exit 0 both builds (re-checked npt-tcpinfo and its dependents), fmt --check exit 0. Disk 4.5 GB free.
- Side effect to know: siocoutq-root-check.py's git no longer sees GIT_CONFIG_* either, so a safe.directory passed that
  way would not reach it; a root run on another user's checkout would then print "git status failed: ... dubious
  ownership" in its provenance line (visible), never another repository's commit.
- Not done here (by design): a nightly toolchain is not detected (named in the known limits above DEPS_CHECKED; the
  keys and RUSTC_BOOTSTRAP it would need are refused whatever the channel); a config [env] RUSTC_BOOTSTRAP falls under
  the existing [env] limit. Nothing committed.
DONE
