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
- Placement (advisor-confirmed): 2026-11-11 entries at the very top, A3.2, A3.1, A2.3, A4 (newest first by time). 2026-11-10 foundation-chat entries A2.2, A2.1, A1.1(+A1.2, A1.3) contiguous directly below the #46 go (16:04:59) and above the status-diet block: entries not moved from STATUS first within their day, newest first (all three saved before 16:04:59), then the moved blocks in STATUS's order; the rule is stated in the reworded italic line.
- B2 done: generator docs-keeper-gen-decisions.py (rebuilds from bb2d817 + appendix; idempotent) wrote docs/DECISIONS.md (628 lines). Next: B3 checker.
- B3 done: docs-keeper-check-decisions.py RESULT: PASS (output saved in docs-keeper-check-decisions.out): 9 appendix blocks; A2.3 848 chars; A4 = bb2d817 message's quoted lines; only the italic paragraph changed (7 -> 11 lines); 492/492 other old lines unchanged in order; inserted 63 quote, 55 header, 8 pointer, 11 italic lines; 10 quote runs each = one source block, each once; 3 pointers. None of the 9 blocks was in bb2d817's DECISIONS.md before (grep). Next: B4 STATUS.
- B4 done: docs-keeper-status.py (idempotent, from bb2d817) rewrote STATUS.md: Last updated, Last merged #49, kit-freeze exceptions (status-diet merged; his 11-11 addition scoped to the foundation chat), CI bb2d817 run 42991918886, #50 row replaces #47, #49 row removed, Next 1-2 (with pointer beside "Wave 1 runs one heavy spike at a time"), Waiting first bullet (#50's go), follow-up line "from status-diet". 112 lines.
- B5 done (same script): archive In flight gets #49's merged row built from its STATUS row's cells by script + merge facts; kit-v2-1 Low at :822 marked Fixed in dd12aeb [verified gate.mjs:355, README.md:35]. Next: B6 ROADMAP.
- B6 done: docs-keeper-roadmap.py (idempotent, from bb2d817): status-diet entry MERGED with facts; new 2.1a entry on top ([2026-11-10 to 11-11, PR #50: in review]); Kit v3.1 and v2 decision drafts folded into Build history after "Docs after v2 research"; 12 Recent work entries; Next/open "v2, next" gets a dated pointer and #44 "merged". Next: B7 decision-doc pointers.
- B7 done: docs-keeper-pointers.py (idempotent, from bb2d817; his sentence "Steps 2.3 to 2.6 keep their order and wait for their results." copied by script from A1.1): ADR 0002 pointer after :1425 inside step 1 (later lines +4); SPIKE_PLAN pointers as own paragraphs after :91 (after 'three days."', before the mermaid fence) and after the "In words" paragraph (:125), and inside "Depends on" after :432 (later lines +4/+8/+12). No other doc cites SPIKE_PLAN or ADR 0002 lines past those points (grep: only 0002:34, :472, :570).
- Finding for the main session: same claim, no pointer added (not named in the brief; decision docs): ADR 0002 :38 (SLICE-ORDER summary), :724 (part 12), :1115 ("What a spike must prove"); SPIKE_PLAN :96 (mermaid W1 label), :194 (netcode proof test's Place). Also DECISIONS :370 area (drafts brief inputs mention SLICE-ORDER) untouched.
- B9 done: REPO_MAP lists no recent reports one by one (only the 2026-10-31/11-01 ones), so no line added; repo-map-check: "repo map OK: 116 entries cover 103 required paths".
- Next: B10 final checks (rerun all scripts, checker, repo-map-check, line counts), advisor review, DONE.
- B10 fixes (advisor review 2): ROADMAP E3 wording now "31 of 31 PASS (faults fail alike on all three gates; controls pass)"; DECISIONS FREEZE-FDN and SPLIT/SCOPE pointers made nested bullets ("    - _Pointer...") so they close the blockquote; checker updated; archive #49 row CI "push run 42991918886, on bb2d817"; "no game code" verified (git diff 2d22b69 bb2d817 on the five game files: empty); A2.2 header "sent after 2.1a's report (PR #47)" (ffc16d8 inference dropped); STATUS Waiting "Then, in his order of 2026-11-11, 2.1b and #48 (2.2)" (no "each on his go"); kit-freeze line points at his two additions in DECISIONS (no paraphrase).
- Drift (corrected): stale `tools/` cites newly caused by dd12aeb, not covered by R7's risk (worded for the legacy-file move): LEGACY_INVENTORY TL1 (:387, "OLD_PREFIX :35", now gate.mjs:38; other gate.mjs ranges shifted), TL6 (:392, "gate.mjs:333 and README:34 call gate-globals.json 'reviewed data' ... deferred": now untrue, fixed by dd12aeb), TL9 (:395, "file read outside the repo"; smoke now says "outside the build folder", same folder at the root layout; smoke.mjs ranges shifted); research/r5-toolchain-quality.md:925 ("file read outside the repo") and :1737 (quotes README's old "reviewed data, not a gate-tool change"). Not edited (research docs, cited at their dates); for the main session. START_HERE/MODULE_MAP/TECHNICAL: no claim made untrue.
- Line citations into DECISIONS.md / STATUS_ARCHIVE.md: none in docs/ (not reports), .claude/, CLAUDE.md. Citations into STATUS.md lines exist only in LEGACY_INVENTORY and research r1/r2/r3/r5 (old, pre-split; not caused by this pass) and the archive's own "STATUS:3".
- [assumed] list: none. Every fact used came from the caller's verified facts, git (commits, dates, parents, diffs) or the code (tools/gate.mjs:355, tools/README.md:35).
- Rerun warning: the four scripts (docs-keeper-gen-decisions.py, -status.py, -roadmap.py, -pointers.py) rebuild their files from bb2d817 and overwrite; never rerun them after the main session hand-edits #50's STATUS row or anything else in those files. docs-keeper-fix2.py was a one-off edit to the scripts (already applied; rerunning it fails its asserts, harmless).
- Final state: STATUS.md 112 lines; checker RESULT: PASS (output in docs-keeper-check-decisions.out); repo-map-check "repo map OK: 116 entries cover 103 required paths"; report file untouched; nothing committed.
- Files changed: docs/DECISIONS.md, docs/STATUS.md, docs/STATUS_ARCHIVE.md, docs/ROADMAP.md, docs/SPIKE_PLAN.md, docs/adr/0002-code-structure-build-toolchain.md. REPO_MAP unchanged.
DONE
