# R1 unit boundary-worker: the Rust ↔ TypeScript boundary and the simulation in a Web Worker

Brief question (quoted, `docs/V2_PHASE0_BRIEF.md` §4, R1 row): "the Rust ↔ TypeScript boundary (shared memory,
wasm-bindgen, generated TypeScript types); running the simulation in a Web Worker".

Unit scope (from the task): how inputs go in and snapshots and events come out with no call per entity per frame;
wasm-bindgen and type generation; SharedArrayBuffer and Atomics against postMessage with transferables; reading state
straight out of Wasm memory; Wasm threads; cross-origin isolation (COOP and COEP) and what it costs; the worker on the
minimum hardware; browser support in Chrome, Edge, Firefox and Safari; offline play through the same simulation in
the worker. Author: engine-architect, 2026-10-27. Status: draft (unverified). All pass marks and the reference hardware
below are **proposed**.

## 2. Findings

Tiers: every source in this unit is official (vendor docs, specifications, the Rust project, the spec repos and the
browser-compat data that MDN publishes). No shipped-game source was found (see section 7, "Sources not read"). "Confirmed in the source"
means the claim rests on the sentence quoted; "inferred" means it rests on a summary or on reasoning across sources.

**Tooling: wasm-bindgen and type generation**

- **R1-BND-F1.** The Rust and WebAssembly working group's GitHub org was wound down: wasm-bindgen moved to a new
  `wasm-bindgen` org with new maintainers, and every other rustwasm repository (wasm-pack, gloo, twiggy, walrus and
  the rest) was archived in place. Quotes: "All other repositories in the rustwasm organization are going to be
  archived in place." · "a critical mass of new maintainers has already been reached" · "Transferring wasm-bindgen
  will occur in the weeks following this post." · Source: https://blog.rust-lang.org/inside-rust/2025/07/21/sunsetting-the-rustwasm-github-org/
  (Inside Rust blog, 2025-07-21) · official · read 2026-10-27 · confirmed in the source.
- **R1-BND-F2.** wasm-bindgen ships often and with breaking changes in 2026; since 0.2.128 the CLI and the library
  must be the same version. Quote (0.2.128, "05 Sep"): "Requires matching `wasm-bindgen` and CLI versions (schema
  bump)." The cadence (0.2.118 on 10 Apr to 0.2.129 on 25 Sep, breaking changes in 0.2.122, 0.2.123 and 0.2.128) is
  from the release list's summary. · Sources: https://github.com/wasm-bindgen/wasm-bindgen/releases/tag/0.2.128 and
  https://github.com/wasm-bindgen/wasm-bindgen/releases · official · read 2026-10-27 · version-match rule confirmed
  in the source; cadence and the year 2026 inferred (the page shows no year; asset timestamps do).
- **R1-BND-F3.** wasm-bindgen writes a TypeScript declaration file by default. Quote: "Output a TypeScript
  declaration file for the generated JavaScript bindings. This is on by default." · Source:
  https://raw.githubusercontent.com/wasm-bindgen/wasm-bindgen/main/guide/src/reference/cli.md (guide source, main
  branch) · official · read 2026-10-27 · confirmed in the source.
- **R1-BND-F4.** `typescript_custom_section` appends text to that generated `.d.ts` and is meant for code generation.
  Quotes: "it will append the contents of the string to the `.d.ts` file exported by `wasm-bindgen-cli`" · "The
  primary target for this feature is for code generation." · Source:
  https://raw.githubusercontent.com/wasm-bindgen/wasm-bindgen/main/guide/src/reference/attributes/on-rust-exports/typescript_custom_section.md
  · official · read 2026-10-27 · confirmed in the source.
- **R1-BND-F5.** Passing structured data through serde costs many boundary calls. Quotes: serde-wasm-bindgen "works
  by directly manipulating JavaScript values" and "requires a lot of calls back and forth between Rust and
  JavaScript, which can sometimes be slow"; the JSON route "can be anywhere from 2x to 0.2x the speed of
  `serde-wasm-bindgen`". · Source:
  https://raw.githubusercontent.com/wasm-bindgen/wasm-bindgen/main/guide/src/reference/arbitrary-data-with-serde.md
  · official · read 2026-10-27 · confirmed in the source.
- **R1-BND-F6.** Slices passed through wasm-bindgen are copied, not shared: a `&mut [T]` parameter "only accepts a
  `TypedArray`, since the results are copied back into its buffer"; plain JS arrays are "copied into the Wasm memory
  the same way as a `TypedArray`". · Source:
  https://raw.githubusercontent.com/wasm-bindgen/wasm-bindgen/main/guide/src/reference/types/number-slices.md ·
  official · read 2026-10-27 · confirmed in the source for `&mut` and plain arrays; for `&[T]` from a TypedArray,
  inferred.
- **R1-BND-F7.** The Rust and WebAssembly book's interface rule: keep long-lived data inside Wasm linear memory and
  let JS read it through a typed-array view, so nothing is copied per tick. Quotes: "Minimizing copying into and out
  of the WebAssembly linear memory. Unnecessary copies impose unnecessary overhead." · "large, long-lived data
  structures are implemented as Rust types that live in the WebAssembly linear memory" · "we get a pointer to the
  universe's cells, construct a `Uint8Array` overlaying the cells buffer" · "By working with pointers and overlays,
  we avoid copying the cells across the boundary on every tick." · Source:
  https://rustwasm.github.io/docs/book/game-of-life/implementing.html (the page says the project is "no longer
  maintained") · official (archived) · read 2026-10-27 · confirmed in the source.
