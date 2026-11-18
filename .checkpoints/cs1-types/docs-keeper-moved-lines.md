docs-keeper-cs1: the lines moved by change-set 1's docs pass (PR #58), under Mustafa's OLD-LINES of 2026-11-18
(11:08:25 UTC, docs/DECISIONS.md): "A docs pass of either lane may move those lines by script, byte for byte, never
reworded, also when another lane wrote them. Its report lists each line it moved."

The script: `.checkpoints/cs1-types/docs-keeper-move.mjs` (beside this file). It checks every anchor line first and
refuses on any mismatch, copies lines as they are, checks each block byte for byte at its new place and prints a
sha256 per block at both places. Run on the base main 22c6afa (all four record files equal on main, HEAD daf3d98 and
the working tree: blobs in docs-keeper-notes.md). Its output: 9 checks ok; sha256/16 equal at both places.

| # | File, old line(s) | New place (line numbers right after the script) | First words | Who wrote it (blame) | sha256/16 |
|---|---|---|---|---|---|
| 1 | docs/STATUS.md:29 | docs/STATUS_ARCHIVE.md:159, "In flight" (first row, newest first) | `\| #57 \| Part 15 of the build plan, "The second lane", to main` | this lane: d5529ca (#57) | a993c8c2dffbd4e4 |
| 2 | docs/STATUS.md:113-120 (8 lines) | docs/STATUS_ARCHIVE.md:1275-1282, a new last section after "Health" | `## Where STATUS's old sections went (STATUS-DIET, 2026-11-10;` and its table of 7 lines | 3f98c0b (#52, merged 4460ef5) | 322749469aa9df50 |
| 3 | docs/ROADMAP.md:257 | docs/ROADMAP.md:90, "Build history", after #42's folded entry | `**The VFX editor and private-match XP [2026-11-07, MERGED]**` | 3f98c0b (#52) | 1acba721c50a0f66 |
| 4 | docs/ROADMAP.md:241-255 (15 lines) | docs/ROADMAP.md:92-106, "Build history", after #43's | `**Kit v3.4, the review policy [2026-11-07 to 11-09, MERGED]**` | 3f98c0b (#52) | 38b5aefb3c479c7c |

Why each is a line about a merged pull request:
1. #57's row: #57 merged as 22c6afa (2026-11-18, 11:09:31 UTC). This lane's own line: after the move its cells were
   edited into the archive's four-column form (merged state), as #57's pass did for #56's row. Listed here although
   OLD-LINES is not needed for an own line.
2. The table records where status-diet (PR #49, merged as bb2d817 on 2026-11-11) moved STATUS's old sections. STATUS
   keeps a one-line pointer of this lane's where it stood, so STATUS:85's "one hop through the table below" (#52's
   line, unchanged) still leads to it. In the archive a line of this lane's after the table says it was moved and that
   its "this file" means `docs/STATUS.md`.
3. vfx-xp-notes, PR #43, merged as 69ea88e (2026-11-07).
4. kit-v3-4, PR #44, merged as 44446a5 (2026-11-09).

Separators: blank lines only. Removed: ROADMAP:240 and :256 (between the entries). Added: ROADMAP:89, :91 and :107
(around the two folded entries, so they render as their own paragraphs between the list's items; no "- " was put in
front of them, which would change their bytes) and STATUS_ARCHIVE:1274 (before the moved table).

Check run after the script (a moved line must reappear byte for byte; result: removed non-blank 25, added non-blank
25, the two sorted lists equal; blank lines removed 2, added 4):
  git diff -U0 -- docs/STATUS.md docs/STATUS_ARCHIVE.md docs/ROADMAP.md | node -e '<collect "-" and "+" lines, drop
  blanks, sort, compare>'   (the full one-liner is in docs-keeper-notes.md)

Not moved, though about merged pull requests: STATUS's open follow-up ids from #49, #50, #51 and #48 (STATUS:83-98 on
main) stay, since docs-keeper keeps open follow-ups by id in STATUS; ROADMAP's #46 entry and newer stay (12 entries
after #58's).
