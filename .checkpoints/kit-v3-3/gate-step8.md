The full gate with smoke for kit-v3-3, step 8, run by the main session on 2026-11-05 ~02:40 UTC in this worktree
(working tree: steps 2 to 7 committed, step 4c uncommitted), with PLAYWRIGHT_BROWSERS_PATH set to the scratchpad's
Chromium build 1194 (the container has 1243). Output, verbatim:

```
SHAMBLE FRONTLINES gate | base merge-base(HEAD, origin/main) = 12d7961 | head = working tree
note: local main is behind origin/main, so origin/main is used (update it: git fetch origin main:main)
[1]  syntax         PASS  5/5 sources parse (shamble-core.js, shamble-game.js, shamble-editor.js, index.html inline#1, editor.html inline#1)
[2]  concatenation  PASS  index.html: shamble-core.js + shamble-game.js + inline#1 | editor.html: shamble-core.js + shamble-game.js + shamble-editor.js + inline#1 (external, skipped: three.min.js)
[3]  symbols        PASS  core 138 / game 714 / editor 353 / indexInline 0 / editorInline 0 | +0 -0 | cross-file duplicates 0, within-file 0
[4]  player build   PASS  51 refs to editor-only symbols: 51 guarded, 0 dead, 0 flagged
[5a] smoke          PASS  index.html 8/8 steps, 0 errors (2/2 shots reached the canvas, moved 0.9 m) | editor.html 5/5 steps, 0 errors | 0 warnings | renderer ANGLE (Google, Vulkan 1.3.0 (SwiftShader Device (Subzero) (0x0000C0DE)), SwiftShader driver)
[5b] names          PASS  index.html: 3 scripts + 0 DOM handlers, 0 problems | editor.html: 4 scripts + 0 DOM handlers, 0 problems | allowed: 63 built-ins (Node v22.23.4) + 15 in gate-globals.json + 459 THREE members (r128)
[5c] layering       PASS  shamble-core.js uses 21 names it does not declare; 0 references into game/editor/inline symbols
[5d] script order   PASS  index.html: three r128 (cdnjs) -> shamble-core.js -> shamble-game.js -> inline | editor.html: three r128 (cdnjs) -> shamble-core.js -> shamble-game.js -> shamble-editor.js -> inline
GATE PASS
```

Tests at the same tree: node --test of routine-guard, settings and checkpoint: 555/555.
DONE
