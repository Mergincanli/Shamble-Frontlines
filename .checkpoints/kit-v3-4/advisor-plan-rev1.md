# Advisor: review of kit-v3-4 plan revision 1 (2026-11-07)

Read from origin/main: `.claude/agents/advisor.md` and `CLAUDE.md` (git show). The worktree's kit and docs equal main's
(`git diff --quiet origin/main -- .claude CLAUDE.md docs`), so line numbers below are main's.

## Verdict
**APPROVE WITH CONDITIONS (13)**, plus two decisions for Mustafa (TIER0-SCOPE, REVERT-EXCEPTION). The docs parts (F,
then D) may start now. The kit sitting writes no tier-0 line before his TIER0-SCOPE answer, and the revert exception
goes in as text he confirms as its own decision, not through his go on the PR alone.

The plan is faithful in shape: it keeps "sees, hears or feels", adds security-auditor to every tier-2 change, makes
money tier 3, reads "auto-merge" as the PR Advisor's pinned merge, and changes no guard, settings or CI file. What it
must fix: four places where a reworded rule could loosen one he did not ask to loosen (the verifier under the budget,
the tier mapping dropping needs-mustafa triggers, the revert's merge path, tier 3's "nothing merges"), and a few
facts and placements.

## Rulings on plan §7
- **TIER0-SCOPE: his, confirmed.** His literal words ("Tier 0 docs only ... auto-merge") read as (b); (a) keeps a check
  he has today (ADRs are escalation category 3, advisor.md:39; design decisions category 2, :38). Choosing between his
  literal words and a check he set is not mine. ESCALATE; recommend (a).
- **T0-MERGE: (a).** Mine (a technical choice). GitHub auto-merge skips the expected-head check the guard enforces for
  every session (routine-guard.mjs:1393; merge check :1444-1466); (a) is today's unattended merge (advisor.md:134).
  The report says in one line that "auto-merge" was read this way.
- **DOCS-RIDE: (a).** One PR, one go; A4 needs his go anyway. If the kit sitting cannot happen the day the docs are
  built, split F and D into their own docs PR rather than hold his answers back.
- **SCALE-HOME: (a), CLAUDE.md.** Every agent gets CLAUDE.md in context; one definition cannot drift. Condition 5.
- **REVERT-EXCEPTION: the design is sound, the decision is his.** It writes an exception into CLAUDE.md's Never line
  "edit before the plan is approved" (:240), escalation category 3. ESCALATE; recommend yes, day sessions only, with
  conditions 7 and 8.

## Conditions
1. **Tiers add, never remove.** The tier text says the needs-mustafa list stays whole: step 0's kit-touching diff ->
   day-session review (advisor.md:76-78); step 2's routine rules (CI green void when `.github/` or gate files change;
   `shamble-*.js`, the pages, or a sim/render/measurement/bug-repro claim -> "verify in a day session", :100-105);
   step 4's major-change triggers and "when unsure ... needs-mustafa" (:114-117). A PR's risk tier sets its minimum
   reviews and never lifts a needs-mustafa trigger; a PR's tier is the highest any changed file or effect reaches.
   Plan §2 A's mapping lists these lines too (it names only Brave checks, tiers 2-3 and his words today).
2. **Tier 0's file set, stated.** Tier 0 = `docs/**` and markdown outside the kit; any `.claude/` file or CLAUDE.md is
   tier 2 although it is text; `tools/` is tier 1 except the gate's own files (tier 2, guard :26-27). Whatever he
   answers to TIER0-SCOPE, the text gives each kind of docs PR (his words or decisions, ADRs, the register, design and
   look docs) exactly one tier and one merge path.
3. **Tier 3 reworded.** Replace the checklist's "nothing merges" with: no session or agent carries out the
   irreversible or paying act; a tier-3 PR merges only on his explicit decision of that act and his go on its head.
   CLAUDE.md's Never lines ("force-push, hard-reset or rewrite history", "delete user data or saves", :242) stay
   absolute; tier 3 names no exception to them. "Money" = any act that creates a cost (accounts, paid plans, services),
   as infra-engineer.md:40 and ADR 0006:24-25 already say.
