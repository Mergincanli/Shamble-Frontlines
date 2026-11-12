# U4e merge text script: notes (agent, 2026-11-12)

Task: write .checkpoints/netcode-proof-test/u4e_merge_text.py (phases A and B, dry run only, never --apply), applying
the texts of U4e's merge commit as advisor-u4e-r1.md (R9, T1-T6) and advisor-u4e-r2.md (Q3, T1, T2 (3), T3, T5, T7)
rule them. I write only this file and the script.

## Plan
1. Read both rulings whole (done), both model scripts (done); hash the rulings and targets (done: the five target
   prefixes equal r2's spot-check list: README 0ad7a80f, SERIES 881d7fde, carries cdab0240, plan 2bef127b, build-plan
   3f58ff0d; catalogue 35a27979; r1 7c129309, r2 14b0b180).
2. Read each target at its anchor: README §1 (T1), SERIES.md (T4), catalogue :90 (T7), u4s-carries.md (entries 69,
   46, 70, 45 as section ends; 62 (h)), plan.md U6 row (T3), u4-build-plan.md §4.1 (T6), u4df-report.md (anchor form).
3. Write the script: sha256 asserts; block reader by T heading; old spans read from the rulings' inline code where the
   ruling marks them; phase A edits; phase B edits; word-for-word checks; width checks; unified diff in the dry run.
4. Dry-run phase A and phase B until every assert passes; record each text placed here.

## Resumed after the usage limit (2026-11-12, same session's workflow)
- Step 1 re-checked: r1 7c129309..., r2 14b0b180...; targets README 0ad7a80f, SERIES 881d7fde, catalogue 35a27979,
  carries cdab0240, plan 2bef127b, build-plan 3f58ff0d (unchanged). u4e-report.md does not exist yet (anchor simulated).
- Step 2 done: anchors read. README §1: the old T1 text spans :66-67 inside the `decoded_hash` item (:56-71, indent 4).
  SERIES: soak_trigger row :218, slow_loris row :217, counter_wrap row :150 (same table). Catalogue: `Resync` row :90,
  Authority cell (5th) starts "Sender:". Carries: §U4r ends at 69 (:317-321), §U4f at 46 (:382-385), §U4g at 70
  (:394-398), §U4h at 45 (:403-405); 62 (h)'s two spans at :308-309. Plan: U6 row :1246 ends with T3's old text.
  Build plan §4.1: npt-client :193, npt-room :194, npt-server :195. 73-81 and C-U6-12..16 absent everywhere.
- Target paths: every ruling's path is the task's (marks/ = spikes/netcode-proof/marks/; plan.md, u4s-carries.md,
  u4-build-plan.md and u4e-report.md in the notes folder). No other file carries a U6 row.
- Design: placements (section and entry) parsed from the rulings' labels; inline old/anchor spans cut from the
  rulings' prose between their marks; prose refilled only from the line where the change starts to the item's or
  entry's end (u4j's refill, risky line starts guarded); every edit word-checked on the collapsed file.

## Placed (one line per text)
- Script written: .checkpoints/netcode-proof-test/u4e_merge_text.py (dry run only; never --apply).
- Phase A dry run: exit 0, every assert passed, 3 edits.
  - T1 (r2 :226; old text :221-224) -> README.md:66-71 refilled to 12 lines at indent 4 (from the line the old text
    starts on to the item's end); word check passed; no prose line over 120.
  - T4 (r1 :614; place :610-612) -> SERIES.md, new row before :218 (soak_trigger), slow_loris above, same table as
    counter_wrap; name order holds; 6 bars as the table. NOTE: the heading "## Series (235)" now undercounts (236 rows);
    neither ruling changes it: not by script, for the main session / the advisor.
  - T7 (r2 :364; old text :360-362) -> catalogue :90, the `Resync` row's Authority cell, in its line; cell still starts
    "Sender:"; bar count unchanged.
- Phase B dry run: exit 0, every assert passed, 10 edits (carries 73-81 and C-U6-12..16 asserted absent first).
  - T2 (1) (r1 :507-552; place :505) -> carries 73-77 after entry 69, the last of §U4r (:322).
  - T2 (2) (r1 :558; spans :554-556) -> entry 62 (h): "all" -> "measured", the sentence after "If it is not 0, ...";
    :308-310 refilled to 6 lines at indent 4.
  - T2 (3) (r2 :232-256, replacing r1 :562-573) -> carry 80 after entry 46, the last of §U4f.
  - T2 (4) (r1 :577-589; place :575) -> carries 78, 79 after entry 70, the last of §U4g.
  - T2 (5) (r1 :593-598; place :591) -> carry 81 after entry 45, the last of §U4h.
  - T3 (r2 :265; old text :260-263) -> plan.md:1246, the U6 row's end, in its line.
  - T6 (r1 :690/:694, :699/:703, :707/:711) -> u4-build-plan.md §4.1 :194 (room, whole), :195 (server, end from
    "bites builds only:"), :193 (client, whole).
  - T5 (r2 :275-356) -> u4e-report.md at ANCHOR (simulated: the report does not exist yet); 26 `<...>` places left for
    the main session's fills from M5's lines.
- Re-run after moving the C-U6 check to phase B's start: both outputs byte-equal to the first runs; the usage guard
  refuses no or a wrong --phase; every target's sha256 unchanged; no u4e-report.md written. DONE.
