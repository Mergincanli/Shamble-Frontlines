# Mustafa's browser check of the first page (pull request #61): his reports, word for word, and their reading

_What this is: the record his go on #61 asks for ("My browser reports are recorded with the next docs pass, not by a
push to this head.": `docs/DECISIONS.md`, the entry of 2026-11-19, 14:28:25 UTC). He ran the zip of change-set 2
(`shamble-v2-02-first-page.zip`, 245,953 bytes, sha256
c49d8a7f9b599bc5d9855fca0c782650a796b0f54ec4cf284badb5f2a920c5f4, built from commit 7d8e2de2e) in his own browser
(Brave) and in Firefox on his Windows computer, and pasted what the page's "Copy report" button gave him into the build
chat (session_019YNMUV9LFhASseJ9EP12kL). Appendix A holds those four messages whole, copied by script from the
session's transcript and never retyped. The reading above it is the build chat's, written at 15:12 UTC the same day;
the merge (f5030a3, 15:10:54 UTC) followed his last report. The change-set's own report:
`docs/reports/2026-11-19-cs2-first-page.md`._

_In short: errors 0 in all eight reports; WebGPU starts in both browsers on his machine, and the forced WebGL 2
fallback works in both; the pause on a hidden tab works in both (0 ticks while hidden). One thing is not explained
(BR-GAP below), and three things were not shown (NOT-SHOWN below). The follow-ups stand in
`docs/STATUS_ARCHIVE.md`, "Follow-ups", "From change-set 2", under these ids._

