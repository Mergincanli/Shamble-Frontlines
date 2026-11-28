# Advisor: the plan review of BRAWL-1's lane (workflow wf_533068a9-b2b), 2026-11-14

Written by script, word for word, from the workflow's journal (advisor-brawl1-plan's StructuredOutput result). The plan: brawl1-plan.md. Its notes: advisor-brawl1-plan.md. Nothing below is the main session's.

## Verdict: APPROVE WITH CONDITIONS

## Conditions

1. 1. The leash's bounds (Q1): R_IN at least 1,300 cm. That is above the wall stand-in's contact reach: its north face at 1,220 cm plus the soldier's 40 cm radius gives 1,260 cm (greybox.rs:156; world.rs:75-77). So no fixed box lies in the steering band (the crates reach at most 990 cm). R_OUT at least R_IN + 100 cm and at most 1,500 cm, which keeps 3 m to the nearest door entry: a soldier's centre at 1,800 cm, building 0's door at x 1,800 with |y| < 60 (greybox.rs:83-136). The plan's record replaces '2.6 m from any footprint' with this door-entry margin, and notes that the spawn grid's corners lie at 1,414 cm (world.rs:703-722; scene.rs:153). The sideways part may stay or go. The leash applies to Kind::Brawl only, so the open-loop scripts are unchanged. Every leash vector has x^2 + y^2 <= 127^2 (the unit test asserts it), so the sim never takes its scale path (world.rs:494-498). Tuning stays inside these bounds, and any tuned value is recorded.
2. 2. The stuck print: the sim-only test names any soldier whose position stays inside a 100 cm square for more than 300 commands given, with its slot, tick and position. This replaces 'has not moved', which a soldier rocking by 1 cm at a box face passes. Any soldier the full test names comes to me before the 10-minute runs.
3. 3. The sim-only test (Fix (c)): (i) Both barred patterns feed each slot its position from 8 steps earlier: the largest lead a run takes (--lead's cap), as a step count with no time claim. (ii) The skip pattern's rate and longest streak are stated in the test's doc and printed. (iii) A third pattern is printed only, never barred. It models B1: every slot re-runs its last command for npt_sim::ticks_for_ms(1_000) ticks at ten burst starts, the first at 30 s and then every 60 s (the run scripts' B1 settings). Its result (any area != 0, the largest distance from the centre) goes into the build notes and into U4g's brief. (iv) Every length comes from npt_sim::ticks_for_ms (610,000 ms for the full run, 120,000 ms for the short copy), never a literal tick count (carry 39). (v) The World starts at the judged runs' first tick: World::new(scene, seed, 63_736).
4. 4. npt-bots gains no pub item. The decision function is crate-private and called by step(). The steering unit test and the sim-only test live in a #[cfg(test)] module in npt-bots' src/: npt-sim is already a normal dependency, so Cargo.toml does not change. npt-bots' library code names no World or SimCore; only that test module does.
5. 5. The npt-client read item (Q2): one read-only pub method on ClientEndpoint (&self). It returns, by value, an Option of the own soldier's decoded grid x, y (and z) and its dead bit (flags bit 3), taken from the newest tick that is complete (Tick::complete) and filed Filed::Hash, with ticks compared by the endpoint's wrap rule. No reference into the ring, no float, no new dependency. Its doc says it reads the client's own decoded state. Its npt-client test shows: None before the first complete tick; after a complete tick, Some equal to the own record's x, y, z and dead bit; no change from a newer incomplete, refused or sentinel tick. The aim stays on the mirror view: build ruling 1 point 7 and its U6 follow-up stand.
6. 6. Part 1's proof at the old pins. check.mjs, det-u4.mjs and `npt-lab hash --manifest crates/lab/precheck-recordings.json`, byte-equal at 6933b56a, 5b79abb2 and fad8f67d, prove only scenario::run's half, because they regenerate through `scenario --record`. run_leg's half needs two more proofs. (i) The clean pre-check pair: record SS-P0-s4 at its entry's settings (sha256 6933b56a), then run `npt-lab precheck --manifest crates/lab/precheck-recordings.json --recording <file> --commit <sha>` with no bite on the measured build. Its output lines must equal the '## pair clean' section of results/u4r-precheck-30/faf42aeb034c/runs.txt line for line, except the build line (commit and binary sha256). The crates have not changed since faf42aeb. This costs about 320 s and 17 MB. (ii) The precheck tests' --nocapture output (r6, the late-command bite, wallclock), captured before Part 1 and after, must diff identical. Any difference comes to me whole. Section 6 of the plan says which proof covers which half.
7. 7. The re-pin (Q3) is ruled for exactly three entries: SS-P0-s4 (precheck-recordings.json), and SS-P2-s1 and ES-P3-s2 (det-recordings.json). It comes only after condition 6's results and Part 2's tests pass and are in the build notes. The manifests' diff changes only those three entries' `bytes` and `sha256`; every other field and both 'about' texts stay. Each new pin must be reproduced at the merge step's counted run (condition 9). The build notes and the report give each new pin beside the old one. Any other mismatch, any mismatch at the old pins, and a new pin that is not reproduced each come to me whole. No retry.
8. 8. det-u4.mjs: every call has `uptime` first in the same main-run.sh call and runs while no other lane builds or runs. A non-cargo sum over 240 s stops the step and comes to me whole with the uptime, never retried (INF-R2-2).
9. 9. The plan names the merge step in U4r's M-list form. First, the merged bytes must equal the reviewed bytes (cmp and the tree fingerprint). Then one counted main-run.sh call on the merged tree: uptime; check.mjs with all 11 steps; det-u4.mjs; the precheck manifest regeneration at the new pin (build ruling 1: det-u4 at every merge that touches the scenario path). Then the gate (--no-smoke, with the reason, if Chromium cannot start), CI green, and my review of the pushed head.
10. 10. Fix (b): every name the change writes gets a marks/SERIES.md row marked 'none (recorded)'. That means the tick name, plus any lab end-event field that carries the measured total. The list is written on every logged tick where it is not empty, in any phase. The Counts total counts measured ticks, and the summary line prints it. No judge change and no marks.json change.
11. 11. The reviewers: test-engineer, netcode-architect, rust-reviewer and engine-architect. engine-architect reviews the npt-client pub item and the form of EA-U4R-1 and EA-U4R-2. It is one round on the final diff, run in two batches of at most 3, each seat at effort xhigh (netcode).
12. 12. TE-R2-2's case 3 is written in full digits, because num() parses u64 (scenario.rs:1327-1332) and '3e17' would give a different usage error: --warmup-s 300000000000000000 --measured-s 300000000000000000 --after-max-s 100000000000000000. The case pins '--after-max-s: the run's last tick overflows 64 bits'.
13. 13. Q4: the full test is #[ignore = "<reason>"] (the precedent is net l2 socket_tests.rs:512). It is run by name, in release, at the head that goes to review, and its log is kept under .gate/. The short copy (seed 1, ticks_for_ms(120_000), both barred patterns) runs in every check.mjs. The build notes record the short copy's time in each build and the full test's release time. If the short copy takes more than 60 s in either build, it is shortened and the change recorded.
14. 14. The 10-minute evidence runs: their command lines go into the evidence log, and their set and run names mark them as evidence, never u4r-*. Each run's -17 mismatches and incomplete count are recorded as facts, not as a verdict. If either run misses its bar (everyone_in_view_min_share under 0.99, or a building total above 0), or its bytes exceed its U4r-F3 rate, the re-run sets do not start, and it comes to me whole.
15. 15. The expected lines for the re-run sets, with my words clarified: for -57, a difference in a verdict, or in whether a bite is shown, comes to me whole. Its values are recorded beside U4r's, and the report names the largest relative change. The rest stays as the plan has it: TICK16-WORDING's line, the building figure at 0 on every run, and each set's raw-log bytes against its projection.
16. 16. Disk: the build-phase figure uses the measured full shared target, 8.2 GiB (u4r-disk-plan.md:12), so about 4.7 GiB is free after the refill. The plan's 'about 9 GiB' would give 3.9 GiB, and its '4.2' was my own unit mix-up. Brief rule 6 applies: 4 GB free before any wasm32 or release build, 2 GB before any other build. A floor stop is reported and never cleaned around. The kept-logs draft's ratio is corrected to the measured 3.55 times, about 2.1 GiB of archives for the 7.4 GiB.
17. 17. Q6: the builder's brief comes to me first, for a conformance check only: every section of the plan and every condition here is carried, and nothing new is decided.
18. 18. The report records my two corrections. ADV-BP-1: N2-1's 'all four checks fail' was wrong; under the leave plant two fail (Q5). ADV-BP-2: my sets-r2 figure of 'about 4.2 GiB free after the refill' mixed units.

