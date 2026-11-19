# perf-auditor, review round 2 of cs2-first-page (30a5a13..d257bb6; fix commits 29378dd and d257bb6)

2026-11-19T08:05:04Z plan: read round-1 notes and the fix diff (sim-worker/src, client/src/app, client/src/render, e2e/tests/skeleton); judge P1 (Math.ceil sleep) and A1 (allocation floors and limits) with the lines; re-run the round-1 scratch pacer with Math.ceil in .gate/perf-r2 to measure tick lateness against PM-0004-68; run the unit tests read-only; scan the fix diff for new per-tick or per-frame allocations or slowdowns; no browser and no rebuild in the main folder; notes here only.

## Findings (appended as made)

- 2026-11-19T08:11:50Z read: the whole fix diff (15 files), host.ts/pacing.ts/worker.ts/worker-link.ts/boot.ts/allocation.spec.ts
  at d257bb6, PM-0004-68's row (ADR 0004 line 1195), the planted-defect run 2 notes (finished 08:09:14Z, every plant caught,
  7 and 7b at allocation.spec.ts:58 and :60). The main folder's dist (07:50:27Z, commit string "81e916d38+changes") already
  holds the fixes: worker asset has setTimer(...,Math.ceil(e)) and doneAt:timeOrigin+before; page asset has receive(e) and
  the input message {type,sequence,returned}. The session's readings.mjs (L3 fixed there: polling 500 ms, read after 1,200
  blocks) is running now (browser up since ~08:09:30Z): no browser of mine until it ends, so its numbers stay clean.
