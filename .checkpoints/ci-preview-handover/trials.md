# Trials so far (2026-11-26, worktree `.gate/ci4`, main 2483e9a)
- The production page built here with the approved wasm-bindgen (both sha256 checked): 5 files, none dot-named
  (`find .gate/dist-production -name '.*'` prints only nothing below the folder), 892,209 bytes.
- The drafted guard step (`draft-toolchain.diff`), extracted and run by `plant-test.sh` (one mktemp folder per case):
  the real build exit 0 ("5 files, 892209 bytes"); a symbolic link, a fifo, an empty build, the build folder as a link
  and `.gate` as a link each exit 1 with their own line. The programs it names (`/usr/bin/bash`, `find`, `wc`, `du`,
  `cut`) are root-owned here; on the runner, inferred.
- The draft YAML parses (11 steps); the guard's script passes `bash -n`.
- The upload step cannot run outside GitHub; its inputs read from the action's own text at cf430e0 (plan R2-6).
- The draft config (`draft-playwright-config.diff`): lists Chromium 17 (the replay test joined it with #70), Firefox 3
  (the replay test and the 2 rule tests), node 1; the e2e type check passes.
- Waiting: the Firefox run (his FF-REFETCH).
