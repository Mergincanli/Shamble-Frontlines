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
- Debug suites (mine): protocol 19 + decoder 8, room 12, client-endpoint 25, sim 17 + 6, sim-wasm 12 + 2: all pass.
- Q1 read: one wire form per message holds (codec.rs:156-405; every field read, finish() refuses trailing bytes,
  unknown flag bits refused, sequences implied consecutive and gaps 1..255 so encode(decode(b)) == b on everything
  taken, and encode is injective so decode(encode(m)) == m). Traps before online play: acks have no per-group
  uniqueness and the room bounds an ack only from above (lib.rs:173-185); the Join's version sits after the token
  (codec.rs:9, 159-163); an encode failure in build_input sends nothing and is counted nowhere (endpoint.rs:228).
- Q2 read: no plain arithmetic or index on a decoded value in codec, room, queue, endpoint, pacer, loopback; all work
  bounded (<= 8 commands per Input, queue 16, <= 16 take_due per step). LocalMatch::step keeps an infinite host time
  (local_match.rs:131: `now_ms >= self.now_ms` takes +inf), after which every finite time counts as bad (Low).
- Q3 candidate: before the queue's first take (`taken == None`, queue.rs:69-74) sequences may spread past half the
  circle; the order is then cyclic and a repeated sequence is stored twice (to be shown by a probe).
- Q4 read: the pacer's +1 per command and the endpoint's +1 tick per command (endpoint.rs:224-226) give presses
  advanced <= sequence distance = tick distance between any two of an honest client's commands, so admit
  (lib.rs:219-235) never refuses one, whatever is lost, repeated or reordered (to be shown by a probe over a toy link).
- PROBE crates/room/tests/nc_probe_queue.rs (my copy, untracked): before the queue's first take, inputs with
  sequences 0, 40000, 20000, 0 -> ran 0, 20000, 40000, 0 (commands_run 4, duplicates 0): one sequence ran twice
  (queue.rs:69-96; hostile client only, nothing gained: admit still holds ticks and presses). After a first take the
  repeat is caught (duplicates 1). Outage of 45 commands (> QUEUE_LEN, > cap): no stall, presses 200 = 200, refused 0,
  missing 38 (the built queue beats my design's TooFarAhead ring here). 1M random inputs: no panic, <= 2 run per tick.
- PROBE crates/sim-wasm/tests/nc_probe_link.rs: real LocalMatch over a toy TransportPair (delay 2+-1 and 3+-2 ticks,
  5 % loss, 1 % dup, outages 45 and 100 ticks; start tick 131,069, first sequence 65,500): 10,000 presses applied
  exactly in 7 runs, room refused 0, pacer refused (0,0), lost 0 doubled 0; but endpoint `refused` 712-1,976 per run
  on an honest link (stale or doubled snapshots, endpoint.rs:233-242), summed into the block's refused
  (local_match.rs:182-189).
- PROBE crates/client-endpoint/tests/nc_probe_endpoint.rs: 32,768 commands after the last confirmation, each Input
  carries 0 commands, not even the newest (endpoint.rs:204-209): a stall the room cannot end (it confirms only what
  arrives). 1M taken snapshots with random fields and 1M mutated ones: no panic.
- Clippy (mine) on the libraries of protocol, room, client-endpoint, sim-wasm: 0 warnings with the denied lints.
- Plants (read, not rerun): .gate/cs3b/plants-3b-run1.log "34 of 34 caught"; wire-unchecked (the real reader made
  unchecked) caught by the decoder run at input 1.
- Q5 read: local_match.rs:148-191 is ADR 0002 :402-407's order; step 1 Join -> JoinAccepted -> first snapshot
  (reply queued before the snapshot, so the endpoint is Joined when the snapshot arrives); step 2's command for T
  reaches the room before T runs; host.ts:80-82 makes one input call then step per tick.
- ADR 0001 :583-584, :596-598: every L1 run reorders 1 % of adjacent snapshot pairs (P0 included), so F1 below holds.
FINDINGS (final)
- F1 Medium endpoint.rs:233-242 + local_match.rs:182-189: honest-link stale/early snapshots counted as refused and
  summed into the block's one "refused"; 712-1,976 per run on my toy link; 3c's L1 makes it non-zero by construction.
- F2 Low queue.rs:69-96: no window before the first take; sequence run twice (probe).
- F3 Low endpoint.rs:204-209, :228: newest command not sent 32,768 commands after the last confirmation (probe), or
  on an encode error (uncounted); unreachable in 3b.
- F4 Low press.rs:3 vs :31-32/:46: "never dropped" vs the 255 cap; bounded departure from (a)'s "nothing is ever
  lost"; shows on the page as lost (report.ts:26-29, press-tally.ts:51-57).
- F5 Low traps: acks bounded only from above, two per group allowed (lib.rs:173-185); Join version after the token
  (codec.rs:159-163); press check vacuous at 255 covered and narrow counters wrap on long gaps (lib.rs:229).
- F6 Low local_match.rs:131 keeps +inf.
- F7 Low (3c) L1 needs a clock handle (no link accessor) and an exactly-once reliable channel (local_match.rs:171-177).
VERDICT: PASS (no Critical or High; F1 fixed in this pass or as 3c's first step, before L1's 0-refusal mark).
DONE
