# engine-architect synthesis fixes, final round (docs at 604a497)


### FIX FC-2 (README: the draft-local code families, as skeptic-5 upholds them)
FILE: docs/adr/README.md
BEFORE:
appendix, `docs/reports/2026-11-01-v2-decision-drafts-appendix.md` §3–§4, quotes them word for word. Codes local to one
draft are not register codes: ADR 0002's gate checks G1–G13, ADR 0004's spike claims S1–S22 and ADR 0006's option
labels (G1–G4, S1–S3, K1–K4, B1–B11); a register code always carries its word ("gap G5", "seam S1").
AFTER:
appendix, `docs/reports/2026-11-01-v2-decision-drafts-appendix.md` §3–§4, quotes them word for word. Codes local to one
draft are not register codes, for example ADR 0002's gate checks G0–G13, ADR 0004's spike claims S1–S22, ADR 0006's
option labels (G1–G4, K1–K4, S1–S5) and its baseline items B1–B12 (part 13); a register code always carries its word
("gap G5", "seam S1").
END

### FIX FC-3 (SPIKE_PLAN item 3: the spike measures for R1-RR-OQ5; the Advisor sets the limit)
FILE: docs/SPIKE_PLAN.md
BEFORE:
  (optional). **It decides:** R1-BND-OQ2, OQ3 (the core count recorded), OQ6, OQ9 (only if isolation is adopted);
  R1-XU-OQ3, OQ4; R1-TICK-OQ2; R1-DATA-OQ8; R1-RR-OQ5 (with the sim core spike); R5-LAYOUT-OQ4; R5-TSLINT-OQ3, OQ4,
  OQ6; R5-TEST-OQ1; R6-OBS-OQ3, OQ4 (optional); critic ranks 4 (the implementation side) and 37; it reports the copy time for
  R1-RR-OQ6 (PM-0002-10; ADR 0002 answers R1-RR-OQ6, proposed, for rust-reviewer to confirm); it gives OQ-0002-1,
AFTER:
  (optional). **It decides:** R1-BND-OQ2, OQ3 (the core count recorded), OQ6, OQ9 (only if isolation is adopted);
  R1-XU-OQ3, OQ4; R1-TICK-OQ2; R1-DATA-OQ8; R5-LAYOUT-OQ4; R5-TSLINT-OQ3, OQ4, OQ6; R5-TEST-OQ1; R6-OBS-OQ3, OQ4
  (optional); critic ranks 4 (the implementation side) and 37. With the sim core spike it measures the `sim-wasm` size
  for R1-RR-OQ5, whose first limit the Advisor sets once the content and ability spike adds content's share (ADR 0002
  part 4); it reports the copy time for R1-RR-OQ6 (PM-0002-10; ADR 0002 answers R1-RR-OQ6, proposed, for rust-reviewer
  to confirm); it gives OQ-0002-1,
END

### FIX FC-4 (SPIKE_PLAN, the foundation: the deploy token is ADR 0006 part 13, item B9; OQ-0006-13 first)
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Depends on:** the drafts accepted; the boundary spike's and the netcode proof test's results; the preview hosting
  check before the preview step; Mustafa's client-host token (ADR 0006 part 1) and branch-protection setting.
AFTER:
- **Depends on:** the drafts accepted; the boundary spike's and the netcode proof test's results; the preview hosting
  check before the preview step; Mustafa's client-host deploy token in the `preview` environment (ADR 0006 part 13,
  item B9; how production stays out of that token's reach is OQ-0006-13, answered before the first client-host token
  is added) and branch-protection setting.
END

### FIX SA-1 routed by infra-engineer (SPIKE_PLAN item 6: the test project's token is revoked when the check ends)
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Depends on:** the drafts approved; Mustafa's Cloudflare account with a test project and its deploy token in a
  `preview` environment (secrets are his to add); a static test page with a module worker, a `.wasm` and a hashed
  bundle stand-in; throwaway deploy and close workflows; ML and MAC with the four browsers.
AFTER:
- **Depends on:** the drafts approved; Mustafa's Cloudflare account with a test project and its deploy token in a
  `preview` environment (secrets are his to add; he revokes the token when the check ends: ADR 0006 part 13, item B9);
  a static test page with a module worker, a `.wasm` and a hashed bundle stand-in; throwaway deploy and close
  workflows; ML and MAC with the four browsers.
END

### FIX ADR 0002 batch routed (ENGINE_OVERVIEW diagram: the `client-endpoint` crate in the Worker)
FILE: docs/ENGINE_OVERVIEW.md
BEFORE:
    WASM["sim-wasm (Rust as Wasm)<br/>client endpoint: prediction, reconciliation, interpolation<br/>local server: room + sim"]
AFTER:
    WASM["sim-wasm (Rust as Wasm)<br/>client-endpoint: prediction, reconciliation, interpolation, aim cast, block writer<br/>local server: room + sim"]
END

