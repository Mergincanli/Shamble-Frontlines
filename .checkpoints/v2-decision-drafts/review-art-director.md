# Art-director review: the v2 decision drafts (plan step 10, wave 2)

Reviewed at commit 47eadd9 on claude/laughing-lovelace-ox61se; the docs in scope are unchanged at HEAD
(`git diff 47eadd9 HEAD -- docs/adr docs/SPIKE_PLAN.md docs/LOOK_V2.md docs/research` is empty). Judged against
docs/LOOK_V2.md (Mustafa's look brief), not the legacy ART_DIRECTION.md. Nothing renders yet: text only.
Scope: ADR 0003 (all of it), ADR 0005's asset pipeline and its GPU rules, and SPIKE_PLAN §5 (the look proof test).

## Verdicts
| Document (scope) | Verdict | In one line |
|---|---|---|
| ADR 0003, rendering | **REVISE** | Every LOOK_V2 item has a technique and a checklist step, the tree style stays open, and the ink proposal follows rule 6 [C9]. Two text fixes are needed before Mustafa can judge the look: (1) the tier data may switch a `[DECIDED]` item off on Low; (2) the checklist never shows him the Low frame. Findings 1–2. |
| ADR 0005, the asset pipeline and its GPU rules | **APPROVE WITH NOTES** | It takes LOOK_V2 §8's flow as given, decides no look and leaves trees open. It needs a material class for buildings (finding 6) and one building more in the test assets (finding 7). |
| SPIKE_PLAN §5, the look proof test | **APPROVE WITH NOTES** | Its place, owner and dependencies are right. It needs the second house (finding 7), a word on the `[DEFAULT]` items (finding 13), and legacy views taken in Brave (finding 8). |

What holds, checked item by item:
- **Every LOOK_V2 item is traced.** Each `[DECIDED]` and `[DEFAULT]` item of §1–§8 has a row in 0003's trace table
  (0003:412–451) and a step in the look checklist (0003:1056–1119). I mapped all 38 rows (37 items plus the tree style); none is missing.
- **The `[OPEN]` tree style stays open:** 0003:451, OQ-0003-9 at 0003:776–782, 0005:1594–1595, and families
  checklist step 4 at 0005:2146.
- **Rule 6 [C9] is followed for the ink.** One family is proposed on cost (0003:181–184). The visible difference is
  named and left to Mustafa (0003:187–192), and the strongest argument against is stated honestly (0003:632–637).
- **No colour output is proposed.** None was researched (0003:206–211, OQ-0003-3).
- **Visible values are left to tuning:** line widths, ramps, gains, AO and glow (0003:220–223).
- **Rules LOOK_V2 states are built as stated, and nothing more.** Fragments are drawn only, and their damage is left
  to Mustafa (0003:247–249, step 24). Planks and debris as routes are ADR 0004's (0003:238–241).
- **No feature "for later" in my domain.** GPU particles come only as an F-B extra with a measured gain (0003:245).
  HRTF is a measured option (0003:364). OffscreenCanvas is not proposed (0003:389–390). Kill-cams and replays are
  kept possible, not built (0003:31–32).

## Findings (ranked)
| # | severity | doc:line | the exact text (short) | the problem, with its evidence | the smallest fix | the text's author |
|---|---|---|---|---|---|---|
| 1 | high | 0003:1195; also 0003:163–164, 212, 309, 191, 637, 1189–1190, 665 | "Any technique misses on Low: a tier change in data moves it to High only, with no code change"; "every effect has a quality-tier switch (off, or a lower resolution scale)"; shadow "1024² or off"; "or the hull a High-only setting"; "AO off on Low" | Low is the default on every integrated adapter and every WebGL 2 player (0003:333–334), which covers the whole minimum class. The text lets bold ink (LOOK_V2 §1 `[DECIDED]`, §2 "thick black outlines") and soft shadows (§2 `[DECIDED]`) drop out of Low through a data change, with no look call by Mustafa. Plan rule 6 makes line widths and the like tuning. Whether a `[DECIDED]` item exists at all on the minimum machine is not a value: it decides the look beyond LOOK_V2. Shadows also carry height in a top-down game with floors and ledges (VISION_V2 §6.2): without them, a unit on an upper floor and one on the ground look alike from above. | Add a look floor to part 7. The techniques that carry a `[DECIDED]` item are the toon materials (5.a), the ink (5.b), one sun shadow map (5.f) and the emissive colour of the emissive slots. A tier may lower them (resolution scale, map size), never switch them off. Taking one off Low is Mustafa's call in the look proof test, on the Low frame (finding 2). The `[DEFAULT]` items (AO, bloom, comic touches) may still be off on Low, as written. Apply the same change to Rollback (:1188–1195), "Revisit when" (:665) and the fail route (:1028–1030). | graphics-engineer (parts 5, 7, Consequences, Rollback); test-engineer (the "When a mark fails" text, :1028–1030) |
| 2 | high | 0003:1051–1052; steps 1–30 (:1056–1119) | "His PC is far above the recommended class, so what he sees is the look, never the budget." | Every checklist step runs on MP at its own settings, so Mustafa judges only the High look. Most minimum-class players see Low: pixel ratio 1, scene scale 0.75–1, a 1024² shadow map or none, AO possibly off (0003:316, 309, 199). Legacy uses a 2048² map over 100 m (CD17, LEGACY_INVENTORY:98). A screen-space line drawn in pixels also aliases more at a lower resolution (r3-rendering.md line 813, quoted at 0003:192). He would sign off a look the minimum class never sees, which is exactly what LOOK_V2 §9's side-by-side judgement is for. | Add one step: the whole frame, the ink, the shadows and the AO views (steps 2, 4, 6, 7), shown at the Low tier's data beside High and legacy. MP can render Low's settings, because this step judges the look, not the budget. Expected: every `[DECIDED]` item still reads at Low. Record: achieved or not yet, per item. | test-engineer (the checklist); graphics-engineer (part 7's tier table names what Low shows) |
| 3 | medium | 0003:1056–1119; 0003:932–934 | LS runs "at BS's counts wherever a mark asks for the fight on screen" | No checklist step shows the busy frame: 48 soldiers in three teams, 4 very large units and 300 projectiles with their effects. Each step judges one item alone. LOOK_V2 §6.1 ("units, faction colours and effects are the brightest things on screen"), §6.4 (only three things glow) and §5 ("readable from a distance") are claims about the whole crowded screen. A dusk mood (step 19) can also flatten faction colours. Readability under load is the first thing that breaks. | Add one step: LS at the fight on screen, at the game camera, by day and at dusk. Expected: his own soldier, the three factions and a thrown grenade read at a glance; effects and glow do not wash out the units (§5, §6.1, §6.4). | test-engineer |
| 4 | medium | 0003:177–179, 187–190; step 4 (:1062–1064) | "depth edges carry the silhouette ink, normal edges the inner lines"; the hull draws "legacy's ink contact line where objects meet the ground" | Where a wall or a pair of boots meets the ground, the depth is continuous and only the surface direction changes. Under the proposed mapping the contact line is therefore drawn at the thin inner weight. The base of every building gets the thin line, against LOOK_V2 §2's "especially strong on buildings". Legacy's hull clamp draws a deliberate contact line (CL6, LEGACY_INVENTORY:115). This is my inference from R3-LOOK-F7's two edge kinds (r3-rendering.md lines 360–364), not research. It is a visible difference the draft does not name, so Mustafa may not know to look for it. | Name it in 5.b's visible-difference list. Point step 4's Expected line at where buildings and soldiers meet the ground: "the ink at the base of buildings is as heavy as their silhouette". The family choice stays Mustafa's. | graphics-engineer (5.b); test-engineer (step 4) |
| 5 | medium | 0003:1052–1053; steps 2, 8, 9 (:1059, 1070, 1072); 0005:2153–2156 | "beside a legacy screenshot at the same framing"; "at the game camera"; "at the closest and at the farthest camera framing"; "A slow zoom" | The camera framing is open (R2-INT-OQ2, a design question: README:119; CD5 becomes "camera framing pending", 0003:695). "The game camera" is therefore undefined, and legacy has no zoom range: one framing at height 40, boom 56.6, fov 15, yaw 45° (CD5, LEGACY_INVENTORY:86). Silhouettes, ink in pixels and LOD all depend on that view. Two people would set up different views, and step 9 cannot be set up at all. | State the test view: CD5's legacy framing, as a test view and not a camera decision. Name step 9's close and far views as test views, or mark step 9 and the LOD checklist "not judged until R2-INT-OQ2". | test-engineer (steps); graphics-engineer (part 11) |
| 6 | medium | 0003:220; 0005:1553–1554 (and AP-3, 0005:1392) | "Each material class (environment, units, faction colours, visual effects)"; "(for example environment, unit, faction colour, emissive)" | Neither example list has a class for buildings. LOOK_V2 §2 wants buildings' ink heaviest. §6.1 wants "the ground and environment" one step calmer. The roof fade needs structures marked (5.i). With either list, buildings fall into "environment" and get the ground's ink weight. In 5.b the per-class ink weight is read from the class value (0003:180–181), so the heavy building ink has no input. | Name a structure class in both example lists. Or carry the ink weight as its own field per asset in the asset list (AP-2), separate from the class that sets the value. | graphics-engineer (0003 5.h; 0005 GPU part 2); engine-architect (AP-3) |
| 7 | medium | 0003:398; 0005:1591–1592; SPIKE_PLAN:236–237; against step 14 (0003:1082–1083) | "one two-floor building with an interior and its damage stages" | The test assets hold one building. Step 14 shows "Both houses beside legacy's", and both are LOOK_V2 §2 `[DECIDED]` items: the half-timbered house with the brick-tile roof, and the plastered house with the silo tower. Families step 1 (0005:2142) names only the half-timbered house. The plastered house has no named source. | List both houses in the test assets, or say that the second comes from the ported kit (and so carries no thatch). | graphics-engineer (0003 part 11; 0005 GPU part 8); engine-architect (SPIKE_PLAN) |
| 8 | low | 0003:1053 | "taken by the session with legacy's pageshot, headed" | The session's container has no GPU (no `/dev/dri`). Under `xvfb-run`, "headed" still renders on the CPU: `tools/pageshot.js:9` says headless is SwiftShader, and headed means the real GPU only where one exists. Multisampling, shadow filtering and fonts then differ from Brave, so the side-by-side compares two renderers as well as two looks. | Take the legacy views on MP in Brave, at CD5's framing: legacy opens from index.html. Or label each session shot with the renderer pageshot prints, and use those shots only to match framing. | test-engineer |
| 9 | low | 0003:1080–1081 | "the cheaper rig's difference does not distract at the distance it is used" | The budget LOD picks soldiers by importance, not distance: "own squad, nearest, shooting" (0003:269–271). Under the top-down camera, cheaper-rig soldiers stand right next to full-rig ones. | Reword: "on the soldiers it is used for, in the fight on screen at Low's animation budget". | test-engineer |
| 10 | low | 0003:250–253; PM-0003-9 (:986) | halftone as "legacy's DOM overlay … costed in the look proof test against the post-processing row" | A CSS multiply layer is composited by the browser, outside the renderer's timestamp queries, so the post-processing row cannot see its cost. ART_DIRECTION.md:121 also records that the overlay "also dots the HUD/UI". A flash should animate opacity only; animating the sheet's transform hits the stacking-context trap (START_HERE.md:134). | Cost the flash as frame time with and without it. Show it over the HUD in step 27, so Mustafa judges the dotted HUD as well. | graphics-engineer (5.l); test-engineer (PM-0003-9, step 27) |
| 11 | low | 0003:189, 316 | the screen-space pass "draws a line in pixels"; Low ratio 1 with scale 0.75–1, High ratio 2 | A width set in render pixels shows at a different on-screen weight on each tier and pixel ratio, unless it scales with them. The ink weight would then change with the tier rather than by Mustafa's pick. | State that the ink width scales with pixel ratio × resolution scale, so a tier changes the cost, not the weight. | graphics-engineer |
| 12 | low | 0003:1084–1085; 0005:2146 | "logs and trees with variety. The trees are labelled placeholders … not judged here" | "Trees with variety, as a base to build on" is `[DECIDED]` (§2), while the style is `[OPEN]` (§8). The step asks for variety but judges nothing about the trees. | Say the placeholders are legacy's ported tree builders (GW18), judged for variety and lushness only, so the decided item is judged while the style stays open. | test-engineer; graphics-engineer (part 11) |
| 13 | low | SPIKE_PLAN:222 | "how each `[DECIDED]` item of LOOK_V2 looks" | The checklist also judges the `[DEFAULT]` items, which stand unless vetoed: steps 16, 18, 19, 21, 22 and 27, and parts of 17 and 20. | Change it to "each `[DECIDED]` and `[DEFAULT]` item", so a veto is recorded. | engine-architect |
| 14 | low | 0005:1569–1571; 0003:168–172 | "A soldier as drawn (body, headgear and held weapon) uses at most 4 materials per pass" | Each material carries one class ramp, so at most 4 surface kinds (skin, cloth, faction colour, metal, emissive) shade differently on a soldier. Legacy used separate skin, cloth, metal and lacquer ramps (CL2). This is not a defect: the cap is arithmetic, and the spike already builds the test soldier to it (0005:1672–1673). | Say in steps 10–12 that the soldier shown is built at the cap, so Mustafa judges what the budget allows. | test-engineer; graphics-engineer |