## The choices (the plan's section 13)

- **Q1, the leash** (a) Toward the cluster centre, with the bounds of condition 1: R_IN at least 1,300 cm; R_OUT from R_IN + 100 cm up to 1,500 cm. The proposed R_IN of 1,200 cm is refused. **Why:** The centre needs nothing from npt-sim, so EA-U4R-4 stays shut. Building entry is only through a door gap, at a soldier's centre at least 1,800 cm out (greybox.rs:83-136), so the margin is 3 m or more. But at R_IN 1,200 cm the wall stand-in (contact reach 1,260 cm) sits inside the steering band. A soldier steering in from the north hits its north face and stays in steer mode. The origin steer's x part points to x = 0, so it slides toward the middle and rocks at about |x| 35 cm, where the 1 cm snap erases the steer (A4). A sideways part used only on an exact axis pushes it out, and the steer pulls it back. With R_IN above 1,260 cm, no fixed box lies in the band, and the crates (990 cm at most) are never met while steering.
- **Q2, the position's source (and whether an npt-client pub read item may come now)** (a) Yes, it may come now, in condition 5's form. Option (b) is barred. **Why:** Verified: the brawl's view is mirror.view (scenario.rs:1004-1008, mirror.rs:227-244), and the mirror's Records are built from the room's own EntityRecords (mirror.rs:110-138). That is server state, which Fix (a) bars. R16 is a facts row ('npt-client exposes no decoded record'), not a rule. Build ruling 1 point 7 named 'an npt-client read item (R16)' as the route once bots need the client's own view, and BRAWL-1 now needs it for movement. It is a new pub item, so engine-architect joins the round (condition 11). npt-bots gains none (condition 4).
- **Q3, the re-pin** (a) Ruled here, for exactly SS-P0-s4, SS-P2-s1 and ES-P3-s2, under conditions 7 to 9. **Why:** Build ruling 1 says a unit on the scenario path re-pins and never retries. INF-U4R-1 says a re-pin needs a ruling. This is that ruling. It is bound to the old-pin proofs coming first, to a manifest diff that touches only bytes and sha256, and to the new pins being reproduced at the merge's counted run. Anything else comes to me whole.
- **Q4, the sim-only test's cost** (a) The full test is #[ignore]d and run by name in release for the record; a short copy runs in every check.mjs (condition 13). **Why:** In the dev profile npt-sim is not optimised (Cargo.toml opts only rand_chacha and ppv-lite86), and check.mjs runs both builds. There is a precedent: net l2 socket_tests.rs:512. An ignored test is not a skip, because cargoCounts reads passed and failed, and 'SKIPPED' is a test's own print (check.mjs:1937). My ruling asks for the full test once, with its log, at the fix's commit.
- **Q5, N2-1's proof under the leave plant** (a) Two of the four checks fail by name, plus the second plant, under which the roster check fails alone. **Why:** Verified at scenario.rs:496-503. With room.leave (:499) deleted, owner.remove (:500) and ended.push (:502) still run, so only the room check and the roster check fail. With roster.leave (room.rs:691) deleted, clients[i] is still set to None, so only the new check fails. My r2 words 'all four' were wrong: correction ADV-BP-1, mine.
- **Q6, the builder's brief** (a) It comes to me first, for a conformance check only (condition 17). **Why:** This ruling adds exact conditions (bounds, proofs, re-pin limits, test design). A brief that drops one would cost runs. The check decides nothing new.
- **Q7, KEPT-LOGS** (a) It goes to Mustafa now. The lane does not wait for his answer. My recommendation to him is option (c), archive then remove, with the old 4 GB build cache named in the same rule. **Why:** The approved cleans hold this lane, but only just: about 4.7 GiB is free in the build phase against the 4 GB rule. U4g, right after, does not fit at all: about 6.6 GiB free after the re-run sets against a refill of about 8.2 to 9 GiB. The rule is a kit change (risk tier 2, its own PR with security-auditor, his go in a day session), so it needs lead time. A narrow kit rule is his own LEFTOVER-DELETE route. I measured 3.55 times compression (one P3 run, zlib-6, in memory), not 5 to 10 times.
- **Q8, Part 1's place** (a) In this lane, as ruled, and proved at the old pins first, with condition 6's added proofs. **Why:** The old pins exist now, and the crates have not changed since faf42aeb, so a byte-level 'before' exists today: the clean pair's runs.txt. Moving the refactors to U4h would only move the same proof and add another lane's overhead. Part 1 is about 190-295 lines with roughly an hour of proof runs. That is acceptable on the critical path.

