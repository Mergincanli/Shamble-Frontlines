# foundation-restructure 2.1a: the edited gate against each kind of base, and the layout checks (2026-11-10)

All runs `node tools/gate.mjs <base> --no-smoke --json` in throwaway worktrees built from commits:
- `.gate/fdn-base-new` at dd12aeb (the 2.1a commit: edited gate, files at the root)
- `.gate/fdn-base-leg` at 9af08f7 (dd12aeb plus a local, never-pushed commit of `git mv` of the five files into legacy/)
- `.gate/fdn-base-old` at 44446a5 (OLD: main's gate, files at the root)

| Gate, head | Base | Result | Base prefix | Check 3 |
|---|---|---|---|---|
| edited, root (dd12aeb) | 44446a5 (root) | PASS | "" | +0 -0 |
| edited, root | 8e58937 (CS-1's `shamble-frontlines-repo/`) | PASS | `shamble-frontlines-repo/` | +34 -0 |
| edited, root | 9af08f7 (legacy/): the revert case | PASS | `legacy/` | +0 -0 |
| edited, root | dd12aeb (own head) | PASS | "" | +0 -0 |
| edited, legacy/ (9af08f7) | 44446a5 (root) | PASS | "" | +0 -0 |
| edited, legacy/ | 8e58937 (CS-1) | PASS | `shamble-frontlines-repo/` | +34 -0 |
| edited, legacy/ | 9af08f7 (own head) | PASS | `legacy/` | +0 -0 |
| OLD, root (44446a5) | 44446a5 | PASS | (not printed by OLD) | +0 -0 |
| OLD, root | 8e58937 (CS-1) | PASS | (not printed) | +34 -0 |
| OLD, root | 9af08f7 (legacy/) | DIES, exit 2: "gate: shamble-core.js not found at base 9af08f7" | | |

The last row is the critic's F1 shown on real git: today's gate cannot read a base with the files in legacy/, which is
what CI would meet on the revert of a one-PR 2.1. With 2.1a's gate on main first, that base reads (third row).

Both copies at once (dd12aeb worktree, the five files also copied into legacy/): exit 2, "gate: the legacy engine files
are in two places, legacy/ (shamble-core.js, shamble-game.js, shamble-editor.js) and the repo root (...): keep one copy".
The copies were then deleted by name and the worktree is clean (but for the untracked `tools/node_modules` link).

The folder rule as a table (scratch copies, `rows-test.mjs`): 8 rows and 2 smoke-CLI rows, ALL PASS (root only;
legacy/ only; both: exit 2; neither: today's "shamble-core.js is missing from the working tree", exit 2; partial in
legacy/: "legacy/shamble-editor.js is missing", exit 2; root plus a stray legacy/ engine file: exit 2; partial at the
root: "shamble-editor.js is missing", exit 2; legacy/ plus a stray root index.html: PASS, build legacy/).

Without Playwright (`npm ci --omit=optional`, as CI's gate job): `--no-smoke` PASS on a legacy/ layout (`.gate/fdn-noopt`).

Full gate with smoke on the main folder after the write (root layout, base 44446a5): GATE PASS (1-4, 5a-5d; symbols
+0 -0; smoke index 8/8, editor 5/5). Local smoke runs Chromium 1243 through `.gate/pw-browsers` (the pinned 1194 is not
installed here); CI runs the pinned one.
