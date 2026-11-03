# verifier-fixcheck-e3b verdicts (HEAD c74a2e5, base 4374a1c)

Scope: the three things e3 left unread: (1) SPIKE_PLAN item 5 to end, (2) cross-doc consistency over nine docs, (3) SPIKE_PLAN count statements.

## Task 1: SPIKE_PLAN from item 5 to the end (diff 4374a1c..HEAD; the nine docs are unchanged 59d41e0..HEAD)
Quote check by script (scratch/q2.py): every quote in these hunks is in the inputs file word for word (6 of 6).

| # | SPIKE_PLAN line | claim (short) | source | verdict | where | note |
|---|---|---|---|---|---|---|
| 1 | 274-275 | item 4 Place: second heavy spike, after the proof test; OQ-0001-2 (a) the Advisor's; the 60 Hz run follows | inputs SPIKE-ORDER, TICK-60-SETUP | confirmed | inputs:5, :7 | (item 4, re-read for context) |
| 2 | 312-314 | Mustafa makes the models with AI help in Blender, blockouts with skin, named slots, LOOK_V2 proportions | LOOK-TEST-INPUTS (Mustafa's) | confirmed (paraphrase) | inputs:11 | "as blockouts" states his minimum bar as the plan; see finding 2 |
| 3 | 314-316 | each object may be a blockout; the export part's rules reach him before he makes them: advisor's reading | ruling LOOK-TEST-INPUTS, SPIKE-ORDER bullet 3 | confirmed | verdict Rulings | marked |
| 4 | 316-318 | OQ-0006-18 before the first model; if before the design session, to Mustafa then: advisor's reading | ruling LATER | confirmed | verdict Rulings, LATER | marked |
| 5 | 324-326 | look test: last heavy spike, after the boundary spike, quote "then the look proof test once its models exist" | SPIKE-ORDER (Mustafa's) | confirmed | inputs:5 word for word | |
| 6 | 327-328 | Size: making the models is Mustafa's own work in Blender, "not a session's" | LOOK-TEST-INPUTS | unsupported (unmarked reading) | inputs:11 says "I make the test models with AI help" | finding 1 |
| 7 | 348-349 | preview check alongside any heavy spike, quote | SPIKE-ORDER (Mustafa's) | confirmed | inputs:5 word for word | |
| 8 | 383-386 | client entry for every sim id of a presented kind; OQ-0005-1 the Advisor's; presented kinds per CD-2: advisor's reading | OQ-0005-1, ruling OQ-0005-1 | confirmed | inputs:15; verdict ruling q10 | |
| 9 | 402-405 | AB13 quote, the Advisor's; reading "new reviewed generic operation counts as built from data" marked | OQ-0005-AB13, ruling q11 | confirmed | inputs:16 word for word | |
| 10 | 414-416 | PM-0003-44 gated at 6 of 600 on RD at High; the Advisor's, confirmed by Mustafa; reported on ML marked as reading | OQ-0003-17, ruling q9 | confirmed | inputs:16; ADR 0003:1207 agrees | |
| 11 | 427-429 | PM-0003-18 to -21 gate on ML's soaked set; OQ-0003-20 the Advisor's, quote | OQ-0003-20, ruling q9 | confirmed | inputs:16 word for word; ADR 0003:961, :1216 | |
| 12 | 431 | heading "a research unit, not a spike" | ruling SPIKE-ORDER bullet 3 | confirmed | "is agent research, not a heavy spike" | |
| 13 | 439-441 | LOOK-TEST-INPUTS quote, Mustafa's | inputs | confirmed | inputs:11 word for word | |
| 14 | 441-445 | Blender route, LOOK_V2 not edited; runs alongside none; between two wave-1 spikes before the models; recommended right after the proof test; marked "both are the advisor's reading" | rulings LOOK-TEST-INPUTS, SPIKE-ORDER bullet 3 | confirmed | verdict Rulings | ruling applied |
| 15 | 475-479 | Feel Lab: the rewind cap's weighing gone; OQ-0001-5's rule Mustafa's; Brave check of the measured cap the revisit trigger: marked | ruling REWIND-CAP bullet 5 | confirmed | verdict Rulings | ruling applied |
| 16 | 490-492 | slice 6's 60 Hz run makes the final call whatever the early signal (TICK-60-SETUP, the Advisor's); PM-0001-39, -40, -63, PM-0004-71, -72 decide there: marked | TICK-60-SETUP, ruling q3 | confirmed | inputs:7; ADR 0001:1516-1518; ADR 0004:1194-1195 exist | |
| 17 | 495-497 | still open: OQ-0003-1 (ML, RD, MAC to name), OQ-0006-1/-2, OQ-0006-20 in part, OQ-0006-15 in part, OQ-0006-19 | ruling SPIKE-MACHINES (d), partly answered list | confirmed | verdict Rulings q7 | |
| 18 | 497-500 | the stop answered J6 (5), OQ-0001-1, -2, -3, -16, OQ-0006-17, ASSET-RESEARCH timing, OQ-SPIKE-1 | inputs; rulings q6, q7 (b) | confirmed | | |
| 19 | 517-520 | OQ-SPIKE-1 answered by Mustafa, quote | LOOK-TEST-INPUTS | confirmed | inputs:11 word for word | |
| 20 | 520 | "Making the models is his own work, not a session's, so no spike's size carries it." | LOOK-TEST-INPUTS | unsupported (unmarked reading) | as row 6 | finding 1 |
| 21 | 520-524 | LOOK_V2 not edited; Part 11's list stays; blockout at his bar; checklist steps for the look test's plan: marked | ruling LOOK-TEST-INPUTS | confirmed | verdict Rulings q8 | |

Rulings named in the task, all applied: the export part between two wave-1 spikes before the models (rows 3, 14); the
Feel Lab weighing of the cap gone (row 15); OQ-0003-17's reading (row 10) and OQ-0003-20's (row 11); OQ-SPIKE-1 (rows
19-21). Attribution: every answer in these hunks carries the marks of condition 2.
END TASK 1
## Task 2: consistency across the nine documents (six ADRs, README, SPIKE_PLAN, ENGINE_OVERVIEW at c74a2e5)
Method: grep of each topic's terms over the nine files, then each hit read in context.

| topic | places read | verdict | disagreement (both texts) |
|---|---|---|---|
| P2's values, half each way, 2 % each direction | 0001:567-577, :1097-1101, :1204-1206, :1395-1406, PM-0001-3, -37; 0002:514, :1184, PM-0002-49; 0003:1410; 0004:1100; 0005:2245, :2414; 0006:1200-1204; README:45, :68; SPIKE_PLAN:135-136 | consistent | none (0001:574 and :1262 quote the research's own older figures, marked as restated) |
| 60 Hz rule (margin + research marks + goal budget at rooms per VM at 60 Hz, PM-0001-63; wave-1 early signal; slice 6 decides) | 0001:13-15, :28-33, :124-146, :728, :1062-1079, :1296, :1389, :1648-1653, :1723, :1746; 0002:49-50, :1346-1348; 0004:47-52, :831-832, :1092-1098, :1253; 0005:2242-2243, :2371; 0006:53-54, :340-347, :914, :1046-1047, :1122-1124, :1194-1195, :1300; README:16, :26, :43-44, N2 (:370); SPIKE_PLAN:15-16, :91, :171-212, :490-492; ENGINE_OVERVIEW:16-18, :129 | consistent | none. 0002:49, 0004:47, 0006:53 state the margin as a necessary condition and link GLOSSARY §8 (allowed by the ruling); 0001:13 points to part 2. 0005:2243 "measured in one run" (unchanged since base) means one run per measurement, as GLOSSARY §8 says |
| rewind cap rule (p99 at P2, rounded up to a whole tick; no value yet) | 0001:368-373, :1003-1009, :1036, :1111-1120, :1314, :1326-1330, PM-0001-49, checklist step 7 (:1619), R2 rows :1782, :1806, :1843; README:60; SPIKE_PLAN:163-168, :475-479 | consistent | none (0001:1006-1009 gives Mustafa's rule in his words, without the p99 reading; it doesn't contradict it) |
| wave 1's order (proof test, sim core, 60 Hz run, boundary, look test once its models exist; preview alongside any) | 0001:581-584, :1248, :1648-1653; 0002:716-717, :1084-1087, :1393-1396; 0003:976-980, :1366-1369; 0004:49, :1246; 0006:1124, :1348-1349; README:28, :47, :54; SPIKE_PLAN:48-95, :145, :205, :238, :274, :323, :348, :439-445 | DISAGREES (1) | ADR 0004:1246, migration phase 1 ("Spikes", before phase 2 "The foundation"): "in the order Mustafa accepted (SPIKE-ORDER (b), ...): the netcode proof test first; then the sim core spike; then the 60 Hz margin run, ...; then the boundary spike; then the content and ability spike". SPIKE_PLAN:397: "Place: wave 3, after the sim core spike and the foundation, before slice 1", and :64-68 (mermaid W1 --> F --> W3). Finding 3 |
| | SPIKE_PLAN:504-505 (OQ-SPIKE-1, Why it matters) | stale question text | "the look proof test runs in wave 1, before the asset-pipeline research", while SPIKE_PLAN:439-445 now runs the research's export part before the look test. Finding 5 |
| header wording (condition 4) | the six ADR headers (:3, :7 or :8), README:16-21 Status column and :26-31, SPIKE_PLAN:3-5, ENGINE_OVERVIEW:3-4 | consistent | none: "Proposed" and "Signed off by Mustafa: as the basis for the spikes (2026-11-03, SPIKE-ORDER (b)); not yet accepted" in all nine |
| reference machines (OQ-0003-1; MP stands in for none) | 0001:1363-1368; 0002:1162-1166; 0003:813-822, :1366; README:18, :53; SPIKE_PLAN:30-38, :129, :234, :269, :307, :495 | substance consistent; label differs | ADR 0003:821: "Answered in part by Mustafa (2026-11-03): "Test PC: my own, in Brave"", and 0003:1366: "(OQ-0003-1, answered in part: still open, Mustafa's)". README:53: "Still open after the stop: ..." and SPIKE_PLAN:129: "(OQ-0003-1, still open)", SPIKE_PLAN:495: "ML, RD and MAC still to be named", with no "answered in part". MP stands in for none in all of them. Finding 4 |
| ROOM-CRASH "marked crashed" | 0001:487, :747, :1223-1226, PM-0001-59; 0006:479-481, :1408; README:70 | consistent | none |
| OQ-0005-1's presented kinds | 0005:319-329 (CD-2), :723, :1562-1567, :1580, :1938-1944, :2089, :2119-2120, PM-0005-4 (:2273); README:120; SPIKE_PLAN:383-386 | DISAGREES (1) | ADR 0005 CD-2 (:321-328) lists the presented kinds as unit, vehicle, weapon and faction, and says: "building parts and props are entities in it, not kinds of their own (AP-7)". ADR 0005 PM-0005-4 (:2273): "the presented kinds as CD-2 names them (from AP-2, CD-13 and AB-3: units, vehicles, weapons, factions, building parts and props)". Finding 2 |
END TASK 2
## Task 3: SPIKE_PLAN's own count statements against the script counts
Scripts: e3's count_oq2.py (re-run at c74a2e5), my scratch/pmcover.py (every PM id the table and items name, ranges
expanded), and a row count of each draft's "| PM-000N-k |" rows.

