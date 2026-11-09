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
