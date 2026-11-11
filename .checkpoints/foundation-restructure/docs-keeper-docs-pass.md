Task: docs-keeper docs pass for foundation step 2.1a (foundation-restructure, branch claude/pensive-heisenberg-aazidf, head cec48cf).

Plan (blocks, in order):
B1 read: agent file, DECISIONS header + entries, report Appendix A, STATUS, archive rows, ROADMAP recent, e3-summary/checkpoint facts
B2 DECISIONS: new entries via script (A3.2, A3.1, A2.3, A4, A2.2, A2.1, A1.1-3), pointers, Queued item, italic line
B3 DECISIONS check script run + output
B4 STATUS edits
B5 STATUS_ARCHIVE edits (#49 row moved by script, kit-v2-1 Low fixed)
B6 ROADMAP edits
B7 decision-doc pointers (ADR 0002, SPIKE_PLAN)
B8 drift grep (legacy gate file reading)
B9 REPO_MAP + repo-map-check
B10 final self-review; DONE

Notes:
- B1 finding: save times (git) for 11-10 foundation-chat words: A1.1-A1.3 saved in ca45435 at 00:10:07 UTC; A2.1 after d389d91 (08:55:57) and saved in 91ae816 (09:15:09 UTC); A2.2 after ffc16d8 (11:57:47) and saved in 828a3e0 (15:02:59 UTC). All before #46 go (16:04:59).
- B1 finding: DECISIONS 11-10 group: :87 #46 go (16:04:59); :95 status-diet carried (06:33/07:08/10:08); :134 kit-v3-5 for the record (11-09 and 11-10; SPLIT/SCOPE at :169-173); italic :174 then 11-05. Queued FREEZE-FDN record :27-33. SLICE-ORDER cond 3 at :388.
- B1 finding: kit-v2-1 Low at STATUS_ARCHIVE.md:821; dd12aeb fixes wording at tools/gate.mjs:355 and tools/README.md:35 [verified].
- B1 finding: ROADMAP Recent work has 13 entries; +2.1a = 14; fold the two oldest (v2 decision drafts, Kit v3.1) -> 12.
- B1 finding: decision-doc lines: ADR 0002 :1422 "The spikes before the foundation"; SPIKE_PLAN :86 "runs before the foundation (SLICE-ORDER, condition 3)", :110-111 "wave 1 runs one heavy spike / at a time" (In words), :429 Depends on.
- B8 drift (done early): no doc claim made untrue. START_HERE/TECHNICAL/MODULE_MAP describe no folder rule. LEGACY_INVENTORY TL1 cites "OLD_PREFIX :35" (now gate.mjs:38) and TL9 "file read outside the repo" (smoke now says "outside the build folder"; same folder at the root layout): stale line cites under R7's open risk; not edited, reported.
- B1 done (orientation). Next: advisor consult, then B2.
