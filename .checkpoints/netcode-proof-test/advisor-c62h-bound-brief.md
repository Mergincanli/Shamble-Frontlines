# Advisor: plan review of the C62H-BOUND build brief (c62h-bound-brief.md)

Seat: advisor (main's .claude/agents/advisor.md, read with `git show origin/main:.claude/agents/advisor.md`, 27881 bytes).
Read only: this notes file is the only file I change.
Repo HEAD at start: 77d3ad7 (checkpoint autosave); origin/main 53272a1.

## Plan
1. Read the brief (540 lines) and its notes.
2. Read his words: history.md 2113-2129 (13:35 rule), 2131-2146 (C62H-BOUND (a) + conditions), 2163-2175 (C62H-BOUND-YES).
3. Read my earlier reads: advisor-c62h-bound-full.json (C1-C11, Q1-Q5), advisor-c62h-bound-r2-full.json (te_rulings,
   replacements, planned_size, paste_blocks, other_changes 6), c62h-bound-test-full.json (TE-CB-1..10), the design json.
4. Frozen texts: c62h-bound-texts-v3.md, c62h-bound-freeze.txt; recompute sha256 of each block; paste sha256.
5. Card: NOW block, KEPT-LOGS lines, C62H-BOUND blocks, MH-L1 note. advisor-merged-head-full.json for head.
6. Code: spikes/netcode-proof (l1, lab, judge, marks, ops/runs/c-u4r-runs.mjs, check.mjs); runners u4r-run.sh, main-run.sh sha256.
7. Earlier briefs for form: u4r-brief.md, brawl1-brief*.md.
8. Cross-check: every C5-C11 item, te_rulings, other_changes 6, his YES words (freeze, P0 re-judge, size stop, stop rules) carried,
   nothing beyond; frozen texts by script + sha256; order build -> review round -> merge into branch -> P2 then P3 -> re-judge -> report;
   builder decides nothing.
9. Findings with brief line + source line; verdict; structured output.

## Progress
- 1-5 read. Hashes recomputed: texts-v3 3f21c9b7, freeze 9cb8be17, paste cfac7502, u4r-run.sh e9437ae2, main-run.sh
  8df2dee3, peak.py dfacdcc0, c-u4r-runs.mjs 99dd070e, marks.json 9fb5be57, SERIES.md 3411eee7, README.md b49f3441:
  all equal the brief's. Brief sha256 ba6bdeb0 (540 lines) = its notes.

- 6-7 checked. Insert algorithm reproduced in memory (python3 -I, no file written): SERIES part 1 89295b57, part 3
  6898e119, marks.json c1935a5b (json.loads ok; difflib +26 -0), README f3c3c804 (+16 -11; refill 16 lines, max 120;
  word diff one insert of 73 words = readme-sentence). All four equal the brief's :101, :109, :116, :121.
- Base facts: 442a284 tree 6ca9f77f = HEAD 77d3ad7's; 717b802..442a284 adds only b5cb253's raw-logs.sha256; 0457c85 ok.
- Code cites spot-checked (l1 :245-247, :325-398, :1084-1110; scenario :204, :234-252, :739, :989, :1612, :1756;
  logs :161, :420, :495; precheck :312; contract :465-513 incl. r2 (b); figures :224-279, :316, :632; eval :343-353,
  :1083-1095, :1210-1231, :1475-1478; runner :29-34, :166, :289-291, :306-332, :464, :475, :566; gate.mjs:75): hold.
- Coverage of C5-C11, r2 te_rulings, other_changes 6, his YES words: all carried (table in summary). Order checked.
- B-2 verified on the fixtures: base/lost-group/hashed-and-incomplete change as the brief says; none-compared A 6 stays
  FAIL; differ, refused-group, orphan, permuted-slots, sparse, sampled, slot-unlogged, short-lists, l1-aarch64-claim keep
  their answers; L2 cases untouched (checks on L1 parts only); no cli.rs or other reader of pm-0001-17/base.
- B-3 verified: advisor-brawl1-sets-p0-full.md:7 "P0 STANDS AND COUNTS"; advisor-u4r-sets-r2-full.md:10 (051995db9cea
  never counted). Re-judge comparison form checked against the P0 verdict files (-17 why "bite ...: no run yet" lines,
  bites list entries; L1 P2/P3/B1 parts carry no checks when they hold no runs).

## Findings (a line per finding)
- ADV-CBB-1 Medium: route left open (brief :39-51); route W's per-step stop/resume and insert part 1/3 timing undefined.
- ADV-CBB-2 Medium: SIZE-CHECK pathspec/total includes results/ (brief :304-308, :486); results add ~3,000 lines.
- ADV-CBB-3 Medium: section 10 "His" stop rules reworded, not quoted (brief :509-517 vs paste :120-124).
- ADV-CBB-4 Low: re-judge is two judge calls vs his "a judge call" (brief :389-396 vs history :2174); (ii) exactly one.
- ADV-CBB-5 Low: B-2's changed fixture answer must be named in the 13:35 read line; no other test assertion may move.
- ADV-CBB-6 Low: "about" values a script must assert (brief :286-288, :469): runner prints 552 and 1530 exactly.
- ADV-CBB-7 Low: risk tier not named (plan-review checklist): tier 1.
- Rulings: B-1 yes; B-2 in (commit 3) with conditions; B-3 confirmed. Order (re-judge before the sets) accepted.
- Verdict: APPROVE WITH CONDITIONS.
