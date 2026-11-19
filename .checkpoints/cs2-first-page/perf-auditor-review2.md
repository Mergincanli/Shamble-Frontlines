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
