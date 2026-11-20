# U4f2's review round 1 (2026-11-20): the shared brief for the four seats

## What changed
- **The unit:** U4f2 by `.checkpoints/netcode-proof-test/u4f2-brief.md` rev 1.2 (filled at launch; `<base>` a289def;
  Mustafa's SIZE-U4F (a), history.md:6896-6903, and U4F2-SIZE (a), history.md:7255-7271): PM-0001-21's forged-command
  test (forgers of kinds 3, 7, 8, 9, 10 and 11 in npt-bots, every twin), the refusal attribution (C-U4F2-3), f28's B
  halves, C-U4F2-5 (Steady stands still), -21's rows and set in the run scripts, and the four riders (C-U4F2-7 with
  T-6, C-U4F2-8, ADV-U4FP-3).
- **The diff:** 13 tracked files, +1,715 -175, plus the new `spikes/netcode-proof/crates/lab/src/forged.rs` (362):
  2,077 added, under the 3,000 stop. Read it as the patch `.checkpoints/netcode-proof-test/wip/u4f2-build.patch`
  (sha256 e9a0ae3e3f48af18a7a947fdf9aba849061fa32fbd91876a948c46f3046406b). The base files are the main folder's
  `spikes/` (equal to a289def's). The built tree is the builder's worktree
  `/home/user/Shamble-Frontlines/.claude/worktrees/agent-aa62d31b6fa5b9807`: read its files with the Read, Grep and
  Glob tools at their absolute paths; never `cd` into it.
- **The spec:** u4f2-brief.md rev 1.2 (sections 1-9: what is built, the carries by number in section 3, the tests in
  2.10, the end checks); the advisor's brief check `.checkpoints/netcode-proof-test/advisor-u4f2-brief.md` (its
  conditions and the rulings Q1-Q5, applied in the brief's section 9); u4s-carries.md's U4f2 section (:541-547 and the
  pointer lines after it); the parts of `.checkpoints/netcode-proof-test/u4f-brief.md` the brief cites (build rules
  :831-958, 5.1-5.4, section 6 :1163-1183).
- **The builder's record:** `.checkpoints/netcode-proof-test/u4f2-build.md` (blocks 1-8, the 11 plants and their
  restores, the hand runs, findings F-A to F-E, the departures, the end checks).
- **The evidence** (logs, read only): `/home/user/Shamble-Frontlines/.gate/u4f/u4f2-*.log` (b1-t to b8-k7, plant1 to
  plant11, hand-a, hand-b, hand-long, hand-long2, times-f16, times-rest, doc2, fmt3, fmt4) and the end checks: end-a
  (doc FAIL), end-a2 (deps FAIL), **end-a3 and end-b (all 11 steps pass each; the test step 1,124.2 s and 1,125.1 s
  against the 1,207 s stop line; 702 measured and 799 bites tests, +60 each on a289def's 642 and 739; the wasm step
  27,000 ticks, 0 mismatching)**. Hand-run folders: `.gate/u4f/u4f2-hand`, `u4f2-hand2`, `u4f2-hand-long`,
  `u4f2-hand-long2`.
