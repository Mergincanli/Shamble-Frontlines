# Mustafa, 2026-11-24, 21:48:49 UTC: his browser check of change-set 3a's zip: two reports from Brave, one from Firefox, and two screenshots

_Copied by script from the chat's transcript (session_01WLuqnvpAh9XTitx8ehKJcQ), never retyped: the user message of 2026-11-24T21:48:49.710Z (uuid b5bb0503-5aef-45fc-b832-6bc2a147b40c), whole (4451 bytes, sha256 393a9e4be27c2927a215344efe2aacc4a4a3a4a16953e054340e8db072c5ce99). What it answers: the chat's request of 17:42:35 UTC: the browser check of the zip by its README (Brave in front for 2 minutes, covered for 1 minute, another tab for 1 minute, "Copy report", whether Brave Shields was up, what a double-click on index.html shows; then Firefox for 2 minutes and its report). The message's two pictures are not in the repository (one shows a folder path of his computer); what they show is written in the browser-check record_

---
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
