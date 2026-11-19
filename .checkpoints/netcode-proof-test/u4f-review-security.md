# U4f review panel, round 1: seat security (security-auditor), 2026-11-19

## Plan (what I read, in order)
1. Mustafa's SIZE-U4F words (history.md:6896-6903) and the stop ruling's U4f2 list (advisor-u4f-stop.md:80-89) and
   S1-S7/N1 (:198-210), so I report nothing out of scope and answer S1 against my SA3-7.
2. u4f-brief.md 2.4 (kinds and order), 2.5 (counters), 2.6-2.8 (cheat suites, never-acker, cap), 2.9 (carries), 2.10
   negative cases, 4 build rules; advisor rulings L1-L6, build-1 R1-R4/F1-F11.
3. The patch's file list (git apply --numstat), then the room-side command checks in the worktree: decode path, kind
   order, clamp points, seq/tick checks, counters, never-acker, the 1,600 cap, carry 76 Kick path.
4. The client clock and lead controller only for what a client sends the room trusts (stamps, ticks, Ping/Pong).
5. npt-bots' cheat suites (clamp, lag switch, fast clock) and forgers 1,2,4,5,6: do they claim -21 before U4f2?
6. Runs only to prove a finding, through the runner, logs under .gate/u4f/review-security/.
7. Verdict, findings table, coverage table, own lines, runs, not checked.
(This agent has no Edit tool: the notes file is rewritten whole with Write at each step, as the brief allows.)

## Progress (one line per check or finding, in the order made)
- Read the panel brief, history.md:6896-6903 (SIZE-U4F (a)), advisor-u4f-stop.md whole, u4f-brief.md 2.1-2.9
  (:356-644). Patch sha256 74ae26aa… matches; 29 files (numstat). The worktree's runner hashes, recomputed read-only
  by u4f-run.sh:67's own commands: diff 54cb6faa…, untracked e8dc3663…, equal to review-pre.log and both end checks.
  Memory: no earlier U4f decision.
- input.rs (618 lines) read whole. Order as 2.4: filter (seq not forward) → newest_seq → kind 1 (dist vs next_tick,
  signed i64) → kind 2 (accepted or join anchor) → rejoin overlap (accepted None only) → 4 (i32 widened, max 32,768:
  no overflow) → 5 (after8 + covered) → 6 (extent) → late / A's W → 10 (len >= 8) → accept. Structures bounded: kept 16,
  stamps 64, buffered 8 (bite 32). Every tick compare is `dist` (wrapping_sub as i32): no u32 add/sub on wire ticks
  except `depth` (wrapping_sub, safe).
- room.rs read :1-1400 and its diff. Slot overwritten from the session (room.rs:992; run_one :1366); kept_stamps and
  run_presses (:582-583) sized to the scene's slots, indexed only after `clients.get_mut(slot)` succeeds (:939-945,
  :997) or from a roster slot checked against clients.len() (:816): no panic. Checks.next_tick = the tick the room
  runs next (:923; R2 (g)). Kind 8 first, by datagram, returns before newest_seq moves (:949-988). Refusal records
  bounded 4,096 a tick (:35, :932-938). Option A's drained list: at most 8 per joined client.
- A fresh session's first command has no room press compare (accepted None): no finding, since the sim takes one edge
  per changed counter per command (world.rs:1340-1346) and its cooldown refuses a fire edge (kind 7).
- Pong: +8 B `server_ns` (msg.rs), 25 B within Limits::PONG 32 (limits.rs:33); the catalogue row updated (24 B,
  rounded up to 32 B); the pump's `pong()` reads only id, echo_ns and hold_us (pump.rs:502-518): a client's
  server_ns is never trusted.
- seam.rs:16-21: the sim reads slot, seq, buttons, presses, move and aim only; stamp and view time stay room-side.
- Sim's debug_assert (world.rs:2129-2133; canonical :257-262): adjacent pairs strictly ascending wrap-aware. Clean
  build: option A's stamps in one tick lie in [t-8, t+12], strictly rising (kind 2): holds. Release has no debug
  assertions (Cargo.toml:116-120: overflow checks on for sim and proto only).
- New room arithmetic on wire values: dist, the i32 move square, b.wrapping_sub(a), ran_early's wrapping_sub, take's
  `before - len` (retain only shrinks): all safe in debug and test builds.
