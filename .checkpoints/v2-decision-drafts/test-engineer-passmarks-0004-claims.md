# Claims ledger: ADR-0004's pass marks (test-engineer), 2026-11-01

A notes file beside `test-engineer-passmarks-0004.md`, in the plan's ledger shape (rule 9). It joins
`rust-reviewer-adr0004-claims.md` [C15]; rows are numbered TE1, TE2 … so they never clash with that ledger's 1–360.
One row per claim or number in the pass-mark subsection and the spike entries. Lines are at 9e7db28 (the research
files are unchanged at the head: `git diff --quiet 9e7db28 HEAD -- docs/research/` exits 0, checked 2026-11-01). Short
names: r1 = `docs/research/r1-simulation-core.md`, r2 = `r2-netcode.md`, r3 = `r3-rendering.md`, r4 =
`r4-physics-navigation.md`, r5 = `r5-toolchain-quality.md`, r6 = `r6-infrastructure.md`, GL = `GLOSSARY.md`, INV =
`docs/LEGACY_INVENTORY.md`, draft = `rust-reviewer-adr0004-draft.md`, 0001 = `netcode-architect-adr0001-draft.md`, 0002
= `engine-architect-adr0002-draft.md`, PM3 = `test-engineer-passmarks-0003.md`, map =
`engine-architect-decision-map.md`, plan = `plan.md`.

Tiers, aligned with rust-reviewer's ledger for the same marks (its rows 301–350): `research verified` = an R1 or R2
recommendation's text, its proposed pass marks included, or a GLOSSARY definition built from them · `research, source
to check` = R3–R6 recommendations (their proposed marks included), any finding outside a verifier's checked set, an
open question or a legacy fact · `arithmetic` (shown) · `settled` (a STATUS code, GLOSSARY, the plan, or the map's and
the drafts' division of scope) · `design doc` (VISION_V2, LOOP_V2, LOOK_V2) · `proposed (the draft's)` (a mark the
ADR 0004 draft proposes, with its reason there) · `proposed (test-engineer)` (a method choice, reading, count or planted
defect of this table, with its reason). Every pass mark in R1–R6 is itself "proposed" in the research; the first tier
names where the number comes from. Where a row rests on several tiers, all are named, weakest last.

Mark codes: PRE the "How to read the table" bullets · PM-n the table row · FAIL the failure rule · NOM the "no
measurable mark" lines · NBC the "No Brave checklist" paragraph · SE the spike entries.

## How to read the table

