# Advisor round ruling: U4f-F1's review round 1 (2026-11-20), from main's advisor.md

Read only. Diff: wip/u4f-f1-build.patch (sha256 37d87a10bed52b81..., base cb7e374) and the builder's worktree
(.claude/worktrees/agent-ad5f72ed146da5f93). The four seats' files read whole; the panel brief, plan rev 1.1, my plan
review (advisor-u4f-f1-plan.md), the build record, U4f2's brief rev 1.1 and my check of it, TEST-FIRST
(history.md:2234-2282). Nothing run but git/grep/sed/wc/sha256sum.

## Plan
1. Confirm the diff (patch hash, worktree hash) and read the code at each finding's lines (seam.rs, world.rs, room.rs,
   input.rs, the tests).
2. TE-F1-1 = NC-F1-1: the room's lost by construction; NC-F1-1's in-lane form (the sim reports the edges it ran);
   TEST-FIRST's reach (pass marks, judge, marks.json); rule fix vs wording.
3. NC-F1-2 and SA-F1-1: the first-command reach, honest rejoin; carry to U4f2 widened or fix in F1; what it adds to
   U4f2's lane and size; U4F2-SIZE.
4. Every Low (RR-F1-1..4, TE-F1-2..6, NC-F1-3, -4, SA-F1-2, -3): fix, carry or no action.
5. The fix pass: list, size against 300, round decision.
6. The -20 re-run's expected figures (my plan review :119-140): restate if moved.
7. What goes to Mustafa, which message.
8. Tables: ruling per finding, FP-n with acceptance checks, round decision, expected figures, the Mustafa line.

## Block 0: what I checked (DONE)
- The patch: sha256 37d87a10bed52b81ebc722a58667d0437bd2941b4460955905927bbb2b20e3d7 (wip/u4f-f1-build.patch, my run),
  equal to `git diff cb7e374 -- spikes/` in the worktree (my run); the worktree at cb7e374, 5 files, +205 -24.
- Read whole: the four seats' files, the panel brief, plan rev 1.1, the build record, my plan review, U4f2's brief rev
  1.1 and my check of it, msg-2026-11-20.md (U4F2-SIZE's text), TEST-FIRST (history.md:2234-2282), the 2026-11-17
  one-round choice (history.md:6818-6819), u4s-carries.md's U4f2 and U4g sections (:541-575), plan.md:1253 (U6's
  carries C-U6-1 to -18), marks.json -58 (:4148-4294) and -20 (:4749-4766).
- Code read at each finding's lines: seam.rs:72-80; world.rs:1338-1530; room.rs:352-358, :1027-1110, :1328-1350;
  input.rs:285-300; bots/src/lib.rs:289; scene.rs:229-234 (course: `projectile_capacity` 16, `grenade_fuse_ms`
  2,000).

