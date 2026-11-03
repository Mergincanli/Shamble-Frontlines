# verifier fixcheck-d3 unit c: whole-document sweeps over the nine v2 decision documents

Verifier: fixcheck-d3c (fresh). Started 2026-11-03 07:57 Amsterdam at HEAD 78ccee8 (branch claude/laughing-lovelace-ox61se).
Documents: docs/adr/0001..0006, docs/ENGINE_OVERVIEW.md, docs/SPIKE_PLAN.md, docs/adr/README.md.
Baseline for task 1: 47eadd9. Scripts: verifier-fixcheck-d3c-scratch/.

## Task 1: lost or doubled structural lines, 47eadd9 against HEAD (78ccee8)

Scripts: verifier-fixcheck-d3c-scratch/structdiff.py (lost and changed lines, each attributed to the fix block whose
BEFORE holds it; a multiset check for a copy lost while its key survives) and doubled.py (doubled headings, bold ids,
table first cells, PM rows, numbered-list numbers, a label twice in one item). Outputs: structdiff.out, doubled-HEAD.out.

Calibration (does the method catch the two known losses?): `structdiff.py 47eadd9 604a497` reports
"LOST 47eadd9:832: - **AB-3. Weapons keep ...**" and the Feel Lab step 4 as CHANGED into a second step 3;
`doubled.py 604a497` reports "3 at line 2349 then 3 at line 2351" (Feel Lab), the AB-2/AB-3 merge (What, Why,
Research, Need twice in the item at line 887) and AB-5's doubled Need (948, 949). Outputs: calib-47eadd9-604a497.out,
doubled-604a497.out. Both known losses are repaired at HEAD (AB-3 heading at 0005:903; Feel Lab steps 1-4 at 2348-2355).

Output at HEAD, structdiff.py 47eadd9 HEAD:
    TOTAL structural lines at 47eadd9: 3225; without the same key at HEAD: lost 0, changed 21
    multiset check: no key whose count fell
Per document (structural lines 47eadd9 -> HEAD): 0001 554->568, 0002 475->477, 0003 480->493, 0004 415->421,
0005 540->544, 0006 305->318, ENGINE_OVERVIEW 39->39, SPIKE_PLAN 130->132, README 287->314.

