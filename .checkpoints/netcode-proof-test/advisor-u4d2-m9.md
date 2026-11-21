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
- Dry runs (mine, 09:37-09:41 UTC, from spikes/netcode-proof, CARGO_NET_OFFLINE=true and CARGO_TARGET_DIR=<shared>
  exported as main-run.sh does, cargo 1.99.1):
  - `cargo clean --target-dir <shared>/wasm32-unknown-unknown --release --dry-run`: "Summary 131 files, 40.2MiB
    total", exit 0. With `-v`, its 158 paths `diff` equal to `find <shared>/wasm32-unknown-unknown/release` (every
    path inside that folder, the folder included); `find -type f` there: 131 files, 42,156,887 bytes = 40.2 MiB.
  - `cargo clean --target-dir <shared>/wasm32-unknown-unknown/release --dry-run`: refused, exit 101, "missing or
    invalid `CACHEDIR.TAG` file ... cleaning has been aborted to prevent accidental deletion of unrelated files" (the
    tag check holds even in a dry run; only the two roots carry a tag).
  - `cargo clean --target-dir <shared> --release --target wasm32-unknown-unknown --dry-run -v`: "Summary 0 files" now
    that `<shared>/release` is gone. With the log's first clean removing 1134 files and this form 0, it named the same
    folder: `--target` changes nothing without `-p`.
  - The six part-cleans, each root with `--profile dev`, `--doc`, `--release`: `<shared>` 32708, 723, 0 files;
    `<shared>/wasm32-unknown-unknown` 514, 494, 131 files; sum 34570. The folder-level `cargo clean --target-dir
    <shared> --dry-run`: 34577 files, 7.9 GiB. `find -type f` per part gives the same six counts (debug 32708, doc
    723, wasm32 debug 514, doc 494, release 131, tmp 0); the 7 left are the roots' own files: `<shared>`'s
    CACHEDIR.TAG, .rustc_info.json, .rustdoc_fingerprint.json, .lane.lock; the triple root's CACHEDIR.TAG,
    .rustdoc_fingerprint.json and .rustc_info.json.
  - **A dry run wrote one file:** `<shared>/wasm32-unknown-unknown/.rustc_info.json` (356 bytes, 09:37:38, my first
    dry run: cargo caches rustc's version info in whatever target folder it is given). Nothing else under the shared
    target is newer than 09:32 (`find -newermt`). It is cargo's own cache in cargo's own tagged folder; builds into
    `<shared>` read `<shared>/.rustc_info.json`, not this one. So 34576 (the brief's count) became 34577.
- The toolchain's own manual (cargo-clean.1, 1.99.1): :14 "With no options, cargo clean will delete the entire target
  directory"; :17-18 "When no packages are selected, all packages and all dependencies in the workspace are cleaned";
  :45-47 `--release` "Remove all artifacts in the release directory"; :63-65 `--target` "Clean for the specified target
  architecture". The page does not say that without `-p` the profile folder is `<target-dir>/<profile>` whatever
  `--target` says; the dry runs show it.
- Bash 5.2.21's `help exec`: "If COMMAND is not specified, any redirections take effect in the current shell ... Exit
  Status: Returns success unless COMMAND is not found or a redirection error occurs." So `exec 9>"$LOCK" || ...` can
  catch a failed open. flock is util-linux 2.39.3.
- Free disk now 8 GB (`df`). spikes/ unchanged since cc8595d (`git diff --quiet`); HEAD 2c5cf54; main-run.sh
  (sha256 ed9934be…) and u4d2-run.sh (d1fd0e7e…) as committed.
- The runners read whole: main-run.sh (the lock at :20-21, `exec 9>` and `flock -x 9`, neither checked; `mkdir -p` of
  the log's folder at :18, no path rule for the log; the provenance grep at the end unanchored and run after its own
  header is written, so it counts the header: cold-run.log and m5-counted.log both show `1 --- provenance ...`);
  u4d2-run.sh (the lock at :45-46, unchecked the same way; the log must be under .gate/u4d2/; its provenance computed
  first and anchored `^ *`). The lock's path is named by main-run.sh, u4d2-run.sh, u4e-run.sh, u4df-run.sh (lanes
  done), te2-5-runs.sh (through main-run.sh), two infra probes (read only), the briefs' direct `flock` forms
  (u4e-brief.md:730-734) and W1-TARGET item 1; check.mjs only in a comment (:2259). Nothing in docs/ or tools/ names
  it; under .claude/ only the worktrees' old copies of these notes do (grep).
- `.claude/worktrees/u4d2-build` (51520b4) still exists (`git worktree list`).
- No custom cargo profile in the workspace (Cargo.toml's `[profile.*]` are per-package overrides of dev and release
  only), so `--profile dev` (folder `debug`) and `--release` cover every profile folder.

## Rulings (one block per item of the brief)

### Block 1. Item 1: the cold figure's route. APPROVE a second cold run now, the wasm32 release folder cleaned through its own tagged root. No partial figure, no full clean, no more dry runs first. U4r's brief waits for it.
- **Why the first run stopped: my error, corrected in writing.** On cargo 1.99.1, `cargo clean` with no `-p` removes
  `<target-dir>/<profile folder>` whole and reads `--target` only when packages are named. So the second clean of my
  M9 (round 2, Block 5) named `<shared>/release` again, which the first had just emptied ("Removed 0 files"), and the
  wasm32 release folder stayed. My round-2 ruling listed exactly this under "Not checked" (from memory), and M9's
  fourth guard was there for it: it stopped the run before check.mjs. Nothing was lost: the host release clean (1134
  files, 293.6 MiB) was meant, and the lock's file and both tags stand. Label: Medium (a wrong command in a ruling,
  which a session hit; its own guard stopped it, so no wrong figure was made). The main session was right to stop and
  bring it here, and right to improvise no other delete.
