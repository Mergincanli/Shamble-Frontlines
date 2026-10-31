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
