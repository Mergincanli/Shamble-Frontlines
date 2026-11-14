# Advisor: U4r judged sets, round 2 (P0 results; -17 L1 P0 INVALID; tick16 flagged bite; closing U4r's judged runs)

Started 2026-11-14. Seat: day-session advisor agent (read-only shell; this notes file only).

## Plan
1. Read advisor.md from main (done), advisor-u4r-sets-r1-full.md (my step 7, BRAWL-1), advisor-u4r-f2-head-full.md.
2. Check fact 1: judge exit codes, sha256s vs P3/B1/P2, runs.txt lines (fixed line, judge lines, raw-logs line).
3. Check facts 2-3: verdict.json and verdict-bites.json for P0 (python3 -I print-only).
4. Check fact 4: kept log summary lines.
5. Check fact 5: fingerprint script and its output; how pre:PM-0001-2 is defined (marks.json, judge code); what
   "everyone in view" measures; whether the brawl onset is the same BRAWL-1 cause seen in P3/B1/P2 or a second cause.
6. Check fact 6-7: judge-apart logs, peak, free space.
7. Read code: how P0 issues commands (one per tick), the brawl/visibility mechanics, to rule on cause.
8. Rule Q1, Q2, Q3; findings; next steps; to Mustafa.

## Checks (a line per check)
