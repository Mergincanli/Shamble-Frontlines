# Advisor: TE-M3 plan review (te-m3-plan.md rev 1), notes

## Plan
1. Read the plan, the runner, main-run.sh (done).
2. Read the logs (.gate/te-m3/: loop.log, one-h6.log, one-h6-1-4.log, build logs).
3. Read the rulings: advisor-u4d2-r1.md TE-M3 block (:287-310), advisor-u4j-r2.md B2.3.
4. Read the code at HEAD: delay.rs (pump, shutdown block, life/polls counting, current()), delay_tests.rs (the test,
   `until`, NeverShut), delay_clock.rs.
5. Check U3c's record (u3c-fix2-c2b.md:74; b2e879b's delay_tests.rs around :2002).
6. Check the cause step by step against the code; look for alternatives (e.g. pump double-call, a defect hidden).
7. Judge the change (smallest, hides nothing?), the proof P1-P6, the bound.
8. Rule on TE-Q1..TE-Q4; write advisor-te-m3.md.

## Blocks done
- Block 1 (plan, runner, main-run.sh read): done.
- Block 2 (logs): loop.log 60 of 60 exit 0, "169 passed; 0 failed; 12 ignored" each (an older runner's args "30 2";
  the runner was extended after, commit da6e42a). one-h6.log: 8 pass then 1-4 exit 101; one-h6-1-4.log panics at
  delay_tests.rs:2605:5 "3 shutdown calls", its own line (2 runs, 1 after, 3 calls, 3691096 ns late); frame 14 is
  the test fn at :2606:72, frame 15 closure at :2552:78. The 8 passes: (2, 1, 2), 0.17-2.27 ms late. Builds: same
  binary hash npt_net-301de27e05915ce7 both loops. Plan's numbers match the logs.
- Block 3 (rulings): u4d2-r1 :287-310 read (route: plan first, "never a retry, never a looser bound without its
  cause"; rule stays "until TE-M3's cause is found and fixed"); u4j-r2 B2.3 (:146-171) read.
- Block 4 (code, cc8595d = HEAD for spikes/, diff --quiet exit 0): every cited line checks (delay.rs:1621-1622,
  1640-1642, 1703-1710, 1848-1858, 1163-1175; tests :94-105, :2538-2541, :2551, :2580-2605). Pump::poll is one pass
  with lock blocks released between (opening block counts the run; shutdown block re-reads c.shutdown under a new
  lock). Only call site of S's poll_shutdown: delay.rs:1858 (grep), once per run. A task is polled by one thread at a
  time, so at most one run is under way when `before` is read. Deduction: s_calls <= polls + 1 is the exact invariant
  of correct code; (2,1,3) implies one uncounted run (started before `before`, shutdown block after the shutdown
  began). Shutter woken only when down_done or pump_done (wakers(), :844-853), so no early return of ds.shutdown().
- Block 4 finding candidates: (i) the doc comment keeps "and S's poll_shutdown no more often", which the new bound
  contradicts: must be replaced, not appended to; (ii) "(woken by the wait's own last check ...)" names one waker;
  the clock's fire for the model's last event (the final ACK) is as likely: the run under way needs no named waker;
  (iii) P4 names :2604/:2605, which move once the doc sentence and the multi-line assert land: name by message.
  (iv) s_calls <= 3 can fail on correct code: R0 under way and notified, the shutdown's wake delayed past R1's start
  -> (3 runs, 4 calls); polls + 1 is right.
- Block 5 (U3c record): b2e879b's test body is byte-identical to HEAD's (diff of :1940-2003 vs :2544-2607 empty);
  :2002 there is the last statement, as :2606 is now; the record names 2002 and the fn line 1948, the two lines of the
  frames named after the test (frames 14, 15 today), so the plan's reading is plausible, not proven; the raw output
  is gone (no copy under .gate/ or .checkpoints/). If 2002 was the panic site, it was "every byte, then EOF".
- Block 6 (my own reproduction of P1, through main-run.sh, HEAD 2f01711, spikes/ = cc8595d (diff --quiet exit 0)):
  `te_m3_loop.sh adv-h6 one 150 2 6` -> .gate/te-m3/adv-h6.log: 89 runs, then loop 2 run 44 exit 101
  (adv-h6-2-44.log): panicked at delay_tests.rs:2605:5 "3 shutdown calls", its line (2 runs, 1 after, 3 calls),
  ended only 191416 ns after the deadline (so lateness is not the trigger). Tally of the 89 own lines: 87 x (2,1,2),
  1 x (2,1,3) (the failure), 1 x (3,1,3) (the doc's third run, a pass). Same binary npt_net-301de27e05915ce7.
