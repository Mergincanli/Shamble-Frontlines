# U4f-F1 review: security seat (scoped by advisor condition 6 to hostile client input)

Reviewer: security-auditor. Date: 2026-11-20. Diff: `.checkpoints/netcode-proof-test/wip/u4f-f1-build.patch` (base cb7e374),
worktree `.claude/worktrees/agent-ad5f72ed146da5f93`. Read-only seat: no cargo, no scripts, nothing under spikes/ or .gate/.
(This agent has no Edit tool: the file is rewritten whole with Write at each block, as the brief allows.)

## Plan
1. Read the patch, the plan (rev 1.1), advisor-u4f-f1-plan.md conditions 1-9 (esp. 6) and Blocks 1-3, the build record.
2. Bounds: every new loop's trip count as a function of client-chosen bytes; cost per command (CPU, allocation) at the
   worst case; the grenade-pool rescan (rust seat RR-F1-1).
3. A session's first command (ADV-U4FF1-3): what a hostile client buys with counters chosen freely on its first command
   (up to 255 edges per button), and the honest rejoin case (RR-F1-3).
4. The path with PM-0001-21 / no-press-edge bitten (room/src/input.rs:293): what still holds without the check.
5. Room counts (seen/lost/doubled) as a client-controlled number: overflow, log/telemetry inflation.
6. Panic / overflow paths reachable from input; wasm32 vs native.
7. Findings, anti-cheat coverage rows touched, verdict.

## Progress
- [x] 1 read  - [x] 2 bounds  - [x] 3 first command  - [x] 4 bitten path  - [x] 5 counts  - [x] 6 panics  - [x] 7 verdict

## Block 1: read (done)
- Worktree at cb7e374, 5 files modified; `git diff cb7e374 -- spikes/` sha256 37d87a10bed52b81..., equal to
  wip/u4f-f1-build.patch (checked). +205 -24. Line numbers below are the worktree's.
- Read: the panel brief; u4f-f1-plan.md rev 1.1; advisor-u4f-f1-plan.md whole (conditions 1-9; Block 1's first-command
  and loop-bound reading :45-55); u4f-f1-build.md whole; the rust seat's file whole (RR-F1-1..4); my own U4f review
  (u4f-review-security.md) and memory (nothing on press edges).
