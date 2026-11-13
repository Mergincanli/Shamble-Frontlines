# Skeptic, review round 1, D8 (= IE-4, Low): G0's 2.3 finding should record the image's stable = the pin and the active-toolchain check

Plan: re-read IE-4 (review-r1-findings.json), toolchain.yml and the report's G0 row at 4d78ad6, plus G0's full text in
STATUS_ARCHIVE, the register row Q2.2-a and ADR 0002's G0; re-fetch the head job's and M2's CI logs; re-read the
runner image's software list and install-rust.sh, and rustup's CHANGELOG, book and release-stable.toml; check the plant
list and toolchain.yml's history for any run without the install step; reproduce rustup's override order locally only if
rustup is on this machine (in .gate/skeptic-D8/, never in the worktree); then judge holds, severity and the fix.
Default: does not hold unless the evidence is there.

## Checks
- C0 head: the worktree sits at saver commit 5ad4e55; `git diff --stat 4d78ad6 HEAD` touches `.checkpoints/` only, so the
  workflow, the report, STATUS_ARCHIVE and the register read here are the reviewed head's (read with `git show 4d78ad6:`).
- C1 where G0 is recorded (lasting docs at 4d78ad6): report :166 "the toolchain-file check (exit 2) moves to 2.3's G0
  under `--prove`"; STATUS_ARCHIVE :586-589 (G0's full text) the same plus "M2: red at the pin check with exit 2"; the
  register row Q2.2-a (adr/README.md :95) the exit-2 mark only. Grep of the report, STATUS, STATUS_ARCHIVE, DECISIONS,
  ROADMAP, REPO_MAP and the register for RUSTUP_AUTO_INSTALL / auto-install / RUSTUP_TOOLCHAIN / the image's stable /
  self-update: no hit. The only lasting mention of the source check is the report's Outcome :25-27 ("the active
  toolchain comes from the file"), as built, with no reason and no carry to 2.3. So "G0's finding carries only the
  exit-2 mark" HOLDS as stated.
