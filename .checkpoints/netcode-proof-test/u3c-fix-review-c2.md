# U3c fix-pass re-review, part C2 (security-auditor): check.mjs and npt-tcpinfo

Scope: `git diff 95597ef..81203da -- spikes/`, the C2 half only (check.mjs, check.test.mjs, Cargo.toml lint comment,
crates/tcpinfo/src/{lib,linux,tests}.rs, results/u3c-kernel-check/siocoutq-root-check.py).
Findings assigned: S1, F-D, F-E, F-N, F8, F-J, S5 (part c).
Inputs read: u3c-review-result.txt (S1 :399, S5 :431, F-D :527, F-N :599, F-E :607, F-J :639, F8 :76 and verdicts),
u3c-fix-c2.md (fix notes), u3c-plan.md rev 2, advisor-u3c-plan.md.
Rules: no source edits; bypass attempts in a scratch copy only (scratchpad), CARGO_TARGET_DIR=.gate/u3c-review-target;
no iptables, no #[ignore]d evidence tests. Memory: none stored (no .claude/agent-memory for this agent).

## Plan
1. Read the C2 diff (check.mjs, check.test.mjs, tcpinfo, root script, Cargo.toml).
2. Per finding: fixed / partly / not, file:line, the test that fails when the fix is reverted (revert in a scratch copy).
3. Bypass attempts against the new rules in a scratch copy of spikes/netcode-proof (outside the main tree).
4. False positives: run check.mjs (deps, self) on the real tree; judge refusals of legitimate setups (RUSTC_WRAPPER,
   CARGO_HOME config, sccache, CI env).
5. F8: each of the 4 unsafe blocks: SAFETY invariant, lengths, placement.
6. Regressions.

## Progress
- Block 1 (read): C2 diff read: check.mjs (906 lines at 81203da), linux.rs (4 unsafe blocks: :53, :82, :126, :152),
  tests.rs (MPTCP test), siocoutq-root-check.py (uncommitted()/provenance() :154-182), Cargo.toml lint comment :69-79.
- Block 2 (baseline): scratch copy = `git archive 81203da spikes/netcode-proof` in
  scratchpad/c2review/base. `node check.mjs --only deps,self` PASS there (78 crate files, 1 cargo config; self 30/30)
  and PASS on the real tree: no false positive on the tree as it is.
- BYPASS A (S1, reproduced): a first line `#!/x /*` in crates/room/src/evil.rs (+ `pub mod evil;` in lib.rs). rustc
  strips a shebang line in ANY source file (module files too), lexRust (check.mjs:177-225) does not: it opens a block
  comment at the `/*`, which runs to a `*/` that rustc sees inside `// */`. Between them: a macro-built
  `#[allow($l)]` + `lax!(unsafe_code)` over `unsafe { *(&raw const x) }`. Result in scratch copy atkA:
  `node check.mjs --only deps` PASS; `cargo check -p npt-room` OK; `cargo clippy -p npt-room --all-targets -- -D
  warnings` OK. Control atkActl (same file, `#[deny($l)]`): rustc errors "usage of an `unsafe` block" at evil.rs:2,
  so rustc does compile line 2. The UNSAFE_CODE token scan (check.mjs:159, :440-443) is blind to it.
- BYPASS B (S1, reproduced): `pub const A: [char; 2] = ['\u{0_0_0_0_4_1}','"'];` then the same macro-built allow +
  unsafe block, then `// "`. rustc accepts `\u{...}` with up to 6 hex digits and any underscores; lexRust's charLit
  (check.mjs:179) allows `[0-9a-fA-F_]{1,6}` (underscores counted), so it fails on the first literal, re-syncs on
  `','`, and opens a string at the `"` rustc reads inside `'"'`; that fake string blanks the unsafe code in the mask
  up to the `"` in `// "`. Scratch atkB: deps PASS, clippy -p npt-room -D warnings OK; control atkBctl (`deny`):
  rustc "usage of an `unsafe` block" at evil.rs:2.
