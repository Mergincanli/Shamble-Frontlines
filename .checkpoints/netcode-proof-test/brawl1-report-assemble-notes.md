# Notes: assembling BRAWL-1's report section into u4r-report.md (assembly agent), 2026-11-15

## Plan
- Baseline: tree clean at aef7590; u4r-report.md 667 lines, sha256 baed0445...; draft sha256 81965249... (604 lines).
- Edits bottom-up so the earlier targets keep their line numbers; Edit tool only; read each target first:
  A. INSERT draft :389-524 after u4r :667 (end of file).
  B. INSERT draft :212-383 after u4r :655/:656 (blank), before :657 "### EVQ-LOAD".
  C. INSERT draft :13-207 after u4r :614/:615 (blank), before :616 "### Review round 2".
  D. Item (1): replace u4r :562-564 (old TICK16 line) with draft :546-555.
  E. Item (3): replace u4r :525-526 with draft :601-603.
  F. Item (2): replace u4r :215-216 with draft :576-580.
- Then: print-only check that every inserted block equals its draft lines, no [[ ]] left; git diff --stat; final line
  numbers.

## Progress
- A done: draft :389-524 inserted after u4r :667, at :669-804 (pre-shift numbers); print-only check: equal to the
  draft lines; file ends with one newline.
