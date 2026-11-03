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
    its jitter, so the round trip keeps the profile's mean and range, and each direction drops the profile's loss share
    on its own (at P2, 2 % each way, so about 4 % of round trips lose a packet) (decided: OQ-0001-16, answered by the
    Advisor, 2026-11-03: "Latency and jitter split half each way; loss as P2 defines it"; the loss per direction is the
    advisor's reading, 2026-11-03, as PM-0001-3 measures it). Every R2 mark is restated against one: the budgets unit's
    "100 ms RTT, ±10 ms jitter, 2 % loss" becomes P2; the interest unit's "0, 50, 100 and 200 ms latency with ±20 ms
    jitter and 0–5 % loss" becomes P0, P1, P2 and P4; the rooms unit's "0-200 ms simulated latency" with 0–5 % loss
    becomes P0 to P4. The netcode proof test's "100 ms of simulated latency" is P2, VISION_V2 §4's "about 100 ms of
    ping" (OQ-0001-3, answered by the Advisor, 2026-11-03: "yes", P2 is "100 ms round trip, ±20 ms jitter, 2 % loss").
    The simulator that runs the profiles is ADR 0002's. *Research:* R2-XU-OQ12,
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

