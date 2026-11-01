# ADR 0001 fix round (plan step 9): exact replacements for docs/adr/0001-multiplayer-netcode.md

Author: netcode-architect, 2026-11-01. Each BEFORE is copied from the file at its current head and is unique in it.

### FIX F2
BEFORE:
  5. *Weight in the Worker.* The interpolation buffer and the aim ray live in the Worker, and in a local match the
     Worker also encodes and decodes every message for the loopback path; that cost is not measured yet and counts in
     the client's sim share, ADR 0004's (the proof test reports it).
AFTER:
  5. *Weight in the Worker.* The interpolation buffer and the aim ray live in the Worker, and in a local match the
     Worker also encodes and decodes every message for the loopback path; that cost is not measured yet. It is
     reported beside the client's sim share (ADR 0004's), not inside it, since that share is sim step time only
     (GLOSSARY.md:214–218).
END

### FIX F4
BEFORE:
| R2-BUD-REC1 | changed: the snapshot half (30 Hz) stands; the tick half is replaced by TICK-30; inputs repeat every unacknowledged command up to the cap (R2-SYNC-REC2) instead of 3; marks restated at 30 Hz and against P2 and P3 | parts 2, 5; budgets table |
| R2-BUD-REC2 | adopted; the hard cap proposed at 256 kbps | part 6; budgets table |
AFTER:
| R2-BUD-REC1 | changed: the snapshot half (30 Hz) stands; the tick half is replaced by TICK-30; inputs repeat every unacknowledged command up to the cap (R2-SYNC-REC2) instead of 3; marks restated at 30 Hz and against P2 and P3; its Shabzon mark (mean delivered fire rate within 1 % of 13.9 shots/s at the chosen tick, r2-netcode.md:2324–2325) is a sim mark, carried by ADR 0004's mark S4 (one time representation holds every rate, which meets R1-ABL-REC3's 1 % for the Shabzon) | parts 2, 5; budgets table; ADR 0004 |
| R2-BUD-REC2 | changed: its precision mark "decoded positions within 1 cm and angles within 0.01° of the server's values" (r2-netcode.md:2341) cannot pass for pitch, sent in 10 bits of 0.18° steps (line 1748); restated as positions within 1 cm, yaw within 0.01° (a 16-bit step is about 0.0055°, line 2263) and pitch within one 0.18° step (proposed: the field's own resolution); the hard cap proposed at 256 kbps | part 6; budgets table |
END

### FIX F5 (Appendix A row)
BEFORE:
| R2-XU-OQ18 | answered: the drafts' trace reading (a VISION_V2 §3 or §4 target counts like LOOP_V2, written "need: VISION_V2 §4 only" and listed in docs/adr/README.md): R2-BUD-REC4, R2-BUD-REC5, R2-TRN-REC6; R2-BUD-REC3 traces to §6.1; R2-INT-REC7 is not built | parts 7, 10 |
AFTER:
| R2-XU-OQ18 | open: OQ-0001-14 (r2-netcode.md:3268 leaves it to the Advisor); until the Advisor rules, these drafts use the session's trace reading (a VISION_V2 §3 or §4 target counts like LOOP_V2, written "need: VISION_V2 §4 only" and listed in docs/adr/README.md) for R2-BUD-REC4, R2-BUD-REC5 and R2-TRN-REC6; R2-BUD-REC3 traces to §6.1; R2-INT-REC7 is not built | parts 7, 10; Open questions |
END

### FIX F5 (new open question)
BEFORE:
- **OQ-0001-13. What do real players' networks show?** How many need the WebSocket fallback, the IPv6 share, and how
  often an address change forces a reconnect. · *Why it matters:* how good the fallback must feel, the IPv6 overhead,
  how often reconnect runs (not its design). · *Options:* none (measurements). · *Decider:* connection telemetry from
  the first online playtest (ADR 0006's monitoring). · *What decides it:* the measured shares. · *Research ids:*
  R2-TRN-OQ7, R2-BUD-OQ11, R2-ROOM-OQ8.
AFTER:
- **OQ-0001-13. What do real players' networks show?** How many need the WebSocket fallback, the IPv6 share, and how
  often an address change forces a reconnect. · *Why it matters:* how good the fallback must feel, the IPv6 overhead,
  how often reconnect runs (not its design). · *Options:* none (measurements). · *Decider:* connection telemetry from
  the first online playtest (ADR 0006's monitoring). · *What decides it:* the measured shares. · *Research ids:*
  R2-TRN-OQ7, R2-BUD-OQ11, R2-ROOM-OQ8.
- **OQ-0001-14. Is a VISION_V2 §3 or §4 target trace enough for an engine feature?** R2-BUD-REC4, R2-BUD-REC5 and
  R2-TRN-REC6 serve only VISION_V2 §4's "matches feel fair up to about 100 ms of ping". · *Why it matters:* the trace
  rule admits a feature only with its need, and these three carry the fairness target. · *Options:* (a) yes: a §3 or §4
  target counts like LOOP_V2, written "need: VISION_V2 §4 only" and listed in `docs/adr/README.md` (the reading these
  drafts use until the ruling); (b) no: each is traced to a §6 item its own reasoning supports, or dropped. ·
  *Decider:* the Advisor, at the stop. · *What decides it:* the Advisor's ruling. · *Research ids:* R2-XU-OQ18
  (r2-netcode.md, Open questions, lines 3260–3269).
END

### FIX F6
BEFORE:
- **Load and soak:** bots as headless protocol clients, 48 per room, 30 minutes at P2 for every mode on the test-mode
  list, with server tick time, bytes per client, memory slope, queue lengths, desyncs, panics and hangs measured, and
AFTER:
- **Load and soak:** bots as headless protocol clients, a full room at each mode's N, 30 minutes at P2 for every mode on
  the test-mode list (2 to 64 players: GLOSSARY.md:375–388), with server tick time, bytes per client, memory slope, queue lengths, desyncs, panics and hangs measured, and
END

### FIX F7
BEFORE:
| **U1. Every tick, newest + repeats (chosen, at 30 Hz, repeating every unacknowledged command up to the cap)** | low | about 22.3 kbps at 30 Hz with 8 B commands | ≤ 48 kbps | the catch-up cap equals the redundancy cap | no extra input delay | CS1's command record | an input is lost only after several lost packets in a row | more bytes than U2 |
AFTER:
| **U1. Every tick, newest + repeats (chosen, at 30 Hz, repeating every unacknowledged command up to the cap)** | low | about 22.3 kbps at 30 Hz with 8 B commands | ≤ 48 kbps | the catch-up cap no larger than the redundancy cap | no extra input delay | CS1's command record | an input is lost only after several lost packets in a row | more bytes than U2 |
END

### FIX F8
BEFORE:
and written down (R5-TEST-REC6). A run set that fails V-S is neither a pass nor a fail. Then, by the draft's own
Rollback (lines 1056–1068): a 30 Hz baseline mark (PM-0001-4 to PM-0001-27) that fails sends part 2 and part 3 back to
AFTER:
and written down (R5-TEST-REC6). A run set that fails V-S is neither a pass nor a fail. Then, by this record's own
Rollback section: a 30 Hz baseline mark (PM-0001-4 to PM-0001-27) that fails sends part 2 and part 3 back to
END

### FIX F9
BEFORE:
   (r2-netcode.md:2402–2403) is restated: "the topics' own part marks fit inside the server tick budget's 50 % of the
   tick period at 30 Hz, re-flood ticks included, and at 60 Hz on every tick but one that both encodes a snapshot and
   re-floods after a portal change (at N = 54, MODES-3 (b)); at N = 48 that tick fits by 0.05 ms (8.28 of 8.33 ms)
   while its server-only share is 0.24 ms over its own 25 % (4.41 of 4.17 ms) (GLOSSARY.md:62–75)." *Why:* one
AFTER:
   (r2-netcode.md:2402–2403) is restated: "the topics' own part marks fit inside the server tick budget's 50 % of the
   tick period at 30 Hz, and at 60 Hz on every tick but one that both encodes a snapshot and
   re-floods after a portal change (at N = 54, MODES-3 (b)); at N = 48 that tick fits by 0.05 ms (8.28 of 8.33 ms)
   while its server-only share is 0.24 ms over its own 25 % (4.41 of 4.17 ms) (GLOSSARY.md:62–75)." At 30 Hz the
   marks fit with re-flood ticks included (10.18 of 16.67 ms: GLOSSARY.md:69, 76–77). *Why:* one
END

### FIX F10 (Q6)
BEFORE:
### Q6. Projectiles (sync options 3.6, lines 1655–1664; encodings E3 and E3b, lines 1694–1700)
AFTER:
### Q6. Projectiles (sync options 3.6, lines 1655–1664; encodings E3 and E3b, lines 1694–1702)
END

### FIX F10 (Q9)
BEFORE:
### Q9. Downstream encoding (budgets option table B, lines 1694–1700)
AFTER:
### Q9. Downstream encoding (budgets option table B, lines 1694–1702)
END
