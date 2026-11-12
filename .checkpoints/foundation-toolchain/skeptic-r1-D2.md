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
- C6 CI on 4d78ad6 (GET commits/4d78ad6/check-runs, total 6): toolchain success on push run 43312017234 (job
  131510223737) and pull_request run 43312027194 (job 131510257155); gate and smoke success on both. SA-4's evidence
  HOLDS. PR #48 today: open, draft, head 1ebb1ca (an autosave), mergeable_state blocked. Open PRs: #48 and #40 (#40's
  head is now a5784c21, not IE-1's 8eb5d09: drift in a "not verified" aside, immaterial to D2).
- C7 "until 2.3": ADR 0002 :1421-1422 (foundation checklist item 2): "After the gate and CI (phase 2.3): in the
  repository's settings, name the single required check ... in main's branch protection"; part 9 (:642-643) "one
  aggregate required check"; PM-0002-57 (:1323). So "until 2.3" matches the ADR, but it lands only when he sets it and
  the kit lines change (each asked of him); the rollback rule changes only by a kit edit.
- C8 IE-1's cost note vs GitHub docs ("Troubleshooting required status checks", docs.github.com article API, read
  2026-11-12): :94 "If `build` is required, the pull request is blocked with "Waiting for status to be reported.""; :62
  a workflow skipped by path/branch filter or commit message: "Associated checks stay in a "Pending" state and block
  merging". HOLDS: requiring `toolchain` now would block PRs whose head never ran it, and removing toolchain.yml while
  it is required would block every PR.
- C9 kit status of IE-1's 2.3 list: routine-guard.mjs:35-48 protectedRel covers CLAUDE.md, .github/, .claude/ and
  GATE_FILES (:28-29, tools/ gate files only); docs/STATUS.md is not protected (E9: mergeAllowed). So "each a kit edit
  asked of him" is wrong for STATUS.md:22 (a docs-keeper edit).
- C10 precision: the report's Risks bullet 2 (:143-144) leans on "CI's `--locked` steps still fail a lock that does
  not match" as the pins' mitigation; with toolchain not required, that failure stops a merge only through
  advisor.md:323's text. Same gap as D2 (context for its severity, not a new finding). The deduped line's "blocks no
  merge" drops IE-1's "mechanically": by advisor.md:323's text a red toolchain does forbid an approved merge, so the
  fix must keep "mechanically" (or "not by the ruleset or the guard").

## Draft verdict (before the advisor call)
HOLDS, Medium. Every factual leg re-checked (C1-C6). Medium, not Low: the report drops a risk its approved plan named
(C3) and the K1-GO decision entry omits what the job does not enforce, which bears on his decision; small reach
(until 2.3; v2 code PRs only, which the PR Advisor cannot merge; advisor.md:323's text still forbids merging with
failing checks; row :169 flags the question as open). SA-4's leg alone is Low (the advisor reviews every pushed head).
Fix: right in substance, report text only, breaks nothing; refinements: keep "mechanically" and carry plan risk 2
whole (both halves, line :323 on the branch, not :322); the 2.3 list misses advisor.md:294, guard :6 :990 and its
tests (better: a grep at 2.3), and STATUS.md:22 is not a kit edit; SA-4's "merge check" must mean the session's own
read of all check runs (both toolchain events) on the final SHA, since the guard cannot change here.

## After the advisor call
- C11 who advisor.md:323 binds: :33 the advisor merges "only when running as the PR Advisor routine"; :216 "Called
  from any other session, give the verdict only"; the routine's act needs "all checks green" (:200) / "every check is
  green" (:203); :323's verb is Merge. The routine cannot merge code or CI PRs (E9: not mergeAllowed), and those are
  the PRs that can break toolchain; they merge from a day session under CLAUDE.md:128-131 ("with the gate and smoke
  checks green"; :92's "Once approved with checks green" is unspecific). So plan risk 2's "advisor.md:322 already
  forbids merging ... so the job blocks merges by the rules" overstates on the day-session route (#48's and every v2
  code PR's): there, not even text requires a green toolchain. Supports Medium; makes SA-4's item-3 fix necessary; the
  fix must reword plan risk 2, not copy it.
- C12 guard tests: routine-guard.test.mjs :45 (fixture default checks gate+smoke), :144 'merge, no smoke' DAY, :145
  'merge, gate failed' DAY, :364 'merge, gate still running' DAY change with mergeCheck; :2163 is unrelated
  (tools/smoke.mjs path). The DAY cases confirm mergeCheck runs for day sessions.

## Final verdict
D2 HOLDS, Medium (SA-4's leg alone Low). Fix right in substance, report text only, breaks nothing (item 3 stricter
than CLAUDE.md:130, allowed); needs: plan risk 2 reworded for the day-session route (C11), K1-GO's sentence naming
what does not require the check, "until 2.3" tied to his settings step and the kit edits, advisor.md:294 added to the
2.3 list (guard :6 :990 and tests :45 :144 :145 :364 too; STATUS.md:22 not a kit edit), SA-4's merge check as the
session's own read of both toolchain runs on the final SHA, and Risks bullet 2 (:143-144) pointing to the new bullet.
