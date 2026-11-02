32: [no id] Code or OQ
34: OQ-0001-1 line 852->1022 DECIDER
   README Q: Is the 60 Hz margin measured against the 60 Hz server tick budget (8.33 ms), so 60 Hz needs a p99 server tick time of at most 6.25 ms plus every 60 Hz research mark?
   DRAFT  T: Is TICK-60-HEADROOM's reading confirmed?
   README D: Mustafa and the Advisor (confirm at the stop)
   DRAFT  D: Mustafa and the Advisor, "confirm at the stop"
35: OQ-0001-2 line 859->1029 
   README Q: How does the stress scene's sim load reach the 60 Hz margin run: (a) the sim core spike's sim inside the proof-test server (recommended), (b) a stand-in replaying measured step times, (c) wait until the real sim runs the stress scene?
   DRAFT  T: How does the stress scene's sim load reach the 60 Hz margin run (TICK-60-SETUP)?
   README D: the Advisor
   DRAFT  D: the Advisor
36: OQ-0001-3 line 869->1049 DECIDER
   README Q: Does the netcode proof test's "100 ms of simulated latency" mean profile P2 (100 ms round trip, with P2's jitter and loss), or 100 ms each way?
   DRAFT  T: Is "100 ms of simulated latency" P2?
   README D: the Advisor (who wrote the definition)
   DRAFT  D: the Advisor, who wrote the definition
37: OQ-0001-4 line 874->1054 
   README Q: Does the client predict the player's own abilities? Decided after the Feel Lab test on the Molotov and the Shield.
   DRAFT  T: Does the client predict the player's own abilities (ABL-PREDICT)?
   README D: Mustafa
   DRAFT  D: Mustafa
38: [no id] ASSET-RESEARCH (timing)
39: OQ-0005-AB5 line 1158->1239 
   README Q: How does an account-unlocked weapon reach a match (LOOP_V2 §8, VISION_V2 §1), when buying offers no weapon choice and other weapons come only from drops?
   DRAFT  T: WEAPON-UNLOCKS: how does an account-unlocked weapon reach a match, when buying offers no weapon choice and other weapons come only from drops?
   README D: Mustafa, with the Advisor (LOOP_V2 §8 is theirs)
   DRAFT  D: Mustafa, with the Advisor (LOOP_V2 §8 is theirs)
40: OQ-0005-AB1 line 1105->1186 
   README Q: How many weapons can a player carry besides the class weapon, and what happens to the one in hand on a pickup?
   DRAFT  T: Carry slots: how many weapons can a player carry besides the class weapon, and what happens to the one in hand when they take a pickup?
   README D: Mustafa
   DRAFT  D: Mustafa
41: OQ-0005-AB2 line 1117->1198 
   README Q: What does a looted gun come with, and can it be refilled?
   DRAFT  T: Ammo for looted weapons: what does a looted gun come with, and can it be refilled?
   README D: Mustafa
   DRAFT  D: Mustafa
42: OQ-0005-AB3 line 1125->1206 
   README Q: What earns a level, how fast, and does a track the player switches to catch up?
   DRAFT  T: How fast tracks level: what earns a level, how fast, and does a track the player switches to catch up?
   README D: Mustafa
   DRAFT  D: Mustafa
43: OQ-0003-1 line 723->800 
   README Q: Which laptop (the minimum) and which desktop (the recommended) are bought or borrowed as the two reference machines?
   DRAFT  T: Which laptop and which desktop are the two reference machines?
   README D: Mustafa and the Advisor (money and availability)
   DRAFT  D: Mustafa and the Advisor (money and availability)
44: [no id] Spike order (J6 (5))
45: OQ-0001-14 line 927->1120 
   README Q: Is a VISION_V2 §3 or §4 target trace enough for an engine feature (written "need: VISION_V2 §4 only" and listed below)? The session read it so on 2026-10-31.
   DRAFT  T: Is a VISION_V2 §3 or §4 target trace enough for an engine feature?
   README D: the Advisor, at the stop
   DRAFT  D: the Advisor, at the stop
46: OQ-0002-8 line 897->1053 
   README Q: May a record propose a structure no research Options section offers, when a settled input forces it and the research leaves the question to that record (the `room` and `navbake` crates)?
   DRAFT  T: May this record propose a structure no research Options section offers, when a settled input forces it and the research leaves the question to this record?
   README D: the Advisor, at the stop
   DRAFT  D: the Advisor, at the stop. -
