# Advisor: plan review of MAIN-WALL (2026-11-26)
Plan `.checkpoints/main-wall/plan.md` at ed2dd094 (base main 4a62873e), from main's `.claude/agents/advisor.md`.
Ids only: no gap detail and no test detail in this file (his 2026-11-25 20:35 item 3). The brief's fixes went to the
caller in the hand-back, not here.

## Verdict: REVISE (text only; the design W1 holds)
The rule design, the stop gates and the tier are right. The pushed plan text needs six changes, and the unpushed
brief needs fixes, before the plan goes to Mustafa. On re-review the advisor checks only these items.

## Risk tier: 2
Repository settings (two rulesets, read-only settings checks). Not 3: rulesets and Actions on a public repository
cost nothing (GitHub docs, about-rulesets: "available in public repositories with GitHub Free"; Actions billing,
cited in the design), and the one act that cannot be undone, deleting the throwaway, is his own click on a repository
that holds no data. A private throwaway would need GitHub Pro (same docs sentence): that option is tier 3.

## The five questions
1. His conditions: met, except two: "one or two change-sets" against the rule-file plan by name (C2) and his
   15:05 item 4's R4-1 question (C3). GUARD-KIT's parts: read right (his 15:05 item 4 replaces his 13:20 item 3).
   Measurement in a separate one-repository chat or by his clicks, results by paste: right.
2. Design against the docs: sound. Re-read by the advisor today: available-rules line 19 (Restrict updates), 23
   (deletions on by default), 151 (default branch and force pushes); about-rulesets line 68 and 75 (aggregated, all
   rules apply); creating-rulesets line 64 and 72 (admin role, "For pull requests only"); REST rules (bypass_mode
   `exempt`: "rules will not be run"); permission levels for a personal account (owner and collaborators only).
   Cross-ruleset bypass is in no doc: T1 decides it, as the plan says. Stop gates right: T1 first, in each mode; no
   real click before the results, the advisor's and security-auditor's review of them, and his go.
3. Gap-detail discipline: plan and checkpoint hold ids and the rule's guarantee only: acceptable. Keeping the design
   and the brief unpushed: right. The brief is safe for a one-repository chat (no secret; its workflows act only in the
   throwaway) but incomplete (see hand-back). New: the throwaway is public, so its test files and
   run logs are visible (decision THROWAWAY-PUBLIC).
4. Tier and process: tier 2, reviewers right in kind, but security-auditor must read the design and the brief
   before he pastes anything (C6). The docs pull request as T10's day-session merge: fine; its STATUS line marks the
   PR Advisor half of T10 open until that merge happens.
5. Before it goes to him: THROWAWAY-PUBLIC (below).

## Conditions (all text; numbered)
- C1 (Medium): plan section 5 says this chat cannot read rulesets; the design records two read-only `gh api` reads
  of this repository and its ruleset today (settings allow that path). Say which is true for this chat, and assign
  T0's real-repository read-back in T10 and click 4's fallback to match. If the session's rule is "gh for search
  only", the report records the design's reads as a departure.
- C2 (Medium): name the rule-file plan (the review tracks) and say: two change-sets; its merge-on-green rule takes
  effect only after click 11 and T10 pass (his "the wall is built first either way").
- C3 (Medium): record whether security-auditor was asked if R4-1 can wait behind MAIN-WALL (his 15:05 item 4);
  if it says no, the two lines go to him before the measurement starts.
- C4 (Medium): click 6 gives him test ids only. Give him an unpushed test card (steps, expected result, stop line
  for each of his own tests), handed over with the brief in chat; the plan names it.
- C5 (Low): name the source of guard-wrappers' F2-F7 (not defined anywhere on main) and confirm they are docs-only;
  any that touches a kit or gate file moves to the guard change-set.
- C6 (Medium): security-auditor reviews the design, the brief and the card before anything goes to him, and the
  results before click 11; findings addressed or deferred in writing.
- C7 (Low): the checkpoint cites the other lane at 7aadfc17; it is at d49e8003 now. Note the SHA read.
- Brief and card: the fixes in the hand-back; keep a copy under `.gate/main-wall/` (the scratchpad can be cleared).

## Decisions needed (for Mustafa, with the plan)
- THROWAWAY-PUBLIC: the measurement's test files and run logs sit in a public repository under his account until it
  is deleted. Why it matters: his 20:35 item 3 keeps the gap's detail out of pushed notes, and these files are
  pushed, though not to this repository. Options: (a) public, with a neutral name and neutral file names, deleted as
  soon as the results are reviewed (re-created if a re-test is needed), not kept until T10; (b) private, which needs
  GitHub Pro for rulesets (a paid plan: tier 3) unless he already has it; (c) drop the tests that need those files,
  which leaves "only his account" without its main proof. Recommendation: (a).

## For Mustafa
After C1-C7: his go on the plan; then the clicks as listed. Note: the test chat runs without the guard (no repository
hooks there); its limits are the brief's words and the chat's one-repository binding.

# Advisor: re-review of MAIN-WALL's plan (2026-11-26, plan at eedf59d3)
Read: the plan at eedf59d3, security-auditor's public notes, its unpushed detail (second pass wins), the brief and his
test card (unpushed, `.gate/main-wall/`). Ids only here.

