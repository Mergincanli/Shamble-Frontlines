# Mustafa's browser check of the first page (#61): his reports, read against the zip's README

_Written 2026-11-19, 15:12 UTC (from the clock) by the main session, from his chat messages saved beside this file by
script (each file's first line gives the message's time, uuid, byte count and sha256). His words of 14:28:25 UTC:
"My browser reports are recorded with the next docs pass, not by a push to this head." This file is the input for that
docs pass. The zip: `shamble-v2-02-first-page.zip`, 245,953 bytes, sha256 c49d8a7f...c5f4, built from commit 7d8e2de2e._

## What came, in order
| His message (UTC) | File | What it holds |
|---|---|---|
| 14:28:25 | `mustafa-2026-11-19-1516-go-61.md` | His go on #61 and BROWSER-SET (a). It says the reports "are in this chat"; none was. The merge waited. |
| 14:31:30 | `mustafa-2026-11-19T143130.md` | "a, yes" (his answer VISION-NOTE (a): the dated note's text as put to him), and a request for the steps again. |
| 14:46:25 | `mustafa-2026-11-19T144625.md` | Brave: four reports (the served page, `?backend=webgl`, `?pad=4096`, `?pad=65536`) and the version line. |
| 14:54:19 | `mustafa-2026-11-19T145419.md` | Firefox: a screenshot of the page opened as a file (the picture is not in the notes; a copy lies in `.gate/cs2-first-page/merge/`), and the served page's report. |
| 15:02:50 | `mustafa-2026-11-19T150250.md` | Brave: two re-checks of the hidden tab. |
| 15:09:29 | `mustafa-2026-11-19T150929.md` | Firefox: the report of `?backend=webgl`. |

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

## Observations for the follow-ups (ids are this note's; none is an error count above 0)
- **BR-GAP.** Brave's first run drew 46,382 frames while it ticked for 410 s: an average of 113 a second, where every
  other Brave run averages 180 (frames over ticks: 185.0, 180.3 and 180.1 in the three short runs, 180.1 and 182.7 in the re-checks). So for about 150 s
  the page drew nothing while the simulation ran on, and no pause was recorded. Not explained by the reports: the
  window may have been covered or the screen off. The pause on a tab switch works in Brave (the re-check). To read
  again with change-set 3's browser check: what the page is told when Brave's window is covered.
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
