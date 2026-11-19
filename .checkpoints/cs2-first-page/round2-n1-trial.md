# perf-auditor's N1 (round 2, Medium): the proposed change to the allocation test, tried in the scratch copy

Written 2026-11-19 09:35 UTC (from the clock) by the main session. **Nothing of this is in the change-set yet:**
the change waits for Mustafa's answer to SIZE-STOP-2 (no code is added or changed until then). The candidate change is
`round2-n1.diff` beside this file (`packages/e2e/tests/skeleton/allocation.spec.ts` only: the window starts at block
90 instead of 300, so its 8 seconds end near block 340, before the browser optimises the message listener at about
block 470; per block the floor goes from 150 to 430 and the limit from 390 to 610; the comments say what the numbers
are). It stands applied in the worktree `.gate/cs2-plants` (9f28a87, whose code is d257bb6's), nowhere else.

Runs there, container's Chromium 153 headless, software rendering, 09:09:54 to 09:12:37 UTC, the box otherwise idle
(load 0.03 at the start):

| build | bytes per frame | bytes per block | result |
|---|---|---|---|
| clean | 72 | 501 | passed |
| clean | 46 | 510 | passed |
| clean | 63 | 489 | passed |
| clean | 48 | 504 | passed |
| clean | 61 | 489 | passed |
| plant 7b (a 64-byte typed array per block in `WorkerLink.receive`), rebuilt | 68 | 711 | FAILED at `expect(perBlock).toBeLessThan(610)` |
| plant 7b | 58 | 684 | FAILED at the same line |
| plant 7b | 61 | 696 | FAILED at the same line |
| clean again (the plant taken back by its reverse diff, rebuilt) | 57 | 478 | passed |

So: clean 478 to 510 (six runs here; perf-auditor read 482 to 514 in six cold windows), the plant 684 to 711 (perf-auditor:
718 to 727). The limit of 610 has about 100 bytes of room above the clean page and about 75 below the plant; the floor
of 430 has about 50 below the clean page. Not tried: plant 7 (per frame; its limits do not change), a machine with a
real GPU at 60 frames a second (perf-auditor's N2, Low: there the frame callback may be optimised inside the window).
When the change is applied, `plants.py`'s plant 7b must name the new line (`toBeLessThan\(610\)`).
