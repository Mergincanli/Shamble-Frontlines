# test-engineer: U4 build plan rev 2 conditions check (notes)

Plan: (1) read advisor-u4.md conditions 1-13 + nits; (2) diff aebe8ea vs now of u4-build-plan.md, verify old sha;
(3) per condition/nit: met (quote line) or not; (4) per changed hunk: which condition allows it; (5) check
u4-plan-md-edits.md: old text verbatim in plan.md, new text matches ruling, none missing.

## Log
- Step 1 done: advisor-u4.md read (conditions 1-13 at :79-171; rulings :196-226). aebe8ea sha256 27e03158... confirmed;
  current file sha256 f7961cfe..., 1012 lines (was 901). No agent memory dir found.
- Step 2 done: diff aebe8ea..now = 43 hunks (matches author's "43 spots"), +169/-58. Hunk->condition map drafted
  (H1 header=cond 12 bookkeeping; H2,H5,H39,H41 nit a; H3,H4,H8,H22,H24,H25,H40 cond 2; H6,H37 cond 8; H7,H9-H13,H21
  cond 1; H14,H33 cond 4; H15 nit b; H16,H17,H30 cond 3; H18,H34,H35,H36 cond 10; H19 nit e; H20,H31 cond 5(+9);
  H23 cond 2,4 nits c,f; H26-H28 cond 7; H29 nit d + cond 6; H32 cond 2,3; H38 nit f; H42,H43 cond 11).
  To verify: D-6 numbers vs marks.json; "option B" in H32; "burst"->"1 s gap" in H29; "full pool"/non-finite in H12;
  wave-1 crate list in H18; stale PRE-/SimEvent::Clamped/270 refs; Q6 ruling (b) vs plan's recommendation; risk 11.
- Step 3 (conditions in text) findings so far:
  - D-6 (u4-build-plan.md:595-598) matches marks.json PM-0001-2 SS checks exactly (soldiers>=48, team>=16, big
    vehicles>=15, very large>=4, projectiles>=300, shots mean 665-735, portals mean 0.95-1.05). Cond 5 met.
  - "option B" in -30 bullet (:730): only InputBuffer option B exists at U4r (U4e :678; option A is U4f :762); marks.json
    -30 notes "input buffer deep enough for P0 on both, recorded" -> within cond 3. OK.
  - "1 s gap" (:696) matches §4.14 -18 row (:442 "a 1 s gap evicts B"). Cond 6 met.
  - wave-1 bites crates npt-sim/proto/server (:497) match lanes (:808-810); npt-l1 already has bites (l1/Cargo.toml:15,
    lab/Cargo.toml:14). npt-server depends on npt-link (server/Cargo.toml:20). Cond 10 met.
  - New over-120-col lines from scripted edits: :31 (129), :461 (158), :700 (138), :874 (127), :908 (124). Cosmetic.
  - Residual stale text not required by a condition: Q16 text :970 "clamp twin built from the forged run's clamp
    events"; Q10 text :946-951 (record of the question); §4.12 :396-399 contract test "resolves" (U4r :713-716 adds
    "passing"); §2 C3 row :145 "U6 if Q6 (b)".
- Step 5a: plan.md sha256 daf25395... matches the edits file's pin; all 13 Old blocks + 2 checkpoint anchors occur
  exactly once at the stated lines (430, 391, 574, 476, 485, 603, 505, 503, 562, 1226, 567, 576, 423; ckpt 305, 311);
  no overlaps. Checkpoint file sha256 9bc00568... (no pin in the edits file).
- Step 4 done (every changed line): word-diff of all 43 hunks read; every removed span is replaced by its condition's
  text (e.g. :87 "about 300" -> "at least 300"; -30 bullet's judged form -> recorded test). No unmapped change.
  H1 (top provenance line) is bookkeeping under cond 12's pin. Author choices accepted: debug assert moved to the
  room's rewind_at (needed by D-10, and by approved D-9 :602-603 already); "a full pool" counted outside the hash
  (:237; literal "every refusal counter"; pool contents stay hashed, so no determinism loss); fourth recording;
  replay kept at U4 with its test; wave-2 manifest lines in U4s (serve 10 (b); dev deps don't count, plan.md §4.2).
- Step 5 done (edits file): all 13 + 2 Old texts verbatim, once, at stated lines; new texts match rulings: E1 cond
  7(c) (+ADR 0002:264-265 verified), E2 backed by approved plan old :601, E3 Q7 (§10.2 is plan.md:1037), E4-E6 Q19
  (no other _ms left: plan.md:476,477,485,603 only), E7 Q13+Q6, E8 Q25, E9 Q6 (:562), E10 cond 8 (ADR 0002:502
  verified), E11-E12 risk 11 SimCore/HitHistory (:567,:576 only), E13 proto->sim (ADR 0002:188 verified), ckpt C3
  :305, ckpt C-U7-1 after :311-312. Nothing on the verdict's list is missing.
- Out of scope, for the advisor (O1): at U4r the tolerance and cap do not exist yet (U4e :678-679 "tolerance and cap
  rules land in U4h"; U4h :787), so the -30 pre-check's setup condition (0 replacements, 0 clips) and the RTT
  argument hold by construction at U4r: the guard cannot fire. Fix: re-run the pre-check pair after U4h merges, or
  state it. Needs the advisor (cond 12).
- VERDICT: complete. Only cosmetic/optional items. Final answer returned.
