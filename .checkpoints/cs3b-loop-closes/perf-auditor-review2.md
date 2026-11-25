# perf-auditor, cs3b round 2 (head 0dfca749; round-1 head bbbf09e4; base 28254a73)

Copy: /home/user/Shamble-Frontlines/.gate/ci-wasm-wt/.gate/perf-auditor-cs3b2 (detached at 0dfca749).
Round-1 copy (bbbf09e4): .gate/perf-auditor-cs3b; its harnesses in .gate/perf-auditor-cs3b/.gate/perf/
(worker-tick-bench.mjs, link-bench3b.mjs, key-to-tick-sim.mjs). Round-1 notes: perf-auditor-review1.md (Q1, Q5, sizes done).

## Plan
A1. Q2/Q3 page bytes per frame and per block on the fixed head: link-bench3b.mjs (real WorkerLink, structuredClone'd
    messages, never-optimised harness loop, sampling interval 1 + heap delta), base bbbf09e4 vs head 0dfca749;
    Chromium (CDP sampling) if time allows.
A2. Q4 pool misses: arithmetic from packages/sim-worker/src/block-pool.ts + host.ts; B/s at 6/10/30/60 fps; defect or
    expected; cheapest change.
A3. Q6 players' Wasm 53,783: build in my copy, section split (name/data/code, per-function where tools allow) vs
    main 31,718 and bbbf09e4 56,717; compare with ADR 0002 part 4's size marks.
B1. Fix pass cost, page: worker-link.ts (#done/#doneCount, slice per frame, olderPosts), press-tally.ts
    (heldWithoutPress): B per frame, B per block, ns per block, base vs head (same harness).
B2. Fix pass cost, Worker: local_match.rs + endpoint.rs: worker-tick-bench.mjs, ns per tick vs 650, B per tick vs
    445..451, base vs head.
B3. boot.ts key-to-tick from event.timeStamp: read the change; re-run key-to-tick-sim.mjs against head code; does the
    conclusion (p95 at 60 fps < 50 ms in every phase) still describe what the page measures?
C.  The per-block allocation vs frame-rate puzzle: what blockArrived (boot.ts) and its callees allocate per block at
    ~1 vs 3-5 blocks per frame (Node harness with controlled blocks per frame; Chromium if possible); then judge
    allocation.spec.ts's answer (refuse < 2.5 blocks/frame, per-frame limit follows blocks per frame).
D.  Browser runs only one at a time, port 4173 checked free first; test-engineer goes first.
E.  Answer (<= 30 lines) via SubagentHandback; baselines to memory (.claude/agent-memory-local/perf-auditor/MEMORY.md).

## Readings and findings (one line each, as found)
