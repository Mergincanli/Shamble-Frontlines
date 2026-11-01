# graphics-engineer: inventory fixes, round 3 (completeness critic item 5; advisor condition 18)

Author: graphics-engineer (ADR 0003), 2026-11-01. Every BEFORE was copied by script
(scratchpad gen_fix3.py) from `docs/adr/0003-rendering.md` at 9ed8d67, whole lines; each occurs exactly once, and the
blocks apply in order without touching each other (script-checked). No doc edited; nothing committed.

Outcomes: CL5 lifted (drop unchanged); CC10 lifted (port); GW5 becomes OQ-0003-15 (Mustafa's call), with OQ-0003-15's
item list extended; GE3, GE14, GE16 and GP8 keep the pending note for the HUD's layout, on the table's existing HUD
lines (whose text now also says that no draft or spike settles it); KT5 lifted for the 3D look, kept for the UI
criteria; GW10 is ADR 0005's (NOTE).

### FIX CL5
FILE: docs/adr/0003-rendering.md
BEFORE:
| CL3 (112) | redesign (look brief pending) | redesign in TSL only if kept: LOOK_V2 names no skin rim, so whether one stays is Mustafa's call in the look proof test | the rim was GLSL injection (R3-WGPU-F6) |
AFTER:
| CL3 (112) | redesign (look brief pending) | redesign in TSL only if kept: LOOK_V2 names no skin rim, so whether one stays is Mustafa's call in the look proof test | the rim was GLSL injection (R3-WGPU-F6) |
| CL5 (114) | drop (look brief pending) | unchanged (drop; Mustafa still picks the mark, as the row says): a variant look is an asset's own palette in its faction colour slots, given the toon look at load (part 5.a), and a glowing part is an emissive slot (5.d), never a post-pass over toon materials; pending removed | LOOK_V2 §8 (authored models; step 5, "the engine applies the comic look"); the row's own lesson; no game caller since D6 |
END

### FIX CC10
FILE: docs/adr/0003-rendering.md
BEFORE:
| CC6, CC8, CC9, CC11, CC12, GC6 (136–138, 140, 141, 245) | port to TypeScript (pending) | port to TypeScript: kit data, sockets, IK and props port; the gun and prop shapes become authored models (ADR 0005); pending removed | LOOK_V2 §3 (real weapon silhouettes) |
AFTER:
| CC6, CC8, CC9, CC11, CC12, GC6 (136–138, 140, 141, 245) | port to TypeScript (pending) | port to TypeScript: kit data, sockets, IK and props port; the gun and prop shapes become authored models (ADR 0005); pending removed | LOOK_V2 §3 (real weapon silhouettes) |
| CC10 (139) | port to TypeScript (pending) | port to TypeScript as the row says: the arm-mode enum in the animation state, the rest pose folded into the animation port on the authored, skinned soldier (part 6); pose values are tuning; pending removed | LOOK_V2 §3 (slimmer, serious proportions: soldiers become authored, skinned models, as for CL10, CC1–CC5); §2 keeps small movement touches |
END

### FIX GW5
FILE: docs/adr/0003-rendering.md
BEFORE:
| GW4 (192) | port to TypeScript (pending) | port to TypeScript: the texel-snapped sun frustum (part 5.f); pending removed | LOOK_V2 §2 soft shadows |
AFTER:
| GW4 (192) | port to TypeScript (pending) | port to TypeScript: the texel-snapped sun frustum (part 5.f); pending removed | LOOK_V2 §2 soft shadows |
| GW5 (193) | port to TypeScript (pending) | unchanged: the ground grid, the red arena border and the world labels are drawn on the page, the labels' textures cached if kept; the bounds are map data (ADR 0005, CD-11) that the sim enforces (both from the row's note); pending becomes "Mustafa's call: keep, change or drop each (OQ-0003-15)" | LOOK_V2 names none of the three |
END

### FIX GW5 (OQ-0003-15's items)
FILE: docs/adr/0003-rendering.md
BEFORE:
  comic words (CL12, GW3), the kill vignette (GP9), the toy-soldier death fall (GA14), the skin rim (CL3) and the
  camera-player cutaway, which fades structures between the camera and the player beyond LOOK_V2 §4's roofs (GW17,
  `LEGACY_INVENTORY.md` line 205).
