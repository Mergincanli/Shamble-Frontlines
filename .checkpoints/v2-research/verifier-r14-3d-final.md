# verifier-r14-3d-final: step 3d final check of R1 and R4, glossary nits, corrected UNREAD_SOURCES rows (task: prompts/verifier-r14-3d-final.txt)

Started 2026-10-31.

## Check 1 (fix items) — closed
- Item 1 r1:2049 (5b57e40 r1:2050-2051): "The other 25 % of the tick period, inside the server tick budget (GLOSSARY: 50 % of the tick period), is the server-only share". Names the tick period; matches GLOSSARY "sim step share and server-only share" (25 % of tick period, 4.2 ms at 60 Hz). Logged nit (value unchanged: intended 4.2 ms). Applied.
- Item 2: five bare 48s -> "the largest mode's N (48, or 54 under MODES-3 (b))": r1:2283, r1:2740, r4:778-779, r4:879. Not first uses (first linked uses r1:1925, r4:708), so no link needed. Applied.
- r4:880-881 avoidance: "≤ 0.2 ms p99 native at N = 48 (proposed; at N = 54 the tick-budget check scales it ×1.125, GLOSSARY#1-tick-budgets)". Recomputed GLOSSARY §1 60 Hz sim share at N=54: 1.125+0.225+1.125+0.833+0.833 = 4.142 -> 4.14 (table says 4.14); without scaling avoidance it would be 4.117. So §1 does scale avoidance ×1.125. Consistent. Anchor #1-tick-budgets resolves to "## 1. Tick budgets". Applied.
- Item 3 pointers: r1:2285 R1-BND-REC2 +OQ16,+OQ17 (OQ16 r1:3097 "measured in the R1-BND-REC2 spike"; OQ17 r1:3102 "R1-BND-REC2 mark 5"); r1:2241 R1-DET-REC6 +OQ2 (r1:3022 "R1-DET-REC6's suite"); r4:898 R4-NAV-REC4 +OQ5 (r4:1193 "R4-NAV-REC4 wants a bot-free build"). Applied.
- R4-NAV-REC2 Rests on (r4:855-856): stress scene replaces "R1-TICK-REC2 (the P1 scene), R1-XU-OQ1"; "settled R1-XU-OQ1's scene part" holds (r1:2846-2847). Applied.
- R1-ECS-REC1 Rests on (r1:1915-1916): names R2's worked estimate for the 15 big vehicles. Applied.
- Hash-map ban beside iteration wording: r1:1633 (options table), r1:2406-2408 (R1-ABL-REC3), r1:3139-3140 (pointers between units, names R1-ECS-REC2, which holds the ban r1:1954-1955). Applied; anchor exists (G:711).
- Part 7 "Step 3d fixes (2026-10-30):" r1:3715-3725, r4:1430-1437, typed. Present.

## Check 2 (5 recommendation changes) — closed
R1-BND-REC2 mark 4, R1-BND-OQ6, R4-PHYS-REC4 spike, R4-NAV-REC3 bots, R4-NAV-REC3 avoidance cost. Values hold: under (a) N=48 unchanged; under (b) 54 from GLOSSARY "largest mode" (G:331-337). Numbers: 10 min, 6 m, 3 s, 0.2 ms pre-existing; ×1.125 = 54/48 arithmetic stated in GLOSSARY §1; "(proposed)" marks kept (r4:778, r4:879, r4:880; r1:2278 spike "proposed"). Both readings explicit in each sentence. Nit only: r4:880 states the scaled value as "the tick-budget check scales it" (GLOSSARY says "for this check only") rather than as a pass mark at 54; the prompt allowed exactly this wording.

## Check 3 (no new claim, number or source) — closed
No URL or source added; only new number is ×1.125 (GLOSSARY §1 arithmetic) and "4.2 ms at 60 Hz" in the part-7 record (GLOSSARY figure). New phrases "it replaced R1-TICK-REC2's P1 scene and settled R1-XU-OQ1's scene part" and "an R2 assumption, through the stress scene" restate G:389-410 and r1:2846.
