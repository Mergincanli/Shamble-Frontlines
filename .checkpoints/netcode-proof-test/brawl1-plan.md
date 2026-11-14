# BRAWL-1, the next npt-lab lane: the plan (for the advisor's review)

Written by the main session on 2026-11-14 at about 23:45 UTC. It answers the advisor's P0 ruling
(advisor-u4r-sets-r2-full.md), finding BRAWL-1 (Fix (a) to (d)) and next steps 4 to 6, with the items the r2 ruling
homed in this lane (advisor-u4r-r2-full.md, item 18: N2-1 with TE-R2-1, TE-R2-2, RR4-L2, RR4-L1; round 1's EA-U4R-1,
EA-U4R-2 and RR3-L2 (a)). The code facts are in brawl1-facts-full.md: two readers' returns, written word for word, with
file:line evidence; their notes are brawl1-facts-a.md and -b.md. No edit comes before your review. U4r-F3, the runner
change, is a separate change-set; it lands before this lane's re-run sets.

## 1. Goal

- **The cause, as you ruled it:** the brawl's walk is open loop. A re-run command, or a step that collide() cuts short,
  leaves an offset that nothing corrects. Soldiers then drift into buildings at every profile, and while a door is shut
  the room rightly drops them. That voids -17's L1 parts and fails -2's SS part.
- **The fix:** each brawler steers by its own soldier's decoded position.
- **The proof:**
  - a sim-only test with no soldier in a building area on any tick;
  - two 10-minute runs with everyone in view at or above 0.99;
  - then the re-run sets P3, P2 and P0 (with its six bites) at the fix's commit, before U4g's judged B1 runs.
- -2's SS part passing everywhere is the fix's proof. -17's parts and bites count from that commit.

## 2. The steering (Fix (a))

**Where the position comes from: the client's decoded snapshot.** The brawl's `view` today is the lab mirror's frame,
which is built from the room's own records (mirror.rs:107-150; brawl1-facts A2). That is server state, so it is
barred. The client's decoded state is private (R16: npt-client exposes no decoded state). So:

- **npt-client gains one pub read item.** It gives the own soldier's decoded x, y and z (1 cm grid integers) and its
  dead flag (flags bit 3), from the newest complete, filed tick, as an `Option`.
  - It returns None before the first complete tick, and when the newest complete tick has no own section.
  - It never reads a tick whose header moved on without a complete frame (client/src/lib.rs:147-164, :682-760).
  - This is the route your U4r build ruling 1, point 7 names for U6 (an npt-client pub read item). Q2 asks whether it
    may come now.
- **npt-bots:** step() already holds `&mut ClientEndpoint`. It reads the item and passes the result to one decision
  function, a new pub `next_command`. step() and the sim-only test both call it.
  - Behaviour::command stays private behind it.
  - npt-bots names no World or SimCore: its use of npt-sim stays Command and Scene.

**The rule: a leash toward the cluster centre, in cm, with no rate-derived constant (FP-7 item 1; carry 39).**

| Point | Value | Where |
|---|---|---|
| Team homes | 10 m from the centre | scene.rs:143-154 |
| Crates | 6 and 9 m | greybox.rs |
| Wall stand-in | y 11.8-12.2 m, x -3..3 m | greybox.rs |
| Fight vehicles' bands | about 1.3-17.7 m | world.rs:772-810 |
| Nearest building footprint corner | 16.6 m (buildings 1, 2, 4 and 5); 18 m (buildings 0 and 3) | greybox.rs:83-163 |

- **Proposed:**
  - **R_OUT = 1,400 cm.** Past it, the brawler's walk vector becomes a vector toward the origin, scaled to full
    command length.
  - **R_IN = 1,200 cm.** It walks the open loop again once back inside. The hysteresis stops it flipping each step.
  - **A sideways part.** When the steer is straight along one axis, a deterministic sideways part, never 0 after the
    1 cm snap (|m| > 3 of 127), lets it slide off a box face. That is A4's pin: a goal straight behind a crate or the
    wall stand-in.
  - **None** (no complete tick yet) keeps today's open-loop walk. **Dead** (bit 3) keeps it too: a respawn puts the
    soldier at home.
  - The aim and fire stay as today.
- **Why the centre and not each team's home:**
  - The homes are pub(crate) in npt-sim with no accessor, so a home leash would open crates/sim, and EA-U4R-4 with it.
  - The origin needs nothing from npt-sim.
  - A 14 m leash keeps every soldier at least 2.6 m from any footprint, with homes, crates and the wall inside it.
