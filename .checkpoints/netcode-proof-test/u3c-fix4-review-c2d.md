# test-engineer review: U3c C2d fix (uncommitted on ecdeeb0)

Started 2026-11-08. Worktree /home/user/Shamble-Frontlines/.claude/worktrees/wf_b13b9984-6bc-1 (read only, never
edited). Scratch copy: /home/user/Shamble-Frontlines/.gate/c2d-review-scratch. CARGO_TARGET_DIR=
/home/user/Shamble-Frontlines/.gate/u3c-review-target only. Answers my C2c review (u3c-fix3-review-c2c.md N1-N5).
Scripts: scratchpad c2d-review/ (c2c-review/ for the old ones).

## Plan
1. df; git status / git diff in the worktree (3 files); read the fix agent's notes u3c-fix4-c2d.md.
2. Touch the worktree's crate .rs, reproduce `node spikes/netcode-proof/check.mjs` inside the worktree first;
   record test counts, deps/self counts.
3. Copy spikes/netcode-proof to .gate/c2d-review-scratch; control self tests.
4. N2: rerun my n2 plant (basic + literal) through deps; revert inheritsWorkspaceLints mask -> test must fail;
   revert the unsafe_code name scan clean -> test must fail; revert tomlHeaderHeads -> test fails. Check the reader
   sweep (grep every TOML/Rust raw reader). Look for false positives on committed manifests.
5. N1: delete member-manifest loop in scratch -> a test must fail.
6. N3: revert the env refusal -> test fails; check no FP on this container env and `rustup run`.
7. N4: rerun BOM rustdoc plant -> deps FAIL; revert BOM strip -> test fails; BOM on real manifests passes.
8. N5: read the comment, check cites against the fetched v6.8 copies (scratchpad c2d-linux-v6.8/ + sha256s).
9. Claims in check.mjs / manifests made false? DEPS_CHECKED text, workspace Cargo.toml comment (not_fixed item).
10. Final worktree run after touching .rs; verdict.

## Findings (as I go)
- Step 1: df 4.8G free. git status: 3 files modified (check.mjs, check.test.mjs, linux.rs) + the fix agent's notes
  untracked; HEAD ecdeeb0. Diff +140 -25 read in full. File sha256s = the fix agent's finals (check.mjs
  73c97dc1...2937, check.test.mjs 9b535a29...b076, linux.rs db5cbc73...767d).
