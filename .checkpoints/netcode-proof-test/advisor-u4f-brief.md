# The advisor's check of the U4f builder brief (rev 3.2), 2026-11-17

Seat: a day session's advisor (workflow agent, opus, effort xhigh), on main's `.claude/agents/advisor.md` (read with
`git show origin/main:.claude/agents/advisor.md`). Read only: no build, no run, no edit of a source or of the brief, no
commit. Head b40ba07 when this was written; `git diff --stat 0ad9661 HEAD -- spikes/netcode-proof` prints nothing, so
every code cite of the brief is at the head. The brief: u4f-brief.md rev 3.2, sha256
e9f33e61cffc5b229774000d9f6ac8ab1acc9567078caca14e3b2f14167a33ba, 1,734 lines (r31_check.py's line). Notes:
advisor-u4f-brief-notes.md (this run was cut once after its first block and resumed; the notes hold every block).

## Verdict
**APPROVE WITH CONDITIONS.** The build may launch once conditions L1 to L5 are met; L6 is the launch. One of them (L2)
is Mustafa's yes to one bar value, LEAD-BOUND, before the build, as the brief itself plans (U4F-Q8 (iii), 5.6 B3).
- Coverage (check 1): every item of plan §5 U4f (:805-816) and every carry of §U4f (46, 80 (a)-(e), 86) is in the
  brief and testable as worded, but for three gaps of mine below (A1 to A3). Every addition beyond the plan is marked
  and traced to a plan item or a review finding; no engine feature "for later".
- Placeholders (check 2): P1 to P36 each find their anchor once (stamps_anchors.py, run by me), P24 by its key; every
  stamp equals 0.2's. No run of his words stands outside section 12 item 3 (r31_check.py, and my own wider
  u4f-r3/adv_words_check.py against all of history.md: two 8-word hits, neither his words; below).
- My rulings since rev 2 (check 3): applied as ruled.
- The review rounds (check 6): round 1's three Highs and round 2's seven Mediums (six distinct) are answered soundly
  in rev 3.2; I accept each answer, with the conditions their questions' rulings set. No Critical or High is open: no
  third round.
- **My findings: three Medium (A1 to A3) and four Low (A4 to A7).** None High; no security defect. Each is fixed in the
  fold (L1) by the words below; none needs a review round.
- All 18 questions ruled; none escalated. One bar value goes to Mustafa for his yes (LEAD-BOUND, by my Q8 (iii)
  ruling), KL-U4F only if the disk is short after 5.5 item 2. No rule or pass bar is loosened, and no word or decision
  of Mustafa's changes.
- Risk tier 1 (spike code and the spike's message catalogue on PR #40's branch). This ruling is a notes-folder text.

## What I read, and the state it rests on
- The brief whole (rev 3.2, three change lists, sections 0-12, the NA table); its history as the writers' accounts:
  rev 3's two checks (TE3-1..15, SA3-1..8), rev 3.1, round 2's two checks (TE31-1..11, SA31-1..4) and the reviser's
  "Revision for round 2".
- The sources: u4-build-plan.md §5 U4f (:805-816), U4-BRIEFS (:1054-1056), §7, §9; plan.md §4.4 (:507-526); carries
  §U4f (:507-539) and 57, 62 (d)/(h), 74, 75; history.md :2040-2232 (his words, read, never copied here); CHECKPOINTS
  §11 (:231-249); ADR 0001:220-259 and rows -21, -47 (:1611, :1644); the catalogue :1-125; .claude/rules/netcode.md;
  marks.json -20 (:4644-4804), -21 (:4805-5145), -47 (:9072-9135); SERIES.md's U4f rows; my rulings named in check 3;
  D4's three files (d4-compare.md read whole).
- The code at the head: client/src/lib.rs:200-530; room/src/input.rs whole; room/src/room.rs:630-810;
  server/src/pump.rs:1-110, :330-518; lab/src/scenario.rs:455-562; bots/src/lib.rs:1-135; sim/src/ticks.rs;
  sim/src/world.rs:1225-1264 and its respawn phase (:1734-1742); sim/src/scene.rs's speeds; l1/src/lib.rs:140-224;
  c-u4r-runs.mjs:1-60; c-u4r-r2.mjs:1-45; main-run.sh whole; `git grep` of every `Pong {` literal.
- Print-only scripts I ran: stamps_anchors.py, r31_check.py, and two of my own under u4f-r3/: adv_words_check.py and
  adv_phrases.py (every fold anchor named below counted in the flattened brief: once each).

## Check 2: the placeholders and his words
- Every start anchor once, the end where section 12 item 3 says: P1 :507 to :540 (the line before "## U4g" at :541),
  P2 :9-10, P3 to P13 at the lines 12 item 3 gives, P14 :292-293, P15 :2125, P16 :2127, P17 :2187, P18 :2048-2051, P19
  :2141, P20 :2143-2146, P21 :2175, P22 :2212, P23 :2216, P24 df-ruling :462-464 (its key once; the bare words twice,
  as the brief says), P25 :465-468, P26 j-r2 :298, P27 m9 :252-256, P28 :259-264, P29 d2-r2 :304-308, P30 te-m3
  :127-128, P31 c62h :20, P32 b1-p3 :158, P33 r-b2 :28, P34 r-r1 :122, P35 u4r-report :1138-1142, P36 :1163.
