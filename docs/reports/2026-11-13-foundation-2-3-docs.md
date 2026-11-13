# Advisor report: foundation-2-3-docs, step 2.3's first docs pass (2026-11-13; PR #52; branch claude/sweet-mayer-grsukw for cs/foundation-2-3-docs; base main 4d21c1a)

_Draft. Appendix A is written first, by script, so that `docs/DECISIONS.md` can copy Mustafa's words from it; the body
follows in step 5 of the plan (`.checkpoints/foundation-2-3-docs/plan.md`, revision 2)._

## Appendix A. Mustafa's words for this change-set, in full

_Each block below is copied byte for byte from its source by `report-appendix.mjs` (in the notes folder, readable at this branch's last checkpoint commit, named in the header above), never retyped. A1 and A2 are his messages in this session (session_019YNMUWSsyRgaYWxk4auQ5b, his personal account), each read from the session's transcript by `words.mjs`: a message's single text. A3 is his go for #48, read from merge commit 4d21c1a's message and checked byte for byte against foundation 2.2's chat record. His decision of 2026-11-13 is the text inside its quote marks in A2, cut by `words.mjs` at its first sentence ("Step 2.3 may be planned now.") and its last words ("only exception."), since it holds quote marks of its own. The same script checks every block against its source, and every quote of his words in this report's body against the item it cites. His other input in this session was a command: `/advisor fable` (2026-11-13 18:30:08 UTC), whose output said this conversation keeps Opus 5.5 as its advisor model until `/clear` or `/compact`. The session's other entries are the harness's and its agents': the command's caveat, the workflow-authoring skill (loaded with A2, 18:47:56 UTC), a notice that the session's worker had restarted (19:22:24 UTC), and the hand-backs and task notifications of the session's own agents; `words.mjs` classifies every entry of the transcript and finds no other message of his. Entries in `docs/DECISIONS.md` copy their quotes from here, because the notes folder is deleted before the pull request is marked ready._

### A1. His message before the task, 2026-11-13 18:29:55 UTC (transcript message b031379c-ea4c-47db-b279-54e937bf147a)

```text
Hi, don't do anything yet, setting you up.
```

### A2. His task message: step 2.3's first docs pass and its plan, with his decision of 2026-11-13 on step 2.3, 2026-11-13 18:47:55 UTC (transcript message e5fa7ca0-6494-44d0-8366-81fc612c7958)

```text
My words of 2026-11-13, for a fresh chat on my personal account. Foundation step 2.3: the gate v2 and CI, and the end of the kit's freeze.

Where things stand. Step 2.2 is merged: main is 4d21c1a7a4d39613bffce5d5f9bbc92f2f88a300 (PR #48), with gate, smoke and toolchain green on it. The toolchain chat is finished. One other chat works on this repo: the netcode proof test (PR #40, branch claude/netcode-proof-test-plan-hcky5g, my university account). Never push to its branch and never touch spikes/.

Model. This step is gate, CI and rules work, so by my words of 2026-11-12 its main model is Opus 5.5. Check this session's model first and stop in one line if it is not Opus 5.5. Agents run on Opus 5.5; reviewers and the verifier may run at effort max, as in step 2.2. Fable 5.2 advises through the advisor tool. My words of 2026-11-11 on it apply: consult it at the five moments, record each consultation in the checkpoint in one line, and count them in the report. If the advisor tool is not on, say so in one line and carry on.

My decision of 2026-11-13 on step 2.3, to record word for word:
"Step 2.3 may be planned now. The parts of step 2.3 that need no proof test's result may be built before the proof tests report, after my go on the plan. Every gate check that needs a result joins later, when that result is in, with its pass marks unchanged. Steps 2.4 to 2.6 still wait for their results. This changes my words of 2026-11-10 ("Steps 2.3 to 2.6 keep their order and wait for their results") for step 2.3 only. These words are the rule from now, also where CLAUDE.md and .claude/agents/advisor.md still name steps 2.1 and 2.2 as the only exception."

Read first: CLAUDE.md, .claude/CHECKPOINTS.md, docs/STATUS.md (it is one merge behind: it still lists PR #48 as in flight), my words of 2026-11-10 to 2026-11-12 and "Queued" in docs/DECISIONS.md, the message of merge commit 4d21c1a (my words for PR #48), and docs/reports/2026-11-12-foundation-2-2.md. When the plan starts, also: docs/adr/0002-code-structure-build-toolchain.md (parts 7 to 9, the migration plan's step 2.3 and the pass marks PM-0002-39 to PM-0002-62), and part 3 of the old foundation plan at commit 26e0d82702a3b16627690bf545de08812f2bcd83, .checkpoints/foundation-restructure/plan.md.

First: the docs pass. Its own small pull request, docs only, with no kit, gate or CI file in it.
- My words for PR #48 go into docs/DECISIONS.md, captured by script from merge commit 4d21c1a's message, as step 2.2 did for PR #51.
- My decision above goes in word for word, with dated pointers at the older lines it overtakes, as foundation 2.1a did for my decision of 2026-11-10.
- docs/STATUS.md and docs/ROADMAP.md become true for main 4d21c1a.
- The five report fixes listed in PR #48's description: M1, L1, L2, L3 and L5.
Plan it, have the advisor agent review the plan, build it, review it within the budget, mark it ready and stop for my go.

Then, after the docs pass is merged: plan step 2.3. No build, and no edit outside your checkpoint. The plan gives me:
1. Check by check, G0 to G13: built now, or waiting, and for which result. Start from part 3 of the old foundation plan and check it against today's main.
2. For every done-when line and pass mark of step 2.3: met by the first part, or moved unchanged to a named later change-set. The split drops no mark and loosens no mark.
3. The cut into change-sets that each merge on their own, in order. Keep rule and guard edits apart from the gate v2's code where you can, so each pull request carries one kind of risk.
4. Every item that waits for this step, each marked: in step 2.3 (which change-set), later (when), or dropped (why). The lists: "Queued" in docs/DECISIONS.md; the "Findings for 2.3" of the 2.1a, 2.1b and 2.2 reports; STATUS's "Follow-ups" for the gate step; the three CI security items I accepted with PR #48 (SETPRIV-HARDEN, SETUP-NODE-CACHE, CUT-ESCAPE). Lean to the smallest step that makes the gate, CI and the rules true.
5. Every kit, gate and CI edit you foresee: the file, and the exact before and after text where you can already write it. Among them: my model rule of 2026-11-12 in CLAUDE.md and the 13 agent files; my words of 2026-11-11 on the advisor tool; the three lines that name steps 2.1 and 2.2 as the only exception (on main 4d21c1a: CLAUDE.md lines 22 and 55, .claude/agents/advisor.md line 108); the lines that name gate and smoke as the required checks.
6. For each guard change, three lines: what it changes, whether it makes the guard stricter or looser, and what could go wrong.
7. The gate v2's script path (OQ-0002-2's third part), as a decision with options and your recommendation.
8. What PR #40 must do after each of your merges, and when I change the required check in my branch protection.
9. What I do myself, in plain words: settings on GitHub, mode switches, anything on my Windows laptop.

Kit, gate and CI files. The kit's freeze (KIT-FREEZE-2) ends with this step. That makes no edit unasked: every edit to a kit, gate or CI file is asked of me in chat with its exact text and written only after my yes, with the chat in Accept edits. A guard prompt alone is not my yes. No such edit happens in the docs pass or while planning.

How to work. CLAUDE.md's change-set protocol. Your branch stands in for the cs/ branch; the docs pass and each later change-set are separate pull requests from it, one at a time. At most 3 agents, the saver running, a checkpoint after every step. If the guard, the deny list or the classifier refuses a command, record it with the route you took. If a rule in the repo forbids something here and my words above do not cover it, the repo wins: stop and tell me in three lines.

What this block covers: the docs pass up to ready, and the planning of step 2.3. It covers no kit, gate or CI edit, no build of step 2.3 and no merge.

End the docs pass with: the pull request's number and head SHA, the run ids of gate, smoke and toolchain on that head (toolchain is not a required check yet, so read its runs yourself), and what you need from me. End the plan with: its path, a one-screen summary, the table of checks, the decisions you need from me, and how full this chat is.
```

### A3. His go for #48, in foundation 2.2's session (session_019YNMUSc7FA68ujhLgXfEJA), 2026-11-13 16:16:47 UTC (the time and the session per merge commit 4d21c1a's message, which quotes it whole; extracted from that message, and re-quoting it gives the commit's block back byte for byte; checked byte for byte against that session's chat record, message 96467900-83d8-4b54-8f8b-7884a0dc0fde, read with the sessions API, where the event is stamped 2026-11-13 16:16:36 UTC)

```text
My words of 2026-11-13 for PR #48 (foundation step 2.2, the toolchain).

1. ADR-INDEX (a). Yes to the register row Q2.2-a, with this merge.

2. K1-GO (a). Yes to .github/workflows/toolchain.yml as it stands on head 9b20b67624d87af29e8b7063f383e4f9a4748aa7.

3. OVERFLOW-ALL (a). Yes: overflow checks stay on in release builds for all our own Rust crates, and the register row stays. My yes covers its three points: outside libraries are decided when the first one arrives, the ADR's overflow test moves into another of our crates that calls the simulation, and the rule that keeps player input from causing such a stop goes to the drafts' owners.

4. merge #48. This go covers one thing: merging PR #48 at head 9b20b67624d87af29e8b7063f383e4f9a4748aa7 into main by a merge commit, as #46, #49, #50 and #51 were, with these words quoted in the merge message. It covers no new commit on the branch and no start on step 2.3.

5. With this merge I accept four security items left open: OVERFLOW-PANIC-RULE (due with the first code in the room crate or a server crate), and SETPRIV-HARDEN, SETUP-NODE-CACHE and CUT-ESCAPE (each for step 2.3, as a CI edit that asks me). The report says one; four is right.

6. The report fixes listed in the PR description (M1, L1, L2, L3, L5) and the DECISIONS.md entry for these words go into step 2.3's first docs pass. Put both on step 2.3's list.

Before you merge, check that the PR's head is still that SHA, that main is still 2aa5e9ce03e9ab9ccfecd53e6aabf951abe705dd, and that all six CI job runs on the head are green: toolchain, gate and smoke, each for push and for pull request. If anything differs or a run is missing, do not merge; tell me what you found and stop.

After the merge, tell me main's new SHA, the ids of the six runs you read, and the state of CI on main, in a few lines. Then stop. This chat is finished after that.
```
