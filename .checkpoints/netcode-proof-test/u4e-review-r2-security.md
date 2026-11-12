# U4e round 2: security-auditor review (on what changed: the fix pass FX-1 to FX-13)

Seat: security-auditor (workflow agent, opus). Read only: no edit to any copy; notes here only, written with the Write
tool (rewritten whole each time, content kept). Fixed copy: `.claude/worktrees/wf_0fa6595a-3e2-1` (uncommitted on
97bb9d3). Round 1's tree: `.gate/u4e-r1`. Log prefix: `sec`. Cites are into the fixed copy's
`spikes/netcode-proof/crates/` unless a path says otherwise. Judged against advisor-u4e-r1.md R1-R8 and T1 (the packet,
u4e-r2-packet.md).

## Plan
1. Check the copy equals fx-hashes-end.sha256 (sha256sum -c, read only) and that the diff stays inside R8's bounds
   (files only under crates/room, crates/server (not schedule.rs), crates/client; no manifest, Cargo.lock, npt-proto,
   npt-sim, npt-link or npt-lab file; one new public item `RttSource` and `take_control`'s changed type).
2. The pump (server/src/pump.rs): NC-M3 (`pong` from `r.arrival_ns`: can a client make the arrival lie?), A2
   (`RttSource`: can a ws client pick `Link`, or a wt client make its RTT what it wants), SEC-U4E-1 (pre-Join forged
   kind closes; does it close any legitimate path, does it reach Joining/Joined), SEC-U4E-3 (`joining_dropped`: is the
   catch-all now hiding any other state), TE-5's size checks (do the tests prove the cap the pump enforces).
3. The client's decode path (client/src/lib.rs): R1's case 3 (old_header, bad_header) against T1's sentence; bounded
   memory under hostile server bytes (bad_headers deque, out.hashes growth per input call, lost runs); SEC-U4E-7's
   checked add; any new panic (wrapping, indexing, the const assert).
4. The Resync limit (FX-7, room.rs): who can trigger a full state, how often; the forced (overflow) path; the join's;
   what one costs the room (encode, bytes, ack ring); can the limit be bypassed by Leave/re-Join or by a slot replace;
   does `last_full_state` reset where it must; wrap of `next.wrapping_sub(f)`.
5. A1's bytes (room.rs `send_control`, `full_state`, `take_control`; server/tests/common/mod.rs `stream_of`): what is
   dropped when an encode fails (silent?), the channel the caller reads from the type byte (a confusion that sends a
   FullState on Control or the reverse), the library contract for U6's caller.
6. Anything else in the changed code: new panics, unbounded growth (scripted_* vectors), cross-client leaks; the
   riders RR-2 (event age), RR-3, RR-10; anything beyond R8's scope.
7. Anti-cheat coverage table updated for what changed; verdict and findings in the schema.

## Blocks (written as each finishes)
