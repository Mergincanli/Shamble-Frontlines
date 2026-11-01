# Verifier verdicts: the synthesis (ENGINE_OVERVIEW, SPIKE_PLAN, adr/README, GLOSSARY §8), 2026-11-01

Base: the synthesis commit 80a27b3 (drafts as assembled). HEAD at writing: d7f8d9a; since 80a27b3 only
`docs/adr/0006-infrastructure-operations.md` changed in docs (fix round 09803a9), noted as drift (finding 6).
Ledger: `engine-architect-synthesis-claims.md` (rows 1–25, 31–57). Rows X1–X9 are claims in the documents with no
ledger row (check 1, "both ways"). Line references: E = docs/ENGINE_OVERVIEW.md, SP = docs/SPIKE_PLAN.md,
RM = docs/adr/README.md, G = docs/research/GLOSSARY.md; 000N = docs/adr/000N-*.md at 80a27b3.

| # | claim (short) | cited | verdict | where and the words | note |
|---|---|---|---|---|---|
| 1 | 30 Hz baseline; 60 Hz only with a 60 Hz margin ≥ 25 % | TICK-30; G §8 | confirmed | STATUS:161–163 "a 30 Hz server tick is the baseline for 48 players; 60 Hz only if the netcode proof test shows at least 25 % headroom"; G:788–793 | E:14–15, SP:14, RM:317 |
| 2 | 48 players, largest mode 24v24 | MODES-3 (a) | confirmed | STATUS:147 (MODES-3 (a): totals, biggest match 24v24); G:386–388 | |
| 3 | 60 Hz margin: p99 ≤ 6.25 ms vs 8.33 ms; one vCPU of RS; stress scene N = 48, client per soldier; 30-min soak; re-flood; REC6's two shares, P1–P3 | cond. 2; G §8; 0001 part 2, OQ-0001-1 | confirmed | advisor-verdicts.md:21–25 (cond. 2) word for word in substance; G:788–793; SP:132–137; 0001:840 (OQ-0001-1). Arithmetic: 0.75 × 8.333 = 6.25 ms | |
| 4 | Stress scene: 16v16v16, 15 big vehicles (4 very large), 300 projectiles, ≤ 8 effects/soldier (384), DM_Tri-sized greybox | G §3 | confirmed | G:399–411 ("15 in play … 4 are very large units"; "300"; "N × 8 (384 at N = 48)"; "DM_Tri-sized greybox") | SP:21–25 |
| 5 | Test-mode list: 24v24, 12v12v12; 16v16v16; 1v1, 4v4, 2v2v2, 6v6v6; N = 64 headroom | G §2 | confirmed | G:378–388 | SP:26–27 |
| 6 | Sizes S ≈ a day, M 2–4 days, L a week+ | passmark files | confirmed | passmarks-0001:263 (same line in all six files) | SP:17–18 |
| 7 | Each spike's size | passmark spike entries | confirmed | passmarks-0001:289 (L); -0002:266 (L), :290 (L, six change-sets); -0003:246, :264 (L, L); -0004:247 (L), :270 (S), :281 (S), :294 (M); -0005:260 (L), :282 (S/M), :298 (M), :311 (M); -0006:184 (S, ≥ 2 days), :207 (M), :220 (S) | SP:72–85 and items |
| 8 | Pass-mark id ranges per spike | drafts' tables | confirmed | table sections: 0001:1092–1153 (1–56; "The 60 Hz margin" 39–40); 0002:1003/1038/1074 (1–34, 35–69, 70–76); 0003:951/969/995 (1–17, 18–42, 43); 0004:939/1001/1007/1009/1013 (1–61, 62–66, 67, 68–70, 71–72); 0005:1993/1995/2012/2028/2034/2042 (1–32, 33–37, 38–44, 45–46); 0006:828/839/857 (1–10, 11–27, 28–31). Row counts 56, 76, 43, 72, 46, 31 = max ids. Unchanged at HEAD for 0006 | conditionals checked: PM-0001-54/-55, PM-0002-21/-27/-33, PM-0003-40, PM-0005-11, PM-0005-44, PM-0006-13 |
| 9 | 60 Hz run's marks PM-0001-39, -40; PM-0004-71, -72 | 0001, 0004 | confirmed | 0001:1134–1135; 0004:1014–1015 | |
| 10 | Checklist homes | drafts' #### headings | confirmed | 0001:1192 / PM-0001-53; 0002:1136 / PM-0002-34; 0002:1159; 0003:1023 / PM-0003-17; 0003:1092; 0005:2095 / PM-0005-17; 0005:2110 / PM-0005-37; 0005:2123; 0005:2128 / PM-0005-46; 0006:897 / PM-0006-9; 0006:920 steps 1–2 = PM-0006-12, PM-0006-18 | 0006 lines at 80a27b3 (HEAD: 924, 947) |
| 11 | N = 12 to 48 frame-time comparison | 0002:883–890 | confirmed | 0002:884 "at N = 12, 24, 36 and 48 soldiers" | SP:155 |
| 12 | 60, 120, 144 Hz displays | PM-0004-69 | confirmed | 0004:1011 "on 60, 120 and 144 Hz displays" | SP:159 |
| 13 | 8 rooms per VM, CCX23 and CX33 | 0006 spike item 2 | confirmed | 0006 (80a27b3) :740–741 "on the CCX23 and on the CX33, 8 rooms of 48 players"; source R6-HOST-F15 spot-checked (findings-checked.tsv:624) | |
| 14 | PREV-DELETE within 24 h | 0006 part 3 | confirmed | 0006:155 "24 h after a pull request closes"; PM-0006-6 | SP:247 |
| 15 | 25 MiB largest file | 0006 part 1; R6-HOST-REC1 | confirmed | 0006 spike item 1 "file stays under 25 MiB"; R6-HOST-F3 (r6:72–79, spot-checked r6:1109); re-read at source, fetch audit row 1: "The maximum file size for a single Cloudflare Pages site asset is 25 MiB" | SP:246, :343 |
| 16 | 384 active effects | G §3 | confirmed | G:404; 48 × 8 = 384; PM-0004-62 "with 384 active effects" | SP:290 |
| 17 | About 60 marks, seven targets, three machines | passmarks-0004:247–249 | confirmed | passmarks-0004:248 "a determinism suite over seven targets; three machines; about 60 marks" (PM-0004-1 to -61) | SP:185 lists six targets; the seventh (ML Windows browsers) is in PM-0004-19's hardware column (finding 9) |
| 18 | Three T1 abilities; Molotov and Shield | LOOP_V2 §5; ABL-PREDICT | confirmed | LOOP_V2:58 "T1 class abilities (Molotov, Riot Shield, Banzai blade)"; STATUS:165 | |
| 19 | 88 OQs (13, 7, 15, 7, 34, 12) | grep of drafts | confirmed | `grep -c '^- \*\*OQ-'` per draft = 13, 7, 15, 7, 34, 12; sum 88; 0005: 8 CD + 13 AB + 3 AP + 6 AS + 4 = 34 | RM:15–22. HEAD 0006 still 12 |
| 20 | 38 DESIGN ids | id table | confirmed | ids.tsv owner DESIGN = 38; RM:108–145 id set equal (diff empty, no duplicates) | |
| 21 | 28 critic ranks; 72 XT; 30 XU | id table, map f2 | confirmed | RM:176–203 = 28 rank rows holding 72 XT ids = ids.tsv's 72 (diff empty); owners and ranks per row equal ids.tsv; ids.tsv has 30 XU | |
| 22 | Line numbers in RM rows | drafts at assembly | confirmed | every OQ line in RM:32–103 equals the OQ's line at 80a27b3 (e.g. 0001:840, 846, 856, 861, 867, 881–905; 0002:800, 836, 848, 859; 0003:706–793; 0004:742–784; 0005:583–632, 1095–1226, 1481, 1489, 1624, 1833–1876; 0006:598–701); 0001:161 "(at N = 54, MODES-3 (b))"; 0005:76 ASSET-RESEARCH | drift: 0006 OQ lines moved +15 to +26 after 09803a9 (finding 6); RM:10 warns lines may move |
| 23 | P2 = 100 ms round trip with jitter and loss | OQ-0001-3 | confirmed | 0001:856–857 "P2: 100 ms round trip, ±20 ms jitter, 2 % loss (part 13)"; 0001:392 | RM:34 |
| 24 | 118 rows "look brief pending" | map LE-H | confirmed | map:921; source LEGACY_INVENTORY.md:23 "Look brief pending on 118 rows" | RM:225 |
| 25 | GLOSSARY at 9e7db28: 742 newlines, 64,088 bytes, unterminated last line | git show | confirmed | `wc -lc` = 742 64088; `cmp` of the first 64,088 bytes of G with 9e7db28: identical; byte 64,089 = "\n" | process claim; not stated in the documents themselves |
| 31 | One Rust sim, native per room and Wasm in one Worker; client endpoint online, local server in local play from slice 1 | brief §1; SLICE-ORDER 1; 0002 parts 5–6; 0004 | confirmed | STATUS:140–141 (condition 1); 0002:303–308 (part 6); 0002:182 (`sim-wasm`: client endpoint and local server) | E:8–10 |
| 32 | Page makes no Wasm call, no network; one command up, one transferred block down (B2; B3 measured) | 0002 part 5 | confirmed | 0002:248–266 (B2 "transferred to the page"; "The SharedArrayBuffer ring (B3) is the spike's measured" alternative; C1 "One command per tick, posted"); 0002:286–287 "never touch the page" | E:49–50, 55–63 |
| 33 | Crates and dependency rules | 0002 part 3 | unsupported (one clause) | 0002:197–206 confirm every rule; but E:68 "tools (`devtools`, `navbake`, the editor) may depend on everything": 0002:185 `devtools` "anything", 0002:186 `navbake` depends on "`sim`, `content`" | finding 5 |
| 34 | Per-tick flow | 0001 parts 4–9, 11; 0004 parts 4–19 | confirmed | 0004:161 (tick counter), :253–262 (spread on the authority-only stream, SEC-SPREAD), 0001:369 "bots through the human input path", 0001:214, 285 (parts 6, 9), 0001:266 (part 8, rewind cap) | E:72–84 |
| 35 | Per-frame flow | 0003 part 10 | confirmed | 0003:373–388 (part 10: newest block, interpolate, animate, draw, events, cursor ray) | ledger's 0003:377 is the part's 5th line; part starts 0003:373 |
| 36 | State ownership; production pins one content set per match | 0001 "State ownership"; CD-7 | confirmed | 0001:444 "### State ownership"; 0005:358 "CD-7. Reload: production pins one content set per match" | E:90–93 |
| 37 | The §6 table, draft and part per requirement | map e1; drafts' parts | wrong (one cell) | all parts checked against the drafts' part headings (0001:110–400, 0002:133–543, 0003:122–389 with 5.i at 0003:223, 0004:125–474, 0005 CD/AB/AP headings, 0006:121–385). Wrong: E:101 "ADR 0005 AB-6 (the two primitives §6.4 needs: an effect removed when its source dies, and marking)"; 0005:875 names the two primitives as "an effect removed when its source dies, and a trigger on the last hit or tick that moves the victim's item to the source" (= r1:2372–2376, R1-ABL-REC1 mark 3); marking is a separate AB-6 row, 0005:876 ("an effect that grants a tag … what marking does (R1-ABL-OQ6)") | finding 1 |
| 38 | Netcode proof test before the foundation | SLICE-ORDER 3 | confirmed | STATUS:144 "The netcode proof test … still runs before the foundation." | SP:40, RM:42 |
| 39 | The rest of the order is a proposal | map J6 (5), part g | confirmed | map:962–965 (the proposed order, same waves as SP:44–70) | |
| 40 | Spikes throwaway in `spikes/`, legacy-exempt; none before approval | SPIKE-SCOPE (a); brief §4 | confirmed | STATUS:235–236; brief:48 ("exempt from the legacy hard constraints"); brief:108 "No engine code, and no spikes yet."; CLAUDE.md "only after Mustafa and the Advisor have approved the drafts" | SP:12–13 |
| 41 | Each spike's proves / depends / subsumes / decides lists | passmark spike entries; drafts | confirmed | script diff of every research id per item (SP items 1, 3–10, foundation) against passmarks-000N "Spike entries": the only differences are parenthetical cross-references (R1-XU-OQ12, R3-BUDGET-OQ2/OQ3, R1-DATA-REC2, R3-CHAR-REC1/4, R3-LOOK-F3, R1-BND-REC1, R2-XU-OQ1, R2-SYNC-OQ2/OQ15) and the moves the entries themselves ask for (R1-ABL-OQ7, R1-XU-OQ7 to the sim core spike: passmarks-0005 "PM-0004-9 and PM-0004-11 run their marks in the sim core spike"; carry-forward 2 and R1-TICK-OQ1 to item 2) | |
| 42 | ADR 0004 parts per spike | 0004:822–843 | confirmed | S1–S14, S19, S20 sim core (parts 2–4, 6–15, 20, 21); S15, S16 content (16, 18); S17 proof test (5); S18 boundary (5); S2 in the boundary spike's Worker (6); S20 with the boundary spike (20) | |
| 43 | ADR 0001 parts per item | 0001:930–994 | confirmed | 0001:931 (parts 1, 10), :948 (2, 3, 6, 11, 14), :965 (2), :968 (4, 7, 8) | SP:90–96 |
| 44 | ADR 0003 look parts 1, 3, 4, 5, 11; bench 6–9 | 0003:806, :838 | confirmed | 0003:806 "(LOOK_V2 §9; parts 1, 3, 4, 5 and 11)"; 0003:838 "(parts 6, 7, 8 and 9)" | |
| 45 | ADR 0006 hosting parts 1–3; ops parts 5, 7, 8, 9; optional Sentry | 0006:717–769 | confirmed | 0006 (80a27b3) :718 "proves parts 1, 2" and 3; :739 "They prove parts 5, 7, 8 and 9"; :763 "Optional, in the boundary spike … only if Mustafa has created the EU Sentry organization" | |
| 46 | R2-SYNC-REC6 mark 1 and R5-LAYOUT-REC4 placed in the sim core spike | map g; passmarks-0004 | confirmed | passmarks-0004:257–260 "Not placed by any draft … SPIKE_PLAN places them" | SP:204–207 |
| 47 | R1-TICK-REC4 in the proof test through PM-0004-67 | map g; PM-0004-67 | confirmed | passmarks-0004:281–283; 0004:1008 basis "R1-TICK-REC4 (lines 2082–2092)" | SP:121–122 |
| 48 | Asset-pipeline research: graphics-engineer, verified like R3, own daytime session, before the asset spike; timing for Mustafa and the Advisor | ASSET-RESEARCH (a), cond. 4 | confirmed | advisor-verdicts.md:32–37 (cond. 4: "graphics-engineer, verified like R3, in its own daytime session … SPIKE_PLAN places it before the asset spike"; "so Mustafa and the Advisor can move the research earlier") | SP:328–337, RM:36 |
| 49 | Feel Lab test after abilities are playable (slice 4) | ABL-PREDICT; map LE-N | confirmed | STATUS:153, :165; map:959–960 "once abilities and projectiles are playable (LE-N)"; passmarks-0005 Feel Lab entry | |
| 50 | Decisions table rows (decider per OQ) | each draft's "Decider" | confirmed | every RM row's decider read against the OQ text; every omitted OQ has a decider that is neither Mustafa nor the Advisor (0001-6, -13; 0002-2, -3, -7; 0003-6, -7, -14; 0004-2, -3, -7; 0005-CD8, AP1, AS1, AS3–AS6; 0006-5, -11) | notes: OQ-0003-6's "what decides it" includes "Mustafa's look" (0003:744–745) (finding 8); OQ-0006-5 gained the Advisor after 09803a9 (finding 6) |
| 51 | WEAPON-UNLOCKS, carry slots, looted ammo, levelling = AB5, AB1, AB2, AB3 | 0005:1095–1148; UPGRADE-SCOPE | confirmed | 0005:1148 "OQ-0005-AB5. WEAPON-UNLOCKS"; 0005:1095, :1107, :1115; inputs/UPGRADE-SCOPE "carry slots, ammo for looted weapons, and how fast tracks level" | |
| 52 | 38 DESIGN rows, question and why | id table notes; research | confirmed | dispositions equal ids.tsv notes; 21 question texts read (r1:2667, 2721, 2772–2775, 2815; r2:3014, 3018, 3091–3099, 3255; r4:1112–1142; r6:913, 915) | finding 7 for RM:109's "decision map S11" |
| 53 | Loose-ends rows and where each is resolved | map f1–f3, J2; drafts | citation off (STATUS lines) | CF-1..5 = STATUS:167–182; 0001:161; 0002:120, :339–340, :773; 0003:140, :1182; 0005:282; 0006 part 7 (80a27b3 :214–229); PM-0002-4, PM-0003-20 (LE-B); PM-0006-6. But RM:208–214 and RM:333 cite STATUS:198, 199, 200–201, 202, 257, 262, 273: right at 9e7db28, moved by 549490f; now STATUS:246, 249, 250–251, 252, 314, 319, 330 | finding 2 |
| 54 | Needs outside VISION_V2 §6, per draft | the drafts' "Need:" lines; map J3 | wrong (incomplete and over-inclusive) | lines checked against every "Need:" line of 0001–0006 at 80a27b3. Missing: R2-BUD-REC4 (the input buffer and the clock), 0001:1358 "need: VISION_V2 §4 only", listed by 0001:1433 (R2-XU-OQ18) with R2-BUD-REC5 and R2-TRN-REC6. Over-inclusive: RM:244 "roof fade: LOOK_V2 §4" (0003:232 "Need: VISION_V2 §6.2; LOOK_V2 §4", and E:99 places part 5.i under §6.2); RM:251 "three damage stages and structure states (part 14): LOOK_V2 §4" (0004:391–392: structure states need §6.2, §6.4, §6.5; "the damage stages: LOOK_V2 §4 only") | finding 3, finding 4 |
| 55 | Codes the drafts cite | map parts a, c, d, h, j; grep | confirmed | grep of 0001–0006: seams S1, S3, S4, S6, S12; gaps G1–G5, G7–G9, G11, G15; N1, N2, N6, N12, N30, N32, N34, N37, N39–N41, N48; I-1..I-10, I-12..I-21; J1–J6; LE-B = RM:265–337 (I-11 defined but not cited: harmless). Definitions match map:802–822 | |
| 56 | The drafts replace ADR-001..003 | STATUS; CLAUDE.md | confirmed | STATUS:246 "ADR-001..003 below: the v2 drafts `docs/adr/0001`–`0006` replace them"; CLAUDE.md "they replace ADR-001..003" | |
| 57 | G §8 quotes equal brief 52–59 and 60–66 | diff | confirmed | every `>` line of G §8 (G:750–757, 768–774), prefix removed, equals DRAFTS_BRIEF.md:52–66 byte for byte (15 lines, `diff` empty). Closing note G:797 equals cond. 3's text byte for byte; G:85 "which does not change" | |
| X1 | The page never decides a hit, a death or a purchase | (none) | confirmed | 0001:110–114 "Every outcome (movement results, shots, damage, deaths, economy, …) comes from the authority's sim" | E:11–12 |
| X2 | Content is JSON that one Rust crate validates the same way on server, Worker and editor | (none) | confirmed | 0005:9–10 "one Rust program, the content crate, reads and checks in the same way on the server, in the browser and in the editor"; CD-1, CD-2 (0005:258, 269) | E:12–14 |
| X3 | Diagram labels (sim crate: plain typed tables, integer ticks, parry3d, baked navigation; one room per process; hashed bundle) | (none) | confirmed | 0004:125, 161, 295, 320; 0006:178 "one room per process"; CD-8 (0005:379) | E:18–48 |
| X4 | "The drafts" table: what each decides and its spikes; 0004 has shares in four others | (none) | confirmed | part lists of each draft (row 37's headings); 0004 shares: PM-0004-62–66, -67, -68–70, -71–72 = four spikes | E:113–120 |
| X5 | "What it rests on": the settled list | (none) | confirmed | STATUS:113–116, 137–165, 207, 235 (MODES-3 (a), TICK-30, SLICE-ORDER, PREVIEW-WALL (a), SEC-SPREAD (a), ABL-PREDICT, BASELINE-WHEN (b), UPGRADE-SCOPE, SPIKE-SCOPE (a), R7-MARKS (a), LOOK_V2 with glTF/GLB) | E:123–128 |
| X6 | Machines RS, SH, LG, MV (a CX33 with the metrics stack), ML, RD, MAC, MP | (none) | confirmed | 0001:1019 (SH "the shared CX33"; LG); 0006:814–815 "MV, a CX33 in the same location running Prometheus"; 0003:892, 896 (ML, RD) | SP:28–32 |
| X7 | Network profiles P0–P4, simulator layers L1, L2 | (none) | confirmed | 0001:391–399 (part 13); 0001:1047 (L1, L2; R5-TEST-REC4) | SP:33–34 |
| X8 | Later milestones list | (none) | confirmed | map:955–960 (same ids and slices); the mode-files line from ADR 0001 (R2-ROOM-REC7) | SP:374–380 |
| X9 | RM drafts table: titles, authors, status | (none) | confirmed | each draft's line 1 and line 5 (**Author:**, **Status:** Proposed); 0005's three authors | RM:15–20 |

## Counts
61 rows (52 ledger rows, 9 added). Confirmed 57 · wrong 2 (rows 37, 54) · unsupported 1 (row 33) · citation off 1
(row 53) · unverified 0 · pending 0. Arithmetic recomputed: 6.25 = 0.75 × 8.333; 8.33 = 50 % of 16.67;
384 = 48 × 8; 88 = 13 + 7 + 15 + 7 + 34 + 12; 34 = 8 + 13 + 3 + 6 + 4; 28 ranks hold 72 XT ids. All hold.

## Rule checks (brief §4)
- Rule 2 (a number with no source): none; every number traces to a settled code, GLOSSARY, a draft or a count.
- Rule 3 ("not researched" proposed as the decision): none; OQ-0005-AS1–AS5 and AP1 stay open (SP:329–333, :355).
- Rule 4 / condition 8 (traces): the documents add no feature; the register of needs outside §6 is incomplete and
  over-inclusive (findings 3 and 4).
- Rule 5: no GLOSSARY "Retire (exact)" phrase in E, SP, RM or G §8 (grep of every phrase of G:87–792). "headroom"
  appears only as the N = 64 headroom run (SP:27, :184), the decision code TICK-60-HEADROOM, and G §8's own retire
  note (G:793–794): never for the 60 Hz margin.
- Rule 6 / condition 9: nothing decided beyond LOOK_V2; the look calls are named as Mustafa's (SP:223–226, RM:56–66).
- Condition 12: settled inputs listed as "never reopened" (E:122–128); none weighed as an option.
- Rule 7 / condition 2: SP:14–15 ("Every server number is at the 30 Hz baseline … Where a research mark gives 48 and
  54, 48 holds"); E:14–15; G §8's definition equals condition 2 (G:788–795).

## Findings (ranked)
1. **Wrong, E:101 (row 37).** Text: "ADR 0005 AB-6 (the two primitives §6.4 needs: an effect removed when its source
   dies, and marking)". ADR 0005 (0005:875) and its source R1-ABL-REC1 mark 3 (r1:2372–2376) name the second primitive
   as "a trigger on the last hit or tick that moves the victim's item to the source"; marking is a different AB-6 row
   (0005:876, "an effect that grants a tag"; what marking does stays R1-ABL-OQ6). Smallest fix: "ADR 0005 AB-6 (the
   leech's two primitives, an effect removed when its source dies and a last-hit trigger that moves the victim's item,
   and marking as an effect that grants a tag)".
