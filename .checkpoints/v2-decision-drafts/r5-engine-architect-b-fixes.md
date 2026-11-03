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
| OQ-0006-21 (COMMUNITY-SERVERS) | Could players one day host their own servers, and what must the drafts keep possible for it? Mustafa (2026-11-03), a new question, "keep the door open, don't design it": "players may one day host their own servers, possibly with revenue such as official private servers they rent." and "The drafts must not lock it out: the room server stays one self-contained program with a config file (players, length, rules), checks the game's version and content hash, and marks each match as official or community, so progression can count on official servers only." No option is weighed and no timing is set. | Mustafa | [ADR 0006](0006-infrastructure-operations.md) OQ-0006-21 |
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

