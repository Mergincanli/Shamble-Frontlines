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

## Claims, ADR 0002 (HEAD line numbers)

| # | doc:line | claim (short) | cited | verdict | where and the words | note |
|---|---|---|---|---|---|---|
| 47 | 0002:3-7 | header per condition 4 | SPIKE-ORDER (b) | confirmed | byte for byte condition 4's wording | O15 |
| 48 | 0002:49-50 | TICK-30 restatement "60 Hz only with the 60 Hz margin ... (ADR 0001 owns it)" unchanged | TICK-60-HEADROOM | confirmed (no change) | ruling q2: restatements linking §8 state a necessary condition and may stay | H23 |
| 49 | 0002:233-235 | part 3: rule 3 allows the three crates (OQ-0002-8 (a), the Advisor) | OQ-0002-8 | confirmed | inputs: "OQ-0002-8 (Advisor): (a)." | E2 |
| 50 | 0002:512-518 | part 8 simulator: latency and jitter half each way, loss as the profile defines it (the Advisor's answer, OQ-0001-16) | NET-PROFILES | confirmed | paraphrase of the words, no over-reading in this line (see finding F1 on PM-0002-49) | N8 |
| 51 | 0002:544-549 | part 8 bench: who creates/deletes answered, quote; covering bench and 3-hour soaks is the advisor's reading | SPIKE-MACHINES | confirmed | quote exact; ruling (b) | M22 |
| 52 | 0002:715-717 | part 12: boundary spike in wave 1 after proof test, sim core spike and 60 Hz run, one heavy spike at a time | SPIKE-ORDER | confirmed | inputs: "the netcode proof test first, then the sim core spike (then the 60 Hz run), then the boundary spike" | O18 |
| 53 | 0002:752 | Options table chosen row: "proposed, and allowed by the Advisor's ruling at the stop (OQ-0002-8 (a))" | OQ-0002-8 | confirmed | map E5 said "no change needed"; the change only replaces the pending wording; option rows kept | E5 |
| 54 | 0002:904-906 | trade-off: rule 3 allows proposing them | OQ-0002-8 | confirmed | | E3 |
| 55 | 0002:1074-1076 | OQ-0002-8: "Answered by the Advisor (2026-11-03): "(a)."", option (a)'s text restated; options kept | OQ-0002-8 | confirmed | quote exact; (a)'s text at 0002:1069-1070 matches | E1 |
| 56 | 0002:1084-1089 | spike text: wave 1 after the three; shares harness and stand-in sim; sim core spike's sim serves "the marks that need the real sim" (reading) | SPIKE-ORDER | confirmed in substance | ruling SPIKE-ORDER bullet 2 ("now uses the sim core spike's sim"); wording, see finding F2 | O18 |
| 57 | 0002:1133-1135 | spike item 7: P2 with the sim core spike's sim, which runs before this spike | SPIKE-ORDER | confirmed | | |
| 58 | 0002:1177-1180 | key BX: Worker padded to 13.33 ms; sim core spike's sim also runs in this Worker, BX rows again | SPIKE-ORDER | confirmed | ruling: "stand-in until then" clause may go | |
| 59 | 0002:1250 | PM-0002-28: "with the stand-in sim: reported only" goes; sim core spike's sim runs before (7 cells) | SPIKE-ORDER | confirmed | ruling SPIKE-ORDER bullet 2 | |
| 60 | 0002:1272 | PM-0002-49: in each direction; per-direction values "as part 8 splits them (as PM-0001-3 reads them)"; source adds OQ-0001-16 answered (7 cells) | NET-PROFILES | confirmed with a note | part 8 (0002:512-518) splits latency and jitter only; loss per direction is the advisor's reading, marked only in ADR 0001 (finding F1) | N9 |
| 61 | 0002:1346-1349 | note: no mark at 60 Hz; wave-1 run after sim core spike an early signal; slice 6 decides; quote | TICK-60-SETUP | confirmed | quote exact | |
| 62 | 0002:1392-1396 | migration 0: signed off as basis; step 1: one heavy spike at a time, order | SPIKE-ORDER | confirmed | | O18 |
| 63 | 0002:1524, 1527 | Appendix A R4-XT-OQ5, R5-LAYOUT-REC2: allowed by OQ-0002-8 (a), the Advisor | OQ-0002-8 | confirmed | | E4 |
END CLAIMS 0002

## Claims, ADR 0004 (HEAD line numbers)

| # | doc:line | claim (short) | cited | verdict | where and the words | note |
|---|---|---|---|---|---|---|
| 64 | 0004:3-7 | header per condition 4 | SPIKE-ORDER (b) | confirmed | condition 4's wording | O15 |
| 65 | 0004:47-53 | TICK-30: ADR 0001 measures twice (wave-1 early, slice 6 final); TICK-60-SETUP quote; full rule part 2 with Mustafa's addition quoted | TICK-60-* | confirmed | both quotes exact; ruling q2 lists 0004:45-47 among the lines that change | S10, H23 |
| 66 | 0004:182, 612 (HEAD) | "60 Hz only with the 60 Hz margin" restatements unchanged | TICK-60-HEADROOM | confirmed (no change) | ruling q2: necessary condition, may stay | H23 |
| 67 | 0004:570-574 | part 20: OQ-0001-18 answered by the Advisor, quote; a room never resumes | ROOM-CRASH | confirmed | quote exact | RC6 |
| 68 | 0004:830-832 | Revisit when: tick moves to 60 Hz at slice 6's call (margin and goal budget, PM-0001-63) | TICK-60-* | confirmed | | |
| 69 | 0004:981-985 | spike intro: wave 1's run, OQ-0001-2 (a) with S1's stand-ins; "an early signal only"; final call at slice 6 | TICK-60-SETUP | confirmed | "an early signal only" is a substring of the words | S11 |
| 70 | 0004:991 | S1 row: 60 Hz load in wave 1 = this sim with these stand-ins, early signal; slice 6 on the real crate; numbers unchanged | TICK-60-SETUP | confirmed | | S12 |
| 71 | 0004:1093-1097 | key Tick: 60 Hz set read from each of ADR 0001's two 60 Hz runs; wave 1 decides nothing; slice 6 decides (reading); TICK-60-HEADROOM confirmed | TICK-60-* | confirmed | "as an early signal only" exact; ruling q3 ("PM-0004-71 and -72 run with each 60 Hz run but decide only at slice 6") | S13, H17 |
| 72 | 0004:1101 | key Network: OQ-0001-3 answered by the Advisor | NET-PROFILES | confirmed | | N10 |
| 73 | 0004:1124 | PM-0004-8: presented-frame count on ML at Low is PM-0003-44, reported there, gating on RD at High; OQ-0003-17 the Advisor's, confirmed by Mustafa | OQ-0003-17 | confirmed | words: "(recommended desktop, High)"; ML at Low reported is PM-0003-44's text since 4374a1c; attribution per condition 2 | |
| 74 | 0004:1194-1195 | PM-0004-71/72: with each of ADR 0001's 60 Hz runs, deciding only at slice 6; wave 1 sim = sim core spike's sim + S1 stand-ins; slice 6 real crate (7 cells) | TICK-60-SETUP | confirmed | ruling q3 | S13 |
| 75 | 0004:1246 | migration 1: order quoted ("wave 1 runs one heavy spike at a time"); proof test, sim core spike, 60 Hz run (early signal), boundary, content; "or its measured step times" gone | SPIKE-ORDER, TICK-60-SETUP | confirmed | quote exact; order matches the words | S14, O20 |
| 76 | 0004:1253 | migration 8: 60 Hz run on the real sim makes the final call, quote; runs whatever (reading); PM-0004-71/72 decide there | TICK-60-SETUP | confirmed | quote exact | S14 |
| 77 | 0004 OQ entries (LA3: :902-:951 at base) | the LATER questions' own entries not edited | LATER | confirmed (no change) | ruling LATER: "The drafts' own entries for these questions are not edited (LA3)"; diff has no hunk there | LA3 |
END CLAIMS 0004

## Rules (task 5)
- glossary_check_drafts.py (repo root): entries 37; exact hits 0 (0001: 0, 0002: 0, 0004: 0).
- Script verifier-fixcheck-e1-scratch/rules.py on every changed line (250 in 0001, 49 in 0002, 37 in 0004): 0 prose
  lines over 120 characters; 0 PM rows with other than 7 cells; 0 sentences directly under a table.
- PM rows: 0001 62 -> 63 (PM-0001-63 added, condition 6), 0002 78, 0004 75 (README:23 says 63, 78, 75: matches).
- Open questions (script on each "## Open questions"): 0001 18, answered -1, -2, -3, -14, -16, -18, in part -5
  (open 12); 0002 8, answered -8 (open 7); 0004 12, none answered (open 12).
- Options kept: every answered OQ keeps its Options list (OQ-0001-1, -2, -3, -5, -14, -16, -18; OQ-0002-8 read).
END RULES

## Findings (ranked; all low; HEAD 5bd2849, my three files unchanged since 59d41e0)

F1 (low, unmarked reading, test-engineer: stop-test-engineer-0002-fixes.md) 0002:1272 PM-0002-49
- Current: "within 5 % (relative) of the per-direction values of P0–P4 and B1, as part 8 splits them (as PM-0001-3
  reads them; ...)".
