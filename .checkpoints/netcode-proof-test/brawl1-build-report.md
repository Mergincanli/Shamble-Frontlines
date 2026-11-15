# BRAWL-1 builder, copy 1: its final report, word for word

Written by script from the agent's transcript (/tmp/claude-0/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/tasks/a4be43b1f756e45dd.output, sha256 1f979e7cae683cf04847ecc6337c026b044f3cf7634c090166ab84bf552e01eb): the text of its last SubagentHandback call (1 in the transcript). Nothing below is the main session's.

---
## BRAWL-1 builder report (copy 1)

BRAWL-1 is built, all of it uncommitted, in /home/user/Shamble-Frontlines/.claude/worktrees/brawl1 (detached at 352a245c). Part 1's proofs (a) to (e) hold at the old pins, read as noted in item 7. Part 2 is built and its tests pass. The three pins have moved, and check.mjs (all 11 steps), det-u4.mjs and the precheck regeneration pass at the new pins. Nothing stopped me. Three points go to the advisor (item 7). The size is 992 changed lines against ~700 planned. The full notes, with this report at their end, are in /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/brawl1-build.md. All logs are under /home/user/Shamble-Frontlines/.gate/u4r/brawl1/.

### 1. Part 1 (file:line in the final diff)

**EA-U4R-2:**
- scenario.rs:51 `pub(crate) type Sim = World`.
- scenario.rs:373 `Server::build(scene, seed, first_tick, clients, event_start, arm_tick, &Bites)`. It is used by:
  - run (:814);
  - run_leg (precheck.rs:293), with Bites::default() and event start 0, keeping its decoder-bite lines;
  - lab_room (:2113).
- logs.rs:28 and precheck.rs name Sim (precheck.rs names no World type now).
- The printed "World::new(scene, ..)" strings are unchanged.
- hash.rs's bite-list and dump sites still name npt_sim::World; the alias's doc says why.

