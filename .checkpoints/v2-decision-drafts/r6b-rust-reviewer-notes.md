# r6b rust-reviewer notes

- 22:28 started: created fixes file; next: read advisor-verdict-r6 amendments, plan, inputs.
- 22:30 read amendments 1+2, cond 6; grep of 0004 outside pass marks: no CX33/cheaper host/location/order; scratch tree set up. Next: FIX ARM-LG-1 at part 7 line 290, NOTE for the grep.
- 22:31 FIX R6B-RR-1 written (part 7, line 290 -> 4 lines; one sentence: LG not an ARM host, advisor's reading; pointer to ADR 0001's rule scoped to its LG rows). Next: NOTE R6B-RR-2 (grep), checks.
- 22:32 NOTE R6B-RR-2 (grep: nothing in my text) and NOTE R6B-RR-3 (for test-engineer: R1-ECS-OQ4, R1-TICK-OQ5 from RS; Appendix A 1329/1334 stand) written. Next: apply in scratch tree, glossary, widths, diff.
- 22:33 FIX R6B-RR-1 revised: 'An ARM load generator, such as the netcode proof test's (ADR 0001's LG)' (covers a soak bot VM too); paragraph tail 290-295 rewrapped 6->10, words checked identical but the sentence. Next: re-run dry-run, apply, glossary.
- 22:35 checks (scratch tree r6b-rust-reviewer-scratch/tree, laid out as the repository): dry run on the doc on disk ok
  (R6B-RR-1: 6 lines -> 10); applied in tree/; glossary (default and --skip-records) exact 0 for 0004 and outputs
  identical before/after; new lines <= 120 chars; words outside the new sentence identical; 0004 identical outside
  290-299; table rows and cell counts unchanged; PM rows 0004 75 -> 75, all six ADRs 341; draft_pointers same (0
  checked); no quote marks in the new text; docs on disk untouched (git status clean). Scratch: before/, tree/,
  applied.diff, gl_*.out, dp_*.out, dryrun_disk.out, apply.out, rewrap.py, sentence.txt.
DONE
