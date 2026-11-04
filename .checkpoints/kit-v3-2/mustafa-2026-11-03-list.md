# Mustafa's task list of 2026-11-03, word for word
Source: session_0173vVo5sT21uP4DcQtKvmAU, user message at 2026-11-03T22:08:25Z (read with list_events, 2026-11-04
~20:05 UTC).

```text
Small change-sets, one at a time, plan first (Mustafa, 2026-11-03).
A. Kit (rule changes: each needs my go).
 1. Update PR #30 (kit-v3-1) onto main, resolve its conflicts, have it re-reviewed, then wait for my go.
 2. kit-v3-2, into .claude/CHECKPOINTS.md for every session: at most 3 agents at a time; every agent writes its plan into its notes before writing, then saves as it goes (each finished block to disk at once, a notes line after each finding, a restart continues after the last complete block); scratch files live in the notes folder, text and small scripts only, never build output or binaries (builds go to a temp folder); the background saver is checked and restarted at every checkpoint update. Also TOY-BOX: reword CLAUDE.md's "Red Alert 3 toy-box look" and the art-director's matching lines to point at LOOK_V2.
B. Docs:
 1. EDITOR_V2: add "Plan view: draw to build". Two views of the same map document side by side: a top-down plan where you draw outlines on a grid (lines, rectangles, hexagons, octagons, free polygons), and the 3D scene, which builds itself live from the drawing. A drawn outline is a footprint with a height and a kind (wall, building, floor, cover, ramp, zone); the kind decides what it becomes, e.g. a building footprint becomes a building-kit house with floors and a roof. Anything can be moved, rescaled and reshaped in either view; both stay in sync because both read the same document. Works with three-team mirroring (draw in one third; the others follow at 120°) and with floors. Precedents: Hammer's 2D views, Doom-style sector editors, SketchUp's push-pull. Built with Editor v2 (slice 5); no code now.
 2. VISION_V2: a short "Community servers (later)" note: why (bigger or longer matches, community growth, a safety net if the game grows faster than our funds, possible revenue such as renting out official private servers), the trust rule (progression counts on official servers only), and precedents (Counter-Strike and Rust community servers, Battlefield's rented servers, Minecraft Realms).
```
