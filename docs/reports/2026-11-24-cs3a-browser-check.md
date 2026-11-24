# Mustafa's browser check of change-set 3a: his reports, word for word, and their reading

_What this is: the record of his browser check of change-set 3a's zip (`shamble-v2-03a-rules-boundaries.zip`, 246,904
bytes, sha256 8915aedebf1800f06f9e6d98bc7b1cf33855d3e22c2e8b4a4612df75d3c366f7, built from commit c5f6a85a3; the two
commits after it changed only comments and docs, and the game's WebAssembly file stayed byte for byte the same). He
ran it in his own browser (Brave) and in Firefox on his Windows computer and pasted what the page's "Copy report"
button gave him into the build chat (session_01WLuqnvpAh9XTitx8ehKJcQ) at 2026-11-24, 21:48:49 UTC, with two
pictures. Appendix A holds that message whole, copied by script from the chat's transcript and never retyped. The
reading above it is the build chat's. The change-set's own report: `docs/reports/2026-11-24-cs3a-rules-boundaries.md`._

_In short: errors 0 in all three reports; the commit shown is c5f6a85a3; WebGPU in both browsers; the match stands
still while the page is hidden (0 ticks while hidden); the "frames" line now agrees with the run's average in
Firefox; the two things his first check of the page did not show are shown (Brave Shields was up; the page opened as
a file shows its one line of text). One line is not what the zip's README said to expect: "last pause" shows 16.5 s,
not about 60 s (below)._

## Brave (his own browser)
"Chrome/158.0.0.0" in the report's browser line, Windows; display 2560x1440 at 1.25x; 23 logical processors. Both
reports: backend WebGPU, adapter "(not given)"; errors 0; blocks equal to ticks; draw calls 3, triangles 3, lines 62;
drops 0, pool misses 0, undetached 0, at most 1 tick per wake.

| Report at (UTC) | Ticks | Frames | Readout's "frames" line | Frames over the ticked time | Pause | Lateness (p50, p99, max, ms) |
|---|---|---|---|---|---|---|
| 21:39:08 | 4988 (166.3 s) | 29925 | "180.0 a second" (fps 179.97) | 180.0 a second | "none yet", pauses 0 | 1.467, 5.800, 5.900 |
| 21:42:38 | 7093 (236.4 s) | 42571 | "180.0 a second" (fps 179.97) | 180.1 a second | "16.5 s hidden · ticks while hidden: 0", pauses 6 | 1.433, 4.667, 4.933 |

Against the README's steps:
- Step 1 (the served page): as expected: the ground with its grid, "errors: 0", the commit c5f6a85a3.
- Step 2 (in front for 2 minutes): the first report, taken after 166 s of ticks. The "frames" line holds at 180.0.
- Step 3 (the window covered on purpose): in his words, "When I placed the thing explorer window in front of it: it
  dropped to 179 fps briefly". No gap opened between frames and ticks: between the two reports the page drew 12,646
  frames in 2,105 ticks (70.2 s), which is 180.2 a second, the same rate as before.
- Step 4 (another tab): the pause works: "ticks while hidden: 0", drops 0. **Not as the README words it:** it said
  "last pause" should show about 60 s hidden; the report shows 16.5 s, the last of six pauses. A report holds only
  the last pause's length. By the two reports' own clock times 210.1 s passed between them and the match ticked for
  70.2 s of them, so the match stood still for about 140 s in all across those six pauses, which fits a covered
  minute and a minute in another tab. So his Brave reports a fully covered window as hidden too, and the match
  pauses for it. Whether each of the six pauses was a covered window or a tab switch the reports do not say.
- Step 5: both reports pasted. Brave Shields was up for the page (his first picture: "Shields up for this site", 0
  trackers blocked, fingerprinting blocked; the page beside it shows ticks 8230, "errors: 0" and the same last
  pause). A double-click on `index.html` (his second picture, the file opened from a folder of his computer) shows
  the page's one line: "This page does not run when it is opened as a file. In its folder, run "node serve.mjs" and
  open the address it prints." The pictures are not in the repository: the second shows a folder path of his computer.

