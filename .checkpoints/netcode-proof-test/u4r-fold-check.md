# U4r fold check (adversarial check of u4r_fold.py)

## Plan
1. Hash the inputs (ruling, brief, te-m3, te-m3-check, the script) against the task's stamps.
2. Read the script whole; run the dry run (`python3 -I .checkpoints/netcode-proof-test/u4r_fold.py`), read its whole output.
3. Read the ruling :74-382 and :415-420; list every item L1 includes/excludes.
4. For each item compare the diff hunk to the ruling's text word for word (white space aside).
5. Find every "by U4R-Qn" and every "P0 1" in the brief; say how the fold treats each (:178, :357-358).
6. Check section 10's new table rows against the ruling's lines; check stamps vs sha256; check no line outside items changed.
7. Verdict + findings.

## Stamps
(filled as I go)

## Findings
(one line per finding as I go)