## Block 1: TE-F1-1 = NC-F1-1, the room's lost (DONE)
- **The fact holds.** room.rs:1341-1342: `advance = p.wrapping_sub(presses[k])`, `taken = Command::edges_taken(presses[k],
  p)` = `now.wrapping_sub(last)` (seam.rs:78-80): the same expression, so `lost += advance.saturating_sub(taken)`
  (room.rs:1347) is 0 for every u8 pair. -20's (marks.json:4749) and -58's (:4214) "0 presses lost" cannot fail at run
  scale unless the fn departs from the wrapping difference (a saturating fn would show at -58's wrap).
- **What it was before F1.** The room never read the sim: cb7e374's `run_one` counted `lost += edges.saturating_sub(1)`
  (the patch's removed line), the room's own statement of the sim's one-edge rule. It failed at 350677f because that
  rule lost presses on a 2-edge advance, not because the room observed the sim. And "0 presses doubled" has been 0 by
  construction since U4f: a repeat is `last_run` (input.rs:362-376), whose counters are the slot's run counters, so its
  advance is 0 (room.rs:1343-1344; NC Block 2, TE Block 3). So F1 does not turn an observation into a constant: it
  turns the room's statement of a lossy rule into the statement of the rule ADR 0001:226 asks for ("a resent command
  can neither lose nor double a press").
- **My plan review's premise, corrected.** advisor-u4f-f1-plan.md:143-145 said the count "is still what the sim does not
  take, and the plant shows it can fail". As built (my condition 2's own contract), it is what the one rule does not
  take; the plant edits the fn, so room and sim move together (NC Block 2, :55-57). T1 (room/tests/commands.rs:618-643,
  its refusal assert :642) and T2's count halves (sim/tests/world.rs:1826-1921) are what tie the sim's take to the rule.
- **The in-lane fix (NC-F1-1's option) weighed and declined for F1:**
  - Its gain: a run that sees a sim taking fewer edges than the rule gives (a loop or reference bug). T2's count halves
    and plant 1 already show that at unit scale, by name (f1-b3-plant1.log), and in an honest run the room's run
    counters and the sim's `claimed` walk the same counter sequence (NC Block 2, :58-62), so the new count would read 0
    there too.
  - Its cost: a per-slot count in the sim, read through the seam (a `SimCore` method: three test doubles implement the
    trait, room/tests/room.rs:506 and :974, sim/tests/seam.rs:328) or an `InputReports` field (which the room's
    `clone_from` copies each tick, room.rs:1046); counting at every edge's place (the dead or plate return, melee, each
    loop, both bites); the room's lost moved after the step. About 50-80 lines: F1 near its 300 stop, and a design
    change to the seam, which the 2026-11-17 choice gives a second round (history.md:6818-6819).
  - So: keep the code, bind the wording. Not a lowered bar: the mark's text, number, series and SERIES.md:42's meaning
    are unchanged, the count never observed the sim, and the system now meets the ADR property by design.
- **Is anything his?** No decision. TEST-FIRST keeps his "any change to a pass mark, to the judge or to marks.json"
  (history.md:2262); none changes. The rule change itself was R20-Q1, already ruled under rule 2 and in the 11-20
  message (msg-2026-11-20.md:15-20, :25). But he should know a judged check now holds by design: one line in the next
  day's message (Block 6), and a judged "presses seen equal the scripted count" check goes on TEST-FIRST rule 1's
  "After the close" list as his (a marks.json change, NC-F1-1's last sentence).
- **The binding (FP-1, FP-5 (i)):** the code docs at seam.rs:76-77 ("so the two cannot drift"), room.rs:353-357 and
  :1328-1331 say the room's lost is counted with the one rule, not read from the sim; and the -20 L1 record says, in
  these terms: "'0 presses lost' is 0 by the press-counter design after U4f-F1: npt-room counts with the rule npt-sim
  runs (`Command::edges_taken`), as '0 presses doubled' has been 0 by design since U4f; it guards that rule against the
  room's own wrapping difference, not the sim's take. The sim's take of every edge is proven by T1 and T2 and their
  planted bugs. The run's evidence that no scripted press went missing is presses = 432,000 per option-B run, the
  scripted count (48 x 18,000 / 2); the per-tick state hashes equal to 350677f's show the 1-edge path unchanged and the
  extra fire edge state-neutral at run scale." TE-F1-1 and NC-F1-1: Medium, resolved by the binding (NC's "High then"
  applies only if the PASS were reported as run-scale evidence of the take).

## Block 2: NC-F1-2 and SA-F1-1, a session's first command (DONE)
- **The facts hold** (read): kind 5 runs only `if let Some(prev) = &self.accepted` (input.rs:287); every join builds a
  fresh buffer while the slot's references live on (the sim's `claimed`, world.rs:681-682; the room's `run_presses`,
  room.rs:508-510); `JoinAccepted` carries no counters (room.rs:851-861); the bots start at `presses: [0; 4]`
  (bots/src/lib.rs:289). Under F1 the sim takes the whole advance (world.rs:1346-1350, :1504-1509).
- **Its reaches, named** (the carried item names all five):
  1. hostile: a session's first accepted command carries up to 255 edges per button, a counter moved back read as
     256 minus the step; throw edges become a free pool of grenades bursting at one point on one tick (SA Block 2: ES 64
     x 50 = 3,200 damage in 4 m against health 1,000; SS up to 320), against one grenade before F1;
  2. repeatable at every rejoin (one `Join` per connection, no rejoin limit found: SA Block 3);
  3. honest: a restarted client (counters from 0) against the slot's old references reads (0 - old) mod 256 per button:
     a phantom shot (as before F1), a reload start, grenades if it throws;
  4. hidden: F1 counts that advance seen with lost 0, where cb7e374 counted it lost (SA Block 3), so the press series no
     longer flags it;
  5. the cross-player pool: a full pool refuses every other player's throws until the fuses run out (SA Block 2).
- **Who meets it before it closes:** no -20 or -22 run (no rejoin; -22's bots press nothing); no -21 run (no -20, -21
  or -22 suite replaces a session, u4f-brief.md:1402; no forger throws or rejoins, :533; no bot presses throw: SA Block
  3); the clock suite reconnects but never fires (lab/src/scenario.rs:1213-1214). The first judged runs that meet it:
  -19 (R3 and R7: trapped, rejoining browser clients; plan.md:730) and PM-0004-61's online half (traps). One more to
  guard: U4g's -58 must wrap the press counters in 10 s (marks.json:4246, :4254); if its suite starts a client's press
  counters near the wrap on its first command, that is reach 3 at a judged run.
- **Not in F1:** a session's baseline is a design point ADR 0001 part 5 does not settle (NC Block 4), so building it in
  F1 would need a second round (history.md:6818-6819); F1 is at 205 of its 300 stop; and it would hold back -20's re-run.
- **Not in U4f2 either (condition 7 revised, with reason):**
  - The form two seats lead with (SA's (a), RR-F1-3's first example): `JoinAccepted` carries the slot's run counters,
    the client starts from them, kind 5 checks the first command against them from the join tick (input.rs:270's
    anchor). It keeps ADR 0001:226 across a rejoin for a restarted and a continuing client alike, and needs the wire
    (npt-proto, the catalogue's JoinAccepted row, npt-client, npt-bots): outside U4f2's lane (u4f2-brief.md:130-132).
  - That wire change is already U6's: C-U6-13 puts the room's event base on `JoinAccepted` or `FullState` (plan.md:1253;
    u4s-carries.md:357-365, carry 75's stopgap), and C-U6-16 bounds the full states one slot draws by Resync and re-join
    together. The press base belongs in the same message and the same review.
  - NC's room-only form (a per-session offset and kind 5 against {join tick, counters 0}) fits U4f2's npt-room, but it
    holds only if every session's counters start at 0: a client that reconnects keeping its counters would be refused
    for up to 255 ticks and then have its old presses taken again (doubled). And folding it in costs about 70-150 added
    lines (code 25-45, tests 45-100): U4f2's estimate to about 1,800-2,400 (up to about 2,900 at U4f's 1.2) against the
    3,000 stop, a figure that differs from what msg-2026-11-20.md:40-42 puts to him.
- **Ruling: carry, widened, to U6 as C-U6-19,** with C-U6-13 and C-U6-16:
  - **Text:** "A session's first command (ADV-U4FF1-3, NC-F1-2, SA-F1-1, RR-F1-3): `JoinAccepted` carries the slot's run
    press counters (the room's `run_presses`, equal to the sim's `claimed`); the client and the bots start each session
    from them; the room checks a session's first command under kind 5 against them, `covered` from the join tick. Tests:
    a fresh session's first command 200 throw edges on is refused (PressEdge); an honest restart that starts from
    `JoinAccepted`'s counters makes no shot, reload or grenade it did not press, and its seen equals its presses; N
    rejoins buy nothing; each plant (the base not sent, the first-command check skipped) fails by name. U6's brief may
    propose the room-only form instead, with the netcode and security seats' read, if it shows every session's counters
    start at 0. ADR 0001 part 5's sentence on where a session's counters start goes to the readings PR."
  - **The gate:** closed before -19's runs, before PM-0004-61's online half, and before any run with a throwing or
    rejoining hostile client or a client restarted mid-run. Every unit brief before U6's merge (U4g, U4h, U5) states in
    one line that its runs have none, or brings this item first.
  - **To U4g's brief:** -58 reaches the press counters' wrap without a first command carrying more edges than the ticks
    it covers from the slot's references: by pressing through the wrap, or by a start the lab gives both the client and
    the slot's references (carry 75's form for event numbers), never by an unchecked first command.
  - **To U4f2 (C-U4F2-9, a record line, no build):** -21's report names kind 5's first-command bypass (input.rs:287),
    that no -21 forger rejoins or throws, and C-U6-19 as its closure.
- **What it adds to U4f2:** nothing to its lane (npt-room stays open for C-U4F2-3 and -8 only), about 3 report lines, and
  SA-F1-2's case (C-U4F2-10, Block 3: about 10-20 lines inside U4F2-Q4's f33 work). The estimate stays "about 1,700 to
  2,250" (up to about 2,700 at 1.2): **U4F2-SIZE as written in msg-2026-11-20.md:39-48 does not change.**
