# Skeptic r1, D2 (IE-1 Medium + SA-4 Low): `toolchain` is not a required check on main

Plan: (1) re-query GitHub's API without a token (rules for main, the ruleset, branch, protection); (2) re-read the
report at 4d78ad6 (:140-151 Risks, :169 REQUIRED-CHECK, :201-206 K1-GO, :212-213 For Mustafa item 3) and plan.md
Part 10 risk 2 (:654-657); (3) re-read the kit lines the finding cites (guard mergeCheck :1486, CLAUDE.md :119 :130,
advisor.md :157 :163 :224 :273-276 :323, STATUS.md :22) and grep for any it misses; (4) confirm toolchain's runs on
4d78ad6 from the API; (5) judge severity and the fix. Head under review 4d78ad6; worktree HEAD differs only in
.checkpoints/ (git diff --name-only 4d78ad6 HEAD, outside .checkpoints/: empty).

## Checks
- C1 API, no token, 2026-11-12 (saved under .gate/skeptic-D2/api_*.json): GET rules/branches/main -> 200: one
  ruleset, 25069670, rules deletion, non_fast_forward, pull_request (0 approvals), required_status_checks
  [gate, smoke] (integration 15368, strict false). GET rulesets -> only 25069670 "Shamble Ruleset 1", active,
  include ~DEFAULT_BRANCH, updated 2026-10-17. GET branches/main -> protected true, classic protection enabled
  false, contexts []. GET branches/main/protection -> 404. bypass_actors [] (anonymous read; may be redacted).
  HOLDS: `toolchain` is not a required check on main.
- C2 report at 4d78ad6 (git show to .gate/skeptic-D2/report-4d78ad6.md): it is a draft ("The review round ... added
  in step 7", :3-4; "## Reviews (step 7)", :121-122). :169 lists REQUIRED-CHECK in "Findings for 2.3" with its answer
  deferred to "step 7's infra review" (so "left open" was by design; the review now supplies the answer). Risks
  :140-151: four bullets (sudo, pins unguarded, not a sandbox, TOOLKIT-RESOLVES-UP); none says a red toolchain blocks
  no merge or is no rollback trigger. K1-GO :201-206: recommends (a) "It is green on every head..."; no word on
  required/merge/rollback. For Mustafa 3 :212-213: "...CI's gate and smoke are green on it, say "merge #48"". HOLDS as
  stated (all three gaps are real at 4d78ad6).
- C3 plan.md at 4d78ad6 (unchanged since) Part 10 risk 2 (:654-658): "A red `toolchain` job on main is not a rollback
  trigger: CLAUDE.md's Rollback names `gate` and `smoke`. Until 2.3's one required check, a red `toolchain` on main is
  fixed forward, plan first; advisor.md:322 already forbids merging with "failing or pending checks", so the job blocks
  merges by the rules, though not by branch protection or the guard". HOLDS: the report dropped a risk its own approved
  plan named.
- C4 kit lines (CLAUDE.md, advisor.md, STATUS.md, routine-guard.mjs: identical at 4d78ad6 and worktree HEAD):
  CLAUDE.md:119 Rollback names only "`gate` or `smoke` red on main"; :130 merge rule "with the gate and smoke checks
  green". advisor.md:157 "merge only when `gate` and `smoke` are green"; :163 routine reads `gate` and `smoke`;
  :224 likewise; :273 "Main breaks" names only gate/smoke; :323 Never "Merge with failing or pending checks, or unless
  the `gate` and `smoke` jobs both passed" (the only text that covers a red toolchain). routine-guard.mjs:1425 calls
  mergeCheck for every session ("merge fails closed for every session"), and :1486 loops over ['gate','smoke'] only.
  HOLDS: nothing mechanical (ruleset or guard) stops a merge on a red toolchain; only advisor.md:323's text does.
- C5 completeness of IE-1's 2.3 list (grep -rnw smoke over CLAUDE.md, .claude/, .github/, STATUS, START_HERE,
  tools/README): it names CLAUDE.md:119 :130, advisor.md :157 :163 :224 :273 :323, STATUS.md:22, the guard's
  mergeCheck. It MISSES advisor.md:294 (Revert's "Pinned" test: "the red `gate` or `smoke` run on main names that
  commit"), routine-guard.mjs:6 (header comment) and :990 (the PR Advisor's deny text "CI's gate and smoke results
  stand in"), routine-guard.test.mjs (5 lines name smoke), and START_HERE.md:87 (describes CI's jobs). A small gap
  in a 2.3 to-do list: better phrased as "every kit line that names `gate` and `smoke` as the checks (grep at 2.3)".
