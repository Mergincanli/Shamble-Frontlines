# U4d round 2 (combined with W1-S1's npt-proto half): test-engineer

Copy: /home/user/Shamble-Frontlines/.gate/u4d-s1 (detached at 21a8597; nothing committed). Round 1 (builder's state):
.gate/u4d-prefix. Fix record: u4d-build.md "Fix pass, round 1"; proto half: s1-proto.md. Rulings: advisor-u4-wave1.md
W1-S1 :45-111, W1-U4D :113-304, W1-PUB :353-436. Round 1 mine: u4d-review-test.md (M1-M5, L1-L6, N1-N3).
Other round 2 notes (read-only): u4d-round2-{netcode,security,rust,arch}.md. Memory: none (no agent-memory folder).
Rules: CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target, CARGO_NET_OFFLINE=true; `cd <copy> || exit 1`
alone on its own line in a subshell; never background a chain with cd; idle wait (pgrep cargo/rustc) + touch all crates
files + 2 s toucher inside the same command for counted runs; provenance pre-run (Compiling npt-sim / npt-proto with
this copy's path); df > 2 GB; plants one at a time, restore, sha256 check; commit nothing.
Expected counts: measured 351, bites 388, self 52 of 52.
DEVIATION (stated): this run has no Edit tool (the harness refused it: "No such tool available: Edit"), as in round 1.
So: notes are rewritten whole with Write; plants are applied by a scratch script (scratchpad r2t-plant.py) that
replaces exactly one unique match (asserts count == 1), saves a byte copy first, restores from it and checks sha256
against r2t-ref.sha256 after each plant. Same method as round 1's u4d-test-bites.py. Runner: scratchpad r2t-run.sh
(idle 8 s, df, touch, 2 s toucher inside the script, output to stdout only).

## Plan
1. sha256 of every file in crates/sim and crates/proto; compare with u4d-build.md's list and s1-ref.sha256.
2. Read the diff (prefix vs copy for sim; 21a8597 vs copy for proto) and the other round 2 notes.
3. Round-1 findings M1-M5, L1-L6, N1-N3: CLOSED / NOT CLOSED with proof (test + plant).
4. Re-plant (one at a time, named test fails, restore, sha256): Q2, Q6, Q11a, Q13, Q19; P1, P3, P7; plus my own
   plants where a test's power is in doubt (O1 hull-part rotation sign, O2 rotation ignored, O3 phase 3 stale soldier
   records, O4 decode cap undone, O5 carry 13's derived clone_from); probes X1 (H1 count over 54,000 SS ticks), X2 (ES
   bands over 18,000 ticks = 10 min).
5. Carries 9, 12, 13, 14, 36 against code and tests.
6. The fixer's open points A (ES knobs), B (ES drift), C (SOLDIER 8 vs 9; new public items).
7. Settle by test/plant any other reviewer's finding a run can settle.
8. check.mjs once at the end (fresh touch, idle, 2 s toucher): every step's counts.
9. Final sha256 = step 1's.

## Progress
- [1] sha256 of all 34 files under crates/sim (17) and crates/proto (17): scratchpad r2t-ref.sha256. crates/sim: 19 of
  20 f-ref.sha256 entries OK (u4d-build.md :305-315's list plus the unchanged files and workspace files); crates/sim
  also `diff -r` identical to U4d's worktree wf_f54246ff-e0d-1. The one mismatch is check.mjs (f-ref 6d1ed188, here
  7eddc285), which equals 21a8597's (U4a's merged check.mjs; `git diff 21a8597` clean): expected. crates/proto: all 7
  s1-ref.sha256 entries OK (snapshot 48ca1183, error f52dbb94, common b23b687b, wire 69ef15ec, bites 96d07cc9, sim
  seam 0c41b406, quant 0d41e906). Workspace Cargo.toml 75b6c160 and Cargo.lock 3da1e15f = 21a8597. git status: 10 M,
  9 ??. df 11 GB free.
- [2] Read: sim diff prefix -> copy (scratchpad r2t-sim.diff, 3,103 lines: history, lib, replay, scene, seam, world,
  tests replay/seam/world), proto src + tests diff vs 21a8597 (r2t-proto-src.diff, r2t-proto-tests.diff), the four
  other round-2 notes (netcode, security, rust, arch: all APPROVE WITH FINDINGS, no Critical/High). Doubts to settle
  by plant: (a) hull o part composition has no semantic test (h1 checks "box moved", a_m1 compares the function with
  itself): O1, O2; (b) phase 3's present volumes now come from refreshed soldier records: which test sees a stale
  refresh? O3; (c) rust L1 = my L5 (decode cap) has no test: O4; (d) h1's "changes only when health does" is measured
  over 600 ticks only, and its comparison includes the record's area (not on the wire: proto between() reads grid
  only, snapshot.rs:86-115): X1 over an S30; (e) fixer's point B: X2 over ES's 10-minute run length.
