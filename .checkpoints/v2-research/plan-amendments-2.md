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

## The advisor's verdict and how its 13 conditions are applied
APPROVE with 13 conditions, 2026-10-30 about 17:20 UTC (verbatim: `advisor-amendments-2-verdict.md`). Applied as follows,
in this file and in every prompt before launch:
1. Three outcomes per contradiction: **resolved**, **aligned** (one shared open question, same words in every affected
   topic, one decider and a decision code) or **escalated** (needs a decision). The report counts each kind.
2. Order: #10, #11, #9 and their glossary terms first, then #1; #19 before #21 and #22; #5 with #6. A resolution that
   depends on an escalated item holds under every option (written in terms of, for example, "the largest mode's N"); a
   glossary entry that depends on one states both readings and the code, and picks neither.
3. Step 3a works in small units (one term or one contradiction each, written whole); `resolutions.md` gives, per item,
   both sides' words with ids, the exact before and after text per file, and the basis ids.
4. Every changed number comes from the files (id), is arithmetic shown in `resolutions.md`, or is a pass mark marked
   "proposed" with its reason. No new sourced fact.
5. Step 3b runs at xhigh; its verdict goes to `advisor-resolutions-verdict.md`. MODES-3, PREVIEW-WALL and BASELINE-WHEN
   are asked now (the session's report of 2026-10-30).
6. Step 3c: an owner who finds an approved resolution wrong for its file logs OBJECT with the reason and does not apply
   it (the advisor decides); owners change nothing beyond the approved text, the glossary terms and nits; quoted source
   text never changes. The session records the SHA before step 3c and commits each owner's file as it returns, after
   the write-guard check.
7. Steps 3a and 3c fetch nothing; only step 3d fetches. curl rules: GET over https only (`--proto =https`), no data,
   upload or method flags; no `-L` (a redirect is recorded and followed as a new fetch only when its target host is on
   the allow list); no auth headers, cookies, netrc or config files; time and size caps; `-o` into a fresh empty
   scratchpad folder; downloads never run, Python on them only with `-I`; the HTTP code and final URL recorded. The
   host audit lists requested and final hosts by tool. The report says past curl use is covered by (a), except the 3
   api.github.com calls of the #29 review (off the allow list; a PR review, not research).
8. UNREAD_SOURCES.md: inputs all 8 not-read tables and the 6 hinge tables; URLs matched ignoring the scheme, `www.`
   and a trailing slash; rows without a URL stay as their own rows in the topic's words; the brief question from the
   unit column, or for R3 to R6 from the per-question draft or table that names the URL; every "needed for" cell cites
   topic ids, or quotes the notes with the file at a commit SHA, or says "no tie found in the file or notes"; "search
   lead" only where the topic's own words say so; a script shows every source URL appears exactly once, with the counts
   in the report. Built and pushed first; re-run after step 3c; step 3d samples 10 rows.
9. The glossary is checkable: each entry gives the term, its one definition, the words that mark it, the synonyms to
   retire, and the ids that use it per topic. After step 3c a search script lists every hit of each term and each
   retired synonym per topic and must find zero retired synonyms outside quoted source text. Each topic header links
   GLOSSARY.md, and so does the first use of each term; the "used in" lists are refreshed from the script's output (in
   the report).
10. Step 3d in three units, at most 4 agents at once: R2 at xhigh; R1 and R4 at high; R3, R5 and R6 at high. In R3 to
    R6 every changed recommendation gets at least one basis claim checked at its source unless its verifier table
    already confirms one; in R1 and R2 every changed basis claim not already confirmed is checked; the "N of M" labels
    are recounted; a value that changes because of a definition is a recommendation change and is checked.
11. The other 29 items: rank 41 is fixed in R5 as a nit (cite R1-BND-REC2 mark 5); items a resolution settles are
    marked "settled by resolution <rank>"; each shared open question carries its decision code.
12. The 12 resolutions and the advisor's per-item verdicts go into the report word for word. Risks: a spend cut in the
    middle of step 3c (each owner's file is committed as it returns; a cut-off owner resumes from its notes); a
    resolution taking Mustafa's call (escalated items stay open questions until he answers); R1 or R2 losing "verified"
    (step 3d recounts; the label changes if a changed basis claim is not confirmed). Evidence: the script outputs, the
    verifier tables, the gate, the repo-map check. Answers from Mustafa or the Advisor that arrive before the PR is
    marked ready are applied the same way (owner applies, verifier checks); later ones go to the synthesis.
13. NS-2 is called "NS-2 (2026-10-30, a fresh session per run)". Mustafa's item lists the exact settings with the
    one-line prompt word for word; order: create the new routine, then disable the old one in the same sitting, and
    delete the old one after one good night; the session then reads both routines back and confirms only one is enabled.
Nits applied: the write-guard stand-in also allows `.checkpoints/v2-research.md`; research launched between 07:30 and
08:30 counts a still-running night run in the 4-at-once limit until the WINDOW-0730 rules PR lands.
