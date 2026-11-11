# Foundation 2.1a: the legacy gate and tools read both layouts (PR #50, replacing #47)

_The report is written at the change-set's last step; Appendix A is final._

## Appendix A. Mustafa's words for this change-set, in full

_Each block below is copied byte for byte from its source by `report-appendix.py` (in the notes folder, readable at this branch's last checkpoint commit), never retyped; the same script checks every block against its source. Pointers in `docs/DECISIONS.md` lead here, because the notes folder is deleted before the PR is marked ready._

### A1.1. His decision of 2026-11-10 (the foundation chat, session_019YNMUVo8MVM3gdiMNaN5xs; quoted in his prompt there as "Mustafa's decision of 2026-11-10, word for word"; saved by that chat in `mustafa-2026-11-10.md`, not re-checked against its chat record here)

```text
I now have a second account, so the reason for 'one heavy spike at a time' (usage cuts on one account) no longer holds for it. The second account may plan and build foundation steps 2.1 and 2.2 now. Steps 2.3 to 2.6 keep their order and wait for their results. On this account Fable 5.2 may advise through the advisor tool, which overrides CLAUDE.md's model rule here until that rule is updated; the main model and every agent stay Opus 5.5. One account per branch.
```

### A1.2. His follow-up in the same chat, 2026-11-10, after its first turn stopped on the model rule (same source)

```text
The main model is now Opus 5.5, as my decision says. Carry on with my prompt above from its first step. What you already read stays read; nothing else changes.
```

### A1.3. The task as he gave it there, his numbered list (same source)

```text
This session plans only. No code and no file moves.
1. Plan foundation step 2.1, the restructure (the five legacy files into legacy/), as its own change-set. List every file that names a legacy path: tools/, the .claude/ hooks and rules, .github/, CLAUDE.md, the docs. Mark which are kit or gate files, because each of those edits asks Mustafa.
2. Sketch step 2.2, the toolchain, on one page: what it can pin now, and what waits for the boundary spike (name the open questions).
3. Name what in steps 2.3 to 2.6 must wait, and for which result.
4. Say when 2.1 can build without colliding with PR #46 (it edits the guard), and what PR #40 must do after 2.1 merges.
```

### A2.1. His answers of 2026-11-10 in the foundation chat, after the plan's report (plan revision 3 at d389d91): FREEZE-FDN, SPLIT, SCOPE, the build go, the merge order, both accounts (saved by that chat in `mustafa-2026-11-10-answers.md`, not re-checked against its chat record here)

```text
My answers of 2026-11-10, for the foundation chat.

FREEZE-FDN: (a). KIT-FREEZE-2 stops new rules, new guard checks and kit redesign. It does not stop the smallest edits the foundation needs to keep the kit true after its own moves. Each such edit still asks me and merges on my go.

SPLIT: (a). Two pull requests under the one plan: 2.1a, then 2.1b.

SCOPE: (a). I accept ADR 0002's part 1 alone: the repository after the restructure, with the legacy hard constraints applying to legacy/ only once the files have moved. Record it in the ADR's Status line in 2.1b's docs pass, on my go. Nothing else in ADR 0002 is accepted by this.

Build 2.1a now: the dry run, the gate edits (each asks me) and the mutation proof. Do it in this chat if CLAUDE.md's rules allow it. If they call for a fresh session, give me its prompt in one paragraph instead. The main model stays Opus 5.5.

Merge order, as your plan says: #46, then STATUS-DIET and LANE-RULES, then 2.1a, then 2.1b, each on my go. Keep 2.1a's pull request a draft until its turn.

Both Claude accounts are mine, and work from either one counts the same. The kit chat is writing that rule into CLAUDE.md; do not wait for it.

When 2.1a is built, end with: the pull request's number, what the proof showed, and anything you need from me.
```

### A2.2. His follow-up of 2026-11-10 there, after 2.1a's report (PR #47): the two gate prompts approved, #47 not watched, step 2.2 planned in a fresh session (same source)

```text
1. The two gate prompts this morning (tools/gate.mjs and tools/smoke.mjs): yes, I approved both.

2. Do not watch PR #47 (foundation step 2.1a). It is a draft waiting for its turn, and I will tell you when that turn comes.

3. Yes, plan step 2.2 (the toolchain) now, in a fresh session on this account. Give me its prompt in one paragraph. Plan only, no build: what it pins now, what stays an empty slot until the boundary spike, the CI job that runs the Rust checks, and its own plan review by the advisor agent. The main model stays Opus 5.5.
```

### A2.3. His message of 2026-11-11 there, 09:46:31 UTC (#47's turn; byte-identical to the chat record, event c49a5660-c1d0-4c58-a54c-983fe54c4394, 848 characters, checked on 2026-11-11 by this change-set)

```text
PR #49 is merged. STATUS is split: docs/STATUS.md is a dashboard of at most 120 lines, my decisions are in docs/DECISIONS.md word for word, the rest is in docs/STATUS_ARCHIVE.md. CLAUDE.md has a new "Lanes and accounts" section and the docs-keeper agent follows the split.

It is #47's turn (order: #49, then #47, then 2.1b, then #48).
1. Run /context. If this chat is over about half full, save a checkpoint, give me a resume card and stop for a fresh chat.
2. Update your branch from main and re-read CLAUDE.md and docs/STATUS.md.
3. Do your docs pass in the new layout: your row in STATUS's "In flight" table, my words in DECISIONS.md, the rest in the archive. Move text by script, never by retyping.
4. Finish as planned: the review round, the report, checkpoints folded in and deleted, the PR marked ready for the advisor. Then stop for my go.
```

### A3.1. His prompt in this session (session_019YNMUWLffMoE47SbvkqJ5P), 2026-11-11 10:01:35 UTC (first sent at 09:59:00 UTC under Fable 5.2, the same text): the foundation chat's resume card as he pasted it, plus his two additions at the end, "my words of 2026-11-11" (extracted by script from the session's events: `mustafa-2026-11-11.md`)

```text
You are the foundation chat for Shamble Frontlines on Mustafa's personal account, resuming foundation step 2.1a, draft PR #47 (branch `claude/advisor-fable-4hxq72`), at its turn. His words: "the main model and every agent stay Opus 5.5"; Fable 5.2 may advise only through the advisor tool, as "an extra". Check this session's model with get_session first, and stop in one line if it is not Opus 5.5. If this session's own branch is not `claude/advisor-fable-4hxq72`, you cannot push to #47: stop and ask him whether to restart on that branch, or to bring #47's commits into this branch and open a replacement PR with #47 closed and pointing to it. Read CLAUDE.md (with its new "Lanes and accounts" section), `.claude/CHECKPOINTS.md`, then the checkpoint `.checkpoints/foundation-restructure.md`, and follow its "Next steps" 1-4 as written. They carry his steps of 2026-11-11:

1. bring main (bb2d817) in;
2. do the docs pass in the new layout (#47's row in STATUS's "In flight", his words in `docs/DECISIONS.md`, the rest in `docs/STATUS_ARCHIVE.md`), text moved by script, never retyped;
3. the review round (security-auditor, test-engineer, infra-engineer), then the verifier;
4. the report, with the checkpoint folded in and deleted in the last commit, and #47 marked ready.

Then stop for his go. His words are in `.checkpoints/foundation-restructure/mustafa-2026-11-10*.md`, the plan in `plan.md`, and the evidence in `e3-summary.md` (with rebuild commands), `e3-bases.md`, `e3-results.json` and `rows-test.mjs`. Never redo finished work: resume from "Next steps" (CHECKPOINTS §8). Kit and gate edits ask him; if Auto mode's checker refuses an edit inside `.claude/`, stop and tell him. At most 3 agents at a time, each on Opus with its effort stated, the saver running while they work, a checkpoint saved after every step. Do not watch #47. End with: #47's state and head SHA, the review results, the report's path, and what you need from him (Decisions needed, then For Mustafa).

Two additions, my words of 2026-11-11:
- Before any edit to a kit or gate file, ask me in chat with the exact change and write it only after my yes. Do not rely on the guard's prompt: in Auto mode it can be answered without me seeing it (the toolchain chat recorded this as a finding for foundation step 2.3).
- If the branch check fails, my lean is the replacement pull request. Still stop and ask me first.
```

### A3.2. His message in this session, 2026-11-11 11:56:44 UTC: BRANCH-47 (a) and his words on the advisor tool (same source)

```text
BRANCH-47: (a). The replacement pull request from this session, as you describe it.
- My yes covers exactly two things. One: the single fast-forward of the main folder to your pushed merge, as a one-time exception to the Never rule, because the merge's .claude/, CLAUDE.md and .github/ are byte-identical to main's. Two: the one gate-file write the merge brings (dd12aeb's change to tools/gate.mjs and tools/smoke.mjs, nothing new added).
- Before the fast-forward, check once more that those three are still identical to main's. If they are not, or if the guard refuses the fast-forward, do not work around it: switch to "worktree only" and tell me.
- Record the exception in one line in the checkpoint and in the report. Any later kit or gate edit gets its own ask in chat.
- Say in the new pull request that it replaces #47, and close #47 with a comment pointing to it.
Then continue with steps 2 to 4.

My words of 2026-11-11 on the advisor tool. They sharpen "it is an extra"; nothing else changes.
- "Extra" means on top of the recorded plan review by the advisor agent, never instead of it. It does not mean optional.
- When the advisor tool is on in this session, consult it every time at these moments:
  1. before you commit to a plan or an approach;
  2. when an error or a failed approach comes back a second time;
  3. before any change to rules, the gate, CI or security-relevant code;
  4. on a design call with more than one defensible answer;
  5. before you declare a step or the task done.
- Usage is not a reason to skip a consultation. Quality comes first.
- Its advice cannot be quoted into the repo, so record each consultation in the checkpoint in one line: when, your question, and what you changed because of it. Count them in your report.
- If the advisor tool is not on in this session, say so in one line and carry on; I will decide.
- The main model and every agent stay Opus 5.5.
- Record these words in docs/DECISIONS.md with your next docs pass, unless they are already there. CLAUDE.md keeps its wording until the kit unfreezes at the gate step (2.3); this clarification goes into it then.
```

### A3.3. His message in this session, 2026-11-11 14:54:11 UTC: F1-FIX (a), FREEZE-FDN (a) for one check, and his yes to the exact change to `tools/gate.mjs` (extracted by script from the session's events, event 8e7ad9dc-cdf8-43a7-817b-ceaaba67081e: `mustafa-2026-11-11-f1.md`)

```text
My words of 2026-11-11, for the foundation chat (PR #50, foundation step 2.1a).

F1-FIX: (a). Close the hole in #50 now, with this exact change, then re-run the proofs.

FREEZE-FDN: (a). Inside my exception. The check exists only so that the foundation's own move (2.1b) and its revert are judged safely, and it asked me first. This reading covers this one check. Any other new check still asks me.

Yes to the exact change to tools/gate.mjs, as in .checkpoints/foundation-restructure/f1-gate.patch at eaf5a4580e31920073d7270b4baafbbe63da3c5a.
- My yes covers exactly one thing: writing this change into tools/gate.mjs on #50's branch. It does not cover any other kit or gate edit, and it does not cover the merge.
- After the write, tools/gate.mjs must be byte-identical to what that patch file produces from the current file (git blob 7a8480c, as the patch's index line names). If it is not, or if the guard refuses the write, do not work around it: stop and tell me.
- tools/smoke.mjs stays unchanged. Its wording Low becomes a follow-up for 2.3.

Then continue as you listed: the full mutation proof on the new gate, both directions of the move test with raw output and exit codes, the decoy on main's gate and on the new one, round 2 on the change only, the verifier, the report. Then mark #50 ready.
- Round 2 is the last round, as the review budget says. A third only for an open Critical or High.
- The security-auditor asked for one run of the new gate on Windows before 2.1b. In the report, give me that step with the exact command.
- In the report, say how full this chat is, so I can decide where 2.1b runs.
- Record these words in docs/DECISIONS.md with this change-set's docs pass.
```

### A3.4. His message in this session, 2026-11-11 16:32:18 UTC: his words on permission prompts (extracted by script from the session's events, event 557d0202-6418-4c26-9d03-9b1ee13fd5bf: `mustafa-2026-11-11-prompts.md`)

```text
My words of 2026-11-11 on permission prompts, for the foundation chat (PR #50, foundation step 2.1a).
- I am switching this chat to Auto mode, so routine commands stop asking me. Nothing else changes.
- Any edit to a kit or gate file still asks me in chat first, with the exact change, and is written only after my yes. My yes of today covered one write, and that write is done.
- A prompt from the guard counts as not seen by me until I have answered the same question in chat.
- Do not edit any settings file, hook or allow-list to get fewer prompts.
- Where you can, keep shell lines simple enough for the guard to read, and put multi-step work in a script under .gate/ that you run with node.
- In the report, add two things: which kinds of command asked me most today, and every refusal by the guard, the deny list or the classifier with the route you took instead. Both are findings for 2.3.
Carry on from where you are. The mutation proof keeps running.
```

### A3.5. His message in this session, 2026-11-11 16:54:54 UTC: a correction to his words on permission prompts (A3.4) (extracted by script from the session's events, event a10ee1d6-3120-4eb1-b83a-4e84afe50a94: `mustafa-2026-11-11-prompts-correction.md`)

```text
My words of 2026-11-11, a correction to my words on permission prompts of 16:32 UTC, for the foundation chat (PR #50, foundation step 2.1a).
- Your tally shows this chat was already in Auto mode. Nothing was switched. Read that line as "this chat stays in Auto mode".
- The line "A prompt from the guard counts as not seen by me until I have answered the same question in chat" covers two things only: an edit to a kit or gate file, and a move of the main folder.
- For every other question from the guard, my answer on the prompt is my answer. Do not ask me in chat as well. Examples: a worktree under .gate/, a switch inside such a worktree, a shell line the guard cannot read that touches no kit or gate file.
- In round 2's and the verifier's briefs, tell the agents which forms the guard refused today, so they write with the file tools or with one plain command and a literal path. An agent cannot be asked, so those writes are refused.
- The tally is enough as it is. No further agent for it.
- In the ready report, give me CI's gate and smoke results for the final head, with the run numbers. If smoke is not green on the final head, say why and leave #50 a draft.
- Record these words in docs/DECISIONS.md next to the words they correct.
```

### A4. His go for #49, 2026-11-11, about 09:02 UTC, as the merge commit bb2d817's message records it (quoted there with "> "; the prefix is removed here). Not checked against the chat record: that day session (session_0173vVo6Vtm2C54toWvTt8sR) is not readable from this account

```text
1. Kit-edit prompts: whether or not each of the three prompts reached me, I approve those three edits now: the CLAUDE.md edit (read list, look line, lanes and accounts, the advisor-tool lines) and the two docs-keeper.md edits for the STATUS split. My Advisor read their diffs. Do not undo them, and push nothing to PR #49 before the merge.
2. READINGS: (a). I accept both readings.
3. merge #49
Pinned to 2e6e5d2af8424aa971d858a2aab0387c073287d2, by merge commit like #46, after you re-check that the head and CI's gate and smoke have not changed. My Advisor's merge check passed on that head.
After the merge: stop. The next change-set records it.
```
