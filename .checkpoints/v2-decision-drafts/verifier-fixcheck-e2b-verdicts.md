# verifier-fixcheck-e2b verdicts (ADR 0005 changed lines)
HEAD c74a2e5 · base 4374a1c · e2 checked C1, C2 (OQ-0005-1 and AB13 quotes)

## Claims
| # | doc:line | claim (short) | cited | verdict | where and the words | note |
|---|---|---|---|---|---|---|
| H1 | 0005:3-8 | header: Proposed; "Signed off by Mustafa: as the basis for the spikes (2026-11-03, SPIKE-ORDER (b)); not yet accepted" | condition 4 | confirmed | verdict condition 4's wording, exact | |
| L1 | 0005:83-89 (Settled inputs) | export/slot part (AS3, AS4) before the look test; LOOK-TEST-INPUTS quote; LOOK_V2 §8 unchanged, Blender the route for test models and the part's export (marked) | LOOK-TEST-INPUTS (Mustafa); ruling q8 | confirmed | quote word for word (inputs line 10); ruling q8 "LOOK_V2 is untouched. Blender is recorded as the route for the look test's models and for the research part's export"; AS3/AS4 named by plan r4 line 30 | |
| O1 | 0005:102-103 | "no section answered either"; the Advisor answered OQ-0005-1 at the stop | OQ-0005-1 (Advisor) | confirmed | | |
| K1 | 0005:286-287 (CD-2 stages) | whole-set checks add "every id of a presented kind has its client entry" | ruling q10; condition 6 | confirmed | a clause in the existing whole-set stage, not a new row | |
| K2 | 0005:319-330 (CD-2) | quote; binds presented kinds (marked); presented: unit, vehicle, weapon, faction; none: AB-4's six, modes, rule tables, movement data, moment rules; maps (CD-11); building parts and props are map entities (AP-7) | OQ-0005-1; ruling q10 | confirmed | quote word for word (inputs line 14); lists cover every kind of CD-6 (sim content) and AB-4; game-designer NOTE C9-2 confirms the ability kinds "as C4-2 lists them" | e2 C1 checked the quote |
| K3 | 0005:513-516 (CD-13) | faction colours and model ids in a client definition keyed by its id (OQ-0005-1, answered by the Advisor); a faction without it fails CD-2 | OQ-0005-1 | confirmed | within the words ("keyed by the same id"; "fails if a simulation id has no client entry") | |
| K4 | 0005:723-724 (content item 2) | proposed: per presented kind CD-2 names, a set with a missing client entry refused 100 % | OQ-0005-1 | confirmed | marked proposed with its reason | |
| A1 | 0005:822-824 (findings) | AB12: the write-up "is accepted for concepts", and "any specific claim ... once it can be read" | OQ-0005-AB12 (Advisor) | confirmed | both fragments word for word (inputs line 15) | |
| A2 | 0005:1192-1201 (strongest argument against) | Advisor accepted it "for concepts" (AB12); AB13 quote; new reviewed generic operation counts as data (marked) | AB12, AB13 (Advisor); ruling q11 | confirmed | quotes word for word | e2 C2 |
| A3 | 0005:1369-1373 (OQ-0005-AB12) | answered, quote; "Decided: (a), for concepts"; specific claims checked against Epic's documentation once readable | OQ-0005-AB12 (Advisor) | confirmed, note | quote word for word; options (a), (b) kept; no list of the specific claims a record relies on (AB-1:876 "GAS's default sums its multipliers" is one) | finding F4 |
| A4 | 0005:1380-1387 (OQ-0005-AB13) | answered, quote; reading (marked): new generic op counts as data, as AB-1/AB-6 have it; "needed often" no longer a trigger; (b) unscheduled | AB13 (Advisor); ruling q11 | confirmed | quote word for word; "every new operation is a reviewed Rust change" is at 0005:875 (AB-1); options (a)(b) kept as weighed | |
| P1 | 0005:1497-1500 (asset pipeline, Settled inputs) | export/slot part before the look test; LOOK-TEST-INPUTS: that timing, Blender route, "which Mustafa makes with AI help as blockouts with a skin ..." | LOOK-TEST-INPUTS (Mustafa) | over-read (low) | words: "blockouts with a skin, named colour slots and LOOK_V2's proportions are enough"; ruling q8 "each object may be a blockout": a bar, not a statement that the models are blockouts | finding F5 |
| P2 | 0005:1560-1570 (AP-2) | asset id of a definition in its client definition (OQ-0005-1, answered by the Advisor); map holds its own; CD-2 refuses an id of a presented kind with no client entry | OQ-0005-1; ruling q10 | confirmed | "OQ-0005-1's rule" points to CD-2's marked reading | |
| P3 | 0005:1579-1582 (AP-3) | faction colours from the faction's client definition (CD-13; OQ-0005-1 answered) | OQ-0005-1 | confirmed | | |
| P4 | 0005:1691-1694 (OQ-0005-AP1) | quote "Route: Blender (...)"; test models in Blender, their source files included; Blender also the research's export route (marked); AP1 keeps its place before the asset spike | LOOK-TEST-INPUTS; ruling q8 | confirmed | quote word for word | |
| G1 | 0005:1736-1742 (GPU rules intro) | export/slot part (AS3, AS4) before the look test; quote; Blender the export route and LOOK_V2 §8 as written (marked) | LOOK-TEST-INPUTS; ruling q8 | confirmed | quote word for word; LOOK_V2.md unchanged since 4374a1c (git diff --stat empty) | |
| G2 | 0005:1810-1813 (GPU part 8) | Mustafa makes them with AI help in Blender; quote; list stays, each may be a blockout (marked) | LOOK-TEST-INPUTS; ruling q8 | confirmed | quote word for word; ruling q8 "Part 11's list of objects stays; each object may be a blockout at Mustafa's bar" | |
| G3 | 0005:1857-1860 (GPU open questions intro) | AS3, AS4 before the look test | LOOK-TEST-INPUTS | confirmed | plan r4 line 30 names AS3, AS4 | |
| G4 | 0005:1875-1877 (OQ-0005-AS3) | timing quotes; on Blender's export (marked) | LOOK-TEST-INPUTS; ruling q8 | confirmed | "the asset research's export and slot-naming part" and "runs before the look proof test" word for word | |
| G5 | 0005:1882-1886 (OQ-0005-AS4) | what decides it: Blender's export for the test models (quote) and Spline's while LOOK_V2 §8 names it (marked) | LOOK-TEST-INPUTS; ruling LOOK_V2-§8 | confirmed | quote word for word; options kept ("a manual convention ... or LOOK_V2 §8's automated step") | |
| F1 | 0005:1938 (Question F2 table, row b) | "(chosen: OQ-0005-1, answered by the Advisor, 2026-11-03)" | OQ-0005-1 | confirmed | row 7 cells; row a kept | |
| F2 | 0005:1941-1944 (Question F2) | Decided (b) (Advisor's answer); "the answer adds one whole-set check, an id of a presented kind with no client entry fails" | OQ-0005-1; ruling q10 | unmarked reading (low) | words: "fails if a simulation id has no client entry"; narrowing to presented kinds is ruling q10, marked at 0005:321 and 2120, unmarked here | finding F3 |
| T1 | 0005:1958-1959 (Why one model) | unit's model named in the unit's client definition (OQ-0005-1) | OQ-0005-1 | confirmed | | |
| T2 | 0005:1997-2001 (Revisit when) | "a designed ability can't be built from data (OQ-0005-AB13, answered by the Advisor)"; "ability marks 1 or 2 fail" gone | AB13; ruling q11 | confirmed | words exact in substance | |
| T3 | 0005:2052-2053 (OQ intro) | an answered question names who and when | condition 2 | confirmed | the three answered rows do | |
| T4 | 0005:2076 (table, AB12) | answered by the Advisor, quote | AB12 | confirmed | quote word for word; 4 cells | |
| T5 | 0005:2077 (table, AB13) | quote; new reviewed op counts as data; talk weighed, unscheduled (marked) | AB13; ruling q11 | confirmed | no "needed often" left in the frame's table | |
| T6 | 0005:2089 (table, OQ-0005-1) | "answered by the Advisor, 2026-11-03: client definitions keyed by the same id; an id of a presented kind with no client entry fails the content check, CD-2" | OQ-0005-1; ruling q10 | unmarked reading (low) | words: "the content check fails if a simulation id has no client entry"; "presented kind" is ruling q10, here given as the Advisor's answer | finding F3 |
| T7 | 0005:2108-2121 (frame OQ-0005-1) | fix round history; quote; Decided (b); one whole-set check binding presented kinds, none for others (marked); CD-2 names both lists | OQ-0005-1; ruling q10 | confirmed | quote word for word; options (a), (b) kept | |
| M1 | 0005:2248 (PM key, P2) | P2 is VISION_V2 §4's "about 100 ms of ping" (OQ-0001-3, answered by the Advisor) | NET-PROFILES (Advisor); ruling q6 | confirmed | inputs: "(VISION_V2 §4's "about 100 ms of ping")"; ruling "OQ-0001-3 and -16 are answered" | |
| M2 | 0005:2273 (PM-0005-4) | bite on an existing CD-2 row; quote; reading marked; presented kinds "as CD-2 names them (... units, vehicles, weapons, factions, building parts and props)"; client-entry corpus class; controls "mode and tuning table" | OQ-0005-1; condition 6 | inconsistent (medium) | quote word for word, 7 cells, no new row (46 PM rows base and head); but CD-2 (0005:327-328) says building parts and props "are entities in it, not kinds of their own (AP-7)", and game-designer's NOTE C9-2 confirms units, vehicles, weapons, factions only | finding F1 |
| M3 | 0005:2326-2332 (When a mark fails) | new reviewed op for a mechanic failing PM-0005-18 to -21; A3 back only when ...; quote; reading marked; "option A4 ... if many fail" gone | AB13; ruling q11; NOTE B4-2 | confirmed | quote word for word | |
| M4 | 0005:2457-2460 (Migration, Assets) | look test uses exports of the models Mustafa makes in Blender, after the export/slot part; the rest of the research, then the asset spike | LOOK-TEST-INPUTS | confirmed | | |
| M5 | 0005:2486-2489 (Rollback) | failing mechanic becomes a new reviewed op, still built from data (marked); A3 only when ... (quote fragment, marked) | AB13; ruling q11 | confirmed | "many fail" and A4 fallback gone | |
| X1 | 0005:2537 (App A, R1-ABL-OQ1) | answered: AB12 (Advisor); its words | AB12 | confirmed | word for word (unquoted, attributed) | |
| X2 | 0005:2538 (App A, R1-ABL-OQ2) | answered: AB13; reading the talk weighed, unscheduled "(the advisor's reading)" | AB13; condition 2 | rule (low) | mark lacks ", 2026-11-03" (condition 2's form) | finding F6 |
| X3 | 0005:2540 (App A, R1-ABL-OQ9) | AB13's answer sets when A3 returns | AB13 | confirmed | | |
| X4 | 0005:2504 (App A counts, unchanged) | "answered 8 · open 9" | count | wrong (medium) | recount of the 34 rows at HEAD: adopted 5, changed 6, answered 10, open 7, spike 6 (R1-ABL-OQ1, OQ2 moved open → answered) | finding F2 |
| X5 | 0005:384-386 (CD-6 client content, unchanged) and 2284 (PM-0005-15) | client content lists feel kit, quality tiers, asset list, slot classes, look values | OQ-0005-1 consistency | note (low) | the client entries now decided (CD-2, CD-13, AP-2: model ids, faction colours) are client content but not in CD-6's list or PM-0005-15's changed values | finding F7 |

## Rules and consistency checks
- Glossary: `glossary_check_drafts.py` exact hits 0 (0005: exact 0).
- New prose lines (135 added lines, scratch/rules.py): none over 120 characters.
- Tables: every row has its header's cell count (0 mismatches); PM rows 7 cells; no text line directly under a table.
- PM rows 46 at base and HEAD (OQ-0005-1 is a bite on PM-0005-4, no new row: condition 6). OQ table rows 36 and 36.
- Options kept: AB12 (a)(b), AB13 (a)(b), Question F2 rows a and b, OQ-0005-1 (a)(b), AS4's routes.
- AB13 triggers: "needed often" survives only in AB13's option (a) text (0005:1378, a weighed option) and in its
  Decided line saying it is no longer a trigger (0005:1384); "many fail" and "marks 1 or 2 fail" are gone from the
  Rollback, "Revisit when", the frame's table, "When a mark fails" and Appendix A.
- LOOK_V2.md: unchanged since 4374a1c. SPIKE-ORDER: 0005 says nothing that runs a heavy spike alongside another; its
  spike list (0005:2169-2185) and Migration step 1 (netcode proof test first) agree with the answer.
- Header: condition 4's wording exact.
END CHECKS

## Findings (ranked)
- F1 (medium, inconsistent, test-engineer, Pass marks) 0005:2273, PM-0005-4: current text "the presented kinds as
  CD-2 names them (from AP-2, CD-13 and AB-3: units, vehicles, weapons, factions, building parts and props)". CD-2
  (0005:321-328, engine-architect, with game-designer's NOTE C9-2) names units, vehicles, weapons and factions, and says
  "building parts and props are entities in it, not kinds of their own (AP-7)"; CD-6 has no building or prop kind. So
  the row's corpus "for every presented kind" asks for two kinds that do not exist. Same row: the controls say "a
  tuning table" where CD-2 and CD-6 say "rule tables". Smallest fix: "...: units, vehicles, weapons and factions)" and
  "(a mode, a rule table)" / "sets whose mode and rule table have no entry". (Also found by unit e3b, Task 2.)
- F2 (medium, count, engine-architect, Appendix A) 0005:2504: current text "Counts: adopted 5 · changed 6 · answered
  8 · open 9 · spike 6 · rejected 0 (34 rows)." After FIX blocks moved R1-ABL-OQ1 and R1-ABL-OQ2 from open to answered
  (0005:2537-2538), a script recount of the 34 rows gives adopted 5 · changed 6 · answered 10 · open 7 · spike 6.
  Smallest fix: "answered 10 · open 7".
- F3 (low, unmarked reading, engine-architect, frame) 0005:2089 (OQ table row) and 0005:1942-1944 (Question F2):
  current text "(answered by the Advisor, 2026-11-03: client definitions keyed by the same id; an id of a presented kind
  with no client entry fails the content check, CD-2)" and "the answer adds one whole-set check, an id of a presented
  kind with no client entry fails (CD-2 names the kinds)". The words: "the content check fails if a simulation id has
  no client entry"; the narrowing to presented kinds is ruling q10, marked at 0005:321 and 0005:2120 but given here as
  the answer. Smallest fix: add "(the advisor's reading, 2026-11-03)" after "fails the content check" and after "with no
  client entry fails".
