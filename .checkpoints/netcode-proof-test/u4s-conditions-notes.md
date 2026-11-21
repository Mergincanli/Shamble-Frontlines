# U4s conditions: netcode-architect notes (2026-11-09)

Task: apply the advisor's U4s ruling (advisor-u4.md:236-383), conditions 1 (a)-(d) to u4-build-plan.md and condition 2
(plus O1) to u4s-carries.md. Text only: no code, no builds, no runs, no network.

## Plan
1. Read the plan's §4.1, §4.2, §4.3, §4.14, §4.15, §5 U4a a4, §5 U4d D-4, Lanes wave 1, §7.2.
2. Apply condition 1 (a)-(d), each marked "(the advisor's U4s ruling, condition 1 (x))".
3. Apply condition 2 and O1 to u4s-carries.md.
4. Record the new sha256 and the edit list here.

## Progress
- Steps 1-2 done: condition 1 (a)-(d) applied to u4-build-plan.md by u4s-cond-edit.py (replace-once: refuses unless each
  old text occurs exactly once; all or none) with spec u4s-cond-plan-spec.txt: 12 edits. D-4 and the freeze checked
  word for word against advisor-u4.md:355-360 and :328-333 (u4s-cond-wordcheck.py: True, True).
- Step 3 done: condition 2 and O1 applied to u4s-carries.md (spec u4s-cond-carries-spec.txt, 9 edits; base sha256
  1500ca04…, the one the advisor reviewed), then one replace-once fix to entry 36 (the literal quoted as the code has it:
  `assert_eq!(EntityKind::ALL.len(), 7)`, crates/sim/tests/seam.rs:440).
- Step 4: this file. All steps complete.

## The hashes
- u4-build-plan.md: before f7961cfee8cdff10d9bd64ac0df7859ec59cd8d9724367e713f994e298f33302 (1,012 lines); **after
  7ec1b324287faefc7b1968184b599c8da691fcde9f992c888f195ed57579ff87** (1,031 lines; git diff: +36 -17).
- u4s-carries.md: before 1500ca04425f9872cdbe00464356addece691fcc130dcb90a6bd5fbab2856f41 (173 lines); after
  0b4b896b792d111cccd26d2700b000e1ebc8b5cdadb0c151540e20749ca2ee13 (230 lines; 44 entries).

## Edits to u4-build-plan.md (section, condition); line numbers are the new file's
Every edit carries the marker "(the advisor's U4s ruling, condition 1 (x))" on one line (grep finds 12).
1. §4.1, new npt-sim row (:186): `EntityKind::ALL`, `EntityKind::layout`, `grid::Field::integer`,
   `full_state_ack_ticks_at`, `HitHistory::for_ms` in place of `HitHistory::new`; bites-only `grid::Config::floor_rounding`,
   `OWN_COPY_SNAP_BITES`, `own_copy_snap_with_bite`; removed bites-only `grid::Config::BITES` and `grid::Config::with_bite`.
   Condition 1 (a), carry 30.
2. §4.1, npt-client row (:191): bites-only `BITES` and `grid_with_bite`. Condition 1 (a), carry 30.
3. §4.2, "Refused input leaves no trace" (:237-241): "non-finite components" dropped from the refusal-counter list.
   Condition 1 (b), NF-D4.
4. §4.3, the FullState line (:246-247): `full_state_ack_ticks_at(TICK_HZ)`, which is `ticks_for_ms(2_000)`; `TICK_HZ`
   asserted in 1 to 1,000. Condition 1 (a), carry 32.
5. §4.14, the -17/client-rounding row (:443): the id lives in npt-client (`npt_client::BITES`, `grid_with_bite`), the
   grid holding only the defect. Condition 1 (a), carry 31.
6. §4.14, the -41/server-quant-differs row (:448): the id lives at npt-sim's own-copy snap side (`OWN_COPY_SNAP_BITES`,
   `own_copy_snap_with_bite`), the grid holding only the defect. Condition 1 (a), carry 31.
7. §4.15, the Fields bullet (:463): "move 4" becomes "move 2" (carry 33's "move is 2 B"; the listed widths sum to 39 B
   only with it). Condition 1 (a), carry 33.
8. §4.15, the Size bullet (:466-468): 39 B per command (move is 2 B), so about 1,872 B a tick of commands for 48 soldiers
   before spawns and the tick's own header. Condition 1 (a), carry 33.
9. §5 U4a, a4 (:527-529): -57's bites set through `Quant`'s own bite choice in npt-proto, as integer coarsening, not
   through the grid config; the ids in npt-proto's own list. Condition 1 (a), carries 31 and 5.
