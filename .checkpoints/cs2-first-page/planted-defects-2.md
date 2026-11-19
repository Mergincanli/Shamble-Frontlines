# planted defects, started 2026-11-19 08:03:59 UTC

## 1. the page calls into WebAssembly after it has started (PM-0002-2's own bite)
check: node node_modules/@playwright/test/cli.js test -c packages/e2e/playwright.config.ts --project=chromium --trace off skeleton.spec
exit 1: FAILED as it must, at the line that names the defect 
        > 59 |     expect(await page.evaluate(() => (window as unknown as { __wasmCalls: number }).__wasmCalls)).toBe(0);
        > 59 |     expect(await page.evaluate(() => (window as unknown as { __wasmCalls: number }).__wasmCalls)).toBe(0);

## 2. a block posted without its transfer list (PM-0002-6)
check: node node_modules/@playwright/test/cli.js test -c packages/e2e/playwright.config.ts --project=chromium --trace off skeleton.spec
exit 1: FAILED as it must, at the line that names the defect 
        > 47 |     expect(data.worker?.undetached).toBe(0);
        > 47 |     expect(data.worker?.undetached).toBe(0);

## 3. a copy that keeps its view of memory.buffer (PM-0002-6): the copy test alone
check: cd packages/sim-worker && node ../../node_modules/vitest/vitest.mjs run --dir boundary -t 'after the memory grows'
exit 1: FAILED as it must, at the line that names the defect 
       × after the memory grows between two steps, the next block is the block of a run without the growth 8ms
     FAIL  boundary/sim-wasm.test.mjs > after the memory grows between two steps, the next block is the block of a run without the growth

## 3b. a host that keeps its own view and copies around copyBlock (PM-0002-6): the host on the real Wasm
check: cd packages/sim-worker && node ../../node_modules/vitest/vitest.mjs run --dir boundary -t 'the host on the real Wasm'
exit 1: FAILED as it must, at the line that names the defect 
       × the host on the real Wasm: two calls per tick, one transferred block per tick, each passing the check 15ms
     FAIL  boundary/sim-wasm.test.mjs > the host on the real Wasm: two calls per tick, one transferred block per tick, each passing the check

## 4. a header field added in Rust without regenerating (PM-0002-16): the export test
check: cargo test -p client-endpoint --locked
exit 101: FAILED as it must, at the line that names the defect 
    test layout_ts::generated_layout_is_current ... FAILED

## 4b. the same field: the page's check refuses the block Rust now writes
check: cd packages/sim-worker && node ../../node_modules/vitest/vitest.mjs run --dir boundary
exit 1: FAILED as it must, at the line that names the defect 
    +   "reason": "layout-hash",

## 5. a tool one patch off: the recorded tool version is 0.2.129 while Cargo.lock holds 0.2.130 (PM-0002-18)
check: node packages/client/scripts/build.mjs --wasm-only
exit 1: FAILED as it must, at the line that names the defect 
    build: Cargo.lock records wasm-bindgen 0.2.130 and wasm-bindgen-tool.mjs the tool 0.2.129

## 5b. the tool's recorded sha256 changed by one digit: the file is not run
check: node packages/client/scripts/build.mjs --wasm-only
exit 1: FAILED as it must, at the line that names the defect 
    build: the wasm-bindgen tool at /home/user/Shamble-Frontlines/.gate/tools/wasm-bindgen-0.2.130/wasm-bindgen has sha256 ca4df773020dae87d92b9eadb2b85412bea7a9cf6318ab9999dd119fd9f46931, not the recorded one: it is not run

## 6. a misspelt field name in the writer's lookup
check: cargo build -p client-endpoint --locked
exit 101: FAILED as it must, at the line that names the defect 
