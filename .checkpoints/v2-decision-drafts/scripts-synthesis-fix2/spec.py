import importlib.util, re
spec_ = importlib.util.spec_from_file_location('oqx', SNAP + '/../fix2gen/oqx.py'); oqx = importlib.util.module_from_spec(spec_); spec_.loader.exec_module(oqx)
names, oqs = oqx.extract(SNAP)
line_of = {o[2]: o[1] for o in oqs}
O, SP, RM = 'docs/ENGINE_OVERVIEW.md', 'docs/SPIKE_PLAN.md', 'docs/adr/README.md'
def ptr(oq):
    a = oq[3:7]; return f'[ADR {a}]({names[a]}) line {line_of[oq]}'

# ---------------- ENGINE_OVERVIEW ----------------
fix('G-OVW-14 (glossary: "48" alone for the largest mode)', O, 14, 15, after=
"editor. The server ticks at 30 Hz (TICK-30) for up to N = 48 players (the largest mode, 24v24: MODES-3 (a)); 60 Hz\n"
"only with [the 60 Hz margin](research/GLOSSARY.md#the-60-hz-margin).")
fix('completeness-15 (the diagram shows the per-frame flow and the §6 placement)', O, 19, 47, after='''flowchart TB
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
  CONTENT -.->|"read by id"| SIM''')
fix('completeness-15 (the words under the diagram)', O, 49, 53, after=
"""In words: the page posts one command per tick to the Worker and receives one block per tick back; it makes no call
into Wasm and touches no network (ADR 0002 part 5). Per frame, the page interpolates, animates and draws the newest
block and plays its events as sound and HUD feedback (ADR 0003 part 10). The Worker holds the Rust sim as Wasm and
owns the connection (ADR 0002 part 6). Online, the Worker talks to the room process over WebTransport, or WebSocket
when that fails (ADR 0001 part 10); in local play the same Worker runs the room and the sim itself, behind the same
messages (SLICE-ORDER condition 1; ADR 0002 part 6). The sim crate and the content crate sit below everything. Each
part's label names the VISION_V2 §6 items it serves; the table below gives the draft and part.""")
fix('F13 (no "captures" event: ADR 0004 part 19\'s words)', O, 56, 59, after=
"""- **Down (Worker → page):** one transferred output block per tick (B2), with a header (tick, layout hash, counts),
  sections per entity kind and an event section (the events of ADR 0004 part 19, such as hits, deaths and kills,
  blasts, purchases, uplink cache events, tech tiers, upgrade picks and pickups, and moments); the SharedArrayBuffer
  ring (B3) is measured against it in the boundary spike. The fields of remote entities online are ADR 0001's; the
  layout is ADR 0002's.""")
fix('verifier-synthesis row 33 (tools do not depend on everything)', O, 65, 69, after=
"""**Dependencies point one way** (ADR 0002 part 3, each a gate check): `sim` depends on `content` only, with no
transport, no async runtime and no clock; `room` builds for Wasm with no async runtime; `protocol` uses `sim` and
`content` types; `sim-wasm` = `sim` + `room` + `protocol` + `content`, never `navbake` or `server`; `server` adds the
transport and the runtime; `client` never reaches `editor`. Among the tools, `devtools` may depend on any workspace
crate and `navbake` on `sim` and `content`; the editor uses `client` code and the `navbake` and `sim` Wasm; nothing
imports `legacy/` or `spikes/`. Each crate enters with the first slice that needs it.""")
fix('verifier-synthesis row 37 (§6.4: the leech\'s two primitives, and marking)', O, 101, 101, subs=[(
"ADR 0005 AB-6 (the two primitives §6.4 needs: an effect removed when its source dies, and marking)",
"ADR 0005 AB-6 (leech: an effect removed when its source dies and a last-hit trigger that moves the victim's item to the source; marking: an effect that grants a tag)")])

# ---------------- SPIKE_PLAN ----------------
fix('G-SPK-15 (glossary: "48" alone)', SP, 15, 15, subs=[("Where a research mark gives 48 and 54, 48 holds", "Where a research mark gives N = 48 and N = 54, N = 48 holds")])
fix('F4 (one MAC definition)', SP, 30, 32, after=
"""  MV, a CX33 running the metrics stack (ADR 0006); ML, the minimum laptop, and RD, the recommended desktop (ADR 0003,
  OQ-0003-1: the models are not named yet); MAC, a macOS machine with the stable Safari on the day of the run, its
  version recorded (WebGPU from Safari 26.0: r3-rendering.md, R3-WGPU-F19, lines 171–174; WebTransport from 26.4:
  r2-netcode.md, R2-TRN-F7, lines 959–975); MP, Mustafa's PC in Brave, for what he judges only.""")