| statement in SPIKE_PLAN | its count | script | verdict |
|---|---|---|---|
| table :88-101 and items: pass marks placed per draft | ids 1-63, 1-78, 1-47, 1-75, 1-46, 1-32 all placed, none beyond | rows 63, 78, 47, 75, 46, 32 = 341; highest ids the same | confirmed: no id missing, none invented |
| item 2 and table row 2: the 60 Hz group PM-0001-39, -40, -63; PM-0004-71, -72 | 5 ids | the rows exist (0001:1516-1518; 0004:1194-1195) | confirmed |
| numbered spikes (### 1 to 10; table rows 1 to 10, plus two "–" rows that aren't spikes) | 10 | 10 headings "### N." | confirmed |
| "Open questions": OQ-SPIKE-1, its only one (README: "SPIKE_PLAN adds one of its own") | 1 | 1 "- **OQ-SPIKE" entry | confirmed |
| "What this plan does not decide" (:495-500): still open OQ-0003-1, OQ-0006-1, -2, -15 (in part), -19, -20 (in part); answered OQ-0001-1, -2, -3, -16, OQ-0006-17 | as listed | count_oq2: open 98 (12, 7, 17, 10, 33, 19) of 114; OQ-0001-1, -2, -3, -16 and OQ-0006-17 among the answered; OQ-0003-1, OQ-0006-15, -20 marked "answered in part" and counted open | confirmed |
| foundation "L (six change-sets)" (:96) | 6 | ADR 0002 migration phase 2 has steps 1 to 6 (:1397-1430) | confirmed |
| sim core spike "about 60 marks" (:278) | about 60 | PM-0004-1 to -61, -73, -74 = 63 | confirmed (approximate, stated as such) |
SPIKE_PLAN states no total of open questions or pass marks of its own (the 114, 98 and 341 totals are README's, checked
by e3 and matching the scripts).
END TASK 3

Correction to Task 1 row 2's note: "as blockouts" is judged faithful (he makes them, and blockouts are enough); no
finding is raised for it.

## Findings (ranked)
1. **medium, inconsistent: OQ-0005-1's presented kinds** (author: test-engineer, PM-0005-4).
   - Current text, ADR 0005:2273 (PM-0005-4): "the presented kinds as CD-2 names them (from AP-2, CD-13 and AB-3: units,
     vehicles, weapons, factions, building parts and props)".
   - CD-2 (ADR 0005:321-328), which the row says it follows: the presented kinds are a unit, a vehicle, a weapon and a
     faction, and "building parts and props are entities in it, not kinds of their own (AP-7)". The ruling (q10) has
     engine-architect name the kinds in CD-2, so CD-2 is the reference.
   - Smallest fix: "(from AP-2, CD-13 and AB-3: units, vehicles, weapons and factions)".
