# U4c review round 2, rust-reviewer (read only, on what changed)

Pre-fix: /home/user/Shamble-Frontlines/.gate/u4c-prefix (lib.rs 1647e75d expected). Fixed:
/home/user/Shamble-Frontlines/.claude/worktrees/wf_f54246ff-e0d-2 (lib.rs d6e6e130, tests.rs e8014c67, tests/l1.rs
9c23f9cc per the fix notes). Round 1: u4c-review-rust.md (M L-c5/next_event_ns; Lows: lib.rs:12-13 overstated
independence, close() no time, next_event_ns net-wide; nits BITES doc, stale()/rank() invariants, space 1 inert;
memory Low), u4c-review-netcode.md (F1-F7), u4c-review-test.md (M1, M2, L1-L4, N1, O1, O2).

## Plan
1. sha256 of the three files in both copies vs the fix notes.
2. diff -ru of crates/l1; read every hunk.
3. Per claimed fix: M1 (drive/woken, new host test), M2 (packet threshold), L1 (time threshold), L2 (acks only in
   bursts), L3 (lc6 link order), L4 (burst comment + PTO assertion), N1 (fails()), F1 (handshake doc line).
4. Defects added by the fix: test logic soundness, panics/overflow in test helpers, determinism, clippy-level issues,
   docs, vacuity.
5. Verdict.

## Progress
