# stop-engine-architect-a fixes (engine-architect: ADR 0002, ADR 0005 frame/content/asset pipeline, GLOSSARY section 8)


### NOTE H23
no change: docs/adr/0002-code-structure-build-toolchain.md:47-48 ("60 Hz only with the 60 Hz margin of at least 25 %
(ADR 0001 owns it)", linking GLOSSARY §8) states a necessary condition. The advisor's ruling on TICK-60-HEADROOM
(advisor-verdict-r4.md, Rulings, question 2): restatements that say "60 Hz only with the 60 Hz margin" and link §8
(0002:47 named) may stay once §8 is amended; FIX H24 and FIX S23 amend that entry, which now states the full rule and
points to ADR 0001 part 2. The other places in H23 are rust-reviewer's (0004) and infra-engineer's (0006).
END

### NOTE H23-2
no change: not my text. docs/adr/0002-code-structure-build-toolchain.md:1332-1333 sits in "Pass marks" (test-engineer's
text): "No mark here is set at 60 Hz: TICK-30's baseline is 30 Hz, and the 60 Hz margin is ADR 0001's, measured in one
run of the netcode proof test." Not in the map. By TICK-60-SETUP (the Advisor, 2026-11-03: "(a), as an early signal
only. The final 60 Hz call is measured again with the real sim at slice 6.") and the verdict's ruling ("Every line
that says the proof test measures or decides the 60 Hz call changes"), the clause after the comma needs, for example:
"the 60 Hz margin is ADR 0001's: an early signal in wave 1, and the run that decides at slice 6 on the real sim". For
test-engineer.
END

### FIX H24
FILE: docs/research/GLOSSARY.md
BEFORE:
  6.25 ms) and every 60 Hz mark the research sets, as written (R2-BUD-REC6's two shares, R1-TICK-REC2 P1–P3);
  otherwise the tick stays at 30 Hz.
- **Depends on:** TICK-60-HEADROOM (the advisor's reading of 2026-10-31: TICK-30's 25 % is measured against the
  server tick budget at 60 Hz, 8.33 ms), confirmed by Mustafa and the Advisor at the stop.
AFTER:
  A 60 Hz tick needs, in the run that decides, a 60 Hz margin of at least 25 % (a p99 server tick time of at most
  6.25 ms), every 60 Hz mark the research sets, as written (R2-BUD-REC6's two shares, R1-TICK-REC2 P1–P3), and the
  goal budget (ADR 0006 part 8) holding at the rooms per VM measured at 60 Hz (PM-0001-63; ADR 0001 part 2 states the
  full rule); otherwise the tick stays at 30 Hz.
- **Depends on:** TICK-60-HEADROOM (the advisor's reading of 2026-10-31: TICK-30's 25 % is measured against the
  server tick budget at 60 Hz, 8.33 ms), confirmed by Mustafa (2026-11-03) with one addition, in his words: "60 Hz
  also needs the goal budget to hold at the rooms per VM measured at 60 Hz."
END

### FIX S23
FILE: docs/research/GLOSSARY.md
BEFORE:
- **Definition:** 1 − (p99 server tick time at a 60 Hz tick ÷ the server tick budget at 60 Hz, 8.33 ms). It is measured
  in the netcode proof test, in one run (p99s do not add: see time statistics): on one vCPU of the reference server, in
  the stress scene at N = 48 with a snapshot-receiving client on every soldier, over R2-BUD-REC6's 30-minute soak,
  re-flood ticks included. A 60 Hz tick needs a 60 Hz margin of at least 25 % (a p99 server tick time of at most
AFTER:
- **Definition:** 1 − (p99 server tick time at a 60 Hz tick ÷ the server tick budget at 60 Hz, 8.33 ms). Each
  measurement is one run (p99s do not add: see time statistics): on one vCPU of the reference server, in the stress
  scene at N = 48 with a snapshot-receiving client on every soldier, over R2-BUD-REC6's 30-minute soak, re-flood ticks
  included. It is measured in wave 1, after the sim core spike (SPIKE-ORDER, Mustafa, 2026-11-03), as an early signal
  only, and again at slice 6 on the real sim, the run that decides (TICK-60-SETUP, the Advisor, 2026-11-03: "(a), as
  an early signal only. The final 60 Hz call is measured again with the real sim at slice 6.").
END

### FIX N8
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
     simulator on simulated time behind the thin transport interface (per direction, half the profile's round trip each
     way, proposed by R5; latency, jitter, loss, reordering, duplication, a rate limit, and a packet-fate log), running
     the real server code and headless bot clients in one process; L2, an impairment wrapper around the server's UDP
     socket under the real quinn endpoint for real browsers over WebTransport, with a delaying TCP proxy for WebSocket.
AFTER:
     simulator on simulated time behind the thin transport interface (latency, jitter, loss, reordering, duplication, a
     rate limit, and a packet-fate log; per direction, latency and jitter split half each way and loss is as the
     profile defines it, ADR 0001 part 13: the Advisor's answer to OQ-0001-16, NET-PROFILES, 2026-11-03, where R5 had
     proposed the round trip's half split), running the real server code and headless bot clients in one process; L2,
     an impairment wrapper around the server's UDP socket under the real quinn endpoint for real browsers over
     WebTransport, with a delaying TCP proxy for WebSocket.
END

### FIX O15
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
**Author:** engine-architect (the "Pass marks" table: test-engineer, plan step 4) · **Reviewed by:** (filled in at review) · **Signed off by Mustafa:** no
AFTER:
**Author:** engine-architect (the "Pass marks" table: test-engineer, plan step 4) ·
**Reviewed by:** (filled in at review) ·
**Signed off by Mustafa:** as the basis for the spikes (2026-11-03, SPIKE-ORDER (b)); not yet accepted
END

### FIX O18
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
12. **Its spike: the Rust ↔ TypeScript boundary spike**, with the netcode proof test and before the foundation, since
    its result fixes the skeleton the foundation builds; what it must prove is below. The foundation itself is this
AFTER:
12. **Its spike: the Rust ↔ TypeScript boundary spike**, in wave 1 after the netcode proof test, the sim core spike and
    the 60 Hz run, one heavy spike at a time (SPIKE-ORDER (b), Mustafa, 2026-11-03), and before the foundation, since
    its result fixes the skeleton the foundation builds; what it must prove is below. The foundation itself is this
END

### FIX O18-2
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
**The Rust ↔ TypeScript boundary spike** (throwaway, in `spikes/`, SPIKE-SCOPE (a)) runs with the netcode proof test,
before the foundation, and shares its client harness and stand-in sim. Its results go into this record before Mustafa
accepts it. The candidate marks below are the research's (cited) or are marked proposed with the reason; test-engineer
sets the final table.
AFTER:
**The Rust ↔ TypeScript boundary spike** (throwaway, in `spikes/`, SPIKE-SCOPE (a)) runs in wave 1, one heavy spike
at a time, after the netcode proof test, the sim core spike and the 60 Hz run (SPIKE-ORDER (b), Mustafa, 2026-11-03),
and before the foundation. It shares the netcode proof test's client harness and stand-in sim, and the sim core
spike's sim, which exists by then, serves the marks that need the real sim (the advisor's reading, 2026-11-03). Its
results go into this record before Mustafa accepts it. The candidate marks below are the research's (cited) or are
marked proposed with the reason; test-engineer sets the final table.
END

### FIX O18-3
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
0. **The drafts approved** by Mustafa and the Advisor; ADR 0003 names the minimum laptop.
1. **The spikes before the foundation** (`docs/SPIKE_PLAN.md` orders them): the netcode proof test (ADR 0001) and,
   beside it, this record's boundary spike, sharing the client harness. *Done when:* the boundary spike's marks are
   recorded in this record, OQ-0002-3 is answered, and OQ-0002-1 and OQ-0002-4 have their spike results.
AFTER:
0. **The drafts signed off** by Mustafa as the basis for the spikes (2026-11-03, SPIKE-ORDER (b)); ADR 0003 names the
   minimum laptop.
1. **The spikes before the foundation** (`docs/SPIKE_PLAN.md` orders them), one heavy spike at a time (SPIKE-ORDER (b),
   Mustafa, 2026-11-03): the netcode proof test (ADR 0001) first, then the sim core spike and the 60 Hz run, then this
   record's boundary spike, which shares the proof test's client harness. *Done when:* the boundary spike's marks are
   recorded in this record, OQ-0002-3 is answered, and OQ-0002-1 and OQ-0002-4 have their spike results.
END

### FIX O18-4
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
   R1-TICK-REC2's P2 with the stand-in load; if the timer pacing fails its jitter mark, W3 is measured on an isolated
   build (OQ-0002-4).
AFTER:
   R1-TICK-REC2's P2 with the sim core spike's sim, which runs before this spike (SPIKE-ORDER (b)); if the timer
   pacing fails its jitter mark, W3 is measured on an isolated build (OQ-0002-4).
END

### NOTE O18-5
no change: not my text. In ADR 0002's "Pass marks" (test-engineer's), two clauses assume the boundary spike may run
before the sim core spike's sim exists: PM-0002-28 (0002:1236) "the sim core spike's sim in this Worker once it
exists" and "with the stand-in sim: reported only", and the BX-P key (0002:1164-1166) "the real sim's cost is unknown
until ADR 0004's sim core spike; once that sim runs in this Worker, the BX rows run again with it". Under SPIKE-ORDER
(b) the sim core spike runs before the boundary spike, so the sim exists by then (the advisor's reading, 2026-11-03:
"its 'stand-in until then' clause may go"). Optional, for test-engineer; FIX O18-2 and O18-4 say so in my text.
END

### FIX M22
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
     holds no token for it (timings on shared runners are not hardware results): it runs on a CCX23 created per bench
     session in the spikes' own Hetzner project and deleted after it, ADR 0006 part 8's cost line (proposed there: 20
     bench hours a month, 20 × 0.1386 = 2.77 EUR, R6-HOST-F15, r6 lines 150–158), and who creates and deletes it
     without a session reading a secret is OQ-0006-17; the 3-hour soak before each playtest uses the same path. No
     bench crate, coverage tool or cargo-nextest is adopted: none was read.
AFTER:
     holds no token for it (timings on shared runners are not hardware results): it runs on a CCX23 created per bench
     session in the spikes' own Hetzner project and deleted after it, ADR 0006 part 8's cost line (proposed there: 20
     bench hours a month, 20 × 0.1386 = 2.77 EUR, R6-HOST-F15, r6 lines 150–158). Who creates and deletes it is
     answered (SPIKE-MACHINES, Mustafa, 2026-11-03; OQ-0006-17): "I create and delete the VMs myself for each run;
     sessions get SSH access only and no Hetzner token until the security baseline"; that this covers the bench and the
     3-hour soaks as well as the spikes is the advisor's reading (2026-11-03). The 3-hour soak before each playtest uses
     the same path. No bench crate, coverage tool or cargo-nextest is adopted: none was read.
END

### FIX E1
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
  - Decider: the Advisor, at the stop.
AFTER:
  - Decider: the Advisor, at the stop.
  - Answered by the Advisor (2026-11-03): "(a)." Option (a) is decided: rule 3 allows it in such cases, so the
    `room`, `client-endpoint` and `navbake` crates stay proposed, labelled "not researched", and the first foundation
    milestone that builds them is their test.
END

### FIX E3
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
  would risk shipping in the player's Wasm. No research compared other placements, so the first foundation milestone
  that builds them is also their test; whether rule 3 allows proposing them at all is OQ-0002-8.
AFTER:
  would risk shipping in the player's Wasm. No research compared other placements, so the first foundation milestone
  that builds them is also their test; rule 3 allows proposing them (OQ-0002-8, answered (a) by the Advisor,
  2026-11-03).
END

### FIX E2
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
     three are proposed (not researched): no research Options section offers a crate for the room logic, the client
     endpoint or the bake, and SLICE-ORDER condition 1 and R4-XT-OQ5 leave them to this record; whether plan rule 3
     allows that is OQ-0002-8, for the Advisor at the stop. The supervisor's crate (slice 3) is beyond R5's list too,
     but the research asks for it: R6-HOST-REC2 puts a small supervisor on every VM (r6-infrastructure.md lines
AFTER:
     three are proposed (not researched): no research Options section offers a crate for the room logic, the client
     endpoint or the bake, and SLICE-ORDER condition 1 and R4-XT-OQ5 leave them to this record; plan rule 3 allows
     that (OQ-0002-8, answered (a) by the Advisor, 2026-11-03). The supervisor's crate (slice 3) is beyond R5's list
     too, but the research asks for it: R6-HOST-REC2 puts a small supervisor on every VM (r6-infrastructure.md lines
END

### FIX E4
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
| R4-XT-OQ5 | answered: the bake in its own `navbake` crate (proposed, not researched: OQ-0002-8), never linked into `sim-wasm`; the map checks as G13 | parts 3 and 9 |
AFTER:
| R4-XT-OQ5 | answered: the bake in its own `navbake` crate (proposed, not researched; allowed by OQ-0002-8 (a), the Advisor, 2026-11-03), never linked into `sim-wasm`; the map checks as G13 | parts 3 and 9 |
END

### FIX E4-2
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
| R5-LAYOUT-REC2 | changed: the `room`, `client-endpoint` and `navbake` crates added (proposed, not researched: OQ-0002-8), and the supervisor's crate at slice 3 (R6-HOST-REC2's supervisor, ADR 0006 part 5); nothing imports `spikes/` | part 3: SLICE-ORDER condition 1 runs the local server in the Worker; ADR 0001's network marks run the client endpoint natively; R4-XT-OQ5 |
AFTER:
| R5-LAYOUT-REC2 | changed: the `room`, `client-endpoint` and `navbake` crates added (proposed, not researched; allowed by OQ-0002-8 (a), the Advisor, 2026-11-03), and the supervisor's crate at slice 3 (R6-HOST-REC2's supervisor, ADR 0006 part 5); nothing imports `spikes/` | part 3: SLICE-ORDER condition 1 runs the local server in the Worker; ADR 0001's network marks run the client endpoint natively; R4-XT-OQ5 |
END

### FIX E5
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
| **R5-LAYOUT-REC2 plus an I/O-free `room` crate, a `client-endpoint` crate with no bindings and a `navbake` crate above `sim` (chosen)** | low–medium: three more crates and three more dependency checks | – | the player's Wasm carries neither the bake nor the server's I/O | the same decoders and room checks run locally and online | none | new | one room logic for the local server and the room process; one client endpoint for the Worker and the headless clients; the bake shared by CI, map save and the editor | not researched as a placement (no Options section offers a crate for the room logic, the client endpoint or the bake; SLICE-ORDER condition 1 and R4-XT-OQ5 leave it to this record): proposed only for the Advisor's ruling at the stop (OQ-0002-8) |
AFTER:
| **R5-LAYOUT-REC2 plus an I/O-free `room` crate, a `client-endpoint` crate with no bindings and a `navbake` crate above `sim` (chosen)** | low–medium: three more crates and three more dependency checks | – | the player's Wasm carries neither the bake nor the server's I/O | the same decoders and room checks run locally and online | none | new | one room logic for the local server and the room process; one client endpoint for the Worker and the headless clients; the bake shared by CI, map save and the editor | not researched as a placement (no Options section offers a crate for the room logic, the client endpoint or the bake; SLICE-ORDER condition 1 and R4-XT-OQ5 leave it to this record): proposed, and allowed by the Advisor's ruling at the stop (OQ-0002-8 (a), 2026-11-03) |
END

### FIX O15-2
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
**Author:** engine-architect (frame, content data, the asset pipeline) · game-designer (the ability and effect system) · graphics-engineer (the asset pipeline's GPU rules) · test-engineer (the "Pass marks" table, plan step 4) · **Reviewed by:** (filled in at review) · **Signed off by Mustafa:** no
AFTER:
**Author:** engine-architect (frame, content data, the asset pipeline) · game-designer (the ability and effect
system) · graphics-engineer (the asset pipeline's GPU rules) · test-engineer (the "Pass marks" table, plan step 4) ·
**Reviewed by:** (filled in at review) ·
**Signed off by Mustafa:** as the basis for the spikes (2026-11-03, SPIKE-ORDER (b)); not yet accepted
END

### NOTE O22
no change: docs/adr/0005-content-abilities-assets.md:2364-2365 ("The netcode proof test runs first; it needs none of
this record's machinery") agrees with SPIKE-ORDER (b) (Mustafa, 2026-11-03: "the netcode proof test first"). This
record's own spikes are not in wave 1: the content and ability spike runs after the sim core spike and before slice
1 (0005:2103), and the look proof test's asset share follows ADR 0003's place, so SPIKE-ORDER moves no other
migration line here (FIX L7-3 changes the Assets line, for LOOK-TEST-INPUTS).
END

### FIX L7
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
- **ASSET-RESEARCH (a)** (the advisor's ruling, 2026-10-31): the asset part takes LOOK_V2 §8's flow as given and
  proposes only what R1-DATA, R3-CHAR and R3-BUDGET support; compression formats, the LOD tool, export checks, the
  material-slot renaming step and the per-unit budget numbers are open questions marked "not researched", grouped
  under one research unit, the asset-pipeline research (graphics-engineer, verified like R3, in its own daytime
  session), which `docs/SPIKE_PLAN.md` places before the asset spike.
AFTER:
- **ASSET-RESEARCH (a)** (the advisor's ruling, 2026-10-31): the asset part takes LOOK_V2 §8's flow as given and
  proposes only what R1-DATA, R3-CHAR and R3-BUDGET support; compression formats, the LOD tool, export checks, the
  material-slot renaming step and the per-unit budget numbers are open questions marked "not researched", grouped
  under one research unit, the asset-pipeline research (graphics-engineer, verified like R3, in its own daytime
  session), which `docs/SPIKE_PLAN.md` places before the asset spike; its export and slot-naming part (OQ-0005-AS3,
  AS4) runs earlier, before the look proof test (LOOK-TEST-INPUTS, below).
- **LOOK-TEST-INPUTS** (Mustafa, 2026-11-03): "yes, the asset research's export and slot-naming part runs before the
  look proof test. Route: Blender (Spline's skins and named slots are unproven). I make the test models with AI help
  in Blender; blockouts with a skin, named colour slots and LOOK_V2's proportions are enough." LOOK_V2 §8's flow is
  unchanged; Blender is the route for the look proof test's models and for that research part's export (the
  advisor's reading, 2026-11-03).
END

### FIX L7-2
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
**Settled inputs.** LOOK_V2 (authored glTF/GLB models allowed; brief §1). ASSET-RESEARCH (a), the advisor's ruling of
2026-10-31 (plan, "Rulings"; decision map, part j, J4): this part takes LOOK_V2 §8's flow as given and proposes only
what R1-DATA, R3-CHAR and R3-BUDGET support; compression formats, the LOD tool, export checks, the material-slot
renaming step and the per-unit budget numbers are open questions marked "not researched", grouped under one research
unit, the asset-pipeline research (graphics-engineer, verified like R3, in its own daytime session), which
`docs/SPIKE_PLAN.md` places before the asset spike. SPIKE-SCOPE (a).
AFTER:
**Settled inputs.** LOOK_V2 (authored glTF/GLB models allowed; brief §1). ASSET-RESEARCH (a), the advisor's ruling of
2026-10-31 (plan, "Rulings"; decision map, part j, J4): this part takes LOOK_V2 §8's flow as given and proposes only
what R1-DATA, R3-CHAR and R3-BUDGET support; compression formats, the LOD tool, export checks, the material-slot
renaming step and the per-unit budget numbers are open questions marked "not researched", grouped under one research
unit, the asset-pipeline research (graphics-engineer, verified like R3, in its own daytime session), which
`docs/SPIKE_PLAN.md` places before the asset spike, with its export and slot-naming part before the look proof test.
LOOK-TEST-INPUTS (Mustafa, 2026-11-03; the frame's Settled inputs quote it): that timing, and Blender as the route for
the look proof test's models, which Mustafa makes with AI help as blockouts with a skin, named colour slots and
LOOK_V2's proportions. SPIKE-SCOPE (a).
END

### FIX L7-3
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
- **Assets:** the look proof test uses plain glTF/GLB exports (the GPU section's part 8); the asset-pipeline research,
  then the asset spike, come before the first authored model enters the game (AP-1 to AP-6); until OQ-0005-AP2 is
  answered, buildings and props keep the ported kit beside authored assets (AP-7).
AFTER:
- **Assets:** the look proof test uses plain glTF/GLB exports (the GPU section's part 8) of the models Mustafa makes in
  Blender, after the asset-pipeline research's export and slot-naming part (LOOK-TEST-INPUTS); the rest of that
  research, then the asset spike, come before the first authored model enters the game (AP-1 to AP-6); until
  OQ-0005-AP2 is answered, buildings and props keep the ported kit beside authored assets (AP-7).
END

### FIX L9
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
    Spline's export and the tools for step 4 can do, and that answer. Research ids: none.
AFTER:
    Spline's export and the tools for step 4 can do, and that answer. Research ids: none.
  - Since LOOK-TEST-INPUTS (Mustafa, 2026-11-03: "Route: Blender (Spline's skins and named slots are unproven)"),
    the look proof test's models are made in Blender, so the source files this question places include their Blender
    files; Blender is also the route for the research's export part (the advisor's reading, 2026-11-03). This
    question keeps its place, with the rest of the research, before the asset spike.
END

### FIX C1
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
    Before the fix round AP-2 and CD-13 put these references on the simulation definitions; now AP-2, AP-3 and CD-13
    state option (b) as a proposal pending this question and decide nothing.
AFTER:
    Before the fix round AP-2 and CD-13 put these references on the simulation definitions; the fix round made AP-2,
    AP-3 and CD-13 state option (b) as a proposal pending this question, deciding nothing.
END

### FIX C1-2
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
  - Proposed: (b) for definitions, for the reasons under Question F2, pending this question.
AFTER:
  - Answered by the Advisor (2026-11-03): "the client's data, keyed by the same id; the content check fails if a
    simulation id has no client entry."
  - Decided: (b) for definitions, as proposed before the stop for the reasons under Question F2; AP-2, AP-3 and CD-13
    now state it as decided. The answer adds one whole-set check (CD-2), which binds the kinds the client presents;
    kinds with nothing to present need no entry (the advisor's reading, 2026-11-03). CD-2 names both lists.
END

### FIX C3
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
one Appendix A. Where two sections disagreed, the frame asks the question (OQ-0005-1, OQ-0005-2); after the fix
round no section answers either, and each points to it.
AFTER:
one Appendix A. Where two sections disagreed, the frame asks the question (OQ-0005-1, OQ-0005-2); after the fix
round no section answered either, and each points to it. The Advisor answered OQ-0005-1 at the stop (2026-11-03).
END

### FIX C4
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
    whole-set checks (every reference resolves; every objective a mode needs is in its map; no fallback map, weapon or
    model) → a frozen content set with its hash. Errors name the file, with line and column for bad syntax and the
AFTER:
    whole-set checks (every reference resolves; every objective a mode needs is in its map; every id of a presented
    kind has its client entry, below; no fallback map, weapon or model) → a frozen content set with its hash. Errors
    name the file, with line and column for bad syntax and the
END

### FIX C4-2
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
    (proposed); which row gives way when a pool is full is ADR 0004's OQ-0004-11, and AB-1 holds the game-designer's
    proposal per kind for it.
AFTER:
    (proposed); which row gives way when a pool is full is ADR 0004's OQ-0004-11, and AB-1 holds the game-designer's
    proposal per kind for it.
  - The client entries (OQ-0005-1, answered by the Advisor, 2026-11-03: "the client's data, keyed by the same id; the
    content check fails if a simulation id has no client entry"): the rule binds the kinds the client presents (the
    advisor's reading, 2026-11-03). Each id of a presented kind needs a client definition keyed by that id, or the
    whole-set check fails, naming the file: a unit (its model, AP-2); a vehicle (its model, AP-2); a weapon (its feel
    kit and sockets, AB-3 and CD-14, and its model, AP-2); a faction (its colours and its uniform models, CD-13 and
    AP-2). Kinds with nothing to present need no entry: of the ability section's kinds (AB-4), the tag registry, the
    attribute sets, effects, abilities, upgrade tracks and the drop and pickup values; and modes, the rule tables, the
    movement data and the moment rules (CD-6, CD-12). A map needs none either: the map document holds both sides
    itself, its references to assets included (CD-11; EDITOR_V2 §2, rule 1), and building parts and props are
    entities in it, not kinds of their own (AP-7). A kind that gains a presentation side joins the first list in the
    same change.
END

### FIX C4-3
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
   Proposed addition (reason: R1-RR-OQ2): the corpus includes, for every bounded collection, a file one element over
   its bound, refused 100 % of the time. Zero panics is also R1-RR-OQ3's mark for the loader. The spike records the
AFTER:
   Proposed addition (reason: R1-RR-OQ2): the corpus includes, for every bounded collection, a file one element over
   its bound, refused 100 % of the time. Proposed (reason: OQ-0005-1's answer, 2026-11-03): for each presented kind
   CD-2 names, a set with one definition whose client entry is missing, refused 100 % of the time. Zero panics is also
   R1-RR-OQ3's mark for the loader. The spike records the
END

### FIX C5
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
    models are assets named by id (AP-2), and its colours reach the assets' faction colour slots (AP-3; the GPU
    section's part 2). Where its colours and model ids sit is OQ-0005-1: proposed, pending it, in a client definition
    keyed by the faction's id, as the feel kit is keyed by the weapon's (CD-14), so the faction's simulation definition
    holds no presentation field. A faction variant is a full definition of its own that references shared definitions
AFTER:
    models are assets named by id (AP-2), and its colours reach the assets' faction colour slots (AP-3; the GPU
    section's part 2). Its colours and model ids sit in a client definition keyed by the faction's id, as the feel kit
    is keyed by the weapon's (CD-14), so the faction's simulation definition holds no presentation field (OQ-0005-1,
    answered by the Advisor, 2026-11-03), and a faction without that entry fails the whole-set check (CD-2). A faction
    variant is a full definition of its own that references shared definitions
END

### FIX C2
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
| OQ-0005-1 | Do presentation references sit on the simulation definitions or in client definitions keyed by the same id? (proposed: client definitions) | the Advisor, at the stop review | this frame |
AFTER:
| OQ-0005-1 | Do presentation references sit on the simulation definitions or in client definitions keyed by the same id? (answered by the Advisor, 2026-11-03: client definitions keyed by the same id; an id of a presented kind with no client entry fails the content check, CD-2) | the Advisor, at the stop review | this frame |
END

### FIX C2-2
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
Every open question of the four sections, one line each, with its decider and the section that holds its full text
(the question, why it matters, the options and the research ids), then the frame's own four. Mustafa's questions carry
no pick; where a section proposes an answer, the line says so.
AFTER:
Every open question of the four sections, one line each, with its decider and the section that holds its full text
(the question, why it matters, the options and the research ids), then the frame's own four. Mustafa's questions carry
no pick; where a section proposes an answer, the line says so, and an answered question names who answered it and
when.
END

### FIX C6
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
  - What: each unit, vehicle, weapon, faction uniform, building part and prop names its model by an asset id. Where
    that id sits is OQ-0005-1: proposed, pending it, in a client definition keyed by the simulation definition's id,
    as the feel kit is (CD-14), so no simulation definition holds an asset id (CD-6); the map document holds its own
    references either way (CD-11). One asset list in the content set (client content, CD-6) holds for each asset: its
    id; its file name with its hash; its material slots, each with its class (AP-3); its LOD levels, if it has any (the
    GPU section's part 5); and its counts of triangles, materials and textures, which the budget check reads (AP-5).
    The loader's whole-set checks (CD-2) refuse a reference to an asset the list does not hold, and a slot whose class
    is not in the class list, naming the file and the field; nothing falls back to a default model. Who writes the
    list (the automated step or a person), and whether the gate recounts a file or trusts its entry, is OQ-0005-AP1.
AFTER:
  - What: each unit, vehicle, weapon, faction uniform, building part and prop names its model by an asset id. For a
    definition, that id sits in a client definition keyed by the simulation definition's id, as the feel kit is
    (CD-14), so no simulation definition holds an asset id (CD-6; OQ-0005-1, answered by the Advisor, 2026-11-03); the
    map document holds its own references (CD-11). One asset list in the content set (client content, CD-6) holds for
    each asset: its id; its file name with its hash; its material slots, each with its class (AP-3); its LOD levels, if
    it has any (the GPU section's part 5); and its counts of triangles, materials and textures, which the budget check
    reads (AP-5). The loader's whole-set checks (CD-2) refuse a reference to an asset the list does not hold, a slot
    whose class is not in the class list, and an id of a presented kind with no client entry (OQ-0005-1's rule), naming
    the file and the field; nothing falls back to a default model. Who writes the list (the automated step or a
    person), and whether the gate recounts a file or trusts its entry, is OQ-0005-AP1.
END

### FIX C6-2
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
    0003 (interface I-8). Adding a class is a content change reviewed together with the ADR 0003 material it needs. A
    faction's colours reach its faction colour slots from the faction's data (CD-13; where those colours sit is
    OQ-0005-1). The table says which slots exist; how a class looks is ADR 0003's and Mustafa's (LOOK_V2).
AFTER:
    0003 (interface I-8). Adding a class is a content change reviewed together with the ADR 0003 material it needs. A
    faction's colours reach its faction colour slots from the faction's client definition (CD-13; OQ-0005-1, answered
    by the Advisor, 2026-11-03). The table says which slots exist; how a class looks is ADR 0003's and Mustafa's
    (LOOK_V2).
END

### FIX C7
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
| **b. A client definition keyed by the simulation definition's id, as the feel kit is (AB-3, CD-14)** (proposed, pending OQ-0005-1) | Low: one more file for each definition with a presentation side | files kept in step | – | presentation only; no simulation type holds a presentation field, so CD-6's split holds by kind | an art or feel change touches only client files | new; the feel kit already works this way |
AFTER:
| **b. A client definition keyed by the simulation definition's id, as the feel kit is (AB-3, CD-14)** (chosen: OQ-0005-1, answered by the Advisor, 2026-11-03) | Low: one more file for each definition with a presentation side | files kept in step | – | presentation only; no simulation type holds a presentation field, so CD-6's split holds by kind | an art or feel change touches only client files | new; the feel kit already works this way |
END

### FIX C7-2
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
- Both options apply R1-DATA-REC8's simulation and client groups (r1, lines 2596–2598). The map document holds both
  sides in one document either way (EDITOR_V2 §2, rule 1). Proposed: (b) for definitions, because the feel kit already
  takes it and a weapon's presentation (feel kit, sockets, model) then sits in one place. The question stays open for
  the Advisor as OQ-0005-1; AP-2, AP-3 and CD-13 state (b) as the proposal pending it and decide nothing.
AFTER:
- Both options apply R1-DATA-REC8's simulation and client groups (r1, lines 2596–2598). The map document holds both
  sides in one document either way (EDITOR_V2 §2, rule 1). Decided: (b) for definitions (the Advisor's answer to
  OQ-0005-1, 2026-11-03), as proposed before the stop, because the feel kit already takes it and a weapon's
  presentation (feel kit, sockets, model) then sits in one place; the answer adds one whole-set check, an id of a
  presented kind with no client entry fails (CD-2 names the kinds), and AP-2, AP-3 and CD-13 state (b) as decided.
END

### FIX C7-3
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
- **Why one model:** a unit names its weapons, abilities and track, and its model is named under the unit's id (AP-2;
  where that reference sits is OQ-0005-1); a faction names its units
AFTER:
- **Why one model:** a unit names its weapons, abilities and track, and its model is named under the unit's id (AP-2:
  in the unit's client definition, OQ-0005-1); a faction names its units
END

### FIX A4
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
| OQ-0005-AB12 | Is tranek's write-up accepted as the GAS reference, or does a verifier read Epic's documentation first? (proposed: accept it) | the Advisor | The ability and effect system |
AFTER:
| OQ-0005-AB12 | Is tranek's write-up accepted as the GAS reference, or does a verifier read Epic's documentation first? (answered by the Advisor, 2026-11-03: "tranek's write-up is accepted for concepts; any specific claim a record relies on is checked against Epic's documentation once it can be read") | the Advisor | The ability and effect system |
END

### FIX A4-2
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
| R1-ABL-OQ1 | open: OQ-0005-AB12 | the Advisor decides the reference |
AFTER:
| R1-ABL-OQ1 | answered: OQ-0005-AB12 (the Advisor, 2026-11-03) | tranek's write-up is accepted for concepts; any specific claim a record relies on is checked against Epic's documentation once it can be read |
END

### FIX B3
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
- The ability items 1 or 2 fail for one mechanic: it becomes a new reviewed operation; if many fail, those mechanics
  fall back to option A4 with E1's tables kept, and OQ-0005-AB13 reopens scripting.
AFTER:
- The ability items 1 or 2 fail for a mechanic: it becomes a new reviewed operation, which still counts as built from
  data (the advisor's reading, 2026-11-03; AB-6). Scripting (option A3) comes back only when a designed ability can't
  be built from data (OQ-0005-AB13, answered by the Advisor, 2026-11-03): one designed ability that needs
  ability-specific code no operation can express (the advisor's reading, 2026-11-03).
END

### FIX B4
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
- **Revisit when:** content's load time misses its budget, or content's share takes the Wasm file past the limit ADR
  0002 sets (OQ-0005-CD2, option
  D), or garde fails on wasm32 (CD-2); the ability marks 1 or 2 fail (OQ-0005-AB13); track content exists (the stress scene's active effects
  per soldier restated with ADR 0004, AB-10); ABL-PREDICT is answered; Mustafa and the Advisor pick the mid-match swap
AFTER:
- **Revisit when:** content's load time misses its budget, or content's share takes the Wasm file past the limit ADR
  0002 sets (OQ-0005-CD2, option D), or garde fails on wasm32 (CD-2); a designed ability can't be built from data
  (OQ-0005-AB13, answered by the Advisor, 2026-11-03); track content exists (the stress scene's active effects per
  soldier restated with ADR 0004, AB-10); ABL-PREDICT is answered; Mustafa and the Advisor pick the mid-match swap
END

### NOTE B4-2
no change: not my text. docs/adr/0005-content-abilities-assets.md:2263 sits in "Pass marks" (test-engineer's): its
failure rule reads "a new reviewed operation for one mechanic that fails PM-0005-18 to -21, option A4 with E1's tables
if many fail (OQ-0005-AB13)". The Advisor's answer ("scripting comes back only when a designed ability can't be built
from data") and the verdict's ruling ("'Needed often' and 'many fail' are no longer triggers: align the rollback") ask
that "option A4 with E1's tables if many fail" go, for example: "a new reviewed operation for a mechanic that fails
PM-0005-18 to -21; scripting only when a designed ability can't be built from data (OQ-0005-AB13)". FIX B3 aligns the
frame's Rollback and FIX B4 its "Revisit when". For test-engineer.
END

### FIX B5
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
| OQ-0005-AB13 | When does scripting (option A3) come back? (proposed: if the spike's ability mark 1 or 2 fails, or new operations are needed often; the talk is read when a copy appears) | the Advisor | The ability and effect system |
AFTER:
| OQ-0005-AB13 | When does scripting (option A3) come back? (answered by the Advisor, 2026-11-03: "scripting comes back only when a designed ability can't be built from data"; a new reviewed operation counts as built from data, and reading the talk stays a weighed option with nothing scheduling it: the advisor's reading, 2026-11-03) | the Advisor | The ability and effect system |
END

### FIX B5-2
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
| R1-ABL-OQ2 | open: OQ-0005-AB13 | the source, or the spike's trigger |
AFTER:
| R1-ABL-OQ2 | answered: OQ-0005-AB13 (the Advisor, 2026-11-03) | scripting comes back only when a designed ability can't be built from data; reading the talk stays a weighed option, unscheduled (the advisor's reading) |
END

### FIX B5-3
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
| R1-ABL-OQ9 | answered: it matters only if option A3 returns; A3 is not adopted (AB-1), and OQ-0005-AB13 names the trigger | AB-1; Options, Question A |
AFTER:
| R1-ABL-OQ9 | answered: it matters only if option A3 returns; A3 is not adopted (AB-1), and OQ-0005-AB13's answer sets when it returns: only when a designed ability can't be built from data (the Advisor, 2026-11-03) | AB-1; Options, Question A |
END

### NOTE X5
no change: OQ-0001-14 is answered (the Advisor, 2026-11-03: "yes, a VISION_V2 §3 or §4 trace counts, marked as
such"), and my need lines that rest on §3 or §4 alone are already marked so: ADR 0005 :134 ("the quality-tier table:
VISION_V2 §4 only"), :144 ("VISION_V2 §3 and §4 only"), :514 ("VISION_V2 §4 only (ADR 0003's)"). ADR 0002's need
lines that name §3 or §4 (:275, :408, :477, :639) pair them with a §6 item, GLOSSARY §8's foundation or SLICE-ORDER
(:639's G11, "§4's fairness target", sits under "GLOSSARY §8, the foundation only", which README's "Needs outside
VISION_V2 §6" lists). The verdict's ruling (OQ-0001-14 (a)): "No need line changes." 0005:1034 is game-designer's.
END

### NOTE M24
no change: docs/research/GLOSSARY.md:707-712 ("Game-server certificate": "Development and preview servers use
self-signed, pinned certificates valid at most 14 days") is base text in lines 1-743, which keep their bytes (the
write guard's prefix check), and the verdict keeps it a NOTE (condition 3, last item; ruling GLOSSARY-8): "The older
entry at lines 707-712 (map M24) stays a NOTE in the report's open findings". What differs since SPIKE-MACHINES
(Mustafa, 2026-11-03: "Certificates: a private hash-pinned certificate for WebTransport; for the WebSocket leg, a
domain I register"): the netcode proof test's WebSocket leg, on a development server, presents a trusted certificate
on that domain, not a self-signed pinned one. For the report's open findings (the session); for the Advisor.
END
