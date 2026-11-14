# Plan: foundation 2.3, the first docs pass (foundation-2-3-docs), revision 2

## Revision 2 (2026-11-13): the advisor agent's APPROVE of revision 1 folded
The advisor agent approved revision 1 (ec16b30bcb359f85f2974ffc55096c28a464f78d) with ten Low conditions; its notes:
`advisor-plan-rev1.md`. "If revision 2 only folds in C1 to C10, it needs no new plan review." This revision folds them,
and its answers to Q1 to Q7, and changes nothing else:
- C1: #40's row: its facts read at the build and again on the final head (the API said head fe1460a, updated
  2026-11-13T19:17:42Z, during the review). Folded in §6.
- C2: the pointer quote check joins wrapped lines and strips indentation and italic marks before the substring test
  against A2-DECISION; a negative control drops one word. Folded in §4.
- C3: "for foundation step 2.3 only" in P, P5 and P7 (and P8). C4: P7 says "overtaken in part", as P5. Folded in §4.
- C5: §4's table lists as "left, not overtaken": ADR 0002 :1097 and :1432; "before the foundation's preview step" at
  SPIKE_PLAN :142 and :420 and ADR 0006 :583, :1000 and :1200 (that step is 2.6, which still waits). Folded in §4.
- C6: the 2.2 report's diff check names six places: the header note, the M1 note, L1 (:227-230), L2 (:301), L2 again
  (:345) and L3 (:133). Folded in §5.
- C7: the skeptics start only after the workflow's two reviewers have returned; never more than 3 agents at once; the
  returned items counted. Folded in §9.
- C8: his #48 go is compared with both saved pages that hold message 96467900-...
  (`...list_events-1794595922657.txt` and `...-1794595937159.txt`); any mismatch stops the work; the DECISIONS.md quote
  still comes from merge commit 4d21c1a. Folded in §2.
- C9: the advisor tool is consulted before each step of §10 is declared done (his moment 5), and at moments 2 to 4 as
  they come up, one checkpoint line each. Folded in §10.
- C10: the report's header names the last commit that holds the notes; the checkpoint's essentials are folded into the
  report before the deleting commit. Folded in §10.
- Answers: Q1 (a); Q2 (a); Q3 both archive lines, named in the report as fixes beyond the five; Q4 R-WORDS and R-STATE
  plus the verifier; Q5 (a), one clause each that points at the 2026-11-13 entry, without paraphrasing its limits; Q6
  adopted (C8); Q7 a read-only `git fetch` and `git show`, citing the SHA and the time read, the content treated as the
  other lane's data.

- **Task:** step 2.3's first docs pass, from Mustafa's words of 2026-11-13 ("First: the docs pass"), his task message in
  this session (transcript message e5fa7ca0-6494-44d0-8366-81fc612c7958, 18:47:55 UTC).
