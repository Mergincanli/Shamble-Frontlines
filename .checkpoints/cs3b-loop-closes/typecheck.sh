#!/usr/bin/env bash
# The type checks of .github/workflows/toolchain.yml (its lines 164 to 170 and 180 to 191), with tsc started through
# node (the guard refuses npx here). Run from the repository's root, after a build (it reads pkg/sim_wasm.d.ts).
set -euo pipefail
tsc() { node node_modules/typescript/bin/tsc "$@"; }
tsc --version
strict="--noEmit --strict --noUncheckedIndexedAccess --exactOptionalPropertyTypes --noImplicitOverride --noFallthroughCasesInSwitch --noImplicitReturns --isolatedModules --verbatimModuleSyntax --erasableSyntaxOnly --allowImportingTsExtensions --module preserve --target es2023 --lib es2023"
shipped=$(find packages/types/src -name "*.ts" ! -name "*.test.ts" | LC_ALL=C sort)
tests=$(find packages/types/src -name "*.test.ts" | LC_ALL=C sort)
tsc --ignoreConfig $strict $shipped && echo "types: shipped ok"
tsc --ignoreConfig $strict --skipLibCheck $shipped $tests && echo "types: with tests ok"
common="${strict% --lib es2023}"
worker=$(find packages/sim-worker/src -name "*.ts" ! -name "*.test.ts" | LC_ALL=C sort)
worker_tests=$(find packages/sim-worker/src -name "*.test.ts" | LC_ALL=C sort)
page=$(find packages/client/src -name "*.ts" ! -name "*.test.ts" | LC_ALL=C sort)
page_tests=$(find packages/client/src -name "*.test.ts" | LC_ALL=C sort)
specs=$(find packages/e2e/tests -name "*.ts" | LC_ALL=C sort)
tsc --ignoreConfig $common --lib es2023,webworker,esnext.disposable $worker packages/sim-worker/pkg/sim_wasm.d.ts && echo "1 worker: shipped ok"
tsc --ignoreConfig $common --lib es2023,webworker,esnext.disposable --skipLibCheck $worker $worker_tests packages/sim-worker/pkg/sim_wasm.d.ts && echo "2 worker: with tests ok"
tsc --ignoreConfig $common --lib es2023,dom,dom.iterable,esnext.disposable --types vite/client $page && echo "3 page: shipped ok"
tsc --ignoreConfig $common --lib es2023,dom,dom.iterable,esnext.disposable --types vite/client --skipLibCheck $page $page_tests && echo "4 page: with tests ok"
tsc --ignoreConfig $common --lib es2023,dom,dom.iterable --skipLibCheck packages/e2e/node-names.d.ts packages/e2e/playwright.config.ts $specs && echo "5 specs ok"