- F4 (low, unsupported, game-designer, ability section) 0005:1371-1373, OQ-0005-AB12 Decided: "The specific claims a
  record relies on are checked against Epic's documentation once it can be read". The answer is recorded word for word,
  but no list of those specific claims exists, so the later check has nothing named to check; AB-1 relies on at least
  one ("GAS's default sums its multipliers", 0005:876). Smallest fix: one clause naming them, e.g. "The specific claims
  this record relies on: GAS's default aggregation sums multipliers (AB-1)". (The verdict r4 has no AB12 ruling; the
  task's "specific claims listed" is the expectation checked here.)
- F5 (low, over-read, engine-architect, asset pipeline) 0005:1498-1500: current text "Blender as the route for the
  look proof test's models, which Mustafa makes with AI help as blockouts with a skin, named colour slots and LOOK_V2's
  proportions." The words: "I make the test models with AI help in Blender; blockouts with a skin, named colour slots
  and LOOK_V2's proportions are enough" (a bar; ruling q8 "each object may be a blockout"). Smallest fix: "which
  Mustafa makes with AI help, where blockouts with a skin, named colour slots and LOOK_V2's proportions are enough."
- F6 (low, rule, engine-architect, Appendix A) 0005:2538: current text "(the advisor's reading)". Condition 2's mark is
  "(the advisor's reading, 2026-11-03)". Smallest fix: add ", 2026-11-03".
- F7 (low, inconsistent, engine-architect, content data; test-engineer, PM-0005-15) 0005:384-386 (CD-6 client content)
  and 0005:2284 (PM-0005-15): CD-6's client content lists "the feel kit per weapon and the quality-tier table (CD-14),
  the asset list and the material-slot classes ..., and the look values", and PM-0005-15 changes "the feel kit, the
  asset list, the material-slot classes, the look values"; neither names the client entries OQ-0005-1 now decides (a
  unit's, vehicle's or weapon's model id and a faction's colours and uniform models, CD-2, CD-13, AP-2). Unchanged
  lines, so not a regression of this round. Smallest fix: add "the client entries keyed by simulation ids (CD-2)" to
  CD-6's client content and to PM-0005-15's list of changed values.
END FINDINGS

## Result
FAIL: 39 claims judged (rows H1-X5), 32 confirmed (A3 with a note: finding F4), 7 not confirmed (P1, F2, T6, M2, X2,
X4, X5: findings F1-F3, F5-F7).
Two medium findings (F1 pass-mark kinds contradict CD-2; F2 stale Appendix A count) need a one-phrase fix each before
the PR is marked ready; five low.
END RESULT
