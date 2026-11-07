# netcode-architect: the WebSocket reconnect record (ws-reconnect)

Task (workflow wf_f3167435-5b4, phase 1): draft `.checkpoints/netcode-proof-test/ws-reconnect-record-draft.md`, which
lands in the docs PR as `docs/design/ws-reconnect.md`. Mustafa's decision 2026-11-07 ~13:45 UTC (checkpoint
"Decisions taken"): WS-SCOPE no (fix 1 only in the proof test); the reconnect = a requirement for the real client,
built and measured before the first online playtest, target = today's model (buffer + reconnect: 99 % within about
1.05 s); RESUME-CRED deferred with it; WS-BAR three-part form now.

Rules: write only this file and the draft (Write tool); no tracked file edited; no commit; git from inside the folder,
never -C; file:line for every claim. Effort xhigh.

Process slip (recorded): one harmless shell command created and at once deleted an empty temp file next to this notes
file (`cat >> ...tmp </dev/null; rm -f`); no content written. Every save since uses the Write tool.

## Plan
1. Read sources: the checkpoint (done), fixes.rs (the harness: policy knobs, rows, bar evaluator, ws_bar_fix1),
   tcp_model.rs header (v3 name, SYN timer knob, TLS flights), the four skeptic/build notes (u3-verify2-harness,
   u3-verify3-harness, u3-verify3-stats, u3-build-v3-harness), u3-verify-fixes-adr.md (ADR conflict reading),
   .gate output files (u3v3-tables1.txt, u3v3-barfix1.txt), ADR 0001 (lines on tokens, reconnect, fallback,
   PM-0001-18), the docs/design/ folder's format, git log for bf9d248 / 709e44a / 1e020a7.
2. Record each finding below with file:line, as I go.
3. Verify numbers against the .gate outputs (the 600,000-phase rows: recommended, fix 1 alone, no-timestamp,
   Windows-like 1 s and 3 s, abort-500, join-or-close across 0.6/1.5/2.5 s).
