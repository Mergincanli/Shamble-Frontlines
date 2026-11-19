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
- B1 read: build plan card 316-333, Step 2.5 table 463-476 read. ADR 0001: command contents :225-239 (tick, seq, move,
  buttons, press counter per button, aim point <=80 bits, view time; ack of newest snapshot decoded; server checks incl.
  "press counter advances by at most one edge per button per tick covered", "trailing bytes or unknown fields dropped
  whole", seq wrap handled); catalogue :682-686 (Join: token, protocol version, content hash; JoinAccepted: slot, team,
  squad, mode id, mode hash, content hash, server tick, tick and snapshot rates; Snapshot header: 32-bit server tick,
  baseline tick, newest command received, last processed command, group index); transport interface :441-446 (send
  unreliable, send reliable on a named channel, receive, link state and RTT; decoders never panic, check sizes before
  allocating; sim thread never parses raw bytes); profiles part 13 :583-600 (P0..P4, B1; half latency and half jitter
  each way, uniform jitter; loss per direction at the profile's rate; explicit reorder 1 % of adjacent pairs);
  counter-wrap test :1823-1827 (start server tick a few below a multiple of 65,536; every 16-bit counter near wrap).
- B1 read: ADR 0002 crate rows :186-194; lints :462-474 (protocol decoders: unwrap_used, expect_used, indexing_slicing
  at deny; room timers count ticks; client endpoint takes time only as host values); control API :330-346 (one input
  call + one step call per tick; input call = newest input + datagrams with arrival times + previous send time + host's
  current time; step returns where outputs lie and when next tick is due; <= 4 Wasm calls per tick incl. glue); part 6
  :402-407 the step order (endpoint builds T's command + predicts T; command via loopback; local server runs T with
  every slot's command, encodes T's snapshot; endpoint decodes, reconciles, writes block); L1 :520-526 (in one process,
  real server code + headless bot clients); PM-0002-49 :1323 (2 runs identity; 1 per profile stats; 1,000,000 packets
  per profile per direction; within 5 % relative of per-direction values of P0-P4 and B1; G11's one-minute run per
  profile under 2 minutes total; "jitter read against the distribution L1's own documentation states"); note :1357-1360
  (1,000,000 packets: band = 5 standard errors at 1 %); PM-0002-53 :1327 (planted u16 overflow with + panics in dev and
  release, native and Wasm; wrapping_add equal in all four); PM-0002-64 :1338.
- FINDING F-P0: PM-0002-49's "within 5 % (relative)" is undefined at P0 (0 ms, 0 jitter, 0 % loss): a relative band
  around zero is zero width. Needs an absolute reading at P0 (exactly 0 lost, delay exactly 0 ticks/us). (ADR 0002 :1323)
- FINDING F-B1: B1's "one burst of 1 s at 100 % loss" inside 1,000,000 packets: loss over 1,000,000 packets is then
  P2's 2 % plus the burst's share, which depends on the packet rate; the mark must say what B1's expected loss is
  (per-direction P2 loss + burst packets / total). (ADR 0001 :585, ADR 0002 :1323)
- B1 code facts (main f5030a3 = this branch's crates): protocol has messages.rs (Join{token Vec<u8>, protocol_version
  u16, content_hash [u8;32]}, JoinAccepted{slot u8, team u8, squad u8, mode_id String, mode_hash, content_hash,
  server_tick u32, tick_rate_hz u8, snapshot_rate_hz u8}, Input{acks Vec<SnapshotAck{group u8, tick u32}>, commands
  Vec<Command>}, Command{tick u32, sequence u16, move_x i8, move_y i8, buttons u16, presses [u8;16], aim AimPoint{x i16,
  y u16, z i16}, view_time ViewTime{tick u32, fraction u16}}, Snapshot{server_tick u32, baseline_tick Option<u32>,
  newest_command_received Option<u16>, last_processed_command Option<u16>, group u8}); no codec; ts_export.rs writes the
  4 messages' TS into packages/types/src/generated/protocol (any field change regenerates it). protocol has NO deps.
  room and devtools: empty stubs. client-endpoint: no deps; block.rs (BlockWriter: header tick + layout_hash only),
  layout.rs (BLOCK_LAYOUT header stride 8), schedule.rs (LocalSchedule f64 anchor; next_due_ms; ticked; rebase).
  sim: World{tick u32, seed [u8;32]}, World::new(seed) at tick 0, authority_step() checked_add, write_state (tick then
  seed, 36 B), state_hash fnv1a_64 (provisional until PM-0004-21's report, cs2 report risk 5: "before the first
  recording is committed (change-set 3)"). sim-wasm: Sim{world, writer, schedule, newest_input u32}; wasm calls:
  new(seed, mode, pad, now_ms), block_ptr, block_len, input(sequence u32), newest_input() (tests only, EA-F10), step()->f64
  (NO now: NA-N1), rebase(now)->f64, tick_hz(). sim-worker: host.ts (HostSim{input(sequence), step(), rebase, block_ptr,
  block_len}; Pacer step = input(newestSequence) then step(); visibility hidden -> pacer.pause()), pacing.ts (Pacer: no
  finite check on the due time step() returns: NA-N7; while loop `nextDueMs <= now` with NaN never runs and returns NaN
  sleep -> Math.ceil(NaN) -> setTimeout(NaN) = 0 ms busy loop), messages.ts (input = {sequence, returned}; isCount
  0..2^32-1). Page: boot.ts:120-123 `sequence` per drawn frame (NA-N5: rename to post number); worker-link.ts:121
  frame(sequence).
- B1 NA-N1..N7 text (STATUS_ARCHIVE :1085-1091): N1 step gets the host's time; N2 hidden-tab pause from the mode once
  online lands, held buttons cleared on hide; N3 who picks the blend fraction (one sentence in picture.ts now, a field at
  cs4; events read from every block); N4 seed = authority-only section written last, shared state a prefix; N5 page's
  per-frame number is not the command sequence: name it the post number; N6 a start tick in the authority's match
  setup; N7 Pacer throws on a non-finite due time; plus: checked_sub or wrap-aware compares on decoded ticks.
- B1 ADR 0004 part 8 :309-332 (seed from host, never sent/logged; streams' state part of authority's state + hash;
  every stream starts authority-only); part 9 :338-356 (recorded match = seed, map and mode data, content hash, per-tick
  inputs; first mismatching tick + field-level diff; PM-0004-21 picks the hash function and its cost per tick).
- FINDING F-ECHO: the card's row (build plan :249) wants keys "echoed back by the local server through the real
  messages" and "zero lost presses", but the Snapshot type has a header only (no own-state section). An echo needs the
  authority to hold the slot's applied input (buttons + press totals) in hashed state and the snapshot to carry it in a
  minimal owner section; otherwise the first recording hashes only tick + seed (inputs change nothing, so "same hashes
  natively and in Wasm" proves nothing about inputs).
- FINDING F-PRESS: the page posts only a per-frame number today; a tap shorter than a frame is lost unless the page
  counts presses cumulatively per button (keydown, not repeat) and posts the running counts; the endpoint must then pace
  them out at most one edge per button per tick (ADR 0001 :235's server check), carrying any backlog to the next tick.
- B1 DONE. Next: Block 2 (design, crate by crate).
