# netcode-architect, review round 1 of change-set 3b (head bbbf09e4, base 28254a73)

Copy: /home/user/Shamble-Frontlines/.gate/ci-wasm-wt/.gate/netcode-architect-cs3b (detached at bbbf09e4).

## Plan
1. Read my design (netcode-architect-cs3.md, blocks 2 and 3), ADR 0001 (part 5, PM-0001-34), ADR 0002 part 6,
   the plan's 5.1, DECISIONS.md PRESS-PACE.
2. Read the code in my copy: protocol (wire, wrap, codec, messages, transport, loopback, tests/decoder_inputs),
   room (lib, queue), client-endpoint (post, press, endpoint), sim-wasm local_match, sim world.
3. Q1 wire: one form per message, whole-or-nothing, roundtrip both ways; traps for delta coding / narrow widths.
4. Q2 hostile bytes: every arithmetic and index on decoded values; caps; wrap compares at edges.
5. Q3 room checks and queue: find a sequence that loses a press, runs a command twice, or stalls.
6. Q4 pacer and echo under loss and bursts; lost/doubled count under late or repeated snapshots; >255 drop.
7. Q5 step order in LocalMatch::step vs ADR 0002 part 6.
8. Q6 decoder test vs PM-0001-34 as written; run it in release with --nocapture.
9. Q7 what assumes 3c is absent.
10. Run cargo tests for protocol/room/client-endpoint/sim/sim-wasm; small probes of my own as test files in my
    copy only if needed (never tracked files), else reasoning with file:line.

## Findings (one line each, as found)
- Q6 run (mine, release, my copy at bbbf09e4): 8 passed in 5.87 s; per type 10,000,000 random + 10,000,000 mutated,
  0 panics; taken random/mutated: Join 67/2,732,095, JoinAccepted 284/2,431,990, JoinRejected 5,340/135,022,
  Input 18/2,587,079, Snapshot 7,864/2,355,138; largest single allocation Join 1024 (cap 1024), JoinAccepted 64,
  JoinRejected 0, Input 0 (the reused Input was sized on the pool's warm-up decodes), Snapshot 0 (cap 0). Plants:
  unchecked index found at input 17 (in a 100,000-per-half run), over-allocation at input 0. Meets the row's count and
  both zeros for the five types that exist; the row's "every message type of the catalogue" and cargo-fuzz are not met
  (started, not passed, as the test's own doc says).
