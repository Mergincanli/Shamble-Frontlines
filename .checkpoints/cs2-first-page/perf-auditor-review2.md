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
- 2026-11-19T08:24:30Z NEW, to confirm: the per-block sum is mid-transition AT the spec's own point. Mirror of the message
  kinds (scratch .gate/perf-auditor-r2-msg.mjs, copy here; 2 rounds, cold, ~242 messages each): a block costs 437-448 B
  under "receive" (event.data's deserialization 294-295 + onMessage 142-154), a report message 373-385 B. Real page in
  three windows (scratch .gate/perf-auditor-r2-tree.mjs, copy here; 08:23:06-08:23:36Z, load ~0-1):
    W0 blocks 74->319:  under receive 511.2 B/block (self 368.0), onMessage 143.2 (all below receive), listener total 531.4
    W1 blocks 349->596 (the spec's window): under receive 246.7 (self 177.8); onMessage 131.1, of which 62.1 NOT below
       receive; the listener's own bytes 182.1 B/block: receive was inlined into the listener partway through
    W1b blocks 613->860: under receive 0.0 (floor trips), listener self 429.1
  Per frame under "frame": 41.1 / 61.2 / 64.0 (the rAF callback stays cold this early at ~5 fps).
  So the clean cold cost is ~510 B per block, above the 390 limit, and the "about 300" the spec's comment and the session
  report is the visible part of a switch that lands inside the window (the listener reaches V8's optimising tier after a
  few hundred calls; messages come at a fixed ~33 a second, so the switch falls at about the same message count on every
  machine, i.e. inside a window that starts at 300 blocks). Plant arithmetic: visible fraction f = 246.7/511 = 0.48 here,
  0.58 in the first run (296.7); a 64-B array per block (+~187 B while visible) reads f x 698: 335 here (< 390: MISSED,
  while clean 246.7 passes the 150 floor), 405 in the first run (caught). Checking the spread with two more runs.
- 2026-11-19T08:30:30Z spread confirmed; the plant measured. Two more tree runs: W0 505.9 / 509.6 B per block (cold),
  W1 (from 365 / 340 blocks) 229.5 / 276.1, W1b 0.0 / 0.0. Plant 7b exactly as plants.py:60-62 (Uint8Array(64) in
  receive, a scratch field) patched into a COPY of the main folder's dist (.gate/perf-auditor-r2-dist-7b; the dist was
  rebuilt by the session at 08:15:18Z from ae47239 = the d257bb6 code; only the commit string differs), the spec's flow
  (300 blocks, 8 s window, report() inside), fresh context per run, alternating (scratch .gate/perf-auditor-r2-plant.mjs,
  copy here; 08:27:36-08:29:37Z, load 0.6-2.2):
    clean x3: W0 (cold) 482.2 / 510.4 / 514.4 B/block; spec window 331.1 / 307.7 / 317.6 -> pass x3
    plant x3: W0 (cold) 726.4 / 726.9 / 717.9 B/block (the plant adds ~215 B); spec window 439.3 / 442.7 / 439.9 -> FAIL x3
    at the 390 limit (caught). Per frame in the spec window 51-65 B (floor 25, limit 170 fine).
  Where the listener switches, from the visible fraction in all 10 windows: at block 464-478 (~470), the same on every run
  (V8's tier-up at a fixed call count; blocks come at a fixed 30 a second). The spec's window starts at ~305-320 blocks, so
  ~60-69 % of it is visible: plant ~440 > 390 (margin ~50), clean ~310-330 < 390 (margin ~60). It works here, but only
  because the switch lands ~2/3 into the window. If a Chromium update moves the switch past the window, the clean page
  reads ~505 and FAILS at 390; if it moves earlier, the plant is missed while the 150 floor still passes (floor trips only
  below 150/505 = 30 % visible; the plant is missed below 390/725 = 54 %). The spec's comment "about 300 bytes" per clean
  block is the visible part, not the cost (~505 cold: ~370 event.data incl. ~30 B/block of report messages, ~140 in
  onMessage), and the stated condition "nothing is optimised" does not hold in the container's own runs.
  => N1 (Medium): fix = sample the cold region: running(page, 60), an 8 s window (ends ~300 blocks, well before ~470),
  per-block floor ~430 and limit ~610 (clean cold 482-514 in 6 runs, plant 718-727), per-frame floor 25 / limit 170 as
  they are; or (CS5) sum under a frame the JIT cannot inline: WorkerLink.handleEvent registered as the listener object
  (the browser calls it, so it is always a physical frame; the listener's total held 520-540 B per block in every tier).
- 2026-11-19T08:36:40Z console texts (scratch .gate/perf-auditor-r2-console.mjs, copy here; 12 s): "No available adapters",
  three's "WebGPU is not available, running under WebGL2 backend" (the only message through three's hook), the backend
  info line, 4 x Chrome's "GPU stall due to ReadPixels" (then "will no longer repeat"; the renderer runs with
  --disable-gpu-compositing, so the WebGL canvas is read back every frame for the software compositor: part of why the
  container draws ~5 fps; an environment artifact, not the code), and the one error = favicon.ico 404 (a resource error,
  rightly not in the page's count). The hook is on no per-frame path.
- Unit tests at the branch head (08:32Z): sim-worker src 30/30, boundary 7/7, client 48/48 pass.
- Worker tick path A/B in Node 22.23.4 / V8 12.4 (scratch .gate/perf-auditor-r2-tick-alloc.mjs, copy here; host.ts and its
  imports as at 30a5a13 = arm a and d257bb6 = arm b, same Wasm, one process per arm, two runs each):
    cold ticks: a 975.7 / 992.7, b 991.7 / 977.4 B per tick; hot: a 501.3 / 501.7, b 502.1 / 501.2
    input message alone: cold a 149.3 / 156.5, b 155.7 / 154.2; hot a 56.6 / 56.6, b 56.7 / 56.5
    timer delays: a 33.333, 33.323, ... (fractional); b 34, 34, ... -> the ceil costs nothing per tick (within +-1 %).
  The viewTime saving is on the deserialization side (browser only), not visible in this stub.
- Verdict so far: P1 closed (improvement), A1 closed for the fully blind case; NEW N1 Medium (per-block window straddles the
  listener's tier-up; fix above); Low: per-frame has the same partial band on a 20-60 fps machine (floor 25 trips below
  ~40 % visible, plant 7 is missed below ~68 %); Low: PM-0004-68 readings need the one-line rounding note; L2/L4 stay
  follow-ups; nothing in the fixes allocates more or runs slower per tick or per frame.
