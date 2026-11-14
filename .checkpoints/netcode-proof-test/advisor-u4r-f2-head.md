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