AFTER:
  comic words (CL12, GW3), the kill vignette (GP9), the toy-soldier death fall (GA14), the skin rim (CL3), the
  camera-player cutaway, which fades structures between the camera and the player beyond LOOK_V2 §4's roofs (GW17,
  `LEGACY_INVENTORY.md` line 205), and the ground grid, the red arena border and the world text labels (GW5, line 193).
END

### FIX GE14, GE16, GP8, GE3
FILE: docs/adr/0003-rendering.md
BEFORE:
| GP7, GP10, GP12, PG3, PG7, PG8, PG10, PG14, PG16 (317, 320, 322, 361, 365, 366, 368, 372, 374) | port to TypeScript (pending) | unchanged; pending stays for the HUD's layout, which LOOK_V2 does not set | LOOK_V2 covers the 3D look, not the HUD |
| PG2, PG4–PG6, PG11, PG12, PG15, PG20 (360, 362–364, 369, 370, 373, 378) | redesign (pending) | unchanged; pending stays for the HUD's layout, which LOOK_V2 does not set | LOOK_V2 covers the 3D look, not the HUD |
AFTER:
| GE14, GE16, GP7, GP10, GP12, PG3, PG7, PG8, PG10, PG14, PG16 (300, 302, 317, 320, 322, 361, 365, 366, 368, 372, 374) | port to TypeScript (pending) | unchanged; pending stays for the HUD's layout, which LOOK_V2 does not set and no draft or spike settles (Mustafa's decision) | LOOK_V2 covers the 3D look, not the HUD |
| GP8, PG2, PG4–PG6, PG11, PG12, PG15, PG20 (318, 360, 362–364, 369, 370, 373, 378) | redesign (pending) | unchanged; pending stays for the HUD's layout, which LOOK_V2 does not set and no draft or spike settles (Mustafa's decision) | LOOK_V2 covers the 3D look, not the HUD |
| GE3 (289) | redesign (pending) | unchanged: the money is a sim phase (ADR 0004, part 17) under LOOP_V2 §2, §3 and §8, which replace the resupply terminal; this record draws only the credit HUD and the terminal prompt (part 10); pending stays for the HUD's layout, which LOOK_V2 does not set and no draft or spike settles (Mustafa's decision) | LOOK_V2 covers the 3D look, not the HUD |
END

### FIX KT5
FILE: docs/adr/0003-rendering.md
BEFORE:
| GP9 (319) | port to TypeScript (pending) | unchanged; pending becomes "Mustafa's call: LOOK_V2 does not mention the kill vignette" | presentation-time hit-stop stays (R1-TICK-REC5) |
AFTER:
| GP9 (319) | port to TypeScript (pending) | unchanged; pending becomes "Mustafa's call: LOOK_V2 does not mention the kill vignette" | presentation-time hit-stop stays (R1-TICK-REC5) |
| KT5 (414) | redesign (look brief pending) | unchanged: only the before-and-after evidence technique carries over, with the backend printed on every shot (part 4; ADR 0002's G12); pending removed for the 3D look, whose criteria are LOOK_V2 §1–§8 item by item, as the look checklist names them, with the visible values Mustafa picks in the look proof test; it stays for the UI criteria (the war-room tokens), with the HUD's layout | LOOK_V2 §9 (judged by Mustafa in the browser, beside legacy); the agent file's stale look lines are STATUS's KIT-ALIGN, a kit change-set |
END

### NOTE GW10
ADR 0005: spawns become team objectives in the map document (its CD-11, "objectives as data (EDITOR_V2 §3)"); spawning inside the barracks is the sim's rule (LOOP_V2 §2; ADR 0004).
The markers are drawn only on the editor's layer, whose scene no draft designs (CD-11), so ADR 0003 draws nothing of GW10 and LOOK_V2 leaves it no look question.
END

### NOTE GW5 (look checklist step 29)
Step 29 of the look checklist (`docs/adr/0003-rendering.md` line 1108, test-engineer's text) lists OQ-0003-15's items; with FIX GW5 (OQ-0003-15's items) applied, it should also show the ground grid, the red arena border and the world labels.
Not written here: the checklist is test-engineer's (plan step 4); the session routes it.
END
