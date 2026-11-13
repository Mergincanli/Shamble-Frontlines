# The advisor's ruling on U4d2's M9 cold run, which stopped at its fourth guard: the brief (the main session, 2026-11-13 about 09:40 UTC)

**Your role:** read `cd /home/user/Shamble-Frontlines && git show origin/main:.claude/agents/advisor.md` first and work
as it says. Rule on one stop under your round-2 ruling's M9 (advisor-u4d2-r2.md, Block 5: "If any command before the
step fails, the run stops there and it comes to me"), and on INF2-4's runner follow-up, which the two possible routes
depend on. Write your ruling to `.checkpoints/netcode-proof-test/advisor-u4d2-m9.md`, one block at a time. Write files
with the Write and Edit tools only (no sed, no shell redirect, no python writes; never `cargo fmt` writing files: apply
rustfmt's diff by Edit); an agent with no Edit tool rewrites its notes file whole with Write. Run nothing that builds,
cleans or touches a copy; reading, git, sha256 and `cargo clean ... --dry-run` (it deletes nothing) are fine.

**Where things stand (all paths under /home/user/Shamble-Frontlines):**
- U4d2 is merged at cc8595d (M1 to M8 met: .checkpoints/netcode-proof-test/u4d2-build.md, "Merge into the spike
  branch"; u4d2-report.md); CI green on it (gate runs 43401792311 push and 43401799970 pull_request); the saver
  restarted; `.gate/u4d2-rr-target` removed by `cargo clean --target-dir` (85 files, 38.0 MiB:
  .gate/u4d2-merge/rr-target-clean.log). No lane and no agent is running.
- **The cold run** (.gate/u4d2-merge/cold-run.log), one main-run.sh call with your M9 commands in your order: `cat
  /proc/uptime` (4102.92 s) exit 0; `cargo clean --target-dir <shared> --release` exit 0, "Removed 1134 files, 293.6MiB
  total"; `cargo clean --target-dir <shared> --release --target wasm32-unknown-unknown` exit 0, "Removed 0 files";
  `test -f <shared>/.lane.lock` exit 0; `test -f <shared>/CACHEDIR.TAG` exit 0; `test ! -e <shared>/release/deps` exit 0;
  `test ! -e <shared>/wasm32-unknown-unknown/release/deps` **exit 1**. main-run.sh stopped there: `node check.mjs
  --only wasm` did not run. (`<shared>` is /home/user/Shamble-Frontlines/.gate/shared-target; cargo 1.99.1.)
- **Read after the stop (read only):** `<shared>/wasm32-unknown-unknown/` holds CACHEDIR.TAG, debug, doc and release;
  release is 40M with 44 files in deps. `cargo clean ... --dry-run` from spikes/netcode-proof: `--target-dir <shared>
  --release --target wasm32-unknown-unknown`: "Summary 0 files"; the same with CARGO_TARGET_DIR=<shared> instead of
  `--target-dir`: 0 files; `--target-dir <shared> --profile release --target wasm32-unknown-unknown`: 0 files;
  `--target-dir <shared> --target wasm32-unknown-unknown` (no profile): "Summary 34576 files, 7.9GiB total", the whole
  shared target, a full clean, which your Block 5 forbids until INF2-4's runner follow-up is in.
- So now: the host's release artifacts are gone (the next release build rebuilds them from nothing); the wasm32 release
  artifacts are as they were (warm); the lock file and CACHEDIR.TAG are in place.

**What I ask you to rule on:**
1. **The cold figure's route** on cargo 1.99.1, where no `cargo clean` form tried removes the wasm32 release folder
   alone. For example: a partial cold figure now (host release cold, wasm32 release warm, so labelled); another method
   for the wasm32 release folder (which, and is it approved under §7.1 and W1-TARGET item 3?); a full clean after
   INF2-4's follow-up; or more dry runs first (name them). Say what goes on the card and in U4's report, and whether
   U4r's brief waits for it.
2. **INF2-4's runner follow-up** (your round-2 ruling, Block 2: the runners fail closed when they cannot take the lock,
   and nothing makes the shared target's folder by hand): its form in main-run.sh and u4d2-run.sh (helper scripts in
   the notes folder, not kit or gate files), its proof, and whether it needs a plan step with you first; and how a full
   clean would then work, given that the lock's file lives inside the folder a full clean deletes.

For each item give your ruling with the words wherever a text is to be written, and say what goes to Mustafa (if
anything), with the reason.
