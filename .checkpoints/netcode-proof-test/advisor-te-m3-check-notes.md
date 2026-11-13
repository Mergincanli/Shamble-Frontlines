# Advisor: check of c49773f (TE-M3's cause and fix), notes

Task: the check my TE-M3 ruling asked for (advisor-te-m3.md, TE-Q4 and "What the main session does next" item 7).
Effort xhigh. Read-only on source; notes and ruling via Write/Edit only.

## Plan
1. Read advisor.md (main) - done. Read my TE-M3 ruling (advisor-te-m3.md + its notes) and te-m3-plan.md section 7.
2. Read the diff `git show c49773f`; check it against the plan (symbols, scope, files: only npt-net's test?).
3. Check conditions 1-7 against the record and the logs in .gate/te-m3/ (p2 tally, p3, p4a/p4b + run logs, p5-check,
   p6-gate).
4. Check CI: runs 43428174558 (push) and 43428182600 (pull_request) on c49773f (gh, read only).
5. Run the fixed test myself under the same load (main-run.sh ... te_m3_loop.sh adv-check one 150 2 6); wait for the
   other advisor's lock if held; tally; any failure kept whole, ruled on, never retried.
6. Write the ruling advisor-te-m3-check.md; hand back.

## Log
- Block 1 done: read advisor.md (main), advisor-te-m3.md, te-m3-plan.md (sections 4 and 7).
- Block 2 done (the diff): c49773f touches 2 files: delay_tests.rs (+10 -3) and te-m3-plan.md (record). Doc comment
  from "so the pump runs at most 3 times" onward = AM-2's text word for word (joined and compared). Assertion =
  plan 3.1's bound and message, rustfmt layout, `\` continuation gives "under way". Changed lines <= 120 cols.
  Blobs at c49773f: delay_tests.rs 74d49de7..., delay.rs aaa6a588... (unchanged since cc8595d; spikes at c49773f^ ==
  cc8595d). Parent's delay_tests.rs 17c63a39... Working tree equals c49773f for both; HEAD now 87d80dd, no spikes/
  change since c49773f. New lines: polls assert :2607, s_calls assert :2608.
- Block 3 done (logs): P2 tally: 600 logs, 600 own lines, 0 bad, exits {0: 600}; (2,1,2) 591, (2,1,3) 1 (p2-2-34.log,
  passed, 189456 ns late), (3,1,2) 3, (3,1,3) 5; S = runs+1: 1. 0 panics; all 600 "1 passed; 0 failed". sha256 +
  Compiling npt-net at start. P3: 20 of 20 exit 0, "169 passed; 0 failed; 12 ignored", 0 panics, sha + Compiling at
  start. P4a: exit 101, :2607:5 "no spin while waiting: 22632 runs", own line 22632/22632. P4b: exit 101, :2608:5
  "4 shutdown calls for 2 pump runs: ...", own line 2 runs 4 calls. P4 logs carry no sha line (runner's AM-7 lines
  added after; AM-7 asks P2/P3/P5 only); P2's start shows delay.rs aaa6a588 = restored. P5: sha at start, 11 of 11
  PASS, 491/561, self 71/71, marks 81, redact 264 items 0 findings; no Compiling line (check.mjs output) - but
  shared-target npt_net-088534cb (12:38:31) and npt_net-ed8673ee (12:41:49) were rebuilt inside P5's window. P6: GATE
  PASS (--no-smoke). No other tags in .gate/te-m3 (no hidden retry): build/run, one-h6, adv-h6, p4a, p4b, p2, p3, p5,
  p6, in that order.
- Block 4 done (CI): runs 43428174558 (push) and 43428182600 (pull_request), both headSha c49773f..., gate + smoke
  success; no other runs on that commit.
- Block 5: my run next (main-run.sh, tag adv-check, one 150 2 6).
