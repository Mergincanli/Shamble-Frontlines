# Skeptic r1, D3 (IE-2 + TE-1 + TE-2, all Low): "green on every push/head"; "about 20 seconds"

Plan: (1) re-read the report at 4d78ad6 (:60, :96 E2, :106 E4c, :108 E5, :206 K1-GO) and confirm the worktree's HEAD
changes no line outside `.checkpoints/`; (2) re-pull every Actions run on the branch from GitHub's API without a
token (into .gate/skeptic-D3/), tally by workflow, event, SHA and conclusion, and check each cancelled run the
findings name; (3) check each of the four reverts has no completed run and no pull_request run, and whether a later
completed run exists on an equal tree (git diff, `git rev-parse <sha>^{tree}`); (4) re-time every green toolchain job
from /actions/runs/<id>/jobs (started_at to completed_at) and the rustup step, for E2's 14 jobs and for all green
jobs; (5) judge each claim and each proposed fix wording, at which severity, and what the fixes would break.

- Check 1 (report text): at 4d78ad6 the phrases are where the findings say: :60 "smoke job was green on every push",
  :96 E2 "The job takes about 20 seconds", :106 E4c and :108 E5 "green on every push", :206 K1-GO "green on every
  head". The worktree's HEAD 1ebb1ca changes only `.checkpoints/` files since 4d78ad6.
- Check 2 (runs, GitHub's API through the session's proxy, which authenticates: rate limit 15000 an hour; 2026-11-12;
  .gate/skeptic-D3/runs-p1.json): 82 runs, run_attempt 1 for all.
  gate: 37 success, 5 cancelled, 0 failure. toolchain: 22 success, 14 failure, 4 cancelled. Exactly IE-2's and TE-1's
  tallies. Every one of the 37 green gate runs has both jobs, gate=success and smoke=success (none skipped).
- Check 3 (cancelled runs, jobs API): the reverts 2ee2bff, 5c64404, b60a996, 00cee0b each have exactly two runs, push
  only, both cancelled (toolchain 42892028850, 42892220355, 42893280538, 42893505538; gate 42892028937, 42892219915,
  42893280696, 42893505392), and no pull_request run. Each revert was committed 2 s before the next plant and each
  run was superseded by the next plant's push run created 2 to 4 s later; concurrency cancel-in-progress is at
  toolchain.yml:16-18 and gate.yml:15-17 (group = workflow + ref, so push and pull_request runs never cancel each
  other). M4's 2d9766f gate push run 42893510654 was cancelled with no job started (superseded by 66d9888's push run
  at 20:16:48); its pull_request twin 42893520512 is green. All IDs in IE-2 and TE-1 match.