| # | the claim or number (short) | where | citation (file, id, lines) | tier |
|---|---|---|---|---|
| TE1 | RS: Hetzner CCX23, 4 dedicated vCPU, 16 GB, x86-64 AMD, Linux, one EU location; a working pick while R6-HOST-OQ1 is open | PRE | r6 R6-HOST-REC2, 645–650; GL 225–230 | research, source to check (R6's proposal) |
| TE2 | Server marks on one vCPU with no other room on it | PRE | GL, the measuring unit, 244–250 | settled (GLOSSARY) |
| TE3 | Bench pinned to one vCPU, the VM otherwise idle; the pinning tool the harness's to document | PRE | none | proposed (test-engineer: a neighbour on the same VM moves a p99) |
| TE4 | CX33 measured beside RS, reported, never gated | PRE, PM-4 | r6 R6-HOST-REC2, 645–647 | research, source to check |
| TE5 | ML and RD are ADR 0003's machines, named and recorded by PM-0003-1 | PRE | r3 R3-BUDGET-REC1, 1096–1103; PM3 PM-0003-1 | research, source to check (R3's proposal); settled (ADR 0003's pass marks) |
| TE6 | ML run labels: browser and version, `hardwareConcurrency`, core count, mains power, power mode | PRE | r1 R1-BND-OQ3, 2728–2731; PM3 "H" bullet | research, source to check (OQ); proposed (test-engineer, as ADR 0003: a laptop's power state moves its numbers) |
| TE7 | ML's Wasm marks in Chrome and Firefox on Windows 11; Edge reported | PRE, PM-8 | r3 R3-BUDGET-REC1, 1097–1099 (Chrome and Firefox on ML) | proposed (test-engineer: the sim's speed differs by engine) |
| TE8 | MAC: macOS with Safari 26; real Safari through its WebDriver; model recorded; never a budget result | PRE | r5 R5-TEST-REC2, 1415–1418; PM3 MAC | research, source to check; proposed (test-engineer, as ADR 0003) |
| TE9 | CI: counts, hashes and checks only, never a time | PRE | r5 R5-TEST-REC1, line 1400 | research, source to check |
| TE10 | A64 only before ADR 0006 may pick ARM hosts | PRE, PM-26 | draft part 7, 238–240; r1 R1-RR-OQ1, 2929–2931 | research, source to check (RR risk); proposed (the draft's rule) |
| TE11 | No mark is judged on Mustafa's machine | PRE, NBC | draft spike intro, 818–819 | settled (the draft's statement; plan rule 12) |
| TE12 | SS: N = 48 in 16v16v16; 15 big vehicles, 4 very large units of at least 20 parts; 300 live projectiles; 8 active effects per soldier (384); one DM_Tri-sized greybox | PRE | GL §3, 397–420, 438–442; STATUS MODES-3 (a) | research verified (GL); settled |
| TE13 | Scale steps N = 12, 24, 36, 48 and the N = 64 headroom run, labelled headroom | PRE | GL 400–402, 355–359; r1 R1-TICK-REC2 P1, 2044 | research verified |
| TE14 | 15 big vehicles and 300 projectiles at every step; active effects per soldier | PRE | GL 400–405 | proposed (test-engineer: reading; GL counts only active effects per soldier) |
| TE15 | SS driven from a recording (seed, map and mode data, content hash, per-tick inputs) | PRE | draft part 9, 283–285; r1 R1-DET-REC6, 2223–2224 | research verified |
| TE16 | Dead soldiers respawn in the cluster next tick; projectiles and active effects topped up each tick; counts checked | PRE, PM-2 | none | proposed (test-engineer: a lighter scene would pass a budget falsely) |
| TE17 | O3: plateau, ramps, drop-downs, 6 m gaps | PRE | LOOP_V2 §9 | design doc |
| TE18 | TB: two floors with stairs, a window and an open stairwell; three damage stages | PRE | r4 R4-FLOORS-REC2 (a), 936–937; R4-LOS-REC1 mark 2, 978–979; draft S13, 835 | research, source to check; proposed (the draft's, for three stages) |
| TE19 | LB: ledges 2.5, 2.9, 3.0, 3.1 m; platforms 1.2, 2.5, 4.0 m with ramps | PRE | r4 R4-FLOORS-REC1, 918–920; R4-PHYS-REC1 mark 1, 693 | research, source to check |
| TE20 | BR: box-only scenes, legacy CS6's fuzz method | PRE | r4 R4-PHYS-REC2 (a), 739–740; R4-LOS-REC1 mark 1, 976–977 | research, source to check |
| TE21 | M10 = 18,000 ticks at 30 Hz | PRE | 600 × 30 = 18,000 | arithmetic |
| TE22 | T: 3 runs | PRE | r1 R1-TICK-REC2 P1, 2047 ("36,000 ticks per run, 3 runs") | research verified |
| TE23 | The 3 runs replay one recording | PRE | none | proposed (test-engineer: the same work each run, so V judges machine noise only) |
| TE24 | 900 warm-up ticks (30 s of game time) | PRE | 900 ÷ 30 = 30 s; PM3 "H" (300 warm-up frames, R3-MEASURE-REC1, r3 1181–1185) | proposed (test-engineer: caches fill and first allocations happen early); arithmetic |
| TE25 | 36,000 measured ticks = 1,200 s = 20 minutes at 30 Hz | PRE | r1 2047; 36,000 ÷ 30 = 1,200 | research verified; arithmetic |
| TE26 | Run labels: commit, machine, OS and kernel, compiler version, build flags | PRE | none | proposed (test-engineer: each moves a timing) |
| TE27 | Every one of the 3 runs meets the mark | PRE | PM3 "H" (R3-BUDGET-REC2, r3 1162–1163) | proposed (test-engineer: an average hides a failing run) |
| TE28 | Nearest rank: p99 = 35,640th, p95 = 34,200th, median = 18,000th of 36,000; 17,820th, 17,100th, 9,000th of 18,000 | PRE | ⌈0.99 × 36,000⌉ = 35,640; ⌈0.95 × 36,000⌉ = 34,200; ⌈0.5 × 36,000⌉ = 18,000; ⌈0.99 × 18,000⌉ = 17,820; ⌈0.95 × 18,000⌉ = 17,100; ⌈0.5 × 18,000⌉ = 9,000 | arithmetic; proposed (test-engineer: one definition, as ADR 0003) |
| TE29 | No statistic means p99; p99s never added; the step's total measured directly | PRE | GL, time statistics, 141–147 | settled (GLOSSARY) |
| TE30 | A one-off event has a plain limit | PRE | GL 146–147 | settled (GLOSSARY) |
| TE31 | V: 3 run medians within 3 %; an invalid set is neither pass nor fail | PRE | r3 R3-MEASURE-REC1, 1192–1193; PM3 V | research, source to check; proposed (test-engineer: R3's two-run mark applied to every timed set, as ADR 0003) |
| TE32 | Tick bench = a `devtools` replay runner mode; mean, p50, p95, p99, max per phase and total; 3 runs per build; host monotonic clock outside the sim | PRE | 0002 part 8, 409–415 | settled (division of scope: ADR 0002's bench method) |
| TE33 | Phase times through a bench entry that calls the phases one by one; the total timed around the ordinary step in its own runs | PRE | none | proposed (test-engineer: a timer import fails PM-0004-10, and the total must be measured directly) |
| TE34 | The bench reports its timer's resolution | PRE | none | proposed (test-engineer) |
| TE35 | `performance.now()` 5 µs isolated, 100 µs not | PRE | r3 R3-MEASURE-F7, 641–643 | research, source to check (R3 records it "confirmed in the source") |
| TE36 | A Wasm mark under 1 ms runs with the isolation flag, or passes with 0.1 ms to spare | PRE | PM3 "Timers" | proposed (test-engineer: one timer step, as ADR 0003) |
| TE37 | "Wasm ≤ 2 × native" read on one machine (ML), p99 against p99 per run pair | PRE, PM-7, PM-31 | r1 R1-ECS-REC1 mark 1, 1929; r4 R4-PHYS-REC1 mark 2, 703 | proposed (test-engineer: across RS and ML the ratio measures the two CPUs) |
| TE38 | 8.33, 13.33, 1.67 ms are 25 %, 40 %, 5 % of 33.33 ms; the draft and GLOSSARY round the first two | PRE | GL 82–83; 0.25 × 33.33 = 8.33; 0.40 × 33.33 = 13.33; 0.05 × 33.33 = 1.67 | arithmetic |
| TE39 | Every mark at 30 Hz; 60 Hz forms only in the 60 Hz set, with the 60 Hz margin's test; never summed | PRE | STATUS TICK-30; plan TICK-60-HEADROOM, 83–91; map J1 §8, 1103–1109 | settled |
| TE40 | The "at 30 and 60 Hz" halves of S3, S4, S9, S11, S13 wait in the 60 Hz set | PRE, PM-72 | draft S3, S4, S9, S11, S13 (825–826, 831, 833, 835); plan 85–88 ("every 60 Hz mark the research sets"); draft spike intro, 815–816 | proposed (test-engineer: reading of TICK-60-HEADROOM) |
| TE41 | Profiles P0–P4 and B1 (round trip, jitter, loss, values as listed) | PRE | r2, sync unit profiles, 1987–1990 | research verified |
| TE42 | The network simulator is ADR 0002's (R5-TEST-REC4's L1 and L2) | PRE | r5 R5-TEST-REC4, 1454–1460; map N27 | research, source to check; settled (division) |
| TE43 | P2 is VISION_V2 §4's "about 100 ms of ping" | PRE | map J6 (3), 1220–1221; 0001 OQ-0001-3, 856–860 | settled (map), pending the Advisor's confirmation (OQ-0001-3) |
| TE44 | "a check without a failing case can't be shown to work" | PRE | r5 R5-GATE-REC3, 1727–1728 | research, source to check |
| TE45 | Rule A: one planted 64-byte allocation per tick must be counted | PRE | PM3 PM-0003-29 | proposed (test-engineer: a counter proves zero only if it can see one) |
| TE46 | Rule B: a calibrated integer loop, the measured margin plus 1 ms, no clock read; adding a constant raises every percentile by it | PRE | PM3 rule B | proposed (test-engineer, as ADR 0003); arithmetic (a percentile of x + c is the percentile of x plus c) |
| TE47 | Tests that join the gate: 20 identical verdicts; fixed `PROPTEST_RNG_SEED` in the gate, fresh nightly | PRE | r5 R5-TEST-REC6, 1492–1494, 1503–1504 | research, source to check |

## The sim core spike (PM-0004-1 to PM-0004-61)

| # | the claim or number (short) | where | citation (file, id, lines) | tier |
|---|---|---|---|---|
| TE48 | The machines named and recorded before the first run | PM-1 | r3 R3-BUDGET-REC1 mark, 1114–1116; r1 R1-TICK-OQ6, 2662–2666 | research, source to check |
| TE49 | Load counts hold on every measured tick | PM-2 | GL 397–420 | research verified (GL); proposed (test-engineer: every tick) |
| TE50 | S14's bot set: legacy GN combat behaviour plus the carried cache | PM-2 | draft S14, 836; INV GN4–GN6, GN13; r4 R4-NAV-REC2, 859–861 | proposed (the draft's: "bots with full AI" has no research definition) |
| TE51 | Each bot logs a scan, a shot and a route every minute; the cache moves | PM-2 | none | proposed (test-engineer: an idle bot lightens the scene) |
| TE52 | A run failing PM-2 is invalid; bite: dead soldiers stay dead | PM-2 | none | proposed (test-engineer) |
| TE53 | p99 sim step time ≤ 8.33 ms at N = 48 on RS | PM-3 | r1 R1-TICK-REC2 P1, 2049–2050; R1-ECS-REC1 mark 1, 1926–1929; GL 189–191 | research verified; arithmetic (TE38) |
| TE54 | Mean, p50, p95, p99 and max reported; parts in their own rows; the rest reported | PM-3 | r1 2050; draft part 6, 209–214 | research verified |
| TE55 | parry3d's `enhanced-determinism` on during the budget runs | PM-3 | draft part 10, 296 | proposed (the draft's decision) |
| TE56 | Rule B in the movement phase | PM-3 | none | proposed (test-engineer) |
| TE57 | Scale steps and the N = 64 headroom run reported, parts beside the total | PM-4 | r1 R1-TICK-REC2 P1, 2044–2050 | research verified |
| TE58 | CX33 at N = 48 reported | PM-4 | r6 R6-HOST-REC2, 645–647 | research, source to check |
| TE59 | Legacy JS step 0.80–1.00 ms mean at 48, a reference, not a prediction | PM-4 | r1 R1-ECS-REC1 mark 1, 1930–1931; R1-TICK-OQ5, 2658–2660 | research verified |
| TE60 | Parallelism weighed only if PM-0004-3 fails; decides R1-ECS-OQ4, R1-TICK-OQ5 | PM-4 | draft part 1, 132–133; r1 R1-ECS-OQ4, 2622–2626 | research, source to check (OQ); proposed (the draft's rule) |
| TE61 | p99 ≤ 2 × mean at N = 48, same run | PM-5 | r1 R1-TICK-REC3, 2074–2076 | research verified |
| TE62 | Bite: stagger removed, slow work on 1 tick in 30, above the 1 % the p99 reads | PM-5 | 1 ÷ 30 = 3.3 % > 1 % | arithmetic; proposed (test-engineer) |
| TE63 | 0 heap allocations per step in steady state | PM-6 | draft part 6, 219–221; r1 R1-ABL-REC6, 2471–2472 | proposed (the draft's, from R1-ABL-REC6's rule) |
| TE64 | Counted with a counting global allocator | PM-6 | r1 R1-ECS-REC1 mark 2, 1933; R1-ABL-REC6, 2471–2472 | research verified |
| TE65 | Wasm in a Worker ≤ 2 × native | PM-7 | r1 R1-ECS-REC1 mark 1, 1929 | research verified |
| TE66 | Same machine, one OS, per run pair; all 3 pairs ≤ 2 | PM-7 | none | proposed (test-engineer) |
| TE67 | p99 ≤ 13.33 ms in Wasm in a dedicated Worker on ML | PM-8 | r1 R1-TICK-REC2 P2, 2054–2056; GL 214–216 | research verified; arithmetic |
| TE68 | N from R1-TICK-OQ7; reported at 12, 24, 36, 48 meanwhile | PM-8 | r1 R1-TICK-OQ7, 2667–2670; draft S2, 824 | research, source to check (OQ); proposed (the draft's) |
| TE69 | "Holds 30+ fps on Low" read as ADR 0003's ML row: median ≤ 28 ms, p95 ≤ 33.3 ms | PM-8 | r1 2056–2057 ("R3 owns the render budget"); r3 R3-BUDGET-REC2, 1123 | research verified (R1); research, source to check (R3's row); proposed (test-engineer: the reading) |
| TE70 | Runs in the boundary spike's Worker with the sim core spike's sim | PM-8 | draft S2, 824; map part g, 942; 0002 item 7, 918–921 | settled (division) |
| TE71 | Step times from the Worker's timer around the step, isolation flag on; frames in mode U | PM-8 | PM3 "U and P", "Timers" | proposed (test-engineer, as ADR 0003) |
| TE72 | 13 legacy intervals: 12 `fireInterval`s and 1 `burstInterval` | PM-9 | `docs/legacy-tuning.json` (counted by grep: 0.072, 0.09, 0.13, 0.2, 0.35, 0.42, 0.45, 0.7, 0.8, 0.85, 1.05, 3.4; burst 0.06) | research, source to check (the legacy tuning dump, R7) |
| TE73 | 60 s of held fire at 30 Hz: within 1 shot of 60 ÷ interval | PM-9 | r1 R1-TICK-REC1, 2023–2024 | research verified |
| TE74 | The burst interval run as a repeating timer | PM-9 | none | proposed (test-engineer: reading) |
| TE75 | Shabzon 0.072 s: 833.3 shots; 1 shot is 0.12 %, inside R1-ABL-REC3's 1 % | PM-9 | 60 ÷ 0.072 = 833.3; 1 ÷ 833.3 = 0.12 %; r1 R1-ABL-REC3, 2416–2417 | arithmetic; research verified |
| TE76 | CD13's 0.86 s within one tick (33.3 ms) | PM-9 | r1 R1-ABL-REC3, 2417–2418; INV CD13, line 94 | research verified |
| TE77 | Bite: reset-not-carry, 10.0 instead of 13.9 shots per second at 30 Hz | PM-9 | r1 R1-ABL-REC3, 2411–2412; INV GC5, line 244 | research verified |
| TE78 | 0 `std::time` uses; 0 host time imports in the `.wasm` | PM-10 | r1 R1-TICK-REC1, 2027 | research verified |
| TE79 | The import list read with a Wasm parser (tool ADR 0002's) | PM-10 | none | proposed (test-engineer) |
| TE80 | G2 mutation `Instant::now()` in a tick; control `Instant::now()` in `server` | PM-10 | r5 R5-GATE-REC3, 1714 | research, source to check |
| TE81 | The loader's rounding report lists every duration | PM-11 | r1 R1-ABL-REC3, 2403–2404; draft part 4, 167–171 | research verified |
| TE82 | Units per tick: no research number; chosen from the report | PM-11 | draft part 4, 171–172 | proposed (the draft's) |
| TE83 | 0 legacy intervals under one tick at 30 Hz (0.06 s against 0.0333 s) | PM-11 | r1 R1-ABL-OQ7, 2776–2778; 1 ÷ 30 = 0.0333 | research, source to check (OQ); arithmetic |
| TE84 | Bite: a planted 0.02 s interval is flagged | PM-11 | none | proposed (test-engineer) |
| TE85 | 0 allocations per snapshot after warm-up | PM-12 | r1 R1-ECS-REC1 mark 2, 1932–1934 | research verified |
| TE86 | Snapshot p99 ≤ 0.2 ms native, ≤ 0.5 ms Wasm | PM-12 | r1 R1-ECS-REC1 mark 2, 1935–1936 | research verified |
| TE87 | Wasm snapshot timing with the isolation flag on | PM-12 | r3 641–643 | proposed (test-engineer: 0.5 ms against a 100 µs timer step) |
| TE88 | Bite: a derived `clone_from` does not reuse buffers | PM-12 | r1 R1-ECS-OQ2, 2614–2618 | research, source to check (OQ, settled at R1's fix step) |
| TE89 | Restore and continue: per-tick hashes equal to an uninterrupted 10-minute bot match | PM-13 | r1 R1-ECS-REC1 mark 4, 1942–1944 | research verified |
| TE90 | A snapshot every 30th tick, restored 5 ticks later: 600 restores | PM-13 | 18,000 ÷ 30 = 600 | proposed (test-engineer: many different states, as a predicting client re-simulates); arithmetic |
| TE91 | Serialize → deserialize → serialize byte-identical at every 300th tick: 60 checks | PM-13 | r1 1944–1945; 18,000 ÷ 300 = 60 | research verified; arithmetic; proposed (test-engineer: the spacing) |
| TE92 | The streams continue identically after a restore | PM-13 | r1 R1-ECS-REC2 mark 3, 1971; R1-DET-REC4, 2206 | research verified |
| TE93 | Ids survive restore | PM-13 | r1 R1-ECS-REC3 mark 1, 1986 | research verified |
| TE94 | Bite: one stream's position left out of `clone_from` | PM-13 | none | proposed (test-engineer) |
| TE95 | Whole-world restore plus k = ⌈0.15 × 30⌉ = 5 ticks within 8 ms p99 in Wasm on ML | PM-14 | r1 R1-ECS-REC1 mark 3, 1938–1941; 0.15 × 30 = 4.5, up to 5 | research verified; arithmetic |
| TE96 | Not run under R2-SYNC-REC3's prediction scope | PM-14 | draft part 3, 153–155; r2 R2-SYNC-REC3, 2068–2105 | research verified (R2); proposed (the draft's) |
| TE97 | Property test, 10,000 cases: a despawned id never resolves | PM-15 | r1 R1-ECS-REC3 mark 2, 1987–1988 | research verified |
| TE98 | Sequences of up to 1,000 operations on 64 slots | PM-15 | none | proposed (test-engineer: a pool smaller than the sequence forces reuse) |
| TE99 | Bite: no generation bump on reuse | PM-15 | none | proposed (test-engineer) |
| TE100 | Server states applied on the client: 0 id misses; no id-keyed map (a reviewer's check) | PM-16 | r1 R1-ECS-REC3 mark 3, 1989 | research verified; proposed (test-engineer: the method) |
| TE101 | Two instances in one process equal their solo runs | PM-17 | draft part 1, 131–132; 0002 spike item 1, 887–888 (OQ-0002-3 (b)) | proposed (test-engineer: no research mark for part 1's rule) |
| TE102 | Two native processes: the same hash on 36,000 of 36,000 ticks | PM-18 | r1 R1-TICK-REC1, 2025–2026 | research verified |
| TE103 | The two processes' std `HashMap` seeds logged and different | PM-18 | r1 R1-ECS-REC2 mark 1, 1967–1968; R1-DET-REC5, 2218–2221 | research verified; proposed (test-engineer: the seed check) |
| TE104 | The replay runner built as the server builds the sim | PM-18 | r5 R5-TEST-REC2, 1413; R5-LAYOUT-REC4, 1206–1209 | research, source to check |
| TE105 | Bots-only 24v24 and 12v12v12, 10 minutes, 3 seeds: 0 mismatches native ×2 and against Wasm | PM-19 | r1 R1-DET-REC1, 2139–2144 | research verified |
| TE106 | No third match under MODES-3 (a) | PM-19 | STATUS MODES-3 (a); r1 2140–2141 | settled |
| TE107 | Wasm in Node, Chromium, Firefox, WebKit under Playwright; real Safari through its WebDriver | PM-19 | r5 R5-TEST-REC2, 1412–1418; r1 R1-DET-REC6, 2226–2227 | research, source to check (R5); research verified (R1) |
| TE108 | ML once, Chrome and Firefox on Windows | PM-19 | none | proposed (test-engineer: the players' platform, which CI's Linux runners are not) |
| TE109 | Nightly in CI after the spike | PM-19 | r5 R5-TEST-REC1, 1396; R5-TEST-REC2, 1419–1420 | research, source to check |
| TE110 | Flag on: 0 mismatches native against Wasm over 10 minutes; cost on and off reported, native and in a browser | PM-20 | r1 R1-DET-REC3, 2178–2182; R1-DET-OQ1, 2680 | research verified (as written for Rapier; the transfer to parry3d is the draft's part 10) |
| TE111 | Three injected divergences, each reported at its first tick with a field-level diff; hashing cost reported | PM-21 | r1 R1-DET-REC6, 2227–2228, 2238–2240; r5 R5-TEST-REC2, 1427–1428 | research verified |
| TE112 | Each injection armed at tick T with a value known to differ; `sin` input from the unit-vector table, else a 1-ulp stand-in; a `HashMap` of at least 32 keys, both orders logged | PM-21 | r1 R1-DET-REC5, 2218–2221 ("inferred, not certain") | proposed (test-engineer: an injection that changes no state proves nothing) |
| TE113 | The hash function named in the spike | PM-21 | draft part 9, 282–283 | proposed (the draft's: the research names none) |
| TE114 | Control: a presentation draw outside the sim passes | PM-21 | r5 R5-GATE-REC3, 1721 | research, source to check |
| TE115 | At least 10,000 inputs per `libm` function, bit-identical on x86-64 and Wasm per engine | PM-22 | r1 R1-DET-REC2, 2165–2167 | research verified |
| TE116 | The input table: edge cases and seeded values, committed | PM-22 | none | proposed (test-engineer) |
| TE117 | Wasm `sqrt` against a correctly rounded reference | PM-22 | r1 R1-DET-OQ10, 2710–2712 | research, source to check (OQ) |
| TE118 | Accuracy reported in ulps; "enough" read as PM-0004-30's and -54's marks passing with `libm` | PM-22 | r1 R1-DET-OQ8, 2704–2705 | research, source to check (OQ); proposed (test-engineer: the reading) |
| TE119 | `libm` against std cost reported, with method | PM-22 | r1 R1-DET-REC2, 2167–2168 | research verified |
| TE120 | Bite: one output moved by one ulp on one target | PM-22 | none | proposed (test-engineer) |
| TE121 | The build fails on a std transcendental, `algebraic_*`, `relaxed-simd` | PM-23 | r1 R1-DET-REC2, 2166–2168 | research verified |
| TE122 | The hash-map ban; a reviewed `#[allow]` for a lookup-only map outside the state passes | PM-23 | r1 R1-ECS-REC2 mark 2, 1969–1970; GL 729–734 | research verified (GL); settled |
| TE123 | The server builds for baseline x86-64, never `+fma` or `-C target-cpu=native` | PM-23 | draft part 7, 237–239; r1 R1-RR-OQ1, 2929–2931 | research, source to check (RR risk); proposed (the draft's rule) |
| TE124 | G2 mutations: a `HashMap` field in `sim`, `x.sin()` | PM-23 | r5 R5-GATE-REC3, 1714 | research, source to check |
| TE125 | NaN never enters: inputs checked at entry; finite assertions each tick in debug and test builds | PM-24 | r1 R1-DET-REC2, 2150–2152 | research verified |
| TE126 | Bite: a planted `0.0 / 0.0` | PM-24 | none | proposed (test-engineer) |
| TE127 | `simd128` on decides R1-DET-OQ2; off until then | PM-25 | r1 R1-DET-OQ2, 2682–2685; R1-DET-REC2, 2150 | research, source to check (OQ); research verified |
| TE128 | Native AArch64 in the hash test before ARM hosts | PM-26 | r1 R1-DET-REC2, 2166; R1-RR-OQ1, 2929–2931; draft part 7, 239–240 | research verified; research, source to check |
| TE129 | First 10,000 outputs of each stream identical on native and Wasm | PM-27 | r1 R1-DET-REC4, 2205 | research verified |
| TE130 | The sim's own sampling functions with pinned test vectors | PM-27 | r1 R1-DET-OQ5, 2691–2696; draft part 8, 254–256 | research, source to check (OQ) |
| TE131 | ChaCha8's cost per tick reported; decides whether R1-DET-OQ11 returns | PM-27 | r1 R1-DET-OQ11, 2713–2717 | research, source to check |
| TE132 | Bite: a stream seeded from the clock | PM-27 | none | proposed (test-engineer) |
| TE133 | Muting sound, moving the camera, changing the display rate change no hash | PM-28 | r1 R1-DET-REC4, 2206–2207 | research verified |
| TE134 | Displays at 60 Hz and 144 Hz (or the highest on hand), on RD | PM-28 | none | proposed (test-engineer) |
| TE135 | Bite: G10's presentation draw inside the sim | PM-28 | r5 R5-GATE-REC3, 1721 | research, source to check |
| TE136 | 0 bytes of authority-only streams in the predicting client's state | PM-29 | draft S8, 830; GL 525–532; STATUS SEC-SPREAD (a) | proposed (the draft's); settled |
| TE137 | Method: a byte search of each tick's serialization for every authority-only seed and state | PM-29 | none | proposed (test-engineer) |
| TE138 | Step up 0.55 m, stopped by 0.56 m | PM-30 | r4 R4-PHYS-REC1 mark 1, 693–694 | research, source to check |
| TE139 | Apex within 1 mm of the integrated value | PM-30 | r4 694–695 | research, source to check |
| TE140 | 0 tunnellings through 0.2 m in 10,000 moves at 13.27 m/s (6.3 × 1.62 × 1.3) | PM-30 | r4 695–698; 6.3 × 1.62 × 1.3 = 13.27 | research, source to check; arithmetic |
| TE141 | 0.44 m per tick at 30 Hz | PM-30 | r4 698; 13.27 ÷ 30 = 0.44 | arithmetic |
| TE142 | Run at the shipped tick, 30 Hz | PM-30, PM-59 | r4 694 ("at the shipped tick"); STATUS TICK-30 | settled |
| TE143 | Movement plus physics ≤ 1 ms native mean, p99 reported, apart from AI and abilities | PM-31 | r4 R4-PHYS-REC1 mark 2, 700–703; GL 55 | research, source to check; research verified (GL) |
| TE144 | Wasm ≤ 2 × native for movement and physics | PM-31 | r4 703 | research, source to check |
| TE145 | Static BVH nodes byte-identical on both targets | PM-32 | r4 R4-PHYS-REC1 mark 3, 704–705 | research, source to check |
| TE146 | BVH build ≤ 50 ms native at match start | PM-32 | r4 R4-PHYS-REC2 (d), 742–743 | research, source to check |
| TE147 | Restore plus index rebuild: 10,000 rays byte-identical | PM-32 | r4 R4-PHYS-REC2 (c), 741–742 | research, source to check |
| TE148 | The build timed 10 times; bite: another insertion order on Wasm | PM-32 | none | proposed (test-engineer) |
| TE149 | Sizes with parry3d, rapier3d and neither, after wasm-opt and gzip | PM-33 | r4 R4-PHYS-REC1 mark 4, 706–707; r1 R1-ECS-REC1 mark 5, 1946–1947 | research, source to check; research verified |
| TE150 | Smaller than a bevy_ecs 0.20 stub with default features off | PM-33 | r1 1946–1948 | research verified |
| TE151 | One-line incremental native rebuild ≤ 10 s on the dev machine | PM-33 | r1 1948 | research verified |
| TE152 | The rebuild timed 5 times, each within 10 s | PM-33 | GL 146–147 | proposed (test-engineer) |
| TE153 | Option B with autostep on: parity, cost, size; C unless B wins marks 1 and 2 | PM-34 | r4 R4-PHYS-REC1 mark 5, 708–711; R4-PHYS-OQ2, 1085 | research, source to check |
| TE154 | 0 `Err(Unsupported)`; the pairs listed | PM-35 | r4 R4-PHYS-REC1 mark 6, 712–717 | research, source to check |
| TE155 | The ground shape from OQ-0004-3 (R4-PHYS-OQ4) | PM-35 | draft OQ-0004-3, 758–764; r4 1090 | research, source to check (OQ) |
| TE156 | 20,000 random rays: the same hits as legacy's SIM | PM-36 | r4 R4-PHYS-REC2 (a), 739–740 | research, source to check |
| TE157 | Distances within the fixture's stated tolerance; the CS6 golden captured once from legacy | PM-36 | r5 R5-TEST-REC3, 1431–1437 | research, source to check |
| TE158 | 0 of 1,200 bot shots more than 1 cm above a ramp | PM-36 | r4 R4-PHYS-REC2 (b), 740–741 | research, source to check |
| TE159 | 0 tunnellings through 0.2 m in 10,000 shots at the fastest projectile speed, 30 Hz | PM-37 | r4 R4-PHYS-REC3, 758–759 | research, source to check |
| TE160 | A grenade bounces back in 1,000 of 1,000 throws | PM-37 | r4 759 ("100 % of 1,000") | research, source to check |
| TE161 | A fixed throw's landing point identical on native and Wasm | PM-37 | r4 759–760 | research, source to check |
| TE162 | 0 ticks with more than 1 cm inside a hull, 10 minutes, a crowd of 48, turning | PM-38 | r4 R4-PHYS-REC4, 779–782 | research, source to check |
| TE163 | The turning method picked: a nonlinear cast or an overlap test | PM-38 | r4 767–769; draft part 10, 305–306 | research, source to check |
| TE164 | No substeps at 30 Hz: 6 m/s is 0.2 m per tick (answers R1-TICK-OQ9) | PM-38 | r4 771–774; r1 R1-TICK-OQ9, 2675 | research, source to check; arithmetic (6 ÷ 30 = 0.2) |
| TE165 | The far corner of a 2 km map | PM-39 | r4 R4-PHYS-REC5, 786, 796–797; draft part 7, 231 | proposed (test-engineer: reading of "the largest planned map", whose size LOOP_V2 §9 leaves open) |
| TE166 | Distance per second within 0.1 % over a 10-minute route | PM-39 | r4 797–798 | research, source to check |
| TE167 | Each generator bakes O3 in ≤ 1 s native, ≤ 3 s in a browser Worker | PM-40 | r4 R4-NAV-REC1 mark 1, 831–832 | research, source to check |
| TE168 | On R1-TICK-REC2's reference hardware (RS, ML) | PM-40 | r4 830 | research, source to check |
| TE169 | 5 bakes per generator and target | PM-40 | none | proposed (test-engineer) |
| TE170 | ≥ 99.9 % of 10,000 stood points on a node; 0 of 10,000 node points unstandable for a 0.7 m soldier | PM-41 | r4 R4-NAV-REC1 mark 2, 833–834 | research, source to check |
| TE171 | A 6 m gap stays a route; every gap under 6 m reported | PM-42 | r4 R4-NAV-REC1 mark 3, 835 | research, source to check |
| TE172 | Reachability inside the sim crate from Node and CI, identical on native and Wasm | PM-42 | r4 R4-NAV-REC1 mark 4, 836–837 | research, source to check |
| TE173 | A planted 5.5 m gap must be reported | PM-42 | none | proposed (test-engineer) |
| TE174 | 10,000 path queries byte-identical; the baked data hashes the same | PM-43 | r4 R4-NAV-REC1 mark 5, 838–839 | research, source to check |
| TE175 | Comparison: node count, size, mean and p99 query, path length; pick by marks 1–5 first | PM-44 | r4 R4-NAV-REC1 mark 6, 840–841 | research, source to check |
| TE176 | Navigation p99 ≤ 1.67 ms at N = 48 and N = 64, with a carried cache | PM-45 | r4 R4-NAV-REC2, 859–863; GL 58 | research, source to check; research verified (GL); arithmetic |
| TE177 | 0 bots waiting more than 1 s for a route | PM-45 | r4 863 | research, source to check |
| TE178 | At most 2 replans per bot per second at p99, read over all bot-seconds | PM-45 | r4 863–864 | research, source to check; proposed (test-engineer: the reading) |
| TE179 | Stall rate ≤ 7.73 %, TL13's method, 3 runs per build | PM-45 | r4 864–865 | research, source to check |
| TE180 | Stall = alive and moved under 0.3 m in 3 s | PM-45, PM-46, PM-55 | INV TL13, line 398 | research, source to check (legacy fact) |
| TE181 | Bite: legacy GN13's per-call route budget (the storm) | PM-45 | INV GN13, line 278; r4 852–853 | research, source to check |
| TE182 | 48 bots, a 6 m choke, 10 minutes: 0 deadlocks over 3 s; hashes identical | PM-46 | r4 R4-NAV-REC3, 880–882 | research, source to check |
| TE183 | A deadlock read as TL13's stall test for a bot with a route | PM-46 | INV TL13, line 398 | proposed (test-engineer: a sourced definition for "deadlock") |
| TE184 | Avoidance ≤ 0.2 ms p99 native at N = 48 | PM-46 | r4 882; GL 56 | research, source to check; research verified (GL) |
| TE185 | Measured in SS, the share's scene; reported in the choke | PM-46 | GL 53–56 | proposed (test-engineer) |
| TE186 | 0 avoidance pairs across floors on TB | PM-46 | r4 883–884 | research, source to check |
| TE187 | A Worker build without bots skips the navigation data (size reported); with bots meets P2 | PM-47 | r4 R4-NAV-REC4, 896–897 | research, source to check |
| TE188 | 0 jump links above the reach measured at 30 Hz | PM-48 | r4 R4-FLOORS-REC1 (a), 919–920 | research, source to check |
| TE189 | "2.9 m kept, 3.0 and 3.1 m rejected" is the research's 60 Hz form; legacy reach 2.98 m | PM-48, PM-72 | r4 919–920 | research, source to check |
| TE190 | Each kept link: 1,000 of 1,000 traversals | PM-48 | r4 R4-FLOORS-REC1 (b), 921 | research, source to check |
| TE191 | 0 % of routes need an unreachable surface (legacy Cabana 27.5 %) | PM-48 | r4 R4-FLOORS-REC1 (c), 921–922 | research, source to check |
| TE192 | Drop-down when shorter in ≥ 95 % of 200 trials | PM-48 | r4 R4-FLOORS-REC1 (d), 922–923 | research, source to check |
| TE193 | Bite: closed-form reach, about 9 cm over the integrated jump at 60 Hz | PM-48, PM-59 | r4 913–914; INV GW14, line 202 | research, source to check |
| TE194 | Floors: different layer and area ids; edges only along stairs; 100 of 100; roof not walkable | PM-49 | r4 R4-FLOORS-REC2, 936–939 | research, source to check |
| TE195 | Stale check 1 of 1; hash matches after the re-bake | PM-50 | r4 R4-FLOORS-REC3, 951–952 | research, source to check |
| TE196 | Called from Node (the editor's path) and CI | PM-50 | none | proposed (test-engineer: the spike has no editor) |
| TE197 | 20,000 eye-target pairs against legacy `hasLOS`, margin 0.5 m | PM-51 | r4 R4-LOS-REC1 mark 1, 976–977 | research, source to check |
| TE198 | 0 of 10,000 through floors; 100 % through openings | PM-51 | r4 mark 2, 978–979 | research, source to check |
| TE199 | 0 of 10,000 ramp disagreements | PM-51 | r4 mark 3, 980–981 | research, source to check |
| TE200 | The full pass p99 ≤ 1.67 ms at 24v24 and 16v16v16, three points | PM-52 | r4 R4-LOS-REC1 mark 4, 982–985; GL 59 | research, source to check; research verified (GL); arithmetic |
| TE201 | One segment query, mean and p99, native and in a browser Worker | PM-52 | r4 982–983 | research, source to check |
| TE202 | The pass's results hash identically each tick | PM-52 | r4 mark 5, 986 | research, source to check |
| TE203 | Every bot at least 4 scans per second in one fight, within the share | PM-53 | r4 R4-LOS-REC2, 1002–1005 | research, source to check |
| TE204 | Read as the minimum over bot-seconds | PM-53 | none | proposed (test-engineer: "every bot … per second") |
| TE205 | Culled share and rays per tick reported | PM-53 | r4 1005–1006 | research, source to check |
| TE206 | Bite: legacy GN4's budget, about 2.5 scans per bot per second at 48 | PM-53 | r4 996–997; INV GN4, line 269 | research, source to check |
| TE207 | A downward cursor ray within 1 cm of the slope in 1,000 of 1,000 | PM-54 | r4 R4-LOS-REC3, 1015–1016 | research, source to check |
| TE208 | R2-SYNC-REC8 mark 1 is ADR 0001's | PM-54 | draft S12, 834 | settled (division) |
| TE209 | Bite: ramps as boxes, legacy GC3's 1.25 m | PM-54 | r4 1012, 1016 | research, source to check |
| TE210 | SS 10 minutes: 0 stuck over 3 s against a vehicle; ≤ 2 replans p99; hashes; marking work reported | PM-55 | r4 R4-OBST-REC1, 1037–1041 | research, source to check |
| TE211 | The hull footprint form picked (R4-OBST-OQ4) | PM-55 | r4 1145 | research, source to check (OQ) |
| TE212 | 1,000 cycles: 0 ticks of disagreement; 1,000 traversals; a flip ≤ 0.1 ms native p99 | PM-56 | r4 R4-OBST-REC2, 1058–1061 | research, source to check |
| TE213 | 2,000 flips | PM-56 | 2 × 1,000 | arithmetic |
| TE214 | The portal flag is the area connectivity ADR 0001's re-flood reads | PM-56 | draft part 14, 373–375; r4 1047–1048 | research, source to check; proposed (the draft's) |
| TE215 | Three stages, a plank link only in the ruin, the bake within PM-0004-40's limits | PM-57 | draft S13, 835; LOOK_V2 §4 | proposed (the draft's); design doc |
| TE216 | Bot reaction reported, no mark | PM-58 | r2 R2-SYNC-OQ13, 2952–2954; draft S14, 836 | research, source to check (OQ) |
| TE217 | Median, p95 and max, counted in ticks | PM-58 | none | proposed (test-engineer) |
| TE218 | Reach within 1 mm for `T.jumpV` 11, `T.gravity` 24, `T.stepUp` 0.55 | PM-59 | r1 R1-TICK-REC6, 2124–2125 | research verified |
| TE219 | 0 panics over fuzzed commands fed to the sim | PM-60 | draft S20, 842 | proposed (the draft's: part 20's rule needs its own test) |
| TE220 | 10 million fuzzed commands | PM-60 | r2 R2-TRN-REC4 mark 4, 2696 | proposed (test-engineer: ADR 0001's decoder count) |
| TE221 | Nightly cargo-fuzz on Linux | PM-60 | r5 R5-TEST-REC1, 1399; R5-TEST-REC4, 1460–1461 | research, source to check |
| TE222 | Bite: a planted `.unwrap()` on a client-sent pickup id | PM-60 | draft part 20, 463–465 | proposed (test-engineer) |
| TE223 | A trap ends in a fresh instance back in step through ADR 0001's resync | PM-61 | draft part 20, 466–467; r1 R1-RR-OQ3, 2936–2940 | research, source to check (RR risk); proposed (the draft's) |
| TE224 | The join mark: a complete world within 300 ms at 100 ms round trip and 2 % loss | PM-61 | r2 R2-BUD-REC3, 2359; 0001 trap row 550, spike item 2, 952 | research verified |
| TE225 | "Back in step" read as the client-held state's hash equal to the server's | PM-61 | GL, state hash, 546–550 | proposed (test-engineer: a definition) |
| TE226 | The trap reaches Sentry with Rust names if the EU organisation exists, else the page's log | PM-61 | r6 R6-OBS-REC3, 765–766; 0002 item 10, 929–930 | research, source to check |
| TE227 | 20 traps per browser at random ticks, 20 of 20 | PM-61 | none | proposed (test-engineer) |
| TE228 | At P2 | PM-61 | r2 1989; map J6 (3) | research verified; settled |

## The content and ability spike (PM-0004-62 to PM-0004-66)

| # | the claim or number (short) | where | citation (file, id, lines) | tier |
|---|---|---|---|---|
| TE229 | The ability, effect and weapon step p99 ≤ 1 ms with 384 active effects, RS and ML in Wasm; 0 allocations | PM-62 | r1 R1-ABL-REC6, 2465–2472; GL 57; 48 × 8 = 384 | research verified; arithmetic |
| TE230 | The isolation flag on for ML | PM-62 | r3 641–643 | proposed (test-engineer: a 1 ms mark against a 100 µs step) |
| TE231 | 120 s scripted scenario, every test ability, 3 seeds: byte-identical logs on native and Wasm | PM-63 | r1 R1-ABL-REC3, 2418–2419 | research verified |
| TE232 | 120 s = 3,600 ticks at 30 Hz | PM-63 | 120 × 30 | arithmetic |
| TE233 | Also in the three engines | PM-63 | none | proposed (test-engineer: PM-0004-19's targets) |
| TE234 | 0 track entries lost or changed over deaths and switches | PM-64 | UPGRADE-SCOPE; draft S16, 838 | settled; proposed (the draft's) |
| TE235 | Each of 48 slots dies at least 10 times and switches at least 10 times in 18,000 ticks | PM-64 | none | proposed (test-engineer: the numbers S16 asks test-engineer to set) |
| TE236 | A weapon type's track resumes with the next gun of that type | PM-64 | UPGRADE-SCOPE ("progress belongs to the weapon type"); draft part 18, 430–431 | settled |
| TE237 | Property test, 10,000 cases | PM-64 | draft test plan, 873–874; r1 R1-ECS-REC3 mark 2, 1987–1988 (the count) | proposed (test-engineer: the same count as PM-0004-15) |
| TE238 | Bite: a track on the Pawn | PM-64 | UPGRADE-SCOPE ("the tracks live on the PlayerState, not the Pawn") | settled; proposed (test-engineer: the bite) |
| TE239 | 100 of 100 consecutive matches start with empty records | PM-65 | UPGRADE-SCOPE; LOOP_V2 §8 | settled; design doc; proposed (test-engineer: the count) |
| TE240 | 0 allocations in the record table after match start | PM-65 | draft part 18, 431–433 | proposed (the draft's rule; the test test-engineer's) |
| TE241 | Pickups keep their ids through restores | PM-66 | draft S16, 838 | proposed (the draft's) |
| TE242 | Ties resolve the same on native and Wasm in 1,000 of 1,000 | PM-66 | draft part 18, 437–438; S16 | proposed (test-engineer: the count) |
| TE243 | Bite: claims iterated in a std `HashMap`'s order | PM-66 | none | proposed (test-engineer) |

## The netcode proof test, the boundary spike and the 60 Hz set (PM-0004-67 to PM-0004-72)

| # | the claim or number (short) | where | citation (file, id, lines) | tier |
|---|---|---|---|---|
| TE244 | Server tick start jitter p99 ≤ 2 ms over 36,000 ticks on RS (Linux) | PM-67 | r1 R1-TICK-REC4, 2090–2091 | research verified |
| TE245 | Back on schedule within 1 s after a 200 ms stall | PM-67 | r1 2091 | research verified |
| TE246 | Never more than 7 ticks per wake: ⌊0.25 × 30⌋ = 7 (the research's 15 at 60 Hz) | PM-67 | r1 2082–2083, 2092; draft part 5, 184–185 | proposed (the draft's restatement); arithmetic |
| TE247 | "Back on schedule" = no ticks owed and later starts within the jitter mark | PM-67 | none | proposed (test-engineer: a definition) |
| TE248 | A 1 s stall: at most 7 ticks in that wake; dropped time counted | PM-67 | draft part 5, 185–187; r1 2083 | research verified; proposed (test-engineer: the check) |
| TE249 | Overrun and dropped-time counters in the metrics output | PM-67 | r1 2092 | research verified |
| TE250 | Run in the netcode proof test under its 30 Hz load; 3 runs | PM-67 | map part g, 941 (R1-TICK-REC4 in the proof test); draft S17, 839 | settled (map); proposed (test-engineer: the runs) |
| TE251 | Bite: ticks counted from the timer period (Tokio's millisecond timers) | PM-67 | r1 2081–2082 | research verified (the reason); proposed (test-engineer: the bite) |
| TE252 | Worker tick start jitter p99 ≤ 4 ms over 10 minutes, four browsers | PM-68 | r1 R1-TICK-REC5, 2109–2110 | research verified |
| TE253 | 10 minutes = 18,000 ticks | PM-68 | 600 × 30 | arithmetic |
| TE254 | W2 (`setTimeout` plus a `performance.now()` accumulator); W3 on an isolated build if W2 fails | PM-68 | r1 2096–2097; 0002 item 7, 920–921 | research verified; settled (division) |
| TE255 | 3 runs per browser | PM-68 | none | proposed (test-engineer) |
| TE256 | Displacement SD ≤ 5 % of its mean on 60, 120 and 144 Hz displays | PM-69 | r1 R1-TICK-REC5, 2111–2112 | research verified |
| TE257 | Mode P | PM-69 | PM3 "U and P" | proposed (test-engineer, as ADR 0003) |
| TE258 | 3 runs of 60 s per display and browser | PM-69 | none | proposed (test-engineer) |
| TE259 | 60 s hidden: local play no tick, resumes; online back in step within 0.5 s | PM-70 | r1 R1-TICK-REC5, 2113–2114 | research verified |
| TE260 | Never more than 7 ticks per frame (the research's 15 at 60 Hz) | PM-70 | r1 2114; draft S18, 840 | proposed (the draft's restatement) |
| TE261 | More than 5 minutes hidden, run as 6 minutes | PM-70 | r1 R1-BND-OQ2, 2724–2727 | research, source to check (OQ); proposed (test-engineer: 6) |
| TE262 | "Back in step" = server state applied and commands accepted again | PM-70 | none | proposed (test-engineer: a definition) |
| TE263 | Online at P2 | PM-70 | r2 1989 | proposed (test-engineer: R1-TICK-REC5 names no profile) |
| TE264 | 5 hides per duration and browser | PM-70 | none | proposed (test-engineer) |
| TE265 | At 60 Hz: p99 sim step time ≤ 4.17 ms (25 % of 16.67 ms), max ≤ 16.67 ms, zero overruns | PM-71 | r1 R1-TICK-REC2 P1, 2047–2049; GL 53–60; 0.25 × 16.67 = 4.17 | research verified; arithmetic |
| TE266 | Navigation and visibility pass ≤ 0.83 ms each at 60 Hz | PM-71 | GL 58–59; 0.05 × 16.67 = 0.83 | research verified (GL); arithmetic |
| TE267 | The client's sim share at 60 Hz ≤ 6.67 ms | PM-71 | r1 R1-TICK-REC2 P2, 2055–2056; GL 215–216; 0.40 × 16.67 = 6.67 | research verified; arithmetic |
| TE268 | Read inside ADR 0001's one 60 Hz run with R2-BUD-REC6's 30-minute soak; OQ-0001-2 (a); never summed | PM-71 | map J1 §8, 1103–1109; plan 83–91; 0001 OQ-0001-2, 846–855, spike item 3, 965–967 | settled |
| TE269 | Only if ADR 0001 schedules the run | PM-71 | map J6 (2), 1215–1219 | settled |
| TE270 | The same shot count at 30 and 60 Hz within 1 shot | PM-72 | r1 R1-TICK-REC1, 2023–2024 | research verified |
| TE271 | Sight checks per game second per bot equal within 1 % at 30 and 60 Hz | PM-72 | r1 R1-TICK-REC3, 2075–2076 | research verified |
| TE272 | R4-PHYS-REC3, REC4, R4-FLOORS-REC1, R4-OBST-REC1 at 60 Hz | PM-72 | r4 758–759, 779–784, 919–921, 1037–1038 | research, source to check |
| TE273 | Durations re-converted and links re-baked for 60 Hz | PM-72 | draft Trade-off, 697–699; Consequences, 716–717 | proposed (the draft's) |

## The failure rule, the no-mark lines, the Brave paragraph and the spike entries

| # | the claim or number (short) | where | citation (file, id, lines) | tier |
|---|---|---|---|---|
| TE274 | A spike passes when every applicable mark passes; conditional, reported and decision marks as stated; PM-0004-8 waits for R1-TICK-OQ7 | FAIL | PM3 failure rule | proposed (test-engineer, as ADR 0003) |
| TE275 | No re-run until green; data kept; cause found | FAIL | r5 R5-TEST-REC6, 1490–1498 | research, source to check |
| TE276 | A failed mark goes back to the draft by its Rollback routes | FAIL | draft Rollback, 891–904 | proposed (the draft's) |
| TE277 | A mark run in another draft's spike comes back to this draft | FAIL | none | proposed (test-engineer: the decision part is this record's) |
| TE278 | Numbers change only in the draft; budgets are data | FAIL | r5 R5-GATE-REC4, 1734–1738 | research, source to check |
| TE279 | A failed V or PM-0004-2 is neither pass nor fail | FAIL | none | proposed (test-engineer) |
| TE280 | The tick rate is ADR 0001's | NOM | STATUS TICK-30; map N2, 726 | settled |
| TE281 | Local recovery after a trap: OQ-0004-7 | NOM | draft OQ-0004-7, 792–800 | proposed (the draft's) |
| TE282 | Bot structure: OQ-0004-1 | NOM | draft OQ-0004-1, 742–749 | proposed (the draft's) |
| TE283 | Part 17's check is a recorded full match in slice 4 | NOM | draft migration phase 6, 860 | proposed (the draft's) |
| TE284 | Part 18's open rules (OQ-0004-5, -6; ADR 0005's three) | NOM | draft 773–791; plan 74–75 | settled; proposed (the draft's) |
| TE285 | Part 19: the event list is in the state hash | NOM | draft part 19, 452 | proposed (the draft's) |
| TE286 | S20's content half is ADR 0005's; the suite's CI time under 3 minutes is ADR 0002's | NOM | draft S20, 842; r5 R5-TEST-REC2, 1428–1429 | research, source to check; settled (division) |
| TE287 | No network outside PM-0004-61 and -70 | NOM | draft test plan, 884–885 | proposed (the draft's) |
| TE288 | No claim is judged by a person | NBC | draft 818–819; plan rule 12, 142–147 | settled |
| TE289 | The netcode proof test's checklist is ADR 0001's; the boundary spike's is ADR 0002's | NBC | 0001 spike item 4, 968–969; 0002, 932–933 | settled (division) |
| TE290 | Spike names, what each proves, owners | SE | map part g, 941–944 | settled (the map's assignment) |
| TE291 | The sim core spike runs in parallel with the netcode proof test; its sim or step times feed the 60 Hz margin's run | SE | map part g, 943, 962–965; 0001 OQ-0001-2, 846–855 | settled (the map's proposal, for SPIKE_PLAN) |
| TE292 | Dependencies: R6-HOST-OQ1; ML and RD named; a Mac; the scenes; the bot set; the goldens (R5-TEST-REC3); throwaway tools in ADR 0002's shape | SE | map part g, 943; r5 1431–1437; 0002 part 8, 409–415 | settled; research, source to check |
| TE293 | Sizes: the sim core spike L; ADR 0004's shares S (content), S (netcode), M (boundary) | SE | none | proposed (test-engineer: the work listed in each entry) |
| TE294 | Subsumes and decides lists | SE | map part g, 941–944; draft Appendix A, 909–997 | settled (the map's assignment; the draft's dispositions) |
| TE295 | The map places R2-SYNC-REC6 mark 1 and R5-LAYOUT-REC4 in the sim core spike; the draft's spike table names neither | SE | map 943; r2 R2-SYNC-REC6 mark 1, 2217–2218; r5 R5-LAYOUT-REC4, 1206–1218; draft 821–842 | settled (both texts, read 2026-11-01) |
| TE296 | ADR 0001's spike section does not name R1-TICK-REC4 | SE | 0001, 915–993 (grep: 0 hits); map 941 | settled (both texts, read 2026-11-01) |
| TE297 | ADR 0002's item 7 names S18 and R1-TICK-REC2 P2 | SE | 0002, 918–921 | settled |
| TE298 | R1-TICK-OQ2 and R1-BND-OQ2 are one measurement (R1-XU-OQ12) | SE | draft part 5, 194–195 | proposed (the draft's) |

## Counts
- Rows: 298 (TE1–TE298), counted by script on this file; every one of PM-0004-1 to PM-0004-72 has at least one row.
- By first tier (script on this file): research verified 65 · research, source to check 106 · settled 29 · arithmetic
  9 · design doc 1 · proposed 88 (the draft's 20; test-engineer's 68). Further tiers inside a row are named there.
- Proposed by test-engineer (method choices, readings, counts and planted defects, each with its reason): 87 rows name
  it at some tier.
