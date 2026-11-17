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
