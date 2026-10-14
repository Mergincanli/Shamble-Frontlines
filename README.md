# Shamble Frontlines

A browser-based top-down team shooter: Foxhole's overhead camera, C&C Renegade's base-assault economy, Red Alert 3's
toy-box look and three factions (Soviets, Allies, Empire), and Gears of War's weighty gun feel. Built in Three.js r128
in the locked "Living Comic" style: flat two-band cel shading with bold ink outlines.

**Today:** PvE against 6 bots: four classes, a two-currency economy, the Juggernaut walker, destroy the enemy HQ to win.
**Next:** multiplayer on an authoritative server, up to 16v16 or 12v12v12 (squads of 4).

## Play
- Double-click `index.html`. It needs an internet connection for now: three.js loads from a CDN.
- `editor.html` is the authoring build: Map Editor, Character Lab, Anim Lab.
- Controls: open the main menu's **Field manual** tab, or press Esc in game.

## Repository layout
| Path | What it is |
|---|---|
| `shamble-core.js` | Engine: render pipeline (NPR), characters and factions, gun and building kits, map data, simulation math |
| `shamble-game.js` | Gameplay, AI, economy, HUD, menus, animation, the main loop and `bootGame` |
| `shamble-editor.js` | Authoring tools (loaded by editor.html only) |
| `index.html` / `editor.html` | Player build / authoring build |
| `docs/` | Status, design, art direction, architecture, roadmap, decisions (ADRs), reports, reference material |
| `tools/` | Verification toolkit (headless renderer, verify battery, match simulator): never shipped |
| `CLAUDE.md`, `.claude/` | Instructions, agents, skills, rules and hooks for Claude Code |

## Where to start reading
`docs/STATUS.md` (where things stand) → `docs/START_HERE.md` (how it's built) → `docs/GAME_DESIGN.md` and
`docs/DESIGN_NOTES.md` (why, and what's still open) → `docs/ART_DIRECTION.md` (the look).

## How work happens
Development runs through Claude Code: one change-set per branch, a plan approved before any edit, every change
verified by the gate and reviewed by project agents, visual work confirmed in Brave before merge. See `CLAUDE.md`.

Private project. All rights reserved.
