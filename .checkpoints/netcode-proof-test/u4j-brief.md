# U4j builder brief (the judge and marks change), for the first free lane

Written 2026-11-09 about 22:25 UTC by the main session. Under the advisor's U4S-BUILDER ruling (advisor-u4.md:365-371)
a builder agent may build U4j only in a free slot (never a 4th agent; reviewers count), as general-purpose, opus,
effort xhigh, in an isolated worktree under §7.1, on its Lanes files; the main session merges it with §5's reviewers
(test-engineer, netcode-architect, security-auditor), and its report says who built it. The agent's prompt is this
file's "Prompt" section, with `<base>` replaced by the spike branch's head at launch.

## Prompt
You build sub-unit U4j of the netcode proof test (spike branch claude/netcode-proof-test-plan-hcky5g): the judge and
marks change in npt-judge, npt-log and marks/, in a fresh isolated git worktree (your current directory). Other lanes
build at the same time in their own worktrees (U4a crates/proto and check.mjs; U4c crates/l1; U4d crates/sim; U4b
crates/server, done and waiting for review).

**Start**
1. `git fetch origin claude/netcode-proof-test-plan-hcky5g`, then `git merge --ff-only <base>`; if that cannot
   fast-forward, `git switch -c u4j-build <base>` in your worktree; check `git log -1` shows `<base>`.
2. Notes: `.checkpoints/netcode-proof-test/u4j-build.md` in your worktree: plan first, a Progress line per item, DONE
   at the end. Save as you go. If the file exists from a cut run, read it and continue after its last line.
3. Read the approved plan `.checkpoints/netcode-proof-test/u4-build-plan.md` (rev 2 with the advisor's conditions and
   rulings' edits; sha256 0d69332c…) and the advisor's rulings `.checkpoints/netcode-proof-test/advisor-u4.md` (the U4
   verdict: conditions 2 and 4 and nits (c) and (f); "Rulings"; "Ruling: U4s"; "Ruling: F1 and F2").

**What to build: the approved plan, exactly.** §5's "U4j, the judge and marks change" (its Builds and Tests), and the
sections it builds to: §4.6 (the client-held hash: whole-tick hash only on a complete tick, the sentinel for a refused
group, a lost group leaves the tick unlogged and counted; the six decoded-hash fixtures), §4.12 (logs and the
precondition table, Q22), §2's rows for C4, the -18 split, the scene selects and `hs` (u4-build-plan.md:147-154), and
the rulings Q11, Q20, Q22. The advisor's condition 2: no `PRE-` id, no marks/prechecks.json, no `"kind": "precheck"`
rule, nor their tests (-30's pre-check is a recorded npt-lab test the judge never reads); README §1 gains the
`loopback` layer value in one line, with no judge change. Condition 4: README §4's `pre:PM-0001-2` sentence (:193-197)
gains -22's exception and its reason. Nit (c): README states how `hs` writes a client with no hash at a tick (`null`),
the form the six fixtures use. Nit (f): README and SERIES fix the format of the run-structured bound's machine-readable
line and of the WebSocket practice run's outcome record (fixes.rs's unit writing that format comes later; keep the
format readable by the judge's parser). SERIES.md gains the rows rev 2 adds (`snap_incomplete`, `burst_eviction`, the
clamp events, the precheck suite) and the figure rows' formulas. No public item beyond what these need; list every
pub item you add, by crate (A19). npt-judge's dependencies stay npt-log and serde (check.mjs's deps step).

**Rules (the U4 plan §7.1; Mustafa's rule of 2026-11-09):** every cargo command, check.mjs included, runs with
CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target and CARGO_NET_OFFLINE=true. Before each build,
touch every file under spikes/netcode-proof/crates in your copy. Each run's test count matches the expected delta;
`--list` names the new tests; a bite or fixture that must fail is shown failing by name; a mismatch means another
copy's artifact ran: touch, rebuild, never report that run. Other lanes build in the same shared folder: cargo's lock
serializes you. No copy cleans the shared folder. `df -h /home/user` before each build; below 2 GB free, stop and
report. **Your files only: crates/judge, crates/log, marks/ (marks.json, README.md, SERIES.md and the fixtures)**; no
manifest or Cargo.lock edit, no check.mjs edit (U4a owns it in wave 1): if a check.mjs change is needed, record it in
your report for the main session. Run git only inside your copy, never `git -C` or `git -c`. Write files with the
Write and Edit tools, never shell redirects, heredocs or `python3 -c`. Commit nothing. Planted failures are planted in
place, one at a time, each file restored and sha256-checked; never in a plain copy of the source.

**marks.json edits:** every row you change keeps the drift test and check.mjs's marks step passing (the rows' draft
sha256s against docs/adr/ stay as they are: you edit marks.json's checks and parts, never the ADR rows). State each
marks.json change in your report as before and after.

**At the end, all must pass in your copy:** `node spikes/netcode-proof/check.mjs`, all 10 steps, twice in a row (say
the counts as deltas on the base's measured and bites counts); `npt-judge --check-marks` on marks.json; clippy -D
warnings host and wasm32 in both builds (check.mjs's steps); `cargo +1.99.1 fmt --all --check`.

Return: worktree path, base SHA, files changed, pub items added by crate, the tests and fixtures with results (each
PASS, FAIL, gap and INVALID fixture by name), the marks.json changes, the check lines and counts, what L-c8's judged
re-run (U4c) and U4r need from this judge, and anything you could not do within your files.
