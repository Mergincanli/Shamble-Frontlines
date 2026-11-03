# Stop answers: graphics-engineer fixes (ADR 0003, ADR 0005 GPU-rules section)

### FIX O15
FILE: docs/adr/0003-rendering.md
BEFORE:
**Author:** graphics-engineer (the "Pass marks" table: test-engineer, plan step 4) · **Reviewed by:** (filled in at review) · **Signed off by Mustafa:** no
AFTER:
**Author:** graphics-engineer (the "Pass marks" table: test-engineer, plan step 4) ·
**Reviewed by:** (filled in at review) ·
**Signed off by Mustafa:** as the basis for the spikes (2026-11-03, SPIKE-ORDER (b)); not yet accepted
END

### FIX O19
FILE: docs/adr/0003-rendering.md
BEFORE:
Two spikes, throwaway and in `spikes/` (SPIKE-SCOPE (a)); none starts before Mustafa and the Advisor approve the
drafts. `docs/SPIKE_PLAN.md` gives their final names, order and sizes. Every candidate mark below is proposed in the
AFTER:
Two spikes, throwaway and in `spikes/` (SPIKE-SCOPE (a)). "The six drafts are the basis for the spikes" (Mustafa,
2026-11-03, SPIKE-ORDER (b)), and wave 1 runs "one heavy spike at a time": the look proof test comes fourth there,
"once its models exist" (Migration plan, step 2); the render budget bench comes later, before slice 1 (step 4).
`docs/SPIKE_PLAN.md` gives their final names, order and sizes. Every candidate mark below is proposed in the
END

### FIX O19-2
FILE: docs/adr/0003-rendering.md
BEFORE:
2. **The look proof test**, in parallel with the netcode proof test (no shared dependency). *Milestone:* the
   technique table and both ink families measured on both machines; Mustafa's verdict on every LOOK_V2 item and on
   OQ-0003-2, -3, -5, -6, -8, -12, -15, -16, -18 and -19, recorded in this ADR.
AFTER:
2. **The look proof test**, fourth in wave 1, which runs "one heavy spike at a time": after the netcode proof test,
   the sim core spike (then the 60 Hz run) and the boundary spike, "once its models exist" (SPIKE-ORDER (b), Mustafa,
   2026-11-03); the asset research's export and slot-naming part runs before it (LOOK-TEST-INPUTS, part 11).
   *Milestone:* the technique table and both ink families measured on both machines; Mustafa's verdict on every
   LOOK_V2 item and on OQ-0003-2, -3, -5, -6, -8, -12, -15, -16, -18 and -19, recorded in this ADR.
END

### FIX M21
FILE: docs/adr/0003-rendering.md
BEFORE:
  2728–2731). Research ids: R3-BUDGET-OQ1 (lines 1364–1365), R1-TICK-OQ6 (r1-simulation-core.md lines 2662–2665),
  R1-BND-OQ3.
