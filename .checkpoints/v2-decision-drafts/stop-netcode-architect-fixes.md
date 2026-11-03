# stop-netcode-architect fixes (ADR 0001, stop answers 2026-11-03)

### FIX O15
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
**Author:** netcode-architect (the "Pass marks" table: test-engineer, plan step 4) · **Reviewed by:** (filled in at review) · **Signed off by Mustafa:** no
AFTER:
**Author:** netcode-architect (the "Pass marks" table: test-engineer, plan step 4)
**Reviewed by:** (filled in at review)
**Signed off by Mustafa:** as the basis for the spikes (2026-11-03, SPIKE-ORDER (b)); not yet accepted
END

### FIX H3, S3
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
cap. The server ticks at 30 Hz, and 60 Hz only if the netcode proof test shows the 60 Hz margin. The record also sets
the budgets that test is judged against, the message catalogue, the transport (WebTransport with a WebSocket fallback
AFTER:
cap. The server ticks at 30 Hz; it moves to 60 Hz only if the 60 Hz margin passes and the goal budget holds at the rooms
per VM measured at 60 Hz, both judged at slice 6 on the real sim, with the netcode proof test's 60 Hz run as an early
signal only (part 2). The record also sets the budgets the netcode proof test is judged against, the message catalogue,
the transport (WebTransport with a WebSocket fallback
END

### FIX H4
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
- **Tick:** TICK-30 (brief §1): "a 30 Hz server tick is the baseline for 48 players; 60 Hz only if the netcode proof
  test shows at least 25 % headroom on the target hardware" (STATUS, TICK-30), read as
  [the 60 Hz margin](../research/GLOSSARY.md#the-60-hz-margin) (TICK-60-HEADROOM, the advisor's reading of
  2026-10-31, confirmed by Mustafa and the Advisor at the stop).
AFTER:
- **Tick:** TICK-30 (brief §1): "a 30 Hz server tick is the baseline for 48 players; 60 Hz only if the netcode proof
  test shows at least 25 % headroom on the target hardware" (STATUS, TICK-30), read as
  [the 60 Hz margin](../research/GLOSSARY.md#the-60-hz-margin) (TICK-60-HEADROOM, the advisor's reading of 2026-10-31),
  confirmed by Mustafa (2026-11-03) "with one addition: 60 Hz also needs the goal budget to hold at the rooms per VM
  measured at 60 Hz" (OQ-0001-1). The final 60 Hz call is measured at slice 6 with the real sim; the netcode proof
  test's 60 Hz run is an early signal only (TICK-60-SETUP, the Advisor, 2026-11-03: OQ-0001-2; part 2).
END

### FIX H2, S2
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
2. **The tick rate rule (TICK-30).** The server ticks at 30 Hz ([the tick period](../research/GLOSSARY.md#tick-period)
   33.3 ms). It moves to 60 Hz only if the netcode proof test shows all of this, in one run on
   [one vCPU](../research/GLOSSARY.md#the-measuring-unit-one-vcpu) of
   [the reference server](../research/GLOSSARY.md#the-reference-server), in
   [the stress scene](../research/GLOSSARY.md#the-stress-scene) at N = 48 with a snapshot-receiving client on every
   soldier, over R2-BUD-REC6's 30-minute soak, re-flood ticks included: a p99
   [server tick time](../research/GLOSSARY.md#server-tick-time) of at most 6.25 ms against the server tick budget at
   60 Hz, 8.33 ms (a 60 Hz margin of at least 25 %: 1 − 6.25 ÷ 8.33 = 25 %), and every 60 Hz mark the research sets, as
   written (R2-BUD-REC6's two shares, 4.17 ms each; R1-TICK-REC2 P1–P3 at 60 Hz). Otherwise the tick stays at 30 Hz.
   Snapshots go out 30 times a second: on every tick at 30 Hz, and on every second tick in the 60 Hz margin run
   (R2-BUD-REC1's snapshot half, below); inputs are sampled and sent every tick. 60 or 64 Hz is asked only if the
   margin passes (R1-TICK-OQ1). *Why:* at N = 48 the 60 Hz marks leave almost no margin (finding 3), while every legacy
AFTER:
2. **The tick rate rule (TICK-30).** The server ticks at 30 Hz ([the tick period](../research/GLOSSARY.md#tick-period)
   33.3 ms). It moves to 60 Hz only if one run shows all of this, on
   [one vCPU](../research/GLOSSARY.md#the-measuring-unit-one-vcpu) of
   [the reference server](../research/GLOSSARY.md#the-reference-server), in
   [the stress scene](../research/GLOSSARY.md#the-stress-scene) at N = 48 with a snapshot-receiving client on every
   soldier, over R2-BUD-REC6's 30-minute soak, re-flood ticks included: a p99
   [server tick time](../research/GLOSSARY.md#server-tick-time) of at most 6.25 ms against the server tick budget at
   60 Hz, 8.33 ms (a 60 Hz margin of at least 25 %: 1 − 6.25 ÷ 8.33 = 25 %), and every 60 Hz mark the research sets, as
   written (R2-BUD-REC6's two shares, 4.17 ms each; R1-TICK-REC2 P1–P3 at 60 Hz); and only if the goal budget holds at
   the rooms per VM measured at 60 Hz, Mustafa's addition (2026-11-03, OQ-0001-1): "60 Hz also needs the goal budget to
   hold at the rooms per VM measured at 60 Hz". The goal budget is ADR 0006 part 8's as it stands when the call is made,
   at slice 6 the figure OQ-0006-19 fixes, and it holds when the fleet the goal modes need at the rooms per VM measured
   at 60 Hz (part 8's formula, the same machine, N = 48) costs no more than that figure (the advisor's reading,
   2026-11-03; PM-0001-63). Otherwise the tick stays at 30 Hz. The run that decides is slice 6's, with the real sim,
   whatever the early signal showed (migration step 8). The wave-1 60 Hz run, on the netcode proof test's kept server
   and harness after the sim core spike (SPIKE-ORDER (b)), with the sim core spike's sim and ADR 0004's S1 stand-ins
   (OQ-0001-2 (a)), is an early signal only (TICK-60-SETUP, the Advisor, 2026-11-03: "(a), as an early signal only. The
   final 60 Hz call is measured again with the real sim at slice 6."): it decides nothing, its rooms-per-VM cost is
   reported beside part 8's provisional figures, a pass starts no 60 Hz work before slice 6 and a fail closes nothing
   (the advisor's reading, 2026-11-03). Snapshots go out 30 times a second: on every tick at 30 Hz, and on every second
   tick in each 60 Hz margin run (R2-BUD-REC1's snapshot half, below); inputs are sampled and sent every tick. 60 or
   64 Hz is asked only if slice 6's 60 Hz run passes (R1-TICK-OQ1; the advisor's reading, 2026-11-03). *Why:* at N = 48
   the 60 Hz marks leave almost no margin (finding 3), while every legacy
END

### FIX RW4
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
   inflate it as part 8's accepted latency faking already allows). The server keeps a per-client input buffer of 1–3
   ticks (proposed), adapted per client; each command is stamped with the server tick it is meant for. When the server
   runs commands and how the client keeps them just in time are the proof test's to decide (OQ-0001-6); sub-tick fire
   stamps are OQ-0001-7. *The aim point:* the page posts its newest input once per drawn frame, with its cursor ray from
AFTER:
   inflate it as part 8's accepted latency faking already allows). The server keeps a per-client input buffer of
   1–3 ticks (proposed), adapted per client; each command is stamped with the server tick it is meant for. When the
   server runs commands and how the client keeps them just in time are the proof test's to decide (OQ-0001-6), aiming
   for "the smallest input buffer that holds" (Mustafa, 2026-11-03, REWIND-CAP): of the options that pass its marks
   (corrections, fairness, the fast-clock bound), the smallest buffer, with commands run on arrival counting as none
   (the advisor's reading, 2026-11-03); sub-tick fire stamps are OQ-0001-7. *The aim point:* the page posts its newest
   input once per drawn frame, with its cursor ray from
END

### FIX RW2
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
   OQ-0001-5; beyond it, shooters lead by the excess. The history keeps the cap plus a margin (data), at most 1 MB
   for the stress scene. Filters: targets the shooter could hit (friendly fire from mode data), targets sent to and
AFTER:
   so claim rewind up to the cap, an accepted risk. The cap bounds the final rewind on every path: data, its value set
   by Mustafa's rule (2026-11-03, OQ-0001-5): "the cap covers the honest rewind the proof test measures at P2, rounded
   up; beyond the cap, the victim's view wins", read as the honest rewind's p99 at P2, rounded up to a whole tick at the
   run's tick rate (the advisor's reading, 2026-11-03), its value recorded when the proof test reports; beyond it,
   shooters lead by the excess. The history keeps the cap plus a margin (data), at most 1 MB
END

### FIX RW3
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
   needs, nothing is summarised away); ADR 0006 part 9 uploads the file with the record and keeps it 14 days. On the
   round-trip reading of R2's profiles, a P2 player with a 100–150 ms delay needs a rewind of 200–250 ms by R2's
   formula (R2-SYNC-REC5's arithmetic), and 233–383 ms once the buffer (33.3–100 ms at 1–3 ticks) and the blend
   (0–33.3 ms) are added (arithmetic), so a 200 ms cap clips every honest shot at the latency VISION_V2 §4 names, and
   a 250 ms cap all but those with the shortest buffer and blend; the proof test records each shot's RTT, delay,
   buffer wait and blend, and the share each candidate cap clips (OQ-0001-5). *Why:* without it, a 100 ms player
AFTER:
   needs, nothing is summarised away); ADR 0006 part 9 uploads the file with the record and keeps it 14 days. On the
   round-trip reading of R2's profiles, a P2 player with a 100–150 ms delay needs a rewind of 200–250 ms by R2's formula
   (R2-SYNC-REC5's arithmetic), and 233–383 ms once the buffer (33.3–100 ms at 1–3 ticks) and the blend (0–33.3 ms) are
   added (arithmetic), so a 200 ms cap would clip every honest shot at the latency VISION_V2 §4 names, and a 250 ms cap
   all but those with the shortest buffer and blend; the cap instead follows the measured honest rewind (OQ-0001-5's
   rule): the proof test records each shot's RTT, delay, buffer wait, blend and rewind, and sets the cap from them.
   *Why:* without it, a 100 ms player
END

### FIX M14
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
    carry-forward 4); development servers and the netcode proof test use self-signed, pinned certificates for
    WebTransport, valid at most 14 days, whose hash the page fetches from its own HTTPS origin (R2-TRN-REC5); whether
    the staging server that multiplayer previews use presents such a pinned certificate or the browser-trusted one on
    both transports (a hash written into a static preview goes stale) is OQ-0006-5's, with ADR 0002 part 10's two
    shapes; the WebSocket fallback cannot
    pin a hash, since its constructor takes only a URL and sub-protocols (r2-netcode.md, Findings, R2-ROOM-F12, lines
    1494–1500), and presents the certificate ADR 0006's OQ-0006-5 names for development and staging. *Why:*
AFTER:
    carry-forward 4); development servers and the netcode proof test use self-signed, pinned certificates for
    WebTransport, valid at most 14 days, whose hash the page fetches from its own HTTPS origin (R2-TRN-REC5), as Mustafa
    set for the spikes (2026-11-03, SPIKE-MACHINES): "a private hash-pinned certificate for WebTransport"; whether the
    staging server that multiplayer previews use presents such a pinned certificate or the browser-trusted one on both
    transports (a hash written into a static preview goes stale) is OQ-0006-5's, with ADR 0002 part 10's two shapes; the
    WebSocket fallback cannot pin a hash, since its constructor takes only a URL and sub-protocols (r2-netcode.md,
    Findings, R2-ROOM-F12, lines 1494–1500), so in the proof test it presents a browser-trusted certificate on a domain
    Mustafa registers ("for the WebSocket leg, a domain I register": SPIKE-MACHINES, 2026-11-03), OQ-0006-5's
    development case (i), issued by a method the proof test's plan sets with security-auditor (the advisor's reading,
    2026-11-03). *Why:*
END

### FIX RC2
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
    connections and sends nothing (ADR 0004 part 20 says the same; whether a match should ever resume instead is
    OQ-0001-18). *Join:* a
AFTER:
    connections and sends nothing (ADR 0004 part 20 says the same). A match never resumes instead: OQ-0001-18, answered
    by the Advisor (2026-11-03): "end the match, as proposed, and keep the crash and match records", both written and
    kept under ADR 0006 part 11's proposed retention (the advisor's reading, 2026-11-03). *Join:* a
END

### FIX N3
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
    Brief question 1, lines 1987–1992). Each direction carries half the profile's latency and a uniform jitter of
    ± half its jitter, so the round trip keeps the profile's mean and range (proposed; OQ-0001-16). Every R2 mark is
    restated against one: the budgets unit's "100 ms RTT, ±10 ms
    jitter, 2 % loss" becomes P2; the interest unit's "0, 50, 100 and 200 ms latency with ±20 ms jitter and 0–5 % loss"
    becomes P0, P1, P2 and P4; the rooms unit's "0-200 ms simulated latency" with 0–5 % loss becomes P0 to P4. The
    netcode proof test's "100 ms of simulated latency" is read as P2, VISION_V2 §4's "about 100 ms of ping"; the Advisor
    confirms the reading (OQ-0001-3). The simulator that runs the profiles is ADR 0002's. *Research:* R2-XU-OQ12,
AFTER:
    Brief question 1, lines 1987–1992). Each direction carries half the profile's latency and a uniform jitter of ± half
    its jitter, so the round trip keeps the profile's mean and range, and each direction loses packets at the profile's
    loss rate, independently (at P2, 2 % each way, so about 4 % of round trips lose a packet) (decided: OQ-0001-16,
    answered by the Advisor, 2026-11-03: "Latency and jitter split half each way; loss as P2 defines it"; the loss per
    direction is the advisor's reading, 2026-11-03, as PM-0001-3 measures it). Every R2 mark is restated against one:
    the budgets unit's "100 ms RTT, ±10 ms jitter, 2 % loss" becomes P2; the interest unit's "0, 50, 100 and 200 ms
    latency with ±20 ms jitter and 0–5 % loss" becomes P0, P1, P2 and P4; the rooms unit's "0-200 ms simulated latency"
    with 0–5 % loss becomes P0 to P4. The netcode proof test's "100 ms of simulated latency" is P2, VISION_V2 §4's
    "about 100 ms of ping" (OQ-0001-3, answered by the Advisor, 2026-11-03: "yes", P2 is "100 ms round trip, ±20 ms
    jitter, 2 % loss"). The simulator that runs the profiles is ADR 0002's. *Research:* R2-XU-OQ12,
END

### FIX O16
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
    `spikes/` (SPIKE-SCOPE (a)), once Mustafa and the Advisor approve the drafts. It takes in the research spikes of
AFTER:
    `spikes/` (SPIKE-SCOPE (a)), now that Mustafa has signed the drafts off as the basis for the spikes (2026-11-03,
    SPIKE-ORDER (b)), first in wave 1: "wave 1 runs one heavy spike at a time: the netcode proof test first". It takes
    in the research spikes of
END

### FIX H5, S4
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| The 60 Hz margin run (decides 60 Hz) | p99 server tick time ≤ 6.25 ms against 8.33 ms; each share ≤ 4.17 ms p99; R1-TICK-REC2 P1–P3 at 60 Hz; one run, same setup | TICK-60-HEADROOM (the advisor's reading, confirmed at the stop); GLOSSARY §8, the 60 Hz margin |
AFTER:
| The 60 Hz margin run (an early signal in wave 1; slice 6's run with the real sim decides 60 Hz) | p99 server tick time ≤ 6.25 ms against 8.33 ms; each share ≤ 4.17 ms p99; R1-TICK-REC2 P1–P3 at 60 Hz; one run, same setup; the rooms per VM at 60 Hz counted (N = 48, the machine of ADR 0006 part 8's formula) and the goal budget holding at that count (PM-0001-63; in wave 1 that cost is reported beside part 8's provisional figures and decides nothing) | TICK-60-HEADROOM (the advisor's reading, confirmed by Mustafa on 2026-11-03 with his addition, the goal budget); TICK-60-SETUP (the Advisor, 2026-11-03); GLOSSARY §8, the 60 Hz margin |
END

### FIX RC3
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| A panic on the room's tick thread | the match ends: `RoomEnded` with a crash reason where possible, the match record marked crashed, the process exits non-zero and the supervisor records the exit (part 11; OQ-0001-18) | the process gone within 1 s and every client told or closed (proposed: a mark beside PM-0001-26, test-engineer's) |
AFTER:
| A panic on the room's tick thread | the match ends: `RoomEnded` with a crash reason where possible, the match record marked crashed, the process exits non-zero and the supervisor records the exit (part 11; OQ-0001-18, answered (a) by the Advisor, 2026-11-03) | the process gone within 1 s and every client told or closed (PM-0001-59) |
END

### NOTE RW8
no change: the advisor's REWIND-CAP ruling (2026-11-03) keeps "the one-interval tolerance and part 8's anti-cheat text" unchanged; the line's "up to the cap (an accepted risk; OQ-0001-5)" stays true, since the cap's value still comes from OQ-0001-5 (its rule is now recorded there and in part 8, FIX RW1 and RW2).
END

### FIX RW6
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
     blend (0–1 tick) are counted (part 8), so the security review's 200 ms cap clips every honest shot at the very
     latency VISION_V2 §4 names and 250 ms nearly every one, while a longer cap lengthens the victim's "hit behind
     cover" window, which no source measures. Running commands on arrival (OQ-0001-6 (a)) would remove the buffer's
     term. This is a feel trade-off, not an engineering one (OQ-0001-5).
AFTER:
     blend (0–1 tick) are counted (part 8), so the security review's 200 ms cap would clip every honest shot at the very
     latency VISION_V2 §4 names and 250 ms nearly every one, while a longer cap lengthens the victim's "hit behind
     cover" window, which no source measures. Running commands on arrival (OQ-0001-6 (a)) would remove the buffer's
     term. This is a feel trade-off, not an engineering one, and Mustafa's rule settles the value (OQ-0001-5,
     2026-11-03): the cap covers the honest rewind the proof test measures at P2, rounded up, and beyond it the victim's
     view wins; the "hit behind cover" window inside the cap is recorded (the netcode checklist) and weighed by feel
     when he checks the measured cap in Brave (the advisor's reading, 2026-11-03).
END

### FIX RW7
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
- **Revisit when:** the netcode proof test reports (the tick rate, the WebTransport crate, the rewind cap's clipped
  share, any per-kind E3b, the process model, the datagram cap); a browser moves to a new WebTransport draft; connection
AFTER:
- **Revisit when:** the netcode proof test reports (the WebTransport crate, the rewind cap's value by OQ-0001-5's rule,
  any per-kind E3b, the process model, the datagram cap); Mustafa checks the measured cap's feel in Brave (the advisor's
  reading, 2026-11-03); slice 6's 60 Hz run reports (the tick rate: part 2); a browser moves to a new WebTransport
  draft; connection
END

### FIX H1
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
- **OQ-0001-1. Is TICK-60-HEADROOM's reading confirmed?** The 60 Hz margin is measured against the server tick budget
AFTER:
- **OQ-0001-1. Is TICK-60-HEADROOM's reading confirmed?** Answered by Mustafa (2026-11-03): "confirm, with one addition:
  60 Hz also needs the goal budget to hold at the rooms per VM measured at 60 Hz." So (a) is decided, with his addition:
  part 2 holds the goal-budget condition and its reading, and PM-0001-63 counts the rooms per VM at 60 Hz with the
  goal-budget check. The question as asked: the 60 Hz margin is measured against the server tick budget
END

### FIX S1
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
- **OQ-0001-2. How does the stress scene's sim load reach the 60 Hz margin run (TICK-60-SETUP)?** · *Why it matters:*
AFTER:
- **OQ-0001-2. How does the stress scene's sim load reach the 60 Hz margin run (TICK-60-SETUP)?** Answered by the
  Advisor (2026-11-03): "(a), as an early signal only. The final 60 Hz call is measured again with the real sim at
  slice 6." So (a) is decided for the wave-1 run, which decides nothing: slice 6 always runs the final 60 Hz measurement
  on the real sim, whatever the early signal shows; a pass starts no 60 Hz work before slice 6 (PM-0004-71 and -72 run
  with each 60 Hz run but decide only at slice 6, and R1-TICK-OQ1 is asked only after slice 6 passes), and a fail closes
  nothing (the advisor's reading, 2026-11-03). The options below stay as the record of what was weighed, (c) and (d)
  included. · *Why it matters:*
END

### FIX N1
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
- **OQ-0001-3. Is "100 ms of simulated latency" P2?** The proof test's human client runs at P2: 100 ms round trip,
AFTER:
- **OQ-0001-3. Is "100 ms of simulated latency" P2?** Answered by the Advisor (2026-11-03): "yes. '100 ms' is P2: 100 ms
  round trip, ±20 ms jitter, 2 % loss (VISION_V2 §4's 'about 100 ms of ping'). Latency and jitter split half each way;
  loss as P2 defines it." So (a) is decided (its last sentence answers OQ-0001-16). The proof test's human client runs
  at P2: 100 ms round trip,
END

### FIX RW1
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
- **OQ-0001-5. The rewind cap: which value, and the victim's side?** · *Why it matters:* at P2 a 30 Hz client needs
  200–250 ms of rewind by R2's formula and 233–383 ms once the input buffer and the page's blend are counted (part 8),
  so both candidate caps clip honest shots at VISION_V2 §4's latency, while a longer cap lengthens the window in which
  a victim is hit just after reaching cover; the cap is also the most rewind a client faking its latency can claim
  (part 8). · *Options:* (a) 200 ms (the default principles; the security review prefers it, with a tolerance of one
  snapshot interval); (b) 250 ms (the sync unit's proposal); (c) a cap set from the proof test's measured honest
  rewind (the p99 at P2), the one-interval tolerance unchanged (not researched: the research weighs 200 and 250 ms
  only); (d) run commands on arrival (OQ-0001-6 (a), Source's model), which removes the buffer's term from every
  rewind; (e) start the interpolation delay at 2 intervals instead of 3, at the cost of more extrapolation under loss
  (part 7). · *Recommendation:* none until the proof test reports; the netcode review leans to (c) with (d) measured
  beside it, but (c) is not researched, so choosing it is the deciders' call. · *Decider:* Mustafa and the Advisor. ·
  *What decides it:* the proof test's numbers at P2 and P3 (each shot's measured rewind, the share of shots each cap
  clips, the "hit after cover" window), then a Feel Lab weighing. · *Research ids:* R2-XU-OQ1, R2-SYNC-OQ2,
  R2-SYNC-OQ15, R2-XU-OQ2.
AFTER:
- **OQ-0001-5. The rewind cap: which value, and the victim's side?** Answered in part by Mustafa (2026-11-03): "no value
  now. Rule: the cap covers the honest rewind the proof test measures at P2, rounded up; beyond the cap, the victim's
  view wins. Aim for the smallest input buffer that holds." The rule is decided, in option (c)'s shape: the honest
  rewind is its p99 at P2, as option (c) and the stop report's recommendation he answered define it, with the run's
  largest rewind reported beside it; rounded up means to a whole tick at the run's tick rate, as PM-0001-48 rounds; the
  one-interval tolerance and part 8's anti-cheat text are unchanged; the victim's side is decided, since beyond the cap
  the victim's view wins (shooters lead by the excess); the buffer aim guides OQ-0001-6 (the advisor's reading,
  2026-11-03). Still open: the cap's value, decided by the proof test's measured honest rewind at P2 under that rule and
  recorded when the proof test reports. · *Why it matters:* at P2 a 30 Hz client needs 200–250 ms of rewind by R2's
  formula and 233–383 ms once the input buffer and the page's blend are counted (part 8), so both candidate caps clip
  honest shots at VISION_V2 §4's latency, while a longer cap lengthens the window in which a victim is hit just after
  reaching cover; the cap is also the most rewind a client faking its latency can claim (part 8). · *Options:* (a)
  200 ms (the default principles; the security review prefers it, with a tolerance of one snapshot interval); (b) 250 ms
  (the sync unit's proposal); (c) a cap set from the proof test's measured honest rewind (the p99 at P2), the
  one-interval tolerance unchanged (not researched: the research weighs 200 and 250 ms only); (d) run commands on
  arrival (OQ-0001-6 (a), Source's model), which removes the buffer's term from every rewind; (e) start the
  interpolation delay at 2 intervals instead of 3, at the cost of more extrapolation under loss (part 7). ·
  *Recommendation:* none until the proof test reports; the netcode review leans to (c) with (d) measured beside it, but
  (c) is not researched, so choosing it is the deciders' call. · *Decider:* Mustafa and the Advisor. · *What decides
  it:* the value: the proof test's measured honest rewind at P2 (each shot's measured rewind), with the "hit after
  cover" window inside the cap recorded beside it; the share each candidate cap clips and a later Feel Lab weighing no
  longer decide anything, since no candidates are left to weigh (the advisor's reading, 2026-11-03). · *Research ids:*
  R2-XU-OQ1, R2-SYNC-OQ2, R2-SYNC-OQ15, R2-XU-OQ2.
END

### FIX RW5
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
  skipping or repeating a tick (not researched). · *Decider:* the netcode proof test. · *What decides it:* correction
  rates, fairness and the measured rewind under P2 to P4. · *Research ids:* R2-SYNC-OQ3, R1-TICK-OQ8, R1-XT-OQ5 =
  R2-XT-OQ5 (the time-sync part).
AFTER:
  skipping or repeating a tick (not researched). · *Decider:* the netcode proof test. · *What decides it:* correction
  rates, fairness and the measured rewind under P2 to P4, aiming for "the smallest input buffer that holds" (Mustafa,
  2026-11-03, REWIND-CAP): of the options that pass the proof test's marks (corrections, fairness, the fast-clock
  bound), the smallest buffer, with (a), run on arrival, counting as none; the options stay (the advisor's reading,
  2026-11-03). · *Research ids:* R2-SYNC-OQ3, R1-TICK-OQ8, R1-XT-OQ5 = R2-XT-OQ5 (the time-sync part).
END

### FIX X1
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
- **OQ-0001-14. Is a VISION_V2 §3 or §4 target trace enough for an engine feature?** R2-BUD-REC4, R2-BUD-REC5 and
AFTER:
- **OQ-0001-14. Is a VISION_V2 §3 or §4 target trace enough for an engine feature?** Answered by the Advisor
  (2026-11-03): "yes, a VISION_V2 §3 or §4 trace counts, marked as such." So (a) is decided: the needs written
  "VISION_V2 §4 only" and listed in `docs/adr/README.md` stand as written. The question as asked: R2-BUD-REC4,
  R2-BUD-REC5 and
END

### FIX N2
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
- **OQ-0001-16. How does each network profile split its latency and jitter over the two directions?** Part 13 gives
AFTER:
- **OQ-0001-16. How does each network profile split its latency and jitter over the two directions?** Answered by the
  Advisor (2026-11-03): "Latency and jitter split half each way; loss as P2 defines it." So (a) is decided, and each
  direction loses packets at the profile's loss rate, independently: at P2, 2 % each way, as PM-0001-3 measures it, so
  about 4 % of round trips lose a packet (the advisor's reading, 2026-11-03). Part 13 gives
END

### FIX RC1
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
- **OQ-0001-18. When a room's simulation panics mid-match, does the match end (ROOM-CRASH)?** Part 11's proposal: the
AFTER:
- **OQ-0001-18. When a room's simulation panics mid-match, does the match end (ROOM-CRASH)?** Answered by the Advisor
  (2026-11-03): "end the match, as proposed, and keep the crash and match records." So (a) is decided: the match ends,
  and the crash report and the match record are written and kept under ADR 0006 part 11's proposed retention, with no
  new period, the record marked crashed as part 11 and PM-0001-59 word it (the advisor's reading, 2026-11-03). The
  options below stay as the record of what was weighed. Part 11's proposal: the
END

### FIX O16-2
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
It runs before the foundation (SLICE-ORDER condition 3), throwaway, in `spikes/` (SPIKE-SCOPE (a)), only after
Mustafa and the Advisor approve the drafts. Its sim is a throwaway one with movement, hitscan and bots as protocol
AFTER:
It runs before the foundation (SLICE-ORDER condition 3), throwaway, in `spikes/` (SPIKE-SCOPE (a)), now that Mustafa has
signed the drafts off as the basis for the spikes (2026-11-03, SPIKE-ORDER (b)), as the first of wave 1's heavy spikes,
which run one at a time. Its sim is a throwaway one with movement, hitscan and bots as protocol
END

### FIX M15
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
     attempts, and reject one valid for 15 days (R2-TRN-REC5 marks 1 and 3); the WebSocket leg first needs the
     certificate OQ-0006-5 names for development and staging (the Advisor, before the test starts; ADR 0006's
     PM-0006-15).
AFTER:
     attempts, and reject one valid for 15 days (R2-TRN-REC5 marks 1 and 3), the "private hash-pinned certificate for
     WebTransport" Mustafa set (SPIKE-MACHINES, 2026-11-03); the WebSocket leg first needs a browser-trusted certificate
     on the domain he registers ("for the WebSocket leg, a domain I register"), OQ-0006-5's development case (i), issued
     by a method the proof test's plan sets with security-auditor (the advisor's reading, 2026-11-03; ADR 0006's
     PM-0006-15).
END

### FIX H6, S5
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
3. **The 60 Hz margin (part 2):** in one run with part 2's setup, p99 server tick time at most 6.25 ms at a 60 Hz tick,
   both shares within 4.17 ms, and R1-TICK-REC2 P1–P3 at 60 Hz; how the sim load gets there is OQ-0001-2. A failure
   keeps 30 Hz.
AFTER:
3. **The 60 Hz margin (part 2):** in one run with part 2's setup, p99 server tick time at most 6.25 ms at a 60 Hz tick,
   both shares within 4.17 ms, and R1-TICK-REC2 P1–P3 at 60 Hz, with the rooms per VM at 60 Hz counted and the goal
   budget checked at that count (PM-0001-63; Mustafa's addition, OQ-0001-1). In wave 1 this run is an early signal only,
   on the netcode proof test's kept server and harness, in its own VM session after the sim core spike
   (SPIKE-ORDER (b)), with the sim core spike's sim and ADR 0004's S1 stand-ins (OQ-0001-2 (a), the Advisor,
   2026-11-03): it reports separately and decides nothing, and the proof test closes at 30 Hz without it (the advisor's
   reading, 2026-11-03). Slice 6 runs it again with the real sim and decides the tick rate (migration step 8); a failure
   there keeps 30 Hz.
END

### FIX RW9
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
     shot rewound beyond the cap, and the share each candidate cap clips recorded (mark 6, OQ-0001-5); a cheat client at
AFTER:
     shot rewound beyond the cap, the cap set by OQ-0001-5's rule from the measured honest rewind at P2 (mark 6; the
     rule is Mustafa's, 2026-11-03); a cheat client at
END

### FIX RW9-2
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
   command runs, with each shot's RTT, delay, buffer wait and blend (R2-XU-OQ2; the cap itself is OQ-0001-5's); the
AFTER:
   command runs, with each shot's RTT, delay, buffer wait and blend (R2-XU-OQ2), and from it the rewind cap by
   OQ-0001-5's rule (Mustafa, 2026-11-03): the honest rewind's p99 at P2, rounded up to a whole tick, the run's largest
   rewind reported beside it (the advisor's reading, 2026-11-03); the
END

### FIX H10, S8, O17
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
1. **The netcode proof test** (before the foundation): run after the drafts are approved, beside the other phase 0
   spikes `docs/SPIKE_PLAN.md` orders. *Done when:* the pass marks are met and recorded in this record, with the tick
   rate (30 Hz unless the 60 Hz margin passes), the WebTransport crate, E3 and any per-kind E3b, the process model
   and the datagram
   cap named; Mustafa's Brave checklist passed.
AFTER:
1. **The netcode proof test** (before the foundation): first in wave 1, now that Mustafa has signed the drafts off as
   the basis for the spikes (2026-11-03, SPIKE-ORDER (b)); wave 1 runs one heavy spike at a time, in
   `docs/SPIKE_PLAN.md`'s order. *Done when:* the pass marks outside the 60 Hz group are met and recorded in this
   record, at the 30 Hz baseline, with the WebTransport crate, E3 and any per-kind E3b, the process model, the datagram
   cap and the rewind cap (OQ-0001-5's rule) named; Mustafa's Brave checklist passed. The 60 Hz early signal runs later,
   on this test's kept server and harness in its own VM session after the sim core spike, and reports separately: it
   decides nothing, and the tick stays at 30 Hz until slice 6 (part 2; the advisor's reading, 2026-11-03).
END

### FIX RW1-2
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
   and pickups (part 12), and the Feel Lab tests (OQ-0001-4, -5, -8, -10). *Done when:* the refusal tests of part 12
   and R2-ROOM-REC1 mark 3 pass, and Mustafa has answered the Feel Lab questions.
AFTER:
   and pickups (part 12), and the Feel Lab tests (OQ-0001-4, -8, -10; the rewind cap's feel is Mustafa's Brave check of
   the measured cap, not a Feel Lab weighing, under REWIND-CAP: the advisor's reading, 2026-11-03). *Done when:* the
   refusal tests of part 12 and R2-ROOM-REC1 mark 3 pass, and Mustafa has answered the Feel Lab questions.
END

### FIX S8-2
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
8. **Slice 6, scale to 48:** the budgets again with the real sim in the stress scene, 8 rooms per VM (with ADR 0006),
   the soaks on the test-mode list, and the 60 Hz margin run here if OQ-0001-2 chose (c). *Done when:* the budgets
   table passes with the real sim and R5-TEST-REC5's soak passes for every mode on
   [the test-mode list](../research/GLOSSARY.md#the-test-mode-list).
AFTER:
8. **Slice 6, scale to 48:** the budgets again with the real sim in the stress scene, 8 rooms per VM (with ADR 0006),
   the soaks on the test-mode list, and the final 60 Hz margin run with the real sim, which always runs here whatever
   the wave-1 early signal showed, with the rooms per VM at 60 Hz counted and the goal budget checked at that count
   (part 2; PM-0001-63; TICK-60-SETUP, the Advisor, 2026-11-03). *Done when:* the budgets table passes with the real
   sim, the tick rate is set by part 2's rule, and R5-TEST-REC5's soak passes for every mode on
   [the test-mode list](../research/GLOSSARY.md#the-test-mode-list).
END

### FIX H11
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
- **The 60 Hz margin fails:** nothing to undo; the tick stays at 30 Hz.
AFTER:
- **The 60 Hz margin fails, or the goal budget does not hold at the rooms per VM measured at 60 Hz:** nothing to undo;
  the tick stays at 30 Hz. A failing early signal in wave 1 closes nothing: slice 6 runs the final 60 Hz measurement
  with the real sim all the same (part 2; the advisor's reading, 2026-11-03).
END

### FIX S9
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| R1-TICK-REC2 | changed: TICK-30 replaces its rule (r1-simulation-core.md:2029, 2061–2062); 30 Hz is the baseline, 60 Hz only with the 60 Hz margin; P1–P3 stay marks at 30 Hz and, for the margin run, at 60 Hz; P1 and P2 are ADR 0004's, P3 this record's | parts 2, 4; budgets table |
AFTER:
| R1-TICK-REC2 | changed: TICK-30 replaces its rule (r1-simulation-core.md:2029, 2061–2062); 30 Hz is the baseline, 60 Hz only with the 60 Hz margin and the goal budget holding at the rooms per VM measured at 60 Hz, decided at slice 6 on the real sim (part 2; OQ-0001-1, OQ-0001-2, answered 2026-11-03); P1–P3 stay marks at 30 Hz and, for the margin runs, at 60 Hz; P1 and P2 are ADR 0004's, P3 this record's | parts 2, 4; budgets table |
END

### FIX S9-2
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| R1-TICK-OQ1 | spike: only if the 60 Hz margin passes; then ADR 0004's determinism suite and ADR 0002's Worker smoothness test decide | part 2 |
AFTER:
| R1-TICK-OQ1 | spike: only if slice 6's 60 Hz run passes (part 2; the advisor's reading, 2026-11-03); then ADR 0004's determinism suite and ADR 0002's Worker smoothness test decide | part 2 |
END

### FIX RW13
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| R2-BUD-REC4 | changed: the interpolation bound follows R2-SYNC-REC4 (R2-XU-OQ3); the rewind cap is OQ-0001-5 and the history keeps the cap plus a margin, not 1 s (R2-XU-OQ7); the input buffer and the clock are adopted; need: VISION_V2 §4 only | parts 5, 7, 8 |
AFTER:
| R2-BUD-REC4 | changed: the interpolation bound follows R2-SYNC-REC4 (R2-XU-OQ3); the rewind cap follows OQ-0001-5's rule (Mustafa, 2026-11-03: the honest rewind the proof test measures at P2, rounded up; its value from the proof test) and the history keeps the cap plus a margin, not 1 s (R2-XU-OQ7); the input buffer and the clock are adopted; need: VISION_V2 §4 only | parts 5, 7, 8 |
END

### FIX RC5
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| R2-ROOM-REC2 | adopted; a panic on the tick thread ends the match and exits the process non-zero (OQ-0001-18) | part 11 |
AFTER:
| R2-ROOM-REC2 | adopted; a panic on the tick thread ends the match and exits the process non-zero (OQ-0001-18, answered (a) by the Advisor, 2026-11-03) | part 11 |
END

### FIX RW13-2
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| R2-SYNC-OQ2 | open: OQ-0001-5 | part 8 |
AFTER:
| R2-SYNC-OQ2 | answered in part: the cap follows OQ-0001-5's rule (Mustafa, 2026-11-03), not a fixed 200 or 250 ms; its value is set when the proof test reports | part 8 |
END

### FIX RW13-3
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| R2-SYNC-OQ14 | answered: no recommendation rests on the unread Valve pages; the victim's side is OQ-0001-5 | part 8 |
AFTER:
| R2-SYNC-OQ14 | answered: no recommendation rests on the unread Valve pages; the victim's side is answered by OQ-0001-5's rule (Mustafa, 2026-11-03: beyond the cap, the victim's view wins) | part 8 |
END

### FIX RW13-4
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| R2-SYNC-OQ15 | open: OQ-0001-5 | part 8 |
AFTER:
| R2-SYNC-OQ15 | answered: beyond the cap, the victim's view wins (OQ-0001-5, Mustafa, 2026-11-03); inside it the proof test records the "hit after cover" window (the netcode checklist) | part 8 |
END

### FIX RW13-5
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| R2-XU-OQ1 | open: OQ-0001-5 | part 8 |
AFTER:
| R2-XU-OQ1 | answered in part: the cap follows OQ-0001-5's rule (Mustafa, 2026-11-03), not a fixed 200 or 250 ms; its value is set when the proof test reports | part 8 |
END

### FIX X2
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| R2-XU-OQ18 | open: OQ-0001-14 (r2-netcode.md:3268 leaves it to the Advisor); until the Advisor rules, these drafts use the session's trace reading (a VISION_V2 §3 or §4 target counts like LOOP_V2, written "need: VISION_V2 §4 only" and listed in docs/adr/README.md) for R2-BUD-REC4, R2-BUD-REC5 and R2-TRN-REC6; R2-BUD-REC3 traces to §6.1; R2-INT-REC7 is not built | parts 7, 10; Open questions |
AFTER:
| R2-XU-OQ18 | answered: OQ-0001-14 (a), by the Advisor (2026-11-03), whom r2-netcode.md:3268 left it to: a VISION_V2 §3 or §4 target counts like LOOP_V2, written "need: VISION_V2 §4 only" and listed in docs/adr/README.md, for R2-BUD-REC4, R2-BUD-REC5 and R2-TRN-REC6; R2-BUD-REC3 traces to §6.1; R2-INT-REC7 is not built | parts 7, 10; Open questions |
END

### NOTE X5
no change: ADR 0001's need lines (part 10's congestion handling and part 13: "VISION_V2 §4 only"; R2-BUD-REC4's "need: VISION_V2 §4 only" in Appendix A) are already "marked as such", which is what the Advisor's answer to OQ-0001-14 asks; the verdict rules "OQ-0001-14: (a). No need line changes." The other drafts' need lines belong to their authors.
END