- Check 4 (trees): `git diff <plant>^ <revert>` is 0 lines for all 7 pairs, with and without ':!.checkpoints'. The
  four cancelled reverts have the same full tree id as 66d9888 (2499a670886a...), which is green on both workflows and
  both events (toolchain 42893708734, 42893718519; gate 42893708791, 42893718283). Against fb17451 they differ only in
  `.checkpoints/` (26 lines). So the substance holds; "their trees equal fb17451's" (IE-2's fix) is true only outside
  `.checkpoints/`; 66d9888 is the exact match.
- Check 5 (failures): the 14 failed toolchain runs are the 7 plants x push and pull_request; each failed at the step
  the report names (M1a, M1b, M4, M5 at the network-cut step; M2 at the pin check; M3, M6 at the fetch step). None
  cancelled.
- Check 6 (timings, jobs API started_at..completed_at): all 22 green toolchain jobs 19 to 45 s, median 24.5, mean
  25.8. TE-2's 16 (E2's 14 + 4d78ad6's 2) reproduce value for value: median 24, mean 26.1. E2's 14 alone: median 23.5.
  Push jobs median 23 (mean 22.4), pull_request jobs median 28 (mean 29.3). 19 s is the minimum (2 of 22 jobs); 5 of 22
  are at or under 20 s (re-computed by script, analyse.js and green-toolchain-rows.json in .gate/skeptic-D3/).
  The rustup install step 8 to 14 s (median 10) and the network-cut step 4 to 6 s, in both sets.
- Check 7 (head): 4d78ad6 has 6 check runs, all completed success (gate, smoke, toolchain, on push and pull_request).
  The PR's description (fetched, head now 32386e1, draft) repeats neither claim; no other tracked file at 4d78ad6
  repeats them. So the fix's scope is the report's :60, :96, :106, :108, :206. Earlier reports used the careful
  quantifier ("its smoke green wherever it ran", 2026-11-10-status-diet.md:35), so the convention exists.
- Check 8 (rustup): the head's push job log (job 131510223737): "info: downloading 6 components" at 21:37:10.7, the
  step done at 21:37:20.8; toolchain.yml has no Rust cache step. IE-2's "6 components each run, no cache" holds.
- Check 9 (does the number matter): ADR 0002's budgets file takes "starting values from the first spikes'
  measurements"; PM-0002-56 measures CI time at p50 (the 3rd of 5 runs) against 10 minutes. So no decision rides on
  20 vs 24 s (TE-2's "the gate v2 work needs this number to be accurate" is overstated), but the median is the
  project's own CI-time statistic, which favours TE-2's median wording over "about 20".
- Check 10 (aside, not a finding): checkpoint commits marked [skip ci] get no runs (1ebb1ca, 32386e1, both after
  4d78ad6), so the fix should keep "completed" in its wording, as both proposals do.
- Check 11 (the timing depends on the measure; .gate/skeptic-D3/measures.js): over the same jobs, the jobs API's
  started_at..completed_at gives median 24.5 (all 22; 19 to 45 s), 24 (E2+head 16), 23.5 (E2's 14). The steps' own
  times (sum = span, first step start to last step done) give median 21.5 (all 22; 16 to 42 s), 21 (E2's 14 and
  E2+head 16). Log spans of 4 jobs match the step span (22.0/22, 20.2/21, 41.5/42, 16.1/16 s); the API adds 1 to 4 s
  over the 22 (2 to 4 in those four), mostly after the last step (e.g. 131510223737: last step done 21:37:29, job completed 21:37:32). Run time
  (run_started_at..updated_at): median 28 (all 22). E2's sentence sits after "Each log: ...", and on the logs' clock
  the median job is 21 s: "about 20 seconds" is typical there, and the fastest is 16 s. So the timing half holds only
  on the API's measure; what holds on every measure is the spread "about 20" hides (up to 42-45 s; pull_request jobs
  slower: step-time median 27 vs 21 s for push).
- Check 12 (overlap): D2's fix also rewrites K1-GO (:201-206: tell him a red `toolchain` blocks no merge until 2.3);
  the two edits to :206 should land as one rewrite.

Verdict: holds, Low (the quantifier half firmly; the timing half only on the jobs API's measure). Every quantifier
site is false as written (four revert pushes never completed a run; M4's gate push run never started). The substance
holds: every completed gate/smoke run green; the only red runs are the 14 planted ones, each at the named step; the
reverts' tree is byte-identical to 66d9888's, green on both workflows and events. "About 20 seconds" is near the
minimum by the API's start and end (median 24) but the median by the logs' own clock (21; fastest 16). Wording, not
evidence that does not hold: Low.
Fix: right in direction. IE-2's misses :60 and :206; TE-1's covers all four quantifier sites. Both say the cancelled
trees "equal fb17451's": exact only for 66d9888 (same tree id); against fb17451 only outside `.checkpoints/`. At :206
the toolchain sentence must keep the planted carve-out in the same clause ("green on every head whose runs completed"
alone is false for the 7 plants), and it lands together with D2's K1-GO rewrite. E2: whatever numbers it gives must
name the measure (TE-2's and IE-2's are the API's; a reader timing the logs gets 16 to 42 s, median 21), state the set
(the jobs E2 lists; IE-2's 22 include 8 jobs E2 does not cite; its "median 25" is 24.5) and be recomputed at the final
head. Breaks nothing (report wording only; K1-GO's options and recommendation unchanged); a Low rides the D1/D2 fix
pass, no push of its own.
