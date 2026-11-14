# U4r review, round 1, infra-engineer seat (batch 2)

Snapshot under review: wip/u4r.patch sha256 572c4f69435da2ad6f62d54d4bcbc47b04d6329b5493e0faf78fcecfdc4a09c2 (checked by sha256sum 2026-11-14).
Packet: u4r-packet.md sha256 cbd82485... (checked); tests file u4r-packet-tests.md sha256 d12555c8... (checked).
Read copy: /home/user/Shamble-Frontlines/.gate/u4r-snap1 (2fbec77 + patch). Cite file:line there.
Logs of my own runs: /home/user/Shamble-Frontlines/.gate/u4r-review-infra/ (task text; the panel file names .gate/u4r/rev-infra-<n>.log: the task text is the newer, more specific word, so I follow it).
Verdict vocabulary: the schema's (APPROVE / APPROVE WITH FINDINGS / REQUEST CHANGES).
No Edit tool in this agent: this file is rewritten whole with Write each time.

## Plan
1. Sources: panel section 6 + "What every seat is told" (read); carry 84 (u4s-carries.md :402-416); ruling 1 point 1 (advisor-u4r-build1-full.md); brief :1898-1899 and section 4 rules (no-retry, floors); LEFTOVER-DELETE wording.
2. Item (1): check.mjs wasmStep: shared deadline WASM_CALL_LIMIT_MS 300 s from first non-cargo call; skip-and-name after a stop; process-tree kill by documented means; the two self-tests (fake runner+clock; real processes with grandchild ~200 ms).
3. Item (2): ops/runs/det-u4.mjs + wasmStep's two options: imports wasmStep with det-recordings.json, bites half off, own .gate/ folder, prints like check.mjs main, exit 0 only on PASS, no retry; check.test.mjs case for the options.
4. Item (3): times vs budgets: c5-end1.log:174, c5-end2.log:174 (39.6 s / 41.9 s), warm/cold labels with uptime; det-u4.log:24 200.5 s; non-cargo <= 240 s inside 300 s; report section 5 item 4 and return item 9 vs ruling 1 point 1 (7).
5. Item (4): ops/runs/c-u4r-runs.mjs and c-u4r-r2.mjs: lane lock and -x, refuse uncommitted state, .gate/ folder by commit 12 hex, df and floors before release builds, no clean/delete. Report disk figures only.
6. Runs only if a finding needs one (through u4r-run.sh -x, df first).

## Progress (done)

- Read: panel s6 + common; carry 84; ruling 1 point 1; brief 2.3, s4, 12 items 7-8; check.mjs diff and wasmStep/run/main
  (snap1 check.mjs:1884-1907, 2267-2580, 2636-2666); check.test.mjs diff; det-u4.mjs; both manifests; c-u4r-runs.mjs;
  c-u4r-r2.mjs; u4r-run.sh; main-run.sh; npt-judge main.rs (exit 0 whatever the verdicts); det-u4.log; c5-end1.log;
  c5-end2.log head and step lines; builder notes :733-756, :950-1100, :1195-1260, :1489-1560; card :718-731 (M9 cold).
- No report on disk (no u4r-report*; u4r-build.md:1539-1541 "Next: the report"): item (3)'s report words counted from
  notes and logs only.
- The runner refuses any copy outside .claude/worktrees/ and any log outside .gate/u4r/ (u4r-run.sh:36-43): no run
  through it in the read copy is possible (rust and security seats say the same). So det-u4 was not re-run by me.
- Probe 1 (.gate/u4r-review-infra/probe-1.log): run() imported read-only, the test's own child string, limits 200 to
  50 ms, 10 each: timedOut 50/50, both pids printed 50/50, left running 0/50. Carry 84 (b) holds on this machine; the
  test's 200 ms limit is about 4x what Node needs here.
- Item (1) met: shared deadline (check.mjs:2331-2349), skip and name (:2337-2339), stopped call named with time left
  (:2345-2347), group kill with documented limits (:1884-1907), both self-tests (check.test.mjs diff), plants
  (u4r-build.md:990-999). Item (2) met (det-u4.mjs:21-36; options test). Item (3): 39.6 s warm, 41.9 s machine-cold
  deps-warm, both far under 200 s; det-u4 200.5 s, non-cargo 190.9 s under 240 s (warm, 01:11 tree).

## Findings

