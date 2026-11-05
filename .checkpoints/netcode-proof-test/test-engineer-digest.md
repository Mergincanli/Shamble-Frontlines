# test-engineer digest: netcode proof test (SPIKE_PLAN item 1) — link simulator, load layer, client harness, evidence plan

## Plan (blocks, in order)
1. Read ADR 0002: L1/L2, simulator, R5-TEST-REC4/5, client harness, Worker host, plain renderer, SPIKE-SCOPE (a), spikes/ rules, pass marks naming the netcode proof test.
2. Read ADR 0004: S17, PM-0004-67, PM-0004-61, PM-0004-70.
3. Read ADR 0001: Test plan (1727-1758), Migration plan (1684-1726), part 13 P0.. profiles and bursts.
4. Read GLOSSARY §2, §3, §8, "time statistics".
5. Read r5-toolchain-quality.md: R5-TEST-REC4, REC5, OQ2 exact text.
6. Container toolchain (read-only commands).
7. Read SPIKE_PLAN item 1 (130-204) for planted defects and marks (grep "planted").
8. Write sections A-G.

## Notes
- 0002:515-521 L1 = seeded link sim on simulated time behind thin transport interface (latency, jitter, loss, reorder, dup, rate limit, packet-fate log); per direction latency+jitter half each way, loss "as the profile defines it" (ADR 0001 part 13, OQ-0001-16, NET-PROFILES 2026-11-03); real server code + headless bot clients in one process. L2 = impairment wrapper around server's UDP socket under real quinn endpoint, browsers over WT; delaying TCP proxy for WS.
- 0002:843-846 options table: L1 chosen (deterministic, seeded, "no QUIC, no browser"); L2 chosen (real time; "WebSocket needs a separate proxy"); turmoil not chosen; tc netem "docs not read: not proposable". 0002:1607-1608 R5-TEST-OQ5 turmoil not added; R5-TEST-OQ6 no OS-level layer unless L2 cannot reproduce a playtest bug. 0002:1604 R5-TEST-OQ2 -> spike: the netcode proof test (WT crates' interop).
- 0002:522-531 soak/load: bots = headless Rust clients on real protocol (client-endpoint); CI judged only by desyncs/panics/hangs/RSS slope/queue lengths/bytes per client per s; tick time judged only on 1 vCPU of reference server.
- 0002:532-538 flake rule: retry 0, fixed PROPTEST_RNG_SEED in gate, simulated time only in sim+net tests, Playwright clock, failures keep seed+recording+trace, issue carries seed.
- 0002:546-551 bench/soak machine = CCX23 per session in spikes' own Hetzner project; SPIKE-MACHINES quote.
- 0002:580 G11 = L1 one minute per profile P0-P4 + B1. 0002:1301 PM-0002-49 (foundation row): identical fate log + hashes on 2 runs; 1,000,000 packets per profile per direction, loss/mean one-way delay/jitter within 5 % relative; G11 < 2 min total over 5 runs. 0002:1335-1338 note: 100,000 packets too few (1 seed in 9 fails).
- 0002:1199-1213 BX = proof test's throwaway sim (movement, hitscan, bots as protocol clients) in the Worker; "page draws the blocks with the shared client harness's plain renderer, not the look"; Online = Worker client endpoint to proof test's room process over WT "through that test's link impairment"; P0/P2 definitions r2 1987-1990.
- 0002:1113-1116 boundary spike shares proof test's client harness and stand-in sim. 0002:1422-1425 migration step 1 same.
- 0002:1239-1242 bites rule + Rule B (busy-wait sized margin+1 ms).
- 0002 pass-mark table holds no PM row owned by the proof test; it only consumes the proof test's room process (PM-0002-22/23/26/78 online rows).
- Block 1 partial: L1/L2, harness, pass marks done; next: SPIKE-SCOPE/spikes rules + pins.
- SPIKE_PLAN:12-17 rules for every spike: throwaway, in spikes/, exempt from legacy hard constraints (SPIKE-SCOPE (a)); own change-set plan first; one heavy spike at a time; server numbers at 30 Hz.
- SPIKE_PLAN:67-68 profiles P0-P4 and L1/L2: proof test builds them first, throwaway.
- SPIKE_PLAN:45-57 Safari leg: GitHub macOS runner stands as MAC only when results record (i) Safari >= 26.4, (ii) real Safari via safaridriver, (iii) one WT session to RS carrying datagrams + one WS session on the domain; yes/no and count rows only (PM-0001-28, -14, -25 Origin, PM-0006-15); time rows stay on borrowed Mac; workflow reviewed by security-auditor + infra-engineer; .github/ change to Mustafa.
- SPIKE_PLAN:62-66 no draft writes an address; each run records day's addresses + new SSH host keys check.
- SPIKE_PLAN:117 marks: PM-0001-1..-62; PM-0004-67; PM-0006-11..-27. 141-145: -39/-40 belong to 60 Hz run; -54/-55 conditional; PM-0006-13 and PM-0001-27 share runs; server also serves online halves of PM-0004-61, -70.
- SPIKE_PLAN:160-167 throwaway sim native+Wasm with stand-in vehicles + projectiles at stress counts (PM-0001-2); S1 stand-ins 384 effect rows, 1 ms burned, 2.80 ms remainder; density runs pad to sim step share ceiling; L1/L2 built here first, half each way, 2 % each direction independently.
- SPIKE_PLAN:179-183 builds first the client harness (page + Worker host + plain renderer, on the throwaway sim; ADR 0002's spike text); closes at 30 Hz; 60 Hz run reuses kept server and harness in its own VM session.
- SPIKE_PLAN:184-186 size L; 194 R5-TEST-REC4 and REC5 built here first; 199 decides R5-TEST-OQ2.
- 0002:136-148 layout: spikes/ at root; nothing in crates/ or packages/ imports spikes/; 0002:210 dependency rule "nothing imports legacy/ or spikes/". 0002:157-170 pins: rust-toolchain.toml one exact stable (1.99.1 when read), clippy, rustfmt, wasm32-unknown-unknown, profile minimal; resolver 3, edition 2024; Node pinned like legacy pins 22 (TL17); TypeScript 6.0.x exact; wasm-bindgen CLI = Cargo.lock version, no wasm-pack. three.js version: ADR 0003's (0002:958-960). These pins are the foundation's (part 2 Need: GLOSSARY §8 foundation only) — whether spikes must use them: not found in 0002.
- 0002:944-947 spikes exempt now under SPIKE-SCOPE (a).
- 0002:629-634 proof test's room server on x86-64 RS; LG Arm only; bot clients' aarch64 build is the spike's own; no verdict rests on native aarch64 agreeing with x86-64 or Wasm.
- 0002:1040-1043 macOS runner Safari check is the spike's own; leaves OQ-0002-5 open.
- Block 1 (ADR 0002) DONE. Block 7 (SPIKE_PLAN item 1 + shared fixtures) DONE.
- 0001:567-580 part 13: profiles P0=0/no jitter/0 %; P1=50 ms ±10 ms 1 %; P2=100 ±20 2 %; P3=150 ±30 5 %; P4=200 ±40 5 %; B1=P2 + one 1 s burst at 100 % loss; every run also reordering. Each direction: half latency + uniform jitter ± half its jitter; each direction loses at profile's loss rate independently; P2 = 2 % each way, ~4 % of round trips. 0001:1436-1444 key: at P2 50 ± 10 ms each way; expected arrival = send + one-way delay; lateness counted from it; 1 - 0.98*0.98 = 3.96 %.
- 0001:1447-1450 reordering = what the sim produces under jitter; each run records measured reorder share; a reorder mark counts only if share > 0 (proposed; R2 names no rate). Duplication rate: not found in 0001. Rate limit values: PM-0001-23 128 kbps for 10 s; PM-0001-24 bottleneck at 50 % of snapshot stream rate.
- 0001:1204-1215 OQ-0001-16 answered (a); arithmetic: ±20 ms on downlink -> 8.3 % late vs 3 % mark; split matters.
- 0001:1410-1411 L1 = simulated time, any machine or CI: logic marks only (counts, shares, hashes), never a time mark.
- 0001:1337-1346, 1381-1384 LG aarch64: no verdict rests on aarch64 agreeing; rows -17,-19,-48,-54: verdict from L1/x86-64/browser, or LG mismatch counts only if replayed on L1 x86-64.
- 0001:1451-1464 S30: 60 s warm-up after last client's first full state ack, then 30 min = 54,000 ticks; tick counter starts 1,800 below 65,536; portal changes 1/s (3.3 % of ticks at 30 Hz).
- 0001:1465-1472 stats: nearest rank ceil(0.99 n), 53,460th of 54,000; no-statistic time mark = p99; p99s never added; one-off = plain limit; share = count over whole sample; bandwidth no statistic = mean; kbps 1,000 bit/s; MB 10^6.
- 0001:1473-1485 timers: monotonic clock per tick+phase, counters always on; clock resolution measured once per machine (smallest nonzero diff over 10^6 reads); CPU shares from /proc per-thread every second; L2 client times use server-clock estimate with PM-0001-47 error bound.
- 0001:1486-1498 V-S: timed server mark 3 runs all must meet; valid if tick thread alone on vCPU, steal <= 1 %, LG inputs 30/s within 1 %, each LG CPU < 80 % every second; rooms-per-VM rows use PM-0006-13 rules; "clean run" = median of 3 within 2.5 %; invalid run neither pass nor fail.
- 0001:1499-1506 Bites: each automated mark's planted defect must fail it in a 5-minute run (proposed); Rule B-S busy-wait = mark - measured p99 + 10 % of mark; Rule B-N padding = margin + 10 % of mark; no retries.
- 0001:1728-1730 Test plan network: L1 + L2 running P0-P4 and B1, each with reordering. 1735-1739 counter-wrap L1 gate test. 1740-1743 load/soak: bots on client-endpoint, 30 min at P2 per mode, 3-hour soak before playtest. 1744-1751 hostile input suites. 1754-1755 browsers.
- 0001:1687-1693 migration step 1 done-when: marks outside 60 Hz group met + recorded at 30 Hz, WT crate, E3/E3b, process model, datagram cap, rewind cap named; Brave checklist passed.
- 0001:1697-1698 foundation: loopback with L1 behind it in CI.
- Block 3 (ADR 0001 test/migration/part 13/key) DONE; next: PM-0001-2/-3 rows + bites extraction.
- 0001:1513 PM-0001-3 (simulator precondition): P1-P4 + B1 on L1 AND L2, each direction, loss/mean one-way delay/jitter over 100,000 packets within 5 % relative + RTT mean within 5 %; L1 same seed -> identical fate log on 2 runs; P0 on L2 = real path recorded; MP RTT median 100 ± 5 ms at P2; L2 injected delay per direction "set to half of the profile's RTT minus the path's measured base RTT (median of 10 s of pings at session start; proposed)"; runs once per profile+layer before any mark under it counts and at start of every L2 session. NO planted defect named. CONFLICT: 100,000 packets vs PM-0002-49's 1,000,000 and its note 0002:1335-1338 (100,000 fails a correct sim ~1 seed in 9 at P1). AMBIGUITY: "half of RTT minus base RTT" reads as 50 - base per direction (RTT = 100 - base) vs (100 - base)/2 (RTT = 100).
- 0001:1512 PM-0001-2 counters per tick; bite: load script stops firing after 10 min fails 700 shots/s.
- Bites extracted (0001:1511-1578) via scratch-te/bites.py; rows with NO named planted defect: PM-0001-3, -18, -27 (uses PM-0006-13's), -36, -53 (human), -55 (conditional), -56 (records); rule B-S rows: -4, -5, -6, -9, -11, -44; rule B-N: -12, -13; PM-0001-28 control: wrong cert hash fails 100/100.
- 0004:207-225 part 5: server loop wakes on timer, takes number of due ticks from monotonic clock; catch-up per wake bounded 0.25 s = 7 ticks at 30 Hz; longer stall drops time + records it; overrun + dropped interval counters (ADR 0006 I-12); why (226): Tokio timers ms-granular.
- 0004:1011 S17 (spike = netcode proof test): RS Linux, tick start jitter p99 <= 2 ms over 36,000 ticks (20 min at 30 Hz); after injected 200 ms stall back on schedule within 1 s, never > 7 ticks in one wake; overrun + dropped-time counters.
- 0004:1193 PM-0004-67: at 30 Hz, jitter = actual minus scheduled start, p99 <= 2 ms over 36,000 ticks; 200 ms stall -> on schedule within 1 s (no ticks owed, every later start within jitter mark: proposed), <= 7 ticks per wake (floor(0.25*30)=7); 1 s stall -> <= 7 ticks that wake + dropped time in counter (proposed); counters in metrics; RS (Linux); under proof test's 30 Hz baseline load; planted sleep on tick thread at a fixed tick; monotonic clock; 3 runs of 36,000 ticks each meets. Bite: loop counting ticks from timer period instead of elapsed time. GAP: no bite for the 7-tick cap or the dropped-time counter (e.g. unbounded catch-up; counter never incremented). V-S validity (steal, pinning) not stated for this row.
- 0004:1182 PM-0004-61 online half: boundary spike's Worker vs proof test's server at P2 (ADR 0002 simulator); 20 traps per browser at random ticks; resync within 300 ms at P2 (ADR 0001 join mark); ML Chrome/Edge/Firefox, MAC Safari; bites: host not restarting instance; host reporting trap without reading hook buffer.
- 0004:1197 PM-0004-70 online half: after 60 s and 6 min hidden, online within 0.5 s of tab return client applied server state + commands accepted again, never > 7 ticks in one frame; boundary spike's page vs proof test's server at P2; 5 hides per duration+browser; bite: Worker simulating the whole hidden gap.
- 0004:1234-1235 only -61 and -70 name a profile; -67 runs under proof test's own load.
- So: -61/-70 run in the boundary spike, not this change-set; this change-set must leave the room server + L2 at P2 re-runnable (kept), SPIKE_PLAN:145.
- Block 2 (ADR 0004) DONE; next: GLOSSARY.
- GLOSSARY:141-147 time statistics: no-statistic time mark = p99; mean always called mean; p99s do not add; tick time p99 measured directly; one-off deadline = plain limit.
- GLOSSARY:375-394 test-mode list (MODES-3 (a)): 24v24, 12v12v12; 16v16v16; 1v1, 4v4, 2v2v2, 6v6v6; N = 64 headroom. Largest mode 24v24 N = 48 (335-354).
- GLOSSARY:397-437 stress scene: N = 48 in 16v16v16 one cluster; 15 big vehicles (4 very large); 300 live projectiles; up to 8 effects per soldier (384); one DM_Tri-sized 3D greybox; server mark incl. server-only share puts a snapshot-receiving client on every soldier.
- GLOSSARY:765-781 netcode proof test definition: Rust sim natively as server and as Wasm in the client; WT with WS fallback; 48 simulated clients at 30 Hz within ADR 0001 budgets (tick time, bandwidth per client, memory); one human client at 100 ms simulated latency sees prediction/reconciliation/lag-comp hits that look correct.
- GLOSSARY:782-799 60 Hz margin: not this change-set.
- Block 4 (GLOSSARY) DONE; next r5 REC4/REC5/OQ2.
- r5:1454-1473 R5-TEST-REC4 exact: L1 own link sim behind R2-TRN-REC4 transport interface, seeded, simulated time, per direction (half RTT each way, proposed), latency/jitter/loss/reorder/dup/rate limit (F12) + packet-fate log; real server sim + headless bots in one process. L2 impairment wrapper around server's UDP socket under real quinn endpoint (F13), same profiles, real browsers over WT; WS gets TCP proxy with delay and stalls ("loss on TCP shows as delay"). Fuzz targets every decoder + content loader. Spike must prove (proposed): L1 identical fate log + per-tick hashes on two runs; loss/mean delay/jitter over 100,000 packets within 5 % relative; L2 app RTT vs quinn rtt() within ±5 ms at 100 ms injected RTT (R2-TRN-REC4 mark 2), then reported under P2 jitter+loss; 1-min L1 per profile P0-P4+B1 < 2 min total.
- r5:1475-1489 R5-TEST-REC5 exact: bots headless Rust protocol clients; nightly per mode over L1 at P2 30 min; one L2 run with 4 real browser clients among bots; measured tick time p50/p99/max, bytes per client per s, RSS slope, queue lengths, desyncs (sampled clients' hashes), panics, hangs, TL13 match metrics; 3-hour soak before playtest. Spike mark: 30-min room at N = 48 at P2: 0 panics, 0 hangs, 0 desyncs, memory slope <= 1 MB per 10 min after warm-up, tick p99 inside budget; planted leak (one buffer per snapshot) flagged within run.
- r5:1804-1805 R5-TEST-OQ2 exact: "Can wtransport or web-transport-quinn be built on a custom quinn endpoint, so L2 sits under them? F13 is quinn's; the two candidates (R2-TRN-REC1) were not read for it. Decides it: R2's interop spike." r5:2050 crate docs not fetched. No drafted fallback if the answer is no (tc netem not proposable 0002:846; R5-TEST-OQ6 no OS layer unless L2 fails to reproduce a playtest bug 0002:1608) -> decision.
- Block 5 (r5) DONE; next toolchain.
- Container toolchain (2026-11-05, read-only): rustc 1.97.0 (2026-07-07); cargo 1.97.0; installed targets: x86_64-unknown-linux-gnu only (NO wasm32-unknown-unknown, NO aarch64); toolchains: stable-x86_64 only; node v22.23.4; wasm-pack, wasm-bindgen, wasm-opt: not found; clang: /usr/bin/clang. Pin text 0002:157 says 1.99.1 "when the research read it" (foundation); container is 1.97.0, below it.
- Block 6 (toolchain) DONE; next: ssh-probe read, then write sections A-G.
- PM-0006-11..-27 bites (0006:1349-1365): -13 busy-wait 20 % of single-room p99 in one room; -15 rule added for probed TCP port / SSH open to all; -16 rule open to every source; -17 snapshots sent twice per tick; -19 50 µs busy-wait per logged event; -20 format! in per-tick log; -21 log line with client address + token found; -22 unregistered metric; -23 the stall is the planted event, control 0 overruns before; -24 planted player label; -25 control for -27; -27 control: 14-day cert connects. NO bite: -11, -12, -14, -18, -26.
- Duplication: no rate in any profile and no 0001 mark exercises network duplication (grep "duplicat" in 0001: none); L1 must have it by design (0002:516). Rate limit semantics (queue size, drop-tail vs delay) not found; used by PM-0001-23 (128 kbps 10 s) and -24 (50 % bottleneck) on L2.
- Results-log format: not found (only "results log", "results-log header per machine" 0001:1511, "run's record" 0001:1512).
- ARITHMETIC (reorder): with independent uniform per-direction jitter ±j/2, two packets Δ apart reorder only if Δ < j. A 30 Hz flow (Δ = 33.33 ms): P2 (±10, diff range 20) 0 %; P3 (±15, range 30) 0 %; P4 (±20, range 40) (40-33.33)^2/(2*40^2) = 1.39 % of consecutive pairs. So PM-0001-20 "10,000 scripted presses with reordering" under P3 on a one-datagram-per-tick command flow sees 0 reorder, and 0001:1448-1449 says a reorder mark counts only if share > 0 -> cannot count. Reorder appears only for back-to-back datagrams (split snapshots, pings, acks).
- ARITHMETIC (PM-0001-3 count): SE of loss at 100,000 packets: P1 0.0315 pp vs band 0.05 pp (1.59 SE, ~11 % of correct seeds fail: 0002:1335-1338's "1 in 9"); P2 0.0443 pp vs 0.10 pp (2.26 SE, ~2.4 %); P3/P4 0.0689 pp vs 0.25 pp (3.6 SE, ~0.03 %). On L2 (real time, not replayable) this makes a correct impairment fail P1 about 1 run in 9 -> a flake by construction.
- Jitter statistic for the 5 % band: not defined in 0001 (PM-0001-3) nor 0002 (PM-0002-49: "the distribution L1's own documentation states (proposed: R2 names none)"); uniform ±10 ms has SD 10/sqrt(3) = 5.774 ms (arithmetic).
- B1 for PM-0001-3: expected per-direction loss value not stated (depends on packet count and burst count; PM-0001-18 uses 10 bursts per run, proposed); whether the burst hits both directions at once: not found.
- tools/gate.mjs:34 FILES = the five legacy files only -> today's gate never reads spikes/. .github/workflows/gate.yml: on push + pull_request, no path filter, jobs gate + smoke on ubuntu-24.04, Node 22, no Rust job -> CI builds/tests nothing in spikes/. spikes/ does not exist yet.
- PM-0001-15 (0001:1526) bite "flagged within the run" (S30, 30 min) vs key's 5-minute bite run (0001:1499-1500); its marks need >= 20 min (RSS at 20 min; queues last 10 vs first 10 min) -> bite run length conflict for memory/queue marks.
- Approx. count (my tally of 0001/0004/0006 rows run on RS/LG with a named bite or rule B-S/B-N): about 42 planted runs -> about 3.5 h at 5 min each, before builds and swaps (an estimate for the cost cap, not a draft number).
- Block 8 (sections) next.

---
# DIGEST (sections A-G)
Keys: 0001 = docs/adr/0001-multiplayer-netcode.md, 0002 = docs/adr/0002-code-structure-build-toolchain.md, 0004 = docs/adr/0004-simulation-core.md, 0006 = docs/adr/0006-infrastructure-operations.md, SP = docs/SPIKE_PLAN.md, GL = docs/research/GLOSSARY.md, r5 = docs/research/r5-toolchain-quality.md. "REC:" = my recommendation, never settled.

## A. The link simulator as designed
| Item | L1 | L2 | Source |
|---|---|---|---|
| Where it sits | behind the thin transport interface (R2-TRN-REC4's), our own code | an impairment wrapper around the server's UDP socket, under the real quinn endpoint; WebSocket: a delaying TCP proxy ("delay and stalls"; "loss on TCP shows as delay") | 0002:515-521; r5:1454-1459; 0001:1444-1445, 1728-1730 |
| Time | simulated time | real time | 0002:516, 843-845 |
| Determinism | seeded; same seed + profile -> identical packet-fate log (PM-0001-3) and identical per-tick hashes (R5 mark; PM-0002-49) | not seeded-replayable (real time) | 0001:1513; r5:1469-1470; 0002:1301 |
| What runs on it | the real server code + headless bot clients in one process | real browsers over WebTransport; LG's clients | 0002:519-521; 0001:1444-1445 |
| Injects | latency, jitter, loss, reordering, duplication, a rate limit, packet-fate log | "the same profiles" (r5); rate limit used by PM-0001-23/-24 on L2 | 0002:516-517; r5:1456-1459; 0001:1536-1537 |
| Marks allowed | logic only (counts, shares, hashes), never a time mark; "any machine or CI" | time marks on RS | 0001:1410-1411 |
| Gate later | G11: one minute per profile P0-P4 + B1, under 2 min total (foundation, not this change-set) | nightly | 0002:580, 1301, 496 |
| Rejected | turmoil (QUIC on it unread; R5-TEST-OQ5: not added) | OS-level `tc netem` ("docs not read: not proposable"; R5-TEST-OQ6: no OS layer unless L2 cannot reproduce a playtest bug) | 0002:844, 846, 1607-1608 |

**Profiles** (0001:567-572, 1436-1443): P0 = 0 ms, no jitter, 0 %; P1 = 50 ms, ±10 ms, 1 %; P2 = 100 ms, ±20 ms, 2 %; P3 = 150 ms, ±30 ms, 5 %; P4 = 200 ms, ±40 ms, 5 %; B1 = P2 plus one burst of 1 s at 100 % loss. Latency is the round trip. **Per direction:** half the latency, a uniform jitter of ± half the jitter, the profile's loss drawn on its own: at P2, 50 ± 10 ms each way and 2 % each way, independently; about 4 % of round trips lose a packet (1 − 0.98 × 0.98 = 3.96 %). Expected arrival = send time + that one-way delay; every lateness counts from it (0001:1443-1444). Settled by OQ-0001-16 (0001:1204-1207) and SP:165-167. Every run "also reordering" (0001:567); reordering = "what the simulator produces under the profile's jitter", each run records its measured reorder share, a reorder mark counts only if the share > 0 (0001:1447-1450, proposed).
**Not found in the drafts:** a duplication rate for any profile (no 0001 mark tests network duplication); the rate limiter's queue (size, drop-tail or delay); B1's expected per-direction loss for PM-0001-3 and whether its burst hits both directions; which jitter statistic the 5 % band compares (PM-0002-49: "the distribution L1's own documentation states (proposed: R2 names none)", 0002:1301).
**R5-TEST-OQ2** (r5:1804-1805, exact): "Can wtransport or web-transport-quinn be built on a custom quinn endpoint, so L2 sits under them? F13 is quinn's; the two candidates (R2-TRN-REC1) were not read for it. Decides it: R2's interop spike." Assigned to this test (0002:1604; SP:199). No drafted fallback if neither crate allows it.
**PM-0001-3** (0001:1513), the simulator's precondition: P1-P4 + B1, on L1 and on L2, each direction, loss / mean one-way delay / jitter over 100,000 packets within 5 % (relative), round-trip mean within 5 %; L1 fate-log identity on two runs; P0 on L2 = the real path, recorded; MP's server-measured RTT median 100 ± 5 ms at P2; L2's injected delay per direction "set to half of the profile's RTT minus the path's measured base RTT (median of 10 s of pings at session start; proposed)"; runs once per profile and layer before any mark under that profile counts, and at the start of every L2 session. No planted defect named.
**R5's L2 mark** (r5:1470-1472): app-level RTT vs quinn's server-side `rtt()` within ±5 ms at a 100 ms injected round trip, then reported under P2 (it is PM-0001-33 in 0001:1548).

## B. The client harness
**What the drafts say (exact text, by line):**
- SP:179-181: "It builds first the client harness the boundary spike shares (the page with its Worker host and a plain renderer, on the throwaway sim: ADR 0002's spike text), accepted with the order (J6 (5))."
- 0002:1115-1116: the boundary spike "shares the netcode proof test's client harness and stand-in sim, and the sim core spike's sim, which exists by then, serves the marks that need a sim beyond the stand-in".
- 0002:1199-1205 (BX): "the netcode proof test's throwaway sim (movement, hitscan, bots as protocol clients) in the Worker ... The page draws the blocks with the shared client harness's plain renderer, not the look (the look is ADR 0003's)."
- 0002:1211-1213: "Online: the Worker's client endpoint connected to the netcode proof test's room process over WebTransport through that test's link impairment, at the profile the row names".
- GL:768 (the test's definition): "the Rust simulation runs natively as a server, and as Wasm in the client".
- The Worker host's shape (foundation design the boundary spike measures): one input call + one step call per tick; datagrams carried in the input call, each stamped with the host's `performance.now()` in its own message event; the step returns when the next tick is due and the host sets its timer from it; catch-up at most 7 ticks per wake; a fresh view of `memory.buffer` after every call (0002:325-341); the Worker owns the connection online, the page sees only the block (0002:369-373, H2); the local-match order (0002:394-403).
**What this test's own marks need from it** (0001 rows on ML/MAC/MP): prediction + reconciliation (PM-0001-41/-42/-60), interpolation and extrapolated-frame counts (-45/-46/-61), the client clock with message-event stamps (-47 bite: "receive times quantised to the tick"; -33), input-to-screen through presented frames (-43), the aim cast from what the page drew (-51), damage shown only on the server's event, logged by the page (-52), own cues once (-62), the four-browser path on both backends (-28, -29, -32, -36).
**Left to the boundary spike** (SP:119; 0002:1119-1175, 1249-1281): PM-0002-1 to -34, -77, -78 (the counting build 0002:1232-1235, one copy per tick, page allocations, B2 against B3, generated types, isolation, the page never handling a packet PM-0002-22, the busy page PM-0002-23, size and tools, `file://` editor, Sentry), and ADR 0004's PM-0004-8, -28, -61, -68 to -70 (the Worker's pacing, hidden tabs, trap recovery).
REC: build the host to part 5/6's shape now (so the boundary spike measures the code it will keep), but judge none of PM-0002-*; skip the counting build and B3.

## C. spikes/ rules, pins, gate and CI; the container
| Rule | Text | Source |
|---|---|---|
| Throwaway, in `spikes/`, exempt from the legacy hard constraints | SPIKE-SCOPE (a) | SP:12-13; 0002:39-40, 944-947 |
| Own change-set, plan first; one heavy spike at a time | CLAUDE.md protocol | SP:13-15 |
| Server numbers at 30 Hz; N = 48 holds over 54 | TICK-30, MODES-3 (a) | SP:15-17 |
| Layout | `spikes/` at the root beside `crates/`, `packages/`, `legacy/` (after the foundation's restructure) | 0002:140-141 |
| Import ban | nothing in `crates/` or `packages/` imports `spikes/`; a gate rule from the foundation | 0002:144-145, 210 |
| Kept | "the proof test's kept server and harness" reused by the 60 Hz run, in its own VM session after the sim core spike; the boundary spike reuses the harness, the throwaway sim and the room process; PM-0004-61/-70's online halves run against "the netcode proof test's server at P2" | SP:106-107, 118, 182, 275; 0004:1182, 1197 |
| How "kept" is kept (branch, tag, merged to main?) | not found | – |
| Toolchain pins for spikes | not found: 0002 part 2's pins are the foundation's (Need: "GLOSSARY §8, the foundation only", 0002:179) | 0002:157-179 |
| Foundation pins (for reference) | `rust-toolchain.toml`: one exact stable Rust ("1.99.1 when the research read it"), clippy, rustfmt, `wasm32-unknown-unknown`, `profile = "minimal"`; `resolver = "3"`, `edition = "2024"`; `Cargo.lock` committed, `--locked`; Node pinned as legacy pins 22 (TL17); TypeScript 6.0.x exact; wasm-bindgen CLI = `Cargo.lock`'s version, not wasm-pack; three.js version is ADR 0003's | 0002:157-170, 958-960 |
| Crates | "quinn and the WebTransport crate the netcode proof test picks" | 0002:239 |
| LG's build | the bot clients' aarch64 build "is that spike's own" | 0002:630-632 |
| Today's gate | checks only the five legacy files (`FILES`, tools/gate.mjs:34): never reads `spikes/` | tools/gate.mjs:34 |
| Today's CI | `.github/workflows/gate.yml`: push and pull_request, no path filter, jobs `gate` and `smoke` on ubuntu-24.04 with Node 22, no Rust job: nothing in `spikes/` is built or tested by CI | gate.yml |
| Safari runner | a GitHub macOS runner stands as MAC for yes/no and count rows only once (i) Safari >= 26.4, (ii) real Safari via safaridriver, (iii) one WT session to RS carrying datagrams + one WS session on the domain are recorded; time rows stay on a borrowed Mac; the workflow is this plan's, reviewed by security-auditor + infra-engineer; any `.github/` change goes to Mustafa | SP:45-57; 0001:1394-1404; 0002:1040-1043 |

**The container (read 2026-11-05, read-only):** rustc 1.97.0 (2026-07-07), cargo 1.97.0; installed targets: `x86_64-unknown-linux-gnu` only (no `wasm32-unknown-unknown`, no `aarch64-unknown-linux-gnu`); toolchains: stable x86_64 only (no nightly for cargo-fuzz); node v22.23.4; wasm-pack, wasm-bindgen, wasm-opt: not on the PATH; clang: /usr/bin/clang. Plus (ssh-probe.md): no ssh client, no egress but HTTPS 443 through a TLS-re-terminating gateway; arbitrary public HTTPS hosts readable; no WebSocket upgrade, no QUIC out. Not checked: whether `rustup target add`, `cargo install` and Playwright's Chromium work here (they need egress to static.rust-lang.org / crates.io, which the probe suggests is allowed: unverified).

## D. S17 / PM-0004-67; the online halves of PM-0004-61 and -70
| Mark | Number | Setup | Runs; bite | Source |
|---|---|---|---|---|
| Design (part 5) | loop wakes on a timer, takes due ticks from a monotonic clock; catch-up per wake ≤ 0.25 s = 7 ticks at 30 Hz (⌊0.25 × 30⌋); a longer stall drops the time and records it; overrun + dropped-interval counters | server room loop | – | 0004:207-211 |
| PM-0004-67 (S17) | 30 Hz: tick start jitter (actual − scheduled) p99 ≤ 2 ms over 36,000 ticks; after an injected 200 ms stall, on schedule within 1 s ("no ticks owed, and every later start within the jitter mark": proposed) and ≤ 7 ticks in one wake; after an injected 1 s stall ≤ 7 ticks that wake and the dropped time in the counter (proposed); counters in the metrics | RS (Linux); the proof test's room under its 30 Hz baseline load; "a planted sleep on the tick thread at a fixed tick"; the loop's monotonic clock | 3 runs of 36,000 ticks, each meets. Bite: "a loop that counts ticks from its timer period instead of elapsed time" | 0004:1193, 1011 |
| PM-0004-61 online half | after a trap in the client's Worker: a fresh instance holds the server's state (client-held hash = server's), resync within 300 ms at P2; 20 of 20 hook messages arrive whole | ML Chrome/Edge/Firefox, MAC Safari; the boundary spike's Worker against this test's server at P2 (ADR 0002's simulator); 20 traps per browser at random ticks | bites: a host that does not restart; a host that reports without reading the hook's buffer | 0004:1182 |
| PM-0004-70 online half | after 60 s and 6 min hidden: within 0.5 s of the tab's return the client applied the server's state and its commands are accepted, never > 7 ticks in one frame | ML, MAC; the boundary spike's page against this test's server at P2 | 5 hides per duration and browser; bite: a Worker that simulates the whole hidden gap | 0004:1197 |
Only -61 and -70 name a profile; -67 runs under this test's own load (0004:1234-1235). -61 and -70 are judged in the boundary spike (SP:119); this change-set only has to leave the server and L2 at P2 re-runnable for them (SP:145).
Gaps: -67 names one bite for three sub-marks: the 7-tick cap and the dropped-time counter have none (REC, proposed: an uncapped catch-up loop; a drop that never increments the counter). -67 states no validity rule; 0001's V-S (tick thread alone on its vCPU, steal ≤ 1 %, 0001:1486-1490) covers 0001's timed marks (REC: apply V-S to -67; decision for the advisor). Whether -67's 36,000 ticks may be read from the S30 soaks (the stall runs must be separate, or they break the 0-overrun marks): decision.

## E. The evidence plan (CLAUDE.md: "netcode → deterministic tests with injected latency, jitter and loss")
| Group | What | Where it can run | Marks it serves |
|---|---|---|---|
| E1 unit (cargo test) | L1's samplers (per-direction delay, uniform jitter, loss, reorder, dup, rate limiter, burst), fate-log writer, codec, quantisers, wrap-aware compare | container | building blocks of PM-0001-3, -17, -57, -58 |
| E2 property (proptest, fixed seed) | every sent packet has exactly one fate; delivered time ≥ send time; delivered + dropped = sent + duplicated; decoders never panic; wrap compare is antisymmetric across 2^16 | container | PM-0001-3, -34, -58 |
| E3 deterministic L1 (seeded, 48 headless clients, simulated time) | PM-0001-3 (L1), -17, -57, -58, -18 (3 seeds), -20, -21, -22, -41, -42 (snapshot share), -45 (per-tick count), -47, -48, -49, -50, -51, -54/-55 (if their condition holds), -60, -61, -62; -30's hash on loopback | container (Linux x86-64): L1 is "any machine or CI", logic only (0001:1410-1411); also the x86-64 replay that judges LG mismatches (0001:1343-1345) | as listed |
| E4 any-machine | -34 fuzz (needs a nightly: missing here), -35 dependency check | container (fuzz after a nightly is installed) | -34, -35 |
| E5 Wasm + harness | sim native vs Wasm hash; harness page + Worker on loopback | container only if wasm32 + wasm-bindgen CLI + Playwright Chromium + local QUIC work (unverified) | GL:768; rough check for -28 |
| E6 L2 (real time) | impairment socket + TCP proxy on RS; LG's clients; ML, MAC, MP browsers | RS, LG; browsers | -3 (L2), -17, -18, -19, -20, -23, -24, -31, -33, -37, -38, -48, -49, -54 |
| E7 timed server marks (V-S, 3 runs) | single room, tick thread pinned | one vCPU of RS | -4 to -16, -26, -59; PM-0004-67; PM-0006-17, -19, -20 |
| E8 rooms per VM | 8 rooms, unpinned, PM-0006-13's rules, 3 sessions at 3 times of day | RS + LG | -27; PM-0006-13, -14 |
| E9 soaks | S30: 60 s warm-up, 54,000 measured ticks, tick counter from 1,800 below 65,536, doors 1/s | RS + LG | -15 and every S30 row |
| E10 browsers + people | ML (rough checks only until OQ-0003-1 names it), MAC (macOS runner: yes/no and counts only), MP Brave checklist (-53), operations checklist (PM-0006-12, -18) | Mustafa's machines; GitHub macOS runner | -28, -29, -32, -36, -43 to -46, -52, -53 |
| E11 planted defects | each automated mark's bite on the spike's own build, 5-minute run (0001:1499-1501); rules B-S and B-N | where its mark runs | section F |
| E12 checker self-test | the verdict script run on fixed fixtures: one that must PASS, one FAIL, one INVALID | container | every scripted mark |
**This container** (no SSH, HTTPS-only egress, no QUIC out): E1-E4, E12 and probably E5; it can read results a VM publishes on 443 with a browser-trusted certificate (ssh-probe.md, second pass). **RS/LG:** E6-E9 and their bites, driven by cloud-init or by Claude Code on Mustafa's PC. **Browsers:** E10.
**Recording (not found in the drafts beyond "results log", "results-log header per machine" 0001:1511 and "the run's record" 0001:1512).** REC: per run, one JSON Lines results log: a header record (PM-0001-1's fields, build commit and binary hash, profile, seed, clock resolution 0001:1475-1476, affinity map), per-tick records (scheduled and actual start, phase times, counters), per-second records (per-thread CPU from `/proc`, steal, RSS, queue lengths, bytes per client, inputs per client), event records (joins, bursts, stalls, door changes, planted-defect id), a footer (exit code); the packet-fate log as a separate file (direction, flow, sequence, send time, fate, delay), kept under `.gate/`, only summaries and hashes committed. A checker reads a committed mark table as data (id, statistic, threshold with its draft line, sample window, validity clauses) and writes one verdict per mark: PASS, FAIL, INVALID (V-S failed: "neither a pass nor a fail", 0001:1497-1498), STAND-IN (PM-0001-4 when the stand-in sim is lighter, 0001:1432-1435), REPORTED, NOT-RUN. Percentiles by nearest rank, ⌈0.99 n⌉ (0001:1465-1466). A threshold in that table that differs from the drafts is a decision, never an edit. No retries: a failing run keeps its data, seed, commit, machine, browser, profile (0001:1505-1506).

## F. Planted defects the drafts list for this test (each must fail its mark before the pass counts; all "proposed", 0001:1501)
| Mark | Planted defect | Line |
|---|---|---|
| PM-0001-1 | a single-room run with a second room's tick thread on the same vCPU is flagged invalid | 0001:1511 |
| -2 | a load script that stops firing after 10 minutes fails the 700 shots per second | 1512 |
| -4, -5 (each share), -6, -9, -11 (send call), -44 | rule B-S: busy-wait = mark − measured p99 + 10 % of the mark | 1502-1503, 1515-1522, 1563 |
| -12 (each scene), -13 | rule B-N: padding = measured margin + 10 % of the mark | 1503-1504, 1523-1524 |
| -7 | a history kept for the whole match fails the 1 MB | 1518 |
| -8 | an accumulator that ranks by entity id fails "the nearest 8" | 1519 |
| -10 | an encoder whose work grows with N² | 1521 |
| -14 | an entity group that is never split | 1525 |
| -15 | R5's planted leak (one buffer kept per snapshot), flagged within the run | 1526 |
| -16 | a start that loads its mode file twice over a 1 s delay | 1527 |
| -17 | a 16-bit tick in the snapshot header; a client quantiser that rounds differently | 1528 |
| -57 | positions at 4 cm; yaw in 12 bits; pitch in 8 bits | 1529 |
| -58 | a plain `<` instead of the wrap-aware compare; the header tick narrowed to 16 bits | 1530 |
| -19 | the first full state sent as datagrams | 1532 |
| -20 | a redundancy cap of 1 | 1533 |
| -21 | each check removed in turn; the tick check relaxed to "not older than"; a refusal in place of the clamp | 1534 |
| -22 | an unbounded event queue | 1535 |
| -23 | the own state ranked like any entity | 1536 |
| -24 | a blocking send that queues every snapshot | 1537 |
| -25 | the library's default limits; the `Join` deadline from the upgrade's end; per-address limits keyed by the whole IPv6 address | 1538 |
| -26 | `panic = "abort"`; a shared `std::sync::Mutex` poisoned by a handler panic | 1539 |
| -59 | a tick-thread panic that leaves the process running | 1540 |
| -28 | control: a wrong certificate hash fails 100 of 100 | 1543 |
| -29 | no WebTransport timeout | 1544 |
| -30 | a wall-clock read on one transport's command path; validity: a command delayed past its tick makes the run invalid | 1545 |
| -31 | a server that queues every snapshot over WebSocket | 1546 |
| -32 | the datagram writer detected by browser name | 1547 |
| -33 | pongs answered from the tick thread; the client's pong without its hold time | 1548 |
| -34 | an unchecked index in one decoder | 1549 |
| -35 | `quinn` added to the sim crate; quinn-proto pinned at 0.11.17 | 1550 |
| -37 | a send queue that holds each snapshot for 3 intervals | 1552 |
| -38 | bulk data on the control stream | 1553 |
| -41 | the server's run speed 1 % above the client's; the server quantising its own copy differently | 1559 |
| -42 | smoothing off | 1560 |
| -60 | the own-state section without the changed value | 1561 |
| -43 | own movement drawn from snapshots only | 1562 |
| -45 | R5-GATE-REC3's G11 mutation, "a snapshot buffer one slot too short for P2" | 1564 |
| -46 | respawns interpolated like movement | 1565 |
| -61 | remote events released on arrival into the newest block | 1566 |
| -47 | an estimator taking the newest raw sample; receive times quantised to the tick | 1567 |
| -48 | a rewind without the interpolation delay; an estimate without the buffer wait | 1568 |
| -49 | the cap applied to the estimate only; a client delaying every pong by 150 ms must still meet the cap | 1569 |
| -50 | the view time accepted without the tolerance check | 1570 |
| -51 | the server's ray parallel to the client's with a planted 10 cm fire-origin error | 1571 |
| -52 | damage shown on the client's own predicted hit | 1572 |
| -62 | cues written from the replay; the server echoing the owner's shots | 1573 |
| -54 | a client trajectory step of 1/60 s against the server's 1/30 s | 1576 |
| PM-0004-67 | a loop counting ticks from its timer period | 0004:1193 |
| PM-0006-13 (and -27 through it) | a busy-wait of 20 % of the single-room p99 in every tick of one room | 0006:1351 |
| PM-0006-15 / -16 | a rule added for the probed TCP port; SSH open to every source / a rule open to every source | 0006:1353-1354 |
| PM-0006-17 | snapshots sent twice per tick | 0006:1355 |
| PM-0006-19 / -20 | a 50 µs busy-wait per logged event / a `format!` in a per-tick log call | 0006:1357-1358 |
| PM-0006-21 / -22 | a log line with one client's address and one token / one metric unregistered | 0006:1359-1360 |
| PM-0006-23 / -24 | the stall is the planted event (control: 0 overruns first) / a planted `player` label | 0006:1361-1362 |
| PM-0006-25 / -27 | control pair: a 14-day certificate connects; a rule-breaking one is refused | 0006:1363, 1365 |
**No bite named:** PM-0001-3 (the simulator itself), -18, -27 (only via PM-0006-13), -36, -55; PM-0006-11, -12, -14, -18, -26; PM-0004-67's cap and counter. REC (proposed, for the advisor): for PM-0001-3, (a) the full round trip applied each way fails the mean-delay band, (b) loss drawn once per round trip fails the per-direction loss, (c) a normal jitter in place of the uniform fails the jitter check, (d) the RNG seeded from the clock fails the fate-log identity, (e) on L2, the delay not net of the base RTT fails MP's 100 ± 5 ms; for -18, a baseline that never falls back (deltas only against the last acknowledged snapshot) fails the 500 ms.

## G. Problems (ranked) and a recommended order
**Problems: each a decision for the advisor or Mustafa unless marked REC**
1. **Packet count:** PM-0001-3 says 100,000 packets (0001:1513; r5:1470); PM-0002-49 says 1,000,000 because 100,000 fails a correct simulator about 1 seed in 9 at P1 (0002:1301, 1335-1338). On L1 a seed then decides the verdict; on L2 (real time, no replay) it is a flake by construction (my arithmetic: P1 1.59 SE, about 11 % of runs; P2 2.26 SE, about 2.4 %). REC: 1,000,000 per profile and direction on both layers.
2. **Reordering never happens on a 30 Hz flow at P2 or P3** under independent uniform half-split jitter (arithmetic: packets 33.33 ms apart reorder only if the jitter range exceeds 33.33 ms; P2's range is 20 ms, P3's 30 ms, P4's 40 ms gives 1.39 % of pairs). So PM-0001-20's "10,000 scripted presses with reordering" under P3 (0001:1533) sees 0 reordered command datagrams, and by 0001:1447-1450 cannot count. Options: an explicit reorder share per profile (R5 lists reordering as its own capability, r5:1456; a rate needs a number: not found), or count reorder only on back-to-back datagrams (split snapshots, pings).
3. **Rows judged on ML cannot close the test** until OQ-0003-1 names the laptop (rough checks only: 0001:1390-1392; SP:58-61), yet migration step 1 is done only when every mark outside the 60 Hz group is met (0001:1689-1691): -43, -44, -45, -46, -52 and ML's halves of -28, -29, -32, -36, -42. Decide whether the change-set closes with those rows "pending ML" or waits.
4. **R5-TEST-OQ2 has no fallback.** If neither WebTransport crate can sit on a custom quinn endpoint, the drafts offer no L2 (netem not proposable, 0002:846; R5-TEST-OQ6, 0002:1608). REC: answer it first, in this container, before any VM hour; options to name now: (a) the crate that allows it; (b) a user-space UDP relay in front of the server (it changes the source address the server sees, which PM-0001-25's per-address limits read); (c) read netem's docs and propose it.
5. **Simulator spec gaps** (not found): the jitter statistic for the 5 % band; B1's expected loss and whether its burst cuts both directions; the rate limiter's queue (size, drop or delay: PM-0001-24's "never older than 3 intervals" could fail on the simulator's own queue); a duplication rate (L1 has it by design, 0002:516; no mark uses it). REC: the plan proposes each, labelled proposed, for the advisor.
6. **L2's delay formula is ambiguous** (0001:1513): "half of the profile's RTT minus the path's measured base RTT" gives 50 − b per direction (round trip 100 − b) or (100 − b) ÷ 2 (round trip 100). Only the second meets MP's 100 ± 5 ms once his base RTT exceeds 5 ms. Advisor to confirm the reading.
7. **Bite length:** the key's 5-minute bite run (0001:1499-1500) cannot show PM-0001-15's leak "within the run" (S30), its 20-minute read or its 10-minute windows (0001:1526). Decide the bite run's length for memory and queue marks.
8. **Rows with no planted defect** (section F) cannot be shown to fail: PM-0001-3, -18, -36, -55; PM-0006-11, -12, -14, -18, -26; PM-0004-67's cap and counter. REC: the proposed bites in F.
9. **No machine guards `spikes/`:** today's gate reads only the five legacy files (tools/gate.mjs:34) and CI has no Rust job (gate.yml). REC: the spike's own test command, run in-session at every step with its output in the checkpoint; any CI job for it (and the Safari workflow) is a `.github/` change for Mustafa's approval.
10. **Container toolchain:** rustc 1.97.0 with no wasm32 or aarch64 target, no nightly, no wasm-bindgen CLI or wasm-opt; spike pins are not fixed by the drafts. REC: the plan pins its own exact toolchain in `spikes/<name>/rust-toolchain.toml` (with wasm32), `Cargo.lock` + `--locked`; LG's aarch64 clients built natively on LG from the same commit (no cross linker needed); every binary's hash in the results header.
11. **"Kept" is undefined** (SP:106-107, 182). REC: the server, harness, L1/L2 and checker merge on main under `spikes/`, with the commit SHA written in ADR 0001's results, so the 60 Hz run and the boundary spike start from one known commit.
12. **PM-0004-67:** no validity rule and no statement whether its 36,000 ticks share the S30 runs (D). REC: V-S applies; the jitter read from 3 S30 runs, the stall runs separate.
13. **VM hours:** about 42 planted runs on RS/LG at 5 minutes each, about 3.5 h before builds and swaps (my estimate for the cost cap), plus PM-0006-13's 3 sessions at 3 times of day (at least 3 VM sessions).

**Recommended order, small resumable units (REC)**
1. Container probes, one unit each, results to the notes: install the wasm32 target and the wasm-bindgen CLI; a nightly for cargo-fuzz; Playwright's Chromium; a WebTransport session over loopback QUIC inside the container.
2. `spikes/` skeleton, pins, results-log format, the mark table as data, the checker and its PASS/FAIL/INVALID fixtures (E12).
3. L1 with E1/E2 tests; PM-0001-3's L1 half with the proposed simulator bites; decide items 1, 2 and 5 before this unit closes.
4. R5-TEST-OQ2 on loopback: both crates on a custom quinn endpoint over the impairment socket; the TCP proxy; L2's shape fixed (item 4).
5. Throwaway sim, protocol, room and headless client endpoint; then the L1 row groups one per unit, each with its bites: codec (-17, -57, -58); commands and hostile input (-20, -21, -22); prediction (-41, -42, -60); interpolation, clock and events (-45, -47, -61, -62); lag compensation (-48 to -51); burst (-18); any-machine rows (-30 on loopback, -34, -35).
6. Wasm build, native-against-Wasm hash, the client harness (page, Worker host, plain renderer), a local rough check.
7. The run-day package (cloud-init or Mustafa's PC), the run plan with hours and cost cap, the DNS records; VM session 1: preconditions (-1, -2, -3 on L2; PM-0006-11, -12) and the 5-minute calibration; then timed single-room marks with their bites; then L2 rows, S30 soaks with PM-0004-67, rooms per VM over 3 sessions, the operations marks.
8. Browsers: the macOS-runner check (workflow to Mustafa first), ML rough checks, MP's Brave checklist.
9. Verdicts into ADR 0001's record; the kept commit named.
- Block 8 (sections A-G) DONE.
DONE
