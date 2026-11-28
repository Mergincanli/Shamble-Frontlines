# The moved-mark block for Mustafa's game chat (P6 item 2; his item 7 of 2026-11-22, history.md:7606)
Built from marks.json (titles, parts and checks read by script: scratch markrows.mjs) and replan-marks.md section 1
(destinations), with MARK-NOTES item (2) quoted from replan.md:107-111. Build-plan lines checked on main f5030a3.

```text
MOVED MARKS: from the netcode proof test (PR #40) to the real game.
For the Advisor, before change-set 3's four questions. Mustafa's decisions: NPT-TRIM (21 November 2026) and his
answers to the re-plan (22 November 2026).

WHY. The proof test now answers only what the real game cannot answer cheaply before Slice 3: does the server carry
48 players at 30 ticks a second within budget on the planned machine, and do WebTransport and the WebSocket fallback
hold up from real browsers over real networks. The marks below moved to the real game. Nothing is dropped or
loosened: each is measured on the real game at the first build that can show it (Mustafa's rule of 14 November),
against its bar exactly as written.

WHERE THE BARS ARE. Each mark's bar is its row in the ADR on main: PM-0001-* in docs/adr/0001-multiplayer-netcode.md,
PM-0006-* in docs/adr/0006-infrastructure-operations.md. The proof test's judge holds the same bars, unchanged, in
spikes/netcode-proof/marks/marks.json on PR #40's branch (last changed at 15102d1; sha256 c1935a5b...), and reports
every moved mark or part as MOVED, never PASS. Line numbers below are in docs/reports/2026-11-17-build-plan-slice-1.md
on main (f5030a3). Machines: ML = the typical student laptop Mustafa names for the whole game (OQ-0003-1); RD = the
named desktop; MAC = a Mac for Safari; MP = Mustafa's own PC. "The human run" = the proof test's R7, moved whole to
Slice 3.

FOR CHANGE-SET 3 ITSELF (three items; nothing else lands in it):
1. PM-0001-34 "No decoder panics or over-allocates" starts with change-set 3's codec and grows per message type as the
   real protocol adds them (10 million random and mutated inputs; 0 panics; 0 allocations above the size limits).
2. OQ-0001-6 (commands run on arrival, or buffered one per tick): the proof test no longer picks it, because its
   inputs moved. Slice 3 picks it; change-set 7 builds both ways (:389-391). On Mustafa's go, the build plan's
   OQ-0001-6 line (:909) and Part 6 change to say so; today :390 still says the proof test chooses. Until Slice 3,
   the proof test's server runs use the draft's buffered way, and its real-network command run tests both.
3. Change-set 3 builds its own L1 link simulator (:321-323). The proof test's L1 (seeded delay, jitter, loss and
   reordering, with a packet-fate log; PM-0001-3's L1 half passes on it) is offered to that plan rather than built
   twice. It has no rate limit and no duplication, which :322 lists. Taking it is that plan's choice.

MOVED WHOLE (29 marks: -34 above, and these 28), by destination:
- Change-set 6, Look (:371-384): PM-0001-32 One client build runs in all four browsers on both backends (both
  backends measured in 6, :381-382; the browsers online at Slice 3; Safari later).
- Change-set 7, Lag (:386-397): PM-0001-41 Prediction is exact when commands arrive in time (:395) · -42 Corrections
  stay small in a crowd (48 scripted clients, :392) · -60 A server-side change to own movement or fire timing is
  predicted from the next snapshot on (movement here; fire timing in 8, :404) · -43 Own movement and own shots feel
  the same at any latency (movement; shots in 8; its RD part when RD is named) · -44 The replay fits on the minimum
  laptop (:395-396) · -45 Remote soldiers are drawn without guessing (the interpolation buffer, :388) · -46 The delay
  adapts, and respawns never slide (the delay here; respawns in 8, :403) · -47 The client's clock is right (the clock
  estimate, :388; its L1 pass in the proof test, a4cac39, stays as a record).
- Change-set 8, Shoot (:399-413): -61 Remote events play with the poses they belong to (:403) · -51 Aiming at height
  hits the clicked point (hitscan, :402; its lag-compensated parts at Slice 3) · -52 Hit feedback waits for the
  server (the hit marker on confirmation, :405; the browsers at Slice 3) · -62 Own shots play their cues once,
  however often they are replayed (own weapon state predicted, :404).
- Change-set 9, Throw (:415-419): -54 Client-side projectile flight holds, or E3b carries the kinds that still desync
  (the one shared trajectory function, :416).
- Slice 3 (transports, the WebTransport crate choice, lag compensation: :895, :899): -33 The client's RTT agrees with
  the server's (needs WebTransport's rtt()) · -35 The transport stays swappable and current (its swap needs the
  second transport) · -48 Hits register as the shooter saw them · -49 The rewind cap holds; what it clips is known
  · -50 A cheat cannot buy rewind · -53 Mustafa's verdict: prediction, reconciliation and lag-compensated hits look
  correct at P2 (the human run).
- Slice 6, the real sim (docs/SPIKE_PLAN.md:633-635): -39 The 60 Hz margin · -40 The research's other 60 Hz marks
  · -63 The goal budget holds at the rooms per VM measured at 60 Hz (all three were already outside the proof test).
- The security baseline before the first online playtest (docs/SPIKE_PLAN.md:631): -25 Hostile input is contained on
  both transports · PM-0006-15 The firewall lets both game transports in and nothing else · PM-0006-26 The
  certificate rotates without dropping anyone · PM-0006-27 A certificate that breaks the browsers' rule is refused.
- The first slice that adds vehicles (not cut yet; docs/SPIKE_PLAN.md:629): -55 A soldier beside a moving vehicle is
  not corrected much.

KEPT IN THE PROOF TEST, WITH PARTS THAT MOVE (only the named parts move):
- PM-0001-1 The rig is named: part RD goes with -43's RD part (when RD is named); part MAC: Safari later.
- PM-0001-3 The network profiles are what they say: part "MP at P2" goes to Slice 3 with the human run.
- PM-0001-14 No datagram is ever too big: part "MAC Safari": Safari later.
- PM-0001-19 A client joining mid-match has the whole world fast: part "ML Chrome" goes to Slice 3 with the human run.
- PM-0001-28 Each WebTransport candidate works in every browser: the wtransport parts go to Slice 3's crate choice
  (:895), since the proof test carries one crate, web-transport-quinn; the MAC Safari parts: Safari later.
- PM-0001-29 The fallback takes over when QUIC is blocked, and PM-0006-25 The development certificate works in every
  browser: their Safari parts: Safari later.
- PM-0001-31 The fallback's stalls stay short: part "ML Chrome over WebSocket at P2" goes to Slice 3 with the human run.
- PM-0001-36 Datagram size is measured per browser: the Safari parts: Safari later.
- PM-0001-56 The proof test's records are complete: the MARK-NOTES text Mustafa approved on 22 November: "-56's record
  counts 4 items, not 10: items 5-10 move with their marks; item 1 keeps only the carried crate's results on -11 and
  -28 (the other crate, -25, -35 and the pick move to Slice 3); item 2 keeps each browser's draft and its fallback
  (with RE-PLAN-OK (1)), Safari later; item 4 keeps NewReno against Cubic, while the browsers' rates at P2 to P4 move
  with the human run." (RE-PLAN-OK (1): the QUIC-blocked fallback and the hash-pinned certificate stay in the proof
  test's first run.)
- PM-0001-58 Every counter survives its wrap: its check "0 corrections larger than 1 mm not traced to a late or lost
  command (PM-0001-41's count)" and its planted bug lt-tick-offset read the client's prediction, so they move with -41
  to change-set 7.
- PM-0006-14 Rooms per VM counted: parts N=36 and N=18 go to slice 6, on the real sim.

ALSO TO THE SECURITY BASELINE (work cut from the proof test's server, no marks of its own): the flood and abuse
clients, slow-loris deadlines and header caps, the address table and /64 cycler, Origin checks, socket-minted tokens
and capabilities, per-session controls, per-slot systemd units, the second WebTransport crate, the certificate's live
swap and revocation.

NOT MOVED, BUT TIED TO A GAME-WIDE CHOICE: seven kept marks' laptop parts (-1, -14, -28, -29, -36, PM-0006-16,
PM-0006-25) are judged only on ML. The proof test's R8 (a short later run) judges them once Mustafa names ML
(OQ-0003-1); until then they read NOT-RUN.

FOR REFERENCE, NOT MERGED (on PR #40's branch, in .checkpoints/netcode-proof-test/): the proof test's built half of
prediction (wip/u4g-early.patch, 2,053 lines); the briefs for prediction, interpolation and events (u4g-brief.md) and
for lag compensation (u4h-brief.md); the decoder fuzzing begun for -34 (wip/u5-early.patch, 1,658 lines).

SOURCES: the full 81-mark table is replan-marks.md section 1, and the re-plan is replan.md, both in
.checkpoints/netcode-proof-test/ on PR #40's branch. The proof test's final report on main repeats this table at its
close.
```
