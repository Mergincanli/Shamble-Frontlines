# verifier-fixcheck-e1 verdicts (ADR 0001, 0002, 0004; HEAD 59d41e0, base 4374a1c)

## Claims

| # | doc:line | claim (short) | cited | verdict | where and the words | note |
|---|---|---|---|---|---|---|
END CLAIMS-HEADER (rows follow below in blocks)

## Rebuild (task 4)
- From 4374a1c, scratch copies in verifier-fixcheck-e1-scratch/docs/adr/, apply_fixes.py in this order:
  0001: stop-test-engineer-fixes.md (23 blocks for 0001, all ok), stop-netcode-architect-fixes.md (42, all ok).
  0004: stop-test-engineer-fixes.md (4, ok), stop-rust-reviewer-fixes.md (8, ok).
  0002: stop-test-engineer-fixes.md (0 for 0002), stop-engine-architect-a-fixes.md (13, ok), stop-test-engineer-0002-fixes.md (4, ok).
- cmp against HEAD 59d41e0: all three IDENTICAL, byte for byte.
END REBUILD

## Claims, ADR 0001 (HEAD line numbers)

| # | doc:line | claim (short) | cited | verdict | where and the words | note |
|---|---|---|---|---|---|---|
| 1 | 0001:3-7 | header: Proposed; signed off as the basis for the spikes; not yet accepted | SPIKE-ORDER (b) | confirmed | verdict condition 4 wording, byte for byte | O15 |
| 2 | 0001:13-16 | summary: 60 Hz only if margin passes and goal budget holds, judged at slice 6; proof-test run an early signal (part 2) | TICK-60-HEADROOM, TICK-60-SETUP | confirmed | inputs: "60 Hz also needs the goal budget to hold..."; "(a), as an early signal only. The final 60 Hz call is measured again..."; ruling: summary states the full rule or points to part 2 | H3, S3 |
| 3 | 0001:29-33 | Requirements: quote of the addition; confirmed by Mustafa; TICK-60-SETUP the Advisor's | TICK-60-HEADROOM | confirmed | quote word for word; attribution per condition 2 | H4 |
| 4 | 0001:83-85 | Takes: the goal budget of ADR 0006 part 8, which the 60 Hz rule reads | OQ-0001-1 | confirmed | follows the ruling (part 8's goal budget) | |
| 5 | 0001:126-146 | part 2: third condition quoted; goal budget = part 8's at the call, slice 6 = OQ-0006-19's figure; "holds" = fleet at k60 (formula, same machine, N=48) <= figure, marked reading; slice 6 decides whatever the signal; wave-1 run after sim core spike, kept server/harness, S1 stand-ins; pass starts nothing, fail closes nothing; R1-TICK-OQ1 only after slice 6 passes | TICK-60-HEADROOM, -SETUP, SPIKE-ORDER | confirmed | both quotes word for word; rulings TICK-60-HEADROOM (q2), TICK-60-SETUP (q3), SPIKE-ORDER (q4); every reading marked "(the advisor's reading, 2026-11-03)" | H2, S2 |
| 6 | 0001:249-255 | part 5: buffer aim quoted; smallest passing buffer, run-on-arrival as none (reading) | REWIND-CAP | confirmed | "Aim for the smallest input buffer that holds" (quoted substring exact); ruling REWIND-CAP bullet 6 | RW4 |
| 7 | 0001:369-375 | part 8: cap rule quoted; p99 at P2, whole tick at run's rate (reading); value recorded when test reports | REWIND-CAP | confirmed | quote exact; ruling bullets 1-3 | RW2 |
| 8 | 0001:385-390 | part 8 arithmetic kept as the reason; cap follows the measured honest rewind | REWIND-CAP | confirmed | ruling: candidates go | RW3 |
| 9 | 0001:457-465 | part 10: WebTransport pinned cert quoted; WebSocket leg: trusted cert on his domain, OQ-0006-5 case (i), method for the proof test's plan with security-auditor (reading) | SPIKE-MACHINES | confirmed | "a private hash-pinned certificate for WebTransport"; "for the WebSocket leg, a domain I register" exact; ruling (e) | M14 |
| 10 | 0001:489-493 | part 11: match never resumes; quote; records kept under ADR 0006 part 11's retention (reading) | ROOM-CRASH (the Advisor) | confirmed | quote exact; ruling ROOM-CRASH | RC2 |
| 11 | 0001:568-579 | part 13: half each way; loss per direction 2 %, ~4 % round trips (reading); OQ-0001-3 answered "yes" | NET-PROFILES (the Advisor) | confirmed | quotes exact; ruling NET-PROFILES; 1-0.98^2=3.96 % | N3 |
| 12 | 0001:581-584 | part 14: signed off as basis; first in wave 1, quote | SPIKE-ORDER | confirmed | "wave 1 runs one heavy spike at a time: the netcode proof test first" exact | O16 |
| 13 | 0001:727 | budgets row: early signal / slice 6 decides; rooms per VM at 60 Hz + goal budget (PM-0001-63); basis confirmed by Mustafa | TICK-60-* | confirmed | rulings q2, q3 | H5, S4 |
| 14 | 0001:747 | failure row: OQ-0001-18 answered (a); names PM-0001-59 | ROOM-CRASH | confirmed | PM-0001-59 exists | RC3 |
| 15 | 0001:1002-1007 | trade-off 1: rule settles the value; window weighed at his Brave check (reading) | REWIND-CAP | confirmed | ruling bullet 5 (Brave check of the measured cap the feel trigger) | RW6 |
| 16 | 0001:1035-1039 | Revisit when: cap value by rule; Brave check of the cap; slice 6 run for the tick rate | REWIND-CAP, TICK-60-SETUP | confirmed | rulings | RW7 |
| 17 | 0001:1060-1065 | OQ-0001-1 answered by Mustafa, quote; (a) with addition; PM-0001-63; options kept | TICK-60-HEADROOM | confirmed | quote exact | H1 |
| 18 | 0001:1071-1078 | OQ-0001-2 answered by the Advisor, quote; slice 6 always; PM-0004-71/72 decide only at slice 6; R1-TICK-OQ1 after; (c),(d) kept | TICK-60-SETUP | confirmed | quote exact; ruling q3 | S1 |
| 19 | 0001:1097-1101 | OQ-0001-3 answered by the Advisor, quote; last sentence answers OQ-0001-16 | NET-PROFILES | confirmed | words exact; the inner double quotes of the inputs are written as single quotes inside the outer quote (punctuation only) | N1 |
| 20 | 0001:1111-1131 | OQ-0001-5 "Answered in part", quote; rule decided in (c)'s shape (reading); still open: the value, by the proof test's measured honest rewind under the rule; What decides it: no candidates left | REWIND-CAP | confirmed | quote exact; ruling REWIND-CAP; COUNTS (stays open) | RW1 |
| 21 | 0001:1141-1145 | OQ-0001-6 What decides it: aim quoted; smallest passing buffer; options stay (reading) | REWIND-CAP | confirmed | ruling bullet 6 | RW5 |
| 22 | 0001:1181-1185 | OQ-0001-14 answered by the Advisor, quote; (a); need lines stand | OQ-0001-14 | confirmed | "yes, a VISION_V2 §3 or §4 trace counts, marked as such." exact; ruling: no need line changes | X1 |
| 23 | 0001:1202-1206 | OQ-0001-16 answered by the Advisor, quote; (a); loss per direction (reading) | NET-PROFILES | confirmed | quote exact | N2 |
| 24 | 0001:1222-1227 | OQ-0001-18 answered by the Advisor, quote; (a); records kept, no new period, "marked crashed" (reading); options stay | ROOM-CRASH | confirmed | quote exact; ruling ROOM-CRASH | RC1 |
| 25 | 0001:1247-1250 | spike intro: signed off as basis; first of wave 1's heavy spikes, one at a time | SPIKE-ORDER | confirmed | | O16 |
| 26 | 0001:1271-1276 | spike item 1: certs quoted; WebSocket cert on his domain, case (i), method in the plan (reading) | SPIKE-MACHINES | confirmed | quotes exact; ruling (e) | M15 |
| 27 | 0001:1294-1301 | spike item 3: rooms per VM at 60 Hz + goal budget (PM-0001-63); wave 1 early signal, own VM session after sim core spike; proof test closes at 30 Hz without it (reading); slice 6 decides | TICK-60-*, SPIKE-ORDER | confirmed | ruling SPIKE-ORDER bullet 1 | H6, S5 |
| 28 | 0001:1313-1315, 1327-1331 | items 4, 5: cap set by the rule; p99 at P2 rounded to whole ticks, largest reported (reading) | REWIND-CAP | confirmed | | RW9 |
| 29 | 0001:1339 | key group list adds PM-0001-63 | | confirmed | | H8 |
| 30 | 0001:1352-1360 | key machines: PM-0001-63 unpinned; LG CX33s quoted; MP = test PC quoted, stands in for none of ML/RD/MAC (reading) | SPIKE-MACHINES | confirmed | "the CX33 runs the simulated clients", "Test PC: my own, in Brave" exact; ruling (c), (d) | M16 |
| 31 | 0001:1387-1392 | key "The sim in the run": S1 stand-ins in wave 1, real sim at slice 6; OQ-0001-2 quoted | TICK-60-SETUP | confirmed | quote exact | S6 |
| 32 | 0001:1396-1403 | key Profiles: loss per direction drawn on its own, ~4 % (3.96 %); answered quote; OQ-0001-3 quote | NET-PROFILES | confirmed | quotes exact; "proposed" on the half split dropped (now decided) | N4, N5 |
| 33 | 0001:1449-1453 | validity: PM-0001-27 and -63 under PM-0006-13 rules | | confirmed | | |
| 34 | 0001:1474-1476 | PM-0001-1, -2, -3: PM-0001-63 exempt/added; PM-0001-3 adds "its loss", answered | NET-PROFILES | confirmed | | N6 |
| 35 | 0001:1503 | PM-0001-59: answered (a); quote; kept under part 11's retention (reading); conditional tail dropped | ROOM-CRASH | confirmed | quote exact; ruling: "PM-0001-59's tail about option (b) goes" | RC4 |
| 36 | 0001:1515-1518 | 60 Hz group renamed; PM-0001-39 (sim per key; confirmed); PM-0001-40 needs -63; new PM-0001-63 (7 cells) | TICK-60-HEADROOM | confirmed | condition 6; arithmetic checked: ceil(20/8)+1=4 -> 361.44 EUR; 6 -> 534.42; k60=1 -> 21 x 86.49 + 8.99 + 6.49 = 1,831.77; 72,000 ticks, p99 the 71,280th; figures as ADR 0006:328-332 | H7 |
| 37 | 0001:1531-1532 | PM-0001-48: test cap stays (reading in "proposed"); PM-0001-49: rule quoted, value recorded, largest beside it | REWIND-CAP | confirmed | ruling: "PM-0001-48's test cap ... stays"; quote exact | RW10, RW11 |
| 38 | 0001:1537 | PM-0001-53 at P2, OQ-0001-3 answered | NET-PROFILES | confirmed | | |
| 39 | 0001:1541-1556 | When a mark fails: 60 Hz group outside the pass; early signal; slice 6 decides; quote; readings marked | TICK-60-SETUP | confirmed | quote exact | H9, S7 |
| 40 | 0001:1582-1588 | no-pass-number list: OQ-0001-6 smallest buffer (quote, reading); density at 60 Hz is PM-0001-63 | REWIND-CAP | confirmed | quote exact | |
| 41 | 0001:1618-1642 | checklist: step 7 uses the rule rounded up; 200/250 step goes; cover step kept with step 7's cap; renumbered 10-14; cross refs (steps 4 and 11) updated | REWIND-CAP | confirmed | ruling: step 10 goes, step 7 uses rule, step 11 stays; grep: no other in-file or cross-doc reference to steps 10-15 (0003:1320 cites step 7 only) | RW12 |
| 42 | 0001:1647-1653 | migration step 1: first in wave 1, one at a time; Done when without the 60 Hz group, rewind cap named; 60 Hz early signal later, separately | SPIKE-ORDER, TICK-60-SETUP | confirmed | ruling SPIKE-ORDER bullet 1 | H10, S8, O17 |
| 43 | 0001:1673-1676 | slice 4: Feel Lab list drops -5; cap feel = Brave check (reading) | REWIND-CAP | confirmed | ruling: Feel Lab weighing of the cap goes | |
| 44 | 0001:1681-1686 | slice 6: final 60 Hz run always, with rooms per VM and goal budget; Done when tick by part 2 | TICK-60-SETUP | confirmed | | S8 |
| 45 | 0001:1721-1724 | rollback: margin fails or goal budget does not hold; early fail closes nothing (reading) | TICK-60-* | confirmed | | H11 |
| 46 | 0001:1746-1750, 1782, 1798, 1806, 1816-1817, 1843, 1857 | Appendix A rows: R1-TICK-REC2, R1-TICK-OQ1, R2-BUD-REC4, R2-ROOM-REC2, R2-SYNC-OQ2 (in part), -OQ14, -OQ15 (answered), R2-XU-OQ1 (in part), R2-XU-OQ18 (answered) | various | confirmed | rulings; attributions per condition 2 | S9, RC5, RW13, X2 |
END CLAIMS 0001
