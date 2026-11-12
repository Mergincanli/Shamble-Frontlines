# E8, the rollback rehearsal (foundation 2.1b; his E8-RUN (a), 2026-11-11 21:39:44 UTC)

Run by the main session in the throwaway worktree .gate/fr-e8 (one level under .gate/), 2026-11-11 about 22:37 UTC. Nothing pushed, no branch made.

1. git worktree add --detach .gate/fr-e8 origin/main -> 76b4f7a (re-fetched just before: origin/main = 76b4f7a65e8ad0fb9ba6d3752ffec2a9e673bbd3)
2. tools/node_modules linked (ln -s ../../../tools/node_modules .gate/fr-e8/tools/node_modules)
3. in the worktree: git merge --no-ff --no-edit f0322d2f810d7803ad608ec70033e589d6bf7290 (the PR head then) -> M = c33ac993ce2c25bbaf856105fc7a4864a1aa4bcf
   (the guard refused the first form, git -C .gate/fr-e8 merge ...: "git: no option before the subcommand (run git from the folder, without -C, -c or --git-dir)"; route: cd into the worktree, then git)
4. in the worktree: git revert -m 1 --no-edit HEAD -> R = 5f91112d86c727d6c7127a2bc9773387c4f13b0a
5. the reverted tree vs 76b4f7a: git diff --stat 76b4f7a HEAD -> empty (identical to main)

## node tools/gate.mjs c33ac993ce2c25bbaf856105fc7a4864a1aa4bcf (CI's base on a revert PR is the main that holds M)
```
SHAMBLE FRONTLINES gate | base c33ac993ce2c25bbaf856105fc7a4864a1aa4bcf = c33ac99 (files in legacy/) | head = working tree (files at the root)
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

## node tools/gate.mjs --smoke-only
```
SHAMBLE FRONTLINES gate | smoke only | head = working tree (files at the root)
[5a] smoke          PASS  index.html 8/8 steps, 0 errors (2/2 shots reached the canvas, moved 0.9 m) | editor.html 5/5 steps, 0 errors | 0 warnings | renderer ANGLE (Google, Vulkan 1.3.0 (SwiftShader Device (Subzero) (0x0000C0DE)), SwiftShader driver)
GATE PASS (smoke only: checks 1-4 and 5b-5d not run)
```
