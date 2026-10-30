Task: step 3b review of engine-architect's resolutions (12 contradictions), GLOSSARY.md and cross-topic-oqs.md, plus engine-architect's question (plan-amendments-2).

## Progress
- read: plan-amendments-2, my verdict, EA report+notes, resolutions.md whole (1390 lines).
- next: verify claims against topic files per item; then glossary; then cross-topic-oqs.
- independent quote check: my script (scratchpad adv/check.py) finds all 116 "before" quotes exactly once (whitespace-normalised) in their files; topic files unchanged since 8dee746 (git diff empty). cross-topic-oqs has 1 before-quote, found once.
- arithmetic of #1 re-done: all sums/leftovers/N=54 scaling/25% option correct.

## Verdicts so far
- #1 REVISE (small): (a) C1.21 replace "if the measured ticks do not sit well below the ceiling" with "unless R2-ROOM-REC2 mark 2 passes on it (8 rooms, each p99 within 10 % of the single-room figure)"; (b) state what happens if the whole tick fails at the rate the rule picks: a failed R2-BUD-REC6 mark for the synthesis; rule unchanged; (c) C1.5 laptop share nit (sim step share on server, client sim share on laptop); (d) arithmetic table must live somewhere permanent (C1.9 cites GLOSSARY's tick-budget section -> check glossary has it); (e) owners log C1.1, C1.8 (base of 25 % now the period + interest/sends added), C1.9-11, C1.15-18 as recommendation changes.
- #5 APPROVE (aligned ABL-PREDICT; interim default = not predicted, both answers need no redesign).
- #6 APPROVE.
- #9 APPROVE with one wording fix: "a stated rule" -> "R2's stated assumption (one vehicle per squad of 4 ...; one harvester per team, GE6)" (R2 labels the row "assumption", r2:1758).
- #10 ESCALATE MODES-3 (Mustafa); interim wording approved. Note: CLAUDE.md says "two teams of 2–8 and three teams of 6–18 players" -> reads per team; after his answer CLAUDE.md wording needs aligning (cat. 3, Mustafa).
- #11 APPROVE.
- #17 ESCALATE BASELINE-WHEN (Advisor); interim C17.1-9 approved; (b)-only set approved.
- #19 APPROVE; nit: "0 USD on Pages' Free plan" -> say the price is the plan's name (no price page read) in C19.3/C19.8.
- #21 ESCALATE PREVIEW-WALL (Mustafa); REVISE interim: C21.1 must keep R5's unreleased-art trigger as is (not fold into PREVIEW-WALL, which was asked about the staging server only); C21.2 "its staging-server part is shared as"; C21.4 wording contradiction ("talks only to staging" + "no preview talks to a game server"); add a line to R6-HOST-REC3 that previews use the staging server only once PREVIEW-WALL is answered.
- #22 APPROVE (rank 41 nit folded in C22.1 correct: R1-BND-REC5 says "passes REC2's threshold").
- #23 ESCALATE BASELINE-WHEN; interim C23.1-3 approved; (b)-only REVISE: C23.b2 must also fix "It needs no TCP 80 on game VMs." and the Reason (C2's), C23.b1 parenthesise C2's description.
- #29 APPROVE with nit: C29.2 "no default-hasher HashMap" -> "no std HashMap or HashSet" (the ban is by type path, any hasher; R5-RUSTLINT-REC2).
- next: glossary, cross-topic-oqs, EA question.
