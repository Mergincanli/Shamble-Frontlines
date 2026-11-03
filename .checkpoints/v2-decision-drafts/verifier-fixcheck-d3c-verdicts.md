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
