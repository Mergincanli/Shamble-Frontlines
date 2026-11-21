# Advisor: check of c49773f (TE2-5 (c), TE-M3's cause and fix)

Checked 2026-11-13 at effort xhigh, working from main's `.claude/agents/advisor.md`. This is the check my TE-M3
ruling asked for (advisor-te-m3.md, TE-Q4 and "What the main session does next" item 7). It is not a review round.
Notes: advisor-te-m3-check-notes.md. Branch head now 87d80dd. Nothing under spikes/ has changed since c49773f
(`git diff --stat c49773f HEAD -- spikes/` is empty).

## Verdict: APPROVE

The commit is what the plan and my conditions asked for, and nothing more. All seven conditions are met. The logs
support every number in the record. CI is green on c49773f. My own 300 runs under the same load all passed. One Low
remains (CK-1, the card's wording). It does not block, and it rides the card update that comes next anyway.

Risk tier 1 (a test file in a spike). Nothing for a player to see, hear or feel. Nothing for Mustafa.

## What I checked myself
- **The diff** (`git show c49773f`). It touches 2 files: `spikes/netcode-proof/crates/net/src/l2/delay_tests.rs`
  (+10 −3) and `.checkpoints/netcode-proof-test/te-m3-plan.md` (the record).
  - The doc comment, from "so the pump runs at most 3 times" onward, is AM-2's text word for word. I joined the
    comment lines and compared them with the text.
  - The assertion matches plan §3.1: `s_calls <= polls + 1`, with its message, in rustfmt's layout. The string's `\`
    continuation produces "under way", as P4 (b)'s panic text shows.
  - Every changed line is at most 120 columns.
  - No other line changed: `polls <= 3` and `polls_after_deadline == 1` stay.
  - The two assertions now sit at :2607 (no spin) and :2608 (S's calls).
- **The bytes.**
  - At c49773f, delay_tests.rs's blob has sha256 74d49de7… (17c63a39… at its parent).
  - delay.rs's blob is aaa6a588…, unchanged since cc8595d (spikes/ at c49773f^ equals cc8595d).
  - The working tree equals both blobs.
- **CI.** Run 43428174558 (push) and run 43428182600 (pull_request) both have headSha c49773fdc8dc…. In both, the
  `gate` and `smoke` jobs succeeded. No other run exists on that commit.
- **The gate, myself** (main folder, head 87d80dd):
  - `node tools/gate.mjs --no-smoke`: GATE PASS, exit 0.
  - The full gate passes checks 1 to 4 and 5b to 5d. 5a (smoke) fails only because this container has no Playwright
    Chromium (`/opt/pw-browsers/chromium_headless_shell-1194` is missing).
  - The branch changes no `legacy/` file against origin/main, and CI's smoke job is green on c49773f. So the smoke
    result stands on CI.
- **The logs under .gate/te-m3/.** Every tag there, in time order: build/run (the first loop), one-h6 (P1), adv-h6 (my
  P1), p4a, p4b, p2, p3, p5, p6. No tag is missing and none was repeated, so there was no hidden retry.

## Conditions 1 to 7
1. **AM-1, P2's pass criterion: met.** `te_m3_tally.py p2` gives:
   - 600 run logs and 600 own lines, with 0 runs that do not have exactly one;
   - exit codes {0: 600};
   - the triples (2, 1, 2) 591, (2, 1, 3) 1, (3, 1, 2) 3 and (3, 1, 3) 5;
   - "runs with S calls = runs + 1: 1".
   That one run is p2-2-34.log: (2, 1, 3), passed, 189456 ns after the deadline. All 600 logs read "1 passed;
   0 failed", and none panicked. The record gives the whole tally and the command that made it.
2. **AM-2 and AM-3, the comment and the record: met.**
   - The doc comment is AM-2's text (see the diff above). It names no waker.
   - The record's mechanism line names both wakers (the clock's fire for the model's last event, and `until`'s step)
     and says that the bound needs neither.
   - The assertion is in rustfmt's layout. check.mjs's fmt step passes in P5.
3. **AM-4, P4 by message: met.**
   - (a): exit 101, "panicked at crates/net/src/l2/delay_tests.rs:2607:5: no spin while waiting: 22632 runs". Its own
     line: 22632 runs, 22632 S calls.
   - (b): exit 101, ":2608:5: 4 shutdown calls for 2 pump runs: at most one per run, and one more for a run already
     under way when the wait began". Its own line: 2 runs, 4 S calls.
   - Each plant was built: p4a-build.log and p4b-build.log both show "Compiling npt-net".
   - delay.rs was restored. P2's start, 20 s after p4b, prints aaa6a588…, the committed blob.
4. **AM-5, P3's stop rule: met.** te_m3_loop.sh stops every loop at the first non-zero exit, names it and keeps each
   run's whole output in its own file. That rule was in force in both modes.
   - P3 had no failure: 20 of 20 runs exited 0, each reading "169 passed; 0 failed; 12 ignored", and none panicked.
   - So nothing had to come to me.
5. **AM-6, the risk tier: met.** te-m3-plan.md line 8: "Risk tier 1 (a spike's test file); no Brave check; reviewers
   per TE-Q4".
6. **AM-7, the bytes behind each run: met.**
   - P2 and P3 each print the sha256 of both files at their start (74d49de7… and aaa6a588…), plus the "Compiling
     npt-net v0.0.0 (/home/user/Shamble-Frontlines/spikes/netcode-proof/crates/net)" line.
   - P5 prints both sha256 at its start but has no Compiling line, because check.mjs prints only its own step lines.
     The record says so. I add two facts:
     - the shared target's npt-net test binaries npt_net-088534cb… (12:38:31) and npt_net-ed8673ee… (12:41:49) were
       rebuilt inside P5's window (12:36:18 to 12:46:02);
     - main-run.sh touches every file under crates/ first.
     So P5 built the committed bytes.
   - P5's results: 11 of 11 steps PASS, 491 / 561 tests, 0 failed, self 71 of 71, marks 81, redact 264 items with
     0 findings.
7. **P1 listed twice: met.** The record lists both one-h6-1-4.log and adv-h6-2-44.log. Each panics at
   delay_tests.rs:2605:5 with "3 shutdown calls", and each run's own line reads (2, 1, 3).

## My run of the fixed test (the same load)
- **Command:** `bash .checkpoints/netcode-proof-test/main-run.sh .gate/te-m3/adv-check.log bash
  .checkpoints/netcode-proof-test/te_m3_loop.sh adv-check one 150 2 6`.
- **Setup:** the lane lock was held from 12:52:51 UTC, and the runner was idle after 0 s.
- **The bytes at its start:** sha256 74d49de7… and aaa6a588…, and the "Compiling npt-net" line.
- **The binary:** npt_net-301de27e05915ce7, built at 12:52:53. It contains the new message string ("one more for a run
  already under way when the wait began"; `grep -a`: 1).
- **Result:** 12:52:53 to 12:53:58. **300 of 300 runs exited 0**, main-run.sh exited 0, and the first failure was
  "none".
- **The tally** (`python3 -I .checkpoints/netcode-proof-test/te_m3_tally.py adv-check`):
  - 300 run logs and 300 own lines, with 0 runs that do not have exactly one;
  - exit codes {0: 300};
  - (2, 1, 2) 294, (3, 1, 2) 1, (3, 1, 3) 5;
  - "runs with S calls = runs + 1: 0".
- **Also:** no run panicked, and none had polls = 4. Lateness ran from 0.095 to 15.3 ms, with a median of 0.24 ms.
- **Honest limit:** the case the fix covers did not happen in my 300 runs. So my run proves there was no other
  failure under this load. It does not exercise the +1 itself. P2's p2-2-34.log does that, and 3 such runs in about
  1,000 runs under load (P1 twice, P2 once) fit a rare overlap.

## Findings (CLAUDE.md's one scale)
- **CK-1, Low: the card's TE-M3 entry still carries text from before the commit.** The entry correctly says
  "COMMITTED c49773f" and "CI green". But it goes on: "Then P6 (gate), ONE commit (delay_tests.rs and the record),
  push, CI, the advisor's check of the commit. If cut: the working tree holds the edit (delay_tests.rs 74d49de7…) and
  no plant (delay.rs aaa6a588…); re-run the interrupted P-step." Those steps are done, so a resumed session could
  re-run them. **Fix:** in the card update that comes next (item 1 below), delete those two sentences. In their place
  write: "Advisor's check: APPROVE (advisor-te-m3-check.md); TE-M3 closed at c49773f."
- **Observations (no finding, no change asked):**
  - te-m3-plan.md §4 still words P2 to P4 as in rev 1. The pass criteria that held are my conditions
    (advisor-te-m3.md), and §7 records each one as met. When the plan is folded into the report, cite the conditions
    as the pass criteria.
  - P4's logs carry no sha256 line. The runner gained its AM-7 lines after P4 ran, and AM-7 asked for P2, P3 and P5
    only. P2's start shows the restore.
  - My earlier Observation (polls = 4 on correct code) has not appeared in 900 runs of the fixed test (P2 and mine).
    It stays watched, as stated there.

## What the main session does next
1. **The card** (my TE-M3 ruling's item 8, with CK-1):
   - the TE-M3 entry points to c49773f and to this check, with CK-1's fix;
   - "Rules in force" keeps TE2-5's rule until U4r's merge (TE-Q2). It already says so.
   - P5 had no test-step failure, so that half of TE-Q2's condition holds so far.
2. **U4r's brief:** its TE-M3 line carries TE-Q3's words, the commit c49773f and the rule's end (at U4r's merge, and
   only if U4r's counted and judged runs have no check.mjs test-step failure). Edit the brief only after the other
   advisor's check of it has returned, never while that agent works on it.
3. **U4r's counted and judged runs watch this test like every other.** Any failure of it (polls = 4 included) or of
   any other test in check.mjs's test step comes to me whole, under TE2-5. Never a retry.
4. **The spike's pull request** gets its final review with the test-engineer seat (CLAUDE.md step 5). This diff is
   part of it (TE-Q4).
5. **Keep the logs:** .gate/te-m3/ as it is, adv-check-* included, until the spike's report has folded the record.

## What I did not check, and why
- **The plants' exact code.** No diff of them was kept. Their effects match the record: thousands of runs for (a),
  and 4 calls in 2 runs for (b). The restore is shown by delay.rs's sha256.
- **Smoke locally.** This container has no Playwright Chromium. CI's smoke job is green on c49773f, and the branch
  changes no legacy/ file.
- **P5 myself.** I did not re-run check.mjs. I read its log and the shared target's build times instead. I am not
  re-checking P5's whole-workspace result: the diff touches one test, and my run, P2 and P3 cover it.

## Decisions needed
None. Nothing here is Mustafa's.

## For Mustafa
Nothing.
