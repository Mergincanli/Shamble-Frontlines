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

## Rulings

(written block by block below)
