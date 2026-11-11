# docs-keeper, foundation 2.1b step 10 (the docs pass): notes

Brief: `.checkpoints/foundation-restructure/docs-keeper-brief.md`. Edit only docs/ with the Edit tool; no commit, no push.

## Plan
1. Read the targets by text: REPO_MAP, START_HERE, MODULE_MAP, TECHNICAL, STATUS, STATUS_ARCHIVE, ROADMAP, ADR 0002
   (:7, OQ-0002-2 entry, the two R5 rows), the register (ADR 0002 row, status sentence, loose-ends tables).
2. Grep ADR 0002 for every OQ-0002-2; grep docs/ for lines misstating ADR 0002's status or where the legacy files live:
   list as "stale, left for the report", unedited.
3. Edits 1-9 of the brief, one note line each.
4. Run `node .claude/scripts/repo-map-check.mjs` and `node .gate/foundation-restructure/docs-check.mjs`; fix within the list.
5. Answer: files changed, stale lines, last lines of both checks, what was not done.

Check constraints seen in docs-check.mjs: quoted spans of 20+ chars in "..." on added lines must be his words or a docs
line at main; START_HERE's added `legacy` commands must be E6's strings; the register's PM-0002-36 row is one pure
added line inside the loose-ends section; ADR 0002 hunks only on :7, the OQ-0002-2 entry and the two rows.

## Log
- Read all targets. Verified: c090843 = five R100 renames, blobs equal at 76b4f7a, HEAD (legacy/) and 44446a5.
  PR #50: merged_at 2026-11-11T18:38:28Z, merge commit 76b4f7a (parents bb2d817, f6becf5); main CI run 43079707687
  (#671, gate workflow, success, push). PR #51: open, draft, head c5d4872 at read time.
- ADR 0002 OQ-0002-2 mentions: :150, :563, :998 (entry), :1358, :1431, :1564, :1589, :1596 (row), :1626 (row).
- Risk seen: docs-check check 6 matches placeholder commands (`... legacy eval <snippet.js>`, `... legacy <cmd>`,
  `pageshot.js legacy index.html <prefix> [--headed]`, `perfbots.js --map=tri --pre=<file.js>`) against E6's literal
  strings: likely false FAILs. ROADMAP Recent work would reach 13 (fold = history, on the Never list).
- Register: ADR 0002 row status column (+ the SCOPE (a) words, "8 open" kept); status sentence (main :33 → two lines, :34 byte-same); one row "Q-A1: how PM-0002-36's mark is read" appended to "The critic's remaining items" (Code cell avoids a leading PM- for check 1).
- ADR 0002: :7 acceptance words (exact brief string); OQ-0002-2 answer appended after its Research ids line (three sub-bullets, pointer to the 2.1b report and DECISIONS 21:39:44 UTC); rows R5-LAYOUT-OQ5 and R5-GATE-OQ3 → "answered in part: OQ-0002-2". Line 3 untouched.
- START_HERE: legacy banner (new first blockquote, one line); :12 double-click names legacy/index.html; :22-24 files in legacy/ [verified]; :105, :115-117 folder `.` → `legacy` (plain substitution: check 6 core rule passes; README's `index.html[?map=tri]` form would not); :113 folder-argument sentence with [verified] tag.
- REPO_MAP: five root rows → one `legacy/` row (file descriptions kept, the file:line rule); :6 date 2026-11-11 only (:5 untouched). repo-map-check: OK 112 entries / 99 paths, exit 0.
- MODULE_MAP: one line above the file table (all five in legacy/, [verified]). TECHNICAL §9: the two inventory lines name legacy/ [verified].
