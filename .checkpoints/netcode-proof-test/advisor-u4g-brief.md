# The advisor's check of U4g's brief, draft rev 1 (2026-11-20)

Seat: a day session's advisor (general agent acting on main's `.claude/agents/advisor.md`, read with `git show
origin/main:`), branch claude/netcode-proof-test-plan-hcky5g at 8e89a72 (spike tree 2b7647ff…, equal to 391adda's).
Read only; this file is my one write; no cargo, check.mjs, npt-lab or judge run; no commit.

## Plan
1. Read the brief (247 lines), its notes and u4g-brief-carry.md whole; TEST-FIRST (history.md:2234-2282), SIZE-CHECK
   (:2048-2051, :2067-2075), RECS-1043 (:2204-2227), the C62H-BOUND routing (:2477-2496), ADV-CBR-1 (:6425-6429).
2. Check the claims against the code at the head (crates/sim, client, room, proto, lab), marks.json's nine marks,
   SERIES.md's rows, the plan (§4.14, §5 U4g, §9), U4f's and U4f2's briefs, the stop ruling, U4f's report.
3. Look for U4g carries the brief misses: every place that routes work to U4g (history, u4r-report, rulings).
4. Size against SIZE-CHECK and U4f's real overrun; frame U4G-Q1 and U4G-Q2 for him.
5. Rule U4G-Q3 to Q8 (and anything new), findings ADV-U4GB-n, verdict.

## Notes (one line per check, written as I go)
- Stamps: marks.json c1935a5b… 12,630 lines, SERIES.md 7f11afbe… 823 lines, spike tree 2b7647ff…: as the brief says.
- Code claims, each read at the head: ClientSettings coded (proto/src/msg.rs:124-126, :471) and read by no room code;
  `World::predict` (sim/src/world.rs:1047-1068); own-state record's two attributes (world.rs:1857-1873; seam.rs:165-172);
  `Room::script` (room.rs:794-806); `cosmetic_sent` per client, every cosmetic event (room.rs:119, :1591); snapshot()
  from :1437, entity changes at :1519 after the events; -41's two bites (world.rs:609-616; sim lib.rs:47); `World::BITES`
  indexed by npt-hash-wasm (world.rs:944-954; hash-wasm/src/lib.rs:84); client BITES six (lib.rs:34-41); clock.rs:7;
  `last_run_command` (lib.rs:455); press counters u8 (seam.rs:33); CO 1 slot, 100 ms, 30 rounds, 2 s reload
  (scene.rs:218-250); lab options (scenario.rs:2063-2065, :1918-1921); `counter_wrap` writer (logs.rs:339). All hold.
- No crate writes cmd_refused/lost/doubled, untraced or corrections, snap_corr_over_5cm, block_extrap*, *_misplaced,
  own_cue, own_change, own_cosmetic_sent or collision_rule (grep): holds. press_lost/doubled are written (room, lab).
- -58's six bite sites are all built already: proto quant.rs:39 (tick16), room lib.rs:41-43 (lt-ack, lt-cmd-seq,
  lt-press), client lib.rs:36-37 (lt-tick-offset, lt-event-seq). Only lt-tick-offset's replay half is U4g's.
- marks.json's nine marks: parts, mins, checks, bites and lines as the brief's section 2 table gives them; counts 75
  measured + 14 bites = §9's list (u4-build-plan.md:1025-1029). -41's bite runs have no profile named in the brief.
- Judge-peak arithmetic: 302.2 + 2.3403 x 3,780 = 9,148.5: holds (the fit as runs.txt prints it).
- -58's wrap names today: room cmd_seq, press, ack; client tick_offset, event_seq, cmd_seq: five distinct, one record
  per name (carry 77 (a)). -58 needs >= 6. NC-U4R-4 (advisor-u4r-r1-full.md:73-76; u4r-report.md:281: "U4g's brief")
  is not in the draft.
- **Missed carry:** the advisor's U4r build ruling 3, point 1 (advisor-u4r-build3-full.md:13-16; history.md:4950-4979;
  u4r-report.md:46, :304, :312, :545): the room's recovery after a B1 eviction (carry 78 widened), its target, -17's and
  -18's L1 B1 parts and -18's no-fallback bitten run moved to U4g; "U4g's brief names all of this". Point 4 (:34-36):
  the ack.rs Low rides U4g's npt-room work. Neither is in u4s-carries.md (task (a) of history.md:4964-4968 not done) nor
  in the plan's U4g entry (task (c) not done), nor in the draft.
- u4g-brief-carry.md and EVQ-LOAD are about -17's and -18's B1 parts (pre:PM-0001-2), so the draft's Q7 ("U4r's -18
  B1 verdict") and Q8 ("-45's ... voids nothing there") rest on a wrong premise.
