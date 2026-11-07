# Advisor report: vfx-xp-notes (2026-11-07, branch `claude/sleepy-tesla-ofsxsr` for cs/vfx-xp-notes, base main 4c6db5b)
## Goal
Mustafa's two items of 2026-11-06, both "For the next docs pass" (next by his VFX-ORDER (a)): a VFX editor in
`docs/EDITOR_V2.md` as a future tool, and progression in private (rented) matches in VISION_V2's community-servers
note. Docs only; also the docs after PR #42 and the advisor's four Low notes on B.2's final head.
## Outcome
DONE: his words stand in EDITOR_V2 §7 and VISION_V2 §5; the review round found nothing Critical or High, and its one
fix pass is in. Two questions go to him below (XP-TERM, LOOK); nothing waits on them.
## Plan and approval
- Plan revision 1; the advisor's **APPROVE WITH CONDITIONS (10)**, all met: C1 E1 (his ten VFX lines, the kept opening
  pieces, his XP lines; his plain "[DECIDED]" set as the tag, "Open," kept beside `[OPEN]`); C2 the XP-TERM list,
  mapping none; C3 and C5 the two questions below; C4 the register clauses (four, and a fifth from the review);
  C6 EDITOR_V2's 112 lines, the header at :7; C7 J3's need rule; C8 B.2's follow-up "bears on it", no record field
  added; C9 #40's lines at 0588a17; C10 the CI facts.
- Rulings: VFX-PLACE (a) §7; VFX-TAG (a); XP-PLACE (a); XP-TERM (a); REG (a), widened; LOOK (a); HEADER (a).
## What changed (vs main 4c6db5b; symbols +0 −0, as planned: docs only)
- `docs/EDITOR_V2.md`: a dated header line (:7) and §7, "A future tool: the VFX editor (after the map features)":
  the tool `[LATER]`, his ten principles word for word, each `[DECIDED]`.
- `docs/VISION_V2.md` §5 (:71-75): under the trust rule, his XP rule for private (rented) matches (`[DECIDED]`) and
  the Advisor's two suggestions (`[OPEN]`); the dated header line names it (rewritten in place).
- `docs/adr/README.md`: "bears on it" clauses on OQ-0003-15 (:103), OQ-0003-16 (:104), OQ-0005-CD7 (:122, not "is
  the Feel Lab"), OQ-0006-21 (:160) and R2-ROOM-OQ7 (:183). No ADR draft edited.
- STATUS and ROADMAP: #42 merged; this pass; the VFX queue entry removed (his words stand in EDITOR_V2 §7);
  follow-ups XP-TERM, LOOK, the need rule, the VFX tool's design session, stale citations; the #42 CI stall as facts.
  The B.2 report: corrections after the merge.
## Evidence
- E1 (`e-checks.py`): PASS, 30 checks; the verifier also compared `mustafa-words.md` with both chat events (byte for
  byte) and checked §7's ten bullets in his order. Appendix A equals the words file. E2: EDITOR_V2 gains only :7 and
  §7, VISION_V2 only the XP block and its header line. E3: EDITOR_V2 `[DECIDED]` 11 to 21, `[LATER]` 2 to 3;
  VISION_V2 `[DECIDED]` 12 to 13, `[OPEN]` 3 to 4; the rest unchanged.
- E4: STATUS:660's EDITOR_V2 citations moved to :94 and :95; ADR 0005's VISION_V2 ranges and R1's two older
  citations are follow-ups with true numbers. The check script misses two citation forms ("(lines A–B)" and
  "EDITOR_V2 :N") and a line split across two bullets; the verifier's wider checks found only R1's :3631 (a
  follow-up). E5: the merge and CI facts checked against git and the API (the verifier). E6: `gate.mjs --no-smoke`
  PASS, symbols +0 −0; repo map OK; CI green on d5e2c62 and e967196. E7: only the six docs, this report and the notes.
- **Not verified:** smoke locally (CI runs it).
## Reviews
- The verifier, PASS WITH NOTES: V-1 to V-5, Low (R1's :3631 cite, a count, :3's verdict wording, the script's limits,
  a background line): fixed, V-4 here.
- game-designer, PASS WITH NOTES: GD-1, Medium (the fact that tells XP-TERM's options apart): in STATUS and below;
  GD-2 to GD-6, Low (the rule's edges, the tool's design questions, LOOP_V2:235, the header line): follow-ups or fixed.
- art-director, PASS WITH NOTES: AD-1, Medium ("onomatopoeia" bears on OQ-0003-15): a fifth register clause; AD-2 to
  AD-6, Low: the LOOK question sharpened, follow-ups; AD-3 (a pointer beside his line) not taken: the LOOK question
  covers it. Nothing Critical or High: no skeptics, no second round.
## Decisions and deviations
- His words: his opening "record a VFX editor in EDITOR_V2" is an instruction, so only "A VFX editor", "as a future
  tool, after Editor v2's map features" and "Decided principles, design later" are kept (two capital letters
  changed: "A", and "On" in the XP lead-in). Everything else is his, word for word.
- docs-keeper folded ROADMAP's oldest recent entry into Build history (its 8-12 rule).
## Risks
1. "XP" stays undefined until he answers XP-TERM; the drafts' account step waits on it.
2. His comic-flavour list and LOOK_V2 §7's "used sparingly": the LOOK question, for him and the Advisor.
3. PR #40 still edits STATUS and conflicts with main; whichever merges second resolves it.
## Proposed next step
A4, the review policy (kit, plan first): the kit freeze has lifted (his UNDER-WAY (a), as the netcode session records).
## Decisions needed
- **XP-TERM:** which "XP" does his rule cut (25 % of normal XP with bots in a private match, 100 % humans only)? Why:
  "XP" already names other things, and the rule reads differently for each; Service points come only from a share
  of the team's uplink tech, never from kills, so the Advisor's open suggestion (1) fits only an XP earned per
  action. Options: (a) all account progression (the account level and Service points); (b) Service points only;
  (c) the account level only; (d) also the in-match weapon XP. Recommendation: (a).
- **LOOK:** does his "Comic flavour" line change LOOK_V2 §7, whose comic touches are "used sparingly"? Why: the look is
  his and the Advisor's. Options: (a) no, §7 stands until his VFX style guide and the look proof test (one reading:
  his line says what the tool can make, §7 how often); (b) yes, the look brief's owners update §7 in their own pass.
  Recommendation: (a).