- **R1-BND-F8.** tsify generates TypeScript definitions for Rust types used with wasm-bindgen. Quote: "Tsify is a
  library for generating TypeScript definitions from rust code." Version 0.5.8, repository madonoharu/tsify. Its
  values cross through serde-wasm-bindgen (the `js` feature) and its `into_wasm_abi`/`from_wasm_abi` attributes are
  deprecated in favour of a `Ts<T>` wrapper (summary). A second crate, tsify-next 0.5.6 (repository siefkenj/tsify,
  page date 2026-10-05), exists; neither page says which line is maintained. · Sources:
  https://docs.rs/tsify/latest/tsify/ and https://docs.rs/tsify-next/latest/tsify_next/ · official (crate docs) ·
  read 2026-10-27 · description confirmed in the source; the serde route, the deprecation and tsify-next's status
  inferred.
- **R1-BND-F9.** ts-rs generates TypeScript declarations from Rust types at test time, with no runtime part. Quotes:
  "Generate typescript type declarations from rust types." · bindings are written "when running `cargo test` or
  `cargo test export_bindings`". Version 12.0.1 (MIT, repository Aleph-Alpha/ts-rs); output to `./bindings` or
  `TS_RS_EXPORT_DIR`; the default `serde-compat` feature means "serde attributes are parsed for enums and structs".
  MSRV 1.88.0 (summary). · Source: https://docs.rs/ts-rs/latest/ts_rs/ · official (crate docs) · read 2026-10-27 ·
  confirmed in the source (MSRV inferred).
- **R1-BND-F10.** Only `#[repr(C)]` (or another explicit representation) gives a Rust struct a layout another
  language can read by offset. Quotes: default representation: "The only data layout guarantees made by this
  representation are those required for soundness" and "The ordering does not have to be the same as the order in
  which the fields are specified"; C representation: "The second purpose is to create types that you can soundly
  perform operations on that rely on data layout"; layout rule: "For each field in declaration order in the struct
  ... add padding bytes to the current offset until it is a multiple of the field's alignment." · Source:
  https://doc.rust-lang.org/reference/type-layout.html (Rust Reference, current) · official · read 2026-10-27 ·
  confirmed in the source.
- **R1-BND-F11.** A typed array built on a buffer is a view, not a copy, and its offset must be a multiple of its
  element size. Quotes: "When called with an `ArrayBuffer` or `SharedArrayBuffer` instance, and optionally a
  `byteOffset` and a `length` argument, a new typed array view is created that views the specified buffer." ·
  RangeError when "`byteOffset` is not an integral multiple of the new typed array's element size." · Source:
  https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray (last modified
  2026-10-26) · official · read 2026-10-27 · confirmed in the source.

**Shared memory, Atomics, transfer, Wasm memory**

- **R1-BND-F12.** Sharing memory between the page and a worker requires a secure, cross-origin-isolated page; without
  it `postMessage` throws for a SharedArrayBuffer. Quotes: "To use shared memory your document must be in a secure
  context and cross-origin isolated." · "Unless the two headers mentioned above are set, the various `postMessage()`
  APIs will throw for `SharedArrayBuffer` objects. If they are set, `postMessage()` on `Window` objects and dedicated
  workers will function and allow for memory sharing." · Source:
  https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer (last
  modified 2026-02-10) · official · read 2026-10-27 · confirmed in the source.
- **R1-BND-F13.** Writes to shared memory reach the other thread only "eventually"; ordering needs Atomics. Quotes: "a
  side effect to the block in one agent will eventually become visible in the other agent" · "To synchronize, atomic
  operations are needed." (SharedArrayBuffer page) · "Atomic operations make sure that predictable values are written
  and read, that operations are finished before the next operation starts and that operations are not interrupted."
  · Sources: the SharedArrayBuffer page above and
  https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Atomics (last modified
  2026-02-10) · official · read 2026-10-27 · confirmed in the source.
- **R1-BND-F14.** The page cannot block on shared memory; it can only wait asynchronously. Quotes: `Atomics.wait`:
  "(Most browsers will not allow `wait()` on the browser's main thread.)" · `Atomics.waitAsync` "is non-blocking and,
  unlike `Atomics.wait()`, can be used on the main thread." · Sources: the Atomics page above and
  https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Atomics/waitAsync (last modified
  2026-02-10) · official · read 2026-10-27 · confirmed in the source.
- **R1-BND-F15.** Transferring an ArrayBuffer moves it without copying, and the sender loses it; without a transfer
  list the data is copied. Quotes: "the memory resource that it points to is _literally_ moved between contexts in a
  fast and efficient zero-copy operation" · "the original object is no longer usable ... and any attempt to read or
  write the object will throw an exception." Typed arrays are not transferable, their buffers are; the list includes
  `WebTransportReceiveStream` and `WebTransportSendStream` (summary). · Source:
  https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API/Transferable_objects (last modified 2025-09-18) ·
  official · read 2026-10-27 · zero-copy move and detachment confirmed in the source; the copy-by-default and the
  list inferred.
- **R1-BND-F16.** Growing a non-shared Wasm memory detaches every existing view of it; for shared memory the old
  buffer keeps its old length. Quotes: "Every call to `grow` will detach any references to the old `buffer`, even for
  `grow(0)`!" · "For a shared `Memory` instance, the initial `buffer` ... will not become detached, but rather its
  length will not be updated." · Source:
  https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Memory/grow (last modified
  2026-05-14) · official · read 2026-10-27 · confirmed in the source.
- **R1-BND-F17.** A Wasm memory is a SharedArrayBuffer only when created shared, a module must declare a shared memory
  to import one, and a shared memory must have a fixed maximum. Quotes: "When this flag is set to `true`, the
  constructed `Memory` object can be shared between workers via `postMessage()`, just like `SharedArrayBuffer`, and
  the backing `buffer` of the `Memory` object is a `SharedArrayBuffer`." (SharedArrayBuffer page) · "it is a
  validation error to import non-shared memory if the module's memory import specifies it as being shared." · "Note
  that shared linear memory without an explicit maximum size is not permitted." · Sources: the SharedArrayBuffer page
  above and https://raw.githubusercontent.com/WebAssembly/threads/main/proposals/threads/Overview.md (threads
  proposal overview, main) · official · read 2026-10-27 · confirmed in the source; the reverse rule (a module that
  declares unshared memory cannot import a shared one) is inferred from a truncated quote.