4. Check the ADR 0001 line numbers myself (grep on the branch's docs/adr/0001-multiplayer-netcode.md).
5. Draft in sections (1)-(6) + header/status + sources; write each finished block to the draft at once.
6. Self-check: every claim cited; numbers match the outputs; plain words; lines <= 120. Then DONE.

## Findings (a line each, as read)
- F1 commits: 1e020a7 (10:50 UTC) = model v3 + harness v3; 709e44a = clean-ups, fixes.rs only +4 lines (prints the
  censored count, ws_limit_tail) "no number moves"; bf9d248 = ws_bar_fix1 + `budget_scale` knob default 1
  (fixes.rs diff: `opts()` sets budget_scale 1; the `fits` test multiplies by it). So the 600,000-phase numbers in
  `.gate/netcode-proof-test/u3v3-tables1.txt` (run 10:48, at 1e020a7) hold unchanged at bf9d248. The checkpoint says
  "fixes.rs at 709e44a" (netcode-proof-test.md:358), the task says bf9d248: both give the same rows; the record names
  bf9d248 (the newest, the one carrying the fix-1 bound) and says the rows are unchanged since 1e020a7. `git log
  bf9d248..HEAD -- tcp_model*` is empty: the files at HEAD (803c6c1) equal bf9d248's.
- F2 600,000-phase rows (u3v3-tables1.txt:120-131): today 1,121/2,686/4,068/6,484/max 12,068; fix 1 alone
  654/2,164/3,945/5,801/max 11,384 (52,406 over 1 s, 24,442 over 1.5 s, 15,226 over 2 s); fix 3 alone (64 KB +
  reconnect) 844/1,374/1,952/2,297/max 5,343; recommended (fix 1 + reconnect, never abandoned) p50 654, p99 1,051,
  p99.9 1,859, p99.99 2,092, max 4,196; over 1 s 6,768 (11.280 per mille [11.016-11.550]), over 1.5 s 1,009, over 2 s
  205, over 2.5 s 10, over 3 s 4. Attempts 1.000 a phase, takeovers 0.1608 before / 0.1757 after recovery, cancelled
  0.664 (u3v3-tables1.txt:131).
- F3 60,000-phase variant rows (u3v3-tables1.txt:82-117, ws_limit_table): recommended 654/1,054/1,827/2,066/3,013;
  Firefox identical (:98); full TLS (no resumption) 654/1,091/1,838/max 3,013 (:100); no timestamps (Windows peer)
  645/886/1,484/1,969/2,165 (:102); Windows-like 1 s SYN never abandoned 645/886/1,484/2,046/2,871 (:110); abort-500
  Windows 1 s 645/903/1,347/2,607 (:112); Windows-like 3 s SYN never abandoned 646/2,035/2,729/3,069/3,890 (:114);
  abort-500 Windows 3 s 645/903/1,347/max 2,607 (:116); abort-500 Chromium 654/1,142/1,376/1,813/2,544, attempts
  2.911 (:90); abort 1 s Chromium 654/1,071/1,746/2,227 (:92), Firefox 654/1,249/1,804/2,825, 4,601 over 1 s (:108);
  abort-500 Firefox 651/1,129/1,358/2,544 (:106); two endpoints 630/945/1,820/2,855 (:94); parallel (no browser runs
  it) 642/844/2,217/4,050 (:96); full state counting as recovery 653/1,022 (:104).
- F4 multi-burst (u3v3-tables1.txt:16-59): recommended 10 bursts x 2,000 p99 1,091 max 2,234 (:17), 30 x 1,000 p99
  1,096 max 2,258, throttled >= 1 ms 635, longest 421 ms (Chromium, :22-23); abort-500 Chromium 30 bursts: throttled
  10,208, longest 4,994 ms, capped 2, max 5,220 (:44-45); abort-500 Firefox 30 bursts no throttle, longest wait 399 ms
  (:55-56).
- F5 side effects, 30 min per profile, no bursts (u3v3-tables1.txt:61-80): adaptive budget skips 3/21/143/219 per mille
  at P1-P4 (:65-68); with the reconnect never abandoned: attempts 0/9/15/7, takeovers 0/2/1/1 (:69-72). (Round 2's
  numbers 0/0/11/18 were v2; v3 differs.)
- F6 fix-1 bound (u3v3-barfix1.txt, bf9d248 ws_bar_fix1): fix 1 correct 600,000 p50 654, p99 2,164, p99.9 3,945,
  p99.99 5,801, max 11,384; 480: "half within 0.67 s, at most 5 of 480 over 2.50 s, none over 6.90 s" 0.9681
  [0.9602-0.9754]; today 0.0000; 2x budget 0.0000; x1.05 0.001, +20 ms 0.159; 1,440: "half within 0.67 s, at most 11
  of 1440 over 2.50 s, none over 9.80 s" 0.9665. Censored 0.
- F7 u3-verify2-harness.md (round 2, v2 model numbers, still valid as mechanisms): H1 Chromium endpoint lock (lock taken
  before the TCP connect, keyed IPEndPoint + NAK, freed 10 ms after release: websocket_endpoint_lock_manager.cc
  :26, :28-45, :47-58, :142-159; transport_connect_sub_job.cc:117-131) (:24-36); H2 Chromium handshake 240 s, connect
  job 4 min; IPv4 sub-job 300 ms after IPv6 with its own lock (:37-41); H3 Chromium throttler
  websocket_throttler.cc:46-55 formula rand(1000..5000) x 2^min(p + f/(s+1), 16) / 65536 ms, counts roll each 2 min
  (:42-45); H4 Firefox FailDelay 200-400 ms after any failed stop incl. page close when no record exists, cleared by a
  successful open or 60 s; admission queue keyed by IP without port; open timeout 20 s (:46-55); H6 never-abandoned
  -> takeover after 99 % of bursts (v2; H6 fix = cancel-on-fresh in v3) (:61-67); H7 busy used room state (:68-71);
  H8 cap per run not per outage (:72-77); H9 echo RTT field "newest command received" ADR 0001:668 (:78-84); H10 TLS
  flights, rustls providers, resumption (:85-93); H11 SYN timer decides the gain (:94-100); H12 Firefox abort rows
  (:101-104); H13 takeover costs, graceful close, command overlap (:105-116); H14 FullState on takeover is a modelling
  choice (:117-123); H17 L2's DelayStream above TLS (:131-141); H18 LG is no browser, E23 per-address caps (:142-147);
  H19 partial messages (:148-151). Conflict list (:156-193) and "bound to the session" readings 1-3 (:195-207).
- F8 u3-verify3-harness.md (round 3, on v3 at 1e020a7): V1 Chromium throttler confirmed incl. IsClean erase after a
  2-min period with no success (:35-44); V2 MS-ABI argument order on Windows (:45-56); V3 Firefox FailDelay 200 +
  rand()%200, x1.5 growth capped 60 s, OnConnected clears, Close before data = NS_ERROR_NOT_CONNECTED, queue keyed by
  address w/o port, open timer 20 s (:57-64); V4 Firefox records also grow on an established channel's failure (reset,
  close timeout): requirement graceful close of a replaced connection (:65-73); V6 fix 1 alone on a no-timestamp client
  646/2,035/3,829/5,295/12,550 (60,000) and Windows 3 s/2 s SYN rows (:79-87); V8 throttle ratchet mechanism
  (:90-98); V9 never-abandoned worst tail = its own handshake in backoff (:99-104); V10 once a Join is sent let it
  land (:105-108); V11 join-or-close: removes the throttle ratchet, abort-500 30-burst max 5,220 -> 2,105, costs 0.55-
  0.67 extra handshakes per outage (:109-121); V12 fixed 500 ms abort fails at P4, scaled max(500 ms, 4 x RTT) rows
  P2/P3/P4 (:122-129); V13 burst-length sweep 0.6/1.5/2.5 s (:130-140); V14 TLS resumption: Chromium session cache
  key, 2 tickets per entry, rustls 0.23.46 sends 2 tickets, stores 256 (:141-154); V15 Windows option faithful at the
  server end, not at the client end (:155-161); V19 L2 changes; sockops BPF TIMEOUT_INIT, tcp_syn_linear_timeouts 0
  (:187-195); V20 current Windows 1 s, 4 retries (:196-206). Item 7 policy recommendation (:208-236); Item 8 ADR list
  (:238-265); L2 list (:267-281); verdict 15 never-abandoned REFUTED as across-lengths policy (:308-311).
