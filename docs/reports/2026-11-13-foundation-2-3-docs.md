# Advisor report: foundation-2-3-docs, step 2.3's first docs pass (2026-11-13; PR #52; branch claude/sweet-mayer-grsukw for cs/foundation-2-3-docs; base main 4d21c1a)

_The notes (the plan, the scripts, the agents' notes, the evidence files) are deleted before the pull request is marked
ready; the last commit that holds them is {{NOTES-SHA}}._

## Goal
Mustafa's first item for step 2.3: "First: the docs pass. Its own small pull request, docs only, with no kit, gate or CI file in it." (A2) His words for PR #48 and his decision of 2026-11-13 go into `docs/DECISIONS.md` by script, with dated pointers at the older lines the decision overtakes; STATUS and ROADMAP become true for main 4d21c1a; the 2.2 report gets its five fixes. Nothing a player sees, hears or feels changes.

## Outcome
Built and checked in four commits (1123175, 2f96f5c, 83c6ab6 and step 5's), every script check and the gate passing,
CI green on each. Review round 1, the verifier and the advisor agent's review of the final head follow; their results
are recorded below as they come, and the last in the pull request's body.

## Plan and approval
- The plan: `.checkpoints/foundation-2-3-docs/plan.md`, revisions 1, 2 and 2a.
- The advisor agent: APPROVE of revision 1 (ec16b30) with ten Low conditions, C1 to C10, folded in revision 2 ("If
  revision 2 only folds in C1 to C10, it needs no new plan review"). Its answers: M1 as a dated correction (Q1 (a)); the
  decision alone, then the whole message (Q2 (a)); the archive's two copies of the same claims fixed too (Q3); two
  reviewers plus the verifier (Q4); one pointing clause each in two archive follow-ups (Q5); his go for #48 checked
  against the chat record (Q6); #40's row read without switching (Q7).
- Revision 2a, its ruling before any tracked edit: L2 worded as seven job logs read; the header note's ending; its
  conditions: (a) the Appendix A hash check stays, (b) `fixes.mjs --check` re-runs on the final head.
- Revision 2b, its ruling on step 4's departures (STATUS within 120 lines once his words were filled in): all
  accepted, "Last merged" with its archive pointer restored at no line cost, E-TASK's header saying STATUS quotes the
  decision, and one High it found in that header (H-1, below), fixed in step 5.
- The conditions as met: C1 #40's facts read at the build and on the final head; C2 the pointers' quotes joined and
  tested, with a negative control; C3 and C4 the wording; C5 the site table; C6 six hunks; C7 the skeptics after the
  reviewers, never more than 3 agents; C8 both saved pages identical; C9 the advisor tool before each step was called
  done (4 at step 5's commit consultations); C10 this header names the notes' last commit, and the checkpoint is folded in here.

## What changed
- `docs/DECISIONS.md`: three entries at the top: his decision of 2026-11-13 (alone, then his whole task message), his
  message before it, and his go for #48. One sentence in his 2026-11-10 entry's header (two lines reflowed, its other
  words unchanged), and the pointer P5 at SLICE-ORDER's condition 3. Nothing leaves "Queued" and nothing is added.
  H-1 (High, the advisor agent, fixed in step 5): E-TASK's header said the kit lines are not edited "by his words they
  are the rule there too", which reads as if the old "2.1 and 2.2" lines stayed the rule; it now says "his words are the
  rule there too".
- Dated pointers quoting his decision's first four sentences, "for foundation step 2.3 only": `docs/SPIKE_PLAN.md`
  (P1 to P3), ADR 0002's migration step 1 (P4), `docs/ROADMAP.md` (P7); in `docs/STATUS.md` (P8) the same four
  sentences, quoted after STATUS's own lead-in.
- `docs/reports/2026-11-12-foundation-2-2.md`: M1, L1, L2 (worded as the logs read), L3 and a dated header note.
- `docs/STATUS_ARCHIVE.md`: L5; Q3's two lines; Q5's two clauses; #48's row in "In flight"; the 2.2 block's opening
  ("merged as 4d21c1a"); three lines that still called his go for Q2.2-a and OVERFLOW-ALL pending.
- `docs/STATUS.md` (120 lines) and `docs/ROADMAP.md`: true for main 4d21c1a (docs-keeper, then the session's checks);
  `docs/REPO_MAP.md` unchanged (it lists four reports; its `docs/reports/` row covers this one).
- This report. No code: the gate's symbol check is +0 −0.

## Evidence
| Id | What | Result |
|---|---|---|
| E1 | his words | `words.mjs`: A1 (42 characters) and A2 (6,190 characters, 36 lines) from this session's transcript; A3 (1,872 characters) from 4d21c1a's message, the round trip OK and identical byte for byte to foundation 2.2's chat record (message 96467900-…, its event stamped 16:16:36 UTC; the commit says 16:16:47) and to that session's merge call; the decision (562 characters) cut by its anchors; the transcript scan finds no other message of his. `report-appendix.mjs`: the blocks and the body's quotes OK |
| E2 | DECISIONS.md | `decisions.mjs --check`: OK; six negative controls caught (each quote, the header sentence, the entry title); the only lines removed against 4d21c1a are the 2026-11-10 header's two reflowed lines; 2.2's `decisions-entries.mjs --check`: OK, 13 entries |
| E3 | the pointers | `pointers.mjs --check`: P1 to P5, P7, P8 and his kit-freeze words in STATUS; each quote joined and found in his decision; a dropped word caught; SPIKE_PLAN +18 −0 lines, ADR 0002 +6 −0 |
| E4 | the fixes | `fixes.mjs --check --hunks`: the 2.2 report's Appendix A sha256 equal to 4d21c1a's; 2.2's body-quote check: 13 blocks, 10 quotes; the whole-file checks; 6 hunks in the report, 5 in the archive (before docs-keeper); L2: seven job logs read; L3: the saved run's flag list; L1: the 2.2 checkpoint's F1 |
| E5 | scope | `scope.mjs`, main's guard (`protectedRel`, `mergeAllowed` from origin/main 4d21c1a): every path under `docs/`, none protected, file modes plain; DECISIONS.md and ADR 0002 not mergeAllowed, so the PR Advisor cannot merge it |
| E6 | gate | `node tools/gate.mjs --no-smoke`: GATE PASS after each step (the smoke in CI: this container's Chromium is not the toolkit's); repo map OK |
| E7 | CI | Green on each step's commit (gate, with its gate and smoke jobs, and toolchain; push, then pull_request): 1123175 (43497813351, 43497824049; 43497813242, 43497823762); 2f96f5c (43501529364, 43501539671; 43501529328, 43501539371); 83c6ab6 (43504752852, 43504760971; 43504752955, 43504761167); the final head's runs: in the pull request's body at ready |
| E8 | facts | `quotes.mjs` on STATUS, ROADMAP, the archive and REPO_MAP: 15 quoted spans on the added lines, all found in his words or the base (one split line judged by hand); R-STATE and the verifier on every SHA, run id and time |

## Reviews
Pending at step 5's commit: review round 1 (R-WORDS on his words and decisions, R-STATE on the state and the fixes;
Opus 5.5 at effort max, each Medium or higher finding then tested by a skeptic), the verifier, and the advisor agent's
review of the final head.

## The advisor tool: consultations (his words of 2026-11-11)
In this conversation Opus 5.5 answers the advisor tool (the `/advisor fable` output: until `/clear` or `/compact`).
1. 2026-11-13 ~20:00 Amsterdam, moment 1 (the approach): saved first; every pointer quotes all four limits; the grep
   widened; the decision cut by anchors; 2.2's appendix check replaced by a hash check; KIT-FREEZE-2 quoted, not
   paraphrased; Q1 to Q4 put to the advisor agent; the final head approved as the commit that deletes the checkpoint.
2. ~20:57, moment 5 for step 1 and moment 1 for step 2: the entry title from one source; `--check` honours `--only`;
   the transcript scan; DECISIONS.md's diff proven; the saver recorded; the gate before the commit; a plain push.
3. ~21:15, moment 5 for step 2 and moment 1 for step 3: the M5 and pull_request runs read (L2 worded as read, revision
   2a); the header note's "every other word" with a whole-file check; the hunk counts recorded; placeholders and a
   quote-span check for step 4.
4. ~21:50, moment 5 for step 4 and moment 1 for step 5: step 4 not done yet (the REPO_MAP claim; the #48 row read whole;
   the archive's word-level diff; STATUS's lines checked by diff; the departures ruled as revision 2b); the transcript
   re-scanned; CI read by SHA; three disclosures; `review-r1.js` hardened.

## Decisions and deviations
- L2 is worded as the logs read: on 2026-11-12 the runners differed, and c901187's pull_request run had 2.56.1 at 15:34
  UTC. That refines the line in PR #48's description, on the advisor agent's ruling.
- Step 4's departures, ruled as revision 2b: STATUS's kit line is pointers (FREEZE-FDN's quotes and his exceptions
  stand in DECISIONS.md, "Queued") with his kit-freeze words of 2026-11-13 quoted by script; P8 is his four sentences
  quoted after STATUS's lead-in; the old-sections table's two "same names" rows are one; three archive lines beyond
  plan §6; no REPO_MAP row (the map lists four reports).
- The header note says "every other word" unchanged, not "every other line" (the touched paragraphs reflow), backed by
  a whole-file check, on the same ruling.
- Beyond the five: the archive's copies of the same two claims, and a pointing clause in each of two follow-ups (Q3,
  Q5).
- `tools/` was not installed in this container: `npm ci --prefix tools` (CLAUDE.md's setup step; gitignored).
- Commands refused by the guard, the deny list or the classifier: none so far (no command was refused).
- docs-keeper's scratch file `fillcount.mjs` held a retyped copy of his decision's four sentences and his kit-freeze
  words, to count lines; the saver pushed it (e6fc6a9) before it was deleted. The copy is byte-identical to the script
  cut (checked), it stands in no doc, and history keeps it.
- Two of the seven L2 logs (fd14855's push run, 45f1bb5's pull_request run) were read by the session only; the advisor
  agent read the other five.
- The transcript-based checks (`words.mjs`'s A1, A2 and its scan; the chat-record pages) run only in this session's
  container: elsewhere the sessions API re-reads them.

## Risks
- The chat record's copies of his #48 go are API pages saved outside the repository; the sessions API re-reads them.
- STATUS's Routines section dates from 2026-11-10: this account cannot read the university account's routines.

## Findings for step 2.3's plan
- The kit lines that still name steps 2.1 and 2.2 as the only exception (CLAUDE.md :22 and :55, `.claude/agents/advisor.md`
  :108): his decision is the rule there; step 2.3 asks their edits.
- BRIEF-STALE and the five "before the foundation" lines now lack the 2026-11-13 decision too (the archive's clauses).
- This container had no `tools/` install; the local smoke still cannot start (the toolkit's Playwright wants another
  Chromium build).

## Proposed next step
Step 2.3's plan, after #52 merges, in his order.

## Decisions needed
None.

## For Mustafa
1. Read this report and Appendix A (your words, copied by script).
2. When the advisor agent has approved the final head and gate, smoke and toolchain are green on it, say "merge #52" in
   this chat. No Brave check: nothing a player sees, hears or feels changes.

## Appendix A. Mustafa's words for this change-set, in full

_Each block below is copied byte for byte from its source by `report-appendix.mjs` (in the notes folder, readable at this branch's last checkpoint commit, named in the header above), never retyped. A1 and A2 are his messages in this session (session_019YNMUWSsyRgaYWxk4auQ5b, his personal account), each read from the session's transcript by `words.mjs`: a message's single text. A3 is his go for #48, read from merge commit 4d21c1a's message and checked byte for byte against foundation 2.2's chat record. His decision of 2026-11-13 is the text inside its quote marks in A2, cut by `words.mjs` at its first sentence ("Step 2.3 may be planned now.") and its last words ("only exception."), since it holds quote marks of its own. The same script checks every block against its source, and every quote of his words in this report's body against the item it cites. His other input in this session was a command: `/advisor fable` (2026-11-13 18:30:08 UTC), whose output said this conversation keeps Opus 5.5 as its advisor model until `/clear` or `/compact`. The session's other entries are the harness's and its agents': the command's caveat, the workflow-authoring skill (loaded with A2, 18:47:56 UTC), a notice that the session's worker had restarted (19:22:24 UTC), the advisor-report skill's text (loaded by the session), stop-hook feedback, and the hand-backs and task notifications of the session's own agents; `words.mjs` classifies every entry of the transcript and finds no other message of his. Entries in `docs/DECISIONS.md` copy their quotes from here, because the notes folder is deleted before the pull request is marked ready._

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