## Questions for Mustafa or the Advisor
- **LOOK-FLOOR. Which LOOK_V2 items may a quality tier switch off on the minimum laptop?**
  - Why it matters: Low is the default for every integrated adapter and every WebGL 2 player.
  - Options:
    - (a) none of the `[DECIDED]` ones (toon, ink, one sun shadow, emissive colour), which may only be lowered;
    - (b) any of them, if Low meets its budget only that way;
    - (c) Mustafa decides per item in the look proof test, on the Low frame.
  - Recommendation: (a), with (c) as the route when a mark fails (findings 1–2).
- **REDUCED-MOTION. Should the concussion shake, the halftone flashes and line boil have a reduced-motion setting, or
  follow the system's `prefers-reduced-motion`?**
  - Why it matters: legacy applies that setting only to the war-room menus (shamble-game.js:3039, index.html:321),
    never to the camera shake, and LOOK_V2 §2 and §7 make the shake and the flashes a feature.
  - Options: (a) as legacy (menus only); (b) a setting that softens the shake and switches the flashes off; (c) follow
    the system setting by default.
  - Recommendation: (b) or (c). It changes no `[DECIDED]` item for players who keep the default.
- **COLOUR-TELL. Are the three factions told apart by colour alone?**
  - Why it matters: in a three-team fight, players with colour-vision deficiency may not tell the teams apart.
    Checklist step 11 expects "the factions told apart by colour", and LOOK_V2 §2 makes uniform details the seed of
    faction identity.
  - Options: (a) colour only, for now; (b) the fight view (finding 3) is also judged through a colour-blind
    simulation; (c) faction shape cues are a later design session's.
  - Recommendation: (b): it costs one extra view.

