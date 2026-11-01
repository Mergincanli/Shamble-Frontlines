# ADR 0002: fix round 2 (plan step 9), engine-architect, 2026-11-01

Fixes against `docs/adr/0002-code-structure-build-toolchain.md` (the file at the time of writing; every BEFORE occurs
exactly once there and no two overlap, checked by script). Sources: verifier-adr0002-verdicts.md as
verifier-skeptic-2-results.md judged it (row 36 = F-6, F-2, F-3, F-5, F-8, F-10, F-11, F-12), and
engine-architect-consistency-findings.md (F11, F12, F13 for 0002:253). Pass-mark rows are left to test-engineer.

### FIX row-36 (F-6)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
- The headless "SIM fence" leaked through comments (row CS11; r1-simulation-core.md, legacy facts, lines 1176–1178),
  the frame loop stepped the world inside the render loop (rows GP13 and GP14; r1 lines 1159–1164), and the input command
  was declared but never wired (row CS1; r1 lines 1150–1154): the crate boundary, the Worker and the per-tick command fix
  these.
AFTER:
- The headless "SIM fence", a fence made of comments, leaked: shape data came from the visual recipe and `hasLOS` read a
  global (row CS11; r1-simulation-core.md, legacy facts, lines 1176–1178); the frame loop stepped the world inside the
  render loop (rows GP13 and GP14; r1 lines 1159–1164); and the input command was declared but never wired (row CS1; r1
  lines 1150–1154): the crate boundary, the Worker and the per-tick command fix these.
END

### FIX F-12
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
   - Need: GLOSSARY §8, the foundation only ("the toolchain"); through R1-DET-REC6's native-against-Wasm comparison,
     VISION_V2 §6.6 and §6.8 (re-runnable matches).
AFTER:
   - Need: GLOSSARY §8, the foundation only ("the toolchain"); through R1-DET-REC6's native-against-Wasm comparison,
     VISION_V2 §6.6 and §3, as R5-CI-REC2 traces it (r5-toolchain-quality.md, line 1532; R1-DET-REC6 serves §6.6 and
     §3, r1-simulation-core.md lines 2236–2237).
END

### FIX F-3 (part 3)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
     lines 888–899). Both placements answer questions the critic gave this record; no research compared them with
     other placements.
AFTER:
     lines 888–899). Both placements answer questions the critic gave this record; no research compared them with
     other placements. Both are proposed (not researched): no research Options section offers a crate for the room
     logic or the bake, and SLICE-ORDER condition 1 and R4-XT-OQ5 leave them to this record; whether plan rule 3
     allows that is OQ-0002-8, for the Advisor at the stop.
END

### FIX F-11
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
   - The cache question (critic rank 40) is answered by this list: no ECS crate and parry3d rather than Rapier (ADR
     0004, part 1), quinn and the WebTransport crate the netcode proof test picks (ADR 0001, part 10); the cache is
     measured on this workspace against R5-CI-REC1's 8 GB mark in the foundation (part 9).
AFTER:
   - The cache question (critic rank 40) is answered by this list: no ECS crate (ADR 0004, part 1) and parry3d rather
     than Rapier (ADR 0004, part 10), quinn and the WebTransport crate the netcode proof test picks (ADR 0001, part 10);
     the cache is measured on this workspace against R5-CI-REC1's 8 GB mark in the foundation (part 9).
END

### FIX F-10
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
   `sim-wasm` and `server` and fails if one appears (the rule is ADR 0004's, part 9; the check is this record's).
AFTER:
   `sim-wasm` and `server` and fails if one appears (the rule is ADR 0004's, part 7; the check is this record's).
END

### FIX F11 (consistency; part 4)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
   reason. The Wasm size budget has no research number: the boundary spike and the sim core spike measure the
   `sim-wasm` file (raw and gzip, after wasm-opt and without it, `opt-level` "s" against "z", with parry3d), and the
   budget file (part 9) takes the measured value as its first limit; every served file stays under the client host's
   25 MiB per-file limit.
AFTER:
   reason. The Wasm size budget has no research number, and this record owns it (R1-RR-OQ5): the boundary spike and
   the sim core spike measure the `sim-wasm` file (raw and gzip, after wasm-opt and without it, `opt-level` "s" against
   "z", with parry3d), the content and ability spike measures the share the `content` crate adds (ADR 0005,
   OQ-0005-CD2), and the Advisor sets the first limit in the budget file (part 9) from both measurements; until then
   G7 records the `sim-wasm` size and does not gate it. Every served file stays under the client host's 25 MiB
   per-file limit.
END

