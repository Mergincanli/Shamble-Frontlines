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
