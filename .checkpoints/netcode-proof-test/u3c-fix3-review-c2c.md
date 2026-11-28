# test-engineer review: U3c final fix round, C2c (uncommitted on c37dfe1)

Started 2026-11-08. Worktree /home/user/Shamble-Frontlines/.claude/worktrees/wf_5c9f078a-da1-2 (read only, never
edited). Scratch copy: /home/user/Shamble-Frontlines/.gate/c2c-review-scratch (spikes/netcode-proof without .git).
CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/u3c-review-target only. Answers my C2b review
(u3c-fix2-review-c2b.md F1-F6). df -h /home/user at start: 4.8G free. Scripts in the session scratchpad
(/tmp/claude-0/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/scratchpad/c2c-review/).

## Plan
1. Read git diff (all 10 files) and the fix agent's notes (u3c-fix3-c2c.md in the worktree).
2. Reproduce `node spikes/netcode-proof/check.mjs` inside the worktree (first, before any scratch build); record
   test count, deps/self counts.
3. Copy spikes/netcode-proof to .gate/c2c-review-scratch.
4. F1: rerun my plant (hyphen sub-tables in tcpinfo Cargo.toml) -> deps must FAIL; revert each new refusal
   (header, hyphen key, dotted/inline, UNSAFE_CODE_NAME in the line scans) -> a test must fail. Look for new ways
   round (quoted keys with escapes, whitespace in headers, `[ lints . rust ]`, multi-line strings / arrays hiding a
   header, CRLF, `lints.workspace`, target.'cfg'.lints? (not a cargo key), priority on unsafe_code).
5. F2: rerun GIT_DIR victim repro against the fixed python test; revert the scrub -> new test must fail;
   siocoutq-root-check.py provenance with GIT_DIR.
6. F3: revert cargoConfigFiles(here, env) -> {} -> test must fail.
7. F4: revert io::stderr() -> io::sink() in tests.rs -> self must fail.
8. F5: RUSTC_BOOTSTRAP + cargo-features + profile rustflags -> deps FAIL; revert each refusal -> test fails;
   false positives: this container env, rustup run env, the committed manifests; other spellings
   (`rustflags` under [profile.x.package.y], `cargo_features`?, `[profile.dev.build-override]`, RUSTC_BOOTSTRAP=0,
   lowercase env name). Other ways round: `[target.<triple>] rustflags` in manifest? (not valid in manifest),
   CARGO_ENCODED_RUSTFLAGS / RUSTFLAGS env (already refused?), `[env]`, cargo's own channel override env.
9. F6: read linux.rs comments; check the cited lines and the claim about the following call.
10. Note: wasm32 runner limit listed.
11. Verdict + table.

