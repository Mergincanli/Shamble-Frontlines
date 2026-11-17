# U4f brief rev 3: netcode-architect notes

Head read: 8afcc11b63906d3660b21613deb0729bdd8937bc (2026-11-17 17:15 UTC). Fresh run (no notes file before).

## Plan (blocks; a line after each when done)
- B1 Read rev 2 (34a2e55:u4f-brief-draft.md), the two checks, the drafter's and reviser's notes, the night card.
- B2 Read the sources at HEAD: u4-build-plan.md §5 U4f and its cites; u4s-carries.md (rule, U4f section, named carries);
  u4r-brief.md (form: section 0, section 4, section 12, placeholders); c62h-bound-brief.md (stops, texts-for-his-yes,
  13:35 read); history.md :2067-2075, :2125-2127, :2131-2232; CHECKPOINTS.md §11.
- B3 Rulings since 11fd190: advisor-* files changed since 11fd190; grep for U4f, U4F-, carries 46/80/86/57/74,
  "the next npt-judge"; the card's Open findings "From C62H-BOUND".
- B4 Code at HEAD: the recheck rows of rev 2 walked one by one (file:line); added rows for C62H-BOUND, U3C-HELD-1,
  REFS-KIT.
- B5 Placeholders: a python3 -I script under u4f-r3/ that checks each anchor is found once; sha256 of the 3 files.
- B6 Write rev 3 to u4f-brief.md.
- B7 Self-check, return summary.

## Block log
- B1 done: rev 2 read whole (1,117 lines: change list :12-66, counts :71-88, s0 :94-125, s1 :129-173, s2 :177-477,
  s3 :481-506, s4 :510-585, s5 :589-698, s6 :702-717, s7 :721-746, s8 recheck :750-786 (33 rows), s9 :790-810 (17
  rows), s10 Q1-Q16 :814-1020, s11 D1-D15 :1024-1042, s12 :1046-1097, NA table :1101-1117). Night card read (SC-1..3,
  NA-13, Next steps 1-4). Reviser's notes read (flag: 2.3 total-key route; limits). HEAD moved to e56c3dd while I
  read (the session's commit of this notes file); stamp the head at the end.
