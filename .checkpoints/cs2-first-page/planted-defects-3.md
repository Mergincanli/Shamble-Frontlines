# planted defects, started 2026-11-19 11:00:59 UTC
the tree at the start: HEAD 9f28a87cc7deb69caadf959aeb954d0f0d5984f9, with 210 lines of uncommitted diff in crates/ and packages/

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
       × after the memory grows between two steps, the next block is the block of a run without the growth 9ms
     FAIL  boundary/sim-wasm.test.mjs > after the memory grows between two steps, the next block is the block of a run without the growth

## 3b. a host that keeps its own view and copies around copyBlock (PM-0002-6): the host on the real Wasm
check: cd packages/sim-worker && node ../../node_modules/vitest/vitest.mjs run --dir boundary -t 'the host on the real Wasm'
exit 1: FAILED as it must, at the line that names the defect 
       × the host on the real Wasm: two calls per tick, one transferred block per tick, each passing the check 13ms
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
    error[E0080]: evaluation panicked: the header has a tick field
    9 | const TICK: Field = layout::field("header", "tik").expect("the header has a tick field");

## 7. a 64-byte typed array allocated per frame in the page's hand-off code (PM-0002-7's bite)
check: node node_modules/@playwright/test/cli.js test -c packages/e2e/playwright.config.ts --project=chromium --trace off allocation.spec
exit 1: FAILED as it must, at the line that names the defect 
        > 62 |   expect(perFrame).toBeLessThan(170);

## 7b. a 64-byte typed array allocated per block where the message event is unpacked
check: node node_modules/@playwright/test/cli.js test -c packages/e2e/playwright.config.ts --project=chromium --trace off allocation.spec
exit 1: FAILED as it must, at the line that names the defect 
        > 64 |   expect(perBlock).toBeLessThan(610);

## 8. a Worker that ignores the hidden tab and runs on (PM-0004-70's bite)
check: node node_modules/@playwright/test/cli.js test -c packages/e2e/playwright.config.ts --project=chromium --trace off skeleton.spec
exit 1: FAILED as it must, at the line that names the defect 
        > 83 |   expect(stillHidden.worker?.ticks).toBe(hidden);

## 8b. a Worker that runs the hidden gap on the tab's return instead of dropping it (the 7-tick cap kept)
check: node node_modules/@playwright/test/cli.js test -c packages/e2e/playwright.config.ts --project=chromium --trace off skeleton.spec
exit 1: FAILED as it must, at the line that names the defect 
        > 89 |   expect(after.worker?.drops).toBe(0);

## 9. a small server that lets a path out of its folder
check: cd packages/client && node ../../node_modules/vitest/vitest.mjs run
exit 1: FAILED as it must, at the line that names the defect 
         × a path that leaves the folder, however it is written 16ms
         × a link inside the folder that leads outside it 9ms
     FAIL  serve.test.mjs > what it refuses > a path that leaves the folder, however it is written
     FAIL  serve.test.mjs > what it refuses > a link inside the folder that leads outside it

## 9b. a small server that leaves the file of a broken-off download open
check: cd packages/client && node ../../node_modules/vitest/vitest.mjs run
exit 1: FAILED as it must, at the line that names the defect 
         × a download that is broken off leaves no file open 259ms
     FAIL  serve.test.mjs > what it refuses > a download that is broken off leaves no file open

## 10. an isolation flag that sets COOP but not COEP (PM-0002-20's own bite)
check: node node_modules/@playwright/test/cli.js test -c packages/e2e/playwright.config.ts --project=chromium --trace off isolation.spec
exit 1: FAILED as it must, at the line that names the defect 
      ✘  1 [chromium] › packages/e2e/tests/skeleton/isolation.spec.ts:15:5 › with the flag on under Vite's dev server, crossOriginIsolated is true on the page and in the Worker (3.9s)
      ✘  3 [chromium] › packages/e2e/tests/skeleton/isolation.spec.ts:15:5 › with the flag on under Vite's preview server, crossOriginIsolated is true on the page and in the Worker (3.1s)

## 11. the page posts its input on every other frame only
check: node node_modules/@playwright/test/cli.js test -c packages/e2e/playwright.config.ts --project=chromium --trace off skeleton.spec
exit 1: FAILED as it must, at the line that names the defect 
        > 56 |     expect(frames - posts).toBeLessThanOrEqual(1);
        > 56 |     expect(frames - posts).toBeLessThanOrEqual(1);

## 12. the build's switch is not folded in: development code in a production build
check: node packages/client/scripts/build.mjs --production --out-dir .gate/dist-prod
exit 1: FAILED as it must, at the line that names the defect 
      assets/overlay--eYwvZFu.js: development code in a production build ("Copy report")
      assets/index-lFfRQCof.js: development code in a production build ("__shambleReport")
      assets/report-D6SeR0nu.js: development code in a production build ("ticks while hidden")
      assets/report-D6SeR0nu.js: development code in a production build ("first page: report")

## 14. a Worker whose script cannot be loaded: its error event has no message, and the page passes that on
check: node node_modules/@playwright/test/cli.js test -c packages/e2e/playwright.config.ts --project=chromium --trace off skeleton.spec
exit 1: FAILED as it must, at the line that names the defect 
        > 105 |     await expect(page.locator("#notice")).toHaveText(new RegExp(`^The match stopped: .*${said}`));

## 14b. a Worker's error is not cancelled, so the window counts it a second time
check: node node_modules/@playwright/test/cli.js test -c packages/e2e/playwright.config.ts --project=chromium --trace off skeleton.spec
exit 1: FAILED as it must, at the line that names the defect 
        > 107 |     expect((await report(page)).errors).toBe(1);

## 15. the page does something its content security policy refuses (an inline style)
check: node node_modules/@playwright/test/cli.js test -c packages/e2e/playwright.config.ts --project=chromium --trace off skeleton.spec
exit 1: FAILED as it must, at the line that names the defect 
        > 57 |     expect(later.errors).toBe(0);
        > 57 |     expect(later.errors).toBe(0);

## 13. the page imports the Wasm's glue itself
check: node packages/client/scripts/build.mjs
exit 1: FAILED as it must, at the line that names the defect 
