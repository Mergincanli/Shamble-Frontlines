# test-engineer touch-up PM-0004-22 notes

- 07:48 started: no prior notes; created fixes file and scratch. Next: read part 7, NOTE D3A-1-PM22, skeptic file, rr_euclid.out.
- 07:52 read part 7 (0004:257-277), NOTE D3A-1-PM22, skeptic-6 verdict, rr_euclid.rs/.out; table cols: mark|proves|number|hardware|method|runs|basis; Bites rule 0004:1091-1093 needs a planted defect. Next: own exact recomputation in scratch.
- 07:58 own recomputation done (te_euclid_ref.py/.out: matches rr_euclid.out 8/8; te_rem_round.rs/.out: rustc 1.97.0 rem_euclid(-2^-30,1)=1.0=|b|, so the reference must be rounded once to the type). Next: read glossary/apply scripts, compose FIX.