- **The constants:** R_OUT and R_IN are distances in cm against positions in cm, so they are not rate-derived (A8).
  The walk's TURN_EVERY and RETARGET_EVERY stay pure command counts.
- **The builder may tune** R_OUT, R_IN and the sideways part, within the sim-only test's bar. Any tuned value lands in
  the plan's record.

## 3. The figure (Fix (b))

- **What:** per measured tick, every slot whose feet are in a building area, with the area. "Every" because P2's s2 had
  two (FP-P2).
- **Server side, in Logs::room_tick** (lab/src/logs.rs:470-578). It already iterates the Soldier records of
  `room.sim().entity_records()`. Each record's pub `area` is the sim's own feet area: greybox.area_of, half-open on each
  footprint; area i+1 is building i (world.rs:1753-1772). The test is `area != 0`.
- **Its record:**
  - a new name in room.jsonl's tick record, written on ticks where the list is not empty (the builder names it);
  - a measured total in Counts;
  - the run's summary line prints it as "in a building: N slot-ticks (slots …)". summary_line prints neither it nor
    everyone-in-view today.
- **A recording only:** no judge change and no marks.json change. The new name gets a marks/SERIES.md row ("none
  (recorded)"): the contract's check (b) fails on a written name with no row (contract.rs:64-112). The row is the main
  session's Edit, under "Names added with U4r" or a new BRAWL-1 heading.
- **Why server side:** the wire carries area 0 (proto snapshot.rs:150-160), so a client-side test would need a copy of
  area_of's rule, and two definitions could drift. This is the lab's record, not the bot's steering, so reading the
  server's state is right here.

## 4. The evidence (Fix (c))

1. **A unit test of the steering**, in npt-bots' tests/ (bots/tests/behaviour.rs has b1-b5; none tests the walk's
   closure). next_command with set positions must show:
   - inside R_IN, the walk is as today;
   - past R_OUT, the steer points to the origin;
   - the hysteresis between them;
   - the sideways part on each axis-aligned case;
   - None and dead keep the walk;
   - under a skip pattern (the last command re-run) and blocked steps (positions that do not move), the steer stays
     toward the origin until inside R_IN.
2. **The sim-only test** (no network):
   - **Setup:** npt-sim's World plus next_command; SS; 48 clients; a judged run's full 18,300 ticks; seeds 1 to 3.
   - **Two patterns:**
     - a command every tick;
     - a seeded skip pattern, where a skipped tick re-runs the last command unchanged, as InputBuffer::take does
       (room/src/input.rs:127-151).
   - **The position fed in:** World's own record, standing in for the decoded position. With no network they are equal,
     and the test checks steering, not decoding.
   - **Bar:** no Soldier record with `area != 0` on any tick, for every seed and both patterns.
   - **Printed, not barred:** each run's largest distance from the centre, and any soldier that has not moved for more
     than 300 commanded steps, named.
   - **Home and cost:** it lives in npt-bots' tests/, which needs no Cargo.toml change: npt-bots already depends on
     npt-sim and npt-client (A6). The cost is unmeasured: about 109,800 SS World steps per build, and check.mjs runs both
     builds. So, Q4:
     - (a) the full test is `#[ignore]` and run by name, in release, for the record, with its log kept. A short
       non-ignored copy (seed 1, 3,600 ticks, both patterns) guards the rule in every check.mjs run.
     - (b) the full test runs in check.mjs.
3. **Two 10-minute runs** at the fix's commit: SS P0 seed 1 and SS P3 seed 1. Each is the judged sets' measured-run
   command, with `--out .gate/brawl1-evidence/<run>`. Each is then judged with `--only PM-0001-2,PM-0001-17,PM-0001-57`
   into scratch, under the lane lock, with peak.py.
   - **Bar:** each run's everyone-in-view check (PM-0001-2's everyone_in_view_min_share) at or above 0.99, and the
     building figure's total 0.
   - **Also read:** each run's bytes, against U4r-F3's rates (P0 603.7 MB, P3 183.3 MB). A run above its rate comes to
     you before the re-run sets.
   - **Wall time:** about 250 s at P0 and 335 s at P3 (A9).
   - These runs are evidence: never counted, never in results/.
   - They are not a mode of c-u4r-runs.mjs, so the commands are run by hand, and the command lines are written into the
     evidence log.

## 5. The recordings that move (a re-pin ruling, Q3)

Any brawl change moves three runner recordings (A7):

| Recording | File | Old pin |
|---|---|---|
| SS-P0-s4 | precheck-recordings.json | 6933b56a, 16,950,444 B |
| SS-P2-s1 | det-recordings.json | 5b79abb2, 16,937,652 B |
| ES-P3-s2 | det-recordings.json | fad8f67d, 16,929,501 B |

