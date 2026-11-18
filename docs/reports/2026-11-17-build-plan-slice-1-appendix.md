# Appendix to the build plan's report: his words and the inputs, byte for byte

_Made by `decisions-insert.mjs` (change-set 0) from the files that were copied by script from the build chat's
transcript (session_019YNMUV9LFhASseJ9EP12kL) and from his attachments. Nothing inside a fence was retyped. Under
each block stand its sha256 and its size, counted without the file's last line end. The repository stores LF line
ends only (`.gitattributes`): a text that arrived with Windows line ends (CR LF) stands here with LF line ends,
and the line under its block also gives the size and sha256 of the file as it was attached. Everything in A4 to A7 and A10 is the Art lead's proposal unless a
line of Mustafa's says otherwise (his words of 2026-11-17, A3, CHANGE 3)._

## A1. His message of 2026-11-14, 13:58:05 UTC: the build plan's task, with his decision of that day
````text
My words of 2026-11-14, for a fresh chat on my personal account. We start building the game: foundation steps 2.4 and 2.5, then Slice 1, move and shoot.

Where things stand. Main is 4460ef5b8d6dc8566b29c15ded75c6f0c8be386b, with gate, smoke and toolchain green on it. It holds the empty skeleton of the new game, six Rust crates and five TypeScript packages, and no game code. Two other chats work on this repo. The netcode proof test: PR #40, branch claude/netcode-proof-test-plan-hcky5g, my university account. Step 2.3, the gate v2 and CI: branch claude/sweet-mayer-grsukw, my personal account; its plan is written and waits for my go. Never push to their branches and never touch spikes/.

Model. This chat plans and writes game and engine code, so by my words of 2026-11-12 its main model is Fable 5.2, and so is every builder agent you start to write that code. Check this session's model first and stop in one line if it is not Fable 5.2. Every other agent runs on Opus 5.5; the agents that review, verify or design game and engine code run at effort max. The advisor tool stays off in this chat: you are the model it would ask.

My decision of 2026-11-14, to record word for word:
"The game is built now, on my personal account, while the netcode proof test goes on on my university account. Foundation steps 2.4 and 2.5 and Slice 1 may be planned now and built after my go on the plan, on the decision drafts as proposed. They do not wait for the tests that docs/SPIKE_PLAN.md places before Slice 1 (the sim core spike, the 60 Hz run, the boundary spike, the look proof test, the content and ability spike and the render budget bench), nor for the end of step 2.3. Those tests are not built as separate tests unless the plan shows a question the real game cannot answer. No pass mark and no open question of theirs is dropped or loosened: each is measured on the real game, at the first build that can show it, and a draft is accepted only when its marks are met. If a mark fails, the code that rests on it is redone. Slice 3, online, still waits for the netcode proof test's result. Step 2.6, the previews, follows the preview hosting check as planned. Blender is optional: Slice 1 waits for no model made in it, and its first soldier is a stand-in built in code from the Art project's sheets. This changes, for these steps: my words of 2026-11-03 (SPIKE-ORDER (b), "wave 1 runs one heavy spike at a time", and LOOK-TEST-INPUTS, "Route: Blender"), condition 3 of SLICE-ORDER, my words of 2026-11-10 ("Steps 2.3 to 2.6 keep their order and wait for their results") and my words of 2026-11-13 ("Steps 2.4 to 2.6 still wait for their results"). These words are the rule from now, also where CLAUDE.md, .claude/agents/advisor.md, docs/SPIKE_PLAN.md and the decision drafts still say otherwise."

