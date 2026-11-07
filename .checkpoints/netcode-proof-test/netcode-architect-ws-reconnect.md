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

---

# Revision 2 (2026-11-07, after Mustafa's 14:30 refinement and the advisor's revision 2)

Task (workflow, computed): align the record with the advisor's revised docs-PR words (advisor revision 2: E-PR, E0-E13,
rulings R1-R21; its E11 lists the record's required contents (1)-(11)); fix the verifier's must-correct item 5
(`verify-ws-docs.md` V20: §5 must mark the ADR rows this docs PR itself changes, plus a row for OQ-0001-19); fix §6.1's
cite (V21) and §1.1's fix-1 sentence (V25, V17); cite the committed outputs under
`spikes/netcode-proof/results/u3-ws-fallback/` (975e72b), not `.gate/`; keep "slice 3"; keep every number tied to
its source. Branch claude/netcode-proof-test-plan-hcky5g, head 95a04a1. Same rules as before (Write tool only; no
tracked file; no commit; git from inside the folder, never -C).

## Plan (revision 2)
1. Re-read: the record draft, the verifier's V1-V31, the checkpoint's "Decisions taken" at 95a04a1 (14:30 and 13:45
   words), the committed outputs (README, tables, bar) and their line numbers; confirm they equal the .gate copies.
2. Re-pin every `.checkpoints/` cite to one commit (95a04a1) and check each there; check the ADR 0001 lines the docs
   PR changes (424, 506-509, 747, 1295, 1462-1469, 1556, 1571, 1697-1699, 1734-1738, OQ-0001-18's end) on the branch.
3. Check the fixes.rs cites the advisor's words rely on (ws_bar_fix1 :2000-2011; :1259-1263; :776-777; :786-787;
   three_part :1528-1537) and the round-3 lines the record cites for the robust policy.
4. Revise the draft in place, block by block: header (pins, status); §0; §1.1 (14:30 words, fix-1 sentence); §1.2;
   §2.3 (point to PM-0001-18, quote only the first bound, the tolerance); §3.7 (robust policy p99, the patch path);
   §5 (rows this PR changes, OQ-0001-19 row, "422-423" -> 423, line numbers as before this PR); §6.1 (cite); §6.4
   (WS-RECONNECT-BAR, proposed, 14:30 tolerance if borrowed); §6.5 and §7 (results paths, the patch); §8.
5. Self-check: every number against its source line; every cite resolves at its pinned commit; lines <= 120. DONE.

