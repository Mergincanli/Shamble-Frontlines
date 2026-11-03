# r5 engine-architect-a fixes

### FIX e1-F2
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
and before the foundation. It shares the netcode proof test's client harness and stand-in sim, and the sim core
spike's sim, which exists by then, serves the marks that need the real sim (the advisor's reading, 2026-11-03). Its
results go into this record before Mustafa accepts it. The candidate marks below are the research's (cited) or are
marked proposed with the reason; test-engineer sets the final table.
AFTER:
and before the foundation. It shares the netcode proof test's client harness and stand-in sim, and the sim core spike's
sim, which exists by then, serves the marks that need a sim beyond the stand-in (the advisor's reading, 2026-11-03).
Its results go into this record before Mustafa accepts it. The candidate marks below are the research's (cited) or are
marked proposed with the reason; test-engineer sets the final table.
END

### FIX LOOK_V2-8
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
  in Blender; blockouts with a skin, named colour slots and LOOK_V2's proportions are enough." LOOK_V2 §8's flow is
  unchanged; Blender is the route for the look proof test's models and for that research part's export (the
  advisor's reading, 2026-11-03).
AFTER:
  in Blender; blockouts with a skin, named colour slots and LOOK_V2's proportions are enough." Blender is the route
  for the look proof test's models and for that research part's export (the advisor's reading, 2026-11-03).
  LOOK_V2 §8's flow stays as written (LOOK_V2-§8, Mustafa, 2026-11-03): "yes, keep it as it is, with Blender for the
  test models only. When the asset research's export part reports, §8 says "Spline or Blender"."
END

### FIX e2b-F2
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
Counts: adopted 5 · changed 6 · answered 8 · open 9 · spike 6 · rejected 0 (34 rows).
AFTER:
Counts: adopted 5 · changed 6 · answered 10 · open 7 · spike 6 · rejected 0 (34 rows).
END

### FIX e2b-F3a
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
  presentation (feel kit, sockets, model) then sits in one place; the answer adds one whole-set check, an id of a
  presented kind with no client entry fails (CD-2 names the kinds), and AP-2, AP-3 and CD-13 state (b) as decided.
AFTER:
  presentation (feel kit, sockets, model) then sits in one place; the answer adds one whole-set check: an id of a
  presented kind (CD-2 names the kinds) with no client entry fails (the advisor's reading, 2026-11-03); AP-2, AP-3 and
  CD-13 state (b) as decided.
END

### FIX e2b-F3b
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
| OQ-0005-1 | Do presentation references sit on the simulation definitions or in client definitions keyed by the same id? (answered by the Advisor, 2026-11-03: client definitions keyed by the same id; an id of a presented kind with no client entry fails the content check, CD-2) | the Advisor, at the stop review | this frame |
AFTER:
| OQ-0005-1 | Do presentation references sit on the simulation definitions or in client definitions keyed by the same id? (answered by the Advisor, 2026-11-03: client definitions keyed by the same id; an id of a presented kind with no client entry fails the content check (the advisor's reading, 2026-11-03), CD-2) | the Advisor, at the stop review | this frame |
END

### FIX e2b-F5
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
the look proof test's models, which Mustafa makes with AI help as blockouts with a skin, named colour slots and
LOOK_V2's proportions. SPIKE-SCOPE (a).
AFTER:
the look proof test's models, which Mustafa makes with AI help, where blockouts with a skin, named colour slots and
LOOK_V2's proportions are enough. SPIKE-SCOPE (a).
END

### FIX e2b-F6
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
| R1-ABL-OQ2 | answered: OQ-0005-AB13 (the Advisor, 2026-11-03) | scripting comes back only when a designed ability can't be built from data; reading the talk stays a weighed option, unscheduled (the advisor's reading) |
AFTER:
| R1-ABL-OQ2 | answered: OQ-0005-AB13 (the Advisor, 2026-11-03) | scripting comes back only when a designed ability can't be built from data; reading the talk stays a weighed option, unscheduled (the advisor's reading, 2026-11-03) |
END

### FIX e2b-F7
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
  - What, *client content* (presentation only; the sim never reads it): the feel kit per weapon and the quality-tier
    table (CD-14), the asset list and the material-slot classes (the asset pipeline section, AP-2 and AP-3), and the
    look values ADR 0003 tunes in its look proof test (CD-10).
AFTER:
  - What, *client content* (presentation only; the sim never reads it): the client entries keyed by simulation ids
    (CD-2; a weapon's feel kit is part of its entry, CD-14); the quality-tier table (CD-14); the asset list and the
    material-slot classes (the asset pipeline section, AP-2 and AP-3); and the look values ADR 0003 tunes in its look
    proof test (CD-10).
END

### NOTE COMMUNITY-SERVERS-0002
no change: (docs/adr/0002-code-structure-build-toolchain.md) ADR 0002 makes the room process need neither the
supervisor nor the lobby to start, so no sentence reads as closing the door and no pointer to OQ-0006-21 goes in. The
`server` crate (0002:192) is "the room process", and its "Depends on" cell names `room`, `protocol`, `sim` and
`content`, no supervisor or lobby crate; its "supervisor hook" is the room's side of ADR 0001's supervisor calls
(0001:676: spawn, health check, exit, record upload), which the supervisor drives, not a condition for starting. The
`supervisor` row (0002:193) "runs room processes as programs and links no `sim`, `room` or `sim-wasm` code", so a room
started from a config file is the same program started a second way (the advisor's ruling (1) on COMMUNITY-SERVERS,
verdict r5). ADR 0002's own gate already runs a local server with neither: G12 (0002:578) and the browser end-to-end
layer (0002:489) run Playwright "with the built client and a local server" over both transport backends, and name no
supervisor or lobby. No lobby crate exists until ADR 0006 decides where the lobby runs (0002:200-201). The certificate
the supervisor holds (0002:682) is the multiplayer preview's arrangement on the staging VM only. Outside this record:
ADR 0001's catalogue passes "the lobby's public key" at spawn (0001:676); that text is netcode-architect's to check
under condition 6.
END

### NOTE COMMUNITY-SERVERS-0005
no change: (docs/adr/0005-content-abilities-assets.md) ADR 0005's mode, content-hash and record text locks none of
his three points out, so no pointer to OQ-0006-21 goes in. (1) "one self-contained program with a config file
(players, length, rules)": a room is "a mode id plus allowed overrides" (0005:241, 0005:1977; R2-ROOM-REC7, ADR
0001's); a mode is one content definition whose fields include team size, the rules table and match length (CD-6,
0005:379-381; I-7, 0005:110); a private room's overrides name tunables from an allow list with the schema's ranges
(CD-9, 0005:450-451). The text says that ADR 0001's `HostCommand` carries the overrides, not that nothing else may,
and what the allow list holds stays a design question (R1-DATA-OQ10). (2) "checks the game's version and content
hash": CD-8 (0005:430-433) has the room name the hash at join and the Worker refuse a mismatch, wherever the room
runs; the version is ADR 0001's `Join`. (3) "marks each match as official or community": CD-8 and CD-12 (0005:436,
0005:499-500) say every match record carries the content hash and the mode id and leave the record's shape to ADR
0006 (R6-TEL-REC1), so such a field is ADR 0006's to add later (OQ-0006-21, infra-engineer). The archive of content
sets follows the records the supervisor uploads (CD-8, 0005:437-439), and only records a production supervisor
uploads enter the store (ADR 0006:1485), which fits "progression can count on official servers only" (the advisor's
ruling on COMMUNITY-SERVERS, verdict r5: "official-only storage keys fit").
END

