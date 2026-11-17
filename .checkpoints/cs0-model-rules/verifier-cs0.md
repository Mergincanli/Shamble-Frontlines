Task: cs0-verifier (the verifier's pass on change-set 0, PR #56, branch claude/wonderful-hamilton-jdmlnf, base c94c81b, head cacea55)

## Plan (blocks, in order)
- B0 Orientation: base/head, files the change-set touches under docs/, working-tree state (docs-keeper's four files excluded).
- B1 (A) Run the four checkers: decisions-insert --check, appendix-verify HEAD, pointers-insert --check, report-build --check.
- B2 (A1) Re-extract his five messages from the transcript with my own script (line by line); sha256 vs the saved messages.
- B3 (A2) Entry headers' times and message ids vs the transcript records.
- B4 (A3) Read the entry headers, the six pointers in DECISIONS, the three in SPIKE_PLAN, the one in ADR 0002: more or less than his words?
- B5 (B) His answers in the drafts: OQ-0002-6, OQ-0006-18, OQ-0003-1 (in part), three "do soldiers block" rows, PM-0004-34; register arithmetic 98->96, 8->7, 17->16.
- B6 (C) The two research files: >=20 key claims spot-checked (>=6 in the joint list); [inferred] tags; nothing unproven stated as proven; agreement between the files and with ADR 0005 PM-0005-47..56.
- B7 (D) ADR 0005 rows vs test-engineer's file sections 1 and 3; section diff vs c94c81b; AS3/AS4 entries.
- B8 (E) Report: quotes-check on "Since the plan"; read for words beyond his; section 5 vs the research files.
- B9 (F) gate --no-smoke; repo-map-check; CR and .checkpoints citations in added docs; LOOK_V2's changed line; size-count and its "copied" rule.
- B10 Final answer from notes; DONE.

## Notes
### B0 Orientation — done
- Head cacea55 (cacea552d27f), branch claude/wonderful-hamilton-jdmlnf. Base c94c81b = origin/main (local `main` is stale at 44446a5; `git merge-base HEAD main` gives 44446a5, so every command here passes c94c81b explicitly).
- Change-set touches 13 files, all under docs/: DECISIONS (+243), LOOK_V2 (4), SPIKE_PLAN (49), adr 0002 (+16), 0004 (2), 0005 (105), 0006 (+3), adr/README (24), reports/...-appendix (+454, new), reports/2026-11-17-build-plan-slice-1.md (+1421, new), research/README (+2), research/asset-export-rules.md (+397, new), research/soldier-joint-list.md (+224, new). 2890 insertions, 54 deletions. No file outside docs/ (besides .checkpoints).
- Working tree: docs/STATUS.md modified (docs-keeper, not judged); .checkpoints scratch modified. Next: B1.
### B1 (A) the four checkers at HEAD — done
- decisions-insert --check: exit 0. 5 entries equal + appendix equal: answers-2133 1754 B bc1c0a9fc96b6182; notes-2058 2028 B dbc58ec975052e3a; go 10544 B dbc486cafd1a3568; addition 2340 B be95e64501c5c96d; task 7538 B cbe41d9e2bd7ec1f. Header lines >120: 0; A10 3,244-byte copy: yes; CR: none.
- appendix-verify HEAD: exit 0. 11 blocks, 0 mismatches, no CR (A1..A11; A10 3243 B d685d08bd487caa5).
- pointers-insert --check: EXIT 1. Counts ok (DECISIONS 6, SPIKE_PLAN 3, ADR 0002 1) but "lines over 120 near a pointer: 2" in SPIKE_PLAN: lines 592 (121 cols) and 605 (127 cols), both pre-existing (base lines 551/562, blame ^4460ef5), flagged only because the new PM-0005-47..56 sentence (596-598, 65c8fe8) contains "change-set 0 of the build plan". Checker false positive, not a doc defect.
- report-build --check: exit 0. 1421 lines; plan below the mark equals 68addf9:.checkpoints/build-plan-slice-1/plan.md; sha256 b9ea0e5bf681b61d, 119696 B.
- What the checkers do NOT prove (so later blocks must): pointers --check only counts the tag string; it never compares a pointer's text with what the script would write, so the quotes inside the pointers as committed are unchecked (B4). decisions-insert compares DECISIONS with the saved message files, not the saved files with the transcript (B2). quotes-check matches spans against his 6 messages AND the 4 Art texts together, so it cannot catch an Art-lead quote attributed to Mustafa (B8). size-count reads the working tree (docs-keeper's in-progress lines included) (B9).
- FINDING F1 (Low): pointers-insert.mjs --check exits 1 at HEAD on two pre-existing long lines (SPIKE_PLAN.md:592, :605); the line-length rule should look only at lines the change-set adds. Next: B2.
