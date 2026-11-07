# u3-verify3-harness: third skeptic round on harness v3 (fixes.rs) at 1e020a7

Agent: netcode-architect (skeptic). Started 2026-11-07. Writes only this file. No tracked edits, no commits.
Line numbers: fixes.rs and tcp_model.rs at 1e020a7 (spikes/netcode-proof/crates/net/src/). Scratch sources:
scratchpad = /tmp/claude-0/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/scratchpad.
Diagnostics: throwaway worktree .gate/u3v3v-wt (detached at 8ba8522 = 1e020a7 + checkpoints), target
.gate/u3v3v-target; patch kept at scratchpad/v3h-diag/diag.patch (outputs rows.txt, ratchet.txt, order.txt,
worst.txt, nocancel.txt there). The patch changes nothing with its flags off: the pinned test passes with it.

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
  [0.980-0.997]). My worktree: model + harness tests 49 pass, 7 ignored (0.30 s); the pin holds.
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
  (one more doubling; no change at the cap or while the throttler does not exist). CONFIRMED. Measured (order.txt,
  30 bursts x 1,000): left-to-right changes the recoveries by noise only (recommended p99 1,096 -> 1,098, max 2,258
  both; abort-500 p99.99 3,399 -> 2,870, max 5,220 -> 5,170); the throttle reaches 1,059 ms instead of 421 in the
  recommended row. Unmodelled detail: a page's close() of the blocker reaches the network service through a posted
  task and a Mojo disconnect (round 2 H1), so the closed tracker may still be pending when the new one is issued: p
  one higher in the abort rows; the model frees it at once (fixes.rs:817-829). Direction: abort rows slightly good.
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
  failure; failures within 60 s of each other grow it x1.5 (after about 11 failures 20 s apart, 300 x 1.5^11 = 26 s
  exceeds the spacing and the next reconnect waits). Requirement for the ADR: the room closes a replaced connection
  with a WebSocket close frame and a graceful TCP close (reads until the client's close), never a reset (also round 2
  H13's reason). Not measured.
- V5 SOURCE Windows' SYN timer (WebFetch 2026-11-07, Microsoft Tech Community networking blog "TCP Templates for
  Windows Server 2019 - How to tune your..." (first published on TechNet 2018-10-03), its Get-NetTCPSetting
  -SettingName InternetCustom output after `netsh int tcp reset` ("Reset to default"): InitialRto(ms) 3000,
  MaxSynRetransmissions 2, Timestamps Disabled, MinRto(ms) 300, DelayedAckTimeout(ms) 40). That is Server 2019 (the
  Windows 10 1809 stack); no primary source found for Windows 11 clients (searches 2026-11-07: a Thales admin doc
  also says default 3000, range 300-3000; forum outputs with 1000 or 2000 are tuned machines). So the Windows-like
  3 s row is plausibly the DEFAULT Windows client, not an outlier. Sure: medium. Mustafa's check settles his own:
  `Get-NetTCPSetting -SettingName InternetCustom,Internet | Format-List SettingName,InitialRtoMs,MaxSynRetransmissions`
  (or `netsh interface tcp show global`, line "Initial RTO").
- V6 (diag rows.txt, 60,000 phases each): fix 1 alone with no timestamps (= any Windows-like client, the SYN timer
  plays no part without a reconnect): 646/2,035/3,829/5,295/12,550; over 1/1.5/2 s 2,396/1,727/843. Against it the
  never-abandoned reconnect on a Windows-like client: SYN 3 s 646/2,035/2,729/3,069/3,890, over 1/1.5/2 s
  2,393/1,727/844; SYN 2 s 646/1,739/2,027/2,147/3,890, over 1/1.5/2 s 2,393/1,728/91. So the main session's "with
  3 s the reconnect gains nothing" is CONFIRMED for everything up to 2 s (the counts match to the unit) and wrong past
  2.7 s: it still cuts p99.9 3,829 -> 2,729 and the max 12,550 -> 3,890. With SYN 1 s: 886/1,484/2,871, over 1 s 285.
  Abort at 500 ms is identical for SYN 1, 2 and 3 s (645/903/1,347/1,746/2,607, over 1 s 349): it never waits for an
  OS SYN retry (each attempt is closed 500 ms after creation, before its first SYN timer fires).
- V7 (side observation for the model skeptic): fix 1 alone WITH timestamps (the Linux client) is worse than without:
  over 1 s 5,336 vs 2,396, p99 2,170 vs 2,035 (60,000 each). The Linux-client rows carry this; not judged here.
- V8 (diag ratchet.txt, abort-500 Chromium, 30 bursts x 1,000 lineages): 127 of 1,000 lineages reach a >= 1 s
  throttle, the first at burst 10 or later. Worst lineage (r 757): bursts 1-12 each made 3 attempts and ALL were
  failures (2 aborted at 500 ms, the third closed by H6 when the old connection recovered first: 0 takeovers), so f
  grew by 3 a burst while s stayed at the page's first WebSocket's 1: f/(s+1) = 36/2 at burst 12, the cap 16, delays
  4,072/4,994/4,554 ms; at the 240 s roll the period just ended had no success, IsClean erased the throttler and
  burst 13's attempts started at 0 ms again (and won a takeover). So the 4,994 ms is Chromium's rule faithfully run:
  abort-500 makes about 3 failures and 0.49 successes a burst; the exponent reaches 4 (delays up to 1.2 s) once
  f >= 4(s+1), i.e. after about 8-10 bursts in 4 minutes without a winning attempt; IsClean bounds it to the 2-minute
  periods. The never-abandoned row makes about 0.66 failures (H6 closes) and 0.34 successes a burst: max 421 ms.
- V9 (diag worst.txt, recommended, 600,000 phases): 10 phases over 2.5 s; the worst four are the reconnect's OWN
  handshake in backoff: seed 179118 attempt began at 253 ms, its 101 at 5,059 ms (4,196; fix 1 alone 4,936); seeds
  67783 and 556300: no 101 by 4.4-4.8 s, the old connection recovered first (3,766, 3,375 = fix 1 alone); seed 10296:
  101 at 2,788 ms, its Join lost, takeover at 4,578 ms after the old connection recovered at 4,013 (3,013). A
  never-abandoned attempt that loses a flight waits on a new connection's 1 s initial RTO and its doublings; an abort
  cadence starts fresh. That is why abort-500 has the better p99.9/max in single bursts (1,376/2,544 vs 1,859/4,196).
- V10 (diag worst.txt, 10,000 phases): every takeover after the recovery (1,742) had its 101, so its Join, at or
  before the recovery: "after its 101 it runs to its Join" (fixes.rs:653-658, :800-804) costs nothing a page could
  avoid by a check at the 101; once a Join is sent the page must let it land (closing would let the room take the
  session over to a dead connection). CONFIRMED as the right choice.
