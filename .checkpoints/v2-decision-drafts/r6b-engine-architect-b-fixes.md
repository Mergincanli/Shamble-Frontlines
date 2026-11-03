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
| Reference machines (OQ-0003-1) | Which laptop (the minimum) and which desktop (the recommended) are bought or borrowed as the two reference machines? Answered in part by Mustafa (2026-11-03): "minimum: a typical student laptop with integrated graphics; recommended: a desktop with a mid-range card (RTX 3060/4060 class); Mac: a MacBook Air, M1 or newer, for Safari. All borrowed." Until they are named, his laptop is the stand-in for ML, in his words: “Until then, my laptop's integrated graphics (Brave set to "Power saving") stands in for the minimum, for rough checks only.” His "Test PC: my own, in Brave" (at the stop, 2026-11-03) is MP. A rough check is reported, never judged against a pass mark. MP stays what he judges, never a measured result, and nothing stands in for RD or MAC (the advisor's reading, 2026-11-03), except a GitHub macOS runner for the netcode proof test's yes-or-no and count Safari rows, if the check Mustafa set first finds it can (his words of 2026-11-03 on when the machines are needed, and the runner's limits: [SPIKE_PLAN](../SPIKE_PLAN.md), "Machines"). A row on ML, RD or MAC is judged only on a named machine, and naming them no longer gates the netcode proof test's start (the advisor's reading, 2026-11-03). Still open: the actual machines, named by Mustafa before the look proof test. | Mustafa and the Advisor (money and availability) | [ADR 0003](0003-rendering.md) OQ-0003-1 |
END

### FIX IDX-8
FILE: docs/adr/README.md
BEFORE:
| The `wss` certificate on development and staging (OQ-0006-5) | How are the per-hostname production certificates issued, how long do they live, and which certificate does the WebSocket fallback present on development and staging servers, since a WebSocket cannot pin a certificate hash? The netcode proof test's WebSocket leg needs the development and staging answer first, and a trusted certificate there needs a domain (OQ-0006-15, answered by Mustafa, 2026-11-03). Answered in part by Mustafa (2026-11-03): "Certificates: a private hash-pinned certificate for WebTransport; for the WebSocket leg, a domain I register." The WebSocket leg uses the development case (i), a trusted certificate on his domain (the advisor's reading, 2026-11-03); still open: how that certificate is issued on a spike VM (http-01 and tls-alpn-01 need an open port, dns-01 a secret; OQ-0006-16's port plan applies), decided in the netcode proof test's own plan with security-auditor; the staging case, decided by the Advisor; and the production certificates, decided by security-auditor in the security baseline. | security-auditor in the security baseline, with the port plan (OQ-0006-16); the development and staging case before the netcode proof test starts (the Advisor, from a short read) | [ADR 0006](0006-infrastructure-operations.md) OQ-0006-5; SPIKE_PLAN item 1 |
AFTER:
| The `wss` certificate on development and staging (OQ-0006-5) | How are the per-hostname production certificates issued, how long do they live, and which certificate does the WebSocket fallback present on development and staging servers, since a WebSocket cannot pin a certificate hash? The netcode proof test's WebSocket leg needs the development and staging answer first, and a trusted certificate there needs a domain (OQ-0006-15, answered by Mustafa, 2026-11-03; registered, Mustafa, 2026-11-03). Answered in part by Mustafa (2026-11-03): "Certificates: a private hash-pinned certificate for WebTransport; for the WebSocket leg, a domain I register." The WebSocket leg uses the development case (i), a trusted certificate on his domain (the advisor's reading, 2026-11-03); still open: how that certificate is issued on a spike VM (http-01 and tls-alpn-01 need an open port, dns-01 a secret; OQ-0006-16's port plan applies), decided in the netcode proof test's own plan with security-auditor; the staging case, decided by the Advisor; and the production certificates, decided by security-auditor in the security baseline. | security-auditor in the security baseline, with the port plan (OQ-0006-16); the development and staging case before the netcode proof test starts (the Advisor, from a short read) | [ADR 0006](0006-infrastructure-operations.md) OQ-0006-5; SPIKE_PLAN item 1 |
END

### NOTE IDX-N1
FILE: docs/adr/README.md
no change: README:82 (OQ-0002-5) and README:138 (OQ-0006-7) already give the macOS runners' price (free while the
repository is public) and ask about pull requests and paid runners, not the spike; OQ-0002-5 stays open (AMENDMENT 2,
MACHINES-TIMING). README:141 (OQ-0006-10) says "billing address", not a server address. README:147 (OQ-0006-17) quotes
his stop answer, which still holds; his run-day words ("They'll be created fresh on run day ...") go in SPIKE_PLAN's
machines paragraph, which the register's rows reach through OQ-0006-1 and -2. README:385 (N34, the reference server,
R6-HOST-REC2) stays: the reference server stands (OQ-0006-1, option (a)). README:26 to 31 (the rest of the total's
paragraph, the pass marks 341) need no change: no pass-mark row is added or removed.
END