## What came, in order
| His message (UTC) | Where | What it holds |
|---|---|---|
| 14:28:25 | `docs/DECISIONS.md` | His go on #61 and BROWSER-SET (a). It says the reports "are in this chat"; none was. The merge waited. |
| 14:31:30 | `docs/DECISIONS.md` | "a, yes" (his answer VISION-NOTE (a): the dated note's text as put to him), and a request for the steps again. |
| 14:46:25 | Appendix A, message 1 | Brave: four reports (the served page, `?backend=webgl`, `?pad=4096`, `?pad=65536`) and the version line. |
| 14:54:19 | Appendix A, message 2 | Firefox: a screenshot of the page opened as a file (the picture is not in the repository: it shows a folder path of his computer), and the served page's report. |
| 15:02:50 | Appendix A, message 3 | Brave: two re-checks of the hidden tab. |
| 15:09:29 | Appendix A, message 4 | Firefox: the report of `?backend=webgl`. |

## Brave (his own browser)
Version line as pasted: "1.100.58 Chromium: 158.0.8127.42 (Official Build) (64-bit)". Windows; display 2560x1440 at
1.25x. Every report: errors 0; blocks equal to ticks; the commit 7d8e2de2e; draw calls 3, triangles 3, lines 62,
geometries 3, textures 3, shader stages 6 (graphics-engineer's prediction for WebGPU, exactly); drops 0, pool misses
0, undetached 0, at most 1 tick per wake.

| Report at (UTC) | Page | Backend line | Ticks | Frames | Pause | Other readings |
|---|---|---|---|---|---|---|
| 14:43:32 | served | WebGPU; adapter "(not given)" | 12311 | 46382 | "none yet", pauses 0 | lateness p50 2.2, p99 13.2, max 15.833 ms; hand-off p50 0.00, p95 0.20 ms |
| 14:43:42 | `?backend=webgl` | "WebGL 2 fallback, asked for by ?backend=webgl (Brave)" | 54 | 333 | none | hand-off p50 0.40, p95 0.80 ms; timestamp query yes |
| 14:44:28 | `?pad=4096` | WebGPU | 910 | 5468 | none | blocks of 4104 bytes; hand-off p50 0.20, p95 0.30 ms; copy p99 0.1 ms; lateness p99 4.033 ms |
| 14:45:41 | `?pad=65536` | WebGPU | 1677 | 10068 | none | blocks of 65544 bytes; hand-off p50 "-0.00", p95 0.20 ms; copy p99 0.1 ms; lateness p99 5.733 ms |
| 14:55:42 | served (re-check) | WebGPU | 407 | 2444 | "45.0 s hidden · ticks while hidden: 0", pauses 1 | lateness p99 5.733 ms |
| 15:02:36 | served (re-check 2) | WebGPU | 164 | 999 | "59.5 s hidden · ticks while hidden: 0", pauses 3 | lateness p99 4.4 ms |

Against the README:
- Steps 4, 5, 8: as expected. The first WebGPU run of this page anywhere.
- Steps 6 and 7 (the hidden tab): **not as expected in the first report** ("last pause: none yet"). Told to him before
  any merge (his condition). His re-check with a tab switch shows the pause: 45.0 s hidden, 0 ticks while hidden.
  The second re-check shows three hidden stretches, the last 59.5 s, with 0 ticks; its whole run has 164 ticks (5.5 s,
  which is also what its 999 frames come to), so the match ran in none of them. **A single stretch of 6 minutes is not
  shown by any report** (only the last pause is in a report).
- Step 9 (`?backend=webgl`): as expected.
- Step 10 (the two padded runs): readings, not judged.
- Step 11 (the version line): given.
- Not given: what a double-click on `index.html` showed in Brave (step 2), and his Shields setting (step 4).

## Firefox
"Firefox/160.0" (rv:160.0), Windows; display 2560x1440 at 1x; 24 logical processors. Every report: errors 0; blocks
equal to ticks; the same six render counts; drops 0, pool misses 0, undetached 0.

| Report at (UTC) | Page | Backend line | Ticks | Frames | Pause | Other readings |
|---|---|---|---|---|---|---|
| (screenshot) | opened as a file | the page's own line: "This page does not run when it is opened as a file. In its folder, run "node serve.mjs" and open the address it prints." No picture. | | | | as in Chromium |
| 14:53:34 | served | WebGPU; adapter "(not given)" | 6720 | 40295 | "16.8 s hidden · ticks while hidden: 0", pauses 1 | readout "239.2 a second", frame p50 4.18, p99 8.34 ms; lateness p50 0.667, p99 4.667, max 5.667 ms; wakes 6982 for 6717 ticks; hand-off p50 1.00, p95 2.00 ms |
| 15:09:11 | `?backend=webgl` | "WebGL 2 fallback, asked for by ?backend=webgl (ANGLE (NVIDIA, NVIDIA GeForce GTX 980 Direct3D11 vs_5_0 ps_5_0), or similar)" | 1022 | 6128 | "3.8 s hidden · ticks while hidden: 0", pauses 1 | timestamp query no; wakes 1126 for 1018 ticks; lateness p99 4.667 ms; hand-off p50 0.00, p95 1.00 ms |

Against the README: F2, F3, F5, F6 and F7 as expected (F5's and F7's hidden times are shorter than the README's
minute: he came back sooner; the line that matters, 0 ticks while hidden, holds). F4 (two minutes in view): the
served run ticked for 224 s. The three points the reading before the run left open: the page opened as a file keeps
its own line of text; three.js's WebGPU backend starts in his Firefox; the adapter's name is "(not given)" on WebGPU
and a rounded name on WebGL.

## Observations for the follow-ups (the ids are this record's; none is an error count above 0)
- **BR-GAP.** Brave's first run drew 46,382 frames while it ticked for 410 s: an average of 113 a second, where every
  other Brave run averages 180 (frames over ticks: 185.0, 180.3 and 180.1 in the three short runs, 180.1 and 182.7 in
  the re-checks). So for about 150 s the page drew nothing while the simulation ran on, and no pause was recorded.
  Not explained by the reports: the window may have been covered or the screen off. The pause on a tab switch works
  in Brave (the re-check). To read again with change-set 3's browser check: what the page is told when Brave's
  window is covered.
- **FPS-LINE.** The readout's "frames a second" is worked out from the usual gap between two frames. In Firefox it
  says 239.2 where the run's average is 179.9 (40,295 frames in 224 s; 6,128 in 34.1 s). The line should be a count
  over time.
- **NEG-ZERO.** "deliveryP50: -0.00 ms" (Brave, `?pad=65536`): the page's and the Worker's clocks, each rounded by the
  browser, can give a difference just under 0.
- **FF-WAKES.** In Firefox 4% to 10% of the Worker's wakes come before a tick is due (6982 wakes for 6717 ticks; 1126
  for 1018), with no dropped tick and lateness p99 4.667 ms. A reading for the pacing marks in change-set 3.
- **BR-LATE.** Brave's first run has lateness p99 13.2 ms and max 15.833 ms; its other runs p99 4.0 to 6.3 ms. It reads
  like the system's coarse timer (about 15.6 ms) while the window was not in front; with BR-GAP.
- **ADAPTER.** Brave names no adapter on WebGPU and "Brave" on WebGL; Firefox none on WebGPU and a rounded name on
  WebGL. A report from Brave cannot say which GPU ran; his machine's, from Firefox: NVIDIA GeForce GTX 980. Brave
  reports 6 logical processors where Firefox reports 24.
- **TQ-FF.** No timestamp query on Firefox's WebGL 2 (yes on Brave's WebGL 2 and on WebGPU in both).
- **NOT-SHOWN.** A single hidden stretch of 6 minutes; Brave's double-click on `index.html`; his Shields setting.

