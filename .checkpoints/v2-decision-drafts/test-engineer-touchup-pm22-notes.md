# test-engineer touch-up PM-0004-22 notes

- 07:48 started: no prior notes; created fixes file and scratch. Next: read part 7, NOTE D3A-1-PM22, skeptic file, rr_euclid.out.
- 07:52 read part 7 (0004:257-277), NOTE D3A-1-PM22, skeptic-6 verdict, rr_euclid.rs/.out; table cols: mark|proves|number|hardware|method|runs|basis; Bites rule 0004:1091-1093 needs a planted defect. Next: own exact recomputation in scratch.
- 07:58 own recomputation done (te_euclid_ref.py/.out: matches rr_euclid.out 8/8; te_rem_round.rs/.out: rustc 1.97.0 rem_euclid(-2^-30,1)=1.0=|b|, so the reference must be rounded once to the type). Next: read glossary/apply scripts, compose FIX.
- 08:05 te_signed_zero.rs/.out: rustc 1.97.0 div_euclid(0.5,-1)=-0.0, rem_euclid(-1,1)=-0.0 (exact value integer 0): reference compare must be by value (==), zero sign left to to_bits identity. Next: compose FIX PM22-EUCLID from the on-disk row (script), write to fixes file.
- 08:12 FIX PM22-EUCLID written (row stays 1 line, 7 cells). Next: dry run, glossary before/after, NOTE with evidence.
- 08:18 dry run ok (1->1 line, 1410 lines); glossary exact 0 before/after, 0004 in context 98 both, hit lists identical (with and without --skip-records). Next: NOTE PM22-EVIDENCE, final checks.
- 08:25 NOTE PM22-EVIDENCE written; dry run re-run ok. DONE
