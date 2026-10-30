# The Retire split: engine-architect's report (verbatim)

Workflow run wf_1eb87409-4bd (prompt `prompts/engine-architect-retire-split.txt`, launched 2026-10-30 18:21 UTC), returned about 18:34 UTC. Notes: `engine-architect-retire-split.md` (ends with DONE).

I split all 34 Retire fields in /home/user/Shamble-Frontlines/docs/research/GLOSSARY.md. I also updated the "How to read an entry" description of the field labels. The compare script reports 0 problems, and nothing was committed.

**Counts:**
- **Exact:** 30 phrases. Today's check finds 40 hits for them, and I judged all of them wrong.
- **In context:** 73 phrases (70 distinct; "tick time" and "48" repeat across entries).
- **Keeps:** 8 (the 111 quoted phrases before = 30 + 73 + 8).

**Borderline calls:**
- "one core" went to exact. Its one hit (R2-INT-F20) is server work, but it was measured on an unrecorded legacy machine.
- These went to in context because a hit is a correct use or names a unit's own old scene:
  - "the tick interval" (Source's own term)
  - "login wall" (a website's login wall)
  - "stress load", "R1-TICK-REC2's P1 scene", "R1-TICK's scene"
- The three security-baseline names are wrong at all 9 hits, but they stayed in context because of their "as different names" clause.

**Script gap:** glossary_check.py misses a phrase split by a line break in a topic file, such as r4:691-692 and r6:609-610. Matching `\s+` between words would fix it. I rejoined the 8 glossary-side splits.

Notes: /home/user/Shamble-Frontlines/.checkpoints/v2-research/engine-architect-retire-split.md
