# U4f2 review round 1: the netcode seat (netcode-architect), 2026-11-20

(No Edit tool in this agent: the file is rewritten whole with Write after each block, as the brief allows.)

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

### Block 1 (DONE): F-B, its cause in the code, and a fix inside the plan

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
- (a) **Count only items the sim could judge.** 2.6's kind-7 row defines the item as "a fire or reload press edge
  where the sim refuses it" (u4f-brief.md:525); an edge into a dead soldier is not one. Each room tick the lab gives
  the attribution each forger's soldier state from the server's own World (alive and generation at the tick's start;
  the lab already reads Soldier records for the movement series, rev 3.3's A1, u4f-brief.md:540-546). A kind-7 item is
  left out of `sent_count` and `unrejected_count` when its forger was dead at the start of the tick that ran d or d-1,
  or its generation changed between them (a revive), and is counted apart in a new report-only field
  `event.forged_kind.dead_left_out_count` (one new row in U4f2's SERIES.md subsection, P:2419-2425). Size: about 30-45
  lines (forged.rs, scenario.rs's tick call), one f33 case on hand-made records (neg: a dead-window item with no
  record is left out and counted apart; a live item with no record still counts unrejected), its plant (the dead
  window ignored: the case fails by name), one SERIES row.
- (b) **Shrink the twin hazard and the reload shot loss:** `RELOAD_ITEMS` (P:16) from 40 to about 400 (3-4 reload
  items a judged run, the hazard about ten times smaller, about 1.6 shots a second back on slot 6); f15's
  `taken[1] >= 2` (P:806) becomes `>= 1` at its 300 steps. About 3 lines.
- **Whose:** (a) reads "forged command" for kind 7 the way A1 reads "moves": the advisor rules it, and like A1 it joins
  the readings PR's list for him (plan :971-974). If the advisor reads it as changing what an existing SERIES row
  means (`sent_count`, SERIES.md:158), it is Mustafa's under TEST-FIRST and the fix pass stops for him.
- **Not inside the plan:** a sim record for a dead soldier's edges (crates/sim is shut, and kind 7's own-kind rule
  would gain a kind: his); a leash or a dead-bit gate in the forger (closed loop, against u4-build-plan.md:411-412 and
  2.12's rule, Q1); new aims for Steady (Q1 point 1 keeps the aim).

**Runs for the fix pass** (the forged run is deterministic, so the same 610 s run reproduces the 18):
- Before the fix (diagnostic only, not committed): `npt-lab scenario ... --suite forged --forge-every 12 --cmd-timing A
  --seed 1 --warmup-s 10 --measured-s 600` with a count of kind-7 items whose d or d-1 tick had slot 6 dead or
  revived. Expected: 18 (all of kind 7's unrejected), which proves the cause.
- After the fix, the same run: kind 7 `unrejected_count` 0, `dead_left_out_count` about 18 or a little more,
  `sent_count` at least 1,000; the same under `--cmd-timing B` (with slot 6's `commands_rerun_count` 0, so option B's
  re-run cannot move the weapon clock between d-1 and d); and one no-* bite other than no-cooldown at 600 s (for
  example no-aim-bounds) whose only unrejected kind is its own (kind 6), showing the bites' power restored.
