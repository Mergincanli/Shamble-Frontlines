# U4s fix pass: rust-reviewer (fix-review)

Worktree: /home/user/Shamble-Frontlines/.claude/worktrees/wf_aa2cc979-004-1 (uncommitted; base c8abc85)
Inputs: the three reviews (u4s-review-netcode.md, -arch.md, -rust.md, this folder); the fix pass notes (worktree's
.checkpoints/netcode-proof-test/u4s-build.md "## Fix pass", items 1-11); carries u4s-carries.md (this folder).
Rules (§7.1): CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target; CARGO_NET_OFFLINE=true; touch crates
before each build; df -h /home/user before each build (< 2 GB stop); git only inside the copy (no -C/-c); Write only
for files (no Edit tool in this session: this notes file is rewritten whole); bites planted in place one at a time by
my plant script (u4s-fix-rust-plant.py, this folder: in place, reverse replacement, sha256 checked), restored, sha256
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
- (resumed after the spend-limit cut, 2026-11-09) Re-checked before building on it: fingerprint recomputed =
  6235158e... over 393 files; git status same 21 entries; `u4s-fix-rust-plant.py status`: all 6 targets == baseline
  (the script's BASE = the step-1 hashes). df 13G free. Step 2's reading re-done for the files the bites touch.
- Reading notes (no bite needed): Field::integer (grid.rs:59-70) computes span in i64 then u64, so i32::MIN..=MAX is 32
  bits with no overflow; assert min<max. HitHistory::for_ms (history.rs:22-33) ceil via ticks_for_ms; allocation is
  sized by a u32 ms (<= ~128.8M entries at 30 Hz); the 1 MB cap is carry 14 (U4d). check.mjs SIM_DEPENDENCIES check
  (:117-124) is silent if npt-sim is missing from the workspace (other members push "is not a workspace member"): nit.
  EntityKind::ALL completeness: s6 (tests/seam.rs:440-443) holds len 7 + strictly ascending, so today it is every
  variant once; an 8th variant with ALL left at 7 compiles and passes (layout()'s match is exhaustive, ALL is not): nit.
  check.mjs:609: outside npt-sim a non-derived *_TICKS name is accepted if its initializer merely names TICK_HZ
  (`7 * TICK_HZ / 30` style), and no s1 row pins it at 60 Hz: stated design (doc :558-561), nit.
  grep crates/ for HitHistory::new, Config::with_bite, Config::BITES: none (exit 1). Cargo.lock vs base: +14, 0 new
  [[package]] (npt-proto's 4 deps incl. the 2 dev; npt-sim's 3; room/server/client edges).
- Bites (each: plant, touch, df 13G, run, restore == baseline sha256; measured seam 8 tests ran, bites seam 10 +
  client 1):
  F4-floor-span (history.rs: span floored): s4 FAILED seam.rs:250 "left 63 right 64"; s4b FAILED :291 (1 ms -> None).
  F5a-second-home (grid.rs: bites-only BITES with -17's id appended) -> check.mjs fmt, clippy, clippy-wasm32, deps,
    marks all PASS; npt-sim bites 10/10, npt-client bites 1/1 pass. => GAP confirmed: "no id in two crates' lists"
    (carries 11, 24) is checked by nothing; the arch F1 regression (ids back in the grid) goes unseen.
  F5b-client-no-defect (client maps -17 to Config::default): client bites test FAILED bites.rs:12 (floor false/true).
  F6-all-duplicate (ALL ends Wall, Wall): s6 FAILED seam.rs:443 "Wall".
  F7-build-dep-unshipped (check.mjs sim check counts only normal deps): self-test not ok 9 "a build dependency ships";
    47 tests, 46 pass.
  F9a-fn-drift (ack_ring_ticks_at = tick_hz + 2: 32 at 30 Hz, 62 at 60): s1 FAILED seam.rs:90 "ACK_RING_TICKS at 60
    Hz" 62 vs 64; deps PASS (as designed: the grep reads constants).
  F9b-60hz-literal-rate (TICK_HZ 60 + FULL_STATE_ACK_TICKS = full_state_ack_ticks_at(30)): cargo check E0080
    "FULL_STATE_ACK_TICKS is full_state_ack_ticks_at(TICK_HZ)".
  F10a-client-rederives (client lib.rs: LOOP_CATCHUP = npt_sim::TICK_HZ / 4): real-tree deps FAIL "crates/client/src/
    lib.rs:24 sets LOOP_CATCHUP from `npt_sim::TICK_HZ / 4` outside crates/sim/src/".
  F10b-catchup-names-dropped (TICK_NAMED = /_TICKS$/): self-test not ok 46; 47 tests, 46 pass.
- Process note: one empty `python3 -I - <<'X'` heredoc ran by mistake while locating a line in my plant script; python
  read an empty script and wrote nothing (the next status check: all targets == baseline). The plant script was then
  rewritten whole with Write to add F3-doc-link.
- Item 3: RUSTDOCFLAGS=-D warnings cargo doc --no-deps -p npt-sim -p npt-client, measured and --all-features: both exit 0.
  F3-doc-link (seam.rs rewind_outside -> rewind_outsde): rustdoc "error: unresolved link to InputReports::rewind_outsde";
  check.mjs fmt, clippy, deps all PASS (no doc step: Observation A confirmed). Restored a203150c == baseline.
- Rust 6 (carry 16): clippy -W missing_docs, npt-sim + npt-client lib, bites: 14 hits, all seam.rs (:93, :94, :99, :160,
  :221 x2, :225-228, :231 x2, :238, :239) = carry 16's 14 names; the fix pass added no undocumented pub item.
- Item 8: cargo tree -d -e normal,build f183b34a07038121 (= the pass's baseline); --all-features --target all
  edb1202d1ed375db (= the fix agent's); duplicate set getrandom 0.2/0.4, syn 2/3, synstructure 0.13/0.14,
  windows-link, windows-sys = my first review's set; the one new line is rand_chacha [dev-dependencies] npt-proto
  under zerocopy-derive -> ppv-lite86 (consistent with T1). npt-proto normal,build depth 1: npt-link, npt-sim only;
  dev: rand_chacha 0.10.0, rand_core 0.10.1.
- --list (touch, df 13G): measured seam 8 (s1, s2 x2, s3, s4, s4b, s5, s6), client 0; bites seam 10 (+ the_grids_one_
  defect..., pm_0001_41...), client tests/bites.rs 1 (pm_0001_17...). Pre-fix 6 / 7 -> +2 / +4: = 273->275, 288->292.
- Next: final check.mjs; fingerprint; verdict.
