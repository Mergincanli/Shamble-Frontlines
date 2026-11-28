# U4f-F1 review round, netcode-architect seat (2026-11-20)

Read only. Brief: u4f-f1-panel-brief.md. Diff: wip/u4f-f1-build.patch (sha256 37d87a10bed52b81..., equal to
`git diff cb7e374 -- spikes/` in the builder's worktree: 205+/24-, 5 files). Line numbers below are the worktree's
(.claude/worktrees/agent-ad5f72ed146da5f93) unless named otherwise. Nothing run but read-only git/grep/sed.

## Plan
1. Read the patch whole; the build record (u4f-f1-build.md); the conditions 1-9 (advisor-u4f-f1-plan.md:186-221).
2. ADR 0001 part 5 (docs/adr/0001-multiplayer-netcode.md:220-275): does "each edge through the server's own state"
   match the new run_command (reload, fire, throw, melee; clock once; one shot per command)?
3. The room's counts: run_one's seen/lost/doubled from the shared fn; SERIES.md:42's meaning; option A and option B paths
   (input.rs kind 5, the late drops, the repeat path); doubled on a repeat.
4. The expected -20 figures (advisor-u4f-f1-plan.md:119-140): does the diff keep state-neutral for -20's bots (fire held
   only), so per-tick h equal to 350677f's? Anything in the diff that could move a hash or a summary line.
5. A session's first command (ADV-U4FF1-3): what F1 does now, what the build notes say, what U4f2 must carry.
6. Findings, verdict.

## Blocks

### Block 1: the diff against ADR 0001 part 5 (done)
- ADR cites hold on main's copy: :226 "a press counter per button (a resent command can neither lose nor double a
  press)"; :235 "a press counter advances by at most one edge per button for each tick the command covers"; :236-237
  "fire, ability and reload edges meet the server's own cooldown, ammunition and reload state".
- world.rs `run_command` (:1338-1511): edges from the seam fn (:1346-1350); `claimed` updated before the dead or plate
  return (:1351-1355, as today: U4FF1-Q2); the weapon clock once (:1415-1423); reload loop (:1426-1435), each pass
  against the state the pass before left; fire: today's code on `edge` (:1436-1486), then `1..edges[FIRE]` refused by
  the kind test, authority only (:1489-1499); throw loop, authority only (:1505-1509); melee taken, no effect. This is
  condition 1 as written and reads ADR :236-237 as "each edge meets the server's own state".
- One shot per command = the server's cooldown for every scene weapon (interval above one tick, so after a shot `wait`
  > 0 and a re-read of `ready` would refuse too); only sub-tick test intervals see a difference (labelled Cooldown).
- Prediction (`predict`, authority false): the reload loop runs as on the authority; extra fire edges and throws are
  authority-only and change no state; so prediction still equals the authority's tick for any advance (D-3 by reading).
- Counters carry counts, not order: one command's edges run reload, fire, throw. A late-dropped pair "fire at t (full
  magazine), reload at t+1" collapsed into one command refuses the reload (the magazine is full when the reload edge
  runs, before the shot). Unreachable in -20 (bots press only fire: bots/src/lib.rs:645). The ADR does not state the
  order, nor one shot per command, nor that collapsed throws leave one origin at one tick (the stand-in's throw has no
  cooldown: two co-located grenades) -> NC-F1-3.

### Block 2: the room's counts, SERIES.md:42, options A and B (done)
- run_one (room.rs:1340-1349): advance = `p.wrapping_sub(presses[k])`; taken = `Command::edges_taken(presses[k], p)` =
  `now.wrapping_sub(last)` (seam.rs:78-79): the same expression. So lost = advance - taken is 0 in every build and every
  run (no bite edits the fn). Doubled on B's repeat was already 0 by construction (the repeat is `last_run`,
  input.rs:367-372, whose counters are the slot's `run_presses`); F1 leaves that.
- The room computes "taken" from its own copy of the rule, in its own (arrival) order; it never observes what the sim
  took. A sim that skipped the fn is caught only by T1's refusal assert (commands.rs:642) and T2's count halves, never
  by a run. Illustration (a bite path only): under A with kind 2 relaxed, the room counts in arrival order and the sim
  runs in canonical order (room.rs:445-447), so a backwards step reads 255 seen in the room where the sim took 1.
