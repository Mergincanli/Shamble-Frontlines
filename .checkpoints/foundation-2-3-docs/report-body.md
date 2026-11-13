# Advisor report: foundation-2-3-docs, step 2.3's first docs pass (2026-11-13; PR #52; branch claude/sweet-mayer-grsukw for cs/foundation-2-3-docs; base main 4d21c1a)

_The notes (the plan, the scripts, the agents' notes, the evidence files) are deleted before the pull request is marked
ready; the last commit that holds them is {{NOTES-SHA}}._

## Goal
Mustafa's first item for step 2.3: "First: the docs pass. Its own small pull request, docs only, with no kit, gate or CI file in it." (A2) His words for PR #48 and his decision of 2026-11-13 go into `docs/DECISIONS.md` by script, with dated pointers at the older lines the decision overtakes; STATUS and ROADMAP become true for main 4d21c1a; the 2.2 report gets its five fixes. Nothing a player sees, hears or feels changes.

## Outcome
- Built in five commits (1123175, 2f96f5c, 83c6ab6, d59ca43, then review round 1's fix pass), every script check and
  the gate passing; CI green on the first four (E7), the fix pass's runs read after its push. Review round 1 (two
  reviewers, four skeptics) found one High and two Mediums, all fixed; round 2, the verifier and the advisor agent's
  review of the final head follow.
- **A gap against his rule on the advisor tool** (his words of 2026-11-11, moment 5: consult before a step or the task
  is declared done): steps 0 and 3 had no closing consultation of their own (step 3's was made late, consultation 6);
  steps 1, 2, 4 and 5 were consulted 6 seconds to 11 minutes after the checkpoint had called them done; and the STATUS
  cut, a design call (moment 4), was committed before its consultation. Since round 1, each consultation comes before
  the "done" save it covers. Details: "The advisor tool", below.

## Plan and approval
- The plan: `.checkpoints/foundation-2-3-docs/plan.md`, revisions 1, 2, 2a, 2b and 2c.
- The advisor agent: APPROVE of revision 1 (ec16b30) with ten Low conditions, C1 to C10, folded in revision 2 ("If
  revision 2 only folds in C1 to C10, it needs no new plan review"). Its answers: M1 as a dated correction (Q1 (a)); the
  decision alone, then the whole message (Q2 (a)); the archive's two copies of the same claims fixed too (Q3); two
  reviewers plus the verifier (Q4); one pointing clause each in two archive follow-ups (Q5); his go for #48 checked
  against the chat record (Q6); #40's row read without switching (Q7).
- Its rulings: 2a (before any tracked edit: L2 worded as seven job logs read; the header note's ending; the Appendix A
  hash check kept; `fixes.mjs --check` re-run on the final head); 2b (step 4's departures accepted; "Last merged" keeps
  its archive pointer; E-TASK's header says STATUS quotes the decision; its High H-1, fixed in step 5); 2c (round 1's
  fix pass: every item approved; the advisor-tool gap stated in these top lines and in the end message; round 2 on what
  changed).
- The conditions: C1 #40's facts read at the build (the second read, on the final head: step 8); C2 the pointers'
  quotes joined and tested, with a negative control; C3 and C4 the wording; C5 the site table (round 1 found three more
  hits, below); C6 six hunks in the 2.2 report; C7 the skeptics only after both reviewers, never more than 3 agents; C8
  his #48 go identical to the chat record (one saved page) and to the 2.2 session's merge call (the other); C9 not met
  in full (the gap above); C10 this header names the notes' last commit and the checkpoint is folded in here (step 8).

## What changed
- `docs/DECISIONS.md`: three entries at the top: his decision of 2026-11-13 (alone, then his whole task message), his
  message before it, and his go for #48. One sentence in his 2026-11-10 entry's header (two lines reflowed, its other
  words unchanged), and the pointer P5 at SLICE-ORDER's condition 3. Nothing leaves "Queued" and nothing is added.
  E-TASK's header says the kit lines are not edited here: "his words of 2026-11-13 are the rule there too" (H-1, High,
  fixed in step 5, and R-WORDS's F2, round 1).
- Dated pointers quoting his decision's first four sentences, "for foundation step 2.3 only": `docs/SPIKE_PLAN.md`
  (P1 to P3), ADR 0002's migration step 1 (P4), `docs/ROADMAP.md` (P7); in `docs/STATUS.md` (P8) the same four
  sentences, quoted after STATUS's own lead-in.
- `docs/reports/2026-11-12-foundation-2-2.md`: M1, L1 (its git wording credited to 9b20b67 as it stood, corrected by
  L2), L2 (worded as the logs read), L3 and a dated header note.
- `docs/STATUS_ARCHIVE.md`: L5; Q3's two lines; Q5's two clauses; #48's row (its time's source named); the 2.2 block's
  opening ("merged as 4d21c1a"); three lines that still called his go for Q2.2-a and OVERFLOW-ALL pending;
  BRIEF-STALE's date; the 2.1a follow-up's line numbers, with ADR 0003 and ADR 0006 added; REQUIRED-CHECK in the past
  tense, with #48's toolchain run ids.
- `docs/STATUS.md` (120 lines): true for main 4d21c1a; the kit line as pointers, with his kit words of 2026-11-13
  quoted to the end of his paragraph; `toolchain` "not a required check (REQUIRED-CHECK, a finding for step 2.3)".
  `docs/ROADMAP.md`: #48 merged, #52's entry, and docs-keeper's fold of the oldest Recent-work entry (#36, the design
  walk) into Build history. `docs/REPO_MAP.md` unchanged (it lists four reports; its `docs/reports/` row covers this).
- This report. No code: the gate's symbol check is +0 −0.

## Evidence
| Id | What | Result |
|---|---|---|
| E1 | his words | `words.mjs`: A1 (42 characters) and A2 (6,190 characters, 36 lines) from this session's transcript; A3 (1,872 characters) from 4d21c1a's message, the round trip OK and identical byte for byte to foundation 2.2's chat record (message 96467900-…, its event stamped 16:16:36 UTC; the commit says 16:16:47) and to that session's merge call; the decision (562 characters) cut by its anchors; the transcript scan finds no other message of his. `report-appendix.mjs`: the blocks and the body's quotes OK |
| E2 | DECISIONS.md | `decisions.mjs --check`: OK; six negative controls caught (each quote, the header sentence, the entry title); the only lines removed against 4d21c1a are the 2026-11-10 header's two reflowed lines; 2.2's `decisions-entries.mjs --check`: OK, 13 entries |
| E3 | the pointers | `pointers.mjs --check`: P1 to P5, P7, P8 and his kit words in STATUS (to the end of his paragraph); each quote joined and found in his decision; a dropped word caught; SPIKE_PLAN +18 −0 lines, ADR 0002 +6 −0 |
| E4 | the fixes | `fixes.mjs --check` on the head: the 2.2 report's Appendix A sha256 equal to 4d21c1a's; 2.2's body-quote check: 13 blocks, 10 quotes; the report's whole-file check (base plus the edits, no other word changed). At step 3 (2f96f5c), `--check --hunks` also showed 6 hunks in the report and 5 in the archive, and the archive's whole-file check, before docs-keeper's step 4. L2: seven job logs read; L3: the saved run's flag list; L1: the 2.2 checkpoint's F1 |
| E5 | scope | `scope.mjs`, main's guard (`protectedRel`, `mergeAllowed` from origin/main 4d21c1a): every path under `docs/`, none protected, file modes plain; DECISIONS.md and ADR 0002 not mergeAllowed, so the PR Advisor cannot merge it |
| E6 | gate | `node tools/gate.mjs --no-smoke`: GATE PASS after each step (the smoke in CI: this container's Chromium is not the toolkit's); repo map OK |
| E7 | CI | Green on each step's commit (gate, with its gate and smoke jobs, and toolchain; push, then pull_request): 1123175 (43497813351, 43497824049; 43497813242, 43497823762); 2f96f5c (43501529364, 43501539671; 43501529328, 43501539371); 83c6ab6 (43504752852, 43504760971; 43504752955, 43504761167); d59ca43 (43506267790, 43506274539; 43506267759, 43506274417); the final head's runs: in the pull request's body at ready |
| E8 | facts | `quotes.mjs` on STATUS, ROADMAP, the archive and REPO_MAP: every quoted span on the added lines found in his words, the base or DECISIONS.md (one split line judged by hand); R-STATE and the verifier on every SHA, run id and time |

## Reviews
Round 1, on d59ca43 (workflow wf_eb6cb5e5-3ce): R-WORDS (his words and decisions) and R-STATE (the state, the fixes,
this report), each a general agent on Opus 5.5 at effort max; 2 of 2 returned. 23 findings: R-WORDS 6, R-STATE 17.
Each Medium or higher was then tested by a skeptic of its own (4 of 4 returned): all four hold, at their severities.
Every finding was fixed in round 1's fix pass unless its row says otherwise.

| Id | Severity | Finding | Disposition |
|---|---|---|---|
| RS-F1 | High (held) | the report called C9 met, though step 3 had no closing consultation | fixed: every gap named (Outcome; "The advisor tool"); the late consultation 6 |
| RS-F2 | Medium (held) | "no command was refused" was false for the agents | fixed: the session's none and the agents' four, with routes ("Decisions and deviations") |
| RW-F1 = RS-F4 | Medium (held) | STATUS's kit quote stopped before his paragraph's last sentence | fixed: quoted to the paragraph's end (2c, item 3) |
| RS-F1b | Low | each step's "done" save came before its consultation | fixed: named in the same list |
| RS-F3 | Low | E4 cited `--check --hunks` as passing on the head | fixed: E4 above |
| RS-SELF | Low | R-STATE's scratch copies held fragments of his words, pushed by the saver | disclosed ("Decisions and deviations") |
| RS-L-a | Low | STATUS: "not a required check until step 2.3" | fixed with "(REQUIRED-CHECK, a finding for step 2.3)"; its suggested text not used, as it stated step 2.3's unapproved plan as fact |
| RS-L-b | Low | the 2.1a follow-up's line numbers, shifted by this PR | fixed: current numbers, the old ones in brackets |
| RS-L-c | Low | REQUIRED-CHECK in the present tense | fixed: past tense, run ids 43452386039 and 43452396473 |
| RS-L-d, RS-L-f, RW-F3 | Low | revision 2b left out; conditions written as met; a garbled count; E8's sources | fixed ("Plan and approval", E8) |
| RS-L-e | Low | "C8 both saved pages identical" overstated | fixed (C8) |
| RS-L-g | Low | docs-keeper's ROADMAP fold not named; "in review" against "draft" | fixed ("What changed"); one state word in the final commit, after the verifier |
| RS-L-h | Low | L1 credited 9b20b67 with the corrected git wording | fixed (2c, item 2) |
| RS-L-i | Low | #48's time without its source | fixed in the archive's row; the 2.2 report's M1 note keeps the DECISIONS entry's title time (it names that entry) |
| RS-L-j | Low | the M1 note's text differs from plan §5's draft | disclosed ("Decisions and deviations") |
| RW-F2 | Low | E-TASK's "his words" open to the old reading; "lists" in the present tense | fixed (2c, item 1) |
| RW-F4 | Low | d59ca43's `decisions.mjs` held the pre-H-1 header | no doc change: the notes' last commit, named in the header, holds the matching script |
| RW-F5 | Low | three "before the foundation" hits missing from the plan's site table | fixed: ADR 0003 :51-52 and ADR 0006 :56-57 added to the archive's follow-up; the register's OQ-0006-13 row (:155) is the preview step, 2.6, which still waits (S8), so it is left |
| RW-F6 = RS-L-k | Low | BRIEF-STALE kept "[verified 2026-11-12]" | fixed |

Round 2 (on what changed since d59ca43), the verifier and the advisor agent's review of the final head: to come.

## The advisor tool: consultations (his words of 2026-11-11)
In this conversation Opus 5.5 answers the advisor tool (the `/advisor fable` output: until `/clear` or `/compact`).
Times are the calls' own, from the session's transcript (UTC). The "step N done" saves: step 0 19:22:36 (ec16b30),
step 1 19:55:30 (35c3e81), step 2 20:02:41 (de3845d), step 3 20:27:37 (1137ec4), step 4 20:47:34 (1cfe292), step 5
20:57:42 (636f260).
1. 18:59:07, moment 1 (the approach): saved first; every pointer quotes all four limits; the grep widened; the decision
   cut by anchors; 2.2's appendix check replaced by a hash check; KIT-FREEZE-2 quoted, not paraphrased; questions put to
   the advisor agent; the final head approved as the commit that deletes the checkpoint.
2. 19:55:37, moment 5 for step 1 (7 seconds after its "done" save) and moment 1 for step 2: the entry title from one
   source; `--check` honours `--only`; the transcript scan; DECISIONS.md's diff proven; the gate before the commit.
3. 20:13:33, moment 5 for step 2 (11 minutes after its "done" save) and moment 1 for step 3: the M5 and pull_request runs
   read (L2 worded as read, revision 2a); the header note's "every other word" with a whole-file check.
4. 20:47:41, moment 5 for step 4 (7 seconds after its "done" save) and moment 1 for step 5: step 4 not done yet (the
   REPO_MAP claim; the #48 row read whole; the archive's word-level diff; the departures ruled as revision 2b).
5. 20:57:48, moment 5 for step 5 (6 seconds after its "done" save) and moment 1 for step 6: `review-r1.js` brought up to
   revision 2b; the saver restarted before the review.
6. 21:58:15, late moment 5 for step 3 (its "done" save had none) and moment 1 for the fix pass: every gap named; L-a's
   suggested text refused; revision 2c asked before any write.
7. 22:09:54, moment 5 for round 1's fix pass, before its commit and "done" save, and moment 1 for round 2: not done yet
   (the word-level diff read; "CI green" limited to the first four commits; revision 2c in the plan; #52's own STATUS
   row and ROADMAP entry made true); round 2 with new notes names, briefed on what changed only.
The gaps: step 0's "done" save had no consultation of its own (number 1 came before the step's work); steps 1, 2, 4 and
5 were consulted after their "done" saves; step 3 had none until number 6; and the STATUS cut (a design call, moment 4)
was committed in 83c6ab6 (20:47) before number 4.

## Decisions and deviations
- L2 is worded as the logs read: on 2026-11-12 the runners differed, and c901187's pull_request run had 2.56.1 at 15:34
  UTC. That refines the line in PR #48's description, on the advisor agent's ruling (2a).
- Step 4's departures, ruled as revision 2b: STATUS's kit line is pointers (FREEZE-FDN's quotes and his exceptions
  stand in DECISIONS.md, "Queued") with his kit words of 2026-11-13 quoted by script; P8 is his four sentences quoted
  after STATUS's lead-in; the old-sections table's two "same names" rows are one; archive lines beyond plan §6; no
  REPO_MAP row.
- The header note says "every other word" unchanged, not "every other line" (the touched paragraphs reflow), backed by
  a whole-file check (2a). The M1 note's text is more precise than plan §5's draft: "(its needs-mustafa triggers, step
  4)" and "a security finding not fixed in the PR, a follow-up included", as main's advisor.md words it (RS-L-j).
