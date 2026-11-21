# Advisor: the ruling on BRAWL-1's builder points (workflow wf_bbfeece0-4c7), 2026-11-15

Written by script, word for word, from the workflow's journal (advisor-points' StructuredOutput result). The points: brawl1-points.md. Its notes: advisor-brawl1-points.md. Nothing below is the main session's.

**Snapshot:** wip/brawl1.patch sha256 ee2973aa8ad8c453b5720099c27b37cb9980820da74f33ad9c78ddd7ad591dc7. I checked that it equals `git diff` of the read copy (/home/user/Shamble-Frontlines/.claude/worktrees/brawl1, HEAD 352a245c, the change uncommitted), in both plain and --binary form. numstat: 10 files, 875 insertions and 123 deletions, 998 changed lines. I ran no cargo or check.mjs command. Every judgement below rests on the builder's logs under .gate/u4r/brawl1/ and on the source in the read copy. I diffed and grepped those myself. My notes: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/advisor-brawl1-points.md.

## Summary

I ruled on all three points and on the recorded items. None of them changes the diff under review. Point 1 (TICKS-NAME): the constant stays as built, MAX_ENTRY_TICK_COUNT = 36,000. RR4-L1's ruled name was my error, because check.mjs refuses it. Point 2 (PROOF-E): proof (e) holds. Its two differences are the wallclock bite's L1 hash, which already varies between two runs at <base> by the bite's design, and libtest's filtered-out count, which is +1 for RR3-L2 (a)'s new test. Condition 6 (ii)'s 'identical' was my wording, and it could not hold at <base>. Point 3: I accept r10_carry_82's move to a boundary below the cap. Recorded items: I accept the zero-test reruns, the check.mjs fix and rerun, and the size (998 lines, under the 1,400 'well past' line). I also re-checked proof (d): 109 lines against 109, with only the build line's commit and binary sha256 differing. One correction to the record matters for U4g. Pattern (iii)'s 1,832 cm is not a fixed face or a vehicle's reach. The three maxima lie at three different angles in one 1 cm band, which fits the leash's own reach under a 30-tick re-run. That reach is past the 1,800 cm door entry, so 'area 0 in three seeds' does not bound B1. Further findings: one Medium (the pattern (iii) reading, for U4g's brief) and five Lows. Nothing for Mustafa.

## Rulings

### Point 1 (TICKS-NAME): the form of RR4-L1's constant

- **Ruling:** Option (a) stands: hash.rs:239 `const MAX_ENTRY_TICK_COUNT: u32 = 36_000;`, with its doc at :236-238. RR4-L1's ruled name `MAX_ENTRY_TICKS` is amended to MAX_ENTRY_TICK_COUNT. That name was my error, recorded as correction ADV-BP-4. Options (b) and (c) are refused, and so is carry 39's route (d), the constant put in npt-sim.
- **Why:** Facts checked. RR4-L1 (advisor-u4r-r2-full.md:92-97) rules the name MAX_ENTRY_TICKS. check.mjs:760 `TICK_NAMED = /_TICKS$|CATCHUP/` makes any matching const outside npt-sim and not in NOT_TICK_COUNTS (:761-765) a tick count, and :839-846 refuses it as a literal. p1-a-check.log:50 shows that refusal for hash.rs:238. As built, the refusal at :338-343 prints the constant, and its text equals the ruled message. r10_fp3 asserts that text exactly (:1655-1662) and reads a synthetic entry at 36,000 (:1663-1666). Final check: final-check.log, all 11 steps PASS.

Why (a) wins: the cap is a resource bound per tick, not a time. RR4-L1's own reason is encode's reserve of ticks x 1,900 B (replay.rs:296) and a runner entry's work per tick. The runner check already scales the entries with the rate (hash.rs:358-361, `x npt_sim::TICK_HZ`). At 60 Hz today's 300 s entries would be 18,000 ticks, still under 36,000, and the cap should stay 36,000. Option (b), `4 * ticks_for_ms(300_000)`, makes the bound follow the rate (72,000 at 60 Hz) for no reason, and carry 39 bars it (u4s-carries.md:169-172). Route (d), carry 18's (u4s-carries.md:129-132), has the same wrong meaning. It would also put npt-lab's manifest policy into npt-sim, against layering, and change npt-sim's ticks.rs, s1's DERIVED table and check.mjs's TICK_DERIVED. Option (c) changes check.mjs. By round 2's step 18 (advisor-u4r-r2-full.md:24), that pulls the whole check.mjs follow-up (SEC-U4R-R2-1/-2, TE-R2-3, INF-R2-5, INF-U4R-10 (2)) into this lane, which is already 998 lines and has no security-auditor seat.