- SERIES.md:42 "press edges lost this tick (against the press counters)": the text stays true; its value is now a
  constant. ADR 0001's PM-0001-20 row (main's :1610) asks "10,000 scripted presses with reordering: 0 lost and 0
  doubled", lost against the scripted presses. The judged series measured the sim's rule only; F1 fixes the rule, so
  the judged check can no longer fail at run time (marks.json:4749-4757). The same for PM-0001-58's "0 presses lost"
  across the u8 wrap (marks.json:4214-4220): the wrap is now computed by the one fn on both sides. The run-scale
  evidence that every scripted press arrived is "presses" = 432,000 per B run (= 48 x 18,000 / 2, the scripted count in
  the window: advisor-u4f-20.md Block 1), a reported figure, not a judged check -> NC-F1-1.
- The advisor's Block 4 premise (advisor-u4f-f1-plan.md:143-145, "its count is still what the sim does not take, and
  the plant shows it can fail") holds only through T1: the count is what the one fn does not take; the plant edits the
  fn, so it moves room and sim together.
- Option B: a late command returns before `accepted` is set (input.rs:313 vs :340), so kind 5's reference stays the last
  on-time command and the next command's advance (at most the ticks covered) is what the sim takes. T1 is -20's
  mechanism exactly (late drops, not link loss). Option A: multi-edge only after a past-window Late or a Catchup
  refusal; each command keeps its own clock tick and at most one shot, as before. Per slot, arrival order = tick order
  = canonical order for every honest stream (kind 2: strictly newer), so room and sim walk the same counter sequence.

### Block 3: the expected -20 figures (advisor-u4f-f1-plan.md:119-140) (done)
- Bots hold fire and press only fire (bots/src/lib.rs:645, :658-662), so every extra edge in -20 is a refused fire
  edge: no state, no event (T2 a-d: `twins` asserts equal events and hashes on every tick).
- Readers of sim refusals: room.rs:1046 clones the reports; the lab reads them only for clamp events
  (scenario.rs:1374-1386) and -21's clamp suite's slot-0 refused set (:1408; that script never presses). No series,
  summary field or wire byte reads a fire or reload refusal. The summary line's fields (runs.txt, b-s1): commands, late,
  press refused (room kind 5), buffer full, rewinds refused, stale, overflow, heals, client hashes, incomplete,
  datagrams, shots per second, carry 49, projectiles per set, audit, fate log, connect order: none moves while the
  state is equal.
- So the advisor's expectations hold by reading: each summary line equal but its wall time (and the binaries'
  sha256 on the header lines); per-tick `h` equal to 350677f's measured/ and bites/ logs; `press_lost_count` 0 (by
  construction: NC-F1-1); B presses 432,000 each, A 432,003 / 432,003 / 432,009 (seen = advance, unchanged); the bite's
  on time 0.894544 unchanged, its lost 3,614 -> 0.
- End-check logs, read: f1-b4-check1.log and -check2.log end exit 0; test step "measured build: 641 tests passed ...
  bites build: 738 tests passed", wasm step "27000 ticks compared, 0 mismatching" native-native and native-Wasm.

### Block 4: a session's first command (ADV-U4FF1-3), carried to U4f2 (done)
- Code: kind 5 runs only `if let Some(prev) = &self.accepted` (input.rs:287); a session's InputBuffer starts with none
  (room.rs:830); the sim's `claimed` (world.rs:681-682) and the room's `run_presses` (room.rs:508-510) outlive the
  session. The build notes (u4f-f1-build.md:62-66) record F1's behaviour (all taken: fire one shot + refusals, reload
  one start + refusals, throw up to the free pool; seen = the advance, lost 0). Condition 7 is met in the notes.
