#!/usr/bin/env bash
# The toolchain job's own commands, run here by hand (the session may not run npx or npm test): the same order, each
# tool called through node. From the worktree's root: bash .checkpoints/cs4a-recording/local-checks.sh
# It stops at the first failure and prints which step failed; the log is .gate/local-checks.log.
set -euo pipefail
export SHAMBLE_WASM_BINDGEN="$PWD/.gate/wb-tool/wasm-bindgen"
tsc() { node node_modules/typescript/bin/tsc "$@"; }
vitest() { node node_modules/vitest/vitest.mjs "$@"; }
step() { echo "== $*"; }
eval "$(cat .gate/strict-line.txt)"
step "format and lints (not in CI's job: run by hand)"; cargo fmt --all -- --check; cargo clippy --workspace --all-targets --locked -- -D warnings
step "cargo test, twice (the second writes the generated types again)"; cargo test --workspace --locked; SHAMBLE_WRITE_TYPES=1 cargo test --workspace --locked
changed=$(git status --porcelain --untracked-files=all -- packages/types/src/generated); [ -z "$changed" ] || { echo "the generated types changed: $changed"; exit 1; }
step "release tests and the WebAssembly build of the five crates"; cargo test --release -p sim -p protocol --locked
cargo build --locked --target wasm32-unknown-unknown -p sim -p protocol -p room -p client-endpoint -p sim-wasm
step "each package's own tests"; for p in client sim-worker types net; do (cd packages/$p && if [ "$p" = sim-worker ]; then node ../../node_modules/vitest/vitest.mjs run --dir src; else node ../../node_modules/vitest/vitest.mjs run; fi); done
(cd packages/e2e && node ../../node_modules/@playwright/test/cli.js test --project=node)
step "type checks of the types package"
shipped=$(find packages/types/src -name "*.ts" ! -name "*.test.ts" | LC_ALL=C sort); tests=$(find packages/types/src -name "*.test.ts" | LC_ALL=C sort)
tsc --ignoreConfig $strict $shipped; tsc --ignoreConfig $strict --skipLibCheck $shipped $tests
step "the production page, then the development page"
node packages/client/scripts/build.mjs --production --out-dir "$PWD/.gate/dist-production"
sha256sum packages/sim-worker/pkg/sim_wasm_bg.wasm; stat -c '%s bytes (production)' packages/sim-worker/pkg/sim_wasm_bg.wasm
node packages/client/scripts/build.mjs --out-dir "$PWD/.gate/dist-development"
sha256sum packages/sim-worker/pkg/sim_wasm_bg.wasm; stat -c '%s bytes (development)' packages/sim-worker/pkg/sim_wasm_bg.wasm
step "the boundary tests"; vitest run --root packages/sim-worker --dir packages/sim-worker/boundary
step "type checks of the Worker, the page and the browser specs"
common="${strict% --lib es2023}"
worker=$(find packages/sim-worker/src -name "*.ts" ! -name "*.test.ts" | LC_ALL=C sort); worker_tests=$(find packages/sim-worker/src -name "*.test.ts" | LC_ALL=C sort)
page=$(find packages/client/src -name "*.ts" ! -name "*.test.ts" | LC_ALL=C sort); page_tests=$(find packages/client/src -name "*.test.ts" | LC_ALL=C sort)
specs=$(find packages/e2e/tests -name "*.ts" | LC_ALL=C sort)
tsc --ignoreConfig $common --lib es2023,webworker,esnext.disposable $worker packages/sim-worker/pkg/sim_wasm.d.ts
tsc --ignoreConfig $common --lib es2023,webworker,esnext.disposable --skipLibCheck $worker $worker_tests packages/sim-worker/pkg/sim_wasm.d.ts
tsc --ignoreConfig $common --lib es2023,dom,dom.iterable,esnext.disposable --types vite/client $page
tsc --ignoreConfig $common --lib es2023,dom,dom.iterable,esnext.disposable --types vite/client --skipLibCheck $page $page_tests
tsc --ignoreConfig $common --lib es2023,dom,dom.iterable --skipLibCheck packages/e2e/node-names.d.ts packages/e2e/playwright.config.ts $specs
step "the browser tests, timed (CI stops them after 5 minutes)"
start=$(date +%s)
SHAMBLE_DIST="$PWD/.gate/dist-development" SHAMBLE_CHROMIUM=/opt/pw-browsers/chromium node node_modules/@playwright/test/cli.js test -c packages/e2e/playwright.config.ts --project=chromium --workers=1 --retries=0 --trace off --reporter=list --global-timeout=300000
echo "the browser tests took $(( $(date +%s) - start )) seconds"
step "the recordings are what their scripts make"; cargo run -q -p devtools --locked --bin make_recordings -- --check
echo "ALL LOCAL CHECKS PASSED"
