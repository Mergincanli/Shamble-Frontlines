# U6a build review, test seat (round 1 of 1, NPT-TRIM), 2026-11-22

Inputs: u6a-brief.md rev 1.1 (sha256 c109691a...), wip/u6a-build.patch (sha256 a9f8b6e0...1d39; base aa5c62b6), built
tree /home/user/Shamble-Frontlines/.claude/worktrees/agent-a95ebf14368f277ff (read only).
W = /home/user/Shamble-Frontlines/.claude/worktrees/agent-a95ebf14368f277ff/spikes/netcode-proof
One question: is the unit proven, as section 3 (amended by 0.1) says it must be.

## Plan
1. Check the two input hashes (brief, patch).
2. Counts: list every new #[test] in the patch by group; compare to section 3's groups (T6 2, TickLoop 10, binary 5,
   S4 8, -2 4, npt-super 2, C-U6-17 1, C-U6-18 1; +3 bites); read each test's asserts against the group's words.
3. Plants P1-P11, P7b: read each log (failing test name, failure reason, not compile error), restore hashes vs clean.
4. Bites: binary.rs's double-load test and its "npt-super form" log vs super/src/main.rs log(); no-smt-map and
   shared-vcpu bite tests; the judge's checks by name.
5. Timing: every test that reads a real clock or sleeps; injected clock where a time is asserted; 5 s cap; OPEN time
   reported; flakiness risks by test name.
6. C-U6-17 carry test and C-U6-18 order test against section 4's words.
7. Header truthfulness: F4 (stand_ins [], burn 0) and carry 60 params asserted by a test?
8. Item 7: -2 everyone flag parity; /proc/stat failed read -> steal 0.
9. SERIES.md rows (condition 1's list) written and equal to the code's keys.
10. Section 3's not-verified list gaps.
11. Verdict, findings table, not verified, DONE.

## Notes (one line per finding, as found)
