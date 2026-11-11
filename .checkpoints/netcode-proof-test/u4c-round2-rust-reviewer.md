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
- Step 1: hashes = the fix notes (pre-fix 1647e75d/c21a362f/639ab9c5; fixed d6e6e130/e8014c67/9c23f9cc).
- Step 2: diff read. lib.rs: only the handshake doc line (:54-57). tests/l1.rs: burst comment + "no stream resend in
  [2.2 s, 3 s)" assertion; new acknowledgements_are_lost_only_inside_bursts. tests.rs: Rig.at, drive(), woken(), Lc5
  struct, L-c5 event mode on drive, host_at_next_event, packet_threshold, time_threshold, lc6 links 2/3 fixed(0) with
  link 3 first, fails(r, lc, check). Byte offsets used by the tests/l1.rs checks = for_each_canonical (lib.rs:351-397:
  b[5] code, b[6] kind, b[16..24] sent, b[36..40] resends; ACK_CODE 3; tag 19 B filtered by len).
- Candidate (new code): drive() on a stuck next_event_ns steps t by 1 ns to the horizon (5 s / 10 s = 5e9/1e10
  iterations): a plant that names a past instant forever hangs instead of failing by name. Same in old L-c5 (t += 1),
  new for host_at_next_event. Low.
- (resumed 2026-11-11, new rust-reviewer run; the lines above are the cut run's.) Hashes re-checked: = above. Whole diff
  (771 lines) re-read. Fixer's section read (u4c-build.md:163-end of the worktree copy): r2-* plant logs on the final
  bytes, check.mjs r2-check-1/-2 10/10 (289/316) with foreign lanes in the shared target at their edges (a third run
  pending). Step 3 open items: Recv.arrival_ns meaning vs woken()'s `a == due` (stream HOL, reorder holds, P0 gate+1);
  canonical byte offsets (b[16..24], b[36..40] resends meaning, b[5]/b[6]); Profile jitter semantics vs
  time_threshold's doc; burst membership by send time; woken() timers/declared not horizon-filtered; drive()'s 1 ns
  stepping on stuck.
