# verifier-skeptic-6-d3e-1: skeptic on fixcheck-d3 unit e, verdict D3E-5

## Verdict under test
D3E-5 (medium, wrong), doc docs/reports/2026-11-01-v2-decision-drafts.md:33-34 and 66-68, author main session.
Claim: the report says a skeptic re-checked every wrong/unsupported verdict first, and the owner checked four "wrong"
verdicts (fixcheck-d0 row 9; fixcheck-d1 F1, F2, F10) instead of a fresh skeptic. The verdict says d1 had 3 wrong + 3
unsupported, d2 had 9 wrong/unsupported rows, d0 had row 9; skeptics 1-5 came before d0; skeptic-6 judged only D3A-1;
so 16 rows went to owners with no fresh skeptic.

## Notes (progress)
- started; next: read report lines 25-75.
- read report:33-34, 66-68 (at working tree, HEAD 32a6a83); d0, d1, d2 verdict files; skeptic-1..5 prompts; plan:230-231.
  Next: check skeptic coverage of the drafts' verifiers and fixcheck-a..c (does the fix's "every ... of the drafts'
  verifiers and of fixcheck-a to -c" hold?).

## Block 1: the report text (read myself)
- docs/reports/2026-11-01-v2-decision-drafts.md:33-34: "A skeptic re-checked every "wrong" or "unsupported" verdict
  first (five rounds: 30 upheld, 9 partly, 2 refuted)."
- :66-68: "At the end, for time, the owner checked four "wrong" verdicts against their sources instead of a fresh
  skeptic (fixcheck-d0 row 9; fixcheck-d1 F1, F2, F10); fixcheck-d3 then judged the four on their own: every verdict
  holds, and every fix is right but d0 row 9's wording, ... (D3A-1, fixed after a skeptic)."
- plan.md:230-231 (the rule): "A "wrong" or "unsupported" verdict gets one skeptic (a fresh verifier, xhigh, asked to
  refute it) before it goes to the owner."
END Block 1

## Block 2: d0, d1, d2 wrong/unsupported rows (read myself)
- d0 (verifier-fixcheck-d0-verdicts.md:19): row 9 "wrong in part". Rows 6, 7, 8b "confirmed, with a note"; 8c
  "confirmed in substance, wording imprecise" (:17), not wrong/unsupported. So d0: 1 row.
- d1 count line (verifier-fixcheck-d1-verdicts.md:445): "3 wrong (N42, I18, I19); 3 unsupported (N12, I21, I24)".
  Rows: N12 :41 "unsupported (a review's estimate, its parts not shown)"; N42 :71 "wrong"; I18 :300 "wrong against
  ADR 0001 at HEAD; citation off"; I19 :301 "the ADR 0001 parts stale" (counted wrong at :445); I21 :303 "the last
  unsupported"; I24 :306 "the 15–30 hours unsupported (a review's guess)". Findings: F1 = N42 (:486), F10 = I18+I19
  (:~465-476), F13 = N12 + I24 (:~536), F14 = I21 (:540). F2 (:477) is a consistency finding ("3 inconsistent (F2, F9,
  F10)", :446-447 and :452), not a wrong/unsupported row verdict. So d1: 6 rows; the report's "F1, F2, F10" covers 3 of
  the 6 rows (N42, I18, I19) plus F2; N12, I21, I24 not named.
- d2 (verifier-fixcheck-d2-verdicts.md, verdict column): 32 :40 "wrong in part"; 34 :42 "wrong"; 51 :59 "unsupported
  as a LOOK_V2 expectation"; 73 :81 "confirmed, one phrase unsupported"; 77 :85 "wrong in one word"; 92 :100 "wrong
  (structure)"; 96 :104 "wrong (stale)"; 111 :119 "wrong (structure)"; S12 :306 "wrong (stale)". 9 rows; none named in
  the report.
- So 1 + 6 + 9 = 16 rows (the verdict's count holds by verdict label).
END Block 2