- Step 2 (22:29:26-22:30:50 UTC, after touching the worktree's 62 crate .rs, mtime only): `node check.mjs` in the
  worktree exit 0, 9/9: test 260 measured / 274 bites, 0 failed, 0 skips (= fix agent's); deps PASS 78 crate files +
  1 config; self 43/43; redact 10/10, 149 items. Output .gate/check-c2d-review-1.txt.
  (Process note: the two lines above were first appended with a shell heredoc; from here on, the Write tool only. No
  Edit tool in this session, so the file is rewritten whole each time.)
- Step 3: scratch copy .gate/c2d-review-scratch/netcode-proof (3.1 MB, cp -a, 149 files; check.mjs sha256
  73c97dc1...2937 = the worktree's); control `node --test check.test.mjs` 43/43.
- Revert battery (scratchpad c2d-review/bites.mjs, out bites-out.txt; control 43/43 before and after; check.mjs sha
  73c97dc1...2937 after): 10 of 12 CAUGHT.
  CAUGHT (each 42/43): N2a mask -> raw regex-stripped lines, and mask -> clean (the C2c N2 test, check.test.mjs:276);
  N2b name scan -> raw lines (same test); N2c tomlHeaderHeads -> the old mask regex (C2R-4/7/8 test, :944-946); N1
  member loop deleted (C2R-5 test, :1040 plant / :1062-1065 row); N3 override dropped, hasOwn -> `in`, and the old
  `k === "RUSTC_BOOTSTRAP"` line (F5/N3 test, :957-970); N4 BOM not blanked, and BOM blanked with `i` left at 0 (C2c N4
  test, :681).
  MISSED (both design details beside a fix, not a fix reverted): (a) the name scan on `mask` instead of `clean`
  (check.mjs:480): the comment's reason ("a quoted key is the lint to cargo", :477-479) has no test; (b)
  tomlHeaderHeads taking `[[` only (single `[include]` skipped, check.mjs:861): no test. Neither opens a way round: in
  scratch (plants.mjs), `[lints]\nworkspace = true\n[lints.rust]\n"unsafe_code" = "allow"` in npt-sim is cargo's own
  error ("cannot override `workspace.lints` in `lints`", metadata exit 101), and a crate without workspace = true is
  refused by inheritsWorkspaceLints; the fix agent's probe has `["include"]` as cargo's error. Nits only.
- Plants through `check.mjs --only deps` + `cargo +1.99.1 metadata --no-deps --offline` in scratch (plants.mjs, out
  plants-out.txt; all files restored, 149 sha256 equal to before, created .cargo/extra.toml removed): all 15 as
  expected. N2 basic and literal plants: deps FAIL "crates/sim/Cargo.toml does not inherit" while cargo metadata exit 0.
  N2 probe, TOML 1.1 multi-line inline table `[lints]\nrust = {\n  workspace = true\n}` (inheritsWorkspaceLints reads
  that line as the inherit): cargo's own error ("invalid type: boolean `true`", exit 101), so no way round. N1 member
  rustflags FAIL. N3 override=nightly alone FAIL with its line; with cargo-features in the workspace manifest FAIL on
  both (metadata exit 0 there: cargo accepted it). N4: BOM + line-1 rustdoc dotted key FAIL ":1 gives lints as a
  dotted key"; BOM + line-1 `[lints.rustdoc]` header FAIL (hyphen name); a BOM on all 16 member manifests + the
  workspace manifest + .cargo/config.toml PASS, metadata 0 (no FP); BOM + line-1 `[[include]]` FAIL. N2c
  `[["include"]]` and `[[\t'include'\t]]` FAIL. Controls before/after PASS.
- Reran my C2c reproductions on the fixed scratch copy (outputs c2d-review/probes-rerun-out.txt,
  probes2-rerun-out.txt, n2-e2e-rerun-out.txt): probes: BOM + rustdoc dotted key now FAIL at :1 (was PASS);
  override=nightly alone now FAIL (was PASS); override still makes stable cargo accept cargo-features (metadata 0),
  which manifestKeyProblems refuses; multi-line inline table FAIL as before. probes2: BOM + all-inline lints now FAIL
  at :1 "dotted key or inline table" (before: only because tables went missing); BOM + cargo-features FAIL on both
  lines. n2-e2e (22:38 UTC, scratch build of npt-sim in the review target): A committed manifest + shebang plant: deps
  PASS, cargo check exit 101 with --forbid=unsafe_code (the compiler backstop); B [lints] in a string: deps now FAIL
  "does not inherit" while cargo check exit 0 with no lint flag. N2's way round is closed. Scratch restored (sim/src
  holds lib.rs only; 149 sha256 equal).
- N3 FP: this container's env has CARGO_HOME, CARGO_HTTP_CAINFO, RUSTUP_HOME, RUST_BACKTRACE (+ RUSTUP_TOOLCHAIN,
  RUST_RECURSION_COUNT under `rustup run 1.99.1`); none matches UNSTABLE_ENV; ~/.cargo has no config. `rustup run
  1.99.1 node check.mjs --only deps` in scratch: PASS. No CI workflow runs check.mjs (.github/workflows has no
  netcode-proof reference). Deps PASS in my step-2 run.
- N2 reader sweep, checked by my own grep of check.mjs (split("\n"), matchAll, .test, readFileSync, read()): TOML
  readers at :428 (inheritsWorkspaceLints, mask), :480 (name scan, clean), :564/:578 (lintTables, mask + tomlKeyAt),
  :676 (workspaceLintProblems, mask), :857 (tomlHeaderHeads, mask + tomlKeyAt), :939 / :976 (cargoConfigProblems,
  manifestKeyProblems via tomlValues: mask + strings). No raw TOML reader left. Raw readers left: referenceProblems
  (any file, `spikes/` substring, fail closed by design), marks (docs/ADR rows, not TOML), lexRust's .rs readers
  (raw for RELAX_UNSAFE on purpose: fail closed; pathProblems/isPlacedAllow read literal values from code by design).
  The fix agent's list matches. rust-toolchain.toml, deny.toml, clippy.toml are not read by check.mjs (known limit
  :1146-1148 names rust-toolchain.toml).
