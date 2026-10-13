---
paths:
  - "**/shamble-game.js"
---
# Rules for shamble-game.js (gameplay + shared dispatch)
- `PLAYER_FACTION` sits at the very top because the player is built while the script loads (temporal dead zone).
  Code that runs at load must never touch a `const`/`let` declared later in the file.
- Every new top-level `let` joins the let warm-up block at the end of the file (V8 const-tracking deopts). For new
  module state prefer one `const` state object with pre-declared fields.
- Hot paths (`loop`, `update`, animation, AI, FX, occlusion): no allocation per frame. No `Math.hypot` (use `hyp2`/`hyp3`),
  no per-frame closures, arrays or objects, no destructuring `for…of`; inline math in tight loops; pre-declare fields on
  long-lived objects (enemies, `ps`).
- `loop()` owns `renderer.shadowMap.autoUpdate` (the shadow map is reused during hit-stop). Never set it elsewhere.
- Per-frame DOM writes go through `_sd` / `_so` / `_st` (write only on change).
- FX go through `FXPool` (per-kind caps); never dispose pooled meshes or shared materials. Damage numbers use `_dnPool`.
- Bots: the `NAV` grid is built at boot, rebuilt in `deploy()` (~40 ms) and invalidated by `clearMap`.
- ANIM v2 is cosmetic: the player fires from `playerFireOrigin()`, bots from a fixed chest point. After any animation
  change, re-measure the docs/ANIMATION_PLAN.md §6 metrics.
- War-room UI behaviour (`wr*`, `openManual` / `closeManual`) lives here; its DOM and CSS live in both pages.
- Every reference to an editor symbol must be mode- or `typeof`-guarded: the player build has no editor.
