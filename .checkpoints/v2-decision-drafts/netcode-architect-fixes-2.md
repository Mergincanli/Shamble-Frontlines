# ADR 0001, second fix round (plan step 9, consistency findings): exact replacements

Author: netcode-architect, 2026-11-01. Each BEFORE was copied from the file at 03df0ae by script and is unique in it.

### FIX F1
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| `ContentReload` (development builds only) | room → client | control stream | public | on a valid content change | the changed set and the tick it applies from | only if ADR 0005 keeps development reload over the game connection (R1-DATA-REC5) |
AFTER:
| `ContentReload` (development builds only) | room → client | control stream | public | on a valid content change | the changed set and the tick it applies from | only if OQ-0005-CD4 chooses option (b), the mid-match swap (R1-DATA-REC5); under its option (a) nothing in a running match changes and no message is sent |
END

### FIX F2
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
    pick and pickup telemetry into the match record (fields: ADR 0006). No rule is decided here: carry slots, ammo for
    looted weapons and how fast tracks level are Mustafa's (ADR 0005 lists them). *Why:* the tracks belong to the
AFTER:
    pick and pickup telemetry into the match record (fields: ADR 0006). No rule is decided here: carry slots, ammo for
    looted weapons, how fast tracks level, the drop rules and bots' records are Mustafa's (ADR 0005 lists them; the
    last two are OQ-0005-AB9 and OQ-0005-AB10). *Why:* the tracks belong to the
END

