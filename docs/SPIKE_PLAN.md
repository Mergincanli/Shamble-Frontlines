# SPIKE_PLAN: the v2 spikes

**Status:** Proposed, with the six decision drafts (`docs/adr/0001`–`0006`) · **Date:** 2026-11-01 · **Author:**
engine-architect (the synthesis, plan step 6), from test-engineer's spike entries and the drafts' "What a spike must
prove" parts · **Signed off by Mustafa:** no

This file names every spike the six drafts ask for, once; the drafts' working names point here. For each spike it says
what it proves (by ADR and decision part), which pass marks judge it (by id: the numbers live only in each draft's
"### Pass marks" table and are never restated here), who judges what, where it sits in the order, a rough size, and the
research spikes it takes in. It decides nothing new: where the drafts leave a choice open, it names the open question.

**Rules for every spike.** A spike is throwaway, lives in `spikes/`, and is exempt from the legacy hard constraints
(SPIKE-SCOPE (a)). None starts before Mustafa and the Advisor approve the drafts (the drafts brief, §4; CLAUDE.md
"Phase 0"). Every server number is at the 30 Hz baseline (TICK-30); the 60 Hz form runs only in the 60 Hz margin run
below. Where a research mark gives 48 and 54, 48 holds (MODES-3 (a)).