2. **medium, inconsistent: wave order in ADR 0004's migration plan** (author: rust-reviewer, FIX S14-1).
   - Current text, ADR 0004:1246, phase 1 "Spikes", which comes before phase 2 "The foundation": "in the order Mustafa
     accepted (SPIKE-ORDER (b), ...): the netcode proof test first; then the sim core spike; then the 60 Hz margin run,
     ...; then the boundary spike; then the content and ability spike".
   - SPIKE_PLAN:397 places the content and ability spike in "wave 3, after the sim core spike and the foundation, before
     slice 1" (also its mermaid :64-68 and ADR 0005:2169). The phase-1 list predates this round, but the round's edit
     now calls it Mustafa's accepted order.
   - Smallest fix: replace "; then the content and ability spike" with "; after the foundation (wave 3), the content and
     ability spike", or move that spike to a row after phase 2.
3. **low, unmarked reading: "not a session's"** (author: engine-architect, stop-engine-architect-b; from map row L1).
   - Current text, SPIKE_PLAN:327-328: "which is Mustafa's own work in Blender (OQ-SPIKE-1, answered), not a session's",
     and SPIKE_PLAN:520: "Making the models is his own work, not a session's, so no spike's size carries it."
   - His words (inputs:11): "I make the test models with AI help in Blender". "AI help" could be a session's help, and
     no verdict ruling says "not a session's". Neither place is marked as a reading.
   - Smallest fix: drop ", not a session's" in both places, or mark it "(the advisor's reading, 2026-11-03)" if the
     advisor confirms it.
