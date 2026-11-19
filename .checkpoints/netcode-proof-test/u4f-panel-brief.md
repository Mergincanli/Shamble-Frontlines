# U4f's review panel, round 1 (2026-11-19): the brief for each seat

You are one seat of U4f's review panel. U4f is the clock-and-commands unit of the netcode proof test (PR #40's spike,
`spikes/netcode-proof/`). This is round 1 of at most two (CLAUDE.md, "Review budget": a third only for an open Critical
or High). Your job: say, with evidence, whether U4f as built is right and proven. The advisor rules on your findings
and a builder fixes them; you change nothing.

## 1. What you review
- **The tree:** builder 3's worktree `/home/user/Shamble-Frontlines/.claude/worktrees/agent-a0825a5a2491eb175` (HEAD
  07289a4; U4f's change uncommitted in it). The change is `git diff HEAD` there plus 9 new files (`git status --short`),
  byte for byte the snapshot `/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/wip/u4f-build.patch`
  (sha256 74ae26aa29c229d6…; 29 files, +6,213 / -307; `git apply --numstat` on it lists them). Its runner hashes (diff
  54cb6faa…, untracked e8dc3663…) equal both end checks' (re-checked 13:44 UTC: .gate/u4f/review-pre.log). The snapshot
  applies unchanged to the lane's head: main's take-in touched none of its files. **Read only: change no file in that
  worktree, the main folder or anywhere else, except your notes file and your runner logs.**
- **U4f's scope after Mustafa's SIZE-U4F (a)** (his words: history.md:6896-6903; read them there). U4f closes with what
  is built. In scope: -47's client clock (brief 2.1), the tick-lead controller (2.2), command timing under both options
  (2.3; -20), every room-side command check with its -21 counter (2.4, 2.5), the never-acker and the event cap of 1,600
  (2.6, 2.8; -22), the client carries 80 (2.9), three of -21's four cheat suites whole with their tests (clamp, lag
  switch, fast clock: 2.6, 2.7), and npt-bots' forgers of kinds 1, 2, 4, 5 and 6 with f15's five cases (built ahead for
  the next unit). **Not in U4f** (the next unit, U4f2, with its own review): the list at advisor-u4f-stop.md:80-89 (the
  forgers of kinds 3 and 7-11, `--suite forged`, the attribution, the twins, f16, f22, f23, f33, p1, p7, p14, p17,
  -21's set, f28's clamp and lag-switch B halves). Do not report those as missing. Do report anything built that is
  wrong without them, and any claim, log or script that would let -21 be judged before U4f2.
- **What it must meet:** `.checkpoints/netcode-proof-test/u4f-brief.md` (2,040 lines; read the sections your seat
  needs: 2.1-2.13 what to build, 2.10 the tests and negative cases, 2.11 the bites, 2.13 the public items, 4 the build
  rules, 5.3 the pre-runs, 5.6 the bars, 6 the end checks); the advisor's rulings on the brief (advisor-u4f-brief.md:
  rulings :173-198, conditions L1-L6 :369-419) and on the fold (advisor-u4f-fold-check.md: D3-1, the `reorder` event);
  the build-1 ruling (advisor-u4f-build1.md, 283 lines: R1-R4, F1-F11, the 22 lines :162-246); the stop ruling
  (advisor-u4f-stop.md, 247 lines: R2 the cap 1,600; R3 (i) the fast clock sends no Ping). Its findings S1-S7 and note
  N1 (:198-210) are known and ruled: do not report them again, but say so if you disagree with one.
- **Frozen by his words** (LEAD-BOUND: history.md:6791-6806; TEST-FIRST: :2234-2282): marks.json, every judge file,
  every existing SERIES.md row, and the lead bound of 400 ms (u4f-lead-bound.md). The patch holds no marks.json or judge
  file and adds 20 SERIES.md lines, removing none. A change to any of them, or a built part that needs one changed to
  pass, is a High.