4. **Verification sits outside the budget.** Drop "within the budget" from the changeset skill's verifier line
   (SKILL.md:25-26), or scope it to reviewer agents only. The gate after every step and the verifier's PASS on the final
   head stay required; gate and verifier re-runs are not review rounds; a verifier FAIL counts as High. (CLAUDE.md:182:
   "Explicitly not allowed: skipping verification, shrinking evidence".)
5. **The scale's floors.** In CLAUDE.md's scale, High includes: a rule loosened beyond his words; his words misquoted,
   or a decision of his recorded wrongly or as decided when it is open; evidence that does not hold. Critical includes
   any way around the guard, the merge check or branch protection. One sentence: every reviewer and the verifier labels
   each finding on this scale; security-auditor.md:52 points to it; the old words map major -> High, minor -> Medium,
   nit -> Low.
6. **What the budget never drops.** A Critical or High still open after round 3 stops the change-set and goes to him
   under Decisions needed; it is never merged as a follow-up. "A security finding left open" stays needs-mustafa at any
   severity (advisor.md:115). "Address or explicitly defer every finding" (CLAUDE.md:83) stays. A round run in batches
   of at most 3 agents (CHECKPOINTS.md §4, :68-69) is one round; skeptic checks belong to their round. The advisor's
   plan reviews and its review of every pushed head (CLAUDE.md:99; advisor.md:137-138, :145) are not rounds and stay;
   the advisor does not REVISE or BLOCK on Low findings alone.
7. **A revert's merge path is today's.** No new merge exception: a revert PR is classed by step 0, 2 and 4 like any PR
   (a revert of a game-file merge is "verify in a day session"; of a kit merge, a day session on his go). Only the plan
   step is waived. Plan §2 C's "the PR Advisor for tiers 0-1" becomes "by today's classes".
8. **Revert safety, in the rollback text.** Day sessions only: the night shift and the PR Advisor report a red main and
   never revert (NIGHT_SHIFT.md:60; an unplanned item is not approved night work). "Main breaks" needs the red check
   re-run once and pinned to one merge (not a flaky runner). The revert is built in a separate worktree from
   origin/main (`git worktree add .gate/revert-<n> origin/main`), never by switching the main folder (CLAUDE.md:246-252).
   A revert that touches kit or gate files asks Mustafa before `git revert` runs: the guard does not read `git revert`
   (STATUS:867, limit L4), but CLAUDE.md:63-68 still requires the ask. The advisor checks the exact inverse AND whether
   a later merge touched the same files (then the result is a new state, reviewed as a normal change). Re-landing the
   change is a new PR through the normal protocol.