- Beyond the five fixes: the archive's copies of the same two claims, and a pointing clause in each of two follow-ups
  (Q3, Q5); round 1's archive fixes (2c, item 5).
- `tools/` was not installed in this container: `npm ci --prefix tools` (CLAUDE.md's setup step; gitignored).
- Commands refused by the guard, the deny list or the classifier: none of the session's (the transcript scan). The
  agents', all by the guard, each with the route taken: the advisor agent, 19:31:18 UTC (a loop writing to a variable
  path; route: a read-only python3 script); R-WORDS, 21:14:45 (the same kind; route: literal paths); R-STATE, 21:05:55
  (a `node -e` line with comments; route: a script file); R-STATE, 21:17:54 (a `sed -i` on a scratch file; route: the
  Edit tool).
- Retyped copies of his words in scratch files, each pushed by the saver and kept by history, none in any doc:
  docs-keeper's `fillcount.mjs` (e6fc6a9; his decision's four sentences and his kit words, byte-identical to the script
  cuts, checked); an earlier version of R-WORDS's notes (ec8f1bd, fragments); R-STATE's notes and `rs-roadmap.mjs`
  (fragments). His words are public in DECISIONS.md already.
- Two of the seven L2 logs (fd14855's push run, 45f1bb5's pull_request run) were read by the session only; the advisor
  agent read the other five.
