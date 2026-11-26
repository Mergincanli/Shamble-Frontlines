# Question to Mustafa, 2026-11-26: DIFF-LIST (asked in the chat at the end of 4b-2's step 7)

**DIFF-LIST: your yes to the list below, printed from your Export at 30 steps.**

- **The question in plain words:** are the values in this list your tunings, and may they become the game's
  starting table?
- **Why it matters:** your point 8c: "Those are my tunings, and I confirm the list before it becomes the starting
  table." Step 8 writes the game's first content file, the soldier's movement table, from your Export. It writes
  no content file before your yes.
- **What is new since the plan showed you this list:** it is printed again from the Export you sent at 30 steps
  (sha256 a9d47785...c4ab, saved by script, no value typed by hand). It is the plan's list with two rows changed
  in part B and nothing else: the row for the step rate is gone (the Export is at 30 now), and one row is new,
  the gun in your hands when you exported (SMG to shotgun), which the game does not carry. Parts A and C are the
  same, name for name and value for value.
- **What of it enters the game in 4b:** only the soldier's movement table. Of the list's rows, these: the sprint
  multiplier 1.52 (the old game: 1.214); the body's radius 0.4 m (the old game's code: 0.7 m); full steering in
  the air (100 %); the push's strength 8 and radius 0.4 m (the old game has no such rule). The table's other ten values
  equal the old game's by the same comparison (its "equal" list, in `testlines-compare-30.txt`: walk speed 6.3,
  the two easing times from its one acceleration 14, gravity 24, the step 0.55, the jump's reach 3.0708, the
  three stamina numbers 7, 0.2 and 0.3, the sights' speed multiplier 0.55). The camera's rows enter with 4c and the guns' rows with change-sets 8 and 9; their plans
  ask you again.
- **Options:** (a) yes, the list as printed; (b) yes, except the rows you name (you send a new Export, or say
  which rows wait); (c) no: the movement table waits, and step 8 builds its tools on test files only.
- **Recommended: (a).** The values are your Export's, unchanged; a later Export replaces them the same way.

_Printed by `testlines-difflist.mjs` from the saved Export (sha256 a9d47785...c4ab) and from
`testlines-compare.txt`; no value is typed by hand._

**A. The 23 names where his Export differs from the old game's tables** (`docs/legacy-tuning.json`):
| The old game's name | The old game | His Export |
|---|---|---|
| `UNIT_TYPES.conscript.sprintMult` | 1.214 | 1.52 |
| `T.camHeight` | 40 | 24.5 |
| `T.camBack` | 56.6 | 34.7 |
| `T.fov` | 15 | 13 |
| `T.camYawBase` | 0.7853981633974483 | 1.117011 |
| `T.lookLerp` | 9 | 20.5 |
| `T.camPushLerp` | 9 | 2.5 |
| `T.lookAheadBase` | 5.5 | 3.4 |
| `T.lookAheadFloor` | 3.5 | 4.5 |
| `T.lookAheadHip` | 8 | 4.9 |
| `T.lookFwdRange` | 26 | 15.9 |
| `T.lookBackRange` | 16 | 9.8 |
| `T.lookSideMax` | 6 | 3.7 |
| `T.posFwdMax` | 3.2 | 0 |
| `T.posBackMax` | 2.2 | 1.3 |
| `T.posSideMax` | 2 | 0 |
| `WEAPONS.shabzon.recoil` | 0.3 | 0 |
| `WEAPONS.shabzon.spreadHip` | 0.075 | 0 |
| `WEAPONS.shabzon.spreadPerShot` | 0.005 | 0 |
| `WEAPONS.shabzon.spreadMax` | 0.16 | 0 |
| `WEAPONS.shabzon.spreadDecay` | 0.7 | 0 |
| `T_CHARGE.nade.pitch (deg)` | 0.72 rad = 41.253 deg | 41.3 |
| `T.foxPanMax` | 16 | 9.8 |

