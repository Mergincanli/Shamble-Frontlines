# U4r review, round 1: netcode-architect seat (batch 1)

Snapshot under review: wip/u4r.patch sha256 572c4f69435da2ad6f62d54d4bcbc47b04d6329b5493e0faf78fcecfdc4a09c2 (checked by sha256sum at start).
Packet u4r-packet.md sha256 cbd82485... ; tests file u4r-packet-tests.md sha256 d12555c8... (both checked).
Read copy: .gate/u4r-snap1 (2fbec77 + patch). No cargo or check.mjs run (none needed). Two read-only python probes over
the builder's .gate/u4r/ logs: u4r-review-netcode-probe1.py, -probe2.py (this folder; print only, write nothing).
(No Edit tool in this agent: this file is rewritten whole with Write each time.)

## Plan
1. Read sources: panel §1 + "What every seat is told"; plan §5 U4r (:750-803); brief sections 2, 4, 7; carries §U4r; rulings 1-4.
2. Item (1) runner: scenario.rs, logs.rs, main.rs, bots/src/lib.rs.
3. Item (2) precheck.rs; bites wallclock and late-command; depth relation; result lines.
4. Item (3) mirror.rs.
5. Item (4) tick16 runs, cap exception, ops/runs/c-u4r-runs.mjs settings.
6. Item (5) report statements vs logs (no report on disk: notes' figures vs logs).
7. Item (6) .claude/rules/netcode.md on npt-server edits.
8. Return verdict + findings.

## Progress (all done)
- [done] Sources read: panel §1 + every-seat; plan :745-803; brief 2.1-2.7, 3.1-3.3, 4, 5, 6, 7.1-7.4; rulings 1-4 full.
- [done] Code read: scenario.rs :1-1730 and test list; logs.rs whole; mirror.rs whole; precheck.rs whole; bots lib.rs
  whole; main.rs diff; npt-server diff (pump.rs, schedule.rs, tests/schedule.rs); lib.rs diff; both manifests;
  c-u4r-runs.mjs and c-u4r-r2.mjs whole; replay.rs head.
- [done] Probes over .gate/u4r/r2 (builder evidence, pre-commit): b1 worst slot 21 at 246/900 = 0.273333; b1 96 heals
  max 129.6 ms, 0 full states; b1-nofallback(2) 96 heals, max 1,688,286,323 ns, 71 slow all after a full state, fast
  14 at (burst 0, age 0) + 11 at (burst 1, ages 33/34); ES 1,001 reliable events in 20 s = 50.05/s, 1,440 shots;
  tick16d tick_start 64,651, ticks_below_wrap 885, measured 64,951-65,550 (wrap at measured tick 585), everyone
  28,800/28,800, after 46, after_capped false, 48 overflow, 0 breaches; tick16-clean tick_start 64,936, measured
  65,236-65,835, everyone 1.0, after 0, 0 breaches; every clean short run late < 1 % (max b1 0.617 %), stale 0,
  rewinds_refused 0; d13.log lines match ruling 1 point 8.
- Checked, no finding: SimClock monotone; RQ8 order (poll_accept, io, tick+send, clients in slot order, inbox poll);
  L1 gate_ns < now (never same instant); stream_of unambiguous (Control 2 B frame vs FullState 4 B frame, even under
  no-channel); room first tick = tick_start, so the heal's due_ns(k_of) is right; kicked flag resets on join;
  header params (scene/clients/profile/seed/cmd_timing/tick_start/ticks_below_wrap/suite/stand_ins, lead, starts,
  weapon+health, event cap+provisional, burst_starts_ns+burst_spacing_ns, features, bite, binary sha);
  ticks_below_wrap 0 at tick 0 matches SERIES.md:253's words; Bites::of includes b.pump (ruling 2 p5 (b));
  late-command bite: T released oldest-first, exactly one late, off_stamp 0; wallclock XOR never 0, aim only;
  depth relation and r6's whole-list compare with configured L (ruling 3 p5); result lines carry build, recording,
  leg config with ring, refused per command, depth (ruling 2 p1, p2); mirror radius counts apart + full breach
  counter (ruling 1 p2), room-tick lookup (ruling 3 p7 (b)), last_acked keep (ruling 3 p8 (A)); cap exception keyed
  on bite id (ruling 3 p7 (a)); c-u4r-runs.mjs: tick16 at 47,251, others at 63,736, no -18 bite, B1 kept for -57
  (rulings 3 p1, p3; 4 p3); c-u4r-r2.mjs tick16 expect text amended (ruling 4 p1); npt-server edits bites-only,
  allow on the site alone, field rule held (netcode.md: measured build unchanged).

## Findings
- F-a Medium (item 1, carry 76's boundary): scenario.rs:490-497. A joined session the pump closes (stream malformed:
  pump.rs:419-421 -> close_malformed :364-367, Kick(malformed)) is dropped by the lab, but Room::leave(slot) is never
  called and `owner` keeps the slot. Room::leave's doc (room.rs:689) names "a closed link" as the caller's case. The
  ghost slot stays live in the roster, keeps option B's rerun of its last command each tick (fill_input
  room.rs:841-876), keeps being encoded and queuing events until an overflow kick. Not reached by any U4r run
  (honest clients); reached by U4f's hostile clients on this glue. Fix: in the closed branch, when
  owner.get(&slot) == Some(&x.id): self.room.leave(slot); self.owner.remove(&slot); then push (slot, "closed"); add a
  plant-failing unit test beside r8/r9 (a joined session sends a malformed Control frame; afterwards room.client(slot)
  is None and the roster no longer holds it live).
