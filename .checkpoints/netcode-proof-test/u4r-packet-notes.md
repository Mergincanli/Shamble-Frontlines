# U4r packet config: notes

## Plan
1. Read review_packet.py (rules, FORBIDDEN, ALLOWED, words_of_mustafa) and u4d2-packet.json (form).
2. Check the facts: snap1's git status (26 files), HEADs, patch sha256, counts in c5-end1/2 logs.
3. Plan sections: §5 U4r entry, §4.1, the §4/§7 sections the brief's sections 2 and 7 cite.
4. marks.json parts (-17, -57, -18, -30, PM-0001-2) by line range; SERIES.md rows; catalogue class section r4.
5. The four rulings, whole. advisor-u4r-brief.md is FORBIDDEN and not ALLOWED: a question, not an edit.
6. Carries include regex: §U4r entries plus 62, 73-78, 82-85, 87, 89 and others the plan's U4r entry names.
7. Write u4r-packet.json; check it parses; check every path exists; check headings match exactly once.

## Progress
- Script read: FORBIDDEN is checked on "docs" paths only; ALLOWED holds advisor-u4e-brief.md and advisor-u4d2-brief.md,
  not advisor-u4r-brief.md (it matches `-brief[^/]*\.md$`): a question, no edit. advisor-u4r-build*-full.md pass.
- Heads checked: snap1 status 26 entries (13 M, 13 ??); worktree at 2fbec77 with the same 26 under spikes/ (27th entry
  is the builder's notes, outside spikes/); cmp of the 26 files snap1 vs worktree: all equal; main folder HEAD b084998,
  `git diff --stat 2fbec77 HEAD -- spikes/` and `git status -- spikes/` empty; 614823c..2fbec77 touches 2 .checkpoints
  files only; 4e91830..614823c spikes/ equal; patch sha256 572c4f69... ok.
- Counts: .gate/l3-manifests/counted.log (HEAD 4e91830): 11 steps, 491, 561, 71 of 71. c5-end1.log and c5-end2.log
  (HEAD 2fbec77, worktree): all 11 steps pass, 526, 603, 76 of 76, exit 0 each.
- Plan sections cited by brief sections 2 and 7: §4.1, 4.4, 4.5, 4.6, 4.7, 4.9, 4.11, 4.12, 4.13, 4.14, 4.15, 7.1, 7.2;
  plan.md §4.7's npt-lab lines 617-631 and plan.md:454. Left out: plan :169-170 (§3), :842-843 (§5 U4h), §2 :150.
- marks.json (main folder = base): PM-0001-2 151-624, -17 3844-4006, -57 4007-4105, -18 4261-4443, -30 6511-6577.
- SERIES.md: rows found by the patch's added-line string literals (npt-lab is the only writer of names; no series
  literal in room/server/client/bots sources), checked against the brief's R26 list; 18 blocks at base line numbers.
- Catalogue (equal to base): the class vocabulary, §3 Field groups, §6 What the class audit test asserts.
- Carries: §U4r ids 40, 49, 58, 59, 60, 62, 63, 69, 73-77, 82-85; rulings name 14, 28, 49, 57-60, 62, 73, 75, 77, 78,
  82-85, 87; task adds 74, 76, 89. `|U4r` adds 61, 66, 86, 88. Rulings 2-4's carry amendments (59, 62 (e), 73, 77 (b),
  78) are not in u4s-carries.md yet (only ruling 1's: 62 (h), 82, 87, 89): a question.
