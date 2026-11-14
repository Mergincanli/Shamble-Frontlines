# Advisor: the conformance check of BRAWL-1's builder brief (workflow wf_12d3f66b-ca1), 2026-11-15

Written by script, word for word, from the workflow's journal (advisor-brawl1-brief's StructuredOutput result). The brief: brawl1-brief.md. Its notes: advisor-brawl1-brief.md. Nothing below is the main session's.

## Verdict: CONFORMS AFTER EDITS

BRAWL-1's builder brief carries the approved plan and my 18 conditions closely. Conditions 2, 5 and 12 are word for word, and N2-1's message and RR4-L1's const are the rulings' own text. It decides nothing that has to come back to me. I checked these facts and they hold: the runner's sha256 is 6efb9157…729bc8, and its log-path check accepts any file under .gate/u4r/, so .gate/u4r/brawl1/ is fine. The clean pair's "## pair clean" section is at runs.txt:16-125, and c-u4r-runs.mjs:509 runs exactly condition 6's command with no extra arguments. The cited lines hold: precheck.rs:691 and :705-706, room.rs:691, scenario.rs:1442, net/src/l2/socket_tests.rs:512, client lib.rs:162, sim ticks.rs:35, world.rs:701 and :120, and seam.rs:346. No commit has touched crates/ since faf42aeb, and ops/runs last changed at 493b3d7.

23 gaps, all to be fixed by exact edits:
- **Two wrong facts in the disk section (Medium):** the brief says about 13.2 GiB is free, but the true figure is 13,205-13,225 MiB, about 12.9 GiB. It also says the runner checks for 4 GB before every command, but it checks once per call, before the first command.
- **One plan line dropped (Medium):** the "Not in this lane" list. Without it, nothing tells the builder not to edit npt-sim, and Q1's ruling keeps EA-U4R-4 shut.
- **Condition 18 (Low-Medium):** it is narrowed to "where they touch your work" and its words are missing.
- **Low items:**
  - plan words dropped: "scaled to full command length", the sideways part's spec, TURN_EVERY and RETARGET_EVERY, None when there is no own section, the summary text, the synthetic entries, the last tick's definition, the record of the messages, the RR3-L2 test, and the one-loop driver;
  - condition text dropped: 1's record (door-entry margin, spawn corners), 3 (i)'s "step count with no time claim", the half-coverage note of 6, "building" in 8, and the merge-step clauses of 7, 9 and 11;
  - the goal line;
  - the RR4-L1 ruling's "three committed manifests still read", which the plan dropped and my review missed.

Six new decisions are all small process or stricter-than-ruled choices, and all may stay. Apply E1-E23 word for word and launch. No second check is needed if the brief's diff holds only these edits. Notes: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/advisor-brawl1-brief.md.

## Edits

1. **Where:** E1. Section 1 item 5 (Disk), first line
   - **Gap:** Wrong fact (Medium; condition 16): 'about 13.2 GiB is free now'. The plan says 13,225 MiB, and df shows 13,205 MiB at this check: about 12.9 GiB. 13.2 - 8.2 would give 5.0, not condition 16's 4.7.
   - **Words:** Replace 'about 13.2 GiB is free now and the shared target is cold.' with: 'about 12.9 GiB (13,205 MiB on 2026-11-14) is free now and the shared target is cold (1 MiB).'
2. **Where:** E2. Section 1 item 5, the sub-bullet 'The runner checks 4 GB before every command.'
   - **Gap:** Wrong fact (Medium): u4r-run.sh (sha256 6efb9157…) reads df once per call, before the first command. It never checks again between the commands of a '::' sequence.
   - **Words:** Replace it with: 'The runner checks 4 GB once per call, before its first command, never between the commands of one call; rule 6 applies to every build inside a call.'
3. **Where:** E3. Section 3, Part 2 item 2 (npt-bots), the bullet 'past R_OUT, …'
   - **Gap:** Plan section 2's 'scaled to full command length' is dropped (Low). Condition 1 bounds the vector but did not replace its length.
   - **Words:** 'past R_OUT, the walk vector becomes a vector toward the origin, scaled to full command length within x^2 + y^2 <= 127^2, so the sim never takes its scale path;'
4. **Where:** E4. Same item, the bullet 'the sideways part may stay or go;'
   - **Gap:** Plan section 2's spec of the sideways part, if it is kept, is dropped (Low).
   - **Words:** 'the sideways part may stay or go (condition 1). If it stays, it is the plan's: deterministic, used only when the steer is straight along one axis, never 0 after the 1 cm snap (|m| > 3 of 127), and inside the 127^2 bound;'
