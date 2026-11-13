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

### Read (done)
- u4r-build.md whole (569 lines); advisor-u4r-build1-full.md whole (169 lines) and advisor-u4r-build1.md; brief
  :330-420, :1135-1375; plan u4-build-plan.md :760-800; marks.json -30 (:6535-6580); SERIES.md :101, :240;
  room/src/input.rs :1-160; lab/src/precheck.rs whole (777 lines) in the snapshot.

### Point 1 DEPTH (done)
- Code: input.rs:28-29 doc "How many ticks ahead the newest buffered command was at the last take"; `take`
  (:120-150) removes the tick's command, then `depth = buffered.last().tick - tick` (:142-145). Pacing: precheck.rs
  :174-179 sends cmds with tick <= ran + lead at step 5 after tick `ran`; at take j the newest received was sent at
  instant j-1, stamped j-1+L, so depth L-1 by definition, both legs (Loopback: next instant is the next due tick;
  L1 P0: arrival next ns, before the take). Copy 2's figure {0: 48, 1: 5,568} per leg (5,616 = 48 x 117).
- precheck.rs:369-373 reads depth for every client in first_run after each tick, first takes included (the 48 zeros
  are the first takes, indistinguishable from a later 0 in a histogram). :568-569 prints "lead L
  (buffer_depth_ticks_count L, both legs)". r6's doc (:694-696) says "the room's measured depth the lead" but the test
  asserts no depth (:715-720 prints only): doc false and unasserted.
- marks.json -30 report "input buffer depth set for P0 (ticks)", series header.params.buffer_depth_ticks_count, stat
  max; SERIES.md:240 "-30's input buffer depth set for P0". The param is the SET value; under option B what is set is
  the pacing lead L. SERIES.md:101 `second.s.buffer_depth_count` is the room's measured series (-20's).
- Ruling: (a) with an exact comparison: header/param value L; the room's measure reported beside it; the expected
  relation is measured = L - 1 at each take after a client's first command run; report every take where measured is
  not L - 1, by (client, tick, measured), lossless (first-take depth per client on its client line, histogram over
  later takes, the deviations listed); r6 asserts it. (b) rejected: L - 1 is an observation, not what was set, and it
  would make -30's "depth set" read 1 at P0 while the replay sends 2 ahead. Words changed: the brief's 7.2 reading
  ("report any tick ... where the two differ") is the main session's brief text, approved by me; I amend it. Not
  Mustafa's. Severity Medium (as built the report's comparison would flag every tick, or say nothing; r6's doc false).
