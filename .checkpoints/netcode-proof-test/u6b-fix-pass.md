# U6b's fix list after its one round (my leans; the advisor rules the items marked ADVISOR)
2026-11-23 21:4x UTC. Snapshot 49f4488f… (5,498 added; the stop past 6,000). Reviews: u6b-review-netcode.md
(APPROVE WITH FINDINGS: 2 Medium, 12 Low), u6b-review-security.md (REVISE: 2 Medium, 3 Low), u6b-review-test.md
(REVISE: 1 High, 8 Medium, 12 Low). Every Medium and the High read in full in those files.

## 1. The size line
- Fixing every Medium in place, and the High with its plant, adds about 450-500 lines: about 5,950-6,000 before any
  Low or reading, so the stop would likely come in the fix pass.
- Lean, for the advisor: the valve's second half leaves U6b now. That is Bite::BudgetX2, WsServer::with_bite,
  WS_BUDGET_BITE, the binary's arming and the two pm_0001_18_* bite tests, about 100-130 lines. It is the bite
  PM-0001-18/ws-budget-x2, which R3 runs and R1 does not (F-VALVE; R1's bites block runs PM-0001-1, PM-0006-11 and
  PM-0001-16). It goes to U6e with T-1 as its condition, word for word: it must be armed through an instance's
  accepted connection, with a plant removing the per-link arming.
  - This is the valve's own rule, applied now: "past 5,400 they wait for U6e if R1 reads neither". The reader stamp
    stays (NR-U6B-9: R1's).
  - With it: about 5,850-5,900 after the Mediums and NR-U6B-3, so room for the R1 Lows below.
- **ADVISOR (A1):** is the High T-1 closed for U6b by that move, with T-1 a condition on U6e, or fixed here with a
  second round?

## 2. Fix now (in this order; each with its test and, where named, a plant; one notes line each with the running size)
1. **SEC-U6B-1:** from a link's drop, the drain and the close run under one bound (CLOSE_WAIT), then `session.close`;
   the same around WsLink's drain loop. A test with a peer that never grants control-stream credit gets its permit
   back within the bound; a plant (the bound removed) fails it. This also covers T-13.
2. **SEC-U6B-2:** the server's `transport()` sets small stream counts, a 64 KiB stream window and a 1-2 MiB connection
   window, with a test. The unidirectional count allows HTTP/3's three (control, QPACK encoder, QPACK decoder) plus
   margin, stated; U7's headless Chromium run confirms it later.
3. **NR-U6B-1:** the sessions end first (stop, the tick thread joined, the session tasks awaited within FLUSH while the
   log writer stays open), then `Net::end`, then the log's end. A binary test: a WsClient still connected when stdin
   closes gives one `ws_summary`, and `ws_split` comes after it.
4. **NR-U6B-2:** the gate's tick period comes from the caller (`tick_ns` through `Listen` or `WsServer::bind`;
   npt-server passes 1e9 / npt_sim::TICK_HZ). `TICK_NS` stays in tests only; a 60 Hz test gives N = 10 at srtt 100 ms.
5. **T-2 to T-9**, the test seat's Mediums, as its table words each fix:
   - T-2: the injected clock moves during the handshake;
   - T-3: a bounded wait on the arrivals in place of the 300 ms sleep;
   - T-4: the 5 s real-clock cap;
   - T-5: the report and the self-test assert R as the first closing wake after its release;
   - T-6: the pool's mark test on a local Pool, which also covers NR-U6B-12 and T-20;
   - T-7: the serving stamps against OPEN, and P14 re-planted between bind and serve;
   - T-8: timeouts and the Open assert in the end-to-end test;
   - T-9: FULL_STATE_MAX exactly.
6. **NR-U6B-3 (F-PING)**, once the advisor words condition 1:
   - outside datagrams above the largest Ping or Pong are refused and counted;
   - condition 1 leaves out the link's own queue for outside datagrams;
   - both counts go in `event.ws_accept` (two SERIES rows);
   - the trait doc names the rule.
7. **R1-relevant Lows, while the running size stays under about 5,950** (in this order):
   - NR-U6B-6: the mark on the room's clock, one row; R1.md's INVALID rule counts from the mark;
   - SEC-U6B-4 = T-15: a negative test for the pin's signature check, and its plant;
   - NR-U6B-4: an empty tick, and `announced` decremented before the size check;
   - NR-U6B-5: one header row for the pool's added heap;
   - T-12: the bite path pushes `r`;
   - SEC-U6B-5: P15's test lists the whole run folder;
   - T-17: the hash file's mode against the umask.
   Anything past about 5,950 is carried by number (section 3).
8. **The readings, no crate code:** 20 starts to OPEN by a scratch script under the notes folder (the release binary,
   a fresh token file, log and hash path for each; p50 and max; any failure named). The record follows
   u6b-review-test.md section 5, with T-10: each plant's `git diff` is saved beside its log, and T-19: F28's plant.
9. **End checks:** check.mjs twice, its count lines against the base, every plant (old and new) failing its named test.
   Only then the size line and the return.

## 3. Carried by number (none blocks R1)
- **U6d:** NR-U6B-11 (the unit of `unsent_unreliable`), NR-U6B-13 (refused sessions counted), NR-U6B-14 (the plain
  path's drift), T-16 (C1c on DelayStream).
- **U6e:** the valve's bite with T-1 (if A1 agrees), T-11 (the WT kick test runs concurrently), T-14 (a deadline
  inside QUIC's handshake), T-18 (the early class asserted), T-21 (port 0).
- **U7's or U8's record step:** NR-U6B-10 (F-NEG: a presence-only draft count, security to confirm E22, or -56 item 2
  by number).
- **The security baseline:** SEC-U6B-3 (Retry before the permit).
- **Report text only:** NR-U6B-7 (the skipped row's one-row shift), NR-U6B-8 (the extra pub items), NR-U6B-9 (the
  reader stamp is R1's), T-20 (the named-vs-extra count split).

## 4. The 48-connection reading and the memory bound: ADVISOR (A2)
- Lean: at U6c's close, not as an example in U6b. U6c's load generator is the tool R1 uses: 48 WsClients at P2 for
  60 s against the release server. The four parts, early readers, violations, pooled L and the mark come from the
  server's own `ws_split`; VmHWM is read before any connection and at 56.
- Condition 8 allows this ("before Stop B's read of R1's run plan"). It saves an example's 150-250 lines here.

## 5. A second round: ADVISOR (A3)
- With A1's move, no High stays in U6b; Mediums below the bar do not open a second round (NPT-TRIM).
- Lean: the advisor's check of the fix diff, with the three seats' findings beside it, before the merge's M-steps.
- Without the move: the test seat reads T-1's fix and T-2 to T-9's (round 2, on what changed).
