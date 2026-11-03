# stop-engine-architect-b fixes (engine-architect, synthesis docs: README, SPIKE_PLAN, ENGINE_OVERVIEW)

### FIX RC9
FILE: docs/adr/README.md
BEFORE:
| ROOM-CRASH (OQ-0001-18) | When a room's simulation panics mid-match, does the match end (proposed), or does the room restore its last snapshot and continue? | the Advisor | [ADR 0001](0001-multiplayer-netcode.md) line 1155 |
AFTER:
| ROOM-CRASH (OQ-0001-18) | When a room's simulation panics mid-match, does the match end, or does the room restore its last snapshot and continue? Answered by the Advisor (2026-11-03): "end the match, as proposed, and keep the crash and match records." "Keep" means written and kept under ADR 0006 part 11's proposed retention, with no new period (the advisor's reading, 2026-11-03). | the Advisor | [ADR 0001](0001-multiplayer-netcode.md) line 1155 |
END

### FIX RC10
FILE: docs/adr/README.md
BEFORE:
| [0001](0001-multiplayer-netcode.md) | Multiplayer and netcode | netcode-architect; pass marks: test-engineer | Proposed | the netcode proof test; the 60 Hz margin run | 18 (OQ-0001-1 to -18) |
| [0002](0002-code-structure-build-toolchain.md) | Code structure, build and toolchain | engine-architect; pass marks: test-engineer | Proposed | the Rust ↔ TypeScript boundary spike; the foundation's acceptance checks (not a spike) | 8 (OQ-0002-1 to -8) |
| [0003](0003-rendering.md) | Rendering and presentation | graphics-engineer; pass marks: test-engineer | Proposed | the look proof test; the render budget bench | 20 numbered (OQ-0003-1 to -20), 19 open: -14 answered by its decider, the verifier |
| [0004](0004-simulation-core.md) | The simulation core | rust-reviewer; pass marks: test-engineer | Proposed | the sim core spike; shares in the netcode proof test, the boundary spike and the content and ability spike | 12 numbered (OQ-0004-1 to -12), 10 open: -5 and -6 moved to OQ-0005-AB9 and AB10 |
| [0005](0005-content-abilities-assets.md) | Content data, abilities and assets | engine-architect (frame, content data, asset pipeline); game-designer (ability and effect system); graphics-engineer (the asset pipeline's GPU rules); pass marks: test-engineer | Proposed | the content and ability spike; a share of the look proof test; the asset spike (after the asset-pipeline research); the Feel Lab test | 36 (CD1–CD8, AB1–AB15, AP1–AP3, AS1–AS6, OQ-0005-1 to -4) |
| [0006](0006-infrastructure-operations.md) | Infrastructure and operations | infra-engineer; pass marks: test-engineer | Proposed | the preview hosting check; operations marks in the netcode proof test; an optional share of the boundary spike | 20 (OQ-0006-1 to -20) |
AFTER:
| [0001](0001-multiplayer-netcode.md) | Multiplayer and netcode | netcode-architect; pass marks: test-engineer | Proposed. Signed off by Mustafa: as the basis for the spikes (2026-11-03, SPIKE-ORDER (b)); not yet accepted | the netcode proof test; the 60 Hz margin run (an early signal; slice 6's run on the real sim makes the call) | 18 numbered (OQ-0001-1 to -18), 12 open: -1, -2, -3, -14, -16 and -18 answered at the stop (2026-11-03); -5 answered in part |
| [0002](0002-code-structure-build-toolchain.md) | Code structure, build and toolchain | engine-architect; pass marks: test-engineer | Proposed. Signed off by Mustafa: as the basis for the spikes (2026-11-03, SPIKE-ORDER (b)); not yet accepted | the Rust ↔ TypeScript boundary spike; the foundation's acceptance checks (not a spike) | 8 numbered (OQ-0002-1 to -8), 7 open: -8 answered at the stop (2026-11-03) |
| [0003](0003-rendering.md) | Rendering and presentation | graphics-engineer; pass marks: test-engineer | Proposed. Signed off by Mustafa: as the basis for the spikes (2026-11-03, SPIKE-ORDER (b)); not yet accepted | the look proof test; the render budget bench | 20 numbered (OQ-0003-1 to -20), 17 open: -14 answered by its decider, the verifier; -17 and -20 answered at the stop (2026-11-03) |
| [0004](0004-simulation-core.md) | The simulation core | rust-reviewer; pass marks: test-engineer | Proposed. Signed off by Mustafa: as the basis for the spikes (2026-11-03, SPIKE-ORDER (b)); not yet accepted | the sim core spike; shares in the netcode proof test, the boundary spike and the content and ability spike | 12 numbered (OQ-0004-1 to -12), 10 open: -5 and -6 moved to OQ-0005-AB9 and AB10 |
| [0005](0005-content-abilities-assets.md) | Content data, abilities and assets | engine-architect (frame, content data, asset pipeline); game-designer (ability and effect system); graphics-engineer (the asset pipeline's GPU rules); pass marks: test-engineer | Proposed. Signed off by Mustafa: as the basis for the spikes (2026-11-03, SPIKE-ORDER (b)); not yet accepted | the content and ability spike; a share of the look proof test; the asset spike (after the asset-pipeline research); the Feel Lab test | 36 numbered (CD1–CD8, AB1–AB15, AP1–AP3, AS1–AS6, OQ-0005-1 to -4), 33 open: OQ-0005-1, AB12 and AB13 answered at the stop (2026-11-03) |
| [0006](0006-infrastructure-operations.md) | Infrastructure and operations | infra-engineer; pass marks: test-engineer | Proposed. Signed off by Mustafa: as the basis for the spikes (2026-11-03, SPIKE-ORDER (b)); not yet accepted | the preview hosting check; operations marks in the netcode proof test; an optional share of the boundary spike | 20 numbered (OQ-0006-1 to -20), 19 open: -17 answered at the stop (2026-11-03); -5, -15 and -20 answered in part |
END

### FIX O14
FILE: docs/adr/README.md
BEFORE:
Total: 114 numbered open questions, 111 of them open (OQ-0004-5 and -6 point to ADR 0005; OQ-0003-14 is answered);
SPIKE_PLAN adds one of its own, OQ-SPIKE-1. Pass marks: 62, 78, 47, 75, 46 and 32 rows (340). None of the drafts is
signed off; no spike starts before Mustafa and the Advisor approve them (the drafts brief, §4).
AFTER:
Total: 114 numbered open questions, 98 of them open (OQ-0004-5 and -6 point to ADR 0005; OQ-0003-14 is answered; 13
were answered at the stop, 2026-11-03; a question answered in part stays open, marked "answered in part"); SPIKE_PLAN
adds one of its own, OQ-SPIKE-1, answered by Mustafa (2026-11-03). Pass marks: 63, 78, 47, 75, 46 and 32 rows (341;
ADR 0001's PM-0001-63 joined its 60 Hz group at the stop). Status of every draft: Proposed. Signed off by Mustafa: as
the basis for the spikes (2026-11-03, SPIKE-ORDER (b)); not yet accepted. His words: "(b) accept with one change. The
six drafts are the basis for the spikes, in SPIKE_PLAN's order, but wave 1 runs one heavy spike at a time". A draft
becomes Accepted only when Mustafa signs it off after its spikes report; marks passing never does it on its own (the
advisor's reading, 2026-11-03).
END

### FIX H18
FILE: docs/adr/README.md
BEFORE:
| TICK-60-HEADROOM (OQ-0001-1) | Is the 60 Hz margin measured against the 60 Hz server tick budget (8.33 ms), so 60 Hz needs a p99 server tick time of at most 6.25 ms plus every 60 Hz research mark? | Mustafa and the Advisor (confirm at the stop) | [ADR 0001](0001-multiplayer-netcode.md) part 2, line 1022; [GLOSSARY §8](../research/GLOSSARY.md#the-60-hz-margin) |
AFTER:
| TICK-60-HEADROOM (OQ-0001-1) | Is the 60 Hz margin measured against the 60 Hz server tick budget (8.33 ms), so 60 Hz needs a p99 server tick time of at most 6.25 ms plus every 60 Hz research mark? Answered by Mustafa (2026-11-03): "confirm, with one addition: 60 Hz also needs the goal budget to hold at the rooms per VM measured at 60 Hz." The goal budget is ADR 0006 part 8's as it stands when the call is made (at slice 6, the figure OQ-0006-19 fixes); it holds when the fleet the goal modes need at the rooms per VM measured at 60 Hz (part 8's formula, same machine, N = 48) costs no more than that figure (the advisor's reading, 2026-11-03). The new mark is ADR 0001's PM-0001-63. | Mustafa and the Advisor (confirm at the stop) | [ADR 0001](0001-multiplayer-netcode.md) part 2, line 1022; [GLOSSARY §8](../research/GLOSSARY.md#the-60-hz-margin) |
END

### FIX S21
FILE: docs/adr/README.md
BEFORE:
| TICK-60-SETUP (OQ-0001-2) | How does the stress scene's sim load reach the 60 Hz margin run, when the wave-1 sim has no ability and effect step, economy, objectives or moments yet: (a) the sim core spike's sim inside the proof-test server, each missing part stood in by burning its ceiling as ADR 0004's S1 does (recommended; the test review's Q1), (b) a stand-in replaying measured step times, (c) wait until the real sim runs the stress scene, (d) wait until the content and ability spike's effect step has joined? | the Advisor | [ADR 0001](0001-multiplayer-netcode.md) line 1029; SPIKE_PLAN item 2 |
AFTER:
| TICK-60-SETUP (OQ-0001-2) | How does the stress scene's sim load reach the 60 Hz margin run, when the wave-1 sim has no ability and effect step, economy, objectives or moments yet: (a) the sim core spike's sim inside the proof-test server, each missing part stood in by burning its ceiling as ADR 0004's S1 does (recommended; the test review's Q1), (b) a stand-in replaying measured step times, (c) wait until the real sim runs the stress scene, (d) wait until the content and ability spike's effect step has joined? Answered by the Advisor (2026-11-03): "(a), as an early signal only. The final 60 Hz call is measured again with the real sim at slice 6." Slice 6 always runs that measurement, whatever the early signal shows; the early run decides nothing, a pass starts no 60 Hz work before slice 6, and a fail closes nothing (the advisor's reading, 2026-11-03). | the Advisor | [ADR 0001](0001-multiplayer-netcode.md) line 1029; SPIKE_PLAN item 2 |
END

### FIX S21-2
FILE: docs/adr/README.md
BEFORE:
| LE-K | the netcode proof test runs before the foundation with a throwaway sim | OQ-0001-2 (TICK-60-SETUP) |
AFTER:
| LE-K | the netcode proof test runs before the foundation with a throwaway sim | OQ-0001-2 (TICK-60-SETUP), answered (a) by the Advisor (2026-11-03): the wave-1 60 Hz run is an early signal only, and slice 6 makes the final 60 Hz call on the real sim |
END

### NOTE S21-3
no change: README:380 (J6) defines what the decision map's part j held on 2026-10-31 ("TICK-60-SETUP for the
Advisor" among them); that stays a true definition of the code. The answer is recorded in rows :35 (FIX S21) and :257
(FIX S21-2).
END

### FIX N16
FILE: docs/adr/README.md
BEFORE:
| "100 ms" reading (OQ-0001-3) | Does the netcode proof test's "100 ms of simulated latency" mean profile P2 (100 ms round trip, with P2's jitter and loss), or 100 ms each way? | the Advisor (who wrote the definition) | [ADR 0001](0001-multiplayer-netcode.md) line 1049 |
AFTER:
| "100 ms" reading (OQ-0001-3) | Does the netcode proof test's "100 ms of simulated latency" mean profile P2 (100 ms round trip, with P2's jitter and loss), or 100 ms each way? Answered by the Advisor (2026-11-03): “yes. "100 ms" is P2: 100 ms round trip, ±20 ms jitter, 2 % loss (VISION_V2 §4's "about 100 ms of ping"). Latency and jitter split half each way; loss as P2 defines it.” | the Advisor (who wrote the definition) | [ADR 0001](0001-multiplayer-netcode.md) line 1049 |
END

### FIX N16-2
FILE: docs/adr/README.md
BEFORE:
| OQ-0001-16 | How does each network profile split its latency and jitter over the two directions (half each way is proposed)? | the Advisor, before the proof test starts | [ADR 0001](0001-multiplayer-netcode.md) line 1138 |
AFTER:
| OQ-0001-16 | How does each network profile split its latency and jitter over the two directions (half each way was proposed)? Answered by the Advisor (2026-11-03): "Latency and jitter split half each way; loss as P2 defines it." Loss as P2 defines it is 2 % on each direction, applied independently, as PM-0001-3 measures, so about 4 % of round trips lose a packet (the advisor's reading, 2026-11-03). | the Advisor, before the proof test starts | [ADR 0001](0001-multiplayer-netcode.md) line 1138 |
END

### FIX N16-3
FILE: docs/adr/README.md
BEFORE:
| LE-F | brief §2's "100 ms" does not say round trip or one way | OQ-0001-3 |
AFTER:
| LE-F | brief §2's "100 ms" does not say round trip or one way | OQ-0001-3, answered by the Advisor (2026-11-03): P2, a 100 ms round trip |
END

### FIX RW16
FILE: docs/adr/README.md
BEFORE:
| OQ-0001-5 | The rewind cap: which value (200 ms, 250 ms, or one set from the proof test's measured honest rewind), and the victim's side? | Mustafa and the Advisor | [ADR 0001](0001-multiplayer-netcode.md) line 1060 |
AFTER:
| OQ-0001-5 | The rewind cap: which value (200 ms, 250 ms, or one set from the proof test's measured honest rewind), and the victim's side? Answered in part by Mustafa (2026-11-03): "no value now. Rule: the cap covers the honest rewind the proof test measures at P2, rounded up; beyond the cap, the victim's view wins. Aim for the smallest input buffer that holds." The honest rewind measured at P2 is its p99 at P2, the run's largest rewind reported beside it, rounded up to a whole number of ticks at the run's tick rate (the advisor's reading, 2026-11-03); still open: the value, recorded when the netcode proof test reports, decided by its measurement under that rule. | Mustafa and the Advisor | [ADR 0001](0001-multiplayer-netcode.md) line 1060 |
END

### FIX O13
FILE: docs/adr/README.md
BEFORE:
| Spike order (J6 (5)) | Only the netcode proof test's place is settled (before the foundation); is SPIKE_PLAN's proposed order of the rest accepted, with the builders it names for the shared client harness (the netcode proof test) and the measurement harness's CPU side (the boundary spike)? | Mustafa and the Advisor | [SPIKE_PLAN](../SPIKE_PLAN.md), "The spikes, in the proposed order" |
AFTER:
| Spike order (J6 (5)) | Only the netcode proof test's place is settled (before the foundation); is SPIKE_PLAN's proposed order of the rest accepted, with the builders it names for the shared client harness (the netcode proof test) and the measurement harness's CPU side (the boundary spike)? Answered by Mustafa (2026-11-03, SPIKE-ORDER): "(b) accept with one change. The six drafts are the basis for the spikes, in SPIKE_PLAN's order, but wave 1 runs one heavy spike at a time: the netcode proof test first, then the sim core spike (then the 60 Hz run), then the boundary spike, then the look proof test once its models exist. The preview hosting check may run alongside any of them. Reason: seven usage-limit cuts in three days." | Mustafa and the Advisor | [SPIKE_PLAN](../SPIKE_PLAN.md), "The spikes, in their order" |
END

### FIX M23
FILE: docs/adr/README.md
BEFORE:
| Reference machines (OQ-0003-1) | Which laptop (the minimum) and which desktop (the recommended) are bought or borrowed as the two reference machines? | Mustafa and the Advisor (money and availability) | [ADR 0003](0003-rendering.md) line 800 |
AFTER:
| Reference machines (OQ-0003-1) | Which laptop (the minimum) and which desktop (the recommended) are bought or borrowed as the two reference machines? Still open after the stop: Mustafa's "Test PC: my own, in Brave" (2026-11-03) is MP, what he judges and never a measured result, so it stands in for none of ML, RD and MAC, and the netcode proof test cannot start without ML and RD (the advisor's reading, 2026-11-03). | Mustafa and the Advisor (money and availability) | [ADR 0003](0003-rendering.md) line 800 |
END

### FIX M23-2
FILE: docs/adr/README.md
BEFORE:
| The `wss` certificate on development and staging (OQ-0006-5) | How are the per-hostname production certificates issued, how long do they live, and which certificate does the WebSocket fallback present on development and staging servers, since a WebSocket cannot pin a certificate hash? The netcode proof test's WebSocket leg needs the development and staging answer first, and a trusted certificate there needs a domain (OQ-0006-15). | security-auditor in the security baseline, with the port plan (OQ-0006-16); the development and staging case before the netcode proof test starts (the Advisor, from a short read) | [ADR 0006](0006-infrastructure-operations.md) line 807; SPIKE_PLAN item 1 |
AFTER:
| The `wss` certificate on development and staging (OQ-0006-5) | How are the per-hostname production certificates issued, how long do they live, and which certificate does the WebSocket fallback present on development and staging servers, since a WebSocket cannot pin a certificate hash? The netcode proof test's WebSocket leg needs the development and staging answer first, and a trusted certificate there needs a domain (OQ-0006-15). Answered in part by Mustafa (2026-11-03): "Certificates: a private hash-pinned certificate for WebTransport; for the WebSocket leg, a domain I register." The WebSocket leg uses the development case (i), a trusted certificate on his domain (the advisor's reading, 2026-11-03); still open: how that certificate is issued on a spike VM (http-01 and tls-alpn-01 need an open port, dns-01 a secret; OQ-0006-16's port plan applies), decided in the netcode proof test's own plan with security-auditor; the staging case, decided by the Advisor; and the production certificates, decided by security-auditor in the security baseline. | security-auditor in the security baseline, with the port plan (OQ-0006-16); the development and staging case before the netcode proof test starts (the Advisor, from a short read) | [ADR 0006](0006-infrastructure-operations.md) line 807; SPIKE_PLAN item 1 |
END

### FIX M23-3
FILE: docs/adr/README.md
BEFORE:
| OQ-0006-1 | Can Mustafa order Hetzner's CCX23 and CX33 in an EU location? | Mustafa (a console check in his own account; agents create no account), then a research read if (b) | [ADR 0006](0006-infrastructure-operations.md) line 762 |
AFTER:
| OQ-0006-1 | Can Mustafa order Hetzner's CCX23 and CX33 in an EU location? Still open after the stop (2026-11-03): his console check. | Mustafa (a console check in his own account; agents create no account), then a research read if (b) | [ADR 0006](0006-infrastructure-operations.md) line 762 |
END

### FIX M23-4
FILE: docs/adr/README.md
BEFORE:
| OQ-0006-15 (DOMAIN) | Which domain do the client host, the lobby and the game servers use, and when is it registered? | Mustafa (an account and a cost), after the Advisor's OQ-0006-5 read | [ADR 0006](0006-infrastructure-operations.md) line 934 |
AFTER:
| OQ-0006-15 (DOMAIN) | Which domain do the client host, the lobby and the game servers use, and when is it registered? Answered in part by Mustafa (2026-11-03): "for the WebSocket leg, a domain I register." It is registered before the netcode proof test (the advisor's reading, 2026-11-03); still open: which domain, which registrar and which DNS host, and whether production uses it later, decided by Mustafa. | Mustafa (an account and a cost), after the Advisor's OQ-0006-5 read | [ADR 0006](0006-infrastructure-operations.md) line 934 |
END

### FIX M23-5
FILE: docs/adr/README.md
BEFORE:
| OQ-0006-17 (SPIKE-VM-ACCESS) | How do sessions get VMs for the spikes, the tick bench and the 3-hour soak before each playtest without holding a secret? | Mustafa (his account), with the Advisor | [ADR 0006](0006-infrastructure-operations.md) line 962 |
AFTER:
| OQ-0006-17 (SPIKE-VM-ACCESS) | How do sessions get VMs for the spikes, the tick bench and the 3-hour soak before each playtest without holding a secret? Answered by Mustafa (2026-11-03): "a separate Hetzner project. I create and delete the VMs myself for each run; sessions get SSH access only and no Hetzner token until the security baseline. VMs exist only during a run." The baseline comes before the first online playtest, so this covers the tick bench and the 3-hour soaks too; how a session's SSH key is made and held without it reading a secret it did not make is for the netcode proof test's own plan, with security-auditor and the advisor (the advisor's reading, 2026-11-03). | Mustafa (his account), with the Advisor | [ADR 0006](0006-infrastructure-operations.md) line 962 |
END

### FIX M23-6
FILE: docs/adr/README.md
BEFORE:
| OQ-0006-20 | What may the netcode proof test's VMs cost? | Mustafa (money) | [ADR 0006](0006-infrastructure-operations.md) line 1003 |
AFTER:
| OQ-0006-20 | What may the netcode proof test's VMs cost? Answered in part by Mustafa (2026-11-03): "VMs exist only during a run."; still open: the cost cap, decided by Mustafa (money). | Mustafa (money) | [ADR 0006](0006-infrastructure-operations.md) line 1003 |
END

### FIX L6
FILE: docs/adr/README.md
BEFORE:
| ASSET-RESEARCH (timing) | The asset-pipeline research runs before the asset spike; should it run earlier (for example beside the first spikes) so art production can start sooner, or so its slice on export checks and slot renaming answers OQ-SPIKE-1 before the look proof test? | Mustafa and the Advisor | [ADR 0005](0005-content-abilities-assets.md) line 76 and the asset pipeline part; SPIKE_PLAN, "The asset-pipeline research" |
AFTER:
| ASSET-RESEARCH (timing) | The asset-pipeline research runs before the asset spike; should it run earlier (for example beside the first spikes) so art production can start sooner, or so its slice on export checks and slot renaming answers OQ-SPIKE-1 before the look proof test? Answered by Mustafa (2026-11-03, LOOK-TEST-INPUTS): "yes, the asset research's export and slot-naming part runs before the look proof test." The rest of the research stays before the asset spike; the part is agent research, not a heavy spike, so it does not run alongside one but between two wave-1 spikes, early enough that its export and slot rules reach Mustafa before he makes the models (recommended: right after the netcode proof test; the advisor's reading, 2026-11-03). | Mustafa and the Advisor | [ADR 0005](0005-content-abilities-assets.md) line 76 and the asset pipeline part; SPIKE_PLAN, "The asset-pipeline research" |
END

### FIX L6-2
FILE: docs/adr/README.md
BEFORE:
| Look test models (OQ-SPIKE-1) | Who makes the look proof test's authored models (a skinned soldier with faction colour slots, the houses, a very large unit of at least 20 parts, as glTF/GLB with named slots), and by which route, since whether Spline's export carries a skin and named slots is not researched? (the graphics review's Q2) | Mustafa and the Advisor, with ASSET-RESEARCH's timing | [SPIKE_PLAN](../SPIKE_PLAN.md), "Open questions"; [ADR 0003](0003-rendering.md) part 11 |
AFTER:
| Look test models (OQ-SPIKE-1) | Who makes the look proof test's authored models (a skinned soldier with faction colour slots, the houses, a very large unit of at least 20 parts, as glTF/GLB with named slots), and by which route, since whether Spline's export carries a skin and named slots is not researched? (the graphics review's Q2) Answered by Mustafa (2026-11-03): "Route: Blender (Spline's skins and named slots are unproven). I make the test models with AI help in Blender; blockouts with a skin, named colour slots and LOOK_V2's proportions are enough." LOOK_V2 is not edited: Blender is the route for the look test's models and the research part's export (the advisor's reading, 2026-11-03). | Mustafa and the Advisor, with ASSET-RESEARCH's timing | [SPIKE_PLAN](../SPIKE_PLAN.md), "Open questions"; [ADR 0003](0003-rendering.md) part 11 |
END

### FIX E6
FILE: docs/adr/README.md
BEFORE:
| Structure without a research option (OQ-0002-8) | May a record propose a structure no research Options section offers, when a settled input forces it and the research leaves the question to that record (the `room` and `navbake` crates)? | the Advisor, at the stop | [ADR 0002](0002-code-structure-build-toolchain.md) line 1053 |
AFTER:
| Structure without a research option (OQ-0002-8) | May a record propose a structure no research Options section offers, when a settled input forces it and the research leaves the question to that record (the `room`, `client-endpoint` and `navbake` crates)? Answered by the Advisor (2026-11-03): "(a)." The three crates stay proposed, labelled "not researched", and the first foundation milestone that builds them is their test (option (a) in ADR 0002). | the Advisor, at the stop | [ADR 0002](0002-code-structure-build-toolchain.md) line 1053 |
END

### FIX C10
FILE: docs/adr/README.md
BEFORE:
| OQ-0005-1 | Do presentation references sit on the simulation definitions, or in client definitions keyed by the same id? | the Advisor, at the stop review (the game-designer changes AB-4's kinds if the answer needs it) | [ADR 0005](0005-content-abilities-assets.md) line 2039 |
AFTER:
| OQ-0005-1 | Do presentation references sit on the simulation definitions, or in client definitions keyed by the same id? Answered by the Advisor (2026-11-03): "the client's data, keyed by the same id; the content check fails if a simulation id has no client entry." The rule binds the kinds the client presents, which ADR 0005's CD-2 names; kinds with nothing to present (tuning tables, modes) need no entry (the advisor's reading, 2026-11-03). | the Advisor, at the stop review (the game-designer changes AB-4's kinds if the answer needs it) | [ADR 0005](0005-content-abilities-assets.md) line 2039 |
END

### FIX X4
FILE: docs/adr/README.md
BEFORE:
| Trace reading (OQ-0001-14; LE-G, R2-XU-OQ18) | Is a VISION_V2 §3 or §4 target trace enough for an engine feature (written "need: VISION_V2 §4 only" and listed below)? The session read it so on 2026-10-31. | the Advisor, at the stop | [ADR 0001](0001-multiplayer-netcode.md) line 1120; "Needs outside VISION_V2 §6" below |
AFTER:
| Trace reading (OQ-0001-14; LE-G, R2-XU-OQ18) | Is a VISION_V2 §3 or §4 target trace enough for an engine feature (written "need: VISION_V2 §4 only" and listed below)? The session read it so on 2026-10-31. Answered by the Advisor (2026-11-03): "yes, a VISION_V2 §3 or §4 trace counts, marked as such." No need line changes: each is already marked (the advisor's reading, 2026-11-03). | the Advisor, at the stop | [ADR 0001](0001-multiplayer-netcode.md) line 1120; "Needs outside VISION_V2 §6" below |
END

### FIX X4-2
FILE: docs/adr/README.md
BEFORE:
| LE-G | does a VISION_V2 §3 or §4 target count as a need | "Decisions the drafts ask for" (trace reading) |
AFTER:
| LE-G | does a VISION_V2 §3 or §4 target count as a need | "Decisions the drafts ask for" (trace reading): yes, answered by the Advisor (2026-11-03, OQ-0001-14) |
END

### FIX X4-3
FILE: docs/adr/README.md
BEFORE:
Features a draft proposes whose need is only in LOOP_V2, EDITOR_V2, LOOK_V2, UPGRADE-SCOPE, GLOSSARY §8's foundation or
a VISION_V2 §3/§4 target (plan rule 4, the advisor's condition 8, and the session's trace reading of 2026-10-31, decision
map J3), one line each, for the stop review. Each draft's "Need:" line is the source.
AFTER:
Features a draft proposes whose need is only in LOOP_V2, EDITOR_V2, LOOK_V2, UPGRADE-SCOPE, GLOSSARY §8's foundation or
a VISION_V2 §3/§4 target (plan rule 4, the advisor's condition 8, and the trace rule: the session's reading of
2026-10-31, decision map J3, which the Advisor confirmed on 2026-11-03 in the answer to OQ-0001-14), one line each,
for the stop review. Each draft's "Need:" line is the source.
END

### NOTE X4-4
no change: README:377 (J3) defines what the decision map's part j held on 2026-10-31 (the trace rule and the
session's reading); that stays a true definition of the code. The Advisor's answer is recorded in rows :46 (FIX X4),
:253 (FIX X4-2) and the "Needs outside VISION_V2 §6" intro (FIX X4-3).
END

### FIX F7
FILE: docs/adr/README.md
BEFORE:
| OQ-0003-17 | How is VISION_V2 §6.9's "without dropping frames" read as a mark: how many of 600 presented frames may miss a refresh with the very large units on screen, or the p95 rows as written? | the Advisor, confirmed by Mustafa at the stop review (VISION_V2 is theirs) | [ADR 0003](0003-rendering.md) line 914 |
AFTER:
| OQ-0003-17 | How is VISION_V2 §6.9's "without dropping frames" read as a mark: how many of 600 presented frames may miss a refresh with the very large units on screen, or the p95 rows as written? Answered by the Advisor, confirmed by Mustafa (2026-11-03): "at most 6 of 600 presented frames may miss a refresh with the very large units on screen (recommended desktop, High); the p95 rows stay as diagnostics; I confirm this reading." PM-0003-44 is gated on RD at High and reported on ML; the p95 rows are diagnostics for §6.9's reading, and PM-0003-18 to -21 keep gating their own needs (the advisor's reading, 2026-11-03). | the Advisor, confirmed by Mustafa at the stop review (VISION_V2 is theirs) | [ADR 0003](0003-rendering.md) line 914 |
END

### FIX Q6
FILE: docs/adr/README.md
BEFORE:
| OQ-0003-20 | Is the minimum laptop's budget judged after 15 minutes of play, or on a cold laptop? (the perf review's HEAT-SOAK) | the Advisor | [ADR 0003](0003-rendering.md) line 938 |
AFTER:
| OQ-0003-20 | Is the minimum laptop's budget judged after 15 minutes of play, or on a cold laptop? (the perf review's HEAT-SOAK) Answered by the Advisor (2026-11-03): "judge after 15 minutes of play." The soaked set gates PM-0003-18 to -21 on ML (the advisor's reading, 2026-11-03). | the Advisor | [ADR 0003](0003-rendering.md) line 938 |
END

### FIX A5
FILE: docs/adr/README.md
BEFORE:
| OQ-0005-AB12 | Is tranek's write-up accepted as the GAS reference, or does a verifier read Epic's documentation first? | the Advisor | [ADR 0005](0005-content-abilities-assets.md) line 1332 |
AFTER:
| OQ-0005-AB12 | Is tranek's write-up accepted as the GAS reference, or does a verifier read Epic's documentation first? Answered by the Advisor (2026-11-03): "tranek's write-up is accepted for concepts; any specific claim a record relies on is checked against Epic's documentation once it can be read." | the Advisor | [ADR 0005](0005-content-abilities-assets.md) line 1332 |
END

### FIX B7
FILE: docs/adr/README.md
BEFORE:
| OQ-0005-AB13 | When does scripting (option A3) come back? | the Advisor (the source) | [ADR 0005](0005-content-abilities-assets.md) line 1341 |
AFTER:
| OQ-0005-AB13 | When does scripting (option A3) come back? Answered by the Advisor (2026-11-03): "scripting comes back only when a designed ability can't be built from data." Built from data includes a new reviewed generic operation, so scripting comes back when one designed ability needs ability-specific code no operation can express; option (b), reading the Overwatch talk, stays a weighed option that nothing schedules (the advisor's reading, 2026-11-03). | the Advisor (the source) | [ADR 0005](0005-content-abilities-assets.md) line 1341 |
END

### FIX LA1-2
FILE: docs/adr/README.md
BEFORE:
| UPGRADE-SCOPE: carry slots (OQ-0005-AB1) | How many weapons can a player carry besides the class weapon, and what happens to the one in hand on a pickup? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1188 |
| UPGRADE-SCOPE: looted ammo (OQ-0005-AB2) | What does a looted gun come with, and can it be refilled? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1200 |
| UPGRADE-SCOPE: levelling (OQ-0005-AB3) | What earns a level, how fast, and does a track the player switches to catch up? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1208 |
AFTER:
| UPGRADE-SCOPE: carry slots (OQ-0005-AB1) | How many weapons can a player carry besides the class weapon, and what happens to the one in hand on a pickup? | Mustafa; LATER: the design session after the netcode proof test | [ADR 0005](0005-content-abilities-assets.md) line 1188 |
| UPGRADE-SCOPE: looted ammo (OQ-0005-AB2) | What does a looted gun come with, and can it be refilled? | Mustafa; LATER: the design session after the netcode proof test | [ADR 0005](0005-content-abilities-assets.md) line 1200 |
| UPGRADE-SCOPE: levelling (OQ-0005-AB3) | What earns a level, how fast, and does a track the player switches to catch up? | Mustafa; LATER: the design session after the netcode proof test | [ADR 0005](0005-content-abilities-assets.md) line 1208 |
END

### FIX LA1-3
FILE: docs/adr/README.md
BEFORE:
| OQ-0003-16 | The look floor: which LOOK_V2 items may a quality tier switch off on the minimum laptop? (the graphics review's Q1, does every `[DECIDED]` item hold at Low; the art review's LOOK-FLOOR) | Mustafa, in the look proof test, on the Low frame and the techniques' measured costs | [ADR 0003](0003-rendering.md) line 898 |
AFTER:
| OQ-0003-16 | The look floor: which LOOK_V2 items may a quality tier switch off on the minimum laptop? (the graphics review's Q1, does every `[DECIDED]` item hold at Low; the art review's LOOK-FLOOR) | Mustafa, in the look proof test, on the Low frame and the techniques' measured costs; LATER, keeping its own timing: decided in the look proof test (the advisor's reading, 2026-11-03) | [ADR 0003](0003-rendering.md) line 898 |
END

### FIX LA1-4
FILE: docs/adr/README.md
BEFORE:
| OQ-0004-8 | How does an uplink steal meet defenders, and two enemy teams? | Mustafa, with the game-designer | [ADR 0004](0004-simulation-core.md) line 902 |
| OQ-0004-9 | Is a carried tech cache's position public? | Mustafa | [ADR 0004](0004-simulation-core.md) line 915 |
| OQ-0004-10 | Does a player who rejoins after the grace window get their player record back? | Mustafa, with OQ-0005-AB10 (what a human gets on taking a bot's slot) | [ADR 0004](0004-simulation-core.md) line 926 |
| OQ-0004-11 | When a pool is full, which row gives way? (ADR 0005 AB-1 proposes the rule per kind; the reviews' POOL-FULL) | the game-designer proposes per kind; Mustafa confirms where a player could notice | [ADR 0004](0004-simulation-core.md) line 938 |
| OQ-0004-12 | Do bots take world pickups? | Mustafa | [ADR 0004](0004-simulation-core.md) line 951 |
AFTER:
| OQ-0004-8 | How does an uplink steal meet defenders, and two enemy teams? | Mustafa, with the game-designer; LATER: the design session after the netcode proof test | [ADR 0004](0004-simulation-core.md) line 902 |
| OQ-0004-9 | Is a carried tech cache's position public? | Mustafa; LATER: the design session after the netcode proof test | [ADR 0004](0004-simulation-core.md) line 915 |
| OQ-0004-10 | Does a player who rejoins after the grace window get their player record back? | Mustafa, with OQ-0005-AB10 (what a human gets on taking a bot's slot); LATER: the design session after the netcode proof test | [ADR 0004](0004-simulation-core.md) line 926 |
| OQ-0004-11 | When a pool is full, which row gives way? (ADR 0005 AB-1 proposes the rule per kind; the reviews' POOL-FULL) | the game-designer proposes per kind; Mustafa confirms where a player could notice; LATER: the design session after the netcode proof test | [ADR 0004](0004-simulation-core.md) line 938 |
| OQ-0004-12 | Do bots take world pickups? | Mustafa; LATER: the design session after the netcode proof test | [ADR 0004](0004-simulation-core.md) line 951 |
END

### FIX LA1-5
FILE: docs/adr/README.md
BEFORE:
| OQ-0005-AB14 | What does spawn protection cover, and what ends it early? | Mustafa (the 3 s is the Advisor's `[DEFAULT]`; what it covers and what ends it are new rules) | [ADR 0005](0005-content-abilities-assets.md) line 1349 |
| OQ-0005-AB15 | Who sees a player's upgrade picks? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1367 |
AFTER:
| OQ-0005-AB14 | What does spawn protection cover, and what ends it early? | Mustafa (the 3 s is the Advisor's `[DEFAULT]`; what it covers and what ends it are new rules); LATER: the design session after the netcode proof test | [ADR 0005](0005-content-abilities-assets.md) line 1349 |
| OQ-0005-AB15 | Who sees a player's upgrade picks? | Mustafa; LATER: the design session after the netcode proof test | [ADR 0005](0005-content-abilities-assets.md) line 1367 |
END

### FIX LA1-6
FILE: docs/adr/README.md
BEFORE:
| OQ-0006-13 | How is the production player build kept out of the preview deploy token's reach: a required reviewer on `preview`, production in a second Cloudflare account, or a token limited to the preview project? | the Advisor, with security-auditor; Mustafa for a second account and for the approvals (a) asks of him | [ADR 0006](0006-infrastructure-operations.md) line 896 |
| OQ-0006-14 | How do releases reach production while rooms run, and how is a bad server release rolled back? | the Advisor for the model; Mustafa for when players may be refused | [ADR 0006](0006-infrastructure-operations.md) line 915 |
AFTER:
| OQ-0006-13 | How is the production player build kept out of the preview deploy token's reach: a required reviewer on `preview`, production in a second Cloudflare account, or a token limited to the preview project? | the Advisor, with security-auditor; Mustafa for a second account and for the approvals (a) asks of him; LATER, keeping its own timing: before the foundation's client-host token, and until then the preview hosting check runs under ADR 0006 part 13's item B9 interim rule, every deploy waiting for Mustafa's approval (the advisor's reading, 2026-11-03) | [ADR 0006](0006-infrastructure-operations.md) line 896 |
| OQ-0006-14 | How do releases reach production while rooms run, and how is a bad server release rolled back? | the Advisor for the model; Mustafa for when players may be refused; LATER: the design session after the netcode proof test | [ADR 0006](0006-infrastructure-operations.md) line 915 |
END

### FIX LA1-7
FILE: docs/adr/README.md
BEFORE:
| OQ-0006-18 (UNRELEASED-ART) | Must unreleased art stay unseen, or is a preview behind a login enough? | Mustafa, with OQ-0005-AP1 (where model files live), before the first authored model is committed | [ADR 0006](0006-infrastructure-operations.md) line 975 |
AFTER:
| OQ-0006-18 (UNRELEASED-ART) | Must unreleased art stay unseen, or is a preview behind a login enough? | Mustafa, with OQ-0005-AP1 (where model files live), before the first authored model is committed; LATER, keeping its own timing: if the first model comes before the design session, it goes to Mustafa then (the advisor's reading, 2026-11-03) | [ADR 0006](0006-infrastructure-operations.md) line 975 |
END

### FIX LA1
FILE: docs/adr/README.md
BEFORE:
Every decision code and open question the drafts route to Mustafa or the Advisor, one row each. The first rows are the
ones the brief and the advisor's conditions name; the rest follow by draft. Open questions decided by a spike alone, or
by an agent, are in the drafts and in [SPIKE_PLAN](../SPIKE_PLAN.md), not here.
AFTER:
Every decision code and open question the drafts route to Mustafa or the Advisor, one row each. The first rows are the
ones the brief and the advisor's conditions name; the rest follow by draft. Open questions decided by a spike alone, or
by an agent, are in the drafts and in [SPIKE_PLAN](../SPIKE_PLAN.md), not here. A row answered at the stop
(2026-11-03) quotes the answer word for word, from Mustafa or from the Advisor as Mustafa's message marks them; a
reading the advisor's verdict adds is marked "(the advisor's reading, 2026-11-03)". Rows marked LATER go to the design
session Mustafa set at the stop ("a design session after the netcode proof test."), their deciders unchanged; three
keep their own timing, set by the evidence they need (the advisor's reading, 2026-11-03).
END

### FIX LA2
FILE: docs/adr/README.md
BEFORE:
| The design questions | The 38 research questions below that are pure design matters | Mustafa, or a later design session | "Design questions" below |
AFTER:
| The design questions | The 38 research questions below that are pure design matters | Mustafa, or a later design session; LATER: they join the agenda of the design session after the netcode proof test, their deciders unchanged, and Mustafa may trim the list (the advisor's reading, 2026-11-03) | "Design questions" below |
END

### FIX H19
FILE: docs/adr/README.md
BEFORE:
| CF-2 | R2-BUD-REC6's reason (r2:2402–2403) holds at N = 54, not at N = 48; restate for 30 Hz and say what the 60 Hz margin is measured against (PR #28 review, finding 2) | ADR 0001 parts 2 and 3: the budget restated at 30 Hz, the corrected reason with "(at N = 54, MODES-3 (b))" in part 3 (line 162 at 604a497), the 60 Hz margin as [GLOSSARY §8](../research/GLOSSARY.md#the-60-hz-margin) defines it; its reading is OQ-0001-1 |
AFTER:
| CF-2 | R2-BUD-REC6's reason (r2:2402–2403) holds at N = 54, not at N = 48; restate for 30 Hz and say what the 60 Hz margin is measured against (PR #28 review, finding 2) | ADR 0001 parts 2 and 3: the budget restated at 30 Hz, the corrected reason with "(at N = 54, MODES-3 (b))" in part 3 (line 162 at 604a497), the 60 Hz margin as [GLOSSARY §8](../research/GLOSSARY.md#the-60-hz-margin) defines it; its reading is OQ-0001-1, confirmed by Mustafa on 2026-11-03 with his addition (the goal budget holding at the rooms per VM measured at 60 Hz) |
END

### FIX H19-2
FILE: docs/adr/README.md
BEFORE:
| N2 | the tick rate: 30 Hz; 60 Hz only with a 60 Hz margin of at least 25 % | 0001 |
AFTER:
| N2 | the tick rate: 30 Hz; 60 Hz only with a 60 Hz margin of at least 25 %, every 60 Hz research mark and the goal budget holding at the rooms per VM measured at 60 Hz, as slice 6's run on the real sim shows (ADR 0001 part 2; the netcode proof test's 60 Hz run is an early signal only) | 0001 |
END

### NOTE T1
no change: README has no TOY-BOX row, and none of the nine documents has a toy-box line (grep -i "toy" finds only
ADR 0003's legacy "toy-soldier death fall", GA14, at :784, :893 and :1302: map row T3, another subject); the plan's
"README's row" does not exist (condition 1). Mustafa's answer (2026-11-03: "reword both to point at LOOK_V2, in a small kit change-set after PR #30
is updated, together with the checkpoint anti-waste rules.") touches CLAUDE.md line 4 and the art-director's file
line 10, outside this PR; STATUS and the report record that kit change-set (the TOY-BOX ruling: nothing changes in the
nine documents).
END

### FIX RISK3
FILE: docs/adr/README.md
BEFORE:
each, and their files stay readable in the branch's history (`.checkpoints/v2-decision-drafts/review-*.md`). Codes local
AFTER:
each, and their files stay readable in PR #33's commits (`.checkpoints/v2-decision-drafts/review-*.md`). Codes local
END

### FIX O11
FILE: docs/SPIKE_PLAN.md
BEFORE:
**Status:** Proposed, with the six decision drafts (`docs/adr/0001`–`0006`) · **Date:** 2026-11-01 · **Author:**
engine-architect (the synthesis, plan step 6), from test-engineer's spike entries and the drafts' "What a spike must
prove" parts · **Signed off by Mustafa:** no
AFTER:
**Status:** Proposed, with the six decision drafts (`docs/adr/0001`–`0006`) · **Date:** 2026-11-01 · **Author:**
engine-architect (the synthesis, plan step 6), from test-engineer's spike entries and the drafts' "What a spike must
prove" parts · **Signed off by Mustafa:** as the basis for the spikes (2026-11-03, SPIKE-ORDER (b)); not yet accepted
END

### FIX O11-2
FILE: docs/SPIKE_PLAN.md
BEFORE:
**Rules for every spike.** A spike is throwaway, lives in `spikes/`, and is exempt from the legacy hard constraints
(SPIKE-SCOPE (a)). None starts before Mustafa and the Advisor approve the drafts (the drafts brief, §4; CLAUDE.md
"Phase 0"). Every server number is at the 30 Hz baseline (TICK-30); the 60 Hz form runs only in the 60 Hz margin run
below. Where a research mark gives N = 48 and N = 54, N = 48 holds (MODES-3 (a)).
AFTER:
**Rules for every spike.** A spike is throwaway, lives in `spikes/`, and is exempt from the legacy hard constraints
(SPIKE-SCOPE (a)). Mustafa signed the six drafts off as the basis for the spikes on 2026-11-03 (SPIKE-ORDER (b)); each
spike is still its own change-set, plan first (CLAUDE.md, "The change-set protocol"), and wave 1 runs one heavy spike
at a time (below). Every server number is at the 30 Hz baseline (TICK-30); the 60 Hz form runs only in the 60 Hz
margin run below and in slice 6's final 60 Hz run. Where a research mark gives N = 48 and N = 54, N = 48 holds
(MODES-3 (a)).
END

### FIX M17
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Machines** (named in each draft's pass-mark key): RS, the reference server, and SH, its shared neighbour (ADR 0006,
  OQ-0006-1; [GLOSSARY: the reference server](research/GLOSSARY.md#the-reference-server)); LG, the load generator, and
  MV, a CX33 running the metrics stack (ADR 0006); ML, the minimum laptop, and RD, the recommended desktop (ADR 0003,
  OQ-0003-1: the models are not named yet); MAC, a macOS machine with the stable Safari on the day of the run, its
  version recorded (WebGPU from Safari 26.0: r3-rendering.md, R3-WGPU-F19, lines 171–174; WebTransport from 26.4:
  r2-netcode.md, R2-TRN-F7, lines 959–975); MP, Mustafa's PC in Brave, for what he judges only.
AFTER:
- **Machines** (named in each draft's pass-mark key): RS, the reference server, and SH, its shared neighbour (ADR 0006,
  OQ-0006-1; [GLOSSARY: the reference server](research/GLOSSARY.md#the-reference-server)); LG, the load generator, a
  CX33 running the simulated clients (Mustafa, 2026-11-03: "the CX33 runs the simulated clients"; how many comes from
  the calibration run: the advisor's reading, 2026-11-03), and MV, a CX33 running the metrics stack (ADR 0006); ML, the
  minimum laptop, and RD, the recommended desktop (ADR 0003, OQ-0003-1: the models are not named yet); MAC, a macOS
  machine with the stable Safari on the day of the run, its version recorded (WebGPU from Safari 26.0: r3-rendering.md,
  R3-WGPU-F19, lines 171–174; WebTransport from 26.4: r2-netcode.md, R2-TRN-F7, lines 959–975); MP, Mustafa's PC in
  Brave, for what he judges only ("Test PC: my own, in Brave", 2026-11-03; it stands in for none of ML, RD and MAC: the
  advisor's reading, 2026-11-03). The VMs sit in a separate Hetzner project; Mustafa creates and deletes them himself
  for each run, and they exist only during a run (OQ-0006-17, answered by Mustafa, 2026-11-03).
END

### FIX O1
FILE: docs/SPIKE_PLAN.md
BEFORE:
## The spikes, in the proposed order

The netcode proof test's place is settled: it runs before the foundation (SLICE-ORDER, condition 3). The rest of the
order below is a proposal for Mustafa and the Advisor (J6 (5) in `docs/adr/README.md`'s codes); `docs/adr/README.md` lists it
under "Decisions the drafts ask for".
AFTER:
## The spikes, in their order

The netcode proof test's place is settled: it runs before the foundation (SLICE-ORDER, condition 3). Mustafa accepted
the order below on 2026-11-03 (SPIKE-ORDER (b); J6 (5) in `docs/adr/README.md`'s codes): "(b) accept with one change.
The six drafts are the basis for the spikes, in SPIKE_PLAN's order, but wave 1 runs one heavy spike at a time: the
netcode proof test first, then the sim core spike (then the 60 Hz run), then the boundary spike, then the look proof
test once its models exist. The preview hosting check may run alongside any of them. Reason: seven usage-limit cuts in
three days."
END

### FIX O2
FILE: docs/SPIKE_PLAN.md
BEFORE:
  A["Drafts approved<br/>(Mustafa and the Advisor)"] --> W1
  subgraph W1["Wave 1: before the foundation, in parallel"]
    NPT["Netcode proof test (L)<br/>first: SLICE-ORDER"]
    BND["Rust-TypeScript boundary spike (L)"]
    SIM["Sim core spike (L)"]
    LOOK["Look proof test (L)<br/>Mustafa judges"]
    PREV["Preview hosting check (S)"]
  end
  SIM -->|"its sim, if OQ-0001-2 (a)"| M60["60 Hz margin run<br/>(inside the netcode proof test)"]
  NPT --> M60
  W1 --> F["The foundation<br/>(acceptance checks, not a spike)"]
  F --> W3
  subgraph W3["Wave 3: before slice 1"]
    CA["Content and ability spike (L)"]
    RBB["Render budget bench (L)<br/>may merge into the look proof test"]
  end
  W3 --> S["Slices 1 to 6 (SLICE-ORDER)"]
AFTER:
  A["Drafts signed off as the basis for the spikes<br/>(Mustafa, 2026-11-03)"] --> W1
  subgraph W1["Wave 1: before the foundation, one heavy spike at a time"]
    NPT["Netcode proof test (L)<br/>first: SLICE-ORDER"] --> SIM["Sim core spike (L)"]
    SIM -->|"its sim: OQ-0001-2 (a)"| M60["60 Hz margin run: an early signal<br/>(the proof test's kept harness)"]
    M60 --> BND["Rust-TypeScript boundary spike (L)"]
    BND --> LOOK["Look proof test (L)<br/>once its models exist; Mustafa judges"]
    PREV["Preview hosting check (S)<br/>alongside any of them"]
  end
  W1 --> F["The foundation<br/>(acceptance checks, not a spike)"]
  F --> W3
  subgraph W3["Wave 3: before slice 1"]
    CA["Content and ability spike (L)"]
    RBB["Render budget bench (L)<br/>may merge into the look proof test"]
  end
  W3 --> S["Slices 1 to 6 (SLICE-ORDER)<br/>slice 6: the final 60 Hz call, on the real sim"]
END

### FIX L3
FILE: docs/SPIKE_PLAN.md
BEFORE:
  APR["Asset-pipeline research<br/>(timing: ASSET-RESEARCH)"] --> AS["Asset spike (M)<br/>before the first authored model"]
AFTER:
  APX["Asset-pipeline research: export and slot naming<br/>(between two wave-1 spikes, before the models)"] --> LOOK
  APR["Asset-pipeline research: the rest"] --> AS["Asset spike (M)<br/>before the first authored model"]
END

### FIX O3
FILE: docs/SPIKE_PLAN.md
BEFORE:
In words: (0) the drafts are approved; (1) wave 1 runs five spikes in parallel, the netcode proof test first in
priority, with the 60 Hz margin run inside it once its sim load is settled (OQ-0001-2); (2) the foundation; (3) the
content and ability spike and the render budget bench, before slice 1; (4) slices 1–6, with the asset-pipeline research
and the asset spike before the first authored model, and the Feel Lab test once abilities are playable.
AFTER:
In words: (0) Mustafa signed the drafts off as the basis for the spikes (2026-11-03); (1) wave 1 runs one heavy spike
at a time, in his order: the netcode proof test, the sim core spike, the 60 Hz margin run (an early signal: OQ-0001-2
(a), answered by the Advisor, 2026-11-03), the boundary spike, then the look proof test once its models exist; the
preview hosting check may run alongside any of them. The 60 Hz margin run uses the proof test's kept server and
harness in its own VM session after the sim core spike, and the proof test closes at 30 Hz without it; the
asset-pipeline research's export and slot-naming part is agent research, not a heavy spike, so it runs alongside none
but between two of them, early enough that its export and slot rules reach Mustafa before he makes the models
(recommended: right after the netcode proof test); both are the advisor's reading, 2026-11-03. (2) the foundation;
(3) the content and ability spike and the render budget bench, before slice 1; (4) slices 1–6, with the rest of the
asset-pipeline research and the asset spike before the first authored model, the Feel Lab test once abilities are
playable, and slice 6's 60 Hz margin run on the real sim, which makes the final 60 Hz call (TICK-60-SETUP).
END

### FIX O4
FILE: docs/SPIKE_PLAN.md
BEFORE:
| 1 | The netcode proof test | L (+ M for ADR 0006's operations marks) | wave 1, first | PM-0001-1 to -62; PM-0004-67; PM-0006-11 to -27 |
AFTER:
| 1 | The netcode proof test | L (+ M for ADR 0006's operations marks) | wave 1, 1st | PM-0001-1 to -62; PM-0004-67; PM-0006-11 to -27 |
END

### FIX S17
FILE: docs/SPIKE_PLAN.md
BEFORE:
| 2 | The 60 Hz margin run | inside 1 | wave 1, after its sim load is settled | PM-0001-39, -40; PM-0004-71, -72 |
AFTER:
| 2 | The 60 Hz margin run (an early signal; slice 6 decides) | part of 1's L, on its kept server and harness | wave 1, after 4, in its own VM session | PM-0001-39, -40, -63; PM-0004-71, -72 |
END

### FIX O4-2
FILE: docs/SPIKE_PLAN.md
BEFORE:
| 3 | The Rust ↔ TypeScript boundary spike | L (+ M ADR 0004's share; + S ADR 0006's, optional) | wave 1 | PM-0002-1 to -34, -77, -78; PM-0004-8, -28, -61, -68 to -70; PM-0006-28 to -31 (optional) |
| 4 | The sim core spike | L, the largest | wave 1 | PM-0004-1 to -61, -73, -74 |
| 5 | The look proof test | L (+ S or M for ADR 0005's share) | wave 1 | PM-0003-1 to -17; PM-0005-33 to -37 |
| 6 | The preview hosting check | S (calendar: at least 2 days) | wave 1; ends before the foundation's preview step | PM-0006-1 to -10, -32 |
AFTER:
| 3 | The Rust ↔ TypeScript boundary spike | L (+ M ADR 0004's share; + S ADR 0006's, optional) | wave 1, 3rd: after 2 | PM-0002-1 to -34, -77, -78; PM-0004-8, -28, -61, -68 to -70; PM-0006-28 to -31 (optional) |
| 4 | The sim core spike | L, the largest | wave 1, 2nd: after 1 | PM-0004-1 to -61, -73, -74 |
| 5 | The look proof test | L (+ S or M for ADR 0005's share) | wave 1, 4th: after 3, once its models exist | PM-0003-1 to -17; PM-0005-33 to -37 |
| 6 | The preview hosting check | S (calendar: at least 2 days) | wave 1, alongside any of them; ends before the foundation's preview step | PM-0006-1 to -10, -32 |
END

### FIX L3-2
FILE: docs/SPIKE_PLAN.md
BEFORE:
| – | The asset-pipeline research (a research unit, not a spike) | one daytime session | before 9 | its own marks, appended to 9 |
AFTER:
| – | The asset-pipeline research (a research unit, not a spike) | one daytime session | its export and slot-naming part between two wave-1 spikes, before 5's models are made; the rest before 9 | its own marks, appended to 9 |
END

### FIX N14
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Who judges what:** the scripts judge every measured mark. Mustafa judges "look correct": the netcode checklist
  (ADR 0001, inside PM-0001-53), at the profile OQ-0001-3 confirms, beside the measured correction, interpolation and
  hit-registration marks. He also runs the operations checklist (ADR 0006, inside PM-0006-12 and PM-0006-18).
AFTER:
- **Who judges what:** the scripts judge every measured mark. Mustafa judges "look correct": the netcode checklist
  (ADR 0001, inside PM-0001-53), at P2 (OQ-0001-3, answered by the Advisor, 2026-11-03), beside the measured
  correction, interpolation and hit-registration marks. He also runs the operations checklist (ADR 0006, inside
  PM-0006-12 and PM-0006-18).
END

### FIX M18
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Depends on:** the drafts approved; OQ-0006-1 answered (the CCX23 and CX33 orderable in one EU location, in
  Mustafa's Hetzner account), with the location measured before RS is created (OQ-0006-2), the way sessions get VMs
  without holding a secret (OQ-0006-17), and LG and MV beside them; ML, RD and MAC named (OQ-0003-1); a throwaway sim
  with movement, hitscan and bots as protocol clients, native and Wasm, with stand-in big vehicles and live projectiles
  at the stress scene's counts (PM-0001-2); its 30 Hz marks may add ADR 0004's S1 stand-ins (384 effect rows with the
  effect step's 1 ms burned per tick, the 2.80 ms remainder topped up: ADR 0001, OQ-0001-2), and its density runs pad
  its sim step to the sim step share's ceiling, so rooms per VM is a lower bound and the goal budget provisional until
  slice 6 (ADR 0006 spike item 2; OQ-0006-19, read before those runs); the link simulator's L1 and L2 (ADR 0002's
  design), built here first, once the Advisor has read how each profile splits over the two directions (OQ-0001-16);
  a DM_Tri-sized greybox with enterable buildings and the target course; the port ranges (OQ-0006-16) and the
  development certificates for WebTransport (ADR 0006); for the WebSocket leg, OQ-0006-5's development and staging
  answer (the Advisor, from a short read, before the test starts), since a WebSocket cannot pin a certificate hash
  (ADR 0006, OQ-0006-5), and a domain first if OQ-0006-15 picks (b); the Advisor's readings of OQ-0001-3 before the
  checklist; Mustafa's session in Brave; the spike's run plan under its bill mark (PM-0006-18) and the cost Mustafa
  allows (OQ-0006-20) before the first VM.
AFTER:
- **Depends on:** the drafts approved; OQ-0006-1 answered (the CCX23 and CX33 orderable in one EU location, in
  Mustafa's Hetzner account; still open after the stop), with the location measured before RS is created (OQ-0006-2);
  the VMs as OQ-0006-17's answer sets them (Mustafa, 2026-11-03: "a separate Hetzner project. I create and delete the
  VMs myself for each run; sessions get SSH access only and no Hetzner token until the security baseline. VMs exist
  only during a run."), LG and MV beside RS and SH; the session's SSH key, made and held without the session reading
  a secret it did not make, as this test's own plan sets it with security-auditor and the advisor (the advisor's
  reading, 2026-11-03); ML, RD and MAC named (OQ-0003-1, still open); a throwaway sim
  with movement, hitscan and bots as protocol clients, native and Wasm, with stand-in big vehicles and live projectiles
  at the stress scene's counts (PM-0001-2); its 30 Hz marks may add ADR 0004's S1 stand-ins (384 effect rows with the
  effect step's 1 ms burned per tick, the 2.80 ms remainder topped up: ADR 0001, OQ-0001-2), and its density runs pad
  its sim step to the sim step share's ceiling, so rooms per VM is a lower bound and the goal budget provisional until
  slice 6 (ADR 0006 spike item 2; OQ-0006-19, read before those runs); the link simulator's L1 and L2 (ADR 0002's
  design), built here first, each profile's latency and jitter split half each way and its loss as P2 defines it
  (OQ-0001-16, answered by the Advisor, 2026-11-03), 2 % on each direction, applied independently (the advisor's
  reading, 2026-11-03); a DM_Tri-sized greybox with enterable buildings and the target course; the port ranges
  (OQ-0006-16) and the development certificate for WebTransport, private and hash-pinned (ADR 0006; Mustafa,
  2026-11-03); for the WebSocket leg, which cannot pin a certificate hash, a trusted certificate on "a domain I
  register" (Mustafa, 2026-11-03: OQ-0006-5's development case (i), the advisor's reading, and OQ-0006-15, both answered
  in part), the domain registered before the test starts and the certificate's issuance method chosen in this test's
  own plan with security-auditor (the advisor's reading, 2026-11-03); Mustafa's session in Brave; the spike's run plan
  under its bill mark (PM-0006-18) and the cost Mustafa allows (OQ-0006-20, still open; answered in part: "VMs exist
  only during a run.") before the first VM.
END

### FIX O5
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Place:** wave 1, first in priority: before the foundation (SLICE-ORDER condition 3). It builds first the client
  harness the boundary spike shares (the page with its Worker host and a plain renderer, on the throwaway sim: ADR
  0002's spike text), proposed with the order (J6 (5)).
AFTER:
- **Place:** wave 1, first (SPIKE-ORDER (b)): before the foundation (SLICE-ORDER condition 3). It builds first the
  client harness the boundary spike shares (the page with its Worker host and a plain renderer, on the throwaway sim:
  ADR 0002's spike text), accepted with the order (J6 (5)). It closes at 30 Hz: the 60 Hz margin run follows the sim
  core spike on this test's kept server and harness, in its own VM session, and reports separately (item 2; the
  advisor's reading, 2026-11-03).
END

### FIX RW14
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **It decides:** R2-TRN-OQ1, OQ2, OQ3, OQ5, OQ8; R2-BUD-OQ3, OQ7, OQ9, OQ10; R2-SYNC-OQ3 (through OQ-0001-6), OQ5 (if
  vehicles move), OQ12; R2-ROOM-OQ1; R2-XU-OQ2, OQ4, OQ5, OQ6, OQ7, OQ15; OQ-0001-6, OQ-0001-7; R5-TEST-OQ2 (the
  WebTransport crates' interop); R6-COST-OQ1; it feeds OQ-0006-2 and gives Mustafa and the Advisor the numbers for
  OQ-0001-5, and Mustafa the input-to-screen numbers for OQ-0001-15.
AFTER:
- **It decides:** R2-TRN-OQ1, OQ2, OQ3, OQ5, OQ8; R2-BUD-OQ3, OQ7, OQ9, OQ10; R2-SYNC-OQ3 (through OQ-0001-6), OQ5 (if
  vehicles move), OQ12; R2-ROOM-OQ1; R2-XU-OQ2, OQ4, OQ5, OQ6, OQ7, OQ15; OQ-0001-6 (Mustafa, 2026-11-03: "Aim for the
  smallest input buffer that holds."; of the options that pass its marks, the smallest buffer, run on arrival counting
  as none: the advisor's reading, 2026-11-03), OQ-0001-7; R5-TEST-OQ2 (the WebTransport crates' interop); R6-COST-OQ1;
  it feeds OQ-0006-2; it sets the rewind cap's value by OQ-0001-5's rule (Mustafa, 2026-11-03: "the cap covers the
  honest rewind the proof test measures at P2, rounded up; beyond the cap, the victim's view wins."), its p99 at P2
  rounded up to a whole number of ticks at the run's tick rate (the advisor's reading, 2026-11-03); and it gives Mustafa
  the input-to-screen numbers for OQ-0001-15.
END

### FIX O6
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Defined in:** [GLOSSARY §8, the 60 Hz margin](research/GLOSSARY.md#the-60-hz-margin). Owner: ADR 0001 part 2. A
  run inside the netcode proof test, not a separate spike.
AFTER:
- **Defined in:** [GLOSSARY §8, the 60 Hz margin](research/GLOSSARY.md#the-60-hz-margin). Owner: ADR 0001 part 2. A
  run on the netcode proof test's kept server and harness, not a separate spike, in its own VM session after the sim
  core spike (SPIKE-ORDER (b); the advisor's reading, 2026-11-03).
END

### FIX H20
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **What it proves:** whether the server may tick at 60 Hz. As ADR 0001 states it: in one run, on one vCPU of the
  reference server, in the stress scene at N = 48 with a snapshot-receiving client on every soldier, over R2-BUD-REC6's
  30-minute soak with re-flood ticks, a 60 Hz margin of at least 25 % (a p99 server tick time of at most 6.25 ms against
  8.33 ms) and every 60 Hz mark the research sets (R2-BUD-REC6's two shares, R1-TICK-REC2 P1–P3). Otherwise the tick
  stays at 30 Hz. The margin is measured in one run, never summed from separate runs (p99s do not add:
  [GLOSSARY, time statistics](research/GLOSSARY.md#time-statistics)).
- **Pass marks:** PM-0001-39, PM-0001-40; ADR 0004's PM-0004-71 (the sim step share at 60 Hz, read from the same run)
  and PM-0004-72 (the time mechanism and movement checks at 60 Hz).
- **Who judges what:** the scripts. Before the run, Mustafa and the Advisor confirm TICK-60-HEADROOM's reading
  (OQ-0001-1) and the Advisor picks how the stress scene's sim load reaches the run (TICK-60-SETUP, OQ-0001-2).
AFTER:
- **What it proves:** an early signal on whether the server may tick at 60 Hz (OQ-0001-2, answered by the Advisor,
  2026-11-03: "(a), as an early signal only. The final 60 Hz call is measured again with the real sim at slice 6.").
  The rule, as ADR 0001 part 2 states it: in one run, on one vCPU of the reference server, in the stress scene at
  N = 48 with a snapshot-receiving client on every soldier, over R2-BUD-REC6's 30-minute soak with re-flood ticks, a
  60 Hz margin of at least 25 % (a p99 server tick time of at most 6.25 ms against 8.33 ms), every 60 Hz mark the
  research sets (R2-BUD-REC6's two shares, R1-TICK-REC2 P1–P3), and the goal budget holding at the rooms per VM
  measured at 60 Hz (Mustafa's addition, below). Otherwise the tick stays at 30 Hz. The margin is measured in one run,
  never summed from separate runs (p99s do not add:
  [GLOSSARY, time statistics](research/GLOSSARY.md#time-statistics)). The goal budget is ADR 0006 part 8's as it
  stands when the call is made (at slice 6, the figure OQ-0006-19 fixes); it holds when the fleet the goal modes need
  at the rooms per VM measured at 60 Hz (part 8's formula, the same machine, N = 48) costs no more than that figure. In
  this early run the cost is reported beside part 8's provisional figures and decides nothing (the advisor's reading,
  2026-11-03).
- **Pass marks:** PM-0001-39, PM-0001-40, PM-0001-63 (rooms per VM at 60 Hz, with the goal-budget check); ADR 0004's
  PM-0004-71 (the sim step share at 60 Hz, read from the same run) and PM-0004-72 (the time mechanism and movement
  checks at 60 Hz). Each runs in this early run and again in slice 6's run, where they decide (the advisor's reading,
  2026-11-03).
- **Who judges what:** the scripts. Mustafa confirmed TICK-60-HEADROOM's reading (OQ-0001-1) on 2026-11-03: "confirm,
  with one addition: 60 Hz also needs the goal budget to hold at the rooms per VM measured at 60 Hz." The Advisor
  picked how the stress scene's sim load reaches the run the same day (TICK-60-SETUP, OQ-0001-2 (a)).
END

### FIX S18
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Depends on:** OQ-0001-1 and OQ-0001-2 answered. Under OQ-0001-2's recommended option (a), the sim core spike's sim
  runs inside the proof-test server, so the run waits until that sim runs the stress scene; the parts of the sim step
  share it does not build are stood in as ADR 0004's S1 stands them in, each named in the run's record: the ability
  and effect step (it arrives only with the content and ability spike, wave 3) as 384 rows in the effect pool with its
  1 ms ceiling burned per tick, and economy, objectives, moments and events (slice 4) as the remainder topped up per
  tick to 0.30 ms at 60 Hz (4.17 − 3.87, arithmetic on ADR 0004 part 6), so a pass still holds once the real parts
  meet their ceilings. Under (d) the run waits instead until the content and ability spike's effect step has joined
  that sim; under (c), the fallback, the check waits until the real sim runs the stress scene (slice 6) and the tick
  stays at 30 Hz until then.
AFTER:
- **Depends on:** OQ-0001-1 and OQ-0001-2, both answered on 2026-11-03. Under OQ-0001-2's option (a), the Advisor's
  answer, the sim core spike's sim runs inside the proof-test server, so the run waits until that sim runs the stress
  scene; the parts of the sim step share it does not build are stood in as ADR 0004's S1 stands them in, each named in
  the run's record: the ability and effect step (it arrives only with the content and ability spike, wave 3) as 384
  rows in the effect pool with its 1 ms ceiling burned per tick, and economy, objectives, moments and events (slice 4)
  as the remainder topped up per tick to 0.30 ms at 60 Hz (4.17 − 3.87, arithmetic on ADR 0004 part 6), so a pass
  still holds once the real parts meet their ceilings. Options (c) and (d) stay in ADR 0001 as weighed: under (d) the
  run would have waited until the content and ability spike's effect step joined that sim, and under (c), the
  fallback, until the real sim runs the stress scene at slice 6, where the final 60 Hz call now runs whatever this
  run shows (the advisor's reading, 2026-11-03).
END

### FIX O6-2
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Place:** wave 1, inside the netcode proof test, after the sim core spike's sim runs the stress scene (option (a));
  under (d), after the content and ability spike (wave 3).
AFTER:
- **Place:** wave 1, after the sim core spike and before the boundary spike (SPIKE-ORDER (b): "then the sim core spike
  (then the 60 Hz run)"), on the netcode proof test's kept server and harness, in its own VM session. The final 60 Hz
  call: slice 6's run on the real sim ("Later milestones" below).
END

### FIX S18-2
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Size:** part of the netcode proof test's L (the soaks repeated at 60 Hz).
- **Research spikes it subsumes:** R1-TICK-REC2 P1 at 60 Hz (ADR 0004's share), P2 at 60 Hz, P3 at 60 Hz; R2-BUD-REC6's
  60 Hz shares. **It decides:** carry-forward 2 (the 60 Hz margin); R1-TICK-OQ1 only if the margin passes (then ADR
  0004 and ADR 0002 decide).
AFTER:
- **Size:** part of the netcode proof test's L (the soaks repeated at 60 Hz, in their own VM session).
- **Research spikes it subsumes:** R1-TICK-REC2 P1 at 60 Hz (ADR 0004's share), P2 at 60 Hz, P3 at 60 Hz; R2-BUD-REC6's
  60 Hz shares. **It decides:** nothing on its own: it is the early signal on carry-forward 2 (the 60 Hz margin), and
  slice 6's run on the real sim decides the tick rate. A pass starts no 60 Hz work before slice 6 and a fail closes
  nothing; R1-TICK-OQ1 (60 or 64 Hz) is asked only after slice 6's run passes, and then ADR 0004 and ADR 0002 decide
  it (the advisor's reading, 2026-11-03).
END

### FIX O7
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Depends on:** the drafts approved; ML named and on hand (OQ-0003-1), MAC, and displays at 120 and 144 Hz; the
  netcode proof test's throwaway sim, client harness and room process (it runs beside it); the sim core spike's sim
  for PM-0002-28 and PM-0004-8, -28 (a stand-in until then); Mustafa's session in Brave.
- **Place:** wave 1, beside the netcode proof test; its result fixes the skeleton the foundation builds. It builds
  first, on the shared client harness, the CPU side of ADR 0003 part 8's measurement harness (frame time, the
  renderer's counters, the run sets and their labels), which the look proof test and the render budget bench then use
  (proposed with the order, J6 (5); the graphics review's finding 13).
AFTER:
- **Depends on:** the drafts approved; ML named and on hand (OQ-0003-1), MAC, and displays at 120 and 144 Hz; the
  netcode proof test's throwaway sim, client harness and room process (it runs after it); the sim core spike's sim
  for PM-0002-28 and PM-0004-8, -28 (it runs after that spike, so the sim is there: the advisor's reading,
  2026-11-03); Mustafa's session in Brave.
- **Place:** wave 1, third of the heavy spikes, after the sim core spike and the 60 Hz margin run (SPIKE-ORDER (b));
  its result fixes the skeleton the foundation builds. It builds first, on the shared client harness, the CPU side of
  ADR 0003 part 8's measurement harness (frame time, the renderer's counters, the run sets and their labels), which
  the look proof test and the render budget bench then use (accepted with the order, J6 (5); the graphics review's
  finding 13).
END

### FIX O8
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Place:** wave 1, in parallel with the netcode proof test; its sim (OQ-0001-2 (a)) feeds the 60 Hz margin run.
AFTER:
- **Place:** wave 1, second of the heavy spikes, after the netcode proof test (SPIKE-ORDER (b)); its sim (OQ-0001-2
  (a), answered by the Advisor, 2026-11-03) feeds the 60 Hz margin run that follows it.
END

### NOTE S19
no change: no block of its own. Its one place, SPIKE_PLAN:228 (the sim core spike's Place line), is rewritten by FIX
O8, which carries this row's change: "its sim (OQ-0001-2 (a), answered by the Advisor, 2026-11-03) feeds the 60 Hz
margin run that follows it".
END

### FIX O9
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Place:** wave 1, in parallel with the netcode proof test (they share nothing but the boundary spike's measurement
  harness); before any art production and before slice 1's renderer.
AFTER:
- **Place:** wave 1, last of the heavy spikes: after the boundary spike, whose measurement harness it uses, once its
  models exist (SPIKE-ORDER (b): "then the look proof test once its models exist"); before any art production and
  before slice 1's renderer.
END

### FIX L4-2
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Size:** L, without making the models, which is sized once OQ-SPIKE-1 is answered; ADR 0005's share S, or M if the
  kit builders are ported for the families' comparison.
AFTER:
- **Size:** L, without making the models, which is Mustafa's own work in Blender (OQ-SPIKE-1, answered), not a
  session's; ADR 0005's share S, or M if the kit builders are ported for the families' comparison.
END

### FIX L4
FILE: docs/SPIKE_PLAN.md
BEFORE:
  unit of at least 20 parts, a few props; who makes these models and by which route is OQ-SPIKE-1 (below), and whether
  a committed model may be seen by anyone is OQ-0006-18 (with OQ-0005-AP1), both answered before the first model is
  committed; legacy screenshots at the same framing; ADR 0003 part 8's measurement harness, whose CPU side the boundary
  spike builds first and to which this test adds the GPU timestamp queries (its timed marks wait for it, its look work
  does not); a synthetic recording for the scene; the content crate's whole-set check (a minimal loader, since the
  content and ability spike comes later); for OQ-0005-AP2, the ported kit builders beside the authored models (ADR
  0005's share; ADR 0003 part 11).
AFTER:
  unit of at least 20 parts, a few props; Mustafa makes them himself with AI help in Blender, as blockouts with a
  skin, named colour slots and LOOK_V2's proportions (OQ-SPIKE-1, answered by Mustafa, 2026-11-03, below); each object
  of the list may be a blockout at his bar, and the asset-pipeline research's export and slot-naming part reaches him
  with its export and slot rules before he makes them (the advisor's reading, 2026-11-03); whether a committed model
  may be seen by anyone is OQ-0006-18 (with OQ-0005-AP1), answered before the first model is committed, and if that
  comes before the design session after the netcode proof test, it goes to Mustafa then (the advisor's reading,
  2026-11-03); legacy screenshots at the same framing; ADR 0003 part 8's measurement harness, whose CPU side the
  boundary spike builds first and to which this test adds the GPU timestamp queries (its timed marks wait for it, its
  look work does not); a synthetic recording for the scene; the content crate's whole-set check (a minimal loader,
  since the content and ability spike comes later); for OQ-0005-AP2, the ported kit builders beside the authored
  models (ADR 0005's share; ADR 0003 part 11).
END

### FIX C11
FILE: docs/SPIKE_PLAN.md
BEFORE:
  server-side checks of overrides; the legacy tuning imported with every value accounted for; presentation that cannot
  reach an outcome; the legacy weapons and the three T1 abilities from data with no ability-specific Rust code; a new
  combination by data alone; VISION_V2 §6.4's two primitives; tech gates; Mustafa's upgrade tracks on test tracks
  (UPGRADE-SCOPE as checks); the ability step's cost at 384 active effects within its part of the sim step share, with
  no heap allocation, replaying byte for byte on native and Wasm (ADR 0004).
AFTER:
  server-side checks of overrides; the legacy tuning imported with every value accounted for; presentation that cannot
  reach an outcome, and a client entry for every simulation id of a presented kind, without which the content check
  fails (OQ-0005-1, answered by the Advisor, 2026-11-03; the presented kinds as ADR 0005's CD-2 names them: the
  advisor's reading, 2026-11-03); the legacy weapons and the three T1 abilities from data with no ability-specific
  Rust code; a new combination by data alone; VISION_V2 §6.4's two primitives; tech gates; Mustafa's upgrade tracks on
  test tracks (UPGRADE-SCOPE as checks); the ability step's cost at 384 active effects within its part of the sim step
  share, with no heap allocation, replaying byte for byte on native and Wasm (ADR 0004).
END

### FIX O10
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Place:** wave 1, in parallel with the netcode proof test; it must end before the foundation's preview step (a
  failure of PREV-DELETE with no workaround brings back ADR 0006's alternative, Workers static assets).
AFTER:
- **Place:** wave 1, alongside any of the heavy spikes (SPIKE-ORDER (b): "The preview hosting check may run alongside
  any of them."); it must end before the foundation's preview step (a failure of PREV-DELETE with no workaround brings
  back ADR 0006's alternative, Workers static assets).
END

### FIX B6
FILE: docs/SPIKE_PLAN.md
BEFORE:
  OQ4, OQ5, OQ9; R1-RR-OQ2, OQ3 (the loader's half); it gives Mustafa what he needs for OQ-0005-CD1 and the Advisor
  OQ-0005-AB13's trigger.
AFTER:
  OQ4, OQ5, OQ9; R1-RR-OQ2, OQ3 (the loader's half); it gives Mustafa what he needs for OQ-0005-CD1, and the evidence
  for OQ-0005-AB13's rule (answered by the Advisor, 2026-11-03: "scripting comes back only when a designed ability
  can't be built from data."; a new reviewed generic operation counts as built from data, so scripting comes back when
  one designed ability needs ability-specific code no operation can express: the advisor's reading, 2026-11-03).
END

### FIX F5
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Pass marks:** PM-0003-18 to PM-0003-42 (PM-0003-40 only if occlusion rays are ever proposed), PM-0003-44 to
  PM-0003-47 (PM-0003-44 reported until OQ-0003-17 sets its share), PM-0003-1 checked again; PM-0003-43 at each later
  upgrade.
AFTER:
- **Pass marks:** PM-0003-18 to PM-0003-42 (PM-0003-40 only if occlusion rays are ever proposed), PM-0003-44 to
  PM-0003-47 (PM-0003-44 gated: at most 6 of 600 presented frames may miss a refresh with the very large units on
  screen, on RD at High, OQ-0003-17, answered by the Advisor and confirmed by Mustafa, 2026-11-03; reported on ML: the
  advisor's reading, 2026-11-03), PM-0003-1 checked again; PM-0003-43 at each later upgrade.
END

### FIX Q5
FILE: docs/SPIKE_PLAN.md
BEFORE:
  (ADR 0003 places REC1 and REC4 here); R3-AUDIO-REC1 to REC4; R3-WGPU-REC2 (later). **It decides:** R3-CHAR-OQ1, OQ2,
  OQ4 (OQ6 only if the draw-call cap is still missed); R3-BUDGET-OQ5, and R3-BUDGET-OQ6 with OQ-0003-4; R3-AUDIO-OQ1,
  OQ2 (OQ6 only if the built-in panners miss PM-0003-38); it gives Mustafa the numbers for OQ-0003-10, and the Advisor
  the dropped-frame counts for OQ-0003-17 and the cold and soaked sets for OQ-0003-20.
AFTER:
  (ADR 0003 places REC1 and REC4 here); R3-AUDIO-REC1 to REC4; R3-WGPU-REC2 (later). **It decides:** R3-CHAR-OQ1, OQ2,
  OQ4 (OQ6 only if the draw-call cap is still missed); R3-BUDGET-OQ5, and R3-BUDGET-OQ6 with OQ-0003-4; R3-AUDIO-OQ1,
  OQ2 (OQ6 only if the built-in panners miss PM-0003-38); it gives Mustafa the numbers for OQ-0003-10. PM-0003-18 to
  -21 gate on the minimum laptop's soaked set, after 15 minutes of play (OQ-0003-20, answered by the Advisor,
  2026-11-03: "judge after 15 minutes of play."), and PM-0003-44's share is OQ-0003-17's answer (Pass marks above).
END

### FIX L2
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Place:** before the asset spike. **Timing is a decision for Mustafa and the Advisor** (ASSET-RESEARCH in
  `docs/adr/README.md`): they may move it earlier, for example beside wave 1, if art production should start sooner;
  under OQ-SPIKE-1's option (a), its slice on export checks and slot renaming (OQ-0005-AS3, AS4) runs before the look
  proof test.
AFTER:
- **Place:** its export and slot-naming part (OQ-0005-AS3, AS4) runs before the look proof test (Mustafa, 2026-11-03,
  LOOK-TEST-INPUTS: "yes, the asset research's export and slot-naming part runs before the look proof test. Route:
  Blender (Spline's skins and named slots are unproven)."); Blender is the route for the look test's models and for
  this part's export, and LOOK_V2 is not edited. The part is agent research, not a heavy spike, so it runs alongside
  none: it sits between two wave-1 spikes, early enough that its export and slot rules reach Mustafa before he makes
  the models (recommended: right after the netcode proof test); both are the advisor's reading, 2026-11-03. The rest of
  the research runs before the asset spike.
END

### FIX L2-2
FILE: docs/SPIKE_PLAN.md
BEFORE:
### The asset-pipeline research (a research unit, before the asset spike)
AFTER:
### The asset-pipeline research (a research unit, not a spike)
END

### FIX S17-2
FILE: docs/SPIKE_PLAN.md
BEFORE:
rejections (R2-ROOM-REC7's mark, ADR 0001's, at slices 2 and 3).
AFTER:
rejections (R2-ROOM-REC7's mark, ADR 0001's, at slices 2 and 3); and slice 6's 60 Hz margin run on the real sim, which
makes the final 60 Hz call whatever the wave-1 early signal showed (TICK-60-SETUP, answered by the Advisor,
2026-11-03), with PM-0001-39, -40, -63 and PM-0004-71, -72 deciding there (the advisor's reading, 2026-11-03).
END

### FIX RW15
FILE: docs/SPIKE_PLAN.md
BEFORE:
  session once their subject is playable: the rewind cap's feel (OQ-0001-5), the projectile prestep (OQ-0001-8), sub-tick
  shot stamps if the netcode proof test says they are needed (OQ-0001-7).
AFTER:
  session once their subject is playable: the projectile prestep (OQ-0001-8), sub-tick shot stamps if the netcode
  proof test says they are needed (OQ-0001-7). The rewind cap is not weighed here: OQ-0001-5's rule (Mustafa,
  2026-11-03) sets it from the proof test's measured honest rewind, and his Brave check of the measured cap stays the
  trigger to revisit its feel (the advisor's reading, 2026-11-03).
END

### FIX O12
FILE: docs/SPIKE_PLAN.md
BEFORE:
The order after the netcode proof test (J6 (5), with the harness builders it proposes), TICK-60-HEADROOM (OQ-0001-1),
TICK-60-SETUP (OQ-0001-2: the stand-ins, or wait), the "100 ms" reading (OQ-0001-3), the profiles' split over the two
directions (OQ-0001-16), the reference machines (OQ-0003-1), the server location (OQ-0006-1, OQ-0006-2), the spikes'
VMs and their cost (OQ-0006-17, OQ-0006-20), the provisional goal budget (OQ-0006-19), ASSET-RESEARCH's timing and
OQ-SPIKE-1 below are listed for Mustafa and the Advisor in `docs/adr/README.md`, "Decisions the drafts ask for".
AFTER:
The reference machines (OQ-0003-1: ML, RD and MAC still to be named), the server location (OQ-0006-1, OQ-0006-2), the
spikes' cost (OQ-0006-20, answered in part), the domain itself (OQ-0006-15, answered in part) and the provisional goal
budget (OQ-0006-19) are listed for Mustafa and the Advisor in `docs/adr/README.md`, "Decisions the drafts ask for".
The stop of 2026-11-03 answered the order after the netcode proof test, with the harness builders it names (J6 (5),
SPIKE-ORDER (b)), TICK-60-HEADROOM (OQ-0001-1), TICK-60-SETUP (OQ-0001-2), the "100 ms" reading (OQ-0001-3), the
profiles' split over the two directions (OQ-0001-16), the spikes' VMs (OQ-0006-17), ASSET-RESEARCH's timing and
OQ-SPIKE-1 (below); the register records each answer.
END

### NOTE H21
no change: no block of its own. Its place, SPIKE_PLAN:429-433 ("What this plan does not decide", one paragraph
shared with O12, H21, S20, N15, M20 and L5), is rewritten by FIX O12, which carries this row's change: "TICK-60-HEADROOM (OQ-0001-1)" leaves the undecided list and is named among the answers.
END

### NOTE S20
no change: no block of its own. Its place, SPIKE_PLAN:429-433 ("What this plan does not decide", one paragraph
shared with O12, H21, S20, N15, M20 and L5), is rewritten by FIX O12, which carries this row's change: "TICK-60-SETUP (OQ-0001-2: the stand-ins, or wait)" leaves the undecided list and is named among the answers.
END

### NOTE N15
no change: no block of its own. Its place, SPIKE_PLAN:429-433 ("What this plan does not decide", one paragraph
shared with O12, H21, S20, N15, M20 and L5), is rewritten by FIX O12, which carries this row's change: the "100 ms" reading (OQ-0001-3) and the profiles' split (OQ-0001-16) leave the undecided list and are named among the answers.
END

### NOTE M20
no change: no block of its own. Its place, SPIKE_PLAN:429-433 ("What this plan does not decide", one paragraph
shared with O12, H21, S20, N15, M20 and L5), is rewritten by FIX O12, which carries this row's change: OQ-0006-17 leaves the undecided list (answered); the reference machines (OQ-0003-1) stay, and OQ-0006-20 and OQ-0006-15 stay marked "answered in part".
END

### NOTE L5
no change: no block of its own. Its place, SPIKE_PLAN:429-433 ("What this plan does not decide", one paragraph
shared with O12, H21, S20, N15, M20 and L5), is rewritten by FIX O12, which carries this row's change: "ASSET-RESEARCH's timing and OQ-SPIKE-1" leave the undecided list and are named among the answers.
END

### FIX L1
FILE: docs/SPIKE_PLAN.md
BEFORE:
  answered. *Research ids:* none (gap G1).
AFTER:
  answered. *Research ids:* none (gap G1). Answered by Mustafa (2026-11-03): "yes, the asset research's export and
  slot-naming part runs before the look proof test. Route: Blender (Spline's skins and named slots are unproven). I
  make the test models with AI help in Blender; blockouts with a skin, named colour slots and LOOK_V2's proportions
  are enough." Making the models is his own work, not a session's, so no spike's size carries it. LOOK_V2 is not
  edited: Blender is the route for the look test's models and for the research part's export. Part 11's list of
  objects stays, each object may be a blockout at his bar, and which look-checklist steps a blockout can judge is for
  the look test's own plan (graphics-engineer, test-engineer), any visual call being Mustafa's (the advisor's reading,
  2026-11-03).
END

### FIX O15
FILE: docs/ENGINE_OVERVIEW.md
BEFORE:
**Status:** Proposed, with the six decision drafts · **Date:** 2026-11-01 · **Author:** engine-architect (the
synthesis) · **Signed off by Mustafa:** no. This page joins what [ADR 0001–0006](adr/README.md) propose and decides
nothing of its own; every detail links to its draft. The spikes are in [SPIKE_PLAN](SPIKE_PLAN.md).
AFTER:
**Status:** Proposed, with the six decision drafts · **Date:** 2026-11-01 · **Author:** engine-architect (the
synthesis) · **Signed off by Mustafa:** as the basis for the spikes (2026-11-03, SPIKE-ORDER (b)); not yet accepted.
This page joins what [ADR 0001–0006](adr/README.md) propose and decides nothing of its own; every detail links to its
draft. The spikes are in [SPIKE_PLAN](SPIKE_PLAN.md).
END

### FIX H22
FILE: docs/ENGINE_OVERVIEW.md
BEFORE:
same way on the server, in the Worker and in the editor. The server ticks at 30 Hz (TICK-30) for up to N = 48 players
(the largest mode, 24v24: MODES-3 (a)); 60 Hz only with [the 60 Hz margin](research/GLOSSARY.md#the-60-hz-margin).
AFTER:
same way on the server, in the Worker and in the editor. The server ticks at 30 Hz (TICK-30) for up to N = 48 players
(the largest mode, 24v24: MODES-3 (a)); 60 Hz only with [the 60 Hz margin](research/GLOSSARY.md#the-60-hz-margin),
every 60 Hz research mark and the goal budget holding at the rooms per VM measured at 60 Hz, as slice 6's run on the
real sim shows; the netcode proof test's 60 Hz run is an early signal only (ADR 0001 part 2).
END

### FIX F6
FILE: docs/ENGINE_OVERVIEW.md
BEFORE:
| 9 | A spectacle budget: a few very large units without dropping frames | ADR 0003 parts 3, 6, 7 (no first-sight stutter, very large units, budgets; part 7's presented-frame count reads "without dropping frames", PM-0003-44, and how many such frames a run may hold is OQ-0003-17); ADR 0002 part 5 (the sim off the page's thread); ADR 0004 (its share: the very large units are in the stress scene its sim is measured in, GLOSSARY §3) |
AFTER:
| 9 | A spectacle budget: a few very large units without dropping frames | ADR 0003 parts 3, 6, 7 (no first-sight stutter, very large units, budgets; part 7's presented-frame count reads "without dropping frames", PM-0003-44: at most 6 of 600 presented frames may miss a refresh with the very large units on screen, on the recommended desktop at High, OQ-0003-17, answered 2026-11-03); ADR 0002 part 5 (the sim off the page's thread); ADR 0004 (its share: the very large units are in the stress scene its sim is measured in, GLOSSARY §3) |
END

### FIX S22
FILE: docs/ENGINE_OVERVIEW.md
BEFORE:
| [0001](adr/0001-multiplayer-netcode.md) | authority, the tick rule, budgets, prediction, snapshots, lag compensation, interest, transport, rooms, upgrades' network side | the netcode proof test; the 60 Hz margin run |
AFTER:
| [0001](adr/0001-multiplayer-netcode.md) | authority, the tick rule, budgets, prediction, snapshots, lag compensation, interest, transport, rooms, upgrades' network side | the netcode proof test; the 60 Hz margin run (an early signal; slice 6's run on the real sim makes the call) |
END

### NOTE M19
no change: SPIKE_PLAN:190 (boundary spike, "ML named and on hand"), :223 (sim core spike, "ML and RD named") and :260
(look test, "ML and RD named and on hand") stay as written: Mustafa's "Test PC: my own, in Brave" is MP, what he
judges and never a measured result, and stands in for none of ML, RD and MAC, so OQ-0003-1 is unchanged in substance
(the advisor's ruling, SPIKE-MACHINES (d), escalated as REF-MACHINES). FIX O7 rewrites :190's paragraph and keeps
"ML named and on hand (OQ-0003-1)" word for word.
END

