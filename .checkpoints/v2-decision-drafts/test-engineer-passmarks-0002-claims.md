# Claims ledger: ADR-0002's pass marks (test-engineer), 2026-11-01

A notes file beside `test-engineer-passmarks-0002.md`, in the plan's ledger shape (rule 9). It joins
`engine-architect-adr0002-claims.md` [C15]; rows are numbered TE1, TE2 … so they never clash with that ledger's
1–261. One row per claim or number in the pass-mark subsection and the spike entries. Lines are at 9e7db28 (the
research files are unchanged at the head: `git diff --quiet 9e7db28 HEAD -- docs/research/r1-simulation-core.md …
r6-infrastructure.md` exits 0, checked 2026-11-01). Short names: r1 = `docs/research/r1-simulation-core.md`, r2 =
`r2-netcode.md`, r3 = `r3-rendering.md`, r5 = `r5-toolchain-quality.md`, r6 = `r6-infrastructure.md`, GL =
`docs/research/GLOSSARY.md`, draft = `engine-architect-adr0002-draft.md`, map = `engine-architect-decision-map.md`,
0001 / 0003 / 0004 / 0005 = the other drafts' notes files (`netcode-architect-adr0001-draft.md`,
`test-engineer-passmarks-0003.md`, `rust-reviewer-adr0004-draft.md`, `engine-architect-adr0005-content-section.md`).

