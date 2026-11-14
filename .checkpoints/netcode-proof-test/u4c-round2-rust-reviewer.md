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
- Step 3 done (per claimed fix, on the final bytes; plant logs spot-checked in the worktree's u4c-fix/r2-*.log, the
  failure lines = the fix notes word for word, "Compiling npt-l1 (this copy)" in each):
  - M1 / my round-1 M (L-c5 blind to next_event_ns): CLOSED. drive() (tests.rs:854) calls only at next_event_ns and
    the script's real sends; woken() (:889) asserts each hand-out at its due instant (Ready.arrival_ns = apply instant,
    lib.rs:1356-1388; gate+1 per lib.rs:1005, :1672) and a call at every log arrival, PTO and declaration. Event kinds
    all covered: HeldDue fires at the held datagram's logged arrival or is stale (lib.rs:1134-1168, :962-973). Plants
    none/no-timers/no-acks fail L-c5 + host test; no-ready/skips/no-held fail the host test (r2-m1-*.log).
  - M2 packet threshold: CLOSED (1.101/1.102/1.103 s acks, by count at 1.103 s exactly; 1M -> by time 1112.5, 2 ->
    1102.0; r2-m2-*.log). L1 latest-RTT term: CLOSED (oracle computes 9/8 x max itself; uses Rtt::sample for the
    smoothed value only; srtt-only fails trial 1). L2 acks only in bursts: CLOSED (in_burst on send time, lib.rs:1426,
    = the test's [1 s, 2 s); counts (372, 222, 17383) vs floors 50/50/3000; plant 361 wrong). L3: CLOSED (fixed(0)
    links, link 3 first, one call applies both 70 ms arrivals; link-first rank fails). L4: CLOSED (comment reworded;
    offsets b[16..24] sent, b[36..40] resends = "its piece's earlier sends" per lib.rs:341-349, so the filter selects
    resends only; bound 2.0 + 9/8 x 120 ms = 2.135 < 2.2 s). N1: CLOSED (lc + ": " prefix and check words). F1 doc
    line: accurate (RFC 9002 §6.2.1 "MUST NOT set its PTO timer for the Application Data packet number space until the
    handshake is confirmed", fetched 2026-11-11); its ruling is netcode's / the main session's.
  - Not claimed, stay follow-ups (not reopened): my Lows lib.rs:12-13 independence (netcode F4), close() no time,
    next_event_ns net-wide; nits; memory Low.
- Step 4 (defects of the fix): lib.rs change is 4 doc lines only (:54-57): no model code, so no effect on determinism,
  Wasm size or speed (by construction; not measured). New test code: no unsafe, no unchecked arithmetic that can
  wrap (x - s with ack arrival >= s; debug overflow checks on in tests), no panics on the canonical bytes (len checked
  before slicing; tag 19 B filtered).
  - NEW Low A: time_threshold doc (tests.rs:1242-1243) says "±20 ms of jitter (50 ± 10 ms each way)", the profile is
    jitter_ns 40 ms (:1256) = ±40 ms round trip, 50 ± 20 ms each way (Profile doc lib.rs:150, :156; Model::delay :789).
    Doc only; no assertion depends on it.
  - NEW Low B: drive() (tests.rs:874-877) steps 1 ns per call while next_event_ns names a past instant, up to the
    horizon (5e9 / 1e10 steps): a model that keeps naming a past instant hangs the test instead of failing on its
    stuck check. Same pattern as the pre-fix L-c5 loop (t += 1), new in the host test. Implausible from a real defect
    (advance pops every event <= now, lib.rs:947-958). Follow-up.
- Evidence note (test-engineer's to judge): r2-check-1/-2 10/10 (289/316) with foreign lanes in the shared target at
  their edges; r2-check-3 log empty (15 B) at the time read.
- Verdict: APPROVE WITH FINDINGS (all claimed fixes closed; 2 new Lows, follow-ups). DONE
