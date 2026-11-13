# u4d2_merge_text.py: notes (the agent writing the script for U4d2's merge texts)

## Plan
1. Read the model script u4e_merge_text.py whole; the rulings' "## The texts" (r1 :548-649, r2 :313-467), r1 Block 9
   and r2 Block 5 (M8's rules); the target files at the main folder's head.
2. Hash the rulings (sha256sum) and the targets; find each old text / anchor in its target (count once).
3. Write u4d2_merge_text.py in the model's form: phase A = T4 (r1); phase B = T1 (r1), T2, T3, T5 (r2). Each code block
   read by its heading (the first fenced block after "### Tn."), the rulings' and targets' sha256 asserted, each old
   text found once, each new text compared byte for byte in place after, the diff's removed/added lines asserted to be
   exactly the ruled ones, no changed prose line over 120, the dry run printing the unified diff per file.
4. Run `--phase A` and `--phase B` without `--apply`; record exit codes and summaries.
5. Return path, sha256, the two dry runs' results, and anything not placeable as ruled.

## Notes (one line per step, as done)