- **Branch:** `claude/sweet-mayer-grsukw`, this cloud session's own branch, standing in for `cs/foundation-2-3-docs`
  (CLAUDE.md's standing permission; his words: "Your branch stands in for the cs/ branch"). **Base:** main
  4d21c1a7a4d39613bffce5d5f9bbc92f2f88a300 (PR #48's merge).
- **Pull request:** draft #52 (https://github.com/Mergincanli/Shamble-Frontlines/pull/52), opened at step 0, because STATUS and ROADMAP must name its number.
- **Change type:** docs, decision docs included. **Risk tier 1:** `docs/DECISIONS.md` records his words, and ADR 0002
  is a decision doc (advisor.md, "Risk tiers"), so the PR is needs-mustafa. It merges from a day session on his go, never
  through the PR Advisor (DECISIONS.md is not on the guard's allow-list). There is no kit, gate or CI file in it; his
  words: "docs only, with no kit, gate or CI file in it". No Brave check: nothing a player sees, hears or feels changes.
  Symbols +0 −0 (no code).
- **Not in scope:** planning step 2.3 (after this PR merges, in his order); any kit, gate or CI edit; PR #40's branch;
  `spikes/`.

## Revision 2a (2026-11-13): two §5 text changes, approved by the advisor agent's ruling (notes: its addendum)
Found while preparing step 3 and approved before any tracked edit: (1) L2 worded as seven job logs read
(`l2-git-versions.md`: the runners differed on 2026-11-12; 2.56.1 already on c901187's pull_request run at 15:34 UTC);
the texts in `fixes.mjs` (NEW_GIT; L1's "(on 2026-11-12, 2.56.0 or 2.56.1 by runner)"); the report names it "worded as
the logs read" and the end message tells Mustafa the PR description's L2 line was refined by these logs. (2) The header
note ends "Appendix A is unchanged, and so is every other word: the paragraphs these fixes touch are re-wrapped from the
edited line on.", backed by `fixes.mjs`'s whole-file check; conditions: (a) the Appendix A sha256 check stays; (b)
`fixes.mjs --check` runs on the 2.2 report again on the final head (the archive part at step 3 only, before docs-keeper).
§5's L2 and header-note texts below are replaced by these.

## Revision 2e (2026-11-14, asked ~00:40 UTC): the verifier's fix pass (fix pass 3), asked of the advisor agent
The verifier failed a1d0317 (agent ad5b8a361fccd378c; notes verifier-1.md): F-V1 (High), F-V3 (Low); all else held.
Consultation 12 (2026-11-14 00:34:39 UTC) came first. The changed texts (E1 to E4) and the record-only items (E5):

E1. F-V1: the D&D bullet on the advisor model, replacing approved D1(d) (as reworded in a1d0317) whole:
"- The advisor model (RW2-F1, High, held by its skeptic; fixed in round 2's fix pass): from step 0 the checkpoint, and
from d59ca43 this report, said Opus 5.5 answered the advisor tool, taking the `/advisor fable` output's forecast as
fact; round 1 compared the claim only with that output. The records say Fable 5.2 (the advisor-tool entry; every call's
usage record). No chat message named the advisor model before round 2. Three chat messages since bear on it (the
transcript's lines 1887, 2122 and 2869): at 22:48:47 UTC the session relayed the finding and added "If true, I told you
the wrong thing at the start."; at 22:59:04 it wrote that it was checking "where I first told you the advisor model"; at
23:51:05 it wrote that Fable 5.2 had answered all 11 advisor-tool consultations, that no chat message of its own had
named the advisor model before 22:48 UTC, and that the wrong claim had been only in the report and the checkpoint."
(Each read in full from the transcript; 1887 and 2122 described from their own text.)

E2. F-V3: the copies bullet, replacing "- Retyped copies of his words in scratch files, ..." whole:
"- Copies of his words in the notes folder, each pushed by the saver and kept by history, none in any doc. Certain
copies (whole sentences or longer): the plan (his decision, his kit words and A1, which its pointer texts and checks
used); this report's draft body (its quotes) and `fp2-body.py`; the checkpoint; the advisor agent's notes (A4 whole, and
runs of A2 and A3); docs-keeper's `fillcount.mjs` (e6fc6a9; his decision's four sentences and his kit words,
byte-identical to the script cuts), `dk-para.txt` and `dk-para-wrapped.txt`; the scripts that cut or check his words
and the review briefs (`decisions.mjs`, `pointers.mjs`, `fixpass1.mjs`, `review-r1.js` and `review-r2.js` with their
launched copies: anchors, negative controls, and his kit words for the reviewers); and R-WORDS's notes at ec8f1bd.
Fragments, among others: R-STATE's notes and `rs-roadmap.mjs` (RS-SELF), R2-WORDS's notes at 461c82b and 7b5f53c
(RW2-F5). `fp3-fragments-all.mjs` checks every version of every notes file for runs of 30 characters or more of his
words; shorter shared runs, among them the base SHA, file paths and names his message also holds, exist in other files
and are not listed. His words are public in DECISIONS.md already."
(The certain copies were each seen by the session: the run's text printed to the console only, never saved.)

E3. Effort: "each a general agent on Opus 5.5 at effort max" (Reviews, rounds 1 and 2) becomes "each a general agent on
Opus 5.5, asked for effort max"; the verifier's line says the same. The transcripts record the model only.

E4. Reviews: after round 2's table, the verifier as it happened:
"The verifier (a general agent on Opus 5.5, asked for effort max, reading main's verifier.md), on a1d0317: FAIL, with
one High and one Low; everything else it checked held (the gate; the eight scripts; every SHA, time and run id; the
records of the consultations, the agents' models and the refusals; Appendix A and DECISIONS.md). Its scope note:
everything after 554face, the new DECISIONS entry included, had the verifier and the advisor agent but no reviewer.
| F-V1 | High | "Decisions and deviations" told how Mustafa was told about the advisor model from recall: it claimed two updates
no record holds, left out the recorded one of 22:59:04 and dropped the "If true" of 22:48:47 | fixed: the three recorded
messages, each from its own text (fix pass 3) |
| F-V3 | Low | the copies of his words: "round 2's 14 notes files" held five of round 1's; the list was incomplete | fixed:
the certain copies named, with the check and its threshold |
Its re-run on fix pass 3: to come." Outcome's first bullet: eight commits (a1d0317 named, then the verifier's fix pass);
"The verifier failed a1d0317 on one High (how Mustafa was told, written from recall), fixed in fix pass 3; its re-run
and the advisor agent's review of the final head follow."

E5. For the record (the session's text): the refusals bullet gains the verifier's two, dated 2026-11-14 (00:12:07, the
permission system denied a compound command with `rm -rf`; route: the Write tool, each step on its own, plain `rm` and
`rmdir` of named files; 00:16:46, the guard refused a loop writing to a variable path; route: literal paths); entry 9's
"Mustafa told in chat" anchored to the record (the correction is on record at 23:51:05, line 2869); entry 11's "worded
from the records" replaced by "reworded, still from recall: the verifier failed it (F-V1)"; entry 12 (moment 2 for the
clause, moment 1 for fix pass 3, moment 4 for F-V3's form; before the advisor agent was asked); the "done" saves line
gains round 2's fix pass 23:49:35 (7249a13, after number 11); the commit message withdraws a1d0317's claim of two
unrecorded updates; the checkpoint's consultation-11 line and the verifier's brief corrected likewise. The verifier is
resumed (same agent) with a narrow brief: the diff from a1d0317, the gate and the eight scripts, and every sentence about
what was said in chat against a transcript line; head, notes SHA, cut-off and call count updated.

## Revision 2d (2026-11-13, asked 23:01:53 UTC): review round 2's fix pass (the advisor agent's ruling, 23:02:43 UTC:
## APPROVE D1 to D6, D6(a) as (a1) with three conditions; D7 agreed: no round 3; its notes: "PLAN REVISION 2d")
Ruling's conditions for D6(a1): the entry's header says only what the message is ("his status request ... sent while
review round 2 ran") and does not read it as a decision; the report states the cut-off time after which no message of
his is in the appendix; any later message of his before the final commit the same way (A5 on), and one after the final
commit means a new head. D1: keep "Why that forecast did not hold is not established."; the end message names the
correction in its top lines, beside the advisor-tool gap. D7: the verifier must still confirm the fix against the
records (a verifier FAIL counts as High); the advisor agent's final-head review re-checks RW2-F1.
Round 2 on 554face (workflow wf_dcd47e10-70e): 2 of 2 reviewers and 1 of 1 skeptic returned. One High, held by its
skeptic (RW2-F1); nine Lows (RW2-F2 to F5, R2-L1 to L4, R2-SELF). Two more items found by the session after round 2
(D6). The changed texts:

D1. The advisor model (RW2-F1, High). Records, checked by the session (`r2w-advisor-model.py` re-run after the
compaction): the transcript's advisor-tool entry at 18:47:56 UTC (with A2) names claude-fable-5-2, and again at
22:54:40 (after the compaction); each of the 9 calls (18:59:07 to 22:49:58) has an advisor_message usage record, all
claude-fable-5-2. The `/advisor fable` output (18:30:08): "Advisor set to Fable 5.2" then "The current conversation
keeps Opus 5.5 as its advisor model until /clear or /compact, so the tool it has already declared stays unchanged;
turning the advisor on or off applies right away." No chat message of the session named the advisor model before
22:48 UTC (every main-session text searched); the PR body never did. The wrong claim stood in the report (:102, from
d59ca43) and the checkpoint (step 0's line and the consultations heading; corrected in ef1a7c1).
 (a) Report :102. Before: "In this conversation Opus 5.5 answers the advisor tool (the `/advisor fable` output: until
     `/clear` or `/compact`)." After: "Fable 5.2 (`claude-fable-5-2`) answered every consultation below, by the
     session's records: the transcript's advisor-tool entry names that model (18:47:56 UTC, with A2), and so does each
     call's usage record. The `/advisor fable` output (18:30:08 UTC) had said "The current conversation keeps Opus 5.5
     as its advisor model until /clear or /compact"; this report and the checkpoint repeated that as fact until round 2
     (RW2-F1; "Decisions and deviations"). Why that forecast did not hold is not established."
 (b) The appendix intro (report-appendix.mjs). Before: "...whose output said this conversation keeps Opus 5.5 as its
     advisor model until `/clear` or `/compact`." After: "...whose output said this conversation keeps Opus 5.5 as its
     advisor model until `/clear` or `/compact`; by the session's records, Fable 5.2 answered every advisor-tool call
     ("The advisor tool")."
 (c) Outcome, a new bullet after the gap bullet: "**The advisor model, corrected in round 2:** by the session's records,
     Fable 5.2 answered every advisor-tool consultation, as his task message has it ("Fable 5.2 advises through the
     advisor tool." (A2)). Until round 2 this report and the checkpoint said Opus 5.5, from the `/advisor fable`
     output's forecast (RW2-F1). Details: "The advisor tool"."
 (d) "Decisions and deviations", a new first bullet: "The advisor model (RW2-F1, High, held by its skeptic; fixed in
     round 2's fix pass): from step 0 the checkpoint, and from d59ca43 this report, said Opus 5.5 answered the advisor
     tool, taking the `/advisor fable` output's forecast as fact; round 1 compared the claim only with that output. The
     records say Fable 5.2 (the advisor-tool entry; every call's usage record). No chat message named the advisor model
     before round 2; Mustafa was told in chat after it."
 (e) The checkpoint: done (ef1a7c1). The PR body at ready names Fable 5.2; so does the end message.

D2. RW2-F3, #48's row (archive :159). Before: "at 16:16:47 UTC (per the merge commit's message; the chat record's event
is stamped 16:16:36 UTC) and quoted whole there". After: "at 16:16:47 UTC by the merge commit's message, which quotes it
whole (the chat record's event is stamped 16:16:36 UTC)".

D3. R2-L1, the 2.1a follow-up (archive :547-553). Before: "...also ADR 0003 :51-52 and ADR 0006 :56-57, each wrapped
across two lines. Nor to his decision..." After: "...also ADR 0003 :51-52 and ADR 0006 :56-57, each wrapped across two
lines; and in the other drafts: ADR 0001 :603, :1100, :1301, :1771; ADR 0003 :1373; ADR 0004 :1245, :1251; ADR 0005 :92;
ADR 0006 :1469, :1482; the register (`docs/adr/README.md`) :72, :292. Nor to his decision..." (each line read by the
session: every one holds "before the foundation"; none of these files is touched by this PR, so the numbers are main's).

D4. RW2-F4 = R2-L2, #52's waiting lines. STATUS :29, before: "review round 1 (two reviewers), the verifier, the advisor
agent's review of its final head; then his go (risk tier 1, decision docs)"; after: "the verifier, the advisor agent's
review of its final head; then his go (risk tier 1, decision docs)". ROADMAP :123-124, before: "Next: review round 1, the
verifier, the advisor agent's review of the final head, then his go."; after: "Next: the verifier, the advisor agent's
review of the final head, then his go." The final commit then drops "the verifier, " and sets the state word, as planned.

D5. Report-only fixes (the session's text, for the record): RW2-F2 (the "done" saves line gains review round 1 22:04:19
(3f98c0b, after number 6), round 1's fix pass 22:19:55 (461c82b, after number 8), review round 2 22:52:50 (ef1a7c1,
after number 9); entry 6 opens "moment 5 for review round 1 (6 minutes before its "done" save)"); R2-L3 (the gap
sentence: "step 4's departures (the STATUS cut, a design call, moment 4; the archive lines beyond plan §6) were committed
in 83c6ab6 (20:47:12) before number 4"; entries 2 and 3 get back their "for step 4" clauses from the checkpoint); R2-L4
(Outcome: "Since round 1, every "done" save has come after its consultation."); R2-L2's entry 7 ("naming the archive's
changes truly; their waiting lines fixed in round 2's fix pass, R2-L2"); RW2-F5 (the scratch-copies bullet adds R2-WORDS's
round-2 notes, 461c82b and 7b5f53c, fragments, rewritten as pointers); R2-SELF (the refusals bullet adds R2-STATE,
22:28:31, a loop writing to a variable path; route: literal paths); the round-2 Reviews table (10 rows); Outcome's first
bullet (six commits; round 2's counts); consultations 8 and 9 and any later one before the final commit; E7 with
554face's runs.

D6. Found by the session after round 2 (`words.mjs --check` now fails twice):
 (a) His message at 22:27:05 UTC (transcript message a994258f-93c9-45d2-8878-31e7cfee6dcd), sent while round 2 ran,
     is not an item, so the appendix intro's "finds no other message of his" is no longer true. Options: (a1) A4 in
     words.mjs's ITEMS and in Appendix A, copied by script, the intro's "A1 and A2 are his messages" made "A1, A2 and
     A4", and an entry at the top of DECISIONS.md in E-HI's form: "**Taken by Mustafa, 2026-11-13, 22:27:05 UTC: his
     status request, in foundation 2.3's session** (foundation 2.3, PR #52; his message in that change-set's session,
     session_019YNMUWSsyRgaYWxk4auQ5b, on his personal account, extracted by script from the session's transcript,
     message a994258f-93c9-45d2-8878-31e7cfee6dcd), sent while review round 2 ran. Copied by script from
     `docs/reports/2026-11-13-foundation-2-3-docs.md`, Appendix A4, and checked byte for byte against it:" then his
     words as a quote block; (a2) A4 in the appendix only (it decides nothing). Recommended: (a1), E-HI's precedent
     (every message of his in this session is in the register); any later message of his before the final commit the
     same way (A5 on).
 (b) The compaction summary (22:54:40 UTC, a user entry marked isCompactSummary) is unclassified: words.mjs classifies
     it as the harness's, and the intro's list of other entries names "the summary that carried the session on when its
     context filled (22:54:40 UTC)" (the session's only compaction; the transcript's one compact_boundary).

D7. Round 3: the budget allows a third round "only for an open Critical or High". RW2-F1 is fixed in this pass, so none
stays open: no round 3. The verifier checks every claim about what ran against a record (the correction included), and
the advisor agent's review of the final head re-checks RW2-F1's fix.

## Revision 2c (2026-11-13): review round 1's fix pass, ruled by the advisor agent (its notes: "PLAN REVISION 2c")
All approved: (1) E-TASK's header "his words of 2026-11-13 are the rule there too, and step 2.3's plan is to list their
edits"; (2) the 2.2 report's L1 "(as worded at 9b20b67; L2 corrected it after the merge, in Risks and the CUT-ESCAPE
row)"; (3) STATUS's kit quote to the end of his paragraph; (4) STATUS's main line "(REQUIRED-CHECK, a finding for step
2.3)" (the reviewer's text refused: it stated step 2.3's plan as fact); (5) the archive: BRIEF-STALE re-dated, the 2.1a
follow-up's lines (current numbers; ADR 0003 :51-52 and ADR 0006 :56-57 added), REQUIRED-CHECK in the past tense with
run ids, #48's row naming its time's source; (6) the advisor-tool gap disclosed in full, in the report's top lines and
the end message, and a consultation before every "done" save from now on; (7) the refusals (none by the session, four by
agents) and the pushed scratch fragments disclosed; (8) round 2 on what changed only, then the verifier with all 23
findings' dispositions. Site table additions (R-WORDS F5): S9 ADR 0003 :51-52 and ADR 0006 :56-57, "before the
foundation" lines wrapped across two lines (S4's class, now in the archive's follow-up); S10 the register's OQ-0006-13
row (:155, the preview step, 2.6: S8's class, left).

## Revision 2b (2026-11-13): step 4's departures, ruled by the advisor agent (its notes: "PLAN REVISION 2b")
STATUS within 120 lines once his words were filled in: (1) the kit line as pointers, his kit-freeze words quoted; (2)
P8 as his four sentences quoted after STATUS's lead-in; (3) the old-sections table's two "same names" rows joined; (4)
three archive lines beyond §6 (his go for Q2.2-a and OVERFLOW-ALL came with the merge); (5) no REPO_MAP row, and the
claim dropped from STATUS, ROADMAP and the report: all APPROVE. (6) "Last merged" keeps its archive pointer (its two
lines, no line cost). (7) E-TASK's header: "SPIKE_PLAN, ADR 0002's migration step 1 and ROADMAP; STATUS's "Next" quotes
it)". H-1 (High): E-TASK's header "by his words they are the rule there too" becomes "his words are the rule there too".
L-x (Low): ROADMAP's #48 entry "his go asked, then given with the merge". All carried out in step 5 (d59ca43).

## 1. What he asked, and what carries each
1. "My words for PR #48 go into docs/DECISIONS.md, captured by script from merge commit 4d21c1a's message, as step 2.2
   did for PR #51." Carried by §3, entry E-48.
2. "My decision above goes in word for word, with dated pointers at the older lines it overtakes, as foundation 2.1a did
   for my decision of 2026-11-10." Carried by §3, entry E-TASK, and §4.
3. "docs/STATUS.md and docs/ROADMAP.md become true for main 4d21c1a." Carried by §6.
4. "The five report fixes listed in PR #48's description: M1, L1, L2, L3 and L5." Carried by §5.
5. Also: this docs pass's own report (protocol step 6), the archive's row for #48, and REPO_MAP if its check asks.

## 2. His words: sources and capture (never retyped)
The script `words.mjs` (in the notes folder) reads each source:
- **A1:** "Hi, don't do anything yet, setting you up." From this session's transcript
  (`/root/.claude/projects/-home-user-Shamble-Frontlines/7261f3d7-f651-586a-9c39-c052553b68a3.jsonl`), message
  b031379c-ea4c-47db-b279-54e937bf147a, 2026-11-13 18:29:55 UTC: a user entry whose content is one string, and exactly
  one entry with that uuid.
- **A2:** his task message, from the same transcript: message e5fa7ca0-6494-44d0-8366-81fc612c7958, 18:47:55 UTC, one
  string.
- **A2-DECISION:** his decision of 2026-11-13, cut from inside its quote marks in A2: the text after
  `to record word for word:\n"` up to the quote mark that closes it. The script asserts that it starts with
  `Step 2.3 may be planned now.`, ends with `only exception.`, is one line, and equals A2's slice between those anchors.
  The decision holds nested quote marks (`("Steps 2.3 to 2.6 …")`), so cutting at the next quote mark would truncate it.
- **A3:** his go for #48, from merge commit 4d21c1a's message: the "> " lines after "quoted whole", unquoted. Re-quoting
  them must give the commit's block back byte for byte (2.2's `go51()` method). It is also checked byte for byte against
  the 2.2 session's chat record: message 96467900-83d8-4b54-8f8b-7884a0dc0fde in session_019YNMUSc7FA68ujhLgXfEJA's
  events, read with the sessions API (list_events) and saved by the harness to this session's tool-results folder. The
  script parses that saved page, which is not tracked; a reviewer can re-read the event with get_event. C8: the
  second saved page that holds the same message (`...list_events-1794595922657.txt`) is compared too; any mismatch
  stops the work. The DECISIONS.md quote comes from the merge commit, the source his words name.
- **Not words:** `/advisor fable` (18:30:08 UTC, a command; its output said this conversation keeps Opus 5.5 as the
  advisor model until `/clear` or `/compact`) and the workflow-authoring skill, loaded with A2 (18:47:56 UTC). The
  appendix's intro names both.

The script `report-appendix.mjs` writes the new report's "Appendix A. Mustafa's words for this change-set, in full"
(A1, A2 and A3, each in a text fence, a four-backtick fence when the text holds three backticks). It then re-reads the
report and checks every block against its source byte for byte, and every quote of his in the report's body, written as
"<words>" (A<n>), against the item it cites. Exit 1 on any mismatch.

## 3. `docs/DECISIONS.md`
The script `decisions.mjs` copies from the report's appendix, and its `--check` re-verifies the result. Three entries go
at the top of "Decisions, newest first" (within 2026-11-13, newest first). Each header is written for this file; each
quote is copied from the appendix.
- **E-TASK:** **Taken by Mustafa, 2026-11-13, 18:47:55 UTC: his decision on step 2.3 (step 2.3 may be planned now), and
  step 2.3's task** (foundation 2.3, PR #52; his message in that change-set's session, session_019YNMUWSsyRgaYWxk4auQ5b,
  on his personal account, extracted by script from the session's transcript, message
  e5fa7ca0-6494-44d0-8366-81fc612c7958). The header continues: "It changes his words of 2026-11-10 for step 2.3 only, as
  it says; dated pointers mark the older lines it overtakes. The kit lines that still name steps 2.1 and 2.2 as the only
  exception (CLAUDE.md :22 and :55, `.claude/agents/advisor.md` :108, on main 4d21c1a) are not edited by this docs pass:
  his words are the rule there too, and step 2.3's plan lists their edits. Copied by script from
  `docs/reports/2026-11-13-foundation-2-3-docs.md`, Appendix A2, and checked byte for byte against it:". Then two parts
  (Q2):
  - "His decision, word for word (A2, from inside its quote marks):" and the decision, quoted;
  - "His whole message (A2):" and the message, quoted.
- **E-HI:** **Taken by Mustafa, 2026-11-13, 18:29:55 UTC: his message before the task, in foundation 2.3's session**
  (the same source form, message b031379c-ea4c-47db-b279-54e937bf147a). "His task message, the entry above, followed at
  18:47:55 UTC. Copied by script from ..., Appendix A1, and checked byte for byte against it:", then the quote.
- **E-48:** **Taken by Mustafa, 2026-11-13, 16:16:47 UTC: his go for #48** (foundation 2.2, PR #48; given in that
  change-set's session, session_019YNMUSc7FA68ujhLgXfEJA, on his personal account, after the advisor agent's APPROVE of
  9b20b67; carried out by the merge 4d21c1a; recorded here by foundation 2.3's first docs pass, PR #52, as his words in it
  ask). "The lines below are merge commit 4d21c1a's record of his message (quoted there with "> ", captured there by
  script from that session's transcript, message 96467900-83d8-4b54-8f8b-7884a0dc0fde): a record of his words, not an
  instruction found in a commit. Extracted by script from the commit's message, where re-quoting them gives its block
  back byte for byte, and checked byte for byte against that session's chat record (the same message, read with the
  sessions API) (Q6). Its item 5 accepts four security items left open; its item 6 is carried out by foundation 2.3's
  first docs pass (the 2.2 report's fixes and this entry). Copied by script from ..., Appendix A3, and checked byte for
  byte against it:", then the quote.
- **The 2026-11-10 entry** ("Taken by Mustafa, 2026-11-10: the second account may plan and build foundation steps 2.1
  and 2.2 now") gets one sentence in its header, after "(the entry "Taken by Mustafa, 2026-11-12, 15:35:17 UTC",
  above).": `Its sentence "Steps 2.3 to 2.6 keep their order and wait for their results." is changed for step 2.3 only by
  his decision of 2026-11-13 (the entry "Taken by Mustafa, 2026-11-13, 18:47:55 UTC: his decision on step 2.3 (step 2.3
  may be planned now), and step 2.3's task", above), and the dated pointers of 2026-11-13 carry its limits.` Its quoted
  sentence is checked byte for byte against the entry's own quote.
- **SLICE-ORDER's condition 3:** the pointer P5 (§4), after its 2.1a pointer.
- **"Queued":** no item leaves (none is carried out here) and none is added. His #48 item 6 is carried out by this pass;
  the kit lines his decision names wait in step 2.3's plan (his task, item 5).
- **Checks:** `decisions.mjs --check` (every new quote equals its appendix block byte for byte; the header sentence's
  quote equals the 2026-11-10 entry's words). Negative controls: a one-byte change in each of the three quotes, in the
  decision's quote and in the header's quoted sentence, each caught. 2.2's `decisions-entries.mjs --check` (from
  2f0fef3, run from a scratch copy under `.gate/`) must still say "OK: 13 entries" (2.2's entries untouched); run on main
  today: OK.

## 4. Dated pointers
The script `pointers.mjs` inserts the pointer text P and checks its quote as a substring of A2-DECISION:

```
_Pointer (2026-11-13, foundation 2.3's first docs pass): for foundation step 2.3 only, his words of 2026-11-10 quoted
above are changed by his decision of 2026-11-13 (`docs/DECISIONS.md`, the entry "Taken by Mustafa, 2026-11-13, 18:47:55 UTC:
his decision on step 2.3 (step 2.3 may be planned now), and step 2.3's task"): "Step 2.3 may be planned now. The parts
of step 2.3 that need no proof test's result may be built before the proof tests report, after my go on the plan. Every
gate check that needs a result joins later, when that result is in, with its pass marks unchanged. Steps 2.4 to 2.6
still wait for their results."_
```

The quote carries every limit he set: planning now; building only the parts that need no proof test's result, and only
after his go on the plan; the result-bound checks later, with their marks unchanged; steps 2.4 to 2.6 still waiting.

The sites: every hit of 2.1a's pointer marker, of "2.3 to 2.6", of "steps 2.1 and 2.2" and of "before the foundation"
in tracked files outside `docs/reports/`, the archive included (grep of main 4d21c1a, 2026-11-13).

| # | Site on main 4d21c1a | Action |
|---|---|---|
| P1 | `docs/SPIKE_PLAN.md:93-95` (2.1a's pointer, quoting "Steps 2.3 to 2.6 ...") | P added on the next line |
| P2 | `docs/SPIKE_PLAN.md:131-133` | P added |
| P3 | `docs/SPIKE_PLAN.md:441-444` (the foundation entry's "Depends on") | P added, indented like 2.1a's pointer |
| P4 | ADR 0002 :1440-1443 (migration plan, step 1) | P added, indented |
| P5 | `docs/DECISIONS.md:850-852` (SLICE-ORDER condition 3; 2.1a's pointer) | a variant: `_Pointer (2026-11-13, foundation 2.3's first docs pass): for foundation step 2.3 only, overtaken in part by his decision of 2026-11-13: the entry "Taken by Mustafa, 2026-11-13, 18:47:55 UTC: ...", above; its words: "<the same four sentences>"_` |
| P6 | `docs/DECISIONS.md:534-543` (the 2026-11-10 entry) | the header sentence (§3) |
| P7 | `docs/ROADMAP.md:93` (the v2 line's "pointer, 2026-11-11: ...") | `; pointer, 2026-11-13: for foundation step 2.3 only, overtaken in part by his decision of 2026-11-13, `docs/DECISIONS.md`: "<the four sentences>"` (C3, C4) inside the same parentheses, after 2.1a's pointer |
| P8 | `docs/STATUS.md:38` (Next, item 2: 2.1a's pointer) | STATUS is rewritten (§6); its Next item for step 2.3 carries P's quote |
| S1 | CLAUDE.md :22 and :55; `.claude/agents/advisor.md:108` | left: kit files, and no kit edit in this pass (his words); his decision is the rule there; step 2.3's plan lists the edits |
| S2 | `docs/V2_PHASE0_BRIEF.md:20-21` and `:144` | left: his and the Advisor's brief; no session edits it (BRIEF-STALE) |
| S3 | `docs/STATUS_ARCHIVE.md:853-856` (BRIEF-STALE, a live follow-up) | Q5: one clause added naming the 2026-11-13 decision (recommended), or left |
| S4 | the five "before the foundation" lines 2.1a left (ADR 0002 :38, :724, :1129, cited then as :1115; SPIKE_PLAN :100, :202) and the other drafts' lines (ADR 0001 :603, :1100, :1301, :1771; ADR 0003 :1373; ADR 0004 :1245, :1251; ADR 0005 :92; ADR 0006 :1469, :1482; the register :72, :292) | left: the archive's follow-up "For the drafts' next revision: five "before the foundation" lines with no pointer" covers them; Q5: one clause added to it naming the 2026-11-13 decision too (recommended) |
| S5 | `docs/ROADMAP.md:114` and `:142` (2.2's and 2.1a's entries name the 2026-11-10 decision) | left: true as history |
| S6 | `docs/DECISIONS.md:550` ("3. Name what in steps 2.3 to 2.6 must wait") | left: his task words of 2026-11-10 |
| S7 | ADR 0002 :1097 (the layout research before the foundation) and :1432 ("with the netcode proof test first") | left, not overtaken (C5) |
| S8 | "before the foundation's preview step": SPIKE_PLAN :142 and :420; ADR 0006 :583, :1000 and :1200 | left, not overtaken: that step is 2.6, which still waits (C5) |

**Checks:** each pointer's quote is a substring of A2-DECISION (and so of A2), tested after joining the pointer's wrapped
lines and stripping indentation and italic marks (C2), with a negative control that drops one word from one pointer;
each pointer sits on the line right after its anchor (2.1a's pointer's last line); P1 to P5 and P7 appear exactly once
each; SPIKE_PLAN's and ADR 0002's diffs are added lines only.

## 5. The five report fixes
Their source: PR #48's description, and the advisor agent's findings in its review of 9b20b67, read from the 2.2
session's record (list_events, the tool result of 2026-11-13 ~15:55 UTC).

In `docs/reports/2026-11-12-foundation-2-2.md`:
- **M1** (Medium): "For Mustafa", step 3, says his merge accepts "one security finding left open"; four were. Q1.
  Recommended (a): keep step 3 as written, since it is what was asked of him, and add a dated correction under it:
  ```
     _Corrected after the merge (2026-11-13, by step 2.3's first docs pass, PR #52; the advisor agent's M1 on 9b20b67):
     four security findings were left open, not one. By main's `.claude/agents/advisor.md` (PR review, step 4), a
     finding not fixed in the PR counts as left open, so beside SA2-2 three of round 1's stay open for step 2.3, each a
     CI edit that asks him: SETPRIV-HARDEN (SA-2), the SETUP-NODE-CACHE setting (SA-3) and CUT-ESCAPE (SA-1). His go for
     #48 accepted all four (item 5; `docs/DECISIONS.md`, 2026-11-13, 16:16:47 UTC)._
  ```
  Option (b): rewrite step 3's last sentence in place to name the four, with the header note only.
- **L1** (the verifier paragraph, :227-230): `Its eight Low notes are fixed in the last commit before ready: the
  runner's git version (2.56.1 since 2026-11-12), ..., the checkpoint (deleted) and a comma in E2.` becomes `Of its
  eight Low notes, seven are fixed in the last commit before ready: the runner's git version (2.56.1 from a5823cf's runs
  on, 2026-11-12 18:39 UTC), ..., the checkpoint (deleted) and a comma in E2; the eighth, that departure
  (OVERFLOW-XCRATE-CI's recorded line), was named for the advisor agent's review of the final head, which accepted it.`
  Sources: the 2.2 checkpoint at 2f0fef3, "F1 a departure from D1-R2 not in the brief's list (OVERFLOW-XCRATE-CI's
  recorded line, report and archive)"; the advisor agent: "F1 was named for this review, not fixed".
- **L2** (Risks, :300-301, and the CUT-ESCAPE row, :345): `(the runner's version on 2026-11-10; 2.56.1 since
  2026-11-12)` becomes `(the runner's version on 2026-11-10, and still on c901187's and fd14855's runs of 2026-11-12;
  2.56.1 from a5823cf's runs on, 2026-11-12 18:39 UTC)`. The versions are read from the checkout step's log of each run
  (c901187: 43249204290; fd14855: 43259689336; a5823cf: 43282089237).
- **L3** (E12's row, :133): `rustc's overflow flag on `sim` only;` becomes `rustc's overflow flag on `sim` only (the
  saved run did not compile `protocol`, whose own block also sets it);`. Source: `xcrate-before-1de98a9.txt` at
  2f0fef3, whose flag list names extdep, room, sim and sim_wasm only.
- **A dated header note**, under the report's first italic paragraph:
  ```
  _Corrected after the merge (2026-11-13) by step 2.3's first docs pass (PR #52), as his go for #48 asks (item 6): the
  advisor agent's report lines from its review of 9b20b67, listed in PR #48's description: M1 (a dated correction under
  "For Mustafa", step 3), L1 and L2 (the verifier paragraph in "Reviews"; L2 also in "Risks" and the CUT-ESCAPE row) and
  L3 (E12's row); L5 is in `docs/STATUS_ARCHIVE.md`. Appendix A and every other line are unchanged._
  ```

In `docs/STATUS_ARCHIVE.md`:
- **L5:** the "From foundation 2.2" block's intro (:574): `readable at the branch's last checkpoint commit` becomes
  `readable at the branch's last checkpoint commit, 2f0fef3b208c10d5c38b921d24fb5f90f71b7ecb`.
- Q3: also EMPTY-SLOTS (:834: `readable at the branch's last checkpoint commit;` becomes `..., 2f0fef3;`) and L2's text
  at :731-732 (the same claim as the report's Risks).

**Checks:** the 2.2 report's Appendix A is byte-identical to origin/main's (sha256 of the text from the appendix heading
to the end); the body-quote check of 2.2's `report-appendix.mjs` (its `check()`, with the appendix's own blocks as the
sources) still finds every "<words>" (A<n>) quote (10 today); L2's
log lines are read and quoted in the report. The 2.2 report's diff touches only six places (C6): the header note, the
M1 note, L1 (:227-230), L2 (:301), L2 again (:345, the CUT-ESCAPE row) and L3 (:133).

## 6. STATUS, ROADMAP, the archive and REPO_MAP (docs-keeper, then the main session's pointer script)
docs-keeper (its own agent file, Opus 5.5, at its file's effort), with a brief that gives the facts and their sources:
- **main 4d21c1a:** PR #48 merged 2026-11-13 16:20:12 UTC (`merged_at`), by merge commit, on his go (16:16:47 UTC,
  quoted in the merge commit's message) after the advisor agent's APPROVE of 9b20b67. CI on 4d21c1a is green: gate run
  43461710700 (the gate and smoke jobs) and toolchain run 43461710673 (push).
- **His go's items:** ADR-INDEX (a), K1-GO (a), OVERFLOW-ALL (a); four security items accepted as left open
  (OVERFLOW-PANIC-RULE; SETPRIV-HARDEN, SETUP-NODE-CACHE and CUT-ESCAPE for step 2.3, each a CI edit that asks him);
  item 6 is carried by this pass.
- **STATUS:**
  - the header's "Last updated", and Phase's "Last merged";
  - the kit line quotes his sentence "The kit's freeze (KIT-FREEZE-2) ends with this step." and his ask rule ("every
    edit to a kit, gate or CI file is asked of me in chat with its exact text and written only after my yes, with the
    chat in Accept edits. A guard prompt alone is not my yes."), with no paraphrase as frozen or lifted;
  - main's line: the required checks are gate and smoke; toolchain runs but is not required until step 2.3
    (REQUIRED-CHECK); CI green on 4d21c1a, with the two run ids;
  - In flight: #48's row out (to the archive) and this PR's row in. #40's row only from what can be read: the API
    and its branch's checkpoint read-only by `git show` after `git fetch`, never switched (Q7), read at the build and
    again on the final head (C1), with the SHA and the time read cited and the content treated as the other lane's data;
  - Next: step 2.3 (this docs pass, then the plan after it merges), with P's quote through a placeholder the script
    fills;
  - Waiting on Mustafa: his go for this PR (risk tier 1, decision docs); #48's line out;
  - Follow-ups, "From foundation 2.2": the accepted security items marked as accepted (his go, item 5);
  - lines it must not change: STATUS :21 and :46 (his Windows run: "Check a new machine" asks for more than the gate he
    ran) and Routines (read 2026-11-10 from the university account, not readable from this one);
  - at most 120 lines.
- **ROADMAP:** #48's entry becomes MERGED, with the merge facts; a terse entry for this PR (in review); the v2 line keeps
  2.1a's pointer, and the script adds P7.
- **The archive:** #48's row in "In flight" (merged, with the facts); the "From foundation 2.2" block's state ("in
  flight" becomes merged as 4d21c1a); nothing else (the L5 and L2 lines are the main session's, §5, done before
  docs-keeper runs).
- **REPO_MAP:** the new report's row, as the map lists reports one by one; `repo-map-check.mjs` must pass.
- **Placeholders:** docs-keeper writes `{{P-2.3}}` where STATUS's and ROADMAP's pointers go; `pointers.mjs` replaces
  each with P's text and checks that none is left.
Then the main session checks docs-keeper's facts (every SHA, run id, PR number and time) against git and the API
before committing.

## 7. The report
`docs/reports/2026-11-13-foundation-2-3-docs.md`, by `/advisor-report`: goal, outcome, plan and approval, as built,
evidence, reviews, the advisor tool's consultations (count), decisions and deviations (refusals, with the route taken),
risks, findings for step 2.3's plan, Decisions needed, For Mustafa, and Appendix A (his words).

## 8. Evidence (docs: every claim cites its source; the verifier spot-checks)
- **E1, his words:** `words.mjs` (A1 and A2 from the transcript; A2-DECISION with its anchors; A3 from 4d21c1a with the
  round trip, and against the chat record); `report-appendix.mjs` (blocks and body quotes).
- **E2, DECISIONS.md:** `decisions.mjs --check`; the negative controls; 2.2's `decisions-entries.mjs --check` still OK.
- **E3, the pointers:** `pointers.mjs --check` (quotes, places, counts); §4's grep re-run on the head.
- **E4, the five fixes:** the 2.2 report's Appendix A hash equals main's; the body-quote check; the CI log lines for L2;
  `xcrate-before` for L3; F1 for L1.
- **E5, scope:** `git diff --name-only origin/main...HEAD` lists only `docs/` paths (and `.checkpoints/` until the last
  commit); main's guard (`protectedRel`, `mergeAllowed`, imported from `git show origin/main:.claude/hooks/routine-guard.mjs`
  in a scratch copy) on that list finds no protected file, with DECISIONS.md and ADR 0002 not mergeAllowed (a
  day-session merge on his go).
- **E6:** STATUS at most 120 lines; `node .claude/scripts/repo-map-check.mjs` OK; `node tools/gate.mjs --no-smoke` GATE
  PASS (the smoke runs in CI: this container's Chromium is not the toolkit's, as in 2.2).
- **E7, CI on the final head:** gate and smoke (required) and toolchain, each for push and pull_request, read by run id.
- **E8, facts:** every SHA, run id, PR number and time in STATUS, ROADMAP and the archive matches git and the API
  (round 1's R-STATE and the verifier).

## 9. Reviews (within the budget) and agents
- **Round 1** (one round; a second only on what changed; a third only for an open Critical or High), on the built head,
  as a workflow of two agents (Q4):
  - R-WORDS reviews adversarially every line that records, quotes or points at his words or decisions: fidelity, the
    headers' facts, every limit carried, nothing loosened or recorded as decided while open, sites missed.
  - R-STATE reviews adversarially STATUS, ROADMAP, the archive, REPO_MAP and the five fixes against git, the API and CI
    logs.
  - Both are general agents on Opus 5.5 at effort max (his words: "reviewers and the verifier may run at effort max").
    Each Medium or higher finding is then tested by a skeptic of its own (Opus 5.5, max, at most 3 at a time), started
    only after both reviewers have returned (C7: never more than 3 agents at once; the returned items counted); the
    session checks the Lows. One fix pass.
- **The verifier:** a general agent reading main's `verifier.md`, as in 2.2 (Opus 5.5, max), on the head after the fix
  pass, until PASS.
- **The advisor agent's review of the final head:** the commit that deletes the checkpoint, recorded in the PR body;
  nothing is pushed after it.
- No named reviewer from CLAUDE.md step 5 fits a docs-only change with no code, CI, network or storage (security-auditor
  is for risk tier 2); Q4 asks the advisor agent.
- docs-keeper runs once (§6). At most 3 agents at a time; the saver runs while agents run; a checkpoint after every step.
- **Every agent brief:** CHECKPOINTS.md's "Saving your progress" block with the absolute notes path
  `/home/user/Shamble-Frontlines/.checkpoints/foundation-2-3-docs/`; write with the file tools, or one plain command with
  a literal path; `model: "opus"` for general agents; an explicit effort on every workflow `agent()` call; the returned
  items counted.

## 10. Steps and commit points
The advisor tool is consulted before each step below is declared done (his moment 5), and at moments 2 to 4 as they
come up, with one checkpoint line each (C9).
0. The checkpoint and this plan saved; the saver started; the draft PR opened; the advisor tool consulted (1).
1. The advisor agent's plan review (APPROVE, REVISE or ESCALATE) against this plan's commit SHA; its conditions folded
   into revision 2.
2. The scripts and the new report's skeleton with Appendix A; DECISIONS.md's entries, the header sentence and P5; P1 to
   P4 (`pointers.mjs`); the checks and negative controls. One commit.
3. The five fixes (the 2.2 report and the archive); the CI log reads; the checks. One commit.
4. docs-keeper's sync; the placeholders filled (P7, P8); the fact checks. One commit.
5. The report's body; E1 to E8; the gate; push; CI read.
6. Review round 1, the skeptics, one fix pass.
7. The verifier, until PASS.
8. The advisor tool (moment 5); the checkpoint's essentials folded into the report; the saver stopped; the last
   checkpoint commit made; the deleting commit by hand writes that commit's SHA into the report's header (C10) and deletes
   `.checkpoints/foundation-2-3-docs*` (CI runs); the advisor agent's review of that SHA; the PR body; ready; stop for his go. The end message gives the PR's
   number and head SHA, the run ids of gate, smoke and toolchain on that head (push and pull_request), and what is needed
   from him.

## 11. Risks
- A pointer that drops one of his limits reads as a looser rule (High on the repo's scale). Mitigation: one quote,
  checked by script, at every site; R-WORDS's brief names it.
- His #48 go: the chat record's copy comes from a saved API page outside the repository. If it differs from the commit's
  quote, stop and report; never pick one silently.
- docs-keeper's sync touches facts it cannot see (the university account's routines); its brief forbids changing those
  lines.
- STATUS's 120-line cap: the new kit line and the Next item must fit; docs-keeper counts.
- A refused command (guard, deny list, classifier) is recorded with the route taken (his words).

## 12. Questions for the advisor agent
- **Q1, M1's form:** (a) a dated correction under step 3 (recommended: it keeps what was asked of him); (b) step 3
  rewritten in place.
- **Q2, E-TASK's form:** (a) the decision alone, then the whole message (recommended: "to record word for word", and the
  full record, as 2.1b's and 2.2's task entries); (b) the whole message only, with the header naming the decision; (c)
  the decision only, with the message in the report's appendix.
- **Q3, beyond the five:** the archive's copy of L2's claim (:731-732) and its second "last checkpoint commit" line
  (:834): fix both (recommended: the same claims), or the named lines only.
- **Q4, reviewers:** R-WORDS and R-STATE as general agents plus the verifier, or a named reviewer as well.
- **Q5, BRIEF-STALE and the "before the foundation" follow-up:** one clause each naming the 2026-11-13 decision
  (recommended), or leave both.
- **Q6, A3 against the chat record:** adopt (recommended), or extract from the commit only, as 2.2 did for #51.
- **Q7, #40's row:** read its branch's checkpoint read-only (`git fetch` and `git show`, no switch), or carry the row over
  with its date.
