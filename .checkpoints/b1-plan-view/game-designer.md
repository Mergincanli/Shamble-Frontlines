Task: game-designer review of B.1 (b1-plan-view, PR #39, head 4f57d26): pointers, balance tools vs loop, register clause, stale wording.
Plan: 1 read advisor conditions + mustafa words; 2 read diff; 3 check pointers + §6; 4 balance tools vs LOOP/VISION/UNITS; 5 register clause + STATUS; 6 stale wording; 7 verdict.

## Block 1: sources read (DONE)
advisor-plan-rev1.md (14 conditions), mustafa-words.md, diff af8393e..4f57d26 (EDITOR_V2, adr/README, STATUS),
EDITOR_V2 at 4f57d26 (107 lines), LOOP_V2 (260), VISION_V2 (88), UNITS grep, LEGACY_TUNING speeds, ADR 0005 CD4
(:670-681), ADR 0002 SLICE-ORDER (:34-35), register rows R4-LOS-OQ1/OQ2, R6-TEL-OQ2, trace rule README:292-297.
His words in EDITOR_V2:53-80 match mustafa-words.md (read side by side; E1 is the verifier's).

## Block 2: findings (DONE)
- B1-GD1 medium EDITOR_V2.md:47-48 Test pointer names only "play instantly"; his (2) differs from the old line:
  "at the cursor" and "keep editing while the match runs" vs old "then return to the editor exactly where you were"
  (modal). Fix: "(Mustafa's tool order (2) bears on it: one key drops you in at the cursor, and editing goes on while
  the match runs: §3, Decided 2026-11-05.)" (paraphrase, not his whole line).
- B1-GD2 medium EDITOR_V2.md:72 vs :102 "Built with Editor v2 (slice 5)" in the same doc as §6 build slice 5
  "Verticality"; nothing in the doc says which (advisor N3/cond 6 put it in the plan's risks only). Fix: add to the §6
  line (:106-107, session text) "His "slice 5" (plan view, 2026-11-03) is SLICE-ORDER's v2 slice 5, "Octopon-3 and
  Editor v2" (ADR 0002:34-35), not build slice 5 here."
- B1-GD3 medium adr/README.md:118 clause quotes his words under a question about simulation NUMBERS; it does not say
  his words are about map editing, leave open whether edits reach the running match, nor that under both options map
  geometry and navigation data apply at the next match (ADR 0005:676). Reads as a lean to (b), which would not even
  carry map edits. Fix: append "; his words do not say whether those edits reach the running match, and under both
  options map geometry and navigation data apply at the next match (ADR 0005 OQ-0005-CD4)."
- B1-GD4 low STATUS.md:440-444 "against OQ-0005-CD4's "next match"" frames a conflict; omit reading where none
  exists (edits go to the document, applied at the next Test or match). Missing: if his words mean live edits,
  CD4's "Why it matters" premise ("an engine feature no §6 item names", ADR 0005:671) changes, since EDITOR_V2 is a
  need source (README:292-297). Cite ADR 0005:670-681, not 669-680 (669 is CD3's last line).
- B1-GD5 medium EDITOR_V2.md:19-20 §2 rule 1 "Edits change the document; nothing else is saved." meets trace
  sketches (a photo), sticky notes, snapshots ("store only what changed") and stamps (links, three edit levels). No
  pointer. Fix: "(Mustafa's stamps, trace sketches, sticky notes and snapshots bear on it: §3, Decided 2026-11-05.)"
- B1-GD6 medium EDITOR_V2.md:42-43 "Objectives as data" lists spawns, barracks+terminals, harvest fields, uplink
  sites, cache spawn points; LOOP_V2 also has refineries (:32), power plants (:42), the HQ (:96, :130), supply points
  (:259). His heatmaps ("each objective") and fairness panel ("every objective") read from this list. Line is the
  Advisor's [DEFAULT]: pointer + STATUS follow-up for the Advisor, no rewrite.
- B1-GD7 medium modes: fairness panel "under 120° mirroring" and §3 Three-team symmetry (:40-41) cover three-team
  maps only; VISION §3 [DECIDED] (:45-46) "two-team and three-team battles are both tested, and maps are designed per
  mode"; §6.1 modes as data; §7 balance across modes open. Fix: STATUS follow-up (architecture plan asks Mustafa) +
  pointer on Three-team symmetry ("(Mustafa's plan view and fairness panel work under 120° mirroring: §3, ...)").
- B1-GD8 low fairness vs rotation: LOOP §4 (:51 sites rotate, two at once at full scale) and §9 (:155 pair sites
  plus centre): per-moment times are unequal by design, equal only over the cycle; cache spawns (§4 :46) need rotated
  triples. STATUS follow-up: the panel's pass rule is a design rule (Mustafa with game-designer). Math in Block 3.
- B1-GD9 low "leapers" (EDITOR_V2.md:76) defined nowhere (grep LOOP_V2, VISION_V2, UNITS, DESIGN_NOTES; nearest
  UNITS.md:49 [IDEA] Soviet walker's leap). Ruler inputs the loop names: per-class speed (legacy speedMult 1/0.95/1.5/
  1.05 on moveSpeed 6.3), carrier "slow" (§4 :47, no number), one-way drop-downs (§9 :157), weapon weight [IDEA] §7
  :126; per-type top speed field ADR 0005:401. STATUS follow-up + question for Mustafa (later, unit list).
- B1-GD10 low heatmaps/sightlines meet: LOOP §11 :188 "at least 3 runs per build"; §10 :170 role mix shapes fights;
  VISION §6.6 telemetry and R6-TEL-OQ2 (README:201); sightlines meet R4-LOS-OQ1/OQ2 (README:191-192) and LOOP §9 :162
  [OPEN] aiming at height. STATUS follow-up lines.
- B1-GD11 low EDITOR_V2.md:106-107 "ranks the tools" picks the priority reading of "the tool order" (could be the
  order of use when making a map); and the line covers only the tool order, not the other seven items or the plan
  view. Fix: "Mustafa's tool order and his other editor decisions (§3, Decided 2026-11-05): the architecture plan
  (step 2) maps them onto these slices, for his approval."
- B1-GD12 low EDITOR_V2.md:36-37 pointer omits "drag handles or typed numbers", the part that meets the old
  "move/rotate gizmo plus a numeric inspector" (no size handle), and the plan view's "moved, rescaled and reshaped".
- B1-GD13 low EDITOR_V2.md:8 "Principle (Mustafa, 2026-11-05): §3, Decided 2026-11-05." names no principle (cond 1's
  example form). Fix: "Principle (Mustafa, 2026-11-05), a lean editor: §3, Decided 2026-11-05."
- B1-GD14 low EDITOR_V2.md:45-46 Validation pointer lacks "bear on it": reads as placing heatmaps, fairness panel,
  sightlines inside the Validation panel (hence slice 7); his words do not place them. Fix: add "bear on it".
- B1-GD15 low EDITOR_V2.md:49-50 LATER pointer at the end of a three-item line reads as covering interiors and terrain
  too (cond 9: no pointer on interiors) and leaves the prefab library's tag ambiguous. Fix: "(On a prefab library v2:
  Mustafa decided instances (stamps), which bear on it: §3, ...)".
- B1-GD16 low EDITOR_V2.md:82-86 §4 bars: none for "instantly" (2) nor smoothness with a match running while editing.
  STATUS follow-up for the architecture plan's test plan (§6 step 2).

## Block 3: math (DONE)
Illustrative geometry only (map size is LOOP §9 [OPEN]): bases at radius R = 100 m, 120° apart; pair uplink sites at
r = 60 m on the bisector between two bases; centre site at 0.
- Near team to its pair site: sqrt(R² + r² − 2Rr·cos 60°) = sqrt(7,600) = 87.2 m. Far team: R + r = 160 m. Centre: 100 m.
- Speeds (legacy, LEGACY_TUNING: moveSpeed 6.3 m/s × speedMult): Conscript 6.3, Peacekeeper 5.985, Warrior 9.45 m/s;
  Juggernaut maxSpeed 6; harvester 7.
- Times, near/far: Conscript 13.8 s / 25.4 s; Peacekeeper 14.6 / 26.7; Warrior 9.2 / 16.9. Per active pair site the
  two near teams arrive 11.6 s before the far team (Conscript).
- Over one rotation cycle each team is near two of the three pair sites: mean (2 × 87.2 + 160) / 3 = 111.5 m for
  every team: equal. With two sites at once (AB + BC), B is near both; over the three pairings each team gets that once.
- Under pure 120° rotation distance per team is equal by construction; the panel's real work: places the rotation
  breaks (hand edits, the centre, cache spawn points), and per-team speed differences from tier two (LOOP §5 :74-75).
- Pillars: per-moment inequality feeds swings (1) and choice cost (4); none of the tools measures the snowball (2) or
  the way back: that side is match telemetry (VISION §6.6). Both sides stay unmeasured by the editor: note, not defect.

## Block 4: verdict (DONE)
PASS WITH NOTES. No critical or high. His words intact and in his order; no pointer decides for him. 6 medium
(GD1 Test pointer, GD2 slice 5, GD3 register clause, GD5 rule 1, GD6 objectives list, GD7 modes), 10 low.

DONE
