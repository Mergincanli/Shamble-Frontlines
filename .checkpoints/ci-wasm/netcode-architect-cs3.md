Task: change-set 3 ("The loop closes"), the netcode half of its plan (netcode-architect, part "cs3").

## Plan (blocks, in order)
- Block 0: cs3-context.md read (done before this file was made).
- Block 1: Read: card + Step 2.5 table (build plan 316-333, 463-476); ADR 0001 by grep (join exchange, command + press
  counters, snapshot, transport interface + loopback, L1, Worker step order); ADR 0002 (crate list, PM-0002-49/-50/-53/-64);
  code: protocol (messages.rs, ts_export.rs), room lib.rs, client-endpoint (block.rs, layout.rs, schedule.rs), sim
  (world.rs, state_hash.rs), sim-wasm lib.rs, devtools lib.rs, packages/sim-worker/src (host.ts, pacing.ts, messages);
  cs2 report; STATUS_ARCHIVE PR #61 block "For change-set 3" (NA-N1..N7).
- Block 2: Design crate by crate (protocol, room, client-endpoint, Worker step, devtools/L1).
- Block 3: Tests that prove it.
- Block 4: Size and cuts.
- Block 5: Where the build plan / drafts are wrong; questions for Mustafa.
- Then DONE, then answer.

## Notes (one line per finding)
