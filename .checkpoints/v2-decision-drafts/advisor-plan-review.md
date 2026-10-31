# Advisor plan review: v2-decision-drafts plan rev 1

Started 2026-10-31. Notes as I go.

## Unit 1: inputs read (plan rev 1, brief, LOOK_V2, STATUS, GLOSSARY §1-2, report, #28 comment, template, agent files)
- LOOK_V2 sha256 52b4fc06... matches plan; 6617e25 adds only docs/LOOK_V2.md. Branch diff vs 9e7db28: 5 files (LOOK_V2 + checkpoint files).
- FINDING A (write guard): local origin/main and main are STALE at b6effde (PR #28), remote main is 9e7db28. `git diff origin/main...HEAD` lists 17 files incl. STATUS, ROADMAP, r1-r6, VISION_V2 (PR #31/#32 changes) -> guard would be noisy AND blind to edits of research files. Gate default base also stale. Fix: fetch, compare vs 9e7db28 explicitly.
- FINDING B (term clash): GLOSSARY:355-373 "headroom" already means a run above the largest mode's N (N=64). TICK-30's "25 % headroom" is a time margin. Rule 5 forbids a new meaning. Advisor's own TICK-30 word "margin" (8.28 of 8.33). -> GLOSSARY §8 entry "the 60 Hz margin".
- FINDING C: GLOSSARY:84-85 says R1-TICK-REC2's rule "does not change"; TICK-30 supersedes it; also r2:2415-2416. Append-only §8 note (lines must hold).
- TICK-60-HEADROOM: base = server tick budget at 60 Hz 8.33 ms (TICK-30 words compare 8.28 to 8.33). 25% -> p99 <= 6.25 ms. Rule now, confirm at stop.
- research-ids.tsv: counts verified (REC 167, OQ 241, RR 7, XU 30, XT 72, SHARED 18; 521 unique). The two "R1 ids defined twice" are REFERENCES in r3 (r3:524, r3:771 legacy-facts lists), not definitions.
- #28 comment 6542571633 read: finding 2 (R2-BUD-REC6 reason, add "at N=54") and finding 4 (cond 9 + cond 13 no longer applies). Plan carries cond 9 (step 7), not cond 13 closure.
- Efforts missing: consistency critic, completeness critic, skeptic, fix owners, step-10 reviewers.
- Self-review gap: test-engineer authors pass marks and reviews "is it proven" of them.
- CLAUDE.md:4 "Red Alert 3 toy-box look" and art-director.md:10 "joyful, readable, toy-box battlefield" vs LOOK_V2 §3 adults. KIT-ALIGN list lacks them.
- VISION_V2 §6 has [LATER] (8 sub, 10) and [OPEN] (4 seats): rule 4 needs a line.
- decision-map prompt allows owner DESIGN / SPIKE_PLAN; plan rule 8 coverage counts only "the six drafts".
## Unit 2: more checks
- R3 has ONE glTF mention (R3-CHAR-F8, r3:476); plan's "12 glTF hits" are "GLB" inside "WebGLBackend". 0 hits KTX2/meshopt/Draco/Spline (word match). Conclusion of risk 3 holds.
- Superseded lines: r1:2029 (R1-TICK-REC2 rule), r2:2415-2416 ("the rule does not change"), GLOSSARY:84-85 ("which does not change"), r2:2402-2403 (#28 finding 2).
- STATUS ADR-001..003 entries (STATUS:198-202) and ADR-001-bound items (STATUS:257 SEC-MDL-2, :262 SRI/CSP, :273 wall-clock reads) not in the plan's register.
- R1 verifier checked only claims recommendations rest on (r1:3233) -> "research verified" tier must be scoped.
- EDITOR_V2 §2 (document rules) and §5 (double-click; "if ADR-002 moves to modules") touched by 0002/0005; plan silent.
- Decision map in progress (notes through R6); agent already flagged the DESIGN/SPIKE_PLAN coverage seam. Addendum, not redo.
- nproc = 4 (concurrency note plausible). Gate not run: plan review, I edit nothing (gate writes .gate/); run at PR review.
- R7-MARKS reading: plan defers inventory edits to acceptance; differs from "changed when their ADR draft comes up" -> approve, report states it.
- Step 0 edit before approval: ratify (brief §0 orders it first; byte copy). Cannot check vs the attachment itself.
## Verdict: APPROVE with 19 conditions (see final answer). Decisions: ASSET-RESEARCH (a) w/ conditions; TICK-60-HEADROOM base ruled now (server tick budget at 60 Hz, 8.33 ms; >=25 % margin = p99 <= 6.25 ms; term "the 60 Hz margin"), confirm at stop; GLOSSARY-8 yes, append-only, 3 entries + TICK-30 note; CHECKPOINT-AT-STOP keep.
DONE
