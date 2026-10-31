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
   the netcode proof test shows at least 25 % headroom. 0001 proposes what the headroom is measured against
   (carry-forward 2; the plan's TICK-60-HEADROOM, for the advisor at the stop). Also the snapshot rate (R2-BUD-REC1),
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
   Pawn; picks survive death and reset at the start of every match); how a pick applies after a switch to a different
   unit is an open question (brief §3).
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
| S1 | Tick | 0001: the tick rate rule under TICK-30 and the 25 % headroom base; the snapshot rate; the server tick budget's total (50 %) and the server-only share (25 %) and its parts | 0004: the fixed-tick mechanism (counter, `dt`, conversion, cadences, server loop, Worker pacing); the sim step share (25 %) and its parts; the client's sim share (40 %); determinism | 0001 adds 0004's sim step share into the total; neither restates the other's parts. R1-TICK-REC2's P1 and P2 sim marks are 0004's; P3 (prediction replay) is 0001's. |
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
| R1-TICK-REC2 | 0001 | 0004,SPIKE_PLAN | the tick rate rule: changed by TICK-30 (30 Hz baseline, 60 Hz only with >= 25 % headroom); P1/P2 sim marks are 0004, P3 replay 0001 |
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
| R1-TICK-OQ1 | 0001 | 0004 | 60 vs 64 Hz; under TICK-30 it matters only if 60 Hz passes its headroom test |
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
holds). The 30 Hz column is the baseline (TICK-30); the 60 Hz column is kept only where the headroom test needs it
(carry-forward 2). "p99" unless named otherwise ([GLOSSARY: time statistics], GLOSSARY.md:141).

### c1. Tick and server budgets
| # | Number | 30 Hz (baseline) | 60 Hz (headroom test only) | Source | Owner | GLOSSARY term |
|---|---|---|---|---|---|---|
| N1 | Tick period | 33.3 ms | 16.7 ms | arithmetic, 1 / rate (GLOSSARY.md:82-96) | 0001 (the rate), 0004 (the constant) | tick period |
| N2 | Tick rate | 30 Hz | only if the netcode proof test shows ≥ 25 % headroom | brief §1 (TICK-30), replacing R1-TICK-REC2's rule (r1:2061-2062) | 0001 | tick period |
| N3 | 60 Hz headroom base (what 25 % is measured against) | n/a | proposed in the plan (TICK-60-HEADROOM): the stress scene with a snapshot-receiving client on every soldier, one vCPU of the reference server, p99 server tick time at 60 Hz ≤ 75 % of the 8.33 ms server tick budget (arithmetic: 1 − 0.25), re-flood ticks included | brief §1; plan.md "Decisions for you"; carry-forward 2 | 0001 (the advisor confirms at the stop) | the server tick budget |
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
