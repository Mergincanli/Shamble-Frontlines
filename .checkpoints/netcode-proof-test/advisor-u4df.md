# Advisor: U4d-F round 1 (the estimate scene fix), 2026-11-12

Seat: the advisor agent, called from the day session's workflow (claude/netcode-proof-test-plan-hcky5g, head 1148c45;
PR #40). Role read whole from main (`git show origin/main:.claude/agents/advisor.md`, 334 lines). I rule; the main
session applies every text change and builds the fix pass. I write only this file, with the Write tool, block by block.

## Plan
1. Read the inputs whole: the brief (done), both reviews (done), the builder's record (done), my U4d close ruling
   B1.1-B1.3 (done). Then: the card's "Running" (item 4), the build brief's TE2-5 clause and B2.3 (item 4), U4j's
   M1-M9 (item 5), carry 59's text (NF-L4), the reviewers' briefs' write rule (item 6).
2. Spot-check at the copy: the diff's file list, scene.rs's 40 and its doc, t_m1's assertions, the reckoning's lines in
   `synthetic`, 3b's switch; and the snapshot (wip/u4df.patch at 9dd037a) against the copy's diff.
3. Spot-check the logs behind the rulings: nc-probe.log and nc-long.log (NF-M1), te-plant-p1.log and te-plant-p3.log
   (TF-M1), end-check4.log / end-check5.log / te-check.log (the counts).
4. Rule items 1-6 in order, one block each, then the two lists and "What I checked, and what I did not".

## Progress
- [x] 1 read  - [x] 2 spot-checked (facts cited in each block)  - [x] 3 logs  - [x] R1  - [ ] R2  - [ ] R3  - [ ] R4
- [ ] R5  - [ ] R6  - [ ] the texts (T1-T3)  - [ ] the end sections.

## Rulings

### R1. NF-M1 (Medium) with TF-L2 (Low): 40 stays, the acceptance stands as met, the report says what the level and the halves are, and a later re-measure judges by RM. Nothing here needs Mustafa's word. APPROVE.

**The finding holds.** I re-derived the netcode-architect's 36-minute figures from nc-long.log's own per-minute counts
(seeds 1-9, 23:26:56-23:27:26 UTC, exit 0): the nine seeds' mean per 300 s block from minute 2 is 10.096, 9.692, 9.941,
9.901, 9.799, 9.857 and 9.887 (minutes 32-35); the slope over minutes 2-35 is -0.0032 a second per minute around a mean
of 9.882; all 27 ten-minute windows (minutes 2-11, 12-21, 22-31) are in band, the lowest 9.537; 6 of the 54 single
300 s blocks are out of band; and seeds 1-3's minutes 2-11 equal the official windows (9.878, 9.743, 10.098). The
test-engineer's run through the test's own `EsRun` (te-probe-seeds4-9.log) gives seed 6's second half 9.446, the same
figure. The label stays Medium.

**Why 40 stays:**
- The acceptance as I ruled it (B1.2 step 4) is met on seeds 1, 2 and 3, and three independent runs agree on every
  figure (the builder's s6-three-seeds.log and end-three-seeds.log, the netcode-architect's nc-suite.log, the
  test-engineer's te-base-suite.log). The rule for choosing the share (the largest smallest margin) was written down
  before the sweep chose, and no band moved. Nothing in the acceptance changed after the fact.
- What the finding shows is that my per-seed half check is noisier at this level than I assumed when I wrote it: a
  300 s half's noise is about 0.2-0.36 a second, so at a level of 9.88 one half in about 9 misses by noise, and three
  seeds' six halves all pass only about half the time. That makes 40 one passing set of halves, not a margin. It does
  not make 40 wrong: the windows, the quantity PM-0001-2 judges run by run ("on every ES run", ADR 0001:1480), are all
  in band on seeds 1-9 (9.693 to 10.171; spread 0.166 across seeds).
- A re-tune now would buy little and cost a re-pin. The share moves only the aimed hits (about 0.7 of ES's 9.9 events
  a second at 40; blasts and sky shots meeting the shooter's chest make the rest), so 36 or 38 would lift the level by
  about 0.1 at most, would be chosen on the same noise, and would re-pin ES's recording and widen round 2.

**Does any of it change the acceptance in a way that needs Mustafa's word? No.**
- His words bind two things: the scene is fixed before U4d2 pins the recordings, and PM-0001-2's 9.5 floor does not
  move. The drift is fixed: before U4d-F the rate fell from about 11.5 to about 4.3 a second within 12 minutes in every
  seed (measurement A, s4-probe-11.log); after it, the rate is flat over 36 minutes and nine seeds. The floor is
  unchanged in every check, now and in RM: every judged window lies in 9.5-10.5.
- The per-seed half check is mine (B1.2 step 4, "Against drift"), not his and not PM-0001-2's. RM below replaces it,
  for later re-measures only, by a drift check on the nine seeds' mean. That changes my rule, not his floor: RM keeps
  the floor run by run on every seed's window and widens it from 3 seeds to 9.
- t_m1, the counted test (seed 2), keeps its half check exactly as built. For given code it is deterministic, and it
  passes today (halves 9.880 and 9.606, window 9.743). Its job from now on is the tripwire that sends a later change
  that moves ES to RM; it is no longer the reckoning's guard (R2, FX-1).

**TF-L2, my erratum.** B1.2 step 4 says "A stationary generator makes the warm-up's length immaterial; that is what
this step shows." The step did not show it: inside the judged window the second half is lower than the first in 8 of
9 seeds (mean -0.404 a second; the nine seeds' halves average 10.096 and 9.692), the same in every seed, and the dip
does not recur after minute 12. The netcode-architect reads it as the hulls' loops, which run on the World's elapsed
ticks from zero in every run, so every run's minutes 2-11 see the same arrangement; I accept that reading as likely,
not proven. What the step did show is that the decay is gone. The report says so (T1), beside NF-M1's numbers.

**How a later re-measure judges drift: RM (carry 67; its words in T2).** In short:
- **When:** before the merge of any change that moves ES's synthetic recording (D-7's ES digest) or the World's
  outcome on it (t_m1's printed figures), and at once whenever t_m1 fails.
- **Runs:** seeds 1-9, each a 36-minute ES recording (64,800 ticks from tick 1,000,000), counted as `EsRun` counts.
- **The floor, run by run:** each seed's window (minutes 2-11) meets all four bands, with 48 soldiers and 15 vehicles
  on every tick and every hull within 1,400 cm.
- **Drift:** the nine seeds' mean gameplay-event rate over each 300 s block from minute 2 to minute 31 lies in
  9.5-10.5 (today: 10.096, 9.692, 9.941, 9.901, 9.799 and 9.857, so today's scene passes RM). The slope over minutes
  2-35 is reported beside it.
- **Reported, not judged:** every seed's two halves.
- **The share** is re-set only when the floor or the drift check fails at the current value, then to the value whose
  nine-seed mean window lies nearest 10.0 with both passing, never on three seeds' margins; the numbers come to me
  before D-7 or t_m1 is re-pinned for it.
- **t_m1 failing while RM passes:** the session changes neither the share nor t_m1 and brings RM's numbers to me; I
  rule on t_m1's form then.

R1 written 2026-11-12.