- Not only hostile: an honest rejoin or replacement whose client restarts its counters at 0 (the bots: `presses: [0;
  4]`, bots/src/lib.rs:289; the browser client after a Worker trap: PM-0001-19's rejoins, marks.json:4642) meets the
  slot's old counters, so its first command's advance is (0 - old) mod 256 for each button the old session pressed:
  under F1 a phantom shot (as before), refusals, a reload start, and for a thrower grenades up to the free pool in one
  tick; seen inflated by up to 255 per button per rejoin. ADR 0001 part 5 does not say where a session's counters
  start. No F1 run meets it (-20: 48 joins, no rejoin) -> NC-F1-2.
- Recommendation for U4f2 (keeps crates/sim shut): each session's counters start at 0; the room keeps a per-session
  offset, set at join to the slot's `run_presses` (= the sim's `claimed`), adds it to each accepted command's counters
  before the sim sees them, and runs kind 5 on the first command against a virtual previous command {the join tick,
  counters 0}. Then no command goes unchecked and an honest rejoin carries no phantom press. Alternatives: JoinAccepted
  carries the slot's counters and the client starts from them; or a session's first command is a baseline that takes
  0 edges (at most one press per button lost at join).

### Block 5: prediction carry (U6) (done)
- `predict` reads and writes `claimed` (world.rs:1055 -> :1348-1351). A reconciliation that replays unacknowledged
  commands without first restoring `claimed` to the last-run command's counters reads a backwards step as up to 255
  edges (wrapping). F1 changes no predicted state there (the first edge acts as the old rule's did; extra fire edges and
  throws are authority-only; extra reload edges meet the first's started reload), but the hazard is the prediction
  unit's to close. No reconciliation is built yet (client/src calls no `predict`) -> NC-F1-4.

## Findings
| Id | Severity | Where | What goes wrong | Fix |
|---|---|---|---|---|
| NC-F1-1 | Medium | room.rs:1341-1347; seam.rs:77-79; marks.json:4749-4757 (-20), :4214-4220 (-58); SERIES.md:42 | `lost = advance - taken` with `taken` the same `wrapping_sub` as `advance`: `press_lost_count` is 0 in every run and build, so -20's and -58's judged "0 presses lost" can no longer fail at run time; the room never observes the sim (only T1:642 and T2 tie the sim to the fn). Reported as run-scale evidence, the -20 PASS would not hold (High then) | The -20 record (condition 5's report) says "0 by construction under F1's one rule, proven by T1/T2 and their plants" and cites presses = 432,000 per B run (= the scripted count) as the run evidence; seam.rs:77 "so the two cannot drift" -> "the room and the rule cannot drift; T1 ties the sim to it". Optional, in the lane: the sim reports the edges it ran through checks (InputReports) and the room's lost = seen - those. A judged "seen = scripted" check is marks.json: Mustafa's (TEST-FIRST) |
| NC-F1-2 | Medium | input.rs:287; room.rs:508-510, :830; world.rs:681-682; bots/src/lib.rs:289; u4f-f1-build.md:62-66 | The first-command carry is framed as hostile input; it is also the honest rejoin or replacement path (counters restarted at 0 against the slot's old ones): a phantom shot, reload and up to a pool of grenades, seen inflated by up to 255. ADR 0001 part 5 names no session baseline | U4f2's brief item names both reaches and decides the baseline; recommended: per-session room offset set at join + kind 5 on the first command against {join tick, 0} (crates/sim stays shut). ADR 0001 part 5 gets one sentence at its next revision (my lane). No F1 change |
| NC-F1-3 | Low | world.rs:1424-1509; ADR 0001:235-237 (main) | The ADR does not state the per-command button order (reload, fire, throw; counters carry counts, not order: a collapsed "fire then reload" on a full magazine refuses the reload), one shot per command, or that collapsed throw edges leave one origin at one tick unless the ability's own cooldown refuses them | One sentence each in ADR 0001 part 5 at its next revision (draft, my lane). No F1 change |
| NC-F1-4 | Low | world.rs:1055, :1348-1351 | `predict` reads `claimed`; a replay that does not restore it to the last-run command's counters reads a backwards step as up to 255 edges. F1 changes no predicted state, but the hazard is now visible | Carry to the prediction unit's brief: `claimed` is part of the state a reconciliation restores before its replay |

## Verdict
APPROVE WITH FINDINGS. The diff is condition 1-4 and 7-9 as written and reads ADR 0001 part 5 (:226, :235-237)
correctly; state-neutral for every -20 run by reading, so the advisor's expected figures hold. No pass mark, judge rule,
marks.json, money or main is touched by the diff. Named for Mustafa under TEST-FIRST (not decided here): after F1,
-20's and -58's "0 presses lost" are 0 by construction (NC-F1-1); a judged seen-versus-scripted check would be a
marks.json change, his.
