# Round 2's fix pass (plan revision 2d): the report body's edits in report-body.md, each an exact replacement found
# once; every line written stays within 120 characters. Run once: python3 -I fp2-body.py (from the notes folder)
p = 'report-body.md'
s = open(p, encoding='utf-8').read()
long_before = {l for l in s.split('\n') if len(l) > 120}
def rep(old, new):
    global s
    n = s.count(old)
    if n != 1: raise SystemExit(f'found {n} times: {old[:100]!r}')
    s = s.replace(old, new)

# Outcome: the build line; the gap bullet (R2-L3, R2-L4, the late moment 4); the advisor model (D1 (c)).
rep("""- Built in five commits (1123175, 2f96f5c, 83c6ab6, d59ca43, then review round 1's fix pass), every script check and
  the gate passing; CI green on each (E7). Review round 1 (two
  reviewers, four skeptics) found one High and two Mediums, all fixed; round 2, the verifier and the advisor agent's
  review of the final head follow.
""", """- Built in six commits (1123175, 2f96f5c, 83c6ab6, d59ca43, 554face for review round 1's fix pass, then round 2's fix
  pass), every script check and the gate passing; CI green on each through 554face (E7). Review round 1 (two reviewers,
  four skeptics) found one High and two Mediums, all fixed. Review round 2 (two reviewers, one skeptic) found one High,
  the advisor model (below), and eight Lows, all fixed or disclosed. The verifier and the advisor agent's review of the
  final head follow.
""")
rep("""  steps 1, 2, 4 and 5 were consulted 6 seconds to 11 minutes after the checkpoint had called them done; and the STATUS
  cut, a design call (moment 4), was committed before its consultation. Since round 1, each consultation comes before
  the "done" save it covers. Details: "The advisor tool", below.
""", """  steps 1, 2, 4 and 5 were consulted 6 seconds to 11 minutes after the checkpoint had called them done; and step 4's
  departures, among them the STATUS cut (a design call, moment 4), were committed before their consultation. Since
  round 1, every "done" save has come after its consultation; in round 2's fix pass, the call on recording his status
  request went to the advisor agent before the advisor tool was consulted (moment 4, late: number 10). Details: "The
  advisor tool", below.
- **The advisor model, corrected in round 2:** by the session's records, Fable 5.2 answered every advisor-tool
  consultation, as his task message has it ("Fable 5.2 advises through the advisor tool." (A2)). Until round 2 this
  report and the checkpoint said Opus 5.5, from the `/advisor fable` output's forecast (RW2-F1). Details: "The advisor
  tool".
""")

# Plan and approval: revision 2d.
rep("- The plan: `.checkpoints/foundation-2-3-docs/plan.md`, revisions 1, 2, 2a, 2b and 2c.\n",
    "- The plan: `.checkpoints/foundation-2-3-docs/plan.md`, revisions 1, 2, 2a, 2b, 2c and 2d.\n")
rep("""  fix pass: every item approved; the advisor-tool gap stated in these top lines and in the end message; round 2 on what
  changed).
""", """  fix pass: every item approved; the advisor-tool gap stated in these top lines and in the end message; round 2 on what
  changed); 2d (round 2's fix pass: every item approved; his status request recorded as A4 and in DECISIONS.md in
  E-HI's form, on three conditions: the entry says only what the message is, the appendix states its cut-off, and a
  later message of his is handled the same way; no round 3, as no Critical or High stays open).
""")