Precedent: NC-U4R-7 (advisor-u4r-r1-full.md:90-92) kept TURN_EVERY and RETARGET_EVERY as literal counts because they claim no time. The doc at :236-238 claims no time and says openly why the name avoids `*_TICKS`. check.mjs's own doc (:794 comment block, 'Known limit') leaves a count under another name to review, and this is that review. No other const in crates/ ends in _TICK_COUNT (grep). The points file's range check.mjs:785-806 is slightly off: the rule's sentence is at :784-788, the code at :839-846. Not material.
- **Changes the diff under review:** no
- **The change:** None. The report records ADV-BP-4: 'RR4-L1's ruled name MAX_ENTRY_TICKS falls under check.mjs's TICK_NAMED rule and is refused as a literal; I did not check it; the name is MAX_ENTRY_TICK_COUNT.' A Low follow-up goes to the check.mjs follow-up: further finding ADV-PT-L3.

### Point 2 (PROOF-E): proof (e)'s two differences under condition 6 (ii)

- **Ruling:** I accept proof (e) with exactly these two differences. Together with proof (d), it proves run_leg's half of Part 1 at the old pins. The diff stays as it is. Condition 6 (ii)'s 'must diff identical' was my wording, and it could not hold at <base> itself: correction ADV-BP-3. Future captures of these tests mask exactly the L1 leg's hash values in the wallclock bite's comparer and result lines ('theirs ... hash' and the L1 hex). They keep the index, the loopback hash, compared and mismatching, and they explain libtest's 'filtered out' count by the binary's test count. Nothing else is masked.
- **Why:** I diffed the logs myself with the normaliser at brawl1-build.md:52, which leaves out runner, Compiling, Finished, Running and Blocking lines and libtest's seconds. Each log has 67 lines after normalising.
- A against B, both at <base>: only lines 46-47 differ. The L1 hash is 9375924675163200265 / 821df9d0f7ec1b09 against 1388086141469307493 / 134378fdc3ebb265.
- A against e2: the same two lines (now 5648772452115156074 / 4e64789de1560c6a), plus line 20 (52 to 53 filtered out) and line 66 (58 to 59).
- In all four logs, index 3, loopback hash 6328158668484839173 (57d222c7ed590705), compared 120 and mismatching 117 are equal. Every line of r6 and of the late-command bite is identical, including the deterministic bite mismatch at index 60 with both hashes. All four logs end with exit 0.

Why the hash varies: server pump.rs:236-243 (bites builds only) XORs each command's aim[0] with 1 | (SystemTime ns & 0xfe), so the L1 leg's World hash is wall-clock dependent by design.

Why the count moves: the patch adds exactly one #[test] in crates/lab, logs.rs's rr3_l2_a test (patch line 884). That matches +1 in both builds.

Proof (d), re-checked myself: p1-cd.log:38-150 against runs.txt:17-125, 109 lines each. Only the build line differs, and only in commit and binary sha256.

One process note: 'comes to me whole' did not say 'stop', so the builder went on to the re-pin, which condition 7 places after condition 6's results. This ruling cures that, and nothing needs redoing. The wording was mine, so it is part of ADV-BP-3.
- **Changes the diff under review:** no
- **The change:** None. The report records ADV-BP-3: 'Condition 6 (ii)'s identical-diff test could not hold at <base>, because the wallclock bite's L1 hash varies with wall-clock ns (pump.rs:236-243); and 'comes to me whole' did not say stop.'

### Point 3: r10_carry_82's boundary

- **Ruling:** I accept the change as built: the entry's bytes are rewritten to 360,000, so the boundary is 30,000 ticks (accepted) and 30,001 is refused with 'cannot fit'. That is below the 36,000 cap. The test keeps its purpose, the bytes/12 rule on both sides of its boundary, and the cap's own boundary is tested in r10_fp3.
- **Why:** Base: `at = 799_693 / 12` gives 66,641, and the test asserted it reads OK. With the cap at hash.rs:338, checked after bytes/12 at :325, 66,641 is now refused. p1-a-check.log:47 names exactly this failing test. The at+1 case was still caught by 'cannot fit' first, because bytes/12 is checked before the cap.

