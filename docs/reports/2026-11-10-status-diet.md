# Advisor report: status-diet (2026-11-10; branch `claude/busy-turing-48e4i9` for cs/status-diet, base main 2d22b69)
## Goal
Mustafa's queue of 2026-11-10, first item: STATUS-DIET with LANE-RULES (ACCOUNTS, his Models addition, the read-list
line). STATUS becomes a short dashboard again, his decisions sit word for word in one file, the rest is archived; the
lane, account and model rules go into CLAUDE.md; #46's merge is recorded. His words are the spec, not reworded.
## Outcome
DONE, waiting on his go (risk tier 2: CLAUDE.md and docs-keeper.md; `docs/DECISIONS.md` is a decision doc, so it merges
from a day session). One review round, PASS WITH NOTES twice, nothing Critical or High; one fix pass.
## Plan and approval
- Plan revision 1 (2f9d467). The advisor: APPROVE with conditions 1-12 and rulings Q1-Q5, no new revision; its addendum
  on his #46 go words: OK with (a)-(c). Met: 1 the script-made index, each code with who took it (its gaps: SD-L3);
  2 the live archive sections named in its intro and STATUS's pointers, and a reading for his go (Decisions needed);
  3 the Models sentence as worded; 4 docs-keeper at its smallest (M-1 restored its old duties); 5 DECISIONS.md's intro;
  6 CLAUDE.md:167 left, a pointer line in STATUS's Routines; 7 and 8 E5 and E3; 9 and 12 below; 10 E4; 11 E1 on the
  pure-move commit alone; addendum (a) Decisions needed, (b) the PX-1 pointer, (c) the chat-record check (E3).
