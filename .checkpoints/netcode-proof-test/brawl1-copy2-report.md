# BRAWL-1 builder, copy 2 (the fix pass of review round 1): its final report, word for word

Written by script from the agent's transcript (/tmp/claude-0/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/tasks/a2e54bc7201dcffed.output, sha256 afe7bca7823ae1133ebbf491067999ee5f6768adc264c646f1f157184db9f08a): the text of its last SubagentHandback call (1 in the transcript). Nothing below is the main session's.

---
## Fix pass, round 1: report (BRAWL-1 builder, copy 2)

The fix pass (the ruling's steps 3 to 11) is done in /home/user/Shamble-Frontlines/.claude/worktrees/brawl1, all uncommitted, every edit by the Edit tool. All four plants bit as ruled and each file came back to its exact sha256. check.mjs passes all 11 steps with 3 more tests per build. det-u4.mjs and the precheck regeneration reproduce all three pins, and every figure the full sim-only test printed in round 1 is unchanged. No stop was met. Size: 1,266 changed lines (stop line 1,400).

The spend limit cut me at 06:53:13 UTC; I resumed at 07:13 with no plant in place and nothing lost. The final tree's `git diff HEAD | sha256sum` is c2561b6ca54c8cde994fcdad5954c63d332df73f2db038c12aba3d71a0daa49f (ee2973aa at the start). The full report is at the end of /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/brawl1-build.md, under "## Fix pass, round 1: report" (line 620). FP-9's words are in the section "FP-9: report words" just above it.

**Start checks and FP-0:** all held (HEAD 352a245c, brief 921c3243, runner 2f754d12, diff ee2973aa, 6,565 MiB free). My first runner log, before any edit (.gate/u4r/brawl1/fp-0-header.log), shows `diff ee2973aa...`. The manifests' sha256 were taken before the first edit.

**Each step** (paths under spikes/netcode-proof/crates/)
- **FP-1** (bots/src/lib.rs, test module):
  - (a) the 48-record assert, :565;
  - (b) the dead-bit tie in soldiers() (:513) with the ruled message; Record.dead (:502), counted at :569; printed; the short copy's `r.dead > 0` (:700) stays there, since seed 1 has deaths in both patterns;
  - (c) the area test, :709;
  - (d) printed() (:661) words the skips by pattern.
- **FP-2** (the unit test, :362): blocked loop :380, kept-state case :413, outer-radius cases :423 and :429, const bounds :338.
- **FP-3** (lab/src/logs.rs):
  - the figure test at :878 (warm-up asserts :912, measured count :924, room.jsonl and both summary-line cases);
  - EA-B1-4's words at :6-7;
  - the figure's own code is unchanged.
- **FP-4** (npt-client): the tick-105 sentinel case (endpoint.rs:438-450); the wrap test (:463); doc words at lib.rs:350-353 and :246-249.
- **FP-5** (bots lib.rs, docs only): :59-62, the leash doc :250-253, toward_centre's doc :130-132.
- **FP-6**:
  - Server::new is now private (scenario.rs:396);
  - Server::build's doc rewrapped (:367-371);
  - comments reworded at :1432-1434 and :2576;
  - the two B1 cases at :2588-2606;
  - hash.rs:1561-1564 uses 359,993 bytes.

**New tests (+3 per build: measured 535 → 538, bites 613 → 616):**
- bots/src/lib.rs `tests::brawl1_a_soldier_teleported_into_building_0_reads_area_1`
- lab/src/logs.rs `logs::tests::brawl1_the_building_figure_counts_a_soldier_in_a_building_in_measured_ticks_only`
- client/tests/endpoint.rs `brawl1_own_soldier_compares_ticks_by_the_endpoint_s_wrap_rule`

**The plants** (logs under .gate/u4r/brawl1/, each run with u4r-run.sh -x -k; each file's sha256 is listed as before / planted / after, and after equals before every time)

| Plant | What was cut | sha256 before / planted / after | Log | Result |
|---|---|---|---|---|
| P1 | `return walk;` as the first line of `fn leash` (bots lib.rs:255) | 83a8541b / 77be9b00 / 83a8541b | fp-plant-1.log | short copy exit 101 in both builds, "Skips: first Some((64395, 11, 1))"; full test exit 101, Skips listed for seeds 1-3, EveryTick for seeds 1-2 |
| P2 | building filter set to `.filter(\|_\| false)` (logs.rs:588) | 83008fce / b902a7ee / 83008fce | fp-plant-2.log | exit 101 at logs.rs:924, "measured: one slot-tick", left 0, right 1 |
| P3 | Hash filter removed (client lib.rs:359) | 2aca00c8 / 7a77c71a / 2aca00c8 | fp-plant-3.log | exit 101, "a complete tick turned sentinel is not read: tick 101 again" |
| P4 | `.reduce` replaced by `.max_by_key(\|t\| t.tick)` (client lib.rs:360) | 2aca00c8 / 6aeeec0a / 2aca00c8 | fp-plant-4.log | exit 101 at the wrap test's last assert, "tick 0 comes after u32::MAX by the endpoint's wrap rule" |

- **P1's per-seed figures** (in a building, first, and the largest distance with slot, tick, x and y) are in the report. Among them: the short copy's Skips run had 7,728 soldier-ticks in buildings, at most 3,813 cm out; the full Skips runs had 68,964, 95,212 and 62,107.
- **The door-entry bar was not added,** because P1's bite rule held on area alone.
- **P2:** the warm-up asserts before :924 check for zero, so they hold under any plant that removes the figure; :924 is the first figure assert that can fail.

**FP-8 results** (each log's header shows diff c2561b6c)
- (1) final-fp-check.log: all 11 steps PASS. "test 715.8 s measured build: 538 tests passed, 0 failed, 0 reported a skip; bites build: 616 tests passed, 0 failed, 0 reported a skip".
- (2) final-fp-det.log: `uptime` first ("load average: 1.26, 1.93, 1.42"), with no other lane building or running.
  - "PASS det-u4 227.3 s"; non-cargo sum 213.3 s (210.0 + 2.4 + 0.9).
  - The regenerated files reproduce SS-P2-s1 1ec25de7c9ce8ea869ed9a8b343746ba131593e1de5c65bc9c1c626ce01cc76d and ES-P3-s2 6abf7643982b5970a630c97daa4d0815ceb2022958b18e18116397dcad9c893e.
- (3) final-fp-regen-tests.log: "regenerated SS-P0-s4: 16950444 bytes, sha256 1f00d2d5f70a2c7b6836ede611b5f1c281319a5312060ead91e016da3c0b1b42, as the manifest's".
- (4) Same log:
  - The short copy took 10.94 s (measured build) and 10.79 s (bites build). Dead figures: EveryTick 1,190, Skips 1,334.
  - The full test passed in release in 23.84 s.
  - All 13 printed lines are identical to round 1's (final-regen-tests.log:41-42, :62-63, :83-91), once the new dead figure is set aside and the old skip text is mapped to the new wording.
- (5) Both manifests are unchanged: det a2faf3ea81623c39a13b04f9a97cadb900d8ec482df4375173720f70dae95d99, precheck 5ccf57f059026e072cb000c0f9e66ca41ec81b9a02089fd6c26c699c21416881.
- (6) Numstat against 352a245, SERIES.md included: 1,139 added, 127 deleted, 1,266 changed (998 at the snapshot).

**FP-9, items 1-6** are written in the notes:
1. The plant records, and the bar not added.
2. Pattern (iii): I withdraw "a fixed radius stops it" and the 14 m vehicle (stand-ins are off at bots lib.rs:541; no solid lies within 1.6 m of the three maxima).
3. TE-B1-8 in the ruling's words (its glue line is now lib.rs:228).
4. ADV-BP-1 to ADV-BP-4 in the advisor's words, with Point 2's masking rule.
5. MAX_ENTRY_TICK_COUNT stands.
6. The four follow-ups with their homes.

**Where I departed from the ruled words, or found them slightly off** (none is a stop)
- **The figure test's no-log case** (FP-3 step 7): the ruled `settings(&dir, "--profile P0 --seed 1 --logs 0")` cannot run, because the parser refuses `--logs 0` without `--record`. The test adds `--record <scratch>/brawl1.rec`; the parser only stores that path and nothing writes it.
- **A line reference:** soldiers()'s ruled sentence now names "client lib.rs:250-251" instead of "247-248". FP-4 (d)'s added doc lines moved the constants there.
- **Logs::open's argument:** the test passes `Some(dir.as_path())` instead of the ruling's `Some(&dir)`. It is the same path.
- **"Below 2^31" is off by one point:** (-32,768, -32,768) gives exactly 2^31. Nothing can overflow either way, so I wrote the ruled words.
- **The ruling file's length:** its sha256 is right, but it has 460 lines, not the 249 the resume part says. The "brief's section 4" the resume part cites is "What you hand over"; the rules are in sections 1, 2 and 5. I followed the ruling's own list.
- **Small additions beyond the ruling:** a few doc clauses, a rewrapped module paragraph in logs.rs, a two-line comment above the RR-B1-4 cases, and two "tick filed" asserts in the wrap test.

**Left undone:** nothing in steps 3 to 11. I committed and pushed nothing.

Decisions needed:
- LOGS0-RECORD: should the figure test's no-log case keep the added `--record <scratch path>`? It matters because the ruled line cannot run as written: the parser refuses `--logs 0` without `--record`. Options: (a) keep the added `--record`, as built; (b) the advisor names another form. Recommendation: (a).

For Mustafa: nothing. This is spike code; nothing a player sees, hears or feels changes.
