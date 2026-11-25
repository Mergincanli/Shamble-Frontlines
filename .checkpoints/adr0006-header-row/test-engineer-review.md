# test-engineer review, round 1: ADR 0006 header-row revision (09faefa..1c45ed7)

Reviewer: test-engineer (owner of ADR 0006's pass marks). Started 2026-11-25. Notes only; no docs edited, no commit.
Memory: no local agent-memory folder for test-engineer in this cloud session (checked .claude/ and ~/.claude/).

## Plan
1. Read the ADR diff (E1, E2, E3, E4, E6, E7, E9, E10) against plan.md revision 2, section 3. DONE.
2. Read Mustafa's ONE-OFF-HEADER (a) and STEP4-COUNT (b) (DECISIONS :169-193) and the options as put (report :278-301). DONE.
3. E1/E2: enumerate what fails PM-0006-2 on each URL form after the revision; re-derive the bite and the "met by
   readings already taken" claim from the raw verify artifacts in `.gate/preview-hosting-check/day1/`
   (round-a, bite-production, restore-production) with jq, not only from Appendix A's prose.
4. E4/E9/E10: can PM-0006-7 fail, can it record something wrong; consistency with PM-0006-9, the "When a mark fails"
   paragraph (ADR :1420-1434) and the hosting checklist intro (:1462-1467).
5. The advisor's note: the preview `noindex` gap; word a follow-up.
6. Looser-than-his-words check on every revised cell.
7. Verdict and ranked findings, file:line, exact fix.

## Findings (appended as found)