- adv_words_check.py (every 8-word run of the brief outside 12 item 3 against all 2,232 lines of history.md): two hits.
  :414 is a path (`CARGO_TARGET_DIR=…/.gate/shared-target`), not prose. :2212 is PACK-READ's option (a), the session's
  own text that he agreed to (the brief already says P22 is never quoted as his words); KL-U4F's draft reuses its
  "never the P0 folders, the reference sets or the c-u2 folders", which is right: the same protected list.

## Check 3: my rulings since rev 2, as applied
Applied as ruled: the merge-stop ruling's C3 (U3C-HELD-1 before U4f's merge: closed, 9.1) and its pooled taker for the
deferred Lows (RR-CB-2 taken in npt-lab; the judge Lows left to npt-judge's next change; 9.2); the head check's
ADV-CBH-1, ADV-CBH-2 and H2 (5.4); report-head R1 (ADV-CBR-1 is U4g's) and R4 (packing by 8.1's list only); c62h :20's
interim rule (P31); c62h-review :82-93 (5.6's read and its report line); U3C-HELD-1's plan review (the u4r-P3 pack
before U4f's counted run: 5.5 item 1); r-b2 point 31 (P33: section 6 item 4, now triggered by Q6 (b), A2); my U4r brief
rulings U4R-Q2 (c) (new SERIES.md rows only), U4R-Q4 (carry 86) and U4R-Q14 (carry 74's leads, the bots' rules).

## Findings, each with its exact fix
Old phrases are matched with each line break and the next line's indent read as one space; adv_phrases.py counted each
anchor below once.

**A1, Medium: "movement" on a death or respawn tick is undefined, and the natural reading fails a correct build.** -21's
lag-switch check is the max over ticks of `tick.v.lagswitch_excess_move_um` ≤ 0 (marks.json:5062-5073; SERIES.md:34:
"movement this tick minus CMD_CATCHUP ticks of top-speed movement"), and the fast clock's movement check is per
episode (:5038-5049; SERIES.md:156). In SS the 47 other clients brawl with fire held, so the lag-switch and fast-clock
clients die and respawn; the respawn phase places a soldier at a free team point (sim/src/world.rs:1734-1742; 1,000 ms
after death, scene.rs:186), metres in one tick, against 8 × 20 cm = 160 cm (600 cm/s, scene.rs:184). Read as the
position change, the first respawn fails the lag-switch check, and the episode check, on a correct build; dead ticks
also lower the measured movement while an arithmetic reference (Q11 (iii) (c)) does not. 5.3's p3 and p4 would catch
it before the sets, at the cost of a stall. Fix, in the fold:
- Before "### 2.7 npt-lab: suites, settings and series" add: "**Movement (rev 3.3, the advisor's A1).** Both movement
  series (`tick.v.lagswitch_excess_move_um`, SERIES.md:34; `event.fast_clock.excess_move_um`, :156) count the
  soldier's displacement made by the sim's movement step only: a tick in which the soldier is dead at its start or its
  end, or the respawn step places it (world.rs:1734-1742), adds 0 movement and is counted apart (a report-only series
  per client, the builder's name: 5.6 B2), and the fast clock's honest reference (U4F-Q11 (iii) (c)) sums the same
  ticks the measured movement sums. This reads ADR 0001:1611's "moves" as a soldier's movement by its commands (a
  respawn's placement is the server's); the reading joins the readings PR's list (plan :971-974) and is stated in
  LEAD-BOUND's entry for him (12 item 6)."
- f19's row: after "at a 2 s episode" add "; a case with a death and respawn inside the episode: the respawn tick adds
  no movement and is counted apart (A1)". f20's row: replace "movement within the catch-up cap's |" with "movement within
  the catch-up cap's; a case with a respawn of the lag client: that tick's placement is not movement and is counted
  apart (A1) |".
- U4F-Q11 (iii) (c): add "; the reference and the measured movement sum the same live ticks (A1), and the builder's
  collision-free path is shown against the static boxes, the doors' and the wall's schedule and the vehicles' parts over
  the episodes' ticks (world.rs:1230-1264), soldiers not colliding with each other".
- 5.6: a new row "| B8 | the two movement series read as the sim's movement step, a respawn's placement and dead ticks
  apart (A1) | the lab's writers; no SERIES.md text | not a bar's text: a reading of ADR 0001:1611's "moves" | built |
  stated in LEAD-BOUND's entry; the readings PR |".

**A2, Medium: U4F-Q6 (b), the recommended (and now ruled) option, needs npt-server, which the brief's lane omits.** The
room's `Pong` is built in npt-server (pump.rs:458-467), not in npt-proto, and `Pong` is a struct with public fields, so a
new field breaks every literal of it: client/src/lib.rs:473 (in the lane), proto/src/msg.rs:434 and
proto/tests/common/mod.rs:366 (opened by Q6 (b)), and server/tests/pump.rs:276 and :310 (not in the lane). As written,
the builder meets a lane stop at its first build. Opening server/tests meets P33's trigger ("the next unit whose lane
opens npt-server's manifest or tests": r-b2 :28), so section 6 item 4 and D9 apply. And `Decoder::pump` holds `now_ns`
and `tick` but no tick's due instant (pump.rs:372-377; the lab passes `tick = last_tick + 1`, scenario.rs:476-480), so a
"µs since the tick's due instant" field needs a signature change through npt-server's own callers, while the server's
clock at the send (the `now_ns` it holds) needs none. Fix, in the fold:
- U4F-Q1 (a): replace "plus crates/proto and the catalogue only by U4F-Q6 (b) and crates/server only by U4F-Q14 (a) or
  U4F-Q6 (d);" with "plus, by U4F-Q6 (b): crates/proto (the `Pong` field, its codec and tests, proto/src/msg.rs and
  proto/tests), the catalogue's `Pong` row (first: .claude/rules/netcode.md:17), crates/server/src/pump.rs (the room's
  `Pong`, :458-467, and nothing else), crates/server/tests/pump.rs (its two `Pong` literals, :276 and :310, and the
  room-side test) and, for P33's compile fix, crates/server/tests/loopback.rs; crates/server further only by U4F-Q14
  (a) or U4F-Q6 (d);".
- Section 4's lane: replace "crates/proto and the catalogue if U4F-Q6 (b) rules a wire change; crates/sim's carry 18
  places if U4F-Q8 (a); crates/server if U4F-Q14 (a) or U4F-Q6 (d) needs a pump change." with "by U4F-Q6 (b): the
  files U4F-Q1 (a) names for it (crates/proto, the catalogue's `Pong` row, server/src/pump.rs's `Pong` only,
  server/tests/pump.rs, server/tests/loopback.rs for P33 only); no npt-server manifest and no other npt-server item:
  a fix that needs a Cargo.toml change, or a change to `Decoder::pump`'s signature or any other public npt-server
  item, stops (section 1 item 5)."
- 2.5: replace "U4f changes no pump code by this brief; if U4F-Q14 or U4F-Q6 needs any, the lane opens npt-server
  (U4F-Q1) and section 6's `--all-features` item applies." with "By U4F-Q6 (b) U4f changes one place of pump code, where
  the room builds its `Pong` (:458-467), and the two `Pong` literals of server/tests/pump.rs; that opens npt-server's
  tests, so section 6 item 4 applies (P33)."
- Section 6 item 4: replace "4. **Only if a ruling opens npt-server before U4h** (U4F-Q1, U4F-Q14 (a) or U4F-Q6 (d)):"
  with "4. **Since U4F-Q6 (b) opens npt-server's tests** (P33's trigger):". D9's row: replace "(only if a ruling opens
  npt-server)" with "(U4F-Q6 (b) opens npt-server's tests)", and its "When" to "after the build". The fix moves or
  re-gates loopback.rs's use of `npt_client::BITES` (r-b2 :26-27) by test edits only; a moved test is counted as -1 and
  +1, stated with its `--list` lines.
- 2.13: the npt-server row becomes "| npt-server | the room's `Pong` sets the new field (U4F-Q6 (b)); no new public item
  | 2.1 |"; a new row "| npt-proto | `Pong`'s server-time field and its codec (U4F-Q6 (b)) | 2.1 |".
- 2.10: one more test, by U4F-Q6 (b) (+1, both builds): the room's `Pong` carries the server's time at its send, read
  against the lab's simulated `now` (in server/tests/pump.rs or a lab test).

**A3, Medium: what `tick.s.cmd_on_time` counts for a command that never arrives is undefined, and the choice decides
p12.** SERIES.md:61 writes "one per command due this tick: 1 when it arrived before its tick ran". If the room writes an
entry only for commands that arrive, a command whose every copy was lost is in no denominator: under the
redundancy-1 bite at P3 (5 % loss each way, l1/src/lib.rs:190-192) about 5 % of commands vanish, the rest arrive on
time, the on-time share stays near 1, and p12's named check ("at least 99.9 % of commands reach the server before their
tick runs") passes, against the bite's own text ("fails the 99.9 %": marks.json:4795) and Q10's arithmetic ("the 99.9 %
check fails (about 95 %)"). Under the controller's contiguous stamps (2.2) every tick from a client's first stamp has
one command due, so the room can count by stamp. Fix, in the fold:
- 2.3's "Missing, by stamp (NA-14)" row: append to option A's cell " `tick.s.cmd_on_time` (SERIES.md:61) likewise by
  stamp (rev 3.3, the advisor's A3): at each tick, one entry per joined client whose first accepted stamp is at or
  before it, 1 when the command stamped for this tick arrived before the tick ran, else 0 (late or never arrived), so a
  command lost in every copy counts against the 99.9 %, as -20's bite says (marks.json:4795)", and to option B's cell
  " and `tick.s.cmd_on_time` the same by its stamp".
- f34's row: add "; and `tick.s.cmd_on_time` 0 for a stamped command that never arrived and for one that arrived after
  its tick, 1 for one that arrived before, under both options (A3)".
- 5.3's p12: after "fails, PM-0001-2 passing;" add " about 95 % on time, a lost command counted by its stamp (A3);".
- Carry 74's option-A reading (5.3's closing paragraph and D15): "the commands that arrived after their stamped tick
  ran" reads "the commands not on time by their stamp (late or never arrived)": stricter, the same series.

**A4, Low: the honest ping interval is a rate-derived count that nothing pins.** 2.1 has the client ping only at its
steps, every 3 steps in D4 (10 a second at 30 Hz, exactly the room's limit: pump.rs:22, :85); at 60 Hz every 3 steps is
20 a second, and `ping_limited` rises on an honest client. Fix: 2.1's "Ping rate and phase" cell gains "the interval in
ns of host time (the first step at or after it), or a step count from the rate passed in, pinned at 30 and 60 Hz in
f31 (A4)"; f31's row gains "; the honest ping interval (A4)".

**A5, Low: 5.6 B2 misses three report-only series these rulings add.** Fix: B2's list gains "U4F-Q3 (c)'s ticks each
command ran before its stamp under option A; the lead bound's header param (U4F-Q8 (b)); A1's dead and respawn ticks
apart".

**A6, Low: 5.5 item 2's part-clean sits before 5.3's pre-runs, which build in the shared target.** c-u4f-r2.mjs runs on
c-u4r-r2.mjs's model, under the runner or main-run.sh (c-u4r-r2.mjs:25-26), and main-run.sh sets the shared target
(:15-17); a part-clean before them is undone by their release builds. Fix: 5.5 item 2 (from "2. **After U4f's merge and
before its first set,**") reads "after U4f's merge, its counted runs and 5.3's pre-runs, and before its first judged
set"; D2's "and after item 2" reading is judged against the judged sets still to run (-47, -22, -20 and -21: about 3,317
MiB, 3,727 less the pre-runs' 315 and the cap runs' 95 already on disk) plus the main folder cache's growth from their
release builds (`du -sm` of .gate/netcode-proof-test/target before and after the first set's builds).

**A7, Low: three cites into .claude/rules/netcode.md are off by its 6-line front matter.** The catalogue rule is :17, not
:15 (section 1 item 5), and the unbounded-buffer rule is :14-15, not :13-14 (2.1's Bounds row and 2.4 item 2). Fix:
"netcode.md:15" → "netcode.md:17"; both "netcode.md:13-14" → "netcode.md:14-15" (each found once as a token: Grep).

## The questions: each ruling in a line
- **U4F-Q1:** (a) with (d), and by U4F-Q6 (b) also crates/proto, the catalogue's `Pong` row, server/src/pump.rs's
  `Pong`, server/tests/pump.rs and (for P33) server/tests/loopback.rs (A2).
- **U4F-Q2:** (a); KL-U4F only if D2 after 5.5 item 2 (A6's place) shows the room short; then (d) with (c), nothing of a
  U4f set packed before his yes in his own words.
- **U4F-Q3:** (c).
- **U4F-Q4:** (b).
- **U4F-Q5:** (a); P25's first `<crate>` names every crate the lane opens.
- **U4F-Q6:** (b), with an averaging filter; the field is the server's time at the `Pong`'s send, exact to 1 µs or
  better, inside the 32 B limit, with no change to `Decoder::pump`'s signature (the server's ns at the send, 8 B, meets
  this); f1 at 4.0 ms on a trace that carries P2's 2 % loss.
- **U4F-Q7:** (b) with (i) and (iii) (c).
- **U4F-Q8:** (i) (b) at 400 ms (12 ticks at 30 Hz, 24 at 60 Hz); (ii) (c); (iii) (e): LEAD-BOUND to him before the
  build.
- **U4F-Q9:** (a), K = 16 (8 + 8).
- **U4F-Q10:** (a).
- **U4F-Q11:** (i) (a); (ii) (a); (iii) (c), with A1's movement rule; (iv) (a); (v) (b).
- **U4F-Q12:** (b); the figures and the value come to me before any -22 run.
- **U4F-Q13:** (b).
- **U4F-Q14:** (b); the hold-time cap's -21 counter (plan.md:511, :515) is a named follow-up for the first unit that
  runs `ws` sessions.
- **U4F-Q15:** (c).
- **U4F-Q16:** (b) (`ACK_RING_TICKS`: 32 at 30 Hz).
- **U4F-Q17:** (a).
- **U4F-Q18:** (i) (a) (W = `CMD_CATCHUP` = 8) with (ii) (d).

## The reasoning, question by question
**Q1.** The narrowest lane in which every bite has its site and every ruled item compiles. (d): the ws rows' wording is
one text with delay.rs's docs, and U4f never opens npt-net. A2 adds what Q6 (b) needs; nothing else in npt-server.

**Q2.** Arithmetic from D2 (18:54 UTC): 5,468 − 4,096 = 1,372 MiB above the floor; the u4r-P3 pack gives back about
519 × 0.73 ≈ 380, so about 1,752; the shared target holds 6,346 MiB, and the last part-clean gave back 6,125
(4,467 → 10,592: the card), so after 5.5 item 1 the room is likely near 7,800 MiB above the floor, of which the build
phase takes much back (a counted run grew the shared target about 4.76 GB from near-cold:
advisor-disk-deadlock.md:18). Item 2's part-clean (after 5.3, A6) returns it before the sets. The sets still to run then
need about 3,317 MiB plus the main cache's growth, so the room likely covers them without KL-U4F; the reading decides,
never this estimate. PACK-READ (a)'s "one or two closed, older log folders" is what his yes covered, so a U4f set's
own folder, or C62H-BOUND's two closed sets, waits for KL-U4F. Splitting -21 (c) lowers each call's memory and
projection, not the total: it is the ordering under (d), as the brief says. Evidence is never shrunk to fit.

**Q3.** (c) rests on a reading already in ADR 0001:253-255 ("commands run on arrival counting as none", the advisor's
reading of 2026-11-03), so it needs no new reading: under option A `second.s.buffer_depth_count` is 0, the header's
`buffer_depth_ticks_count` (-30's param, SERIES.md:240) is not written, and the early-run ticks are reported (A5's row).

**Q4.** (b): -47 reads no hash and no event. The sim's only events are Shot, Hit and Kill (world.rs:1550, :1620,
:1638), so a never-firing script stamps none, and carry 75's bound (scenario.rs:70) can never be met; carry 62 (d)'s
head rule bears on hash logging, which (b) does not write. (a) is U6's join design.

**Q5.** (a): it costs nothing and keeps a failure from being retried unnamed; the merge-stop ruling applied the same
pattern after TE2-5 ended. P25's substitution reads "npt-room, npt-client, npt-bots or npt-lab (and npt-proto,
npt-server and npt-sim, which U4F-Q1 opens)".

**Q6.** On D4's figures (p99 of |error| at 2 s at P2, 99,000 connects; the bar 5 ms): as built no estimator passes
(the mean 5.975 ms with random phases, 8.867 with one phase per connect; the lowest-RTT pick about 18.3); (e) as
worded 12.781 (half the lowest own-ping RTT about 7.3 ms short of the one-way delay; half the mean RTT 5.49, still
over). With an exact server time the mean gives 2.346 (the sd of a 3,000-connect p99 0.049, so about 0.085 for a
1,000-connect run) and its tick-quantised bite 11.927; the newest-sample bite about 8.9. The lowest-RTT pick gives
4.856, 0.14 ms under the bar (about 0.17 sd at 1,000 connects, 0.100 at 3,000), so a judged run of it would pass or
fail by chance. So only (b) passes with both bites showing, and only with an averaging filter. f1's 4.0 ms sits about
19 sd of a 1,000-connect p99 above the mean and about 5 sd below the lowest-RTT pick: it fails the wrong filter every
time and the right one never, with 1.65 ms left for what D4 did not model (the ADR's drift, halving and reset rules;
L1's reorder step; the read instant). ADR 0001:239-242 says "from its own ping samples" and names no pick, so the
averaging filter needs no reading, and the ADR fixes no `Pong` field: the field is a spike wire change inside the plan,
the catalogue's (status Proposed, a spike design doc) updated first, with its tests' latency, jitter and loss cases
(.claude/rules/netcode.md:17; hence P2's 2 % loss in f1's trace: D4 shows it changes no verdict). The form: the pump
holds the server's `now` but not a tick's due instant (A2), so the server's ns at the send (8 B; the `Pong` grows from
16 B to 24 B, inside its 32 B limit, catalogue :92) is the form that changes nothing else; the client gets the tick
grid from it with the `Pong`'s tick and its snapshots, and the controller's margin covers what is left. Under (a) or
(e), -47 fails on a correct build, so neither can be ruled without ruling -47's fate. (d) changes when the room answers
and needs a reading of ADR 0001:247-248.

**Q7.** (b) is ADR 0001:250-251's form ("each command is stamped with the server tick it is meant for") and net F104's;
with Q6 (b) its estimate is good to a few ms against a 33.3 ms tick. Its target moves 1 a step but for ClockSync's
corrections, so (iii) (c)'s catch-up rarely acts; under (a) the newest tick jumps 3 or more in about 1.16 % of P3's
steps, and (c)'s carried stamps keep the backlog at most 3 with none unused (the test-engineer's model), so (c) holds
under either form and (e) does not. (i) keeps U4r's suites and the -30 pre-check reproducible (and with them the short
P2 C62H-BOUND test's settings), and `lead_ticks_count` stays true where it is written. f36 runs on the ruled form.

**Q8.** `ticks_for_ms_at(30, 400)` = ⌈12,000 / 1,000⌉ = 12 and at 60 Hz 24 (sim/src/ticks.rs:25-31), a room setting in
ms with the rate passed, so no new constant (carry 39). It exceeds every honest lead: an honest command arrives a
margin and the buffer target ahead of the room's tick under Q7 (b), and even U4R-Q14's newest-relative P4 lead of 9
fits under 12 less a margin of 2. It lets the catch-up kind be built under option B: at P0's lead of 2 the buffer has
8 − (2 − 1) = 7 free places, so 8 distinct stamps are needed inside the bound, about 10 ticks (input.rs:114-116). It
sets the fast-clock checks' allowance: 12 commands and 12 × 20 cm = 240 cm of top-speed movement per 60 s episode;
under option A after an idle the room's own limit is 12 + W = 20, while the checks still allow 12 and the judged fast
clock never idles. (ii) (c): under option B a stamp-ahead buys input-buffer wait, which §4.9's view-time estimate must
cap at the honest lead, not the bound: U4h's carry (SA-11). (iii) (e): the bound's first value sets how loose a judged
check is; by his 13:35 rule (P15) that comes to him before it is built.

**Q9.** (a), K = 16: an overtaken honest datagram (L1's adjacent-pair swaps, l1/src/lib.rs:146) carries sequences at
most one datagram older than the newest, inside 8 + 8; (b) counts honest overtaken copies as forged and lets a forgery
inside the window pass; (c) hides NA-7. The kept window is a fixed ring per client: bounded (.claude/rules/netcode.md).
Q9 (a)'s flood test is a negative case.

**Q10.** (a): under option A with no repeats about 5 % of P3's commands never run, so a -20 bite run there risks
PM-0001-2's shot floor (about 700 × 0.95 = 665) and INVALID; under B the repeat keeps the shots and the 99.9 % check
fails (about 95 %, by A3's counting). -21's bites run at P0, where the option changes nothing but no-catchup-cap, which
plan :463 puts under A.

**Q11.** (i) (a): r2's P3 suite showed that load; a press every second command is one edge per two covered ticks,
inside the room's press rule. (ii) (a): no loss to confound the twins and no clamp reruns (SA-10). (iii) (c): the
row's words hold once its test shows the arithmetic equals an honest client on the same inputs, with A1's live-tick
rule and the collision-free path shown against every box that moves (doors, wall, vehicles; soldiers do not collide:
world.rs:1230-1264). (iv) (a): four short runs, about 60 MiB (5.5: 195 → 255, the fast clock's at 2.7 times a short run). (v) (b), with p9 before the set.

**Q12.** (b): the cap must lie above the honest high-water mark with a stated margin and below what a never-acker's
queue can reach before its events expire (255 ticks: events.rs:19-20), about 17.3 a tick × 255 ≈ 4,400 at EVQ-LOAD's
full-view rate and about 3,900 at 460 a second (NA-12), else -22's headroom never reaches 0 and the check fails a
correct build; the formula as written lands under the need (NA-3, P32's 27 resyncs). The cap is no bar (marks.json:5227
calls it a design constant); carry 57 has its figures come to me first. Plan §4.7's text is the main session's commit.

**Q13.** (b): no wire or rule change, one refusal per command by construction, and the twins keep the sequence numbers
the state hash reads (world.rs:1979, :1995-2009); both round-1 checkers confirmed the construction, and SA3-2's
renumbered variant is rightly rejected.

**Q14.** (b): on L1 the RTT comes from the link (pump.rs:339-344), so the hold clamp (:510) bears on no L1 mark and no
forged kind is its; a counter would read 0 on every U4f run. It stays a plan item not met (plan.md:511 gives every check
a -21 counter; :515 names the hold-time cap): U4f's report names it with why, and the card and STATUS carry it for the
first unit that runs `ws` sessions. (c) makes new bars.

**Q15.** (c): nothing loosened or shrunk, no bar changed; p19's arithmetic holds (the Input bucket passes 8 of 12
datagrams, pump.rs:20, :84; W drops the oldest 3 or 4), so an honest hold never puts more than 8 new commands into a
tick, and the cap is shown acting by the forged suite's kind 10.

**Q16.** (b): an honest refused header lies within the ring of the newest tick (B1's 1 s gap is 30 ticks, under 32),
the bound is an existing rate-derived constant, and on honest L1 runs no datagram is refused, so no filing changes and
C62H-BOUND's frozen checks are untouched.

**Q17.** (a): one comparator line; the bite run's far-ahead stamps can still cycle (t − 5, t + 2^30, t + 2^31 − 2 in
one tick, SA31-3), and a panic in the room's tick makes a run INVALID.

**Q18.** (a) with (d): the join anchor closes the fresh-slot bank (SA3-1), W the idle bank (at most 8 over the bound;
an honest resend after a burst, at most `REDUNDANCY_CAP` = 8, fits, and so do the catch-up kind's 16), and the per-slot
kept stamp the rejoin bank (SA31-1). (d) is ADR 0001:231-233's check as written ("strictly newer than the last command
run", which belongs to the slot's soldier, who lives on across a rejoin: room.rs:648-653, scenario.rs:458-472), so it
needs no reading; (e) would need one. Its store is a fixed array of the room's slots, never keyed by client data. W
reads as `CMD_CATCHUP` itself, so no new `*_TICKS` constant.

## The review rounds (check 6)
- **Round 1's Highs, accepted:** TE3-1 (missing inputs at P3): contiguous stamps with rev 3.2's carried catch-up hold
  under Q7 (b) as ruled, f36 shows both -20 shares on a P3 trace before any run, p5 and p6 read them by name. TE3-2
  (per-kind "rejected and counted"): 2.4 item 6's rules and f33; round 2 found only wording, fixed (TE31-11). SA3-1
  (option A's lower bound): the join anchor and W by Q18 (a), the rejoin by Q18 (ii) (d), the first stamp by 2.2.
- **Round 2's Mediums, accepted:** TE31-1 (R11: the catch-up carried, `stamps_skipped` expected 0, f36 ≤ 0.1 %);
  TE31-2 (R12: no twin in 5.3, the Gap read apart, the rule per run, p13 named); TE31-3 (R13: p19 reworded with P1's
  arithmetic); TE31-4 = SA31-2 (R14: the first stamp, f32 (iii), f13 (iv)); TE31-5 (R16: each f16 pair carries its kind,
  `--forge-every`; f29 under Q4 (b) asserts no event stamped); SA31-1 (R15: the per-slot kept stamp, by Q18 (ii) (d)).
  The not-taken half of TE31-1 (f36 under both Q7 forms) is right: an open question stops the builder, and Q7 is now
  ruled.
- **Rounds:** two done; no Critical or High open, so no third. My A1 to A7 are this check's (not a round) and are fixed
  in the fold; the build's panel reviews them on the code.

## The bars for Mustafa (his 13:35 rule, P15 and P16; 5.6)
- **To him before the build: one entry, LEAD-BOUND** (B3, by Q8 (iii) (e)). Nothing that reads the bound (kind 1, the
  fast clock, the catch-up kind under option B) is built before his yes in his own words, so the build waits for it.
- **To him only if needed: KL-U4F**, if D2 after 5.5 item 2 (A6's place) shows the room short of the judged sets still
  to run.
- **Not to him:** B1 (no marks.json or judge change), B2 (report-only SERIES.md rows; the read recommended and named
  in the report), B4 (the event cap, a design constant; its figures to me first), B5-B7 (not taken by these rulings),
  B8 (A1's movement reading: a reading of ADR 0001:1611, stated inside LEAD-BOUND's entry so he sees it before the
  build, and listed for the readings PR), f1's 4.0 ms (a unit test's pin, stricter than -47's 5 ms, no marks text), W
  (a room refusal rule; no check's allowance changes) and A3's counting (SERIES.md:61's own words, stricter, as -20's
  bite text says).
- **What LEAD-BOUND's entry must say** (the main session drafts it; I and the test-engineer read the draft once, no
  round, before it goes; c62hb-paste's form, under Decisions needed, its code explained in it):
  1. The question in plain words: may U4f's room refuse a command stamped more than 400 ms ahead of the server (12
     ticks at 30 Hz, 24 at 60 Hz); a 1.5x-clock client then runs at most 12 more commands, and moves at most 12 ticks
     of top-speed movement (240 cm) more, per 60 s than the server's ticks allow before -21's fast-clock checks fail.
  2. The value as it is built, in a fence: a room setting in ms turned into ticks with the rate passed
     (`ticks_for_ms_at`), recorded in every run's header under its row's name, pinned at 30 and 60 Hz.
  3. What it does not change: no marks.json, SERIES.md row or judge text; the two fast-clock checks' words stay; under
     option A after an idle the room's own limit is the bound plus the past window (20 commands) while the checks still
     allow 12, and the judged fast clock never idles; the limit holds per slot across rejoins (Q18 (ii) (d)).
  4. How both movement checks read movement (A1): the sim's movement step only; a respawn's placement and dead ticks
     are not movement and are counted apart; else the first respawn of the lag-switch or fast-clock client fails a
     correct build.
  5. Why it matters, one line: it sets how loose the fast-clock checks are, and under option B how much input-buffer
     wait a stamp-ahead buys (capped at the honest lead in U4h's view-time estimate: the carry to U4h).
  6. The options with their arithmetic: (a) 400 ms, (b) 600 ms (18 ticks), (c) 1,000 ms (30 ticks), (d) another value,
     which must exceed every honest lead (U4R-Q14's P4 9 less nothing, plus a margin) and let the catch-up kind be
     built under option B (about 10 ticks at P0); the recommendation (a), and that it is mine by U4F-Q8.
  7. The planned size: the brief's estimate as restated in the fold (about 5,150 added lines, 4,200 to 6,200), beside
     M's 3,000, and that the build stops and tells him in three lines past 6,000, as his SIZE-CHECK says.
  8. The runs it governs: -21's two fast-clock parts (3 + 3 measured runs and the not-older-than bite run, option A),
     and every -21 run's kind-1 refusals.
  9. The freeze: the value recorded at his yes; before the judged runs a script shows the room's built default and
     every judged run's header hold it; any change after his yes comes back to him; the value is never changed to fit a
     run's result (P21's rule for C62H-BOUND, applied the same way).
  10. What stays open: nothing else for him now; KL-U4F only if the disk is short after the build.
- **What KL-U4F's entry must say, if it is needed:** D2's readings after 5.5 item 2 in MiB (free, above the floor), the
  judged sets still to run and their estimate, the folders proposed with their sizes (each closed U4f set's own, and
  under (c) first u4r-P2-8ee453c22882 and u4r-P3-d66e23cd78f2), never the two P0 folders, the reference sets or the
  c-u2 folders; each pack by 8.1's procedure (the list committed and pushed, pack, read-back, the walk again, free
  before and after); the options (a), (b), (c) as section 12 item 6 drafts them; the recommendation; nothing packed
  before his yes in his own words.

## Conditions before the launch (the main session's)
- **L1. Fold this ruling into the brief by script (rev 3.3):** A1 to A7 as written; the 18 rulings into the places
  each "by U4F-Qn" names (section 4's lane and Q1 (a) as A2 words them; the conditionals made firm: f38 and f39, Q6
  (b)'s +4 and A2's +1, Q9 (a)'s +1, Q11 (iii) (c)'s +1; p15 to p20 kept; KL-U4F's trigger at A6's place; the 2.10
  counts **at least +118 measured, +133 bites, 67 negative cases**, plus T-6's +5 only if contract.rs is edited and
  P33's moved test counted −1 +1; pre-runs **21**; disk **about 3,727 MiB**); SIZE-CHECK's table restated with a row
  for this ruling (about +300 lines: Q6 (b)'s codec, catalogue row, pump line, server test literals and P33's fix; A1;
  A3; total about 5,150, 4,200 to 6,200); section 10 gains a "Ruled" line per question pointing here; the fold recorded
  under the title. Each old phrase found once (adv_phrases.py's method). If a phrase fails to match, or the fold must
  change any other word, it comes back to me; otherwise no second check.
- **L2. LEAD-BOUND to him:** the entry drafted as above, read once by me and the test-engineer, posted under Decisions
  needed, his yes in his own words recorded in history.md and on the card, the value's line and its sha256 recorded.
  No launch before his yes; if he picks (b), (c) or (d), the fold's numbers for the bound (B3, 12 item 6, Q8, f31) are
  restated by script and come back to me only if anything but those numbers changes.
- **L3. The main session's own commit before launch:** SA-11's carry to carries' U4h section (Q8 (ii) (c)); on the
  card and for STATUS: the hold-time cap's -21 counter (Q14 (b)) and SA-16's variant (section 12 item 11); the readings
  PR's list gains A1's reading of ADR 0001:1611's "moves", in this ruling's words.
- **L4. Section 12 at launch:** items 1 and 2 (`<base>` and D1; u4f-run.sh by script from u4r-run.sh, only the lane's
  names changed, its .diff beside it, sha256 and `bash -n` exit 0), item 3 (the placeholders, P25 as Q5 rules it), item
  10 (re-stamp: history.md will hold his LEAD-BOUND yes; D6's diff; re-run stamps_anchors.py, r31_check.py and
  adv_words_check.py on rev 3.3), item 13 (D3 by the Grep tool, then the u4r-P3 pack by 8.1's procedure and the shared
  target's part-clean, with D2's readings before and after each on the card).
- **L5. The card:** "Rules in force" unchanged; the U4f brief entry names this ruling, rev 3.3, LEAD-BOUND's state and
  the 21 pre-runs.
- **L6. Launch as section 1 says:** one builder (general-purpose, opus, xhigh) in its own worktree from `<base>`, under
  W1-TARGET's lock and u4f-run.sh; at most three agents at a time, reviewers and the builder counted; the panel of
  section 12 item 8 (netcode-architect, security-auditor, test-engineer, rust-reviewer, engine-architect on the public
  items, npt-proto's and npt-server's included) in two batches after the build.

## The build rules (check 7) and the D-items (check 8)
- **Build rules, confirmed:** the lane runner made from u4r-run.sh by script with only the lane's names changed (my
  U4r form); SIZE-CHECK with the planned size (M's 3,000), the brief's estimate beside it and the stop past 6,000 in
  his words (P18); his command-forms rule (P17) with "a call that may still ask is named first"; KEPT-LOGS §11 and 5.5
  (only u4r-P3 packed at launch, the protected folders never, a U4f set's folder only after KL-U4F); the stop clause
  by Q5 (a); the no-socket rule. A2 adds the server stop ("no manifest, no pump signature").
- **D-items, confirmed runnable as written and their bars right:** D1 (main-run.sh's usage `<log> <command...>`, run
  from spikes/netcode-proof: main-run.sh:11, :26; the exclude form for kept-logs lists), D3 (the Grep tool, before the
  pack), D6 (plain `git diff --stat`), D9 (the runner's form; by A2 it now runs, its bar exit 0), D10 (plain git), D11 to
  D15 (5.3's runs; D12's per-run rule as rev 3.2 states it; D15's 0.1 % read by A3's counting), D14 (the plant in one
  `-x -k` call, restored and its sha256 checked). D7 and D8 are the builder's, their command lines in its notes, as U4r's
  D4 and D13 were.

## What the main session does next, in order
1. Record this ruling on the card (L5).
2. Draft LEAD-BOUND's entry (L2) and send it to me and the test-engineer for one read; then post it to him.
3. L3's commit.
4. L1: fold this ruling into the brief (rev 3.3); then L4's section 12 items on that text (after his yes: the value).
5. After his yes: L6, the launch.
6. After the build: the panel in two batches; my merge conditions in U4r's M-form; the merge; 5.3's pre-runs; 5.5 item 2
   (A6's place) with D2; KL-U4F to him only if short; the event cap's figures to me before -22's runs; then the sets
   one at a time (-47, -22, -20, -21).

## What I checked, and what I did not
- **Checked, by reading:** the brief whole; the plan, carries, history, §11, ADR and catalogue lines named above; the
  code at every cite my findings and rulings rest on; D4's comparison file.
- **Checked, by running (print-only):** stamps_anchors.py, r31_check.py, adv_words_check.py and adv_phrases.py; plain
  git (`rev-parse`, `diff --stat`, `grep`, `log`).
- **Not checked:** I built, tested, timed and measured nothing. D4's figures are the two models' (not mine re-run);
  the disk figures after the part-cleans are estimates until D2 reads them; whether the brawl kills the lag-switch and
  fast-clock clients in a judged run is from the scene's design (fire held, targets in view), not a count; the server-ns
  form's tick-grid handling in the controller is the builder's design, shown by f5 and f36.

## Decisions needed (Mustafa)
- **LEAD-BOUND** (the lead bound's first value, which sets how loose -21's fast-clock checks are): its entry is the main
  session's to draft from the list above, after my and the test-engineer's read; options 400, 600 or 1,000 ms or another
  value; my recommendation 400 ms. Nothing that reads it is built before his yes.
- **KL-U4F**, only if the disk is short after the build (not now).

## For Mustafa
1. When the main session posts LEAD-BOUND: answer it in the chat (a yes to 400 ms, or another value). Nothing else
   from this check.
