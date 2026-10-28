# R3-AUDIO draft: spatial audio with Web Audio

Question 7 of R3. Author: graphics-engineer (Opus 5.5, effort high), 2026-10-28. Source summaries with exact quotes:
`.checkpoints/v2-research/graphics-engineer-r3.md` (S53–S62).

The sound design itself (what each weapon sounds like) is LOOP_V2 §7's per-weapon feel kit and is not R3's to choose.
This section covers the spatial machinery, its cost and its limits.

## Findings

**R3-AUDIO-F1.** A `PannerNode` "represents the location, direction, and behavior of an audio source signal in a
simulated physical space" and "uses right-hand Cartesian coordinates"; positions are relative to the context's one
`AudioListener`, and position and orientation are `AudioParam`s (`positionX` and so on).
· Source: https://developer.mozilla.org/en-US/docs/Web/API/PannerNode (last modified Sep 28, 2026) · official · read
2026-10-28 · confirmed in the source.

**R3-AUDIO-F2.** Two panning models: "`equalpower`: Represents the equal-power panning algorithm, generally regarded as
simple and efficient. `equalpower` is the default value." and "`HRTF`: Renders a stereo output of higher quality than
`equalpower` — it uses a convolution with measured impulse responses from human subjects."
· Source: https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/panningModel (Apr 7, 2023) · official · read
2026-10-28 · confirmed in the source.

**R3-AUDIO-F3.** Three distance models, `inverse` the default: linear `1 - rolloffFactor * (distance - refDistance) /
(maxDistance - refDistance)`; inverse `refDistance / (refDistance + rolloffFactor * (Math.max(distance, refDistance) -
refDistance))`; exponential `pow((Math.max(distance, refDistance) / refDistance, -rolloffFactor)` (as MDN prints it, with an
unbalanced parenthesis). Past `maxDistance` "the volume is not reduced any further"; three.js notes that `maxDistance` "is
used only by the `linear` distance model."
· Source: https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/distanceModel (Apr 7, 2023);
https://developer.mozilla.org/en-US/docs/Web/API/PannerNode; https://threejs.org/docs/pages/PositionalAudio.html
(setMaxDistance) · official · read 2026-10-28 · confirmed in the source.

**R3-AUDIO-F4.** Neither inverse nor exponential ever reaches zero gain, so a distant source keeps costing a voice
unless the game stops it; only linear reaches zero (at `maxDistance` with `rolloffFactor` 1).
· Source: F3 · inferred (from the formulas).

