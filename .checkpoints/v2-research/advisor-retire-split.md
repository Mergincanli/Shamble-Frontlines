Task: advisor verdict on splitting GLOSSARY Retire into exact / in-context (condition 9, before step 3c)
Checked: GLOSSARY.md Retire fields (l.20-26, 79-574); glossary_check.py; ran it --skip-records: 1335 hits.
Findings:
- Parser takes EVERY quoted string in Retire, incl. kept/qualifier words: "Core" 76, "big" 53, "Whole ticks" 5, "the security baseline" 2 (the correct name) -> false hits.
- Substring, case-insensitive count: "core" hits "simulation core"/"score"/"cores"; "tick time" hits the term "Server tick time" (counted 3x across entries); "48" counted twice (2 entries, 244 each).
- So ~380 of 1335 are artefacts, before any split.
Verdict: APPROVE with conditions (parse fix, word boundaries, own-term exclusion, dedupe, exact = no correct hit today, re-baseline before 3c, quoted-set diff check).
DONE
