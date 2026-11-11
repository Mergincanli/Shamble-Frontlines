## Changes in rev 2

One revision pass for netcode-architect's check (`netcode-architect-u4r-check.md`: REVISE, 0 Critical, 0 High, 8
Medium, 8 Low). Each finding's cite was rechecked at df8975b before acting on it; all 16 are accepted, two with a
correction to the check's own fix (N2, N6). "check Nk" below is the check's finding k; section 11's run items are
renamed D1-D12 (X1) so the two never collide.

- **N1 (Medium), accepted.** 7.2's RTT rule now times the sample of the server end of that client's L1 link, as the
  pump reads it, never the client end's (each link keeps one estimate per end: l1/src/lib.rs:456-458, :784-785; plan
  :755-757 says "the server's L1 link").
- **N2 (Medium), accepted, with a correction.** U4R-Q10 rewritten: (a') the run-level bite with the only timing that
  could fire it, and (b') a unit test on doctored records (new test r7 in 2.4), recommending (b') at U4r; 8.2 item 3
  reworded ("can fire only when a command runs in the instant JoinAccepted is sent"). Correction (X3): at df8975b a
  P0 ack's sample applies at its packet's own send instant, not at the next instant as the check says
  (l1/src/lib.rs:515 with 0 delay each way at P0, :418-420, and acks applied whenever `at_ns <= now`, :527, at any send
  or poll, :582, :661, :714), so even (a')'s timing may not fire; whether it can is U4c's and U4e's code (R4, R15 and
  new run item D12).
- **N3 (Low), accepted.** 7.2 logs one more field per client, the first command's receipt instant, so 8.4's "the
  re-run needs no new log" holds whichever instant U4h's estimate reads the RTT at; U4R-Q12 names both premises for
  U4h's carry (that comparand, and the Loopback leg's sessions taking the link's `rtt_ns`, not `ws` pings: §4.9
  :341-344); 7.2 asks the leg glue to class them so; R15's check gains it.
- **N4 (Medium), accepted.** U4R-Q12 now names both readings of the advisor's "6 runs" with their cost ((i) one
  recording, three pairs; (ii) -30's three-recording shape, marks.json:6389), recommends (i) with its reason, and adds
  why a third reading (three clean pairs on one recording) is not offered (a deterministic repeat). 7.2's "runs at
  U4r" bullet no longer adopts one reading of the advisor's words; 8.3 points to U4R-Q12.
- **N5 (Medium), accepted.** New recheck row R33 (every reader of the per-client RTT and every replace, clip or
  substitute path in npt-room and npt-server at `<base>`) and run item D10 with the check's two `git grep` lines (the
  second also searching `rewind_at`, so its body is read); 7.2 and 8.1 point to R33.
- **N6 (Medium), accepted, with a correction.** 7.2 gains "Pacing and the tick base (plan silent: state and test
  them)" and 2.4 gains test r6 (a clean short replay gives `late_count` 0 on both legs). Correction: the check names
  "the recording header's start tick", but §4.15 :463's recording header (scene, seed, TICK_HZ, pool sizes, capacity)
  has none: rev 2 takes the recording run's own `tick_start` and asks R30 whether `Recording` carries it. R17's check
  gains how far ahead option B keeps a command.
- **N7 (Medium), accepted.** 7.2's comparer and 8.2 item 3 name the hash: npt-sim's canonical per-tick hash of the
  room's `World` (FNV-1a-64: plan :590), one per tick in each leg's room log, never the mirror hash per (client, tick).
  Extra (X4): rev 1's 2.1 log list had no per-tick World hash in the room log, which the comparer reads (plan :634);
  added, and R30's check asks its name.
- **N8 (Medium), accepted.** 7.2's wallclock bite: the read must change a sim input on that leg on every run, whatever
  R is, never through the RTT, lateness or a rate bucket's refill; a bitten pair that comes out PASS is a missed bite,
  reported, never re-planted without saying so (marks.json:6377: "a wall-clock read on one transport's command path");
  U4R-Q6 and 2.5's row carry it. The check's second example (the accepted-command set) is left out: a command the pump
  drops can come back by redundancy or count as late, which makes the pair VOID.
- **N9 (Medium), accepted.** -18's heal record (2.1, 7.1): the first decode of a `Snapshot` delta encoded at or after
  the return; a `FullState` never counts; each burst's record notes whether a `FullState` reached that client between
  the burst's start and its heal; a bite run healed through one goes to the advisor with condition 6's report. New
  recheck row R34 and run item D11 (whether npt-client sends `Resync` on an undecodable delta; the event queue's cap).
- **N10 (Low), accepted.** 8.2 item 3 reworded to "the sim's inputs (which command runs at each tick, and its Rewind)
  are independent of the link once `late_count` is 0 and no view time is replaced or clipped"; 7.2's result and 7.4
  list per leg the admits, any drop or kick.
- **N11 (Low), accepted.** 7.2: "whose packet L1 acknowledges at once (U4c's immediate acks; nothing the replay client
  sends; Loopback has none)".
- **N12 (Low), accepted.** 7.2's RTT rule: (a) the first-sample instant as the pump reads it (link/src/lib.rs:79-80);
  (b) the run instant is the tick's due instant, VOID only if strictly earlier, equal instants not VOID; (c) the
  Loopback column reads "Some(0) from the start"; (d) 8.2 item 2 (and, for the same reason, 8.3's "Why") give U4c's
  333 ms with df8975b's `None` (l1/src/lib.rs:349-353) until R2 settles it.
- **N13 (Low), accepted, with one addition (X5).** 7.2 and 2.5: the late-command bite holds the command out of every
  `Input`, redundant copies included, until its tick has run on that leg, and then sends it (delayed, never dropped: a
  command never sent is a miss, which the late counter may not count); its records state the delayed command's tick
  and the leg's `late_count` (expected 1). R17's check gains how option B counts a late command.
- **N14 (Low), accepted.** 8.3's cost states both branches: 6 runs (about 1,800 s simulated) with no replacement, 9
  runs (about 2,700 s) otherwise. Extra (X6): rev 1's wall-time range there ("about 6 min (R = 300) to about 1.2 h (R =
  30)" for 2,100 s) was wrong by 7.3's own table (2,100 s is 3.5 min at R = 300 and 35 min at R = 30); recomputed.
- **N15 (Low), accepted.** -18 (2.1, 7.1): marks.json:4307's plain words named and the record following the ruling
  whichever words U4j's split carries, the report saying which; the builder states how the 10 offsets are drawn and
  the bursts spaced; `burst_eviction` per client and burst.
- **N16 (Low), accepted.** 7.2 marks the out-of-ring count as an addition beyond the plan, with its reason.
- **X1 (extra).** Section 11's run items N1-N9 renamed D1-D9 (they collided with the check's ids), every cite updated
  (4 rule 4; 7.3; 8.3; R2, R9-R12, R20, R29; U4R-Q3); new D10-D12. Carry 20's "arch N2" is the carry's own words, kept.
- **X2 (extra, found fixing N6).** Loopback delivers in the instant of the send (link/src/lib.rs:154-163, :187), L1
  only after it (l1/src/lib.rs:30-31, :727-728), so the two legs can admit a client at different instants: 7.2 asks
  the builder to state whether an admit changes the World's hashed state and logs each client's admit tick per leg.
- **X7 (extra).** The new written names (the RTT rule's three per-client instants, each client's admit tick, -18's
  `FullState` flag) join R26's list and U4R-Q2 (i); 6's return list items 2 and 3 name 7.2's statements and r6, r7;
  7.4's pre-check bullet lists the new records; section 9's preamble counts R33 and R34.
