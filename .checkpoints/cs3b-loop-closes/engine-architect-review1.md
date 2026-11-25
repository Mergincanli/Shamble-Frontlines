# engine-architect, change-set 3b review, round 1 (head bbbf09e4, base 28254a73)

Copy: /home/user/Shamble-Frontlines/.gate/ci-wasm-wt/.gate/engine-architect-cs3b (detached at bbbf09e4).
Memory: none found (.claude/agent-memory-local/engine-architect/ does not exist).

## Plan
1. Read the context (done), my design engine-architect-cs3.md blocks 2 and 3, the plan cs3-plan.md sections 5 and 8,
   the resume card's C1-C8, ADR 0002 parts 3, 5, 6, DECISIONS.md for WB-HEALTH (a).
2. Read the diff by area: sim-wasm (lib.rs, local_match.rs, tests), client-endpoint (layout.rs, layout_ts.rs, block.rs,
   endpoint.rs, post.rs, press.rs), generated layout.ts, crate manifests, sim-worker src + boundary test, client src +
   build-checks.mjs.
3. Run: cargo test for sim-wasm and client-endpoint; cargo tree for edges; build the wasm glue if feasible and read
   its text; the sim-worker and client Node tests; the boundary test if it runs without a browser.
4. Answer Q1-Q7 with evidence; write one line per finding here as found; DONE last.

