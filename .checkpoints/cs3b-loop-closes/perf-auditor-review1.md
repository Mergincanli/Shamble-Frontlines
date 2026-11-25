# perf-auditor, cs3b round 1 (head bbbf09e4, base 28254a73)

Copy: /home/user/Shamble-Frontlines/.gate/ci-wasm-wt/.gate/perf-auditor-cs3b (detached at bbbf09e4).
Baselines from memory: cs3a (Worker per wake 410.81/421.56 B, ~186-192 ns; page per block Node 280/289.8 B spec window;
allocation.spec runner per block 487..539; Wasm 31,718 raw at 3a fix pass).

## Plan
1. Read the brief's files in my copy: host.ts, pacing.ts, messages.ts, worker.ts, local_match.rs (+ what it calls:
   loopback, codec, room, endpoint), boot.ts, worker-link.ts, keyboard.ts, press-tally.ts, overlay/keys.ts,
   overlay/report.ts, allocation.spec.ts, loop.spec.ts; ADR 0002 part 4 (PM-0002-13, size marks).
2. Q1 Worker per tick: Node harness on the real SimHost + Wasm (cs3a method: harness loop never-optimised, sampling
   heap profiler interval 1, heap delta with big semispace), base vs head; ns per tick and B per tick by allocator.
3. Q2/Q3 page per frame / per block: Node harness on the real WorkerLink with structuredClone'd messages (cs3a method),
   base vs head; then Chromium (CDP sampling, interval 1) if time allows; build the page in my copy and run
   allocation.spec.ts N times (port 4173 free check before each run) for the spread.
4. Q4 pool misses: arithmetic from host.ts pool code + the loop spec's poolMisses; bytes per second at 6/10/30/60 fps.
5. Q5 key-to-tick: read keyToTick, model frame wait + tick wait + hop at 60 Hz (uniform phase arithmetic + a small Monte
   Carlo), compare with PM-0002-13's 50 ms p95; cheapest change.
6. Q6 Wasm size: build players' Wasm, twiggy-like section split if tools exist, compare with ADR 0002 part 4 marks.
7. Answer (<= 40 lines), save baselines to memory.

## Findings (one line each, as found)
- [evidence sizes, my builds, dev page] head bbbf09e4: players' Wasm 56,717 / 23,023 gzip / 19,497 br (= rust-reviewer's); Worker chunk 9,255 / 3,659 / 3,312; index 829,991 / 223,583; keys chunk 690; report 3,095. Base 28254a73: Wasm 31,718 / 12,642 / 10,755; Worker 8,853 / 3,477; index 826,210 / 222,217; report 2,374. So 3b: Wasm +24,999 raw (+78.8 %), +10,381 gzip, +8,742 br; Worker chunk +402 / +182; index +3,781 / +1,366.
- [read] PM-0002-13 (ADR 0002:1284): from the key event's `timeStamp` to the start of the tick that applies it, <= 33.33 + 16.67 = 50.00 ms at p95, ML 60 Hz panel, BX N = 48. ADR part 5 (:318-329) bounds the design's wait by one frame + one tick + delivery, i.e. the mark sits AT the design's worst case. 3b's reading starts at the handler's clock() (boot.ts:60), not at event.timeStamp.
- [evidence Q1, Node 22.23.5 harness .gate/perf-auditor-cs3b/.gate/perf/worker-tick-bench.mjs: real SimHost + real built Wasm, one tick per wake, buffer returned each wake, harness loop never optimised] time, 5 x 1M wakes, 2 runs: base 268.0 / 279.5 ns median per tick, head 651.7 / 648.1 ns (+~375 ns = the new Wasm step; rust-reviewer's 535-572 ns for input+step), head with press words >= 2^31 663.1 / 719.6 ns. Load 1.75 on 4 vCPU.
- [evidence Q1 bytes] heap delta, 512 MB semispace, 3 x 500k: base 421.50 B/tick (= cs3a's 421.56 mode); head 445.45 or 450.77 B/tick (two JIT modes, each exact over 3 reps): +24 to +29 B/tick = +0.7 to +0.9 KB/s at 30 Hz. Big press words: +0 in the host (the extra 32 B is the harness's own word()). Sampling interval 1: copyBlock 192 (pre-existing, both), #wake 157.3 -> 170.7 (+13.4: the block message's startAt field and its number box), new: host's step closure 16.0 (its f64 return boxed: --trace-turbo-inlining shows sim_input/sim_step Wasm bodies never inlined, 300 and 179 > 30). No allocation from the six input numbers.
- [evidence Q5, .gate/perf-auditor-cs3b/.gate/perf/key-to-tick-sim.mjs: real KeyboardInput + WorkerLink + SimHost + Pacer + built Wasm on one virtual clock; frames on a 60.000 Hz grid; 10,000 key events (5,000 presses of 6 keys, held 40-120 ms, 80-320 ms apart); timeStamp->handler d = 1 ms, post hop 0.15-0.45 ms, Worker timer +0-0.6 ms (lateness p50 0.8, p99 1.5); each event timed to the first block with post_used >= its carrying post] 60 fps, 16 phases: per-run p95 from timeStamp 34.8..48.0 ms (median of runs 39.8), p50 19.6..33.1, max 52.4; 0 of 16 runs over 50.00. The phase is locked within a run (60 = 2 x 30), so a run's p95 depends on where frames fall against ticks; worst phase leaves 2.0 ms. The link's own Ring (600 newest, one change at a time) reads the same within ~1 ms.
