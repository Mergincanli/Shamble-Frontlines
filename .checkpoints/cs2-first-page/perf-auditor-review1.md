# perf-auditor review 1, cs2 first page (head 30a5a13, worktree .gate/cs2-plants)
Started 2026-11-19T07:16:24Z
Plan: read plan s5/s9 + resume card steps 29/35/36/39 + ADR 0002 p5 / 0004 p5 marks; read the 18 files; run unit tests, readings.mjs, overflow-cost.mjs, allocation.spec (x3+) and a Node micro-harness for Ring.percentile and the tick path; write each finding here as made; answer Q1-Q8.

## Findings (appended as made)

- 07:24:03Z read: plan s5/s9, card steps 28-41, ADR 0002 part 5 + PM-0002-3/6/7/8/26 rows, ADR 0004 part 5 + PM-0004-68/69/70 rows, AUDIT_BUGS_PERF, the 18 files at 30a5a13, plants.py/planted-defects.md, the first three reviewers' notes (no repeats). No perf-auditor memory exists (.claude/agent-memory-local/ holds only security-auditor): no earlier v2 baselines.
- Hypotheses to test: (H1) the pacer's setTimeout delay is fractional; WebIDL long truncates it and the 4 ms nested-timer clamp then makes most ticks ~4 ms late (fits lateness p50 3.7-3.8 ms); (H2) the sampling heap profiler attributes inlined callees to the physical caller, so by-name sums under frame/onMessage miss allocations once boot.ts's arrows tier up; event.data deserialization is outside onMessage; (H3) delivery p50/p95 = half/0.95 of the ~116 ms frame (uniform arrival).