### FIX F13 (consistency; event names)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
     (own weapon state, money, upgrade tracks and offers: decision map I-21) and the event section for the tick (hits,
     deaths, blasts, cosmetic shot events, purchases, captures, pick results, drops, moments: ADR 0004 part 19 emits
     them, ADR 0001 sets their visibility). Online, remote entities are written at the interpolation delay from the
     Worker's interpolation buffer and the local player as predicted (ADR 0001 parts 4, 6, 7); the fields of remote
AFTER:
     (own weapon state, money, upgrade tracks and offers: decision map I-21) and the event section for the tick (the
     events of ADR 0004 part 19: hits, deaths and kills, blasts, cosmetic shot events, purchases, uplink cache events,
     harvester deliveries and kills, structure kills and stage changes, tech tiers, upgrade picks and pickups, and the
     moments its moment phase emits; ADR 0001 sets their visibility). Online, remote entities are written at the
     interpolation delay from the Worker's interpolation buffer and the local player as predicted (ADR 0001 parts 4,
     6, 7); the fields of remote
END

### FIX F-8 (part 5)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
     and copy time at N = 48); ADR 0003 reads typed arrays either way. The block writer serializes field by field, as
     [the state hash](../research/GLOSSARY.md#state-hash) already does (R1-DET-REC6), so struct padding never reaches the page and no `unsafe` is needed; if
     the spike's copy time says otherwise, R1-RR-OQ6's two alternatives (a derive that rejects padding, or `unsafe` with
     a written invariant and a test) are weighed then. The SharedArrayBuffer ring (B3) is the spike's measured
     alternative, never the sim's own shared Wasm memory (B4). At the 30 Hz baseline that is 30 blocks a second
AFTER:
     and copy time at N = 48); ADR 0003 reads typed arrays either way. The block writer serializes field by field
     (proposed: R1-DET-REC6's method for [the state hash](../research/GLOSSARY.md#state-hash), applied to the block;
     R1-RR-OQ6's decider, rust-reviewer, confirms at review), so struct padding never reaches the page and no `unsafe`
     is needed; if the spike's copy time says otherwise, R1-RR-OQ6's two alternatives (a derive that rejects padding,
     or `unsafe` with a written invariant and a test) are weighed then. The SharedArrayBuffer ring (B3) is the spike's
     measured alternative, never the sim's own shared Wasm memory (B4). At the 30 Hz baseline that is 30 blocks a second
END

### FIX F-2 (part 5, the control API)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
   - **The small control API** (create, mode flag, load the content bundle, step, the block's pointer and length) is
     wasm-bindgen's own, with its `.d.ts` (option D1); it is called a constant number of times per tick, never per
     entity.
AFTER:
   - **The small control API** (create, mode flag, load the content bundle, one input call per tick, and the step) is
     wasm-bindgen's own, with its `.d.ts` (option D1). Each tick the Worker's host makes one input call, which carries
     the page's command and every datagram received since the last tick, and one step call; the step leaves the
     outgoing datagrams and the block in linear memory and returns where they lie. The Worker's Wasm calls per tick,
     wasm-bindgen's glue calls included, are meant to stay within R1-BND-REC1 mark 2's four, which the spike counts
     (item 1); none is made per entity.
END

### FIX F-5
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
     The gate regenerates all of them and fails on any difference or hand edit (G4).
AFTER:
     The gate regenerates all of them and fails on any difference or hand edit (G4).
   - **Content files in development:** a small Vite plugin, in the `client` package's Vite config and never in a
     production build, pushes changed content files to the page, which posts them to the Worker (R1-DATA-REC5,
     r1-simulation-core.md lines 2545–2547; ADR 0005, CD-7); whether Vite's hot update reaches the Worker directly is
     PM-0002-31's to measure (R1-DATA-OQ8). What a running match does with a changed file is ADR 0005's
     (OQ-0005-CD4).
END

### FIX F-2 (part 5, the connection)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
   - **The Worker owns the network connection** online (option H2): server snapshots, prediction and reconciliation
     never touch the page, which sees only the block. The Worker's host hands the tick's received datagrams to Wasm in
     one call and takes the tick's outgoing ones in one call, so its Wasm calls per tick stay the same at every N
     (R1-BND-REC1 mark 2), even when a snapshot is split into several datagrams. What flows over the connection is ADR
     0001's.
AFTER:
   - **The Worker owns the network connection** online (option H2): server snapshots, prediction and reconciliation
     never touch the page, which sees only the block. The received datagrams ride in the tick's one input call and the
     outgoing ones come back through the step (the control API above), so the Wasm calls per tick stay the same at
     every N (R1-BND-REC1 mark 2), even when a snapshot is split into several datagrams. What flows over the connection
     is ADR 0001's.
END

### FIX F12 (consistency; seam S11)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
     question, decision map S11).
AFTER:
     question for Mustafa, listed in `docs/adr/README.md`'s design questions).
END

### FIX F-3 (Q4)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
| **R5-LAYOUT-REC2 plus an I/O-free `room` crate and a `navbake` crate above `sim` (chosen)** | low–medium: two more crates and two more dependency checks | – | the player's Wasm carries neither the bake nor the server's I/O | the same decoders and room checks run locally and online | none | new | one room logic for the local server and the room process; the bake shared by CI, map save and the editor | two placements no research compared with others (the critic's questions only frame them) |
AFTER:
| **R5-LAYOUT-REC2 plus an I/O-free `room` crate and a `navbake` crate above `sim` (chosen)** | low–medium: two more crates and two more dependency checks | – | the player's Wasm carries neither the bake nor the server's I/O | the same decoders and room checks run locally and online | none | new | one room logic for the local server and the room process; the bake shared by CI, map save and the editor | not researched as a placement (no Options section offers a crate for the room logic or the bake; SLICE-ORDER condition 1 and R4-XT-OQ5 leave it to this record): proposed only for the Advisor's ruling at the stop (OQ-0002-8) |
END

### FIX F-3 (trade-off)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
- **Two crates beyond R5's list** (`room`, `navbake`) cost two more dependency checks; without them the local server
  cannot run in the Worker, and the bake's generator would risk shipping in the player's Wasm. No research compared
  other placements, so the first foundation milestone that builds them is also their test.
AFTER:
- **Two crates beyond R5's list** (`room`, `navbake`) cost two more dependency checks; without them the local server
  cannot run in the Worker, and the bake's generator would risk shipping in the player's Wasm. No research compared
  other placements, so the first foundation milestone that builds them is also their test; whether rule 3 allows
  proposing them at all is OQ-0002-8.
END

### FIX F-3 (new open question)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
  - Research ids: R1-BND-OQ7.
AFTER:
  - Research ids: R1-BND-OQ7.
- **OQ-0002-8. May this record propose a structure no research Options section offers, when a settled input forces it
  and the research leaves the question to this record?**
  - Why it matters: plan rule 3 says an option the research lacks is labelled "not researched" and cannot be the
    proposed decision. SLICE-ORDER condition 1 runs the local server in the Worker, which needs room logic that builds
    for Wasm, while R5-LAYOUT-REC2 puts rooms inside `server` (r5-toolchain-quality.md, lines 1173–1174); and the critic's
    questions hand structure to this record with no answer on one side ("Decided by: ADR-0002": R4-XT-OQ5,
    r4-physics-navigation.md lines 1195–1200, the bake's place; R5-XT-OQ6, r5 lines 1996–2001, the multiplayer-preview
    workflow, whose parts part 10 takes from research recommendations).
  - Options: (a) rule 3 allows it in such cases: the `room` and `navbake` crates stay proposed, labelled "not
    researched", and the first foundation milestone that builds them is their test; (b) rule 3 holds: both become open
    questions, a short layout research unit answers them before the foundation, and part 3 keeps R5-LAYOUT-REC2's list
    until then.
  - Decider: the Advisor, at the stop.
  - What decides it: whether rule 3 is meant to cover structure that a settled input forces and the research hands to
    this record.
  - Research ids: none of its own; it concerns R5-LAYOUT-REC2 (changed), R4-XT-OQ5 and R5-XT-OQ7.
END

### FIX F-2 (spike item 1)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
   Wasm calls per tick at N = 12 and N = 48, at most 4 for the online client and for a local match in one instance
AFTER:
   Wasm calls per tick at N = 12 and N = 48, wasm-bindgen's glue calls included (part 5 batches the command and the
   received datagrams into one input call), at most 4 for the online client and for a local match in one instance
END

### FIX F12 (consistency; spike item 7)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
7. **The Worker's pacing and the client's sim share** (ADR 0004's marks, measured here as the decision map's part g
   places them): ADR 0004's S18 (tick start jitter, smoothness on 60, 120 and 144 Hz displays, hidden tabs) and
AFTER:
7. **The Worker's pacing and the client's sim share** (ADR 0004's marks, measured here as `docs/SPIKE_PLAN.md`
   places them, item 3): ADR 0004's S18 (tick start jitter, smoothness on 60, 120 and 144 Hz displays, hidden tabs) and
END

### FIX F11 (consistency; spike item 8)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
8. **Size and tools** (parts 2, 4 and 7): the `sim-wasm` file's size recorded raw and gzipped, with and without
   wasm-opt, at `opt-level` "s" and "z" (R1-RR-OQ5, R5-CI-OQ4); no pass mark (the research has no number): the
   measurement becomes the budget file's first limit, under the client host's 25 MiB per-file limit; the count of
   type errors and casts the page needs against three.js's types under the base tsconfig (R5-TSLINT-OQ4); `tsc -b` time
AFTER:
8. **Size and tools** (parts 2, 4 and 7): the `sim-wasm` file's size recorded raw and gzipped, with and without
   wasm-opt, at `opt-level` "s" and "z" (R1-RR-OQ5, R5-CI-OQ4); no pass mark (the research has no number): the
   measurement is one input to the budget's first limit, which the Advisor sets after the content and ability spike
   has measured the content crate's share (part 4); every served file stays under the client host's 25 MiB per-file
   limit; the count of type errors and casts the page needs against three.js's types under the base tsconfig
   (R5-TSLINT-OQ4); `tsc -b` time
END

### FIX F12 (consistency; the foundation)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
**The foundation** is not a spike: its acceptance checks are the milestones of the Migration plan's phase 2 (decision
map part g, "Not throwaway spikes").
AFTER:
**The foundation** is not a spike: its acceptance checks are the milestones of the Migration plan's phase 2
(`docs/SPIKE_PLAN.md`, "The foundation's acceptance checks (not a spike)").
END