Read first: CLAUDE.md; .claude/CHECKPOINTS.md; docs/STATUS.md (behind by one merge: it still lists PR #52 as open); docs/ENGINE_OVERVIEW.md (the engine on one page); the migration plan in docs/adr/0002-code-structure-build-toolchain.md (steps 2.4, 2.5 and 2.6 and Slice 1, with their done-when lines) and every milestone it names in docs/adr/0001-multiplayer-netcode.md, 0003-rendering.md and 0004-simulation-core.md; docs/adr/0005-content-abilities-assets.md for weapons as data; docs/SPIKE_PLAN.md (the entries of the tests named in my decision: their questions and pass marks); docs/LOOP_V2.md (the game loop design; its section 7 is combat feel) and docs/LOOK_V2.md (the look brief); in docs/DECISIONS.md: SLICE-ORDER with its conditions, my words of 2026-11-12 on models, and "Queued" (my note for Slice 1 on the live camera tuning panel is there); and the plan for step 2.3 on branch claude/sweet-mayer-grsukw, .checkpoints/foundation-2-3-plan/plan.md, for the gate checks it lets ride steps 2.4 and 2.5.

The task now: the plan. No build, and no edit outside your checkpoint. The plan gives me:
1. The cut into change-sets that each merge on their own, in order, from step 2.4 to the end of Slice 1. Cut it so that the first thing I can open in Brave comes as early as the drafts allow, and so that every change-set after it ends in something I can open, play and judge. For each change-set: what I will see or feel in one plain line, its planned size, your estimate of its working time, and whether its merge needs my go.
2. For every done-when line and pass mark of steps 2.4 and 2.5 and Slice 1: the change-set that meets it.
3. For every question and pass mark of the tests named in my decision: where on the real game it is measured and at which change-set, or why it must stay a separate test. Name the few that must be measured before more code rests on them, and put those first. I expect among them: the same result natively and in WebAssembly from the first tick; the cost of the hand-off between the page and the simulation at 12 players; the frame time on the weakest target machine once the look is on.
4. What Slice 1 needs from the Art project, as a list I can paste there: the soldier, the test scene, colours, effects. The soldier starts as a stand-in built in code.
5. Where my queued words for Slice 1 land (the live camera tuning panel from its first playable build). My look decisions of 13 and 14 November are not in the repo yet; a docs chat carries them in. Plan the look work against docs/LOOK_V2.md as it stands and say which change-sets wait for that carry.
6. What needs the netcode proof test's result and so stays out until Slice 3.
7. How the code is checked while step 2.3's gate v2 is not built: the commands you run on every change-set (format, lint, type checks, tests) and show in its report, and which of step 2.3's change-sets you need merged first, if any.
8. How I open each build in Brave on my Windows laptop, in steps one at a time, and when the preview hosting check and step 2.6 can run, so that I get a link for every build.
9. The decisions you need from me, each with its options and your recommendation, in plain words.
10. What I do myself.

How to work. CLAUDE.md's change-set protocol. Your branch stands in for the cs/ branch; one pull request at a time. At most 3 agents, the saver running, a checkpoint after every step. Have the advisor agent review the plan. The plan gets no review rounds and no verifier; every change-set gets the full review when it is built. In a report, write about your own process and about what you told me only what a record shows, with its time. Write each change-set's planned size in the plan; when one passes twice its plan while it is built, stop and tell me in three lines: what grew, why, and the cheaper option. Ask me to compact when the chat is nearly full, and do not stand still waiting for it. If the guard, the deny list or the classifier refuses a command, record it with the route you took. No kit, gate or CI file is edited in this chat without asking me in chat with its exact text. If a rule in the repo forbids something here and my words above do not cover it, the repo wins: stop and tell me in three lines.

What this block covers: the plan. It covers no build and no merge. My decision above is recorded in docs/DECISIONS.md by step 2.3's first docs change-set, which the 2.3 chat builds; open no docs pull request for it. No build starts before that record is on main and I have given my go on your plan.

End with: the plan's path, a one-screen summary in plain words, the table of change-sets with what I see after each and your estimates, the decisions you need from me, and how full this chat is. Then stop for my go.
````
sha256 of the block: `cbe41d9e2bd7ec1f65468f94ff6f6f4e9cc9c07a955304940665afc2c18a1c9c` (7538 bytes).

## A2. His addition of 2026-11-14, 14:12:13 UTC (his label: 15:03)
````text
My words of 2026-11-14, 15:03, an addition to my block of today. Take it into the plan you are writing.

On Blender, my words: "Okay, you have convinced me for the Blender usecase, I was not aware that there was a workflow possible from Blender to web, lets make use of it. Its especially nice because of the MCP <> Claude workflow".

What this changes in my decision of today. Its sentences on Blender now read:
"Slice 1 waits for no model: its first soldier is a stand-in built in code from the Art project's sheets. The models themselves are made in Blender, as my words of 2026-11-03 say (LOOK-TEST-INPUTS, "Route: Blender"), by Claude through the Blender MCP on my PC from the Art project's brief, and exported as GLB. The first model replaces the stand-in when it is ready. This changes, for these steps: my words of 2026-11-03 (SPIKE-ORDER (b), "wave 1 runs one heavy spike at a time"), condition 3 of SLICE-ORDER, my words of 2026-11-10 ("Steps 2.3 to 2.6 keep their order and wait for their results") and my words of 2026-11-13 ("Steps 2.4 to 2.6 still wait for their results")."
So my words of 2026-11-03 on the route stand unchanged. Everything else in the decision stays as I wrote it.

Add to the plan:
11. The rules a model file must follow so that the engine loads it and colours it per faction: the file format and export settings, scale, axes and origin, the names of the colour areas, the caps for triangles and materials, and whether Slice 1's soldier needs a skin. This is the export and slot-naming part of the asset research (OQ-0005-AS3 and AS4 in docs/adr/0005-content-abilities-assets.md; its entry in docs/SPIKE_PLAN.md). Say who answers it and when. Put it early: the Art project's Blender brief waits for these rules, and I want them as a short list I can paste there.
12. The change-set in which the first Blender model replaces the stand-in. The spike plan places the asset spike "before the first authored model enters the game". Treat it as the tests named in my decision: say which of its pass marks (PM-0005-38 to PM-0005-44) are measured on the real game at that change-set and which later, with none dropped or loosened.
13. In your list of decisions for me: whether a committed model may be public (OQ-0006-18), with options and your recommendation. No model file is committed before I have answered it.
````
sha256 of the block: `be95e64501c5c96d5ccb2594636e43ad8c251677d16e31e7059cf898c1eef00d` (2340 bytes).

## A3. His message of 2026-11-17, 19:10:44 UTC (his label: 20:10): his go, his answers, four changes
````text
My words of 2026-11-17, 20:10 (Amsterdam time): my go on your plan, with my answers and four changes. Where a line below carries a date, that is the day I said it.

MY GO
Main has moved since your plan: it is c94c81b440c77f7979e0f944a5341319f9a57d1b. Three pull requests were merged: #53 (a kit rule for packing old test logs), #54 (three lines in the night-shift rules) and #55 (docs). None touches a game file. Take main in first, and tell me in three lines if anything in your plan no longer holds.
I give my go on the plan as written (revision 2, commit 68addf9): build change-set 0 now, then the change-sets in the plan's order, each as its own pull request that stops for my go on the merge. This go covers no merge. I could save the plan file you sent on my laptop, so the route works for a file; change-set 1's test archive proves it for a zip.

MY ANSWERS TO THE TWELVE DECISIONS
1. SPLIT-25: (a). Step 2.5 is built as two change-sets.
2. FIRST-SOLDIER: (a). The Conscript of the sheets; the SMG and the rifle both in change-set 8; I judge the SMG's feel first.
3. GATE-FOLLOW: (b), with (i). You ask me for each gate and CI edit with its exact text; for those edits I set my words of 2026-11-12 ("The guard, the gate and CI work stays with Opus 5.5.") aside. The gate's network step waits for the netcode proof test's report.
4. MACHINES. My words: "it can be both: we can have a lower target for desktops: rtx 2070 and i7-9700k and lower level laptops with Intel Iris Xe graphics or the AMD equivalent, don't over complicate it for now. keep it simple. I would want to target higher level of quality game, I don't want to aim for a lower level fidelity because the hardware is bad, we just try to do our best to write and optimize the game as good as possible. So a higher floor doesn't sound bad."
   - The classes are named now. The recommended desktop is an RTX 2070 with an i7-9700K; this names the desktop of my words of 2026-11-03 ("a desktop with a mid-range card (RTX 3060/4060 class)"). The minimum laptop is of the class the rendering draft already names: Intel Iris Xe graphics or the AMD equivalent.
   - The actual machines I name before change-set 6, your option (a): on 2026-11-03 I said I name them before the look proof test, and change-set 6 is where that test now happens. Until then my words stand: "The laptop and desktop stay on my timing: rough checks first, then a short re-run once they're named."
   - On quality: the look is built for the desktop on High, and Low is what the laptop gets. My line in docs/VISION_V2.md ("Minimum: a laptop with integrated graphics runs the largest mode at 30+ fps on Low") stands for now. If Low cannot hold it on that laptop without hurting the look, bring me the numbers and I decide whether the floor rises. Do not lower the look on High to reach it.
5. BRAVE-ROUTE: "A, if not possible: B".
6. PREVIEW-START: (a). It has started, in the second builder chat (change 2 below).
7. STAND-IN-LOOK: (a). I judge proportions and weapon shapes on the Blender model.
8. COMPARE: (b). The comparison against a library's character controller runs only if our own movement misses its marks. This is my word for the change to PM-0004-34's wording. It is the one exception to "No pass mark and no open question of theirs is dropped or loosened" in my decision of 2026-11-14; the other comparisons run.
9. MODEL-PUBLIC: (a), and my words: "A, once the game is up, I will look to make the repo private later on." The sheet notes you pushed may stay.
10. RULES-WHO: (a), and my words: "A, but might eventually go with C with time depending on how it will go".
11. SCOPE-RELOAD: my words: "Yes active reload will be a part of slice 1, but a more simplified version, hitting it will make the reload end faster. The additional depth will be added later." It joins change-set 8.
12. START-RULES: (b), I decide both now. The jump: the formula's value, a reach of 3.07 m (OQ-0002-6, its option (a)). Two soldiers meeting: they push each other softly and never block. Both are numbers and rules I can change after I have felt them in Brave.

CHANGE 1: CHANGE-SET 0 RECORDS MY DECISIONS
My first block left the record of my decision to step 2.3's docs change-set. That changes: your change-set 0 records it. Into docs/DECISIONS.md, by script from this chat's transcript: my block of 2026-11-14 with its decision, my addition of 15:03 that day, and this message. Pull request #55 brought docs/STATUS.md and docs/ROADMAP.md up to date for main on 2026-11-17; change-set 0 changes them only where my decision changes what they say. No game code is built before change-set 0 is on main.
The 2.3 chat stays stopped for now: I pass it nothing until its plan has been checked. So its lint and format settings will not be on main before your change-set 2; work as your Part 7 says for that case.

CHANGE 2: A SECOND BUILDER CHAT
My words of 2026-11-14, 22:03: "I also decided to change my mind regarding building the game with Fable 5.2, the main builder can using fable but I also want an opus 5.5 bot work on it too, also work on other parts of the main game. I have seen how strong opus 5.5 is with a 5.3 fable advisor." The advisor I mean is Fable 5.2.
That chat exists since 2026-11-15: branch claude/optimistic-albattani-mdoonz, main model Opus 5.5, Fable 5.2 on its advisor tool. It runs the preview hosting check in a separate test repository of mine and touches nothing of yours.
My decision, to record word for word:
"Two chats build the game on my personal account. The first builder chat keeps Fable 5.2 as its main model. A second builder chat has Opus 5.5 as its main model, with Fable 5.2 answering its advisor tool, and it also writes game and engine code. This changes my words of 2026-11-12 ("On my personal account, Fable 5.2 is the main model of every chat that writes game or engine code, and of every builder agent that chat starts to write that code.") for the second builder chat only. Every file has one owner: the first builder chat's plan says which change-sets and folders belong to the second chat, and neither chat edits the other's files."
Before change-set 1 starts, add a short Part 15 to your plan, "The second lane". My proposal: the second chat takes the preview hosting check and step 2.6, the carry of my look decisions into docs/LOOK_V2.md, the comparisons that stay (Part 3.2), and from change-set 6 on the picture track: 6 Look, 10 Body, 11 Feel, and M. You keep 0 to 5, the simulation track (7 Lag, 8 Shoot, 9 Throw) and 12. Say where you agree and where you do not, with your reasons. For each unit of the second lane: the folders and files it owns, what it needs merged first, what it must not touch, and how the two chats' pull requests take turns. Every CI and workflow file also gets one owner there, and Part 15 says which chat builds the gun models. Send me Part 15 as a file when it is written; it does not hold change-set 0.

CHANGE 3: THE SOLDIER AND THE GUNS FOLLOW THE ART LEAD'S BRIEFS
Attached, three texts and two pictures: the Art lead's build brief for the Conscript's first blockout (2026-11-14, 16:35), its addition (2026-11-15: the poses, the marking, the hat and the rifle, the yard, the ground), its SMG brief (2026-11-15, 17:35), the redrawn Sheet 16, and my SMG line drawing. The soldier brief replaces the sturdier sizes you typed into your notes on 2026-11-14 at 15:18 UTC. Everything in them is the Art lead's proposal unless a line says it is mine.
- The stand-in takes the artwork's proportions. My words to the Art lead on 2026-11-14: "A, lets see how you do it, if its not sufficient, we'll go for B (using MCP)".
- No skin colour on this first stand-in: hands and face take the Dark material, so four materials hold, the rifle included. My words on 2026-11-14: "Answer: go with A". This is about colour only: the soldier stays a skinned mesh, as your Part 11 says.
- Its size, 1.80 m to the top of the hat, is a placeholder: hang the soldier on one root so it scales with one number.
- Units stay as decided in docs/EDITOR_V2.md, "1 unit = 1 metre". When I speak of Unreal units I mean centimetres.
- The brief's names for colour areas and bones are working names; the names of your change-set 0 win.
- Line widths: docs/LOOK_V2.md stands until I decide otherwise. The Art sheet's widths, units thicker than buildings, are a proposal: make both switchable at change-set 6 so I judge them in Brave.
- A first model file exists: AMK9_v4_stage1.glb, a rifle prototype written by a script (glTF 2.0, 63 KB, 2 meshes, 2,596 triangles, 1 material, four sockets, 1.12 m long, +Y up, +Z to the muzzle). It stays with me until you ask for it.
- The poses. Where the addition's hand points and a gun's sockets disagree, the sockets win: the right hand's point and the gun's direction set the pose; the left hand, the butt and the muzzle follow from the gun model's sockets, as the SMG brief already does.
- The muzzle is where the flash and the tracer start. In the simulation a shot's start is data of its own: docs/adr/0005-content-abilities-assets.md says no shot origin is ever read from a model.
- The SMG: my drawing is the chosen design, and the brief's numbers win over the drawing. Its sockets are named as in the rifle file: socket_grip, socket_support, socket_muzzle, socket_sight, and the new socket_eject.
- The weapons line. In your list for the Art project, read "(never a plain tube)" as struck. The words "no plain tubes or cylinders" in docs/LOOK_V2.md line 30 and in the rendering draft are not mine. My words of 2026-11-15: "I never said those words." Treat them as not decided; the carry of my look decisions removes them.

CHANGE 4: SHAMBLE VFX EXISTS
A third Claude Code chat has built Shamble VFX, my effects editor, in its own repo (github.com/Mergincanli/ShambleVFX): an editor, a small runtime made to ship in the game, and an export folder the game loads by event name. Your plan does not know it. In Part 15, say how change-set 11 (Feel) uses it. My lean: the game loads Shamble VFX's exported effects through its runtime and builds no effect a second time. Say what has to be proven for that (the same picture in the game as in the editor, its cost inside the frame budget, how the runtime's code reaches this repo) and what it changes in your sizes. docs/EDITOR_V2.md section 7 still calls the VFX editor a later tool (my words of 2026-11-06): say which line needs a dated pointer.