9. **Checklists against step 0, step 2 and Never.** Each tier's list starts with "confirm the tier from the diff's file
   list" and defers to step 2 for how the gate is established in each seat (no line implies CI green is enough in a
   day session: Never :167). The tier-2 list says the routine classes it needs-mustafa and asks for a day-session
   review (step 0, :76-78); the list is for that review. Tier 0 also checks: no `.checkpoints/` files, STATUS and
   ROADMAP updated (step 2). "Ignore: nothing marked security" becomes plain words ("ignore nothing that touches
   security"). E2 lists each checklist line against the Never list and step 0.
10. **Part D.** SPIKE_PLAN's foundation note carries item 4 in his words and a one-line pointer to item 6's notes (he
    wrote "next to the fast checks on save"). No note maps "from the first playtests on" to a slice or milestone. The
    ADR 0003 timing gap (slice 6, 0003:1245) goes as a "bears on it" clause on a named existing row (README:355, I-18)
    or a STATUS follow-up: no new register row, no edit to ADR 0002 or 0003. The part 9 note is a pointer, no new metric.
11. **Part F, narrow.** R2-ROOM-OQ7 (README:183) reads "Mustafa (2026-11-07) answers which progression 'XP' means:" +
    his words; whether unrented private matches count for Service points, and telemetry, stay open. B.2's
    rented-servers follow-up (STATUS:494-498, "waits on XP-TERM") is updated the same way. Low note 2 is fixed where
    it lives (STATUS:107 and ROADMAP:94: five rows, OQ-0003-15 added); the vfx report's dated note carries notes 1, 3
    and 4 and "XP-TERM and LOOK answered 2026-11-07, (a) and (a)". VISION_V2's tag counts stay unchanged.
12. **The report.** When A4's queue entry leaves STATUS, his A4 words, A4's addition, A4-FREEZE and his 2026-11-07
    answers stand word for word in the report's appendix (the notes are deleted before ready) and STATUS points there
    (B.2 precedent, STATUS:573-575). Decisions needed lists TIER0-SCOPE and REVERT-EXCEPTION with options, plus the
    "auto-merge" reading as an info line.
13. **Evidence and the kit sitting.** E3 runs all three kit test files on main and head with equal counts:
    `.claude/hooks/routine-guard.test.mjs`, `.claude/hooks/settings.test.mjs`, `.claude/scripts/checkpoint.test.mjs`
    (the plan's `.claude/scripts/*.test.mjs` misses the guard's two). E6 shows no `.claude/hooks/`,
    `.claude/settings.json`, `.github/` or gate file in `git diff --name-only origin/main`. E2 covers every changed
    sentence of the four kit files, each "looser" with his exact words. Each kit file's new text is prepared in the
    notes and its diff against main read before the one Write; `.claude/` writes in Accept edits or Manual mode
    (CLAUDE.md:66-67). CLAUDE.md grows by about 25 lines at most (detail in advisor.md); the report gives the counts.

## For Mustafa (escalations)
- **TIER0-SCOPE:** your A4 says "Tier 0: docs only, merged by the PR Advisor without you". Should that include docs
  that record your own words or decisions, the decision drafts (ADRs), the register of open questions, and the design
  and look docs (VISION_V2, LOOP_V2, LOOK_V2, EDITOR_V2)? Why it matters: today those wait for your go, and your check
  of where your words land has caught real mistakes. Options: (a) no: tier 0 is docs that record no decision of yours;
  those others keep your go before merge; (b) yes: every docs-only PR merges without you, and you read after.
  Recommendation: (a).
- **REVERT-EXCEPTION:** if main breaks, may a day session open the revert at once, without first writing a plan and
  waiting for the advisor's approval of it? Why it matters: it adds an exception to your "Never: edit before the plan
  is approved". The revert still gets the advisor's check (an exact undo, CI green) and merges by today's rules (your
  go for rules and kit). Options: (a) yes, a pure revert is the one exception to "plan first", day sessions only;
  (b) no, a revert gets a short plan and the advisor's approval first, like any change. Recommendation: (a).

## What I checked
- His words: STATUS:544-570 against facts §0 (equal). Answers file :9 (XP-TERM (a), LOOK (a)).
- Guard: routine-guard.mjs:26-43 (protectedRel, GATE_FILES), :1340 (gh pr merge), :1393 (auto-merge refused, every
  session), :1444-1466 (mergeCheck: 40-char SHA, night refused, gate+smoke, pr-advisor refused on protected files).
- advisor.md (main): :33-34, :36-43 (categories), :76-78 (step 0), :98-105 (step 2), :113-119 (classes), :162-169
  (Never). CLAUDE.md (main): :63-68, :78-90, :92-101, :177-182, :240-252.
- SKILL.md:25-29 (verifier "Fix and re-run until PASS"; reviewers once). security-auditor.md:52 (the four labels).
- CHECKPOINTS.md:64-69 (3 agents; a round of 5 runs as 3, then 2). NIGHT_SHIFT.md:52-54, :60-63 (no rule on a red
  main; never merge, never push to main).
- SPIKE_PLAN:420-438 (no word on fast checks, rollback, reports, telemetry). ADR 0002:472-474 (per-edit hook, no format
  check). ADR 0003:1245 (field data, slice 6). ADR 0006:3, :7, :24-25, part 9 (:398-430), part 10 (:431-457),
  part 15 (:580-589), OQ-0006-14 (:999-1015, "not read" at :1011), :1552-1554.
- Register: README:103, :153, :160, :183, :202, :355 (no OQ row on field-data timing; I-18 is the interface row).
- VISION_V2:66-76; STATUS:490-526 (B.2 follow-up, XP-TERM, LOOK); STATUS:107 and ROADMAP:94 ("four register rows").
- Kit tests exist at `.claude/hooks/routine-guard.test.mjs`, `.claude/hooks/settings.test.mjs`,
  `.claude/scripts/checkpoint.test.mjs`.
- Not checked: the PR #43 comment 6701970962 itself, the live branch-protection ruleset, the routines' live settings
  (no GitHub reads in this review); the gate (no code changes yet; it runs at build).
