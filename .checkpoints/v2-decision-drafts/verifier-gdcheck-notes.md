# verifier-gdcheck notes

## Round 1 (touchup2-game-designer-fixes.md: F1b, LOST-AB1-WHY, F10, F9, F8)
- 2026-11-03 start. HEAD cda0577 (docs/ identical to 41665c7: only the report appendix changed since). ADR 0005 on disk 2499 lines. Next: dry-run.
- Step 1 dry-run done: all 5 BEFOREs occur once (on-disk lines F9 869-876, AB1-WHY 882-883, F1b 900-901, F10 948-949, F8 1297-1300); applied copy at verifier-gdcheck-scratch/tree/docs/adr/0005 (2501 lines), identical to author's applied copy. Step 2: section bounds 746..1432; all 5 inside. Next: step 3 per block.
- Step 3: F1b, LOST-AB1-WHY, F10 rows written (all hold; byte-identical to 47eadd9 where restored). Next: F9 (LEGACY_TUNING), F8 (ADR 0001 part 12, LOOP_V2 §4).