- **R1-BND-F18.** Building Rust for Wasm threads needs nightly Rust, the atomics target feature and a rebuilt
  standard library, works only with the `web` or `no-modules` output, and keeps `std::thread` unusable. Quotes: "Note
  that this requires a nightly Rust toolchain." · `RUSTFLAGS='-C target-feature=+atomics'` with `-Z
  build-std=panic_abort,std` · "Currently it's required to use the `--target no-modules` or `--target web` flag" ·
  "The main thread in a browser cannot block." · "the standard library has no viable route to implement the
  `std::thread` module" · "This demo should work in the latest Firefox and Chrome versions at this time." · Source:
  https://raw.githubusercontent.com/wasm-bindgen/wasm-bindgen/main/guide/src/examples/raytrace.md (guide source,
  main, undated) · official · read 2026-10-27 · confirmed in the source. That Rust emits a shared memory only in this
  atomics build is inferred (the page says threads need it; it does not say so of shared memory in those words).
- **R1-BND-F19.** On the default Rust Wasm target, `std::thread::spawn` panics, and the default Wasm features
  include bulk-memory and nontrapping-fptoint from Rust 1.87.0. Quote: "`std::thread::spawn` will panic. There is no
  means by which this can be overridden." Default features listed: multivalue, mutable-globals, reference-types,
  sign-ext, nontrapping-fptoint (Rust 1.87.0+, LLVM 20+), bulk-memory (Rust 1.87.0+, LLVM 20+). Target tier 2. ·
  Source: https://doc.rust-lang.org/rustc/platform-support/wasm32-unknown-unknown.html (rustc book, current) ·
  official · read 2026-10-27 · confirmed in the source.

**Cross-origin isolation and its costs**

- **R1-BND-F20.** Cross-origin isolation is COOP `same-origin` plus COEP `require-corp` or `credentialless`. Quote:
  "To use these features in a document, you will need to set the COOP header to `same-origin` and the
  `Cross-Origin-Embedder-Policy` header to `require-corp` (or `credentialless`)." It also unlocks finer timers and a
  memory API: isolated documents get "`Performance.now()` offers better precision" and
  "`Performance.measureUserAgentSpecificMemory()` can be called". · Sources:
  https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Cross-Origin-Opener-Policy (last modified
  2026-07-12) and https://developer.mozilla.org/en-US/docs/Web/API/Window/crossOriginIsolated (last modified
  2026-08-21) · official · read 2026-10-27 · confirmed in the source.
- **R1-BND-F21.** Under `require-corp`, every cross-origin `no-cors` resource needs a CORP header, and frames are
  covered too. Quotes: "A document can only load resources requested in `no-cors` mode from the same origin, or
  resources that have explicitly set the `Cross-Origin-Resource-Policy` header to a value that allows it to be
  embedded." · `credentialless`: requests "are sent without credentials: cookies are omitted in the request, and
  ignored in the response." · "The entire chain of the documents, including parent frames and child frames, must be
  cross-origin isolated, too." · Sources:
  https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Cross-Origin-Embedder-Policy (last modified
  2026-09-04) and https://web.dev/articles/coop-coep (last updated 2022-06-21) · official · read 2026-10-27 ·
  confirmed in the source.
- **R1-BND-F22.** Safari has no `credentialless`, so an isolated page on Safari must use `require-corp`. Browser
  compatibility data: COEP header Chrome "83", Edge "mirror", Firefox "79", Safari "15.2"; `credentialless` Chrome
  "96", Edge "mirror", Firefox "119", Safari `false`. web.dev's line that credentialless "is not yet supported by other
  browsers" than Chrome and Edge is out of date. · Source:
  https://raw.githubusercontent.com/mdn/browser-compat-data/main/http/headers/Cross-Origin-Embedder-Policy.json (main
  branch) · official · read 2026-10-27 · confirmed in the source (data values).
- **R1-BND-F23.** COOP `same-origin` cuts the link to popups that do not opt in, which is how OAuth and payment
  popups usually work; the value meant for those popups does not give isolation. Quotes: "A document with
  `same-origin` will always open a document with `unsafe-none` in a new BCG." · "When opened in a new BCG, any
  references between the new document and its opener are severed." · `same-origin-allow-popups` is for "when using a
  cross-origin service for OAuth or payments". · Source: the COOP page above · official · read 2026-10-27 ·
  confirmed in the source; the consequence for a later login (VISION_V2 §5) inferred.
- **R1-BND-F24.** Rollout is testable before it is enforced, and the headers must cover worker scripts as well.
  Quote: with `Cross-Origin-Embedder-Policy-Report-Only` you "will receive reports without blocking embedded
  content." That COEP must also be sent on worker scripts is from the page's summary. · Source:
  https://web.dev/articles/coop-coep (published 2020-04-13, last updated 2022-06-21) · official, dated · read
  2026-10-27 · report-only confirmed in the source; worker scripts inferred.

**Browser support (desktop Chrome, Edge, Firefox, Safari)**

- **R1-BND-F25.** Module workers work in all four target browsers: `Worker` `options.type` "module": Chrome "80",
  Edge "mirror", Firefox "114", Safari "15". The wasm-bindgen guide's line that "only Chrome supports modules in web
  workers" is out of date. · Sources: https://raw.githubusercontent.com/mdn/browser-compat-data/main/api/Worker.json
  (main) and https://raw.githubusercontent.com/wasm-bindgen/wasm-bindgen/main/guide/src/examples/wasm-in-web-worker.md
  · official · read 2026-10-27 · confirmed in the source (data values).
- **R1-BND-F26.** SharedArrayBuffer and Atomics are in all four: SharedArrayBuffer Chrome "68", Edge "mirror",
  Firefox "79", Safari "15.2"; growable SharedArrayBuffer Chrome "111", Firefox "128", Safari "16.4"; `Atomics.wait`
  and `notify` Chrome "68", Firefox "78", Safari "15.2"; `Atomics.waitAsync` Chrome "90", Firefox "145", Safari
  "16.4". MDN marks waitAsync "Baseline 2025" ("since November 2025"). · Sources:
  https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/builtins/SharedArrayBuffer.json,
  https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/builtins/Atomics.json, the waitAsync page
  above · official · read 2026-10-27 · confirmed in the source (data values).
- **R1-BND-F27.** The Wasm features a default Rust build uses, and Wasm threads, are in all three engines: threads
  Chrome "74", Firefox "79", Safari "14.1"; bulk memory "75"/"79"/"15"; reference types "96"/"79"/"15"; non-trapping
  float-to-int "75"/"64"/"15"; sign extension "74"/"62"/"14.1"; multi-value "85"/"78"/"13.1". Edge is not listed
  (it is Chromium). · Source: https://raw.githubusercontent.com/WebAssembly/website/main/features.json (the
  webassembly.org feature table, main) · official · read 2026-10-27 · confirmed in the source (data values); "Edge
  follows Chrome" inferred.
- **R1-BND-F28.** wasm-bindgen supports all four browsers. Quote: "Firefox, Chrome, Safari, and Edge browsers are
  all supported by `wasm-bindgen`." · Source:
  https://raw.githubusercontent.com/wasm-bindgen/wasm-bindgen/main/guide/src/reference/browser-support.md ·
  official · read 2026-10-27 · confirmed in the source.

**Networking from a worker, timing and the hardware**

- **R1-BND-F29.** Both transports the netcode plans can be opened inside a worker. WebTransport: "Note: This feature
  is available in Web Workers." and "Since March 2026, this feature works across the latest devices and browser
  versions." WebSocket: "This feature is available in Web Workers." · Sources:
  https://developer.mozilla.org/en-US/docs/Web/API/WebTransport (last modified 2026-10-12) and
  https://developer.mozilla.org/en-US/docs/Web/API/WebSocket (last modified 2024-09-25) · official · read 2026-10-27
  · confirmed in the source.
- **R1-BND-F30.** Timer resolution is coarser without isolation, and a worker has its own time origin. Quotes:
  "Resolution in isolated contexts: 5 microseconds" · "Resolution in non-isolated contexts: 100 microseconds" ·
  `performance.now()` is available in workers, and its origin there is "the time when the worker is run". · Source:
  https://developer.mozilla.org/en-US/docs/Web/API/Performance/now (last modified 2025-08-25) · official · read
  2026-10-27 · confirmed in the source.
- **R1-BND-F31.** Chrome throttles timers in hidden pages, to once per minute after five minutes; the article does not
  say whether workers are affected. Quotes: "The page has been hidden for more than 5 minutes." · "The chain count is
  5 or greater." · "The page has been silent for at least 30 seconds." · "the browser will check timers in this group
  once per minute". · Source: https://developer.chrome.com/blog/timer-throttling-in-chrome-88 (last updated
  2021-01-18; Chrome 88) · official · read 2026-10-27 · conditions confirmed in the source; the article's silence on
  workers inferred from its summary.
- **R1-BND-F32.** The core count a page sees is not a hardware fact. Quotes: `hardwareConcurrency` "returns the number
  of logical processors available to run threads on the user's computer" · "The browser may, however, choose to
  report a lower number of logical cores in order to represent more accurately the number of `Worker`s that can run
  at once, so don't treat this as an absolute measurement". · Source:
  https://developer.mozilla.org/en-US/docs/Web/API/Navigator/hardwareConcurrency (last modified 2024-10-16) ·
  official · read 2026-10-27 · confirmed in the source.
- **R1-BND-F33.** Vite (docs v8.3.4) bundles module workers and Wasm and can set dev-server headers. Quotes: "A web
  worker script can be imported using `new Worker()` and `new SharedWorker()`." · "The worker detection will only work
  if the `new URL()` constructor is used directly inside the `new Worker()` declaration." · Wasm: "When you need
  control over when and how the module is instantiated, import it with `?init`." · `server.headers`: "Specify server
  response headers." · Sources: https://vite.dev/guide/features and https://vite.dev/config/server-options ·
  official · read 2026-10-27 · confirmed in the source.

## 3. Legacy facts used

- **CS1** (fixed tick and input-command groundwork, mark: drop). `FIXED_DT` 1/60 s, the `BTN` bitmask (LEGACY_TUNING
  "Simulation: tick, input bits, hit bodies": FIRE 1, ADS 2, RELOAD 4, MELEE 8, GRENADE 16, SHIELD 32, SPRINT 64) and
  `makeInputCommand(seq, tick)` were declared and never wired. For v2: the idea of one input command per tick (seq,
  tick, move, aim, buttons) crosses the boundary as one small record; its exact format is R2's (ADR-0001). The lesson
  is to wire it from the first spike, not to declare it.
- **GP2** (input mapping and listeners, mark: port to TypeScript). Legacy runs reload, ability, grenade, bow release,
  E, weapon switch and ADS at event time, outside `update()`. For v2: listeners stay on the page and only emit the
  command; one-shot actions become press and release edges carried so a press is neither lost nor doubled (the row's
  own v2 note).
- **GP13** (frame loop, mark: redesign). One rAF loop runs a variable-dt `update(dt)` (dt clamped to 0.05 s) and
  freezes the world for pause, shop, tutorial and hit-stop. For v2: the fixed-tick sim runs apart from the render
  loop, which interpolates; nothing on the page can pause the sim.
- **GP14** (the step order, mark: rewrite in Rust). HUD writes are interleaved through the step; `update()` costs
  0.49 ms per frame (AUDIT_BUGS_PERF §1). For v2: the sim touches no DOM, and the HUD reads snapshots and events on the
  page.
- **GP10** (HUD writers, mark: port to TypeScript, look brief pending). About 30 DOM writes per frame (AUDIT P3) fell
  to "0.01 real writes per frame" with change-only setters (CS3). For v2: the HUD keeps change-only writes and is fed
  from snapshot fields and events, never from calls into the sim.
- **GC2** (soldier slots, mark: rewrite in Rust). 12 fixed slots, slot id equal to sim id; perfbots at 48 soldiers:
  combat 0.80–1.00 ms per update step (legacy JavaScript, not Rust). For v2: a stable slot id per entity is what lets
  the page index a flat snapshot with no lookup; the legacy step cost gives a scale for what a worker would take off
  the main thread, not a v2 figure.
- **GA15** (per-frame animation driver). Every soldier fills one state struct per frame (aim yaw, reload progress,
  throw charge, ADS, grounded and more), while aim pitch travels in a side channel through the gun's rotation. For
  v2: the fields presentation needs per soldier must be named fields of the snapshot layout (aim pitch included, for
  VISION_V2 §6.2 "aiming at height"); purely cosmetic state (kick, breathing clock) stays on the client.
- **CS11** (headless SIM fence, mark: redesign). The comment fence leaked: shape data came from the visual recipe and
  `hasLOS` read a global. For v2: the crate boundary is enforced by the compiler, and the boundary module only
  reads what the sim publishes.

## 4. Options

### Q1. Where the client simulation runs

| Option | What it is | Complexity and effort | Performance and risk | Fit with VISION_V2 |
|---|---|---|---|---|
| A1 Main thread | Wasm sim instantiated on the page, stepped inside the render loop | Lowest: no worker, direct calls | The tick competes with rendering and DOM; a long tick (prediction replay, many bots) drops frames; legacy is this shape (GP13, GP14) | Weak for §6.9 spectacle budget and §4's 30+ fps minimum |
| A2 One dedicated module worker for the sim | Worker owns the Wasm module; page keeps input, HUD and rendering | One boundary; module workers in all four browsers (F25); Vite bundles it (F33) | Tick time leaves the render thread; costs one thread and a per-tick hand-off | Serves §6.2, §6.9, §4 |
| A3 A2 plus rendering in another worker | The renderer also leaves the page (OffscreenCanvas) | Highest; R3's question | Page keeps only DOM; not assessed here | R3 decides; not needed to answer this unit |

### Q2. How snapshots and events leave the sim

| Option | Copies per tick | Isolation needed | Toolchain | Main risks |
|---|---|---|---|---|
| B1 Structured clone of JS objects | Every field, plus per-entity object building in the worker (F5's per-field cost) | No | Stable | Allocation and per-entity work every tick: breaks the "no call per entity" rule |
| B2 One flat buffer, transferred from a pool | One block copy in the worker from linear memory into a pooled ArrayBuffer; the transfer itself is zero-copy (F15) | No | Stable Rust, no Wasm threads | Delivery waits for the page's message queue (inferred, to be measured); the pool must recycle buffers (the sender loses each one, F15) |
| B3 One flat buffer in a SharedArrayBuffer ring | One block copy into a ring slot; the page reads the newest complete slot with Atomics (F13) | Yes (F12, F20); Safari only with `require-corp` (F22) | Stable Rust | Torn reads unless slots are published with Atomics; events need their own lossless ring; isolation costs (F21, F23) |
| B4 Page reads the sim's own shared Wasm memory | None | Yes | Nightly Rust, atomics, rebuilt std, `web`/`no-modules` output only (F18); fixed maximum memory (F17) | Most brittle (F18's caveats); views go stale on growth (F16); the sim must double-buffer its output to avoid torn reads |

Inside the buffer, two layouts are possible: records per entity (`#[repr(C)]` structs, F10) or sections per field
(all positions in one `f32` run, and so on). Either way each section starts at an offset aligned to its element size
(F11), and a section header says how many live entries each kind has.