- **Your rule:** "a SHA-256 mismatch comes to me whole" (INF-U4R-1).
- **What this plan asks (Q3):** that you rule the re-pin here, for exactly these three entries.
  - The builder regenerates each by its own route (`npt-lab hash` prints the new bytes and SHA-256). It records each new
    pin beside the old one in the build notes and the report.
  - det-u4.mjs and `npt-lab hash --manifest crates/lab/precheck-recordings.json` then pass at the new pins.
  - The three synthetic entries in recordings.json do not move.
  - Any other mismatch comes to you whole.
- **det-u4.mjs's time:** its non-cargo time was 227.2 s against the 240 s line (INF-R2-2), and its regeneration is
  about 195 s per call. The steering adds a few integer operations per command. If the sum crosses 240 s, it stops and
  comes to you, never retried.
- **The -30 pre-check set** (runs.txt at d666aeb) re-records SS-P0-s4 and checks it against the manifest. Once SS-P0-s4
  is re-pinned, it is re-made at U4h, as carry 40 and carry 88 already say, not in this lane.
- **c-u4r-r2.mjs's expect texts** hold brawl figures: b1 "everyone in view 0.273333", es "50.05", tick16 "720". They are
  restated at r2's next run. No r2 run is planned.

## 6. The items homed in this lane (brawl1-facts B)

**Part 1: they move no pin.** EA-U4R-1 and EA-U4R-2 are checked at the old pins before the brawl changes. det-u4.mjs and
the precheck regeneration must be byte-equal at 6933b56a, 5b79abb2 and fad8f67d, because their proof of no behaviour
change holds only at the old pins.

- **EA-U4R-2** (`pub(crate) type Sim = npt_sim::World`; one server constructor used by run and run_leg):
  - The constructor takes scene, seed, first tick, clients, event start, arm tick and bites. run needs event_start and
    arm_tick (scenario.rs:746, :760); run_leg passes Bites::default() and event start 0, then sets decoder_bite as today.
  - logs.rs and precheck.rs name Sim.
  - The printed "World::new(scene, ..)" strings (precheck.rs:691, :705) stay word for word, so runs.txt stays
    comparable.
  - lab_room (test-only) uses the constructor if it fits.
  - About 60-90 lines.