- INF-U4R-1 (Medium): det-u4's one run (det-u4.log:1-26, 01:08-01:11 UTC) predates ruling 3's mirror.rs keep() and
  scenario.rs edits (u4r-build.md:1215-1222, written 01:44-01:49). mirror.view feeds the bots (scenario.rs:963-968);
  keep() now holds 35 frames when the pin is inside the ring (mirror.rs:86-96), so a client 34 ticks behind now gets a
  view where it got none: a path to the runner recordings' bytes (det-recordings.json:4-5 SHA pins). Likely unchanged
  (P2/P3 delay 150 ms, l1/src/lib.rs:191), but no run on this head; check.mjs never regenerates these entries.
- INF-U4R-2 (Medium): c-u4r-runs.mjs:126-131 (bin dir, three release builds, set folder) and c-u4r-r2.mjs:110-116
  come before any floor() (first at :146/:162/:180 and :125); rule 6 floors every release build; a floor stop at the
  first run leaves the set folder, which then refuses any re-run at that commit (:123, :109), with no delete allowed.
- INF-U4R-3 (Medium): floor() (c-u4r-runs.mjs:93-97) checks free >= 4 GiB now, not free minus the next run's logs, so
  one call cannot keep ruling 1 point 3's main-session stop rule (3); P0 measured and -57 bite runs ~509 MB each
  (u4r-build.md:739); at today's 4.68 GiB the P0 set ends near 3.5 GiB with two of eight runs, folder blocking.
- INF-U4R-4 (Medium, out of seat and patch): u4r-run.sh:63,68 and main-run.sh:38,43 read `df -BG`, which rounds up:
  5,025,054,720 B (4.68 GiB) reads "5G" (df at 07:59 UTC); c5-end2.log:2 "free 5 GB" vs notes :1519 "df 4.7G";
  det-u4.log:2 "free 5 GB" vs notes :1008 "4.9G". The 4 GB floor passes at 3.01 GiB.
- INF-U4R-5 (Low): det-u4.mjs:30 runs wasmStep alone; check.mjs runs deps first so the builds' flags are checked
  (check.mjs:2593-2594). Run bare (its header :5), a RUSTFLAGS/profile/linker in the env goes unchecked.
- INF-U4R-6 (Low): nothing prints the non-cargo sum against 240 s or the deadline used (check.mjs:2576-2578): the
  advisor's 240 s trigger rests on a hand sum of 11 rounded entries at each later merge (190.9 s today).
- INF-U4R-7 (Low): cold label: 41.9 s (c5-end2) is machine-cold, deps-warm; M9's cold (every release artifact built from
  nothing: card :729) not measured for U4r's tree; uptime only in notes, not logs. Bound: U4d2's M9 cold 61.6 s.
- INF-U4R-8 (Low): c-u4r-runs.mjs states no lock or target rule; main-run.sh:17 would put it on the shared target
  (brief 12 item 8: advisor first); runs.txt records neither CARGO_TARGET_DIR nor load; wall seconds feed r5/7.3.
- INF-U4R-9 (Low): judge reads the live marks.json at the set's end (c-u4r-runs.mjs:196; :176 at start); c-u4r-r2.mjs
  :130-131 per suite; untracked files (e.g. a build.rs cargo finds by itself) pass the git check (:115, :104).
- INF-U4R-10 (Low): run() hardening: spawnSync keeps killSignal SIGTERM and waits for the child (check.mjs:1895), the
  group is killed only on ETIMEDOUT (:1897), the deadline reads Date.now (wall clock, :2321 default).
- INF-U4R-11 (Low): c-u4r-r2.mjs:93-99 spawnSync has no timeout while it holds the exclusive lane lock (its header
  :22-23); the runners' flock waits without -w (u4r-run.sh:53).

## Disk figures (reported, not settled)
- Free 5,025,054,720 B (df -B1, 07:59 UTC; df -h 4.7G). Main folder's cache .gate/netcode-proof-test/target 6.8G;
  .gate/netcode-proof-test/runs 2.0G; shared-target 8.2G; .gate/u4r 867M; the builder's det-u4 folder 37M, wasm-step 43M.
- Projected sets (rates u4r-build.md:739-748): P0 about 3.2 GB (3 x 509 measured, 3 x 509 -57 bites, 2-3 x 56 -17
  bites), P2 0.66, P3 0.38, B1 0.57: about 4.8 GB, against 0.68 GiB above the floor today.

## Cost per run
- check.mjs 11.9 and 12.6 min under the exclusive lock (c5-end1 02:23:36-02:35:28; c5-end2 06:44:05-06:56:43), wasm
  step 39.6 / 41.9 s of it; det-u4 adds 200.5 s plus its two pre-builds (about 3.6 min) at every merge on the plan's
  crate list. No CI and no money: this container only; offline under the runners.

## Verdict
APPROVE WITH FINDINGS: carry 84 (a)-(c) and ruling 1 point 1's items (4)-(6) are met; four Mediums for the fix pass.
