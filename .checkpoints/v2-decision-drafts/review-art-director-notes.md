Task: art-director review of the v2 decision drafts (plan step 10, wave 2), commit 47eadd9 on claude/laughing-lovelace-ox61se; scope ADR 0003 all, ADR 0005 asset pipeline + GPU rules, SPIKE_PLAN look proof test, against docs/LOOK_V2.md.

- Started 2026-11-02. No agent memory dir exists (no calibration notes). Scope docs unchanged between 47eadd9 and HEAD (git diff empty for docs/adr, SPIKE_PLAN, ENGINE_OVERVIEW, LOOK_V2, VISION_V2, research).
- Next: read LOOK_V2, brief, plan rules, advisor verdicts; then ADR 0003.
- Read LOOK_V2, brief, plan rules (6, 9, 12), advisor conditions 4, 9, 10, 18. Reading ADR 0003 lines 1-200 so far.
- ADR 0003 read to line 618 (Decision, LOOK_V2 table, Options). Candidate findings so far:
  F-a: tier table may switch any post effect off (0003:163-164 "every effect has a quality-tier switch (off...)", :332-333) -> ink (DECIDED §1/§2) could be off at Low; 5.b :191 "or the hull a High-only setting" -> no ink at Low. Need a look floor per tier (question/medium).
  F-b: shadow row "1024² or off" at Low (:212, :309) -> DECIDED §2 soft shadows off on min laptop. Same look-floor issue.
  F-c: per-class ink weight (buildings heaviest, DECIDED §2) is unresearched for the proposed screen-space family (OQ-0003-2) but a plain value in own TSL hull: robustness of proposal vs a DECIDED item; check OQ-0003-2 and pass marks.
  F-d: halftone DOM overlay (5.l) costed "against the post-processing row" but it is compositor cost; legacy halftone trap (animated transform on a sheet) - check DESIGN_NOTES/ART_DIRECTION.
  F-e: reduced motion for concussion shake / halftone / line boil: check legacy.
- ADR 0003 read to 1185 (spikes, pass marks, look checklist, migration). More candidates:
  F-f: look checklist (0003:1049-1119) never shows the Low tier look (ML default; WebGL2 default Low, :333-334) - Mustafa judges only MP/High; add a Low-frame step.
  F-g: no step judges the busy fight (48 soldiers + 4 VLU + 300 projectiles) for readability / faction tell-apart at game camera (incl. dusk). Steps judge items in isolation.
  F-h: "the game camera" undefined (framing open, R2-INT-OQ2; CD5) - step 9 closest/farthest framing undefined; state legacy's camera as the test camera.
  F-i: legacy screenshots "taken by the session with legacy's pageshot, headed" (:1053) - cloud has no GPU -> SwiftShader; better: legacy index.html run in Brave on MP beside v2. Verify /dev/dri.
  F-j: step 13 "at the distance it is used" mismatches L-2 (importance, not distance).
  F-k: PM-0003-12 note: post passes (ink 1 + AO/denoise + bloom 12) exceed 14-draw margin under 800 -> Low may have to drop glow: look-floor issue again (perf-auditor owns the number).
  Revisit-when (:665) "drop a technique to High only" and pass-mark fail route (:1028-1030) let a DECIDED item leave Low without Mustafa's look call.
- (resumed 11:07 after spend cut; deadline 12:00) Checked: no GPU in container (/dev/dri absent) -> F-i holds. Legacy CD5 camera: height 40, boom 56.6, fov 15, yaw 45 (inventory line 86) -> state it as the look test camera (F-h). CL6: hull in world units, contact line clamp, chars 0.8x.
  F-l: screen-space mapping depth=thick/normal=thin puts the wall-ground contact line (no depth jump) in the thin class -> buildings' base ink thin; draft never names this visible difference; checklist step 4 should point at ground contact.
  F-d confirmed: ART_DIRECTION.md:121 CSS overlay also dots HUD; flashes = animated full-screen multiply layer; compositor cost, not in GPU post row. Legacy reduced-motion covers only war-room UI (shamble-game.js:3039), not shake.
- Next: skim 0003 rollback/appendix, ADR 0005 asset part + GPU rules, SPIKE_PLAN look proof test; write review file.
- 0003 rollback read: ":1195 Any technique misses on Low: a tier change in data moves it to High only, with no code change" + ":1190 AO off on Low" -> F-a/F-b become one HIGH finding (look floor per tier).
- ADR 0005 asset part (1293-1525) and GPU rules (1527-1677) read. Candidates:
  G1 (low): 4 materials per soldier as drawn incl. headgear and weapon (0005:1569-1571) + one ramp per class -> limits distinct shading kinds; spike mark exists (:1672-1673); name the visible consequence in the checklist.
  G2 (medium): class examples "environment, unit, faction colour, emissive" (0005:1553-1554; 0003:220) have no structure/building class; LOOK_V2 §2 buildings' ink heaviest needs buildings told apart from environment (value: calmer; ink: heaviest). Fix: name a structure class or make ink weight a per-asset field.
  G3 (low): tree placeholders vs DECIDED "trees with variety" (0003 step 15 :1084-1085): say placeholders = legacy GW18 builders, judged for variety not style.
- SPIKE_PLAN §5 look proof test (219-249), §8, asset research, §9 read. Candidates:
  S1 (medium-low): asset list names ONE building (SPIKE_PLAN:236-237; 0003:398; 0005:1591-1592) but checklist step 14 (0003:1082-1083) needs BOTH houses (DECIDED §2); say which is authored and which is kit.
  S2 (low): SPIKE_PLAN:222 "how each [DECIDED] item ... looks" omits the [DEFAULT] items the checklist judges (steps 16-22, 27).
- Next: grep README/ENGINE_OVERVIEW for look decisions; write review file.
- README/ENGINE_OVERVIEW grep: no look decision beyond LOOK_V2 found; README:247 lists the look features as LOOK_V2-only traces; R2-INT-OQ2 camera DESIGN (README:119).
- Writing review-art-director.md now (verdicts: 0003 REVISE; 0005 asset+GPU APPROVE WITH NOTES; SPIKE_PLAN look APPROVE WITH NOTES).
