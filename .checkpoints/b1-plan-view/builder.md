Task: B.1 builder (b1-plan-view): edit EDITOR_V2, docs/adr/README.md OQ-0005-CD4 row, A3 report corrections; E1-E3 scripts; gate. Never commit or push.

## Plan (blocks, in order)
1. E3 expected counts written (before any edit) + e3 script.
2. EDITOR_V2 edits (header line, intro pointer, §3 dated block, five pointers, §6 pointer).
3. docs/adr/README.md:118 clause.
4. A3 report "Corrections after the merge" note.
5. E1 and E2 scripts; run E1 (EDITOR_V2 part, and STATUS part as info), E2, E3; record output.
6. gate --no-smoke and repo-map-check; record output.
7. Answer facts (line numbers for STATUS:599's :51/:52; r1:3631 check), DONE.

## Notes
- Fresh start (no earlier builder.md). Pointers use parentheses, never square brackets, so no tag counts change.
- Block 1 DONE: e3-tags.mjs written with expected counts before any edit (before 2/4/1/2/0; after DECIDED 11 = 2 + 8 lines + plan view, others unchanged, no new tag). Pre-edit run: before ok, after FAIL only on DECIDED 2 vs 11 (as expected, nothing built yet).
- Block 2 DONE: EDITOR_V2 edited (Edit tool). New lines: :6 "Updated on 2026-11-05 ..." (in the intro paragraph, LOOP_V2:5 form); :8 Principle pointer; :27-28 §2 rule 4 pointer; :36-37 Select and transform pointer; :45-46 Validation panel pointer; :47-48 Test pointer; :49-50 [LATER] pointer; :51-80 the dated block (heading bullet untagged, 9 [DECIDED] sub-bullets: his 8 lines in his order, plan view after the views line at :63-70 with "Mustafa, 2026-11-03 (his B.1)"); :103-104 §6 pointer paragraph. No pointer on interiors. File 66 -> 104 lines. §5 lines now :89 and :90 (were :51, :52). Only line >120 chars is :33, pre-existing and untouched.
- Finding: r1-simulation-core.md:3631 cites EDITOR_V2.md:39 (Validation panel, bot reachability); that text is now :44, so the cite no longer hits (a dated record; left as condition 11 says).
- Block 3 DONE: docs/adr/README.md:118 OQ-0005-CD4 question cell gains: Mustafa (2026-11-05) bears on it: "you can keep editing while the match runs" (EDITOR_V2 §3, his tool order (2)). Column count unchanged; no option picked; no ADR draft edited.
- Block 4 DONE: A3 report: new section 'Corrections after the merge (2026-11-05, B.1)' at docs/reports/2026-11-05-kit-v3-3.md:82-88 (before Appendix A), pure insertion (7 lines added, 0 removed): 176 = first scan, finals 178 and 193; no automated review of 0c481e1 arrived, the count of six stands. Cites comment 6669362242 on head 8b7cfb3 (from the advisor's N17).
- Block 5 DONE: e1-words.mjs, e2-bullets.mjs, e3-tags.mjs (read-only, Node; sources from git objects 4f268da / af8393e so they rerun after later edits). Output in builder-evidence.txt: E1 PASS (9 pieces once each in EDITOR_V2; mustafa-words.md cross-check 9/9; STATUS: B.2, both lead lines once each), E2 PASS (headings 7 identical; 42/42 base blocks in order: 36 unchanged, 5 with one pointer; 12 new blocks, 9 [DECIDED]; no interiors), E3 PASS (after: DECIDED 11, DEFAULT 4, OPEN 1, LATER 2, IDEA 0, no other tag).
- Mutants (scratch copies in scratchpad/b1wrap, not in the repo): E1 fails on the base file (9), on a one-word change in the plan view, and on a doubled Principle; E2 fails on a changed old word, a renamed heading, a second pointer, a pointer naming interiors (after a fix to how E2 collects pointers), and a renumbered slice. E1 has --also <file> for the report appendix and --no-status.
- Block 6 DONE (after a container restart): tools/ deps installed in the worktree (npm ci, node_modules is gitignored). `node tools/gate.mjs --no-smoke`: GATE PASS (base af8393e; 1-4, 5b-5d PASS, symbols +0 -0, 5a skipped), output in builder-gate.txt. repo-map-check: "repo map OK: 113 entries cover 100 required paths".
- Block 7 DONE: STATUS:599's EDITOR_V2.md:51 -> :89 ("It still runs by double-clicking editor.html"), :52 -> :90 ("if ADR-002 moves the project to modules"). r1-simulation-core.md:3631's EDITOR_V2.md:39 no longer hits (the text is now :44). EDITOR_V2 is 107 lines (was 66); §6 pointer at :106-107.
DONE
