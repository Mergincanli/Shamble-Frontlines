# Checkpoint: kit-hold-ci
- **Goal:** one small pull request with three things and nothing else: the checkpoint script holds a push that changes
  a manifest or a lockfile until the security check of that change has answered (his DEP-HOLD (a), an exception to
  the kit freeze for this one change), and the two CI edits in `.github/workflows/toolchain.yml` (CI-TYPES: a type
  check and a regenerate-and-compare step; CI-CUT part 1: no cache in the toolchain job), each with the exact text
  of the report of change-set 1 at head 1bfe462, section "The exact texts asked for". His words (2026-11-18,
  15:38:13 UTC, his label 16:24 Amsterdam; `kit-hold-ci/mustafa-2026-11-18-1624-merge-58.md`, by script).
- **Branch:** claude/wonderful-hamilton-jdmlnf (stands in for cs/kit-hold-ci) · **Base:** main
  9b4b518f520aa2360a8007dc857ba74b343c8a40 (the merge of #58; the branch was fast-forwarded to it and pushed)
- **Last update:** 2026-11-18 16:41 CET (from the clock)

## What his words ask (read the file; these are pointers, not his words)
- DEP-HOLD (a): "Show me its exact text first, with how the hold is released. Keep it to that: no guard change and no
  longer watch list now." The CI check of option (c) goes on the list for the quality gate v2's plan.
- CI-TYPES (a): the exact text as in the report at that head. "Its pull request shows two things from a real run: the
  compare step failing once on a planted change, and passing on the clean tree."
- CI-CUT (a): part 1 now with the exact text; part 2 waits for the gate v2's workflow. "Its pull request shows "Cache
  mode: none" in the job's log."
- LANE-ISSUE (a): this chat opens the issue. "A comment there is never my word." WATCH-58 (b): no.
- After the merge: "One small pull request with the kit hold and the two CI edits, nothing else in it. Its ready
  report goes to the Advisor before my go." "You may write change-set 2's plan beside it. No new library of
  change-set 2 is committed before that pull request is merged."
- What he accepted with the merge of #58: the two misses as written; the size 940; round 2's Lows as follow-ups ("The
  two doc lines that claim more than the code does are corrected in the next pull request that touches those
  files."); what was not verified.

## Steps done
1. Pull request #58 merged on his go of 15:38:13 UTC: merge commit 9b4b518, 2026-11-18 15:39:28 UTC, parents 22c6afa
   and 1bfe462, tree equal to the approved head, his words in its message byte for byte (2,618 bytes, by script).
   Before the merge: the head read again (1bfe462) and its six check runs (gate, smoke, toolchain, for the push and
   the pull request): all success. After it: the branch fast-forwarded to main and pushed.
2. Session check at 15:38 UTC (get_session): model claude-fable-5-2 (configured and last served); context 671,689 of
   1,000,000 tokens (automatic compaction at 901,600): ask him to compact at a natural stop.

## Current step
3. Open the "Lane requests" issue. Read `.claude/scripts/checkpoint.mjs`, its test and `.claude/CHECKPOINTS.md`;
   write the plan with the exact text of the kit edit and how the hold is released; the advisor agent on the plan;
   then show Mustafa the exact text and ask for his yes (and for the chat in Accept edits: the guard asks on every
   `.claude/` and `.github/` edit). No kit or CI file is edited before that.

## Next steps
4. Build (three edits), the script's tests, the real CI runs (a planted change that the compare step fails on; the
   clean tree; "Cache mode: none" in the job's log), reviews (risk tier 2: the advisor agent and security-auditor;
   infra-engineer for the workflow), the verifier, the docs pass (DECISIONS by script: his go on #58 and his five
   answers; STATUS, ROADMAP, the map for #58's merge and this change), the report, the pull request; its ready
   report goes to the Advisor before his go.
5. Change-set 2's plan may be written beside it (no new library committed before this pull request is merged).

## Open findings
- From #58 (STATUS has them): round 2's Lows; the two doc lines to correct with the next change to those files.
- The gate v2's plan list gains: a CI check that refuses to build when the lockfile gains an entry with no recorded
  review (his words on DEP-HOLD's option (c)); CI-CUT part 2.

## Running work
- saver: to start. agents: none.
