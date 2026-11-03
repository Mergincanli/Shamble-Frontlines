# r5 engine-architect b: fix blocks (README, SPIKE_PLAN, ENGINE_OVERVIEW)

### FIX IDX-1
FILE: docs/adr/README.md
BEFORE:
| [0003](0003-rendering.md) | Rendering and presentation | graphics-engineer; pass marks: test-engineer | Proposed. Signed off by Mustafa: as the basis for the spikes (2026-11-03, SPIKE-ORDER (b)); not yet accepted | the look proof test; the render budget bench | 20 numbered (OQ-0003-1 to -20), 17 open: -14 answered by its decider, the verifier; -17 and -20 answered at the stop (2026-11-03) |
AFTER:
| [0003](0003-rendering.md) | Rendering and presentation | graphics-engineer; pass marks: test-engineer | Proposed. Signed off by Mustafa: as the basis for the spikes (2026-11-03, SPIKE-ORDER (b)); not yet accepted | the look proof test; the render budget bench | 20 numbered (OQ-0003-1 to -20), 17 open: -14 answered by its decider, the verifier; -17 and -20 answered at the stop (2026-11-03); -1 answered in part by Mustafa (2026-11-03) |
END

### FIX IDX-2
FILE: docs/adr/README.md
BEFORE:
| [0006](0006-infrastructure-operations.md) | Infrastructure and operations | infra-engineer; pass marks: test-engineer | Proposed. Signed off by Mustafa: as the basis for the spikes (2026-11-03, SPIKE-ORDER (b)); not yet accepted | the preview hosting check; operations marks in the netcode proof test; an optional share of the boundary spike | 20 numbered (OQ-0006-1 to -20), 19 open: -17 answered at the stop (2026-11-03); -5, -15 and -20 answered in part |
AFTER:
| [0006](0006-infrastructure-operations.md) | Infrastructure and operations | infra-engineer; pass marks: test-engineer | Proposed. Signed off by Mustafa: as the basis for the spikes (2026-11-03, SPIKE-ORDER (b)); not yet accepted | the preview hosting check; operations marks in the netcode proof test; an optional share of the boundary spike | 21 numbered (OQ-0006-1 to -21; -21, COMMUNITY-SERVERS, added 2026-11-03), 18 open: -17 answered at the stop (2026-11-03); -15 and -20 answered by Mustafa (2026-11-03); -5 answered in part |
END