## Appendix A. His four messages, word for word
_Each block below is one chat message's text, quoted line for line ("> " before each line). The first lines of his
third message quote the build chat's re-check steps back, each with his "Done:"._

### Message 1: Brave: four reports and the version line
_Chat message of 2026-11-19T14:46:25.705Z (uuid 208ef246-5dad-4b30-b94e-86869610798c), its text whole.
Copied by script (5734 bytes, sha256 995e19221308a4ad5682416ccdd89ef25d9e508a31c94272164f9a7ee46b159f):_
> ~6 min copy (I think) 
> Shamble Frontlines, first page: report
> backend: WebGPU
> ticks: 12311 at 30 a second · blocks: 12311 of 8 bytes
> frames: 178.6 a second · frame time 5.60 ms (p99 5.70 ms)
> errors: 0
> last pause: none yet
> isolation: no · commit: 7d8e2de2e
>
> commit: 7d8e2de2e
> run: (none)
> at: 2026-11-19T14:43:32.408Z
> browser: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/158.0.0.0 Safari/537.36
> display: 2560x1440 at 1.25x, 24 bit
> hardwareConcurrency: 6
> crossOriginIsolated: false
> backend: WebGPU
> adapter: (not given)
> forceWebGL: false
> webgpuOffered: true
> timestampQuery: true
> render.drawCalls: 3
> render.triangles: 3
> render.lines: 62
> render.geometries: 3
> render.textures: 3
> render.programs: 6
> tick: 12311
> tickHz: 30
> blocks: 12311
> blockBytes: 8
> inputPosts: 46381
> frames: 46382
> fps: 178.57
> frameP50: 5.60 ms
> frameP99: 5.70 ms
> deliveryP50: 0.00 ms
> deliveryP95: 0.20 ms
> errors: 0
> lastError: (none)
> pauses: 0
> lastPauseHiddenMs: (none)
> lastPauseTicksWhileHidden: (none)
> worker.ticks: 12309
> worker.wakes: 12311
> worker.maxTicksInOneWake: 1
> worker.drops: 0
> worker.droppedMs: 0
> worker.pauses: 0
> worker.latenessP50: 2.200
> worker.latenessP99: 13.200
> worker.latenessMax: 15.833
> worker.copyP50: 0
> worker.copyP99: 0.100
> worker.blockBytes: 8
> worker.poolMisses: 0
> worker.poolRefused: 0
> worker.undetached: 0
> worker.hardwareConcurrency: 6
> worker.crossOriginIsolated: false
>
> step 10 copy:
> Shamble Frontlines, first page: report
> backend: WebGL 2 fallback, asked for by ?backend=webgl (Brave)
> ticks: 54 at 30 a second · blocks: 54 of 8 bytes
> frames: 178.6 a second · frame time 5.60 ms (p99 5.80 ms)
> errors: 0
> last pause: none yet
> isolation: no · commit: 7d8e2de2e
>
> commit: 7d8e2de2e
> run: (none)
> at: 2026-11-19T14:43:42.730Z
> browser: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/158.0.0.0 Safari/537.36
> display: 2560x1440 at 1.25x, 24 bit
> hardwareConcurrency: 6
> crossOriginIsolated: false
> backend: WebGL 2 fallback
> adapter: Brave
> forceWebGL: true
> webgpuOffered: true
> timestampQuery: true
> render.drawCalls: 3
> render.triangles: 3
> render.lines: 62
> render.geometries: 3
> render.textures: 3
> render.programs: 6
> tick: 54
> tickHz: 30
> blocks: 54
> blockBytes: 8
> inputPosts: 330
> frames: 333
> fps: 178.57
> frameP50: 5.60 ms
> frameP99: 5.80 ms
> deliveryP50: 0.40 ms
> deliveryP95: 0.80 ms
> errors: 0
> lastError: (none)
> pauses: 0
> lastPauseHiddenMs: (none)
> lastPauseTicksWhileHidden: (none)
> worker.ticks: 48
> worker.wakes: 49
> worker.maxTicksInOneWake: 1
> worker.drops: 0
> worker.droppedMs: 0
> worker.pauses: 0
> worker.latenessP50: 0.633
> worker.latenessP99: 6.267
> worker.latenessMax: 6.267
> worker.copyP50: 0
> worker.copyP99: 0.100
> worker.blockBytes: 8
> worker.poolMisses: 0
> worker.poolRefused: 0
> worker.undetached: 0
> worker.hardwareConcurrency: 6
> worker.crossOriginIsolated: false
>
> step 11 (30 seconds)
> Shamble Frontlines, first page: report
> backend: WebGPU
> ticks: 910 at 30 a second · blocks: 910 of 4104 bytes
> frames: 178.6 a second · frame time 5.60 ms (p99 5.70 ms)
> errors: 0
> last pause: none yet
> isolation: no · commit: 7d8e2de2e
>
> commit: 7d8e2de2e
> run: (none)
> at: 2026-11-19T14:44:28.814Z
> browser: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/158.0.0.0 Safari/537.36
> display: 2560x1440 at 1.25x, 24 bit
> hardwareConcurrency: 6
> crossOriginIsolated: false
> backend: WebGPU
> adapter: (not given)
> forceWebGL: false
> webgpuOffered: true
> timestampQuery: true
> render.drawCalls: 3
> render.triangles: 3
> render.lines: 62
> render.geometries: 3
> render.textures: 3
> render.programs: 6
> tick: 910
> tickHz: 30
> blocks: 910
> blockBytes: 4104
> inputPosts: 5466
> frames: 5468
> fps: 178.57
> frameP50: 5.60 ms
> frameP99: 5.70 ms
> deliveryP50: 0.20 ms
> deliveryP95: 0.30 ms
> errors: 0
> lastError: (none)
> pauses: 0
> lastPauseHiddenMs: (none)
> lastPauseTicksWhileHidden: (none)
> worker.ticks: 907
> worker.wakes: 908
> worker.maxTicksInOneWake: 1
> worker.drops: 0
> worker.droppedMs: 0
> worker.pauses: 0
> worker.latenessP50: 1.600
> worker.latenessP99: 4.033
> worker.latenessMax: 6.433
> worker.copyP50: 0
> worker.copyP99: 0.100
> worker.blockBytes: 4104
> worker.poolMisses: 0
> worker.poolRefused: 0
> worker.undetached: 0
> worker.hardwareConcurrency: 6
> worker.crossOriginIsolated: false
>
> step 12 (~30 seconds ish)
> Shamble Frontlines, first page: report
> backend: WebGPU
> ticks: 1677 at 30 a second · blocks: 1677 of 65544 bytes
> frames: 178.6 a second · frame time 5.60 ms (p99 5.80 ms)
> errors: 0
> last pause: none yet
> isolation: no · commit: 7d8e2de2e
>
> commit: 7d8e2de2e
> run: (none)
> at: 2026-11-19T14:45:41.261Z
> browser: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/158.0.0.0 Safari/537.36
> display: 2560x1440 at 1.25x, 24 bit
> hardwareConcurrency: 6
> crossOriginIsolated: false
> backend: WebGPU
> adapter: (not given)
> forceWebGL: false
> webgpuOffered: true
> timestampQuery: true
> render.drawCalls: 3
> render.triangles: 3
> render.lines: 62
> render.geometries: 3
> render.textures: 3
> render.programs: 6
> tick: 1677
> tickHz: 30
> blocks: 1677
> blockBytes: 65544
> inputPosts: 10065
> frames: 10068
> fps: 178.57
> frameP50: 5.60 ms
> frameP99: 5.80 ms
> deliveryP50: -0.00 ms
> deliveryP95: 0.20 ms
> errors: 0
> lastError: (none)
> pauses: 0
> lastPauseHiddenMs: (none)
> lastPauseTicksWhileHidden: (none)
> worker.ticks: 1674
> worker.wakes: 1676
> worker.maxTicksInOneWake: 1
> worker.drops: 0
> worker.droppedMs: 0
> worker.pauses: 0
> worker.latenessP50: 2.100
> worker.latenessP99: 5.733
> worker.latenessMax: 5.867
> worker.copyP50: 0
> worker.copyP99: 0.100
> worker.blockBytes: 65544
> worker.poolMisses: 0
> worker.poolRefused: 0
> worker.undetached: 0
> worker.hardwareConcurrency: 6
> worker.crossOriginIsolated: false
>
> 1.100.58 Chromium: 158.0.8127.42 (Official Build) (64-bit) 
>
> I will have to install firefox, you can have this for now

