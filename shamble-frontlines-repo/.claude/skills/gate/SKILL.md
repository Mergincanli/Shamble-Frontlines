---
name: gate
description: 'The SHAMBLE FRONTLINES verification gate: syntax of every file and inline script, load-order concatenation for both pages, symbol conservation against a base ref, and player-build resolvability. Use after ANY edit to shamble-*.js or the HTML pages, and before every commit, report or merge.'
argument-hint: '[baseRef, default: merge-base with main]'
---
Run `node tools/gate.mjs $ARGUMENTS` and read the whole output.

If tools/gate.mjs does not exist yet (before Phase 0 lands), run the manual battery instead:
1. `node --check` each shamble-*.js, and each page's inline script extracted to a temp file.
2. Concatenate in load order and `node --check`: core+game+index-inline, and core+game+editor+editor-inline.
3. `node tools/battery.js <dir>` on a worktree of the base ref and on the working tree; diff the `symbols` lists
   (no drops, no cross-file duplicates; the diff must equal the plan's declared changes).
4. In the working-tree battery output, `check4.flagged` must be empty.

Report: PASS/FAIL per check, the symbol diff (+added / −dropped), and whether it matches the approved plan.
A FAIL blocks the change-set until fixed. Never "fix" a failure by editing the gate tools.
