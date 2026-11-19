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
  52" is the run's second (ADV-U4F20-3). Margin 1,600 / 1,117 = 1.432.
- A's presses above 432,000 (= 48 x 18,000 / 2, B's exact figure): A counts presses by the tick a command runs, not its
  stamp, so commands stamped in warmup run inside measured tick 300 (A: 28 and 30 presses at tick 300, B: 22; per-phase
  sums A s1 warmup 6,784 / measured 432,003 / after 841). A window-edge effect, not a doubling; the report should say so
  (ADV-U4F20-3).

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

## Block 3: the mechanism (the code, read)
- **The room's count** (room/src/room.rs:1330-1349, `run_one`): per button, `edges = counter - last run counter`;
  seen += edges, lost += edges - 1, doubled += edges on option B's repeat. Its doc (:353-356) says why: the sim takes
  one edge from a changed counter. The count is a true count of what the sim does.
- **The sim's rule** (sim/src/world.rs:1340-1346, U4d's, commit 39f4290): `edge[k] = c.presses[k] != claimed[k]`, one
  edge whatever the advance, justified in its comment by "(the room allows one per button per tick)".
- **That premise is false since U4f** (room/src/input.rs:285-300, kind 5): the room accepts "at most one edge per button
  for each tick the command covers", `covered` measured from the newest *accepted* command. Option B's late commands
  return `Offered::Late` (:307-314) before `self.accepted = Some(c)` (:340), so the next on-time command covers 3 ticks
  and its 2 edges are accepted; the sim then takes 1.
- **Option B itself** (input.rs:360-384, `take`; room.rs:1071-1089) runs exactly one command per tick, the stamped one,
  else the last again, and drops commands stamped for a tick already run: ADR 0001:1159-1161's (b) as written.
- **ADR 0001 part 5** (:225-226): a command holds "a press counter per button (a resent command can neither lose nor
  double a press)"; (:234-235) "a press counter advances by at most one edge per button for each tick the command
  covers"; (:235-237) "fire, ability and reload edges meet the server's own cooldown, ammunition and reload state". So
  the ADR's (b) drops the late *command*, not its *press*: the press rides the next command's counter, and the
  authority is to take each edge and judge it by its own state. The stand-in's one-edge rule is what loses it.
- **Why A shows 0:** A runs each late command on its own in the tick it arrives (input.rs:386-393; room.rs:1093-1107),
  so every counter advance is 1. A command lost in every one of its 8 copies would give A the same multi-edge case; at
  P3's 5 % that is about 0.05^8 per command, so it never shows.
- **What the fix would change** (my reading of world.rs:1408-1466, not run): with fire held, a second fire edge in one
  tick meets the cooldown (a shot sets `wait` above 0) and is refused; "refused input leaves no trace in the state"
  (world.rs:30-35). So taking every edge leaves the brawl's state, hashes, events and queue as they are; it changes the
  room's lost count (to 0) and the sim's cooldown refusal reports (+1 per such tick). Expected, not proven.
- **Where else it reaches:** PM-0001-58's "0 presses lost" (marks.json: parts L1 P0 and L1 P3, suite `wrap`, no
  `cmd_timing` select; the lab's default is B, lab/src/scenario.rs:2124-2125). Its `wrap` suite is U4g's and not built
  yet; if it scripts presses under B, it meets the same rule at P3 whenever two commands in a row arrive late: at
  -20's rate (6 in 3 x 600 s for 48 clients) a 3 x 10 s set would meet it about one time in ten, a FAIL with no wrap in
  it. R3's L2 part of -20 under B (VM time) meets it too. -21's checks read no press series; -22's runs carry no press
  edges (Block 5).

## Block 4: the reorder share (R20-Q4)
- Every L1 header here carries `params.reorder_step_permille` 10 (the configured step), never `reorder_permille`: the
  header is written at the log's open, before any share exists (advisor-u4f-fold-check.md:52-58, D3-1).
- The measured share is the lab's end-of-run `reorder` event (lab/src/scenario.rs:1747-1757: `permille` the lower of
  the two directions, RFC 4737 reordered per 1000 delivered): A and B alike per seed, s1 100 (up 160, down 100), s2 101
  (161, 101), s3 101 (160, 101), one event in each lab.jsonl, none in room or client logs.
- The validity rule reads the header param *or* the event (judge/src/eval.rs:864-881), so `reorder` holds on all six
  runs and ADR 0001:598's "each stream's measured reorder share is above 0" is met (the lower of the two is above 0).