## Verdict: APPROVE, with conditions 1-4 applied before the plan goes to him (no further advisor review needed if
applied as written; any other change comes back)
Risk tier stays 2. W1, the stop gates and the order hold.

## My conditions C1-C7: all met
C1 (this chat does not read rulesets; T10's read-back and click 4's fallback are his; the design's reads recorded as a
departure), C2 (two change-sets; merge-on-green only after click 11 and T10), C3 (security-auditor's answer recorded),
C4 (his card exists), C5 (F2-F7 named, source the advisor's final-head review of #73, docs only), C6 (review done),
C7 (SHA read named). THROWAWAY-PUBLIC (a), his answer: applied (neutral names, deletion after review).

## security-auditor's findings: SA-MW-1 to SA-MW-12 read as met, except two gaps below
SA-MW-2, -3 (click 0 first; GUARD-SETTINGS as his decision), -4, -5, -6, -7, -8, -10, -11, -12: met in plan, brief and
card. SA-MW-1: met in the brief's pass mark, but see condition 1. SA-MW-9: met in the plan; see condition 2.

## Conditions
1. (Medium) T4-BASELINE is his open decision, but the brief already runs the baseline route as a pass mark, and
   the clicks it needs (switch the second sandbox ruleset off, then back on) are in neither the plan's click list nor
   his card. Mark the brief's baseline route "only if Mustafa chose T4-BASELINE (a)", and add those two clicks, each
   with its undo, to section 4 (under click 6) and to his card.
2. (Low) Section 5's TECHNICAL note also carries SA-MW-9's invariant (the auditor asked for the same line there).
3. (Low) Section 4 B's opening line also names security-auditor's review of the results (section 5 already says so).
4. (Low) The brief's contents-write control in job `merge` uses a fixed branch name; a second run would get "already
   exists", which proves nothing. Use a name unique per run.

## Gap-detail discipline (pushed files)
The plan and both notes files hold ids, classes and the rule's guarantee. Low: section 7's T4-BASELINE wording says a
little more about T4's actor than the rest of the plan; the class is already in pushed notes, so this alone triggers
no push. If the plan is pushed again anyway, shorten it to "a control with the second ruleset off, so the refusal is
shown to come from it; cost: one more public record in the sandbox", and give him the full reason in chat.

## Not checked by me
The security-auditor's answer on R4-1 (taken as recorded in the plan); live rulesets, the Claude app's
Administration level and the proxy's write policy (click 0 and the measurement decide them).

## Decisions needed (go to him with the plan; both as written in plan section 7)
- T4-BASELINE: recommended (a). I agree: without it, T4 can end "unproven" and "only his account" has no proof.
- GUARD-SETTINGS: only if click 0 shows write on Administration; recommended (a). I agree.

## For Mustafa
After conditions 1-4: his go on the plan, his answers on T4-BASELINE and (if needed) GUARD-SETTINGS, then click 0.

# Advisor: results review of MAIN-WALL before click B (2026-11-26, mode Exempt)
Read: the results (unpushed, `.gate/main-wall/`), the checkpoint, the brief, his card, the plan, my earlier verdicts,
the other lane's security-auditor notes at 7aadfc17. From main's `.claude/agents/advisor.md`. Ids only here.

## Verdict: APPROVE click 11 (mode Exempt): conditions 1-2 before it, 3 at it, 4-6 after
Risk tier 2. T1 PASS and counts (his account merged in the same mode). T4 PASS on a same-head control, graded from
reply text by his fallback (no rule record); detail in the unpushed results. T5, T7 PASS. Click 10 dropped: accepted
(click 0 is from today).

## Dropped tests (at his request)
- T1e: inferred, not measured; not needed on the real repository.
- T3a (his own web merge): not a precondition. Optional, least work: one screenshot of the docs pull request's merge
  box when green (no click), before the session merges it. STATUS records it as not measured until first real use.
- T2 second account: unmeasured for his personal Claude account (same GitHub user); covered by the first merge from
  a personal-account chat. The PR Advisor half of T10 does not cover it (same account as the test chat).
- T6 (optional), T9: not needed. T8: no rule record; graded from reply text by his instruction; recorded as such.

## Conditions
1. (High, process) security-auditor reviews the results before click 11 (plan section 4 B, tier 2), including the
   test chat's two deviations and condition 4's release point; any Critical or High open stops click 11.
2. His go on click 11 in his own words in this chat.
3. Click 11 exactly as measured (plan section 2's settings, mode Exempt); "Shamble Ruleset 1" untouched.
4. The other lane: its builder may resume work that stores no credential once click 12's read-back matches
   condition 3. The token sitting itself waits for T10's day-session half (the first day-session merge under the
   wall), not for the PR Advisor half: only a refused day-session merge leads to click 11's undo. This narrows the
   plan line he approved ("waits for click 11 and T10"): say so to him.
5. (Low) Click 0's missing Actions > General screenshot rides with click 12; not a precondition.
6. The docs pull request records every deviation and dropped test, quoting his own words from the chat exactly;
   STATUS keeps T10's PR Advisor half and the personal-account merge open until each first happens. Then click 9.