### Q3. How inputs go in
- **C1 A small message per tick** carrying the latest command (seq, tick, move, aim, held buttons, edge counters).
  Works without isolation; one message per tick.
- **C2 An input slot in a SharedArrayBuffer**, written by the page and read by the worker with Atomics. Needs
  isolation; saves a message.
Either way, one-shot actions travel as edge counters (a count of presses and releases), so a duplicated or late
command neither loses nor doubles a press (GP2).

### Q4. Generated TypeScript types

| Option | Covers | Runtime cost | Maintenance | Fit |
|---|---|---|---|---|
| D1 wasm-bindgen's own `.d.ts` | The exported control functions and classes (create, load content, step, pointer and length of the output block) | Calls only, a constant number per tick | Default output (F3); CLI must match the library (F2) | Needed whatever else is chosen |
| D2 tsify | Serde types as TS types, values passed by serde | Per-field calls (F5) | Two forks, status unclear (F8) | Fine for rare messages, wrong for per-tick state |
| D3 ts-rs | Any Rust type as a TS declaration, written at `cargo test` | None (types only) | One crate, 12.0.1 (F9) | Message, protocol and content-schema types shared by sim, server, client and tools |
| D4 Own layout generator | The flat snapshot and event layout: offsets, strides, element types and a layout hash, emitted as a TS module or through `typescript_custom_section` (F4) | None (types and constants) | Ours to keep; small | The only option that covers the bulk buffer |

