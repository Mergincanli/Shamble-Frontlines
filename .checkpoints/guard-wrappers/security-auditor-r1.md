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
- B4b done: random fuzz: 0 throws. Exhaustive main-vs-head pass over the four wrappers that lost hand-written
  branches (9804 lines): no deny became a pass; the deny->ask lines all have a dash word as the real program.
- B4c done: false-ask probe of this repository's common lines.
- B5 done: rows and mutants read; rule C is pinned by a row with its reason.
- Trial fix for SA-a, SA-g, SA-h in a .gate copy (sa-mkfix.mjs, suite copy sa-suite/): suite 1436/1437, the one
  failure is the uniqueness of a mutant anchor my trial wording duplicated.
- Findings (ids only; scratch case files in .gate/guard-wrappers/ hold the forms):
  SA-d Critical, pre-existing (sa-cases1.txt helper block, sa-cases5.txt); SA-a Medium (sa-cases1.txt input block,
  sa-cases6.txt); SA-g Medium (sa-cases4.txt, sa-cases3.txt); SA-b Low (sa-cases1.txt path/function block);
  SA-c Low (sa-cases1.txt read-on block); SA-e Low (sa-cases2.txt, sa-exh.mjs, sa-exh2.mjs); SA-f Low (tests);
  SA-h Low (sa-cases3.txt).
- The notes file's pushed versions (07897f22, b32e4ee9) hold only the plan and this id list: nothing leaked.
- Verdict: REVISE.
DONE
