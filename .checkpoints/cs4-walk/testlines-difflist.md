_Printed by `testlines-difflist.mjs` from the saved Export (sha256 9ad1d9f9...6498) and from
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
3 of them in the groups the game does not take: the grid, the metrics room, the Hornet):
| Group and name | Start | His value |
|---|---|---|
| `simulation.step_rate_hz` | 30 | 60 |
| `soldier.faction` | "soviets" | "empire" |
| `movement.sprint_multiplier` | 1.214 | 1.52 |
| `smg.recoil_view_kick` | 0.3 | 0 |
| `smg.spread_rest_rad` | 0.075 | 0 |
| `smg.spread_per_shot_rad` | 0.005 | 0 |
| `smg.spread_max_rad` | 0.16 | 0 |
| `smg.spread_recovery_rad_per_s` | 0.7 | 0 |
| `aim_down_sights.ads_flatten_floor_deg` | 28.5 | 36 |
| `camera.lens_vertical_fov_deg` | 15 | 13 |
| `camera_framing.look_follow_rate_per_s` | 9 | 20.5 |
| `camera_framing.edge_push_rate_per_s` | 9 | 2.5 |
| `camera_framing.look_ahead_min_m` | 2.1 | 4.5 |
| `camera_framing.camera_shift_forward_m` | 2 | 0 |
| `camera_framing.camera_shift_side_m` | 1.2 | 0 |

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
