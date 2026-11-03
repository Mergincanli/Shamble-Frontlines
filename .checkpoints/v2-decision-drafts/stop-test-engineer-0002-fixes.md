# test-engineer: ADR 0002 pass marks vs the stop answers (FIX/NOTE blocks)

### FIX H23-2
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
- No mark here is set at 60 Hz: TICK-30's baseline is 30 Hz, and the 60 Hz margin is ADR 0001's, measured in one run
  of the netcode proof test.
AFTER:
- No mark here is set at 60 Hz: TICK-30's baseline is 30 Hz, and the 60 Hz margin is ADR 0001's. Its wave-1 run,
  after the sim core spike (SPIKE-ORDER (b), Mustafa, 2026-11-03), is an early signal only, and slice 6's run on the
  real sim decides (TICK-60-SETUP, the Advisor, 2026-11-03: "(a), as an early signal only. The final 60 Hz call is
  measured again with the real sim at slice 6.").
END


### NOTE H23-3
no change: the Pass marks key's "Tick and profiles" bullet (docs/adr/0002-code-structure-build-toolchain.md:1206-1208:
"no mark here is set at 60 Hz ([the 60 Hz margin](../research/GLOSSARY.md#the-60-hz-margin) is ADR 0001's)") only
names the owner and links GLOSSARY §8; it says nothing on how many runs or which run decides. By the advisor's ruling
on TICK-60-HEADROOM, a restatement that links §8 may stay once §8 is amended, and engine-architect's FIX H24 and S23
amend that entry (wave 1's early signal, slice 6's deciding run, the goal budget at 60 Hz). The one line in these pass
marks that said how the margin is measured ("in one run of the netcode proof test") is FIX H23-2.
END
### FIX O18-5
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
| PM-0002-28 | The sim fits the client's sim share in this Worker (ADR 0004's S2; R1-TICK-REC2 P2) | ADR 0004's PM-0004-8, measured here once for both records: its numbers, its one reading of "holds 30+ fps on Low" (ADR 0003's minimum-laptop frame rows, median at most 28 ms and p95 at most 33.3 ms, mode U), its machines, runs and bites stand, and this row sets none of its own (as PM-0002-26 does for pacing). Restated: p99 [sim step time](../research/GLOSSARY.md#sim-step-time) at most 13.33 ms at 30 Hz in Wasm in this Worker on ML while the page meets that frame row for the whole run; N for the pass from R1-TICK-OQ7 (a design question), the scale steps 12, 24, 36 and 48 reported meanwhile; with the stand-in sim: reported only | ML: Chrome, Firefox | the sim core spike's sim in this Worker once it exists; step time from the Worker's `performance.now()` around the step; frame times in mode U (PM-0004-8's); the page draws with the plain renderer until ADR 0003's renderer runs at Low, and the run repeats with it then (proposed: "on Low" names ADR 0003's tier); run length and count as ADR 0004's pass marks set them | as ADR 0004's S2 | R1-TICK-REC2 P2 (r1, 2054–2057); GLOSSARY lines 214–218 (40 % of 33.33 ms = 13.33 ms); ADR 0004 S2 and PM-0004-8; the perf review of 2026-11-02, finding 5 (one mark, one reading) |
AFTER:
| PM-0002-28 | The sim fits the client's sim share in this Worker (ADR 0004's S2; R1-TICK-REC2 P2) | ADR 0004's PM-0004-8, measured here once for both records: its numbers, its one reading of "holds 30+ fps on Low" (ADR 0003's minimum-laptop frame rows, median at most 28 ms and p95 at most 33.3 ms, mode U), its machines, runs and bites stand, and this row sets none of its own (as PM-0002-26 does for pacing). Restated: p99 [sim step time](../research/GLOSSARY.md#sim-step-time) at most 13.33 ms at 30 Hz in Wasm in this Worker on ML while the page meets that frame row for the whole run; N for the pass from R1-TICK-OQ7 (a design question), the scale steps 12, 24, 36 and 48 reported meanwhile | ML: Chrome, Firefox | the sim core spike's sim in this Worker (that spike runs before this one: SPIKE-ORDER (b), Mustafa, 2026-11-03); step time from the Worker's `performance.now()` around the step; frame times in mode U (PM-0004-8's); the page draws with the plain renderer until ADR 0003's renderer runs at Low, and the run repeats with it then (proposed: "on Low" names ADR 0003's tier); run length and count as ADR 0004's pass marks set them | as ADR 0004's S2 | R1-TICK-REC2 P2 (r1, 2054–2057); GLOSSARY lines 214–218 (40 % of 33.33 ms = 13.33 ms); ADR 0004 S2 and PM-0004-8; the perf review of 2026-11-02, finding 5 (one mark, one reading) |
END

### FIX O18-6
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
  CPU the Worker may use (proposed: the real sim's cost is unknown until ADR 0004's sim core spike; once that sim
  runs in this Worker, the BX rows run again with it).
AFTER:
  CPU the Worker may use (proposed: the game's sim, from slice 1 on, may cost up to the share; the sim core spike's
  sim, which exists by then (that spike runs before this one: SPIKE-ORDER (b), Mustafa, 2026-11-03), also runs in
  this Worker, and the BX rows run again with it).
END


### NOTE O18-7
no change at the other spike-order places in these pass marks (docs/adr/0002-code-structure-build-toolchain.md):
- the BX key (:1170-1176), "the netcode proof test's throwaway sim ... in the Worker": the boundary spike still shares
  the proof test's stand-in sim, and the sim core spike's sim serves the marks that need it (the spike text at
  :1086-1087, engine-architect's, after FIX O18-2);
- the "Online" key (:1180-1183), "the netcode proof test's room process ... that test's link impairment": the proof
  test's kept code, which the boundary spike runs after that test (SPIKE-ORDER (b));
- PM-0002-26 (:1247), "with the sim core spike's sim at N = 48": already right under the new order;
- PM-0002-29 (:1250), "with parry3d once the sim core spike's sim is in": holds as written, since that sim is in this
  Worker during this spike (FIX O18-5, O18-6); it has no stand-in branch to drop;
- PM-0002-28's "the run repeats with it then" (ADR 0003's renderer at Low): the look proof test still comes after the
  boundary spike (SPIKE-ORDER (b)), so the renderer still arrives later;
- the spike checklist's step 5 (:1364), "Only if the session gives an online link": a conditional step, which holds
  when the proof test's server runs on a VM Mustafa creates for the run (SPIKE-MACHINES).
`docs/SPIKE_PLAN.md` item 3's "PM-0002-28 gates once the sim core spike's sim runs in the Worker" is engine-architect's
text and agrees with FIX O18-5.
END
### FIX N9-2
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
| PM-0002-49 | L1 is exact and true to its profiles (part 8: the network simulator; G11) | the same seed and profile give an identical packet-fate log and identical per-tick state hashes on two runs; over 1,000,000 packets per profile, measured loss, mean delay and jitter each within 5 % (relative) of P0–P4 and B1 (proposed count: see the note under the table); G11's one-minute run per profile takes under 2 minutes in total (a plain limit) | CI | L1 headless in Rust; loss, delay and jitter computed from the fate log; jitter read against the distribution L1's own documentation states (proposed: R2 names none) | 2 runs for the identity; 1 per profile for the statistics; G11's time over 5 runs | R5-TEST-REC4's mark (r5, 1469–1473); profiles (r2, 1987–1990) |
AFTER:
| PM-0002-49 | L1 is exact and true to its profiles (part 8: the network simulator; G11) | the same seed and profile give an identical packet-fate log and identical per-tick state hashes on two runs; over 1,000,000 packets per profile, in each direction, measured loss, mean one-way delay and jitter each within 5 % (relative) of the per-direction values of P0–P4 and B1, as part 8 splits them (as PM-0001-3 reads them; proposed count: see the note under the table); G11's one-minute run per profile takes under 2 minutes in total (a plain limit) | CI | L1 headless in Rust; loss, delay and jitter computed from the fate log; jitter read against the distribution L1's own documentation states (proposed: R2 names none) | 2 runs for the identity; 1 per profile for the statistics; G11's time over 5 runs | R5-TEST-REC4's mark (r5, 1469–1473); profiles (r2, 1987–1990); the split each way: part 8 and ADR 0001 part 13 (OQ-0001-16, answered by the Advisor, 2026-11-03) |
END


### NOTE P2-1
no change at the other profile places in these pass marks (docs/adr/0002-code-structure-build-toolchain.md): the
"Online" key (:1183) gives "P2 = 100 ms round trip, ±20 ms jitter, 2 % loss", the Advisor's P2 value for value
(NET-PROFILES, 2026-11-03: "100 ms round trip, ±20 ms jitter, 2 % loss"). The online rows (PM-0002-3, -22, -23, -26,
-77, -78) run through the netcode proof test's link impairment, which applies the split each way as ADR 0001 part 13
now states it; they measure the boundary, not the network, so none restates the split. PM-0002-50's "one slot too
short for P2" and PM-0002-72's and -76's "over L1 at P2" read P2 as the profile, which is unchanged. The one row that
checks the profiles' numbers is PM-0002-49 (FIX N9-2). Left as it was, outside the stop's answers: PM-0002-49's
"jitter read against the distribution L1's own documentation states (proposed: R2 names none)" beside ADR 0001 part
13's proposed uniform shape (that proposal predates 03ef222); for fixcheck-e to weigh, not for this round.
END