- 2.5 (lab scenario.rs:549-746): counts read before every end: Leave before room.receive (:631-637), closed link
  before Room::leave (:672-681), replace read before the join resets (:599-605, :647-650), kick with the kicked
  ClientSlot kept (:729-741). SERIES.md :381-382 list every room and pump refusal field. Carry 76: the replaced
  session is dropped before anything else routes (:647-650); a kicked slot: receive_input returns (room.rs:946),
  Resync ignored (:898), fill_input, route_events and encode_all skip it.
- Never-acker: ack groups 0 → no ack processing (room.rs:949); its queue fills to 1,600 → one forced resync; a second
  overflow inside it, or the forced full state unacknowledged past FULL_STATE_ACK_TICKS, kicks (:1255-1257,
  :1289-1295). Memory bounded by the cap (about 64 KB a client). R2's 1,600 under carry 50's 4,328: agree.
- -21 before U4f2: c-u4f-runs.mjs SETS holds 47, 22 and 20 only; its usage refuses any other key (:62). c-u4f-r2.mjs
  judges its -21 rows with --only PM-0001-2,PM-0001-21 on 20 s and 70 s runs, written only under .gate/ (never
  results/), every -21 count part failing by length (1,000 points, 100 bursts, 10 episodes; no forged rows). The 20
  new SERIES.md rows claim no -21 judgement. No route found.
- S1 against my SA3-7: CONFIRM (own line below).
- Forgers 1, 2, 4, 5 and 6 (bots/src/lib.rs:167-192, :307-316, :461-476) match 2.6's table, one field each; f15
  (every 3, :1030-1135) checks against the endpoint's previous command. → SEC-U4F-1 (Low).
- `Room::refusals()` and `refusals_dropped()` have no lab reader yet (grep of lab/src: none): their consumer is the
  attribution, U4f2's → carry C-U4F2-SEC-1 below, not a finding. 48 clients x 8 Inputs x 8 commands = 3,072 < 4,096
  a tick, so nothing can drop at 48.
- The fast-clock tally (scenario.rs:984-1035) counts from the room's own commands_run and the sim's soldier record,
  never the cheat's claims; its bound is the room's (`lead_bound().1`, :1186).
- header.params.lead_bound_ms written from the constant (scenario.rs:1899), not the room's `lead_bound()`. → SEC-U4F-2
  (Low).
- Weapon clock and reload advance once per command run (world.rs:1409-1417), so option A's post-idle burst compresses
  movement, fire clock and reload alike, bounded by f38 at W + bound = 20 commands over any span: SA3-1's ruled
  resolution (U4F-Q18 (a)), recorded here as a property for the OQ-0001-6 choice, not a finding.
- The rejoin overlap sits right after kind 2 (input.rs:277-279), the late rule after kinds 4-6 (:308-314). The
  overlap is the kind-2 reference across joins (2.3), so its place is the natural reading; a forged field inside a
  rejoin's overlap window (at most the bound's 12 ticks) reads `rejoin_overlap`, not its kind. The command is dropped
  either way and no forger rejoins: note N-SEC-1, not a finding.
- No run made: both findings are proved by reading and grep; a plant or new test is the fix pass's (brief section 3).

## Verdict: APPROVE WITH FINDINGS
Two Lows, no Critical, High or Medium. The room-side checks of kinds 1-6, 8 and 10 run in 2.4's order on validated,
session-scoped values, every structure a client sizes is bounded, no new arithmetic on a wire value can panic or wrap
wrongly, and the never-acker ends in a resync and then a kick. Nothing lets -21 be judged before U4f2.

