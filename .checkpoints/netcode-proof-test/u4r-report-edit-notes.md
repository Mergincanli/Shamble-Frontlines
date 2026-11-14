# Notes: the u4r-report.md edit (MRG-L1 to L3, the judged runs items 1 and 2, the follow-ups)

## Plan
1. Read the report whole (220 lines) and the sources: advisor-u4r-merge-full.md (Findings), advisor-u4r-r2run-full.md,
   advisor-u4r-c62h-full.md, r2.txt, the c62h first-try and record READMEs and verdicts, c62h-check.log, git log for
   5e563b9 and a36babd.
2. Edits by Edit only: (a) MRG-L1 (:88, :95), MRG-L2 (:107-127), MRG-L3 (:53-57); (b) judged runs items 1 and 2;
   (c) Follow-ups: JDG-ONLY-1, and RUN-L1 plus the whole-as-lost suite's Low on the U4r-F1 line.
3. Keep at most about 260 lines; report line count, sha256 and each edit's new range.

## Edits (a line each)
- MRG-L3: lines 56-59, the EA-U4R-2 sentence in the advisor's words added to the round-1 bullet.
- MRG-L1: lines 89-91 (provenance counted over the whole call, seven builds) and 98-100 (uptime at 13:44:36, 15 min
  41 s before det-u4 at 14:00:17, m5.log:3, :184; load at det-u4's start not measured).
- MRG-L2: carry 20 at 115-118, carry 69 at 123-125 (advisor-u4r-merge.md C13 (b)); the "Not found by number" line
  deleted; 73's bare ":1430-1439" made u4r-build.md:1430-1439 (126) so it does not read as c-u4r-r2.mjs; 119-131 reflowed.
- Judged runs: heading 211 now "1 and 2 done, the rest pending"; item 1 at 212-229 (r2.log times, e59ece9 = 71ee13d's
  spikes/ by git diff --quiet exit 0, 42 of 42 exit 0 in r2.txt, no "stopped at"; suites from r2run "Each suite");
  item 2 at 230-244 (5e563b9, a36babd, check-marks per 5e563b9's message and the card, c62h-check.log, first try,
  record README and verdicts read by python print); item 3's ":19" made advisor-u4r-r2-full.md:19 (245).
- Follow-ups: RUN-L1 and the whole-as-lost suite Low on the U4r-F1 line (173-178); JDG-ONLY-1 (179-186), noting the
  ruling's own disagreement ("both run scripts already do" vs its reach on c-u4r-r2.mjs:87).
- Result: 266 lines (target about 260), sha256 fd9e63b0…54b5.