- -58's lab: warm-up in whole seconds (scenario.rs:2075), every slot admitted and resolved in the warm-up or the run
  fails (:1525-1533; README §1 :79-80); ticks_below_wrap pinned at 30 by -58's setup check: the tick wraps at tick 30.
- U4f's full scope: 6,213 at the stop with about 80 % built (advisor-u4f-stop.md:56-57) plus builder 3's rest 1,400-
  1,550 (:101-102) = about 7,600-7,800 against 5,150: about 1.5, not 1.2; client and room about 2.0 (u4f-report.md:314-315).
- `tick.v.block_extrapolated` is a per-tick flag (SERIES.md:24) in one L1 client log for all slots (logs.rs:1-7).
- `second.s.snap_corr_over_5cm` pools every applied snapshot of every client (SERIES.md:122): -55 on SS with 48 clients
  would dilute the walker's share.
- U4r's B1 runs: 821 MiB of logs for 3 (u4r-report.md:475), about 274 MiB a run; -17's 35 s tick16 bite pre-run
  2,937,440 B, its clean partner 24,813,880 B (c-u4r-r2.mjs:88-89).

## Verdict: REVISE (two Highs; rev 2 comes back to me for the changed parts only)
What holds: the lane, the code claims, the nine marks' parts, checks and bites, the 75 + 14 count against §9, the
carries of u4s-carries.md:554-585 by number, C-U6-19's line, LEAD-BOUND, the judge-peak arithmetic, the rule that the
build changes no marks.json text, judge, README or existing SERIES.md row. What does not: the brief misses the B1 work
routed to U4g by my U4r build ruling 3, and its U4G-Q2 re-offers "not now", which he declined on 2026-11-17.

## Findings (CLAUDE.md's scale)
- **ADV-U4GB-1 (High): the B1 scope routed to U4g is missing.** My U4r build ruling 3, point 1
  (advisor-u4r-build3-full.md:13-15): the room's recovery after an eviction is a defect; "the fix goes to U4g, which
  already owns carry 78 in the same function (`snapshot`). Carry 78 widens to the room's recovery after an eviction.
  Target: on B1 with 10 bursts in 600 s, PM-0001-2's everyone-in-view share is at least 0.99, which allows at most 18
  flag-0 ticks per client per burst on average. It is shown first by a short B1 suite of U4g's build, then by -17's and
  -18's L1 B1 judged runs"; "-18's bitten judged run moves to U4g"; "U4g's brief names all of this". Point 4 (:34-36):
  the no-fallback bite's hole in ack.rs (take the stale copy when a tick becomes last_acked, ack.rs:109-114, :156-158),
  one ring unit test and a short bitten B1 run with no heal within 500 ms, riding U4g's npt-room work. U4r's report
  holds the same lines (:46, :304, :312, :545, :1035, :1167, :1555, :1688). Neither the carries file nor the plan's U4g
  entry was updated (history.md:4964-4973, tasks (a) and (c)), so the drafter could not see it. Effects in the draft:
  a builder would leave the recovery and ack.rs unbuilt; no set would carry -17's and -18's L1 B1 parts or -18's bite;
  section 2's "PM-0001-2 is in no judge call" is wrong (those parts carry pre:PM-0001-2); U4G-Q7's "U4r's -18 B1 verdict"
  and U4G-Q8's "voids nothing there" rest on a wrong premise (u4g-brief-carry.md and EVQ-LOAD are about exactly these
  parts). **Fix:** a carry block in section 4 naming ruling 3 points 1 and 4 with the target and the moved runs; block 4
  gains the recovery (its form the plan's: my example, the relevant soldiers the baseline lacks first with carry 78's
  reorder) and ack.rs; the B1 set listing -2, -17, -18 and -45 (U4G-Q3 below), -18/no-fallback's bite run, the short B1
  suite and the short bitten B1 run in c-u4g-r2.mjs; any ADR 0001 or catalogue words the form changes go on the
  readings-PR list (his go before R5), never edited in the build. -17's B1 part already holds both lost-tick checks
  (marks.json:3983-4004 name "L1 B1"), so no marks.json text is needed for it. The plan's U4g entry and u4s-carries.md
  gain these lines at the main session's next edit (ruling 3's tasks (a) and (c)).