# What changed: four DECISIONS entries; the archive's follow-up lines.
rep("""- `docs/DECISIONS.md`: three entries at the top: his decision of 2026-11-13 (alone, then his whole task message), his
  message before it, and his go for #48. One sentence in his 2026-11-10 entry's header (two lines reflowed, its other
  words unchanged), and the pointer P5 at SLICE-ORDER's condition 3. Nothing leaves "Queued" and nothing is added.
  E-TASK's header says the kit lines are not edited here: "his words of 2026-11-13 are the rule there too" (H-1, High,
  fixed in step 5, and R-WORDS's F2, round 1).
""", """- `docs/DECISIONS.md`: four entries at the top: his status request (sent while review round 2 ran), his decision of
  2026-11-13 (alone, then his whole task message), his message before it, and his go for #48. One sentence in his
  2026-11-10 entry's header (two lines reflowed, its other words unchanged), and the pointer P5 at SLICE-ORDER's
  condition 3. Nothing leaves "Queued" and nothing is added. E-TASK's header says the kit lines are not edited here:
  "his words of 2026-11-13 are the rule there too" (H-1, High, fixed in step 5, and R-WORDS's F2, round 1).
""")
rep("""  BRIEF-STALE's date; the 2.1a follow-up's line numbers, with ADR 0003 and ADR 0006 added; REQUIRED-CHECK in the past
  tense, with #48's toolchain run ids.
""", """  BRIEF-STALE's date; the 2.1a follow-up's line numbers, with ADR 0003 and ADR 0006 added and, in round 2's fix pass,
  the other drafts' twelve lines; REQUIRED-CHECK in the past tense, with #48's toolchain run ids.
""")

# Evidence: E1, E2 (re-run after the pass), E7.
rep("""| E1 | his words | `words.mjs`: A1 (42 characters) and A2 (6,190 characters, 36 lines) from this session's transcript; A3 (1,872 characters) from 4d21c1a's message, the round trip OK and identical byte for byte to foundation 2.2's chat record (message 96467900-…, its event stamped 16:16:36 UTC; the commit says 16:16:47) and to that session's merge call; the decision (562 characters) cut by its anchors; the transcript scan finds no other message of his. `report-appendix.mjs`: the blocks and the body's quotes OK |""",
"""| E1 | his words | `words.mjs`: A1 (42 characters), A2 (6,190 characters, 36 lines) and A4 (68 characters) from this session's transcript; A3 (1,872 characters) from 4d21c1a's message, the round trip OK and identical byte for byte to foundation 2.2's chat record (message 96467900-…, its event stamped 16:16:36 UTC; the commit says 16:16:47) and to that session's merge call; the decision (562 characters) cut by its anchors; the transcript scan finds no other message of his up to the appendix's cut-off. `report-appendix.mjs`: the scan first (no problem), then the 4 blocks and the body's 2 quotes OK |""")
rep("""| E2 | DECISIONS.md | `decisions.mjs --check`: OK; six negative controls caught (each quote, the header sentence, the entry title);""",
"""| E2 | DECISIONS.md | `decisions.mjs --check`: OK, four entries; seven negative controls caught (each quote, the header sentence, the entry title);""")
rep("""554face, round 1's fix pass (43517490537, 43517496130; 43517490505, 43517496110); the final head's runs: in the pull request's body at ready |""",
"""554face, round 1's fix pass (43517490537, 43517496130; 43517490505, 43517496110); round 2's fix pass: listed in the final commit; the final head's runs: in the pull request's body at ready |""")

# Reviews: round 2.
rep("""Round 2 (on what changed since d59ca43), the verifier and the advisor agent's review of the final head: to come.
""", """Round 2, on what changed from d59ca43 to 554face (workflow wf_dcd47e10-70e): R2-WORDS and R2-STATE, each a general
agent on Opus 5.5 at effort max; 2 of 2 returned. 10 findings: R2-WORDS 5, R2-STATE 5, one of them found by both
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

The verifier and the advisor agent's review of the final head: to come.
""")