End each change-set as your plan says. In every report, write about your own process only what a record shows, with its time.
````
sha256 of the block: `dbc486cafd1a356896115842be683a4cecccf51e15ecd9cbb0d9229e52bc856f` (10544 bytes).

## A4. The Art lead's brief as he attached it on 2026-11-17: ART_BRIEF_2026-11-14_1635_conscript_stand_in.md
````text
BUILD BRIEF: THE CONSCRIPT'S FIRST BLOCKOUT. From the Art lead, 2026-11-14, 16:35. It replaces my brief of 16:20.
For whoever builds it: the builder chat, in code, as the stand-in soldier of Slice 1. A Blender session can use the same list later. This brief covers his shapes and colours only. Movement, shooting and the rig's rules are the build plan's.

WHAT IT IS
- Unit: the Conscript, the tier-one rifleman (bolt-action rifle, Molotov), in the uniform all three factions share: fur hat with ear flaps, scarf over the lower face, long double-breasted coat with a belt, flat shoulder boards, tall boots.
- Build: slim and serious, about 8 heads tall. Wide shoulders, a clear waist at the belt, a coat that flares to the hem. Not chunky. The proportions are measured from the Conscript artwork, which Mustafa calls the style direction.
- Game camera: 35.3 degrees above the ground, 15 degree lens, turned 45 degrees. Headgear, shoulders and the coat's outline matter most.
- The picture of all this is Sheet 16 (s16_engine_soldier.jpg). Where the picture and these numbers differ, the numbers win.

