# U3c last fix round, part C2d: check.mjs nits N1-N5 (test-engineer's C2c verdict)

Agent: workflow fix agent C2d, worktree .claude/worktrees/wf_b13b9984-6bc-1 at ecdeeb0 (spike branch
claude/netcode-proof-test-plan-hcky5g). Inputs: u3c-fix3-review-c2c.md (N1-N5), u3c-fix3-c2c.md.
Rules: cargo +1.99.1 with CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/u3c-review-target only; touch every .rs
under spikes/netcode-proof/crates before the first build; df -h /home/user before each build (stop under 1.5 GB);
no iptables/tc, no #[ignore]d evidence tests, no network but N5's two reads; commit nothing; no git -C / -c;
Write/Edit tools only.

## Plan
1. N2: inheritsWorkspaceLints reads tomlScan(text).mask lines (strings and comments blanked); unsafeProblems' manifest
   line scan too. Tests: `[lints]\nworkspace = true` moved into a basic and into a literal multi-line string -> fails.
   Then list every TOML/Rust line reader in check.mjs, masked or not; move any unmasked TOML reader onto the mask.
2. N1: a member plant in the C2R-5 depsFixture (rustflags in crates/room/Cargo.toml), asserted in the bad run.
3. N3: refuse __CARGO_TEST_CHANNEL_OVERRIDE_DO_NOT_USE_THIS in the environment (any value), test; name the class
   (other ways to unlock unstable features on stable) in the known limits above DEPS_CHECKED; DEPS_CHECKED line.
4. N4: strip one leading UTF-8 BOM in tomlScan (the one place every TOML reader goes through), test (a line-1
   rustdoc lint key after a BOM refused; manifestKeyProblems unchanged).
5. N5: one line in unacked_and_unsent's SOL_SOCKET SAFETY comment on non-inet families that answer 0x5411
   (AF_UNIX, AF_PACKET), cited from v6.8 sources (local copies, else git.kernel.org/github at v6.8 if no prompt).
6. Bites: revert each fix (N1-N4) once, run its test, record how it fails, restore.
7. Final: check.mjs 9/9 twice; clippy -D warnings with/without --all-features; cargo fmt --all --check.

## Progress
- Start: worktree fast-forwarded to ecdeeb0 (git log -1 checked). df -h /home/user 4.9G free; the 62 crate .rs files
  touched (mtime only) before any build. Baseline node --test check.test.mjs 41/41. Env names here as in C2c's notes;
  no __CARGO_TEST_CHANNEL_OVERRIDE_DO_NOT_USE_THIS, no RUSTC_BOOTSTRAP. No file under spikes/netcode-proof holds a BOM.
- N5 sources: af_unix.c and af_packet.c were not in the local copies; fetched at v6.8 from raw.githubusercontent.com
  (torvalds/linux, one curl, went through with no prompt) into scratchpad c2d-linux-v6.8/ (net_unix_af_unix.c sha256
  a5ecafbd...e9, 3702 lines; net_packet_af_packet.c 0dd5e3a5...36, 4786 lines). unix_ioctl :3036, SIOCOUTQ :3043-3045
  `put_user(amount, (int __user *)arg)` (amount a long, written as one int); packet_ioctl :4213, SIOCOUTQ :4219-4223
  `int amount = sk_wmem_alloc_get(sk); return put_user(amount, (int __user *)arg)`. socket.c :1222 / :1232 re-checked
  (u3cbfix copy = c2review/k68/socket.c, cmp equal).
- Before any fix (scratchpad c2d/repro.mjs, output c2d/repro-before.txt): N2 basic and literal plants -> [] (passes);
  name scan with `{ a = "#", unsafe_code = 1 }` -> []; N3 env -> []; N4 BOM + rustdoc dotted key -> []. All reproduced.
- Item 1 (N2) done: inheritsWorkspaceLints(text) reads tomlScan(text).mask lines; unsafeProblems' manifest name scan
  reads tomlScan(text).clean (see the reader list below for why clean, not mask). New test "a crate's `[lints]
  workspace = true` inside a multi-line string inherits nothing, basic or literal (review C2c N2)": both plants ->
  "does not inherit"; CRLF + trailing comments still inherit; `#` in a string no longer hides the name. node --test
  42/42.