47: OQ-0006-5 line 655->807 DECIDER
   README Q: Which certificate does the WebSocket fallback present on development and staging servers, since a WebSocket cannot pin a certificate hash? The netcode proof test's WebSocket leg needs the answer first.
   DRAFT  T: How are the per-hostname production certificates issued, how long do they live, and which certificate does the WebSocket fallback present on development and staging servers?
   README D: the Advisor, from a short read, before the netcode proof test starts (production issuance: security-auditor, in the security baseline)
   DRAFT  D: security-auditor in the security baseline, with the port plan (OQ-0006-16); the development and staging case before the netcode proof test starts (the Advisor, from a short read)
48: [no id] Fragments as damage
49: [no id] The design questions
50: OQ-0001-5 line 880->1060 
   README Q: The rewind cap: 200 or 250 ms, and the victim's side?
   DRAFT  T: The rewind cap: which value, and the victim's side?
   README D: Mustafa and the Advisor
   DRAFT  D: Mustafa and the Advisor
51: OQ-0001-7 line 894->1084 DECIDER
   README Q: Do shots carry their moment inside a tick at 30 Hz?
   DRAFT  T: Do shots and the active-reload tap carry their moment inside a tick at 30 Hz?
   README D: the netcode proof test, then Mustafa in the Feel Lab if it fails or feels late
   DRAFT  D: for shots, the netcode proof test, then Mustafa in the Feel Lab if it fails or feels late; for the reload tap, Mustafa in the Feel Lab
52: OQ-0001-8 line 899->1092 
   README Q: How far does a new projectile move forward on spawn?
   DRAFT  T: How far does a new projectile move forward on spawn?
   README D: Mustafa
   DRAFT  D: Mustafa
53: OQ-0001-9 line 903->1096 
   README Q: What else does lag compensation cover?
   DRAFT  T: What else does lag compensation cover?
   README D: the netcode proof test for cover; for melee, a design call by Mustafa with the game-designer
   DRAFT  D: the netcode proof test for cover; for melee, a design call by Mustafa with the game-designer
54: OQ-0001-10 line 909->1102 
   README Q: Are weapon pickups predicted?
   DRAFT  T: Are weapon pickups predicted?
   README D: Mustafa
   DRAFT  D: Mustafa
55: OQ-0001-11 line 913->1106 
   README Q: Room codes: length, alphabet and access.
   DRAFT  T: Room codes: length, alphabet and access.
   README D: Mustafa (what a host may set, R2-ROOM-OQ5) with the security review's options
   DRAFT  D: Mustafa (what a host may set, R2-ROOM-OQ5) with the security review's options
56: OQ-0001-12 line 918->1111 
   README Q: Do far sound events reveal too much?
   DRAFT  T: Do far sound events reveal too much?
   README D: Mustafa with the game-designer
   DRAFT  D: Mustafa with the game-designer
57: OQ-0002-1 line 820->971 
   README Q: Does the v2 editor still open by double-clicking its HTML file?
   DRAFT  T: Does the v2 editor still open by double-clicking its HTML file?
   README D: the boundary spike (does it load?), then Mustafa if EDITOR_V2 §5's rule must change
   DRAFT  D: the boundary spike (does it load?), then Mustafa if EDITOR_V2 §5's rule must change. -
58: OQ-0002-4 line 856->1007 
   README Q: Does production ever turn on cross-origin isolation, and can a later login popup live with it?
   DRAFT  T: Does production ever turn on cross-origin isolation, and can a later login popup live with it?
   README D: the boundary spike's results, then this record; for embeds and the login flow under isolation, Mustafa with ADR 0006 (R1-BND-OQ5, only if isolation is adopted)
   DRAFT  D: the boundary spike's results, then this record; for embeds and the login flow under isolation, Mustafa with ADR 0006 (R1-BND-OQ5, only if isolation is adopted). -
59: OQ-0002-5 line 868->1019 DECIDER
   README Q: Does real Safari run on pull requests, and does the end-to-end suite need it?
   DRAFT  T: Does real Safari run on pull requests, and does the end-to-end suite need it?
   README D: infra-engineer's reading of the macOS runner's time and cost (ADR 0006 for costs; Mustafa for any paid plan), then this record
   DRAFT  D: this record, from the macOS job's wall time, which infra-engineer measures in the foundation; ADR 0006 and Mustafa only if the repository stops being public, since a private repository's runners draw on paid minutes (the same page, read at the step-10 review). -
