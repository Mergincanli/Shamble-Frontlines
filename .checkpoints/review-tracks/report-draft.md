# Advisor report: review-tracks (2026-11-27, branch claude/confident-gates-tbnq38 for cs/review-tracks, base origin/main b8f02b01)
## Goal
Review effort follows what a change touches (a light or a full track, worked out by a script, in doubt full), and a
chat that built a pull request can merge it on his Advisor's merge check (PASS), with his go kept wherever his rules
need it. A change under content/source/ needs his go (SOURCE-GO (a)).
## Outcome
DONE on the branch: eight kit files applied, reviewed and verified. Open: his answer to PASS-FORGE (below), then the PR.
## Plan and approval
Plan (.checkpoints/review-tracks/plan.md, folded in below): the track script, the needs-his-go list ("His go keeps"),
two merge ways, the guard's merge-check rules. The advisor agent approved it after two revisions; he gave PLAN-GO (a)
(full version), MERGE-PROMPT (d) (replacing (A)) and WAKE-END (a); later ADDITIONS-OK (a) and SIZE-LINE (a).
## What changed (git diff --numstat origin/main...HEAD; 1029 added, 66 removed)
- CLAUDE.md (+39/-13): the plan and review steps name the track; the two merge ways; tier 1's row; "Review tracks".
- .claude/agents/advisor.md (+93/-42): content/source/ escalates; "His go keeps"; step 5's merge-check slot; the after-check; toolchain.
- .claude/agents/security-auditor.md (+2/-1): a Medium or above left open keeps needs-mustafa; a Low goes to the merge check.
- .claude/hooks/routine-guard.mjs (+58/-5): the merge-check line refused in every connector write and in plain gh forms;
  merges only through the github connector, never a subagent's; toolchain required; the day-session allow on a real PASS.
- .claude/hooks/guard.mjs (+4/-1): prints that allow only for a day session's PreToolUse.
- .claude/hooks/routine-guard.test.mjs (+233/-4): 30 new tests and a 31-mutant run.
- .claude/scripts/track.mjs (new, 269) and track.test.mjs (new, 331): the track, reviewers, his go floors, the words check.
Game symbols: +0 -0 (gate check 3), as planned.
## Evidence
- Gate: checks 1-4 and 5b-5d PASS; 5a smoke not run here (Chromium build mismatch), CI runs it.
- Suites, real folder: guard 1581/1581 (main 1551), settings 127/127, checkpoint 33/33, kept-logs 9 of 10 (as main),
  track 85/85 (fixture repositories for the git layer). Same counts on the tree merged with main (verifier).
- Mutants: track.mjs 58 of 62 caught (left: the diff-algorithm pin and the graft setting, shown by security-auditor's
  probes instead; two that cannot change the result); the guard's 31 new-rule mutants all caught.
- Real history: the words check on all 162 record entries at the base: every entry naming a saved file and a sha256
  passes (44); the track replayed in head mode on 11 merged PRs equals the plan-time track 11/11.
- Applied = reviewed: all eight files byte-equal to the reviewed copy (cmp, three times: after the applies, by the
  verifier, after taking main in at 583c85a0). Applies: 8d15e7bf..dcd8f563, one file per commit.
## Reviews
- Plan: the advisor agent and security-auditor (two rounds), then a quick look at (d). Pre-apply on the built copy:
  security-auditor, test-engineer (18 findings) and the advisor agent: all REVISE, all fixed in the copy.
- Re-checks of the fix passes: the advisor agent and security-auditor found a skipped saved-file compare (High), wrapped
  headers, folder twins, a tag or local git state moving the base, a third saved-file form, and merge-name shapes: fixed.
- Build review: the advisor agent APPROVE; security-auditor APPROVE; verifier PASS with notes (size figure, repo map).
- Open, follow-ups: REPO-PIN (Medium: pin this repository and a plain PR number on the allow path), UNSCANNED-VERBS (Low),
  the diff-algorithm pin's test (Low), SA-RT2-1's guard-file half (Low). Open for him: PASS-FORGE.
## Decisions and deviations
- Departures, all approved: twins only on the change's own paths; text triggers only under packages/, tools/, legacy/;
  "no saved file named" listed, not floored; the record's newline and header forms accepted (ADDITIONS-OK).
- A pre-existing merge-scope gap (security-auditor SA-RC-5 and SA-RC-9) was closed here, not deferred.
- Main came in through a worktree (the main folder parked on main), the advisor agent's method; docs written there.
- Process slips, owned: checkpoint c6f530a named a gap still open on main in one line (reworded a minute later in
  cdf1624; history kept); two apply rows took two edits each; size reached 1029, past the stop line (his SIZE-LINE (a)).
## Risks
1. PASS-FORGE: until the guard change-set, a session could fake a PASS through a route that runs without a prompt.
2. Shell-route forms of the merge-check line (R4-1's class): the guard change-set.
3. Binary files are now a go floor: PRs adding test vectors or recordings wait for him.
4. The allow's first real merge in a live session is untested (plan risk 9).
## Proposed next step
The guard change-set: R4-1's fix, the script runners off the no-prompt list, REPO-PIN, GUARD-KIT and FEWER-ASKS.
## Decisions needed
- **PASS-FORGE:** should a merge skip your tap on a PASS comment while a session could still fake one through a script
  that runs without a prompt? Why: until the guard change-set, only your Advisor's own list of PASS comments would catch
  a fake. (a) accept it until the guard change-set, its fix first there; (b) take the scratch-script runs off the
  no-prompt list now (one settings edit, one prompt; scratch scripts then ask each time); (c) turn the no-prompt merge
  off until then (one guard edit, one prompt). Recommended: (a) (the advisor agent too); security-auditor prefers (c).
## For Mustafa
1. Answer PASS-FORGE.
2. After the PR opens and the advisor agent approves its head: his Advisor's merge check, then your go on that head.
3. Later, one click: add toolchain as a required check in "Shamble Ruleset 1".
