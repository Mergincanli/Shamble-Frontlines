# Mustafa's browser check of change-set 3b: his two reports and their reading

_What this is: the record of his browser check of change-set 3b's zip (`shamble-v2-03b-the-loop-closes.zip`, 259,212
bytes, sha256 3fdcf9f6a1173a707cfc4f1f8620482693ba36abc6e708322b6d3628a4071f82, built from commit 8f835d114; the
commits after it change only docs and notes, and its code is the final code 6abf3d2a, the game's WebAssembly file
byte for byte the same). He ran it in Brave and in Firefox on his Windows computer and pasted what the page's "Copy
report" button gave him into the build chat (session_01WLuqnvpAh9XTitx8ehKJcQ): Brave at 2026-11-25, 11:38:37 UTC,
Firefox at 12:14:23 UTC. His two messages stand whole in `docs/DECISIONS.md` (the entries of those two times),
copied by script from the chat's transcript and never retyped. The reading here is the build chat's. The
change-set's own report: `docs/reports/2026-11-25-cs3b-the-loop-closes.md`._

_In short: every number he was asked to check is 0 in both browsers; errors 0; the commit shown is 8f835d114; WebGPU
in both; every press and release he made came back (215 of 215 in each browser). One remark of his, on the
hidden-tab step in Brave, is read below; it did not come again when he tried it a second time, or in Firefox._

## The two reports
| | Brave (report at 11:17:19 UTC) | Firefox (report at 12:12:42 UTC) |
|---|---|---|
| Browser line | "Chrome/158.0.0.0", Windows (Brave reports itself so) | "Firefox/161.0", Windows |
| Display, logical processors | 2560x1440 at 1.25x; 11 | 2048x1280 at 1.25x; 24 |
| Backend | WebGPU, adapter "(not given)" | WebGPU, adapter "(not given)" |
| Commit | 8f835d114 | 8f835d114 |
| Ticks, blocks | 5,528 and 5,528 (44 bytes each) | 4,786 and 4,786 |
| Frames | 134.5 a second (frame time 5.60 ms, p99 20.70 ms) | 179.8 a second (4.18 ms, p99 8.34 ms) |
| Messages sent, received, refused | 5,527, 5,528, **0** | 4,785, 4,786, **0** |
| Presses made, echoed, lost, doubled | 215, 215, **0**, **0** | 215, 215, **0**, **0** |
| Releases made, worked out, doubled | 215, 215, **0** | 215, 215, **0** |
| Ticks on older keys (`olderPosts`) | **0** | **0** |
| Ticks on other keys than their post's (`wrongKeys`) | **0** | **0** |
| Errors | **0** | **0** |
| Input posts, post used | 31,944 and 31,941 | 28,497 and 28,492 |
| Key to tick, median and 95th percentile | 21.50 ms and 44.80 ms over 422 key changes | 30.00 ms and 49.00 ms over 392 |
| Pauses, the last one | 1; 3.2 s hidden, 0 ticks while hidden | 3; 0.5 s hidden, 0 ticks while hidden |
| The Worker | 0 drops, at most 1 tick per wake, 0 late posts ignored, 0 pool misses | the same |
| The Worker's lateness, median, p99, max | 1.967, 14.067, 15.800 ms | 0.667, 4.667, 5 ms |

Draw calls 3, triangles 3, lines 62 in both, as before. The messages sent are one fewer than the ticks in both: the
first tick carries the join, not a command, as built.

## Against the zip's steps
- **Steps 1 to 5 and 7 to 9:** done in both browsers, by his words: of Brave "(all done, except 1 which I will
  comment under >)", of Firefox "Did the same for firefox (all steps):" and "Did some extra thorough spamming and
  testing, came in pretty clean." The six numbers of step 8 are 0 in both.
- **Step 6 (hold A, another tab for 30 seconds, come back): his one remark, in Brave.** He saw the A box still lit
  for a split second after coming back, and then dark. In Firefox the step went as the README says, and when he
  tried it again in Brave it did not happen ("I think it was either a 1 time flaw from my side and not an issue").
  **The reading, from the code:** the match stands still while the tab is hidden (both reports: 0 ticks while
  hidden); a box shows the server's last echo; the Worker lets go of every key when the tab hides, and the first
  tick after the return carries that to the server. So the picture from before the pause can show until that first
  tick, about a thirtieth of a second, when the first frame after the return is drawn before the first block
  arrives. The README's "A should be dark" was inexact. No key stayed held in the match: the match was paused, and
  `releases.doubled` and `wrongKeys` are 0. The chat asked him whether to leave it or to make the boxes go dark
  the moment the tab hides (RESUME-FLASH); he named no letter and wrote that it is not an issue. Nothing is changed;
  what the first picture after a pause shows is the follow-up 3B-RESUME-FLASH, for change-set 4, when a soldier is
  on screen.

## What the reports show beyond the steps
- **WebGPU ran in both browsers** on his machine; every headless run here and on GitHub falls back to WebGL 2.
- **Key to tick, a reading and not a judgement of the mark:** 44.8 ms at the 95th percentile in Brave at 134 frames
  a second and 49.0 ms in Firefox at 180, against the mark's 50 ms on a named machine at 60 frames a second
  (PM-0002-13). Close to the limit in Firefox; the follow-up 3B-KEY-POST (post the keys from the key handler too)
  stands before that mark is judged.
- **The Worker's pool never missed** at these frame rates (it misses on a slow page only, by design).
