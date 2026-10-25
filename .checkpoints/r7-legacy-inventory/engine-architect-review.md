Task: R7 legacy inventory - engine-architect review of module and Rust <-> TypeScript split (rows CD CS GW GN GC GP + every 'rewrite in Rust' row)
CD1-CD21: read in full; spot-checked T/TEAMS/TUNE literals core:66-107 (sed). No issue yet for CD (pending cross-checks).
CS1-CS11: read in full; checked SIM block core:1474-1544 and syncSimWorld game:1440-1462; NaN claim CS6 verified by reasoning. Note: syncSimWorld reads o.mesh.rotation.y/position (render transforms -> collision). loop game:3053: hitStop/paused/shopOpen/tutorialOpen gate update(dt) - check GW covers.
GW1-GW21: read in full; checked resolveObstacles/surfaceHeightAt/tickVertical/blastReaches game:2526-2543. Candidate finding: GW13 resolveObstacles reads o.mesh.rotation.y and o.mesh.position (render transform -> collision), not flagged as hidden coupling; GW16 covers syncSimWorld similarly (reads o.mesh.rotation.y at :1452) - check wording.
GN1-GN14: read in full; checked update() NAV.budget game:2671, navBuild callers (1107, 3368), enemyClimbSteer 1600-1606. Bot sim position = en.group.position (THREE Group) - no row says positions live on render nodes; check GC2/GP.
GC1-GC19: read in full; verified GC13 throw origins game:1405-1406,1415,1420-1421,1733,1758-1760,1797-1800,2856-2859 (grenade sim position = g.mesh.position; gravity 20 hardcoded). Captured in GC13.
GP1-GP16: read in full; verified wall-clock reads game:65,957,2173,2253,2674,2781,2947 (957 tbox bob, 2781/2947 jug anim only - cosmetic); getMuzzle callers 1142,1406,1472,1733,1760; animation (271-490) reads but never writes group transforms (no root motion).
Rust rows elsewhere: CL11, CM11, CM13, GE5, GE8 read in full.
FINDINGS so far:
F1 should GW15: dead buildings stay in losMeshes (rebuildLOS game:552 adds every b.mesh) and sink ~1.43 s to 15% height (game:2944, dt*0.7, 0.85h); blastReaches raycasts the mesh via matrixWorld (r128 Mesh.raycast tools/node_modules/three/build/three.js:8290-8305), refreshed by renderer.render (:17867) -> cosmetic sink + render timing decide blast reach; bullets ignore dead buildings (game:1450 s.alive=b.alive). Not in GW15.
F2 should GW13: resolveObstacles takes collider orientation/centre from render mesh (o.mesh.rotation.y, o.mesh.position, game:2526) and THREE.Box3 o.box; agents' authoritative positions are THREE scene nodes (player.position, en.group.position: callers 2780, 2933). Not stated in GW13.
F3 should GE8: presentTeam contested->null rule not in LOOP_V2; §4 steal = hold E, contest unspecified; reason overstates 'LOOP_V2 §4 keeps hold-to-steal'.
F4 should GP2: 'Add rebinding' = feature no VISION §6 need asks for.
F5 nit GN5: 'seeded RNG for deterministic replays' ties need to [LATER] replays (§6.8).
F6 nit CS5: 'maybe full 3D rotation' speculative; EDITOR_V2 §3 needs any Y rotation (120 deg symmetry, 15 deg snap).
F7 should CS8: add list of authoritative state living on three.js objects (player.position/en.group.position moved in place 2780/2933; facing player.rotation.y set from aim 2710, read by shield 1514 and playerFireOrigin 1420; grenades g.mesh.position 2857; o.mesh.rotation.y 1452/2526; Box3 colliders). Also nit: CS8 v2_note 'once per shot' vs frame guard.
F8 should CS8+CM13 evidence: STATUS.md:118 561 B is stale (three-teams-core.md:31 says array removed; game:1458 builds none).
F9 blocking GW16 evidence ':1457 builds none' -> houses loop is :1458 (1457 = const rampHi).
F10 should GE5: tech-income halving (game:2434) is replaced by LOOP_V2 §4; Rust mark keeps it silently.
F11 should GP14 v2_note 'Port the order...' lists an order that is not legacy (legacy: timers, economy+objectives 2692-2698, aim 2707, player shots 2722-2731 BEFORE movement 2748-2780, grenades, re-sync 2862, bolts, bots 2893 (steer->fire->collide), sink, jugs, FX) and keeps re-sync that GW16 drops.
F12 should CD5 lbp False while GP4 (same camera) True.
F13 should page 'Not covered' list: coverage-final.cjs regex lookbehind (?<![\w.#x/:]) skips ':848-851' style refs -> e.g. damageHarvester (GE6 :848-851), _fireO (GC5 :1419), _occHit (GW17 :2556-2558), _arcGeo (GC13 :1702-1705) wrongly listed as 'neither'.
F14 nit CS1 'repo-wide grep finds no reader' - tools/tuning-dump.js:12 lists BTN (dump only).
