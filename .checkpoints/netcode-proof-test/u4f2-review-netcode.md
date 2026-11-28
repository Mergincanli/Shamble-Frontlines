# U4f2 review round 1: the netcode seat (netcode-architect), 2026-11-20

(No Edit tool in this agent: the file is rewritten whole with Write after each block, as the brief allows.)

## Verdict: REQUEST CHANGES (block 3 has the table)

## Plan
1. Read the panel brief, u4f2-brief.md rev 1.2, u4f2-build.md (done), the advisor's brief check (Q1-Q5 rulings), the
   plan's -21 entry and marks.json's -21 checks (what each check reads).
2. Read the patch whole (forged.rs, bots lib.rs, scenario.rs, room input.rs/room.rs, contract.rs, client lib.rs, the
   scripts), with the forged suite against ADR 0001 / plan rev 3.3 (refused by server state, counted, attributed).
3. F-B: trace kind 7's item path (bots Forge::reload / fire items) -> room -> sim run_command (world.rs:1353 `!s.alive`
   return, respawn's weapon reset) -> forged.rs's kind-7 rule (Cooldown/Reload records). Find the cause in the code,
   cross-check with the hand-long logs if they carry anything, and propose a fix inside the plan (no pass mark, judge or
   marks.json change).
4. F-A (lag client standing), C-U4F2-5's route (c) against Q1, the fixed-lead endpoints, HOLD_STEPS vs carry 39.
5. Findings, verdict.

## Blocks
(paths: `W` = the builder's worktree
`/home/user/Shamble-Frontlines/.claude/worktrees/agent-aa62d31b6fa5b9807/spikes/netcode-proof`, `P` = the patch
`.checkpoints/netcode-proof-test/wip/u4f2-build.patch`, line numbers in it)

### Block 1 (DONE): F-B, its cause in the code, and a fix inside the plan (NC-U4F2-1)

**What the run shows** (`.gate/u4f/u4f2-hand-long2/l1-21-forged-a-s1/lab.jsonl:52-62`): every kind sent 1,523 (kind 10
1,656), unrejected 0 for all but kind 7 (slot 6): 18. Slot 6's `client_counts` (lab.jsonl:10): commands 18,299 for
18,300 ticks, `commands_repeat` 0, `late` 0, `steps_without_command` 0, `stamps_skipped` 0: every forged command ran,
one a tick. So the 18 are items that ran with no Cooldown or Reload record. No log carries deaths (the room log's events
are admits only; the client log carries hashes), so the cause is read from the code, and it holds:

1. **A dead soldier's command consumes its edges and records nothing.** `W/crates/sim/src/world.rs:1351` sets
   `claimed` to the command's counters, then `:1353` returns on `!s.alive` before the weapon (`:1412-1499`, where
   every Reload, Cooldown and Ammunition record is made). forged.rs's kind-7 rule counts an item rejected only on a
   Cooldown or Reload record of its sequence (P:2705-2709, P:2736), so every item run while slot 6 is dead counts
   unrejected. Kinds 1-6 and 8-11 are refused before the sim (room, pump), so only kind 7 is exposed.
2. **A revive resets the weapon.** `world.rs:1192-1203` (`revive`) sets `wait` 0, `reload_left` 0 and the magazine full.
   A fire item whose d-1 ran dead and d alive fires its forged press (no record; the twin fires too, by held fire, so no
   hash difference). A reload item whose d-1 is the first live command finds the magazine full (`:1427`: the honest
   edge refused), fires (`:1473`), and d's forged edge then starts a legal reload: no record, and a state the twin
   lacks (`reload_left`, `FLAG_RELOADING`: hashed at `:2011`, `:2016`), so `twin_hash_mismatch_count` > 0 on that run.
   No open-loop construction closes this hole: whichever command first runs after a revive meets a full magazine.
