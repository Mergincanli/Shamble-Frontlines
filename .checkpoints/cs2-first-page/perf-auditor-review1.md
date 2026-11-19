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
- A1 confirmed on the real page (07:34:17Z; scratch .gate/perf-auditor-alloc-warm.mjs, copy here; the spec's
  exact method, worktree dist): after the spec's wait (106 frames so far) 74 B per frame under "frame", 115 B per block
  under "onMessage"; after 150 s more (942 frames so far) 0 B per frame under "frame", 95 B per block under onMessage.
  The listener boot.ts:40-41 (index-DaT5vCul.js:541:48354, e=>{let t=u.onMessage(e.data)...}) holds 92,596 / 88,352 B
  self per 8 s window (~270-360 B per block: event.data's deserialization, plus ~32 report messages), outside the sum.
  So the spec counts ~30 % of the boundary's per-block bytes (~370-415 B per block, ~11-12 KB/s at 30 Hz; PM-0002-7's
  0.1 MB/s is still ~8x away).
- Side reading (three's own per-frame allocation, WebGL 2 fallback, empty scene): whole page 211,928 B/s at 5.75 fps
  before tier-up (~37 KB per frame; top: multiplyMatrices, invert, applyMatrix4, getMaxScaleOnAxis) and 77,000 B/s at
  5.9 fps after (~13 KB per frame; getCacheKey, _bindUniforms, updateForRender). At 60 fps that is ~0.8 MB/s from a
  3-draw scene: a number for cs4/cs6 to track per object, not a cs2 finding.

### Q4 readings: the shared clock holds; the per-copy reading cannot resolve the copy (07:35:48Z)
Scratch .gate/perf-auditor-clock.mjs (copy here), Chromium 153, blank page, not isolated: 400 Worker stamps 5 ms apart,
page minus Worker on timeOrigin + now(): min 0.000, p50 0.100, p95 0.400, max 4.700 ms, 0 negative; both clocks step
0.100 ms. So no offset between the two contexts; the method's error is about +-0.2 ms, far under PM-0002-8's 16.7 ms.
The copy's real cost, batch-timed in a Worker (both views made each time, 5 batches): 8 B 76 ns, 1,032 B 67 ns, 4,104 B
94 ns, 16,392 B 400 ns, 65,544 B 1,510 ns. The readings' "copy 0.000 p50 / 0.100 p99 ms at every size" is the clock's
step, not the curve: report it as "below 0.1 ms (the clock's step)", and take the curve by batch timing or on the
isolated build (5 us steps).
- L1 (Low) host.ts:139-143 / messages.ts:28-29: doneAt is stamped after the copy, while PM-0002-8 starts the clock when
  the step returns, before the copy, and messages.ts says "when the Worker finished the step". Numerically nil today
  (copy <= 1.5 us at 64 KiB); fix: post timeOrigin + before (or stamp in Pacer.wake right after step()).

### Q7: Ring.percentile cost (same time)
Scratch .gate/perf-auditor-ring.mjs and -readout.mjs (copies here). Node 22 (V8 12.4): Ring(600).percentile 37.8 us
per call (36.2-39.2, 7 batches of 20,000); Ring(18,000) 1,808 us. Chromium 153: copy + sort of 600 doubles 25.1 us
(24.2-28.2); the page's whole report function (boot.ts data(): 5 percentiles, runHeader, counters) 66.0 us (61-72).
Per readout update: ~0.1 ms of JS on the page, ~0.1 ms in the Worker (4 sorts in stats()), 4 times a second: fine.
At a full run's 18,000 samples a sort costs ~1.8 ms: compute it once at the end, or keep a 0.1 ms-bin histogram.

### Q6: overflow-cost.mjs A/A (07:36:22Z)
The main session's script, unchanged, run 5 times with the same built Wasm in both arms (two folder copies, sha256
f4758211... for both; wrapper .gate/perf-auditor-aa.mjs, copy here; load average ~2 on 4 vCPU, other reviewers' browsers
running): "with" minus "without" medians +0.02, -0.51, +0.60, -0.14, +0.20 ns (17.3-18.7 ns per tick; per-run maxima up
to 28 ns). The reported +0.67 ns (14.71 vs 14.04, +4.8 %) is the size of the method's own A/A spread (+-0.6 ns, ~3.4 %),
so "nothing measurable" is the right reading; the honest form adds the floor: "below this method's resolution, about
0.6 ns of ~15-18 ns per tick (an A/A run of the same script)". The absolute per-tick time moved ~20 % between sessions
with machine load, so only the paired difference inside one run means anything. The tick timed is two JS-to-Wasm calls
through the glue in Node's V8, not Chrome's; a resolution claim would need ~20 interleaved pairs with an A/A control.

