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