- N5 (comment, linux.rs:140-144): cites checked in the fetched copies (scratchpad c2d-linux-v6.8/, sha256
  a5ecafbd...67e9 af_unix.c 3702 lines, 0dd5e3a5...1636 af_packet.c 4786 lines: equal to the fix agent's notes):
  af_unix.c:3036 unix_ioctl, :3043-3045 `case SIOCOUTQ: amount = unix_outq_len(sk); err = put_user(amount, (int
  __user *)arg);` (amount a long, put_user writes sizeof(int)); af_packet.c:4213 packet_ioctl, :4219-4223 `int amount
  = sk_wmem_alloc_get(sk); return put_user(amount, (int __user *)arg);`. Lines exact. Neither file is in the local
  v6.8 copies (find: none), as the notes say. I did not refetch them (no network use in this review), so their v6.8
  provenance rests on the fix agent's curl from raw.githubusercontent.com/torvalds/linux at v6.8. The comment says the
  other answering families were not read and are taken on convention; the notes say no man page was read for the
  `int *` convention, and the comment cites none: consistent. Width <= 120 on the new lines (the two 121-wide lines,
  :57 and :134, predate C2d).
- Claims: DEPS_CHECKED ("no RUSTC_BOOTSTRAP or cargo channel override", check.mjs:1171) and the known-limits bullet
  (:1148-1153) are true as the code reads; tomlScan's "every TOML file check.mjs reads goes through here" (:771-772)
  holds by the sweep. The workspace Cargo.toml comment (:83, "... test runner or RUSTC_BOOTSTRAP") is still true,
  incomplete (no channel override); it points to DEPS_CHECKED's comment for the rest. Not made false: a nit.
- Final worktree run (22:39:10-22:40:30 UTC, after the scratch build: all 62 crate .rs touched again; df 4.9G):
  check.mjs exit 0, 9/9, test 260 / 274, 0 failed, 0 skips (same counts), deps PASS, self 43/43, redact 10/10 149
  items; output .gate/check-c2d-review-2.txt. Then (62 .rs touched again) cargo +1.99.1 clippy --locked --workspace
  --all-targets -- -D warnings exit 0; the same with --all-features exit 0; cargo +1.99.1 fmt --all --check exit 0.
  Worktree content unchanged (git diff --stat 3 files +140 -25; the three sha256s as at start; HEAD ecdeeb0).

## Verdict: complete. N1-N5 done; every code fix bites when reverted; no gap, no regression, no false positive found.

| Item | Claim | Proof (reverted in scratch / reproduced) | Gap |
|---|---|---|---|
| N2 | `[lints] workspace = true` in a multi-line string, basic or literal, inherits nothing | test :276 fails on mask -> raw and mask -> clean; plants deps FAIL; n2-e2e B deps FAIL (cargo: no lint flag) | none |
| N2 sweep 1 | member name scan reads past `#` in a string | :276 fails on the raw revert | `clean` vs `mask` choice untested (nit, no bypass) |
| N2 sweep 2 | `[["include"]]` and other quoted heads refused | :944-946 fails on the old regex; plants deps FAIL (quoted, tabs, after BOM) | single `[include]` untested (cargo's error) |
| N1 | the member-manifest loop is proven wired | C2R-5 test fails on deletion (:1040, :1062-1065) | none |
| N3 | channel override refused, any value; class named | :957-970 fails on 3 reverts; deps FAIL; no FP here or under rustup run | none |
| N4 | one leading BOM read past in every TOML read | :681 fails on 2 reverts; plants FAIL; BOM on all 18 TOML files PASS | none |
| N5 | non-inet families that answer 0x5411 write one int | af_unix.c:3043-3045, af_packet.c:4219-4223 match the fetched copies | others taken on convention (stated) |

Ranked nits (none blocking): (1) check.mjs:477-480 the `clean` choice has no test; (2) check.mjs:861 a single
`[include]` head has no test; (3) Cargo.toml:83 names RUSTC_BOOTSTRAP only.
DONE
