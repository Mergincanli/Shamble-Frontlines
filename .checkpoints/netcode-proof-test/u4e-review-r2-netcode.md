# U4e round 2, netcode-architect seat (read only; log prefix nc)

Started 2026-11-12. Packet: u4e-r2-packet.md. Fixed copy: .claude/worktrees/wf_0fa6595a-3e2-1 (uncommitted on 97bb9d3).
Base: .gate/u4e-r1. Judged against advisor-u4e-r1.md R1-R8 and T1. No Edit tool in this seat: the notes are rewritten
whole with Write each time (T4 item 1). No run made (none needed for a reading; runs named for the test seat).
Line numbers are the fixed copy's unless said.

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
  The two new pub counter fields (`ClientCounters::resyncs_limited`, `Counters::joining_dropped`) are the "own
  name" counters R3 and R5 ask for, fields of already-public structs: not a new public item beyond R8 by my reading
  (the engine-architect's seat rules on public items).
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
  client side. A case-3 group runs no `event_number` and no `rebuild_sequences` (it returns first): the bitten header
  moves no client counter.
- FX-1, honest runs: no group or full state can name a tick before `first` (the room sends no snapshot while the
  join's full state is unacknowledged: room.rs:1083-1089; FullState is the first header; L1 duplicates nothing), so
  case 3 is unreachable for a correct build: stricter, never looser. Wrap-safe at the 32-bit wrap too (a post-wrap
  tick is after newest: case 1). Lab compare (server/tests/common/mod.rs:370-380) counts a sentinel or a client key
  with no or another server hash as bad, as SERIES.md:266 does ("A client hash with no server hash counts on every
  machine"); under the bite the coverage gap (ticks 14..65,519 unresolved) is outranked by "a mismatch still fails the
  run" (SERIES.md:266; carry 73: judge/src/figures.rs:542-554): FAIL, not INVALID, at U4r. The judged bite runs start
  near the wrap (-58's `ticks_below_wrap` 30; -17/tick16's "the bite run must cross it": marks.json:3991-3993), so
  every post-wrap low-16 header (0 .. run length) is below every client's `first` (about 65,506 on): case 3 throughout.
- FX-1, the tests (server/tests/loopback.rs:264-348; the twin :296-332): clean: 58 compared, 0 bad, no key below
  65,520, 65,537's pair equal; bitten (both ids): 58 compared, 28 bad, slot 0's orphans [0..13] (14, Q-1's
  prediction), none at or past 65,536, stale 0 (fx-t5print.log:18-22). Plant (a) fails the twin at loopback.rs:328
  (fx-p1a.log:46-74), plant (b) fails the clean test at :282 in s03 and s04 (fx-p1b-s0{3,4}.log). The renamed twin is
  named nowhere else (grep of .mjs, .json, .md, .rs, .toml). NC-H1 CLOSED.
- NC2-L1 (Low) case 3's form beyond "the sentinel under t" is held by reading only. Under the bite slots 0..15 hold
  no live tick (the ring holds 65,504..65,535 and only 65,520 on were used), and a case-3 tick is behind `newest` by
  definition (`see` cannot move it back), so neither test can tell R1's form (no `slot_for`, never acknowledged, once)
  from one that routes case 3 through `see` + `slot_for`, or that acknowledges it (the room would widen 0..13 to
  65,536..65,549, ticks it did send: Held, and the output stays the same). Honest runs never reach case 3, and a
  bitten run fails either way, so no judged outcome depends on it today; it guards the form R1 set. Cheapest proof
  (the test seat's, by plant): in `bad_header` (client/src/lib.rs:521-530) add `let i = self.slot_for(t);
  self.acknowledge(t, 0);` before the push, run `cargo test --locked --all-features -p npt-client -p npt-server`:
  predicted every test passes. A direct endpoint test would pin it: join at 100, feed ticks to about 140 (slot
  60 % 32 = 28 live with tick 124), then a group of tick 60, fed twice: `(60, 0)` filed once, no ack naming 60 in the
  next `Input`, `newest_tick()` unchanged, and a delta on baseline 124 still decodes. Follow-up, not before merge.
- FX-2 NC-M3 (server/src/pump.rs:430, :453-470): `pong(&p, r.arrival_ns)`; the test pumps 20 ms after arrival and
  pins 100 ms (server/tests/pump.rs:328-333); its plant fails it by name (fx-p2.log:18-51). Effect on later marks: a
  `ws` session's RTT loses its +0 to 33 ms bias, so -50's gap check and U4h's tolerance read the true value
  (stricter). NC-M3 CLOSED.
- FX-4 A2 (pump.rs:180-203, :295-303): `RttSource` chosen at `Decoder::new`, `rtt_ns` matches on it, no `or_else`;
  every caller passes one (the lab and pump tests' `decoder` pass `Link`, e10 and two tests `Pings`); nothing else in
  the workspace reads `rtt_ns` from a Decoder. Its two tests fail under the plant (fx-p4.log:31-91). No session can
  mix. Matches §4.9 :352-354 (read at 1e8c188). Met.
- FX-13 SEC-U4E-1 (pump.rs:371-392): in AwaitJoin a stream message refused as WrongChannel or WrongDirection counts
  `first_not_join` and closes with Kick(malformed) and returns (the Join behind it never read); joined sessions keep
  §4.10's count-and-go-on; other stream refusals close as before. SEC-U4E-3 (pump.rs:432-434): the `_` arm is
  reachable only as (Joining, stream non-Join): counted. Both tests fail under their plants (fx-p13a.log:18-47,
  fx-p13b.log:18-51). The comment at :373-374 says "on a joined session" while a Joining session's forged kind also
  takes count-and-go-on (R5 ruled only AwaitJoin): wording, not a finding. Met.
- FX-7, the Resync limit (room.rs:716-737, :1123-1125): `last_full_state` set only when a full state is encoded and
  sent (join's, forced, answered; a carry-54 refusal leaves it and `full_state_due` for the next tick); a Resync is
  answered only when the answer's tick (`next_tick`) is at least `FULL_STATE_ACK_TICKS` after it (by path), else
  `resyncs_limited`; one in flight still `resyncs_dropped` first; the overflow's forced resync never consults it
  (route_events :1023-1034). The SEC-U4E-4 test pins both edges exactly (every Resync at next - f < 60 limited, the one
  at next - f = 60 answered: room/tests/room.rs:412-467); its plant fails it (fx-p7.log:12-46). e9 (:359-410) now runs
  60 ticks first so its first Resync is not limited; its old assertions kept. R3's rule as written. Met.
- NC2-M1 (Medium) the limit's drop (R3's rule, built as ruled) leaves a correct client that resyncs within 2 s of its
  last full state with no answer and no sign, and a later judged mark meets exactly that. ADR 0004 part 20 (:582) and
  ADR 0001's failure table (:771) recover a Worker trap "through ADR 0001's resync path" (`Resync`); PM-0004-61 (ADR
  0004:1182) injects "20 traps per browser at random ticks (proposed)", each "within ADR 0001's join mark (a complete
  world within 300 ms at P2)", 20 of 20; PM-0001-19's ML Chrome part checks `untimed_trap_count` == 0 and every join
  within 300 ms (marks.json, -19). R3's premise ("an honest client ... never resyncs twice in 2 s") holds for players
  but not for that injector: 20 uniform traps over a 10-minute session leave every gap above 2 s with probability
  about (1 - 42/600)^20 = 0.23 (arithmetic), and a trap within 2 s of the join is limited too (the join's full state
  counts). Each such trap is dropped (`resyncs_limited`), gets no FullState, and fails -19 and PM-0004-61 on a correct
  build (a false FAIL, never a pass). Deferring the answer would not save the 300 ms either; the catalogue's Resync row
  (docs/design/netcode-proof-messages.md:90) still names only the one-in-flight rule, so U6's client cannot know a
  Resync may go unanswered. No U4 mark reads it (the endpoint sends no Resync; -22's forced resync is not limited).
  Route (the advisor's): a carry with C-U6-12 and the S20 / -19 run briefs: the trap injector draws each trap at least
  `FULL_STATE_ACK_TICKS` plus the rejoin after the client's last full state (its join's or the last trap's), or the
  limit's form is revisited before those runs (a burst of two, or the full state counted against the window, R3's
  other option); the client's resync path re-sends a Resync that brought no FullState after `FULL_STATE_ACK_TICKS`;
  the catalogue's Resync row gains the limit and `resyncs_limited`. Cheapest proof: reading (the two ADR rows and
  -19's checks); by run, the SEC-U4E-4 test already shows the drop. Not before merge.
- A1 (my focus's edge): the lab's `stream_of` (server/tests/common/mod.rs:44-58) reads the type byte at 2 then 4; a
  FullState's third length byte is 0 below 65,536 B and 1 (TYPES[0] = Input, a datagram row) at the 65,536 B limit, so
  it never misroutes within `Limits::FULL_STATE`. Clean bytes equal `npt_proto::encode`'s (the A1 test); under a bite
  the full state now carries the bite too (only the snapshot header carries tick16, so FX-1's outcome is unchanged):
  more faithful, no honest-run change. `send_control` (room.rs:325-333) drops an unencodable JoinAccepted or Kick
  silently, but every field of both is a plain integer in range by type: not a finding.
- A4 (room.rs:615-633, :829-839): the script waits in its own vectors and moves into that tick's input; `tick_input()`
  after `tick()` is the whole input; the step sees the same content at the same tick as before: no determinism change,
  and a recording or twin stepped on `tick_input()` now gets the scripted spawns. Met.
- Carry 50 (room.rs:1211-1226): `route_events` expires events past age 255 (:985) before `encode_all` for every
  unkicked client in the same tick, so the `continue` is unreachable in a correct room (debug assert; in release the
  event stays queued, unsent, and the next `expire` drops it). The test shows 255 carried, none at 256 (fx-pg.log's
  plant trips the debug assert at room.rs:1214, under the test's name). SEC-U4E-7 (client/src/lib.rs:582-593): checked
  add, a new run on overflow; `see` iterates at most the ring per header (the bulk run is one call). Met.
- Counted runs (fx-counted1.log, fx-counted2.log): "check: all 10 steps pass", 473 / 541 / 52 of 52, = 458 + 15,
  524 + 17, the 17 new tests by name in the record. Not re-run by me (the test seat makes the counted run).

## Findings, ranked
1. NC2-M1 Medium: the Resync limit drops a correct client's Resync within 2 s of its last full state; PM-0004-61's
   random-tick traps and -19's trap checks would FAIL a correct build; the catalogue row lacks the limit. To the
   advisor (R8: a carry or follow-up). Not before merge.
2. NC2-L1 Low: case 3's no-slot, no-ack and once properties are pinned by reading only; a plant routing case 3
   through `slot_for` and `acknowledge` passes every test (the test seat's run settles it). Follow-up. Not before merge.

## Round-1 findings of my seat in the fix pass
- NC-H1 (FX-1): CLOSED (case 3 as R1 and T1; the twin and the clean test assert what the judge counts; both plants
  fail by name).
- NC-M3 (FX-2): CLOSED (arrival time; test and plant).
- In my focus, other seats' ids: A2 met, SEC-U4E-1 met, SEC-U4E-3 met, SEC-U4E-4's room half met as R3 words it (with
  NC2-M1 on its consequence), SEC-U4E-7 met.

## Verdict
APPROVE WITH FINDINGS: no Critical or High; one Medium (NC2-M1, a carry for the advisor) and one Low (NC2-L1, a
follow-up). Nothing the fix pass changed goes beyond R8's scope or bounds; no change can make a later honest judged
run INVALID, and none lets a mark pass more easily (case 3 is stricter, NC-M3 removes a bias, A2 removes a fallback).