- The report row (marks.json:4786-4790) reads only `header.params.reorder_permille`, so it prints "not available". D3-1
  routed the validity to the event and left the report row on the header form. Not by design: a gap in marks.json's
  report row, which -58's row (:4265) shares for its L1 runs; -3's probe writes the header form (lab/src/main.rs:136-139)
  and is unaffected. The fix is a marks.json (or judge) change: Mustafa's under TEST-FIRST. Nothing is blocked:
  -20's report states the event's figures by hand. ADV-U4F20-2.

## Block 5: -22's set (R20-Q3; the coordinator's update, checked)
- results/u4f/7192607d99a1/pm-0001-22/: verdict.json's part "L1, one never-acker among 48" PASS, l1-22-s1..s3 valid
  (the mark NOT-RUN in that call only for its bite, by the split's design); verdict-bites.json: PM-0001-22/unbounded
  shown (valid; headroom -3,922 against == 0); verdict-load.json: PM-0001-2 FAIL on "a connected snapshot-receiving
  client on every soldier" = 47 on each run (the never-acker kicked), as designed and never counted.
- `git diff --quiet 25d316e 7192607 -- spikes/netcode-proof ':(exclude)spikes/netcode-proof/results'` exit 0, and the
  same for 350677f: -20 and -22 ran on one tree.
- **N1's line, checked:** lab.jsonl `client_counts` on each measured run: slot 0 alone has overflow resyncs 1, kicked
  1 and rewinds refused 33 / 39 / 37; every other slot 0 on all three. Slot 0 is the never-acker
  (lab/src/scenario.rs:851-852, :1242-1244). No honest client has an overflow resync in -20 or -22: 1,600 stands.
- **Untouched by ADV-U4F20-1:** -22's args (c-u4f-runs.mjs:50) carry no `--brawl-press-every` and its bots are brawl
  kind, so no command has a press edge (bots/src/lib.rs:639-645: `every` 0); the one-edge rule never runs.

## Rulings
- **R20-Q1: neither of the two, as the read puts them. The count is honest; the cause is the stand-in, not option (b).**
  - The measurement holds: the six ticks reproduce, two by hand (Block 2), and the room counts what the sim does
    (Block 3). The bar was held and judged exactly.
  - It is not a property of (b) as ADR 0001 defines it. (b) drops the late *command* (:1159-1161); part 5 keeps the
    *press*: "a press counter per button (a resent command can neither lose nor double a press)" (:226), with up to
    one edge per button for each tick a command covers (:235), each edge meeting the server's own state (:236-237).
    The press is lost by npt-sim's one-edge rule (sim/src/world.rs:1340-1346, U4d's), whose premise "(the room allows
    one per button per tick)" stopped being true when U4f's kind 5 (room/src/input.rs:285-300) began accepting a
    command that covers 3 ticks with 2 edges. So it is a defect of the stand-in that U4f's build exposed (ADV-U4F20-1).
  - **Crate and fix:** npt-sim takes every edge a counter's advance carries, each through the same cooldown,
    ammunition and reload checks (a refused one counted as now, leaving no trace); npt-room's lost count
    (room.rs:1338-1345) comes from the same rule, defined once, so the two cannot drift again; world.rs's comment is
    corrected. Tests: an option-B room test with two late drops before a 2-edge command gives lost 0 and seen 2; a sim
    test that the 2-edge command leaves the same state hash as the 1-edge build (the second fire edge a cooldown
    refusal: expected state-neutral, Block 3, so prove it rather than assume it).
  - **Re-runs:** -20's L1 set whole (`c-u4f-runs.mjs 20`: 7 runs, about 41 min, no money) on the first commit that holds
    the fix, judged as before; that verdict is -20's L1 record. The 350677f folder stays committed as the record of the
    defect. The bite should stay shown (its on-time 0.894544 does not depend on the edge rule). -22 does not re-run.
  - **When:** in U4f2 (next and small; -21's judged runs then run on the fixed build) or as a fix step before it; at the
    latest before U4g's -58 runs and before R3.
  - **For the OQ-0001-6 reading:** the 350677f B FAIL is not evidence against (b). The reading states the mechanism
    with the six ticks and the rule it found: after dropped late commands, (b) runs a command whose counters carry
    several edges, and the authority must take each one (the foundation's sim inherits this whichever option is
    picked; (a) meets it after a command lost in every copy). -20 then separates the options by buffer depth only
    (A 0 / 0, B median 2, max 5), as NC-U4F-5 already asks. The pick stays R3's (plan.md:846-848): under OQ-0001-6's rule
    (ADR 0001:1164-1166), (a) at buffer 0 wins if it passes the pick's other marks, so -20 decides between them only if
    (a) fails one of those.
- **R20-Q2: nothing needs his word.** No pass mark, judge rule or marks.json number changes; no money; nothing on
  main. The fix and the re-run are inside the plan (U4's stand-in; -20's planned set; plan.md:1285's "its cause is
  found before the next run under that mark": found here), and both can be undone. So rule 2: by this ruling, recorded
  with its reason and named in the next day's message as a result. ADV-U4F20-2's report-row fix is a marks.json change,
  so it is his; it blocks nothing, so it goes on rule 1's list, named in the same message.
- **R20-Q3: -22's set stands.** It ran on the same tree (Block 5), its runs carry no press edges, so the fix cannot
  reach them, and its PASS, bite and load report read as the coordinator states. N1's line holds: the refused rewinds
  (33 / 39 / 37), the overflow resync and the kick are all slot 0's, the never-acker's. With no honest overflow resync
  in -20 or -22, 1,600 stands; the 1.43 margin is named for U4g as advisor-u4f-stop.md:132-134 sets.
- **R20-Q4: a gap, not by design** (Block 4). Validity holds through the `reorder` event; the report row reads a header
  param L1 never writes. -20's report states the shares by hand from the event (lower 100 / 101 / 101 per mille, up
  160 / 161 / 160, down 100 / 101 / 101, A and B alike). The fix to marks.json's row is his (ADV-U4F20-2).
- **R20-Q5: yes.** Carry 74's B reading (u4s-carries.md:355: "the room's `late` count) beside the commands sent") is
  the session counters against the bots' commands: 142 / 879,392 = 0.0161 %, 114 / 879,452 = 0.0130 %, 123 / 879,387
  = 0.0140 %, far under carry 74's 1 %, so B's marks count. Say beside it that the counters cover the whole run (warmup
  6 / 2 / 3 included) and the ticks' measured sums are 136 / 112 / 120. Option A by its own reading (not on time by stamp,
  advisor-u4f-brief.md:148-149): 0.0157 %, 0.0130 %, 0.0139 %, also under 1 %.