- **The route: `cargo clean --target-dir <shared>/wasm32-unknown-unknown --release`.** That folder is cargo's own
  triple root and holds its own CACHEDIR.TAG ("created by cargo", Nov 9 02:43), so cargo takes it as a target folder,
  and `--release` removes its `release` folder alone. Proven by dry run (Notes): 131 files, 40.2 MiB, its 158 paths
  equal to `find` of that one folder. Cargo still refuses the untagged release folder itself (exit 101), so the check
  that keeps it from unrelated files stays in force. The run also repeats the host clean (0 files expected), so it
  stands alone whatever has happened since 09:31.
- **Approved under the rules, and mine to approve:**
  - LEFTOVER-DELETE's approved method is `cargo clean --target-dir` (history.md:404-409; Mustafa's words: "a one-off
    delete past the deny list isn't a habit we start"). This is that method, on a folder cargo made and tagged: no
    `rm`, nothing past the deny list, no kit rule needed.
  - W1-TARGET item 3 lets the main session, between waves and with no lane running, empty the shared folder by
    `cargo clean --target-dir`. This removes one part of it (40 MiB of 7.9 GiB), inside that permission. §7.1 item 7
    ("No copy cleans the shared folder") binds the copies, not the main session (item 3's own words). Wave 2 is over
    and no lane runs.
  - It is a technical method inside Mustafa's rules that deletes less than they allow, so nothing goes to him.
- **The other routes, each rejected:**
  - A partial figure (host cold, wasm32 warm): it is not INF-1's cold figure, it would need its own label and a second
    run anyway, and the full cold run costs one more 40 MiB rebuild.
  - A full clean: the step builds release only (WASM_BUILDS, check.mjs:2120-2140), so the debug and doc folders
    (about 7 GB on disk) play no part in the figure, and they are U4r's lanes' warm cache.
  - `-p` with `--target`: it would clean the named packages only; every dependency would have to be named, or the
    figure is half warm.
  - More dry runs: done here; the run's own guards check the result in the run itself.
- **The order:** the cold run now, with main-run.sh as committed (sha256 ed9934be…), then INF2-4's follow-up (Block
  2). In this run the lock's file and the folder's tag are in place and nothing removes them, so INF2-4's hazard cannot
  arise; and the cold run's runner is then the same as the stopped one's, which keeps its record simple.
- **Stops, as M9:** a command before the step fails → the run stops and it comes to me; a call stopped by the 300 s
  limit → M5's rule (`pgrep -a npt-lab`, `pgrep -af loader.mjs`, record, end them), then me; the step over 300 s → me,
  as U4r's det-u4.mjs question; a wasm FAIL for any other reason → me. Nothing is written on the card or in the report
  as the cold figure until the step passes.
