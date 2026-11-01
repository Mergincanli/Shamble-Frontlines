Task: fix3 inventory rows (CL5, CC10, GW5, GW10, GE3, GE14, GE16, GP8, KT5) for ADR 0003 legacy inventory table; FIX blocks to graphics-engineer-inventory-fixes-3.md (2026-11-01, base 9ed8d67)
Read: prompt, critic item 5, LOOK_V2 (78 lines), 0003 table 683-714 (9ed8d67 = working tree, identical), OQ-0003-15 (807-813), checklist step 29 (1108, test-engineer's), 0005 CD-11 (objectives as data, map bounds), 0004 part 17 (money as sim phase), 0002 G12 (backend on every shot), STATUS:182 KIT-ALIGN.
CL5 (114): add own line after CL3; drop unchanged (Mustafa still picks per row); pending lifted by LOOK_V2 §8 step 5 + part 5.a (variant = slot palette, not post-pass; visor = emissive slot 5.d).
CC10 (139): add own line after CC6... line; port; enum + rest pose folded into animation port on authored skinned soldier (part 6); pending lifted by LOOK_V2 §3, §2.
GW5 (193): add own line after GW4; port unchanged; bounds = 0005 CD-11 + sim; pending becomes Mustafa's call OQ-0003-15; second FIX adds GW5 items to OQ-0003-15; NOTE: checklist step 29 is test-engineer's.
GW10 (198): NOTE -> ADR 0005 (CD-11 objectives as data; editor-only markers; spawning LOOP_V2 §2 / 0004).
GE3 (289): own line after PG redesign line; redesign unchanged; money = 0004 part 17; credit HUD part 10; pending stays (HUD layout, no draft/spike settles: Mustafa's).
GE14, GE16 (300, 302): join line 712 (port, HUD layout pending stays) with their PG11/PG12 twins' outcome; clause added: no draft or spike settles it (Mustafa's).
GP8 (318): join line 713 (redesign, HUD layout pending stays), twin PG15.
KT5 (414): own line after GP9; redesign unchanged; A/B technique with backend on every shot (part 4, 0002 G12); pending lifted for 3D look (LOOK_V2 §1-§8 via look checklist; values = look proof test picks), stays for UI criteria; agent file = KIT-ALIGN.
Next: write fixes file by script from 9ed8d67, verify BEFORE unique + apply cleanly.
Written: .checkpoints/v2-decision-drafts/graphics-engineer-inventory-fixes-3.md (6 FIX blocks, 2 NOTEs: GW10 -> ADR 0005; checklist step 29 is test-engineer's). BEFOREs copied by script from 9ed8d67 (scratchpad gen_fix3.py); independent parse-apply check (apply_check.py): 6 blocks, each BEFORE once, whole lines, apply in order; doc unchanged vs 9ed8d67.
DONE
