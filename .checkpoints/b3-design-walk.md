# Checkpoint: b3-design-walk (B3: Mustafa's 2026-11-04 design decisions into LOOP_V2 and UNITS.md)

- **Branch:** `claude/sleepy-tesla-ofsxsr`. It stands in for `cs/b3-design-walk`: this cloud session may push only
  there. The work happens in the worktree
  `/tmp/claude-0/-home-user-Shamble-Frontlines/263eddf3-7643-5640-b1d5-39be43d3567c/scratchpad/b3`, on local branch
  `b3-design-walk`, started from main 4893ab6 (PR #35 merged). The push is a fast-forward from the branch's old head
  76c35bf. Checkpoint commits are made by hand and marked `[skip ci]`.
- **Started:** 2026-11-04, right after PR #35 merged on Mustafa's go ("Merge $35", read as #35).
- **Current step:** 2, plan revision 2, which is below.
  - Plan revision 1 with the advisor's 11 conditions applied: `.checkpoints/b3-design-walk/plan-rev1-folded.md`.
  - Its verdict, APPROVE with 11 conditions: `.checkpoints/b3-design-walk/advisor-plan-rev1.md`.
  - Mustafa's message and his answers, word for word: `.checkpoints/b3-design-walk/mustafa-2026-11-04.md`.
  - The checked mapping, with its skeptic check: `.checkpoints/b3-design-walk/mapping-checked.md`.
  - Next: the advisor reviews revision 2's changes, then the build.
- **Agents (condition 11 of the docs pass; B3's condition 8):** all Opus 5.5. advisor, game-designer, verifier and
  docs-keeper run at high. engine-architect is not used (B3 condition 8). Notes go to
  `.checkpoints/b3-design-walk/<agent>-<part>.md`.

## Goal
Record Mustafa's design walk of 2026-11-04 in docs/LOOP_V2.md (rules) and docs/UNITS.md (units and factions), word
for word, marked decided or idea. Revision 2 adds his answers to B3-Q1 to B3-Q3. It is docs only, symbols +0 −0, and
there is no Brave check. A change to rule text in the core loop is likely needs-mustafa: his reconfirmation.

## Mustafa's answers (2026-11-04, after plan revision 1's APPROVE), word for word
> Answers to B3's questions (Mustafa):
> - B3-Q1: (c), unit by unit when I write the unit list, with two fixed rules: every faction has an anti-vehicle
>   answer from tech 1; and a role every faction has (anti-vehicle, infiltration) shares one core with a faction twist
>   on top, like the infiltrators.
> - B3-Q2: (a). Account progression gives class variants and cosmetics only. New classes come through the team's
>   tech in the match, so nobody is ever missing a counter.
> - B3-Q3: (i) no new building: the HQ holds the team's tech, so infiltrating the HQ steals tech; "cuts their power"
>   works like a destroyed power plant for a set time. (ii) a short window on a fresh body, no downed state; the
>   reveal shows only that enemy's team.

## Plan revision 2: what changes from revision 1 (folded)
His answers close the four open items Q-UNIQUE, Q-UNLOCK, Q-TECHBLD and Q-DOWNED. Each new doc line quotes his words
and cites "(Mustafa, 2026-11-04, B3-Qn)".
- **L7** (§5, after line 73): the `[OPEN]` "Shared or unique?" block becomes:
  ```
  - `[DECIDED]` Shared or per faction: decided unit by unit when Mustafa writes the unit list, with two fixed rules:
    every faction has an anti-vehicle answer from tech 1; and a role every faction has (anti-vehicle, infiltration)
    shares one core with a faction twist on top, like the infiltrators. (Mustafa, 2026-11-04, B3-Q1 (c).) This
    covers the Javelin and the Officer, shared at tier two today, and the Juggernaut (§6); their lines stay as they
    are until the unit list decides them.
  ```
- **L8** (§5, the infiltrator block): its two `[OPEN]` sub-bullets ("Which building is the 'tech building'?" and
  "What is 'downed'?") become:
  ```
    - `[DECIDED]` The "tech building" is the HQ: no new building; the HQ holds the team's tech, so infiltrating the
      HQ steals tech. "Cuts their power" works like a destroyed power plant for a set time. (Mustafa, 2026-11-04,
      B3-Q3 (i).)
    - `[DECIDED]` The KGB agent's "downed enemy": a short window on a fresh body, no downed state; the reveal shows
      only that enemy's team. (Mustafa, 2026-11-04, B3-Q3 (ii).)
  ```
  The block's first line keeps his 2026-11-04 words verbatim ("tech building: steals tech"); the sub-bullet above
  says which building that is.
- **L9** (§6, after line 82): the `[OPEN]` pointer becomes the untagged
  `  - Shared or per faction: decided unit by unit in the unit list (§5; Mustafa, 2026-11-04, B3-Q1 (c)).`
- **L12** (§8, line 105 and after): B3-Q2 (a) plainly supersedes line 105's "more classes".
  - Line 105 ("Unlocks give breadth, never raw power: more classes, sidegrade variants, cosmetics.") is replaced by
    his line, and its old words go word for word into §12's supersede list (B3-SUPERSEDE).
  - The same goes for the role line "Unlocks add variants and specialists" (line 112).
  - The inserted lines become:
  ```
  - `[DECIDED]` Account progression gives breadth only (class variants, cosmetics), never power. (Mustafa, 2026-11-04)
  - `[DECIDED]` Account progression gives class variants and cosmetics only. New classes come through the team's
    tech in the match, so nobody is ever missing a counter. (Mustafa, 2026-11-04, B3-Q2 (a).)
  ```
  (Line 105's replacement is the second line.)
- **Counts and evidence:**
  - "Open since 2026-11-04" falls to 0 everywhere, so E4 expects 0.
  - §8 "Questions for Mustafa" becomes "Answered by Mustafa (2026-11-04)", with his three answers word for word.
  - The report's appendix carries his message and these answers word for word (condition 5).
- **The register** (docs/adr/README.md, B3-REG): his answers add nothing new to ADR open questions beyond revision 1's
  pointers, but B3-Q2 (a) bears on OQ-0005-AB5's account-unlock path. Record a "bears on" clause only; no ADR edit.
- **STATUS, ROADMAP and REPO_MAP (docs-keeper, at the end):** besides B3's own entries, this pass takes the follow-ups
  from PR #35's final review, so no separate docs pass is needed:
  - STATUS's "draft" for #35 becomes merged (4893ab6, 2026-11-04 15:50 UTC, on Mustafa's go);
  - A4 goes into ROADMAP's queue;
  - "about 14:02" becomes "14:03";
  - the "B3" qualifier;
  - the STATUS:230 wrap;
  - the two routine proofs, once their records exist: the PR Advisor's 18:45 UTC scheduled run and the night shift's
    first guarded night.
  The list is in `scratchpad/queue/docs-pass-items.md`, "For the NEXT docs pass".
- **B3-QUEUE** is moot: #35 merged, so B3 runs now in this day session and the Night queue entry is not needed.

## Step log
1. Plan revision 1 (scratch) and the advisor's APPROVE with 11 conditions, then folded (2026-11-04 ~15:20 UTC).
2. PR #35 merged as 4893ab6 (15:50 UTC). This branch was restarted from it, and plan revision 2 written.

## Next steps
1. The advisor reviews revision 2's changes.
2. Build:
   - the edits in LOOP_V2, UNITS, DESIGN_NOTES and the ADR index, per the folded plan plus revision 2;
   - then docs-keeper's STATUS, ROADMAP and REPO_MAP, with the #35 follow-ups.
3. Evidence (E1 to E10), gate, reviews (game-designer, the verifier, a completeness critic, each with a skeptic),
   fixes.
4. Report with the appendix; delete `.checkpoints/b3-design-walk*` in the last commit (not `[skip ci]`); mark the PR
   ready; the advisor's final-head review; Mustafa's go.