60: OQ-0002-6 line 879->1035 
   README Q: Which jump reach do the golden tests pin?
   DRAFT  T: Which jump reach do the golden tests pin?
   README D: the game-designer with Mustafa, from the loop slice on (a rule question)
   DRAFT  D: the game-designer with Mustafa, from the loop slice on (a rule question). -
61: OQ-0003-2 line 731->808 
   README Q: The ink's width, its inner lines and its weight on buildings.
   DRAFT  T: The ink's width, its inner lines and its weight on buildings.
   README D: Mustafa, in the look proof test, with its measured cost; the family in part 5.b follows his pick
   DRAFT  D: Mustafa, in the look proof test, with its measured cost; the family in part 5.b follows his pick
62: OQ-0003-3 line 740->819 
   README Q: Which colour output: authored colour as shown, or a tone-mapped output?
   DRAFT  T: Which colour output: authored colour as shown, or a tone-mapped output?
   README D: Mustafa, in the look proof test, side by side with legacy
   DRAFT  D: Mustafa, in the look proof test, side by side with legacy
63: OQ-0003-4 line 746->825 
   README Q: Is one sun shadow map enough, or are cascades or another setup needed?
   DRAFT  T: Is one sun shadow map enough, or are cascades or another setup needed?
   README D: the look proof test (softness, Mustafa) and the render budget bench (the shadow row)
   DRAFT  D: the look proof test (softness, Mustafa) and the render budget bench (the shadow row)
64: OQ-0003-5 line 752->831 
   README Q: How are "warm sun with cool shadows" and moods per map lit?
   DRAFT  T: How are "warm sun with cool shadows" and moods per map lit?
   README D: Mustafa, in the look proof test, with each option's GPU time
   DRAFT  D: Mustafa, in the look proof test, with each option's GPU time
65: OQ-0003-8 line 771->850 
   README Q: How is ink line boil drawn?
   DRAFT  T: How is ink line boil drawn?
   README D: Mustafa, in the look proof test
   DRAFT  D: Mustafa, in the look proof test
66: OQ-0003-9 line 776->855 
   README Q: How are vegetation and the lush countryside drawn, and in which tree style?
   DRAFT  T: How are vegetation and the lush countryside drawn, and in which tree style?
   README D: Mustafa (the style, a later design session); the render budget bench (the technique)
   DRAFT  D: Mustafa (the style, a later design session); the render budget bench (the technique)
67: OQ-0003-10 line 783->862 
   README Q: Does the frame rate follow a 120 or 144 Hz screen, or is it capped?
   DRAFT  T: Does the frame rate follow a 120 or 144 Hz screen, or is it capped?
   README D: Mustafa (feel), on the render budget bench's numbers
   DRAFT  D: Mustafa (feel), on the render budget bench's numbers
68: OQ-0003-11 line 789->868 
   README Q: How many players use the WebGL 2 fallback?
   DRAFT  T: How many players use the WebGL 2 fallback?
   README D: Mustafa, on client field data after launch
   DRAFT  D: Mustafa, on client field data after launch
69: OQ-0003-12 line 796->875 
   README Q: Is the pixel ratio capped at 1 on Low for HiDPI laptops?
   DRAFT  T: Is the pixel ratio capped at 1 on Low for HiDPI laptops?
   README D: Mustafa, in the look proof test
   DRAFT  D: Mustafa, in the look proof test
70: OQ-0003-13 line 801->880 
   README Q: Where does the audio listener sit under the v2 camera?
   DRAFT  T: Where does the audio listener sit under the v2 camera?
   README D: the camera framing first (R2-INT-OQ2, a design question), then Mustafa's listening check
   DRAFT  D: the camera framing first (R2-INT-OQ2, a design question), then Mustafa's listening check
71: OQ-0003-15 line 812->891 
   README Q: The legacy look touches LOOK_V2 does not name: keep, change or drop each?
   DRAFT  T: The legacy look touches LOOK_V2 does not name: keep, change or drop each?
   README D: Mustafa, in the look proof test
   DRAFT  D: Mustafa, in the look proof test
72: OQ-0004-1 line 755->855 
   README Q: How is bot behaviour structured?
   DRAFT  T: How is bot behaviour structured?
   README D: a bot research unit, or the slice 2 plan, with the game-designer; Mustafa for the behaviour set
   DRAFT  D: a bot research unit, or the slice 2 plan, with the game-designer; Mustafa for the behaviour set. -
