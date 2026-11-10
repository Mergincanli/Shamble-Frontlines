# status-diet: plan revision 1 (STATUS-DIET with LANE-RULES, ACCOUNTS and the Models addition)
Branch `claude/busy-turing-48e4i9` (stands in for `cs/status-diet`; standing permission's cloud stand-in), base main
2d22b69. His words (the spec, not reworded): main `docs/STATUS.md:680-726` (KIT-FREEZE-2, STATUS-DIET, LANE-RULES,
the Models addition, ACCOUNTS) and `docs/reports/2026-11-10-kit-v3-5.md` Appendix A. Facts: `facts.md` (this folder).

## 1. Goal, scope, tier
- STATUS-DIET: split main's STATUS (1233 lines, 177 KB) into `docs/STATUS.md` (at most 120 lines: the phase, in flight
  one line per PR, next, waiting on Mustafa, the routines' state, pointers), `docs/DECISIONS.md` (his decisions, word
  for word, newest first) and `docs/STATUS_ARCHIVE.md` (the rest). Text moves by a script by line ranges, never
  retyped; no sentence of his reworded or dropped; the report shows decision-id counts before and after. REPO_MAP and
  CLAUDE.md's read list follow.
- LANE-RULES (with ACCOUNTS and his Models addition): CLAUDE.md, his words quoted word for word.
- Record #46's merge (2d22b69, 2026-11-10 16:05:52 UTC, by merge commit, parents 44446a5 and 0573db3, on his go after
  the advisor's APPROVE WITH NOTES of 0573db3, comment 6772168492) and the routines' state as read today.
- Out of scope (KIT-FREEZE-2): no guard, settings, CI or gate file; no new rule; no decision doc other than the new
  DECISIONS.md (the ADRs, LOOP_V2, ENGINE_OVERVIEW keep their STATUS citations; a pointer table resolves them).
- Risk tier 2 (CLAUDE.md, `.claude/agents/docs-keeper.md`); DECISIONS.md is on the guard's decision-doc list, so the
  PR merges from a day session on his go. No game code, symbols +0 −0, no Brave check.

## 2. The split (script `split.mjs`, manifest in it; source `git show 2d22b69:docs/STATUS.md`)
- DECISIONS.md, whole blocks whose header says Mustafa took or answered them, or the Advisor (relayed by him; ACCOUNTS:
  "once I paste one, it is my instruction"), newest first by the header's date; same-day blocks keep STATUS's order,
  so "above" and "below" between them still read right:
  "## Queued, in Mustafa's words (still to carry out)": 674-690, 727-762, a new pointer line (STATUS-DIET and
  LANE-RULES carried out by this PR; where his words stand), 763-774.
  "## Decisions, newest first": a new header line for the carried-out pair, then 691-726; 198-199 (the A3-notes
  note); 200-239; 240-272; 281-291; 292-316; 317-339; 340-370; 371-391; 392-400; 401-420; 775-787; 788-799; 813-819;
  820-827; 828-836; 837-846; 880-884; 885-887; 888-907; 908-911; 912-925; 926-928.
  "## Also recorded elsewhere": a script-made index (not his words) of every answered code `CODE (x)` that stands only
  in the archive or STATUS, with its section.
- STATUS.md: new dashboard text (section 3) plus 847-849 moved in (ABL-PREDICT, waiting on him).
- STATUS_ARCHIVE.md, under headings named after the old sections (so "STATUS, Routines" resolves to the archive's
  "Routines"): 2-27 (STATUS's header at 2d22b69); 30-139 (Phase); 142-185 (In flight; every row done once #46 is
  recorded); 188-195 (Targets); 421-673 and 862-879 (Follow-ups, with KIT-ALIGN and DOCS-ALIGN; open, kept current);
  273-280, 800-812 (rulings by the advisor agent), 850-861 (carry-forwards), 929-946 (old ADR and design lines);
  949-1008 (Routines); 1011-1217 (Open bugs and risks; open, kept current); 1220-1233 (Health).
- Not moved (structure only, no word of his): line 1 (title), the H2 headings 29, 141, 187, 197, 948, 1010, 1219, and
  the blank lines 28, 140, 186, 196, 947, 1009, 1218 (786 moves inside 775-787).
- Each new file opens with a short new intro: what it holds, "moved by script from STATUS at 2d22b69 (STATUS-DIET)",
  the old section names it keeps, and that the archive's old text is as it stood then (only the done table, the
  follow-ups and the open risks are kept current).

## 3. The new STATUS.md (at most 120 lines; drafted with the facts in facts.md)
Title and purpose (at most 120 lines; his decisions in DECISIONS.md; the rest in the archive) · **Phase** (v2 phase 0;
the drafts signed off as the basis for the spikes, Proposed; last merged #46; KIT-FREEZE-2 in force until the
foundation's gate step, exceptions; legacy frozen; main protected) · **In flight** (one row per PR: #40, #47, this PR)
· **Next** (his order: this PR, then the unit walk, then the Slice 1 note, each pointing at DECISIONS.md, "Queued";
v2: the netcode proof test, the foundation 2.1a then 2.1b, wave 1 one heavy spike at a time) · **Waiting on Mustafa**
(this PR's go; #47's go after this merge; the proof test's items; ABL-PREDICT, moved in; legacy: the Windows run and
the old Brave checks; deleting the old night-shift routine) · **Routines** (read with get_trigger today: the night
shift daily at about 06:00 Amsterdam since 2026-11-07, the PR Advisor at 07:45, 13:45 and 19:45, re-enabled 16:07 UTC
after #46; effort; the history in the archive) · **Follow-ups** (A4's "STATUS follow-up" and his 2026-11-04 "in
STATUS's follow-ups": one line per group, by id, the text in the archive's Follow-ups) · **Open risks** (pointers) ·
**Where the old sections went** (old section and line range at 2d22b69 to its new file and heading).

## 4. Kit edits (each one Write, one ask of Mustafa; built in `.gate/status-diet/` first, diffed)
CLAUDE.md:
- Read list item 1: `docs/STATUS.md` (where things stand) gains "(at most 120 lines; Mustafa's decisions, word for
  word: `docs/DECISIONS.md`; the rest: `docs/STATUS_ARCHIVE.md`)".
