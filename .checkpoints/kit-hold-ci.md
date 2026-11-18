# Checkpoint: kit-hold-ci
- **Goal:** one small pull request with three things and nothing else: the checkpoint script holds a push that changes
  a manifest or a lockfile until the security check of that change has answered (his DEP-HOLD (a), an exception to
  the kit freeze for this one change), and the two CI edits in `.github/workflows/toolchain.yml` (CI-TYPES: a type
  check and a regenerate-and-compare step; CI-CUT part 1: no cache in the toolchain job), each with the exact text
  of the report of change-set 1 at head 1bfe462, section "The exact texts asked for". His words (2026-11-18,
  15:38:13 UTC, his label 16:24 Amsterdam; `kit-hold-ci/mustafa-2026-11-18-1624-merge-58.md`, by script).
- **Branch:** claude/wonderful-hamilton-jdmlnf (stands in for cs/kit-hold-ci) · **Base:** main
  9b4b518f520aa2360a8007dc857ba74b343c8a40 (the merge of #58; the branch was fast-forwarded to it and pushed)
- **Last update:** 2026-11-18 17:11 CET (from the clock)

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

## Steps done (continued)
3. The "Lane requests" issue is open: #59 (15:41 UTC; body: what it is for, only comments from the `Mergincanli`
   account count, a comment there is never his word).
4. The three edits exist as scratch copies under `.gate/kit-hold-ci/` (gitignored; **no kit, gate or CI file is edited
   yet**) and as diffs in the notes: `kit-hold-ci/kit-edit.diff` (`checkpoint.mjs` +36 lines changed, its tests +87,
   one bullet in `CHECKPOINTS.md` section 3) and `kit-hold-ci/ci-edit.diff` (the workflow; built by script from the
   report at 1bfe462, blocks 1 to 3 of "The exact texts asked for"; `cache-mode: none` after `timeout-minutes: 15`;
   the scratch YAML parses).
   - The hold: `save` commits locally, does not push, names the files and exits 1 when the commits to push change
     `Cargo.toml`, `Cargo.lock`, `package.json`, `package-lock.json` or `npm-shrinkwrap.json` (any folder) against the
     remote branch (main's merge-base for a branch not on the remote). Released by
     `.checkpoints/<slug>/dependency-check.md` in the commit to push: one line per file, `<sha256>  <path>` as
     `sha256sum` prints it (`deleted  <path>` for a removed file).
   - Proven on the scratch copies: `node --test .gate/kit-hold-ci/checkpoint.test.mjs` 30 pass; the same test file
     against today's script (`CHECKPOINT_SCRIPT=$PWD/.claude/scripts/checkpoint.mjs`): 27 pass, the 3 new fail; the
     kit's own 27 tests against the edited script: 27 pass.
5. The plan is written (`kit-hold-ci/plan.md`): risk tier 2; the evidence he asked for (the compare step failing on
   a planted change: a generated file committed with the executable bit, because a content change fails one line
   earlier; the clean tree passing; "Cache mode: none" in the job's log; stop if `cache-mode` does not parse); the
   reading of "nothing else in it" (the report, the DECISIONS entry and the docs pass for #58's merge ride along);
   change-set 2's plan drafted beside it into this notes folder.

6. The advisor agent on the plan (15:46 to 16:07 UTC by its clock; `kit-hold-ci/advisor-plan.md`): APPROVE on eight
   conditions; no Critical or High; three Mediums, all wording (the five names are a reading; a manifest edit with no
   dependency change still takes a security answer; the stop rule covers every new CI line). It replayed today's
   miss in a scratch clone: the edited script held it, today's script pushed it. All taken into `plan.md` (its last
   section). Folded into the kit text beyond its conditions: a link or submodule named like a lockfile is held; the
   hold is computed for every save (so a lockfile moved into the notes folder is held, and the background saver's
   push is held too); a test that the record is read from the commit to push. The scratch copies were rebuilt:
   `node --test .gate/kit-hold-ci/checkpoint.test.mjs` 31 pass; against today's script 27 pass and the 4 new fail;
   the kit's own 27 tests against the edited script pass. `kit-edit.diff` regenerated (script +40 -1, tests +120 -1,
   rules +8); `ci-edit.diff` unchanged (+13). Size 181 lines against 150 planned (stop line 300).

## Current step
7. (16:11 UTC) The advisor agent gives one line on the revised kit diff (resumed by message). Then the message to
   Mustafa: the kit edit's exact text (the script and the rules bullet in full; the tests summarised, with the diff
   file named), how the hold is released, what it does not cover, the five names as a reading, the known costs, the
   evidence run, why the CI plant is a mode change, the stop rule, how "nothing else in it" is read, his steps (his
   yes; the chat in Accept edits; the guard's prompts). **Stop there for his yes.** Before it the main session does
   not: copy the scratch files into `.claude/` or `.github/`; run the scratch script's `save` or `watch` here; move
   the saver off main's script; push the plant commit; write the CI edits.
   After his yes: write the four files (three kit, one workflow) so that `git diff` equals the two diff files byte
   for byte; run the tests; commit by path; push; read the clean run (stop rule); the plant commit (a generated
   file with the executable bit), its run must end at the compare step, then the take-back commit; reviews
   (security-auditor, infra-engineer, test-engineer, one batch); the verifier; DECISIONS by script; docs-keeper;
   the report (with the advisor agent's list); the advisor agent on the final head; the pull request; its ready
   report goes to the Advisor before his go. Change-set 2's plan draft (engine-architect) may start once the edits
   are written (it counts toward the three agents; no manifest-named file in the notes).

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
- saver: background task bzuzw7uzy, started 15:41 UTC on 2026-11-18 (2-hour limit).
- agents (1 of 3): the advisor agent, resumed at 16:11 UTC for one line on the revised kit diff.
