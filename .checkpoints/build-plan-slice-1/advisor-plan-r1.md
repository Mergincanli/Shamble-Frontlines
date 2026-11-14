Task: plan-review-r1 (advisor plan review of build plan for foundation steps 2.4, 2.5 and Slice 1; plan.md revision 1, branch claude/wonderful-hamilton-jdmlnf, base main 4460ef5)

## Plan (blocks, in order)
B1. Read Mustafa's two message files (task + addition) -> list his items 1-13 and "How to work".
B2. Read plan.md in parts (Parts 0-12, Decisions needed, For Mustafa) -> structure notes.
B3. Read DECISIONS.md models lines 229-251 + any other source the plan quotes.
B4. Check 1: items 1-13 answered (where, right answer).
B5. Check 2: done-when lines vs ADR 0002 1471-1491, 0001 1778-1786, 0003 1390-1399, 0004 1252-1253, 0005 2439-2445.
B6. Check 3: pass marks spot-check >=20 rows vs drafts' tables; run count-marks.mjs; SPIKE_PLAN 147-160; 3.2.
B7. Check 4: measure-first list 3.1 and change-set order.
B8. Check 5: rules (GATE-FOLLOW, Part 7, kit/gate edits, spikes/, one PR at a time, 3-agent cap, models/effort, VISION_V2 s6 traceability).
B9. Check 6: look (Parts 4, 5, 11, 12; Look/Body/Feel/first-model change-sets).
B10. Check 7: every quotation of Mustafa vs source.
B11. Check 8: Decisions needed + For Mustafa.
B12. Check 9: estimates and risks.
B13. Check 10: plain words.
B14. Verdict + answer draft. Then DONE.

