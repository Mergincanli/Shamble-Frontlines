# U4f review panel, round 1: seat netcode (netcode-architect)

Tree: /home/user/Shamble-Frontlines/.claude/worktrees/agent-a0825a5a2491eb175 (HEAD 07289a4, U4f uncommitted). Read only.

## Plan
1. Read the scope: history.md:6896-6903 (SIZE-U4F (a)), advisor-u4f-stop.md (R2, R3, S1-S7, N1, :80-89 the U4f2 list).
2. Read u4f-brief.md 2.1-2.9 (clock, lead controller, command timing, checks, never-acker, cap, carry 80), 2.13, 5.6; advisor-u4f-build1.md R2 (g).
3. Read ADR 0001's clock / lead sections and plan.md §4.8 for the controller's contract.
4. Read the diff in the worktree: the client clock, the lead controller, the room's tick loop and command intake, the never-acker, the cap, the Kick path (carry 76).
5. Check the room's per-tick order and the lead bound counted from the tick the room runs next.
6. N1: hand-s22's 30 refused rewinds (w2-hand.log:32, :53): whose and why, from logs and code.
7. Runs only to prove a finding, through u4f-run.sh, logs under .gate/u4f/review-netcode/.
8. Verdict and findings table at the end.

## Findings (one line each, as found)