### FIX F-2 (note under the pass-mark table)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
- **PM-0002-3, "at most 4".** The draft's own per-tick list for the online client (the command in; the tick's
  received datagrams in one call; the step; the outgoing datagrams in one call; the block's pointer and length) is 5
  or 6 calls unless inputs are batched into one call, and wasm-bindgen's glue makes its own allocator call whenever
  bytes are passed in. The row gates control-API calls at 4 and reports the glue's calls; the drafter confirms the
  reading, or the design batches its inputs.
AFTER:
- **PM-0002-3, "at most 4".** The draft's first per-tick list for the online client (the command in; the tick's
  received datagrams in one call; the step; the outgoing datagrams in one call; the block's pointer and length) came
  to 5 or 6 calls, and wasm-bindgen's glue makes its own allocator call whenever bytes are passed in. Part 5 now
  batches the command and the received datagrams into one input call, and the step returns where the outgoing
  datagrams and the block lie.
END

### FIX F11 (consistency; Appendix A)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
| R1-RR-OQ5 | spike: the boundary spike and the sim core spike measure the size; the budget file takes the measurement | part 4; the spike's item 8 |
AFTER:
| R1-RR-OQ5 | spike: the boundary spike and the sim core spike measure the `sim-wasm` size, and the content and ability spike the content crate's share; the Advisor sets the first limit from both | part 4; the spike's item 8 |
END

