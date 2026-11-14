# BRAWL-1's builder brief (for the advisor's conformance check, condition 17)

Written by the main session on 2026-11-14 at about 23:55 UTC. It carries the approved plan (brawl1-plan.md) and the
advisor's plan review (advisor-brawl1-plan-full.md, conditions 1 to 18, choices Q1 to Q8) into one set of instructions.
It decides nothing new. Where the plan and the review differ, the review wins, and this brief follows the review.

## 0. Who you are and what binds you

- **Who:** you are BRAWL-1's builder for the netcode proof test (PR #40, spike branch
  claude/netcode-proof-test-plan-hcky5g). You are one builder, alone, in your own isolated worktree.
- **What binds you, in order:**
  1. advisor-brawl1-plan-full.md: conditions 1-18 and the answers to Q1-Q8;
  2. brawl1-plan.md;
  3. the rulings it answers: advisor-u4r-sets-r2-full.md (finding BRAWL-1, Fix (a) to (d)); advisor-u4r-r2-full.md
     (N2-1, TE-R2-1, TE-R2-2, RR4-L1, RR4-L2, item 18); advisor-u4r-r1-full.md (EA-U4R-1, EA-U4R-2, RR3-L2).
  All are in /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/.
- **The code facts:** brawl1-facts-full.md (file:line as of 2026-11-14). Check each before you rely on it. The crates
  have not changed since faf42aeb; ops/runs/ changed at 493b3d7 (U4r-F3).
- **What is not yours** (the main session does these after your build):
  - the review round;
  - the merge step (condition 9);
  - the marks/SERIES.md edit (condition 10: you draft the rows);
  - the two 10-minute evidence runs (condition 14);
  - the re-run sets (condition 15);
  - the -30 pre-check set's re-run, at U4h (plan section 5);
  - c-u4r-r2.mjs's expect texts, restated at r2's next run.

## 1. Start

1. **Your worktree** starts at the spike branch's head at launch; the launch prompt names it as `<base>`. Write in your
   notes the worktree's path and branch, `git rev-parse HEAD` (equal to `<base>`), and `git status --short` (empty).
2. **Your notes:** /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/brawl1-build.md.
   - Write your plan first, then a line after each finished item.
   - A restart reads them first and continues after the last complete block.
3. **Save as you go.** Each finished item goes to the notes at once, never only in your context.
4. **The runner:** /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4r-run.sh, unchanged, sha256
   6efb91572cf06fd9ad6472c077cd39c9a59bfed8d4a3dac561992aafd5729bc8. Check the sha256 before the first use.
   - Logs go under /home/user/Shamble-Frontlines/.gate/u4r/brawl1/.
   - Use `-x` (exclusive) for every check.mjs run, det-u4.mjs, every plant, every bite run and every counted run.
   - It runs with CARGO_TARGET_DIR on .gate/shared-target and takes the lane lock.
5. **Disk** (condition 16): about 13.2 GiB is free now and the shared target is cold. Your first check.mjs run refills
   it, about 8.2 GiB, which leaves about 4.7 GiB free.
   - Rule 6: 4 GB free before any wasm32 or release build, and 2 GB before any other build. The runner checks 4 GB
     before every command.
   - A floor stop is reported whole, never cleaned around.
   - No build target of your own, and no 10-minute run.

## 2. Limits

- **T4 item 1, word for word:** "Write files with the Write and Edit tools only (no sed, no shell redirect, no python
  writes; never `cargo fmt` writing files: apply rustfmt's diff by Edit); an agent with no Edit tool rewrites its notes
  file whole with Write."
- **Git:** only in your worktree, as `cd <worktree> && git ...`. Never `git -C`, `git -c`, `git checkout` or
  `git restore`. Commit nothing and push nothing.
- **No kit or gate file:** nothing under .claude/, no CLAUDE.md, nothing under .github/, not tools/gate.mjs. Do not edit
  marks/ either: draft the SERIES.md rows in your notes (section 4).
