# touchup-test-engineer fixes (F7, F6, F12 from verifier-fixcheck-d1), docs at 449356c

F7 holds (the V-S key at 0001:1358–1359 keeps LG's 30-inputs clause for PM-0001-27; the row at 0001:1409 says "not
V-S"), but the fix goes in the key, not the row: the row is the later, deliberate alignment (test-engineer A
follow-up c, "FIX alignment (PM-0001-27: validity is PM-0006-13's, not V-S …)"), and it matches PM-0006-13
("Validity is bracket B's and LG's 80 % rule, not ADR 0001's V-S … ADR 0001's PM-0001-27 is this set of runs, read
the same way", 0006:1183), PM-0006-14 ("validity as PM-0006-13 (…, never ADR 0001's V-S)", 0006:1184) and
SPIKE_PLAN:100–102 ("share one set of runs under PM-0006-13's rules"). Adding the clause to the row alone would make
one set of runs valid in one table and invalid in the other. The density runs still carry PM-0001-2's load counters
("every run of PM-0001-4 to PM-0001-27", 0001:1380) and LG's 80 % rule with its calibration run (0006:1111–1112).
Adds one line.

### FIX F7
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
  under 80 % in every second (ADR 0006's LG key) (proposed: a starved load generator lightens the load). The first two
  clauses hold for single-room marks only: the rooms-per-VM row PM-0001-27 runs under ADR 0006's PM-0006-13 rules
  instead (no pinning, its bracket B, 3 sessions at 3 times of day, steal recorded per second and reported, never a
  validity condition), since eight tick threads cannot each be alone on four vCPUs and on SH steal is what is measured
  ("repeated at 3 times of day (shared hosts vary with neighbours)": r6-infrastructure.md, R6-HOST-REC2's mark, lines
AFTER:
  under 80 % in every second (ADR 0006's LG key) (proposed: a starved load generator lightens the load). The first two
  clauses and the 30-inputs count hold for single-room marks only: the rooms-per-VM row PM-0001-27 runs under ADR 0006's
  PM-0006-13 rules instead (no pinning, its bracket B and LG's 80 % rule, 3 sessions at 3 times of day, steal recorded
  per second and reported, never a validity condition), since eight tick threads cannot each be alone on four vCPUs and
  on SH steal is what is measured ("repeated at 3 times of day (shared hosts vary with neighbours)":
  r6-infrastructure.md, R6-HOST-REC2's mark, lines
END

F6 holds: part 4 gives the formula ("the round trip plus the input buffer plus one tick", with a 1–3 tick buffer) and
"deeper at P3 and P4" (0001:181–183), not the figures; P3 = 150 ms and P4 = 200 ms (0001:538–539). Arithmetic:
216.7 ÷ 33.3 = 6.5, 283.3 ÷ 33.3 = 8.5, 266.7 ÷ 33.3 = 8.0, 333.3 ÷ 33.3 = 10.0. Same line count.

### FIX F6
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-44 | The replay fits on the minimum laptop (part 4) | resetting and replaying the local soldier over 6 ticks (200 ms at 30 Hz: R1's mark and the budgets row as written) at most 1 ms p99, and over 7 ticks, the deepest replay part 4's arithmetic gives at P2 ((100 + 100 + 33.3) ÷ 33.3 = 7, with a 3-tick buffer), at most 1 ms p99 too (proposed by the netcode review: R1's 200 ms covers a buffer of at most 2 ticks); 12 ticks for the 60 Hz form (PM-0001-40); reported: the real replay depth per profile P0 to P4 (median and max, from PM-0001-41's runs; part 4 gives 6.5–8.5 ticks at P3 and 8–10 at P4) and the replay time forced at P3's and P4's measured maximum depths | ML: Chrome and Firefox (proposed: two Wasm engines), in Wasm in the Worker | in SS's fight at P2 for 10 minutes, the replay forced on every snapshot with exactly 6 ticks, then in separate runs with exactly 7 (proposed: with the skip on, too few and too easy replays are timed); `performance.now()` around reset and replay, isolation flag on; the vehicle case only if the sim has a piloted vehicle | 3 runs per depth, each passes (18,000 replays each; p99 = the 17,820th). Bites: rule B-S applied to the replay (a busy-wait in it) | r1 R1-TICK-REC2 P3 (lines 2058–2060); the draft, part 4 (the replay of 200 ms within 1 ms p99) and the budgets table ("Budgets for the netcode proof test"), row "Prediction replay"; r3 R3-MEASURE-F7 (lines 641–646); the draft, part 4 (the real depth, 5 to 7 ticks at P2); the netcode review of 2026-11-02, finding 10 |
AFTER:
| PM-0001-44 | The replay fits on the minimum laptop (part 4) | resetting and replaying the local soldier over 6 ticks (200 ms at 30 Hz: R1's mark and the budgets row as written) at most 1 ms p99, and over 7 ticks, the deepest replay part 4's arithmetic gives at P2 ((100 + 100 + 33.3) ÷ 33.3 = 7, with a 3-tick buffer), at most 1 ms p99 too (proposed by the netcode review: R1's 200 ms covers a buffer of at most 2 ticks); 12 ticks for the 60 Hz form (PM-0001-40); reported: the real replay depth per profile P0 to P4 (median and max, from PM-0001-41's runs; part 4's formula gives 6.5–8.5 ticks at P3 and 8–10 at P4: (150 + 66.7) ÷ 33.3 to (150 + 133.3) ÷ 33.3 and (200 + 66.7) ÷ 33.3 to (200 + 133.3) ÷ 33.3, arithmetic) and the replay time forced at P3's and P4's measured maximum depths | ML: Chrome and Firefox (proposed: two Wasm engines), in Wasm in the Worker | in SS's fight at P2 for 10 minutes, the replay forced on every snapshot with exactly 6 ticks, then in separate runs with exactly 7 (proposed: with the skip on, too few and too easy replays are timed); `performance.now()` around reset and replay, isolation flag on; the vehicle case only if the sim has a piloted vehicle | 3 runs per depth, each passes (18,000 replays each; p99 = the 17,820th). Bites: rule B-S applied to the replay (a busy-wait in it) | r1 R1-TICK-REC2 P3 (lines 2058–2060); the draft, part 4 (the replay of 200 ms within 1 ms p99) and the budgets table ("Budgets for the netcode proof test"), row "Prediction replay"; r3 R3-MEASURE-F7 (lines 641–646); the draft, part 4 (the real depth, 5 to 7 ticks at P2); the netcode review of 2026-11-02, finding 10 |
END

F12 holds: part 13 infers that "an account-scoped token can then deploy the production player build too" (0006:509–510)
and its interim (b) keeps the reviewer gate "unless OQ-0006-13 puts production where the preview token cannot reach
it" (0006:515–516), so the third leg is expected not to be refused today, yet the row passes only on "3 of 3"
(0006:1179). The row's claim (a pull request reaches no secret and no production) rests on the first two legs; the
third becomes a report for OQ-0006-13. Same line count; the three runs, control and bite stay.

### FIX F12
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
| PM-0006-32 | B9's interim position holds: a pull request cannot reach a deploy secret or production (part 13, B9; spike 1; with ADR 0002's PM-0002-66) | 3 of 3: a test pull request on TR whose deploy job is edited to print a marker gets no secret until the `preview` environment's required reviewer approves it (the job prints only whether the secret is set, never its value); a job in a pull request that names the `production` environment is refused by its `main`-only deployment-branch rule; a deploy of the production branch tried with the `preview` environment's token is refused (the preview token cannot publish production; if no token scope the check finds can refuse it, the result is recorded as not refused and goes to OQ-0006-13); recorded: how the branch rule treats a pull request's merge ref (not read), and Mustafa's revocation of the test token when the check ends | TR, CF; MP (Mustafa approves or withholds the review, and revokes the token) | three throwaway workflow runs on TR; each run's log and the environment's deployment history pasted into the results log | 1 run per case. Control: after the reviewer approves, the same job reports the secret set. Bite: with the required reviewer removed, the edited job must report the secret set (the check can fail) | the draft's spike 1, B9's bullet (proposed there, from the security review's finding 1); part 13, B9; R5-PREV-REC1 (the deploy token held only by its job, as part 13 cites it) |
AFTER:
| PM-0006-32 | B9's interim position holds: a pull request cannot reach a deploy secret or production (part 13, B9; spike 1; with ADR 0002's PM-0002-66) | 2 of 2: a test pull request on TR whose deploy job is edited to print a marker gets no secret until the `preview` environment's required reviewer approves it (the job prints only whether the secret is set, never its value); a job in a pull request that names the `production` environment is refused by its `main`-only deployment-branch rule; reported, not a pass condition: whether a deploy of the production branch tried with the `preview` environment's token is refused (part 13 expects the account-scoped token to reach production; if no token scope the check finds can refuse it, the result is recorded as not refused and goes to OQ-0006-13); recorded: how the branch rule treats a pull request's merge ref (not read), and Mustafa's revocation of the test token when the check ends | TR, CF; MP (Mustafa approves or withholds the review, and revokes the token) | three throwaway workflow runs on TR; each run's log and the environment's deployment history pasted into the results log | 1 run per case. Control: after the reviewer approves, the same job reports the secret set. Bite: with the required reviewer removed, the edited job must report the secret set (the check can fail) | the draft's spike 1, B9's bullet (proposed there, from the security review's finding 1); part 13, B9; R5-PREV-REC1 (the deploy token held only by its job, as part 13 cites it) |
END

F1a holds (lines on disk, one above the verifier's): 0005:2349–2350 (the old step 3, "a short start … may show and
then undo") and 0005:2351–2354 (GD-8's step 3, "with no start shown") are both numbered 3 and contradict each other;
the old step 4 (test-engineer-B-fixes-final.md, block "FIX GD-8", its BEFORE) is gone, though PM-0005-46 (0005:2251)
needs "his answer recorded" and SPIKE_PLAN:372 says the checklist "answers ABL-PREDICT". The fix deletes the old step 3
(2 lines) and restores step 4 (2 lines): step 4's line comes back, and the section's line count stays as it is now.

### FIX F1a
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
3. Before his team reaches Tech 1, press each ability with A and with B. Expected: the press is refused in both; in
   the predicted variant a short start of the ability may show and then undo. Record: whether that undo is acceptable.
3. Before his team reaches Tech 1, press each ability with A and with B. Expected: the press is refused in both, with
   no start shown, since the client knows its team's tier (team state goes to the team, ADR 0001) and the ability's
   required tag (AB-5), so a predicted variant can refuse the press locally. Record: any start that shows, in either
   variant, as a defect of that variant.
AFTER:
3. Before his team reaches Tech 1, press each ability with A and with B. Expected: the press is refused in both, with
   no start shown, since the client knows its team's tier (team state goes to the team, ADR 0001) and the ability's
   required tag (AB-5), so a predicted variant can refuse the press locally. Record: any start that shows, in either
   variant, as a defect of that variant.
4. Record: his answer to ABL-PREDICT (does the client predict the player's own abilities), and whether it differs per
   ability.
END