# The advisor tool: the model (D1 (a)); the "done" saves (RW2-F2); entries 2, 3 (R2-L3), 6 (RW2-F2), 7 (R2-L2);
# entries 9 and 10; the gaps (R2-L3, the late moment 4).
rep("""In this conversation Opus 5.5 answers the advisor tool (the `/advisor fable` output: until `/clear` or `/compact`).
Times are the calls' own, from the session's transcript (UTC). The "step N done" saves: step 0 19:22:36 (ec16b30),
step 1 19:55:30 (35c3e81), step 2 20:02:41 (de3845d), step 3 20:27:37 (1137ec4), step 4 20:47:34 (1cfe292), step 5
20:57:42 (636f260), review round 1 22:04:22 (3f98c0b, after number 6), round 1's fix pass 22:19:55 (461c82b, after
number 8).
""", """Fable 5.2 (`claude-fable-5-2`) answered every consultation below, by the session's records: the transcript's
advisor-tool entry names that model (18:47:56 UTC, with A2), and so does each call's usage record. The `/advisor fable`
output (18:30:08 UTC) had said "The current conversation keeps Opus 5.5 as its advisor model until /clear or /compact";
this report and the checkpoint repeated that as fact until round 2 (RW2-F1; "Decisions and deviations"). Why that
forecast did not hold is not established.
Times are the calls' own, from the session's transcript (UTC); the "done" saves' times are their commits'. The "done"
saves: step 0 19:22:36 (ec16b30), step 1 19:55:30 (35c3e81), step 2 20:02:41 (de3845d), step 3 20:27:37 (1137ec4),
step 4 20:47:34 (1cfe292), step 5 20:57:42 (636f260), review round 1 22:04:19 (3f98c0b, after number 6), round 1's fix
pass 22:19:55 (461c82b, after number 8), review round 2 22:52:48 (ef1a7c1, after number 9).
""")
rep("""   source; `--check` honours `--only`; the transcript scan; DECISIONS.md's diff proven; the gate before the commit.
""", """   source; `--check` honours `--only`; the transcript scan; DECISIONS.md's diff proven; the gate before the commit; for
   step 4, docs-keeper's placeholder in STATUS only.
""")
rep("""   read (L2 worded as read, revision 2a); the header note's "every other word" with a whole-file check.
""", """   read (L2 worded as read, revision 2a); the header note's "every other word" with a whole-file check; for step 4,
   placeholders for his kit words and a quote-span check.
""")
rep("""6. 21:58:15, late moment 5 for step 3 (its "done" save had none) and moment 1 for the fix pass: every gap named; L-a's
   suggested text refused; revision 2c asked before any write.
""", """6. 21:58:15, moment 5 for review round 1 (6 minutes before its "done" save), late moment 5 for step 3 (its "done" save
   had none) and moment 1 for the fix pass: every gap named; L-a's suggested text refused; revision 2c asked before any
   write.
""")
rep("""   (the word-level diff read; "CI green" limited to the first four commits; revision 2c in the plan; #52's own STATUS
   row and ROADMAP entry made true); round 2 with new notes names, briefed on what changed only.
""", """   (the word-level diff read; "CI green" limited to the first four commits; revision 2c in the plan; #52's own STATUS
   row and ROADMAP entry made to name the archive's changes truly, their waiting lines fixed only in round 2's fix pass,
   R2-L2); round 2 with new notes names, briefed on what changed only.
""")
rep("""   the 2.2 report's six hunks were re-checked; the checkpoint brought up to date in that save; no docs or script edits
   while round 2 runs.
The gaps: step 0's "done" save had no consultation of its own (number 1 came before the step's work); steps 1, 2, 4 and
5 were consulted after their "done" saves; step 3 had none until number 6; and the STATUS cut (a design call, moment 4)
was committed in 83c6ab6 (20:47) before number 4.
""", """   the 2.2 report's six hunks were re-checked; the checkpoint brought up to date in that save; no docs or script edits
   while round 2 runs.
9. 22:49:58, moment 5 for review round 2 (before its "done" save) and moment 1 for its fix pass: the advisor-model
   correction worded from the records only and made everywhere; revision 2d asked before any write, round 3 put to the
   advisor agent; Mustafa told in chat; the main session's and the agents' models checked in the records.
10. 23:03:56, late moment 4 for recording his status request (after the session's recommendation to the advisor agent,
   its ruling at 23:02:43 and a notes-only script change; before any tracked edit) and moment 1 for writing round 2's
   fix pass: the scripts' expectations first; round 2's count; times from commits; the appendix's cut-off from a
   scan-gated write; the intro's list checked against the scan; midnight.
The gaps: step 0's "done" save had no consultation of its own (number 1 came before the step's work); steps 1, 2, 4 and
5 were consulted after their "done" saves; step 3 had none until number 6; step 4's departures (the STATUS cut, a design
call, moment 4; the archive lines beyond plan §6) were committed in 83c6ab6 (20:47:12) before number 4; and number 10
came after the session had put its recommendation on his status request to the advisor agent.
""")