3. **Why slot 6 dies, and why only long runs show it.** C-U4F2-5 (c) stands every steady soldier (P:527); Steady holds
   fire on every command (`W/crates/bots/src/lib.rs:1050-1054`) at a fixed blind point within 1,000 cm of the centre
   (`:75`, `:505-511`, `:1037`); each shot's ray runs through the point to the full 60 m reach
   (`world.rs:1636-1643`). SS: damage 1, health 1,000, respawn 1,000 ms (`W/crates/sim/src/scene.rs:65`, `:185-186`).
   A soldier on one shooter's ray takes 14.58 hits a second and dies every 68.6 s + 1 s; a death leaves 29 commands
   run dead (respawn_left 30, `world.rs:1701`, `:1809-1818`), about 2.4 items at `--forge-every 12`. 18 items is about
   7 deaths of slot 6 in 610 s. A short run cannot kill on one ray (f16's 6 s, the hand runs' 15 s), which is why f16
   and hand-a/hand-b read 0. The first shape's 41 of 1,270 fits the same cause: its forged edges sat inside a 54-command
   reload window, which a revive empties.
4. **The builder's reading is right**, and two things follow that it did not name:
   - **The 11 forged bites lose their power at the judged length.** -21's check is `max` over every kind's
     `unrejected_count` (`spikes/netcode-proof/marks/marks.json:4958-4969`), so kind 7's dead items fail it on every
     600 s forged bite run whatever the bite: a no-* bite whose defect never reaches its own kind would still read
     "FAILS" (the hand-b runs showed each bite's own kind only at 15 s).
   - **The twin check can fail spuriously** (item 2): about (slot 6's deaths) / (12 x 40) a run, about 1.5 % at seed 1
     A, about 5-9 % over the set's six forged measured runs if their death counts are alike.

**Severity: High** (a check that fails on a judged run for a construction reason, and 11 bites whose evidence would not
hold).

**The fix (inside the plan: no pass mark, judge rule, marks.json text or existing SERIES.md row changes):**
- (a) **Count only the items the sim could judge.** 2.6's kind-7 row defines the item as "a fire or reload press edge
  where the sim refuses it" (u4f-brief.md:525); an edge into a dead soldier, or one whose construction a revive reset,
  is not one. After each room tick the lab hands the attribution each forger's alive flag from the server's own World
  (its Soldier record; the lab already reads Soldier records for the movement series, rev 3.3's A1,
  u4f-brief.md:540-546). A kind-7 item is left out of `sent_count` and `unrejected_count` when its forger's soldier was
  dead at the end of the tick before any tick that ran one of the item's commands d-3 to d (the three rest commands and
  the forged one; the forger gives consecutive sequences, so they are d's sequence less 1, 2, 3): that covers a death
  before d, a dead d-1 with a live d, and the reload hole (a first live d-1 means a dead d-2). It is counted apart in a
  new report-only field `event.forged_kind.dead_left_out_count` (one new row in U4f2's SERIES.md subsection,
  P:2419-2425). Size: about 30-45 lines (forged.rs: a 4-entry ring per forger of (seq, dead at its run); scenario.rs:
  the flags passed with `tick`), one f33 case on hand-made records (neg: a dead-window item with no record is left out
  and counted apart; a live item with no record still counts unrejected), its plant (the dead window ignored: the case
  fails by name), one SERIES row.
- (b) **Shrink what (a) cannot reach, the twin hazard** (item 2's reload hole changes state, so the twin check sees it
  whatever the attribution counts): `RELOAD_ITEMS` (P:16) from 40 to about 400 (3-4 reload items a judged run, the
  hazard about ten times smaller, about 1 % over the set; also about 1.6 shots a second back on slot 6); f15's
  `taken[1] >= 2` (P:806) becomes `>= 1` at its 300 steps. About 3 lines. The residual is named in -21's report.
- **Whose:** (a) reads "forged command" for kind 7 the way A1 reads "moves": the advisor rules it, and like A1 it joins
  the readings PR's list for him (plan :971-974). If the advisor reads it as changing what an existing SERIES row
  means (`sent_count`, SERIES.md:158: "forged commands sent"), it is Mustafa's under TEST-FIRST and the fix pass stops
  for him.
- **Not inside the plan:** a sim record for a dead soldier's edges (crates/sim is shut, and kind 7's own-kind rule
  would gain a kind: his); a leash or a dead-bit gate in the forger (closed loop, against u4-build-plan.md:411-412 and
  2.12's rule, Q1); new aims for Steady (Q1 point 1 keeps the aim).

**Runs for the fix pass** (the forged run is deterministic, so the same 610 s run reproduces the 18):
- Before the fix (a diagnostic, not committed): `npt-lab scenario ... --suite forged --forge-every 12 --cmd-timing A
  --seed 1 --warmup-s 10 --measured-s 600` with a count of kind-7 items with a dead tick in d-3..d. Expected: every
  one of kind 7's 18 unrejected items among them, which proves the cause.
- After the fix, the same run: kind 7 `unrejected_count` 0, `dead_left_out_count` about 18 to 20, `sent_count` at
  least 1,000; the same under `--cmd-timing B` (with slot 6's `commands_rerun_count` 0, so option B's re-run cannot move
  the weapon clock between d-1 and d); and one no-* bite other than no-cooldown at 600 s (for example no-aim-bounds)
  whose only unrejected kind is its own (kind 6), showing the bites' power restored.

### Block 2 (DONE): the rest of the diff with this seat's focus

**NC-U4F2-2, Medium: `FIRE_REST` (P:17) is a tick count written as a literal (carry 39), and kind 7 breaks at 60 Hz.**
- The rest must let the weapon's cooldown run out before the honest press on d-1: after a held-fire shot the wait is up
  to `interval_us x TICK_HZ` units and falls 1,000,000 a command (`world.rs:1415-1416`; `UNITS_PER_TICK`,
  `W/crates/sim/src/seam.rs:177`), so the rest needs ceil(68,571 x TICK_HZ / 10^6) commands: 3 at 30 Hz (2.06), 5 at
  60 Hz (4.11). At 60 Hz, whenever the last held shot falls on d-4 (about 1 in 4), the honest press is refused and the
  forged press on d, its weapon now ready, fires: about a quarter of kind 7's fire items unrejected. The 60 Hz run's
  change-set "replaces npt-sim through the seam and changes no other crate" (`W/crates/sim/src/ticks.rs:10-11`), so
  `FIRE_REST` would stay 3. check.mjs:760's name rule (`_TICKS$|CATCHUP`) does not see the name.
- The doc's condition (P:11-15, "its interval is more than a tick") is wrong: the forged press on d is refused only if
  `interval_us x TICK_HZ` is at least 2,000,000 (d-1's shot leaves -999,999 + cost, d takes 1,000,000 more). SS at 30 Hz
  gives 2,057,130: 1.9 ms of margin.
- **Fix:** `forger()` derives the rest from the scene's weapon it already holds (`Behaviour::script(&scene, ...)`):
  a const fn `fire_rest_at(tick_hz, interval_us)` = ceil(interval_us x tick_hz / 10^6), pinned at 30 and 60 Hz in a
  unit test (3 and 5), and `forger(7, ...)` refuses (returns `None`, a usage error in the lab) a weapon under two ticks;
  the doc corrected. About 12-20 lines.

**NC-U4F2-3, Medium: F-A, the lag-switch suite's movement check cannot fail.**
- `tick.v.lagswitch_excess_move_um` is the tick's movement less 8 ticks at full speed (`W/crates/lab/src/logs.rs:699-706`).
  The lag client wraps the run's behaviour (`W/crates/lab/src/scenario.rs:1265-1267`), now the standing steady script,
  so the series reads -8 x 20 cm on every tick: no hold length and no cap breach can fail "lag switch: no tick moves
  further than the catch-up cap allows" (marks.json:5062-5073). TE-7 already had it unable to fail at 200 ms; now it
  cannot fail at all, and f20's assert (scenario.rs:4225) passes by construction.
- **What still carries the cap:** kind 10's forged check, with PM-0001-21/no-catchup-cap in the forged suite (P:2553,
  P:2562), and the report series `lagswitch_cmds_count` (max at most 8). So -21's coverage of the cap holds; the
  lag-switch parts' own movement verdict is a PASS that is no evidence.
- **Fix (i), preferred:** slot 0 of the lag-switch suite walks the fast clock's lane (bots/src/lib.rs:81-88, :633-645:
  a band no box, door, wall or vehicle part reaches, so the vehicle carry that sank route (b) cannot meet it). Closed
  loop is allowed there: the suite has no twin, and Q1's open-loop rule binds the forged and clamp suites' scripts.
  `lane()` reads its state from `self.fast` (:634), so a lag client needs that state outside `FastClock`: a small bots
  change the advisor rules against Q1 point 1 ("the Lane arm stays"). About 20-35 lines. Runs: the P1 lag-switch run
  under A and B, 610 s, seeds 1 to 3 (in a building 0 on every slot, PM-0001-2 passing, the movement series now near
  -2 x 20 cm on the burst ticks); and p19's 400 ms hold under PM-0001-21/no-catchup-cap at P1, option A: excess above
  0 (the check's power shown at last).
- **Fix (ii), the fallback:** a record line in -21's report that the movement check cannot fail by construction and the
  cap's evidence is kind 10's. Keeping a judged check that cannot fail is the advisor's to weigh under TEST-FIRST rule 2;
  if he reads it as a bar loosened, Mustafa's.

**NC-U4F2-4, Medium: F-E's plant 9 most likely never sent the honest command under option A, so F-E holds for option B
only.**
- Under A a tick with no command runs nothing for that slot: no re-run (room.rs:1085-1087), no idle step in the sim
  (world.rs:2181-2191). A twin that sends the refused command's honest content runs one more command than the forged
  run; that advances the weapon clock and moves `wait` and `ammo`, which the hash reads (world.rs:2014-2015). So a
  correct plant should fail f16_option_a_kind_{1,2,4,5,6} by "the first state hash that differs". It passed all 12
  (`.gate/u4f/u4f2-plant9.log:20-34`). Likely cause: forge_input's arm for kinds 1, 2, 4, 5 and 6 has no twin guard
  (P:144-147), and `f.due` is set from `!skip` (P:446), so a plant that only drops the skip makes the twin forge the
  same command. Under B the empty tick re-runs the last command (input.rs:473), which the standing script makes equal to
  the honest one: there F-E's reading holds, and the attribution's "no command of (slot, seq) runs" rule (P:2737-2748)
  still covers a refused command's content running.
- **Fix:** re-run plant 9 with the twin's `f.due` also false (the honest command really sent): expected f16_option_a
  kinds 1, 2, 4, 5, 6 FAIL by name, option B PASS; record F-E as option B's limit only. Add `if !twin` to that arm (as
  kinds 3, 7 and 11 have) so a twin can never forge. About 2 lines, plus the run.

**NC-U4F2-5, Low: the forgers' fixed lead is not recorded, and `--lead` reaches it unguarded.**
- The forgers stamp at `s.lead` (scenario.rs:1230; `lead_for(P0)` = 2, :2741-2742), but with the controller on the
  header writes no lead (scenario.rs:1988-1995) and their `lead_now_count` reads -1 (lab.jsonl:10). Kind 10's A burst is
  shaped for P0's lead of 2 (stamps about t-5 to t+10 against the 12-tick bound: u4f-brief.md:526): at `--lead 5` its
  top stamps pass the bound and are refused under kind 1, so kind 10 reads unrejected.
- It also relies on its two same-instant datagrams arriving in order, which holds at P0 only (zero delay: the reorder
  step finds nothing to swap, `W/crates/l1/src/lib.rs:15-22`, :1167-1173); at P1 the step can put the forged half
  first, and the room then accepts it. The suite is P0 by U4F-Q11 (ii) (a), so this is a record line.
- **Fix:** write `params.forger_lead_ticks_count` on a forged run (with its row in U4f2's SERIES subsection), make
  `--lead` with `--suite forged` a usage error, and name the P0 order assumption in the forge_every row. About 8 lines.

**NC-U4F2-6, Low: `twin()`'s doc (P:295-298) says the room-whole kinds' refused commands are skipped "at the next
step"; the code skips them at the due step (P:445, P:455-458); only kind 10's skips wait for the next step (P:504-507,
P:416-418).** Fix: the doc names the two timings. About 2 lines.

**Checked, no finding:**
- **ADR 0001 and the plan (rev 3.3):** each forged kind is refused by the server's own state (the room's checks for 1-6
  and 10, the room's ack ring for 8, the pump for 9 and 11, the sim's weapon for 7), counted (the room's and pump's
  counters, the sim's reports) and attributed (`RefusalRecord { slot, seq, kind }`, P:2319-2326; `refusals()`,
  P:2346-2349; `event.forged_kind`, P:2755-2768). Nothing a client sends is trusted for a verdict: the attribution reads
  the room's records, the sim's reports and the `TickInput`, never a client's claim. `run_one` keeps every command field
  but the slot (room.rs:1380-1386), so kind 3's copy comparison (P:2716) works on the real form.
