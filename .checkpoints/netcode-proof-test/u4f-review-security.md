# U4f review panel, round 1: seat security (security-auditor), 2026-11-19

## Plan (what I read, in order)
1. Mustafa's SIZE-U4F words (history.md:6896-6903) and the stop ruling's U4f2 list (advisor-u4f-stop.md:80-89) and
   S1-S7/N1 (:198-210), so I report nothing out of scope and answer S1 against my SA3-7.
2. u4f-brief.md 2.4 (kinds and order), 2.5 (counters), 2.6-2.8 (cheat suites, never-acker, cap), 2.9 (carries), 2.10
   negative cases, 4 build rules; advisor rulings L1-L6, build-1 R1-R4/F1-F11.
3. The patch's file list (git apply --numstat), then the room-side command checks in the worktree: decode path, kind
   order, clamp points, seq/tick checks, counters, never-acker, the 1,600 cap, carry 76 Kick path.
4. The client clock and lead controller only for what a client sends the room trusts (stamps, ticks, Ping/Pong).
5. npt-bots' cheat suites (clamp, lag switch, fast clock) and forgers 1,2,4,5,6: do they claim -21 before U4f2?
6. Runs only to prove a finding, through the runner, logs under .gate/u4f/review-security/.
7. Verdict, findings table, coverage table, own lines, runs, not checked.

## Progress