## Findings (revision 2, a line each, as found)
- W1 The checkpoint's working copy equals 95a04a1 (`git diff --quiet 95a04a1 -- .checkpoints/netcode-proof-test.md`).
  Lines at 95a04a1: the 14:30 refinement :349-356 (his words :349-353, the session's gloss :353-356); the 13:45 words
  :357-362; "Facts from Mustafa" heading :336, his laptop :337-346; the 2026-11-06 23:40 answer :370-371; U3-Q1..Q9
  agreement :391-392, the Q4 (b) reading "the wrapper above TLS, below tungstenite" :400-401; P2's definition
  :417-418; "the 30 stale snapshots drain from cwnd 1" :234-235 (S13 (e), U3a's run).
- W2 The committed outputs equal the .gate copies byte for byte (`cmp`): `tables-1e020a7.txt` = u3v3-tables1.txt,
  `bar-fix1-bf9d248.txt` = u3v3-barfix1.txt; so every old `u3v3-tables1.txt:N` cite maps to `tables-1e020a7.txt:N`
  unchanged (rows re-read: :5-6, :16-23, :44-45, :55-56, :61-72, :82-117, :120-131, :154). The bar file's lines:
  :7 fix 1 correct, :8 today, :9 budget x2 (p50 776), :11 the Y = 2.5 s bar at 480 (0.9681, bootstrap 0.9602-0.9754;
  x1.05 0.001, +20 ms 0.159), :14 the 1,440 bar (0.9665), :16 censored 0. The README (results/u3-ws-fallback/
  README.md:9-13) gives the reproduce commands, with `cargo +1.99.1`.
- W3 The patch `reconnect-policies-1e020a7.patch` (440 lines) holds the round-3 diagnostic tests: diag_rows (:179, V6),
  diag_order (:220), diag_nocancel (:239, V11), diag_lengths (:274, V13), diag_profiles (:311, V12), diag_ratchet
  (:359), diag_worst (:392, V10); it applies to fixes.rs at 1e020a7 (README.md:13). It replaces the scratchpad path
  (F17), which is no longer needed as a source.
- W4 Every `.checkpoints/netcode-proof-test/` note the record cites is unchanged since its last commit (10b1a53,
  73f451d, d9e0036, 88856ea, 59baae1; plan.md 709e44a), so each equals its 95a04a1 copy: one pin, 95a04a1, serves
  every `.checkpoints/` cite (E11 (9)); §7 still lists each file's last commit for `git show`.
- W5 plan.md's resume-secret line is :1024-1025 at 95a04a1 (§10.3 "Join tokens and the roster" starts :1016), not
  :1022-1023 (the draft's 803c6c1 cite; plan.md changed at 709e44a only, so 803c6c1's numbering differed or the cite
  slipped). Corrected to :1024-1025.
- W6 fixes.rs unchanged since bf9d248 (`git log -1`); tcp_model.rs last changed at 709e44a (the overflow guard, V15),
  equal at bf9d248 and HEAD. Fix 1 in the model: adaptive_limit :164-169 (doc :164, 100 ms before the first sample
  :166, N at :167, floor 3 at :168); the write rule :776-777 (fits, plus nothing owed and the 64 KB budget's room);
  the model keeps no snapshot queue: a tick's snapshot that does not fit is never written (:777-789), and the room's
  skips are counted (P2 22 per mille with the reconnect, 21 without: tables :66, :70). So "older unsent snapshots are
  dropped" (draft §1.1) was wrong: nothing written is dropped; the newest that does not fit is skipped (V17, V25).
- W7 The kernel facts for §1.1 come from the verifier's check of Linux v6.8 (verify-ws-docs.md:27, V12): SIOCOUTQ
  answers write_seq - snd_una (net/ipv4/tcp.c:630), tcpi_rtt = srtt_us >> 3 (:3842); the model's readings are
  tcp_model.rs:2167-2177 (srtt_ns "what TCP_INFO's tcpi_rtt reports"; outstanding = write_seq - snd_una "SIOCOUTQ").
  I did not re-read the kernel myself (sure: high on the verifier's cite).
- W8 ws_bar_fix1 is fixes.rs:2000-2011 (doc :2000-2008, fn :2011); the draft's :2004-2011 corrected (V25) in §2.3
  and §6.5. three_part() :1528-1537 (V14: doc :1528-1529, fn :1530).
- W9 ADR 0001 on the branch equals origin/main (`git diff --stat origin/main` empty for the file). The docs PR's edits
  hit, before it lands: 424 (E0), 506-509 (E5, one sentence before "*Private rooms:*"), 1250-1252 (E6 anchors
  OQ-0001-19 between OQ-0001-18's end :1250 and "## What a spike must prove" :1252), 747 (E2), 1295 (E3), 1462-1469
  (E4), 1556 (E1), 1571 (E8), 1697-1699 (E13), 1737-1738 (E7); and docs/adr/README.md's register rows (E9, E10). The
  §5 row "422-423" is 423 alone (V25: 422 is "axum's `ws`").
- W10 Rows the docs PR does NOT change, which the record's §5 keeps for the reconnect work: 423, 429-432, 445-451,
  494-497, 503-504, the rest of 506-509 (the rule itself: "the client fetches a fresh token; the newest connection for
  a player id wins"), 529-530, 616-617, 668, 670-672, 682, 758-759, new failure rows, 782-784, 959-964, 1640-1641,
  1756-1758. And three rows this PR changes that the reconnect work changes again later: 747 and 1556 (the reconnect's
  own bar, if the reconnect work adds one) and 1737-1738 (Done-when, if its target or bar wording moves).
- W11 Consistency of the target's words: the advisor's E5/E6/E7 say "within about 1.05 s of a 1 s outage at P2"; the
  record measures from the link's return (fixes.rs:19-21; tables :129 is "first fresh delta after the return"). The
  record says "of the link's return after a 1 s outage"; open point to the advisor: E5/E6/E7 could say "after the link
  comes back" to match PM-0001-18's own clock (E1: "the time from the link coming back"). Not my words to change.
- W12 §2.3 must point to PM-0001-18's row (E1) and quote only the first bound (E11 (7)); the 14:30 words are quoted
  in §1.1 because §1.1 describes WS-BAR and §6.4 discusses borrowing it (E11 (1)). The tolerance's mechanics (s = M_p
  / M_m, f, the band, the guard) are the advisor's readings in the PR (rulings R4-R8): the record names them by
  reference only, so the two cannot drift.
- W13 §6.4: the advisor escalates WS-RECONNECT-BAR with recommendation (b), leave open (escalate_to_mustafa, second
  entry). The record keeps the borrowed form as "proposed, Mustafa to confirm (WS-RECONNECT-BAR)", presents (a) and
  (b), and says that if the form is borrowed, the 14:30 tolerance comes with it, and why a 10 % tolerance's statistics
  (median-based) need re-deriving for a 99 % tail target (the median of the reconnect target barely moves: 654 for both
  fix 1 alone and the target, tables :123, :129; the reconnect changes only the tail, so a slowdown measured at the
  median says little about the reconnect's tail). That is a new point for that work, recorded in §6.4 and §8.
- W14 E6's "Decider" names Mustafa for the credential and the target if the policy misses; the record's §4.2 "Who
  decides" said "Mustafa, in the reconnect work, with security-auditor's review"; aligned to E6: Mustafa (the
  credential, the target if missed), netcode-architect with security-auditor (the design), the Advisor confirming.

## Revision 2, resumed (2026-11-07 18:34 UTC, after the usage limit)
State found: the draft already carries revision 2's blocks (header pins, §0, §1.1 with both sets of words and the
fix-1 sentence, §2.3, §3.7, §5.1/§5.2, §6.1, §6.4, §6.5, §7, §8; committed by the autosave at d06639a/6a1a505); this
notes block had findings W1-W14 but no result and no DONE. Working copies of both files equal HEAD 6a1a505.

### Plan (resumed)
R1. Check the draft against the advisor's final revision-2 words (the computed task's E-PR, E0-E13, R1-R21) item by
    item, E11 (1)-(11) above all; list what is still missing.
R2. Self-check every cite still unchecked in this revision: the `.checkpoints/` lines at 95a04a1; the committed
    outputs' lines (tables, bar, README, patch); fixes.rs and tcp_model.rs lines at bf9d248; ADR 0001 lines in §5.
R3. Fix what R1-R2 find, in place, with the Write tool (one whole-file write of the draft).
R4. Line lengths <= 120 outside tables; then the result block and DONE here.

### Findings (resumed, a line each)
- X1 Pins at 95a04a1 re-read (`git show 95a04a1:...`): checkpoint :234-235, :337-346, :349-353 (gloss :353-356),
  :357-362, :370-371, :391-392, :400-401, :417-418 all hold; verify-ws-docs.md :27 (V12), :38 (V23), :39 (V24) hold
  (62c3440, which added it, is an ancestor of 95a04a1); plan.md :1016 and :1024-1025 hold. Each cited note's last
  commit (10b1a53, 73f451d, d9e0036, 88856ea, 88856ea, 59baae1, plan.md 709e44a) equals §7's list and HEAD's copy.
- X2 Committed outputs (975e72b, `git log -1 -- results/u3-ws-fallback`): README.md's table is :10-14, the reproduce
  commands :12 (tables), :13 (bar), :14 (patch). W2's "README.md:9-13" and W3's "(README.md:13)" were one line off;
  the draft already cites :10-14, :12, :13 and :14 correctly. tables :22-23, :44-45, :66, :69-72, :88, :92, :96, :98,
  :100, :102, :104, :108, :110, :112, :114, :116, :120-131, :154 re-read: every number in Tables 1-2 and §2-§3 holds.
  bar :7-16 hold (x1.05 0.001 and +20 ms 0.159 are the end of :11).
- X3 fixes.rs at bf9d248 (= HEAD): :1-24, :30-42, :63-91, :164-169, :171-174, :194, :501, :584-605, :653-658,
  :666-676, :684-700, :706-734, :736-790, :792-808, :1187-1263, :1276-1277, :1329-1333, :1528-1530, :1552, :1615,
  :1626, :1682-1683, :1833-1858, :2000-2011 hold. Two cites slightly off: multi_burst_table is :1375-1381 (doc :1375,
  fn :1379), the draft said :1374-1380 (§6.2 item 7, §6.5); BurstOut is :463-476, the draft said :463-479 (§6.3).
- X4 tcp_model.rs: :1-10, :40-41 (S6, the tail-loss probe), :79 (MODEL), :210-213, :230-235, :2167-2177 hold;
  SYN_LINEAR_TIMEOUTS is :106 with its doc at :105 (the draft cited :105 alone; widened to :105-106).
- X5 Round-3 notes re-read at the cited lines: u3-verify3-harness.md :51-53 (throttle 1,059 vs 421 ms by argument
  order), :79-81, :119-129, :177-179, :180-186, :187-195, :228-231, :242-248, :268-281, :286-290;
  u3-verify3-model.md :158-169, :193-196, :200-216, :236-242; u3-verify3-stats.md :30-31, :133-147, :148-160,
  :174-182, :241-243. All hold.
- X6 ADR 0001 at 95a04a1 equals origin/main (`git diff --stat origin/main` empty). §5.1's anchors re-read: 424,
  506-509, 747, 1249-1252, 1295, 1461-1469, 1556, 1571, 1673-1674, 1697-1700, 1734-1738, 1640-1641. All hold.
- X7 GAP (E11 (4), "the 2 s Join deadline per attempt"): the record did not name it. ADR 0001:445-446: "`Join` within
  2 s, counted from the TCP accept or the QUIC handshake's first packet, so TLS and the WebSocket upgrade fall inside
  it". The harness does not model it (no deadline in fixes.rs: grep). The 600,000-phase maximum's trace: the
  reconnect's SYN 3 completes at +1,254 ms, its first data's RTO is ~2.4 s from a ~1.1 s SYN-ACK sample, its
  ClientHello is lost and resent at +3,754, takeover +4,111 (u3-verify3-model.md:205-208): about 2.5 s from the
  connect to the Join, past the 2 s rule, so a room enforcing it would have closed that attempt. My reasoning, not a
  run: the rule binds only when a handshake retry inflates the new connection's first RTO and a TLS or upgrade
  segment is then lost (two 2 % losses on one takeover), so p99 is untouched and p99.9 and the maximum may move either
  way. Added to §3.9 (fact plus the choice) and §8. Sure: medium.
- X8 GAP (E11 (8), the trace): the record had no trace line. Added to the header: ADR 0001 part 10's need,
  "VISION_V2 §6.1; §4 (four browsers)" (0001:473); VISION_V2.md:52 (Chrome, Edge, Firefox and Safari), :55 (fair up
  to about 100 ms of ping), :63 (§6.1, modes as data with rooms and private matches).
- X9 GAP (E11 (3)): §1.2's policy bullet lacked the 300 ms spacing of attempt starts (serial()'s cadence,
  fixes.rs:1189-1196, the row :1236-1239) and the cancel and resume lines (:706-734, :736-790). Added.
- X10 GAP (E11 (9)): the header named the branch for the outputs, not for the code; added the branch to the code
  line, and a "model and harness" entry to §7.
- X11 GAP (E6's "each browser engine"): §6.2 item 2 named Chromium-based browsers and Firefox only; VISION_V2 §4
  names Safari too (WebKit, §8 item 5: not researched). Item 2 now names each engine §4's browsers run on.
- X12 §6.4's "cannot show 99 % to better than about 2 %" tied to its number: 9 of 480 is 1.9 %.
- X13 E11 (1)-(11) otherwise met by the draft as found: both sets of words verbatim with pins (§1.1); the target row
  and Windows-like rows (§1.2, Table 2); parallel reference only (§3.2); RESUME-CRED readings (§4.1); measuring below
  TCP (§6.1-§6.2); slice 3 and a miss to Mustafa (§0, §1.1, §5.1, §6.4); §2.3 points to PM-0001-18's row and quotes
  only the first bound and its pass chances; sources pinned (§7); §5.1 marks every row this PR changes (424, 506-509,
  OQ-0001-19 at 1250-1252, 747, 1295, 1462-1469, 1556, 1571, 1697-1699, 1737-1738, the register) and §5.2 the later
  ones, 423 alone; §1.1's fix-1 sentence; §6.1's pinned cite; ws_bar_fix1 :2000-2011. No `.gate/`, `u3v3-`,
  scratchpad or 803c6c1 cite is left (grep).