73: OQ-0004-4 line 778->878 
   README Q: What moves a building through its damage stages?
   DRAFT  T: What moves a building through its damage stages?
   README D: Mustafa, with the game-designer (rule 6: no rule beyond LOOK_V2 is designed here)
   DRAFT  D: Mustafa, with the game-designer (rule 6: no rule beyond LOOK_V2 is designed here). -
74: OQ-0005-CD1 line 587->619 
   README Q: Do hand-edited content files need comments?
   DRAFT  T: Do hand-edited content files need comments?
   README D: Mustafa, after a short trial in the content and ability spike
   DRAFT  D: Mustafa, after a short trial in the content and ability spike
75: OQ-0005-CD2 line 593->625 
   README Q: What load time may content cost, how much of the Wasm file is content's, and is JSON fast and small enough?
   DRAFT  T: What load time may content cost, how much of the Wasm file is content's, and is JSON fast and small enough?
   README D: for load time, the content and ability spike's measurement, then the Advisor sets the budget; for size, ADR 0002 part 4: the Advisor sets the first `sim-wasm` limit from the boundary and sim core spikes' measurements and this spike's content share, and until then the size is recorded, not gated
   DRAFT  D: for load time, the content and ability spike's measurement, then the Advisor sets the budget; for size, ADR 0002 part 4: the Advisor sets the first `sim-wasm` limit from the boundary and sim core spikes' measurements and this spike's content share, and until then the size is recorded, not gated
76: OQ-0005-CD3 line 606->638 
   README Q: May production ever change numbers in the middle of a match?
   DRAFT  T: May production ever change numbers in the middle of a match?
   README D: Mustafa
   DRAFT  D: Mustafa
77: OQ-0005-CD4 line 614->646 
   README Q: In development, do simulation numbers apply at the next match, or inside a running match?
   DRAFT  T: In development, do simulation numbers apply at the next match, or inside a running match?
   README D: Mustafa and the Advisor
   DRAFT  D: Mustafa and the Advisor
78: OQ-0005-CD5 line 626->658 
   README Q: Faction variants: full definitions that reference shared ones, or overrides of a base?
   DRAFT  T: Faction variants: full definitions that reference shared ones, or overrides of a base?
   README D: the faction design session (VISION_V2 §7, faction asymmetry; Mustafa)
   DRAFT  D: the faction design session (VISION_V2 §7, faction asymmetry; Mustafa)