### Q5. Wasm threads in the client sim
- **E1 None:** one single-threaded sim in one worker, stable Rust, the default target (F19).
- **E2 Threads** (a thread pool across workers): needs nightly, atomics, a rebuilt std and isolation (F18, F12);
  `std::thread` stays unusable (F18, F19). No VISION_V2 §6 item asks the client to split one tick across cores.

### Q6. Cross-origin isolation
- **G1 Not isolated:** B2 and C1 work everywhere; no header or embed constraints; timers at 100 µs (F30).
- **G2 Isolated in production:** enables B3/C2; COOP `same-origin` + COEP `require-corp` (Safari, F22); every
  cross-origin resource needs CORP or CORS and frames must be isolated too (F21); cross-origin popups lose their
  opener (F23).
- **G3 Isolated in development and previews only,** for 5 µs timers and the memory API in profiling (F20, F30),
  while production runs G1. The boundary code is the same either way if B2/C1 is chosen.

### Q7. Who owns the network connection (shared with R2)
- **H1 The page** receives packets and forwards them to the worker: two hops per packet, and a busy frame delays
  server snapshots and prediction.
- **H2 The worker** opens WebTransport or WebSocket itself (F29): server snapshots, prediction and reconciliation
  never touch the page; the page only sees the presentation snapshot.

