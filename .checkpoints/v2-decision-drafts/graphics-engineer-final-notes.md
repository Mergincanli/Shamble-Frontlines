Task: final fix round (step 10 / step 9 once more) for ADR 0003 (minus Pass marks) and ADR 0005 GPU-rules section; output graphics-engineer-fixes-final.md. Started 2026-11-02.
- Read: prompt; reviews PA (4,8,10,11), EA (11,12,14,15), fixcheck-a L2, fixcheck-c 9, GE (1,4,Q1), AD (1,4,6,7,10,11,14, Qs), TE-5; ADR 0003 whole, ADR 0005 GPU section + AP-2/AP-3; LOOK_V2; r3 BUDGET-REC2/REC3, LOOK-F7..F15, WGPU-F14/F15; LI CL4, CL6, GW17, CD16, CD5; ART_DIRECTION:121; legacy reduced-motion lines verified (shamble-game.js:3039, index.html:321). docs/adr unchanged 47eadd9..HEAD.
- Key source: r3:1172-1173 (R3-BUDGET-REC3) "Which effects exist at all is the look brief's call; the tier table only says how much each may cost" -> backs LOOK-FLOOR as Mustafa's.
- Plan: new OQs: OQ-0003-16 LOOK-FLOOR (Mustafa), OQ-0003-17 DROPPED-FRAMES (Advisor, confirmed by Mustafa), OQ-0003-18 REDUCED-MOTION (Mustafa), OQ-0003-19 COLOUR-TELL (Mustafa). Unused ids checked.
- PA-8 arithmetic: 786 + 2 bars + 5 VFX kinds + 4..8 projectile kinds + 2..16 post passes = 799..817; hull 1,058+13..31 = 1,071..1,089 (inside 1,100).
- PA-10: 4 x 16.6 = 66.4 MB multisampled + 16.6 resolve; 3 attachments x 66.4 = 199.2 MB at 1080p; x0.5625 at scale 0.75 = 112 MB.
- PA-11: 500k / (48 x 2) = 5,208 ~ 5.2k.
- Verified: _matSig at shamble-core.js:356-359 keys type, colour, emissive, map, gradientMap, side, alpha settings, nature gain; ADR 0001 part 5 (line 203) Worker casts the aim ray; part 7 (line 256) buffer in the Worker; GLOSSARY live projectile kinds (bolt, grenade, Molotov, rocket).
- Approach: generator script in scratchpad pulls BEFORE by line range from 47eadd9 (exact), AFTER hand-written; then check uniqueness + no overlap + apply to scratch copy.
- Line ranges confirmed against 47eadd9 snapshot (scratchpad 0003.md/0005.md). Writing generator gen_final.py in scratchpad next.
