# docs-keeper, B.2 (b2-community-servers), 2026-11-06

## Plan
Edits only docs/STATUS.md, docs/ROADMAP.md, docs/reports/2026-11-06-b1-plan-view.md (conditions C5-C9):
1. STATUS:3 merge list: #39 and #41 (C5, time from merged_at).
2. STATUS Phase: B.1 merged, B.2 in flight, queue (B.2, then A4 only); A4 substance untouched, one neutral clause.
3. STATUS In flight: B.1 row merged; new B.2 row above it; B3 row "taken by B.2 (PR #42)".
4. STATUS line numbers (C6): :446 -> README:294-298; :447 -> :119; :465 -> :202; :466 -> :192-193.
5. STATUS queued words: B.2 entry removed, one-line pointer.
6. STATUS follow-ups: C8 (OQ-0006-21 pointer at next revision) and C9 (r1 :3301/:3535 stale VISION_V2.md:54).
7. ROADMAP: B.1 entry MERGED, B.2 entry above, B3 entry "taken by B.2 (PR #42)", "v2, next" line true.
8. B.1 report: dated correction note (form of kit-v3-3 report :82-87).
Then checks: diff --stat, line numbers, widths, repo-map-check, gate ee736ec --no-smoke.

## Progress
- STATUS:3 done (#41 and #39 merged, B.2 in flight).
- STATUS Phase done (B.1 merged in Done; B.2 in flight; queue: B.2 then only A4; A4 substance kept; UNDER-WAY clause).
- STATUS B.1 row: merged, :119, final-head APPROVE and its two Low notes, waiting none.
- STATUS B.2 row added above B.1's; B3 row: hand-over taken by B.2 (PR #42).
- STATUS line numbers (C6): :294-298, :119, :202, :192-193.
- STATUS queued words: B.2's entry and his two lead lines removed; pointer left (A4 entries untouched).
- STATUS follow-ups: From B.2 (C8 OQ-0006-21 pointer; C9 r1 :3301/:3535 stale cite), before the kit-v3-1 line.
- ROADMAP: v2-next queue true; B.2 entry; B.1 MERGED; B3 entry taken by B.2 (C7); R7 entry folded into Build history (12 entries kept).
- B.1 report: dated 'Corrections after the merge (2026-11-06, B.2)' note before Appendix A.

## Checks (DONE)
- `git diff --stat ee736ec -- docs/`: ROADMAP, STATUS, VISION_V2, adr/README.md, the B.1 report only (5 files).
- Line numbers against the worktree's README (411 lines, same count as ee736ec): :119 CD4, :160 OQ-0006-21,
  :192-193 R4-LOS-OQ1/OQ2, :202 R6-TEL-OQ2, :294 the "Needs outside VISION_V2 §6" heading through :298 its paragraph
  (:299 starts the ADR 0001 list). af8393e:118 and ee736ec:119 both CD4. VISION_V2's 100 ms line: :55 at ee736ec, :58
  now; r1-simulation-core.md:3301 and :3535 cite :54.
- Width: no changed line over 120 except STATUS:3, table rows and ROADMAP entries (single long lines by style).
- `node .claude/scripts/repo-map-check.mjs`: OK, 113 entries cover 100 required paths.
- `node tools/gate.mjs ee736ec --no-smoke`: GATE PASS, symbols +0 -0.
- `e-checks.py` rerun after the edits: RESULT PASS.
- Extra (role rule, ROADMAP keeps 8-12 Recent work entries): the oldest, R7 legacy inventory, folded into Build history.