5. **Where:** E5. Same item, after 'Record every tuned value.'
   - **Gap:** Two things are dropped (Low). Plan section 2: TURN_EVERY and RETARGET_EVERY stay command counts. Condition 1: the record's door-entry margin and the spawn grid corners.
   - **Words:** 'The walk's TURN_EVERY and RETARGET_EVERY stay pure command counts. Your record of the leash gives its margin as the door-entry margin (a soldier's centre at 1,800 cm at building 0's door, x 1,800 with |y| < 60: greybox.rs:83-136), never "2.6 m from any footprint", and notes that the spawn grid's corners lie at 1,414 cm (world.rs:703-722; scene.rs:153).'
6. **Where:** E6. Section 3, Part 2 item 1 (the read item), after '…with ticks compared by the endpoint's wrap rule.'
   - **Gap:** Plan section 2's second None case is dropped (Low).
   - **Words:** 'It also returns None when the newest complete, filed tick has no own section (plan section 2).'
7. **Where:** E7. Part 2 item 4, the sim-only test, the Setup bullet
   - **Gap:** Plan 4.2 says what drives the test and where its position comes from. Both are dropped (Low).
   - **Words:** Add: 'It drives npt-sim's World through the crate-private decision function. The position fed in is World's own record, standing in for the decoded position: with no network they are equal, and the test checks steering, not decoding.'
8. **Where:** E8. Part 2 item 4, Pattern (i)
   - **Gap:** Condition 3 (i)'s words are dropped (Low; carry 39).
   - **Words:** 'Pattern (i), barred: a command every tick, each slot fed its position from 8 steps earlier: the largest lead a run takes (--lead's cap), as a step count with no time claim.'
9. **Where:** E9. Part 2 item 4, the bullet 'The steering unit test (plan section 4.1's list)…'
   - **Gap:** The list is cited, not carried, and plan 4.1's text names next_command and tests/, both replaced by condition 4 (Low).
   - **Words:** 'The steering unit test calls the crate-private decision function with set positions and shows: inside R_IN, the walk as today; past R_OUT, the steer toward the origin; the hysteresis between them; the sideways part on each axis-aligned case (if kept); None and dead keep the walk; under a skip pattern (the last command re-run) and blocked steps (positions that do not move), the steer stays toward the origin until inside R_IN. It also asserts x^2 + y^2 <= 127^2 on every leash vector.'
10. **Where:** E10. Part 2 item 3 (the figure)
   - **Gap:** Plan section 3's exact summary text and the record's place are dropped (Low).
   - **Words:** After the first sentence add: 'A new name in room.jsonl's tick record (you name it) holds the list.' Replace 'the summary line prints the total' with: 'the summary line prints it as "in a building: N slot-ticks (slots …)"'.
11. **Where:** E11. Part 2 item 5 (the re-pin), after 'Every other field and both "about" texts stay.'
   - **Gap:** A plan section 5 sentence is dropped (Low).
   - **Words:** 'The three synthetic entries in recordings.json do not move.'
12. **Where:** E12. Part 1 item 4 (TE-R2-2 with RR4-L2)
   - **Gap:** Two plan section 6 / RR4-L2 items are dropped (Low): the run's last tick's definition, and 'the builder words them, and they go in the record'.
   - **Words:** First refusal: 'a run's last tick (measured_end + after_max_ticks) above u64::from(u32::MAX);'. After 'Each refusal is a usage error naming its options.' add: 'You word the messages; each goes in your notes and report.'
13. **Where:** E13. Part 1 item 6 (RR3-L2 (a))
   - **Gap:** The plan's unit test content and 'finish reads it' are dropped (Low).
   - **Words:** Replace the last two sub-bullets with: '`last` stays (finish reads it); a new logs.rs unit test: Logs::open(None, ..), admit, filed over many ticks, then `res` is empty and started() is true.'
14. **Where:** E14. Part 1 item 2 (EA-U4R-1, the narrow form)
   - **Gap:** The plan's exclusion is dropped (Low).
   - **Words:** Add: 'The one-loop driver is not built.'
15. **Where:** E15. Part 1 item 5 (RR4-L1), the r10_fp3 line
   - **Gap:** RR4-L1's ruling (advisor-u4r-r2-full.md:94, which the brief makes binding) also asks that the three committed manifests still read. The plan replaced that clause and my review missed it (Low). Carrying it adds nothing new.
   - **Words:** 'r10_fp3: 36,001 is refused by name, a synthetic entry at 36,000 passes, and the three committed manifests still read (RR4-L1's ruling).'
16. **Where:** E16. Part 1's proofs, after (c)
   - **Gap:** Condition 6 asked for a statement of which proof covers which half. It is missing (Low).
   - **Words:** '(a) to (c) prove only scenario::run's half: they regenerate through `scenario --record`. (d) and (e) prove run_leg's half.'
17. **Where:** E17. Part 1's proofs, (b)
   - **Gap:** Condition 8 says 'builds or runs'; the brief says 'running' (Low).
   - **Words:** Replace 'with no other lane running' with 'with no other lane building or running'.