### FIX IDX-3
FILE: docs/adr/README.md
BEFORE:
Total: 114 numbered open questions, 98 of them open (OQ-0004-5 and -6 point to ADR 0005; OQ-0003-14 is answered; 13
were answered at the stop, 2026-11-03; a question answered in part stays open, marked "answered in part"); SPIKE_PLAN
adds one of its own, OQ-SPIKE-1, answered by Mustafa (2026-11-03). Pass marks: 63, 78, 47, 75, 46 and 32 rows (341;
ADR 0001's PM-0001-63 joined its 60 Hz group at the stop). Status of every draft: Proposed. Signed off by Mustafa: as
the basis for the spikes (2026-11-03, SPIKE-ORDER (b)); not yet accepted. His words: "(b) accept with one change. The
AFTER:
Total: 115 numbered open questions, 97 of them open (OQ-0004-5 and -6 point to ADR 0005; OQ-0003-14 is answered; 13 were
answered at the stop, 2026-11-03, and OQ-0006-15 and -20 by Mustafa's answers to the escalations later that day;
OQ-0006-21 is new; a question answered in part stays open, marked "answered in part"); SPIKE_PLAN adds one of its own,
OQ-SPIKE-1, answered by Mustafa (2026-11-03). The open counts follow the drafts' own entries: this register records
Mustafa's design answers of 2026-11-03 for OQ-0005-AB1, AB3, AB11 and AB15, AB14 in part, his lean for AB5 and the
fragments as damage (rows in "Decisions the drafts ask for" below); ADR 0005 takes them up at its next revision, and so
do ADR 0001 part 12 (AB15's visibility class) and ADR 0003 (the fragments). Pass marks: 63, 78, 47, 75, 46 and 32 rows
(341; ADR 0001's PM-0001-63 joined its 60 Hz group at the stop). Status of every draft: Proposed. Signed off by Mustafa:
as the basis for the spikes (2026-11-03, SPIKE-ORDER (b)); not yet accepted. His words: "(b) accept with one change. The
END

### FIX IDX-4
FILE: docs/adr/README.md
BEFORE:
keep their own timing, set by the evidence they need (the advisor's reading, 2026-11-03).
AFTER:
keep their own timing, set by the evidence they need (the advisor's reading, 2026-11-03). Rows that Mustafa's design
answers and his answers to the escalations touch (both later on 2026-11-03) quote him the same way, each labelled no
stronger than his words. One design answer is a name, in his words: “Name: the in-match picks are "amplifications"
("amps").”; the drafts still say "picks". Two more stay open, in his words: "Still open: faction asymmetry (Mustafa
works on the unit list first) and the war around the matches."; faction asymmetry sits beside OQ-0005-CD5 below, and the
war around the matches maps to no id.
END

### FIX IDX-5
FILE: docs/adr/README.md
BEFORE:
| WEAPON-UNLOCKS (OQ-0005-AB5) | How does an account-unlocked weapon reach a match (LOOP_V2 §8, VISION_V2 §1), when buying offers no weapon choice and other weapons come only from drops? | Mustafa, with the Advisor (LOOP_V2 §8 is theirs) | [ADR 0005](0005-content-abilities-assets.md) OQ-0005-AB5 |
AFTER:
| WEAPON-UNLOCKS (OQ-0005-AB5) | How does an account-unlocked weapon reach a match (LOOP_V2 §8, VISION_V2 §1), when buying offers no weapon choice and other weapons come only from drops? A lean from Mustafa (2026-11-03), still open: "leaning towards class variants, with more variants added over time. Not final." | Mustafa, with the Advisor (LOOP_V2 §8 is theirs) | [ADR 0005](0005-content-abilities-assets.md) OQ-0005-AB5 |
END

### FIX IDX-6
FILE: docs/adr/README.md
BEFORE:
| UPGRADE-SCOPE: carry slots (OQ-0005-AB1) | How many weapons can a player carry besides the class weapon, and what happens to the one in hand on a pickup? | Mustafa; LATER: the design session after the netcode proof test | [ADR 0005](0005-content-abilities-assets.md) OQ-0005-AB1 |
AFTER:
| UPGRADE-SCOPE: carry slots (OQ-0005-AB1) | How many weapons can a player carry besides the class weapon, and what happens to the one in hand on a pickup? Answered by Mustafa (2026-11-03): "a looted weapon replaces the main (class) weapon; the sidearm stays as backup. Assumed, to confirm: the class's own abilities stay." | Mustafa | [ADR 0005](0005-content-abilities-assets.md) OQ-0005-AB1 |
END

### FIX IDX-7
FILE: docs/adr/README.md
BEFORE:
| UPGRADE-SCOPE: looted ammo (OQ-0005-AB2) | What does a looted gun come with, and can it be refilled? | Mustafa; LATER: the design session after the netcode proof test | [ADR 0005](0005-content-abilities-assets.md) OQ-0005-AB2 |
AFTER:
| UPGRADE-SCOPE: looted ammo (OQ-0005-AB2) | What does a looted gun come with, and can it be refilled? Mustafa (2026-11-03): "still open (infinite, limited, or supply lines by an engineer class)." The Advisor's lean, as his message marks it, and not an answer: "limited but generous, refilled at the barracks and supply points." | Mustafa; LATER: the design session after the netcode proof test | [ADR 0005](0005-content-abilities-assets.md) OQ-0005-AB2 |
END

### FIX IDX-8
FILE: docs/adr/README.md
BEFORE:
| UPGRADE-SCOPE: levelling (OQ-0005-AB3) | What earns a level, how fast, and does a track the player switches to catch up? | Mustafa; LATER: the design session after the netcode proof test | [ADR 0005](0005-content-abilities-assets.md) OQ-0005-AB3 |
AFTER:
| UPGRADE-SCOPE: levelling (OQ-0005-AB3) | What earns a level, how fast, and does a track the player switches to catch up? Answered by Mustafa (2026-11-03): "XP from kills, assists and objective work while using that unit or weapon; fast early levels; a track that's behind the player's best levels faster." | Mustafa | [ADR 0005](0005-content-abilities-assets.md) OQ-0005-AB3 |
END

### FIX IDX-9
FILE: docs/adr/README.md
BEFORE:
| Reference machines (OQ-0003-1) | Which laptop (the minimum) and which desktop (the recommended) are bought or borrowed as the two reference machines? Still open after the stop: Mustafa's "Test PC: my own, in Brave" (2026-11-03) is MP, what he judges and never a measured result, so it stands in for none of ML, RD and MAC, and the netcode proof test cannot start without ML and RD (the advisor's reading, 2026-11-03). | Mustafa and the Advisor (money and availability) | [ADR 0003](0003-rendering.md) OQ-0003-1 |
AFTER:
| Reference machines (OQ-0003-1) | Which laptop (the minimum) and which desktop (the recommended) are bought or borrowed as the two reference machines? Answered in part by Mustafa (2026-11-03): "minimum: a typical student laptop with integrated graphics; recommended: a desktop with a mid-range card (RTX 3060/4060 class); Mac: a MacBook Air, M1 or newer, for Safari. All borrowed." Until they are named, his laptop is the stand-in for ML, in his words: “Until then, my laptop's integrated graphics (Brave set to "Power saving") stands in for the minimum, for rough checks only.” His "Test PC: my own, in Brave" (at the stop, 2026-11-03) is MP. A rough check is reported, never judged against a pass mark. MP stays what he judges, never a measured result, and nothing stands in for RD or MAC (the advisor's reading, 2026-11-03). A row on ML, RD or MAC is judged only on a named machine, and naming them no longer gates the netcode proof test's start (the advisor's reading, 2026-11-03). Still open: the actual machines, named by Mustafa before the look proof test. | Mustafa and the Advisor (money and availability) | [ADR 0003](0003-rendering.md) OQ-0003-1 |
END

### FIX IDX-10
FILE: docs/adr/README.md
BEFORE:
| Fragments as damage | Do grenade fragments deal damage per fragment? LOOK_V2 §3 and §7 bring fragmentation back as a visible effect, which ADR 0003 draws from the blast event; any rule beyond that is a design question (no research id). | Mustafa with the game-designer | [ADR 0004](0004-simulation-core.md) "Open questions", closing paragraph; [ADR 0003](0003-rendering.md) part 5.k, OQ-0003-7 and the look checklist step 24 |
AFTER:
| Fragments as damage | Do grenade fragments deal damage per fragment? LOOK_V2 §3 and §7 bring fragmentation back as a visible effect, which ADR 0003 draws from the blast event; any rule beyond that is a design question (no research id). Mustafa (2026-11-03): "Fragments as damage: visual only for now. The server sends one blast event, the browser draws the fragments, and the spray matches the real damage radius." | Mustafa with the game-designer | [ADR 0004](0004-simulation-core.md) "Open questions", closing paragraph; [ADR 0003](0003-rendering.md) part 5.k, OQ-0003-7 and the look checklist step 24 |
END

### FIX IDX-11
FILE: docs/adr/README.md
BEFORE:
| OQ-0005-CD5 | Faction variants: full definitions that reference shared ones, or overrides of a base? | the faction design session (VISION_V2 §7, faction asymmetry; Mustafa) | [ADR 0005](0005-content-abilities-assets.md) OQ-0005-CD5 |
AFTER:
| OQ-0005-CD5 | Faction variants: full definitions that reference shared ones, or overrides of a base? Faction asymmetry stays open, in Mustafa's words (2026-11-03): "Still open: faction asymmetry (Mustafa works on the unit list first)". | the faction design session (VISION_V2 §7, faction asymmetry; Mustafa) | [ADR 0005](0005-content-abilities-assets.md) OQ-0005-CD5 |
END

### FIX IDX-12
FILE: docs/adr/README.md
BEFORE:
| OQ-0005-AB11 | Does the Riot Shield answer the Banzai blade, and is the Peacekeeper's side of the triangle strong enough before Tech 1? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) OQ-0005-AB11 |
AFTER:
| OQ-0005-AB11 | Does the Riot Shield answer the Banzai blade, and is the Peacekeeper's side of the triangle strong enough before Tech 1? Answered for the Shield and the blade by Mustafa (2026-11-03): "a tall, slightly curved Riot Shield blocks the Banzai blade from the front, so the Warrior must flank; the Conscript's Molotov punishes the shield hard. LOOP_V2's counter triangle; numbers tuned later." The Peacekeeper's strength before Tech 1 is left to his "numbers tuned later" (the advisor's reading, 2026-11-03). | Mustafa | [ADR 0005](0005-content-abilities-assets.md) OQ-0005-AB11 |
END

### FIX IDX-13
FILE: docs/adr/README.md
BEFORE:
| OQ-0005-AB14 | What does spawn protection cover, and what ends it early? | Mustafa (the 3 s is the Advisor's `[DEFAULT]`; what it covers and what ends it are new rules); LATER: the design session after the netcode proof test | [ADR 0005](0005-content-abilities-assets.md) OQ-0005-AB14 |
AFTER:
| OQ-0005-AB14 | What does spawn protection cover, and what ends it early? Answered in part by Mustafa (2026-11-03): "3 s, against spawn killing." The Advisor's pencil, which Mustafa may veto: "Firing ends it early". Still open: what it covers, ADR 0005's options (a) damage only or (b) damage and every effect another team applies. | Mustafa (what it covers is a new rule; he may veto the Advisor's pencil for what ends it); LATER: the design session after the netcode proof test | [ADR 0005](0005-content-abilities-assets.md) OQ-0005-AB14 |
END

### FIX IDX-14
FILE: docs/adr/README.md
BEFORE:
| OQ-0005-AB15 | Who sees a player's upgrade picks? | Mustafa; LATER: the design session after the netcode proof test | [ADR 0005](0005-content-abilities-assets.md) OQ-0005-AB15 |
AFTER:
| OQ-0005-AB15 | Who sees a player's upgrade picks? Answered by Mustafa (2026-11-03): "Upgrade visibility: enemies can see a player's build." Placing it at OQ-0005-AB15 is the advisor's reading (2026-11-03). How enemies see it is not said: the scoreboard or a cue on the soldier; a cue is ADR 0003's to draw, and the look is Mustafa's. | Mustafa | [ADR 0005](0005-content-abilities-assets.md) OQ-0005-AB15; [ADR 0001](0001-multiplayer-netcode.md) part 12 (the picks' visibility class) |
END

### FIX IDX-15
FILE: docs/adr/README.md
BEFORE:
| OQ-0006-15 (DOMAIN) | Which domain do the client host, the lobby and the game servers use, and when is it registered? Answered in part by Mustafa (2026-11-03): "for the WebSocket leg, a domain I register." It is registered before the netcode proof test (the advisor's reading, 2026-11-03); still open: which domain, which registrar and which DNS host, and whether production uses it later, decided by Mustafa. | Mustafa (an account and a cost), after the Advisor's OQ-0006-5 read | [ADR 0006](0006-infrastructure-operations.md) OQ-0006-15 |
AFTER:
| OQ-0006-15 (DOMAIN) | Which domain do the client host, the lobby and the game servers use, and when is it registered? Answered by Mustafa (2026-11-03): "shamblefrontlines.com, which I register at Cloudflare." At the stop he had written: "for the WebSocket leg, a domain I register." It is registered before the netcode proof test (the advisor's reading, 2026-11-03); production uses the same domain, since it is the game's name and ADR 0006 part 6 needs one registrable domain (the advisor's reading, 2026-11-03). The DNS host and the prices stay "not researched". | Mustafa (an account and a cost), after the Advisor's OQ-0006-5 read | [ADR 0006](0006-infrastructure-operations.md) OQ-0006-15 |
END

### FIX IDX-16
FILE: docs/adr/README.md
BEFORE:
| OQ-0006-20 | What may the netcode proof test's VMs cost? Answered in part by Mustafa (2026-11-03): "VMs exist only during a run."; still open: the cost cap, decided by Mustafa (money). | Mustafa (money) | [ADR 0006](0006-infrastructure-operations.md) OQ-0006-20 |
AFTER:
| OQ-0006-20 | What may the netcode proof test's VMs cost? Answered by Mustafa (2026-11-03): "one cap per run, from its planned hours, written in that run's plan for my OK. I create and delete the VMs myself." At the stop he had written: "VMs exist only during a run." | Mustafa (money) | [ADR 0006](0006-infrastructure-operations.md) OQ-0006-20 |
| OQ-0006-21 (COMMUNITY-SERVERS) | Could the game one day let its community host servers, and what must the drafts keep possible for it? Mustafa (2026-11-03), a new question, "keep the door open, don't design it": "players may one day host their own servers, possibly with revenue such as official private servers they rent." and "The drafts must not lock it out: the room server stays one self-contained program with a config file (players, length, rules), checks the game's version and content hash, and marks each match as official or community, so progression can count on official servers only." No option is weighed and no timing is set. | Mustafa | [ADR 0006](0006-infrastructure-operations.md) OQ-0006-21 |
END

### NOTE IDX-N1
no change: README:337 (I-21, "sent to their owner") and README:389 (J5, "carry slots, looted ammo and levelling open
for Mustafa") describe the drafts as they stand (ADR 0001 part 12's owner-only class) and what the decision map's part
j held on 2026-10-31; both stay true. Under DRAFT-ENTRIES (a) the drafts take the design answers up at their next
revision, which FIX IDX-3 says beside the counts, so the interface table changes with ADR 0001 part 12, not before it.
END

### NOTE IDX-N2
no change: COMMUNITY-SERVERS against the register (condition 6). I-7 (README:323, a room = mode id + allowed
overrides), I-10 (:326, the match record), I-11 (:327, room process, supervisor and lobby) and seam S4 (:344) say
nothing that makes a room need the supervisor or the lobby to start, or that rules out an official or community mark
on a match, so no sentence there reads as closing the door and none gets a pointer. The one README place is the new
row (FIX IDX-16); "Needs outside VISION_V2 §6" gets no entry, since OQ-0006-21 adds no feature.
END

### NOTE IDX-N3
no change: rows kept as they are. README:16 (ADR 0001, 12 open) and :20 (ADR 0005, 33 open) keep their counts (the
advisor's COUNTS ruling: ADR 0005 stays 33 under DRAFT-ENTRIES (a); REF-MACHINES answers no ADR 0001 question).
README:47 (ASSET-RESEARCH) and :48 (OQ-SPIKE-1) keep their text: the ruling puts Mustafa's LOOK_V2-§8 answer in ADR
0005's Settled inputs (engine-architect a) and SPIKE_PLAN's research section (FIX SPK-9), and row 48's "LOOK_V2 is
not edited: Blender is the route for the look test's models" agrees with "keep it as it is, with Blender for the test
models only". README:57 (OQ-0006-5) stays answered in part; its "a trusted certificate there needs a domain
(OQ-0006-15)" stays true now that the domain is named. README:124 (OQ-0006-1) stays open (his console check).
END

### FIX SPK-1
FILE: docs/SPIKE_PLAN.md
BEFORE:
  minimum laptop, and RD, the recommended desktop (ADR 0003, OQ-0003-1: the models are not named yet); MAC, a macOS
  machine with the stable Safari on the day of the run, its version recorded (WebGPU from Safari 26.0: r3-rendering.md,
  R3-WGPU-F19, lines 171–174; WebTransport from 26.4: r2-netcode.md, R2-TRN-F7, lines 959–975); MP, Mustafa's PC in
  Brave, for what he judges only ("Test PC: my own, in Brave", 2026-11-03; it stands in for none of ML, RD and MAC: the
  advisor's reading, 2026-11-03). The VMs sit in a separate Hetzner project; Mustafa creates and deletes them himself
  for each run, and they exist only during a run (OQ-0006-17, answered by Mustafa, 2026-11-03).
AFTER:
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
END

### FIX SPK-2
FILE: docs/SPIKE_PLAN.md
BEFORE:
  reading, 2026-11-03); ML, RD and MAC named (OQ-0003-1, still open); a throwaway sim
AFTER:
  reading, 2026-11-03); the machines its rows name (OQ-0003-1, answered in part: Mustafa names them before the look
  proof test; until then the stand-in for ML, for rough checks only); this spike's own plan lists its rows that run
  before they are named, and none of them is judged on the stand-in (the advisor's reading, 2026-11-03); a throwaway sim
END

### FIX SPK-3
FILE: docs/SPIKE_PLAN.md
BEFORE:
  2026-11-03); for the WebSocket leg, which cannot pin a certificate hash, a trusted certificate on "a domain I
  register" (Mustafa, 2026-11-03; OQ-0006-5's development case (i) by the advisor's reading, and OQ-0006-15, both
  answered in part), the domain registered before the test starts and the certificate's issuance method chosen in this
  test's own plan with security-auditor (the advisor's reading, 2026-11-03); Mustafa's session in Brave; the spike's
  run plan under its bill mark (PM-0006-18) and the cost Mustafa allows (OQ-0006-20, still open; answered in part: "VMs
  exist only during a run.") before the first VM.
AFTER:
  2026-11-03); for the WebSocket leg, which cannot pin a certificate hash, a trusted certificate on "a domain I
  register" (Mustafa, 2026-11-03; OQ-0006-5's development case (i) by the advisor's reading, answered in part),
  shamblefrontlines.com (OQ-0006-15, answered by Mustafa, 2026-11-03: "shamblefrontlines.com, which I register at
  Cloudflare."), the domain registered before the test starts and the certificate's issuance method chosen in this
  test's own plan with security-auditor (the advisor's reading, 2026-11-03); Mustafa's session in Brave; the spike's run
  plan under its bill mark (PM-0006-18), with each run's cost cap, from its planned hours, written in that run's plan
  for Mustafa's OK (OQ-0006-20, answered by Mustafa, 2026-11-03: "one cap per run, from its planned hours, written in
  that run's plan for my OK. I create and delete the VMs myself.").
END

### FIX SPK-4
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Depends on:** the drafts approved; ML named and on hand (OQ-0003-1), MAC, and displays at 120 and 144 Hz; the
  netcode proof test's throwaway sim, client harness and room process (it runs after it); the sim core spike's sim
  for PM-0002-28 and PM-0004-8, -28 (it runs after that spike, so the sim is there: the advisor's reading,
  2026-11-03); Mustafa's session in Brave.
AFTER:
- **Depends on:** the drafts approved; the machines its rows name (OQ-0003-1, answered in part: Mustafa names them
  before the look proof test; until then the stand-in for ML, for rough checks only); this spike's own plan lists its
  rows that run before they are named, and none of them is judged on the stand-in (the advisor's reading, 2026-11-03);
  displays at 120 and 144 Hz; the netcode proof test's throwaway sim, client harness and room process (it runs after
  it); the sim core spike's sim for PM-0002-28 and PM-0004-8, -28 (it runs after that spike, so the sim is there: the
  advisor's reading, 2026-11-03); Mustafa's session in Brave.
END

### FIX SPK-5
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Depends on:** the drafts approved; RS orderable (OQ-0006-1), ML and RD named (OQ-0003-1), MAC; the
  scenes built in the spike (the DM_Tri-sized greybox for the stress scene, the Octopon-3 greybox, the two-floor test
  building with three stages, the ledge block, the box-only parity scenes); a stated bot set for "full AI" (ADR 0004's
  S14; OQ-0004-1 stays open); legacy golden fixtures captured from the legacy oracle (R5-TEST-REC3); a throwaway replay
  runner and tick bench in ADR 0002's shape; ADR 0001's prediction scope.
AFTER:
- **Depends on:** the drafts approved; RS orderable (OQ-0006-1); the machines its rows name (OQ-0003-1, answered in
  part: Mustafa names them before the look proof test; until then the stand-in for ML, for rough checks only); this
  spike's own plan lists its rows that run before they are named, and none of them is judged on the stand-in (the
  advisor's reading, 2026-11-03); the scenes built in the spike (the DM_Tri-sized greybox for the stress scene, the
  Octopon-3 greybox, the two-floor test building with three stages, the ledge block, the box-only parity scenes); a
  stated bot set for "full AI" (ADR 0004's S14; OQ-0004-1 stays open); legacy golden fixtures captured from the legacy
  oracle (R5-TEST-REC3); a throwaway replay runner and tick bench in ADR 0002's shape; ADR 0001's prediction scope.
END

### FIX SPK-6
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Depends on:** the drafts approved; ML and RD named and on hand (OQ-0003-1), MAC; a few authored test assets as plain
AFTER:
- **Depends on:** the drafts approved; ML, RD and MAC named and on hand (OQ-0003-1, answered in part: Mustafa names them
  before the look proof test); a few authored test assets as plain
END

### FIX SPK-7
FILE: docs/SPIKE_PLAN.md
BEFORE:
  unit of at least 20 parts, a few props; Mustafa makes them himself with AI help in Blender, as blockouts with a
  skin, named colour slots and LOOK_V2's proportions (OQ-SPIKE-1, answered by Mustafa, 2026-11-03, below); each object
  of the list may be a blockout at his bar, and the asset-pipeline research's export and slot-naming part reaches him
  with its export and slot rules before he makes them (the advisor's reading, 2026-11-03); whether a committed model
  may be seen by anyone is OQ-0006-18 (with OQ-0005-AP1), answered before the first model is committed, and if that
  comes before the design session after the netcode proof test, it goes to Mustafa then (the advisor's reading,
  2026-11-03); legacy screenshots at the same framing; ADR 0003 part 8's measurement harness, whose CPU side the
  boundary spike builds first and to which this test adds the GPU timestamp queries (its timed marks wait for it, its
  look work does not); a synthetic recording for the scene; the content crate's whole-set check (a minimal loader,
AFTER:
  unit of at least 20 parts, a few props; Mustafa makes them himself with AI help in Blender, where blockouts with a
  skin, named colour slots and LOOK_V2's proportions are enough (OQ-SPIKE-1, answered by Mustafa, 2026-11-03, below);
  each object of the list may be a blockout at his bar, and the asset-pipeline research's export and slot-naming part
  reaches him with its export and slot rules before he makes them (the advisor's reading, 2026-11-03); whether a
  committed model may be seen by anyone is OQ-0006-18 (with OQ-0005-AP1), answered before the first model is committed,
  and if that comes before the design session after the netcode proof test, it goes to Mustafa then (the advisor's
  reading, 2026-11-03); legacy screenshots at the same framing; ADR 0003 part 8's measurement harness, whose CPU side
  the boundary spike builds first and to which this test adds the GPU timestamp queries (its timed marks wait for it,
  its look work does not); a synthetic recording for the scene; the content crate's whole-set check (a minimal loader,
END

### FIX SPK-8
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Size:** L, without making the models, which is Mustafa's own work in Blender (OQ-SPIKE-1, answered), not a
  session's; ADR 0005's share S, or M if the kit builders are ported for the families' comparison.
AFTER:
- **Size:** L, without making the models, which is Mustafa's own work in Blender (OQ-SPIKE-1, answered); ADR 0005's
  share S, or M if the kit builders are ported for the families' comparison.
END

### FIX SPK-9
FILE: docs/SPIKE_PLAN.md
BEFORE:
  Blender (Spline's skins and named slots are unproven)."); Blender is the route for the look test's models and for
  this part's export, and LOOK_V2 is not edited. The part is agent research, not a heavy spike, so it runs alongside
  none: it sits between two wave-1 spikes, early enough that its export and slot rules reach Mustafa before he makes
  the models (recommended: right after the netcode proof test); both are the advisor's reading, 2026-11-03. The rest of
  the research runs before the asset spike.
AFTER:
  Blender (Spline's skins and named slots are unproven)."); Blender is the route for the look test's models and for this
  part's export, and LOOK_V2 is not edited; Mustafa (2026-11-03, LOOK_V2-§8): “yes, keep it as it is, with Blender for
  the test models only. When the asset research's export part reports, §8 says "Spline or Blender".” That edit belongs
  to that part's change-set and goes to Mustafa for the merge (the advisor's reading, 2026-11-03). The part is agent
  research, not a heavy spike, so it runs alongside none: it sits between two wave-1 spikes, early enough that its
  export and slot rules reach Mustafa before he makes the models (recommended: right after the netcode proof test); both
  are the advisor's reading, 2026-11-03. The rest of the research runs before the asset spike.
END

### FIX SPK-10
FILE: docs/SPIKE_PLAN.md
BEFORE:
The reference machines (OQ-0003-1: ML, RD and MAC still to be named), the server location (OQ-0006-1, OQ-0006-2), the
spikes' cost (OQ-0006-20, answered in part), the domain itself (OQ-0006-15, answered in part) and the provisional goal
budget (OQ-0006-19) are listed for Mustafa and the Advisor in `docs/adr/README.md`, "Decisions the drafts ask for".
The stop of 2026-11-03 answered the order after the netcode proof test, with the harness builders it names (J6 (5),
SPIKE-ORDER (b)), TICK-60-HEADROOM (OQ-0001-1), TICK-60-SETUP (OQ-0001-2), the "100 ms" reading (OQ-0001-3), the
profiles' split over the two directions (OQ-0001-16), the spikes' VMs (OQ-0006-17), ASSET-RESEARCH's timing and
OQ-SPIKE-1 (below); the register records each answer.
AFTER:
The reference machines ML, RD and MAC (OQ-0003-1, answered in part: Mustafa names them before the look proof test), the
server location (OQ-0006-1, OQ-0006-2) and the provisional goal budget (OQ-0006-19) are listed for Mustafa and the
Advisor in `docs/adr/README.md`, "Decisions the drafts ask for". The stop of 2026-11-03 answered the order after the
netcode proof test, with the harness builders it names (J6 (5), SPIKE-ORDER (b)), TICK-60-HEADROOM (OQ-0001-1),
TICK-60-SETUP (OQ-0001-2), the "100 ms" reading (OQ-0001-3), the profiles' split over the two directions (OQ-0001-16),
the spikes' VMs (OQ-0006-17), ASSET-RESEARCH's timing and OQ-SPIKE-1 (below); Mustafa's answers to the escalations,
later that day, answered the spikes' cost (OQ-0006-20) and the domain (OQ-0006-15); the register records each answer.
END

### FIX SPK-11
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **OQ-SPIKE-1. Who makes the look proof test's authored models, and by which route?** *Why it matters:* the look proof
  test runs in wave 1, before the asset-pipeline research, and needs as glTF/GLB with named material slots a skinned
  soldier with faction colour slots, the two-floor house with its damage stages and a very large unit of at least 20
  parts (ADR 0003 part 11); without them the look checklist cannot judge the soldier, the buildings or the very large
  unit. LOOK_V2 §8's flow makes a model in Spline ("AI-generated for drafts, or a clean model built by Claude") and
  exports glTF/GLB, but whether that export carries a skin and named slots is not researched (gap G1; OQ-0005-AS3,
AFTER:
- **OQ-SPIKE-1. Who makes the look proof test's authored models, and by which route?** *Why it matters (as asked):* the
  look proof test runs in wave 1, before the asset-pipeline research, and needs as glTF/GLB with named material slots a
  skinned soldier with faction colour slots, the two-floor house with its damage stages and a very large unit of at
  least 20 parts (ADR 0003 part 11); without them the look checklist cannot judge the soldier, the buildings or the very
  large unit. LOOK_V2 §8's flow makes a model in Spline ("AI-generated for drafts, or a clean model built by Claude")
  and exports glTF/GLB, but whether that export carries a skin and named slots is not researched (gap G1; OQ-0005-AS3,
END

### FIX SPK-12
FILE: docs/SPIKE_PLAN.md
BEFORE:
  are enough." Making the models is his own work, not a session's, so no spike's size carries it. LOOK_V2 is not
AFTER:
  are enough." Making the models is his own work, so no spike's size carries it. LOOK_V2 is not
END

### NOTE SPK-N1
no change: SPIKE_PLAN:347, the preview hosting check's "Depends on" ("ML and MAC with the four browsers"). Condition
4 names items 1, 3 and 4 only, so item 6 keeps its text. Left for the report, beside MACHINES-TIMING: the check may
run alongside any heavy spike (SPIKE-ORDER (b)), so it can run before ML and MAC are named, and its rows PM-0006-1 to
-6 and -8 name ML or MAC as the browser machine (ADR 0006's pass-mark table, machine column; PM-0006-2 says "a header
check, not a hardware result"). Whether those rows may run on the stand-in for ML is that check's own plan to say.
END

### NOTE SPK-N2
no change: the other places these answers touch in SPIKE_PLAN. Item 7 (:395, "RS and ML; MAC"), item 8 and item 10
run in wave 3 or later, after the look proof test, when Mustafa has named the machines, so they keep their text.
OQ-SPIKE-1's answer (:517-524) keeps "LOOK_V2 is not edited: Blender is the route for the look test's models", which
agrees with his LOOK_V2-§8 answer; that answer is quoted once, in the research section (FIX SPK-9), as the ruling
places it. "The netcode proof test doesn't measure client frames" is not quoted in SPIKE_PLAN: condition 4 quotes it
once, in ADR 0001's key (test-engineer); item 1 carries condition 4's clause instead (FIX SPK-2).
END

### NOTE OVW-N1
FILE: docs/ENGINE_OVERVIEW.md
no change: no line of ENGINE_OVERVIEW states the reference machines, the domain, the spikes' cost, an upgrade or
spawn rule, the fragments or community servers (grep for OQ-0003-1, machine, domain, cost, fragment, visib, spawn,
community, stand-in finds :93 and :116, ADR 0001's visibility classes in general; :114, the ability step's cost; :134,
ADR 0006 deciding costs). Its server box (:36, "supervisor, lobby, metrics, match record") lists the server side's
parts and does not make a room need the supervisor or the lobby to start, so nothing there reads as closing the
door on COMMUNITY-SERVERS (condition 6). Its "Open, for Mustafa and the Advisor" section points to the register,
which carries the answers (FIX IDX-1 to IDX-16).
END

