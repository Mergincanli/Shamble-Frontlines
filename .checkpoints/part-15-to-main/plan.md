# Plan: Part 15, "The second lane", to main with Mustafa's answers (docs only)

Change-set after change-set 0, before change-set 1. Branch `claude/wonderful-hamilton-jdmlnf` (stands in for
`cs/part-15-second-lane`), base main d7dad97 (the merge of #56). Risk tier 1 (decision docs, his words): his go; no
Brave check.

## Goal, in his words (2026-11-17, 22:59:16 UTC; `mustafa-2026-11-17-2357-merge-56.md`)
"Part 15 with my six answers goes to main next, as a docs change of its own. Keep it to Part 15's text, my answers by
script, the pointers they need and the rules file's line on gun files. It also brings the status page and the roadmap
up to date for this merge." Then change-set 1.

## What is built, and how
| # | File | Change | By |
|---|---|---|---|
| 1 | `docs/reports/2026-11-17-build-plan-slice-1-part-15.md` (new) | a short section "His answers" (written: the six answers quoted, and what each adds to Part 15's text), then Part 15's text copied from commit ea851a8 and not edited, then his two messages in fences with their sha256 (P1: 22:10:34 UTC, his answers; P2: 22:59:16 UTC, his go on the merge of #56) | script |
| 2 | `docs/DECISIONS.md` | two entries, newest first, his words quoted by script: his answers to Part 15's six questions (with his rule on the stop line and on #56); his go on the merge of #56 (what he accepts; what comes after). Two dated pointers: at the entry of 2026-11-14, 13:58:05 UTC ("one pull request at a time" becomes one per lane, his answer LANE-SPLIT); at the entry of 2026-11-17, 19:10:44 UTC (the plan part his CHANGE 2 asks for is Part 15, now on main) | script |
| 3 | `docs/adr/0002-code-structure-build-toolchain.md` | one dated pointer at the package list: his answer VFX-ROUTE (a), "This is also my word on a seventh package beside the six the structure draft names." | exact edit |
| 4 | `docs/research/asset-export-rules.md` | rule AX-R1's line on gun files: "until he answers" becomes his answer GUN-FILES (a), quoted | exact edit |
| 5 | `docs/reports/2026-11-17-build-plan-slice-1.md` | one dated pointer in "Since the plan", where it says Part 15 goes to main once answered (the plan below the mark stays equal to commit 68addf9) | exact edit |
| 6 | `docs/STATUS.md`, `docs/ROADMAP.md`, `docs/REPO_MAP.md`, `docs/STATUS_ARCHIVE.md` | #56 merged (d7dad97, 2026-11-17, 23:00:12 UTC, on his go); the open items of #56's row into "Follow-ups"; Part 15 answered and on main with this change; next: change-set 1; older lines to the archive where STATUS's 120 lines need it | docs-keeper |
| 7 | `docs/reports/2026-11-17-part-15-second-lane.md` (new) | this change-set's report | main session |

Not in it: `docs/EDITOR_V2.md`'s pointer (Part 15, 15.5: it rides lane 2's docs change for the look carry); any kit,
gate or CI file; anything of change-set 1; the Shamble VFX digest (Part 15 cites it; it stays readable at commit
d2c4caa, `.checkpoints/part-15/shamble-vfx-digest.md`, and the report says so).

## "His answers": what the written section says (each with his words quoted, checked by script)
- LANE-SPLIT (a): his yes to the four conditions, to one pull request at a time per lane, and to the owners of 15.4.
- RECORD-FILES (a), with his addition: "It does not reword, re-wrap or move another lane's lines."
- VFX-ROUTE (a): a seventh package; "Nobody edits the copy by hand. If the copy cannot pass the game's checks as it
  is, that is a finding for the VFX chat."; "The re-copy script can also check: it fails when the copy differs from
  the pinned commit." This is stricter than Part 15's text, which said the copy "may need changes"; the section says
  so, and that 15.2's line on who owns the copied source reads with it: lane 2 re-copies, nobody edits.
- VFX-FIX (a): "The VFX chat fixes its repository and the copy here is renewed. I pass the findings on."
- VFX-MOMENTS (a): built in the game for Slice 1; "their numbers live in data files"; "Hit-stop is in no card of the
  plan and I add none now."
- GUN-FILES (a): "Every gun file that enters the game comes through Blender."
- What stays with him from Part 15's "For Mustafa": telling lane 2 which chat on his PC makes the model files;
  giving lane 2 Part 15 and asking for its list of needs from the block before change-set 4's plan is reviewed; the
  exports for the VFX proof.

## Questions for the advisor agent in this plan's review
1. His go on the merge of #56 is recorded in DECISIONS by this change-set (row 2), as the foundation steps recorded
   each merge go, and because it holds what he accepts with the merge (PM-0005-54 stands as a fail; the size as
   counted; the Advisor's comparison of A10: "equal"). Is that inside "Keep it to ..." read with "It also brings the
   status page and the roadmap up to date for this merge", or does it wait for change-set 1's docs pass?
2. Are the pointers of rows 2, 3 and 5 the ones his answers need, and is any missing (the register; SPIKE_PLAN; ADR
   0003's line on the three.js pin; ADR 0005)?
3. docs-keeper may edit `docs/STATUS_ARCHIVE.md` this time (its last pass could not move lines there): right?

## Evidence and reviews
- Scripts check: both DECISIONS entries equal his saved messages byte for byte; the fenced blocks' sha256; Part 15's
  text in the report equals commit ea851a8's byte for byte; every quote in the written section and in the pointers
  is in his saved messages; the plan in the build plan's report still equals commit 68addf9.
- `node tools/gate.mjs --no-smoke`, `node .claude/scripts/repo-map-check.mjs`; CI `gate`, `smoke`, `toolchain` on the
  final head.
- Reviewers on the final diff, each at effort max: the verifier (his words; every claim of the written section and
  of STATUS against the record), docs-keeper's pass, then the advisor agent on the final head. At most two rounds.
- Size: about 150 written lines (the written section about 40, entry headers and pointers about 30, the rules line
  3, STATUS and ROADMAP about 20, the report about 55); copied: Part 15's 257 lines and his two messages. Stop line:
  300 written lines; past it, stop and tell him in three lines.
