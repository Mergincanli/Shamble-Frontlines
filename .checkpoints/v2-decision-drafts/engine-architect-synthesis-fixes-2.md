# Synthesis, second fix round (plan step 9): engine-architect, 2026-11-02

Base: commit 9a03999 (every BEFORE copied from `git show 9a03999:<path>` by script, whole lines, each checked to occur
exactly once in its file). Covers: consistency F1, F2, F4, F7, F8, F9, F10, F12, F13, F14; the synthesis verifier's rows
33, 37, 53, 54 and findings 6–9 (as skeptic 3 judges rows 33, 37, 54); the addendum (counts, pointers, MAC, fragments as
damage, R1-RR-OQ6, OQ-0002-8, the inventory rows, completeness items 4, 11, 15); verifier fixcheck-b's closing list.
Glossary judgements (completeness item 11): in the notes file, engine-architect-synthesis-fix2-notes.md.

### FIX G-OVW-14 (glossary: "48" alone for the largest mode)
FILE: docs/ENGINE_OVERVIEW.md
BEFORE:
editor. The server ticks at 30 Hz (TICK-30) for up to 48 players (MODES-3 (a)); 60 Hz only with
[the 60 Hz margin](research/GLOSSARY.md#the-60-hz-margin).
AFTER:
editor. The server ticks at 30 Hz (TICK-30) for up to N = 48 players (the largest mode, 24v24: MODES-3 (a)); 60 Hz
only with [the 60 Hz margin](research/GLOSSARY.md#the-60-hz-margin).
END

### FIX completeness-15 (the diagram shows the per-frame flow and the §6 placement)
FILE: docs/ENGINE_OVERVIEW.md
BEFORE:
flowchart TB
  subgraph PAGE["Browser page: TypeScript (ADR 0003, ADR 0002)"]
    IN["Input and camera<br/>packages: client"]
    R["three.js WebGPURenderer, WebGL 2 fallback<br/>TSL materials, the look, VFX"]
    AU["Web Audio"]
    HUD["HUD and menus (DOM)"]
  end
  subgraph WK["Web Worker (ADR 0002 part 5, part 6)"]
    SW["sim-worker: paces the tick"]
    WASM["sim-wasm (Rust as Wasm)<br/>client endpoint: prediction, reconciliation, interpolation<br/>local server: room + sim"]
    NET["net: WebTransport, WebSocket fallback"]
  end
  subgraph SRV["Match server: native Rust (ADR 0001, ADR 0006)"]
    ROOM["server: one room per process<br/>room + sim, interest, encoding, lag compensation"]
    SUP["supervisor, lobby, metrics, match record"]
  end
  CONTENT["content crate: JSON validated, one hashed bundle (ADR 0005)"]
  SIM["sim crate: plain typed tables, integer ticks, parry3d, baked navigation (ADR 0004)"]
  IN -->|"one command per tick (posted)"| SW
  SW -->|"one transferred block per tick: header, entity sections, events"| R
  R --> AU
  R --> HUD
  SW --- WASM
  WASM --- NET
  NET <-->|"commands up; snapshots and events down (ADR 0001 protocol)"| ROOM
  ROOM --- SUP
  SIM -.->|"same code"| WASM
  SIM -.->|"same code"| ROOM
  CONTENT -.->|"read by id"| SIM
AFTER:
flowchart TB
  subgraph PAGE["Browser page: TypeScript (ADR 0003, ADR 0002)"]
    IN["Input and camera<br/>packages: client<br/>§6.2 the aim point"]
    R["three.js WebGPURenderer, WebGL 2 fallback<br/>TSL materials, the look, VFX<br/>§6.7 faction colour · §6.9 the spectacle budget"]
    AU["Web Audio"]
    HUD["HUD and menus (DOM)"]
  end
  subgraph WK["Web Worker (ADR 0002 part 5, part 6)"]
    SW["sim-worker: paces the tick"]
    WASM["sim-wasm (Rust as Wasm)<br/>client endpoint: prediction, reconciliation, interpolation<br/>local server: room + sim"]
    NET["net: WebTransport, WebSocket fallback"]
  end
  subgraph SRV["Match server: native Rust (ADR 0001, ADR 0006)"]
    ROOM["server: one room per process<br/>room + sim, interest, encoding, lag compensation<br/>§6.1 rooms and private matches"]
    SUP["supervisor, lobby, metrics, match record<br/>§6.6 telemetry · §6.8 moments stored · §6.10 kept possible"]
  end
  CONTENT["content crate: JSON validated, one hashed bundle (ADR 0005)<br/>§6.1 modes · §6.3 abilities and effects · §6.7 factions"]
  SIM["sim crate: plain typed tables, integer ticks, parry3d, baked navigation (ADR 0004)<br/>§6.2 the 3D sim · §6.3 · §6.4 vehicles · §6.5 economy · §6.8 moments"]
  IN -->|"per tick: one command, posted"| SW
  SW -->|"per tick: one transferred block: header, entity sections, events"| R
  IN -->|"per frame: camera"| R
  R -->|"per frame: interpolate, animate, draw the newest block"| R
  R -->|"per frame: events as sound"| AU
  R -->|"per frame: events as HUD feedback"| HUD
  SW --- WASM
  WASM --- NET
  NET <-->|"per tick: commands up; snapshots and events down (ADR 0001 protocol)"| ROOM
  ROOM --- SUP
  SIM -.->|"same code"| WASM
  SIM -.->|"same code"| ROOM
  CONTENT -.->|"read by id"| SIM
END

### FIX completeness-15 (the words under the diagram)
FILE: docs/ENGINE_OVERVIEW.md
BEFORE:
In words: the page posts one command per tick to the Worker and receives one block per tick back; it makes no call
into Wasm and touches no network (ADR 0002 part 5). The Worker holds the Rust sim as Wasm and owns the connection (ADR
0002 part 6). Online, the Worker talks to the room process over WebTransport, or WebSocket when that fails (ADR 0001
part 10); in local play the same Worker runs the room and the sim itself, behind the same messages (SLICE-ORDER
condition 1; ADR 0002 part 6). The sim crate and the content crate sit below everything.
AFTER:
In words: the page posts one command per tick to the Worker and receives one block per tick back; it makes no call
into Wasm and touches no network (ADR 0002 part 5). Per frame, the page interpolates, animates and draws the newest
block and plays its events as sound and HUD feedback (ADR 0003 part 10). The Worker holds the Rust sim as Wasm and
owns the connection (ADR 0002 part 6). Online, the Worker talks to the room process over WebTransport, or WebSocket
when that fails (ADR 0001 part 10); in local play the same Worker runs the room and the sim itself, behind the same
messages (SLICE-ORDER condition 1; ADR 0002 part 6). The sim crate and the content crate sit below everything. Each
part's label names the VISION_V2 §6 items it serves; the table below gives the draft and part.
END

### FIX F13 (no "captures" event: ADR 0004 part 19's words)
FILE: docs/ENGINE_OVERVIEW.md
BEFORE:
- **Down (Worker → page):** one transferred output block per tick (B2), with a header (tick, layout hash, counts),
  sections per entity kind and an event section (hits, deaths, purchases, captures, moments); the SharedArrayBuffer
  ring (B3) is measured against it in the boundary spike. The fields of remote entities online are ADR 0001's; the
  layout is ADR 0002's.
AFTER:
- **Down (Worker → page):** one transferred output block per tick (B2), with a header (tick, layout hash, counts),
  sections per entity kind and an event section (the events of ADR 0004 part 19, such as hits, deaths and kills,
  blasts, purchases, uplink cache events, tech tiers, upgrade picks and pickups, and moments); the SharedArrayBuffer
  ring (B3) is measured against it in the boundary spike. The fields of remote entities online are ADR 0001's; the
  layout is ADR 0002's.
END

### FIX verifier-synthesis row 33 (tools do not depend on everything)
FILE: docs/ENGINE_OVERVIEW.md
BEFORE:
**Dependencies point one way** (ADR 0002 part 3, each a gate check): `sim` depends on `content` only, with no
transport, no async runtime and no clock; `room` builds for Wasm with no async runtime; `protocol` uses `sim` and
`content` types; `sim-wasm` = `sim` + `room` + `protocol` + `content`, never `navbake` or `server`; `server` adds the
transport and the runtime; `client` never reaches `editor`; tools (`devtools`, `navbake`, the editor) may depend on
everything. Each crate enters with the first slice that needs it.
AFTER:
**Dependencies point one way** (ADR 0002 part 3, each a gate check): `sim` depends on `content` only, with no
transport, no async runtime and no clock; `room` builds for Wasm with no async runtime; `protocol` uses `sim` and
`content` types; `sim-wasm` = `sim` + `room` + `protocol` + `content`, never `navbake` or `server`; `server` adds the
transport and the runtime; `client` never reaches `editor`. Among the tools, `devtools` may depend on any workspace
crate and `navbake` on `sim` and `content`; the editor uses `client` code and the `navbake` and `sim` Wasm; nothing
imports `legacy/` or `spikes/`. Each crate enters with the first slice that needs it.
END

### FIX verifier-synthesis row 37 (§6.4: the leech's two primitives, and marking)
FILE: docs/ENGINE_OVERVIEW.md
BEFORE:
| 4 | Vehicles as first-class: big, multi-part, own weapons and abilities | ADR 0004 parts 10 and 14 (compound bodies, vehicles as obstacles); ADR 0001 parts 6 and 9 (parts in snapshots and interest); ADR 0003 part 6 (very large units); ADR 0005 AB-6 (the two primitives §6.4 needs: an effect removed when its source dies, and marking). `[OPEN]` seats: no feature; R2-SYNC-OQ7 stays a design question |
AFTER:
| 4 | Vehicles as first-class: big, multi-part, own weapons and abilities | ADR 0004 parts 10 and 14 (compound bodies, vehicles as obstacles); ADR 0001 parts 6 and 9 (parts in snapshots and interest); ADR 0003 part 6 (very large units); ADR 0005 AB-6 (leech: an effect removed when its source dies and a last-hit trigger that moves the victim's item to the source; marking: an effect that grants a tag). `[OPEN]` seats: no feature; R2-SYNC-OQ7 stays a design question |
END

### FIX G-SPK-15 (glossary: "48" alone)
FILE: docs/SPIKE_PLAN.md
BEFORE:
below. Where a research mark gives 48 and 54, 48 holds (MODES-3 (a)).
AFTER:
below. Where a research mark gives N = 48 and N = 54, N = 48 holds (MODES-3 (a)).
END

### FIX F4 (one MAC definition)
FILE: docs/SPIKE_PLAN.md
BEFORE:
  MV, a CX33 running the metrics stack (ADR 0006); ML, the minimum laptop, RD, the recommended desktop, and MAC, a macOS
  machine with Safari (ADR 0003, OQ-0003-1: the models are not named yet); MP, Mustafa's PC in Brave, for what he judges
  only.
AFTER:
  MV, a CX33 running the metrics stack (ADR 0006); ML, the minimum laptop, and RD, the recommended desktop (ADR 0003,
  OQ-0003-1: the models are not named yet); MAC, a macOS machine with the stable Safari on the day of the run, its
  version recorded (WebGPU from Safari 26.0: r3-rendering.md, R3-WGPU-F19, lines 171–174; WebTransport from 26.4:
  r2-netcode.md, R2-TRN-F7, lines 959–975); MP, Mustafa's PC in Brave, for what he judges only.
END

### FIX J6-order (J6 has its home in README)
FILE: docs/SPIKE_PLAN.md
BEFORE:
order below is a proposal for Mustafa and the Advisor (decision map part j, J6 (5)); `docs/adr/README.md` lists it
AFTER:
order below is a proposal for Mustafa and the Advisor (J6 (5) in `docs/adr/README.md`'s codes); `docs/adr/README.md` lists it
END

### FIX addendum (PM-0001-57 in the netcode proof test row)
FILE: docs/SPIKE_PLAN.md
BEFORE:
| 1 | The netcode proof test | L (+ M for ADR 0006's operations marks) | wave 1, first | PM-0001-1 to -56; PM-0004-67; PM-0006-11 to -27 |
AFTER:
| 1 | The netcode proof test | L (+ M for ADR 0006's operations marks) | wave 1, first | PM-0001-1 to -57; PM-0004-67; PM-0006-11 to -27 |
END

### FIX F7 (PM-0004-73 and -74 in the sim core spike row)
FILE: docs/SPIKE_PLAN.md
BEFORE:
| 4 | The sim core spike | L, the largest | wave 1 | PM-0004-1 to -61 |
AFTER:
| 4 | The sim core spike | L, the largest | wave 1 | PM-0004-1 to -61, -73, -74 |
END

### FIX F8 (item 1 proves the `wss` leg's certificate)
FILE: docs/SPIKE_PLAN.md
BEFORE:
  match-hour, the spike's own bill, logging cost, metrics, the development certificates).
AFTER:
  match-hour, the spike's own bill, logging cost, metrics, the development certificates for WebTransport, and the
  WebSocket leg's `wss` sessions on the certificate OQ-0006-5 names, PM-0006-15).
END

### FIX addendum (PM-0001-57)
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Pass marks:** PM-0001-1 to PM-0001-56 (PM-0001-39 and -40 belong to the 60 Hz margin run below; PM-0001-54 and -55
AFTER:
- **Pass marks:** PM-0001-1 to PM-0001-57 (PM-0001-39 and -40 belong to the 60 Hz margin run below; PM-0001-54 and -55
END

### FIX F8, G-SPK-106 (item 1 depends on OQ-0006-5; big vehicles and live projectiles)
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Depends on:** the drafts approved; OQ-0006-1 answered (the CCX23 and CX33 orderable in one EU location, in
  Mustafa's Hetzner account) and LG and MV beside them; ML, RD and MAC named (OQ-0003-1); a throwaway sim with movement,
  hitscan and bots as protocol clients, native and Wasm, with stand-in vehicles and projectiles at the stress scene's
  counts (PM-0001-2); the link simulator's L1 and L2 (ADR 0002's design), built here first; a DM_Tri-sized greybox with
  enterable buildings and the target course; the development certificates (ADR 0006); the Advisor's readings of
  OQ-0001-3 before the checklist; Mustafa's session in Brave; the spike's run plan under its bill mark (PM-0006-18)
  before the first VM.
AFTER:
- **Depends on:** the drafts approved; OQ-0006-1 answered (the CCX23 and CX33 orderable in one EU location, in
  Mustafa's Hetzner account) and LG and MV beside them; ML, RD and MAC named (OQ-0003-1); a throwaway sim with movement,
  hitscan and bots as protocol clients, native and Wasm, with stand-in big vehicles and live projectiles at the stress
  scene's counts (PM-0001-2); the link simulator's L1 and L2 (ADR 0002's design), built here first; a DM_Tri-sized
  greybox with enterable buildings and the target course; the development certificates for WebTransport (ADR 0006);
  for the WebSocket leg, OQ-0006-5's development and staging answer (the Advisor, from a short read, before the test
  starts), since a WebSocket cannot pin a certificate hash (ADR 0006, OQ-0006-5); the Advisor's readings of OQ-0001-3
  before the checklist; Mustafa's session in Brave; the spike's run plan under its bill mark (PM-0006-18) before the
  first VM.
END

### FIX F12/drift (R1-TICK-REC4: ADR 0001 now names it)
FILE: docs/SPIKE_PLAN.md
BEFORE:
  mark) and R2-SYNC-REC7 mark 3; R1-TICK-REC4 (through PM-0004-67: the decision map placed it here, ADR 0001's spike
  text does not name it); R2-TRN-REC5 marks 1 (the development half), 2 and 3, R6-HOST-REC2, R6-COST-REC1, R6-OBS-REC1
AFTER:
  mark) and R2-SYNC-REC7 mark 3; R1-TICK-REC4 (ADR 0004's S17, PM-0004-67, run on this
  spike's server, as ADR 0001's spike text says); R2-TRN-REC5 marks 1 (the development half), 2 and 3, R6-HOST-REC2, R6-COST-REC1, R6-OBS-REC1
END

### FIX F14 (R5-TEST-OQ2, as ADR 0001 now lists it)
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **It decides:** R2-TRN-OQ1, OQ2, OQ3, OQ5, OQ8; R2-BUD-OQ3, OQ7, OQ9, OQ10; R2-SYNC-OQ3 (through OQ-0001-6), OQ5 (if
  vehicles move), OQ12; R2-ROOM-OQ1; R2-XU-OQ2, OQ4, OQ5, OQ6, OQ7, OQ15; OQ-0001-6, OQ-0001-7; R6-COST-OQ1; it feeds
  OQ-0006-2 and gives Mustafa and the Advisor the numbers for OQ-0001-5.
AFTER:
- **It decides:** R2-TRN-OQ1, OQ2, OQ3, OQ5, OQ8; R2-BUD-OQ3, OQ7, OQ9, OQ10; R2-SYNC-OQ3 (through OQ-0001-6), OQ5 (if
  vehicles move), OQ12; R2-ROOM-OQ1; R2-XU-OQ2, OQ4, OQ5, OQ6, OQ7, OQ15; OQ-0001-6, OQ-0001-7; R5-TEST-OQ2 (the
  WebTransport crates' interop); R6-COST-OQ1; it feeds OQ-0006-2 and gives Mustafa and the Advisor the numbers for
  OQ-0001-5.
END

### FIX F1 (no "hot reload into the Worker"; line 160 names PM-0002-31)
FILE: docs/SPIKE_PLAN.md
BEFORE:
  the client + local server skeleton; one Wasm instance or two in a local match, OQ-0002-3), parts 2, 4 and 7 (Wasm
  size, tools, hot reload into the Worker), part 11 (the editor from `file://`, OQ-0002-1); ADR 0004 part 5 (S18: the
AFTER:
  the client + local server skeleton; one Wasm instance or two in a local match, OQ-0002-3), parts 2, 4 and 7 (Wasm
  size and tools), part 11 (the editor from `file://`, OQ-0002-1); ADR 0004 part 5 (S18: the
END

### FIX F4 (MAC, item 3)
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Depends on:** the drafts approved; ML named and on hand (OQ-0003-1), MAC with the latest Safari, and displays at
AFTER:
- **Depends on:** the drafts approved; ML named and on hand (OQ-0003-1), MAC, and displays at
END

### FIX addendum (a) (R1-RR-OQ6: ADR 0002 answers it; the spike reports the copy time)
FILE: docs/SPIKE_PLAN.md
BEFORE:
  R1-XU-OQ3, OQ4; R1-TICK-OQ2; R1-DATA-OQ8; R1-RR-OQ5 (with the sim core spike), OQ6; R5-LAYOUT-OQ4; R5-TSLINT-OQ3, OQ4,
  OQ6; R5-TEST-OQ1; R6-OBS-OQ3, OQ4 (optional); critic ranks 4 (the implementation side) and 37; it gives OQ-0002-1,
AFTER:
  R1-XU-OQ3, OQ4; R1-TICK-OQ2; R1-DATA-OQ8; R1-RR-OQ5 (with the sim core spike); R5-LAYOUT-OQ4; R5-TSLINT-OQ3, OQ4,
  OQ6; R5-TEST-OQ1; R6-OBS-OQ3, OQ4 (optional); critic ranks 4 (the implementation side) and 37; it reports the copy time for
  R1-RR-OQ6 (PM-0002-10; ADR 0002 answers R1-RR-OQ6, proposed, for rust-reviewer to confirm); it gives OQ-0002-1,
END

### FIX verifier-synthesis finding 9 (the seven targets named)
FILE: docs/SPIKE_PLAN.md
BEFORE:
  cross-target identity by per-tick state hashes on native, Node, three browser engines and real Safari, with a suite
AFTER:
  cross-target identity by per-tick state hashes on seven targets (native, Node, three browser engines, real Safari on
  MAC, and Chrome and Firefox on the minimum laptop's Windows: PM-0004-19), with a suite
END

### FIX F7 (PM-0004-73 and -74, item 4)
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Pass marks:** PM-0004-1 to PM-0004-61 (PM-0004-8 and -28 in the boundary spike's Worker; PM-0004-61 with the netcode
  proof test's server).
AFTER:
- **Pass marks:** PM-0004-1 to PM-0004-61, PM-0004-73 and PM-0004-74 (PM-0004-8 and -28 in the boundary spike's
  Worker; PM-0004-61 with the netcode proof test's server).
END

### FIX F4 (MAC, item 4)
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Depends on:** the drafts approved; RS orderable (OQ-0006-1), ML and RD named (OQ-0003-1), MAC with Safari; the
AFTER:
- **Depends on:** the drafts approved; RS orderable (OQ-0006-1), ML and RD named (OQ-0003-1), MAC; the
END

### FIX F7 (the two marks now have ADR 0004 rows)
FILE: docs/SPIKE_PLAN.md
BEFORE:
  only if tiling is needed); R5-TEST-REC2's suite. **Placed here by this plan** (the decision map's part g put them
  here; no draft's table names them; ADR 0001 and ADR 0002 adopt the recommendations): R2-SYNC-REC6 mark 1 (throws
  bit-identical on Wasm and native) and R5-LAYOUT-REC4 (one feature set for `sim` on every target); the foundation later
AFTER:
  only if tiling is needed); R5-TEST-REC2's suite; R2-SYNC-REC6 mark 1 (throws bit-identical on Wasm and native: ADR
  0004's S21, PM-0004-73; the id is ADR 0001's) and R5-LAYOUT-REC4 (one feature set for `sim` on every target: S22,
  PM-0004-74; the id is ADR 0002's); the foundation later
END

### FIX verifier-synthesis finding 8 (OQ-0003-6 named the same in both places)
FILE: docs/SPIKE_PLAN.md
BEFORE:
  makes the calls ADR 0003 leaves to him (OQ-0003-2, -3, -5, -8, -12, -15; the ink family). The scripts judge the
  frame-time and GPU marks on ML and RD.
AFTER:
  makes the calls ADR 0003 leaves to him (OQ-0003-2, -3, -5, -8, -12, -15; the ink family); OQ-0003-6 (the roof fade)
  is the test's to decide, with his look among what decides it. The scripts judge the frame-time and GPU marks on ML
  and RD.
END

### FIX F4 (MAC, item 5)
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Depends on:** the drafts approved; ML and RD named and on hand (OQ-0003-1), MAC with Safari; a few authored test
AFTER:
- **Depends on:** the drafts approved; ML and RD named and on hand (OQ-0003-1), MAC; a few authored test
END

### FIX G-SPK-274 (glossary: "Cloudflare" alone for the client host)
FILE: docs/SPIKE_PLAN.md
BEFORE:
  check before the preview step; Mustafa's Cloudflare token and branch-protection setting.
AFTER:
  check before the preview step; Mustafa's client-host token (ADR 0006 part 1) and branch-protection setting.
END

### FIX F4 (MAC, item 7)
FILE: docs/SPIKE_PLAN.md
BEFORE:
  LOOP_V2 §5's T1 abilities; RS and ML; MAC with Safari (PM-0005-2); the boundary spike's Worker and build settings;
AFTER:
  LOOP_V2 §5's T1 abilities; RS and ML; MAC (PM-0005-2); the boundary spike's Worker and build settings;
END

### FIX G-SPK-310 (glossary: "characters" for soldiers)
FILE: docs/SPIKE_PLAN.md
BEFORE:
  with the techniques the look proof test kept, on both machines: characters at N = 48 on screen, the four very large
AFTER:
  with the techniques the look proof test kept, on both machines: soldiers at N = 48 on screen, the four very large
END

### FIX F12 (no "part g": the map leaves)
FILE: docs/SPIKE_PLAN.md
BEFORE:
  (ADR 0003 places REC1 and REC4 here, where the decision map's part g had them in the look proof test; the draft
  holds); R3-AUDIO-REC1 to REC4; R3-WGPU-REC2 (later). **It decides:** R3-CHAR-OQ1, OQ2, OQ4 (OQ6 only if the draw-call
AFTER:
  (ADR 0003 places REC1 and REC4
  here); R3-AUDIO-REC1 to REC4; R3-WGPU-REC2 (later). **It decides:** R3-CHAR-OQ1, OQ2, OQ4 (OQ6 only if the draw-call
END

### FIX F12 (part h: README's research gaps)
FILE: docs/SPIKE_PLAN.md
BEFORE:
  (decision map part h, G1), so ADR 0005 takes LOOK_V2 §8's flow as given and lists the rest as "not researched"
AFTER:
  (gap G1 in `docs/adr/README.md`), so ADR 0005 takes LOOK_V2 §8's flow as given and lists the rest as "not researched"
END

### FIX F12 (part h, item 9)
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Research spikes it subsumes:** none in R1–R6 (decision map part h); the asset-pipeline research's, once written.
AFTER:
- **Research spikes it subsumes:** none in R1–R6 (gap G1 in `docs/adr/README.md`); the asset-pipeline research's, once written.
END

### FIX F12 (LE-N has its home in README)
FILE: docs/SPIKE_PLAN.md
BEFORE:
  pre-acceptance spike: it needs abilities in the engine (decision map LE-N).
AFTER:
  pre-acceptance spike: it needs abilities in the engine (LE-N in `docs/adr/README.md`).
END

### FIX F13 (the full name outside ADR 0001)
FILE: docs/SPIKE_PLAN.md
BEFORE:
  shot stamps if the proof test says they are needed (OQ-0001-7).
AFTER:
  shot stamps if the netcode proof test says they are needed (OQ-0001-7).
END

### FIX F12 (no "part g", later milestones)
FILE: docs/SPIKE_PLAN.md
BEFORE:
From the decision map's part g, as the drafts keep them: R2-ROOM-REC1, REC3–REC8 and R2-INT-REC2, REC3, REC5–REC7
AFTER:
As the drafts keep them: R2-ROOM-REC1, REC3–REC8 and R2-INT-REC2, REC3, REC5–REC7
END

### FIX F9 (what the line numbers are)
FILE: docs/adr/README.md
BEFORE:
The drafts replace the legacy entries ADR-001, ADR-002 and ADR-003 (`docs/STATUS.md`, "ADR-001..003 below"; see the
loose-ends register). Line numbers below are the drafts' at assembly (2026-11-01); the fix round may move them.
AFTER:
The drafts replace the legacy entries ADR-001, ADR-002 and ADR-003 (`docs/STATUS.md`, "ADR-001..003 below"; see the
loose-ends register). Line numbers below are the drafts' at commit 9a03999, after the fix rounds; a later edit may move
them, so each row also names its OQ id or part.
END

### FIX drift (a), addendum (counts at 9a03999)
FILE: docs/adr/README.md
BEFORE:
| [0001](0001-multiplayer-netcode.md) | Multiplayer and netcode | netcode-architect; pass marks: test-engineer | Proposed | the netcode proof test; the 60 Hz margin run | 13 (OQ-0001-1 to -13) |
| [0002](0002-code-structure-build-toolchain.md) | Code structure, build and toolchain | engine-architect; pass marks: test-engineer | Proposed | the Rust ↔ TypeScript boundary spike; the foundation's acceptance checks (not a spike) | 7 (OQ-0002-1 to -7) |
| [0003](0003-rendering.md) | Rendering and presentation | graphics-engineer; pass marks: test-engineer | Proposed | the look proof test; the render budget bench | 15 (OQ-0003-1 to -15) |
| [0004](0004-simulation-core.md) | The simulation core | rust-reviewer; pass marks: test-engineer | Proposed | the sim core spike; shares in the netcode proof test, the boundary spike and the content and ability spike | 7 (OQ-0004-1 to -7) |
| [0005](0005-content-abilities-assets.md) | Content data, abilities and assets | engine-architect (frame, content data, asset pipeline); game-designer (ability and effect system); graphics-engineer (the asset pipeline's GPU rules); pass marks: test-engineer | Proposed | the content and ability spike; a share of the look proof test; the asset spike (after the asset-pipeline research); the Feel Lab test | 34 (CD1–CD8, AB1–AB13, AP1–AP3, AS1–AS6, OQ-0005-1 to -4) |
| [0006](0006-infrastructure-operations.md) | Infrastructure and operations | infra-engineer; pass marks: test-engineer | Proposed | the preview hosting check; operations marks in the netcode proof test; an optional share of the boundary spike | 12 (OQ-0006-1 to -12) |

Total: 88 open questions. None of the drafts is signed off; no spike starts before Mustafa and the Advisor approve them
(the drafts brief, §4).
AFTER:
| [0001](0001-multiplayer-netcode.md) | Multiplayer and netcode | netcode-architect; pass marks: test-engineer | Proposed | the netcode proof test; the 60 Hz margin run | 14 (OQ-0001-1 to -14) |
| [0002](0002-code-structure-build-toolchain.md) | Code structure, build and toolchain | engine-architect; pass marks: test-engineer | Proposed | the Rust ↔ TypeScript boundary spike; the foundation's acceptance checks (not a spike) | 8 (OQ-0002-1 to -8) |
| [0003](0003-rendering.md) | Rendering and presentation | graphics-engineer; pass marks: test-engineer | Proposed | the look proof test; the render budget bench | 15 (OQ-0003-1 to -15) |
| [0004](0004-simulation-core.md) | The simulation core | rust-reviewer; pass marks: test-engineer | Proposed | the sim core spike; shares in the netcode proof test, the boundary spike and the content and ability spike | 7 numbered (OQ-0004-1 to -7), 5 open: -5 and -6 moved to OQ-0005-AB9 and AB10 |
| [0005](0005-content-abilities-assets.md) | Content data, abilities and assets | engine-architect (frame, content data, asset pipeline); game-designer (ability and effect system); graphics-engineer (the asset pipeline's GPU rules); pass marks: test-engineer | Proposed | the content and ability spike; a share of the look proof test; the asset spike (after the asset-pipeline research); the Feel Lab test | 34 (CD1–CD8, AB1–AB13, AP1–AP3, AS1–AS6, OQ-0005-1 to -4) |
| [0006](0006-infrastructure-operations.md) | Infrastructure and operations | infra-engineer; pass marks: test-engineer | Proposed | the preview hosting check; operations marks in the netcode proof test; an optional share of the boundary spike | 12 (OQ-0006-1 to -12) |

Total: 90 numbered open questions, 88 of them open (OQ-0004-5 and -6 point to ADR 0005). Pass marks: 57, 76, 43, 74, 46
and 31 rows (327). None of the drafts is signed off; no spike starts before Mustafa and the Advisor approve them (the
drafts brief, §4).
END

### FIX F2, F8, F9, F14, drift (a)–(c), addendum (the decisions table re-pinned at 9a03999)
FILE: docs/adr/README.md
BEFORE:
| TICK-60-HEADROOM (OQ-0001-1) | Is the 60 Hz margin measured against the 60 Hz server tick budget (8.33 ms), so 60 Hz needs a p99 server tick time of at most 6.25 ms plus every 60 Hz research mark? | Mustafa and the Advisor (confirm at the stop) | [ADR 0001](0001-multiplayer-netcode.md) part 2, line 840; [GLOSSARY §8](../research/GLOSSARY.md#the-60-hz-margin) |
| TICK-60-SETUP (OQ-0001-2) | How does the stress scene's sim load reach the 60 Hz margin run: (a) the sim core spike's sim inside the proof-test server (recommended), (b) a stand-in replaying measured step times, (c) wait until the real sim runs the stress scene? | the Advisor | [ADR 0001](0001-multiplayer-netcode.md) line 846; SPIKE_PLAN item 2 |
| "100 ms" reading (OQ-0001-3) | Does the netcode proof test's "100 ms of simulated latency" mean profile P2 (100 ms round trip, with P2's jitter and loss), or 100 ms each way? | the Advisor (who wrote the definition) | [ADR 0001](0001-multiplayer-netcode.md) line 856 |
| ABL-PREDICT (OQ-0001-4) | Does the client predict the player's own abilities? Decided after the Feel Lab test on the Molotov and the Shield. | Mustafa | [ADR 0001](0001-multiplayer-netcode.md) line 861; [ADR 0005](0005-content-abilities-assets.md) AB-9; SPIKE_PLAN item 10 |
| ASSET-RESEARCH (timing) | The asset-pipeline research runs before the asset spike; should it run earlier (for example beside the first spikes) so art production can start sooner? | Mustafa and the Advisor | [ADR 0005](0005-content-abilities-assets.md) line 76 and the asset pipeline part; SPIKE_PLAN, "The asset-pipeline research" |
| WEAPON-UNLOCKS (OQ-0005-AB5) | How does an account-unlocked weapon reach a match (LOOP_V2 §8, VISION_V2 §1), when buying offers no weapon choice and other weapons come only from drops? | Mustafa, with the Advisor | [ADR 0005](0005-content-abilities-assets.md) line 1148 |
| UPGRADE-SCOPE: carry slots (OQ-0005-AB1) | How many weapons can a player carry besides the class weapon, and what happens to the one in hand on a pickup? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1095 |
| UPGRADE-SCOPE: looted ammo (OQ-0005-AB2) | What does a looted gun come with, and can it be refilled? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1107 |
| UPGRADE-SCOPE: levelling (OQ-0005-AB3) | What earns a level, how fast, and does a track the player switches to catch up? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1115 |
| Reference machines (OQ-0003-1) | Which laptop (the minimum) and which desktop (the recommended) are bought or borrowed as the two reference machines? | Mustafa and the Advisor (money and availability) | [ADR 0003](0003-rendering.md) line 706 |
| Spike order (decision map J6 (5)) | Only the netcode proof test's place is settled (before the foundation); is SPIKE_PLAN's proposed order of the rest accepted? | Mustafa and the Advisor | [SPIKE_PLAN](../SPIKE_PLAN.md), "The spikes, in the proposed order" |
| Trace reading (LE-G; R2-XU-OQ18) | Does a VISION_V2 §3 or §4 target count as a feature's need like LOOP_V2 does (written "need: VISION_V2 §4 only" and listed below)? The session read it so on 2026-10-31. | the Advisor | [ADR 0001](0001-multiplayer-netcode.md) Appendix A (R2-XU-OQ18); "Needs outside VISION_V2 §6" below |
| The design questions | The 38 research questions below that are pure design matters | Mustafa, or a later design session | "Design questions" below |
| OQ-0001-5 | The rewind cap: 200 or 250 ms, and the victim's side? | Mustafa and the Advisor | [ADR 0001](0001-multiplayer-netcode.md) line 867 |
| OQ-0001-7 | Do shots carry their moment inside a tick at 30 Hz? | the netcode proof test, then Mustafa in the Feel Lab if it fails or feels late | [ADR 0001](0001-multiplayer-netcode.md) line 881 |
| OQ-0001-8 | How far does a new projectile move forward on spawn? | Mustafa | [ADR 0001](0001-multiplayer-netcode.md) line 886 |
| OQ-0001-9 | What else does lag compensation cover? | the netcode proof test for cover; for melee, a design call by Mustafa with the game-designer | [ADR 0001](0001-multiplayer-netcode.md) line 890 |
| OQ-0001-10 | Are weapon pickups predicted? | Mustafa | [ADR 0001](0001-multiplayer-netcode.md) line 896 |
| OQ-0001-11 | Room codes: length, alphabet and access. | Mustafa (what a host may set, R2-ROOM-OQ5) with the security review's options | [ADR 0001](0001-multiplayer-netcode.md) line 900 |
| OQ-0001-12 | Do far sound events reveal too much? | Mustafa with the game-designer | [ADR 0001](0001-multiplayer-netcode.md) line 905 |
| OQ-0002-1 | Does the v2 editor still open by double-clicking its HTML file? | the boundary spike (does it load?), then Mustafa if EDITOR_V2 §5's rule must change | [ADR 0002](0002-code-structure-build-toolchain.md) line 800 |
| OQ-0002-4 | Does production ever turn on cross-origin isolation, and can a later login popup live with it? | the boundary spike's results, then this record; for embeds and the login flow under isolation, Mustafa with ADR 0006 (R1-BND-OQ5, only if is | [ADR 0002](0002-code-structure-build-toolchain.md) line 836 |
| OQ-0002-5 | Does real Safari run on pull requests, and does the end-to-end suite need it? | infra-engineer's reading of the macOS runner's time and cost (ADR 0006 for costs; Mustafa for any paid plan), then this record | [ADR 0002](0002-code-structure-build-toolchain.md) line 848 |
| OQ-0002-6 | Which jump reach do the golden tests pin? | the game-designer with Mustafa, from the loop slice on (a rule question) | [ADR 0002](0002-code-structure-build-toolchain.md) line 859 |
| OQ-0003-2 | The ink's width, its inner lines and its weight on buildings. | Mustafa, in the look proof test, with its measured cost; the family in part 5.b follows his pick | [ADR 0003](0003-rendering.md) line 714 |
| OQ-0003-3 | Which colour output: authored colour as shown, or a tone-mapped output? | Mustafa, in the look proof test, side by side with legacy | [ADR 0003](0003-rendering.md) line 722 |
| OQ-0003-4 | Is one sun shadow map enough, or are cascades or another setup needed? | the look proof test (softness, Mustafa) and the render budget bench (the shadow row) | [ADR 0003](0003-rendering.md) line 728 |
| OQ-0003-5 | How are "warm sun with cool shadows" and moods per map lit? | Mustafa, in the look proof test, with each option's GPU time | [ADR 0003](0003-rendering.md) line 734 |
| OQ-0003-8 | How is ink line boil drawn? | Mustafa, in the look proof test | [ADR 0003](0003-rendering.md) line 753 |
| OQ-0003-9 | How are vegetation and the lush countryside drawn, and in which tree style? | Mustafa (the style, a later design session); the render budget bench (the technique) | [ADR 0003](0003-rendering.md) line 758 |
| OQ-0003-10 | Does the frame rate follow a 120 or 144 Hz screen, or is it capped? | Mustafa (feel), on the render budget bench's numbers | [ADR 0003](0003-rendering.md) line 765 |
| OQ-0003-11 | Should v2 one day drop the WebGL 2 fallback, and how many players use it? | Mustafa | [ADR 0003](0003-rendering.md) line 771 |
| OQ-0003-12 | Is the pixel ratio capped at 1 on Low for HiDPI laptops? | Mustafa, in the look proof test | [ADR 0003](0003-rendering.md) line 777 |
| OQ-0003-13 | Where does the audio listener sit under the v2 camera? | the camera framing first (R2-INT-OQ2, a design question), then Mustafa's listening check | [ADR 0003](0003-rendering.md) line 782 |
| OQ-0003-15 | The legacy look touches LOOK_V2 does not name: keep, change or drop each? | Mustafa, in the look proof test | [ADR 0003](0003-rendering.md) line 793 |
| OQ-0004-1 | How is bot behaviour structured? | a bot research unit, or the slice 2 plan, with the game-designer; Mustafa for the behaviour set | [ADR 0004](0004-simulation-core.md) line 742 |
| OQ-0004-4 | What moves a building through its damage stages? | Mustafa, with the game-designer (rule 6: no rule beyond LOOK_V2 is designed here) | [ADR 0004](0004-simulation-core.md) line 765 |
| OQ-0004-5 | Which weapons drop, and who may take them? | Mustafa with the game-designer; ADR 0005 holds the answers as data | [ADR 0004](0004-simulation-core.md) line 773 |
| OQ-0004-6 | What do bots and slot takeovers do with player records? | Mustafa with the game-designer | [ADR 0004](0004-simulation-core.md) line 784 |
| OQ-0005-CD1 | Do hand-edited content files need comments? | Mustafa, after a short trial in the content and ability spike | [ADR 0005](0005-content-abilities-assets.md) line 583 |
| OQ-0005-CD2 | What load time and Wasm size may content cost, and is JSON fast and small enough? | the content and ability spike's measurement, then the Advisor sets the budgets | [ADR 0005](0005-content-abilities-assets.md) line 589 |
| OQ-0005-CD3 | May production ever change numbers in the middle of a match? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 598 |
| OQ-0005-CD4 | In development, do simulation numbers apply at the next match, or inside a running match? | Mustafa and the Advisor | [ADR 0005](0005-content-abilities-assets.md) line 606 |
| OQ-0005-CD5 | Faction variants: full definitions that reference shared ones, or overrides of a base? | the faction design session (VISION_V2 §7, faction asymmetry; Mustafa) | [ADR 0005](0005-content-abilities-assets.md) line 618 |
| OQ-0005-CD6 | How are moment rules written? | the moment design session (Mustafa with the game-designer, with R6-TEL-OQ3's thresholds), then ADR 0004 | [ADR 0005](0005-content-abilities-assets.md) line 624 |
| OQ-0005-CD7 | What is the Feel Lab tool? | a later design session with Mustafa, before the feel pass (LOOP_V2 §11 leaves the feel pass out of the greybox v0); ADR 0003 for its screen | [ADR 0005](0005-content-abilities-assets.md) line 632 |
| OQ-0005-AB4 | What is one track: which definitions share one? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1135 |
| OQ-0005-AB6 | How do the player's tracks sit beside the team's tech tiers? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1163 |
| OQ-0005-AB7 | May a pick add raw power (more damage, more health), or only change how a unit or weapon plays? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1171 |
| OQ-0005-AB8 | The offer: when does it appear, and when can the player answer it? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1180 |
| OQ-0005-AB9 | Which drop and pickup rules does UPGRADE-SCOPE leave unstated? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1188 |
| OQ-0005-AB10 | Do bots have tracks and make picks, and what does a human get when taking a bot's slot? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1197 |
| OQ-0005-AB11 | Does the Riot Shield answer the Banzai blade, and is the Peacekeeper's side of the triangle strong enough before Tech 1? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1206 |
| OQ-0005-AB12 | Is tranek's write-up accepted as the GAS reference, or does a verifier read Epic's documentation first? | the Advisor | [ADR 0005](0005-content-abilities-assets.md) line 1217 |
| OQ-0005-AB13 | When does scripting (option A3) come back? | the Advisor (the source) | [ADR 0005](0005-content-abilities-assets.md) line 1226 |
| OQ-0005-AP2 | Which asset families become authored models, and which stay procedural builders? | Mustafa, in the look proof test (how each looks, beside legacy screenshots, LOOK_V2 §9), with the asset-pipeline research's cost and tooling | [ADR 0005](0005-content-abilities-assets.md) line 1481 |
| OQ-0005-AP3 | May an AI-generated draft model ever ship? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1489 |
| OQ-0005-AS2 | Which tool builds the LOD levels, and how many levels? | the asset-pipeline research, then the asset spike and Mustafa (silhouettes kept) | [ADR 0005](0005-content-abilities-assets.md) line 1624 |
| OQ-0005-1 | Do presentation references sit on the simulation definitions, or in client definitions keyed by the same id? | the Advisor, at the stop review, on the text the authors write in the fix round (engine-architect for AP-2, AP-3, CD-6 and CD-13; the game-d | [ADR 0005](0005-content-abilities-assets.md) line 1833 |
| OQ-0005-2 | Is a weapon definition the same thing as a weapon type? | Mustafa answers OQ-0005-AB4; the content data section's author aligns CD-6's wording in the fix round, so that it leaves AB4 open. Research | [ADR 0005](0005-content-abilities-assets.md) line 1852 |
| OQ-0005-3 | What does the content hash cover? | the Advisor, with the infra-engineer, whose ADR 0006 owns the analysis (R6-TEL-REC3) | [ADR 0005](0005-content-abilities-assets.md) line 1864 |
| OQ-0005-4 | What do the Character Lab and the Anim Lab work on once soldiers are authored models? | Mustafa with the Advisor (EDITOR_V2 is theirs) | [ADR 0005](0005-content-abilities-assets.md) line 1876 |
| OQ-0006-1 | Can Mustafa order Hetzner's CCX23 and CX33 in an EU location? | Mustafa (a console check in his own account; agents create no account), then a research read if (b) | [ADR 0006](0006-infrastructure-operations.md) line 598 |
| OQ-0006-2 | Which EU location: Nuremberg, Helsinki, or Falkenstein if offered? | a measurement (the netcode proof test's setup or a small check before it), confirmed by Mustafa | [ADR 0006](0006-infrastructure-operations.md) line 608 |
| OQ-0006-3 | Where does the lobby run, and what stores guest ids and the room directory? | the Advisor, from a short lobby read done with the security baseline (its key storage is item B5); Mustafa for any paid service. ADR 0001 ke | [ADR 0006](0006-infrastructure-operations.md) line 615 |
| OQ-0006-4 | Once the access wall is on, does it cover every preview, or only multiplayer previews and previews with unreleased art? | Mustafa (convenience against one more project to run) | [ADR 0006](0006-infrastructure-operations.md) line 626 |
| OQ-0006-6 | The security baseline: what it decides for each waiting item. | security-auditor, in its own fresh session (BASELINE-WHEN (b)); the Advisor and Mustafa for the policy calls | [ADR 0006](0006-infrastructure-operations.md) line 647 |
| OQ-0006-7 | Will any CI runner be paid for: GPU, macOS, self-hosted reference machines, or a cache above 10 GB? | Mustafa for any paid plan; ADR 0002 says which jobs need one; the security baseline weighs (c) | [ADR 0006](0006-infrastructure-operations.md) line 658 |
| OQ-0006-8 | What do Sentry and Grafana really cost, and how do logs and metrics reach Grafana Cloud? | Mustafa, with a read of sentry.io/pricing before he creates the Sentry organization; a read of Grafana's ingestion docs if M2 is wanted | [ADR 0006](0006-infrastructure-operations.md) line 668 |
| OQ-0006-9 | What does EU data-protection law require of us, and how long are match records kept? | Mustafa | [ADR 0006](0006-infrastructure-operations.md) line 677 |
| OQ-0006-10 | How many players, and when; which VAT and currency; is included traffic per VM, pooled or prorated? | Mustafa (his plans, his billing address, the first invoice) | [ADR 0006](0006-infrastructure-operations.md) line 686 |
| OQ-0006-12 | If production isolation is ever adopted: embeds, cross-origin popups, CORP and the login flow. | Mustafa with this record, only if production isolation is adopted (R1-BND-REC2 mark 5's test) | [ADR 0006](0006-infrastructure-operations.md) line 701 |
AFTER:
| TICK-60-HEADROOM (OQ-0001-1) | Is the 60 Hz margin measured against the 60 Hz server tick budget (8.33 ms), so 60 Hz needs a p99 server tick time of at most 6.25 ms plus every 60 Hz research mark? | Mustafa and the Advisor (confirm at the stop) | [ADR 0001](0001-multiplayer-netcode.md) part 2, line 852; [GLOSSARY §8](../research/GLOSSARY.md#the-60-hz-margin) |
| TICK-60-SETUP (OQ-0001-2) | How does the stress scene's sim load reach the 60 Hz margin run: (a) the sim core spike's sim inside the proof-test server (recommended), (b) a stand-in replaying measured step times, (c) wait until the real sim runs the stress scene? | the Advisor | [ADR 0001](0001-multiplayer-netcode.md) line 859; SPIKE_PLAN item 2 |
| "100 ms" reading (OQ-0001-3) | Does the netcode proof test's "100 ms of simulated latency" mean profile P2 (100 ms round trip, with P2's jitter and loss), or 100 ms each way? | the Advisor (who wrote the definition) | [ADR 0001](0001-multiplayer-netcode.md) line 869 |
| ABL-PREDICT (OQ-0001-4) | Does the client predict the player's own abilities? Decided after the Feel Lab test on the Molotov and the Shield. | Mustafa | [ADR 0001](0001-multiplayer-netcode.md) line 874; [ADR 0005](0005-content-abilities-assets.md) AB-9; SPIKE_PLAN item 10 |
| ASSET-RESEARCH (timing) | The asset-pipeline research runs before the asset spike; should it run earlier (for example beside the first spikes) so art production can start sooner? | Mustafa and the Advisor | [ADR 0005](0005-content-abilities-assets.md) line 76 and the asset pipeline part; SPIKE_PLAN, "The asset-pipeline research" |
| WEAPON-UNLOCKS (OQ-0005-AB5) | How does an account-unlocked weapon reach a match (LOOP_V2 §8, VISION_V2 §1), when buying offers no weapon choice and other weapons come only from drops? | Mustafa, with the Advisor (LOOP_V2 §8 is theirs) | [ADR 0005](0005-content-abilities-assets.md) line 1158 |
| UPGRADE-SCOPE: carry slots (OQ-0005-AB1) | How many weapons can a player carry besides the class weapon, and what happens to the one in hand on a pickup? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1105 |
| UPGRADE-SCOPE: looted ammo (OQ-0005-AB2) | What does a looted gun come with, and can it be refilled? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1117 |
| UPGRADE-SCOPE: levelling (OQ-0005-AB3) | What earns a level, how fast, and does a track the player switches to catch up? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1125 |
| Reference machines (OQ-0003-1) | Which laptop (the minimum) and which desktop (the recommended) are bought or borrowed as the two reference machines? | Mustafa and the Advisor (money and availability) | [ADR 0003](0003-rendering.md) line 723 |
| Spike order (J6 (5)) | Only the netcode proof test's place is settled (before the foundation); is SPIKE_PLAN's proposed order of the rest accepted? | Mustafa and the Advisor | [SPIKE_PLAN](../SPIKE_PLAN.md), "The spikes, in the proposed order" |
| Trace reading (OQ-0001-14; LE-G, R2-XU-OQ18) | Is a VISION_V2 §3 or §4 target trace enough for an engine feature (written "need: VISION_V2 §4 only" and listed below)? The session read it so on 2026-10-31. | the Advisor, at the stop | [ADR 0001](0001-multiplayer-netcode.md) line 927; "Needs outside VISION_V2 §6" below |
| Structure without a research option (OQ-0002-8) | May a record propose a structure no research Options section offers, when a settled input forces it and the research leaves the question to that record (the `room` and `navbake` crates)? | the Advisor, at the stop | [ADR 0002](0002-code-structure-build-toolchain.md) line 897 |
| The `wss` certificate on development and staging (OQ-0006-5) | Which certificate does the WebSocket fallback present on development and staging servers, since a WebSocket cannot pin a certificate hash? The netcode proof test's WebSocket leg needs the answer first. | the Advisor, from a short read, before the netcode proof test starts (production issuance: security-auditor, in the security baseline) | [ADR 0006](0006-infrastructure-operations.md) line 655; SPIKE_PLAN item 1 |
| Fragments as damage | Do grenade fragments deal damage per fragment? LOOK_V2 §3 and §7 bring fragmentation back as a visible effect, which ADR 0003 draws from the blast event; any rule beyond that is a design question (no research id). | Mustafa with the game-designer | [ADR 0004](0004-simulation-core.md) "Open questions", closing paragraph; [ADR 0003](0003-rendering.md) part 5.k, OQ-0003-7 and the look checklist step 24 |
| The design questions | The 38 research questions below that are pure design matters | Mustafa, or a later design session | "Design questions" below |
| OQ-0001-5 | The rewind cap: 200 or 250 ms, and the victim's side? | Mustafa and the Advisor | [ADR 0001](0001-multiplayer-netcode.md) line 880 |
| OQ-0001-7 | Do shots carry their moment inside a tick at 30 Hz? | the netcode proof test, then Mustafa in the Feel Lab if it fails or feels late | [ADR 0001](0001-multiplayer-netcode.md) line 894 |
| OQ-0001-8 | How far does a new projectile move forward on spawn? | Mustafa | [ADR 0001](0001-multiplayer-netcode.md) line 899 |
| OQ-0001-9 | What else does lag compensation cover? | the netcode proof test for cover; for melee, a design call by Mustafa with the game-designer | [ADR 0001](0001-multiplayer-netcode.md) line 903 |
| OQ-0001-10 | Are weapon pickups predicted? | Mustafa | [ADR 0001](0001-multiplayer-netcode.md) line 909 |
| OQ-0001-11 | Room codes: length, alphabet and access. | Mustafa (what a host may set, R2-ROOM-OQ5) with the security review's options | [ADR 0001](0001-multiplayer-netcode.md) line 913 |
| OQ-0001-12 | Do far sound events reveal too much? | Mustafa with the game-designer | [ADR 0001](0001-multiplayer-netcode.md) line 918 |
| OQ-0002-1 | Does the v2 editor still open by double-clicking its HTML file? | the boundary spike (does it load?), then Mustafa if EDITOR_V2 §5's rule must change | [ADR 0002](0002-code-structure-build-toolchain.md) line 820 |
| OQ-0002-4 | Does production ever turn on cross-origin isolation, and can a later login popup live with it? | the boundary spike's results, then this record; for embeds and the login flow under isolation, Mustafa with ADR 0006 (R1-BND-OQ5, only if isolation is adopted) | [ADR 0002](0002-code-structure-build-toolchain.md) line 856 |
| OQ-0002-5 | Does real Safari run on pull requests, and does the end-to-end suite need it? | infra-engineer's reading of the macOS runner's time and cost (ADR 0006 for costs; Mustafa for any paid plan), then this record | [ADR 0002](0002-code-structure-build-toolchain.md) line 868 |
| OQ-0002-6 | Which jump reach do the golden tests pin? | the game-designer with Mustafa, from the loop slice on (a rule question) | [ADR 0002](0002-code-structure-build-toolchain.md) line 879 |
| OQ-0003-2 | The ink's width, its inner lines and its weight on buildings. | Mustafa, in the look proof test, with its measured cost; the family in part 5.b follows his pick | [ADR 0003](0003-rendering.md) line 731 |
| OQ-0003-3 | Which colour output: authored colour as shown, or a tone-mapped output? | Mustafa, in the look proof test, side by side with legacy | [ADR 0003](0003-rendering.md) line 740 |
| OQ-0003-4 | Is one sun shadow map enough, or are cascades or another setup needed? | the look proof test (softness, Mustafa) and the render budget bench (the shadow row) | [ADR 0003](0003-rendering.md) line 746 |
| OQ-0003-5 | How are "warm sun with cool shadows" and moods per map lit? | Mustafa, in the look proof test, with each option's GPU time | [ADR 0003](0003-rendering.md) line 752 |
| OQ-0003-8 | How is ink line boil drawn? | Mustafa, in the look proof test | [ADR 0003](0003-rendering.md) line 771 |
| OQ-0003-9 | How are vegetation and the lush countryside drawn, and in which tree style? | Mustafa (the style, a later design session); the render budget bench (the technique) | [ADR 0003](0003-rendering.md) line 776 |
| OQ-0003-10 | Does the frame rate follow a 120 or 144 Hz screen, or is it capped? | Mustafa (feel), on the render budget bench's numbers | [ADR 0003](0003-rendering.md) line 783 |
| OQ-0003-11 | How many players use the WebGL 2 fallback? | Mustafa, on client field data after launch | [ADR 0003](0003-rendering.md) line 789 |
| OQ-0003-12 | Is the pixel ratio capped at 1 on Low for HiDPI laptops? | Mustafa, in the look proof test | [ADR 0003](0003-rendering.md) line 796 |
| OQ-0003-13 | Where does the audio listener sit under the v2 camera? | the camera framing first (R2-INT-OQ2, a design question), then Mustafa's listening check | [ADR 0003](0003-rendering.md) line 801 |
| OQ-0003-15 | The legacy look touches LOOK_V2 does not name: keep, change or drop each? | Mustafa, in the look proof test | [ADR 0003](0003-rendering.md) line 812 |
| OQ-0004-1 | How is bot behaviour structured? | a bot research unit, or the slice 2 plan, with the game-designer; Mustafa for the behaviour set | [ADR 0004](0004-simulation-core.md) line 755 |
| OQ-0004-4 | What moves a building through its damage stages? | Mustafa, with the game-designer (rule 6: no rule beyond LOOK_V2 is designed here) | [ADR 0004](0004-simulation-core.md) line 778 |
| OQ-0005-CD1 | Do hand-edited content files need comments? | Mustafa, after a short trial in the content and ability spike | [ADR 0005](0005-content-abilities-assets.md) line 587 |
| OQ-0005-CD2 | What load time may content cost, how much of the Wasm file is content's, and is JSON fast and small enough? | for load time, the content and ability spike's measurement, then the Advisor sets the budget; for size, ADR 0002 part 4: the Advisor sets the first `sim-wasm` limit from the boundary and sim core spikes' measurements and this spike's content share, and until then the size is recorded, not gated | [ADR 0005](0005-content-abilities-assets.md) line 593 |
| OQ-0005-CD3 | May production ever change numbers in the middle of a match? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 606 |
| OQ-0005-CD4 | In development, do simulation numbers apply at the next match, or inside a running match? | Mustafa and the Advisor | [ADR 0005](0005-content-abilities-assets.md) line 614 |
| OQ-0005-CD5 | Faction variants: full definitions that reference shared ones, or overrides of a base? | the faction design session (VISION_V2 §7, faction asymmetry; Mustafa) | [ADR 0005](0005-content-abilities-assets.md) line 626 |
| OQ-0005-CD6 | How are moment rules written? | the moment design session (Mustafa with the game-designer, with R6-TEL-OQ3's thresholds), then ADR 0004 | [ADR 0005](0005-content-abilities-assets.md) line 632 |
| OQ-0005-CD7 | What is the Feel Lab tool? | a later design session with Mustafa, before the feel pass (LOOP_V2 §11 leaves the feel pass out of the greybox v0); ADR 0003 for its screen | [ADR 0005](0005-content-abilities-assets.md) line 640 |
| OQ-0005-AB4 | What is one track: which definitions share one? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1145 |
| OQ-0005-AB6 | How do the player's tracks sit beside the team's tech tiers? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1174 |
| OQ-0005-AB7 | May a pick add raw power (more damage, more health), or only change how a unit or weapon plays? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1182 |
| OQ-0005-AB8 | The offer: when does it appear, and when can the player answer it? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1191 |
| OQ-0005-AB9 | Which drop and pickup rules does UPGRADE-SCOPE leave unstated? | Mustafa, with the game-designer | [ADR 0005](0005-content-abilities-assets.md) line 1199 |
| OQ-0005-AB10 | Do bots have tracks and make picks, and what does a human get when taking a bot's slot? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1212 |
| OQ-0005-AB11 | Does the Riot Shield answer the Banzai blade, and is the Peacekeeper's side of the triangle strong enough before Tech 1? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1221 |
| OQ-0005-AB12 | Is tranek's write-up accepted as the GAS reference, or does a verifier read Epic's documentation first? | the Advisor | [ADR 0005](0005-content-abilities-assets.md) line 1232 |
| OQ-0005-AB13 | When does scripting (option A3) come back? | the Advisor (the source) | [ADR 0005](0005-content-abilities-assets.md) line 1241 |
| OQ-0005-AP2 | Which asset families become authored models, and which stay procedural builders? | Mustafa, in the look proof test (how each looks, beside legacy screenshots, LOOK_V2 §9), with the asset-pipeline research's cost and tooling. Trees wait for the tree style (LOOK_V2 §8 `[OPEN]`) | [ADR 0005](0005-content-abilities-assets.md) line 1498 |
| OQ-0005-AP3 | May an AI-generated draft model ever ship? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1506 |
| OQ-0005-AS2 | Which tool builds the LOD levels, and how many levels? | the asset-pipeline research, then the asset spike and Mustafa (silhouettes kept) | [ADR 0005](0005-content-abilities-assets.md) line 1643 |
| OQ-0005-1 | Do presentation references sit on the simulation definitions, or in client definitions keyed by the same id? | the Advisor, at the stop review (the game-designer changes AB-4's kinds if the answer needs it) | [ADR 0005](0005-content-abilities-assets.md) line 1860 |
| OQ-0005-2 | Is a weapon definition the same thing as a weapon type? | Mustafa answers OQ-0005-AB4; the question stays open for the Advisor until then | [ADR 0005](0005-content-abilities-assets.md) line 1880 |
| OQ-0005-3 | What does the content hash cover? | the Advisor, with the infra-engineer, whose ADR 0006 owns the analysis (R6-TEL-REC3) | [ADR 0005](0005-content-abilities-assets.md) line 1891 |
| OQ-0005-4 | What do the Character Lab and the Anim Lab work on once soldiers are authored models? | Mustafa with the Advisor (EDITOR_V2 is theirs) | [ADR 0005](0005-content-abilities-assets.md) line 1903 |
| OQ-0006-1 | Can Mustafa order Hetzner's CCX23 and CX33 in an EU location? | Mustafa (a console check in his own account; agents create no account), then a research read if (b) | [ADR 0006](0006-infrastructure-operations.md) line 613 |
| OQ-0006-2 | Which EU location: Nuremberg, Helsinki, or Falkenstein if offered? | a measurement (the netcode proof test's setup or a small check before it), confirmed by Mustafa | [ADR 0006](0006-infrastructure-operations.md) line 625 |
| OQ-0006-3 | Where does the lobby run, and what stores guest ids and the room directory? | the Advisor, from a short lobby read done with the security baseline (its key storage is item B5); Mustafa for any paid service. ADR 0001 keeps the protocol, ADR 0002 the crate | [ADR 0006](0006-infrastructure-operations.md) line 632 |
| OQ-0006-4 | Once the access wall is on, does it cover every preview, or only multiplayer previews and previews with unreleased art? | Mustafa (convenience against one more project to run) | [ADR 0006](0006-infrastructure-operations.md) line 643 |
| OQ-0006-6 | The security baseline: what it decides for each waiting item. | security-auditor, in its own fresh session (BASELINE-WHEN (b)); the Advisor and Mustafa for the policy calls | [ADR 0006](0006-infrastructure-operations.md) line 673 |
| OQ-0006-7 | Will any CI runner be paid for: GPU, macOS, self-hosted reference machines, or a cache above 10 GB? | Mustafa for any paid plan; ADR 0002 says which jobs need one; the security baseline weighs (c) | [ADR 0006](0006-infrastructure-operations.md) line 684 |
| OQ-0006-8 | What do Sentry and Grafana really cost, and how do logs and metrics reach Grafana Cloud? | Mustafa, with a read of sentry.io/pricing before he creates the Sentry organization; a read of Grafana's ingestion docs if M2 is wanted | [ADR 0006](0006-infrastructure-operations.md) line 694 |
| OQ-0006-9 | What does EU data-protection law require of us, and how long are match records kept? | Mustafa | [ADR 0006](0006-infrastructure-operations.md) line 703 |
| OQ-0006-10 | How many players, and when; which VAT and currency; is included traffic per VM, pooled or prorated? | Mustafa (his plans, his billing address, the first invoice) | [ADR 0006](0006-infrastructure-operations.md) line 712 |
| OQ-0006-12 | If production isolation is ever adopted: embeds, cross-origin popups, CORP and the login flow. | Mustafa with this record, only if production isolation is adopted (R1-BND-REC2 mark 5's test) | [ADR 0006](0006-infrastructure-operations.md) line 727 |
END

### FIX verifier-synthesis finding 7 (S11 now defined below)
FILE: docs/adr/README.md
BEFORE:
| R1-BND-OQ1 | DESIGN: Mustafa | whether solo play against bots ships offline is a product need VISION_V2 does not state; SLICE-ORDER already gives local play from slice 2 (decision map S11) |
AFTER:
| R1-BND-OQ1 | DESIGN: Mustafa | whether solo play against bots ships offline is a product need VISION_V2 does not state; SLICE-ORDER already gives local play from slice 2 (seam S11) |
END

### FIX F10 (STATUS cited by item)
FILE: docs/adr/README.md
BEFORE:
**STATUS's five carry-forwards** (`docs/STATUS.md`, "Next: the decision drafts")
AFTER:
**STATUS's five carry-forwards** (`docs/STATUS.md`, "The five carry-forwards into the decision drafts")
END

### FIX G-IDX-155 (glossary: "48" alone)
FILE: docs/adr/README.md
BEFORE:
| CF-3 | Use 48 where a mark gives 48 and 54; where a sentence says "until … answers", the answer applies | every draft (each Appendix A notes the answered code); checked in the consistency pass |
AFTER:
| CF-3 | Use N = 48 where a mark gives N = 48 and N = 54; where a sentence says "until … answers", the answer applies | every draft (each Appendix A notes the answered code); checked in the consistency pass |
END

### FIX G-IDX-166 (glossary: "server-only" quoted as SEC-SPREAD's words)
FILE: docs/adr/README.md
BEFORE:
| SEC-SPREAD (a) | answered: server-only spread | ADR 0004 part 8 (the authority-only stream); ADR 0001 (no client-held copy); ADR 0003 (tracers from a cosmetic spread the hit event corrects) |
AFTER:
| SEC-SPREAD (a) | answered: "server-only spread" (the spread is known only to the authority) | ADR 0004 part 8 (the authority-only stream); ADR 0001 (no client-held copy); ADR 0003 (tracers from a cosmetic spread the hit event corrects) |
END

### FIX F10, row 53, G-IDX-209 (STATUS by item; N = 48)
FILE: docs/adr/README.md
BEFORE:
| OLD-ADR | STATUS:198: the v2 drafts replace ADR-001..003 | this index; docs-keeper marks STATUS's three entries replaced |
| ADR-001 | STATUS:199 (authority, tick, transport, hosting, offline play; its logged inputs) | ADR 0001 (authority, the tick rule, transport, rooms; forged commands refused); ADR 0004 (the tick mechanism, one hit world, phases, random streams, events, sight at 48); ADR 0006 (hosting); ADR 0005 (server-side schema checks); offline play is DESIGN (R1-BND-OQ1) |
| ADR-002 | STATUS:200–201 (modules, bundler, TypeScript, double-click build; "evidence first via the build spike") | ADR 0002 (the stack is settled; the build spike's role passes to the boundary spike and the foundation; the double-click player build ends with the legacy engine, under "Hard constraints in CLAUDE.md that change"); the editor's double-click rule is OQ-0002-1 |
| ADR-003 | STATUS:202 (stay on r128 or upgrade) | ADR 0003 part 2 (the latest three.js with WebGPURenderer, pinned, upgraded step by step: R3-WGPU-REC2) |
| SEC-MDL-2 | STATUS:257 (`__proto__` through `envMerge` / `rulesMerge`) | ADR 0005 CD-2 and CD-9 (content parsed into typed Rust structs by one validating crate, fuzzed with `__proto__` keys; server-side validation); the legacy file stays frozen |
| CDN-SRI | STATUS:262 (no SRI or CSP on the three.js CDN tag) | ADR 0002 (three.js bundled from npm with Vite: no CDN tag); the CSP is the security baseline's (ADR 0006 part 13, OQ-0006-6) |
| WALL-CLOCK | STATUS:273 (11 `performance.now` / `Date.now` reads) | ADR 0004 part 4 (the tick counter is the sim's only clock; the step reads no clock); ADR 0002 part 7 (`Instant` and `SystemTime` banned in `crates/sim`); the legacy reads stay in the frozen legacy game |
AFTER:
| OLD-ADR | STATUS, "ADR-001..003 below": the v2 drafts replace ADR-001..003 | this index; docs-keeper marks STATUS's three entries replaced |
| ADR-001 | STATUS, "ADR-001 Multiplayer architecture" (authority, tick, transport, hosting, offline play; its logged inputs) | ADR 0001 (authority, the tick rule, transport, rooms; forged commands refused); ADR 0004 (the tick mechanism, one hit world, phases, random streams, events, sight at N = 48); ADR 0006 (hosting); ADR 0005 (server-side schema checks); offline play is DESIGN (R1-BND-OQ1) |
| ADR-002 | STATUS, "ADR-002 Code structure & build" (modules, bundler, TypeScript, double-click build; "evidence first via the build spike") | ADR 0002 (the stack is settled; the build spike's role passes to the boundary spike and the foundation; the double-click player build ends with the legacy engine, under "Hard constraints in CLAUDE.md that change"); the editor's double-click rule is OQ-0002-1 |
| ADR-003 | STATUS, "ADR-003 Three.js version" (stay on r128 or upgrade) | ADR 0003 part 2 (the latest three.js with WebGPURenderer, pinned, upgraded step by step: R3-WGPU-REC2) |
| SEC-MDL-2 | STATUS, "Open bugs and risks", SEC-MDL-2 (`__proto__` through `envMerge` / `rulesMerge`) | ADR 0005 CD-2 and CD-9 (content parsed into typed Rust structs by one validating crate, fuzzed with `__proto__` keys; server-side validation); the legacy file stays frozen |
| CDN-SRI | STATUS, "Open bugs and risks", the item deferred from the gate-check-5 reviews (no SRI or CSP on the three.js CDN tag) | ADR 0002 (three.js bundled from npm with Vite: no CDN tag); the CSP is the security baseline's (ADR 0006 part 13, OQ-0006-6) |
| WALL-CLOCK | STATUS, "Open bugs and risks", the wall-clock item (11 `performance.now` / `Date.now` reads) | ADR 0004 part 4 (the tick counter is the sim's only clock; the step reads no clock); ADR 0002 part 7 (`Instant` and `SystemTime` banned in `crates/sim`); the legacy reads stay in the frozen legacy game |
END

### FIX addendum (LE-H: the 18 inventory rows)
FILE: docs/adr/README.md
BEFORE:
| LE-H | R7-MARKS (a): 118 inventory rows are "look brief pending" | ADR 0003 and ADR 0005 list the rows their decisions change; `docs/LEGACY_INVENTORY.md` changes only after Mustafa accepts the drafts |
AFTER:
| LE-H | R7-MARKS (a): 118 inventory rows are "look brief pending" | ADR 0003, ADR 0004 and ADR 0005 list the rows their decisions change, each in its "Legacy inventory rows" table; the 18 rows no draft had listed (CL5, CC10, GW5–GW8, GW10, GW11, GN2, GE3, GE4, GE7, GE9, GE10, GE14, GE16, GP8, KT5) are now there; `docs/LEGACY_INVENTORY.md` changes only after Mustafa accepts the drafts |
END

### FIX F13 (the full name)
FILE: docs/adr/README.md
BEFORE:
| LE-K | the proof test runs before the foundation with a throwaway sim | OQ-0001-2 (TICK-60-SETUP) |
AFTER:
| LE-K | the netcode proof test runs before the foundation with a throwaway sim | OQ-0001-2 (TICK-60-SETUP) |
END

### FIX verifier-synthesis row 54 (R2-BUD-REC4)
FILE: docs/adr/README.md
BEFORE:
- **ADR 0001:** R2-BUD-REC5 and R2-TRN-REC6 (part 10), the network profiles against which every mark is restated (part 13, R2-XT-OQ14): VISION_V2 §4 only.
AFTER:
- **ADR 0001:** R2-BUD-REC4 (the input buffer and the clock, parts 5 and 7), R2-BUD-REC5 and R2-TRN-REC6 (part 10), the network profiles against which every mark is restated (part 13, R2-XT-OQ14): VISION_V2 §4 only.
END

### FIX verifier-synthesis row 54 (roof fade beside §6.2)
FILE: docs/adr/README.md
BEFORE:
- **ADR 0003:** the look pipeline (part 5): LOOK_V2 §1–§7 only (with VISION_V2 §6.9 for its cost); halftone flashes and line boil: LOOK_V2 §7 only; the concussion shake: LOOK_V2 §2 only; roof fade: LOOK_V2 §4.
AFTER:
- **ADR 0003:** the look pipeline (part 5): LOOK_V2 §1–§7 only (with VISION_V2 §6.9 for its cost); halftone flashes and line boil: LOOK_V2 §7 only; the concussion shake: LOOK_V2 §2 only; roof fade (5.i): LOOK_V2 §4, beside §6.2.
END

### FIX verifier-synthesis row 54 (damage stages only)
FILE: docs/adr/README.md
BEFORE:
- **ADR 0004:** three damage stages and structure states (part 14): LOOK_V2 §4.
AFTER:
- **ADR 0004:** three damage stages (part 14): LOOK_V2 §4 only (the structure states themselves serve §6.2, §6.4 and §6.5).
END

### FIX F12, completeness-4 (plan pointers; local code families)
FILE: docs/adr/README.md
BEFORE:
The drafts cite codes from the decision map (`.checkpoints/v2-decision-drafts/engine-architect-decision-map.md`, a
checkpoint file deleted before the PR is marked ready). Their definitions live here from now on. Decision codes such
as TICK-30, SLICE-ORDER or UPGRADE-SCOPE live in `docs/STATUS.md`; GLOSSARY terms in `docs/research/GLOSSARY.md`.
AFTER:
The drafts cite codes from the decision map (`.checkpoints/v2-decision-drafts/engine-architect-decision-map.md`, a
checkpoint file deleted before the PR is marked ready). Their definitions live here from now on. Decision codes such
as TICK-30, SLICE-ORDER or UPGRADE-SCOPE live in `docs/STATUS.md`; GLOSSARY terms in `docs/research/GLOSSARY.md`.
Plan rules, plan steps and the advisor's conditions ("plan rule n", "plan step n", "condition n"): the report's
appendix, `docs/reports/2026-11-01-v2-decision-drafts-appendix.md` §3–§4, quotes them word for word. Codes local to one
draft are not register codes: ADR 0002's gate checks G1–G13, ADR 0004's spike claims S1–S22 and ADR 0006's option
labels (G1–G4, S1–S3, K1–K4, B1–B11); a register code always carries its word ("gap G5", "seam S1").
END

### FIX G-IDX-279 (glossary: "tick time" names its kind)
FILE: docs/adr/README.md
BEFORE:
| I-12 | Server → monitoring | tick time histogram, overruns, dropped time, per-phase time, sessions, bytes, memory, room starts and crashes | 0006; 0004 and 0001 emit | |
AFTER:
| I-12 | Server → monitoring | server tick time histogram, overruns, dropped time, per-phase time, sessions, bytes, memory, room starts and crashes | 0006; 0004 and 0001 emit | |
END

### FIX F12 (S1–S22)
FILE: docs/adr/README.md
BEFORE:
**Seams** (cited as "seam S-n"; the map's seams, not ADR 0004's own claim numbers S1–S20)
AFTER:
**Seams** (cited as "seam S-n"; the map's seams, not ADR 0004's own claim numbers S1–S22)
END

### FIX verifier-synthesis finding 7, F12 (seam S11 defined)
FILE: docs/adr/README.md
BEFORE:
| S12 | Security | 0006: the security baseline listed open, its interim positions and every item waiting on it | 0001, 0002, 0005 cite the baseline item where their design waits on it |
AFTER:
| S11 | Offline and local play | DESIGN: whether offline play ships, and at the largest mode on the minimum laptop (R1-BND-OQ1, R1-TICK-OQ7, critic rank 15) | 0002 and 0004: the local server from slice 1 and local play with bots in slice 2 (SLICE-ORDER), whatever the answer |
| S12 | Security | 0006: the security baseline listed open, its interim positions and every item waiting on it | 0001, 0002, 0005 cite the baseline item where their design waits on it |
END

### FIX G-IDX-306 (glossary: the visual-effects row)
FILE: docs/adr/README.md
BEFORE:
| G5 | visual effects (fragments, muzzle flashes, tracers, impacts, debris) | ADR 0003 OQ-0003-7; the effects budget row stands |
AFTER:
| G5 | visual effects (fragments, muzzle flashes, tracers, impacts, debris) | ADR 0003 OQ-0003-7; the visual-effects budget row stands |
END

### FIX F10 (J2: STATUS items by name)
FILE: docs/adr/README.md
BEFORE:
| J2 | three register items: the old ADR-001..003 entries, STATUS:257, :262, :273, and PR #28 finding 4's closed condition (all in the loose-ends register above) |
AFTER:
| J2 | three register items: the old ADR-001..003 entries, STATUS's SEC-MDL-2, CDN-tag and wall-clock items, and PR #28 finding 4's closed condition (all in the loose-ends register above) |
END

### NOTE F1
Not the critic's wording at SPIKE_PLAN:158: "content files reaching the Worker in development (PM-0002-31, R1-DATA-OQ8)" is already at SPIKE_PLAN:160, so the fix drops "hot reload into the Worker" and leaves parts 2, 4 and 7 as "Wasm size and tools" (ADR 0002 spike item 8, "Size and tools (parts 2, 4 and 7)"). Same outcome, no duplicate.
END

### NOTE verifier-synthesis row 54, second half
Applied though skeptic 3 did not re-check it: ADR 0004 at 9a03999, part 14's Need line (0004:395–396) reads "VISION_V2 §6.2 ..., §6.4 ..., §6.5 ...; the damage stages: LOOK_V2 §4 only", so only the damage stages belong in the register, as the verifier said.
END

### NOTE completeness-15
The §6 placement is put on the parts' labels in the one diagram (no second diagram), and per-frame edges are added on the page; the page stays at about 137 lines (condition 19). The §6 table stays as the place that names draft and part.
END

### NOTE counts
Open questions at 9a03999: 0001 14, 0002 8, 0003 15, 0004 7 numbered (OQ-0004-5 and -6 now say "Moved" to OQ-0005-AB9 and AB10, so 5 open), 0005 34, 0006 12: 90 numbered, 88 open; the prompt's "89" did not yet count OQ-0002-8 or the two moved ones. Pass marks: 57, 76, 43, 74, 46, 31 rows (327), each max id equal to its row count.
END
