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

## Check 4 (glossary hits on changed lines, 9 rows of glossary-hits-3d-final-r14.tsv) — closed: 9 right, 0 wrong
- R1 2050 · server tick budget · "25 %" · right: "25 % of the tick period, inside the server tick budget" names its whole.
- R1 2051 · server tick budget · "50 %" · right: "50 % of the tick period" beside "the server tick budget" link.
- R1 2281 · largest mode · "48" · right: "the largest mode's N soldiers (48, or 54 under MODES-3 (b))".
- R1 2408 · shared/authority-only stream · "the simulation's seeded RNG" · right: same sentence names "an authority-only stream" (r1:2409).
- R1 2740 · largest mode · "48" · right: beside "the largest mode's N".
- R4 778 · largest mode · "48" · right: beside "the largest mode's N" (r4:778-779).
- R4 855 · stress scene · "R1-TICK-REC2's P1 scene" · right: record of what the stress scene replaced, beside "the stress scene" link.
- R4 879 · largest mode · "48" · right: beside "the largest mode's N".
- R4 881 · largest mode · "48" · right: "at N = 48" is the stated measuring point, with N = 54's scaling in the same sentence and "the largest mode's N bots" at r4:879.
- Retire (exact): glossary_check.py --skip-records on the working tree (R1/R4/GLOSSARY identical to 5b57e40): R1 exact 0, R4 exact 0 (R1 in-context 129, R4 56; +1 in R4 = the new "N = 48" at r4:881).

## Check 5 (GLOSSARY nits) — closed
Word diff d54f3c0..5b57e40: only the five. (1) Time statistics definition, G:143-144 sentence added as asked. (2) G:70 "and the interest marks (0.70 ms)" (table G:60 = 0.70) and G:72-73 exact sums 4.11 (5.11), 9.92 (10.92): my recomputation 4.110, 9.918 -> matches. (3) G:74 and G:170 "for the server tick". (4) G:177 in "The server tick budget" Used today (the wording sat there, not in the shares entry, whose R1 line G:205 never had the defect); holds. (5) G:155. Removed tokens: "ms.", "whole" x2, "period)", "budget", "ruling)." — no number removed or changed.

## Check 6 (UNREAD_SOURCES, 33 changed rows) — 31 hold, 2 do not
All 33 "now" cells found exactly once in part 2 at 5b57e40. Part 1 rows checked against each topic's hinge table at 5b57e40 (r1:3203-3222, r2:3524-3536, r3:1511-1528, r5:2031-2048, r6:1029-1045): copied row for row.
Hold (tsv row: tie): 1 gdcvault 1024001 (R1-ECS-REC1; R1-TICK-REC2 "the same Overwatch talk"; R2-BUD-REC1 "gdcvault 1024001"; R2-SYNC-REC1; R2-SYNC-REC2) · 2 rand book (R1-DET-REC4) · 3,4 Epic docs and GameplayPrediction.h (R1-ABL-REC1..5) · 5 other Overwatch talks (R2-SYNC-REC3) · 6 sohu/juejin/gamedev write-ups (R2-SYNC-REC1) · 7 discourse 87805 (R3-WGPU-REC1) · 8 WebGL 2 spec (R3-GL2-REC1) · 9 EXT_disjoint (R3-GL2-REC3) · 10 GTAO (R3-LOOK-REC1) · 11 TF2 NPAR, 12 GG Xrd (R3-LOOK-REC3) · 13,14 SkinningNode src/docs (R3-CHAR-REC1) · 15 Steam, 16 Intel/AMD/NVIDIA (R3-BUDGET-REC1) · 17 GitHub runners (R3-MEASURE-REC3) · 18 lists.w3.org, 19 codereview (R3-AUDIO-REC1) · 20 brave.com (R3-WGPU-REC4) · 21 rapier.rs/docs/ "no tie found" (only r4:1516, the not-read row itself; no finding or note names the index) · 22 clippy lint list (R5-RUSTLINT-REC2, REC3) · 24 arithmetic_side_effects.rs (R5-RUSTLINT-REC3, REC4: "their clippy source files", r5:1788 OQ2) · 26 npm major (R5-LAYOUT-REC1) · 27 quickcheck/bolero (R5-TEST-REC1) · 28 wtransport (R5-TEST-REC2, REC4) · 29 netem (R5-TEST-REC4) · 30 nextest etc. (R5-TEST-REC1, REC6) · 31 Grafana (R6-OBS-REC2) · 32 R2 data location (R6-TEL-REC2) · 33 Workers limits (R6-HOST-REC1).
Do NOT hold:
- tsv 23, UNREAD:221 clippy_lints/src/disallowed_methods.rs -> "R5-RUSTLINT-REC3, REC4". The hinge row is "(`panic`, `arithmetic_side_effects` left out) | their clippy source files". disallowed_methods.rs is the disallowed lint, read via disallowed_items.rs (r5:2432 "the lint moved to disallowed_items.rs, read"; r5:2332 S38). Cause: build_unread.py MANUAL_TIES key 'https://raw.githubusercontent.com/rust-lang/rust-clippy/master/clippy_lints/src/' is a prefix: the same prefix artefact the fix meant to remove. Should be "no tie found in the file or notes".
- tsv 25, UNREAD:223 github.com/rust-lang/rust-clippy/tree/master/clippy_lints/src/disallowed -> same; same cause (prefix key 'https://github.com/rust-lang/rust-clippy/tree/master/clippy_lints/src/'). Should be "no tie found".
Sample rows re-checked: 3 (write-ups) now R2-SYNC-REC1, holds; 5 (rapier.rs/docs/) now no tie, holds; 10 (Intel/AMD/NVIDIA) now R3-BUDGET-REC1, holds.
Nits outside the 33: UNREAD:102 sohu.com/a/148848770 ("Overwatch talk write-up") and :103 juejin.cn still "no tie found" while R2-SYNC-REC1's hinge row names sohu.com and juejin.cn write-ups (no URL, so identity not provable); UNREAD:137 gdcvault 1024041 (Dan Reed's talk) lists only R1-ABL-REC1, while R2-SYNC-REC3's hinge row names "Dan Reed's on networking scripted weapons and abilities".

## Check 7 (labels) — closed
R1 (r1:10) keeps "verified (every recommendation claim)": the fix round added one basis (R1-ECS-REC1 Rests on R2's worked estimate, confirmed last pass as K1, r2:1762/1777) and changed only proposed marks' N. R4 (r4:7) keeps "spot-checked (50 of 52 key claims)": R4-NAV-REC2's new basis is the glossary's stress scene (no source claim); R4-PHYS-REC4 and R4-NAV-REC3 changed proposed marks only.

## Host audit
No fetch (curl 0, WebFetch 0). No source check needed: no new source or source claim in the diffs.

## Summary
Checks 1-5, 7 closed; check 6: two rows wrong (UNREAD:221, :223), same prefix-tie fault in build_unread.py MANUAL_TIES. Nits only otherwise.
DONE