- The transcript-based checks (`words.mjs`'s A1, A2 and its scan; the chat-record pages) run only in this session's
  container: elsewhere the sessions API re-reads them.

## Risks
- The chat record's copies of his #48 go are API pages saved outside the repository; the sessions API re-reads them.
- STATUS's Routines section dates from 2026-11-10: this account cannot read the university account's routines.

## Findings for step 2.3's plan
- The kit lines that still name steps 2.1 and 2.2 as the only exception (CLAUDE.md :22 and :55, `.claude/agents/advisor.md`
  :108): his decision is the rule there; step 2.3's plan is to list their edits.
- BRIEF-STALE and the "before the foundation" lines now lack the 2026-11-13 decision too (the archive's clauses).
- This container had no `tools/` install; the local smoke still cannot start (the toolkit's Playwright wants another
  Chromium build).
- The advisor-tool rule's moment 5 is easy to miss when the checkpoint is saved first: worth a line in CLAUDE.md's
  advisor-tool wording, which waits in "Queued" for step 2.3.

## Proposed next step
Step 2.3's plan, after #52 merges, in his order.

## Decisions needed
None.

## For Mustafa
1. Read this report and Appendix A (your words, copied by script). Note the advisor-tool gap in "Outcome".
2. When the advisor agent has approved the final head and gate, smoke and toolchain are green on it, say "merge #52" in
   this chat. No Brave check: nothing a player sees, hears or feels changes.