- **Security status:** SA-F1-1 stays open until C-U6-19 closes it, so the PR that carries it to main stays
  needs-mustafa at any severity (advisor.md step 4; SA's process note). PR #40 is needs-mustafa anyway; F1's record and
  U4's report name it in a line of its own.

## Block 3: the Lows (DONE; the rule: fix only where the file is open and cheap, else carry by number or no action)
- **RR-F1-1, the full-pool rescan:** no action. Bounded (at most 255 x pool `live` reads per command, no allocation:
  RR Block 2, SA Block 2); its large reach is C-U6-19's first command or a bites build; inside a session kind 5 caps a
  command at 127 edges. Changing `throw`'s return moves T2 (f)'s exact lists (sim/tests/world.rs:1895-1921) for no
  measured gain. F1's record names it.
- **RR-F1-2 = TE-F1-2, the u8 wrap:** fix, FP-2 (a unit test; seam.rs's fn is open and the test is about 10 lines).
- **RR-F1-3, the honest restart:** folded into C-U6-19 (Block 2).
- **RR-F1-4 = TE-F1-4, Wasm parity of the multi-edge path:** no code; FP-5 (ii) states it in F1's record ("by reading:
  the rule is integer-only and an extra throw runs the single throw's float path; the wasm step's 27,000 ticks are not
  shown to hold a multi-edge command"). Counting them needs a program run on the recordings: not worth it for a Low.
- **TE-F1-3, T1's sim half never seen failing:** fix, FP-3 (two asserts swap places in an open test file; one plant
  re-run).
- **TE-F1-5, a bite's extra edge untested:** carry to U4g by id, beside carry 47 (u4s-carries.md:549-555), which already
  has U4g edit crates/sim: two bites-build 2-edge twins (PM-0001-21/no-cooldown, refuse-not-clamp), taken only if open
  and cheap there, else left for crates/sim's next opener. No -21 bite run sends a multi-edge command (kind 5 bounds the
  forgers; the kind-5 forger moves fire only: SA Block 4), so no judged figure rests on it.