10. §5 U4d, D-4 (:597-603): replaced by the ruling's new D-4, word for word (advisor-u4.md:355-360). Condition 1 (b),
    NF-D4.
11. Lanes, the wave-1 bullet (:822-829): condition 3's freeze joins it as a sub-bullet, word for word
    (advisor-u4.md:328-333). Condition 1 (d).
12. §7.2, "Each sub-unit" (:878-880): "…or, for U4s, U4b and U4j, the main session or one builder in a free lane".
    Condition 1 (c).
Not edited under (b): §4.1:183 (old and new numbering) holds neither "non-finite components" nor `non_finite` (grep
over the whole plan before the edits: "non-finite" only at old :236 and :593), so there was nothing to drop there.

## Edits to u4s-carries.md (condition 2 and O1)
- Header: a completion note (entries 34-44 added, carry 7 reworded; 44 entries).
- Carry 7 reworded (U4d): "…and no state depends on any other `Command` field (seam.rs `Command`; carry 8's order
  assert reads `tick`, and that is correct)". Condition 2.
- 34 (U4a): the four check.mjs nits (u4s-fix2-review-rust.md:90-93), home U4a's brief or U4d2's, never the main session
  while U4a owns check.mjs. Condition 2.
- 35 (U4b) and 39 (U4e): a tick count needed outside npt-sim goes into npt-sim as `<name>_at` plus its constant (carry
  18's route), used by path; why: check.mjs:627 (verified at HEAD: `/\bTICK_HZ\b/.test(init)` passes the initializer),
  u4s-fix-review-rust.md:114-115. Condition 2.
- 36 (U4d): s6 ties `EntityKind::ALL` to the enum with an exhaustive match (u4s-fix-review-rust.md:112-113). Condition 2.
- 37 (U4d): `TickInput` stays integer-only, its spawn, teleport and attribute fields included; U4d removes
  `InputReports::non_finite`; a float field in `TickInput` brings the counter back (NF-D4, advisor-u4.md:351-364).
  Condition 2.
- 38 (U4d): `StateDump` holds exactly what `state_hash` hashes, never `InputReports`. Condition 2.
- 40 (new section U4r): O1: U4r's brief states the pre-check's setup condition holds by construction at U4r (no
  tolerance and no cap until U4h) and re-runs the pre-check pair after U4h merges (6 runs) or says why not.
- 41 (U4d2 section, heading widened to crates/proto/Cargo.toml and check.mjs): the proto Cargo.toml bites comment
  (:13-14, verified: "the wire grid's bites are chosen through its config value") at its next edit. Condition 2.
- 42 (U4d2 section): entry 34's second home, if U4a's report does not list 34 as met. Condition 2.
- 43, 44 (new section "U4's report"): the 1 MB cap is carry 14 at U4d, not U4h (u4s-report.md:58-59); the public-items
  line (u4s-report.md:38-40) takes carry 30's list (advisor-u4.md:268-270). Condition 2.

## Open points for the advisor (not edited: beyond the ruling's list)
- F1 §4.15's derived sizes: "about 17.8 MB per 9,000 ticks and 53 MB for three" and "about 4.4 million calls each"
  rest on the old 1,973 B a tick (41 x 48 + 5). At 39 B: about 16.9 MB, about 51 MB and about 4.2 million words
  (1,872 + 5 = 1,877 B a tick; 16.8 MB, 50.5 MB without the 5 B). Condition 1 (a) names only "39 B a command, about
  1,872 B a tick of commands", so I left them; a one-line edit if the advisor approves it.
- F2 Lanes and entries 35/39: in wave 1 crates/sim is U4d's and check.mjs U4a's; in wave 2 crates/sim is no lane's and
  check.mjs U4d2's. Carry 18's route needs both files, so neither U4b nor U4e can follow it inside its own lane. Each
  entry carries the rule word for word and adds, outside the quoted rule, that the builder asks the main session.
- F3 NF-D4's U4h line ("U4h's AimCast states what a non-finite ray does", advisor-u4.md:362-364) is not on condition 2's
  list, so no carry holds it; it belongs in U4h's brief.
- F4 Citations: condition 1 (b)'s "§4.1:183" has nothing to drop (above); O1's "u4-conditions-check.md:39-42" is at
  :42-45 on disk; entry 40 cites :42-45.
- F5 u4s-carries.md:3-7 still say the build is uncommitted in a worktree (it merged at f080551); not on condition 2's
  list, left as is.
- Not mine to edit: advisor-u4.md (condition 1's list in the U4 verdict, :82, loses "non-finite components" as the
  ruling says; and the plan's new sha256 recorded under the ruling): the main session records both.
