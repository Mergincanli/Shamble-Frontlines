# Plan: kit-hold-ci (the kit hold and the two CI edits, one small pull request)

Owner: the main session (the first builder chat, lane 1). Branch `claude/wonderful-hamilton-jdmlnf` (stands in for
`cs/kit-hold-ci`), base main 9b4b518 (the merge of #58). **Risk tier 2** (the kit and CI): reviews by the advisor agent
and security-auditor; it merges on Mustafa's go from a day session. No Brave check: nothing a player sees.

## His words that set it (2026-11-18, 15:38:13 UTC; `mustafa-2026-11-18-1624-merge-58.md`, by script)
- "One small pull request with the kit hold and the two CI edits, nothing else in it. Its ready report goes to the
  Advisor before my go."
- DEP-HOLD: "(a). The checkpoint script holds a push that changes a manifest or a lockfile until the security check of
  that change has answered. This is my exception to the kit freeze, for this one change. Show me its exact text first,
  with how the hold is released. Keep it to that: no guard change and no longer watch list now. The check in CI that
  option (c) adds is not decided here: put it on the list for the quality gate v2's plan."
- CI-TYPES: "(a), yes, with the exact text as it stands in the report at this head, in the section "The exact texts
  asked for". Its pull request shows two things from a real run: the compare step failing once on a planted change,
  and passing on the clean tree."
- CI-CUT: "(a). Part 1 now, with the exact text of that same section; part 2 waits for the quality gate v2's workflow.
  Its pull request shows "Cache mode: none" in the job's log."

## The three edits (no other code, kit, gate or CI change)
1. **The kit hold** (his exception to the freeze): `.claude/scripts/checkpoint.mjs`, its tests
   `.claude/scripts/checkpoint.test.mjs`, one bullet in `.claude/CHECKPOINTS.md` section 3. Exact text:
   `kit-edit.diff` beside this file (made from scratch copies under `.gate/kit-hold-ci/`; no kit file is touched yet).
   - What it does: `save` (the foreground form; the background saver already never pushes code) commits the notes
     locally as before; then, when the commits it would push change a manifest or a lockfile (`Cargo.toml`,
     `Cargo.lock`, `package.json`, `package-lock.json`, `npm-shrinkwrap.json`, in any folder: the list of the report's
     option (a), no longer), it does not push, names the files, and exits 1.
   - "Would push" is measured tree to tree against the remote branch, as the script's skip-mark rule already does;
     against main's merge-base while the branch is not on the remote yet. What cannot be read is held.
   - **How the hold is released:** by a record in the notes, `.checkpoints/<slug>/dependency-check.md`, in the commit
     to push: one line per held file, `<sha256>  <path>` as `sha256sum` prints it (`deleted  <path>` for a removed
     file). The session writes it after security-auditor's answer on that change, with the answer's notes named in
     the same file; then `save` again pushes everything. The record releases exactly that content: when the file
     changes again, the hold is back.
   - What it does not do (his "Keep it to that"): no guard change, so a plain `git push` is not held (the rules' new
     bullet says so); no longer watch list; no CI check.
   - A known cost, to tell him: a manifest edit that changes no dependency (a description line) is held too; its
     record then simply says so.
2. **CI-TYPES** and 3. **CI-CUT part 1**: `.github/workflows/toolchain.yml`. Exact text: `ci-edit.diff` beside this
   file, built by script from the report at head 1bfe462 (its section "The exact texts asked for", blocks 1 to 3) and
   equal to it byte for byte. He has said yes to both texts.

## How it is asked and written (his rules)
- The kit edit's text is shown to him in chat first, with how the hold is released; it is written only after his
  yes. The two CI texts have his yes already.
- Every edit to `.claude/` and `.github/` raises the guard's prompt; his words of 2026-11-13 ask for the chat in
  Accept edits when such an edit is written. So the three edits are written in one go, when he has said yes to the
  kit text and the chat is in Accept edits. Until then only notes and scratch copies exist.

## Evidence
- The kit hold (logic): the script's test file, `node --test .claude/scripts/checkpoint.test.mjs`: 30 tests (27
  existing, 3 new). Bite: the three new tests fail against today's script and pass against the edited one; the 27
  existing tests pass against the edited one (run already on the scratch copies: 27 pass and 3 fail; 30 pass).
  Planted in the review: the hold removed from the script, the release check accepting any line, the watch list
  emptied: the new tests must fail each time.
- CI-TYPES, from real runs on the pull request's branch:
  - **the compare step failing once on a planted change.** A content change of a generated file cannot reach the
    compare step: the plain `cargo test --workspace --locked` one line above it fails first. The planted change that
    does reach it: one generated file committed with the executable bit (`git update-index --chmod=+x`). The plain
    run compares bytes and passes; write mode deletes and rewrites the file, git then reports a mode change, and the
    compare step fails with its message. One commit plants it, the next commit takes it back (a new commit, no
    history rewritten); both runs are named in the report with the failing step's log lines.
  - **passing on the clean tree:** the run on the head.
  - The type check lines: the same head's run; a second planted commit is not asked and not made.
- CI-CUT part 1: the line "Cache mode: none" in the job's log of the head's run (read with the GitHub tools). **If
  the workflow does not parse with `cache-mode`, or the log does not show that line: stop and tell him in three
  lines; no other text is tried without his word** (the text was checked against the documentation only).
- The gate (`node tools/gate.mjs --no-smoke`), the repository map check, `gate`, `smoke` and `toolchain` green on the
  head.

## Reviews (risk tier 2)
The advisor agent on this plan and on every pushed head; security-auditor (the hold's logic and its release, the two
CI edits, what the hold does not cover); infra-engineer (the workflow edit: the cut script, git inside the cut, the
cache key); test-engineer (do the new tests and the planted CI run prove their claims). All Opus 5.5 at effort max, at
most 3 at a time, within two rounds. Then the verifier.

## The docs that go with it (the main session's reading of "nothing else in it", for the advisor agent to rule on)
"Nothing else" is read as: no other code, kit, gate or CI change, and no work of change-set 2. As with #56, #57 and
#58, the pull request carries its own report and the records that are due: his message of 15:38:13 UTC in
`docs/DECISIONS.md` by script (his go on #58, his five answers); STATUS, ROADMAP, the archive and the map for the
merge of #58 and for this change (docs-keeper, under RECORD-FILES and OLD-LINES); "Cache mode: none" and the two
CI runs in the report; the gate v2's list gains the CI check of DEP-HOLD's option (c) and CI-CUT part 2. If he wants
the records apart, they move to a docs change of their own.

## Change-set 2's plan, beside it
His words: "You may write change-set 2's plan beside it. No new library of change-set 2 is committed before that pull
request is merged." engine-architect drafts it (effort max) into this notes folder (`cs2-plan-draft.md`); nothing of it
is built, no manifest is touched. The notes are deleted with this checkpoint in the last commit and taken up from
that commit when change-set 2 starts.

## Size
The build plan's count is for game code; this change is kit and CI. Planned: about 40 lines in the script, 90 in its
tests, 6 in the rules, 13 in the workflow (150 in all). Twice that (300) is the stop line.

## What Mustafa must do
1. Read the kit edit's exact text and how the hold is released; say yes or what to change.
2. Put the chat in Accept edits for the three edits and answer the guard's prompts.
3. After the Advisor's check of the ready report: his go on the merge.
