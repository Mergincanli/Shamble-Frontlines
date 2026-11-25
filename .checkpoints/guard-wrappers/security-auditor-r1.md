# security-auditor round 1: guard-wrappers (SAME-GAP) review of 5df545c, .claude/hooks diff

Plan (blocks, in order):
B1 read public plan + advisor notes + detail plan + out-of-scope list
B2 read the diff (guard.mjs, tests)
B3 read unwrap/WRAP_OPTS/rule U/git()/gh() in full at head
B4 probe scripts in .gate/guard-wrappers/ (forms per wrapper, rule U, throws, false asks)
B5 judge tests/mutants, turn limit
B6 write findings (generic only here), DONE

Notes (this folder is public: finding ids and severities only; every form and detail is in the handback, never here):
- B1 done: plan, advisor notes, detail plan, out-of-scope list read.
- B2 done: guard diff read.
- B3 done: bash() flow read.
- B4a done: first probe batch (harness in .gate/guard-wrappers/ built from the test file's own fakes).
- B4b done: fuzz of random chains: 0 throws; no case where main is stricter than head in a way that matters.
- B4c done: false-ask probe of this repository's common lines.
- B5 done: rows and mutants read; rule C is pinned by a row with its reason.
- Findings (ids only): SA-a Medium, SA-g Medium, SA-b Low, SA-c Low, SA-e Low, SA-f Low (tests), SA-h Low;
  SA-d pre-existing and out of this diff's scope, for Mustafa. Details in the handback only.
- Verdict: REVISE (two Mediums with small fixes).
- Next: B6 answer.
