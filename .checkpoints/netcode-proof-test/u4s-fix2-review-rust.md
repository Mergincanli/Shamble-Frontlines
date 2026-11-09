# U4s last small round: rust-reviewer (fix2-review)

Worktree: /home/user/Shamble-Frontlines/.claude/worktrees/wf_aa2cc979-004-1 (uncommitted; base c8abc85)
Inputs: worktree's .checkpoints/netcode-proof-test/u4s-build.md "## Last small round"; my earlier check
u4s-fix-review-rust.md (this folder): G1 (bite id in two crates), G2 (no rustdoc step), npt-sim membership nit.
Rules (§7.1): CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target; CARGO_NET_OFFLINE=true; touch crates
before each build; df -h /home/user before each build (< 2 GB stop); git only inside the copy (no -C/-c); Write/Edit
only for files (no redirects/heredocs/python -c); plants in place one at a time with Edit, reverse Edit, sha256 checked;
counts equal run to run; commit nothing.

## Plan
1. Fingerprint (sorted sha256 of every file under spikes/netcode-proof + .checkpoints in the copy, plus l1 lib.rs) and
   git status; per-file sha256 of the files I will plant in (check.mjs, check.test.mjs, grid.rs, sim lib.rs, seam.rs,
   l1 lib.rs, a lab src file).
2. Read the three changes: check.mjs BITE_ID/BITE_ID_LITERAL/testModules/biteHomeProblems, depsStep, DEPS_CHECKED;
   docBothBuilds/STEPS.doc/run(env); dependencyProblems npt-sim nit; check.test.mjs tests; l1 lib.rs:241 doc fix.
3. Per change, plants (in place, one at a time, restore + sha256), preferring ones the builder did not run:
   G1: an id in two crates (different from P1: e.g. sim + l1, or a tests/ vs src case), regex edge cases (an id in a
       non-bite context such as a comment; a `#[cfg(test)]` on a non-mod item; a nested mod; a mod named other than
       tests; an id in an attribute), revert check (biteHomeProblems returns []).
   G2: a rustdoc warning other than broken link (e.g. private_intra_doc_links, bare URL, invalid html) on measured and
       bites builds; revert check (doc step removed from STEPS / RUSTDOCFLAGS dropped).
   Nit: revert check (B4c done by the builder; I repeat one variant).
4. False positives on the committed tree: an id legitimately in one crate's src and its tests (src + tests/ same
   crate); doc warnings on measured or --all-features build (whole-workspace rustdoc both builds, -D warnings).
5. Final: touch, df, check.mjs once; 10/10; counts 275 / 292 / self 48; fingerprint equal to step 1.
6. Verdict, gaps with exact fix, regressions, check line. DONE.

## Progress
(started 2026-11-09)