### Message 2: Firefox: the page opened as a file (a screenshot), and the served page's report
_Chat message of 2026-11-19T14:54:19.245Z (uuid 966b7349-bd09-4725-8c14-eedd47741704), its text whole;
it also carried one picture, which is not in the repository.
Copied by script (1534 bytes, sha256 5f26b4a2176c872301b93d12257189a95e91e4bc2218e2464b63c60d96de15fa):_
> Firefox shows me this with ctrl + o (screenshot)
>
> from the firefox (didn't keep much track of the minutes, but it should be around 2-3 minutes): 
> Shamble Frontlines, first page: report
> backend: WebGPU
> ticks: 6720 at 30 a second · blocks: 6720 of 8 bytes
> frames: 239.2 a second · frame time 4.18 ms (p99 8.34 ms)
> errors: 0
> last pause: 16.8 s hidden · ticks while hidden: 0 (a match that ran on: about 505)
> isolation: no · commit: 7d8e2de2e
>
> commit: 7d8e2de2e
> run: (none)
> at: 2026-11-19T14:53:34.221Z
> browser: Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:160.0) Gecko/20100101 Firefox/160.0
> display: 2560x1440 at 1x, 24 bit
> hardwareConcurrency: 24
> crossOriginIsolated: false
> backend: WebGPU
> adapter: (not given)
> forceWebGL: false
> webgpuOffered: true
> timestampQuery: true
> render.drawCalls: 3
> render.triangles: 3
> render.lines: 62
> render.geometries: 3
> render.textures: 3
> render.programs: 6
> tick: 6720
> tickHz: 30
> blocks: 6720
> blockBytes: 8
> inputPosts: 40294
> frames: 40295
> fps: 239.23
> frameP50: 4.18 ms
> frameP99: 8.34 ms
> deliveryP50: 1.00 ms
> deliveryP95: 2.00 ms
> errors: 0
> lastError: (none)
> pauses: 1
> lastPauseHiddenMs: 16839
> lastPauseTicksWhileHidden: 0
> worker.ticks: 6717
> worker.wakes: 6982
> worker.maxTicksInOneWake: 1
> worker.drops: 0
> worker.droppedMs: 0
> worker.pauses: 1
> worker.latenessP50: 0.667
> worker.latenessP99: 4.667
> worker.latenessMax: 5.667
> worker.copyP50: 0
> worker.copyP99: 0
> worker.blockBytes: 8
> worker.poolMisses: 0
> worker.poolRefused: 0
> worker.undetached: 0
> worker.hardwareConcurrency: 24
> worker.crossOriginIsolated: false

