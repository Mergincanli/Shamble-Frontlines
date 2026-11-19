# Advisor ruling: U4f's judged -20 set (2026-11-19, from main's advisor.md)

Read only. Sources: `.checkpoints/netcode-proof-test/u4f-20-read.md` (the main session's read), the committed results at
`spikes/netcode-proof/results/u4f/350677f0aef2/pm-0001-20/`, the raw logs at
`.gate/netcode-proof-test/runs/u4f-20-350677f0aef2/measured/`, marks.json's PM-0001-20 entry, room.rs, input.rs,
world.rs, ADR 0001 OQ-0001-6, plan.md's option pick. -22's set finished at 21:17 UTC (coordinator's update).

## Plan
1. Verdicts and figures: runs.txt, verdict.json, verdict-bites.json against the read; marks.json :4645-4800.
2. Lost presses: run press_lost.py (`python3 -I`), then spot-check at least two ticks by hand in room.jsonl.
3. Mechanism: room.rs press accounting (:350-356, where `presses` fills), world.rs edge rule, input.rs late drop,
   ADR 0001 :1156-1167, plan.md :728, :843-848.
4. Rule R20-Q1 to Q5; findings ADV-U4F20-n; what goes to Mustafa (TEST-FIRST, history.md:2234-2282).

## Block 1: the verdicts and figures (checked against the files: all hold)
- runs.txt: seven summary lines; A late 0, B late 142 / 114 / 123; overflow resyncs 0 on all seven; press refused 0;
  commands 879,392 / 879,452 / 879,387 per seed (equal under A and B: `summary.commands` is the bots' `given()`,
  lab/src/scenario.rs:1706, so "sent" is right).
- verdict.json: PM-0001-2 "SS runs" PASS, 6 of 6 valid, no non-pass check; ES, N=24, N=36 NOT-RUN (by design).
  PM-0001-20 option A PASS 3/3 (0.999843 / 0.99987 / 0.999861; missing 0.000157 / 0.00013 / 0.000139; presses
  432,003 / 432,003 / 432,009; lost 0; doubled 0); option B FAIL 3/3 valid, only "0 presses lost" fails (3, 1, 2),
  on-time and missing equal to A's per seed, presses 432,000 each; L2 parts NOT-RUN. Report: depth A 0/0, B median 2,
  max 5 on each seed; reorder share "not available" on all six.
- verdict-bites.json: PM-0001-20/redundancy-1 shown (valid; on time 0.894544, missing 0.105456, lost 3,614; header
  `cmd_timing` B, as c-u4f-runs.mjs:52 sets).
- marks.json :4645-4800: the five checks read room's `tick.s.cmd_on_time` (share ==1, >= 0.999), `tick.s.input_missing`
  (<= 0.001), `tick.v.press_count` (sum >= 10,000), `tick.v.press_lost_count` (sum == 0), `tick.v.press_doubled_count`
  (sum == 0); validity `reorder` and per-part `pre:PM-0001-2`; report rows `second.s.buffer_depth_count` p50/max and
  `header.params.reorder_permille` max. Bars as the read states.
- My own counts from the raw logs: client_counts 48 records per run; overflow 0 on all seven runs; late sums 142, 114,
  123 (B), 0 (A, bite). Room ticks' `tick.v.late_count`: warmup 6 / 2 / 3 + measured 136 / 112 / 120 = 142 / 114 / 123
  (F6 holds). Late shares 0.0161 %, 0.0130 %, 0.0140 % (recomputed). Queue high-water (my own pass over
  `second.s.event_queue_max_count`): A 866 / 1,117 / 1,083, B 1,001 / 683 / 1,045, bite 771; largest 1,117 at A seed 2,
  slot 27, record `sec` 52, which is the 43rd measured second (measured starts at `sec` 10): the read's "measured second
  52" is the run's second (ADV-U4F20-4). Margin 1,600 / 1,117 = 1.432.
- A's presses above 432,000 (= 48 x 18,000 / 2, B's exact figure): A counts presses by the tick a command runs, not its
  stamp, so commands stamped in warmup run inside measured tick 300 (A: 28 and 30 presses at tick 300, B: 22; per-phase
  sums A s1 warmup 6,784 / measured 432,003 / after 841). A window-edge effect, not a doubling; the report should say so
  (ADV-U4F20-4).

## Block 2: the lost presses (press_lost.py run with `python3 -I`; two ticks checked by hand: both hold)
- The script (read it first: it opens room.jsonl only, read-only) gives exactly the read's six ticks: b-s1 13594 (slot
  26, 13592-13593), 13824 (slot 37, 13822-13823), 17417 (slot 16, 17414-17416); b-s2 4441 (slot 45, 4439-4440); b-s3
  4446 (slot 7), 4955 (slot 18); a-s1 none. Each tick: lost 1, `tick.v.late_count` 2 (3 at 17417).
- **By hand, b-s1 ticks 13589-13596:** `cmd_on_time` has 48 entries; slot 26 is 0 (and `input_missing` 1) on 13592 and
  13593 only. `press_count` runs 26/22 on odd/even ticks; 13592 reads 21 (slot 26's press not run: the repeat has 0
  edges), 13594 reads 23 with lost 1 and late 2: slot 26's command for 13594 carries its counter two on from the last
  one run (13591's), the room counts 2 seen, 1 lost.
- **By hand, b-s2 ticks 4436-4443:** slot 45 is the only slot not on time on 4439 and 4440; 4439 reads 19 presses
  (20 expected), 4441 reads 21 (20 + 1) with lost 1, late 2.
- **The same ticks under A (a-s1, 13590-13596):** slot 26 is not on time on 13592-13593 too (the same arrival pattern);
  13594 runs the late commands one by one: press 30, lost 0, late 0.
- lab.jsonl: slot 26 (b-s1) late 6, slot 45 (b-s2) late 3, overflow 0, press refused 0.