## 5. Recommendations

- **R1-BND-REC1. Run the client simulation in one dedicated module Web Worker, single-threaded, on stable Rust
  (A2 + E1).** The page keeps input listeners, the HUD and rendering and never instantiates the sim. Reason: the
  tick leaves the render thread, the worker form works in all four target browsers, and Wasm threads would bring
  nightly Rust, a rebuilt std and isolation for no §6 need. Rests on: F12, F18, F19, F25, F27, F28, F33 (and legacy GP13,
  GP14, GC2). Serves: VISION_V2 §6.2 (the 3D sim and its line-of-sight work run every tick off the render thread),
  §6.9 (the render thread keeps its frame budget), §4 (four browsers; minimum 30+ fps). Spike must prove (proposed):
  (1) on the proposed minimum reference machine, a laptop with integrated graphics per VISION_V2 §4 (model named in
  the ADR, its `hardwareConcurrency` recorded), at the LOOP_V2 §11 scale points N = 12, 24, 36, 48 soldiers, the
  page's frame time p95 with the sim in the worker is no worse than with no sim at all by more than 1.0 ms; (2) the
  page makes zero calls into Wasm, and the worker makes the same number of Wasm calls per tick at N = 12 and N = 48
  (at most 4); (3) the same build runs in the latest desktop Chrome, Edge, Firefox and Safari.
- **R1-BND-REC2. One flat, versioned output block per tick, written by Rust into preallocated linear memory and handed
  to the page as one transferred ArrayBuffer from a small pool (B2); do not share the sim's Wasm memory (not B4).**
  The block holds a header (tick, layout hash, counts per entity kind), aligned sections per entity kind (soldiers,
  vehicle parts, projectiles, structures) and an event section for the tick (hits, deaths, purchases, captures,
  moments). The worker copies the block out once per tick; the page binds typed-array views (no copy) and keeps the
  latest two blocks for interpolation, returning older ones to the pool. B3 (SharedArrayBuffer ring) is the
  spike's measured alternative. Reason: this is the book's "pointers and overlays" rule with one bounded copy,
  without isolation or nightly Rust; B4's zero copies cost the threads toolchain, isolation, a fixed memory maximum
  and re-binding on growth. Rests on: F7, F10, F11, F12, F15, F16, F17, F18 (and legacy GA15, GP10, GC2). Serves:
  §6.2, §6.4 (multi-part vehicles as records), §6.8 (moments arrive as events), §6.9. Spike must prove (proposed):
  (1) per tick: at most one block copy in the worker, zero copies and zero per-entity calls on the page; (2) on the
  page, no buffer or typed-array allocation per frame after warm-up, and boundary-caused heap growth under
  0.1 MB/s over a 10-minute match (allocation sampling); (3) the time from the worker finishing tick T to T's block
  being readable on the page is at most one display frame (16.7 ms at 60 Hz) at p95, measured on a shared clock
  (`performance.timeOrigin + performance.now()` on both sides, since a worker has its own time origin, F30); (4) zero
  lost or reordered events over a 10-minute match with 48 soldiers; (5) B2 against B3 on the four browsers: if B3
  saves at least 0.5 ms of page time per frame or one frame of latency at p95 on the minimum machine, the ADR weighs
  isolation (REC5); otherwise B2 stands.
- **R1-BND-REC3. Inputs enter as one command per tick, posted from the page to the worker (C1).** The command
  carries seq, tick, move, aim (yaw and pitch, for aiming at height), the held-button bitmask and press and release
  edge counters; the worker stamps the tick it applies it to. The exact fields are R2's. Reason: one small message
  per tick is constant work and needs no isolation; edge counters fix the legacy event-time actions (GP2) and the
  never-wired command (CS1). Rests on: F15 (small copied messages), F12 (C2 would need isolation) and legacy CS1,
  GP2. Serves: §6.2 (aiming at height), §4 (fair up to about 100 ms of ping: commands carry ticks for R2's lag
  compensation). Spike must prove (proposed): (1) in a test with injected delay, duplication and reordering of
  commands, zero lost and zero doubled presses over 10,000 scripted presses; (2) offline, from the key event's
  timestamp to the tick that applies it: at most one tick plus one frame at p95.
- **R1-BND-REC4. Generate every type that crosses the boundary from Rust, from three sources: wasm-bindgen's default
  `.d.ts` for the small control API (D1); our own small generator for the output block's layout, with a layout hash
  checked at startup (D4); ts-rs for low-rate messages, protocol and content-schema types (D3). Not tsify for now.**
  Pin the wasm-bindgen CLI to the library version. Reason: D1 is free and needed anyway; only D4 covers the bulk
  block; ts-rs has no runtime part and serves server, client and tools alike, while tsify's values cross by serde
  (per-field calls) and its maintenance line is unclear. Rests on: F2, F3, F4, F5, F8, F9, F10, F11. Serves: §6.3
  and §6.7 (units, abilities and faction content as data typed from one definition), and the starting principle of
  a narrow typed boundary. Spike must prove (proposed): (1) CI regenerates all types and finds zero diff against the
  committed output; (2) a deliberate Rust layout change without regenerating fails at build or startup (layout hash
  mismatch), proven by a test; (3) `tsc --strict` passes on the generated output; (4) CI fails when the wasm-bindgen
  CLI and library versions differ.