fix('J6-order (J6 has its home in README)', SP, 41, 41, subs=[("(decision map part j, J6 (5))", "(J6 (5) in `docs/adr/README.md`'s codes)")])
fix('addendum (PM-0001-57 in the netcode proof test row)', SP, 74, 74, subs=[("PM-0001-1 to -56;", "PM-0001-1 to -57;")])
fix('F7 (PM-0004-73 and -74 in the sim core spike row)', SP, 77, 77, subs=[("| PM-0004-1 to -61 |", "| PM-0004-1 to -61, -73, -74 |")])
fix('F8 (item 1 proves the `wss` leg\'s certificate)', SP, 96, 96, after=
"""  match-hour, the spike's own bill, logging cost, metrics, the development certificates for WebTransport, and the
  WebSocket leg's `wss` sessions on the certificate OQ-0006-5 names, PM-0006-15).""")
fix('addendum (PM-0001-57)', SP, 97, 97, subs=[("PM-0001-1 to PM-0001-56", "PM-0001-1 to PM-0001-57")])
fix('F8, G-SPK-106 (item 1 depends on OQ-0006-5; big vehicles and live projectiles)', SP, 104, 110, after=
"""- **Depends on:** the drafts approved; OQ-0006-1 answered (the CCX23 and CX33 orderable in one EU location, in
  Mustafa's Hetzner account) and LG and MV beside them; ML, RD and MAC named (OQ-0003-1); a throwaway sim with movement,
  hitscan and bots as protocol clients, native and Wasm, with stand-in big vehicles and live projectiles at the stress
  scene's counts (PM-0001-2); the link simulator's L1 and L2 (ADR 0002's design), built here first; a DM_Tri-sized
  greybox with enterable buildings and the target course; the development certificates for WebTransport (ADR 0006);
  for the WebSocket leg, OQ-0006-5's development and staging answer (the Advisor, from a short read, before the test
  starts), since a WebSocket cannot pin a certificate hash (ADR 0006, OQ-0006-5); the Advisor's readings of OQ-0001-3
  before the checklist; Mustafa's session in Brave; the spike's run plan under its bill mark (PM-0006-18) before the
  first VM.""")
fix('F12/drift (R1-TICK-REC4: ADR 0001 now names it)', SP, 121, 122, subs=[(
"R1-TICK-REC4 (through PM-0004-67: the decision map placed it here, ADR 0001's spike\n  text does not name it)",
"R1-TICK-REC4 (ADR 0004's S17, PM-0004-67, run on this\n  spike's server, as ADR 0001's spike text says)")])
fix('F14 (R5-TEST-OQ2, as ADR 0001 now lists it)', SP, 125, 127, after=
"""- **It decides:** R2-TRN-OQ1, OQ2, OQ3, OQ5, OQ8; R2-BUD-OQ3, OQ7, OQ9, OQ10; R2-SYNC-OQ3 (through OQ-0001-6), OQ5 (if
  vehicles move), OQ12; R2-ROOM-OQ1; R2-XU-OQ2, OQ4, OQ5, OQ6, OQ7, OQ15; OQ-0001-6, OQ-0001-7; R5-TEST-OQ2 (the
  WebTransport crates' interop); R6-COST-OQ1; it feeds OQ-0006-2 and gives Mustafa and the Advisor the numbers for
  OQ-0001-5.""")
fix('F1 (no "hot reload into the Worker"; line 160 names PM-0002-31)', SP, 157, 158, subs=[("(Wasm\n  size, tools, hot reload into the Worker)", "(Wasm\n  size and tools)")])
fix('F4 (MAC, item 3)', SP, 167, 167, subs=[("MAC with the latest Safari,", "MAC,")])
fix('addendum (a) (R1-RR-OQ6: ADR 0002 answers it; the spike reports the copy time)', SP, 176, 177, subs=[
("R1-RR-OQ5 (with the sim core spike), OQ6; ", "R1-RR-OQ5 (with the sim core spike); "),
("and 37; it gives OQ-0002-1,", "and 37; it reports the copy time for\n  R1-RR-OQ6 (PM-0002-10; ADR 0002 answers R1-RR-OQ6, proposed, for rust-reviewer to confirm); it gives OQ-0002-1,")])
fix('verifier-synthesis finding 9 (the seven targets named)', SP, 185, 185, after=
"""  cross-target identity by per-tick state hashes on seven targets (native, Node, three browser engines, real Safari on
  MAC, and Chrome and Firefox on the minimum laptop's Windows: PM-0004-19), with a suite""")
