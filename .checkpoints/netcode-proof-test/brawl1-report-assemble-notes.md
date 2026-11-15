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
- B done: draft :212-383 inserted after u4r :655 + blank, at :657-828 (pre-shift numbers); EVQ-LOAD now at :830;
  print-only check: equal to the draft lines.
- C done: draft :13-207 inserted after u4r :614 + blank, at :616-810; round 2 now at :812; print-only check: equal.
- D done: item (1), old :562-564 (matched exactly) replaced by draft :546-555.
- E done: item (3), old :525-526 (matched exactly) replaced by draft :601-603.
- F done: item (2), old :215-216 (matched exactly) replaced by draft :576-580.
- Final check (print-only python): the file equals aef7590's u4r-report.md with the three replacements and the three
  inserts, each word for word from the draft; no [[ or ]] left; 667 -> 1,184 lines; sha256 3164a6ee...
- The background saver committed a mid-run state at 8fff936 (11:33:46 UTC, after A). `git diff --numstat aef7590`:
  u4r-report.md 524/7, plus this notes file 20/0; nothing else.
- Final line numbers: item (2) :215-219; item (3) :528-530; item (1) :566-575; BRAWL-1 heading :625; insert C
  :627-821; round 2 (kept) :823-862; insert B :864-1035; EVQ-LOAD (kept) :1037-1041; TICK16 (kept) :1043-1047;
  insert A :1049-1184.
- Flag, not changed (KEEP): the kept heading at :1043 still says "it replaces the line at :562-564"; that line now sits
  at :566-575.