- **TE-F1-6, T2 (f)'s literals:** fix, FP-4 (two asserts naming the course's `projectile_capacity` 16 and that the fuse
  outlasts the test's 20 ticks: scene.rs:232, :234).
- **NC-F1-3, ADR 0001 part 5's unstated order:** carry to the readings PR before R5 (the card's list "For the readings
  PR before R5"): one sentence each for the per-command order (reload, fire, throw), one shot per command, and collapsed
  throw edges at one origin and tick. The stand-in's docs state the first two (world.rs:30-41, :1343-1345).
- **NC-F1-4, `predict` reads `claimed`:** met by carry 47 (U4g): its reset sets "the press counters of the client's own
  command at the snapshot's last-run sequence, exactly as the authority held them" (u4s-carries.md:550-554). U4g's
  brief names NC-F1-4 beside 47 (a replay without that reset reads a backwards step as up to 255 edges). No new carry.
- **SA-F1-2, the attribution's own kind:** carry to U4f2 as **C-U4F2-10**: C-U4F2-1's "a refusal record of its own
  kind" (u4f2-brief.md:44-46) stays exact after C-U4F2-3 puts the room's records in the sim's struct form (room and sim
  kinds kept distinct); f16 or f33 gains one case under PM-0001-21/no-press-edge whose forged kind-5 sequences carry sim
  Cooldown records and still count in `unrejected_count`, with a plant ("any kind matches") failing it by name. Its
  option-B kind-7 note (a late forged edge's record names the next honest sequence) is a record line: the forged suite
  runs at P0, with no late command (u4f-brief.md:533). About 10-20 lines, inside the f33 row's range: no size change.
  The own-kind rule itself is not changed (it would be his: TEST-FIRST).
- **SA-F1-3, the throw has no rate state:** FP-5 (iii) in F1's record ("the stand-in's throw has no cooldown, so neither
  T2 (f) nor any -21 figure shows ADR 0001:236's ability check; a press bank of up to 127 throw edges bursts in one
  tick"); the v2 rule ("every press action has server-side rate state, or takes at most one edge per command") goes to
  the readings PR with NC-F1-3.
- **Notes, no finding:** room.rs:1046's `clone_from` per tick is already C-U6-15's "no per-tick copy of the sim's reports"
  (plan.md:1253): no action. reload_ms = 0 (RR Block 2): pre-existing, no scene has it: no action.

## Block 4: the fix pass, its size, the round (DONE)
- **Who:** F1's builder setup (one agent, Opus 5.5 at xhigh, the same worktree, notes as a B5 section of its
  u4f-f1-build.md, snapshots by worktree_patch.py, the shared target under the lane lock, the standing write sentence
  word for word). FP-5 is the main session's.