- **ADV-U4GB-2 (High): U4G-Q2's option (c) re-asks a decision of his.** C62HB-Q4 offered him (a) now, (b) with U4g,
  (c) not now (u4r-report.md:1735-1757; history.md:2216); he chose (b) (RECS-1043, history.md:2204-2227), knowing "the
  same wording yes from you first" (u4r-report.md:1750). What is open is only the exact wording, which my C62H-BOUND
  routing put "with U4g (the wording yes first; ADV-CBR-1's caveat in U4g's carry)" (history.md:2495-2496). **Fix:** drop
  (c); the question is a yes to an exact text (U4G-Q2's wording below).
- **ADV-U4GB-3 (Medium): U4G-Q2's route (a) is not shown to be possible, and its timing is wrong.** -58's setup check
  pins the server tick 30 below the wrap (marks.json -58), so the tick wraps at the run's tick 30; the warm-up is whole
  seconds (scenario.rs:2075), and every slot must be admitted and resolve a tick in the warm-up or the run fails
  (scenario.rs:1525-1533; marks/README.md:79-80). So "the warm-up ends before the first wrap" means a warm-up of at most
  1 s, in which all 48 clients join at P3 (5 % loss) — unproven, and at 1 s the wrap is the first measured tick, the
  ticks before it unread. Route (b)'s whole-run reading must leave out the after phase, whose stop-decided ticks the
  figures cannot explain (u4r-report.md:1689-1690). At P0, `snap_whole_despite_link_loss_count` is vacuous (no link
  loss); -17's own P0 form is "loses no group" (`tick.s.snap_incomplete`, a series, which takes `window` with no judge
  change). And nothing in U4g's build needs his answer: only -58's judged set does. **Fix:** as U4G-Q2's drafting
  below; "before the build" becomes "before -58's judged set".
- **ADV-U4GB-4 (Medium): the size projection understates the risk.** "U4f's 1.2 overrun" (advisor-u4f-stop.md:104-105,
  my own line) is 6,213 over 5,150 at the stop, with about 80 % of U4f's items built (:56-57); its whole scope ran to
  about 7,600-7,800 (:101-102), about 1.5 times its estimate, and its client and room about 2.0 times (u4f-report.md:
  314-315). ADV-U4GB-1, -5, -6 and my rulings add about 150-350 lines (the recovery about 50-120 with its tests,
  ack.rs about 20-50, the tick wrap record about 15-40, the B1 rows about 20-50, the prints and cross-checks of Q6, Q8
  and Q9 about 40-100). So whole: about 3,350-5,850 (middle about 4,600), about 5,000-8,800 at 1.5. Also: option (c)
  (the stop raised) is a loosening nobody asked for, drop it; each half's figure and stop must be in the option he
  says yes to (as his SIZE-U4F (a) set U4f2's 1,500); it is not a stop on him now (U4g's launch waits for U4f2's merge
  and -21's runs anyway). **Fix:** section 6 and U4G-Q1 as below; blocks reordered so the prediction-and-cues half is
  built first (blocks 1, 2, the room's carry 79, server-echo, -58's counters and -60's site, CueLog, their lab suites,
  NC-U4R-4 and their script rows), then interpolation, release, carry 78 and the recovery: the first half is the same
  work under either answer, so the build need not wait for him, and his answer only sets where the unit closes.
- **ADV-U4GB-5 (Medium): NC-U4R-4 is missing.** -58 needs at least 6 wrap records, one per counter name (carry 77
  (a)); today the room and client report five names (input.rs:23, :25; room.rs:27; client lib.rs:57-61), the server
  tick's low 16 bits not among them. Route (advisor-u4r-r1-full.md:75): the lab writes one `tick` `counter_wrap` record
  when the room's tick crosses a multiple of 65,536. **Fix:** in block 6 and section 3's -58 row (6 distinct names on the
  wrap suite's short run, a plant dropping the `tick` record failing by name); in section 4 by number.
- **ADV-U4GB-6 (Medium): -45's two flags have no defined writer in a 48-client L1 log.** `tick.v.block_extrapolated` and
  `.block_extrap_over_interval` are per-tick flags "client: 1 when the block built this tick ..." (SERIES.md:23-24), and
  L1 writes one client log for all slots (logs.rs:1-7). Read as "any client this tick", the share is 1 - (1 - p)^48 for
  a per-client share p (for example p = 0.04 % gives about 1.9 %: my arithmetic, not a run), so a correct build can
  fail the 1 % bar; a per-slot list would be a new series name (marks.json: his). **Fix:** U4G-Q9, ruled below.
- **ADV-U4GB-7 (Medium): two parts could pass with nothing measured.** -55 on SS with 48 clients pools every client's
  snapshots into the share (SERIES.md:122), so the one walker beside the vehicle is diluted 48 times and the bite may
  not lift it past 2 %. -60's fire-interval changes make no correction unless the soldier fires, so half the 200
  changes can pass vacuously. **Fix:** U4G-Q3's ruling below.
- **ADV-U4GB-8 (Low):** -41's two bite runs name no profile (P0 is the one where every correction is untraced; any part
  may serve: README §5); section 1.2 should say -58's six sites are built (proto quant.rs:39, room lib.rs:41-43, client
  lib.rs:36-37), so U4g adds only lt-tick-offset's replay half and the run.