## What I did not check
- **No renders.** Nothing renders yet, so every look judgement here is on text. Brave stays the final gate.
- **Numbers.** I did not re-verify the research citations or numbers; the verifier and the consistency and
  completeness critics covered those. Only the CD5, CD17, CL6 and GW17 rows were read in LEGACY_INVENTORY.
- **Outside my scope:**
  - ADR 0005's content data and ability system;
  - the budget arithmetic itself, which is perf-auditor's (finding 1 touches only which items a tier may drop);
  - PM-0003-12's note that the post passes fit in 14 draws under 800: if bloom's 12 draws count, Low may have to
    drop bloom. That is perf-auditor's number, and finding 1's floor keeps the emissive colour either way.
- **Partly read:** ADR 0001, 0002, 0004, 0006 and ENGINE_OVERVIEW were only grepped for look content, and none
  decides any look. README was read only at its look rows (18, 48, 61–71, 90, 118–137, 239–260, 282, 311–316).
- **The look checklist's length** (30 steps, plus 5 families steps, in "one session") was not judged as a session
  plan.
- **My memory.** No agent-memory directory exists, so there were no calibration notes from earlier reviews to apply.
- **My own agent file.** It still describes the legacy look ("joyful, readable, toy-box battlefield"; "v2 look is not
  decided"). That is STATUS's KIT-ALIGN item (advisor condition 18). I judged against LOOK_V2 as the prompt says.
