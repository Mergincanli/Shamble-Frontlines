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