AXES AND SIZE
- Metres. L = to his left, F = forward, the way he faces, U = up. Positions are (L, F, U). Origin on the ground between the feet.
- Size is a placeholder: 1.80 to the top of the hat (180 Unreal units). Hang every part on one root so the whole soldier scales with one number.
- Coarse shapes only: boxes and 8-sided cylinders. "Tapered" means the two ends have different sizes. One part per name below.

HEIGHTS TO HIT (U, and share of the height)
ground 0 | boot top 0.36 (20 %) | coat hem 0.43 (24 %) | fists 0.77 (43 %) | belt 1.045 to 1.10 (60 %) | shoulders 1.49 (83 %) | chin 1.54 (86 %) | brow 1.66 (92 %) | top of hat 1.80
WIDTHS TO HIT (L)
shoulders over the arms 0.49 (27 %) | chest 0.38 | waist at the belt 0.33 (18 %) | coat hem 0.61 (34 %) | hat 0.235 | hat with ear flaps 0.28

MATERIALS: 4 flat colours per soldier, the rifle included. WORKING NAMES; the build plan's names replace them.
           Soviets    Allies     Empire
Coat       #A72123    #235DBD    #F5F3EA
Top        #D41414    #1F6FF0    #890089
Trim       #8F6F30    #FFFD82    #8F6F30
Dark       #4D4843    #4D4843    #4D4843
Mustafa's: all Coat and Top values, the Soviet and Allied Trim. The Art lead's: the Empire Trim and Dark.
No skin, leather, wood or steel: hands, face strip, belt, scarf, boots and rifle are all Dark (Mustafa, 14 November: no skin on this first stand-in).

PARTS, biggest first. Size is width (L) by depth (F); U is from-to. Left parts are given; mirror them for the right. Each line ends with its material and, for a rigid skin, the bone it follows (working bone names).
1. coat_skirt. Tapered. U 0.43 to 1.045. At the hem 0.61 by 0.46; at the belt 0.33 by 0.23. Coat. Hips.
2. torso. Tapered. U 1.10 to 1.47. At the belt 0.33 by 0.22; at the top 0.38 by 0.25. Coat. Chest.
3. belt. U 1.045 to 1.10. 0.345 by 0.245. Dark. Hips.
4. boot_L/R. Tapered, U 0.10 to 0.50 (the top is hidden in the coat), 0.12 across at the ankle, 0.145 at the top, centre at L 0.11. Dark. Leg (one piece from hip to ankle).
5. foot_L/R. Box 0.11 by 0.28, U 0 to 0.10, heel at F -0.08, toe at F 0.20. Dark. Foot.
6. trouser_L/R. Cylinder 0.165 across, U 0.36 to 0.50, round the boot top. Dark. Leg.
7. Arm. Joints in the rest pose: shoulder (0.19, 0, 1.44), elbow (0.255, 0, 1.14), wrist (0.31, 0.03, 0.875). The arm hangs clear of the body, 12 degrees out.
   upper_arm_L/R: tapered, shoulder to elbow, 0.115 across at the shoulder, 0.10 at the elbow. Coat. Upper arm.
   forearm_L/R: tapered, elbow to wrist, 0.10 to 0.09. Coat. Forearm.
   cuff_L/R: 0.125 across, on the last 0.07 of the forearm. Wider than the forearm on purpose. Coat. Forearm.
   hand_L/R: box 0.09 by 0.10, 0.10 high, centre (0.32, 0.04, 0.82). Dark. Hand.
8. board_L/R. Flat box 0.17 (L) by 0.09 (F), 0.015 thick, lying on the shoulder, centre (0.165, 0, 1.485). Trim. Chest.
   board_plate_L/R: flat box 0.13 by 0.05 on top of it, so a 0.02 rim of Trim shows all round. Top. Chest.
9. collar. 8-sided, 0.20 across, U 1.45 to 1.51. Coat. Chest.
10. scarf. Tapered 8-sided, U 1.49 to 1.615, 0.15 across at the bottom, 0.17 at the top. Dark. Head.
11. head. 8-sided, 0.18 by 0.21, U 1.54 to 1.70. Only the strip from 1.615 to 1.66 shows. Dark. Head.
12. hat_band. 8-sided, 0.235 by 0.25, U 1.66 to 1.76. Coat. Head, as are 13 to 15.
13. hat_crown. Tapered, U 1.76 to 1.795, 0.225 across at the bottom, 0.20 at the top. Coat.
    hat_top: a disc 0.19 across on its flat top, U 1.795 to 1.80. Top.
14. hat_flap_front. Slab 0.19 wide, 0.02 thick, U 1.665 to 1.75, on the front of the band at F 0.13. Dark.
15. ear_flap_L/R. Slab 0.03 (L) by 0.12 (F), U 1.55 to 1.72, centre at L 0.125. Coat.
16. Small parts, may be left out. Pouch: box 0.12 by 0.06, U 0.88 to 1.04, on the coat's front at his left hip, centre (0.13, 0.155), Dark, hips. Buckle: 0.07 wide, U 1.04 to 1.105, front centre of the belt, Trim. Buttons: two rows of four, 0.026 squares at L 0.05 and -0.05, U 1.17, 1.25, 1.33 and 1.41, on the front of the torso, Trim.
17. Rifle. Stand-in: 1.12 long, 0.06 wide; butt 0.26 behind the grip, muzzle 0.86 ahead of it; 0.13 thick at the butt, 0.075 at the grip, 0.055 at the fore-end, barrel 0.03. Dark. Or load AMK9_v4_stage1.glb, which has grip, support, muzzle and sight sockets.
    Idle hold: across the chest, in a plane 0.22 in front of the body's centre, rising 42 degrees from the right hand at (-0.13, 0.22, 1.06) towards the left shoulder; the left hand under the fore-end at (0.21, 0.22, 1.32). Other poses are the build plan's.

MARKINGS (may be left out at game size, where they are about 4 by 3 px)
- One on each board plate, radius 0.022, and one on the hat's front flap, radius 0.03, centre at U 1.708.
- Soviets: star; Trim on the boards, Top on the hat. Allies: chevron; Trim on both. Empire: flower; the Coat's white on both.
- Shapes on a grid from -1 to 1. Star: 5 points, inner corners at 0.42 of the outer radius. Chevron, 6 corners: (-1, -0.15) (0, 0.85) (1, -0.15) (1, -0.85) (0, 0.15) (-1, -0.85). Flower: 5 round petals out to 1.0, dipping to 0.4 between them, with an open round centre of radius 0.27.

