# Checkpoint: v2-decision-drafts
- **Goal:** the v2 decision drafts (`docs/ENGINE_OVERVIEW.md`, `docs/adr/0001`–`0006`, the spike plan, the loose ends)
  from the drafts brief (`v2-decision-drafts/inputs/DRAFTS_BRIEF.md`), then a draft PR, the report and a stop for Mustafa
  and the Advisor · plan: `v2-decision-drafts/plan.md` (revision 3: the advisor's APPROVE of revision 1 with 19 conditions folded in revision 2, verdict verbatim in `advisor-verdicts.md` §1; revision 3 applies Mustafa's UPGRADE-SCOPE update)
- **Branch:** claude/laughing-lovelace-ox61se (the session's branch, standing in for cs/v2-decision-drafts) ·
  **Base:** origin/main 9e7db28 (PR #32's merge)
- **Last update:** 2026-10-31 18:57 Amsterdam

## Steps done
1. Step 0 (brief §0): `docs/LOOK_V2.md` added byte for byte (sha256 52b4fc06…) · 6617e25; the brief copied to `inputs/`;
   plan revision 1 written · d6c71e4.
2. Plan review: the advisor's APPROVE with 19 conditions (about 18:35); folded into plan revision 2 (no second review
   needed, per the verdict); `origin/main` fetched to 9e7db28 (condition 1); the id list rebuilt (REC 165: the two R3
   rows were references, condition 6) · 5ab5eae.
3. Mustafa's UPGRADE-SCOPE update (about 18:41, in the session's chat; it replaces the brief's upgrade note): saved
   verbatim to `inputs/UPGRADE-SCOPE-mustafa-2026-10-31.md`; plan revision 3 applies it (settled inputs, the ADR 0005
   paragraph, the map addendum's part (e), docs-keeper's STATUS record) · f927ab3.
4. Decision map (step 2) done; coverage script: 521 of 521 ids, one owner each; addendum sent; drafting started
   (0003, 0006, 0005's ability section) · b2719db.
5. The map's addendum done (part j); 0001 and 0004 launched · this commit.

## Current step
Step 3, drafting: five units running (0001, 0003 + 0005's asset section, 0004, 0005's ability section, 0006).

## Next steps
1. As slots free (≤ 5 at once): 0002 (`prompts/engine-architect-adr0002.txt`); 0005's content data and frame
   (engine-architect, after both 0005 sections); test-engineer's pass marks per draft as each lands (step 4).
2. Pass marks per draft as each lands (step 4); assembly (step 5); synthesis (step 6); verification (step 7; no
   fetching verifier after 22:00, no new agent after 23:00).

## Open findings
- The research has no asset-pipeline findings (ASSET-RESEARCH, in the plan).

## Key numbers
- Research ids defined in r1–r6 at 9e7db28 (`inputs/research-ids.tsv`, built by script, rebuilt after the review): 533
  rows, 521 unique ids: REC 165, OQ 241, RR 7, XU 30, XT 72, 6 shared codes (18 copies). Matches the research report's
  165 recommendations and 278 own open questions (241 + 30 + 7).
- Findings checked by each topic's verifier (`inputs/findings-checked.tsv`, script-built guide): R1 172/185, R2 136/152,
  R3 44/92, R4 49/62, R5 49/117, R6 18/42.

## Decisions taken
- 2026-10-31: the drafts brief (Mustafa and the Advisor) and LOOK_V2 (attachments of the session's first message).
- 2026-10-31 about 18:41, Mustafa: the UPGRADE-SCOPE update (tracks per unit and per weapon type, pick one of three per
  level, owned by the player, surviving death and switching, reset every match; a class comes with its own weapon;
  other weapons only from enemy drops, as world pickups; progress on the weapon type; on the PlayerState). Open for him:
  carry slots, ammo for looted weapons, track levelling speed with catch-up.
- 2026-10-31 about 18:35, the advisor: plan APPROVE with 19 conditions; rulings ASSET-RESEARCH (a), TICK-60-HEADROOM
  ruled now (base 8.33 ms, p99 ≤ 6.25 ms; confirmed at the stop), GLOSSARY-8 yes (three entries and a note),
  CHECKPOINT-AT-STOP keep.

## Running work
- advisor plan review: done (APPROVE), notes `advisor-plan-review.md` (DONE).
- engine-architect decision map: done about 18:45 (521 of 521 ids, one owner each: 0001 127, 0002 104, 0003 71,
  0004 82, 0005 34, 0006 65, DESIGN 38); its addendum (part j, from line 1032) done about 18:55, owners unchanged,
  coverage re-run 521/521; GLOSSARY §8 text in part J1.
- Drafting, launched about 18:56: netcode-architect (0001; `netcode-architect-adr0001-notes.md`), rust-reviewer (0004;
  `rust-reviewer-adr0004-notes.md`).
- Drafting, launched about 18:48 (background agents, effort xhigh, opus): graphics-engineer (0003 + 0005 asset section;
  notes `graphics-engineer-adr0003-notes.md`), infra-engineer (0006; `infra-engineer-adr0006-notes.md`), game-designer
  (0005 ability section; `game-designer-adr0005-notes.md`). Prompts in `prompts/`.
- checkpoint saver: `checkpoint.mjs watch` in the background since 18:22 (110 min).
