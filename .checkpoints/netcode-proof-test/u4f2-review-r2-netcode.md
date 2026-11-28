# U4f2 review round 2: the netcode seat (netcode-architect), 2026-11-21

Round 2 (the last, NPT-TRIM history.md:7561-7593: netcode and test seats; the rust seat's FP-3 and bots items split
to me). Scope: the fix pass's diff `git diff 3441d47f…ca 7c362a63…a` (7 files, +638 -88). Read only. (No Edit tool in
this agent: the file is rewritten whole with Write after each block.)

## Verdict: APPROVE WITH FINDINGS (three Lows; none triggers a push under NPT-TRIM; block 5)

## Plan
1. Read the ruling's blocks 4-5 and FP table (done), part A's ruling (done), his F-B-COUNT words and NPT-TRIM (done),
   the build record's Blocks 9-10 (done), my round-1 file (done).
2. The diff's stat, then each file's hunks: bots lib.rs (FP-1 `forge_step`, FP-3 `fire_rest_at`/`forger`, F-G, FP-4
   `lane_west`/`lane_walk`, FP-5, FP-8 `checked_mul`, FP-2 `kind_7_left_out`, T-K7, fp3, fp5), forged.rs (FP-2's
   window, FP-6, FP-8), scenario.rs (`dead_before`, FP-4 wiring, FP-7, FP-8 usage, f16/f20), room.rs, SERIES.md, the
   two scripts.
3. FP-2's window against his words: when `dead_before` is sampled relative to the command phase, `recent`'s first-run
   rule, option B's re-run, d-1 never ran, the counts, "on every run".
4. FP-1's construction (reload half), FP-3 const fn and pins, F-G, FP-4 lane and the fast clock (R5's proof), FP-5, the
   bots refactor.
5. The departure: bots lib.rs `kind_7_left_out` and T-K7 lines; can any non-forged line move.
6. The evidence logs (R2, R3, lag proof, fp2 tests/plant) against the figures claimed.
7. Findings table and verdict.

## Blocks
(paths: `W` = `/home/user/Shamble-Frontlines/.claude/worktrees/agent-aa62d31b6fa5b9807/spikes/netcode-proof`; `S` =
`W/crates/sim/src/world.rs`; `B` = `W/crates/bots/src/lib.rs`; `F` = `W/crates/lab/src/forged.rs`; `L` =
`W/crates/lab/src/scenario.rs`)

### Block 1 (DONE): FP-2's window against his words

His words (history.md:7548): "Leave kind 7's presses out of that count when the server's own state shows its soldier
dead as that command or the one before it ran; report the number left out beside the check on every run."

**1. "The server's own state ... as that command ran" = `dead_before`, exactly, in this sim.**
- The tick's order (S:2151-2207): phase 0 `apply_inputs` (spawns, teleports), movers, `pre` snapshots, then the
  command phase (S:2181-2191: every `run_command`), then `run_shots`, projectiles, effects, `run_respawns`,
  `write_records`. A command never changes any soldier's `alive`: a shot is only queued in `self.fired` (S:1474-1485)
  and lands in `run_shots` (S:1613-1658, `damage` S:1683-1713), a throw's damage in `run_projectiles` (S:1716-1731).
  So every command of a tick runs against the alive state its tick began with; "dead as that command ran" is one bit
  per tick, the state entering its command phase.
- What could change that bit between one tick's records and the next tick's command phase is phase 0's spawns
  (S:1152-1161: `revive`). The room fills spawns only from `Room::script` (room.rs:806-817, :1074-1077), and the lab's
  only callers are tests (logs.rs:1058, L:3280; precheck.rs is another path). So in a scenario run the records after
  tick T are the state entering T+1's commands.
- `dead_before` is the forger slots whose Soldier record reads health 0 after the tick (L:1469-1473), handed to the
  next tick's `Attribution::tick` (L:1468). Health 0 is the dead bit: `damage` sets `alive = false` exactly when health
  reaches 0 (S:1698-1702), `revive` sets health to the scene's (S:1196), every soldier starts alive at full health
  (S:755-758), and SS has no plates (the only alive-with-any-health exception, S:1694). The record writes `s.health`
  as an integer field (S:1848). T-K7's `soldiers()` asserts flags bit 3 = (health 0) on every record it reads
  (B:2249-2260). The records are the room's own World (`server.room.sim()`, L:1426), never a client's copy. So
  `dead_before` is "the server's own state", sampled at the moment his words name.