MUST READ AT GAME SIZE
The outline (fur hat, long coat that widens to the hem, rifle across the chest); the coat colour; the Top colour on the hat top and both boards; the Trim rim round each board; the black line round the whole figure.

SEND BACK, for the Art lead's check
1. Numbers: the soldier's bounding box in the rest pose without the rifle (expected about 0.73 wide over the hands, 0.46 deep, 1.80 high); shoulder width over the arms (0.49); body width without the arms at the belt (0.345) and at the hem (0.61); triangle count. No triangle budget is set yet, so only report it.
2. Pictures on a plain grey ground: front, side, back and top without perspective; and the game's own camera at 1080p with four copies in a row, 1.5 m apart, each a quarter turn from the next, so one faces the camera, one faces away and two show their sides. Send the full frame and a crop of the four enlarged 3 times. Each should be about 148 px tall.
The Art lead answers "pass" or a numbered list of fixes.

DO NOT
- No textures, no shading work, no extra parts, no rounding or smoothing before the check passes.
- No model file in the game's repo.

IF THIS IS BUILT IN BLENDER LATER
Z up, the soldier faces -Y, his left is +X, so (L, F, U) is (X, -Y, Z). Parent all parts to one empty at the origin, apply scale, one object per part, four materials with the working names. Game camera: vertical field of view 15 degrees, at (23.50, -23.50, 24.44), aimed at (0, 0, 0.9), 1920 by 1080. No export until the export settings are decided.
````
sha256 of the block: `cea9998e79aac473950561e06c3f2a3c6f73602c557cf54fc35da78707420bb1` (7731 bytes). As attached, with CR LF line ends: `455a3f5a368aefaff504705ce8149a4099fe8a2d22795a1c2a809d9b8de17d8c` (7803 bytes).

## A5. The Art lead's brief as he attached it on 2026-11-17: ART_BRIEF_ADDITION_2026-11-15_poses_yard.md. Its section A1 is replaced by A10
````text
ADDITION TO THE BUILD BRIEF OF 2026-11-14, 16:35. From the Art lead, 2026-11-15.
For the builder chat. It answers parts A and C of the build chat's list. Everything here is the Art lead's proposal unless a line says it is Mustafa's. Positions are (L, F, U) in metres, as in the brief: L to his left, F forward, U up.

A1. HOW HE HOLDS THE GUN
Three poses for the rifle (1.12 m long). Place the hands; the arms follow. From the game camera, the line of the gun tells the player what the soldier is doing.
- Idle, standing, as in the brief: across the chest, in a plane 0.22 in front of the body's centre, rising 42 degrees from the right hand at (-0.13, 0.22, 1.06) towards the left shoulder. Left hand at (0.21, 0.22, 1.32).
- Aiming and firing, also while walking: the rifle level, pointing straight ahead, at chest height. Butt at (-0.12, 0.05, 1.40) against the right shoulder. Right hand on the grip at (-0.12, 0.31, 1.36). Left hand under the fore-end at (-0.10, 0.58, 1.37). Muzzle at (-0.12, 1.17, 1.40); shots leave from there. The chest turns 20 degrees to his right, so the left shoulder comes forward. The head keeps facing forward.
- Sprinting, when the gun is not ready: across the chest as in idle, but flatter, rising 25 degrees. Right hand at (-0.13, 0.22, 1.10), left hand at (0.23, 0.22, 1.25). The body leans 10 degrees forward.
The loop design gives every gun a sprint-to-fire delay (Mustafa, 2026-11-04). The swing from the sprint pose to the aiming pose is where that delay shows.
A shorter gun such as the SMG uses the same three poses. Its hand points come with its own sheet.

A2. THE MARKING AT GAME SIZE
Leave it out. The plain shoulder board is enough: a Trim rim round a Top plate, together with the Top colour on the hat top, tells the faction from the game camera. Keep the marking as its own optional piece on each board plate and on the hat's front flap, off at game size and on in close views.

A3. HAT AND RIFLE
- The hat is part of the body's model: one of its named parts, following the head bone. It belongs to the uniform, and all three factions share it.
- The rifle is a model of its own. The right hand holds it at its grip socket and the left hand goes to its support socket. Slice 1 has two guns on one soldier, and the AMK-9 file already has the sockets.
- The rifle uses the soldier's Dark material, so the soldier with his gun still has 4 materials.
- This also answers B4 for now: in Slice 1 every gun stays Dark. A gun is told by its shape; the look brief asks for "real silhouettes, no plain tubes or cylinders". A second gun colour would be a fifth material.

