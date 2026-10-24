Task: R7 legacy harvest, reader for area CL (core look pipeline, shamble-core.js 233-559)
Checks: HEAD 698fbad = 48ffb51 + checkpoint files only (git diff --stat: .checkpoints only). docs/VISION_V2.md does NOT exist at 48ffb51 (find: none).
Scope finding: comic SFX code lives in shamble-game.js:46-68 (core:420 header is orphaned, followed by CHAR_DEFAULT). Procedural canvas textures: core:187 (CD range), core:987-1294 (CM range); in-range only DataTexture ramps.
CL1 flags+ink constants: core:239 NPR/COMIC, 240 INK 0x11121a, 241 WORLD_OUTLINE .09, 242 CHAR_OUTLINE .8, 294 OUTLINE_GROUND_Y .02; game:3379 mutable-let shim writes NPR/COMIC/CHAR_OUTLINE. D (ART_DIRECTION §1/§3). redesign (look pending). LBP true
CL2 cel ramps: core:244 _toonGrad, 247 _mkGrad, 248-251 skin/cloth/metal/lacquer, 267 gradForKind; RGBFormat DataTexture (gone in later three). comment 243 says 3 bands, data 2. D ART_DIRECTION §2 shadow .31 light .94. redesign LBP
CL3 skin rim: core:254 RIM_*, 255 attachSkinRim (onBeforeCompile GLSL injection, customProgramCacheKey). C. redesign LBP
CL4 toonify core:278 (per-mesh MeshToonMaterial, nature gain bake). M via CS12 (first merge attempt merged nothing). redesign LBP
CL5 applyVillain core:268; in-game dormant (game:3211 no elite D6; game:3364 loop never fires), used by editor:288 Character Lab. AUDIT:17 visor not a bug. port TS LBP. overlap CC
CL6 ink hull makeOutlineMat core:295 + addHull core:389. M STATUS:84 ~120 dc/soldier; 3tc report 5,843 dc at 48. redesign LBP
CL7 ensureOutlineNormals core:333, _outlineDirs 311, _oNCache 310 (cap 4000 clear-all). M TECHNICAL:50 ~33 ms boot ~1 ms/soldier. redesign LBP
CL8 stylizeScene core:397, stylizeOne 411; game:3363 call; PR#8 dc 1992/2406/3502 -> 1992; 3tc parked bots fix. M. redesign LBP
CL9 mergeStaticGroup core:360, _matSig 356, scratch 353; callers game:577,583,598,604,608. M CS12 1,611->370 meshes; 398->202 village; 284->162 manor; +150 ms; +12% tris. TECHNICAL:249 stale. port TS LBP
CL10 rig defs CHAR_DEFAULT core:421, CHAR_MALE 463, CHAR_FEMALE 508. D TECHNICAL:133,150. port TS LBP. overlap CC
CL11 hit shape CHAR_DEFAULT.hit core:454?, _BODY_HIT 462 -> BODY core:1482. rewrite Rust, LBP false
CL12 comic SFX game:46-68, 2959, 3243; COMIC core:239. M AUDIT B2 1.1 ms -> CS1 0 built per kill. port TS LBP
CL13 procedural canvas textures core:187 grass (Math.random monkeypatch, no finally), 987-1294 kit. D TECHNICAL:54 512x512. port TS LBP
Fix: CL11 hit at core:455 (CHAR_DEFAULT.hit), _BODY_HIT 462. LOOP_V2 §9 greybox first (look not needed in v0); §11 "character rework" not in v0; §7 feel kit as data.
DONE
