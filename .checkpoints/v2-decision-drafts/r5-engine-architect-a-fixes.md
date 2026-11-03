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