- **Size:** FP-1 to FP-4 about 20-30 added lines (FP-3 a move, counted added and removed), so F1 ends at about 225-235
  of its 300 stop. SIZE-CHECK after each item; past 300 the stop stands as planned.
- **No second round.** The 2026-11-17 choice (history.md:6818-6819): a second round only where round 1 leaves a High or
  a design change. Round 1 left no Critical or High (four seats APPROVE WITH FINDINGS; 4 Mediums, all ruled above), and
  the pass changes no design: doc comments, one new test, two test edits; no code token in src changes; the one design
  point (C-U6-19) leaves F1.
- **My check of the pass instead** (not a round): the pass's diff whole against 37d87a10's tree; in src only doc-comment
  lines change (seam.rs, room.rs); FP-2's and FP-3's pass and plant logs and the restored sha256s; check.mjs twice
  (all 11 steps; 642 measured and 739 bites, +8 on 634 and 731); the final snapshot's sha256. If an item departs from
  its named fix or its acceptance check, round 2 runs on that item alone with its seat (test for TE-*, rust for RR-*,
  netcode for NC-*); a third only for an open Critical or High.
- **Then, as planned:** the merge into the lane (gate --no-smoke, check.mjs, CI), my head check, -20's L1 re-run from the
  merge commit (condition 5), and condition 10 (U4f2's brief rebased on F1's merge, with C-U4F2-9 and -10 as ruled here).

## Block 5: the -20 re-run's expected figures (DONE)
The pass touches no code path a run executes (FP-1 is doc comments; FP-2 to FP-4 are tests), so my plan review's
expectations (advisor-u4f-f1-plan.md:119-140) stand, restated with the round's precisions:
- every summary line equal to runs.txt:23-29 but its wall time, and but the header lines' binary sha256s (a new build:
  NC Block 3); fate log digest, client hashes, shots, projectiles and audit included;
- every measured room log's per-tick `h` equal, tick by tick, to the kept raw logs
  (.gate/netcode-proof-test/runs/u4f-20-350677f0aef2/measured/; bites/ for the bite run);
- `tick.v.press_lost_count` 0 everywhere, the six ticks included: by construction (Block 1), not evidence of the take;
- presses: B 432,000 per run (the scripted count), A 432,003 / 432,003 / 432,009 (seen is the advance, unchanged);
  doubled 0;