79: OQ-0005-CD6 line 632->664 
   README Q: How are moment rules written?
   DRAFT  T: How are moment rules written?
   README D: the moment design session (Mustafa with the game-designer, with R6-TEL-OQ3's thresholds), then ADR 0004
   DRAFT  D: the moment design session (Mustafa with the game-designer, with R6-TEL-OQ3's thresholds), then ADR 0004
80: OQ-0005-CD7 line 640->672 
   README Q: What is the Feel Lab tool?
   DRAFT  T: What is the Feel Lab tool?
   README D: a later design session with Mustafa, before the feel pass (LOOP_V2 §11 leaves the feel pass out of the greybox v0); ADR 0003 for its screen
   DRAFT  D: a later design session with Mustafa, before the feel pass (LOOP_V2 §11 leaves the feel pass out of the greybox v0); ADR 0003 for its screen
81: OQ-0005-AB4 line 1145->1226 
   README Q: What is one track: which definitions share one?
   DRAFT  T: What is one track: which definitions share one?
   README D: Mustafa
   DRAFT  D: Mustafa
82: OQ-0005-AB6 line 1174->1255 
   README Q: How do the player's tracks sit beside the team's tech tiers?
   DRAFT  T: How do the player's tracks sit beside the team's tech tiers?
   README D: Mustafa
   DRAFT  D: Mustafa
83: OQ-0005-AB7 line 1182->1263 
   README Q: May a pick add raw power (more damage, more health), or only change how a unit or weapon plays?
   DRAFT  T: May a pick add raw power (more damage, more health), or only change how a unit or weapon plays?
   README D: Mustafa
   DRAFT  D: Mustafa
84: OQ-0005-AB8 line 1191->1274 
   README Q: The offer: when does it appear, and when can the player answer it?
   DRAFT  T: The offer: when does it appear, and when can the player answer it?
   README D: Mustafa
   DRAFT  D: Mustafa
85: OQ-0005-AB9 line 1199->1288 
   README Q: Which drop and pickup rules does UPGRADE-SCOPE leave unstated?
   DRAFT  T: Which drop and pickup rules does UPGRADE-SCOPE leave unstated?
   README D: Mustafa, with the game-designer
   DRAFT  D: Mustafa, with the game-designer
86: OQ-0005-AB10 line 1212->1305 
   README Q: Do bots have tracks and make picks, and what does a human get when taking a bot's slot?
   DRAFT  T: Do bots have tracks and make picks, and what does a human get when taking a bot's slot?
   README D: Mustafa
   DRAFT  D: Mustafa
87: OQ-0005-AB11 line 1221->1319 
   README Q: Does the Riot Shield answer the Banzai blade, and is the Peacekeeper's side of the triangle strong enough before Tech 1?
   DRAFT  T: Does the Riot Shield answer the Banzai blade, and is the Peacekeeper's side of the triangle strong enough before Tech 1?
   README D: Mustafa
   DRAFT  D: Mustafa
88: OQ-0005-AB12 line 1232->1330 
   README Q: Is tranek's write-up accepted as the GAS reference, or does a verifier read Epic's documentation first?
   DRAFT  T: Is tranek's write-up accepted as the GAS reference, or does a verifier read Epic's documentation first?
   README D: the Advisor
   DRAFT  D: the Advisor
89: OQ-0005-AB13 line 1241->1339 
   README Q: When does scripting (option A3) come back?
   DRAFT  T: When does scripting (option A3) come back?
   README D: the Advisor (the source)
   DRAFT  D: the Advisor (the source)
90: OQ-0005-AP2 line 1498->1646 
   README Q: Which asset families become authored models, and which stay procedural builders?
   DRAFT  T: Which asset families become authored models, and which stay procedural builders?
   README D: Mustafa, in the look proof test (how each looks, beside legacy screenshots, LOOK_V2 §9), with the asset-pipeline research's cost and tooling. Trees wait for the tree style (LOOK_V2 §8 `[OPEN]`)
   DRAFT  D: Mustafa, in the look proof test (how each looks, beside legacy screenshots, LOOK_V2 §9), with the asset-pipeline research's cost and tooling. Trees wait for the tree style (LOOK_V2 §8 `[OPEN]`)
91: OQ-0005-AP3 line 1506->1654 
   README Q: May an AI-generated draft model ever ship?
   DRAFT  T: May an AI-generated draft model ever ship?
   README D: Mustafa
   DRAFT  D: Mustafa
92: OQ-0005-AS2 line 1643->1809 
   README Q: Which tool builds the LOD levels, and how many levels?
   DRAFT  T: Which tool builds the LOD levels, and how many levels?
   README D: the asset-pipeline research, then the asset spike and Mustafa (silhouettes kept)
   DRAFT  D: the asset-pipeline research, then the asset spike and Mustafa (silhouettes kept)
93: OQ-0005-1 line 1860->2037 
   README Q: Do presentation references sit on the simulation definitions, or in client definitions keyed by the same id?
   DRAFT  T: Do presentation references sit on the simulation definitions, or in client definitions keyed by the same id?
   README D: the Advisor, at the stop review (the game-designer changes AB-4's kinds if the answer needs it)
   DRAFT  D: the Advisor, at the stop review (the game-designer changes AB-4's kinds if the answer needs it)
94: OQ-0005-2 line 1880->2057 
   README Q: Is a weapon definition the same thing as a weapon type?
   DRAFT  T: Is a weapon definition the same thing as a weapon type?
   README D: Mustafa answers OQ-0005-AB4; the question stays open for the Advisor until then
   DRAFT  D: Mustafa answers OQ-0005-AB4; the question stays open for the Advisor until then
95: OQ-0005-3 line 1891->2068 
   README Q: What does the content hash cover?
   DRAFT  T: What does the content hash cover?
   README D: the Advisor, with the infra-engineer, whose ADR 0006 owns the analysis (R6-TEL-REC3)
   DRAFT  D: the Advisor, with the infra-engineer, whose ADR 0006 owns the analysis (R6-TEL-REC3)
96: OQ-0005-4 line 1903->2080 
   README Q: What do the Character Lab and the Anim Lab work on once soldiers are authored models?
   DRAFT  T: What do the Character Lab and the Anim Lab work on once soldiers are authored models?
   README D: Mustafa with the Advisor (EDITOR_V2 is theirs)
   DRAFT  D: Mustafa with the Advisor (EDITOR_V2 is theirs)
97: OQ-0006-1 line 613->762 
   README Q: Can Mustafa order Hetzner's CCX23 and CX33 in an EU location?
   DRAFT  T: Can Mustafa order Hetzner's CCX23 and CX33 in an EU location?
   README D: Mustafa (a console check in his own account; agents create no account), then a research read if (b)
   DRAFT  D: Mustafa (a console check in his own account; agents create no account), then a research read if (b)
98: OQ-0006-2 line 625->774 DECIDER
   README Q: Which EU location: Nuremberg, Helsinki, or Falkenstein if offered?
   DRAFT  T: Which EU location: Nuremberg, Helsinki, or Falkenstein if offered?
   README D: a measurement (the netcode proof test's setup or a small check before it), confirmed by Mustafa
   DRAFT  D: a measurement made before the reference server is created (a small check at the start of the netcode proof test's setup), confirmed by Mustafa; the reference server is then created in the location it picks
99: OQ-0006-3 line 632->782 
   README Q: Where does the lobby run, and what stores guest ids and the room directory?
   DRAFT  T: Where does the lobby run, and what stores guest ids and the room directory?
   README D: the Advisor, from a short lobby read done with the security baseline (its key storage is item B5); Mustafa for any paid service. ADR 0001 keeps the protocol, ADR 0002 the crate
   DRAFT  D: the Advisor, from a short lobby read done with the security baseline (its key storage is item B5); Mustafa for any paid service. ADR 0001 keeps the protocol, ADR 0002 the crate
100: OQ-0006-4 line 643->795 
   README Q: Once the access wall is on, does it cover every preview, or only multiplayer previews and previews with unreleased art?
   DRAFT  T: Once the access wall is on, does it cover every preview, or only multiplayer previews and previews with unreleased art?
   README D: Mustafa (convenience against one more project to run)
   DRAFT  D: Mustafa (convenience against one more project to run)
101: OQ-0006-6 line 673->833 
   README Q: The security baseline: what it decides for each waiting item.
   DRAFT  T: The security baseline: what it decides for each waiting item.
   README D: security-auditor, in its own fresh session (BASELINE-WHEN (b)); the Advisor and Mustafa for the policy calls
   DRAFT  D: security-auditor, in its own fresh session (BASELINE-WHEN (b)); the Advisor and Mustafa for the policy calls
102: OQ-0006-7 line 684->844 
   README Q: Will any CI runner be paid for: GPU, macOS, self-hosted reference machines, or a cache above 10 GB?
   DRAFT  T: Will any CI runner be paid for: GPU, self-hosted reference machines, or a cache above 10 GB?
   README D: Mustafa for any paid plan; ADR 0002 says which jobs need one; the security baseline weighs (c)
   DRAFT  D: Mustafa for any paid plan; ADR 0002 says which jobs need one; the security baseline weighs (c)
103: OQ-0006-8 line 694->854 
   README Q: What do Sentry and Grafana really cost, and how do logs and metrics reach Grafana Cloud?
   DRAFT  T: What do Sentry and Grafana really cost, and how do logs and metrics reach Grafana Cloud?
   README D: Mustafa, with a read of sentry.io/pricing before he creates the Sentry organization; a read of Grafana's ingestion docs if M2 is wanted
   DRAFT  D: Mustafa, with a read of sentry.io/pricing before he creates the Sentry organization; a read of Grafana's ingestion docs if M2 is wanted
104: OQ-0006-9 line 703->863 
   README Q: What does EU data-protection law require of us, and how long are match records kept?
   DRAFT  T: What does EU data-protection law require of us, and how long are match records kept?
   README D: Mustafa
   DRAFT  D: Mustafa
105: OQ-0006-10 line 712->873 
   README Q: How many players, and when; which VAT and currency; is included traffic per VM, pooled or prorated?
   DRAFT  T: How many players, and when; which VAT and currency; is included traffic per VM, pooled or prorated?
   README D: Mustafa (his plans, his billing address, the first invoice)
   DRAFT  D: Mustafa (his plans, his billing address, the first invoice)
106: OQ-0006-12 line 727->888 
   README Q: If production isolation is ever adopted: embeds, cross-origin popups, CORP and the login flow.
   DRAFT  T: If production isolation is ever adopted: embeds, cross-origin popups, CORP and the login flow.
   README D: Mustafa with this record, only if production isolation is adopted (R1-BND-REC2 mark 5's test)
   DRAFT  D: Mustafa with this record, only if production isolation is adopted (R1-BND-REC2 mark 5's test)
