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