**Sizes** (test-engineer's definitions, plan step 4): **S** about a day, **M** 2 to 4 days, **L** a week or more of
session work. A share of another spike is sized as the part it adds.

## Shared fixtures (defined once, cited by every spike)
- **The stress scene:** [GLOSSARY §3](research/GLOSSARY.md#the-stress-scene). At N = 48 (MODES-3 (a)) it is three teams
  of 16 bots in one fight, 15 big vehicles (4 of them very large units), 300 live projectiles, up to 8 active effects per
  soldier, on one DM_Tri-sized 3D greybox. Its counts stay proposals until the game-designer sets caps (R2-BUD-OQ5,
  a design question: `docs/adr/README.md`); the spikes use them as GLOSSARY §3 gives them. A server mark that includes
  the server-only share puts a snapshot-receiving client on every soldier (GLOSSARY §3).
- **The test-mode list:** [GLOSSARY §2](research/GLOSSARY.md#the-test-mode-list): 24v24 and 12v12v12; 16v16v16;
  1v1, 4v4, 2v2v2 and 6v6v6; the N = 64 run, labelled headroom.
- **Machines** (named in each draft's pass-mark key): RS, the reference server, and SH, its shared neighbour (ADR 0006,
  OQ-0006-1; [GLOSSARY: the reference server](research/GLOSSARY.md#the-reference-server)); LG, the load generator, and
  MV, a CX33 running the metrics stack (ADR 0006); ML, the minimum laptop, RD, the recommended desktop, and MAC, a macOS
  machine with Safari (ADR 0003, OQ-0003-1: the models are not named yet); MP, Mustafa's PC in Brave, for what he judges
  only.
- **Network profiles** P0–P4 and the two link-simulator layers L1 and L2: ADR 0001 part 13 (profiles) and ADR 0002 (the
  simulator's design, R5-TEST-REC4). The netcode proof test builds them first, throwaway.
- **Recorded matches** and per-tick state hashes: ADR 0004 (the sim core spike makes the first ones); until one exists,
  a spike that needs a recording uses a synthetic one and says so (ADR 0003's pass-mark key).

## The spikes, in the proposed order

The netcode proof test's place is settled: it runs before the foundation (SLICE-ORDER, condition 3). The rest of the
order below is a proposal for Mustafa and the Advisor (decision map part j, J6 (5)); `docs/adr/README.md` lists it
under "Decisions the drafts ask for".

```mermaid
flowchart LR
  A["Drafts approved<br/>(Mustafa and the Advisor)"] --> W1
  subgraph W1["Wave 1: before the foundation, in parallel"]
    NPT["Netcode proof test (L)<br/>first: SLICE-ORDER"]
    BND["Rust-TypeScript boundary spike (L)"]
    SIM["Sim core spike (L)"]
    LOOK["Look proof test (L)<br/>Mustafa judges"]
    PREV["Preview hosting check (S)"]
  end
  SIM -->|"its sim, if OQ-0001-2 (a)"| M60["60 Hz margin run<br/>(inside the netcode proof test)"]
  NPT --> M60
  W1 --> F["The foundation<br/>(acceptance checks, not a spike)"]
  F --> W3
  subgraph W3["Wave 3: before slice 1"]
    CA["Content and ability spike (L)"]
    RBB["Render budget bench (L)<br/>may merge into the look proof test"]
  end
  W3 --> S["Slices 1 to 6 (SLICE-ORDER)"]
  APR["Asset-pipeline research<br/>(timing: ASSET-RESEARCH)"] --> AS["Asset spike (M)<br/>before the first authored model"]
  S --> FL["Feel Lab test (M)<br/>after abilities are playable (slice 4)"]
```

In words: (0) the drafts are approved; (1) wave 1 runs five spikes in parallel, the netcode proof test first in
priority, with the 60 Hz margin run inside it once its sim load is settled (OQ-0001-2); (2) the foundation; (3) the
content and ability spike and the render budget bench, before slice 1; (4) slices 1–6, with the asset-pipeline research
and the asset spike before the first authored model, and the Feel Lab test once abilities are playable.

| # | Spike | Size | Place | Pass marks |
|---|---|---|---|---|
| 1 | The netcode proof test | L (+ M for ADR 0006's operations marks) | wave 1, first | PM-0001-1 to -56; PM-0004-67; PM-0006-11 to -27 |
| 2 | The 60 Hz margin run | inside 1 | wave 1, after its sim load is settled | PM-0001-39, -40; PM-0004-71, -72 |
| 3 | The Rust ↔ TypeScript boundary spike | L (+ M ADR 0004's share; + S ADR 0006's, optional) | wave 1 | PM-0002-1 to -34; PM-0004-8, -28, -61, -68 to -70; PM-0006-28 to -31 (optional) |
| 4 | The sim core spike | L, the largest | wave 1 | PM-0004-1 to -61 |
| 5 | The look proof test | L (+ S or M for ADR 0005's share) | wave 1 | PM-0003-1 to -17; PM-0005-33 to -37 |
| 6 | The preview hosting check | S (calendar: at least 2 days) | wave 1; ends before the foundation's preview step | PM-0006-1 to -10 |
| – | The foundation's acceptance checks (not a spike) | L (six change-sets) | step 2 | PM-0002-35 to -69; later PM-0002-70 to -76 |
| 7 | The content and ability spike | L | wave 3 | PM-0005-1 to -32; PM-0004-62 to -66 |
| 8 | The render budget bench | L | wave 3 | PM-0003-18 to -42 (PM-0003-1 again); PM-0003-43 at each later upgrade |
| – | The asset-pipeline research (a research unit, not a spike) | one daytime session | before 9 | its own marks, appended to 9 |
| 9 | The asset spike | M (resized after the research) | before the first authored model | PM-0005-38 to -44, plus the research's marks |
| 10 | The Feel Lab test (ABL-PREDICT) | M | after abilities are playable (slice 4) | PM-0005-45, -46 |

### 1. The netcode proof test
- **Defined in:** [GLOSSARY §8](research/GLOSSARY.md#the-netcode-proof-test) (the Advisor's drafts brief, §2). Owner:
  ADR 0001 ("What a spike must prove").
- **What it proves:** ADR 0001 parts 1 and 10 (the online path end to end on Chrome, Edge, Firefox and Safari, over
  WebTransport with the WebSocket fallback); parts 2, 3, 6, 11 and 14 (48 simulated clients in the stress scene within
  ADR 0001's budgets at the 30 Hz baseline: server tick time and its shares, bandwidth per client both ways, memory per
  room, rooms per VM, under loss, bursts and hostile input); parts 4, 7 and 8 (one human client at P2 sees prediction,
  reconciliation and lag-compensated hits that look correct). Also ADR 0004 part 5 (S17: the server loop keeps a 30 Hz
  tick from elapsed time) and ADR 0006 parts 5, 7, 8 and 9 (rooms per VM on both machine types, the firewall, egress per
  match-hour, the spike's own bill, logging cost, metrics, the development certificates).
- **Pass marks:** PM-0001-1 to PM-0001-56 (PM-0001-39 and -40 belong to the 60 Hz margin run below; PM-0001-54 and -55
  only when their condition holds); PM-0004-67; PM-0006-11 to PM-0006-27 (PM-0006-13 shares its runs with ADR 0001's
  R2-ROOM-REC2 marks: one set of runs, read the same in both tables). Its server also serves the online halves of
  PM-0004-61 and PM-0004-70.
- **Who judges what:** the scripts judge every measured mark. Mustafa judges "look correct": the netcode checklist
  (ADR 0001, inside PM-0001-53), at the profile OQ-0001-3 confirms, beside the measured correction, interpolation and
  hit-registration marks. He also runs the operations checklist (ADR 0006, inside PM-0006-12 and PM-0006-18).
- **Depends on:** the drafts approved; OQ-0006-1 answered (the CCX23 and CX33 orderable in one EU location, in
  Mustafa's Hetzner account) and LG and MV beside them; ML, RD and MAC named (OQ-0003-1); a throwaway sim with movement,
  hitscan and bots as protocol clients, native and Wasm, with stand-in vehicles and projectiles at the stress scene's
  counts (PM-0001-2); the link simulator's L1 and L2 (ADR 0002's design), built here first; a DM_Tri-sized greybox with
  enterable buildings and the target course; the development certificates (ADR 0006); the Advisor's readings of
  OQ-0001-3 before the checklist; Mustafa's session in Brave; the spike's run plan under its bill mark (PM-0006-18)
  before the first VM.
- **Place:** wave 1, first in priority: before the foundation (SLICE-ORDER condition 3). It shares its client harness
  with the boundary spike.
- **Size:** L. Two transports on four browsers, two simulator layers, a room server with interest, encoding and lag
  compensation at 48 clients, a load generator, repeated 30-minute soaks, 8 rooms on two machine types, planted defects,
  Mustafa's session; ADR 0006's operations marks add M; ADR 0004's share is S.
- **Research spikes it subsumes:** R2-BUD-REC6 (the server tick budget in the stress scene), R2-SYNC-REC9 (48
  clients), R2-TRN-REC1 and R2-TRN-REC3 (both transports: interop, load, hostile input), R2-ROOM-REC2 (a room holds its
  tick; memory; rooms per VM), R1-TICK-REC2's P1 at 30 Hz through the total, and P3; with them R2-SYNC-REC1 to REC5 and
  REC8, R2-BUD-REC1 to REC5, R2-TRN-REC2, REC4 and REC6, R1-ABL-REC4 (own weapons; the ability half waits for the Feel
  Lab test), R2-INT-REC1 (c), REC2 (b) and REC4 (their costs); conditionally R2-SYNC-REC6 (through R2-BUD-REC2's flight
  mark) and R2-SYNC-REC7 mark 3; R1-TICK-REC4 (through PM-0004-67: the decision map placed it here, ADR 0001's spike
  text does not name it); R2-TRN-REC5 marks 1 (the development half), 2 and 3, R6-HOST-REC2, R6-COST-REC1, R6-OBS-REC1
  and REC2 (ADR 0006's marks); R5-TEST-REC4 and REC5 (the simulator and the load layer) are built here first. Together
  these are the research spike the netcode proof test is (carry-forward 5).
- **It decides:** R2-TRN-OQ1, OQ2, OQ3, OQ5, OQ8; R2-BUD-OQ3, OQ7, OQ9, OQ10; R2-SYNC-OQ3 (through OQ-0001-6), OQ5 (if
  vehicles move), OQ12; R2-ROOM-OQ1; R2-XU-OQ2, OQ4, OQ5, OQ6, OQ7, OQ15; OQ-0001-6, OQ-0001-7; R6-COST-OQ1; it feeds
  OQ-0006-2 and gives Mustafa and the Advisor the numbers for OQ-0001-5.

### 2. The 60 Hz margin run (TICK-60-HEADROOM; TICK-60-SETUP)
- **Defined in:** [GLOSSARY §8, the 60 Hz margin](research/GLOSSARY.md#the-60-hz-margin). Owner: ADR 0001 part 2. A
  run inside the netcode proof test, not a separate spike.
- **What it proves:** whether the server may tick at 60 Hz. As ADR 0001 states it: in one run, on one vCPU of the
  reference server, in the stress scene at N = 48 with a snapshot-receiving client on every soldier, over R2-BUD-REC6's
  30-minute soak with re-flood ticks, a 60 Hz margin of at least 25 % (a p99 server tick time of at most 6.25 ms against
  8.33 ms) and every 60 Hz mark the research sets (R2-BUD-REC6's two shares, R1-TICK-REC2 P1–P3). Otherwise the tick
  stays at 30 Hz. The margin is measured in one run, never summed from separate runs (p99s do not add:
  [GLOSSARY, time statistics](research/GLOSSARY.md#time-statistics)).
- **Pass marks:** PM-0001-39, PM-0001-40; ADR 0004's PM-0004-71 (the sim step share at 60 Hz, read from the same run)
  and PM-0004-72 (the time mechanism and movement checks at 60 Hz).
- **Who judges what:** the scripts. Before the run, Mustafa and the Advisor confirm TICK-60-HEADROOM's reading
  (OQ-0001-1) and the Advisor picks how the stress scene's sim load reaches the run (TICK-60-SETUP, OQ-0001-2).
- **Depends on:** OQ-0001-1 and OQ-0001-2 answered. Under OQ-0001-2's recommended option (a), the sim core spike's sim
  runs inside the proof-test server, so the run waits until that sim runs the stress scene; under (c), the fallback,
  the check waits until the real sim runs the stress scene (slice 6) and the tick stays at 30 Hz until then.
- **Place:** wave 1, inside the netcode proof test, after the sim core spike's sim runs the stress scene (option (a)).
- **Size:** part of the netcode proof test's L (the soaks repeated at 60 Hz).
- **Research spikes it subsumes:** R1-TICK-REC2 P1 at 60 Hz (ADR 0004's share), P2 at 60 Hz, P3 at 60 Hz; R2-BUD-REC6's
  60 Hz shares. **It decides:** carry-forward 2 (the 60 Hz margin); R1-TICK-OQ1 only if the margin passes (then ADR
  0004 and ADR 0002 decide).

### 3. The Rust ↔ TypeScript boundary spike
- **Owner:** ADR 0002 ("What a spike must prove", items 1–10). The drafts also call it "the boundary spike".
- **What it proves:** ADR 0002 part 5 (one module Worker; the page makes no Wasm call and touches no network; one
  transferred block per tick, B2, with the SharedArrayBuffer ring, B3, measured against it; one command per tick; the
  Worker's calls per tick constant in N; the page's frame time with the sim within its budget at N = 12 to 48 on the
  minimum laptop; the generated types and the layout hash; the isolation flag), part 6 (the Worker owns the connection;
  the client + local server skeleton; one Wasm instance or two in a local match, OQ-0002-3), parts 2, 4 and 7 (Wasm
  size, tools, hot reload into the Worker), part 11 (the editor from `file://`, OQ-0002-1); ADR 0004 part 5 (S18: the
  Worker's pacing, smooth motion on 60, 120 and 144 Hz displays, hidden tabs), part 6 (S2: the client's sim share) and
  part 20 (S20: recovery after a trap); ADR 0005's content files reaching the Worker (PM-0002-31, R1-DATA-OQ8).
- **Pass marks:** PM-0002-1 to PM-0002-34 (PM-0002-21, -27 and -33 conditional; PM-0002-28 gates once the sim core
  spike's sim runs in the Worker); PM-0004-8, PM-0004-28, PM-0004-68 to PM-0004-70, and PM-0004-61 with the netcode proof
  test's server; optional, only if Mustafa has created the EU Sentry organisation (OQ-0006-8): PM-0006-28 to PM-0006-31
  (otherwise they move to the first online playtest's milestone). ADR 0003's PM-0003-20 (the p99 of the page's own sim
  work) is measured on this spike's Worker and block in the render budget bench.
- **Who judges what:** the scripts; Mustafa runs the spike checklist (ADR 0002, inside PM-0002-34) in Brave.
- **Depends on:** the drafts approved; ML named and on hand (OQ-0003-1), MAC with the latest Safari, and displays at
  120 and 144 Hz; the netcode proof test's throwaway sim, client harness and room process (it runs beside it); ADR
  0003's frame-time method (its part 8); the sim core spike's sim for PM-0002-28 and PM-0004-8, -28 (a stand-in until
  then); Mustafa's session in Brave.
- **Place:** wave 1, beside the netcode proof test; its result fixes the skeleton the foundation builds.
- **Size:** L (ADR 0002's part); ADR 0004's share M; ADR 0006's optional share S.
- **Research spikes it subsumes:** R1-BND-REC1 to REC5, and REC6's marks (measured once, here, in the shared client
  harness); R1-TICK-REC5 (as ADR 0004's S18); R1-TICK-REC2 P2 (as ADR 0004's S2); R5-LAYOUT-REC3, REC5; R6-OBS-REC3
  (optional). **It decides:** R1-BND-OQ2, OQ3 (the core count recorded), OQ6, OQ9 (only if isolation is adopted);
  R1-XU-OQ3, OQ4; R1-TICK-OQ2; R1-DATA-OQ8; R1-RR-OQ5 (with the sim core spike), OQ6; R5-LAYOUT-OQ4; R5-TSLINT-OQ3, OQ4,
  OQ6; R5-TEST-OQ1; R6-OBS-OQ3, OQ4 (optional); critic ranks 4 (the implementation side) and 37; it gives OQ-0002-1,
  OQ-0002-3, OQ-0002-4 and OQ-0004-7 their results and feeds R5-CI-OQ4.

### 4. The sim core spike
- **Owner:** ADR 0004 ("What a spike must prove", claims S1–S14, S19, S20).
- **What it proves:** ADR 0004 parts 2, 3, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 20 and 21: the Rust sim on one vCPU
  of the reference server and in Wasm on the minimum laptop, in the stress scene at N = 48 at 30 Hz (the sim step share
  and its parts, the scale steps and the N = 64 headroom run); snapshots, exact restore and stable ids; same-build and
  cross-target identity by per-tick state hashes on native, Node, three browser engines and real Safari, with a suite
  that catches each planted divergence at its tick; parry3d with our own movement; both navigation generators, the path
  queue, avoidance, floors and typed links; sight; big vehicles as obstacles, structure states and three damage stages;
  the editor's reach checks; no panic from commands.
- **Pass marks:** PM-0004-1 to PM-0004-61 (PM-0004-8 and -28 in the boundary spike's Worker; PM-0004-61 with the netcode
  proof test's server).
- **Who judges what:** the scripts; no mark here needs a person's judgement.
- **Depends on:** the drafts approved; RS orderable (OQ-0006-1), ML and RD named (OQ-0003-1), MAC with Safari; the
  scenes built in the spike (the DM_Tri-sized greybox for the stress scene, the Octopon-3 greybox, the two-floor test
  building with three stages, the ledge block, the box-only parity scenes); a stated bot set for "full AI" (ADR 0004's
  S14; OQ-0004-1 stays open); legacy golden fixtures captured from the legacy oracle (R5-TEST-REC3); a throwaway replay
  runner and tick bench in ADR 0002's shape; ADR 0001's prediction scope.
- **Place:** wave 1, in parallel with the netcode proof test; its sim (OQ-0001-2 (a)) feeds the 60 Hz margin run.
- **Size:** L, the largest spike (a throwaway sim with movement, the vehicle sweep, two navigation generators, bots,
  sight and structure states; a determinism suite over seven targets; three machines; about 60 marks, most of them
  scripted checks that can join CI afterwards).
- **Research spikes it subsumes:** R1-ECS-REC1, REC2, REC3 (REC4 only if Rapier comes back); R1-TICK-REC1, REC2 (P1; P2
  with the boundary spike), REC3, REC6; R1-DET-REC1 to REC6 (REC7 stands or falls with them); R1-ABL-REC3's fire-rate
  half; R4-PHYS-REC1 to REC5; R4-NAV-REC1 to REC4; R4-FLOORS-REC1 to REC3; R4-LOS-REC1 to REC3; R4-OBST-REC1, REC2 (REC3
  only if tiling is needed); R5-TEST-REC2's suite. **Placed here by this plan** (the decision map's part g put them
  here; no draft's table names them; ADR 0001 and ADR 0002 adopt the recommendations): R2-SYNC-REC6 mark 1 (throws
  bit-identical on Wasm and native) and R5-LAYOUT-REC4 (one feature set for `sim` on every target); the foundation later
  re-proves R5-LAYOUT-REC4 as a gate check. **It decides:** R1-ECS-OQ1 (the size recorded), OQ4 (OQ5 only if Rapier
  wins); R1-TICK-OQ5, OQ9; R1-DET-OQ1, OQ2, OQ4, OQ8, OQ10, OQ11 (back only if the cost asks); R1-RR-OQ1; R4-PHYS-OQ2,
  OQ3, OQ4 (with ADR 0005's map format); R4-NAV-OQ1 to OQ3; R4-LOS-OQ5; R4-OBST-OQ4; R1-ABL-OQ7 and R1-XU-OQ7 (through
  PM-0004-9 and -11); critic rank 3.

### 5. The look proof test (LOOK_V2 §9)
- **Owner:** ADR 0003 ("The look proof test", parts 1, 3, 4, 5 and 11); ADR 0005's share (its GPU and asset-pipeline
  sections' first bullets).
- **What it proves:** how each `[DECIDED]` item of [LOOK_V2](LOOK_V2.md) looks, judged by Mustafa in Brave beside legacy
  screenshots at the same framing; each look technique's GPU time, draw calls and memory alone, on both reference
  machines and both backends; both ink families at 12, 24 and 48 soldiers on the minimum laptop at Low; one build on
  both backends in the four browsers with the same look; no first-sight shader stutter; the backend reported (and
  Brave's on Mustafa's PC). For ADR 0005: plain glTF/GLB exports load on both backends with the engine's shared
  materials per slot class, every model listed by id, a broken link refused at load, and Mustafa's pick per asset
  family (OQ-0005-AP2).
- **Pass marks:** PM-0003-1 to PM-0003-17; PM-0005-33 to PM-0005-37.
- **Who judges what:** Mustafa judges the whole look: the look checklist (ADR 0003, inside PM-0003-17), which names each
  LOOK_V2 item it shows beside a legacy screenshot, and the families checklist (ADR 0005, inside PM-0005-37). He also
  makes the calls ADR 0003 leaves to him (OQ-0003-2, -3, -5, -8, -12, -15; the ink family). The scripts judge the
  frame-time and GPU marks on ML and RD.
- **Depends on:** the drafts approved; ML and RD named and on hand (OQ-0003-1), MAC with Safari; a few authored test
  assets as plain glTF/GLB exports with named material slots and no compression or LOD step (ASSET-RESEARCH (a): one
  soldier with faction colour slots, one weapon, one two-floor building with its damage stages, one very large unit,
  a few props); legacy screenshots at the same framing; the measurement harness of ADR 0003 part 8, built here first; a
  synthetic recording for the scene; the content crate's whole-set check (a minimal loader, since the content and
  ability spike comes later); for OQ-0005-AP2, the ported kit builders beside the authored models (ADR 0005's share; ADR
  0003's own asset list names only authored assets).
- **Place:** wave 1, in parallel with the netcode proof test (they share nothing); before any art production and before
  slice 1's renderer.
- **Size:** L; ADR 0005's share S, or M if the kit builders are ported for the families' comparison.
- **Research spikes it subsumes:** R3-LOOK-REC1, REC2, REC3; R3-WGPU-REC1, REC3, REC4; R3-GL2-REC1, REC3; R3-BUDGET-REC1
  (the machines named). **It decides:** R3-WGPU-OQ1, OQ2, OQ3; R3-GL2-OQ3; R3-LOOK-OQ1, OQ3, OQ4; R3-MEASURE-OQ1, OQ3;
  R3-GL2-OQ1 and R3-LOOK-OQ6 after their source readings; it gives Mustafa what he needs for OQ-0003-2, -3, -5, -6, -8,
  -12 and -15, and OQ-0005-AP2.

### 6. The preview hosting check
- **Owner:** ADR 0006 ("What a spike must prove", item 1).
- **What it proves:** ADR 0006 parts 1, 2 and 3: the client host serves the production and preview header sets as
  generated, on branch aliases and hash URLs; `crossOriginIsolated` follows the isolation flag in Chrome, Edge, Firefox,
  Safari and Brave; the content bundle is cached immutably; every file fits the host's 25 MiB limit; **PREV-DELETE:** a
  closed pull request's build is gone from every link within 24 h; what the access wall covers, with one login per
  Brave session; whether direct uploads count against the host's build allowance; whether GitHub shows the environment
  URL.
- **Pass marks:** PM-0006-1 to PM-0006-10.
- **Who judges what:** the scripts; Mustafa runs the hosting checklist (ADR 0006, inside PM-0006-9) in Brave and in his
  dashboard.
- **Depends on:** the drafts approved; Mustafa's Cloudflare account with a test project and its deploy token in a
  `preview` environment (secrets are his to add); a static test page with a module worker, a `.wasm` and a hashed
  bundle stand-in; throwaway deploy and close workflows; ML and MAC with the four browsers.
- **Place:** wave 1, in parallel with the netcode proof test; it must end before the foundation's preview step (a
  failure of PREV-DELETE with no workaround brings back ADR 0006's alternative, Workers static assets).
- **Size:** S of session work, with a calendar span of at least two days (a 24 h wait after each close, then a policy
  switch).
- **Research spikes it subsumes:** R6-HOST-REC1; R5-PREV-REC2; R5-PREV-REC3 (the 24 h mark; the sweep job is the
  foundation's); R5-PREV-REC4 marks 2 and 3 (mark 1 is the foundation's); R5-PREV-REC5's mark (on the test project).
  **It decides:** PREV-DELETE (R5-PREV-OQ2); R5-PREV-OQ5; R5-PREV-OQ6; it gives Mustafa what he needs for OQ-0006-4.

### The foundation's acceptance checks (not a spike)
- **Defined in:** [GLOSSARY §8, the foundation](research/GLOSSARY.md#the-foundation). Owner: ADR 0002's migration plan,
  phase 2. Listed here so the order can place it; its checks are milestones, not a throwaway spike.
- **What it proves:** the move keeps the legacy game playable and its oracle exact; the pinned toolchains build and
  test from a fresh clone; every gate check bites; CI's required check cannot be fooled; the generated types hold in CI;
  the client + local server skeleton draws an empty scene from blocks; previews arrive with the right headers. It ends
  when an empty scene runs through the full stack in the browser with CI green and a preview link.
- **Pass marks:** PM-0002-35 to PM-0002-69 (PM-0002-69: the foundation is done); later, at slices 1 to 6, PM-0002-70 to
  PM-0002-76. Mustafa runs the foundation checklist (ADR 0002).
- **Depends on:** the drafts accepted; the boundary spike's and the netcode proof test's results; the preview hosting
  check before the preview step; Mustafa's Cloudflare token and branch-protection setting.
- **Size:** L (six change-sets: restructure, toolchain, gate and CI, generated types, skeleton, previews).
- **Research it carries:** R5-LAYOUT-REC1, REC2, REC6; R5-TSLINT-REC1–REC4; R5-RUSTLINT-REC1–REC6; R5-TEST-REC1,
  REC6; R5-CI-REC1–REC5; R5-PREV-REC1, REC4 (mark 1); R5-GATE-REC1–REC4; R1-BND-REC4's CI checks. It re-proves as gate
  checks marks first proven in the sim core spike (R5-LAYOUT-REC4; R5-TEST-REC2's tools) and the netcode proof test
  (R5-TEST-REC4's L1). It decides R5-GATE-OQ5 and R5-CI-OQ1.

### 7. The content and ability spike
- **Owner:** ADR 0005 ("What a spike must prove", item 1: the content data section's items 1–9 and the ability
  section's items 1–7); ADR 0004's share (parts 16 and 18: S15, S16).
- **What it proves:** one Rust content crate gives one verdict on native and Wasm (exact floats, one canonical writer, a
  labelled fuzz corpus with no crash and no accepted invalid file, bounded collections, no panics, a schema and types
  that never drift); exact migrations of Cabana and DM_Tri; development reload; one hashed bundle checked at join;
  server-side checks of overrides; the legacy tuning imported with every value accounted for; presentation that cannot
  reach an outcome; the legacy weapons and the three T1 abilities from data with no ability-specific Rust code; a new
  combination by data alone; VISION_V2 §6.4's two primitives; tech gates; Mustafa's upgrade tracks on test tracks
  (UPGRADE-SCOPE as checks); the ability step's cost at 384 active effects within its part of the sim step share, with
  no heap allocation, replaying byte for byte on native and Wasm (ADR 0004).
- **Pass marks:** PM-0005-1 to PM-0005-32 (PM-0005-11 only if OQ-0005-CD4 picks option (b)); PM-0004-62 to PM-0004-66.
- **Who judges what:** the scripts; Mustafa's content trial (the content checklist, ADR 0005, inside PM-0005-17) answers
  OQ-0005-CD1.
- **Depends on:** the sim core spike's throwaway sim (the step, the stress scene, the replay runner, the tick bench);
  `docs/legacy-tuning.json` and the legacy oracle for the goldens; Cabana, DM_Tri and any maps Mustafa supplies;
  LOOP_V2 §5's T1 abilities; RS and ML; MAC with Safari (PM-0005-2); the boundary spike's Worker and build settings;
  ADR 0002's network simulator only for PM-0005-11; Mustafa's session for the content trial.
- **Place:** wave 3, after the sim core spike and the foundation, before slice 1 (weapons are data from slice 1).
- **Size:** L (ADR 0004's share S).
- **Research spikes it subsumes:** R1-DATA-REC1 to REC8 (REC5's marks 2 and 3 only under OQ-0005-CD4 (b); REC7 without
  the map-upload path); R1-ABL-REC1, REC2, REC5, REC6; R1-ABL-REC3's replay half; R5-TEST-REC3's goldens for the tuning
  tables and locational damage. **It decides:** R1-DATA-OQ2 (the numbers; the Advisor sets the budgets, OQ-0005-CD2),
  OQ4, OQ5, OQ9; R1-RR-OQ2, OQ3 (the loader's half); it gives Mustafa what he needs for OQ-0005-CD1 and the Advisor
  OQ-0005-AB13's trigger.

### 8. The render budget bench (may merge into the look proof test)
- **Owner:** ADR 0003 ("The render budget bench", parts 6, 7, 8 and 9).
- **What it proves:** every row of ADR 0003's budget table in the benchmark scene (a recording of the stress scene),
  with the techniques the look proof test kept, on both machines: characters at N = 48 on screen, the four very large
  units, the Firefox WebGL 2 worst case, the quality tiers, the harness and the CI split, audio in the benchmark fight;
  later, each three.js upgrade.
- **Pass marks:** PM-0003-18 to PM-0003-42 (PM-0003-40 only if occlusion rays are ever proposed), PM-0003-1 checked
  again; PM-0003-43 at each later upgrade.
- **Who judges what:** the scripts; Mustafa's bench checklist (ADR 0003) for frame pacing and voices, beside PM-0003-38,
  -39 and -42; he answers OQ-0003-10 on its numbers.
- **Depends on:** the look proof test's kept techniques and Mustafa's picks; the boundary spike's Worker and block
  (PM-0003-20 runs on them); a recorded match from the sim core spike, or a synthetic one; the gate v2's CI job
  (PM-0003-36), so it comes after the foundation.
- **Place:** wave 3, after the look proof test, the boundary spike and the foundation; before slice 1's renderer work.
- **Size:** L.
- **Research spikes it subsumes:** R3-CHAR-REC1 to REC4; R3-BUDGET-REC2, REC3; R3-GL2-REC2; R3-MEASURE-REC1 to REC4
  (ADR 0003 places REC1 and REC4 here, where the decision map's part g had them in the look proof test; the draft
  holds); R3-AUDIO-REC1 to REC4; R3-WGPU-REC2 (later). **It decides:** R3-CHAR-OQ1, OQ2, OQ4 (OQ6 only if the draw-call
  cap is still missed); R3-BUDGET-OQ5, and R3-BUDGET-OQ6 with OQ-0003-4; R3-AUDIO-OQ1, OQ2 (OQ6 only if the built-in
  panners miss PM-0003-38); it gives Mustafa the numbers for OQ-0003-10.

### The asset-pipeline research (a research unit, before the asset spike)
- **Ruled by:** ASSET-RESEARCH (a) (the advisor's condition 4). The research has no findings on the asset pipeline
  (decision map part h, G1), so ADR 0005 takes LOOK_V2 §8's flow as given and lists the rest as "not researched"
  open questions: OQ-0005-AS1 (compression formats), AS2 (the LOD tool and levels), AS3 (export checks), AS4 (the
  material-slot renaming step), AS5 (the per-unit budgets beyond the two derived caps), and OQ-0005-AP1 (where the
  automated step runs).
- **Who:** graphics-engineer, verified like R3, in its own daytime session; it appends its marks to the asset spike
  before that spike starts.
- **Place:** before the asset spike. **Timing is a decision for Mustafa and the Advisor** (ASSET-RESEARCH in
  `docs/adr/README.md`): they may move it earlier, for example beside wave 1, if art production should start sooner.

### 9. The asset spike
- **Owner:** ADR 0005 ("What a spike must prove", item 3: the GPU section's and the asset-pipeline section's second
  bullets).
- **What it proves:** the automated step processes the test models and writes their list entries; asset files are
  served hashed and immutable, under the host's 25 MiB limit; the gate holds the per-unit budgets and keeps asset readers
  out of `sim`, `server` and the player build; every slot classed and every faction slot coloured on both backends; the
  stress scene's fight on screen from real models within ADR 0003's frame totals; LOD levels that keep the silhouette,
  if any; the asset-pipeline research's own marks for OQ-0005-AS1 to AS5.
- **Pass marks:** PM-0005-38 to PM-0005-44 (PM-0005-44 only with its condition), plus the research's marks.
- **Who judges what:** the scripts; Mustafa's LOD checklist (ADR 0005), only with PM-0005-44's condition.
- **Depends on:** the asset-pipeline research, verified; the look proof test's results and Mustafa's family picks
  (OQ-0005-AP2); the render budget bench and its benchmark scene (PM-0005-43); ADR 0006's preview hosting (PM-0005-39);
  the gate v2's `--prove` (the foundation); authored models for every family that goes through the flow.
- **Place:** before the first authored model enters the game.
- **Size:** M, to be sized again once the research names the tools.
- **Research spikes it subsumes:** none in R1–R6 (decision map part h); the asset-pipeline research's, once written.
  **It decides:** proves the answers to OQ-0005-AP1 and OQ-0005-AS1 to AS5.

### 10. The Feel Lab test (ABL-PREDICT)
- **Owner:** ADR 0005 (the ability section, item 8); ADR 0001 OQ-0001-4 lists ABL-PREDICT. A later test, not a
  pre-acceptance spike: it needs abilities in the engine (decision map LE-N).
- **What it proves:** what predicting the player's own abilities changes for the Molotov and the Shield at P2, in
  numbers (response time, confirmed share, rollbacks, corrections) and in Mustafa's verdict, blind between the two
  variants.
- **Pass marks:** PM-0005-45, PM-0005-46; ADR 0001's R1-ABL-REC4 ability marks run here if the predicted variant is built.
- **Who judges what:** Mustafa: the Feel Lab checklist (ADR 0005, inside PM-0005-46) answers ABL-PREDICT.
- **Depends on:** abilities playable in the engine (slice 4: tech tiers and the three T1 abilities); both variants
  switchable in one build; ADR 0002's network simulator (L2) at P2; the server on RS; Mustafa's session in Brave.
- **Place:** after abilities are playable (slice 4). The other Feel Lab weighings the drafts name run in the same kind of
  session once their subject is playable: the rewind cap's feel (OQ-0001-5), the projectile prestep (OQ-0001-8), sub-tick
  shot stamps if the proof test says they are needed (OQ-0001-7).
- **Size:** M.
- **Research spikes it subsumes:** R1-ABL-REC4's ability half; R1-ABL-OQ3's ability-cue half. **It gives Mustafa** what
  he needs for ABL-PREDICT (R2-SYNC-OQ10, R1-ABL-OQ4).

## Later milestones (marks proven in phase 1 slices, not spikes)
From the decision map's part g, as the drafts keep them: R2-ROOM-REC1, REC3–REC8 and R2-INT-REC2, REC3, REC5–REC7
(slice 3, online; the visibility pass with slice 2's bots); R2-SYNC-REC6, REC7 (when projectiles and vehicles enter);
R6-HOST-REC3 and the access wall (the first multiplayer preview, PREVIEW-WALL (a)); R6-HOST-REC4, R6-OBS-REC3, REC4
(before the first online playtest, with the security baseline, BASELINE-WHEN (b)); R6-TEL-REC1–REC3, R6-COST-REC2
(slice 4, the loop); R1-DATA-REC5's editor reload (slice 5); R4-OBST-REC3 (only if tiling is needed); the mode files'
rejections (R2-ROOM-REC7's mark, ADR 0001's, at slices 2 and 3).

## What this plan does not decide
The order after the netcode proof test, TICK-60-HEADROOM (OQ-0001-1), TICK-60-SETUP (OQ-0001-2), the "100
ms" reading (OQ-0001-3), the reference machines (OQ-0003-1), the server location (OQ-0006-1, OQ-0006-2) and
ASSET-RESEARCH's timing are listed for Mustafa and the Advisor in `docs/adr/README.md`, "Decisions the drafts ask for".
