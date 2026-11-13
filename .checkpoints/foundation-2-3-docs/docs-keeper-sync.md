Task: docs-keeper sync for step 2.3's first docs pass (PR #52): STATUS, ROADMAP, STATUS_ARCHIVE (2 edits), REPO_MAP row, for main 4d21c1a.

Plan (blocks, in order):
B1 read plan.md §6 + STATUS.md + relevant archive/roadmap/repo-map sections
B2 fetch PR #40 checkpoint (read only)
B3 edit STATUS.md (header, phase, in flight, next, waiting, follow-ups); count lines <=120
B4 edit ROADMAP.md (#48 entry, new #52 entry, count entries, Next/open line)
B5 edit STATUS_ARCHIVE.md (2 edits)
B6 edit REPO_MAP.md row + run repo-map-check
B7 self-check diff; DONE

Notes:
- B1 done. STATUS = 120 lines at start (must stay <=120). ROADMAP Recent work = 12 entries -> adding #52 makes 13 -> fold oldest (B3 design walk, :205) into Build history after TEST-RUN line (:80). Archive: #48 row goes above 2.1b row (:159); "From foundation 2.2" at :553. git: 4d21c1a parents 2aa5e9c and 9b20b67, commit dated 16:20:10 UTC (merged_at 16:20:12 per brief). DECISIONS entries for 2026-11-13 present at :110, :159, :165. ROADMAP: no placeholder (session adds pointer by script after 2026-11-11 sentence).
- B2 done. git fetch OK at 2026-11-13T20:29:42Z: FETCH_HEAD 0066b635b65b4016c91af88825120312776a0a63 (committed 19:32:51 UTC). Its checkpoint (other lane's data): NOW 19:35 UTC: U4d2 merged (cc8595d) and closed; L6 launched, the U4r builder running (brief 2fbec77); main 4d21c1a merged in as 7685b2b (CI green on it, per their card). Row state to write: draft; U1-U3 done; U4: U4d2 merged, U4r's builder running; main 4d21c1a merged in (7685b2b); as read at 0066b63, 2026-11-13 20:29 UTC.
- Advisor consulted (before writing). Findings: pointers.mjs finds ROADMAP's P7 by text anchor (fold-safe); fills {{P-2.3}} with 4 lines (indent 3) and {{Q-KIT}} with 3 lines (indent 2), wrap 120, no 120-line assert -> filled STATUS = placeholder count + 5. Target <=115 with placeholders. Scratch fillcount.mjs deleted (it held a retyped copy of his words for counting).
- Finding (report, not fix): archive :607 "is to come with PR #48's merge and is still waiting" is stale after the merge; outside my two edits.
- Finding: no run ids for #48's head runs in brief -> omit from archive row.
- B3 in progress: header, Last merged, kit lead-in + {{Q-KIT}}, main line, #40 row, #52 row (dk-row52.mjs), Next item 2 + {{P-2.3}}, Waiting #52 line, follow-ups 2.2 block (dk-fu22.mjs) all written. Now trimming: STATUS at 123 (with placeholders); line 44 is 121 chars.
- Finding (report): the saver's autosave e6fc6a9 committed scratch fillcount.mjs (my retyped copy of his four sentences and kit words, for line counting only) before I deleted it; deleted in the working tree now; the next autosave records the delete. Not in any doc.
- B3 done. STATUS = 120 lines with the two placeholders (Q-KIT at :20, indent 2; P-2.3 at :38, indent 3); after pointers.mjs fills them: 125 (P8 4 lines, Q-KIT 3 lines). Cuts made inside edited paragraphs: Last merged pointer shortened to "Older: the archive; ROADMAP."; Next item 2's SPIKE_PLAN spike list condensed to "(SPIKE_PLAN)"; #48's 2 Waiting lines -> 1 for #52; the four accepted ids moved out of the 2.2 follow-up list into one "Accepted as left open (his go for #48, item 5)" sentence. Protected lines (legacy line, now :21; Windows run line, now :45) and Routines untouched (verify in B7). Next: B4 ROADMAP.
- B4 done. ROADMAP: v2 line (Next/open) gained "(PR #40); the foundation: 2.1a, 2.1b and 2.2 merged (PRs #50, #51, #48), step 2.3 planned after its first docs pass (PR #52) merges;" before "wave 1", 2026-11-11 pointer byte-intact (P7 anchor is found by text; fold shifts lines: Next/open v2 line now :97). New #52 entry at top of Recent work; #48 header MERGED + merge facts sentence; B3 design walk entry folded into Build history (:81-84, after TEST-RUN); Recent work = 12. Next: B5 archive.
- B5 done. Archive: #48 row inserted at :159 (above 2.1b), 2.2 block opening at :554 changed (line now 140 chars, not re-wrapped). Next: B6 REPO_MAP.
- B6 done. REPO_MAP: no 2026-11-12 report row exists (the map lists only four 10-31/11-01 reports plus the docs/reports/ folder row); added the new report's row after the last report row (:76, after 2026-11-01 appendix). Check: 'repo map OK: 121 entries cover 107 required paths', exit 0. Next: B7 self-check.
- B7 checks: diff --stat = 4 docs files (REPO_MAP +1; ROADMAP +17 -6; STATUS +21 -21; ARCHIVE +2 -1); archive 2 hunks (:159 insert, :554 change); Routines section identical to HEAD; HEAD's STATUS :21 and :46 present byte-identical (now :21 and :45); {{Q-KIT}} once at :20 (2 spaces), {{P-2.3}} once at :38 (3 spaces); no placeholder in ROADMAP; STATUS wc -l 120. No new quoted span of his words added in any file.
- Advisor consulted (before done): fixes applied: #52 row State -> "draft"; Last merged pointer -> "Every merge: ROADMAP." (archive pointer dropped, a cut). Id check on the 2.2 follow-ups bullet, HEAD vs now: 34 hyphenated ids + G0, identical. STATUS still 120.

## Answer material
Hunks (git diff -U0 HEAD):
- STATUS: :5 Last updated; :13-14 (was 13-16) Last merged = #48 + "Every merge: ROADMAP."; :16 kit bullet opening "**The kit** (KIT-FREEZE-2, ..." (no "frozen"); :19-20 (was 20) lead-in "His words of 2026-11-13 for step 2.3 (`docs/DECISIONS.md`):" + "  {{Q-KIT}}"; :22-24 (was 22-23) main: gate+smoke required, toolchain on every push/PR not required until 2.3 (REQUIRED-CHECK), CI green 4d21c1a (gate run 43461710700, toolchain run 43461710673); :29-30 #40 row state + #52 row (replaces #48's); :36-39 (was 35-39) Next item 2: foundation state, step 2.3 lead-in + "   {{P-2.3}}", wave 1 condensed, old 2026-11-10 pointer sentence gone; :42 (was 42-43) Waiting: his go for #52; :95-99 (was 96-99) 2.2 follow-ups: four accepted ids moved into "Accepted as left open (his go for #48, item 5): ..." sentence.
- ROADMAP: :81-84 new Build history line (B3 design walk folded); :97 (was 93) v2 line clause before "wave 1"; :117-125 (was 113) new #52 entry + #48 header MERGED; :138-140 (was 126-127) #48 merge sentence; :216-217 removed (old B3 entry + blank).
- ARCHIVE: :159 new #48 row; :554 block opening "merged as 4d21c1a on 2026-11-13".
- REPO_MAP: :76 new report row.
Facts/sources: git log 4d21c1a (parents 2aa5e9c, 9b20b67; committed 16:20:10 UTC; message: go 16:16:47 UTC, session_019YNMUSc7FA68ujhLgXfEJA, APPROVE of 9b20b67); toolchain.yml `on: push, pull_request`; DECISIONS.md :110 and :165 entries; git fetch of #40 branch 20:29:42Z, FETCH_HEAD 0066b63 (19:32:51 UTC), its checkpoint NOW 19:35 UTC; report file exists with Appendix A1-A3; 2.2 report E6 symbols +0 -0; session checkpoint (steps 1-3 to 2f96f5c).
From brief, not re-checked: merged_at 16:20:12; run ids 43461710700/43461710673 and job ids; #40 draft state (API); #52 contents and waiting list; risk tier.
Overflow: STATUS 120 with placeholders, 125 after pointers.mjs (P8 4 lines, Q-KIT 3 lines; no cap assert).
Report-not-fix: REPO_MAP premise (no 2026-11-12 row; row put after 2026-11-01 appendix; "Last checked: 2026-11-12" left); archive :608 stale "still waiting"; STATUS cuts (SPIKE_PLAN list, SLICE-ORDER date, archive pointer in Last merged); fillcount.mjs in autosave e6fc6a9; only quoted span added: "bears on" (carried from folded entry).
Repo-map check: "repo map OK: 121 entries cover 107 required paths", exit 0.
DONE