fix('F7 (PM-0004-73 and -74, item 4)', SP, 189, 190, after=
"""- **Pass marks:** PM-0004-1 to PM-0004-61, PM-0004-73 and PM-0004-74 (PM-0004-8 and -28 in the boundary spike's
  Worker; PM-0004-61 with the netcode proof test's server).""")
fix('F4 (MAC, item 4)', SP, 192, 192, subs=[("MAC with Safari;", "MAC;")])
fix('F7 (the two marks now have ADR 0004 rows)', SP, 204, 206, after=
"""  only if tiling is needed); R5-TEST-REC2's suite; R2-SYNC-REC6 mark 1 (throws bit-identical on Wasm and native: ADR
  0004's S21, PM-0004-73; the id is ADR 0001's) and R5-LAYOUT-REC4 (one feature set for `sim` on every target: S22,
  PM-0004-74; the id is ADR 0002's); the foundation later""")
fix('verifier-synthesis finding 8 (OQ-0003-6 named the same in both places)', SP, 225, 226, after=
"""  makes the calls ADR 0003 leaves to him (OQ-0003-2, -3, -5, -8, -12, -15; the ink family); OQ-0003-6 (the roof fade)
  is the test's to decide, with his look among what decides it. The scripts judge the frame-time and GPU marks on ML
  and RD.""")
fix('F4 (MAC, item 5)', SP, 227, 227, subs=[("MAC with Safari;", "MAC;")])
fix('G-SPK-274 (glossary: "Cloudflare" alone for the client host)', SP, 274, 274, subs=[("Mustafa's Cloudflare token", "Mustafa's client-host token (ADR 0006 part 1)")])
fix('F4 (MAC, item 7)', SP, 297, 297, subs=[("MAC with Safari (PM-0005-2)", "MAC (PM-0005-2)")])
fix('G-SPK-310 (glossary: "characters" for soldiers)', SP, 310, 310, subs=[("characters at N = 48 on screen", "soldiers at N = 48 on screen")])
fix('F12 (no "part g": the map leaves)', SP, 323, 324, subs=[("(ADR 0003 places REC1 and REC4 here, where the decision map's part g had them in the look proof test; the draft\n  holds)", "(ADR 0003 places REC1 and REC4\n  here)")])
fix('F12 (part h: README\'s research gaps)', SP, 330, 330, subs=[("(decision map part h, G1)", "(gap G1 in `docs/adr/README.md`)")])
fix('F12 (part h, item 9)', SP, 354, 354, subs=[("none in R1–R6 (decision map part h);", "none in R1–R6 (gap G1 in `docs/adr/README.md`);")])
fix('F12 (LE-N has its home in README)', SP, 359, 359, subs=[("(decision map LE-N)", "(LE-N in `docs/adr/README.md`)")])
fix('F13 (the full name outside ADR 0001)', SP, 369, 369, subs=[("if the proof test says", "if the netcode proof test says")])
fix('F12 (no "part g", later milestones)', SP, 375, 375, subs=[("From the decision map's part g, as the drafts keep them:", "As the drafts keep them:")])

# ---------------- README ----------------
fix('F9 (what the line numbers are)', RM, 9, 10, after=
"""The drafts replace the legacy entries ADR-001, ADR-002 and ADR-003 (`docs/STATUS.md`, "ADR-001..003 below"; see the
loose-ends register). Line numbers below are the drafts' at commit 9a03999, after the fix rounds; a later edit may move
them, so each row also names its OQ id or part.""")
fix('drift (a), addendum (counts at 9a03999)', RM, 15, 23, subs=[
("| 13 (OQ-0001-1 to -13) |", "| 14 (OQ-0001-1 to -14) |"),
("| 7 (OQ-0002-1 to -7) |", "| 8 (OQ-0002-1 to -8) |"),
("| 7 (OQ-0004-1 to -7) |", "| 7 numbered (OQ-0004-1 to -7), 5 open: -5 and -6 moved to OQ-0005-AB9 and AB10 |"),
("Total: 88 open questions. None of the drafts is signed off; no spike starts before Mustafa and the Advisor approve them\n(the drafts brief, §4).",
 "Total: 90 numbered open questions, 88 of them open (OQ-0004-5 and -6 point to ADR 0005). Pass marks: 57, 76, 43, 74, 46\nand 31 rows (327). None of the drafts is signed off; no spike starts before Mustafa and the Advisor approve them (the\ndrafts brief, §4).")])

