#!/usr/bin/env bash
# The fast checks while a page step is being written: the type checks of the Worker, the page and the browser specs,
# as the full local checks run them (local-checks.sh beside this file), and the page's and the Worker's unit tests.
# No git; writes no file. From the worktree's root: bash .checkpoints/cs4c-soldier/ts-checks.sh
set -euo pipefail
tsc() { node node_modules/typescript/bin/tsc "$@"; }
eval "$(cat .gate/strict-line.txt)"
common="${strict% --lib es2023}"
worker=$(find packages/sim-worker/src -name "*.ts" ! -name "*.test.ts" | LC_ALL=C sort); worker_tests=$(find packages/sim-worker/src -name "*.test.ts" | LC_ALL=C sort)
page=$(find packages/client/src -name "*.ts" ! -name "*.test.ts" | LC_ALL=C sort); page_tests=$(find packages/client/src -name "*.test.ts" | LC_ALL=C sort)
specs=$(find packages/e2e/tests -name "*.ts" | LC_ALL=C sort)
echo "== the Worker"; tsc --ignoreConfig $common --lib es2023,webworker,esnext.disposable $worker packages/sim-worker/pkg/sim_wasm.d.ts
tsc --ignoreConfig $common --lib es2023,webworker,esnext.disposable --skipLibCheck $worker $worker_tests packages/sim-worker/pkg/sim_wasm.d.ts
echo "== the page"; tsc --ignoreConfig $common --lib es2023,dom,dom.iterable,esnext.disposable --types vite/client $page
tsc --ignoreConfig $common --lib es2023,dom,dom.iterable,esnext.disposable --types vite/client --skipLibCheck $page $page_tests
echo "== the browser specs"; tsc --ignoreConfig $common --lib es2023,dom,dom.iterable --skipLibCheck packages/e2e/node-names.d.ts packages/e2e/playwright.config.ts $specs
echo "== unit tests"; (cd packages/client && node ../../node_modules/vitest/vitest.mjs run)
(cd packages/sim-worker && node ../../node_modules/vitest/vitest.mjs run --dir src)
echo "TS CHECKS PASSED"