### Q5 runs: allocation.spec x3 in the worktree (07:37:52Z)
67 / 79 / 61 B per frame, 118 / 117 / 125 B per block, 4,163 / 4,195 / 4,289 B/s; 44 / 47 / 44 frames (5.5-5.9 fps with
other browsers running), 253-257 blocks; 3 passed. Per-frame sampling noise ~ 67 B / sqrt(45) ~ 10 B (1 sd), so 170 sits
~10 sd above clean and the plant (261) ~9 sd above 170: statistically sound where nothing tiers up. Flakiness sources:
frames > 20 needs >= 2.6 fps over 8 s (2x margin at 5.5 fps; three parallel browsers could break it); the counts are
read outside the sampled window (253-257 blocks for "8 s" = ~8.5 s), so per-frame and per-block read ~6 % low (read
the report after startSampling and before stopSampling); and A1 (fail-open once callers tier up).

### L2 (Low) the pool's size reasons from one tick per frame and one tick per wake (07:39:36Z)
host.ts:30-31 (POOL_SIZE 4: "the page keeps the two newest blocks, one is on its way and one is being filled") holds while
the page draws at least one frame per ~2 ticks and the Worker runs one tick per wake. A catch-up wake runs up to 7
(pacing.ts:7) and a long page frame piles blocks into #returned until the next frame. Model with the real BlockPool
(scratch .gate/perf-auditor-pool-sim.mjs, copy here; the page's keep-two / return-next-frame rule; blocks taken at the
end of the page's busy part): misses per 600 ticks 160.1 at 5.5 fps, 0.2 at 8.6, 0.0-0.1 from 12 to 144 fps; one 250 ms
hitch every 5 s gives 13.9 (30 fps) and 15.8 (60 fps), 0.2 with a pool of 8. Matches the real page: 175 misses per 617
ticks at 5.5 fps (P1 run) and the card's 4-11 at 8.6 fps (frame-time jitter). So 4 is right at steady frame rates, and
the container's misses say "frames slower than ~3 ticks", not "pool too small". Each miss is a new ArrayBuffer of the
block's size in the Worker, dropped again when the free list is full: harmless at 8 B, real churn at cs4/cs5 block sizes
during hitches. Fix when blocks grow: size 9 (2 kept + 7 per wake) or keep what misses made up to that cap; correct the
comment either way.

### Q3 reading: per-frame allocation in a production build
Beyond the input message (the {type, sequence, viewTime, returned} object): the fresh empty array each frame
(worker-link.ts:114; ~16 B), the PictureFrame literal (boot.ts:100; ~28 B, outside allocation.spec's sum), and three's own
render allocations (~37 KB per frame before tier-up, ~13 KB after, in the WebGL 2 fallback's empty scene; see A1's side
reading). The cold sum under frame() is 61-79 B (x6 runs). Both small ones could go (reuse the array after the post,
since postMessage serializes synchronously; pass the PictureFrame fields or reuse one object), but at ~45 B per frame
they are not worth a push on their own.

### Q2: the definition is right; the biases (07:40:51Z)
Tick start minus its due time from step() (pacing.ts:93) is PM-0004-68's "scheduled and actual start" read against the
Worker's own schedule. Biases: (1) P1, dominant and self-inflicted; with it fixed W2 reads p99 1.40-1.47 ms here, so the
head's numbers would wrongly point to OQ-0002-4's W3 fallback; (2) the ring keeps 600 ticks (20 s) and readings.mjs reads
at 600-618 blocks, so start-up ticks sit in the window and latenessMax is the window's worst, not the run's (PM-0004-68
wants the 17,820th of 18,000: log every start or keep a 0.1 ms histogram); (3) ticks dropped by a rebase never enter the
ring (a stall reads as drops, not lateness): print drops beside p99; (4) clock step 0.1 ms and the readout's own work
(~0.1 ms of sorts in stats(), 4 times a second; input posts as Worker tasks): negligible.

### L3 (Low) the readings' windows and polling
readings.mjs and report.ts's running() wait with page.waitForFunction (Playwright's default polling is every animation
frame), so boot.ts's data() (66 us, 5 sorts, a few KB of objects and strings) runs every frame while readings are taken,
and the 600-sample rings still hold start-up. Poll every 500-1000 ms and read after 1,200 blocks (or clear the rings).

### L4 (Low, cs6 note) "frame time" is the rAF interval
FrameTimes.push gets three's rAF timestamp (Animation.js update(time) -> boot.ts:97-101), so the readout's "frame time"
is the time between presented frames: pinned at 16.67 ms on a 60 Hz panel until a frame is missed, blind to headroom.
CS6's frame-time marks need the callback's own work time (performance.now() at its start and end) beside it.

### L5 (Low) allocation.spec reads its counts outside the sampled window
allocation.spec.ts:26-34: report() before startSampling and after stopSampling, so frames and blocks include the CDP round
trips (253-257 blocks in an "8 s" window): per-frame and per-block read ~6 % low. Read after start, before stop.

### Q8 (what cs3 and cs5 should measure first)
cs3: retake pacing after P1 (wakes per tick ~1.0; every tick start logged over 18,000 ticks with the display-rate input
posts running: 60/120/144 Hz); the Worker's time per tick (input + step + copy + post: median, p95, p99) as the baseline
PM-0002-25/28 grow from; calls per tick with the glue, rebase included (PM-0002-3); press-to-tick latency (PM-0002-13);
the state hash's cost per tick at the new state size (PM-0004-21). cs5: PM-0002-7 with a method that survives tier-up
(A/B with the hand-off off on whole-page bytes, or caller-level sums checked after 10 minutes) plus an off-heap count; the
copy curve by batch timing (here 0.07-1.5 us for 8 B-64 KiB) or the isolated build; delivery with the 16 ms busy-wait
and doneAt at the step's return; pool misses during hitches at real block sizes; three's own per-frame bytes per object.
Note for cs5's mark reading: PM-0002-7's "0 ArrayBuffer and 0 typed-array allocations per frame" can only mean backing
stores under B2, since every transferred block arrives as a new ArrayBuffer object and the page binds new views on it.

## Done 2026-11-19T07:42:11Z: verdict = findings to fix before the merge (P1 Medium, A1 Medium; L1-L5 Low)
Not checked: WebGPU (no adapter here), a real GPU, headed or Brave runs; the P1 fix and A1's floor on the real page (no
rebuild; P1's fix tried on the pacer's logic in a scratch Worker); the Worker's tick allocation inside Chromium (Node's V8
instead); a build without the overflow overrides (A/A only); the readout's DOM and layout cost; GC pauses and memory over
10 minutes; the production build. Baselines saved to .claude/agent-memory-local/perf-auditor/MEMORY.md.
Scratch scripts: this folder's perf-auditor-*.mjs (run copies under .gate/; .gate/perf-auditor-pkg-a and -b are the A/A's
two copies of the built pkg, removable).