2. **Citation off, RM:208–214 and RM:333 (row 53).** "STATUS:198", ":199", ":200–201", ":202", ":257", ":262",
   ":273" hold at 9e7db28; docs-keeper's 549490f moved them. Now STATUS:246, 249, 250–251, 252, 314, 319, 330. Fix:
   update the numbers, or cite STATUS by item ("ADR-001..003 below", SEC-MDL-2, the CDN-tag item, the wall-clock item).
3. **Incomplete register, RM:239 (row 54; rule 4, condition 8).** ADR 0001 marks R2-BUD-REC4 (the input buffer and the
   clock, parts 5 and 7) "need: VISION_V2 §4 only" (0001:1358), and its R2-XU-OQ18 row (0001:1433) lists it with
   R2-BUD-REC5 and R2-TRN-REC6 as the features README lists. Fix: add "R2-BUD-REC4 (the input buffer and the clock,
   parts 5 and 7)" to the ADR 0001 line.
4. **Over-inclusive register, RM:244 and RM:251 (row 54).** "roof fade: LOOK_V2 §4": its Need line is "VISION_V2
   §6.2; LOOK_V2 §4" (0003:232), and E:99 lists part 5.i under §6.2. "three damage stages and structure states (part
   14): LOOK_V2 §4": structure states need §6.2, §6.4, §6.5; only "the damage stages: LOOK_V2 §4 only" (0004:391–392).
   Fix: drop the roof fade from RM:244 (or add "beside §6.2"); RM:251 "three damage stages (part 14): LOOK_V2 §4 only".