## Findings
| Id | Severity | Where | What goes wrong | Fix |
|---|---|---|---|---|
| ADV-U4F20-1 | High | spikes/netcode-proof/crates/sim/src/world.rs:1340-1346 (with room/src/room.rs:1338-1345, room/src/input.rs:285-300) | The sim takes one edge from a changed counter; the room accepts a command carrying one edge per covered tick, which option B produces after late drops. Presses are lost by the stand-in, not by (b): -20's B part FAILs for a cause outside its subject; -58's P3 "0 presses lost" (if its suite presses under B) and R3's L2 B run of -20 (VM time) would meet it too | R20-Q1: every edge taken, the room's count from the same rule, the two tests, -20's L1 set re-run on the fixed commit; in U4f2 or before it, at the latest before -58's runs and R3 |
| ADV-U4F20-2 | Medium | spikes/netcode-proof/marks/marks.json:4786-4790 (-58's twin row :4263-4267) | The "measured reorder share" report row reads `header.params.reorder_permille`, which L1 scenario runs never write (the share is the `reorder` event since D3-1); the figure prints "not available" though it was measured | Mustafa's (marks.json): rule 1's list; until then -20's and -58's reports state the event's figures by hand (the row at :4263-4267 is -58's) |
| ADV-U4F20-3 | Low | .checkpoints/netcode-proof-test/u4f-20-read.md:42-44, :17 | "measured second 52" is the run's `sec` (the 43rd measured second); A's presses 432,003 / 432,009 sit above the scripted 432,000 with no line why (A counts by the tick a command runs: the window's edge, Block 1) | -20's report says both |

**For Mustafa:** nothing to decide. One line in the next day's message: "-20 at 350677f: option A PASS; option B FAIL on
'0 presses lost' (3, 1, 2) because the stand-in sim took one press per changed counter, not because of option (b);
npt-sim fixed and -20's L1 set re-run (no money) by the advisor's ruling; -22 PASS stands; a marks.json report-row gap
(the reorder share) is on the after-close list, yours."