- F9 u3-verify3-stats.md: F6 median part fragile (+7 % time scale -> 0.505) (:71-79); F7 search picks grid corner
  (:80-94); F8 subtle broken builds: trigger floor 600 ms passes 0.98 at 480 (:95-107); F10 one tick phase per run
  with 20 s spacing; spread bursts by a random 0-33 ms (:117-132, :241-243); F13 judge supports p50 / count-with-where
  / max on a params.transport "ws" part (:148-160); F15 mechanism checks instead of a median (:168-182).
- F10 u3-build-v3-harness.md: Chromium throttler argument order (:45-48), Firefox source lines (:49-54); the build
  numbers are on model v2 (superseded); cite only for mechanisms.
- F11 fixes.rs (bf9d248 = HEAD): header states fix 1 and fix 3's policy (:1-24); TLS flight sizes Resumed 1,900/1,300,
  Full 1,800/3,800 (:108-135); SNAP 1,050, INPUT 88, FINISHED_UPGRADE 474, SWITCHING 200, JOIN 140, RESUME 4,200
  (:33-42); UNLOCK_DELAY 10 ms (:43-45), THROTTLE_PERIOD 120 s (:46-47), FF_OPEN_TIMEOUT 20 s (:48-49), FF lifetime and
  max 60 s (:50-52), FOLLOW 3 s (:53-54); Opts incl. budget_scale (:63-76); Reconnect {min_gap, endpoints, cadence,
  abort_after, max_attempts} (:78-91); Browser (:93-106); ClientOs + client_cfg (:137-162); adaptive_limit (:164-169);
  trigger max(min_gap, 2 x app_rtt + 100 ms) (:171-174); echo RTT EWMA 1/8 (:666-669); outage reset on a whole snapshot
  (:670-676) and at a switch (:692-695); switch closes every other (:683-704); cancel H6 (:706-734); room tick: newest
  Join takes over, full state RESUME, no snapshot that tick (:736-790); attempt_due (:792-808); attempt incl. abort
  (:810-848); pump with Chromium/Firefox gates (:850-879); serial() 300 ms min_gap, 1 endpoint, cap 8 (:1187-1197);
  opts() defaults Chromium, Resumed, Linux, scale 1 (:1199-1210); configs() rows (:1212-1257); recommended() =
  configs()[3] (:1259-1263); ws_limit_table variants incl. Windows rows (:1276-1327).
