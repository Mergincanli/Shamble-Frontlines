# Advisor: review of the pushed head of U4r-F2 (791bc55), 2026-11-14

## Plan
1. git: 791bc55 (show, stat, files), later commits on the branch (only .checkpoints/?).
2. The diff against conditions 1 to 9 (plan review advisor-u4r-f2-plan-full.md) and Q1 to Q5.
3. Evidence: .gate/u4r-f2/logs (summary, sources, staging logs, redact, gate-no-smoke, edited-copies), real/, harness/, scratch/.
4. infra-engineer review (INF-F2-1..3) and the card's item (6).
5. Card NOW block: condition 7 figures (fit, F, projected peaks, break-even F), R, steps before P0. Recompute.
6. CI on 791bc55 via GitHub (if reachable).
7. ADV-Q, FP-P2 (read room.jsonl/lab.jsonl of u4r-P2 myself), PRE-P0.
8. Verdict, next steps, Mustafa.

## Checks (a line per check)
1. git: 791bc55 touches c-u4r-runs.mjs (+52/-22 in that file per the message; stat 74 lines) and 3 .checkpoints files. Later commit 208d7dc touches only .checkpoints/netcode-proof-test.md. OK.
2. Code vs condition 2: judgeCall runs unchanged(what) first; calls array built left to right (measured, then bites); a throw in either unchanged() propagates before mkdirSync(results). OK.
3. Code vs condition 3: (a) judge lines OK; (b) SPLIT_LINE equals the condition's text word for word, above the two lines; (c) raw-logs line names "measured/ and bites/; "; (d) exit line = catch's "c-u4r-runs: " + "<what> <end>" joined "; " + ": runs.txt written, with no verdict from that call|those calls (to the advisor)". OK. (e) non-split: unchanged("the judge"), "judge: <line>", throw text identical to HEAD's, written line identical. OK.
4. Condition 8: INF-F1-1's sentence in the header equals advisor-u4r-f1-head-full.md:33's words exactly, after the flock sentence. OK.
5. Layout: npt-lab create_dir_all(&s.out) at scenario.rs:771 makes <logs>/<group>/<run>; judge log_files (lib.rs:19-34) recurses, .jsonl only. OK.
6. Tested copy = committed: 791bc55's c-u4r-runs.mjs sha256 18c1dcf6... = sources.log's f2 copy = main folder now; 791bc55~1's c5cd4d9c... = sources.log's head copy. Shims, scratch compare.py (56be2e1d) and helper test (ecfc8c84) equal sources.log. harness.mjs now d0dd4ec3... vs sources.log 99f583a2...: edited 21:16:18 after the 21:01 run (the INF-F2-3 NOTES re-point, line 33's comment says so); the kept harness is not byte-for-byte the run's. Low housekeeping (no re-run since).
7. summary.log: 17 stagings, all PASS. Read check lines of cmp-P0 (20/20, incl. (i) layout, (d) order, argv except --out, two judge calls same --marks/--only, runs.txt diff only the 3 allowed lines), cmp-P3 (v), cmp-r2 (judge argv equal, r2.txt identical masked), fail-measured/-bites/-both (3(d) lines exact; which file kept), fail-P3 (HEAD = F2 exit line), lim-measured/-bites (stopped at 2.0 s; other call runs; group killed), recheck-bites-edit/-commit (stop names "the judge (bite runs)", call 1 ran, call 2 not, no results/), recheck-measured-edit control. Conditions 2, 3, 4 proven.
8. Condition 5: my own python3 -I key-by-key read of real/*.json: for -2, -17, -57, measured-alone parts/report/verdict/title = combined; bites-alone bites = combined; -17 tick16 and whole-as-lost shown in both; -57's three bites not_run in r2's sample (2 of P0's 6 bites exercised; the rest by judge code, infra checked figures.rs). Bites-alone parts' why all read "0 of n valid runs"; measured-alone bites read "no run yet": SPLIT_LINE's words hold. OK.
9. infra-engineer's review read: INF-F2-1 Medium (stop order during the measured call orphans the bites call) fixed in card item (6), no code; INF-F2-2 Low (three comment wordings), INF-F2-3 Low (/dev/shm leftovers; harness NOTES path). Severities agreed.
10. Condition 7, recomputed from the four judge-apart logs (.gate/u4r-judge-apart/*.log; judge 65acacfe, marks 9fb5be57 in each): my least-squares fit 212.3 + 2.5062 x (card 212.5 + 2.5059 x; residuals 1.1, -3.8, -7.2, 9.8 MiB). F = max(1.44, 1.504, 1.239, 1.5) = 1.504. Measured 2,190.7 MiB logs -> 5,702.7 MiB = 5.57 GiB, break-even F 2.75; bites 2,431.7 -> 6,306.6 MiB = 6.16 GiB, break-even F 2.47-2.48; one call 11.52 GiB. R = 9,218.2-9,218.6 MiB. Card's figures hold. P3, B1, P2 runs.txt heads all b3b4a60d / 33e1f49f / 65acacfe / 9fb5be57. Condition 7 met for the start; its head check is at P0's start.
11. New since launch: f998e74 (card only): (B) 21:17:37 and (C) ~21:18 done, df 8,138 -> 8,618 -> 16,492 MiB; R met. Done before this review although the card (208d7dc) said "if APPROVE and CI is green: (B), then (C)". Sets ruling step 6 ties (B)/(C) to condition 7's at-most-10-GiB result, not to this review; the cleans only lend space (shared-target caches refill cold). No harm; noted, not a finding. Before P0: no check.mjs or shared-target build (would take ~9 GiB back), df at P0's start.