**B. What he changed from the sandbox's start values** (the Export's own list: 18 entries,
3 of them left out here because the game does not take their group: `measuring_grid`):
| Group and name | Start | His value | |
|---|---|---|---|
| `soldier.faction` | "soviets" | "empire" | not carried: the sandbox session's own choice |
| `soldier.gun` | "smg" | "shotgun" | not carried: the sandbox session's own choice (the gun in his hands when he exported) |
| `movement.sprint_multiplier` | 1.214 | 1.52 |  |
| `smg.recoil_view_kick` | 0.3 | 0 |  |
| `smg.spread_rest_rad` | 0.075 | 0 |  |
| `smg.spread_per_shot_rad` | 0.005 | 0 |  |
| `smg.spread_max_rad` | 0.16 | 0 |  |
| `smg.spread_recovery_rad_per_s` | 0.7 | 0 |  |
| `aim_down_sights.ads_flatten_floor_deg` | 28.5 | 36 |  |
| `camera.lens_vertical_fov_deg` | 15 | 13 |  |
| `camera_framing.look_follow_rate_per_s` | 9 | 20.5 |  |
| `camera_framing.edge_push_rate_per_s` | 9 | 2.5 |  |
| `camera_framing.look_ahead_min_m` | 2.1 | 4.5 |  |
| `camera_framing.camera_shift_forward_m` | 2 | 0 |  |
| `camera_framing.camera_shift_side_m` | 1.2 | 0 |  |

**C. Values with no number in the old game's tables, used by Walk:**
| His Export's name | His value | The old game |
|---|---|---|
| `movement.body_radius_m` | 0.4 | a number in its code, not in a table: `resolveObstacles(player.position,0.7)` (legacy/shamble-game.js:2780) |
| `movement.air_control_pct` | 100 | no number: full steering in the air |
| `pushing.push_strength_per_s` | 8 | no such rule: soldiers pass through each other |
| `pushing.push_radius_m` | 0.4 | no such rule |
| `aim_down_sights.ads_flatten_floor_deg` | 36 | a number in its code: `const _tgt=Math.max(28.5,Math.min(_base,_toAim))` (legacy/shamble-game.js:2826) |
| `aim_down_sights.ads_dead_zone_scale` | 0.45 | a number in its code: `const dzx=ps.ads?T.camDeadX*0.45:T.camDeadX` (legacy/shamble-game.js:2809) |
| `aim_down_sights.ads_camera_ease_per_s` | 7 | a number in its code: `adsPitchK+=(_t-adsPitchK)*(1-Math.exp(-7*dt))` (legacy/shamble-game.js:2822) |

**The same kind, used from change-sets 8 and 9 on** (their plans ask again; shown so the list is whole):
| His Export's name | His value | The old game |
|---|---|---|
| `smg.sprint_to_fire_delay_s` | 0.15 | no such rule (SLICE-FEEL adds it) |
| `smg.active_reload_window_start_pct` | 40 | no such rule (SCOPE-RELOAD adds it) |
| `smg.active_reload_window_width_pct` | 20 | no such rule |
| `rifle.sprint_to_fire_delay_s` | 0.25 | no such rule (SLICE-FEEL adds it) |
| `rifle.active_reload_window_start_pct` | 40 | no such rule (SCOPE-RELOAD adds it) |
| `rifle.active_reload_window_width_pct` | 20 | no such rule |
| `shotgun.sprint_to_fire_delay_s` | 0.3 | no such rule (SLICE-FEEL adds it) |
| `shotgun.active_reload_window_start_pct` | 40 | no such rule (SCOPE-RELOAD adds it) |
| `shotgun.active_reload_window_width_pct` | 20 | no such rule |
| `shotgun.falloff_start_m` | 15 | no falloff on this gun |
| `shotgun.falloff_end_m` | 34 | no falloff on this gun |
| `shotgun.falloff_min_share` | 1 | no falloff on this gun |
| `grenade.grenade_refill_s` | 5 | not among the Export's old-game names |
| `grenade.bounce_kept` | 0.4 | not among the Export's old-game names |
| `grenade.slide_kept` | 0.6 | not among the Export's old-game names |
| `grenade.fuse_s` | 1.5 | not among the Export's old-game names |
