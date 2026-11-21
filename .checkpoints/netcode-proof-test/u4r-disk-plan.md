# U4r's judged sets: the disk plan (the round-2 ruling's step 15; the advisor's U4r build ruling 1, point 3)

Written by the main session on 2026-11-14 at about 17:40 UTC, before any c-u4r-runs.mjs set and before D4. Sources:
advisor-u4r-r2-full.md step 15, advisor-u4r-build1-full.md:28-29 (ruling 1 point 3, its five parts), advisor-u4d2-m9.md
Block 2 (W1-TARGET item 3 as amended), and the runner's own projection, computed below.

## 1. Free space now

- `df -B1 .` at 17:33 UTC: **4,687,171,584 B available = 4,470 MiB.** The ruling had 4,681 MiB. Since then the r2
  record run kept 114 MiB (.gate/netcode-proof-test/r2-e59ece9ecafc, `du -sm`; the ruling projected about 162), plus
  r2's bin copy (7 MiB), carry 62 (h)'s two folders (about 1 MiB each) and U4r-F1's logs.
- The build caches: `.gate/shared-target` 8.2G (debug 7.7G, release 282M, doc 24M, wasm32 176M);
  `.gate/netcode-proof-test/target`, the main folder's own cache, 6.8G (debug 6,066 MiB by `du -sb`, release 679M, doc
  11M, wasm32 78M).

## 2. What is still to come

The judged sets' and the pre-check's figures come from the runner's own functions, not from a hand count. I imported
`planJudged` and `planPrecheck` from spikes/netcode-proof/ops/runs/c-u4r-runs.mjs (as committed at f1f131b). The rates
are its BYTES_10MIN_57, BYTES_10MIN_OTHER and PAIR_BYTES, plus the manifest entry's bytes. The marks are today's
marks.json (5e563b9), where -17's bites now include whole-as-lost (carry 62 (h)), so P0 has one bite run more than the
ruling counted (+53.4 MiB, about 230 s).

| Step | Runs | Projected logs (MiB) | Free space the runner wants at the start (total + 4 GiB, MiB) |
|---|---|---|---|
| D4: r5's 15 runs, 60 s each, in the main folder's cache | 15 | 320 (r5's own folder, .gate/u4r/r5, `du -sm`) | none (no runner check); 4 GiB before its release build |
| Pre-check | the recording SS-P0-s4 + 3 pairs | 16.2 | 4,112.2 |
| P3 | 3 measured | 359.9 | 4,455.9 |
| B1 | 3 measured | 545.3 | 4,641.3 |
| P2 | 3 measured | 628.9 | 4,724.9 |
| P0 | 3 measured at 485.5 each; bites tick16, client-rounding and whole-as-lost at 53.4 each; pos-4cm, yaw-12bit and pitch-8bit at 485.5 each | 3,073.3 | 7,169.3 |
| **Logs in all** | | **4,943.6** | |

Two more items, as estimates:
- **The main folder's release builds:** npt-lab, npt-lab with bites, and npt-judge. That cache's release folder was last
  written on 2026-11-11 (npt-lab then 769,728 B, before the sim and room crates), so the first build recompiles most
  of the workspace. Budget: up to 400 MiB. The shared target's whole release folder is 282 MiB.
- **Each set's bin copies** (.gate/netcode-proof-test/bin/<set>-<commit12>/): r2's is 7 MiB. Budget: up to 100 MiB for
  the five sets.

## 3. The floors

- 4 GiB (4,096 MiB) free before every release build, and before the pre-check's recording (section 4 rule 6; main-run.sh
  and the runner both use exactly 4 GiB since FP-0).
- The runner's own checks: free space minus the set's total must be at least 4 GiB before the builds and again after
  them; before each run, free space minus that run's bytes must be at least 4 GiB. Otherwise it stops before that run.

## 4. Without a clean, the sets cannot run

4,470 MiB now. After D4 (up to 400 MiB of build and 320 MiB of logs), at most 3,750 MiB is left. That is below P3's
4,456, the smallest judged set's start check. P0 alone wants 7,169 MiB at its start. So the clean comes first, before D4.