## Findings

### BP-1 (Medium)

- **Where:** brawl1-plan.md section 6 Part 1 and section 7 step 1; precheck.rs:285-480 (run_leg); hash.rs:413-480 and det-u4.mjs:1-37 (regeneration via scenario::run only)
- **Wrong:** The plan proves EA-U4R-1 and EA-U4R-2 by det-u4.mjs and the precheck regeneration, byte-equal at the old pins. Both regenerate through `scenario --record`, so they never run run_leg. The SimClock change and the shared constructor in run_leg would be proven only by the precheck unit tests, whose outputs are not byte-compared. Half the refactor's claimed evidence does not hold.
- **Fix:** Condition 6: the clean pre-check pair at the old pin, diffed line for line against results/u4r-precheck-30/faf42aeb034c/runs.txt's '## pair clean' section, plus the precheck tests' --nocapture output diffed before and after Part 1. Any difference comes to me whole.

### BP-2 (Medium)

- **Where:** brawl1-plan.md section 2 (R_IN 1,200 cm, the sideways part) and 4.2 (the stuck print); greybox.rs:156 (the wall stand-in); world.rs:1230-1285, :1343-1361
- **Wrong:** The wall stand-in's contact reach (1,260 cm) lies inside the steering band [1,200, ...]. A soldier that steers in from the north is funnelled along the wall's north face to about |x| 35 cm, where the 1 cm snap erases the steer. The sideways part, used only on an exact axis, pushes it out and the steer pulls it back: a rocking pin. The 'has not moved in 300 steps' print cannot see it. The wall falls when shot (500 health), so it likely heals, and no verdict is wrong, but the plan's claim that the sideways part answers A4's pins is not true here.
- **Fix:** Conditions 1 and 2: R_IN at least 1,300 cm, R_OUT from R_IN + 100 up to 1,500 cm; the stuck print by a 100 cm square over 300 commands; any soldier named comes to me.