## For Mustafa
1. Your go to merge PR #43 at its final head, once the advisor has approved that head: confirm your words and marks
   (EDITOR_V2 §7, VISION_V2 §5) and that "XP" stays as you wrote it until XP-TERM. No Brave check.
2. XP-TERM and LOOK, whenever suits you.

## Appendix A: his words, whole (from the session's words file)
The VFX editor item (2026-11-06, 23:27 UTC, session_0173vVo7FcYUpbWgRW8oQyHU):
```text
For the next docs pass (Mustafa, 2026-11-06): record a VFX editor in EDITOR_V2 as a future tool, after Editor v2's map features. Decided principles, design later:
- Essential: the game's punch depends on it. Inspired by Unreal's Niagara (emitters with a stack of modules: spawn, velocity, colour over life, size over life, gravity, collision), but without its clutter.
- What you see is what you get: the editor uses the game's own renderer and lighting, and saves the same effect data the game loads, triggered by the game's events (hits, blasts, deaths).
- Test in context: trigger a shot or blast in a live preview, from the real game camera, and keep editing while it plays.
- Easy to pick up: each effect opens with a few big sliders (size, intensity, colour, duration); the full emitter and module stack folds open on request. One clean panel; curves edited inline. Presets to start from (muzzle flash, explosion, smoke, sparks).
- Whole impact moments: one asset can bundle particles, sound, camera shake and hit-stop, timed together, so punch is authored in one place.
- Faction colours: effects use the same role colour slots as units, so one explosion recolours per faction.
- Cost and quality: each effect shows its cost against a budget, and has quality tiers (Low drops extras but stays readable).
- Comic flavour: ink-outlined smoke, impact frames, halftone flashes, onomatopoeia.
- A VFX style guide comes first, from a separate reference study, so the tool fits our look.
- Built to iterate; no bloat.
```
The private-match progression item (2026-11-06, 23:50 UTC, the same session):
```text
For the next docs pass (Mustafa, 2026-11-06): add to VISION_V2's community-servers note, on progression in private (rented) matches:
- [DECIDED] With bots in the match: 25 % of normal XP; humans only: 100 %.
- Open, Advisor's suggestions: (1) weigh XP per action (kills on bots give little), so public matches with bot fillers aren't punished; (2) against friends trading easy wins: full XP only with a minimum number of humans per team, plus a daily XP cap on private matches.
```
VFX-ORDER (2026-11-06):
```text
VFX-ORDER: (a).
```

## Appendix B: the steps
His words (6e88f9a); plan revision 1 (3124aeb); the advisor's APPROVE (2f6742f); the build d5e2c62; docs-keeper's
pass (e967196); the review round (a814ce9, 08ac3fb) and the fix pass (5912c68); this report. The notes
(`.checkpoints/vfx-xp-notes/`) are deleted in the last commit, as CHECKPOINTS.md §9 says; they stay in the branch
history, last at this report's commit.
