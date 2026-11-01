# Fourth pass-mark fix round: test-engineer (pointers into the drafts, 2026-11-01)

Author: test-engineer, 19:28–19:55 Amsterdam. Every BEFORE was copied by script from docs/adr/0001-multiplayer-netcode.md at aa85bf0
(the working tree matches it), whole lines, unique. Each old pointer was resolved against the draft it was
written on (netcode-architect-adr0001-draft.md, whose lines it matches: e.g. its line 527 is the row "Server
tick time, 30 Hz (the baseline)", now line 533), then replaced by the part and the row, bullet or item it names.
No line number into ADR 0001 is kept. Research and GLOSSARY citations are untouched.

### NOTE (the other five drafts)
Their pass-mark sections were searched (every "line"/"lines" citation not tied to r1–r6, GLOSSARY or another
named file, and every "the draft …" phrase): none points into its own draft by line number. They point by part,
claim (S1–S22), spike item or row, which do not drift. No blocks for them.
END

### FIX NOTE B (key: the wrap)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
  (proposed: the draft's wrap row, lines 240–243 and 555, expects it only at 60 Hz). Portal changes are scripted at one
AFTER:
  (proposed: the draft's part 6 (the snapshot header's tick) and its "Failure modes" row "A tick counter wraps"
  expect it only at 60 Hz). Portal changes are scripted at one
END

### FIX NOTE B (PM-0001-4)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-4 | The server tick budget holds at 30 Hz (parts 2, 3) | [server tick time](../research/GLOSSARY.md#server-tick-time) p99 at most 16.67 ms; max under 33.33 ms; 0 overruns and 0 dropped time (the loop's own counters) | one vCPU of RS | SS at P2, S30; the server tick time measured directly on the tick thread, from the start of its work to its end (sim step, commands, lag-compensation queries, interest, encoding, sends) | 3 runs, each passes; V-S; rule B-S | the draft, budgets row (line 527); r2 R2-BUD-REC6 (lines 2411–2416) restated at 30 Hz; GLOSSARY, the server tick budget (lines 167–188); r1 R1-TICK-REC2 P1 (lines 2044–2053); R2-ROOM-REC2 mark 1 (lines 2775–2776: its 20 minutes fall inside S30) |
AFTER:
| PM-0001-4 | The server tick budget holds at 30 Hz (parts 2, 3) | [server tick time](../research/GLOSSARY.md#server-tick-time) p99 at most 16.67 ms; max under 33.33 ms; 0 overruns and 0 dropped time (the loop's own counters) | one vCPU of RS | SS at P2, S30; the server tick time measured directly on the tick thread, from the start of its work to its end (sim step, commands, lag-compensation queries, interest, encoding, sends) | 3 runs, each passes; V-S; rule B-S | the draft, the budgets table ("Budgets for the netcode proof test"), row "Server tick time, 30 Hz (the baseline)"; r2 R2-BUD-REC6 (lines 2411–2416) restated at 30 Hz; GLOSSARY, the server tick budget (lines 167–188); r1 R1-TICK-REC2 P1 (lines 2044–2053); R2-ROOM-REC2 mark 1 (lines 2775–2776: its 20 minutes fall inside S30) |
END

### FIX NOTE B (PM-0001-5)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-5 | Its two shares hold, each on its own (part 3) | sim step time p99 at most 8.33 ms; server-only work p99 at most 8.33 ms; each its own per-tick series, in the same runs as PM-0001-4; the sim step share's own parts are ADR 0004's rows | one vCPU of RS | per tick, the server-only work is command handling, the lag-compensation queries (timed inside the step), interest, encoding and the tick's sends; the sim step time is the step's elapsed time minus its lag-compensation queries (proposed: the queries run inside the step but count in the server-only share, part 3) | as PM-0001-4; rule B-S in each share | the draft, budgets row (line 528); r2 R2-BUD-REC6 (lines 2398–2401, 2413–2414), R2-SYNC-REC9 mark 1 (lines 2292–2296); GLOSSARY, the sim step share and the server-only share (lines 189–213) |
AFTER:
| PM-0001-5 | Its two shares hold, each on its own (part 3) | sim step time p99 at most 8.33 ms; server-only work p99 at most 8.33 ms; each its own per-tick series, in the same runs as PM-0001-4; the sim step share's own parts are ADR 0004's rows | one vCPU of RS | per tick, the server-only work is command handling, the lag-compensation queries (timed inside the step), interest, encoding and the tick's sends; the sim step time is the step's elapsed time minus its lag-compensation queries (proposed: the queries run inside the step but count in the server-only share, part 3) | as PM-0001-4; rule B-S in each share | the draft, the budgets table ("Budgets for the netcode proof test"), row "Its two shares, 30 Hz"; r2 R2-BUD-REC6 (lines 2398–2401, 2413–2414), R2-SYNC-REC9 mark 1 (lines 2292–2296); GLOSSARY, the sim step share and the server-only share (lines 189–213) |
END

### FIX NOTE B (PM-0001-7)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-7 | Lag compensation stays cheap and small (part 8) | a rewound shot against 48 candidates p99 at most 20 µs at 700 shots per second (700 ÷ 30 × 20 µs = 0.47 ms per tick, a planning check); the hit history for the stress scene's soldiers and big vehicles at most 1 MB | one vCPU of RS | each rewound-shot query timed in PM-0001-4's runs; the history's bytes read from its own capacity counters every minute | as PM-0001-4; rule B-S for the time. Bites: a history kept for the whole match (never trimmed) fails the 1 MB | r2 R2-SYNC-REC5 marks 3–4 (lines 2184–2187); the draft, part 8 (lines 274–275) |
AFTER:
| PM-0001-7 | Lag compensation stays cheap and small (part 8) | a rewound shot against 48 candidates p99 at most 20 µs at 700 shots per second (700 ÷ 30 × 20 µs = 0.47 ms per tick, a planning check); the hit history for the stress scene's soldiers and big vehicles at most 1 MB | one vCPU of RS | each rewound-shot query timed in PM-0001-4's runs; the history's bytes read from its own capacity counters every minute | as PM-0001-4; rule B-S for the time. Bites: a history kept for the whole match (never trimmed) fails the 1 MB | r2 R2-SYNC-REC5 marks 3–4 (lines 2184–2187); the draft, part 8 (the history: "at most 1 MB for the stress scene") |
END

### FIX NOTE B (PM-0001-8)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-8 | Interest picks under the cap, in time (part 9 (a), (d); part 6's cap) | the region query for 48 clients p99 at most 0.2 ms per tick; ranking and packing p99 at most 0.5 ms per tick; 0 snapshots over the per-client budget (256 kbps: 256,000 ÷ 8 ÷ 30 = 1,067 B per snapshot interval, counted at the UDP payload plus 28 B per datagram); the nearest 8 enemies in every snapshot; every relevant entity at least once in every 4 snapshots | one vCPU of RS | SS with its fight in the centre field (R2-INT-REC4's scene), PM-0001-4's runs; each phase timed per tick; the per-client checks from the packer's own log, by script | as PM-0001-4; rule B-S for the times. Bites: an accumulator that ranks by entity id fails "the nearest 8" | r2 R2-INT-REC1 (c) (lines 2440–2441); R2-INT-REC4 (lines 2497–2500); the draft, part 9 (d) (lines 294–295) and the cap (lines 223–224) |
AFTER:
| PM-0001-8 | Interest picks under the cap, in time (part 9 (a), (d); part 6's cap) | the region query for 48 clients p99 at most 0.2 ms per tick; ranking and packing p99 at most 0.5 ms per tick; 0 snapshots over the per-client budget (256 kbps: 256,000 ÷ 8 ÷ 30 = 1,067 B per snapshot interval, counted at the UDP payload plus 28 B per datagram); the nearest 8 enemies in every snapshot; every relevant entity at least once in every 4 snapshots | one vCPU of RS | SS with its fight in the centre field (R2-INT-REC4's scene), PM-0001-4's runs; each phase timed per tick; the per-client checks from the packer's own log, by script | as PM-0001-4; rule B-S for the times. Bites: an accumulator that ranks by entity id fails "the nearest 8" | r2 R2-INT-REC1 (c) (lines 2440–2441); R2-INT-REC4 (lines 2497–2500); the draft, part 9 (d) (the per-client priority accumulator) and part 6's per-client hard cap |
END

### FIX NOTE B (PM-0001-11)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-11 | The transport runtime fits beside the tick (part 10) | the transport runtime's threads use at most 1 vCPU (p99 of 1-second windows: GLOSSARY's no-statistic rule, read per second) with 48 sessions sending 30 input datagrams per second each and receiving 30 snapshots per second; p99 send call at most 1 ms | RS (the three vCPUs outside the tick's) | PM-0001-4's runs; the runtime threads' CPU time read every second; every `send_datagram` call timed | as PM-0001-4; rule B-S on the send call | r2 R2-TRN-REC1 mark 2 (lines 2589–2594; its 60 input datagrams per second are the 60 Hz form, 30 at TICK-30: the draft, line 532); GLOSSARY, transport runtime time (lines 130–140) |
AFTER:
| PM-0001-11 | The transport runtime fits beside the tick (part 10) | the transport runtime's threads use at most 1 vCPU (p99 of 1-second windows: GLOSSARY's no-statistic rule, read per second) with 48 sessions sending 30 input datagrams per second each and receiving 30 snapshots per second; p99 send call at most 1 ms | RS (the three vCPUs outside the tick's) | PM-0001-4's runs; the runtime threads' CPU time read every second; every `send_datagram` call timed | as PM-0001-4; rule B-S on the send call | r2 R2-TRN-REC1 mark 2 (lines 2589–2594; its 60 input datagrams per second are the 60 Hz form, 30 at TICK-30: the draft, the budgets table ("Budgets for the netcode proof test"), row "Transport runtime"); GLOSSARY, transport runtime time (lines 130–140) |
END

### FIX NOTE B (PM-0001-12)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-12 | The downstream fits per client (parts 6, 9) | mean at most 160 kbps; p99 of 1-second windows at most 256 kbps; every 1-second window at most 256 kbps (the hard cap; proposed reading of "hard cap"); 0 datagrams above the cap rule; counted per client at the server's send side, UDP payload plus 28 B per datagram. Reported beside it: the same counts in ES (the scene the research mark was set on) and, if LG reaches RS over IPv6, the IPv6 figure | RS (send-side counters) | SS at P2, everyone in view (PM-0001-2), PM-0001-4's runs; ES for 10 minutes (R2-BUD-REC2's own length) | as PM-0001-4; rule B-N | r2 R2-BUD-REC2 (lines 2339–2344); the draft, budgets row (line 533) and part 6 (lines 222–224); worked estimate (lines 1754–1779, 1784: +4.8 kbps for IPv6) |
AFTER:
| PM-0001-12 | The downstream fits per client (parts 6, 9) | mean at most 160 kbps; p99 of 1-second windows at most 256 kbps; every 1-second window at most 256 kbps (the hard cap; proposed reading of "hard cap"); 0 datagrams above the cap rule; counted per client at the server's send side, UDP payload plus 28 B per datagram. Reported beside it: the same counts in ES (the scene the research mark was set on) and, if LG reaches RS over IPv6, the IPv6 figure | RS (send-side counters) | SS at P2, everyone in view (PM-0001-2), PM-0001-4's runs; ES for 10 minutes (R2-BUD-REC2's own length) | as PM-0001-4; rule B-N | r2 R2-BUD-REC2 (lines 2339–2344); the draft, the budgets table ("Budgets for the netcode proof test"), row "Downstream per client", and part 6's per-client hard cap; worked estimate (lines 1754–1779, 1784: +4.8 kbps for IPv6) |
END

### FIX NOTE B (PM-0001-13)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-13 | The upstream fits per client (part 5; measures R2-XU-OQ4) | mean at most 48 kbps per client (IPv4), counted at the UDP payload plus 28 B per datagram (the mean: proposed reading); reported: the p99 of 1-second windows, the largest input datagram and the real command size | RS (receive-side counters) | P2, PM-0001-4's runs; every input datagram's size logged per client | as PM-0001-4; rule B-N | r2 R2-BUD-REC1 (lines 2320–2325), restated against P2 (part 13); the draft, lines 507–512 |
AFTER:
| PM-0001-13 | The upstream fits per client (part 5; measures R2-XU-OQ4) | mean at most 48 kbps per client (IPv4), counted at the UDP payload plus 28 B per datagram (the mean: proposed reading); reported: the p99 of 1-second windows, the largest input datagram and the real command size | RS (receive-side counters) | P2, PM-0001-4's runs; every input datagram's size logged per client | as PM-0001-4; rule B-N | r2 R2-BUD-REC1 (lines 2320–2325), restated against P2 (part 13); the draft, "Bandwidth estimate", the bullet "Upstream per client" |
END

### FIX NOTE B (PM-0001-14)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-14 | No datagram is ever too big (part 6) | 0 snapshot datagrams above the cap (min(the smallest `maxDatagramSize` of PM-0001-36, 1,150 B); 1,000 B until measured) and 0 above any browser's reported maximum, server and browser side; 0 fragmented IP packets | RS; LG; ML and MAC browsers | S30 at P2: every datagram's size checked against the cap at the send side; a packet capture on RS counts fragments (proposed tool) | every S30 and every browser session. Bites: an entity group that is never split | r2 R2-SYNC-REC1 mark 3 (line 2021); R2-BUD-REC3 (lines 2345–2348, 2358–2360); the draft, part 6 (lines 224–228) |
AFTER:
| PM-0001-14 | No datagram is ever too big (part 6) | 0 snapshot datagrams above the cap (min(the smallest `maxDatagramSize` of PM-0001-36, 1,150 B); 1,000 B until measured) and 0 above any browser's reported maximum, server and browser side; 0 fragmented IP packets | RS; LG; ML and MAC browsers | S30 at P2: every datagram's size checked against the cap at the send side; a packet capture on RS counts fragments (proposed tool) | every S30 and every browser session. Bites: an entity group that is never split | r2 R2-SYNC-REC1 mark 3 (line 2021); R2-BUD-REC3 (lines 2345–2348, 2358–2360); the draft, part 6 (*Datagrams:* the size cap, the split by entity groups, the first full state over a reliable stream) |
END

### FIX NOTE B (PM-0001-15)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-15 | A room's memory stays bounded and flat (parts 10, 11; decides R2-ROOM-OQ1 with PM-0001-16) | resident memory at most 64 MB at the start (all 48 clients joined) and after 20 minutes, and its peak over 1-second samples at most 64 MB (proposed: a ceiling read at two moments misses a peak between them); slope within 1 MB per 10 minutes after warm-up; every per-client queue (outbound, unacknowledged events) flat: its max over the last 10 minutes at most its max over the first 10 minutes after warm-up (proposed reading of "flat") | RS | S30 at P2; resident memory read every second; queue lengths from the server's counters | as PM-0001-4. Bites: R5-TEST-REC5's planted leak (one buffer kept per snapshot) is flagged within the run | r2 R2-ROOM-REC2 mark 3 (lines 2778–2779); R2-TRN-REC6 mark 1 (line 2742); r5 R5-TEST-REC5's mark (lines 1486–1489); the draft, budgets row (line 535) |
AFTER:
| PM-0001-15 | A room's memory stays bounded and flat (parts 10, 11; decides R2-ROOM-OQ1 with PM-0001-16) | resident memory at most 64 MB at the start (all 48 clients joined) and after 20 minutes, and its peak over 1-second samples at most 64 MB (proposed: a ceiling read at two moments misses a peak between them); slope within 1 MB per 10 minutes after warm-up; every per-client queue (outbound, unacknowledged events) flat: its max over the last 10 minutes at most its max over the first 10 minutes after warm-up (proposed reading of "flat") | RS | S30 at P2; resident memory read every second; queue lengths from the server's counters | as PM-0001-4. Bites: R5-TEST-REC5's planted leak (one buffer kept per snapshot) is flagged within the run | r2 R2-ROOM-REC2 mark 3 (lines 2778–2779); R2-TRN-REC6 mark 1 (line 2742); r5 R5-TEST-REC5's mark (lines 1486–1489); the draft, the budgets table ("Budgets for the netcode proof test"), row "Memory per room" |
END

### FIX NOTE B (PM-0001-16)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-16 | A room starts fast (part 11) | every room start, process spawn to Open, under 2 s (a plain limit) | RS and SH | every room start in the spike, at least 20 (proposed), timed by the supervisor's stand-in | every start. Bites: a start that loads its mode file twice over a 1 s delay | r2 R2-ROOM-REC2 mark 5 (line 2780); the draft, budgets row (line 536) |
AFTER:
| PM-0001-16 | A room starts fast (part 11) | every room start, process spawn to Open, under 2 s (a plain limit) | RS and SH | every room start in the spike, at least 20 (proposed), timed by the supervisor's stand-in | every start. Bites: a start that loads its mode file twice over a 1 s delay | r2 R2-ROOM-REC2 mark 5 (line 2780); the draft, the budgets table ("Budgets for the netcode proof test"), row "Room start-up" |
END

### FIX NOTE B (PM-0001-17)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-17 | Every decoded snapshot is the server's state (part 6) | 100 % of decoded snapshots rebuild the server's quantised state for that tick and that client's entity set, checked by [the client-held state's hash](../research/GLOSSARY.md#state-hash); also across the 16-bit tick wrap of every S30 | L1 (48 headless clients); L2: LG's 48 clients | 10 minutes each under P0, P2, P3 and B1; the server logs each client's quantised hash per tick, the client logs its decoded hash; compared by script | L1: 3 seeds per profile; L2: 1 run per profile; 0 mismatches in each. Bites: a 16-bit tick in the snapshot header (fails at the wrap); a client quantiser that rounds differently from the server's | r2 R2-SYNC-REC1 mark 1 (lines 2017–2019); the draft, part 6 (lines 240–243) |
AFTER:
| PM-0001-17 | Every decoded snapshot is the server's state (part 6) | 100 % of decoded snapshots rebuild the server's quantised state for that tick and that client's entity set, checked by [the client-held state's hash](../research/GLOSSARY.md#state-hash); also across the 16-bit tick wrap of every S30 | L1 (48 headless clients); L2: LG's 48 clients | 10 minutes each under P0, P2, P3 and B1; the server logs each client's quantised hash per tick, the client logs its decoded hash; compared by script | L1: 3 seeds per profile; L2: 1 run per profile; 0 mismatches in each. Bites: a 16-bit tick in the snapshot header (fails at the wrap); a client quantiser that rounds differently from the server's | r2 R2-SYNC-REC1 mark 1 (lines 2017–2019); the draft, part 6 (the snapshot header's 32-bit server tick and its 16-bit offsets) |
END

### FIX NOTE B (PM-0001-18)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-18 | A loss burst heals (part 6) | after B1's burst, every client decodes deltas again within 500 ms of the link coming back (a plain limit per client and burst) | L1; L2 with LG | B1 with 48 clients, 10 bursts per run (proposed) | L1: 3 seeds; L2: 1 run; every client after every burst | r2 R2-SYNC-REC1 mark 2 (line 2020); the draft, failure modes (line 542) |
AFTER:
| PM-0001-18 | A loss burst heals (part 6) | after B1's burst, every client decodes deltas again within 500 ms of the link coming back (a plain limit per client and burst) | L1; L2 with LG | B1 with 48 clients, 10 bursts per run (proposed) | L1: 3 seeds; L2: 1 run; every client after every burst | r2 R2-SYNC-REC1 mark 2 (line 2020); the draft, "Failure modes", row "A burst of total loss (B1)" |
END

### FIX NOTE B (PM-0001-19)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-19 | A client joining mid-match, or resyncing, has the whole world fast (parts 6, 11; the failure mode "a trap in the client's Wasm sim") | every join: a complete world (the client-held state's hash equal to the server's for that client) within 300 ms of sending `Join` at P2 (a plain limit); reported: p50, p99, max, and the time from the page's first connect call | L2: LG's clients and ML's Chrome | 100 joins into a running S30 room (proposed), including rejoins after a trap injected into the client's Worker (ADR 0004's S20 injects them; this row times them) | every join. Bites: the first full state sent as datagrams instead of on its stream | r2 R2-BUD-REC3 (line 2359: 100 ms RTT and 2 % loss, restated as P2), "the one join mark" (the draft, line 952; R2-XU-OQ6); the draft, failure modes (line 550) |
AFTER:
| PM-0001-19 | A client joining mid-match, or resyncing, has the whole world fast (parts 6, 11; the failure mode "a trap in the client's Wasm sim") | every join: a complete world (the client-held state's hash equal to the server's for that client) within 300 ms of sending `Join` at P2 (a plain limit); reported: p50, p99, max, and the time from the page's first connect call | L2: LG's clients and ML's Chrome | 100 joins into a running S30 room (proposed), including rejoins after a trap injected into the client's Worker (ADR 0004's S20 injects them; this row times them) | every join. Bites: the first full state sent as datagrams instead of on its stream | r2 R2-BUD-REC3 (line 2359: 100 ms RTT and 2 % loss, restated as P2), "the one join mark" (the draft, "What a spike must prove" item 2; R2-XU-OQ6); the draft, "Failure modes", row "A trap in the client's Wasm sim" |
END

### FIX NOTE B (PM-0001-21)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-21 | Forged commands and lag switches change nothing (parts 1, 5) | every forged command of R2-SYNC-REC2 mark 3's set rejected and counted, with 0 effect beyond the command's valid part (the state hash of a run with the forged set equals that of a run with only their valid parts); a lag-switch client (inputs held 200 ms under P1, then sent in one burst) moves no further in any tick than the catch-up cap allows | L1 | 1,000 forged commands per kind (proposed); 100 lag-switch bursts (proposed) | 3 seeds; 0 accepted in each. Bites: each check removed in turn lets its forged kind through | r2 R2-SYNC-REC2 marks 3–4 (lines 2060–2066); the draft, part 5 (lines 190–196) |
AFTER:
| PM-0001-21 | Forged commands and lag switches change nothing (parts 1, 5) | every forged command of R2-SYNC-REC2 mark 3's set rejected and counted, with 0 effect beyond the command's valid part (the state hash of a run with the forged set equals that of a run with only their valid parts); a lag-switch client (inputs held 200 ms under P1, then sent in one burst) moves no further in any tick than the catch-up cap allows | L1 | 1,000 forged commands per kind (proposed); 100 lag-switch bursts (proposed) | 3 seeds; 0 accepted in each. Bites: each check removed in turn lets its forged kind through | r2 R2-SYNC-REC2 marks 3–4 (lines 2060–2066); the draft, part 5 (the server's checks on every command) |
END

### FIX NOTE B (PM-0001-22)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-22 | A client that never acknowledges cannot grow the server (part 6) | its event queue held at the cap; the client resynced or dropped | L1 | one non-acknowledging client among 48 for 10 minutes | 3 seeds. Bites: an unbounded event queue | r2 R2-BUD-REC2 (lines 2342–2344); the draft, part 6 (lines 230–232) |
AFTER:
| PM-0001-22 | A client that never acknowledges cannot grow the server (part 6) | its event queue held at the cap; the client resynced or dropped | L1 | one non-acknowledging client among 48 for 10 minutes | 3 seeds. Bites: an unbounded event queue | r2 R2-BUD-REC2 (lines 2342–2344); the draft, part 6 (*Events:* the bounded queue of unacknowledged events) |
END

### FIX NOTE B (PM-0001-29)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-29 | The fallback takes over when QUIC is blocked (part 10; decides R2-TRN-OQ2) | with UDP blocked to the server, each of the four browsers reaches a playing WebSocket session at most 3 s after the page starts connecting (a plain limit; WebTransport timeout 1.5 s) | ML, MAC; RS with its UDP range closed | 20 attempts per browser (proposed) | every attempt. Bites: no WebTransport timeout (the attempt hangs) | r2 R2-TRN-REC3 mark 1 (lines 2664–2665); the draft, part 10 (line 312) |
AFTER:
| PM-0001-29 | The fallback takes over when QUIC is blocked (part 10; decides R2-TRN-OQ2) | with UDP blocked to the server, each of the four browsers reaches a playing WebSocket session at most 3 s after the page starts connecting (a plain limit; WebTransport timeout 1.5 s) | ML, MAC; RS with its UDP range closed | 20 attempts per browser (proposed) | every attempt. Bites: no WebTransport timeout (the attempt hangs) | r2 R2-TRN-REC3 mark 1 (lines 2664–2665); the draft, part 10 (WebTransport tried first with a timeout, then WebSocket) |
END

### FIX NOTE B (PM-0001-31)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-31 | The fallback's stalls stay short (part 10) | over WebSocket at P2: p99 extra snapshot delay at most 250 ms (extra delay: a snapshot's age when decoded minus the profile's one-way delay, 50 ms; proposed definition); the server never holds more than one unsent snapshot per client (its max, sampled every tick) | L2's TCP proxy on RS; LG; ML (Chrome) | 10 minutes at P2 | 1 run. Bites: a server that queues every snapshot over WebSocket | r2 R2-TRN-REC3 mark 3 (lines 2668–2669: "100 ms RTT and 1 % loss", restated as P2 by the draft, line 937) |
AFTER:
| PM-0001-31 | The fallback's stalls stay short (part 10) | over WebSocket at P2: p99 extra snapshot delay at most 250 ms (extra delay: a snapshot's age when decoded minus the profile's one-way delay, 50 ms; proposed definition); the server never holds more than one unsent snapshot per client (its max, sampled every tick) | L2's TCP proxy on RS; LG; ML (Chrome) | 10 minutes at P2 | 1 run. Bites: a server that queues every snapshot over WebSocket | r2 R2-TRN-REC3 mark 3 (lines 2668–2669: "100 ms RTT and 1 % loss", restated as P2 by the draft, "What a spike must prove" item 1) |
END

### FIX NOTE B (PM-0001-33)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-33 | The client's RTT agrees with the server's (parts 5, 8) | p99 of the absolute difference between app-level RTT and quinn's `rtt()`, sampled every second over 10 minutes, at most 5 ms at P2 (the draft's restatement); also recorded at a fixed 100 ms with no jitter or loss (R5-TEST-REC4's reading) | L2 on RS; ML (Chrome) | both estimates logged every second | 1 run per profile. Bites: pongs answered from the tick thread (up to one tick period late) | r2 R2-TRN-REC4 mark 2 (line 2694); the draft, line 940; r5 R5-TEST-REC4 (lines 1471–1472) |
AFTER:
| PM-0001-33 | The client's RTT agrees with the server's (parts 5, 8) | p99 of the absolute difference between app-level RTT and quinn's `rtt()`, sampled every second over 10 minutes, at most 5 ms at P2 (the draft's restatement); also recorded at a fixed 100 ms with no jitter or loss (R5-TEST-REC4's reading) | L2 on RS; ML (Chrome) | both estimates logged every second | 1 run per profile. Bites: pongs answered from the tick thread (up to one tick period late) | r2 R2-TRN-REC4 mark 2 (line 2694); the draft, "What a spike must prove" item 1; r5 R5-TEST-REC4 (lines 1471–1472) |
END

### FIX NOTE B (PM-0001-36)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-36 | Datagram size is measured per browser (part 6; decides R2-BUD-OQ3 = R2-TRN-OQ3 and R2-XU-OQ5) | `maxDatagramSize` recorded at session start and every 10 s in the four browsers: at least 1,000 B in all four on loopback and on a real EU path; otherwise the cap drops to the smallest value measured | ML, MAC; to a local server and to RS from an EU consumer connection (proposed: the path players use) | the page logs the value; 10 minutes per browser and path | 1 session per browser and path | r2 R2-TRN-REC2 mark 1 (lines 2631–2632); the draft, part 6 (lines 224–226) |
AFTER:
| PM-0001-36 | Datagram size is measured per browser (part 6; decides R2-BUD-OQ3 = R2-TRN-OQ3 and R2-XU-OQ5) | `maxDatagramSize` recorded at session start and every 10 s in the four browsers: at least 1,000 B in all four on loopback and on a real EU path; otherwise the cap drops to the smallest value measured | ML, MAC; to a local server and to RS from an EU consumer connection (proposed: the path players use) | the page logs the value; 10 minutes per browser and path | 1 session per browser and path | r2 R2-TRN-REC2 mark 1 (lines 2631–2632); the draft, part 6 (*Datagrams:* the size cap, `maxDatagramSize`) |
END

### FIX NOTE B (PM-0001-39)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-39 | The 60 Hz margin (part 2) | at a 60 Hz tick, every number from the same run: server tick time p99 at most 6.25 ms against [the server tick budget](../research/GLOSSARY.md#the-server-tick-budget) at 60 Hz, 8.33 ms (a 60 Hz margin of 1 − 6.25 ÷ 8.33 = 25 %); sim step time p99 and server-only work p99 at most 4.17 ms each; max under 16.67 ms; 0 overruns and 0 dropped time; encoding p99 at most 50 µs per client per snapshot; and PM-0001-17's hash check holding across tick 65,536 (reached at 18.2 minutes: 65,536 ÷ 60 ÷ 60) | one vCPU of RS | PM-0001-4's setup at a 60 Hz tick (SS at N = 48 with a snapshot-receiving client on every soldier, P2, S60, re-flood ticks at PM-0001-2's rate), with the sim OQ-0001-2 picks (recorded); the server tick time measured directly, never summed from its phases | 3 runs, each a whole measurement in itself (proposed reading of "in one run": every number of a run comes from that run, none is combined across runs), each passes; V-S; rule B-S | the draft, part 2 (lines 118–127), budgets row (line 530), spike item 3 (lines 965–967); GLOSSARY §8, [the 60 Hz margin](../research/GLOSSARY.md#the-60-hz-margin) (decision map J1); TICK-60-HEADROOM; r2 R2-BUD-REC6 (lines 2413–2415) |
AFTER:
| PM-0001-39 | The 60 Hz margin (part 2) | at a 60 Hz tick, every number from the same run: server tick time p99 at most 6.25 ms against [the server tick budget](../research/GLOSSARY.md#the-server-tick-budget) at 60 Hz, 8.33 ms (a 60 Hz margin of 1 − 6.25 ÷ 8.33 = 25 %); sim step time p99 and server-only work p99 at most 4.17 ms each; max under 16.67 ms; 0 overruns and 0 dropped time; encoding p99 at most 50 µs per client per snapshot; and PM-0001-17's hash check holding across tick 65,536 (reached at 18.2 minutes: 65,536 ÷ 60 ÷ 60) | one vCPU of RS | PM-0001-4's setup at a 60 Hz tick (SS at N = 48 with a snapshot-receiving client on every soldier, P2, S60, re-flood ticks at PM-0001-2's rate), with the sim OQ-0001-2 picks (recorded); the server tick time measured directly, never summed from its phases | 3 runs, each a whole measurement in itself (proposed reading of "in one run": every number of a run comes from that run, none is combined across runs), each passes; V-S; rule B-S | the draft, part 2 (the tick rate rule), the budgets table ("Budgets for the netcode proof test"), row "The 60 Hz margin run (decides 60 Hz)", and "What a spike must prove" item 3; GLOSSARY §8, [the 60 Hz margin](../research/GLOSSARY.md#the-60-hz-margin) (decision map J1); TICK-60-HEADROOM; r2 R2-BUD-REC6 (lines 2413–2415) |
END

### FIX NOTE B (PM-0001-40)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-40 | The research's other 60 Hz marks (part 2) | R1-TICK-REC2 P1 at 60 Hz (sim step time p99 at most 4.17 ms at N = 48, max at most 16.67 ms, 0 overruns, 36,000 ticks, 3 runs: ADR 0004's sim core spike row, run at 60 Hz); P2 at 60 Hz (the client's sim share, p99 at most 6.67 ms in Wasm in a dedicated Worker on ML while the page holds 30+ fps on Low: ADR 0004's row); P3 at 60 Hz (12 ticks of replay at most 1 ms p99 on ML: PM-0001-44's method). 60 Hz needs all three and PM-0001-39 | RS (P1); ML (P2, P3) | as each owning row says | as each owning row says | r1 R1-TICK-REC2 P1–P3 (lines 2044–2060); the draft, part 2 (lines 125–126) |
AFTER:
| PM-0001-40 | The research's other 60 Hz marks (part 2) | R1-TICK-REC2 P1 at 60 Hz (sim step time p99 at most 4.17 ms at N = 48, max at most 16.67 ms, 0 overruns, 36,000 ticks, 3 runs: ADR 0004's sim core spike row, run at 60 Hz); P2 at 60 Hz (the client's sim share, p99 at most 6.67 ms in Wasm in a dedicated Worker on ML while the page holds 30+ fps on Low: ADR 0004's row); P3 at 60 Hz (12 ticks of replay at most 1 ms p99 on ML: PM-0001-44's method). 60 Hz needs all three and PM-0001-39 | RS (P1); ML (P2, P3) | as each owning row says | as each owning row says | r1 R1-TICK-REC2 P1–P3 (lines 2044–2060); the draft, part 2 ("every 60 Hz mark the research sets, as written") |
END

### FIX NOTE B (PM-0001-41)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-41 | Prediction is exact when commands arrive in time (part 4) | on CO under P0 to P4: 0 corrections larger than 1 mm whenever every command arrived in time; every other correction counted and traced to a late or lost command (100 % traced) | L1 (the client endpoint's Rust code, headless); ML: Chrome over L2 | 10 minutes per profile; the Worker counts corrections from part 4's replay check and logs each command's arrival | L1: 3 seeds per profile; ML: 1 run at P2. Bites: the server's run speed 1 % above the client's; the server quantising its own copy differently from what it sends | r2 R2-SYNC-REC3 mark 1 (lines 2101–2102); the draft, part 4 (lines 165–174) |
AFTER:
| PM-0001-41 | Prediction is exact when commands arrive in time (part 4) | on CO under P0 to P4: 0 corrections larger than 1 mm whenever every command arrived in time; every other correction counted and traced to a late or lost command (100 % traced) | L1 (the client endpoint's Rust code, headless); ML: Chrome over L2 | 10 minutes per profile; the Worker counts corrections from part 4's replay check and logs each command's arrival | L1: 3 seeds per profile; ML: 1 run at P2. Bites: the server's run speed 1 % above the client's; the server quantising its own copy differently from what it sends | r2 R2-SYNC-REC3 mark 1 (lines 2101–2102); the draft, part 4 (the prediction scope and its replay) |
END

### FIX NOTE B (PM-0001-42)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-42 | Corrections stay small in a crowd (part 4) | under P3 in a brawl of 48: corrections larger than 5 cm in fewer than 1 % of snapshots, and the visible correction never moves the player more than 5 cm in one rendered frame below the snap threshold (2 m, proposed in the draft) | ML: Chrome over L2 with LG's 47 clients; L1 for the snapshot share | the local player in SS's fight, 10 minutes (proposed); the page logs the drawn correction per frame | L1: 3 seeds; ML: 1 run. Bites: smoothing off (a correction applied in one frame) | r2 R2-SYNC-REC3 mark 2 (lines 2103–2104); the draft, part 4 (lines 172–174) |
AFTER:
| PM-0001-42 | Corrections stay small in a crowd (part 4) | under P3 in a brawl of 48: corrections larger than 5 cm in fewer than 1 % of snapshots, and the visible correction never moves the player more than 5 cm in one rendered frame below the snap threshold (2 m, proposed in the draft) | ML: Chrome over L2 with LG's 47 clients; L1 for the snapshot share | the local player in SS's fight, 10 minutes (proposed); the page logs the drawn correction per frame | L1: 3 seeds; ML: 1 run. Bites: smoothing off (a correction applied in one frame) | r2 R2-SYNC-REC3 mark 2 (lines 2103–2104); the draft, part 4 (the correction's fade and snap) |
END

### FIX NOTE B (PM-0001-44)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-44 | The replay fits on the minimum laptop (part 4) | resetting and replaying the local soldier over 6 ticks (200 ms at 30 Hz) at most 1 ms p99; 12 ticks for the 60 Hz form (PM-0001-40) | ML: Chrome and Firefox (proposed: two Wasm engines), in Wasm in the Worker | in SS's fight at P2 for 10 minutes, the replay forced on every snapshot with exactly 6 ticks (proposed: with the skip on, too few and too easy replays are timed); `performance.now()` around reset and replay, isolation flag on; the vehicle case only if the sim has a piloted vehicle | 3 runs, each passes (18,000 replays each; p99 = the 17,820th). Bites: rule B-S applied to the replay (a busy-wait in it) | r1 R1-TICK-REC2 P3 (lines 2058–2060); the draft, part 4 (lines 170–171) and budgets row (line 531); r3 R3-MEASURE-F7 (lines 641–646) |
AFTER:
| PM-0001-44 | The replay fits on the minimum laptop (part 4) | resetting and replaying the local soldier over 6 ticks (200 ms at 30 Hz) at most 1 ms p99; 12 ticks for the 60 Hz form (PM-0001-40) | ML: Chrome and Firefox (proposed: two Wasm engines), in Wasm in the Worker | in SS's fight at P2 for 10 minutes, the replay forced on every snapshot with exactly 6 ticks (proposed: with the skip on, too few and too easy replays are timed); `performance.now()` around reset and replay, isolation flag on; the vehicle case only if the sim has a piloted vehicle | 3 runs, each passes (18,000 replays each; p99 = the 17,820th). Bites: rule B-S applied to the replay (a busy-wait in it) | r1 R1-TICK-REC2 P3 (lines 2058–2060); the draft, part 4 (the replay of 200 ms within 1 ms p99) and the budgets table ("Budgets for the netcode proof test"), row "Prediction replay"; r3 R3-MEASURE-F7 (lines 641–646) |
END

### FIX NOTE B (PM-0001-45)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-45 | Remote soldiers are drawn without guessing (part 7) | extrapolation in fewer than 1 % of rendered frames at P2 and 3 % at P3; under B1, no extrapolation beyond one snapshot interval (0 frames over it) | ML: Chrome over L2; L1 for the per-tick count | 10 minutes per profile; the Worker flags each block built by extrapolation, and the page counts frames drawn from flagged blocks | L1: 3 seeds; ML: 1 run per profile. Bites: R5-GATE-REC3's G11 mutation, "a snapshot buffer one slot too short for P2" (r5 line 1722) | r2 R2-SYNC-REC4 marks 1–2 (lines 2128–2129); the draft, part 7 (lines 253–258) |
AFTER:
| PM-0001-45 | Remote soldiers are drawn without guessing (part 7) | extrapolation in fewer than 1 % of rendered frames at P2 and 3 % at P3; under B1, no extrapolation beyond one snapshot interval (0 frames over it) | ML: Chrome over L2; L1 for the per-tick count | 10 minutes per profile; the Worker flags each block built by extrapolation, and the page counts frames drawn from flagged blocks | L1: 3 seeds; ML: 1 run per profile. Bites: R5-GATE-REC3's G11 mutation, "a snapshot buffer one slot too short for P2" (r5 line 1722) | r2 R2-SYNC-REC4 marks 1–2 (lines 2128–2129); the draft, part 7 (the interpolation delay and its range) |
END

### FIX NOTE B (PM-0001-46)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-46 | The delay adapts, and respawns never slide (part 7) | the adaptive delay settles within 5 s of a profile change ("settled": within ± half a snapshot interval of its final value, the mean of the next 30 s, and staying there; proposed); the delay stays within 2 to 5 intervals, and an announced change of more than one interval per second is clamped; 0 visible slides in 1,000 scripted respawns and teleports (a slide: a rendered frame that draws the entity on the line between its old and new place, more than 0.1 m from both; proposed) | ML: Chrome over L2 | 20 changes each way between P1 and P3 (proposed); 1,000 respawns and teleports | 1 run. Bites: respawns interpolated like movement | r2 R2-SYNC-REC4 marks 3–4 (lines 2130–2131); the draft, part 7 (lines 256–262) |
AFTER:
| PM-0001-46 | The delay adapts, and respawns never slide (part 7) | the adaptive delay settles within 5 s of a profile change ("settled": within ± half a snapshot interval of its final value, the mean of the next 30 s, and staying there; proposed); the delay stays within 2 to 5 intervals, and an announced change of more than one interval per second is clamped; 0 visible slides in 1,000 scripted respawns and teleports (a slide: a rendered frame that draws the entity on the line between its old and new place, more than 0.1 m from both; proposed) | ML: Chrome over L2 | 20 changes each way between P1 and P3 (proposed); 1,000 respawns and teleports | 1 run. Bites: respawns interpolated like movement | r2 R2-SYNC-REC4 marks 3–4 (lines 2130–2131); the draft, part 7 (the delay's adaptation, placed respawns and the announced delay) |
END

### FIX NOTE B (PM-0001-47)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-47 | The client's clock is right (parts 5, 7) | clock offset error at most 5 ms after 2 s of samples, against the simulator's true clock, at P2 (p99 over 1,000 connects with random true offsets; proposed count) | L1 (the client endpoint's Rust code) | each connect's estimate compared at 2 s with the simulator's clock | 3 seeds. Bites: an estimator that takes the newest raw sample | r2 R2-BUD-REC4 (lines 2378–2380), restated at P2 (part 13); the draft, part 5 (lines 196–199) |
AFTER:
| PM-0001-47 | The client's clock is right (parts 5, 7) | clock offset error at most 5 ms after 2 s of samples, against the simulator's true clock, at P2 (p99 over 1,000 connects with random true offsets; proposed count) | L1 (the client endpoint's Rust code) | each connect's estimate compared at 2 s with the simulator's clock | 3 seeds. Bites: an estimator that takes the newest raw sample | r2 R2-BUD-REC4 (lines 2378–2380), restated at P2 (part 13); the draft, part 5 (*Time:* the clock estimate and the input buffer) |
END

### FIX NOTE B (PM-0001-48)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-48 | Hits register as the shooter saw them (part 8; decides OQ-0001-7) | a scripted shooter at TC's sprinting target under P0 to P3: of the shots whose ray hits the target's volume in the shooter's view (the state the Worker published for the command's view time), at least 99 % register on the server up to P2 and 95 % at P3; false hits (a miss in the view counted as a hit) below 0.5 % | L1; L2 with LG's shooter | 2,000 shots per profile (proposed: about 20 misses at the 99 % mark, so a handful of shots does not decide the run) | L1: 3 seeds; L2: 1 run per profile. Bites: a rewind that leaves out the interpolation delay (at 9 m/s, 100 ms is about 0.9 m) | r2 R2-SYNC-REC5 mark 1 (lines 2179–2181); the draft, OQ-0001-7 (lines 881–885) |
AFTER:
| PM-0001-48 | Hits register as the shooter saw them (part 8; decides OQ-0001-7) | a scripted shooter at TC's sprinting target under P0 to P3: of the shots whose ray hits the target's volume in the shooter's view (the state the Worker published for the command's view time), at least 99 % register on the server up to P2 and 95 % at P3; false hits (a miss in the view counted as a hit) below 0.5 % | L1; L2 with LG's shooter | 2,000 shots per profile (proposed: about 20 misses at the 99 % mark, so a handful of shots does not decide the run) | L1: 3 seeds; L2: 1 run per profile. Bites: a rewind that leaves out the interpolation delay (at 9 m/s, 100 ms is about 0.9 m) | r2 R2-SYNC-REC5 mark 1 (lines 2179–2181); the draft, OQ-0001-7 |
END

### FIX NOTE B (PM-0001-49)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-49 | The rewind cap holds; what each cap clips is known (part 8; feeds OQ-0001-5; decides R2-XU-OQ2) | 100 % of shots resolved with a rewind at most the cap, honest and cheat clients, P0 to P4; per candidate cap (200 and 250 ms) and profile, the share of shots the cap clips recorded; the rewind estimate: how far behind the server's present the client's view is when its command arrives (median, p99) per profile | L1; L2 with LG | PM-0001-48's shots, run once per cap | as PM-0001-48. Bites: the cap applied to the estimate only, not to the final rewind | r2 R2-SYNC-REC5 mark 6 (lines 2191–2192), the cap's text (lines 2154–2159); R2-BUD-REC1 (lines 2320–2323); the draft, part 8 (lines 273–280) |
AFTER:
| PM-0001-49 | The rewind cap holds; what each cap clips is known (part 8; feeds OQ-0001-5; decides R2-XU-OQ2) | 100 % of shots resolved with a rewind at most the cap, honest and cheat clients, P0 to P4; per candidate cap (200 and 250 ms) and profile, the share of shots the cap clips recorded; the rewind estimate: how far behind the server's present the client's view is when its command arrives (median, p99) per profile | L1; L2 with LG | PM-0001-48's shots, run once per cap | as PM-0001-48. Bites: the cap applied to the estimate only, not to the final rewind | r2 R2-SYNC-REC5 mark 6 (lines 2191–2192), the cap's text (lines 2154–2159); R2-BUD-REC1 (lines 2320–2323); the draft, part 8 (the rewind cap and the share it clips) |
END

### FIX NOTE B (PM-0001-50)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-50 | A cheat cannot buy rewind (part 8) | a scripted cheat client that announces the largest allowed delay and stamps its view time at the tolerance's edge gains no more rewind than the tolerance (at most one snapshot interval, 33.33 ms) over an honest client at the same RTT, under P0 and P1 | L1 | 1,000 shots per profile and client | 3 seeds. Bites: the view time accepted without the tolerance check | r2 R2-SYNC-REC5 mark 5 (lines 2188–2190); the draft, part 8 (lines 270–273) |
AFTER:
| PM-0001-50 | A cheat cannot buy rewind (part 8) | a scripted cheat client that announces the largest allowed delay and stamps its view time at the tolerance's edge gains no more rewind than the tolerance (at most one snapshot interval, 33.33 ms) over an honest client at the same RTT, under P0 and P1 | L1 | 1,000 shots per profile and client | 3 seeds. Bites: the view time accepted without the tolerance check | r2 R2-SYNC-REC5 mark 5 (lines 2188–2190); the draft, part 8 (the view-time tolerance) |
END

### FIX NOTE B (PM-0001-51)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-51 | Aiming at height hits the clicked point (part 5) | at TC's static targets on the ground, at 2.5 m and at 4 m, under P0 to P3: the server's hit point within 5 cm of the client-picked point in at least 99.9 % of shots; the aim at most 80 bits per command before delta coding | L1; ML: Chrome over L2 at P2 | 1,000 shots per height and profile (proposed: one miss allowed per 1,000); the aim's bits read from the codec's field widths and checked on 1,000 encoded commands | L1: 3 seeds; ML: 1 run. Bites: the server's ray built parallel to the client's (a direction) with a planted 10 cm fire-origin error | r2 R2-SYNC-REC8 marks 1–2 (lines 2272–2274); the draft, part 5 (lines 201–207) |
AFTER:
| PM-0001-51 | Aiming at height hits the clicked point (part 5) | at TC's static targets on the ground, at 2.5 m and at 4 m, under P0 to P3: the server's hit point within 5 cm of the client-picked point in at least 99.9 % of shots; the aim at most 80 bits per command before delta coding | L1; ML: Chrome over L2 at P2 | 1,000 shots per height and profile (proposed: one miss allowed per 1,000); the aim's bits read from the codec's field widths and checked on 1,000 encoded commands | L1: 3 seeds; ML: 1 run. Bites: the server's ray built parallel to the client's (a direction) with a planted 10 cm fire-origin error | r2 R2-SYNC-REC8 marks 1–2 (lines 2272–2274); the draft, part 5 (*The aim point:*) |
END

### FIX NOTE B (PM-0001-52)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-52 | Hit feedback waits for the server (part 4; LOOP_V2 §7) | 0 damage numbers or kills on another soldier shown before the server's event for it | ML: Chrome over L2; MP (in the checklist session) | the page logs each shown number and kill with its event id and the time the Worker received that event; checked by script over every browser session of the spike and 1,000 scripted hits (proposed) | every session. Bites: damage shown on the client's own predicted hit | r1 R1-ABL-REC4's mark (lines 2440–2445); the draft, part 4 (lines 174–175) |
AFTER:
| PM-0001-52 | Hit feedback waits for the server (part 4; LOOP_V2 §7) | 0 damage numbers or kills on another soldier shown before the server's event for it | ML: Chrome over L2; MP (in the checklist session) | the page logs each shown number and kill with its event id and the time the Worker received that event; checked by script over every browser session of the spike and 1,000 scripted hits (proposed) | every session. Bites: damage shown on the client's own predicted hit | r1 R1-ABL-REC4's mark (lines 2440–2445); the draft, part 4 ("Never predicted") |
END

### FIX NOTE B (PM-0001-53)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-53 | Mustafa's verdict: prediction, reconciliation and lag-compensated hits look correct at P2 (parts 4, 7, 8) | every step of the netcode checklist below judged "achieved"; 0 left "not yet" | MP, Brave on Windows | the netcode checklist below, at the profile OQ-0001-3 confirms (P2 until then), beside the measurable marks PM-0001-41 to PM-0001-52 | one session; a step marked "not yet" goes back to the draft and is shown again | GLOSSARY §8, the netcode proof test (brief §2); plan rule 12; the draft, spike item 4 (lines 968–969) |
AFTER:
| PM-0001-53 | Mustafa's verdict: prediction, reconciliation and lag-compensated hits look correct at P2 (parts 4, 7, 8) | every step of the netcode checklist below judged "achieved"; 0 left "not yet" | MP, Brave on Windows | the netcode checklist below, at the profile OQ-0001-3 confirms (P2 until then), beside the measurable marks PM-0001-41 to PM-0001-52 | one session; a step marked "not yet" goes back to the draft and is shown again | GLOSSARY §8, the netcode proof test (brief §2); plan rule 12; the draft, "What a spike must prove" item 4 |
END

### FIX NOTE B (PM-0001-54)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-54 | Client-side projectile flight holds, or E3b replaces it (part 6; conditional) | only if live projectiles are in the load: 0 desyncs of client-side flight against the server over 1,000 throws; with any desync, E3b is used and PM-0001-12 runs again with it | L1; L2 with LG | 1,000 scripted throws, each landing compared with the server's | 3 seeds. Bites: a client trajectory step of 1/60 s against the server's 1/30 s | r2 R2-BUD-REC2 (line 2342); the draft, spike item 5 (lines 986–987) |
AFTER:
| PM-0001-54 | Client-side projectile flight holds, or E3b replaces it (part 6; conditional) | only if live projectiles are in the load: 0 desyncs of client-side flight against the server over 1,000 throws; with any desync, E3b is used and PM-0001-12 runs again with it | L1; L2 with LG | 1,000 scripted throws, each landing compared with the server's | 3 seeds. Bites: a client trajectory step of 1/60 s against the server's 1/30 s | r2 R2-BUD-REC2 (line 2342); the draft, "What a spike must prove" item 5 (E3 or E3b) |
END

### FIX NOTE B (PM-0001-56)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-56 | The proof test's records are complete (part 14, item 5) | recorded: both crates' results on PM-0001-11, -25, -28, -35 and their pick, with its reason (R2-TRN-OQ5: the draft names no rule beyond the marks); each browser's negotiated draft and its behaviour with UDP blocked (R2-TRN-OQ1, OQ2); the real record sizes per entity kind and the command size (R2-XU-OQ15, OQ4); NewReno against Cubic (PM-0001-24) and each browser's received snapshot rate and age at P2 to P4 (R2-BUD-OQ10, R2-TRN-OQ8); the smoothness of a remote soldier sprinting straight and turning, drawn with linear interpolation (the standard deviation of its per-frame displacement as a share of the mean, R1-TICK-REC5's measure) beside Mustafa's step 5 (R2-SYNC-OQ12); OQ-0001-6's two command-timing options, each run with PM-0001-42's correction share and PM-0001-48's registration at P2 to P4; Brave's transport and version on MP | as each row names | read once per item and written to the results log | passes when every item has a number or a value | the draft, spike item 5 (lines 985–993); r1 R1-TICK-REC5's measure (lines 2111–2112); OQ-0001-6 (lines 874–880) |
AFTER:
| PM-0001-56 | The proof test's records are complete (part 14, item 5) | recorded: both crates' results on PM-0001-11, -25, -28, -35 and their pick, with its reason (R2-TRN-OQ5: the draft names no rule beyond the marks); each browser's negotiated draft and its behaviour with UDP blocked (R2-TRN-OQ1, OQ2); the real record sizes per entity kind and the command size (R2-XU-OQ15, OQ4); NewReno against Cubic (PM-0001-24) and each browser's received snapshot rate and age at P2 to P4 (R2-BUD-OQ10, R2-TRN-OQ8); the smoothness of a remote soldier sprinting straight and turning, drawn with linear interpolation (the standard deviation of its per-frame displacement as a share of the mean, R1-TICK-REC5's measure) beside Mustafa's step 5 (R2-SYNC-OQ12); OQ-0001-6's two command-timing options, each run with PM-0001-42's correction share and PM-0001-48's registration at P2 to P4; Brave's transport and version on MP | as each row names | read once per item and written to the results log | passes when every item has a number or a value | the draft, "What a spike must prove" item 5; r1 R1-TICK-REC5's measure (lines 2111–2112); OQ-0001-6 |
END