- F12 ADR 0001 lines checked at HEAD (803c6c1): part 10 transport 419-474 (WebTransport first 1.5 s timeout :423;
  newest-only over WebSocket :424; one TCP port per room :429-432; Join within 2 s, handshake cap, per-address cap
  above 1, new sessions rate-limited :445-451); part 11 Join token claims :494-497 (expiry 30 s + 5 s leeway, unique
  id), replay cache :504; *Reconnect* :506-509 (grace 60 s, fresh token, newest connection wins, older closed); grace
  window 60 s = 1,800 ticks :479; the Worker owns the connection :616-617; tick :618-623; message table: Snapshot
  header :668 (newest command received, last processed command), Join "once, within 2 s" :670, JoinAccepted :671,
  FullState "at join and on resync" :672, Lobby calls :682; failure modes: B1 :747, no QUIC :748, A player drops :758,
  the lobby is down :759; anti-cheat admission :782-784; Q22 :959-964 (A chosen; B resume secret rejected :963);
  PM-0001-18 :1556 (500 ms plain limit, L1 + L2, B1, 10 bursts); spike item 2 "deltas again within 500 ms after B1"
  :1295; scope :1640-1641 (part 11's reconnect = slice 2 and slice 3 milestones); migration step 5 slice 3 online
  "lag compensation and reconnect ... Done when ... Mustafa's first online playtest is run" :1734-1738; test plan
  network simulation L2 "a delaying TCP proxy for WebSocket" :1756-1758; playtests from slice 3 :1784-1785. All the
  round-3 cited line numbers still hold at HEAD (PR #41 merged earlier; nothing since moved them).
- F13 Observation: the task calls the future builder "slice-2"; ADR 0001's migration plan puts reconnect in slice 3
  (step 5, :1734-1738), while the scope line :1640-1641 says "slice 2 and slice 3 milestones" for part 11 as a whole.
  The record says: the online client's slice (slice 3 in the migration plan), before the first online playtest; open
  point for the main session.
- F14 u3-verify-fixes-adr.md (round 1): RFC 6455 §4.1 quote (:83-88); Chromium lock keyed IP+port, UnlockEndpoint only
  in Upgrade() (:89-96); RFC 6455 §7.2.3 backoff SHOULD (:97-101); Firefox admission queue per address w/o port,
  FailDelay (:102-109); F14 parallel attempts not runnable in a browser (:110-118); F17 WebTransport-first on a
  reconnect: a WebSocket session must reconnect over WebSocket at once (:134-137); F18 catalogue rows (:138-143); F19
  30 s expiry -> refresh about every 25 s (:144-148); F20 no rule says when the fresh token is fetched (:149-152);
  WHATWG API: open/message/error/close only, no TCP events (:153-157); F22 spare in script at most 35 s (:161-163);
  ADR list (:189-210); A2 "bound to the session" (:237-240).
- F15 SOURCE RFC 6455 re-read by me (rfc-editor.org/rfc/rfc6455.html, WebFetch 2026-11-07): §4.1 item 2 "There MUST
  be no more than one connection in a CONNECTING state." and "If multiple connections to the same IP address are
  attempted simultaneously, the client MUST serialize them"; §7.2.3 "clients SHOULD use some form of backoff when
  trying to reconnect after abnormal closures", "The first reconnect attempt SHOULD be delayed by a random amount of
  time" (0-5 s suggested as reasonable). Confirms round 1's quotes. Sure: high.
- F16 u3-verify3-model.md: the p50 rise 435 -> 654 is Linux's undo path + PRR from an empty pipe (:99-123); attribution
  (NoUndo 509/828) (:124-131); max 4,196 = a three-loss tail draw incl. the reconnect's own SYN-ACK and ClientHello
  losses (:200-210); no model defect moves p99 1,051 or max 4,196 (:211-212); v6.12 rows identical (:160-169);
  residual: Windows client stack (delayed ACK 200 ms class, min RTO 300 ms class) can move the median by tens of ms
  (:213-216).
- F17 Durability: the .gate outputs are gitignored (`.gitignore:2: .gate/`), so u3v3-tables1.txt and u3v3-barfix1.txt
  vanish; the round-3 diagnostic patch for the scaled abort, join-or-close and burst lengths lives only in the session
  scratchpad (`scratchpad/v3h-diag/diag.patch`, 20,753 B, exists now); the checkpoint notes live in git history at:
  u3-verify2-harness.md 73f451d, u3-verify3-harness.md d9e0036, u3-verify3-stats.md and -model.md 88856ea,
  u3-build-v3-harness.md 59baae1, u3-verify-fixes-adr.md 10b1a53. The record must carry its numbers itself; open point:
  commit the two outputs under spikes/netcode-proof/results/ and add the policy options to fixes.rs.
- F18 main (origin/main 69ea88e) has the same ADR 0001 as HEAD (git diff --stat empty): line numbers hold on main.
  The docs PR's E4-type edit (1460-1469, a longer paragraph) may shift every later line; the record gives anchors
  (quoted words) beside each line number.
- F19 binomial arithmetic (python -I, exact sums): for a build whose share over 1.05 s is 1.0 % (the model's p99),
  the 95 % allowance for "at most k over 1.05 s": N = 480 -> k = 9 (P 0.976); N = 1,440 -> k = 21 (0.964); N = 4,800
  -> k = 60 (0.961). Fix 1 alone sits at 87.3 per mille over 1 s and fix 3 alone at 178.4 (u3v3-tables1.txt:124,
  :127): far above; subtle breaks (trigger floor 600 ms) are the hard ones (u3-verify3-stats.md:95-107).
- F20 Conflict to flag: the policy that is most robust across burst lengths (scaled abort + join-or-close) has p99
  1,126 ms at P2 1 s (V12, 20,000 phases, throwaway patch), about 75 ms above the target's 1,051 (never abandoned).
  Mustafa's target says "about 1.05 s"; whether 1.13 s is "about" is his call, or that work measures both on the full
  600,000 tail first. Recorded as a decision for that work.
- F21 Self-check corrections applied in the final draft: ADR 0001:1462-1463 says only "inside the server's accept
  path"; "above TLS, below the WebSocket library" is Mustafa's U3-Q4 (b) (checkpoint :400-401), cited so; "three
  rounds found it faithful" reworded (rounds 1-2 found missing kernel paths, v3 added them, round 3 confirmed);
  join-or-close's extra handshakes 0.55-0.67 at 1 s (:119-121) vs 0.55-0.77 across lengths (:228-231) stated both;
  the docs PR's 424 / 747 / 1295 / 1556 edits marked "planned" (the advisor writes them in parallel); Table 3/4 marked
  "sure: medium, one run, not pinned". Note: the saver has already committed an earlier draft (git status shows the
  draft modified, not new).

## Result (finished block)
- Draft: `.checkpoints/netcode-proof-test/ws-reconnect-record-draft.md`, 565 lines, sections: header; §0 in short;
  §1 requirement and target (Mustafa verbatim; model/harness/commits; Table 1 600,000 rows; Table 2 client variants
  incl. no-timestamp, Windows 1 s and 3 s, same-client fix-1 baseline; Mustafa's laptop); §2 why fix 1 falls short and
  what the reconnect does, plus the proof test's fix-1 bound; §3 policy (page API, serialization RFC/Chromium/Firefox,
  throttles, trigger and header field, attempts and cap per outage, WebSocket-only reconnect, §7.2.3, cancel vs
  join-or-close, never abandoned vs scaled abort with Tables 3-4 across P2-P4 and 0.6/1/1.5/2.5 s, takeovers and their
  costs, room side incl. graceful close, admission limits, TLS resumption, SYN timers); §4 RESUME-CRED (deferred;
  readings 1-3; refresh, use, where, lobby down, fetch-during); §5 ADR 0001 line table with anchors (24 rows); §6
  measurement (why L2 can't; 8 setup needs; per-burst records; the bar incl. binomial sample sizes; harness to reuse
  with commands and git-show fallback); §7 sources with commits; §8 open questions.
- Open points for the main session: (1) F13 slice 2 vs slice 3 wording; (2) F17 durability: the .gate outputs and
  the round-3 diag patch are not in git (commit the outputs under results/, add scaled-abort/join-or-close options to
  fixes.rs or keep the patch before the session ends); (3) F20 the robust policy may miss "about 1.05 s" by ~75 ms
  (Mustafa's call in that work); (4) whether ADR 0001 gets a one-line pointer to this record in the docs PR (the
  advisor's words); (5) §6.4's bar form borrows WS-BAR's rule by analogy: Mustafa to confirm for the reconnect;
  (6) the record cites `.checkpoints/` paths that leave the tree; each is pinned to a commit in §7.

DONE
