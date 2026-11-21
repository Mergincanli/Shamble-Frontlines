# Advisor: C62H-BOUND pushed head check (brief s6 item 3), head ddd38bace3c63050d2087a1acf3a6e74c1d2ecb9

Role: main's .claude/agents/advisor.md (read 2026-11-16). Read only; notes here only.

## Plan
1. Head and parents: ddd38ba on origin branch; parents chain efe8401 <- eb2bebf <- b06991e <- 15102d1 <- 561bf42 <- ddd38ba; ddd38ba touches .checkpoints/ only.
2. Each cherry-pick touches only spikes/netcode-proof/; merged bytes = c62h-bound-build (42b9646) bytes (git diff --quiet).
3. Merge-stop ruling (advisor-c62h-bound-merge-stop-full.json): conditions C1-C8, pass bar for next run.
4. merge.log (run 1 FAIL recorded by name), merge-2.log pass bar item by item.
5. Gate log, built-check log (25 PASS 0 FAIL).
6. CI: as relayed (not re-read); note.
7. Size check 967/1,050.
8. Review round handling (12 Lows deferred in writing), follow-ups wording (U3C-HELD-1 vs C3).
9. c62h_bound_set_check.py sha256 committed in ddd38ba.
10. Pre-flight written commands (C7): check against crates/lab/src/main.rs parser, scenario.rs settings, ops/runs/c-u4r-runs.mjs:329, :485, :533.
11. Verdict.

