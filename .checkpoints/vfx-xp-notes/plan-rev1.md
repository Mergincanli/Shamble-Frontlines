# vfx-xp-notes plan, revision 1 (2026-11-07): the VFX editor note and private-match XP

Branch `claude/sleepy-tesla-ofsxsr` stands in for `cs/vfx-xp-notes` (draft PR #43). The worktree is from main 4c6db5b
(PR #42, B.2, merged). Docs only: no code, so symbols +0 −0. Not player-facing. Not kit, so no kit question arises.

## 1. Goal
Record Mustafa's two items of 2026-11-06, both "For the next docs pass" (VFX-ORDER (a): this pass comes next, then A4):
- **The VFX editor** in `docs/EDITOR_V2.md`, as a future tool after Editor v2's map features: his ten decided
  principles, word for word; the design comes later.
- **Progression in private (rented) matches** in VISION_V2's community-servers note (§5): his `[DECIDED]` XP rule and
  the Advisor's two suggestions, which he marked open.
- Also: the docs after PR #42's merge, and the advisor's four Low notes on B.2's final head (PR #42 comment
  6698888516).

## 2. The sources (word for word in `mustafa-words.md`)
- The VFX item: user event 0f5f8a45…, 2026-11-06T23:27:12Z (saved first at a79fef1, queued in STATUS:498-520).
- The XP item: user event 634a1bb3…, 2026-11-06T23:50:53Z.
- VFX-ORDER (a), in his "merge #42" message.

## 3. The edits to `docs/EDITOR_V2.md` (113 lines today)
1. **A new §7 at the end**, so §1-§6 keep their numbers (VFX-PLACE, §8):
   ```
   ## 7. A future tool: the VFX editor (after the map features)
   `[LATER]` **A VFX editor**, as a future tool, after Editor v2's map features (Mustafa, 2026-11-06). Decided
   principles, design later:
   - `[DECIDED]` Essential: the game's punch depends on it. Inspired by Unreal's Niagara (emitters with a stack of
     modules: spawn, velocity, colour over life, size over life, gravity, collision), but without its clutter.
   - … his other nine lines, each whole, in his order, each `[DECIDED]` …
   ```
   The one change to his words: his opening "record a VFX editor in EDITOR_V2" is an instruction to us. Only "a VFX
   editor", "as a future tool, after Editor v2's map features" and "Decided principles, design later" are kept, as B.1
   dropped "EDITOR_V2: add".
2. **A dated header line** after line 6: "Updated on 2026-11-06 with Mustafa's VFX editor, a future tool after the map
   features: §7. The scope above stays the map editor." One new line, so the lines after it move by one:
   STATUS:656's `docs/EDITOR_V2.md:93` and `:94` become :94 and :95 (E4 checks every EDITOR_V2 line citation).
3. **Not touched:** §3's `[LATER]` line (interiors, sculpting, prefab library v2). The VFX editor is its own section,
   not a map feature.

## 4. The edits to `docs/VISION_V2.md` §5 (the community-servers note)
After the trust rule (XP-PLACE, §8):
```
  - On progression in private (rented) matches (Mustafa, 2026-11-06):
    - `[DECIDED]` With bots in the match: 25 % of normal XP; humans only: 100 %.
    - `[OPEN]` Open, Advisor's suggestions: (1) weigh XP per action (kills on bots give little), so public matches
      with bot fillers aren't punished; (2) against friends trading easy wins: full XP only with a minimum number of
      humans per team, plus a daily XP cap on private matches.
```
- His words stay whole: he wrote the `[DECIDED]` himself, and "Open," stays beside the tag, as B.1 kept "Decided,"
  beside `[DECIDED]`.
- "XP" is his word. LOOP_V2 §8 has an account level (the career rank) and Service points; it has no "XP". B.2 maps
  nothing: a STATUS follow-up asks which one XP means (XP-TERM, §8).
- The line about public matches with bot fillers stays where he put it, in the community-servers note.
- About 5 new lines, so §6 and §7 move. The STATUS follow-up on ADR 0005's VISION_V2 line ranges gets this pass's
  numbers (E4 computes them).

## 5. The other docs
- **The register** (REG, §8). Each row gets one "Mustafa (2026-11-06) bears on it" clause, in his words, and no ADR is
  edited:
  - **R2-ROOM-OQ7** (README:183, "whether private matches count for Service points and telemetry"): his XP rule;
    telemetry stays open;
  - **OQ-0005-CD7** (README:122, "What is the Feel Lab tool?"): his "Whole impact moments" and "Test in context".
- **LOOK_V2** (LOOK, §8). His "Comic flavour" and "A VFX style guide comes first" meet LOOK_V2 §7 (Juice): "Comic impact
  touches for big moments, used sparingly: halftone flashes, ink 'line boil'" (`[DEFAULT]`). The look brief is
  Mustafa's and the Advisor's, so this pass decides nothing about it. Recommended: a STATUS follow-up for the brief's
  owners, and no edit to LOOK_V2.
- **STATUS, PR #42 merged** (4c6db5b; `merged_at` read from the API at build time; on Mustafa's go after the advisor's
  APPROVE of e223ecc, comment 6698888516; CI green on main). The edits:
  - the header, Phase and B.2's row;
  - this pass's row;
  - the queue: this pass in flight, then A4;
  - the VFX queue entry (STATUS:498-520) removed with a pointer, since EDITOR_V2 §7 holds it;
  - B.2's follow-up "Not decided by the note" updated: his XP rule now covers private (rented) matches;
  - the advisor's Low note 1 (STATUS:500's "Next after B.2" as the session's reading) goes with the entry;
  - follow-ups: XP-TERM; the VFX reference study and style guide come first (his principle), a research task for
    later; the LOOK note; the match record must tell bots-or-humans and private matches apart for the XP rule (ADR
    0006 part 12 already tags bot and private matches: the drafts' next revision checks it).
- **CI:** one STATUS line under Open bugs and risks for infra-engineer. #42's pull_request smoke job stalled twice for
  about 7 minutes installing Chromium's system libraries (apt), before any test; the push run did the same step in 11 s.
- **The B.2 report:** a dated "Corrections after the merge" note with the advisor's Low notes 2-4:
  - #40's stale lines, with its :3, at #40's SHA;
  - For Mustafa 1(b): "at the level of principle only";
  - the check script's FAIL (1) on the dated Appendix B title.
- **ROADMAP:** B.2's entry merged, this pass's entry, the queue text.

## 6. Evidence
- **E1:** his words byte for byte: his ten VFX lines and the kept opening pieces, his two XP lines, each once.
  - Checked against `mustafa-words.md` and the chat events.
  - In STATUS, the VFX queue entry is gone and its pointer is there.
- **E2:** nothing else changed. In EDITOR_V2: §7, the header line. In VISION_V2: the XP block only. All headings
  unchanged.
- **E3:** tag counts per file, before and after, written down first:
  - EDITOR_V2: `[DECIDED]` +10, `[LATER]` +1;
  - VISION_V2: `[DECIDED]` +1, `[OPEN]` +1.
- **E4:** every "§N" citation of both files still resolves.
  - Every line citation into EDITOR_V2 or VISION_V2 is listed, by line and by range.
  - Each one this pass moves is fixed (STATUS) or recorded as a follow-up (ADR 0005, R1's research record).
- **E5:** #42's merge facts from the API; every changed line number checked.
- **E6:** `gate.mjs --no-smoke` and `repo-map-check.mjs`.
- **E7:** scope: EDITOR_V2, VISION_V2, `docs/adr/README.md` (two clauses), STATUS, ROADMAP, the B.2 report, this
  pass's report, and the notes.

## 7. Reviews (one round, at most 3 agents at a time, Opus 5.5 at high)
- **the verifier:** E1-E7;
- **game-designer:** the XP rule and the Advisor's suggestions against LOOP_V2 §8, with nothing decided that he left
  open; and the VFX "impact moments" against the feel kit (LOOP_V2 §7);
- **art-director:** the VFX principles against LOOK_V2 §7 and ART_DIRECTION, with nothing in the look decided by
  placement or label;
- **graphics-engineer**, only if the advisor asks: the cost, quality-tier and same-renderer principles against ADR
  0003.
- Review budget: one fix pass; skeptics only for High or above; the rest are STATUS follow-ups.

## 8. Decisions for the advisor
- **VFX-PLACE:** (a) a new EDITOR_V2 §7 at the end (recommended: his "after Editor v2's map features"; §1-§6 keep
  their numbers); (b) a `[LATER]` item in §3; (c) a separate document (no: he said EDITOR_V2).
- **VFX-TAG:** (a) `[LATER]` on the tool, `[DECIDED]` on each of his ten principles (recommended, as in B.2's note);
  (b) `[DECIDED]` `[LATER]` on the whole section.
- **XP-PLACE:** (a) a sub-bullet block after the trust rule (recommended); (b) at the note's end.
- **XP-TERM:** (a) keep "XP" as he wrote it, with a STATUS follow-up asking which LOOP_V2 §8 measure it is
  (recommended); (b) ask Mustafa now, before the build.
- **REG:** (a) the two "bears on it" clauses (R2-ROOM-OQ7, OQ-0005-CD7) (recommended); (b) none.
- **LOOK:** (a) a STATUS follow-up for the look brief's owners, no LOOK_V2 edit (recommended); (b) a pointer in LOOK_V2
  §7.
- **HEADER:** (a) the dated EDITOR_V2 line, with STATUS:656's two citations moved (recommended); (b) none.

## 9. Mustafa's check
His go to merge. He confirms three things:
- his words and marks;
- XP-TERM's reading;
- that the VFX section sits where he meant ("after Editor v2's map features").

No Brave check.

## 10. Risks
1. **The look:** the comic-flavour line could read as a look decision beyond LOOK_V2 §7's `[DEFAULT]`. Mitigation:
   his words only, LOOK (a), and art-director's review.
2. **"XP" is not a defined term.** Mitigation: XP-TERM (a).
3. **The VFX tool as an engine need.** It is `[LATER]` in EDITOR_V2, a need source (README:294-298), and J3 gives no
   feature to `[LATER]` items. The drafts' next revision decides any need when the tool is designed.
4. **PR #40 still edits STATUS.** B.2 is merged. This pass keeps its STATUS edits to its own lines.
