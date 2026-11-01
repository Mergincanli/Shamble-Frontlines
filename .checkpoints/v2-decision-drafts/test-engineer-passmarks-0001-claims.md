# Claims ledger: ADR-0001's pass marks (test-engineer), 2026-11-01

A notes file beside `test-engineer-passmarks-0001.md`, in the plan's ledger shape (rule 9). It joins
`netcode-architect-adr0001-claims.md` [C15]; rows are numbered TE1, TE2 … so they never clash with that ledger's
1–360. One row per claim or number in the pass-mark subsection and the spike entry. Lines are at 9e7db28 (the research
files are unchanged at the head: `git diff --quiet 9e7db28 HEAD -- docs/research/` exits 0, and GLOSSARY.md is still
its 742 lines, so §8 is cited from the decision map's part j, J1). Short names: r1 = `docs/research/r1-simulation-core.md`,
r2 = `r2-netcode.md`, r3 = `r3-rendering.md`, r5 = `r5-toolchain-quality.md`, r6 = `r6-infrastructure.md`, GL =
`docs/research/GLOSSARY.md`, draft = `netcode-architect-adr0001-draft.md`, map = `engine-architect-decision-map.md`,
0003-PM = `test-engineer-passmarks-0003.md`, 0004 = `rust-reviewer-adr0004-draft.md`, 0006 =
`infra-engineer-adr0006-draft.md`.

Tiers (the plan's): `research verified` = an R1 or R2 recommendation claim (its marks are proposed in the research;
the tier says the text is what R1 or R2 recommends), or a finding in a verifier's checked set · `research, source to
check` = any other research statement · `arithmetic` (shown) · `settled` (a STATUS code, GLOSSARY, the plan or the
map's assignment) · `design doc` (VISION_V2, LOOP_V2) · `proposed` (R5's or R6's proposed mark, "proposed (R5's)";
the draft's own proposal, "proposed (the draft's)"; or the test-engineer's own, "proposed (test-engineer)", with its
reason).

Mark codes: PRE the "How to read the table" bullets · PM-n the table row · FAIL the failure rule · NOM the "no
measurable mark" lines · CL the netcode checklist (step n) · SE the spike entry.

Line checks: `scripts/cite_check.py` on the subsection: 151 ids cited, 0 missing, 0 line-range problems in its 20
parenthesized citation groups. The short-form cites in the table's basis column (r2 marks deep inside a
recommendation, 32 of them more than 20 lines below the id's heading) were each read at the cited lines on 2026-11-01.

## How to read the table

| # | the claim or number (short) | where | citation (file, id, lines) | tier |
|---|---|---|---|---|
| TE1 | RS: Hetzner Cloud CCX23, 4 dedicated vCPU, 16 GB, x86-64 AMD, one EU location | PRE | r6 R6-HOST-REC2, 645–650; GL the reference server, 225–243 | settled (GLOSSARY) |
| TE2 | A server time mark is the work's elapsed time on one vCPU with no other room on it | PRE | GL the measuring unit, 244–264 | settled (GLOSSARY) |
| TE3 | The tick thread pinned alone to one vCPU; the transport runtime on the other three | PRE | none | proposed (test-engineer: the measured vCPU holds only the tick, so the measuring unit holds) |
| TE4 | SH: the shared CX33 (4 vCPU, 8 GB) beside it, rooms per VM only | PRE | r6 R6-HOST-REC2, 645–647, 667–671 | research, source to check |
| TE5 | LG: a second VM in the same location runs the headless bot clients that speak the real protocol | PRE | r5 R5-TEST-REC5, 1475–1480 (bots as protocol clients) | proposed (test-engineer: clients on RS would take CPU from the measured work) |
| TE6 | ML, RD, MAC as ADR 0003's pass marks name them; models open | PRE | r3 R3-BUDGET-REC1, 1096–1102; 0003-PM "How to read the table"; OQ-0003-1 | proposed (R3's machines); settled (0003-PM's names) |
| TE7 | ML runs Chrome, Edge and Firefox on Windows 11; MAC runs Safari, the fourth browser | PRE | VISION_V2 §4 (four browsers); r3 R3-BUDGET-REC1, 1096–1100 | design doc |
| TE8 | MP: only what Mustafa judges and Brave's transport; never a measured result | PRE | plan rule 12; brief's "Mustafa's own machine in Brave … for what he judges" | settled (plan) |
| TE9 | L1 is on simulated time, so it carries logic marks only, never a time mark | PRE | r5 R5-TEST-REC4, 1454–1457 ("seeded, on simulated time") | proposed (test-engineer: a simulated clock times nothing real) |
| TE10 | Browser versions: stable releases on the day, recorded; R2 names Chrome 156, Edge 156, Firefox 158, Safari 27 on macOS | PRE | r2 R2-TRN-REC1 mark 1, 2586 | research verified |
| TE11 | SS: the stress scene at N = 48 (three teams of 16, one cluster), 15 big vehicles, 4 very large units, 300 live projectiles, up to 8 active effects per soldier, a snapshot-receiving client on every soldier, one DM_Tri-sized greybox | PRE | GL §3 the stress scene, 397–437 | settled (GLOSSARY) |
| TE12 | SS's greybox holds enterable buildings with areas and portals | PRE, SE | draft part 9 (b), 288–292 | proposed (test-engineer: re-flood ticks need portals, and the 60 Hz margin includes them) |
| TE13 | ES: 48 soldiers and 15 big vehicles all in view, 32 live projectiles at 0.44 spawns per player per second | PRE | r2 worked estimate, 1754–1779 | research, source to check (R2's assumptions) |
| TE14 | CO: a scripted course with no other players | PRE | r2 R2-SYNC-REC3 mark 1, 2101 | research verified |
| TE15 | TC: a target sprinting at 9 m/s changing direction at random; static targets on the ground, at 2.5 m and 4 m | PRE | r2 R2-SYNC-REC5 mark 1, 2179; R2-SYNC-REC8 mark 1, 2272 | research verified |
| TE16 | The proof test's sim: movement, hitscan, bots as protocol clients | PRE | draft "What a spike must prove", 927–928 | proposed (the draft's) |
| TE17 | The 60 Hz margin's sim is OQ-0001-2's pick; the draft recommends (a) | PRE | draft OQ-0001-2, 846–855; map J6 (2), 1215–1219 | settled (the map's assignment); the recommendation proposed (the draft's) |
| TE18 | A 30 Hz row on a stand-in proves the server-only share for that load; PM-4 marked "stand-in" if the stand-in's sim step time is under the sim core spike's | PRE | draft OQ-0001-2 ("the 30 Hz baseline marks may use a stand-in"), 853–854; migration step 8, 1030–1033 | proposed (test-engineer: a lighter sim makes the total easier than the real one) |
| TE19 | Profiles P0–P4 and B1 with their numbers; latency is the round trip | PRE | r2 profiles, 1987–1990; draft part 13, 391–394 | research verified |
| TE20 | L1 runs the headless clients; L2 under quinn's socket, a delaying TCP proxy for WebSocket | PRE | r5 R5-TEST-REC4, 1454–1461 | research, source to check |
| TE21 | "100 ms of simulated latency" is P2; the Advisor confirms (OQ-0001-3) | PRE | draft part 13, 397–398; OQ-0001-3, 856–860; map J6 (3), 1220–1221 | settled (the map's assignment); proposed (the draft's reading) |
| TE22 | If read as 100 ms each way, the human-client rows run at a 200 ms round trip | PRE | draft OQ-0001-3 option (b), 858 | proposed (the draft's option) |
| TE23 | Reordering: what the simulator produces under the jitter; reorder share recorded; a mark naming reordering counts only if it is above 0 | PRE | r2 "Every spike also covers reordering", 1991–1992 | proposed (test-engineer: R2 names no rate; a mark that names reordering cannot pass on a run with none) |
| TE24 | S30: a 60 s warm-up after the last client's first full state is acknowledged, then 30 minutes measured | PRE | r2 R2-BUD-REC6's 30-minute soak, 2413; warm-up none | proposed (test-engineer: joins and first full states are not the steady state) |
| TE25 | 30 minutes at 30 Hz = 54,000 ticks; at 60 Hz = 108,000 | PRE | 30 × 60 × 30 = 54,000; 30 × 60 × 60 = 108,000 | arithmetic |
| TE26 | p99 over measured ticks; max, overruns and dropped time over the whole run, joins included | PRE | none | proposed (test-engineer: a real match has its join burst too) |
| TE27 | The tick counter starts 1,800 ticks below 65,536 so every S30 crosses a 16-bit wrap | PRE | draft part 6 (wrap after 36.4 min at 30 Hz, 18.2 at 60 Hz), 240–243; failure modes row, 555; 60 × 30 = 1,800 | proposed (test-engineer: the draft's own wrap check runs only at 60 Hz) |
| TE28 | Portal changes once per second put re-flood ticks on 3.3 % of ticks at 30 Hz and 1.7 % at 60 Hz, above the 1 % a p99 reads | PRE | 1 ÷ 30 = 3.3 %; 1 ÷ 60 = 1.7 %; GL time statistics, 141–147 | arithmetic; the rate proposed (test-engineer: "re-flood ticks included" names no rate, and below 1 % of ticks they would not reach the p99) |
| TE29 | p99 by nearest rank: the ⌈0.99 n⌉-th smallest; 53,460th of 54,000; 106,920th of 108,000 | PRE | 0.99 × 54,000 = 53,460; 0.99 × 108,000 = 106,920 | arithmetic; nearest rank proposed (test-engineer: one definition, as 0003-PM) |
| TE30 | A time mark with no statistic is a p99; p99s never added; server tick time's p99 measured directly | PRE | GL time statistics, 141–147 | settled (GLOSSARY) |
| TE31 | A one-off deadline (join, recovery, start) is a plain limit: every event meets it | PRE | GL time statistics, 145–147 | settled (GLOSSARY) |
| TE32 | A share is a count over the run's whole sample | PRE | none | proposed (test-engineer: shares in R2's marks are of a whole run's shots, frames or snapshots) |
| TE33 | A bandwidth mark with no statistic is read as a mean | PRE | r2 R2-BUD-REC2's main mark is a mean, 2339 | proposed (test-engineer: R2's own form for the downstream) |
| TE34 | kbps = 1,000 bit/s; MB = 10^6 B | PRE | none | proposed (test-engineer: the stricter reading of the research's units) |
| TE35 | The tick thread times the tick and each phase every tick; counters on in every run, as in production | PRE | r6 R6-OBS-REC2, 741–742 (server tick time histogram, per-phase time) | proposed (test-engineer); R6's metric list research, source to check |
| TE36 | The server clock's resolution is not researched: measured per machine; a part under 1 µs timed per tick and divided by its count | PRE | none (no research finding on the server clock) | proposed (test-engineer: one read per pair would cost as much as the work) |
| TE37 | CPU shares from per-thread CPU time every second | PRE | GL transport runtime time ("a share of one vCPU per second"), 130–133 | proposed (test-engineer tool) |
| TE38 | `performance.now()` 5 µs isolated, 100 µs otherwise; marks of 1 ms or less run with the isolation flag | PRE | r3 R3-MEASURE-F7, 641–646 (confirmed, r3:1587); GL the isolation flag, 636–650 | research verified (in R3's checked set); the rule proposed (test-engineer, as 0003-PM) |
| TE39 | That a Worker of an isolated page is isolated too is not researched; `crossOriginIsolated` read in the Worker | PRE | none | proposed (test-engineer) |
| TE40 | L2 client times use the clock estimate, whose error bound is PM-47's | PRE | r2 R2-BUD-REC4 mark, 2378–2380 | research verified |
| TE41 | Presented frames on a 60 Hz display; no frame-time budget here; one tick (33.3 ms) is two refresh periods | PRE | 1000 ÷ 30 = 33.3 ms; 1000 ÷ 60 = 16.7 ms; 2 × 16.7 = 33.3 | arithmetic; mode P proposed (test-engineer: latency is about what a player sees; the 0003-PM vsync lesson) |
| TE42 | A timed server mark runs 3 times and every run must meet it | PRE | r1 R1-TICK-REC2 P1 ("3 runs"), 2047 | research verified (P1's count); its use for every timed server mark proposed (test-engineer) |
| TE43 | Validity: steal time at most 1 % of the run | PRE | none | proposed (test-engineer: a VM's neighbours show as steal; not researched) |
| TE44 | Validity: LG's clients sent 30 inputs per second each within 1 % | PRE | draft part 5 (one command per tick), 185 | proposed (test-engineer: a starved load generator lightens the load) |
| TE45 | A clean reference is the median of 3 clean runs whose p99s lie within 2.5 % of their median | PRE | r2 R2-TRN-REC6 mark 2 (5 %), 2743; 5 % ÷ 2 = 2.5 % | proposed (test-engineer: half the tightest tolerance compared against; the 0003-PM overlay lesson) |
| TE46 | An invalid run is neither pass nor fail; its cause is found first | PRE, FAIL | r5 R5-TEST-REC6, 1490–1505 | proposed (R5's rule, applied to measurement) |
| TE47 | A planted defect must fail each automated mark on the spike's build before its pass counts; a 5-minute run | PRE | r5 R5-GATE-REC3 and its reason, 1706–1728 ("a check without a failing case can't be shown to work") | research, source to check; the run length proposed (test-engineer) |
| TE48 | Rule B-S: a busy-wait sized to the mark minus the measured p99 plus 10 % of the mark | PRE | none | proposed (test-engineer: a relative size suits marks from 20 µs to 16.7 ms) |
| TE49 | Rule B-N: padding sized to the measured margin plus 10 % of the mark | PRE | none | proposed (test-engineer) |
| TE50 | No retries: data, seed, commit, machine, browser and profile kept; cause found | PRE, FAIL | r5 R5-TEST-REC6, 1490–1505 | proposed (R5's) |

## Preconditions

| # | the claim or number (short) | where | citation (file, id, lines) | tier |
|---|---|---|---|---|
| TE51 | RS, SH and LG named with plan, CPU, vCPUs, memory, location, kernel; ML, RD, MAC as ADR 0003 | PM-1 | r6 R6-HOST-REC2, 645–650; r3 R3-BUDGET-REC1, 1096–1102; 0003-PM PM-0003-1 | proposed (test-engineer, as 0003-PM) |
| TE52 | The tick thread's affinity is one vCPU no other thread uses | PM-1 | GL the measuring unit, 244–248 | settled (GLOSSARY); the check proposed (test-engineer) |
| TE53 | Bite: a second room's tick thread on the same vCPU flags the run invalid | PM-1 | none | proposed (test-engineer) |
| TE54 | 48 soldiers in three teams of 16, each with a snapshot-receiving client | PM-2 | GL the stress scene, 399–401, 416–417; r2 R2-BUD-REC6, 2412 | settled (GLOSSARY) |
| TE55 | 15 big vehicles, 4 very large units of at least 20 parts | PM-2 | GL 403, 438–442 | settled (GLOSSARY) |
| TE56 | 300 live projectiles; up to 8 active effects per soldier | PM-2 | GL 404–405, 458–460 | settled (GLOSSARY) |
| TE57 | Hitscan fire at 700 shots per second | PM-2 | r2 R2-SYNC-REC5 mark 4, 2186–2187; R2-SYNC-REC9, 2280 ("at most about 700 at 48, … inferred") | research verified |
| TE58 | 700 shots per second within ±5 % as a run mean | PM-2 | none | proposed (test-engineer: the lag-compensation part must run at its planning load) |
| TE59 | Every client's relevant set holds all 47 others on at least 99 % of its snapshots ("everyone in view") | PM-2, PM-12 | r2 R2-BUD-REC2 ("all in view"), 2339; 48 − 1 = 47 | proposed (test-engineer: the downstream mark is the all-in-view case; the share proposed) |
| TE60 | A sim lacking vehicles, projectiles or effects adds stand-in entities replicated, hit-tested and filtered at the same counts | PM-2 | draft "What a spike must prove" (the sim has movement, hitscan, bots), 927–928; GL 397–437 | proposed (test-engineer: the server-only share must carry the stress scene's load) |
| TE61 | Bite: a load script that stops firing after 10 minutes | PM-2 | none | proposed (test-engineer) |
| TE62 | Per profile on L1 and L2: loss, mean delay, jitter over 100,000 packets within 5 % of the profile; the same seed gives an identical packet-fate log | PM-3 | r5 R5-TEST-REC4 mark, 1469–1471 | proposed (R5's, for L1); its use on L2 proposed (test-engineer) |
| TE63 | P0 on L2 is the real path, recorded | PM-3 | none | proposed (test-engineer: a real path has its own delay, so "0 ms" cannot be met there) |
| TE64 | MP's server-measured RTT median within 100 ± 5 ms at P2; base RTT recorded | PM-3 | r2 R2-TRN-REC4 mark 2 (±5 ms), 2694 | proposed (test-engineer: the session must be at P2's latency, judged with R2's own RTT tolerance) |
| TE65 | The injected delay is the profile's RTT minus the path's base RTT (median of 10 s of pings) | PM-3, CL intro | none | proposed (test-engineer) |

## The 30 Hz baseline

| # | the claim or number (short) | where | citation (file, id, lines) | tier |
|---|---|---|---|---|
| TE66 | Server tick time p99 ≤ 16.7 ms; max < 33.3 ms; 0 overruns | PM-4 | draft budgets row, 527; r2 R2-BUD-REC6, 2411–2416; GL the server tick budget, 167–172 | research verified (restated at 30 Hz by the draft); settled (GLOSSARY) |
| TE67 | 0 dropped time (R2-BUD-REC6's "0 missed ticks", read by the loop's dropped-time counter) | PM-4 | r2 R2-BUD-REC6, 2414; r1 R1-TICK-REC4 (dropped time recorded), 2082–2084 | research verified; the counter as the reading proposed (test-engineer) |
| TE68 | The whole tick timed directly, start to end, on the tick thread | PM-4 | GL server tick time, 113–118; time statistics, 145 | settled (GLOSSARY) |
| TE69 | R2-ROOM-REC2 mark 1's 20 minutes fall inside S30 | PM-4 | r2 R2-ROOM-REC2 mark 1, 2775–2776 | research verified |
| TE70 | Shares: sim step p99 ≤ 8.3 ms and server-only p99 ≤ 8.3 ms, each its own per-tick series | PM-5 | draft budgets row, 528; r2 R2-BUD-REC6, 2398–2401; R2-SYNC-REC9 mark 1, 2292–2296; GL 189–213 | research verified; settled (GLOSSARY) |
| TE71 | The sim step time is the step's elapsed time minus its lag-compensation queries | PM-5 | draft part 3 and data flow (queries counted in the server-only share), 145–149, 429–432 | proposed (test-engineer: the draft's accounting, made measurable) |
| TE72 | The sim step share's own parts are ADR 0004's rows | PM-5 | draft part 3, 148–149; map N7 (owner 0004), 731 | settled (the map's assignment) |
| TE73 | Encoding p99 ≤ 50 µs per client per snapshot; 48 × 50 µs = 2.40 ms | PM-6 | r2 R2-BUD-REC6, 2414–2415; GL 55–69 | research verified; arithmetic |
| TE74 | 48 × 30 = 1,440 encode samples per second | PM-6 | arithmetic | arithmetic |
| TE75 | A rewound shot p99 ≤ 20 µs at 700 shots per second, 48 candidates; 700 ÷ 30 × 20 µs = 0.47 ms per tick | PM-7 | r2 R2-SYNC-REC5 mark 4, 2186–2187; GL 55–69 | research verified; arithmetic |
| TE76 | History ≤ 1 MB for the stress scene's soldiers and big vehicles | PM-7 | r2 R2-SYNC-REC5 mark 3, 2184–2185; draft part 8, 274–275 | research verified |
| TE77 | Bite: a history never trimmed fails 1 MB | PM-7 | none | proposed (test-engineer) |
| TE78 | Region query p99 ≤ 0.2 ms per tick for 48 clients | PM-8 | r2 R2-INT-REC1 (c), 2440–2441 | research verified |
| TE79 | Ranking and packing p99 ≤ 0.5 ms per tick; 0 snapshots over the per-client budget; nearest 8 enemies in every snapshot; every relevant entity at least every 4 snapshots | PM-8 | r2 R2-INT-REC4, 2497–2500; draft part 9 (d), 294–295 | research verified |
| TE80 | 256 kbps cap = 256,000 ÷ 8 ÷ 30 = 1,067 B per snapshot interval, at UDP payload + 28 B per datagram | PM-8 | draft part 6, 223–224 (the cap proposed in the draft) | arithmetic; the cap proposed (the draft's) |
| TE81 | SS with its fight in the centre field is R2-INT-REC4's scene | PM-8 | r2 R2-INT-REC4, 2497 | research verified |
| TE82 | Bite: an accumulator ranking by entity id | PM-8 | none | proposed (test-engineer) |
| TE83 | Pair test p99 ≤ 0.1 µs per enemy pair; 768 pairs at 16v16v16 (48 × 32 ÷ 2); 0.08 ms | PM-9 | r2 R2-INT-REC2 (b), 2463–2464; GL 55–69 | research verified; arithmetic |
| TE84 | Re-flood p99 ≤ 1 ms over the run's re-floods; area count recorded | PM-9 | r2 R2-INT-REC2 (b), 2463–2464 ("for the map's area count") | research verified |
| TE85 | The pair test timed per tick and divided by that tick's pairs | PM-9 | TE36 | proposed (test-engineer) |
| TE86 | Mean server-only work per tick ÷ (clients × mean relevant entities) at N = 48 ≤ 1.1 × that at 24 and 36 | PM-10 | r2 R2-SYNC-REC9 mark 2, 2297–2298 ("no faster than linearly in players × visible entities") | research verified; the 1.1 tolerance and the use of means proposed (test-engineer: R2 names no tolerance; means compare cost per unit of work) |
| TE87 | Scale steps N = 24 and 36 (three teams of 8 and of 12), 10 minutes each | PM-10 | GL stress scene scale steps, 400–402; 24 ÷ 3 = 8, 36 ÷ 3 = 12 | settled (GLOSSARY); the length proposed (test-engineer: a mean settles well within it) |
| TE88 | Bite: work growing with the square of N | PM-10 | none | proposed (test-engineer) |
| TE89 | Transport runtime ≤ 1 vCPU with 48 sessions; p99 send call ≤ 1 ms | PM-11 | r2 R2-TRN-REC1 mark 2, 2589–2594 | research verified |
| TE90 | 30 input datagrams per second each (the research's 60 is the 60 Hz form) | PM-11 | draft budgets row, 532; map N29, 757 | proposed (the draft's restatement at TICK-30) |
| TE91 | "At most 1 vCPU" read as the p99 of 1-second windows | PM-11 | GL transport runtime time ("a share of one vCPU per second"), 130–133; time statistics, 141–143 | proposed (test-engineer: GLOSSARY's no-statistic rule, read per second) |
| TE92 | Downstream mean ≤ 160 kbps; p99 of 1-s windows ≤ 256 kbps; UDP payload + 28 B per datagram | PM-12 | r2 R2-BUD-REC2, 2339–2341; draft budgets row, 533 | research verified |
| TE93 | Every 1-s window ≤ 256 kbps (the hard cap) | PM-12 | draft part 6 ("a per-client hard cap"), 223–224 | proposed (test-engineer reading of "hard cap") |
| TE94 | The server tick and downstream rows run in SS, as the draft's rule says | PM-12 | draft budgets preamble ("in the stress scene … unless the row says otherwise"), 521–523 | proposed (the draft's) |
| TE95 | ES reported beside it, 10 minutes (R2-BUD-REC2's length) | PM-12 | r2 R2-BUD-REC2 ("10-minute bot match"), 2339; worked estimate, 1754–1779 | proposed (test-engineer: the scene the research mark was set on, so a failure in SS can be told apart from an encoding failure) |
| TE96 | IPv6 adds 20 B per datagram: +4.8 kbps at 30 Hz | PM-12 | r2 worked estimate, 1784 | research, source to check (R2's arithmetic) |
| TE97 | Upstream mean ≤ 48 kbps per client (IPv4) | PM-13 | r2 R2-BUD-REC1, 2324; draft 507–512 | research verified |
| TE98 | Restated against P2 | PM-13 | draft part 13, 394–396 | proposed (the draft's restatement) |
| TE99 | The mean as the gating statistic; p99 of 1-s windows, largest datagram and command size reported | PM-13 | r2 R2-XU-OQ4 (command size open), 3151 | proposed (test-engineer: R2's U1 was a fixed size, the draft's redundancy varies) |
| TE100 | 0 snapshot datagrams above the cap and the reported maximum, server and browser side | PM-14 | r2 R2-SYNC-REC1 mark 3, 2021; R2-BUD-REC3, 2358–2359 | research verified |
| TE101 | The cap is min(the smallest `maxDatagramSize`, 1,150 B); 1,000 B until measured | PM-14 | r2 R2-BUD-REC3, 2345–2346; draft part 6, 224–226 | research verified; proposed (the draft's 1,000 B for the load) |
| TE102 | 0 fragmented IP packets (packet capture on RS) | PM-14 | r2 R2-BUD-REC3 ("no IP or application-level fragmentation"), 2346 | research verified; the capture proposed (test-engineer tool) |
| TE103 | Resident memory ≤ 64 MB at start and after 20 minutes | PM-15 | r2 R2-ROOM-REC2 mark 3, 2778–2779; draft budgets row, 535 | research verified (the 64 MB proposed in R2) |
| TE104 | The 1-s sampled peak ≤ 64 MB | PM-15 | none | proposed (test-engineer: a ceiling read at two moments misses a peak between them) |
| TE105 | Slope within 1 MB per 10 minutes after warm-up | PM-15 | r5 R5-TEST-REC5 mark, 1486–1488 | proposed (R5's) |
| TE106 | Queues flat: last 10 minutes' max ≤ first 10 minutes' max | PM-15 | r2 R2-TRN-REC6 mark 1 ("no queue growth"), 2742 | proposed (test-engineer reading of "flat") |
| TE107 | Bite: one buffer kept per snapshot, flagged within the run | PM-15 | r5 R5-TEST-REC5 mark, 1488–1489 | proposed (R5's) |
| TE108 | Room start-up < 2 s, every start, at least 20 starts | PM-16 | r2 R2-ROOM-REC2 mark 5, 2780; GL time statistics (a plain limit), 145–147 | research verified; the count proposed (test-engineer) |
| TE109 | 100 % of decoded snapshots rebuild the server's quantised state by the client-held state's hash; 10 minutes each under P0, P2, P3, B1 | PM-17 | r2 R2-SYNC-REC1 mark 1, 2017–2019; GL state hash, 546–551 | research verified; settled (GLOSSARY term) |
| TE110 | The same check holds across the 16-bit wrap | PM-17 | draft part 6, 240–243 | proposed (test-engineer, TE27) |
| TE111 | L1: 3 seeds per profile; L2: 1 run | PM-17 and others | none | proposed (test-engineer: seeds vary the random parts; L2 runs are real and slow) |
| TE112 | Bites: a 16-bit header tick; a client quantiser that rounds differently | PM-17 | none | proposed (test-engineer) |
| TE113 | After B1, every client decodes deltas within 500 ms; 10 bursts per run | PM-18 | r2 R2-SYNC-REC1 mark 2, 2020; draft failure modes, 542 | research verified; the count proposed (test-engineer) |
| TE114 | Every join: a complete world within 300 ms of sending `Join` at P2 | PM-19 | r2 R2-BUD-REC3, 2359 ("100 ms RTT and 2 % loss"); draft 952 (the one join mark, R2-XU-OQ6) | research verified; the start point ("from sending `Join`") proposed (test-engineer: the research names none) |
| TE115 | Complete world = the client-held state's hash equal to the server's for that client | PM-19 | GL state hash, 546–551; r2 R2-SYNC-REC1 mark 1, 2017–2019 | proposed (test-engineer: the hash check makes "complete" testable) |
| TE116 | 100 joins, including rejoins after a Worker trap that ADR 0004's S20 injects | PM-19 | draft failure modes ("the join mark"), 550; 0004 S20 | proposed (test-engineer count); settled (0004's S20 names the join mark) |
| TE117 | ≥ 99.9 % of commands before their tick under P3 | PM-20 | r2 R2-SYNC-REC2 mark 1, 2058 | research verified |
| TE118 | 10,000 presses with reordering: 0 lost, 0 doubled | PM-20 | r2 R2-SYNC-REC2 mark 2, 2059 | research verified |
| TE119 | Missing inputs ≤ 0.1 % of player-ticks at P3 (from 5 % random loss) | PM-20 | r2 R2-BUD-REC1, 2323–2324; draft part 13 | research verified; restated (the draft's) |
| TE120 | Bite: a redundancy cap of 1 | PM-20 | none | proposed (test-engineer) |
| TE121 | Every forged kind rejected and counted, 0 effect beyond the valid part; lag switch (200 ms held under P1) moves no further than the catch-up cap | PM-21 | r2 R2-SYNC-REC2 marks 3–4, 2060–2066 | research verified |
| TE122 | "0 effect" checked by the state hash of the forged run against the valid-parts run | PM-21 | GL state hash, 546–551 | proposed (test-engineer method) |
| TE123 | 1,000 forged commands per kind; 100 lag-switch bursts | PM-21 | none | proposed (test-engineer counts) |
| TE124 | A never-acknowledging client: queue held at the cap, resynced or dropped | PM-22 | r2 R2-BUD-REC2, 2342–2344; draft part 6, 230–232 | research verified |
| TE125 | Throttle 128 kbps for 10 s: ≤ 2 missing per second; own state in 100 % of snapshots; full rate within 2 s | PM-23 | r2 R2-BUD-REC5, 2395–2397 | research verified |
| TE126 | 10 throttles per run | PM-23 | none | proposed (test-engineer count) |
| TE127 | Bottleneck at 50 %: newest snapshot never older than 3 intervals (100 ms); memory per session flat; tick p99 within 5 % of clean; NewReno vs Cubic, the lower p99 snapshot age picked | PM-24 | r2 R2-TRN-REC6 marks 1–3, 2741–2744; 3 × 33.3 = 100 ms | research verified; arithmetic |
| TE128 | Flood: 1,000 non-joining sessions plus 10 oversize clients; tick p99 within 10 % of clean; memory under 64 MB; 403 from four browsers | PM-25 | r2 R2-TRN-REC1 mark 4, 2597–2609 | research verified |
| TE129 | The same suite over WebSocket, plus oversized frames and messages; `ws://` refused | PM-25 | r2 R2-TRN-REC3 mark 4, 2670–2671; draft test plan, 1045–1046 | research verified |
| TE130 | A session without `Join` after 2 s closed | PM-25 | r2 R2-TRN-REC1 mark 4, 2603; draft part 10, 328 | research verified |
| TE131 | A handler panic closes only that connection; 0 of the other 47 disconnect; 0 overruns in the next minute | PM-26 | r2 R2-ROOM-REC2 mark 6, 2780–2782 | research verified; "0 overruns in the next minute" proposed (test-engineer reading of "the room's tick continues") |
| TE132 | 10 injections; bite `panic = "abort"` | PM-26 | draft part 11 (`panic = "unwind"`), 348 | proposed (test-engineer) |
| TE133 | 8 rooms as 8 processes: each p99 within 10 % of one room over 20 minutes; SIGKILL of one | PM-27 | r2 R2-ROOM-REC2 marks 2, 4, 2777, 2779–2780; r6 R6-HOST-REC2 mark, 667–671 | research verified |
| TE134 | "Unchanged" read as within 5 % | PM-27 | r2 R2-TRN-REC6 mark 2 ("unchanged (within 5 %)"), 2743 | proposed (test-engineer: R2's own reading of "unchanged") |
| TE135 | Every room in SS with a snapshot-receiving client on every soldier | PM-27 | GL the stress scene, 416–417 | proposed (test-engineer: a room of server-side bots sends no snapshots, so its density leaves out encoding and transport) |
| TE136 | ADR 0006 repeats SH at 3 times of day | PM-27, SE | 0006 spike part 2, 740–742; r6 R6-HOST-REC2, 667–669 | settled (0006's text) |

## The online path on four browsers

| # | the claim or number (short) | where | citation (file, id, lines) | tier |
|---|---|---|---|---|
| TE137 | Each candidate crate (wtransport 0.7.x, web-transport-quinn 0.13.x): ≥ 99 of 100 attempts per browser; negotiated draft recorded | PM-28 | r2 R2-TRN-REC1 mark 1, 2586–2588; draft part 10, 308–309 | research verified |
| TE138 | A server on the same local network ("clean local network") | PM-28 | r2 R2-TRN-REC1 mark 1, 2588 | proposed (test-engineer reading) |
| TE139 | Control: a wrong certificate hash fails 100 of 100 | PM-28 | none | proposed (test-engineer) |
| TE140 | UDP blocked: a playing WebSocket session ≤ 3 s; WebTransport timeout 1.5 s; 20 attempts per browser | PM-29 | r2 R2-TRN-REC3 mark 1, 2664–2665; draft part 10, 312 | research verified; the count proposed (test-engineer) |
| TE141 | One recorded input replay: identical server state hashes on every tick over both transports | PM-30 | r2 R2-TRN-REC3 mark 2, 2666–2667 | research verified |
| TE142 | 3 recordings of 5 minutes; bite: a wall-clock read on one path | PM-30 | none | proposed (test-engineer) |
| TE143 | Over WebSocket at P2: p99 extra snapshot delay ≤ 250 ms; ≤ 1 unsent snapshot per client | PM-31 | r2 R2-TRN-REC3 mark 3, 2668–2669; draft 937 (restated as P2) | research verified; restated (the draft's) |
| TE144 | Extra delay = age when decoded minus the profile's one-way delay (50 ms) | PM-31 | 100 ÷ 2 = 50 ms | proposed (test-engineer definition; R2 names none) |
| TE145 | One build on four browsers and both backends | PM-32 | r2 R2-TRN-REC4 mark 1, 2692–2693 | research verified |
| TE146 | "Runs unchanged" read as a scripted 60 s session with 0 console and page errors | PM-32 | 0003-PM PM-0003-2 (the same reading for the look proof test) | proposed (test-engineer reading) |
| TE147 | App RTT within ±5 ms of quinn's `rtt()`: gated at P2 (the draft); also at a fixed 100 ms (R5's reading) | PM-33 | r2 R2-TRN-REC4 mark 2, 2694; draft 940; r5 R5-TEST-REC4, 1471–1472 | research verified; the P2 restatement proposed (the draft's) |
| TE148 | Statistic: p99 of the absolute difference, sampled each second over 10 minutes | PM-33 | GL time statistics, 141–143 | proposed (test-engineer: GLOSSARY's no-statistic rule) |
| TE149 | Bite: pongs answered from the tick thread | PM-33 | none | proposed (test-engineer) |
| TE150 | Fuzz: 10 million inputs per message type, 0 panics, 0 allocations above the limits | PM-34 | r2 R2-TRN-REC4 mark 4, 2696–2697; r5 R5-TEST-REC4 (cargo-fuzz), 1460–1461 | research verified |
| TE151 | Swap by diff; `cargo tree`; version floors quinn-proto ≥ 0.11.18, rustls ≥ 0.23.46, ring ≥ 0.17.12, tokio ≥ 1.54.1; `dangerous-configuration` off | PM-35 | r2 R2-TRN-REC1 marks 3, 5, 2595–2596, 2610–2612; R2-TRN-REC4 mark 3, 2695 | research verified |
| TE152 | Bite: `quinn` added to the sim crate (G5's mutation) | PM-35 | r5 R5-GATE-REC3, G5 row, 1717 | proposed (R5's mutation) |
| TE153 | `maxDatagramSize` at start and every 10 s; ≥ 1,000 B on loopback and a real EU path; else the cap drops | PM-36 | r2 R2-TRN-REC2 mark 1, 2631–2632 | research verified |
| TE154 | The EU path from an EU consumer connection to RS | PM-36 | none | proposed (test-engineer: the path players use) |
| TE155 | Lost or late (more than two intervals) ≤ injected loss + 1 point: 3 % at P2 | PM-37 | r2 R2-TRN-REC2 mark 2, 2633–2634; 2 % + 1 = 3 % | research verified; arithmetic |
| TE156 | 0 of 10,000 one-shot actions at 5 % loss (P3); a 1 MB bulk transfer adds ≤ 5 ms to the control stream's p99 | PM-38 | r2 R2-TRN-REC2 marks 3–4, 2635–2637 | research verified |
| TE157 | The bulk mark read as p99 with the transfer minus p99 without | PM-38 | none | proposed (test-engineer reading; comparing two p99s, not adding them) |

## The 60 Hz margin

| # | the claim or number (short) | where | citation (file, id, lines) | tier |
|---|---|---|---|---|
| TE158 | p99 server tick time ≤ 6.25 ms against 8.33 ms: 1 − 6.25 ÷ 8.33 = 25 % | PM-39 | draft part 2, 118–127; budgets row, 530; map J1 (GLOSSARY §8 text), 1103–1111; plan TICK-60-HEADROOM, 83–91 | settled (TICK-60-HEADROOM, confirmed at the stop); arithmetic |
| TE159 | Shares ≤ 4.2 ms each; max < 16.7 ms; 0 overruns; encode ≤ 50 µs | PM-39 | r2 R2-BUD-REC6, 2413–2415; r1 R1-TICK-REC2 P1, 2047–2049; GL 167–172 | research verified; settled (GLOSSARY) |
| TE160 | The setup: one vCPU of RS, SS at N = 48 with a client on every soldier, 30-minute soak, re-flood ticks included | PM-39 | map J1 N3, 1055; GLOSSARY §8 text, 1104–1109 | settled |
| TE161 | P2 in the margin run, as in the 30 Hz soak | PM-39 | r2 R2-SYNC-REC9 mark 1 (P2), 2292–2293 | proposed (test-engineer: the two runs then differ only in the tick rate) |
| TE162 | Tick 65,536 reached at 18.2 minutes at 60 Hz | PM-39 | 65,536 ÷ 60 ÷ 60 = 18.2; draft 241–242 | arithmetic |
| TE163 | "In one run" read as: every number of a run from that run; 3 runs, each passes | PM-39 | GL §8 ("measured … in one run (p99s do not add)"), map 1104–1105; r1 R1-TICK-REC2 P1 (3 runs), 2047 | proposed (test-engineer reading) |
| TE164 | A failure keeps 30 Hz | PM-39, FAIL | draft spike item 3, 966–967; Rollback, 1058 | proposed (the draft's) |
| TE165 | P1 at 60 Hz: sim step p99 ≤ 4.2 ms at N = 48, max ≤ 16.7 ms, 0 overruns, 36,000 ticks, 3 runs | PM-40 | r1 R1-TICK-REC2 P1, 2044–2049; 0004 S1 | research verified; settled (0004's row) |
| TE166 | P2 at 60 Hz: the client's sim share p99 ≤ 6.7 ms in Wasm on ML while the page holds 30+ fps on Low | PM-40 | r1 R1-TICK-REC2 P2, 2054–2057; GL 214–217 | research verified |
| TE167 | P3 at 60 Hz: 12 ticks of replay ≤ 1 ms p99 | PM-40 | r1 R1-TICK-REC2 P3, 2058–2060 | research verified |
| TE168 | 60 Hz needs all three and PM-39 | PM-40 | draft part 2, 125–126 ("every 60 Hz mark the research sets, as written") | settled (TICK-60-HEADROOM) |

## The human client at P2

| # | the claim or number (short) | where | citation (file, id, lines) | tier |
|---|---|---|---|---|
| TE169 | 0 corrections > 1 mm when every command arrived in time, P0–P4; every other correction traced | PM-41 | r2 R2-SYNC-REC3 mark 1, 2101–2102 | research verified |
| TE170 | The Worker counts corrections from part 4's replay check | PM-41 | draft part 6 ("the Worker counts prediction mismatches and correction sizes"), 246–248 | proposed (the draft's) |
| TE171 | Bites: server run speed 1 % above the client's; the server quantising its own copy differently from what it sends | PM-41 | draft part 4 (own state unquantised or quantised the same way), 173–174 | proposed (test-engineer) |
| TE172 | P3 brawl of 48: corrections > 5 cm in < 1 % of snapshots; ≤ 5 cm per frame below the 2 m snap | PM-42 | r2 R2-SYNC-REC3 mark 2, 2103–2104; draft part 4 (2 m proposed), 172–173 | research verified |
| TE173 | 10 minutes; bite: smoothing off | PM-42 | none | proposed (test-engineer) |
| TE174 | Input-to-screen for own movement and muzzle flash the same at P0 and P4 within one tick (33.3 ms) | PM-43 | r2 R2-SYNC-REC3 mark 3, 2105 | research verified |
| TE175 | Read on both the p99 and the median difference | PM-43 | GL time statistics, 141–143 | proposed (test-engineer: "the same" read on both statistics) |
| TE176 | 200 inputs per profile and kind via the Chrome DevTools Protocol; event time stamp to the animation-frame time stamp | PM-43 | none | proposed (test-engineer method; not researched) |
| TE177 | ML and RD, Chrome on WebGPU | PM-43 | r3 R3-BUDGET-REC1, 1096–1102 | proposed (test-engineer: client marks on both reference machines) |
| TE178 | Replay of 6 ticks (200 ms at 30 Hz) ≤ 1 ms p99 on ML | PM-44 | r1 R1-TICK-REC2 P3, 2058–2060; draft budgets row, 531 | research verified |
| TE179 | Chrome and Firefox (two Wasm engines) | PM-44 | none | proposed (test-engineer: the laptop's two Wasm compilers) |
| TE180 | The replay forced on every snapshot with exactly 6 ticks | PM-44 | draft part 4 (skip when matching), 169–170 | proposed (test-engineer: with the skip on, too few and too easy replays are timed) |
| TE181 | 10 minutes = 18,000 snapshots at 30 Hz; p99 = the 17,820th | PM-44 | 10 × 60 × 30 = 18,000; 0.99 × 18,000 = 17,820 | arithmetic |
| TE182 | The vehicle case only if the sim has a piloted vehicle | PM-44 | r1 R1-TICK-REC2 P3 ("or the vehicle it drives"), 2058–2059 | proposed (test-engineer: the proof test's sim has none) |
| TE183 | Extrapolation < 1 % of rendered frames at P2, < 3 % at P3; under B1 none beyond one interval | PM-45 | r2 R2-SYNC-REC4 marks 1–2, 2128–2129 | research verified |
| TE184 | The Worker flags blocks built by extrapolation; the page counts frames drawn from them | PM-45 | draft part 7 (the Worker publishes delayed poses; extrapolates at most one interval), 253–258 | proposed (test-engineer method) |
| TE185 | Bite: G11's "a snapshot buffer one slot too short for P2" | PM-45 | r5 R5-GATE-REC3, G11 row, 1722 | proposed (R5's mutation) |
| TE186 | The delay settles within 5 s of a profile change; 0 slides in 1,000 respawns | PM-46 | r2 R2-SYNC-REC4 marks 3–4, 2130–2131 | research verified |
| TE187 | "Settled": within ± half an interval of the next 30 s's mean, and staying there | PM-46 | none | proposed (test-engineer definition) |
| TE188 | A slide: a frame drawing the entity on the line between old and new place, > 0.1 m from both | PM-46 | none | proposed (test-engineer definition) |
| TE189 | The delay within 2 to 5 intervals; announced changes clamped to one interval per second | PM-46 | draft part 7, 256–262; r2 R2-SYNC-REC5 (the clamp), 2151–2153 | proposed (the draft's); research verified |
| TE190 | 20 changes each way between P1 and P3 | PM-46 | none | proposed (test-engineer count) |
| TE191 | Clock offset error ≤ 5 ms after 2 s of samples against the simulator's true clock, at P2 | PM-47 | r2 R2-BUD-REC4, 2378–2380; draft part 13 (restated P2) | research verified; restated (the draft's) |
| TE192 | p99 over 1,000 connects with random true offsets | PM-47 | GL time statistics, 141–143 | proposed (test-engineer statistic and count) |
| TE193 | ≥ 99 % registered up to P2, 95 % at P3; false hits < 0.5 %; 9 m/s target | PM-48 | r2 R2-SYNC-REC5 mark 1, 2179–2181 | research verified |
| TE194 | The shooter's view is the state the Worker published for the command's view time | PM-48 | draft part 5 (the Worker casts against what it published), 202–205 | proposed (test-engineer reading of "the shooter's rendered view") |
| TE195 | 2,000 shots per profile: about 20 misses at the 99 % mark | PM-48 | 2,000 × 1 % = 20 | arithmetic; the count proposed (test-engineer) |
| TE196 | Bite: a rewind leaving out the interpolation delay; at 9 m/s, 100 ms is about 0.9 m | PM-48 | 9 × 0.1 = 0.9 m | arithmetic; the bite proposed (test-engineer) |
| TE197 | Decides OQ-0001-7 (sub-tick stamps) | PM-48 | draft OQ-0001-7 ("what decides it: R2-SYNC-REC5 mark 1's registration at 30 Hz"), 881–885 | proposed (the draft's) |
| TE198 | 100 % of shots within the cap, honest and cheat clients, P0–P4 | PM-49 | r2 R2-SYNC-REC5 mark 6, 2191–2192 | research verified |
| TE199 | The share each cap (200, 250 ms) clips recorded per profile | PM-49 | r2 R2-BUD-REC1, 2320–2323; draft part 8, 273–280; OQ-0001-5, 867–873 | research verified |
| TE200 | The rewind estimate: the view's lag behind the server's present when its command arrives (median, p99) | PM-49 | draft spike item 5 (R2-XU-OQ2), 989–990 | proposed (the draft's) |
| TE201 | Bite: the cap applied to the estimate only | PM-49 | r2 R2-SYNC-REC5 (Source's clamp bounds only its estimate), 2154–2157 | research verified (the failure it plants); the bite proposed (test-engineer) |
| TE202 | A cheat at the tolerance's edge gains no more than the tolerance (≤ one interval, 33.3 ms) under P0 and P1, 1,000 shots each | PM-50 | r2 R2-SYNC-REC5 mark 5, 2188–2190; draft part 8, 270–273 | research verified |
| TE203 | Server hit point within 5 cm of the client-picked point in ≥ 99.9 % at 0, 2.5, 4 m, P0–P3; aim ≤ 80 bits | PM-51 | r2 R2-SYNC-REC8 marks 1–2, 2272–2274 | research verified |
| TE204 | 1,000 shots per height and profile (one miss allowed) | PM-51 | 1,000 × 0.1 % = 1 | arithmetic; the count proposed (test-engineer) |
| TE205 | Bite: a parallel ray with a 10 cm origin error | PM-51 | r2 R2-SYNC-REC8 (a direction misses by the origin error), 2263–2267 | research verified (the reasoning); the bite proposed (test-engineer) |
| TE206 | 0 damage numbers or kills on another soldier before the server's event | PM-52 | r1 R1-ABL-REC4 mark, 2444–2445 | research verified |
| TE207 | 1,000 scripted hits plus every browser session | PM-52 | none | proposed (test-engineer count) |
| TE208 | Mustafa's verdict: every checklist step "achieved" | PM-53 | plan rule 12, 142–147; GLOSSARY §8 the netcode proof test (map 1086–1095) | settled |

## What the proof test decides and records

| # | the claim or number (short) | where | citation (file, id, lines) | tier |
|---|---|---|---|---|
| TE209 | Only if projectiles are in the load: 0 desyncs over 1,000 throws, else E3b and PM-12 again | PM-54 | r2 R2-BUD-REC2, 2342; draft spike item 5, 986–987; part 6, 221–222 | research verified; the rerun proposed (test-engineer: E3b changes the downstream) |
| TE210 | Bite: a client trajectory step of 1/60 s against the server's 1/30 s | PM-54 | none | proposed (test-engineer) |
| TE211 | Only if the sim moves vehicles: corrections > 5 cm in < 2 % of snapshots under P2 | PM-55 | r2 R2-SYNC-REC7 mark 3, 2244–2245; draft Appendix A row R2-SYNC-OQ5 | research verified; the condition proposed (test-engineer: the proof test's sim has no vehicles) |
| TE212 | The records of spike item 5: crates, drafts, UDP-blocked behaviour, record and command sizes, controllers, smoothness, command timing, Brave's transport | PM-56 | draft spike item 5, 985–993 | proposed (the draft's list) |
| TE213 | The crate pick: the draft names no rule beyond the marks | PM-56, NOM | draft part 10 ("picked by the netcode proof test"), 308–309; R2-TRN-REC1, 2565–2566 | settled (both texts, read 2026-11-01) |
| TE214 | Smoothness: the standard deviation of per-frame displacement as a share of its mean (R1-TICK-REC5's measure) | PM-56 | r1 R1-TICK-REC5, 2111–2112 | research verified |
| TE215 | OQ-0001-6's two options each run with PM-42's correction share and PM-48's registration at P2–P4 | PM-56 | draft OQ-0001-6 ("correction rates and fairness under P2 to P4"), 874–880 | proposed (test-engineer: the draft names the measures, not a rule) |

## Failure rule, no-mark lines, checklist, spike entry

| # | the claim or number (short) | where | citation (file, id, lines) | tier |
|---|---|---|---|---|
| TE216 | Conditional marks pass when their condition does not hold; PM-56 when complete; PM-39, PM-40 only where OQ-0001-2 places the run | FAIL | draft OQ-0001-2 option (c), 850–851; migration step 8, 1030–1031 | proposed (test-engineer, as 0003-PM) |
| TE217 | Rollback routes: 30 Hz failure back to the drafts; 60 Hz failure keeps 30 Hz; the other crate; both crates fail → drafts, Mustafa and the Advisor; E3b; option B; data retry for corrections, interpolation and hits | FAIL | draft Rollback, 1056–1066 | proposed (the draft's) |
| TE218 | A data retry happens once, its change and reason recorded | FAIL | draft Rollback ("a first retry needs no code change"), 1065–1066 | proposed (test-engineer reading of "a first retry") |
| TE219 | A mark's number changes only in the draft, with its reason | FAIL | r5 R5-GATE-REC4, 1734–1744 | proposed (R5's rule, applied to spike marks) |
| TE220 | Part 1 has no number of its own; the catalogue carries no client field for state, hits or RTT | NOM | draft part 1, 110–114; message catalogue, 476–495 | proposed (test-engineer reading of the draft) |
| TE221 | Part 9 (b) hiding, (e), (f): the reader client and byte audit at slice 3; R2-INT-REC1 (a) pop-in at slice 5 | NOM | draft migration steps 5 and 7, 1017–1029 | proposed (the draft's) |
| TE222 | The loopback backend is the foundation's; the development certificates are ADR 0006's marks | NOM | draft migration step 2, 1005–1009; 0006 spike part 2, 756–760; map ids (R2-TRN-REC5 owner 0006) | settled (the map's assignment) |
| TE223 | Part 11's lobby, tokens, reconnect, private rooms, lifecycle and server bots: slices 2 and 3; test tokens in the proof test | NOM | draft migration steps 4–5, 1014–1021 | proposed (the draft's); the test tokens proposed (test-engineer) |
| TE224 | Part 12 at slice 4 | NOM | draft migration step 6, 1022–1025 | proposed (the draft's) |
| TE225 | OQ-0001-4, -8, -10 and -9's melee: Feel Lab; -11, -12 design calls; -13 telemetry | NOM | draft OQs, 861–913 | proposed (the draft's) |
| TE226 | ADR 0006's operations marks and ADR 0004's S17 run on this spike's rig | NOM, SE | 0006, 738–760; 0004 S17 | settled (both drafts' texts) |
| TE227 | The room: 47 bot clients and Mustafa in the 48th slot | CL intro | draft part 11 (a human takes a bot's slot), 361–363; MODES-3 (a) N = 48 | proposed (test-engineer: N stays 48) |
| TE228 | Profile and cap switched from the overlay (a spike-only control) | CL intro | none | proposed (test-engineer) |
| TE229 | Step 1: playing within 3 s | CL 1 | r2 R2-TRN-REC3 mark 1, 2664–2665 | research verified |
| TE230 | Steps 2–3: own soldier and muzzle flash feel the same at P0 and P2 | CL 2–3 | r2 R2-SYNC-REC3 mark 3, 2105; draft part 4 ("no input lag for what the player controls"), 180–181 | research verified |
| TE231 | Step 4: a correction glides (fade 100 ms) unless over 2 m | CL 4 | draft part 4, 172–173 | proposed (the draft's) |
| TE232 | Step 5: fast movers' look decides Hermite | CL 5 | draft part 7 ("Hermite only if the proof test shows artefacts"), 258–259; R2-SYNC-OQ12 | proposed (the draft's) |
| TE233 | Step 6: respawns placed, not slid | CL 6 | r2 R2-SYNC-REC4 ("placed, not interpolated"), 2116; draft part 7, 258 | research verified |
| TE234 | Step 7: "the shot feels instant; hit feedback arrives when the hit is confirmed" | CL 7 | LOOP_V2 §7; draft Context, 35 | design doc |
| TE235 | Step 8: no damage number or kill before the hit marker | CL 8 | r1 R1-ABL-REC4 mark, 2444–2445 | research verified |
| TE236 | Step 9: shots land where clicked at every height | CL 9 | r2 R2-SYNC-REC8, 2271–2273 | research verified |
| TE237 | Steps 10–11: observations per cap feed OQ-0001-5's Feel Lab weighing; no pick | CL 10–11 | draft OQ-0001-5 ("the proof test's numbers …, then a Feel Lab weighing"), 867–873 | proposed (the draft's) |
| TE238 | Step 12: the repeats cover 8 ticks (267 ms) of a 1 s burst, so one correction may follow | CL 12 | draft part 5 (redundancy cap 8), 186; 8 × 33.3 = 267 ms; 2 m snap, 173 | arithmetic; proposed (the draft's cap) |
| TE239 | Step 12: within half a second the world catches up | CL 12 | r2 R2-SYNC-REC1 mark 2 (500 ms), 2020 | research verified |
| TE240 | Step 13: P3 and P4 test that play degrades gracefully | CL 13 | r2, 1991 | research verified |
| TE241 | Step 14: other soldiers may stall on loss over the fallback | CL 14 | draft Consequences ("head-of-line stalls under loss"), 815–816 | proposed (the draft's) |
| TE242 | Step 15: three verdicts: prediction, reconciliation, lag-compensated hits | CL 15 | GLOSSARY §8 the netcode proof test (map 1094–1095) | settled |
| TE243 | Spike name, what it proves, its owner 0001 | SE | map part g, 941 | settled (the map's assignment) |
| TE244 | Runs first, before the foundation, in parallel with the other phase 0 spikes | SE | SLICE-ORDER condition 3 (draft 401); map part g order, 962–965 | settled (SLICE-ORDER); the parallel order proposed (the map's) |
| TE245 | Dependencies: RS and SH orderable (OQ-0006-1, R6-HOST-OQ1); LG for 384 clients (8 × 48) | SE | 0006 OQ-0006-1, 598–611; 8 × 48 = 384 | settled (0006's OQ); arithmetic |
| TE246 | The 60 Hz run under (a) needs the sim core spike's sim to run the stress scene first | SE | draft OQ-0001-2, 849–852; map part g sim core spike row, 943 | proposed (the draft's) |
| TE247 | Size: L | SE | none | proposed (test-engineer: the work listed in the entry) |
| TE248 | Subsumes the research spikes of part 14, with the further recs listed | SE | draft part 14, 400–406; map part g, 941, 966–969 | settled (the draft's and the map's lists) |
| TE249 | Decides list (R2-TRN-OQ1, OQ2, OQ3, OQ5, OQ8; R2-BUD-OQ3, OQ7, OQ9, OQ10; R2-SYNC-OQ3, OQ5, OQ12; R2-ROOM-OQ1; R2-XU-OQ2, OQ4, OQ5, OQ6, OQ7, OQ15; OQ-0001-7; carry-forward 2; R1-TICK-OQ1 conditionally) | SE | draft Appendix A, 1075–1202 (the "spike" rows); map part g, 941 | settled (the draft's dispositions) |

## Counts
- Rows: 249 (TE1–TE249).
- By first tier (counted on this file by script): proposed 117 · research verified 84 · settled 29 · arithmetic 12 ·
  research, source to check 5 · design doc 2.
- Proposed by test-engineer (method choices, readings, counts and planted defects, each with its reason): 87 rows.
