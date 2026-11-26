# Mustafa's Testlines Export at 30 steps, with his three lines before it (2026-11-26)

_Copied by script from the chat's transcript (session_019YNMUVZR6ociHFWjByxkrq), never retyped: the message he sent while the chat was working (stored as a queued command) of 2026-11-26T15:07:24.742Z (uuid 247437b9-08a2-42a6-803f-bc3eda89ae6a), whole (15708 bytes, sha256 dc361452139f9dc15eae41445d03c441670c892980c5760e0187b0adcdd9440c). What it answers: his own point of 2026-11-26, 14:09 UTC (My Export at 30 steps comes before the data step, step 8 in the new numbering, which is in 4b-2) and the chat's standing ask for the Export at 30 steps. The Export alone, byte for byte, is cs4b-content/testlines-export-30.json with its manifest._

---
I have played on testlines.html on steps set to 30, and have exported data you needed:
I had forgotten to send this earlier, you can continue with your work: 
This is for later when you need it again
{
  "format": "testlines-export",
  "format_version": 1,
  "what": "Shamble Testlines feel values",
  "note": "Sandbox values, not decisions. Start values are the old game's numbers unless the sandbox says otherwise.",
  "build": "step E (Export fit for the hand-over to the game)",
  "build_commit": "4c34e750f5edda22547e3bbe4074faadee5961ae",
  "camera_profile": "Experimental",
  "exported": "2026-11-26 15:59:38 (UTC+01:00)",
  "exported_utc": "2026-11-26T14:59:38.351Z",
  "warnings": [],
  "values": {
    "simulation": {
      "step_rate_hz": 30
    },
    "measuring_grid": {
      "grid_on": true,
      "grid_thin_step_m": 0.25,
      "grid_ground_colour": "grey",
      "grid_blocks_colour": "dark",
      "grid_room_colour": "orange"
    },
    "metrics_room": {
      "wall_height_m": 3,
      "wall_thickness_m": 0.2,
      "wall_length_m": 4,
      "door_width_m": 1,
      "door_height_m": 2.1,
      "window_width_m": 1.2,
      "window_height_m": 1.2,
      "window_sill_m": 0.9,
      "cover_height_m": 1,
      "cover_length_m": 2.4,
      "cover_thickness_m": 0.3,
      "stair_rise_m": 0.18,
      "stair_tread_m": 0.28,
      "stair_steps": 8,
      "stair_width_m": 1.2,
      "stair_landing_m": 1.5
    },
    "soldier": {
      "faction": "empire",
      "gun": "shotgun",
      "shots_aim_at": "point"
    },
    "movement": {
      "walk_speed_m_per_s": 6.3,
      "sprint_multiplier": 1.52,
      "speed_up_time_to_90pct_s": 0.164,
      "stop_time_to_10pct_s": 0.164,
      "air_control_pct": 100,
      "body_radius_m": 0.4
    },
    "jump": {
      "jump_reach_m": 3.071,
      "gravity_m_per_s2": 24,
      "step_up_m": 0.55
    },
    "stamina": {
      "stamina_on": true,
      "sprint_time_from_full_s": 7,
      "refill_time_from_empty_s": 5,
      "sprint_returns_at_pct": 30
    },
    "pushing": {
      "dummies_walk": true,
      "push_strength_per_s": 8,
      "push_radius_m": 0.4,
      "dummy_health": 100,
      "dummy_down_time_s": 4
    },
    "smg": {
      "fire_interval_s": 0.072,
      "damage": 8,
      "magazine": 36,
      "reload_time_s": 1.9,
      "recoil_view_kick": 0,
      "spread_rest_rad": 0,
      "spread_rest_ads_rad": 0.06,
      "spread_per_shot_rad": 0,
      "spread_max_rad": 0,
      "spread_recovery_rad_per_s": 0,
      "active_reload_window_start_pct": 40,
      "active_reload_window_width_pct": 20,
      "range_m": 55,
      "sprint_to_fire_delay_s": 0.15,
      "falloff_start_m": 15,
      "falloff_end_m": 35,
      "falloff_min_share": 0.2
    },
    "rifle": {
      "fire_interval_s": 0.2,
      "damage": 34,
      "magazine": 1,
      "reload_time_s": 3.5,
      "recoil_view_kick": 2,
      "spread_rest_rad": 0.022,
      "spread_rest_ads_rad": 0.006,
      "spread_per_shot_rad": 0,
      "spread_max_rad": 0.05,
      "spread_recovery_rad_per_s": 0.3,
      "active_reload_window_start_pct": 40,
      "active_reload_window_width_pct": 20,
      "range_m": 300,
      "sprint_to_fire_delay_s": 0.25
    },
    "shotgun": {
      "fire_interval_s": 0.85,
      "damage": 11,
      "magazine": 2,
      "reload_time_s": 2.6,
      "recoil_view_kick": 4.5,
      "spread_rest_rad": 0.1,
      "spread_rest_ads_rad": 0.05,
      "spread_per_shot_rad": 0,
      "spread_max_rad": 0.16,
      "spread_recovery_rad_per_s": 0.5,
      "active_reload_window_start_pct": 40,
      "active_reload_window_width_pct": 20,
      "range_m": 34,
      "sprint_to_fire_delay_s": 0.3,
      "pellets": 8,
      "falloff_start_m": 15,
      "falloff_end_m": 34,
      "falloff_min_share": 1
    },
    "grenade": {
      "grenades_carried": 3,
      "grenade_refill_s": 5,
      "throw_speed_max_m_per_s": 26,
      "launch_angle_deg": 41.3,
      "grenade_gravity_m_per_s2": 20,
      "bounce_kept": 0.4,
      "slide_kept": 0.6,
      "fuse_s": 1.5,
      "blast_radius_m": 6.5,
      "blast_damage": 140,
      "camera_kick_strength": 24,
      "camera_kick_range_m": 34,
      "camera_kick_stiffness": 110,
      "camera_kick_damping": 12,
      "explosion_look": "sheet",
      "explosion_play_time_s": 1,
      "explosion_width_m": 8,
      "explosion_pull_to_camera_m": 2,
      "fragments": true
    },
    "hornet": {
      "top_speed_m_per_s": 12,
      "acceleration_m_per_s2": 7,
      "braking_m_per_s2": 14,
      "reverse_speed_m_per_s": 4,
      "coast_slowing_m_per_s2": 2.5,
      "turn_rate_deg_per_s": 140,
      "body_radius_m": 0.45,
      "step_up_m": 0.3,
      "lean_forward_deg": 12,
      "lean_back_deg": 22,
      "lean_recovery_per_s": 7,
      "lean_settle": 0.35,
      "charge_time_s": 1,
      "fire_interval_s": 0.45,
      "pellets": 8,
      "spread_no_charge_rad": 0.14,
      "spread_full_charge_rad": 0,
      "pellet_damage": 12,
      "pellet_range_m": 22,
      "beam_damage": 120,
      "beam_range_m": 40,
      "kick_pitch_deg": 35,
      "kick_slide_m": 0.06,
      "kick_brake_m_per_s": 3,
      "weak_time_s": 0.9,
      "weak_steering": 0.3,
      "mount_range_m": 2
    },
    "aim_down_sights": {
      "ads_button": "hold",
      "sight_line": true,
      "ads_speed_multiplier": 0.55,
      "ads_camera": "pitch",
      "ads_flatten_floor_deg": 36,
      "ads_slide_reach_m": 9.8,
      "ads_dead_zone_scale": 0.45,
      "ads_camera_ease_per_s": 7
    },
    "camera": {
      "height_m": 24.5,
      "distance_m": 34.7,
      "angle_override_deg": "auto",
      "turn_deg": 64,
      "lens_vertical_fov_deg": 13
    },
    "camera_framing": {
      "camera_follow_rate_per_s": 7,
      "look_follow_rate_per_s": 20.5,
      "dead_zone_x": 0.6,
      "dead_zone_y": 0.52,
      "edge_push_rate_per_s": 2.5,
      "look_ahead_base_m": 3.4,
      "look_ahead_min_m": 4.5,
      "look_ahead_max_m": 4.9,
      "look_ahead_gain": 0.55,
      "look_forward_range_m": 15.9,
      "look_back_range_m": 9.8,
      "look_side_max_m": 3.7,
      "camera_shift_forward_m": 0,
      "camera_shift_back_m": 1.3,
      "camera_shift_side_m": 0,
      "turn_follow_rate_per_s": 14
    },
    "mouse": {
      "camera_turn_rad_per_px": 0.006
    }
  },
  "changed_from_start": [
    {
      "group": "measuring_grid",
      "name": "grid_thin_step_m",
      "label": "Thin lines every (m)",
      "start": 0.5,
      "current": 0.25
    },
    {
      "group": "measuring_grid",
      "name": "grid_blocks_colour",
      "label": "Yard blocks colour",
      "start": "orange",
      "current": "dark"
    },
    {
      "group": "measuring_grid",
      "name": "grid_room_colour",
      "label": "Metrics room colour",
      "start": "blue",
      "current": "orange"
    },
    {
      "group": "soldier",
      "name": "faction",
      "label": "Faction",
      "start": "soviets",
      "current": "empire"
    },
    {
      "group": "soldier",
      "name": "gun",
      "label": "Gun (keys 1, 2 and 3)",
      "start": "smg",
      "current": "shotgun"
    },
    {
      "group": "movement",
      "name": "sprint_multiplier",
      "label": "Sprint",
      "start": 1.214,
      "current": 1.52
    },
    {
      "group": "smg",
      "name": "recoil_view_kick",
      "label": "Recoil (view kick)",
      "start": 0.3,
      "current": 0
    },
    {
      "group": "smg",
      "name": "spread_rest_rad",
      "label": "Spread at rest",
      "start": 0.075,
      "current": 0
    },
    {
      "group": "smg",
      "name": "spread_per_shot_rad",
      "label": "Spread added per shot",
      "start": 0.005,
      "current": 0
    },
    {
      "group": "smg",
      "name": "spread_max_rad",
      "label": "Spread at most",
      "start": 0.16,
      "current": 0
    },
    {
      "group": "smg",
      "name": "spread_recovery_rad_per_s",
      "label": "Spread recovery",
      "start": 0.7,
      "current": 0
    },
    {
      "group": "aim_down_sights",
      "name": "ads_flatten_floor_deg",
      "label": "Flatten: no flatter than",
      "start": 28.5,
      "current": 36
    },
    {
      "group": "camera",
      "name": "lens_vertical_fov_deg",
      "label": "Lens (vertical field of view)",
      "start": 15,
      "current": 13
    },
    {
      "group": "camera_framing",
      "name": "look_follow_rate_per_s",
      "label": "Look point follow rate",
      "start": 9,
      "current": 20.5
    },
    {
      "group": "camera_framing",
      "name": "edge_push_rate_per_s",
      "label": "Edge push ease rate",
      "start": 9,
      "current": 2.5
    },
    {
      "group": "camera_framing",
      "name": "look_ahead_min_m",
      "label": "Look-ahead minimum",
      "start": 2.1,
      "current": 4.5
    },
    {
      "group": "camera_framing",
      "name": "camera_shift_forward_m",
      "label": "Camera shift forward",
      "start": 2,
      "current": 0
    },
    {
      "group": "camera_framing",
      "name": "camera_shift_side_m",
      "label": "Camera shift sideways",
      "start": 1.2,
      "current": 0
    }
  ],
  "derived": {
    "sprint_speed_m_per_s": 9.576,
    "speed_up_rate_per_s": 14,
    "stop_rate_per_s": 14,
    "jump_rise_m": 2.521,
    "jump_takeoff_speed_m_per_s": 11,
    "jump_airtime_s": 0.917,
    "camera_view_angle_at_rest_deg": 30.73,
    "camera_angle_to_the_feet_deg": 35.22,
    "camera_straight_distance_m": 42.48,
    "metrics_room_built": {
      "door_width_m": 1,
      "door_height_m": 2.1,
      "window_width_m": 1.2,
      "window_sill_m": 0.9,
      "window_height_m": 1.2,
      "stair_height_m": 1.44,
      "stair_angle_deg": 32.7,
      "note": "The sizes as built: a slider asking for more than its wall allows is held back (see layout)."
    }
  },
  "constants": {
    "metrics_room_layout": {
      "wall_plane_x_m": 20,
      "door_wall_centre_z_m": -11,
      "window_wall_centre_z_m": -17,
      "cover_centre_z_m": -22.5,
      "stair_centre_z_m": -27.5,
      "stair_first_riser_x_m": 21.5,
      "stair_slope": "through the middle of each tread",
      "opening_limits": "no wider than the wall less 0.2 m; sill at least 0.1 m under the top; at least 0.05 m of wall over it"
    },
    "hornet": {
      "park_x_m": 6,
      "park_z_m": 1,
      "park_heading_deg": 180,
      "gun_height_m": 1.25,
      "footplate_height_m": 0.25,
      "wheel_radius_m": 0.42,
      "wheel_width_m": 0.12,
      "slide_fade_per_s": 14,
      "wall_drag_per_s": 8,
      "camera_kick_pellets": 1.4,
      "camera_kick_beam": 3.2
    },
    "grid_look": {
      "thick_line_m": 0.04,
      "thick_line_min_px": 2,
      "thick_line_shade": 0.36,
      "thin_line_m": 0.012,
      "thin_line_min_px": 1,
      "thin_line_shade": 0.74,
      "thin_lines_fade_from_px": 10,
      "thin_lines_gone_at_px": 5,
      "colours": {
        "grey": "#A6A6A6",
        "dark": "#707070",
        "orange": "#D98C42",
        "blue": "#6A93C9",
        "green": "#8DB463",
        "purple": "#A07CBC"
      }
    },
    "shot_geometry": {
      "gun_height_m": 1.36,
      "hit_cylinder_radius_m": 0.3,
      "hit_cylinder_height_m": 1.8,
      "level_shot_height_grace_m": 0.6,
      "aim_dead_zone_m": 0.3,
      "note": "Shots leave from the gun height above the feet; a dummy is hit inside the upright cylinder from its feet. Level shots still count within the grace above its head or below its feet. Aiming at the point, an aim point closer than the dead zone (on the ground) keeps the aim the soldier had."
    },
    "explosion_sheet": {
      "source": "art/explosion-sheet.png",
      "picture_px": [
        1024,
        572
      ],
      "grid": [
        4,
        4
      ],
      "cell_px": [
        256,
        143
      ],
      "frames": 16,
      "key_colour_rgb": [
        0.5,
        10.7,
        252.7
      ],
      "noise_floor": 0.2,
      "colour_bleed_px": 1,
      "ground_line_search_share": 0.3,
      "clear_share": 0.788,
      "anchor_ground_line_from_top": 0.867,
      "anchor_middle_from_left": 0.501,
      "frame_solid_share": [
        0.009,
        0.065,
        0.244,
        0.34,
        0.382,
        0.486,
        0.435,
        0.336,
        0.19,
        0.117,
        0.1,
        0.023,
        0.001,
        0,
        0,
        0
      ],
      "drawn_at_once": 8,
      "max_stretch": 4,
      "note": "The owner's sheet, keyed at build time by tools/key-sheet.mjs: the key colour is the measured mean of the flat blue background; cover below the noise floor is clear. The anchor (ground line from the top, middle from the left, as shares of a frame) is measured on the sheet and sits on the burst point. Frames play in reading order, in even steps. The picture stands upright, turned toward the camera, stretched up by 1 / cos of the camera's height angle over it (at most max_stretch)."
    }
  },
  "measured": {
    "soldier_height_m": 1.8,
    "soldier_on_screen_px": 198.7,
    "view_height_px": 1272,
    "soldier_share_of_view_height": 0.1562,
    "note": "At the camera in use when exported, from the steady camera (no shake); pixels are CSS pixels."
  },
  "old_game_terms": {
    "T.moveSpeed": 6.3,
    "UNIT_TYPES.conscript.sprintMult": 1.52,
    "T.accel (speed-up)": 14,
    "T.accel (stop)": 14,
    "T.gravity": 24,
    "T.jumpV": 11,
    "T.stepUp": 0.55,
    "EJUMP_REACH": 3.0708,
    "STAM.max": 7,
    "STAM.regen": 0.2,
    "STAM.lockUntil": 0.3,
    "T.camHeight": 24.5,
    "T.camBack": 34.7,
    "T.fov": 13,
    "T.camYawBase": 1.117011,
    "T.camYawSens": 0.006,
    "T.camYawLerp": 14,
    "T.camLerp": 7,
    "T.lookLerp": 20.5,
    "T.camDeadX": 0.6,
    "T.camDeadY": 0.52,
    "T.camPushLerp": 2.5,
    "T.lookAheadBase": 3.4,
    "T.lookAheadFloor": 4.5,
    "T.lookAheadHip": 4.9,
    "T.lookFwdGain": 0.55,
    "T.lookFwdRange": 15.9,
    "T.lookBackRange": 9.8,
    "T.lookSideMax": 3.7,
    "T.posFwdMax": 0,
    "T.posBackMax": 1.3,
    "T.posSideMax": 0,
    "UNIT_TYPES.conscript.maxHealth": 100,
    "T.enemyRespawn": 4,
    "WEAPONS.shabzon.fireInterval": 0.072,
    "WEAPONS.shabzon.damage": 8,
    "WEAPONS.shabzon.mag": 36,
    "WEAPONS.shabzon.reload": 1.9,
    "WEAPONS.shabzon.recoil": 0,
    "WEAPONS.shabzon.spreadHip": 0,
    "WEAPONS.shabzon.spreadADS": 0.06,
    "WEAPONS.shabzon.spreadPerShot": 0,
    "WEAPONS.shabzon.spreadMax": 0,
    "WEAPONS.shabzon.spreadDecay": 0,
    "WEAPONS.shabzon.range": 55,
    "WEAPONS.shabzon.falloffStart": 15,
    "WEAPONS.shabzon.falloffEnd": 35,
    "WEAPONS.shabzon.falloffMin": 0.2,
    "WEAPONS.amk.fireInterval": 0.2,
    "WEAPONS.amk.damage": 34,
    "WEAPONS.amk.mag": 1,
    "WEAPONS.amk.reload": 3.5,
    "WEAPONS.amk.recoil": 2,
    "WEAPONS.amk.spreadHip": 0.022,
    "WEAPONS.amk.spreadADS": 0.006,
    "WEAPONS.amk.spreadPerShot": 0,
    "WEAPONS.amk.spreadMax": 0.05,
    "WEAPONS.amk.spreadDecay": 0.3,
    "WEAPONS.amk.range": 300,
    "WEAPONS.bagpipe.fireInterval": 0.85,
    "WEAPONS.bagpipe.damage": 11,
    "WEAPONS.bagpipe.mag": 2,
    "WEAPONS.bagpipe.reload": 2.6,
    "WEAPONS.bagpipe.recoil": 4.5,
    "WEAPONS.bagpipe.spreadHip": 0.1,
    "WEAPONS.bagpipe.spreadADS": 0.05,
    "WEAPONS.bagpipe.spreadPerShot": 0,
    "WEAPONS.bagpipe.spreadMax": 0.16,
    "WEAPONS.bagpipe.spreadDecay": 0.5,
    "WEAPONS.bagpipe.range": 34,
    "WEAPONS.bagpipe.pellets": 8,
    "T.grenadeMax": 3,
    "T_CHARGE.nade.sMax": 26,
    "T_CHARGE.nade.pitch (deg)": 41.3,
    "T_CHARGE.nade.grav": 20,
    "T.grenadeRadius": 6.5,
    "T.grenadeDamage": 140,
    "T.camKickImpulse": 24,
    "T.camKickRange": 34,
    "T.camKickStiffness": 110,
    "T.camKickDamping": 12,
    "T.adsMoveMult": 0.55,
    "T.foxPanMax": 9.8
  }
}
