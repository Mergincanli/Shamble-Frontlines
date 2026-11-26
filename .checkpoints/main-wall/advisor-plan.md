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
