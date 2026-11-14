# U4r review, round 1: netcode-architect seat (batch 1)

Snapshot under review: wip/u4r.patch sha256 572c4f69435da2ad6f62d54d4bcbc47b04d6329b5493e0faf78fcecfdc4a09c2 (checked by sha256sum at start).
Packet u4r-packet.md sha256 cbd82485... ; tests file u4r-packet-tests.md sha256 d12555c8... (both checked).
Read copy: .gate/u4r-snap1 (2fbec77 + patch). Logs of my runs (if any): .gate/u4r-review-netcode/.
(No Edit tool in this agent: this file is rewritten whole with Write each time.)

## Plan
1. Read sources: panel §1 + "What every seat is told"; plan §5 U4r (:750-803); brief sections 2, 4, 7; carries §U4r (49, 58, 60, 62, 69, 74, 75, 76, 77, 40, 83a); rulings 1-4.
2. Item (1) runner: scenario.rs, logs.rs, main.rs, bots/src/lib.rs (SimClock, L1Scenario order, header, --bite dispatch, Bites::of, logs vs carry 62, lead, starts, wraps, clamp, rates, TUNE-1, ES bytes, session handling).
3. Item (2) precheck.rs vs plan :772-796, brief 7.2, carries 40, 83(a); bites wallclock (pump.rs:228-246) and late-command; depth relation; result lines.
4. Item (3) mirror.rs: radius copy + counts, room-tick lookup :396-414, last_acked keep.
5. Item (4) tick16 runs, cap exception scenario.rs:1034-1049, ops/runs/c-u4r-runs.mjs settings.
6. Item (5) report statements vs logs (report may be absent: then the build notes' claimed statements vs logs).
7. Item (6) .claude/rules/netcode.md on npt-server edits.
8. Return verdict + findings.

## Progress / findings (a line each as found)
- [done] Sources read: panel §1 + every-seat; plan :745-803; brief 2.1-2.7, 3.1-3.3, 4, 5, 6, 7.1-7.4; rulings 1-4 full.
- [done] Code read: scenario.rs :1-1530; logs.rs whole; mirror.rs whole; precheck.rs whole; bots lib.rs whole; server diff.
- Checked, no finding: stream_of (Control 2 B frame vs FullState 4 B frame makes the trial decode unambiguous even
  under no-channel); room first tick = tick_start (World hit volumes name first_tick - 1), so heal's due_ns(k_of) right;
  kicked flag resets on join (room.rs:659); late-command bite releases T oldest-first, one late; wallclock XOR never 0.
- F-a (Medium, item 1 carry 76): scenario.rs:490-497 a joined session the pump closes (close_malformed: Kick(malformed))
  is dropped but Room::leave(slot) is never called and owner not cleared; Room::leave's doc (room.rs:689) names "a closed
  link". Ghost slot keeps option B reruns + snapshots + roster live.
- F-b (Low, item 1 dispatch): Bites::of accepts PM-0001-30/late-command for `scenario` (any includes late_command,
  scenario.rs:236) but run() never reads it: a scenario bite run names a bite that plants nothing.
- F-c (Low, item 2): precheck verdict uses end-of-leg counters (precheck.rs:455-462); a client dropped/kicked loses its
  late and rewinds_refused counts (room.client None -> default 0), refused_logged per tick is not in the verdict.
- F-d (Low, item 1 carry 77 (a)): writer can emit at most 5 distinct counter names (room: cmd_seq, press, ack; client:
  tick_offset, event_seq); -58 needs >= 6 (marks.json :4204-4210, incl. "server tick's low 16 bits"); no lab tick-wrap
  record. For U4g's brief, not a U4r judged part.
