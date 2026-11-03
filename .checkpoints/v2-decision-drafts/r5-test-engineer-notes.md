# r5 test-engineer notes

- 17:49 started; read verdict r5, inputs, verifier e1/e2/e2b/e3b verdicts next.
- 17:55 read verdict r5 (conditions 3, 4, 7a), inputs, e1 F1, e2 F1/F2, e2b F1/F7. Tools in scratch: add_block.py (BEFORE from disk lines), row_edit.py, glossary_check_scratch.py (tree/). Next: locate lines in 0002, 0006, 0003, 0005, 0001.
- 18:05 FIX e1-F1 written (PM-0002-49 reading mark). Next: e2-F2 (0006 key profile).
- 18:07 FIX e2-F2 written (0006 key profile: loss on each marked; OQ-0001-16 + R5 cite moved to the half split, R5-TEST-REC4 says only 'half the round trip each way'). Next: e2-F1 key S (0003:1122-1123).
- 18:09 FIX e2-F1-keyS written (0003 key S: option (a) mapping marked). Next: e2b-F1 (PM-0005-4).
- 18:11 FIX e2b-F1 written (PM-0005-4: kinds units, vehicles, weapons and factions; both tuning table -> rule table). Next: e2b-F7 PM-0005-15.
- 18:12 FIX e2b-F7-PM15 written (PM-0005-15 adds the client entries keyed by simulation ids, CD-2). Next: condition 3+4, ADR 0001 key (1364-1369), then ADR 0003 key (1078-1090).
- 18:18 FIX c3-c4-0001-key written (ADR 0001 key: OQ-0003-1 answered in part, classes quoted, still open, stand-in for ML + reading, client-frames quote + reading; 'stands in for none' gone). Next: ADR 0003 key (1078-1090).
- 18:22 FIX c3-0003-key written (ADR 0003 key: classes quoted after MP, class text untouched, still open, stand-in + reading). Next: dry-run all, glossary check in tree, quote verbatim check, label grep.
- 18:31 c3-0003-key reworded ('The models of ML, RD and MAC'); NOTEs n1 (0003 OQ entry, graphics-engineer), n2 (other keys, no change), n3 (no rows) written. Next: fresh tree, dry-run, apply, glossary, rules; then DONE.
- 18:36 checks on a fresh scratch tree (tree/ = disk copy + my blocks; base/ = disk copy): apply_fixes dry-run ok on
  disk paths and on base/ for 0001, 0002, 0003, 0005, 0006 (7 FIX blocks, all applied); glossary_check (tree): exact 0
  in every topic, counts identical to base; rules.py: PM rows 63/78/47/75/46/32 = 341 (unchanged), 0 bad cell counts,
  0 changed prose lines over 120, 0 sentences under a table; every Mustafa quote in my AFTERs found verbatim in the
  inputs (whitespace normalised); "doesn't measure client frames" once (ADR 0001 key); "stands in for none" left only
  at 0003:822 (graphics-engineer, NOTE n1); "tuning table" gone from PM-0005-4.
DONE