### Message 3: Brave: two re-checks of the hidden tab
_Chat message of 2026-11-19T15:02:50.628Z (uuid 315fc809-839e-4799-a8bd-e590f9457e65), its text whole.
Copied by script (3280 bytes, sha256 48b31acd10ab0e046d1a041be535d9fccd7c70c14674ccea1365c5c7e28de2b5):_
> Brave re-check (server still running from `node serve.mjs`):
>
> 1. Open `http://127.0.0.1:4173` in Brave.
> 2. Press Ctrl+T to open a new tab in the same Brave window. Leave Brave on that new tab for 1 minute.
> 3. Click back on the page’s tab. Press Copy report and paste it here.
>
> Done: 
> Shamble Frontlines, first page: report
> backend: WebGPU
> ticks: 407 at 30 a second · blocks: 407 of 8 bytes
> frames: 178.6 a second · frame time 5.60 ms (p99 5.70 ms)
> errors: 0
> last pause: 45.0 s hidden · ticks while hidden: 0 (a match that ran on: about 1349)
> isolation: no · commit: 7d8e2de2e
>
> commit: 7d8e2de2e
> run: (none)
> at: 2026-11-19T14:55:42.550Z
> browser: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/158.0.0.0 Safari/537.36
> display: 2560x1440 at 1.25x, 24 bit
> hardwareConcurrency: 6
> crossOriginIsolated: false
> backend: WebGPU
> adapter: (not given)
> forceWebGL: false
> webgpuOffered: true
> timestampQuery: true
> render.drawCalls: 3
> render.triangles: 3
> render.lines: 62
> render.geometries: 3
> render.textures: 3
> render.programs: 6
> tick: 407
> tickHz: 30
> blocks: 407
> blockBytes: 8
> inputPosts: 2443
> frames: 2444
> fps: 178.57
> frameP50: 5.60 ms
> frameP99: 5.70 ms
> deliveryP50: 0.10 ms
> deliveryP95: 0.30 ms
> errors: 0
> lastError: (none)
> pauses: 1
> lastPauseHiddenMs: 44979
> lastPauseTicksWhileHidden: 0
> worker.ticks: 400
> worker.wakes: 402
> worker.maxTicksInOneWake: 1
> worker.drops: 0
> worker.droppedMs: 0
> worker.pauses: 1
> worker.latenessP50: 1.600
> worker.latenessP99: 5.733
> worker.latenessMax: 5.867
> worker.copyP50: 0
> worker.copyP99: 0.100
> worker.blockBytes: 8
> worker.poolMisses: 0
> worker.poolRefused: 0
> worker.undetached: 0
> worker.hardwareConcurrency: 6
> worker.crossOriginIsolated: false
>
> Press Ctrl+T again and leave Brave on the new tab for 6 minutes (a good moment to install Firefox). Click back on the page’s tab, Copy report, paste.
>
> Done: 
> Shamble Frontlines, first page: report
> backend: WebGPU
> ticks: 164 at 30 a second · blocks: 164 of 8 bytes
> frames: 178.6 a second · frame time 5.60 ms (p99 5.80 ms)
> errors: 0
> last pause: 59.5 s hidden · ticks while hidden: 0 (a match that ran on: about 1786)
> isolation: no · commit: 7d8e2de2e
>
> commit: 7d8e2de2e
> run: (none)
> at: 2026-11-19T15:02:36.683Z
> browser: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/158.0.0.0 Safari/537.36
> display: 2560x1440 at 1.25x, 24 bit
> hardwareConcurrency: 6
> crossOriginIsolated: false
> backend: WebGPU
> adapter: (not given)
> forceWebGL: false
> webgpuOffered: true
> timestampQuery: true
> render.drawCalls: 3
> render.triangles: 3
> render.lines: 62
> render.geometries: 3
> render.textures: 3
> render.programs: 6
> tick: 164
> tickHz: 30
> blocks: 164
> blockBytes: 8
> inputPosts: 996
> frames: 999
> fps: 178.57
> frameP50: 5.60 ms
> frameP99: 5.80 ms
> deliveryP50: 0.20 ms
> deliveryP95: 0.30 ms
> errors: 0
> lastError: (none)
> pauses: 3
> lastPauseHiddenMs: 59534
> lastPauseTicksWhileHidden: 0
> worker.ticks: 162
> worker.wakes: 166
> worker.maxTicksInOneWake: 1
> worker.drops: 0
> worker.droppedMs: 0
> worker.pauses: 3
> worker.latenessP50: 1.100
> worker.latenessP99: 4.400
> worker.latenessMax: 4.467
> worker.copyP50: 0
> worker.copyP99: 0.100
> worker.blockBytes: 8
> worker.poolMisses: 0
> worker.poolRefused: 0
> worker.undetached: 0
> worker.hardwareConcurrency: 6
> worker.crossOriginIsolated: false