- F-b Low (item 1, dispatch): scenario.rs:226, :229-236. Bites::of accepts PM-0001-30/late-command for `scenario`
  (`any` includes late_command) but run() never reads `late_command` (grep: only precheck.rs:297): a scenario run
  with that id writes header.bite = late-command and plants nothing. Fix: in run(), refuse (Failure::Usage, exit 2) an
  id whose only instance is late_command: "--bite PM-0001-30/late-command is the pre-check's (npt-lab precheck)".
- F-c Low (item 2): precheck.rs:455-462 and :486-501. The verdict reads each client's room counters once at the leg's
  end; a client dropped or kicked on a leg reads as 0 late and 0 rewinds_refused (unwrap_or_default), so a late
  command of a later-dropped client is invisible to the VOID rule; refused_logged (per tick) is printed but not in the
  verdict. Not reachable in a clean replay (no kick); fix: sum `late` per tick as logs.rs room_tick does (or record
  each client's counters when its session ends), and VOID on max(rewinds_refused, refused_logged).
- F-d Low (item 1, carry 77 (a), for U4g): logs.rs:270-283 dedupes by name over the room's and client's wraps; the
  names reported are room cmd_seq (room/src/input.rs:86), press (:104), ack (room.rs:765), client tick_offset
  (client/src/lib.rs:661), event_seq (:673): five. -58's check needs >= 6 counter_wrap records and names the server
  tick's low 16 bits among six (marks.json:4204-4210); nothing writes a tick wrap. -58 is U4g's (U4R-Q5), so this is a
  line for U4g's brief: the lab writes a `tick` counter_wrap when the room's tick crosses a multiple of 65,536, or the
  check's count is re-read. Confidence medium (which wire counter is "acknowledgement numbers" not settled here).
- F-e Low (item 1, carry 76 and carry 62 (c)): scenario.rs:410-418 pushes (slot, "replaced") into `ended`, and run()
  calls logs.gone(slot) for every ended entry (:857-859, :925-927); writable() then stops waiting for that slot
  (logs.rs:358-370) although the new session holds it, so a run with a replacement writes the new session's ticks as
  null and ends `crash` on its later filings. No U4r run replaces (one Join per client); U6's reconnects would. Fix:
  skip logs.gone for "replaced".
- F-f Low (item 4, carry 58): c-u4r-runs.mjs:179-186 records each run's summary line (which holds "overflow resyncs
  N") but flags nothing; carry 58 sends any overflow resync in a judged -17/-18 run to the advisor before it counts,
  and ruling 4 p1 item 4 expects about 48 on the tick16 bite run, all in the after phase. Also the pre-check pairs'
  expectation check reads only the first word (:164-165: VOID), not 7.2's "VOID by late_count alone". Fix: parse
  "overflow resyncs (\d+)" and append "(overflow resyncs: to the advisor, carry 58)" to the run's line when > 0; check
  the late-command pair's result line equals "result: VOID (L1 late_count 1)".
- F-g Low (item 1, carry 39 / brief 2.6): bots/src/lib.rs:49-52 RETARGET_EVERY = 30 ("about 1 s at 30 commands a
  second") and TURN_EVERY = 4 (the circle's size "at the stress scene's speed") are rate-derived counts with a local
  form; at another TICK_HZ the brawl's retarget time and circle change. Not judged, deterministic at any rate. Fix: name
  them as command counts with no time claim in their docs, or derive from npt_sim's functions; list the need in the
  report (carry 39's route).

## Could not check
- The report (item 5): no U4r report on disk (u4r-build.md ends at "Next ... then r5 and section 5's end checks"). The
  ruled statements (r4's words, the brawl's view, D13, B1's FAIL and cause, ES's FAIL with 7.250, -18's bite counts)
  were checked against the logs only (figures above, all matching the rulings); their wording in the report is the
  main session's read once it exists.
- r5, section 5's end checks and the record runs (c-u4r-r2.mjs and c-u4r-runs.mjs from a merge commit): not made yet.
- The 9,000-tick pre-check pair itself (only r6's 120-tick pair is shown).

## Verdict
APPROVE WITH FINDINGS (one Medium, six Lows; no Critical or High).