## 5. The clean: one approved part-clean of the main folder's cache, its dev profile

- **What:** `.gate/netcode-proof-test/target/debug`, 6,066 MiB. Its newest file was written on 2026-11-12 at 16:12 UTC
  (npt-judge). Nothing still to come uses it. Since W1-TARGET (2026-11-10), main-run.sh and every builder copy build in
  .gate/shared-target. c-u4r-runs.mjs and D4 build release only in the main folder's cache, where the runner refuses
  CARGO_TARGET_DIR. No script names this folder.
- **The method:** W1-TARGET item 3 as amended (advisor-u4d2-m9.md Block 2), applied to this cache's tagged root: `cargo
  clean --target-dir <root> --profile dev`, with its `test` guards. One call, from spikes/netcode-proof, holding the
  lane lock exclusively, offline:
  ```text
  cargo clean --target-dir /home/user/Shamble-Frontlines/.gate/netcode-proof-test/target --profile dev ::
  test -f /home/user/Shamble-Frontlines/.gate/netcode-proof-test/target/CACHEDIR.TAG ::
  test ! -e /home/user/Shamble-Frontlines/.gate/netcode-proof-test/target/debug ::
  test -d /home/user/Shamble-Frontlines/.gate/netcode-proof-test/target/release ::
  ls -A /home/user/Shamble-Frontlines/.gate/netcode-proof-test/target
  ```
  Never the folder-level form, never `--target` without `-p`, and nothing deleted by hand.
- **When:** after the advisor's review of f1f131b has returned, with no lane running: `pgrep -a cargo; pgrep -a rustc;
  pgrep -a npt-lab; pgrep -af npt-judge; pgrep -af loader.mjs` all empty, and no agent running.
- **What stays, and why:**
  - **The shared target, all 8.2G.** It is check.mjs's warm cache for JDG-ONLY-1's fix and the later lanes. A clean
    there would be refilled (7.7 GiB of debug) by the next check.mjs run.
  - **The main cache's release folder.** D4 and the sets build there.
  - **The main cache's doc and wasm32 folders.** 89 MiB in all: not worth a clean.
- **Its record:** the call's log in .gate/u4r-sets/clean-main-dev.log, with `df -B1` before and after, on the card.

## 6. With the clean

| After | Free space, projected (MiB) | The next start check |
|---|---|---|
| The clean | 10,536 | |
| D4 (build up to 400, logs 320) | 9,816 | |
| The bin copies (up to 100 across the sets) | 9,716 | |
| Pre-check | 9,700 | wanted 4,112: room |
| P3 | 9,340 | wanted 4,456: room |
| B1 | 8,795 | wanted 4,641: room |
| P2 | 8,166 | wanted 4,725: room |
| P0 | 5,093 | wanted 7,169 at its start, 8,166 projected: room, about 1,000 MiB spare |

At the end, about 5,090 MiB stays free: about 1,000 MiB over the floor.

## 7. Rules while the sets run

1. One set at a time, by the card's run procedure. `df -B1` goes in the set's kept log before each set. The runner
   prints its own projection and stops by itself before any run that would break the floor.
2. **No other lane during the sets.** U4f has not started: its brief goes to the advisor before its build. Any lane
   that does run during the sets (a copy and its builds) has its space added to this plan first, with `df` before it.
3. No log is deleted, moved off or replaced by a compressed copy. The /dev/shm scratch folders are memory, not disk,
   and they stay (LEFTOVER-DELETE).
4. If a set's start check says no room anyway: stop. The smaller sets may go first, and the P0 set goes to Mustafa under
   Decisions needed (ruling 1 point 3 (5): his LEFTOVER-DELETE route, a narrow kit rule), never a one-off delete.
5. P0's judge memory projection (step 17: over 10 GiB goes to the advisor first) is about memory, not disk. It is
   measured apart, into a scratch path that writes only a verdict file.