As built (hash.rs:1560-1568): the `.replace` of '"bytes": 799693' checks itself. If it ever matched nothing, 30,001 ticks would read OK against 799,693 bytes, and expect_err would fail. The cap's own boundary sits in r10_fp3 at :1655-1666: 36,001 is refused with the exact text, and a synthetic entry at 36,000 reads.

The facts text at brawl1-facts-full.md:113-116 named only r10_fp3's numbers and the synthetic 36,000 entry, so it missed this test. check.mjs caught the miss, as it should.

One small loss of strength (Low, ADV-PT-L2): 360,000 divides by 12 exactly, so a ceil plant would now pass, where base's 799,693 caught it.
- **Changes the diff under review:** no
- **The change:** None now. If a push is needed anyway, use 359,993 bytes: 29,999 at the boundary, 30,000 refused with 'cannot fit' (ADV-PT-L2).

### Recorded items: the zero-test runs, proof (a)'s failed first run, the size, pattern (iii)

- **Ruling:** 1. Zero-test runs: accepted. They are selector mistakes, and no failed run was rerun into a pass. 2. p1-a-check then p1-a-check2: accepted. The second run follows a fix of Part 1's own code against its checks. It is not a retry of a result under a no-retry rule. 3. Size: 998 lines is past the plan (about 700; 450-750) but under the 1,400 'well past' line, so there is no stop and no SIZE-CHECK note to Mustafa. It goes on the card. 4. Pattern (iii): recorded as printed only (condition 3 (iii)), with no diff change. The record of what the 1,832 cm means needs correcting for U4g's brief (further finding ADV-PT-M1).
- **Why:** 1. p1-before-1.log ran '--lib' twice: 'running 0 tests', 5 filtered out. p2-full-release.log ran '--exact' without 'tests::': 0 ran, 3 filtered out. The reruns ran the intended tests: p1-before-a, 1 then 3 passed; p2-full-release-2, 1 passed in 23.51 s.
2. p1-a-check.log failed the test step (r10_carry_82) and the deps step (the TICKS name). p1-a-check2.log passed all 11 steps (532/610). The log timestamps are in order: check2 03:08-03:20, then det, then cd, then e2 03:33-03:35, and Part 2 from 03:39. final-check.log passed all 11 steps (535/613).
3. The size is the numstat above; the 1,400 line is my plan review's Size section.
4. final-regen-tests.log prints the Bursts maxima at (1734, -592), (1671, -751) and (1586, -917). These are three different angles, all with isqrt 1832. That is a radius, not a face. My reading, computed and not run: the walk turns 22.5 degrees every 4 commands at 20 cm a command (bots lib.rs:30-52; 600 cm/s at 30 Hz), a circle of radius about 205 cm. A circle kept inside R_OUT (centre about 1,195 cm out) that re-runs one walk command for 30 ticks (600 cm) ends at most about 1,195 + sqrt(205^2 + 600^2), about 1,829 cm out. That matches.

Aside: a Bursts line prints 'skipped 14400 of 0 chances (0 per 1000)', because chances are counted only under Skips (lib.rs:555-560, :610-627). That is ADV-PT-L1.
- **Changes the diff under review:** no
- **The change:** None to the diff. The main session carries ADV-PT-M1's reading into U4g's brief, beside the builder's, labelled as computed.

## Further findings

### ADV-PT-M1 (Medium)

