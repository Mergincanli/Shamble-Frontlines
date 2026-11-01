# Fixes for ADR 0006 (infrastructure and operations), plan step 9: test-engineer's pass marks

Author: test-engineer, 2026-11-01. Against `docs/adr/0006-infrastructure-operations.md` at the branch head (915d3ef). Each BEFORE was copied from the file by script (whole lines) and is unique in it; the fixes were applied in order to a scratch copy with scripts/apply_fixes.py and none overlaps another. Only rows and text of the "### Pass marks" subsection are touched. No fix changes a decision.

Findings fixed: 5 (TE140) and 6 (verifier-adr0006-verdicts.md). The reasons in fix 6 are the ledger's (TE60, TE76, TE77, TE81, TE84, TE98, TE170), written into the record because the ledger is a checkpoint file and leaves the branch with it. Ledger rows that change (test-engineer-passmarks-0006-claims.md): TE140 (now labelled proposed, with its reason), TE60, TE76, TE77, TE81, TE84, TE98, TE170 (reasons now in the record).

### FIX 5 (TE140, PM-0006-17)
BEFORE:
| PM-0006-17 | Egress per 24v24 match-hour matches R2's estimate (part 8) | bytes sent from RS's network interface in one room-hour within ±20 % of R2's E3 estimate at the counts that ran (R2's formula, r2 lines 1764–1766, evaluated in the results with the run's P, V, shots and projectile spawns, the arithmetic shown); at R2's own counts (P = 48, V = 15 moving vehicles, 0.44 projectile spawns per player per second, every entity in view) that is 3.156 GB, so 2.52 to 3.79 GB. Beside it, the room's own bytes-sent metric agrees with the interface counter within 5 % (proposed: R6-COST-REC2's tolerance, a check that the counter measures the room). With interest management on: the figure reported, only if the proof test's sim has the interest filter | RS with LG; one room alone | 48 bot clients over IPv4 (R2's case), all in view; three 20-minute matches back to back, joins and full states included (proposed: a room-hour as a real room spends it); the interface's transmitted-byte counter read at the start and end | 1 room-hour per setting; P2 on LG. Bite: snapshots sent twice per tick (about 2 × the reference) must fail | R6-COST-REC1's mark (r6 lines 716–717); the egress table (r6 lines 438–446: 146.1 kbps × 48 = 7.01 Mbit/s; × 3,600 s ÷ 8 = 3.156 GB); R2's estimate (r2 lines 1759–1762, 1775); A-1 (r6 line 515) |
AFTER:
| PM-0006-17 | Egress per 24v24 match-hour matches R2's estimate (part 8) | bytes sent from RS's network interface in one room-hour within ±20 % of R2's E3 estimate at the counts that ran (R2's formula, r2 lines 1764–1766, evaluated in the results with the run's P, V, shots and projectile spawns, the arithmetic shown; proposed, in place of R6-COST-REC1's fixed 3.16 GB, which holds only at R2's counts: the proof test's load need not have them, since it stands in for the stress scene, whose 700 shots per second (ADR 0001's PM-0001-2) are almost ten times R2's 72, 1.5 per player per second × 48, r2 line 1759); at R2's own counts (P = 48, V = 15 moving vehicles, 0.44 projectile spawns per player per second, every entity in view) that is 3.156 GB, so 2.52 to 3.79 GB. Beside it, the room's own bytes-sent metric agrees with the interface counter within 5 % (proposed: R6-COST-REC2's tolerance, a check that the counter measures the room). With interest management on: the figure reported, only if the proof test's sim has the interest filter | RS with LG; one room alone | 48 bot clients over IPv4 (R2's case), all in view; three 20-minute matches back to back, joins and full states included (proposed: a room-hour as a real room spends it); the interface's transmitted-byte counter read at the start and end | 1 room-hour per setting; P2 on LG. Bite: snapshots sent twice per tick (about 2 × the reference) must fail | R6-COST-REC1's mark (r6 lines 716–717); the egress table (r6 lines 438–446: 146.1 kbps × 48 = 7.01 Mbit/s; × 3,600 s ÷ 8 = 3.156 GB); R2's estimate (r2 lines 1759–1762, 1775); A-1 (r6 line 515) |
END

### FIX 6 (How to read the table: counts and settings)
BEFORE:
- **Bites.** Each automated mark names a planted defect that must make it fail before its pass counts, because "a
  check without a failing case can't be shown to work" (R5-GATE-REC3, r5 lines 1727–1728); a control is a correct case
  that must pass. The planted defects and controls are proposed.
AFTER:
- **Bites.** Each automated mark names a planted defect that must make it fail before its pass counts, because "a
  check without a failing case can't be shown to work" (R5-GATE-REC3, r5 lines 1727–1728); a control is a correct case
  that must pass. The planted defects and controls are proposed.
- **Counts and settings.** Run counts and method settings that a row states without a citation are proposed by
  test-engineer, each for the reason given here: PM-0006-4's 3 repeats from fresh profiles (a fresh profile makes
  each first visit a real download, and 3, PM-0006-3's count of loads, keeps one lucky pass from deciding);
  PM-0006-6's four pull requests (the two ways one of ours ends, closed unmerged with its `cs/` branch kept and
  merged with the branch deleted, plus a control and a bite) and its hourly probes (they record when each URL
  stopped serving); PM-0006-7's 10 uploads (a rise of 10 stands far from 0 and from a stray build or two, and
  spends 2 % of the 500 builds a month); PM-0006-8's at least 2 open branches (with one, a wall set on that
  branch alone would pass too); PM-0006-12's 100 echoes (each lost echo is 1 % of the loss `ping` prints);
  PM-0006-22's quarter of the bots on WebSocket (both transports then show in the session metric while most bots
  stay on WebTransport, the primary) and its kill at minute 20 (the crash, the end and the new start then fall
  inside the 30-minute soak, with 10 minutes left to see the new room run).
END
