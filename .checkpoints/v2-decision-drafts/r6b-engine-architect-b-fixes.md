# r6b engine-architect-b fixes
### FIX IDX-1
FILE: docs/adr/README.md
BEFORE:
| [0006](0006-infrastructure-operations.md) | Infrastructure and operations | infra-engineer; pass marks: test-engineer | Proposed. Signed off by Mustafa: as the basis for the spikes (2026-11-03, SPIKE-ORDER (b)); not yet accepted | the preview hosting check; operations marks in the netcode proof test; an optional share of the boundary spike | 21 numbered (OQ-0006-1 to -21; -21, COMMUNITY-SERVERS, added 2026-11-03), 18 open: -5 answered in part; -17 answered at the stop (2026-11-03); -15 and -20 answered by Mustafa (2026-11-03) |
AFTER:
| [0006](0006-infrastructure-operations.md) | Infrastructure and operations | infra-engineer; pass marks: test-engineer | Proposed. Signed off by Mustafa: as the basis for the spikes (2026-11-03, SPIKE-ORDER (b)); not yet accepted | the preview hosting check; operations marks in the netcode proof test; an optional share of the boundary spike | 22 numbered (OQ-0006-1 to -22; -21, COMMUNITY-SERVERS, and -22, the staging server's x86 plan, added 2026-11-03), 17 open: -5 answered in part; -17 answered at the stop (2026-11-03); -15 and -20 answered by Mustafa (2026-11-03); -1 and -2 answered by Mustafa (2026-11-03), by his Hetzner check |
END

### FIX IDX-2
FILE: docs/adr/README.md
BEFORE:
Total: 115 numbered open questions, 97 of them open (OQ-0004-5 and -6 point to ADR 0005; OQ-0003-14 is answered; a
question answered in part stays open, marked "answered in part"; 13 were answered at the stop, 2026-11-03, OQ-0006-15
and -20 by Mustafa's answers to the escalations later that day, and OQ-0006-21 is new); SPIKE_PLAN adds one of its own,
AFTER:
Total: 116 numbered open questions, 96 of them open (OQ-0004-5 and -6 point to ADR 0005; OQ-0003-14 is answered; a
question answered in part stays open, marked "answered in part"; 13 were answered at the stop, 2026-11-03, OQ-0006-15
and -20 by Mustafa's answers to the escalations later that day, and OQ-0006-21 is new; OQ-0006-1 and -2 were answered
and OQ-0006-22 is new, from his Hetzner check later that day); SPIKE_PLAN adds one of its own,
END

### FIX IDX-3
FILE: docs/adr/README.md
BEFORE:
| OQ-0006-1 | Can Mustafa order Hetzner's CCX23 and CX33 in an EU location? Still open after the stop (2026-11-03): his console check. | Mustafa (a console check in his own account; agents create no account), then a research read if (b) | [ADR 0006](0006-infrastructure-operations.md) OQ-0006-1 |
AFTER:
| OQ-0006-1 | Can Mustafa order Hetzner's CCX23 and CX33 in an EU location? Answered by Mustafa's console check (2026-11-03): "in Falkenstein the cost-optimized x86 line (CX33) can't be ordered; only its Arm64 version can." and, later that day, "I created both once to try the console and deleted them right away." (the CCX23 and the CAX21). Option (a) holds: the reference server stands (the advisor's reading, 2026-11-03). The entry still carries R6-HOST-OQ4 (Hetzner's dedicated root servers, read before ADR 0006 part 8's goal budget is fixed) and R6-HOST-OQ7 (the IPv4 price, read at sign-up), each open with its own trigger. The VMs are created fresh on each run day, so capacity on the day is not guaranteed: ADR 0001's failure row for no Hetzner capacity in the EU and option (b)'s route stay the fallback (the advisor's reading, 2026-11-03). The staging server's plan is OQ-0006-22. | Mustafa (a console check in his own account; agents create no account), then a research read if (b) | [ADR 0006](0006-infrastructure-operations.md) OQ-0006-1 |
END

### FIX IDX-4
FILE: docs/adr/README.md
BEFORE:
| OQ-0006-2 | Which EU location: Nuremberg, Helsinki, or Falkenstein if offered? | a measurement made before the reference server is created (a small check at the start of the netcode proof test's setup), confirmed by Mustafa | [ADR 0006](0006-infrastructure-operations.md) OQ-0006-2 |
AFTER:
| OQ-0006-2 | Which EU location: Nuremberg, Helsinki, or Falkenstein if offered? Answered by Mustafa (2026-11-03): "both test servers will be in Falkenstein, not Nuremberg: a CCX23 for the game server and a CAX21 for the 48 simulated players". Both are created in `fsn1`; the options in ADR 0006 stay as the record of what was weighed. The first production fleet starts in the test servers' location unless playtesters' round trips (R6-HOST-OQ2, still open) say otherwise (the advisor's reading, 2026-11-03). | Mustafa (his words above). The round-trip measurement that was to pick the location before the reference server is created is now PM-0006-12's check, reported against VISION_V2 §4's "about 100 ms", which decides nothing (the advisor's reading, 2026-11-03) | [ADR 0006](0006-infrastructure-operations.md) OQ-0006-2 |
END

### FIX IDX-5
FILE: docs/adr/README.md
BEFORE:
| OQ-0006-21 (COMMUNITY-SERVERS) | Could the game one day let its community host servers, and what must the drafts keep possible for it? Mustafa (2026-11-03), a new question, "keep the door open, don't design it": "players may one day host their own servers, possibly with revenue such as official private servers they rent." and "The drafts must not lock it out: the room server stays one self-contained program with a config file (players, length, rules), checks the game's version and content hash, and marks each match as official or community, so progression can count on official servers only." No option is weighed and no timing is set. | Mustafa | [ADR 0006](0006-infrastructure-operations.md) OQ-0006-21 |
AFTER:
| OQ-0006-21 (COMMUNITY-SERVERS) | Could the game one day let its community host servers, and what must the drafts keep possible for it? Mustafa (2026-11-03), a new question, "keep the door open, don't design it": "players may one day host their own servers, possibly with revenue such as official private servers they rent." and "The drafts must not lock it out: the room server stays one self-contained program with a config file (players, length, rules), checks the game's version and content hash, and marks each match as official or community, so progression can count on official servers only." No option is weighed and no timing is set. | Mustafa | [ADR 0006](0006-infrastructure-operations.md) OQ-0006-21 |
| OQ-0006-22 | Which x86 plan runs the staging game server, now that the proposed CX33 cannot be ordered in Falkenstein (Mustafa's check, 2026-11-03)? Staging runs the room server, so an Arm plan would make it an Arm host (ADR 0004's PM-0004-26 first), and ADR 0006 part 8's lean case rests on the CX33's price. The options are in ADR 0006's entry. | Mustafa (money), with the Advisor; before slice 3 | [ADR 0006](0006-infrastructure-operations.md) OQ-0006-22 |
END

### FIX IDX-6
FILE: docs/adr/README.md
BEFORE:
| OQ-0006-15 (DOMAIN) | Which domain do the client host, the lobby and the game servers use, and when is it registered? Answered by Mustafa (2026-11-03): "shamblefrontlines.com, which I register at Cloudflare." At the stop he had written: "for the WebSocket leg, a domain I register." It is registered before the netcode proof test (the advisor's reading, 2026-11-03); production uses the same domain, since it is the game's name and ADR 0006 part 6 needs one registrable domain (the advisor's reading, 2026-11-03). The DNS host and the prices stay "not researched". | Mustafa (an account and a cost), after the Advisor's OQ-0006-5 read | [ADR 0006](0006-infrastructure-operations.md) OQ-0006-15 |
AFTER:
| OQ-0006-15 (DOMAIN) | Which domain do the client host, the lobby and the game servers use, and when is it registered? Answered by Mustafa (2026-11-03): "shamblefrontlines.com, which I register at Cloudflare." At the stop he had written: "for the WebSocket leg, a domain I register." It is registered, in his words (2026-11-03): "DOMAIN: done. shamblefrontlines.com is registered at Cloudflare." Production uses the same domain, since it is the game's name and ADR 0006 part 6 needs one registrable domain (the advisor's reading, 2026-11-03). The DNS host and the prices stay "not researched". | Mustafa (an account and a cost), after the Advisor's OQ-0006-5 read | [ADR 0006](0006-infrastructure-operations.md) OQ-0006-15 |
END

### FIX IDX-7
FILE: docs/adr/README.md
BEFORE:
| Reference machines (OQ-0003-1) | Which laptop (the minimum) and which desktop (the recommended) are bought or borrowed as the two reference machines? Answered in part by Mustafa (2026-11-03): "minimum: a typical student laptop with integrated graphics; recommended: a desktop with a mid-range card (RTX 3060/4060 class); Mac: a MacBook Air, M1 or newer, for Safari. All borrowed." Until they are named, his laptop is the stand-in for ML, in his words: “Until then, my laptop's integrated graphics (Brave set to "Power saving") stands in for the minimum, for rough checks only.” His "Test PC: my own, in Brave" (at the stop, 2026-11-03) is MP. A rough check is reported, never judged against a pass mark. MP stays what he judges, never a measured result, and nothing stands in for RD or MAC (the advisor's reading, 2026-11-03). A row on ML, RD or MAC is judged only on a named machine, and naming them no longer gates the netcode proof test's start (the advisor's reading, 2026-11-03). Still open: the actual machines, named by Mustafa before the look proof test. | Mustafa and the Advisor (money and availability) | [ADR 0003](0003-rendering.md) OQ-0003-1 |
AFTER:
| Reference machines (OQ-0003-1) | Which laptop (the minimum) and which desktop (the recommended) are bought or borrowed as the two reference machines? Answered in part by Mustafa (2026-11-03): "minimum: a typical student laptop with integrated graphics; recommended: a desktop with a mid-range card (RTX 3060/4060 class); Mac: a MacBook Air, M1 or newer, for Safari. All borrowed." Until they are named, his laptop is the stand-in for ML, in his words: “Until then, my laptop's integrated graphics (Brave set to "Power saving") stands in for the minimum, for rough checks only.” His "Test PC: my own, in Brave" (at the stop, 2026-11-03) is MP. A rough check is reported, never judged against a pass mark. MP stays what he judges, never a measured result, and nothing stands in for RD or MAC (the advisor's reading, 2026-11-03), except a GitHub macOS runner for the netcode proof test's yes-or-no Safari rows, if the check Mustafa set first finds it can (his words of 2026-11-03 on when the machines are needed, and the runner's limits: [SPIKE_PLAN](../SPIKE_PLAN.md), "Machines"). A row on ML, RD or MAC is judged only on a named machine, and naming them no longer gates the netcode proof test's start (the advisor's reading, 2026-11-03). Still open: the actual machines, named by Mustafa before the look proof test. | Mustafa and the Advisor (money and availability) | [ADR 0003](0003-rendering.md) OQ-0003-1 |
END

