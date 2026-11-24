# U6c round 2, the netcode seat (netcode-architect, xhigh), on the fix pass's diff only. Read only.
Inputs, sha256 checked equal (2026-11-24): advisor-u6c-r1.md ad72c0b8…faed179, u6c-review-netcode.md f29e1ac1…,
u6c-review-test.md 79197b42…, u6c-review-security.md c29540fa…, u6c-build.md a9d45dd4…, wip/u6c-fix.patch 0549389e….
Diff: `git diff aad21031… 6c3ff00b…` (10 files, +356/-94). Paths below: W = .claude/worktrees/agent-a3a3e3fdf559de095/
spikes/netcode-proof. (This seat has no Edit tool: the file is rewritten whole with Write at each save.)
No run made (no runner call): the builder's and the main session's logs are the evidence, plus two A2 logs the test
left in /tmp (npt-u6c-load-7849-a2-wt, npt-u6c-load-7253-a2-ws) and the probe reading's logs, read by grep for fields.

## 1. Plan
1. FP-1: read `leg_series` whole in W (net l2/probe.rs) and `run_leg` (server probe.rs) at the changed lines and what
   they call (in_burst, sec, the RTT join, window_count, the E set, the Measured phase, the second rows). Check: exactly
   `packets` outside the bursts, t_in order, tie rule, up for the pinned peer only, the two spans, edge cases (short
   tail, receive batch sharing one t_in, a burst at the end, peer None, start's second vs first record), interaction
   with the RTT join (kept vs echoes ordinal), the measured-phase extension vs the judge's window and the per-second rows.
2. SEC-U6C-4's pin: `Intake::take` with `ProbeDatagram::decode(b, None)`; what decode(None) accepts; can a real peer's
   first datagrams fail it (leg tag, version, size), so the leg pins late or never.
3. NC-U6C-3: `probe_down_unindexed_count` and its SERIES.md row.
4. load.rs riders: NC-U6C-5, NC-U6C-6, SEC-U6C-3's load.rs half, NC-U6C-8's SERIES.md rows.
5. Readings 1-3 (only the code defining each named count, the per-second input count, U8's lines in item 3, and
   tests/load.rs's `a2`).
6. Write notes per finding as I go; verdict, table, per-item status, outside the diff, not checked, DONE.

## 2. Notes (one line per finding)
- SEC-U6C-4 pin (server probe.rs:283): a real peer's every datagram is 40 B, magic NPT2, leg 0-5 (net probe.rs:283-291,
  :489 leg starts 0, :535-547), so decode(b, None) holds from its first datagram: no leg is left without a peer and no
  pin comes later than in round 1. `None` (any leg), not `Some(leg)`, is required: at each leg change the peer still
  tags the old leg until RS's new down stream reaches it, which starts only after the pin (probe.rs:480-485); Some(l)
  would deadlock every leg after P0 until peer_wait_ns. Closed.
- Reading 1 (A2 over WT, min 0): not FP-5's transport handling nor F-NAMES (a2 never uses F-NAMES' rule). From the
  left-behind log /tmp/npt-u6c-load-7849-a2-wt/load.jsonl: sec 0 is Measured (READY came inside it, load.rs:667-683)
  with slot 23 at 0 (joined late in it); secs 1-69 every client 22-31; sec 70 the room's close (about 10 each); secs
  71-99 all 0 (slot 41 alone 30 then 3), because the room ends at secs+10 = 70 s (tests/load.rs:345) and LG runs to
  secs+40 = 100 s (:351) over WT (F-WT-CLOSE: the close reaches LG only at quinn's idle timeout), every second after
  READY Measured (load.rs:679-683), and a2 takes every Measured second (tests/load.rs:371-376). WS's 5 is slot 19 in
  its close second 70 (/tmp/npt-u6c-load-7253-a2-ws/load.jsonl:72); its sec 0 is warm-up. -> NC-U6C-R2-1 (Low).
- FP-1 read (net probe.rs:689-821): peer found first (:700-710, same rule as round 1, now also admitting the peer's
  Lost fates before its first Delivered one); per direction candidates t_in >= start, up the peer's ordinal (:715-720),
  stable sort by t_in (:721), keep until `outside` reaches `packets`, then only fates at the last kept t_in (:723-731);
  rows from sec(start) to the later direction's last record (:733-737); k counts every Delivered up fate before the
  keep test (:759-771), so the echo join stays aligned. Exact by construction, whatever either clock does: up records
  depend only on how many of the peer's fates RS stamps after its own start, down records on RS's own pacer.
- FP-1 against the ten checks: they count `second.s.probe_*_lost` entries in measured seconds (round 1's N1 note), i.e.
  records outside the burst, not `probe_up_count` (the E set's delivered samples: P1 990,031). The Measured extension
  (server probe.rs:603-609) is what keeps P1's 213 or so up records of second 110 in the count: P1's sec 110 is
  `measured`, sec 111 `after` (room-P1.jsonl). Without it the up count would be about 999,787. Closed.
- Ties (a receive batch stamped once): every fate at the last kept t_in is kept, so the count is >= packets, by at most
  the batch size less one; `>= 1000000` passes, the span is unchanged, SERIES.md:917-918 says so. Honest.
- Short tail: fewer fates than packets after the start (LG ended, or kernel drops above the tail's spare of about
  10,300): the records are every fate to the poller's stop, the count is short, the check fails, the span shows how far
  they ran. Honest, with one corner -> NC-U6C-R2-3 (Low): records past the seconds loop's last row lose their rows.
- Measured extension side effect: P1's sec 110 holds about 213 up records and 0 down records while the socket
  delivered 9,906 downs in it (room-P1.jsonl sec 110: `probe_down_lost` [] , `l2_down_delivered_count` 9906). Pooled
  reads and counts are right; a per-second read of Measured probe rows would see a near-empty sample -> NC-U6C-R2-2
  (Low, conditional on the judge, not read here).
- NC-U6C-3 (net probe.rs:739-743, summary :812; SERIES.md:946): counts down fates with index u64::MAX over the whole
  log, which is the join's scope. Closed. The row names QueueDrop and PoolDrop; Oversize fates take NO_INDEX too
  (impair.rs:482, :490-491), never for the probe's 40 B downs -> wording, NC-U6C-R2-4 (Low).
- Reading 2, recv_err/send_err (net probe.rs:515-522, :549-552; socket connected at :481): on a connected UDP socket an
  ICMP port unreachable sets a pending error that the next recv or send returns; a send that returns it is not sent.
  So closed-port sends ≈ recv_err + 2 × send_err = 737,190 + 28,454 = 765,644 ≈ 76.6 s at 10,000/s: the main session's
  76 s confirmed (about 1,009 of them the P0-P1 gap: snmp NoPorts 444 -> 1,453 between P0's end edge and P1's start
  edge). sent + send_err = 2,999,958 ≈ 300 s × 10,000. received 2,209,090 ≈ RS's 2,220,305 downs less P1's 1 % model
  loss over its whole leg (≈ 11,120) less ≈ 137 kernel drops at LG's socket: consistent within about 50.
- Reading 2, udp_drops_count (server probe.rs:95-101): the drops column of the /proc/net/udp row whose LOCAL port is
  30003: RS's probe socket, not the peer's (its row has an ephemeral local port). Start edge (read at :465, after the
  pretouch at :462, before the poller at :478): P0 679, P1 569 = LG's datagrams overflowing the buffer while the fate
  log was pretouched with no reader. Rise between edges: P0 17, P1 222; up span excess P0 1.64 ms (≈ 16 intervals), P1
  21.4 ms (≈ 214): the same datagrams, dropped by RS's kernel before any fate, read past by FP-1 by design. Not a
  defect in FP-1's lines. Bears on U8's P1 rule -> Outside the diff, Medium.
- Reading 2, refused 484 (server probe.rs:286-295): datagrams from the pinned peer that do not decode for leg 1: the
  peer tags the last down's leg (net probe.rs:507, :536, :543), so at P1's start it tags P0 until P1's first down
  reaches it (RS's pacer starts after the pin, :480-485; then P1's down one-way, then P1's up one-way for the ups):
  ≈ one P1 round trip (tail 1.03 s => one-way + half jitter 30 ms) × 10,000 ≈ 500. Their fates lie in P1's first
  ≈ 50 ms of a 10 s warm-up, so none is a record; each pushes a (NO_ECHO, 0) record (:294), keeping the join aligned
  (P1 echo_unmatched 0, records_left 0). Touches neither P1's counts nor its loss (records: up 9,969 lost = 0.997 %,
  down 10,019 = 1.002 %). Repeats at each later leg, in its warm-up.
- Reading 3 (U8 timing): measured legs ≈ 112 s (window 100 + tail 1.00/1.03 + ≈ 1 s post-loop and next bind); a leg
  passes 122 s only with a tail above ≈ 11 s (a one-way delay near 10 s). B1 ≈ 10 + 101 + tail + 1 ≈ 113 s. Six legs
  ≈ 675-680 s against U8's 732 s and LG's 792 s. Nothing moves 122 s or 0.20 h. One peer session across legs: each
  gap ≈ 0.1 s, its effects (ICMP errors at LG, pretouch overflow, stale-tag refusals) all before the window's start.
- Load.rs riders: SEC-U6C-3 (load.rs:265-268) digits only, test "room sign" (load_tests.rs:45): closed. NC-U6C-6
  (load.rs:563-567, :596-599): closed; one return after the header still has no footer (:549, the runtime) -> Outside
  the diff, Low. NC-U6C-5 (load.rs:629-638): HostState::failed is state 2 (load.rs:95-97); HANDSHAKE_NS 10 s < READY_NS
  30 s (:37-39), so by the line every client is Open, closed or failed: the count is complete. Closed (line half).
  NC-U6C-8 (SERIES.md:947-948; LG writes room, mode, transport, behaviour at load.rs:443-448, the probe test checks
  every header param and probe_peer field, tests/load.rs:285-293): closed; two row texts short -> NC-U6C-R2-4.

## 3. Verdict
APPROVE WITH FINDINGS

## 4. Findings
| Id | Severity | Finding | File:line under W | Smallest fix | Test or plant that shows it |
|---|---|---|---|---|---|
| NC-U6C-R2-1 | Low | A2's printed "inputs a client a second" min over WebTransport is 0 by construction: it takes every Measured second (tests/load.rs:371-376), which includes the READY second (partial: slot 23 at 0) and, over WT, the 29 seconds after the room's close (room secs+10 = 70 s at :345, LG secs+40 = 100 s at :351; F-WT-CLOSE keeps LG to its duration). The whole seconds while the room ran (1-69) give 22-31 for all 48 clients. | crates/bots/tests/load.rs:345, :351, :371-376 | Print the min and max over the whole seconds while the room ran: skip the first Measured second and stop before the room's close second (LG seconds below secs + 10 − 1, or the room log's last second); keep the all-seconds pair beside it if wanted | Rerun A2 over WT: the printed min becomes about 22 (the left-behind log, secs 1-69) |
| NC-U6C-R2-2 | Low (conditional) | FP-1's Measured extension (needed for the count) makes a Measured second holding one direction's tail only: P1's sec 110 has about 213 up records, 0 down records, while the socket delivered 9,906 downs in it. Counts and pooled reads are right; any PM-0001-3 read that is per second over Measured probe rows (a per-second loss or delay bound, a "records each way every second" rule) would read a near-empty sample there. Not confirmed: marks.json not read in this round. | crates/server/src/probe.rs:603-609; crates/net/src/l2/probe.rs:733-737 | None if every PM-0001-3 read pools; else the judge reads per-second checks over the nominal window only, or SERIES.md:919 names the extension second so the judge can skip it | A grep of marks.json's PM-0001-3 for per-second reads of `second.s.probe_*` (one reader, before R1) |
| NC-U6C-R2-3 | Low | Records whose second lies past the seconds loop's last row lose their per-second lists silently: `leg_series` has no end bound and `run_leg` attaches series seconds by a peekable match over the loop's rows (probe.rs:610-631), so an unmatched trailing second is dropped while the summary's `probe_*_window_count` still counts it. Needs an up shortfall above the tail's spare (≥ about 10,000 fates missing: heavy kernel drops) and fates stamped after the loop's end (the poller runs until :548-551) in a second with no row (a whole-second tail, as P0's 1.000 s). The check then fails on the rows' count while the summary says the count was reached. | crates/net/src/l2/probe.rs:715-720; crates/server/src/probe.rs:543, :610-631 | Bound the candidates by the loop's end (pass `zero + warm + window + tail` and keep `t_in < end`; a short count stays honest), or count unattached series seconds in the summary | A `leg_series` unit with up fates missing so the count completes past `end`: with the bound, the records stop at `end` |
| NC-U6C-R2-4 | Low | Row wording: `probe_down_unindexed_count` names QueueDrop and PoolDrop, but Oversize fates take no index too (impair.rs:482, :490-491; never for the probe's 40 B downs); `probe_peer_refused_count` says "another length or magic" but decode also refuses a leg above 5 (net probe.rs:298); the send and receive error rows do not say they are mostly ECONNREFUSED from ICMP port unreachable while RS's port is closed (between legs and after RS's last leg), as net probe.rs:472-473 says. R1's LG summary will show about a million of them on a healthy run (reading 2: 751,417 from LG's last 76 s). | marks/SERIES.md:946, :947 | Three wording edits in those rows | None (text) |

Count: Critical 0, High 0, Medium 0, Low 4 (in the diff); outside the diff: Medium 1, Low 2.

## 5. Fix-pass items in my scope, by round-1 id
- **NC-U6C-1 (High), FP-1: closed.** Each direction's records are its first `packets` fates outside the bursts from
  the window's start, in t_in order, ties at the last kept t_in kept, up for the pinned peer only (net
  probe.rs:711-732); `run_leg` passes `zero + warm`, `s.packets`, `interval` (server probe.rs:562-570); the tail (:456)
  holds the extra sends; the Measured phase runs to the last record's second (:603-609), which the count needs (P1's
  sec 110 measured). Exact whatever the two clocks do: the up count depends only on how many of the peer's fates RS
  stamps after its own start, with about 1.03 s (≈ 10,300 fates) of tail to spare; the down count on RS's own pacer.
  Measured on the container: P0 and P1 1,000,000 records a direction (`probe_*_window_count`). The spans
  `probe_up_window_ns` and `probe_down_window_ns` (net probe.rs:805-808) are as defined: P1's down 100,000,078,360 =
  the pacer's slot 1,099,999 (zero + 109.99995 s) plus ≈ 28 µs lateness minus the start plus 100 µs; the up spans'
  excess (P0 1.64 ms, P1 21.4 ms) matches RS's socket drops between the edges (17, 222). SERIES.md:916-919 and :933-935
  match the code. "Fewer than packets: the count is what there is" is honest at both edges asked: a receive batch on
  one t_in gives >= packets (documented); a short tail gives a short count and a failing check, with NC-U6C-R2-3's
  corner. Low follow-ups: NC-U6C-R2-2, NC-U6C-R2-3.
- **SEC-U6C-4's pin (netcode half): closed.** A real peer's datagrams always decode with `None` (40 B, NPT2, leg 0-5,
  the peer's leg starting at 0: net probe.rs:283-291, :489), so no leg is left without a peer and none pins later.
  `None` rather than `Some(leg)` is what keeps later legs from waiting on a peer that still tags the previous leg.
  What the pin no longer accepts (a non-probe datagram from LG's IPv4) is recorded OTHER and counted, as before.
- **NC-U6C-3 (Low): closed.** `probe_down_unindexed_count` (net probe.rs:739-743, :812) counts every down fate with no
  index over the whole log (the join's scope); 0 on P0 and P1. Row wording: NC-U6C-R2-4.
- **NC-U6C-5 = F-FAILED (Low): closed for the line** (load.rs:629-638: `handshake_failed_count=<n>`, complete by the
  line since HANDSHAKE_NS 10 s < READY_NS 30 s). Its log half goes to U6e (Q-FP6-1, the main session's lean (b)).
- **NC-U6C-6 (Low): closed** for the two returns named (load.rs:563-567, :596-599: footer `stopped` before `fail`).
  One more return after the header has no footer (outside the diff, below).
- **SEC-U6C-3, load.rs half (Low): closed** (load.rs:265-268 digits only; load_tests.rs:45 "+5" refused).
- **NC-U6C-8 = F-SERIES-LOAD (Low): closed** (SERIES.md:947-948; tests/load.rs:285-293 checks every `probe_peer` field
  and header param of LG's probe log). Row texts: NC-U6C-R2-4.

### The main session's readings, weighed
1. **A2 over WebTransport, min 0: a start and end second the reading counts and no bar does, not a defect in FP-5's
   transport lines or in F-NAMES' rule.** `a2` (tests/load.rs:333-388) never applies F-NAMES' rule (that is
   hosts_four's, :176-193); it prints the min over every Measured second (:371-376). LG marks every second after READY
   Measured (load.rs:679-683). In the left-behind log (/tmp/npt-u6c-load-7849-a2-wt/load.jsonl): sec 0 is Measured
   (READY inside it) with slot 23 at 0; secs 1-69 every one of the 48 clients 22-31; sec 70 the room's close (about
   10 each); secs 71-99 all 0, except slot 41 with 30 then 3. The room ends at secs + 10 = 70 s (:345) and LG at its
   own secs + 40 = 100 s over WT (:351, F-WT-CLOSE). WebSocket's 5 is slot 19 in its close second 70
   (/tmp/npt-u6c-load-7253-a2-ws/load.jsonl:72); its sec 0 is warm-up. So 48 WT clients gave commands every whole
   second while the room ran. NC-U6C-R2-1 (Low) makes the printed figure say so. Slot 41's 33 commands in the 1.1 s
   after the others stopped exceed MAX_LEAD's 9 (load.rs:45): cause not traced, for F-WT-CLOSE's takers (U6d, U6e).
2. **The probe reading: no defect in FP-1's lines; nothing changes U8's 122 s or R1's 0.20 h.**
   - recv_err counts the reader's errors other than WouldBlock and TimedOut (net probe.rs:515-522); send_err the failed
     sends (:549-552). The socket is connected (:481), so each ICMP port unreachable surfaces once, as ECONNREFUSED on
     the next recv or send, and a failing send is not sent. Then closed-port sends ≈ recv_err + 2 × send_err =
     765,644 ≈ 76.6 s at 10,000 a second: the main session's arithmetic holds, a little closer than its 760,000. About
     1,009 of them are the P0-P1 gap (snmp `no_ports` 444 at P0's end edge, 1,453 at P1's start edge); the rest are
     LG's last ≈ 76 s after RS's last leg (an artifact of the reading's 300 s peer run, as the main session says).
   - `udp_drops_count` (server probe.rs:95-101) is the drops column of the /proc/net/udp row whose local port is the
     probe's: RS's own socket, not the peer's. The start value (P0 679, P1 569) is LG's datagrams overflowing the
     receive buffer during the fate log's pretouch (:462), before the poller starts (:478). The rise between the two
     edges (P0 17, P1 222) is datagrams RS's kernel dropped during the leg (the poller behind); they have no fate. They
     explain the up spans' excess (P0 ≈ 16 intervals, P1 ≈ 214): FP-1 read past them and kept exactly 1,000,000. They
     bear on P1's span, not on its counts or its loss. Machine-wide RcvbufErrors rose 359 in P1: the other ≈ 137 are
     likely at LG's socket (downs the model delivered and LG never took, invisible to the model's down loss).
   - `probe_refused_count` 484: a refused datagram still makes an up fate and an up record if its t_in is at or after
     the start; these lie in P1's first ≈ 50 ms of a 10 s warm-up, so none is a record. Each pushes (NO_ECHO, 0), so
     the echo join stays aligned. It touches neither P1's counts nor its loss. P1's loss by the records: up 9,969
     (0.997 %), down 10,019 (1.002 %), not 1.1 %. It repeats at every later leg's start, inside its warm-up.
   - The 21.4 ms: the kernel drops, not the kept-records rule, the tail or the spans.
3. **Coverage for U8:** no leg can pass 122 s short of a profile tail near 11 s. Measured legs ≈ 112 s: window 100 s,
   tail 1.00 or 1.03 s (server probe.rs:456), ≈ 1 s of post-loop work and the next bind (drain ≈ 80 ms, the two sorts,
   the log's build and write). P2-P4 add only their tails (one-way + half jitter + 1 s); their fate logs are sized the
   same, so drain and write stay ≈ 1 s. B1 ≈ 10 + 101 + its tail + 1 ≈ 113 s. Six legs ≈ 675-680 s against U8's 732
   s and LG's 792 s (u8-build.patch:1712, :1878-1881); the block waits for LG's end anyway (:1782-1784), so its 0.23 h
   stands as U8 states it. One peer session across six legs: each gap is ≈ 0.1 s with no socket on the port (the log
   write and the next bind). Its effects are ICMP errors at LG, the new socket's pretouch overflow and ≈ one round
   trip of stale-tag refusals, all before the window's start. None reaches a record or a count the marks read. No idle
   second shows because the gap is a tenth of LG's second.

## 6. Outside the diff
- **Medium, before R1 (U8's C9 edit, or U6c's merge):** RS reads each leg's start edge while its receive buffer is
  full and no reader runs: the pretouch (server probe.rs:462) runs before the edge read (:465) and the poller (:478),
  with LG sending since the previous leg (P1's start value 569). Any datagram arriving between that read and the
  poller's first receive is dropped and counted between the edges. U8's rule `udp_drops_count` "equal at both on P1"
  (u8-build.patch:1709-1710) would then send a healthy P1 to the advisor at R1 ("a P1 drop goes to the advisor before
  any L2 mark counts"). How likely: one datagram per 100 µs against a read-to-receive gap of likely 0.1-1 ms. Reading
  2 fits: P1's 222 drops less its ≈ 214-interval span excess leaves ≈ 8 outside the window. Separately, the
  container's own P1 already fails that rule (222 drops during the leg). Smallest fix: read `udp_drops_count` once
  more at the window's start edge (the seconds loop's `edge` hook, :513-521, which already reads slack and policy), as
  a header field, and have U8 compare the end edge with it; or start the poller before the pretouch.
- **Low, after R1 (U6e):** load.rs:543-550: a failed runtime build returns `fail(1, "the runtime")` after the header
  (:539) with no footer, the one such return NC-U6C-6 left (NC-SB-2).
- **Low, after R1 (U6d, U6e: F-WT-CLOSE):** in A2 over WT, slot 41 gave 33 commands in the 1.1 s after the room's
  close while every other client stopped within its close second; MAX_LEAD is 9 (load.rs:45). Cause not traced.

## 7. What I did not check
- I ran nothing (no runner, no cargo, no plant). FP-1's ties and short tail are reasoned from the code; no test pins
  them (the FP-1 test, net probe_tests.rs:294, uses distinct stamps and a full tail). The test seat reviews the tests.
- marks.json not read: NC-U6C-R2-2 is conditional on whether any PM-0001-3 read is per second over Measured probe rows.
  That the ten checks count `second.s.probe_*_lost` entries comes from round 1's netcode note N1, not a re-read.
- The ICMP error rule (one pending error per ICMP on a connected UDP socket, a failing send not sent) is Linux's
  documented behaviour as I know it. The 765,644 figure assumes every closed-port send drew an ICMP (no rate limit on
  this loopback). Not traced in the kernel here. On R1's real network RS's ICMP rate limit would cut these errors.
- P2, P3, P4 and B1's profiles (their delays, hence their tails) were not read; reading 3's bound holds for any
  profile with one-way plus half jitter under about 9 s.
- U8's patch read only at :1708-1715, :1780-1786 and :1876-1881. The P1 rule at :1709-1710 lies inside the first span.
- Slot 41's late commands and the WT close path were not traced. U7's dist, serve.rs and the plants belong to the
  other seats.

DONE
