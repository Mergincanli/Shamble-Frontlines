# Advisor: U4r build, copy 2's points (ruling 2)

Started 2026-11-13 (after 21:51 UTC). Advisor agent file read from origin/main (335 lines).
Code read only in /home/user/Shamble-Frontlines/.gate/u4r-snap1 (2fbec77 + wip/u4r.patch).
Hashes checked: u4r-brief.md 59d570b8a73f... (matches); wip/u4r.patch 48fa2959799d... (matches).

## Plan
1. Read: u4r-build.md whole (569 lines; copy 2's part :284-569), my ruling 1 (advisor-u4r-build1-full.md and
   advisor-u4r-build1.md), my brief approval (advisor-u4r-brief.md: L1-L6), the brief's cited lines (7.2 :1193-1200,
   :1306-1311, :1358-1362, params :361-364), Mustafa's decisions in history.md as they bear on U4r.
2. Point 1 DEPTH: read input.rs (the measure, :28-29), the room's take, precheck.rs's depth histogram; decide L vs L - 1.
3. Point 2 LOGS: read precheck.rs result lines, the brief's records list; decide whether a per-leg log is required.
4. Point 3: (i) Cargo features of npt-server / npt-client and tests/loopback.rs; (ii) r3's test temp folder;
   (iii) log header commit naming vs uncommitted code; ruling 1's point 4 main-session step.
5. Point 4: conformance read of copy 2's Builder lines (notes :421-540) against the fixed copy's code.
6. Point 5: anything else in copy 2's part.
7. Write each point's block here as done; then return StructuredOutput.

## Blocks