- **R1-BND-REC5. Do not require cross-origin isolation for v2's start (G1); allow it in development and previews
  for profiling (G3). Adopt it in production only if the spike's B2-against-B3 test passes REC2's threshold.** If
  adopted: COOP `same-origin` with COEP `require-corp` (Safari has no `credentialless`), every asset same-origin or
  served with CORP, rollout through the report-only header first, and the later login (VISION_V2 §5) on a page or
  redirect flow outside the isolated game page. Reason: isolation buys only B3/C2 and finer timers, and it costs
  constraints on hosting, embeds and popups on every browser. Rests on: F12, F20, F21, F22, F23, F24, F26, F30.
  Serves: §4 (four browsers including Safari), §5 (guest play first, login later). Spike must prove (proposed): if
  isolation is adopted, `crossOriginIsolated` is true in all four browsers on a preview link, and the report-only
  run of a full match shows zero COEP violations.
- **R1-BND-REC6. The worker owns the network connection in multiplayer and runs the same sim locally when offline
  (H2).** Online, server snapshots, prediction and reconciliation run in the worker, which then publishes the same
  output block to the page; offline, the same worker build runs the match itself, with bots filling the slots. The
  page code is the same in both. Reason: both planned transports exist in workers, so the page never handles a
  network packet, and one boundary serves prediction and offline play. Rests on: F29, F25 (and legacy GP13). Serves:
  §6.1 (modes as data; rooms and private matches), VISION_V2 §3 ("Bots fill empty slots"), §4 (fair up to about
  100 ms: prediction is not delayed by a busy frame). Spike must prove (proposed): (1) the page handles zero network
  messages (a counter); (2) a 100 ms busy loop injected on the page leaves no gap in the worker's processing of
  server snapshots (worker timestamps); (3) the offline match and the online client run the same Wasm file and the
  same page code, differing only in a mode flag. Offline play's status as a v2 need is OQ1.

## 6. Open questions

- **R1-BND-OQ1. Is offline (solo against bots) play a v2 need?** VISION_V2 §6 does not list it; §3 says bots fill
  empty slots, and the legacy game is PvE against bots. REC6 costs no extra boundary work if yes. Decides: Mustafa
  (VISION_V2).
- **R1-BND-OQ2. Does a worker's tick timer keep its rate in a hidden tab, in each of the four browsers?** Chrome's
  throttling article does not mention workers (F31). It matters for offline matches and for prediction while the
  player has tabbed away. Decides: a spike measurement (tick rate with the tab hidden for more than 5 minutes, per
  browser).
- **R1-BND-OQ3. What is the minimum reference machine, and how many cores does it give the page?**
  `hardwareConcurrency` may under-report (F32), and no source read here names the integrated-graphics laptop of
  VISION_V2 §4. Decides: the ADR names a model; the spike records its core count and measures page and worker
  contention.
- **R1-BND-OQ4. Which tsify line, if any, is maintained?** Two forks exist (F8). Only matters if ts-rs proves short.
  Decides: reading both repositories' activity (github.com, allowed) in the synthesis.
- **R1-BND-OQ5. Will the game page ever need third-party embeds** (video, ads, analytics, social or payment widgets)
  **or cross-origin popups?** Each would need CORP or CORS under isolation, or a separate page (F21, F23). Decides:
  Mustafa (VISION_V2 §5) with R6.
- **R1-BND-OQ6. How big is one output block for the largest mode, and is the layout per entity or per field?** It
  depends on the entity caps (soldiers, multi-part vehicles, projectiles, events), on R2's snapshot contents and on
  R3's instancing needs. It decides whether the one copy per tick matters. Decides: the spike measures bytes per
  tick and copy time at N = 48.
