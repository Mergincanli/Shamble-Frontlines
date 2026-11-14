Task: digest-0002 (build plan for foundation 2.4, 2.5, Slice 1: digest of ADR 0002 pass marks + PM-0006-28..31 into marks-0002.md)

## Plan (blocks, in order)
B1. Read SPIKE_PLAN 288-331 and 440-472.
B2. Read ADR 0002 lines 1126-1300 (What a spike must prove, items, start of pass marks).
B3. Read ADR 0002 lines 1300-1430 (rest of pass marks).
B4. Read ADR 0002 migration plan 1431-1506.
B5. Read ADR 0006 PM-0006-28..31 (from 1268).
B6. Write marks-0002.md table rows PM-0002-1..39.
B7. Write rows PM-0002-40..78 + PM-0006-28..31.
B8. Write sections Items, Open questions, 2.4/2.5/Slice 1, Measure early, count line.
B9. DONE.

## Notes
- B1 SPIKE_PLAN: boundary spike marks = PM-0002-1..34, -77, -78 (-21,-27,-33 conditional; -78 reported; -28 gates once sim core sim runs in Worker); PM-0006-28..31 optional only if EU Sentry org exists (OQ-0006-8), else move to first online playtest milestone. Foundation = PM-0002-35..69 (69 = foundation done); slices 1-6 = PM-0002-70..76. SPIKE_PLAN 323-331 decides: R1-BND-OQ2, OQ3 (core count recorded), OQ6, OQ9 (only if isolation adopted); R1-XU-OQ3, OQ4; R1-TICK-OQ2; R1-DATA-OQ8; R5-LAYOUT-OQ4; R5-TSLINT-OQ3,OQ4,OQ6; R5-TEST-OQ1; R6-OBS-OQ3,OQ4 (optional); critic ranks 4, 37; sim-wasm size for R1-RR-OQ5 (limit set later by Advisor after content spike); copy time R1-RR-OQ6 (PM-0002-10); results for OQ-0002-1, -3, -4, OQ-0004-7; feeds R5-CI-OQ4. Also PM-0003-20 measured on this spike's Worker in render bench. Foundation decides R5-GATE-OQ5, R5-CI-OQ1.
- B1 done. Next B2.
- B2 read 1126-1299 (items 1-10 at 1133-1189; Brave judging 1191; table conventions 1205-1259: ML/MAC/MP/CI; BX = proof test sim + scripted movers, N in 3 teams, 15 big vehicles (4 very large of 20 parts), 300 projectiles; BX-P = Worker step padded to 13.33 ms; 10-min match = BX N=48 local, 18,000 ticks; online P0/P2 (P2 = 100 ms RTT, +-20 ms jitter, 2 % loss); H = 3x600 frames after 300 warm-up; U uncapped/P presented; isolation-flag build for sub-ms timers; counting build; rule B). Rows 1-34, 77, 78 at 1264-1299.
- B4 read migration plan 1431-1506: 2.4 done when "the marks of the spike's item 4 hold in CI" (1471-1472) = PM-0002-15..19. 2.5 done when loopback Join/JoinAccepted/Input/Snapshot with L1 in CI; newest input posted each frame, every tick on newest post, one block per tick, empty scene naming backend; page zero Wasm calls; first recording's per-tick hashes match natively twice and in Wasm (1473-1478). Slice 1 done when ADR 0001/0004 slice-1 milestones pass + "boundary marks of the spike's items 1-3 hold on the real slice at N = 12" (1487-1491) = PM-0002-1..14, 77. Slice 2: G13 + nightly 30-min bot room N=48 over L1 (1492-1494). Slice 3: G12 both backends, multiplayer preview vs staging; Sentry upload job if Sentry set up (1495-1498). Slice 4: boundary round-trip test covers every new message/section (1499-1501). Slice 5: G8 no editor chunk; editor checks = G13 (1502-1504). Slice 6: R5-TEST-REC5 marks every mode with server tick budget (1505-1506).
- FINDING: PM-0002-11 (B2 vs B3) and PM-0002-10 (per-entity vs per-field layout) and PM-0002-25 (one vs two instances) each need a second implementation the real game does not build: variant builds or separate rig.
- FINDING: migration plan for Slice 1 names items 1-3 only, so PM-0002-31 (content to Worker, content enters in Slice 1) and PM-0002-28 (sim share) are not in Slice 1's done-when though Slice 1 is the first build able to show them.
- B2+B4 done. Next: write rows 1-34,77,78 into marks file (B6), then B3.
- B6 done: marks-0002.md written with header + rows PM-0002-1..34, 77, 78 (table order). Next B3 (read 0002:1300-1430) then append rows 35-76.
- B3 part 1: read 0002:1300-1339 (rows 35-72). Appended rows 35-72 to marks-0002.md. FINDING: PM-0002-70 lists -2,-3,-4,-6,-7,-8,-9,-12,-13 at N=12 (not -5, -14, -77 though migration plan says "items 1-3"). FINDING: PM-0002-72 server tick p99 only on RS one vCPU, never CI. Next: read 0002:1340-1430 (rows 73-76, checklists), append rows 73-76.
- B3 done: read 0002:1340-1429 (rows 73-76; notes; when a mark fails 1354-1369: conditional = -21,-27,-33; reported = -10,-29,-30,-31,-32,-78, -25's report; failed mark -> engine-architect revises decision part via Rollback routes (B3, C2 behind isolation flag, W3, OQ-0002-3's other layout, dependency moved to server); -26/-28 numbers change only in ADR 0004; spike checklist 1395-1416; foundation checklist 1418-1428). Updated row 34 with checklist steps; appended rows 73-76. FINDING: "What has no measurable mark" 1378-1379: interpolation buffer in the Worker (critic rank 4) has no mark of its own; ADR 0001's interpolation marks run through this Worker. FINDING: 1389-1392 60 Hz decided at slice 6 on real sim. Next B5 (ADR 0006 PM-0006-28..31).
- B5 done: PM-0006-28..31 at 0006:1395-1399 (group header 1395: only if EU Sentry org created; in ADR 0002's boundary spike, otherwise at the first online playtest's milestone). CF = Cloudflare Pages test project (0006:1278). Rows appended. B7 done (all table rows written). Next B8 sections.
- FINDING: PM-0002-69 (0002:1335) requires "PM-0002-35 to PM-0002-68 pass", which includes PM-0002-62, but PM-0002-62 (0002:1328) "lands in its kit change-set, not in the foundation". Contradiction.
- FINDING: SPIKE_PLAN 302-303 names only -78 as reported; ADR 0002 1355-1357 names -10, -29, -30, -31, -32, -78 and -25's report. SPIKE_PLAN narrower (not a number conflict).
- FINDING: PM-0002-4 needs a "no sim" page mode replaying 1,200 recorded blocks (a real-game test mode to add).
- B8 started: writing sections (Items, OQs, 2.4/2.5/Slice 1, Measure early, count).
- B8 done: sections Items, OQs, 2.4/2.5/Slice 1, Measure early (-25, -26, -3, -8, -4, -13 with -12, -77, -28), Count (82 expected, 82 rows, none missing) appended to marks-0002.md.
DONE
