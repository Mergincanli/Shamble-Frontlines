# r6b rust-reviewer fixes (ADR 0004 outside Pass marks)

### FIX R6B-RR-1
FILE: docs/adr/0004-simulation-core.md
BEFORE:
   test before ADR 0006 may pick ARM hosts (the build flags are ADR 0002's). parry3d's own `enhanced-determinism` flag
AFTER:
   test before ADR 0006 may pick ARM hosts (the build flags are ADR 0002's). The netcode proof test's ARM load generator
   (ADR 0001's LG) is not an ARM host in this sense, since it serves no player and runs no room (the advisor's reading,
   2026-11-03), and under ADR 0001's rule for the rows that compare an LG client's result with the server's, no verdict
   of those rows rests on native AArch64 agreeing with x86-64 or Wasm. parry3d's own `enhanced-determinism` flag
END

### NOTE R6B-RR-2
FILE: docs/adr/0004-simulation-core.md
no change: a grep of ADR 0004 outside its "Pass marks" section (lines 1–1013 and 1238–1421 on disk) for CX33, CCX13,
CCX23, CAX21, Hetzner, "cheaper", Nuremberg, nbg1, Helsinki, hel1, Falkenstein, fsn1, R6-HOST-OQ1, OQ-0006-, SH, MV,
DS, LG, "shared vCPU", "every server", "every native", "address", "order(ed|able)" and x86 finds no plan, no
location, no machine's orderability and no "cheaper host" in this author's text. Its places that name the server
(87 "the reference server (ADR 0006, R6-HOST-REC2)"; 234, 827, 979, 1005, 1007 and 1272, the glossary's "the
reference server") name the term only, never its plan or location, so Mustafa's check changes none of them. Every
"order" hit is an iteration, phase or slice order. The ARM places outside the pass marks (part 7 at 289–290, S7 at
997, R1-RR-OQ1's row at 1354) stay as condition 6 says; FIX R6B-RR-1 adds the one sentence. The CX33, "the cheaper
host", the location and R6-HOST-OQ1 appear only at 1030–1032 (the RS and CX33 keys), 1117 (PM-0004-1) and 1120
(PM-0004-4), all inside the "Pass marks" section: test-engineer's text.
END

### NOTE R6B-RR-3
FILE: docs/adr/0004-simulation-core.md
no change (for test-engineer, PM-0004-4, condition 6's statement): this author's Appendix A rows for R1-ECS-OQ4 (1329,
"spike: the sim core spike (S1); one thread per match unless S1 fails at N = 48") and R1-TICK-OQ5 (1334, "spike: the
sim core spike (S1, S2)") name no cheaper host, so they stand once PM-0004-4 drops its CX33 part. The research's own
deciders need no shared host either: R1-ECS-OQ4 is "Decided by: REC1 mark 1's native sim step time against the sim
step share … at the tick rate the tick unit picks, and R6's hosting cost per match" (r1-simulation-core.md, lines
2624–2626), and R1-TICK-OQ5 is "Decided by: REC2's P1–P3" (lines 2658–2660), measured on RS and, for P2 and P3, the
minimum laptop. So, in this author's view, both are still decided from RS alone on the server side (R1-ECS-OQ4's
cost side read at the CCX23's price, R6-HOST-F15). If test-engineer states instead that either stays open,
rows 1329 and 1334 need a matching change, which this round cannot make (no re-check round): say so in the report.
END