5. **Unsupported clause, E:68 (row 33).** "tools (`devtools`, `navbake`, the editor) may depend on everything":
   0002:185 gives `devtools` "anything", but 0002:186 gives `navbake` "`sim`, `content`". Fix: "`devtools` may depend
   on anything; `navbake` on `sim` and `content`".
6. **Drift after the synthesis (rows 22, 50; not an error at 80a27b3).** ADR 0006's fix round 09803a9 moved its OQ
   lines (now 613, 625, 632, 643, 655, 673, 684, 694, 703, 712, 719, 727) and gave OQ-0006-5's development and staging
   case to the Advisor ("the development and staging case before the netcode proof test starts (the Advisor, from a
   short read)"). Fix in the register pass after the fix round: add an OQ-0006-5 row (the Advisor, dev and staging
   certificates) and re-run the line numbers; RM:10 already warns lines may move. 0006's PM ranges and OQ count are
   unchanged at HEAD.
7. **Dangling code, RM:109 (row 52).** "(decision map S11)": S11 is a map seam (map:213) that README's seam table
   (RM:290–297) does not define, and the map is deleted before the PR is ready. Fix: cite "SLICE-ORDER condition 1"
   only, or add S11 to the seam table.
8. **Small inconsistency, SP:225 vs SP:239 (row 50).** SP:225 names Mustafa's calls as OQ-0003-2, -3, -5, -8, -12,
   -15; SP:239 adds -6. OQ-0003-6's decider is "the look proof test", but what decides it includes "Mustafa's look"
   (0003:744–745); RM's decisions table omits it. Fix: add -6 to SP:225, or say at SP:239 that -6 is the test's.