- **EA-U4R-1** (one shared loop for RQ8's steps 1-3), in the narrow form:
  - one shared function for steps 1-2 (accept; server.io with admitted and ended) plus the due count, with each caller
    keeping its own per-tick body;
  - run_leg uses SimClock, as the reviewer asked;
  - admitted and ended go to each caller's own sinks: the runner's logs.admit and logs.gone, and the pre-check's maps
    and o.ended.
  - About 40-70 lines. The one-loop driver (150-250 lines) is not proposed.
- **N2-1 with TE-R2-1:**
  - closed_checks gains the ruled fourth check with the ruled message; its doc says four.
  - fp1's doc rests the "not a replacement" claim on the roster.
  - The no-op assert on `ended` (scenario.rs:2209-2212) is dropped.
  - **One correction to the ruled proof (Q5).** Under FP-1's leave plant (room.leave deleted), owner.remove and
    ended.push still run, so two of the four checks fail by name, not four: "the room still holds slot 0: its slot was
    not freed" and "the roster still holds slot 0: a rejoin would be a replacement".
  - A second plant (roster.leave at room.rs:691 deleted) leaves the new check as the only one to fail.
  - Each plant is restored by Edit, with its sha256 checked.
- **TE-R2-2 and RR4-L2** (one change, parse()):
  - **Order:** the 64-bit checks come first and keep their messages, so TE-R2-2's ruled prefix holds. Then RR4-L2's
    caps:
    - a run's last tick (measured_end + after_max_ticks) above u64::from(u32::MAX);
    - the last burst's tick above the same;
    - --bursts above 1,000;
    - --burst-every-s 0 with more than one burst.
  - **Where the caps apply:** the burst caps wherever bursts are used (B1, as today's guard). The last-tick cap applies
    on every profile.
  - **Messages:** each names its options; the builder words them, and they go in the record.
  - **fp3's cases:**
    - TE-R2-2's three cases. Case 1 is on a B1 base line, case 2 equals the ruled message exactly, and case 3 pins the
      64-bit after-max refusal (`--warmup-s 3e17 --measured-s 3e17 --after-max-s 1e17`).
    - RR4-L2's ruled case (`--profile B1 --bursts 2 --burst-every-s 600000000000`, naming --burst-every-s).
    - One case each for the bursts cap, every-s 0, and an --after-max-s that fits in 64 bits but passes the u32 cap.
  - The comment at scenario.rs:1442 says exactly what is bounded.
  - About 45-70 lines.
- **RR4-L1:**
  - MAX_ENTRY_TICKS = 36,000, refused right after the bytes cap and before the runner checks, so the new message, not
    the runner's, is the one that fires. Its message is the ruled one.
  - r10_fp3 gains a case refusing 36,001 by name, and a synthetic entry at 36,000 that passes.
  - About 12-18 lines.
- **RR3-L2 (a):**
  - A Logs flag is set from open's `dir.is_some()`. On a no-log run, filed() resolves only and never grows `res`.
    `last` stays (finish reads it).
  - A new logs.rs unit test: Logs::open(None, ..), admit, filed over many ticks, then `res` is empty and started() is
    true.
  - About 25-35 lines.

**Part 2: BRAWL-1 itself** (sections 2 to 4), then the re-pins (section 5).

**Not in this lane:** EA-U4R-4, because the leash needs no npt-sim change; T-6's contract half, because contract.rs is
not edited; JDG-ONLY-1, because there is no judge change; and the check.mjs follow-up.

## 7. Who builds, and the order

- **One builder copy:** general-purpose, opus, effort xhigh (netcode), in an isolated worktree, building in
  .gate/shared-target (Mustafa's rule). Its brief comes from this plan once approved. Q6 asks whether the brief comes to
  you first, as U4r's did.
- **The order inside the copy:**
  1. **Part 1,** checked with check.mjs plus det-u4.mjs and the precheck regeneration byte-equal at the old pins. These
     results go in the build notes before Part 2 starts.
  2. **Part 2.**
  3. **The re-pins,** with check.mjs and det-u4.mjs passing at the new pins.
  - A stop rule as U4r's copies had: at its context limit, a copy writes its state and the next copy continues from its
    notes and the diff.
- **SERIES.md's row:** the main session's Edit, before the merge.
- **One review round on the lane's final diff:** test-engineer, netcode-architect and rust-reviewer, together as one
  round. Below High, a finding becomes a follow-up (the budget).
- **The merge into the spike branch:** CI, then your review of the head.
- **After the merge:**
  1. (B) and (C) on the shared target (section 8);
  2. the two 10-minute runs (4.3);
  3. the re-run sets (section 9), each set's results committed before the next.

## 8. The disk (kept-logs-draft.md; your step 5)

- **Today:** free 13,225 MiB. The kept raw run logs take 7,380 MiB (U4r 5,385, U2 1,996). The main folder's cache is
  776 MiB and the shared target 1 MiB.
- **The build phase:** a full check.mjs refills the shared target with about 9 GiB, so free falls to about 4.2 GiB. That
  is only just above the 4 GiB floor (the brief's section 4 rule 6). So no reviewer builds a target of its own, and no
  10-minute run comes in that phase.
- **After the merge:**
  - (B) and (C) take free back to about 13.2 GiB.
  - Then the evidence runs (about 0.7 GiB) and the main folder's release builds (about 1 GiB).
  - **The re-run sets,** at U4r-F3's start figures (condition 7): P3 524.4, P2 780.8 and P0 3,614.6 MiB, each plus the
    4,096 MiB floor.
  - **Before P0:** about 10,200 MiB free, against the 7,711 MiB P0 needs.
  - **After P0:** about 6,600 MiB free.
- **So the approved cleans hold this lane.** They do not hold U4g after it: its refill is about 9 GiB against 6.6 GiB
  free.
- **The draft's KEPT-LOGS question** (a narrow kit rule to remove a closed run-set folder) goes to Mustafa through you.
  Q7 asks when.
- **RAM:** the largest judge call in the re-run sets projects 4,719 MiB, on a 16 GiB machine with no swap.

## 9. The re-run sets and their expected lines (your step 6)

- **The sets:** P3, then P2, then P0 (split, as U4r-F2), by c-u4r-runs.mjs (with U4r-F3) at the fix's commit, before
  U4g's judged B1 runs. The run procedure on the card applies, items (1) to (7).
- **Expected:**
  - -2 SS PASS on every run;
  - -17 L1 P3, P2 and P0 PASS with 0 mismatches;
  - -17's three bites shown;
  - -57 L1 P3, P2 and P0 PASS, its three bites shown, its values recorded beside U4r's (any difference comes to you);
  - tick16's flag: "one overflow resync per client, all in the after phase, none in the measured window, no kick"
    (TICK16-WORDING);
  - each run's building figure 0.
  - Anything else comes to you whole.
- **The bytes:** each set's raw-log bytes are recorded against its projection.

## 10. Risks

- **The leash pins a soldier.** The sideways part and the hysteresis answer A4's pins. The sim-only test names any
  soldier that has not moved in 300 commanded steps. A soldier stuck outside a building keeps everyone-in-view, since it
  stays in area 0 and within the 64 m radius.
- **A stale decoded position under loss.** At P3, the own position comes from the newest complete tick, a few ticks
  old. At 20 cm a step, the 2.6 m margin from 14 m to the nearest footprint absorbs it. The P3 10-minute run checks it.
- **The npt-client read moves the scenario path.** The runner SHA-256s move anyway (section 5). R16's exception is
  asked in Q2.
- **The refactors (Part 1)** change behaviour only if their proof at the old pins fails. That failure comes to you.
- **det-u4.mjs crosses 240 s:** it stops and comes to you (INF-R2-2).
- **-57's bytes grow** with everyone in view. The evidence runs measure the bytes before the sets (4.3).
- **The disk:** the build phase runs at the floor (section 8). A floor stop is a stop, never a wrong result.
- **The builder's context:** the lane is wide (client, bots, lab, logs, hash, precheck, manifests). Two copies may be
  needed, and the stop rule applies.

## 11. Change type, tier, reviewers

- **Risk tier 1:** spike code and tests. No game code, no kit or gate file, no judge or marks.json change; marks/SERIES.md
  gains one row (recorded only).
- **Types and their evidence:**
  - netcode and sim behaviour: deterministic tests and the 10-minute runs;
  - the refactors: byte-equal regeneration at the old pins.
- **Reviewers:** test-engineer, netcode-architect and rust-reviewer (your ruling).

## 12. Planned size (SIZE-CHECK)

| Part | Lines changed (estimate) |
|---|---|
| Part 1: EA-U4R-2 60-90, EA-U4R-1 40-70, N2-1 8-12, TE-R2-2 with RR4-L2 45-70, RR4-L1 12-18, RR3-L2 (a) 25-35 | about 190-295 |
| Part 2: the npt-client read with its test 30-50; next_command, the leash and the unit test 80-120; the sim-only test 80-120; the figure in logs.rs with Counts and the summary 30-50; SERIES.md 2; the manifests 6; docs and comments 20-40 | about 250-390 |

- **The whole lane:** about 450-700 lines. The card carries 700 as the plan.
- **"Well past"** is more than twice that: more than about 1,400.

## 13. Choices for you

- **Q1, the leash:**
  - (a) toward the cluster centre: R_OUT 1,400 cm, R_IN 1,200 cm, with a sideways part; tuned within the sim-only test's
    bar (recommended: no npt-sim change, 2.6 m from the nearest footprint);
  - (b) toward each team's home, which opens npt-sim for an accessor (and EA-U4R-4).
- **Q2, the position's source:**
  - (a) a new npt-client pub read item, the own soldier's decoded position from the newest complete filed tick, as your
    U6 route (recommended);
  - (b) the brawl's existing `view`, which is the room-side mirror and so server state, which Fix (a) bars.
- **Q3, the re-pin:**
  - (a) ruled here for exactly the three entries of section 5, each new pin beside the old, any other mismatch to you
    (recommended);
  - (b) each mismatch comes to you whole during the build.
- **Q4, the sim-only test's cost:**
  - (a) the full test `#[ignore]`d and run by name in release for the record, plus a short non-ignored copy
    (recommended);
  - (b) the full test in check.mjs, on both builds.
- **Q5, N2-1's proof under the leave plant:**
  - (a) two of the four checks fail by name, plus the second plant where the new check fails alone (recommended: what
    the code can show);
  - (b) your ruled "all four", which the plant cannot show.
- **Q6, the builder's brief:**
  - (a) to you first, as U4r's was (recommended);
  - (b) this plan's review covers it.
- **Q7, KEPT-LOGS:**
  - (a) to Mustafa now, so a rule can land before this lane's build (recommended: the build phase gains margin, and U4g
    needs it anyway);
  - (b) to Mustafa before U4g only, since the approved cleans hold this lane.
- **Q8, Part 1's place:**
  - (a) in this lane, as ruled, proved at the old pins first (recommended);
  - (b) EA-U4R-1 and EA-U4R-2 move to U4h's lane, which their deadline (before the pre-check's re-run after U4h) allows,
    and that shortens this lane.