- **ADV-U4GB-9 (Low):** section 6's end check item 4 (npt-server's tests, all features) should run: U4g changes
  npt-room's and npt-client's public items and bite lists, which npt-server's tests and the lab's bite router
  (scenario.rs:240-261) read; the test step's line is TE31-10's rule, 1.5 times `<base>`'s measured test step, written in
  the fill form at launch (1,207 s was 5b29296b's); a block whose estimate passes about 400 lines is split (the lab by
  suite), as build ruling line 4 asks.
- **ADV-U4GB-10 (Low):** the disk lines give no total against the free space at launch: the sets come to about 2.6-7.8
  GiB, and U4r's B1 runs logged about 274 MiB a run (821 MiB for 3: u4r-report.md:475), above the 62-210 range; -58's
  "about 3 MiB" a run is a bite run's size (its 35 s clean partner logged 24.8 MB: c-u4r-r2.mjs:89). State the total,
  D2 at launch against the 4,096 MiB floor, and that a set past it waits for a pack of a closed set by 8.1's procedure.
- **ADV-U4GB-11 (Low):** section 5's "no ... run count changes" must read "no run dropped; -60's +3 as ruled (U4G-Q4)",
  or it contradicts the ruling.

## Rulings on U4G-Q3 to Q9 (mine under TEST-FIRST rule 2: inside plan rev 3.3, undoable, no pass mark, no money, nothing
## on main; the main session records each with its reason and names it in the day's message)
- **U4G-Q3, the scenes: the lean, with three conditions.**
  1. **-45 on SS, 48 clients, the brawl's load** (suite `extrapolation`, one option named in the header), and its B1
     runs with 10 bursts in 600 s as U4r's B1 set had them (c-u4r-runs.mjs:26). Those three B1 runs list -2, -17, -18 and -45,
     so one set serves -45's, -17's and -18's L1 B1 parts (pre:PM-0001-2 holds only on the brawl's load); -45's own B1
     part has no precondition and counts even when an entry voids -17's and -18's.
  2. **-55 on SS with one client** (slot 0, walking beside a looping very large unit; the stand-ins drive the vehicles),
     so the flag's share is that soldier's; each run prints its contact ticks (the soldier's box within the hull's
     push-out range), so a pass is not a pass with no contact. No new scene.
  3. **-60 and -62 on CO**, as listed; -60's soldier walks the course and fires throughout, so every change of either kind
     changes the predicted state, and a unit test shows one fire-interval change making its one correction. -61 on SS's
     fight at P2, 48 clients, as the notes say.
- **U4G-Q4, -60 under both options: (b), 3 runs per option, mine.** plan.md:844-846 picks OQ-0001-6's option at R3
  "using -20, -21, -41, -42 and -60 on L1", which needs -60 under each option. Three more short CO runs add evidence and
  loosen nothing: the part pools both options, so every run must pass; marks.json's `min` 3 stands. It is not a run-count
  change of the kind his words guard (no run dropped, no bar moved), and rule 2's list of what waits for him names no
  run count. §9's -60 count reads 6 at the next plan edit; runs.txt reports per option, as -41's.
- **U4G-Q5, the three bite sites: as listed, with these terms.** -60/own-state-missing in npt-room: the own group
  carries the two attribute fields' pre-change values while a change lasts (the room knows them from its own script);
  its unit proof, a correction on every snapshot of one change. -61/on-arrival in npt-client's release: a remote event
  lands in the newest block. -55/vehicle-snap in npt-sim, in a list apart from `World::BITES` (`CHECK_BITES`' form, so
  npt-hash-wasm's index does not move), the lab's router (scenario.rs:240-261) taking it as it takes `CHECK_BITES`, armed
  in the authority's World only (as -41's speed bite); its unit proof, a soldier against a moving hull ending at least
  5 cm apart between the bitten authority step and `predict`. No id in two crates' lists (carry 24).