- **On the card** (filled from cold-run2.log; replacing the "STOPPED" status; prose lines wrapped at 120):
  "**The cold run, second call** (<HH:MM> UTC; .gate/u4d2-merge/cold-run2.log; uptime <s> s; the advisor's M9 ruling,
  advisor-u4d2-m9.md Block 1): the first call (cold-run.log) stopped at its fourth guard because `cargo clean
  --release --target wasm32-unknown-unknown` with no `-p` cleans `<shared>/release` and ignores `--target` (the
  advisor's error in M9; dry runs). The second cleaned `<shared>/release` again (<n> files) and the wasm32 release
  folder through its own tagged root, `cargo clean --target-dir <shared>/wasm32-unknown-unknown --release` (<n> files,
  <MiB> MiB); its five guards exit 0; then `node check.mjs --only wasm` PASS in <s> s, against a 5-minute budget: the
  four release builds <s> s (npt-lab <s>, npt-lab bites <s>, npt-hash-wasm <s>, its bites <s>), the rest <s> s; no
  call stopped. Cold: every release artifact of both targets built from nothing (dependencies, build scripts, proc
  macros and the workspace's crates). Warm: the toolchain, cargo's registry cache, the debug and doc folders (the step
  reads neither) and, at that uptime, the page cache. This is INF-1's cold figure."
- **In U4's report.** U4's report folds the sub-unit reports (u4-build-plan.md §7.2) and has no file yet, so the figure
  goes into u4d2-report.md, which it folds, as a new bullet right after "The step's time" bullet:
  "- **The cold figure (INF-1; measured after the merge, M9).** <s> s for the wasm step (PASS), against a 5-minute
  budget, from one `node check.mjs --only wasm` in main-run.sh's exclusive call (2026-11-13 <HH:MM> UTC, uptime <s> s;
  .gate/u4d2-merge/cold-run2.log), after both targets' release folders were cleaned by `cargo clean --target-dir`: the
  host's in `<shared>`, the wasm32 one through its own tagged root `<shared>/wasm32-unknown-unknown` (the advisor's M9
  ruling, advisor-u4d2-m9.md). The four release builds took <s> s (npt-lab <s>, its bites <s>, npt-hash-wasm <s>, its
  bites <s>), the rest <s> s; no call stopped. Cold: every release artifact of both targets. Warm: the toolchain,
  cargo's registry cache and the debug and doc folders, which the step does not read. A first try stopped before the
  step at its own guard: `--release --target wasm32-unknown-unknown` with no `-p` cleans `<shared>/release` only."
  And its follow-ups line "The cold figure (INF-1; M9): ..." gains, at its end: " Done after the merge: the cold
  figure's bullet, above." Both by Edit, committed with the card.
- **U4r's brief waits for it: yes.** The figure is the input M9 names for U4r's det-u4.mjs question (over 300 s it
  comes to me as that question), and the run takes minutes. INF2-4's follow-up also comes before U4r's brief (Block 2).

Block 1 written 2026-11-13.

### Block 2. Item 2: INF2-4's runner follow-up. The form below in both runners, its proof P1 to P4, no plan step first (this block is its plan). A full clean keeps the lock's file where it is: the six part-cleans, never the folder-level form.
- **Keep the lock's file where it is; do not move it.** Moving it out of the folder (say to `.gate/`) would make the
  folder-level clean safe, but every runner, every brief's direct `flock` form (u4e-brief.md:730-734 has them) and
  W1-TARGET item 1 would have to change at once; one old form left anywhere and two lanes hold two different locks and
  exclude nothing, silently: a worse failure than INF2-4's, which at least shows. Keeping it costs nothing, because no
  clean this project needs removes the folder (below).