## Findings
| Id | Severity | Where | What goes wrong | Cheapest proof | Fix |
|---|---|---|---|---|---|
| SEC-U4F-1 | Low | bots/src/lib.rs:167-192 (`forged`), :307-316 (`forger`: `every.max(1)`), :461-476 | Kinds 2 and 5 forge against `prev`, the endpoint's previous command, not the room's newest accepted. With `every` 1 (allowed), seq n forged and refused; seq n+1 forged to tick_n (kind 2) passes the room's rule against tick_{n-1}, and kind 5's fire counter at prev + covered + 1 equals the room's covered when the steady script holds fire unpressed (no edge on n). A correct room accepts it, so U4f2's attribution would count it unrejected (a false FAIL). The same follows after a lost previous command on a lossy profile. | A room-level test in f16's shape: `forger(2, 1, 12)` and `forger(5, 1, 12)` on the steady script at P0 into a `Room<World>`, counting forged seqs with no refusal record of their kind: above 0 today (about half). f15 cannot show it: it checks only the forged field (:1093-1106). | `forger` returns `None` for `every < 2`, and its doc names the precondition (the previous command accepted: every >= 2 on a lossless profile, P0). U4f2's brief carries it to `--forge-every`'s parse and to f16. A few lines; rides the fix pass, or goes to U4f2 by number. |
| SEC-U4F-2 | Low | lab/src/scenario.rs:1899; room/src/room.rs:599-605, :625-628 | `header.params.lead_bound_ms` is written from the constant `Room::LEAD_BOUND_MS`, not from the room's `lead_bound()`, whose doc says "for the run's header". `with_lead_bound_ms` is public. No lab path calls it today (grep: only room/tests/commands.rs:814), and the excess uses the room's real bound (:1186), so no run is wrong now. But a later lab path that set another bound would still record the frozen 400 in every header while the room ran the other value: the LEAD-BOUND record would be false. | grep, done: scenario.rs:1899 writes the constant; no lab caller of `with_lead_bound_ms`. | Write `server.room.lead_bound().0`, or assert at the header write that it equals `Room::LEAD_BOUND_MS`. One line; rides the fix pass. |

S1-S7 and N1 (advisor-u4f-stop.md:198-210): no disagreement. S1 confirmed (own line).

## Notes and carries (not findings)
- N-SEC-1: a rejoin's overlap window attributes a forged field as `rejoin_overlap` (input.rs:277-279, ahead of 4-6).
  The command is dropped either way and no judged forger rejoins. If U4f2's attribution ever rejoins a forger, it
  must read this counter.
