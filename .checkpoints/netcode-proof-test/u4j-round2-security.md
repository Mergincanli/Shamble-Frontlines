# security-auditor, U4j round 2 (the fix pass on round 1's F1-F12)

## Plan
1. Read the review packet (u4j-round2-packet.md) in full: diffs, plan's U4j section, rulings FX-1..FX-14, round-1 findings.
2. F1 first: read the fixed practice.rs `find` and the factor's computation in the fixed copy; build a table of every
   path (IN TOLERANCE, BEYOND with/without chance ruling, FAR BETTER, a second practice run, a bite run) with f and
   file:line; check FX-6 (pinned bound line), FX-1 (band check), FX-8 (6-decimal cap) for any log input that raises f.
3. F2-F6, F9 first half (FX-11): closed or not, with evidence; new paths (hostile log: panics, overflow, duplicate or
   missing fields).
4. Run the judge's tests through the runner (exit code + test summary line).
5. Verdict.

Copies: pre-fix .gate/u4j-prefix; fixed .claude/worktrees/agent-a3b8dfeb01ed52b50 (never edited).

## Notes (a line after each finding)
