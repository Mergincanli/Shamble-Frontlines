# r5 rust-reviewer notes

- 17:49 started; items e1 F3, e3b 2, COMMUNITY-SERVERS. Next: read verdict r5, inputs, fixcheck verdicts.
- 17:50 FIX e1-F3 written (phase 0 row = ADR 0002 step 0's wording). Next: FIX e3b-2 (phase 1 row), then COMMUNITY-SERVERS grep.
- 17:51 FIX e3b-2 written (phase 1: wave 1 before the foundation, content spike in wave 3 after phase 2; milestone S19–S22, S21/S22 were missing). Next: COMMUNITY-SERVERS grep of ADR 0004.
- 17:52 NOTE COMMUNITY-SERVERS written (no change; :568-569 supervisor clause describes the official deployment, the room needs none). Next: scratch tree, dry-run, glossary check, line/cell checks.
- 17:53 checks: dry run 2/2 ok on the doc on disk (docs untouched); glossary on the scratch tree exact 0 before and after,
  hits file identical, 0 unpaired quotes; 0004 tables 17, none uneven, none glued; migration rows 9 x 3 cells; PM rows
  341 (63, 78, 47, 75, 46, 32) x 7 cells; prose over 120 in 0004: 29 before = 29 after (pre-existing, none mine);
  quote "in SPIKE_PLAN's order, but wave 1 runs one heavy spike at a time" verbatim in STOP-ANSWERS; no other r5 file
  touches 0004; label grep of 0004: no OQ-0003-1, OQ-0006-15/-20, "stands in for none" (condition 3 doesn't reach 0004).
  Scratch kept: before/ and tree/ copies of 0004, applied.diff, dryrun/apply/glossary/tables outputs, scripts.
- DONE: FIX e1-F3, FIX e3b-2, NOTE COMMUNITY-SERVERS (no change).
