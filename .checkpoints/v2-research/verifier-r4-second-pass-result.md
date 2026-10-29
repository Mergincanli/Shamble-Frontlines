# R4 wave 2 second pass: the verifier's report, verbatim

Workflow run wf_50c96d66-3f6 (script `v2-research-r4-second-pass-run1.js`, prompt `prompts/verifier-r4-second-pass.txt`), returned 2026-10-29 (about 21:28 UTC). Notes: `verifier-r4-second-pass.md`.

VERDICT: PASS WITH NOTES. Everything the spot-check raised is now fixed in the file, every new quote holds at its source, and what remains is only nits.

I skipped the gate and the symbol checks, as the task said. I read web pages with WebFetch only, on allow-listed domains. My notes, ending with DONE, are in /home/user/Shamble-Frontlines/.checkpoints/v2-research/verifier-r4-second-pass.md.

**Item 1: the spot-check's findings.** Closed. All 12 critical-read items and all 3 wrong or unsupported claims are fixed. None was cut and none was turned into an open question.
- **Item 1, parry "any two shapes":** fixed. R4-PHYS-F20 is corrected at :173. R4-PHYS-REC1's reason is reworded at :672-674, and the new mark 6 is at :700-705. R4-PHYS-REC3 is reworded at :738 and R4-PHYS-REC4 at :759.
- **Item 2, tunnelling test speed:** fixed at :686-691, now 13.3 m/s.
- **Item 3, wrong §6.1 traces:** fixed in six Serves lines (:809, :836, :856, :868, :966, :996). No §6.1 citation is left except the correction note itself.
- **Item 4, tiled bake:** fixed at :1021-1034. Tiling is now conditional, and the editor pointer at :1170 says the same.
- **Item 5, the missing Detour option:** fixed inside existing text: R4-NAV-F8 at :299-305, the option A cell at :617, R4-NAV-REC1 at :794-797, the TL;DR at :42 and a pointer at :1144. The owner's reason for not adding a new option column is sound: no source read weighs the C-capable Wasm targets for the sim.
- **Item 6, absences labelled confirmed:** fixed. R4-PHYS-F27 (:225), R4-NAV-F16 (:358) and R4-LOS-F5 (:500) now read "inferred".
- **Item 7, turning vehicles:** fixed at :750-753, and the mark adds turns at :763.
- **Item 8, 2D avoidance:** fixed at :846-848 and :859.
- **Item 9, incomplete "Rests on" lists:** fixed at :721-722, :740, :904-905 and :1013-1014.
- **Item 10, autostep cost warning:** fixed at :66, :677 and :698.
- **Item 11, vehicle count:** fixed at :837 and :1116.
- **Item 12, nits:** fixed at :268, :531, :142 and :428.

**Item 2: the rewritten recommendations.** Closed apart from three nits. Each rewritten recommendation follows from its findings and names them in "Rests on". The only finding id the fix added to any "Rests on" list is R4-PHYS-F13 (in R4-PHYS-REC4), and it checks out at its source. No two pass marks contradict each other: R4-PHYS-REC4, R4-NAV-REC2 and R4-OBST-REC1 all use 8 vehicles, and R1's P1 scene still has 4, which the file openly leaves to the synthesis (R1-XU-OQ1, :1116). The three nits:
- **(n2) The tiling condition is worded three ways.** The heading at :1022 says "mark 1 fails, or a measured Test delay needs it". The Serves line at :1032 says "only if R4-NAV-REC1 mark 1 fails". The pointer at :1170 says "only if a full bake misses mark 1". They should say the same thing.
- **(n3) R4-NAV-REC1 still overstates the source.** At :795 it says Detour "cannot be linked into" the browser build. The source only says "There is no C/C++ toolchain for this target. While interop is theoretically possible…". The TL;DR and option A were already softened; this line should match them.
- **(n4) One citation is too broad.** R4-OBST-REC2 at :1014 cites "LOOP_V2 §3 (team-funded rebuilds)". The only rebuild in §3 is a harvester's (`LOOP_V2.md:32-35`); a structure being rebuilt is our reading.
- (n5, trivial) At :868, "offline too" reads as if it were §6.2's wording; it is our gloss.

**Item 3: numbers.** Closed. All are sourced or redone:
- The tunnelling speed: 6.3 × 1.62 × 1.3 = 13.27 m/s, which is 0.221 m per tick at 60 Hz and 0.442 m at 30 Hz, both above the 0.2 m wall.
- The P1 vehicle count, 8: proposed, and traced to R4-PHYS-REC4.
- 18 ways to move plus `TRAVEL_INVALID`, and the 12 Rapier releases counting both ends and one beta: both checked at their sources.

**Item 4: open questions.** Closed. The fix added none (still 23), and the reference it added to R4-NAV-OQ4 claims nothing new.

**Item 5: header, totals and part 7.** Part 7 and the totals match the file. The header label does not.
- **Totals match:** 62 findings, 18 recommendations, 23 open questions; 47 sources read (37 official, 8 shipped game, 2 other); 32 not read.
- **Part 7 matches:** 49 rows, verdicts 46 confirmed, 2 wrong, 1 unsupported, and final states 41 no change plus 8 fixed.
- **(n1) The header label is out of date.** I counted the distinct R4 findings named in "Rests on" lists with a script: 51 at abdf477, 52 at HEAD (R4-PHYS-F13 added). R4-PHYS-F13 is now confirmed at its source, so the check stands at 50 of 52. The header still says "49 of 51". Part 7 (:1198) explains the change, but the label does not reflect it.
- (n6, trivial) Source 47's URL is marked "inferred". It opens at that URL, so "inferred" can go.