AFTER:
  2728–2731). Research ids: R3-BUDGET-OQ1 (lines 1364–1365), R1-TICK-OQ6 (r1-simulation-core.md lines 2662–2665),
  R1-BND-OQ3.
  Answered in part by Mustafa (2026-11-03): "Test PC: my own, in Brave" (SPIKE-MACHINES). That PC is MP, which the
  pass marks' key defines as what he judges and never a budget result, so it stands in for none of ML, RD or MAC
  (the advisor's reading, 2026-11-03); still open: which laptop (ML) and which desktop (RD), and which Mac (MAC) for
  the rows that name Safari, decided by Mustafa (the advisor's reading, 2026-11-03, escalates it to him: about 22 of
  the netcode proof test's 62 marks are measured on ML or RD).
END

### FIX M21-2
FILE: docs/adr/0003-rendering.md
BEFORE:
1. **Drafts approved** by Mustafa and the Advisor; the two reference machines named (OQ-0003-1).
AFTER:
1. **Drafts signed off as the basis for the spikes** by Mustafa (2026-11-03, SPIKE-ORDER (b); this record stays
   Proposed, not yet accepted); the two reference machines named (OQ-0003-1, answered in part: still open, Mustafa's).
END

### FIX L11
FILE: docs/adr/0003-rendering.md
BEFORE:
    (CL10, `LEGACY_INVENTORY.md` line 119). It decides nothing about compression or LODs (ASSET-RESEARCH (a), the advisor's ruling; J4).
AFTER:
    (CL10, `LEGACY_INVENTORY.md` line 119). It decides nothing about compression or LODs (ASSET-RESEARCH (a), the advisor's ruling; J4).
    Mustafa makes them (LOOK-TEST-INPUTS, 2026-11-03): "Route: Blender (Spline's skins and named slots are
    unproven). I make the test models with AI help in Blender; blockouts with a skin, named colour slots and
    LOOK_V2's proportions are enough." The list above stays, and each model on it may be a blockout at that bar;
    which look checklist steps a blockout can judge is for the look proof test's own plan (graphics-engineer,
    test-engineer), and any visual call is Mustafa's (the advisor's reading, 2026-11-03). The same answer says "the
    asset research's export and slot-naming part runs before the look proof test": that part is ADR 0005's
    OQ-0005-AS3 and AS4, and it works on Blender's export, so that its export and slot rules reach Mustafa before he
    makes the models (the advisor's reading, 2026-11-03).
END

### FIX L11-2
FILE: docs/adr/0003-rendering.md
BEFORE:
- **Needs:** LOOK_V2; both reference machines (OQ-0003-1); Mustafa's time in Brave; a few authored test assets as
  plain glTF/GLB exports (part 11), with ADR 0005's ported kit builders beside them and a minimal content loader that
  runs the content crate's whole-set check (ADR 0005); legacy screenshots at the same framing.
AFTER:
- **Needs:** LOOK_V2; both reference machines (OQ-0003-1); Mustafa's time in Brave; a few authored test assets as
  plain glTF/GLB exports, the models Mustafa makes in Blender, blockouts at his bar allowed (part 11;
  LOOK-TEST-INPUTS), after the asset research's export and slot-naming part, with ADR 0005's ported kit builders
  beside them and a minimal content loader that runs the content crate's whole-set check (ADR 0005); legacy
  screenshots at the same framing.
END

### NOTE L13
no change: ADR 0003:525 is the trace row of LOOK_V2 §8's `[DEFAULT]` flow ("sketch → Spline model → glTF/GLB → …"),
and by the advisor's ruling on LOOK-TEST-INPUTS LOOK_V2 is not edited; the row's own column for the test says only
"plain glTF/GLB exports, with no compression or LOD step", which Blender's exports are. Blender as the route for the
look proof test's models is recorded in part 11 (FIX L11) and in ADR 0005's GPU section (FIX L8, L10); whether §8
itself changes is the report's escalation to Mustafa (LOOK_V2-§8), not this record's.
END

### NOTE T3
no change at my places (ADR 0003:784, the GA14 row of "Legacy inventory rows", and :892-893, OQ-0003-15's list, "the
toy-soldier death fall"): both name a legacy animation left to Mustafa's call in the look proof test (OQ-0003-15), not
TOY-BOX's subject (CLAUDE.md line 4 and the art-director's file, reworded in a later kit change-set); by the advisor's
ruling TOY-BOX changes nothing in the nine documents. :1302 is test-engineer's (its NOTE T3).
END

### FIX F1
FILE: docs/adr/0003-rendering.md
BEFORE:
  then the render budget bench's counts. Research ids: R3-CHAR-REC4 (lines 1088–1092; it stays adopted, no new
  disposition).
AFTER:
  then the render budget bench's counts. Research ids: R3-CHAR-REC4 (lines 1088–1092; it stays adopted, no new
  disposition).
  Answered by the Advisor, confirmed by Mustafa (2026-11-03): the Advisor, "at most 6 of 600 presented frames may
  miss a refresh with the very large units on screen (recommended desktop, High); the p95 rows stay as
  diagnostics"; Mustafa, "I confirm this reading". That is option (a), with the share at 6 of 600 (1 %), and (b) is
  not chosen. The p95 frame rows are diagnostics for this reading and keep gating their own needs, as PM-0003-18 to
  -21 do, and the minimum laptop's count stays reported (the advisor's reading, 2026-11-03).
END

### FIX F2
FILE: docs/adr/0003-rendering.md
BEFORE:
     frames (5 %) run over, so §6.9's "without dropping frames" also gets a presented-frame count (proposed): on the
     recommended machine at High, Chrome on WebGPU, with the four units and their visual effects on screen and normal
     vsync on a 60 Hz display, the frames whose interval passes 1.5 refresh periods are counted (the loop follows the
     display, R3-BUDGET-F2, lines 545–549, so such a frame missed at least one refresh); the same count is reported
     on the minimum laptop at Low against a 33.3 ms period. How many such frames a run may hold (0 of 600, or a small
     share) is OQ-0003-17; the pass-mark row is test-engineer's.