- **Open from the build** (the builder's words are in u4f2-build.md; weigh each, do not take its reading as proven):
  - **F-B** (the builder: High for -21's set): on one 610 s forged run (seed 1, option A), kind 7 left 18 of 1,523
    items unrejected. The builder's unverified reading: a forged edge sent while the forger is dead gets no sim record
    (world.rs:1353), and a respawn resets the weapon. -21's check "every forged command rejected and counted" would fail.
  - **F-A:** the lag-switch suite's lag client now stands still, so -21's lag check "no tick moves further than the
    catch-up cap allows" cannot fail by movement.
  - **F-C:** shots per measured second 677 and 685 on seed 1, against the 665-735 band.
  - **F-E:** plant 9 (a twin that makes the refused command honest) is not caught with the standing scripts.
  - **Departures from the brief:** f16 shares one 16-client pair per option (11 forgers + 5 honest), for the test-step
    budget; the forgers run on fixed-lead endpoints; C-U4F2-5 took route (c) (route (b) was shown to fail); kind 7 was
    rebuilt so `PM-0001-21/no-cooldown` reaches it (F-D); `CATCHUP_HOLD` was renamed `HOLD_STEPS` to pass
    check.mjs:760's name rule (6 steps at both 30 and 60 Hz), with rustfmt's diff for it applied by Edit.
  - The test step's margin: 82 s under the 1,207 s line (931.9 s at 1faf894, before U4f2).
- **Not done yet (the main session's, after this round):** the advisor's round ruling, the fix pass, the merge into the
  lane, the gate, CI, the advisor's head check, c-u4f-r2.mjs whole (21 rows) and -21's set.

## The four seats (each reviews the whole diff, with its own focus; batch 1: netcode, security, test; batch 2: rust)
- **netcode-architect:** the forged suite against the plan (rev 3.3) and ADR 0001 (each forged command refused by the
  server's own state, counted, attributed); the attribution (C-U4F2-3: `CommandRefusal`, `RefusalRecord { slot, seq,
  kind }`, `event.forged_kind`); options A and B; **F-B's cause and a fix inside the plan** (no pass mark, judge or
  marks.json change); F-A (the lag check's power with a standing client); C-U4F2-5's route (c) against Q1's ruling
  ((b), else (c), never (a)); the fixed-lead endpoints; the `HOLD_STEPS` name against the tick-count rule (carry 39).
- **security-auditor** (hostile input, by the brief's section 7): each forged kind (3, 7, 8, 9, 10, 11) as a hostile
  client's command: refused by the room's own state, nothing trusting the client; the attribution's bounds
  (`refusals_dropped_count`, SEC-1); `--forge-every`'s parser (below 2 a usage error; overflow and odd values); any new
  path where a forged command changes state; the rebuilt kind 7 (fire edges inside the cooldown, reload edges inside
  the reload).
- **test-engineer:** whether the 60 new tests and the 11 plants prove the brief's items (2.10's 51 plus 9 extras, the
  22 negative cases, Q4's f33 case, C-U4F2-10's case); plant 9 (F-E) and plant 1's history; f16's shared pair against
  the brief; the 702/799 counts; F-C's margin; the test step's margin to the 1,207 s line and what the next unit adds;
  c-u4f-r2.mjs's rows and c-u4f-runs.mjs's set 21 against 5.1-5.4 (24 measured, 12 twins, 13 bites, two judge calls;
  NO_STAR; each twin naming the other); p19's `expect`; what stays unproven until -21's set.
- **rust-reviewer:** Rust correctness and safety; identical results on wasm32 and native; the §4.11 bans (no HashMap
  or HashSet, no clock read, the room float-free); integer-only; no panic or overflow path in the forgers and the
  attribution; allocation in hot paths (the room's refusal records per command); clippy; the `HOLD_STEPS` rename (a
  step count, not a tick count?) and the doc-link fix.

## Rules for every seat
- Read and search only: the Read, Grep and Glob tools; from the main folder `git show`, `git log`, `git diff` (the
  main folder's history), `sed -n`, `wc`, `sha256sum`, one command per call in simple forms (no `cd &&`, `;` or pipes).
  Never `cd` into the worktree. Do not run cargo, check.mjs, npt-lab, npt-judge or run scripts, and write nothing
  under spikes/ or .gate/. A claim that needs a run: name the run (command, expected result) for the fix pass.
- Write files with the Write and Edit tools only (no sed, no shell redirect, no python writes; never `cargo fmt` writing
  files: apply rustfmt's diff by Edit); an agent with no Edit tool rewrites its notes file whole with Write.
- Your one file: `.checkpoints/netcode-proof-test/u4f2-review-<seat>.md` (seat: netcode, security, test or rust).
  Write its plan first, then each finished block as you go (a cut may stop you; a restart continues after the last
  complete block).
- Findings on CLAUDE.md's one scale (Critical, High, Medium, Low), each with an id (NC-U4F2-n, SA-U4F2-n, TE-U4F2-n or
  RR-U4F2-n), file:line, what goes wrong, the fix and its rough size in lines. Verdict: APPROVE, APPROVE WITH FINDINGS
  or REQUEST CHANGES.
- Anything that would touch a pass mark, the judge, marks.json, money or main: name it (Mustafa's, under TEST-FIRST).
- Return (your final text, at most 15 lines): the verdict, each finding's id, severity and one line, your file's path.
