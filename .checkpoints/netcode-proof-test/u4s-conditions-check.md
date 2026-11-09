# U4s conditions check: test-engineer notes (2026-11-09)

Task: check only that the advisor's U4s ruling (advisor-u4.md:236-383) is applied. Read-only on every other file.

## Plan
1. Hashes: HEAD's u4-build-plan.md = f7961cfe…; working files' sha256 vs the ones recorded in u4s-conditions-notes.md.
2. Diff u4-build-plan.md HEAD vs working: each hunk traced to condition 1 (a)-(d); nothing else; D-4 word for word
   (advisor-u4.md:355-360); freeze word for word (condition 3, :328-333).
3. u4s-carries.md vs condition 2's list and O1: every item present.
4. Verdict and exact fixes.

## Progress
- Step 1: HEAD:u4-build-plan.md sha256 f7961cfee8cdff10d9bd64ac0df7859ec59cd8d9724367e713f994e298f33302 (matches the
  expected f7961cfe…). Working u4-build-plan.md 7ec1b324287faefc7b1968184b599c8da691fcde9f992c888f195ed57579ff87 (1,031
  lines); working u4s-carries.md 0b4b896b792d111cccd26d2700b000e1ebc8b5cdadb0c151540e20749ca2ee13 (230 lines);
  HEAD:u4s-carries.md 1500ca04425f9872cdbe00464356addece691fcc130dcb90a6bd5fbab2856f41. All four equal what
  u4s-conditions-notes.md records. Step 1 done.