4. **low, inconsistent label: OQ-0003-1** (author: engine-architect, README:53 and SPIKE_PLAN; graphics-engineer, ADR
   0003).
   - ADR 0003:821: "Answered in part by Mustafa (2026-11-03): "Test PC: my own, in Brave"", and ADR 0003:1366:
     "(OQ-0003-1, answered in part: still open, Mustafa's)". The count script reads it as answered in part.
   - README:53: "Still open after the stop: ..."; SPIKE_PLAN:129: "(OQ-0003-1, still open)"; SPIKE_PLAN:495: "(OQ-0003-1:
     ML, RD and MAC still to be named)".
   - All agree in substance: open, and MP stands in for none. The ruling (SPIKE-MACHINES (d)) says "unchanged in
     substance", and COUNTS says a partly answered question is marked "answered in part".
   - Smallest fix: README:53 "Answered in part, still open after the stop: ..."; SPIKE_PLAN:129 "(OQ-0003-1, answered
     in part, still open)". Or drop "answered in part" from ADR 0003, if the advisor reads "unchanged in substance" as
     not answered at all.
5. **low, stale question text: OQ-SPIKE-1** (author: engine-architect).
   - Current text, SPIKE_PLAN:504-505: "*Why it matters:* the look proof test runs in wave 1, before the asset-pipeline
     research". Since the answer, the research's export part runs before the look test (SPIKE_PLAN:439-445). The
     answer is appended at the end of the entry (:517).
   - Smallest fix: "*Why it matters (as asked):*", or "was to run in wave 1, before the asset-pipeline research".
END FINDINGS

## Not read
- Nothing of the three items left. Out of scope, not re-checked: quotes and attribution inside the six ADRs (units e1,
  e2), README's quotes (e3), the map's dispositions (e3).

## Summary
PASS WITH NOTES. 36 claims judged, 31 confirmed. Task 1: 21 rows, 19 confirmed, 2 unmarked readings (finding 3); every
quote word for word (6 of 6); attribution matches condition 2; the named rulings are applied. Task 2: 8 topics, 5
consistent throughout; 2 disagreements (findings 1, 2); 1 label difference (finding 4); plus 1 stale question text
(finding 5). Task 3: 7 count statements, all confirmed (341 of 341 pass marks placed; 10 spikes; 1 OQ-SPIKE; the open
and answered lists match 114 numbered, 98 open).
DONE