### FIX SPK-1
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Machines** (named in each draft's pass-mark key): RS, the reference server, and SH, its shared neighbour (ADR 0006,
  OQ-0006-1; [GLOSSARY: the reference server](research/GLOSSARY.md#the-reference-server)); LG, the load generator, a
  CX33 running the simulated clients (Mustafa, 2026-11-03: "the CX33 runs the simulated clients"; how many comes from
  the calibration run: the advisor's reading, 2026-11-03), and MV, a CX33 running the metrics stack (ADR 0006); ML, the
  minimum laptop, RD, the recommended desktop, and MAC, the Mac for Safari, with the stable Safari on the day of the
  run, its version recorded (WebGPU from Safari 26.0: r3-rendering.md, R3-WGPU-F19, lines 171–174; WebTransport from
  26.4: r2-netcode.md, R2-TRN-F7, lines 959–975). Which machines they are is OQ-0003-1 (ADR 0003), answered in part by
  Mustafa (2026-11-03): "minimum: a typical student laptop with integrated graphics; recommended: a desktop with a
  mid-range card (RTX 3060/4060 class); Mac: a MacBook Air, M1 or newer, for Safari. All borrowed."; still open: the
  actual machines, named by Mustafa before the look proof test. Until they are named, his laptop is the stand-in for ML,
  in his words: “Until then, my laptop's integrated graphics (Brave set to "Power saving") stands in for the minimum,
  for rough checks only.” MP, Mustafa's PC in Brave, is for what he judges only ("Test PC: my own, in Brave",
  2026-11-03). A rough check is reported, never judged against a pass mark. MP stays what he judges, never a measured
  result, and nothing stands in for RD or MAC (the advisor's reading, 2026-11-03). The VMs sit in a separate Hetzner
  project; Mustafa creates and deletes them himself for each run, and they exist only during a run (OQ-0006-17, answered
  by Mustafa, 2026-11-03).
AFTER:
- **Machines** (named in each draft's pass-mark key): RS, the reference server, the CCX23 (x86-64 AMD), on which every
  server row runs, and LG, the load generator, a CAX21 (Arm64) running the simulated clients, "so their build targets
  aarch64" (Mustafa's Hetzner check, 2026-11-03), both created in Falkenstein (`fsn1`) on the run day (ADR 0006,
  OQ-0006-1 and OQ-0006-2, answered by Mustafa, 2026-11-03: "both test servers will be in Falkenstein, not Nuremberg: a
  CCX23 for the game server and a CAX21 for the 48 simulated players"; [GLOSSARY: the reference
  server](research/GLOSSARY.md#the-reference-server)); a run whose bots one CAX21 cannot carry under 80 % adds as many
  CAX21s as its 5-minute calibration shows are needed, inside the cost cap Mustafa OKs for that run (LG-COUNT, the
  option he chose, 2026-11-03); MV, the metrics role: Prometheus on LG's CAX21, never on RS, inside LG's 80 % rule,
  standing in for the staging game server, and a run with more than one CAX21 says in its plan which one carries it (ADR
  0006; the advisor's reading, 2026-11-03); ML, the minimum laptop, RD, the recommended desktop, and MAC, the Mac for
  Safari, with the stable Safari on the day of the run, its version recorded (WebGPU from Safari 26.0: r3-rendering.md,
  R3-WGPU-F19, lines 171–174; WebTransport from 26.4: r2-netcode.md, R2-TRN-F7, lines 959–975). Which machines they are
  is OQ-0003-1 (ADR 0003), answered in part by Mustafa (2026-11-03): "minimum: a typical student laptop with integrated
  graphics; recommended: a desktop with a mid-range card (RTX 3060/4060 class); Mac: a MacBook Air, M1 or newer, for
  Safari. All borrowed."; still open: the actual machines, named by Mustafa before the look proof test. When each
  machine is needed, in his words (Mustafa, 2026-11-03): "For the netcode proof test's Safari leg, check whether
  GitHub's macOS runners (real Safari, free while the repository is public) have a Safari new enough for WebTransport
  and can reach the test servers. If they can, that is the Safari check, and I borrow the Mac only for the look proof
  test. If they can't, I borrow the Mac before the netcode proof test. The laptop and desktop stay on my timing: rough
  checks first, then a short re-run once they're named." Under his first branch a GitHub macOS runner stands as MAC for
  the netcode proof test's Safari rows only, once the run's results record (i) the runner image's Safari version on the
  day, at least the 26.4 that R2-TRN-F7 gives for WebTransport, (ii) real Safari driven through safaridriver, not a
  WebKit build, and (iii) from the runner, one WebTransport session to RS that opens and carries datagrams and one
  WebSocket session on the domain; it stands in only for yes-or-no and count rows (PM-0001-28's four-browser path and
  library pick, PM-0001-14, PM-0001-25's Origin, PM-0006-15), and a row whose limit is a time, or which ADR 0003's look
  proof test needs, stays on the borrowed Mac; the workflow that runs it is part of the proof test's plan, reviewed by
  security-auditor and infra-engineer, and any `.github/` change in this repository goes to Mustafa; the later spikes'
  Safari rows are each spike's own plan's, in the same way (the advisor's reading, 2026-11-03). Until they are named,
  his laptop is the stand-in for ML, in his words: “Until then, my laptop's integrated graphics (Brave set to "Power
  saving") stands in for the minimum, for rough checks only.” MP, Mustafa's PC in Brave, is for what he judges only
  ("Test PC: my own, in Brave", 2026-11-03). A rough check is reported, never judged against a pass mark. MP stays what
  he judges, never a measured result, and nothing else stands in for RD or MAC (the advisor's reading, 2026-11-03). The
  VMs sit in a separate Hetzner project; Mustafa creates and deletes them himself for each run, and they exist only
  during a run (OQ-0006-17, answered by Mustafa, 2026-11-03), in his words: "I created both once to try the console and
  deleted them right away. They'll be created fresh on run day, so their IP addresses will be new then." No draft writes
  an address: each run records the day's addresses with its results, and makes the settings bound to them, and its check
  of the new SSH host keys, on that day (ADR 0006's key; the advisor's reading, 2026-11-03).
END

### FIX SPK-2
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **What it proves:** ADR 0001 parts 1 and 10 (the online path end to end on Chrome, Edge, Firefox and Safari, over
  WebTransport with the WebSocket fallback); parts 2, 3, 6, 11 and 14 (48 simulated clients in the stress scene within
  ADR 0001's budgets at the 30 Hz baseline: server tick time and its shares, bandwidth per client both ways, memory per
  room, rooms per VM, under loss, bursts and hostile input); parts 4, 7 and 8 (one human client at P2 sees prediction,
  reconciliation and lag-compensated hits that look correct). Also ADR 0004 part 5 (S17: the server loop keeps a 30 Hz
  tick from elapsed time) and ADR 0006 parts 5, 7, 8 and 9 (rooms per VM on both machine types, the firewall, egress per
  match-hour, the spike's own bill, logging cost, metrics, the development certificates for WebTransport, and the
  WebSocket leg's `wss` sessions on the certificate OQ-0006-5 names, PM-0006-15).
AFTER:
- **What it proves:** ADR 0001 parts 1 and 10 (the online path end to end on Chrome, Edge, Firefox and Safari, over
  WebTransport with the WebSocket fallback); parts 2, 3, 6, 11 and 14 (48 simulated clients in the stress scene within
  ADR 0001's budgets at the 30 Hz baseline: server tick time and its shares, bandwidth per client both ways, memory per
  room, rooms per VM, under loss, bursts and hostile input); parts 4, 7 and 8 (one human client at P2 sees prediction,
  reconciliation and lag-compensated hits that look correct). Also ADR 0004 part 5 (S17: the server loop keeps a 30 Hz
  tick from elapsed time) and ADR 0006 parts 5, 7, 8 and 9 (rooms per VM on the reference server, the firewall, egress
  per match-hour, the spike's own bill, logging cost, metrics, the development certificates for WebTransport, and the
  WebSocket leg's `wss` sessions on the certificate OQ-0006-5 names, PM-0006-15).
END

### FIX SPK-3
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Depends on:** the drafts approved; OQ-0006-1 answered (the CCX23 and CX33 orderable in one EU location, in
  Mustafa's Hetzner account; still open after the stop), with the location measured before RS is created (OQ-0006-2);
  the VMs as OQ-0006-17's answer sets them (Mustafa, 2026-11-03: "a separate Hetzner project. I create and delete the
  VMs myself for each run; sessions get SSH access only and no Hetzner token until the security baseline. VMs exist
  only during a run."), LG and MV beside RS and SH; the session's SSH key, made and held without the session reading
  a secret it did not make, as this test's own plan sets it with security-auditor and the advisor (the advisor's
  reading, 2026-11-03); the machines its rows name (OQ-0003-1, answered in part: Mustafa names them before the look
  proof test; until then the stand-in for ML, for rough checks only); this spike's own plan lists its rows that run
  before they are named, and none of them is judged on the stand-in (the advisor's reading, 2026-11-03); a throwaway sim
  with movement, hitscan and bots as protocol clients, native and Wasm, with stand-in big vehicles and live projectiles
  at the stress scene's counts (PM-0001-2); its 30 Hz marks may add ADR 0004's S1 stand-ins (384 effect rows with the
  effect step's 1 ms burned per tick, the 2.80 ms remainder topped up: ADR 0001, OQ-0001-2), and its density runs pad
  its sim step to the sim step share's ceiling, so rooms per VM is a lower bound and the goal budget provisional until
  slice 6 (ADR 0006 spike item 2; OQ-0006-19, read before those runs); the link simulator's L1 and L2 (ADR 0002's
  design), built here first, each profile's latency and jitter split half each way and its loss as P2 defines it
  (OQ-0001-16, answered by the Advisor, 2026-11-03), 2 % on each direction, applied independently (the advisor's
  reading, 2026-11-03); a DM_Tri-sized greybox with enterable buildings and the target course; the port ranges
  (OQ-0006-16) and the development certificate for WebTransport, private and hash-pinned (ADR 0006; Mustafa,
  2026-11-03); for the WebSocket leg, which cannot pin a certificate hash, a trusted certificate (OQ-0006-5, answered in
  part: its development case (i) by the advisor's reading) on "a domain I register" (Mustafa, 2026-11-03),
  shamblefrontlines.com (OQ-0006-15, answered by Mustafa, 2026-11-03: "shamblefrontlines.com, which I register at
  Cloudflare."), the domain registered before the test starts and the certificate's issuance method chosen in this
  test's own plan with security-auditor (the advisor's reading, 2026-11-03); Mustafa's session in Brave; the spike's run
  plan under its bill mark (PM-0006-18), with each run's cost cap, from its planned hours, written in that run's plan
  for Mustafa's OK (OQ-0006-20, answered by Mustafa, 2026-11-03: "one cap per run, from its planned hours, written in
  that run's plan for my OK. I create and delete the VMs myself.").
AFTER:
- **Depends on:** the drafts approved; OQ-0006-1 and OQ-0006-2, answered by Mustafa (2026-11-03): RS and LG, the CCX23
  and the CAX21, created in `fsn1` on the run day (Machines, above); the VMs as OQ-0006-17's answer sets them (Mustafa,
  2026-11-03: "a separate Hetzner project. I create and delete the VMs myself for each run; sessions get SSH access only
  and no Hetzner token until the security baseline. VMs exist only during a run."), LG, with MV's role on it, beside RS;
  the session's SSH key, made and held without the session reading a secret it did not make, as this test's own plan
  sets it with security-auditor and the advisor (the advisor's reading, 2026-11-03); the machines its rows name
  (OQ-0003-1, answered in part: Mustafa names them before the look proof test), the laptop and desktop on his timing:
  "rough checks first, then a short re-run once they're named" (Mustafa, 2026-11-03), the rough checks on the stand-in
  for ML; this spike's own plan lists its rows that run before they are named, and nothing is judged on a rough check
  (the advisor's reading, 2026-11-03); its Safari leg starts with the check Mustafa set on GitHub's macOS runners, and
  the Mac is borrowed before this test only if they cannot pass it (Machines, above); a throwaway sim with movement,
  hitscan and bots as protocol clients, native and Wasm, with stand-in big vehicles and live projectiles at the stress
  scene's counts (PM-0001-2); its 30 Hz marks may add ADR 0004's S1 stand-ins (384 effect rows with the effect step's 1
  ms burned per tick, the 2.80 ms remainder topped up: ADR 0001, OQ-0001-2), and its density runs pad its sim step to
  the sim step share's ceiling, so rooms per VM is a lower bound and the goal budget provisional until slice 6 (ADR 0006
  spike item 2; OQ-0006-19, read before those runs); the link simulator's L1 and L2 (ADR 0002's design), built here
  first, each profile's latency and jitter split half each way and its loss as P2 defines it (OQ-0001-16, answered by
  the Advisor, 2026-11-03), 2 % on each direction, applied independently (the advisor's reading, 2026-11-03); a
  DM_Tri-sized greybox with enterable buildings and the target course; the port ranges (OQ-0006-16) and the development
  certificate for WebTransport, private and hash-pinned (ADR 0006; Mustafa, 2026-11-03); for the WebSocket leg, which
  cannot pin a certificate hash, a trusted certificate (OQ-0006-5, answered in part: its development case (i) by the
  advisor's reading) on "a domain I register" (Mustafa, 2026-11-03), shamblefrontlines.com (OQ-0006-15, answered by
  Mustafa, 2026-11-03: "shamblefrontlines.com, which I register at Cloudflare."), the domain registered (Mustafa,
  2026-11-03: "shamblefrontlines.com is registered at Cloudflare."), its DNS record set on each run day to that day's
  address before the certificate is issued, and the certificate's issuance method chosen in this test's own plan with
  security-auditor (the advisor's reading, 2026-11-03); Mustafa's session in Brave; the spike's run plan under its bill
  mark (PM-0006-18), with each run's cost cap, from its planned hours, written in that run's plan for Mustafa's OK
  (OQ-0006-20, answered by Mustafa, 2026-11-03: "one cap per run, from its planned hours, written in that run's plan for
  my OK. I create and delete the VMs myself.").
END

### FIX SPK-4
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Size:** L, the client harness included. Two transports on four browsers, two simulator layers, a room server with
  interest, encoding and lag compensation at 48 clients, a load generator, repeated 30-minute soaks, 8 rooms on two
  machine types, planted defects, Mustafa's session; ADR 0006's operations marks add M; ADR 0004's share is S.
AFTER:
- **Size:** L, the client harness included. Two transports on four browsers, two simulator layers, a room server with
  interest, encoding and lag compensation at 48 clients, a load generator, repeated 30-minute soaks, 8 rooms on the
  reference server, planted defects, Mustafa's session; ADR 0006's operations marks add M; ADR 0004's share is S.
END

### FIX SPK-5
FILE: docs/SPIKE_PLAN.md
BEFORE:
  as none: the advisor's reading, 2026-11-03), OQ-0001-7; R5-TEST-OQ2 (the WebTransport crates' interop); R6-COST-OQ1;
  it feeds OQ-0006-2; it sets the rewind cap's value by OQ-0001-5's rule (Mustafa, 2026-11-03: "the cap covers the
AFTER:
  as none: the advisor's reading, 2026-11-03), OQ-0001-7; R5-TEST-OQ2 (the WebTransport crates' interop); R6-COST-OQ1;
  it reports the round trip to `fsn1` (PM-0006-12), which decides nothing now that OQ-0006-2 is answered; it sets the
  rewind cap's value by OQ-0001-5's rule (Mustafa, 2026-11-03: "the cap covers the
END

### FIX SPK-6
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Depends on:** the drafts approved; the machines its rows name (OQ-0003-1, answered in part: Mustafa names them
  before the look proof test; until then the stand-in for ML, for rough checks only); this spike's own plan lists its
  rows that run before they are named, and none of them is judged on the stand-in (the advisor's reading, 2026-11-03);
  displays at 120 and 144 Hz; the netcode proof test's throwaway sim, client harness and room process (it runs after
AFTER:
- **Depends on:** the drafts approved; the machines its rows name (OQ-0003-1, answered in part: Mustafa names them
  before the look proof test), the laptop and desktop on his timing: "rough checks first, then a short re-run once
  they're named" (Mustafa, 2026-11-03), the rough checks on the stand-in for ML; this spike's own plan lists its rows
  that run before they are named, and nothing is judged on a rough check (the advisor's reading, 2026-11-03); displays
  at 120 and 144 Hz; the netcode proof test's throwaway sim, client harness and room process (it runs after
END

### FIX SPK-7
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Depends on:** the drafts approved; RS orderable (OQ-0006-1); the machines its rows name (OQ-0003-1, answered in
  part: Mustafa names them before the look proof test; until then the stand-in for ML, for rough checks only); this
  spike's own plan lists its rows that run before they are named, and none of them is judged on the stand-in (the
  advisor's reading, 2026-11-03); the scenes built in the spike (the DM_Tri-sized greybox for the stress scene, the
AFTER:
- **Depends on:** the drafts approved; RS, the CCX23 in `fsn1` (OQ-0006-1 and OQ-0006-2, answered by Mustafa,
  2026-11-03); the machines its rows name (OQ-0003-1, answered in part: Mustafa names them before the look proof test),
  the laptop and desktop on his timing: "rough checks first, then a short re-run once they're named" (Mustafa,
  2026-11-03), the rough checks on the stand-in for ML; this spike's own plan lists its rows that run before they are
  named, and nothing is judged on a rough check (the advisor's reading, 2026-11-03); the scenes built in the spike (the
  DM_Tri-sized greybox for the stress scene, the
END

### FIX SPK-8
FILE: docs/SPIKE_PLAN.md
BEFORE:
The reference machines ML, RD and MAC (OQ-0003-1, answered in part: Mustafa names them before the look proof test), the
server location (OQ-0006-1, OQ-0006-2) and the provisional goal budget (OQ-0006-19) are listed for Mustafa and the
Advisor in `docs/adr/README.md`, "Decisions the drafts ask for". The stop of 2026-11-03 answered the order after the
netcode proof test, with the harness builders it names (J6 (5), SPIKE-ORDER (b)), TICK-60-HEADROOM (OQ-0001-1),
TICK-60-SETUP (OQ-0001-2), the "100 ms" reading (OQ-0001-3), the profiles' split over the two directions (OQ-0001-16),
the spikes' VMs (OQ-0006-17), ASSET-RESEARCH's timing and OQ-SPIKE-1 (below); Mustafa's answers to the escalations,
later that day, answered the spikes' cost (OQ-0006-20) and the domain (OQ-0006-15); the register records each answer.
AFTER:
The reference machines ML, RD and MAC (OQ-0003-1, answered in part: Mustafa names them before the look proof test), the
provisional goal budget (OQ-0006-19) and the staging server's plan (OQ-0006-22, before slice 3) are listed for Mustafa
and the Advisor in `docs/adr/README.md`, "Decisions the drafts ask for". The stop of 2026-11-03 answered the order after
the netcode proof test, with the harness builders it names (J6 (5), SPIKE-ORDER (b)), TICK-60-HEADROOM (OQ-0001-1),
TICK-60-SETUP (OQ-0001-2), the "100 ms" reading (OQ-0001-3), the profiles' split over the two directions (OQ-0001-16),
the spikes' VMs (OQ-0006-17), ASSET-RESEARCH's timing and OQ-SPIKE-1 (below); Mustafa's answers to the escalations,
later that day, answered the spikes' cost (OQ-0006-20) and the domain (OQ-0006-15), and his Hetzner check and his later
answers settled the servers and their location (OQ-0006-1, OQ-0006-2) and when each machine is needed (Machines, above);
the register records each answer.
END

### NOTE SPK-N1
FILE: docs/SPIKE_PLAN.md
no change: SPIKE_PLAN:274 (item 4's "real Safari on MAC", PM-0004-19), :321 (item 5, "ML, RD and MAC named and on
hand": the Mac is borrowed for the look proof test under both of his branches), :352 and :362 (the preview hosting
check's Safari and "ML and MAC with the four browsers") and :410 (the content and ability spike's "MAC (PM-0005-2)")
stay: the machines paragraph (SPK-1) says the later spikes' Safari rows are each spike's own plan's, in the same way
as the netcode proof test's (AMENDMENT 2). SPIKE_PLAN:30 to 45, 112 to 119, 129 to 154, 160 to 162, 175 to 176, 244 to
247, 281 to 284 and 512 to 518 are SPK-1 to SPK-8. No CX33, SH, `nbg1`, `hel1` or fixed address is left in
SPIKE_PLAN; "Nuremberg" appears only inside his quoted words (SPK-1). No pass-mark row and no open question is added
(OQ-SPIKE-1 stays answered).
END

### NOTE OVW-N1
FILE: docs/ENGINE_OVERVIEW.md
no change: no line of ENGINE_OVERVIEW becomes untrue. It names no machine, location, plan, price or address (grep for
CX33, CAX21, CCX23, SH, MV, Hetzner, Falkenstein, `fsn1`, Nuremberg, address, Safari, MAC: no hit); it gives no count
of open questions (its "Open, for Mustafa and the Advisor" section points to the register, which IDX-1 to IDX-8
update); its 0006 row ("game servers, ... costs, ...") and the diagram's server node stay true with the rig of two
machines.
END

