Mustafa's words for B.1 (b1-plan-view), word for word.

2026-11-03, his task list (session_0173vVo5sT21uP4DcQtKvmAU, his message at 2026-11-03T22:08:25Z; copied from A3's
queue file `.checkpoints/kit-v3-3/queue/mustafa-2026-11-03-list.md` at 4f268da), item B.1 as he wrote it (under "B. Docs:",
after his line "Small change-sets, one at a time, plan first (Mustafa, 2026-11-03)."):
```text
 1. EDITOR_V2: add "Plan view: draw to build". Two views of the same map document side by side: a top-down plan where you draw outlines on a grid (lines, rectangles, hexagons, octagons, free polygons), and the 3D scene, which builds itself live from the drawing. A drawn outline is a footprint with a height and a kind (wall, building, floor, cover, ramp, zone); the kind decides what it becomes, e.g. a building footprint becomes a building-kit house with floors and a roof. Anything can be moved, rescaled and reshaped in either view; both stay in sync because both read the same document. Works with three-team mirroring (draw in one third; the others follow at 120°) and with floors. Precedents: Hammer's 2D views, Doom-style sector editors, SketchUp's push-pull. Built with Editor v2 (slice 5); no code now.
```

2026-11-05 (~16:25 UTC), the A2 and A3 day session (session_0173vVo7FcYUpbWgRW8oQyHU), his B.1 addition (copied from
A3's queue file `.checkpoints/kit-v3-3/queue/mustafa-2026-11-05-a4-b1.md` at 4f268da):
```text
B.1 addition: record these editor decisions from my talk with the Advisor in EDITOR_V2, together with the plan view, marked as below:
- Decided, the tool order: (1) greybox now, art later: you build with blocks, and the same layout swaps to real art with one switch, like modular kits; (2) play instantly: one key drops you in at the cursor with bots, and you can keep editing while the match runs; (3) bot test runs with heatmaps (where fights and deaths happen, how long each team needs to reach each objective); (4) a fairness panel (each team's distance and time to every objective under 120° mirroring); (5) sightline checks from the game camera, last, as a balancing tool.
- Decided: all the basic primitives (cube, sphere, cylinder, plane and the like), with one consistent scale across the engine. Principles: 1 unit = 1 metre; textures tile in world space (they repeat per metre and never stretch); primitives snap to gameplay sizes (exact numbers later, once they can be tested in the editor).
- Decided: every object has width, length and height, set with drag handles or typed numbers, plus how high it sits. The old editor lacked this.
- Decided: the plan view looks straight down, flat (orthographic, like a blueprint); the 3D view flies freely; one key shows the real game camera angle.
- Decided: trace sketches (a photo of a paper sketch under the plan view, to draw over); sticky notes on the map; snapshots, kept lightweight (they store only what changed).
- Decided, lower priority: a travel-time ruler showing metres and seconds, using the game's own movement data for soldiers, vehicles and leapers.
- Decided: instances (stamps) are an optional, top-level feature with three edit levels: all copies; a tagged group (e.g. "room A") for variations and art passes; one copy as an override (like Unity's prefab variants and overrides). A stamp can also be a plain, unlinked copy.
- Principle: a lean, solid editor, planned and built with the same care as the game; no bloat.
```

2026-11-05 (~16:25 UTC), same message, his A4-FREEZE answer (it sets the queue: B.1 and B.2 are docs and run after
A3; A4 waits for the kit freeze to lift):
"A4-FREEZE: (a). A4 waits until the netcode proof test is under way."
