# rust-reviewer, review round 2, PR #48 (foundation 2.2, the toolchain)

Head under review: 4bccf55be7bbf54de6e114ddbc6aeadb2cab3f3e (branch claude/laughing-pasteur-fwya24); round 1's head 4d78ad6.
Scope: only what changed outside .checkpoints/ since 4d78ad6 (1de98a9, 4bd9dd5, 297d48a + 8d622ac, 8a4f9b0, 4bccf55).
Worktree: /home/user/Shamble-Frontlines/.gate/foundation-toolchain-wt (read only, except this file).
Scratch copies: /home/user/Shamble-Frontlines/.gate/rust-reviewer-r2/ (git archive of 4bccf55).
Agent memory: none exists (no .claude/agent-memory*/rust-reviewer); round 1's notes are the baseline
(rust-reviewer-review-r1.md, C1-C15, RR-1..RR-3).

## Plan
1. Read the code diff (Cargo.toml:21-29, the four lib.rs doc comments), the ruling D1 (conditions 1-7), the deduped
   round-1 dispositions and the report's overflow lines (Outcome, As built, E1, M5, M7, OVERFLOW-* rows, OVERFLOW-ALL).
2. Focus 1 (reach): fresh git-archive copy of 4bccf55; re-run xcrate-check.sh; diff with xcrate-after-4bd9dd5.txt and
   xcrate-before-1de98a9.txt. Add: -v flags per crate (release, native and wasm32), every member + a non-member
   path dep; wasm32 by behaviour (Node) for sim inline/generic code compiled in a workspace cdylib.
3. Focus 2 ("*"): Cargo book 1.99.1 profiles.html (Overrides; precedence list), cargo source at the 1.99.1 tag
   (profiles.rs: ProfilePackageSpec::All match, the precedence of named over "*"), and behaviour: does "*" skip
   members; does it match a path dep outside the workspace; a path dep INSIDE the workspace folder but not a member
   (exclude); a git/registry-like dep (vendored or local registry if possible); does a named override beat "*";
   what if a member is ALSO named in a package override; warnings for unmatched names.
4. Focus 3 (profiles): bench (inherits release), a custom `inherits = "release"`, `--profile test`, dev, test;
   doc tests; build scripts / proc macros (build-override: does `[profile.release]` overflow-checks reach
   build-override? does "*" reach build deps?); `cargo test --release` uses which profile for the test harness.
5. Focus 4: the Cargo.toml comment at :21-24, sentence by sentence, against 2-4.
6. Focus 5: the four doc comments from 1de98a9 against ADR 0002 parts 3 and 5, the migration steps 2.3-2.5,
   OQ-0002-3 and OQ-0002-7.
7. Focus 6: M7 (297d48a, 8d622ac): the diffs (exact revert?), CI runs (red at the K1b line, inside the cut; green
   after); what the report claims for M7 and whether that claim is limited to what M7 can show.
8. Forward look on route A only: anything that changes behaviour beyond overflow (debug-assertions? no), cost in
   release for room/server code (panic vs wrap: security-auditor's), Wasm size impact (toy numbers only), the
   interaction with a future `panic = "abort"` / wasm32 trap.
9. Record a line per check and per finding below; final answer: verdict, findings RR2-n, checked-right list,
   not verified.

## Checks (one line each, as done)

## Findings