- verdicts: PM-0001-2 "SS runs" PASS, 6 of 6 valid; PM-0001-20 option A PASS 3/3, every figure as at 350677f; option B
  PASS 3/3, on-time and missing as at 350677f (equal to A's per seed), lost 0, doubled 0; L2 parts NOT-RUN; the bite
  PM-0001-20/redundancy-1 shown by its on-time 0.894544 (marks.json:4794-4795's 99.9 %), its lost 3,614 going to 0 by
  design; the reorder report row still "not available" (ADV-U4F20-2, his), the event's figures stated by hand;
- any other difference stops the record and comes to me before the verdict counts.

## Block 6: what goes to Mustafa (DONE)
- **Nothing to decide, and U4F2-SIZE does not change** (Block 2): msg-2026-11-20.md:39-48 stands as written.
- **One entry in the next day's message** (2026-11-21 Amsterdam; or in msg-2026-11-20.md's "Choices taken without you"
  if it has not gone out), under TEST-FIRST rule 2, so he can reverse it:
  - U4f-F1's one review round (four reviewers) found nothing serious; a small fix pass (doc wording, one new test, two
    test touches), checked by the advisor, with no second round.
  - After F1, "0 presses lost" in -20 and -58 holds by the press counters' design, as "0 presses doubled" already did:
    the room counts with the same rule the simulation runs. Unit tests with planted bugs prove the simulation takes each
    press; the runs show the press count equal to the scripted one (432,000 per option-B run). A judged check of that
    count would be a marks.json change, his: on the after-the-close list.
  - A gap F1 widened: a client's first command after it joins is not edge-checked, so a hostile client could throw a
    pool of grenades at once on each rejoin, and a restarted honest client makes phantom presses. No run before -19's
    (run day R3) meets it; it goes to U6 with the join message's other starting values, closed before -19 runs.
- **"After the close" gains one line:** a judged "presses seen equal the scripted count" check for -20 and -58 (NC-F1-1;
  a marks.json change, his).

## The ruling per finding (each seat's severity kept)
| Id | Sev | Ruling | Why (evidence) |
|---|---|---|---|
| TE-F1-1 = NC-F1-1 | Medium | **keep the code, bind the wording: FP-1, FP-5 (i)**; nothing for him to decide; one line in the next message; a judged presses-equal-scripted check on "After the close" (his) | room.rs:1341-1347 with seam.rs:78-80: lost 0 by construction; cb7e374's room never read the sim either (`lost += edges - 1`); doubled 0 by design since U4f (input.rs:362-376); the in-lane count costs a seam change and a second round for an observation T2's plants already make (Block 1) |
| NC-F1-2 | Medium | **carry, widened: C-U6-19** (with C-U6-13, -16); C-U4F2-9 a record line in U4f2; U4g's -58 line | ADR 0001 part 5 names no session base; the honest restart reaches it (bots/src/lib.rs:289; room.rs:851-861); no run before -19 meets it (Block 2) |
| SA-F1-1 | Medium | **carry, widened: C-U6-19**, gated before -19, PM-0004-61's online half and any run with a throwing or rejoining hostile client or a mid-run restart; open security finding named in F1's record and U4's report | input.rs:287; the pool burst (world.rs:1504-1509, :1525-1560); repeatable per rejoin; hidden from the series; the wire form fits U6's JoinAccepted change (plan.md:1253, C-U6-13) |
| RR-F1-1 | Low | no action (recorded) | bounded, no allocation; its large reach leaves with C-U6-19 (Block 3) |
| RR-F1-2 = TE-F1-2 | Low | **fix: FP-2** | seam.rs:78-80 untested across the wrap with more than one edge |
| RR-F1-3 | Low | folded into C-U6-19 | the honest restart (Block 2, reach 3) |
| RR-F1-4 = TE-F1-4 | Low | no code: **FP-5 (ii)** record line | a count needs a program run on the recordings |
| TE-F1-3 | Low | **fix: FP-3** | commands.rs:636 stops the plant before :642 |
| TE-F1-5 | Low | carry to U4g by id, beside carry 47 (open and cheap only) | no -21 bite run sends a multi-edge command |
| TE-F1-6 | Low | **fix: FP-4** | sim/tests/world.rs:1899, :1904, :1908 pin scene.rs:232, :234 unnamed |
| NC-F1-3 | Low | carry to the readings PR before R5 | ADR 0001:235-237 states neither order, one shot nor collapsed throws |
| NC-F1-4 | Low | met by carry 47 (U4g); U4g's brief names it beside 47 | u4s-carries.md:550-554 resets the press counters |
| SA-F1-2 | Low | carry to U4f2: **C-U4F2-10** (one f16/f33 case under no-press-edge, its plant); no size change | C-U4F2-1's own-kind rule (u4f2-brief.md:44-46) after C-U4F2-3's struct form |
| SA-F1-3 | Low | **FP-5 (iii)** record line; the v2 rate-state rule to the readings PR | the stand-in's throw has no cooldown (world.rs:1525-1560) |

