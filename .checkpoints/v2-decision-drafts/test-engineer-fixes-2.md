# Second pass-mark fix round: test-engineer (plan step 9, after the consistency pass)

Author: test-engineer, 2026-11-01, resumed 19:07 Amsterdam. Against the docs/adr files at c05fc15 (after 8eb2f67
and b375829). Every BEFORE was copied from its file by script (whole lines) and is unique in it; each file's
blocks were dry-applied in this order to scratch copies with scripts/apply_fixes.py (the FILE line picks the
file), and none overlaps another. Only test-engineer's pass-mark sections are touched (the tables, their keys,
their notes and Mustafa's checklists). No fix changes a decision.

What each finding got:
- F3 (two decimals): every tick share and tick period written as a pass-mark number now has two decimals (8.33,
  16.67, 33.33, 13.33, 4.17, 6.67). Left at one decimal on purpose: R3's frame-time budgets (p95 at most 33.3 and
  16.7 ms, ADR 0003 and PM-0004-8, the research's own numbers for frames, not tick shares) and the display refresh
  periods in PM-0002-8 and PM-0002-11.
- F7: PM-0004-73 (S21, R2-SYNC-REC6 mark 1) and PM-0004-74 (S22, R5-LAYOUT-REC4), placed after PM-0004-61 and
  numbered after the last row so no existing number moves. SPIKE_PLAN:204-207 can cite them (the synthesis).
- F13: PM-0005-32 "4 active effects per soldier"; PM-0006-13 "8 rooms at N = 48"; PM-0006-17 keeps R2's symbol and
  word, "V = 15 vehicles" (R2's vehicles row, r2:1762, counts a harvester per team, so "big vehicles" would not be
  R2's count); "moving" is dropped, since it was not R2's word.
- PM-0002-42: the `erasableSyntaxOnly` plant becomes a parameter property, the `enum` recorded, as in PM-0002-17.
- R2-BUD-REC2's restated precision mark: a new row, PM-0001-57. PM-0001-17 does not cover it: its hash proves the
  decoded values equal the server's quantised ones, not how far quantising moved them.
- F4 (MAC, handed over by rust-reviewer and graphics-engineer): the keys in 0003, 0004 and 0005 take SPIKE_PLAN's
  machine with the version rule (WebGPU from 26.0, WebTransport from 26.4); rows say "MAC: Safari". 0005's key was
  given by the critic to the 0005 frame author: drop that block if engine-architect sends one too.
- F2-PMtext: rust-reviewer's held block, taken over as written (the BEFORE re-copied).
- graphics-engineer's notes for 0003: step 24 cites the README decision "fragments as damage" (the row the
  synthesis adds; 0003:247-248 already cites it); step 29 adds the camera-player cutaway (OQ-0003-15).

### FIX F3 (0001 key, the frame-based tolerance)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
  is a frame-time budget, and the one frame-based tolerance (PM-0001-43, one tick = 33.3 ms) is two refresh periods, so
AFTER:
  is a frame-time budget, and the one frame-based tolerance (PM-0001-43, one tick = 33.33 ms) is two refresh periods, so
END

