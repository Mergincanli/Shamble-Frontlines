# engine-architect, change-set 3b review, round 2 (head 0dfca749; round 1 head bbbf09e4)

Copy: /home/user/Shamble-Frontlines/.gate/ci-wasm-wt/.gate/engine-architect-cs3b2 (detached at 0dfca749).
Memory: none for engine-architect (.claude/agent-memory-local/ has security-auditor and perf-auditor only).
Read: my round-1 notes, review-context.md, review-context-2.md, the card's "Open findings" (EA-1..EA-9 there).

## Plan
1. Diff `git diff bbbf09e4 0dfca749 -- crates packages` by area: client-endpoint (endpoint.rs, layout.rs,
   layout_ts.rs, block.rs, post.rs, press.rs, lib.rs), sim-wasm (lib.rs, local_match.rs, tests/any_bytes.rs),
   generated layout.ts + types index.ts, sim-worker (host.ts, boundary test), client (worker-link.ts, boot.ts,
   keyboard.ts, press-tally.ts, report.ts), e2e report.ts.
2. Q1 EA-1: read Endpoint's new API and LocalMatch's fields; list per-client state still in LocalMatch; judge
   write_block(refused_elsewhere); Readout late/unsent. Against ADR 0002 parts 3, 5, 6 (read them).
3. Q2 EA-2: grep every place the block's shape (offsets, strides, total) is stated: Rust, generator, generated TS,
   page, worker, boundary test, types tests.
4. Q3 EA-3: post contract stated on both sides (post.rs, sim-wasm lib.rs, host.ts).
5. Q4 EA-4/5/7: keyboard.ts + boot.ts caller; step(_now_ms); boundary test pin (Sim.prototype names).
6. Q5 the page additions: olderPosts, #done/#doneCount/slice per frame, heldWithoutPress, report fields,
   timeStamp: side of boundary, dev-only reaching production (grep import graph; no page build this round).
7. Q6 any_bytes.rs placement vs the dependency rules (cargo tree, manifests unchanged: git diff of Cargo.toml).
8. Q7 3c: what the split helps/blocks; shared_hash L3: what the export returns, where the buffer lives.
9. Run: cargo test -p client-endpoint, -p sim-wasm; clippy workspace all-targets; npm ci offline; vitest client,
   sim-worker (--dir src), types; build wasm only if needed for the glue (no page build).
10. Answer: <= 30 lines, VERDICT first, EA-1..EA-9 one line each, new findings, Not verified.

## Findings (one line each, as found)
- Copy made at 0dfca749. Diff read: 33 files; sim-wasm/src/lib.rs and sim-worker/src/host.ts unchanged by the fix pass; no Cargo.toml/Cargo.lock change (git diff --stat: none).
- cargo test --locked -p client-endpoint -p sim-wasm -p room (my copy): 26; room 13; sim-wasm 13 + any_bytes 2 + panic_hook 1 + step_alloc 1: all pass (= the card's counts).
- EA-1: Endpoint now holds post (endpoint.rs:95), writer (:109), set_post (:158), block/block_len (:164-171), write_block(refused_elsewhere) (:175-178), build_input(out) (:226). LocalMatch keeps room, endpoint, link, schedule, incoming/outgoing, unsent (local_match.rs:35-45). Fixed. Still per-client in LocalMatch: the schedule (LocalSchedule, the client's tick clock; already a client-endpoint type) and the client half of `unsent` (one counter for both ends' sends, local_match.rs:122-142); online needs both on the client side. -> note for 3c, not a 3b defect.
- write_block(refused_elsewhere) (endpoint.rs:173-178): the block's `link.refused` means "all refusals this process sees" (layout.rs:82-83 says so: "by either side, in local play"); online it narrows to the endpoint + its link. Honest seam, documented; a typed second count would be cleaner when online arrives. Not a finding now.
- New naming hazard: Readout.unsent (endpoint.rs:74, encode failures, never in the block) vs LocalMatch::unsent (local_match.rs:43, link refusals, summed into block refused). Same word, two meanings, side by side in clean() (local_match.rs:209,215). -> Low (wording).
- EA-2: end_of (layout.rs:115-119) used by offsets (:128) and render_typescript (layout_ts.rs:39); BLOCK_BYTES printed (layout.ts:40) and used by worker-link.ts:254 and boundary test :46. Remaining restatements are tests' golden values (layout.rs:438-439, block.rs:106, local_match.rs:385 [36..40], types index.test.ts:12). Fixed.
- EA-3: host.ts:44-46 states the rule and :104-116 keeps it; post.rs:1-4, endpoint.rs:156-157, local_match.rs:100-101 state "takes what it is given"; boundary test :86-92 pins last-handed-wins. sim-wasm lib.rs:118-120 (the bindings, unchanged) says only "the page's newest input, once per tick before step": does not say the Wasm takes the last one whatever its number. -> Low (one sentence). Owner right: only the host sees every post, and the host is mode-agnostic (no offline-only path).