- C-U4F2-SEC-1 (for U4f2's brief): when the attribution reads `Room::refusals()`, it also logs `refusals_dropped()`
  per tick or at the run's end, so a refused forged item whose record was dropped can never read as unrejected.
- C-OQ6-SEC-1 (for the OQ-0001-6 choice; ruled, recorded): under option A each command run advances movement, the
  weapon clock and the reload one tick (world.rs:1409-1417), so a burst after an idle compresses up to W + bound = 20
  ticks of all three into about 3 ticks (8 a tick; f38's bound). Under B, one a tick. -21's lag-switch check measures
  movement; the fire clock rides the same cap.
- Known and out of U4f: the view time's tolerance and the rewind cap are U4h's (rewind.rs:1-2); the rewind is bounded
  only by the 2 s ring until then.

## Anti-cheat coverage table (player action → where the server validates it → gap)
| Player action | Where the server validates it | Gap |
|---|---|---|
| Stamp a command far ahead | kind 1, input.rs:265 (signed distance from the room's next tick; 12 ticks from LEAD_BOUND_MS 400: room.rs:524, :580) | none |
| Stamp equal or older, or at or before the join | kind 2, input.rs:270-275; join anchor room.rs:829, :857 | none |
| Rejoin to replay stamps | kept stamp per slot, room.rs:997-1000; overlap input.rs:277-279 | attribution only (N-SEC-1) |
| Resend an accepted command changed | kind 3, input.rs:237-259 | none |
| Jump or skip sequences | outside-window, input.rs:238-240; newest_seq moves on any new seq (by U4F-Q13 (b)) | self-harm only, per slot |
| Long move vector | kind 4, input.rs:281-284 (i32-widened) | none |
| Extra press edges | kind 5, input.rs:286-300; the sim takes one edge per command, world.rs:1340-1346 | first command of a session uncompared (bounded by the sim's one-edge rule) |
| Aim off the map | kind 6, input.rs:302-306; z by the clamp (SA-14) | by design |
| Aim beyond reach | the sim's clamp, world.rs:1283-1312 (reported, never refused) | by design |
| Fire or reload against cooldown | the sim, world.rs:1418-1462 (kind 7) | attribution is U4f2's |
| Burst of commands (lag switch, idle bank) | kind 10 and A's W, input.rs:308-323 | bounded at W + bound = 20 over any span (f38); compresses movement and weapon clock (C-OQ6-SEC-1) |
| Fast clock | kinds 1 and 2; one command a tick under B | ping load not exercised (S1: fine) |
| Forge the slot in a command | overwritten from the session, room.rs:992, :1366 | none |
| Acknowledge a snapshot never sent | kind 8, room.rs:949-988 | none |
| Never acknowledge | event cap 1,600, forced resync, kick, room.rs:1246-1295 | none |
| Forge a Pong's server time | ignored, pump.rs:502-518 | none |
| Trailing bytes, unknown field, wrong channel (9, 11) | the pump, pump.rs:405-431 | forgers are U4f2's |
| Flood Inputs | pump bucket of 8 refilled at 60 a second, pump.rs:449-456 | none |
| Claim an old view time | the 2 s ring only (rewind.rs) | U4h's tolerance and cap (known) |
| Flood refusals | records bounded at 4,096 a tick, room.rs:35, :932-938 | refusals_dropped not logged yet (C-U4F2-SEC-1) |

## Own lines (the brief's, each in a line of its own)
- S1 (the fast client sends no Ping) against my SA3-7: CONFIRM. SA3-7 asked only that its ping overflow be reported,
  never failed. No room check reads a client's Ping: kind 1 counts from the room's own next tick (input.rs:265). The
  fixed-lead endpoint still answers the room's Pings (client/src/lib.rs:712-725, :788-792), so the room keeps its RTT
  samples for that client, and the ping bucket keeps its pump tests. The suite sends no ping load; no check is weaker.
- Nothing built lets -21 be judged before U4f2: c-u4f-runs.mjs has no -21 set (SETS 47, 22, 20; usage :62), and
  c-u4f-r2.mjs's -21 rows are 20 s and 70 s pre-runs under .gate/ whose count parts fail by length.
- S1-S7 and N1 as ruled: I disagree with none.
- Memory: not updated (this brief limits my writes to this file and runner logs). Proposed memory lines for the
  session: (1) U4f threat model: the room trusts no client clock, Ping or Pong time; stamps are bounded by the room's
  next tick, the join anchor and the per-slot kept stamp. (2) Accepted, pending OQ-0001-6: option A's burst
  compresses movement and weapon clock within W + bound = 20 (C-OQ6-SEC-1). (3) U4f2: forgers need every >= 2 on a
  lossless profile (SEC-U4F-1), and the attribution must log refusals_dropped (C-U4F2-SEC-1).

## What I ran (all read-only; no cargo, no runner call, no runner log written)
- sha256sum of wip/u4f-build.patch; `git log -1`, `git status --short`, `git apply --numstat` on the patch;
  `git diff HEAD` of room.rs, input.rs, sim world.rs, pump.rs, msg.rs, events.rs, room lib.rs, SERIES.md and the
  catalogue (in the worktree, from its folder); the runner's two tree hashes recomputed by u4f-run.sh:67's commands;
  greps of the sources, marks.json and SERIES.md named above.

## What I did not check
- I built, ran and timed nothing; the end checks, hand runs and cap figures are the builders' logs, read only through
  the ruling and the notes.
- Read in part only: lab/src/scenario.rs (:440-760, :984-1047, :1186, :1232, :1890-1905 of 4,215); client/src/lib.rs
  (:700-800); clock.rs (:135-230); bots/src/lib.rs (:60-620, :1018-1162); room/tests/commands.rs (:100-280 and its test
  list). Not read: logs.rs, client tests (trace.rs, carry80.rs, bites.rs), proto/tests/pong.rs, room/tests/bites.rs
  and room.rs tests, server tests, c-u4f-cap.mjs, the judge's reading of the -21 series.
- Brief sections 2.10-2.13, 4, 5.3, 5.6 and 6, advisor-u4f-brief.md L1-L6, advisor-u4f-fold-check.md, and
  advisor-u4f-build1.md beyond its findings index (:16-31, :251-261): other seats' ground.
- The client's ClockSync against a hostile server (outside this threat model; the rust seat's arithmetic).
- Transport, origin, tokens, CSP, supply chain and CI: the patch touches none of them (no manifest, lockfile, workflow
  or hosting file among its 29).
