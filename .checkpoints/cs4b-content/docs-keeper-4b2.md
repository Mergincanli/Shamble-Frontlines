# docs-keeper, 4b-2 docs pass ("The world", steps 6 to 11)
Started 2026-11-27 (worktree `.gate/cs4b-2`, branch `cs/4b-content`; code head 914f3b4). Brief:
`docs-keeper-brief-4b2.md`. STATUS before: 120 lines.

## Plan
1. ROADMAP: 4b-2 entry at the top of Recent work (steps 6-11; step 12 to 4c by his STEP12-GO (a)); merge pointers
   for #75 (b9aad01) and #78 (b8f02b0) under their entries; a "Next / open" pointer; fold the oldest entries into
   Build history byte for byte (13 + 1 -> 12).
2. REPO_MAP: one line per new file/folder of 4b-2 (git diff --diff-filter=A b8f02b0..HEAD); rows made untrue by 4b-2
   fixed; repo-map-check.
3. docs/adr/README.md: pointer to his 13:57:21 UTC item 9 beside VISION-RULE v2's pointers (the paragraph line
   57-59; rows R2-INT-OQ1, R4-LOS-OQ1, R4-LOS-OQ2), with his two open points in his own words only.
4. STATUS by script: rows for merged #76 (MAIN-WALL), #75, #78 and line 5 (about merged #78/#76) moved to the archive
   byte for byte (OLD-LINES (a)); new 4b-2 row; Last merged; Next 2; Waiting; VISION-RULE pointer; "From change-set
   4b-2" ids; archive block "From change-set 4b-2" in Follow-ups. <= 120 lines.

## Done
- Slip (mine, reported): early read-only calls ran `cd <worktree>; git ...` (git after cd and `;`), against the brief's
  "git alone in its call"; all reads, none wrote; from then on git runs bare.
- 3 DONE: docs/adr/README.md lines 59-62 (one sentence after version 2's pointer, his open points quoted) and rows
  R2-INT-OQ1 (182), R4-LOS-OQ1 (200), R4-LOS-OQ2 (201): a pointer appended by `docs-keeper-4b2-adr.mjs`, no word of a
  row changed. Quote checked (whitespace-normalised includes) against DECISIONS' "Open: ..." line: true.
- 2 DONE: REPO_MAP by `docs-keeper-4b2-repomap.mjs` (25 rows added, 24 rows corrected or extended, 266 -> 292 lines):
  rows made untrue by 4b-2 fixed (Cargo.toml's dependency list and overrides, crates/ (sim, sim-wasm, devtools'
  protocol now a dev-dependency, content's dependents), rules_scan's `#![cfg(test)]`, time.rs's re-export,
  content/ (dev-dependency, facing, MIN_SPAWNS), sim-wasm lib.rs `new(seed, mode, pad_bytes, bundle)`,
  local_match, wasm.rs, recordings (.rec 122,497 bytes), dependency_rules, vectors, generated/, types index,
  sim-worker, vite.config, build.mjs, build-checks.mjs, app/, skeleton/, rules/). Each of the 43 added paths: an
  exact row or named in its folder's row (checked by script; the 8 generated .ts by type name, his Export and its
  manifest by description in `content/`'s row). repo-map-check: OK, 263 entries cover 110 required paths. The report's
  row is held back (no report file yet; its path unknown).