## Findings (one line each, as found)
- Q1 read: Sim = one field `local: LocalMatch` + host-input checks (lib.rs:59-91); Endpoint (client-endpoint/endpoint.rs:83-99) holds join state, pacer, sequence, sent ring, readout. BUT LocalMatch (sim-wasm/local_match.rs:45-59) keeps the newest post + its by-number guard, the block writer, the local schedule, now_ms, counters: per-client state my design 2.1 put in the endpoint; ADR 0002 part 6 "no code path exists only offline". -> Medium/Low (decide).
- Edges (cargo tree, my run): sim none; protocol none; room->protocol,sim; client-endpoint->protocol,sim; sim-wasm->c-e,protocol,room,sim,wasm-bindgen; devtools->c-e,protocol,room,sim,sim-wasm,wasm-bindgen. room/c-e not on each other, no wasm-bindgen: OK. devtools names c-e, protocol, room and uses none (grep: only `use sim_wasm as _`, wasm.rs:10); Cargo.toml comment claims the replay/L1 use (3c). -> Low.
- Layout: render_typescript derives section starts with its own running sum (layout_ts.rs:25,37), not layout::offsets (layout.rs:130) which the writer uses via section_offset: two derivations; hash does not cover starts or the rule. Boundary test reads echo/link by printed offsets (sim-wasm.test.mjs:33-37,76-84) so drift today would bite there. -> Low.
- Page length check: worker-link.ts:241 exact length, :243 >= BLOCK_END before any read past the header: good. BLOCK_END = LINK.offset+LINK.stride (worker-link.ts:18) names the last section by hand; same in boundary test :46 and types index.test.ts:12. -> Low (print BLOCK_BYTES).
- Native tests in my copy: client-endpoint 25, sim-wasm 12 + panic_hook 1 + step_alloc 1: all pass.
- Tool file checked: sha256 ca4df773...6931, 8170784 bytes = his TOOL-SOURCE values.
- Wasm built in my copy (build.mjs --wasm-only, the checked tool): players' Wasm 56,717 bytes, sha256 ded90598...f4f8 = the card's and CI's (eb0d2314 log line 1202). Boundary tests on it: 2 files, 18 passed. Client Node 81, sim-worker 46: passed.
- Q2 glue (my build, pkg/sim_wasm.js:45-47 input, :88-91 step): one wasm.* call each, no malloc/string/array; constructor alone mallocs (seed, once). Surface: module Sim, default, initSync, panic_buffer, tick_hz; Sim.prototype block_len, block_ptr, constructor, free, input, rebase, step (+ wasm-bindgen's __destroy_into_raw, Symbol.dispose). newest_input gone (grep, .d.ts). Odd JS values (-1, NaN, 1.5, 2^40, +-Inf, undefined, null, "7", {}, []) into input: no trap (my script). Rust side: post.rs:30-50 no indexing/arithmetic.
- Boundary exports test (sim-wasm.test.mjs:139-143) pins module names only, not Sim's methods: a method added to the bindgen impl passes it. -> Low (pin Sim.prototype too).
- Anchor: one writer of a host time = rebase (schedule.rs:135; Pacer ctor pacing.ts:68); LocalMatch::new seeds 0.0 (local_match.rs:72); step never writes it. OK.
- Double guard: host.ts:104-116 is the needed one (only it sees every post; it forwards one per tick). Rust local_match.rs:116-122 never fires on the shipped path; differs at the edges (first post vs Post::NONE=0, post.rs:47-50; half circle); counts into block `refused` (local_match.rs:186-188), a message counter. With the Rust guard, a host that takes the last post to arrive loses/doubles no press: presses.spec's mark assertions (:113-114) pass under that bite; only the counters (:120,:122) catch it. -> Low: one owner (the host).
- now_ms kept with a monotonic guard + bad_times counter that nothing reads, not even the block (local_match.rs:131-135, 41-42; refused sum :186-188 leaves bad_times out). -> Low (dead state).
- Page: preventDefault on game keys outside text boxes (boot.ts:58-61); button blurs on click (overlay.ts:19-20) so Space never re-clicks it; but key-ups whose target is a text box are dropped too, so a key held when focus enters the fallback textarea (overlay.ts:46 select()) stays held. -> Low.
- Production: CI log eb0d2314 lines 1131-1139 vs 1186-1199: keys/overlay/report chunks only in development; one CSS asset (index-CYgNJTPa.css, 746 B) in both, so #key-diagram rules (styles.css:51-71) ship in production like #overlay's; scan reads .js only (build-checks.mjs:163). -> Low/nit.
- No innerHTML in client/src or sim-worker/src (grep). textContent everywhere.
- Q6: NA-N1 done (lib.rs:128; host.ts:80-83); NA-N2 done both sides (host.ts:121-124; boot.ts:64, 91-92; host.test.ts:173-185); NA-N4/N6 present (sim world.rs:18-21, 25-29; netcode judges); NA-N5 done (messages.ts:14-15; post.rs:1-2); NA-N7 done on all three paths (pacing.ts:40-43; host.ts:160-174; worker.ts:56-60; pacing.test.ts:111); EA-F9 partly (see above); EA-F10 bindings done; EA-F13 anchor done; TE-NT7 done (boot.ts:72-77 blockArrived; allocation.spec.ts:51); PA-N2: plan wording met (frame+follow, allocation.spec.ts:49), its own risk not (window still from block 90, :32; floors from 3a, :68-71); FF-WAKES: report carries worker.wakes/ticks/maxTicksInOneWake (report.ts:141-146), early-wake share = (wakes-ticks)/wakes exact only while maxTicksInOneWake <= 1; reading waits for his Firefox step; NA-N3 sentence done (picture.ts:21-23).
- Q7: recording: no obstacle; Sim::create + input + step run natively (lib.rs tests :181-200) = the Worker's code minus glue; record now_ms per tick now (read by slice 3's clock); the new number as u32 halves: u64 crosses as a BigInt (heap object), multi-value returns arrive as an Array (sim_new's [number, number, number], pkg .d.ts). L1: LocalMatch<P: TransportPair> takes it, but has no link accessor (local_match.rs:45-112) and hands now_ms to nobody, so 3c adds link_mut() or a time hand-off; L1 must not duplicate the reliable control channel (a second JoinAccepted panics, local_match.rs:172-174). devtools edges: 3c needs protocol (TransportPair) and client-endpoint (Post); room only if named.
- Verdict: CHANGES NEEDED (one Medium: EA-F9 partly done, fix or record; rest Low).
DONE
