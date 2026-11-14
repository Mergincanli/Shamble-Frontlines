# Advisor: carry 62 (h)'s step-5 record came INVALID (pre:PM-0001-2: not in marks.json), 2026-11-14

## Plan
1. Read advisor.md from main (done).
2. Read my ruling advisor-u4r-r2run-full.md (done): I had claimed Rule::Pre reads the whole marks file whatever --only says (eval.rs:958-996).
3. Check commit 5e563b9's diff (marks.json, plan line, card only; crates unchanged since e59ece9).
4. Read the step-5 record folder (README.txt, both verdicts) and the diagnostic folder (--only PM-0001-2,PM-0001-17).
5. Read the judge's code: main.rs (--only handling), marks loading/filtering, eval.rs Rule::Pre (:958-996) and where "not in marks.json" is produced. Find the line(s) that make pre:PM-0001-2 unfindable under --only PM-0001-17.
6. Decide: intended or defect; severity, reach (c-u4r-runs.mjs's judged sets, c-u4r-r2.mjs, earlier verdicts), home.
7. Decide step 5's record form and folder; whether carry 62 (h) stands; marks.json change unchanged or not.
8. Next steps in order; anything for Mustafa.

## Checks (a line per check)
