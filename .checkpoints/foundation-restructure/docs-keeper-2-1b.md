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
- STATUS: Last updated (2.1b, main 76b4f7a); Phase SCOPE line "recorded in ADR 0002 by 2.1b (#51)"; Last merged → #50
  (76b4f7a, 18:38:28 UTC, merge commit, his go quoted in its message, APPROVE of f6becf5); CI green on 76b4f7a (run
  43079707687); In flight #50 row → #51 row; Next: #50 merged, #51 in flight; Waiting: his go for #51 + his report steps
  (pointer to his task message); Follow-ups: From 2.1b, the Node 24 finding (pointer, no quote); risk: R7 line numbers
  hold. 120 lines.
- STATUS_ARCHIVE: #50 merged row at the top of "In flight" (facts: GitHub API merged_at, merge commit 76b4f7a message, main run 43079707687); risk R7 item 3 (:843) corrected, [verified] (R100 x5, blobs equal).
- ROADMAP: 2.1b entry at the top of Recent work (#50 merge stated in it). Not done (brief Never: ROADMAP history / nothing wider): the 2.1a entry still reads "PR #50: in review"; Recent work now 13 entries, fold not done.
- START_HERE :116-118 rewrapped (the folder-argument sentence). Both checks: repo-map OK (112/99, exit 0); docs check PASS (exit 0).

## Stale, left for the report (unedited)
- docs/LEGACY_TUNING.md:9 `preview.js . eval tools/tuning-dump.js` (oracle file, Never list; regenerate via R7's builder).
- docs/ENGINE_OVERVIEW.md:4 and docs/SPIKE_PLAN.md:5 "not yet accepted" with no ADR 0002 part 1 exception.
- docs/V2_PHASE0_BRIEF.md:10 "It moves to `legacy/` in phase 1"; ADR 0001:1769 (same future tense).
- ADR 0002 :150 "is the phase 1 restructure plan's call (OQ-0002-2)" reads as pending (now answered in part).
- Register ADR 0002 row: open-question column lists no "-2 answered in part" (kept exactly: REG-TRUE (b), brief 9a).
- Register status sentence: after the inserted SCOPE (a) clause, "His words:" (SPIKE-ORDER (b)'s) may read as SCOPE (a)'s.
- ROADMAP: the 2.1a entry still "PR #50: in review" / "Next: the advisor's review and his go"; Recent work at 13 (fold
  into Build history not done); :3 banner names root-less file names.
- STATUS "From foundation 2.1a": "for 2.1b's plan, IE-2 ... SA-L1" (taken into 2.1b's plan check; state for the report).
- DECISIONS.md: no entry for his go for #50 (merge commit 76b4f7a's message: also his approval of dd12aeb's two gate edits
  and the five follow-ups he accepted). Script-only file.
- CLAUDE.md (outside scope): :47-:52 hard constraints (scope to legacy/ only; :48 names index.html), :53-:55 (the
  "Decisions pending" paragraph: ADR 0002 part 1 now accepted); :45 "index.html's live path"; :242 `<dir>` form (fine).
- START_HERE banner reworded: the docs-stay-in-docs/ decision now points at ADR 0002's OQ-0002-2 answer; KT21 cited for
  the redesign mark only (KT21's row holds START_HERE's own mark, not the docs location).
- Evidence read back: E1-E2 PASS, E3 bases PASS, E3 31/31 entries fail the same checks in old and head, E4 PASS (21/21),
  E5 SHARED, E6 14/14 pass, E8 GATE PASS (full and smoke-only). CI (GitHub API): c090843 runs #674/#675, 1e6f84e
  #676/#677, b6b3b81 #678/#679, all success.
- Stale add: STATUS_ARCHIVE "Fixed in dd12aeb ... on main once #50 merges" (main :865) is now past tense.