- **My U4f line that F1 overturns:** u4f-review-security.md:31-32 ("A fresh session's first command has no room press
  compare (accepted None): no finding, since the sim takes one edge per changed counter per command") and its coverage
  row "Extra press edges ... first command of a session uncompared (bounded by the sim's one-edge rule)" (:108). That
  bound was the old one-edge rule; F1 removes it by design. So the first-command gap is re-rated here, not "no finding".

## Block 2: the loops' bounds and cost (done)
- The edges per button per command are `Command::edges_taken(claimed, now)` = `now.wrapping_sub(claimed)`, a u8
  (seam.rs:78-80; world.rs:1346-1350). Three loops: reload `0..edges[1]` (:1426), further fire `1..edges[0]` (:1490,
  authority only), throw `0..edges[2]` (:1506, authority only). Each at most 255 trips; melee taken, no loop.
- **What bounds the advance a hostile client can put in one command** (input.rs:285-300, kind 5, against the newest
  accepted command; the sim's reference is the last run, which equals it whenever every accepted command runs):
  - inside a session: `after8` (proto/src/wrap.rs:43-48: at most 127 forward) and `covered` = ticks since the newest
    accepted. So up to min(127, ticks of silence): a client that sends nothing (or whose commands all go late) for
    127 ticks (4.2 s at TICK_HZ 30, sim/src/ticks.rs:12) may carry 127 edges on one button in one command. The
    sustained rate stays at most one edge per tick per button (the sum of `covered` is the elapsed ticks, plus kind 1's
    lead of 12): F1 adds concentration, not volume, inside a session;
  - a session's first accepted command: no bound at all (`if let Some(prev) = &self.accepted`, input.rs:287), so up to
    255 per button, including a counter that moved back (read as 256 minus the step). Volume, not just concentration:
    see Block 3.
- **Per-edge effect:** fire: one shot per command at most (`fired` once, world.rs:1473-1477), each further edge one
  Refusal push (state-neutral; T2 a-d prove the twin half). Reload: one start, the rest refused (T2 e). Throw: one
  grenade per free pool slot (`throw`, :1525-1560), the rest PoolFull, one push each. So the only state a multi-edge
  advance buys is grenades.
- **Grenades from one command land together.** All share `origin`, `d`, `start_tick` (= `next_tick`) and the scene's
  fuse (:1532-1560), so they burst at one point on one tick, each doing BLAST_DAMAGE 50 to every soldier within
  BLAST_RADIUS 4 m (world.rs:107-110, :1719-1729). 20 grenades (1,000 / 50) kill any soldier at SS/ES health 1,000
  (scene.rs:185; ES takes it by `..Scene::stress()`); 2 do at the course's 100 (:245). Free pool: SS 320 (:181), ES 64
  (:210), course 16 (:233). Before F1 one command threw at most one grenade (50 damage).
- **Cross-player effect:** the pool is the world's, shared by every slot (world.rs:687, :865). A burst that fills it
  makes every other player's throw PoolFull until the fuses run out (2,000 ms SS, 1,515 ms ES), and in ES each grenade
  also takes a spawn from the stand-in spawner (:1532-1534, :1742-1748), which then waits.
- **CPU:** each throw edge scans the pool (`position`, :1526); with the pool full, each further edge is a full scan:
  255 x 320 = 81,600 reads per command (RR-F1-1). Inside a session that needs 127 ticks of silence per burst (127 x 320
  = 40,640 reads, about once per 4 s per client); per rejoin, 255 x 320. Bounded; no allocation per edge in steady
  state (the refusal Vec keeps its capacity, cleared each step: rust seat Block 2). The rejoin's own cost (a new
  connection, a Join, a full state) is larger than the scans. I concur with RR-F1-1 at Low.
- **Memory:** a command adds at most 255 + 254 + 255 = 764 sim `Refusal` records (slot u8, seq u16, kind: a few bytes
  each); per tick at most the commands run times that; cleared each step. No structure keyed by client data grows.
  The room's own refusal records stay capped at 4,096 a tick (room.rs:35); the sim's list is separate and bounded by
  the commands run.

## Block 3: a session's first command, hostile and honest (done)
- **The reach** (input.rs:287; room.rs:830 `InputBuffer::new` per join; the sim's `claimed` (world.rs:681-682, :854)
  and the room's `run_presses` (room.rs:508-510, :584) outlive the session): a session's first accepted command (after
  the rejoin overlap, input.rs:277-279) carries an advance up to 255 per button that no check bounds. F1's sim takes
  all of it: for throw, a free pool of grenades at one point and tick (Block 2: up to 64 x 50 = 3,200 damage in ES's
  4 m, 320 x 50 in SS), against one grenade before F1.
- **Repeatable:** each new connection gets one `Join` (pump.rs:434-448); `Room::join` admits a token holder again and
  replaces the live session (room.rs:814-863; roster.rs:116-124). I find no limit on how often a token holder rejoins
  (no per-token or per-address rejoin limit in server/ or room/: grep). So every rejoin buys one unbounded advance:
  volume beyond the one-edge-per-tick rate, repeatable at the connection setup rate.
- **Honest, too (RR-F1-3, concur):** JoinAccepted carries no counters (room.rs:851-861), and the bots start theirs at 0
  (bots/src/lib.rs:289). A restarted honest client rejoining its slot reads as an advance of 256 minus its old counter.
  For today's bots (fire only, bots/src/lib.rs:643-646) that is one shot plus up to 254 Cooldown refusals:
  state-neutral. **What F1 changes in the record:** before F1 the room counted that phantom advance as lost (seen +n,
  lost +n-1), a visible flag in `press_lost_count`; after F1 it counts seen +n, lost 0 (room.rs:1340-1348), so an
  honest rejoin's phantom presses vanish from every series (sim refusals reach no log: lab/src/scenario.rs:1374 reads
  clamps only). Today only the clock suite reconnects, and its clients never fire (scenario.rs:851, :1213-1214), so no
  judged run holds one. ML browser runs and page reloads: not checked.
- **No run meets the hostile throw before U4f2:** no bot presses throw (grep: `PRESS_THROW` only in sim/src/replay.rs's
  synthetic recordings, 1 in 400, and world.rs's loop); -20 has no rejoin; -22's bots press nothing (advisor Block 1).
- **On the carry:** ADV-U4FF1-3's Medium and its carry to U4f2 (condition 7) stand; F1 need not change kind 5. But the
  carried item must name the rejoin repeat, the one-point blast and the series that no longer shows an honest rejoin,
  and it must be closed before any run with a hostile client that throws or rejoins, or with a client that restarts
  mid-run. Fix shapes for U4f2 to decide: (a) JoinAccepted carries the slot's run counters (`run_presses[slot]`, equal
  to the sim's `claimed`), the client starts from them, and kind 5 checks a session's first command against them with
  `covered` from the join tick (kind 2's anchor, input.rs:270); or (b) the room re-baselines on a session's first
  command (0 edges taken from it; needs a sim hook, and U4f2's lane keeps crates/sim shut). (a) keeps ADR 0001:226's
  "neither lose nor double" across a rejoin; (b) can lose a press made on the first command. -> SA-F1-1.

## Block 4: the path with PM-0001-21/no-press-edge bitten (done)
- `no_press_edge` is `self.bitten(RoomBite::NoPressEdge)` (room.rs:650), and `bitten` is `false` outside the `bites`
  feature (room.rs:696-705): no measured build reaches the path.
- Bitten, kind 5 is off on every command (input.rs:293 skips both the `after8` and the `covered` test), so every
  command, not only the first, may carry up to 255 edges per button, a counter moved back included. F1's sim takes
  them: fire one shot and the rest refused, reload one start and the rest refused (state-neutral), throw up to the free
  pool. Before F1, one edge each.
- The forged suite's kind-5 forger moves only the fire counter, covered + 1 edges (bots/src/lib.rs:179-184), so the
  bite run's state effect is as before F1 (at most one shot per command); its show is the attribution's kind-5
  `unrejected_count`, not the state. The unit proof (room/tests/commands.rs:1050-1055: two fire edges, seq 2 runs)
  still passes (f1-b1-bites.log, check1.log:9).
- **What F1 adds on this path:** each forged kind-5 command that runs now leaves `covered` sim Cooldown (or Ammunition)
  records naming its own seq, where the old rule left none. U4f2's attribution (u4f2-brief.md:36-40, C-U4F2-1's proof
  :44-46: "no forged sequence without a refusal record of its own kind") stays right only if it matches the kind: a
  kind-5 item is rejected only by the room's PressEdge record, never by a sim Cooldown record of the same (slot, seq).
  C-U4F2-3 puts the room's records in the sim's struct form `{ slot, seq, kind }`, so the two kinds must stay
  distinct there. Were the attribution to match any record for a forged (slot, seq), the bitten run would read
  kind-5 items as rejected and the bite would pass: a bite that must fail. -> SA-F1-2 (a carry to U4f2's build).
- Same carry, kind 7 under option B: a forged fire edge whose command goes late is now taken on the next command's
  seq, so its Cooldown record names the honest seq, not the forged one. At P0 (lossless) the forged suite has no late
  commands (SEC-U4F-1's precondition), so only a lossy profile meets it.

## Block 5: the room's counts (done)
- `run_one` (room.rs:1340-1348): per tick `t.presses` are u32 sums reset each tick (room.rs:388); a command adds at
  most 4 x 255; 48 clients x 8 commands x 1,020 = 391,680, far from u32. `saturating_sub` guards lost. No panic.
- A hostile client can raise its own slot's `press_count` (seen) by up to 1,020 per session start; the room-wide series
  sum it (lab/src/logs.rs:617, :667-668). Only runs with hostile clients meet it; the forged suite is judged by the
  attribution, and -20's "0 presses lost" reads lost, which stays 0. Note, no finding.
- Melee is counted seen and taken with no effect (both sides), so the counts and the sim agree on all four buttons.

## Block 6: panic and overflow from input; wasm32 (done)
- New arithmetic on client bytes: `wrapping_sub` (never traps) and u8 ranges (`1..0` and `1..1` are empty); report
  counters u64 +1 per edge; `edges[Command::PRESS_*]` constant indices; `claimed[slot]` behind the existing
  `slot >= soldiers.len()` return (world.rs:1340-1342). `throw`'s `spawn_acc -= 30,000` (i64) runs only for a grenade
  that found a free slot: per tick at most the pool, unreachable overflow. Overflow checks are on for npt-sim in
  release (rust seat Block 1). The extreme case runs: d4's first command takes 255 edges per button on every slot and
  passes (`d4_extreme_recorded_commands_step_without_an_overflow`, f1-b1-sim.log:66; F-read-3).
- No unsafe, no new dependency (check1.log:10, the deps step). Integer-only rule; wasm parity 27,000 ticks, 0
  mismatching (check1.log:11); whether those runs hold a multi-edge throw is not shown (RR-F1-4, concur).
- No `(owner, seq)`-keyed map in sim, room, client or proto (grep: the only maps are world.rs:628's test-scale
  HashMap and interest.rs:159's `(EntityKind, u16)` key), so several grenades with one command's seq collide nowhere.

## Verdict: APPROVE WITH FINDINGS
One Medium (carried, as the advisor already ruled) and two Lows; no Critical or High. The fix itself is sound for
hostile input: integer-only, panic-free at 255 edges (d4 runs it), no allocation per edge in steady state, nothing keyed
by client data grows, fire and reload stay state-neutral on every extra edge (T2 a-e, both plants biting by name). Its
one new reach is the throw: an advance the room does not bound (a session's first command, every rejoin) now buys a pool
of grenades in one tick, where the old rule bought one. No run before U4f2 meets it (no bot throws; no judged run
rejoins a firing client). F1 need not change kind 5 (condition 7), but the carried item grows (SA-F1-1).

Process note: SA-F1-1 to -3 are security findings not fixed in F1. Under advisor.md step 4 (:103, :180-181,
:267-268), one left open keeps the PR that carries it to main needs-mustafa at any severity, until U4f2 closes it or
the report defers it by name.

## Findings
| Id | Severity | Where | What goes wrong | Fix |
|---|---|---|---|---|
| SA-F1-1 | Medium | room/src/input.rs:287; room.rs:830, :851-861; world.rs:1346-1350, :1504-1508, :1525-1560; pump.rs:434-448; roster.rs:116-124 | A session's first accepted command has no kind-5 bound, and its reference (the sim's `claimed`, the room's `run_presses`) outlives the session. F1 now takes the whole advance: up to 255 throw edges become a free pool of grenades bursting at one point on one tick (ES: 64 x 50 = 3,200 damage in 4 m against health 1,000; SS: up to 320), against one grenade before. It repeats on every rejoin (one Join per connection, no rejoin limit found), and an honest restart reaches it too (RR-F1-3), now counted seen with lost 0, so it vanishes from the press series. No run before U4f2 meets it. | Concur with ADV-U4FF1-3's carry (condition 7); widen the carried item in U4f2's brief to name the rejoin repeat, the one-point blast and the hidden honest restart, and close it before any run with a throwing or rejoining hostile client or a mid-run restart. Shape (a): JoinAccepted carries the slot's run counters, the client starts from them, kind 5 checks the first command against them with `covered` from the join tick (input.rs:270); or (b): re-baseline on a session's first command (0 edges taken; needs a sim hook). A room test: a fresh session's first command 200 throw edges on is refused (a), or throws none (b) |
| SA-F1-2 | Low | world.rs:1487-1500 (extra-edge refusals); room/src/input.rs:293; u4f2-brief.md:36-40, :44-46 | F1 adds a new source of sim refusal records: one Cooldown (or Ammunition) per extra fire edge, naming that command's seq. Under the PM-0001-21/no-press-edge bite, every forged kind-5 command that runs now carries such records with its own seq. If U4f2's attribution matched any record for a forged (slot, seq), it would read the bitten kind-5 items as rejected and the bite would pass. Under option B, a late forged kind-7 edge's record lands on the next (honest) seq. | Carry to U4f2's brief: the attribution matches a forged item only by a record of its own kind (C-U4F2-1's words), with room and sim kinds kept distinct after C-U4F2-3; f16 or f33 adds a case under the no-press-edge bite whose forged kind-5 seqs carry sim Cooldown records and still count in `unrejected_count`; plant "any kind matches" fails it by name. No mark changes; changing the own-kind rule would be Mustafa's (TEST-FIRST) |
| SA-F1-3 | Low | world.rs:1504-1509, :1525-1560; world.rs:30-41 (module doc); room/src/input.rs:285-300 | Inside a session, a press bank (127 ticks of silence, or a lag switch whose held commands go late) carries up to 127 throw edges in one command, and F1 throws them all at once (20 already kill at health 1,000). The stand-in's throw has no server-side rate state, so ADR 0001:236's "edges meet the server's own cooldown" has nothing to meet for it; the module doc's "each through its own check" holds for throw only as the pool. The sustained rate stays one edge a tick: concentration, not volume. | No code change in F1. The report states that the stand-in's throw has no cooldown, so neither T2 (f) nor any -21 figure shows ADR 0001:236's ability check. For v2 (threat model): every press action has server-side rate state (cooldown or count); a press without one takes at most one edge per command |

Concur, no new id: RR-F1-1 (Low: the full-pool rescan, bounded; rejoin cost dominates), RR-F1-3 (folded into SA-F1-1),
RR-F1-4 (Low: the multi-edge wasm parity rests on reading).

Nothing in F1 touches a pass mark, the judge, marks.json, money or main. SA-F1-2 concerns how U4f2's lab will compute
a judged series (`event.forged_kind.unrejected_count`); its own-kind rule must not change without Mustafa (TEST-FIRST).

## Anti-cheat coverage table (the rows F1 touches: player action -> where the server validates it -> gap)
| Player action | Where the server validates it | Gap |
|---|---|---|
| Extra press edges in one command, inside a session | kind 5 against the newest accepted (input.rs:285-300: at most `covered`, below 128); the sim takes each edge through its own check (world.rs:1346-1350, :1426, :1490, :1506) | throw has no rate state (SA-F1-3) |
| Bank presses by silence or a lag switch | kind 5's `covered` and `after8` (at most 127 a command, one a tick sustained) | fire and reload state-neutral; throw bursts (SA-F1-3) |
| Forged counters on a session's first command, or on each rejoin | none (input.rs:287) | SA-F1-1 (carried to U4f2) |
| Counter moved back | kind 5's `after8` inside a session | first command reads it as up to 255 (SA-F1-1) |
| More than one shot per command | the sim's one-shot rule (world.rs:1473-1500): further edges Cooldown or Ammunition | none (T2 a-d) |
| Reload spam | the sim's per-edge reload check (world.rs:1426-1436) | none (T2 e); reload_ms = 0 pre-existing, no scene has it |
| Throw into a full pool | PoolFull per edge (world.rs:1525-1530) | CPU rescan per edge (RR-F1-1, Low) |
| Inflate the press series | seen counts the advance (room.rs:1340-1348) | note only: lost stays 0; forged suite judged by attribution |
| Forged kind-5 under the no-press-edge bite | bites builds only (room.rs:696-705) | attribution must match by kind (SA-F1-2) |

## Proposed memory lines (the brief limits my writes to this file; for the session to record)
1. Threat model, press edges after U4f-F1: the stand-in takes every edge of an advance; inside a session kind 5 bounds
   it (at most 127 a command, one a tick sustained); a session's first command is unbounded until U4f2 closes SA-F1-1 /
   ADV-U4FF1-3.
2. v2 rule: every press action needs server-side rate state (cooldown or count); without it the per-edge rule turns a
   press bank into a one-tick burst (SA-F1-3).
3. The press-counter reference must be continuous across a rejoin (handed to the client at join) or re-baselined;
   never an unchecked first command (SA-F1-1).

## What I ran (read only)
- sha256sum of the worktree diff and the patch (equal); `git log -1`, `git status --short`, `git diff --stat`; sed -n
  and grep on the worktree's sim, room, server, bots and lab sources, the evidence logs (check1, b1-sim), the ADR's
  part 5 and marks.json's -21 note (:5144). No cargo, no check.mjs, no script, nothing written under spikes/ or .gate/.

## What I did not check
- Built, ran or timed nothing; the CPU figures are counts of reads, not measured time.
- The client endpoint's and client-wasm's own reconnect counters (client/src); ML browser runs and page reloads.
- U4f2's attribution (not built yet): SA-F1-2 is a carry on its brief, not a reading of its code.
- Transport, origin, tokens, CSP, supply chain and CI: the diff touches none of them (5 files under crates/sim and
  crates/room; no manifest, lockfile, workflow or hosting file).