## Firefox
"Firefox/160.0" (rv:160.0), Windows; display 2048x1280 at 1.25x; 24 logical processors. One report, at 21:47:43 UTC:
backend WebGPU, adapter "(not given)"; errors 0; ticks 5552 (185.1 s), blocks 5552, frames 33297; pauses 0; drops 0;
lateness p50 0.667, p99 4.667, max 5.667 ms; 6042 wakes of the Worker for 5547 ticks.
- Step 6: **the "frames" line now agrees with the run's average.** The readout says "178.4 a second" (fps 178.44);
  the frames over the ticked time come to 179.9 a second. At change-set 2's check the same line said 239.2 where the
  average was 179.9: the line is now a count over time, as change-set 3a meant it.

## What this check adds to the earlier check's open points (`docs/reports/2026-11-19-cs2-browser-check.md`)
- **The two things that check did not show are shown:** the Shields setting (up) and what a double-click on
  `index.html` shows in Brave (the one line of text).
- **The gap of that check (frames far below ticks in one Brave run, not explained) did not come back** with the
  window covered on purpose: here a covered window is a pause, and frames keep pace with ticks. What happened in
  that earlier run stays not explained.
- **Still as before, readings and no errors:** the adapter's name is "(not given)" on WebGPU in both browsers; in
  Firefox the Worker wakes more often than it ticks (6042 for 5547).

## Not shown by this check
A single hidden stretch of a known length (only the last pause is in a report); the forced WebGL 2 fallback and the
padded runs (not asked this time); any browser other than his Brave and his Firefox.

## Appendix A. His message, word for word
_Copied by script from the chat's transcript: the user message of 2026-11-24T21:48:49.710Z (uuid
b5bb0503-5aef-45fc-b832-6bc2a147b40c), its text whole (4451 bytes, sha256 393a9e4be27c2927a215344efe2aacc4a4a3a4a16953e054340e8db072c5ce99). The
message also carried the two pictures described above._

