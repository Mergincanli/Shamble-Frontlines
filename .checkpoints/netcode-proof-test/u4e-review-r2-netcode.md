# U4e round 2, netcode-architect seat (read only; log prefix nc)

Started 2026-11-12. Packet: u4e-r2-packet.md. Fixed copy: .claude/worktrees/wf_0fa6595a-3e2-1 (uncommitted on 97bb9d3).
Base: .gate/u4e-r1. Judged against advisor-u4e-r1.md R1-R8 and T1. No Edit tool in this seat: the notes are rewritten
whole with Write each time (T4 item 1).

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
- Step 1: `sha256sum -c fx-hashes-end.sha256` in the copy: 23 of 23 OK. `diff -rq` of .gate/u4e-r1's spike tree with the
  copy's (target excluded): exactly 11 files differ, all under crates/room, crates/server (not schedule.rs) and
  crates/client, each in the packet's diffs. No manifest, lock, proto, sim, link or lab file. Within R8's bounds.
- Step 2: my round-1 findings R8 put in the fix pass: NC-H1 (FX-1) and NC-M3 (FX-2). NC-M2, NC-L1, L3, L4 are report
  words (T5), not this round's code.
- FX-1, case 3 (client/src/lib.rs:499-530, called at :597, :684, :777): `old_header` returns at once for a tick not
  too old (case 1, unchanged); a too-old tick before `first` (`Wrap::default().after32("tick", t, f)`: f after t) goes
  to `bad_header` (no `see`, no `slot_for`, no `acknowledge`, `newest` untouched, `(t, 0)` pushed once per tick while 32
  are remembered), counted in `refused` for a group or full state (count = true) and not again for the refused
  datagram (receive :474 counted it); from `first` on it is `stale` (case 2, as built). `first` is set in `see` with
  the first `newest` (:551-555), i.e. the first header taken of any of the three kinds. Matches R1's form and T1's
  sentence. The bite writes only the Snapshot header's tick (proto quant.rs:133-139, snapshot.rs:532): FullState's tick
  stays 32-bit, so a forced resync under the bite still lands as case 1 and is acked: no `crash`, no kick from the
  client side.
- FX-1, honest runs: no group or full state can name a tick before `first` (the room sends no snapshot while the
  join's full state is unacknowledged: room.rs:1083-1089; FullState is the first header; L1 duplicates nothing), so
  case 3 is unreachable for a correct build: stricter, never looser. Lab compare (common/mod.rs:370-380) counts a
  sentinel or a client key with no or another server hash as bad, as SERIES.md:266 does ("A client hash with no server
  hash counts on every machine"); under the bite the coverage gap (ticks 14..65,519 unresolved) is outranked by "a
  mismatch still fails the run" (SERIES.md:266): FAIL, not INVALID, at U4r (carry 73's contract test).
- FX-1, the tests (loopback.rs:264-348; :293-332): clean: 58 compared, 0 bad, no key below 65,520, 65,537's pair
  equal; bitten (both ids): 58 compared, 28 bad, slot 0's orphans [0..13] (14, Q-1's prediction), none at or past
  65,536, stale 0 (fx-t5print.log:18-22). Plant (a) fails the twin at loopback.rs:328 (fx-p1a.log:46-74), plant (b)
  fails the clean test at :282 in s03 and s04 (fx-p1b-s0{3,4}.log). NC-H1 CLOSED.
- FX-2 NC-M3 (pump.rs:430, :455-468): `pong(&p, r.arrival_ns)`; the test pumps 20 ms after arrival and pins 100 ms
  (server/tests/pump.rs:328-333); its plant fails it by name (fx-p2.log:18-51). NC-M3 CLOSED.
- FX-4 A2 (pump.rs:180-203, :295-303): `RttSource` chosen at `Decoder::new`, `rtt_ns` matches on it, no `or_else`;
  every caller passes one (the lab and pump.rs's `decoder` pass `Link`, e10 and two tests `Pings`); nothing else in the
  workspace reads `rtt_ns` from a Decoder. Its two tests fail under the plant (fx-p4.log:31-91). No session can mix.
  Matches §4.9 :352-354 (read at 1e8c188).
- FX-13 SEC-U4E-1 (pump.rs:367-389): in AwaitJoin a stream message refused as WrongChannel or WrongDirection counts
  `first_not_join` and closes with Kick(malformed) and returns (the Join behind it never read); joined sessions keep
  §4.10's count-and-go-on; other stream refusals close as before. SEC-U4E-3 (pump.rs:430-433): the `_` arm is
  reachable only as (Joining, stream non-Join): counted. Both tests fail under their plants (fx-p13a.log; p13b).
  The comment at :372-373 says "on a joined session" while a Joining session's forged kind also takes the
  count-and-go-on path: wording only, not a finding.
