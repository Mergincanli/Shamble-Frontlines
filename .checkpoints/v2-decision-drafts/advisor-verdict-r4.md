# Advisor verdict: plan revision 4 (stop answers), v2-decision-drafts

Status: IN PROGRESS (notes below; the verdict block is written last)

## Notes
- started: reading the stop answers, plan r4, the map
- read: stop answers (verbatim file), plan r4, the map (all 14 tables, counts, 14 questions). Next: spot-check the map's key places, CHECKPOINTS §9, the merge mechanics, the 31 MB location, the saver.
- checked: advisor.md same as main; base 9e7db28 = origin/main; CHECKPOINTS §5 (stop the saver before ready), §9 (fold, delete in last commit, CI runs, ls-tree empty). 31 MB is all under .checkpoints/ (294b3cf skeptic-6 scratch; HEAD tree 51.8 MB of which 46.1 MB .checkpoints). GLOSSARY §8 (lines 744+) is THIS PR's own appended text (engine-architect, synthesis), so the 60 Hz entry is in scope to amend, not a settled input. Next: report's Decisions needed, OQ texts.
- checked: report Decisions needed (SPIKE-ORDER asked "accept the six drafts as the basis for the spikes"; REWIND-CAP recommended "set it from the proof test's p99"); OQ-0001-5 (c) defines the measured honest rewind as "the p99 at P2"; OQ-0003-17 options (a) count / (b) p95 rows; OQ-0003-20 answer makes the soaked set gate PM-0003-18..21 (so those rows keep gating); AB13 design treats a new reviewed operation as part of the data model; ADR 0006 part 8 goal budget is a computed fleet cost, provisional until slice 6 (OQ-0006-19); security baseline is before the first online playtest, so the tick bench (slice 1) falls inside "no token until the baseline". README pointers name OQ ids in most rows. Next: merge mechanics (repo squash setting, PR state), the saver, then rulings.