- **Secrets and network:** never read .env or secrets. No network (CARGO_NET_OFFLINE).
- **Pass or fail** comes from the exit code and the test runner's own summary line, never from a grep.
- **A fresh copy** reads its notes and the diff in its worktree before it changes anything.

## 3. The work, in order (plan section 7; conditions 6 and 7)

### Part 1: it moves no pin; prove it at the old pins before Part 2

**Before Part 1's first edit,** capture the precheck tests' `--nocapture` output (r6, the late-command bite, wallclock)
into a log under .gate/u4r/brawl1/ (condition 6 (ii), the "before").

1. **EA-U4R-2** (plan section 6):
   - `pub(crate) type Sim = npt_sim::World`.
   - One `pub(crate)` server constructor, used by run and run_leg. It takes scene, seed, first tick, clients, event
     start, arm tick and bites. run_leg passes Bites::default() and event start 0, then sets decoder_bite as today.
   - logs.rs and precheck.rs name Sim.
   - The printed "World::new(scene, ..)" strings (precheck.rs:691, :705) stay word for word.
   - lab_room uses the constructor if it fits.
2. **EA-U4R-1, the narrow form** (plan section 6):
   - one shared function for RQ8's steps 1-2 (accept; server.io with admitted and ended) plus the due count, with each
     caller keeping its own per-tick body;
   - run_leg uses SimClock;
   - admitted and ended go to each caller's own sinks: the runner's logs.admit and logs.gone, and the pre-check's maps
     and o.ended.
3. **N2-1 with TE-R2-1** (plan section 6; Q5 (a); ADV-BP-1):
   - closed_checks gains the fourth check, with the ruled message: "the roster still holds slot {slot}: a rejoin would
     be a replacement". Its doc says four checks.
   - fp1's doc rests "not a replacement" on the roster.
   - The no-op assert on `ended` is dropped.
   - **The plants,** each restored by Edit with its sha256 checked:
     - room.leave deleted: two of the four fail by name, "the room still holds slot 0: its slot was not freed" and the
       roster message;
     - roster.leave (room.rs:691) deleted: the new check alone fails.
