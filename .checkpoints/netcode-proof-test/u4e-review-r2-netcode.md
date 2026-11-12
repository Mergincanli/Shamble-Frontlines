# U4e round 2, netcode-architect seat (read only; log prefix nc)

Started 2026-11-12. Packet: u4e-r2-packet.md. Fixed copy: .claude/worktrees/wf_0fa6595a-3e2-1 (uncommitted on 97bb9d3).
Base: .gate/u4e-r1. Judged against advisor-u4e-r1.md R1-R8 and T1.

## Plan
1. Check the copy's 23 files against .gate/u4e/fx-hashes-end.sha256 (read only: sha256sum -c).
2. Read my round-1 notes (u4e-review-netcode.md) for my findings R8 put in the fix pass (NC-H1, NC-M3; A2 and
   SEC-U4E-1 and the Resync limit are other seats' ids but my focus).
3. Client case 3 (FX-1): read client/src/lib.rs whole around old_header / bad_header / see / slot_for / receive's
   refused-datagram path; check the three cases against R1 and T1: case 3 never `see`, never `slot_for`, never acked,
   newest unmoved, bounded memory, counted refused; the sentinel's `once` rule vs a case-3 tick that later becomes a
   case-1 tick or a case-2 tick; `first` set on the first header taken (group, full state, refused datagram); wrap
   arithmetic (after32 direction).
4. The tick16 twin (FX-1 tests): what the judge counts (lab.compare, orphans, post-wrap none); clean twin n>=30,
   bad 0, no key below 65,520.
5. NC-M3 (FX-2): pong at r.arrival_ns; the test's shape; can it fail under its plant.
6. A2 (FX-4): RttSource; any caller mixing sources; tests.
7. FX-7 Resync limit: room.rs Resync arm; window arithmetic (next tick vs last_full_state; wrap; the forced resync not
   limited; the join's full state counts); interaction with resyncs_dropped; a later judged run INVALID? (-19, U4r).
8. SEC-U4E-1 (FX-13): pump AwaitJoin path; wrong channel / wrong direction first on the stream; order with the
   stream/size check; Joining-state counter (SEC-U4E-3).
9. Anything else in the changed code: A1's take_control bytes (stream_of in the lab; a judged run sending on the wrong
   stream?), A4 scripted fields, carry 50's age cast and `continue` path (event number's `sent` accounting), SEC-U4E-7.
10. Optional runs through the runner (prefix nc) only if a reading needs one.
11. Verdict.

## Notes (one line per finding as it lands)
