# U4s fix pass: rust-reviewer (fix-review)

Worktree: /home/user/Shamble-Frontlines/.claude/worktrees/wf_aa2cc979-004-1 (uncommitted; base c8abc85)
Inputs: the three reviews (u4s-review-netcode.md, -arch.md, -rust.md, this folder); the fix pass notes (worktree's
.checkpoints/netcode-proof-test/u4s-build.md "## Fix pass", items 1-11); carries u4s-carries.md (this folder).
Rules (§7.1): CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target; CARGO_NET_OFFLINE=true; touch crates
before each build; df -h /home/user before each build (< 2 GB stop); git only inside the copy (no -C/-c); Write only
for files (no Edit tool in this session: this notes file is rewritten whole); bites planted in place one at a time by
my plant script (u4s-rust-plant.py, this folder: in place, reverse replacement, sha256 checked), restored, sha256
checked; commit nothing.

## Plan
1. Fingerprint (sorted sha256 of every file under spikes/netcode-proof + .checkpoints in the copy) and git status;
   per-file sha256 of files I will plant in.
2. Read the fix pass's changes for items 1-11; judge each against its finding.
3. Per item, one bite (in place, run, restore, sha256), preferring bites the fix agent did not run; doc-only items
   are judged by reading (and rustdoc where links matter).
4. Carries: every finding of the three reviews (netcode F1-F12 + side nit, arch F1-F8 + N1-N3, rust 1-7) is fixed or
   listed with its brief.
5. Final: touch, df, check.mjs once; 9/9; counts 275 / 292 / self 47; fingerprint equal to step 1.
6. Verdict, gaps with exact fix, regressions, check line. DONE.

## Progress
(started 2026-11-09)
- Step 1: base c8abc857 (git log -1); status 12 M + 9 ?? (sim src x4, sim tests/, client tests/, 3 notes files).
  Fingerprint (393 files under spikes/netcode-proof + .checkpoints, sorted sha256sum | sha256sum):
  6235158ed0f7201b06e69fc782412cf31fce011ceb4e901458e24c8b98c024c9. No target/ or .gate/ in the copy. Per file
  (first 16 hex): grid ede88bbedfcc0413, history 1c62e5aaf0c3a7d3, lib 073aba2702a23c67, seam a203150c12361d0b, ticks
  5d852f1a81f72e95, tests/seam 384746ab0031de04, client lib c4c4797df588d9ab, client tests/bites e4be8ae00e1faf52,
  proto Cargo.toml 21715f7b1e325ff4, client Cargo.toml 8fb1f79d1e4effda, Cargo.lock 3da1e15fd2ea6293, check.mjs
  508cd49ffd7f1571, check.test.mjs 0cfec1e5f9214139, room lib fd4db3af919701ef. = the fix agent's final hashes.
- Step 2 read: seam.rs, ticks.rs, history.rs, lib.rs, grid.rs, client lib + tests/bites.rs, proto/client manifests,
  Cargo.lock diff, tests/seam.rs, check.mjs :62-124 and :527-630, check.test.mjs :158-169, :1270-1387. Line refs in the
  fix report spot-checked: all land (seam :15-21/:26/:38-42/:58-67/:82-87/:105-153/:166-175/:184-194/:257-260, history
  :19-33, grid :17-19/:21-26/:53-70/:147-150/:160-201, lib :28-44, client :8-22, check.mjs :72/:117-124/:568-570).
- Observation A: check.mjs has no rustdoc step (grep "rustdoc|cargo doc" in check.mjs: only a comment at :928), so
  item 3's doc-link proof (fix agent's N10) is a manual run, not gated.
- Observation B: no mechanical check that a bite id is in one crate's list alone (check.mjs names no PM-0001 id but
  marks at :1182-1184); each site's test pins its own list only. To probe with a bite (re-add an id in grid.rs).
