# U4 build plan rev 2: netcode-architect notes

Task: read both critics (u4-plan-review-test.md, u4-plan-review-arch.md) in full, check each finding against the
code, rewrite u4-build-plan.md as rev 2 with a §0.1 table (finding -> where rev 2 meets it / deferral / why wrong),
keep and update the advisor questions. No code edits, no builds, no runs. Notes by the Write tool only (checkpoint
:294-295); my first append went in by a shell heredoc before I re-read that rule; this rewrite carries it unchanged.

## Plan
1. Read both reviews in full; list every finding id here.
2. Read plan rev 1 in full (645 lines) and its notes.
3. Check each finding against code / marks.json / README / SERIES / ADRs; a line per finding here.
4. Draft rev 2 text: changes per section; §0.1 table.
5. Write rev 2 to u4-build-plan.md (Write tool), DONE line here.

## Sources read (summary per source, as read)
- S-RFC9002 (www.rfc-editor.org/rfc/rfc9002.html, fetched 2026-11-09, first 100,000 of 105,234 chars): §6.2.2 "When no
  previous RTT is available, the initial RTT SHOULD be set to 333 milliseconds"; §6.1.2 time threshold
  "max(kTimeThreshold * max(smoothed_rtt, latest_rtt), kGranularity)", kGranularity RECOMMENDED 1 ms; §6.2.1 "PTO =
  smoothed_rtt + max(4*rttvar, kGranularity) + max_ack_delay", restart "every time an ack-eliciting packet is sent or
  acknowledged", backoff doubles; §5.3 before a sample smoothed_rtt = kInitialRtt, rttvar = kInitialRtt / 2; the first
  sample sets smoothed_rtt = latest_rtt, rttvar = latest_rtt / 2. Sure: high (spec text). Initial PTO (arithmetic):
  333 + max(666, 1) + 25 = 1,024 ms.

## Findings log
- step 1 done: both critics' notes read (test T1-T23 one-liners + T1-T4 in the task text; arch A1-A26, N1-N5
  one-liners + A1-A4 in the task text). Step 2 done: plan rev 1 read in full (646 lines) and its notes M1-M25.
- T1 CONFIRMED: marks.json PM-0001-2 checks `snapshot_clients_count` min >= 48 and `soldiers_count` min >= 48 on
  part "SS runs" {scene SS, clients 48}; README:193-200 pre rule; eval.rs:850-887 Rule::Pre applies the matched part's
  checks. -22's checks: headroom min == 0, event.ack_starved.dropped count >= 1 (marks.json -22). A kick drops
  a client -> 47 -> INVALID under pre. Fix: -22 gets no pre (its select is suite never-ack, no scene).
- T2 CONFIRMED: -30 part C select {suite both-transports, profile P0, machine.role C}, no layer (marks.json -30);
  matched_parts eval.rs:921-930 ignores only role. README:32 layer values L1, L2, real.
- T3 CONFIRMED: SERIES.md:266 compares (client, tick) pairs; a split tick with one group lost has no defined client
  hash. Fix chosen: whole-tick hash only on complete ticks; a received-but-refused group logs a sentinel.
- T4 CONFIRMED: -17 checks = figure ==0 + S30 setup (marks.json -17); notes say "fails, never passes, on 0 compared".
  Fix: client_hash covers the rebuilt 32-bit tick; refusal sentinel; client hashes its HELD presentation state
  re-quantised through the grid (so client-rounding has a real path), exhaustive test grid(deGrid(q)) == q.
- T5 CONFIRMED: ADR 0004:1142 PM-0004-21 "the HashMap holding at least 32 keys whose order decides an outcome, with its
  two orders logged"; PM-0004-18 (same block) "each process logs its std HashMap seed and the two differ".
- T6 CONFIRMED in part: -21's notes already pin not-older-than to fast-clock option A (marks.json -21 notes); -62 needs
  "replays forced as in PM-0001-44" (ADR 0001:1650) but rev 1's Predictor skips matching replays (plan :444-446);
  -48 no-buffer-wait bites only "with a buffer of 2 ticks or more" (marks.json -48 bite text); -45 "one slot too short
  for P2" needs P2's measured need, not 7-1; -41 server-speed-1pct: ADR 0001:1636 -> movement-speed reading; -41
  server-quant-differs needs ADR 0001:207-208's second option (the server snaps its own copy); -58 lt-ack and
  lt-event-seq have no failing check unless their sites are named.
- T7 CONFIRMED: voided datagram loses its ack; snapshot sizes then differ -> group counts differ -> server->client
  packet indexes shift -> fates shift. Fix: twin repeats the previous ack AND twin-compared suites run open-loop.
- T8 CONFIRMED: ADR 0001:238-239 catch-up cap "no larger than the redundancy cap" = 8.
- T9 CONFIRMED: ADR 0002:500-505 "built as the server builds the sim".
- T10 CONFIRMED: l1/src/lib.rs:856-868 `with_bite(seed, bite)` per instance.
- T11-T23 CONFIRMED (T21 partly: 46 + PM-0004-67's 3 = 49 holds; runs ~270, not ~230).
- A1-A19, A25, A26 CONFIRMED (A3: problem agreed, fix differs: clamp inside sim's shot phase). N1-N4 CONFIRMED;
  N5 DEFER.
- Shot-band arithmetic for T1 (mine): 700/s / (48 x 30) = 0.486 shots per command-tick. B1 under option A: each 1 s
  burst loses ~22 of 30 ticks of commands (8 recovered by the redundancy cap) -> ~0.73 s of shots lost per burst; 10
  bursts in 600 s -> mean ~ 700 x (1 - 7.3/600) = 691.5 >= 665. Forged: 11 forgers x 1,000 refused commands x 0.486
  = ~5,350 shots / 600 s = ~8.9/s -> ~691. One client fully silent: 700 x 47/48 = 685.4.
- Recording size (mine): 41 B per command (slot 1, tick 4, seq 2, buttons 2, presses 4, move 4, aim 12, view tick 4,
  frac 1, rewind flag 1, tick_a 4, frac 1, cover 1) x 48 + 5 = ~1,973 B/tick -> ~17.8 MB per 9,000 ticks.
- Design decisions taken for rev 2: new U4s (seam types + grid + manifests, first, alone); U4j (judge, marks, log)
  apart; U4e split into U4e (room + codec path) and U4r (runner); runs separate from builds; grid in npt-sim, proto ->
  sim; HitHistory type in sim, room owns instance; clamp in sim's shot phase; server snaps its own copy (Q23);
  -41 movement reading (Q24); S1 burn/top-up off on L1 (Q26); AddrTable + handshake cap to U6.
- Step 3 done. Step 4: rev 2 being written whole with the Write tool.