### BP-3 (Medium)

- **Where:** brawl1-plan.md 4.2 (the sim-only test); l1 lib.rs:197-201 (B1's 1 s bursts); scenario.rs:1829-1836 (P3 lead 7)
- **Wrong:** The test feeds each soldier its current position, while the real loop acts on a position a full lead old (up to 7 to 8 steps, about 1.6 m). Nothing records how the leash behaves under B1's 1 s bursts of total loss: about 30 or more re-run steps, 6 m or more, against a 3-4 m entry margin. U4g's judged B1 runs could repeat BRAWL-1 unseen. The test also names literal tick counts (18,300 and 3,600), which carry 39 bars outside npt-sim.
- **Fix:** Condition 3: an 8-step delayed feed in both barred patterns; the skip pattern's rate and longest streak stated; a printed-only B1 pattern recorded for U4g's brief; lengths from npt_sim::ticks_for_ms; first tick 63,736.

### BP-4 (Medium)

- **Where:** brawl1-plan.md section 7 ('The merge into the spike branch: CI, then your review of the head'); advisor-u4r-build1-full.md:14
- **Wrong:** The plan names no counted run on the merged tree. Build ruling 1 requires det-u4.mjs at every merge that touches the scenario path, and the new pins must be reproduced outside the builder's copy.
- **Fix:** Condition 9: the M-list form (merged bytes equal the reviewed bytes; one counted main-run.sh call with uptime, check.mjs, det-u4.mjs and the precheck regeneration at the new pin; the gate; CI; my review of the pushed head).

### BP-5 (Low)

- **Where:** brawl1-plan.md sections 2, 7 and 11 (the reviewers; pub next_command); advisor-u4r-r1-full.md:40
- **Wrong:** The lane adds a pub item to npt-client and a pub next_command to npt-bots, and carries engine-architect's own EA-U4R-1 and EA-U4R-2. My round-1 rule is that engine-architect rejoins on a new pub item or seam change, but it is not named. A pub next_command also opens a second public route to commands, against EA-U4R-5's single route (U6), when a crate-private function would do.
- **Fix:** Conditions 4 and 11: npt-bots gains no pub item (the tests sit in-crate), and engine-architect joins the one round.

### BP-6 (Low)

- **Where:** brawl1-plan.md section 6, TE-R2-2 case 3; scenario.rs:1327-1332
- **Wrong:** Case 3 is written as '--warmup-s 3e17 ...'. num() parses u64, so that text would hit 'is not a whole number', not the after-max refusal.
- **Fix:** Condition 12: full digits.

### BP-7 (Low)

- **Where:** brawl1-plan.md section 9 ('-57 ... its values recorded beside U4r's (any difference comes to you)'), from advisor-u4r-sets-r2-full.md BRAWL-1 (d)
- **Wrong:** Every -57 value moves with any brawl change, so read literally, every set would stop for me. My words meant the verdicts.
- **Fix:** Condition 15: a verdict difference, or a change in whether a bite is shown, comes to me; values are recorded side by side with the largest change named.

### BP-8 (Low)

- **Where:** brawl1-plan.md section 8 and kept-logs-draft.md section 2 ('about 9 GiB ... about 4.2 GiB'); u4r-disk-plan.md:12
- **Wrong:** 13,225 MiB minus 9 GiB is 3.9 GiB, under the floor, not 4.2. The measured full shared target is 8.2 GiB, which gives about 4.7 GiB. The 4.2 is my own sets-r2 unit mix-up (ADV-BP-2).
- **Fix:** Condition 16.

### BP-9 (Low)

- **Where:** kept-logs-draft.md section 4 (c) ('about 5 to 10 times smaller ... about 1 GiB')
- **Wrong:** Measured in memory with zlib-6 on P3 s2's three logs (181.4 MB to 51.2 MB): 3.55 times. So about 2.1 GiB of archives, not 1 GiB.
- **Fix:** Condition 16; the figure in the question to Mustafa uses 3.55 times.

### BP-10 (Low)

- **Where:** brawl1-plan.md section 3 ('The new name gets a marks/SERIES.md row'); contract.rs:66-110
- **Wrong:** If the measured total is written to a lab end event, that event field is a second written name, and the contract's check (b) needs a row for it too. The plan names one row.
- **Fix:** Condition 10: a row for every written name.

### ADV-BP-1 (Low)

- **Where:** advisor-u4r-r2-full.md N2-1 ('Under the leave plant (room.leave deleted), all four checks fail by name in both cases')
- **Wrong:** My error. owner.remove and ended.push still run under that plant, so only two checks fail.
- **Fix:** Q5 (a); condition 18 records it in the report.

## Size

About 700 lines planned, as the card carries (the plan's 450-700; my conditions add roughly 30-60 lines of test and doc, for about 450-750). 'Well past' under SIZE-CHECK means more than about 1,400 lines; past that the builder stops, and it goes to Mustafa in three lines.

## The KEPT-LOGS question

KEPT-LOGS: Should a narrow kit rule let a session clear out old test-run logs once their results are saved and their rulings closed?
Why it matters: during the next bot-fix build the disk sits right at its 4 GB safety floor, and the unit after it (U4g's test runs) will not fit at all without freeing space.
Options:
(a) Keep everything. The bot fix still fits, just. The next unit cannot start its runs until this is decided anyway.
(b) A narrow rule for one kind of folder only: once a run set's results are committed and its ruling is closed, the session may delete that set's raw-log folder, recording free space before and after. This frees about 7.4 GB now. Those runs could never be re-judged by hand again; their verdicts stay saved.
(c) The same rule, but each set's raw logs are first packed into one compressed file and checked readable before the folder goes. This frees about 5.3 GB now (the archives keep about 2.1 GB: I measured them about 3.5 times smaller), and a re-judge stays possible if a judge fault is ever found.
My recommendation: (c), and let the same rule also name the one old 4 GB build cache that the clean command refuses to touch (it holds no test evidence). With both, the next unit's runs fit with room to spare: about 7 GB free after its build cache refills, against the 4 GB floor. This is the 'narrow kit rule' route your leftover-delete rule names. It is a kit change, so it comes as its own reviewed pull request (advisor plus security review) and merges only on your go. Nothing is deleted before then, and the bot fix does not wait for your answer.

## For Mustafa

The plan for the bot fix is approved with conditions. Bots will steer back toward the middle using the position their own client decoded, never the server's. I checked the plan against the code. I tightened two numbers so no bot gets stuck behind the small wall in the middle, and added a proof that the run-loop clean-up changes nothing. One question for you, below. Nothing to check in a browser.

Decisions needed:
KEPT-LOGS: Should a narrow kit rule let a session clear out old test-run logs once their results are saved and their rulings closed? Why it matters: the disk sits at its safety floor during the bot-fix build, and the next unit's runs will not fit without it. Options: (a) keep everything (the bot fix fits just; the next unit waits for this decision anyway); (b) a narrow rule to delete a closed run set's raw-log folder (frees about 7.4 GB; those runs can never be re-judged by hand); (c) the same, but pack each set into one checked compressed file first (frees about 5.3 GB, keeps about 2.1 GB, re-judging stays possible). Recommendation: (c), with the same rule also naming the one old 4 GB build cache that the clean command refuses (it holds no evidence). It comes as its own reviewed kit pull request and merges only on your go; nothing is deleted before then.

For Mustafa:
1. Answer KEPT-LOGS (a, b or c) when convenient. The bot fix goes ahead meanwhile.