- **The form: main-run.sh.** Lines 12 to 21 (`S=` to `flock -x 9`) become, exactly:
  ```bash
  S=$M/spikes/netcode-proof
  T=$M/.gate/shared-target
  LOCK=$T/.lane.lock
  export CARGO_TARGET_DIR=$T
  export CARGO_NET_OFFLINE=true
  log="$1"
  shift
  if [ ! -f "$T/CACHEDIR.TAG" ]; then
    echo "STOP: $T has no CACHEDIR.TAG (missing, or not cargo's own folder); no runner makes it"
    exit 5
  fi
  mkdir -p "$(dirname "$log")"
  cd "$S" || exit 1
  exec 9>"$LOCK" || { echo "STOP: cannot open the lane lock $LOCK"; exit 5; }
  flock -x 9 || { echo "STOP: cannot take the lane lock $LOCK"; exit 5; }
  ```
  The tag check comes before the `mkdir`, so no log path can ever make the shared folder. Its provenance (lines 73 to
  75) takes u4d2-run.sh's form, which fixes a Low of mine to label now, **MR9-L1** (main-run.sh's provenance list counts
  its own header line: the grep is unanchored and reads the log after the header is in it; cold-run.log and
  m5-counted.log both show `1 --- provenance ...`; it names no crate path, so no provenance claim was ever wrong; it
  rides this change, which is needed anyway):
  ```bash
  prov=$(grep -E "^ *(Compiling|Checking|Documenting) npt-" "$log" | sed 's/^ *//' | sort | uniq -c)
  {
    echo "--- provenance (Compiling/Checking/Documenting npt-* lines, counted):"
    echo "$prov"
  ```
  And its header comment gains one sentence, after "...exits with that command's code.": "It stops with exit 5 before
  taking the lock if the shared target has no CACHEDIR.TAG, and before any command if the lock's file cannot be opened
  or the lock taken (INF2-4: advisor-u4d2-m9.md, Block 2); no runner ever makes the shared target's folder."