### FIX NA-17 (ENGINE_OVERVIEW: in local play the Worker is both the client endpoint and the local server)
FILE: docs/ENGINE_OVERVIEW.md
BEFORE:
One Rust simulation decides everything that counts. The same code runs natively in each room process on the match
server and as WebAssembly in one Web Worker in the browser, where it is the client endpoint online (prediction and
reconciliation) and the local server in local play, from slice 1 (SLICE-ORDER, condition 1). The page is TypeScript on
three.js `WebGPURenderer` with its WebGL 2 fallback; it draws, animates and plays sound from one block of data per tick
and never decides a hit, a death or a purchase. Content (units, vehicles, weapons, abilities, effects, upgrade tracks,
modes, maps, tuning) is JSON that one Rust content crate validates the same way on the server, in the Worker and in the
editor. The server ticks at 30 Hz (TICK-30) for up to N = 48 players (the largest mode, 24v24: MODES-3 (a)); 60 Hz
only with [the 60 Hz margin](research/GLOSSARY.md#the-60-hz-margin).
AFTER:
One Rust simulation decides everything that counts. The same code runs natively in each room process on the match
server and as WebAssembly in one Web Worker in the browser, where it is the client endpoint (prediction and
reconciliation) online and in local play, and in local play also the local server, from slice 1 (SLICE-ORDER,
condition 1). The page is TypeScript on three.js `WebGPURenderer` with its WebGL 2 fallback; it draws, animates and
plays sound from one block of data per tick and never decides a hit, a death or a purchase. Content (units, vehicles,
weapons, abilities, effects, upgrade tracks, modes, maps, tuning) is JSON that one Rust content crate validates the
same way on the server, in the Worker and in the editor. The server ticks at 30 Hz (TICK-30) for up to N = 48 players
(the largest mode, 24v24: MODES-3 (a)); 60 Hz only with [the 60 Hz margin](research/GLOSSARY.md#the-60-hz-margin).
END

### FIX FC-5, GE-12, NA-7 routed (ENGINE_OVERVIEW diagram: newest input per frame; draw between the two newest blocks; HUD data)
FILE: docs/ENGINE_OVERVIEW.md
BEFORE:
  IN -->|"per tick: one command, posted"| SW
  SW -->|"per tick: one transferred block: header, entity sections, events"| R
  IN -->|"per frame: camera"| R
  R -->|"per frame: interpolate, animate, draw the newest block"| R
  R -->|"per frame: events as sound"| AU
  R -->|"per frame: events as HUD feedback"| HUD
AFTER:
  IN -->|"per frame: the newest input and cursor ray, posted"| SW
  SW -->|"per tick: one transferred block: header, entity sections, owner-only data, events"| R
  IN -->|"per frame: camera"| R
  R -->|"per frame: interpolate between the two newest blocks, animate, draw"| R
  R -->|"per frame: events as sound"| AU
  R -->|"per frame: events and owner-only data as HUD"| HUD
END

### FIX FC-5, GE-12, NA-7 routed (ENGINE_OVERVIEW: the words under the diagram)
FILE: docs/ENGINE_OVERVIEW.md
BEFORE:
In words: the page posts one command per tick to the Worker and receives one block per tick back; it makes no call
into Wasm and touches no network (ADR 0002 part 5). Per frame, the page interpolates, animates and draws the newest
block and plays its events as sound and HUD feedback (ADR 0003 part 10). The Worker holds the Rust sim as Wasm and
owns the connection (ADR 0002 part 6). Online, the Worker talks to the room process over WebTransport, or WebSocket
when that fails (ADR 0001 part 10); in local play the same Worker runs the room and the sim itself, behind the same
messages (SLICE-ORDER condition 1; ADR 0002 part 6). The sim crate and the content crate sit below everything. Each
part's label names the VISION_V2 §6 items it serves; the table below gives the draft and part.
AFTER:
In words: the page posts its newest input to the Worker each drawn frame and receives one block per tick back; it
makes no call into Wasm and touches no network (ADR 0002 part 5). Per frame it interpolates between the two newest
blocks, animates, draws, and plays the block's events as sound and HUD feedback (ADR 0003 part 10). The Worker holds
the Rust sim as Wasm (the client endpoint is its own crate) and owns the connection (ADR 0002 parts 3 and 5). Online
it talks to the room process over WebTransport, or WebSocket when that fails (ADR 0001 part 10); in local play the
same Worker also runs the room and the sim as the local server, reached through the loopback with the same messages
(SLICE-ORDER condition 1; ADR 0002 part 6). The sim crate and the content crate sit below everything. Each part's
label names the main VISION_V2 §6 items it serves; the table below gives every draft and part.
END

### FIX NA-7 and EA-1 routed (ENGINE_OVERVIEW: the boundary, both directions)
FILE: docs/ENGINE_OVERVIEW.md
BEFORE:
- **Down (Worker → page):** one transferred output block per tick (B2), with a header (tick, layout hash, counts),
  sections per entity kind and an event section (the events of ADR 0004 part 19, such as hits, deaths and kills,
  blasts, purchases, uplink cache events, tech tiers, upgrade picks and pickups, and moments); the SharedArrayBuffer
  ring (B3) is measured against it in the boundary spike. The fields of remote entities online are ADR 0001's; the
  layout is ADR 0002's.
- **Up (page → Worker):** one command per tick, posted, including the world aim point from the page's cursor ray
  (interface I-19; ADR 0001 part 5); a small control API (start, mode flag, content bundle).
AFTER:
- **Down (Worker → page):** one transferred output block per tick (B2): a header (tick, layout hash, counts), sections
  per entity kind, the local player's owner-only data (own weapon, money, upgrade tracks and offers) and an event
  section (ADR 0004 part 19's events, such as hits, deaths and kills, blasts, purchases, uplink cache events, tech
  tiers, upgrade picks and pickups, and moments; a remote entity's spatial events come in the block whose delayed time
  reaches their tick, ADR 0001 part 7); the SharedArrayBuffer ring (B3) is measured against it in the boundary spike.
  Remote entities' fields online are ADR 0001's; the layout is ADR 0002's.
- **Up (page → Worker):** the newest input once per drawn frame, with the cursor ray and view time; each tick the
  Worker fills its one network command from the newest post, aim point included (I-19; ADR 0001 part 5; ADR 0002
  part 5); a small control API (start, mode flag, content bundle).
END

### FIX ADR 0002 batch routed (ENGINE_OVERVIEW: the `client-endpoint` crate in the dependency rules)
FILE: docs/ENGINE_OVERVIEW.md
BEFORE:
**Dependencies point one way** (ADR 0002 part 3, each a gate check): `sim` depends on `content` only, with no
transport, no async runtime and no clock; `room` builds for Wasm with no async runtime; `protocol` uses `sim` and
`content` types; `sim-wasm` = `sim` + `room` + `protocol` + `content`, never `navbake` or `server`; `server` adds the
transport and the runtime; `client` never reaches `editor`. Among the tools, `devtools` may depend on any workspace
crate and `navbake` on `sim` and `content`; the editor uses `client` code and the `navbake` and `sim` Wasm; nothing
imports `legacy/` or `spikes/`. Each crate enters with the first slice that needs it.
AFTER:
**Dependencies point one way** (ADR 0002 part 3, each a gate check): `sim` depends on `content` only, with no
transport, no async runtime and no clock; `room` and `client-endpoint` build for Wasm and natively with no async
runtime, no transport and no clock; `protocol` uses `sim` and `content` types; `sim-wasm` holds `client-endpoint`,
`room`, `sim`, `protocol` and `content`, never `navbake` or `server`; `server` adds the transport and the runtime; the
`supervisor` links no sim code; `client` never reaches `editor`. Among the tools, `devtools` may depend on any
workspace crate and `navbake` on `sim` and `content`; the editor uses `client` code and the `navbake` and `sim` Wasm;
nothing imports `legacy/` or `spikes/`. Each crate enters with the first slice that needs it.
END

### FIX FC-5, GE-12, NA-7 routed (ENGINE_OVERVIEW per frame: two newest blocks; the newest input posted)
FILE: docs/ENGINE_OVERVIEW.md
BEFORE:
**Per frame (the page; ADR 0003 part 10):** take the newest block; interpolate and animate (procedural touches, budget
LOD); draw with the look pipeline (ADR 0003 part 5); play events as VFX, sound and HUD feedback (hit feedback on
confirmation, LOOP_V2 §7); post the next command. Presentation randomness stays on the page (ADR 0004 part 8).
AFTER:
**Per frame (the page; ADR 0003 part 10):** keep the two newest blocks; interpolate between them and animate (procedural
touches, budget LOD); draw with the look pipeline (ADR 0003 part 5); play the block's events as VFX, sound and HUD
feedback (hit feedback on confirmation, LOOP_V2 §7); post the newest input with its cursor ray and view time (ADR 0002
part 5). Presentation randomness stays on the page (ADR 0004 part 8).
END

### FIX PA's note, PA-4 landed, OQ-0003-17 routed by graphics-engineer (ENGINE_OVERVIEW §6.9: part 7's presented-frame count)
FILE: docs/ENGINE_OVERVIEW.md
BEFORE:
| 9 | A spectacle budget: a few very large units without dropping frames | ADR 0003 parts 3, 6, 7 (no first-sight stutter, very large units, budgets); ADR 0002 part 5 (the sim off the page's thread); ADR 0004 (its share: the very large units are in the stress scene its sim is measured in, GLOSSARY §3) |
AFTER:
| 9 | A spectacle budget: a few very large units without dropping frames | ADR 0003 parts 3, 6, 7 (no first-sight stutter, very large units, budgets; part 7's presented-frame count reads "without dropping frames", PM-0003-44, and how many such frames a run may hold is OQ-0003-17); ADR 0002 part 5 (the sim off the page's thread); ADR 0004 (its share: the very large units are in the stress scene its sim is measured in, GLOSSARY §3) |
END

### FIX EA-1 routed (ENGINE_OVERVIEW per tick, step 4: remote spatial events on the delayed timeline)
FILE: docs/ENGINE_OVERVIEW.md
BEFORE:
4. Client: the Worker predicts the local player's movement and own weapon, reconciles with each snapshot, and
   interpolates remote entities; a shot is judged on the server against what the shooter saw, within a cap (ADR 0001
   parts 4, 7, 8; the abilities' prediction waits for ABL-PREDICT).
AFTER:
4. Client: the Worker predicts the local player's movement and own weapon, reconciles with each snapshot, interpolates
   remote entities and releases their spatial events on its delayed timeline; a shot is judged on the server against
   what the shooter saw, within a cap (ADR 0001 parts 4, 7, 8; the abilities' prediction waits for ABL-PREDICT).
END

### FIX test-engineer A and B routed (SPIKE_PLAN table: the final round's new pass-mark rows)
FILE: docs/SPIKE_PLAN.md
BEFORE:
| 1 | The netcode proof test | L (+ M for ADR 0006's operations marks) | wave 1, first | PM-0001-1 to -57; PM-0004-67; PM-0006-11 to -27 |
| 2 | The 60 Hz margin run | inside 1 | wave 1, after its sim load is settled | PM-0001-39, -40; PM-0004-71, -72 |
| 3 | The Rust ↔ TypeScript boundary spike | L (+ M ADR 0004's share; + S ADR 0006's, optional) | wave 1 | PM-0002-1 to -34; PM-0004-8, -28, -61, -68 to -70; PM-0006-28 to -31 (optional) |
| 4 | The sim core spike | L, the largest | wave 1 | PM-0004-1 to -61, -73, -74 |
| 5 | The look proof test | L (+ S or M for ADR 0005's share) | wave 1 | PM-0003-1 to -17; PM-0005-33 to -37 |
| 6 | The preview hosting check | S (calendar: at least 2 days) | wave 1; ends before the foundation's preview step | PM-0006-1 to -10 |
| – | The foundation's acceptance checks (not a spike) | L (six change-sets) | step 2 | PM-0002-35 to -69; later PM-0002-70 to -76 |
| 7 | The content and ability spike | L | wave 3 | PM-0005-1 to -32; PM-0004-62 to -66 |
| 8 | The render budget bench | L | wave 3 | PM-0003-18 to -42 (PM-0003-1 again); PM-0003-43 at each later upgrade |
AFTER:
| 1 | The netcode proof test | L (+ M for ADR 0006's operations marks) | wave 1, first | PM-0001-1 to -62; PM-0004-67; PM-0006-11 to -27 |
| 2 | The 60 Hz margin run | inside 1 | wave 1, after its sim load is settled | PM-0001-39, -40; PM-0004-71, -72 |
| 3 | The Rust ↔ TypeScript boundary spike | L (+ M ADR 0004's share; + S ADR 0006's, optional) | wave 1 | PM-0002-1 to -34, -77, -78; PM-0004-8, -28, -61, -68 to -70; PM-0006-28 to -31 (optional) |
| 4 | The sim core spike | L, the largest | wave 1 | PM-0004-1 to -61, -73, -74 |
| 5 | The look proof test | L (+ S or M for ADR 0005's share) | wave 1 | PM-0003-1 to -17; PM-0005-33 to -37 |
| 6 | The preview hosting check | S (calendar: at least 2 days) | wave 1; ends before the foundation's preview step | PM-0006-1 to -10, -32 |
| – | The foundation's acceptance checks (not a spike) | L (six change-sets) | step 2 | PM-0002-35 to -69; later PM-0002-70 to -76 |
| 7 | The content and ability spike | L | wave 3 | PM-0005-1 to -32; PM-0004-62 to -66, -75 |
| 8 | The render budget bench | L | wave 3 | PM-0003-18 to -42, -44 to -47 (PM-0003-1 again); PM-0003-43 at each later upgrade |
END

### FIX IE-2 (SPIKE_PLAN item 1: one set of density runs under PM-0006-13's rules) and test-engineer A routed (PM-0001-62)
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Pass marks:** PM-0001-1 to PM-0001-57 (PM-0001-39 and -40 belong to the 60 Hz margin run below; PM-0001-54 and -55
  only when their condition holds); PM-0004-67; PM-0006-11 to PM-0006-27 (PM-0006-13 shares its runs with ADR 0001's
  R2-ROOM-REC2 marks: one set of runs, read the same in both tables). Its server also serves the online halves of
  PM-0004-61 and PM-0004-70.
AFTER:
- **Pass marks:** PM-0001-1 to PM-0001-62 (PM-0001-39 and -40 belong to the 60 Hz margin run below; PM-0001-54 and -55
  only when their condition holds); PM-0004-67; PM-0006-11 to PM-0006-27 (PM-0006-13 and ADR 0001's PM-0001-27, the
  R2-ROOM-REC2 marks, share one set of runs under PM-0006-13's rules: no CPU pinning, its bracket B, 3 sessions at
  3 times of day, steal recorded and never a validity condition; so both tables read the same runs). Its server also
  serves the online halves of PM-0004-61 and PM-0004-70.
END

### FIX TE-1, TE-2 (TE Q2) and infra-engineer routed (SPIKE_PLAN item 1 depends on: S1's stand-ins, padded density runs, OQ-0006-2, -15, -16, -17, -19, -20, OQ-0001-16)
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Depends on:** the drafts approved; OQ-0006-1 answered (the CCX23 and CX33 orderable in one EU location, in
  Mustafa's Hetzner account) and LG and MV beside them; ML, RD and MAC named (OQ-0003-1); a throwaway sim with movement,
  hitscan and bots as protocol clients, native and Wasm, with stand-in big vehicles and live projectiles at the stress
  scene's counts (PM-0001-2); the link simulator's L1 and L2 (ADR 0002's design), built here first; a DM_Tri-sized
  greybox with enterable buildings and the target course; the development certificates for WebTransport (ADR 0006);
  for the WebSocket leg, OQ-0006-5's development and staging answer (the Advisor, from a short read, before the test
  starts), since a WebSocket cannot pin a certificate hash (ADR 0006, OQ-0006-5); the Advisor's readings of OQ-0001-3
  before the checklist; Mustafa's session in Brave; the spike's run plan under its bill mark (PM-0006-18) before the
  first VM.
AFTER:
- **Depends on:** the drafts approved; OQ-0006-1 answered (the CCX23 and CX33 orderable in one EU location, in
  Mustafa's Hetzner account), with the location measured before RS is created (OQ-0006-2), the way sessions get VMs
  without holding a secret (OQ-0006-17), and LG and MV beside them; ML, RD and MAC named (OQ-0003-1); a throwaway sim
  with movement, hitscan and bots as protocol clients, native and Wasm, with stand-in big vehicles and live projectiles
  at the stress scene's counts (PM-0001-2); its 30 Hz marks may add ADR 0004's S1 stand-ins (384 effect rows with the
  effect step's 1 ms burned per tick, the 2.80 ms remainder topped up: ADR 0001, OQ-0001-2), and its density runs pad
  its sim step to the sim step share's ceiling, so rooms per VM is a lower bound and the goal budget provisional until
  slice 6 (ADR 0006 spike item 2; OQ-0006-19, read before those runs); the link simulator's L1 and L2 (ADR 0002's
  design), built here first, once the Advisor has read how each profile splits over the two directions (OQ-0001-16);
  a DM_Tri-sized greybox with enterable buildings and the target course; the port ranges (OQ-0006-16) and the
  development certificates for WebTransport (ADR 0006); for the WebSocket leg, OQ-0006-5's development and staging
  answer (the Advisor, from a short read, before the test starts), since a WebSocket cannot pin a certificate hash
  (ADR 0006, OQ-0006-5), and a domain first if OQ-0006-15 picks (b); the Advisor's readings of OQ-0001-3 before the
  checklist; Mustafa's session in Brave; the spike's run plan under its bill mark (PM-0006-18) and the cost Mustafa
  allows (OQ-0006-20) before the first VM.
END

### FIX netcode-architect routed (SPIKE_PLAN item 1: the input-to-screen numbers for OQ-0001-15)
FILE: docs/SPIKE_PLAN.md
BEFORE:
  WebTransport crates' interop); R6-COST-OQ1; it feeds OQ-0006-2 and gives Mustafa and the Advisor the numbers for
  OQ-0001-5.
AFTER:
  WebTransport crates' interop); R6-COST-OQ1; it feeds OQ-0006-2 and gives Mustafa and the Advisor the numbers for
  OQ-0001-5, and Mustafa the input-to-screen numbers for OQ-0001-15.
END

### FIX TE-1 (SPIKE_PLAN item 2: the parts the wave-1 sim lacks are stood in, or the verdict waits)
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Depends on:** OQ-0001-1 and OQ-0001-2 answered. Under OQ-0001-2's recommended option (a), the sim core spike's sim
  runs inside the proof-test server, so the run waits until that sim runs the stress scene; under (c), the fallback,
  the check waits until the real sim runs the stress scene (slice 6) and the tick stays at 30 Hz until then.
- **Place:** wave 1, inside the netcode proof test, after the sim core spike's sim runs the stress scene (option (a)).
AFTER:
- **Depends on:** OQ-0001-1 and OQ-0001-2 answered. Under OQ-0001-2's recommended option (a), the sim core spike's sim
  runs inside the proof-test server, so the run waits until that sim runs the stress scene; the parts of the sim step
  share it does not build are stood in as ADR 0004's S1 stands them in, each named in the run's record: the ability
  and effect step (it arrives only with the content and ability spike, wave 3) as 384 rows in the effect pool with its
  1 ms ceiling burned per tick, and economy, objectives, moments and events (slice 4) as the remainder topped up per
  tick to 0.30 ms at 60 Hz (4.17 − 3.87, arithmetic on ADR 0004 part 6), so a pass still holds once the real parts
  meet their ceilings. Under (d) the run waits instead until the content and ability spike's effect step has joined
  that sim; under (c), the fallback, the check waits until the real sim runs the stress scene (slice 6) and the tick
  stays at 30 Hz until then.
- **Place:** wave 1, inside the netcode proof test, after the sim core spike's sim runs the stress scene (option (a));
  under (d), after the content and ability spike (wave 3).
END

### FIX GE-13 (SPIKE_PLAN item 1: it builds the shared client harness first)
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Place:** wave 1, first in priority: before the foundation (SLICE-ORDER condition 3). It shares its client harness
  with the boundary spike.
- **Size:** L. Two transports on four browsers, two simulator layers, a room server with interest, encoding and lag
  compensation at 48 clients, a load generator, repeated 30-minute soaks, 8 rooms on two machine types, planted defects,
  Mustafa's session; ADR 0006's operations marks add M; ADR 0004's share is S.
AFTER:
- **Place:** wave 1, first in priority: before the foundation (SLICE-ORDER condition 3). It builds first the client
  harness the boundary spike shares (the page with its Worker host and a plain renderer, on the throwaway sim: ADR
  0002's spike text), proposed with the order (J6 (5)).
- **Size:** L, the client harness included. Two transports on four browsers, two simulator layers, a room server with
  interest, encoding and lag compensation at 48 clients, a load generator, repeated 30-minute soaks, 8 rooms on two
  machine types, planted defects, Mustafa's session; ADR 0006's operations marks add M; ADR 0004's share is S.
END

### FIX GE-13 (SPIKE_PLAN item 3: it builds the measurement harness's CPU side first)
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Depends on:** the drafts approved; ML named and on hand (OQ-0003-1), MAC, and displays at
  120 and 144 Hz; the netcode proof test's throwaway sim, client harness and room process (it runs beside it); ADR
  0003's frame-time method (its part 8); the sim core spike's sim for PM-0002-28 and PM-0004-8, -28 (a stand-in until
  then); Mustafa's session in Brave.
- **Place:** wave 1, beside the netcode proof test; its result fixes the skeleton the foundation builds.
- **Size:** L (ADR 0002's part); ADR 0004's share M; ADR 0006's optional share S.
AFTER:
- **Depends on:** the drafts approved; ML named and on hand (OQ-0003-1), MAC, and displays at 120 and 144 Hz; the
  netcode proof test's throwaway sim, client harness and room process (it runs beside it); the sim core spike's sim
  for PM-0002-28 and PM-0004-8, -28 (a stand-in until then); Mustafa's session in Brave.
- **Place:** wave 1, beside the netcode proof test; its result fixes the skeleton the foundation builds. It builds
  first, on the shared client harness, the CPU side of ADR 0003 part 8's measurement harness (frame time, the
  renderer's counters, the run sets and their labels), which the look proof test and the render budget bench then use
  (proposed with the order, J6 (5); the graphics review's finding 13).
- **Size:** L (ADR 0002's part, the measurement harness's CPU side included); ADR 0004's share M; ADR 0006's optional
  share S.
END

### FIX NA-7 routed (SPIKE_PLAN item 3: the newest input per frame, one network command per tick; the connection is part 5)
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **What it proves:** ADR 0002 part 5 (one module Worker; the page makes no Wasm call and touches no network; one
  transferred block per tick, B2, with the SharedArrayBuffer ring, B3, measured against it; one command per tick; the
  Worker's calls per tick constant in N; the page's frame time with the sim within its budget at N = 12 to 48 on the
  minimum laptop; the generated types and the layout hash; the isolation flag), part 6 (the Worker owns the connection;
  the client + local server skeleton; one Wasm instance or two in a local match, OQ-0002-3), parts 2, 4 and 7 (Wasm
  size and tools), part 11 (the editor from `file://`, OQ-0002-1); ADR 0004 part 5 (S18: the
  Worker's pacing, smooth motion on 60, 120 and 144 Hz displays, hidden tabs), part 6 (S2: the client's sim share) and
  part 20 (S20: recovery after a trap); ADR 0005's content files reaching the Worker (PM-0002-31, R1-DATA-OQ8).
AFTER:
- **What it proves:** ADR 0002 part 5 (one module Worker; the page makes no Wasm call and touches no network; one
  transferred block per tick, B2, with the SharedArrayBuffer ring, B3, measured against it; the newest input posted
  once per drawn frame and one network command per tick built from it, its aim cast against what the page drew; the
  Worker owns the connection; the Worker's calls per tick constant in N; the page's frame time with the sim within its
  budget at N = 12 to 48 on the minimum laptop; the generated types and the layout hash; the isolation flag), part 6
  (the client + local server skeleton and the Worker's whole cost, reported; one Wasm instance or two in a local
  match, OQ-0002-3), parts 2, 4 and 7 (Wasm size and tools), part 11 (the editor from `file://`, OQ-0002-1); ADR 0004
  part 5 (S18: the Worker's pacing, smooth motion on 60, 120 and 144 Hz displays, hidden tabs), part 6 (S2: the
  client's sim share) and part 20 (S20: recovery after a trap); ADR 0005's content files reaching the Worker
  (PM-0002-31, R1-DATA-OQ8).
END

### FIX test-engineer B routed (SPIKE_PLAN item 3: PM-0002-77 and -78)
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Pass marks:** PM-0002-1 to PM-0002-34 (PM-0002-21, -27 and -33 conditional; PM-0002-28 gates once the sim core
  spike's sim runs in the Worker); PM-0004-8, PM-0004-28, PM-0004-68 to PM-0004-70, and PM-0004-61 with the netcode proof
  test's server; optional, only if Mustafa has created the EU Sentry organisation (OQ-0006-8): PM-0006-28 to PM-0006-31
  (otherwise they move to the first online playtest's milestone). ADR 0003's PM-0003-20 (the p99 of the page's own sim
  work) is measured on this spike's Worker and block in the render budget bench.
AFTER:
- **Pass marks:** PM-0002-1 to PM-0002-34, PM-0002-77 and PM-0002-78 (PM-0002-21, -27 and -33 conditional; PM-0002-78
  reported; PM-0002-28 gates once the sim core spike's sim runs in the Worker); PM-0004-8, PM-0004-28, PM-0004-68 to
  PM-0004-70, and PM-0004-61 with the netcode proof test's server; optional, only if Mustafa has created the EU Sentry
  organisation (OQ-0006-8): PM-0006-28 to PM-0006-31 (otherwise they move to the first online playtest's milestone).
  ADR 0003's PM-0003-20 (the p99 of the page's own sim work) is measured on this spike's Worker and block in the render
  budget bench.
END

### FIX FC-6, TE-1 (SPIKE_PLAN item 4: seven targets named as seven; S1's stand-ins for the parts it does not build)
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **What it proves:** ADR 0004 parts 2, 3, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 20 and 21: the Rust sim on one vCPU
  of the reference server and in Wasm on the minimum laptop, in the stress scene at N = 48 at 30 Hz (the sim step share
  and its parts, the scale steps and the N = 64 headroom run); snapshots, exact restore and stable ids; same-build and
  cross-target identity by per-tick state hashes on seven targets (native, Node, three browser engines, real Safari on
  MAC, and Chrome and Firefox on the minimum laptop's Windows: PM-0004-19), with a suite
  that catches each planted divergence at its tick; parry3d with our own movement; both navigation generators, the path
  queue, avoidance, floors and typed links; sight; big vehicles as obstacles, structure states and three damage stages;
  the editor's reach checks; no panic from commands.
AFTER:
- **What it proves:** ADR 0004 parts 2, 3, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 20 and 21: the Rust sim on one vCPU
  of the reference server and in Wasm on the minimum laptop, in the stress scene at N = 48 at 30 Hz (the sim step share
  and its parts, the parts it does not build stood in as S1 says: 384 effect rows with the effect step's 1 ms ceiling
  burned per tick and the 2.80 ms remainder topped up, each named in the run's record; the scale steps and the N = 64
  headroom run); snapshots, exact restore and stable ids; same-build and cross-target identity by per-tick state
  hashes on seven targets (native, Node, three browser engines, real Safari on MAC, and Windows on the minimum laptop,
  in Chrome and Firefox: PM-0004-19), with a suite that catches each planted divergence at its tick; parry3d with our
  own movement; both navigation generators, the path queue, avoidance, floors and typed links; sight; big vehicles as
  obstacles, structure states and three damage stages; the editor's reach checks; no panic from commands.
END

### FIX TE-1 (SPIKE_PLAN item 4 size: S1's stand-ins in the throwaway sim)
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Size:** L, the largest spike (a throwaway sim with movement, the vehicle sweep, two navigation generators, bots,
  sight and structure states; a determinism suite over seven targets; three machines; about 60 marks, most of them
  scripted checks that can join CI afterwards).
AFTER:
- **Size:** L, the largest spike (a throwaway sim with movement, the vehicle sweep, two navigation generators, bots,
  sight and structure states, and S1's stand-ins for the effect step and the remainder; a determinism suite over seven
  targets; three machines; about 60 marks, most of them scripted checks that can join CI afterwards).
END

### FIX GE Q1, AD questions, OQ-0003-16, -18, -19 routed by graphics-engineer (SPIKE_PLAN item 5: the calls Mustafa makes in the test)
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Who judges what:** Mustafa judges the whole look: the look checklist (ADR 0003, inside PM-0003-17), which names each
  LOOK_V2 item it shows beside a legacy screenshot, and the families checklist (ADR 0005, inside PM-0005-37). He also
  makes the calls ADR 0003 leaves to him (OQ-0003-2, -3, -5, -8, -12, -15; the ink family); OQ-0003-6 (the roof fade)
  is the test's to decide, with his look among what decides it. The scripts judge the frame-time and GPU marks on ML
  and RD.
AFTER:
- **Who judges what:** Mustafa judges the whole look: the look checklist (ADR 0003, inside PM-0003-17), which names each
  LOOK_V2 item it shows beside a legacy screenshot, and the families checklist (ADR 0005, inside PM-0005-37). He also
  makes the calls ADR 0003 leaves to him (OQ-0003-2, -3, -5, -8, -12, -15; the ink family; the look floor on the Low
  frame, OQ-0003-16; reduced motion, OQ-0003-18; whether colour alone tells the factions apart, OQ-0003-19); OQ-0003-6
  (the roof fade) is the test's to decide, with his look among what decides it. The scripts judge the frame-time and
  GPU marks on ML and RD.
END

### FIX AD-13 (SPIKE_PLAN item 5: the [DEFAULT] items are judged too, and a veto is recorded)
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **What it proves:** how each `[DECIDED]` item of [LOOK_V2](LOOK_V2.md) looks, judged by Mustafa in Brave beside legacy
  screenshots at the same framing; each look technique's GPU time, draw calls and memory alone, on both reference
  machines and both backends; both ink families at 12, 24 and 48 soldiers on the minimum laptop at Low; one build on
  both backends in the four browsers with the same look; no first-sight shader stutter; the backend reported (and
  Brave's on Mustafa's PC). For ADR 0005: plain glTF/GLB exports load on both backends with the engine's shared
  materials per slot class, every model listed by id, a broken link refused at load, and Mustafa's pick per asset
  family (OQ-0005-AP2).
AFTER:
- **What it proves:** how each `[DECIDED]` and `[DEFAULT]` item of [LOOK_V2](LOOK_V2.md) looks (a `[DEFAULT]` stands
  unless vetoed, so a veto is recorded), judged by Mustafa in Brave beside legacy screenshots at the same framing;
  each look technique's GPU time, draw calls and memory alone, on both reference machines and both backends; both ink
  families at 12, 24 and 48 soldiers on the minimum laptop at Low; one build on both backends in the four browsers with
  the same look; no first-sight shader stutter; the backend reported (and Brave's on Mustafa's PC). For ADR 0005:
  plain glTF/GLB exports load on both backends with the engine's shared materials per slot class, every model listed
  by id, a broken link refused at load, and Mustafa's pick per asset family (OQ-0005-AP2).
END

### FIX AD-7, GE-5, GE-13 and OQ-0006-18 routed (SPIKE_PLAN item 5: both houses, who makes the models, the harness, unreleased art)
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Depends on:** the drafts approved; ML and RD named and on hand (OQ-0003-1), MAC; a few authored test
  assets as plain glTF/GLB exports with named material slots and no compression or LOD step (ASSET-RESEARCH (a): one
  soldier with faction colour slots, one weapon, one two-floor building with its damage stages, one very large unit,
  a few props); legacy screenshots at the same framing; the measurement harness of ADR 0003 part 8, built here first; a
  synthetic recording for the scene; the content crate's whole-set check (a minimal loader, since the content and
  ability spike comes later); for OQ-0005-AP2, the ported kit builders beside the authored models (ADR 0005's share; ADR
  0003's own asset list names only authored assets).
- **Place:** wave 1, in parallel with the netcode proof test (they share nothing); before any art production and before
  slice 1's renderer.
- **Size:** L; ADR 0005's share S, or M if the kit builders are ported for the families' comparison.
AFTER:
- **Depends on:** the drafts approved; ML and RD named and on hand (OQ-0003-1), MAC; a few authored test assets as plain
  glTF/GLB exports with named material slots and no compression or LOD step (ASSET-RESEARCH (a); ADR 0003 part 11):
  one soldier in LOOK_V2 §3's proportions with faction colour slots, one weapon, LOOK_V2 §2's two houses (the
  half-timbered house with the brick-tile roof as the two-floor building with an interior and its damage stages, and
  the plastered house with the silo tower, authored too or drawn by ADR 0005's ported kit builders), one very large
  unit of at least 20 parts, a few props; who makes these models and by which route is OQ-SPIKE-1 (below), and whether
  a committed model may be seen by anyone is OQ-0006-18 (with OQ-0005-AP1), both answered before the first model is
  committed; legacy screenshots at the same framing; ADR 0003 part 8's measurement harness, whose CPU side the boundary
  spike builds first and to which this test adds the GPU timestamp queries (its timed marks wait for it, its look work
  does not); a synthetic recording for the scene; the content crate's whole-set check (a minimal loader, since the
  content and ability spike comes later); for OQ-0005-AP2, the ported kit builders beside the authored models (ADR
  0005's share; ADR 0003 part 11).
- **Place:** wave 1, in parallel with the netcode proof test (they share nothing but the boundary spike's measurement
  harness); before any art production and before slice 1's renderer.
- **Size:** L, without making the models, which is sized once OQ-SPIKE-1 is answered; ADR 0005's share S, or M if the
  kit builders are ported for the families' comparison.
END

### FIX GE Q1, AD questions routed (SPIKE_PLAN item 5: it gives Mustafa what he needs for OQ-0003-16, -18, -19)
FILE: docs/SPIKE_PLAN.md
BEFORE:
  R3-GL2-OQ1 and R3-LOOK-OQ6 after their source readings; it gives Mustafa what he needs for OQ-0003-2, -3, -5, -6, -8,
  -12 and -15, and OQ-0005-AP2.
AFTER:
  R3-GL2-OQ1 and R3-LOOK-OQ6 after their source readings; it gives Mustafa what he needs for OQ-0003-2, -3, -5, -6, -8,
  -12, -15, -16, -18 and -19, and OQ-0005-AP2.
END

### FIX test-engineer B and infra-engineer routed (SPIKE_PLAN item 6: B9's interim position, PM-0006-32)
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **What it proves:** ADR 0006 parts 1, 2 and 3: the client host serves the production and preview header sets as
  generated, on branch aliases and hash URLs; `crossOriginIsolated` follows the isolation flag in Chrome, Edge, Firefox,
  Safari and Brave; the content bundle is cached immutably; every file fits the host's 25 MiB limit; **PREV-DELETE:** a
  closed pull request's build is gone from every link within 24 h; what the access wall covers, with one login per
  Brave session; whether direct uploads count against the host's build allowance; whether GitHub shows the environment
  URL.
- **Pass marks:** PM-0006-1 to PM-0006-10.
AFTER:
- **What it proves:** ADR 0006 parts 1, 2 and 3: the client host serves the production and preview header sets as
  generated, on branch aliases and hash URLs; `crossOriginIsolated` follows the isolation flag in Chrome, Edge, Firefox,
  Safari and Brave; the content bundle is cached immutably; every file fits the host's 25 MiB limit; **PREV-DELETE:** a
  closed pull request's build is gone from every link within 24 h; what the access wall covers, with one login per
  Brave session; whether direct uploads count against the host's build allowance; whether GitHub shows the environment
  URL; and part 13's B9 interim position: a pull request reaches no deploy secret and no production (OQ-0006-13).
- **Pass marks:** PM-0006-1 to PM-0006-10, PM-0006-32.
END

### FIX test-engineer A routed (SPIKE_PLAN item 7: PM-0004-75, conditional until OQ-0004-11 is answered)
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Pass marks:** PM-0005-1 to PM-0005-32 (PM-0005-11 only if OQ-0005-CD4 picks option (b)); PM-0004-62 to PM-0004-66.
AFTER:
- **Pass marks:** PM-0005-1 to PM-0005-32 (PM-0005-11 only if OQ-0005-CD4 picks option (b)); PM-0004-62 to PM-0004-66,
  and PM-0004-75 once OQ-0004-11 is answered (which row gives way when a pool is full).
END

### FIX test-engineer A routed (SPIKE_PLAN item 8: PM-0003-44 to -47)
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Pass marks:** PM-0003-18 to PM-0003-42 (PM-0003-40 only if occlusion rays are ever proposed), PM-0003-1 checked
  again; PM-0003-43 at each later upgrade.
AFTER:
- **Pass marks:** PM-0003-18 to PM-0003-42 (PM-0003-40 only if occlusion rays are ever proposed), PM-0003-44 to
  PM-0003-47 (PM-0003-44 reported until OQ-0003-17 sets its share), PM-0003-1 checked again; PM-0003-43 at each later
  upgrade.
END

### FIX OQ-0003-17 and OQ-0003-20 routed (SPIKE_PLAN item 8: the counts and the soaked set it gives the Advisor)
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Research spikes it subsumes:** R3-CHAR-REC1 to REC4; R3-BUDGET-REC2, REC3; R3-GL2-REC2; R3-MEASURE-REC1 to REC4
  (ADR 0003 places REC1 and REC4
  here); R3-AUDIO-REC1 to REC4; R3-WGPU-REC2 (later). **It decides:** R3-CHAR-OQ1, OQ2, OQ4 (OQ6 only if the draw-call
  cap is still missed); R3-BUDGET-OQ5, and R3-BUDGET-OQ6 with OQ-0003-4; R3-AUDIO-OQ1, OQ2 (OQ6 only if the built-in
  panners miss PM-0003-38); it gives Mustafa the numbers for OQ-0003-10.
AFTER:
- **Research spikes it subsumes:** R3-CHAR-REC1 to REC4; R3-BUDGET-REC2, REC3; R3-GL2-REC2; R3-MEASURE-REC1 to REC4
  (ADR 0003 places REC1 and REC4 here); R3-AUDIO-REC1 to REC4; R3-WGPU-REC2 (later). **It decides:** R3-CHAR-OQ1, OQ2,
  OQ4 (OQ6 only if the draw-call cap is still missed); R3-BUDGET-OQ5, and R3-BUDGET-OQ6 with OQ-0003-4; R3-AUDIO-OQ1,
  OQ2 (OQ6 only if the built-in panners miss PM-0003-38); it gives Mustafa the numbers for OQ-0003-10, and the Advisor
  the dropped-frame counts for OQ-0003-17 and the cold and soaked sets for OQ-0003-20.
END

### FIX GE-5, GE Q2 (SPIKE_PLAN: OQ-SPIKE-1, who makes the look test's models); TE Q1, TE Q2 and the new spike-gating questions in the list
FILE: docs/SPIKE_PLAN.md
BEFORE:
## What this plan does not decide
The order after the netcode proof test, TICK-60-HEADROOM (OQ-0001-1), TICK-60-SETUP (OQ-0001-2), the "100
ms" reading (OQ-0001-3), the reference machines (OQ-0003-1), the server location (OQ-0006-1, OQ-0006-2) and
ASSET-RESEARCH's timing are listed for Mustafa and the Advisor in `docs/adr/README.md`, "Decisions the drafts ask for".
AFTER:
## What this plan does not decide
The order after the netcode proof test (J6 (5), with the harness builders it proposes), TICK-60-HEADROOM (OQ-0001-1),
TICK-60-SETUP (OQ-0001-2: the stand-ins, or wait), the "100 ms" reading (OQ-0001-3), the profiles' split over the two
directions (OQ-0001-16), the reference machines (OQ-0003-1), the server location (OQ-0006-1, OQ-0006-2), the spikes'
VMs and their cost (OQ-0006-17, OQ-0006-20), the provisional goal budget (OQ-0006-19), ASSET-RESEARCH's timing and
OQ-SPIKE-1 below are listed for Mustafa and the Advisor in `docs/adr/README.md`, "Decisions the drafts ask for".

## Open questions
- **OQ-SPIKE-1. Who makes the look proof test's authored models, and by which route?** *Why it matters:* the look proof
  test runs in wave 1, before the asset-pipeline research, and needs as glTF/GLB with named material slots a skinned
  soldier with faction colour slots, the two-floor house with its damage stages and a very large unit of at least 20
  parts (ADR 0003 part 11); without them the look checklist cannot judge the soldier, the buildings or the very large
  unit. LOOK_V2 §8's flow makes a model in Spline ("AI-generated for drafts, or a clean model built by Claude") and
  exports glTF/GLB, but whether that export carries a skin and named slots is not researched (gap G1; OQ-0005-AS3,
  AS4), and making the models is in no spike's size (the graphics review of 2026-11-02, finding 5 and its question
  2). *Options:* (a) the slice of the asset-pipeline research that answers OQ-0005-AS3 and AS4 for skinned and
  multi-part models runs first, and the models are made by the route it finds; (b) the test's own session makes
  throwaway models by a route the test names, and the research follows later; (c) the test starts without the skinned
  soldier and leaves its steps for later. *Recommendation:* (a), the graphics review's: a small slice of research, and
  the look test does not decide the pipeline by accident. *Decider:* Mustafa and the Advisor, with ASSET-RESEARCH's
  timing. *What decides it:* their reading, before the look proof test starts; the making is sized once it is
  answered. *Research ids:* none (gap G1).
END

### FIX GE-5 (SPIKE_PLAN, the asset-pipeline research: its timing now also answers OQ-SPIKE-1)
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Place:** before the asset spike. **Timing is a decision for Mustafa and the Advisor** (ASSET-RESEARCH in
  `docs/adr/README.md`): they may move it earlier, for example beside wave 1, if art production should start sooner.
AFTER:
- **Place:** before the asset spike. **Timing is a decision for Mustafa and the Advisor** (ASSET-RESEARCH in
  `docs/adr/README.md`): they may move it earlier, for example beside wave 1, if art production should start sooner;
  under OQ-SPIKE-1's option (a), its slice on export checks and slot renaming (OQ-0005-AS3, AS4) runs before the look
  proof test.
END