- **R1-BND-OQ7. Is wasm-bindgen's new home healthy for a multi-year project?** It moved in 2025 and broke
  compatibility three times in 2026 (F1, F2). Decides: reading the maintenance issue (#4533) and the guide's
  governance page in the synthesis; pinning versions limits the risk meanwhile.
- **R1-BND-OQ8. Does the server's snapshot reach the worker in the same layout as the page's block?** Reusing it
  would save a translation but couples the wire format to presentation. Decides: R2 (snapshot format) with ADR-0001.

## 7. Sources read

All read 2026-10-27.
1. https://blog.rust-lang.org/inside-rust/2025/07/21/sunsetting-the-rustwasm-github-org/ · "Sunsetting the rustwasm
   GitHub org" · 2025-07-21.
2. https://github.com/wasm-bindgen/wasm-bindgen/releases · wasm-bindgen releases · 0.2.118 to 0.2.129 (2026).
3. https://github.com/wasm-bindgen/wasm-bindgen/releases/tag/0.2.128 · release 0.2.128 · "05 Sep" (2026).
4. https://rustwasm.github.io/docs/wasm-bindgen/examples/raytrace.html · old guide page, marked "no longer
   maintained at this domain" (read only for that notice).
5. https://raw.githubusercontent.com/wasm-bindgen/wasm-bindgen/main/guide/src/examples/raytrace.md · Parallel
   Raytracing · main, undated.
6. https://raw.githubusercontent.com/wasm-bindgen/wasm-bindgen/main/guide/src/SUMMARY.md · guide contents · main.
7. https://raw.githubusercontent.com/wasm-bindgen/wasm-bindgen/main/guide/src/reference/types/number-slices.md ·
   Number Slices · main.
8. https://raw.githubusercontent.com/wasm-bindgen/wasm-bindgen/main/guide/src/reference/arbitrary-data-with-serde.md
   · Arbitrary Data with Serde · main.
9. https://raw.githubusercontent.com/wasm-bindgen/wasm-bindgen/main/guide/src/examples/wasm-in-web-worker.md · Wasm
   in Web Worker · main.
10. https://raw.githubusercontent.com/wasm-bindgen/wasm-bindgen/main/guide/src/reference/browser-support.md ·
    Supported Browsers · main.
11. https://raw.githubusercontent.com/wasm-bindgen/wasm-bindgen/main/guide/src/reference/cli.md · Command Line
    Interface · main.
12. https://raw.githubusercontent.com/wasm-bindgen/wasm-bindgen/main/guide/src/reference/attributes/on-rust-exports/typescript_custom_section.md
    · typescript_custom_section · main.
13. https://doc.rust-lang.org/rustc/platform-support/wasm32-unknown-unknown.html · rustc book, wasm32-unknown-unknown
    · current (unwinding section "as of 2025-10-03").
14. https://doc.rust-lang.org/reference/type-layout.html · Rust Reference, Type layout · current.
15. https://rustwasm.github.io/docs/book/game-of-life/implementing.html · Rust and WebAssembly book, Implementing
    Life · archived ("no longer maintained").
16. https://docs.rs/tsify/latest/tsify/ · tsify 0.5.8.
17. https://docs.rs/tsify-next/latest/tsify_next/ · tsify-next 0.5.6 (2026-10-05).
18. https://docs.rs/ts-rs/latest/ts_rs/ · ts-rs 12.0.1.
19. https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer · MDN ·
    2026-02-10.
20. https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Atomics · MDN · 2026-02-10.
21. https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Atomics/waitAsync · MDN ·
    2026-02-10.
22. https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray · MDN · 2026-10-26.
23. https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Float32Array/Float32Array · MDN ·
    2026-10-26 (defers to TypedArray).
24. https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Memory/grow · MDN ·
    2026-05-14.
25. https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API/Transferable_objects · MDN · 2025-09-18.
26. https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Cross-Origin-Embedder-Policy · MDN ·
    2026-09-04.
27. https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Cross-Origin-Opener-Policy · MDN ·
    2026-07-12.
28. https://developer.mozilla.org/en-US/docs/Web/API/Window/crossOriginIsolated · MDN · 2026-08-21.
29. https://developer.mozilla.org/en-US/docs/Web/API/Performance/now · MDN · 2025-08-25.
30. https://developer.mozilla.org/en-US/docs/Web/API/Navigator/hardwareConcurrency · MDN · 2024-10-16.
31. https://developer.mozilla.org/en-US/docs/Web/API/WebTransport · MDN · 2026-10-12.
32. https://developer.mozilla.org/en-US/docs/Web/API/WebSocket · MDN · 2024-09-25.
33. https://raw.githubusercontent.com/mdn/browser-compat-data/main/http/headers/Cross-Origin-Embedder-Policy.json ·
    MDN browser-compat-data · main.
34. https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/builtins/Atomics.json · main.
35. https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/builtins/SharedArrayBuffer.json · main.
36. https://raw.githubusercontent.com/mdn/browser-compat-data/main/api/Worker.json · main.
37. https://raw.githubusercontent.com/WebAssembly/website/main/features.json · webassembly.org feature table · main.
38. https://raw.githubusercontent.com/WebAssembly/threads/main/proposals/threads/Overview.md · Wasm threads proposal
    overview · main.
39. https://web.dev/articles/coop-coep · "Making your website cross-origin isolated using COOP and COEP" · last
    updated 2022-06-21.
40. https://developer.chrome.com/blog/timer-throttling-in-chrome-88 · "Heavy throttling of chained JS timers
    beginning in Chrome 88" · last updated 2021-01-18.
41. https://vite.dev/guide/features · Vite features · docs v8.3.4.
42. https://vite.dev/config/server-options · Vite server options · docs v8.3.4.

### Sources not read
- https://wasm-bindgen.github.io/wasm-bindgen/ (the guide's new home): outside the allow list (the same guide was read
  from its source on raw.githubusercontent.com).
- https://gh.nn.ci/emscripten-core/emscripten/discussions/24472 (search lead): outside the allow list.
- https://wiki.mozilla.org/Platform/Games/StatusUpdate (search lead): outside the allow list.
- https://hacks.mozilla.org/2017/07/the-next-generation-of-web-gaming/ (allowed): not read: 2017 background,
  superseded by the compatibility data.
- https://www.w3.org/2019/09/03-me-irc (allowed): not read: a meeting log found by search, other tier.
- https://github.com/wasm-bindgen/wasm-bindgen/issues/4533 (allowed): not read in this run; see OQ7.
- Shipped-game talks or papers on a browser game's sim in a worker: none found by search (no source to list).

## 8. Cross-unit notes

- **R1 fixed tick rate:** the worker's tick timer, drift and hidden-tab behaviour (OQ2); how the page interpolates
  between the two latest blocks.
- **R1 identical results Wasm vs native:** the boundary feeds the sim only commands with tick numbers and content
  data; no wall clock or JS randomness enters through it (legacy GP14's leaks).
- **R1 ECS or plain data:** the cost of writing the output block each tick depends on the storage layout; stable slot
  ids (GC2) for indexing.
- **R1 ability and effect system:** cues and effects reach presentation as typed events in the block's event
  section, not as callbacks.
- **R1 data formats and hot reload:** content and tuning enter the worker as low-rate messages; ts-rs types for the
  schemas shared with the editor (REC4); hot reload is a message to the worker.
- **R2:** the input command's fields and edge semantics; whether the worker owns the connection (REC6, F29); the
  server snapshot format against the page block (OQ8); prediction replay cost inside the worker.
- **R3:** rendering in a worker with OffscreenCanvas (Q1 A3); per-entity against per-field layout for instancing
  (OQ6).
- **R5:** Vite's worker and Wasm setup (F33); `server.headers` for COOP and COEP in development (G3); CI checks for
  generated-type diffs and the wasm-bindgen version pin (REC4); Playwright runs on all four browsers.
- **R6:** whether preview and production hosting can set COOP, COEP and CORP headers; third-party embeds and the
  later login flow under isolation (REC5, OQ5). COOP also guards against XS-Leaks (COOP page), a pointer for the
  deferred security baseline.
