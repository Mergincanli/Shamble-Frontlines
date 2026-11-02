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

### FIX FC-5, GE-12, NA-7 and EA-1 routed (ENGINE_OVERVIEW: the words under the diagram)
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
In words: each drawn frame the page posts its newest input, with the cursor ray, to the Worker, and it receives one
block per tick back; it makes no call into Wasm and touches no network (ADR 0002 part 5). Per frame, the page
interpolates between the two newest blocks, animates and draws, and plays the block's events as sound and HUD feedback
(ADR 0003 part 10). The Worker holds the Rust sim as Wasm, with the client endpoint in its own crate, and owns the
connection (ADR 0002 parts 3 and 5); it releases a remote entity's spatial events on that entity's drawn timeline
(ADR 0001 part 7). Online, the Worker talks to the room process over WebTransport, or WebSocket when that fails (ADR
0001 part 10); in local play the same Worker also runs the room and the sim as the local server, which its client
endpoint reaches through the loopback with the same messages (SLICE-ORDER condition 1; ADR 0002 part 6). The sim crate
and the content crate sit below everything. Each part's label names the main VISION_V2 §6 items it serves; the table
below gives every draft and part.
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
- **Down (Worker → page):** one transferred output block per tick (B2), with a header (tick, layout hash, counts),
  sections per entity kind, the local player's owner-only data (own weapon, money, upgrade tracks and offers) and an
  event section (the events of ADR 0004 part 19, such as hits, deaths and kills, blasts, purchases, uplink cache
  events, tech tiers, upgrade picks and pickups, and moments, each in the block its entity's drawn time reaches); the
  SharedArrayBuffer ring (B3) is measured against it in the boundary spike. The fields of remote entities online are
  ADR 0001's; the layout is ADR 0002's.
- **Up (page → Worker):** the newest input, posted once per drawn frame with the cursor ray and the view time; each
  tick the Worker takes the newest post (running press totals lose no press) and fills the tick's one network command,
  world aim point included (interface I-19; ADR 0001 part 5; ADR 0002 part 5); a small control API (start, mode flag,
  content bundle).
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
runtime, no transport and no clock; `protocol` uses `sim` and `content` types; `sim-wasm` = `client-endpoint` + `room`
+ `sim` + `protocol` + `content`, never `navbake` or `server`; `server` adds the transport and the runtime; the
`supervisor` links no sim code; `client` never reaches `editor`. Among the tools, `devtools` may depend on any
workspace crate and `navbake` on `sim` and `content`; the editor uses `client` code and the `navbake` and `sim` Wasm;
nothing imports `legacy/` or `spikes/`. Each crate enters with the first slice that needs it.
END