**2. The window (F:99-114).** At each kind-7 command's first run (re-runs skipped, F:102-104), the item is left out
when `kind_7_left_out(dead now, d - 1's first run)` (B:180-182: `dead_at_d || dead_before == Some(true)`), with `None`
when seq - 1 has no recorded run (then d alone decides, as the ruling's row and the main session's reading say).
- **d - 1 is always found if it ran.** The room refuses a command not newer than the last (kind 2), so slot 6's first
  runs come in sequence order; d - 1's first run is the last entry of `recent` when d runs (one tick before at P0, or
  earlier in the same tick under a catch-up: `ran` is canonical, slot then tick ascending, S:2152-2156). `RECENT_RUNS`
  = 4 (F:52) is more than enough; the bound keeps memory flat.
- **The first run is the right run.** A re-run (option B's empty tick) repeats a command whose edges were claimed at
  its first run (S:1348-1351), so it carries no press; whether the soldier died before the re-run says nothing about
  the press. Counting from the first run follows his "as that command ... ran" for the run that carried the press.
- **Every death-made unrecorded item is inside the window** (rechecked against round 1's cases after FP-1 and F-G):
  d run dead (no record, S:1353): out. Fire item with d - 1 dead and d alive (d holds fire since `every` > rest, so
  the revived weapon fires by the held trigger as the twin's does): out. Reload item with d - 1 dead, d alive (the
  full magazine refuses d's edge, a Reload record): out anyway (harmless over-inclusion his words make). d - 2 dead,
  d - 1 the first live command: the fire item's honest press fires and d is refused (Cooldown); the reload item
  (FP-1) has d - 1's fire released, d's edge refused by the full magazine (Reload), both Worlds fire the same held
  shot on d: recorded, not out. A revive between d - 1's run and d's needs a death and a 30-tick respawn (S:1701)
  inside one command step, impossible at P0 with slot 6's 0 missing ticks (R2). Nothing else stops a kind-7 record
  for a death reason (block 2's NC-U4F2-R2-3 is a construction gap that is not about deaths).
- **The counts.** `left_out` holds only sequences in `items` (F:107: `contains_key` at the run; `forged()` is called at
  the client step that made the command, before its datagram is sent, L:1691-1694, so the item is known before it
  runs; items after the cutoff are never in `items`). `sent_count` = items less left-out (F:184; no underflow, a
  subset), `unrejected` skips left-out items (F:153), `dead_left_out_count` on every record, 0 for kinds other than 7
  (F:188; the loop is gated on `f.kind == 7`, F:101). marks.json's -21 checks read `sent_count` (min >= 1,000,
  marks.json:4935-4944; count >= 11, :4947-4956) and `unrejected_count` (max == 0, :4958-4968): both narrowed, as his
  option (a) said ("the 1,000-per-kind check still applies to what is left"). `measured_sent_count` is left whole: no
  check reads it, and the ruling narrowed only the two.
- **The twin** has no attribution (L:1368) and needs none: the twin check still compares every tick (R2: 18,300 of
  18,300 equal on both options).

**3. "Report the number left out beside the check on every run".** Met at the record: every `forged_kind` record of
every forged run carries `dead_left_out_count` (F:188), with its SERIES row (SERIES.md, U4f2's subsection: no existing
row changed). Not met mechanically at the report: neither the lab's summary line (L:2464-2481) nor c-u4f-runs.mjs's
per-run lines carry it, and the judge prints no unread series; -21's set report must read it from 6 forged measured
logs, 11 bites and 4 r2 rows by hand or the scratch reader (u4f2-read.mjs). -> NC-U4F2-R2-1 (Low).

**4. Tests of the window.** fp2's f33 case (F:446-464, the assertion F:463) covers dead at d, dead at d - 1 only, alive
with and without a record, and d - 1 never run: the five cases the ruling's row asks. T-K7 applies the same function
to its World's flags (B:2080-2106; `ran_dead[j]` sampled before the step's commands, a test spawn counted alive,
B:2010, matching phase 0's order; the sim's own respawn: killed at tick k, `run_respawns` already takes one off at k,
S:1809, so commands k+1..k+29 run dead and k+30 alive, as T-K7's `start` arithmetic assumes). One plant on
`kind_7_left_out` fails both (block 4). Not covered by any case: the first-run rule (F:102-104, option B's re-run). If
it were dropped, a re-run of d (B's empty tick, d + 1 missing) while the soldier is dead would put a recorded item into
`left_out`: never a false pass (it shrinks `sent_count` and grows `dead_left_out_count`), unreached at P0 (0 missing
ticks on slot 6 in R2 B). `RECENT_RUNS`' doc (F:51) says the two commands "are at most a tick apart"; the ring depends
on run order, not distance (true under loss too). -> NC-U4F2-R2-2 (Low).

### Block 2 (DONE): FP-1, FP-3 with F-G, FP-4, FP-5, the bots refactor

**FP-1 (the reload half, by construction): holds.** `rest` (B:944) now releases fire on every item's d - 1 (`k == 1`)
and on the rest before a fire item; no twin guard, so the twin's d - 1 is the same. Every position of a revive around
a reload item gives a record or a left-out: revive two or more commands before d - 1, the soldier fires on a held
command, so d - 1's honest edge starts a reload and refuses d (Reload); d - 1 the first live command, the full magazine
refuses both edges (S:1427) and nothing fires on d - 1, so d is refused too and both Worlds fire the same held shot on
d; d - 1 or d dead, FP-2 leaves it out. Reload items stay 40 items apart, at least 160 commands at 30 Hz (every >= 4)
and 240 at 60 Hz (every >= 6), against a reload of 60 and 120 commands, so a reload never overlaps the next reload
item. T-K7 reaches the case ("reload after a revive 1"), and its plant (fire held on d - 1) fails it by name with the
unrecorded reload and the hash at tick 74 (build record :284-288).

**FP-3 and F-G: the arithmetic is right; the pins hold.**
- `fire_rest_at` (B:166-173): `interval_us as u64 * tick_hz as u64` cannot overflow (u32 x u32 into u64);
  `u64::div_ceil` is const-stable; `None` below 2,000,000 units. The proof of the refusal: after the honest shot on
  d - 1 the wait is w + cost with w in [1 - 10^6, 0] (S:1416 clamps it there), and d takes 10^6 off before its check,
  so d's wait is at least cost - 2 x 10^6 + 1, above 0 exactly when cost >= 2 x 10^6. Readiness on d - 1 after `rest`
  released commands: the wait is at most cost - rest x 10^6 <= 0 since rest = ceil(cost / 10^6). Pins: SS 3 at 30 Hz
  (2,057,130) and 5 at 60 Hz (4,114,260); 66,666 µs `None` (1,999,980), 66,667 µs 3 (2,000,010) (B:1634-1636). The rate
  is `npt_sim::TICK_HZ`, the sim's own constant (carry 39: a 60 Hz sim build moves it with no edit).
- F-G (B:589-593): `every > r`, so d is never inside the next item's rest and holds fire: 2 and 3 refused, 4 taken at
  30 Hz (B:1637-1640); the no-rest weapon refuses every interval (B:1641-1643). The plant (`every >= r`) fails the pin
  (fg-plant.log). The lab's usage error (L:1277-1291) can only be reached by kind 7: kinds 1-6 take every interval the
  parser gives (2 to 1,000,000), and the overflow arm is unreachable under the parser's bound.
- The sim reads the soldier's `fire_interval` (S:1415), which an attribute change can move; the bot reads the scene's
  weapon. No forged run scripts an attribute change (the room's `script` has test callers only), so they agree.
- **A gap FP-3's general claim still has, not about deaths -> NC-U4F2-R2-3 (Low).** A fire item's honest press on
  d - 1 does not fire when a reload is still running there; if that reload ends on d itself (`reload_left` reaches 0 at
  d's decrement, S:1417-1422), d finds the weapon ready (the wait ran down during the reload) and its forged edge fires
  with no record; the twin's d fires the same shot by the held trigger, so the twin check stays equal and only
  `unrejected_count` rises. A reload started on a reload item's d_r - 1 ends on command d_r - 1 + L, L =
  reload_ms x TICK_HZ / 1000 (60 at 30 Hz, 120 at 60 Hz, from `reload_left = reload_ms x 1000 x TICK_HZ`, S:1433; SS's
  reload 2,000 ms, scene.rs:68). A fire item lands there when `every` divides L - 1: **59 at 30 Hz; 7, 17 and 119 at
  60 Hz**, all accepted by `forger(7, …)` today. About one false unrejected item per reload item (40 x every
  commands): about 7 on a 610 s run at 59. The judged 12 divides neither 59 nor 119, so no judged run reaches it; it is
  a false FAIL, never a false pass: the same class as F-G, which the part-A ruling fixed. (Read from the code; no run
  shows it. The run that would: `npt-lab scenario … --suite forged --forge-every 59 --cmd-timing A --seed 1 --warmup-s
  10 --measured-s 600`, expected kind 7 `unrejected_count` about 7 with `dead_left_out_count` beside it.)

**FP-4: the lane and the fast clock.** `west` moved from `Fast` to `Behaviour::lane_west` (B:148, :686-690), `false` at
construction as `Fast::default()` gave it; `lane()` no longer needs `self.fast` (B:676-679). `Kind::Lane` is built only
by `fast_clock` (B:660) and `lane_walk` (B:697), so the only walk that changes is the new lag client's; the fast clock's
decision is the same function of the same state. R5's proof: both fast-clock lines' room logs equal hand-a's per tick,
"equal: 450 ticks" A and B (r5-bites.log:69-74), and the reader compares every tick record's (t, h) over the longer of
the two lists (u4f2-read.mjs:16-23, in the builder's worktree's notes folder), with an `h` on every tick (481 of 481 in
the lag-switch room log, by count): not vacuous. Since hand-a's tree predates the whole fix pass, the equality also
shows that FP-1, FP-3, FP-5 to FP-8 and F-G moved nothing in a fast-clock run. The lag client walks the lane (closed
loop: the suite has no twin, so Q1's open-loop rule is not in play), in a building 0 on every lag-switch run; f20
asserts a burst tick moves it (L:4273-4286) and its plant (the standing script) fails by name; R4 shows the check's
power (excess 600,000 µm on 3 ticks with the cap removed). No new rate-derived literal for the gate's tick-count rule.

**FP-5: right.** The 1, 2, 4, 5, 6 arm is guarded `if !twin` (B:311); a twin of those kinds falls to `_ => {}` (no
other arm matches its kind, B:315-349), and `forge_step` already skips the twin's refused command (B:961, :972-976).
fp5 compares the twin's sent commands with the forger's honest ones less the forged, by sequence (B:2184-2196). Plant 9
in its right form fails fp5 and f16 under both options; the part-A ruling's reading (F-E closed under both, the shot's
`seq` in the hashed events) fits S:1618-1621.

**The bots refactor: clean.** `sides_at` keeps `sides` at (3, 300) for the other kinds; `Forge::rest` is 0 for kinds
other than 7, so `(1..=0)` is empty and `ahead` is gated on kind 7 anyway (B:942-944); `Behaviour::new` computes
`fire_rest` with no RNG draw (the stream is seeded before it, B:531-541), so no behaviour's commands move;
`checked_mul(BURST_KEPT)` (B:588) covers `catchup_step`'s `every x burst_refused` (at most 8, B:997).

### Block 3 (DONE): the departure (bots lib.rs in FP-2's diff)

**Asked of me: can any non-forged line be affected by the bots lib.rs change? No.**
- FP-2's bots part is two things: `pub const fn kind_7_left_out` (B:180-182) and T-K7's lines (its classification by
  the function, its print and messages, its doc). The function has exactly two callers, F:106 (the attribution) and
  B:2082 (T-K7, inside `#[cfg(test)] mod tests`), by grep over every crate. No `Behaviour` path, script, forger,
  lane walk, endpoint, codec, RNG draw or iteration order calls or reads it, and T-K7 is in no release build.
- The attribution runs only when `attribution` is `Some`: a forged-suite run that is not a twin (L:1368-1369). In every
  non-forged line (clamp A/B and twins, fast-clock A/B, lag-switch A/B, refuse-not-clamp, not-older-than, the fast-
  clock proof, R4's bite) neither `Attribution::tick` nor the function is reached. FP-2's scenario.rs part on those
  runs is one empty `Vec` declared (L:1373) and never written or read (its fill is inside the `Some` arm,
  L:1465-1474); f16's added assertion is test code.
- The evidence agrees: the lag-switch A line after FP-2 (r5-lagproof.log:20-23: fate log 0576dea2…, "equal: 481
  ticks") equals its early run, and every forged line of R5 also kept its early fate log (build record :535-536), so
  FP-2 moved no datagram even where it runs.
- **So R4 and R5's non-forged lines need not re-run.** The ruling's letter ("only forged.rs, scenario.rs's attribution
  wiring and SERIES.md") was a proxy for "no code a non-forged line runs"; the extra file meets that purpose, and the
  builder said so before building (build record :498-501). Accepting the departure is the advisor's; my reading
  supports it. One plant on the shared function failing both the f33 case and T-K7 (fp2-plant.log:27-30, :69-74) is a
  better test than two copies of the rule would give.

### Block 4 (DONE): the evidence against the figures claimed
- **FP-2's plant** (fp2-plant.log): the f33 case fails by name at F:463 ([5, 3, 0] against [3, 1, 2]); T-K7 fails at
  B:2101 with items (12, 11), (576, 575), (588, 587) unrecorded outside the (ignored) window, and the first differing
  hash `None`: the twin is untouched by the dead items, as block 1 reads.
- **R2** (r2.log:37-70): A and B every kind `unrejected_count` 0; kind 7 `dead_left_out_count` 18 (A) and 8 (B), sent
  1,505 and 1,515 (1,523 less the left-out); every other kind 0 left out; `counted_count` 1,523 for 8, 9, 11; dropped
  0; slot 6 commands 18,299 of 18,300 ticks, `commands_repeat_count` 0, `steps_without_command_count` 0, measured
  ticks with its command missing 0 of 18,000 under B, so option B re-ran nothing of slot 6's in the measured phase;
  "equal: 18300 ticks" both options; shots 685 and 691. The 18 and 8 equal R2-pre's unrejected counts (build record
  :470), which is the ruling's proof that every one was a dead-window item.
- **R3** (r3.log:23, :29): kind 1 unrejected 1,523 of 1,523, kind 7 unrejected 0 with 10 left out (sent 1,513): the bite
  reaches only its own kind at length now; shots 673 (above the 670 line by 3: the test seat's figure to weigh).
- **The lag-switch proof** (r5-lagproof.log): the tree diff 4e15fbde… (FP-2 in), the same arguments as the early run
  but `--out`, fate log equal, 481 tick hashes equal.
- Read, not re-derived: the judge's verdicts (I did not open verdict.json; the build record's lines are cited as the
  builder's).

### Block 5 (DONE): findings and verdict

| Id | Sev | Where | What goes wrong | Fix (size) |
|---|---|---|---|---|
| NC-U4F2-R2-1 | Low | L:2464-2481 (summary line); ops/runs/c-u4f-runs.mjs:163 (the per-run lines) | his "report the number left out beside the check on every run" is met at the record only: nothing carries `dead_left_out_count` into -21's set report, which must gather it from about 21 forged logs by hand | no code: -21's report plan names its reader (u4f2-read.mjs `forged`, or the lab logs) and prints `dead_left_out_count` per forged run beside "every forged command rejected and counted"; optional follow-up: one summary-line clause on forged runs (about 5 lines, the logs.rs:1121 test kept) |
| NC-U4F2-R2-2 | Low | F:102-104 (first-run rule), F:51 (`RECENT_RUNS` doc) | no case covers option B's re-run of an item while dead (the rule is right but untested; dropping it would only over-count left-outs, never pass falsely); the doc's "at most a tick apart" is not the property the ring needs | a follow-up f33 case: item d first run alive with a record, re-run dead, still counted rejected and not left out (about 10 lines, with its plant); the doc reads "the item's command is the next first run after the one before it" |
| NC-U4F2-R2-3 | Low | B:589-593 (`forger`), B:944 | kind 7 still accepts intervals whose fire item lands on a reload's last command (every dividing L - 1: 59 at 30 Hz; 7, 17, 119 at 60 Hz): d - 1's honest press is refused by the running reload, d's forged edge fires unrecorded; a false FAIL of -21's check, never a false pass, the twin equal; the judged 12 is not reached | `forger(7, …)` also refuses `every` dividing L - 1, L from `scene.weapon().reload_ms` and `TICK_HZ` (a second const fn pinned at 30 and 60 Hz, plant: the refusal removed) (about 10-15 lines), or a record line in -21's report beside F-G |

**Verdict: APPROVE WITH FINDINGS.** FP-2 builds his F-B-COUNT (a) as worded: the dead bit is the server World's own,
taken at the moment each command runs (nothing in a tick's command phase can change it), d - 1 never run leaves d alone
deciding, every record carries `dead_left_out_count`, and R2 shows every one of round 1's unrejected items was a
dead-window item, with the twin equal on both options. FP-1, FP-3 with F-G, FP-4 and FP-5 meet their acceptance; the
fast clock is unchanged (450 ticks equal, A and B). Round 1's High (F-B, NC-U4F2-1) is closed. The three findings are
Low: under NPT-TRIM none triggers a push, and no third round is called for.

**The departure:** no non-forged line can be affected (block 3); R4 and R5's non-forged lines stand without a re-run.

**Mustafa's, named:** nothing new. FP-2's narrowing of `sent_count` and `unrejected_count` for kind 7 is his F-B-COUNT
(a), recorded; -21's report owes his "beside the check on every run" (NC-U4F2-R2-1) and the reading's line with who
reviewed it. No pass mark, judge code, marks.json text, money or main is touched by this pass or by any fix proposed
here.