The 21 changed lines, each reworded on purpose by the block named (its AFTER holds the same item, reworded as the
block's finding asks); none lost:
| # | Line at 47eadd9 | Now at HEAD | Fix block (file:line) | Verdict |
|---|---|---|---|---|
| 1 | 0001:378 part 12 heading "In-match upgrades, weapon drops and pickups" | 0001:503 "... upgrades, purchases, weapon drops ..." | netcode-architect-fixes-final.md:488 GD-2 | kept, renamed |
| 2 | 0001:561 failure row "A tick counter wraps" | 0001:720 "A tick or sequence counter wraps" | netcode-architect-fixes-final.md:693 RR-3 | kept, widened |
| 3 | 0001:665 option row U1 (chosen) | 0001:829 U1 (no longer chosen) | netcode-architect-fixes-final.md:785 PA-3 | kept, choice moved |
| 4 | 0001:666 option row U2 | 0001:830 U2 (chosen) | same block | kept, choice moved |
| 5 | 0001:880 OQ-0001-5 "200 or 250 ms" | 0001:1060 "which value" | netcode-architect-fixes-final.md:879 NA-1 (d) | kept, reworded |
| 6 | 0001:894 OQ-0001-7 | 0001:1084 "+ the active-reload tap" | same block (GD-8) | kept, widened |
| 7 | 0001:1244 checklist step 7 | 0001:1517 step 7 "at the measured cap" | test-engineer-A-fixes-final.md:420 NA-1 | kept, reworded |
| 8 | 0002:274 part 5 "One command per tick, posted" | 0002:311 "The newest input, posted once per drawn frame" | engine-architect-0002-fixes-final.md:228 NA-7 | kept, reworded |
| 9 | 0002:598 Q4 chosen row | 0002:746 + `client-endpoint` crate | engine-architect-0002-fixes-final.md:615 EA-8 | kept, widened |
| 10 | 0002:642 C1 row | 0002:790 C1 "(chosen, posted once per drawn frame ...)" | engine-architect-0002-fixes-final.md:623 NA-7 | kept |
| 11 | 0002:734 trade-off "A Worker, one block out and one command in" | 0002:882 "... the newest input in per frame" | engine-architect-0002-fixes-final.md:631 NA-7 | kept |
| 12 | 0002:745 "Two crates beyond R5's list" | 0002:894 "Three crates ..." | engine-architect-0002-fixes-final.md:644 EA-8 | kept |
| 13 | 0002:1254 migration step 5 "Slice 3, online" | 0002:1424 "+ `supervisor`" | engine-architect-0002-fixes-final.md:841 EA-12 | kept (wrap defect: D3C-1) |
| 14 | 0003:1072 checklist step 9 | 0003:1248 step 9, two test views | test-engineer-B-fixes-final.md:284 AD-5 | kept |
| 15 | 0003:1111 checklist step 28 | 0003:1297 "At Low, ..." | test-engineer-B-fixes-final.md:324 GE-7 | kept |
| 16 | 0004:422 part 17 heading "Economy and objectives" | 0004:480 "Spawning, economy and objectives" | rust-reviewer-fixes-final.md:126 GD-4 + ... | kept, widened |
| 17 | 0005:1604 options row "per slot class" | 0005:1770 "per slot content" | graphics-engineer-fixes-final.md:623 GE-1 | kept, renamed |
| 18 | 0005:1609 "Shared per class." | 0005:1775 "Shared per slot content." | graphics-engineer-fixes-final.md:631 GE-1 | kept, renamed |
| 19 | 0005:2149 families step 4 (trees) | 0005:2329 step 4 (GW18 builders) | test-engineer-B-fixes-final.md:896 AD-12 | kept |
| 20 | 0005:2156 LOD step 2 "A slow zoom" | 0005:2341 "The camera moved slowly ..." | test-engineer-B-fixes-final.md:865 AD-5 | kept |
| 21 | 0006:523 row "Larger paid runners (GPU; macOS ...)" | 0006:661-662, split into "Standard macOS runners ..." and "Larger paid runners (GPU)" | infra-engineer-fixes-final.md:470 IE-3 | kept, split |

Doubled lines at HEAD (doubled.py HEAD, 24 items, all by design):
- (1) 0005's six section headings repeat once per section (Context, Decision, Options considered, Trade-off analysis,
  Open questions x3; What a spike must prove x4): the record's three-section layout, by design.
