# rust-reviewer stop-answer fixes (ADR 0004), 2026-11-03

### FIX O15
FILE: docs/adr/0004-simulation-core.md
BEFORE:
**Author:** rust-reviewer (the "Pass marks" table: test-engineer, plan step 4) · **Reviewed by:** (filled in at review) · **Signed off by Mustafa:** no
AFTER:
**Author:** rust-reviewer (the "Pass marks" table: test-engineer, plan step 4) ·
**Reviewed by:** (filled in at review) ·
**Signed off by Mustafa:** as the basis for the spikes (2026-11-03, SPIKE-ORDER (b)); not yet accepted
END

### FIX S10, H23
FILE: docs/adr/0004-simulation-core.md
BEFORE:
- TICK-30 (STATUS "Decisions pending"): 30 Hz is the baseline; 60 Hz only with
  [the 60 Hz margin](../research/GLOSSARY.md#the-60-hz-margin) of at least 25 %, which ADR 0001 owns and measures in
  the netcode proof test.
AFTER:
- TICK-30 (STATUS "Decisions pending"): 30 Hz is the baseline; 60 Hz only with
  [the 60 Hz margin](../research/GLOSSARY.md#the-60-hz-margin) of at least 25 %, which ADR 0001 owns and measures
  twice: early, in wave 1's 60 Hz run after the sim core spike (SPIKE-ORDER (b)), and finally at slice 6 with the real
  sim (TICK-60-SETUP, the Advisor, 2026-11-03: "(a), as an early signal only. The final 60 Hz call is measured again
  with the real sim at slice 6."). The full rule is ADR 0001's part 2, with Mustafa's addition (TICK-60-HEADROOM,
  2026-11-03; PM-0001-63): "60 Hz also needs the goal budget to hold at the rooms per VM measured at 60 Hz".
END

### FIX RC6
FILE: docs/adr/0004-simulation-core.md
BEFORE:
    room's). Whether a room may instead resume from a snapshot is the Advisor's question in ADR 0001. In the browser
    `-Cpanic=abort` ends the sim's instance, and the trap alone carries no panic message, only `unreachable` and
    frames; so the Wasm build installs a small panic hook of its own (`std::panic::set_hook`, no extra crate) that
    writes the message and location into a fixed buffer in linear memory before the abort, and the Worker's host reads
    it after the trap and attaches it to the report (proposed: a native scratch build on rustc 1.97.0 ran such a hook
    under `panic = "abort"` and kept the message; the Wasm build is S20's to check). Online, after a trap, the Worker's
    host starts a fresh instance and rejoins through ADR 0001's resync path, and the trap is reported (R6-OBS-REC3, ADR
    0006); the panic strategy is ADR 0002's. In local play the trapped instance was the authority; recovery there is
    not researched (OQ-0004-7).
AFTER:
    room's). Whether a room may instead resume from a snapshot was the Advisor's question in ADR 0001 (OQ-0001-18); the
    Advisor answered it on 2026-11-03: "end the match, as proposed, and keep the crash and match records", so a room
    never resumes. In the browser `-Cpanic=abort` ends the sim's instance, and the trap alone carries no panic message,
    only `unreachable` and frames; so the Wasm build installs a small panic hook of its own (`std::panic::set_hook`, no
    extra crate) that writes the message and location into a fixed buffer in linear memory before the abort, and the
    Worker's host reads it after the trap and attaches it to the report (proposed: a native scratch build on rustc
    1.97.0 ran such a hook under `panic = "abort"` and kept the message; the Wasm build is S20's to check). Online,
    after a trap, the Worker's host starts a fresh instance and rejoins through ADR 0001's resync path, and the trap is
    reported (R6-OBS-REC3, ADR 0006); the panic strategy is ADR 0002's. In local play the trapped instance was the
    authority; recovery there is not researched (OQ-0004-7).
END

### FIX H23-2
FILE: docs/adr/0004-simulation-core.md
BEFORE:
  yes (an ability-proc stream may become shared); the 60 Hz margin unlocks 60 Hz (re-convert durations, re-bake links,
  re-run the parity marks).
AFTER:
  yes (an ability-proc stream may become shared); the tick moves to 60 Hz at slice 6's call (ADR 0001 part 2: the 60 Hz
  margin, and the goal budget holding at the rooms per VM measured at 60 Hz, PM-0001-63; re-convert durations, re-bake
  links, re-run the parity marks).
END

### FIX S11
FILE: docs/adr/0004-simulation-core.md
BEFORE:
where the 60 Hz margin run needs it (ADR 0001 owns that run and how the sim's load reaches it: TICK-60-SETUP). Three
claims belong to other spikes, named in the table. Every mark below is the research's (cited; all proposed there),
AFTER:
where wave 1's 60 Hz margin run needs it (ADR 0001 owns that run and how the sim's load reaches it: OQ-0001-2 (a), the
sim core spike's sim with S1's stand-ins). That run is "an early signal only" (TICK-60-SETUP, the Advisor, 2026-11-03);
the final 60 Hz call is measured again with the real sim at slice 6 (migration phase 8). Three claims belong to other
spikes, named in the table. Every mark below is the research's (cited; all proposed there),
END

### FIX S12
FILE: docs/adr/0004-simulation-core.md
BEFORE:
| S1 | The sim fits the sim step share at N = 48 | 6 | p99 sim step time ≤ 8.33 ms at 30 Hz (≤ 4.17 ms for the 60 Hz margin run); mean, p50, p95, p99 and max reported per N at N = 12, 24, 36, 48 and the N = 64 [headroom](../research/GLOSSARY.md#the-design-point-and-headroom) run; 36,000 ticks per run (20 minutes of game time at 30 Hz, 10 at 60 Hz: arithmetic), 3 runs; each part of the share reported on its own against its ceiling (R1-TICK-REC2 P1, r1-simulation-core.md lines 2044–2053; R1-ECS-REC1 mark 1, lines 1923–1931; GLOSSARY.md lines 53–69); the sim core spike builds no ability and effect step (S15 follows it) and no economy, objectives, moments or events (slice 4), so its run stands in for each: 384 rows in the effect pool (so the snapshot, the hash and memory carry them) with the effect step's 1 ms ceiling burned per tick, and the 2.80 ms remainder topped up per tick after the built projectiles and bot decisions, each stand-in named in the run's record and scaled by N / 48 at the other scale steps (proposed: a pass with the stand-ins still holds once the real parts meet their ceilings; a run that fails only through them is reported so, and the share's verdict then waits for S15's step); the built parts are judged against their own ceilings either way; S1 on the real crate is migration phase 8's; the 60 Hz margin run's load is ADR 0001's (OQ-0001-2, TICK-60-SETUP), and at 60 Hz the same two stand-ins would be 1 ms and 0.30 ms (4.17 − 3.87, arithmetic) | the sim core spike |
AFTER:
| S1 | The sim fits the sim step share at N = 48 | 6 | p99 sim step time ≤ 8.33 ms at 30 Hz (≤ 4.17 ms for the 60 Hz margin run); mean, p50, p95, p99 and max reported per N at N = 12, 24, 36, 48 and the N = 64 [headroom](../research/GLOSSARY.md#the-design-point-and-headroom) run; 36,000 ticks per run (20 minutes of game time at 30 Hz, 10 at 60 Hz: arithmetic), 3 runs; each part of the share reported on its own against its ceiling (R1-TICK-REC2 P1, r1-simulation-core.md lines 2044–2053; R1-ECS-REC1 mark 1, lines 1923–1931; GLOSSARY.md lines 53–69); the sim core spike builds no ability and effect step (S15 follows it) and no economy, objectives, moments or events (slice 4), so its run stands in for each: 384 rows in the effect pool (so the snapshot, the hash and memory carry them) with the effect step's 1 ms ceiling burned per tick, and the 2.80 ms remainder topped up per tick after the built projectiles and bot decisions, each stand-in named in the run's record and scaled by N / 48 at the other scale steps (proposed: a pass with the stand-ins still holds once the real parts meet their ceilings; a run that fails only through them is reported so, and the share's verdict then waits for S15's step); the built parts are judged against their own ceilings either way; S1 on the real crate is migration phase 8's; the 60 Hz margin run's load is ADR 0001's: in wave 1's run, an early signal only, it is this sim with these stand-ins (OQ-0001-2 (a), TICK-60-SETUP, the Advisor, 2026-11-03), so at 60 Hz the same two stand-ins are 1 ms and 0.30 ms (4.17 − 3.87, arithmetic); at slice 6 the run is on the real crate (migration phase 8) | the sim core spike |
END

### FIX S14, O20
FILE: docs/adr/0004-simulation-core.md
BEFORE:
| 1 | Spikes, throwaway in `spikes/` (SPIKE-SCOPE (a)): the netcode proof test first; the sim core spike in parallel with it, its sim or its measured step times feeding the 60 Hz margin run as ADR 0001 sets (TICK-60-SETUP); the boundary spike; then the content and ability spike | S1–S14, S19 and S20 recorded in this record; S17 from the netcode proof test; S18 from the boundary spike; S15 and S16 from the content and ability spike; option C or B, the navigation generator, the units per tick and the hash function chosen from the numbers |
AFTER:
| 1 | Spikes, throwaway in `spikes/` (SPIKE-SCOPE (a)), in the order Mustafa accepted (SPIKE-ORDER (b), 2026-11-03: "wave 1 runs one heavy spike at a time"): the netcode proof test first; then the sim core spike; then the 60 Hz margin run, with the sim core spike's sim and S1's stand-ins (OQ-0001-2 (a)), an early signal only (TICK-60-SETUP, the Advisor, 2026-11-03); then the boundary spike; then the content and ability spike | S1–S14, S19 and S20 recorded in this record; S17 from the netcode proof test; S18 from the boundary spike; S15 and S16 from the content and ability spike; option C or B, the navigation generator, the units per tick and the hash function chosen from the numbers |
END

### FIX S14-2
FILE: docs/adr/0004-simulation-core.md
BEFORE:
| 8 | Slice 6, scale to 48: the stress scene on the real sim at N = 12, 24, 36, 48 and the N = 64 headroom run; the 60 Hz margin run if ADR 0001 schedules it | S1 on the real crate: p99 sim step time ≤ 8.33 ms at N = 48, 3 runs per build |
AFTER:
| 8 | Slice 6, scale to 48: the stress scene on the real sim at N = 12, 24, 36, 48 and the N = 64 headroom run; the 60 Hz margin run on the real sim, which makes the final 60 Hz call (TICK-60-SETUP, the Advisor, 2026-11-03: "The final 60 Hz call is measured again with the real sim at slice 6."), run whatever wave 1's early signal showed (the advisor's reading, 2026-11-03; ADR 0001 owns the run and the call) | S1 on the real crate: p99 sim step time ≤ 8.33 ms at N = 48, 3 runs per build; PM-0004-71 and PM-0004-72 in the 60 Hz margin run, where they decide (the pass marks' key) |
END

### NOTE H23-3
no change: ADR 0004's other 60 Hz restatements stay. :182 ("The tick rate is ADR 0001's under TICK-30 (30 Hz; 60 Hz
only with the 60 Hz margin)") and :612 ("settled by TICK-30 (30 Hz baseline; 60 Hz only with the 60 Hz margin, ADR
0001)") state a necessary condition, which the advisor's ruling on TICK-60-HEADROOM lets stay once GLOSSARY §8 is
amended ("0004:182 and 612 ... state a necessary condition. They may stay once §8 is amended"). :807 ("unlocking 60 Hz
under the 60 Hz margin is a rebuild plus a re-bake") states the mechanism's cost, not the rule. The two places that
read as the whole rule or as a measurement change: :45-47 in FIX S10, H23 (measured early in wave 1 and finally at
slice 6; the full rule pointed to ADR 0001 part 2 with Mustafa's addition quoted) and :825-826 in FIX H23-2 ("Revisit
when": the tick moves at slice 6's call, ADR 0001 part 2).
END

### NOTE LA3
no change: the advisor's LATER ruling (2026-11-03): "The drafts' own entries for these questions are not edited
(LA3)". ADR 0004's five, OQ-0004-8 to OQ-0004-12 (:902, :915, :926, :938, :951), keep their deciders and timings as
written; Mustafa's answer ("a design session after the netcode proof test") is recorded in README (one sentence and a
marker on each row) and STATUS, not here.
END

### NOTE X5
no change: ADR 0004's need lines that trace to VISION_V2 §3 or §4 alone already say so: :179 ("VISION_V2 §4 only"),
:251 ("VISION_V2 §3 and §4 only"), :580 ("VISION_V2 §4 only"). That is "marked as such" in the Advisor's answer to
OQ-0001-14 (2026-11-03: "yes, a VISION_V2 §3 or §4 trace counts, marked as such"), and the ruling says "No need line
changes".
END

### NOTE O15-2
no change: migration phase 0 (:1234, "Mustafa and the Advisor approve the drafts | the sign-off line of this record")
stays: its milestone is the sign-off line, which FIX O15 fills with "as the basis for the spikes (2026-11-03,
SPIKE-ORDER (b)); not yet accepted"; the row names no status of its own.
END

### NOTE O20-2
no change by this round (an open finding, not from the stop's words): migration row 1 ends "then the content and
ability spike", inside phase 1, before phase 2 (the foundation), while SPIKE_PLAN's table row 7 places that spike in
wave 3, after the foundation and before slice 1. The mismatch predates the stop; Mustafa's order names only wave 1,
so FIX S14, O20 keeps the row's last item as it was. For the session: a later fix in ADR 0004 (rust-reviewer) can
write "then, after the foundation (SPIKE_PLAN's wave 3), the content and ability spike".
END

