# u3-verify3-harness: third skeptic round on harness v3 (fixes.rs) at 1e020a7

Agent: netcode-architect (skeptic). Started 2026-11-07. Writes only this file. No tracked edits, no commits.
Line numbers: fixes.rs and tcp_model.rs at 1e020a7 (spikes/netcode-proof/crates/net/src/). Scratch sources:
scratchpad = /tmp/claude-0/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/scratchpad.

## Plan
1. Read prior rounds (u3-verify2-harness.md, u3-build-v3-harness.md), commit message, fixes.rs whole.
2. (1) H6/H7/H8/H19, follow-on window, takeover counts: read code, cite lines.
3. (2) Chromium endpoint lock + WS throttler; Firefox admission queue + fail-delay: compare with sources
   (scratchpad chromium-src, cr2; fetch missing).
4. (3) TLS resumption flights: browser WS reconnect resumption; ClientHello size; 1,300 B resumed server flight
   with ML-KEM; full-handshake option.
5. (4) Multi-burst lineage: accumulation between bursts; why abort-500 Chromium hits 4,994 ms throttle by burst 30.
6. (5) Windows-like client option: client_cfg applied to both ends; server-end fidelity.
7. (6) Censoring at 60 s.
8. (7) Policy recommendation; (8) ADR 0001 / plan conflicts update; L2 changes (H17/H18).
9. (9) Too-good / too-bad hazards. Run tests and tables in a throwaway target dir; compare with u3v3-tables1.txt.
10. Verdicts, bugs, must-correct. End with DONE.

## Findings (a line per finding as I go)
- V0 (state): head 1e020a7 + checkpoint commits; spikes/ unchanged since 1e020a7. Read: round 2 notes (H1-H20), the
  harness build notes, the commit message, fixes.rs whole (1,976 lines), .gate/netcode-proof-test/u3v3-tables1.txt
  (4 tests, 171 s). Every number in the task's list appears in that output (spot-checked: tail recommended
  654/1,051/1,859/2,092/4,196, over 1/1.5/2 s 6,768/1,009/205, over 3 s 4 (Wilson 0.007 per mille); multi-burst
  recommended 10 x 2,000: 662/1,091, max 2,234; abort-500 Chromium 30 x 1,000 max 5,220, throttle 4,994 ms; side
  effects 3/22/142/219 per mille, attempts 0/9/15/7, takeovers 0/2/1/1; search 0.997 [0.993-0.999] and 0.990
  [0.980-0.997]). Exception, see V12: "the reconnect gains nothing" for Windows 3 s is not what the table shows.
- V1 SOURCE Chromium throttler (scratchpad/cr2/services_network_websocket_throttler.cc, saved 2026-11-07 from
  chromium main): delay = RandIntInclusive(1000, 5000) * (1 << min(p + f/(s+1), 16)) / 65536 in int (:46-55):
  whole ms; failed = tracker destroyed before OnCompleteHandshake (:27-33); success at OnAddChannelResponse
  (services_network_websocket.cc:362-374, the 101); Roll prev = cur, cur = 0 (:70-76); IsClean (:62-68) checks
  previous successes twice, previous failures never, so after a roll a throttler with no pending tracker and no
  success in the period just ended is erased (OnTimer :131-144) and the next WebSocket finds none (0 ms, :100-102).
  The delay is applied before AddChannel (websocket.cc:649-658), so the lock and TCP connect wait for it. Model
  (fixes.rs:200-256): formula, int ms, below(draw, 4001) = 1,000-5,000 inclusive, two-bucket roll, erase rule, timer
  start at the first tracker (:239-241 vs :124-127), success at the 101 (fixes.rs:675-677 -> :311-316), failure on
  every pre-101 end (close_conn :606-614, cancel :706-732, switch :699-701, abort :817-828): CONFIRMED. Sure: high.
- V2 SOURCE argument order (scratchpad/u3v3h/src/websocket_factory.cc:93-101: IssuePendingConnectionTracker and
  CalculateDelay are arguments of one make_unique call; clang fetched 2026-11-07 into scratchpad/v3h-clang:
  TargetCXXABI.h:188-190 areArgsDestroyedLeftToRightInCallee() = isMicrosoft(); CallUtils.cpp:55-60
  shouldEvaluateArgsLeftToRight = false under that ABI unless ForceLeftToRight; CGCall.cpp:5155-5161 "We must
  evaluate arguments from right to left in the MS C++ ABI"). So on Windows (clang-cl, MS ABI) CalculateDelay runs
  first: p leaves out the new tracker; on Linux, macOS, Android, ChromeOS (Itanium ABI) left to right: p counts it
  (one more doubling; no change at the cap or while the throttler does not exist). CONFIRMED. Unmodelled detail: a
  page's close() of the blocker reaches the network service through a posted task and a Mojo disconnect (round 2
  H1), so the closed tracker may still be pending when the new one is issued: p one higher (one more doubling) in
  the abort rows; the model frees it at once (fixes.rs:817-829). Direction: abort rows slightly too good in Chromium.
- V3 SOURCE Firefox (scratchpad/chromium-src/ff_WebSocketChannel.cpp): FailDelay 200 + rand()%200 (:128-130),
  FailedAgain x1.5 capped 60 s (:134-145), RemainingDelay 0 once since > next (:148-154), IsExpired at last + 60 s +
  delay (:156-160), Lookup returns a matching record even when expired (:199-222), DelayOrBegin (:225-266),
  OnConnected at the upgrade OK + transport (:3800-3808, :4053-4061) removes the record and expired ones (:268-281),
  OnStopSession: known record grows unless NS_ERROR_NOT_CONNECTED, none: Add (:389-421); Close() before data
  started stops with NS_ERROR_NOT_CONNECTED unless 1001 (:3630-3655); queue keyed by address + origin suffix, no
  port (:540-552); open timer 20 s (:1181, :1393). Model (fixes.rs:258-372, :862-875): CONFIRMED, one admission slot
  across endpoints, delayed attempts start without a re-check (as the timer path does). Sure: high.
- V4 (Firefox, unmodelled, design point): OnStopSession runs for an ESTABLISHED channel too (:2459-2463), and any
  failed reason adds or grows (x1.5, last failure moved) the (IP, path, port) record the next reconnect consults:
  a close handshake that times out (mCloseTimeout 20 s, :1180, :2094-2098 -> :3280-3290 NS_ERROR_NET_TIMEOUT_EXTERNAL)
  or a reset from the server. With one endpoint the old connection and the reconnect share that key. If the room
  resets a replaced connection (closing a socket with unread input makes Linux send RST), each takeover leaves a
  failure; failures within 60 s of each other grow it x1.5 (300 ms x 1.5^10 = 17 s after 10). With B1's 20 s spacing
  a 200-400 ms delay never reaches the next burst, but repeated outages closer together would. Requirement for the
  ADR: the room closes a replaced connection with a WebSocket close frame and a graceful TCP close (reads until the
  client's close), never a reset (also round 2 H13's reason). Not measured.
