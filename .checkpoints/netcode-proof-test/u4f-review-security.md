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

## Progress
- 14:0x UTC: read the panel brief, history.md:6896-6903 (SIZE-U4F (a)), advisor-u4f-stop.md whole, u4f-brief.md
  2.1-2.9 (:356-644). Patch sha256 74ae26aa… matches; 29 files (numstat). Memory: no earlier U4f decision.
- input.rs (618 lines) read whole. Order as 2.4: filter (seq not forward) → newest_seq → kind 1 (dist vs next_tick,
  signed i64) → kind 2 (accepted or join anchor) → rejoin overlap (accepted None only) → 4 (i32 widened, max 32,768:
  no overflow) → 5 (after8 + covered) → 6 (extent) → late / A's W → 10 (len >= 8) → accept. Structures bounded: kept 16,
  stamps 64, buffered 8 (bite 32). Every tick compare is `dist` (wrapping_sub as i32): no u32 add/sub on wire ticks
  except `depth` (wrapping_sub, safe).
- room.rs read :1-1400 (struct, new, receive_input, tick, fill_input, route_events, encode_all, run_one) and the
  diff. (q2) slot overwritten from the session (room.rs:992 `Command { slot, ..*cmd }`; run_one :1366 again); the
  two per-slot arrays (kept_stamps, run_presses: :582-583) are sized to the scene's slots and indexed only after
  `clients.get_mut(slot)` succeeds (:939-945, :997) or from a roster slot checked against clients.len() (:816):
  no panic. (q3) Checks.next_tick = self.next_tick, the tick the room runs next (:923; R2 (g)). Kind 8 before any
  command, by datagram, returns before newest_seq moves (:949-988). refusal records bounded 4,096 a tick (:35,
  :932-938). Option A's drained list: at most 8 per joined client.
- (q1) closed, no finding: the sim takes one edge per changed counter per command (world.rs:1340-1346), and its own
  cooldown refuses a fire edge (kind 7), so a fresh session's unchecked first press counters buy at most one edge,
  as any honest command can.
- Pong: +8 B `server_ns` (msg.rs), 25 B within Limits::PONG 32 (limits.rs:33); the pump's `pong()` reads only
  id/echo_ns/hold_us (pump.rs:502-518): a client's server_ns is never trusted.
- seam.rs:16-21: the sim reads slot, seq, buttons, presses, move, aim only; stamp and view time stay room-side.
  The view time's tolerance and cap are U4h's (rewind.rs:1-2): known, out of scope.
- Sim's debug_assert (world.rs:2129-2133, canonical :257-262): adjacent pairs strictly ascending wrap-aware. Clean
  build: option A's stamps in one tick lie in [t-8, t+12], strictly rising (kind 2): holds. Release has no debug
  assertions (Cargo.toml:116-120: overflow checks on for sim/proto only), so a bite run cannot panic there.
- Arithmetic on wire values in new room code: dist, i32 move square, b.wrapping_sub(a), ran_early wrapping_sub,
  take's `before - len` (retain only shrinks): all safe in debug.
- 2.5 (lab scenario.rs:549-746): counts read before every end: Leave before room.receive (:631-637), closed link
  before Room::leave (:672-681), replace read before the join resets (:599-605, :647-650), kick with the kicked
  ClientSlot kept (:729-741). SERIES.md :381-382 list every room and pump refusal field. Carry 76: the replaced
  session dropped before anything else routes (:647-650); a kicked slot: receive_input returns (:946), Resync
  ignored (:898), fill_input/route_events/encode_all skip it.
- Never-acker: ack groups 0 → no ack processing (room.rs:949); queue fills to 1,600 → one forced resync; a second
  overflow inside it, or the forced full state unacknowledged past FULL_STATE_ACK_TICKS, kicks (:1255-1257,
  :1289-1295). Memory bounded by the cap (about 64 KB a client). R2's 1,600 under carry 50's 4,328: agree.
- -21 before U4f2: c-u4f-runs.mjs SETS holds 47, 22, 20 only; its usage refuses any other key (:62). c-u4f-r2.mjs
  judges -21 rows with --only PM-0001-2,PM-0001-21 on 20 s / 70 s runs, written only under .gate/ (never results/),
  every -21 count part failing by length (1,000 points, 100 bursts, 10 episodes; no forged rows). No route found.
- S1 (fast client sends no Ping) against my SA3-7: CONFIRM. SA3-7 asked only that its ping overflow be reported,
  never failed. No room check reads a client Ping (kind 1 counts from the room's own next tick: input.rs:265); the
  fixed-lead endpoint still answers the room's Pings (client/src/lib.rs:712-725, :788-792), so the room's RTT samples
  for it exist; the ping bucket keeps its pump tests. Not a weaker check: the suite simply sends no ping load.
- Forgers 1, 2, 4, 5, 6 (bots/src/lib.rs:167-192, :307-316, :461-476): match 2.6's table; one field each; f15 (every 3)
  checks against the endpoint's previous command. FINDING SEC-U4F-1 (Low): kinds 2 and 5 forge against `prev`, the
  endpoint's previous command, not the room's newest accepted. If prev was itself refused (forger every = 1, which
  `forger` allows: `every.max(1)`, :311) or lost, a correct room accepts the "forged" command by its rule (kind 2:
  tick_n > tick_{n-1}; kind 5 with the steady script's held fire: edges = covered), so U4f2's lab would count it
  unrejected on a correct build. Fix: `forger` returns None for every < 2 and its doc names the precondition (prev
  accepted: every >= 2, a lossless profile); U4f2's brief carries it to f16.
- Carry for U4f2 (not a finding: the attribution is U4f2's): `Room::refusals()` and `refusals_dropped()` have no
  lab reader yet (grep of lab/src: none); when the attribution reads refusals, it must log refusals_dropped, since a
  dropped record makes a refused forged item read as unrejected. 48 clients x 8 Inputs x 8 commands = 3,072 < 4,096
  a tick, so it cannot drop at 48.