## What changed (vs main 2d22b69; symbols +0 −0)
- `docs/STATUS.md`: 1233 lines (177,445 bytes) to 109: the phase, in flight (one line per PR: #40, #47, #48, #49), next,
  waiting on Mustafa, the routines (read today), follow-ups and open risks by id, where the old sections went.
- `docs/DECISIONS.md` (new, 498 lines): the queue of his words; his blocks, newest first, his #46 go on top; an index.
- `docs/STATUS_ARCHIVE.md` (new, 845 lines): the rest, under the old section names; live: Follow-ups, Open bugs.
- `CLAUDE.md`: the read list (DECISIONS.md, the archive; ART_DIRECTION the legacy look; his LOOK_V2 line); "Lanes and
  accounts" (his Lanes line and ACCOUNTS); Models (his Fable 5.2 exception and his addition, quoted).
- `.claude/agents/docs-keeper.md`: item 1 quotes his split and keeps its old duties; item 5 points to DECISIONS.md.
- REPO_MAP, START_HERE:20, TECHNICAL:240, ROADMAP. No guard, settings, CI or gate file.
## Evidence
- E1, the move (`check.mjs --rev 8ead8f1`: ALL PASS; the verifier's own count: 0 short): 1233 = 1218 moved lines
  (STATUS 3, DECISIONS 424, archive 791) + 15 structure lines (the title, 7 headings, 7 blank). At the head, 1217 of
  1218 are still byte for byte; the one changed is the kit-v3-5 row (1b030e3, #46's merge).
- E2, decision ids before and after, none lost: answered codes 65 and 65 (104 mentions kept); hyphenated codes 248 and
  254 (6 new: ranges in new text); codes that lead an entry 112 and 132 (20 new: the index); his dated lines 109, 109.
- E3, his words: 9 of 9 quotes in CLAUDE.md equal STATUS 702-726 and the kit-v3-5 report's Appendix A; the blockquote
  is exactly his six bullets. His #46 go: the merge message's lines equal his chat message byte for byte (event
  103f0452-db09-48fb-8529-fc794ef1f06c, 294 characters; the session read it, the verifier checked the saved copy).
- E4: gate PASS (`--no-smoke`), symbols +0 −0; CI gate and smoke green on every pushed head; repo map OK; kit tests on
  main and head: routine-guard 1107/1107, settings 127/127, checkpoint 27/27.
- E5, the rule-diff table (Appendix B): nine rows, one looser, by his own Models line.
- E6: #46 by git and the API (the verifier); the routines read with get_trigger, 2026-11-10 20:10 UTC (session only).
- His STATUS-DIET says "153 KB and 1,033 lines"; at 2d22b69 the file had 177,445 bytes and 1233 lines (it grew after he
  measured). His line is not edited.
- Not verified: smoke locally (CI runs it); whether the guard's three "ask" prompts reached him (L13: untested here).
## Reviews (one round on 37434d5; two agents, Opus 5.5 at high)
- security-auditor, PASS WITH NOTES: nothing looser than his words; every open security item still listed; the merge
  path not widened. N4 and N8 fixed; N1, N2, N3, N6 follow-ups (SD-N1 to SD-N6); N5 Decisions needed; N7 noted.
- The verifier, PASS WITH NOTES: E1-E6 reproduced on its own. M-1 (Medium, the same as N4) fixed in adfe108; L-6 and
  L-7 fixed; L-5 corrected below; L-3 a follow-up (SD-L3); L-1 and L-4 left (harmless); L-2 is this report.
## Decisions and deviations
- The advisor agent's rulings went to the archive, not DECISIONS.md (Q1). STATUS-DIET and LANE-RULES left "Queued" for
  the dated list (Q5); KIT-FREEZE-2, the unit walk and the Slice 1 note stay queued.
- The A3-notes line sits after the 2026-11-09/10 block, not before it (L-1). The main session wrote the ROADMAP entry
  (no docs-keeper pass: this change-set is the docs pass). `check.mjs`'s dated-line count tests the move, not later
  heads (the verifier's note); its own count covers the head.
## Risks
1. PR #40's branch edits STATUS and REPO_MAP and holds his answers of 2026-11-06 that main never had: it conflicts when
   it merges main, and its lane moves that block into DECISIONS.md. This lane never pushes there.
2. The foundation's 2.1b and 2.2 (#47, #48) touch STATUS, REPO_MAP and CLAUDE.md: their STATUS edits move to the new
   layout.
3. His words in the archive merge by path, as STATUS did (SD-N6).
## Proposed next step
His unit walk (UNITS.md §0 and LOOP_V2), in a fresh chat, from his words in DECISIONS.md, "Queued".
## Decisions needed
- **READINGS:** two readings of his words, for his reconfirmation. (1) STATUS-DIET's "the rest (done change-sets, old
  measurements, closed risks)" also holds the open follow-ups and open risks, as live archive sections listed by id in
  STATUS. (2) His #46 go, "The open security follow-ups stay in STATUS", is met by STATUS naming PX-1, W1, W2, W3 and
  the Windows-name limits with one line each, their text in the archive. Why: both set what a session reads first.
  Options: (a) accept both; (b) also keep the security follow-ups' full text in STATUS (about 16 more lines, still under
  120); (c) another home. Recommended: (a).
## For Mustafa
1. Confirm you saw and approved the three kit-edit prompts of 2026-11-10 (CLAUDE.md and docs-keeper.md about 20:20 UTC,
   docs-keeper.md again about 20:40 UTC). If you did not, say so and I revert them before the merge.
2. READINGS: (a), (b) or (c).
3. Your go to merge #49 at its final head once the advisor has approved it ("merge #49"; risk tier 2; no Brave check).
4. After the merge, tell the netcode chat (#40) and the foundation chats (#47, #48) that STATUS is split; #40 moves its
   block of your 2026-11-06 answers into DECISIONS.md.

## Appendix A: the line ranges (STATUS at 2d22b69, `git show 2d22b69:docs/STATUS.md`)
The script and its checks: `.checkpoints/status-diet/` (split.mjs, check.mjs, e3.mjs, the templates in tpl/,
followups.tsv), deleted in the last commit before the PR was marked ready; read them at that commit's parent
(`git show <parent>:.checkpoints/status-diet/<file>`; the parent is named in the PR).
- DECISIONS.md: "Queued" 674-690, 727-762, a pointer line, 763-774; "Decisions, newest first": his go for #46 (lines
  5-7 of 2d22b69's message), a header line, 691-726, 200-239, 198-199, 240-272, 281-291, 292-316, 317-339, 340-370,
  371-391, 392-400, 401-420, 775-787, 788-799, 813-819, 820-827, 828-836, 837-846, 880-884, 885-887, 888-907, 908-911,
  912-925, 926-928; then the index.
- STATUS.md: 847-849 (ABL-PREDICT, in "Waiting on Mustafa"); the rest is new text.
- STATUS_ARCHIVE.md: 2-27 (the old header), 30-139 (Phase), 142-185 (In flight), 188-195 (Targets), 421-673 and 862-879
  (Follow-ups), 273-280, 800-812, 850-861 and 929-946 (Decisions pending: the rest), 949-1008 (Routines), 1011-1217
  (Open bugs and risks), 1220-1233 (Health).
- Not moved (structure only): 1, the headings 29, 141, 187, 197, 948, 1010, 1219, the blank lines 28, 140, 186, 196,
  947, 1009, 1218.

## Appendix B: E5, the rule-diff table (main 2d22b69 against the head; line numbers at the head)
| # | File and place | Before | After | Verdict |
|---|---|---|---|---|
| 1 | CLAUDE.md:27-29, read list item 1 | `docs/STATUS.md` (where things stand) | adds DECISIONS.md and the archive | same (pointers; his "CLAUDE.md's read list to match") |
| 2 | CLAUDE.md:32, read list item 3 | ART_DIRECTION (locked look, authoritative) | (the legacy build's locked look) | same meaning as his read-list line |
| 3 | CLAUDE.md:37-38 | (none) | his read-list line, quoted: "for v2 work, LOOK_V2.md is the look brief and wins; ART_DIRECTION.md describes the legacy game only." | his words |
| 4 | CLAUDE.md:178-179, Models | "No other model." | "No other model, except Fable 5.2 as the session's advisor through Claude Code's advisor tool, on Mustafa's personal account (his words below)." | looser, by his words: "on my personal account, Fable 5.2 may be the session's advisor through Claude Code's advisor tool. The main model and every agent stay Opus 5.5." |
| 5 | CLAUDE.md:180-183 | (none) | his Models line and his addition, quoted | his words; the addition is stricter ("it is an extra. The recorded plan review by the advisor agent stays required.") |
| 6 | CLAUDE.md:133-146, "Lanes and accounts" | (none) | his Lanes line and ACCOUNTS' five bullets, one blockquote | his words (new rules) |
| 7 | docs-keeper.md:13-21, item 1 | STATUS: phase, in-flight work, recently landed, pending decisions, open risks, the health table; "keep it one screen long" | his STATUS-DIET split quoted; the same duties, each mapped to its new file | same duties; "at most 120 lines" replaces "one screen long", by his cap |
| 8 | docs-keeper.md:30-31, item 5 | update the "Decisions pending" lines in STATUS | record his words in DECISIONS.md and update STATUS's Phase line | same, re-pointed |
| 9 | (unchanged on purpose) | CLAUDE.md:85, :107, :167, :173, :200; advisor.md; the skills; the guard | | the advisor's condition 6 |