| id | claim, short | source | verdict | location and exact words |
|---|---|---|---|---|
| R4-PHYS-F20a | `cast_shapes` can return an error for a pair | docs.rs parry3d fn.cast_shapes (0.31.1), source 47 | confirmed | signature: ") -> Result<Option<ShapeCastHit>, Unsupported>"; "`Err(Unsupported)` - This shape pair is not supported" |
| R4-PHYS-F20b | one linear and one nonlinear swept test, an overlap test and ray casts | docs.rs parry3d query index | confirmed | "Computes when two shapes moving with translation and rotation will first collide."; "Tests whether two shapes are intersecting (overlapping)."; "Traits of objects which can be transformed and tested for intersection with a ray." |
| R4-PHYS-F2 | autostep cost warning | docs.rs rapier3d 0.36.1, `autostep` field | confirmed | "Note that autostepping is currently a very computationally expensive feature, so it is disabled by default." |
| R4-PHYS-F22 | Parry 0.26 went with Rapier 0.32, January 2026 | dimforge.com 2025 review; CHANGELOG | confirmed | "Starting with rapier 0.32 and parry 0.26, most of their API and internals using the nalgebra linear algebra library" / "have been modified to use glam instead."; "## v0.32.0 (09 Jan. 2026)" |
| R4-PHYS-F15 | 12 releases, counting both ends and one beta | Rapier CHANGELOG | confirmed | 12 headings from "## v0.36.1 (4 October 2026)" to "## v0.30.1 (17 Oct. 2025)", including "## v0.35.0-beta.0 (02 August 2026)" |
| R4-PHYS-F13 | nonlinear casts cost more (now in REC4's basis) | rapier.rs scene_queries | confirmed | "Nonlinear shape-casting is more expensive than the linear one," "so it is recommended to use `cast_shape` whenever the shape doesn't rotate." |
| R4-NAV-F2 | height, climb and radius in voxels; cell size in world units | Recast.h, rcConfig | confirmed | walkableHeight, walkableClimb and walkableRadius each "[Units: vx]"; cs "The xz-plane cell size to use for fields. [Limit: > 0] [Units: wu]" |
| R4-NAV-F8 | two Wasm targets that compile C/C++ | rustc book, wasm32-unknown-unknown | confirmed | "`wasm32-unknown-emscripten` - for web-based use cases the Emscripten toolchain is typically chosen for running C/C++."; "`wasm32-wasip1` - the wasi-sdk toolchain is used to compile C/C++ on this target and can interop with Rust code." |
| R4-NAV-F15 | foldhash is an optional dependency of hashbrown; no default hasher named | docs.rs hashbrown 0.17.1 | confirmed | "foldhash ^0.2.0 _normal_ _optional_"; "Default hash builder for the `S` type parameter of HashMap and HashSet."; no default hasher named on the page |
| R4-FLOORS-F5 | 18 ways to move plus `TRAVEL_INVALID` | aasfile.h | confirmed | "#define TRAVEL_INVALID 1 //temporary not possible" through "#define TRAVEL_FUNCBOB 19 //func bob": 19 consecutive codes |
| Mark-speed | fastest soldier about 13.3 m/s | shamble-core.js:67, :134; shamble-game.js:2756, :2774; LEGACY_INVENTORY.md:313 | confirmed | `moveSpeed:6.3`; Warrior `sprintMult:1.62`; `cs=T.moveSpeed*unitOf(2).sprintMult*1.3`; `if(ps.banzai) sp=Math.max(sp,1)*1.3`; "T.sprintMult 1.4286 (never read: sprint uses the per-unit value)" |
| Mark-bots | bots move at 6.3 m/s at most | shamble-game.js:2909; shamble-core.js:79 | confirmed | `T.enemyMoveSpeed*eu.speedMult` with `enemyMoveSpeed:4.2` and the highest `speedMult` 1.5: 4.2 × 1.5 = 6.3 |
| Arith-1 | 0.22 m per tick at 60 Hz, 0.44 m at 30 Hz | arithmetic | confirmed | 13.2678 / 60 = 0.221; 13.2678 / 30 = 0.442 |
| Trace-§3 | "Bots fill empty slots."; "up to 24v24" | VISION_V2.md:48, :44 | confirmed | "Bots fill empty slots."; "**The goal:** 12v12v12 or 24v24." |
| Trace-§6.4 | §6.4's own words | VISION_V2.md:71 | confirmed | "big and multi-part, with their own weapons and abilities (leech, marking);" |
| Trace-LOOP | the LOOP_V2 sections added to "Rests on" | LOOP_V2.md:22, :72-73, :116-117, :32-35 | confirmed (§3: harvester rebuilds only, n4) | "Everyone spawns inside their own barracks"; "Its rockets knock infantry back"; "structures players can enter `[LATER]`"; "A destroyed harvester is rebuilt with money from the whole team" |
| Trace-P1 | R1's P1 scene has 4 big vehicles | r1-simulation-core.md:2752 | confirmed | "plus 4 big vehicles and 200 live projectiles" |

**Counts:** 17 checked: 17 confirmed, 0 wrong, 0 unsupported.

**Only nits?** Yes: n1 to n6. None changes a recommendation or a pass mark.

**Header label the file should carry:** "spot-checked (50 of 52 key claims)", plus a note that the second pass on the owner's fixes ran on 2026-10-29 and closed with nits only, as R3's header does.

Files:
- /home/user/Shamble-Frontlines/docs/research/r4-physics-navigation.md
- /home/user/Shamble-Frontlines/.checkpoints/v2-research/verifier-r4-second-pass.md

**Decisions needed:** none.

**For Mustafa:** nothing.
