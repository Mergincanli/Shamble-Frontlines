# docs-keeper notes: vfx-xp-notes (2026-11-07)

Role read from `origin/main:.claude/agents/docs-keeper.md` and `origin/main:CLAUDE.md`. Worktree at d5e2c62.
Scope: only `docs/STATUS.md`, `docs/ROADMAP.md`, `docs/reports/2026-11-06-b2-community-servers.md`.
Conditions owned: C2, C3, C5, C7, C8, C9, C10, plus the advisor's four Low notes on B.2's final head.

## Plan
1. STATUS:3 header: date 2026-11-07; B.2 (#42) merged with the facts; In flight: this pass (PR #43, d5e2c62, review
   round next).
2. STATUS Phase: B.2 added to "Done"; the In flight and Queue paragraph (:33-47): this pass in flight, then A4
   (freeze lifted).
3. STATUS In flight table: B.2's row to merged (facts), waiting "none"; this pass's new row above it.
4. STATUS follow-ups "From B.2": the ADR 0005 line ranges reworded to this head (§6 80–99, :2550 27–99; §2-§4
   unchanged at 27–43, 45–52, 54–58), checked against VISION_V2; "Not decided by the note" to "bears on" (C8), the
   match record (ADR 0006:474-475, no field now).
5. STATUS "From vfx-xp-notes" block: XP-TERM (C2, C3), LOOK (C5), the need rule (C7).
6. STATUS queue: VFX entry removed, one-line pointer (Low note 1 goes with it).
7. STATUS Open bugs and risks: the CI line (C10) for infra-engineer.
8. STATUS DOCS-ALIGN line: EDITOR_V2 :93/:94 to :94/:95.
9. ROADMAP: B.2's entry MERGED; this pass's entry above it; "v2, next" queue text.
10. B.2 report: "Corrections after the merge (2026-11-07, vfx-xp-notes)" after For Mustafa, before the appendices
    (Low notes 2-4; C9).
11. Checks: diff stat, line numbers, widths, e-checks.py, repo-map-check, gate --no-smoke.

## Facts checked against the files (before editing)
- VISION_V2 at d5e2c62: §2 :27 (content to :43), §3 :45 (to :52), §4 :54 (to :58), §5 :60, §6 :80 (to :99), §7 :101;
  at 4c6db5b §6 was :75. ADR 0005:27, :48-50, :2550 still say 62–81, 25–40, 42–49, 51–55, 25–81 [verified].
- LOOP_V2:242 weapon XP `[DECIDED]`, :237-238 tracks reset each match; README:69 OQ-0005-AB3 quotes it;
  DESIGN_NOTES:23 "account XP"; LEGACY_INVENTORY:302 GE15 legacy career XP; LOOP_V2:140 "the existing career rank"
  [verified].
- LOOK_V2:63 `[DEFAULT]` "Comic impact touches for big moments, used sparingly" (LOOK_V2 §7, :60) [verified].
- README:408 J3 "no feature for `[LATER]` or `[OPEN]` items" [verified]. ADR 0006:474 "private flag", :475 "humans
  and bots per team" [verified].
- EDITOR_V2:94 "It still runs by double-clicking editor.html", :95 "if ADR-002 moves the project to modules"
  [verified].
- B.2's old C9 rule (e-checks.py at 5c0999f, :112-120): every `VISION_V2:N` citation outside STATUS and ROADMAP must
  be in R1's research record; the report's Appendix B title "VISION_V2:36-37 at ee736ec" is the one it trips on.

## Progress
- Done, STATUS: :3 (date, #42 merged, this pass in flight); Phase :32-36 (B.2 merged) and :37-42 (In flight, queue);
  table :106 (this pass's new row) and :107 (B.2 merged, waiting none); "From B.2" :480-497 (no field now, ranges at
  d5e2c62, "bears on"); research citation item "and at d5e2c62"; "From vfx-xp-notes" :503-517 (XP-TERM, LOOK, need
  rule); queue: VFX entry removed (old :498-520, Low note 1 with it), pointer :549-550 above B.2's; DOCS-ALIGN :659
  (:94, :95); Open bugs and risks: the CI line (C10) first. Helper: `dk-rows.py` (table rows).
- Done, STATUS rewraps: :41-48 (the queue paragraph) and the XP-TERM item, at 120 characters (`dk-wrap.py`). Final
  STATUS line numbers: this pass's row :106, B.2's :107; "From B.2" :480-501; "From vfx-xp-notes" :503-519; VFX
  pointer :550-551 (above B.2's :552); DOCS-ALIGN :660; the CI line :790-794.
- Done, ROADMAP (`dk-roadmap.py`): "v2, next" :74 (B.2 merged, this pass in flight as #43, then A4); this pass's
  entry :94; B.2's entry :96 MERGED with the facts and the Low notes; "Docs after R7" folded into Build history :61
  (13 entries would exceed 12; 12 now).
- Done, the B.2 report: "Corrections after the merge (2026-11-07, vfx-xp-notes)" :71-81, after For Mustafa, before
  Appendix A (Low notes 1 noted, 2-4 as corrections; C9: #40's :3, :31, :173 at 0588a17).
- Line references checked at this head: VISION_V2 :71-75 (XP block), §2 :27-43, §3 :45-52, §4 :54-58, §6 :80-99,
  :58 (100 ms); README :104, :122, :160, :183, :408, :69; EDITOR_V2 :7, :94, :95, §7 from :115; ADR 0006:474-475;
  LOOP_V2 :140, :147, :237-238, :242; LOOK_V2 :63; DESIGN_NOTES:23; LEGACY_INVENTORY:302; gate.yml step order
  (:86 before :91) [all verified]. B.2's old C9 trip: `git grep` at e223ecc finds the Appendix B title as the only
  `VISION_V2:N` citation outside STATUS, ROADMAP and R1 [verified]; the old script itself not rerun.

## Results
- `git diff --stat 4c6db5b -- docs/`: EDITOR_V2 +22, ROADMAP 10, STATUS 105, VISION_V2 +5, adr/README 8, the B.2
  report +12: six files, only the allowed ones.
- Widths (characters): no changed line over 120 except STATUS:3, table rows and ROADMAP entries.
- `python3 -I .checkpoints/vfx-xp-notes/e-checks.py 4c6db5b`: RESULT PASS (30 PASS, 0 FAIL); E4 lists STATUS's new
  citations (:106, :488, :495, :660), all true at this head.
- `node .claude/scripts/repo-map-check.mjs`: repo map OK (113 entries, 100 required paths).
- `node tools/gate.mjs 4c6db5b --no-smoke`: GATE PASS, symbols +0 −0.