- **C-U4F2-3:** `CommandRefusal`, the struct records, the sim's own `Refusal` left alone; forged.rs keeps the room's
  kinds and the sim's apart (P:2700-2710), and C-U4F2-10's case bites (plant 1b).
- **Options A and B:** both run every kind (f16's 24, f22 per option). Kind 10's two shapes fit the two caps (A: the
  per-tick cap, input.rs:322; B: the full buffer, BITE_HELD); the long A run reads kind 10 unrejected 0 over 1,656
  items. W is `CMD_CATCHUP` = 8 (input.rs:13, :175), a command count, so the A burst's oldest stamps (about t-5) stay
  inside it at either tick rate.
- **`HOLD_STEPS` against carry 39:** a real count identity, not a renamed tick count: the burst's 8 accepted commands
  over the episode's 8 steps (6 + 2) leave the forger's lead unchanged, at one step a tick at any rate; W (8) and the
  lead bound (12 ticks at 30 Hz, 24 at 60 Hz) both hold the burst at 60 Hz too. The rename is right; `FIRE_REST`
  (NC-U4F2-2) is the one in this file that is really a tick count.
- **C-U4F2-5's route (c) against Q1:** (b) was tried and shown not to hold (a 1/32-length circle still enters,
  u4f2-b7-walk-fix.log); the builder's reading fits the code (`collide` sets a walking soldier at a part's face whatever
  the step, world.rs:1260-1268, :1283-1291; a still soldier skips the test, :1253-1255). (c) changes only Steady's
  `move_xy` (P:524-527), the content stays a function of seed, slot and step, the plant (today's walk) bites (plant 8,
  Skips 8,433), and the full test reads 0 in all 12 (seed, pattern) pairs. So (c) meets Q1 as ruled. Its costs, which
  the ruling could not weigh: F-B's standing lines of fire (NC-U4F2-1), F-A (NC-U4F2-3) and F-E under option B
  (NC-U4F2-4). The fast clock's honest reference is arithmetic (scenario.rs:1038-1046), so standing does not touch it.
