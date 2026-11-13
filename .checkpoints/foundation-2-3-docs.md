# Checkpoint: foundation-2-3-docs
- **Goal:** step 2.3's first docs pass (Mustafa's words of 2026-11-13): his go for #48 and his 2026-11-13 decision in
  DECISIONS.md by script, with dated pointers; STATUS and ROADMAP true for main 4d21c1a; the 2.2 report's fixes M1, L1,
  L2, L3 and L5. Its own PR, docs only; ready, then stop for his go. After it merges (a new checkpoint): plan step 2.3.
  · plan: `.checkpoints/foundation-2-3-docs/plan.md`
- **Branch:** claude/sweet-mayer-grsukw (for cs/foundation-2-3-docs) · **Base:** main
  4d21c1a7a4d39613bffce5d5f9bbc92f2f88a300
- **Last update:** 2026-11-13 22:08 Amsterdam

## Steps done
0. The model check: Opus 5.5 (get_session: session_context.model and last_served_model are claude-opus-5-5; effort
   max). The advisor tool is on; the `/advisor fable` output said this conversation keeps Opus 5.5 behind it until
   `/clear` or `/compact`. Orientation: CLAUDE.md, CHECKPOINTS.md, STATUS, DECISIONS (2026-11-10 to 11-13 and
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

## Current step
Step 6 of the plan: review round 1 on d59ca43 (the workflow `review-r1.js`, args.head = d59ca43...). Notes done before step 2:
- `words.mjs` written and run read-only: A1 42 chars; A2 6190 chars, 36 lines; A3 1872 chars, 17 lines, the round trip
  OK, and identical byte for byte to the 2.2 chat record (event 96467900-..., created_at 2026-11-13T16:16:36.563252Z;
  the merge commit says 16:16:47 UTC: the report cites the commit's time and names the event's); A2-DECISION 562 chars,
  one line, inside A2, cut by its anchors.
- `report-appendix.mjs` written; tested on a scratch copy in `.gate/fdn23-docs/` (OK; a one-byte change in A3 caught).
- L2's log lines read (`l2-git-versions.md`): 2.56.0 on c901187 (15:34 UTC) and fd14855 (16:32 UTC); 2.56.1 on a5823cf
  (18:39 UTC).

- `pointers.mjs` tested on scratch trees (`.gate/fdn23-docs/tree2`): P1 to P5, P7 and P8 OK; idempotent; negative
  control (one word dropped from P2's quote) caught.

- Step 3 prepared: `fixes.mjs` (11 edits: the header note, M1, L1 with L2, L2 x2, L3 in the report; L5, Q3 x2, Q5 x2 in
  the archive; minimal reflow that stops where the original line breaks resume); tested on `.gate/fdn23-docs/tree4`: OK,
  the report's Appendix A equal to the base's, 2.2's body-quote check 10 quotes, 6 hunks in the report and 5 in the
  archive, every added non-table line at most 120 characters; a second write run refuses ("already written").

## Next steps
1. The advisor tool (C9: moment 5 for step 5, moment 1 for step 6); CI on d59ca43 read.
2. Stop the saver (TaskStop bofczdenq) and start a fresh one; then launch `review-r1.js` with args {head: d59ca43...};
   count what comes back; one fix pass; the advisor agent's review of what changed only if needed.
3. Step 7: the verifier (a general agent reading main's verifier.md, Opus 5.5 max) until PASS.
4. Step 8: the advisor tool; fold the checkpoint into the report; stop the saver; last checkpoint commit; the deleting
   commit writes {{NOTES-SHA}}; `fixes.mjs --check` before it; the advisor agent's review of that SHA (give it the
   notes' last SHA to run the scripts from); PR body; ready; stop.
2. (done earlier) The advisor tool (C9: moment 5 for step 2, moment 1 for step 3).
2. Step 3: `fixes.mjs` applies M1 (dated correction under "For Mustafa" step 3), L1, L2 (x3 in the report: :228 inside
   L1's sentence, :301, :345), L3 and the header note to the 2.2 report; L5 (:574) and Q3's two lines (:732, :834) in
   the archive; checks: Appendix A hash equal to 4d21c1a's, 2.2's body-quote check on the report's own blocks, the six
   places only (C6); one commit, pushed.
3. Then steps 4 to 8.

## Advisor tool consultations (his words of 2026-11-11; in this conversation Opus 5.5 answers the tool)
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

## Refusals (guard, deny list, classifier) and the route taken
- None so far.

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
- Saver: background task bofczdenq · output
  `/tmp/claude-0/-home-user-Shamble-Frontlines/7261f3d7-f651-586a-9c39-c052553b68a3/tasks/bofczdenq.output` · started
  19:22 UTC (20:22 Amsterdam) · stops by ~21:12 UTC (22:12 Amsterdam) · last check 19:57 UTC: ok ("watching ... every 10
  min for 110 min").
