# The advisor's ruling on U4d2's M9 cold run, stopped at its fourth guard (2026-11-13, from advisor-u4d2-m9-brief.md)

Seat: the advisor agent, run by the main session's workflow, working from main's `.claude/agents/advisor.md` (read at
origin/main). It rules; it edits no code and runs nothing that builds, cleans or touches a copy (reading, git, sha256
and `cargo clean ... --dry-run` only).

## Plan
1. Read the brief whole (done), main's advisor.md (done), my round-2 ruling's Block 2 (INF2-4) and Block 5 (M9) and its
   "Not checked" list (done), W1-TARGET (advisor-u4-wave1.md:558-586, done), u4-build-plan.md §7.1 (:909-921, done),
   LEFTOVER-DELETE and the shared-target rule (history.md:404-424, done), main's deny list (settings.json, done).
2. Read the cold run's log and main-run.sh whole (done); read u4d2-run.sh; grep every runner and text that names the
   lane lock's path; read check.mjs's wasm step for the target folder it builds into (done: no `--target-dir`, so
   CARGO_TARGET_DIR from main-run.sh; WASM_BUILDS :2120-2140, two host and two wasm32 release builds).
3. Find why `--release --target wasm32-unknown-unknown` removed nothing (the shared target's tree as it is now; dry
   runs only), and test by dry run a form that removes the wasm32 release folder alone: `cargo clean --target-dir
   <shared>/wasm32-unknown-unknown --release` (that folder holds its own cargo-made CACHEDIR.TAG), listing every path
   it would remove, and checking afterwards that the dry runs changed nothing.
4. Rule item 1 (the cold figure's route; the card's and U4's report's words; whether U4r's brief waits), then item 2
   (INF2-4's runner follow-up: its form, its proof, whether it needs a plan step with me; how a full clean then works),
   one block each, written as each is finished.
5. The exact commands of the run I approve, Decisions needed, For Mustafa, what I checked and did not.

## Notes (one line per check, as done)
- Cold run (.gate/u4d2-merge/cold-run.log, read whole): lock held 09:31:51, free 7 GB, HEAD 1290327; uptime 4102.92;
  host release clean "Removed 1134 files, 293.6MiB"; the wasm32 form "Removed 0 files"; guards 1-3 exit 0, guard 4
  (`test ! -e <shared>/wasm32-unknown-unknown/release/deps`) exit 1; check.mjs did not run; exit 1. Its provenance line
  counts its own header once (`1 --- provenance (Compiling/Checking/Documenting npt-* lines, counted):`): the header
  itself matches the grep (a Low, below).
- The shared target now (09:36 UTC, read only): root holds .lane.lock (09:31), .rustc_info.json, CACHEDIR.TAG (Nov 9),
  debug, doc, tmp, wasm32-unknown-unknown; no `release` (the host clean held). `<shared>/wasm32-unknown-unknown` holds
  its own CACHEDIR.TAG ("created by cargo", Nov 9 02:43), debug, doc, release; release 41,431,282 bytes, 131 files
  (158 entries), with cargo's three lock files (.cargo-lock, .cargo-build-lock, .cargo-artifact-lock). Nothing under
  the root to depth 3 is newer than 09:32. No cargo, rustc, npt-lab, main-run or u4d2-run process.
- Cargo config: spikes/netcode-proof/.cargo/config.toml sets only `[build] target-dir`; no build-dir; no user config;
  the only CARGO_* in the environment is CARGO_HTTP_CAINFO. check.mjs passes no `--target-dir` (grep), so its builds
  use main-run.sh's CARGO_TARGET_DIR.