**R3-AUDIO-F5.** three.js's `PositionalAudio` creates a PannerNode with `this.panner.panningModel = 'HRTF';`, so it
uses the costlier model by default (against Web Audio's own default, F2); every `updateMatrixWorld` while playing ramps
the panner to the object's position (`panner.positionX.linearRampToValueAtTime( _position.x, endTime );`, with `endTime
= this.context.currentTime + this.listener.timeDelta`), without allocating.
· Source: https://raw.githubusercontent.com/mrdoob/three.js/r186/src/audio/PositionalAudio.js · official (source) · read
2026-10-28 · confirmed in the source for the quotes; "without allocating" is inferred (the fetch tool's reading).

**R3-AUDIO-F6.** three.js's `AudioListener` ("a virtual listener of the all positional and non-positional audio
effects") ramps the context's listener to its own world position the same way and owns a master gain
(`this.gain = this.context.createGain();`); three's example adds it to the camera.
· Source: https://raw.githubusercontent.com/mrdoob/three.js/r186/src/audio/AudioListener.js;
https://threejs.org/docs/pages/PositionalAudio.html (example) · official · read 2026-10-28 · confirmed in the source.

**R3-AUDIO-F7.** The simpler alternative, `StereoPannerNode`, is "designed to deal with the common use case of creating
simple left and right stereo panning effects" and is "much simpler to use, but obviously nowhere near as versatile.";
the panner "does some very involved math under the hood".
· Source: https://developer.mozilla.org/en-US/docs/Web/API/Web_Audio_API/Web_audio_spatialization_basics (Aug 27,
2026) · official · read 2026-10-28 · confirmed in the source.

**R3-AUDIO-F8.** Occlusion has no node of its own; the building block is a `BiquadFilterNode` ("a simple low-order
filter" with "exactly one input and one output"), whose `lowpass` type is a "Standard second-order resonant lowpass
filter with 12dB/octave rolloff. Frequencies below the cutoff pass through; frequencies above it are attenuated."
· Source: https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode (Jun 23, 2025) · official · read
2026-10-28 · confirmed in the source (that occlusion is approximated with it is inferred; no source read describes
occlusion).

**R3-AUDIO-F9.** No source read gives the CPU cost of a voice, of HRTF against equal power, or a voice limit; the Web
Audio specification's PannerNode and spatialization sections could not be reached (the fetched text of the Editor's
Draft of 27 October 2026 stops before them).
· Source: https://webaudio.github.io/web-audio-api/ (Editor's Draft, 27 October 2026; read in part) and the MDN pages
above · confirmed by absence.

**R3-AUDIO-F10.** Starting sound "outside the context of handling a user input event is subject to autoplay rules."
· Source: https://developer.mozilla.org/en-US/docs/Web/Media/Guides/Autoplay (Sep 10, 2026) · official · read
2026-10-28 · confirmed in the source.

**R3-AUDIO-F11.** With a top-down camera 15.5 to 52 m above the ground (legacy CAM_STYLES), a listener on the camera
hears every on-screen source at nearly the same distance, so distance attenuation barely separates near from far; a
listener at the player's position (oriented with the camera, so left on screen is left in the ears) keeps distance
meaningful. Legacy already measures gain from the player and pans by the camera's right axis (GP6).
· Source: docs/LEGACY_TUNING.md CAM_STYLES; legacy GP6; F3 · inferred (geometry; the v2 camera is look brief pending).

### Legacy facts used
- **GP6** (synthesized audio): every sound is generated at runtime from oscillators and noise buffers (no files);
  enemy shots and booms use inverse-distance gain and a `StereoPannerNode` against the camera's right axis; the
  AudioContext starts on the first pointer or key press; most shots build a fresh AudioBuffer in a JS loop (only the AMK
  and war-room sounds cache theirs); "there is no voice limit and no occlusion"; the only limits are a 55 ms impact
  throttle and the 90 m bot-shot sound cull. The row asks R3 for the distance model, the occlusion question and a spike
  that "counts concurrent voices in a 48-bot fight".
- **GN8 / TUNE.fx**: `botSfxR` 90 m and `botCullR` 50 m (LEGACY_TUNING); shots at or hitting the player are never culled.
- **GC16**: the Juggernaut's footfalls play a thunk with camera shake: large units have their own sounds (VISION_V2 §6.9
  "and their effects").
- **GW15**: legacy keeps an explosion line-of-sight list (`rebuildLOS`, `losMeshes`); occlusion could reuse a client
  copy of the static geometry the same way.
- **R1-TICK-REC5 / R1-BND-REC2** (wave 1): the sim runs in a Worker and the page receives events per tick (hits,
  deaths, …); sounds are triggered on the page from those events (inferred; R1's event section).

## Options

Spatialization:
| Option | What it is | CPU | Quality | Fit |
|---|---|---|---|---|
| P-A. `StereoPannerNode` plus own gain (legacy) | left-right pan and a computed distance gain | lowest (F7) | no front-back or height cues | Fits a top-down view where on-screen left and right matter most |
| P-B. `PannerNode` with `equalpower` | 3D position against a listener, built-in distance model (F1–F3) | low ("simple and efficient", F2) | left-right with distance; no binaural cues | One listener, data-driven distance curves |
| P-C. `PannerNode` with `HRTF` (three.js's default, F5) | convolution with measured responses (F2) | highest; unmeasured (F9) | best on headphones | Only for few, important sources |
| P-D. Mixed: HRTF for a few priority voices, equal power for the rest | P-C + P-B by priority | bounded | good where it matters | Needs a priority system anyway (V-B) |

Voice management:
| Option | What it is | Trade-off |
|---|---|---|
| V-A. No limit (legacy) | every event plays | Unbounded nodes in a 48-player fight (GP6) |
| V-B. Voice cap with priorities | a fixed number of voices; priority by kind (own weapon, shots at the player, large units, nearby, far), distance and age; the lowest is stolen or never started; per-kind caps and throttles | Bounded cost; needs tuning; quiet sources vanish |
| V-C. Distance cull only (legacy's 90 m) | sources beyond a radius never start | Simple, but a cluster within the radius is still unbounded |

Occlusion:
| Option | What it is | Cost | Risk |
|---|---|---|---|
| O-A. None (legacy) | – | none | Sounds through walls read as in the open |
| O-B. Ray test from listener to source against static geometry, lowpass plus gain (F8) | one ray per playing voice, at a low rate | one filter per voice; rays on the CPU | Thin walls; floors need height-aware rays (VISION_V2 §6.2) |
| O-C. Room or zone based | "inside building X" flags from the map data; muffle between zones | cheap | Coarse |

## Recommendations

**R3-AUDIO-REC1. One `AudioListener` at the local player's position, oriented with the camera; sources as `PannerNode`s
with `equalpower` by default (P-B), the distance model and its numbers as per-weapon data (LOOP_V2 §7 feel kit); HRTF
only as a measured option for a few priority voices (P-D).**
Reason: a panner against one listener replaces legacy's split (gain from the player, pan from the camera, GP6) with one
model (F1, F3); equal power is the efficient model (F2), while three's `PositionalAudio` silently picks HRTF (F5) and
no source gives HRTF's cost (F9); a listener at the camera flattens distance under a top-down view (F11).
Rests on: R3-AUDIO-F1, F2, F3, F5, F6, F9, F11; legacy GP6.
Serves: VISION_V2 §3 (48+ players: hearing where a fight is), §6.9 (large units and their effects).
Spike must prove (proposed): with 32 concurrent equal-power voices, a 5-minute recorded capture has no dropout (checked by
a listener and by the AudioContext staying in the "running" state) and main-thread audio work stays under
0.5 ms per frame on the minimum machine; HRTF for 4 priority voices adds less than 0.3 ms.

**R3-AUDIO-REC2. A voice manager with a cap and priorities (V-B) plus distance culling: proposed start of 32 voices on
Low and 48 on High, priorities as data (own weapon and shots at the player first, then large units, then by distance),
per-kind throttles as legacy's 55 ms impact throttle, pooled nodes and pre-built noise buffers.**
Reason: inverse and exponential gains never reach zero (F4), so without a cap every distant shot of a 48-player fight
keeps a voice; legacy has no cap (GP6) and only a 90 m cull (GN8); it also builds a buffer per shot (GP6). Proposed
numbers are starting points; no source gives a voice budget (F9).
Rests on: R3-AUDIO-F3, F4, F9; legacy GP6, GN8.
Serves: VISION_V2 §3, §6.9.
Spike must prove (proposed): in the benchmark fight (R3-MEASURE-REC2), the count of live source nodes never exceeds the
cap, shots at the player are never dropped (0 of all such events in 3 runs), and audio code allocates no new
AudioBuffer after warm-up.

**R3-AUDIO-REC3. Occlusion starts as option O-C (zones from map data) or none; O-B (rays plus a lowpass, F8) only if the
spike shows players cannot tell sounds behind walls, and then at a low ray rate per voice, height-aware.**
Reason: no source read describes occlusion or its cost (F8, F9); legacy has none (GP6); the inventory asks "whether
occlusion is worth its cost". Rays against static geometry are cheap per voice but need floors and heights (VISION_V2
§6.2), which R4's line-of-sight work owns.
Rests on: R3-AUDIO-F8, F9; legacy GP6, GW15.
Serves: VISION_V2 §6.2 (floors and building interiors), §6.9.
Spike must prove (proposed): with O-B on, at most 1 ray per voice per 100 ms and at most 0.2 ms per frame for all rays at
the voice cap on the minimum machine; a blind listening test where 4 of 5 testers tell an occluded source from an open
one.

**R3-AUDIO-REC4. Sounds are triggered on the page from the per-tick event section (R1-BND-REC2) and from interpolated
positions (R1-TICK-REC5), never from the sim directly; the AudioContext starts suspended and resumes on the first user
input (F10), as legacy does.**
Reason: the sim runs in a Worker (R1-TICK-REC5), Web Audio plays on the page's context, and autoplay rules gate the
start (F10); legacy's first-input start (GP6) already works.
Rests on: R3-AUDIO-F10; legacy GP6; wave 1 R1-BND-REC2, R1-TICK-REC5 (cited).
Serves: VISION_V2 §4 (all four browsers), §6.9 (large units and their effects heard in step with what is drawn).
Spike must prove (proposed): sound onset trails the drawn event by at most one rendered frame plus the output
latency, measured on 100 shots; zero autoplay warnings in the console on all four browsers.

## Open questions

- **R3-AUDIO-OQ1. HRTF cost.** No allow-listed source gives it (F9); the Chromium and W3C mailing-list discussions found
  by search are off the allow list. Decided by: REC1's spike mark.
- **R3-AUDIO-OQ2. Voice counts in a real fight.** The inventory asks for a count of concurrent voices in a 48-bot fight
  (GP6); no measurement exists. Decided by: the spike's counter under REC2.
- **R3-AUDIO-OQ3. Listener placement.** F11 assumes a legacy-like top-down camera; the camera is look brief pending.
  Decided by: the look brief's camera, then a listening check by Mustafa.
- **R3-AUDIO-OQ4. Spec details.** The specification's PannerNode and spatialization sections were not reached (F9);
  MDN's pages stood in. Decided by: a verifier read of https://www.w3.org/TR/webaudio-1.1/ in parts.
- **R3-AUDIO-OQ5. Occlusion ownership.** Rays across floors depend on R4's line-of-sight data. Decided by: R4 and the
  synthesis.

### Recommendations that hinge on a source not read
| Recommendation | Source | Reason not read |
|---|---|---|
| R3-AUDIO-REC1 | Web Audio API specification, PannerNode and §6 spatialization (webaudio.github.io, w3.org/TR/webaudio-1.1) | fetched text truncated before those sections (MDN read instead) |
| R3-AUDIO-REC1 | W3C public-audio list, 2014 (HRTF cost; lists.w3.org) | outside the allow list |
| R3-AUDIO-REC1 | Chromium code review 652073003 (equal-power default; codereview.chromium.org) | outside the allow list |

## Sources
Read (all 2026-10-28): S53 Web Audio Editor's Draft (official, in part); S54 MDN distanceModel; S55 MDN panningModel; S56
MDN PannerNode; S57 three.js PositionalAudio docs; S58 PositionalAudio.js r186; S59 AudioListener.js r186; S60 MDN Web
audio spatialization basics; S61 MDN BiquadFilterNode; S62 MDN Autoplay guide. All official.
Not read: the three rows above; abratabia.com spatial audio guide (search lead, outside the allow list).

## Cross-topic notes
- R4: occlusion rays (REC3) want the same static geometry and floor data as line of sight.
- LOOP_V2 §7: distance curves and voice priorities sit in the per-weapon feel-kit data.