- The words / ruling: NET-PROFILES (the Advisor) splits latency and jitter half each way; part 8 (0002:514-515) says
  "loss is as the profile defines it". The 2 % per direction is the advisor's reading, marked so only in ADR 0001
  part 13; PM-0002-49 attributes the per-direction loss to part 8's split.
- Smallest fix: "... as part 8 splits latency and jitter and ADR 0001 part 13 reads the loss (the advisor's reading,
  2026-11-03) ...".
END F1

F2 (low, over-read in wording, engine-architect-a: stop-engine-architect-a-fixes.md) 0002:1086-1087
- Current: "the sim core spike's sim, which exists by then, serves the marks that need the real sim (the advisor's
  reading, 2026-11-03)".
- The ruling: SPIKE-ORDER bullet 2: "The boundary spike now uses the sim core spike's sim". "The real sim" elsewhere
  (verdict :77, :160) is slice 6's real crate; the spike's sim is a throwaway spike sim.
- Smallest fix: "serves the marks that need a sim beyond the stand-in".
END F2

F3 (low, inconsistent, rust-reviewer: stop-rust-reviewer-fixes.md; not on the map's list) 0004:1245 migration phase 0
- Current: "| 0 | Mustafa and the Advisor approve the drafts | the sign-off line of this record |" (unchanged).
- The words: STATUS-FIELD (b), condition 4: signed off "as the basis for the spikes (2026-11-03, SPIKE-ORDER (b))";
  0002:1391 and 0001:1649 record step 0 as done; 0004 still reads as a future approval.
- Smallest fix: "| 0 | The drafts signed off by Mustafa as the basis for the spikes (2026-11-03, SPIKE-ORDER (b)) | the
  sign-off line of this record |".
END F3

## Result
PASS WITH NOTES: 77 claims judged, 77 confirmed (2 with a note: #56, #60); rebuild identical; rules clean; 3 low findings.
END RESULT