- C2 the facts are known, but only in the plan: plan.md :279-281 (RUSTUP_AUTO_INSTALL's reason, N4), :378-382 (the image's
  default Rust equals the pin, "so the version check alone cannot show the toolchain came from the file": the
  active-toolchain check, the advisor's F-5), Appendix B N1 :740 and N4 :743. The plan is deleted before ready
  (CLAUDE.md Checkpoints 6; the report's Appendix A note), readable only at a checkpoint commit. ADR 0002's G0 row
  (:569) says "toolchain equals `rust-toolchain.toml`" and PM-0002-39 (:1305) "never uses another Rust", but neither
  names the RUSTUP_TOOLCHAIN / override case or auto-install, and PM-0002-39's mutation list has no such mutation. So
  "not recorded for 2.3" HOLDS for the lasting record; the code itself (toolchain.yml :21 comment, :48 and :50 messages)
  states what it checks but not why the version check is not enough.
- C3 head job 131510223737 (re-fetched, .gate/skeptic-D8/logs/): "Image: ubuntu-24.04", "Version: 20261111.370.1";
  the install step: "syncing channel updates for 1.99.1-x86_64-unknown-linux-gnu", "downloading 6 components", "the
  active toolchain `1.99.1-x86_64-unknown-linux-gnu` has been installed", "it's active because: overridden by
  '.../rust-toolchain.toml'"; the check: `rustup show active-toolchain` = "1.99.1-x86_64-unknown-linux-gnu (overridden
  by '.../rust-toolchain.toml')", "rustc 1.99.1 (58cf8f9c3 2026-10-13)". No "checking for self-update" line anywhere in
  the install step. HOLDS as cited.
- C4 M2 job 130098974545 (re-fetched): image 20261101.360.1; "info: using existing install for
  stable-x86_64-unknown-linux-gnu", "the active toolchain `stable-x86_64-unknown-linux-gnu` has been installed", "it's
  active because: it's the default toolchain"; then "sed: can't read rust-toolchain.toml", "Process completed with exit
  code 2" (set -e on the command substitution). So M2 used the explicit install, which found the image's stable; no
  proxy ran before the failure, so RUSTUP_AUTO_INSTALL played no part. HOLDS.
- C5 no run without the install step: `git log --full-history origin/main..4d78ad6 -- .github/workflows/toolchain.yml`
  = fb17451 (K1), fd14855 (K1b) and the merge c901187 (the file is new against main); none of the seven plants (1bee16c,
  363c455, 0d3ec36, af65262, 2d9766f, 45f1bb5, b764298) touches the workflow (their --stat). In every green run the
  explicit install puts 1.99.1 in place before any proxy runs. So "RUSTUP_AUTO_INSTALL=0 was never seen working in a
  run" HOLDS: no evidence exists either way.
- C6 the image (runner-images, tag ubuntu24/20261111.370, fetched 2026-11-12 into .gate/skeptic-D8/dl/): the readme's
  "Rust Tools": "Cargo 1.99.1", "Rust 1.99.1", "Rustdoc 1.99.1", "Rustup 1.29.3"; install-rust.sh: `sh -s -- -y
  --default-toolchain=stable --profile=minimal`, then `rustup component add rustfmt clippy`; no auto-self-update setting
  and no wasm32 target. static.rust-lang.org/rustup/release-stable.toml: version '1.29.3'. So "the image's own stable
  equals the pin today" HOLDS (also on M2's older image 20261101.360.1, per plan.md :378). Side fact the finding does
  not use: the image's stable lacks `wasm32-unknown-unknown`, so today a wrong-but-same-version toolchain would still go
  red later, at the wasm32 build (exit 1, not G0's exit 2); not a silent pass of the whole job.
- C7 rustup CHANGELOG (master, fetched 2026-11-12): 1.28.0 (2025-03-04) "rustup will no longer automatically install the
  active toolchain if it is not installed. [pr#3985]"; 1.28.1 (2025-03-05) "Automatic install is enabled by default but
  can be opted out by setting `RUSTUP_AUTO_INSTALL` environment variable to `0`. [pr#4214] [pr#4227]"; 1.29.0
  (2026-03-05) "fix(cli/rustup-mode): add missing self-update in `rustup toolchain install` ... pull/4566". All three
  quotes HOLD. NOT cited by the finding: 1.28.0's "Disable automatic self updates in CI environments by @djc in
  .../pull/3907".
- C8 the self-update question the finding leaves "Unverified" is answerable from rustup's source at tag 1.29.3 (the
  image's and today's rustup): src/cli/self_update.rs :474-479 `SelfUpdateMode::from_cfg` returns `Disable` when
  `CI` is set and `RUSTUP_CI` is not ("If we're in CI ... disable automatic self updates"); :516-521 `update()` returns
  at once unless the mode is Enable. src/cli/rustup_mode.rs :752 `ToolchainSubcmd::Install { opts } => update(cfg, opts,
  true)`; :1050 the mode from `from_cfg`; :1122-1126 `ensure_active_toolchain`, the two "info:" lines the head log shows,
  then `self_update_mode.update(...)`. GitHub's variables reference (docs.github.com, fetched 2026-11-12): "CI  Always
  set to true". So on the runner the install step never self-updates, with or without `--no-self-update`; the image's
  rustup is already the one that runs. The fix's third bullet ("Consider `--no-self-update`, so the image's rustup is
  the one that runs") rests on a reason that does not hold in CI. The flag would change behaviour only where CI is unset,
  e.g. a local run of 2.3's G0 ("the same locally and in CI", ADR 0002 part 9), where it would keep G0 from
  self-updating a developer's rustup: a different reason, not the one given.
- C9 rustup's source at tag 1.29.3, src/config.rs: `should_auto_install` :435-441 (`RUSTUP_AUTO_INSTALL` set ->
  `mode != "0"`), the override order :592-621 (`+toolchain`, then `RUSTUP_TOOLCHAIN`, then the directory walk: override
  database or `rust-toolchain{.toml}`, then the default) and the reasons :103-113 ("overridden by environment variable
  RUSTUP_TOOLCHAIN", "directory override for '...'", "overridden by '<file>'", "it's the default toolchain"). So
  `case *rust-toolchain.toml*` (:50) rejects every source but the file, and :48 gives the env source its own message.
- C10 local reproduction (rustup 1.28.2 on this machine; toolchains `1.99.1` and `stable` = 1.97.0; the head's
  rust-toolchain.toml and toolchain.yml :46-52 copied to .gate/skeptic-D8/, run with `bash --noprofile --norc -eo
  pipefail`, RUSTUP_AUTO_INSTALL=0; three variants: full :46-52, without :48, version-only :46-47+:51-52):
  - RUSTUP_TOOLCHAIN unset: all three exit 0 ("1.99.1-x86_64-unknown-linux-gnu (overridden by '.../rust-toolchain.toml')").
  - RUSTUP_TOOLCHAIN=1.99.1 (same version, chosen by the env, not the file): full exit 1 at :48; without :48 exit 1 at
    :50 ("... (overridden by environment variable RUSTUP_TOOLCHAIN)"); version-only exit 0 "rustc equals the pin 1.99.1".
    => the core claim REPRODUCED: a version-only check passes on a toolchain the file did not choose when versions match.
  - RUSTUP_TOOLCHAIN=stable (here 1.97.0, not the pin): all three exit 1, the version-only one with "rustc is not the pin".
    => the fix's mutation 1 as written proves the source check only while the image's stable equals the pin; when stable
    moves on, a version-only G0 fails it too. Image-independent form: RUSTUP_TOOLCHAIN set to the pin's own channel.
  - The fix's mutation 2, with an empty RUSTUP_HOME in .gate/skeptic-D8/rh (so no install step has run) and a dead dist
    server (RUSTUP_DIST_SERVER=http://127.0.0.1:9; nothing downloaded), CI=true: with RUSTUP_AUTO_INSTALL=0, exit 1 at
    :49, "error: override toolchain '1.99.1-x86_64-unknown-linux-gnu' is not installed ... the toolchain file ...
    specifies an uninstalled toolchain", no download tried; with it unset, "info: syncing channel updates for
    '1.99.1-x86_64-unknown-linux-gnu'" then a download error (in CI, with the network, it would install and pass).
    => mutation 2 is SOUND and discriminates. (Local rustup 1.28.2; 1.29.3's source, C9, has the same rule.)
- C11 the fix's "No edit to toolchain.yml now (A10)": A10 item 2, "Any other change to the CI job asks me again" (the
  report's Appendix A10). Apt. The fix is docs-only and should stay out of the register row Q2.2-a (a decision doc).

- C12 (after the advisor tool's read) mutation 2 on CI's rustup, from the 1.29.3 source: rustup_mode.rs :1324-1352
  `show_active_toolchain` -> `maybe_ensure_active_toolchain(None)`, which with RUSTUP_AUTO_INSTALL=0 takes
  `active_toolchain()` (no install, config.rs :555-565), then `Toolchain::with_source` (toolchain.rs :75-108) returns
  "override toolchain '{name}' is not installed" with "the toolchain file at '...' specifies an uninstalled toolchain":
  the same text 1.28.2 printed in C10. So on 1.29.3 too the check step fails at `rustup show active-toolchain` (:49),
  from source, not from a run. M2's image re-read (runner-images tag ubuntu24/20261101.360, fetched 2026-11-12): "Rust
  1.99.1", "Rustup 1.29.2": stable equalled the pin there too.
- Labels (what is inferred, not seen on the runner): "the image's stable equals the pin" rests on the two images'
  software lists; no log prints the image stable's `rustc --version`. "The image's stable lacks wasm32" is inferred from
  install-rust.sh; no run shows it, so it is not part of the severity reasoning. The rustup book's "Basics" page was not
  re-read; the `--no-self-update` flag's doc comment in the 1.29.3 source was (rustup_mode.rs :453-455, "Don't perform
  self update when running the `rustup toolchain install` command").

## Verdict
- HOLDS, Low. Every factual claim re-checked holds (C1-C7, C10, C12); the core claim was reproduced (C10: a version-only
  check passes on a same-version toolchain the file did not choose; :48 and :50 each catch it). The gap is a
  carry-forward record only: both facts live today only in plan.md (N1 :740, N4 :743), deleted before ready; 2.2's job
  has the checks, and ADR 0002's G0 (:569) and PM-0002-39 point the right way but name neither case. Not Medium: no
  wrong result in this PR; the reach is 2.3's G0 design, and a Low rides another push (the review budget).
- The fix: bullets 1 and 2 right, with refinements: (a) record the reason and the checks' substance (RUSTUP_TOOLCHAIN
  unset, `rustup show active-toolchain` names rust-toolchain.toml, RUSTUP_AUTO_INSTALL=0, and why), not only line numbers
  of a file 2.3 replaces; in the report row AND STATUS_ARCHIVE's G0 full text (:586-589), never the register row Q2.2-a;
  (b) mutation 1 should set RUSTUP_TOOLCHAIN to the pin's own channel, which discriminates on any image (C10); `stable`
  stops discriminating once the image's stable moves past the pin, so keeping it is optional. Mutation 2 is sound (C10,
  C12). Bullet 3's stated reason is moot on the runner (C8: rustup disables self-update when CI is set, and GitHub always
  sets CI=true, so the image's rustup already runs): drop it or reword its reason; whether a local G0 run would want the
  flag is 2.3's question, with no evidence here either way. Breaks nothing: text only, no CI edit, no decision doc.