hand = [
f"| TICK-60-HEADROOM (OQ-0001-1) | Is the 60 Hz margin measured against the 60 Hz server tick budget (8.33 ms), so 60 Hz needs a p99 server tick time of at most 6.25 ms plus every 60 Hz research mark? | Mustafa and the Advisor (confirm at the stop) | [ADR 0001](0001-multiplayer-netcode.md) part 2, line {line_of['OQ-0001-1']}; [GLOSSARY §8](../research/GLOSSARY.md#the-60-hz-margin) |",
f"| TICK-60-SETUP (OQ-0001-2) | How does the stress scene's sim load reach the 60 Hz margin run: (a) the sim core spike's sim inside the proof-test server (recommended), (b) a stand-in replaying measured step times, (c) wait until the real sim runs the stress scene? | the Advisor | {ptr('OQ-0001-2')}; SPIKE_PLAN item 2 |",
f"| \"100 ms\" reading (OQ-0001-3) | Does the netcode proof test's \"100 ms of simulated latency\" mean profile P2 (100 ms round trip, with P2's jitter and loss), or 100 ms each way? | the Advisor (who wrote the definition) | {ptr('OQ-0001-3')} |",
f"| ABL-PREDICT (OQ-0001-4) | Does the client predict the player's own abilities? Decided after the Feel Lab test on the Molotov and the Shield. | Mustafa | {ptr('OQ-0001-4')}; [ADR 0005](0005-content-abilities-assets.md) AB-9; SPIKE_PLAN item 10 |",
"| ASSET-RESEARCH (timing) | The asset-pipeline research runs before the asset spike; should it run earlier (for example beside the first spikes) so art production can start sooner? | Mustafa and the Advisor | [ADR 0005](0005-content-abilities-assets.md) line 76 and the asset pipeline part; SPIKE_PLAN, \"The asset-pipeline research\" |",
f"| WEAPON-UNLOCKS (OQ-0005-AB5) | How does an account-unlocked weapon reach a match (LOOP_V2 §8, VISION_V2 §1), when buying offers no weapon choice and other weapons come only from drops? | Mustafa, with the Advisor (LOOP_V2 §8 is theirs) | {ptr('OQ-0005-AB5')} |",
f"| UPGRADE-SCOPE: carry slots (OQ-0005-AB1) | How many weapons can a player carry besides the class weapon, and what happens to the one in hand on a pickup? | Mustafa | {ptr('OQ-0005-AB1')} |",
f"| UPGRADE-SCOPE: looted ammo (OQ-0005-AB2) | What does a looted gun come with, and can it be refilled? | Mustafa | {ptr('OQ-0005-AB2')} |",
f"| UPGRADE-SCOPE: levelling (OQ-0005-AB3) | What earns a level, how fast, and does a track the player switches to catch up? | Mustafa | {ptr('OQ-0005-AB3')} |",
f"| Reference machines (OQ-0003-1) | Which laptop (the minimum) and which desktop (the recommended) are bought or borrowed as the two reference machines? | Mustafa and the Advisor (money and availability) | {ptr('OQ-0003-1')} |",
"| Spike order (J6 (5)) | Only the netcode proof test's place is settled (before the foundation); is SPIKE_PLAN's proposed order of the rest accepted? | Mustafa and the Advisor | [SPIKE_PLAN](../SPIKE_PLAN.md), \"The spikes, in the proposed order\" |",
f"| Trace reading (OQ-0001-14; LE-G, R2-XU-OQ18) | Is a VISION_V2 §3 or §4 target trace enough for an engine feature (written \"need: VISION_V2 §4 only\" and listed below)? The session read it so on 2026-10-31. | the Advisor, at the stop | {ptr('OQ-0001-14')}; \"Needs outside VISION_V2 §6\" below |",
f"| Structure without a research option (OQ-0002-8) | May a record propose a structure no research Options section offers, when a settled input forces it and the research leaves the question to that record (the `room` and `navbake` crates)? | the Advisor, at the stop | {ptr('OQ-0002-8')} |",
f"| The `wss` certificate on development and staging (OQ-0006-5) | Which certificate does the WebSocket fallback present on development and staging servers, since a WebSocket cannot pin a certificate hash? The netcode proof test's WebSocket leg needs the answer first. | the Advisor, from a short read, before the netcode proof test starts (production issuance: security-auditor, in the security baseline) | {ptr('OQ-0006-5')}; SPIKE_PLAN item 1 |",
"| Fragments as damage | Do grenade fragments deal damage per fragment? LOOK_V2 §3 and §7 bring fragmentation back as a visible effect, which ADR 0003 draws from the blast event; any rule beyond that is a design question (no research id). | Mustafa with the game-designer | [ADR 0004](0004-simulation-core.md) \"Open questions\", closing paragraph; [ADR 0003](0003-rendering.md) part 5.k, OQ-0003-7 and the look checklist step 24 |",
"| The design questions | The 38 research questions below that are pure design matters | Mustafa, or a later design session | \"Design questions\" below |",
]
skip = {'OQ-0001-1','OQ-0001-2','OQ-0001-3','OQ-0001-4','OQ-0001-14','OQ-0002-8','OQ-0003-1','OQ-0005-AB1','OQ-0005-AB2','OQ-0005-AB3','OQ-0005-AB5','OQ-0006-5'}
gen = []
for a, ln, oq, q, d in oqs:
    if oq in skip or not re.search(r'Mustafa|Advisor', d): continue
    d = d.rstrip(' -.')
    gen.append(f'| {oq} | {q} | {d} | [ADR {a}]({names[a]}) line {ln} |')
