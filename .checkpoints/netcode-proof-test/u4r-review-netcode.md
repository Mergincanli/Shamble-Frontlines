# U4r review, round 1: netcode-architect seat (batch 1)

Snapshot under review: wip/u4r.patch sha256 572c4f69435da2ad6f62d54d4bcbc47b04d6329b5493e0faf78fcecfdc4a09c2 (checked by sha256sum at start).
Packet u4r-packet.md sha256 cbd82485... ; tests file u4r-packet-tests.md sha256 d12555c8... (both checked).
Read copy: .gate/u4r-snap1 (2fbec77 + patch). Logs of my runs (if any): .gate/u4r-review-netcode/.

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
