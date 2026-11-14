# Advisor report: foundation-2-3-docs, step 2.3's first docs pass (2026-11-13; PR #52; branch claude/sweet-mayer-grsukw for cs/foundation-2-3-docs; base main 4d21c1a)

_The notes (the plan, the scripts, the agents' notes, the evidence files) are deleted before the pull request is marked
ready; the last commit that holds them is {{NOTES-SHA}}._

## Goal
Mustafa's first item for step 2.3: "First: the docs pass. Its own small pull request, docs only, with no kit, gate or CI file in it." (A2) His words for PR #48 and his decision of 2026-11-13 go into `docs/DECISIONS.md` by script, with dated pointers at the older lines the decision overtakes; STATUS and ROADMAP become true for main 4d21c1a; the 2.2 report gets its five fixes. Nothing a player sees, hears or feels changes.

## Outcome
- Built in nine commits (1123175, 2f96f5c, 83c6ab6, d59ca43, 554face for review round 1's fix pass, 8e4fe7c for round
  2's, a1d0317 its follow-up, 922cadf the verifier's fix pass, then the final commit, which deletes the notes), every
  script check and the gate passing; CI green on each through 922cadf (E7). Review round 1 (two reviewers, four
  skeptics) found one High and two Mediums, all fixed. Review round 2 (two reviewers, one skeptic) found one High, the
  advisor model (below), and eight Lows, all fixed or disclosed. The verifier failed a1d0317 on one High (how Mustafa
  was told about the advisor model, written from recall), fixed in 922cadf; its re-run passed 922cadf with two Lows,
  fixed in the final commit; the advisor agent's review of the final head follows.
- **A gap against his rule on the advisor tool** (his words of 2026-11-11, moment 5: consult before a step or the task
  is declared done): steps 0 and 3 had no closing consultation of their own (step 3's was made late, consultation 6);
  steps 1, 2, 4 and 5 were consulted 6 seconds to 11 minutes after the checkpoint had called them done; and step 4's
  departures, among them the STATUS cut (a design call, moment 4), were committed before their consultation. Since
  round 1, every "done" save has come after its consultation; in round 2's fix pass, the call on recording his status
  request went to the advisor agent before the advisor tool was consulted (moment 4, late: number 10). Details: "The
  advisor tool", below.
- **The advisor model, corrected in round 2:** by the session's records, Fable 5.2 answered every advisor-tool
  consultation, as his task message has it ("Fable 5.2 advises through the advisor tool." (A2)). Until round 2 this
  report and the checkpoint said Opus 5.5, from the `/advisor fable` output's forecast (RW2-F1). Details: "The advisor
  tool".

## Plan and approval
- The plan: `.checkpoints/foundation-2-3-docs/plan.md`, revisions 1, 2, 2a, 2b, 2c, 2d, 2e and 2f.
- The advisor agent: APPROVE of revision 1 (ec16b30) with ten Low conditions, C1 to C10, folded in revision 2 ("If
  revision 2 only folds in C1 to C10, it needs no new plan review"). Its answers: M1 as a dated correction (Q1 (a)); the
  decision alone, then the whole message (Q2 (a)); the archive's two copies of the same claims fixed too (Q3); two
  reviewers plus the verifier (Q4); one pointing clause each in two archive follow-ups (Q5); his go for #48 checked
  against the chat record (Q6); #40's row read without switching (Q7).
- Its rulings: 2a (before any tracked edit: L2 worded as seven job logs read; the header note's ending; the Appendix A
  hash check kept; `fixes.mjs --check` re-run on the final head); 2b (step 4's departures accepted; "Last merged" keeps
  its archive pointer; E-TASK's header says STATUS quotes the decision; its High H-1, fixed in step 5); 2c (round 1's
  fix pass: every item approved; the advisor-tool gap stated in these top lines and in the end message; round 2 on what
  changed); 2d (round 2's fix pass: every item approved; his status request recorded as A4 and in DECISIONS.md in E-HI's
  form, on three conditions: the entry says only what the message is, the appendix states its cut-off, and a later
  message of his is handled the same way; no round 3, as no Critical or High stays open); 2e (the verifier's fix pass:
  the chat bullet in its own exact words, the rest approved, no new review round); 2f (the final commit's texts).
- The conditions: C1 #40's facts read at the build and again for the final commit (its checkpoint at d0d1cfd, read
  2026-11-14 01:05 UTC); C2 the pointers' quotes joined and tested, with a negative control; C3 and C4 the wording; C5
  the site table (round 1 found three more hits, below); C6 six hunks in the 2.2 report; C7 the skeptics only after both
  reviewers, never more than 3 agents; C8 his #48 go identical to the chat record (one saved page) and to the 2.2
  session's merge call (the other); C9 not met in full (the gap above); C10 this header names the notes' last commit,
  and the checkpoint is folded in ("From the checkpoint").

## What changed
- `docs/DECISIONS.md`: four entries at the top: his status request (sent while review round 2 ran), his decision of
  2026-11-13 (alone, then his whole task message), his message before it, and his go for #48. One sentence in his
  2026-11-10 entry's header (two lines reflowed, its other words unchanged), and the pointer P5 at SLICE-ORDER's
  condition 3. Nothing leaves "Queued" and nothing is added. E-TASK's header says the kit lines are not edited here:
  "his words of 2026-11-13 are the rule there too" (H-1, High, fixed in step 5, and R-WORDS's F2, round 1).
- Dated pointers quoting his decision's first four sentences, "for foundation step 2.3 only": `docs/SPIKE_PLAN.md`
  (P1 to P3), ADR 0002's migration step 1 (P4), `docs/ROADMAP.md` (P7); in `docs/STATUS.md` (P8) the same four
  sentences, quoted after STATUS's own lead-in.
- `docs/reports/2026-11-12-foundation-2-2.md`: M1, L1 (its git wording credited to 9b20b67 as it stood, corrected by
  L2), L2 (worded as the logs read), L3 and a dated header note.
- `docs/STATUS_ARCHIVE.md`: L5; Q3's two lines; Q5's two clauses; #48's row (its time's source named); the 2.2 block's
  opening ("merged as 4d21c1a"); three lines that still called his go for Q2.2-a and OVERFLOW-ALL pending;
  BRIEF-STALE's date; the 2.1a follow-up's line numbers, with ADR 0003 and ADR 0006 added and, in round 2's fix pass,
  the other drafts' twelve lines; REQUIRED-CHECK in the past tense, with #48's toolchain run ids.
- `docs/STATUS.md` (120 lines): true for main 4d21c1a; the kit line as pointers, with his kit words of 2026-11-13
  quoted to the end of his paragraph; `toolchain` "not a required check (REQUIRED-CHECK, a finding for step 2.3)".
  `docs/ROADMAP.md`: #48 merged, #52's entry, and docs-keeper's fold of the oldest Recent-work entry (#36, the design
  walk) into Build history. `docs/REPO_MAP.md` unchanged (it lists four reports; its `docs/reports/` row covers this).
- This report. No code: the gate's symbol check is +0 −0.

## Evidence
| Id | What | Result |
|---|---|---|
| E1 | his words | `words.mjs`: A1 (42 characters), A2 (6,190 characters, 36 lines) and A4 (68 characters) from this session's transcript; A3 (1,872 characters) from 4d21c1a's message, the round trip OK and identical byte for byte to foundation 2.2's chat record (message 96467900-…, its event stamped 16:16:36 UTC; the commit says 16:16:47) and to that session's merge call; the decision (562 characters) cut by its anchors; the transcript scan finds no other message of his up to the appendix's cut-off. `report-appendix.mjs`: the scan first (no problem), then the 4 blocks and the body's 2 quotes OK |
| E2 | DECISIONS.md | `decisions.mjs --check`: OK, four entries; seven negative controls caught (each quote, the header sentence, the entry title); the only lines removed against 4d21c1a are the 2026-11-10 header's two reflowed lines; 2.2's `decisions-entries.mjs --check` on this tree: OK, 13 entries |
| E3 | the pointers | `pointers.mjs --check`: P1 to P5, P7, P8 and his kit words in STATUS (to the end of his paragraph); each quote joined and found in his decision; a dropped word caught; SPIKE_PLAN +18 −0 lines, ADR 0002 +6 −0 |
| E4 | the fixes | `fixes.mjs --check` on the head: the 2.2 report's Appendix A sha256 equal to 4d21c1a's; 2.2's body-quote check: 13 blocks, 10 quotes; the report's whole-file check (base plus the edits, no other word changed). At step 3 (2f96f5c), `--check --hunks` also showed 6 hunks in the report and 5 in the archive, and the archive's whole-file check, before docs-keeper's step 4. L2: seven job logs read; L3: the saved run's flag list; L1: the 2.2 checkpoint's F1 |
| E5 | scope | `scope.mjs`, main's guard (`protectedRel`, `mergeAllowed` from origin/main 4d21c1a): every path under `docs/`, none protected, file modes plain; DECISIONS.md and ADR 0002 not mergeAllowed, so the PR Advisor cannot merge it |
| E6 | gate | `node tools/gate.mjs --no-smoke`: GATE PASS after each step (the smoke in CI: this container's Chromium is not the toolkit's); repo map OK |
| E7 | CI | Green on each step's commit (gate, with its gate and smoke jobs, and toolchain; push, then pull_request): 1123175 (43497813351, 43497824049; 43497813242, 43497823762); 2f96f5c (43501529364, 43501539671; 43501529328, 43501539371); 83c6ab6 (43504752852, 43504760971; 43504752955, 43504761167); d59ca43 (43506267790, 43506274539; 43506267759, 43506274417); 554face, round 1's fix pass (43517490537, 43517496130; 43517490505, 43517496110); 8e4fe7c, round 2's fix pass (43528414752, 43528420177; 43528414776, 43528420127); a1d0317, its follow-up (43529793262, 43529799958; 43529793189, 43529799786); 922cadf, the verifier's fix pass (43536145305, 43536148552; 43536145352, 43536148433); the final head's runs: in the pull request's body at ready |
| E8 | facts | `quotes.mjs` on STATUS, ROADMAP, the archive and REPO_MAP: every quoted span on the added lines found in his words, the base or DECISIONS.md (one split line judged by hand); R-STATE and the verifier on every SHA, run id and time |

## Reviews
Round 1, on d59ca43 (workflow wf_eb6cb5e5-3ce): R-WORDS (his words and decisions) and R-STATE (the state, the fixes,
this report), each a general agent on Opus 5.5, asked for effort max; 2 of 2 returned. 23 findings: R-WORDS 6, R-STATE
17. Each Medium or higher was then tested by a skeptic of its own (4 of 4 returned): all four hold, at their severities.
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

Round 2, on what changed from d59ca43 to 554face (workflow wf_dcd47e10-70e): R2-WORDS and R2-STATE, each a general agent
on Opus 5.5, asked for effort max; 2 of 2 returned. 10 findings: R2-WORDS 5, R2-STATE 5, one of them found by both
(RW2-F4 = R2-L2), so nine in the table. Its one High was then tested by a skeptic (1 of 1 returned): it holds, at High.
Every finding was fixed in round 2's fix pass (plan revision 2d) unless its row says otherwise.

| Id | Severity | Finding | Disposition |
|---|---|---|---|
| RW2-F1 | High (held) | the report said Opus 5.5 answered the advisor tool; the records say Fable 5.2 | fixed from the records (Outcome; "The advisor tool"; "Decisions and deviations"; the appendix's intro; the checkpoint) |
| RW2-F2 | Low | the timeline stopped at step 5's "done" save; no consultation labelled moment 5 for review round 1 | fixed ("The advisor tool") |
| RW2-F3 | Low | #48's row: "quoted whole there" after a bracket naming two sources | fixed: "which quotes it whole" |
| RW2-F4 = R2-L2 | Low | #52's waiting lines in STATUS and ROADMAP still listed review round 1; entry 7's "made true" overstated | fixed; entry 7 reworded |
| RW2-F5 | Low | R2-WORDS's notes held fragments of his words, pushed by the saver | disclosed ("Decisions and deviations") |
| R2-L1 | Low | the 2.1a follow-up left out the other drafts' "before the foundation" lines | fixed: the twelve lines listed |
| R2-L3 | Low | the gaps named only the STATUS cut among step 4's departures; entries 2 and 3 had dropped step 4 | fixed ("The advisor tool") |
| R2-L4 | Low | Outcome's "each consultation comes before the "done" save it covers" read literally false | fixed (Outcome) |
| R2-SELF | Low | R2-STATE's own refusal by the guard, for the refusal list | disclosed ("Decisions and deviations") |

The verifier (a general agent on Opus 5.5, asked for effort max, reading main's verifier.md), on a1d0317: FAIL, with one
High and one Low; everything else it checked held (the gate; the eight scripts; every SHA, time and run id; the records
of the consultations, the agents' models and the refusals; Appendix A and DECISIONS.md). Its scope note, in its words
(its hand-back, transcript line 2922): "A4, E-STATUS and everything after 554face (8e4fe7c and a1d0317) had no reviewer.
Only this verifier covered them." The advisor agent ruled on the texts (plan revisions 2d and 2e) and reviews the final
head.

| Id | Severity | Finding | Disposition |
|---|---|---|---|
| F-V1 | High | "Decisions and deviations" told how Mustafa was told about the advisor model from recall: it claimed two updates no record holds, left out the recorded one of 22:59:04 and dropped the "If true" of 22:48:47 | fixed: the recorded messages, each from its own text (the verifier's fix pass) |
| F-V3 | Low | the copies of his words: "round 2's 14 notes files" held five of round 1's; the list was incomplete | fixed: the certain copies named, with the check and its threshold |

Its re-run, on 922cadf: PASS WITH NOTES. F-V1 fixed; nothing Critical, High or Medium; two Lows and two nits, all
fixed in the final commit.

| Id | Severity | Finding | Disposition |
|---|---|---|---|
| F-V5a | Low | four files named as certain copies hold fragments only (`review-r1.js` with its launched copy, `dk-para.txt`, `dk-para-wrapped.txt`) | fixed: moved to "Fragments", the 30-character threshold in the definition, `words.mjs`'s 28-character anchor named |
| F-V6 | Low | its scope note was reworded | fixed: quoted from its hand-back |

The verifier did not see the changes after 922cadf; the advisor agent's review of the final head covers them.

## The advisor tool: consultations (his words of 2026-11-11)
Fable 5.2 (`claude-fable-5-2`) answered every consultation below, by the session's records: the transcript's
advisor-tool entry names that model (18:47:56 UTC, with A2), and so does each call's usage record. The `/advisor fable`
output (18:30:08 UTC) had said "The current conversation keeps Opus 5.5 as its advisor model until /clear or /compact";
this report and the checkpoint repeated that as fact until round 2 (RW2-F1; "Decisions and deviations"). Why that
forecast did not hold is not established.
Times are the calls' own, from the session's transcript (UTC); the "done" saves' times are their commits'. The "done"
saves: step 0 19:22:36 (ec16b30), step 1 19:55:30 (35c3e81), step 2 20:02:41 (de3845d), step 3 20:27:37 (1137ec4),
step 4 20:47:34 (1cfe292), step 5 20:57:42 (636f260), review round 1 22:04:19 (3f98c0b, after number 6), round 1's fix
pass 22:19:55 (461c82b, after number 8), review round 2 22:52:48 (ef1a7c1, after number 9), round 2's fix pass
23:49:35 (7249a13, after number 11), the verifier's fix pass 2026-11-14 00:46:06 (dd4d880, after number 12), the
verifier 01:04:17 (b37cdd2, after number 13).
1. 18:59:07, moment 1 (the approach): saved first; every pointer quotes all four limits; the grep widened; the decision
   cut by anchors; 2.2's appendix check replaced by a hash check; KIT-FREEZE-2 quoted, not paraphrased; questions put to
   the advisor agent; the final head approved as the commit that deletes the checkpoint.
2. 19:55:37, moment 5 for step 1 (7 seconds after its "done" save) and moment 1 for step 2: the entry title from one
   source; `--check` honours `--only`; the transcript scan; DECISIONS.md's diff proven; the gate before the commit; for
   step 4, docs-keeper's placeholder in STATUS only.
3. 20:13:33, moment 5 for step 2 (11 minutes after its "done" save) and moment 1 for step 3: the M5 and pull_request runs
   read (L2 worded as read, revision 2a); the header note's "every other word" with a whole-file check; for step 4,
   placeholders for his kit words and a quote-span check.
4. 20:47:41, moment 5 for step 4 (7 seconds after its "done" save) and moment 1 for step 5: step 4 not done yet (the
   REPO_MAP claim; the #48 row read whole; the archive's word-level diff; the departures ruled as revision 2b).
5. 20:57:48, moment 5 for step 5 (6 seconds after its "done" save) and moment 1 for step 6: `review-r1.js` brought up to
   revision 2b; the saver restarted before the review.
6. 21:58:15, moment 5 for review round 1 (6 minutes before its "done" save), late moment 5 for step 3 (its "done" save
   had none) and moment 1 for the fix pass: every gap named; L-a's suggested text refused; revision 2c asked before any
   write.
7. 22:09:54, moment 5 for round 1's fix pass, before its commit and "done" save, and moment 1 for round 2: not done yet
   (the word-level diff read; "CI green" limited to the first four commits; revision 2c in the plan; #52's own STATUS
   row and ROADMAP entry made to name the archive's changes truly, their waiting lines fixed only in round 2's fix pass,
   R2-L2); round 2 with new notes names, briefed on what changed only.
8. 22:15:20, moment 5 for round 1's fix pass, before its "done" save: done once the word-level diff on the commit and
   the 2.2 report's six hunks were re-checked; the checkpoint brought up to date in that save; no docs or script edits
   while round 2 runs.
9. 22:49:58, moment 5 for review round 2 (before its "done" save) and moment 1 for its fix pass: the advisor-model
   correction worded from the records only and made everywhere; revision 2d asked before any write, round 3 put to the
   advisor agent; the correction for Mustafa in chat (on record at 23:51:05, transcript line 2869); the main session's
   and the agents' models checked in the records.
10. 23:03:56, late moment 4 for recording his status request (after the session's recommendation to the advisor agent,
   its ruling at 23:02:43 and a notes-only script change; before any tracked edit) and moment 1 for writing round 2's
   fix pass: the scripts' expectations first; round 2's count; times from commits; the appendix's cut-off from a
   scan-gated write; the intro's list checked against the scan; midnight.
11. 23:39:10, moment 5 for round 2's fix pass, after its commit 8e4fe7c and before any "done" save: not done yet (the
   clause on how Mustafa was told reworded, still from recall: the verifier failed it, F-V1; E2's check run on this
   tree; the fragment check widened; what is already known added now), and moment 1 for the verifier (its brief: the
   head, what is pending by design, the CI tools, no words of his in its notes, 2.2's script copy, nothing posted on
   GitHub).
12. 2026-11-14 00:34:39, moment 2 for the clause on how Mustafa was told (its wording failed a second time), moment 1
   for the verifier's fix pass and moment 4 for the form of F-V3's fix, before the advisor agent was asked: the clause
   from the recorded messages only, each read in full and described from its own text; nothing about messages the
   records do not hold; F-V3 narrowed (no counts; the certain copies named); "asked for effort max"; the verifier's FAIL
   recorded as it happened; the same verifier resumed with a narrow brief.
13. 2026-11-14 01:01:09, moment 5 for step 7, the verifier (before its "done" save), and moment 1 for the final edits:
   step 7 done; revision 2f asked before writing; a keyword screen of every chat text from 22:40 UTC; no status line on
   how Mustafa was told until the final commit; the fold pointing at sections; the appendix intro's list re-derived from
   the scan; the saver stopped before the notes SHA; every count recomputed; nothing pushed after the final commit.
{{C14}}
The gaps: step 0's "done" save had no consultation of its own (number 1 came before the step's work); steps 1, 2, 4 and
5 were consulted after their "done" saves; step 3 had none until number 6; step 4's departures (the STATUS cut, a design
call, moment 4; the archive lines beyond plan §6) were committed in 83c6ab6 (20:47:12) before number 4; and number 10
came after the session had put its recommendation on his status request to the advisor agent.

## Decisions and deviations
- The advisor model (RW2-F1, High, held by its skeptic; fixed in round 2's fix pass): from step 0 the checkpoint, and
  from d59ca43 this report, said Opus 5.5 answered the advisor tool, taking the `/advisor fable` output's forecast as
  fact; round 1 compared the claim only with that output. The records say Fable 5.2 (the advisor-tool entry; every
  call's usage record). No chat message named the advisor model before round 2. Six chat messages since bear on it (the
  transcript's lines 1887, 2122, 2869, 2929, 3183 and 3195): at 22:48:47 UTC the session relayed the finding and added
  "If true, I told you the wrong thing at the start."; at 22:59:04 it wrote that it was checking "where I first told you
  the advisor model"; at 23:51:05 it wrote that Fable 5.2 had answered all 11 advisor-tool consultations, that no chat
  message of its own had named the advisor model before 22:48 UTC, that its 22:48 message and its first update after the
  context reset had wrongly said it had told him Opus 5.5 at the start, and that the wrong claim had been only in the
  report and the checkpoint; at 00:31:35 on 2026-11-14 it wrote that the verifier had failed this bullet's earlier
  wording (F-V1), that its 22:59 update is on record, and that two correcting updates it believed it had sent are not;
  at 00:46:25 it wrote that this bullet was now worded only from its four chat messages on record, in the advisor
  agent's exact wording; at 01:00:24 it wrote that its 00:46 update also bore on how it had told him about the advisor
  tool's model, and that by the advisor agent's rule the report had to list it too.
- His status request (22:27:05 UTC), sent while review round 2 ran, was found by the transcript scan after round 2: it
  is A4, with an entry in DECISIONS.md in E-HI's form (revision 2d). The appendix states the scan's cut-off; a message
  of his after it is added the same way before the final commit, and one after the final commit means a new head.
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
- Commands refused by the guard, the deny list or the classifier (the transcript scan): the session's one, by the guard,
  23:33:46 UTC (a `git -C` command; route: git run from the folder); besides, the harness's own Bash rule stopped a
  foreground `sleep` at 23:38:22 (route: CI read through the API instead). The agents', all by the guard, each with the
  route taken: the advisor agent, 19:31:18 UTC (a loop writing to a variable path; route: a read-only python3 script);
  R-WORDS, 21:14:45 (the same kind; route: literal paths); R-STATE, 21:05:55 (a `node -e` line with comments; route: a
  script file); R-STATE, 21:17:54 (a `sed -i` on a scratch file; route: the Edit tool); R2-STATE, 22:28:31 (a loop
  writing to a variable path; route: literal paths); the verifier, 2026-11-14 00:12:07 (a compound command with an `rm
  -rf`, denied by the permission system; route: the Write tool, each step on its own, plain `rm` and `rmdir` of named
  files) and 00:16:46 (the guard: a loop writing to a variable path; route: literal paths).
- Copies of his words in the notes folder, each pushed by the saver and kept by history, none of them in docs/. Certain
  copies, meaning files where some pushed version holds a run of 30 characters or more that is a whole sentence of his
  or longer: the plan (his decision, his kit words and A1, which its pointer texts and checks used); this report's draft
  body (its quotes) and `fp2-body.py`; the checkpoint; the advisor agent's notes (A4 whole, and runs of A2 and A3);
  docs-keeper's `fillcount.mjs` (e6fc6a9; his decision's four sentences and his kit words, byte-identical to the script
  cuts); the scripts that cut or check his words and a review brief (`decisions.mjs`, `pointers.mjs`, `fixpass1.mjs`,
  and `review-r2.js` with its launched copy: anchors, negative controls, and his kit words for the reviewers); and
  R-WORDS's notes at ec8f1bd. `words.mjs` also holds one whole sentence of his under that length: the decision's first
  sentence (28 characters), its cut anchor. Fragments, among others: R-STATE's notes and `rs-roadmap.mjs` (RS-SELF),
  R2-WORDS's notes at 461c82b and 7b5f53c (RW2-F5), `review-r1.js` with its launched copy, and docs-keeper's
  `dk-para.txt` and `dk-para-wrapped.txt`. `fp3-fragments-all.mjs` checks every version of every notes file for runs of
  30 characters or more of his words; shorter shared runs, among them the base SHA, file paths and names his message
  also holds, exist in other files and are not listed. His words are public in DECISIONS.md already.
- Two of the seven L2 logs (fd14855's push run, 45f1bb5's pull_request run) were read by the session only; the advisor
  agent read the other five.
- The transcript-based checks (`words.mjs`'s A1, A2 and its scan; the chat-record pages) run only in this session's
  container: elsewhere the sessions API re-reads them.

## From the checkpoint
The checkpoint's essentials (`.claude/CHECKPOINTS.md` §9): the steps and their commits are in Outcome, E7 and "The
advisor tool" (the "done" saves); the decisions in "Plan and approval" and "Decisions and deviations"; the findings in
"Reviews" and "Findings for step 2.3's plan"; none is open. Numbers not given above: main 4d21c1a's CI (gate run
43461710700, toolchain run 43461710673, both success); PR #40's checkpoint as read for STATUS (0066b63, read on
2026-11-13 at 20:29 UTC; d0d1cfd, read on 2026-11-14 at 01:05 UTC); STATUS at 120 lines.

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
- A session should read the advisor tool's model from its records (the transcript's advisor-tool entry), not from the
  `/advisor` command's output, which forecast Opus 5.5 here (RW2-F1).

## Proposed next step
Step 2.3's plan, after #52 merges, in his order.

## Decisions needed
None.

## For Mustafa
1. Read this report and Appendix A (your words, copied by script). Note the advisor-tool gap and the advisor-model
   correction in "Outcome".
2. When the advisor agent has approved the final head and gate, smoke and toolchain are green on it, say "merge #52" in
   this chat. No Brave check: nothing a player sees, hears or feels changes.
