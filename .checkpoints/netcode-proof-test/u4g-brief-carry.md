# Carried into U4g's brief: the advisor's words from BRAWL-1's round-1 ruling

Copied by u4g_brief_carry.py, word for word, from advisor-brawl1-r1-full.md lines 279-288 (sha256 86bec63bfffbd676efbc09e09c35a952e63abc7865e89b20b5aebec11bee75c5). Nothing below is the main session's.

## U4g's brief

These words go into U4g's brief, and the main session records them on the card and in U4r's report for U4g. They merge my ADV-PT-M1 with NA-B1-2 (c), and TE-B1-5's kinematic bound without its obstacle reading. The readings marked 'computed' were not run.

"BRAWL-1's pattern (iii), B1's bursts, in the sim-only test (printed only, never barred):
- Measured: in the full test (release, seeds 1 to 3, 610 s each), no Soldier record had area != 0. The largest distance from the centre was 1,832 cm in every seed, each at a burst's end, at (1734, -592), (1671, -751) and (1586, -917): three angles, one 1 cm band.
- Not a stop by a solid: the test runs with stand-ins off, and no solid lies within 1.6 m of those points. It is the leash's own reach under a 30-tick re-run. Computed, not run: about 1,830 to 1,840 cm. One reading takes the walk's 205 cm circle kept inside the 1,400 cm outer radius, plus 600 cm of re-run. The other takes a start just inside the 1,300 cm inner radius, plus 30 re-runs at about 18 cm. By kinematics the reach is at most about 2,000 cm: a soldier at the outer radius re-running 30 steps of 20 cm straight out.
- That reach passes the 1,800 cm door-entry radius of buildings 0 and 3, whose doors lie on the x axis (|y| < 60 for a soldier's centre). The door entries of buildings 1, 2, 4 and 5 lie at about 1,864 cm, within only the kinematic bound.
- The test is optimistic after a burst. It feeds each slot its World position from 8 steps earlier at once, while a real client may steer on its last complete pre-burst tick until a complete post-burst tick arrives.
- So 'area 0 in three seeds' is neither a bound nor a margin. A B1 judged run can carry a soldier into a door gap. That lowers -2's everyone-in-view share (bar 0.99), which makes -17's parts invalid (their validity is pre:PM-0001-2): BRAWL-1's INVALID again. The building figure on every B1 run is the check. U4g's plan says how a B1 building entry reads in -17, or measures how often it happens, before its judged B1 runs. One way to measure it is a printed door-gap margin per burst, TE-B1-5's option."
