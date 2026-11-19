# U4f's -20 set: the main session's read, for the advisor's ruling (2026-11-19, about 20:55 UTC)

## The run
- `flock -x .gate/shared-target/.lane.lock node spikes/netcode-proof/ops/runs/c-u4f-runs.mjs 20 --date 2026-11-19`,
  repo root, CARGO_TARGET_DIR unset; 20:07:24 to about 20:48 UTC, exit 0; at 350677f, whose spikes/netcode-proof
  differs from 25d316e's only by -47's results (`git diff --quiet 25d316e 350677f -- spikes/netcode-proof
  ':(exclude)spikes/netcode-proof/results'` exit 0). Results: spikes/netcode-proof/results/u4f/350677f0aef2/pm-0001-20/
  (runs.txt, verdict.json, verdict-bites.json), committed at 1840e68 and pushed. Raw logs:
  .gate/netcode-proof-test/runs/u4f-20-350677f0aef2/ (measured/ and bites/). The set: SS, 48 clients, P3, U4f's
  controller, the brawl with `--brawl-press-every 2`, 600 s measured, seeds 1-3 under each option, the bite
  redundancy-1 under option B at seed 1 (c-u4f-runs.mjs:52).

## The verdicts (verdict.json; verdict-bites.json)
- **PM-0001-2, SS runs: PASS**, all six runs valid (the other parts NOT-RUN by design: ES, N=24, N=36, the bite).
- **PM-0001-20, L1 P3 option A: PASS** (3 of 3 valid): on time 0.999843 / 0.99987 / 0.999861 (bar >= 0.999); missing
  0.000157 / 0.00013 / 0.000139 (bar <= 0.001); presses 432,003 / 432,003 / 432,009 (bar >= 10,000); lost 0; doubled 0.
- **PM-0001-20, L1 P3 option B: FAIL** (3 of 3 valid): on time and missing equal to option A's per seed; presses
  432,000 each; **"0 presses lost" = 3, 1 and 2** (bar == 0); doubled 0. The L2 parts are R3's (NOT-RUN).
- **The bite PM-0001-20/redundancy-1: shown** (its run valid).
- Report figures: buffer depth per client, option A median 0 and max 0, option B median 2 and max 5 (ticks); "measured
  reorder share" reads "not available: no values for header.params.reorder_permille" on all six runs.

## Where each press was lost (scratchpad press_lost.py, read only, on room.jsonl's tick records)
Every lost press comes on a tick where one slot's command runs after that slot's commands missed the two or three
ticks before it (`tick.s.cmd_on_time` 0 on those ticks); no other slot missed the tick before:
- b-s1: tick 13594 (slot 26 missed 13592-13593), tick 13824 (slot 37, 13822-13823), tick 17417 (slot 16, 17414-17416).
- b-s2: tick 4441 (slot 45, 4439-4440).
- b-s3: tick 4446 (slot 7, 4444-4445), tick 4955 (slot 18, 4953-4954).
- a-s1: no tick lost a press (the same arrival pattern per seed: option A's on-time shares equal B's).
- The room's definition (room/src/room.rs:352-355): "lost: the edges beyond the one the sim takes from a changed
  counter (npt-sim world.rs: an edge is a counter that differs from the one it last claimed)". With a press every
  second command, two consecutive commands dropped late under option B leave the next command run with its counter two
  presses on; the sim takes one edge, so one press counts lost. Under option A the late commands still run, each in
  the tick it arrives, so every advance is taken one at a time.
- ADR 0001's option (b) (:1159-1162): "buffer them and run exactly one per client per tick, repeating the last on a
  miss, late commands dropped and catch-up limited to the redundancy window".

## The other figures -20's report owes
- **Overflow resyncs:** 0 on every client of all seven runs (lab.jsonl `client_counts`, 48 records each; the summary
  lines). So no honest overflow resync: 1,600 stands (advisor-u4f-stop.md:126-134), and -22's set started at 20:51 UTC.
- **The queue's 10-minute high-water** (`second.s.event_queue_max_count`, scratchpad queue_hw.py): A 866 / 1,117 /
  1,083, B 1,001 / 683 / 1,045, the bite 771; the largest **1,117** (A seed 2, slot 27, measured second 52); every
  run's maximum falls in measured seconds 30-53. **Margin 1,600 / 1,117 = 1.43, under 1.5: named for U4g**
  (advisor-u4f-stop.md:132-134). The 60 s figures beside it: cap.txt's P3 maxima 689 / 1,001, 683 / 449, 1,045 / 935.
- **F6** (advisor-u4f-r2.md:143-147): on the three option-B runs the session counters (lab.jsonl `client_counts`
  `late_count`, summed: 142, 114, 123) equal the room ticks' `tick.v.late_count` summed over all phases (142, 114, 123);
  over the measured phase alone 136, 112, 120. Against the commands sent (879,392 / 879,452 / 879,387: the summary
  lines) the counters give late shares of 0.0161 %, 0.0130 % and 0.0140 %.
- **NC-U4F-5 and C-OQ6-SEC-1** (advisor-u4f-r1.md:107, :133): for the OQ-0001-6 reading.

## Questions for the advisor
- **R20-Q1:** is option B's FAIL an honest property of option (b) as ADR 0001 defines it, measured on the stand-in's
  one-edge rule, or a defect in U4f's build (npt-room's option B, the lab, or npt-sim's edge rule) to fix before the
  result counts? My lean: honest; the result counts as U4f's -20 record (A PASS, B FAIL), and the OQ-0001-6 reading
  states the mechanism with the six ticks above. Under OQ-0001-6's rule (ADR 0001:1164-1166: "of the options that
  pass the proof test's marks ..., the smallest buffer, with (a), run on arrival, counting as none") it is evidence for
  the pick, which the plan makes at R3 with -48 and -49 (plan.md:846-848), not now.
- **R20-Q2:** does anything here go to Mustafa (a pass mark, the judge, marks.json, the plan's stops)? My lean: no; the
  bar is kept exactly and judged; the pick is the plan's later step. Named in the next day's message as a result.
- **R20-Q3:** -22's set (running since 20:51 UTC: P0, the never-acker, no presses) stands whatever R20-Q1 finds?
- **R20-Q4:** the report line "measured reorder share: not available" (no `header.params.reorder_permille` on L1): by
  design, or a lab gap to route?
- **R20-Q5:** F6's figure for carry 74's B reading: the counters against the commands sent, as above?
