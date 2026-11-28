# Testlines for change-set 4's plan (the main session's notes for his point 8, a to g)

_Written by the main session, 2026-11-25, from a read-only copy of his public sandbox Mergincanli/Shamble-Testlines,
branch `claude/sleepy-edison-ygmwfe` at commit be608cad (3c07269 when his Advisor read it): its `README.md`,
`src/params.ts` (the one table of every tuned value) and `src/exporter.ts` (what the Export holds). The copy sits in
the session's scratch folder, outside this repository. Looked at; no code copied; **no value taken**: the start values
in `params.ts` are the sandbox's, and by his words "The values come from my Export and from nothing else". Names, units
and meanings only. His words: point 8 of `mustafa-2026-11-25-takeover.md`; SLICE-FEEL in `docs/DECISIONS.md`
(2026-11-19, 13:36:37 UTC)._

## 1. What the Export is (from `src/exporter.ts`)
One JSON document. Its top-level names: `format` ("testlines-export"), `format_version` (1), `what`, `note`, `build`,
`build_commit`, `camera_profile`, `exported`, `exported_utc`, `warnings`, `values`, `changed_from_start`, `derived`,
`constants`, `measured`, `old_game_terms`.
- `values` holds one object per group, each value under a name that carries its unit. The groups: `simulation`,
  `measuring_grid`, `metrics_room`, `soldier`, `movement`, `jump`, `stamina`, `pushing`, `smg`, `rifle`, `shotgun`,
  `grenade`, `hornet`, `aim_down_sights`, `camera`, `camera_framing`, `mouse`. A slider on "auto" is written as the
  word "auto"; numbers are rounded to 3 decimals or the slider's own digits.
- `old_game_terms` holds the same values under the old game's names, in the old game's units (radians where the old
  game used radians): for example `T.moveSpeed`, `UNIT_TYPES.conscript.sprintMult`, `T.accel (speed-up)`,
  `T.accel (stop)`, `T.gravity`, `T.jumpV`, `T.stepUp`, `EJUMP_REACH`, `STAM.max`, `STAM.regen`, `STAM.lockUntil`,
  the camera's `T.cam...` and `T.look...` names, and each gun's `WEAPONS.<id>.<field>`.
- `warnings` holds a line when the step rate is not 30; `values.simulation.step_rate_hz` holds the rate itself.
- `changed_from_start` lists every value that differs from the sandbox's start value, with both.

## 2. His seven points, each with what the plan does
**a. Read the sandbox; copy no code.** Done as above. The change-set's report names the commit that was read.

**b. The values come from his Export only.** The plan asks him to paste the Export's JSON (asked in the chat on
2026-11-25). A script saves it whole from the chat's transcript, never retyped:
`content/source/testlines-export.json`, with `content/source/testlines-export.manifest.txt` beside it (the file's
byte count and sha256, the message's time and id, and the Export's own `build`, `build_commit`, `exported_utc` and
`camera_profile`). If his message holds words around the JSON, the script takes the one JSON document in it, checks
that it parses and that `format` and `format_version` are the two above, and records the message's own bytes and
sha256 as well. The script and both files are committed. Nothing is read off a screenshot.

**c. One table, and the old game's terms compared by script.**
- The table is a data file, `content/source/testlines-map.json`: one row per value Slice 1 uses, with the value's
  place in the Export (`values.<group>.<name>`), its place in the game's content (file and field), its unit, the
  change-set that uses it, and its home (point g). The docs show the same table, printed from that file by the
  script, so the table people read and the table the check uses cannot drift. Section 3 below is its first draft,
  names and units only.
