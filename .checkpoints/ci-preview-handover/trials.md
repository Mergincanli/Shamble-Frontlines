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
- Firefox 161.0 (his FF-REFETCH (a), 14:00 Amsterdam: the file fetched again, 90,437,572 bytes, sha256 OK, unpacked
  with dpkg-deb -x; no graphics library files): the firefox project with the draft config, 3 passed in 5.3 s; the replay
  test "first-loop.rec: 1800 steps, every shared hash as in the file, in ... Firefox/161.0" (1.1 s), the 2 rule tests.
- Chromium (Chrome for Testing 158 here) with the draft config: 17 passed in 3.2 min (the replay test included).
- Checksums: `.github/workflows/toolchain.yml` today 01898e4b3833812e945882106baa2bb3590866aff3c8d2dfa487a1fe38ce5742,
  after the draft 067521c960c768ef29c9ed310151c53d649f2b5190fd6df120784c7837c03464; `packages/e2e/playwright.config.ts`
  today ef2136a14096fd12d5b9cce7b7fe28d611279850412a3269e5868139b19c9b0e, after the draft
  35abfd54fddb43d7bf8077b281c153c8625c22d863d04922afd0b7b1ecfe80c8.