```text
Did the things for the zip
Left it open for ~2 minutes and pressed copy (I know it wasn't the instruction, but I have done it anyway):
Shamble Frontlines, first page: report
backend: WebGPU
ticks: 4988 at 30 a second · blocks: 4988 of 8 bytes
frames: 180.0 a second · frame time 5.60 ms (p99 5.70 ms)
errors: 0
last pause: none yet
isolation: no · commit: c5f6a85a3

commit: c5f6a85a3
run: (none)
at: 2026-11-24T21:39:08.227Z
browser: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/158.0.0.0 Safari/537.36
display: 2560x1440 at 1.25x, 24 bit
hardwareConcurrency: 23
crossOriginIsolated: false
backend: WebGPU
adapter: (not given)
forceWebGL: false
webgpuOffered: true
timestampQuery: true
render.drawCalls: 3
render.triangles: 3
render.lines: 62
render.geometries: 3
render.textures: 3
render.programs: 6
tick: 4988
tickHz: 30
blocks: 4988
blockBytes: 8
inputPosts: 29923
frames: 29925
fps: 179.97
frameP50: 5.60 ms
frameP99: 5.70 ms
deliveryP50: 0.10 ms
deliveryP95: 0.20 ms
errors: 0
lastError: (none)
pauses: 0
lastPauseHiddenMs: (none)
lastPauseTicksWhileHidden: (none)
worker.ticks: 4984
worker.wakes: 4985
worker.maxTicksInOneWake: 1
worker.drops: 0
worker.droppedMs: 0
worker.pauses: 0
worker.latenessP50: 1.467
worker.latenessP99: 5.800
worker.latenessMax: 5.900
worker.copyP50: 0
worker.copyP99: 0.100
worker.blockBytes: 8
worker.poolMisses: 0
worker.poolRefused: 0
worker.undetached: 0
worker.hardwareConcurrency: 23
worker.crossOriginIsolated: false

When I placed the thing explorer window in front of it: it dropped to 179 fps briefly

then I swapped tabs for 1 min:
Shamble Frontlines, first page: report
backend: WebGPU
ticks: 7093 at 30 a second · blocks: 7093 of 8 bytes
frames: 180.0 a second · frame time 5.60 ms (p99 5.70 ms)
errors: 0
last pause: 16.5 s hidden · ticks while hidden: 0 (a match that ran on: about 496)
isolation: no · commit: c5f6a85a3

commit: c5f6a85a3
run: (none)
at: 2026-11-24T21:42:38.355Z
browser: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/158.0.0.0 Safari/537.36
display: 2560x1440 at 1.25x, 24 bit
hardwareConcurrency: 23
crossOriginIsolated: false
backend: WebGPU
adapter: (not given)
forceWebGL: false
webgpuOffered: true
timestampQuery: true
render.drawCalls: 3
render.triangles: 3
render.lines: 62
render.geometries: 3
render.textures: 3
render.programs: 6
tick: 7093
tickHz: 30
blocks: 7093
blockBytes: 8
inputPosts: 42569
frames: 42571
fps: 179.97
frameP50: 5.60 ms
frameP99: 5.70 ms
deliveryP50: 0.10 ms
deliveryP95: 0.30 ms
errors: 0
lastError: (none)
pauses: 6
lastPauseHiddenMs: 16530
lastPauseTicksWhileHidden: 0
worker.ticks: 7088
worker.wakes: 7095
worker.maxTicksInOneWake: 1
worker.drops: 0
worker.droppedMs: 0
worker.pauses: 6
worker.latenessP50: 1.433
worker.latenessP99: 4.667
worker.latenessMax: 4.933
worker.copyP50: 0
worker.copyP99: 0.100
worker.blockBytes: 8
worker.poolMisses: 0
worker.poolRefused: 0
worker.undetached: 0
worker.hardwareConcurrency: 23
worker.crossOriginIsolated: false


From firefox (might have been closer to 3 minutes instead of 2)
Shamble Frontlines, first page: report
backend: WebGPU
ticks: 5552 at 30 a second · blocks: 5552 of 8 bytes
frames: 178.4 a second · frame time 4.18 ms (p99 8.62 ms)
errors: 0
last pause: none yet
isolation: no · commit: c5f6a85a3

commit: c5f6a85a3
run: (none)
at: 2026-11-24T21:47:43.672Z
browser: Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:160.0) Gecko/20100101 Firefox/160.0
display: 2048x1280 at 1.25x, 24 bit
hardwareConcurrency: 24
crossOriginIsolated: false
backend: WebGPU
adapter: (not given)
forceWebGL: false
webgpuOffered: true
timestampQuery: true
render.drawCalls: 3
render.triangles: 3
render.lines: 62
render.geometries: 3
render.textures: 3
render.programs: 6
tick: 5552
tickHz: 30
blocks: 5552
blockBytes: 8
inputPosts: 33296
frames: 33297
fps: 178.44
frameP50: 4.18 ms
frameP99: 8.62 ms
deliveryP50: 0.00 ms
deliveryP95: 1.00 ms
errors: 0
lastError: (none)
pauses: 0
lastPauseHiddenMs: (none)
lastPauseTicksWhileHidden: (none)
worker.ticks: 5547
worker.wakes: 6042
worker.maxTicksInOneWake: 1
worker.drops: 0
worker.droppedMs: 0
worker.pauses: 0
worker.latenessP50: 0.667
worker.latenessP99: 4.667
worker.latenessMax: 5.667
worker.copyP50: 0
worker.copyP99: 0
worker.blockBytes: 8
worker.poolMisses: 0
worker.poolRefused: 0
worker.undetached: 0
worker.hardwareConcurrency: 24
worker.crossOriginIsolated: false
```
