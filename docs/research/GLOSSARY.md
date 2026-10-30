# Glossary: shared terms for the v2 research topics (R1–R6)

- **Purpose:** one definition per term that the six topics (`r1-simulation-core.md` to `r6-infrastructure.md`) use
  with different meanings today, so that every pass mark, budget and open question means the same thing in every
  topic. It starts with the tick budgets, then the terms the critic's 12 contradictions turn on (the stress scene, the
  mode sizes and the largest mode, the reference server, preview isolation and the like). It was asked for by the
  Advisor on 2026-10-30 ("Add a shared glossary (docs/research/GLOSSARY.md) with one definition per term, starting
  with the tick budgets, and make every topic use it").
- **Date:** 2026-10-30.
- **Author:** engine-architect (step 3a of `.checkpoints/v2-research/plan-amendments-2.md`).
- **Status:** terms approved 2026-10-30 (advisor, step 3b; its changes folded in by the session). Nothing here is applied to a topic
  yet; the owners apply the approved terms in step 3c. Every number here comes from a topic file (its id is given), is
  arithmetic on those numbers (shown), or is a pass mark marked "proposed" with its reason. No new source was read.

## How to read an entry

Each entry has these parts:
- **Definition:** the one meaning the topics use from now on.
- **Marked by:** the words that mark the term in a topic's text.
- **Retire (exact)** and **Retire (in context):** synonyms and loose wordings that stop being used, outside quoted
  source text, which never changes. The topics' dated records (bracketed fix-pass notes, verifier tables, "settled at
  the fix step" notes) also keep their words, since they record what a text said; the search script of condition 9
  should skip them as it skips quotes (a proposal for the advisor). A "Retire (exact)" phrase is wrong wherever else it
  appears (none of its hits on 2026-10-30 is a correct use). A "Retire (in context)" phrase is retired only where it
  carries the term's meaning, as its context words say.
- **Keeps:** words the text keeps that the Retire field quoted before 2026-10-30 (kept phrases, marking words,
  replacement words), written without quotation marks so the search script does not count them.
- **Depends on:** only where the definition turns on a decision not yet taken. The entry then states every reading
  with its decision code and picks none.
- **Used today:** per topic, the ids whose text uses the term or one of its retired synonyms today, and how. The lists
  are a first pass from a search of each topic file on 2026-10-30; after step 3c a search script refreshes them
  (advisor condition 9). Where the text has no id, a section name or a line reference (for example r2:3245, at commit
  8dee746) stands in.

Decision codes used below: **MODES-3** (Mustafa: VISION_V2 §3's starting sizes, totals or per team), **PREVIEW-WALL**
(Mustafa: an access wall on previews that reach the staging server), **BASELINE-WHEN** (the Advisor: research the
deferred security baseline before ADR 0006, or list it open), **SEC-SPREAD** (Mustafa: matching tracers against cheat
resistance, R2-SYNC-OQ11), **ABL-PREDICT** (Mustafa, after a Feel Lab test: does the client predict its own
abilities), **PREV-DELETE** (a spike: does a closed PR's preview really disappear). In step 3c their full wording
goes into each affected topic's "Shared open questions" subsection; until then it is in
`.checkpoints/v2-research/resolutions.md`.

## 1. Tick budgets

The terms below make one budget out of the figures the topics give today. The budget itself (its total and its two
shares) is resolution 1, decided by the advisor on 2026-10-30; every figure inside it is a topic's own mark, restated
against the defined whole. The check below adds each part's ceiling: a planning check that the marks fit together, not
a bound (see "Time statistics"); the server tick time's p99 is always measured directly.

| Part (N = 48) | Mark and id | 60 Hz, ms | 30 Hz, ms |
|---|---|---|---|
| movement and physics | ≤ 1 ms mean, R4-PHYS-REC1 mark 2 (mean mark; its p99 is reported, not capped) | 1.00 | 1.00 |
| avoidance | ≤ 0.2 ms p99, R4-NAV-REC3 | 0.20 | 0.20 |
| abilities, effects, weapons | ≤ 1 ms, R1-ABL-REC6 | 1.00 | 1.00 |
| navigation | 5 % of the period, R4-NAV-REC2 | 0.83 | 1.67 |
| visibility pass | 5 % of the period, R2-INT-REC3 | 0.83 | 1.67 |
| **sim step share, sum (limit)** | | **3.87 (4.17)** | **5.53 (8.33)** |
| snapshot encoding | 48 × 50 µs, R2-BUD-REC6 | 2.40 | 2.40 |
| lag compensation | 700 shots/s × 20 µs, R2-SYNC-REC5 mark 4 | 0.23 | 0.47 |
| interest | 0.2 + 0.5 ms, R2-INT-REC1, R2-INT-REC4 | 0.70 | 0.70 |
| enclosure pair test | 0.1 µs × 768 enemy pairs, R2-INT-REC2 (b) | 0.08 | 0.08 |
| **server-only share, sum (limit)** | | **3.41 (4.17)** | **3.64 (8.33)** |
| re-flood after a portal change (that tick only) | +1.00 ms, R2-INT-REC2 (b) | +1.00 | +1.00 |
| server-only share on a re-flood tick | | 4.41 | 4.64 |
| **server tick, sum (limit 50 %)** | | **7.28 (8.33)** | **9.18 (16.67)** |
| server tick on a re-flood tick | | 8.28 | 10.18 |

Snapshots run at 30 Hz (R2-BUD-REC1), so at a 60 Hz tick every second tick encodes. Left at 60 Hz: 0.30 ms in the sim
step share, 0.76 ms in the server-only share (−0.24 on a re-flood tick). At N = 54 (MODES-3 (b); the per-soldier,
per-client and per-shot marks scaled by 1.125 for this check only, 972 enemy pairs): at 60 Hz the sim step share is
4.14 ms (0.03 left), the server-only share 3.85 ms (4.85 on a re-flood tick), the server tick 7.99 ms (8.99 on a
re-flood tick, over 8.33); at 30 Hz 5.81, 4.12 (5.12), 9.93 (10.93) ms. At 30 Hz the marks fit at N = 48 and N = 54,
re-flood ticks included; at 60 Hz they fit on every tick without a re-flood. The 60 Hz tight spots (the visibility
pass, the re-flood tick, the rest of the sim at N = 54) are for the spike to measure. Under 25 % of the period for the
whole tick the same rows give 6.44 ms at 60 Hz (5.24 with encoding spread over two ticks) against 4.17, and 7.51 ms at
30 Hz (8.51 with a re-flood) against 8.33.

### Tick period
- **Definition:** the time between two ticks, 1 / tick rate: 16.7 ms at 60 Hz, 33.3 ms at 30 Hz (r1:2015-2016,
  r2:2378). Every share below is a share of the tick period. The tick rate itself follows R1-TICK-REC2's decision rule
  (60 Hz if P1–P3 pass at 60 Hz, else 30 Hz), which does not change.
- **Marked by:** "the tick period".
- **Retire (exact):** none.
- **Retire (in context):** "the tick interval" for our tick period (R2-BUD-REC6, R2-XU-OQ8); "the period" alone where
  it could mean the snapshot interval.
- **Used today:** R1: R1-TICK-REC2 (P1's "25 % of 16.7 ms", P2's "40 % of the period"), R1-ECS-OQ4. R2: R2-BUD-REC6
  ("tick interval"), R2-ROOM-REC2 ("tick period"), R2-XU-OQ8, the pointers between units (r2:3245). R3–R6: none.

### Sim step time
- **Definition:** the time of one call of the simulation step: every phase of the sim crate's tick (movement and
  physics queries with avoidance, abilities, effects and weapons, AI with navigation, the shared visibility pass,
  projectiles, economy, objectives and moments). It is the same code on the server (native) and in the client's Worker
  (Wasm). On the server it is measured on one vCPU of the reference server; in the client on the minimum laptop.
- **Marked by:** "sim step time", "the sim step".
- **Retire (exact):** "the per-tick sim budget" (R1-DET-REC3).
- **Retire (in context):** "tick time", "mean tick", "p99 tick" and "tick cost" where only the simulation is meant
  (R1-ECS-REC1 mark 1, R1-TICK-REC2 P1 and P2, R1-TICK-REC3, R1-ECS-OQ4, R1-XU-OQ1, R4-PHYS-REC1 mark 2,
  R5-LAYOUT-REC5, the R5 benchmark row in R5-TEST-REC1).
- **Keeps:** Whole ticks for an integer count of ticks (R1-TICK-REC1), a different phrase.
- **Used today:** R1: R1-ECS-REC1, R1-TICK-REC2, R1-TICK-REC3, R1-DET-REC3, R1-ECS-OQ4, R1-TICK-OQ5, R1-BND-OQ2,
  R1-XU-OQ1. R2: the budgets options table E ("Simulation step"), R2-BUD-REC6 ("the simulation step"). R4:
  R4-PHYS-REC1 mark 2. R5: R5-LAYOUT-REC5, R5-TEST-REC1. R3, R6: none.

### Server tick time
- **Definition:** the time one server tick takes on the room's tick thread, from the start of the tick's work to its
  end: the sim step, plus the server-only work of that tick, which is command handling (R2-SYNC-REC2), lag-compensated
  hit tests (R2-SYNC-REC5), interest (the region query, R2-INT-REC1, and ranking and packing, R2-INT-REC4), snapshot
  encoding for every client on a snapshot tick (R2-SYNC-REC1, R2-BUD-REC6) and the tick's non-waiting send calls
  (R2-TRN-REC6). The work of the network runtime on other threads is not in it (see the transport runtime).
- **Marked by:** "server tick time", "server tick p99".
- **Retire (exact):** "whole tick" and "p99 whole tick" (R2-BUD-REC6).
- **Retire (in context):** "a room's tick", "tick p99" and "tick time" where the whole server tick is meant
  (R2-ROOM-REC2, R2-TRN-REC1 mark 4, R2-TRN-REC6 mark 2, R5-TEST-REC5, R6-HOST-REC2, R6-OBS-REC1, R6-OBS-REC2,
  R6-TEL-REC1, R6's options tables and K3).
- **Used today:** R1: R1-TICK-REC2 ("leaves room for snapshot encoding, interest management", r1:2017). R2:
  R2-BUD-REC6, R2-ROOM-REC2, R2-TRN-REC1 mark 4, R2-TRN-REC6 mark 2, R2-ROOM-OQ1, R2-XU-OQ8. R5: R5-TEST-REC5. R6:
  R6-HOST-REC2, R6-OBS-REC1, R6-OBS-REC2, R6-TEL-REC1, the options table Q2 and K3. R3, R4: none.

### Transport runtime time
- **Definition:** the CPU time a room's network runtime uses outside the tick thread (encryption, sockets,
  acknowledgements, receiving datagrams), counted as a share of one vCPU per second. R2-ROOM-REC2 runs "the sim on its
  own thread and network I/O on an async runtime", so this time is not in the server tick time, but it is in the
  rooms-per-VM figure (R2-ROOM-REC2 mark 2, R6-COST-OQ1). R2-TRN-REC1 mark 2 caps it for one process with 48 sessions.
- **Marked by:** "transport runtime time", "the transport runtime".
- **Retire (exact):** "1 CPU core for the transport".
- **Retire (in context):** "the transport layer" as a CPU budget (R2-TRN-REC1 mark 2, R2-XU-OQ8).
- **Used today:** R2: the rooms options (r2:1900), R2-TRN-REC1 mark 2, R2-ROOM-REC2, R2-XU-OQ8. R5: R5-LAYOUT-REC2 and
  R5-GATE-REC1 (G5) name the transport only as a crate the sim must not depend on. R1, R3, R4, R6: none.

### Time statistics
- **Definition:** a time mark that names no statistic is a p99 over the run it names; a mean is always called "mean".
  Shares are compared at p99. Adding the parts' p99 ceilings is a planning check that the marks fit together, not a
  bound: p99s do not add (two parts that each take 1 ms on 0.6 % of ticks and nothing otherwise each have a p99 of 0,
  but together they take 1 ms on about 1.2 % of ticks, so the sum's p99 is 1 ms). The server tick time's p99 is always
  measured directly, with the parts reported beside it.
- **Marked by:** "p99", "mean".
- **Retire (exact):** none.
- **Retire (in context):** a time mark with no statistic, such as "costs at most 1 ms per tick" (R1-ABL-REC6),
  "at most 0.2 ms per tick" (R2-INT-REC1), "at most 0.5 ms per tick" (R2-INT-REC4), and R2-INT-REC2 (b)'s pair test
  ("at most 0.1 µs per enemy pair") and its re-flood: each now reads as a p99.
- **Used today:** every topic that sets time marks (R1, R2, R3, R4, R6); R3's frame marks already name p95 and the
  median.

### The server tick budget
- **Definition:** the p99 server tick time a room may use, on one vCPU of the reference server, tested in the stress
  scene: at most 50 % of the tick period, so 8.3 ms at 60 Hz and 16.7 ms at 30 Hz, with the max under one tick period and zero
  overruns (R1-TICK-REC2 P1's "max at most 16.7 ms, zero overruns"). It is split into the sim step share and the
  server-only share, 25 % of the tick period each. The 50 % is R2-ROOM-REC2's figure; the two 25 % shares are
  R1-TICK-REC2 P1's figure for the sim and R2-SYNC-REC9's for the sync work, read against the tick period.
- **Decided by the advisor, 2026-10-30** (resolution 1; 25 % for the whole tick, R2-BUD-REC6's figure, was weighed and
  not taken). No escalated decision.
- **Marked by:** "the server tick budget".
- **Retire (exact):** "the tick budget", "the tick's p99 budget" (R4-NAV-REC2),
  "the share of the tick budget the tick-rate unit sets" (R2-SYNC-REC1 mark 4).
- **Retire (in context):** "the server tick budget" with no number behind it, "CPU budget per tick" for the whole
  (R2-BUD-REC6), and "25 %" or "50 %" with no named whole.
- **Used today:** R1: R1-TICK-REC2 (25 % for the sim alone), R1-DET-REC3 ("the per-tick sim budget"), R1-XU-OQ1. R2:
  the TL;DR ("R1's CPU mark"), the budgets options table E, R2-SYNC-REC1 mark 4, R2-SYNC-REC9 (25 % "of the tick
  budget"), R2-BUD-REC1 ("its CPU mark"), R2-BUD-REC6 (25 % for the whole tick), R2-INT-REC3 (10 % "of the server tick
  budget"), R2-ROOM-REC2 (50 % of the tick period), R2-XU-OQ8. R4: R4-NAV-REC2 ("the tick's p99 budget"), R4-LOS-REC1
  mark 4 and R4-LOS-REC2 ("10 % budget"), the legacy facts used (r4:526). R6: R6-HOST-REC2 ("the per-tick budget"),
  R6-OBS-REC2, the options table Q2 ("the tick budget"). R3, R5: none (R5-TEST-REC5 names "R1-TICK's budget" and "the
  budget ADR-0004 sets").

### The sim step share and the server-only share
- **Definition:** the two parts of the server tick budget, each at most 25 % of the tick period at p99 (4.2 ms at
  60 Hz, 8.3 ms at 30 Hz):
  - **the sim step share:** the sim step time. Inside it each topic keeps its own mark, reported separately: movement
    and physics queries at most 1 ms native mean (R4-PHYS-REC1 mark 2) with avoidance at most 0.2 ms (R4-NAV-REC3);
    abilities, effects and weapons at most 1 ms (R1-ABL-REC6); navigation at most 5 % of the tick period
    (R4-NAV-REC2's "10 %" of the server tick budget: 0.8 ms at 60 Hz, 1.7 ms at 30 Hz); the shared visibility pass at
    most 5 % of the tick period (R2-INT-REC3's "10 %", the same figures); the rest of the sim takes what is left;
  - **the server-only share:** the server-only work of the server tick time. Inside it: snapshot encoding at most
    50 µs per client per snapshot (R2-BUD-REC6), a rewound shot at most 20 µs at 700 shots per second (R2-SYNC-REC5
    mark 4), the region query at most 0.2 ms (R2-INT-REC1), ranking and packing at most 0.5 ms (R2-INT-REC4), the
    enclosure pair test at most 0.1 µs per enemy pair and, on a tick with a portal change, its re-flood at most 1 ms
    (R2-INT-REC2 (b)), command handling and the tick's send calls (each send call at most 1 ms, R2-TRN-REC1 mark 2;
    per call, not per tick: the tick's sends have no per-tick mark and share what is left).
- **Marked by:** "the sim step share", "the server-only share", "a share of the tick period".
- **Retire (exact):** "the share of the tick budget" (R2-SYNC-REC1 mark 4), "25 % of the tick budget" (R2-SYNC-REC9),
  "half of that mark's mean tick" (R4-PHYS-REC1 mark 2).
- **Retire (in context):** "10 % of the server tick budget" or "10 % budget" without its value (R2-INT-REC3,
  R4-NAV-REC2, R4-LOS-REC1, R4-LOS-REC2).
- **Used today:** R1: R1-TICK-REC2 P1, R1-ABL-REC6, R1-DET-REC3. R2: R2-SYNC-REC1 mark 4, R2-SYNC-REC5 mark 4,
  R2-SYNC-REC9, R2-BUD-REC6, R2-INT-REC1, R2-INT-REC3, R2-INT-REC4, R2-TRN-REC1 mark 2, R2-XU-OQ8. R4: R4-PHYS-REC1
  mark 2, R4-NAV-REC2, R4-NAV-REC3, R4-LOS-REC1 mark 4, R4-LOS-REC2. R3, R5, R6: none.

### The client's sim share
- **Definition:** in the client, the sim step time in Wasm in the Worker on the minimum laptop, at most 40 % of the
  tick period at p99 (6.7 ms at 60 Hz, 13.3 ms at 30 Hz: R1-TICK-REC2 P2), with the page's own cost of the sim at most
  1.0 ms per frame (R1-BND-REC1 mark 1, R3-BUDGET-REC2's row "sim cost seen by the page"). It is a separate budget from
  the server tick budget and is not part of the 50 %.
- **Marked by:** "the client's sim share", "the page's cost of the sim".
- **Retire (exact):** none.
- **Retire (in context):** "tick time" for the client's Worker (R1-TICK-REC2 P2).
- **Used today:** R1: R1-TICK-REC2 P2, R1-BND-REC1, R1-ABL-REC6 (the minimum laptop), R1-ECS-REC1 ("Wasm in a Worker
  ≤ 2× native"). R3: R3-BUDGET-REC2. R4: R4-PHYS-REC1 mark 2, R4-NAV-REC4. R2, R5, R6: none.

### The reference server
- **Definition:** the machine on which every native server pass mark is measured: a Hetzner Cloud CCX23 (4 dedicated
  vCPU, 16 GB, x86-64 AMD), in one EU location, with the shared-vCPU CX33 measured beside it (R6-HOST-REC2, which names
  it as the answer to R2-XU-OQ9). It stays a working pick while R6-HOST-OQ1 (Hetzner's EU plans showed "currently
  unavailable" on 2026-10-28) is open, and the reference hardware as a whole is still approved by Mustafa
  (R1-TICK-OQ6). The client's reference machines are R3-BUDGET-REC1's minimum laptop and recommended desktop.
- **Marked by:** "the reference server".
- **Retire (exact):** "one server core of the R6 host type" (R1-ECS-REC1), "one core of the proposed reference server"
  (R1-TICK-REC2), "the proposed reference server core" (R1-ABL-REC6), "one core of the R6 server type" (r4:658),
  "one vCPU of the EU instance type R6 recommends" (R2-BUD-REC6), "one 4-vCPU EU cloud VM, R6 to name it"
  (R2-ROOM-REC2), "reference server hardware R6 names" (R2-TRN-REC1), "reference host".
- **Retire (in context):** "to be named with R3, R5 and R6" for the server.
- **Used today:** R1: R1-ECS-REC1, R1-TICK-REC2, R1-TICK-REC4 ("the reference server (Linux)"), R1-ABL-REC6,
  R1-TICK-OQ6, R1-XU-OQ12. R2: R2-SYNC-REC1 mark 4, R2-SYNC-REC5 mark 4, R2-SYNC-REC9, R2-BUD-REC6, R2-INT-REC1,
  R2-INT-REC3, R2-INT-REC4, R2-TRN-REC1 mark 2, R2-ROOM-REC2, R2-XU-OQ9 (all "the proposed reference server" or an R6
  pointer). R4: the recommendations' preface (r4:658), R4-PHYS-REC1, R4-NAV-REC1 (via R1-TICK-REC2). R6: R6-HOST-REC2
  (names it), R6-COST-REC1, R6-COST-OQ1, the cost tables, the pointer to R2 (r6:942-943). R3, R5: none.

### The measuring unit: one vCPU
- **Definition:** a server time mark is the elapsed time of that work on one vCPU of the reference server, with no other
  room on that vCPU, unless the mark itself measures neighbours (R2-ROOM-REC2 mark 2: 8 rooms on one host). "A share of
  one vCPU" is the fraction of that vCPU's time the work uses. Rooms per VM and cost per room-hour are counted in the
  same unit (R6-COST A-2: "8 rooms of 48 players per 4-vCPU VM"; r6:452-454). The vCPU is the unit Hetzner sells
  (R6-HOST-F15), and it is not always a whole core: AWS lists a "c6a.large (1 core, 2 vCPU, 4 GB)" (R6-HOST-F23). No
  topic says whether a CCX23 dedicated vCPU is a whole core, so "core" is not used for server marks.
- **Marked by:** "one vCPU", "per vCPU", "a share of one vCPU".
- **Retire (exact):** "one core", "per core", "server core", "1 CPU core": every use today is for server work
  (R1-ECS-REC1, R1-TICK-REC2, R1-ABL-REC6, R1-ECS-OQ4, R1-TICK-OQ6, r1:2955-2956, r4:658, R2-TRN-REC1 mark 2).
- **Retire (in context):** none.
- **Keeps:** Core, for client hardware (at least 4 cores and 8 threads, R3-BUDGET-REC1) and where a source is quoted.
- **Used today:** R1: "core" in R1-ECS-REC1, R1-TICK-REC2, R1-ABL-REC6, R1-ECS-OQ4, R1-TICK-OQ6 and the pointer to R6
  (r1:2955-2956). R2: "vCPU" in R2-BUD-REC6, R2-ROOM-REC2, R2-XU-OQ9; "core" in R2-TRN-REC1 mark 2, R2-BUD-REC6's
  reason ("more than one room per core"), the budgets options table E ("several rooms per core") and the pointers
  between units (r2:3244, "rooms per core"). R4: "core" (r4:658). R6: "vCPU" throughout (R6-HOST-REC2, R6-COST A-2,
  A-3, R6-COST-OQ1, the cost tables); r6:453 reads R1-ECS-OQ4's "one per core" as one per vCPU. R3, R5: none.

## 2. Modes and scale

### Player count (N)
- **Definition:** the number of soldiers in one match, humans and bots together (bots fill empty slots, VISION_V2 §3).
  A mark written "at N = 48" counts both. Vehicles, projectiles and effects are counted apart (see the stress scene).
- **Marked by:** "N", "N = …", "… players" or "… soldiers" in a pass mark or an estimate.
- **Retire (exact):** none.
- **Retire (in context):** "characters" for soldiers (R1-DET-REC3: "48 characters"); a bare "48" where the largest mode
  is meant.
- **Keeps:** write the largest mode's N.
- **Used today:** R1: R1-ECS-REC1 (48 soldiers), R1-TICK-REC2 (N = 12 / 24 / 36 / 48 / 64), R1-TICK-REC3 (N = 48),
  R1-DET-REC1, R1-DET-REC3, R1-BND-REC1 (N = 12, 24, 36, 48), R1-ABL-REC6, R1-BND-OQ6. R2: the worked estimate
  (P = players and bots), R2-SYNC-REC9, R2-BUD-REC6, R2-INT-REC1, R2-TRN-REC1 mark 2, R2-ROOM-REC2. R3:
  R3-BUDGET-REC2, R3-BUDGET-OQ4. R4: R4-PHYS-REC4, R4-NAV-REC2 (N = 48 and 64), R4-NAV-REC3. R5: R5-TEST-REC5. R6: the
  cost tables ("Humans", A-1 to A-5).

### Team size and how modes are written
- **Definition:** the team size is the number of slots on one team. A mode is written per team: "24v24" is two teams of
  24 (N = 48), "12v12v12" three teams of 12 (N = 36); "T × S" is T teams of S slots ("3 × 18" is N = 54). A size that
  counts the whole match says "in all" or gives N.
- **Marked by:** "AvB", "AvBvC", "T × S", "team size", "slots".
- **Retire (exact):** none.
- **Retire (in context):** "2×2"-style labels that do not say whether they count per team (R2-ROOM-REC7), once MODES-3
  is answered; "players" for a team size.
- **Used today:** R1: R1-DET-REC1 (24v24, 12v12v12), R1-TICK-REC2, R1-ABL-REC6 ("48 soldiers (VISION_V2 §3, 24v24)").
  R2: the worked estimate (4v4, 6v6v6, 12v12v12, "24v24 or 16v16v16", 32v32), R2-INT-REC3 (24v24 and 16v16v16),
  R2-ROOM-REC6 ("3 teams × 16 slots"), R2-ROOM-REC7 ("2×2, 2×8, 3×6, 3×18, 12v12v12 and 24v24"), R2-XU-OQ13. R3:
  R3-AUDIO-REC1, R3-BUDGET-OQ4. R4: R4-NAV-REC2, R4-LOS-REC1 mark 4, R4-LOS-REC2 (24v24 and 16v16v16). R5:
  R5-TEST-REC5 (24v24 = 48, 12v12v12 = 36). R6: the egress table (4v4, 6v6v6, 12v12v12, 24v24).

### The start modes and the goal modes
- **Definition:** the goal modes are 24v24 (N = 48) and 12v12v12 (N = 36): VISION_V2 §3, "The goal: 12v12v12 or
  24v24", which every topic reads the same way. The start modes are VISION_V2 §3's "two teams with 2–8 players, and
  three teams with 6–18 players"; which modes those words name depends on MODES-3.
- **Depends on:** MODES-3. Reading (a), totals: 1v1 to 4v4 (N = 2 to 8) and 2v2v2 to 6v6v6 (N = 6 to 18). Reading (b),
  per team: 2v2 to 8v8 (N = 4 to 16) and 6v6v6 to 18v18v18 (N = 18 to 54). This glossary picks neither.
- **Marked by:** "start", "start small", "goal".
- **Retire (exact):** none.
- **Retire (in context):** "the goal" for a start mode; "(start, upper end)" with no reading named.
- **Used today:** R2: the worked estimate (rows "4v4 (VISION_V2 §3 start, upper end)" and "6v6v6 (start, upper end)":
  reading (a)), R2-ROOM-REC7 (reading (b)), R2-BUD-OQ6 and R2-XU-OQ13 (ask the question). R3: R3-BUDGET-OQ4 (quotes
  both). R6: the egress table and the Start scenario (reading (a)). R1, R4, R5: the goals only.

### The largest mode, and its N
- **Definition:** the mode, among the start and goal modes, with the most players; "the largest mode's N" is its
  player count. Every pass mark set "at the largest mode" uses this N.
- **Depends on:** MODES-3. Under (a) the largest mode is the goal 24v24, N = 48. Under (b) it is the start mode
  18v18v18, N = 54 (R2-BUD-OQ6: "per team they would mean up to 16 and 54 players"). This glossary picks neither; a
  mark gives both values until Mustafa answers.
- **Marked by:** "the largest mode", "the largest mode's N".
- **Retire (exact):** none.
- **Retire (in context):** "the largest mode (48 soldiers, 47 of them bots)" (R1-TICK-OQ7) and "Largest N"
  (R3-BUDGET-OQ4) where they fix N at 48; "48" alone where the largest mode is meant.
- **Used today:** R1: R1-TICK-REC2 (P2: "offline at the largest mode (OQ7 decides N)"), R1-DET-REC3 ("the largest
  mode's size (48 characters …)"), R1-TICK-OQ7, R1-DET-OQ1, R1-BND-OQ6, R1-XU-OQ12. R2: none by name (R2-BUD-OQ6 asks
  the question). R3: the budgets findings (r3:534, quoting VISION_V2 §4), R3-CHAR-REC2, R3-BUDGET-REC2,
  R3-MEASURE-REC2, R3-BUDGET-OQ4. R4: R4-NAV-REC4 ("offline play at the largest mode"). R5, R6: none by name.

### The design point and headroom
- **Definition:** the design point is N = 48, the 24v24 goal (VISION_V2 §3), which R2's estimate and R6's cost table
  label "design point 48" and which R3-BUDGET-OQ4 and R5-TEST-REC5 tie to "design for 48+". A run above the largest
  mode's N is a headroom run and is labelled so (for example N = 64: R1-TICK-REC2 P1, R4-NAV-REC2, R2's "32v32 (48+
  headroom)" row).
- **Depends on:** MODES-3, for how the two relate. Under (a) the design point equals the largest mode's N (48). Under
  (b) the largest mode's N (54) is above the design point (R2-BUD-OQ6: "54 is above the 48 design point").
- **Marked by:** "design point", "headroom", "48+".
- **Retire (exact):** none.
- **Retire (in context):** "the largest mode" for the design point.
- **Used today:** R2: the worked estimate ("24v24 or 16v16v16 (goal; design point 48)", "32v32 (48+ headroom)"),
  R2-BUD-OQ6, R2-TRN-REC2. R3: R3-BUDGET-OQ4. R5: R5-TEST-REC5. R6: the egress table ("24v24 (goal, design point 48)").

### The test-mode list
- **Definition:** the one list of modes the spikes run (proposed by resolution 10; it replaces the topics' separate
  lists):
  1. the goal modes, 24v24 (N = 48) and 12v12v12 (N = 36);
  2. 16v16v16 (N = 48), the top of LOOP_V2 §2's three-team range "12v12v12 to 16v16v16, possibly more" (quoted at
     r1:311 and r2:466), which R2-INT-REC3, R4-LOS-REC1 mark 4 and R4-LOS-REC2 already test;
  3. the smallest and the largest start mode of each team count (MODES-3);
  4. the largest mode, where it is not already in the list (MODES-3);
  5. the headroom run, N = 64, labelled headroom.
- **Depends on:** MODES-3, for items 3 and 4. Under (a): 1v1, 4v4, 2v2v2 and 6v6v6, and the largest mode is 24v24
  (item 1). Under (b): 2v2, 8v8, 6v6v6 and 18v18v18, and the largest mode is 18v18v18 (N = 54).
- **Marked by:** "the test-mode list", "the test modes".
- **Retire (exact):** none.
- **Retire (in context):** each topic's own mode list in a pass mark that means "every mode we test"
  (R2-ROOM-REC7's "2×2, 2×8, 3×6, 3×18, 12v12v12 and 24v24"; R5-TEST-REC5's "24v24 = 48 and 12v12v12 = 36").
- **Used today:** R1: R1-DET-REC1 (24v24 and 12v12v12). R2: R2-INT-REC3, R2-ROOM-REC6, R2-ROOM-REC7. R4: R4-LOS-REC1
  mark 4, R4-LOS-REC2. R5: R5-TEST-REC2 (two teams, three teams, bots only), R5-TEST-REC5. R3, R6: none.

## 3. The stress scene

### The stress scene
- **Definition:** the one worst-case scene on which every stress pass mark is set, in every topic (proposed by
  resolution 9; the game-designer's caps on vehicles and projectiles replace its counts when they exist, R2-BUD-OQ5):
  - **soldiers:** the largest mode's N, bots with full AI, in three teams (16v16v16 under MODES-3 (a), 18v18v18 under
    (b)), all in one combat cluster; a mark that asks for scale steps also runs N = 12, 24, 36, 48 and the N = 64
    headroom run (R1-TICK-REC2 P1);
  - **big vehicles:** 15 in play, harvesters included, of which 4 are very large units;
  - **live projectiles:** 300;
  - **active effects:** up to 8 per soldier, so N × 8 (384 at N = 48, 432 at N = 54);
  - **where:** the soldiers, the 4 very large units and the live projectiles are in the one fight; the other 11 big
    vehicles move elsewhere on the same map. For a render mark the fight is what is on screen ("the stress scene's
    fight on screen"); the benchmark scene (R3-MEASURE-REC2) is a recording of this scene with a scripted camera;
  - **map:** one 3D greybox shared by every mark that uses the scene (R1-TICK-REC2's "DM_Tri-sized greybox").
  Each count's basis is in resolution 9: the soldier count from MODES-3; 15 big vehicles from R2's worked estimate at
  48 players in three teams, which is R2's stated assumption (one vehicle per squad of 4, LOOP_V2 §2's squad size,
  plus one harvester per team, GE6: 12 + 3; R2 labels the row "assumption", r2:1758, r2:1771), where 4 and 8 come with
  no rule; 4 very
  large units from R1-TICK-REC2, R3-CHAR-REC4 and VISION_V2 §6.9 ("a few very large units"); 300 projectiles from
  R1-ECS-REC1; 8 effects per soldier from R1-ABL-REC6. R2's worked bandwidth estimate is not the stress scene: its
  counts (15 vehicles, 32 projectiles alive) are its own assumptions for bytes per second. For a server mark that
  includes the server-only share, every soldier has a snapshot-receiving client (R2-SYNC-REC9 mark 1), as the budget's
  48 × 50 µs assumes.
- **Depends on:** MODES-3, for N and the three-team arrangement only; every other count is the same under both readings.
- **Marked by:** "the stress scene", "the stress scene's fight on screen".
- **Retire (exact):** "P1's scene" (R1-TICK-REC3), "the worst case of the largest mode" (R3-BUDGET-REC2,
  R3-MEASURE-REC2), "R4-PHYS-REC4's stress scene" (R4-OBST-REC1), "a physics scene of the largest mode's size"
  (R1-DET-REC3), "the worst case (48 players plus vehicles and projectiles …)" (R2-INT-REC4), each with its own counts.
- **Retire (in context):** "stress load" (R1-ECS-REC1, R4-PHYS-REC1, R4-PHYS-REC4), "R1-TICK-REC2's P1 scene"
  (R4-NAV-REC2) and "R1-TICK's scene" (R3-BUDGET-REC2, R3-CHAR-REC4), each with its own counts, where the scene a
  stress mark is set on is meant, not where a Rests-on line or a list of wave-1 changes names that unit's own scene.
- **Used today:** R1: R1-ECS-REC1 marks 1 and 2 (48 soldiers, 8 vehicles, 300 projectiles, 200 effects),
  R1-TICK-REC2 P1 (4 big vehicles, 200 projectiles), R1-TICK-REC3, R1-DET-REC3, R1-ABL-REC6 (8 effects per soldier),
  R1-XU-OQ1 (lists the differences). R2: R2-SYNC-REC5 mark 3 ("48 soldiers plus 12 vehicles and harvesters"),
  R2-INT-REC4, R2-BUD-OQ5; the worked estimate (its own counts). R3: R3-CHAR-REC4, R3-BUDGET-REC2 (4 large units,
  200 projectiles), R3-MEASURE-REC2, and the marks run "in the benchmark scene" (R3-WGPU-REC1, R3-WGPU-REC2,
  R3-GL2-REC1, R3-GL2-REC2, R3-CHAR-REC1, R3-BUDGET-REC3, R3-MEASURE-REC3, R3-MEASURE-REC4, R3-WGPU-OQ2,
  R3-CHAR-OQ4). R4: R4-PHYS-REC1 mark 2, R4-PHYS-REC4 (8 big vehicles), R4-NAV-REC2, R4-OBST-REC1, R4-LOS-REC2 ("worst
  case"), the wave-1 list (r4:1115-1120). R5, R6: none.

### Big vehicle and very large unit
- **Definition:** a big vehicle is a multi-part vehicle in the sim (VISION_V2 §6.4: "big and multi-part"), harvesters
  included (R4-PHYS-REC4). A very large unit is one of VISION_V2 §6.9's "very large units"; in the stress
  scene each is a big vehicle of at least 20 parts (R3-CHAR-REC4's test load). Which units are very large, and whether
  every one is a vehicle, is the roster's (the game-designer's).
- **Marked by:** "big vehicle(s)", "very large unit(s)".
- **Retire (exact):** none.
- **Retire (in context):** "vehicles" without the word big in a stress count (R1-ECS-REC1: "8 vehicles");
  "large units" (R3) where it means the stress scene's very large units; "vehicles and harvesters" (R2-SYNC-REC5
  mark 3) as two counts.
- **Keeps:** big, the word that marks a stress count (big vehicles).
- **Used today:** R1: R1-ECS-REC1 ("8 vehicles"), R1-ECS-REC4, R1-TICK-REC2 ("4 big vehicles"), R1-TICK-OQ9,
  R1-XU-OQ1. R2: the worked estimate ("Vehicles: one per squad of 4 … + one harvester per team"), R2-SYNC-REC4,
  R2-SYNC-REC5 mark 3, R2-SYNC-REC7. R3: R3-WGPU-REC3, R3-LOOK-REC1, R3-CHAR-REC4, R3-BUDGET-REC2, R3-MEASURE-REC2,
  R3-AUDIO-REC1, R3-AUDIO-REC2, R3-AUDIO-REC4 ("large units"). R4: R4-PHYS-REC1, R4-PHYS-REC4, R4-NAV-REC2,
  R4-OBST-REC1, R4-LOS-OQ1, the wave-1 list. R5, R6: none for the count (R6's telemetry names unit kinds).

### Live projectile
- **Definition:** a projectile entity in flight in the sim (bolt, grenade, Molotov, rocket: R4-PHYS-REC3), counted at
  one tick. The stress scene holds 300.
- **Marked by:** "live projectiles".
- **Retire (exact):** none.
- **Retire (in context):** "200 projectiles" as a stress count (R3-MEASURE-REC2); "projectiles alive" for the stress
  count (R2's estimate keeps its own 32 as an assumption for bandwidth).
- **Used today:** R1: R1-ECS-REC1 (300), R1-TICK-REC2 (200), R1-XU-OQ1. R2: the worked estimate (32 alive at 48),
  the encoding options table (r2:1695), R2-BUD-REC3 ("32 live projectiles"), R2-BUD-OQ5. R3: R3-BUDGET-REC2 (200),
  R3-MEASURE-REC2 (200). R4, R5, R6: none for the count.

### Active effect (and visual effect)
- **Definition:** an active effect is a gameplay effect currently applied in the sim (R1-ABL-REC1: instant, timed or
  permanent, optionally periodic). A visual effect is presentation only: sprites, particles and the like that the
  client draws (R3's "effects" in "200 live projectiles and their effects", "effects, sprites, HUD"). The two are never
  counted together; the stress scene counts active effects (8 per soldier).
- **Marked by:** "active effect(s)"; "visual effect(s)" for presentation.
- **Retire (exact):** none.
- **Retire (in context):** a bare "effects" in a count (R3-BUDGET-REC2's heading, R1-ECS-REC1's "200 active effects"
  as the stress count).
- **Used today:** R1: R1-ECS-REC1 (200 active effects), R1-ABL-REC1 (the effect model), R1-ABL-REC6 (up to 8 per
  soldier), R1-XU-OQ1. R3: "effects" as visual effects in R3-BUDGET-REC2 (heading and the GPU row "effects, sprites, HUD
  in the 3D view"), R3-BUDGET-REC3 ("the effect caps (FX pools)"), R3-AUDIO-REC1 ("large units and their effects"). R2,
  R4, R5, R6: none.

## 4. Prediction and random streams

### The authority
- **Definition:** the sim instance whose results count: the match server online; the client's own Worker in an
  offline match, where the same build runs the whole match with bots (R1-BND-REC6). The client's sim online is a
  predicting copy, never the authority (R2-SYNC-REC3: "without giving the client authority over anything").
- **Marked by:** "the authority", "authoritative".
- **Retire (exact):** none.
- **Retire (in context):** "server-only" for state an offline match also holds.
- **Keeps:** write authority-only.
- **Used today:** R1: R1-DET-REC1 ("the server is the authority"), R1-DET-F32, R1-BND-REC6. R2: R2-SYNC-REC1,
  R2-SYNC-REC3, R2-SYNC-REC6 ("keep the server the authority"), R2-SYNC-OQ11. R5: R5-GATE-REC3. R3, R4, R6: none.

### The prediction scope
- **Definition:** what the client's predicting sim runs ahead of the server for the local player: its own movement,
  own weapon state, own vehicle and cosmetic shot feedback (R2-SYNC-REC3, option 3.2 C), each replayed from the
  authoritative state on every snapshot (Quake III's method, R2-SYNC-REC3). Never predicted: damage, deaths and hits on
  others, and the uplink countdown and steals (R2-SYNC-REC3; R1-ABL-REC4 agrees on damage).
- **Depends on:** ABL-PREDICT (Mustafa, after a Feel Lab test), for the player's own abilities: (yes) the scope adds
  own ability activation, cost, cooldown, granted tags and movement changes such as Banzai speed and Shield slow
  (R1-ABL-REC4, option D2); (no) ability presses wait for the server's confirmation (R2-SYNC-REC3's scope as it
  stands). This glossary picks neither.
- **Marked by:** "the prediction scope", "predict", "predicted".
- **Retire (exact):** none.
- **Retire (in context):** "predict only the local player's own abilities and weapons" as a settled scope
  (R1-ABL-REC4's heading); "partial prediction" (R1-ECS-REC1 mark 3) for this scope.
- **Used today:** R1: R1-ECS-REC1 mark 3, R1-DET-REC1, R1-BND-REC6, R1-ABL-REC4, R1-ECS-OQ3, R1-DET-OQ6, R1-DET-OQ9,
  R1-ABL-OQ4, R1-XU-OQ2, the pointers to R2. R2: R2-SYNC-REC3, R2-SYNC-REC6, R2-SYNC-REC7, R2-SYNC-REC8,
  R2-SYNC-REC9, R2-SYNC-OQ5, R2-SYNC-OQ7, R2-SYNC-OQ10, R2-SYNC-OQ11, the hinge table (r2:3341). R4: R4-NAV-REC4
  ("Prediction replays only the local player or its vehicle"). R3, R5, R6: none.

### Shared stream and authority-only stream
- **Definition:** the sim's random numbers come from one seeded generator with one stream per purpose
  (R1-DET-REC4). A shared stream is held by the authority and by the client's predicting sim alike. An authority-only stream is seeded and held by the
  authority alone, stays out of the client's predicting state and out of any hash compared between a client and the
  server. Weapon spread is an authority-only stream (R2-SYNC-REC3, from the security review's finding H2: a seed the
  client knows lets a cheat cancel the spread through the aim point it sends); the client draws its tracers from a
  cosmetic spread, and the hit event corrects them. Presentation randomness (effects, sound, camera shake) uses a
  separate client generator and is no sim stream at all (R1-DET-REC4).
- **Depends on:** SEC-SPREAD (Mustafa, R2-SYNC-OQ11), for weapon spread only: under the security review's answer it
  stays authority-only; if Mustafa picks matching tracers, it becomes a shared stream. Every other part of the
  definition is the same under both.
- **Marked by:** "shared stream", "authority-only stream".
- **Retire (exact):** "server-only random stream" (R2-SYNC-REC3), since an offline match's Worker is the authority too.
- **Retire (in context):** "the simulation's seeded RNG" for spread without naming the stream kind (R1-ABL-REC3).
- **Keeps:** server-only keyed random stream, in the bracketed fix-pass record in R2-SYNC-OQ11.
- **Used today:** R1: R1-DET-REC4 (streams per purpose, "part of the sim state and its hash"), R1-ABL-REC3 ("Random
  rolls (spread) come from the simulation's seeded RNG"), R1-DET-OQ11, R1-DET-OQ9. R2: R2-SYNC-F20, R2-SYNC-REC3
  ("a server-only random stream"), R2-SYNC-OQ11, the pointers to R1 (r2:3279-3280). R3–R6: none.

### State hash
- **Definition:** a fixed, unseeded hash of a canonical serialization of a sim state after a tick (R1-DET-REC6). "The
  state hash" alone means the authority's whole state, all streams included, as the determinism suite and the
  same-build tests use it (R1-DET-REC1, REC6). A hash compared between a client and the server (R1-DET-OQ9) covers only
  the state the client holds, so it leaves authority-only streams out.
- **Marked by:** "the state hash", "per-tick state hash", "the client-held state's hash".
- **Retire (exact):** none.
- **Retire (in context):** "its hash" for the RNG where it is unclear which state is meant (R1-DET-REC4).
- **Used today:** R1: R1-ECS-REC2, R1-TICK-REC1, R1-DET-REC1, R1-DET-REC4, R1-DET-REC6, R1-DET-OQ9, R1-RR-OQ6, the
  pointers between R1's units. R2: R2-INT-REC3, R2-TRN-REC3, R2-ROOM-REC6, the pointers to R1. R4: R4-PHYS-REC1,
  R4-PHYS-REC3, R4-NAV-REC3, R4-OBST-REC1. R5: R5-TEST-REC1, R5-TEST-REC2, R5-TEST-REC4. R3, R6: none.

## 5. Previews and hosting

### Preview
- **Definition:** the per-PR preview link: the build CI made for a pull request, deployed by a separate job that holds
  the only host token (R5-PREV-REC1), with one stable link per PR branch, public but unlisted (`noindex`) by default
  (R5-PREV-REC5). It runs the match offline in the worker with bots and needs no game server (R6-HOST-REC3,
  R1-BND-REC6), unless it is a multiplayer preview that talks to the staging game server (and then PREVIEW-WALL
  applies). It carries the preview header set. Until PREVIEW-WALL is answered there are no multiplayer previews.
- **Marked by:** "preview", "preview link", "multiplayer preview".
- **Retire (exact):** none.
- **Retire (in context):** "deploy preview" (Netlify's product name, R5-PREV-F15) for our previews;
  "preview deployment" where the link Mustafa opens is meant.
- **Used today:** R1: R1-BND-REC5 (G3, previews for profiling), the pointers to R5 and R6. R3: R3-MEASURE-REC2 (the
  benchmark on preview links), R3-MEASURE-OQ4. R5: the R5-PREV part (R5-PREV-F1–F20, REC1–REC5, OQ1–OQ6),
  R5-LAYOUT-REC5, R5-CI-REC5, R5-XQ-OQ3, the pointers to R6. R6: R6-HOST-F3, F4, F6, F7, R6-HOST-REC1, R6-HOST-REC3,
  R6-HOST-OQ5, R6-SEC-OQ1, the cost tables. R2: R2-TRN-REC5 ("Development and previews: self-signed …"). R4: none.

### The client host
- **Definition:** where the player build and the previews are served from: Cloudflare Pages, with Cloudflare Workers
  static assets as the alternative that takes the same `_headers` file (proposed by resolution 19; R6-HOST-REC1 named
  Workers first). The pick rests on R5-PREV-REC2's needs list, which Pages meets on the findings read, all but the
  deletion of a branch's latest deployment (R5-PREV-F13; PREV-DELETE).
- **Decided by the advisor, 2026-10-30** (resolution 19); PREV-DELETE (a spike) may send it back to Workers.
- **Marked by:** "the client host", "Cloudflare Pages", "Workers static assets (the alternative)".
- **Retire (exact):** "Cloudflare (Workers static assets; Pages as the same-headers alternative)" (R6-HOST-REC1's old
  order).
- **Retire (in context):** "Cloudflare" alone where one of the two products is meant.
- **Used today:** R5: R5-PREV-REC2, R5-PREV-REC3, R5-PREV-OQ2, R5-PREV-OQ4, the Pages findings (R5-PREV-F10–F13, F18).
  R6: the TL;DR, the options table Q1 (row A), R6-HOST-REC1, R6-HOST-F1–F6, R6-COST's client line, the pointers to R1.
  R1, R2, R3, R4: none.

### The staging game server
- **Definition:** the one small shared VM (proposed CX33) whose supervisor starts the room binary of the build a
  preview names, with self-signed, pinned, short-lived certificates (R6-HOST-REC3, R2-TRN-REC5). It is the only game
  server a preview may talk to; production servers refuse preview origins (R6-HOST-REC3's mark).
- **Marked by:** "the staging game server", "the staging VM".
- **Retire (exact):** none.
- **Retire (in context):** "a real server" where the staging server is meant (R5-PREV-REC5); "an EU server" for
  previews (R5-PREV-OQ3).
- **Used today:** R5: R5-PREV-REC5 ("a real server"), R5-PREV-OQ3. R6: R6-HOST-REC1, R6-HOST-REC3, R6-OBS-REC2,
  R6-TEL-REC3, the cost tables ("Staging VM for previews"), the pointer to R5 (r6:946). R1–R4: none (R2-TRN-REC5's
  "Development and previews" certificates are its certificates).

### The access wall
- **Definition:** a login in front of a preview: the host's access policy, which limits viewing to Mustafa's
  Cloudflare account (R5-PREV-F12, R6-HOST-F4), or Cloudflare Access (R5-PREV-F14). Whether multiplayer previews sit
  behind it is PREVIEW-WALL.
- **Depends on:** PREVIEW-WALL (Mustafa): (a) on from the first preview that reaches the staging game server; (b)
  off, since previews hold no secrets; (c) decided after the security baseline. This glossary picks none. Under every
  option, previews that reach no game server and carry no unreleased art are public but unlisted; one that carries
  unreleased art is behind the wall (R5-PREV-REC5, R5-PREV-OQ1).
- **Marked by:** "the access wall".
- **Retire (exact):** none.
- **Retire (in context):** "the access policy", "Cloudflare Access" and "private previews" as names for the decision
  (each stays as the name of a product or setting); "login wall" as a name for the access wall.
- **Used today:** R5: R5-PREV-F12, R5-PREV-F14, the options for previews (r5:1123), R5-PREV-REC3, R5-PREV-REC5,
  R5-PREV-OQ1. R6: R6-HOST-F4, the options table Q1, R6-HOST-REC1, R6-HOST-OQ5, R6-SEC-OQ1. R1–R4: none.

### Preview isolation and the isolation flag
- **Definition:** the isolation flag is one build switch that adds the cross-origin isolation headers, COOP
  `same-origin` and COEP `require-corp` (R1-BND-REC5), to a build's header set. It is off by default everywhere:
  local development, previews and production. It is on only for profiling builds, in local development or on a
  preview built with it on (R1-BND-REC5's option G3: "allow it in development and previews for profiling"). Preview
  isolation is a preview built with the flag on. Production turns it on only if R1-BND-REC2 mark 5's B2-against-B3
  test leads the ADR to adopt isolation; then the default flips everywhere at once.
- **Marked by:** "the isolation flag", "an isolated preview", "a preview built with the isolation flag on".
- **Retire (exact):** none.
- **Retire (in context):** "COOP and COEP on preview hostnames only" for every preview (R5-PREV-REC4);
  "when R1-BND-REC5's option G3 is on" (R6-HOST-REC1) as the name of the switch; "always" isolated previews.
- **Used today:** R1: R1-BND-REC5 (G1 and G3), R1-BND-OQ9, the boundary options (r1:1768), the pointers to R5 and R6.
  R3: R3-MEASURE-F7, R3-MEASURE-OQ4. R5: R5-LAYOUT-REC5 ("only when an isolation flag is set"), R5-PREV-REC4 (every
  preview), R5-GATE-REC1, R5-GATE-REC3, R5-XQ-OQ3, the CI options (r5:1104). R6: the options tables Q1 and Q2,
  R6-HOST-REC1 ("when R1-BND-REC5's option G3 is on"), R6-HOST-F1, R6-HOST-F12. R2, R4: none.

### The production header set and the preview header set
- **Definition:** the production header set is the set of response headers the player build ships with: `.wasm` as
  `application/wasm` (R5-PREV-F20), R6's content security policy once the security baseline writes it, and no COOP or
  COEP until isolation is adopted (R1-BND-REC5). The preview header set is the production header set plus
  `X-Robots-Tag: noindex` (R5-PREV-F12, F15). The preview Mustafa checks in Brave carries the preview header set with
  the isolation flag off, so it runs the headers production ships (R5-XQ-OQ3's option). Local development uses the
  same two sets in Vite's `server.headers` and `preview.headers`, behind the same flag (R5-LAYOUT-REC5, R5-PREV-F19).
- **Marked by:** "the production header set", "the preview header set".
- **Retire (exact):** none.
- **Retire (in context):** "a preview-only header set" that differs from production in anything but `noindex`
  (R5-PREV-REC4).
- **Used today:** R5: R5-PREV-REC4, R5-XQ-OQ3, the pointers to R6's baseline (r5:1954). R6: R6-HOST-F7 (Netlify's one
  set for every context), R6-HOST-REC1. R1: R1-BND-REC5 (what production adds if isolation is adopted). R2, R3, R4:
  none.

## 6. The deferred security baseline

### The security baseline
- **Definition:** R6's fifth question, "content security policy, dependency audits, anti-cheat basics", deferred by
  the Advisor on 2026-10-27; "ADR 0006 lists it as open, or it is researched before 0006 is accepted" (R6-SEC-OQ1).
  R6-SEC-OQ1 lists what waits on it. When it is researched is BASELINE-WHEN.
- **Depends on:** BASELINE-WHEN (the Advisor): (a) researched now, as one security-auditor topic before the drafts;
  (b) listed open in ADR 0006, with interim positions (R5-GATE-REC1's scheduled advisories check as the mechanism;
  R2-TRN-REC5's certificate per hostname, with R6's wildcard as an option the baseline weighs); (c) left as one shared
  open question. This glossary picks none.
- **Marked by:** "the security baseline".
- **Retire (exact):** none.
- **Retire (in context):** "R6's deferred baseline", "the deferred baseline", "R6's security baseline" as different
  names (R2-ROOM-OQ2, R2-ROOM-OQ11, R2-TRN-REC1, R5-TSLINT-OQ5, R5-GATE-OQ4).
- **Keeps:** one name, the security baseline.
- **Used today:** R1: the pointers (r1:2961). R2: R2-TRN-REC1 mark 5, R2-SYNC-REC5 ("R6's anti-cheat basics"),
  R2-ROOM-OQ2, R2-ROOM-OQ11, the pointers to R6 (r2:3310). R5: R5-TSLINT-OQ5, R5-GATE-OQ4, the pointers to R6's
  baseline (r5:1952). R6: the header, the TL;DR, section 5 (R6-SEC), the certificate options (r6:422), R6-HOST-REC4,
  R6-HOST-OQ5, R6-SEC-OQ1. R3, R4: none.

### The advisories check
- **Definition:** the CI check that compares the locked Rust dependencies with published security advisories
  (cargo-deny's advisories check). R5-GATE-REC1 runs it on a schedule, outside the PR gate, and on every PR that
  changes `Cargo.lock`, because "a new advisory must not turn an unrelated PR red" (R5-GATE-F3). Whether an advisory
  may block a PR, and who owns that policy, is the security baseline's (R5-GATE-OQ4, R6-SEC-OQ1; BASELINE-WHEN). The
  dependency rules in the PR gate (G5: cargo-deny bans, duplicate and wildcard versions) are a different check.
- **Marked by:** "the advisories check".
- **Retire (exact):** "audit gate" (R2-TRN-REC1 mark 5).
- **Retire (in context):** "cargo-audit and cargo-deny as CI gates" as the name of the check (R1-RR-OQ7: the
  reviewer's proposal stays quoted as such), "dependency audits" for the check itself (the phrase stays for the
  baseline's subject).
- **Used today:** R1: R1-RR-OQ7. R2: R2-TRN-F40, R2-TRN-REC1 mark 5, R2-TRN-OQ5, the transport options (r2:1840).
  R5: R5-CI-F15, R5-GATE-F3, R5-GATE-F4, R5-LAYOUT-REC2, R5-GATE-REC1 (G5 and the scheduled list), R5-LAYOUT-OQ3,
  R5-GATE-OQ4, the pointers to R6's baseline. R6: the header, R6-SEC-OQ1. R3, R4: none.

### Game-server certificate
- **Definition:** the TLS certificate a production game server presents to browsers. R2-TRN-REC5 asks for "a publicly
  trusted certificate per game server hostname"; R6-HOST-REC4 proposes one wildcard certificate (option C2), which puts
  one private key on every game VM. Development and preview servers use self-signed, pinned certificates valid at most
  14 days (R2-TRN-REC5; the staging game server). Which production form holds waits on the security baseline.
- **Depends on:** BASELINE-WHEN (the Advisor), as above: under (b) the certificate per hostname holds and the wildcard
  is an option the baseline weighs; under (a) the research decides; under (c) both stay as written with one shared
  question. This glossary picks none.
- **Marked by:** "a certificate per hostname", "the wildcard certificate".
- **Retire (exact):** none.
- **Retire (in context):** "a publicly trusted certificate" for production with no form named (per hostname or
  wildcard).
- **Used today:** R2: R2-TRN-REC5, R2-TRN-OQ4, the pointers to R6. R6: the TL;DR, R6-HOST-F24, the certificate
  options (C1–C3), R6-HOST-REC4, R6-HOST-OQ6, R6-SEC-OQ1, the pointers to R2. R1, R3, R4, R5: none.

## 7. Determinism checks in the sim crate

### The hash-map ban in the sim crate
- **Definition:** the sim crate uses no std `HashMap` or `HashSet`: clippy's `disallowed-types` denies the types in
  `crates/sim/clippy.toml`, with `iter_over_hash_type` as a backstop (R5-RUSTLINT-REC2); the replacements are
  `BTreeMap` or an index-ordered `Vec` (R1-ECS-REC2, R1-DET-REC5). The only escape is a reviewed
  `#[allow(clippy::disallowed_types)]` with a written reason, for a map that is only looked up and lies outside the
  serialized sim state (R5-RUSTLINT-OQ5; resolution 29). Maps from other crates are R5-RUSTLINT-OQ3.
- **Marked by:** "the hash-map ban".
- **Retire (exact):** none.
- **Retire (in context):** "hash maps serve lookups only" as a rule by itself (R1-DET-REC5);
  "zero uses of std HashMap or HashSet iteration in … outcome code" as the check (R1-ECS-REC2 mark 2), since no lint
  can find that.
- **Used today:** R1: the TL;DR, R1-ECS-F23, R1-DET-F25, R1-DET-F33, R1-ABL-F26, R1-ECS-REC2, R1-DET-REC5,
  R1-DET-REC6, R1-ABL-REC3, R1-XU-OQ5, the pointers to R5. R4: R4-NAV-F15 (a crate's hasher). R5: the TL;DR,
  R5-RUSTLINT-F8, F9, R5-RUSTLINT-REC2, R5-TEST-REC2, R5-CI-REC5, R5-GATE-REC3, R5-RUSTLINT-OQ3, R5-RUSTLINT-OQ5, the
  pointers from R5-CI. R2, R3, R6: none.
