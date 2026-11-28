# U4d build review: netcode-architect (read only)

Worktree: /home/user/Shamble-Frontlines/.claude/worktrees/wf_f54246ff-e0d-1 (uncommitted, base 097f8970)
Plan: u4-build-plan.md §5 U4d + §4.2, §4.10, §4.13, §4.15; advisor-u4.md; ADR 0004.

## Plan
1. Read plan §5 U4d section + §4.2/4.10/4.13/4.15 + carries 7-20, 36-38; advisor rulings.
2. git status / git diff in worktree; read each new/modified file.
3. Check: seam freeze kept; refusals outside hash; scenes; recordings; carries as worded; no U4e-U4h pull-forward.
4. Findings with file:line; verdict.

## Progress
- [1] Read plan §4.2/4.10/4.13/4.15, §5 U4d, carries 7-20/36-38 (u4s-carries.md), advisor-u4.md conditions 1-5 and U4s ruling cond 3 (freeze).
- [2] Diff: seam.rs (+button consts, TickInput spawns/teleports/attributes, Placement/AttributeChange/Attribute, layout completed within 12, HitVolumes soldiers/cover + hand Clone + Eq, VolumeBox, docs; non_finite removed), history.rs 1 MB cap (1<<20 / MAX_BYTES 4,536 = 231 ticks), grid.rs doc (carry 20), lib.rs re-exports. Freeze list (Command, Rewind, CoverVariant, EntityKind, EntityRecord, 5 grid fields): field shapes unchanged so far; HitVolumes not in freeze list.
- [3] world.rs read whole. Notes: refusals (cooldown/ammo/reload/rewind_outside/pool_full) and clamps only in reports, never events/hash (write_state excludes reports + `claimed`). `claimed` (last press counters seen, client's claim) outside hash but decides later edges (`!=` rule, world.rs:1035-1040); doc world.rs:24-30 redefines twin as counters one lower from forged press on -> vs §4.2 "holds what the server accepted, never what the client claimed" and §4.10 "everything else equal": reading for advisor + U4r carry (candidate MEDIUM). blend_key reset each step (ok). Rewind held() check per command -> None (carry 10 ok). HitVolumes.tick = own counter (carry 15 ok). Phase 8 build_volumes(tick). predict: no rng, no count, own snap on wire grid; -41 bites authority-only. Clamp f64, slack 0.0087 m.
- Scenes (scene.rs): SS ok vs §4.13 (48/3x16, 15 veh 4x20 parts, 300 proj, 384 effects, 68,571 us -> 700/s, health 1000 dmg 1, magazine 10,000). ES lacks ADR 0001's "10 reliable gameplay events a second" (no source in sim; candidate LOW). SS magazine 10,000 = ~686 s of fire: runs > ~11 min (S30?) run dry w/o reload presses (check marks -22 length).
- [4] tests read (world.rs D-1..D-10, carries 7/8/9/13/14/15/38, ray_cast, bites; replay.rs D-2, D-2 hash-map, D-7 pinned sizes 16,697,066 / 16,698,782 / 798,759 B; round trip 39 B; seam.rs s6 next_kind exhaustive match = carry 36 ok).
- Candidate findings so far:
  M1 SS magazine 10,000 = 685.7 s of fire; PM-0001-2 applies to S30 runs (-22 at U4f; bite fire-stops "after 10 minutes") -> clean SS soldier that survives stops firing at 11.4 min. scene.rs:105-107,127.
  M2 claimed counters / twin-for-press reading (world.rs:24-30, 480-481, 1035-1040) vs §4.2/§4.10/cond 1 words; needs advisor reading + U4r carry.
  M3 FLAG_FIRED reset per tick in step (world.rs ~1795) vs per command in predict (world.rs ~722) -> catch-up tick (2 cmds, first fires) mispredicts flags; D-3 covers 1 cmd/tick only.
  M4 4 large vehicles loop at 40-49 m (world.rs ~589) vs §4.13 "the 4 of at least 20 parts in the fight" (F113: in the fight, other 11 elsewhere); stand-in projectiles spread +-60 m (world.rs spawn_stand_in) vs F113 "in the fight".
  L: ES lacks "10 reliable gameplay events a second" (ADR 0001:1478); clamp reported for every command w/ far aim, not only shots (§4.2 "inside the shot phase"); carry 13 counting allocator -> storage-identity stand-in (route to U5 with PM-0001-34); 1 MB = 1<<20 vs F1's MB=10^6 (nit); PARENT field redundant (nit); held-back command's Rewind may fall out of ring (nit); non-finite ray -> Miss decided now (carry 45 nit).
- [5] ADR 0001:313-314 + catalogue :138: "parts, and the pilot, travel with their parent" -> build writes each VehiclePart record as a WORLD transform changing every tick (124 parts in SS) = F112's "breaks 1,000 B" case -> candidate HIGH (H1). Effect rows: EFFECT_LEFT decrements every tick -> 384 records change every tick; catalogue has no wire group for effect rows -> MEDIUM (state room-only or store end tick). FLAG_FIRED in the 8 flag bits toggles every shot vs ADR 0001:310-313 (flags change rarely; action starts are cosmetic events) -> fold into M3 (drop FLAG_FIRED fixes both).
- [6] Resumed after cut (2026-11-10). All 15 file sha256 match builder's report. Next: re-pin line numbers for H1/M1-M4/L, check freeze shapes vs base, recordings, carries 7-20/36-38 wording, U4e-U4h pull-forward.
- [7] Re-verified after resume (line numbers on the sha256 the builder reported):
  - Freeze: Command/Rewind/CoverVariant/EntityKind/EntityRecord(FIELDS 12) unchanged; grid doc only; TickInput fields added; non_finite dropped; layouts completed within FIELDS (Soldier 12, Part 6, Projectile 9, Wall 2, Effect 3). KEPT.
  - Refusals: cooldown/ammo/reload/rewind_outside/pool_full + clamps only in reports (world.rs:1132-1148, 1016-1022, 1832-1834, 1178); write_state excludes reports and `claimed` (world.rs:1645-1647). OK. `claimed` decides later edges (world.rs:1036-1040) -> M2.
  - H1 confirmed: place_vehicles world.rs:930-938 writes world pos; write_records :1568-1583; load_movers_into :1765-1777; volumes :1233-1239. Catalogue :138, ADR 0001:313-314, scout F112.
  - M1 confirmed: scene.rs:105-107,127; marks.json PM-0001-2 notes (S30 bite run past 10 min); generator no reloads replay.rs:433-438.
  - M3 confirmed: step clears FLAG_FIRED once per tick world.rs:1821-1824; predict per command :756; FLAG_FIRED set :1153; flags in soldier record :1532.
  - M4 confirmed: world.rs:587-591 radius 40+3v m for every vehicle; large = v<4 -> 40-49 m; spawn_stand_in ±60 m :1438-1442.
  - Effects: EFFECT_LEFT seam.rs:202 counts down world.rs:1468-1474, record :1637 -> every effect record changes every tick.
  - ES: no source for "10 reliable events/s" (marks PM-0001-2 ES band; only L2 PM-0001-12 selects ES) -> LOW.
  - 1 MB: history.rs MAX_RING_BYTES 1<<20 vs marks.json 1,000,000 -> LOW.
  - TC plates vertical (thin in y) facing the shooter: OK for -51 (a horizontal plate at 30 m would put the underside hit ~23 cm off at a grazing ray); no finding.
  - Carries 7-12, 14-20, 36-38 met; 13 partial (storage identity, no counting allocator); 18 vacuous (no new tick constant).
  - No U4e-U4h pull-forward beyond ray_cast's non-finite -> Miss (carry 45, U4h states it): nit.
- Verdict: REVISE (H1), with M1-M5 fixed or routed.
DONE