### P1 (Medium, fix before merge) the pacer's fractional sleep makes ~half the ticks ~4 ms late (07:27:00Z)
pacing.ts:106 returns Math.max(0, nextDue - now) (fractional) and host.ts:134 passes it to setTimeout. WebIDL types the
delay as long (truncated toward zero), so the timer fires up to 1 ms before the due time; wake() then finds no tick due
(pacing.ts:92) and sleeps again for ~0.1-0.9 ms, which the nested-timer clamp (nesting level > 5: at least 4 ms) turns
into ~4.2 ms. Measured (scratch .gate/perf-auditor-pacing.mjs, copy in this folder; Chromium 153.0.8010.12 headless,
Playwright default launch, 07:25-07:26Z):
- real page (worktree dist, pad 0): 617 ticks, 1011 wakes (1.64 per tick), lateness p50 3.70, p99 7.57, max 22.17 ms.
- pacer logic alone in a blob Worker on a blank page, 600 ticks, two rounds: as the head (fractional delay) 1.48 / 1.49
  wakes per tick (285 / 295 wakes found no tick due; their sleeps asked p50 0.10 ms, got p50 4.20 ms, p99 5.30 / 4.80),
  lateness p50 0.23 / 0.30, p99 4.40 / 4.30 ms (over PM-0004-68's 4 ms on an idle page); with Math.ceil(delay) 1.00
  wakes per tick, lateness p50 0.73 / 0.70, p99 1.40 / 1.47, max 4.93 / 3.50 ms.
So the container's lateness reading (p50 3.7-3.8, p99 6.7-9.2) is mostly self-inflicted, not W2's jitter. Smallest fix:
setTimer(this.#wake, Math.ceil(sleep)) (host.ts:134) or return Math.ceil in pacing.ts:106; a pacing test with a clock
that wakes 0.5 ms early pins it (no empty wake). Retake the lateness reading after the fix.

### Q3/Q4 note: delivery = the page's frame, confirmed (same run)
At 5.5 fps (frame p50 183.3 ms; three browsers on 4 vCPUs) delivery p50 96.0 = 0.52 x frame, p95 185.8 = 1.01 x frame:
a block waits for the main thread's current frame task, uniform over the frame (the card's 58-63 / 112-119 ms at 8.6 fps
is 0.5 / 0.97 x 116 ms). Pool misses: 175 of 617 ticks at 5.5 fps (5.5 ticks per frame, more than the free list's cap of
4 per return) against 4-11 per 600 at 8.6 fps (3.5 per frame): misses follow ticks per frame, not the pool size.

### A1 (Medium, fix before merge) allocation.spec's by-name sums lose the bytes once the callers tier up (07:30:07Z)
allocation.spec.ts:17-18, 37-38 sum the profile under functionName "frame" and "onMessage". Chromium's sampling heap
profiler records physical frames only, so a method inlined into an optimized caller is billed to the caller. Measured on a
mirror page (scratch .gate/perf-auditor-inline.mjs and -inline2.mjs, copies here; Chromium 153.0.8010.12, default launch,
samplingInterval 64, collected objects included, 07:27-07:30Z), a class with the same frame() body called once per task:
- first 300 calls: 72.6 / 78.4 B per call under "frame" (the real page's clean 63-81);
- after 2,000 to 60,000 calls: 1.3-12.5 B per call under "frame"; the caller holds 54-56 B (ch.port1.onmessage self
  1,124,816 B over 20,000 calls); the move starts between 300 and 1,000 calls;
- with the planted Uint8Array(64): "frame" reads 48.3-54.7 B per call after >= 2,000 calls (limit 170: the plant passes);
  the caller holds 202-210 B.
- event.data's deserialization is billed to the listener that reads it, not to onMessage: 297-302 B per block (89,308 B
  over 300 cold; 6,034,740 B over 20,000 hot) against 49-52 B under onMessage; PM-0002-7 counts "the browser's message
  event per block" in the boundary's scope. boot.ts:40-41 reads event.data in an anonymous arrow: outside the sum.
- a Uint8Array(4096) per call adds ~111 B per call to the profile (185 vs 74 clean) and a Uint8Array(64) ~187 B: backing
  stores over 64 B live off the V8 heap and the sampling profiler never sees them, so "MB/s" under-counts such a leak ~40x.
Why the container passed: at 5.5-8.6 fps the whole spec makes ~150 calls of the frame callback (300-block wait + 8 s), so
nothing tiers up; at 60 Hz the 10 s wait alone is 600 frames. Smallest fix now: a floor (perFrame and perBlock each >= ~30 B,
the clean message and view must be seen) so the spec fails instead of passing when the bytes leave the names; say in the
spec that the by-name method holds only unoptimized. For CS5: an A/B (the same page with the hand-off off) on total page
bytes, or caller-level attribution checked against the inlining, plus an off-heap count (ArrayBuffer constructions).

### Q1 reading: the Worker's tick path allocates ~280 B per tick hot, ~1 KB cold (07:32:19Z)
Scratch .gate/perf-auditor-tick-alloc.mjs (copy here): the real host.ts/pacing.ts/block-pool.ts/read-block.ts and the
built Wasm (30a5a13) in Node 22.23.4 (V8 12.4), env stub that allocates nothing per tick, in-process sampling heap
profiler (interval 32, collected objects included). Tick plus one input message returning one buffer: cold (first 300)
965.9 B per tick (#afterTick 215.3 self, Uint8Array 193.6 = the two views, isPageToWorker 79.9, for-of iterator
next 78.3 + values 48.0, wake 41.2, the stub's own array push 155.0); hot (20,000 after 100,000) 501.6 B per tick, of
which 276.1 inside Pacer.wake (the tick path, all inlined) and 224.9 in the driver (inlined onMessage + stub). Input
message alone (no buffer returned): 152.1 B cold, 57.4 B hot (the .every closure in messages.ts:83 and the for-of
iterator in host.ts:87). So beyond the two views and the message object, each tick also allocates the transfer list
[buffer] (host.ts:143) and boxed doubles (doneAt, timeOrigin + doneAt, the copy-time and lateness differences; all
boxed while unoptimized). 30 Hz x ~280 B = ~8 KB/s in the Worker: harmless (not PM-0002-7's scope, which is the page).
Cheap trims if ever wanted: one reused transfer array and message object (postMessage serializes synchronously).
Pool: 120,336 ticks, 0 misses when the page returns each buffer within a tick (stub).