Tiers (the plan's): `research verified` = an R1 or R2 recommendation claim (and the R1 findings those rest on, r1
lines 3233–3256), or a finding in a topic verifier's checked set (R5: r5 lines 2108–2222; R3: a finding marked
"confirmed in the source") · `research, source to check` = any other finding, open question or recommendation text
of R3–R6 · `arithmetic` (shown) · `settled` (a STATUS code, a GLOSSARY entry, the plan, or the map's assignment) ·
`design doc` (VISION_V2, LOOP_V2, EDITOR_V2, LOOK_V2) · `proposed` ("proposed (R5's)" and the like for a research
topic's proposed mark; "proposed (the draft's)" for engine-architect's; "proposed (test-engineer)" for my own method
choices, readings and planted defects, each with its reason). Two extra labels, as the drafter's ledger uses them:
`cross-draft` (a statement about another draft, checked in the consistency pass) and `inventory` (a fact read in the
repository's own files).

Mark codes: PRE the "How to read the table" bullets · PM-n the table row · NOTE the notes under the table · FAIL the
failure rule · NOM the "no measurable mark" lines · SC the spike checklist (step n) · FC the foundation checklist
(step n) · SE the spike entries.

## How to read the table

| # | the claim or number (short) | where | citation (file, id, lines) | tier |
|---|---|---|---|---|
| TE1 | ML: the minimum laptop ADR 0003 names; Windows 11; Chrome, Edge, Firefox | PRE | r3 R3-BUDGET-REC1, 1096–1103; OQ-0003-1 (0003) | proposed (R3's) |
| TE2 | ML's own panel is 60 Hz unless a row names another display | PRE | none | proposed (test-engineer: a laptop's usual panel; the real rate is recorded by PM-1) |
| TE3 | MAC: a macOS machine with the latest Safari; the draft names none; its model recorded | PRE | r1 R1-BND-REC1 mark 3, 2263 ("the latest desktop Chrome, Edge, Firefox and Safari"); draft 878–936 (no Mac named) | research verified; proposed (test-engineer: record the model) |
| TE4 | MP: Mustafa's PC, Windows, RTX 5080, in Brave; never a budget result | PRE | r3 R3-BUDGET-REC1, 1102–1103 ("a headroom and Brave check, never the recommended reference"); CLAUDE.md (Mustafa plays in Brave on Windows) | proposed (R3's); settled |
| TE5 | CI: ubuntu-24.04, 4 CPUs, 16 GB, no GPU | PRE | r5 R5-CI-F17, 673–676 | research, source to check |
| TE6 | CI gives counts, never a frame or hardware time | PRE | r3 R3-MEASURE-REC3, 1206–1209; draft part 8 ("timings on shared CI runners are not hardware results", 416–417) | proposed (R3's); proposed (the draft's) |
| TE7 | BX uses the netcode proof test's throwaway sim (movement, hitscan, bots as protocol clients) | PRE | 0001 spike section ("a throwaway one with movement, hitscan and bots as protocol clients"); map part g, line 941 | cross-draft |
| TE8 | BX at the stress scene's counts: N soldiers in three teams in one fight (16v16v16 at 48), 15 big vehicles, 4 very large units, 300 live projectiles | PRE | GL 397–420 | settled |
| TE9 | Very large units of 20 parts | PRE | GL 438–442 (at least 20 parts) | settled (the minimum the entry sets) |
| TE10 | The scale steps 12, 24, 36 in three equal teams | PRE | GL 400–402 (the steps) | proposed (test-engineer: the glossary names the steps, not their teams) |
| TE11 | Scripted movers fill the sections the stand-in does not simulate | PRE | none | proposed (test-engineer: the boundary's cost depends on the counts, not the rules) |
| TE12 | The page draws with the plain harness renderer, not the look | PRE | plan rules 6 and 10 (the look is ADR 0003's) | proposed (test-engineer) |
| TE13 | BX-P: the Worker's step padded to 13.3 ms per tick | PRE | GL 214–218 (the client's sim share, 13.3 ms at 30 Hz) | settled (the number); proposed (test-engineer: the padding, the real sim's cost being unknown until the sim core spike) |
| TE14 | The 10-minute match: 600 s × 30 = 18,000 ticks | PRE | TICK-30; r1 R1-BND-REC2 marks 2 and 4, 2277–2281 ("a 10-minute match") | arithmetic |
| TE15 | Online: the proof test's room process over WebTransport through its link impairment | PRE | 0001 spike section items 1 and 4 | cross-draft; proposed (test-engineer) |
| TE16 | P0 = 0 ms, no jitter, 0 % loss; P2 = 100 ms round trip, ±20 ms, 2 % | PRE | r2, 1987–1990 | research verified |
| TE17 | H: 3 runs of 600 frames after 300 warm-up | PRE | r3 R3-MEASURE-REC1, 1181–1185 | proposed (R3's) |
| TE18 | Frame time from `performance.now()`, never the rAF timestamp (1 ms precision) | PRE | r3 R3-MEASURE-F7, 641–643 ("confirmed in the source", 646) | research verified |
| TE19 | Nearest-rank percentiles: 570th of 600; 17,100th and 17,820th of 18,000 | PRE | ⌈0.95 × 600⌉ = 570; ⌈0.95 × 18,000⌉ = 17,100; ⌈0.99 × 18,000⌉ = 17,820 | arithmetic; proposed (test-engineer: one definition, as ADR 0003's pass marks) |
| TE20 | V: run medians within 3 %; an invalid set neither passes nor fails | PRE | r3 R3-MEASURE-REC1 mark, 1192–1193 | proposed (R3's); proposed (test-engineer: applied to every timed set, as ADR 0003's) |
| TE21 | A count mark's statistic is the run's max | PRE | none | proposed (test-engineer, as ADR 0003's) |
| TE22 | No statistic means p99; a one-off duration is a plain limit | PRE | GL 141–147 | settled |
| TE23 | U and P modes | PRE | 0003 "How to read the table" (U and P) | proposed (test-engineer, as ADR 0003's) |
| TE24 | `performance.now()` at 100 µs without isolation, 5 µs with it | PRE | r3 R3-MEASURE-F7, 641–643 | research verified |
| TE25 | Marks under 1 ms run on the isolation-flag build | PRE | 0003 "Timers" bullet | proposed (test-engineer, as ADR 0003's) |
| TE26 | Shared clock `performance.timeOrigin + performance.now()`; a worker has its own time origin | PRE | r1 R1-BND-REC2 mark 3, 2278–2280; R1-BND-F30 (r1, 1104) | research verified |
| TE27 | The counting build (export counters, page stubs) | PRE | none | proposed (test-engineer: counts that cost the same on both sides of a comparison) |
| TE28 | 30 Hz tick, 30 blocks a second; no 60 Hz mark here | PRE | TICK-30; GLOSSARY §8, the 60 Hz margin (map J1, 1103–1116) | settled |
| TE29 | The command shim has no network profile (posted messages are reliable) | PRE | r1 R1-BND-REC3, 2290–2291 (posted messages copied by structured clone, F34) | proposed (test-engineer reading) |
| TE30 | Bites: "a check without a failing case can't be shown to work" | PRE | r5 R5-GATE-REC3, 1727–1728 | proposed (R5's) |
| TE31 | Rule B: a busy-wait of the margin plus 1 ms must fail a budget mark | PRE | 0003 "Bites" bullet | proposed (test-engineer, as ADR 0003's) |
| TE32 | `--prove`: a mutation fails exactly its check; a control passes | PRE | r5 R5-GATE-REC3, 1706–1709 | proposed (R5's) |

## The boundary spike's rows

| # | the claim or number (short) | where | citation (file, id, lines) | tier |
|---|---|---|---|---|
| TE33 | ML named with CPU, GPU, RAM, screen; `hardwareConcurrency` recorded | PM-1 | r1 R1-BND-REC1 mark 1, 2259–2261; R1-BND-OQ3, 2727–2730; r3 R3-BUDGET-REC1 mark, 1114–1116 | research verified; proposed (R3's) |
| TE34 | Refresh rates, displays and build commit recorded on every row | PM-1 | none | proposed (test-engineer: pacing and latency depend on the display) |
| TE35 | 0 calls into Wasm and 0 instantiations by the page | PM-2 | r1 R1-BND-REC1 mark 2, 2261–2262 | research verified |
| TE36 | Bite: a debug readout that instantiates `sim-wasm` on the page | PM-2 | none | proposed (test-engineer) |
| TE37 | Same calls per tick at N = 12 and N = 48, at most 4 | PM-3 | r1 R1-BND-REC1 mark 2, 2261–2263 | research verified |
| TE38 | At most 8 calls for two instances | PM-3 | draft spike item 1 (4 per instance × 2 instances) | arithmetic; proposed (the draft's) |
| TE39 | The glue's allocator calls counted for "the same at both N", reported beside; "at most 4" applied to control-API calls | PM-3; NOTE | none (R1 does not say whether glue calls count) | proposed (test-engineer reading) |
| TE40 | 1,800 ticks (one minute) per N and mode | PM-3 | 60 s × 30 = 1,800 | arithmetic; proposed (test-engineer) |
| TE41 | Bite: one call per soldier per tick | PM-3 | none | proposed (test-engineer) |
| TE42 | Page frame time p95 with the sim no worse than with no sim by more than 1.0 ms at N = 12, 24, 36, 48 on ML | PM-4 | r1 R1-BND-REC1 mark 1, 2259–2261 | research verified |
| TE43 | The same at N = 48 on BX-P | PM-4 | none | proposed (test-engineer: the Worker's worst allowed load) |
| TE44 | Both statistics gate, each in its own spike: this p95 difference here, ADR 0003's p99 (PM-0003-20) in the bench | PM-4; SE | map LE-B and N12 (line 734); 0003 PM-0003-20 | settled (the map gives test-engineer the choice); cross-draft |
| TE45 | The page's cost of the sim at most 1.0 ms per frame | PM-4 | GL 214–218; r3 R3-BUDGET-REC2 row, 1127 | settled |
| TE46 | 10 interleaved pairs; the median of paired p95 differences (mean of the 5th and 6th smallest) | PM-4 | 0003 PM-0003-37 (the same design) | proposed (test-engineer: the mark is about the run-to-run spread) |
| TE47 | 3 % of a 28 ms median is 0.84 ms | PM-4 | r3 1124 (median 28 ms on ML at Low); r3 1192–1193 (3 %); 0.03 × 28 = 0.84 | arithmetic |
| TE48 | The no-sim baseline: recorded blocks replayed on the page, 1,200 blocks, 40 s | PM-4; NOTE | 40 s × 30 = 1,200 | arithmetic; proposed (test-engineer: the baseline must draw the same scene) |
| TE49 | Mode U; rule B in the block handler | PM-4 | TE23; TE31 | proposed (test-engineer) |
| TE50 | The match runs in the four browsers and Brave | PM-5 | r1 R1-BND-REC1 mark 3, 2263; draft spike item 1 (Brave) | research verified; proposed (the draft's) |
| TE51 | "Runs" read as 0 console errors, 0 page errors, 18,000 ± 1 ticks, one block per tick | PM-5 | none; ±1 for the start and stop ticks | proposed (test-engineer reading) |
| TE52 | Bite: a `console.error` 2 s after load | PM-5 | r5 R5-GATE-REC3 G12 row, 1723 | proposed (R5's) |
| TE53 | Per tick at most 1 copy in the Worker, 0 copies and 0 per-entity calls on the page | PM-6 | r1 R1-BND-REC2 mark 1, 2276–2277 | research verified |
| TE54 | Exactly 1 message from the Worker per tick | PM-6 | draft part 5 (one block per tick) | proposed (the draft's design, counted) |
| TE55 | The detached-buffer check and the view-identity check | PM-6 | r1 R1-BND-F15 (a transfer is the zero-copy exception, cited by R1-BND-REC3, 2290–2291) | research verified; proposed (test-engineer method) |
| TE56 | Bite: the block posted without its transfer list | PM-6 | none | proposed (test-engineer) |
| TE57 | 0 buffer and typed-array allocations per frame after warm-up | PM-7 | r1 R1-BND-REC2 mark 2, 2277–2278 | research verified |
| TE58 | Warm-up = the first 300 frames | PM-7 | r3 R3-MEASURE-REC1, 1183–1184 | proposed (R3's) |
| TE59 | Boundary-caused growth under 0.1 MB/s, read as the allocation rate of the boundary's page code (with the message event) | PM-7 | r1 R1-BND-REC2 mark 2, 2277–2278 ("allocation sampling") | research verified (the number); proposed (test-engineer reading) |
| TE60 | 0.1 MB/s × 600 s = 60 MB | PM-7 | arithmetic | arithmetic |
| TE61 | The sampling method that includes objects collected by minor and major collections | PM-7 | `tools/perfbots.js` lines 9–10 (legacy TL15's method) | inventory |
| TE62 | Chrome only; Firefox and Safari not measured by this method | PM-7 | the inspector protocol is Chrome's (the method of TE61) | proposed (test-engineer) |
| TE63 | Garbage collection and heap through the Chrome DevTools Protocol | PM-7 | 0003 PM-0003-24 ("proposed tool; not researched") | proposed (test-engineer) |
| TE64 | 3 matches; bite: a 64-byte typed array per frame | PM-7 | 0003 PM-0003-29 (the same bite) | proposed (test-engineer) |
| TE65 | Block latency at most one display frame (16.7 ms at 60 Hz) at p95 | PM-8 | r1 R1-BND-REC2 mark 3, 2278–2280 | research verified |
| TE66 | The 17,100th of 18,000 | PM-8 | TE19 | arithmetic |
| TE67 | Endpoints: the step returns (before the copy) to the views bound | PM-8 | none | proposed (test-engineer reading of "finishing tick T" and "readable") |
| TE68 | The page loaded to 16 ms per frame (ADR 0003's main-thread row on ML at Low) | PM-8; PM-13 | r3 R3-BUDGET-REC2 row, 1125 | proposed (R3's number); proposed (test-engineer: on a light page the mark could not fail) |
| TE69 | Mode P on a 60 Hz panel; 3 matches per browser; Safari reported | PM-8 | none | proposed (test-engineer: the threshold is on the minimum machine) |
| TE70 | Bite: tick T's block posted after stepping T + 1 | PM-8 | none | proposed (test-engineer) |
| TE71 | 0 lost and 0 reordered events over the 10-minute match at N = 48 | PM-9 | r1 R1-BND-REC2 mark 4, 2280–2281; MODES-3 (a) | research verified; settled |
| TE72 | A per-match event sequence number in the spike's event section | PM-9 | none | proposed (test-engineer) |
| TE73 | Bite: a block returned to the pool unread | PM-9 | none | proposed (test-engineer) |
| TE74 | Bytes per tick and copy time at N = 48, per entity and per field | PM-10 | r1 R1-BND-OQ6, 2736–2739 | research verified |
| TE75 | The field-by-field writer's time beside the copy | PM-10 | r1 R1-RR-OQ6, 2951–2954; draft part 5 (259–262), spike item 2 | research verified; proposed (the draft's) |
| TE76 | Critic rank 37 is decided here | PM-10 | map line 698 (OffscreenCanvas, or a per-entity or per-field layout) and line 942 | settled |
| TE77 | B3 weighed only if it saves at least 0.5 ms of page time per frame or one frame of latency at p95 on the minimum machine | PM-11 | r1 R1-BND-REC2 mark 5, 2282–2284 | research verified |
| TE78 | "Page time" read as the block handler and view binding, at p95 | PM-11 | none | proposed (test-engineer reading) |
| TE79 | B2 and B3 both on the isolation-flag build | PM-11 | r1 R1-BND-REC5, 2309–2311 (B3 needs isolation); r3 R3-MEASURE-F7 (timers change) | research verified; proposed (test-engineer: one footing) |
| TE80 | 10 interleaved pairs | PM-11 | as TE46 | proposed (test-engineer) |
| TE81 | 0 lost and 0 doubled presses over 10,000 scripted presses | PM-12 | r1 R1-BND-REC3 mark 1, 2292–2294 | research verified |
| TE82 | Releases counted too; some presses shorter than one tick | PM-12 | r1 R1-BND-REC3, 2287–2290 (edge counters) | proposed (test-engineer: the edge counters exist for short presses) |
| TE83 | The shim: delays uniform from 0 to 100 ms, 10 % sent twice | PM-12 | none (R1 gives no amounts) | proposed (test-engineer: a spread wider than one 33.3 ms tick reorders most neighbours) |
| TE84 | The network's own press mark is ADR 0001's (R2-SYNC-REC2 at P3) | PM-12 | 0001 spike section item 2 | cross-draft |
| TE85 | Bite: only the held-button bitmask read | PM-12 | none | proposed (test-engineer) |
| TE86 | Key event to the tick that applies it, at most one tick plus one frame at p95 | PM-13 | r1 R1-BND-REC3 mark 2, 2294–2295 | research verified |
| TE87 | 33.3 + 16.7 = 50.0 ms | PM-13 | 1000 / 30 = 33.3 (TICK-30); 1000 / 60 = 16.7 | arithmetic |
| TE88 | The 9,500th of 10,000 | PM-13 | ⌈0.95 × 10,000⌉ = 9,500 | arithmetic |
| TE89 | Key events through Playwright, headed; Safari through safaridriver | PM-13 | r5 R5-TEST-F8 (Playwright cannot drive branded Safari; in the R5 verifier table); r5 R5-TEST-REC2, 1416–1419 (safaridriver) | research verified; proposed (test-engineer method) |
| TE90 | Bite: commands posted only from the frame callback plus one extra tick of buffering | PM-13 | none | proposed (test-engineer) |
| TE91 | Cursor ray and view time in the same message, 10,000 of 10,000 | PM-14 | draft part 5 (265–269) and spike item 3 | proposed (the draft's) |
| TE92 | Zero diff on regeneration | PM-15 | r1 R1-BND-REC4 mark 1, 2304–2305 | research verified |
| TE93 | A rename without regeneration and a hand edit each fail | PM-15 | r5 R5-LAYOUT-REC3 mark, 1203–1204 | proposed (R5's) |
| TE94 | Control: a rename with regeneration passes | PM-15; PM-43 | r5 R5-GATE-REC3 G4 row, 1716 | proposed (R5's) |
| TE95 | A layout change without regeneration fails at build or startup, proven by a test | PM-16 | r1 R1-BND-REC4 mark 2, 2305–2306 | research verified |
| TE96 | The test: one field added, the old generated layout | PM-16 | none | proposed (test-engineer) |
| TE97 | `tsc --strict` passes on the generated output | PM-17 | r1 R1-BND-REC4 mark 3, 2306 | research verified |
| TE98 | With `erasableSyntaxOnly`; bite: a generated `enum` | PM-17 | r5 R5-TSLINT-OQ3, 1770–1772 | research, source to check |
| TE99 | CI fails on a CLI/library mismatch | PM-18 | r1 R1-BND-REC4 mark 4, 2306–2307; r5 R5-CI-REC3 mark 1, 1544–1546 | research verified; proposed (R5's) |
| TE100 | The CLI one patch off | PM-18 | r5 R5-GATE-REC3 G4 row, 1716 | proposed (R5's) |
| TE101 | Two generator runs byte-identical | PM-19 | r5 R5-LAYOUT-REC3 mark, 1203–1204 | proposed (R5's) |
| TE102 | 20 runs give 20 identical sets | PM-19 | r5 R5-LAYOUT-OQ4, 1759–1760 (parallel threads); r5 R5-TEST-REC6 mark, 1503 (20 identical verdicts) | proposed (test-engineer: a race shows only sometimes) |
| TE103 | `crossOriginIsolated` true with the flag, false without, Chromium, Firefox, WebKit, `vite` and `vite preview` | PM-20 | r5 R5-LAYOUT-REC5 mark, 1229–1231 | proposed (R5's) |
| TE104 | 12 cases | PM-20 | 3 engines × 2 servers × 2 flag settings = 12 | arithmetic |
| TE105 | Read in the Worker too; bite: COOP without COEP | PM-20 | none | proposed (test-engineer) |
| TE106 | If isolation is adopted: true on a preview link in four browsers; 0 COEP violations in a report-only full match | PM-21 | r1 R1-BND-REC5 mark, 2321–2324 | research verified |
| TE107 | `same-origin-allow-popups` recorded in the four browsers | PM-21 | r1 R1-BND-OQ9, 2750–2755; draft OQ-0002-4 ("What decides it") | research verified; proposed (the draft's) |
| TE108 | "A full match" read as the 10-minute match; a `ReportingObserver` collects violations | PM-21 | none | proposed (test-engineer; the tool not researched) |
| TE109 | The page handles 0 network messages | PM-22 | r1 R1-BND-REC6 mark 1, 2334–2335 | research verified |
| TE110 | A 10-minute online session at P2; CI at P0; bite: the page opens the WebSocket itself | PM-22 | none | proposed (test-engineer) |
| TE111 | A 100 ms busy loop on the page leaves no gap in the Worker's snapshot processing | PM-23 | r1 R1-BND-REC6 mark 2, 2335–2336 | research verified |
| TE112 | One loop every 3 s, 100 loops; "no gap" = no interval over two snapshot intervals | PM-23 | none | proposed (test-engineer reading: a path through the page shows the whole 100 ms) |
| TE113 | 2 × 33.3 = 66.7 ms at 30 Hz snapshots | PM-23 | TICK-30 (every tick a snapshot tick: map N21) | arithmetic |
| TE114 | Gated at P0, reported at P2 | PM-23 | r2, 1989 | proposed (test-engineer: P0 isolates the claim from network jitter) |
| TE115 | The same Wasm file and page code, the mode flag only | PM-24 | r1 R1-BND-REC6 mark 3, 2336–2337 | research verified |
| TE116 | Files compared by sha256 from the network log | PM-24 | none | proposed (test-engineer) |
| TE117 | OQ-0002-3's measures: calls, the Worker's time per tick, memory | PM-25 | draft OQ-0002-3 ("What decides it") | proposed (the draft's) |
| TE118 | (b) gated: a client-endpoint trap leaves the authority's tick advancing | PM-25 | draft OQ-0002-3 option (b) ("a client trap leaves the authority running"); 0004 OQ-0004-7 | proposed (test-engineer: the reason (b) exists) |
| TE119 | Jitter p99 at most 4 ms over 10 minutes; smoothness SD at most 5 % of the mean on 60, 120, 144 Hz; after 60 s hidden, local play ran no tick and online is back in step within 0.5 s | PM-26 | r1 R1-TICK-REC5 marks, 2109–2114; 0004 S18 | research verified; cross-draft |
| TE120 | At most 7 ticks in one frame (the research's 15 at 60 Hz) | PM-26 | 0004 part 5 and S18 (⌊0.25 × 30⌋ = 7) | arithmetic; proposed (ADR 0004's) |
| TE121 | ADR 0004's pass marks own the S18 numbers; where they differ, ADR 0004's stand | PM-26; FAIL | map part c (the owner states a number; others cite) | settled |
| TE122 | Hidden "more than 5 minutes" read as 6 | PM-26; SC 4 | r1 R1-BND-OQ2, 2723–2726; R1-XU-OQ12, 2903–2906 (Chrome's intensive throttling after 5 minutes) | research verified (the question); proposed (test-engineer: 6, so a minute of throttling) |
| TE123 | "Back in step" read as the predicted tick's lead within ±1 tick of its lead before hiding | PM-26 | none | proposed (test-engineer reading, unless ADR 0004's pass marks define it) |
| TE124 | 120 and 144 Hz displays attached to ML; none named in the draft | PM-26; SE | draft spike item 7 (no display named) | proposed (test-engineer) |
| TE125 | The 17,820th of 18,000; 3 hidden trials per duration, mode and browser | PM-26 | TE19; none | arithmetic; proposed (test-engineer) |
| TE126 | Bites: a timer with no accumulator; a Worker that simulates the hidden gap | PM-26 | r1 R1-TICK-REC5, 2095–2103 (the accumulator; the gap is not simulated) | proposed (test-engineer) |
| TE127 | W3 on the isolated build only if W2 fails the jitter mark | PM-27 | r1 R1-TICK-REC5, 2097; R1-XU-OQ4, 2861–2865 | research verified |
| TE128 | p99 sim step time at most 13.3 ms at 30 Hz in Wasm in the Worker while the page holds 30+ fps on Low | PM-28 | r1 R1-TICK-REC2 P2, 2054–2057; 0004 S2 | research verified; cross-draft |
| TE129 | 40 % of 33.3 ms = 13.3 ms | PM-28 | GL 214–218 | arithmetic |
| TE130 | "30+ fps for the whole run" read as every 1-second window holding at least 30 presented frames | PM-28 | VISION_V2 §4 (30+ fps) | proposed (test-engineer reading) |
| TE131 | N for the pass from R1-TICK-OQ7; the scale steps reported meanwhile | PM-28 | 0004 S2 | cross-draft |
| TE132 | Size raw and gzip, with and without wasm-opt, "s" and "z" | PM-29 | r1 R1-RR-OQ5, 2946–2950; draft part 4 (233–235) | research verified |
| TE133 | The wasm-opt version recorded | PM-29 | r5 R5-CI-OQ4, 1833–1834 | research, source to check |
| TE134 | Every served file under 25 MiB | PM-29 | r6 R6-HOST-REC1 mark, 642; map N37 | research, source to check |
| TE135 | 25 × 1,048,576 = 26,214,400 bytes | PM-29 | arithmetic | arithmetic |
| TE136 | The first value becomes the budget file's first limit | PM-29 | r5 R5-GATE-REC4, 1737–1738 | proposed (R5's) |
| TE137 | gzip at level 9 | PM-29 | none | proposed (test-engineer) |
| TE138 | Type errors and casts against three.js's types counted | PM-30 | r5 R5-TSLINT-OQ4, 1773–1775 | research, source to check |
| TE139 | `tsc -b` with `skipLibCheck` on and off, median of 3 | PM-30 | r5 R5-TSLINT-OQ6, 1779–1780 | research, source to check; proposed (test-engineer: 3 runs) |
| TE140 | Whether Playwright's clock reaches the Worker | PM-30 | r5 R5-TEST-OQ1, 1802–1803 | research, source to check |
| TE141 | R1-DATA-OQ8 belongs to this spike | PM-31; SE | map part g, line 942; 0005 lines 520 and 795 | settled (the map's assignment); cross-draft |
| TE142 | The page relay (a Vite plugin pushes files to the page, which posts them to the Worker) | PM-31 | r1 R1-DATA-REC5, 2545–2547 | research verified |
| TE143 | Within 1 s of the save, 20 of 20 saves | PM-31 | 0005 CD-7 (R1-DATA-REC5 mark 1, r1 2559) | cross-draft; proposed (test-engineer: 20 saves) |
| TE144 | The built editor from `file://` in five browsers, recorded | PM-32 | draft OQ-0002-1 and spike item 9; r1 R1-DATA-OQ11, 2817–2823 | proposed (the draft's) |
| TE145 | A Wasm trap reaches Sentry with Rust names, 4 of 4 | PM-33 | r6 R6-OBS-REC3 mark, 765–766 | proposed (R6's) |
| TE146 | Only if the EU Sentry organisation exists | PM-33 | draft spike item 10 | proposed (the draft's); cross-draft |
| TE147 | Mustafa's Brave run with a checklist | PM-34 | plan rule 12; draft 932–933 | settled |

## The foundation's rows

| # | the claim or number (short) | where | citation (file, id, lines) | tier |
|---|---|---|---|---|
| TE148 | Today's gate passes on `legacy/`; the four Phase 0 mutation proofs fail on the right checks | PM-35 | r5 R5-LAYOUT-REC6 mark, 1243–1245 | proposed (R5's) |
| TE149 | The tuning dump reproduced byte for byte | PM-36 | r5 R5-LAYOUT-REC6 mark, 1245 | proposed (R5's) |
| TE150 | 2 runs: the dump "was byte-identical over two runs" | PM-36 | r5 R5-TEST-REC3, 1444–1445 | research, source to check |
| TE151 | Fresh clone: tests pass with no network after install | PM-37 | r5 R5-LAYOUT-REC1 mark, 1166–1168 | proposed (R5's) |
| TE152 | A fresh runner builds native and wasm32 from `rust-toolchain.toml` alone | PM-37 | r5 R5-CI-REC2 mark 1, 1532–1533 | proposed (R5's) |
| TE153 | The network cut off by a namespace with no interfaces | PM-37 | none | proposed (test-engineer; not researched) |
| TE154 | Versions equal the pins in every job's log | PM-38 | r5 R5-CI-REC2 mark 3, 1534–1535; R5-RUSTLINT-REC6 mark, 1380; R5-LAYOUT-REC1 mark, 1168 | proposed (R5's) |
| TE155 | Node, TypeScript and the wasm-bindgen CLI checked too | PM-38 | draft part 2 (151–163: all pinned) | proposed (test-engineer: the draft pins them) |
| TE156 | The same lint result on two runs a week apart | PM-38 | r5 R5-RUSTLINT-REC6 mark, 1380–1381 | proposed (R5's) |
| TE157 | Deleting the toolchain file fails G0 with exit 2 | PM-39 | r5 R5-LAYOUT-REC1 mark, 1168–1169; G0 row, 1712 | proposed (R5's) |
| TE158 | `Cargo.toml` without `Cargo.lock` fails at the first cargo step | PM-39 | r5 R5-CI-REC2 mark 2, 1533–1534 | proposed (R5's) |
| TE159 | A missing Playwright browser gives exit 2 with the setup command | PM-39 | r5 R5-GATE-REC2 mark, 1704–1705 | proposed (R5's) |
| TE160 | Control: the toolchain bumped consistently | PM-39 | r5 G0 row, 1712 | proposed (R5's) |
| TE161 | G1 mutations (misformatted `.rs`, CRLF, misformatted `.ts`) and control | PM-40 | r5 G1 row, 1713 | proposed (R5's) |
| TE162 | After `prettier --write`, ESLint reports 0 problems | PM-40 | r5 R5-TSLINT-REC3 mark, 1290–1291 | proposed (R5's) |
| TE163 | The formatted tree byte-identical on Windows and Linux | PM-40 | r5 R5-RUSTLINT-REC5 mark, 1372–1373 | proposed (R5's) |
| TE164 | G2 mutations and controls (the hash-map ban, the clock, `sin`, panics, correctness, style, `unsafe`, the lint opt-in) | PM-41 | r5 G2 row, 1714; R5-RUSTLINT-REC1 mark, 1318–1320; REC2 mark, 1336–1339; REC3 mark, 1351–1352 | proposed (R5's) |
| TE165 | The build-flag mutations (`+fma`, `target-cpu=native`, `relaxed-simd`) | PM-41 | draft part 4 (229–231) | proposed (the draft's) |
| TE166 | The text-scan fallback if clippy cannot name the float methods | PM-41 | draft part 7 (346–347); r5 R5-RUSTLINT-OQ1 | proposed (the draft's) |
| TE167 | G3 mutations: one per strict flag (seven) and per lint rule (four); control | PM-42 | r5 R5-TSLINT-REC1 mark, 1261–1263; REC2 mark, 1279–1281; G3 row, 1715 | proposed (R5's) |
| TE168 | `sim-worker` and `net` never import rendering or DOM; nothing imports `legacy/` or `spikes/` | PM-42 | draft part 3 (197–198); r5 R5-TSLINT-REC2, 1268–1269 | proposed (the draft's; R5's) |
| TE169 | G4 mutations and control | PM-43 | r5 G4 row, 1716 | proposed (R5's) |
| TE170 | G5: transport or `tokio`/`quinn` in `sim`; the feature check | PM-44 | r5 R5-LAYOUT-REC2 mark, 1189–1192; REC4 mark, 1216–1218; G5 row, 1717 | proposed (R5's) |
| TE171 | G5: `tokio` in `room`; `navbake` or `server` reachable from `sim-wasm` | PM-44 | draft part 3 (192–195) | proposed (the draft's) |
| TE172 | G5: a wildcard version; a duplicate outside the skip list | PM-44 | r5 R5-GATE-REC1 G5 row, 1663 | proposed (R5's) |
| TE173 | G7 mutations and control; raising a budget only through the file | PM-45 | r5 G7 row, 1718; R5-GATE-REC4 mark, 1743–1744 | proposed (R5's) |
| TE174 | G8: a dynamic `import()` of the editor; control; the player page loads no editor file | PM-46 | r5 R5-LAYOUT-REC2, 1191–1192; R5-LAYOUT-REC5 mark, 1231; G8 row, 1719 | proposed (R5's) |
| TE175 | G9 mutations; one planted defect per layer, the benchmark layer excepted | PM-47 | r5 G9 row, 1720; R5-TEST-REC1 mark, 1407–1409 | proposed (R5's) |
| TE176 | R1-DET-REC6's three divergences reported at the first tick, with a field-level diff | PM-48 | r5 R5-TEST-REC2 mark, 1427–1428; G10 row, 1721 | proposed (R5's) |
| TE177 | G10 under 3 minutes on the CI runner | PM-48 | r5 R5-TEST-REC2 mark, 1428–1429 | proposed (R5's) |
| TE178 | Times over 5 runs | PM-48; PM-49; PM-54 | r5 R5-CI-REC5 mark 2, 1580 (5 runs) | proposed (test-engineer: R5's run count) |
| TE179 | L1: the same seed and profile give an identical fate log and hashes | PM-49 | r5 R5-TEST-REC4 mark, 1469–1470 | proposed (R5's) |
| TE180 | Loss, mean delay and jitter within 5 % (relative) | PM-49 | r5 R5-TEST-REC4 mark, 1470–1471 | proposed (R5's) |
| TE181 | 1,000,000 packets per profile instead of 100,000 | PM-49; NOTE | TE182 | proposed (test-engineer) |
| TE182 | √(0.01 × 0.99 ÷ 100,000) = 0.031 points; 0.05 ÷ 0.031 = 1.6 standard errors, missed about 1 seed in 9 (two-sided 11 %); √(0.01 × 0.99 ÷ 1,000,000) = 0.010 points, 5 standard errors | NOTE | binomial standard error; P1 = 1 % loss (r2, 1989) | arithmetic |
| TE183 | G11 under 2 minutes in total | PM-49 | r5 R5-TEST-REC4 mark, 1472–1473 | proposed (R5's) |
| TE184 | Jitter read against L1's own documented distribution | PM-49 | none (R2 names no distribution) | proposed (test-engineer) |
| TE185 | G11 mutations and control | PM-50 | r5 G11 row, 1722 | proposed (R5's) |
| TE186 | G12 mutations and control | PM-51 | r5 G12 row, 1723; R5-GATE-REC1 G12, 1670 | proposed (R5's) |
| TE187 | G12 runs once with `forceWebGL`; the backend on every shot | PM-51 | draft part 9 (475–478) | proposed (the draft's) |
| TE188 | `--prove`: every mutation fails exactly its checks; at least one mutation and one control per check id | PM-52 | r5 R5-GATE-REC3, 1706–1709; mark, 1731 | proposed (R5's) |
| TE189 | One `--prove` run under 30 minutes, a plain limit | PM-52 | r5 R5-GATE-REC3 mark, 1731–1732; GL 146–147 | proposed (R5's); settled |
| TE190 | A planted `u16` overflow panics in four builds; `wrapping_add` agrees | PM-53 | r5 R5-RUSTLINT-REC4 mark, 1362–1364 | proposed (R5's) |
| TE191 | No separate limit for the overflow checks' cost (the draft sets none; it sits inside the sim step share) | PM-53 | draft part 4 (223–236); r5 1364 ("the ADR sets the limit") | proposed (test-engineer reading) |
| TE192 | `tsc -b` under 30 s warm | PM-54 | r5 R5-TSLINT-REC1 mark, 1263–1264 | proposed (R5's) |
| TE193 | Typed lint under 60 s warm | PM-54 | r5 R5-TSLINT-REC2 mark, 1281–1282 | proposed (R5's) |
| TE194 | Clippy under 30 s warm, under 3 minutes cold | PM-54 | r5 R5-RUSTLINT-REC1 mark, 1319–1320 | proposed (R5's) |
| TE195 | `--quick` under 60 s; the full gate under 10 minutes warm, Linux and Windows, 5 runs | PM-55 | r5 R5-GATE-REC2, 1691–1692; mark, 1703–1704 | proposed (R5's) |
| TE196 | Linux on the session's machine, Windows on MP | PM-55 | draft Consequences (Mustafa's Windows machine runs the gate, 749–750) | proposed (test-engineer: R5's "recent desktop") |
| TE197 | At most 10 minutes p50 over 5 warm runs, at most 20 minutes cold, per job and in total | PM-56 | r5 R5-CI-REC5 mark 2, 1579–1581; map N40 | proposed (R5's) |
| TE198 | The 3rd of 5 | PM-56 | ⌈0.5 × 5⌉ = 3 | arithmetic |
| TE199 | Cold read as each of 3 runs | PM-56 | none (R5 gives no cold count) | proposed (test-engineer) |
| TE200 | A TypeScript-only pull request passes; a failing and a cancelled job each turn the check red | PM-57 | r5 R5-CI-REC5 marks 1 and 5, 1579, 1582–1583 | proposed (R5's) |
| TE201 | A skipped job turns it red | PM-57 | draft part 9 (458–460); r5 R5-CI-REC5, 1557–1559 | proposed (the draft's; R5's) |
| TE202 | Removing a check from the workflow fails the aggregate job | PM-57 | r5 R5-GATE-REC2 mark, 1704 | proposed (R5's) |
| TE203 | A fork pull request runs every job with no secret; the deploy job skipped | PM-58 | r5 R5-CI-REC5 mark 4, 1582; R5-PREV-REC1 mark 2, 1601–1602 | proposed (R5's) |
| TE204 | A test account for the fork | PM-58; SE | none | proposed (test-engineer) |
| TE205 | No dependency recompiled on a warm cache; cache at or below 8 GB; warm against cold over 3 runs | PM-59 | r5 R5-CI-REC1 marks 1–3, 1518–1521 | proposed (R5's) |
| TE206 | A warm `npm ci` faster than a cold one; the Playwright job against legacy's 57–67 s | PM-59 | r5 R5-CI-REC4 marks, 1553–1555 | proposed (R5's) |
| TE207 | The CLI install under 30 s over 3 runs, exactly the lock's version | PM-60 | r5 R5-CI-REC3 mark 2, 1545–1546; REC3, 1536–1539 | proposed (R5's) |
| TE208 | 20 runs give 20 identical verdicts; the planted timing test; the retry-only test | PM-61 | r5 R5-TEST-REC6 mark, 1503–1505 | proposed (R5's) |
| TE209 | Every layer's clean run passes 20 times in a row | PM-61 | r5 R5-TEST-REC1 mark, 1409–1410 | proposed (R5's) |
| TE210 | The hook within 10 s at p95 over 20 edits, Windows and Linux; any working folder | PM-62 | r5 R5-TSLINT-REC4 mark, 1299–1301 | proposed (R5's) |
| TE211 | The 19th of 20 | PM-62 | ⌈0.95 × 20⌉ = 19 | arithmetic |
| TE212 | The hook lands in a kit change-set | PM-62 | draft part 7 (352–354) | proposed (the draft's) |
| TE213 | Generated types hold in CI | PM-63 | draft migration plan 2.4 (968–969) | proposed (the draft's) |
| TE214 | One command and one block per tick; 0 page Wasm calls; the empty scene names its backend; ADR 0001's and ADR 0004's milestones | PM-64 | draft migration plan 2.5 (970–974) | proposed (the draft's); cross-draft |
| TE215 | A 1-minute match, 1,800 ticks | PM-64 | 60 × 30 = 1,800 | arithmetic; proposed (test-engineer: G12's time budget) |
| TE216 | The link within 5 minutes after the build job, median of 5 | PM-65 | r5 R5-PREV-REC1 mark 1, 1600–1601; map N39 | proposed (R5's) |
| TE217 | The deploy job runs no pull-request code; the test pull request | PM-66 | r5 R5-PREV-REC1 mark 3, 1602–1603 | proposed (R5's) |
| TE218 | The marker in the deploy job's log | PM-66 | none | proposed (test-engineer) |
| TE219 | The header check passes and fails when a header is removed; no COOP or COEP without the flag | PM-67 | r5 R5-PREV-REC4 marks 1 and 3, 1636–1640 | proposed (R5's) |
| TE220 | `crossOriginIsolated` on previews in Chrome, Edge, Firefox, Safari and Brave | PM-68 | r5 R5-PREV-REC4 mark 2, 1638–1640 | proposed (R5's) |
| TE221 | The foundation ends with an empty scene, CI green and a preview link | PM-69; FC 5 | GLOSSARY §8, the foundation (map J1, 1068–1078) | settled |
| TE222 | The link opens in Brave without a login | PM-69; FC 3 | r5 R5-PREV-REC5 mark, 1648–1650; PREVIEW-WALL (a) (the wall is for previews that reach the staging game server) | proposed (R5's); settled |

## Later milestones

| # | the claim or number (short) | where | citation (file, id, lines) | tier |
|---|---|---|---|---|
| TE223 | The boundary marks hold on slice 1's real sim at N = 12 | PM-70 | draft migration plan step 3 (981–985) | proposed (the draft's) |
| TE224 | Goldens fail when moved; CS6's 20,000 of 20,000; a regenerated fixture byte-identical; the list per row | PM-71 | r5 R5-TEST-REC3 mark, 1451–1453 | proposed (R5's) |
| TE225 | G13's pair (an unreachable spawn fails; a valid map passes) | PM-72 | draft part 9 (446–448) | proposed (the draft's) |
| TE226 | A 30-minute bot room at N = 48 at P2: 0 panics, 0 hangs, 0 desyncs, slope within 1 MB per 10 minutes; a planted leak flagged | PM-72 | r5 R5-TEST-REC5 mark, 1487–1489 | proposed (R5's) |
| TE227 | Server tick time against the budget counts only from one vCPU of the reference server, never CI | PM-72; PM-76 | GL 167–172 (the budget on one vCPU of the reference server); GL 244–250 | settled; proposed (test-engineer: CI is not a hardware result) |
| TE228 | The server tick budget: 16.7 ms at 30 Hz | PM-72 | GL 168–169 | settled |
| TE229 | G12 on both backends; a multiplayer preview behind the access wall | PM-73 | draft migration plan step 5 (989–992); PREVIEW-WALL (a) | proposed (the draft's); settled |
| TE230 | The round-trip test covers every message kind and section | PM-74 | draft migration plan step 6 (993–995); r5 R5-TEST-REC1 boundary row, 1391 | proposed (the draft's); proposed (R5's) |
| TE231 | G8 finds no editor chunk; the editor's checks equal G13 | PM-75 | draft migration plan step 7 (996–998) | proposed (the draft's) |
| TE232 | Every mode of the test-mode list: 1v1, 4v4, 2v2v2, 6v6v6, 12v12v12, 16v16v16, 24v24, N = 64 headroom | PM-76 | GL 375–387 (MODES-3 (a)) | settled |
| TE233 | 30 minutes each over L1 at P2; one L2 run with 4 real browser clients; a 3-hour soak before each playtest | PM-76 | r5 R5-TEST-REC5, 1478–1486 | proposed (R5's) |

## Notes, failure rule, no-mark lines, checklists, spike entries

| # | the claim or number (short) | where | citation (file, id, lines) | tier |
|---|---|---|---|---|
| TE234 | The draft's online per-tick calls are 5 or 6 unless batched | NOTE | draft part 5 (265–272: the command; 286–289: datagrams in and out in one call each) and the control API (270–272) | proposed (test-engineer reading) |
| TE235 | wasm-bindgen's glue makes an allocator call when bytes are passed in | NOTE | none read in R1 or R5 | proposed (test-engineer; not researched, a fact to confirm in the spike) |
| TE236 | A failing run is never re-run until green; its data kept, its cause written | FAIL | r5 R5-TEST-REC6, 1490–1498 | proposed (R5's) |
| TE237 | A failed mark goes back to the draft by its Rollback routes, then to the advisor and to Mustafa and the Advisor | FAIL | draft Rollback (1022–1039); plan steps 9 and 13 | settled |
| TE238 | A mark's number changes only in its owning draft | FAIL | map part c (owners) | settled |
| TE239 | A failing foundation mark blocks its change-set; a budget raise is a reviewed file change | FAIL | CLAUDE.md, the change-set protocol, step 4; r5 R5-GATE-REC4, 1734–1738 | settled; proposed (R5's) |
| TE240 | OQ-0002-2: the restructure's marks hold in either layout | NOM | draft OQ-0002-2 | proposed (test-engineer reading) |
| TE241 | The latest-dependencies job never blocks; TypeScript 7 its own change-set; Dependabot's SHA pins tried on one action | NOM | draft part 2 (157–158, 164), part 9 (468–469) | proposed (the draft's) |
| TE242 | Crate entry by slice holds by review; the lobby crate waits for ADR 0006 | NOM | draft part 3 (174–175, 189–190) | proposed (the draft's) |
| TE243 | `panic = "unwind"` per connection is ADR 0001's mark; the restart after a trap is ADR 0004's S20 | NOM | 0001 spike section item 2 (R2-ROOM-REC2 mark 6); 0004 S20 | cross-draft |
| TE244 | The interpolation buffer (critic rank 4) has no mark of its own here | NOM | map line 676; draft part 5 (254–256) | settled |
| TE245 | `textContent` holds by review until the security baseline | NOM | draft part 7 (333–334) | proposed (the draft's) |
| TE246 | The tick bench never gates; fuzz marks are ADR 0001's and ADR 0005's; real Safari waits for OQ-0002-5; the jump is not golden | NOM | draft part 8 (379, 415); r5 R5-TEST-REC1, 1400; draft OQ-0002-5, OQ-0002-6 | proposed (the draft's; R5's) |
| TE247 | WebGPU in CI is ADR 0003's counts; no arm64 leg; Sentry upload and advisories wait | NOM | draft part 9 (470–486); 0003 PM-0003-36 | proposed (the draft's); cross-draft |
| TE248 | The delivery token waits for the baseline (item B9); PREV-DELETE is ADR 0006's | NOM | draft part 10 (515–517); map part g, line 947 | settled |
| TE249 | OQ-0002-7 is a reading | NOM | draft OQ-0002-7 | proposed (the draft's) |
| TE250 | No 60 Hz mark here | NOM | TICK-30; GLOSSARY §8, the 60 Hz margin (map J1) | settled |
| TE251 | No CI time is a hardware result | NOM | r3 R3-MEASURE-F8; draft part 8 (416–417) | research, source to check; proposed (the draft's) |
| TE252 | His PC is far above the minimum laptop: never the budget | SC | r3 R3-BUDGET-REC1, 1102–1103 | proposed (R3's) |
| TE253 | About 18,000 ticks after 10 minutes | SC 1 | TE14 | arithmetic |
| TE254 | Even motion of a constant-speed soldier | SC 2 | r1 R1-TICK-REC5, 2111–2112 | research verified (the mark); proposed (test-engineer: the check) |
| TE255 | 1 minute and 6 minutes away | SC 3–4 | TE119; TE122 | research verified; proposed (test-engineer) |
| TE256 | Online back within about half a second, no burst | SC 5 | r1 R1-TICK-REC5, 2113–2114 | research verified |
| TE257 | The editor from `file://`; EDITOR_V2 §5's rule is his call later | SC 6 | draft OQ-0002-1; EDITOR_V2 §5 | design doc; proposed (the draft's) |
| TE258 | Legacy by double-click after the move | FC 1 | draft migration plan 2.1 (953) | proposed (the draft's) |
| TE259 | He names the required check in branch protection | FC 2 | draft migration plan 2.3 (966–967) | proposed (the draft's) |
| TE260 | The preview link: no login; the empty scene names its backend | FC 3 | draft migration plan 2.6 (978); 2.5 (972–973) | proposed (the draft's) |
| TE261 | Previews with the flag on and off read isolated yes and no | FC 4 | r5 R5-PREV-REC4 mark 2, 1638–1640 | proposed (R5's) |
| TE262 | Spike names and the "not throwaway" foundation entry from the map | SE | map part g, lines 942, 950–954 | settled (the map's assignment) |
| TE263 | The boundary spike runs beside the netcode proof test and shares its harness and sim; before the foundation | SE | map part g, line 942; draft part 12 (543–545) | settled (the map's proposal, for SPIKE_PLAN) |
| TE264 | R1-BND-REC6 sits under the netcode proof test in the map, while the draft's spike item 6 holds its marks; ADR 0001's spike section does not restate them | SE | map line 941; draft 913–917; 0001 lines 915–994 | settled (all three texts read 2026-11-01) |
| TE265 | The draft's spike list has no item for R1-DATA-OQ8 | SE | draft 878–936 | settled (read) |
| TE266 | The decides list (R1-BND-OQ2, OQ3, OQ6, OQ9; R1-XU-OQ3, OQ4; R1-TICK-OQ2; R1-DATA-OQ8; R1-RR-OQ5, OQ6; R5-LAYOUT-OQ4; R5-TSLINT-OQ3, OQ4, OQ6; R5-TEST-OQ1; ranks 4 and 37) | SE | map line 942; draft Appendix A (1041–1150); r1 and r5 lines in TE33–TE140 | settled |
| TE267 | The foundation's research list | SE | map lines 950–954 | settled |
| TE268 | R5-LAYOUT-REC4 and R5-TEST-REC2 are the sim core spike's in the map, R5-TEST-REC4 the netcode proof test's | SE | map lines 941, 943 | settled |
| TE269 | The preview hosting check comes before the preview step | SE | map line 947 | settled (the map's proposal) |
| TE270 | Sizes: both L | SE | none | proposed (test-engineer: the work each entry lists) |
| TE271 | P2's "on Low" is ADR 0003's tier: the plain renderer until ADR 0003's renderer runs at Low, then the run repeats with it | PM-28 | r1 R1-TICK-REC2 P2, 2054–2057 ("30+ fps on Low") | proposed (test-engineer reading) |

## Counts
- Rows: 271 (TE1–TE271), counted by script on this file; every row has five cells, ids in order.
- By first-listed tier (script on this file): research verified 41 · research, source to check 9 · arithmetic 21 ·
  settled 29 · design doc 1 · inventory 1 · cross-draft 6 · proposed 163 (R5's 64, test-engineer's own 62, the
  draft's 28, R3's 8, R6's 1).