- BYPASS C (S1/F-E path rule, reproduced): `#[path = ".\x2e/.\x2e/client-wasm/src/evil.rs"] pub mod evil;` in
  npt-room's lib.rs; crates/client-wasm/src/evil.rs holds `#[allow(unsafe_code)]` + an unsafe block (client-wasm's
  files are never scanned for unsafe: check.mjs:438). rustc unescapes the literal (the first try with a leading
  `\x2e\x2e` showed rustc's span "crates/room/src/../../client-wasm/src/evil.rs"); pathProblems reads the raw text
  (check.mjs:291) and splits `.\x2e` into "." and "x2e" (:278), so it thinks the file is deeper in crates/room/.
  Scratch atkC: deps PASS; clippy -p npt-room --all-targets -D warnings OK. (A LEADING backslash is caught only by
  accident: :275 treats it as an absolute path.)
- BYPASS D (S1/F-E path rule, reproduced): `use std::include as inc; inc!("../../client-wasm/src/evil.rs");` in
  npt-room's lib.rs. pathProblems only knows the macro by its name (`\binclude\s*!`, check.mjs:298). Scratch atkD:
  deps PASS, clippy OK; control (`#[deny(unsafe_code)]` in evil.rs): rustc errors at
  "crates/room/src/../../client-wasm/src/evil.rs:2:36", so the file is compiled into npt-room. The same rename
  would let a build.rs-generated OUT_DIR file in (the computed-path refusal at :300 is keyed on the name too).
- BYPASS E (F-D, reproduced): root Cargo.toml line 80 rewritten as `"unsafe_code" = "allow"` (a TOML basic-string
  key escape; cargo decodes it to unsafe_code), plus a decoy appended: `[workspace.metadata.note] text = """`
  / `[workspace.lints.rust]` / `unsafe_code = "deny"` / `"""`. workspaceLintProblems (check.mjs:486-502) scans
  tomlScan(text).clean, which KEEPS strings, so the decoy's two lines inside the string count as the table header
  and the `found` line, while the escaped real key never matches `\bunsafe_code\b`. Scratch atkE: deps PASS;
  `cargo check -v -p npt-room` passes `--allow=unsafe_code` (base copy: `--deny=unsafe_code`).
- BYPASS F (S1 cargoConfigProblems, reproduced): spike .cargo/config.toml gains `include = ["extra.toml"]`;
  .cargo/extra.toml holds `[build] rustflags = ["--cap-lints", "warn"]`. cargo 1.99.1 loads it on stable (cargo -v
  shows `--cap-lints warn` on npt-proto and npt-room); cargoConfigFiles (check.mjs:671-689) reads only config/config.toml
  and never follows `include`. Impact shown in scratch atkF: an unsafe block in srtt_us with no SAFETY comment and two
  unsafe operations passes `cargo clippy -p npt-tcpinfo --all-targets -- -D warnings` (2 warnings only); control (the
  include removed): 2 errors (undocumented_unsafe_blocks, multiple_unsafe_ops_per_block). deps PASS throughout.
- BYPASS G (S1 cargoConfigProblems, reproduced): spike .cargo/config.toml gains
  `[env] CLIPPY_ARGS = { value = "--cap-lints=warn__CLIPPY_HACKERY__", force = true }`. cargo's [env] reaches the
  clippy-driver process and replaces cargo-clippy's own CLIPPY_ARGS (`-D warnings`). Scratch atkG (with atkF's
  undocumented two-op block in srtt_us): deps PASS; `cargo clippy -p npt-tcpinfo --all-targets -- -D warnings`
  finishes with 2 warnings. cargoConfigProblems (check.mjs:641-665) reads only `rustflags` keys and alias keys.