## Notes (a line after each point)
- 1 head: ddd38ba parent 561bf42 <- 15102d1 <- b06991e <- eb2bebf <- efe8401. ddd38ba touches only .checkpoints/ (card, merge-stop notes+json added, build.md, set_check.py). Branch tip now 04ebe83 (card-only checkpoint [skip ci] on ddd38ba; outside my head, card only).
- 2 merged bytes: each cherry-pick touches only spikes/netcode-proof/ (raw modes 100644; one A fixture). git diff --quiet c62h-bound-build(42b9646) 561bf42 -- spikes/netcode-proof ':(exclude)...results' exit 0; non-.checkpoints diff 561bf42 vs 42b9646 empty; patch-ids equal pairwise (036447a4, 654d080f, 77464093, f1d1de9f).
- 3 ruling read: C1-C8, pass bar, preflight field.
- 4 merge.log: FAIL at test, name at :51 (libtest failures list), bites 429/1, measured 552/0; exit 1; det-u4/build/hash not run. BUT failed-steps/test.log is GONE: check.mjs main() rmSync's <step>.log at each step's start (check.mjs:2655-2657, :2676-2677), so merge-2's test step deleted the 1577-line file (dir mtime 16:31). Panic text survives only as quoted in build.md:431-434 and the merge-stop ruling (both committed in ddd38ba). -> Low finding: the record/report must stop citing test.log:1407/:1476-1532 as on disk.
- 4b merge-2.log, pass bar item by item: header free 5278 MiB, HEAD 561bf42, diff+untracked e3b0c442 OK; check.mjs exit 0, "all 11 steps pass", test "552 passed, 0 failed ...; bites 632 passed, 0 failed" OK; det-u4 PASS 215.5 s, non-cargo = 200.8+2.5+0.9+0.0... = 204.3 < 240 OK; release build exit 0 OK; regenerated SS-P0-s4 line exact OK; 9000 ticks 0 mismatching equal OK; other cargo/rustc empty. PASS WHOLE.
- 5 gate-merge.log: GATE PASS --no-smoke, base 53272a1, symbols +0 -0. Built check: 25 PASS 0 FAIL; my own sha256 of ddd38ba's marks.json c1935a5b, SERIES.md 6898e119, README.md f3c3c804 = brief's expected.
- 9 set_check.py at ddd38ba sha256 e9eb9b23...ea8 = working copy = ruling's. Brief 663 lines, 7936074c... OK.
- note: 98a7c1d (saver) committed this notes file; branch tip moves with card saves only.
- 7 size: numstat efe8401..561bf42 spikes/netcode-proof = +904 -63 = 967 (= 044fab1..42b9646); <= 1,050 OK.
- 8 review: test 7 Lows (TE-CBH-1..7), netcode 3 (NA-CBB-1..3), rust 2 (RR-CB-1..2) = 12 Low, all APPROVE WITH FINDINGS; read_line present in test JSON. TE-CBH-1/-2 fixed in script; ten deferred, listed on card :3675-3678 OK.
- 8b follow-ups: U3C-HELD-1 card :3663-3672 has C3's quoted title, Medium, taker main session, before another lane's counted check.mjs run (U4f's merge), every-write bound, planted bug (drop released_model), NC-U3c-2 sentence. Omits C3's reason clause ("so that run does not meet the failure unnamed"), adds "at the latest" - not looser. Cites failed-steps/test.log (gone) -> same Low. CHECK-NFF-1 = C4 OK. C6 card :93-94 OK.
- 10 pre-flight, lab flags: scenario::parse (scenario.rs:1447-1562) takes run,set,date,commit,out,scene,profile,seed,role,marks,suite,clients,tick-start,warmup-s,measured-s,after-max-s: all 16 in the written step 4 are accepted, none twice; unknown flags refused (:1560). --commit must be 40 lowercase hex (machine.rs:77-86): "<the pushed head, full>" OK if full. Runner form (c-u4r-runs.mjs:485 common, :533 settings, :535 call; planJudged :329 extra) = same flags; runner omits --clients (default scene.slots()), pre-flight passes 48 = -17 part select clients 48 OK.
- 10b CONCERN: -17's L1 P2 part has validity pre:PM-0001-2 (marks.json); eval.rs:958-996 runs PM-0001-2's SS-runs checks on the run; judge_run (eval.rs:1186-1188) returns with NO checks when a validity rule fails. P2_SHORT's 6 s warm-up / 4 s measured vs the sets' 10 s / 600 s (c-u4r-runs.mjs:118-119): if any SS load check (300 projectiles min, 700 shots/s mean, portal 0.95-1.05, 48 soldiers, teams of 16, everyone-in-view 0.99) fails on the short run, the figures are never read -> "not informative" and the pre-flight is wasted. Fix cheap: --warmup-s 10 (the sets' warm-up), keep it short measured; judge still INVALID/NOT-RUN by min 3 (fine: ruling covers it).
- 10c FINDING (Medium, my own ruling's wording copied faithfully): judge main.rs:85-93 `marks.marks.retain(only)`; with --only PM-0001-17 the Pre rule's lookup (eval.rs:960-962, ctx.marks = the retained file, eval.rs:1724-1737) pushes "pre:PM-0001-2: not in marks.json" -> run INVALID -> no check evaluated (eval.rs:1186-1188). Pre-flight non-informative BY CONSTRUCTION. Runner passes --only listedOf(P2) = PM-0001-2,PM-0001-17,PM-0001-57 (c-u4r-runs.mjs:305, :569). Fix: --only PM-0001-2,PM-0001-17.
- 10d Low: no rebuild/expected hash for T/npt-lab (merge-2 printed none; shared target shared by worktrees). main-run.sh touches crates/ (main-run.sh header) so `cargo build --locked --release -p npt-lab` in the call rebuilds from the main folder's bytes (~10 s). npt-judge sha256 must be compared with p0-rejudge.log's line explicitly. --commit: the full `git rev-parse HEAD` at the call (runner's form :415), spike tree outside results/ = ddd38ba's; --date the day it runs.
- 10e other checks: judge reads logs recursively (lib.rs:19-34); lab create_dir_all(out) (scenario.rs:879); seconds counted from the run's first tick (logs.rs:716) so tick_start 63736 leaves measured seconds whole; SS portal change 1/s and 700 shots/s by design (scene.rs:157-166).
- 11 own-ruling correction (Low): merge-stop said "npt-lab does not depend on npt-net at all"; it links it via npt-server (server/Cargo.toml:29) and npt-bots (bots/Cargo.toml:25) - merge-2's release build compiles npt-net. grep: no DelayStream / l2::delay in lab, server or bots src. Conclusion unchanged.
- 12 builder returns: step1-4 status done; counts 545/625, 551/631, 552/632, 552/632 = plan.
- pre-flight section is in ddd38ba's build.md (committed), unchanged since.
- VERDICT: APPROVE WITH CONDITIONS (head passes its merge step whole; re-judge may run; pre-flight commands corrected before it runs).
