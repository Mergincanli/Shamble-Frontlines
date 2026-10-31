# Plan: docs-after-31 (docs only), revision 1

## Goal
Hand over cleanly, as the Advisor asked on 2026-10-31 ("Hand over cleanly: make sure STATUS and the checkpoints are
current, then stop."): STATUS and ROADMAP record PR #31's merge (2f5b52e) and DRAFTS-SESSION (a), so the fresh session
that plans the decision drafts starts from a current dashboard. Then the v2 day session stops.

The decisions, word for word (the session's chat, 2026-10-31, about 15:20 UTC):
> Mustafa: merge #31.
> Advisor: DRAFTS-SESSION: (a). Mustafa starts a fresh session with the Advisor's drafts brief.
> Hand over cleanly: make sure STATUS and the checkpoints are current, then stop.

## Branch
`cs/docs-after-31`, from origin/main 2f5b52e (PR #31's merge commit), in the main checkout.

## Change type and scope
Docs only, symbols +0 −0: `docs/STATUS.md` and `docs/ROADMAP.md` (docs-keeper). Nothing else: no research file, no
rules file, no issue edit (the Night queue needs no change: nothing new can run unattended). Checkpoints: none is open
on main or on this session's branches (`git ls-tree` on origin/main, cs/v2-research and cs/docs-after-v2-research: 0
files); this change-set's own checkpoint is deleted before ready (CHECKPOINTS.md §9).

## The edits (docs-keeper)
1. `docs/STATUS.md`:
   - Header line 3: add docs-after-v2-research (PR #31, 2f5b52e, 2026-10-31) to the merged list.
   - Phase, the v2 paragraph: docs-after-v2-research joins "Done" (PR #31, 2f5b52e, 2026-10-31; docs only). "Next":
     the decision drafts, planned in a fresh session that Mustafa starts with the Advisor's drafts brief (DRAFTS-SESSION
     (a)); the v2 day session (session_0173vVo5q59ameR6cnN9BENg) handed over and stopped on 2026-10-31. The advisor's
     cosmetic note on PR #31 (its review, finding 3): the proof-test sentence says "before the foundation" twice in a
     row; keep only the quote ("… the slices in the new order (…), with, in their words (Mustafa and the Advisor): "The
     netcode proof test (48 simulated players over WebTransport) still runs before the foundation."").
   - In flight: the docs-after-v2-research row becomes "merged: PR #31, 2f5b52e, 2026-10-31, on Mustafa's \"merge #31\"
     after the advisor's APPROVE of 1c687d1" | done | none. A new first row: docs-after-31 (this change-set; docs only:
     STATUS and ROADMAP after #31's merge, and the handover) | `cs/docs-after-31`, PR #<n> | waiting on the advisor's
     review of its head, then Mustafa's go.
   - Decisions pending, the group "Taken by the Advisor 2026-10-31 (relayed by Mustafa)": add DRAFTS-SESSION (a) (where
     the decision drafts are planned): "Mustafa starts a fresh session with the Advisor's drafts brief."
   - Routines: unchanged unless the new routine's schedule has changed when docs-keeper edits (the session reads it
     again just before and passes the result; at 15:26 UTC it still read `CRON_TZ=Europe/Amsterdam 10 0 * * *`).
2. `docs/ROADMAP.md`:
   - Recent work: "Docs after v2 research [2026-10-31, IN PROGRESS]" becomes "[2026-10-31, MERGED]", PR #31, 2f5b52e,
     on Mustafa's "merge #31" after the advisor's APPROVE of 1c687d1; its last sentence notes the handover (DRAFTS-SESSION
     (a)) and this PR. No new entry (11 entries stay; the file's 8–12 rule).
   - Next / open, the "v2, next" bullet: the decision drafts are planned "in a fresh session from the Advisor's drafts
     brief (DRAFTS-SESSION (a))".

## Evidence and report
- No report file: the PR body is the report (docs-only precedent). Gate PASS on the branch; CI `gate` and `smoke` on
  the head; repo-map check (no paths added or removed).
- The advisor reviews this plan, then the PR's final head (after the checkpoint is deleted, as for #31).

## Merge path
Draft PR; checkpoint folded into the body and deleted in a CI-running commit; the advisor reviews that head; ready;
Mustafa's "merge #<n>" in this session's chat, or, if he has moved on, in the session he gives it in (the advisor's
verdict is posted on the PR naming the full SHA, so that session can merge it). Then the v2 day session stops.

## Risks
1. This PR's own STATUS row will read "waiting on … go" after it merges, like every docs-after PR's: the drafts
   session's first STATUS update marks it merged.
2. If Mustafa changes the night routine's schedule after docs-keeper edits, STATUS's Routines line goes stale until the
   next docs pass.
