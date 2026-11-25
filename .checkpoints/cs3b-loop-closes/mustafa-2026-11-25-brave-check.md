# Mustafa, 2026-11-25: RELEASE-COUNT (a) with two conditions, and his browser check in Brave (the Copy report and his note on the hidden-tab step)

_Copied by script from the chat's transcript (session_01WLuqnvpAh9XTitx8ehKJcQ), never retyped: the user message of 2026-11-25T11:38:37.977Z (uuid 1c462a05-585f-4152-955c-ff2492c2dada), whole (4879 bytes, sha256 76f8c4f841dda18886a8077bc418b5dd1f2bd1424e955d6b39aef27d5407d30b). What it answers: Builder V3's question of 2026-11-25, about 04:30 UTC (question-2026-11-25-browser-check.md): the browser check of change-set 3b and RELEASE-COUNT_

---
For Builder V3, from me, written with my main Advisor (25 November, 12:15 Amsterdam).

1. RELEASE-COUNT (how releases are counted for the presses mark PM-0002-12): (a), accept as built. The messages carry press counters only, as ADR 0001 part 5 has it, and releases are worked out from the presses and the held keys. No release counter is added in change-set 3c.
2. Two conditions. First: the report keeps saying that presses were counted by the row's own method and releases by working them out, and it calls no part of that mark passed. The mark's numbers stay as the row has them. Second: the two drafts are made to agree at their next revision (ADR 0002 part 5's sentence about press and release edge counters, and the method cell of the row PM-0002-12), with the reason written there. That revision comes to me before it is written. Put it on the follow-up list under a name.
3. My browser check comes in my next message: the Copy reports from Brave and from Firefox.
4. After my check, as your question file says: the last commit, your advisor agent's approval of exactly that commit, GitHub's checks green on it. Then give me its full commit id in one line and push nothing after it. My main Advisor opens the pull request from pr-title.txt and pr-body.md and checks it, and I tap the merge.
5. On the board, set your thread back to doing now, and to done after the merge.

Answer in at most 6 lines. Plain words, as before.

your request: (all done, except 1 which I will comment under >)
The browser check, about 20 minutes, Brave first and then Firefox once, with the zip above. Its README.txt has the ten steps. In short:
Unzip, run node serve.mjs in the folder, and open http://127.0.0.1:4173 in Brave with the window in front. You should see the grey ground, six boxes at the bottom left (W, A, S, D, Space, Shift), "errors: 0" and the commit 8f835d114.
Hold W: its box lights a moment later and stays lit. Let go: it goes dark.
Press S, D, left Shift and right Shift in turn: each lights only its own box.
Tap Space ten times: "presses" ends with the same number made and echoed, "0 lost" and "0 doubled".
Hold D, click on another window, let go of D there, and click back: D is dark.
Hold A, switch to another tab for 30 seconds, and come back: A is dark.
>I did this and switched to another tab on the same browser, but when I opened it back again I could see for a very split second that it was still going on and that it got reset after I had tabbed back, but it was actually still going until the moment I tabbed back.
Cover Brave's window for a minute, then bring it back.
Leave it 2 minutes, press "Copy report" and paste the result here. These should all be 0: messages.refused, olderPosts, wrongKeys, presses.lost, presses.doubled, releases.doubled.
In Firefox: steps 2 to 5 once, wait 2 minutes, "Copy report", and paste it here.
Anything else you see is what the check is for: paste what you see.

report: (I have pressed a bit more and did some extra checks)
Shamble Frontlines, first page: report
backend: WebGPU
ticks: 5528 at 30 a second · blocks: 5528 of 44 bytes
frames: 134.5 a second · frame time 5.60 ms (p99 20.70 ms)
messages: sent 5527 · received 5528 · refused 0
presses: 215 made · 215 echoed · 0 lost · 0 doubled
errors: 0
last pause: 3.2 s hidden · ticks while hidden: 0 (a match that ran on: about 96)
isolation: no · commit: 8f835d114

commit: 8f835d114
run: (none)
at: 2026-11-25T11:17:19.017Z
browser: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/158.0.0.0 Safari/537.36
display: 2560x1440 at 1.25x, 24 bit
hardwareConcurrency: 11
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
tick: 5528
tickHz: 30
blocks: 5528
blockBytes: 44
inputPosts: 31944
messages.sent: 5527
messages.received: 5528
messages.refused: 0
postUsed: 31941
olderPosts: 0
wrongKeys: 0
keys.bound: W A S D Space Shift
presses.made: 215
presses.echoed: 215
presses.lost: 0
presses.doubled: 0
releases.made: 215
releases.echoed: 215
releases.doubled: 0
keyToTickP50: 21.50 ms
keyToTickP95: 44.80 ms
keyToTickCount: 422
frames: 31945
fps: 134.46
frameP50: 5.60 ms
frameP99: 20.70 ms
deliveryP50: 0.00 ms
deliveryP95: 0.20 ms
errors: 0
lastError: (none)
pauses: 1
lastPauseHiddenMs: 3200
lastPauseTicksWhileHidden: 0
worker.ticks: 5522
worker.wakes: 5524
worker.maxTicksInOneWake: 1
worker.drops: 0
worker.droppedMs: 0
worker.pauses: 1
worker.stalePosts: 0
worker.latenessP50: 1.967
worker.latenessP99: 14.067
worker.latenessMax: 15.800
worker.copyP50: 0
worker.copyP99: 0.100
worker.blockBytes: 44
worker.poolMisses: 0
worker.poolRefused: 0
worker.undetached: 0
worker.hardwareConcurrency: 11
worker.crossOriginIsolated: false
