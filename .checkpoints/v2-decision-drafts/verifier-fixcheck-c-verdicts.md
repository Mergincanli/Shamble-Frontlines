# verifier-fixcheck-c: the drafts' rounds 3 and 4 (57c43c2..47eadd9) and the synthesis round (80a27b3..47eadd9)

Verifier, 2026-11-02 from 09:28 (Amsterdam). Branch claude/laughing-lovelace-ox61se, HEAD = 47eadd9 (the working tree's
docs equal 47eadd9). Every doc line below is the file at 47eadd9 (copies in `.gate/fixcheck-c/*-head.md`; diffs
`adrs.diff`, `adrs.wdiff`, `synth.diff`). Research lines are the topic files at 9e7db28 (unchanged on the branch).
Changed lines whose only change is a pointer were judged on the changed words (word diff); the rest of such a row was
verified by the earlier units and is not re-judged. Notes: `verifier-fixcheck-c-notes.md`.

`r1` = docs/research/r1-simulation-core.md; `LI` = docs/LEGACY_INVENTORY.md; `NA-draft` =
`.checkpoints/v2-decision-drafts/netcode-architect-adr0001-draft.md` (the text the old ADR 0001 line pointers were written
on: its line 527 is the row "Server tick time, 30 Hz", and so on).

## 1. The drafts' rounds 3 and 4, claim by claim