fix('F2, F8, F9, F14, drift (a)–(c), addendum (the decisions table re-pinned at 9a03999)', RM, 32, 103, after='\n'.join(hand + gen))
fix('verifier-synthesis finding 7 (S11 now defined below)', RM, 109, 109, subs=[("(decision map S11)", "(seam S11)")])
fix('F10 (STATUS cited by item)', RM, 150, 150, subs=[('(`docs/STATUS.md`, "Next: the decision drafts")', '(`docs/STATUS.md`, "The five carry-forwards into the decision drafts")')])
fix('G-IDX-155 (glossary: "48" alone)', RM, 155, 155, subs=[("Use 48 where a mark gives 48 and 54;", "Use N = 48 where a mark gives N = 48 and N = 54;")])
fix('G-IDX-166 (glossary: "server-only" quoted as SEC-SPREAD\'s words)', RM, 166, 166, subs=[("| answered: server-only spread |", "| answered: \"server-only spread\" (the spread is known only to the authority) |")])
fix('F10, row 53, G-IDX-209 (STATUS by item; N = 48)', RM, 208, 214, subs=[
("| OLD-ADR | STATUS:198: the v2 drafts", "| OLD-ADR | STATUS, \"ADR-001..003 below\": the v2 drafts"),
("| ADR-001 | STATUS:199 (authority", "| ADR-001 | STATUS, \"ADR-001 Multiplayer architecture\" (authority"),
("sight at 48)", "sight at N = 48)"),
("| ADR-002 | STATUS:200–201 (modules", "| ADR-002 | STATUS, \"ADR-002 Code structure & build\" (modules"),
("| ADR-003 | STATUS:202 (stay", "| ADR-003 | STATUS, \"ADR-003 Three.js version\" (stay"),
("| SEC-MDL-2 | STATUS:257 (", "| SEC-MDL-2 | STATUS, \"Open bugs and risks\", SEC-MDL-2 ("),
("| CDN-SRI | STATUS:262 (", "| CDN-SRI | STATUS, \"Open bugs and risks\", the item deferred from the gate-check-5 reviews ("),
("| WALL-CLOCK | STATUS:273 (", "| WALL-CLOCK | STATUS, \"Open bugs and risks\", the wall-clock item (")])
fix('addendum (LE-H: the 18 inventory rows)', RM, 225, 225, subs=[("| ADR 0003 and ADR 0005 list the rows their decisions change;",
"| ADR 0003, ADR 0004 and ADR 0005 list the rows their decisions change, each in its \"Legacy inventory rows\" table; the 18 rows no draft had listed (CL5, CC10, GW5–GW8, GW10, GW11, GN2, GE3, GE4, GE7, GE9, GE10, GE14, GE16, GP8, KT5) are now there;")])
fix('F13 (the full name)', RM, 228, 228, subs=[("| the proof test runs", "| the netcode proof test runs")])
fix('verifier-synthesis row 54 (R2-BUD-REC4)', RM, 239, 239, subs=[("- **ADR 0001:** R2-BUD-REC5 and R2-TRN-REC6 (part 10),", "- **ADR 0001:** R2-BUD-REC4 (the input buffer and the clock, parts 5 and 7), R2-BUD-REC5 and R2-TRN-REC6 (part 10),")])
fix('verifier-synthesis row 54 (roof fade beside §6.2)', RM, 244, 244, subs=[("roof fade: LOOK_V2 §4.", "roof fade (5.i): LOOK_V2 §4, beside §6.2.")])
fix('verifier-synthesis row 54 (damage stages only)', RM, 251, 251, after="- **ADR 0004:** three damage stages (part 14): LOOK_V2 §4 only (the structure states themselves serve §6.2, §6.4 and §6.5).")
fix('F12, completeness-4 (plan pointers; local code families)', RM, 261, 263, after=
"""The drafts cite codes from the decision map (`.checkpoints/v2-decision-drafts/engine-architect-decision-map.md`, a
checkpoint file deleted before the PR is marked ready). Their definitions live here from now on. Decision codes such
as TICK-30, SLICE-ORDER or UPGRADE-SCOPE live in `docs/STATUS.md`; GLOSSARY terms in `docs/research/GLOSSARY.md`.
Plan rules, plan steps and the advisor's conditions ("plan rule n", "plan step n", "condition n"): the report's
appendix, `docs/reports/2026-11-01-v2-decision-drafts-appendix.md` §3–§4, quotes them word for word. Codes local to one
draft are not register codes: ADR 0002's gate checks G1–G13, ADR 0004's spike claims S1–S22 and ADR 0006's option
labels (G1–G4, S1–S3, K1–K4, B1–B11); a register code always carries its word ("gap G5", "seam S1").""")
fix('G-IDX-279 (glossary: "tick time" names its kind)', RM, 279, 279, subs=[("| tick time histogram,", "| server tick time histogram,")])
fix('F12 (S1–S22)', RM, 290, 290, subs=[("not ADR 0004's own claim numbers S1–S20)", "not ADR 0004's own claim numbers S1–S22)")])
fix('verifier-synthesis finding 7, F12 (seam S11 defined)', RM, 297, 297, subs=[("| S12 | Security |",
"| S11 | Offline and local play | DESIGN: whether offline play ships, and at the largest mode on the minimum laptop (R1-BND-OQ1, R1-TICK-OQ7, critic rank 15) | 0002 and 0004: the local server from slice 1 and local play with bots in slice 2 (SLICE-ORDER), whatever the answer |\n| S12 | Security |")])
fix('G-IDX-306 (glossary: the visual-effects row)', RM, 306, 306, subs=[("the effects budget row stands", "the visual-effects budget row stands")])
fix('F10 (J2: STATUS items by name)', RM, 333, 333, subs=[("STATUS:257, :262, :273,", "STATUS's SEC-MDL-2, CDN-tag and wall-clock items,")])

