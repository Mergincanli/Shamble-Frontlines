Task: digest-0004 (pass-mark digest of ADR 0004 for the build plan of foundation 2.4, 2.5 and Slice 1; output marks-0004.md)

## Plan (blocks, in order)
B1. Read SPIKE_PLAN 22-82 (fixtures), 244-287 (60 Hz run), 333-370 (sim core spike). Notes below.
B2. Read ADR 0004 980-1018 (what a spike must prove: claims S1..S22).
B3. Read ADR 0004 1018-1242 (pass marks PM-0004-1..75).
B4. Read ADR 0004 1243-1287 (migration plan, test plan) + open questions headings 864-979.
B5. Write marks-0004.md section 1 (table), rows 1-40.
B6. Write table rows 41-75.
B7. Write sections 2-7.
B8. DONE + answer.

## Notes
- B1 DONE. SPIKE_PLAN fixtures: stress scene N=48 = 3 teams x16 bots, 15 big vehicles (4 very large), 300 live projectiles, up to 8 active effects/soldier, DM_Tri-sized 3D greybox (SP:23-27). Machines RS=CCX23 x86-64 fsn1; LG=CAX21 aarch64; ML min laptop (stand-in: Mustafa laptop, rough checks only, never judged); RD desktop; MAC MacBook Air M1+ Safari (no Mac now); MP judges only (SP:30-78). Recorded matches + per-tick hashes: ADR 0004, sim core spike makes first ones (SP:81-82).
- 60 Hz run (SP:244-287): PM-0004-71 (sim step share at 60 Hz), PM-0004-72 (time mechanism + movement checks at 60 Hz); early signal only; runs again at slice 6, where they decide. Stand-ins: 384 effect rows w/ 1 ms ceiling burned; remainder topped to 0.30 ms at 60 Hz (4.17-3.87).
- Sim core spike (SP:333-370): claims S1-S14, S19, S20; marks PM-0004-1..61, -73, -74 (PM-0004-8 and -28 in boundary spike's Worker; -61 w/ netcode proof server). Seven targets: native, Node, three browser engines, real Safari on MAC, Windows on ML in Chrome and Firefox (PM-0004-19). Stand-ins: 384 effect rows, 1 ms burned, 2.80 ms remainder topped up (30 Hz). Decides list at SP:367-370.
- Finding F1: SPIKE_PLAN 361 says S21/PM-0004-73 and S22/PM-0004-74 subsumed by sim core spike, but SP:334 owner line lists claims S1-S14, S19, S20 only (no S21, S22) though PM list includes -73,-74. Check ADR claims.
- Next: B2.
- B2 DONE (0004:980-1016 claims table). Claim->part->spike: S1 p6 sim core; S2 p6 sim core in boundary Worker; S3 p4; S4 p4; S5 p3; S6 p2; S7 p7,9; S8 p8; S9 p10; S10 p11; S11 p12; S12 p13; S13 p14; S14 p15; S15 p16 content+ability; S16 p18 content+ability; S17 p5 netcode proof (ADR 0001); S18 p5 boundary spike (ADR 0002); S19 p21 sim core; S20 p20 sim core w/ boundary; S21 p7,10 sim core; S22 p7 sim core. Research ids per claim listed in claims column (0004:995-1016) - reread in B7 only the line for each claim if needed (allowed: later block may read a file it needs).
- Finding F1 resolved partly: ADR 0004:1015-1016 puts S21, S22 in sim core spike; SPIKE_PLAN:334 owner line omits them (minor inconsistency, SP:344 and 361-366 include their marks).
- Next: B3.
- B3a (rows 1-25, 0004:1122-1146) read. Key: How-to-read 0004:1030-1117 (RS pinned vCPU; ML Chrome+Firefox Win11, Edge reported; MAC Safari; CI counts/hashes only never a time; A64 only before ADR 0006 picks ARM; SS scenes O3, TB, LB, BR, M10=18,000 ticks; T = 3 runs x (900 warm-up + 36,000 measured); V = 3 medians within 3 %; nearest-rank pct; Wasm<=2x native read on ML same machine; gates 8.33/13.33/1.67 ms; 60 Hz set only in 60 Hz margin test; network profiles P0-P4, B1; bites rules A (64-byte alloc) and B (integer loop margin+1 ms); gate tests 20 identical verdicts).
  Lines: PM1 1122 PM2 1123 PM3 1124 PM4 1125 PM5 1126 PM6 1127 PM7 1128 PM8 1129 PM9 1130 PM10 1131 PM11 1132 PM12 1133 PM13 1134 PM14 1135 PM15 1136 PM16 1137 PM17 1138 PM18 1139 PM19 1140 PM20 1141 PM21 1142 PM22 1143 PM23 1144 PM24 1145 PM25 1146.
- B3b (rows 26-69 read, 0004:1147-1196). Lines: PM26 1147 ... PM61 1182 (sequential: PMn at 1121+n for n=1..61); PM73 1183; PM74 1184; PM62 1186 PM63 1187 PM64 1188 PM65 1189 PM66 1190 PM75 1191; PM67 1193; PM68 1195; PM69 1196. Table order is NOT numeric (73,74 after 61; 75 after 66).
- Finding F2: PM-0004-8 frame mark reads ADR 0003 ML row (median <=28 ms, p95 <=33.3 ms, mode U); needs a renderer page (ADR 0003's renderer or boundary page) -> 2.5 has empty scene only.
- Finding F3: PM-0004-60 fuzz runs on a 1v1 on LB (small scene), not SS -> early (Slice 1) possible once commands exist.
- Finding F4: PM-0004-61 needs server + Worker at P2 + ADR 0001 join mark -> Slice 3 (online).
- B3 DONE. Lines PM70 1197, PM71 1199, PM72 1200. "When a mark fails" 0004:1202-1216: conditional = PM14, 26, 71, 72, 75 (until OQ-0004-11); reported = PM4, 44, 58; decision = PM25, 34, 44; PM8 waits for R1-TICK-OQ7's N. Rollbacks named 1208-1212. Marks run in other spikes: PM8, 28, 61, 67-71. No measurable mark: 1218-1235.
- Migration (0004:1248-1258): phase 2 foundation (1252): "an empty scene runs through the full stack; its per-tick hashes match natively twice and in Wasm"; phase 3 slice 1 (1253): "S4, S5, S6 and S9 hold on the real crate; the determinism suite is green on slice 1 recordings on every target"; phase 4 slice 2 (1254) S2, S3, S10-S12, S14; phase 5 slice 3 (1255) S17, S8 last mark, S20 online recovery; phase 6 slice 4 (1256) S13, S15, S16; phase 7 slice 5 (1257) S19, S10 gap/reach; phase 8 slice 6 (1258) S1, PM71, PM72.
- Finding F5: migration phase 3 (slice 1) milestone names S4, S5, S6, S9 on the real crate, but S9 includes PM-0004-38 (big vehicles in a crowd of 48), -31 cost at N=48 in SS, -36 (bot shots on O3 ramps), -39 far-corner 2 km map; S5 includes PM12 (SS at N=48 snapshot time) and PM14 (conditional). Slice 1 has no vehicles/bots/SS. So "S9 holds" at slice 1 can only be the parts measurable then [unclear what the draft means]. Also S7 (determinism suite green on every target) at slice 1.
- Finding F6: Phase 6 slice 4 lists S13 (structures and damage stages), but task's slice order puts structures? Task says slice 4 = economy, abilities, effects; ADR 0004 phase 6 adds "structure states and damage stages (after OQ-0004-4)" -> S13 at slice 4. And S10 gap/reach at slice 5 (editor) while PM42 reachability "runs in CI" at slice 2.
- Test plan 0004:1262-1286: determinism suite on every sim change incl. real Safari; injected divergences (event, economy $1); unit vectors; property tests; parity fuzzes; fuzzing; benchmarks; soak 36,000 ticks; nav checks in CI; playtests slice 2.
- Next: B4 open questions headings.
- B4 DONE. OQ-0004-1..12 at 0004:866-968 (1 bot structure: slice 2 plan; 2 entity kinds: roster trigger; 3 ground shape: ADR 0005 map format, checked by sim core spike via PM35; 4 damage-stage trigger: Mustafa before slice 4; 5, 6 moved to ADR 0005 AB9/AB10; 7 local trap recovery: boundary spike with ADR 0002 skeleton (=2.5); 8 steal presence: uplinks change-set sims; 9 cache visibility: design; 10 rejoin record: design; 11 pool overflow: game-designer/Mustafa, spikes use (a) as fixture; 12 bots take pickups: slice 2 plan). Design OQs cited not owned 0004:970-978.
  Research OQ -> mark: R1-ECS-OQ1 PM33; R1-ECS-OQ4 PM4; R1-ECS-OQ5 moot unless option B wins (0004:1335); R1-TICK-OQ5 PM4; R1-TICK-OQ9 PM38; R1-DET-OQ1 PM20; R1-DET-OQ2 PM25; R1-DET-OQ4 answered, suite's browser runs check (0004:1344); R1-DET-OQ8, OQ10 PM22; R1-DET-OQ11 PM27; R1-RR-OQ1 PM23 (+fma), PM26 (A64), parry3d SIMD BVH (0004:379); R4-PHYS-OQ2 PM34; R4-PHYS-OQ3 S9 mark 3 = PM32 + S7 suite (0004:1384); R4-PHYS-OQ4 PM35; R4-NAV-OQ1/OQ2 PM40-44; R4-NAV-OQ3 PM45; R4-LOS-OQ5 PM52; R4-OBST-OQ4 PM55; R1-ABL-OQ7, R1-XU-OQ7 PM9, PM11; critic rank 3 = R1 items assuming Rapier, moved to parry3d (0004:376-380) -> PM20, PM33, PM34 [unsure mapping].
- Next: check ADR 0002 migration for where vehicles arrive (PM38/SS need 15 big vehicles), then B5.
- ADR 0002 migration checked (0002:1487-1506): slice 1 "ADR 0001's and ADR 0004's slice-1 milestones pass through this skeleton ... at N = 12"; slice 2 has nightly 30-minute bot room at N = 48 over L1 (so 48 bots exist locally at Slice 2); slice 6 "the tick bench in the stress scene".
- Finding F7: no slice in ADR 0002 or 0004 migration names big vehicles; SS (PM2), PM35 hull pairs, PM38, PM55, PM72 need them -> [unsure] first build; at latest Slice 6.
- Finding F8: separate-rig items: PM33 comparison builds (rapier3d, bevy_ecs 0.20 stub), PM34 Rapier controller (option B), PM40/44 second nav generator, PM26 A64 machine.
- Finding F9: SP:44 machines named "before the look proof test"; that test no longer built separately -> naming trigger gone; ML/RD/MAC-judged marks wait for naming.
- Finding F10: PM71/72 run only with 60 Hz margin test; wave-1 early signal gone with spikes; still decide at Slice 6.
- Next: B5 write table rows 1-40.
- B5 DONE: marks-0004.md header + rows PM1-PM40 written. Next: B6 rows 41-75 (append).
- B6 DONE: rows PM41-PM75 appended. Next: B7 sections 2-7.
- B7 DONE: sections 2-7 written.
- Answer summary: count 75/75, none missing. Measure early: PM19, PM23, PM22, PM9, PM30, PM20, PM13, PM7. Surprises: F5 (phase 3 milestone S5/S6/S9 includes marks slice 1 cannot show), F7 (vehicles in no slice), F8 (separate rigs PM33 comparisons, PM34, PM40/44 second generator, PM26 A64), F9 (machines' naming trigger gone), F10 (60 Hz early signal gone; PM71/72 still decide at Slice 6), F1 (SP:334 omits S21/S22), PM8 waits on R1-TICK-OQ7, PM59/PM42 can run before the slices the migration names.
DONE