- (2) 0005's bold ids CD-1, -2, -3, -4, -7, -8, -9, -10, OQ-0005-CD2, AB-1, -3, -4, -5, -6, -8, -9, -10 appear twice:
  once as the definition ("- **CD-1. JSON is ...**", 261) and once as the spike mark that cites it ("1. **CD-1** (R1-DATA-REC1
  mark 1 ...", 693). Not doubled definitions.
- (3) SPIKE_PLAN:81 and :84, first cell "–" twice in the spike table: the two not-a-spike rows (the foundation's
  acceptance checks; the asset-pipeline research), by design.
- (4) numbered lists: none repeats, skips or restarts out of order in any of the nine documents (calibration: catches the
  Feel Lab's 3, 3 at 604a497).
- (5) no label (Need, What, Why, Research) twice in one item at HEAD (calibration catches 604a497's four).
- PM ids: no PM row twice in any document.
Task 1 result: 3225 structural lines checked; 0 lost, 21 reworded on purpose, 0 doubled. One cosmetic wrap defect
(D3C-1, below).

### D3C-1 (low, other): two fix blocks left a broken wrap in a numbered step
- 0002:1424-1425 (engine-architect, Migration plan, block EA-12 at engine-architect-0002-fixes-final.md:841):
  "5. **Slice 3, online:** `server` and `supervisor` enter; L2 in the nightly run; G12 over both transport backends; the"
  / "   first multiplayer" / "   preview behind the access wall, ...": a two-word line in mid-sentence. Renders as one
  paragraph in Markdown, so nothing is lost; smallest fix: join "first multiplayer" to the next line.
- 0001:1518 (test-engineer A, the netcode checklist step 7, block NA-1 at test-engineer-A-fixes-final.md:420): a 156-character line
  ("it, so the cap clips almost no honest shot), shoot the sprinting target for two minutes. Expected: when his crosshair
  is on the target as he sees it, the"). Smallest fix: rewrap at 120. Cosmetic only.
END

## Task 2: dangling references at HEAD (78ccee8), all nine documents

Scripts: verifier-fixcheck-d3c-scratch/refs.py (ids, codes, parts, research ids, links), steps.py (checklist and
migration-plan steps). Outputs: refs-HEAD.out, steps-HEAD.out. Definitions read by refs.py: 115 OQ ids (114 numbered +
OQ-SPIKE-1, SPIKE_PLAN:436), 340 PM rows, ADR 0005's 31 part codes (CD-1..14, AB-1..10, AP-1..7), its GPU section's
parts 1-9, each ADR's Decision parts (0001: 1-14, 0002: 1-12, 0003: 1-11 with 0003 part 5's sub-items a-n, 0004: 1-21,
0005: 1-9, 0006: 1-15), README's 106 decision-table codes, its register (I-1..21; seams S1, S3, S4, S6, S11, S12; gaps G1,
G2, G3, G4, G5, G7, G8, G9, G11, G15; N-n 12 rows; J1-J6), STATUS's codes, the research's 1,198 defined ids at 9e7db28.

Output (refs.py HEAD), checked / resolved per class:
    0005code 556/556 · I-n 109/109 · J-n 36/36 · N-n 9/9 · OQ 1161/1160 · PM 1165/1165 · code 812/774 · gap 18/18 ·
    link 370/370 · part 1905/1880 · research 4094/4094 · seam 10/10 · TOTAL 10245, 10181 resolved, 64 unresolved
steps.py HEAD: checklist steps 17/17 resolve (each cited step's text matches what the citing sentence says);
migration-plan steps 15/15 resolve (each to the slice the citing sentence names).

The 64 unresolved, read one by one:
- OQ (1): 0002:1625 "ADR 0004 (parts 1, 5, 9, 18-20, OQ-0004-7, S18)": the script read "S18" as OQ shorthand; it is ADR
  0004's spike claim S18 (0004:999). Resolves. No finding.
- part (25): all resolver misses, each read and resolved by hand: "ADR 0004's (part 20)" 0002:251; "ADR 0006's (part 13"
  0002:627, :654; "ADR 0004 (parts 1, 5, 9, 18-20" 0002:1625; "ADR 0004's (its part 14" 0003:264; "ADR 0006 ... its part
  12" 0005:100; "ADR 0001 / part 12" 0005:104, :941; "ADR / 0003, part 5.a" 0005:1697; "ADR 0003 / part 11" 0005:1758;
  "ADR 0003 (its part 5.g)" 0005:1977; "ADR 0004 / part 16" 0005:2094; "ADR 0004 parts 18 and 19 ... (part 18" 0006:448,
  :1478; SPIKE_PLAN:177-179 (ADR 0002 parts 6, 2, 4, 7, 11, under "What it proves: ADR 0002 part 5"); SPIKE_PLAN:288
  ("part 13's B9", ADR 0006 part 13, the preview hosting check); README:66 (OQ-0003-2's row, ADR 0003 part 5.b), README:79
  (OQ-0003-18's row, ADR 0003 parts 5.l, 5.m), README:272 (the ADR 0003 bullet, part 10). All exist. No finding.
- code (38): V-S, B-S, B-N (0001:1354, 1370, 1372 define them in bold; 0003:1104 and 0006:1183-1184 cite "ADR 0001's V-S"),
  and LE-D (README:250, the "LE-C, LE-D" row): resolve; the script's local-definition pattern missed them. The other 15
  occurrences name review codes no repository file defines (D3C-3 below).
Beyond the script: seams cited as "the decision map's S5 ..., S9 ... and S10" resolve to no register row (D3C-2 below).

### D3C-2 (medium, dangling reference): three seam codes the register does not hold
- Where: docs/adr/0005-content-abilities-assets.md:106-107 (engine-architect, the frame's "Where this record sits among
  the drafts"). Current text: "Its seams are the decision map's S5 (content, execution and drawing), S6 / (moments and
  telemetry), S9 (the look and the asset rules) and S10 (the editor)."
- What should resolve: README "Codes the drafts cite" says the map is "a checkpoint file deleted before the PR is marked
  ready. Their definitions live here from now on", but its Seams table (README:325-333) holds S1, S3, S4, S6, S11 and S12
  only. S5, S9 and S10 are defined only in .checkpoints/v2-decision-drafts/engine-architect-decision-map.md:207, 211, 212,
  so they dangle once the checkpoint is deleted. Present at 47eadd9 too (not a fix-block loss).
- Smallest fix (engine-architect, README is the synthesis's): add three rows to README's Seams table from the map's lines
  207, 211 and 212 (S5 content vs execution vs drawing: 0005 / 0004 and 0003; S9 the look vs the asset rules: 0003 /
  0005; S10 the editor: 0002 / 0005, 0004). Or, in 0005:106, drop the three codes and keep the glosses.
END

### D3C-3 (low, dangling reference): review codes and review finding numbers resolve only in checkpoint files
- Review codes cited in the documents and defined in no repository file outside .checkpoints/ (all authored as
  provenance labels beside an id that does resolve):
  - 0001:478 "OQ-0004-10 (GD-REJOIN: Mustafa's, ...)"; 0001:513 "OQ-0005-AB15 (GD-PICK-VIS, Mustafa's)"; 0001:604
    "OQ-0004-9, GD-CARRIER: Mustafa's" (netcode-architect); 0005:1367 "(GD-PICK-VIS)" (game-designer, OQ-0005-AB15).
  - 0002:1253, PM-0002-44's source cell: "the engine-architect, security and Rust reviews of 2026-11-02 (EA-8, SA-10,
    RR-10)" (test-engineer A, pass marks).
  - 0006:932-933 "the infra review's finding / 1 and its question RELEASE-WINDOW" (infra-engineer, OQ-0006-14).
  - README:77, 79, 80, 81, 87: "the art review's LOOK-FLOOR", "REDUCED-MOTION", "COLOUR-TELL", "the test review's
    HEAT-SOAK", "the reviews' POOL-FULL" (engine-architect, synthesis; each row's question text is self-contained).
- Review finding numbers: 35 pointers of the form "the X review, finding n" / "review's question n" (0001 2, 0002 3, 0003
  5, 0004 1, 0006 21, SPIKE_PLAN 1, README 2); the reviews live in .checkpoints/v2-decision-drafts/review-*.md, deleted
  before the PR is marked ready (CHECKPOINTS.md §9); the report appendix §7.2 keeps only each review's verdicts and high
  findings. Also "decision map, part d" (0005:97, :224), "part g" (0005:2103), "parts a, d, g and j" (0005:2500): parts
  d, h and j have README homes (interfaces, gaps, J-n); parts a and g do not.
- Smallest fix: one sentence in README's "Codes the drafts cite": review codes (GD-REJOIN, GD-PICK-VIS, GD-CARRIER,
  RELEASE-WINDOW, LOOK-FLOOR, REDUCED-MOTION, COLOUR-TELL, HEAT-SOAK, POOL-FULL, EA-8, SA-10, RR-10) and "the X review,
  finding n" name the step-10 reviews of 2026-11-02 and the decision map's parts, kept in the branch's history under
  .checkpoints/v2-decision-drafts/ at the last commit before their deletion (name the SHA). No draft text needs to change.
END
Task 2 result: 10,245 references checked plus 32 steps; every id, code, part, research id, link and step resolves
except D3C-2 (3 seam codes) and D3C-3 (12 review codes in 15 places, 35 review finding numbers, map parts a and g).
