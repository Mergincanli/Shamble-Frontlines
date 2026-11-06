# docs-keeper, B.1 (b1-plan-view): STATUS/ROADMAP/REPO_MAP after build 6ddbdd9 and PR #38's merge

Plan: (1) #38 merged facts; (2) B.1 In flight row + ROADMAP entry; (3) Queue line; (4) Mustafa's words block
(remove B.1 entries, pointer, keep B.2/lead lines/A4); (5) scale follow-up; (6) condition 11 line refs + A3 notes
line; (7) REPO_MAP EDITOR_V2 row. Then checks (e1-words.mjs, verbatim checks, line refs, repo-map-check).

## Block 0: facts checked (DONE)
- GitHub API: PR #38 merged_at 2026-11-05T21:25:34Z, merge commit af8393e (parents 12d7961, 8b7cfb3), head 8b7cfb3;
  check runs on af8393e: gate success 21:25:49Z, smoke success 21:26:23Z. Comment 6669362242 by Mergincanli 21:00:35Z
  "APPROVE, head 8b7cfb3...". PR #39: draft, head 6ddbdd9. Mustafa's "merge #38": from the caller (not checkable here).
- git: .checkpoints/kit-v3-3/ has 264 files at 4f268da and 6c6c91a (identical), 0 at 8b7cfb3 and af8393e; 8b7cfb3's
  parent is 6c6c91a, whose parent is 4f268da. STATUS cites `.checkpoints/kit-v3-3` on 18 lines (21 occurrences).
- Finding: STATUS:747-749 says the notes "stay in the branch history, last at 4f268da": true in content, but the last
  commit holding them is 6c6c91a. Fix with the new line.
## Block 1: survey (DONE)
- STATUS EDITOR_V2 line refs: only :599 (:51, :52); EDITOR_V2.md:89 and :90 hold those two §5 lines now [verified].
- REPO_MAP:37 "The map editor rebuild (draft spec)": still true (still a spec with [DEFAULT]/[OPEN]); no edit.
- ADR 0005:401 (top speed, metres per second), ADR 0004:261 ("a map 2 km across"), ADR 0005:669-680 (OQ-0005-CD4,
  R4-PHYS-REC2 at :676) [verified]. No doc but EDITOR_V2:59 and STATUS's quote states "1 unit = 1 metre".
- Drift found: STATUS:757 "Until A3 merges, routines run main's guard" (A3 merged: now past tense). ROADMAP has 11
  Recent work entries; B.1 makes 12 (inside 8-12, no fold). ROADMAP:29 phase line lists no kit-v3-2/3 (incomplete, not
  untrue; left).
- Edit plan: STATUS :3 header, :24-30 Phase, In flight (new B.1 row, kit-v3-3 row to merged), Decisions pending top
  (A3 notes line), Follow-ups (B.1 scale + CD4 follow-up), queue block (B.1 entries out, pointer), :599, :744-757 A3
  entry, Health (CI on main row); ROADMAP :93 and new B.1 entry.
- Block 2 DONE: STATUS :3 header (date 2026-11-06, kit-v3-3 merged, B.1 in flight); Phase :24-38 (kit-v3-3 in Done, B.1 In flight, Queue line, A4 waits for the freeze).
- Block 3 DONE: In flight: B.1 row added (top); kit-v3-3 row to merged (Branch cell 'merged: ...', State 'done: ...', Waiting 'none'). E1-E3 rerun at 6ddbdd9: all PASS.
- Block 4 DONE: :599 refs to :89/:90 [verified]; A3 notes line under '## Decisions pending'; A3 Open bugs entry: merged, notes deleted in 8b7cfb3, last at 6c6c91a; 'Until A3 merges' to past tense (guard/saver unchanged 0c481e1..af8393e, git diff empty).
- Block 5 DONE: Follow-ups: A3 items line to done (PR #38); new B.1 follow-up (scale; editing while the match runs vs OQ-0005-CD4) before the kit-v3-1 Windows line.
- Block 6 DONE: queue block: B.1's two entries removed, B.2 entry keeps the lead lines; one pointer line to EDITOR_V2 §3.
- Block 7 DONE: checks: e1-words.mjs STATUS part PASS (B.2, lead lines 1 and 2 once each), E1 PASS overall; scratchpad/dk/verbatim.mjs (read-only) PASS 16/16: A4's two quote blocks and A4-FREEZE once each vs 4f268da sources; his four phrases in source and STATUS; ADR 0004:261, 0005:401, 0005:669-680; EDITOR_V2:89/:90; §2 rule 4; adr/README:118.
- Block 8 DONE: Health: new 'CI on main' row (af8393e gate/smoke success).
- Block 9 DONE: ROADMAP: new B.1 entry at top of Recent work; kit-v3-3 entry to MERGED (af8393e, his go, APPROVE of 8b7cfb3). 12 entries (inside 8-12, no fold).
- Block 10 DONE: repo-map-check OK (113 entries, 100 paths); gate --no-smoke PASS on the working tree (base af8393e, +0 -0).
- Block 11 DONE (final checks, working tree after all edits): e1-words.mjs E1 PASS (EDITOR_V2 9/9 once; STATUS: B.2,
  lead line 1, lead line 2 once each); verbatim.mjs PASS 16/16; E2 PASS; E3 PASS; repo-map-check OK (113/100);
  gate --no-smoke PASS (earlier in block 10; later edits were STATUS text only). A3 notes line says 17 lines (18 lines
  carry the path including the note itself); all 12 distinct cited paths exist at 6c6c91a.
- Final line map: STATUS :3 header; :24-31 Phase (kit-v3-3 Done, B.1 In flight); :32-34 Queue; :38-41 A4 waits;
  :99 B.1 In flight row; :100 kit-v3-3 row merged; :149-150 A3 notes line; :337-338 A3 follow-up done; :432-443 B.1
  follow-up; :473-482 queue block (B.2 + pointer); :587 EDITOR_V2.md:89/:90; :732-738 and :746-748 A3 Open bugs
  entry; :909 Health 'CI on main'. ROADMAP :93 B.1 entry; :95 kit-v3-3 MERGED. REPO_MAP: unchanged.
DONE
