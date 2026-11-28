# Advisor: plan review of U4r-F2 (judge-memory split before P0)

## My plan
1. Read the plan (u4r-f2-plan.md), my sets ruling (advisor-u4r-sets-r1-full.md), F1 plan + my F1 review for form.
2. Read c-u4r-runs.mjs judged-set loop and judge call (:414-445 and around), committed head.
3. Read eval.rs / figures.rs: how a verdict for a bite (pre:PM-0001-2 for -17) is computed; does it depend on other runs in the same judge call?
4. Read c62h record README + verdict files (bites-only judging precedent).
5. Read .gate/u4r-judge-apart/*.log memory figures; check the plan's numbers.
6. Check layout, exists-check, floor order, condition 1 endings, INF-R2-1 re-check, JDG-ONLY-1 interim rule.
7. Answer Q1-Q5, verdict, size, Mustafa.

## Checks (a line per check)
- Read: plan, sets ruling, F1 plan + my F1 review, the script at HEAD 9bbceb7 (spike tree 866c323b; HEAD's last commits are .checkpoints only).
- Script :414-445: one judge call over <logs>, verdict.json, runs.txt `judge: <stdout|stopped|exit>`; results/ made only AFTER the judge (:432), unchanged("the judge") at :422 before it.
- Judge cross-run reads (eval.rs): set_of(all, run.set) feeds (1) reference() for Threshold::Against, (2) figures' `set`, (3) pooled checks, (4) ws-practice via ctx.all. -2/-17/-57: no Against, no pooled key, no ws-practice; figures used (everyone_in_view_min_share :24, decoded_hash_mismatch_count :542, decoded_hash_compared_count :563, cross_arch_mismatch_count :571) all take `_set` unused; decoded(run) reads the run only. Pre rule (eval.rs:977-1007) evals -2's checks on `&[run]`. eval_bite (:1386-1502): per-run judge_run with rules_of(matched parts) -> pre:PM-0001-2 on the bite run alone; no expect-invalid `rule` on these bites. So per-part and per-bite verdicts are identical in one call or two. CONFIRMED.
- Parts select role measured (part_select :1154); bite runs (role bite) never enter a part; measured runs never enter a bite. Bites call: parts "0 of 3 valid runs" NOT-RUN; measured call: bites "no run yet". Overall line partial in each file (as plan item 5).
- --only retains marks before judging (main.rs:84-96); pre lookup uses the filtered list, so PM-0001-2 must be listed in both calls: listedOf(profile) has it. JDG-ONLY-1 holds.
- read_logs is recursive (lib.rs log_files), reads .jsonl only; marks.json at <logs>/marks.json is not read as a log. Each call's --logs <logs>/<group> sees exactly its group.
- npt-lab --out: create_dir_all (scenario.rs:771); the out path is NOT in any header (grep), so the logs' bytes do not depend on the folder.
- (Process slip: the block above was appended with a shell heredoc, against my limits; from here on Edit only. Content unaffected.)
- c62h record: bites-only judging with r2's npt-judge (65acacfe), --only PM-0001-2,PM-0001-17: both bites shown, runs valid, -2 NOT-RUN with no runs. r2's runs ALL share set "u4r-r2": p0 (role measured, P0, marks -2,-17,-57, 24 MiB), tick16 (bite, -2,-17, 3 MiB), whole-as-lost (bite, -17 only, 3 MiB). So a same-set combined-vs-split check on the real judge is possible at ~30 MiB (better than the plan's 5.3: P3's measured-only judging already exists as P3's own verdict.json; the plan's two halves never meet in one call).
- Memory: 178,922,504 B = 170.6 MiB -> 641.1; 543,608,417 B = 518.4 -> 1,507.8; 859,984,524 B = 820.2 -> 2,260.6 (logs in .gate/u4r-judge-apart). Line through the two ends: 215.8 + 2.493x; middle predicted 1,508.2 vs 1,507.8. Plan's "215 + 2.49x" HOLDS. Reading alone 426.2 / 1,284.8 / 1,851.4 (2.50x, 2.48x, 2.26x); judging overhead above reading 215 / 223 / 409 MiB.
- Groups (runner rates): measured 3 x 509.1 MB = 1,456.6 MiB; bites 3 x 509.1 (-57's, header lists -57) + 3 x 56 MB (-17's) = 1,616.8 MiB; total 3,073.9 MiB (plan 3,073.3: rounding). F=1.5: 2,184.9 -> 5,656 MiB (5.52 GiB); 2,425.2 -> 6,254 MiB (6.11 GiB). F=2.0: 7.29 and 8.07 GiB. One call F=1.5: 11.42 GiB. Plan's figures HOLD. Break-even F (10 GiB): bites call 2.49, measured 2.76. B1 ratio 820.2/545.3 = 1.504 HOLDS.
- check.mjs run() (:1909-1922) is spawnSync: the two calls are sequential, never together. OOM kill -> status null -> "exit by a signal (no verdict)".
- RISK FOUND: P2 runs now (pid 12892, commit ab10f058a828, run l1-p2-s2 at 19:04 UTC). Its INF-R2-1 re-check reads `git status --porcelain --untracked-files=no -- spikes/netcode-proof`: an uncommitted Edit of c-u4r-runs.mjs would stop P2 before its judge. Plan section 6 does not sequence the build after P2. -> condition.
- RISK FOUND: plan 2.3 copies each verdict into results/ "on its own exit 0" and re-checks before each call; if the re-check before call 2 throws after call 1's copy, results/ holds verdict.json and no runs.txt (today a re-check stop leaves no results/ folder: results made at :432 after the judge). -> condition (results/ made and copies only after both calls).
- r2 imports spikeChanged and spikeReading from c-u4r-runs.mjs (c-u4r-r2.mjs:54, :170). INF-F1-5's fix needs both scripts' sh() (trim) -> not a c-u4r-runs.mjs-only change; and it is the re-check P0 depends on -> defer. r2 must be in the harness compare (import unchanged).
- INF-F1-1 sentence: comment only, words fixed in advisor-u4r-f1-head-full.md:33 -> ride.
- RUNS-LINE: needs verdict JSON parsing after a 70-min set (a throw there would lose runs.txt) and -2's part is "SS runs" not "L1 P0" -> defer to the change after P0 that carries RATES-1; F2 gets constant labels on the two judge lines instead.
- Evidence gaps: no gate run named (F1 had gate --no-smoke + CI); compare only P3, not precheck/P0/P2/B1/r2; no both-calls-fail staging; no check that P0's runs.txt shows the same binary and marks sha256s as P3/B1/P2.
- Corrections to my lines above: the Pre rule is eval.rs:958-996 (the plan's citation is right; my ":977-1007" was off); total projected is 3,073.3 MiB (6 x 509.1 + 3 x 56 MB = 3,222.6 MB), as the plan says.
- report_values (eval.rs:1504-1556): per part's valid members only; set used only by figures (unused here) -> measured-alone report[] equals combined. Include in the real-judge compare.
- always_rules(run) and the -17/-57 checks: no ws_bound, so ctx.all (practice::find) is never read.

## Verdict: APPROVE WITH CONDITIONS (9 conditions; findings F2-1..F2-6)
- Q1 (a); Q2 (a); Q3 (a); Q4: INF-F1-1 yes, INF-F1-5 no, RUNS-LINE no (fixed label line instead), RATES-1 no; Q5 (a).
- Size: about 50-70 insertions and 10-15 deletions in c-u4r-runs.mjs with the conditions (plan's 40-60 honest).
- Mustafa: nothing (tier 1, spike branch, his heads-up (2) of the sets ruling already covers "a small fix before that set").