- **The fixed-lead endpoints:** they keep kinds 2 and 5's "previous command accepted" (C-U4F2-1) and each forger's count
  of commands, match 2.6's table ("at P0's lead of 2"), and touch no check: -21's checks read the room's refusals, and
  the controller is -20's. The record gap is NC-U4F2-5.
- **Bounds and determinism:** the forger's queues are drained each step or capped (`STAMPS_KEPT`); forged.rs holds one
  entry per item of the run, in BTreeMap/BTreeSet; no clock read (the Ping's `sent_ns` is the step's injected time); the
  room's per-tick records stay capped at 4,096 with the drop counted and logged (SEC-1).
- **F-C (the test seat's), for the record:** under A the forged suite loses about 6 shots a second to kinds 1, 2, 4, 5
  and 6 (a refused command leaves its tick with none), about 3 to kind 7's rests and reloads, and about 4-5 to dead
  soldiers: about 680-687 against the 665 floor, in line with 677 and 685. NC-U4F2-1 (b) gives back about 1.6.

### Block 3 (DONE): findings and verdict

| Id | Sev | Where | What goes wrong | Fix (size) |
|---|---|---|---|---|
| NC-U4F2-1 | High | W/crates/sim/src/world.rs:1351-1355, :1192-1203; P:2705-2709, P:2736 | F-B: kind-7 items run while the standing forger is dead (or across a revive) get no record, so -21's forged check fails on judged runs and the 11 forged bites read FAILS whatever they reach; a reload item after a revive also breaks the twin hash (rare) | (a) leave such items out, counted apart (`dead_left_out_count`), from the server's World state; (b) `RELOAD_ITEMS` about 400 (30-45 + 3 lines, one f33 case and plant, one SERIES row; the advisor's reading, then his readings list) |
| NC-U4F2-2 | Medium | P:11-17 (bots lib.rs `FIRE_REST`) | a tick count as a literal (carry 39): at 60 Hz about a quarter of kind 7's fire items go unrejected; the doc's "more than a tick" should be "at least two ticks" | `fire_rest_at(tick_hz, interval_us)` from the scene's weapon, pinned at 30 and 60 Hz; refuse a weapon under two ticks (12-20) |
| NC-U4F2-3 | Medium | W/crates/lab/src/scenario.rs:1265-1267; logs.rs:699-706 | F-A: the lag client stands, so the lag movement check cannot fail at any hold | (i) the lag client on the fast clock's lane, with runs (20-35; the advisor rules it against Q1 point 1), else (ii) a record line |
| NC-U4F2-4 | Medium | P:144-147, P:446; .gate/u4f/u4f2-plant9.log:20-34 | F-E's plant 9 likely made the twin forge, not send the honest command; under option A a correct plant should fail f16, so F-E holds for option B only | re-run plant 9 with `f.due` false; `if !twin` on the arm (2 + the run) |
| NC-U4F2-5 | Low | scenario.rs:1230, :1988-1995 | the forgers' fixed lead is unrecorded and `--lead` can push kind 10's A burst past the bound; the burst's in-order arrival holds at P0 only | `params.forger_lead_ticks_count` and its row; `--lead` with `--suite forged` a usage error; a record line (about 8) |
| NC-U4F2-6 | Low | P:295-298 | `twin()`'s doc puts the room-whole kinds' skips at the next step; the code skips at the due step | the doc names both timings (2) |

**Verdict: REQUEST CHANGES** (NC-U4F2-1 is High and blocks -21's set as built; the rest is fix-pass work).

**Mustafa's (TEST-FIRST), named:** NC-U4F2-1 (a) is a reading of "forged command" for kind 7, like A1's of "moves":
the advisor rules it and it joins the readings PR's list for him; if the advisor reads it as changing what
SERIES.md:158's existing `sent_count` row means, the fix pass stops for him. NC-U4F2-3 (ii), keeping a judged check that
cannot fail, is the advisor's under rule 2 and his if read as a bar loosened. No pass mark, judge code, marks.json text,
money or main is touched by any fix proposed here.