### FIX F3 (part 2)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
   written (R2-BUD-REC6's two shares, 4.2 ms each; R1-TICK-REC2 P1–P3 at 60 Hz). Otherwise the tick stays at 30 Hz.
AFTER:
   written (R2-BUD-REC6's two shares, 4.17 ms each; R1-TICK-REC2 P1–P3 at 60 Hz). Otherwise the tick stays at 30 Hz.
END

### FIX F3 (part 3)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
3. **The server tick budget at 30 Hz.** p99 server tick time at most 16.7 ms (50 % of the tick period), max under one
   tick period (33.3 ms), zero overruns, split into
   [the sim step share and the server-only share](../research/GLOSSARY.md#the-sim-step-share-and-the-server-only-share),
   at most 8.3 ms p99 each, measured separately. The sim step share and its parts are ADR 0004's; this record adds that
AFTER:
3. **The server tick budget at 30 Hz.** p99 server tick time at most 16.67 ms (50 % of the tick period), max under one
   tick period (33.33 ms), zero overruns, split into
   [the sim step share and the server-only share](../research/GLOSSARY.md#the-sim-step-share-and-the-server-only-share),
   at most 8.33 ms p99 each, measured separately. The sim step share and its parts are ADR 0004's; this record adds that
END

### FIX F3 (budgets table)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| Server tick time, 30 Hz (the baseline) | p99 ≤ 16.7 ms; max < 33.3 ms; 0 overruns (0 missed ticks) | R2-BUD-REC6 (r2-netcode.md:2398–2416) restated from GLOSSARY §1's 30 Hz column (GLOSSARY.md:53–69, 167–188); R1-TICK-REC2 P1 |
| Its two shares, 30 Hz | sim step time p99 ≤ 8.3 ms (ADR 0004's parts); server-only work p99 ≤ 8.3 ms, measured separately | R2-BUD-REC6; R2-SYNC-REC9 mark 1; GLOSSARY.md:189–213 |
| Parts of the server-only share | snapshot encode ≤ 50 µs per client per snapshot p99; rewound shot ≤ 20 µs p99 at 700 shots/s; region query ≤ 0.2 ms p99; ranking and packing ≤ 0.5 ms p99; pair test ≤ 0.1 µs per pair p99; re-flood ≤ 1 ms p99 (reported beside the whole, never summed into it) | R2-BUD-REC6; R2-SYNC-REC5 mark 4; R2-INT-REC1 (c); R2-INT-REC4; R2-INT-REC2 (b) |
| The 60 Hz margin run (decides 60 Hz) | p99 server tick time ≤ 6.25 ms against 8.33 ms; each share ≤ 4.2 ms p99; R1-TICK-REC2 P1–P3 at 60 Hz; one run, same setup | TICK-60-HEADROOM (the advisor's reading, confirmed at the stop); GLOSSARY §8, the 60 Hz margin |
AFTER:
| Server tick time, 30 Hz (the baseline) | p99 ≤ 16.67 ms; max < 33.33 ms; 0 overruns (0 missed ticks) | R2-BUD-REC6 (r2-netcode.md:2398–2416) restated from GLOSSARY §1's 30 Hz column (GLOSSARY.md:53–69, 167–188); R1-TICK-REC2 P1 |
| Its two shares, 30 Hz | sim step time p99 ≤ 8.33 ms (ADR 0004's parts); server-only work p99 ≤ 8.33 ms, measured separately | R2-BUD-REC6; R2-SYNC-REC9 mark 1; GLOSSARY.md:189–213 |
| Parts of the server-only share | snapshot encode ≤ 50 µs per client per snapshot p99; rewound shot ≤ 20 µs p99 at 700 shots/s; region query ≤ 0.2 ms p99; ranking and packing ≤ 0.5 ms p99; pair test ≤ 0.1 µs per pair p99; re-flood ≤ 1 ms p99 (reported beside the whole, never summed into it) | R2-BUD-REC6; R2-SYNC-REC5 mark 4; R2-INT-REC1 (c); R2-INT-REC4; R2-INT-REC2 (b) |
| The 60 Hz margin run (decides 60 Hz) | p99 server tick time ≤ 6.25 ms against 8.33 ms; each share ≤ 4.17 ms p99; R1-TICK-REC2 P1–P3 at 60 Hz; one run, same setup | TICK-60-HEADROOM (the advisor's reading, confirmed at the stop); GLOSSARY §8, the 60 Hz margin |
END

### FIX F3 (spike item 3)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
   both shares within 4.2 ms, and R1-TICK-REC2 P1–P3 at 60 Hz; how the sim load gets there is OQ-0001-2. A failure
AFTER:
   both shares within 4.17 ms, and R1-TICK-REC2 P1–P3 at 60 Hz; how the sim load gets there is OQ-0001-2. A failure
END

### FIX F8 (part 10)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
    carry-forward 4); development and preview servers use self-signed, pinned certificates valid at most 14 days, whose
    hash the page fetches from its own HTTPS origin (R2-TRN-REC5). *Why:* datagrams suit newest-wins data; both crates
AFTER:
    carry-forward 4); development and preview servers use self-signed, pinned certificates for WebTransport, valid at
    most 14 days, whose hash the page fetches from its own HTTPS origin (R2-TRN-REC5); the WebSocket fallback cannot
    pin a hash, since its constructor takes only a URL and sub-protocols (r2-netcode.md, Findings, R2-ROOM-F12, lines
    1494–1500), and presents the certificate ADR 0006's OQ-0006-5 names for development and staging. *Why:*
    datagrams suit newest-wins data; both crates
END

### FIX F8 (spike item 1)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
   - Development certificates (ADR 0006's): all four browsers connect with a pinned self-signed certificate in 20 of 20
     attempts, and reject one valid for 15 days (R2-TRN-REC5 marks 1 and 3).
AFTER:
   - Development certificates (ADR 0006's): all four browsers connect with a pinned self-signed certificate in 20 of 20
     attempts, and reject one valid for 15 days (R2-TRN-REC5 marks 1 and 3); the WebSocket leg first needs the
     certificate OQ-0006-5 names for development and staging (the Advisor, before the test starts; ADR 0006's
     PM-0006-15).
END

### FIX F10 (Context, Tick)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
  test shows at least 25 % headroom on the target hardware" (STATUS.md, "Decisions pending", lines 118–120), read as
AFTER:
  test shows at least 25 % headroom on the target hardware" (STATUS, TICK-30), read as
END

### FIX F10 (legacy)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
- GC1 (line 240, rewrite in Rust) and STATUS's ADR-001 inputs (STATUS.md line 199): the server must repeat the team
AFTER:
- GC1 (line 240, rewrite in Rust) and STATUS's ADR-001 inputs (STATUS, the ADR-001 entry): the server must repeat the team
END

### FIX F10 (Sources)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
- `docs/STATUS.md`, "Decisions pending" (lines 93–202): SLICE-ORDER, MODES-3, PREVIEW-WALL, SEC-SPREAD, ABL-PREDICT,
AFTER:
- `docs/STATUS.md`, "Decisions pending": SLICE-ORDER, MODES-3, PREVIEW-WALL, SEC-SPREAD, ABL-PREDICT,
END

### FIX F13 (event names)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
   at join and after a resync, goes over a reliable stream; deltas resume once the client acknowledges it. *Events:*
   tick-stamped gameplay events (hits, kills, deaths, purchases, captures, tier changes, pick results, drops) ride in the
AFTER:
   at join and after a resync, goes over a reliable stream; deltas resume once the client acknowledges it. *Events:*
   tick-stamped gameplay events (the events of ADR 0004 part 19: hits, deaths and kills, purchases, uplink cache
   events, tech tiers, upgrade picks and pickups) ride in the
END

### FIX F13 (the proof test)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
  [the netcode proof test](../research/GLOSSARY.md#the-netcode-proof-test) (GLOSSARY §8; brief §2).
AFTER:
  [the netcode proof test](../research/GLOSSARY.md#the-netcode-proof-test) (GLOSSARY §8; brief §2; below also "the proof test").
END

### FIX F14 (R5-TEST-OQ2)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
5. **What the proof test decides:** the WebTransport crate (R2-TRN-OQ5); the datagram cap from the measured
AFTER:
5. **What the proof test decides:** the WebTransport crate (R2-TRN-OQ5), and whether each candidate runs on a custom
   quinn endpoint so the network simulator's second layer sits under it (R5-TEST-OQ2); the datagram cap from the measured
END

### FIX F14 (OQ-0001-1 (b))
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
  the advisor's reading as ruled on 2026-10-31; (b) another reading of TICK-30's "25 % headroom". · *Decider:* Mustafa
AFTER:
  the advisor's reading as ruled on 2026-10-31; (b) not confirmed: Mustafa and the Advisor name the base, and part 2's
  mark and GLOSSARY §8 follow. · *Decider:* Mustafa
END