- **The form: u4d2-run.sh** (the template every later lane runner is copied from). Its lines 17 and 18 become three:
  ```bash
  T=$M/.gate/shared-target
  LOCK=$T/.lane.lock
  export CARGO_TARGET_DIR=$T
  ```
  The same four-line `if [ ! -f "$T/CACHEDIR.TAG" ]; then ... fi` block as main-run.sh's goes between line 42 (`[ -d
  "$S/crates" ] || ...`) and line 43 (`mkdir -p`). Lines 45 and 46 become:
  ```bash
  exec 9>"$LOCK" || { echo "STOP: cannot open the lane lock $LOCK"; exit 5; }
  flock "$mode" 9 || { echo "STOP: cannot take the lane lock $LOCK"; exit 5; }
  ```
  The same header sentence goes after "...exits with the first non-zero code." Its provenance is already right. Exit
  5 is new in both, apart from 1 (cd), 2 (usage), 3 (disk) and 4 (busy). The STOP lines go to stdout, as u4d2-run.sh's
  STOPs already do; the caller reads the exit code first.
- **The other runners.** te2-5-runs.sh runs through main-run.sh, so it is covered. u4e-run.sh and u4df-run.sh are
  retired with their lanes: no brief names them again (a card line). Every later lane runner is copied from u4d2-run.sh
  after this change, and its saved diff against u4d2-run.sh (as u4d2-run.sh.diff is against u4e-run.sh) leaves the
  tag check and the two lock lines unchanged. A brief's direct `flock <file> <command>` form already fails closed:
  flock opens the file itself, cannot open it in a missing folder and then runs nothing; no brief may make the folder.
- **The proof** (logs under .gate/u4d2-merge/, each result one card line):
  - **P1:** `bash -n` on both runners, exit 0.
  - **P2:** each runner's diff against its committed version, saved beside it (`main-run.sh.inf2-4.diff`,
    `u4d2-run.sh.inf2-4.diff`), holding only this block's lines.
  - **P3, the one lock, both runners; MR9-L1:** `bash main-run.sh <M>/.gate/u4d2-merge/inf2-4-p3a.log sleep 8` in the
    background; two seconds later `bash u4d2-run.sh <M>/.claude/worktrees/u4d2-build <M>/.gate/u4d2/inf2-4-p3b.log
    true`. Both exit 0; p3b's "held (-s) at" time is not before p3a's "end" time; p3a's provenance block holds no
    count line (an empty line; before the fix it held `1 --- provenance ...`). If `.claude/worktrees/u4d2-build` is
    gone by then, the second call is instead the first call of U4r's lane runner, with the same check, shown at U4r's
    brief.
  - **P4, fail closed:** for each runner, three copies in the notes folder, each made by a small script that changes
    exactly one line and saves its diff: (a) `T=$M/.gate/no-such-target`; (b) `LOCK=$M/.gate/no-such-dir/.lane.lock`;
    (c) the flock line's fd `9` changed to `99` (never opened). Each copy runs with the one command `touch
    <M>/.gate/u4d2-merge/inf2-4-ran-<runner>-<case>` (u4d2-run.sh's copies with a log under .gate/u4d2/ and the
    u4d2-build worktree): exit 5, its STOP line on stdout, the marker absent, and in (a) and (b) the missing folder
    still missing (`test ! -e`, before and after). The copies stay in the notes folder until the checkpoint is folded.
- **No plan step with me first.** The runners are helper scripts in the notes folder, not kit or gate files, and
  CLAUDE.md counts notes as no edit. This block is the plan: the exact lines, the one new variable `T`, the new exit
  code, the proof. The main session applies it by Edit, runs P1 to P4, and records them on the card; I check the two
  diffs and the proof logs at my check of U4r's brief, which comes anyway (u4-build-plan.md §8 item 1). Any line that
  departs from this block, wrapping aside, comes to me before it is used.
- **How a full clean works now, with the lock's file inside the folder:** never by the folder-level form (`cargo clean
  --target-dir <shared>` with no profile and no `--doc`, which `--target <triple>` alone also is). A full clean is the
  six part-cleans, in one main-run.sh call, between waves, with no lane running:
  ```text
  cargo clean --target-dir <shared> --release :: cargo clean --target-dir <shared> --profile dev ::
  cargo clean --target-dir <shared> --doc :: cargo clean --target-dir <shared>/wasm32-unknown-unknown --release ::
  cargo clean --target-dir <shared>/wasm32-unknown-unknown --profile dev ::
  cargo clean --target-dir <shared>/wasm32-unknown-unknown --doc ::
  test -f <shared>/.lane.lock :: test -f <shared>/CACHEDIR.TAG :: test -f <shared>/wasm32-unknown-unknown/CACHEDIR.TAG ::
  test ! -e <shared>/release :: test ! -e <shared>/debug :: test ! -e <shared>/doc ::
  test ! -e <shared>/wasm32-unknown-unknown/release :: test ! -e <shared>/wasm32-unknown-unknown/debug ::
  test ! -e <shared>/wasm32-unknown-unknown/doc :: ls -A <shared> <shared>/wasm32-unknown-unknown
  ```
  The dry runs (Notes) show these six remove 34570 of the folder-level form's 34577 files; the 7 left are the two
  roots' small files (the lock, both tags, two rustc caches, two rustdoc fingerprints), plus the empty tmp/. So it
  empties every artifact while the lock's file and both tags stay, and the final `ls -A` shows any folder a later
  target or profile adds. It does not need INF2-4's follow-up first (it removes neither the folder, the lock's file nor
  a tag), but none is planned: the cold figure needs release only, debug and doc are U4r's warm cache, and 8 GB are
  free. A folder that must itself go comes to me (the lock moves first, everywhere at once).
- **If the folder is ever missing** (an error, a new machine): every runner stops with exit 5, so no lane can build.
  The main session, with no lane running, makes it by one cargo build into it, which writes cargo's tag: from
  spikes/netcode-proof, `CARGO_TARGET_DIR=<shared> CARGO_NET_OFFLINE=true cargo build --locked -p npt-log`, then `test
  -f <shared>/CACHEDIR.TAG`. Never `mkdir`. This is the one case Block 6's lesson (round 2: never point cargo at a path
  that does not exist) does not cover: the folder cargo makes there carries its tag, so the approved method removes it.
- **W1-TARGET item 3, amended (my text):** "Between waves, with no lane running, the main session may empty the shared
  folder by the six part-cleans in one main-run.sh call (advisor-u4d2-m9.md, Block 2), or any part of it by `cargo
  clean --target-dir` on one of its two tagged roots with `--release`, `--profile dev` or `--doc`, each with its `test`
  guards; never by a clean without one of those three (the folder-level `cargo clean --target-dir .gate/shared-target`,
  or one with `--target` alone) while the lane lock's file lives in it; never `--target` without `-p`."
  advisor-u4-wave1.md stays as written, a record; the card's rules carry this amendment, and later briefs cite this
  block.
- **Two lessons for every later brief that cleans or dry-runs cargo:** (1) never pass `--target` to `cargo clean`
  without `-p`: with a profile it changes nothing, without one it cleans the whole folder; a triple's folders are
  reached through `--target-dir <shared>/<triple>`. (2) `cargo clean --dry-run` deletes nothing, but it may write
  cargo's `.rustc_info.json` into the `--target-dir` it is given: mine did (356 bytes in the triple root, 09:37:38).
  That file is cargo's own cache in its own tagged folder; it stays (a folder-level clean would take it, the part-cleans
  keep it), and no session removes it by hand.

Block 2 written 2026-11-13.

## The runs I approve (exact commands, in this order; `<M>` is /home/user/Shamble-Frontlines)
1. **Before:** `pgrep -a cargo; pgrep -a rustc; pgrep -a npt-lab; pgrep -af loader.mjs` all empty (no lane running),
   and `cd <M> && git diff --quiet cc8595d HEAD -- spikes/` true. Otherwise it waits, or comes to me.
2. **The cold run, second call** (one call, in the background or with a timeout of at least 600 s):
   ```text
   bash <M>/.checkpoints/netcode-proof-test/main-run.sh <M>/.gate/u4d2-merge/cold-run2.log \
     cat /proc/uptime :: \
     cargo clean --target-dir <M>/.gate/shared-target --release :: \
     cargo clean --target-dir <M>/.gate/shared-target/wasm32-unknown-unknown --release :: \
     test -f <M>/.gate/shared-target/.lane.lock :: \
     test -f <M>/.gate/shared-target/CACHEDIR.TAG :: \
     test -f <M>/.gate/shared-target/wasm32-unknown-unknown/CACHEDIR.TAG :: \
     test ! -e <M>/.gate/shared-target/release :: \
     test ! -e <M>/.gate/shared-target/wasm32-unknown-unknown/release :: \
     node check.mjs --only wasm
   ```
   Expected: the host clean 0 files, the wasm32 clean about 131 files and 40.2 MiB, the five guards exit 0, then the
   step. Its stops are Block 1's. On a PASS within 300 s: the card's and the report's words (Block 1).
3. **INF2-4's follow-up** (Block 2), after the cold run: the edits by Edit, then
   - P1: `bash -n <M>/.checkpoints/netcode-proof-test/main-run.sh` and `bash -n
     <M>/.checkpoints/netcode-proof-test/u4d2-run.sh`, exit 0;
   - P2: the two saved diffs, only Block 2's lines;
   - P3: `bash <M>/.checkpoints/netcode-proof-test/main-run.sh <M>/.gate/u4d2-merge/inf2-4-p3a.log sleep 8` in the
     background, then after 2 s `bash <M>/.checkpoints/netcode-proof-test/u4d2-run.sh
     <M>/.claude/worktrees/u4d2-build <M>/.gate/u4d2/inf2-4-p3b.log true`;
   - P4: for each runner `<r>` in main-run and u4d2-run and each case `<c>` in a, b, c, the copy
     `<M>/.checkpoints/netcode-proof-test/<r>.inf2-4-<c>.sh` (one line changed, its diff saved), run as
     `bash <copy> <M>/.gate/u4d2-merge/inf2-4-p4-<r>-<c>.log touch <M>/.gate/u4d2-merge/inf2-4-ran-<r>-<c>` for
     main-run, and `bash <copy> <M>/.claude/worktrees/u4d2-build <M>/.gate/u4d2/inf2-4-p4-<r>-<c>.log touch
     <M>/.gate/u4d2-merge/inf2-4-ran-<r>-<c>` for u4d2-run; each exit 5 with its STOP line, no marker, and
     `<M>/.gate/no-such-target` and `<M>/.gate/no-such-dir` absent before and after.
4. **Then** the card (the cold figure, P1 to P4, MR9-L1 fixed, W1-TARGET item 3 as amended, the two lessons, the
   retired runners), the report's bullet, a checkpoint save; then U4r's brief, which I check with the two diffs and
   the proof logs.

## Verdict, per item of the brief
| Item | Verdict | In one line |
|---|---|---|
| 1. The cold figure's route | APPROVE a second cold run now | the wasm32 release folder through its own tagged root (`--target-dir <shared>/wasm32-unknown-unknown --release`: 131 files, 40.2 MiB by dry run); the host clean repeated; five guards |
| 1. The partial figure | REJECT | not INF-1's figure; the full one costs a 40 MiB rebuild |
| 1. A full clean, `-p`, more dry runs | not needed | the step builds release only; `-p` leaves dependencies warm; dry runs done here |
| 1. My M9 command | my error, Medium, corrected | `--target` without `-p` changes nothing; the guard stopped the run; nothing lost |
| 1. The card and U4's report | the words in Block 1 | u4d2-report.md's new bullet (U4's report folds it) and the card's line |
| 1. U4r's brief | waits for the figure and for the follow-up | minutes of work; the figure is U4r's det-u4.mjs input |
| 2. INF2-4's follow-up | APPROVE the form in Block 2 | a tag check before any mkdir; the open and the flock checked; exit 5; MR9-L1's provenance fix |
| 2. Its proof | P1 to P4 | syntax; the diffs; one lock across both runners; three fail-closed cases per runner |
| 2. A plan step first | none: Block 2 is the plan | I check the diffs and logs at U4r's brief; any other line comes to me first |
| 2. A full clean | the six part-cleans, never the folder-level form | 34570 of 34577 files; the lock's file and both tags stay; the lock does not move |
| 2. W1-TARGET item 3 | amended (my text) | the part-cleans or a tagged root's clean; never the folder-level form while the lock lives in it |

Risk tier 1 (helper scripts in the notes folder and a cargo clean inside the shared build folder; no kit, gate, CI or
game file). No review round: these are a ruling and a follow-up, not a PR head. PR #40 stays needs-mustafa as before.

## Decisions needed
None. The new clean is LEFTOVER-DELETE's approved method (`cargo clean --target-dir`) on a folder cargo made and
tagged, run by the main session between waves as W1-TARGET item 3 allows, deleting less than it allows; the runner
change makes two helper scripts stricter; W1-TARGET item 3's amendment narrows my own text. No word or decision of
Mustafa's changes, nothing a player sees, hears or feels is touched, and nothing is irreversible beyond a cache cargo
rebuilds.

## For Mustafa
Nothing to do. For his information at PR #40's review: the first cold run stopped at its own safety check because a
clean command in my ruling cleaned the wrong folder; nothing was lost, and the re-run uses cargo's own clean on one
narrower folder.

## What I checked, and what I did not
**Checked myself (reading, git, sha256 and `cargo clean --dry-run` only; no build, no real clean, no runner run):** my
role file from main; the brief whole; my round-2 ruling's Blocks 2 and 5 and its "Not checked" list; W1-TARGET;
§7.1; LEFTOVER-DELETE and the shared-target rule at history.md:404-424; main's deny list; the cold run's log whole;
m5-counted.log's provenance; both runners whole and their hashes against HEAD; every file naming the lock's path; the
shared target's tree, counts and tags; the processes (none); spikes/ against cc8595d; check.mjs's WASM_BUILDS and its
cargo calls (no `--target-dir`); the spike's cargo config and the workspace's profiles; the dry runs (Notes: the new
form with `-v`, its path list `diff`-equal to `find` of the folder; the untagged folder refused; the old M9 form 0
files; the six part-cleans and the folder-level form, each count equal to `find`); cargo-clean.1 from the toolchain;
bash's `help exec`; flock's version; free disk.

**Not checked:**
- I ran neither the cold run nor any runner: the route rests on the dry run (131 files, the folder's exact list) and
  the run's own five guards, which stop it if the clean does otherwise.
- Cargo's clean source (not on this machine): "no `-p`, so `--target` is ignored" rests on the dry runs and the log's
  two counts, not on reading the code.
- That `exec 9>"$LOCK" || ...` gives exit 5 in practice: `help exec` says a redirection error returns failure; P4 (b)
  proves it, and in POSIX mode the shell would exit non-zero on its own, which still fails closed.
- That the cold step fits 300 s (the infra seat's bound is about 80 s with a warm page cache): the run measures it.
- My dry runs wrote one 356-byte cache file (Block 2's lesson 2), which I did not expect; I found it by `find
  -newermt` and nothing else changed.
