# U4e merge text script: notes (agent, 2026-11-12)

Task: write .checkpoints/netcode-proof-test/u4e_merge_text.py (phases A and B, dry run only, never --apply), applying
the texts of U4e's merge commit as advisor-u4e-r1.md (R9, T1-T6) and advisor-u4e-r2.md (Q3, T1, T2 (3), T3, T5, T7)
rule them. I write only this file and the script.

## Plan
1. Read both rulings whole (done), both model scripts (done); hash the rulings and targets (done: the five target
   prefixes equal r2's spot-check list: README 0ad7a80f, SERIES 881d7fde, carries cdab0240, plan 2bef127b, build-plan
   3f58ff0d; catalogue 35a27979; r1 7c129309, r2 14b0b180).
2. Read each target at its anchor: README §1 (T1), SERIES.md (T4), catalogue :90 (T7), u4s-carries.md (entries 69,
   46, 70, 45 as section ends; 62 (h)), plan.md U6 row (T3), u4-build-plan.md §4.1 (T6), u4df-report.md (anchor form).
3. Write the script: sha256 asserts; block reader by T heading; old spans read from the rulings' inline code where the
   ruling marks them; phase A edits; phase B edits; word-for-word checks; width checks; unified diff in the dry run.
4. Dry-run phase A and phase B until every assert passes; record each text placed here.

## Placed (one line per text)