AFTER:
     frames (5 %) run over, so §6.9's "without dropping frames" also gets a presented-frame count (OQ-0003-17's
     option (a), decided): on the recommended machine at High, Chrome on WebGPU, with the four units and their visual
     effects on screen and normal vsync on a 60 Hz display, the frames whose interval passes 1.5 refresh periods are
     counted (the loop follows the display, R3-BUDGET-F2, lines 545–549, so such a frame missed at least one
     refresh); the same count is reported on the minimum laptop at Low against a 33.3 ms period. A run may hold at
     most 6 of 600 such frames (OQ-0003-17, answered by the Advisor, confirmed by Mustafa, 2026-11-03); the p95
     frame rows stay as diagnostics for this reading and keep gating their own needs (the advisor's reading,
     2026-11-03). The pass-mark row is test-engineer's (PM-0003-44).
END

### FIX Q1
FILE: docs/adr/0003-rendering.md
BEFORE:
  must pass. (a) and (c) go beyond R3's 3 runs: proposed, for the reason above. Part 8 and the pass marks hold to
  (a) until it is answered. Recommendation: (a), since VISION_V2 §4's "30+ fps on Low" holds over a whole match.
  Decider: the Advisor. What decides it: the Advisor's reading of VISION_V2 §4's "30+ fps on Low" over a whole
  match; the bench's cold and soaked sets, once measured, show how far apart they lie. Research ids: R3-BUDGET-REC2
  (lines 1162–1164, its 3 runs; it stays adopted, no new disposition).
AFTER:
  must pass. (a) and (c) go beyond R3's 3 runs: proposed, for the reason above. Part 8 and the pass marks held to
  (a) until it was answered. Recommendation: (a), since VISION_V2 §4's "30+ fps on Low" holds over a whole match.
  Decider: the Advisor. What decides it: the Advisor's reading of VISION_V2 §4's "30+ fps on Low" over a whole
  match; the bench's cold and soaked sets, once measured, show how far apart they lie. Research ids: R3-BUDGET-REC2
  (lines 1162–1164, its 3 runs; it stays adopted, no new disposition).
  Answered by the Advisor (2026-11-03): "judge after 15 minutes of play." That is option (a): the soaked set S gates
  PM-0003-18 to PM-0003-21 on ML, and the cold set is reported beside it (the advisor's reading, 2026-11-03).
END

### FIX Q2
FILE: docs/adr/0003-rendering.md
BEFORE:
     (R3-MEASURE-REC1, Recommendations Q6, lines 1181–1193). On the minimum laptop the same run set is also taken
     after at least 15 minutes of play (the pass marks' soaked set S); which of the two sets gates is OQ-0003-20.
AFTER:
     (R3-MEASURE-REC1, Recommendations Q6, lines 1181–1193). On the minimum laptop the same run set is also taken
     after at least 15 minutes of play (the pass marks' soaked set S); the soaked set gates and the cold set is
     reported beside it (OQ-0003-20, answered by the Advisor, 2026-11-03: "judge after 15 minutes of play").
END

### FIX L10
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
unit, the **asset-pipeline research** (graphics-engineer, verified like R3, in its own daytime session), which
`docs/SPIKE_PLAN.md` places before the asset spike (ASSET-RESEARCH (a), the advisor's ruling of 2026-10-31).
AFTER:
unit, the **asset-pipeline research** (graphics-engineer, verified like R3, in its own daytime session), which
`docs/SPIKE_PLAN.md` places before the asset spike (ASSET-RESEARCH (a), the advisor's ruling of 2026-10-31), with
its export and slot-naming part (OQ-0005-AS3, AS4) earlier, before ADR 0003's look proof test (LOOK-TEST-INPUTS,
Mustafa, 2026-11-03). The look proof test's models are made in Blender: "Route: Blender (Spline's skins and named
slots are unproven)" (Mustafa, the same answer). Blender is also that research part's export route, and LOOK_V2
§8's flow stays as written (the advisor's reading, 2026-11-03).
END

### FIX L10-2
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
All six are **not researched**. AS1 to AS5 form one research unit, the asset-pipeline research (graphics-engineer,
verified like R3, its own daytime session), which `docs/SPIKE_PLAN.md` places before the asset spike; the asset spike
then proves its answers.
AFTER:
All six are **not researched**. AS1 to AS5 form one research unit, the asset-pipeline research (graphics-engineer,
verified like R3, its own daytime session), which `docs/SPIKE_PLAN.md` places before the asset spike, its export and
slot-naming part (AS3, AS4) before ADR 0003's look proof test (LOOK-TEST-INPUTS, Mustafa, 2026-11-03); the asset
spike then proves its answers.
END