### FIX F-8 (Appendix A)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
| R1-RR-OQ6 | answered: the block writer serializes field by field, as the state hash does; the derive or `unsafe` only if the copy time says so | part 5; the spike's item 2 reports the copy time |
AFTER:
| R1-RR-OQ6 | answered (proposed; rust-reviewer, its named decider, confirms at review): the block writer serializes field by field, as the state hash does; the derive or `unsafe` only if the copy time says so | part 5; the spike's item 2 reports the copy time (PM-0002-10) |
END

### FIX F-3 (Appendix A, R4-XT-OQ5)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
| R4-XT-OQ5 | answered: the bake in its own `navbake` crate, never linked into `sim-wasm`; the map checks as G13 | parts 3 and 9 |
AFTER:
| R4-XT-OQ5 | answered: the bake in its own `navbake` crate (proposed, not researched: OQ-0002-8), never linked into `sim-wasm`; the map checks as G13 | parts 3 and 9 |
END

### FIX F-3 (Appendix A, R5-LAYOUT-REC2)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
| R5-LAYOUT-REC2 | changed: the `room` and `navbake` crates added; nothing imports `spikes/` | part 3: SLICE-ORDER condition 1 runs the local server in the Worker; R4-XT-OQ5 |
AFTER:
| R5-LAYOUT-REC2 | changed: the `room` and `navbake` crates added (proposed, not researched: OQ-0002-8); nothing imports `spikes/` | part 3: SLICE-ORDER condition 1 runs the local server in the Worker; R4-XT-OQ5 |
END

### FIX F11 (consistency; G7)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
   | G7 | budgets | Wasm bytes (raw and gzip) and JS bytes per entry against the committed budget file; fail above it |
AFTER:
   | G7 | budgets | Wasm bytes (raw and gzip) and JS bytes per entry against the committed budget file; fail above it (the `sim-wasm` size is recorded, not gated, until its first limit is set: part 4) |
END