18. **Where:** E18. Section 2 (Limits), a new bullet
   - **Gap:** Plan section 6's 'Not in this lane' is not carried (Medium). Nothing tells the builder that npt-sim is not edited, and Q1's ruling keeps EA-U4R-4 shut.
   - **Words:** '**Not in this lane** (plan section 6): EA-U4R-4 (no npt-sim edit: the leash needs none, Q1); T-6's contract half (contract.rs is not edited); JDG-ONLY-1 (no judge change); and the check.mjs follow-up.'
19. **Where:** E19. Section 4, condition 9's M-list items 2 and 3
   - **Gap:** Condition 7's reproduction clause, condition 9's 'on the merged tree' and its --no-smoke clause are dropped (Low; main-session items, but the brief restates the list).
   - **Words:** Item 2: 'one counted main-run.sh call on the merged tree: uptime, check.mjs with all 11 steps, det-u4.mjs, and the precheck regeneration at the new pin, where each new pin must be reproduced (one that is not comes to the advisor whole: condition 7);'. Item 3: 'the gate (--no-smoke, with the reason, if Chromium cannot start);'.
20. **Where:** E20. Section 6 (your report) and the closing paragraph
   - **Gap:** Condition 18 is narrowed to 'where they touch your work', its words are missing, and it sits outside the report list (Low-Medium).
   - **Words:** Add report item 8: 'The advisor's two corrections, in its words: ADV-BP-1: N2-1's "all four checks fail" was wrong; under the leave plant two fail (Q5). ADV-BP-2: the advisor's sets-r2 figure of "about 4.2 GiB free after the refill" mixed units.' In the closing paragraph replace 'Your report must record the advisor's two corrections (ADV-BP-1, ADV-BP-2) where they touch your work (condition 18).' with 'Your report records both corrections (item 8; condition 18).'
21. **Where:** E21. Section 0 'What is not yours', and the closing paragraph
   - **Gap:** Plan sections 7 and 8's disk cleans after the merge are not named, and condition 11's engine-architect scope is dropped (Low).
   - **Words:** Add to the list: 'the disk cleans (B) and (C) after the merge (plan section 8);'. In the closing paragraph, after 'each at xhigh', add: '; engine-architect reviews the npt-client pub item and the form of EA-U4R-1 and EA-U4R-2'.
22. **Where:** E22. Section 0, a new first bullet
   - **Gap:** Plan section 1 (the goal) is not carried (Low).
   - **Words:** '**The goal** (plan section 1): the brawl's walk is open loop; a re-run command, or a step that collide() cuts short, leaves an offset nothing corrects, so soldiers drift into buildings and the room rightly drops them. The fix: each brawler steers by its own soldier's decoded position, never by server state.'
23. **Where:** E23. Section 0 'What is not yours', the SERIES.md item
   - **Gap:** The citation is wrong (Low): condition 10 does not assign the drafting. Plan section 7 makes the row the main session's Edit.
   - **Words:** 'the marks/SERIES.md edit (plan section 7: the main session's Edit; you draft the rows for it);'

## New decisions

1. **Where:** Section 0, 'What binds you, in order' (review, then plan, then the earlier rulings)
   - **What:** A precedence order: where the plan and the review differ, the review wins.
   - **Ruling:** May stay: my review amends the plan, and the earlier rulings are what the plan answers.
2. **Where:** Section 1 item 4
   - **What:** -x (exclusive) also for det-u4.mjs, and every log under .gate/u4r/brawl1/ (condition 13 said only under .gate/).
   - **Ruling:** May stay: it follows the runner's own rules (the log check accepts .gate/u4r/*) and condition 8's no-other-lane rule.
3. **Where:** Part 2 item 4, 'A soldier the full test names stops you before anything else'
   - **What:** Stricter than condition 2 ('comes to me before the 10-minute runs'): it is a builder stop.
   - **Ruling:** May stay: a re-tune would move the pins again, so stopping before the re-pin is right.
4. **Where:** Section 0 and section 4, the SERIES.md rows
   - **What:** The builder drafts the rows in its notes, and the main session edits SERIES.md.
   - **Ruling:** May stay: the builder names the fields. The citation is fixed by E23.
5. **Where:** Part 1 proof (b) and Part 2 item 5
   - **What:** 'uptime first in the same runner call' (u4r-run.sh) where condition 8 names main-run.sh.
   - **Ruling:** May stay: u4r-run.sh is the builder's runner in its worktree, and main-run.sh stays at the merge step (condition 9).
6. **Where:** Section 1 item 1 and section 4
   - **What:** A start record (worktree path, branch, HEAD equal to <base>, empty status), and wip/brawl1.patch saved at each snapshot.
   - **Ruling:** May stay: process only, and it matches U4r's practice.

## For Mustafa

Nothing new from this check. It decides nothing that is yours. The one open question, KEPT-LOGS (whether a narrow rule may clear old test-run logs; I recommend (c), pack then remove), already went to you with the plan review, and the bot fix does not wait for it.
