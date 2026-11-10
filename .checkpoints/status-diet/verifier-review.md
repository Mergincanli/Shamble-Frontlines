Task: verifier review of change-set status-diet (PR #49, branch claude/busy-turing-48e4i9, head 37434d5, base 2d22b69)

## Plan (blocks, in order)
B1 read inputs (plan, verdict, addendum, e5, followups, go46, scripts)
B2 E1 check.mjs reproduce + own independent multiset check at 8ead8f1 + later-commit diffs
B3 E2 decision-id counts, nets soundness, 10+ block spot-check, ordering, provenance
B4 E3 e3.mjs + CLAUDE.md diff read against his words
B5 E4 gate --no-smoke (+full if possible), repo-map-check, three kit tests
B6 E5 rule-diff table vs git diff CLAUDE.md docs-keeper.md
B7 E6 #46 facts, go quote byte check
B8 conditions 1-12 + addendum (a)-(c)
B9 STATUS.md checks (lines, in flight, follow-ups, security line, pointers)
B10 write answer

## Notes
- B1 done: read plan-rev1, verdict (cond 1-12, Q1-Q5), addendum (a)-(c), e5, followups.tsv (46 open), go46 record, facts, split.mjs, check.mjs.
- check.mjs --rev 8ead8f1: ALL PASS (1233 = 1218 moved [STATUS 3, DEC 424, ARCH 791] + 15 structure; E2a 65u/104m lost 0; E2b 248u/512m lost 0 new 6; E2c 112u/122m lost 0 new 20; RM 109/109; STATUS 103 lines; Q2 46 open named).
- check.mjs --rev 37434d5: 4 FAIL, expected (E1b files differ from script output after later commits; E1c archive lacks the old kit-v3-5 row, edited by 1b030e3). E2 nets a-c at head: lost 0 (b: new-text mentions 77). STATUS 104 lines. Note: E2 RM line counts from the script's rows, not the files on disk -> at head it does not test head files (Low, method note).
