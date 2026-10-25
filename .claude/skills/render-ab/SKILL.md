---
name: render-ab
description: 'Before/after visual evidence for a change-set: 3D renders with tools/preview.js and real-page renders (DOM, HUD, CSS) with tools/pageshot.js, base ref vs working tree, saved as labelled pairs. Use for any visual, UI, lighting, material or animation change.'
argument-hint: '<views, e.g. "menu, pause, village -140,-20, manor 0,-104">'
---
Views requested: $ARGUMENTS

1. Base: `git worktree add .gate/base <baseRef>` (default: merge-base with main); reuse it if already current.
2. For each view, render base and head with identical commands and settings:
   - 3D gameplay view: `node tools/preview.js <dir> shot <out.png> <x> <z>`; menu camera: `... menu <out.png>`.
   - Pages / UI: `node tools/pageshot.js <dir> index.html <prefix> <steps…>` at 1600x900 and 1280x720; use headed GPU
     mode (`--headed`) for anything lighting- or performance-sensitive.
   - Default views when none are given: centre (0,0), west hamlet (-140,-20), manor (0,-104), a base (-150,90), menu.
3. Save as `.gate/renders/<branch>/<view>_base.png` and `<view>_head.png`; add a side-by-side sheet if you can.
4. Look at every pair yourself. One line each: what visibly changed and whether that was intended.
5. State the limits that apply (WebGL1 or SwiftShader, substitute fonts) and finish with a Brave checklist for Mustafa:
   exact screens, positions, states and hovers to screenshot.
