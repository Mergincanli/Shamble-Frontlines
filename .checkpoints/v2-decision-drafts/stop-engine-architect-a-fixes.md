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
