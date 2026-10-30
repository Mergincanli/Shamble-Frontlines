# Task: step 3c owner apply (R1 and R4): resolutions, glossary terms, shared cross-topic OQs (prompt: .checkpoints/v2-research/prompts/engine-architect-r14-consistency.txt)
Inputs read: prompt, advisor-resolutions-verdict.md, plan-amendments-2.md, resolutions.md (all), cross-topic-oqs.md, GLOSSARY.md, glossary-hits-split.tsv (R1 158 rows, R4 73 rows), glossary_check.py. No memory found. Originals copied to scratchpad r14/r1.orig.md, r4.orig.md (= commit 7c6e0d7 state).
Coordinator correction (mid-task): two subsections, "### Shared open questions (the same words in every topic)" (codes, at resolutions.md's place) then "### Cross-topic open questions (consistency critic, 2026-10-29)" (XT-OQs); not the prompt's name. -- nit
## Step 1: resolutions (all 50 before quotes matched exactly once; applied by script r14/apply.py, after texts verbatim; "(GLOSSARY" marks turned into links in step 2)
R1 C1.1 R1-ECS-REC1 mark 1 -- recommendation
R1 C1.2 R1-TICK-REC2 P1 -- nit
R1 C1.3 R1-TICK-REC2 P1 -- nit
R1 C1.4 R1-DET-REC3 -- nit
R1 C1.5 R1-ABL-REC6 (also the p99 rule) -- recommendation
R1 C1.6 R1-ECS-OQ4 -- nit
R4 C1.15 R4-PHYS-REC1 mark 2 -- recommendation
R4 C1.16 R4-NAV-REC2 -- recommendation
R4 C1.17 R4-LOS-REC1 mark 4 -- recommendation
R4 C1.18 R4-LOS-REC2 -- recommendation
R1 C5.1 C5.2 C5.3 C5.4 R1-ABL-REC4 -- recommendation (4 lines: C5.1, C5.2, C5.3, C5.4)
R1 C5.5 R1-ABL-OQ4 -- nit
R1 C5.6 R1-DET-OQ6 -- nit
R1 C6.1 R1-DET-REC4 -- recommendation
R1 C6.2 R1-ABL-REC3 -- recommendation
R1 C6.3 R1-DET-OQ9 -- nit
R1 C9.1 R1-ECS-REC1 mark 1 -- recommendation
R1 C9.2 R1-ECS-REC1 mark 2 -- nit
R1 C9.3 R1-TICK-REC2 P1 -- recommendation
R1 C9.4 R1-TICK-REC2 P1 -- nit
R1 C9.5 R1-TICK-REC3 -- nit
R1 C9.6 R1-DET-REC3 -- nit
R1 C9.7 R1-ABL-REC6 -- recommendation
R1 C9.8 R1-XU-OQ1 (settled) -- nit
R4 C9.17 R4-PHYS-REC1 mark 2 -- nit
R4 C9.18 R4-PHYS-REC4 -- recommendation
R4 C9.19 R4-NAV-REC2 -- recommendation
R4 C9.20 R4-OBST-REC1 -- recommendation
R4 C9.21 wave-1 list -- nit
R4 C9.22 wave-1 list -- nit
R1 C10.1 R1-DET-REC1 -- nit
R1 C10.2 R1-TICK-OQ7 -- nit
R4 C10.14 R4-LOS-REC1 mark 4 -- nit
R4 C10.15 R4-LOS-REC2 -- nit
R1 C11.1 C11.2 C11.3 C11.4 C11.5 C11.6 C11.7 (two pairs) -- nit (7 lines)
R4 C11.13 r4:658 -- nit
R1 C17.1 R1-RR-OQ7 -- nit (C17.b1 not applied: waits on BASELINE-WHEN)
R1 C29.1 R1-DET-REC5, C29.2 R1-ECS-REC2, C29.3 R1-ECS-REC2 mark 2 -- recommendation (3 lines)
## Step 2: glossary (R1), done so far
R1 every "(GLOSSARY" mark in the applied after texts turned into a link GLOSSARY.md#<entry>; touched lines reflowed to <=120 chars (no word changed) -- nit
R1 R1-ECS-REC1: links one vCPU, reference server, stress scene, largest mode's N, live projectiles, sim step time, sim step share, tick period -- nit
R1 R1-ECS-REC1 mark 1 label "Tick cost" -> "Sim step time" (Sim step time, in context "tick cost") -- nit
R1 R1-ECS-REC1 mark 2 "≤ 0.2 ms native and ≤ 0.5 ms Wasm" + "at p99" (Time statistics rule) -- recommendation
R1 R1-ECS-REC1 mark 3 "fits in ≤ 8 ms" -> "fits in ≤ 8 ms at p99" (Time statistics rule) -- recommendation
R1 R1-ECS-REC1 mark 3 "R2's partial prediction" -> "R2's prediction scope (GLOSSARY link)" (prediction scope, in context) -- nit
R1 R1-TICK-REC2 P1 "p99 tick time" -> "p99 sim step time" (combined view of resolutions.md) -- nit
R1 R1-TICK-REC2 P2 "p99 tick time at most 40 % of the period (6.7 ms at 60 Hz, 13.3 ms at 30 Hz)" -> "p99 sim step time within the client's sim share (GLOSSARY: at most 40 % of the tick period, 6.7 ms at 60 Hz, 13.3 ms at 30 Hz)" (same values) -- nit
R1 R1-TICK-REC3 "p99 tick time ... mean tick time" -> "p99 sim step time ... mean sim step time" -- nit
R1 R1-BND-REC5 heading: G3 gains link "(G3; GLOSSARY: the isolation flag)" (resolution 22: "the glossary link goes on its first G3") -- nit
R1 R1-BND-REC5 spike: "preview link" linked (first use of Preview) -- nit
R1 R1-XU-OQ1 heading and closing "the tick budget" -> "the server tick budget" (Retire exact; 2 hits); body "mean tick" -> "mean sim step time", "the period" -> "the tick period"; old values kept (record of the units' proposals) -- nit
R1 R1-RR-OQ7 C17.1 text linked (advisories check, security baseline) -- nit
R1 boundary options table (A2 row) "Tick time leaves the render thread" -> "Sim step time leaves ..." -- nit
R1 pointers to other topics, Security line: "R6's security baseline" -> "the security baseline (R6-SEC-OQ1)" (Security baseline, in context) -- nit
R1 first-use links: N (legacy row TL test, r1:853 orig), 48+ (unit scope, r1:322 orig) -- nit
R1 in-context hits left as they are (other meaning, legacy fact, quoted, a finding about another game, or a record of the units' first proposals): effects (ability model, presentation, legacy FX; none a stress count), vehicles (general, not a stress count), 48 (legacy measurements, goal 24v24, LOOP_V2 scale steps N = 12..48, R1-BND-REC2 mark 4 "48 soldiers", R1-BND-OQ6 "N = 48", source-table row numbers), characters (text characters), players (not team sizes), always / its hash (content bundles, not isolation or RNG), the tick interval (R1-TICK-F13: Source's own tick interval), R1-XU-OQ1 body (old figures as proposed), cargo-audit and cargo-deny as CI gates (R1-RR-OQ7: the reviewer's proposal, stays per GLOSSARY), login wall (r1 sources-not-read row 47: a search result about gdcvault, not our access wall). Time marks not given a statistic: one-off durations (R1-ECS-REC1 mark 5 rebuild ≤ 10 s, R1-TICK-REC4 recovery within 1 s and catch-up bound 0.25 s, R1-TICK-REC5 back in step within 0.5 s, R1-DATA hot reload within 1 s): event deadlines, not costs measured over a run; for the step-3d verifier to judge.
R1 first-use links added (first hit that carries the term's meaning): authoritative (CS8 legacy row), mean (Time statistics, legacy measurement), slots and the goal (entity kinds, ecs legacy facts), big vehicles (tick unit scope), active effects (R1-ABL-F1 paraphrase, outside the quote), state hash (R1-ECS-REC1 mark 4), sim step time (boundary options A2) -- nit (7 lines in one)
R1 first literal hits NOT linked (other meaning): "start" r1:205 (a slice's start), "slots" r1:288 (memory slots), "predicted" r1:507 (a finding's predicted time; the prediction scope is linked at R1-ECS-REC1 mark 3)
Check script after R1 step 2: R1 exact 0.
RESUMED 2026-10-30 after the spend-limit cut (20:10 UTC): state checked, R1 steps 1-2 and R4 step 1 present in the files as logged; R4 step 2, shared OQs, XT-OQs, header, part 7 not yet done.
## Step 3 (R1)
R1 pointers "Cross-topic open questions: R1-XT-OQ<k>" added at the end of the 17 named recommendations: ECS-REC1 (OQ2, OQ7, OQ18), ECS-REC3 (OQ11), ECS-REC4 (OQ2), TICK-REC2 (OQ2), TICK-REC5 (OQ3), DET-REC1 (OQ4), DET-REC2 (OQ1), DET-REC3 (OQ2), BND-REC1 (OQ1, OQ14), BND-REC2 (OQ3, OQ9), BND-REC3 (OQ1), BND-REC4 (OQ13), BND-REC5 (OQ17), BND-REC6 (OQ3), ABL-REC6 (OQ15), DATA-REC3 (OQ13), DATA-REC6 (OQ6) -- nit (17 lines in one)
R1 "### Shared open questions (the same words in every topic)" inserted before "### Pointers between R1's units" (resolutions.md's place), with a 3-line intro -- nit
R1 C5.7 ABL-PREDICT inserted word for word (GLOSSARY mark linked) -- nit
R1 C6.4 SEC-SPREAD inserted word for word -- nit
R1 C10.3 MODES-3 inserted word for word -- nit
R1 C17.2 BASELINE-WHEN inserted word for word -- nit
R1 "### Cross-topic open questions (consistency critic, 2026-10-29)" inserted right after it, with a 3-line intro -- nit
R1 R1-XT-OQ1..OQ18 inserted in the critic's words (ranks 2, 3, 4, 7, 12, 13, 14, 15, 16, 25, 27, 28, 31, 35, 36, 37, 38, 40), each "<id>" set to the R1 id; texts taken by script from cross-topic-oqs.md -- nit (18 lines in one)
## Step 4 (R1)
R1 header: line "Glossary and cross-topic resolutions applied 2026-10-30 (see GLOSSARY.md and part 7)" added with the GLOSSARY.md link; id table gains 2 rows (shared decision questions; R1-XT-OQ1–OQ18); totals 77 -> 99 open questions (+4 shared, +18 XT), R1-XU-OQ1 listed as settled at the consistency step; labels unchanged -- nit
## Step 2 (R4)
R4 every "(GLOSSARY" mark in the applied after texts linked (preface, PHYS-REC1 mark 2, PHYS-REC4, NAV-REC2, LOS-REC1 mark 4, LOS-REC2, OBST-REC1, wave-1 list x2); touched lines reflowed, no word changed -- nit
R4 R4-PHYS-REC1 mark 5 "The same 48 soldiers" -> "The same soldiers as mark 2 (the largest mode's N, GLOSSARY)" (largest mode, in context; follows C9.17's scene; 54 under MODES-3 (b)) -- recommendation
R4 R4-OBST-REC2 spike "a flip costs ≤ 0.1 ms native" -> "... at p99 (proposed; GLOSSARY)" (Time statistics rule) -- recommendation
R4 legacy facts used (R4-LOS), wave-1 line: "≤ 10 % of the tick budget" (Retire exact) -> "≤ 10 % of the server tick budget, GLOSSARY: 5 % of the tick period" (R2-INT-REC3's value as GLOSSARY states it; a pointer, not an R4 mark) -- nit
R4 R4-NAV-REC2 "N = 64 headroom run" linked (design point and headroom); "tick period" linked -- nit
R4 first-use links: N (design inputs, LOOP_V2 §11 scale test), bot slots (needs list), big vehicles (LOS options table), state hash (PHYS-REC3 spike), time statistics (PHYS-REC1 mark 2) -- nit (5 in one)
R4 first literal hits NOT linked (other meaning): "goal" r4:161 (Rapier's 2026 goal), "N" r4:343 (a crate signature `&N`), "preview" r4:719 (the grenade-arc preview), "mean" r4:399 (a stall-rate mean in a legacy fact; Time statistics linked at PHYS-REC1 mark 2). Start/goal modes and Preview are not used in R4 in the glossary's sense.
R4 in-context hits left: vehicles (general / tables / findings), characters (sources' "characters read", Rapier's character controller), 48 (legacy facts GN4/GN13, R4-PHYS-REC4 "a crowd of 48 soldiers", R4-NAV-REC3 choke test "48 bots"; not the largest mode), always (findings, legacy CS2), 25 % (R4-FLOORS-F7 legacy route share), stress load (R4-PHYS-REC4 Rests-on line, kept per GLOSSARY), R1-TICK-REC2's P1 scene (wave-1 list C9.22 after text, names R1's own scene; kept per GLOSSARY). One-off time marks left without a statistic: R4-PHYS-REC2 (d) BVH build ≤ 50 ms at match start, R4-NAV-REC1 mark 1 bake ≤ 1 s / ≤ 3 s; for the step-3d verifier.
## Step 3 (R4)
R4 "### Shared open questions (the same words in every topic)" inserted before "### Wave-1 items R4 changes or answers" (resolutions.md's place), 3-line intro -- nit
R4 C10.16 MODES-3 inserted word for word -- nit
R4 "### Cross-topic open questions (consistency critic, 2026-10-29)" right after it, 3-line intro -- nit
R4 R4-XT-OQ1..OQ8 inserted in the critic's words (ranks 2, 3, 14, 15, 24, 30, 35, 40) -- nit (8 lines in one)
R4 pointers "Cross-topic open questions: ..." at the end of PHYS-REC1 (OQ2, OQ8), PHYS-REC2 (OQ5), NAV-REC1 (OQ5), NAV-REC3 (OQ6), NAV-REC4 (OQ3, OQ4), FLOORS-REC3 (OQ5), LOS-REC1 (OQ7), LOS-REC3 (OQ1) -- nit (8 in one)
R1 R1-XU-OQ2: "R1-ABL-REC4 predicts only the local player's own abilities and weapons" (prediction scope, in context: a settled scope) -> "predicts the local player's own weapons, and its own abilities if ABL-PREDICT says so", matching C5.1 -- nit
## Step 4 (R4)
R4 header: glossary line with GLOSSARY.md link; id table +2 rows; totals 23 -> 32 open questions (+MODES-3, +8 XT); label unchanged -- nit
R1 part 7: "### Consistency resolutions (2026-10-30)" added at the end of "## Verifier spot-check" (resolutions by rank and change id, recommendation changes, glossary summary, questions added, OBJECT: none) -- nit
R4 part 7: the same subsection added at the end of "## Verifier spot-check" -- nit
