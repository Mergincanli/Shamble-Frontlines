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
