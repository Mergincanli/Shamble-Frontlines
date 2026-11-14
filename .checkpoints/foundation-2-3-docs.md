# Checkpoint: foundation-2-3-docs
- **Goal:** step 2.3's first docs pass (Mustafa's words of 2026-11-13): his go for #48 and his 2026-11-13 decision in
  DECISIONS.md by script, with dated pointers; STATUS and ROADMAP true for main 4d21c1a; the 2.2 report's fixes M1, L1,
  L2, L3 and L5. Its own PR, docs only; ready, then stop for his go. After it merges (a new checkpoint): plan step 2.3.
  · plan: `.checkpoints/foundation-2-3-docs/plan.md`
- **Branch:** claude/sweet-mayer-grsukw (for cs/foundation-2-3-docs) · **Base:** main
  4d21c1a7a4d39613bffce5d5f9bbc92f2f88a300
- **Last update:** 2026-11-13 23:17 Amsterdam

## Steps done
0. The model check: Opus 5.5 (get_session: session_context.model and last_served_model are claude-opus-5-5; effort
   max). The advisor tool is on. CORRECTED (round 2, RW2-F1): the `/advisor fable` notice predicted Opus 5.5 behind it until
   `/clear` or `/compact`, but the records show Fable 5.2 (claude-fable-5-2) answered every consultation: the one
   advisor_tool attachment (18:47:56 UTC, model claude-fable-5-2) and every call's advisor usage record. Orientation: CLAUDE.md, CHECKPOINTS.md, STATUS, DECISIONS (2026-11-10 to 11-13 and
   "Queued"), 4d21c1a's message, the 2.2 report, PR #48's description, the advisor agent's verdict on 9b20b67 (the 2.2
   session's record, list_events) and 2.2's notes at 2f0fef3.
1. The checkpoint and plan revision 1 saved (30483c3); the draft PR opened: #52
   (https://github.com/Mergincanli/Shamble-Frontlines/pull/52); the plan names #52 (sed, `#N` to `#52`, 5 places).
2. The advisor agent's plan review of revision 1 at ec16b30bcb359f85f2974ffc55096c28a464f78d (2026-11-13, ~19:32 UTC,
   notes `advisor-plan-rev1.md`): **APPROVE with conditions C1 to C10, all Low**; Q1 (a), Q2 (a), Q3 both lines, Q4
   R-WORDS + R-STATE + the verifier, Q5 (a), Q6 adopt, Q7 read-only fetch allowed. Folded into plan revision 2 (only
   C1 to C10 and the answers: no new review needed, its words). Scripts follow it: `pointers.mjs` (C2 joined-quote
   check, C3/C4 wording), `words.mjs` (C8: both saved pages compared, identical).

