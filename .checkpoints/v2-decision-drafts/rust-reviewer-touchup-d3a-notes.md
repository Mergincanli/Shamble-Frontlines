# rust-reviewer touchup d3a: notes

- 2026-11-03 start: output file created; next: read skeptic file, tests, draft 0004:260-280.
- 06:37Z own check: rr_euclid.rs (rustc 1.97.0 -O, x86_64, exact integer reference): div_euclid wrong in 8/8 (f32,f64 x 4 sign cases: 3->exact 2, -4->-3, -3->-2, 4->3), rem_euclid right 8/8. D3A-1 holds. Next: spot-check std rmeta doc, then compose FIX.
- 06:39Z rr_f64_same_inputs: f64 on (3+2^-22, 1+2^-23) gives the right 2: the f64 failing case is (3+2^-51, 1+2^-52), so the skeptic's 'in f32 and in f64' for one input pair is imprecise; drafting at 6-line budget (272-277): drafts A-D give 8-9 lines. Next: decide reflow scope.
- 06:42Z draft K fits 16 lines when 262-277 are reflowed (net 0); refining wording (name the two methods).
- 06:43Z draft P fits 16 lines (262-277 reflowed, net 0). Next: build AFTER with no break inside the maths, glossary check, dry-run.
- FIX D3A-1 written to fixes file (BEFORE 262-277 by script, 16 -> 16). Next: dry run, glossary check, NOTE blocks.
- dry run ok (16 -> 16; 0004 stays 1410 lines; lines 1-261 and 278-end identical; word diff: only the sentence's words change). Glossary check on scratch repo copy: exact 0 before and after, 0004 in-context 98 both, hit lists identical. Next: NOTE blocks (PM-0004-22 for test-engineer; evidence).
- NOTE D3A-1-PM22 written. Next: NOTE D3A-1-EVIDENCE.
- NOTE D3A-1-EVIDENCE written. Next: final dry run of the whole fixes file, then return. 
- remark 6 refined (mul_add sign test). Final dry run ok (16 -> 16). DONE
