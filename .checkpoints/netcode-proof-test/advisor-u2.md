# Advisor: U2 unit-close review (netcode proof test)

Scope: plan rev 3.2 §12.1 U2 (npt-l1 + unit/property tests; PM-0001-3's L1 half and its bites).
Diff: c104302..6900e52 -- spikes/ (commits 08782eb 921174d 2529675 81c6d01 6900e52).
Not a merge review. Rulings U2-R1..R8, then the unit verdict.

## Plan (my steps)
1. Read plan §4.7, §4.3 E15, §6.1, §6.4, §12.1; ADR 0001 part 13 + PM-0001-3 row; marks.json PM-0001-3.
2. Read the diff: link, l1, l1 tests, lab (main, probe, machine), judge figures + u2 tests, run script, check.mjs, SERIES.md.
3. Read the committed verdict.json and runs.txt; spot-check numbers.
4. Run check.mjs myself (if toolchain present) and reproduce key numbers.
5. Rule R1..R8; unit verdict.

## Notes
- (start) head checked; notes file created.
- Read: plan §4.3 (E15), §4.4, §4.7, §6.1-6.4, §12.1; ADR 0001 part 13 (568-586) and rows -3, -18, -20, -58;
  marks.json -3, -20, -58; the checkpoint (steps 25-29).
- Read: link/src/lib.rs, l1/src/lib.rs, l1/tests/l1.rs, lab/src/{main,probe}.rs, judge figures.rs diff, engine.rs
  u2_ tests, ops/runs/c-u2-pm-0001-3.mjs, results runs.txt + verdict.json.
- verdict.json: 5 L1 parts PASS (P1-P4 25 checks/run, B1 27); L2 parts, MP, not-net-of-base NOT-RUN. Bites:
  rtt-each-way fails mean/p001/p999/p90 high + RTT high (9 checks); loss-per-round-trip fails loss low both ways
  (1.01 %); normal-jitter fails p90 low (54.2 ms) only; clock-seed fails the twin check only. Matches the claims.
- runs.txt: means sit +0.04-0.09 % above the profile (P2 +38-45 us): the reorder step's holds at the probe rate
  (held delay = max(own, succ+0.1 ms), E = d + a/3, x1 % = +a/300: P2 +33 us, P4 +67 us). Explained; inside 5 %.
- runs.txt: same seed gives identical loss counts at P3 and P4 (same ppm, profile not in the key): common random
  numbers across profiles; each profile's part stands alone. Not a defect.
- -58: reorder validity only on its P3 part; -20 at P3 only: no mark names reordering at P0 or P1, so P0/P1 share 0
  blocks nothing.
- REPRODUCED: check.mjs all 9 PASS at a56d4be (97 measured / 102 bites tests; 18 s).
- REPRODUCED: clippy on the measured build (no features), host and wasm32 leaf crates: clean too.
- REPRODUCED: my own release build of npt-lab (binary sha differs from the run's: 8714e453 vs 05a9d4df, same code)
  gives the committed fate-log hashes at P2, B1, P4 seed 1 (913db8e9, feabf2f0, b8a04f2f) and identical summaries.
- REPRODUCED: my python over my P2 / B1 logs gives the judge's values exactly (P2 up loss 0.020191, mean
  50038109.38, p001 40020299, p90 58029785, p999 59981998; RTT mean 100074285.6 over 610746). Max delay 60.099 ms
  (the hold's +0.1 ms at the probe rate).
- R7 finding: check.mjs says "features only add code; none is compiled out by one" - false: probe.rs:231 and :463
  are cfg(not(feature = "bites")), so clippy --all-features never lints them. Today clean either way.
- R6 finding: the judge has no rule that a measured run comes from a build without bites (header build.features
  empty, bite null); plan §4.7 "measured builds never contain them" rests on the run script alone. U2's logs are
  right (measured: features [], bites: ["bites"]).
- SCRATCH (scratchpad r1probe, path deps on npt-l1/npt-link, 60,000 datagrams/way at 30 Hz, seeds 1-3):
  P0 0 %; P1 0 % (1 datagram/tick), max 30.00 ms; P2 0.92-1.06 %, above ceiling ~1 % (max 93.3 ms), below floor 0;
  P3 0.90-1.02 % (max 123.3 ms); P4 2.20-2.31 % (jitter alone 1.30-1.40 %; max 153.3 ms). Step off: 0 % at P0-P3.
  P1 with 3 down datagrams per tick in one instant: 38.7-39.0 % down (the jitter, not the step).
- SCRATCH: B1 stream message sent at burst start 5.000 s arrives 6.053 s (53 ms after the burst ends); at 5.5 s ->
  6.153; 5.999 -> 6.252. No backoff: TCP's 200/400/800 ms would give 6.45 s for the first. Optimistic by up to
  ~400 ms against -18's 500 ms heal limit, if -18's heal path uses a stream.
- CODE (l1 send_stream): one loss draw per message whatever its size (E15's L2 model draws per 1,448 B segment);
  RTT sample taken at the HOL-delayed delivery (arrival = max(own, predecessor)), so messages behind a retransmitted
  one give samples inflated by up to an RTO; RFC 9002 §5.1 would sample only newly-largest acks (overtaken and
  HOL-held packets give none). rtt_ns at P2 30 Hz end values 101.0-105.5 ms (step on) vs 101.0-103.2 (off).
- plan.md not updated in U2 (no LinkState, rtt_ns, npt-lab binary/probe). SERIES.md lacks reorder_step_permille,
  probe_rate_pps, probe_sent_count, burst_starts_ns; its fate_log_twin_equal row lacks the match fields and the
  NOT-RUN/gap rules the code has.
- Judge: a run with any Cannot check makes its part NOT-RUN (eval.rs:1186-1236), so a missing twin can never let a
  part PASS. R8's NOT-RUN is conservative.
- REPRODUCED: legacy gate `node tools/gate.mjs --no-smoke` at a56d4be: PASS (1-4, 5b-5d; symbols +0 -0); tools/ and
  .github/ equal main. CI check runs gate + smoke: success on 6900e52 and 81c6d01.
- Note for U6: Link is used as Box<dyn Link> without Send (L1 is Rc/RefCell, single-threaded); if npt-server's
  runtime moves links across tasks, the L2 backends need `+ Send` on the Listener's return type then.

## Rulings
- U2-R1 APPROVE. It is part 13's "1 % of adjacent datagram pairs swapped" in the only causal form that keeps the
  floor: any swap at 33 ms spacing with +-10 ms one-way jitter puts one of the pair outside the range; holding the
  earlier one puts it above the ceiling, never below the floor (the rejected exchange gives 7-27 ms at P2; netem's
  skip gives 0 ms). Measured (scratch, 3 seeds): P2 0.92-1.06 %, P3 0.90-1.02 %, P4 2.2-2.3 % (jitter 1.3-1.4 % +
  step). Step on in every run, P0 share 0 and P1 (1 datagram per tick) 0 by timing: right; part 13's motivation
  names P2 and P3, and no mark names reordering at P0 or P1 (-20 at P3; -58's reorder rule on its P3 part only).
  Condition N2: the crate doc states the cost (about 1 % of datagrams at P2-P4 30 Hz arrive above the ceiling by
  up to one spacing: P2 max 93.3 ms vs 60). Condition N1: the reading goes in plan §15's R-5 row. Carried C5:
  ImpairSocket's step uses the same model (U3/U6).
- U2-R2 APPROVE for U2 (nothing in U2 reads stream timing). Carried C1, due before any L1 mark reads stream timing
  (PM-0001-18 first, U4): (a) consecutive retries back off (twice the last timeout: RFC 6298 §5.5, RFC 9002
  §6.2.1); (b) loss drawn per 1,448 B segment, any lost segment delaying the message one timeout and holding later
  ones (E15's L2 model); each with a unit test; L1 runs record each stream's stall share (E15). Why: a message sent
  at a B1 burst's start arrives 53 ms after the burst on L1, about 450 ms with TCP's backoff; -18's limit is 500 ms.
  The first timeout max(200 ms, 2 x RTT) is conservative against QUIC's first PTO (about 155-160 ms at P2) and
  equal to Linux's 200 ms floor at P2.
- U2-R3 APPROVE. RFC 9002 §5.3's update is exact (unit test); Karn's rule holds. Carried C2, due before npt-room
  reads rtt_ns (U4): sample only when the packet is the newest of its link and direction to arrive (RFC 9002 §5.1's
  "largest acknowledged newly acknowledged": overtaken and reordered packets give none), and a stream message's
  sample from its own packet arrival, not its in-order delivery (today a message behind a retransmitted one gives a
  sample inflated by up to an RTO); tests for both. U2's -3 reads RTT by echo, not rtt_ns, so its verdict stands.
- U2-R4 APPROVE. LinkState is the least `state` can return (Connecting is for npt-net's clients, E6/E7, not "for
  later"); rtt_ns names its unit; the Datagram panic is a caller's bug, never peer-driven; Sent on a closed link
  matches a datagram to a vanished peer, with `state` the signal. `loss`: add it at U4 with npt-room, its first
  consumer (carried C3), as each backend's own estimate (L1 from its implicit acks; quinn's path stats; None on
  WebSocket), never the simulator's fate, like rtt; §4.4's send-buffer space settled then too. Condition N1: plan
  §4.7 names LinkState, rtt_ns and Recv.arrival_ns now.
- U2-R5 APPROVE. npt-lab must be a binary anyway (§4.2: nothing depends on it; the runner and replays of §4.7);
  `npt-lab probe` belongs there and reads only through npt-link. Echo with hold time is E10's design; the echo choice
  (newest arrival since the last echo) carries no delay bias. reorder_permille = the lower direction is the right
  reading of "each stream's share above 0" (its floor to whole permille can only make a run INVALID, never PASS).
  Conditions: N1 (plan §4.7's binaries list npt-lab with `probe`; §4.1's ops/runs/ holds c-*.mjs; results/ layout);
  N3 (SERIES.md lists reorder_step_permille, probe_rate_pps, probe_sent_count, burst_starts_ns and the up/down
  shares as rows). Carried C5: SERIES.md names the L2 probe's writing binary at U3/U6, not only the library.
- U2-R6 APPROVE. 15 twins are what marks.json's per-part twin checks need; bite twins from the bites build make
  each bite show by its own check (verified in verdict.json: rtt-each-way fails the delay and RTT checks only; loss
  2 loss-low checks; normal-jitter the 2 p90 checks only; clock-seed the twin check only). clock_res_ns = 1 is right
  for a simulated-clock run (N3: say so in README §1). Records: verdict.json + runs.txt committed, raw logs in
  .gate/ (plan §10.6). Carried C4 (U4, with the next judge change): a measured run, or a twin of one, whose build
  lists `bites` or names a bite, and a bite run whose build lacks it, are INVALID, with fixtures (§4.7's "measured
  builds never contain them" today rests on the run script alone; U2's logs checked by hand: right).
- U2-R7 APPROVE with N4: clippy host and wasm32 on both builds (default features, then --all-features), as the
  tests do; the comment "none is compiled out by one" is wrong (probe.rs:231, :463 are cfg(not(feature="bites"))).
  Today clean both ways (my run). Tests on both builds and stdout-only parsing: right.
- U2-R8 APPROVE. Conservative: a missing twin makes the part NOT-RUN (any Cannot check does, eval.rs:1186-1236), so
  it can never PASS; the gap cases (no twin named, wrong role, other layer/profile/seed/bite, bad hash) are record
  faults, INVALID. N3: SERIES.md's fate_log_twin_equal row states these rules as the code has them.

## Verdict on U2
- The unit definition is met: npt-l1 with unit and fixed-seed property tests; PM-0001-3's five L1 parts PASS on 3
  seeds with twins; its four L1 bites shown; check.mjs green; gate run; CI green. CLOSED once N1-N4 land (docs, the
  crate doc and check.mjs only; none moves a measured number, so no re-run). C1-C5 carried into the checkpoint's
  open findings with their units.
- Nothing for Mustafa: every point is technical inside the approved plan; nothing a player sees.

DONE
