# Advisor: review of U4r-F1's pushed head (f1f131b), PR #40, 2026-11-14

## Plan
1. Branch state: git log on the branch after f1f131b (later commits touch only .checkpoints/).
2. git show f1f131b --stat; spikes/ change = exactly the two scripts; sha256 of each at f1f131b vs sources.log.
3. Read the diff in full; check each condition 1..8 in code.
4. Read logs: summary.log, sources.log, staging logs, edited-copies; check each condition in logs.
5. Harness: read it, check it does what logs claim.
6. infra-engineer review: INF-F1-1..5, and their handling (card's NOW block, run procedure).
7. INF-F1-1 optional header sentence: decide.
8. CI on f1f131b (gh or GitHub MCP).
9. Verdict + next steps + Mustafa.

## Checks (a line per check as I go)
- C-branch: branch head 30749a5 = origin; only commit after f1f131b is 30749a5, touching only .checkpoints/netcode-proof-test.md. PASS.
- C-files: f1f131b changes 3 files: the card + the two scripts (mode 100644, no symlink). sha256 at f1f131b: runs c5cd4d9c…, r2 94564f8c… = sources.log's f1/ = working tree. head/ copies = f1f131b^ (8ececd9d…, c861da5d…). check.mjs a169a286…, SERIES 85ef6453…, marks 9fb5be57…, loader a0507d1e…, precheck-recordings 430b295e…, redact-check 941c6925… all = f1f131b. Spike tree 0b47e881 → a88ee99f. PASS.
- C-A (INF-R2-1): spikeChanged/spikeReading exported in runs (:130-146), imported by r2 (:54). runs reading() via sh default cwd ROOT; r2 passes ROOT explicitly. unchanged() uses fresh reading; start.status gone (cond 7); start refusal keeps tracked() (runs :296-297, r2 :164-165). runs.txt/r2.txt name tree. PASS in code.
- C-B (INF-R2-3): runs: binMarks copy + sha check + planOf(binMarks) + floor(runs[0]) before mkdirSync(logs); copy into logs. r2: floor(suite SUITES[0].key) before mkdirSync(out). PASS in code.
- C-C (INF-R2-4): timed() via run(exe,args,ROOT,env,RUN_LIMIT_MS); non-stop non-zero throws sh()'s exact message form (out = stdout+stderr). Stopped run line, pair line, recording throw (Q4 text exact), judge: three forms, verdict copied only on ok, runs.txt then exit 1 with cond 1's text. Signal not named ("by a signal": run() returns no signal; same form as r2's step()). PASS in code; note Low.
- C-D (RUN-L1 + Q6): -v in both builds; filter /^\s*(Compiling|Fresh) npt-/ on stderr; stdout trimmed to builtArtifact; throw form = sh()'s. PASS in code.
- C-E: whole-as-lost marks "PM-0001-2,PM-0001-17", expect word for word = advisor-u4r-c62h-full.md:45. PASS.
- Cond 1 logs: lim-run (shim judge exit 1 on cut l1-p0-s2/client.jsonl; runs.txt with "judge: exit 1 (no verdict): …", run 2's stopped line, no verdict.json, exit 1 "the judge exited 1: runs.txt written with no verdict (to the advisor)"); realjudge (r2's npt-judge 65acacfe… = sha256sum here; uncut exit 0; cut at line 451 exit 1 naming file+line; r2 folder untouched). PASS.
- Cond 2 logs: real git (PATH = cargo-only shims + system git; GIT_CONFIG_GLOBAL=/dev/null): a: HEAD's 3 stop, F1's 3 complete naming commit+tree; b: F1's 3 stop quoting the marks.json status line (r2 too: cwd ROOT proven); c: F1's 3 stop naming both trees. 18/18. PASS.
- Cond 3 logs: floor-{P0,precheck,r2}: F1 no set folder, HEAD's leave it; ii-{P0,precheck}: F1 no folder + re-run completes, HEAD leaves folder + re-run refused. Edited copies diff = one line each (checked by diff against f1f131b / f1f131b^). PASS.
- Cond 4 logs: lim-run, lim-pair, lim-rec (Q4 text exact, no pair, no NaN, no runs.txt), lim-judge (Q3 line, exit line); sleeper dead (state Z, pgrep -f empty); lim-control shows the sleeper survives without run()'s group kill. First lim-* runs FAILed on the harness's kill(pid,0) liveness (zombie); fix to /proc state is sound, not a loosening of condition 4 (pgrep -f empty in both). PASS.
- Cond 5: -v in code; cmp-* show cargo argv = HEAD's + -v, cwd spike; Compiling/Fresh npt-* printed, libm not. Real-build check due at the pre-check set. PASS (shim side).
- Cond 6: card's THE SETS' RUN PROCEDURE holds (a) and (b) as INF-F1-1 corrects it. Plan's risks section NOT updated (774b952d… unchanged): Low, goes to u4r-report.md's F1 entry. Procedure lacks `mkdir -p .gate/u4r-sets` and step 17's getrusage peak-memory wrapper: Low, card text.
- Cond 7: start.status gone; start refusals keep tracked(). PASS.
- Cond 8: helper test 9/9 incl. JDG-ONLY-1 test (file sha 47200f26… = helper.log). Real-tree redact-check: no kept log; holds by composition: redact-check is a per-file line scan; F1's two files 0 findings (redact.log); spikes/ differs from 5e563b9 only by the two scripts, where check.mjs's redact gave 277 items 0 findings (.gate/u4r-merge/c62h-check.log:17). PASS with a Low (keep the log).
- CI on f1f131b (gh api check-runs): gate success 17:30:19, smoke success 17:30:46, toolchain success 17:30:37. f1f131b touches no .github/ or gate file, so the green stands for this change. I did not run the gate myself (task limits: no script runs).
- Infra round: INF-F1-1 (Medium) fixed in card procedure items (5)-(7), no code change: right; my plan review's condition 6(b) premise was wrong (lock fd passed down) — owned. INF-F1-2 fixed in card (2)-(4). INF-F1-3: report line + step 17 memory; home for naming the signal: check.mjs run() returning `signal` at its next change. INF-F1-4: card corrected (42 stagings, 47 runs; summary.log 47 lines, 42 names, 42 logs). INF-F1-5: report line; raw (untrimmed) status read at scripts' next change.
- Header sentence (INF-F1-1 item 4): at the scripts' next change, not now (header is not wrong; card is the binding procedure; a comment-only push is a Low-only push).
- Disk now 4,470 MiB free (df -m); no npt-lab/npt-judge process (pgrep). /dev/shm/u4r-f1 75 MB tmpfs left.
- Verdict: APPROVE. Own Lows: ADV-F1H-1 (card procedure: mkdir -p .gate/u4r-sets; step 17's getrusage wrapper), ADV-F1H-2 (no kept log of the main session's real-tree redact-check and gate --no-smoke), ADV-F1H-3 (condition 6's plan-risks half: into u4r-report.md's F1 entry). No push for any of them.
- Next (round-2 order): report + card first (no code push); D4 (14, keep the 4 GiB floor before its build); disk plan (15); pre-check set (16) by the card's procedure, condition 5 read from its log; judged sets P3, B1, P2, P0 (17). Mustafa: nothing now; heads-up on P0's disk.
- DONE: StructuredOutput returned.
