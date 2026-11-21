# C62H-BOUND (a) design notes (netcode-architect)

Plan: read the record (c62h-review-test-full, advisor-c62h-review-full, history.md:2131-2146), then crates l1/lab/room/server/client/judge, marks.json/SERIES.md/README.md, c-u4r-runs.mjs, u4s-carries.md:518-534; design the closed explanation list, exact texts, size, bites, runs, lane; save each block as finished.

## Block 1: facts from the code (done)
- L1 datagram fate is decided at send, in `Net::datagram` (l1/src/lib.rs:1079-1172): Burst if in a burst (:1097-1098), Lost by the loss draw (:1099-1100), else Delivered{arrival} (:1101-1104); one fate entry pushed per datagram (:1106-1117). The reorder step may later move a *held* datagram's arrival to next arrival + 1 and set overtaken (:1134-1146), never to Lost. A send on a closed link pushes no entry (:1084-1086); too large takes no index (:1087-1089).
- Fate entry fields (l1/src/lib.rs:306-321): link, dir, carries (Datagram / Stream{channel,pn,message,part,parts} / Ack), seq, sent_ns, len, outcome, overtaken, retransmits. No tick, no slot. FateLog entries are private; public API is len, is_empty, for_each_canonical, reorder_counts (:332-438) and L1Net::with_fate_log (:1885-1887).
- Recv to the client carries channel, bytes, arrival_ns only (l1 :1676-1680): no fate index.
- Lab send path: after each room tick, per slot: take_datagrams (room.rs:595-601), take_control, then Server::send (scenario.rs:557-584) -> send_unreliable per group (:569-571), send_reliable per control msg (:572-574). Pings and pongs are sent by the pump (pump.rs:387-404, :467) inside Server::io (scenario.rs:458-545), never inside Server::send. Events applied inside a send push only Ack and Stream entries (l1 :1419-1464, :1543-1584, :1624-1665), never Datagram entries. So the Datagram entries pushed during one Server::send(slot) are exactly that slot's groups for that tick.
- Room: server_hash Some iff the room coded a snapshot or a full state for that client at that tick (room.rs:204-208; set at :1128 and :1357). None on: full state in flight (:1087-1089, NC-L10), budget short (:1173-1176, :1201-1204), encoder refusal (:1119-1122, :1343-1346), kicked (:1047-1049, :1057-1063). Room log writes hs.server_hash per slot each tick (logs.rs:534-564).
- Client files lost only in `see` (client/src/lib.rs:583-615). Every decode failure files the sentinel (:717-807). Stale drop only at >= 32 ticks behind newest (:527-552).
- ACK_RING_TICKS = 32 at 30 Hz, 1.067 s (sim/src/ticks.rs:58-70).
- Lab writer: lost after a hash ignored, hash after lost overrides (logs.rs:368-388); snap_incomplete 1 iff Res::Lost (:442); stop decisions never in the measured window of a valid run (scenario.rs:1126-1137).
- Judge: series sum (eval.rs:282), window (:120-126), empty tick series = gap (:314-327), gap with no failing check -> INVALID (:1215-1231); bite Shown only if every bite run is shown (:1489-1496); figure checks accept parts (marks.rs:543-551, :585-591).

## Block 2: "too late" cannot happen on L1 (done)
- Max datagram delay one_way + half_jitter (P3 90 ms; l1 :215-221, :790-814); the reorder hold adds at most one more delay (:1134-1146): any snapshot datagram arrives within about 180 ms on P3. Tick k+32's is sent 1.067 s later and takes >= 60 ms. A full state in flight blocks every newer send (room.rs:1087-1089). So ring-lateness cannot occur on P0-P4 or B1; stale = 0 on every judged run.

## Block 3: the design (done)
- Closed list: E1 room sent nothing (hs.server_hash null); E2 link lost a snapshot datagram of it (new tick.s.snap_link_lost_count >= 1). Nothing else explains (stale drop, full-state tick, all datagrams arrived, stop decision).
- Record: npt-l1 gains FateLog::server_datagrams_since(from) -> DatagramCounts{sent, lost, burst}; npt-lab marks the fate log length before each Server::send(slot), reads the counts after, asserts sent == groups (and groups non-empty => server_hash Some), writes per slot per tick `tick.s.snap_link_lost_count` in the room log (0 included). Summary line gains measured snapshot datagrams sent / lost by the link (bursts).
- Judge: two figures, snap_lost_unexplained_count (A) and snap_whole_despite_link_loss_count (B), measured window, L1 P2/P3/B1 parts. B catches an overstated record; clean lossy runs catch an understated one (17k-43k real losses must each be explained); missing record = gap = INVALID.
- Bites: whole-as-lost gets a P2 run (A = 48); new PM-0001-17/link-lost-overstated on P2 (B = 48); not on P0.
- Runner: bitesOf runs -17's whole-as-lost and link-lost-overstated on P2; P0 keeps the rest (link-lost-overstated excluded there).
- Size: about 520 (440-600) + JDG-ONLY-1 riding 40-70 counted apart; stop at 1,050.
- Runs: P2 (3 + 2 bites) about 1,734 s, P3 about 1,020.5 s; disk start checks 4,983.6 MiB and 4,620.4 MiB free.
- Questions Q1-Q5 (too late; judge vs lab; bite run length; -58; JDG-ONLY-1).
