docs-keeper-khc: the lines moved by the kit hold's docs pass (PR #60), under Mustafa's OLD-LINES of 2026-11-18
(11:08:25 UTC, docs/DECISIONS.md): "A docs pass of either lane may move those lines by script, byte for byte, never
reworded, also when another lane wrote them. Its report lists each line it moved."

The script: `.checkpoints/kit-hold-ci/docs-keeper-move.mjs` (beside this file; adapted from change-set 1's,
`git show 76df3f3:.checkpoints/cs1-types/docs-keeper-move.mjs`). It checks every anchor line first and refuses on any
mismatch, copies lines as they are, checks each block byte for byte at its new place and prints a sha256 per block at
both places. Run on base main 9b4b518 (the four record files equal on main, HEAD and the working tree: blobs in
docs-keeper-notes.md). Its output: 10 checks ok; sha256/16 equal at both places. Line numbers below are right after
the script; the Edit-tool changes that followed shift some of them (the last column gives each block's first words).

| # | File, old line(s) | New place | First words | Who wrote it (blame on main) | sha256/16 |
|---|---|---|---|---|---|
| 1 | docs/STATUS.md:29 | docs/STATUS_ARCHIVE.md:159, "In flight" (first row, newest first) | `\| #58 \| change-set 1 of the build plan, Types (foundation step 2.4), by lane 1:` | this lane: 95b67cc (#58) | b443cfad50c2a06f |
| 2 | docs/ROADMAP.md:259-270 (12 lines) | docs/ROADMAP.md:108-119, "Build history", after #44's folded entry | `**Kit v3.5, the guard's rename gap [2026-11-09 to 11-10, MERGED]**` | 3f98c0b (#52, merged 4460ef5) | b2f030bd391e2389 |

Why each is a line about a merged pull request:
1. #58's row: #58 merged as 9b4b518 (2026-11-18, 15:39:28 UTC, the merge commit's date; `merged_at` 15:39:29). This
   lane's own line: after the move its cells were edited into the archive's four-column form (merged state), as
   change-set 1's pass did for #57's row. Listed here although OLD-LINES is not needed for an own line.
2. kit-v3-5, PR #46, merged as 2d22b69 (2026-11-10). ROADMAP's "Recent work" would hold 13 entries with #60's;
   docs-keeper keeps 8 to 12, so the oldest folds into "Build history", in date order after #44's.

Separators: blank lines only. Removed: ROADMAP:258 (between #49's and #46's entries). Added: ROADMAP:107 (before the
folded #46 entry, so it renders as its own paragraph after #44's; no "- " was put in front of it, which would change
its bytes).

Not a move, a copy (the source stays as it is): `docs/reports/2026-11-18-cs1-types.md`, "Follow-ups", its lines
234-268 (35 lines), copied by the same script into `docs/STATUS_ARCHIVE.md`, live "Follow-ups", lines 931-965 right
after the script, each line indented by two spaces so it nests under a header line of this pass (written after,
with the Edit tool). Check: the copy with the two spaces removed equals the report's lines (sha256/16
afdd299adf534856 at both places).

Check run after the script (a moved line must reappear byte for byte; result: removed non-blank 13, added non-blank
48, of which 35 are the copy; the 13 left equal the 13 removed, sorted; blank lines removed 1, added 1):
  git diff -U0 -- docs/STATUS.md docs/STATUS_ARCHIVE.md docs/ROADMAP.md | node -e '<collect "-" and "+" lines, drop
  blanks, take the 35 copied lines out of the added ones, sort, compare>'   (the one-liner: docs-keeper-notes.md)

Not moved, though about merged pull requests: STATUS's open follow-up ids from #46, #49, #50, #51 and #48 (STATUS:71-103
on main) stay, since docs-keeper keeps open follow-ups by id in STATUS; ROADMAP's #49 entry and newer stay (12 entries
with #60's).
