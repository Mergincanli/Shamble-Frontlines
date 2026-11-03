# stop-game-designer notes

- 2026-11-03: started; read inputs, verdict r4, map rows C9 A1-A3 B1-B2 LA3, plan r4, my section (746-1434). Docs unchanged since 4374a1c. Next: compose blocks in file order (A2 796-798, C9 note, A3/B2 1166-1171, A1 1332-1340, B1 1341-1348, LA3 note).
- A2 done (FIX A2, 796-798). Next: NOTE C9, NOTE C9-2 (kinds), FIX A3, FIX B2, FIX A1, FIX B1, NOTE LA3.
- NOTE C9 and NOTE C9-2 (ability kinds confirmed: weapon, unit, vehicle, faction present; abilities and tracks join once designed). Next: FIX A3, FIX B2, FIX A1, FIX B1, NOTE LA3.
- FIX A3 (1166-1167), FIX B2 (1171) done. Next: FIX A1 (1339-1340), FIX B1 (1347-1348), NOTE LA3; then dry run, glossary, cells, lengths.
- FIX A1 (1339-1340), FIX B1 (1347-1348), NOTE LA3 done. All 7 rows have blocks (5 FIX: A2 A3 B2 A1 B1; 3 NOTE: C9 C9-2 LA3). Next: dry run, glossary, cells, lengths.
- Checks (2026-11-03): apply_fixes.py --dry-run on a pristine copy at stop-game-designer-scratch/base/docs/adr/0005-...
  (identical to disk, cmp): 5/5 blocks ok, all applied (dryrun.out). Applied copy: diff in applied.diff (+14 lines).
  Glossary (glossary_check_drafts.py pointed at a scratch tree, --skip-records): exact 0 before, 0 after; in context
  93 before, 93 after (hits-base.tsv, hits-tree.tsv; only line shifts). Tables: 0 cell mismatches, 0 prose lines
  under a table (no table row touched). Lines over 120: 0 in the AFTER texts and 0 in the fixes file. Doc copies
  removed from scratch after the run (reproducible). Blocks: FIX A2, A3, B2, A1, B1 (5); NOTE C9, C9-2, LA3 (3).
- Open for the report (not in the doc, by "record only what the words say"): which specific GAS claims ADR 0005's
  ability section relies on for AB12's later check is not listed; the clearest are R1-ABL-F4 (default aggregation,
  AB-1), R1-ABL-F9 (execution code not predicted, AB-2) and R1-ABL-F17 (Fortnite's and Paragon's weapon bookkeeping,
  AB-3 and option B1).
DONE