### FIX F3 (PM-0001-4)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-4 | The server tick budget holds at 30 Hz (parts 2, 3) | [server tick time](../research/GLOSSARY.md#server-tick-time) p99 at most 16.7 ms; max under 33.3 ms; 0 overruns and 0 dropped time (the loop's own counters) | one vCPU of RS | SS at P2, S30; the server tick time measured directly on the tick thread, from the start of its work to its end (sim step, commands, lag-compensation queries, interest, encoding, sends) | 3 runs, each passes; V-S; rule B-S | the draft, budgets row (line 527); r2 R2-BUD-REC6 (lines 2411–2416) restated at 30 Hz; GLOSSARY, the server tick budget (lines 167–188); r1 R1-TICK-REC2 P1 (lines 2044–2053); R2-ROOM-REC2 mark 1 (lines 2775–2776: its 20 minutes fall inside S30) |
AFTER:
| PM-0001-4 | The server tick budget holds at 30 Hz (parts 2, 3) | [server tick time](../research/GLOSSARY.md#server-tick-time) p99 at most 16.67 ms; max under 33.33 ms; 0 overruns and 0 dropped time (the loop's own counters) | one vCPU of RS | SS at P2, S30; the server tick time measured directly on the tick thread, from the start of its work to its end (sim step, commands, lag-compensation queries, interest, encoding, sends) | 3 runs, each passes; V-S; rule B-S | the draft, budgets row (line 527); r2 R2-BUD-REC6 (lines 2411–2416) restated at 30 Hz; GLOSSARY, the server tick budget (lines 167–188); r1 R1-TICK-REC2 P1 (lines 2044–2053); R2-ROOM-REC2 mark 1 (lines 2775–2776: its 20 minutes fall inside S30) |
END

### FIX F3 (PM-0001-5)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-5 | Its two shares hold, each on its own (part 3) | sim step time p99 at most 8.3 ms; server-only work p99 at most 8.3 ms; each its own per-tick series, in the same runs as PM-0001-4; the sim step share's own parts are ADR 0004's rows | one vCPU of RS | per tick, the server-only work is command handling, the lag-compensation queries (timed inside the step), interest, encoding and the tick's sends; the sim step time is the step's elapsed time minus its lag-compensation queries (proposed: the queries run inside the step but count in the server-only share, part 3) | as PM-0001-4; rule B-S in each share | the draft, budgets row (line 528); r2 R2-BUD-REC6 (lines 2398–2401, 2413–2414), R2-SYNC-REC9 mark 1 (lines 2292–2296); GLOSSARY, the sim step share and the server-only share (lines 189–213) |
AFTER:
| PM-0001-5 | Its two shares hold, each on its own (part 3) | sim step time p99 at most 8.33 ms; server-only work p99 at most 8.33 ms; each its own per-tick series, in the same runs as PM-0001-4; the sim step share's own parts are ADR 0004's rows | one vCPU of RS | per tick, the server-only work is command handling, the lag-compensation queries (timed inside the step), interest, encoding and the tick's sends; the sim step time is the step's elapsed time minus its lag-compensation queries (proposed: the queries run inside the step but count in the server-only share, part 3) | as PM-0001-4; rule B-S in each share | the draft, budgets row (line 528); r2 R2-BUD-REC6 (lines 2398–2401, 2413–2414), R2-SYNC-REC9 mark 1 (lines 2292–2296); GLOSSARY, the sim step share and the server-only share (lines 189–213) |
END

### FIX F3 (PM-0001-39)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-39 | The 60 Hz margin (part 2) | at a 60 Hz tick, every number from the same run: server tick time p99 at most 6.25 ms against [the server tick budget](../research/GLOSSARY.md#the-server-tick-budget) at 60 Hz, 8.33 ms (a 60 Hz margin of 1 − 6.25 ÷ 8.33 = 25 %); sim step time p99 and server-only work p99 at most 4.2 ms each; max under 16.7 ms; 0 overruns and 0 dropped time; encoding p99 at most 50 µs per client per snapshot; and PM-0001-17's hash check holding across tick 65,536 (reached at 18.2 minutes: 65,536 ÷ 60 ÷ 60) | one vCPU of RS | PM-0001-4's setup at a 60 Hz tick (SS at N = 48 with a snapshot-receiving client on every soldier, P2, S60, re-flood ticks at PM-0001-2's rate), with the sim OQ-0001-2 picks (recorded); the server tick time measured directly, never summed from its phases | 3 runs, each a whole measurement in itself (proposed reading of "in one run": every number of a run comes from that run, none is combined across runs), each passes; V-S; rule B-S | the draft, part 2 (lines 118–127), budgets row (line 530), spike item 3 (lines 965–967); GLOSSARY §8, [the 60 Hz margin](../research/GLOSSARY.md#the-60-hz-margin) (decision map J1); TICK-60-HEADROOM; r2 R2-BUD-REC6 (lines 2413–2415) |
AFTER:
| PM-0001-39 | The 60 Hz margin (part 2) | at a 60 Hz tick, every number from the same run: server tick time p99 at most 6.25 ms against [the server tick budget](../research/GLOSSARY.md#the-server-tick-budget) at 60 Hz, 8.33 ms (a 60 Hz margin of 1 − 6.25 ÷ 8.33 = 25 %); sim step time p99 and server-only work p99 at most 4.17 ms each; max under 16.67 ms; 0 overruns and 0 dropped time; encoding p99 at most 50 µs per client per snapshot; and PM-0001-17's hash check holding across tick 65,536 (reached at 18.2 minutes: 65,536 ÷ 60 ÷ 60) | one vCPU of RS | PM-0001-4's setup at a 60 Hz tick (SS at N = 48 with a snapshot-receiving client on every soldier, P2, S60, re-flood ticks at PM-0001-2's rate), with the sim OQ-0001-2 picks (recorded); the server tick time measured directly, never summed from its phases | 3 runs, each a whole measurement in itself (proposed reading of "in one run": every number of a run comes from that run, none is combined across runs), each passes; V-S; rule B-S | the draft, part 2 (lines 118–127), budgets row (line 530), spike item 3 (lines 965–967); GLOSSARY §8, [the 60 Hz margin](../research/GLOSSARY.md#the-60-hz-margin) (decision map J1); TICK-60-HEADROOM; r2 R2-BUD-REC6 (lines 2413–2415) |
END

### FIX F3 (PM-0001-40)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-40 | The research's other 60 Hz marks (part 2) | R1-TICK-REC2 P1 at 60 Hz (sim step time p99 at most 4.2 ms at N = 48, max at most 16.7 ms, 0 overruns, 36,000 ticks, 3 runs: ADR 0004's sim core spike row, run at 60 Hz); P2 at 60 Hz (the client's sim share, p99 at most 6.7 ms in Wasm in a dedicated Worker on ML while the page holds 30+ fps on Low: ADR 0004's row); P3 at 60 Hz (12 ticks of replay at most 1 ms p99 on ML: PM-0001-44's method). 60 Hz needs all three and PM-0001-39 | RS (P1); ML (P2, P3) | as each owning row says | as each owning row says | r1 R1-TICK-REC2 P1–P3 (lines 2044–2060); the draft, part 2 (lines 125–126) |
AFTER:
| PM-0001-40 | The research's other 60 Hz marks (part 2) | R1-TICK-REC2 P1 at 60 Hz (sim step time p99 at most 4.17 ms at N = 48, max at most 16.67 ms, 0 overruns, 36,000 ticks, 3 runs: ADR 0004's sim core spike row, run at 60 Hz); P2 at 60 Hz (the client's sim share, p99 at most 6.67 ms in Wasm in a dedicated Worker on ML while the page holds 30+ fps on Low: ADR 0004's row); P3 at 60 Hz (12 ticks of replay at most 1 ms p99 on ML: PM-0001-44's method). 60 Hz needs all three and PM-0001-39 | RS (P1); ML (P2, P3) | as each owning row says | as each owning row says | r1 R1-TICK-REC2 P1–P3 (lines 2044–2060); the draft, part 2 (lines 125–126) |
END

### FIX F3 (PM-0001-43)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-43 | Own movement and own shots feel the same at any latency (part 4) | input-to-screen latency for own movement and for the muzzle flash: p99 at P4 minus p99 at P0 at most 33.3 ms (one tick), and the same for the medians (proposed: "the same" read on both) | ML and RD: Chrome on WebGPU, presented frames on a 60 Hz display | 200 scripted inputs per profile and kind (proposed), sent through the Chrome DevTools Protocol; latency from the input event's time stamp to the first frame that draws its effect (the animation-frame time stamp); isolation flag on | 1 run per machine, profile and kind. Bites: own movement drawn from snapshots only (no prediction) | r2 R2-SYNC-REC3 mark 3 (line 2105) |
AFTER:
| PM-0001-43 | Own movement and own shots feel the same at any latency (part 4) | input-to-screen latency for own movement and for the muzzle flash: p99 at P4 minus p99 at P0 at most 33.33 ms (one tick), and the same for the medians (proposed: "the same" read on both) | ML and RD: Chrome on WebGPU, presented frames on a 60 Hz display | 200 scripted inputs per profile and kind (proposed), sent through the Chrome DevTools Protocol; latency from the input event's time stamp to the first frame that draws its effect (the animation-frame time stamp); isolation flag on | 1 run per machine, profile and kind. Bites: own movement drawn from snapshots only (no prediction) | r2 R2-SYNC-REC3 mark 3 (line 2105) |
END

### FIX F3 (PM-0001-50)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-50 | A cheat cannot buy rewind (part 8) | a scripted cheat client that announces the largest allowed delay and stamps its view time at the tolerance's edge gains no more rewind than the tolerance (at most one snapshot interval, 33.3 ms) over an honest client at the same RTT, under P0 and P1 | L1 | 1,000 shots per profile and client | 3 seeds. Bites: the view time accepted without the tolerance check | r2 R2-SYNC-REC5 mark 5 (lines 2188–2190); the draft, part 8 (lines 270–273) |
AFTER:
| PM-0001-50 | A cheat cannot buy rewind (part 8) | a scripted cheat client that announces the largest allowed delay and stamps its view time at the tolerance's edge gains no more rewind than the tolerance (at most one snapshot interval, 33.33 ms) over an honest client at the same RTT, under P0 and P1 | L1 | 1,000 shots per profile and client | 3 seeds. Bites: the view time accepted without the tolerance check | r2 R2-SYNC-REC5 mark 5 (lines 2188–2190); the draft, part 8 (lines 270–273) |
END

### FIX F3 (0002 key, BX-P)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
  padded by a busy-wait to 13.3 ms per tick, [the client's sim share](../research/GLOSSARY.md#the-clients-sim-share) at 30 Hz, so the page is tested against the most
AFTER:
  padded by a busy-wait to 13.33 ms per tick, [the client's sim share](../research/GLOSSARY.md#the-clients-sim-share) at 30 Hz, so the page is tested against the most
END

### FIX F3 (PM-0002-12)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
| PM-0002-12 | Presses are never lost or doubled (part 5: one command per tick, C1) | 0 lost and 0 doubled presses, and 0 lost and 0 doubled releases, over 10,000 scripted presses, some shorter than one tick | CI: Chromium, Firefox, WebKit under Playwright (counts) | a seeded shim on the page's posting side delays each command by a time drawn uniformly from 0 to 100 ms and sends 10 % of commands twice, which reorders most neighbours (proposed: a spread wider than one 33.3 ms tick; R1 gives no amounts); the shim logs how many it delayed, doubled and reordered (each above 0, or the run is invalid); the Worker counts the presses and releases it applies through the edge counters. No profile: the network's own press mark is ADR 0001's (R2-SYNC-REC2 at P3) | 1 run per engine (counts). Bites: a Worker that reads only the held-button bitmask and ignores the edge counters | R1-BND-REC3 mark 1 (r1, 2292–2294) |
AFTER:
| PM-0002-12 | Presses are never lost or doubled (part 5: one command per tick, C1) | 0 lost and 0 doubled presses, and 0 lost and 0 doubled releases, over 10,000 scripted presses, some shorter than one tick | CI: Chromium, Firefox, WebKit under Playwright (counts) | a seeded shim on the page's posting side delays each command by a time drawn uniformly from 0 to 100 ms and sends 10 % of commands twice, which reorders most neighbours (proposed: a spread wider than one 33.33 ms tick; R1 gives no amounts); the shim logs how many it delayed, doubled and reordered (each above 0, or the run is invalid); the Worker counts the presses and releases it applies through the edge counters. No profile: the network's own press mark is ADR 0001's (R2-SYNC-REC2 at P3) | 1 run per engine (counts). Bites: a Worker that reads only the held-button bitmask and ignores the edge counters | R1-BND-REC3 mark 1 (r1, 2292–2294) |
END

### FIX F3 (PM-0002-13)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
| PM-0002-13 | A press reaches the sim quickly in local play (part 5) | from the key event's `timeStamp` to the start of the tick that applies it, at most one tick plus one frame, 33.3 + 16.7 = 50.0 ms, at p95 (the 9,500th of 10,000) | ML on its 60 Hz panel: Chrome, Edge, Firefox; MAC: Safari (reported) | local mode, BX at N = 48, mode P, the page loaded as in PM-0002-8; 10,000 key events sent through Playwright, headed, so `timeStamp` is the event's own (Safari through safaridriver); the Worker stamps each tick's start on the shared clock | 1 run per browser. Bites: commands posted only from the frame callback plus one extra tick of buffering in the Worker | R1-BND-REC3 mark 2 (r1, 2294–2295); TICK-30 (1000 / 30 = 33.3 ms); a 60 Hz display (1000 / 60 = 16.7 ms) |
AFTER:
| PM-0002-13 | A press reaches the sim quickly in local play (part 5) | from the key event's `timeStamp` to the start of the tick that applies it, at most one tick plus one frame, 33.33 + 16.67 = 50.00 ms, at p95 (the 9,500th of 10,000) | ML on its 60 Hz panel: Chrome, Edge, Firefox; MAC: Safari (reported) | local mode, BX at N = 48, mode P, the page loaded as in PM-0002-8; 10,000 key events sent through Playwright, headed, so `timeStamp` is the event's own (Safari through safaridriver); the Worker stamps each tick's start on the shared clock | 1 run per browser. Bites: commands posted only from the frame callback plus one extra tick of buffering in the Worker | R1-BND-REC3 mark 2 (r1, 2294–2295); TICK-30 (1000 / 30 = 33.33 ms); a 60 Hz display (1000 / 60 = 16.67 ms) |
END

### FIX F3 (PM-0002-23)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
| PM-0002-23 | A busy page does not stall the connection (part 5: H2) | with a 100 ms busy loop injected on the page once every 3 s (100 loops), no interval between two snapshots the Worker processes, among those overlapping a busy loop, longer than two snapshot intervals (2 × 33.3 = 66.7 ms) at P0; the same reported at P2 | ML: Chrome, Edge, Firefox | online; the Worker stamps every processed snapshot; the page logs each busy loop's start and end on the shared clock (proposed reading of "no gap": a path through the page would show the whole 100 ms) | 1 run per browser and profile. Bites: snapshots relayed through the page | R1-BND-REC6 mark 2 (r1, 2335–2336); P0 and P2 (r2, 1989); 30 Hz snapshots (TICK-30) |
AFTER:
| PM-0002-23 | A busy page does not stall the connection (part 5: H2) | with a 100 ms busy loop injected on the page once every 3 s (100 loops), no interval between two snapshots the Worker processes, among those overlapping a busy loop, longer than two snapshot intervals (2 × 33.33 = 66.67 ms) at P0; the same reported at P2 | ML: Chrome, Edge, Firefox | online; the Worker stamps every processed snapshot; the page logs each busy loop's start and end on the shared clock (proposed reading of "no gap": a path through the page would show the whole 100 ms) | 1 run per browser and profile. Bites: snapshots relayed through the page | R1-BND-REC6 mark 2 (r1, 2335–2336); P0 and P2 (r2, 1989); 30 Hz snapshots (TICK-30) |
END

### FIX F3 (PM-0002-25)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
| PM-0002-25 | One Wasm instance or two in a local match (part 6; decides OQ-0002-3) | reported for (a) one instance and (b) two: calls per tick at N = 12 and 48 against PM-0002-3's marks; the Worker's time per tick (median, p95, p99) beside the client's sim share (13.3 ms at 30 Hz); each instance's Wasm memory after 10 minutes; for (b), gated: a trap planted in the client endpoint leaves [the authority](../research/GLOSSARY.md#the-authority)'s tick advancing and its [state hash](../research/GLOSSARY.md#state-hash) stream unbroken (proposed: the reason (b) exists, OQ-0004-7) | ML: Chrome; CI (counts) | BX in local mode, a 10-minute match per layout at N = 12 and 48; the Worker's time from `performance.now()` around each tick's Wasm work; memory from each instance's `memory.buffer.byteLength` | 1 match per layout and N; the trap test 1 of 1 | the draft's OQ-0002-3 ("What decides it"); R1-BND-REC1 mark 2 (r1, 2261–2263); GLOSSARY, the client's sim share (lines 214–218) |
AFTER:
| PM-0002-25 | One Wasm instance or two in a local match (part 6; decides OQ-0002-3) | reported for (a) one instance and (b) two: calls per tick at N = 12 and 48 against PM-0002-3's marks; the Worker's time per tick (median, p95, p99) beside the client's sim share (13.33 ms at 30 Hz); each instance's Wasm memory after 10 minutes; for (b), gated: a trap planted in the client endpoint leaves [the authority](../research/GLOSSARY.md#the-authority)'s tick advancing and its [state hash](../research/GLOSSARY.md#state-hash) stream unbroken (proposed: the reason (b) exists, OQ-0004-7) | ML: Chrome; CI (counts) | BX in local mode, a 10-minute match per layout at N = 12 and 48; the Worker's time from `performance.now()` around each tick's Wasm work; memory from each instance's `memory.buffer.byteLength` | 1 match per layout and N; the trap test 1 of 1 | the draft's OQ-0002-3 ("What decides it"); R1-BND-REC1 mark 2 (r1, 2261–2263); GLOSSARY, the client's sim share (lines 214–218) |
END

### FIX F3 (PM-0002-28)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
| PM-0002-28 | The sim fits the client's sim share in this Worker (ADR 0004's S2; R1-TICK-REC2 P2) | ADR 0004's numbers, measured here: p99 [sim step time](../research/GLOSSARY.md#sim-step-time) at most 13.3 ms at 30 Hz in Wasm in this Worker on ML while the page holds 30+ fps on Low for the whole run, read as every 1-second window holding at least 30 presented frames (proposed reading); N for the pass from R1-TICK-OQ7 (a design question), the scale steps 12, 24, 36 and 48 reported meanwhile; with the stand-in sim: reported only | ML: Chrome, Firefox | the sim core spike's sim in this Worker once it exists; step time from the Worker's `performance.now()` around the step; mode P; the page draws with the plain renderer until ADR 0003's renderer runs at Low, and the run repeats with it then (proposed: "on Low" names ADR 0003's tier); run length and count as ADR 0004's pass marks set them | as ADR 0004's S2 | R1-TICK-REC2 P2 (r1, 2054–2057); GLOSSARY lines 214–218 (40 % of 33.3 ms = 13.3 ms); ADR 0004 S2 |
AFTER:
| PM-0002-28 | The sim fits the client's sim share in this Worker (ADR 0004's S2; R1-TICK-REC2 P2) | ADR 0004's numbers, measured here: p99 [sim step time](../research/GLOSSARY.md#sim-step-time) at most 13.33 ms at 30 Hz in Wasm in this Worker on ML while the page holds 30+ fps on Low for the whole run, read as every 1-second window holding at least 30 presented frames (proposed reading); N for the pass from R1-TICK-OQ7 (a design question), the scale steps 12, 24, 36 and 48 reported meanwhile; with the stand-in sim: reported only | ML: Chrome, Firefox | the sim core spike's sim in this Worker once it exists; step time from the Worker's `performance.now()` around the step; mode P; the page draws with the plain renderer until ADR 0003's renderer runs at Low, and the run repeats with it then (proposed: "on Low" names ADR 0003's tier); run length and count as ADR 0004's pass marks set them | as ADR 0004's S2 | R1-TICK-REC2 P2 (r1, 2054–2057); GLOSSARY lines 214–218 (40 % of 33.33 ms = 13.33 ms); ADR 0004 S2 |
END

### FIX F3 (PM-0002-72)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
| PM-0002-72 | Maps and soaks (slice 2; parts 8 and 9) | G13 fails on a committed map with an unreachable spawn and passes on a valid one; a nightly 30-minute bot room at N = 48 over L1 at P2 runs with 0 panics, 0 hangs, 0 desyncs and a memory slope within 1 MB per 10 minutes after warm-up, and a planted leak (one buffer kept per snapshot) is flagged by the slope check within the run; the server tick time p99 against [the server tick budget](../research/GLOSSARY.md#the-server-tick-budget) (16.7 ms at 30 Hz, ADR 0001's) counts only from a run on [one vCPU](../research/GLOSSARY.md#the-measuring-unit-one-vcpu) of [the reference server](../research/GLOSSARY.md#the-reference-server), never from CI | CI (nightly); the reference server for the tick-time mark | `--prove`'s G13 pair; the soak harness of the draft's part 8 | 1 nightly run; the planted leak once | the draft's part 9 (G13's pair); R5-TEST-REC5's mark (r5, 1487–1489); GLOSSARY lines 167–172 |
AFTER:
| PM-0002-72 | Maps and soaks (slice 2; parts 8 and 9) | G13 fails on a committed map with an unreachable spawn and passes on a valid one; a nightly 30-minute bot room at N = 48 over L1 at P2 runs with 0 panics, 0 hangs, 0 desyncs and a memory slope within 1 MB per 10 minutes after warm-up, and a planted leak (one buffer kept per snapshot) is flagged by the slope check within the run; the server tick time p99 against [the server tick budget](../research/GLOSSARY.md#the-server-tick-budget) (16.67 ms at 30 Hz, ADR 0001's) counts only from a run on [one vCPU](../research/GLOSSARY.md#the-measuring-unit-one-vcpu) of [the reference server](../research/GLOSSARY.md#the-reference-server), never from CI | CI (nightly); the reference server for the tick-time mark | `--prove`'s G13 pair; the soak harness of the draft's part 8 | 1 nightly run; the planted leak once | the draft's part 9 (G13's pair); R5-TEST-REC5's mark (r5, 1487–1489); GLOSSARY lines 167–172 |
END

### FIX F3 (0004 key: the stale rounding note)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
  ML. "8.3 ms", "13.3 ms" and "1.67 ms" are 25 %, 40 % and 5 % of the tick period, 33.33 ms (GL lines 82–83): the marks
  gate at 8.33, 13.33 and 1.67 ms (arithmetic; the draft and GLOSSARY round the first two).
AFTER:
  ML. The marks gate at 8.33, 13.33 and 1.67 ms: 25 %, 40 % and 5 % of the tick period, 33.33 ms (GL lines 82–83;
  arithmetic). Every mark states a share at two decimals, the form of the 60 Hz margin's ruling (8.33, 6.25); prose
  and the GLOSSARY's sums may round to one.
END

### FIX F3 (PM-0004-9)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
| PM-0004-9 | One time representation keeps every legacy rate at the baseline (S4; part 4) | with held fire for 60 s of game time at 30 Hz, each of the 13 legacy intervals (12 `fireInterval`s and the one `burstInterval` in `docs/legacy-tuning.json`, each run as a repeating timer: proposed reading for the burst) gives a shot count within 1 shot of 60 ÷ interval (the Shabzon's 0.072 s: 833.3 shots, so within 1 shot is within 0.12 %, inside R1-ABL-REC3's 1 %: arithmetic); CD13's Warrior-kills-Conscript time of 0.86 s reproduced within one tick (33.3 ms) | CI (counts) | a scripted fire test per interval on the sim crate; the kill time from the sim's events in a scripted duel | 1 run (deterministic). Bites: legacy's reset-not-carry timer (GC5: 10.0 instead of 13.9 shots per second at 30 Hz) | R1-TICK-REC1 (lines 2022–2024); R1-ABL-REC3 (lines 2411–2412, 2416–2418); LEGACY_INVENTORY GC5 (line 244), CD13 (line 94) |
AFTER:
| PM-0004-9 | One time representation keeps every legacy rate at the baseline (S4; part 4) | with held fire for 60 s of game time at 30 Hz, each of the 13 legacy intervals (12 `fireInterval`s and the one `burstInterval` in `docs/legacy-tuning.json`, each run as a repeating timer: proposed reading for the burst) gives a shot count within 1 shot of 60 ÷ interval (the Shabzon's 0.072 s: 833.3 shots, so within 1 shot is within 0.12 %, inside R1-ABL-REC3's 1 %: arithmetic); CD13's Warrior-kills-Conscript time of 0.86 s reproduced within one tick (33.33 ms) | CI (counts) | a scripted fire test per interval on the sim crate; the kill time from the sim's events in a scripted duel | 1 run (deterministic). Bites: legacy's reset-not-carry timer (GC5: 10.0 instead of 13.9 shots per second at 30 Hz) | R1-TICK-REC1 (lines 2022–2024); R1-ABL-REC3 (lines 2411–2412, 2416–2418); LEGACY_INVENTORY GC5 (line 244), CD13 (line 94) |
END

### FIX F3 (PM-0005-22)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
| PM-0005-22 | Weapons as data equal the legacy tables (AB-2, AB-3; ability item 2) | for the seven legacy weapons and the Juggernaut cannon: damage per body part (head 2.6, body 1.0, legs 0.65; Bagpipe head 1.2), the Shabzon's falloff at 10, 15, 25, 35 and 50 m against legacy's `falloffMult`, pellet count, magazine, reload and burst: integer results exactly, floating-point results within 1e-6 relative; timing fields compared as stored values (the loader's converted duration in seconds within the rounding it reports, PM-0004-11) and as behaviour (each shot within one tick, 33.3 ms, of the table's time) (proposed reading: on a 30 Hz tick no shot can land on a 0.06 s instant, so 1e-6 applies to stored values only); a bot and a human firing one definition from the same inputs give identical results (whether v2 keeps one model is Mustafa's, R1-ABL-OQ6) | CI | golden fixtures captured from the legacy oracle (`tools/preview.js legacy eval`), each with the legacy commit, the inventory row and the inputs; v2's tests read them | 1 run; joins the gate. Bites: a v2 value moved beyond its tolerance (R5-TEST-REC3's mark); the legacy reset-not-carry timer (GC5) | R1-ABL-REC2's mark (lines 2396–2400); R5-TEST-REC3 (r5 lines 1431–1453); INV CS7, CD8 (lines 175, 89) |
AFTER:
| PM-0005-22 | Weapons as data equal the legacy tables (AB-2, AB-3; ability item 2) | for the seven legacy weapons and the Juggernaut cannon: damage per body part (head 2.6, body 1.0, legs 0.65; Bagpipe head 1.2), the Shabzon's falloff at 10, 15, 25, 35 and 50 m against legacy's `falloffMult`, pellet count, magazine, reload and burst: integer results exactly, floating-point results within 1e-6 relative; timing fields compared as stored values (the loader's converted duration in seconds within the rounding it reports, PM-0004-11) and as behaviour (each shot within one tick, 33.33 ms, of the table's time) (proposed reading: on a 30 Hz tick no shot can land on a 0.06 s instant, so 1e-6 applies to stored values only); a bot and a human firing one definition from the same inputs give identical results (whether v2 keeps one model is Mustafa's, R1-ABL-OQ6) | CI | golden fixtures captured from the legacy oracle (`tools/preview.js legacy eval`), each with the legacy commit, the inventory row and the inputs; v2's tests read them | 1 run; joins the gate. Bites: a v2 value moved beyond its tolerance (R5-TEST-REC3's mark); the legacy reset-not-carry timer (GC5) | R1-ABL-REC2's mark (lines 2396–2400); R5-TEST-REC3 (r5 lines 1431–1453); INV CS7, CD8 (lines 175, 89) |
END

### FIX F3 and F13 (PM-0006-13)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
| PM-0006-13 | Rooms per VM: 8 rooms of 48 on each machine (part 5; with ADR 0001's R2-ROOM-REC2 marks 1 and 2: one set of runs serves both records) | each of 8 rooms' server tick p99 at most 1.10 × the single-room figure; on SH also R2-ROOM-REC2 mark 1 for every room (p99 at most 16.7 ms, max under 33.3 ms, 0 overruns: [the server tick budget](../research/GLOSSARY.md#the-server-tick-budget) at 30 Hz) and every room start-up (process spawn to Open) under 2 s (R6-HOST-REC2 names both machines; the RS half of marks 1 and 5 is ADR 0001's) | RS and SH, each with LG | 8 room processes under the supervisor, no CPU pinning (as production would run them; recorded); the load at N = 48 (384 bot clients on LG); bracket B; each room process's CPU share of one vCPU per second also recorded (it holds the [transport runtime time](../research/GLOSSARY.md#transport-runtime-time)) | 3 sessions per machine at the 3 times of day; every valid session passes. **On SH it is an adoption mark:** a fail, or an invalid session, keeps the fleet on dedicated vCPUs (K1) and K3 out, as part 8 says; it is not a fail of this record. Bite: a busy-wait of 20 % of the single-room p99 planted in every tick of one room must fail that room's row (proposed) | R6-HOST-REC2's mark (r6 lines 667–671); R2-ROOM-REC2 marks 1, 2, 5 (r2 lines 2774–2780); A-2 (r6 lines 516–522); N32 (0006 the figure, 0001 the test) |
AFTER:
| PM-0006-13 | Rooms per VM: 8 rooms at N = 48 on each machine (part 5; with ADR 0001's R2-ROOM-REC2 marks 1 and 2: one set of runs serves both records) | each of 8 rooms' server tick p99 at most 1.10 × the single-room figure; on SH also R2-ROOM-REC2 mark 1 for every room (p99 at most 16.67 ms, max under 33.33 ms, 0 overruns: [the server tick budget](../research/GLOSSARY.md#the-server-tick-budget) at 30 Hz) and every room start-up (process spawn to Open) under 2 s (R6-HOST-REC2 names both machines; the RS half of marks 1 and 5 is ADR 0001's) | RS and SH, each with LG | 8 room processes under the supervisor, no CPU pinning (as production would run them; recorded); the load at N = 48 (384 bot clients on LG); bracket B; each room process's CPU share of one vCPU per second also recorded (it holds the [transport runtime time](../research/GLOSSARY.md#transport-runtime-time)) | 3 sessions per machine at the 3 times of day; every valid session passes. **On SH it is an adoption mark:** a fail, or an invalid session, keeps the fleet on dedicated vCPUs (K1) and K3 out, as part 8 says; it is not a fail of this record. Bite: a busy-wait of 20 % of the single-room p99 planted in every tick of one room must fail that room's row (proposed) | R6-HOST-REC2's mark (r6 lines 667–671); R2-ROOM-REC2 marks 1, 2, 5 (r2 lines 2774–2780); A-2 (r6 lines 516–522); N32 (0006 the figure, 0001 the test) |
END

### FIX F3 (PM-0006-23)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
| PM-0006-23 | A stall is visible at once (part 9) | after a deliberate 1 s stall inside one tick (proposed size: longer than the catch-up bound, so time is dropped), the first scrape shows the overrun counter up by exactly 1 and the dropped-time counter up by the stall minus the catch-up bound ADR 0004 sets, within one tick period (33.3 ms) | RS, MV | the stall injected by a test command at minute 25 of PM-0006-22's soak; Prometheus' scrape interval set to 15 s and recorded (proposed; the default was not read) | 3 stalls, 5 minutes apart (proposed); each must show. The stall is the planted event. Control: before the first stall the soak shows 0 overruns (R2-ROOM-REC2 mark 1's 0 overruns) | R6-OBS-REC2's mark (r6 lines 751–752); R1-TICK-REC4 (r1 lines 2078–2093) |
AFTER:
| PM-0006-23 | A stall is visible at once (part 9) | after a deliberate 1 s stall inside one tick (proposed size: longer than the catch-up bound, so time is dropped), the first scrape shows the overrun counter up by exactly 1 and the dropped-time counter up by the stall minus the catch-up bound ADR 0004 sets, within one tick period (33.33 ms) | RS, MV | the stall injected by a test command at minute 25 of PM-0006-22's soak; Prometheus' scrape interval set to 15 s and recorded (proposed; the default was not read) | 3 stalls, 5 minutes apart (proposed); each must show. The stall is the planted event. Control: before the first stall the soak shows 0 overruns (R2-ROOM-REC2 mark 1's 0 overruns) | R6-OBS-REC2's mark (r6 lines 751–752); R1-TICK-REC4 (r1 lines 2078–2093) |
END

### FIX F13 (PM-0005-32)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
| PM-0005-32 | How many long-lived active effects the step can carry (AB-10; ability item 7) | reported, no pass number: the ability, effect and weapon step's p99 per tick in SS at 8, 16 and 32 active effects per soldier (384, 768 and 1,536 at N = 48), and the bracket in which it reaches 1 ms (where the crossing lies between two points, points 4 effects per soldier apart are added: proposed); the extra effects are permanent attribute modifiers on the soldier's unit and weapon, the shape AB-8 gives picks (proposed); the effect count per soldier checked equal to the target on every tick | RS (native); ML: Chrome and Firefox (Wasm, isolation flag on) | T per point; the 8-per-soldier point is ADR 0004's PM-0004-62 run (its 1 ms p99 and 0 allocations are that row's pass marks) | T, V per point. Passes when every cell holds a number | AB-10 (proposed there); R1-ABL-REC6 (lines 2465–2472); GL lines 397–418 (8 per soldier) |
AFTER:
| PM-0005-32 | How many long-lived active effects the step can carry (AB-10; ability item 7) | reported, no pass number: the ability, effect and weapon step's p99 per tick in SS at 8, 16 and 32 active effects per soldier (384, 768 and 1,536 at N = 48), and the bracket in which it reaches 1 ms (where the crossing lies between two points, points 4 active effects per soldier apart are added: proposed); the extra effects are permanent attribute modifiers on the soldier's unit and weapon, the shape AB-8 gives picks (proposed); the effect count per soldier checked equal to the target on every tick | RS (native); ML: Chrome and Firefox (Wasm, isolation flag on) | T per point; the 8-per-soldier point is ADR 0004's PM-0004-62 run (its 1 ms p99 and 0 allocations are that row's pass marks) | T, V per point. Passes when every cell holds a number | AB-10 (proposed there); R1-ABL-REC6 (lines 2465–2472); GL lines 397–418 (8 per soldier) |
END

### FIX F13 (PM-0006-17)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
| PM-0006-17 | Egress per 24v24 match-hour matches R2's estimate (part 8) | bytes sent from RS's network interface in one room-hour within ±20 % of R2's E3 estimate at the counts that ran (R2's formula, r2 lines 1764–1766, evaluated in the results with the run's P, V, shots and projectile spawns, the arithmetic shown; proposed, in place of R6-COST-REC1's fixed 3.16 GB, which holds only at R2's counts: the proof test's load need not have them, since it stands in for the stress scene, whose 700 shots per second (ADR 0001's PM-0001-2) are almost ten times R2's 72, 1.5 per player per second × 48, r2 line 1759); at R2's own counts (P = 48, V = 15 moving vehicles, 0.44 projectile spawns per player per second, every entity in view) that is 3.156 GB, so 2.52 to 3.79 GB. Beside it, the room's own bytes-sent metric agrees with the interface counter within 5 % (proposed: R6-COST-REC2's tolerance, a check that the counter measures the room). With interest management on: the figure reported, only if the proof test's sim has the interest filter | RS with LG; one room alone | 48 bot clients over IPv4 (R2's case), all in view; three 20-minute matches back to back, joins and full states included (proposed: a room-hour as a real room spends it); the interface's transmitted-byte counter read at the start and end | 1 room-hour per setting; P2 on LG. Bite: snapshots sent twice per tick (about 2 × the reference) must fail | R6-COST-REC1's mark (r6 lines 716–717); the egress table (r6 lines 438–446: 146.1 kbps × 48 = 7.01 Mbit/s; × 3,600 s ÷ 8 = 3.156 GB); R2's estimate (r2 lines 1759–1762, 1775); A-1 (r6 line 515) |
AFTER:
| PM-0006-17 | Egress per 24v24 match-hour matches R2's estimate (part 8) | bytes sent from RS's network interface in one room-hour within ±20 % of R2's E3 estimate at the counts that ran (R2's formula, r2 lines 1764–1766, evaluated in the results with the run's P, V, shots and projectile spawns, the arithmetic shown; proposed, in place of R6-COST-REC1's fixed 3.16 GB, which holds only at R2's counts: the proof test's load need not have them, since it stands in for the stress scene, whose 700 shots per second (ADR 0001's PM-0001-2) are almost ten times R2's 72, 1.5 per player per second × 48, r2 line 1759); at R2's own counts (P = 48, V = 15 vehicles, 0.44 projectile spawns per player per second, every entity in view) that is 3.156 GB, so 2.52 to 3.79 GB. Beside it, the room's own bytes-sent metric agrees with the interface counter within 5 % (proposed: R6-COST-REC2's tolerance, a check that the counter measures the room). With interest management on: the figure reported, only if the proof test's sim has the interest filter | RS with LG; one room alone | 48 bot clients over IPv4 (R2's case), all in view; three 20-minute matches back to back, joins and full states included (proposed: a room-hour as a real room spends it); the interface's transmitted-byte counter read at the start and end | 1 room-hour per setting; P2 on LG. Bite: snapshots sent twice per tick (about 2 × the reference) must fail | R6-COST-REC1's mark (r6 lines 716–717); the egress table (r6 lines 438–446: 146.1 kbps × 48 = 7.01 Mbit/s; × 3,600 s ÷ 8 = 3.156 GB); R2's estimate (r2 lines 1759–1762, 1775); A-1 (r6 line 515) |
END

### FIX F7 (0004: the table's order note)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
Written by test-engineer (plan step 4). One row per mark, grouped by the spike that runs it: the sim core spike
(PM-0004-1 to PM-0004-61), the content and ability spike (PM-0004-62 to PM-0004-66), the netcode proof test
(PM-0004-67), the boundary spike (PM-0004-68 to PM-0004-70), and the 60 Hz set (PM-0004-71, PM-0004-72), which runs only
with the 60 Hz margin's test. The second column names the claim (S1–S20 above) and the decision part. Every number is
AFTER:
Written by test-engineer (plan step 4). One row per mark, grouped by the spike that runs it: the sim core spike
(PM-0004-1 to PM-0004-61, then PM-0004-73 and PM-0004-74, added at the fix round), the content and ability spike
(PM-0004-62 to PM-0004-66), the netcode proof test (PM-0004-67), the boundary spike (PM-0004-68 to PM-0004-70), and
the 60 Hz set (PM-0004-71, PM-0004-72), which runs only with the 60 Hz margin's test. The second column names the
claim (S1–S22 above) and the decision part. Every number is
END

### FIX F7 (PM-0004-73 and PM-0004-74, after PM-0004-61)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
| PM-0004-61 | A trap online ends in a fresh instance back in step (S20; part 20) | after a trap injected into the client's Worker: a fresh instance runs and holds the server's state (its [client-held state's hash](../research/GLOSSARY.md#state-hash) equal to the server's for that tick), the resync itself within ADR 0001's join mark (a complete world within 300 ms at P2); reported: the time from the trap to a running instance; the trap reaches error reporting (Sentry with Rust function names if ADR 0006's EU organisation exists by then, else the page's own error log) | ML: Chrome, Edge, Firefox; MAC: Safari | the boundary spike's Worker against the netcode proof test's server at P2 (ADR 0002's simulator); 20 traps per browser at random ticks (proposed) | 20 of 20 per browser. Bites: a host that does not restart the instance | R1-RR-OQ3 (lines 2936–2940); R2-BUD-REC3 (r2 line 2359: ADR 0001's join mark); R6-OBS-REC3 (r6 lines 765–766); GL lines 546–550 |
AFTER:
| PM-0004-61 | A trap online ends in a fresh instance back in step (S20; part 20) | after a trap injected into the client's Worker: a fresh instance runs and holds the server's state (its [client-held state's hash](../research/GLOSSARY.md#state-hash) equal to the server's for that tick), the resync itself within ADR 0001's join mark (a complete world within 300 ms at P2); reported: the time from the trap to a running instance; the trap reaches error reporting (Sentry with Rust function names if ADR 0006's EU organisation exists by then, else the page's own error log) | ML: Chrome, Edge, Firefox; MAC: Safari | the boundary spike's Worker against the netcode proof test's server at P2 (ADR 0002's simulator); 20 traps per browser at random ticks (proposed) | 20 of 20 per browser. Bites: a host that does not restart the instance | R1-RR-OQ3 (lines 2936–2940); R2-BUD-REC3 (r2 line 2359: ADR 0001's join mark); R6-OBS-REC3 (r6 lines 765–766); GL lines 546–550 |
| PM-0004-73 | Throws land the same on Wasm and native (S21; parts 7, 10; the id is ADR 0001's) | 10,000 random throws: the trajectory function gives bit-identical points on Wasm and native, 10,000 of 10,000 (every point of every throw equal bit for bit) | CI (native; Wasm in Node and in Chromium, Firefox and WebKit under Playwright) | the shared trajectory function in the sim crate, fed 10,000 throws (start position, velocity, fuse) drawn from one fixed seed; each throw's point at every tick of its flight logged as raw bits on each target and compared by script (proposed: a compared hash alone would not show where two flights part) | 1 run per target (the function is deterministic). Bites: one std `sin` call in the trajectory function at an input where std's `sin` differs between native and Wasm (PM-0004-22's table; if none differs, a 1-ulp planted change stands in, as in PM-0004-21) | R2-SYNC-REC6 mark 1 (r2 line 2218); the draft's S21 |
| PM-0004-74 | One feature set builds the sim on every target (S22; part 7; the id is ADR 0002's) | the resolved feature list of every dependency of `sim` (`cargo tree -e features`) identical for the `server` build and the `sim-wasm` build: 0 differences | CI | `cargo tree -e features` run for `sim` in each build's target and features; the two lists sorted and compared by script | 1 run; the foundation re-proves it as a gate check (ADR 0002). Bites: a planted extra feature on a shared dependency in `server` makes the check fail (R5-LAYOUT-REC4's own) | R5-LAYOUT-REC4 (r5 lines 1216–1218); the draft's S22 |
END

### FIX R2-BUD-REC2 precision (0001: the table's order note)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
Written by test-engineer (plan step 4). One row per mark, in the order of "What a spike must prove": three
preconditions (PM-0001-1 to PM-0001-3); the 48 simulated clients within the budgets at the 30 Hz baseline (PM-0001-4
to PM-0001-27); the online path on four browsers (PM-0001-28 to PM-0001-38); the 60 Hz margin (PM-0001-39, PM-0001-40);
AFTER:
Written by test-engineer (plan step 4). One row per mark, in the order of "What a spike must prove": three
preconditions (PM-0001-1 to PM-0001-3); the 48 simulated clients within the budgets at the 30 Hz baseline (PM-0001-4
to PM-0001-27, with PM-0001-57 after PM-0001-17); the online path on four browsers (PM-0001-28 to PM-0001-38); the
60 Hz margin (PM-0001-39, PM-0001-40);
END

### FIX R2-BUD-REC2 precision (PM-0001-57, after PM-0001-17)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-17 | Every decoded snapshot is the server's state (part 6) | 100 % of decoded snapshots rebuild the server's quantised state for that tick and that client's entity set, checked by [the client-held state's hash](../research/GLOSSARY.md#state-hash); also across the 16-bit tick wrap of every S30 | L1 (48 headless clients); L2: LG's 48 clients | 10 minutes each under P0, P2, P3 and B1; the server logs each client's quantised hash per tick, the client logs its decoded hash; compared by script | L1: 3 seeds per profile; L2: 1 run per profile; 0 mismatches in each. Bites: a 16-bit tick in the snapshot header (fails at the wrap); a client quantiser that rounds differently from the server's | r2 R2-SYNC-REC1 mark 1 (lines 2017–2019); the draft, part 6 (lines 240–243) |
AFTER:
| PM-0001-17 | Every decoded snapshot is the server's state (part 6) | 100 % of decoded snapshots rebuild the server's quantised state for that tick and that client's entity set, checked by [the client-held state's hash](../research/GLOSSARY.md#state-hash); also across the 16-bit tick wrap of every S30 | L1 (48 headless clients); L2: LG's 48 clients | 10 minutes each under P0, P2, P3 and B1; the server logs each client's quantised hash per tick, the client logs its decoded hash; compared by script | L1: 3 seeds per profile; L2: 1 run per profile; 0 mismatches in each. Bites: a 16-bit tick in the snapshot header (fails at the wrap); a client quantiser that rounds differently from the server's | r2 R2-SYNC-REC1 mark 1 (lines 2017–2019); the draft, part 6 (lines 240–243) |
| PM-0001-57 | Decoded values keep the protocol's precision (part 6; R2-BUD-REC2's precision mark as Appendix A restates it) | over every entity value a client decodes in a run, the max error against the server's value before quantising: positions at most 1 cm; yaw at most 0.01°; pitch at most one 0.18° step (the restated mark, proposed there: the 10-bit field's own resolution) | L1 (48 headless clients) | PM-0001-17's L1 runs: the server logs each sent entity's position, yaw and pitch before quantising, the client logs the values it decodes, and a script takes the max error per field (proposed: PM-0001-17's hash proves the decoded values equal the server's quantised ones, not how far quantising moved them) | as PM-0001-17's L1: 3 seeds per profile under P0, P2, P3 and B1; each meets the mark. Bites, one per field: positions quantised to 4 cm (errors up to 2 cm); yaw in 12 bits (0.088° steps); pitch in 8 bits (0.70° steps) | R2-BUD-REC2 (line 2341, "decoded positions within 1 cm and angles within 0.01°"); the 10-bit pitch (line 1748); the 16-bit yaw step, about 0.0055° (line 2263); the draft, part 6 and Appendix A |
END

### FIX PM-0002-42 (the enum plant, as PM-0002-17)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
| PM-0002-42 | G3 bites (parts 3 and 7: TypeScript) | each fails G3 only: one violation per strict flag (an unchecked index read, `x = undefined` into an optional field, a missing `override`, a fall-through `case`, a path without `return`, an `enum`, a value import used only as a type); an unawaited promise; a switch over a generated union missing one kind while having a `default`; an import of `editor` from `client`; a bare `event`; rendering or DOM code imported by `sim-worker` or `net`; an import from `legacy/` or `spikes/`; control: `import type` from `types` in `editor` passes; the clean tree has 0 errors and 0 warnings | CI | `--prove` | every `--prove` run | R5-TSLINT-REC1's mark (r5, 1261–1263); R5-TSLINT-REC2's mark (r5, 1279–1281); R5-GATE-REC3's G3 row (r5, 1715); the draft's parts 3 and 7 |
AFTER:
| PM-0002-42 | G3 bites (parts 3 and 7: TypeScript) | each fails G3 only: one violation per strict flag (an unchecked index read, `x = undefined` into an optional field, a missing `override`, a fall-through `case`, a path without `return`, a class with a parameter property for `erasableSyntaxOnly`, a value import used only as a type; proposed in place of R5-TSLINT-REC1's `enum`: the tsconfig page R5-TSLINT-F12 read gives the parameter property as the flag's example, while that the flag rejects an `enum` is unverified at the source, as in PM-0002-17; the `enum` is planted too, its result recorded, not required); an unawaited promise; a switch over a generated union missing one kind while having a `default`; an import of `editor` from `client`; a bare `event`; rendering or DOM code imported by `sim-worker` or `net`; an import from `legacy/` or `spikes/`; control: `import type` from `types` in `editor` passes; the clean tree has 0 errors and 0 warnings | CI | `--prove` | every `--prove` run | R5-TSLINT-REC1's mark (r5, 1261–1263); R5-TSLINT-F12 (r5, 221–224); R5-TSLINT-REC2's mark (r5, 1279–1281); R5-GATE-REC3's G3 row (r5, 1715); the draft's parts 3 and 7 |
END

### FIX F4 (PM-0002-26)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
| PM-0002-26 | The Worker paces the tick (part 5 with ADR 0004's part 5: ADR 0004's S18; decides R1-TICK-OQ2, R1-BND-OQ2 and the pacing half of R1-XU-OQ4) | ADR 0004's PM-0004-68 to PM-0004-70, measured once for both records: their numbers, definitions, machines, runs and bites stand, and this row sets none of its own (ADR 0004's part 5 decides the pacing). Restated: foreground Worker tick start jitter p99 at most 4 ms over 18,000 ticks (10 minutes at 30 Hz); a constant-speed unit's per-frame on-screen displacement with a standard deviation at most 5 % of its mean, on 60, 120 and 144 Hz displays; after 60 s and after 6 minutes hidden, in local play no tick ran while hidden (the tick counter equal before and after) and play resumes where it stopped, and online, within 0.5 s of the tab's return, the client has applied the server's state and its commands are accepted again (PM-0004-70's proposed definition of "back in step"), never running more than 7 ticks in one frame | as PM-0004-68 to PM-0004-70: jitter and the hidden tab on ML (Chrome, Edge, Firefox) and MAC (Safari 26); smoothness on RD, ADR 0003's recommended desktop (Chrome and Firefox on the displays on hand, each recorded), on ML (Chrome at its own rate) and on MAC (Safari) | the setup PM-0004-68 to PM-0004-70 name: the boundary spike's Worker and page with the sim core spike's sim at N = 48; online against the netcode proof test's server at P2. Jitter: the Worker logs each tick's scheduled and actual start (p99 by nearest rank, the 17,820th of 18,000). Smoothness: a scripted mover at constant speed; the page logs its drawn position every frame; mode P. Hidden: on `visibilitychange`, the tick counter and the state hash are logged at hiding and at showing; ticks per frame counted. "More than 5 minutes" read as 6 (PM-0004-70's proposal; the reason: Chrome's intensive throttling starts at 5 minutes, so the run sees a minute of it) | as PM-0004-68 to PM-0004-70: jitter, 3 runs per browser; smoothness, 3 runs of 60 s per display and browser; hidden, 5 hides per duration and browser; each meets the mark. Bites, as theirs: ticks counted per timer callback instead of from elapsed time; the newest tick drawn without interpolation; a Worker that simulates the whole hidden gap | ADR 0004's PM-0004-68 to PM-0004-70 and S18; R1-TICK-REC5's marks (r1, 2109–2114: 15 ticks at 60 Hz, restated as 7 at 30 Hz by ADR 0004's part 5); R1-BND-OQ2 (r1, 2724–2727); R1-XU-OQ12 (r1, 2903–2906) |
AFTER:
| PM-0002-26 | The Worker paces the tick (part 5 with ADR 0004's part 5: ADR 0004's S18; decides R1-TICK-OQ2, R1-BND-OQ2 and the pacing half of R1-XU-OQ4) | ADR 0004's PM-0004-68 to PM-0004-70, measured once for both records: their numbers, definitions, machines, runs and bites stand, and this row sets none of its own (ADR 0004's part 5 decides the pacing). Restated: foreground Worker tick start jitter p99 at most 4 ms over 18,000 ticks (10 minutes at 30 Hz); a constant-speed unit's per-frame on-screen displacement with a standard deviation at most 5 % of its mean, on 60, 120 and 144 Hz displays; after 60 s and after 6 minutes hidden, in local play no tick ran while hidden (the tick counter equal before and after) and play resumes where it stopped, and online, within 0.5 s of the tab's return, the client has applied the server's state and its commands are accepted again (PM-0004-70's proposed definition of "back in step"), never running more than 7 ticks in one frame | as PM-0004-68 to PM-0004-70: jitter and the hidden tab on ML (Chrome, Edge, Firefox) and MAC (Safari); smoothness on RD, ADR 0003's recommended desktop (Chrome and Firefox on the displays on hand, each recorded), on ML (Chrome at its own rate) and on MAC (Safari) | the setup PM-0004-68 to PM-0004-70 name: the boundary spike's Worker and page with the sim core spike's sim at N = 48; online against the netcode proof test's server at P2. Jitter: the Worker logs each tick's scheduled and actual start (p99 by nearest rank, the 17,820th of 18,000). Smoothness: a scripted mover at constant speed; the page logs its drawn position every frame; mode P. Hidden: on `visibilitychange`, the tick counter and the state hash are logged at hiding and at showing; ticks per frame counted. "More than 5 minutes" read as 6 (PM-0004-70's proposal; the reason: Chrome's intensive throttling starts at 5 minutes, so the run sees a minute of it) | as PM-0004-68 to PM-0004-70: jitter, 3 runs per browser; smoothness, 3 runs of 60 s per display and browser; hidden, 5 hides per duration and browser; each meets the mark. Bites, as theirs: ticks counted per timer callback instead of from elapsed time; the newest tick drawn without interpolation; a Worker that simulates the whole hidden gap | ADR 0004's PM-0004-68 to PM-0004-70 and S18; R1-TICK-REC5's marks (r1, 2109–2114: 15 ticks at 60 Hz, restated as 7 at 30 Hz by ADR 0004's part 5); R1-BND-OQ2 (r1, 2724–2727); R1-XU-OQ12 (r1, 2903–2906) |
END

### FIX F4 (0003 key, MAC)
FILE: docs/adr/0003-rendering.md
BEFORE:
  1920 × 1080 and 2560 × 1440. **MAC**, a macOS machine with Safari 26 for the marks that name Safari; the draft
  names none, so its model is recorded with each result, and it carries no budget row. **MP**, Mustafa's PC (RTX
AFTER:
  1920 × 1080 and 2560 × 1440. **MAC**, SPIKE_PLAN's macOS machine: the stable Safari on the day of the run, its
  version recorded (WebGPU from Safari 26.0, R3-WGPU-F19, lines 171–174; WebTransport from 26.4, R2-TRN-F7,
  r2-netcode.md lines 959–975), for the marks that name Safari; the draft names none, so its model is recorded with
  each result, and it carries no budget row. **MP**, Mustafa's PC (RTX
END

### FIX F4 (PM-0003-2)
FILE: docs/adr/0003-rendering.md
BEFORE:
| PM-0003-2 | One build runs on both backends in every target browser (part 1) | 0 console errors in every start | ML: Chrome, Edge and Firefox (Windows), each on WebGPU and with `forceWebGL`; MAC: Safari 26, both backends | LS played through its whole script; the page's own error hook writes every console error and page error to the results log (no browser driver needed) | the 10 starts of PM-0003-5 per browser and backend; 0 in each. Bites: a `console.error` 2 s after load (R5-GATE-REC3's G12 mutation, r5-toolchain-quality.md line 1723) | R3-WGPU-REC1's mark 1 (lines 910–912) |
AFTER:
| PM-0003-2 | One build runs on both backends in every target browser (part 1) | 0 console errors in every start | ML: Chrome, Edge and Firefox (Windows), each on WebGPU and with `forceWebGL`; MAC: Safari, both backends | LS played through its whole script; the page's own error hook writes every console error and page error to the results log (no browser driver needed) | the 10 starts of PM-0003-5 per browser and backend; 0 in each. Bites: a `console.error` 2 s after load (R5-GATE-REC3's G12 mutation, r5-toolchain-quality.md line 1723) | R3-WGPU-REC1's mark 1 (lines 910–912) |
END

### FIX F4 (PM-0003-3)
FILE: docs/adr/0003-rendering.md
BEFORE:
| PM-0003-3 | The fallback shows the same look (parts 1, 5; decides R3-GL2-OQ3) | at each of 3 fixed frames, at most 1 % of pixels differ by more than 8/255 in any channel between the two backends (a tolerance the research marks proposed); effects that exist on one backend only are off | ML: Chrome and Firefox, WebGPU against `forceWebGL`; MAC: Safari 26, the same pair | LS's three fixed frames read from the canvas on each backend, same machine and browser; a pixel-diff script counts the pixels with any channel difference above 8 | 1 capture per frame and backend. Control: the same backend captured twice gives 0 differing pixels (else the scene does not repeat and the set is invalid). Bites: the ink switched off on the WebGL 2 backend only | R3-WGPU-REC1's mark 2 (lines 912–913); R3-GL2-REC1's mark (lines 965–967) |
AFTER:
| PM-0003-3 | The fallback shows the same look (parts 1, 5; decides R3-GL2-OQ3) | at each of 3 fixed frames, at most 1 % of pixels differ by more than 8/255 in any channel between the two backends (a tolerance the research marks proposed); effects that exist on one backend only are off | ML: Chrome and Firefox, WebGPU against `forceWebGL`; MAC: Safari, the same pair | LS's three fixed frames read from the canvas on each backend, same machine and browser; a pixel-diff script counts the pixels with any channel difference above 8 | 1 capture per frame and backend. Control: the same backend captured twice gives 0 differing pixels (else the scene does not repeat and the set is invalid). Bites: the ink switched off on the WebGL 2 backend only | R3-WGPU-REC1's mark 2 (lines 912–913); R3-GL2-REC1's mark (lines 965–967) |
END

### FIX F4 (PM-0003-5)
FILE: docs/adr/0003-rendering.md
BEFORE:
| PM-0003-5 | The backend is named correctly everywhere (part 4; decides R3-WGPU-OQ1) | correct in 10 of 10 starts per browser and backend; on ML-Linux, Firefox's overlay names "WebGL 2 fallback"; Brave's backend on MP recorded | ML: Chrome, Edge, Firefox; ML-Linux: Firefox, Chrome; MAC: Safari 26; MP: Brave | each start, the overlay's and the log's string compared with what it should be: "WebGPU" when `navigator.gpu` returns an adapter and WebGL 2 is not forced, else "WebGL 2 fallback"; every test screenshot prints the string (R5-GATE-REC1's G12, r5-toolchain-quality.md line 1670) | 10 starts per browser and backend (forced and not). Bites: a hard-coded "WebGPU" label fails on ML-Linux's Firefox | R3-WGPU-REC4's mark (lines 950–951); R3-BUDGET-REC1's mark (lines 1115–1116); R3-WGPU-OQ1 (lines 1286–1289) |
AFTER:
| PM-0003-5 | The backend is named correctly everywhere (part 4; decides R3-WGPU-OQ1) | correct in 10 of 10 starts per browser and backend; on ML-Linux, Firefox's overlay names "WebGL 2 fallback"; Brave's backend on MP recorded | ML: Chrome, Edge, Firefox; ML-Linux: Firefox, Chrome; MAC: Safari; MP: Brave | each start, the overlay's and the log's string compared with what it should be: "WebGPU" when `navigator.gpu` returns an adapter and WebGL 2 is not forced, else "WebGL 2 fallback"; every test screenshot prints the string (R5-GATE-REC1's G12, r5-toolchain-quality.md line 1670) | 10 starts per browser and backend (forced and not). Bites: a hard-coded "WebGPU" label fails on ML-Linux's Firefox | R3-WGPU-REC4's mark (lines 950–951); R3-BUDGET-REC1's mark (lines 1115–1116); R3-WGPU-OQ1 (lines 1286–1289) |
END

### FIX F4 (PM-0003-6)
FILE: docs/adr/0003-rendering.md
BEFORE:
| PM-0003-6 | GPU timing on the fallback never fakes a number (part 4) | Chrome's WebGL 2 backend: a GPU time above 0 in 600 of 600 frame records; Firefox's and Safari's WebGL 2 backends: "GPU time not available" in 600 of 600, never 0 | ML: Chrome and Firefox with `forceWebGL`; ML-Linux: Firefox; MAC: Safari 26 with `forceWebGL` | one H run on LS per browser; the frame records read from the results log | 1 run per browser (a count over every frame). Bites: a harness that writes 0 when the timer is missing fails on Firefox | R3-GL2-REC3's mark (lines 984–985) |
AFTER:
| PM-0003-6 | GPU timing on the fallback never fakes a number (part 4) | Chrome's WebGL 2 backend: a GPU time above 0 in 600 of 600 frame records; Firefox's and Safari's WebGL 2 backends: "GPU time not available" in 600 of 600, never 0 | ML: Chrome and Firefox with `forceWebGL`; ML-Linux: Firefox; MAC: Safari with `forceWebGL` | one H run on LS per browser; the frame records read from the results log | 1 run per browser (a count over every frame). Bites: a harness that writes 0 when the timer is missing fails on Firefox | R3-GL2-REC3's mark (lines 984–985) |
END

### FIX F4 (PM-0003-41)
FILE: docs/adr/0003-rendering.md
BEFORE:
| PM-0003-41 | Sound in step with the picture (part 9) | sound onset after the drawn event at most one rendered frame plus the output latency, p99 over 100 shots (the 99th by nearest rank); 0 autoplay warnings in the console on all four browsers | ML: Chrome, Edge, Firefox; MAC: Safari 26; mode P | 100 shots in BS; per shot, the time the muzzle flash is drawn against the time the sound leaves the output, mapped with the AudioContext's output timestamp and output latency (proposed tools; not researched) | 1 run of 100 shots per browser (the mark's count). Bites: sounds started from the next block's events (one tick late) | R3-AUDIO-REC4's mark (lines 1279–1280) |
AFTER:
| PM-0003-41 | Sound in step with the picture (part 9) | sound onset after the drawn event at most one rendered frame plus the output latency, p99 over 100 shots (the 99th by nearest rank); 0 autoplay warnings in the console on all four browsers | ML: Chrome, Edge, Firefox; MAC: Safari; mode P | 100 shots in BS; per shot, the time the muzzle flash is drawn against the time the sound leaves the output, mapped with the AudioContext's output timestamp and output latency (proposed tools; not researched) | 1 run of 100 shots per browser (the mark's count). Bites: sounds started from the next block's events (one tick late) | R3-AUDIO-REC4's mark (lines 1279–1280) |
END

### FIX F4 (0004 key, MAC; rust-reviewer's held block, taken over)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
  (proposed: the sim's speed differs by engine), and Edge is reported. **MAC**, a macOS machine with Safari 26, for the
  Safari runs (real Safari through its WebDriver: R5-TEST-REC2, r5 lines 1415–1418); its model recorded; never a budget
AFTER:
  (proposed: the sim's speed differs by engine), and Edge is reported. **MAC**, SPIKE_PLAN's macOS machine: the stable
  Safari on the day of the run, its version recorded (WebGPU from Safari 26.0, R3-WGPU-F19, r3 lines 171–174;
  WebTransport from 26.4, R2-TRN-F7, r2 lines 959–975), for the Safari runs (real Safari through its WebDriver:
  R5-TEST-REC2, r5 lines 1415–1418); its model recorded; never a budget
END

### FIX F4 (PM-0004-68)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
| PM-0004-68 | The Worker paces the tick in every browser (S18; part 5) | foreground Worker tick start jitter p99 ≤ 4 ms over 18,000 ticks (10 minutes at 30 Hz) | ML: Chrome, Edge, Firefox; MAC: Safari 26 | the boundary spike's Worker (W2: `setTimeout` waking a `performance.now()` accumulator) with the sim core spike's sim at N = 48; start times logged in the Worker; the isolation flag as shipped (recorded); if W2 fails, W3 is measured on an isolated build (ADR 0002, OQ-0002-4) | 3 runs per browser; each meets the mark. Bites: ticks counted per timer callback instead of from elapsed time | R1-TICK-REC5 (lines 2096–2097, 2109–2110) |
AFTER:
| PM-0004-68 | The Worker paces the tick in every browser (S18; part 5) | foreground Worker tick start jitter p99 ≤ 4 ms over 18,000 ticks (10 minutes at 30 Hz) | ML: Chrome, Edge, Firefox; MAC: Safari | the boundary spike's Worker (W2: `setTimeout` waking a `performance.now()` accumulator) with the sim core spike's sim at N = 48; start times logged in the Worker; the isolation flag as shipped (recorded); if W2 fails, W3 is measured on an isolated build (ADR 0002, OQ-0002-4) | 3 runs per browser; each meets the mark. Bites: ticks counted per timer callback instead of from elapsed time | R1-TICK-REC5 (lines 2096–2097, 2109–2110) |
END

### FIX F4 (0005 key, MAC)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
  Firefox on Windows 11, with mains power and the power mode recorded. **MAC**, a macOS machine with Safari 26, for
  the marks that name Safari (real Safari through its WebDriver: R5-TEST-REC1's Safari row, r5 line 1394). **MP**,
AFTER:
  Firefox on Windows 11, with mains power and the power mode recorded. **MAC**, SPIKE_PLAN's macOS machine: the
  stable Safari on the day of the run, its version recorded (WebGPU from Safari 26.0, R3-WGPU-F19, r3 lines 171–174;
  WebTransport from 26.4, R2-TRN-F7, r2 lines 959–975), for the marks that name Safari (real Safari through its
  WebDriver: R5-TEST-REC1's Safari row, r5 line 1394). **MP**,
END

### FIX F4 (PM-0005-2)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
| PM-0005-2 | Every side reads the same numbers (CD-1; content item 1) | 100 % of doubles parse to the same 64-bit pattern in native Rust, in Rust on Wasm and in `JSON.parse` | CI (native, Wasm, `JSON.parse` in Node and the three engines); MAC: Safari 26 (`JSON.parse`) | the corpus: every number in `docs/legacy-tuning.json` (2,038, counted by script at 9e7db28) plus 10,000 generated doubles, 5,000 from uniform random 64-bit patterns with NaN and the infinities left out (JSON cannot carry them) and 5,000 decimals of up to 17 significant digits inside the legacy file's range (proposed: the first half reaches subnormals and extreme exponents, the second the values content holds), plus −0 and the largest and smallest finite doubles; each side writes each value's bits as 16 hex digits; a script compares | 1 run per target (exact). Bites: a parser that rounds through `f32` on one side | R1-DATA-REC1 mark 1 (lines 2494–2496); CD-1 |
AFTER:
| PM-0005-2 | Every side reads the same numbers (CD-1; content item 1) | 100 % of doubles parse to the same 64-bit pattern in native Rust, in Rust on Wasm and in `JSON.parse` | CI (native, Wasm, `JSON.parse` in Node and the three engines); MAC: Safari (`JSON.parse`) | the corpus: every number in `docs/legacy-tuning.json` (2,038, counted by script at 9e7db28) plus 10,000 generated doubles, 5,000 from uniform random 64-bit patterns with NaN and the infinities left out (JSON cannot carry them) and 5,000 decimals of up to 17 significant digits inside the legacy file's range (proposed: the first half reaches subnormals and extreme exponents, the second the values content holds), plus −0 and the largest and smallest finite doubles; each side writes each value's bits as 16 hex digits; a script compares | 1 run per target (exact). Bites: a parser that rounds through `f32` on one side | R1-DATA-REC1 mark 1 (lines 2494–2496); CD-1 |
END

### FIX F4 (PM-0005-34)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
| PM-0005-34 | The models load on both backends and carry no look of their own (GPU section parts 1, 6) | 0 console errors in every start; 0 meshes of a listed asset drawn with a material other than its slot class's shared material (identity against the material cache); 0 model requests other than listed glTF/GLB files, and 0 requests outside the client's host | ML: Chrome, Edge, Firefox, each on WebGPU and with `forceWebGL`; MAC: Safari 26, both backends | PM-0003-2's starts; after load, a script walks the scene graph; the page's network log | PM-0003-2's 10 starts per browser and backend. Bites: one slot left with its glTF material | the GPU section's spike part (R3-WGPU-REC1 mark 1, r3 lines 910–912); R5-GATE-REC3's G12 mutation (r5 line 1723) |
AFTER:
| PM-0005-34 | The models load on both backends and carry no look of their own (GPU section parts 1, 6) | 0 console errors in every start; 0 meshes of a listed asset drawn with a material other than its slot class's shared material (identity against the material cache); 0 model requests other than listed glTF/GLB files, and 0 requests outside the client's host | ML: Chrome, Edge, Firefox, each on WebGPU and with `forceWebGL`; MAC: Safari, both backends | PM-0003-2's starts; after load, a script walks the scene graph; the page's network log | PM-0003-2's 10 starts per browser and backend. Bites: one slot left with its glTF material | the GPU section's spike part (R3-WGPU-REC1 mark 1, r3 lines 910–912); R5-GATE-REC3's G12 mutation (r5 line 1723) |
END

### FIX F4 (PM-0005-42)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
| PM-0005-42 | Every slot is classed and every faction slot coloured, on both backends (GPU section parts 2, 6) | 0 slots without a class; every faction colour slot's colour equal to its faction definition's colour value, for every faction and test model; 0 console errors | ML: Chrome, Edge, Firefox on both backends; MAC: Safari 26 | the research's processed test models in LS; colours read back from the materials and compared with the faction data | 10 starts per browser and backend (PM-0003-2's count). Bites: a faction slot given another faction's colour | the GPU section's spike part |
AFTER:
| PM-0005-42 | Every slot is classed and every faction slot coloured, on both backends (GPU section parts 2, 6) | 0 slots without a class; every faction colour slot's colour equal to its faction definition's colour value, for every faction and test model; 0 console errors | ML: Chrome, Edge, Firefox on both backends; MAC: Safari | the research's processed test models in LS; colours read back from the materials and compared with the faction data | 10 starts per browser and backend (PM-0003-2's count). Bites: a faction slot given another faction's colour | the GPU section's spike part |
END

### FIX F2 (0004: part 18's open rules; rust-reviewer's held block, taken over)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
- Part 18's open rules (which deaths drop which weapons, who may pick up, ties, takeovers: OQ-0004-5 and -6; carry
AFTER:
- Part 18's open rules (which deaths drop which weapons, who may pick up, ties, takeovers: OQ-0005-AB9 and AB10; carry
END

### FIX F14 (0003 checklist step 24: fragments as damage)
FILE: docs/adr/0003-rendering.md
BEFORE:
    Whether fragments deal damage is not judged here (a design question).
AFTER:
    Whether fragments deal damage is not judged here: a design question for Mustafa with the game-designer
    (`docs/adr/README.md`, "Decisions the drafts ask for": fragments as damage).
END

### FIX X5 (0003 checklist step 29: the camera-player cutaway)
FILE: docs/adr/0003-rendering.md
BEFORE:
29. The legacy touches LOOK_V2 does not name, each shown beside legacy (OQ-0003-15): comic words, the kill vignette,
    the toy-soldier death fall, the skin rim. Pick: keep, change or drop each.
AFTER:
29. The legacy touches LOOK_V2 does not name, each shown beside legacy (OQ-0003-15): comic words, the kill vignette,
    the toy-soldier death fall, the skin rim, and the camera-player cutaway (structures between the camera and the
    player fade, beyond LOOK_V2 §4's roofs; walk the player behind a building). Pick: keep, change or drop each.
END
