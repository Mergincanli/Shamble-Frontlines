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
- #1 more findings (REVISE, substantive but recommendation stands):
  * table omits R4-NAV-REC3 avoidance <=0.2 ms p99 (r4:860) and R2-INT-REC2 (b) pair test <=0.1 us/enemy pair + re-flood <=1 ms after a portal change. Recomputed: sim 60Hz N48 3.87 (left 0.30), N54 4.14 (left 0.03); 30Hz 5.53/5.81. server-only 60Hz N48 3.41, re-flood tick 4.41 > 4.17; N54 3.85 / 4.85; whole 60Hz re-flood+snapshot tick N48 8.28 (<8.33), N54 8.99 (>8.33). B25 still worse -> B50 stands, but claim "marks add up inside at 60 and 30 Hz, N48 and N54" is false; rewrite point 1, C1.9 reason, glossary §1 intro.
  * "sums of p99s are an upper bound" (res 1 + glossary Time statistics) is false: quantiles are not subadditive (two parts 1 ms on 0.6 % of ticks each: p99 0 each, sum's p99 1 ms). Reword as planning check; whole measured directly.
  * physics row is a mean mark mixed into p99 sums: say so.
  * stress scene all-bots -> encoding for 0 clients; server marks need N receiving (headless) clients (R2-SYNC-REC9 mark 1 "48 clients"). Add to glossary stress scene + C1.10.
- glossary findings: very large unit defined as ">=20 parts" turns a test parameter into a design term -> reword; shared-stream example "ability procs" pre-decides a cheat trade -> drop; access wall "previews that reach no game server are public" contradicts R5 art trigger -> fix; status/depends lines to update after approval; stress scene 15 = R2's assumption wording.
- next: cross-topic-oqs.md
- line-number check: all 116 before-quotes start within +-3 lines of the stated line (scratchpad adv/lines.py).
- cross-topic-oqs: 29 items = critic's list; 72 insertions; per-file R1 18/R2 15/R3 8/R4 8/R5 14/R6 9 recomputed from JSON; texts = critic's why+decided_by verbatim (ranks 2,15,25,30,39,41 compared). Verdict APPROVE with nits: rank 15 note (res 21 interim rests on offline previews; R1-BND-OQ1 Mustafa's); in-file line refs in inserted text (r4:1110-1129, r6:757) get "at commit 8dee746". rank 41 via C22.1 OK.
- EA question: YES with rule: skip only marker-identified records ([Fix pass…]/[Second pass…]/[Verifier…] brackets, verifier/spot-check tables and fix logs under headings naming them) + quotes; script lists skipped hits per topic in report; where an old term in a record could mislead, owner adds the new term outside the record (as C6.6). Never edit records.
- #21 new C21.8: R6-HOST-REC3 r6:655-656 before `need no server\n  (R1-BND-REC6).` (unique) + "Until Mustafa answers PREVIEW-WALL (below), previews do not use the staging server."
- #23 (b)-only adds: r6:669 "It needs no TCP 80 on game VMs." and r6:670-671 Reason.
- conditions for 3c drafted (fold first; apply; log rec changes; R3 no look; glossary status; search skip rule; report; agents).
- next: write final answer. 
DONE