4. **TE-R2-2 with RR4-L2** (plan section 6; condition 12):
   - **Order:** the 64-bit checks come first and keep their messages. Then RR4-L2's four refusals:
     - a run's last tick above u64::from(u32::MAX);
     - the last burst's tick above the same;
     - --bursts above 1,000;
     - --burst-every-s 0 with more than one burst.
     Each refusal is a usage error naming its options.
   - **Where the caps apply:** the burst caps where bursts are used (B1, as today's guard); the last-tick cap on every
     profile.
   - **fp3's cases:**
     - TE-R2-2's three. Case 1 is on a B1 base line. Case 2 equals the ruled message. Case 3 is in full digits:
       `--warmup-s 300000000000000000 --measured-s 300000000000000000 --after-max-s 100000000000000000`, pinning
       "--after-max-s: the run's last tick overflows 64 bits".
     - RR4-L2's ruled case: `--profile B1 --bursts 2 --burst-every-s 600000000000`, naming --burst-every-s.
     - One case each for the bursts cap, every-s 0, and an --after-max-s past the u32 cap that fits in 64 bits.
   - The comment at scenario.rs:1442 says exactly what is bounded.
5. **RR4-L1** (plan section 6):
   - `const MAX_ENTRY_TICKS: u32 = 36_000;`, refused right after the bytes cap and before the runner checks, with the
     ruled message.
   - r10_fp3: 36,001 is refused by name, and a synthetic entry at 36,000 passes.
6. **RR3-L2 (a)** (plan section 6):
   - a Logs flag set from open's `dir.is_some()`;
   - on a no-log run, filed() resolves only and never grows `res`; `last` stays;
   - a new logs.rs unit test.

**Part 1's proofs** (condition 6), all at the old pins 6933b56a, 5b79abb2 and fad8f67d, recorded in the notes before
Part 2 starts:
- (a) check.mjs, all steps PASS.
- (b) det-u4.mjs PASS. `uptime` comes first, in the same runner call, with no other lane running. A non-cargo sum over
  240 s stops you and is reported whole with the uptime, never retried (condition 8).
- (c) `npt-lab hash --manifest crates/lab/precheck-recordings.json` byte-equal.
- (d) **The clean pre-check pair,** run_leg's half: record SS-P0-s4 at its entry's settings (sha256 6933b56a), then run
  `npt-lab precheck --manifest crates/lab/precheck-recordings.json --recording <file> --commit <sha>` with no bite, on
  the measured build. Its output lines must equal the "## pair clean" section of
  spikes/netcode-proof/results/u4r-precheck-30/faf42aeb034c/runs.txt line for line, except the build line (commit and
  binary sha256). It takes about 320 s and 17 MB.
- (e) The precheck tests' `--nocapture` output after Part 1 diffs identical to the "before" log.

Any difference in (a) to (e) stops you, and it comes to the advisor whole.

### Part 2: BRAWL-1 itself

1. **The npt-client read item** (Q2 (a); condition 5): one read-only pub method on ClientEndpoint (`&self`).
   - It returns, by value, an `Option` of the own soldier's decoded grid x, y (and z) and its dead bit (flags bit 3).
   - The source is the newest tick that is complete (Tick::complete) and filed Filed::Hash, with ticks compared by the
     endpoint's wrap rule.
   - No reference into the ring, no float, no new dependency. Its doc says it reads the client's own decoded state.
   - Its npt-client test shows: None before the first complete tick; after a complete tick, Some equal to the own
     record's x, y, z and dead bit; and no change from a newer incomplete, refused or sentinel tick.
   - The aim stays on the mirror view (build ruling 1, point 7).
2. **npt-bots** (conditions 1 and 4): no new pub item. A crate-private decision function, called by step(), takes the
   read item's result. The leash, for Kind::Brawl only (the open-loop scripts are unchanged):
   - toward the cluster centre (the origin);
   - R_IN at least 1,300 cm; R_OUT at least R_IN + 100 cm and at most 1,500 cm;
   - past R_OUT, a vector toward the origin with x^2 + y^2 <= 127^2, so the sim never takes its scale path;
   - back inside R_IN, the open-loop walk;
   - None or dead keeps the walk;
   - the sideways part may stay or go;
   - aim and fire as today.
   - The constants are cm against cm, with no rate-derived constant (carry 39; FP-7 item 1). Record every tuned value.
   - npt-bots' library code names no World or SimCore. Only its #[cfg(test)] module does.
3. **The figure** (Fix (b); condition 10): in Logs::room_tick, server side, take the slots (with their areas) whose
   Soldier record has `area != 0`.
   - The list is written on every logged tick where it is not empty, in any phase.
   - Counts counts measured ticks, and the summary line prints the total.
   - Every name you write (the tick name, and any lab end-event field carrying the total) gets a SERIES.md row "none
     (recorded)". Draft the rows in your notes.
   - No judge change and no marks.json change.
4. **The tests,** all in a #[cfg(test)] module in npt-bots' src/, with no Cargo.toml change (conditions 2, 3, 4 and 13):
   - **The steering unit test** (plan section 4.1's list), which also asserts x^2 + y^2 <= 127^2 on every leash vector.
   - **The sim-only test:**
     - Setup: `World::new(scene, seed, 63_736)`, SS, 48 clients, seeds 1 to 3. Lengths come from npt_sim::ticks_for_ms:
       610,000 ms for the full run, 120,000 ms for the short copy. Never a literal tick count.
     - Pattern (i), barred: a command every tick, each slot fed its position from 8 steps earlier.
     - Pattern (ii), barred: a seeded skip pattern, where a skipped tick re-runs the last command unchanged, as
       InputBuffer::take does, with the same 8-step delayed feed. Its rate and longest streak are stated in the doc and
       printed.
     - Pattern (iii), printed only: B1's bursts. Every slot re-runs its last command for ticks_for_ms(1_000) ticks at ten
       burst starts, the first at 30 s and then every 60 s. Its result (any area != 0, and the largest distance from the
       centre) goes into your notes for U4g's brief.
     - The bar: no Soldier record with `area != 0` on any tick, in (i) and (ii).
     - Printed: each run's largest distance from the centre. The stuck print names any soldier whose position stays
       inside a 100 cm square for more than 300 commands given, with its slot, tick and position.
     - **A soldier the full test names stops you before anything else:** it comes to the advisor (condition 2).
   - **The full test** is `#[ignore = "<reason>"]` (the precedent is net l2 socket_tests.rs:512). Run it by name, in
     release, at the head you hand over, with its log kept under .gate/u4r/brawl1/.
   - **The short copy** (seed 1, ticks_for_ms(120_000), both barred patterns) runs in every check.mjs. Record its time in
     each build and the full test's release time. If the short copy takes more than 60 s in either build, shorten it and
     record the change.
5. **The re-pin** (Q3 (a); conditions 7 and 8): only after Part 1's proofs and Part 2's tests pass and are in your
   notes.
   - Regenerate SS-P0-s4 (precheck-recordings.json), and SS-P2-s1 and ES-P3-s2 (det-recordings.json), each by its own
     route: `npt-lab hash` prints the new bytes and sha256.
   - The manifests' diff changes only those three entries' `bytes` and `sha256`. Every other field and both "about"
     texts stay.
   - Write each new pin beside the old one in your notes.
   - Then check.mjs, det-u4.mjs (condition 8 as above) and the precheck manifest regeneration must pass at the new
     pins.
   - Any other mismatch, or a mismatch at the old pins, stops you and comes to the advisor whole. Never retry.

## 4. What you hand over

- **The worktree's uncommitted diff, complete.** The main session saves it as wip/brawl1.patch at each snapshot and
  merges it by condition 9's M-list:
  1. the merged bytes equal the reviewed bytes (cmp and the tree fingerprint);
  2. one counted main-run.sh call: uptime, check.mjs with all 11 steps, det-u4.mjs, and the precheck regeneration at
     the new pin;
  3. the gate;
  4. CI;
  5. the advisor's review of the pushed head.
- **The SERIES.md rows,** drafted in your notes.
- **The size:** `git diff --stat` at the end (SIZE-CHECK: planned about 700 lines, 450-750 with the conditions).

## 5. Stop rules

- **Your context is finite.** Save each item at once. When most of it is used, stop at a clean point (no plant in place,
  no half edit, no run in flight), write a "## State" block, and end. The next copy continues from your notes and the
  diff.
- **Stop and report whole, each time:**
  - any difference in Part 1's proofs (condition 6);
  - det-u4.mjs over 240 s (condition 8);
  - any soldier the full test names stuck (condition 2);
  - a floor stop (condition 16);
  - the size past about 1,400 changed lines (the main session tells Mustafa in three lines);
  - any mismatch the re-pin ruling does not cover (condition 7).

## 6. Your report (your final message, also written at the end of your notes)

1. **Part 1:** each item, with file:line; proofs (a) to (e), each with its log path and result.
2. **Part 2:** the read item; the decision function; the leash's values (R_IN, R_OUT, the sideways part, if kept); the
   figure's names and the drafted SERIES.md rows.
3. **The tests:** the unit test; the short copy's time in each build; the full test's release time and its printed
   results per seed and pattern (largest distance; any stuck soldier named; pattern (iii)'s result).
4. **The re-pin:** the three entries' old and new pins; det-u4.mjs and the precheck regeneration at the new pins, with
   the uptime and the non-cargo sum.
5. **check.mjs's final run:** its counts.
6. **The size:** `git diff --stat` against about 700.
7. **Anything that stopped you,** whole.

After you: the main session runs one review round on the final diff: test-engineer, netcode-architect, rust-reviewer and
engine-architect, in two batches of at most 3, each at xhigh (condition 11). Then the merge step (condition 9), the
evidence runs (condition 14) and the re-run sets (condition 15). Your report must record the advisor's two corrections
(ADV-BP-1, ADV-BP-2) where they touch your work (condition 18).