# Decisions and deviations: the advisor model (D1 (d), with the telling's clause); his status request (D6); the
# refusals (R2-SELF); the scratch copies (RW2-F5).
rep("""## Decisions and deviations
""", """## Decisions and deviations
- The advisor model (RW2-F1, High, held by its skeptic; fixed in round 2's fix pass): from step 0 the checkpoint, and
  from d59ca43 this report, said Opus 5.5 answered the advisor tool, taking the `/advisor fable` output's forecast as
  fact; round 1 compared the claim only with that output. The records say Fable 5.2 (the advisor-tool entry; every
  call's usage record). No chat message named the advisor model before round 2; Mustafa was told in chat after it, at
  first with the claim wrongly placed in an opening message of the session's, which its next message corrected.
- His status request (22:27:05 UTC), sent while review round 2 ran, was found by the transcript scan after round 2: it
  is A4, with an entry in DECISIONS.md in E-HI's form (revision 2d). The appendix states the scan's cut-off; a message
  of his after it is added the same way before the final commit, and one after the final commit means a new head.
""")
rep("""  (a `node -e` line with comments; route: a script file); R-STATE, 21:17:54 (a `sed -i` on a scratch file; route: the
  Edit tool).
""", """  (a `node -e` line with comments; route: a script file); R-STATE, 21:17:54 (a `sed -i` on a scratch file; route: the
  Edit tool); R2-STATE, 22:28:31 (a loop writing to a variable path; route: literal paths).
""")
rep("""  cuts, checked); an earlier version of R-WORDS's notes (ec8f1bd, fragments); R-STATE's notes and `rs-roadmap.mjs`
  (fragments). His words are public in DECISIONS.md already.
""", """  cuts, checked); an earlier version of R-WORDS's notes (ec8f1bd, fragments); R-STATE's notes and `rs-roadmap.mjs`
  (fragments); two versions of R2-WORDS's notes (461c82b and 7b5f53c, fragments; its notes now hold pointers, and no
  pushed version of round 2's other notes holds a run of 30 characters of his words: `fp2-fragments-history.mjs`). His
  words are public in DECISIONS.md already.
""")

# Findings for step 2.3's plan: the advisor model from the records.
rep("""- The advisor-tool rule's moment 5 is easy to miss when the checkpoint is saved first: worth a line in CLAUDE.md's
  advisor-tool wording, which waits in "Queued" for step 2.3.
""", """- The advisor-tool rule's moment 5 is easy to miss when the checkpoint is saved first: worth a line in CLAUDE.md's
  advisor-tool wording, which waits in "Queued" for step 2.3.
- A session should read the advisor tool's model from its records (the transcript's advisor-tool entry), not from the
  `/advisor` command's output, which forecast Opus 5.5 here (RW2-F1).
""")

# For Mustafa, item 1: the correction too.
rep("""1. Read this report and Appendix A (your words, copied by script). Note the advisor-tool gap in "Outcome".
""", """1. Read this report and Appendix A (your words, copied by script). Note the advisor-tool gap and the advisor-model
   correction in "Outcome".
""")

long = [l for l in s.split('\n') if len(l) > 120 and not l.startswith('|') and l not in long_before]
if long: raise SystemExit(f'{len(long)} lines over 120 characters: {long[0][:80]!r}')
open(p, 'w', encoding='utf-8').write(s)
print('ok')
