# Appendix to the v2-research report (2026-10-31)

The records the report (`docs/reports/2026-10-31-v2-research.md`) cites, word for word where the plan asks for it, and the
check scripts' outputs. The working notes they come from are in `.checkpoints/v2-research/` at commit 85f89ea.

## 1. Coverage: every brief §4 question (plan condition 12)

Made by `scripts/coverage_table.py` from each topic file's own question table at 85f89ea. "Lead recommendation" is the
heading of the question's first recommendation, shortened; R2's question 6 is answered across its five units (an
index under its Findings); R6's question 5, the security baseline, was deferred by the Advisor on 2026-10-27
(R6-SEC-OQ1; when it is researched is BASELINE-WHEN).

| Topic | # | Brief question | Recommendations | Lead recommendation (its heading, shortened) | Open questions | Verification |
|---|---|---|---|---|---|---|
| R1 | 1 | ECS or plain data (bevy_ecs, hecs, custom) | R1-ECS-REC1–REC4 | Custom plain data (option C): one sim-state struct of typed tables, phases as plain functions called in a fixe… | R1-ECS-OQ1–OQ7 | verified (every recommendation claim) |
| R1 | 2 | a fixed tick rate | R1-TICK-REC1–REC6 | One fixed tick, counted in integers, the same constant on server and client; durations are converted to ticks … | R1-TICK-OQ1–OQ9 | verified (every recommendation claim) |
| R1 | 3 | identical results between Wasm and native | R1-DET-REC1–REC7 | Two levels, and what the netcode really needs. | R1-DET-OQ1–OQ11 | verified (every recommendation claim) |
| R1 | 4 | the Rust ↔ TypeScript boundary; running the simulation in a Web Worker | R1-BND-REC1–REC6 | Run the client simulation in one dedicated module Web Worker, single-threaded, on stable Rust (A2 + E1). | R1-BND-OQ1–OQ9 | verified (every recommendation claim) |
| R1 | 5 | a data-driven ability and effect system | R1-ABL-REC1–REC6 | A reduced GAS subset, as data, over a closed set of Rust operations (option A2). | R1-ABL-OQ1–OQ10 | verified (every recommendation claim) |
| R1 | 6 | data formats and hot reload of tuning | R1-DATA-REC1–REC8 | JSON is the one content format. | R1-DATA-OQ1–OQ12 | verified (every recommendation claim) |
| R2 | 1 | Snapshots, client prediction, interpolation and lag compensation for 48+ players (GLOSSARY) | R2-SYNC-REC1–REC9 | State sync: delta snapshots against each client's last acknowledged snapshot (option 3.1 E). | R2-SYNC-OQ1–OQ15 | verified (every recommendation claim) |
| R2 | 2 | Tick rate and bandwidth budgets | R2-BUD-REC1–REC6 | Rates: snapshots at 30 Hz; simulation tick 60 Hz if R1's tick spike meets its CPU mark, else 30 Hz; inputs sam… | R2-BUD-OQ1–OQ11 | verified (every recommendation claim) |
| R2 | 3 | Interest management for a top-down camera | R2-INT-REC1–REC7 | Relevance region from camera parameters held as data (option A3), with hysteresis. | R2-INT-OQ1–OQ8 | verified (every recommendation claim) |
| R2 | 4 | The WebTransport server stack in Rust plus the WebSocket fallback | R2-TRN-REC1–REC6 | Build the server on quinn 0.11.x. Pick the WebTransport layer by spike between wtransport 0.7.x and web-transp… | R2-TRN-OQ1–OQ9 | verified (every recommendation claim) |
| R2 | 5 | Rooms and private matches | R2-ROOM-REC1–REC8 | An explicit room lifecycle, one fresh simulation per match. | R2-ROOM-OQ1–OQ11 | verified (every recommendation claim) |
| R2 | 6 | Learn from shipped games (Valve's Source networking, Overwatch's GDC talks, Glenn Fiedler's articles, Quake 3) | – | – | all five | verified (every recommendation claim) |
| R3 | 1 | three.js WebGPURenderer with TSL | R3-WGPU-REC1–REC4 | Option A: one WebGPURenderer, every material and post effect in TSL, the WebGL 2 backend as the fallback; no S… | R3-WGPU-OQ1–OQ5 | spot-checked (45 of 77 key claims) |
| R3 | 2 | the WebGL 2 fallback | R3-GL2-REC1–REC3 | Option F-A for the first slice, F-B only by measured need: every technique the engine ships in the first spike… | R3-GL2-OQ1–OQ5 | spot-checked (45 of 77 key claims) |
| R3 | 3 | candidate techniques for a comic look (toon ramps, screen-space outlines, ambient occlusion, emissive bloom) | R3-LOOK-REC1–REC3 | Measure every candidate technique alone, in one look-test scene, before the look brief closes: GPU time per pa… | R3-LOOK-OQ1–OQ6 | spot-checked (45 of 77 key claims) |
| R3 | 4 | characters (skinning, instancing, LOD) | R3-CHAR-REC1–REC4 | Soldiers: one skinned mesh per soldier (C-B) as the baseline that runs on both backends (rigid weights, one bo… | R3-CHAR-OQ1–OQ6 | spot-checked (45 of 77 key claims) |
| R3 | 5 | budgets for the minimum and recommended hardware | R3-BUDGET-REC1–REC3 | Reference machines (proposed, M-A with M-B as the fallback): the minimum is one named laptop with integrated g… | R3-BUDGET-OQ1–OQ6 | spot-checked (45 of 77 key claims) |
| R3 | 6 | how we measure it | R3-MEASURE-REC1–REC4 | One client measurement harness: per frame it records CPU frame time from performance.now(), GPU time from time… | R3-MEASURE-OQ1–OQ4 | spot-checked (45 of 77 key claims) |
| R3 | 7 | spatial audio with Web Audio | R3-AUDIO-REC1–REC4 | One AudioListener at the local player's position, oriented with the camera, as long as the camera is legacy-li… | R3-AUDIO-OQ1–OQ6 | spot-checked (45 of 77 key claims) |
| R4 | 1 | Rapier 3D or custom | R4-PHYS-REC1–REC5 | No physics engine: use parry3d (pinned, enhanced-determinism on) for shapes, queries and the BVH, and write th… | R4-PHYS-OQ1–OQ5 | spot-checked (50 of 52 key claims) |
| R4 | 2 | navigation meshes in Rust | R4-NAV-REC1–REC4 | The sim owns the navigation runtime; the navigation data is baked by a tool and stored in the map document (op… | R4-NAV-OQ1–OQ5 | spot-checked (50 of 52 key claims) |
| R4 | 3 | bots moving between floors | R4-FLOORS-REC1–REC3 | Typed links generated by the bake and checked by running the sim's own movement, with editor overrides (option… | R4-FLOORS-OQ1–OQ4 | spot-checked (50 of 52 key claims) |
| R4 | 4 | line of sight across heights | R4-LOS-REC1–REC3 | One height-aware sight query in the sim, used by bots, blasts, melee, the shield check and the shared visibili… | R4-LOS-OQ1–OQ5 | spot-checked (50 of 52 key claims) |
| R4 | 5 | obstacles placed mid-match (for later building) | R4-OBST-REC1–REC3 | Moving vehicles are never carved into the navigation data: they are obstacles in the sim's avoidance step, by … | R4-OBST-OQ1–OQ4 | spot-checked (50 of 52 key claims) |
| R5 | 1 | Repository layout (a Cargo workspace plus TypeScript with Vite) | R5-LAYOUT-REC1–REC6 | One repository with both workspaces at its root (option A), npm workspaces, and pinned toolchains. | R5-LAYOUT-OQ1–OQ5 | spot-checked (61 of 115 key claims) |
| R5 | 2 | strict TypeScript and lint rules | R5-TSLINT-REC1–REC4 | TypeScript 6.0.x, pinned exactly, with one shared strict base tsconfig. | R5-TSLINT-OQ1–OQ6 | spot-checked (61 of 115 key claims) |
| R5 | 3 | clippy and rustfmt | R5-RUSTLINT-REC1–REC6 | One workspace lint table, every crate opts in, and the gate turns warnings into errors (option B). | R5-RUSTLINT-OQ1–OQ5 | spot-checked (61 of 115 key claims) |
| R5 | 4 | tests (cargo test, property tests, Vitest, Playwright, golden tests against legacy) | R5-TEST-REC1–REC6 | The layers, the tool for each, and where each runs. | R5-TEST-OQ1–OQ8 | spot-checked (61 of 115 key claims) |
| R5 | 5 | CI with Rust caching | R5-CI-REC1–REC5 | Cache the Rust dependencies with Swatinem/rust-cache v2.9.3, pinned by SHA. Use one shared-key per build kind … | R5-CI-OQ1–OQ7 | spot-checked (61 of 115 key claims) |
| R5 | 6 | preview links per PR | R5-PREV-REC1–REC5 | Use workflow shape P1. Build in the ordinary pull_request workflow with no secret, and deploy from a separate … | R5-PREV-OQ1–OQ6 | spot-checked (61 of 115 key claims) |
| R5 | 7 | the gate v2 that replaces today's gate | R5-GATE-REC1–REC4 | The gate v2's checks, in order, fastest first, each with an id and a reason. | R5-GATE-OQ1–OQ5 | spot-checked (61 of 115 key claims) |
| R6 | 1 | Hosting for previews and EU game servers | R6-HOST-REC1–REC4 | Host the client and the per-PR previews on Cloudflare Pages (GLOSSARY: the client host), with Workers static a… | R6-HOST-OQ1–OQ7 | spot-checked (21 of 42 key claims) |
| R6 | 2 | Costs | R6-COST-REC1–REC2 | Budget the start at about 18 EUR per month (lean) to about 102 EUR + 5 USD (reference), and the goal at about … | R6-COST-OQ1–OQ5 | spot-checked (21 of 42 key claims) |
| R6 | 3 | Logs, metrics and crash reports | R6-OBS-REC1–REC4 | Server logs: tracing with JSON lines on standard output per room process; the supervisor adds VM and room ids … | R6-OBS-OQ1–OQ5 | spot-checked (21 of 42 key claims) |
| R6 | 4 | Balance telemetry (VISION_V2 §6.6) | R6-TEL-REC1–REC3 | Every match writes one record (W2), recorded inside the room (R-a), with this shape (proposed): | R6-TEL-OQ1–OQ5 | spot-checked (21 of 42 key claims) |
| R6 | 5 | Security baseline (deferred) | R6-SEC-– | – | R6-SEC-OQ1 | spot-checked (21 of 42 key claims) |

## 2. The 12 contradictions: the resolutions as approved, and the advisor's rulings (word for word)

### The resolutions, as folded after the advisor's step-3b verdict

From `.checkpoints/v2-research/resolutions.md` at commit 85f89ea (that folder is deleted in this PR's last commit; the history keeps it).

### Resolutions of the critic's 12 contradictions (step 3a, proposals for the advisor's review)

- **Task:** step 3a of `plan-amendments-2.md` (advisor conditions 1–4 applied). Author: engine-architect, 2026-10-30.
- **Status:** proposals. Nothing here is applied; the advisor reviews each item (step 3b, verdict to
  `advisor-resolutions-verdict.md`), and the owners apply only approved items (step 3c).
- **Inputs:** the six topic files as they stand at commit 8dee746 (line numbers below are of that state), the critic's
  list `engine-architect-consistency-result.json`, VISION_V2 §3, §4 and §6, and the advisor's table "Who decides each
  contradiction" (`advisor-amendments-2-verdict.md`). No page was fetched; no new source is used.
- **Outcomes:** **resolved** (one position holds, from findings the files already hold), **aligned** (one shared open
  question, the same words in every affected topic, one decider and a decision code) or **escalated** (a question for
  Mustafa or the Advisor, with options and a recommendation).
- **Numbers:** every changed number comes from a topic file (its id is given), is arithmetic on those numbers (shown
  under "Arithmetic"), or is a pass mark marked "proposed" with its reason.
- **Glossary:** terms such as "the stress scene" or "the server tick budget" are defined in `docs/research/GLOSSARY.md`.
  In the "after" texts, `(GLOSSARY)` marks a first use, which the owner turns into a link.
- **How the before/after texts work:** each change has an id `C<rank>.<n>`. "before" quotes the file at the line
  given, exactly, between the first and the last backtick of the item (inner backticks are the file's own); "after" is
  the replacement for exactly that text, and text outside the quote stays. A line break inside a quote is the file's
  line break, with its indentation. Where an item depends on an escalated decision, the "after" text holds under every
  option (advisor condition 2); changes marked "only if … picks (b)" wait for that answer. Several resolutions change
  different parts of one sentence; their quoted parts never overlap. A script checked on 2026-10-30 that each of the
  116 "before" quotes occurs exactly once in its file (whitespace and line breaks ignored). Owners apply the approved
  changes first and the glossary's renames after, so the quotes still match.
- **Order of work** (advisor condition 2): #10, #11 and #9 with their glossary terms, then #1; #19 before #21 and #22;
  #5 with #6; then #17, #23, #29. The items below are listed by rank.

#### Summary

| Rank | What clashes | Outcome | Decider and code | Changes |
|---|---|---|---|---|
| 1 | What the tick budget covers, and its shares | resolved: one server tick budget, 50 % of the tick period, split into a sim step share and a server-only share of 25 % each (25 % for the whole tick weighed and set aside) | the advisor (engineering) | C1.1–C1.22 |
| 5 | Whether the client predicts the player's own abilities | aligned: one shared question; weapons predicted, damage never, abilities wait | Mustafa after a Feel Lab test, **ABL-PREDICT** (new) | C5.1–C5.10 |
| 6 | The spread random stream: sim state or server-only | resolved for the design: authority-only streams in R1; the trade-off stays open | the advisor; **SEC-SPREAD** stays Mustafa's | C6.1–C6.7 |
| 9 | The stress scene | resolved: one proposed worst-case scene (the largest mode's N in three teams, 15 big vehicles of which 4 very large units, 300 live projectiles, 8 active effects per soldier) | the advisor; caps later (R2-BUD-OQ5) | C9.1–C9.22 |
| 10 | Mode sizes: totals or per team | escalated, with wording that holds under both readings | Mustafa, **MODES-3** | C10.1–C10.23 |
| 11 | One core or one vCPU | resolved: one vCPU of the reference server (CCX23) | the advisor | C11.1–C11.13 |
| 17 | Who owns the dependency audit in CI | escalated, with cross-references that hold under every option | the Advisor, **BASELINE-WHEN** | C17.1–C17.9, C17.b1–b3 |
| 19 | Previews on Workers or Pages | resolved: Cloudflare Pages first, Workers static assets the alternative; one aligned spike item | the advisor; spike **PREV-DELETE** | C19.1–C19.9 |
| 21 | A login wall on previews | escalated; until answered, no preview talks to a game server | Mustafa, **PREVIEW-WALL** | C21.1–C21.7 |
| 22 | Isolation headers on every preview, or a flag | resolved: one isolation flag, off by default; Mustafa's preview carries production's headers | the advisor | C22.1–C22.6 |
| 23 | One wildcard certificate on every game server | escalated (same question as 17) | the Advisor, **BASELINE-WHEN** | C23.1–C23.3, C23.b1–b3 |
| 29 | HashMap in the sim: lookups or a type ban | resolved: the type ban holds, with a reviewed `#[allow]` for a lookup-only map outside the serialized state | the advisor | C29.1–C29.4 |

Counts (advisor condition 1): **7 resolved** (1, 6, 9, 11, 19, 22, 29; 6 leaves Mustafa's SEC-SPREAD open and 19
carries the aligned spike item PREV-DELETE), **1 aligned** (5), **4 escalated** (10, 17, 21, 23; three codes).
Shared questions to insert: MODES-3 (6 topics), ABL-PREDICT (R1, R2), SEC-SPREAD (R1, R2), PREV-DELETE (R5, R6),
PREVIEW-WALL (R5, R6), BASELINE-WHEN (R1, R2, R5, R6).

#### Where shared open questions go

An aligned or escalated item becomes one shared open question with the same words in every affected topic. Each topic
gets one new subsection for them, **"### Shared open questions (the same words in every topic)"**, at the end of its
open questions, placed before:
- R1: `### Pointers between R1's units` (r1:2872);
- R2: `### Pointers between the R2 units` (r2:3215);
- R3: `### Pointers to other topics` (r3:1398);
- R4: `### Wave-1 items R4 changes or answers (said openly, not silently)` (r4:1110);
- R5: `### Recommendations that hinge on a source not read` (r5:1889);
- R6: `### Recommendations that hinge on a source not read` (r6:915).

Each shared question is headed by its decision code in bold (for example **MODES-3 (Mustafa).**), so a search for the
code finds every copy (advisor condition 11). A topic's own earlier question on the same point stays and gains one
closing sentence, "Shared as <code> (below).". The critic's other 29 items go in a second new subsection after it,
`### Cross-topic open questions (consistency critic, 2026-10-29)`, with ids `R<n>-XT-OQ<k>` (see `cross-topic-oqs.md`).


| Rank | What clashes | Outcome | Decider and code |
|---|---|---|---|

#### 1. What the tick budget covers, and its shares

**As approved, 2026-10-30 (advisor, step 3b; `advisor-resolutions-verdict.md`):** REVISE, folded (the decision stands; the arithmetic, the p99 note and changes C1.5, C1.9, C1.10, C1.21 revised; new C1.10b).

**Outcome: resolved** (one server tick budget, split into two shares; every existing figure placed inside one of them).
Decider: the advisor (the advisor's table: "Me (engineering), after #9/#10/#11. The 60-or-30 Hz rule stays as it is").
It builds on resolution 9 (the stress scene), 10 (the largest mode's N) and 11 (one vCPU of the reference server).

**Both sides.**
- The sim alone: r1:2014-2018 (R1-TICK-REC2 P1) "At 60 Hz: p99 tick time at N = 48 at most 4.2 ms (25 % of 16.7 ms)
  … The 25 % leaves room for snapshot encoding, interest management and several matches per core (R2, R6)"; r1:2025
  "Decision rule: 60 Hz if P1–P3 pass at 60 Hz"; r1:1908-1909 (R1-ECS-REC1 mark 1) "a mean tick ≤ 2 ms and a p99 tick
  ≤ 4 ms native"; r1:2394-2396 (R1-ABL-REC6) "at most 1 ms per tick"; r4:691-692 (R4-PHYS-REC1 mark 2) "at most half of
  that mark's mean tick (≤ 1 ms native mean)"; r4:841 (R4-NAV-REC2) "≤ 10 % of the tick's p99 budget".
- The whole or other parts: r2:2377-2379 (R2-BUD-REC6) "for the whole tick, not just the simulation step: at 48
  players, p99 ≤ 25 % of the tick interval (≤ 4 ms at 60 Hz, ≤ 8 ms at 30 Hz)"; r2:2284 (R2-BUD-REC1) "60 Hz if R1's
  tick spike meets its CPU mark"; r2:2733 (R2-ROOM-REC2) "p99 tick time at most 50 % of the tick period"; r2:2276-2278
  (R2-SYNC-REC9) sync work "at most 25 % of the tick budget"; r2:2447-2448 (R2-INT-REC3) "at most 10 % of the server
  tick budget"; r2:2558 (R2-TRN-REC1 mark 2) "at most 1 CPU core for the transport layer"; r6:512 (R6 A-2) "8 rooms of
  48 players per 4-vCPU VM".
- Existing questions: R2-XU-OQ8 (R2's shares only) and R1-XU-OQ1 (R1's units only).

**The proposal (GLOSSARY: the server tick budget; the sim step share and the server-only share).**
- One total: in the stress scene, on one vCPU of the reference server, the p99 server tick time is at most **50 % of
  the tick period** (8.3 ms at 60 Hz, 16.7 ms at 30 Hz), the max under one tick period, zero overruns. This is
  R2-ROOM-REC2's figure.
- Two shares of 25 % of the tick period each (4.2 ms at 60 Hz, 8.3 ms at 30 Hz): the **sim step share**
  (R1-TICK-REC2 P1's figure, unchanged) and the **server-only share** (R2-SYNC-REC9's figure, read against the tick
  period: command handling, lag-compensated hit tests, interest, snapshot encoding, the tick's send calls).
- Inside each share, every topic keeps its own mark, now stated against the defined whole and reported separately:
  movement and physics ≤ 1 ms mean (R4-PHYS-REC1), abilities ≤ 1 ms (R1-ABL-REC6), navigation ≤ 10 % of the budget =
  5 % of the period (R4-NAV-REC2), the visibility pass ≤ 10 % of the budget = 5 % of the period (R2-INT-REC3) in the
  sim step share; encoding ≤ 50 µs per client (R2-BUD-REC6), a rewound shot ≤ 20 µs (R2-SYNC-REC5), the region query
  ≤ 0.2 ms (R2-INT-REC1), ranking and packing ≤ 0.5 ms (R2-INT-REC4) in the server-only share.
- The network runtime's own CPU (encryption, sockets) runs beside the tick, not in it (R2-ROOM-REC2: "network I/O on an
  async runtime"); it is the transport runtime time (GLOSSARY), capped by R2-TRN-REC1 mark 2 and counted in the
  rooms-per-VM figure, not in the tick budget.
- The 60-or-30 Hz rule stays word for word (R1-TICK-REC2: "60 Hz if P1–P3 pass at 60 Hz; otherwise 30 Hz if they
  pass at 30 Hz …"; R2-BUD-REC1: "60 Hz if R1's tick spike meets its CPU mark, else 30 Hz"). P1's CPU mark is the sim
  step share, as today; R2-BUD-REC6 checks the whole at the rate the rule picks.
- A time mark that names no statistic is a p99 (GLOSSARY: time statistics).

**Arithmetic: do the parts fit?** Each part at its own ceiling. Adding the parts' p99 ceilings is a planning check that the marks fit together, not a bound: p99s do not add (two parts that each take 1 ms on 0.6 % of ticks and nothing otherwise each have a p99 of 0, but together they take 1 ms on about 1.2 % of ticks, so the sum's p99 is 1 ms). The server tick time's p99 is always measured directly, with the parts reported beside it. Snapshots run at 30 Hz
(R2-BUD-REC1), so at a 60 Hz tick every second tick encodes, at 30 Hz every tick. Tick period 16.67 ms (60 Hz), 33.33 ms
(30 Hz); 25 % = 4.17 or 8.33 ms; 5 % = 0.83 or 1.67 ms.

| Part (N = 48) | Mark and id | 60 Hz, ms | 30 Hz, ms |
|---|---|---|---|
| movement and physics | ≤ 1 ms mean, R4-PHYS-REC1 mark 2 (mean mark; its p99 is reported, not capped) | 1.00 | 1.00 |
| avoidance | ≤ 0.2 ms p99, R4-NAV-REC3 (r4:860) | 0.20 | 0.20 |
| abilities, effects, weapons | ≤ 1 ms, R1-ABL-REC6 | 1.00 | 1.00 |
| navigation | 5 % of the period, R4-NAV-REC2 | 0.83 | 1.67 |
| visibility pass | 5 % of the period, R2-INT-REC3 | 0.83 | 1.67 |
| **sim step share, sum (limit)** | | **3.87 (4.17)** | **5.53 (8.33)** |
| snapshot encoding | 48 × 50 µs, R2-BUD-REC6 | 2.40 | 2.40 |
| lag compensation | 700 shots/s × 20 µs = 14 ms/s, R2-SYNC-REC5 mark 4 | 0.23 | 0.47 |
| interest | 0.2 + 0.5 ms, R2-INT-REC1, R2-INT-REC4 | 0.70 | 0.70 |
| enclosure pair test | 0.1 µs × 768 enemy pairs, R2-INT-REC2 (b) | 0.08 | 0.08 |
| **server-only share, sum (limit)** | | **3.41 (4.17)** | **3.64 (8.33)** |
| re-flood after a portal change (that tick only) | +1.00 ms, R2-INT-REC2 (b) | +1.00 | +1.00 |
| server-only share on a re-flood tick | | 4.41 | 4.64 |
| **server tick, sum (limit 50 %)** | | **7.28 (8.33)** | **9.18 (16.67)** |
| server tick on a re-flood tick | | 8.28 | 10.18 |

What is left at 60 Hz: 0.30 ms in the sim step share, 0.76 ms in the server-only share (−0.24 on a re-flood tick). At
30 Hz (arithmetic on the table above): 2.80 ms and 4.69 ms (3.69 on a re-flood tick).

At N = 54 (MODES-3 (b); the same 1.125 scaling of the per-soldier, per-client and per-shot marks, an assumption for
this check only, and 972 enemy pairs):

| | 60 Hz | 30 Hz |
|---|---|---|
| Sim step share | 4.14 ms (left 0.03) | 5.81 ms |
| Server-only share | 3.85 ms | 4.12 ms |
| Server-only, re-flood tick | 4.85 ms | 5.12 ms |
| Server tick | 7.99 ms | 9.93 ms |
| Server tick, re-flood tick | 8.99 ms (over 8.33) | 10.93 ms |

The tightest part is the visibility pass: R2-INT-F20's legacy JavaScript figure for a three-point pass of 768 pairs is
6–13 ms (unindexed, per pass), against 0.83 ms per tick at 60 Hz here; the pass is amortized across ticks
(R2-INT-REC3), and a Rust cast with a spatial index has not been measured (R2-INT-F20). The spike measures it; the
proposal does not hide it.

**The option the advisor may prefer instead: 25 % for the whole tick (R2-BUD-REC6's figure).** The whole server tick
≤ 4.17 ms at 60 Hz, 8.33 ms at 30 Hz, with R2-INT-REC3's and R4-NAV-REC2's "10 %" then 2.5 % of the period (0.42 or
0.83 ms). Arithmetic at N = 48, the same rows as above: at 60 Hz 3.03 + 3.41 = 6.44 ms (5.24 ms with encoding spread over two ticks), against 4.17. At 30 Hz 7.51 ms, or 8.51 ms with a re-flood, against 8.33. So under 25 % the topics' own marks cannot meet the budget at 60 Hz on any snapshot tick, nor at 30 Hz on a re-flood tick, and the 60-or-30 rule would land on 30 Hz by arithmetic before
P1–P3 are measured. Its gain: two rooms per vCPU at the ceiling use half of it, leaving the other half for the
transport runtimes and the system, so R6's A-2 would follow from the budget.

**Why 50 % (the recommendation).**
1. At 30 Hz the topics' marks fit inside it at N = 48 and N = 54, re-flood ticks included. At 60 Hz they fit on every
   tick without a re-flood; on a tick that encodes a snapshot and re-floods after a portal change, the server-only
   share's ceilings (4.41 ms at N = 48) pass its 4.17 ms and, at N = 54, the whole (8.99 ms) passes 8.33 ms; at N = 54
   the sim step share keeps 0.03 ms for the rest of the sim. Under 25 % for the whole tick the marks do not fit at
   60 Hz on any snapshot tick, nor at 30 Hz on a re-flood tick, which would decide the tick rate before the spike the
   rule waits for. The 60 Hz tight spots (the visibility pass, the re-flood tick, the rest of the sim at N = 54) are
   for the spike to measure.
2. It changes the fewest marks. R1-TICK-REC2 P1's 25 %, R2-SYNC-REC9's 25 % and R2-ROOM-REC2's 50 % keep their
   numbers. R2-BUD-REC6's whole moves from 25 % to 50 %, R1-ECS-REC1 mark 1's p99 from 4 ms to 4.2 ms (R1-TICK-REC2's
   arithmetic for 25 % of 16.7 ms; R1-XU-OQ1 already lists the two) with its 2 ms mean now reported, and the nested
   "10 %" marks get their value (5 % of the period). R4-PHYS-REC1 keeps its 1 ms.
3. It keeps room for bursts: a p99 tick at half the period can double before it overruns (arithmetic), and the max
   stays under one period with zero overruns (R1-TICK-REC2 P1).

**What it costs, said openly.** The budget no longer promises more than one room per vCPU: at the ceiling two rooms
fill a vCPU at p99 before the transport runtime (arithmetic). R6's A-2 (2 rooms per vCPU) becomes a density to measure
(R2-ROOM-REC2 mark 2, R6-COST-OQ1), not one the budget guarantees; if it fails, the compute per room-hour doubles
(R6's own rows: 0.0173 against 0.0347 EUR, r6:452-453). The density is measured at the tick rate the rule picks,
because 30 Hz "roughly halves the CPU per simulated second" (R1-TICK-REC2, inferred). R1-TICK-REC2's "several matches
per core" and R2-BUD-REC6's "more than one room per core" become "measured, not budgeted". R2-TRN-REC1 mark 2's
"1 vCPU for the transport layer" for one process cannot sit beside 2 rooms per vCPU either; its measured share per room
enters the same density figure.

*Changes per file (before → after).* Line numbers are of commit 8dee746; resolutions 9 and 11 change other parts of
some of the same sentences (their substrings do not overlap with these).

R1:
- C1.1, r1:1908-1909 (R1-ECS-REC1 mark 1):
  - before: `The pass mark: a mean tick ≤ 2 ms and
     a p99 tick ≤ 4 ms native;`
  - after: `The pass mark: a p99 sim step time within the sim step share (GLOSSARY: 25 % of the tick period, 4.2 ms at
     60 Hz and 8.3 ms at 30 Hz, the mark of R1-TICK-REC2 P1), the mean reported, native;`
- C1.2, r1:2015 (R1-TICK-REC2 P1):
  - before: `4.2 ms (25 % of 16.7 ms)`
  - after: `4.2 ms (the sim step share, GLOSSARY: 25 % of 16.7 ms)`
- C1.3, r1:2017-2018 (R1-TICK-REC2 P1):
  - before: `The 25 % leaves room for snapshot encoding, interest management and several matches per core (R2, R6); headroom
  itself is Fiedler's advice (F3).`
  - after: `The other 25 % of the server tick budget (GLOSSARY: 50 % of the tick period) is the server-only share, for
  snapshot encoding, interest management, lag compensation and commands (R2-SYNC-REC9); how many matches share a vCPU
  is measured, not budgeted (R2-ROOM-REC2 mark 2); headroom
  itself is Fiedler's advice (F3).`
- C1.4, r1:2136 (R1-DET-REC3, spike):
  - before: `it fits the per-tick sim budget the tick-rate unit sets.`
  - after: `it fits inside the sim step share (GLOSSARY).`
- C1.5, r1:2396 (R1-ABL-REC6):
  - before: `costs at most 1 ms per tick`
  - after: `costs at most 1 ms per tick at p99, inside the sim step share on the server and the client's sim share on the laptop (GLOSSARY),`
- C1.6, r1:2546-2547 (R1-ECS-OQ4):
  - before: `Decided by: REC1 mark 1's
  native tick time against the tick period the tick unit picks, and R6's hosting cost per match.`
  - after: `Decided by: REC1 mark 1's
  native sim step time against the sim step share (GLOSSARY) at the tick rate the tick unit picks, and R6's hosting
  cost per match.`
- R1-XU-OQ1: settled by C9.8's closing sentence (one stress scene and one server tick budget).

R2:
- C1.7, r2:2016-2017 (R2-SYNC-REC1 mark 4):
  - before: `stays within the share of the tick budget the tick-rate unit sets.`
  - after: `stays within the server-only share (GLOSSARY: 25 % of the tick period, shared with lag compensation,
     interest, commands and the tick's sends).`
- C1.8, r2:2276-2278 (R2-SYNC-REC9 mark 1):
  - before: `Snapshot
     encoding, lag compensation and command handling together use at most 25 % of the tick budget at the 99th
     percentile on the proposed reference server.`
  - after: `Snapshot
     encoding, lag compensation, command handling, interest (the region query, ranking and packing) and the tick's
     send calls together stay within the server-only share (GLOSSARY: 25 % of the tick period) at the 99th
     percentile on one vCPU of the reference server.`
- C1.9, r2:2377-2381 (R2-BUD-REC6, heading and reason):
  - before: `A server CPU budget per tick for the whole tick, not just the simulation step: at 48 players, p99 ≤ 25
  % of the tick interval (≤ 4 ms at 60 Hz, ≤ 8 ms at 30 Hz) including the simulation (R1), lag-compensated hit tests
  and snapshot encoding for every client, measured separately.** Reason: no source read gives a tick budget; 25 %
  leaves room for bursts and for more than one room per core; per-client encode cost is the unknown that grows with
  player count.`
  - after: `A server CPU budget per tick for the whole tick, not just the simulation step: at the largest mode's N,
  p99 server tick time within the server tick budget (GLOSSARY: ≤ 50 % of the tick period, ≤ 8.3 ms at 60 Hz,
  ≤ 16.7 ms at 30 Hz), split into the sim step share (R1) and the server-only share (lag-compensated hit tests,
  interest, snapshot encoding for every client, commands and the tick's sends), 25 % each, measured separately.**
  Reason: no source read gives a tick budget; the topics' own part marks fit inside 50 % at 30 Hz, and at 60 Hz on
  every tick but one that both encodes a snapshot and re-floods after a portal change, while 25 % for the whole tick
  would not hold them at 60 Hz on any snapshot tick (the arithmetic, a planning check of ceilings, is in GLOSSARY's
  tick-budget section); 50 % leaves
  room for bursts (a p99 tick at half the period can double before it overruns); how many rooms share a vCPU is
  measured (R2-ROOM-REC2 mark 2), not budgeted; per-client encode cost is the unknown that grows with player count.`
- C1.10, r2:2385-2387 (R2-BUD-REC6, spike):
  - before: `48
  players, 30-minute soak: p99 whole tick ≤ 4 ms at 60 Hz (or ≤ 8 ms at 30 Hz), 0 missed ticks,`
  - after: `the
  stress scene (GLOSSARY) with a snapshot-receiving client on every soldier (R2-SYNC-REC9 mark 1's "48 clients"),
  30-minute soak: p99 server tick time ≤ 8.3 ms at 60 Hz (or ≤ 16.7 ms at 30 Hz), the sim
  step and the server-only work each within their 25 % share, 0 missed ticks,`
- C1.10b, r2:2387 (R2-BUD-REC6, spike, after C1.10; new in step 3b):
  - before: `per client per snapshot, memory per room reported.`
  - after: `per client per snapshot, memory per room reported. The mark is checked at the tick rate R1-TICK-REC2's rule
  picks; if it fails there, it fails like any spike mark and goes back to the drafts, and the rule does not change.`
- C1.11, r2:2447-2449 (R2-INT-REC3, spike):
  - before: `takes at most 10 % of the server tick budget on the reference server (the tick rate comes
    from the tick unit)`
  - after: `takes at most 10 % of the server tick budget (GLOSSARY: 5 % of the tick period, 0.8 ms at 60 Hz and 1.7 ms
    at 30 Hz, inside the sim step share) on one vCPU of the reference server (the tick rate comes
    from the tick unit)`
- C1.12, r2:2733 (R2-ROOM-REC2, spike):
  - before: `p99 tick time at most 50 % of the tick period chosen by the tick-rate unit,`
  - after: `p99 server tick time within the server tick budget (GLOSSARY: 50 % of the tick period chosen by the tick-rate
  unit),`
- C1.13, r2:2559 (R2-TRN-REC1 mark 2), a sentence added after the mark's last sentence, before `3. Swap cost:`:
  - after: `The transport runtime (GLOSSARY) runs beside the tick, not in it; its measured share of a vCPU per room enters
   the rooms-per-VM figure (R2-ROOM-REC2 mark 2, R6-COST-OQ1).`
- C1.14, r2:3140-3141 (R2-XU-OQ8), at the end of the entry:
  - before: `Decided by:
  the synthesis, which sets one tick budget and divides it; then the spikes.`
  - after: `Decided by:
  the synthesis, which sets one tick budget and divides it; then the spikes. Settled by the shared glossary
  (2026-10-30): one server tick budget, 50 % of the tick period, split into a sim step share and a server-only share
  of 25 % each, every figure above placed inside one of them (GLOSSARY).`

R4:
- C1.15, r4:691-692 (R4-PHYS-REC1 mark 2):
  - before: `take at most half of that mark's
     mean tick (≤ 1 ms native mean),`
  - after: `take at most 1 ms native mean, with the
     p99 reported, inside the sim step share (GLOSSARY),`
- C1.16, r4:841 (R4-NAV-REC2, spike):
  - before: `takes ≤ 10 % of the tick's p99 budget`
  - after: `takes ≤ 10 % of the server tick budget at p99 (GLOSSARY: 5 % of the tick period, inside the sim step
  share)`
- C1.17, r4:954 (R4-LOS-REC1 mark 4):
  - before: `inside its 10 % budget.`
  - after: `inside its share (GLOSSARY: 5 % of the tick period).`
- C1.18, r4:970 (R4-LOS-REC2, spike):
  - before: `within R2-INT-REC3's 10 % budget`
  - after: `within R2-INT-REC3's share (GLOSSARY: 5 % of the tick period)`

R5:
- C1.19, r5:1472-1473 (R5-TEST-REC5):
  - before: `Measured: server tick p50,
  p99 and max (against R1-TICK's budget),`
  - after: `Measured: server tick time p50,
  p99 and max (against the server tick budget, GLOSSARY),`
- C1.20, r5:1481 (R5-TEST-REC5, spike):
  - before: `and tick p99 inside the budget ADR-0004 sets;`
  - after: `and server tick time p99 inside the server tick budget (GLOSSARY; ADR-0004 and ADR-0001 adopt it);`

R6:
- C1.21, r6:512 (A-2):
  - before: `- **A-2.** 8 rooms of 48 players per 4-vCPU VM (R2-ROOM-REC2's proposal); smaller rooms fit at least as many.`
  - after: `- **A-2.** 8 rooms of 48 players per 4-vCPU VM (R2-ROOM-REC2's proposal); smaller rooms fit at least as many.
  It is a density to measure at the tick rate R1-TICK-REC2's rule picks (R2-ROOM-REC2 mark 2), not one the server tick
  budget guarantees: at the budget's ceiling (GLOSSARY: 50 % of one vCPU per room at p99) two rooms fill a vCPU before
  the transport runtime, so the 4-rooms-per-VM row (one per vCPU) is the case to budget unless R2-ROOM-REC2 marks 1 and 2 pass on it
  (R6-COST-OQ1). At 30 Hz the CPU per simulated second roughly halves (R1-TICK-REC2,
  inferred).`
- C1.22, r6:850-851 (R6-COST-OQ1):
  - before: `Decided by: R2-ROOM-REC2's spike on the reference server (R6-HOST-REC2).`
  - after: `Decided by: R2-ROOM-REC2's spike on the reference server (R6-HOST-REC2), at the tick rate R1-TICK-REC2's rule
  picks, with each room's transport runtime counted (GLOSSARY).`

*Combined view of R1-TICK-REC2 P1 with resolutions 1, 9 and 11 and the glossary's "sim step time" applied* (for the
advisor; the owner applies the parts one by one):

> P1 server, native: the stress scene (GLOSSARY) at N = 12 / 24 / 36 / 48 / 64 soldiers and at the largest mode's N
> where it is not among them (MODES-3) (bots with full AI, worst case: all in one combat cluster), with its 15 big
> vehicles, 300 live projectiles and up to 8 active effects per soldier, on a DM_Tri-sized greybox, 36,000 ticks per
> run, 3 runs. At 60 Hz: p99 sim step time at the largest mode's N at most 4.2 ms (the sim step share, GLOSSARY: 25 % of
> 16.7 ms) on one vCPU of the reference server, max at most 16.7 ms, zero overruns. At 30 Hz: p99 at most 8.3 ms (25 %
> of 33.3 ms). Report mean, p50, p95, p99 and max per N. The other 25 % of the server tick budget (GLOSSARY: 50 % of the
> tick period) is the server-only share, for snapshot encoding, interest management, lag compensation and commands
> (R2-SYNC-REC9); how many matches share a vCPU is measured, not budgeted (R2-ROOM-REC2 mark 2); headroom itself is
> Fiedler's advice (F3).

**Basis ids:** R1-TICK-REC2, R1-ECS-REC1, R1-ABL-REC6, R1-DET-REC3, R1-ECS-OQ4, R1-XU-OQ1, R2-BUD-REC1, R2-BUD-REC6,
R2-SYNC-REC1, R2-SYNC-REC5, R2-SYNC-REC9, R2-INT-REC1, R2-INT-REC3, R2-INT-REC4, R2-INT-F20, R2-TRN-REC1, R2-TRN-REC6,
R2-ROOM-REC2, R2-XU-OQ8, R4-PHYS-REC1, R4-NAV-REC2, R4-NAV-REC3, R4-LOS-REC1, R4-LOS-REC2, R5-TEST-REC5, R6 A-2,
R6-COST-OQ1, the cost rows r6:452-453. **New numbers:** 8.3 and 16.7 ms (50 % of the period), 4.2 and 8.3 ms (25 %),
0.8 and 1.7 ms (5 %), 2.4 ms, 0.23 and 0.47 ms, and every sum in the table are arithmetic (shown); the 50 % total and
the 25 % / 25 % split are figures from R2-ROOM-REC2, R1-TICK-REC2 and R2-SYNC-REC9, combined as a proposal; the 1.125
scaling is an assumption for the check only.


#### 5. Whether the client predicts the player's own abilities

**As approved, 2026-10-30 (advisor, step 3b; `advisor-resolutions-verdict.md`):** APPROVE (aligned as ABL-PREDICT).

**Outcome: aligned. Decision code ABL-PREDICT (new); decider: Mustafa (game feel), after the Feel Lab comparison that
R2-SYNC-OQ10 names.** The advisor's table: "Shared wording now (R1-ABL-REC4 depends on R2-SYNC-OQ10). The final call is
Mustafa's (game feel), after a Feel Lab test. Nothing to decide now." Written together with resolution 6.

**Both sides.**
- r1:2354-2356 (R1-ABL-REC4): "Predict only the local player's own abilities and weapons; never damage (option D2).
  The client predicts its own activation, cost, cooldown, granted tags, movement changes (Banzai speed, Shield slow),
  weapon timer and ammo".
- r2:2061-2062 (R2-SYNC-REC3): "Client prediction: own movement, own weapon state, own vehicle and cosmetic shot
  feedback (option 3.2 C)"; r2:2889-2890 (R2-SYNC-OQ10): "Predicting abilities (option 3.2 D). No source on predicting
  abilities and rolling them back was read"; r2:3341: "The prediction scope stops short of abilities (option 3.2 D,
  R2-SYNC-OQ10)".
- Existing questions: R1-ABL-OQ4 and R2-SYNC-OQ10.

**Why aligned, not resolved.** No finding in any topic settles whether ability prediction feels better enough to pay
for its rollback (R2-SYNC-OQ10: no source read); the files name a Feel Lab comparison as the test and the call is game
feel, Mustafa's. Until then both topics must say the same thing: weapons are predicted (both agree: R1's "weapon timer
and ammo", R2's "own weapon state"), damage never (both agree), and abilities wait for ABL-PREDICT. The ability system
is designed so that either answer needs no redesign: R1-ABL-REC4's prediction key, rollback and cue rule are what a
"yes" needs, and with a "no" its ability marks simply drop out.

*Shared question, to insert in R1 and R2 (the "Shared open questions" subsection), word for word:*

> **ABL-PREDICT (Mustafa, after a Feel Lab test). Does the client predict the local player's own abilities?** The
> prediction scope (GLOSSARY) covers the player's own movement, own weapon state, own vehicle and cosmetic shot
> feedback (R2-SYNC-REC3, option 3.2 C), and never damage. R1-ABL-REC4 (option D2) would add the player's own ability
> activation, cost, cooldown, granted tags and movement changes (Banzai speed, Shield slow). No source on predicting
> abilities and rolling them back was read (R2-SYNC-OQ10). If yes, the ability system uses R1-ABL-REC4's prediction
> key (the command's sequence number), undoes a rejected press and keys cues so that a re-simulated activation does
> not play twice; whether that runs through per-effect keys or through R2-SYNC-REC3's replay from the authoritative
> state is R1-ABL-OQ4. If no, ability presses wait for the server's confirmation and R1-ABL-REC4's ability marks drop
> out. Decided by: R2-SYNC-OQ10's Feel Lab comparison of predicted against server-confirmed Molotov and Shield (or a
> readable source), then Mustafa (game feel). Also asked as R2-SYNC-OQ10 and R1-ABL-OQ4.

*Changes per file (before → after).*

R1:
- C5.1, r1:2354 (R1-ABL-REC4, heading):
  - before: `**R1-ABL-REC4. Predict only the local player's own abilities and weapons; never damage (option D2).**`
  - after: `**R1-ABL-REC4. Predict only the local player's own weapons, and its own abilities if ABL-PREDICT says so; never
  damage (option D2).**`
- C5.2, r1:2355-2356 (R1-ABL-REC4):
  - before: `The client predicts its own activation, cost, cooldown, granted tags, movement changes (Banzai speed, Shield slow),
  weapon timer and ammo, and the cues of those;`
  - after: `The client predicts its own weapon timer and ammo (R2-SYNC-REC3's own weapon state) and, if ABL-PREDICT (below)
  says so, its own activation, cost, cooldown, granted tags and movement changes (Banzai speed, Shield slow), and the
  cues of those;`
- C5.3, r1:2367-2368 (R1-ABL-REC4, reason):
  - before: `that is an
  inference, and R2 decides the prediction model (OQ4).`
  - after: `that is an
  inference, and R2 decides the prediction model (OQ4): R2-SYNC-REC3 replays the local player from the authoritative
  state, and leaves abilities to ABL-PREDICT.`
- C5.4, r1:2371-2372 (R1-ABL-REC4, spike):
  - before: `the local player's
  ability press changes its own cooldown, cost and speed in the same client tick as the input; a rejected press (for
  example before Tech 1) is undone within one round trip plus one tick;`
  - after: `if ABL-PREDICT says
  yes, the local player's ability press changes its own cooldown, cost and speed in the same client tick as the input,
  and a rejected press (for example before Tech 1) is undone within one round trip plus one tick;`
- C5.5, r1:2683 (R1-ABL-OQ4), at the end of the entry:
  - before: `What decides it: R2's
  recommendation.`
  - after: `What decides it: R2's
  recommendation. R2-SYNC-REC3's model is the replay from the authoritative state (Quake III's method); for abilities
  the question rides with ABL-PREDICT (below).`
- C5.6, r1:2616-2617 (R1-DET-OQ6), at the end of the entry:
  - before: `It sets how much cross-target identity buys (F32: corrections come only from what the client predicts).`
  - after: `It sets how much cross-target identity buys (F32: corrections come only from what the client predicts).
  R2-SYNC-REC3's answer: own movement, own weapon state, own vehicle and cosmetic shot feedback; abilities are
  ABL-PREDICT (below).`
- C5.7: insert ABL-PREDICT.

R2:
- C5.8, r2:2078-2080 (R2-SYNC-REC3), a third item at the end of the list "What the client does not predict":
  - before: `  - the uplink countdown and steals (LOOP_V2 §4): the client shows the server's tick-stamped state.`
  - after: `  - the uplink countdown and steals (LOOP_V2 §4): the client shows the server's tick-stamped state;
  - its own abilities, until ABL-PREDICT (below) says otherwise (option 3.2 D, OQ10).`
- C5.9, r2:2891 (R2-SYNC-OQ10), at the end of the entry:
  - before: `server-confirmed Molotov and Shield, or a readable source.`
  - after: `server-confirmed Molotov and Shield, or a readable source. Shared as ABL-PREDICT (below); the final call is
  Mustafa's (game feel).`
- C5.10: insert ABL-PREDICT.

R3–R6: no change.

**Basis ids:** R1-ABL-REC4, R1-ABL-OQ4, R1-DET-OQ6, R2-SYNC-REC3, R2-SYNC-OQ10, the hinge row at r2:3341. **New
numbers:** none.


#### 6. The spread random stream: in the sim state or server-only

**As approved, 2026-10-30 (advisor, step 3b; `advisor-resolutions-verdict.md`):** APPROVE.

**Outcome: resolved** for the design: R1 adopts R2's server-only spread stream as an authority-only stream, kept out of
the client's state and out of any hash compared with a client (GLOSSARY: shared stream and authority-only stream; state
hash). The tracer-against-cheating trade-off stays open as **SEC-SPREAD (Mustafa)**, Mustafa's existing decision
(R2-SYNC-OQ11); the design holds under both of its answers. Decider: the advisor for the design (the advisor's table:
"Me for the design (R1 adopts R2's server-only stream, kept out of the client's state and hash)").

**Both sides.**
- r1:2137-2139 (R1-DET-REC4): "one stream per purpose (for example match setup, weapon spread, bot decisions, ability
  procs); the RNG state (seed, stream, position) is part of the sim state and its hash"; r1:2342-2343 (R1-ABL-REC3):
  "Random rolls (spread) come from the simulation's seeded RNG".
- r2:2073-2077 (R2-SYNC-REC3, security fix H2): "Muzzle flash and tracer are drawn at once from a cosmetic spread on
  the client, not from a seed it shares with the server. The authoritative spread comes from a server-only random
  stream"; r2:2897-2899 (R2-SYNC-OQ11): "Still Mustafa's call: matching tracers against cheat resistance (the security
  review's decision SEC-SPREAD)."
- Existing question: R2-SYNC-OQ11 (R2's side only).

**Which holds and why.**
1. The security review's finding, which R2 applied: a seed the client knows "would let a cheat cancel the spread
   through the aim point it sends" (R2-SYNC-REC3; R2-SYNC-OQ11, H2), and Source's server seed is "independent of the
   client" (R2-SYNC-F20). Nothing in R1 argues for a shared spread seed; R1-DET-REC4 lists spread as one purpose among
   others and R1-ABL-REC3 only rules out the activation key as a source of randomness.
2. The client's sim is a predicting copy (R1-BND-REC6, R2-SYNC-REC3), and its weapon prediction covers the timer and
   ammo, not the spread (R1-ABL-REC4, R2-SYNC-REC3: tracers come from a cosmetic spread). So it needs no spread stream.
3. "Part of the sim state and its hash" stays true for the authority, whose whole state the determinism suite hashes
   (R1-DET-REC1, REC6), offline matches included (the Worker is the authority there). Only a hash compared between a
   client and the server (R1-DET-OQ9) must leave authority-only streams out, because the client does not hold them.
4. Under SEC-SPREAD's other answer (matching tracers), weapon spread becomes a shared stream: the client holds it and
   the compared hash includes it. Nothing else changes, so the design leaves SEC-SPREAD open.

*Shared question, to insert in R1 and R2 (the "Shared open questions" subsection), word for word:*

> **SEC-SPREAD (Mustafa). Matching tracers, or cheat resistance?** A spread seed the client knows makes its predicted
> tracers match the server's spread, but lets a cheat cancel the spread through the aim point it sends (R2-SYNC-OQ11,
> the security review's finding H2). The design keeps weapon spread in an authority-only stream (GLOSSARY;
> R1-DET-REC4, R2-SYNC-REC3): the client draws its tracers from a cosmetic spread and the hit event corrects them. If
> Mustafa picks matching tracers, spread moves to a shared stream, inside the client's state and its compared hash;
> nothing else in the stream design changes. Also asked as R2-SYNC-OQ11. Decided by: Mustafa.

*Changes per file (before → after).*

R1:
- C6.1, r1:2139 (R1-DET-REC4):
  - before: `the RNG state (seed, stream, position) is part of the sim state and its hash.`
  - after: `the RNG state (seed, stream, position) is part of the authority's sim state and its state hash (GLOSSARY: the
  authority; state hash). Streams are shared or authority-only (GLOSSARY): an authority-only stream is held by the
  server online and by the Worker in an offline match, and stays out of the client's predicting state and out of any
  hash compared with a client. Weapon spread is authority-only (R2-SYNC-REC3, the security review's finding H2: a seed
  the client knows would let a cheat cancel the spread through its aim point); whether it becomes shared so tracers
  match is SEC-SPREAD (below), Mustafa's call.`
- C6.2, r1:2342-2343 (R1-ABL-REC3):
  - before: `Random rolls (spread) come from the
  simulation's seeded RNG, never from the activation key alone`
  - after: `Random rolls come from the
  simulation's seeded RNG streams (spread from an authority-only stream, R1-DET-REC4; SEC-SPREAD), never from the
  activation key alone`
- C6.3, r1:2624 (R1-DET-OQ9), at the end of the entry:
  - before: `R2 decides (it is protocol); this unit only needs a fixed, unseeded hash in tests.`
  - after: `R2 decides (it is protocol); this unit only needs a fixed, unseeded hash in tests. A hash compared with a
  client covers only the state the client holds, so it leaves authority-only streams out (R1-DET-REC4; GLOSSARY: state
  hash).`
- C6.4: insert SEC-SPREAD.

R2:
- C6.5, r2:2074 (R2-SYNC-REC3):
  - before: `The authoritative spread comes from a server-only random stream,`
  - after: `The authoritative spread comes from an authority-only random stream (GLOSSARY; the server online, the Worker in
  an offline match; R1-DET-REC4),`
- C6.6, r2:2898-2899 (R2-SYNC-OQ11), at the end of the entry:
  - before: `Still Mustafa's call: matching tracers against cheat
  resistance (the security review's decision SEC-SPREAD).]`
  - after: `Still Mustafa's call: matching tracers against cheat
  resistance (the security review's decision SEC-SPREAD).] R1-DET-REC4 now keeps spread in an authority-only stream
  (GLOSSARY). Shared as SEC-SPREAD (below).`
- C6.7: insert SEC-SPREAD.

R3–R6: no change.

**Basis ids:** R1-DET-REC4, R1-ABL-REC3, R1-ABL-REC4, R1-DET-REC1, R1-DET-REC6, R1-DET-OQ9, R1-BND-REC6,
R2-SYNC-REC3, R2-SYNC-F20, R2-SYNC-OQ11. **New numbers:** none.


#### 9. The stress scene

**As approved, 2026-10-30 (advisor, step 3b; `advisor-resolutions-verdict.md`):** REVISE, folded (wording only; the scene is approved).

**Outcome: resolved** (one proposed worst-case scene; the game-designer's caps replace its counts later, R2-BUD-OQ5).
Decider: the advisor (the advisor's table: "Me: one proposed worst-case scene. Caps later (R2-BUD-OQ5)").

**Both sides.**
- r1:1907-1908 (R1-ECS-REC1 mark 1): "48 soldiers (bots), 8 vehicles, 300 live projectiles and 200 active effects";
  r4:765-767 (R4-PHYS-REC4): "keeps 8 big vehicles (R1-ECS-REC1's number), harvesters included"; r4:839-840
  (R4-NAV-REC2): "with 8 big vehicles instead of P1's 4"; r4:1000 (R4-OBST-REC1): "48 bots, 8 big vehicles including
  harvesters".
- r1:2013-2014 (R1-TICK-REC2 P1): "plus 4 big vehicles and 200 live projectiles"; r3:1110-1111 (R3-BUDGET-REC2): "48
  soldiers on screen in one fight, 4 large units, 200 live projectiles and their effects (R1-TICK's scene)"; r3:1079
  (R3-CHAR-REC4) and r3:1187 (R3-MEASURE-REC2) the same.
- r2:1757-1758 (worked estimate): projectiles "32 at 48 players", vehicles "one per squad of 4 (LOOP_V2 §2) + one
  harvester per team (GE6)" (15 at 48, r2:1771); r2:2173 (R2-SYNC-REC5 mark 3): "48 soldiers plus 12 vehicles and
  harvesters".
- r1:2395 (R1-ABL-REC6): "48 soldiers … each with up to 8 active effects"; r1:2133-2134 (R1-DET-REC3): "48 characters
  plus the vehicles VISION_V2 §6.4 implies, count set by R4".
- Existing questions: R1-XU-OQ1 (R1's units only) and R2-BUD-OQ5 (the caps).

**The scene (GLOSSARY: the stress scene), each count with its basis.**

| Part | Count | Basis | Why this one |
|---|---|---|---|
| Soldiers | the largest mode's N (48 or 54, MODES-3), bots with full AI, in three teams, all in one combat cluster; scale steps N = 12, 24, 36, 48 and the N = 64 headroom run where a mark asks | R1-TICK-REC2 P1 (cluster, scale steps); resolution 10 | three teams give more enemy pairs at the same N (arithmetic below), and R2-INT-REC3 bounds the visibility cost by the enemy-pair count (R2-INT-F20); R2's vehicle count is the three-team count |
| Big vehicles | 15 in play, harvesters included | R2's worked estimate at 48 (r2:1758, r2:1771) | the largest count any topic uses at 48, so no mark runs on fewer vehicles than another topic assumes; it comes from R2's stated assumption (one vehicle per squad of 4, LOOP_V2 §2's squad size, plus one harvester per team, GE6; R2 labels the row "assumption", r2:1758), where 4 and 8 come with no rule (R1-ECS-REC1: "proposed, not a design target") |
| of which very large units | 4, each at least 20 parts | R1-TICK-REC2 P1, R3-CHAR-REC4; VISION_V2 §6.9 ("a few very large units") | R3's draw-call arithmetic rests on 4 (r3:1134-1136); the part counts of the other 11 are in no file |
| Live projectiles | 300 | R1-ECS-REC1 | the largest stress count (against 200 in R1-TICK-REC2 and R3); R2's 32 is its bandwidth assumption, not a stress count |
| Active effects | up to 8 per soldier: N × 8 | R1-ABL-REC6 | the effect unit's own load, and the larger one (384 at 48 against R1-ECS-REC1's 200) |
| Where | soldiers, the 4 very large units and the live projectiles in the one fight (on screen for R3's marks); the other 11 big vehicles move elsewhere on the same map | VISION_V2 §6.9; R3-BUDGET-REC2 | keeps every render budget's arithmetic valid while the sim still carries all 15 |
| Map | one 3D greybox shared by every mark, R1-TICK-REC2's "DM_Tri-sized greybox" | R1-TICK-REC2 | one map so results compare |

*Arithmetic.* 48 players in 3 teams with squads of 4: 48 / 4 = 12 squads, plus 3 harvesters = 15 (R2's row at
r2:1771). Enemy pairs at N = 48: 768 at 16v16v16 against 576 at 24v24 (R2-INT-F20, r2:863, gives both). At
N = 54 (18v18v18): 3 × 18 × 18 = 972. Active effects: 48 × 8 = 384; 54 × 8 = 432. Under MODES-3 (b), R2's rule gives no whole number
(18 / 4 = 4.5 squads per team), so 15 big vehicles is kept (proposed) under both readings until the caps.

*Holds under every option of MODES-3:* only N and its three-team arrangement change (16v16v16 or 18v18v18); the other
counts do not.

*Alternatives weighed (for the advisor).* (i) All 15 big vehicles in the fight on screen: a harder render case, but
R3's draw-call arithmetic would need part counts that no file gives. (ii) R1-ECS-REC1's 8 vehicles: fewer than R2's
estimate assumes, so R2's history mark (R2-SYNC-REC5 mark 3) would run on a larger scene than the CPU marks.

*Consequences.* R3-BUDGET-REC2 moves from 200 to 300 projectiles (a recommendation change in R3, logged by its owner);
R2-SYNC-REC5 mark 3's history now holds 15 big vehicles plus N soldiers (its 1 MB mark is unchanged; R2's own estimate is
"about 0.1 MB at 60 Hz for 60 movers", inferred, against 63 movers at N = 48). Under MODES-3 (b), R3's Low draw-call
arithmetic exceeds its 800 cap (C9.13 below), which R3-BUDGET-OQ4 already leaves open.

*Changes per file (before → after).*

R1:
- C9.1, r1:1907-1908 (R1-ECS-REC1 mark 1):
  - before: `Stress load (proposed, not a design target; the game-designer may change it): 48 soldiers (bots),
     8 vehicles, 300 live projectiles and 200 active effects, on a 3D greybox.`
  - after: `The stress scene (GLOSSARY; proposed, not a design target; the game-designer's caps replace its counts,
     R2-BUD-OQ5): the largest mode's N soldiers (bots), 15 big vehicles, 300 live projectiles and up to 8 active
     effects per soldier, on a 3D greybox.`
- C9.2, r1:1914-1915 (R1-ECS-REC1 mark 2):
  - before: `at the
     stress load.`
  - after: `in the
     stress scene.`
- C9.3, r1:2013-2014 (R1-TICK-REC2 P1):
  - before: `N = 12 / 24 / 36 / 48 / 64 soldiers (bots with full AI, worst case: all in one combat cluster),
  plus 4 big vehicles and 200 live projectiles, on a DM_Tri-sized greybox`
  - after: `the stress scene (GLOSSARY) at N = 12 / 24 / 36 / 48 / 64 soldiers and at the largest mode's N where it is
  not among them (MODES-3) (bots with full AI, worst case: all in one combat cluster), with its 15 big vehicles, 300
  live projectiles and up to 8 active effects per soldier, on a DM_Tri-sized greybox`
- C9.4, r1:2015 (R1-TICK-REC2 P1; the same words recur in R1-XU-OQ1 at r1:2752-2753, which C9.8 settles and which
  keeps its quote of the old mark):
  - before: `tick time at N = 48 at most`
  - after: `tick time at the largest mode's N at most`
- C9.5, r1:2036 (R1-TICK-REC3, spike):
  - before: `at N = 48 in P1's scene`
  - after: `at the largest mode's N in the stress scene (GLOSSARY)`
- C9.6, r1:2133-2134 (R1-DET-REC3, spike):
  - before: `a physics scene of the largest mode's size (48 characters plus the
  vehicles VISION_V2 §6.4 implies, count set by R4)`
  - after: `the stress scene (GLOSSARY: the largest mode's N soldiers and 15 big vehicles, the count R4-PHYS-REC4
  takes up)`
- C9.7, r1:2395-2396 (R1-ABL-REC6):
  - before: `for 48 soldiers (VISION_V2 §3, 24v24), each with up to 8 active effects (a
  proposed load)`
  - after: `in the stress scene (GLOSSARY: the largest mode's N soldiers, VISION_V2 §3, each with up to 8 active
  effects, a proposed load)`
- C9.8, r1:2759-2760 (R1-XU-OQ1), at the end of the entry:
  - before: `What decides it: the synthesis picks one stress scene and one split of the tick budget; until then every figure
  stays as each unit proposed it.`
  - after: `What decides it: the synthesis picks one stress scene and one split of the tick budget; until then every figure
  stays as each unit proposed it. Settled by the shared glossary (2026-10-30): one stress scene and one server tick
  budget with its shares (GLOSSARY), which the marks above now use.`

R2:
- C9.9, r2:2173 (R2-SYNC-REC5 mark 3):
  - before: `The history for 48 soldiers plus 12 vehicles and harvesters stays at or under 1 MB.`
  - after: `The history for the stress scene's soldiers and big vehicles (GLOSSARY: the largest mode's N and 15,
  harvesters included) stays at or under 1 MB.`
- C9.10, r2:2464 (R2-INT-REC4, spike):
  - before: `in the worst case (48 players plus vehicles and projectiles in the centre field),`
  - after: `in the stress scene (GLOSSARY), with its fight in the centre field,`
- C9.11, r2:2934-2935 (R2-BUD-OQ5), at the end of the entry:
  - before: `Decides: game-designer's caps; the spike then tests the cap as the worst
  case.`
  - after: `Decides: game-designer's caps; the spike then tests the cap as the worst
  case. Until then every topic's stress marks use one proposed scene (GLOSSARY: the stress scene; 15 big vehicles,
  300 live projectiles); the estimate's 32 projectiles stay its own bandwidth assumption.`

R3:
- C9.12, r3:1079-1080 (R3-CHAR-REC4, spike):
  - before: `four large units (each at least 20 parts; the count of R3-BUDGET-REC2's and R1-TICK's
scene) on screen with 48 soldiers`
  - after: `the stress scene's four very large units (GLOSSARY; each at least 20 parts) on screen with its soldiers`
- C9.13, r3:1136 (R3-BUDGET-REC2, the arithmetic):
  - before: `four large units up to 200 (R3-CHAR-REC4): about 786, inside 800.`
  - after: `four very large units up to 200 (R3-CHAR-REC4): about 786, inside 800, at N = 48 (at N = 54, MODES-3 (b),
  the soldiers alone take 54 × 4 × 2 = 432 draws and the sum is about 834, over 800: R3-BUDGET-OQ4).`
- C9.14, r3:1110-1111 (R3-BUDGET-REC2, heading):
  - before: `measured in the worst case of the largest mode: 48 soldiers on screen
in one fight, 4 large units, 200 live projectiles and their effects (R1-TICK's scene), on the reference machines.`
  - after: `measured in the stress scene's fight on screen (GLOSSARY): the largest mode's N soldiers
in one fight, its 4 very large units and its 300 live projectiles with their visual effects, on the reference machines.`
- C9.15, r3:1187 (R3-MEASURE-REC2, heading):
  - before: `largest mode's worst case (R3-BUDGET-REC2: 48 soldiers in one fight, 4 large units, 200 projectiles)`
  - after: `stress scene (GLOSSARY; R3-BUDGET-REC2's fight on screen)`
- C9.16, r3:1189-1190 (R3-MEASURE-REC2, reason):
  - before: `R1-TICK's scale
scene give the load`
  - after: `the stress scene
(GLOSSARY) give the load`

R4:
- C9.17, r4:691 (R4-PHYS-REC1 mark 2):
  - before: `At R1-ECS-REC1 mark 1's stress load,`
  - after: `In the stress scene (GLOSSARY),`
- C9.18, r4:765-767 (R4-PHYS-REC4, spike):
  - before: `the stress load for R1-DET-REC3 and
  R1-ECS-REC1 mark 1 keeps 8 big vehicles (R1-ECS-REC1's number), harvesters included (LOOP_V2 §3; how many per team
  is the game-designer's), at 30 and 60 Hz.`
  - after: `the stress scene (GLOSSARY), which R1-DET-REC3 and
  R1-ECS-REC1 mark 1 use, holds 15 big vehicles, harvesters included (LOOP_V2 §3; how many per team
  is the game-designer's, R2-BUD-OQ5), at 30 and 60 Hz.`
- C9.19, r4:839-840 (R4-NAV-REC2, spike):
  - before: `in R1-TICK-REC2's P1 scene at N = 48 and N = 64 bots, with 8 big vehicles instead of
  P1's 4 (the count R4-PHYS-REC4 proposes; the one stress scene is R1-XU-OQ1's, for the synthesis),`
  - after: `in the stress scene (GLOSSARY) at the largest mode's N and in its N = 64 headroom run,`
- C9.20, r4:1000 (R4-OBST-REC1, spike):
  - before: `R4-PHYS-REC4's stress scene (48 bots, 8 big vehicles including harvesters)`
  - after: `the stress scene (GLOSSARY: the largest mode's N bots, 15 big vehicles including harvesters)`
- C9.21, r4:1115-1116 (wave-1 list):
  - before: `its spike scene stands, with the
  vehicle count it asked R4 for: 8 big vehicles, harvesters included (R4-PHYS-REC4).`
  - after: `its spike scene is now
  the stress scene (GLOSSARY), whose 15 big vehicles include harvesters (R4-PHYS-REC4).`
- C9.22, r4:1117-1118 (wave-1 list):
  - before: `- R1-TICK-REC2's P1 scene has 4 big vehicles; R4 proposes 8 there too (R4-PHYS-REC4), so R4-NAV-REC2's mark runs P1
  with 8. The one stress scene stays R1-XU-OQ1's, for the synthesis.`
  - after: `- R1-TICK-REC2's P1 scene had 4 big vehicles and R4 proposed 8 (R4-PHYS-REC4); R1, R3 and R4 now share one
  stress scene with 15 (GLOSSARY), which settles the scene part of R1-XU-OQ1.`

R5, R6: no change (no stress counts).

**Basis ids:** R1-ECS-REC1, R1-TICK-REC2, R1-ABL-REC6, R1-DET-REC3, R1-XU-OQ1, R2 worked estimate (r2:1757-1758,
r2:1771), R2-SYNC-REC5, R2-INT-REC3, R2-INT-F20, R2-INT-REC4, R2-BUD-OQ5, R3-BUDGET-REC2, R3-CHAR-REC4,
R3-MEASURE-REC2, R4-PHYS-REC1, R4-PHYS-REC4, R4-NAV-REC2, R4-OBST-REC1; VISION_V2 §6.9. **New numbers:** 15, 300,
8 per soldier and 4 are taken from the files (ids above); 384, 432, 768, 576, 972, 432 draws and 834 are arithmetic
(shown); the scene as a whole is proposed.


#### 10. Mode sizes: totals or per team

**As approved, 2026-10-30 (advisor, step 3b; `advisor-resolutions-verdict.md`):** ESCALATE (MODES-3, Mustafa); the interim wording C10.1–C10.23 is approved.

**Outcome: escalated. Decision code MODES-3; decider: Mustafa** (the advisor's table: "Mustafa now (MODES-3 below)").

**Both sides.**
- Totals: r1:2587-2588 (R1-TICK-OQ7) "the largest mode (48 soldiers, 47 of them bots)"; r1:2098-2099 (R1-DET-REC1) "a
  bots-only 24v24 match (48 players) and a 12v12v12 match (36 players)"; r5:1471 (R5-TEST-REC5) "24v24 = 48 and
  12v12v12 = 36"; r6:442 "6v6v6 (start, three teams, upper end) | 18"; r2:1768-1769 (R2's worked estimate) "Two teams,
  4v4 (VISION_V2 §3 start, upper end)" and "Three teams, 6v6v6 (start, upper end)".
- Per team: r2:2820 (R2-ROOM-REC7) "mode files for 2×2, 2×8, 3×6, 3×18, 12v12v12 and 24v24" (3×18 = 54); r2:2809
  (R2-ROOM-REC6) "3 teams × 16 slots".
- Three-team test sizes: r2:2447 (R2-INT-REC3), r4:954 (R4-LOS-REC1 mark 4) and r4:969 (R4-LOS-REC2) "24v24 and
  16v16v16".
- Existing questions: R2-BUD-OQ6 ("totals or per team? … Decides: Mustafa"), R2-XU-OQ13, R3-BUDGET-OQ4.

**Why it is escalated.** The topics' findings cannot settle how VISION_V2 §3 is read; only its owner can (R2-BUD-OQ6
already names Mustafa). It sets the largest mode's N, and with it the stress scene (resolution 9), the offline scale
test (R1-TICK-OQ7), the soak runs (R5-TEST-REC5) and the cost rows (R6).

**The question for Mustafa (MODES-3).** VISION_V2 §3 says "two teams with 2–8 players, and three teams with 6–18
players" to start. Are these totals or sizes per team?
- (a) Totals: 1v1 to 4v4, and 2v2v2 to 6v6v6. The largest mode is then the goal 24v24, N = 48; the three-team goal
  12v12v12 has 36.
- (b) Per team: 2v2 to 8v8, and 6v6v6 to 18v18v18. The largest mode is then 18v18v18, N = 54.
- **Recommendation: (a).** Under (b) the "start small" modes would be larger (54) than both goals (48 and 36), and 54
  is "above the 48 design point" (R2-BUD-OQ6). A run above 48 stays possible as a labelled headroom run (the design
  point and headroom, GLOSSARY), as R1-TICK-REC2 P1 and R4-NAV-REC2 already run N = 64. This is also the advisor's
  recommendation.

**What changes now, under every option** (basis: VISION_V2 §3; R2-BUD-OQ6; LOOP_V2 §2 as quoted at r1:311 and r2:466,
"12v12v12 to 16v16v16, possibly more"). The topics stop fixing N at 48 where they mean the largest mode, name the
test-mode list (GLOSSARY) instead of their own lists, and carry one shared question.

*Arithmetic for the lists:* under (a), "2–8 players" in two teams is 1 to 4 per team and "6–18 players" in three teams
is 2 to 6 per team; under (b), 2 to 8 and 6 to 18 per team; 3 × 18 = 54; 16v16v16 = 3 × 16 = 48.

*Shared question, to insert in R1, R2, R3, R4, R5 and R6 (the "Shared open questions" subsection), word for word:*

> **MODES-3 (Mustafa). How are VISION_V2 §3's starting sizes read?** VISION_V2 §3 says "two teams with 2–8 players,
> and three teams with 6–18 players" to start, and "The goal: 12v12v12 or 24v24". (a) Totals: 1v1 to 4v4, and 2v2v2
> to 6v6v6; the largest mode is then the goal 24v24, N = 48. (b) Per team: 2v2 to 8v8, and 6v6v6 to 18v18v18; the
> largest mode is then 18v18v18, N = 54. The answer sets the largest mode's N (GLOSSARY), which the stress scene, the
> offline scale test (R1-TICK-OQ7), the soak runs (R5-TEST-REC5) and the cost rows (R6-COST) use, and items 3 and 4 of
> the test-mode list (GLOSSARY). Until it is answered, a mark that needs the largest mode's N gives both values. Also
> asked as R2-BUD-OQ6, R2-XU-OQ13 and R3-BUDGET-OQ4. Decided by: Mustafa.

*Changes per file (before → after; text outside the quote unchanged).*

R1:
- C10.1, r1:2098-2099 (R1-DET-REC1, spike):
  - before: `(a) a bots-only 24v24 match (48 players) and a 12v12v12 match (36 players), the two
  goals of VISION_V2 §3, each run 10 minutes`
  - after: `(a) a bots-only 24v24 match (48 players) and a 12v12v12 match (36 players), the two
  goals of VISION_V2 §3, plus the largest mode where it is neither (GLOSSARY; MODES-3: 18v18v18 under reading (b)),
  each run 10 minutes`
- C10.2, r1:2588 (R1-TICK-OQ7):
  - before: `does not say whether the largest mode (48 soldiers, 47 of them bots) must run offline on the minimum laptop.`
  - after: `does not say whether the largest mode (GLOSSARY; 48 soldiers under MODES-3 (a), 54 under (b), all but the
  player bots) must run offline on the minimum laptop.`
- C10.3: insert the shared question MODES-3.

R2:
- C10.4, r2:1768 (worked estimate, results table):
  - before: `| Two teams, 4v4 (VISION_V2 §3 start, upper end) |`
  - after: `| Two teams, 4v4 (VISION_V2 §3 start, upper end, as totals: MODES-3 (a)) |`
- C10.5, r2:1769:
  - before: `| Three teams, 6v6v6 (start, upper end) |`
  - after: `| Three teams, 6v6v6 (start, upper end, as totals: MODES-3 (a)) |`
- C10.6, r2:2447 (R2-INT-REC3, spike):
  - before: `at 24v24 and 16v16v16, all players in one fight`
  - after: `at 24v24 and 16v16v16 (the test-mode list, GLOSSARY) and, under MODES-3 (b), 18v18v18, all players in one
  fight`
- C10.7, r2:2809 (R2-ROOM-REC6, spike):
  - before: `(1) 3 teams × 16 slots, 30 humans`
  - after: `(1) the largest mode's N in three teams (GLOSSARY; 3 teams × 16 slots under MODES-3 (a), 3 × 18 under (b)),
  30 humans`
- C10.8, r2:2820 (R2-ROOM-REC7, spike):
  - before: `(1) mode files for 2×2, 2×8, 3×6, 3×18, 12v12v12 and 24v24 load`
  - after: `(1) mode files for every mode of the test-mode list (GLOSSARY; under MODES-3 (a): 1v1, 4v4, 2v2v2, 6v6v6,
  12v12v12, 16v16v16 and 24v24; under (b): 2v2, 8v8, 6v6v6, 18v18v18, 12v12v12, 16v16v16 and 24v24; plus the
  N = 64 headroom mode) load`
- C10.9, r2:2938 (R2-BUD-OQ6), at the end of the entry:
  - before: `point but inside the estimate's 64-player row. Decides: Mustafa.`
  - after: `point but inside the estimate's 64-player row. Decides: Mustafa. Shared as MODES-3 (below).`
- C10.10, r2:3173 (R2-XU-OQ13), at the end of the entry:
  - before: `(R2-ROOM-REC6). Decided by: Mustafa (R2-BUD-OQ6).`
  - after: `(R2-ROOM-REC6). Decided by: Mustafa (R2-BUD-OQ6). Shared as MODES-3 (below).`
- C10.11: insert the shared question MODES-3.

R3:
- C10.12, r3:1360-1361 (R3-BUDGET-OQ4), at the end of the entry:
  - before: `OQ7 decides the N of the offline scale test. The draw-call and triangle rows scale with N.`
  - after: `OQ7 decides the N of the offline scale test. The draw-call and triangle rows scale with N. The largest mode's
  N (GLOSSARY) is 48 or 54 by the answer to MODES-3 (below).`
- C10.13: insert the shared question MODES-3.

R4:
- C10.14, r4:954 (R4-LOS-REC1 mark 4):
  - before: `R2-INT-REC3's full pass (24v24 and 16v16v16, three points)`
  - after: `R2-INT-REC3's full pass (24v24 and 16v16v16 from the test-mode list, GLOSSARY, and 18v18v18 under MODES-3
  (b); three points)`
- C10.15, r4:969 (R4-LOS-REC2, spike):
  - before: `at 24v24 and 16v16v16 with everyone in one fight`
  - after: `at 24v24 and 16v16v16 (the test-mode list, GLOSSARY) and, under MODES-3 (b), 18v18v18, with everyone in one
  fight`
- C10.16: insert the shared question MODES-3.

R5:
- C10.17, r5:1471 (R5-TEST-REC5):
  - before: `(24v24 = 48 and 12v12v12 = 36, plus LOOP_V2 §11's N = 12, 24, 36, 48 scale steps)`
  - after: `(24v24 = 48 and 12v12v12 = 36, plus the largest mode where it is neither (GLOSSARY; MODES-3: 18v18v18 = 54
  under reading (b)), plus LOOP_V2 §11's N = 12, 24, 36, 48 scale steps)`
- C10.18: insert the shared question MODES-3.

R6:
- C10.19, r6:441:
  - before: `| 4v4 (start, two teams, upper end) |`
  - after: `| 4v4 (start, two teams, upper end, as totals: MODES-3 (a)) |`
- C10.20, r6:442:
  - before: `| 6v6v6 (start, three teams, upper end) |`
  - after: `| 6v6v6 (start, three teams, upper end, as totals: MODES-3 (a)) |`
- C10.21, r6:447, after the table's closing paragraph:
  - before: `client (R2). Ingress is free on Hetzner (R6-HOST-F17).`
  - after: `client (R2). Ingress is free on Hetzner (R6-HOST-F17). Under MODES-3 (b) the start modes' upper ends are 8v8
  (16 players) and 18v18v18 (54); their rows follow from R2's formula (r2:1760-1762) once Mustafa answers.`
- C10.22, r6:464 (Start scenario):
  - before: `**Start (VISION_V2 §3: two teams of 2–8, three teams of 6–18).**`
  - after: `**Start (VISION_V2 §3: two teams of 2–8, three teams of 6–18, read as totals: MODES-3 (a); under (b) a start
  room holds up to 54).**`
- C10.23: insert the shared question MODES-3.

**Basis ids:** R2-BUD-OQ6, R2-XU-OQ13, R3-BUDGET-OQ4, R1-TICK-OQ7, R2-ROOM-REC6, R2-ROOM-REC7, R2-INT-REC3,
R4-LOS-REC1, R4-LOS-REC2, R1-DET-REC1, R5-TEST-REC5, R6 egress table; VISION_V2 §3; LOOP_V2 §2 as quoted at r1:311 and
r2:466. **New numbers:** 54 (3 × 18, arithmetic), per-team ranges under (a) (arithmetic above); no other.
**Changes that depend on this item elsewhere:** resolution 9 (the stress scene's N) and resolution 1 (P1's N).


#### 11. One core or one vCPU as the measuring unit

**As approved, 2026-10-30 (advisor, step 3b; `advisor-resolutions-verdict.md`):** APPROVE.

**Outcome: resolved.** The reference server is the CCX23 that R6 names, and every server time mark is measured on one
vCPU of it (GLOSSARY: the reference server; the measuring unit). Decider: the advisor (the advisor's table: "Me").

**Both sides.**
- One core: r1:1905-1906 (R1-ECS-REC1) "reference hardware proposed: one server core of the R6 host type"; r1:2015
  (R1-TICK-REC2) "on one core of the proposed reference server"; r1:2396 (R1-ABL-REC6) "on the proposed reference
  server core"; r4:658 "one core of the R6 server type"; r1:2545 (R1-ECS-OQ4) "one match per core".
- One vCPU: r2:2385 (R2-BUD-REC6) "one vCPU of the EU instance type R6 recommends"; r2:2732 (R2-ROOM-REC2) "one 4-vCPU
  EU cloud VM"; r6:628-629 (R6-HOST-REC2) "the dedicated CCX23 (4 vCPU, 16 GB, x86-64 AMD)"; r6:942-943 "'one vCPU of
  the EU instance type' in R2-BUD-REC6 is one vCPU of CCX23".
- Existing questions: R2-XU-OQ9 (answered by R6 for R2 only) and R1-TICK-OQ6.

**Which holds and why: one vCPU of the CCX23.**
1. R6, the topic that owns hosting, names the machine (R6-HOST-REC2) and already states R2's unit in it (r6:942-943).
2. The CCX23 is sold, billed and sized in vCPUs (R6-HOST-F15), and R6's density and costs are per vCPU (R6-COST A-2;
   the per-room-hour rows at r6:452-454, one of which already reads R1-ECS-OQ4's "one per core" as "one per vCPU",
   r6:453). A mark in vCPUs turns straight into rooms per VM and cost per room-hour.
3. "One core" is defined for the reference server nowhere in the six files, and the two units can differ: AWS lists a
   "c6a.large (1 core, 2 vCPU, 4 GB)" (R6-HOST-F23). Measuring R1 and R4 on a "core" and R2 and R6 on a "vCPU" could
   differ by that factor.
4. It holds whatever MODES-3 decides: the unit does not depend on N.

R6-HOST-REC2 stays conditional on R6-HOST-OQ1 (Hetzner's EU capacity), and Mustafa's approval of the reference hardware
(R1-TICK-OQ6) stays as it is. Nothing here changes a number: only the unit's name.

*Changes per file (before → after).*

R1:
- C11.1, r1:1905-1906 (R1-ECS-REC1):
  - before: `reference hardware proposed: one server core of the R6 host type, and the
  VISION_V2 §4 minimum laptop for Wasm`
  - after: `reference hardware proposed: one vCPU of the reference server (GLOSSARY: the CCX23 of R6-HOST-REC2), and the
  VISION_V2 §4 minimum laptop for Wasm`
- C11.2, r1:2012 (R1-TICK-REC2):
  - before: `reference hardware proposed, to be named with R3, R5 and R6`
  - after: `reference hardware proposed: the reference server (GLOSSARY: R6-HOST-REC2's CCX23) and R3-BUDGET-REC1's
  minimum laptop`
- C11.3, r1:2015 (R1-TICK-REC2 P1):
  - before: `on one core of the proposed reference server`
  - after: `on one vCPU of the reference server`
- C11.4, r1:2396 (R1-ABL-REC6):
  - before: `on the proposed reference server core`
  - after: `on one vCPU of the reference server (GLOSSARY)`
- C11.5, r1:2545 (R1-ECS-OQ4, heading):
  - before: `or is one match per core enough?`
  - after: `or is one match per vCPU of the reference server enough?`
- C11.6, r1:2584-2585 (R1-TICK-OQ6):
  - before: `P2 and P3 need a named minimum laptop CPU and P1 a named server core. Decided by: R3, R5 and R6
proposals, approved by Mustafa.`
  - after: `P2 and P3 need a named minimum laptop CPU and P1 a named server. R6 proposes the server, one vCPU of a CCX23
(R6-HOST-REC2; GLOSSARY: the reference server), and R3 the laptop class (R3-BUDGET-REC1). Decided by: those proposals,
approved by Mustafa.`
- C11.7, r1:2955-2956 (pointers to R6):
  - before: `cost per match if one match runs per core (R1-ECS-OQ4)`
  - after: `cost per match if one match runs per vCPU (R1-ECS-OQ4)`
  - before: `and matches per core at 30 Hz against 60 Hz`
  - after: `and matches per vCPU at 30 Hz against 60 Hz`

R2:
- C11.8, r2:2385 (R2-BUD-REC6):
  - before: `on the reference server (proposed: one vCPU of the EU instance type R6 recommends)`
  - after: `on one vCPU of the reference server (GLOSSARY: the CCX23 of R6-HOST-REC2)`
- C11.9, r2:2558-2559 (R2-TRN-REC1 mark 2):
  - before: `most 1 CPU core for the transport layer, and p99 server send-call time ≤ 1 ms, on reference server hardware R6 names
   (proposed).`
  - after: `most 1 vCPU for the transport layer, and p99 server send-call time ≤ 1 ms, on the reference server (GLOSSARY:
   R6-HOST-REC2's CCX23) (proposed).`
- C11.10, r2:2732 (R2-ROOM-REC2):
  - before: `reference host proposed: one 4-vCPU EU cloud VM, R6 to name it`
  - after: `reference host: the reference server (GLOSSARY: the 4-vCPU CCX23 of R6-HOST-REC2)`
- C11.11, r2:3145-3146 (R2-XU-OQ9), at the end of the entry:
  - before: `Decided by: R6's instance choice; the synthesis
  names one reference machine.`
  - after: `Decided by: R6's instance choice; the synthesis
  names one reference machine. Answered by R6-HOST-REC2: the reference server is a CCX23, and every server mark is
  measured on one of its vCPUs (GLOSSARY).`
- C11.12, the words "per core" in the budgets options table E (r2:1721, "several rooms per core") and the pointers
  between units (r2:3244, "rooms per core"): "core" → "vCPU" in each, nothing else. (R2-BUD-REC6's reason, r2:2380,
  "more than one room per core", is rewritten whole by resolution 1, change C1.9; if resolution 1 is not approved,
  "core" → "vCPU" there too.)

R4:
- C11.13, r4:658:
  - before: `(one core of the R6 server type; the
VISION_V2 §4 minimum laptop for Wasm)`
  - after: `(one vCPU of the reference server, GLOSSARY: R6-HOST-REC2's CCX23; the
VISION_V2 §4 minimum laptop for Wasm)`

R6: no change; R6 already uses the term. R6-HOST-REC2 gains the glossary link at its first use.

**Basis ids:** R6-HOST-REC2, R6-HOST-F15, R6-HOST-F23, R6-COST A-2 and the cost rows (r6:452-454), the pointer
r6:942-943, R2-XU-OQ9, R1-TICK-OQ6, R6-HOST-OQ1. **New numbers:** none.


#### 17. Who owns the dependency audit in CI

**As approved, 2026-10-30 (advisor, step 3b; `advisor-resolutions-verdict.md`):** ESCALATE (BASELINE-WHEN, the Advisor); interim C17.1–C17.9 approved; C17.b1–b3 apply only if the Advisor picks (b).

**Outcome: escalated. Decision code BASELINE-WHEN; decider: the Advisor** (relayed by Mustafa; the advisor's table:
"The Advisor (BASELINE-WHEN below)"). Resolution 23 turns on the same code.

**Both sides.**
- r1:2869-2870 (R1-RR-OQ7): "lockfile floors, and cargo-audit and cargo-deny as CI gates. Decides it: R5's CI research
  and the synthesis."; r5:1658-1660 (R5-GATE-REC1, scheduled): "cargo-deny advisories (F3: a new advisory must not turn
  an unrelated PR red; a PR that changes `Cargo.lock` runs the advisories check too)".
- r2:2577-2578 (R2-TRN-REC1 mark 5): "An audit gate that keeps these current belongs to R6's security baseline
  (dependency audits), deferred by the Advisor."; r6:898-900 (R6-SEC-OQ1): the audit and "the rust-reviewer's
  cargo-audit and cargo-deny gates" are deferred.
- Existing questions: R5-GATE-OQ4 ("Decides it: security-auditor with R6's security baseline") and R6-SEC-OQ1.

**Why it is escalated.** The mechanism is designed (R5-GATE-REC1), but who owns the policy, and whether an advisory
blocks a PR, is the security baseline's, which the Advisor deferred (R6-SEC-OQ1). Whether that baseline is researched
before the drafts is the Advisor's call, not an engineering one.

**The question for the Advisor (BASELINE-WHEN).** Research the deferred security baseline before ADR 0006, or list it
open there?
- (a) Research it now: one security-auditor topic, before the drafts.
- (b) List it open in ADR 0006, with interim positions: R5-GATE-REC1's scheduled advisories check stays the mechanism
  and the baseline sets the policy later; R2-TRN-REC5's certificate per game-server hostname stays, and R6's wildcard
  becomes an option the baseline weighs (resolution 23).
- (c) Leave both as one shared open question.
- **Recommendation: (b).** It settles the topics' wording now without deciding the baseline: the scheduled check is
  already designed, with its reason (R5-GATE-F3: an advisory can change without our change), and the per-hostname
  certificate is the safer of the two positions (no private key shared across VMs, R6-HOST-REC4's own concern) while
  R6 already names it as its fallback. This is also the advisor's recommendation.

**What changes now, under every option.** Each topic points to the same question, and the factual cross-references the
critic found missing are added (R5's check exists; R2 and R6 did not cite it). No topic's position on policy changes.

*Shared question, to insert in R1, R2, R5 and R6 (the "Shared open questions" subsection), word for word:*

> **BASELINE-WHEN (the Advisor). Research the deferred security baseline before ADR 0006, or list it open there?** The
> security baseline (GLOSSARY: content security policy, dependency audits, anti-cheat basics) was deferred by the
> Advisor on 2026-10-27, and "ADR 0006 lists it as open, or it is researched before 0006 is accepted" (R6-SEC-OQ1). Two
> cross-topic disagreements wait on it: who owns the advisories check (GLOSSARY) and whether an advisory blocks a pull
> request (R1-RR-OQ7, R2-TRN-REC1 mark 5, R5-GATE-REC1, R5-GATE-OQ4, R6-SEC-OQ1), and one wildcard certificate whose
> private key sits on every game VM (R6-HOST-REC4) against a certificate per game-server hostname (R2-TRN-REC5).
> (a) Research it now, as one security-auditor topic before the drafts. (b) List it open in ADR 0006 with interim
> positions: R5-GATE-REC1's scheduled advisories check stays the mechanism and the baseline sets the policy later;
> R2-TRN-REC5's certificate per hostname stays, and R6's wildcard becomes an option the baseline weighs. (c) Leave both
> as one shared open question. Decided by: the Advisor.

*Changes now (before → after).*

R1:
- C17.1, r1:2869-2870 (R1-RR-OQ7), at the end of the entry:
  - before: `Decides it: R5's CI research and the synthesis.`
  - after: `Decides it: R5's CI research and the synthesis. R5-GATE-REC1 schedules a cargo-deny advisories check and runs
  it on every PR that changes `Cargo.lock` (GLOSSARY: the advisories check); whether an advisory blocks a PR is
  R5-GATE-OQ4, left to the security baseline. Shared as BASELINE-WHEN (below).`
- C17.2: insert BASELINE-WHEN.

R2:
- C17.3, r2:2577-2578 (R2-TRN-REC1 mark 5):
  - before: `An audit gate that keeps these current belongs to R6's security baseline
   (dependency audits), deferred by the Advisor.`
  - after: `An audit gate that keeps these current belongs to R6's security baseline
   (dependency audits), deferred by the Advisor; R5-GATE-REC1 already schedules the advisories check (GLOSSARY), and
   its policy waits on BASELINE-WHEN (below).`
- C17.4: insert BASELINE-WHEN.

R5:
- C17.5, r5:1855-1856 (R5-GATE-OQ4), at the end of the entry:
  - before: `Decides it: security-auditor with R6's
  security baseline.`
  - after: `Decides it: security-auditor with R6's
  security baseline. Shared as BASELINE-WHEN (below).`
- C17.6: insert BASELINE-WHEN.

R6:
- C17.7, r6:895-896 (R6-SEC-OQ1, heading):
  - before: `ADR 0006 lists it as open, or it is researched before 0006 is accepted.**`
  - after: `ADR 0006 lists it as open, or it is researched before 0006 is accepted.** Which one is BASELINE-WHEN (below).`
- C17.8, r6:899-900 (R6-SEC-OQ1, the dependency audits item):
  - before: `the rust-reviewer's cargo-audit and
    cargo-deny gates (R2's list of deferred reviewer findings, item 5: finding R-1).`
  - after: `the rust-reviewer's cargo-audit and
    cargo-deny gates (R2's list of deferred reviewer findings, item 5: finding R-1); the mechanism R5 designs is
    R5-GATE-REC1's scheduled advisories check (R5-GATE-OQ4).`
- C17.9: insert BASELINE-WHEN.

*Changes only if the Advisor picks (b)* (apply after the answer, not before):
- C17.b1, R1-RR-OQ7: "Decides it: R5's CI research and the synthesis." → "Decided (BASELINE-WHEN (b)): the mechanism is
  R5-GATE-REC1's scheduled advisories check, also run on every PR that changes `Cargo.lock`; whether an advisory blocks
  a PR, and who owns that policy, is the security baseline's (R6-SEC-OQ1)."
- C17.b2, R2-TRN-REC1 mark 5: "An audit gate that keeps these current belongs to R6's security baseline (dependency
  audits), deferred by the Advisor." → "The check that keeps these current is R5-GATE-REC1's scheduled advisories check
  (also run when `Cargo.lock` changes); its policy, whether an advisory blocks a PR, belongs to the security baseline
  (dependency audits), deferred by the Advisor (BASELINE-WHEN (b))."
- C17.b3, R5-GATE-OQ4: append "Interim (BASELINE-WHEN (b)): scheduled and on lockfile changes, as R5-GATE-REC1 has it;
  the baseline sets the policy."
- Under (a), the security-auditor's topic replaces these interim texts; under (c), the "now" changes are all.

**Basis ids:** R1-RR-OQ7, R2-TRN-REC1, R2-TRN-F40, R5-GATE-REC1, R5-GATE-F3, R5-GATE-OQ4, R6-SEC-OQ1. **New numbers:**
none.


#### 19. Previews on Cloudflare Workers or Pages

**As approved, 2026-10-30 (advisor, step 3b; `advisor-resolutions-verdict.md`):** REVISE, folded (wording only: C19.3, C19.8; Pages first and PREV-DELETE approved).

**Outcome: resolved** for the host (Cloudflare Pages for the client and the previews, Workers static assets as the
alternative), with one **aligned** spike item attached: **PREV-DELETE** (R5-PREV-OQ2, now shared by R5 and R6). Decider:
the advisor (the advisor's table: "Me. Whether Pages can delete a branch's latest preview (R5-PREV-OQ2) becomes a
shared spike item").

**Both sides.**
- r6:609-610 (R6-HOST-REC1): "Host the client and the per-PR previews on Cloudflare (Workers static assets; Pages as
  the same-headers alternative)".
- r5:1596-1600 (R5-PREV-REC2): "Cloudflare Pages meets every need except full cleanup … Cloudflare Workers static
  assets, Netlify and Vercel meet most"; r5:1601: "F14 shows Workers previews without a stated `noindex`"; r5:1607-1608
  (R5-PREV-REC3): cleanup "(on Pages: `wrangler pages deployment delete … --force`)"; r5:1948-1949: "Please settle
  R5-PREV-OQ2 (Pages deletion of a branch's latest deployment) and R5-PREV-OQ4 (host and price)".
- Existing questions: R5-PREV-OQ2 and R5-PREV-OQ4; R6 cites no R5 id.

**Which holds and why: Pages first, Workers static assets as the alternative.** R5-PREV-REC2's needs list, checked
against the findings both topics hold:

| Need (R5-PREV-REC2) | Cloudflare Pages | Workers static assets |
|---|---|---|
| deploy a prebuilt directory from Actions with a scoped token | yes: wrangler-action, "Pages Edit" token (R5-PREV-F10, F11) | upload command shown (R5-PREV-F14); token scope not read |
| a stable link per PR branch | yes: branch alias (R5-PREV-F12, R6-HOST-F4) | yes: preview named after the branch (R6-HOST-F6, R5-PREV-F14) |
| `noindex` | yes, by default (R5-PREV-F12) | not stated (R5-PREV-F14 "says nothing about static assets or `noindex`") |
| custom headers per hostname | yes: `_headers` rules by hostname pattern (R5-PREV-F18, R6-HOST-F1) | `_headers` exists (R6-HOST-F2); rules per hostname not read |
| an access option | yes: access policy (R5-PREV-F12) | yes: Cloudflare Access (R5-PREV-F14) |
| deletion through a CLI or API | yes, except a branch's latest deployment (R5-PREV-F13) | oldest aliases dropped past 1,000 (R5-PREV-F14); deletion not read |
| limits and price | Free plan: 20,000 files, 25 MiB per asset, unlimited active previews, 500 builds a month (R6-HOST-F3); a price beyond the plan's name not read | static requests and egress free (R6-HOST-F5) |

Pages meets every need on findings already read, all but one deletion case; Workers leaves three needs unread
(`noindex`, rules per hostname, deletion). R6's own reasons (`_headers`, a link per branch, no cap on previews, no
static charge, GitHub Pages ruled out) hold for both products, and the one reason that favours Workers, its "distinctly
broader set of features" (R6-HOST-F6), names no feature any VISION_V2 §6 item needs (R6 budgets Workers Paid "only if
Worker code is needed"). The production client goes on the same host, so a preview and production take the same
`_headers` file. R6-HOST-REC1's 25 MiB mark (F3) is a Pages figure already.

Open points the spike carries (no new claim): PREV-DELETE below; and whether a direct upload counts against Pages'
500 builds a month (R6-HOST-F3 gives the limit; no finding says how direct uploads are counted).

*Aligned spike item, to insert in R5 and R6 (the "Shared open questions" subsection), word for word:*

> **PREV-DELETE (a spike). Does a closed PR's preview really disappear?** On Cloudflare Pages, "The latest deployment
> for a branch cannot be deleted", while `--force` serves "to force deletion of aliased deployments" (R5-PREV-F13), so
> the last link's fate is unclear. On Workers static assets, the alternative host, only "the 1000 most recently deployed
> aliases are retained" (R5-PREV-F14), and an explicit deletion was not read. Decided by: a spike on a test project on
> the client host (GLOSSARY), with R5-PREV-REC3's mark: 24 h after a PR closes, its preview URL no longer serves the
> build (HTTP 404, or the access wall), or the remaining case is written down with the reason. If Pages fails it with
> no workaround, Workers static assets is weighed again (R6-HOST-REC1). Also asked as R5-PREV-OQ2.

*Changes per file (before → after).*

R5:
- C19.1, r5:1599-1600 (R5-PREV-REC2, heading):
  - before: `The choice and its price are R6's.**`
  - after: `R6 hosts the client and the previews on Cloudflare Pages, with Workers static assets as the alternative
  (R6-HOST-REC1; GLOSSARY: the client host); whether a closed PR's preview disappears there is PREV-DELETE (below).**`
- C19.2, r5:1829-1831 (R5-PREV-OQ2), at the end of the entry:
  - before: `Decided by a spike on a test project, or by R6 reading the Pages API docs.`
  - after: `Decided by a spike on a test project, or by R6 reading the Pages API docs. Shared as PREV-DELETE (below).`
- C19.3, r5:1836-1837 (R5-PREV-OQ4), at the end of the entry:
  - before: `This draft gives only the
  technical needs (R5-PREV-REC2).`
  - after: `This draft gives only the
  technical needs (R5-PREV-REC2). Answered by R6-HOST-REC1: Cloudflare Pages, with Workers static assets as the
  alternative; the start budget's line is 0 USD on Pages' Free plan (R6-COST-REC1, R6-HOST-F3; the price is inferred from the plan's name, no Pages price page was read).`
- C19.4: insert PREV-DELETE.

R6:
- C19.5, r6:609-610 (R6-HOST-REC1, heading):
  - before: `Host the client and the per-PR previews on Cloudflare (Workers static assets; Pages as the
  same-headers alternative),`
  - after: `Host the client and the per-PR previews on Cloudflare Pages (GLOSSARY: the client host), with Workers static
  assets as the same-headers alternative,`
- C19.6, r6:615-617 (R6-HOST-REC1, reason):
  - before: `gives every branch its own link with no cap on previews (F3,
  F4, F6), charges nothing for static requests or egress (F5), and GitHub Pages cannot send the headers (F11, F12).`
  - after: `gives every branch its own link with no cap on previews (F3,
  F4, F6), and GitHub Pages cannot send the headers (F11, F12); on Workers, static requests and egress cost nothing
  (F5). Pages comes first because it meets each of R5-PREV-REC2's needs on findings already read (`noindex` by
  default, header rules per hostname, an access policy, deletion through Wrangler: R5-PREV-F12, F13, F18), all but the
  deletion of a branch's latest deployment (PREV-DELETE); for Workers previews `noindex`, rules per hostname and
  deletion were not read (R5-PREV-F14), and Workers' broader features (F6) serve no VISION_V2 §6 item. Whether a
  direct upload counts against Pages' 500 builds a month (F3) is for the spike.`
- C19.7, r6:624-626 (R6-HOST-REC1, spike):
  - before: `(F3: the
  Pages per-asset limit; the Workers static-assets limit was not read, so the mark is rechecked against Workers' own
  limits page before the spike).`
  - after: `(F3: the
  Pages per-asset limit; if the alternative, Workers static assets, is used, the mark is rechecked against Workers'
  own limits page, not read).`
- C19.8, r6:472 (Start scenario, client line):
  - before: `| Client and previews on Cloudflare | 0 USD (free static) |`
  - after: `| Client and previews on Cloudflare Pages | 0 USD (Pages' Free plan, R6-HOST-F3; price inferred from the plan's name) |`
- C19.9: insert PREV-DELETE.

R1–R4: no change.

**Basis ids:** R5-PREV-REC2, R5-PREV-REC3, R5-PREV-F10, F11, F12, F13, F14, F18, R5-PREV-OQ2, R5-PREV-OQ4,
R6-HOST-REC1, R6-HOST-F1–F6, R6-COST's client line (r6:472). **New numbers:** none (500 builds, 25 MiB, 1,000 aliases
are quoted from the findings).


#### 21. A login wall on previews

**As approved, 2026-10-30 (advisor, step 3b; `advisor-resolutions-verdict.md`):** ESCALATE (PREVIEW-WALL, Mustafa); the interim wording REVISED and folded (C21.1, C21.2, C21.4, new C21.8, the shared text).

**Outcome: escalated. Decision code PREVIEW-WALL; decider: Mustafa** (the advisor's table: "Mustafa (privacy against
convenience), now (PREVIEW-WALL below)").

**Both sides.**
- r5:1633-1634 (R5-PREV-REC5): "This changes once a preview talks to a real server or carries unreleased art: then the
  access wall is on."
- r6:611-612 (R6-HOST-REC1): "Previews stay harmless rather than secret: a preview holds no secrets, talks only to the
  staging game server (REC3), and is limited to the account with Cloudflare Access if Mustafa wants it private".
- Existing questions: R5-PREV-OQ1 (Mustafa's call) and R6-HOST-OQ5 (Mustafa, "after the security baseline").

**Why it is escalated.** It trades one login per Brave check (R5-PREV-REC5, R5-PREV-OQ1) against strangers reaching
unreleased work and the staging game server (R6-HOST-REC3): a privacy-against-convenience call that only Mustafa makes
(both topics already name him). Resolution 19 leaves it unchanged: both hosts have an access option (R5-PREV-F12, F14).

**The question for Mustafa (PREVIEW-WALL).** Should per-PR preview links that talk to the staging game server be
private, behind a login on his Cloudflare account?
- (a) Private from the first preview that reaches the staging server.
- (b) Open, since previews hold no secrets (R6-HOST-REC1).
- (c) Decide after the security baseline (see BASELINE-WHEN).
- **Recommendation: (a).** It is R5-PREV-REC5's own condition: a preview that talks to a server is no longer only
  what the public repository already holds (R5-PREV-REC5's reason, inferred there), and the cost is one login per
  session (R5-PREV-REC5's mark). This is also the advisor's recommendation.

**What changes now, under every option.** Until Mustafa answers, no preview talks to a game server: previews run the
match offline in the worker (R6-HOST-REC3, R1-BND-REC6) and stay public but unlisted (R5-PREV-REC5). That holds under
(a), (b) and (c): under (a) the wall goes up before the first multiplayer preview, under (b) nothing changes, under (c)
the multiplayer previews wait for the baseline. The two topics then state one question instead of two positions.

*Shared question, to insert in R5 and R6 (the "Shared open questions" subsection), word for word:*

> **PREVIEW-WALL (Mustafa). Should per-PR previews that talk to the staging game server be private?** Private means
> behind the access wall (GLOSSARY): the host's access policy on Mustafa's Cloudflare account (R5-PREV-F12, R6-HOST-F4)
> or Cloudflare Access (R5-PREV-F14), so each Brave check needs one login per session (R5-PREV-REC5). Open means
> anyone with the link can open the preview and the staging game server it talks to (R6-HOST-REC3). (a) Private from
> the first preview that reaches the staging server; (b) open, since previews hold no secrets; (c) decided after the
> security baseline (BASELINE-WHEN). Until Mustafa answers, no preview talks to a game server: previews run the match
> offline in the worker and stay public but unlisted (R5-PREV-REC5, R6-HOST-REC3); a preview that carries unreleased
> art is behind the access wall, as R5-PREV-REC5 already says (R5-PREV-OQ1 asks that part). Also asked as R5-PREV-OQ1 and
> R6-HOST-OQ5. Decided by: Mustafa.

*Changes per file (before → after).*

R5:
- C21.1, r5:1633-1634 (R5-PREV-REC5):
  - before: `This changes once a preview talks to a real server or carries
  unreleased art: then the access wall is on.`
  - after: `This changes once a preview carries unreleased art: then the access wall (GLOSSARY) is on. Whether it is also
  on once a preview talks to the staging game server (R6-HOST-REC3) is PREVIEW-WALL (Mustafa; below); until he
  answers, no preview talks to a game server.`
- C21.2, r5:1828-1829 (R5-PREV-OQ1), at the end of the entry:
  - before: `and will previews carry anything not already public?`
  - after: `and will previews carry anything not already public? Its staging-server part is shared as PREVIEW-WALL (below).`
- C21.3: insert PREVIEW-WALL.

R6:
- C21.4, r6:611-612 (R6-HOST-REC1):
  - before: `a preview holds no secrets, talks only to the staging game server (REC3),
  and is limited to the account with Cloudflare Access if Mustafa wants it private (R6-HOST-OQ5).`
  - after: `a preview holds no secrets, and once previews reach a game server they talk only to the staging game server
  (REC3); whether those previews sit behind the access wall is PREVIEW-WALL (Mustafa; GLOSSARY: the access wall), and
  until he answers no preview talks to a game server (R6-HOST-OQ5).`
- C21.5, r6:838 (R6-HOST-OQ5), at the end of the entry:
  - before: `Decided by: Mustafa (convenience against privacy),
  after the security baseline.`
  - after: `Decided by: Mustafa (convenience against privacy),
  after the security baseline. Shared as PREVIEW-WALL (below), where the timing is option (c).`
- C21.6, r6:912 (R6-SEC-OQ1):
  - before: `whether previews are private (R6-HOST-OQ5).`
  - after: `whether previews are private (R6-HOST-OQ5; PREVIEW-WALL).`
- C21.7: insert PREVIEW-WALL.
- C21.8, r6:655-656 (R6-HOST-REC3; new in step 3b):
  - before: `need no server
  (R1-BND-REC6).`
  - after: `need no server
  (R1-BND-REC6). Until Mustafa answers PREVIEW-WALL (below), previews do not use the staging server.`

R1–R4: no change.

**Basis ids:** R5-PREV-REC5, R5-PREV-OQ1, R5-PREV-F12, R5-PREV-F14, R6-HOST-REC1, R6-HOST-REC3, R6-HOST-F4,
R6-HOST-OQ5, R6-SEC-OQ1, R1-BND-REC6. **New numbers:** none.


#### 22. Isolation headers on every preview, or behind a flag

**As approved, 2026-10-30 (advisor, step 3b; `advisor-resolutions-verdict.md`):** APPROVE (with the rank-41 fix through C22.1).

**Outcome: resolved.** One isolation flag, off by default in local development, previews and production; on only for
profiling builds (GLOSSARY: preview isolation and the isolation flag; the production header set and the preview header
set). Decider: the advisor (the advisor's table: "Me"); the critic named engine-architect in ADR-0002, with R1-BND-REC2
mark 5 deciding production isolation.

**Both sides.**
- Every preview isolated: r5:1616-1617 (R5-PREV-REC4) "COOP `same-origin` and COEP `require-corp` on preview hostnames
  only (R1-BND-REC5 G3; production stays non-isolated until REC5's test decides)"; r5:1619 "Use the same values in
  Vite's `server.headers` and `preview.headers`, so local development matches the preview".
- Behind a flag: r5:1218-1219 (R5-LAYOUT-REC5) "send COOP `same-origin` and COEP `require-corp` only when an isolation
  flag is set, as R1-BND-REC5 asks for development and previews"; r6:610 (R6-HOST-REC1) "send COOP and COEP from
  `_headers` on previews when R1-BND-REC5's option G3 is on"; r1:2248-2249 (R1-BND-REC5) "Do not require
  cross-origin isolation for v2's start (G1); allow it in development and previews for profiling (G3)".
- Existing question: R5-XQ-OQ3 ("What decides it: engine-architect with R1-BND-REC5's adoption test").

**Which holds and why: the flag.**
1. R1-BND-REC5, the recommendation both R5 parts and R6 serve, says "allow it in development and previews for
   profiling (G3)": allowed for profiling, not required on every preview. R5-LAYOUT-REC5 and R6-HOST-REC1 already read
   it as a switch.
2. With every preview isolated, "Mustafa's Brave check on a preview would run a header set production does not have"
   (R5-XQ-OQ3). His check is the last gate for anything a player sees, so the preview he opens must carry what
   production ships. R5-XQ-OQ3's own option is this one: "the preview Mustafa checks carries the header set production
   will ship (a production-headers preview for his check and an isolated one for profiling)".
3. Profiling keeps its route: an isolated build gives 5 µs CPU timers (R3-MEASURE-F7) and the B3 test of
   R1-BND-REC2 mark 5, on a preview built with the flag on or in local development. R3-MEASURE-OQ4 needs nothing else.
4. Both paths stay tested: "the end-to-end suite should run once isolated and once not" (R5-XQ-OQ3).
5. If R1-BND-REC2 mark 5 leads the ADR to adopt isolation, the default flips everywhere at once, and the preview
   Mustafa checks again matches production.

It depends on no escalated item. It also carries the rank-41 nit (the deciding test is R1-BND-REC2 mark 5, not "REC5's
test"); see `cross-topic-oqs.md`, rank 41.

*Changes per file (before → after).*

R5:
- C22.1, r5:1616-1620 (R5-PREV-REC4):
  - before: `The set: COOP `same-origin` and COEP `require-corp` on preview hostnames only (R1-BND-REC5
  G3; production stays non-isolated until REC5's test decides), `X-Robots-Tag: noindex` (the host default, F12, F15),
  `.wasm` served as `application/wasm`, and R6's content security policy once written. Use the same values in Vite's
  `server.headers` and `preview.headers`, so local development matches the preview (F19).`
  - after: `The set is the preview header set (GLOSSARY): production's header set, that is `.wasm` served as
  `application/wasm` and R6's content security policy once written, plus `X-Robots-Tag: noindex` (the host default,
  F12, F15). COOP `same-origin` and COEP `require-corp` are added only on a preview built with the isolation flag on
  (GLOSSARY; R1-BND-REC5 G3, for profiling); production stays non-isolated until R1-BND-REC2 mark 5's test decides. Vite's
  `server.headers` and `preview.headers` take the same sets behind the same flag (R5-LAYOUT-REC5), so local development
  matches the preview (F19). The preview Mustafa checks is built with the flag off.`
- C22.2, r5:1620-1621 (R5-PREV-REC4):
  - before: `Whether the preview Mustafa checks should
  instead carry production's header set is R5-XQ-OQ3.`
  - after: `This settles R5-XQ-OQ3.`
- C22.3, r5:1626-1628 (R5-PREV-REC4, marks 2 and 3):
  - before: `(2)
  `crossOriginIsolated` is true on a preview link in Chrome, Edge, Firefox, Safari and Brave (R1-BND-REC5's pass mark,
  plus Brave for Mustafa); (3) the production build's headers do not include COOP/COEP.`
  - after: `(2)
  `crossOriginIsolated` is true on a preview built with the isolation flag on, and false on one built with it off, in
  Chrome, Edge, Firefox, Safari and Brave (R1-BND-REC5's pass mark, plus Brave for Mustafa); (3) the production build's
  headers, and those of a preview built with the flag off, do not include COOP/COEP.`
- C22.4, r5:1885-1887 (R5-XQ-OQ3), at the end of the entry:
  - before: `One option among others: the preview Mustafa checks carries the header set
  production will ship (a production-headers preview for his check and an isolated one for profiling).`
  - after: `One option among others: the preview Mustafa checks carries the header set
  production will ship (a production-headers preview for his check and an isolated one for profiling). Settled on
  2026-10-30 (GLOSSARY: preview isolation and the isolation flag): R5-LAYOUT-REC5's flag rule holds for local
  development and previews alike, the preview Mustafa checks is built with the flag off, an isolated preview is built
  only for profiling, and the end-to-end suite runs once with the flag and once without.`

R6:
- C22.5, r6:610 (R6-HOST-REC1, heading):
  - before: `send COOP and COEP from `_headers` on previews when R1-BND-REC5's option G3 is on.**`
  - after: `send COOP and COEP from `_headers` only on previews built with the isolation flag on (GLOSSARY; R1-BND-REC5's
  option G3); the preview Mustafa checks carries production's header set.**`
- C22.6, r6:622-623 (R6-HOST-REC1, spike):
  - before: `when G3 is on and false with it off;`
  - after: `when the isolation flag is on and false with it off;`

R1: no change (R1-BND-REC5 already says "allow"); the glossary link goes on its first "G3".
R3: no change (R3-MEASURE-OQ4 already points to R1-BND-REC5's route).

**Basis ids:** R1-BND-REC5, R1-BND-REC2 mark 5, R5-LAYOUT-REC5, R5-PREV-REC4, R5-PREV-F12, F15, F19, F20, R5-XQ-OQ3,
R6-HOST-REC1, R3-MEASURE-F7, R3-MEASURE-OQ4. **New numbers:** none.


#### 23. One wildcard certificate on every game server

**As approved, 2026-10-30 (advisor, step 3b; `advisor-resolutions-verdict.md`):** ESCALATE (BASELINE-WHEN, the Advisor); interim C23.1–C23.3 approved; the (b)-only set REVISED and folded (C23.b1, C23.b2).

**Outcome: escalated. Decision code BASELINE-WHEN (shared with resolution 17); decider: the Advisor.**

**Both sides.**
- r2:2667 (R2-TRN-REC5): "Production: a publicly trusted certificate per game server hostname."
- r6:664-668 (R6-HOST-REC4): "one wildcard certificate for the game-server names (option C2) … This departs from
  R2-TRN-REC5's wording … one private key sits on every game VM (a key-handling item for the deferred baseline,
  R6-SEC-OQ1). Per-hostname certificates (C1, or dns-01 per name) stay the alternative if the baseline rejects a shared
  key."
- Existing questions: R6-SEC-OQ1 (lists the shared key) and R2-TRN-OQ4 ("Decided by: R6's hosting choice, then the
  REC5 spike").

**Why it is escalated.** R6 states the departure itself and hands the key question to the security baseline; whether
one private key on every VM is acceptable is a security trade neither topic settles, and the baseline's timing is
BASELINE-WHEN, the Advisor's. The question, options and recommendation are resolution 17's: under option (b) R2's
certificate per hostname stays and R6's wildcard becomes an option the baseline weighs.

**What changes now, under every option.** Both topics point to the same question; neither recommendation changes yet.
Under every option, R6-HOST-REC4's own alternative (per-hostname certificates, C1 or dns-01 per name) stays available
with no TCP 80 needed if dns-01 is used per name (R6-HOST-F24: dns-01 proves control with a TXT record; http-01 needs
TCP port 80).

*Shared question:* BASELINE-WHEN, as written in resolution 17 (inserted once per topic; R2 and R6 already get it from
resolution 17).

*Changes now (before → after).*

R2:
- C23.1, r2:2667-2668 (R2-TRN-REC5):
  - before: `- Production: a publicly trusted certificate per game server hostname.`
  - after: `- Production: a publicly trusted certificate per game server hostname. R6-HOST-REC4 proposes one wildcard
  certificate instead, which puts one private key on every game VM; whether the security baseline accepts that is
  BASELINE-WHEN (below).`
- C23.2, r2:3001-3002 (R2-TRN-OQ4), at the end of the entry:
  - before: `Decided by:
R6's hosting choice, then the REC5 spike.`
  - after: `Decided by:
R6's hosting choice, then the REC5 spike. R6's game VMs stay warm, so they are not ephemeral (R6's option C3,
r6:424-425), which leaves production pinning out (REC5); the per-hostname or wildcard form waits on BASELINE-WHEN
(below).`

R6:
- C23.3, r6:668-669 (R6-HOST-REC4):
  - before: `Per-hostname certificates (C1, or dns-01 per name) stay the alternative if the
  baseline rejects a shared key.`
  - after: `Per-hostname certificates (C1, or dns-01 per name) stay the alternative if the
  baseline rejects a shared key; when the baseline is researched is BASELINE-WHEN (below).`

*Changes only if the Advisor picks (b)* (apply after the answer, not before):
- C23.b1, r6:664-665 (R6-HOST-REC4, heading): "Production certificates: one wildcard certificate for the game-server names
  (option C2), from ACME with dns-01, renewed by one job and copied to every game VM." → "Production certificates: a
  publicly trusted certificate per game-server hostname (R2-TRN-REC5; option C1, or ACME dns-01 per name), until the
  security baseline weighs one wildcard certificate (option C2: renewed by one job and copied to every game VM)
  (BASELINE-WHEN (b))."
- C23.b2, r6:665-669 (R6-HOST-REC4, body): "This departs from R2-TRN-REC5's wording" → "C2 would depart from
  R2-TRN-REC5's wording", and "Per-hostname certificates (C1, or dns-01 per name) stay the alternative if the baseline
  rejects a shared key." → "C2 replaces them only if the baseline accepts a shared key."; also r6:669 "It needs no TCP
  80 on game VMs." → "With dns-01 per name it needs no TCP 80 on game VMs (F24); C1's http-01 needs TCP 80."; and the
  Reason (r6:670-671) → "a certificate per hostname is R2-TRN-REC5's and puts no shared private key on the game VMs;
  RFC 8555 proves control through DNS (F24), so dns-01 per name needs no TCP 80. C2's gain, for the baseline to weigh:
  one certificate for all VMs makes adding a VM a copy, not an issuance; RFC 8555 allows wildcard orders (F24)."
- C23.b3, R6-HOST-REC4's spike: "with the wildcard certificate" → "with the per-hostname certificates".
- R2: no change under (b).
- Under (a), the security-auditor's topic decides; under (c), the "now" changes are all.

R1, R3, R4, R5: no change.

**Basis ids:** R2-TRN-REC5, R2-TRN-OQ4, R6-HOST-REC2, R6-HOST-REC4, R6-HOST-F24, R6-SEC-OQ1. **New numbers:** none.


#### 29. HashMap in the sim: lookups allowed, or the type banned

**As approved, 2026-10-30 (advisor, step 3b; `advisor-resolutions-verdict.md`):** REVISE, folded (C29.2).

**Outcome: resolved.** The crate-wide type ban holds (R5-RUSTLINT-REC2); R1's "lookups only" rule becomes the reason a
reviewed `#[allow]` may admit one map (GLOSSARY: the hash-map ban in the sim crate). Decider: the advisor (the advisor's
table: "Me"); R5-RUSTLINT-OQ5 names engine-architect, with the `#[allow]`-with-reason rule for ADR-0002.

**Both sides.**
- r1:2157 (R1-DET-REC5): "hash maps serve lookups only and are never iterated where the order can change an outcome";
  r1:1928-1929 (R1-ECS-REC2): "The sim never iterates a default-hasher HashMap or HashSet (BTreeMap or an index-ordered
  Vec instead)"; r1:1943-1944 (R1-ECS-REC2 mark 2): "A CI check finds zero uses of std HashMap or HashSet iteration in
  the sim crate's outcome code (the tooling is R5's)".
- r5:1317-1319 (R5-RUSTLINT-REC2): `disallowed-types` = `std::collections::HashMap`, `std::collections::HashSet` in the
  sim crate ("replacement `BTreeMap` or an index-ordered `Vec`").
- Existing question: R5-RUSTLINT-OQ5 ("an `#[allow]` with a reason is the escape").

**Which holds and why: the type ban, with a reviewed escape.**
1. R1-ECS-REC2 mark 2 asks for a CI check that finds every HashMap iteration in outcome code, and R5 shows why only a
   type ban can do that: `iter_over_hash_type` flags `for` loops "and only there", so iterator chains such as
   `map.values().sum()` pass it (R5-RUSTLINT-F9; R5-RUSTLINT-REC2's planted case). A lint cannot tell a lookup from an
   iteration through a chain, so "lookups allowed" cannot be checked by the tool that mark 2 needs.
2. R1 already names the replacements, BTreeMap or an index-ordered Vec (R1-ECS-REC2, R1-DET-REC5), and keeps sim state
   in dense arrays or slot maps keyed by entity ids (R1-DET-REC5), so the ban costs the sim no needed structure.
3. A map in the sim state would be iterated anyway: the determinism suite writes "a canonical serialization of its whole
   state" every tick (R1-DET-REC6), which walks every map in it. So R1's "lookups only" can hold only for a map outside
   the serialized state.
4. R5-RUSTLINT-OQ5's escape stays: an `#[allow(clippy::disallowed_types)]` with a written reason, reviewed, for a map
   that is only looked up and is not part of the serialized sim state. ADR-0002 records the allow-with-reason rule.

It depends on no escalated item.

*Changes per file (before → after).*

R1:
- C29.1, r1:2157-2158 (R1-DET-REC5):
  - before: `hash maps serve lookups only and are never iterated where the order can change an outcome; where an ordered map is
  needed, `BTreeMap`.`
  - after: `the sim crate bans std `HashMap` and `HashSet` by type (R5-RUSTLINT-REC2; GLOSSARY: the hash-map ban); a
  reviewed `#[allow]` with a written reason may admit a map that is only looked up and lies outside the serialized
  state, since REC6's canonical serialization walks the whole state; where an ordered map is
  needed, `BTreeMap`.`
- C29.2, r1:1928-1929 (R1-ECS-REC2):
  - before: `The
  sim never iterates a default-hasher HashMap or HashSet (BTreeMap or an index-ordered Vec instead).`
  - after: `The
  sim uses no std HashMap or HashSet (R5-RUSTLINT-REC2 bans the types) (BTreeMap or an index-ordered Vec instead; GLOSSARY: the hash-map ban).`
- C29.3, r1:1943-1944 (R1-ECS-REC2 mark 2):
  - before: `A CI check finds zero uses of std HashMap or HashSet iteration in the sim crate's outcome code (the tooling is
     R5's).`
  - after: `R5-RUSTLINT-REC2's type ban finds zero std HashMap or HashSet in the sim crate outside reviewed `#[allow]`s
     that give their reason (the tooling is R5's).`

R5:
- C29.4, r5:1778-1780 (R5-RUSTLINT-OQ5), at the end of the entry:
  - before: `Decides it: engine-architect, when the sim's modules exist; an `#[allow]` with a
  reason is the escape.`
  - after: `Decides it: engine-architect, when the sim's modules exist; an `#[allow]` with a
  reason is the escape. Settled on 2026-10-30 (GLOSSARY: the hash-map ban): the ban stays crate-wide, and an `#[allow]`
  with a written reason may admit a map that is only looked up and lies outside the serialized sim state
  (R1-DET-REC5, R1-DET-REC6); ADR-0002 records the rule.`

R2, R3, R4, R6: no change.

**Basis ids:** R1-DET-REC5, R1-DET-REC6, R1-ECS-REC2, R5-RUSTLINT-REC2, R5-RUSTLINT-F9, R5-RUSTLINT-OQ5. **New
numbers:** none.

### The advisor's step-3b verdict on the resolutions, the glossary and the cross-topic questions

From `.checkpoints/v2-research/advisor-resolutions-verdict.md` at commit 85f89ea (that folder is deleted in this PR's last commit; the history keeps it).

### Step 3b: the advisor's verdict on the resolutions, the glossary and the cross-topic open questions (verbatim)

The advisor agent, workflow run wf_d38cd72c-feb (prompt `prompts/advisor-resolutions-review.txt`), returned 2026-10-30 about 18:14 UTC. Notes: `advisor-resolutions-review.md`.

**Step 3b verdict: 4 APPROVE, 4 REVISE, 4 ESCALATE.** I approve the tick-budget choice (one server budget of 50 % of the tick period, split 25 % for the sim and 25 % for the server-only work). But resolution 1's claim that every topic's marks fit inside it is wrong at 60 Hz, and I give the corrected numbers below. Engine-architect's question: yes, with a rule.

**TL;DR**
- 25 % for the whole tick still fails worse than 50 % at every size. So 50 % stands, now with honest tight spots at 60 Hz.
- Two fixes run through several items. Previews carrying unreleased art stay behind the login (access wall), as R5 already says. Server marks need a snapshot-receiving client on every soldier, or encoding is never measured.
- The four escalated items keep interim wording that holds under every answer. Their decisions are still open (below).
- No resolution decides the look, a design rule, or anything that is Mustafa's or the Advisor's.

**What I checked myself**
- **Quotes:** my own script finds all 116 "before" quotes exactly once each, and each sits within 3 lines of its stated line. The topic files have not changed since 8dee746 (empty git diff).
- **Arithmetic:** I redid resolution 1's sums, the N = 54 scaling, the 25 % option, and resolution 9's sums (draw calls 786 and 834, enemy pairs 768 and 972).
- **Sources in the topic files:** the 700 shots × 20 µs mark (R2-SYNC-REC5), 0.2 and 0.5 ms (R2-INT-REC1, REC4), R6's cost rows 0.0173 and 0.0347 EUR, R6-HOST-F23's "1 core, 2 vCPU", R6-HOST-F6, R5-GATE-REC1, R5-RUSTLINT-F9, R1-BND-REC5, VISION_V2 §3, LOOP_V2 §2.
- **The other 29 items:** 72 insertions, R1 18 / R2 15 / R3 8 / R4 8 / R5 14 / R6 9, recounted from the critic's JSON. Six texts compared word for word with the critic's.
- **Not checked:** any source page (nothing fetched, as instructed). No gate run: nothing was edited.

#### Per contradiction (by the critic's rank)

**1. What the tick budget covers: REVISE.** The decision stands (50 % total, 25 % sim step share, 25 % server-only share; the 60-or-30 Hz rule unchanged). Required changes:
- **a. Missing rows in the arithmetic table.**
  - Sim step share: avoidance, ≤ 0.2 ms p99 (R4-NAV-REC3, r4:860).
  - Server-only share: the enclosure pair test, 0.1 µs × 768 enemy pairs = 0.08 ms (R2-INT-REC2 (b)).
  - Server-only share, on a tick with a portal change: the re-flood, +1.00 ms (R2-INT-REC2 (b)).
  - Mark the physics row "mean mark; its p99 is reported, not capped".
- **b. New sums at N = 48.**

  | | 60 Hz | 30 Hz |
  |---|---|---|
  | Sim step share | 3.87 ms (limit 4.17) | 5.53 ms (limit 8.33) |
  | Server-only share | 3.41 ms (limit 4.17) | 3.64 ms (limit 8.33) |
  | Server-only, re-flood tick | 4.41 ms | 4.64 ms |
  | Server tick | 7.28 ms (limit 8.33) | 9.18 ms (limit 16.67) |
  | Server tick, re-flood tick | 8.28 ms | 10.18 ms |

  Left at 60 Hz: 0.30 ms in the sim step share, 0.76 ms in the server-only share (−0.24 on a re-flood tick).
- **c. New sums at N = 54** (the same 1.125 scaling, 972 pairs).

  | | 60 Hz | 30 Hz |
  |---|---|---|
  | Sim step share | 4.14 ms (left 0.03) | 5.81 ms |
  | Server-only share | 3.85 ms | 4.12 ms |
  | Server-only, re-flood tick | 4.85 ms | 5.12 ms |
  | Server tick | 7.99 ms | 9.93 ms |
  | Server tick, re-flood tick | 8.99 ms (over 8.33) | 10.93 ms |

- **d. The 25 % option, same rows:** at 60 Hz 3.03 + 3.41 = 6.44 ms (5.24 ms with encoding spread over two ticks), against 4.17. At 30 Hz 7.51 ms, or 8.51 ms with a re-flood, against 8.33.
- **e. "Why 50 %" point 1, and the "holds under either reading" sentence**, replaced by: "At 30 Hz the topics' marks fit inside it at N = 48 and N = 54, re-flood ticks included. At 60 Hz they fit on every tick without a re-flood; on a tick that encodes a snapshot and re-floods after a portal change, the server-only share's ceilings (4.41 ms at N = 48) pass its 4.17 ms and, at N = 54, the whole (8.99 ms) passes 8.33 ms; at N = 54 the sim step share keeps 0.03 ms for the rest of the sim. Under 25 % for the whole tick the marks do not fit at 60 Hz on any snapshot tick, nor at 30 Hz on a re-flood tick, which would decide the tick rate before the spike the rule waits for. The 60 Hz tight spots (the visibility pass, the re-flood tick, the rest of the sim at N = 54) are for the spike to measure."
- **f. The claim that adding p99s gives an upper bound is false.** It appears in resolution 1 and in the glossary's "Time statistics". Replace both with: "Adding the parts' p99 ceilings is a planning check that the marks fit together, not a bound: p99s do not add (two parts that each take 1 ms on 0.6 % of ticks and nothing otherwise each have a p99 of 0, but together they take 1 ms on about 1.2 % of ticks, so the sum's p99 is 1 ms). The server tick time's p99 is always measured directly, with the parts reported beside it."
- **g. C1.9's reason.** Replace "the topics' own part marks add up inside 50 % at both tick rates, while 25 % for the whole tick would not hold them at 60 Hz (the arithmetic is in GLOSSARY's tick-budget section)" with "the topics' own part marks fit inside 50 % at 30 Hz, and at 60 Hz on every tick but one that both encodes a snapshot and re-floods after a portal change, while 25 % for the whole tick would not hold them at 60 Hz on any snapshot tick (the arithmetic, a planning check of ceilings, is in GLOSSARY's tick-budget section)". Put the revised table into GLOSSARY §1, because resolutions.md is deleted at the end, and update the glossary §1 summary sums.
- **h. C1.10 (the R2-BUD-REC6 spike).**
  - Change "the stress scene (GLOSSARY), 30-minute soak:" to "the stress scene (GLOSSARY) with a snapshot-receiving client on every soldier (R2-SYNC-REC9 mark 1's "48 clients"), 30-minute soak:".
  - Add a sentence after `per client per snapshot, memory per room reported.` (r2:2387, unique): "The mark is checked at the tick rate R1-TICK-REC2's rule picks; if it fails there, it fails like any spike mark and goes back to the drafts, and the rule does not change."
- **i. C1.21 (R6 A-2).** Replace "if the measured ticks do not sit well below the ceiling" with "unless R2-ROOM-REC2 marks 1 and 2 pass on it".
- **j. C1.5 (R1-ABL-REC6).** Change "inside the sim step share (GLOSSARY)," to "inside the sim step share on the server and the client's sim share on the laptop (GLOSSARY),".
- **k. Glossary, server-only share.** Add R2-INT-REC2's pair test (0.1 µs per enemy pair) and its re-flood (1 ms). After "(each send call at most 1 ms, R2-TRN-REC1 mark 2)" add "per call, not per tick: the tick's sends have no per-tick mark and share what is left".

**5. Whether the client predicts its own abilities: APPROVE** (aligned as ABL-PREDICT, Mustafa after a Feel Lab test). The interim is that abilities are not predicted. Either answer needs no redesign. Nothing to decide now.

**6. The spread random stream: APPROVE.** Spread comes from an authority-only stream. SEC-SPREAD stays Mustafa's open call, and the design holds under both of its answers.

**9. The stress scene: REVISE** (wording only; the scene is approved).
- In resolution 9's table and the glossary, replace "it comes from a stated rule (squads of 4, LOOP_V2 §2; one harvester per team, GE6), where 4 and 8 come with none" with "it comes from R2's stated assumption (one vehicle per squad of 4, LOOP_V2 §2's squad size, plus one harvester per team, GE6; R2 labels the row "assumption", r2:1758), where 4 and 8 come with no rule".
- Glossary, stress scene: add "For a server mark that includes the server-only share, every soldier has a snapshot-receiving client (R2-SYNC-REC9 mark 1), as the budget's 48 × 50 µs assumes."

**10. Mode sizes, totals or per team: ESCALATE (MODES-3, Mustafa).** The interim wording C10.1–C10.23 is approved: every mark gives both values. Resolutions 1 and 9 hold under both readings.

**11. One core or one vCPU: APPROVE.** Server marks are measured on one vCPU of the CCX23.

**17. Who owns the dependency audit in CI: ESCALATE (BASELINE-WHEN, the Advisor).** Interim C17.1–C17.9 are approved: cross-references only, no policy changes. C17.b1–b3 are approved, to apply only if the Advisor picks (b).

**19. Previews on Workers or Pages: REVISE** (wording only; Pages first is approved, with PREV-DELETE as the shared spike item).
- C19.3: change "0 USD on Pages' Free plan (R6-COST-REC1, R6-HOST-F3)" to "… (R6-COST-REC1, R6-HOST-F3; the price is inferred from the plan's name, no Pages price page was read)".
- C19.8: change "0 USD (Pages' Free plan, R6-HOST-F3)" to "0 USD (Pages' Free plan, R6-HOST-F3; price inferred from the plan's name)".

**21. A login wall on previews: ESCALATE (PREVIEW-WALL, Mustafa); the interim wording needs a REVISE.** C21.1 silently dropped R5's unreleased-art trigger into a question that was asked only about the staging server. That loosening would be Mustafa's call. Changes:
- C21.1 after: "This changes once a preview carries unreleased art: then the access wall (GLOSSARY) is on. Whether it is also on once a preview talks to the staging game server (R6-HOST-REC3) is PREVIEW-WALL (Mustafa; below); until he answers, no preview talks to a game server."
- C21.2 after: "… not already public? Its staging-server part is shared as PREVIEW-WALL (below)."
- C21.4 after: "a preview holds no secrets, and once previews reach a game server they talk only to the staging game server (REC3); whether those previews sit behind the access wall is PREVIEW-WALL (Mustafa; GLOSSARY: the access wall), and until he answers no preview talks to a game server (R6-HOST-OQ5)."
- New C21.8, R6-HOST-REC3 (r6:655-656, unique). Before: `need no server` + line break + `(R1-BND-REC6).`. After: the same, plus "Until Mustafa answers PREVIEW-WALL (below), previews do not use the staging server."
- The shared PREVIEW-WALL text: add to its "Until Mustafa answers" sentence "; a preview that carries unreleased art is behind the access wall, as R5-PREV-REC5 already says (R5-PREV-OQ1 asks that part)".

This interim holds under (a), (b) and (c).

**22. Isolation headers on every preview, or a flag: APPROVE.** One isolation flag, off by default. It also carries the rank-41 fix through C22.1, which is correct: R1-BND-REC5 says "passes REC2's threshold".

**23. One wildcard certificate on every game server: ESCALATE (BASELINE-WHEN, the Advisor).** Interim C23.1–C23.3 are approved. The (b)-only set needs a REVISE:
- C23.b1: put C2's description in brackets: "… until the security baseline weighs one wildcard certificate (option C2: renewed by one job and copied to every game VM) (BASELINE-WHEN (b))."
- C23.b2 also changes r6:669, "It needs no TCP 80 on game VMs.", to "With dns-01 per name it needs no TCP 80 on game VMs (F24); C1's http-01 needs TCP 80."
- C23.b2 also changes the Reason (r6:670-671) to: "a certificate per hostname is R2-TRN-REC5's and puts no shared private key on the game VMs; RFC 8555 proves control through DNS (F24), so dns-01 per name needs no TCP 80. C2's gain, for the baseline to weigh: one certificate for all VMs makes adding a VM a copy, not an issuance; RFC 8555 allows wildcard orders (F24)."

**29. HashMap in the sim: REVISE** (one wording change). The type ban with a reviewed `#[allow]` is approved. C29.2: change "sim uses no default-hasher HashMap or HashSet" to "sim uses no std HashMap or HashSet (R5-RUSTLINT-REC2 bans the types)". The ban is by type, not by hasher.

#### The glossary: REVISE
All 34 entries have the form condition 9 asks for. Every entry that turns on an escalated decision gives each reading and picks none. Changes:
- **Tick budgets:** the fixes in 1c, 1f, 1g and 1k above.
- **Stress scene:** the 15 vehicles' basis and the client sentence from rank 9.
- **Very large unit:** the definition turns a test parameter into a design term. Replace it with "A very large unit is one of VISION_V2 §6.9's 'very large units'; in the stress scene each is a big vehicle of at least 20 parts (R3-CHAR-REC4's test load). Which units are very large, and whether every one is a vehicle, is the roster's (the game-designer's)."
- **Shared stream:** delete the example "the local player's own ability procs if ABL-PREDICT says yes". It would pre-decide, for ability procs, the same cheat trade-off SEC-SPREAD leaves to Mustafa.
- **Access wall:** change "previews that reach no game server are public but unlisted" to "previews that reach no game server and carry no unreleased art are public but unlisted; one that carries unreleased art is behind the wall (R5-PREV-REC5, R5-PREV-OQ1)".
- **Preview:** add "Until PREVIEW-WALL is answered there are no multiplayer previews."
- **Time statistics:** the retire list adds R2-INT-REC2's pair test and re-flood.
- **After this verdict:** the status line becomes "terms approved 2026-10-30 (advisor, step 3b)". The two "Depends on: the advisor's review of resolution 1/19" lines become "Decided by the advisor, 2026-10-30".

#### The cross-topic open questions: APPROVE, with two nits
The 28 questions are in the critic's own words, with both sides' ids. Rank 18 carries BASELINE-WHEN, and each copy ends with "rank N", so a search finds every copy. Nits:
- **Rank 15:** add to its note (not inserted) that resolution 21's interim also rests on offline previews. If Mustafa says offline play is not a need (R1-BND-OQ1), PREVIEW-WALL must be answered before the first preview.
- **Line references inside inserted text** (r4:1110-1129 in rank 3, r6:757 in rank 13) gain "at commit 8dee746", since the insertions shift lines.

#### Engine-architect's question: YES
Dated records keep their wording, and the step-3c term search skips them, under this rule:
- The skip is by marker only:
  - bracketed dated notes starting "[Fix pass", "[Second pass", "[Verifier" or "[Wave";
  - the verifier spot-check tables and fix logs under headings that name them;
  - quoted source text.

  Everything else is searched, including the "Wave-1 items R4 changes or answers" lists.
- The script lists the skipped hits per topic, and the report shows them.
- Records are never edited. Where an old term inside a record could mislead, the owner adds the new term outside it, as C6.6 does.

#### Conditions for step 3c
1. **Fold first.** The session folds this verdict's exact texts into resolutions.md (marked "as approved, 2026-10-30") and GLOSSARY.md, changes nothing else, commits, and records the SHA. A fold that departs from these texts comes back to me.
2. **Apply.** Owners apply the APPROVE and REVISE items as folded. For escalated items they apply only the "now" changes and the shared question. C17.b1–b3 and C23.b1–b3 wait for BASELINE-WHEN. An answer from Mustafa or the Advisor that arrives before the PR is ready is applied as a separate, logged change.
3. **Order in each file:** resolution changes, then glossary renames, then the cross-topic questions, then nits. A quote that no longer matches means OBJECT; owners never improvise.
4. **Log as recommendation changes** (the verifier checks them per condition 10): C1.1, C1.5, C1.8–C1.11, C1.15–C1.18, C1.21; C5.1–C5.4; C6.1, C6.2; C9.1, C9.3, C9.7, C9.9, C9.14, C9.18–C9.20; C19.1, C19.5, C19.6; C21.1, C21.4, C21.8; C22.1, C22.3, C22.5; C29.1–C29.3. Also every mark the "no statistic means p99" rule changes: at least R1-ABL-REC6, R2-INT-REC1, R2-INT-REC4 and R2-INT-REC2.
5. **R3's owner** changes only counts and arithmetic (C9.12–C9.16, C10.12–C10.13) and glossary terms. The look rows (post-processing, outline, style) stay word for word.
6. **GLOSSARY.md** is edited only by the engine-architect agent (its R1 and R4 run), and never while the other owners are renaming terms. Its "used in" lists are refreshed from the search script after step 3c.
7. **No fetching in step 3c; nothing new.** No new number, source or claim. Quoted source text and dated records are untouched. At most 4 agents at once, one per file, efforts as the plan says.
8. **Step 3d** recomputes resolution 1's revised table independently. The report carries the 12 resolutions and these verdicts word for word, including the 60 Hz tight spots and the cost note: if 8 rooms per VM fail, compute per room-hour is 0.0347 instead of 0.0173 EUR (R6's own rows).

#### Decisions needed
- **MODES-3 (Mustafa): how should VISION_V2 §3's starting sizes be read?** The text says "two teams with 2–8 players, and three teams with 6–18 players".
  - **Why it matters:** it sets the largest mode, which the stress scene, the offline test, the soak runs and the cost rows use (48 or 54 players).
  - **Options:** (a) totals: 1v1 to 4v4, and 2v2v2 to 6v6v6, so the largest mode is 24v24 = 48. (b) per team: 2v2 to 8v8, and 6v6v6 to 18v18v18, so the largest mode is 54.
  - **Recommendation:** (a). Under (b), the start modes would be bigger than both goals.
  - **Note:** CLAUDE.md's own wording, "two teams of 2–8", reads like (b). It needs aligning after your answer, and CLAUDE.md changes are yours.
- **PREVIEW-WALL (Mustafa): should per-PR previews that talk to the staging game server be private, behind a Cloudflare login on your account?**
  - **Why it matters:** it trades one login per Brave check against strangers reaching unreleased work and the staging server.
  - **Options:** (a) private from the first such preview; (b) open, since previews hold no secrets; (c) decide after the security baseline.
  - **Recommendation:** (a).
  - **Meanwhile:** no preview talks to a game server, and previews carrying unreleased art stay behind the login, as R5 already says.
- **BASELINE-WHEN (the Advisor, relayed by Mustafa): research the deferred security baseline before ADR 0006, or list it open there?**
  - **Why it matters:** two contradictions wait on it: who owns the CI advisories check and whether it blocks a PR (#17), and one shared certificate key on every game server (#23).
  - **Options:** (a) research it now, as one security-auditor topic; (b) list it open, with interim positions (R5's scheduled advisories check stays the mechanism; one certificate per hostname stays, and R6's wildcard becomes an option the baseline weighs); (c) leave both as one shared open question.
  - **Recommendation:** (b).

#### For Mustafa
1. Answer MODES-3 and PREVIEW-WALL above, if you haven't yet.
2. Pass BASELINE-WHEN to the Advisor.
3. Still open from my last verdict, if not done yet: create the night-shift routine that starts a fresh session on every run, and disable the old one in the same sitting.
4. Later: say "merge #28" once the advisor approves its head.

Files:
- My notes, ending in DONE: /home/user/Shamble-Frontlines/.checkpoints/v2-research/advisor-resolutions-review.md
- Quote check script: /tmp/claude-0/-home-user-Shamble-Frontlines/ed3ff506-9a45-5a14-adc3-75c262311890/scratchpad/adv/check.py
- Line check script: /tmp/claude-0/-home-user-Shamble-Frontlines/ed3ff506-9a45-5a14-adc3-75c262311890/scratchpad/adv/lines.py

### The advisor's ruling on the glossary's Retire split

From `.checkpoints/v2-research/advisor-retire-split-verdict.md` at commit 85f89ea (that folder is deleted in this PR's last commit; the history keeps it).

### The advisor's ruling on the glossary Retire split (verbatim)

Workflow run wf_06dbf34d-b30 (prompt `prompts/advisor-retire-split.txt`), returned 2026-10-30 about 18:21 UTC.

**Verdict: APPROVE, with conditions.** The split serves condition 9's purpose: every topic uses the glossary, and anyone can check it. I ran the script myself and got 1,335 hits. About 380 of them come from the script, not from wrong uses.

**Changes:**
1. **Kept words come out of the quotes.** The script picks up every quoted string in "Retire", including words the text keeps: "Core" (76 hits), "big" (53), "Whole ticks" (5), and "the security baseline" (2), which is the correct name. These words, and replacement words such as "authority-only" and "the largest mode's N", should move to an unquoted "Keeps:" clause. That is a format fix, listed in the edit note.
2. **Script fixes:**
   - Match whole words only, so "core" no longer hits "simulation core" or "score", and "48" no longer hits "1480".
   - Skip hits inside the entry's own term name, its "Marked by" phrases, or a longer phrase. Today "tick time" hits the term "Server tick time", and "whole tick" hits "Whole ticks".
   - Count each place in the text once. Today "48" is counted twice, 244 times under each of two entries.
   - Print every hit with its file, line and nearest id.
3. **The test for "exact":** a phrase goes there only if none of today's hits is a correct use.
   - "Core" goes nowhere: it is not retired at all.
   - "server core" and "1 CPU core" go to exact. "one core" and "per core" go to exact only if all 11 of today's hits are about server work.
   - The id-anchored phrases are exact. Everything with a context clause stays "in context".
4. **Before step 3c:** the session checks that the quoted phrases are the same before and after the edit, apart from item 1. It re-runs the baseline and records both counts in the checkpoint.

**Rule for the step-3d verifiers:** an "in context" hit is wrong when, in its sentence, it carries the term's meaning without the term's marking words. Wrong uses go back to their owner. A hit is right in three cases:
- it means something else (client cores, character models, a stated N = 48 scale point);
- it sits beside the marking words;
- it is quoted text or a dated record.

When unsure, count it wrong. Every hit gets judged, with no sampling, and each judgment records file:line, right or wrong, and one line of reason. The report shows the exact count (it must be 0), the in-context hits, how many were wrong, and how many were fixed.

My notes are in /home/user/Shamble-Frontlines/.checkpoints/v2-research/advisor-retire-split.md.

### The advisor's ruling on step 3c's one OBJECT

From `.checkpoints/v2-research/advisor-object-1-verdict.md` at commit 85f89ea (that folder is deleted in this PR's last commit; the history keeps it).

### The advisor's ruling on step 3c's OBJECT-1 (R2-INT-F20, "one core") and the C23.2 nit (verbatim)

Workflow run wf_02baa00f-caf (prompt `prompts/advisor-object-1.txt`), launched 2026-10-30 20:20 UTC, returned about 20:21 UTC. Notes: `advisor-object-1.md`.

**OBJECT: APPROVE option (b), with two changes.** The owner is right. r2:870 is a share of one core of the legacy test machine, whose hardware is not recorded. "One vCPU" would make it read as a mark on the reference server. So that hit is a correct use, and under my own test "one core" cannot stay in exact. The context words must be narrower than "for server work", because this finding is about server work too.

The approved glossary words:
- Retire (exact): "per core", "server core", "1 CPU core": every use today is for server work.
- Retire (in context): "one core", for a server time mark or a share of the reference server.
- Keeps, add: "; and a share of the machine a legacy measurement ran on (R2-INT-F20)".

Condition, a nit for the step-3d fix round: change r2:870 to "4–9 % of one core of the measuring machine".

**Nit: APPROVE.** At commit 8dee746, r6:424-425 is option C3, so the reference is right. Adding "at commit 8dee746" pins it now that R6's lines have moved.

Notes: /home/user/Shamble-Frontlines/.checkpoints/v2-research/advisor-object-1.md

### The advisor's ruling on the two R3 items of step 3d

From `.checkpoints/v2-research/advisor-3d-r3-verdict.md` at commit 85f89ea (that folder is deleted in this PR's last commit; the history keeps it).

### The advisor's ruling on the two R3 items of step 3d (verbatim)

Workflow run wf_5f85b87e-ee4 (prompt `prompts/advisor-3d-r3.txt`), launched 2026-10-30 20:39 UTC, returned about 20:40 UTC. Notes: `advisor-3d-r3.md`.

**Item 5 (R3-LOOK-REC3): APPROVE (b), with my wording.** Condition 5 keeps the look row word for word. Under the records rule, the new term goes outside the row. The note names both MODES-3 values, the same way R3-CHAR-REC1 now does. The owner adds this line right after the row's "Spike must prove":
"Term note (outside the look row, which keeps its words under condition 5): its "48" means the largest mode's [N](GLOSSARY.md#player-count-n) (48 or 54, MODES-3)."

**Item 6 (R3 time marks with no statistic): APPROVE (a).** The rule is mine from step 3b. Condition 4 already says every mark it changes must be logged. A sub-budget at p99 is stricter than the p95 frame-time mark, so it is safe. Choosing p95 instead would be a new mark choice in a fix round.
1. The owner adds "p99":
   - BUDGET-REC2: ", p99" after each name of the CPU, animation, sim and GPU rows (r3:1124–1131).
   - CHAR-REC1: "2.0 ms (p99)" and "1.0 ms (p99)".
   - CHAR-REC2: "1.0 ms (p99)".
   - CHAR-REC4: "2.0 ms (p99) GPU time".
   - AUDIO-REC3: "0.2 ms (p99) per frame".
2. Each one is logged as a recommendation change. The owner may OBJECT, but must give a reason.
3. The engine-architect corrects R3's "Used today" entry in the glossary (condition 6).

My notes are in /home/user/Shamble-Frontlines/.checkpoints/v2-research/advisor-3d-r3.md.

## 3. The fetch audit (VERIFY-FETCH (a))

Every web fetch in this change-set's agent transcripts and the session's own, by agent, tool and host, from `scripts/fetch_audit.py --grouped` (a WebFetch "redirect notice" row is a redirect the tool reported and did not follow; "(from a file or variable)" is a curl whose address was in a variable or a URL file).

```
transcripts read; fetch rows 1512
  WebFetch: 1414 requests to 54 hosts; 14 different final hosts named (4 hosts)
  curl: 22 requests to 11 hosts; 6 different final hosts named (5 hosts)
hosts off the allow list (tool, requested or final, host, agent: count):
  WebFetch · final (a redirect target, not fetched by the tool) · bevy.org · engine-architect: R1 unit data formats, hot reload: 1
  WebFetch · final (a redirect target, not fetched by the tool) · bevy.org · engine-architect: R1 unit ecs: ECS or plain data: 1
  WebFetch · final (a redirect target, not fetched by the tool) · clippy.rust-lang.org · test-engineer: R5 owner: toolchain and quality: 1
  WebFetch · final (a redirect target, not fetched by the tool) · docs.fly.io · infra-engineer: R6 owner: infrastructure: 1
  WebFetch · final (a redirect target, not fetched by the tool) · gdconf.com · engine-architect: R1 unit abilities: GAS-style system: 2
  WebFetch · final (a redirect target, not fetched by the tool) · gdconf.com · engine-architect: R1 unit ecs: ECS or plain data: 2
  WebFetch · final (a redirect target, not fetched by the tool) · gdconf.com · engine-architect: R1 unit tick: fixed tick rate: 2
  WebFetch · final (a redirect target, not fetched by the tool) · gdconf.com · netcode-architect: R2 unit interest management: 1
  WebFetch · final (a redirect target, not fetched by the tool) · gdconf.com · netcode-architect: R2 unit sync model and lessons: 2
  WebFetch · final (a redirect target, not fetched by the tool) · gdconf.com · netcode-architect: R2 unit tick and bandwidth budgets: 1
  curl · requested · api.github.com · advisor: Advisor PR review of #29: 1
```
| Agent | Run | Tool | Requested host | Final host (if different) | Calls | On the allow list |
|---|---|---|---|---|---|---|
| advisor: Advisor PR review of #29 | background | curl | (from a file or variable) |  | 9 | n/a |
| advisor: Advisor PR review of #29 | background | curl | api.github.com |  | 1 | NO |
| docs-keeper: docs-keeper | wf_336a6842-8fb | curl | (from a file or variable) |  | 1 | n/a |
| engine-architect: R1 unit abilities: GAS-style system | background | WebFetch | developer.valvesoftware.com |  | 2 | yes |
| engine-architect: R1 unit abilities: GAS-style system | background | WebFetch | doc.rust-lang.org |  | 1 | yes |
| engine-architect: R1 unit abilities: GAS-style system | background | WebFetch | docs.rs |  | 5 | yes |
| engine-architect: R1 unit abilities: GAS-style system | background | WebFetch | gdcvault.com |  | 1 | yes |
| engine-architect: R1 unit abilities: GAS-style system | background | WebFetch | github.com |  | 2 | yes |
| engine-architect: R1 unit abilities: GAS-style system | background | WebFetch | raw.githubusercontent.com |  | 16 | yes |
| engine-architect: R1 unit abilities: GAS-style system | background | WebFetch | www.gdcvault.com |  | 1 | yes |
| engine-architect: R1 unit abilities: GAS-style system | background | WebFetch (redirect notice) | gdcvault.com | gdconf.com | 1 | yes; final NO |
| engine-architect: R1 unit abilities: GAS-style system | background | WebFetch (redirect notice) | www.gdcvault.com | gdconf.com | 1 | yes; final NO |
| engine-architect: R1 unit abilities: GAS-style system | background | WebSearch |  |  | 3 | n/a |
| engine-architect: R1 unit boundary and Web Worker | background | WebFetch | blog.rust-lang.org |  | 1 | yes |
| engine-architect: R1 unit boundary and Web Worker | background | WebFetch | developer.chrome.com |  | 1 | yes |
| engine-architect: R1 unit boundary and Web Worker | background | WebFetch | developer.mozilla.org |  | 16 | yes |
| engine-architect: R1 unit boundary and Web Worker | background | WebFetch | doc.rust-lang.org |  | 2 | yes |
| engine-architect: R1 unit boundary and Web Worker | background | WebFetch | docs.rs |  | 3 | yes |
| engine-architect: R1 unit boundary and Web Worker | background | WebFetch | github.com |  | 2 | yes |
| engine-architect: R1 unit boundary and Web Worker | background | WebFetch | raw.githubusercontent.com |  | 14 | yes |
| engine-architect: R1 unit boundary and Web Worker | background | WebFetch | rustwasm.github.io |  | 2 | yes |
| engine-architect: R1 unit boundary and Web Worker | background | WebFetch | vite.dev |  | 2 | yes |
| engine-architect: R1 unit boundary and Web Worker | background | WebFetch | web.dev |  | 1 | yes |
| engine-architect: R1 unit boundary and Web Worker | background | WebSearch |  |  | 1 | n/a |
| engine-architect: R1 unit data formats, hot reload | background | WebFetch | bevyengine.org |  | 1 | yes |
| engine-architect: R1 unit data formats, hot reload | background | WebFetch | cheatsheetseries.owasp.org |  | 2 | yes |
| engine-architect: R1 unit data formats, hot reload | background | WebFetch | datatracker.ietf.org |  | 1 | yes |
| engine-architect: R1 unit data formats, hot reload | background | WebFetch | developer.mozilla.org |  | 6 | yes |
| engine-architect: R1 unit data formats, hot reload | background | WebFetch | developer.valvesoftware.com |  | 2 | yes |
| engine-architect: R1 unit data formats, hot reload | background | WebFetch | doc.rust-lang.org |  | 1 | yes |
| engine-architect: R1 unit data formats, hot reload | background | WebFetch | docs.rs |  | 12 | yes |
| engine-architect: R1 unit data formats, hot reload | background | WebFetch | github.com |  | 5 | yes |
| engine-architect: R1 unit data formats, hot reload | background | WebFetch | raw.githubusercontent.com |  | 8 | yes |
| engine-architect: R1 unit data formats, hot reload | background | WebFetch | vite.dev |  | 2 | yes |
| engine-architect: R1 unit data formats, hot reload | background | WebFetch (redirect notice) | bevyengine.org | bevy.org | 1 | yes; final NO |
| engine-architect: R1 unit data formats, hot reload | background | WebSearch |  |  | 1 | n/a |
| engine-architect: R1 unit determinism: Wasm vs native | background | WebFetch | crates.io |  | 1 | yes |
| engine-architect: R1 unit determinism: Wasm vs native | background | WebFetch | developer.mozilla.org |  | 1 | yes |
| engine-architect: R1 unit determinism: Wasm vs native | background | WebFetch | developer.valvesoftware.com |  | 1 | yes |
| engine-architect: R1 unit determinism: Wasm vs native | background | WebFetch | doc.rust-lang.org |  | 7 | yes |
| engine-architect: R1 unit determinism: Wasm vs native | background | WebFetch | docs.rs |  | 9 | yes |
| engine-architect: R1 unit determinism: Wasm vs native | background | WebFetch | gafferongames.com |  | 4 | yes |
| engine-architect: R1 unit determinism: Wasm vs native | background | WebFetch | github.com |  | 2 | yes |
| engine-architect: R1 unit determinism: Wasm vs native | background | WebFetch | rapier.rs |  | 2 | yes |
| engine-architect: R1 unit determinism: Wasm vs native | background | WebFetch | raw.githubusercontent.com |  | 16 | yes |
| engine-architect: R1 unit determinism: Wasm vs native | background | WebFetch | rust-lang.github.io |  | 2 | yes |
| engine-architect: R1 unit determinism: Wasm vs native | background | WebFetch | webassembly.github.io |  | 6 | yes |
| engine-architect: R1 unit determinism: Wasm vs native | background | WebFetch | webassembly.org |  | 1 | yes |
| engine-architect: R1 unit determinism: Wasm vs native | background | WebSearch |  |  | 2 | n/a |
| engine-architect: R1 unit ecs: ECS or plain data | background | WebFetch | bevyengine.org |  | 1 | yes |
| engine-architect: R1 unit ecs: ECS or plain data | background | WebFetch | developer.valvesoftware.com |  | 1 | yes |
| engine-architect: R1 unit ecs: ECS or plain data | background | WebFetch | doc.rust-lang.org |  | 8 | yes |
| engine-architect: R1 unit ecs: ECS or plain data | background | WebFetch | docs.rs |  | 19 | yes |
| engine-architect: R1 unit ecs: ECS or plain data | background | WebFetch | gdcvault.com |  | 1 | yes |
| engine-architect: R1 unit ecs: ECS or plain data | background | WebFetch | github.com |  | 6 | yes |
| engine-architect: R1 unit ecs: ECS or plain data | background | WebFetch | rapier.rs |  | 1 | yes |
| engine-architect: R1 unit ecs: ECS or plain data | background | WebFetch | raw.githubusercontent.com |  | 4 | yes |
| engine-architect: R1 unit ecs: ECS or plain data | background | WebFetch | rustwasm.github.io |  | 1 | yes |
| engine-architect: R1 unit ecs: ECS or plain data | background | WebFetch | www.gdcvault.com |  | 1 | yes |
| engine-architect: R1 unit ecs: ECS or plain data | background | WebFetch (redirect notice) | bevyengine.org | bevy.org | 1 | yes; final NO |
| engine-architect: R1 unit ecs: ECS or plain data | background | WebFetch (redirect notice) | gdcvault.com | gdconf.com | 1 | yes; final NO |
| engine-architect: R1 unit ecs: ECS or plain data | background | WebFetch (redirect notice) | www.gdcvault.com | gdconf.com | 1 | yes; final NO |
| engine-architect: R1 unit ecs: ECS or plain data | background | WebSearch |  |  | 2 | n/a |
| engine-architect: R1 unit tick: fixed tick rate | background | WebFetch | developer.chrome.com |  | 1 | yes |
| engine-architect: R1 unit tick: fixed tick rate | background | WebFetch | developer.mozilla.org |  | 6 | yes |
| engine-architect: R1 unit tick: fixed tick rate | background | WebFetch | developer.valvesoftware.com |  | 2 | yes |
| engine-architect: R1 unit tick: fixed tick rate | background | WebFetch | docs.rs |  | 6 | yes |
| engine-architect: R1 unit tick: fixed tick rate | background | WebFetch | gafferongames.com |  | 5 | yes |
| engine-architect: R1 unit tick: fixed tick rate | background | WebFetch | gdcvault.com |  | 1 | yes |
| engine-architect: R1 unit tick: fixed tick rate | background | WebFetch | raw.githubusercontent.com |  | 10 | yes |
| engine-architect: R1 unit tick: fixed tick rate | background | WebFetch | www.gdcvault.com |  | 1 | yes |
| engine-architect: R1 unit tick: fixed tick rate | background | WebFetch (redirect notice) | gdcvault.com | gdconf.com | 1 | yes; final NO |
| engine-architect: R1 unit tick: fixed tick rate | background | WebFetch (redirect notice) | www.gdcvault.com | gdconf.com | 1 | yes; final NO |
| engine-architect: R1 unit tick: fixed tick rate | background | WebSearch |  |  | 2 | n/a |
| engine-architect: R1 unit tick: fixed tick rate | background | curl | (proxy status) |  | 2 | n/a |
| engine-architect: R4 owner: physics and navigation | background | WebFetch | dimforge.com |  | 4 | yes |
| engine-architect: R4 owner: physics and navigation | background | WebFetch | doc.rust-lang.org |  | 2 | yes |
| engine-architect: R4 owner: physics and navigation | background | WebFetch | docs.rs |  | 21 | yes |
| engine-architect: R4 owner: physics and navigation | background | WebFetch | github.com |  | 4 | yes |
| engine-architect: R4 owner: physics and navigation | background | WebFetch | rapier.rs |  | 8 | yes |
| engine-architect: R4 owner: physics and navigation | background | WebFetch | raw.githubusercontent.com |  | 15 | yes |
| engine-architect: R4 owner: physics and navigation | background | WebSearch |  |  | 5 | n/a |
| engine-architect: r1-fix | wf_7dd61ce0-f84 | WebFetch | developer.mozilla.org |  | 2 | yes |
| engine-architect: r1-fix | wf_7dd61ce0-f84 | WebFetch | doc.rust-lang.org |  | 2 | yes |
| engine-architect: r1-fix | wf_7dd61ce0-f84 | WebFetch | docs.rs |  | 3 | yes |
| engine-architect: r1-fix | wf_7dd61ce0-f84 | WebFetch | raw.githubusercontent.com |  | 1 | yes |
| engine-architect: r1-fix | wf_7dd61ce0-f84 | WebFetch | webassembly.github.io |  | 1 | yes |
| graphics-engineer: R3 owner: rendering | background | WebFetch | caniuse.com |  | 2 | yes |
| graphics-engineer: R3 owner: rendering | background | WebFetch | developer.chrome.com |  | 4 | yes |
| graphics-engineer: R3 owner: rendering | background | WebFetch | developer.mozilla.org |  | 13 | yes |
| graphics-engineer: R3 owner: rendering | background | WebFetch | docs.github.com |  | 1 | yes |
| graphics-engineer: R3 owner: rendering | background | WebFetch | github.com |  | 4 | yes |
| graphics-engineer: R3 owner: rendering | background | WebFetch | raw.githubusercontent.com |  | 21 | yes |
| graphics-engineer: R3 owner: rendering | background | WebFetch | threejs.org |  | 20 | yes |
| graphics-engineer: R3 owner: rendering | background | WebFetch | web.dev |  | 1 | yes |
| graphics-engineer: R3 owner: rendering | background | WebFetch | webaudio.github.io |  | 3 | yes |
| graphics-engineer: R3 owner: rendering | background | WebFetch | webkit.org |  | 1 | yes |
| graphics-engineer: R3 owner: rendering | background | WebFetch | www.w3.org |  | 3 | yes |
| graphics-engineer: R3 owner: rendering | background | WebSearch |  |  | 6 | n/a |
| infra-engineer: R5 helper: CI and preview links | background | WebFetch | blog.rust-lang.org |  | 2 | yes |
| infra-engineer: R5 helper: CI and preview links | background | WebFetch | developer.mozilla.org |  | 1 | yes |
| infra-engineer: R5 helper: CI and preview links | background | WebFetch | developers.cloudflare.com |  | 5 | yes |
| infra-engineer: R5 helper: CI and preview links | background | WebFetch | doc.rust-lang.org |  | 3 | yes |
| infra-engineer: R5 helper: CI and preview links | background | WebFetch | docs.github.com |  | 15 | yes |
| infra-engineer: R5 helper: CI and preview links | background | WebFetch | docs.netlify.com |  | 2 | yes |
| infra-engineer: R5 helper: CI and preview links | background | WebFetch | github.blog |  | 2 | yes |
| infra-engineer: R5 helper: CI and preview links | background | WebFetch | github.com |  | 10 | yes |
| infra-engineer: R5 helper: CI and preview links | background | WebFetch | playwright.dev |  | 1 | yes |
| infra-engineer: R5 helper: CI and preview links | background | WebFetch | raw.githubusercontent.com |  | 1 | yes |
| infra-engineer: R5 helper: CI and preview links | background | WebFetch | rust-lang.github.io |  | 1 | yes |
| infra-engineer: R5 helper: CI and preview links | background | WebFetch | vercel.com |  | 1 | yes |
| infra-engineer: R5 helper: CI and preview links | background | WebFetch | vite.dev |  | 3 | yes |
| infra-engineer: R5 helper: CI and preview links | background | WebSearch |  |  | 4 | n/a |
| infra-engineer: R6 owner: infrastructure | background | WebFetch | aws.amazon.com |  | 2 | yes |
| infra-engineer: R6 owner: infrastructure | background | WebFetch | cheatsheetseries.owasp.org |  | 1 | yes |
| infra-engineer: R6 owner: infrastructure | background | WebFetch | cloud.google.com |  | 2 | yes |
| infra-engineer: R6 owner: infrastructure | background | WebFetch | developer.mozilla.org |  | 1 | yes |
| infra-engineer: R6 owner: infrastructure | background | WebFetch | developers.cloudflare.com |  | 7 | yes |
| infra-engineer: R6 owner: infrastructure | background | WebFetch | docs.github.com |  | 1 | yes |
| infra-engineer: R6 owner: infrastructure | background | WebFetch | docs.hetzner.com |  | 6 | yes |
| infra-engineer: R6 owner: infrastructure | background | WebFetch | docs.netlify.com |  | 1 | yes |
| infra-engineer: R6 owner: infrastructure | background | WebFetch | docs.rs |  | 4 | yes |
| infra-engineer: R6 owner: infrastructure | background | WebFetch | docs.sentry.io |  | 8 | yes |
| infra-engineer: R6 owner: infrastructure | background | WebFetch | fly.io |  | 2 | yes |
| infra-engineer: R6 owner: infrastructure | background | WebFetch | github.com |  | 1 | yes |
| infra-engineer: R6 owner: infrastructure | background | WebFetch | grafana.com |  | 3 | yes |
| infra-engineer: R6 owner: infrastructure | background | WebFetch | opentelemetry.io |  | 1 | yes |
| infra-engineer: R6 owner: infrastructure | background | WebFetch | prometheus.io |  | 1 | yes |
| infra-engineer: R6 owner: infrastructure | background | WebFetch | vercel.com |  | 4 | yes |
| infra-engineer: R6 owner: infrastructure | background | WebFetch | www.hetzner.com |  | 6 | yes |
| infra-engineer: R6 owner: infrastructure | background | WebFetch | www.rfc-editor.org |  | 2 | yes |
| infra-engineer: R6 owner: infrastructure | background | WebFetch (redirect notice) | fly.io | docs.fly.io | 1 | yes; final NO |
| infra-engineer: R6 owner: infrastructure | background | WebSearch |  |  | 3 | n/a |
| netcode-architect: R2 unit WebTransport stack | background | WebFetch | crates.io |  | 12 | yes |
| netcode-architect: R2 unit WebTransport stack | background | WebFetch | datatracker.ietf.org |  | 8 | yes |
| netcode-architect: R2 unit WebTransport stack | background | WebFetch | developer.chrome.com |  | 1 | yes |
| netcode-architect: R2 unit WebTransport stack | background | WebFetch | developer.mozilla.org |  | 3 | yes |
| netcode-architect: R2 unit WebTransport stack | background | WebFetch | developer.valvesoftware.com |  | 1 | yes |
| netcode-architect: R2 unit WebTransport stack | background | WebFetch | docs.rs |  | 10 | yes |
| netcode-architect: R2 unit WebTransport stack | background | WebFetch | gafferongames.com |  | 2 | yes |
| netcode-architect: R2 unit WebTransport stack | background | WebFetch | github.com |  | 4 | yes |
| netcode-architect: R2 unit WebTransport stack | background | WebFetch | raw.githubusercontent.com |  | 20 | yes |
| netcode-architect: R2 unit WebTransport stack | background | WebFetch | webkit.org |  | 2 | yes |
| netcode-architect: R2 unit WebTransport stack | background | WebFetch | www.rfc-editor.org |  | 6 | yes |
| netcode-architect: R2 unit WebTransport stack | background | WebFetch | www.w3.org |  | 6 | yes |
| netcode-architect: R2 unit WebTransport stack | background | WebSearch |  |  | 1 | n/a |
| netcode-architect: R2 unit interest management | background | WebFetch | developer.chrome.com |  | 1 | yes |
| netcode-architect: R2 unit interest management | background | WebFetch | developer.valvesoftware.com |  | 2 | yes |
| netcode-architect: R2 unit interest management | background | WebFetch | gafferongames.com |  | 2 | yes |
| netcode-architect: R2 unit interest management | background | WebFetch | raw.githubusercontent.com |  | 12 | yes |
| netcode-architect: R2 unit interest management | background | WebFetch | threejs.org |  | 1 | yes |
| netcode-architect: R2 unit interest management | background | WebFetch | www.gdcvault.com |  | 1 | yes |
| netcode-architect: R2 unit interest management | background | WebFetch (redirect notice) | www.gdcvault.com | gdconf.com | 1 | yes; final NO |
| netcode-architect: R2 unit interest management | background | WebSearch |  |  | 3 | n/a |
| netcode-architect: R2 unit interest management | background | curl | (proxy status) |  | 1 | n/a |
| netcode-architect: R2 unit rooms and private matches | background | WebFetch | cheatsheetseries.owasp.org |  | 1 | yes |
| netcode-architect: R2 unit rooms and private matches | background | WebFetch | datatracker.ietf.org |  | 1 | yes |
| netcode-architect: R2 unit rooms and private matches | background | WebFetch | developer.mozilla.org |  | 2 | yes |
| netcode-architect: R2 unit rooms and private matches | background | WebFetch | developer.valvesoftware.com |  | 2 | yes |
| netcode-architect: R2 unit rooms and private matches | background | WebFetch | doc.rust-lang.org |  | 2 | yes |
| netcode-architect: R2 unit rooms and private matches | background | WebFetch | docs.aws.amazon.com |  | 4 | yes |
| netcode-architect: R2 unit rooms and private matches | background | WebFetch | docs.rs |  | 2 | yes |
| netcode-architect: R2 unit rooms and private matches | background | WebFetch | gafferongames.com |  | 1 | yes |
| netcode-architect: R2 unit rooms and private matches | background | WebFetch | learn.microsoft.com |  | 4 | yes |
| netcode-architect: R2 unit rooms and private matches | background | WebFetch | raw.githubusercontent.com |  | 3 | yes |
| netcode-architect: R2 unit rooms and private matches | background | WebFetch | www.rfc-editor.org |  | 1 | yes |
| netcode-architect: R2 unit rooms and private matches | background | WebSearch |  |  | 9 | n/a |
| netcode-architect: R2 unit sync model and lessons | background | WebFetch | developer.valvesoftware.com |  | 5 | yes |
| netcode-architect: R2 unit sync model and lessons | background | WebFetch | gafferongames.com |  | 13 | yes |
| netcode-architect: R2 unit sync model and lessons | background | WebFetch | gdcvault.com |  | 1 | yes |
| netcode-architect: R2 unit sync model and lessons | background | WebFetch | github.com |  | 2 | yes |
| netcode-architect: R2 unit sync model and lessons | background | WebFetch | raw.githubusercontent.com |  | 26 | yes |
| netcode-architect: R2 unit sync model and lessons | background | WebFetch | www.gdcvault.com |  | 1 | yes |
| netcode-architect: R2 unit sync model and lessons | background | WebFetch | www.rfc-editor.org |  | 1 | yes |
| netcode-architect: R2 unit sync model and lessons | background | WebFetch (redirect notice) | gdcvault.com | gdconf.com | 1 | yes; final NO |
| netcode-architect: R2 unit sync model and lessons | background | WebFetch (redirect notice) | www.gdcvault.com | gdconf.com | 1 | yes; final NO |
| netcode-architect: R2 unit sync model and lessons | background | WebSearch |  |  | 2 | n/a |
| netcode-architect: R2 unit sync model and lessons | background | curl | (proxy status) |  | 1 | n/a |
| netcode-architect: R2 unit tick and bandwidth budgets | background | WebFetch | datatracker.ietf.org |  | 2 | yes |
| netcode-architect: R2 unit tick and bandwidth budgets | background | WebFetch | developer.valvesoftware.com |  | 3 | yes |
| netcode-architect: R2 unit tick and bandwidth budgets | background | WebFetch | gafferongames.com |  | 8 | yes |
| netcode-architect: R2 unit tick and bandwidth budgets | background | WebFetch | gdcvault.com |  | 1 | yes |
| netcode-architect: R2 unit tick and bandwidth budgets | background | WebFetch | raw.githubusercontent.com |  | 15 | yes |
| netcode-architect: R2 unit tick and bandwidth budgets | background | WebFetch | w3c.github.io |  | 2 | yes |
| netcode-architect: R2 unit tick and bandwidth budgets | background | WebFetch | www.rfc-editor.org |  | 10 | yes |
| netcode-architect: R2 unit tick and bandwidth budgets | background | WebFetch (redirect notice) | gdcvault.com | gdconf.com | 1 | yes; final NO |
| netcode-architect: R2 unit tick and bandwidth budgets | background | WebSearch |  |  | 1 | n/a |
| netcode-architect: r2-fix | wf_7dd61ce0-f84 | WebFetch | developer.valvesoftware.com |  | 1 | yes |
| netcode-architect: r2-fix | wf_7dd61ce0-f84 | WebSearch |  |  | 1 | n/a |
| netcode-architect: r2-fix | wf_7dd61ce0-f84 | curl | (from a file or variable) |  | 1 | n/a |
| netcode-architect: r2-fix | wf_7dd61ce0-f84 | curl | cheatsheetseries.owasp.org |  | 1 | yes |
| netcode-architect: r2-fix | wf_7dd61ce0-f84 | curl | docs.rs |  | 2 | yes |
| netcode-architect: r2-fix | wf_7dd61ce0-f84 | curl | raw.githubusercontent.com |  | 2 | yes |
| netcode-architect: r2-fix | wf_7dd61ce0-f84 | curl | rustsec.org |  | 3 | yes |
| netcode-architect: r2-fix | wf_7dd61ce0-f84 | curl | www.rfc-editor.org |  | 1 | yes |
| rust-reviewer: rust-reviewer-r1-r2 | wf_38e94d0f-403 | WebFetch | blog.rust-lang.org |  | 1 | yes |
| rust-reviewer: rust-reviewer-r1-r2 | wf_38e94d0f-403 | WebFetch | crates.io |  | 8 | yes |
| rust-reviewer: rust-reviewer-r1-r2 | wf_38e94d0f-403 | WebFetch | doc.rust-lang.org |  | 18 | yes |
| rust-reviewer: rust-reviewer-r1-r2 | wf_38e94d0f-403 | WebFetch | docs.rs |  | 65 | yes |
| rust-reviewer: rust-reviewer-r1-r2 | wf_38e94d0f-403 | WebFetch | github.com |  | 10 | yes |
| rust-reviewer: rust-reviewer-r1-r2 | wf_38e94d0f-403 | WebFetch | rapier.rs |  | 2 | yes |
| rust-reviewer: rust-reviewer-r1-r2 | wf_38e94d0f-403 | WebFetch | raw.githubusercontent.com |  | 28 | yes |
| rust-reviewer: rust-reviewer-r1-r2 | wf_38e94d0f-403 | WebFetch | rust-lang.github.io |  | 1 | yes |
| rust-reviewer: rust-reviewer-r1-r2 | wf_38e94d0f-403 | WebFetch | rustsec.org |  | 30 | yes |
| rust-reviewer: rust-reviewer-r1-r2 | wf_38e94d0f-403 | WebFetch | rustwasm.github.io |  | 2 | yes |
| rust-reviewer: rust-reviewer-r1-r2 | wf_38e94d0f-403 | WebFetch | webassembly.github.io |  | 3 | yes |
| security-auditor: security-auditor-r2 | wf_bce24fc0-5c6 | WebFetch | cheatsheetseries.owasp.org |  | 2 | yes |
| security-auditor: security-auditor-r2 | wf_bce24fc0-5c6 | WebFetch | datatracker.ietf.org |  | 1 | yes |
| security-auditor: security-auditor-r2 | wf_bce24fc0-5c6 | WebFetch | developer.chrome.com |  | 1 | yes |
| security-auditor: security-auditor-r2 | wf_bce24fc0-5c6 | WebFetch | developer.mozilla.org |  | 3 | yes |
| security-auditor: security-auditor-r2 | wf_bce24fc0-5c6 | WebFetch | doc.rust-lang.org |  | 1 | yes |
| security-auditor: security-auditor-r2 | wf_bce24fc0-5c6 | WebFetch | docs.aws.amazon.com |  | 1 | yes |
| security-auditor: security-auditor-r2 | wf_bce24fc0-5c6 | WebFetch | docs.rs |  | 5 | yes |
| security-auditor: security-auditor-r2 | wf_bce24fc0-5c6 | WebFetch | gafferongames.com |  | 2 | yes |
| security-auditor: security-auditor-r2 | wf_bce24fc0-5c6 | WebFetch | learn.microsoft.com |  | 1 | yes |
| security-auditor: security-auditor-r2 | wf_bce24fc0-5c6 | WebFetch | raw.githubusercontent.com |  | 14 | yes |
| security-auditor: security-auditor-r2 | wf_bce24fc0-5c6 | WebFetch | www.rfc-editor.org |  | 4 | yes |
| security-auditor: security-auditor-r2 | wf_bce24fc0-5c6 | WebFetch | www.w3.org |  | 3 | yes |
| test-engineer: R5 owner: toolchain and quality | background | WebFetch | blog.rust-lang.org |  | 1 | yes |
| test-engineer: R5 owner: toolchain and quality | background | WebFetch | doc.rust-lang.org |  | 9 | yes |
| test-engineer: R5 owner: toolchain and quality | background | WebFetch | docs.npmjs.com |  | 1 | yes |
| test-engineer: R5 owner: toolchain and quality | background | WebFetch | docs.rs |  | 9 | yes |
| test-engineer: R5 owner: toolchain and quality | background | WebFetch | embarkstudios.github.io |  | 3 | yes |
| test-engineer: R5 owner: toolchain and quality | background | WebFetch | eslint.org |  | 5 | yes |
| test-engineer: R5 owner: toolchain and quality | background | WebFetch | gafferongames.com |  | 1 | yes |
| test-engineer: R5 owner: toolchain and quality | background | WebFetch | github.com |  | 10 | yes |
| test-engineer: R5 owner: toolchain and quality | background | WebFetch | playwright.dev |  | 5 | yes |
| test-engineer: R5 owner: toolchain and quality | background | WebFetch | raw.githubusercontent.com |  | 11 | yes |
| test-engineer: R5 owner: toolchain and quality | background | WebFetch | rust-lang.github.io |  | 3 | yes |
| test-engineer: R5 owner: toolchain and quality | background | WebFetch | rustwasm.github.io |  | 2 | yes |
| test-engineer: R5 owner: toolchain and quality | background | WebFetch | typescript-eslint.io |  | 6 | yes |
| test-engineer: R5 owner: toolchain and quality | background | WebFetch | vite.dev |  | 6 | yes |
| test-engineer: R5 owner: toolchain and quality | background | WebFetch | vitest.dev |  | 4 | yes |
| test-engineer: R5 owner: toolchain and quality | background | WebFetch | www.typescriptlang.org |  | 4 | yes |
| test-engineer: R5 owner: toolchain and quality | background | WebFetch (redirect notice) | rust-lang.github.io | clippy.rust-lang.org | 1 | yes; final NO |
| test-engineer: R5 owner: toolchain and quality | background | WebSearch |  |  | 2 | n/a |
| test-engineer: r5-fix | wf_888e5b21-090 | WebFetch | github.com |  | 1 | yes |
| verifier: final check r356 | wf_6ff21603-bee | curl | (from a file or variable) |  | 1 | n/a |
| verifier: r1-final-check | wf_0ad9f0c0-59d | WebFetch | developer.mozilla.org |  | 2 | yes |
| verifier: r1-final-check | wf_0ad9f0c0-59d | WebFetch | doc.rust-lang.org |  | 2 | yes |
| verifier: r1-final-check | wf_0ad9f0c0-59d | WebFetch | docs.rs |  | 1 | yes |
| verifier: r1-final-check | wf_0ad9f0c0-59d | WebFetch | webassembly.github.io |  | 1 | yes |
| verifier: r1-second-pass | wf_64d0bbcd-730 | WebFetch | developer.chrome.com |  | 1 | yes |
| verifier: r1-second-pass | wf_64d0bbcd-730 | WebFetch | developer.mozilla.org |  | 4 | yes |
| verifier: r1-second-pass | wf_64d0bbcd-730 | WebFetch | doc.rust-lang.org |  | 5 | yes |
| verifier: r1-second-pass | wf_64d0bbcd-730 | WebFetch | docs.rs |  | 15 | yes |
| verifier: r1-second-pass | wf_64d0bbcd-730 | WebFetch | github.com |  | 1 | yes |
| verifier: r1-second-pass | wf_64d0bbcd-730 | WebFetch | rapier.rs |  | 1 | yes |
| verifier: r1-second-pass | wf_64d0bbcd-730 | WebFetch | raw.githubusercontent.com |  | 5 | yes |
| verifier: r1-second-pass | wf_64d0bbcd-730 | WebFetch | rust-lang.github.io |  | 1 | yes |
| verifier: r1-second-pass | wf_64d0bbcd-730 | WebFetch | webassembly.github.io |  | 1 | yes |
| verifier: r2-final-check | wf_0ad9f0c0-59d | WebFetch | datatracker.ietf.org |  | 1 | yes |
| verifier: r2-final-check | wf_0ad9f0c0-59d | WebFetch | developer.mozilla.org |  | 1 | yes |
| verifier: r2-final-check | wf_0ad9f0c0-59d | WebFetch | raw.githubusercontent.com |  | 8 | yes |
| verifier: r2-final-check | wf_0ad9f0c0-59d | WebFetch | rustsec.org |  | 10 | yes |
| verifier: r2-final-check | wf_0ad9f0c0-59d | WebFetch | w3c.github.io |  | 1 | yes |
| verifier: r2-final-check | wf_0ad9f0c0-59d | WebFetch | webkit.org |  | 2 | yes |
| verifier: r2-final-check | wf_0ad9f0c0-59d | WebFetch | www.w3.org |  | 5 | yes |
| verifier: r2-second-pass | wf_64d0bbcd-730 | WebFetch | cheatsheetseries.owasp.org |  | 2 | yes |
| verifier: r2-second-pass | wf_64d0bbcd-730 | WebFetch | datatracker.ietf.org |  | 2 | yes |
| verifier: r2-second-pass | wf_64d0bbcd-730 | WebFetch | doc.rust-lang.org |  | 1 | yes |
| verifier: r2-second-pass | wf_64d0bbcd-730 | WebFetch | docs.rs |  | 8 | yes |
| verifier: r2-second-pass | wf_64d0bbcd-730 | WebFetch | gafferongames.com |  | 2 | yes |
| verifier: r2-second-pass | wf_64d0bbcd-730 | WebFetch | raw.githubusercontent.com |  | 8 | yes |
| verifier: r2-second-pass | wf_64d0bbcd-730 | WebFetch | rustsec.org |  | 13 | yes |
| verifier: r2-second-pass | wf_64d0bbcd-730 | WebFetch | www.rfc-editor.org |  | 3 | yes |
| verifier: r2-second-pass | wf_64d0bbcd-730 | WebFetch | www.w3.org |  | 5 | yes |
| verifier: r3-second-pass | wf_7260c3bd-5b1 | WebFetch | developer.chrome.com |  | 1 | yes |
| verifier: r3-second-pass | wf_7260c3bd-5b1 | WebFetch | github.com |  | 2 | yes |
| verifier: r3-second-pass | wf_7260c3bd-5b1 | WebFetch | raw.githubusercontent.com |  | 1 | yes |
| verifier: r3-second-pass | wf_7260c3bd-5b1 | WebFetch | threejs.org |  | 2 | yes |
| verifier: r3-spot-check | wf_1f477dcb-7cb | WebFetch | developer.chrome.com |  | 5 | yes |
| verifier: r3-spot-check | wf_1f477dcb-7cb | WebFetch | developer.mozilla.org |  | 5 | yes |
| verifier: r3-spot-check | wf_1f477dcb-7cb | WebFetch | github.com |  | 3 | yes |
| verifier: r3-spot-check | wf_1f477dcb-7cb | WebFetch | raw.githubusercontent.com |  | 12 | yes |
| verifier: r3-spot-check | wf_1f477dcb-7cb | WebFetch | threejs.org |  | 8 | yes |
| verifier: r3-spot-check | wf_1f477dcb-7cb | WebFetch | web.dev |  | 1 | yes |
| verifier: r3-spot-check | wf_1f477dcb-7cb | WebFetch | www.w3.org |  | 4 | yes |
| verifier: r4-second-pass | wf_50c96d66-3f6 | WebFetch | dimforge.com |  | 1 | yes |
| verifier: r4-second-pass | wf_50c96d66-3f6 | WebFetch | doc.rust-lang.org |  | 1 | yes |
| verifier: r4-second-pass | wf_50c96d66-3f6 | WebFetch | docs.rs |  | 4 | yes |
| verifier: r4-second-pass | wf_50c96d66-3f6 | WebFetch | rapier.rs |  | 1 | yes |
| verifier: r4-second-pass | wf_50c96d66-3f6 | WebFetch | raw.githubusercontent.com |  | 3 | yes |
| verifier: r4-spot-check | wf_d3f13cf7-77e | WebFetch | dimforge.com |  | 1 | yes |
| verifier: r4-spot-check | wf_d3f13cf7-77e | WebFetch | doc.rust-lang.org |  | 2 | yes |
| verifier: r4-spot-check | wf_d3f13cf7-77e | WebFetch | docs.rs |  | 17 | yes |
| verifier: r4-spot-check | wf_d3f13cf7-77e | WebFetch | github.com |  | 2 | yes |
| verifier: r4-spot-check | wf_d3f13cf7-77e | WebFetch | rapier.rs |  | 8 | yes |
| verifier: r4-spot-check | wf_d3f13cf7-77e | WebFetch | raw.githubusercontent.com |  | 14 | yes |
| verifier: r5-second-pass | wf_22f1f23c-b99 | WebFetch | docs.rs |  | 2 | yes |
| verifier: r5-second-pass | wf_22f1f23c-b99 | WebFetch | github.com |  | 3 | yes |
| verifier: r5-second-pass | wf_22f1f23c-b99 | WebFetch | playwright.dev |  | 2 | yes |
| verifier: r5-second-pass | wf_22f1f23c-b99 | WebFetch | typescript-eslint.io |  | 2 | yes |
| verifier: r5-second-pass | wf_22f1f23c-b99 | WebFetch | vitest.dev |  | 1 | yes |
| verifier: r5-spot-check | wf_888e5b21-090 | WebFetch | blog.rust-lang.org |  | 2 | yes |
| verifier: r5-spot-check | wf_888e5b21-090 | WebFetch | developer.mozilla.org |  | 1 | yes |
| verifier: r5-spot-check | wf_888e5b21-090 | WebFetch | developers.cloudflare.com |  | 4 | yes |
| verifier: r5-spot-check | wf_888e5b21-090 | WebFetch | doc.rust-lang.org |  | 5 | yes |
| verifier: r5-spot-check | wf_888e5b21-090 | WebFetch | docs.github.com |  | 5 | yes |
| verifier: r5-spot-check | wf_888e5b21-090 | WebFetch | docs.rs |  | 4 | yes |
| verifier: r5-spot-check | wf_888e5b21-090 | WebFetch | embarkstudios.github.io |  | 1 | yes |
| verifier: r5-spot-check | wf_888e5b21-090 | WebFetch | eslint.org |  | 1 | yes |
| verifier: r5-spot-check | wf_888e5b21-090 | WebFetch | gafferongames.com |  | 1 | yes |
| verifier: r5-spot-check | wf_888e5b21-090 | WebFetch | github.blog |  | 2 | yes |
| verifier: r5-spot-check | wf_888e5b21-090 | WebFetch | github.com |  | 7 | yes |
| verifier: r5-spot-check | wf_888e5b21-090 | WebFetch | playwright.dev |  | 3 | yes |
| verifier: r5-spot-check | wf_888e5b21-090 | WebFetch | raw.githubusercontent.com |  | 4 | yes |
| verifier: r5-spot-check | wf_888e5b21-090 | WebFetch | rustwasm.github.io |  | 1 | yes |
| verifier: r5-spot-check | wf_888e5b21-090 | WebFetch | typescript-eslint.io |  | 4 | yes |
| verifier: r5-spot-check | wf_888e5b21-090 | WebFetch | vite.dev |  | 3 | yes |
| verifier: r5-spot-check | wf_888e5b21-090 | WebFetch | vitest.dev |  | 1 | yes |
| verifier: r6-second-pass | wf_d5608bdb-da9 | WebFetch | developers.cloudflare.com |  | 1 | yes |
| verifier: r6-second-pass | wf_d5608bdb-da9 | WebFetch | docs.sentry.io |  | 2 | yes |
| verifier: r6-second-pass | wf_d5608bdb-da9 | WebFetch | fly.io |  | 1 | yes |
| verifier: r6-second-pass | wf_d5608bdb-da9 | WebFetch | grafana.com |  | 1 | yes |
| verifier: r6-second-pass | wf_d5608bdb-da9 | WebFetch | vercel.com |  | 1 | yes |
| verifier: r6-second-pass | wf_d5608bdb-da9 | WebFetch | www.hetzner.com |  | 3 | yes |
| verifier: r6-second-pass | wf_d5608bdb-da9 | WebFetch | www.rfc-editor.org |  | 1 | yes |
| verifier: r6-spot-check | wf_12f840d1-878 | WebFetch | developers.cloudflare.com |  | 3 | yes |
| verifier: r6-spot-check | wf_12f840d1-878 | WebFetch | docs.hetzner.com |  | 1 | yes |
| verifier: r6-spot-check | wf_12f840d1-878 | WebFetch | docs.rs |  | 1 | yes |
| verifier: r6-spot-check | wf_12f840d1-878 | WebFetch | docs.sentry.io |  | 3 | yes |
| verifier: r6-spot-check | wf_12f840d1-878 | WebFetch | fly.io |  | 1 | yes |
| verifier: r6-spot-check | wf_12f840d1-878 | WebFetch | github.com |  | 1 | yes |
| verifier: r6-spot-check | wf_12f840d1-878 | WebFetch | grafana.com |  | 2 | yes |
| verifier: r6-spot-check | wf_12f840d1-878 | WebFetch | prometheus.io |  | 1 | yes |
| verifier: r6-spot-check | wf_12f840d1-878 | WebFetch | www.hetzner.com |  | 4 | yes |
| verifier: r6-spot-check | wf_12f840d1-878 | WebFetch | www.rfc-editor.org |  | 2 | yes |
| verifier: verifier r14 | wf_5519b651-63d | curl | doc.rust-lang.org |  | 1 | yes |
| verifier: verifier r14 | wf_5519b651-63d | curl | raw.githubusercontent.com |  | 1 | yes |
| verifier: verifier r14 | wf_5519b651-63d | curl (final URL printed) |  | doc.rust-lang.org | 1 | n/a; final yes |
| verifier: verifier r14 | wf_5519b651-63d | curl (final URL printed) |  | raw.githubusercontent.com | 1 | n/a; final yes |
| verifier: verifier r2 | wf_efa16a80-44a | curl | (from a file or variable) |  | 1 | n/a |
| verifier: verifier r2 | wf_efa16a80-44a | curl | aws.amazon.com |  | 1 | yes |
| verifier: verifier r2 | wf_efa16a80-44a | curl | www.hetzner.com |  | 2 | yes |
| verifier: verifier r2 | wf_efa16a80-44a | curl (final URL printed) |  | aws.amazon.com | 1 | n/a; final yes |
| verifier: verifier r2 | wf_efa16a80-44a | curl (final URL printed) |  | www.hetzner.com | 2 | n/a; final yes |
| verifier: verifier r356 | wf_1359fb87-b9e | WebFetch | developers.cloudflare.com |  | 1 | yes |
| verifier: verifier r356 | wf_1359fb87-b9e | curl | developers.cloudflare.com |  | 1 | yes |
| verifier: verifier r356 | wf_1359fb87-b9e | curl (final URL printed) |  | developers.cloudflare.com | 1 | n/a; final yes |
| verifier: verifier-r1-abilities | wf_3fe63170-264 | curl | (from a file or variable) |  | 1 | n/a |
| verifier: verifier-r1-abilities | wf_3fe63170-264 | curl | doc.rust-lang.org |  | 1 | yes |
| verifier: verifier-r1-abilities | wf_3fe63170-264 | curl | raw.githubusercontent.com |  | 4 | yes |
| verifier: verifier-r1-boundary-worker | wf_3fe63170-264 | WebFetch | developer.mozilla.org |  | 13 | yes |
| verifier: verifier-r1-boundary-worker | wf_3fe63170-264 | WebFetch | doc.rust-lang.org |  | 2 | yes |
| verifier: verifier-r1-boundary-worker | wf_3fe63170-264 | WebFetch | docs.rs |  | 3 | yes |
| verifier: verifier-r1-boundary-worker | wf_3fe63170-264 | WebFetch | github.com |  | 2 | yes |
| verifier: verifier-r1-boundary-worker | wf_3fe63170-264 | WebFetch | raw.githubusercontent.com |  | 13 | yes |
| verifier: verifier-r1-boundary-worker | wf_3fe63170-264 | WebFetch | rustwasm.github.io |  | 1 | yes |
| verifier: verifier-r1-boundary-worker | wf_3fe63170-264 | WebFetch | vite.dev |  | 2 | yes |
| verifier: verifier-r1-boundary-worker | wf_3fe63170-264 | WebFetch | web.dev |  | 1 | yes |
| verifier: verifier-r1-boundary-worker | wf_3fe63170-264 | WebFetch | webassembly.github.io |  | 1 | yes |
| verifier: verifier-r1-data-hotreload | wf_3fe63170-264 | WebFetch | cheatsheetseries.owasp.org |  | 3 | yes |
| verifier: verifier-r1-data-hotreload | wf_3fe63170-264 | WebFetch | datatracker.ietf.org |  | 1 | yes |
| verifier: verifier-r1-data-hotreload | wf_3fe63170-264 | WebFetch | developer.mozilla.org |  | 6 | yes |
| verifier: verifier-r1-data-hotreload | wf_3fe63170-264 | WebFetch | doc.rust-lang.org |  | 1 | yes |
| verifier: verifier-r1-data-hotreload | wf_3fe63170-264 | WebFetch | docs.rs |  | 10 | yes |
| verifier: verifier-r1-data-hotreload | wf_3fe63170-264 | WebFetch | github.com |  | 3 | yes |
| verifier: verifier-r1-data-hotreload | wf_3fe63170-264 | WebFetch | raw.githubusercontent.com |  | 5 | yes |
| verifier: verifier-r1-data-hotreload | wf_3fe63170-264 | WebFetch | vite.dev |  | 2 | yes |
| verifier: verifier-r1-determinism | wf_3fe63170-264 | WebFetch | developer.mozilla.org |  | 2 | yes |
| verifier: verifier-r1-determinism | wf_3fe63170-264 | WebFetch | doc.rust-lang.org |  | 7 | yes |
| verifier: verifier-r1-determinism | wf_3fe63170-264 | WebFetch | docs.rs |  | 6 | yes |
| verifier: verifier-r1-determinism | wf_3fe63170-264 | WebFetch | gafferongames.com |  | 2 | yes |
| verifier: verifier-r1-determinism | wf_3fe63170-264 | WebFetch | rapier.rs |  | 1 | yes |
| verifier: verifier-r1-determinism | wf_3fe63170-264 | WebFetch | raw.githubusercontent.com |  | 8 | yes |
| verifier: verifier-r1-determinism | wf_3fe63170-264 | WebFetch | rust-lang.github.io |  | 1 | yes |
| verifier: verifier-r1-determinism | wf_3fe63170-264 | WebFetch | webassembly.github.io |  | 4 | yes |
| verifier: verifier-r1-ecs | wf_3fe63170-264 | WebFetch | blog.rust-lang.org |  | 1 | yes |
| verifier: verifier-r1-ecs | wf_3fe63170-264 | WebFetch | doc.rust-lang.org |  | 8 | yes |
| verifier: verifier-r1-ecs | wf_3fe63170-264 | WebFetch | docs.rs |  | 10 | yes |
| verifier: verifier-r1-ecs | wf_3fe63170-264 | WebFetch | github.com |  | 7 | yes |
| verifier: verifier-r1-ecs | wf_3fe63170-264 | WebFetch | rapier.rs |  | 1 | yes |
| verifier: verifier-r1-ecs | wf_3fe63170-264 | WebFetch | raw.githubusercontent.com |  | 3 | yes |
| verifier: verifier-r1-ecs | wf_3fe63170-264 | WebFetch | rustwasm.github.io |  | 1 | yes |
| verifier: verifier-r1-tick | wf_3fe63170-264 | curl | (from a file or variable) |  | 2 | n/a |
| verifier: verifier-r1-tick | wf_3fe63170-264 | curl | developer.mozilla.org |  | 1 | yes |
| verifier: verifier-r2 | wf_bce24fc0-5c6 | curl | (from a file or variable) |  | 1 | n/a |
| verifier: verifier-r2 | wf_bce24fc0-5c6 | curl | (proxy status) |  | 1 | n/a |

## 4. The check scripts' outputs at 85f89ea

### The glossary check (condition 9): `glossary_check.py --skip-records`

```
entries 34; exact hits 0; in-context hits 617 (quotes and dated records skipped)
  R1: exact 0, in context 129, header links GLOSSARY.md: True; skipped: 24 in quoted text, 58 in dated records; blocks with an unpaired straight quote: 34
  R2: exact 0, in context 201, header links GLOSSARY.md: True; skipped: 41 in quoted text, 61 in dated records; blocks with an unpaired straight quote: 0
  R3: exact 0, in context 82, header links GLOSSARY.md: True; skipped: 16 in quoted text, 35 in dated records; blocks with an unpaired straight quote: 0
  R4: exact 0, in context 56, header links GLOSSARY.md: True; skipped: 9 in quoted text, 20 in dated records; blocks with an unpaired straight quote: 0
  R5: exact 0, in context 49, header links GLOSSARY.md: True; skipped: 14 in quoted text, 29 in dated records; blocks with an unpaired straight quote: 0
  R6: exact 0, in context 100, header links GLOSSARY.md: True; skipped: 12 in quoted text, 33 in dated records; blocks with an unpaired straight quote: 0
  not searchable (a pattern in words; the step-3d verifiers read for it): The hash-map ban in the sim crate / context: "zero uses of std HashMap or HashSet iteration in … outcome code"
  not searchable (a pattern in words; the step-3d verifiers read for it): The stress scene / exact: "the worst case (48 players plus vehicles and projectiles …)"
```

### The approved changes found in their files: `apply_check.py`

```
items 143; to read by hand 14
```

The items to read by hand are the ones whose after text a later glossary rename, link or ruling changed; the step-3d verifiers read each (their reports, `verifier-*-3d-result.md`).

### Quoted source text: `quotes_check.py 7c6e0d7 85f89ea`

```
R1: quotes at base 1034, at head 1048; gone 0, new 14
R2: quotes at base 834, at head 864; gone 0, new 27
R3: quotes at base 341, at head 358; gone 0, new 11
R4: quotes at base 361, at head 371; gone 0, new 10
R5: quotes at base 572, at head 593; gone 0, new 20
R6: quotes at base 254, at head 266; gone 0, new 12
```

### UNREAD_SOURCES.md: `build_unread.py`'s check

```
{
 "sha": "8dee746",
 "tables": 8,
 "rows": 187,
 "merged": 180,
 "distinct_urls": 148,
 "hinge_rows": 67,
 "dup_or_missing": [],
 "by_need": {
  "note": 24,
  "hinge": 44,
  "none": 108,
  "named": 4
 }
}```

## 5. The steps with their commits (the checkpoint, folded in; CHECKPOINTS §9)

| Step | What | Commits |
|---|---|---|
| Start | Branch from main 07ab240 after R7 (PR #27); plan and verdicts recovered word for word | 8ff1c74 |
| Wave 1 | R1 and R2: 11 units, assembled; six R1 verifiers, the R2 verifier, rust-reviewer, security-auditor; fix pass; second verifier pass; second fix round and final check: both verified | a6a97fe, 7907f5c, e4cda31, ae492a6 |
| Wave 2 | R3–R6 owners; a spot-check, an owner fix and a second pass each | R3 605c09d → c22675f; R4 abdf477 → 16212b6; R5 9a623a0 → 8948522; R6 9d5e78a → 1decc70 |
| Main merged | PR #29's STATUS and ROADMAP brought in before the docs step | 569f792 |
| Critic | The consistency critic: 41 items (12 contradictions) | in `.checkpoints/` (engine-architect-consistency-result.json) |
| The Advisor's 2026-10-30 decisions | UNREAD_SOURCES.md; the glossary; the 12 contradictions (steps 3a–3d) | 82298ab |
| Step 3a/3b | Glossary and resolutions drafted; the advisor's verdict folded word for word | e797518, 8909523 |
| Retire split | Format-only split, ruled by the advisor | 7c6e0d7 |
| Step 3c | Owners apply, one per file; OBJECT-1 ruled and applied; the glossary's Used today refresh | 1da4e5f, de0a003, 41f9cf9, 2684735, e9ff1f9, 09de883, 36cc386, f67245e |
| Step 3d | Three verifier units with skeptics; fix rounds; three final checks; the completeness critic and its nits | d768190, 10662c4, 1b69364, d54f3c0, 902c249, 1bfd235, f2239f9, 8260592, 3aad9d8, 96a4c29, 2382ea2, 7bb3d2e, b92199c, 749b363, 910768b |
| Docs | docs-keeper (STATUS, ROADMAP, README, REPO_MAP); the report; the gate | see the PR's last commits |

Spend-limit cuts (all resumed from the agents' notes, nothing lost): 2026-10-27 about 18:25 UTC; 2026-10-28 about 16:26
and 20:50 UTC; 2026-10-30 about 18:52 and 21:04 UTC.