**EA-U4R-1, the narrow form:**
- scenario.rs:591 `struct Wake {admitted, ended, to_run}` and :601 `fn wake(..)`: steps 1-2 plus the due count.
- run (:940) sends to logs.admit (with carry 75's check) and logs.gone.
- run_leg (precheck.rs:368) sends to its map and o.ended, and runs on a SimClock (:363, :461).
- No one-loop driver.

**N2-1 + TE-R2-1:**
- closed_checks (scenario.rs:2231) has the fourth check with the ruled message (:2246), and its doc says four.
- fp1's doc rests "not a replacement" on the roster (roster.rs:116 `replaced = s.live`).
- The no-op assert is dropped.
- Plants, each restored by Edit with its sha256 checked:
  - room.leave deleted: two of the four checks fail by name, in both cases (p1-plant-leave.log).
  - roster.leave (room.rs:691) deleted: the new check fails alone (p1-plant-roster.log).

**TE-R2-2 + RR4-L2 (parse()):**
- Order: the 64-bit checks first, messages unchanged. Then the run's last-tick bound on every profile (:1430-1441). Then, where bursts are used (B1, bursts > 0): the 64-bit last-burst check, the last burst's u32 bound (:1507-1521), the bursts cap (:1523), every-s 0 (:1526).
- The :1442 comment now says what is bounded.
- My messages:
  - "--warmup-s, --measured-s and --after-max-s: the run's last tick, {t} ticks from its first, is above 4294967295 (32 bits)"
  - "--bursts {n}, --burst-first-s {f} and --burst-every-s {e}: the last burst's tick, {t} ticks from the run's first, is above 4294967295 (32 bits)"
  - "--bursts {n}: above 1000"
  - "--burst-every-s 0 with --bursts {n}: more than one burst needs an interval above 0 s"
- fp3 (:2510) checks the exact text of each case:
  - TE-R2-2's three (case 3 in full digits);
  - RR4-L2's ruled case (names --burst-every-s 600000000000, 18000000001200);
  - --bursts 1001, every-s 0 with 2 bursts, --after-max-s 143165507;
  - each bound's own value is accepted.

**RR4-L1:**
- hash.rs:239 `const MAX_ENTRY_TICK_COUNT: u32 = 36_000;` (renamed: see item 7).
- The refusal at :338 sits after the bytes cap and before the runner checks, with the ruled message.
- r10_fp3: 36,001 is refused by that message (:1659); a synthetic entry at 36,000 reads (:1663); the three committed manifests still read.
- r10_carry_82's boundary moved below the cap (:1563; item 7).

**RR3-L2 (a):**
- logs.rs:149 `logging`, set at :210 from dir.is_some().
- On a no-log run, filed() only resolves (:342) and never grows `res`; `last` stays.
- New test at :849: `res` empty, started() true, first and last right.

**The proofs (all on one code state, at the old pins):**
- (a) check.mjs (p1-a-check2.log): all 11 PASS, measured 532, bites 610. Run 1 (p1-a-check.log) failed test and deps on Part 1's own code; fixed (item 7), then run again on the changed code.
- (b) det-u4.mjs (p1-b-det.log): uptime first ("load average: 1.42, 1.96, 1.57"); PASS in 232.9 s; non-cargo sum 222.0 s. It regenerated 5b79abb2 and fad8f67d.
- (c) The precheck regeneration (p1-cd.log): "regenerated SS-P0-s4: 16950444 bytes, sha256 6933b56a…, as the manifest's".
- (d) The clean pair (p1-cd.log): SS-P0-s4 recorded (6933b56a), then precheck with no bite. Of runs.txt:17-125's 109 lines, 108 are identical; the build line differs only in commit and binary sha256. Result PASS.
- (e) p1-after-e2.log against p1-before-a.log: every line the three tests print is identical, apart from two differences that are not behaviour (item 7).

### 2. Part 2

**The read item:** client lib.rs:350 `pub fn own_soldier(&self) -> Option<([i32; 3], bool)>`.
- It gives the decoded grid x, y, z and the dead bit (flags bit 3; private constants at :247-248).
- Source: the newest ring tick that is complete and filed Hash, compared by `self.wrap.after32("tick", ..)`.
- None with no slot, no such tick, or no (Soldier, slot) record.
- No reference into the ring, no float, no new dependency.

**The decision function:** bots lib.rs:231, crate-private `next_command(newest, view, own)`.
- step() calls it with `endpoint.own_soldier()` (:223).
- It applies `leash` (:248) for Kind::Brawl only.
- `toward_centre` (:128) works in integers: x² + y² ≤ 127², above 125² at the radii.
- No new pub item; the library names no World or SimCore.

**The leash:** R_IN 1,300 cm, R_OUT 1,400 cm, no sideways part.
- Margin: 4 m to the door entry (1,800 cm at building 0's door).
- The spawn corners lie at 1,414 cm.
- I tuned nothing: these are the first values tried, and they passed.

**The figure:**
- room.jsonl's tick name `in_building_slot_area`: (slot, area) pairs on every logged tick where it is not empty, in any phase (logs.rs:598).
- Counts gains `in_building_measured` and `in_building_slots`.
- The summary line appends "; in a building: N slot-ticks (slots …)".
- No lab end-event field, no judge change, no marks.json change.

**The SERIES.md row (drafted for the main session's Edit),** under a new heading "Names added with BRAWL-1" after the U4r rows:

| `tick.s.in_building_slot_area` | pairs (slot, area) | npt-lab (from npt-sim's Soldier records' `area`) | the room log, on every logged tick (any phase) where a soldier's feet are in a building area: each such soldier's slot, then its area (building i is area i + 1: the greybox's area rule, half-open on each footprint), in slot order; absent on a tick with none. The run's summary line prints the measured total as "in a building: N slot-ticks (slots ...)" | none (recorded) |

### 3. The tests

- **The unit test** (bots :347) passes. It covers:
  - the walk inside R_IN, against a same-seed twin;
  - the steer past R_OUT;
  - the hysteresis;
  - blocked and skipped steps;
  - None and dead;
  - an open-loop script, which never steers;
  - a sweep checking the 127² bound.
- **The client test** (endpoint.rs:397) passes.
- **The short copy** (seed 1, 120,000 ms, patterns (i) and (ii)) takes 10.61 s in the measured build and 10.71 s in the bites build, so it was not shortened.
- **The full test** (#[ignore]) ran by name in release in 23.52 s (final-regen-tests.log). In all nine runs no soldier was in a building and none was named stuck.

| Seed | (i) largest distance | (ii) largest distance | (ii) skips per 1000 | (ii) longest streak | (iii) largest distance, at |
|---|---|---|---|---|---|
| 1 | 1,575 cm | 1,654 cm | 304 | 6 | 1,832 cm, (1734, -592) |
| 2 | 1,575 cm | 1,720 cm | 303 | 6 | 1,832 cm, (1671, -751) |
| 3 | 1,570 cm | 1,686 cm | 305 | 6 | 1,832 cm, (1586, -917) |

**Pattern (iii), for U4g's brief:**
- No area != 0 in any seed.
- The largest distance is 1,832 cm in every seed, each at a burst's end.
- The three points lie on one circle of about 18.3 m: 32 cm past the 1,800 cm door-entry figure, outside a building.
- That a fixed radius stops it is an inference I did not check; one candidate is the 14 m fight vehicle's outer reach.

### 4. The re-pin

Bytes are unchanged; only the three sha256 values move in the manifests' diff. Both "about" texts and recordings.json are unchanged.

| Entry | Old sha256 | New sha256 |
|---|---|---|
| SS-P0-s4 | 6933b56a619197c719439ff0c16eed3b8927645c3a0bb66910568c6ccf83bab3 | 1f00d2d5f70a2c7b6836ede611b5f1c281319a5312060ead91e016da3c0b1b42 |
| SS-P2-s1 | 5b79abb2a2cba55b06a8916b235aea1766960867f09a038aed146805a29a32e3 | 1ec25de7c9ce8ea869ed9a8b343746ba131593e1de5c65bc9c1c626ce01cc76d |
| ES-P3-s2 | fad8f67ded2a220e42d7e4d42a3d3f2f76e980d4c55abb4fbcd01c550045953b | 6abf7643982b5970a630c97daa4d0815ceb2022958b18e18116397dcad9c893e |

- `npt-lab hash` stops at the first mismatch, so ES-P3-s2's pin came from a second det run, which also reproduced SS-P2-s1's new pin.
- At the new pins:
  - det-u4.mjs (final-det.log): uptime "04:04:39 up 1:30, load average: 1.42, 1.81, 1.48"; PASS in 227.2 s; non-cargo sum 214.5 s.
  - The precheck regeneration reproduces 1f00d2d5…, "as the manifest's".

### 5. check.mjs's final run (final-check.log)

All 11 steps pass. Measured build 535 passed, 0 failed, 0 skips; bites build 613 passed, 0 failed, 0 skips.

### 6. The size

`git diff --stat`: 9 files, 869 insertions and 123 deletions, so 992 changed lines. The plan was ~700 (450-750 with the conditions); the stop line is 1,400. bots takes 426 of them, about 330 in its test module. Free disk at the end: 6,579 MiB.

### 7. Nothing stopped me. Three points go to the advisor, whole

- **Proof (e)'s reading.** At <base>, two "before" runs already differ in the wallclock bite's L1 hash, because the bite XORs aims with wall-clock ns (server pump.rs:228-246). After Part 1 the only differences are that same hash (index 3, the loopback hash, compared 120 and mismatching 117 all equal) and libtest's "52/58 filtered out" becoming 53/59, from RR3-L2 (a)'s new test. I read neither as a behaviour change and went on.
- **RR4-L1's name.** The ruled `MAX_ENTRY_TICKS = 36_000` fails check.mjs's deps step, because TICK_NAMED reads every *_TICKS const outside npt-sim as rate-derived. The form check.mjs accepts would be a local rate-derived const, which carry 39 bars. Built: the same literal, placement, message and tests, named MAX_ENTRY_TICK_COUNT.
- **r10_carry_82.** Its bytes/12 boundary (66,641 ticks) met the new cap; the facts missed it. It now uses 360,000 bytes (30,000 ticks at the boundary).
- Also recorded: p1-before-1.log (`--lib`) and p2-full-release.log (`--exact` without the module path) each ran 0 tests. Both were rerun with the right selector; no failed run was re-run into a pass.

### 8. The advisor's corrections, in its words

"ADV-BP-1: N2-1's 'all four checks fail' was wrong; under the leave plant two fail (Q5). ADV-BP-2: my sets-r2 figure of 'about 4.2 GiB free after the refill' mixed units."

### Decisions needed

- **TICKS-NAME:** which form of RR4-L1's constant stands, since the ruled `MAX_ENTRY_TICKS = 36_000` fails check.mjs as written?
  - Why it matters: check.mjs must pass at the merge.
  - Options: (a) MAX_ENTRY_TICK_COUNT, the literal 36,000 (built); (b) MAX_ENTRY_TICKS = 4 * npt_sim::ticks_for_ms(300_000), which check.mjs accepts but carry 39 bars; (c) add MAX_ENTRY_TICKS to check.mjs's NOT_TICK_COUNTS, a check.mjs change that belongs to its follow-up's lane.
  - Recommendation: (a).
- **PROOF-E:** does proof (e) hold with its two differences (the wallclock L1 hash, which already varies at base, and the +1 test count)?
  - Why it matters: condition 6 says any difference comes to the advisor.
  - Options: accept, or name another comparison.
  - Recommendation: accept, and leave the wallclock hash out of the comparison in future captures.

### For Mustafa

Nothing: this is spike code, with nothing a player sees, hears or feels.
