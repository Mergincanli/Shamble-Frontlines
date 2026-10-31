# Decision map for the v2 decision drafts (step 2)

- **What this is:** the working map the seven draft authors read before they write, so that ADR 0001–0006 neither
  overlap nor leave gaps. Notes only: it assigns, it does not decide. Where the research recommends something, that
  recommendation is the owner draft's starting point; the owner adopts, changes or rejects it (plan rule 8).
- **Author:** engine-architect (lead), 2026-10-31, xhigh. **Inputs:** `inputs/DRAFTS_BRIEF.md`, `plan.md` revision 1
  (under the advisor's review: if its verdict changes an input, this map is redone), `docs/VISION_V2.md`,
  `docs/LOOP_V2.md`, `docs/EDITOR_V2.md`, `docs/LOOK_V2.md`, `docs/STATUS.md` "Decisions pending",
  `docs/research/` (README, GLOSSARY, r1–r6), `docs/LEGACY_INVENTORY.md`, `docs/adr/0000-template.md`,
  `inputs/research-ids.tsv` (535 rows at 9e7db28). Line numbers below are at 9e7db28 (the topic files do not change in
  this PR).
- **Companion file:** `engine-architect-decision-map-ids.tsv`: part b as TSV, 521 rows (every id of research-ids.tsv
  once; the 18 SHARED rows as 6 codes; the two R1 ids also listed in r3 as one row each).
- **Words:** "owner" = the one draft whose Appendix A disposes the id (adopted, changed, rejected, answered, still open
  as its OQ, or for a spike); "cites" = drafts that must reference the owner's text and never restate it differently.
  DESIGN = a pure game-design matter no engine draft settles (decider: Mustafa, with the game-designer); the citing
  drafts keep the engine able to carry any answer. Terms are the GLOSSARY's (linked there); spike names below are
  working names only: `docs/SPIKE_PLAN.md` defines the names once (plan rule 5).
- **One remap from the research:** the consistency critic's XT items (2026-10-29) name "ADR-0004 (the boundary)". The
  plan and this brief put the Rust ↔ TypeScript boundary, the Worker and the generated types in ADR 0002, so every
  "ADR-0004 (boundary …)" in an XT text reads "ADR 0002" here (critic ranks 2, 4, 16 and 37).

## a. Scope per draft

### ADR 0001: multiplayer and netcode (netcode-architect)
Decides:
1. **Authority and the prediction scope:** the match server is the authority online, the client's Worker offline
   ([GLOSSARY: the authority]); what the client predicts (R2-SYNC-REC3, R1-ABL-REC4); ABL-PREDICT stays open (the
   Feel Lab test decides); SEC-SPREAD (a) applied (spread on an authority-only stream; tracers drawn from a cosmetic
   spread the hit corrects).
2. **The tick rate rule:** R1-TICK-REC2's rule as TICK-30 changes it (brief §1): 30 Hz is the baseline; 60 Hz only if
   the netcode proof test shows a 60 Hz margin of at least 25 % (GLOSSARY §8; the advisor's ruling, TICK-60-HEADROOM,
   confirmed at the stop: p99 server tick time ≤ 6.25 ms against the 8.33 ms server tick budget at 60 Hz; part j, J1;
   carry-forward 2). Also the snapshot rate (R2-BUD-REC1),
   the input rate and redundancy, time sync and command stamping (R1-TICK-OQ3, OQ4, OQ8; critic rank 12), and 60 or
   64 Hz (R1-TICK-OQ1) should 60 Hz ever pass.
3. **The server tick budget:** its total (50 % of the tick period) and the server-only share (25 %) with every part
   inside it (encoding, lag compensation, interest, the enclosure test and its re-flood, commands, sends): R2-BUD-REC6,
   R2-SYNC-REC9, R2-XU-OQ8, restated for the 30 Hz baseline. It takes the sim step share's figure from 0004 and adds it
   to the total; it never restates its parts.
4. **The message protocol:** every message on the network and on the local server path (SLICE-ORDER condition 1: the
   same messages, no offline-only shortcuts): commands (fields, edges, view time), snapshots and deltas, events with
   their visibility classes, join, resync and the first full state (R2-XU-OQ6), the control stream, the lobby protocol,
   the content hash at join and hash comparison as divergence telemetry (critic rank 13), wire id and tick widths
   (rank 27), snapshot fields including remote soldiers' animation state (rank 16), the interpolation design and on
   which side of the boundary its buffer lives (rank 4), the projectile encoding E3 or E3b (rank 7), the aim point in
   the command (rank 2, with 0002 and 0004).
5. **Bandwidth and buffers:** per-client downstream and upstream budgets, packet size and the split path, the
   interpolation delay, the rewind cap and the hit history, congestion handling (R2-BUD-REC1–REC5; R2-XU-OQ1–OQ7,
   OQ10, OQ11, OQ15).
6. **Interest management and anti-wallhack** (R2-INT-REC1–REC7), inside the game rules DESIGN leaves open (what a
   player may see, R2-INT-OQ1).
7. **Transport** (R2-TRN-REC1–REC4, REC6): quinn, the WebTransport crate picked by spike, the WebSocket fallback, the
   thin transport interface and its backends (including the one the local server uses: interface I-3 in part d).
8. **Rooms and the room process** (R2-ROOM-REC1–REC8): lifecycle, one room per process with the sim on its own thread
   and network I/O on an async runtime, join tokens, reconnect, private rooms, bots filling slots on the server through
   the human input path, a room as a mode id plus allowed overrides (the mode file's format is 0005's), guest identity.
9. **The network profiles** every network mark names (critic rank 32; R2-XU-OQ12).
10. **Its spike: the netcode proof test** (brief §2), with the budgets it is measured against: server tick time,
    bandwidth per client, memory per room.

Takes from the others: the fixed-tick mechanism, the sim step share and the state hash (0004); the boundary, the
Worker and the generated types (0002); the reference server, the machines, certificates, costs and the lobby's hosting
(0006); the mode file format, unit data and the content hash (0005); the client reference machines (0003).

### ADR 0002: code structure, build and toolchain (engine-architect)
Decides:
1. **Layout and toolchains:** one repository with a Cargo workspace and npm workspaces, Vite, pinned Rust, Node,
   TypeScript and wasm-bindgen CLI (R5-LAYOUT-REC1, R5-TSLINT-REC1, R5-RUSTLINT-REC6, R5-CI-REC2–REC3); the phase 1
   move of the old game to `legacy/` with the oracle still runnable (R5-LAYOUT-REC6, OQ5; R5-GATE-OQ3).
2. **Crates, packages and dependency rules** as gate checks (R5-LAYOUT-REC2): where the navigation bake and R4's
   checks live (critic rank 24), the cache question (rank 40), one feature set for the sim on every target
   (R5-LAYOUT-REC4), build profiles and overflow (R1-RR-OQ4, R5-RUSTLINT-REC4), the panic strategy (with 0004's worker
   restart, rank 28), the Wasm size budget (R1-RR-OQ5: the research gives no number; set from the spike's measurement).
3. **The Rust ↔ TypeScript boundary** (R1-BND-REC1–REC6): one dedicated module Worker, single-threaded; the per-tick
   output block (layout, versioning, pool, transfer or shared ring: R1-XU-OQ3, OQ4); posting one command per tick from
   the page (the command's fields are 0001's); generated types from three sources into `packages/types`
   (R1-BND-REC4, R5-LAYOUT-REC3; which source content types come from is 0005's, rank 31); the isolation flag and the
   production isolation decision (R1-BND-REC5, R5-XQ-OQ3); the Worker owning the connection (R1-BND-REC6; the protocol
   over it is 0001's).
4. **The client + local server skeleton** (SLICE-ORDER condition 1; brief §2 "the foundation"): the same Worker build
   runs the authority for a local match and the predicting copy online; the page never handles a network packet; the
   local server speaks 0001's protocol (I-3).
5. **Lint, format and tests** (R5-TSLINT, R5-RUSTLINT, R5-TEST): the test layers and tools, the determinism suite's
   tools (R5-TEST-REC2, for 0004's suite), goldens against legacy, the network simulator (profiles from 0001), the soak
   and load harness, the flake rule, the bench tool and method (rank 36).
6. **CI and the gate v2** (R5-CI, R5-GATE): the jobs and the one required check, caching, budgets as data reviewed
   like code (R5-GATE-REC4: every draft's budgets live there), the mutation suite; the Sentry upload job and the smoke
   test's network rule (rank 34).
7. **The preview build** (R5-PREV-REC1, REC4, OQ5, OQ6): the build job, the deploy job's shape and permissions, the
   generated header sets; how a multiplayer preview gets its server binary and certificate hash (rank 20).
8. **The editor build:** its own entry and package; whether it still opens by double-click with a Wasm validator
   (rank 25; Mustafa if EDITOR_V2 §5's rule must change).
9. **Its spike: the Rust ↔ TypeScript boundary spike.** Its migration plan carries the foundation (brief §2), ending
   when an empty scene runs through the full stack in the browser with CI green and a preview link.

Takes: the protocol (0001), the tick and determinism rules (0004), the client host, tokens, secrets policy and runner
costs (0006), the content schema (0005), the frame budget rows (0003).

### ADR 0003: rendering (graphics-engineer)
Decides:
1. **Renderer and backends:** one WebGPURenderer with TSL, the WebGL 2 backend as the fallback with the same features
   (F-A), the three.js pin and upgrade policy, pipelines built behind the loading screen, the backend reported
   (R3-WGPU-REC1–REC4, R3-GL2-REC1–REC3); dropping the fallback is R3-GL2-OQ5 (Mustafa).
2. **The look, against LOOK_V2:** how each `[DECIDED]` item is achieved, the `[DEFAULT]` items built as defaults, and
   nothing decided where LOOK_V2 says `[OPEN]` (the tree style) (plan rule 6; part e lists every item). The techniques
   are measured before use (R3-LOOK-REC1–REC3).
3. **Characters and large units on screen** (R3-CHAR-REC1–REC4): the skinning path, the animation budget LOD,
   per-instance data written from the output block, multi-part vehicles from the sim's part records.
4. **Client budgets and reference machines** (R3-BUDGET-REC1–REC3): frame time, main-thread CPU (with the page's cost
   of the sim), GPU, draw calls, triangles, GPU memory, heap, quality tiers as data; it names the minimum laptop and the
   recommended desktop for Mustafa and the Advisor (R1-TICK-OQ6, R1-BND-OQ3, R3-BUDGET-OQ1).
5. **Measurement** (R3-MEASURE-REC1–REC4): the harness, the benchmark scene (a recorded match: rests on 0004's
   determinism), what CI gates and what is only reported, the perf overlay; what client field data is measured (rank
   26; where it is stored is 0006's).
6. **Spatial audio** (R3-AUDIO-REC1–REC4): the listener, the voice manager, occlusion (zones first; rays only with 0004,
   rank 35), sounds triggered from the per-tick event section.
7. **The per-frame flow on the page:** read the newest blocks, interpolate, write instance data, render, play sounds,
   update the HUD; presentation randomness on the page's own generator (R1-DET-REC4).
8. **Its spike: the look proof test**, judged by Mustafa in the browser side by side with legacy screenshots (LOOK_V2
   §9), and the render budget bench (part g).

Takes: the block layout and the event section (0002), the snapshot fields (0001), asset formats, material slots and
per-unit budgets (0005), vehicle-part records, floors and structure states (0004).

### ADR 0004: the simulation core (rust-reviewer)
Decides:
1. **The sim state** as plain data (R1-ECS-REC1–REC3): typed tables, phases in a fixed order, entity ids; effects as
   rows or as entities (R1-ABL-OQ10); parallelism inside one match (R1-ECS-OQ4).
2. **The fixed-tick mechanism:** an integer tick counter, a build-time `dt`, durations converted to ticks at load with
   carried remainders as the one time representation (R1-TICK-REC1, R1-ABL-REC3, R1-XU-OQ7), cadences (REC3), the
   server's due-tick loop with bounded catch-up (REC4), the Worker's pacing and its gap rule (REC5), hidden tabs
   (R1-TICK-OQ2 = R1-BND-OQ2), map validation with the sim's own integrator (REC6).
3. **The sim step share** (25 % of the tick period) and every part inside it (movement and physics, avoidance, the
   ability, effect and weapon step, navigation, the visibility pass), and **the client's sim share** (40 %), restated
   at 30 Hz.
4. **Determinism** (R1-DET-REC1–REC7): the two levels, float rules, RNG streams (the authority-only spread stream
   included), iteration order and the hash-map ban as a rule (its lint is 0002's), the determinism suite and the state
   hash, FMA and SIMD across targets (R1-RR-OQ1), parry3d's flag (critic rank 3).
5. **Physics** (R4-PHYS-REC1–REC5): parry3d queries, one static collision world, swept projectiles, kinematic compound
   vehicles, f32.
6. **Navigation, floors, sight and obstacles** (R4-NAV, R4-FLOORS, R4-LOS, R4-OBST): the baked navigation and its tool,
   the path queue, avoidance, navigation only where bots run, floor layers with ceilings, the shared sight query,
   moving obstacles, structures baked in both states (the sim half of LOOK_V2 §4's damage stages).
7. **The game rules the sim executes:** bot behaviour (LOOP_V2 §10; a research gap, part h), economy and objectives
   (§6.5), the ability system's closed set of operations and the damage pipeline as 0005 defines them, the player's
   upgrade picks held on the player's slot, not the unit's (UPGRADE-SCOPE), moment recognition (§6.8) and the
   tick-stamped events the match record is built from (§6.6).
8. **Client robustness:** recovery from a trap in the client's Wasm sim (rank 28: the worker restarts from the server's
   state; the resync path is 0001's, the panic strategy 0002's).
9. **Its spike: the sim core spike** (part g).

Takes: the tick rate (0001), the definitions (0005), the boundary (0002), the reference server (0006) and the client
machines (0003).

### ADR 0005: content data, the ability system and the asset pipeline (engine-architect, game-designer, graphics-engineer)
Decides:
1. **Content data** (R1-DATA-REC1–REC8): JSON, one Rust content crate that loads and validates everywhere, the schema
   generated from the Rust types, format names and versions with migrations, hot reload in development (what a reload
   may change: R1-DATA-OQ7, R1-XU-OQ10, R4-FLOORS-OQ4), the hashed bundle and the content hash every match records,
   server validation with length rules (R1-RR-OQ2), the one-off legacy import; where content's TypeScript types come
   from (rank 31); how definitions sit in files (R1-XU-OQ9); faction variants (R1-DATA-OQ6); the mode file's format
   (R2-ROOM-REC7 is 0001's); unit data, with one top speed per unit type (R2-XU-OQ14); quality tiers and feel-kit data
   (LOOP_V2 §7) as data.
2. **The ability system** (game-designer; R1-ABL-REC1, REC2, REC5): a reduced GAS subset as data over a closed set of
   operations, weapons' own bookkeeping on the shared damage pipeline, one table per kind linked by validated ids.
   **In-match upgrades** ride on it: owned by the player (UPGRADE-SCOPE, brief §1: GAS on the PlayerState, not the
   Pawn; picks survive death and reset at the start of every match); the brief's question how a pick applies after a switch to a different unit is answered by Mustafa's
   UPGRADE-SCOPE update (tracks survive switching; part j, J5, which replaces this item's upgrade wording).
3. **The asset pipeline** (LOOK_V2 §8; graphics-engineer for the GPU rules): glTF/GLB in, the automated step (compress,
   build LODs, rename materials into the faction colour slots), budgets per unit (triangles, materials, textures), no
   Spline runtime; assets carry no baked look, since the engine applies it (0003). The research has no findings here
   (part h; the plan's ASSET-RESEARCH (a): open questions, researched before the asset spike).
4. **Moment rules and telemetry ids as data** (their thresholds are DESIGN, R6-TEL-OQ3).
5. **Its spikes: the content and ability spike**, and the asset spike after its research (part g).

Takes: execution (0004), drawing (0003), the editor build and generated-type package (0002), the reload, override and
content-hash messages (0001), bundle hosting and caching (0006).

### ADR 0006: infrastructure and operations (infra-engineer)
Decides:
1. **The client host and previews:** Cloudflare Pages with Workers static assets as the alternative (R6-HOST-REC1,
   R5-PREV-REC2), privacy (PREVIEW-WALL (a)), cleanup and PREV-DELETE (R5-PREV-REC3), the staging game server
   (R6-HOST-REC3), the production header set as the host serves it (the CSP stays open).
2. **Game servers:** Hetzner Cloud in one EU location, the reference server (CCX23, with the shared CX33 measured
   beside it: R6-HOST-REC2), a supervisor per VM, the firewall, where the lobby runs and stores its data (rank 8,
   R2-ROOM-OQ9), certificates (a certificate per hostname: R6-HOST-REC4, R2-TRN-REC5; carry-forward 4).
3. **Costs** (R6-COST): the start and goal budgets, rooms per VM from 0001's measurement, CI runner costs (rank 33).
4. **Logs, metrics, crash reports and data protection** (R6-OBS-REC1–REC4).
5. **Balance telemetry stored and analysed:** the match record's shape (R6-TEL-REC1), its storage (REC2) and analysis
   (REC3), retention.
6. **The security baseline:** listed open with interim positions (BASELINE-WHEN (b)), with every item that waits on it
   (R6-SEC-OQ1; rank 18).
7. **Its spike:** the preview hosting check (PREV-DELETE), and the operations marks inside the netcode proof test
   (part g).

Takes: the room process and the lobby protocol (0001), the CI workflows (0002), what client field data is (0003), the
events the record is built from (0004), the content hash (0005).

### The seams, settled
| # | Seam | One side decides | Other side decides | Rule |
|---|---|---|---|---|
| S1 | Tick | 0001: the tick rate rule under TICK-30 and the 60 Hz margin (N3); the snapshot rate; the server tick budget's total (50 %) and the server-only share (25 %) and its parts | 0004: the fixed-tick mechanism (counter, `dt`, conversion, cadences, server loop, Worker pacing); the sim step share (25 %) and its parts; the client's sim share (40 %); determinism | 0001 adds 0004's sim step share into the total; neither restates the other's parts. R1-TICK-REC2's P1 and P2 sim marks are 0004's; P3 (prediction replay) is 0001's. |
| S2 | Boundary vs protocol | 0002: the Worker, the output block's layout and versioning, the page-to-worker command posting, the generated types, the isolation flag, the Worker owning the connection, the client + local server skeleton | 0001: every message's fields and meaning, including the command, the snapshot fields the block is filled from, the events and their visibility, and the rule that the local server path uses the same messages | The block's contents derived from snapshots are 0001's fields in 0002's layout (ranks 4, 16). Whether the local server is reached through 0001's transport interface is interface I-3. |
| S3 | CI and previews | 0002: CI, the gate v2, the preview build, the deploy job's shape, the generated header sets, the close and sweep jobs as workflows | 0006: the host, its token scope, the access wall, the cleanup policy and PREV-DELETE, the staging game server, runner costs, secrets policy | The deploy job runs what 0006's host needs; 0006 never specifies a workflow step, 0002 never a host setting. |
| S4 | Rooms vs machines | 0001: the room lifecycle and process model, threads, tokens, reconnect, private rooms, the lobby protocol, the browser-side certificate requirement | 0006: the VMs, the supervisor, the firewall, the lobby's hosting and store, the production certificate form and issuance, rooms per VM as a cost figure, monitoring | 0001's spike measures rooms per process and per VM; 0006 turns the number into cost (R6-COST-OQ1). R2-TRN-REC5 and R2-TRN-OQ4 go to 0006 (certificates). |
| S5 | Content vs execution vs drawing | 0005: the definitions, their schema, validation, versions, hot reload, the bundle and its hash, the ability data model and in-match upgrades, the asset pipeline and per-unit budgets | 0004: executing them inside the sim step (order, integer time, attribute maths, damage, where the upgrade state lives); 0003: drawing them (materials in TSL, LOD switching at run time, faction colour as per-instance data) | A definition is typed once (0005) and read by id; the ability step's cost (1 ms) is a part of 0004's sim step share. |
| S6 | Moments and telemetry | 0004: moments recognized by a sim phase; tick-stamped events (kills with source ids, purchases, uplinks, harvesters, tiers) emitted into the event stream; 0005: moment rules and telemetry ids as data; 0001: the room collects the record and writes it at Ended (R2-ROOM-REC1) | 0006: the record's shape, its storage, analysis and retention (R6-TEL-REC1–REC3) | The moment rules and the telemetry read the same event stream (R6-TEL-REC1). Thresholds are DESIGN (R6-TEL-OQ3). |
| S7 | Spatial audio | 0003 (R3-AUDIO): everything on the page | 0004: sight queries, only if occlusion option O-B is chosen (rank 35) | |
| S8 | Bots and navigation | 0004: bot behaviour, navigation, sight, avoidance, where navigation runs | 0001: bots fill slots on the server and drive the human input path (R2-ROOM-REC6); bots see the present (R2-SYNC-OQ13 is 0004's tuning) | |
| S9 | The look vs the asset rules | 0003: how every LOOK_V2 item is achieved on screen (part e) | 0005: what an asset must carry for it (silhouettes and proportions are authored, material slot names and faction colour slots, emissive slots, LOD levels, per-unit budgets) | Material slot names are defined in 0005 and consumed by 0003 (I-8). Neither decides the look beyond LOOK_V2; the tree style stays open. |
| S10 | The editor | 0002: the editor package, entry and the double-click question | 0005: the map document is content data (schema, versions, migration); 0004: reach checks and the navigation bake the validation panel runs (R1-TICK-REC6, R4-NAV-REC1) | EDITOR_V2's own order of work stays a phase 1 track (slice 5); no draft re-decides its five rules. |
| S11 | Offline and local play | DESIGN: whether offline play ships, and at the largest mode on the minimum laptop (R1-BND-OQ1, R1-TICK-OQ7, rank 15) | 0002 and 0004: the local server from slice 1 and local play with bots in slice 2 are settled (SLICE-ORDER), whatever the answer | The drafts assume local play with bots from slice 2. |
| S12 | Security | 0006: the security baseline listed open, its interim positions and every item waiting on it | 0001, 0002, 0005: they cite the baseline item where their design waits on it (token keys, guest secret, advisories policy, CSP, the `textContent` lint) | |

## b. Ownership of every id

Generated from `engine-architect-decision-map-ids.tsv` (the scratch script `assign.py` that wrote both is not kept;
the TSV is the source). Counts per owner (521 ids): 0001: 127 · 0002: 104 · 0003: 71 · 0004: 82 · 0005: 34 · 0006: 65 ·
DESIGN: 38 · SPIKE_PLAN: 0 (no research id is about the spike set itself; spike items stay with the draft whose
decision they test, and SPIKE_PLAN is cited). Rows are in research-ids.tsv order; "-" = no other draft must cite it,
or no note. A recommendation's note is its starting point only: the owner adopts, changes or rejects it.

| id | owner | must cite | note |
|---|---|---|---|
| R1-ECS-REC1 | 0004 | 0001,0002,SPIKE_PLAN | starting point; its Rapier reasons and mark 5 change with R4-PHYS-REC1 (XT rank 3); marks restated at 30 Hz (TICK-30) |
| R1-ECS-REC2 | 0004 | 0002 | lint side is R5-RUSTLINT-REC2 (0002) |
| R1-ECS-REC3 | 0004 | 0001,0002,0006 | wire widths are XT rank 27 (0001) |
| R1-ECS-REC4 | 0004 | - | conditional on Rapier; R4-PHYS-REC1 recommends parry3d (XT rank 3) |
| R1-TICK-REC1 | 0004 | 0001,0005 | durations converted at load: content side 0005 |
| R1-TICK-REC2 | 0001 | 0004,SPIKE_PLAN | the tick rate rule: changed by TICK-30 (30 Hz baseline, 60 Hz only with a 60 Hz margin of at least 25 %); P1/P2 sim marks are 0004, P3 replay 0001 |
| R1-TICK-REC3 | 0004 | 0006 | telemetry sampling cadence used by R6-TEL-REC1 |
| R1-TICK-REC4 | 0004 | 0001,0006 | the server loop; snapshot sending after the tick is 0001; overrun metrics 0006; "15 ticks" is the 60 Hz form |
| R1-TICK-REC5 | 0004 | 0001,0002,0003 | Worker pacing is the tick mechanism (0004); Worker host 0002; per-frame interpolation 0003; resync 0001. Copy row at r3:771 is a reference |
| R1-TICK-REC6 | 0004 | 0002,0005 | editor reach check uses the sim integrator |
| R1-DET-REC1 | 0004 | 0001 | lockstep rejected; cross-target identity vs E3 is XT rank 7 |
| R1-DET-REC2 | 0004 | 0002 | lint enforcement in 0002 (R5-RUSTLINT) |
| R1-DET-REC3 | 0004 | 0002 | changed by R4-PHYS-REC1: parry3d with enhanced-determinism, not Rapier (XT rank 3) |
| R1-DET-REC4 | 0004 | 0001,0003 | weapon spread on an authority-only stream (SEC-SPREAD (a)); presentation RNG is the page's |
| R1-DET-REC5 | 0004 | 0002 | hash-map ban enforced by R5-RUSTLINT-REC2 |
| R1-DET-REC6 | 0004 | 0002,0006 | suite tools are R5-TEST-REC2 (0002); server CPU in the matrix (0006) |
| R1-DET-REC7 | 0004 | - | - |
| R1-BND-REC1 | 0002 | 0003,0004 | the boundary spike |
| R1-BND-REC2 | 0002 | 0001,0003,0004 | block layout 0002; its remote-entity fields come from 0001 snapshots (XT ranks 4, 16). Copy row at r3:524 is a reference |
| R1-BND-REC3 | 0002 | 0001 | the posting mechanism (C1) is 0002; command fields are 0001 (R2-SYNC-REC2) |
| R1-BND-REC4 | 0002 | 0001,0005 | content-schema types: XT rank 31 (0005) |
| R1-BND-REC5 | 0002 | 0003,0006 | the isolation flag; production headers on the client host (0006) |
| R1-BND-REC6 | 0002 | 0001,0004 | where the connection lives (the Worker) is 0002; what flows over it is 0001 |
| R1-ABL-REC1 | 0005 | 0004 | data model 0005; the sim executes the closed set of operations (0004) |
| R1-ABL-REC2 | 0005 | 0004 | the damage pipeline runs in the sim |
| R1-ABL-REC3 | 0004 | 0005 | one time representation with R1-TICK-REC1 (R1-XU-OQ7) |
| R1-ABL-REC4 | 0001 | 0004,0005 | the prediction scope; abilities wait for ABL-PREDICT |
| R1-ABL-REC5 | 0005 | 0004,0006 | ids also key telemetry (R6-TEL-REC1) |
| R1-ABL-REC6 | 0004 | 0005 | a part of the sim step share (1 ms); 0004 owns the share and its parts |
| R1-DATA-REC1 | 0005 | 0002 | - |
| R1-DATA-REC2 | 0005 | 0002,0004 | content is the sim crate's only dependency (R5-LAYOUT-REC2) |
| R1-DATA-REC3 | 0005 | 0002 | schema output lives in packages/types (R5-LAYOUT-REC3) |
| R1-DATA-REC4 | 0005 | 0002 | golden migration fixtures in the gate |
| R1-DATA-REC5 | 0005 | 0001,0002,0004 | reload messages travel on the game connection (0001); Vite plugin (0002); swap between ticks (0004) |
| R1-DATA-REC6 | 0005 | 0001,0006 | the content hash at join (XT rank 13, 0001); bundle caching on the client host and the match record (0006) |
| R1-DATA-REC7 | 0005 | 0001,0006 | private-match overrides (0001); listed for the security baseline (0006) |
| R1-DATA-REC8 | 0005 | 0002 | golden fixtures R5-TEST-REC3 |
| R1-ECS-OQ1 | 0004 | 0002 | size measured by REC1 mark 5; the Wasm size budget itself is R1-RR-OQ5 (0002) |
| R1-ECS-OQ2 | 0004 | - | settled at the R1 fix step: hand-written clone_from |
| R1-ECS-OQ3 | 0001 | 0004 | prediction scope decides (R2-SYNC-REC3; XT rank 14) |
| R1-ECS-OQ4 | 0004 | 0001,0006 | one room per process (R2-ROOM-REC2); cost per room (0006) |
| R1-ECS-OQ5 | 0004 | - | - |
| R1-ECS-OQ6 | 0004 | - | - |
| R1-ECS-OQ7 | 0004 | 0005 | roster and ability lists decide (game-designer) |
| R1-TICK-OQ1 | 0001 | 0004 | 60 vs 64 Hz; under TICK-30 it matters only if 60 Hz reaches the 60 Hz margin |
| R1-TICK-OQ2 | 0004 | 0002 | stated once with R1-BND-OQ2 (R1-XU-OQ12) |
| R1-TICK-OQ3 | 0001 | - | shipped tick rates (unread sources) |
| R1-TICK-OQ4 | 0001 | 0004 | sub-tick timestamps: live under TICK-30 (XT rank 12) |
| R1-TICK-OQ5 | 0004 | 0001 | measured by P1-P3 and the netcode proof test |
| R1-TICK-OQ6 | 0003 | 0001,0004,0006,SPIKE_PLAN | client machines named in 0003 (R3-BUDGET-REC1); the server half answered by R6-HOST-REC2 (0006); Mustafa approves |
| R1-TICK-OQ7 | DESIGN | 0002,0003,0004 | Mustafa and the Advisor: offline at the largest mode on the minimum laptop (sets N in P2) |
| R1-TICK-OQ8 | 0001 | 0004 | client tick nudging is time sync (XT rank 12) |
| R1-TICK-OQ9 | 0004 | 0001 | substeps for big vehicles: live under TICK-30 |
| R1-DET-OQ1 | 0004 | - | - |
| R1-DET-OQ2 | 0004 | 0002 | simd128 stays off until the suite passes |
| R1-DET-OQ3 | 0004 | - | answered by R4-PHYS-REC5 (f32) unless a map exceeds 2 km |
| R1-DET-OQ4 | 0004 | - | - |
| R1-DET-OQ5 | 0004 | - | - |
| R1-DET-OQ6 | 0001 | 0004 | answered by R2-SYNC-REC3; abilities ride with ABL-PREDICT |
| R1-DET-OQ7 | 0006 | 0004 | answered by R6-HOST-REC2: x86-64 to start; ARM only after the suite covers it |
| R1-DET-OQ8 | 0004 | - | - |
| R1-DET-OQ9 | 0001 | 0004,0006 | hash comparison as divergence telemetry is protocol (XT rank 13) |
| R1-DET-OQ10 | 0004 | - | - |
| R1-DET-OQ11 | 0004 | - | - |
| R1-BND-OQ1 | DESIGN | 0002,0004,0006 | Mustafa (VISION_V2). Partly answered by SLICE-ORDER: slice 2 is "played locally" and the game is client + local server from slice 1; offline as a shipped feature stays open |
| R1-BND-OQ2 | 0004 | 0002 | stated once with R1-TICK-OQ2 (R1-XU-OQ12) |
| R1-BND-OQ3 | 0003 | 0002,0004 | minimum laptop named in 0003 (R3-BUDGET-REC1) |
| R1-BND-OQ4 | 0002 | - | - |
| R1-BND-OQ5 | 0006 | 0002 | Mustafa with ADR-0006, only if production isolation is adopted (XT rank 38) |
| R1-BND-OQ6 | 0002 | 0001,0003 | block size measured in the boundary spike |
| R1-BND-OQ7 | 0002 | - | - |
| R1-BND-OQ8 | 0001 | 0002 | snapshot format decides (XT rank 13) |
| R1-BND-OQ9 | 0002 | 0006 | - |
| R1-ABL-OQ1 | 0005 | - | - |
| R1-ABL-OQ2 | 0005 | - | - |
| R1-ABL-OQ3 | 0005 | - | - |
| R1-ABL-OQ4 | 0001 | 0005 | rides with ABL-PREDICT and R2-SYNC-REC3 |
| R1-ABL-OQ5 | 0004 | 0005 | determinism of attribute math |
| R1-ABL-OQ6 | DESIGN | 0005 | game rules the system must carry but not decide (stacking, regen, seats, marking, active reload numbers) |
| R1-ABL-OQ7 | 0004 | 0005 | under TICK-30 every legacy weapon fits one shot per tick (r1 tuning arithmetic) |
| R1-ABL-OQ8 | 0001 | 0005 | interest decides who sees which tags |
| R1-ABL-OQ9 | 0005 | - | - |
| R1-ABL-OQ10 | 0004 | 0005 | sim data layout decides |
| R1-DATA-OQ1 | 0005 | - | - |
| R1-DATA-OQ2 | 0005 | 0002 | Wasm size |
| R1-DATA-OQ3 | 0005 | 0001 | Mustafa's call on production mid-match changes |
| R1-DATA-OQ4 | 0005 | - | - |
| R1-DATA-OQ5 | 0005 | - | - |
| R1-DATA-OQ6 | 0005 | - | - |
| R1-DATA-OQ7 | 0005 | 0004 | - |
| R1-DATA-OQ8 | 0005 | 0002 | - |
| R1-DATA-OQ9 | 0005 | - | - |
| R1-DATA-OQ10 | DESIGN | 0001,0005 | game-designer and Mustafa: which tunables a private match exposes |
| R1-DATA-OQ11 | 0002 | 0005 | editor build (XT rank 25) |
| R1-DATA-OQ12 | 0005 | 0004 | Mustafa and the Advisor: iteration speed vs REC5 marks 2 and 3 |
| R1-XU-OQ1 | 0004 | 0001,SPIKE_PLAN | settled by GLOSSARY (one stress scene, one server tick budget) |
| R1-XU-OQ2 | 0001 | 0004 | prediction scope (XT rank 14) |
| R1-XU-OQ3 | 0002 | 0003,0004 | B2 against B3 (R1-BND-REC2 mark 5) |
| R1-XU-OQ4 | 0002 | 0004 | W3 needs shared memory and so isolation (0002's call) |
| R1-XU-OQ5 | 0004 | - | settled: per instance |
| R1-XU-OQ6 | 0004 | - | settled: tier "other" |
| R1-XU-OQ7 | 0004 | 0001,0005 | one representation, one mark |
| R1-XU-OQ8 | 0005 | 0002 | XT rank 31 |
| R1-XU-OQ9 | 0005 | - | - |
| R1-XU-OQ10 | 0005 | 0004 | what a reload may change; R1-TICK-REC1 makes the tick a build-time constant |
| R1-XU-OQ11 | 0004 | 0001 | - |
| R1-XU-OQ12 | 0004 | 0002,0003 | hidden tabs here; reference hardware is R1-TICK-OQ6 (0003); offline play is R1-BND-OQ1 (DESIGN) |
| R1-RR-OQ1 | 0004 | 0002,0006 | build flags (no +fma) and ARM hosts |
| R1-RR-OQ2 | 0005 | 0001,0006 | length rules on collections; bounded queues on the network side |
| R1-RR-OQ3 | 0004 | 0001,0002,0005 | worker restart (XT rank 28); panic strategy 0002; loaders return errors 0005 |
| R1-RR-OQ4 | 0002 | 0004 | build profiles; R5-RUSTLINT-REC4 |
| R1-RR-OQ5 | 0002 | 0003,0004 | the Wasm size budget: set from the spike's measured size (no research number) |
| R1-RR-OQ6 | 0002 | 0004 | the block writer; the state hash already reads fields |
| R1-RR-OQ7 | 0006 | 0002 | security baseline (BASELINE-WHEN (b)); mechanism R5-GATE-REC1 |
| R2-SYNC-REC1 | 0001 | 0004 | - |
| R2-SYNC-REC2 | 0001 | 0002 | command fields here; the page-to-worker posting is R1-BND-REC3 (0002) |
| R2-SYNC-REC3 | 0001 | 0004,0005 | ABL-PREDICT open; SEC-SPREAD (a) applied |
| R2-SYNC-REC4 | 0001 | 0002,0003 | where the buffer lives: XT rank 4 |
| R2-SYNC-REC5 | 0001 | 0004 | rewind cap: R2-XU-OQ1 |
| R2-SYNC-REC6 | 0001 | 0004 | the shared trajectory function runs in the sim crate (XT rank 7) |
| R2-SYNC-REC7 | 0001 | 0004 | - |
| R2-SYNC-REC8 | 0001 | 0002,0003,0004 | XT rank 2; R4-LOS-REC3 |
| R2-SYNC-REC9 | 0001 | 0004 | - |
| R2-BUD-REC1 | 0001 | 0004 | snapshot 30 Hz; the tick half is replaced by TICK-30 |
| R2-BUD-REC2 | 0001 | 0004 | E3 depends on XT rank 7 |
| R2-BUD-REC3 | 0001 | - | - |
| R2-BUD-REC4 | 0001 | - | - |
| R2-BUD-REC5 | 0001 | - | - |
| R2-BUD-REC6 | 0001 | 0004,0006 | server tick budget and server-only share; its reason at r2:2402-2403 is carry-forward 2 |
| R2-INT-REC1 | 0001 | 0003 | camera parameters as data |
| R2-INT-REC2 | 0001 | 0004,0005 | areas and portals from map data (R4-FLOORS-REC2) |
| R2-INT-REC3 | 0001 | 0004 | the pass runs inside the sim step share (0004 owns the number) |
| R2-INT-REC4 | 0001 | - | - |
| R2-INT-REC5 | 0001 | 0005 | visibility classes held as data |
| R2-INT-REC6 | 0001 | 0003 | - |
| R2-INT-REC7 | 0001 | - | conditional on R2-XU-OQ17 (spectators) |
| R2-TRN-REC1 | 0001 | 0006 | version floors wait on the security baseline |
| R2-TRN-REC2 | 0001 | - | - |
| R2-TRN-REC3 | 0001 | 0006 | firewall TCP range |
| R2-TRN-REC4 | 0001 | 0002 | thin transport interface; a local backend for the client + local server skeleton is an interface item |
| R2-TRN-REC5 | 0006 | 0001 | certificates (seam: machines and certificates are 0006); carry-forward 4 |
| R2-TRN-REC6 | 0001 | - | - |
| R2-ROOM-REC1 | 0001 | 0006 | records written at Ended |
| R2-ROOM-REC2 | 0001 | 0004,0006 | the room process; the VMs and the supervisor are 0006 |
| R2-ROOM-REC3 | 0001 | 0006 | key storage is a security-baseline item |
| R2-ROOM-REC4 | 0001 | - | - |
| R2-ROOM-REC5 | 0001 | - | - |
| R2-ROOM-REC6 | 0001 | 0004 | bot AI is 0004 |
| R2-ROOM-REC7 | 0001 | 0005 | mode file format and validation are content data (0005) |
| R2-ROOM-REC8 | 0001 | 0006 | guest store hosting (R2-ROOM-OQ9) |
| R2-SYNC-OQ1 | 0001 | - | - |
| R2-SYNC-OQ2 | 0001 | - | - |
| R2-SYNC-OQ3 | 0001 | - | - |
| R2-SYNC-OQ4 | 0001 | - | - |
| R2-SYNC-OQ5 | 0001 | 0004 | body-blocking is DESIGN (XT rank 30); the spike measures the rest |
| R2-SYNC-OQ6 | 0001 | - | - |
| R2-SYNC-OQ7 | DESIGN | 0001,0004,0005 | VISION_V2 §6.4 [OPEN]: seats per vehicle (Mustafa) |
| R2-SYNC-OQ8 | 0001 | 0004 | - |
| R2-SYNC-OQ9 | 0001 | 0005 | design call plus the REC5 spike extended to melee |
| R2-SYNC-OQ10 | 0001 | 0005 | ABL-PREDICT |
| R2-SYNC-OQ11 | 0001 | 0003,0004 | answered: SEC-SPREAD (a), server-only spread |
| R2-SYNC-OQ12 | 0001 | - | - |
| R2-SYNC-OQ13 | 0004 | 0001 | bot reaction tuning |
| R2-SYNC-OQ14 | 0001 | - | - |
| R2-SYNC-OQ15 | 0001 | - | - |
| R2-BUD-OQ1 | 0001 | - | - |
| R2-BUD-OQ2 | 0001 | - | - |
| R2-BUD-OQ3 | 0001 | - | - |
| R2-BUD-OQ4 | 0001 | 0004 | map size, LOOP_V2 §9 [OPEN] |
| R2-BUD-OQ5 | DESIGN | 0001,0004,SPIKE_PLAN | game-designer's caps on vehicles and projectiles; until then the stress scene's counts |
| R2-BUD-OQ6 | 0001 | - | answered: MODES-3 (a) |
| R2-BUD-OQ7 | 0001 | - | - |
| R2-BUD-OQ8 | 0001 | 0004 | XT rank 7 |
| R2-BUD-OQ9 | 0001 | - | - |
| R2-BUD-OQ10 | 0001 | - | - |
| R2-BUD-OQ11 | 0001 | 0006 | - |
| R2-INT-OQ1 | DESIGN | 0001,0003,0004 | what a player may see: LOOK_V2 does not settle it (Mustafa with the game-designer) |
| R2-INT-OQ2 | DESIGN | 0001,0003 | camera framing: LOOK_V2 does not set it |
| R2-INT-OQ3 | DESIGN | 0001 | team-private economy state (LOOP_V2 owner) |
| R2-INT-OQ4 | DESIGN | 0001,0003 | what the minimap shows |
| R2-INT-OQ5 | 0001 | 0004 | - |
| R2-INT-OQ6 | 0001 | 0003 | game design plus a reader-client test |
| R2-INT-OQ7 | 0001 | - | - |
| R2-INT-OQ8 | DESIGN | 0001,0005 | Spy rules, LOOP_V2 §5 [OPEN] |
| R2-TRN-OQ1 | 0001 | - | - |
| R2-TRN-OQ2 | 0001 | - | - |
| R2-TRN-OQ3 | 0001 | - | - |
| R2-TRN-OQ4 | 0006 | 0001 | BASELINE-WHEN (b): per hostname holds |
| R2-TRN-OQ5 | 0001 | - | - |
| R2-TRN-OQ6 | 0001 | - | - |
| R2-TRN-OQ7 | 0001 | 0006 | connection telemetry |
| R2-TRN-OQ8 | 0001 | - | - |
| R2-TRN-OQ9 | 0001 | - | - |
| R2-ROOM-OQ1 | 0001 | 0006 | - |
| R2-ROOM-OQ2 | 0001 | 0006 | security-auditor review; key storage under the security baseline |
| R2-ROOM-OQ3 | DESIGN | 0001 | game rule |
| R2-ROOM-OQ4 | DESIGN | 0001 | game rule |
| R2-ROOM-OQ5 | DESIGN | 0001,0005 | Mustafa: what a private host may set |
| R2-ROOM-OQ6 | DESIGN | 0001 | game rule |
| R2-ROOM-OQ7 | DESIGN | 0001,0006 | also R6-TEL-OQ2 |
| R2-ROOM-OQ8 | 0001 | - | - |
| R2-ROOM-OQ9 | 0006 | 0001,0002 | XT rank 8 |
| R2-ROOM-OQ10 | 0001 | - | - |
| R2-ROOM-OQ11 | 0006 | 0001 | the CSP is the security baseline's |
| R2-XU-OQ1 | 0001 | - | - |
| R2-XU-OQ2 | 0001 | - | - |
| R2-XU-OQ3 | 0001 | - | - |
| R2-XU-OQ4 | 0001 | - | - |
| R2-XU-OQ5 | 0001 | - | - |
| R2-XU-OQ6 | 0001 | - | - |
| R2-XU-OQ7 | 0001 | 0004 | - |
| R2-XU-OQ8 | 0001 | 0004 | settled by GLOSSARY |
| R2-XU-OQ9 | 0006 | 0001 | answered by R6-HOST-REC2 (CCX23) |
| R2-XU-OQ10 | 0001 | - | - |
| R2-XU-OQ11 | 0001 | - | - |
| R2-XU-OQ12 | 0001 | 0002 | named profiles; the simulator is R5-TEST-REC4 (0002) |
| R2-XU-OQ13 | 0001 | - | answered: MODES-3 (a) |
| R2-XU-OQ14 | 0005 | 0001,0004 | one top speed per unit type is unit data |
| R2-XU-OQ15 | 0001 | - | - |
| R2-XU-OQ16 | 0001 | - | - |
| R2-XU-OQ17 | DESIGN | 0001 | Mustafa: whether rooms have spectators or casters |
| R2-XU-OQ18 | 0001 | 0002,0003,0004,0005,0006 | trace rule for §3/§4-only recs: the Advisor; applies to every draft |
| R3-WGPU-REC1 | 0003 | - | - |
| R3-WGPU-REC2 | 0003 | - | - |
| R3-WGPU-REC3 | 0003 | - | - |
| R3-WGPU-REC4 | 0003 | 0002 | secure context; test screenshots print the backend |
| R3-GL2-REC1 | 0003 | - | - |
| R3-GL2-REC2 | 0003 | - | - |
| R3-GL2-REC3 | 0003 | - | - |
| R3-LOOK-REC1 | 0003 | SPIKE_PLAN | the look proof test; LOOK_V2 now exists, so the table serves the [DECIDED] items |
| R3-LOOK-REC2 | 0003 | - | - |
| R3-LOOK-REC3 | 0003 | SPIKE_PLAN | LOOK_V2 keeps outlines (§2 ink, §6.2 line weight) |
| R3-CHAR-REC1 | 0003 | 0005 | per-unit budgets in the asset pipeline |
| R3-CHAR-REC2 | 0003 | - | - |
| R3-CHAR-REC3 | 0003 | 0002 | reads the output block |
| R3-CHAR-REC4 | 0003 | 0004,0005 | parts from the sim's vehicle-part records |
| R3-BUDGET-REC1 | 0003 | 0001,0002,0004,SPIKE_PLAN | names the client reference machines every client mark uses |
| R3-BUDGET-REC2 | 0003 | 0002,0004,0005 | frame budgets; per-unit asset budgets derive from it (0005) |
| R3-BUDGET-REC3 | 0003 | 0005 | quality tiers are data |
| R3-MEASURE-REC1 | 0003 | - | - |
| R3-MEASURE-REC2 | 0003 | 0002,0004 | benchmark scene replays a recorded match (R1-DET) |
| R3-MEASURE-REC3 | 0003 | 0002 | CI gates only hardware-independent numbers |
| R3-MEASURE-REC4 | 0003 | - | - |
| R3-AUDIO-REC1 | 0003 | - | - |
| R3-AUDIO-REC2 | 0003 | - | - |
| R3-AUDIO-REC3 | 0003 | 0004 | XT rank 35 |
| R3-AUDIO-REC4 | 0003 | 0002 | event section of the output block |
| R3-WGPU-OQ1 | 0003 | - | Brave check on Mustafa's machine during the look proof test |
| R3-WGPU-OQ2 | 0003 | - | - |
| R3-WGPU-OQ3 | 0003 | - | - |
| R3-WGPU-OQ4 | 0003 | - | LOOK_V2 decides vibrant colour, not the output transform |
| R3-WGPU-OQ5 | 0003 | - | - |
| R3-GL2-OQ1 | 0003 | - | - |
| R3-GL2-OQ2 | 0003 | 0006 | client telemetry after launch |
| R3-GL2-OQ3 | 0003 | - | - |
| R3-GL2-OQ4 | 0003 | - | - |
| R3-GL2-OQ5 | 0003 | - | Mustafa, with OQ2's share |
| R3-LOOK-OQ1 | 0003 | - | - |
| R3-LOOK-OQ2 | 0003 | 0005 | LOOK_V2 §6.2: thick outer, thin inner; the unit (world or pixel) stays open |
| R3-LOOK-OQ3 | 0003 | - | - |
| R3-LOOK-OQ4 | 0003 | - | - |
| R3-LOOK-OQ5 | 0003 | - | - |
| R3-LOOK-OQ6 | 0003 | - | - |
| R3-CHAR-OQ1 | 0003 | - | - |
| R3-CHAR-OQ2 | 0003 | - | - |
| R3-CHAR-OQ3 | 0005 | 0003 | the character model is authored content (LOOK_V2 §3, §8) |
| R3-CHAR-OQ4 | 0003 | - | - |
| R3-CHAR-OQ5 | DESIGN | 0001,0003 | camera framing (same as R2-INT-OQ2) |
| R3-CHAR-OQ6 | 0003 | - | - |
| R3-BUDGET-OQ1 | 0003 | - | Mustafa and the Advisor pick the models |
| R3-BUDGET-OQ2 | 0003 | - | - |
| R3-BUDGET-OQ3 | 0003 | - | - |
| R3-BUDGET-OQ4 | 0003 | - | answered: MODES-3 (a), N = 48 |
| R3-BUDGET-OQ5 | 0003 | - | - |
| R3-BUDGET-OQ6 | 0003 | - | - |
| R3-MEASURE-OQ1 | 0003 | - | - |
| R3-MEASURE-OQ2 | 0006 | 0002,0003 | cost and security of self-hosted runners (XT rank 33) |
| R3-MEASURE-OQ3 | 0003 | - | - |
| R3-MEASURE-OQ4 | 0002 | 0003 | the isolation flag (R1-BND-REC5) |
| R3-AUDIO-OQ1 | 0003 | - | - |
| R3-AUDIO-OQ2 | 0003 | - | - |
| R3-AUDIO-OQ3 | 0003 | - | - |
| R3-AUDIO-OQ4 | 0003 | - | - |
| R3-AUDIO-OQ5 | 0003 | 0004 | XT rank 35 |
| R3-AUDIO-OQ6 | 0003 | - | - |
| R4-PHYS-REC1 | 0004 | 0002 | supersedes R1's Rapier items (XT rank 3); crate list in 0002 |
| R4-PHYS-REC2 | 0004 | 0005 | static world from the map document |
| R4-PHYS-REC3 | 0004 | - | - |
| R4-PHYS-REC4 | 0004 | 0001,0003 | - |
| R4-PHYS-REC5 | 0004 | - | - |
| R4-NAV-REC1 | 0004 | 0002,0005 | bake tool and map document (XT rank 24) |
| R4-NAV-REC2 | 0004 | - | - |
| R4-NAV-REC3 | 0004 | - | - |
| R4-NAV-REC4 | 0004 | 0002 | Worker build without navigation when no bots |
| R4-FLOORS-REC1 | 0004 | 0005 | editor links |
| R4-FLOORS-REC2 | 0004 | 0001,0003,0005 | floors and ceilings feed enclosures (0001), roof fade (0003), map data (0005) |
| R4-FLOORS-REC3 | 0004 | 0005 | movement data is content |
| R4-LOS-REC1 | 0004 | 0001,0003 | the shared pass serves interest (0001); audio occlusion O-B (0003) |
| R4-LOS-REC2 | 0004 | - | - |
| R4-LOS-REC3 | 0004 | 0001,0002,0003 | XT rank 2 |
| R4-OBST-REC1 | 0004 | - | - |
| R4-OBST-REC2 | 0004 | 0001,0003,0005 | baked states carry LOOK_V2 §4 damage stages only in part (two states) |
| R4-OBST-REC3 | 0004 | 0005 | - |
| R4-PHYS-OQ1 | DESIGN | 0001,0004 | who blocks whom (XT rank 30) |
| R4-PHYS-OQ2 | 0004 | - | - |
| R4-PHYS-OQ3 | 0004 | - | - |
| R4-PHYS-OQ4 | 0004 | - | - |
| R4-PHYS-OQ5 | 0004 | - | Mustafa, only if a §6 item asks for dynamics |
| R4-NAV-OQ1 | 0004 | - | - |
| R4-NAV-OQ2 | 0004 | - | - |
| R4-NAV-OQ3 | 0004 | - | - |
| R4-NAV-OQ4 | 0004 | - | - |
| R4-NAV-OQ5 | DESIGN | 0001,0004 | XT rank 30 |
| R4-FLOORS-OQ1 | DESIGN | 0004 | game design: stances |
| R4-FLOORS-OQ2 | DESIGN | 0004 | game design: vehicle-driving bots, Juggernaut on ramps |
| R4-FLOORS-OQ3 | DESIGN | 0004,0005 | game design: fall damage |
| R4-FLOORS-OQ4 | 0005 | 0004 | what a reload may change (R1-DATA-OQ7) |
| R4-LOS-OQ1 | DESIGN | 0001,0004 | Mustafa with the game-designer |
| R4-LOS-OQ2 | DESIGN | 0001,0004 | same as R2-INT-OQ1 |
| R4-LOS-OQ3 | DESIGN | 0004 | bot field of view |
| R4-LOS-OQ4 | DESIGN | 0004 | fractional blockers are design; yes/no by data flag |
| R4-LOS-OQ5 | 0004 | - | - |
| R4-OBST-OQ1 | DESIGN | 0001,0004 | XT rank 30 |
| R4-OBST-OQ2 | DESIGN | 0004 | harvester lanes, LOOP_V2 §3 |
| R4-OBST-OQ3 | DESIGN | 0004 | only if Mustafa adds building to VISION_V2 §6 |
| R4-OBST-OQ4 | 0004 | - | - |
| R5-LAYOUT-REC1 | 0002 | - | - |
| R5-LAYOUT-REC2 | 0002 | 0001,0004,0005 | crate and package list, dependency rules |
| R5-LAYOUT-REC3 | 0002 | 0005 | - |
| R5-LAYOUT-REC4 | 0002 | 0004 | - |
| R5-LAYOUT-REC5 | 0002 | - | - |
| R5-LAYOUT-REC6 | 0002 | - | - |
| R5-TSLINT-REC1 | 0002 | - | - |
| R5-TSLINT-REC2 | 0002 | - | - |
| R5-TSLINT-REC3 | 0002 | - | - |
| R5-TSLINT-REC4 | 0002 | - | - |
| R5-RUSTLINT-REC1 | 0002 | - | - |
| R5-RUSTLINT-REC2 | 0002 | 0004 | the hash-map ban; determinism rules are 0004 |
| R5-RUSTLINT-REC3 | 0002 | 0001,0005 | decoders and the content loader |
| R5-RUSTLINT-REC4 | 0002 | 0004 | - |
| R5-RUSTLINT-REC5 | 0002 | - | - |
| R5-RUSTLINT-REC6 | 0002 | - | - |
| R5-TEST-REC1 | 0002 | - | - |
| R5-TEST-REC2 | 0002 | 0004 | tools for R1-DET-REC6 |
| R5-TEST-REC3 | 0002 | 0004,0005 | - |
| R5-TEST-REC4 | 0002 | 0001 | profiles are 0001's (XT rank 32) |
| R5-TEST-REC5 | 0002 | 0001,SPIKE_PLAN | the load layer the netcode proof test uses |
| R5-TEST-REC6 | 0002 | - | - |
| R5-CI-REC1 | 0002 | - | - |
| R5-CI-REC2 | 0002 | - | - |
| R5-CI-REC3 | 0002 | - | - |
| R5-CI-REC4 | 0002 | - | - |
| R5-CI-REC5 | 0002 | - | - |
| R5-PREV-REC1 | 0002 | 0006 | the preview build and deploy job (0002); the host and its token scope (0006) |
| R5-PREV-REC2 | 0006 | 0002 | the client host (R6-HOST-REC1) |
| R5-PREV-REC3 | 0006 | 0002 | cleanup policy and PREV-DELETE; the close and sweep jobs are workflows (0002 cites) |
| R5-PREV-REC4 | 0002 | 0006 | the build generates the preview header set; the CSP waits on the security baseline |
| R5-PREV-REC5 | 0006 | 0002 | answered: PREVIEW-WALL (a) |
| R5-GATE-REC1 | 0002 | - | - |
| R5-GATE-REC2 | 0002 | - | - |
| R5-GATE-REC3 | 0002 | - | - |
| R5-GATE-REC4 | 0002 | 0001,0003,0004 | every budget lives as data in the repository |
| R5-LAYOUT-OQ1 | 0002 | - | - |
| R5-LAYOUT-OQ2 | 0002 | - | - |
| R5-LAYOUT-OQ3 | 0002 | - | - |
| R5-LAYOUT-OQ4 | 0002 | - | - |
| R5-LAYOUT-OQ5 | 0002 | - | - |
| R5-TSLINT-OQ1 | 0002 | - | - |
| R5-TSLINT-OQ2 | 0002 | - | - |
| R5-TSLINT-OQ3 | 0002 | - | - |
| R5-TSLINT-OQ4 | 0002 | - | - |
| R5-TSLINT-OQ5 | 0006 | 0002 | security baseline (security-auditor) |
| R5-TSLINT-OQ6 | 0002 | - | - |
| R5-RUSTLINT-OQ1 | 0002 | 0004 | - |
| R5-RUSTLINT-OQ2 | 0002 | - | - |
| R5-RUSTLINT-OQ3 | 0002 | 0004 | - |
| R5-RUSTLINT-OQ4 | 0002 | 0004 | - |
| R5-RUSTLINT-OQ5 | 0002 | 0004 | settled by resolution 29 (GLOSSARY §7) |
| R5-TEST-OQ1 | 0002 | - | - |
| R5-TEST-OQ2 | 0002 | - | - |
| R5-TEST-OQ3 | 0002 | - | - |
| R5-TEST-OQ4 | 0002 | 0006 | macOS runner cost |
| R5-TEST-OQ5 | 0002 | - | - |
| R5-TEST-OQ6 | 0002 | - | - |
| R5-TEST-OQ7 | 0002 | - | - |
| R5-TEST-OQ8 | 0002 | 0004 | bench tool (XT rank 36) |
| R5-CI-OQ1 | 0002 | - | - |
| R5-CI-OQ2 | 0002 | - | - |
| R5-CI-OQ3 | 0002 | - | - |
| R5-CI-OQ4 | 0002 | - | - |
| R5-CI-OQ5 | 0002 | 0003,0006 | - |
| R5-CI-OQ6 | 0002 | - | - |
| R5-CI-OQ7 | 0002 | 0004,0006 | - |
| R5-PREV-OQ1 | 0006 | - | answered: PREVIEW-WALL (a) |
| R5-PREV-OQ2 | 0006 | SPIKE_PLAN | PREV-DELETE |
| R5-PREV-OQ3 | 0006 | 0001,0002 | staging game server |
| R5-PREV-OQ4 | 0006 | - | answered by R6-HOST-REC1 |
| R5-PREV-OQ5 | 0002 | - | - |
| R5-PREV-OQ6 | 0002 | 0006 | REC4's header check on a real preview |
| R5-GATE-OQ1 | 0002 | - | - |
| R5-GATE-OQ2 | 0002 | 0003 | - |
| R5-GATE-OQ3 | 0002 | - | - |
| R5-GATE-OQ4 | 0006 | 0002 | security baseline; interim: scheduled and on lockfile changes |
| R5-GATE-OQ5 | 0002 | - | - |
| R5-XQ-OQ1 | 0002 | - | - |
| R5-XQ-OQ2 | 0002 | - | - |
| R5-XQ-OQ3 | 0002 | 0006 | settled 2026-10-30: the isolation flag |
| R6-HOST-REC1 | 0006 | 0002 | the client host; 25 MiB per-asset limit |
| R6-HOST-REC2 | 0006 | 0001,0002,0004 | names the reference server every server mark uses |
| R6-HOST-REC3 | 0006 | 0001,0002 | staging game server; PREVIEW-WALL (a) |
| R6-HOST-REC4 | 0006 | 0001 | certificate per hostname (carry-forward 4) |
| R6-COST-REC1 | 0006 | 0001 | rooms per VM measured by R2-ROOM-REC2 mark 2 |
| R6-COST-REC2 | 0006 | 0001 | - |
| R6-OBS-REC1 | 0006 | 0001 | - |
| R6-OBS-REC2 | 0006 | 0001,0004 | metrics R1-TICK-REC4 and R2-BUD-REC6 ask for |
| R6-OBS-REC3 | 0006 | 0002,0004 | Sentry upload job (XT rank 34); Wasm trap |
| R6-OBS-REC4 | 0006 | - | - |
| R6-TEL-REC1 | 0006 | 0001,0004,0005 | the record shape; the sim emits events and moments (0004), the room writes at Ended (0001), ids and content hash (0005) |
| R6-TEL-REC2 | 0006 | - | - |
| R6-TEL-REC3 | 0006 | 0004 | - |
| R6-HOST-OQ1 | 0006 | - | - |
| R6-HOST-OQ2 | 0006 | - | - |
| R6-HOST-OQ3 | 0006 | - | - |
| R6-HOST-OQ4 | 0006 | - | - |
| R6-HOST-OQ5 | 0006 | - | answered: PREVIEW-WALL (a) |
| R6-HOST-OQ6 | 0006 | - | only if the security baseline weighs the wildcard |
| R6-HOST-OQ7 | 0006 | - | - |
| R6-COST-OQ1 | 0006 | 0001 | - |
| R6-COST-OQ2 | 0006 | - | - |
| R6-COST-OQ3 | 0006 | - | - |
| R6-COST-OQ4 | 0006 | - | - |
| R6-COST-OQ5 | 0006 | - | - |
| R6-OBS-OQ1 | 0006 | - | - |
| R6-OBS-OQ2 | 0006 | - | - |
| R6-OBS-OQ3 | 0006 | - | - |
| R6-OBS-OQ4 | 0006 | - | - |
| R6-OBS-OQ5 | 0003 | 0006 | R3 owns how client performance is measured (XT rank 26) |
| R6-TEL-OQ1 | 0006 | - | test-engineer and game-designer |
| R6-TEL-OQ2 | DESIGN | 0006 | Mustafa: whether bot and private matches count in rates |
| R6-TEL-OQ3 | DESIGN | 0004,0005,0006 | Mustafa and the game-designer: moment rules and thresholds |
| R6-TEL-OQ4 | 0006 | - | Mustafa after the data-protection read |
| R6-TEL-OQ5 | 0006 | - | - |
| R6-SEC-OQ1 | 0006 | 0001,0002,0005 | BASELINE-WHEN (b): open with interim positions |

**Cross-topic questions (the critic's XT items), one row per critic rank, every copy listed:**

| critic rank | ids (all copies) | owner | must cite | question |
|---|---|---|---|---|
| 2 | R1-XT-OQ1, R2-XT-OQ1, R4-XT-OQ1 | 0001 | 0002,0003,0004 | cursor ray for the aim point and how it crosses the boundary |
| 3 | R1-XT-OQ2, R4-XT-OQ2, R5-XT-OQ1 | 0004 | 0002 | R1 items that still assume Rapier |
| 4 | R1-XT-OQ3, R2-XT-OQ2, R3-XT-OQ1, R5-XT-OQ2 | 0001 | 0002,0003 | where the remote-entity interpolation buffer lives |
| 7 | R1-XT-OQ4, R2-XT-OQ3 | 0001 | 0004 | E3 projectile encoding vs cross-target identity |
| 8 | R2-XT-OQ4, R5-XT-OQ3, R6-XT-OQ1 | 0006 | 0001,0002 | who hosts, costs and builds the lobby and the room supervisor |
| 12 | R1-XT-OQ5, R2-XT-OQ5 | 0001 | 0004 | R1 timing questions (sub-tick stamps, tick nudging, shipped rates) |
| 13 | R1-XT-OQ6, R2-XT-OQ6 | 0001 | 0004,0005,0006 | R1 protocol questions (hash comparison, snapshot layout, mid-match tuning, content hash at join) |
| 14 | R1-XT-OQ7, R2-XT-OQ7, R4-XT-OQ3 | 0001 | 0004 | is whole-world replay still a spike mark |
| 15 | R1-XT-OQ8, R4-XT-OQ4, R5-XT-OQ4, R6-XT-OQ2 | DESIGN | 0002,0004,0006 | offline-play need behind browser navigation and server-less previews (Mustafa and the Advisor) |
| 16 | R1-XT-OQ9, R2-XT-OQ8, R3-XT-OQ2 | 0001 | 0002,0003 | remote soldiers' animation state in snapshot and block |
| 18 | R2-XT-OQ9, R5-XT-OQ5, R6-XT-OQ3 | 0006 | 0001,0002 | recommendations waiting on the security baseline (BASELINE-WHEN (b)) |
| 20 | R2-XT-OQ10, R5-XT-OQ6, R6-XT-OQ4 | 0002 | 0001,0006 | multiplayer previews' server binary and certificate hash |
| 24 | R4-XT-OQ5, R5-XT-OQ7 | 0002 | 0004 | navigation bake and R4 checks in the crates and the gate |
| 25 | R1-XT-OQ10, R3-XT-OQ3, R5-XT-OQ8 | 0002 | 0005 | editor by double-click with a Wasm validator (Mustafa if EDITOR_V2 §5 must change) |
| 26 | R3-XT-OQ4, R6-XT-OQ5 | 0003 | 0006 | who creates the client's field data |
| 27 | R1-XT-OQ11, R2-XT-OQ11 | 0001 | 0004 | wire entity ids and tick width vs generational ids and a 20-minute match |
| 28 | R1-XT-OQ12, R2-XT-OQ12, R6-XT-OQ6 | 0004 | 0001,0002 | recovery from a trap in the client's Wasm sim |
| 30 | R2-XT-OQ13, R4-XT-OQ6 | DESIGN | 0001,0004 | do soldiers block each other (game-designer with Mustafa) |
| 31 | R1-XT-OQ13, R5-XT-OQ9 | 0005 | 0002 | where content's TypeScript types come from |
| 32 | R2-XT-OQ14, R5-XT-OQ10 | 0001 | 0002 | every R2 mark names an R5 network profile |
| 33 | R3-XT-OQ5, R5-XT-OQ11, R6-XT-OQ7 | 0006 | 0002,0003 | CI runner costs (GPU, macOS, reference machines, cache); Mustafa for any paid plan |
| 34 | R5-XT-OQ12, R6-XT-OQ8 | 0002 | 0006 | Sentry upload job and the smoke test's network rule |
| 35 | R1-XT-OQ14, R3-XT-OQ6, R4-XT-OQ7 | 0003 | 0004 | audio occlusion rays, only if O-B |
| 36 | R1-XT-OQ15, R5-XT-OQ13 | 0002 | 0003,0004 | bench method for R1's budget marks |
| 37 | R1-XT-OQ16, R3-XT-OQ7 | 0003 | 0002 | OffscreenCanvas, or a per-entity or per-field block layout |
| 38 | R1-XT-OQ17, R6-XT-OQ9 | 0006 | 0002 | R1-BND-OQ5 embeds and login flow under isolation (only if production isolation) |
| 39 | R2-XT-OQ15, R3-XT-OQ8 | 0003 | 0001 | does the client's cutaway (roof fade) follow the server's visibility rule |
| 40 | R1-XT-OQ18, R4-XT-OQ8, R5-XT-OQ14 | 0002 | 0004 | R5 cache question sizes a workspace no topic proposes |

**Shared questions (one row per decision code, every copy listed):**

| code | copies (file:line) | owner | must cite | state |
|---|---|---|---|---|
| ABL-PREDICT | r1-simulation-core.md:2967, r2-netcode.md:3277 | 0001 | 0004,0005,SPIKE_PLAN | open: Mustafa after a Feel Lab test (Molotov, Shield); 0001 lists it (prediction scope), 0005 carries both answers |
| SEC-SPREAD | r1-simulation-core.md:2979, r2-netcode.md:3289 | 0001 | 0003,0004 | answered (a): spread on an authority-only stream; tracers use a cosmetic spread the hit corrects |
| MODES-3 | r1-simulation-core.md:2987, r2-netcode.md:3297, r3-rendering.md:1411, r4-physics-navigation.md:1153, r5-toolchain-quality.md:1918, r6-infrastructure.md:949 | 0001 | 0002,0003,0004,0005,0006,SPIKE_PLAN | answered (a): totals; largest mode 24v24, N = 48 |
| BASELINE-WHEN | r1-simulation-core.md:2997, r2-netcode.md:3307, r5-toolchain-quality.md:1927, r6-infrastructure.md:958 | 0006 | 0001,0002,0005 | answered (b): 0006 lists the security baseline open with interim positions; researched before the first online playtest |
| PREV-DELETE | r5-toolchain-quality.md:1939, r6-infrastructure.md:970 | 0006 | 0002,SPIKE_PLAN | a spike: does a closed PR's preview disappear on Cloudflare Pages; may send the host back to Workers |
| PREVIEW-WALL | r5-toolchain-quality.md:1946, r6-infrastructure.md:977 | 0006 | 0002 | answered (a): previews that reach the staging game server are behind the access wall from the first one |

## c. Shared numbers

Every number below is used by more than one draft. The owner states it; every other draft cites the owner (and the
research line) and never restates it differently. Values are the research's (cited), arithmetic on them (shown), or a
settled input. N = 48 everywhere (MODES-3 (a), brief §1; carry-forward 3: where a research mark gives 48 and 54, 48
holds). The 30 Hz column is the baseline (TICK-30); the 60 Hz column is kept only where the 60 Hz margin run needs it
(carry-forward 2). "p99" unless named otherwise ([GLOSSARY: time statistics], GLOSSARY.md:141).

### c1. Tick and server budgets
| # | Number | 30 Hz (baseline) | 60 Hz (the 60 Hz margin run only) | Source | Owner | GLOSSARY term |
|---|---|---|---|---|---|---|
| N1 | Tick period | 33.3 ms | 16.7 ms | arithmetic, 1 / rate (GLOSSARY.md:82-96) | 0001 (the rate), 0004 (the constant) | tick period |
| N2 | Tick rate | 30 Hz | only if the netcode proof test shows a 60 Hz margin of at least 25 % (N3); otherwise 30 Hz | TICK-30 (brief §1), replacing R1-TICK-REC2's rule (r1:2029; the rule at r1:2061-2062), r2:2415-2416 and GLOSSARY:85 | 0001 | the 60 Hz margin (GLOSSARY §8) |
| N3 | The 60 Hz margin | n/a | 1 − (p99 server tick time at a 60 Hz tick ÷ 8.33 ms) ≥ 25 %, so p99 ≤ 6.25 ms; setup: one vCPU of the reference server, the stress scene at N = 48 with a snapshot-receiving client on every soldier, R2-BUD-REC6's 30-minute soak, re-flood ticks included; plus every 60 Hz mark the research sets, as written (R2-BUD-REC6's two shares, R1-TICK-REC2 P1–P3); measured in one run, never summed from parts | the advisor's ruling TICK-60-HEADROOM (advisor-verdicts.md §1, condition 2), confirmed by Mustafa and the Advisor at the stop; carry-forward 2 | 0001 | the 60 Hz margin (GLOSSARY §8) |
| N4 | Server tick budget (total) | 16.7 ms (50 %), max under one tick period, zero overruns | 8.3 ms | R2-BUD-REC6 (r2:2398-2419); R1-TICK-REC2 P1 (r1:2044-2053); GLOSSARY.md:167-188 | 0001 | the server tick budget |
| N5 | Sim step share | 8.3 ms (25 %) | 4.2 ms | R1-TICK-REC2 P1 (r1:2044-2053); GLOSSARY.md:189-213 | 0004 | the sim step share |
| N6 | Server-only share | 8.3 ms (25 %) | 4.2 ms | R2-SYNC-REC9, R2-BUD-REC6 (r2:2398-2419); GLOSSARY.md:189-213 | 0001 | the server-only share |
| N7 | Parts of the sim step share (planning check, sum of ceilings) | movement and physics ≤ 1 ms mean (R4-PHYS-REC1 mark 2, r4:700-703); avoidance ≤ 0.2 ms (R4-NAV-REC3, r4:882); abilities, effects, weapons ≤ 1 ms (R1-ABL-REC6, r1:2465-2470); navigation 5 % = 1.67 ms (R4-NAV-REC2, r4:861); visibility pass 5 % = 1.67 ms (R2-INT-REC3, r2:2480); sum 5.53 of 8.33 ms | the same parts: 0.83 ms each for the two 5 % rows; sum 3.87 of 4.17 ms | GLOSSARY.md:46-81 (the table, lines 55-69) | 0004 (all parts; the visibility pass's id is 0001's) | the sim step share |
| N8 | Parts of the server-only share | encoding 48 × 50 µs = 2.40 ms (R2-BUD-REC6, r2:2414); lag compensation 700 shots/s × 20 µs = 0.47 ms (R2-SYNC-REC5 mark 4, r2:2186-2187); interest 0.2 + 0.5 = 0.70 ms (R2-INT-REC1 (c), r2:2440; R2-INT-REC4, r2:2500); enclosure pair test 0.1 µs × 768 pairs = 0.08 ms (R2-INT-REC2 (b), r2:2463-2464); sum 3.64 of 8.33 ms | lag compensation 0.23 ms (shots per tick halve); sum 3.41 of 4.17 ms | GLOSSARY.md:55-69 | 0001 | the server-only share |
| N9 | Re-flood after a portal change (that tick only) | +1.00 ms: server-only 4.64 ms, server tick 10.18 of 16.67 ms | server-only 4.41 of 4.17 ms (0.24 over), server tick 8.28 of 8.33 ms (0.05 left) | R2-INT-REC2 (b) (r2:2463-2464); GLOSSARY.md:66-69; carry-forward 2 | 0001 | the server-only share |
| N10 | Server tick, sum of ceilings | 9.18 of 16.67 ms | 7.28 of 8.33 ms (snapshot encoding every second tick: GLOSSARY.md:71-73) | GLOSSARY.md:68 | 0001 | server tick time |
| N11 | Client's sim share | 13.3 ms (40 %) in Wasm in the Worker, minimum laptop | 6.7 ms | R1-TICK-REC2 P2 (r1:2054-2057); GLOSSARY.md:214-224 | 0004 | the client's sim share |
| N12 | Page's cost of the sim, per frame | ≤ 1.0 ms: p99 in R3-BUDGET-REC2's row (r3:1127); p95 frame-time difference in R1-BND-REC1 mark 1 (r1:2258-2260) | same | GLOSSARY.md:214-224 | 0003 (the frame budget row); 0002 cites it for the boundary spike | the client's sim share (the page's cost of the sim) |
| N13 | Prediction replay | ≤ 1 ms for 200 ms of game time: 6 ticks | 12 ticks | R1-TICK-REC2 P3 (r1:2058-2060) | 0001 | the prediction scope |
| N14 | Whole-world restore and replay (if kept) | k = ceil(0.15 s × 30) = 5 ticks within 8 ms, Wasm, minimum laptop | k = 9 | R1-ECS-REC1 mark 3 (r1:1938-1941); critic rank 14 may drop it | 0004 (keeps or drops it once 0001 sets the scope) | the prediction scope |
| N15 | State snapshot (clone) | ≤ 0.2 ms native, ≤ 0.5 ms Wasm | same | R1-ECS-REC1 mark 2 (r1:1932-1937) | 0004 | state hash (fidelity) |
| N16 | Server loop: catch-up bound and start jitter | ≤ 0.25 s of game time per wake (7.5 ticks: the draft states a whole number, proposed); start jitter ≤ 2 ms | 0.25 s = 15 ticks (the research's form) | R1-TICK-REC4 (r1:2082-2093) | 0004 | (none; operations metric) |
| N17 | Worker tick start jitter | ≤ 4 ms foreground over 10 minutes; hidden 60 s then back in step within 0.5 s online, never more than 15 ticks per frame (60 Hz form; restated at 30 Hz) | same | R1-TICK-REC5 (r1:2110-2116) | 0004 | (none) |
| N18 | Run length of tick marks | 36,000 ticks = 20 minutes of game time | 10 minutes | R1-TICK-REC1, REC2 P1 (r1:2044-2053) (arithmetic) | 0004 | (none) |
| N19 | Fire-rate fidelity | Shabzon within 1 % of 13.9 shots/s; every legacy interval within 1 shot over 60 s | same | R1-ABL-REC3 (r1:2402-2420), R1-TICK-REC1 (r1:2009-2028), R2-BUD-REC1 (r2:2324) | 0004 (one time representation, R1-XU-OQ7) | (none) |
| N20 | Measuring unit | one vCPU of the reference server, no other room on it | same | GLOSSARY.md:244-264 | 0006 (names the machine) | the measuring unit: one vCPU |

### c2. Network
| # | Number | Value | Source | Owner | GLOSSARY term |
|---|---|---|---|---|---|
| N21 | Snapshot rate | 30 Hz (under TICK-30 every tick is a snapshot tick) | R2-BUD-REC1 (r2:2302-2325) | 0001 | (none) |
| N22 | Downstream per client, N = 48, all in view, 30 Hz | mean ≤ 160 kbps, p99 of 1-second windows ≤ 256 kbps (UDP payload + 28 B per datagram); with interest on the Octopon-3 greybox mean ≤ 100 kbps; E3 estimate about 146 kbps (E3b about +42 kbps: R2-BUD-OQ8) | R2-BUD-REC2 (r2:2326-2344) | 0001 | (none) |
| N23 | Upstream per client | ≤ 48 kbps (IPv4, newest input + 3 previous); command size open (R2-XU-OQ4) | R2-BUD-REC1 (r2:2324) | 0001 | (none) |
| N24 | Snapshot datagram cap | 1,000 B (R2-TRN-REC2, r2:2618) or min(runtime maximum, 1,150 B) (R2-BUD-REC3, r2:2345): one cap open (R2-XU-OQ5) | as cited | 0001 | (none) |
| N25 | Interpolation delay | 100 ms default (3 snapshot intervals at 30 Hz), adapting to 150 ms (R2-BUD-REC4, r2:2361-2364) or 2–5 intervals (R2-SYNC-REC4): one bound open (R2-XU-OQ3) | as cited | 0001 | (none) |
| N26 | Rewind cap and history | cap 200 ms (R2-BUD-REC4, r2:2363) or 250 ms (R2-SYNC-REC5): open (R2-XU-OQ1, R2-SYNC-OQ2); 1 s of hitbox history (R2-BUD-REC4), history ≤ 1 MB (R2-SYNC-REC5, R2-XU-OQ7) | as cited | 0001 | (none) |
| N27 | Network profiles | P0–P4 and B1; P2 = 100 ms round trip, ±20 ms jitter, 2 % loss (the budgets marks use ±10 ms: R2-XU-OQ12) | R2-SYNC unit; R2-XU-OQ12 (r2:3220-3225) | 0001 (the simulator that runs them is 0002's) | (none) |
| N28 | Join and resync | complete world within 300 ms at 100 ms RTT, 2 % loss (R2-BUD-REC3), or 1 RTT + 100 ms (R2-ROOM-REC3), or deltas again within 500 ms (R2-SYNC-REC1): one mark open (R2-XU-OQ6) | as cited | 0001 | (none) |
| N29 | Transport runtime | ≤ 1 vCPU per process with 48 sessions; send call ≤ 1 ms; the load's 60 input datagrams/s are the 60 Hz form (30/s at the baseline) | R2-TRN-REC1 mark 2 (r2:2589-2598) | 0001 | transport runtime time |
| N30 | Dev and preview certificates | self-signed, pinned, valid at most 14 days | R2-TRN-REC5 (r2:1879; rec r2:2699-2723) | 0006 | game-server certificate |

### c3. Rooms, servers and costs
| # | Number | Value | Source | Owner | GLOSSARY term |
|---|---|---|---|---|---|
| N31 | Memory per room | RSS reported at start and after 20 minutes; proposed ceiling 64 MB at N = 48 | R2-ROOM-REC2 mark 3 (r2:2778-2779) | 0001 | (none) |
| N32 | Rooms per VM | test load: 8 rooms of 48 as 8 processes on one CCX23, each p99 within 10 % of one room (R2-ROOM-REC2 mark 2, r2:2777); cost assumption A-2 "8 rooms of 48 players per 4-vCPU VM" until measured (r6:455, r6:516; R6-COST-OQ1) | as cited | 0006 (the figure); 0001 (the test) | the measuring unit: one vCPU |
| N33 | Room start-up | under 2 s, process spawn to Open | R2-ROOM-REC2 mark 5 (r2:2780) | 0001 | (none) |
| N34 | The reference server | Hetzner CCX23: 4 dedicated vCPU, 16 GB, x86-64 AMD, one EU location; shared CX33 (4 vCPU, 8 GB) measured beside it | R6-HOST-REC2 (r6:645-650) | 0006 | the reference server |
| N35 | VM prices | CCX23 86.49 EUR/month, CX33 8.99 EUR/month (9.6 times cheaper); 20 TB egress included per VM, 1 EUR per extra TB | R6-HOST-REC2 (r6:656-659), R6-HOST-F15, F16 | 0006 | (none) |
| N36 | Costs | start: about 18 EUR/month (lean) to about 102 EUR + 5 USD (reference); goal: about 240–360 EUR + about 20 USD/month plus Sentry's paid plan; egress about 3.16 GB per 24v24 room-hour; the spike's own bill under 5 EUR | R6-COST-REC1 (r6:703-717); r6:446 | 0006 | the start modes and the goal modes |
| N37 | Client host limit | largest build file under 25 MiB (Pages per-asset limit) | R6-HOST-REC1 mark (r6:642); R6-HOST-F3 | 0006 (0002 cites for the Wasm and bundle sizes) | the client host |
| N38 | Retention and records | logs 14 days, metrics 15 days (proposed); a 24v24 match record ≤ 1 MB uncompressed; recording adds < 1 % to server tick p99 | R6-OBS-REC4 (r6:769-770); R6-TEL-REC1 mark (r6:806-811) | 0006 | (none) |
| N39 | Preview timing | link within 5 min after the build job (median of 5); gone 24 h after the PR closes; sweep leaves 0 closed-PR previews older than 7 days; artifacts kept 7 days | R5-PREV-REC1 mark 1 (r5:1600); R5-PREV-REC3 (r5:1616-1624) | 0002 (link, artifacts); 0006 (deletion) | preview |
| N40 | CI wall time | ≤ 10 min p50 over 5 runs with warm caches, ≤ 20 min cold (proposed) | R5-CI-REC5 mark 2 (r5:1580) | 0002 | (none) |

### c4. Client budgets (the minimum laptop at Low, 30+ fps; the recommended desktop at High, 60 fps)
| # | Number | Minimum, Low | Recommended, High | Source | Owner | GLOSSARY term |
|---|---|---|---|---|---|---|
| N41 | Reference machines | one named laptop, Intel Gen12 (Iris Xe) class or AMD equivalent, ≥ 4 cores and 8 threads, 1920 × 1080, Windows 11, plus a Linux run for the real fallback | one named desktop with a mid-range discrete card, at 1080p and 1440p; Mustafa's RTX 5080 is a headroom and Brave check only | R3-BUDGET-REC1 (r3:1096-1103); models open (R3-BUDGET-OQ1) | 0003 | the reference server (its client counterpart) |
| N42 | Frame time | p95 ≤ 33.3 ms, median ≤ 28 ms | p95 ≤ 16.7 ms, median ≤ 14 ms | R3-BUDGET-REC2 (r3:1123-1124) | 0003 | (none) |
| N43 | Main-thread CPU per frame | ≤ 16 ms; animation ≤ 1.0 ms; sim seen by the page ≤ 1.0 ms | ≤ 8 ms; animation ≤ 2.0 ms; sim ≤ 1.0 ms | r3:1125-1127 | 0003 | the client's sim share |
| N44 | GPU time | total ≤ 25 ms (shadow ≤ 3, main ≤ 16, post ≤ 4, effects ≤ 2) | total ≤ 12 ms (2, 6, 3, 1) | r3:1128-1132 | 0003 | (none) |
| N45 | Draw calls | ≤ 800 without a per-object outline pass at Low, about ≤ 1,100 with one | ≤ 1,500 | r3:1133; arithmetic r3:1145-1158 | 0003 | (none) |
| N46 | Triangles, GPU memory, heap | ≤ 500k; ≤ 512 MB; JS heap ≤ 150 MB after 10 minutes, no growth over the last 5 | ≤ 1.5M; ≤ 1 GB; same heap | r3:1134-1137 | 0003 (0005 derives per-unit budgets from these) | (none) |
| N47 | Per soldier and per very large unit | ≤ 4 draw calls per soldier per pass (before outlines); 4 very large units add ≤ 2.0 ms GPU and ≤ 200 draws (recommended, High) | same | R3-CHAR-REC1 (r3:1044-1049); R3-CHAR-REC4 (r3:1088-1093) | 0003 (0005 cites for per-unit budgets) | big vehicle and very large unit |
| N48 | Audio voices | 32 concurrent voices (proposed start) | same | R3-AUDIO-REC2 (r3:1245) | 0003 | (none) |
| N49 | Block latency and boundary garbage | tick T's block readable on the page within one display frame (p95); boundary-caused heap growth < 0.1 MB/s; B3 adopted only if it saves ≥ 0.5 ms of page time per frame or one frame of latency | same | R1-BND-REC2 marks 2, 3, 5 (r1:2276-2284) | 0002 | (none) |
| N50 | Wasm size | no research number: recorded with and without parry3d or Rapier after wasm-opt and gzip; the budget is set from the measurement (R1-ECS-OQ1, R1-RR-OQ5); hard ceiling from the host: N37 | R1-ECS-REC1 mark 5 (r1:1946-1950); R4-PHYS-REC1 mark 4 | 0002 | (none) |

### c5. Scale and scenes
| # | Number | Value | Source | Owner | GLOSSARY term |
|---|---|---|---|---|---|
| N51 | N, the largest mode | 48 (24v24); design point = largest mode's N; headroom run N = 64 | brief §1 (MODES-3 (a)); GLOSSARY.md:267-374 | settled input (cited by all; 0001 owns the code MODES-3) | player count (N); the largest mode, and its N; the design point and headroom |
| N52 | The stress scene | N = 48 bots with full AI in three teams (16v16v16) in one combat cluster; 15 big vehicles (harvesters included), 4 of them very large units of ≥ 20 parts; 300 live projectiles; up to 8 active effects per soldier (384); one DM_Tri-sized 3D greybox; for server marks a snapshot-receiving client on every soldier; scale steps N = 12, 24, 36, 48 and 64 (headroom) | GLOSSARY.md:397-437 (resolution 9); counts replaced by the game-designer's caps when they exist (R2-BUD-OQ5, DESIGN) | SPIKE_PLAN (a shared spike fixture; no draft restates its counts) | the stress scene |
| N53 | The test-mode list | 24v24, 12v12v12, 16v16v16, 1v1, 4v4, 2v2v2, 6v6v6, and N = 64 labelled headroom | GLOSSARY.md:375-394 (reading (a)) | SPIKE_PLAN | the test-mode list |
| N54 | The netcode proof test's human client | "100 ms of simulated latency" (brief §2); which R2 profile that is (P2's 100 ms round trip, or one way) is for 0001 to state (part i) | brief §2; VISION_V2 §4 ("about 100 ms of ping") | 0001 | (GLOSSARY §8 once added) |

## d. Interfaces between the drafts

What crosses each seam, who defines it ("owner"), and who uses it. A draft that uses an interface cites its owner.

| # | Interface | What crosses | Owner (defines) | Used by |
|---|---|---|---|---|
| I-1 | Content → sim | typed definitions loaded and validated by the content crate, read by id; tuning converted to ticks at load (R1-TICK-REC1) | 0005 (schema, ids); 0004 (how the sim executes them) | 0004, 0001 (unit data such as top speed, R2-XU-OQ14) |
| I-2 | Sim → page | the per-tick output block: header (tick, layout hash, counts), sections per entity kind (soldiers, vehicle parts, projectiles, structures), the event section (hits, deaths, purchases, captures, moments) (R1-BND-REC2) | 0002 (layout, versioning, pool, transfer); 0001 (the fields of remote entities online, ranks 4 and 16); 0004 (writes it) | 0003 (draws, animates, plays sounds from it) |
| I-3 | Page → Worker, and the local server | one command per tick, posted (R1-BND-REC3); the small control API (wasm-bindgen `.d.ts`: start, mode flag, content bundle); the local server reached through the same messages | 0002 (posting, control API, skeleton); 0001 (command fields; whether the local path is a backend of the thin transport interface, R2-TRN-REC4) | 0003 (input mapping on the page) |
| I-4 | Worker ↔ server | the message protocol: commands, snapshots and deltas, events with visibility classes, join and resync, control stream, lobby messages, the content hash at join, dev reload and private-match override messages; WebTransport datagrams and streams, WebSocket fallback | 0001 | 0002 (Worker owns the connection), 0005 (reload, overrides, hash), 0006 (certificates, firewall) |
| I-5 | Generated types | Rust → `packages/types`: the block layout and its hash (own generator), protocol types (ts-rs), content schema and content types (rank 31) | 0002 (package, gate diff); 0005 (content types' source); 0001 (protocol types) | 0003, the editor |
| I-6 | Content bundle | hashed, cached bundle; immutable cache header on the client host; the hash every match records | 0005 (bundle, hash); 0006 (serving it) | 0001 (hash at join), 0006 (match record) |
| I-7 | Modes → rooms | a room = mode id + allowed overrides (R2-ROOM-REC7); the mode file format and validation; the allow list of private-match tunables (DESIGN: R1-DATA-OQ10, R2-ROOM-OQ5) | 0001 (room); 0005 (file) | 0006 (record's mode id) |
| I-8 | Assets → renderer | glTF/GLB with named material slots (faction colour slots, emissive slots), LOD levels, per-unit budgets; no baked look | 0005 (rules; graphics-engineer's GPU rules) | 0003 (loader, materials in TSL, LOD switching, per-instance colour) |
| I-9 | Map document → sim, bake, renderer, editor | static colliders with stable ids (R4-PHYS-REC2); baked navigation stored in the map document (R4-NAV-REC1); floors, ceilings, enclosure areas and portals (R4-FLOORS-REC2, R2-INT-REC2); structure states (LOOK_V2 §4 damage stages); per-map lighting mood (LOOK_V2 §6.5) | 0005 (document schema); 0004 (bake tool, states) | 0001 (enclosures for interest), 0003 (draws stages, roof fade, mood), the editor (0002) |
| I-10 | Events → match record → storage | tick-stamped events and recognized moments; the room's record written at Ended; supervisor upload to EU object storage | 0004 (events, moments); 0001 (the room writes it); 0006 (shape, storage, analysis) | 0005 (ids, content hash) |
| I-11 | Room process ↔ supervisor ↔ lobby | process spawn, host and port from the lobby, join tokens checked by the room offline (R2-ROOM-REC2, REC3) | 0001 (protocol, token format); 0006 (supervisor, lobby hosting, key storage under the security baseline) | 0002 (crate list, rank 8) |
| I-12 | Server → monitoring | server tick time histogram, overruns and dropped time (R1-TICK-REC4), per-phase time (R2-BUD-REC6), sessions by transport, bytes, memory, room starts and crashes (R6-OBS-REC2) | 0006 (metrics, labels, logs); 0004 and 0001 (the counters they emit) | |
| I-13 | CI → client host and staging | the built artifact and its header sets, deployed with the host token from environment `preview`; for multiplayer previews the room binary and the certificate hash (rank 20) | 0002 (workflow); 0006 (host, token scope, staging server, access wall) | 0001 (where the certificate hash is fetched) |
| I-14 | Budgets as data | every budget of every draft as data in the repository, reviewed like code (R5-GATE-REC4) | 0002 (where and how checked); each number's owner (part c) fills its row | all |
| I-15 | Reference hardware | the reference server (N34) and the client reference machines (N41) | 0006; 0003 | every spike |
| I-16 | Test fixtures | the stress scene, the test-mode list (N52, N53), the network profiles (N27), recorded matches | SPIKE_PLAN (scenes, modes); 0001 (profiles); 0004 (recordings) | 0001, 0002, 0003, 0004 |
| I-17 | Determinism suite → CI and benchmark | recorded matches with per-tick state hashes; the benchmark scene replays a recorded match (R3-MEASURE-REC2) | 0004 (rules, suite); 0002 (tools, jobs, R5-TEST-REC2) | 0003 |
| I-18 | Client field data | backend, adapter, frame time per match; crash reports | 0003 (what is measured, rank 26); 0006 (where stored, R6-OBS-REC3, REC4) | |
| I-19 | The aim point | cursor ray against what the page draws → a world aim point in the command (rank 2; R2-SYNC-REC8, R4-LOS-REC3) | 0001 (command field); 0002 (how it crosses); 0004 (the ray query) | 0003 (interpolated state) |
| I-20 | Events → feel | hits, deaths, blasts (position and strength for the concussion shake and the fragments), cosmetic shot events, purchases, moments → VFX, audio, HUD; the shot feels instant, hit feedback on confirmation (LOOP_V2 §7) | 0004 (emits); 0001 (delivery and visibility online); 0002 (event section) | 0003 (presents), 0005 (feel kit data) |
| I-21 | Upgrade picks | the player's picks on the player's slot (UPGRADE-SCOPE), defined as data; sent to their owner; shown on the HUD | 0005 (data model); 0004 (state on the slot); 0001 (visibility class) | 0003 |

## e. Traces

### e1. VISION_V2 §6
| § | Need | Draft and part that serve it |
|---|---|---|
| 6.1 | Modes as data: 2 or 3 teams of any size, one faction per team, rooms and private matches | 0001 a.8 (rooms, private rooms R2-ROOM-REC5, a room as mode id + overrides REC7, bots fill slots REC6), a.5 (the split path at 48+, R2-BUD-REC3); 0005 a.1 (mode files, factions as data); 0006 a.2 (hosting rooms, the lobby); 0004 a.7 (team filtering in the sim) |
| 6.2 | A true 3D sim: floors and interiors, ledges, line of sight across heights; bots between levels; aiming at height | 0004 a.5–a.6 (R4-PHYS, R4-FLOORS, R4-LOS, R4-NAV); 0001 a.4 and a.6 (aim point in the command, enclosure interest); 0002 a.3 (the aim point across the boundary, rank 2); 0003 a.7 (cursor ray against what is drawn; roof fade) |
| 6.3 | A data-driven ability and effect system | 0005 a.2 (GAS subset as data, in-match upgrades); 0004 a.3 and a.7 (execution, its 1 ms part, determinism); 0001 a.1 (prediction scope, ABL-PREDICT) |
| 6.4 | Vehicles as first-class: big, multi-part, own weapons and abilities (leech, marking); seats [OPEN] | 0004 a.5 (kinematic compound bodies, R4-PHYS-REC4; obstacles R4-OBST-REC1); 0001 a.1, a.6 (R2-SYNC-REC7, parts in interest R2-INT-REC6); 0003 a.3 (R3-CHAR-REC4); 0005 a.2 (vehicle definitions, mounts, leech and marking as operations, R1-ABL-REC1 mark 3); seats: DESIGN (R2-SYNC-OQ7) |
| 6.5 | Economy and objectives: money, tech, steals, saving up, team-funded actions; server-side and tunable | 0004 a.7 (sim phases); 0005 a.1 (tuning tables, LOOP_V2 "one data table"); 0001 a.6 (visibility classes; team-private state is DESIGN, R2-INT-OQ3) |
| 6.6 | Balance telemetry | 0004 a.7 (events); 0001 a.8 (the room writes the record at Ended); 0006 a.5 (shape, storage, analysis); 0005 a.1 (ids, content hash) |
| 6.7 | A faction content pipeline: units, uniforms, variants as data | 0005 a.1 (faction variants, R1-DATA-OQ6) and a.3 (faction colour slots); 0003 a.3 (per-instance faction colour, R3-CHAR-REC1, REC3) |
| 6.8 | Moments recognized by the server; [LATER] kill-cams, replays, highlights | 0004 a.7 (recognition); 0005 a.4 (rules as data); 0006 a.5 (stored with the record); [LATER] parts not designed out: recorded matches and per-tick hashes (R1-DET-REC6, R1-TICK-REC1) |
| 6.9 | A spectacle budget: a few very large units and their effects without dropping frames | 0003 a.3–a.4 (R3-CHAR-REC4, R3-BUDGET-REC2's very large units, R3-WGPU-REC3 no first-sight stutter); 0002 a.3 (the sim off the render thread, R1-BND-REC1); 0004 a.3 (4 very large units in the stress scene) |
| 6.10 | [LATER] Accounts: server-side ownership of unlocks and cosmetics | not built ([LATER]); 0001 a.8 (guest identity, R2-ROOM-REC8, prepares it); 0006 a.2 (the lobby's store, R2-ROOM-OQ9). LOOP_V2 §8's Service points storage is a research gap (part h) |

Recommendations that trace only to VISION_V2 §3 or §4 (R2-BUD-REC4, REC5, R2-TRN-REC6, R3-WGPU-REC2, REC4,
R3-GL2-REC3, R3-BUDGET-REC1, R3-MEASURE-REC3; R2-XU-OQ18): whether that is trace enough under plan rule 4 is the
Advisor's (part f, LE-G; the session's reading is recorded in part j, J3). Spatial audio traces to LOOP_V2 §7 (sound in the feel kit) and LOOK_V2 §7 (punchy weapons
through VFX, sound and feedback), which rule 4 accepts.

### e2. LOOK_V2 (every `[DECIDED]` and `[DEFAULT]` item; the `[OPEN]` item stays open)
| LOOK_V2 | Tag | Item | Achieved by (draft, part) | Research behind it |
|---|---|---|---|---|
| §1 | DECIDED | cel-shaded comic rendering with bold ink | 0003 a.2 | R3-LOOK-F1, F2 (toon ramps), R3-LOOK-REC3 (outline families) |
| §1 | DECIDED | iconic, RA-style unit silhouettes | 0005 a.3 (authored models) with 0003 a.2 (outlines, LOD keep the silhouette) | none for the models (part h) |
| §1 | DECIDED | clean, well-lit scenes with emissive accents | 0003 a.2 (lighting, emissive and bloom on the MRT pass) | R3-LOOK-REC2 |
| §1 | DECIDED | lush environments, for adults | 0005 a.3 (assets) with 0003 a.3–a.4 (props instanced, budgets) | R3-CHAR-OQ4 (props); vegetation not researched (part h) |
| §2 | DECIDED | vibrant colours, soft shadows, geometry that reads | 0003 a.2 (output transform, shadows) | R3-WGPU-OQ4 (colour output), R3-BUDGET-OQ6 (shadow technique not researched; r186 removed `PCFSoftShadowMap`, R3-WGPU-F3) |
| §2 | DECIDED | thick black outlines, strongest on buildings | 0003 a.2 | R3-LOOK-REC3, R3-LOOK-OQ2 (width model) |
| §2 | DECIDED | props: logs and trees with variety | 0005 a.3 (assets) with 0003 a.3 (instancing) | R3-CHAR-OQ4; the tree style is `[OPEN]` |
| §2 | DECIDED | uniform details as faction identity (conscript shoulders, stripe, trench coat) | 0005 a.1 and a.3 (models, faction variants) with 0003 a.3 (per-instance colour) | R1-DATA-OQ6, R3-CHAR-REC1 |
| §2 | DECIDED | small movement touches (body reacting to strafing) | 0003 a.3 (procedural animation on the page, budget LOD) | R3-CHAR-REC2; legacy GA rows (port to TypeScript) |
| §2 | DECIDED | architecture: half-timbered house with brick-tile roof, plastered house with silo tower | 0005 a.3 (building assets) with 0003 a.2 (ink strongest on buildings) | none (part h) |
| §2 | DECIDED | grenade concussion: shake tilting away from the blast, stronger closer in | 0003 a.7 (camera effect from the blast event, presentation RNG) with 0004 a.7 (blast event with position and strength) | R1-DET-REC4 (presentation randomness off the sim); technique not researched (part h) |
| §3 | DECIDED | adult audience, not a children's game | 0005 a.3 (assets) and 0003 a.2 (look); no engine feature | none needed |
| §3 | DECIDED | slimmer, serious character proportions | 0005 a.3 (authored character models, R3-CHAR-OQ3) with 0003 a.3 (skinning with any weights, R3-CHAR-REC1) | R3-CHAR-REC1 |
| §3 | DECIDED | weapons with real silhouettes | 0005 a.3 (weapon models; LOOK_V2 §6.3 weapons and headgear first) | none (part h) |
| §3 | DECIDED | the thatched roof goes in its current form | 0005 a.3 (building kit) | legacy CM rows |
| §3 | DECIDED | buildings sized for play: interiors, upper floors, verticality | 0004 a.6 (floor layers with ceilings, R4-FLOORS-REC2); 0001 a.6 (enclosures, R2-INT-REC2); 0005 a.1 (map and building data); 0003 a.2 (roof fade, AO in interiors) | R4-FLOORS, R2-INT-REC2, R3-LOOK-OQ3 |
| §3 | DECIDED | fragmentation comes back (fragments flying) | 0003 a.2 and a.7 (visual effects from the blast event); whether fragments deal damage per fragment is DESIGN | no visual-effects research (part h) |
| §4 | DECIDED | lush countryside plus contested urban centres: ruined enterable buildings, collapsed roofs, planks to upper floors, corners to hold | 0004 a.6 (typed links for planks, R4-FLOORS-REC1; structure states, R4-OBST-REC2); 0005 a.1 (map and building content); 0003 a.2 (ruins drawn); 0001 a.6 (portals) | R4-FLOORS-REC1, R4-OBST-REC2 |
| §4 | DEFAULT | damage stages: intact → damaged → ruin with the interior exposed; debris and planks become routes | 0004 a.6 (states flip baked flags; the research bakes two states, not three); 0005 a.1, a.3 (stages per building, assets per stage); 0003 a.2 (mesh per stage, debris); 0001 a.6 (re-flood on a portal change) | R4-OBST-REC2 (two states and a ruin's shape) only (part h) |
| §4 | DEFAULT | roofs fade when the player is inside | 0003 a.2 (cutaway), with 0001's visibility rule (rank 39) and 0004's floor layers (which enclosure the local player is in) | legacy GW17; R2-INT-REC2; rendering technique not researched (part h) |
| §5 | DECIDED | iconic silhouettes drawable from memory | 0005 a.3 with 0003 a.2 | none for the models (part h) |
| §5 | DECIDED | PA's principle: readable from a distance, depth up close | 0005 a.3 (LOD levels) with 0003 a.3 (LOD switching; the budget LOD) | R3-CHAR-REC2 (animation LOD); mesh LOD generation not researched (part h) |
| §6.1 | DEFAULT | value hierarchy: environment one step calmer than units, faction colours and effects | 0003 a.2 (per material class tone; tiers); 0005 a.3 (material classes) | none (art direction on R3-LOOK's techniques) |
| §6.2 | DEFAULT | line weight: thick on silhouettes, thinner inside | 0003 a.2 | R3-LOOK-OQ2; inner lines not researched (part h) |
| §6.3 | DEFAULT | weapons and headgear first | 0005 a.3 (per-unit budgets favour them) with 0003 | none |
| §6.4 | DEFAULT | glow language: objectives, faction tech, muzzle flashes emissive | 0003 a.2 (emissive and bloom); 0005 a.3 (emissive slots) | R3-LOOK-REC2 |
| §6.5 | DEFAULT | lighting: warm sun, cool shadows; soft AO in corners; moods per map | 0003 a.2 (lighting, AO); 0005 a.1 (mood as map data) | R3-LOOK-OQ3 (AO without the pre-pass); lighting model not researched (part h) |
| §7 | DECIDED | punchy weapons through VFX, sound and feedback (LOOP_V2 §7 feel kit) | 0003 a.2, a.6, a.7 (VFX, audio, feedback on confirmed hits); 0005 a.1 (feel kit as data); 0001 a.1 (split feedback) | R3-AUDIO; VFX not researched (part h) |
| §7 | DECIDED | grenades: fragmentation first, with the concussion screen effect | 0003 a.2, a.7 | as §2 and §3 above |
| §7 | DEFAULT | comic impact touches, sparingly: halftone flashes, ink line boil | 0003 a.2 (as defaults) | none (part h) |
| §8 | DECIDED | v2 may use authored 3D models | 0005 a.3; 0003 a.2 (the engine applies the look) | none for the pipeline (part h) |
| §8 | DEFAULT | the flow: sketch → Spline model → glTF/GLB export → automated compress, LODs, faction-slot renaming → the engine applies the comic look | 0005 a.3 (steps 3–4); 0003 a.2 (step 5) | none (part h) |
| §8 | DEFAULT | budgets per unit (triangles, materials, textures); never Spline's runtime | 0005 a.3 (graphics-engineer), derived from 0003's totals (N45–N47) | R3-BUDGET-REC2, R3-CHAR-REC1 (totals and per-soldier draws only) |
| §8 | OPEN | the tree style | stays open: 0003 lists it as an open question for Mustafa; 0005 notes that tree assets wait for it | |
| §9 | (requirement) | the rendering draft shows how each `[DECIDED]` item is achieved; its look proof test is judged by Mustafa in the browser, side by side with legacy screenshots | 0003 a.8; SPIKE_PLAN | |

## f. Loose-ends register

Each item: where it is resolved, or which draft lists it as an open question, and how. `docs/adr/README.md` carries
the final register (plan step 6); "OQ" means the owner draft's numbered open question (plan rule 7).

### f1. STATUS's five carry-forwards
| Code | Item | Resolved in / listed by | How |
|---|---|---|---|
| CF-1 | Verify every ADR claim and number, R3–R6 included (research plan condition 9; the advisor's #28 review, finding 4) | plan steps 7 and 9 (every draft's claims ledger) | each draft keeps a claims ledger; the verifier checks every claim and number; unreadable sources make the claim "unverified" and the decision an OQ or a spike mark |
| CF-2 | R2-BUD-REC6's reason (r2:2402-2403) holds at N = 54, not at N = 48: at 60 Hz the re-flood tick fits by 0.05 ms (8.28 of 8.33) while its server-only share is 0.24 ms over (4.41 of 4.17); restate for 30 Hz and say what the 60 Hz margin is measured against | 0001 (owner of R2-BUD-REC6) | states the corrected reason in its own text, citing r2:2402-2403 as the superseded line; restates the budget at 30 Hz (N4–N10: 9.18 of 16.67 ms, 10.18 on a re-flood tick); states the 60 Hz margin as the advisor ruled it (N3; TICK-60-HEADROOM, confirmed at the stop) and adds "(at N = 54, MODES-3 (b))" where it restates r2:2402-2403; 0004 cites for the sim step share |
| CF-3 | Use 48 where a mark gives 48 and 54; where a sentence says "until … answers", the answer applies | every draft; checked in the consistency pass (plan step 8) | part c uses 48 throughout; each draft's Appendix A notes the answered code (MODES-3 (a), PREVIEW-WALL (a), SEC-SPREAD (a), BASELINE-WHEN (b)) where a research sentence still waits |
| CF-4 | R6-HOST-REC4 recommends a certificate per hostname; four sentences still call the wildcard R6's proposal (STATUS cites them at b6effde as r2:2703-2705, r6:47, 938-940, 1056; at 9e7db28 they are r2:2704-2706, r6:47, r6:941-943, r6:1065-1066) | 0006 (owner of R6-HOST-REC4, R2-TRN-REC5, R2-TRN-OQ4) | states the per-hostname position (BASELINE-WHEN (b)), lists the four lines as superseded with the corrected text; the wildcard stays an option the security baseline weighs; 0001 cites |
| CF-5 | No doc defines "the netcode proof test" or "the foundation"; the drafts' plan names the research spike the proof test is | brief §2 (the Advisor's definitions); GLOSSARY §8 (plan GLOSSARY-8); SPIKE_PLAN; 0001 (owns the proof test); 0002 (the foundation is its migration plan) | GLOSSARY §8 quotes brief §2; part g names the research spikes the proof test subsumes |

### f2. The critic's remaining items and the answered escalations
| Code | State | Resolved in / listed by | How |
|---|---|---|---|
| ABL-PREDICT | open (Mustafa, after a Feel Lab test on the Molotov and the Shield) | 0001 OQ (the prediction scope); 0005 cites | 0001 lists it with both readings (GLOSSARY: the prediction scope); 0005's ability system carries both (own ability state replayable, or presses wait for confirmation); the Feel Lab test is a later test in SPIKE_PLAN, not a pre-acceptance spike (LE-N) |
| PREV-DELETE | a spike item | 0006; SPIKE_PLAN (the preview hosting check) | the check deletes a branch's latest deployment on a test project; a failure may send the client host back to Workers static assets (GLOSSARY: the client host) |
| MODES-3 (a) | answered: totals; largest mode 24v24, N = 48 | 0001 (code owner); every draft applies it | N = 48 in every mark; the test-mode list reads (a) |
| PREVIEW-WALL (a) | answered: previews that reach the staging game server are private from the first one | 0006; 0002 cites | 0006 sets the access wall on the host for multiplayer previews; offline previews stay public but unlisted, behind the wall if they carry unreleased art (GLOSSARY: the access wall) |
| SEC-SPREAD (a) | answered: server-only spread | 0001; 0004 and 0003 cite | spread drawn from an authority-only stream (0004, R1-DET-REC4); tracers drawn from a cosmetic spread that the hit event corrects (0003); no client-held hash covers it (GLOSSARY: state hash) |
| BASELINE-WHEN (b) | answered: listed open in ADR 0006 with interim positions; researched before the first online playtest, in its own fresh session | 0006 | interim positions: R5-GATE-REC1's scheduled advisories check (also on every `Cargo.lock` change); a certificate per hostname; the items waiting on it are R6-SEC-OQ1's list and critic rank 18 (R2-ROOM-OQ2 key storage, R2-ROOM-OQ11, R5-GATE-OQ4, R5-TSLINT-OQ5, R1-RR-OQ7, R1-DATA-REC7's allow list, COOP as an XS-Leaks guard) |
| XT (28 ranks, 72 ids) | open cross-topic questions | owners in part b (the XT table) | each owner answers it or lists it as an OQ; the citing drafts reference that answer; "ADR-0004 (boundary)" reads 0002 |
| XU (30 ids: R1-XU 12, R2-XU 18) | settled ones (R1-XU-OQ1, OQ5, OQ6; R2-XU-OQ8, OQ9, OQ10, OQ13) and open ones | owners in part b | settled: the owner records "answered (where)"; open: the owner picks one value and one mark, or lists an OQ with the spike that decides it |
| PR #28 finding 2 | R2-BUD-REC6's reason (r2:2402-2403) | 0001 | the same item as CF-2 |

### f3. Loose ends this map found (for the register)
| Code | Item | Resolved in / listed by | How |
|---|---|---|---|
| LE-A | The critic's XT texts put the boundary in ADR 0004; the plan puts it in 0002 | this map | ranks 2, 4, 16, 37 read "0002" for the boundary part |
| LE-B | One budget, two statistics: the page's cost of the sim is p99 ≤ 1.0 ms in R3-BUDGET-REC2 (r3:1127) and a p95 frame-time difference ≤ 1.0 ms in R1-BND-REC1 mark 1 (r1:2258-2260) | 0003 owns the number (N12); 0002 cites | test-engineer's pass marks state both statistics or pick one, with the reason |
| LE-C | R1-TICK-REC4's and REC5's "15 ticks" are the 60 Hz form of 0.25 s | 0004 | restated at 30 Hz as game time (0.25 s), the tick count rounded and marked proposed |
| LE-D | "36,000 ticks per run" is 10 minutes at 60 Hz and 20 minutes at 30 Hz (R1-TICK-REC1, REC2 P1) | 0004 | each mark says game time and ticks |
| LE-E | R2-TRN-REC1 mark 2's load (60 input datagrams/s, 1,000 B snapshots) are placeholders at the 60 Hz form | 0001 | restated at 30 Hz with the command size R2-XU-OQ4 picks |
| LE-F | Brief §2's "100 ms of simulated latency" does not say round trip or one way; R2's P2 is 100 ms round trip, ±20 ms, 2 % loss | 0001 (N54) | names the profile the human client runs; VISION_V2 §4 says "ping" (part i) |
| LE-G | Trace rule: recommendations serving only VISION_V2 §3 or §4 (R2-XU-OQ18 and R3's "no §6 item" lines) against plan rule 4 (§6, LOOP_V2, EDITOR_V2, LOOK_V2) | 0001 lists R2-XU-OQ18; every draft cites | the Advisor decides whether a §3 or §4 target is trace enough (part i) |
| LE-H | R7-MARKS (a): 118 inventory rows are "look brief pending"; LOOK_V2 now exists | 0003 (look pipeline, characters, presentation rows); 0005 (asset and building-kit rows) | each lists the rows whose mark it would change; `docs/LEGACY_INVENTORY.md` changes only after Mustafa accepts the drafts |
| LE-I | SLICE-ORDER's slice 2 ("played locally") and condition 1 (client + local server) partly answer the offline-play questions (R1-BND-OQ1, R1-TICK-OQ7, rank 15) | DESIGN; 0002 and 0004 cite | the drafts build local play with bots from slice 2; whether offline play ships, and at the largest mode on the minimum laptop, stays Mustafa's |
| LE-J | Plan rule 8's script counts owner dispositions in the six drafts only; 38 ids are DESIGN | the session (part i) | proposal: `docs/adr/README.md` lists them under "design questions for Mustafa" and the script counts that list as their owner; citing drafts dispose them as "design question, cited" |
| LE-K | The netcode proof test runs before the foundation, so its sim is a throwaway stand-in; the server tick budget is "tested in the stress scene" (GLOSSARY.md:167-188) | 0001 with test-engineer (part i) | answered (part j, J6 (2)): 0001 proposes how the stress scene's sim load reaches the 60 Hz margin run and lists it for the Advisor (TICK-60-SETUP); the margin is measured in one run, never summed |
| LE-L | The plan's authorship table gives 0001 "R6-HOST (servers)"; this brief's seam puts machines and certificates in 0006 | this map | the map follows the seam: every R6-HOST id, R2-TRN-REC5 and R2-TRN-OQ4 are 0006's; 0001 cites them |
| LE-M | R2-INT-REC3's visibility pass is an 0001 id, but its 5 % is a part of the sim step share | this map | the id is 0001's; the number is 0004's (N7) |
| LE-N | ABL-PREDICT's Feel Lab test needs abilities in the engine, so it cannot be a phase 0 spike | SPIKE_PLAN | listed as a later test with its trigger (abilities playable, slice 4), alongside the other Feel Lab weighings (R2-SYNC-OQ2's rewind cap feel, R2-SYNC-OQ6's projectile prestep) |
| LE-O | The stress scene's counts are proposals until the game-designer sets caps (R2-BUD-OQ5) | SPIKE_PLAN (N52); DESIGN for the caps | the spike plan holds the counts once; drafts cite GLOSSARY and the spike plan |
| LE-P | R1-TICK-REC2's decision rule ("60 Hz if P1–P3 pass at 60 Hz") is replaced by TICK-30 | 0001 | records R1-TICK-REC2 as "changed (TICK-30)", keeping P1–P3 as marks at 30 Hz and at 60 Hz for the 60 Hz margin run |

## g. The spike candidates

Every "what a spike must prove" in the research's recommendations, grouped into a proposed small set. Working names
only (SPIKE_PLAN defines the names, the pass marks come from test-engineer in plan step 4, the sizes from the
synthesis). Spikes are throwaway, in `spikes/`, exempt from the legacy constraints (SPIKE-SCOPE (a)), and none starts
before Mustafa and the Advisor approve the drafts (brief §4). "Subsumes" lists the recommendations whose marks the
spike carries, then the open questions it decides.

| Spike (working name) | Proves | Subsumes | Owner | Depends on | Place in the order |
|---|---|---|---|---|---|
| **The netcode proof test** (brief §2) | the Rust sim native as a server and as Wasm in the client; WebTransport with the WebSocket fallback; 48 simulated clients at a 30 Hz tick within 0001's server tick time, bandwidth per client and memory per room; one human client at 100 ms (N54) sees prediction, reconciliation and lag-compensated hits that look correct; the 60 Hz margin run (N2, N3) | R2-SYNC-REC1, REC2, REC3, REC4, REC5, REC8 (if aiming at height is in the load), REC9; R2-BUD-REC1–REC6; R2-TRN-REC1–REC4, REC6, and REC5's development certificates (self-signed, pinned; the production form is later, with R6-HOST-REC4); R1-ABL-REC4 (own weapons only; own abilities wait for ABL-PREDICT); R2-ROOM-REC2 (marks 1, 3, 5, 6; marks 2 and 4 for rooms per VM); R2-INT-REC1, REC4 (only if the bandwidth mark uses the interest filter); R2-SYNC-REC6 (only if the load has live projectiles: LE-K); R1-TICK-REC2 P1 at 30 and 60 Hz, P3; R1-TICK-REC4; R1-BND-REC6; R5-TEST-REC4 (L1, L2), REC5 (bots as protocol clients); R6-HOST-REC2 (CCX23 and CX33); R6-COST-REC1 (rooms per VM, egress per match-hour, the bill); R6-OBS-REC1 (logging cost), REC2 (metrics). Decides: R2-SYNC-OQ2 (the numbers), OQ3; R2-BUD-OQ3, OQ9, OQ10; R2-TRN-OQ1, OQ3, OQ5, OQ8; R2-ROOM-OQ1; R2-XU-OQ2, OQ5, OQ6, OQ7; R1-TICK-OQ5; R6-COST-OQ1; carry-forward 2 | 0001 | the drafts approved; the reference server orderable (R6-HOST-OQ1); a throwaway sim with movement, hitscan and bots as protocol clients, native and Wasm (shared with the boundary spike's client and the sim core spike); the four browsers (Safari needs a macOS machine); the network profiles named (N27) | first: before the foundation (brief §1, SLICE-ORDER condition 3) |
| **The Rust ↔ TypeScript boundary spike** | one module Worker; the output block (B2, with B3 measured); commands posted; generated types with the layout hash; the isolation flag; four browsers; Worker pacing, smoothness on 60, 120 and 144 Hz displays, hidden tabs; the page's frame time with the sim in the Worker at N = 12, 24, 36, 48; the client's sim share on the minimum laptop | R1-BND-REC1–REC5; R1-TICK-REC5; R1-TICK-REC2 P2 (with the sim core spike's load); R5-LAYOUT-REC3, REC5; R6-OBS-REC3 (a Wasm trap reaching Sentry, if Sentry is set up). Decides: R1-BND-OQ2, OQ3 (core count recorded), OQ6; R1-XU-OQ3, OQ4; R1-TICK-OQ2; R1-DATA-OQ8; R1-RR-OQ5 (size measured), OQ6; critic ranks 4 (the implementation side) and 37 | 0002 | the minimum laptop named (0003, N41); a stand-in sim in Wasm; shares the client harness with the netcode proof test | with the netcode proof test, before the foundation (its result fixes the skeleton the foundation builds; the brief fixes only the proof test's place: part i) |
| **The sim core spike** | sim step time in the stress scene on the reference server and in Wasm; snapshot and fidelity; ids; cross-target identity with per-tick hashes; RNG streams; parry3d queries, movement parity, cost, size; navigation bake, coverage, gaps, reachability, path queue, avoidance; floors and links; sight queries; moving obstacles and structure states | R1-ECS-REC1–REC3 (REC4 only if Rapier comes back); R1-TICK-REC1, REC2 (P1 and P2 sim marks), REC3, REC6; R1-DET-REC1–REC6 (R1-DET-REC7 has no mark of its own: it stands or falls with REC1–REC6); R4-PHYS-REC1–REC5; R4-NAV-REC1–REC4; R4-FLOORS-REC1–REC3; R4-LOS-REC1–REC3; R4-OBST-REC1, REC2 (REC3 only if tiling is needed); R1-ABL-REC6 (with the content spike); R2-SYNC-REC6 mark 1 (trajectory bit-identity); R5-TEST-REC2; R5-LAYOUT-REC4. Decides: R1-ECS-OQ1, OQ4, OQ5; R1-DET-OQ1, OQ2, OQ4, OQ8, OQ10; R1-TICK-OQ9; R1-RR-OQ1; R4-PHYS-OQ2–OQ4; R4-NAV-OQ1–OQ3; R4-LOS-OQ5; R4-OBST-OQ4; critic rank 3 | 0004 | the reference server and the minimum laptop; a 3D greybox (DM_Tri-sized, N52); bot behaviour enough for "full AI" (a research gap, part h); the stress scene's counts (SPIKE_PLAN) | in parallel with the netcode proof test; its sim (or step times calibrating a stand-in) reaches the 60 Hz margin run inside one proof-test run, never summed (TICK-60-SETUP, part j, J6 (2)); before slice 1 |
| **The content and ability spike** | the legacy weapons and the three T1 abilities run from data with no ability-specific Rust code; a new combination by data only; the §6.4 primitives (an effect removed when its source dies; marking); number changes need no code; the JSON corpus, fuzzing with zero panics, the generated schema, migrations, hot reload within 1 s, the bundle hash, server-side validation, the legacy import; in-match upgrade picks survive death and reset at match start (UPGRADE-SCOPE; mark to be written by test-engineer, no research mark) | R1-ABL-REC1, REC2, REC3 (with the sim core spike), REC5, REC6; R1-DATA-REC1–REC8; R5-TEST-REC3 (goldens). Decides: R1-DATA-OQ2, OQ4, OQ5, OQ9; R1-ABL-OQ7; R1-RR-OQ2, OQ3 (zero panics); R1-XU-OQ7 | 0005 | the sim core spike's throwaway sim; `docs/legacy-tuning.json`; LOOP_V2 §5's T1 abilities | after the sim core spike; before slice 1 (weapons are data from slice 1) |
| **The look proof test** (LOOK_V2 §9) | how each LOOK_V2 `[DECIDED]` item looks, judged by Mustafa in the browser side by side with legacy screenshots; each candidate technique's GPU time and memory alone, on both reference machines and both backends; both outline families at 12, 24 and 48 soldiers; backend parity; no first-sight shader stutter; the backend reported (and Brave's backend on Mustafa's machine) | R3-LOOK-REC1–REC3; R3-WGPU-REC1, REC3, REC4; R3-GL2-REC1, REC3; R3-BUDGET-REC1 (the machines named); R3-MEASURE-REC1, REC4. Decides: R3-WGPU-OQ1–OQ3; R3-GL2-OQ1, OQ3; R3-LOOK-OQ1, OQ3, OQ4, OQ6; R3-BUDGET-OQ3; R3-MEASURE-OQ1, OQ3 | 0003 | LOOK_V2; both reference machines; Mustafa's time in Brave; a few hand-made test assets (a soldier, a building with an interior, a vehicle, props), since the asset pipeline is not researched yet | in parallel with the netcode proof test (no shared dependency); before any art production and before slice 1's renderer |
| **The render budget bench** (may merge into the look proof test) | the budget table in the benchmark scene with the techniques the look test keeps: characters at N = 48 on screen, the four very large units, the Firefox WebGL 2 worst case, quality tiers, the CI split, audio in the benchmark fight | R3-CHAR-REC1–REC4; R3-BUDGET-REC2, REC3; R3-GL2-REC2; R3-MEASURE-REC2, REC3; R3-AUDIO-REC1–REC4; R3-WGPU-REC2 (upgrade check, later). Decides: R3-CHAR-OQ1, OQ2, OQ4, OQ6; R3-BUDGET-OQ2, OQ5, OQ6; R3-AUDIO-OQ1, OQ2, OQ6 | 0003 | the look proof test's techniques; the boundary spike's block (per-instance data from it); a recorded match for the benchmark scene (the sim core spike's determinism, or a synthetic recording) | after the look proof test and the boundary spike; before slice 1's renderer work |
| **The preview hosting check** (small) | PREV-DELETE: a closed PR's preview really disappears; the header sets on 100 % of responses; `crossOriginIsolated` true only with the isolation flag; the bundle served immutable; the 25 MiB limit; the environment URL on the PR page | R6-HOST-REC1; R5-PREV-REC2, REC3 (the 24 h mark), REC4's header check. Decides: PREV-DELETE (R5-PREV-OQ2); R5-PREV-OQ5, OQ6 | 0006 | Mustafa's Cloudflare account and a test project (secrets are Mustafa's to add); a static page with a `.wasm` | before the foundation's preview step (a failure sends the host to Workers static assets) |
| **The asset spike** (later) | the asset pipeline once researched: compression, LODs, slot renaming, per-unit budgets, the loader on both backends | none in R1–R6 (part h); ASSET-RESEARCH (a) researches it first | 0005 | the asset research; the look proof test's results | before the first authored model enters the game |

Not throwaway spikes (proven elsewhere, owner in brackets):
- **The foundation's own acceptance checks** (0002): R5-LAYOUT-REC1, REC2, REC6; R5-TSLINT-REC1–REC4;
  R5-RUSTLINT-REC1–REC6; R5-TEST-REC1, REC6; R5-CI-REC1–REC5; R5-PREV-REC1, REC4; R5-GATE-REC1–REC4; R1-BND-REC4's CI
  checks. The foundation ends when an empty scene runs through the full stack with CI green and a preview link
  (brief §2), so these marks are its milestones, not a spike's.
- **Later milestones** (phase 1 slices): R2-ROOM-REC1, REC3–REC8 and R2-INT-REC2, REC3, REC5–REC7 (slice 3, online;
  REC3's visibility pass with the bots of slice 2); R2-SYNC-REC6, REC7 (projectiles and vehicles, when they enter);
  R6-HOST-REC3 and R5-PREV-REC5's access wall (the first multiplayer preview, PREVIEW-WALL (a)); R6-HOST-REC4, R6-OBS-REC3, REC4 (before the first online playtest,
  with the security baseline); R6-TEL-REC1–REC3, R6-COST-REC2 (slice 4, the loop); R1-DATA-REC5's editor reload
  (slice 5); R4-OBST-REC3 (only if tiling is needed); the Feel Lab tests (ABL-PREDICT; R2-SYNC-OQ2's feel weighing;
  R2-SYNC-OQ6) once abilities and projectiles are playable (LE-N).

The proposed order: (0) drafts approved → (1) in parallel: the netcode proof test, the boundary spike, the sim core
spike, the look proof test, the preview hosting check → (2) the foundation (phase 1: restructure, toolchain, CI gate
v2, private previews, generated types, the client + local server skeleton) → (3) the content and ability spike and the
render budget bench, before slice 1 → (4) slices 1–6 (SLICE-ORDER), the asset spike before the first authored model.
The research spike the netcode proof test is (CF-5): the union of R2-BUD-REC6's spike (the server tick budget in the
stress scene with a snapshot-receiving client on every soldier), R2-SYNC-REC9's (48 clients), R2-TRN-REC1's and
REC3's (both transports, interop and load), R2-ROOM-REC2's (a room holds its tick; memory) and R1-TICK-REC2's P1 and
P3, with R2-SYNC-REC3 and REC5's marks for the one human client.

## h. Research gaps

What a draft must decide where R1–R6 have no finding, and what plan rule 2 makes of it (a claim the research does not
support is dropped, becomes an open question, or becomes a spike mark). Counts are greps of `docs/research/r*.md` at
9e7db28.

| # | Gap | What the research has | Owner | Rule 2 makes it |
|---|---|---|---|---|
| G1 | The asset pipeline: texture and mesh compression (KTX2, Basis, meshopt, Draco: 0 hits), LOD generation, Spline export checks, renaming materials into faction colour slots, the glTF/GLB loader on both backends (R3's 12 glTF hits concern skinned instancing), per-unit budgets (triangles, materials, textures) | totals only: R3-BUDGET-REC2 (N44–N46), R3-CHAR-REC1's 4 draws per soldier per pass, R3-CHAR-REC4's very large units (N47) | 0005 (graphics-engineer for the GPU rules) | open questions, researched before the asset spike (the plan's ASSET-RESEARCH (a), pending the advisor); per-unit budgets either arithmetic on N45–N47 (shown) or marked proposed with the reason |
| G2 | Halftone flashes and ink line boil (LOOK_V2 §7 `[DEFAULT]`) | none (0 hits) | 0003 | built as defaults; the technique an OQ; its cost a proposed mark in the look proof test, measured like R3-LOOK-REC1 |
| G3 | Roof fade (LOOK_V2 §4 `[DEFAULT]`) | legacy GW17 (the cutaway), R2-INT-REC2 and critic rank 39 (it follows the server's visibility rule); R3-WGPU-REC3 names the cutaway's blend-state variant; no rendering technique | 0003 (with 0001, 0004) | OQ for the technique and its interaction with the ink pass; a look-test mark; the visibility rule stays R2-INT-OQ1 (DESIGN) |
| G4 | Damage stages: intact → damaged → ruin with the interior exposed; debris and planks as routes (LOOK_V2 §4 `[DEFAULT]`) | R4-OBST-REC2 bakes two states and a ruin's shape; R4-PHYS-REC2's alive flag; legacy ruins sink to 15 % of their height (r4:579) | 0004 (states), 0005 (stage definitions, assets per stage), 0003 (drawing) | the third state and per-stage assets are OQs; extending the two-state flip to three is a proposed mark in the sim core spike |
| G5 | Visual effects: fragments, muzzle flashes, tracers, impacts, debris; the effect pools | R3-BUDGET-REC2's effects row (≤ 2 ms Low, ≤ 1 ms High) and R3-BUDGET-REC3's effect caps; particles appear only as option F-B's example (r3:797) | 0003 | the budget row stands; the technique is an OQ (F-A applies: nothing WebGPU-only without a fallback) |
| G6 | The grenade concussion shake (LOOK_V2 §2) | legacy behaviour; presentation randomness off the sim (R1-DET-REC4) | 0003 | port the legacy behaviour as the default (the inventory's port-to-TypeScript rows); nothing to research beyond the blast event (I-20) |
| G7 | Soft shadows and the lighting model: warm sun, cool shadows, moods per map (LOOK_V2 §2, §6.5) | R3-BUDGET-OQ6: the shadow technique was not researched; r186 removed `PCFSoftShadowMap` from WebGPURenderer (R3-WGPU-F3) | 0003 | OQ; a look-test mark against the shadow-pass row (N44) |
| G8 | Inner lines thinner than the silhouette ink (LOOK_V2 §6.2) | R3-LOOK-OQ2 covers the width model only | 0003 | OQ; look-test mark |
| G9 | Vegetation and "lush" environments; mesh LOD for PA's principle (LOOK_V2 §1, §5) | R3-CHAR-OQ4 (BatchedMesh or InstancedMesh for props); only animation LOD researched (R3-CHAR-REC2); 0 hits for foliage | 0003 with 0005 | OQs (the tree style itself stays `[OPEN]`) |
| G10 | Bot behaviour: three-team fights, buying, escorting and hunting harvesters, carrying and stealing uplinks, the role mix (LOOP_V2 §10) | navigation and sight (R4), cadences (R1-TICK-REC3), legacy GN rows; 0 hits for an AI architecture | 0004 | OQ; the stress scene's "bots with full AI" needs a stated behaviour set (a proposed spike condition) |
| G11 | Moment recognition (VISION_V2 §6.8) | R6-TEL-REC1 stores moments as tagged events; thresholds are R6-TEL-OQ3 (DESIGN); nothing on the mechanism | 0004 (the phase), 0005 (the rule format) | stated at §6.8's level (a sim phase reading the event stream, as R6-TEL-REC1 asks); the rule format an OQ |
| G12 | Economy and objectives as sim phases (VISION_V2 §6.5, LOOP_V2 §3–§4) | cadences (R1-TICK-REC3), visibility classes (R2-INT-REC5), legacy GE rows | 0004 with 0005 | stated at LOOP_V2's level; numbers in 0005's tables; rules stay LOOP_V2's |
| G13 | In-match upgrade tracks (UPGRADE-SCOPE as Mustafa updated it: part j, J5) | the settled input only; 0 research findings | 0005 (game-designer; carriers per J5) | the shape is settled (tracks per unit and weapon type on the player, surviving death and switching: the old unit-switch question is answered); carry slots, ammo for looted weapons and levelling speed are OQs for Mustafa (J5) |
| G14 | Service points and account persistence (LOOP_V2 §8: the prototype records Service points; VISION_V2 §6.10 `[LATER]`) | R2-ROOM-REC8 (guest identity), R2-ROOM-OQ9 (where the lobby stores guest ids) | 0006 | OQ (no store is built before §6.10 is due) |
| G15 | The Feel Lab tool (LOOP_V2 §7: live tuning of the feel kit) | R1-DATA-OQ12 notes the Feel Lab covers client feel data; no research on the tool | 0005 (feel-kit data), 0003 (presentation) | OQ |
| G16 | The HUD and menus on the page (war-room UI) | legacy PG and GP rows (port to TypeScript); no v2 research | 0003 | stated as the page's DOM layer, ported per the inventory rows; no engine feature beyond that |
| G17 | The Wasm size budget | no number (R1-ECS-OQ1, R1-RR-OQ5) | 0002 | the boundary and sim core spikes measure it; the budget is set from the number, under N37's 25 MiB host limit |
| G18 | Aim help onto bodies (LOOP_V2 §9 `[OPEN]`, technical) | R2-SYNC-REC8: if aim help exists, the server applies it (r2:2251-2252); no technique researched | 0001 | OQ, only if aim help is adopted |
| G19 | Map size (LOOP_V2 §9 `[OPEN]`) | R4-PHYS-REC5 (f32 to 2 km), R2-BUD-OQ4 (position bits to ±327 m and 41 m of height) | 0004, 0001 | stays open; each draft states the limit its choice holds to |
| G20 | Camera framing and what a player may see (R2-INT-OQ1, OQ2; R3-CHAR-OQ5; R3-AUDIO-OQ3) | the research left them to the look brief; LOOK_V2 does not set them | DESIGN | stays open (Mustafa); the drafts keep the camera as data (R2-INT-REC1) |
| G21 | The security baseline | deferred by decision (BASELINE-WHEN (b)) | 0006 | listed open with interim positions; researched before the first online playtest |

## i. For the session: what the plan or the brief leaves unclear

All nine were answered on 2026-10-31: part j, J6.

1. **DESIGN ids and the coverage script (LE-J).** Plan rule 8 counts owner dispositions in the six drafts only; 38 ids
   are pure design questions. Proposal: `docs/adr/README.md` lists them as "design questions for Mustafa" and the
   script counts that list as their owner. The session (or the advisor) confirms.
2. **The netcode proof test's sim load (LE-K).** It runs before the foundation, so its sim is a stand-in. Is its
   server tick mark set on the full stress scene with stand-in systems, or on its own load with the sim core spike's
   sim step time added for the 60 Hz decision? This matters for the 60 Hz margin. 0001 with test-engineer
   proposes; the advisor confirms with TICK-60-HEADROOM.
3. **"100 ms of simulated latency" (LE-F, N54).** Round trip (VISION_V2 §4 says "ping") or one way; and which R2
   profile (P2 is 100 ms round trip, ±20 ms, 2 % loss). 0001 states it; the Advisor wrote the definition, so the
   report should ask.
4. **The trace rule for §3 and §4 targets (LE-G, R2-XU-OQ18).** Plan rule 4 lists §6, LOOP_V2, EDITOR_V2 and LOOK_V2;
   several adopted-looking recommendations trace only to VISION_V2 §3 or §4 (performance, browsers, fairness). The
   Advisor decides whether that is trace enough; until then each draft names the §3 or §4 target and flags it.
5. **Which spikes must precede the foundation.** The brief fixes only the netcode proof test. The map proposes the
   boundary spike, the sim core spike, the look proof test and the preview hosting check in parallel with it, and the
   content and ability spike and the render budget bench before slice 1 (part g). The spike plan proposes; the
   Advisor confirms.
6. **The plan's authorship table vs this brief's seams (LE-L).** The table lists "R6-HOST (servers)" under 0001; the
   seam puts machines and certificates in 0006. The map follows the seam; the plan's table may want the same words.
7. **The stress scene's owner (N52, LE-O).** The map makes SPIKE_PLAN its owner (a fixture every spike shares, counts
   held once). If the session prefers a draft, 0004 is the natural one (the sim builds the scene).
8. **The Feel Lab tests (LE-N).** ABL-PREDICT waits for one, but it needs abilities in the engine, so it cannot be a
   phase 0 spike. The map lists it as a later test (slice 4); the Advisor may want it earlier with a stand-in.
9. **ASSET-RESEARCH and TICK-60-HEADROOM** are still with the advisor (plan "Decisions for you"); parts c (N3), g (the
   asset spike) and h (G1) assume the plan's recommendations, (a) and "confirm at the stop".


## j. Addendum (2026-10-31: the advisor's condition 7 and Mustafa's UPGRADE-SCOPE update)

Inputs: `plan.md` revision 3; `advisor-verdicts.md` §1 (APPROVE with 19 conditions, and its rulings);
`inputs/UPGRADE-SCOPE-mustafa-2026-10-31.md` (Mustafa's words; they replace the brief's upgrade note). Part j wins
where it and parts a–i differ.

**Changes made to parts a–i** (and to the id table):
1. "Headroom" for TICK-30's 25 % became "the 60 Hz margin" (plan rule 5): part a (0001 item 2; seam S1), part b
   (R1-TICK-REC2, R1-TICK-OQ1 notes), part c (intro, the 60 Hz column heading, N2, N3), part f (CF-2, LE-K, LE-P),
   part g (the netcode proof test's "Proves"; the sim core spike's place, now "never summed"), part i (item 2).
   "Headroom" still marks the N = 64 run (N51, N53) and R3's RTX 5080 check (N41).
2. N2 and N3 follow the advisor's ruling (J1); CF-2 states the margin as ruled and adds "(at N = 54, MODES-3 (b))".
3. LE-K and the sim core spike's row point to J6 (2).
4. Part a (0005 item 2) and part h (G13): the brief's unit-switch question is answered by Mustafa's update (J5).
5. Part e's trace paragraph and part i's heading point here (J3, J6).
6. `engine-architect-decision-map-ids.tsv`: no owner changed; the notes of R1-TICK-REC2 and R1-TICK-OQ1 say "the 60 Hz
   margin". The coverage result (521 of 521) stands.

### J1. The 60 Hz margin, and GLOSSARY §8 ready to append
Shared-number row (replaces N3; N2 now reads "30 Hz; 60 Hz only with a 60 Hz margin of at least 25 %, otherwise 30 Hz"):

| # | Number | Value | Source | Owner | GLOSSARY term |
|---|---|---|---|---|---|
| N3 | The 60 Hz margin | 1 − (p99 server tick time at 60 Hz ÷ 8.33 ms) ≥ 25 %, so p99 ≤ 6.25 ms; setup: one vCPU of the reference server, the stress scene at N = 48 with a snapshot-receiving client on every soldier, R2-BUD-REC6's 30-minute soak, re-flood ticks included; plus every 60 Hz mark the research sets (R2-BUD-REC6's two shares, R1-TICK-REC2 P1–P3); otherwise 30 Hz | TICK-60-HEADROOM, the advisor's ruling (advisor-verdicts.md §1, condition 2), confirmed at the stop | 0001 | the 60 Hz margin (GLOSSARY §8) |

0001 owns it and the 30 Hz baseline marks (R2-BUD-REC6 restated from GLOSSARY §1's 30 Hz column), and states three
lines as replaced by TICK-30: R1-TICK-REC2's rule (r1:2029), "the rule does not change" (r2:2415-2416) and "which does
not change" (GLOSSARY:85).

The §8 text below goes after GLOSSARY line 742, byte for byte (the session appends it in step 6; the two brief
quotes were copied from `inputs/DRAFTS_BRIEF.md` lines 52-59 and 60-66 by script). Anchors: `#the-foundation`,
`#the-netcode-proof-test`, `#the-60-hz-margin`.

````markdown
## 8. Phase 1 terms (the Advisor, 2026-10-31)

### The foundation
- **Definition:** the Advisor's drafts brief, §2, 2026-10-31:

> - **The foundation.** Phase 1's work before slice 1's gameplay:
>   - the repository restructure, moving the old game to `legacy/`;
>   - the toolchain: a Cargo workspace plus TypeScript with Vite;
>   - CI gate v2, and private preview links;
>   - TypeScript types generated from the Rust code;
>   - the client + local server skeleton: the simulation in a Web Worker, behind the same message protocol the network uses.
>
>   It ends when an empty scene runs through the full stack in the browser, with CI green and a preview link.

- **Marked by:** "the foundation".
- **Retire (exact):** none.
- **Retire (in context):** none.
- **Used today:** none in R1–R6 (a phase 1 term); the decision drafts, `docs/ENGINE_OVERVIEW.md` and
  `docs/SPIKE_PLAN.md` use it.

### The netcode proof test
- **Definition:** the Advisor's drafts brief, §2, 2026-10-31:

> - **The netcode proof test.** A throwaway spike that proves the online path:
>   - the Rust simulation runs natively as a server, and as Wasm in the client;
>   - they connect over WebTransport, with the WebSocket fallback;
>   - 48 simulated clients run at a 30 Hz tick, with tick time, bandwidth per client and memory all within the budgets that
>     ADR 0001 sets;
>   - one human client at 100 ms of simulated latency sees prediction, reconciliation and lag-compensated hits that look
>     correct.

- **Marked by:** "the netcode proof test".
- **Retire (exact):** none.
- **Retire (in context):** none.
- **Used today:** none in R1–R6 (a phase 1 term); the decision drafts, `docs/ENGINE_OVERVIEW.md` and
  `docs/SPIKE_PLAN.md` use it.

### The 60 Hz margin
- **Definition:** 1 − (p99 server tick time at a 60 Hz tick ÷ the server tick budget at 60 Hz, 8.33 ms). It is measured
  in the netcode proof test, in one run (p99s do not add: see time statistics): on one vCPU of the reference server, in
  the stress scene at N = 48 with a snapshot-receiving client on every soldier, over R2-BUD-REC6's 30-minute soak,
  re-flood ticks included. A 60 Hz tick needs a 60 Hz margin of at least 25 % (a p99 server tick time of at most
  6.25 ms) and every 60 Hz mark the research sets, as written (R2-BUD-REC6's two shares, R1-TICK-REC2 P1–P3);
  otherwise the tick stays at 30 Hz.
- **Depends on:** TICK-60-HEADROOM (the advisor's reading of 2026-10-31: TICK-30's 25 % is measured against the
  server tick budget at 60 Hz, 8.33 ms), confirmed by Mustafa and the Advisor at the stop.
- **Marked by:** "the 60 Hz margin".
- **Retire (exact):** none.
- **Retire (in context):** "headroom" where TICK-30's 25 % is meant. TICK-30's own word "headroom" appears only in
  quotes of TICK-30; headroom keeps its meaning of a run above the largest mode's N (the design point and headroom).
- **Used today:** none in R1–R6 (a phase 1 term); ADR 0001 owns it, and ADR 0004 and `docs/SPIKE_PLAN.md` cite it.

TICK-30 (the Advisor, 2026-10-31) replaces, for the drafts, the tick-rate rule the Tick period entry calls unchanged (line 85): 30 Hz is the baseline, 60 Hz needs a 60 Hz margin of at least 25 %.
````

### J2. Loose-ends register: three more items (condition 7 (b))
| Code | Item | Disposed by | How |
|---|---|---|---|
| OLD-ADR | STATUS:198: the v2 drafts replace ADR-001..003 | `docs/adr/README.md`; docs-keeper (STATUS) | README's index records the replacement; STATUS's three entries are marked replaced |
| ADR-001 | STATUS:199 (authority, tick, transport, hosting, offline play) | 0001 (authority, tick rate, transport, rooms); 0004 (the tick mechanism); 0006 (hosting); DESIGN (offline play, S11) | its logged inputs: one hit world for every shooter, K1 → 0004 (R4-PHYS-REC2, R4-LOS-REC1); a phased `update()`, K4 → 0004 (R1-ECS-REC1); the server repeats the team allow-list (`teamOf`) → 0001 (forged commands rejected, R2-SYNC-REC2) with 0004; boot-time `Math.random` per slot, three.js UUIDs on the shared `Math.random`, the camera-dependent FX cull → 0004 (R1-DET-REC4's streams; presentation randomness on the page, 0003); `addTech`'s HUD writes as an event → 0004 (events, I-2, I-20); humans take over bot slots → 0001 (R2-ROOM-REC6); the LOS budget sized for 12 → 0004 (R4-LOS-REC2 at 48); a full server-side schema check, `version:1` never read → 0005 (R1-DATA-REC4, REC7) |
| ADR-002 | STATUS:200-201 (modules, bundler, TypeScript, double-click build; "evidence first via the build spike") | 0002 | the stack is settled (Vite, TypeScript, Cargo workspace: GLOSSARY §8, the foundation); the build spike's role passes to the boundary spike and the foundation; the double-click player build ends with the legacy engine, recorded under template 0000's "Hard constraints in CLAUDE.md that change" (condition 13); the editor's double-click rule is J3's EDITOR_V2 §5 item |
| ADR-003 | STATUS:202 (stay on r128 or upgrade) | 0003 | settled: the latest three.js with WebGPURenderer (brief §1); the pin and step-by-step upgrade policy is R3-WGPU-REC2 |
| SEC-MDL-2 | STATUS:257 (`__proto__` through `envMerge` / `rulesMerge`; "belongs with the server-side map-def validation") | 0005; 0006 cites | content is parsed into typed Rust structs by one validating crate on server and client (R1-DATA-REC2, whose fuzz corpus includes `__proto__` keys; REC7 server-side validation); the legacy file stays frozen |
| CDN-SRI | STATUS:262 (no SRI/CSP on the three.js CDN tag; "belongs with hosting") | 0006; 0002 cites | v2 bundles three.js from npm with Vite and serves it from the client host, so no CDN tag; the CSP is the security baseline's (BASELINE-WHEN (b)), part of the production header set |
| WALL-CLOCK | STATUS:273 (11 `performance.now` / `Date.now` reads; "deterministic shared simulation") | 0004 | the step reads no clock (R1-TICK-REC1, with its CI check; R1-DET-REC4); the legacy reads stay in the frozen legacy game |
| RP-13 | PR #28 finding 4: research-plan condition 13 (two branches from one session) | closed | it no longer applies: the research merged in PR #28, and this change-set runs on one branch |

### J3. Traces (condition 8, and part i's question 4)
- **The rule:** every draft names the VISION_V2 §6 item a feature serves. A feature whose need is only in LOOP_V2,
  EDITOR_V2, LOOK_V2 or GLOSSARY §8's foundation says so ("need: LOOK_V2 §4 only"), and `docs/adr/README.md` lists it,
  one line each, for the stop review. **The session's reading (2026-10-31):** a VISION_V2 §3 or §4 target counts like
  LOOP_V2 under condition 8; the draft writes "need: VISION_V2 §4 only", and README lists it. UPGRADE-SCOPE is a
  settled input (Mustafa, 2026-10-31) and is cited the same way ("need: UPGRADE-SCOPE"). This answers LE-G and
  R2-XU-OQ18's question for every draft.
- **Features for README's list** (need outside §6), by draft:
  - 0001: R2-BUD-REC4, REC5, R2-TRN-REC6 (VISION_V2 §4 only).
  - 0002: the repository restructure, the toolchain, CI gate v2, private preview links, generated types, the client
    + local server skeleton (GLOSSARY §8, the foundation), with R5's lint, test and gate recommendations; the editor
    entry (EDITOR_V2 §5).
  - 0003: R3-WGPU-REC2, REC4, R3-GL2-REC3, R3-BUDGET-REC1, R3-MEASURE-REC3 and the quality tiers (VISION_V2 §4 only);
    spatial audio (LOOP_V2 §7, LOOK_V2 §7); the look techniques (LOOK_V2 §1–§7); roof fade (LOOK_V2 §4); halftone
    flashes and line boil (LOOK_V2 §7); the concussion shake (LOOK_V2 §2); the HUD port (LOOP_V2 §11's three-team HUD).
  - 0004: bot behaviour (LOOP_V2 §10, with VISION_V2 §3's "Bots fill empty slots"); map validation with the sim's
    integrator, reachability and the 6 m gap check (EDITOR_V2 §3, LOOP_V2 §9); tiled re-bakes (EDITOR_V2 §3, only if
    needed); damage stages (LOOK_V2 §4); weapon drops and pickups and the player record (UPGRADE-SCOPE); active reload
    (LOOP_V2 §7).
  - 0005: the feel kit as data and the Feel Lab (LOOP_V2 §7); the asset pipeline (LOOK_V2 §8); the map document rules
    (EDITOR_V2 §2); development hot reload (R1-DATA-REC5; its §6.3 trace is partial, R1-DATA-OQ12); the upgrade
    tracks (UPGRADE-SCOPE).
  - 0006: preview hosting and the access wall (GLOSSARY §8, the foundation; PREVIEW-WALL (a)); CI runner costs.
- **EDITOR_V2 §2's five rules → 0005:** rules 1, 2 and 5 (the document is the truth, versioned JSON with stable ids;
  every edit a command with do and undo; a document layer that knows nothing about THREE and is tested in Node) are
  content data (R1-DATA-REC1–REC4, R1-ECS-REC3's static ids); rules 3 and 4 (the scene mirrors the document per
  entity; unmerged edit mode) are the editor's scene, need only in EDITOR_V2: 0005 lists them for EDITOR_V2's own
  architecture plan (EDITOR_V2 §6 step 2), and 0003 cites only the shared material cache. No draft designs the editor.
- **EDITOR_V2 §5's two lines → 0002, listed for DOCS-ALIGN:** "It still runs by double-clicking editor.html" (critic
  rank 25: whether a Wasm validator loads from `file://`; Mustafa if the rule must change) and "Written so it ports
  cleanly if ADR-002 moves the project to modules" (the v2 stack is modules with Vite, so the line is overtaken; 0002
  says so and leaves the text to its owners).
- **No proposed feature for [LATER] or [OPEN] items;** each draft says only what keeps them possible: VISION_V2 §6.4
  seats [OPEN] (R2-SYNC-OQ7), §6.8 kill-cams, replays and highlights [LATER] (recorded matches keep them possible),
  §6.10 accounts and §5 cosmetics [LATER]; LOOP_V2: the Spy, the Hammer's leech, more tanks and the unlock catalogue
  [LATER], the Spy's cost, map size and aiming at height [OPEN] (aiming at height is still served, because §6.2 names
  it); EDITOR_V2: enterable structures, terrain sculpting and the prefab library v2 [LATER] (interiors and floors are
  served through §6.2 and LOOK_V2 §3); LOOK_V2: the tree style [OPEN]; spectators (R2-INT-REC7) unless Mustafa wants
  them (R2-XU-OQ17).

### J4. How ADR 0005 is put together (condition 11; ASSET-RESEARCH (a))
1. **game-designer** writes the ability system section (in-match upgrades included, J5) into its own notes file;
   **graphics-engineer** writes the asset section's GPU rules into its own notes file; each fixes only its own text.
2. **engine-architect** writes the content data part and the asset pipeline part, then, after both sections land, the
   frame: the header, the context, the joined decision, the consequences, the open-question numbering (OQ-0005-n
   across all parts) and Appendix A, placing the two sections verbatim.
3. **ASSET-RESEARCH (a), the advisor's ruling:** the asset part takes LOOK_V2 §8's flow as given and proposes only what
   R1-DATA, R3-CHAR and R3-BUDGET support. Compression formats, the LOD tool, export checks, the material-slot
   renaming step and the per-unit budget numbers are open questions marked "not researched", grouped under one
   research unit, "asset-pipeline research" (graphics-engineer, verified like R3, in its own daytime session), which
   SPIKE_PLAN places before the asset spike (part g's last row; part h, G1). If the look proof test needs authored
   models, it uses plain glTF/GLB exports and decides nothing about compression or LODs. (Correction to part h, G1:
   R3 has one glTF mention, R3-CHAR-F8 at r3:476; the other hits are "GLB" inside "WebGLBackend".)
4. 0005 designs no upgrade, ability, unit or rule (escalation category 2); it shows that the system can carry them.

### J5. UPGRADE-SCOPE as Mustafa updated it (plan step 2 (e))
His words (verbatim in the input file) replace the brief's upgrade note: a roguelike track per unit and per weapon
type (pick one of three per level), owned by the player; tracks survive death and switching and reset at the start of
every match; a class always comes with its own weapon; other weapons come only from killed enemies' drops, which are
world pickups; progress belongs to the weapon type, so a lost gun's track waits for the next gun of that type; the
tracks live on the PlayerState, not the Pawn.

| Part | Carried by | What it decides (never the upgrade's design) |
|---|---|---|
| The tracks as data (per unit type and per weapon type; levels; offers of three; what a pick grants, through the ability system) | 0005 (game-designer) | the data shape and that the GAS subset can carry it; the class's own weapon in the unit definition |
| The player record that holds the tracks, separate from the soldier entity (the PlayerState, not the Pawn): survives death and unit switches, keeps a weapon type's track while no gun of that type is held, resets at match start | 0004 | where the record lives in the sim state (per player slot, beside the soldier tables) and its reset in the match lifecycle (0001's room lifecycle cites) |
| Weapon drops from killed enemies as world pickup entities; which random stream draws the offers of three | 0004 | pickups as sim entities (their lifetime and pickup rule as data from 0005); the stream kind for offers (GLOSSARY: shared stream and authority-only stream) |
| The server's authority over tracks, offers, picks, drops and pickups; their messages and replication; refusing a pick the server did not offer or a pickup it did not place | 0001 | the messages, visibility classes (tracks and offers to their owner; drops spatial) and whether any of it is predicted (the prediction scope) |
| Pick and track telemetry for balance (picks per track and level, offers shown, looted weapons, time per weapon type) | 0006 (the record's shape), recorded per seam S6 (0004 emits, 0001's room writes) | the fields in R6-TEL-REC1's per-slot section |
| The offer and track UI on the page | 0003 | presentation from owner-only data in the block (I-21) |

- **Open, listed by 0005 for Mustafa with options and no pick:** carry slots; ammo for looted weapons; how fast tracks
  level, including catch-up when switching.
- **WEAPON-UNLOCKS, listed the same way by 0005, never resolved:** LOOP_V2 §8's third gate (an unlocked item bought each
  life with match money) and VISION_V2 §1's "Finally unlocked that weapon" sit beside "no weapon choice when buying"
  and "other weapons come only from killed enemies' drops". 0006 cites it (account progression is [LATER]).
- **Answered:** the brief's question "how a pick applies after a switch to a different unit": tracks survive switching
  (switch back and the track is still there). Parts a (0005 item 2) and h (G13) now say so.
- No research id concerns upgrades, so no owner in the id table changes; interface I-21 reads with this table.

### J6. Part i, answered (2026-10-31)
1. The 38 DESIGN ids are disposed in `docs/adr/README.md`'s register, which the coverage script counts (condition 6).
2. 0001 proposes how the stress scene's sim load reaches the 60 Hz margin run (for example the sim core spike's sim
   inside the proof-test server, a stand-in calibrated by the sim core spike's measured step times, or the 60 Hz check
   deferred until the real sim runs the stress scene), recommends one, and lists it as an open question for the
   Advisor (TICK-60-SETUP). The 30 Hz baseline marks may use a stand-in. Since p99s do not add (GLOSSARY: time
   statistics), the margin is measured in one run, never summed.
3. "100 ms of simulated latency": 0001 states its reading as R2's P2 profile, 100 ms round trip (VISION_V2 §4's "about
   100 ms of ping"), with P2's jitter and loss, and lists the reading for the Advisor to confirm (N54).
4. As J3: a VISION_V2 §3 or §4 target counts like LOOP_V2; "need: VISION_V2 §4 only", listed in README.
5. Only the netcode proof test is settled as before the foundation (SLICE-ORDER); SPIKE_PLAN proposes the rest of
   part g's order for Mustafa and the Advisor.
6. The seam holds: machines and certificates in 0006 (LE-L).
7. SPIKE_PLAN owns the stress scene's use in the spikes, citing GLOSSARY §3 for its definition (N52).
8. The Feel Lab test for ABL-PREDICT needs abilities in the engine, so SPIKE_PLAN places it after them (condition 10
   asks only that it be placed) (LE-N).
9. Ruled: ASSET-RESEARCH (a) (J4); TICK-60-HEADROOM as J1.