- The comparison: the script reads the Export's `old_game_terms` and `docs/legacy-tuning.json` and prints every
  difference. How a name is looked up: `T.x` is `tables.T.values.x`; `STAM.x` likewise; `EJUMP_REACH` is the number
  `tables.EJUMP_REACH.values`; `UNIT_TYPES.<id>.<field>` and `WEAPONS.<id>.<field>` are the entry with that id in
  those tables' lists; `T_CHARGE.nade.<field>` the nested value; a name with a note in brackets (`T.accel
  (speed-up)`, `T.accel (stop)`, `T_CHARGE.nade.pitch (deg)`) is compared with the old value under the name before
  the bracket, converted as the note says. The output has three lists: equal, different (the old value, his value),
  and names with no counterpart in the legacy file. **The "different" list is his tuning: it goes to him as it is
  printed, and nothing becomes the starting table before he confirms it.** A value that has no old name at all (the
  body radius, the push strength, the sprint-to-fire delays, the active-reload window) is listed apart, as "his
  value, no old number".

**d. A script with a `--check` mode.** `content/source/testlines.mjs` (Node's built-in modules only), four modes:
`save` (b), `compare-old` (c), `write` (fills the game's content files from the saved Export through the table, so
no value is typed by hand) and `--check` (fails, naming each row, when a content value differs from the saved
Export through the table, when a row names a place that does not exist, or when the saved Export's bytes do not
match the manifest). `--check` compares only rows whose change-set has landed (the row's `since`). So that it runs
without a CI edit, a plain test in a job CI already runs calls the same check (which test, with the content crate's
design); the request to run the script's own `--check` in CI goes to the CI Builder chat with the generators' check
(3A-GEN-CHECK). Its planted defect: one content value changed by 0.001 must fail it, and so must one byte changed
in the saved Export.

**e. Not taken.** Section 4 lists each by name with one line. None enters a change-set without his word in that
change-set's plan.

**f. 30 steps a second.** `save` refuses to mark an Export usable when `values.simulation.step_rate_hz` is not 30 or
`warnings` is not empty: the chat then tells him before any value is used, with the warning's own line.

**g. Where he tunes once change-set 4 exists.** His question to decide (TUNE-HOME, section 5): the table has a "home"
column, one home per value.

## 3. The table's first draft: names, units and the change-set that uses each (no values)
"Content" names are proposals; the content crate's design fixes them. A duration stays in seconds in the content
(the build plan, Part 6: "every duration is data in seconds") and the simulation turns it into its own time unit
when it loads.

| Export (`values.`...) | Unit | Old game's name | What it is | In the game's content (proposed) | Used from |
|---|---|---|---|---|---|
| `simulation.step_rate_hz` | steps/s | (FIXED_DT) | the sandbox's step rate | not content: checked to be 30 (point f) | 4 |
| `movement.walk_speed_m_per_s` | m/s | T.moveSpeed | walking speed | `movement.walk_speed_m_per_s` | 4 |
| `movement.sprint_multiplier` | times | UNIT_TYPES.conscript.sprintMult | sprint speed as a share of walking | `movement.sprint_multiplier` | 4 |
| `movement.speed_up_time_to_90pct_s` | s | T.accel (speed-up) | how fast he reaches his speed | `movement.speed_up_time_to_90pct_s` | 4 |
| `movement.stop_time_to_10pct_s` | s | T.accel (stop) | how fast he stops | `movement.stop_time_to_10pct_s` | 4 |
| `movement.air_control_pct` | % | none (old game: full) | steering in the air | `movement.air_control_pct` | 4 |
| `movement.body_radius_m` | m | none (old game: 0.7 m against obstacles) | his body against walls and ledges | `movement.body_radius_m` | 4 |
| `jump.jump_reach_m` | m | EJUMP_REACH | the highest ledge a jump reaches (START-RULES) | `movement.jump_reach_m` | 4 |
| `jump.gravity_m_per_s2` | m/s² | T.gravity | gravity | `movement.gravity_m_per_s2` | 4 |
| `jump.step_up_m` | m | T.stepUp | what he walks onto without jumping | `movement.step_up_m` | 4 |
| `stamina.stamina_on` | yes/no | none (old game: on) | whether sprinting drains stamina | `movement.stamina_on` | 4 |
| `stamina.sprint_time_from_full_s` | s | STAM.max | sprint time on a full bar | `movement.sprint_time_from_full_s` | 4 |
| `stamina.refill_time_from_empty_s` | s | STAM.regen (as 1 over it) | time to refill | `movement.stamina_refill_time_s` | 4 |
| `stamina.sprint_returns_at_pct` | % | STAM.lockUntil (as a share) | when sprint comes back after running dry | `movement.sprint_returns_at_pct` | 4 |
| `pushing.push_strength_per_s` | 1/s | none | how stiffly two soldiers push apart (START-RULES) | `movement.push_strength_per_s` | 4 (felt from 5) |
| `pushing.push_radius_m` | m | none | body size for pushing | `movement.push_radius_m` | 4 (felt from 5) |
| `aim_down_sights.ads_speed_multiplier` | times | T.adsMoveMult | walking speed while aiming (SLICE-FEEL) | `movement.ads_speed_multiplier` | 4 |
| `aim_down_sights.ads_button` | hold or toggle | adsMode | how the aim key works | the page's input setting | 4 |
| `aim_down_sights.ads_camera` | Flatten, Slide or Still | T.adsCam | how the camera reacts to aiming | the camera's data | 4 |
| `aim_down_sights.ads_flatten_floor_deg`, `ads_slide_reach_m`, `ads_dead_zone_scale`, `ads_camera_ease_per_s` | °, m, times, 1/s | T.foxPanMax for the reach; the others: old behaviour, no table name | the aiming camera's numbers | the camera's data | 4 |
| `aim_down_sights.sight_line` | yes/no | old behaviour | the line from the gun to where the shot stops | the page (it needs a gun) | 8 |
| `camera.height_m`, `distance_m`, `angle_override_deg`, `turn_deg`, `lens_vertical_fov_deg` | m, m, ° or auto, °, ° | T.camHeight, T.camBack, none, T.camYawBase, T.fov | the camera's place and lens | the camera's data | 4 |
| `camera_framing.` (16 values: the two follow rates, the dead zone, the edge push, the look-ahead's five, the look ranges, the three camera shifts, the turn rate) | 1/s, shares, m | T.camLerp, T.lookLerp, T.camDeadX, T.camDeadY, T.camPushLerp, T.lookAhead..., T.look..., T.pos..., T.camYawLerp | how the camera follows and frames | the camera's data | 4 |
| `mouse.camera_turn_rad_per_px` | rad/px | T.camYawSens | turning the camera with the mouse | the camera's data | 4 |
| top-level `camera_profile` | a name | (the old presets) | which profile the Export was made on | recorded in the manifest; the panel's start preset | 4 |
| `smg.` and `rifle.`: `fire_interval_s`, `damage`, `magazine`, `reload_time_s`, `spread_rest_rad`, `spread_rest_ads_rad`, `spread_per_shot_rad`, `spread_max_rad`, `spread_recovery_rad_per_s`, `range_m`; `smg.falloff_start_m`, `falloff_end_m`, `falloff_min_share` | s, points, rounds, s, rad, rad/s, m, shares | WEAPONS.shabzon.x, WEAPONS.amk.x | the two guns of change-set 8 | the weapon tables | 8 |
| `smg.sprint_to_fire_delay_s`, `rifle.sprint_to_fire_delay_s` | s | none | the wait after a sprint before firing or aiming (SLICE-FEEL) | the weapon tables | 8 (the rule's seat in the simulation: 4 or 8, the plan says) |
| `smg.` and `rifle.` `active_reload_window_start_pct`, `active_reload_window_width_pct` | % of the reload | none | the simple active reload (SCOPE-RELOAD) | the weapon tables | 8 |
| `smg.recoil_view_kick`, `rifle.recoil_view_kick` | a strength | WEAPONS.x.recoil | the view's kick per shot | lane 2's feel data | 11 |
| `pushing.dummy_health`, `pushing.dummy_down_time_s` | points, s | UNIT_TYPES.conscript.maxHealth, T.enemyRespawn | the dummies' health and respawn | the soldier's and the room's data | 8 |
| `shotgun.` (as the guns, with `pellets`) | as the guns | WEAPONS.bagpipe.x | the shotgun of change-set 9 | the weapon tables | 9 |
| `grenade.grenades_carried`, `grenade_refill_s`, `throw_speed_max_m_per_s`, `launch_angle_deg`, `grenade_gravity_m_per_s2`, `bounce_kept`, `slide_kept`, `fuse_s`, `blast_radius_m`, `blast_damage` | count, s, m/s, °, m/s², shares, s, m, points | T.grenadeMax, T_CHARGE.nade.x, T.grenadeRadius, T.grenadeDamage | the frag grenade of change-set 9 | the weapon tables | 9 |
| `grenade.camera_kick_strength`, `camera_kick_range_m`, `camera_kick_stiffness`, `camera_kick_damping` | a strength, m, spring numbers | T.camKick... | the view lurching from a blast | lane 2's feel data | 11 |

Read with it: rows "Used from 8, 9 or 11" enter the content with that change-set, on that change-set's plan; change-set
4 writes only the rows marked 4. Lane 2's rows (11) are a request to lane 2, not lane 1's files.

## 4. Not taken (his point e): each by name, one line
None of these enters a change-set without his word in that change-set's plan.
- **The Hornet** (`values.hornet`, 26 values, and `constants.hornet`): a tier-one Empire vehicle; not part of Slice 1,
  nothing of it is built there (his words of 2026-11-25); its numbers are a sketch.
- **The explosion picture** (`grenade.explosion_look`, `explosion_play_time_s`, `explosion_width_m`,
  `explosion_pull_to_camera_m`, `fragments`, and `constants.explosion_sheet`): his own sprite sheet and how the sandbox
  draws it; a look matter, lane 2's track and his decision.
- **The metrics room** (`values.metrics_room`, 16 values; `derived.metrics_room_built`;
  `constants.metrics_room_layout`): the sandbox's measuring room; the game's test scene is the build plan's street
  corner and training yard.
- **The measuring grid** (`values.measuring_grid`, 5 values; `constants.grid_look`): the sandbox's prototype grid and
  its colours.
- **The sandbox's own session choices** (`soldier.faction`, `soldier.gun`, `pushing.dummies_walk`): what he had
  selected when he exported, not tuning.
- **Sandbox rules that differ from the old game** (the README's own lists), each stays out until he says so:
  - shots aim at the point under the cursor or fly level (`soldier.shots_aim_at`): open in the game (LOOP_V2
    section 9);
  - spread scatters sideways only (the old game: up and down as well);
  - a click in the gap between two shots is kept if the gap has at most 0.15 s left (the old game dropped it);
  - an R press within 0.25 s of a reload that started by itself counts as the press that started it;
  - the sight line ends where the shot would stop, with the sandbox's own look (the old line ran to the aim point);
  - the aiming camera's Flatten tips toward the point the cursor marks in the resting view and never so far that
    his feet leave the screen (the old game: its own 28.5° floor only);
  - the camera profiles Original, Experimental and Indie, and the old presets rescaled to the stand-in's size;
  - grenades: no charge; thrown at the cursor with an arc preview; bounce off blocks and the ramp and are swept;
    roll at a steady slowdown; flight and fuse in steps of 1/120 s; three in hand, one back every 5 s; lobs up to
    80° when the plain arc is blocked;
  - the shotgun's falloff sliders (the old Bagpipe had none), and its reload filling both shells at once
    (`docs/UNITS.md` has them one by one);
  - the stair walked as a slope; the Hornet's ground rules; a body radius of 0.4 m against walls (the old game:
    0.7 m against obstacles);
  - the constants of `constants.shot_geometry` (the gun's height 1.36 m, the hit cylinder, the level shot's grace,
    the aim's dead zone): they belong to shooting (change-set 8) and are listed there for his word.
- **What the Export measured** (`measured`: the soldier's height on screen): a reading, not a value.

Three of these touch change-set 4 directly and are asked in its plan (section "Decisions needed"): the body radius
(a Testlines value with no old number: it is in the table; the difference from the old 0.7 m is on the list he
confirms), the camera profiles (which one the game's camera starts from), and the aiming camera's two differences.

## 5. TUNE-HOME: where does he tune once change-set 4 exists? (his point g)
Why it matters: after change-set 4 two places can change a number, the sandbox and the game's own camera panel (his
queued words of 2026-11-10: a live camera tuning panel from the first playable build, with "copy values"); one value
with two homes drifts. The Testlines README says values flow one way, from the sandbox to the game, never back.
- **(a) The sandbox stays the home of every value in the table.** He tunes in Testlines, exports, pastes; the script
  saves the Export and rewrites the content; the check holds them equal. The game's camera panel moves the camera
  live for trying only: it starts from the content's values each time, and its "copy values" is for comparing, not
  a source.
- **(b) One home per group.** Movement, jump, stamina, pushing and the aiming rule stay in the sandbox, as (a). The
  camera's groups (`camera`, `camera_framing`, `mouse`, the aiming camera's numbers) change home to the game when
  the part with the camera panel merges: they start from the Export once, then he tunes them on the real picture
  with the panel, and "copy values" pasted in the chat becomes the camera's data (the build plan's Part 5). The
  table marks those rows "home: the game" and the check stops comparing them with the Export from then on.
- **(c) Everything moves to the game at change-set 4.** The sandbox is no longer read for the groups change-set 4
  uses; he tunes movement by saving the game's content file, which reloads within a second in development.
- Recommended: **(b)**. The camera has to be judged at the game's own scale and picture, which the panel was asked
  for; movement and the guns keep the sandbox, where he tunes them together with what the game does not have yet
  (his SLICE-FEEL: "the real game must not feel different from it").
