# Amendments to the R1–R6 plan, second set (2026-10-30, this session)

`plan.md` is the approved plan; `plan-amendments.md` is the first set (approved with 8 conditions,
`advisor-amendments-verdict.md`). These amendments carry out the Advisor's decisions of 2026-10-30, relayed by Mustafa
in this session's chat (quoted verbatim in `.checkpoints/v2-research.md`, "The Advisor's decisions, 2026-10-30"). The
advisor agent checks them before any edit.

## The Advisor's words (verbatim)
> - VERIFY-FETCH: (a). Daytime only, read-only fetches from allow-listed hosts, with the host
>   audit in each report. The night shift stays on WebFetch only until kit-v3-1 is merged.
> - Unread sources: commit the full list to the research branch as
>   docs/research/UNREAD_SOURCES.md, with what each source was needed for. The Advisor will look
>   for other copies from there.
> - The critic's 41 items: resolve the 12 contradictions before the decision drafts. Add a shared
>   glossary (docs/research/GLOSSARY.md) with one definition per term, starting with the tick
>   budgets, and make every topic use it.
> - Schedule: back to normal from tonight (the window ends at 07:30, as decided under
>   WINDOW-0730).
> - NS-2 (from the night shift): (c). When you restore the night schedule, make the "Night
>   shift" routine start a fresh session on every firing, keeping the same one-line prompt, the
>   repository, Opus 5.5 and the hourly 00:10–07:10 schedule. If the routine can't be switched,
>   create a new one with those settings and remove the old one, or list it for Mustafa to delete.

## Already done (no plan needed; recorded for the report)
- **Schedule:** the Night shift routine (trig_019LtRqmAdzddyk6K3e3kDJz) is back on `CRON_TZ=Europe/Amsterdam 10 0-7
  * * *` (starts 00:10 to 07:10); issue #16's "Tonight only" line is removed (2026-10-30 about 18:07).
- **NS-2 (c):** the routine cannot be switched (update_trigger has no fresh-session setting). A new fresh-session
  routine was created and checked: its stored session has no repository (`sources: []`) and no connectors (the tool
  cannot attach either), so its night shift could not read NIGHT_SHIFT.md or post to issue #16. It was deleted at once
  and the old routine kept. Creating it with the repository, the GitHub connector and Opus 5.5 needs the Routines page:
  it goes to "For Mustafa", with the old routine to disable once the new one exists.

## What changes in the plan
1. **Fetch method (VERIFY-FETCH (a)).** From now on, verifier and owner prompts in the day window may read
   allow-listed pages with read-only `curl` (GET only, no credentials, output to the scratchpad) as well as WebFetch;
   a quote counts as confirmed only when its exact words were matched in the fetched page. Every report carries a host
   audit (hosts fetched, by tool, from the agent transcripts, as `wave1-tally.md` did). Night runs stay WebFetch only.
   Past curl use (wave 1's three verifiers and the R2 fixer) is covered by this decision; the report states it.
2. **`docs/research/UNREAD_SOURCES.md`** (main session, mechanical; no new claims). One row per unread source, merged
   by URL across the six "Sources not read" tables: the source; the topic(s) and the brief question it was sought for;
   **what it was needed for**: the finding, recommendation or open question that cites it or that the unit's notes tie
   it to, and, for the rows of the six "Recommendations that hinge on a source not read" tables, what hinges on it;
   why it was not read (each topic's words); a blank column "Other copy (the Advisor)". Search leads that were never
   meant as sources are grouped at the end. The file's header says it lists, not judges.
3. **The 12 contradictions and the glossary**, in four steps (the consistency critic's list is
   `engine-architect-consistency-result.json`; the other 29 items are hidden dependencies, gaps and one wrong
   cross-reference):
   a. **Glossary and resolution proposals** (engine-architect, xhigh, one agent; no topic file edited):
      `docs/research/GLOSSARY.md`, one definition per term, starting with the tick budgets (sim step time, whole server
      tick time, sync and transport work, the share of the tick period each may use, the reference server and its
      measuring unit), then the other terms the 12 contradictions turn on (the stress scene, the largest mode and the
      mode sizes, preview isolation, and so on). Each definition names the ids in each topic that use the term today
      and how. Plus `resolutions.md` (notes): for each of the 12, a resolution in one of two forms: **resolved**
      (which position holds, the reason from findings the files already hold, and the exact change per file), or
      **needs a decision** (the question, options, a recommendation, and whose call it is: Mustafa for VISION_V2's
      reading, game feel, cheat trade-offs and privacy; the Advisor for the deferred security baseline). No new
      claims; a resolution that would need a new source becomes "needs a decision" or a spike item.
   b. **Advisor agent review of the resolutions**: per item APPROVE, REVISE or ESCALATE (to Mustafa, under "Decisions
      needed"). Only approved resolutions are applied; escalated ones are applied as one shared open question, worded
      the same in every affected topic, until Mustafa decides.
   c. **Owners apply them**, one agent per file (never two on one file): engine-architect for R1 and R4 (one agent,
      both files), netcode-architect R2, graphics-engineer R3, test-engineer R5 (the helper's R5-CI and R5-PREV parts
      included), infra-engineer R6; effort xhigh for R1, R2 and R4 (netcode and the sim core), high for the rest. Each
      applies, to its own file only: the approved resolutions; the glossary's terms (each first use links
      GLOSSARY.md; a value that changes because of a definition is a recommendation change, logged as such); and the
      critic's other 29 items that touch its file, as cross-topic open questions `R<n>-XT-OQ<k>` in the critic's words
      with both sides' ids. Each logs its changes by type (nit, recommendation, number, source). At most 4 agents at
      once.
   d. **Verifier on the changes** (high; one agent, all six diffs against the commit before step c): every applied
      resolution matches `resolutions.md` as approved; glossary terms are used as defined; no new claim, number or
      source slipped in (any that did is checked at its source or sent back); the labels stay true (R1 and R2 stay
      "verified (every recommendation claim)" only if every changed basis claim is confirmed). Review stops at nits,
      as before.
4. **Then, as planned:** docs-keeper (STATUS, ROADMAP, the README status column, REPO_MAP with GLOSSARY.md and
   UNREAD_SOURCES.md), the gate, the report (now also: the glossary, the 12 resolutions, the escalated decisions, the
   29 cross-topic open questions, the fetch audit under VERIFY-FETCH (a), NS-2's outcome), delete the checkpoint, PR
   #28 ready, the advisor agent's PR review, Mustafa's "merge #28".

## Agents and order
About 9 more agents: the critic-follow-up engine-architect (3a), the advisor (3b), five owner agents (3c, at most 4 at
once), the verifier (3d), docs-keeper; plus the advisor's PR review. Research runs in the day window only (07:30 to
00:10); at most 4 agents at once (the Advisor, 2026-10-28). Units are small and notes-first, as before.

## Write-guard stand-in
Unchanged, plus the two new files: `git status --porcelain` may show changes only under `docs/research/`,
`docs/reports/` and `.checkpoints/v2-research/` (and, for docs-keeper's step, `docs/STATUS.md`, `docs/ROADMAP.md`,
`docs/REPO_MAP.md`).