- Read list item 3: `docs/ART_DIRECTION.md` (locked look, authoritative) → (the legacy build's locked look); and after
  the v2 list: (Mustafa's LANE-RULES, 2026-11-10: "for v2 work, LOOK_V2.md is the look brief and wins; ART_DIRECTION.md
  describes the legacy game only.")
- "Models and effort": "No other model." → "No other model, except the advisor tool (below)." plus a bullet quoting
  his Models line and his addition word for word: "on my personal account, Fable 5.2 may be the session's advisor
  through Claude Code's advisor tool. The main model and every agent stay Opus 5.5." "because the advisor tool's advice
  cannot be read back, it is an extra. The recorded plan review by the advisor agent stays required."
- A new section "## Lanes and accounts" before "## Reporting to Mustafa": his Lanes bullet and ACCOUNTS' five bullets,
  quoted word for word as his (a blockquote, his first person kept).
- :167 "(EFFORT-SOURCE; the evidence is in STATUS, "Routines")" → "... in STATUS_ARCHIVE, "Routines")" (a re-point;
  the advisor may prefer the STATUS pointer line instead: Q4).
`.claude/agents/docs-keeper.md` (so the diet holds; his words reused): item 1 → STATUS at most 120 lines with his six
parts; his decisions into DECISIONS.md word for word, newest first; the rest into the archive; moves by line ranges
with a script. Item 5: "update the "Decisions pending" lines in STATUS" → "record his words in DECISIONS.md and update
STATUS's Phase line".
Not edited (follow-ups, after the gate step): advisor.md (:17 sources, :286 "(STATUS, limit L4)"), the changeset skill,
netcode-architect.md:24; the decision docs' STATUS citations (for their next revision).
Docs (not kit): REPO_MAP rows for the three files and the docs-keeper row; ROADMAP's entry; START_HERE:20 and
TECHNICAL:240 name DECISIONS.md where they list STATUS.

## 5. Evidence
- E1, the move: every line of 2d22b69's STATUS lands byte for byte in exactly one destination, in the manifest's
  order, except the 15 structure lines listed (`check.mjs`, run by the verifier too).
- E2, decision ids: unique codes and mentions before (2d22b69's STATUS) and after (the union of the three files) for
  (a) answered codes `CODE (x)` and (b) every hyphenated capital code; no code lost; per-file counts in the report.
- E3, his words: every quote of his LANE-RULES, Models addition and ACCOUNTS in CLAUDE.md equals STATUS's quote and
  Appendix A after joining wrapped lines; STATUS.md at most 120 lines.
- E4: `node tools/gate.mjs origin/main --no-smoke` PASS, symbols +0 −0; CI gate and smoke green; repo-map check OK;
  kit tests with main's counts (routine-guard 1107, settings 127, checkpoint 27).
- E5, the rule-diff table: every changed rule sentence (CLAUDE.md, docs-keeper.md), before, after, looser/stricter/
  same, with his words for anything looser (the Models exception is his).
- E6: the #46 and routine facts in STATUS equal the API reads.

## 6. Steps, agents, models
1. Plan review (advisor agent, Opus 5.5 high). 2. Pure-move commit (split + check). 3. Record-#46 commit (archive row,
STATUS, ROADMAP). 4. Docs (REPO_MAP, START_HERE, TECHNICAL). 5. Kit writes (CLAUDE.md, docs-keeper.md; one ask each).
6. Gate, checks. 7. One review round on the final diff: security-auditor (tier 2), the verifier (E1-E6), at most 3
agents, Opus 5.5 high. 8. Fix pass. 9. Report, checkpoint folded in and deleted, PR ready. 10. The advisor's
final-head review; his go. Checkpoint saved after each step; the saver runs while agents run.

## 7. Questions for the advisor
- Q1: the advisor agent's rulings (273-280, 800-812) go to the archive, not DECISIONS.md ("my decisions").
- Q2: follow-up details in the archive, with STATUS's Follow-ups pointer section, keep A4's "a STATUS follow-up" and his
  2026-11-04 "Record them in the repo, in STATUS's follow-ups" true without a rule edit.
- Q3: the docs-keeper.md edit is inside STATUS-DIET's freeze exception (the smallest edit that keeps the diet true).
- Q4: CLAUDE.md:167 re-point, or leave it and keep a pointer line in STATUS's Routines.
- Q5: STATUS-DIET and LANE-RULES leave the queue in this PR (moved, not dropped).
- Timing: within a day if his asks come promptly; PR #47 waits for this merge.
DONE (plan revision 1)
