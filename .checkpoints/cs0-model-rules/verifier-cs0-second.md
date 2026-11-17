Task: cs0-verifier-second-look (PR #56, branch claude/wonderful-hamilton-jdmlnf, base main c94c81b; second look at what changed after cacea55)

## Plan (blocks, in order)
- B0: state (branch, HEAD, log cacea55..HEAD, diff stat); read first pass's findings 1-8 from verifier-cs0.md.
- B1: check 1, findings 1-7 closed (finding 1: seven pointers; finding 5: >= 6 renumbered citations).
- B2: check 2, run the six commands (read-only).
- B3: check 3, ADR 0005 rows PM-0005-47..56 vs adr0005-marks.draft.md section 1; map each difference to a finding.
- B4: check 4, docs-keeper's lines in STATUS, ROADMAP, REPO_MAP vs DECISIONS, commits, files; STATUS 120 lines.
- B5: check 5, the report's claims vs records; size recount (numstat 4f20643).
- B6: check 6, CR and >120-column lines (characters) in lines added after cacea55, outside tables.
- B7: answer drafted here, then DONE.

## Notes
### B0 state — done
- HEAD 47546e9 (branch claude/wonderful-hamilton-jdmlnf); base c94c81b = origin/main (local main stale at 44446a5: pass c94c81b explicitly). Docs commits after cacea55: f61f43c, 94aaae8, 4f20643, d6b6b30, a65d26f (the rest are checkpoint commits). Diff cacea55..HEAD -- docs: 12 files, +187 -95.
- First pass's findings (notes F-ids -> caller's numbers, inferred from the brief: 1 = pointers, 5 = renumbered citations, 8 = already closed; Mediums first, then Lows in note order): 1 = F2 (Medium, pointers drop the redo sentence); 2 = F6 (Medium, struck "(never a plain tube)" left out of "Since the plan"); 3 = F1 (Low, pointers-insert --check exit 1 on old long lines); 4 = F3 (Low, SPIKE_PLAN edits not in the plan's file table: name them in the report/PR file list); 5 = F4 (Low, stale SP/0005 line citations in asset-export-rules.md); 6 = F7 (Low, two process statements without times); 7 = F8 (Low, "about 900 / about 1,700" sizes unmeasured); 8 = F5 (Low, rifle marks; fixed in 94aaae8).
- Next: B1.
### B1 check 1 (findings closed) — in progress
- Finding 1 (F2) CLOSED: the rule is quoted in exactly 7 pointers (DECISIONS :536, :1063, :1270; SPIKE_PLAN :110, :167, :497; ADR 0002 :1461; DECISIONS :473 is the entry itself). Word-diff cacea55..HEAD on the three files: only `met."` -> `met. If a mark fails, the code that rests on it is redone."`, x7; rest is rewrap. Each pointer's quote (726 chars, "Foundation steps 2.4 and 2.5" .. "is redone.") is contiguous in his rule at DECISIONS:473. No pointer elsewhere in docs quotes the rule without it.
- Finding 5 (F4) CLOSED for the line numbers: my pairing script (scratchpad cite-pair.mjs) pairs all 51 0005/SP/0003 citations cacea55 vs HEAD: 40 unmoved (first and last line text identical c94c81b vs HEAD), 11 moved, each moved one's first line equal to its base line (SP:537-542 -> 576-581 ends inside the 3-line pointer that replaced old 542; the cited Blender route is 576-578). Read at HEAD: 0005:1874-1899 (AS3 .. AS4 end), 0005:1886 (AS4 question), 0005:1895-1896 ("whether the glTF export keeps material names (not researched)"), 0005:2334 (PM-0005-36; 2311 is now PM-0005-15), SP:568-586 (asset research, "verified like R3" :574), SP:568-573 (AS1, AS2, AS5, AP1), SP:576-578 (Blender route), SP:416-417 (plain exports, no compression or LOD) = 8 read, all right. STATUS:76 (cited at AER:57) unchanged c94c81b/cacea55/HEAD.
- NEW N1 (Low): docs/research/asset-export-rules.md:263-264 (AX-R16's bracket) reads "SP:416-417, 5:1715" and "(0003:306, 3:797)": renumber-cites.py's PAT list continuation `(?:, \d{3,4}...)*` swallowed the next key "0005"/"0003" as a line number and rewrote it as int -> "5"/"3" (d6b6b30; cacea55 had "0005:1715", "0003:797"). Targets right (0005:1715 look-test plain exports; 0003:797 GA1-GA15 port). Only these two (grep for single-digit keys). Fix: "0005:1715", "0003:797".
- Finding 2 (F6) fixed as asked, but the fix carries a wrong block: plan report :70-74 now quotes his struck-line sentence exactly (= appendix A3 :108, his message of 19:10:44 UTC, and DECISIONS :243), names Part 4's line (report :824, list B "The guns" item 2: "(never a plain tube)") and says it predates him (plan 68addf9, 2026-11-14 15:26 UTC). BUT it says the second copy is "the sheet notes in the appendix (A7)": A7 (appendix :297-369) holds no "tube", "cylinder" or "silhouette"; the line is appendix :215, inside A5 (:195-234, the Art lead's ART_BRIEF_ADDITION_2026-11-15_poses_yard.md, A3 HAT AND RIFLE: "the look brief asks for "real silhouettes, no plain tubes or cylinders""). The first pass's own note (F6) mislabelled :215 as A7, and the fix copied it.
- NEW N2 (Medium): docs/reports/2026-11-17-build-plan-slice-1.md:72-73 "the sheet notes in the appendix (A7)" -> "the Art lead's addition of 2026-11-15 in the appendix (A5, its part A3)"; it was written 2026-11-15, so "written before his" still holds. Reach: a session reading A5's line is not told it is one of the two copies. Nit with it: ":824 is list B's item 2; "item 2 of its list" could be read as A2" -> "item B2".
- Finding 3 (F1) CLOSED in the script (diff cacea55..HEAD of pointers-insert.mjs: --check now counts each pointer word for word as built, and lines over 120 only in the pointers' own text); exit code in B2.
- Finding 4 (F3) CLOSED: cs0 report :26-27 "SPIKE_PLAN: his answers and dated pointers (...; SPIKE_PLAN's asset-spike lines name the new marks)" (the LOOK_V2 pointer :581-583 is one of the dated pointers). PR body not checked.
- Finding 6 (F7) CLOSED: plan report :46-47 "at 20:46 UTC (... approved by the advisor agent at 20:39 UTC" and :101 "at 20:25 UTC" = first pass's records 20:46:03Z (L1995), 20:39:12Z (L1937), 20:25:35Z (L1867).
- Finding 7 (F8) CLOSED in form: plan report :99-101 "about 1,200 lines, besides 1,788 copied ones (the measured count is in this change-set's report)"; the cs0 report :38-39 gives 1,200 + 6 and 1,788 but names no commit (asked: "the commit they were counted at"); numbers checked in B5.
- Finding 8 (F5) CLOSED (94aaae8): plan report :138 "marks 1, 2, 5, 7, 8 and parts of 3 and 9".
### B1 — done. Next: B2 (six commands).
### B2 check 2 (six commands at HEAD 47546e9, read-only; docs untouched after: git status shows only .checkpoints) — done
- decisions-insert --check: exit 0 (5 entries equal in DECISIONS and appendix; header lines >120: 0; A10 3,244-byte copy: yes; CR: none).
- pointers-insert --check: exit 0 (DECISIONS 6 ok, word for word 5 of 5; SPIKE_PLAN 3 ok, 3 of 3; ADR 0002 1 ok, 1 of 1; their lines >120: 0) -> finding 3 (F1) closed.
- appendix-verify HEAD: exit 0 (11 blocks, 0 mismatches, CR none).
- report-build --check: exit 0 (plan report 1,423 lines; below the mark = 68addf9; sha256 b9ea0e5bf681b61d, 119,696 B).
- gate --no-smoke: GATE PASS, base c94c81b (origin/main): [1] 5/5, [2] PASS both pages, [3] core 138/game 714/editor 353, +0 -0, dup 0, [4] 51 refs 51 guarded 0 flagged, [5a] SKIP, [5b] 0 problems, [5c] 0 refs, [5d] PASS. Symbol diff +[] -[] = plan (docs only).
- repo-map-check: "repo map OK: 127 entries cover 109 required paths", exit 0.
- Next: B3.
### B3 check 3 (ADR 0005 rows vs adr0005-marks.draft.md section 1) — done
- The draft file is unchanged since c8594b6; its 11 rows (group row + PM-0005-47..56) equal cacea55's rows byte for byte (cmp). Word diff draft -> HEAD (ADR 0005:2344-2354): group row and -49, -50, -52, -54 unchanged; 9 differences in 6 rows:
  1. -47 bites + "a wrong magic; the BIN chunk first" = test-engineer L1 (f61f43c).
  2. -48 bites + "a slot `paint_coat` in entry and file" = TE L1.
  3. -51 bites "an `animations` entry, ... a `COLOR_0` attribute" -> "one copy per listed key, and ..." = TE L1; the row's list (images, animations, cameras, targets, COLOR_0, KHR_lights_punctual) covers both removed bites: superset.
  4. -53 bites + "`socket_grip` moved 2 mm; `socket_muzzle` moved 2 mm off x = 0" = TE M2.
  5. -55 number: "for the skinned figure under a root scaled 1.2, ... where the turn and the scale put the hand" = TE M1 (fix words verbatim).
  6. -55 bites + "`hips` hung beside the root in one run" = TE M1's reasoning ("existing bites still fail it (skin off; line 6's hips-beside-root ...)"), not in its fix words; tighter (a bite added). Note, not a finding.
  7. -56 line 6 "within 1 mm" -> "within 0.5 mm" = TE N1 ("within 0.5 mm for line 6 satisfies both readings").
  8. -56 basis + "0.5 mm in line 6, which holds whether the advisor agent's 1 mm is read flat or times the scale;" = engine-architect L4 (94aaae8; "line 6 now 0.5 mm but the row's proposed: basis names only 1 mm").
  9. -56 bites + "upside down (turned 180 degrees about z);" = TE L1.
- Also f61f43c: bullet (b) "a copy of the passing file, entry or test set-up" = TE L2 (outside the rows). No number loosened, no bite removed without a superset: no row is looser. AER part 5's table carries the same changes.
- Next: B4.
