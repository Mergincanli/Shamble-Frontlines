# Advisor report: guard-wrappers (2026-11-26, branch claude/confident-gates-tbnq38 for cs/guard-wrappers, base main 2483e9a7)

_Corrected 2026-11-26 (MAIN-WALL's docs pass, F2-F7 of the advisor agent's final-head review of #73): this line named base main c1da1bd; the merge base of #73 was 2483e9a7 (`git merge-base` of the merge commit 4a62873e's two parents). The red count "18" below and the "function calls only" line are corrected in place [assumed: the caller's brief of the review's F3 and F7; the review is not in the pushed notes]._

## Goal
Mustafa's SAME-GAP (a), his words: "nothing in front of git or gh gets through unread." The day guard is the seatbelt
that makes a session ask him before a kit or gate file changes, and refuses a subagent; this closes the wrapper gaps.

## Outcome
PARTIAL, merged by his decision R4-END (1): every reviewed form is closed and nothing is looser than round 3's head, but
round 4 left one Critical open (R4-1, below). His interim rule stays until R4-1's fix merges: "git and gh always start
their own command, with no other program in front of them."

## Plan and approval
Plan (`.checkpoints/guard-wrappers/plan.md`, readable at 68b91211f6336a9308a7b420d205854026eac1dd): one option table for every wrapper and keyword the
guard strips (an unknown form asks in a session, refuses a subagent); the wrapper loop's turn limit asks (SA-R1-3);
git or gh behind a program the guard does not strip asks (rule U). The advisor agent: APPROVE with 9 conditions (plan
§7: met in the copy; 8 and 9 at the apply step and here), then APPROVE on each plan change: round 1's fixes with
SA-D-DIRECT, which he chose; round 3 (R2-1 raised to Critical); round 4, hooks-r4b with ADV-R4-1 added, then hooks-r4c.
Every apply followed its rulings: built in a gitignored copy, red then green, each kit Edit asked of Mustafa with its
exact text, sha256 or cmp equal to the copy, one commit for guard and tests.

## What changed
- `.claude/hooks/routine-guard.mjs` (+55/−14 against main): the table `WRAP_OPTS` (a Map) for every stripped name;
  `unwrap()` reads options from it, counts turns, and reads xargs as R3-WAY says; `NON_RUNNERS`; rule U and the xargs
  check behind another program in `bash()`; git's own helper names read as git, with or without a program in front.
- `.claude/hooks/routine-guard.test.mjs` (+111/−3): the new rows (session asks, subagent refusals, silent controls) and
  mutants. 166 added lines of code and tests against main (his stop line: 200, CAP (a)). No game file changed.

## Evidence
- Code commits: 5df545c7 (the rule), 52e3c7ef (round 1's fixes, SA-D-DIRECT), 9ad6bc97 (round 3), 83bc2417 (round 4);
  main merged at 7be0a933 in a separate worktree (the main folder did not move: the advisor agent's ruling).
- Kit suites: routine-guard 1247 on main → 1437 → 1490 → 1499 → 1551/1551; settings 127/127; checkpoint 33/33.
- Red first: the new rows failed on the guard before each step (92 on main's; 18, round 1's count on an intermediate copy of the guard, not on a commit; 10; 45 on 9ad6bc9's).
- Gate PASS on 83bc2417 and on the merged head, checks 1-4 and 5b-5d; 5a smoke skipped here (Playwright cannot start
  the container's Chromium), so CI's smoke is the evidence. Verifier: PASS with notes, 7 of 7 (diff scope; suites and
  gate; red 45 on 9ad6bc9's guard and 195 on main's; size; every new mutant anchor once; SA-R2-2 on the hooks page;
  a sweep rerun, 0 looser, 0 throws). Not verified: smoke here; Claude Code's version outside this container.
- Sweeps, function calls only (no form was run live, except one harmless live check of the timeout wrapper with `true`): round 4, 860,160 lines, 9ad6bc9 → head: 0 throws, 0 looser,
  147,077 stricter; hooks-r4b → head (253,080 lines): only stricter.
- Against main, looser in 91 lines (ADV-R4-2), all present since round 1, by kind: 6 now pass, where the timeout
  wrapper gets a lone dash in its duration's place before a read-only program, git or gh (timeout refuses that dash
  with exit 125, so nothing runs; checked live with `true`); 85 now ask in a session instead of being refused, where
  the display-server wrapper gets a dash word or an unknown option before the program, or `command` gets a dash word
  as its program (subagents are still refused).
- SA-R2-2 (his 3c, nothing changed): a hook that runs out of time blocks the call, because the guard's three hooks set
  `"onFailure": "block"` (Claude Code's hooks reference; needs Claude Code 2.1.295 or later; this container: 2.1.332).
  A session on an older Claude Code would let a timed-out call run.

## Reviews
security-auditor, four rounds (round 4 by his leave, the last): r1 on 5df545c REVISE: SA-d Critical (older) fixed,
SA-a and SA-g Medium fixed, SA-h, SA-b, SA-f Low fixed, SA-c and SA-e recorded; the plugin review's High (an optional
value) fixed. r2 on 52e3c7e APPROVE; R2-1 fixed in round 3; R2-2, R2-3 Low deferred. r3 on 9ad6bc9 REVISE: R3-1 and
R3-2 Critical fixed in round 4 (his R3-WAY (a)); R3-3 to R3-6 Low deferred. r4 on 83bc2417 REVISE: **R4-1 Critical,
open**: one more spelling hides git, gh or xargs from the guard's reading, for a session and a subagent; part is
this change-set's class, part older; open on main too. R4-2 Low (a cost class; fails closed with onFailure) deferred.
Its answer on order: R4-1 can wait behind MAIN-WALL (branch protection, the PR review and his go keep it off main).
The advisor agent's Lows: ADV-R4-2 (above), ADV-R4-3 fixed, ADV-R4-4 deferred.

## Decisions and deviations
- His decisions: CAP (a) and SAME-GAP (a) (17:05); SA-D-DIRECT (a) (20:35); R3-WAY (a) and a last round 4 (23:59);
  R4-END (1) (15:05). Recorded word for word in `docs/DECISIONS.md`.
- Rule U is wider than "with options": it asks for any program the guard does not strip with git or gh after it, with
  or without options, because the guard cannot know which programs take options.
- The turn limit asks on a line with 20 or more stripped words or assignments in front of its command.
- The read-only list in the code leaves out `man` and `rg`, which plan §1 named: each can run another program.
- The pre-built round-4 copy had reopened R2-1; found before any edit and closed in hooks-r4b.

## Risks
1. R4-1, open Critical (above), until the next guard change-set.
2. INPUT-RUN, a named limit: programs outside the guard's table that run their input are not read (the class of the
   stated limits L1-L4, scripts and inline code).
3. False asks, and subagent refusals: ordinary xargs lines such as deleting found files, words like `git-<name>` (R2-2).
4. A throw in the guard lets a day-session call through (by design, read in guard.mjs); every sweep found 0 throws.

## Proposed next step
MAIN-WALL: a rule that only his account can update main, measured on a throwaway repository in a separate chat or by
his clicks.

## Decisions needed
None. R4-END was decided (1); the order after this merge is his (MAIN-WALL, then R4-1's fix with GUARD-KIT, then the
review tracks with SOURCE-GO (a)).

## For Mustafa
1. Your Advisor's merge check on this pull request's head (its full id is in the chat), then your go on that id.

The notes (plan, rulings, reviews, his messages) are deleted from the head and readable at 68b91211f6336a9308a7b420d205854026eac1dd under
`.checkpoints/guard-wrappers.md` and `.checkpoints/guard-wrappers/`. CI (gate, smoke, toolchain) runs on the pull
request's head; the merge needs it green.
