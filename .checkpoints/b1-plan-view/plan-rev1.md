# B.1 plan, revision 1 (b1-plan-view, 2026-11-05): EDITOR_V2's plan view and Mustafa's editor decisions

Branch `claude/sleepy-tesla-ofsxsr` stands in for `cs/b1-plan-view`. The worktree is from main af8393e (PR #38
merged). Docs only: no code, so symbols +0 −0. Not player-facing. Not kit, so the kit freeze does not apply.

## 1. Goal
Record two things in `docs/EDITOR_V2.md`, word for word and marked the way Mustafa marked them:
- his B.1 of 2026-11-03, "Plan view: draw to build";
- his editor decisions of 2026-11-05.

Editor v2's architecture plan (EDITOR_V2 §6 step 2) then builds on his own rules. Also: the docs after PR #38's merge.

## 2. His words (the sources, word for word in `mustafa-words.md`)
- **B.1, 2026-11-03.** From his task list (session_0173vVo5sT21uP4DcQtKvmAU, 22:08:25Z). It is in STATUS's "Queued, in
  Mustafa's words" (STATUS.md:455-470 at af8393e).
- **B.1 addition, 2026-11-05.** "Record these editor decisions from my talk with the Advisor in EDITOR_V2, together
  with the plan view, marked as below". Eight lines follow:
  - seven "Decided" lines, one of them "Decided, lower priority";
  - one "Principle" line.
  - The addition is in STATUS (STATUS.md:474-487).

## 3. The edits to `docs/EDITOR_V2.md` (66 lines today)
1. **The principle, at the top, under the intro paragraph.** His line, whole: "Principle: a lean, solid editor,
   planned and built with the same care as the game; no bloat." It carries the tag set by P-TAG (§7).
2. **A new block in §3 Features, "Decided 2026-11-05 (Mustafa, after his talk with the Advisor)".** It holds his
   seven decision lines, each whole and in his order, each with his mark:
   - the tool order (1) to (5);
   - the primitives and one scale;
   - every object's width, length and height;
   - the views;
   - trace sketches, sticky notes and snapshots;
   - the travel-time ruler (his "lower priority");
   - stamps.
3. **"Plan view: draw to build".** His B.1 paragraph goes in whole, as a `[DECIDED]` sub-item right after the views
   line (the plan view is one of the views). His "Built with Editor v2 (slice 5)" stays as he wrote it.
4. **Where his decisions meet an existing line, the old line stays word for word** and gains only a short pointer in
   brackets ("see Decided 2026-11-05: …"):
   - **"Select and transform"** `[DEFAULT]`, its numeric inspector. See: every object's width, length and height.
   - **"Test"** `[DEFAULT]`: "return to the editor exactly where you were". See tool order (2): "you can keep editing
     while the match runs" goes further.
   - **"Validation panel"** `[DEFAULT]`, its symmetry check. See tool order (3) and (4): heatmaps and the fairness
     panel.
   - **`[LATER]` "a prefab library v2"**. See stamps, now decided as an optional, top-level feature.
   - **§6, the order of work, is unchanged.** One pointer line notes that his tool order (1) to (5) ranks the tools;
     the slices stay the build order of the architecture.
5. **The header's tag line** gains only what P-TAG decides.

## 4. The other docs
- **STATUS: PR #38 merged** (af8393e, 2026-11-05 21:25 UTC, a merge commit, on Mustafa's go after the advisor's APPROVE of
  8b7cfb3, PR comment 6669362242; CI green on main). This updates:
  - the header (:3), Phase (:25-33), the In flight rows (kit-v3-3 to merged), and A3's Open bugs entry (:744, "a
    draft" to merged);
  - B.1's new In flight row;
  - the Queue line: B.1 in flight, then B.2; A4 waits;
  - "Queued, in Mustafa's words": B.1's two entries go once EDITOR_V2 holds them. The block says "each removed by the
    change-set that carries it out". A one-line pointer to EDITOR_V2 stays.
- **ROADMAP:** A3's entry (:93, "a draft" to merged) and B.1's entry.
- **The A3 report: the advisor's two Low notes**, as corrections dated after the merge.
  - E3's "176 files" becomes "176 in the first scan; 178 and 193 in the final scans".
  - A new line says that no automated review of 0c481e1 arrived.
- **REPO_MAP:** only if EDITOR_V2's row is now untrue.
- **A follow-up line in STATUS:** the engine-wide "1 unit = 1 metre" (his words) for the ADR drafts' next revision.
  No ADR draft names a unit convention (grep: no hit in docs/adr/). The drafts are not edited in B.1.

## 5. Evidence
- **E1:** his words byte for byte. Each of his nine lines (B.1's paragraph, and the eight lines of the addition) is
  found exactly once in EDITOR_V2, checked by a script that only reads.
- **E2:** every existing EDITOR_V2 line is unchanged except the pointers. A line diff names each changed line.
- **E3:** tag counts before and after (`[DECIDED]`, `[DEFAULT]`, `[OPEN]`, `[LATER]`, and any new tag).
- **E4:** STATUS and ROADMAP are true. #38's merge facts are checked with the GitHub API. Every line reference is
  re-verified.
- **E5:** `node tools/gate.mjs --no-smoke` (docs only; CI runs smoke) and `repo-map-check.mjs`.

## 6. Reviews
- **The verifier:** E1 to E5.
- **game-designer:** the editor's balance tools (the fairness panel, heatmaps, sightline checks) do not contradict
  LOOP_V2 or VISION_V2.
- **Skeptics:** only for an upheld finding at High or above, under his review budget. Below High it becomes a
  STATUS follow-up.
- **Agents:** Opus 5.5 at high, at most 3 at a time. Notes in `.checkpoints/b1-plan-view/`, committed as they go,
  never only in /tmp.

## 7. Decisions for the advisor
- **P-TAG, how to mark "Principle" and "Decided, lower priority".**
  - (a) `[DECIDED]`, with his words kept in the text ("Principle:" and "lower priority"). No new tag.
  - (b) New tags `[PRINCIPLE]` and a priority mark, defined in the header, as LOOP_V2 added `[IDEA]` in B3.
  - Recommended: (a). His words carry his marks, and the tag set stays small.
- **PLACE, where the decisions go.**
  - A dated block in §3 (recommended): his order and wording stay whole.
  - Or each line moved into the §3 bullet it fits.
- **OVERLAP, the old `[DEFAULT]` and `[LATER]` lines his decisions touch.**
  - Pointers only (recommended): the old text stays, and the reader is sent to his decision.
  - Or leave them untouched.
- **SCALE, "1 unit = 1 metre … across the engine".**
  - EDITOR_V2 plus a STATUS follow-up for the ADR drafts (recommended): the drafts are Proposed, and ADR edits are
    their own change-set.

## 8. Mustafa's check
His go to merge, as for B3: he confirms his words and marks are placed as he meant. No Brave check.

## 9. Risks
1. A pointer could read as a change to a `[DEFAULT]` line. E2 guards this: the line's own words never change.
2. His "(slice 5)": EDITOR_V2 §6's slice 5 is "Verticality". It is recorded as he wrote it and flagged for the
   architecture plan (§6 step 2).
3. "One key drops you in at the cursor with bots" against the hard constraint that the player build never loads
   editor code (EDITOR_V2 §5). The decision is the editor's own Test mode, so no conflict: noted for the
   architecture plan.
