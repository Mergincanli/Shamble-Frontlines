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
  untracked; HEAD ecdeeb0. Diff +140 -25 read in full.
- Step 2 (22:29:26-22:30:50 UTC, after touching the worktree's 62 crate .rs, mtime only): `node check.mjs` in the
  worktree exit 0, 9/9: test 260 measured / 274 bites, 0 failed, 0 skips (= fix agent's); deps PASS 78 crate files +
  1 config; self 43/43; redact 10/10, 149 items. Output .gate/check-c2d-review-1.txt.
  (Process note: the two lines above were first appended with a shell heredoc; from here on, the Write tool only. No
  Edit tool in this session, so the file is rewritten whole each time.)
- Step 3: scratch copy .gate/c2d-review-scratch/netcode-proof (3.1 MB, cp -a, 149 files; check.mjs sha256
  73c97dc1...2937 = the worktree's = the fix agent's final); control `node --test check.test.mjs` 43/43.
- Revert battery (scratchpad c2d-review/bites.mjs, out bites-out.txt; control 43/43 before and after; check.mjs sha
  73c97dc1...2937 after): 10 of 12 CAUGHT.
  CAUGHT (each 42/43): N2a mask -> raw regex-stripped lines, and mask -> clean (the C2c N2 test); N2b name scan -> raw
  lines (C2c N2 test); N2c tomlHeaderHeads -> the old mask regex (C2R-4/7/8 test); N1 member loop deleted (C2R-5
  test); N3 override dropped, hasOwn -> `in`, and the old `k === "RUSTC_BOOTSTRAP"` line (F5/N3 test); N4 BOM not
  blanked, and BOM blanked with `i` left at 0 (C2c N4 test).
  MISSED (both design details beside a fix, not a fix reverted): (a) the name scan on `mask` instead of `clean`
  (check.mjs:480): the comment's reason ("a quoted key is the lint to cargo") has no test; (b) tomlHeaderHeads taking
  `[[` only (single `[include]` skipped, check.mjs:861): no test. Neither opens a way round: in scratch (plants.mjs),
  `[lints]\nworkspace = true\n[lints.rust]\n"unsafe_code" = "allow"` in npt-sim is cargo's own error ("cannot override
  `workspace.lints` in `lints`", metadata exit 101), a crate without workspace = true is refused by
  inheritsWorkspaceLints; and the fix agent's probe has `["include"]` as cargo's error. Nits only.
- Plants through `check.mjs --only deps` + `cargo +1.99.1 metadata --no-deps --offline` in scratch (plants.mjs, out
  plants-out.txt; all files restored, 149 sha256 equal to before, created .cargo/extra.toml removed): all 15 as
  expected. N2 basic and literal plants: deps FAIL "crates/sim/Cargo.toml does not inherit" while cargo metadata exit 0.
  N2 probe, TOML 1.1 multi-line inline table `[lints]\nrust = {\n  workspace = true\n}` (inheritsWorkspaceLints reads
  that line as the inherit): cargo's own error ("invalid type: boolean `true`", exit 101), so no way round. N1 member
  rustflags FAIL. N3 override=nightly alone FAIL with its line; with cargo-features in the workspace manifest FAIL on
  both (metadata exit 0 there: cargo accepted it). N4: BOM + line-1 rustdoc dotted key FAIL ":1 gives lints as a
  dotted key"; BOM + line-1 `[lints.rustdoc]` header FAIL (hyphen name); BOM on all 17 manifests + workspace + config
  PASS, metadata 0 (no FP); BOM + line-1 `[[include]]` FAIL. N2c `[["include"]]` and `[[\t'include'\t]]` FAIL.
  Controls before/after PASS.
- N3 FP: this container's env has CARGO_HOME, CARGO_HTTP_CAINFO, RUSTUP_HOME, RUST_BACKTRACE (+ RUSTUP_TOOLCHAIN,
  RUST_RECURSION_COUNT under `rustup run 1.99.1`); none matches UNSTABLE_ENV; ~/.cargo has no config. No CI workflow
  runs check.mjs (.github/workflows has no netcode-proof reference). Deps PASS in my step-2 run.
