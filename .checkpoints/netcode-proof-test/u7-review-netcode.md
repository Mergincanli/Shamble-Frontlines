# U7 review round: the netcode seat (netcode-architect, 2026-11-26)
Packet: u7-round-packet.md. Diff: wip/u7-build-rs.patch (42 files, base 9177fb5). Brief u7-brief.md rev 1.1 + C1-C14
(advisor-u7-brief.md:110-247; C6 as reworded in advisor-main-66-head.md section 1). Read only, except this file.
No Edit tool in this seat: this file is rewritten whole by Write.

## Plan
1. Builder notes (rs 0030, t 2050). 2. crates/client clock.rs, lib.rs, tests (R1, R2, C3). 3. client-wasm lib.rs,
engine.rs, script.rs (R3, C2, C5, C6). 4. web/src lib.ts, worker.ts, main.ts, hooks.ts (URL, token, framing, chooser,
C9, records C1). 5. e2e t5ii.mjs, room.mjs, serve.mjs, t5i.mjs (transport legs, known-open). 6. Logs. 7. Findings.

## Notes (one line per finding, as I go)
- Read: packet; u7-brief.md rev 1.1; advisor-u7-brief.md C1-C14; launch-0850.md section 3; rs notes 0030; t notes 2050.
- clock.rs (patch :335-650): C-U7-4 met (clamp both ends, closed form saturating, period .max(1), surface pub(crate),
  ping_every_ns gone). C-U7-3: `sent` stamps unstamped pings once, t0 = max(at, wire), matching on wire stamp. Sound.
- engine.rs step order (patch :980-1012): take(inbox) -> sent(prev_send) -> per due step input(None, pending) -> plan ->
  commands_due x script -> step. `sent` precedes the pongs it times. Matches the bots' controlled order. OK.
- client lib.rs base :790-792: next_due_ns = base_ns + steps x period (host grid from the first input), so a Worker
  stall is replayed by LOOP_CATCHUP (7) steps a wake at one `now`, wakes at 0 ms (4 ms nesting clamp).
- FINDING NC-1 (Low): script.rs:1629-1632 derives press counters from the Core's step count; room kind 5
  (room/src/input.rs:286-297) allows edges <= ticks covered. After a >= 2 s stall, catch-up advances steps ~50-90 while
  command ticks advance ~1, so the fire counter can move 2 over 1 tick: a PressEdge refusal against an honest page.
  Untested (room tests have no stall). Fix: count presses from `self.commands` (commands issued), not `self.steps`.
