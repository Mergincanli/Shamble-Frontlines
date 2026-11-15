# C62H-BOUND (a) design: test-engineer read (proof)

Plan: read the record (c62h-review-test-full, advisor-c62h-review-full, history.md:2131-2146, c62h-bound-design), then verify each claim against code (l1, lab scenario/logs, room, client, judge figures/eval/marks, marks.json, SERIES.md, runner), save findings per block, then verdict.

## Blocks
## Block 1: code facts verified (l1, room, client, lab writer)
- l1 Net::datagram decides Lost/Burst at send (lib.rs:1097-1105), entry pushed :1106-1117; reorder only moves a held Delivered arrival (:1134-1146); closed link pushes no entry and returns Sent (:1084-1086); TooLarge pushes none (:1087-1089). Events applied inside a send push only Ack (send_ack :1419-1464) and Stream (send_piece via detect_lost :1581-1583, on_timer :1661-1663) entries. close() clears inbox + held (:1685-1695); arrive() drops when receiver closed (:1333-1335). Inbox unbounded heap; poll gate :1670-1672.
- room: server_hash reset in begin_tick (room.rs:299-300), set only at :1128 (full state, after encode OK) and :1357 (snapshot, after every group coded); budget short returns before (:1173-1176, :1200-1204); refusals return before (:1119-1122, :1343-1346); kicked skipped (:1047-1049) or kicked on FS ack timeout (:1057-1063); in_flight blocks snapshots (:1087-1089); c.datagrams set only at :1380, taken each tick (:595-601).
- client: lost filed only in see() when a tick leaves the ring (client lib.rs:583-615, ring_end=min(n+1,t-31)); groups too old -> stale (:527-552); refusals -> sentinel (:717-807); full state dup if got!=0 (:817-820).
- lab writer: client record written when any slot resolved (logs.rs:433); phase = room tick's phase (:415, :468-474); Open slot -> null + incomplete 0 (:435-442); lost then hash -> hash (:368-373); bite whole-as-lost site :317-330, :362-366.
- Server::send: no session -> Ok(()) with nothing sent (scenario.rs:566-568); send_unreliable result discarded (:569-571).