3. Step 2 (plan §10): 1123175 (pushed) · the report's draft with Appendix A (A1, A2, A3: `report-appendix.mjs` OK);
   DECISIONS.md: E-TASK (the decision and the whole message), E-HI, E-48 at the top, the 2026-11-10 header sentence (2
   lines reflowed, the only removed lines vs 4d21c1a), P5 (`decisions.mjs --check --negative`: OK, 6 of 6 controls
   caught); P1 to P4 in SPIKE_PLAN and ADR 0002 (`pointers.mjs --check --only P1..P5` OK; SPIKE_PLAN +18 -0, ADR 0002
   +6 -0); 2.2's `decisions-entries.mjs --check` OK (13 entries); GATE PASS (--no-smoke; `tools/` installed with `npm ci
   --prefix tools`, gitignored); repo map OK. CI on 1123175 green: gate (push 43497813351, pull_request 43497824049;
   each with its gate and smoke jobs) and toolchain (push 43497813242, pull_request 43497823762).

4. Step 3 (plan §10): 2f96f5c (pushed) · the 2.2 report: the header note, M1, L1 (with L2), L2 (Risks, CUT-ESCAPE
   row), L3; the archive: L5, Q3 x2, Q5 x2. L2 is worded as seven job logs read (`l2-git-versions.md`: c901187's
   pull_request run at 15:34 UTC already had 2.56.1), on the advisor agent's ruling (plan revision 2a, with the header
   note's new ending; its conditions: keep the Appendix A hash check; re-run `fixes.mjs --check` on the final head).
   `fixes.mjs --check --hunks` OK at 2f96f5c's tree: Appendix A sha256 equal to 4d21c1a's; 2.2's body-quote check 13
   blocks, 10 quotes; whole-file checks (report and archive) OK; hunks vs 4d21c1a: the report 6 (-7,0 +8,6; -133
   +139; -227,4 +233,5; -301,5 +308,7; -345 +354; -447,0 +457,6), the archive 5 (-548 +548,3; -574,4 +576,5; -732,6
   +735,8; -834 +839; -855,2 +860,4). GATE PASS (--no-smoke); repo map OK.

5. Step 4 (plan §10): 83c6ab6 (pushed) · docs-keeper's sync (notes `docs-keeper-sync.md`): STATUS, ROADMAP (the oldest
   entry folded into Build history), the archive's #48 row and the 2.2 block's opening; its REPO_MAP row taken out (the
   map lists 4 reports only). The session's `sync-fixes.mjs`: STATUS back to 120 lines once filled (the kit line as
   pointers; the old-sections table's two "same names" rows joined; P8 as his four sentences quoted, after STATUS's own
   lead-in), the archive's opening split under 120 columns, three archive lines that still called his go for Q2.2-a and
   OVERFLOW-ALL pending. `pointers.mjs` filled P7, P8 and Q-KIT; full `pointers.mjs --check` OK; `quotes.mjs`: 13 spans,
   all found; `fixes.mjs --check`, `decisions.mjs --check`, `report-appendix.mjs --check` OK; scope OK; GATE PASS; repo
   map OK. docs-keeper's findings: its scratch `fillcount.mjs` held a retyped copy of his four sentences and kit-freeze
   words, pushed by the saver (e6fc6a9) and then deleted; byte-identical to the script cut (checked); in no doc
   (a report finding, Low); the archive's stale :608 (fixed here). CI on 83c6ab6 green: gate push 43504752852,
   pull_request 43504760971; toolchain push 43504752955, pull_request 43504761167 (2f96f5c: gate 43501529364,
   43501539671; toolchain 43501529328, 43501539371). Proofs (consultation 4): the archive's word-level diff
   2f96f5c..83c6ab6 shows only the #48 row, the block opening and the three "go" phrases; STATUS's base lines 21 and 46
   are present unchanged and Routines is identical to 4d21c1a's; the #48 row's order, times (16:20:10 commit, 16:20:12
   merged_at), parents (2aa5e9c, 9b20b67) and items match git and A3.

6. Step 5 (plan §10): d59ca43c559fe085ca90f45fdbac593236399533 (pushed) · the report's body merged above Appendix A
   (only {{NOTES-SHA}} left, for step 8); E-TASK's header fixed (revision 2b item 7 and H-1, `etask-header.mjs`); STATUS
   "Last merged" restored and #52's row; ROADMAP's #52 and #48 entries (`step5-fixes.mjs`); all checks OK (words.mjs: 17
   entries classified; report-appendix, decisions --negative, pointers, fixes, quotes 15 spans; STATUS 120; scope;
   GATE PASS; repo map). Revision 2b recorded in the plan.

7. Review round 1 on d59ca43 (workflow wf_eb6cb5e5-3ce, done ~21:56 UTC): 2 of 2 reviewers, 4 of 4 skeptics returned.
   R-WORDS: REVISE, 6 findings (F1 Medium: STATUS's kit quote stops before his paragraph's last sentence; F2-F6 Low).
   R-STATE: HIGH FOUND, 17 findings (F1 High: the report calls C9 met though step 3 had no moment-5 consultation; F2
   Medium: "no command was refused" is false for the agents; F4 Medium = R-WORDS F1; F1b, F3, RS-SELF, L-a to L-k Low).
   Each Medium or higher finding held under its skeptic, at its severity. Fix pass: plan revision 2c asked of the
   advisor agent (the changes to approved texts, the C9 timeline, the refusals, round 2 or not).

8. Round 1's fix pass (plan revision 2c, the advisor agent's ruling: every item approved; conditions: the advisor-tool
   gap in the report's top lines and the end message; a consultation before every "done" save; round 2 on what
   changed): 554faceded1948a39eb635c42c4aa12ac9edf496 (pushed 22:13 UTC; the notes as used: 56e7750). CI green: gate push
   43517490537, pull_request 43517496130; toolchain push 43517490505, pull_request 43517496110. Checks: the word-level
   diff d59ca43..554face (only 2c's phrases in STATUS, ROADMAP, the archive, DECISIONS and the 2.2 report); the 2.2
   report's hunks vs 4d21c1a: 6; all scripts --check OK; STATUS 120; GATE PASS. Consultation 8 before this save.

9. Review round 2 on 554face (workflow wf_dcd47e10-70e, done ~22:48 UTC): 2 of 2 reviewers, 1 of 1 skeptic returned.
   R2-WORDS: HIGH FOUND: RW2-F1 (High, held by its skeptic): the report says Opus 5.5 answered the advisor tool; the
   records show Fable 5.2 answered every call (verified by the session: the advisor_tool attachment 18:47:56 UTC model
   claude-fable-5-2; advisorModel fields; the advisor usage records). Lows: RW2-F2 (label consultation 6 also moment 5
   for step 6; add step 6's save 3f98c0b 22:04:19 to the list), RW2-F3 (#48's row: "and quoted whole in the merge
   commit's message"), RW2-F4 = R2-L2 (#52's waiting-on in STATUS and "Next:" in ROADMAP still list round 1: write
   "the verifier, the advisor agent's review of its final head, then his go"), RW2-F5 (R2-WORDS's notes held fragments
   of his words, pushed). R2-STATE: APPROVE, Lows: R2-L1 (the 2.1a follow-up should list plan S4's other lines, verified
   on the tree: ADR 0001 :603, :1100, :1301, :1771; ADR 0003 :1373; ADR 0004 :1245, :1251; ADR 0005 :92; ADR 0006 :1469,
   :1482; the register :72, :292), R2-L3 (step 4's departures, the STATUS cut and the archive lines beyond §6, were
   committed before consultation 4; restore "for step 4" in entry 3), R2-L4 ("Since round 1, every "done" save has come
   after its consultation."), R2-SELF (R2-STATE's guard refusal 22:28:31 UTC, a loop writing to a variable path; route:
   literal paths). Records checked (consultation 9's ask): the main session's 638 assistant turns all claude-opus-5-5,
   effort max (no stop condition); get_session: model and last served claude-opus-5-5; the agents: the advisor agent and
   docs-keeper Opus 5.5 at high; every reviewer and skeptic Opus 5.5 at max.

10. Plan revision 2d (D1 to D7, plan.md at 73db213), asked at 23:01:53 UTC, ruled at 23:02:43: APPROVE D1 to D6 (D6(a) as
   (a1): A4 plus a DECISIONS.md entry in E-HI's form, with three conditions), D7 agreed (no round 3). words.mjs: A4 (his
   status request, 22:27:05) in ITEMS; the compaction summary classified as the harness's; allItems sorted by id; the
   scan passes (28 entries, no problem).

11. Fix pass 2 applied (not yet committed at this save): the scripts first (words.mjs lastEntry; decisions.mjs E-STATUS,
   blocks 4, its negative control; report-appendix.mjs's intro, scan gate and cut-off; fixes.mjs's Q5 with D3), then
   report-body.md (`fp2-body.py`; the refusals bullet re-wrapped by hand), the report copied from it, its appendix
   written (cut-off 2026-11-13 23:34:50 UTC; 4 blocks, 2 body quotes: up by exactly one), `fixpass2.mjs` (D2, D3, D4's
   ROADMAP line, E-STATUS). Its first run left STATUS's #52 row unchanged: apply()'s inline kind took the new text, a
   suffix of the old, as present; fixed in the script and the row replaced by the same code run inline. Checks, all
   exit 0: words.mjs (30 entries, no problem), report-appendix.mjs --check, decisions.mjs --check --negative (4 entries,
   7 controls caught), pointers.mjs --check, fixes.mjs --check (11 edits; Appendix A sha equal; 10 quotes), quotes.mjs
   (15 spans; the one split line judged by hand, as before), scope.mjs (69 paths, none protected), the gate --no-smoke
   PASS, repo map OK. Round 2's notes: no pushed version but R2-WORDS's 461c82b and 7b5f53c holds a 30-character run of
   his words (`fp2-fragments-history.mjs`). The session's own guard refusal, 23:33:46 UTC (`git -C`; route: cd, git).

12. Fix pass 2 committed: 8e4fe7c863418eb034352dacbb3d998abc5b9bb1 (23:36:48 UTC; the notes as used: 1016823),
   pushed. The word-level diff 554face..8e4fe7c read on the commit (DECISIONS +6, ROADMAP 4, STATUS 2, the archive 11,
   the report 144 lines). CI on 8e4fe7c, all success: gate push 43528414752 (jobs gate 132239779212, smoke 132239779524),
   toolchain push 43528414776, gate pull_request 43528420177, toolchain pull_request 43528420127. The saver restarted:
   bw0zf0kg1 stopped, bpjfne3lu started (~23:38 UTC). A foreground `sleep 45` was blocked by the harness's own Bash rule
   (23:38:22 UTC; not the guard, the deny list or the classifier; route: CI read through the API instead). The
   verifier's brief: `verifier-brief.md`.

13. Fix pass 2 DONE: its follow-up a1d0317b5d6d6bef920e394c51c07ca869efb601 (23:48:08 UTC; the notes as used: fe2482c),
   pushed; every script check, 2.2's decisions-entries check and the gate passed before the commit; its word-level
   diff read; CI on a1d0317, all success: gate push 43529793262, toolchain push 43529793189, gate pull_request
   43529799958, toolchain pull_request 43529799786. Consultation 11 said no further consultation is needed for this
   save. The verifier's brief updated for a1d0317 (`verifier-brief.md`).

14. The verifier on a1d0317 (agent ad5b8a361fccd378c, 2026-11-14 ~00:31 UTC): FAIL. F-V1 (High): the report's clause on
   how Mustafa was told (D&D, :181-183) claims two post-compaction updates no record holds and leaves out the recorded
   one with the wrong premise (22:59:04, transcript line 2122, "where I first told you the advisor model"); sub-item
   (Low): the 22:48:47 message said "If true, ..."; the first correction in the records is 23:51:05 (line 2869), after
   a1d0317. F-V3 (Low): "round 2's 14 notes files" (9 are round 2's, 5 are round 1's skeptics' scripts) and the copies
   list incomplete (plan.md; advisor-plan-rev1.md :60 A4 whole). Everything else held (gate; 8 scripts; 11 Fable calls;
   every SHA, time and run id; 28 runs green; 10 findings; Appendix A; DECISIONS; cut-off: no message of his after
   23:47:48; #40 now bd7898f, updated 22:04:59Z). Its refusals: 00:12:07 (the permission system denied a compound command
   with `rm -rf`; route: Write tool, steps one by one, plain rm/rmdir of named files) and 00:16:46 (the guard: a loop
   writing to a variable path; route: literal paths). Not verified by it: agents' effort as run (the transcripts record
   the model only), consultations' content, 3 of 7 L2 logs, local smoke. `fp3-fragments-all.mjs`: 190 file versions, 29
   files with a run of 30+ characters of his words; 60+ only in plan.md (307), fillcount.mjs (251), report-body.md (96),
   advisor-plan-rev1.md (68, A4 whole), decisions.mjs (60); the rest are script anchors and needles, review briefs, short
   phrases in agents' notes and the checkpoint, and the base SHA and paths (the verifier's notes hold only those).

## Current step
Fix pass 3 (the verifier's F-V1 and F-V3): consult the advisor tool (moment 2: the clause failed a second time; moment 1
for the fix), ask the advisor agent for revision 2e, then write, check, commit, push, CI, and the verifier's re-run.

## Next steps
1. Scripts (consultation 10's items 1 to 6 block the commit): grep every "before" string of D2 to D4 in the notes'
   scripts and update their expectations (fixes.mjs's Q5 text gains D3); decisions.mjs: E-STATUS first in ENTRIES,
   blocks 4, a one-byte negative control on its quote; a new insert script for the entry (build refuses a written
   file); report-appendix.mjs: the intro (A1, A2 and A4; the advisor-model record; the compaction summary; the list of
   other entries checked against the scan's 28 rows: the 22:01 restart pair and the 21:54:48 reminder too), scan()
   before writing, refusing on any problem, the cut-off as the last scanned entry's time.
2. Report (report-body.md, then copied in): D1 (a), (c), (d) (with the clause: the session's first telling at ~22:56
   wrongly placed the claim in an opening message; its next message corrected that); D5's items; "What changed": four
   DECISIONS entries; round 2's table with 9 findings (RW2-F4 = R2-L2 counted once: one High, eight Lows) and the same
   count in Outcome and here; consultations 8 (22:15:20), 9 (22:49:58), 10 (23:03:56) and any later; the done saves
   from commit times (3f98c0b 22:04:19, 461c82b 22:19:55, ef1a7c1 22:52:48; 83c6ab6 20:47:12); every evidence row that
   quotes script output re-run and pasted; the body's quote count up by exactly one ("Fable 5.2 advises through the
   advisor tool." (A2) on one line); "For Mustafa" item 1 points at the correction too.
3. The battery, the notes saved last (scripts frozen), the commit, the word-level diff 554face..fix, CI read, a
   consultation before its "done" save.
4. The verifier (its brief: the whole report, every claim about what ran against a record; RW2-F1 against the records;
   the new entry's byte check and negative control; consultation 10's items 4 to 6), the saver restarted first (it
   ends ~23:52 UTC); then step 8 (the plan §10; consultation 10's items 7 to 10: the cut-off; a re-scan path after the
   deleting commit, from the last notes commit into the scratchpad; the advisor-call count equal to the numbered list;
   times after 00:00 UTC date-qualified, "Last updated" and ROADMAP's date from `date -u`, the report's file name kept);
   name D1(d)'s added clause to the advisor agent at its final-head review; the last consultation before the final
   commit, any later one in the PR body.

## Advisor tool consultations (his words of 2026-11-11; Fable 5.2 answered each, per the records: RW2-F1)
1. 2026-11-13 ~20:00 Amsterdam, moment 1, before committing to the docs-pass approach: is the approach right? Changed:
   saved first (checkpoint, plan, draft PR) before any tracked edit; every pointer quotes all four limits; the grep
   widened (ROADMAP:93 and STATUS:38 are sites; every hit listed as added or left); the decision cut by anchors (nested
   quotes); 2.2's appendix check replaced by a hash check plus the body-quote check; KIT-FREEZE-2 quoted, not
   paraphrased; Q1 to Q4 put to the advisor agent; agent briefs per CHECKPOINTS §4; the final head approved as the
   commit that deletes the checkpoint; #40's row only from what can be read.

2. 2026-11-13 ~20:57 Amsterdam, moment 5 for step 1 and moment 1 for step 2 (C9): is step 1 done, and is step 2's build
   right? Changed: `decisions.mjs` imports the entry title from `pointers.mjs` and checks DECISIONS.md holds it;
   `pointers.mjs --check` honours `--only`; `words.mjs` scans every user-type transcript entry and classifies it, and the
   appendix intro names the harness notices; DECISIONS.md's diff proven (only the 2026-11-10 header reflowed, its words
   equal old + one sentence; SPIKE_PLAN and ADR 0002 add lines only; negative controls on both E-TASK quotes); the saver
   recorded below; the gate run before the step-2 commit; a plain push right after it; for step 4, docs-keeper puts
   `{{P-2.3}}` in STATUS only and leaves ROADMAP's 2.1a pointer byte-intact.

3. 2026-11-13 ~21:15 Amsterdam, moment 5 for step 2 and moment 1 for step 3 (C9): is step 2 done, and is step 3's
   approach right? Changed: the M5 runs and the two pull_request runs read (c901187's pull_request run had 2.56.1, so
   L2 is worded as read, on the advisor agent's ruling, plan revision 2a); the header note no longer says "every other
   line" unchanged, and `fixes.mjs` gained a whole-file check; the hunk counts recorded here; for step 4, placeholders
   for his kit-freeze words as well and a quote-span check, the full pointer check after P7 and P8, the saver restarted
   before the review agents, and #40's head refreshed.

4. 2026-11-13 ~21:50 Amsterdam, moment 5 for step 4 and moment 1 for step 5 (C9): is step 4 done? Not yet. Changed:
   STATUS's #52 row and ROADMAP's #52 entry lose their REPO_MAP claim (step 5's commit); the archive's #48 row read whole
   (order, times and items against A3: right); a word-level diff of the archive 2f96f5c..83c6ab6 (only the #48 row, the
   block opening and the three "go" phrases); STATUS's base lines 21 and 46 and Routines checked by diff (unchanged);
   step 4's departures put to the advisor agent (revision 2b); the transcript re-scanned (15 entries; the intro names
   the new kinds); CI read on 83c6ab6 by SHA; the report discloses the retyped copy, the two logs only the session read,
   and that the transcript checks run only here; `review-r1.js` gains `model: 'opus'`, a required head SHA and a line
   against retyping his words.

5. 2026-11-13 ~22:00 Amsterdam, moment 5 for step 5 and moment 1 for step 6 (C9): is step 5 done, is the review
   launch right? Changed: CI on d59ca43 read by SHA (gate 43506267790, 43506274539; toolchain 43506267759,
   43506274417: green); `review-r1.js` brought up to revision 2b (P8 and the kit words as quotes; the ruling's checks for
   R-WORDS; R-STATE: REPO_MAP unchanged, the three "go" lines, the report's body; forward-looking lines named; skeptic
   notes prefixed by reviewer; scripts only with --check); the saver stopped and restarted; for the last content
   commit: #52's STATUS row and ROADMAP entry true for the ready state, #40's second read, the report's consultation
   line, "Last updated".

6. 2026-11-13 21:58:15 UTC (22:58 Amsterdam), LATE moment 5 for step 3 and moment 1 for the fix pass: is step 3 done,
   is the fix approach right? Changed: the C9 disclosure covers every gap (step 0 none; steps 1, 2, 4, 5 consulted after
   the "done" save; step 3 none until this one; the STATUS cut committed before consultation 4), never "met"; L-a's
   suggested text not used (it would state step 2.3's plan as fact); revision 2c asked of the advisor agent before any
   write; the Reviews section accounts for all 23 findings; the word-level diff read after the pass; what lands after
   the verifier kept tiny and listed for the head review; the refusal scan and words.mjs re-run before the last commit.
   Exact times of the advisor-tool calls (from the transcript): 18:59:07, 19:55:37, 20:13:33, 20:47:41, 20:57:48,
   21:58:15 UTC; the "step N done" saves: step 0 19:22:36, 1 19:55:30, 2 20:02:41, 3 20:27:37, 4 20:47:34, 5 20:57:42.

7. 2026-11-13 22:09:54 UTC (23:09 Amsterdam), moment 5 for round 1's fix pass (before its commit and "done" save) and
   moment 1 for round 2: not done yet. Changed: the word-level diff of docs/ d59ca43..tree read (only revision 2c's
   phrases); the report's "CI green" limited to the first four commits; revision 2c in the plan (with the F5 site
   rows); #52's own STATUS row and ROADMAP entry now name the archive's changes truly; the notes saved before the docs
   commit and no script changed after; round 2 as `review-r2.js` with `r2-` notes, briefed on what changed only (both
   SHAs, the 23 dispositions, the 2c rulings, what is pending by design, R-WORDS byte-checks E-TASK's header and the kit
   quote, R-STATE re-derives the consultation timeline and the refusals).

8. 2026-11-13 22:15:20 UTC (23:15 Amsterdam), moment 5 for round 1's fix pass (before its "done" save): done once two
   checks pass. Changed: the word-level diff re-read on the commit (d59ca43..554face) for STATUS and ROADMAP, and the 2.2
   report's 6 hunks counted; this checkpoint brought up to date in the same save; no docs or script edits while round 2
   runs; the saver's output checked; for after round 2: the 22:04:22 save in the report's timeline, 554face's run ids,
   consultations 8 onward, round 2's notes checked for fragments; round 2 the last reviewer round.

9. 2026-11-13 22:49:58 UTC, moment 5 for review round 2 (before its "done" save, ef1a7c1 at 22:52:48) and moment 1 for
   fix pass 2: is round 2 done, how to fix RW2-F1? Changed: the correction worded from the records only and made
   everywhere (report, appendix intro, checkpoint, PR body, end message); revision 2d asked before any write, round 3
   put to the advisor agent; Mustafa told in a top line; the main session's and the agents' models checked in the
   records.
10. 2026-11-13 23:03:56 UTC, moment 4 for D6 (LATE: after the session recommended (a1) to the advisor agent, after its
   ruling at 23:02:43 and after the notes-only words.mjs change; before any tracked edit), and moment 1 for writing fix
   pass 2: is (a1) right, what do my checks miss? Changed: the scripts' expectations first (fixes.mjs's Q5, decisions.mjs
   blocks 4, a new insert script, a negative control); the "opening line" claim struck from this checkpoint; round 2
   counted as 9 findings (RW2-F4 = R2-L2 once); times from commits; the cut-off from a scan-gated appendix write; the
   intro's list checked against the scan; the advisor-call count; midnight; D1(d)'s added clause; "For Mustafa" item 1.

11. 2026-11-13 23:39:10 UTC, moment 5 for round 2's fix pass (after 8e4fe7c, before any "done" save): NOT DONE YET; and
   moment 1 for the verifier. Changed: D1(d)'s clause "which its next message corrected" was false, so the clause was
   reworded, but still from recall (it claimed two post-compaction updates no record holds; the verifier failed it as
   F-V1); E2's 2.2 check run on this tree (OK, 13
   entries); the fragment check widened to all 14 round-2 notes files; entry 11, the sleep block, 8e4fe7c's runs and both
   SHAs added now; "Running work" brought up to date; the verifier's brief: the new head, a pending-by-design list, the
   GitHub tools, no words of his in its notes, 2.2's script copy, nothing posted on GitHub.

12. 2026-11-14 00:34:39 UTC, moment 2 for the clause on how Mustafa was told (its wording failed a second time), moment 1
   for fix pass 3 and moment 4 for F-V3's form, before the advisor agent was asked: the clause from three recorded lines
   only (1887, 2122, 2869), each read in full and described from its own text; nothing about messages the records do
   not hold or not yet sent; F-V3 narrowed (no counts; the certain copies named; "copies", not "retyped"); "asked for
   effort max"; the verifier's FAIL and scope note recorded as they happened; its two refusals; entries 9 and 11 fixed;
   the same verifier resumed with a narrow brief; a1d0317's commit-message claim withdrawn in the next commit's message.

## Refusals (guard, deny list, classifier) and the route taken
- The session: one, by the guard, 2026-11-13 23:33:46 UTC (`git -C <notes folder> diff`: the guard wants git run from
  the folder; route: `cd`, then `git diff`); none before it (the transcript scan). Agents, all by the guard: the advisor agent
  19:31:18 UTC (a loop writing to a variable path; route: a read-only python3 script); R-WORDS 21:14:45 (the same kind;
  route: literal paths); R-STATE 21:05:55 (a `node -e` line with comments; route: the script `rs-roadmap.mjs`); R-STATE
  21:17:54 (a `sed -i` on a scratch file; route: the Edit tool); R2-STATE 22:28:31 (a loop writing to a variable path;
  route: literal paths). docs-keeper, the four skeptics, R2-WORDS and round 2's skeptic: none.

## Open findings
- None.

## Key numbers
- main 4d21c1a; CI on it: gate run 43461710700 and toolchain run 43461710673, both success (the API, 2026-11-13).
- 2.2's `decisions-entries.mjs --check` on main: "OK: 13 entries (12 with quotes)", exit 0 (from a scratch copy in
  `.gate/fdn23-docs/s22/`).
- PR #40: draft, head 0066b635b65b4016c91af88825120312776a0a63, updated 2026-11-13 19:32:53 UTC, mergeable_state
  blocked (the API, read 20:27 UTC; it was b4d1f0a at 18:57 UTC and fe1460a at 19:17 UTC).

## Decisions taken
- The slug `foundation-2-3-docs`; the docs pass is its own PR from this branch (his words).

## Running work
- The verifier on a1d0317: agent ad5b8a361fccd378c (general-purpose, Opus 5.5, effort max), launched ~23:50 UTC;
  notes `verifier-1.md`; brief `verifier-brief.md`. Do not start another: wait for its notification.
- The saver: b3u81dqk2, started 2026-11-14 00:07 UTC with the longest timeout (2 hours, so it stops by ~02:07 UTC).
  bpjfne3lu (started 2026-11-13 ~23:38 UTC) was stopped by its background time limit; its last saves: 58f3fee
  (23:57:25 UTC) and 9ce5aaa ("autosave (terminated)", 2026-11-14 00:07:20 UTC).
- Ultracode was switched on by a system message (2026-11-14 ~00:07 UTC). The repo's limits still apply: at most 3
  agents at a time (CHECKPOINTS.md §2) and the review budget (no third round without an open Critical or High). So no
  extra review workflow on this PR; the verifier and the advisor agent's review of the final head are the gates.
- Done: review round 2 (wf_dcd47e10-70e, 2 of 2 reviewers and 1 of 1 skeptic returned); round 1 (wf_eb6cb5e5-3ce, 6 of 6);
  fix pass 1 (554face, CI green); fix pass 2 (8e4fe7c, CI green: 43528414752, 43528420177, 43528414776, 43528420127).