- Attack A through check.mjs's own steps (rustfmt-formatted plant): `node check.mjs --only
  fmt,clippy,clippy-wasm32,deps,self` -> "check: all 5 steps pass" (scratch atkA, CARGO_TARGET_DIR=.gate/u3c-review-target).
  So the unsafe block in npt-room passes every static step; `cargo check -p npt-room` builds it (test step not run:
  2 min, and the build is what it would add).
- BYPASS H (G's class without any config, reproduced): crates/tcpinfo/build.rs printing
  `cargo::rustc-env=CLIPPY_ARGS=--cap-lints=warn__CLIPPY_HACKERY__`. layoutProblems (check.mjs:516-543) allows a
  custom-build target inside the folder, and no rule reads what a build script emits. Scratch atkH (with atkF's
  undocumented two-op block, config restored): deps PASS; `cargo clippy -v -p npt-tcpinfo --all-targets -- -D
  warnings` runs build-script-build and ends with "warning: unsafe block missing a safety comment", no error.
  No member has a build script or proc-macro today (cargo metadata --no-deps, 16 members), so refusing
  custom-build targets costs nothing now.
- Not a route (probe): frontmatter (`---` block) is "E0658 frontmatters are experimental" in rustc 1.99.1.
- CI (.github/workflows/gate.yml) does not run check.mjs and sets no RUSTFLAGS/RUSTC_WRAPPER: no CI false positive.
- F8 revert (scratch revF8: srtt_us's `require_tcp(...)?;` -> `.ok();`): `cargo test -p npt-tcpinfo` 7 passed,
  2 FAILED: every_descriptor_that_is_not_a_tcp_socket_is_an_error (tests.rs:171, left Unsupported right InvalidInput;
  any Linux host) and an_mptcp_socket_is_not_tcp_for_either_reading (tests.rs:212, Ok where Err expected). Unmodified
  copy: 9/9, MPTCP present here (no skip line).
- F8 SAFETY citations spot-checked against v6.8 (raw.githubusercontent.com torvalds/linux v6.8, saved in
  scratchpad/c2review/k68/): net/socket.c:2364-2365 (SOL_SOCKET -> sk_getsockopt), :2397-2399 + :553-567
  (sockfd_lookup_light -> ENOTSOCK), net/core/sock.c:1609 (lv = sizeof(int)), :1652-1662 (SO_TYPE/PROTOCOL/DOMAIN
  are ints), :1999-2001 (len = min(len, lv); copy_to_sockptr). All hold. Nuance, not in the comments: the cgroup-BPF
  getsockopt hook (socket.c:2377-2380) runs after and may rewrite value/len, bounded by the caller's optlen
  (max_optlen), so the 4-byte write bound holds; a root-installed BPF program could make a non-TCP socket read as
  TCP, but 0x5411 on any socket stays in socket ioctl code (writes one int or ENOTTY): soundness unaffected. Nit.
- F-N bite and revert (scratch revFN): stub's non-Linux `pub use` broken -> `node check.mjs --only clippy-wasm32`
  FAIL (E0432). Then WASM_CRATES reverted to [...LEAF] (check.mjs:786) -> clippy-wasm32 PASS on the broken stub, and
  self 29/30 (check.test.mjs:668-671 fails). Fixed. npt-net's not(unix) handle (tcp_kernel.rs:39-42) stays unbuilt:
  listed in u3c-fix-c2.md:126-127 for the main session.
- Wiring gap (reproduced, scratch revWire): the four new deps-step calls removed from STEPS.deps
  (check.mjs:837-840: workspaceLintProblems, layoutProblems, links, cargoConfigProblems) -> `node check.mjs --only
  self,deps`: self 30/30, deps PASS. The self-tests call each function directly; nothing committed proves the deps
  step calls them (C2's realplant.py did, as a scratch script). A revert of the wiring would go unnoticed.
- S5 (c) revert (scratch revS5: isPlacedAllow = PLACED_ALLOW.test; the crates/<folder>/Cargo.toml test disabled):
  self 28/30, failing check.test.mjs:317 ("only the exact allow with its reason counts") and :557 (layout). Fixed.
- F-J (scratch git repo scratchpad/c2review/fj, script imported with python3 -I -B, no iptables, no probe):
  (1) a new siocoutq-root-check-*.txt only -> "spike commit <sha>" (clean); (2) + crates/room/src/planted.rs ->
  "(with uncommitted changes: spikes/netcode-proof/crates/room/src/planted.rs)"; (3) an EDIT to the committed log
  siocoutq-root-check-20261108T005258Z.txt -> clean (the exclude pathspec hides tracked edits too); (4) a stray
  siocoutq-root-check.py.bak -> reported. Fixed as asked. Nits: (3); and no committed test (scratch fj_check.py only).
- C2's bites.py re-pointed at the 81203da copy (scratchpad/c2review/bites_rerun.py, writes only to c2review/mut):
  control exit 0; 27 of 27 mutations CAUGHT. The unit tests bite every rule as written; the bypasses above are what
  the rules do not model (rustc's lexer, TOML key escapes, unread config inputs), plus the unwired-deps-step gap.
- Fix check: root `unsafe_code = "forbid"` in the atkA and atkD copies -> `cargo check -p npt-room`: "error[E0453]:
  allow(unsafe_code) incompatible with previous forbid" + "usage of an `unsafe` block" in both. An inherited forbid
  (npt-tcpinfo given its own [lints] table) closes A-D at the compiler, whatever the lexer misreads; the token scan
  becomes defence in depth. Only --cap-lints/--force-warn (rustflags, F's include, a wrapper) and a proc macro defeat it.
- Observation outside the assigned findings (check.mjs test step): CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUNNER=true
  `cargo test -p npt-log` exits 0, the unit-test binary is "Running" but prints no result (only doctests' "0 passed");
  cargoConfigProblems({..._RUNNER: "true"}, []) -> []. testBothBuilds (check.mjs:799-805) judges exit status only, so a
  runner (env or `target.*.runner` in a config) would make the test step pass vacuously. Low.
- Regression checks: the 81203da copy passes `node check.mjs --only fmt,clippy,clippy-wasm32,deps,self,redact` (all 6);
  the real tree passes deps and self. The old RELAX scan also read raw text (95597ef check.mjs:332), so comments are no
  new false positive. npt-net passes srtt_us errors through unchanged (crates/net/src/tcp_kernel.rs:23).

## Verdicts
- S1: PARTLY FIXED. The routes the review reproduced are closed: comment in the list, raw identifiers, the macro-built
  allow (through the token scan), RUSTFLAGS / config rustflags with -A, --cap-lints, --force-warn, the clippy/fmt
  aliases, the emoji char literal, the macro-built #[path], proc-macro members, outside path dependencies, and the
  tcpinfo clippy-lint relaxing. Tests: check.test.mjs:227, :366, :405, :433, :481, :557, :599; 27/27 mutations
  re-run here. NOT closed, same class, all reproduced with deps PASS (and clippy where it matters): A (shebang), B
  (`\u{}` with underscores), C (escaped `..` in #[path] into a client-wasm file), D (renamed include!), F (config
  `include`), G ([env] CLIPPY_ARGS), H (build.rs rustc-env CLIPPY_ARGS). Cargo.toml:72-79 and DEPS_CHECKED
  (check.mjs:812-820) claim the property these break.
- F-D: PARTLY FIXED. Plain relaxed forms fail (check.test.mjs:519-539; M11, M11b), but E (escaped key + decoy inside
  a multi-line string) passes deps while cargo passes --allow=unsafe_code. The deps-step wiring has no test.
- F-E: FIXED for its route ([lib] path plant -> deps FAIL; check.test.mjs:557-597, :636-666; M12-M12f, M15, M15b).
  Related path-rule routes C and D are under S1. Wiring untested.
- F-N: FIXED (bite + revert above; check.test.mjs:668-671). npt-net's not(unix) half is carried.
- F8: FIXED. Four blocks (linux.rs:53, :82, :126, :152), one operation each, SAFETY citations verified against v6.8,
  lengths bounded (int_answer :171 requires exactly 4; rtt_from :204-217), all inside the two allowed bodies. Revert:
  tests.rs:171 and :212 fail. Nits: the MPTCP test returns as a pass when MPTCP is absent (tests.rs:203-208); the BPF
  hook nuance; 3 extra getsockopt calls per srtt_us (linux.rs:64-69); plan text should say four blocks (S6).
- F-J: FIXED (cases 1-4 above). Nits: tracked edits of committed logs are hidden too; no committed test.
- S5 (c): FIXED (check.test.mjs:317, :557 fail on revert). (a), (b), (d): U6 carries (u3c-fix-c2.md:113-117).

## Fix advice (for the main session)
1. Compiler-level backstop (shown here: E0453 on A and D): `[workspace.lints.rust] unsafe_code = "forbid"`, with
   npt-tcpinfo given its own [lints] table (deny + the workspace's clippy lints, checked equal by check.mjs); or a
   per-crate `-F unsafe_code` pass over every crate except the two exceptions.
2. lexRust: refuse a .rs file whose first line (after an optional BOM) starts with `#!` and is not an inner
   attribute; accept `\u\{[0-9a-fA-F_]+\}` (a superset of rustc's). Self-tests with A's and B's text.
3. pathProblems: decode the literal's escapes (or refuse a backslash) before resolving; refuse `include` named
   anywhere in code but as `include!(` (a `use ... include` or a macro argument). Tests with C's and D's text.
4. workspaceLintProblems: read headers and keys from tomlScan's mask (strings blanked) and decode quoted keys, or
   refuse a backslash in a quoted key. Test with E's text.
5. cargoConfigProblems / layoutProblems: refuse `include` in a cargo config (or follow it), refuse `[env]` (the spike
   sets none) or at least CLIPPY_* / RUSTC* keys, refuse custom-build targets (none exist), and refuse `runner`
   (env CARGO_TARGET_*_RUNNER or config) or have the test step require a non-zero passed count.
6. One test that runs the deps step's own composition on a planted fixture, so unhooking a check fails a test.
7. RUSTC_WRAPPER: leaving it (sccache) is reasonable; RUSTC, RUSTC_WORKSPACE_WRAPPER, build.rustc*,
   build.rustc-workspace-wrapper and a rust-toolchain.toml `path` are rarely legitimate and cheap to refuse.
8. Until 1-5 land, reword Cargo.toml:72-79 and DEPS_CHECKED to what is checked, naming what is not.

Scratch kept (not deleted): scratchpad/c2review/ (base, atkA..atkH, atkActl, atkBctl, revF8, revFN, revWire, revS5,
plFE, fj, k68, mut, bites_rerun.py, fj_probe.py); build outputs in .gate/u3c-review-target (shared) and
.gate/u3c-review-target/c2fm (one .rmeta probe dir, empty on error). No source file in the main tree was edited.
DONE