note('F1', "Not the critic's wording at SPIKE_PLAN:158: \"content files reaching the Worker in development (PM-0002-31, R1-DATA-OQ8)\" is already at SPIKE_PLAN:160, so the fix drops \"hot reload into the Worker\" and leaves parts 2, 4 and 7 as \"Wasm size and tools\" (ADR 0002 spike item 8, \"Size and tools (parts 2, 4 and 7)\"). Same outcome, no duplicate.")
note('verifier-synthesis row 54, second half', "Applied though skeptic 3 did not re-check it: ADR 0004 at 9a03999, part 14's Need line (0004:395–396) reads \"VISION_V2 §6.2 ..., §6.4 ..., §6.5 ...; the damage stages: LOOK_V2 §4 only\", so only the damage stages belong in the register, as the verifier said.")
note('completeness-15', "The §6 placement is put on the parts' labels in the one diagram (no second diagram), and per-frame edges are added on the page; the page stays at about 137 lines (condition 19). The §6 table stays as the place that names draft and part.")
note('counts', "Open questions at 9a03999: 0001 14, 0002 8, 0003 15, 0004 7 numbered (OQ-0004-5 and -6 now say \"Moved\" to OQ-0005-AB9 and AB10, so 5 open), 0005 34, 0006 12: 90 numbered, 88 open; the prompt's \"89\" did not yet count OQ-0002-8 or the two moved ones. Pass marks: 57, 76, 43, 74, 46, 31 rows (327), each max id equal to its row count.")