- **U4G-Q6, what traces a correction: (a), the client alone, with the room as the test's oracle on L1.** The marks.json
  notes say the client counts (-41's notes), and ML can read nothing else. On L1 the lab also joins the room's late and
  lost records: a unit test on a CO replay at P2 shows the two agree; every judged run prints the corrections the
  client traced that the room's records do not support, and the reverse (reported, never judged); a count above 0 in a
  judged run comes to me before the set counts.
- **U4G-Q7, the cap at 1,600: (a), corrected for ADV-U4GB-1.** EVQ-LOAD's line ("before -18's judged B1 runs count") is
  about U4g's own B1 set, not a re-read of U4r's: U4r's -17 and -18 B1 verdicts were INVALID, never counted
  (u4r-report.md:473), and nothing of them is re-read. So: c-u4g-r2.mjs reads the per-client and per-burst high-water
  marks with the cap opened (c-u4f-cap.mjs's method) on short P4 brawl runs before -42's set, and on short B1 runs on the
  tree that holds carry 78's reorder and the recovery, before the B1 set; every judged SS run prints its overflow
  resyncs; one in a judged -17 or -18 run comes to me before it counts (carry 58); 1,600 stands unless R2's own route
  moves it (advisor-u4f-stop.md:129-131).
- **U4G-Q8, the door gap: (b), the per-burst door-gap margin printed always**, with the "in a building" line, on every
  B1 run, pre-runs and judged. The brief states, as u4g-brief-carry.md asks, how an entry reads: it lowers -2's
  everyone-in-view share under 0.99 for that run, so the run is INVALID for -17 and -18 by pre:PM-0001-2, recorded, never
  counted; -45's B1 part still counts it. The short B1 suite (ruling 3's first showing) must read "in a building: 0" and
  the share at least 0.99 before the B1 set runs; any entry there, or in a judged B1 run, comes to me before the set
  counts or anything is re-run.
- **U4G-Q9 (new), -45's flags in a 48-client log: slot 0 is the observer for the share, every slot for the bound.**
  `tick.v.block_extrapolated` is slot 0's flag (all 48 clients run Interp; the part's bar then reads one client's blocks,
  as its ML part reads one browser's frames); `tick.v.block_extrap_over_interval` is 1 when any slot's block extrapolated
  beyond one interval (faithful to "0 blocks"); a header param names the observer slot (a new SERIES.md row). Every
  slot's own share is printed in runs.txt (reported); any slot above its part's bar comes to me before the set counts.
  A per-slot series would be the faithful form but is marks.json text: it goes on rule 1's after-close list, not to him
  now.
- **NC-U4R-4's route (ADV-U4GB-5): the lab's `tick` record**, as advisor-u4r-r1-full.md:75 leans; no re-read of -58's
  count.

## The size (section 6) against SIZE-CHECK
- **The rule:** the plan gives U4g "size M" (u4-build-plan.md:881), so its plan is about 3,000 lines and the stop is past
  6,000 (history.md:2048-2051, :2067-2075). An estimate above 3,000 but under 6,000 is not a stop by his words; U4f's
  5,150 went to him as information beside M's 3,000 (advisor-u4f-brief.md:354-355). What makes U4G-Q1 his is the lean:
  a split is a plan change (advisor-u4f-stop.md:6-7). The draft's "over about 3,000, so it goes to him" overstates the
  rule; "stopped on him" is wrong until U4g's launch is due.
- **The estimate:** the drafter's table is sound item by item as far as I can judge it from the code's shape (client
  and room are the parts that ran 2.0 times at U4f; the lab's estimate held at about 1.0). With ADV-U4GB-1, -5, -6 and
  the rulings: whole about 3,350-5,850 (middle about 4,600); at U4f's whole-scope 1.5 about 5,000-8,800. The split:
  U4g, prediction and cues, about 2,150-3,700 (up to about 5,550 at 1.5, under 6,000); U4g2, interpolation, events and
  the B1 recovery, about 1,250-2,200 (planned about 1,700, so its stop past 3,400; up to about 3,300 at 1.5). The drafter
  recomputes the table with these items before rev 2; my figures are rounded estimates, not counts.
- **The lean (b) stands, and is stronger.** The cheaper option, in his sense (what keeps the unit near its plan), is (b):
  one more review round and one more merge into the lane (about what U4f2 costs), against a likely mid-build stop under
  (a) that waits on him (U4f: stopped 2026-11-18, his answer 2026-11-19 12:00) and a cut through files both halves share.
  With the blocks reordered (ADV-U4GB-4), nothing waits for his answer until the first half is built.

## U4G-Q1 as it should reach Mustafa (paste-ready; not a stop)
```text
U4G-Q1: the size of the next unit, U4g (prediction, drawing other players, event timing). It is planned as a medium
unit (about 3,000 lines; your stop at 6,000). Our estimate is about 3,350 to 5,850 lines. U4f, the last medium unit,
came to about 1.5 times its estimate in all, so U4g built whole would more likely than not pass 6,000 and stop
mid-build, as U4f did.
(a) Build it whole as planned: one review, one merge. The stop past 6,000 stays; if it is hit, you get three lines then.
(b) Split it now in two, each with its own review and merge: U4g, prediction and own-shot cues (about 2,150 to 3,700
    lines; stop past 6,000), then U4g2, drawing other players, event timing and the server's recovery after a loss
    burst (about 1,250 to 2,200 lines; planned about 1,700, stop past 3,400). No pass mark, judge rule or marks.json
    text changes, and no run is dropped.
The cheaper option is (b): one more review and merge, and no mid-build stop waiting on you. The build starts with the
prediction half either way, so nothing waits for this answer until that half is done.
My lean, and the advisor's: (b).
```

## U4G-Q2 as it should reach Mustafa, and how it is made
- **Not before the build:** only -58's judged set (the last) waits for it. No (c).
- **Drafting, in order:** (1) the main session copies -17's two check entries (marks.json:3983-4004) by script into
  -58's text with `parts` ["L1 P0", "L1 P3"] and names to match, and drafts route B's text beside it (the two figures
  reading the warm-up and measured ticks, not the after phase: the judge change's exact rule and marks.json's naming
  of it); for P0 it weighs -17's own P0 form (ADV-U4GB-3). (2) c-u4g-r2.mjs's wrap rows show route A or not: P0 and P3,
  48 clients, a 1 s warm-up, every slot admitted and resolved in it, every `counter_wrap` at or after the measured start.
  (3) A reviewer reads the text of the route that holds (test-engineer at xhigh: TEST-FIRST, "a reviewer's read on every
  change to marks.json or to the judge"), then I check text, reviewer's read and the short rows. (4) It goes in the day's
  message; his yes is recorded word for word with the text's sha256 (LEAD-BOUND's freeze form); the main session commits
  the marks.json edit (and, under route B, the judge change through its own small change-set with its review) before
  -58's set.
- **The wording to him (route A; route B's text replaces the second paragraph if the short run fails it):**
```text
U4G-Q2: the exact wording of the two lost-tick checks for the counter-wrap test (PM-0001-58). On 2026-11-17 you chose
that this test gets the same two lost-tick checks as PM-0001-17, with U4g, after your yes to the exact wording
(C62HB-Q4 (b)). Here it is: [the two entries, in full, with their sha256].
One detail: these checks read only the measured part of a run, and this test's counters wrap in its first one or two
seconds. So its runs use a 1-second warm-up, which puts every wrap inside the measured part; a short test run showed
all 48 clients joined in time and every wrap measured: [the figures].
(a) Yes to this wording. (b) Change it (say what).
My lean, and the advisor's: (a). Only this test's judged runs wait for your answer.
```
  If route A fails, the second paragraph says the warm-up cannot be that short with 48 clients at P3, and the text
  becomes the judge change that makes the two checks read the warm-up and the measured part (never the ticks the stop
  decides), with its exact words: also his, the same (a)/(b).

## What rev 2 must change (REVISE: the exact changes)
1. **ADV-U4GB-1:** section 4 gains ruling 3's points 1 and 4 by file and line, with the target and the moved runs;
   block 4 gains the recovery after an eviction and ack.rs's fix, each with its test; section 2's table gains -17 L1 B1
   and -18 L1 B1 (served by -45's B1 set: U4G-Q3) and -18/no-fallback's bite run; section 3 gains the recovery's test
   (an evicted baseline under SS load: the first frames hold every relevant soldier; its plant, today's order), the
   ring test, the short B1 suite (share at least 0.99, in a building 0) and the short bitten B1 run (no heal within 500
   ms); section 2's PM-0001-2 sentence corrected; counts **78 measured (with Q4) and 15 bites**, the pre-runs listed.
2. **ADV-U4GB-2 and -3:** U4G-Q2 as above: no (c), not before the build, the drafting order, the wording.
3. **ADV-U4GB-4:** section 6's table with the new items and the 1.5 reading; U4G-Q1 as above (no (c), each half's
   figure and stop, the cheaper option named); the blocks reordered, prediction-and-cues half first; the header's and
   the last section's "two questions before the build" and "stopped on him" corrected.
4. **ADV-U4GB-5:** NC-U4R-4 in block 6, section 3's -58 row and section 4.
5. **Rulings:** U4G-Q3 to Q9 and NC-U4R-4's route folded where each "by U4G-Qn" points; section 8 keeps one "Ruled" line
   per question pointing here.
6. **Lows ADV-U4GB-8 to -11** in the same pass (no push of their own).
7. **The main session's own, before launch** (my ruling 3's tasks (a) and (c), history.md:4964-4973): u4s-carries.md's
   U4g section gains carry 78's widening, the moved -17/-18 B1 parts and -18's bite run, the ack.rs Low and NC-U4R-4;
   the plan's U4g entry gains them under "Marks moved" (and the split, after his answer).
8. Rev 2 comes back to me for sections 1-4, 6, 8 and the last section only; the rest needs no new check unless a change
   reaches it.

## What I did not check
- I ran nothing: no cargo, check.mjs, npt-lab or judge. Route A's feasibility (48 joins at P3 in 1 s) is my reading of
  the lab's rules, not a run; the 1.9 % in ADV-U4GB-6 is arithmetic on an example rate.
- The size table line by line against the code it will take; my added-line figures are rounded judgements.
- Every stamp's sha256 but marks.json's, SERIES.md's and the spike tree; u4f2-brief.md beyond the lane and the shape;
  plan.md:468/:515's clamp bounds; carry 70's ES recording; ADR 0001's profile table; the ten C62H-BOUND Lows' texts.
- Whether any other item is routed to U4g outside the files I searched (history.md, the card, u4r-report.md,
  u4s-carries.md, the advisor's U4r, U4f, BRAWL-1 and C62H-BOUND rulings, by "U4g").

## Decisions needed (for the main session's day message, after rev 2)
- **U4G-Q1 (his):** split U4g now into U4g and U4g2, or build it whole. Lean (b), split; wording above.
- **U4G-Q2 (his, later):** yes to the exact wording of -58's two lost-tick checks, once drafted, read and shown by the
  short run; before -58's judged set only. Lean (a), yes.
- Ruled by me, named in the day's message: U4G-Q3 to Q9 and NC-U4R-4's route.

## For Mustafa
Nothing now from this check: U4G-Q1 rides the next day's message after rev 2; U4G-Q2 follows when its text is ready.

## Rev 2 check
Seat and rules as above; head 5359ceb (rev 2 at 6e93586, 270 lines; item 7 at 5ff1081). Read: u4g-brief.md whole
(sections 1-4, 6, 8 and the last section checked; the header's rev paragraph read), `git diff 9e01aa9 5ff1081` of
u4s-carries.md and u4-build-plan.md, their sources (advisor-u4r-build3-full.md:13-23, :34-36, :66-70;
advisor-u4r-r1-full.md:75), the stamps of section 0, the code lines rev 2 adds (room.rs:1444-1455), and every cite
into this file. Nothing run.

### What holds
- **Items 1-6 of my list are met where the rev paragraph (:11-24) says.** The B1 work is in blocks 8 and 10, section 2's
  -17/-18 row (served by -45's three B1 runs, "0 more"), -18's bite, the pre-runs, section 3's rows (carry 78 and the
  recovery, carry 91, the B1 pre-runs) and section 4's first block; section 2's PM-0001-2 sentence corrected; counts 78
  measured and 15 bites, the B1 runs counted once. U4G-Q2 has no "not now", waits only for -58's set, and points to my
  drafting order and wording. The blocks are reordered, the prediction-and-cues half first. NC-U4R-4 is in block 5,
  section 3 and section 4. Rulings Q3-Q9 and NC-U4R-4's route are folded, one "Ruled" line each, every cite into this
  file landing on its ruling (:137-186, :205-219, :221-247). Lows -8 to -11 met.
- **Section 6's arithmetic:** each column sums to its total (prediction half 2,115-3,690; second half 1,165-2,110; whole
  3,280-5,800), and the 1.5 figures follow (5,535; 3,165; 4,920-8,700).
- **Stamps** (section 0, at 9d491fb): u4s-carries.md ea1913c9… 735, u4-build-plan.md 9186c15b… 1,173,
  advisor-u4r-build3-full.md 78ebb5af… 235, c62h-bound-brief.md 7936074c… 663 (8.1 at :485), history.md 1c763b77…
  7,253, this file ea692318… 285: all equal. New cites hold: room.rs:1446-1454 (the defaults fallback), :1449-1450
  (no-fallback), plan :951-952, :975, u4f-brief.md:326 (TE31-10).
- **Item 7:** carry 78's amendment quotes ruling 3's task line word for word (:15) and paraphrases the rest faithfully;
  carry 91 quotes :36 word for word; carry 92 quotes advisor-u4r-r1-full.md:75 word for word with the ruled route;
  carry 93 quotes :22's task text word for word and sits in U4h's section; carry 77 (b)'s second case is :36's words.
  The plan's U4g bullet matches ruling 3 and my rulings; §9's 196 → 199 and 285 → 288 add exactly -60's three runs
  (-18's bite was already among the 46 bites, so the bite count rightly stays); §11's added line is :69's words
  exactly, and Q22's share line was already there (:1163-1166).

### Findings
- **ADV-U4GB-12 (Medium): the bitten B1 judged run may be INVALID, and nothing in rev 2 would catch it before the set.**
  A bite run shows its bite only on a run that holds its validity rules (marks/README.md §5), and -18's L1 B1 part
  carries pre:PM-0001-2. U4r's short bitten run read INVALID at 0.453333 (u4r-report.md:312), its clients blind until a
  full state (max heal 1.69 s). If that blindness stays after U4g's fixes, a judged bitten run at 10 bursts in 600 s
  exceeds ruling 3's 18 flag-0 ticks per client per burst (about 50 per burst at 1.69 s: my arithmetic), so it is
  INVALID and -18's bite unshown; rev 2's short bitten run expects only "no heal within 500 ms" (section 2 :117-118,
  section 3 :156). **Fix:** the short bitten B1 run also prints each client's flag-0 ticks per burst and the share; if
  the bitten run would not hold pre:PM-0001-2 at the judged setting, it comes to me with those figures before the B1
  set (a setting for the bite run, such as its burst count with -18's max check named as the failing one, is mine to
  rule; anything that needs a validity-rule change is his).
- **ADV-U4GB-13 (Medium): the short B1 suite's expectation is the wrong measure, and the wording is mine.** Ruling 3's
  target is "on B1 with 10 bursts in 600 s ... at least 0.99, which allows at most 18 flag-0 ticks per client per burst
  on average". A short suite (r2's form: 30 s measured, 2 bursts, c-u4r-r2.mjs:64) at 0.99 allows about 4.5 flag-0
  ticks a burst, so it can fail a correct fix. My item 1 wrote "share at least 0.99" for it; rev 2 copied it (section 2
  :117, section 3 :156). **Fix:** the short B1 suite's expect reads "at most 18 flag-0 ticks per client per burst on
  average (ruling 3's target), in a building 0, door-gap margins printed", with the share printed beside it; 0.99 stays
  the judged runs' bar.
- **ADV-U4GB-14 (Low):** section 5 (:197-198) reads "past each half's figure"; the stop is past twice it: "past 6,000
  for U4g and past 3,200 for U4g2". And rev 2 does not say what happens if his answer is not in when block 6 is done:
  the builder runs the end checks on the tree as it stands and returns (the size stop's form); the main session goes
  on by his answer.
- **ADV-U4GB-15 (Low), item 7:** carry 93 says "Carry 59 points here", but carry 59 (u4s-carries.md:274-283) gained no
  pointer (ruling 3, point 2's task 2): U4h's business, the next carries edit. Carry 77 (b)'s "(b)'s counting apart
  ends there (:34)" is the main session's reading, not :34's words; mark it so then.

### The deviation: the second half's figures
**The drafter's figures go to him: about 1,150 to 2,100 lines, planned about 1,600, stop past 3,200.** Mine (1,250 to
2,200, planned 1,700) were rounded additions to rev 1's split, and I said the drafter would recompute the table
(:197-198). Rev 2's figures are the sums of an itemized table that holds every item I named, so they are the better
estimate, and the stop is his rule applied to the plan's figure (twice it), as for U4f2. Said plainly here, not to him:
at U4f's whole-scope 1.5 the second half's top is about 3,150, just under 3,200; at U4f's 2.0 on client and room it
could reach about 3,550 (my arithmetic). That is what the stop is for; raising the figure to make room would be
setting the plan to the overrun. The whole and the first half also take rev 2's figures (3,300-5,800; 2,100-3,700).

### Verdict: APPROVE WITH CONDITIONS
No Critical or High. The conditions are small edits by the main session; none needs a new check by me if met as written:
1. ADV-U4GB-12: section 2's pre-runs and section 3's B1 row gain the bitten run's flag-0 per burst and share, and the
   route to me before the B1 set.
2. ADV-U4GB-13: the short B1 suite's expect as above, in section 2 and section 3.
3. ADV-U4GB-14: section 5's stop wording; the no-answer line at the end of block 6.
4. Section 8's Q1 pointer names this block's text as the final one.
5. ADV-U4GB-15 rides the main session's next carries edit (no push of its own).
U4G-Q1's text below is final whatever these edits do: none of them touches it.

### U4G-Q1, final (for the 2026-11-21 message)
```text
U4G-Q1: the size of the next unit, U4g (prediction, drawing other players, event timing). It is planned as a medium
unit (about 3,000 lines; your stop at 6,000). Our estimate is about 3,300 to 5,800 lines. U4f, the last medium unit,
came to about 1.5 times its estimate in all, so U4g built whole would more likely than not pass 6,000 and stop
mid-build, as U4f did.
(a) Build it whole as planned: one review, one merge. The stop past 6,000 stays; if it is hit, you get three lines then.
(b) Split it now in two, each with its own review and merge: U4g, prediction and own-shot cues (about 2,100 to 3,700
    lines; stop past 6,000), then U4g2, drawing other players, event timing and the server's recovery after a loss
    burst (about 1,150 to 2,100 lines; planned about 1,600, stop past 3,200). No pass mark, judge rule or marks.json
    text changes, and no run is dropped.
The cheaper option is (b): one more review and merge, and no mid-build stop waiting on you. The build starts with the
prediction half either way, so nothing waits for this answer until that half is done.
My lean, and the advisor's: (b).
```

### What I did not check (rev 2)
Nothing run; ADV-U4GB-12's 50 ticks a burst and ADV-U4GB-13's 4.5 are arithmetic on U4r's figures, not runs; sections
0, 5 and 7 only where they touch the items above; the section 6 table's item estimates themselves (only their sums).