| # | doc:line | claim (short) | cited | verdict | where and the words | note |
|---|---|---|---|---|---|---|
| A1 | 0001:40–41 | the stack is a settled input | STATUS "Decisions pending"; brief §1 | confirmed | STATUS:113 "Its settled inputs (never reopened): the stack"; drafts brief §1 (appendix §1, lines 34–37) "**The stack:** … our own netcode with server authority, over WebTransport with a WebSocket fallback" | "brief" in 0001 is the drafts brief (its §2 is the definitions, as 0001:44 uses) |
| A2 | 0001:720 | Q17's B and D settled by the stack | as A1 | confirmed | as A1 | stays one 151-character line on purpose (netcode fixes-3, NOTE A) |
| A3 | 0001:1504 | Sources: STATUS lists the stack among the drafts brief's settled inputs | STATUS | confirmed | STATUS:113 | |
| A4 | 0001:1507–1508 | CLAUDE.md "The v2 rebuild" and V2_PHASE0_BRIEF §0 (line 9) state the netcode part | CLAUDE.md; brief | confirmed | V2_PHASE0_BRIEF:9 "**our own netcode**: server authority, WebTransport with a WebSocket fallback."; CLAUDE.md, "The v2 rebuild", first bullet: "our own netcode (server authority, WebTransport with a WebSocket fallback)" | |
| A5 | 0001:1073–1074 | the wrap: part 6 (the header's tick) and the "Failure modes" row "A tick counter wraps" expect it only at 60 Hz | was NA-draft 240–243, 555 | confirmed; anchor right | NA-draft:240–243 = the 32-bit header tick and "a 16-bit tick wraps after 36.4 minutes at 30 Hz … and 18.2 minutes at 60 Hz, inside a 30-minute soak"; NA-draft:555 = row "A tick counter wraps"; HEAD 0001:243–244 and :561 | |
| A6 | 0001:1122 (PM-0001-4) | budgets table, row "Server tick time, 30 Hz (the baseline)" | was NA-draft 527 | anchor right | NA-draft:527 is that row; HEAD 0001:533 | |
| A7 | 0001:1123 (PM-0001-5) | row "Its two shares, 30 Hz" | was 528 | anchor right | NA-draft:528; HEAD 0001:534 | |
| A8 | 0001:1125 (PM-0001-7) | part 8, the history "at most 1 MB for the stress scene" | was 274–275 | anchor right; quote exact | NA-draft:274–275; HEAD 0001:276–277 "at most 1 MB / for the stress scene" | |
| A9 | 0001:1126 (PM-0001-8) | part 9 (d), the accumulator; part 6's per-client hard cap | was 294–295; 223–224 | anchor right | NA-draft:294 "(d) A per-client priority accumulator"; :223 "A per-client hard cap"; HEAD 0001:296, :224 | |
| A10 | 0001:1129 (PM-0001-11) | budgets row "Transport runtime" | was line 532 | anchor right | NA-draft:532; HEAD 0001:538 "30 input datagrams per second per session" | |
| A11 | 0001:1130 (PM-0001-12) | row "Downstream per client"; part 6's per-client hard cap | was 533; 222–224 | anchor right | NA-draft:533; :223–224; HEAD 0001:539, :224 | |
| A12 | 0001:1131 (PM-0001-13) | "Bandwidth estimate", the bullet "Upstream per client" | was 507–512 | anchor right | NA-draft:507 "- **Upstream per client:**" (the estimate's bullet, not the budgets row of the same name); HEAD 0001:503, :513 | the anchor names the section, so it cannot be mistaken for the budgets row at 0001:540 |
| A13 | 0001:1132 (PM-0001-14) | part 6, *Datagrams:* size cap, split by groups, first full state on a stream | was 224–228 | anchor right | NA-draft:224–228; HEAD 0001:225–229 | |
| A14 | 0001:1133 (PM-0001-15) | row "Memory per room" | was 535 | anchor right | NA-draft:535; HEAD 0001:541 | |
| A15 | 0001:1134 (PM-0001-16) | row "Room start-up" | was 536 | anchor right | NA-draft:536; HEAD 0001:542 | |
| A16 | 0001:1135 (PM-0001-17) | part 6, the header's 32-bit tick and 16-bit offsets | was 240–243 | anchor right | NA-draft:240–241; HEAD 0001:243–244 | |
| A17 | 0001:1137 (PM-0001-18) | "Failure modes", row "A burst of total loss (B1)" | was 542 | anchor right | NA-draft:542; HEAD 0001:548 | |
| A18 | 0001:1138 (PM-0001-19) | "the one join mark": "What a spike must prove" item 2; row "A trap in the client's Wasm sim" | was 952; 550 | anchor right | NA-draft:952 "(R2-BUD-REC3, the one join mark," inside item 2; :550 the trap row; HEAD 0001:974, :556 | |
| A19 | 0001:1140 (PM-0001-21) | part 5, the server's checks on every command | was 190–196 | anchor right | NA-draft:190 "The server checks every command"; HEAD 0001:191 | |
| A20 | 0001:1141 (PM-0001-22) | part 6, *Events:* the bounded queue | was 230–232 | anchor right | NA-draft:230–232; HEAD 0001:229–234 | |
| A21 | 0001:1149 (PM-0001-29) | part 10, WebTransport first with a timeout, then WebSocket | was 312 | anchor right | NA-draft:312; HEAD 0001:314 | |
| A22 | 0001:1151 (PM-0001-31) | the P2 restatement is "What a spike must prove" item 1 | was 937 | anchor right | NA-draft:937 "(restated from "100 ms RTT and 1 % loss")", in item 1; HEAD 0001:957 | |
| A23 | 0001:1153 (PM-0001-33) | the ±5 ms restatement is item 1 | was 940 | anchor right | NA-draft:940; HEAD 0001:959–960 "within ±5 ms at P2" | |
| A24 | 0001:1156 (PM-0001-36) | part 6, *Datagrams:* the size cap, `maxDatagramSize` | was 224–226 | anchor right | NA-draft:224–226; HEAD 0001:225–227 | |
| A25 | 0001:1160 (PM-0001-39) | part 2 (the tick rate rule); row "The 60 Hz margin run (decides 60 Hz)"; item 3 | was 118–127; 530; 965–967 | anchor right | NA-draft:118, :530, :965; HEAD 0001:118, :536, :987 | |
| A26 | 0001:1161 (PM-0001-40) | part 2, "every 60 Hz mark the research sets, as written" | was 125–126 | anchor right; quote exact | HEAD 0001:125–126 "every 60 Hz mark the research sets, as / written" | |
| A27 | 0001:1163 (PM-0001-41) | part 4, the prediction scope and its replay | was 165–174 | anchor right | NA-draft:165–174; HEAD 0001:166 | |
| A28 | 0001:1164 (PM-0001-42) | part 4, the correction's fade and snap | was 172–174 | anchor right | HEAD 0001:173–174 "A correction fades … snaps above a data-set distance (proposed 2 m)" | |
| A29 | 0001:1166 (PM-0001-44) | part 4, the 200 ms replay within 1 ms p99; row "Prediction replay" | was 170–171; 531 | anchor right | HEAD 0001:171–172, :537 | |
| A30 | 0001:1167 (PM-0001-45) | part 7, the interpolation delay and its range | was 253–258 | anchor right, incomplete | NA-draft:253–258 holds the delay, its range and "A late snapshot is extrapolated for at most one interval, then held" (HEAD 0001:259), the sentence this row (extrapolation) rests on | nit: name the extrapolation limit too (Findings 7) |
| A31 | 0001:1168 (PM-0001-46) | part 7, adaptation, placed respawns, the announced delay | was 256–262 | anchor right | HEAD 0001:257–263 | |
| A32 | 0001:1169 (PM-0001-47) | part 5, *Time:* the clock estimate and the input buffer | was 196–199 | anchor right | HEAD 0001:197–200 | |
| A33 | 0001:1170 (PM-0001-48) | OQ-0001-7 | was 881–885 | anchor right | NA-draft:881 OQ-0001-7; HEAD 0001:894 | |
| A34 | 0001:1171 (PM-0001-49) | part 8, the rewind cap and the share it clips | was 273–280 | anchor right | HEAD 0001:275–282 | |
| A35 | 0001:1172 (PM-0001-50) | part 8, the view-time tolerance | was 270–273 | anchor right | HEAD 0001:272–274 | |
| A36 | 0001:1173 (PM-0001-51) | part 5, *The aim point:* | was 201–207 | anchor right | HEAD 0001:202 | |
| A37 | 0001:1174 (PM-0001-52) | part 4, "Never predicted" | was 174–175 | anchor right; quote exact | HEAD 0001:175 "Never predicted: damage, deaths and hits on others" | |
| A38 | 0001:1175 (PM-0001-53) | "What a spike must prove" item 4 | was 968–969 | anchor right | NA-draft:968; HEAD 0001:990 | |
| A39 | 0001:1177 (PM-0001-54) | item 5 (E3 or E3b) | was 986–987 | anchor right | HEAD 0001:1009 "E3 or E3b (0 desyncs of client-side flight over 1,000 throws," | |
| A40 | 0001:1179 (PM-0001-56) | item 5; OQ-0001-6 | was 985–993; 874–880 | anchor right | NA-draft:985, :874; HEAD 0001:1007, :887 | no line number into ADR 0001 is left in its pass marks (script: 0 "the draft … line N" left) |
| A41 | 0001:711, 718, 726, 734, 783; 0002:187, 397, 464 | a blank line under eight tables | GFM tables end at a blank line | confirmed | scripted over the nine docs at HEAD: 0 table rows directly followed by a text line (the one hit, 0001:425, is inside the ASCII diagram's code block) | |
| A42 | 0002:1045 (PM-0002-3) | the same number of Wasm calls per tick at N = 12 and 48, glue included, at most 4 (one instance), 8 (two) | R1-BND-REC1 mark 2 (r1, 2261–2263); part 5; item 1 | confirmed | r1:2262–2263 "the worker makes the same number of Wasm calls per tick at N = 12 and N = 48 (at most 4)"; part 5 (0002:279–284) "one input call, which carries the page's command and every datagram received … and one step call; the step … returns where they lie … wasm-bindgen's glue calls included"; item 1 (0002:922–926) the same, 4 and 8 | resolves fixcheck-b F2 |
| A43 | 0002:1045 | "the glue's allocator calls are exports of the instance, so the counting build counts them" | none | unsupported (no source in the doc); right as far as I know | wasm-bindgen's generated glue passes bytes through the module's exported allocator (`__wbindgen_malloc`); not fetched, as no claim rests on it beyond the counting method | optional: cite the generated `.js` glue or wasm-bindgen's guide (Findings 10) |
| A44 | 0002:1045 | bites: a per-soldier accessor; a no-op export called 5 times per tick | | confirmed (each half of the mark has a failing defect) | N-growth bite fails "the same number"; 5 > 4 fails the cap | |
| A45 | 0002:1071 (PM-0002-29) | this measurement and content's share (OQ-0005-CD2) are the two inputs to the Advisor's first limit; until then G7 records, does not gate | part 4; item 8; OQ-0005-CD2 | confirmed | 0002:237–241 "the Advisor sets the first limit in the budget file (part 9) from both measurements; until then G7 records the `sim-wasm` size and does not gate it"; item 8 (0002:959–962); 0005:593–605 | the row's own header still says "with the sim core spike, decides R1-RR-OQ5" (Findings 3) |
| A46 | 0002:1073 (PM-0002-31) | part 5, content files in development; SPIKE_PLAN item 3 gives this spike R1-DATA-OQ8 | part 5 quote; SPIKE_PLAN item 3 quote | confirmed; quotes exact | 0002:293–294 "whether Vite's hot update reaches the Worker directly is / PM-0002-31's to measure"; SPIKE_PLAN:156 (### 3. The boundary spike), :165 "ADR 0005's content files reaching the Worker (PM-0002-31, R1-DATA-OQ8)" | resolves fixcheck-b F6 |
| A47 | 0002:1122 | "Notes on two rows." (the PM-0002-3 note deleted) | | confirmed | two notes follow (PM-0002-4 and the next); the deleted note's question is answered by part 5 and the row | |
| A48 | 0003:700 (CL5, 114) | mark now drop (look brief pending); Mustafa still picks; variant = palette in faction colour slots at load (5.a), glow = emissive slot (5.d) | LI:114; LOOK_V2 §8 step 5 | confirmed | LI:114 "**drop** — No game caller since D6 … *Look brief pending.*", v2 note "the row recommends drop, and Mustafa picks"; LOOK_V2:66 "v2 may use authored 3D models", :72 "5. the engine applies the comic look"; 0003:168–176 (5.a), :200 (5.d) | |
| A49 | 0003:704 (CC10, 139) | port (pending); enum and rest pose folded into the animation port (part 6) | LI:139; LOOK_V2 §3, §2 | confirmed, one nit | LI:139 "Port the enum and fold the rest pose into the animation port"; LOOK_V2:28 "Slimmer, serious proportions", :20 "Small movement touches" | "soldiers become authored, skinned models" after LOOK_V2 §3's colon is ADR 0003's outcome (part 6, 0003:265–266), not LOOK_V2's words (Findings 9) |
| A50 | 0003:709 (GW5, 193) | port (pending); visuals on the page, label textures cached; bounds map data (CD-11) the sim enforces; pending → OQ-0003-15 | LI:193; 0005 CD-11 | confirmed | LI:193 "The arena bounds … belong in the map document, and the Rust sim must enforce them … v2 should cache label textures"; 0005:433 "the map's bounds (CD21's note)"; LOOK_V2 names no grid, border or labels | |
| A51 | 0003:715–716 (GE14 300, GE16 302, GP8 318) | marks port (pending) / redesign (pending); pending stays for the HUD's layout, which no draft or spike settles | LI:300, 302, 318 | confirmed | LI:300 and :302 "**port to TypeScript** … *Look brief pending.*"; LI:318 "**redesign** … *Look brief pending.*"; grep of the nine docs: no draft or spike sets a HUD layout | |
| A52 | 0003:717 (GE3, 289) | redesign (pending); money a sim phase (0004 part 17) under LOOP_V2 §2, §3, §8, which replace the terminal; this record draws the credit HUD and prompt (part 10) | LI:289; 0004 part 17 | confirmed | LI:289 "LOOP_V2 §2, §3 and §8 replace the player-only cash flow and the resupply terminal"; 0004:422 "Money and harvester deliveries … run as sim phases"; 0003:385 (part 10 (7), the HUD in the DOM) | |
| A53 | 0003:719 (KT5, 414) | redesign (look brief pending); only the A/B technique carries, backend on every shot (part 4; 0002 G12); pending stays for UI criteria; stale agent lines are KIT-ALIGN | LI:414; LOOK_V2 §9; STATUS KIT-ALIGN | confirmed | LI:414 "only the A/B-evidence technique carries over"; 0003:153–154 "every test screenshot prints it"; 0002:462 G12 "the backend string on every shot"; LOOK_V2:77–78; STATUS:188–190 lists `.claude/agents/art-director.md:16` and `:10` | |
| A54 | 0003:816; 0003:1113–1116 | OQ-0003-15 and checklist step 29 add the ground grid, the red arena border and the world text labels (GW5, line 193) | LI:193 | confirmed | LI:193 system "Ground grid, red arena border and world text labels"; README:71 OQ-0003-15 row (question unchanged) | |
| A55 | 0004:748 (GN2, 267) | redesign, pending stays for the head tint; sim/view split; bots buy T1 or respawn as Recruit; random pick from part 8's bot stream | LI:267; LOOP_V2 §5, §10; 0001 slot table; 0004 parts 1, 2, 8, 15 | confirmed | LI:267 "tints the head with the unit's accent", "view state (group, _aimT, _fOn)", "The head-tint rule goes to the look brief"; LOOP_V2:69–70 the Recruit, :133 "buy T1 units"; 0001:366 "the slot table marks each slot human, bot or empty"; 0004:262 "bot decisions" stream; 0004:398 "send the same input commands as humans" | |
| A56 | 0004:749 (GE4, 290) | redesign, pending stays for era HUD and toasts; tiers a sim phase; tier event, never a HUD write; the debt the row cites; x0.5 cut not built; GE4/GE5 question | LI:290, 291; 0004 parts 17, 19 | confirmed | LI:290 evidence "applyDamage runs addTech's HUD writes (ADR-001 debt)", "The x0.5 power-plant tech cut is not in LOOP_V2: decide whether it stays"; LI:291 "Whether a dead plant should also slow tech is a LOOP_V2 question for Mustafa (see GE4)"; 0004:426–427 "A tier reached … is an event (part 19), never a HUD write from the sim (legacy's `addTech`)" | |
| A57 | 0004:750 (GE9, 295) | redesign, pending lifted; uplinks a sim phase "(part 17: cache, carry, place, countdown, steal by GE8's presence count, rotation)" | LI:295; LOOP_V2 §4; 0004 part 17 | confirmed, one wording point | LOOP_V2:44 "Enemies steal the uplink by holding E for a few seconds."; LI:294 (GE8) "§4 step 4 says only that a steal takes holding E … The count itself maps onto uplink steal and defend"; 0004:425 "presence counting for steals (GE8)" | "steal by GE8's presence count" drops LOOP_V2's [DECIDED] hold-E (Findings 1); LI:295's lesson is about the hold-E channel, which the row maps onto carrying |
| A58 | 0004:751 (GE10, 296) | redesign, pending lifted; no silo rule reaches the sim; the safe near-base field takes "income never zero" | LI:296; LOOP_V2 §3, §4 | confirmed | LI:296 "the safe near-base field idea (§3) that takes over the 'income never zero' role"; LOOP_V2:29–30 "A small, safe field near each base gives a trickle"; LOOP_V2:47 `[DEFAULT]` "Uplinks replace silos and tech boxes" | |
| A59 | 0005:1088–1089 | A3 "adds a runtime no §6 item asks for" (r1, Options §5, Question A, line 1798) | r1:1798 | confirmed; quote exact, location exact | r1:1790 "### 5. A data-driven ability and effect system", :1792 "#### Question A", :1798 row A3, last cell "Serves §6.3 but adds a runtime no §6 item asks for" | resolves X2 |
| A60 | 0005:1783–1784 | the intro names ED5, ED7, GW6, GW7, GW8, GW10, GW11 and GE7 as the rows no draft had placed | the table | confirmed | the table's rows 1795–1801 | |
| A61 | 0005:1797 (GW6 194, GW8 196) | redesign (pending); split into the map document, colliders never from a model (AP-1), per-entity scene (EDITOR_V2 §2 rules 1, 3, 4); pending → OQ-0005-AP2, trees → LOOK_V2 §8 `[OPEN]` | LI:194, 196; LOOK_V2 §8 | confirmed, one wording point | LI:194 "**redesign** — EDITOR_V2 §2 rules 1 and 3 …"; LI:196 "rules 3-4"; EDITOR_V2:16, :20, :23; LOOK_V2:66, :74 "`[OPEN]` **The tree style.**"; 0005:1498–1505 OQ-0005-AP2 | "LOOK_V2 §8 … names none of these pieces" holds for §8; §2 names "logs and trees" and two houses (LOOK_V2:17, :21–22) (Findings 8) |
| A62 | 0005:1798 (GW7, 195) | port (pending); builders take (w, d, h0, h1, rot); slope the sim's (GW14); greybox height by brightness | LI:195; LOOP_V2 §9; LOOK_V2 §3 | confirmed | LI:195 "Keep the wedge geometry and the ramp parameters (w, d, h0, h1, rot) as the data model … height-as-brightness for the greybox is in LOOP_V2 §9"; LOOP_V2:124 "height read through brightness (higher is lighter)"; LOOK_V2:32 "verticality" | |
| A63 | 0005:1799 (GW10, 198) | redesign (pending); spawns team objectives (CD-11; EDITOR_V2 §3), barracks spawning the sim's; pending lifted (editor-only markers) | LI:198; EDITOR_V2 §3 | confirmed | LI:198 "EDITOR_V2 §3 'objectives as data' replaces them"; EDITOR_V2:37 "**Objectives as data:** spawns, barracks and terminals"; 0005:432 "objectives as data (EDITOR_V2 §3)"; 0005:435–437 the editor's scene is EDITOR_V2's own plan | |
| A64 | 0005:1800 (GW11, 199) | redesign (pending); mood map data (CD-11) lit by 0003 5.g; terminal mesh → OQ-0005-AP2 | LI:199; LOOK_V2 §6.5 | confirmed | LI:199 "terminal mesh :638-641, env :644"; LOOK_V2:49 "## 6. The Advisor's additions `[DEFAULT]`", :58 "different moods per map"; 0005:433 "a per-map lighting mood (LOOK_V2 §6.5, its values ADR 0003's)"; 0003:216–217 | |
| A65 | 0005:1801 (GE7, 293) | port (pending); glow lifted (emissive slots, AP-3), shapes → OQ-0005-AP2, colours ADR 0003's tuning | LI:293; LOOK_V2 §6.4, §5 | confirmed | LI:293 "the technique carries over to uplinks and harvesters", "Shapes and colours wait for the look brief"; LOOK_V2:54 "objectives, faction tech and muzzle flashes are emissive"; 0005:1392 AP-3 names the emissive class | |
| A66 | 0006:550; 0006:1040 | "The netcode proof test's measurement"; "re-run the netcode proof test there" | GLOSSARY §8 term | confirmed | 0006:765 the rooms-per-VM marks are "this record's operations marks inside the netcode proof test"; grep: no short "proof test" left in 0006 | |
| A67 | 0006:882, 884 (PM-0006-15, -17) | "the netcode proof test's client page", "… load", "… sim" | | confirmed | fixcheck-a L3 resolved | |

Fix files of rounds 3 and 4 (game-designer-fixes-3, infra-engineer-fixes-3, netcode-architect-fixes-3,
test-engineer-fixes-3 and -4, graphics-engineer-inventory-fixes-3, rust-reviewer-inventory-fixes-3,
engine-architect-0005-inventory-fixes-3 and -3b): 60 FIX blocks; 58 AFTER texts are in the docs at HEAD word for word;
the other 2 (0005 inventory fixes-3, the intro and the rows) were rewritten by fixes-3b, whose BEFORE is their AFTER.

### 1b. The synthesis round's new claims (80a27b3..47eadd9)

`E` = docs/ENGINE_OVERVIEW.md, `SP` = docs/SPIKE_PLAN.md, `RM` = docs/adr/README.md (all at 47eadd9).

| # | doc:line | claim (short) | cited | verdict | where and the words | note |
|---|---|---|---|---|---|---|
| B1 | E:14 | up to N = 48 players, the largest mode 24v24 | MODES-3 (a) | confirmed | STATUS:113 "the biggest match 24v24, 48 players (MODES-3)" | |
| B2 | E:21–49 | the diagram's §6 labels: §6.1 ROOM, CONTENT; §6.2 IN, SIM; §6.3 CONTENT, SIM; §6.4 SIM; §6.5 SIM; §6.6, §6.8, §6.10 SUP; §6.7 R, CONTENT; §6.8 SIM; §6.9 R | E's §6 table, VISION_V2 §6 | confirmed; all ten items placed | VISION_V2:64–81 items 1–10; each label matches a part the table at E:104–113 names | E:57 "Each part's label names the VISION_V2 §6 items it serves" overclaims: the table also gives §6.6 and §6.8 to ADR 0004 part 19 and ADR 0005 CD-12, §6.4 to ADR 0003 part 6 and ADR 0005 AB-6, which the SIM, CONTENT and R labels leave out (Findings 5) |
| B3 | E:37–45, 52–53 | per tick: command up, block down, commands and snapshots over the network; per frame: camera, interpolate and animate, sound, HUD feedback | ADR 0003 part 10 | confirmed, one nit | 0003:379–386 "(1) take the newest blocks, keeping two … (2) interpolate between the two newest ticks … (6) start sounds … (7) update the HUD in the DOM" | "draw the newest block" (E:40, :52): the page draws between the two newest blocks; the HUD also reads the block's data (credits, tracks), not only events (Findings 5) |
| B4 | E:61–62 | the event section holds ADR 0004 part 19's events "such as hits, deaths and kills, blasts, purchases, uplink cache events, tech tiers, upgrade picks and pickups, and moments" | 0004 part 19 | confirmed | 0004:454–462, the same words, plus the moment phase; "captures" gone | resolves F13's E part |
| B5 | E:73–75 | `devtools` any workspace crate; `navbake` `sim` and `content`; the editor uses `client` code and the `navbake` and `sim` Wasm; nothing imports `legacy/` or `spikes/` | 0002 part 3 | confirmed | 0002:185 `devtools` "anything"; :186 `navbake` "`sim`, `content`"; :199–200 "nothing imports `legacy/` or `spikes/`"; :549–550 "a Worker from the `navbake` and `sim` crates built to Wasm"; PM-0002-46 (0002:1089) "the editor entry importing `client` code passes" | resolves synthesis row 33 |
| B6 | E:107 | AB-6: leech = removed when its source dies + last-hit trigger; marking = an effect that grants a tag | 0005 AB-6 | confirmed | 0005:884 "an effect removed when its source dies, and a trigger on the last hit or tick that moves the victim's item to the source"; :885 "an effect that grants a tag" | resolves synthesis row 37 |
| B7 | SP:30–33 | MAC: the stable Safari on the day, version recorded; WebGPU from 26.0 (R3-WGPU-F19, r3 171–174); WebTransport from 26.4 (R2-TRN-F7, r2 959–975) | r3, r2 | confirmed | r3:171 "WebGPU "is now shipping in Safari 26.0 for macOS …""; r2:959 **R2-TRN-F7**, table row r2:966 "Safari (macOS; iOS "mirror") \| 26.4 \| 2026-03-24 \| 26.4 \| 27" | resolves F4's SP part |
| B8 | SP:75, 98–99, 111–114, 125–126, 130 (item 1) | PM-0001-1 to -57; the `wss` leg on OQ-0006-5's certificate (PM-0006-15); depends on OQ-0006-5's development answer; R1-TICK-REC4 as ADR 0004's S17, PM-0004-67, "as ADR 0001's spike text says"; decides R5-TEST-OQ2 | 0001; 0006 | confirmed | 0001 PM rows 1–57 (script); 0006:655–668 OQ-0006-5 "the development and staging case before the netcode proof test starts (the Advisor, from a short read)"; 0001:1215–1216 "(R1-TICK-REC4) are ADR 0004's row S17, run on this spike's server"; 0001:1008 "(R5-TEST-OQ2)" | resolves F8, F12/drift, F14 |
| B9 | SP:78, 196, 211–213 (item 4) | PM-0004-1 to -61, -73, -74; R2-SYNC-REC6 mark 1 is S21, PM-0004-73; R5-LAYOUT-REC4 is S22, PM-0004-74 | 0004 | confirmed | 0004:846–847 S21, S22; :1010–1011 PM-0004-73 "(S21; … the id is ADR 0001's)", PM-0004-74 "(S22; … the id is ADR 0002's)" | resolves F7 |
| B10 | SP:191–192 | "seven targets (native, Node, three browser engines, real Safari on MAC, and Chrome and Firefox on the minimum laptop's Windows: PM-0004-19)" | PM-0004-19 | confirmed in substance; count reads off | 0004:967 "between two native processes, and between native and Wasm in Node, in Chromium, Firefox and WebKit …, and in real Safari \| … ML once (Chrome and Firefox on Windows …)" | the list names eight runtimes if Chrome and Firefox count apart (Findings 6) |
| B11 | SP:181–183 | it reports the copy time for R1-RR-OQ6 (PM-0002-10); ADR 0002 answers R1-RR-OQ6, proposed, for rust-reviewer to confirm | 0002 App. A | confirmed | 0002:1327 "R1-RR-OQ6 \| answered (proposed; rust-reviewer, its named decider, confirms at review)"; 0002:1052 PM-0002-10 "the Worker's copy time" | the same line keeps "R1-RR-OQ5 (with the sim core spike)", which part 4 now gives to the Advisor with content's share (Findings 3) |
| B12 | SP:232–233 | OQ-0003-6 (the roof fade) is the test's to decide, with his look among what decides it | 0003 OQ-0003-6 | confirmed | 0003:762–763 "Decider: the look proof test. What decides it: … and Mustafa's look." | resolves synthesis finding 8 |
| B13 | SP:282 | "Mustafa's client-host token (ADR 0006 part 1)" | 0006 part 1 | citation off | part 1 (0006:123–143) names the client host but no token; the deploy token is item B9 of part 13 (0006:385, :392) and the preview hosting check (0006:744) | Findings 4 |
| B14 | SP:163, 338, 362, 367, 377, 383 | no "hot reload"; gap G1 and LE-N in README; the full name; "As the drafts keep them" | RM | confirmed | RM:310 gap G1 = the asset pipeline; RM:234 LE-N "the Feel Lab test needs abilities in the engine" | resolves F12's SP part |
| B15 | RM:10–11 | line numbers are the drafts' at 9a03999 | git | confirmed | `git diff 9a03999 47eadd9 -- docs/adr/000*.md` is empty, so they are HEAD's too | |
| B16 | RM:16–24 | OQ per draft 14, 8, 15, 7 numbered (5 open), 34, 12; total 90 numbered, 88 open; pass marks 57, 76, 43, 74, 46, 31 (327) | the drafts | confirmed by script | `.gate/fixcheck-c/counts.py`: OQ headings 14/8/15/7/34/12 (OQ-0004-5, -6 read "Moved: … OQ-0005-AB9", "AB10", 0004:786, :791); PM rows 1..max with no gap: 57/76/43/74/46/31, sum 327; 0005's 34 = 8 + 13 + 3 + 6 + 4 | |
| B17 | RM:34–106 | the decisions table re-pinned | the drafts | confirmed | draft_pointers.py "pointers checked 69: ok 69, off 0"; the 70th pointer (RM:38, ASSET-RESEARCH, not OQ-keyed) lands on 0005:76 "**ASSET-RESEARCH (a)**" | |
| B18 | RM rows OQ-0001-14, OQ-0002-8, OQ-0006-5, OQ-0003-11, OQ-0005-AB5, -AB9, -AP2, -CD2, -1, -2 | new or changed questions and deciders | each draft's OQ | confirmed | each README decider equals the draft's "Decider" text (script, 11 rows); OQ-0003-11's question equals 0003:789 | OQ-0004-5 and -6 rows dropped (F2) |
| B19 | RM:48 | Fragments as damage: Mustafa with the game-designer; ADR 0004's closing paragraph; ADR 0003 5.k, OQ-0003-7, step 24 | 0004; 0003 | confirmed | 0004:809–811 "listed once in `docs/adr/README.md`'s "Decisions the drafts ask for": whether grenade fragments deal damage per fragment … Mustafa with the game-designer; no research id"; 0003:247–248 (5.k), :1102–1103 (step 24) | makes fixcheck-b F5 and fixcheck-a M1 true |
| B20 | RM:49, Design questions | 38 DESIGN ids | | confirmed by script | 38 rows, 38 unique ids, 38 "DESIGN" dispositions | |
| B21 | RM:153, 211–217 | STATUS cited by item: "The five carry-forwards into the decision drafts"; "ADR-001..003 below"; the three ADR entries; SEC-MDL-2; the CDN-tag item; the wall-clock item | STATUS | confirmed | STATUS:167, :246, :249, :250, :252, :314, :319, :330 | resolves synthesis finding 2, F10 |
| B22 | RM:169 | SEC-SPREAD (a): "server-only spread" (the spread is known only to the authority) | STATUS | confirmed | STATUS:151 ""server-only spread": the spread stays known only to the server"; 0004 part 8's authority-only stream (in local play the Worker is the server) | a reading, not a reopening |
| B23 | RM:228 (LE-H) | the 18 rows (CL5, CC10, GW5–GW8, GW10, GW11, GN2, GE3, GE4, GE7, GE9, GE10, GE14, GE16, GP8, KT5) are now in the three tables | 0003, 0004, 0005 | confirmed | 18 ids counted; rows A48–A65 above | |
| B24 | RM:242, 247, 254 | R2-BUD-REC4 §4 only; roof fade (5.i) "beside §6.2"; three damage stages LOOK_V2 §4 only, structure states §6.2, §6.4, §6.5 | 0001 App. A; 0003 5.i; 0004 part 14 | confirmed | 0001:1384 "need: VISION_V2 §4 only"; 0003:234 "*Need:* VISION_V2 §6.2; LOOK_V2 §4"; 0004:395–396 | resolves synthesis findings 3, 4 |
| B25 | RM:267–270 | plan rules, steps and conditions: the appendix §3–§4 quotes them; local families: 0002 gate checks G1–G13, 0004 claims S1–S22, 0006 option labels G1–G4, S1–S3, K1–K4, B1–B11 | appendix; drafts | partly wrong | appendix:149 §3 (the advisor's review, word for word), :265 §4 (plan revision 3: rules, steps); 0004:846–847 S21, S22; but 0002:450 has a gate check **G0** ("\| G0 \| setup \|"), and 0006:503–506 has record-storage options **S1–S5** ("S4. A SQL database", "S5. A hosted analytics warehouse"), so S4 meets seam S4 unexplained; B1–B11 are part 13's baseline items, not options | Findings 2 |
| B26 | RM:297, 304 | seams "not ADR 0004's own claim numbers S1–S22"; S11 Offline and local play | 0004; RM design rows | confirmed | 0004:854 "(S1–S22 above)"; RM design rows R1-BND-OQ1, R1-TICK-OQ7, R1-XT-OQ8 (critic rank 15) | resolves synthesis finding 7 |
| B27 | RM:286, 314 | "server tick time histogram"; "the visual-effects budget row" | GLOSSARY; 0003 | confirmed | 0003:432 "R3-BUDGET-REC2's visual-effects row" | |
| B28 | E:1–139 | the page stays within about 150 lines | condition 19 | confirmed | `wc -l` 139 (was 133 at 80a27b3) | |

## 2. Did each block resolve its finding, and does it add anything wrong?

Rounds 3 and 4 (the drafts):

| block | finding | resolved? | new problem |
|---|---|---|---|
| game-designer X2 | verifier-adr0005 X2 (a paraphrase shown as a quote) | yes: the words are r1:1798's, with section, question and line | none |
| infra-engineer I3-1, I3-2 | consistency F13, completeness item 8 (short "proof test" in 0006) | yes: 0 short forms left in 0006 | none |
| netcode-architect FIX 1–3 | completeness item 14 (the stack cited by "brief §1" alone) | yes (A1–A4) | none (FIX 1's 134-character line is layout only) |
| aa85bf0 blank lines (netcode NOTE C) | GFM: a line under a table renders as a row | yes: 8 added (0001 ×5, 0002 ×3); 0 table-then-text lines left in the nine docs | none |
| test-engineer F-2 (PM-0002-3 and the note) | fixcheck-b F2 | yes (A42, A47) | the allocator clause is uncited (A43, nit) |
| test-engineer F11 (PM-0002-29) | fixcheck-b F4 | partly: the body is right (A45); the header still says "with the sim core spike, decides R1-RR-OQ5" | Findings 3 |
| test-engineer F12 (PM-0002-31) | fixcheck-b F6 | yes (A46) | none |
| test-engineer F13 (PM-0006-15, -17) | fixcheck-a L3 | yes (A67) | none |
| test-engineer fixes-4 (36 pointer blocks: the wrap and 35 rows) | netcode NOTE B (pointers into 0001 by stale line) | yes: each old line resolved in NA-draft names the row or part the new anchor names (A5–A40); 0 line pointers into 0001 left in its pass marks | PM-0001-45's anchor leaves out the extrapolation sentence (nit, Findings 7) |
| test-engineer fixes-4, step 29 | graphics-engineer NOTE GW5 | yes (A54) | none |
| graphics-engineer inventory 3 (6 blocks) | completeness item 5 (CL5, CC10, GW5, GE3, GE14, GE16, GP8, KT5) | yes: 8 rows, ids, lines, marks and pending notes match LI (A48–A54) | CC10's Why puts ADR 0003's outcome after LOOK_V2 §3's colon (nit, Findings 9) |
| rust-reviewer inventory 3 (1 block, 4 rows) | completeness item 5 (GN2, GE4, GE9, GE10) | yes (A55–A58) | GE9: "steal by GE8's presence count" (Findings 1) |
| engine-architect 0005 inventory 3 and 3b | completeness item 5 (GW6, GW7, GW8, GW10, GW11, GE7) | yes (A60–A65); 3b supersedes 3's two blocks | GW6/GW8's Why scopes "names none of these pieces" to §8 only (nit, Findings 8) |

The synthesis round (engine-architect-synthesis-fixes-2.md: 53 FIX, 4 NOTE). Applied in order to 80a27b3's three files
(the same as 9a03999's) with its own apply.py: "applied 53", and the result equals 47eadd9 byte for byte.

| block | finding | resolved? | new problem |
|---|---|---|---|
| 1 G-OVW-14 | completeness 11 (glossary: "48" alone) | yes (B1) | none |
| 2 completeness-15 (diagram) | completeness 15, brief §3 (per-frame flow, §6 placement) | yes: four per-frame edges, every edge marked per tick or per frame, all ten §6 items on parts (B2, B3) | label "draw the newest block" (nit, Findings 5) |
| 3 completeness-15 (words) | same | yes | E:56–57 "Each part's label names the VISION_V2 §6 items it serves" overclaims (Findings 5) |
| 4 F13 | consistency F13 ("captures") | yes (B4) | none |
| 5 row 33 | synthesis finding 5, skeptic-3 row 33 | yes, skeptic-3's text (B5) | none |
| 6 row 37 | synthesis finding 1, skeptic-3 row 37 | yes, skeptic-3's text (B6) | none |
| 7 G-SPK-15 | completeness 11 | yes | none |
| 8 F4 (MAC) | consistency F4 | yes; both sources confirmed (B7) | none |
| 9 J6-order | consistency F12 | yes: RM:345 J6, item 5 is "only the netcode proof test's place settled" | none |
| 10, 13 addendum (PM-0001-57) | addendum | yes (B8) | none |
| 11, 21 F7 (table, item 4) | consistency F7 | yes (B9) | none |
| 12 F8 (proves) | consistency F8 | yes (B8) | none |
| 14 F8, G-SPK-106 | consistency F8; completeness 11 | yes (B8) | none |
| 15 F12/drift | synthesis finding 6 | yes: 0001:1216 now names R1-TICK-REC4 | none |
| 16 F14 | consistency F14 (R5-TEST-OQ2) | yes: 0001:1008 | none |
| 17 F1 | consistency F1 (SP "hot reload into the Worker") | yes: SP:163 "size and tools"; SP:165 keeps PM-0002-31 | none |
| 18, 22, 25, 27 F4 (MAC, items 3, 4, 5, 7) | consistency F4 | yes: "MAC" alone, defined once at SP:30–33 | none |
| 19 addendum (a) | addendum (a) (R1-RR-OQ6) | yes (B11) | the same line keeps "R1-RR-OQ5 (with the sim core spike)" (old text; Findings 3) |
| 20 finding 9 | synthesis finding 9 | yes (B10) | "seven" against eight runtimes listed (nit, Findings 6) |
| 23 F7 (placed here) | consistency F7 | yes (B9) | none |
| 24 finding 8 | synthesis finding 8 | yes (B12) | none |
| 26 G-SPK-274 | completeness 11 ("Cloudflare" alone) | yes for the glossary point | "(ADR 0006 part 1)": part 1 names no token (Findings 4) |
| 28 G-SPK-310 | completeness 11 ("characters") | yes | none |
| 29, 34 F12 (part g) | consistency F12 | yes: no "part g" left in E, SP, RM | none |
| 30, 31 F12 (part h) | consistency F12 | yes (B14) | none (RM:307's header still offers "part h, G-n" as a citation form; old text) |
| 32 F12 (LE-N) | consistency F12 | yes | none |
| 33, 44 F13 (full name) | consistency F13 | yes: SP:377, RM LE-K | none |
| 35 F9 (what the line numbers are) | consistency F9 | yes (B15) | none |
| 36 drift (a), addendum (counts) | synthesis finding 6 (a) | yes (B16) | none |
| 37 the decisions table | F2, F8, F9, F14, drift (a)–(c), addendum, fixcheck-a M2 | yes: off 0 of 69; OQ-0004-5/-6 rows gone; OQ-0001-14, OQ-0002-8, OQ-0006-5 and the fragments row added; OQ-0003-11's question and 10 deciders follow the drafts (B17–B19) | none |
| 38, 51 finding 7, F12 (S11) | synthesis finding 7 | yes: "seam S11" and its row (B26) | none |
| 39, 42, 53 F10, row 53 (STATUS by item) | synthesis finding 2, consistency F10 | yes (B21) | none |
| 40, 41, 49, 52 glossary (README) | completeness 11 | yes (B22, B27) | none |
| 43 addendum (LE-H) | completeness 5's README side | yes (B23) | none |
| 45, 46, 47 row 54 | synthesis findings 3, 4; skeptic-3 row 54 | yes (B24) | none |
| 48 F12, completeness-4 (codes line) | consistency F12, completeness 4 | yes for the plan pointer (appendix §3–§4 hold them) | the family list is wrong in two places (Findings 2) |
| 50 F12 (S1–S22) | consistency F12 | yes | none |
| NOTE F1, row 54, completeness-15, counts | (notes) | consistent with the doc | none |

Earlier units' items that this round had to make true: fixcheck-b F5 and fixcheck-a M1 (ADR 0004:809 and ADR 0003's
pointers to "fragments as damage") are now true (B19); fixcheck-b F2 and F6 resolved, F4 half resolved (Findings 3);
fixcheck-a L3 resolved.

## 3. ADR 0002's citation-script flags (cite_check.py: 643 ids, 0 missing, 86 groups, 38 range flags)

Every flag was re-read pair by pair (`.gate/fixcheck-c/cite_explain.py`, output `cite0002_explained.txt`): each id
against the range written next to it, in the file it belongs to.

| flags | 0002 line | the group | verdict | where the lines belong, and the id |
|---|---|---|---|---|
| 1–2 | 60 | R1-BND-F2, lines 894–902; R1-BND-REC4, lines 2297–2308 (r1) | script artifact (two pairs in one group, each id tried against both ranges) | r1:894 "**R1-BND-F2.**"; r1:2297 "**R1-BND-REC4.**" |
| 3–4 | 63 | R1-BND-F25, 1073–1077; R1-BND-F29, 1098–1103 (r1) | script artifact (pairs) | r1:1073, r1:1098 |
| 5–6 | 65 | R1-BND-F12, 964–970; R1-BND-F22, 1047–1052 (r1) | script artifact (pairs) | r1:964, r1:1047 |
| 7 | 110 | "row TL11, line 396; r5, R5-TEST-REC3, lines 1444–1445" | script artifact (line 396 is LEGACY_INVENTORY's) | LI:396 row TL11; r5:1444–1445 is R5-TEST-REC3's Reason ("the oracle already runs the real code and produces reproducible dumps (TL11; the tuning dump was byte-identical …"), its id at r5:1431, 13 lines up |
| 8–20 | 169 | r5 ranges with no id; r1: R1-BND-F1, 886–893; R1-BND-REC4, 2297–2308; R1-BND-OQ7, 2741–2744 | script artifact (the R1 ids tried against the r5 ranges; "2741–2744 out of range" is r1's range read against r5's 2,504 lines) | r1:886, r1:2297, r1:2741 |
| 21 | 173 | "r5, line 1532; R1-DET-REC6 serves §6.6 and §3, r1 lines 2236–2237" | script artifact (the R1 id tried against r5) | r1:2236–2237 is R1-DET-REC6's "Serves" line, its id at r1:2222 |
| 22–24 | 209 | R1-BND-F19, r1 1019–1025; R2-TRN-REC4, r2 2673–2698 | script artifact (pairs across two files) | r1:1019; r2:2673 |
| 25 | 222 | r5 ranges; "r1-simulation-core.md lines 3109–3112"; r4 1195–1200, 1211–1214 | script artifact (an r1 range read against r5) | r1:3109 "**R1-XT-OQ18.** … Decided by: ADR-0002 (crate list)", an id the same Ids line names (0002:221) |
| 26 | 245 | r5 ranges; "r1, Open questions, lines 2941–2950" | script artifact (r1 range read against r5) | r1:2941 R1-RR-OQ4, r1:2946 R1-RR-OQ5, both named on 0002:245 |
| 27–31 | 311 | r1 and r5 ranges; "r3-rendering.md, R3-MEASURE-OQ4, lines 1389–1391" | script artifact (the r3 id tried against the r1 and r5 ranges) | r3:1389 "**R3-MEASURE-OQ4.**" |
| 32 | 437 | r5 ranges; "r1-simulation-core.md lines 3097–3100" | script artifact | r1:3097 "**R1-XT-OQ15.** … Decided by: ADR-0002 (bench tool and method)", named on 0002:437 |
| 33–35 | 514 | r5 ranges; "r6-infrastructure.md, R6-XT-OQ8, lines 1030–1034" | script artifact | r6:1030 "**R6-XT-OQ8.**" |
| 36 | 541 | r5 ranges; "r2-netcode.md lines 3384–3389"; r6 1009–1014 | script artifact (r2 range read against r5) | r2:3384 "**R2-XT-OQ10.** … Decided by: ADR-0002 (preview workflow)", named on 0002:540 |
| 37–38 | 902 | R4-XT-OQ5, r4 1195–1200; R5-XT-OQ6, r5 1996–2001 | script artifact (pairs across two files) | r4:1195; r5:1996 |

Real: 0. Script artifacts: 38.

## 4. The rules, on the changed text (plan rules 1–12; advisor-verdicts §1)

- A number with no source: none in the drafts' changed lines; in the synthesis, the counts were recomputed (B16, B20,
  B23), the Safari versions cite R3-WGPU-F19 and R2-TRN-F7 (B7); the code ranges "G1–G13" and "S1–S3" are wrong (Findings 2).
- An option labelled "not researched" proposed as the decision: none.
- A feature with no trace: none added (the inventory rows map legacy rows onto parts that already trace; the synthesis
  adds R2-BUD-REC4 to the register, as ADR 0001 asks).
- A GLOSSARY "Retire (exact)" phrase: glossary_check_drafts.py at HEAD: exact hits 0 (in context: 0001 129, 0002 53,
  0003 64, 0004 90, 0005 76, 0006 81, E 11, SP 15, RM 13). Of the 39 synthesis hits, 14 fall on changed lines (E:14 ×2,
  E:107 ×2, SP:15 ×2, SP:130, SP:318, RM:68, RM:105, RM:158 ×2, RM:212, RM:270): all correct in
  context ("players" counts people or N = 48's total; "48" is "N = 48"; "vehicles" is VISION_V2 §6.4's or ADR 0004
  part 14's word, not a stress count; RM:270's "always" is not the isolation flag).
- Anything decided about the look beyond LOOK_V2: none. CL5's and GE7's lifted notes rest on LOOK_V2 §8 step 5 and
  §6.4 `[DEFAULT]`; shapes, colours, trees, the HUD's layout and the three GW5 visuals stay Mustafa's (OQ-0005-AP2,
  LOOK_V2 §8 `[OPEN]`, OQ-0003-15).
- A settled input reopened: none (RM:169's "known only to the authority" glosses STATUS's "known only to the server";
  B22).

## Counts
- Claims, rounds 3 and 4 (A1–A67): 66 confirmed (4 of them with a wording note: A30, A49, A57, A61), 1 unsupported
  (A43), 0 wrong, 0 citation off. ADR 0001's anchors: 35 old pointers (36 blocks with the wrap) resolved in NA-draft,
  35 of 35 name the row or part the old line meant. Inventory rows: 18 of 18 with id, line, mark and pending note
  matching `docs/LEGACY_INVENTORY.md`; 14 LOOK_V2 section citations, 14 match `docs/LOOK_V2.md`.
- Claims, synthesis (B1–B28): 25 confirmed (3 with a nit: B2, B3, B10), 1 partly wrong (B25), 1 citation off (B13),
  plus B11's old neighbour (Findings 3).
- Blocks: rounds 3 and 4, 13 groups covering 60 FIX: 12 resolved, 1 partly (PM-0002-29's header); synthesis, 53 FIX
  and 4 NOTE: 53 resolved, 5 with a small new problem (blocks 2, 3, 20, 26, 48). The synthesis fix file replays to
  47eadd9 byte for byte.
- The README's counts against the drafts at HEAD, by script: open questions 14, 8, 15, 7 (5 open), 34, 12 = 90
  numbered, 88 open; pass marks 57, 76, 43, 74, 46, 31 = 327; design questions 38. All as stated.
- draft_pointers.py: "pointers checked 69: ok 69, off 0"; the one README pointer the script skips (RM:38) is right;
  ENGINE_OVERVIEW and SPIKE_PLAN have no line pointers into the drafts; 19 sampled pass-mark ids in SPIKE_PLAN and 10
  part pointers in ENGINE_OVERVIEW land on what they name.
- ENGINE_OVERVIEW: 139 lines (condition 19 holds); the diagram has the per-tick and per-frame flow and all ten §6 items.
- ADR 0002 citation flags: 38, all script artifacts, 0 real.
- Glossary: 0 exact hits; 14 in-context hits on the synthesis' changed lines, all correct.

## Findings (ranked; none blocks the drafts)
1. **ADR 0004's GE9 row writes the uplink steal as a presence count (low; wording against a `[DECIDED]` rule).**
   Author: rust-reviewer. 0004:750: "a sim phase (part 17: cache, carry, place, countdown, steal by GE8's presence
   count, rotation)". LOOP_V2 §4 step 4 (`[DECIDED]`, LOOP_V2:44): "Enemies steal the uplink by holding E for a few
   seconds."; GE8's own row (LI:294): "§4 step 4 says only that a steal takes holding E … The count itself maps onto
   uplink steal and defend". The same row maps LI:295's lesson ("the channel blocks weapons only for the player", the
   hold-E channel) onto carrying only. Smallest fix: "steal by holding E (LOOP_V2 §4 step 4), counted with GE8's
   presence count" and "… "can't shoot while carrying", and whatever the steal channel blocks, binds bots as it binds
   humans".
2. **README's list of draft-local code families is wrong in two places (low).** Author: engine-architect (synthesis).
   RM:269–270: "ADR 0002's gate checks G1–G13, ADR 0004's spike claims S1–S22 and ADR 0006's option labels (G1–G4,
   S1–S3, K1–K4, B1–B11)". ADR 0002 has a gate check G0 (0002:450, "| G0 | setup |"); ADR 0006's record-storage options
   run S1–S5 (0006:503–506, "S4. A SQL database", "S5. A hosted analytics warehouse"), so its S4 meets seam S4
   unexplained; B1–B11 are part 13's baseline items (0006:377–387), not options. Smallest fix: "ADR 0002's gate checks
   G0–G13, ADR 0004's spike claims S1–S22, ADR 0006's option labels (G1–G4, K1–K4, S1–S5, among others) and its
   baseline items B1–B11".
3. **R1-RR-OQ5 is still "decided" by the spikes in two places (low; fixcheck-b F4's other half).** Authors:
   test-engineer (PM-0002-29's header), engine-architect (synthesis, SP:181). 0002:1071: "(part 4; with the sim core
   spike, decides R1-RR-OQ5; feeds R5-CI-OQ4)"; SP:181: "R1-RR-OQ5 (with the sim core spike)". ADR 0002 part 4
   (0002:237–241) and App. A (0002:1326): the spikes and the content and ability spike measure; "the Advisor sets the
   first limit from both". Smallest fix: "(part 4; with the sim core spike, an input to R1-RR-OQ5, which the Advisor
   decides; feeds R5-CI-OQ4)" and SP:181 "R1-RR-OQ5 (measured with the sim core spike; the Advisor sets the limit
   after the content and ability spike, ADR 0002 part 4)".
4. **SPIKE_PLAN cites ADR 0006 part 1 for a token part 1 does not name (low; citation off).** Author: engine-architect
   (synthesis). SP:282: "Mustafa's client-host token (ADR 0006 part 1)". Part 1 (0006:123–143) has no token; the deploy
   token is part 13's item B9 (0006:385, :392, "an API token with the scope the research read") and the preview
   hosting check (0006:744, "Mustafa creates the project and adds its token"). Smallest fix: "(ADR 0006 part 13, item
   B9)".
5. **ENGINE_OVERVIEW's new words say a little more than the diagram shows (nit).** Author: engine-architect (synthesis).
   E:56–57: "Each part's label names the VISION_V2 §6 items it serves"; the table (E:105–111) also gives §6.4 to ADR 0003
   part 6 and ADR 0005 AB-6, §6.6 to ADR 0004 part 19 and ADR 0005 CD-12, §6.8 to ADR 0005 CD-12, which the R, SIM and
   CONTENT labels leave out. E:40 and :52: "draw the newest block"; ADR 0003 part 10 (0003:379–381) interpolates
   "between the two newest ticks". Smallest fix: "Each part's label names the main §6 items it serves; the table below
   gives every draft and part"; "interpolate between the two newest blocks, animate, draw".
6. **"Seven targets" lists eight (nit).** Author: engine-architect (synthesis). SP:191–192: "seven targets (native,
   Node, three browser engines, real Safari on MAC, and Chrome and Firefox on the minimum laptop's Windows:
   PM-0004-19)": 1 + 1 + 3 + 1 + 2. Smallest fix: "… and Windows on the minimum laptop, in Chrome and Firefox".
7. **PM-0001-45's new anchor leaves out the sentence the row rests on (nit).** Author: test-engineer. 0001:1167: "the
   draft, part 7 (the interpolation delay and its range)"; the old range (NA-draft:253–258) held "A late snapshot is
   extrapolated for at most one interval, then held" (0001:259), the row's subject. Smallest fix: "part 7 (the
   interpolation delay, its range and the one-interval extrapolation)".
8. **ADR 0005's GW6/GW8 reason scopes "names none of these pieces" to §8 only (nit).** Author: engine-architect.
   0005:1797: "LOOK_V2 §8 allows authored models without requiring them and names none of these pieces". True of §8;
   LOOK_V2 §2 names "Props: logs and trees with variety" and two houses (LOOK_V2:17, :21–22). The outcome (pending stays
   as OQ-0005-AP2) is right. Smallest fix: add "(§2 sets a direction for houses and props, not how they are built)".
9. **ADR 0003's CC10 reason reads ADR 0003's outcome as LOOK_V2's words (nit).** Author: graphics-engineer. 0003:704:
   "LOOK_V2 §3 (slimmer, serious proportions: soldiers become authored, skinned models, as for CL10, CC1–CC5)". LOOK_V2:28
   says "Slimmer, serious proportions"; the authored, skinned soldier is part 6's proposal (0003:265–266). Smallest fix:
   "LOOK_V2 §3 (slimmer, serious proportions), so soldiers become authored, skinned models (part 6, as for CL10,
   CC1–CC5)".
10. **PM-0002-3's reason for counting glue calls has no source (nit).** Author: test-engineer. 0002:1045: "(the glue's
    allocator calls are exports of the instance, so the counting build counts them)". No research line says it; it
    matches wasm-bindgen's generated glue as I know it (not fetched). Smallest fix: none needed for the mark; optionally
    "(proposed: wasm-bindgen's glue passes bytes through the instance's exported allocator)".

## Fetch audit
No page was fetched: every claim in scope could be judged from the repository (research topic files at 9e7db28, the
drafts, STATUS, LOOK_V2, LOOP_V2, EDITOR_V2, VISION_V2, LEGACY_INVENTORY, the report's appendix).

## Not verified
- How GitHub renders the new Mermaid diagram (self-loop edge, `§` and `·` in quoted labels): no Mermaid renderer in this
  container; the syntax is standard Mermaid flowchart syntax.
- A43 (wasm-bindgen's exported allocator) at its source: not fetched, as no mark rests on it.
- The gate (`node tools/gate.mjs`) was not run: this unit is docs-only fix verification, and the earlier units and CI
  cover the gate; no source file changed in 57c43c2..47eadd9 (`git diff --stat` lists docs and checkpoint files only).