- **Where:** brawl1-build-report.md:117-120 and brawl1-build.md:193-196, :338-342 (pattern (iii), for U4g's brief); bots lib.rs:30-66 (MOVES, TURN_EVERY, LEASH_OUT_CM/LEASH_IN_CM); .gate/u4r/brawl1/final-regen-tests.log (the Bursts lines); greybox.rs:64-70, :92-133 (ROOM 400, DOOR_HALF 100: door entry at 1,800 cm on six axes)
- **Wrong:** The record suggests that 'a fixed radius stops it', with the 14 m vehicle's reach as one candidate, and it reads 'no area != 0 in three seeds' as reassurance. The positions do not fit a stop. The three maxima lie at three different angles in one 1 cm band, which fits the leash's own reach under a 30-tick re-run (about R_OUT - 205 + sqrt(205^2 + 600^2), about 1,829 cm; my computation, not a run). Inside condition 1's bounds (R_OUT at least 1,400) that reach passes the 1,800 cm door-entry radius at every angle, door axes included. The real B1 loop also acts on a stale view after each burst. So a B1 judged run can carry a soldier into a door gap, and area 0 in three seeds is not a bound. If U4g's brief carried the builder's inference, its B1 runs could repeat BRAWL-1's INVALID without anyone expecting it.
- **Fix:** No change to BRAWL-1's diff: B1 is printed only, by my condition 3 (iii). The main session puts this reading in U4g's brief, labelled as computed, beside the builder's figures and positions. U4g's plan then says how a B1 building entry reads in -17, or measures how often it happens, before its judged B1 runs.

### ADV-PT-L1 (Low)

- **Where:** spikes/netcode-proof/crates/bots/src/lib.rs:555-560 (chances counted only under Pattern::Skips) and :610-627 (printed)
- **Wrong:** The printed-only Bursts line reads 'skipped 14400 of 0 chances (0 per 1000)'. The rate and the chances are meaningless for that pattern, and the line goes into U4g's brief.
- **Fix:** Print chances and the rate only for Skips; for Bursts, print 'skipped 14400 (10 bursts x 30 ticks x 48 slots)'. This rides a push that is needed anyway; otherwise it becomes a STATUS follow-up.

### ADV-PT-L2 (Low)

- **Where:** spikes/netcode-proof/crates/lab/src/hash.rs:1560-1568 (r10_carry_82, 360,000 bytes)
- **Wrong:** 360,000 divides by 12 exactly, so the test no longer checks that bytes/12 rounds down: a `div_ceil(12)` plant now passes. Base's 799,693 caught it.
- **Fix:** Use 359,993 bytes (`at` = 29,999; 30,000 refused with 'cannot fit'; still under the 36,000 cap) and update the comment. This rides a push that is needed anyway; otherwise it becomes a STATUS follow-up.

### ADV-PT-L3 (Low)

- **Where:** spikes/netcode-proof/check.mjs:755-765 (TICK_NAMED, NOT_TICK_COUNTS); hash.rs:239
- **Wrong:** After ruling 1, MAX_ENTRY_TICK_COUNT is outside the tick-name rule only because of its name. The check's own register of exceptions (NOT_TICK_COUNTS, 'each with its reason') does not show it. A later duration could take the same `_TICK_COUNT` name as a literal and pass unseen.
- **Fix:** Add this to the check.mjs follow-up (check.mjs's next change; before PR #40 is marked ready at the latest). TICK_NAMED becomes /_TICKS$|_TICK_COUNT$|CATCHUP/, and NOT_TICK_COUNTS gains MAX_ENTRY_TICK_COUNT with the reason: 'a manifest's size bound: encode reserves per tick (replay.rs:296), so it does not follow the rate'. Add one check.test.mjs case: a literal `FOO_TICK_COUNT` is refused, and the registered name passes. Not in this lane.

### ADV-BP-3 (Low)

- **Where:** advisor-brawl1-plan-full.md condition 6 (ii) (mine)
- **Wrong:** Condition 6 (ii) required the before and after outputs to diff identical. The wallclock bite's L1 hash varies between two runs at <base> by design (pump.rs:236-243), so the condition could not hold as written. Its 'comes to me whole' also did not say 'stop'.
- **Fix:** The report records ADV-BP-3 beside ADV-BP-1 and ADV-BP-2, in my words, with the masking rule from ruling 2. Nothing to redo.

### ADV-BP-4 (Low)

- **Where:** advisor-u4r-r2-full.md:94 (RR4-L1, mine)
- **Wrong:** RR4-L1 ruled `const MAX_ENTRY_TICKS: u32 = 36_000;`. check.mjs:760 reads that name as a tick count, and :839-846 refuses it as a literal (p1-a-check.log:50). I did not check it.
- **Fix:** The report records ADV-BP-4. The name is MAX_ENTRY_TICK_COUNT (ruling 1).

## For Mustafa

Decisions needed: none. The three points are technical choices inside the approved plan, and all three are mine to rule. For Mustafa: nothing. This is spike code, with nothing a player sees, hears or feels. The size, 998 lines against about 700 planned, is under the 1,400 'well past' line, so no size note is due. KEPT-LOGS stays with you as raised at the plan review. Nothing here changes it.