## Findings (as I go)
- Step 2 (21:27:56-21:29:22 UTC, after touching the worktree's 62 crate .rs files, mtime only): `node check.mjs` in
  the worktree 9/9: test 258 measured / 272 bites, 0 failed, 0 skips (= fix agent's counts; the parallel C1c copy reads
  260/274, so these are this copy's binaries); deps 78 crate files + 1 config; self 41/41; redact 10/10, 148 items.
  Output .gate/check-c2c-review-1.txt. Clippy 2.4 s after the touch (4 cores; C1c's run shows the same 2.5 s):
  re-checked visibly: touching tcpinfo's 4 .rs then `cargo clippy --workspace --all-targets -D warnings` prints
  "Checking npt-tcpinfo ... npt-net ... npt-lab", 0.94 s, exit 0 (incremental cache: fast but real).
- Step 3: scratch copy at .gate/c2c-review-scratch/netcode-proof (3.0 MB, cp -a); control `node --test
  check.test.mjs` there 41/41.
- Revert battery (scratchpad c2c-review/bites.mjs, output bites-out.txt; each file restored and sha256 checked;
  control 41/41 before and after): 20 of 21 CAUGHT. Caught: F1 sub-table refusal, hyphen/caps refusal, dotted/inline
  refusal (both halves: path under base, base under path), quoted header part, open-bracket skip, UNSAFE_CODE_NAME
  (both uses together and each alone); F2 test scrub, hooksPath, gpgsign (each fails the new check.test.mjs test),
  siocoutq-root-check.py scrub (fails the new Python test, and self twice); F3 cargoConfigFiles(here, {}) (C2R-5 test);
  F4 io::sink() in the real test (C2R-9 test); F5 RUSTC_BOOTSTRAP refusal, manifestKeyProblems unhooked, each key.
  MISSED: depsStep's member-manifest loop removed (`for (const f of files) if (f.endsWith("/Cargo.toml"))
  manifests.push(...)`, check.mjs:1146): 41/41. Only the workspace manifest's plant reaches depsStep; the member half
  of "no manifest has a cargo-features or rustflags key" is wired untested (nit N1; cargo ignores a member's
  [profile], so no bypass is known through it).
- Plants through `check.mjs --only deps` in scratch (plants.mjs, plants-out.txt; files restored, sha256 checked):
  my F1 plant (two hyphen sub-tables, level allow, in tcpinfo) FAILS naming both headers (Cargo.toml:35, :39); a
  hyphen twin key in [lints.clippy] FAILS; my F5 plant (RUSTC_BOOTSTRAP=1 + cargo-features + [profile.dev] rustflags
  --cap-lints warn) FAILS on all three; RUSTC_BOOTSTRAP=1 alone FAILS; a member's [profile.dev] rustflags FAILS (in
  cargo metadata: "feature `profile-rustflags` is required"); control PASS.
- FINDING N2 (low; new way round, pre-existing, same class as C2R-3): inheritsWorkspaceLints (check.mjs:421-430) reads
  raw lines (only `#.*$` stripped, strings not masked), so npt-sim's `[lints]\nworkspace = true` moved inside a
  multi-line string (`[package.metadata.npt]\nnote = """ ... """`, basic or literal) passes deps. End to end
  (n2-e2e.mjs, n2-e2e-out.txt; target .gate/u3c-review-target): a shebang-desynced unsafe block in a new
  crates/sim/src/evil.rs (lexRust's documented limit, check.mjs:191-194) with the committed manifest: deps PASS,
  `cargo check -v -p npt-sim` exit 101 (rustc gets --forbid=unsafe_code: "usage of an `unsafe` block"); with the
  [lints] lines inside the string: deps PASS, cargo check exit 0, rustc gets no lint flag at all. So the claims
  "every member inherits its lints" (DEPS_CHECKED) and "Outside the two exception crates the workspace's forbid makes
  rustc refuse unsafe code this misses" (check.mjs:192-194) are false while deps passes. Fix: read the lines from
  tomlScan(text).mask (strings blanked) in inheritsWorkspaceLints; test with this plant.
- Probes (probes.mjs / probes2.mjs, outputs probes-out.txt / probes2-out.txt; cargo metadata only):
  * cargo 1.99.1 accepts a UTF-8 BOM. lintTables' tomlKeyAt skips spaces and tabs only, so a BOM-prefixed line-1
    key is read as nothing: `lints = { rust = {...}, clippy = {..., iter-over-hash-type = {level allow}} }` on line 1
    is not reported as an inline table (deps FAILS only because the tables then go missing: fails closed); a line-1
    dotted key beside a [lints.clippy] header is a cargo "duplicate key" error. Only a new tool's table (a rustdoc
    lint) gets in unseen: deps PASS with `﻿lints.rustdoc.broken-intra-doc-links = "allow"`. Nit N4 (impact
    nil; the "key for key" claim is literally false for rustdoc). manifestKeyProblems is not fooled (JS \s matches
    U+FEFF): `﻿cargo-features` FAILS.
  * __CARGO_TEST_CHANNEL_OVERRIDE_DO_NOT_USE_THIS=nightly (cargo's own test knob) makes stable cargo 1.99.1 accept
    `cargo-features = ["profile-rustflags"]` (cargo metadata exit 0; exit 101 without it) and deps PASSES with it in the
    environment. The keys it unlocks are refused anyway (cargo-features, rustflags; config rustflags are scanned for
    lint-lowering flags; every *RUSTFLAGS env), so no bypass found. Nit N3: the same reason RUSTC_BOOTSTRAP is refused
    for; neither refused nor named among the known limits (check.mjs:1101-1103 names a nightly toolchain only).
  * TOML 1.1 multi-line inline table in [lints.clippy]: cargo accepts it; deps FAILS ("runs past its line"). Good.
- F2 rerun of my C2b reproduction (victim repo in scratchpad c2c-review/victim, one commit c85b695 holding keep.txt):
  GIT_DIR, GIT_WORK_TREE and GIT_INDEX_FILE exported to it, `python3 -I -B test_siocoutq_root_check.py` from the
  scratch copy: Ran 6 tests, OK; victim afterwards: `git log --all` still only c85b695, status clean, keep.txt there.
- F5 false positives: worktree `rustup run 1.99.1 node check.mjs --only deps` PASS (after touching the 62 .rs);
  cargoConfigProblems(process.env, []) = [] under rustup run (env: CARGO_HOME, CARGO_HTTP_CAINFO, CARGO_TARGET_DIR,
  RUSTUP_HOME, RUSTUP_TOOLCHAIN, RUST_BACKTRACE, RUST_RECURSION_COUNT; plus 12 GIT_*). ~/.cargo has no config. The
  committed 17 manifests pass manifestKeyProblems (test) and deps (run).
- F6 checked against the local v6.8 copies (scratchpad u3cbfix-linux-v6.8): net_socket.c:2377-2380 is `if (!compat)
  err = BPF_CGROUP_RUN_PROG_GETSOCKOPT(...)` after sk_getsockopt (:2365) / ops->getsockopt; sock_do_ioctl :1222
  ops->ioctl, :1231-1232 ENOTTY when !is_socket_ioctl_cmd; net_core_sock.c :4191-4199 sock_ioctl_out puts one int,
  :4207-4224 sk_ioctl (raw AF_INET/AF_INET6 go through ipmr/ip6mr first, which do not take 0x5411, then the
  default); af_inet.c:1004 and af_inet6.c:591 call sk_ioctl. Both linux.rs comments (:58-61, :135-140) are right
  as far as they go. Nit N5 (rust-reviewer's call): unacked_and_unsent's line names inet families and families with
  no answer, not a non-inet family that does answer 0x5411 (AF_UNIX, AF_PACKET answer TIOCOUTQ with one int, by my
  reading of the kernel's convention; not in the local copy, so not verified here).
- Note (wasm32 runner): listed in the known limits above DEPS_CHECKED (check.mjs:1106-1108). Done.
- Final worktree run (after the scratch build of npt-sim, all 62 .rs touched again; 21:45:33-21:46:57 UTC): check.mjs
  9/9, test 258 / 272, 0 failed, 0 skips (same counts), self 41/41, deps PASS; output .gate/check-c2c-review-2.txt.
  Worktree content unchanged (git diff --stat: 10 files, +501 -74, as handed over). Scratch copy restored
  (crates/sim/src holds lib.rs only again). Disk 4.8G free throughout.

## Verdict: gaps (small). F1-F6 and the note are done, each code fix bites when reverted; one new way round (N2,
pre-existing) and four nits.

| Item | Claim | Proof (reverted in scratch / reproduced) | Gap |
|---|---|---|---|
| F1 headers | a lint's own sub-table, quoted part, [[...]] refused | check.test.mjs:593 fails on each revert; my plant -> deps FAIL naming both headers | BOM line 1 read as nothing (N4, nil impact) |
| F1 names | `-` / capitals in a lint key refused | :616 fails on revert; hyphen twin plant -> deps FAIL | none |
| F1 dotted/inline | lints by dotted key or inline table anywhere refused | :638 fails on each half and on the open-bracket skip | none |
| F1 L2 | line scans read unsafe-code | :616 / :663 fail on each scan's revert | inheritsWorkspaceLints still raw lines (N2) |
| F2 test | fixture git stays in its scratch repos | :1066 fails on scrub / hooksPath / gpgsign revert; my GIT_DIR victim unchanged | none |
| F2 script | provenance names its own repo | Python test :114 and self fail on revert | none |
| F3 | depsStep reads the given CARGO_HOME | :1016 assertion fails on cargoConfigFiles(here, {}) | none |
| F4 | real MPTCP test writes to stderr | :1040 pin fails on io::sink() | none (deliberate std shadowing is review's) |
| F5 env | RUSTC_BOOTSTRAP refused | :911 fails on revert; deps FAIL; no FP plain or under rustup run | channel override env not named (N3) |
| F5 manifests | cargo-features / rustflags refused in every manifest | :911 fails per key; C2R-5 test fails if unhooked | member-manifest loop unwired-proof (N1) |
| F6 | BPF line in both SOL_SOCKET SAFETY comments | cites match local v6.8 (socket.c:2377-2380, :1222-1232; sock.c:4191-4224) | non-inet answering families unnamed (N5) |
| Note | runner refusal covers wasm32 | check.mjs:1106-1108 | none |

Ranked: N2 (low, fix + test: one line), N1 (nit, one plant in the C2R-5 fixture), N3 (nit, refuse or list),
N4 (nit, list or skip U+FEFF in tomlKeyAt), N5 (nit, comment).
DONE