- P1 code: host.ts:139 setTimer(this.#wake, Math.ceil(sleep)); the only other sleep source, Pacer.resume(), is ignored by
  the host (it calls #wake at once, which runs no tick and returns ~33.3 ms, then ceil). worker.ts:29 passes the delay
  through unchanged. host.test.ts:84-85 pins it: every delay an integer >= 33, 91 wakes for 90 ticks. Removing the ceil
  fails isInteger; floor or round fail too (the stub fires at exactly now+delay, so an early timer finds no tick due, the
  re-sleep is 0 and the 91-wake count and the >= 33 check both fail). Place: right. The pacer stays exact (made-up clock
  tests), the host is where the pacer meets the browser's whole-millisecond timer, and only there can a unit test pin it
  (worker.ts is the untested browser binding).
- L1 closed: host.ts:144-148 (doneAt = timeOrigin + before, the step's return; the copy time still taken from the same
  two now() calls: no extra clock read, no extra allocation).
- L5 closed: allocation.spec.ts:37-40 (report() after startSampling, before stopSampling). Residual: the sampled window is
  now one CDP round trip longer on each side than the counted one, so per-frame/per-block read a few % high instead of
  ~6 % low; nil against the limits.
- Per-tick and per-frame cost of the fixes (code read): nothing new allocates. Worker per tick: Math.ceil returns a Smi
  (no HeapNumber), same two now() calls, same post. Page per frame: the input message lost viewTime (one field and one
  boxed double fewer: ~16 B less per frame on the page, and the Worker no longer reads a key string and a HeapNumber per
  input or runs two checks; the caller's 54-71 B per frame vs round 1's 61-81 fits). Page per message: receive() adds one
  call (inlined once hot), `failure !== ""` one compare, BLOCK_LAYOUT.header.fields.tick.offset four monomorphic loads on a
  plain literal (layout.ts:7-15): ns, no allocation. boot.ts data(): two more fields per report (4 a second): nil.
  three-picture.ts:27-30 console hook: runs only when three logs; an error builds an Error (stack capture) and a page
  error event, so a per-frame error from three would cost tens of us per frame, but only in a state that is already a
  failure (errors must be 0 on the clean page, skeleton.spec.ts).
- 2026-11-19T08:17:54Z real page measured (scratch .gate/perf-auditor-r2-page.mjs, copy here as perf-auditor-r2-page.mjs; the
  main folder's dist of 07:50:27Z = the fixes; own server on a free port; Chromium 153.0.8010.12 headless, Playwright 1.65.0
  default launch; alone on the box, load 0.8 at the start, 4.0 at the end; 08:14:32-08:17:31Z):
  - P1 after 1,200 blocks: ticks 1,213, wakes 1,214 (1.001 per tick; head in round 1: 1.64), most in a wake 1, drops 0;
    lateness over the last 600 ticks p50 0.83, p99 4.30, max 8.40 ms (head: p50 3.70, p99 7.57, max 22.17); fps 5.5.
    At the end (5,355 ticks, 179 s): wakes 5,339 (0.997 per tick: some wakes ran 2 ticks), drops 0, p50 0.90, p99 4.67,
    max 12.40 ms at 4.6 fps. P1 CLOSED: the empty second wake is gone. The p99 left over is the container's CPU load
    (software rendering at ~5 fps); the reading is reported, not judged, in this change-set.
  - W1 (allocation.spec's exact method right after 300 blocks; 98 frames so far): 58.7 B per frame under "frame", 296.7 B
    per block under "receive" (onMessage alone 118.0): pass; matches the session's 54-71 and 293-316.
  - W2 (same window after 170 s; 805 frames and 5,340 blocks so far): 0.0 B per frame under "frame" and 0.0 B per block
    under "receive" (onMessage 99.7, still its own frame); receive is inlined into the listener, (anonymous)
    index-Ba5b81Hu.js:541:48700 holds 93,396 B self. Both floors trip: the spec FAILS, it no longer passes blind.
    A1 CLOSED for the fully blind case, shown on the real page.
  - Console over the run: 6 warnings, 1 info, 1 error (page errors 0, the page's error count 0, so not three's hook,
    which would also have raised the count): one-off messages, not per frame; the hook is not on a hot path.
  - Pool misses 353 of 1,213 ticks at 5.5 fps and 2,131 of 5,355 at 4.6 fps: L2's model (misses follow ticks per frame,
    ~6 here), nothing new.
- 2026-11-19T08:20:40Z pacer split (scratch .gate/perf-auditor-r2-pacer.mjs, copy here; pacing.ts's wake loop and host.ts's
  timer call in blob Workers on a blank page; Chromium 153 headless, default launch; load 1.3 -> 0.3; 08:18:38-08:20:19Z):
  - ceil (d257bb6), 3,000 ticks (100 s): 3,001 wakes (the extra one is start()'s), 0 timer wakes with no tick due, drops 0;
    lateness p50 0.80, p90 1.17, p99 1.87, p99.9 4.00, max 17.60 ms, mean 0.81; 2 ticks over 4 ms (0.07 %).
    Split: rounding (delay given minus exact sleep) p50 0.50, p99 1.00, max 1.00, mean 0.51 ms; the timer's own overshoot
    p50 0.30, p99 1.30, p99.9 3.30, max 17.50 (one hiccup), mean 0.30 ms. Lateness = rounding + overshoot.
  - head (30a5a13, fractional), 600 ticks alongside: 1.283 wakes per tick (169 empty), lateness p50 0.13, p90 4.20, p99 4.30,
    max 4.50 ms, 22.8 % of ticks over 4 ms (the clamped re-sleep: overshoot p90 4.17).
  - Reading: the ceil costs ~0.5 ms at the median by design (0.13 -> 0.80 idle) and buys p99 4.30 -> 1.87 idle and
    7.57 -> 4.30 on the loaded real page, plus ~0.3-0.6 fewer Worker tasks per tick. No early wake in 3,000 (the clock's
    0.1 ms coarsening never read a wake as early). It is INSIDE PM-0004-68's reading: lateness is start minus due
    (pacing.ts:93), so the rounding is counted, not hidden; it is a sawtooth (the due times step by 33.33 ms against
    whole-ms timers), i.e. real start jitter of at most 1 ms. The definition needs no change; the readings' write-up should
    say once that up to 1 ms (mean 0.5) of every start is the whole-millisecond timer by design, leaving ~3 ms of the
    4 ms p99 for the browser (idle here: p99 overshoot 1.3 ms).