## Notes
### B1 DONE (his two messages)
Items: 1 cut into CSs (merge on own; first Brave ASAP per drafts; every later CS ends openable/playable/judgeable; per CS: one plain line seen/felt, planned size, time estimate, needs-go?). 2 every done-when + pass mark of 2.4/2.5/Slice 1 -> CS. 3 every question + PM of the six named tests (sim core spike, 60 Hz run, boundary spike, look proof test, content and ability spike, render budget bench) -> where on real game + which CS, or why separate; measure-first few, first (his 3: native=Wasm from first tick; hand-off cost at 12 players; frame time weakest machine with look on). 4 Art project list (soldier, test scene, colours, effects; stand-in in code). 5 queued words land (live camera tuning panel from first playable build); look decisions of 13/14 Nov not in repo -> plan vs LOOK_V2 as is; which CSs wait for the carry. 6 what waits for netcode proof -> Slice 3. 7 checks while gate v2 not built (format, lint, type, tests; shown in report) + which 2.3 CSs needed first. 8 how to open each build in Brave on Windows, step by step; when preview hosting check + 2.6 run (link per build). 9 decisions with options + rec. 10 what he does. 11 model file rules (format/export, scale, axes, origin, colour-area names, tri/material caps, skin for Slice 1?), who answers + when, early, pasteable short list (OQ-0005-AS3/AS4). 12 CS where first Blender model replaces stand-in; PM-0005-38..44 at that CS vs later, none dropped/loosened. 13 decision OQ-0006-18 (committed model public?), options + rec; no model file committed before his answer.
How to work: protocol; branch = cs/ stand-in; ONE PR at a time; <=3 agents; saver; checkpoint each step; advisor reviews plan, no rounds/verifier for plan; full review per CS when built; report only what a record shows w/ time; planned size per CS, stop at 2x with 3 lines; no kit/gate/CI edit w/o asking in chat with exact text; repo wins where his words don't cover -> stop + 3 lines. Decision recorded by 2.3's first docs CS; open no docs PR; no build before record on main + his go.
Models: chat + builder agents Fable 5.2; every other agent Opus 5.5; review/verify/design game+engine code agents at effort max; advisor tool off.
Blender addition: models made in Blender by Claude via Blender MCP on his PC from Art brief, GLB; first model replaces stand-in when ready; LOOK-TEST-INPUTS "Route: Blender" stands unchanged (NOT in the changed list any more).
Next: B2 read plan.md.
### B2 DONE (plan.md read whole, 1046 lines)
Structure: In one screen 8-43; Part 0 45-77; Part 1 cut table 81-96 (CS 0..12 + M), order 100-111, per-CS 113-257; Part 2 done-when 259-322 (2.4 267-277; 2.5 279-292; Slice 1 294-322); Part 3: 3.1 measure-first 326-339; 3.2 separate tests 341-358; 3.3 boundary 360-395; 3.4 sim core 397-445; 3.5 60 Hz 447-451; 3.6 look proof 453-471; 3.7 render bench 473-505; 3.8 content/ability 507-542; 3.9 OQs 544-564; count 566-568. Part 4 Art list 570-621. Part 5 camera+look 623-657. Part 6 netcode 659-687. Part 7 checks 689-740. Part 8 Brave 742-770. NO PART 9 (numbering skips: Decisions needed stands for item 9). Part 10 him 772-799. Part 11 model rules 801-902. Part 12 M + PM-0005-38..44 904-938. Part 13 risks 940-960. Decisions 965-1036 (11). For Mustafa 1038-1044.
Sums check: lines 600..1200 = 28,200 (CS0's 150 docs excluded); hours 188-199 consistent.
Early findings to verify:
- F? CS 0 (model rules) first, one PR at a time: its round trip may need Blender from his PC -> could block CS 1 and the first Brave build; headless Blender install in container vs CLAUDE.md Never "run unreviewed code fetched from the internet" (plan 811-814).
- F? Part 9 missing in numbering (Low).
- To verify: routine-guard.mjs:84 claim (plan 39-40); DECISIONS quotes; LOOK_V2 78 lines.
Next: B3 DECISIONS.md models + quoted sources.
NOTE (coordinator, mid-run): plan.md changed in 3 places after launch: Part 11 (paste list, source table, still-open list rewritten from finished asset-rules.md; skin paragraph: stand-in = one skinned mesh per colour area on one shared skeleton, named in asset list as a builder); Part 4 block A item 5 (at most 4 colour areas); Part 12 (+ paragraph on OQ-0005-AP3). Re-read those three from disk before judging Parts 4, 11, 12 (block B2's reading of them is stale).
Branch check: HEAD differs from origin/main 4460ef5 only in .checkpoints/ (verified).
Re-read (current disk, 1069 lines): Part 4 at 570-622 (A5 now: "drafts allow at most 4 colour areas ... say which four"); Part 11 at 802-922 (24 rules; rule 7 a bpy export call; rule 12 class prefixes unit_/faction_/emissive_/structure_/environment_; rule 16 4 materials = 4 colour areas; rule 21 joint list proposal; rule 24 no model file before MODEL-PUBLIC; still-open: "four colour areas are few" -> look question his + Advisor's); Part 12 at 924-961 (+ OQ-0005-AP3 para 944-945); Part 13 963-983; Decisions 988-1059; For Mustafa 1061-1067.
- F-low? CS 6 line 197-198 says "one skinned mesh"; Part 11 828 says "one skinned mesh per colour area on one shared skeleton" (inconsistent wording).
- To verify: Part 4 A5 "drafts allow at most 4 colour areas" vs ADR 0005 GPU rule 3 (materials per pass); OQ-0005-AP3 text (is a public repo commit/public preview "shipping"?).
Quotes checked so far (DECISIONS/SPIKE_PLAN): "for rough checks only" exact (DECISIONS:874); "may run alongside any of them" exact (:824); "blockouts with a skin, named colour slots and LOOK_V2's proportions are enough" exact (:836); "Spline or Blender" paraphrase OK (his words at SPIKE_PLAN:540-541, appendix:209); camera Queued quote exact (DECISIONS:74-75); "before the first authored model enters the game" SPIKE_PLAN:560 exact; "verified like R3" SPIKE_PLAN:535 exact (plan omits "in its own daytime session").
- F-med? Decision MACHINES (plan ~1007-1014) omits his standing words MACHINES-TIMING (DECISIONS:892-896): "The laptop and desktop stay on my timing: rough checks first, then a short re-run once they're named." and REF-MACHINES "names the actual machines before the look proof test" (:874). Options/rec should start from them (before-look-test maps to before CS 6 = option b).
- F-low: LOOK_V2 s8 "Spline or Blender" edit: SPIKE_PLAN:541-543 places it in the export part's change-set (= CS 0, advisor's reading); plan 822-823 hands it to the docs chat "unless you say otherwise" -> must be tracked (Part 10 / CS 0 report) so it can't fall through.
- F-low: SPIKE_PLAN:535-536 the research "appends its marks to the asset spike before that spike starts"; CS 0 should write the export part's own marks (AS3 checks) and place them at M.
B5 partial (step 2.4 rows vs ADR 0002 table :1279-1283, :1309, :1329):
- F-med: PM-0002-19 "Met at 1" (plan :274): mark's Where is CI ("cargo test ... run 20 times on one commit"; -63 adds "PM-0002-19's 20 runs once" in CI); 20 runs in the container are "shown" by the plan's own definition (Part 0 :51-53). Met belongs to 2.3 (CI loop) or a test that does the 20 runs inside cargo test in CI.
- F-med: PM-0002-16 "Met at 2" (plan :271): method "builds sim-wasm with one field added ... runs the page's startup check", Where CI; Wasm tests in CI come only 2.3 after 2 (plan's own GATE-FOLLOW :738-740) -> met 2.3 after 2 unless CS 2 shows a CI path.
- F-low: PM-0002-15 met at 1: its two planted defects + control (1 of 1 each) should be named as run once at CS 1 (plan :118-123 names only the export test).
- OK: -17 (2.3), -18 (2.3 after 2), -63 (2.3 after 2), -43 (2.3 after 1); "three generators": 3rd (wasm-bindgen .d.ts) at CS 2, stated.
B6 partial: count-marks.mjs run: expected 201 {0002:36,0006:4,0004:74,0003:47,0005:37,0001:3}, placed 201, missing none, extra none, none twice; asset rows 7. Expected set checked by me against SPIKE_PLAN:147-160 (boundary 0002-1..34,77,78 + 0004-8,28,61,68-70 + 0006-28..31; sim core 0004-1..61,73,74; 60 Hz 0001-39,40,63 + 0004-71,72; look 0003-1..17 + 0005-33..37; content 0005-1..32 + 0004-62..66,75; bench 0003-18..47 incl. 43): matches. Count = presence only; correctness by spot-checks next.
Spot-checks (draft row vs plan row):
1. PM-0002-4 (ADR :1267) vs plan 3.3 :366: OK (5; padded Worker; judged on ML; full BX later). Plan omits Edge/Firefox by name but "judged on the minimum laptop" covers ML's browsers. OK.
2. PM-0002-8 (:1271) vs :370: OK (5; ML 60 Hz). Busy-wait load to 16 ms is a method detail for CS 5's plan.
3. PM-0002-10 (:1273) vs :372 + 3.2 :349: LOOSENED BY OMISSION: mark passes "when every cell holds a number" for ML Chrome/Edge/Firefox + MAC Safari, "every section", isolation-flag build. Plan: throwaway test at CS 5, never merged, no later clauses -> the ML/MAC cells and the event/projectile sections (8, 9) have no place. 
4. PM-0002-11 (:1274) vs :372 + 3.2 :350: same: the B2/B3 call is made "on ML" (Chrome, Edge, Firefox; Safari reported); plan gives no later clause and throws the test away. 
- F-med (3+4): 3.2/3.3 rows for PM-0002-10, -11 (and same pattern for -25, -33's comparison, -34) need their machine clauses as later places, and the throwaway tests kept (e.g. under .gate/ or a tag) or rebuilt when ML/MAC are named; -10 needs every section (again at 12 or after 9).
5. PM-0002-13 (:1276) vs :374: OK (3 reading, 5; judged on ML).
6. PM-0002-25 (:1289) vs 3.2 :348 / 3.1 :335 / 3.3 :381: MISPLACED IN PART: mark asks calls/tick + Worker time at N = 12 and 48, memory after 10 min, ML Chrome + CI counts; at CS 3 the sim has no soldiers (they enter at 4, walkers at 5). Only the trap clause (b) is honest at 3. Needs: the N clauses at 5 (keep the two-instance build), ML Chrome as later place.
7. PM-0002-26 (:1291) vs :383: OK (2/3 jitter+hidden, 4 smoothness, 48 at 5, online Slice 3, named machines).
8. PM-0002-28 = PM-0004-8 (:1293) vs :385: OK (5 reported 12-48, again with Low renderer at 6; pass N from R1-TICK-OQ7 design question; stress scene Slice 6). Low: R1-TICK-OQ7 listed as "a decision" (3.9 :564) with no who/when.
9. PM-0002-32 (:1297) vs :389: OK (player page as the minimal entry at 2; editor entry Slice 5; other browsers on named machines).
10. PM-0002-77 (:1278) vs :376: OK in place (7, local; online Slice 3); omits "CI: Chromium" and "ML: Chrome" later clauses (Low, pattern).
11. PM-0002-21 (:1285) vs :379: OK (conditional on isolation).
12. PM-0002-3 (:1266) vs :365: OK (3, 5; online Slice 3); "CI (counts)" clause not named (Low, pattern).
- F-HIGH?: Part 8 :750-756 + BRAVE-ROUTE (a): "In this chat I post a file ... Download it" - no evidence this cloud chat can hand a file to his laptop (Claude Code on the web: no known download route; PR attachments not in the API; committing zips = binaries in public history that can never be rewritten; release assets = new public practice). Whole Brave chain from CS 2 rests on it. Must be proven (one test file to him before CS 2) or the route named + asked.
- F-med: the zip's "40-line server" (serve.mjs, Part 8 :743-744, :754) runs on his laptop: no CS owns it, no reviewer named; needs 127.0.0.1 bind, no path escape, security-auditor at CS 2, and a place in the repo.
13. PM-0002-34 (:1299, checklist :1395-1412 steps 1-6) vs :391: OK (1->5, 2-4->4, 5->Slice 3, 6->2).
14. PM-0004-7 (ADR 0004 :1128) vs :403: OK (5 reading; judged stress scene on ML, Slice 6).
15. PM-0004-12 (:1133) vs :407: OK (7 alloc count; times Slice 6).
16. PM-0004-14 (:1135) vs :409: OK (not run under REC3 scope).
17. PM-0004-19 (:1140) vs :414: OK (3/4 on; FF+WebKit CI; Safari Mac; ML once; six bot matches Slice 2). Its "nightly in CI" not named (Low).
18. PM-0004-22 (:1143) vs :417: place OK (3, again at 4, 9); later clause omits Firefox/WebKit "each browser engine" in CI (Low, pattern).
19. PM-0004-25 (:1146) vs :420: LOOSENED: decision row "PM-0004-19's suite with simd128 on: 0 mismatches lets it be turned on" (suite = the six bot recordings, CI three engines, MAC). Plan: "4 (the suite with SIMD on decides whether it may be on)", "again at Slice 2" -> a decision on a partial suite (movement recordings, no Firefox/WebKit/Safari). Should be a reading at 4; simd128 stays off until the full suite (Slice 2 + Mac) passes. Medium.
20. PM-0004-33 (:1154) vs :427 + 3.2 :352: OK (own size + rebuild at 4; comparison builds separate at 4); "CI (sizes)" not named (Low).
21. PM-0004-34 (:1155) vs 3.2 :353 / :428: needs -30 parity, -31 cost (T, V on RS = reference server) and -33 size; plan "after 4, one day": the cost clause on RS cannot be judged then (-31 is Slice 6 in the plan :425). Row needs its RS/cost place. Low-Med.
22. PM-0004-30 (:1151) vs :424/:337: OK (4, LB in the yard, CI by cargo test).
23. PM-0004-31 (:1152) vs :425: OK (5 reading; Slice 6 RS/ML).
24. PM-0004-36 (:1157) vs :430: OK (8 rays; bot shots Slice 2).
25. PM-0004-45 (:1166) vs :435: OK (Slice 2 counts, times Slice 6).
26. PM-0003-9 (ADR 0003 :1183) vs :464: place OK (6 techniques, 11 effects, roof fade Slice 5); "halftone and line boil with the first big moment" names no slice (Low); ML Low/RD High in 3.1.
27. PM-0003-12 (:1186) vs :466: times at 6 OK; gated count is whole-frame on LS (world + bars + effects + projectiles + post) -> Slice 6 defensible; but a count reading at 6 (48 stand-ins, per-soldier draws, the bite) and at 12 (bars, effects, projectiles in) is not named; the ink pick at 6 rests on it (Low-Med; ties to measure-first).
28. PM-0003-13 (:1187) vs :467: OK (6 reading; Slice 6).
29. PM-0003-21 (:1196) vs :479: OK (6 post row; rest Slice 6).
30. PM-0003-17 (:1191) vs :471 + Part 5: OK (in pieces; "beside the legacy view" named only for 11; Low).
31. PM-0003-22 (:1197) vs :480: OK (2 baseline, 6 RT memory, M reading; full Slice 6).
32. PM-0003-42 (:1217) vs :500: place OK (4 smoothness); "battery drain on a heavier scene" names no place (Low).
33. PM-0003-46 (:1220) vs :504: SUBSTITUTION: mark's fallback = a WebGPU smoke run "BS for 60 s on ML (Chrome on WebGPU) with 0 console and device errors, counts within the committed baseline, overlay naming WebGPU", log attached to the report. Plan: "your Brave check on your PC until the minimum laptop is named" -> must say it is a reading on MP, the copy-report carries errors/counts/backend, and the ML clause joins when named (Low-Med).
34. PM-0003-1 (:1175) vs :456: OK (MACHINES before 6).
35. PM-0005-2 (ADR 0005 :2275) vs :511: OK.
36. PM-0005-10 (:2283) vs :519: OK (4 local, 8 the Shabzon = SMG per LEGACY_INVENTORY CD12, 11 feel kit; dev server Slice 3); ML/Edge/Firefox 1 s clause not named (Low, pattern).
37. PM-0005-12 (:2285) vs :521: 3.8 row omits the match-record clause (Part 2 :321 has it at Slice 4) (Low).
38. PM-0005-16 (:2289) vs :525: OK.
39. PM-0005-25 (:2299) vs :533: OK (units' duels Slice 2, 35 m clause at 9, abilities/shield/spawn protection Slice 4).
40. PM-0005-39 (:2315) vs Part 12 :954: OK (name hash + size at M; cache/repeat at 2.6).
41. PM-0005-42 (:2318) vs :957: OK.
42. PM-0005-38 (:2314) vs :953: OK as "later" (his item 12 allows later; none dropped).
43. PM-0005-4 (:2277) vs :513: OK.
44. PM-0001-39/-40/-63, PM-0004-71 (ADR 0001 :1632-1634, ADR 0004 :1199) vs 3.5 :447-451: OK (decide at Slice 6; early signal decided nothing per PM-0001-63's own text; -40's 12-tick replay from 7).
45. PM-0004-72 (:1200) vs 3.5: EARLIER POSSIBLE: its CI clauses (every legacy interval same shot count at 30 and 60 Hz within 1 shot; PM-0004-37 tunnelling at 60 Hz) need only a 60 Hz build of the sim (CI; no RS) -> showable at 8 (shots) and 9 (tunnelling); it also proves the CS 4 time unit holds at 60 Hz before timers rest on it. Plan puts all at Slice 6. Medium (his "first build that can show it").
46. PM-0004-9 (:1130) vs :404: place OK (8: 13 intervals, CI). Its duel clause (Warrior kills Conscript 0.86 s) is the same duel PM-0005-25 places at Slice 2 -> inconsistent (Low).
47. PM-0004-11 (:1132) vs :406: OK (4, again 8).
48. PM-0004-6 (:1127) vs :402: OK.
49. PM-0006-28..31 (ADR 0006 :1396-1399, condition :1271, :452) vs :395: OK (conditional on EU Sentry org; else first online playtest).
3.2 check: each listed test is a real comparison or a condition (OK). Missing from 3.2: PM-0005-16's stub-loader comparison build (plan :525 places it at 4 without naming the build) (Low).
B6 DONE (49 rows spot-checked across 0001-0006; findings above). Next: B5 rest (done-when 0001/0003/0004/0005 + 2.5/Slice 1 rows).
B5 cont. ADR 0001 :1778-1786 (foundation + Slice 1 done-when) vs plan Part 2 :282, :301-305:
- OK: replay same hash two runs at 4; PM-0001-41 L1 half at 7 (ML over L2 at Slice 3, as the mark's own Where); PM-0001-42 snapshot share on L1 at 7, per-frame half is the mark's "ML: Chrome over L2 with LG" -> Slice 3 faithful; PM-0001-43 readings, met when ML+RD named (honest).
- F-med: PM-0001-20 (ADR 0001 :1610) runs "under each of OQ-0001-6's two command-timing options"; plan :302 "each command-timing option that is built" + Part 6 one scheduler behind a seam -> if only one is built, the Slice 1 done-when mark is loosened silently. Say both are built at 7, or name the second option's place.
- ADR 0003 :1386-1399 vs plan :283-288, :312-317: OK (foundation: init + backend at 2, counts gate 2.3 after 3, link 2.6; Slice 1: skinned soldier met 10, toon/ink 6 + pick, feel kit/split feedback met 11, PM-0003-7 met 12, PM-0003-29 met 11, Brave met 12). Look-test milestone OQs (-2,-3,-5,-6,-8,-12,-15,-16,-18,-19) all placed in 3.9.
- ADR 0004 :1252-1253 vs plan :289, :306-310: foundation row OK (scan test at 3; lints 2.3). Slice 1 milestone "S4, S5, S6 and S9 hold on the real crate; determinism suite green on slice 1 recordings on every target": plan defers honestly per row (S5 times Slice 6; S6 bots Slice 2; S9 parts later; Safari/Windows when on hand).
- F-med: no summary of which Slice 1 done-when clauses stay open after CS 12 (ADR 0002 :1487-1491 makes ADR 0001/0004 milestones + boundary marks at N=12 the slice's done-when; many are "shown" only, or wait for ML/RD/Mac/2.3/Slice 2/3/6). CS 12 "Close" (:95, :252-255) should list them so "Slice 1 done" is not claimed early and Slice 2 does not rest on unmet marks unknowingly.
- ADR 0005 :2439-2445 vs plan :277, :318-322: foundation line: plan follows ADR 0002 part 3 (content enters at Slice 1) over ADR 0005's "foundation" place, and says so (OK); glTF-reader rule 2.3 G5 before M (OK). Slice 1 done-when "content items 1, 2, 6 and ability item 2 pass on the real crate in CI": rows OK, but "Met at 4" for content item 1 needs Wasm + three engines in CI (2.3 after 2/3) - not named (Low).
- PM-0002-70 (:1337) vs :311 OK; PM-0002-64 (:1330) vs :287 OK; PM-0002-71 (:1338) vs :299 OK.
- F-low: PM-0002-69 (:1335) needs "PM-0002-35 to PM-0002-68 pass" (all of 2.3's gate marks) -> plan :292 "step 2.6" should read "step 2.6 and the end of 2.3".
B5 DONE. Next: B4 summary (items), B7, B8 rules (check 2.3 chat's model for GATE-FOLLOW).