## The fix pass (F1's builder; FP-5 the main session's)
| FP | Finding | The change | Acceptance check |
|---|---|---|---|
| FP-1 | TE-F1-1 = NC-F1-1 | Doc comments at seam.rs:73-77 (`edges_taken`: drop "so the two cannot drift" for "the room's count and the rule cannot drift; the room does not read the sim's take, which T1 and T2 test"), room.rs:353-357 (`presses()`) and :1328-1331 (`run_one`): lost is the advance less what the one rule takes, counted in the room, not read from the sim, 0 while the rule takes every edge | the three docs read so; `git diff` shows only `///` lines changed in src; check.mjs's doc step PASS |
| FP-2 | RR-F1-2 = TE-F1-2 | One test (sim/tests/seam.rs, or beside T2): `edges_taken` gives (255, 1) 2, (254, 0) 2, (1, 0) 255, (0, 255) 255, (7, 7) 0, (0, 1) 1 | passes; the plant `now.saturating_sub(last)` fails it by name (log kept), restored by Edit with seam.rs's sha256 as before; +1 measured and +1 bites |
| FP-3 | TE-F1-3 | T1 (commands.rs:618-643): the refusal assert (:637-642) before the presses assert (:635-636), or both as one tuple | passes; the plant `edges_taken` = min(advance, 1) re-run once fails T1 by name at the refusal assert ([] against [slot 0, seq 4, Cooldown]); restored, seam.rs sha256 63057544...16499 |
| FP-4 | TE-F1-6 | t2_f (sim/tests/world.rs:1895-1921) first asserts `Scene::course()`'s `projectile_capacity` is 16 and that its grenade fuse in ticks exceeds the stream's 20, each with a message naming what it pins | passes; by reading |
| FP-5 | TE-F1-1, RR-F1-4, SA-F1-3, SA-F1-1, RR-F1-1 | The records: F1's record and the -20 L1 record carry (i) Block 1's sentence, (ii) the Wasm line, (iii) the throw's missing rate state, (iv) SA-F1-1 open as C-U6-19 and RR-F1-1's no action with its reason, (v) my plan review's :143-145 premise corrected (Block 1). The carries filed by number: C-U4F2-9, -10 (u4s-carries.md's U4f2 section); C-U6-19 (U6's list, as C-U6-17 and -18 were); U4g's lines (TE-F1-5, NC-F1-4 beside 47, -58's press start, "no such client"); the readings PR list (NC-F1-3, SA-F1-3's rule, the session-start sentence); "After the close" (the judged presses check) | my check reads each line before the merge (carries) and in the -20 record |
End checks: check.mjs twice, all 11 steps, 642 measured and 739 bites, 0 failed, 0 skipped; the final snapshot's sha256.

## The round decision
**One round, closed. No second round** (the 2026-11-17 choice: no High left, no design change in the pass); my check of
the pass in its place (Block 4). Round 2 only for an item that departs from its fix or its acceptance check, with its
seat; a third only for an open Critical or High. Verdict on the round: **APPROVE WITH CONDITIONS FP-1 to FP-5**, then
the merge.

## The re-run's expected figures
As advisor-u4f-f1-plan.md:119-140, unchanged by the pass (Block 5): summary lines equal but wall time and the binaries'
sha256s; per-tick `h` equal to 350677f's kept logs; lost 0 (by construction), doubled 0; presses B 432,000 each, A
432,003 / 432,003 / 432,009; -2 SS PASS 6/6; -20 A PASS 3/3, B PASS 3/3; L2 NOT-RUN; the bite shown at 0.894544 with
lost 3,614 to 0; the reorder row "not available". Any other difference stops for me.

## To Mustafa, in one line
Nothing to decide and U4F2-SIZE unchanged; one entry in the next day's message (Block 6: F1's round and pass, "0 presses
lost" now held by design, the first-command gap to U6 before -19) and one line on "After the close" (a judged
presses-equal-scripted check, his).