- N2 reader list (grep of check.mjs's readers: split("\n"), match/matchAll/test, readFileSync, read()), reader by reader:
  TOML:
  * inheritsWorkspaceLints: was raw lines with a `#.*$` regex strip -> now tomlScan(text).mask (N2's fix).
  * unsafeProblems' member-manifest name scan (UNSAFE_CODE_NAME): was the same raw lines -> now tomlScan(text).clean
    (comments blanked by the scanner, strings kept). Not the mask on purpose: it refuses any mention, so text in a
    string counts and fails closed (it cannot be fooled the N2 way), while the mask would blank a quoted key
    (`"unsafe_code" = ...`, the lint to cargo). The raw regex strip could be fooled the other way: a `#` inside a string
    cut the rest of the line (`note = { a = "#", unsafe_code = 1 }` read as nothing); test added.
  * lintTables (workspace and exception lint tables): mask for structure, tomlKeyAt decodes quoted parts. Masked.
  * workspaceLintProblems' line scan: tomlScan mask. Masked (a quoted key in a lint table is lintTables' refusal).
  * tomlValues (cargoConfigProblems' rustflags/alias/compiler/include/runner keys, manifestKeyProblems'
    cargo-features/rustflags): mask for bare keys, the strings list for quoted keys and values. Masked.
  * cargoConfigProblems' `[[include]]` header regex: on the mask, so a quoted header part was blanked:
    `[["include"]]`, `[[ 'include' ]]` and `[["include"]]` passed (probe: cargo +1.99.1 in scratchpad
    c2d/incprobe/quoted with `[["include"]]` loading ../../extra.toml's `[alias] zz = "version"`: `cargo zz` printed
    "cargo 1.99.1 (5f94df478 2026-08-27)", exit 0; plain [[include]] the same; no include: "no such command", 101; the
    single-table `["include"]` and dotted `include.path` are cargo's own error "expected a list of strings or a list
    of tables", 101). Fixed: tomlHeaderHeads reads each header line's first name with tomlKeyAt (decoded), every line
    opening `[`/`[[` (an array line that only looks like one counts too: fails closed). Test lines added to the
    C2R-4/7/8 test (four forms refused; a string and `["included"]` not).
  * cargoConfigFiles / depsStep read(): whole files, handed to the readers above. layoutProblems: cargo metadata JSON.
  Rust (lexRust's code = comments blanked, mask = comments and literals blanked):
  * unsafeProblems .rs: RELAX_UNSAFE on the raw text OR the mask (raw on purpose: a relaxing named in a comment or a
    string is reported too, fail closed; the mask catches a list with a comment inside); UNSAFE_CODE on the mask.
  * tcpinfoProblems: attributes found in the mask; RELAX_UNSAFE counted in code (literals kept) to report one in a
    string or macro text outside attributes (fail closed); `unsafe_code`, UNSAFE_CODE and the clippy names on the mask.
  * rustAttributes, itemAfter, fnBodies, rootDeny: mask. pathProblems: keys in the mask, the literal read from code at
    that spot (by design: its value is needed). isPlacedAllow: the attribute body from code (the reason's text).
  * lexRust's own known limit (shebang, long `\u{...}`: C2R-1) is unchanged and named above DEPS_CHECKED.
  Other: referenceProblems reads non-spike files raw (`spikes/` anywhere counts: fail closed); marks rows and cargo
  output are not TOML or Rust.
- Item 2 (N1) done: depsFixture's bad run plants `[profile.dev]\nrustflags = ["--cap-lints", "warn"]` in
  crates/room/Cargo.toml and asserts "crates/room/Cargo.toml has a key `rustflags`: ..." among the lines. 42/42.
- Item 3 (N3) done: UNSTABLE_ENV { RUSTC_BOOTSTRAP (message unchanged), __CARGO_TEST_CHANNEL_OVERRIDE_DO_NOT_USE_THIS }
  refused whatever the value (Object.hasOwn, so an env name like `constructor` is no match); cargoConfigProblems doc,
  manifestKeyProblems doc, the known-limits bullet above DEPS_CHECKED (the class: any other way to unlock unstable
  features on a stable toolchain, another variable or a patched/rebuilt cargo or rustc, is out of reach) and the
  DEPS_CHECKED line ("no RUSTC_BOOTSTRAP or cargo channel override"). Test lines in the C2b F5 test (renamed with C2c
  N3): four values refused; look-alike names and `constructor` not. No false positive: cargoConfigProblems(process.env)
  = [] here (repro.mjs). Not edited (outside this part's files): the workspace Cargo.toml comment's list ends "... test
  runner or RUSTC_BOOTSTRAP" (still true; it points at DEPS_CHECKED's comment for the rest).
- Item 4 (N4) done: tomlScan blanks one leading U+FEFF (charCodeAt(0) === 0xfeff -> a space in clean and mask), so
  offsets and lines stay the text's; every TOML reader goes through tomlScan now (N2 moved the last two). New test "one
  byte order mark before line 1 is read past, as cargo 1.99.1 reads it (review C2c N4)": the reviewer's rustdoc plant
  -> ":1 gives lints as a dotted key"; a workspace root dotted key on line 1 -> read; no FP on the real manifests,
  npt-sim's, manifestKeyProblems with a BOM. (Edit-tool note: a `﻿` escape typed into an edit was written as the
  raw BOM bytes, so the code compares the char code and the test builds it with String.fromCharCode; no EF BB BF byte
  in either file, checked with grep -P.) node --test 43/43.
- Item 5 (N5) done, comment only: unacked_and_unsent's SOL_SOCKET SAFETY comment gains: a family that answers 0x5411
  itself writes one int too (the ioctl's `int *` convention), so no more than `answ`: AF_UNIX's unix_ioctl and
  AF_PACKET's packet_ioctl each put_user it through an `int __user *` (v6.8 af_unix.c:3043-3045,
  af_packet.c:4219-4223, read in the fetched copies above); which other families answer it was not enumerated and
  none was read: the comment says they are taken on convention. No man page was read for "convention".
  Lines <= 120 wide (the file's two pre-existing 121-wide lines, :57 and :134, untouched).
- Item 6 (bites): scratchpad c2d/bites.mjs, output c2d/bites-out.txt: 6 of 6 CAUGHT, control 43/43 before and after,
  check.mjs sha256 20cd539c...0a0d before and after. Each revert, the test that fails and how:
  * N2a inheritsWorkspaceLints back on raw lines -> the C2c N2 test: `"""` plant returns [] (expected "does not
    inherit").
  * N2b name scan back on regex-stripped lines -> the C2c N2 test: `{ a = "#", unsafe_code = 1 }` returns []
    (expected "sets unsafe_code").
  * N2c include header back on the mask regex -> the C2R-4/7/8 test: `[["include"]]` returns [] (expected the include
    refusal).
  * N1 depsStep's member-manifest loop deleted -> the C2R-5 test: "manifestKeyProblems (a member)'s plant is not
    reported".
  * N3 override dropped from UNSTABLE_ENV -> the C2b F5 / C2c N3 test: "nightly" returns [] (expected the refusal).
  * N4 BOM not blanked -> the C2c N4 test: the rustdoc plant returns [] (expected ":1 gives lints as a dotted key").
  * N5: comment only, no test possible.
- End to end (scratchpad c2d/e2e.mjs, output c2d/e2e-out.txt; `node check.mjs --only deps` in the worktree, cargo
  metadata only, CARGO_TARGET_DIR the review target; files restored, sha256 checked, extra.toml removed; disk 4.9G):
  control PASS (78 crate files, 1 config); N2 `"""` and `'''` plants FAIL "crates/sim/Cargo.toml does not inherit";
  N3 env FAIL with its line; N4 BOM + rustdoc dotted key FAIL "crates/tcpinfo/Cargo.toml:1 gives lints as a dotted
  key"; N4 BOM alone PASS (no FP); `[["include"]]` + .cargo/extra.toml FAIL with the include line, and `cargo zz` in
  that state printed "cargo 1.99.1 (5f94df478 2026-08-27)" (cargo loaded the included file); control after PASS.
- Expected Rust test counts: f6cd1d7 ran 260 / 274 (task list); f6cd1d7..ecdeeb0 touches no .rs file under the
  spike (diff --stat: results only), and this part adds no Rust test, so 260 / 274 again.
- Item 7 (final). All 62 crate .rs touched before each pair; disk 4.9G, 4.5G, 4.5G, 4.8G free before the builds.
  * check.mjs run 1 (22:21:14-22:23:00 UTC) and run 2 (22:23:10-22:24:25): 9/9 each; test measured 260 / bites 274,
    0 failed, 0 skips (= expected); deps 78 crate files and 1 config; self 43/43 (41 + 2 new tests); redact 10/10,
    149 items (148 at C2c; ecdeeb0 added one results file). Outputs scratchpad c2d/check-run1.txt, run2.txt.
  * Then a string-only tidy of DEPS_CHECKED, so again: bites 6 of 6 CAUGHT (c2d/bites-out-final.txt; control 43/43
    before and after; check.mjs sha256 73c97dc1...2937 restored), touch, check.mjs run 3 (22:25:23-22:26:44) and run 4
    (22:26:51-22:28:05): 9/9 each, same counts (c2d/check-run3.txt, run4.txt).
  * Explicit, after touching npt-tcpinfo's .rs (re-checked it and its dependents: 5 and 4 "Checking" lines):
    cargo +1.99.1 clippy --locked --workspace --all-targets -- -D warnings exit 0; the same with --all-features exit 0;
    cargo +1.99.1 fmt --all --check exit 0 (also after the linux.rs edit, before run 3).
  * Final files: check.mjs 73c97dc1...2937, check.test.mjs 9b535a29...b076, linux.rs db5cbc73...767d; git diff --stat
    3 files, +140 -25; HEAD still ecdeeb0; nothing committed.
- Not done / limits: the workspace Cargo.toml comment (line 83) still names RUSTC_BOOTSTRAP alone among the env refusals
  (true, incomplete; outside this part's files). No man page read for N5's "convention"; families other than AF_UNIX
  and AF_PACKET not read. The include gap was found in the N2 reader sweep, not in the reviewer's list: a critic should
  look at it as new code (tomlHeaderHeads).
DONE