- **Evidence so far:** end checks A2 and B (.gate/u4f/w3-end-check-a2.log, w3-end-check-b.log: all 11 check.mjs steps
  pass on this tree; tests 630 measured, 727 bites build); the hand runs (.gate/u4f/w2-hand.log; w3-hand.log: p3, p9,
  p16 and p20 each exit 0, contract PASS); the cap's figures (.gate/u4f/w2-cap/cap.txt); the builders' notes
  (u4f-build.md: builder 1 :1-309, builder 2 :310-690, builder 3 :692-866, its stop record :827-866).

## 2. Your seat (brief 12 item 8, u4f-brief.md:1997-2002; your agent file for the rest)
- **netcode-architect (seat `netcode`):** the clock and the lead controller against ADR 0001 and plan §4.8; both
  command-timing options; the lead bound counted from the tick the room runs next (advisor-u4f-build1.md R2 (g)); the
  never-acker, the cap and carry 76's Kick path; the room's per-tick order. Also N1 (advisor-u4f-stop.md:208): whose
  are hand-s22's 30 refused rewinds, and why (w2-hand.log:32, :53), if the logs and the code settle it.
- **security-auditor (seat `security`):** the hostile input: 2.4's kinds and their order, the clamp points, the 1.5x
  clock, the lag switch, the never-acker and carry 76's Kick path, 2.5's counters: every value a client sends that the
  room trusts. S1 (the fast client sends no Ping) answers your SA3-7: confirm or object.
- **test-engineer (seat `test`):** is it proven. U4f's share of 2.10's minimums (A2 and B: +78 measured and +95 bites,
  against +119, +134 and 67 negative cases for U4f and U4f2 together; count U4f's negative cases); each plant and bite
  (2.11; marks.json's bite ids against the code's BITES lists, NA-11); every test 2.10 names for a built part. The read
  of SERIES.md's 20 new lines (the brief's recommended read): give it in a line of its own. His 13:35 rule (a
  reviewer's read of every marks.json or judge change) has nothing to read here: confirm the patch changes neither.
- **rust-reviewer (seat `rust`):** Rust safety and speed in the new code: panics or overflow on hostile values
  (stamps, ticks, sequence numbers, counts), allocation in the room's per-tick path, identical results on Wasm and
  native (check.mjs's wasm step), `unsafe` (none expected), error handling.
- **engine-architect (seat `arch`):** the public items and the seam rule (2.13; A19: plan.md §7.2 :930-931): every new
  or widened `pub` item, the crate edges and the dependency rules.

## 3. How to work
- **Notes first:** `/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4f-review-<seat>.md`. Write your
  plan there first (5-10 lines: what you will read, in what order), then one line per finding as you find it, then
  the verdict and the findings table at the end. Write files with the Write and Edit tools only (no sed, no shell
  redirect, no python writes; never `cargo fmt` writing files: apply rustfmt's diff by Edit); an agent with no Edit
  tool rewrites its notes file whole with Write. If you were cut off, your notes file exists: continue after its last
  complete line.
- **Runs, only to prove a finding:** existing tests or lab commands, through the runner, one command per call:
  `bash /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4f-run.sh [-x] <worktree> /home/user/Shamble-Frontlines/.gate/u4f/review-<seat>/<name>.log <command>`
  (`-x` for a bite run, a `--list` or clippy; otherwise shared; the log header shows the tree hashes). Never cargo
  outside the runner, never a full check.mjs (the end checks ran it twice), never a plant or any edit in the worktree:
  describe the plant in your finding and the fix pass proves it. The runner stops under 4 GiB free (about 5 GiB now):
  keep runs few and short.
- **Severity:** CLAUDE.md's one scale (Critical, High, Medium, Low; major = High, minor = Medium, nit = Low).
- **Each finding:** an id (NC-U4F-n, SEC-U4F-n, TE-U4F-n, RR-U4F-n or EA-U4F-n), its severity, where (file:line in the
  worktree), what goes wrong (concrete inputs to a wrong result), the cheapest proof (a test, a run, a grep), the fix.
- **Return:** your verdict (APPROVE, APPROVE WITH FINDINGS or REQUEST CHANGES), the findings, the lines this brief asks
  for in a line of their own, what you ran, and what you did not check.