9. **Small gap, SP:185 vs SP:199 (row 17).** "seven targets" (from passmarks-0004:248) against six listed at SP:185
   ("native, Node, three browser engines and real Safari"); the seventh is ML's Chrome and Firefox on Windows
   (PM-0004-19's hardware). Fix: add "and Chrome and Firefox on Windows" at SP:185.

Ledger-only notes (no document change): ledger 35 cites 0003:377 for part 10 (the part starts at 0003:373); the
ledger's "Not verified" item on XT ranks 2, 4 and 35 is now checked: 0001:1338 (part 5), 0001:1339 (part 7),
0003:1178 (part 9), each matching RM:176, :178, :198.

## Fetch audit
| # | time (Amsterdam) | host | URL | status | bytes | time | why |
|---|---|---|---|---|---|---|---|
| 1 | 2026-11-01 13:56:23 | developers.cloudflare.com (allow-listed) | https://developers.cloudflare.com/pages/platform/limits/ | 200 | 129,246 | 0.42 s | re-read of R6-HOST-F3 (row 15): the sentence "The maximum file size for a single Cloudflare Pages site asset is 25 MiB" is on the page |

Only one fetch: the synthesis restates no research finding directly; the source-level facts it carries (25 MiB:
R6-HOST-F3; CCX23 and CX33: R6-HOST-F15; Nuremberg, Helsinki, Falkenstein: R6-HOST-F14; the 10 GB cache in
OQ-0006-7's question: R5-CI-F5) are all inside their topics' spot-check tables (`inputs/findings-checked.tsv` rows
612, 623, 624, 566: "yes, confirmed"), so condition 16 asks no further source read. Fetched pages were treated as data.