C1. THE TRAINING YARD'S COLOURS
Follow the loop design's greybox rule: height is read through brightness, higher is lighter. Use three of palette A's base colours:
- ground: street #B8AD98
- blocks, ledges and ramps up to about 1 m: stone #D0C2A2
- blocks of 2 m and more: plaster #E6DDC8
- range posts: timber #4A3A2E
- the black line on their edges, as on buildings. Sun and shade values as on Sheet 17.
One thing to watch: a white Empire coat (#F5F3EA) in front of plaster is the weakest pairing in the palette. The yard is a good place to judge it.

C2. WHERE THE STREET MEETS THE GRASS
A little more than two flat colours:
- both stay flat: street #B8AD98 and grass #A7B58F;
- the border is uneven, wandering about 0.1 m to either side, not a ruler line;
- a worn strip about 0.25 m wide runs along the street's edge in a tone between the two: #B0B194;
- no black line on the ground.
````
sha256 of the block: `0ffe3f1989124a1798a643d6d837b3511c9417628bb9ab2299ff4644baf2d887` (3499 bytes).

## A6. The Art lead's brief as he attached it on 2026-11-17: ART_BRIEF_2026-11-15_1735_smg_stage1.md. Its section "How the Conscript holds it" is replaced, and its socket names and magazine corrected, by A10
````text
BUILD BRIEF: THE SMG, STAGE 1 (BLOCKOUT). From the Art lead, 2026-11-15, 17:35. It replaces my SMG brief of earlier today.
For the builder chat. A second gun beside the rifle (AMK9_v4_stage1.glb), built the same way and to the same conventions, so the two can be swapped in the soldier's hands. Mustafa gives you the line drawing it comes from; that drawing is his chosen design. Where the drawing and these numbers differ, the numbers win. All sizes are the Art lead's and are placeholders that scale with the soldier.

WHAT IT IS
- A submachine gun with a rifle-style stock, a fat barrel jacket, a long curved magazine that sticks out to the gun's left side, and a small grip bump under the gun where the barrel starts. The magazine on the left and the grip bump are Mustafa's words of 2026-11-15.
- In Slice 1 the Conscript carries it; Mustafa judges its feel first.
- Total length 0.80 m (the rifle is 1.12 m).
- Four shapes make its silhouette: the jacket, the receiver tube with its magazine housing, the side magazine, and the wedge of the stock.

CONVENTIONS, THE SAME AS THE RIFLE FILE
- glTF 2.0, 1 unit = 1 m, +Y up, +Z to the muzzle, +X to the gun's left. Origin at the grip socket.
- One material: Dark (#4D4843), a working name. The soldier's Dark material is the same one, so soldier and gun stay within 4 materials.
- Names with letters, digits and underscores only.
- If it is built in Blender instead: Z up, the muzzle towards -Y, origin at the grip socket, all transforms applied.
- Coarse shapes only: boxes and 8-sided cylinders. Report the triangle count; no budget is set yet. The rifle has 2,596.

HOW TO READ THE PART LIST
"d" is the distance from the muzzle tip along the barrel, in metres. The bore line is the barrel's centre line. Up, down, left and right are the gun's own, seen from behind it. Round parts are given by their diameter; boxes as width (left-right) by height.

PARTS, biggest first
1. receiver. Cylinder, 0.042 across, d 0.28 to 0.63, on the bore line.
2. jacket. Cylinder, 0.05 across, d 0.11 to 0.28, on the bore line. Fatter than the receiver on purpose. No holes at this stage.
3. stock, in three pieces that touch:
   fore_end: box 0.04 by 0.032, d 0.42 to 0.61, its top 0.018 below the bore line.
   wrist: tapered, 0.036 wide, d 0.61 to 0.675; 0.04 high at the front, 0.036 at the back, sinking so its centre goes from 0.032 to 0.04 below the bore line.
   butt: tapered, 0.036 wide, d 0.675 to 0.80; 0.04 high at the front, 0.10 high at the butt end, with its top edge just below the bore line.
4. mag_housing. Box, d 0.32 to 0.42, 0.045 high, centred on the bore line, reaching from the gun's centre to 0.08 on its left.
5. magazine. A flat curved slab sticking out to the left from the housing, level with the barrel, from 0.08 to 0.26 left of the bore line; 0.045 along the gun, 0.02 high. Build it from three straight pieces, each bent 8 degrees further towards the muzzle, so its far end sits about 0.035 nearer the muzzle. A floor plate closes the end: 0.05 by 0.026, 0.008 thick.
6. collar. 8-sided block, 0.058 across, d 0.28 to 0.32, between jacket and housing.
7. fore_grip. The grip bump: a box 0.03 wide, d 0.28 to 0.32, hanging 0.06 down from the collar's underside. The left hand holds it.
8. barrel. Cylinder, 0.02 across, d 0 to 0.11. muzzle_collar: cylinder 0.034 across, d 0.035 to 0.07.
9. trigger_guard. A thin plate 0.011 wide, d 0.56 to 0.61, hanging 0.03 below the fore-end.
10. Small parts, may be left out at game size: front sight (0.015 wide, 0.028 high, at d 0.09, on top); rear sight (0.022 wide, 0.018 high, at d 0.56, on top); bolt handle (0.014 across, 0.028 long, pointing right, at d 0.37).

SOCKETS: empty nodes, named exactly as in the rifle file, plus one new one
- grip: d 0.64, on the centre, 0.036 below the bore line. This is the origin. The butt end is 0.16 behind it, the muzzle 0.64 ahead.
- support: on the fore_grip, d 0.30, on the centre, 0.07 below the bore line.
- muzzle: d 0, on the bore line.
- sight: d 0.56, 0.04 above the bore line.
- eject (new): d 0.37, 0.028 to the right of the bore line, pointing right and 20 degrees up. Shells leave here, on the side away from the magazine.

HOW THE CONSCRIPT HOLDS IT. Positions are (L, F, U) in the soldier's frame, as in the soldier's brief. The left hand is always on the support socket.
- Aiming and firing: level, pointing straight ahead. Butt at (-0.12, 0.05, 1.40), right hand on the grip at (-0.12, 0.21, 1.36), left hand on the grip bump at (-0.12, 0.55, 1.33), muzzle at (-0.12, 0.85, 1.40). The magazine then sticks out to his left, about 0.48 in front of him.
- Idle: the same grip, lowered to belt height, turned 30 degrees to his left and tipped 20 degrees down. Right hand at (-0.14, 0.20, 1.08).
- Sprinting: level, turned 45 degrees to his left and pulled close. Right hand at (-0.12, 0.22, 1.15). The body leans 10 degrees forward.
Keep the magazine clear of his body and arms in every pose. The movement plan may change these poses.

MUST READ AT GAME SIZE
The magazine sticking out to the side; the fat jacket at the front; the wedge of the stock at the back; a gun clearly shorter than the rifle.

SEND BACK, for the Art lead's check
- Pictures on plain grey: side, top, front, and the game's camera with the Conscript holding it in the aiming pose, beside the same soldier with the rifle. Full frame and a crop enlarged 3 times.
- Numbers: total length (0.80), the magazine's reach to the left (0.26), triangle count.
The Art lead answers "pass" or a numbered list of fixes.

DO NOT
No holes in the jacket, no textures, no second colour, no bevels or smoothing, no parts that are not listed, before the check passes.
````
sha256 of the block: `8355194d13fa08ec8eba1d52b4d9f24108ed004645a585c9f6c3add9c5feff62` (5688 bytes). As attached, with CR LF line ends: `dec3c832abdf9bacba99990d18a9bcdffc3da2e1e30718ac83eebdffbfe89c23` (5744 bytes).

## A7. The three Art sheets of 2026-11-14 (images sent into the chat), as this session typed their text. Its soldier sizes are replaced by the brief in A4 (his words of 2026-11-17, A3, CHANGE 3); kept because the plan's Part 4 cites them
````text
# The Art project's sheets for Slice 1, as received in this chat on 2026-11-14

_Mustafa sent three images into this chat while the plan's review was being folded in, with the words: "The Art
director chat told me to send this to you". The images are the record. The text and numbers below were typed from
them by this session, so that the plan and the later change-sets can use them; a number that matters is read again
from the image (or from the Art project's file) before it enters code. "decided", "picked", "dropped" and "mine" are
the sheets' own labels ("mine" is the Art lead's proposal)._

## Sheet 1: "One Conscript, three factions: the same uniform, recoloured, with the faction's own marking"
- Its line: "Two colour areas change per faction: the coat with the hat, and the top parts (hat top, shoulder boards).
  The marking changes shape. Everything else stays. Blockouts, not the design."
- Soviets (decided): coat #A72123, top #D41414, trim #8F6F30, marking: star.
- Allies (decided): coat #235DBD, top #1F6FF0, trim #FFFD82, marking: chevron.
- Empire A, purple coat (dropped): coat #8A3388, top #890089, trim #8F6F30, marking: flower.
- Empire B, white coat (picked): coat #F5F3EA, top #890089, trim #8F6F30, marking: flower.
- Each faction is shown at game size from four sides: to the camera, right, away, left.

## Sheet 2: "The Conscript as a stand-in for the engine: one soldier built in code from simple shapes"
- Its line: "The shared tier-one uniform of Sheet 15 in 4 materials, rifle included. Sizes are the Art lead's, read from
  the Conscript artwork. A blockout for Slice 1, not the design."
- Heights (front view): top of hat 1.83 m (100 %); chin 1.52 m (83 %); shoulders 1.45 m (79 %); belt 0.98 m (54 %);
  coat hem 0.34 m (19 %); ground 0.00 m. Bands: head and hat 17 %; coat 61 %; legs below hem 19 %.
- Widths: shoulders over the arms 0.74 m (40 %); coat hem 0.61 m (33 %); hem depth 0.45 m (side view).
- "Four materials per soldier":
  1. Coat (coat, sleeves, hat, ear flaps): Soviets #A72123, Allies #235DBD, Empire #F5F3EA (each "decided").
  2. Top (hat top, shoulder-board plates): #D41414, #1F6FF0, #890089 (each "decided").
  3. Trim (board rims, buttons, buckle): #8F6F30 (decided), #FFFD82 (decided), #8F6F30 ("mine").
  4. Dark (boots, trousers, belt, pouch, scarf, hat flap, gloves, face strip, rifle): #4D4843 for all three ("mine").
- "Empire: its flower takes the coat's white (material 1). The drafted cap counts the held rifle too. So skin,
  leather, wood and steel are not materials here: they are all Dark."
- "The three markings as outlines": Soviets: star, 5 points, inner corners at 0.42. Allies: chevron, 6 corners, twice
  as wide as high. Empire: flower, 5 round petals, open centre 0.27. "On the model: radius 0.03 m on each shoulder
  board, 0.036 m on the front of the hat. At game size: about 6 by 3 px."
- "Must read at game size": 1 the outline: fur hat, long coat that widens to the hem, rifle across the chest; 2 the
  coat colour, and the team colour on the hat top and both shoulder boards; 3 the trim rim round each board, and the
  black line round the whole figure.
- "May be left out at game size": 1 buttons, buckle, pouch, ear flaps as separate parts; 2 the shape of the marking,
  the badge on the hat front; 3 face, fingers, scarf folds.
- Shown: front and back in the game's camera for the three factions; a strip at game size; one pair "with skin,
  leather, wood, steel: 8 colours" for comparison.

## Sheet 3: "A test scene for Slice 1: a small street corner in palette A, in the game's camera"
- Its line: "Ground, two houses with an alley between them, and a wall to take cover behind. Sizes and layout are the
  Art lead's."
- House A: 6.1 by 4.4 m, eaves 2.6 m, ridge 4.7 m. House B: 4.6 by 3.8 m, eaves 2.6 m, ridge 4.3 m. Alley between the
  houses: 1.5 m wide. Wall for cover: 3.1 m long, 0.45 m thick, 1.0 m high. Street: 4.3 m wide.
- "The line rule": units: black line, outline 4.8 px, inside 2.1 px. Buildings and walls: black line, outline 3.6 px,
  edges 2.6 px, openings 2.0 px. Plants and grass: no black line, a darker tone of their own colour. "Widths are px on
  a 1080p screen and the Art lead's proposals. Units thicker than buildings: liked by Mustafa, final in the look proof
  test. No tree here: the tree style is open."
- "Palette A, as colour values" (base colour; in the sun; in shade):
  - Street: #B8AD98; #B8A683; #6E6E6E.
  - Grass: #A7B58F; #A7AD7B; #647367.
  - Stone (wall, plinth): #D0C2A2; #D0BA8C; #7C7B75.
  - Plaster (house walls): #E6DDC8; #E6D3AD; #898C90.
  - Timber (frame): #4A3A2E; #4A3728; #2C2521.
  - Roof tiles: #A98470; #A97E61; #655451.
  - "Sun colour #FFF4DC, shadow colour #98A2B8. Base colours: palette A. Sun and shade: the Art lead's."

## What this session takes from them, and what it does not
- Taken as inputs for the plan: the first soldier's sheet is the Conscript's shared tier-one uniform; its sizes; its
  four materials (which already fit the drafts' cap of 4 materials for a soldier with his weapon); the three
  factions' colours; the street corner's layout and sizes; palette A.
- Not decided here: anything about the look. One point differs from `docs/LOOK_V2.md` as it stands on main: the
  sheet's line rule draws units thicker than buildings, while LOOK_V2 §2 says "thick black outlines, especially
  strong on buildings". The sheet says this is final in the look proof test. The docs chat's carry of his look
  decisions settles which text stands; change-set 6 waits for it.
- Not yet received (from Part 4's list): a sheet per gun, the effects, the HUD pieces.
````
sha256 of the block: `2d541381b4d5053d0253d3448683273f948a1730159cdf9e91200afb1c388374` (5623 bytes).

## A8. His message of 2026-11-17, 19:59:06 UTC (his label: 20:20): three small things. Re-sent with one more item as A9
````text
My words of 2026-11-17, 20:20 (Amsterdam time): three small things. None of them holds change-set 0.

1. The two pictures. Attached now: s16_engine_soldier.jpg (the redrawn Sheet 16) and my SMG line drawing. Your card says they did not arrive with my go.
2. Advisor_log_2.md came along by mistake. It is the Advisor chat's working record, not an input for you. Do not read it, do not have an agent digest it, and put no part of it in the repo. If an agent has already started on it, stop that agent and delete its notes before the next push.
3. Your card's "Last update" says 20:25 Amsterdam, and its commit (cb0fd7b9) is from 20:13:23. Stamp times from the clock, never ahead of it.
````
sha256 of the block: `be3e892271c3984972953aa6cc7382d91e030ddc4d25ddb52f3f410541e5929d` (683 bytes).

## A9. His message of 2026-11-17, 20:03:57 UTC (his label: 20:58): four small things
````text
My words of 2026-11-17, 20:58 (Amsterdam time): four small things. None of them holds change-set 0.

1. The two pictures. Attached now: s16_engine_soldier.jpg (the redrawn Sheet 16) and my SMG line drawing. Your card says they did not arrive with my go.

2. Advisor_log_2.md came along by mistake. It is the Advisor chat's working record, not an input for you. Do not read it, do not have an agent digest it, and put no part of it in the repo. If an agent has already started on it, stop that agent and delete its notes before the next push.

3. A fourth Art text, attached: the Art lead's correction of 2026-11-15, 18:40, "The gun poses, restated from the sockets" (ART_BRIEF_CORRECTION_2026-11-15_1840_gun_poses.md). Save it with the other three.
   - It replaces section A1 of the addition of 2026-11-15 and the section "How the Conscript holds it" of the SMG brief of 2026-11-15, 17:35. It also corrects two lines of that brief: the socket names and the magazine. Like the other three it is the Art lead's proposal.
   - It agrees with my go: the right hand sits on socket_grip and the left hand on socket_support, the butt and the muzzle follow from the gun's file, and a shot's start is simulation data.
   - For change-set 0 take only its rule and the socket names. The pose numbers are for the body step (change-set 10).
   - Checked against the rifle file and the SMG brief: 14 of its 15 check points hold to 0.01 m. The one that differs: in the rifle's aiming pose the butt is given at U 1.44. By the file the butt end runs from 0.07 below to 0.03 above the grip socket, so its top edge is at U 1.39. The file wins, by the correction's own rule.
   - For the body step: in the rifle's aiming pose the left arm uses 0.59 of its 0.62, so little is left for recoil and the walk. The SMG's side magazine has to stay clear of the left forearm, as the correction says itself.

4. Your first card's "Last update" said 20:25 Amsterdam, and its commit (cb0fd7b9) is from 20:13:23. Stamp times from the clock, never ahead of it.
````
sha256 of the block: `dbc58ec975052e3ab3a200b438a6ab0d601993e9e5f21a430a7d14c40cd780c5` (2028 bytes).

## A10. The Art lead's correction of 2026-11-15, 18:40, "The gun poses, restated from the sockets": the Art lead's text alone, the copy of 3,244 bytes (his words of 2026-11-17, A11)
````text
CORRECTION: THE GUN POSES, RESTATED FROM THE SOCKETS. From the Art lead, 2026-11-15, 18:40.
For the builder chat. It replaces section A1 of my addition of 2026-11-15 and the section "How the Conscript holds it" of my SMG brief of 17:35, and it corrects two lines of that brief. Everything here is the Art lead's proposal.

THE RULE
A pose is three things: where the gun's grip socket sits, which way the gun points, and how far the chest turns. The right hand is on socket_grip and the left hand on socket_support. The butt and the muzzle follow from the gun's file. If the left hand cannot reach (the arm is 0.62 from the shoulder joint to the middle of the hand), the pose is wrong, not the gun. Positions are (L, F, U) in the soldier's frame. The "check" values are where the sockets land, for you to compare.

THE RIFLE (AMK9_v4_stage1.glb: support socket 0.452 ahead of the grip, muzzle 0.864 ahead, butt end 0.259 behind)
- Idle: grip at (-0.13, 0.22, 1.06). The gun points to his left and 42 degrees up, across the chest. Chest square.
  Check: support (0.19, 0.22, 1.38), muzzle (0.46, 0.22, 1.70).
- Aiming and firing, also while walking: grip at (-0.13, 0.21, 1.36). The gun points straight ahead, level. The chest turns 45 degrees to his right, so the left shoulder comes forward; the head keeps facing forward.
  Check: butt (-0.13, -0.05, 1.44), support (-0.13, 0.66, 1.39), muzzle (-0.13, 1.07, 1.44). The left arm is almost straight.
- Sprinting: grip at (-0.13, 0.22, 1.10). The gun points to his left and 25 degrees up, across the chest. Chest square, body leaning 10 degrees forward.
  Check: support (0.27, 0.22, 1.32), muzzle (0.62, 0.22, 1.54).

THE SMG (support socket 0.34 ahead of the grip and 0.034 lower, muzzle 0.64 ahead, butt end 0.16 behind)
- Aiming and firing: grip at (-0.12, 0.19, 1.36). Straight ahead, level. The chest turns 30 degrees to his right.
  Check: butt (-0.12, 0.03, 1.39), support (-0.12, 0.53, 1.33), muzzle (-0.12, 0.83, 1.40), the magazine's far end (0.14, 0.48, 1.40).
- Idle: grip at (-0.12, 0.19, 1.32). Straight ahead, tipped 15 degrees down. The chest turns 30 degrees to his right.
  Check: support (-0.12, 0.51, 1.20), muzzle (-0.12, 0.82, 1.19).
- Sprinting: grip at (-0.14, 0.18, 1.18). Straight ahead, tipped 50 degrees up. Chest square, body leaning 10 degrees forward.
  Check: support (-0.14, 0.42, 1.42), muzzle (-0.14, 0.56, 1.69).
Why the SMG's idle and sprint changed: turned to his left, the side magazine swings into the left forearm, and at belt height the left hand cannot reach the grip bump. The body-movement step still has to check that the magazine stays clear of his arms.

TWO LINES OF THE SMG BRIEF, CORRECTED
- Socket names: socket_grip, socket_support, socket_muzzle and socket_sight, as in the rifle file, plus socket_eject.
- The magazine: three straight pieces of 0.06. The first points straight out to the left, the second is bent 8 degrees towards the muzzle, the third 16 degrees. The far end then sits 0.025 nearer the muzzle and reaches 0.26 left of the bore line.

ONE WORDING
Where I wrote "shots leave from there": the muzzle socket is where the flash and the tracer start. The simulation's shot origin is its own data and is not read from the model.
````
sha256 of the block: `d685d08bd487caa5fc238af1d16e14bc9f57f45808ceb54f4b1d4537d601899b` (3243 bytes).

## A11. His message of 2026-11-17, 20:39:23 UTC (his label: 21:33): his answers to SIZE-0 and LOG-HISTORY, the correction file, the SMG line drawing
````text
My words of 2026-11-17, 21:33 (Amsterdam time): my answers to SIZE-0 and LOG-HISTORY, and two small things.

SIZE-0 (does change-set 0 go on at its present size, or is it split): (a). It goes on as one pull request.
- I accept the size. The 150 lines were planned for the model rules alone. My CHANGE 1 then put the record of my decisions and the plan's carry into this change-set, and the joint ruling answers a High.
- The new stop line: 1,200 written lines; copied lines do not count. Past it you stop and tell me in three lines, as you did now.
- Build my records and the plan's carry in commits of their own, apart from the rules and the joint list, so a split stays cheap if it is ever needed.
- If review round 1 returns a Critical or High on the rules or the joint list that one fix pass cannot close, split at that point: my records and the plan's carry go first as change-set 0, the rules and the joint list follow as change-set 0b. You need no new word from me for that; tell me in three lines when it happens.
- The review budget is unchanged: at most two review rounds, a third only for a Critical or High.
- No game code before change-set 0 is on main: that stands.

LOG-HISTORY (the five lines of the Advisor log's digest in this branch's history): (a). Leave the history; nothing is rebuilt. The lines hold work points only.

THE CORRECTION FILE
Two copies came with my message of 20:58. In the appendix, A10 is the copy of 3,244 bytes: the Art lead's text alone. The Advisor's check above the line is not the Art lead's and stays out of the appendix.

THE SMG LINE DRAWING
It follows in a message of its own. It is needed at change-set 8, not before. Until it arrives the SMG brief's numbers stand; they win over the drawing in any case.
````
sha256 of the block: `bc1c0a9fc96b6182c4bad312cbe957ccc975d8e89dfd216177b146c7f71586e0` (1754 bytes).

