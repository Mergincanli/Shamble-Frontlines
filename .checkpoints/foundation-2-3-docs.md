# Checkpoint: foundation-2-3-docs
- **Goal:** step 2.3's first docs pass (Mustafa's words of 2026-11-13): his go for #48 and his 2026-11-13 decision in
  DECISIONS.md by script, with dated pointers; STATUS and ROADMAP true for main 4d21c1a; the 2.2 report's fixes M1, L1,
  L2, L3 and L5. Its own PR, docs only; ready, then stop for his go. After it merges (a new checkpoint): plan step 2.3.
  · plan: `.checkpoints/foundation-2-3-docs/plan.md`
- **Branch:** claude/sweet-mayer-grsukw (for cs/foundation-2-3-docs) · **Base:** main
  4d21c1a7a4d39613bffce5d5f9bbc92f2f88a300
- **Last update:** 2026-11-13 21:20 Amsterdam

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

## Current step
Step 3 of the plan: the five report fixes. Notes done before step 2 (no tracked file):
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
1. The advisor tool (C9: moment 5 for step 2, moment 1 for step 3).
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

## Refusals (guard, deny list, classifier) and the route taken
- None so far.

## Open findings
- None.

## Key numbers
- main 4d21c1a; CI on it: gate run 43461710700 and toolchain run 43461710673, both success (the API, 2026-11-13).
- 2.2's `decisions-entries.mjs --check` on main: "OK: 13 entries (12 with quotes)", exit 0 (from a scratch copy in
  `.gate/fdn23-docs/s22/`).
- PR #40: draft, head b4d1f0a, updated 2026-11-13 18:57 UTC (the API).

## Decisions taken
- The slug `foundation-2-3-docs`; the docs pass is its own PR from this branch (his words).

## Running work
- Saver: background task bofczdenq · output
  `/tmp/claude-0/-home-user-Shamble-Frontlines/7261f3d7-f651-586a-9c39-c052553b68a3/tasks/bofczdenq.output` · started
  19:22 UTC (20:22 Amsterdam) · stops by ~21:12 UTC (22:12 Amsterdam) · last check 19:57 UTC: ok ("watching ... every 10
  min for 110 min").
