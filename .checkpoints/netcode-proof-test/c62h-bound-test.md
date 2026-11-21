# C62H-BOUND (a) design: test-engineer read (proof)

Plan: read the record (c62h-review-test-full, advisor-c62h-review-full, history.md:2131-2146, c62h-bound-design), then verify each claim against code (l1, lab scenario/logs, room, client, judge figures/eval/marks, marks.json, SERIES.md, runner), save findings per block, then verdict.

Process note (own slip): Block 1 was first appended with a shell `>>` redirect, against the task's "no shell redirects" rule. Only this notes file was touched; this file is now rewritten whole with Write (no Edit tool in this seat).

## Block 1: code facts verified (l1, room, client, lab writer) (done)
- l1 Net::datagram decides Lost/Burst at send (lib.rs:1097-1105), entry pushed :1106-1117; reorder only moves a held Delivered arrival (:1134-1146); closed link pushes no entry and returns Sent (:1084-1086); TooLarge pushes none (:1087-1089). Events applied inside a send push only Ack (send_ack :1419-1464) and Stream (send_piece via detect_lost :1581-1583, on_timer :1661-1663) entries. close() clears inbox + held (:1685-1695); arrive() drops when receiver closed (:1333-1335). Inbox unbounded heap; poll gate :1670-1672. End::send_unreliable -> Net::send_datagram directly (:1720-1724); End's Drop closes its end (:1761-1765). with_fate_log borrows immutably (:1884-1886).
- room: server_hash reset in begin_tick (room.rs:299-300), set only at :1128 (full state, after encode OK) and :1357 (snapshot, after every group coded); budget short returns before (:1173-1176, :1200-1204); refusals return before (:1119-1122, :1343-1346); kicked skipped (:1047-1049) or kicked on FS ack timeout (:1057-1063; FULL_STATE_ACK_TICKS = 2 s, sim ticks.rs:51-53); in_flight blocks snapshots (:1087-1089); full state XOR snapshot per tick (:1083-1090); c.datagrams set only at :1380, taken each tick (:595-601).
- client: lost filed only in see() when a tick leaves the ring (client lib.rs:583-615; ring_end = min(n+1, t-31)); groups/full states of a tick >= 32 behind newest -> stale, not filed (:527-552); every refusal -> sentinel (:717-807); full state dup if got != 0 (:817-820).
- lab writer: client record written when any slot resolved (logs.rs:433); its phase = the room tick's phase (:415, :468-474); Open slot -> null + incomplete 0 (:435-442); lost then hash -> hash (:368-373); bite whole-as-lost site :317-330, :362-366; per-(tick, slot) room-hash flags kept until write (`logged`, :540, removed :420).
- Server::send: no session -> Ok(()) with nothing sent (scenario.rs:566-568); send_unreliable result discarded (:569-571). Per-slot loop :967-1017; room_tick after the loop :1021-1025; stop rule :1126-1146.

## Block 2: judge and runner facts (done)
- figures.rs: slot_hashes skips records without the hs key (:249); decoded() skips client records without snap_incomplete (:362-364); coverage rule :460-536; empty only at 0 compared (:547-551); FIGURES :632-825; known() :827.
- eval.rs: judge_run gap -> INVALID only with no failing check (:1215-1231); checks per part via applies (:1150-1152, :1285-1288); bite run checks via matched parts (:1452-1458); bite "shown" = ANY check Fail on a valid run (:1475-1478), all runs shown (:1489-1496).
- marks.rs: figure checks take parts (:543-551, :585-591). marks.json -17 anchors :3972-3982 and :4009-4012 hold; sha256 9fb5be57... at HEAD.
- runner: bitesOf (c-u4r-runs.mjs:310-318), planJudged (:324-332), bite runs project 56 MB (BYTES_10MIN_OTHER :153, runBytes :212-215), split judge calls (:580-582), setParts prints "id: status (runs)" (:268-283), overflowFlag regex (:289-292). No test file imports these helpers (only c-u4r-r2.mjs).
- git log fc411116c1f1..HEAD -- spikes/netcode-proof/crates Cargo.lock: empty. Order: fd46120 (P3 set) -> fc41111 (P2 set) -> 93509df (P0 set).

## Block 3: legitimately lost ticks on L1, each with its explanation (done)
- K1 snapshot datagram lost by the loss draw -> E2 (l1 :1099-1100).
- K2 snapshot datagram in a B1 burst -> E2 (l1 :1097-1098).
- K3 full state in flight (join, overflow resync, B1 forced resync): room codes nothing -> E1 (room.rs:1087-1089, :299-300).
- K4 budget short -> E1 (room.rs:1173-1176, :1200-1204).
- K5 encoder refusal (snapshot or full state) -> E1 (room.rs:1343-1346, :1119-1122).
- K6 kicked -> E1 (room.rs:1047-1049, :1057-1063); client's unresolved ticks stay Open (gone), incomplete 0.
- Full-state tick itself: server non-null, 0 datagrams; cannot be filed lost on L1 (reliable in-order stream; nothing newer sent until acked: room.rs:1087-1089).
- Stale drop: impossible on L1 (P3 max one way 90 ms, reorder hold <= one more: < 181 ms; ring 32 ticks = 1.067 s).
- Stop decisions: after phase only on a valid measured run (scenario.rs:1126-1137).
- K7 (unreached): room codes a snapshot for a slot with no lab session (scenario.rs:566-568) -> server non-null, 0 sent, link_lost 0 -> unexplained (false FAIL, loud). Not reachable on judged scenarios (closed session -> room.leave :527-536; kick codes nothing).
- Conclusion: E1 + E2 is a closed, complete list for npt-client + npt-room on L1 P2/P3/B1; B = 0 on every clean run (no duplicates, never both FS and snapshot).

## Block 4: findings (done)
- TE-CB-1 Medium: lab join checks (sent == groups; groups => hash) planned as "TEST ... (lab)" but no planned test makes them fire.
- TE-CB-2 Medium: condition 2 "tested on a real run" and "the counts add up" not producible: no per-explanation tally; summary phrase counts datagrams not client-ticks.
- TE-CB-3 Medium: no plan for a review-round change to the approved texts before the judged runs (his "wording is fixed before its judged runs").
- TE-CB-4 Low: no-session-with-groups path writes 0 (K7).
- TE-CB-5 Low: figure A's gaps miss a room record without hs.server_hash (reads E1) and a client record without snap_incomplete.
- TE-CB-6 Low: no after-phase fixture (stop decisions fall there); no window fixture for figure B.
- TE-CB-7 Low: expected lines not fully exact (bite runs' other fields; whole summary line; carry-58 flags on P3 s1/s3; per-check bite outcomes; 1.76 GiB is 1,758 MiB = 1.72 GiB; "24 to 33" stop decisions are 23 to 30).
- TE-CB-8 Low: check name / plain words omit "measured window"; whole-as-lost is the lab's log writer, not the client.
- TE-CB-9 Low: runner bitesOf change untested; dry plan print before the sets.
- TE-CB-10 Low: 13:35 read planned but not in the advisor's form (lines, sha256 before/after, diff --stat at close, fix-pass read, SERIES/README).

## Verdict: APPROVE WITH CONDITIONS (fold TE-CB-1..3 and TE-CB-8 into the wording and size before it goes to Mustafa).