### Message 4: Firefox: the report of `?backend=webgl`
_Chat message of 2026-11-19T15:09:29.950Z (uuid bb86eb04-fd3f-4434-b4f4-430add72a0cf), its text whole.
Copied by script (1601 bytes, sha256 9ba59018e26664d166dc919935e573ed8ef7dc3c75dd3434d69a90871ec968df):_
> ~30 seconds from firefox:
> Shamble Frontlines, first page: report
> backend: WebGL 2 fallback, asked for by ?backend=webgl (ANGLE (NVIDIA, NVIDIA GeForce GTX 980 Direct3D11 vs_5_0 ps_5_0), or similar)
> ticks: 1022 at 30 a second · blocks: 1022 of 8 bytes
> frames: 239.2 a second · frame time 4.18 ms (p99 8.34 ms)
> errors: 0
> last pause: 3.8 s hidden · ticks while hidden: 0 (a match that ran on: about 114)
> isolation: no · commit: 7d8e2de2e
>
> commit: 7d8e2de2e
> run: (none)
> at: 2026-11-19T15:09:11.435Z
> browser: Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:160.0) Gecko/20100101 Firefox/160.0
> display: 2560x1440 at 1x, 24 bit
> hardwareConcurrency: 24
> crossOriginIsolated: false
> backend: WebGL 2 fallback
> adapter: ANGLE (NVIDIA, NVIDIA GeForce GTX 980 Direct3D11 vs_5_0 ps_5_0), or similar
> forceWebGL: true
> webgpuOffered: true
> timestampQuery: false
> render.drawCalls: 3
> render.triangles: 3
> render.lines: 62
> render.geometries: 3
> render.textures: 3
> render.programs: 6
> tick: 1022
> tickHz: 30
> blocks: 1022
> blockBytes: 8
> inputPosts: 6127
> frames: 6128
> fps: 239.23
> frameP50: 4.18 ms
> frameP99: 8.34 ms
> deliveryP50: 0.00 ms
> deliveryP95: 1.00 ms
> errors: 0
> lastError: (none)
> pauses: 1
> lastPauseHiddenMs: 3796
> lastPauseTicksWhileHidden: 0
> worker.ticks: 1018
> worker.wakes: 1126
> worker.maxTicksInOneWake: 1
> worker.drops: 0
> worker.droppedMs: 0
> worker.pauses: 1
> worker.latenessP50: 0.667
> worker.latenessP99: 4.667
> worker.latenessMax: 5.333
> worker.copyP50: 0
> worker.copyP99: 0
> worker.blockBytes: 8
> worker.poolMisses: 0
> worker.poolRefused: 0
> worker.undetached: 0
> worker.hardwareConcurrency: 24
> worker.crossOriginIsolated: false
