# u3-verify3-harness: third skeptic round on harness v3 (fixes.rs) at 1e020a7

Agent: netcode-architect (skeptic). Started 2026-11-07. Writes only this file. No tracked edits, no commits.
Line numbers: fixes.rs and tcp_model.rs at 1e020a7 (spikes/netcode-proof/crates/net/src/). Scratch sources:
scratchpad = /tmp/claude-0/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/scratchpad.
Diagnostics: throwaway worktree .gate/u3v3v-wt (detached at 8ba8522 = 1e020a7 + checkpoints), target
.gate/u3v3v-target; patch kept at scratchpad/v3h-diag/diag.patch (outputs rows.txt, ratchet.txt, order.txt,
worst.txt, nocancel.txt, profiles.txt there; the burst-length sweep was printed to the console, copied in V13). The
patch changes nothing with its flags off: the pinned test passes with it.

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
  2.7 s: it still cuts p99.9 3,829 -> 2,729 and the max 12,550 -> 3,890. The right baseline is fix 1 alone on the same
  no-timestamp client (not the table's Linux fix-1 row, 2,164/3,945/11,384). With SYN 1 s: 886/1,484/2,871, over 1 s
  285. Abort at 500 ms is identical for SYN 1, 2 and 3 s (645/903/1,347/1,746/2,607, over 1 s 349): it never waits for
  an OS SYN retry (each attempt is closed 500 ms after creation, before its first SYN timer fires).
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
- V11 (diag nocancel.txt; a policy variant, "join-or-close at the 101": never close a pending attempt because the
  old connection recovered; at its 101 send the Join only if the current connection has delivered no whole snapshot
  written at or after the attempt's creation, else close it cleanly with no Join). Single bursts (60,000): identical
  recoveries and takeovers to H6 in every row (the recovery happens either way; V10), so the tables stand. But each
  such attempt now ends as a SUCCESS in Chromium's throttler (and an OnConnected in Firefox, which clears the fail
  record) instead of a failure. 30 bursts x 1,000, Chromium (Windows order): throttled >= 1 ms in 0 bursts in every
  row (was 635 / 10,208 / 13,990). Abort-500: p99 1,202 -> 1,173, p99.9 1,914 -> 1,408, p99.99 3,399 -> 1,858, max
  5,220 -> 2,105, over 2 s 23 -> 2; abort-500 on a Windows-like 3 s client 935/1,587/2,119/3,774 -> 916/1,354/1,795/
  2,119; abort-500 in Firefox 1,160/1,579/2,087/2,290 -> 1,146/1,367/2,019/2,153; never abandoned unchanged
  (1,096/1,821/2,114/2,258). Cost: about 0.67 (never abandoned) or 0.55 (abort-500) extra completed TCP+TLS handshakes
  plus an upgrade per outage that the room sees closed without a Join (inside part 10's 2 s Join rule and the
  handshake cap); attempts linger: still pending 3 s after the recovery in 86 / 49 phases (Linux) and 2,151 / 1,368
  (Windows 3 s) of 60,000, holding the endpoint lock until they open or the next outage aborts them.
- V12 (diag profiles.txt, a 1 s burst on P2, P3, P4, 20,000 phases each; p99/p99.9/max): a FIXED 500 ms abort fails
  at P4 (RTT 200 ms: TCP + TLS + upgrade need 3 RTT plus losses, so attempts die before their 101; 169 phases hit the
  8-attempt cap): 3,501/4,616/6,657 vs never abandoned 2,410/3,087/4,559. Scaled, abort at max(500 ms, 4 x the
  client's echo RTT) with join-or-close: P2 1,126/1,498/2,544 (never abandoned 1,053/1,840/3,013); P3 1,907/2,734/
  3,630 (1,887/2,781/4,872); P4 2,502/3,418/6,189 (2,410/3,087/4,559). On a Windows-like 3 s client the scaled abort
  wins everywhere: P2 924/1,343/2,607 vs 2,027/2,729/3,890; P3 1,727/2,551/3,640 vs 2,774/3,869/7,205; P4
  2,393/3,305/7,357 vs 3,307/4,375/10,316. No-burst side effects, 30 min: P2/P3/P4 attempts 9/15/7, aborts 0/0/0,
  takeovers 2/1/1 (the same as never abandoned); the fixed 500 ms at P4: 11 attempts, 1 abort, 3 takeovers.
- V13 (diag burst-length sweep, P2, 20,000 phases each, p50/p99/max; over 1.5 s): B1's 1 s burst is the one length
  where "never abandoned" shines, because the Linux SYN retry 1 s after the attempt (~0.36 s into the burst) lands
  0.36 s after the return. 0.6 s burst: never abandoned 964/1,270/2,570 (over 1 s 8,780 = 44 %); scaled abort +
  join-or-close 726/1,135/1,901 (1,225); Windows 3 s: 950/1,467/3,894 vs 724/1,123/1,927. 1.5 s burst: 1,330/1,632/
  3,655 (over 1.5 s 613) vs 844/1,378/2,297 (35); Windows 3 s 1,381/1,804/7,661 vs 844/1,421/1,826. 2.5 s burst:
  493/2,466/4,289 (over 1.5 s 2,501) vs 488/1,463/3,228 (166); Windows 3 s 541/2,371/7,842 vs 534/1,454/3,228. Fix 1
  alone: 964/2,454/5,722; 1,330/2,483/12,514; 493/6,462/20,855. Costs of the scaled abort: 1.9/2.8/4.0/5.4 attempts
  and 0.70/0.46/0.98/0.45 takeovers per outage at 0.6/1/1.5/2.5 s (never abandoned: 1.0 attempts, 0.21/0.34/0.49/0.41).
- V14 SOURCE TLS resumption (Chromium main, fetched 2026-11-07 into scratchpad/v3h-tls): the session cache key is the
  server host:port plus destination IP, network anonymization key, privacy mode, proxy chain
  (ssl_client_session_cache.h:45-62); each entry keeps two single-use TLS 1.3 sessions, Push/Pop (:110-129;
  .cc:190-207). WebSocket connections take the same SSL connect path, so a reconnect to the same room host:port from
  the same page resumes if the server sent tickets: rustls servers send 2 TLS 1.3 tickets by default
  (ServerConfig.send_tls13_tickets; stateful, single-use, an in-memory store; my reading of rustls 0.23's docs, not
  re-fetched this round). Sizes: a Chromium ClientHello with X25519MLKEM768 (1,216 B share) + X25519, GREASE ECH and
  a PSK is about 1.85-1.9 KB (2 segments): 1,900 B fair. The resumed server flight with aws-lc-rs's default
  X25519MLKEM768: ServerHello with the 1,120 B share ~1,221 B as a record, CCS 6, EncryptedExtensions ~46,
  Finished ~58: ~1,330 B, one segment: 1,300 B fair (with ring: X25519, ~250 B; pessimistic by a fraction of a
  segment). Caveats: abort cadences pop a ticket at every attempt that reaches its ClientHello, so the two can run
  out (then a full handshake: the "full TLS" row, p99 1,054 -> 1,091, small); the room's rustls store must hold
  2-3 tickets per player (the default 256 entries fit one 48-player room). CONFIRMED. Sure: high on Chromium,
  medium on rustls defaults.
- V15 (the Windows-like option, fixes.rs:135-160, :534, :880): the client's SYN timer reaches only the client's SYN
  (tcp_model.rs:2059-2071, :2402-2413); the server's SYN-ACK keeps Linux's 1 s doubling (:2416-2423); `..o.cfg` keeps
  the server's write budget, minimum RTO and thin-stream settings; timestamps are negotiated, so off for both ends
  and a 1,460 B MSS both ways (tcp_model.rs:586, :2105, :2117) is faithful. FAITHFUL for the server end. Not modelled
  at the Windows client end (it runs Linux's rules): its own 300 ms minimum RTO and loss recovery (client to server:
  inputs, the ClientHello, Finished, the Join), whether it sends D-SACKs (without timestamps the room's spurious-RTO
  undo leans on them); its delayed ACK is 40 ms like Linux's (V5's output). Direction unclear, small.
- V16 (censoring, fixes.rs:1044-1063): a phase not recovered 60 s after the return counts as 60 s (a lower bound:
  conservative for every "none over Z" bar with Z <= 60 s) and is counted in CENSORED; the table prints 0
  (u3v3-tables1.txt, after ws_limit_table); the tail runs after that print, but its maxima (12,068 ms today, 11,384 fix
  1 alone) show no 60 s value, so 0 there too. Multi-burst bursts with no recovery before the next burst, or within
  12 s for the last (fixes.rs:1363-1369), count as over every bar ("no recovery 0" in every row). CONFIRMED.
- V17 (item 1, code): H6 (fixes.rs:704-732, called after switch :976-977; condition created <= newest_whole, the
  newest write time among whole snapshots on the current connection :668-674; the doc says "created", the task says
  "began": the code's creation time is the right one, it is what the page knows); after the 101 it runs to its Join
  (:653-658, cancel skips non-connecting :721; attempt_due returns None for a post-101 blocker :803-804): CONFIRMED.
  H7 (blocker :594-603 = a waiting attempt or any non-current live connection the page has not switched to or
  closed): CONFIRMED. H8 (outage reset on a whole snapshot on the current connection :672 and at a switch :692;
  checked :793; counted :837): CONFIRMED, per outage. H19 (last_data moves only when a whole message completes on the
  current connection :626-634, :668-671): CONFIRMED. Follow-on window (FOLLOW 3 s :54, :1014-1024; "still pending 3 s
  after the recovery 0" in every table row): CONFIRMED. Takeover counts (:741-771, before/after by meas :568-570,
  attributed to the latest burst's window :561-565): CONFIRMED. The task's per-burst figures (attempts 1.0, closed
  0.66, takeovers 0.16 + 0.18) are the 600,000-phase tail's side line: 1.000, 0.664, 0.1608 + 0.1757.
- V18 (item 4, accumulation): the Gate (throttler, Firefox records, endpoint locks) and the per-outage counter live
  in the Sim across bursts (fixes.rs:499-523); the current connection's TcpModel carries cwnd, ssthresh, srtt and Cubic
  state; a takeover starts a fresh connection with no per-destination metrics (tcp_model.rs:62-68 lists the metrics
  cache as not modelled; Linux 6.8 would only lift a new connection's first RTO from a cached RTT, ssthresh caching
  being off by default). Per-burst rows are flat in v3 (recommended #1 653/1,060 ... #10 664/996; #30 661/1,089;
  fix 1 alone #1 654/2,149, #10 664/2,176): the v2 cwnd ratchet is gone. Every burst of a lineage sits at the same
  tick and input phase (20 s = 600 ticks): within-lineage correlation (a stats point, as round 2 H20). CONFIRMED.
- V19 (item 8, L2): round 2's H17 and H18 stand unchanged by v3 (advisor-u3q.md:68-77 "above TLS, below tungstenite",
  :159 "the TLS handshake now runs undelayed above DelayStream", :195 PM-0001-18 "through the accept path's delay
  wrapper"; plan.md:355-356, :419 E15, :533-534 WsClient): in L2 a reconnect's SYN and TLS flights never meet the
  burst and LG's Rust client obeys no browser rule. New with this round: LG is a Linux client (SYN retry at 1 s), so
  even a faithful L2 measures the case V5/V6/V13 show is the favourable one for "never abandoned".
