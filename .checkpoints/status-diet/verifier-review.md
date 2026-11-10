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
- Own E1 (verifier-e1.mjs, multiset, no split.mjs): at 8ead8f1 old 1233 lines/177445 B; new 1423 (STATUS 103, DEC 496, ARCH 824); old texts short in new: 0; 177 new lines are new text; old headings reused as new headings only (## Phase x2: archive + new STATUS). Runs: STATUS 847-849; DEC 674-690 727-762 763-774 691-726 200-239 198-199 240-272 281-420 775-799 813-846 880-928; ARCH 2-196 421-673 862-879 273-280 800-812 850-861 929-1233. PASS.
- Finding L-1 (Low): the A3-notes note (old 198-199) sits after the 11-09/10 block, before the A3 block (tpl line 34), not before 200-239 as plan §2 lists; harmless (closer to the A3 block it serves), but its "cited in this file" now spans DECISIONS and the archive (273-280 rulings cite them too).
- Ordering: DEC dated headers newest first (11-10 go46, 11-10 carried-out, 11-09/10, 11-05, 11-04 x3, 11-03 x4 in STATUS order, 10-31 x6 STATUS order, 10-30, 10-28, 10-27 x2, 10-25, 10-14). Advisor-agent blocks 273-280, 800-812 in archive (Q1). OK. Headers 888, 908, 912, 926 say "Taken by the Advisor" without "relayed by Mustafa" (capital A = claude.ai Advisor per CLAUDE.md naming); accepted as the Advisor's.
- Finding L-2 (Low): DECISIONS intro cites docs/reports/2026-11-10-status-diet.md, which does not exist yet (report item; must exist before ready).
- Later commits: only 1b030e3 and 37434d5 touch the three files (8ead8f1..37434d5). 1b030e3: STATUS_ARCHIVE one line (the kit-v3-5 In-flight row: branch cell -> "merged: PR #46 ..., 2d22b69, 16:05:52 UTC, parents 44446a5 0573db3, ... CI run 42851159265", state cell + the advisor's final-head APPROVE and his go, waiting cell -> "none (re-enabled 16:07:30 UTC)") + ROADMAP 2 lines; matches its message. 37434d5: DECISIONS header 87-89 (new text) and STATUS 2-4, 66-68 (new text); removed lines checked: none is an old STATUS line. e7d78ca, 846ef59 touch none of the three. Head multiset: exactly 1 old line short (that kit-v3-5 row). E1 PASS.
- B2 done. Next B3 (E2).
