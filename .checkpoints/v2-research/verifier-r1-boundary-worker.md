Task: verify R1 (docs/research/r1-simulation-core.md) recommendations R1-BND-* (boundary + Web Worker), wave 1 step 3 Verify. Started 2026-10-28.
Scope: REC1-REC6 (doc lines 2043-2112). Findings to check: F2 F3 F4 F5 F7 F8 F9 F10 F11 F12 F15 F16 F17 F18 F19 F20 F21 F22 F23 F24 F25 F26 F27 F28 F29 F30 F33; legacy GP13 GP14 GC2 GA15 GP10 CS1 GP2; numbers from VISION_V2 §3/§4/§5/§6, LOOP_V2 §11.
Plan: local docs/code first, then web sources in batches.
## Verdicts (appended as found)
- F2a (CLI = lib version since 0.2.128): CONFIRMED. github.com/wasm-bindgen/wasm-bindgen/releases/tag/0.2.128, "Requires matching `wasm-bindgen` and CLI versions (schema bump)." Date line "released this 05 Sep" (no year).
- F3 (.d.ts by default): CONFIRMED. guide cli.md, --typescript: "Output a TypeScript declaration file for the generated JavaScript bindings. This is on by default."
- F4: CONFIRMED. typescript_custom_section.md: "it will append the contents of the string to the `.d.ts` file exported by `wasm-bindgen-cli`" / "The primary target for this feature is for code generation."
- F5: CONFIRMED. arbitrary-data-with-serde.md, heading "An alternative approach - using JSON": "`serde-wasm-bindgen` works by directly manipulating JavaScript values." / "This requires a lot of calls back and forth between Rust and JavaScript, which can sometimes be slow." / "the JSON approach can be anywhere from 2x to 0.2x the speed of `serde-wasm-bindgen`"
Next: F2b cadence (releases list), F7-F11.
- F2b (cadence, breaking in .122/.123/.128, year 2026): PARTLY. releases page: dates 0.2.118 "10 Apr" .. 0.2.129 "25 Sep" confirmed; 0.2.123 "a breaking change for callers that relied on widening `&mut` references." confirmed; 0.2.122 breaking: UNSUPPORTED (no "breaking" in its notes); year: unsupported (no year shown). Not load-bearing for REC4 (REC4 uses only the version-match rule).
- F7: CONFIRMED. rustwasm book game-of-life/implementing: "Minimizing copying into and out of the WebAssembly linear memory." / "large, long-lived data structures are implemented as Rust types that live in the WebAssembly linear memory" / "By working with pointers and overlays, we avoid copying the cells across the boundary on every tick." Banner: "This project and website is no longer maintained".
- F8: CONFIRMED (now incl. serde route & deprecation). docs.rs/tsify 0.5.8: "Tsify is a library for generating TypeScript definitions from Rust code." / js feature "enables serialization through serde-wasm-bindgen" / "Deprecated in favour of using Ts<T> as on function parameters and return type". tsify-next pending.
- F9: CONFIRMED (MSRV not checked). docs.rs/ts-rs 12.0.1: "Generate typescript type declarations from rust types" / "When running `cargo test` or `cargo test export_bindings`, the following TypeScript type will be exported to `bindings/User.ts`" / "With the `serde-compat` feature (enabled by default), serde attributes are parsed for enums and structs." Note: "no runtime part" is inference from test-time export (no quote).
- F8b tsify-next: CONFIRMED as stated. docs.rs/tsify-next: "tsify-next 0.5.6", repo github.com/siefkenj/tsify, "Tsify-next is a library for generating TypeScript definitions from rust code."; no maintenance text (so "neither page says which line is maintained" holds). Page date 2026-10-05 not checked.
- F10: CONFIRMED. Rust Reference type-layout, "The `Rust` representation": "The only data layout guarantees made by this representation are those required for soundness." / "The ordering does not have to be the same as the order in which the fields are specified in the declaration of the type."; "The `C` representation": "The second purpose is to create types that you can soundly perform operations on that rely on data layout such as reinterpreting values as a different type."; "#[repr(C)] Structs": "...add padding bytes to the current offset until it is a multiple of the field's alignment."
- F11: CONFIRMED. MDN TypedArray, Constructor: "When called with an ArrayBuffer or SharedArrayBuffer instance, and optionally a `byteOffset` and a `length` argument, a new typed array view is created that views the specified buffer." Exceptions/RangeError: "...or `byteOffset` is not an integral multiple of the new typed array's element size."
- F12: CONFIRMED. MDN SharedArrayBuffer, Description/Security requirements: "To use shared memory your document must be in a secure context and cross-origin isolated." / API availability: "Unless the two headers mentioned above are set, the various `postMessage()` APIs will throw for `SharedArrayBuffer` objects. If they are set, `postMessage()` on `Window` objects and dedicated workers will function and allow for memory sharing."
- F17a (shared Memory = SAB): CONFIRMED. same page, WebAssembly shared memory: "When this flag is set to `true`, the constructed `Memory` object can be shared between workers via `postMessage()`, just like `SharedArrayBuffer`, and the backing buffer of the `Memory` object is a `SharedArrayBuffer`."
Next: F15, F16, F17b (threads Overview), F18, F19.
- F15a (transfer = zero-copy move, sender loses it): CONFIRMED. MDN Transferable objects: "...the memory resource that it points to is _literally_ moved between contexts in a fast and efficient zero-copy operation." / "Following a transfer, the original object is no longer usable; it no longer points to the transferred resource, and any attempt to read or write the object will throw an exception."
- F15b (without transfer list data is copied): UNSUPPORTED in cited source (no sentence; only a structuredClone code example). Doc itself marks it inferred. REC3 cites F15 exactly for "small copied messages" -> REC3 basis unsupported in this source (true per HTML structured clone, but not cited).
- F15c (typed arrays not transferable; WebTransport streams listed): CONFIRMED. "Typed arrays like Int32Array and Uint8Array, are serializable, but not transferable. However their underlying buffer is an ArrayBuffer, which is a transferable object." List includes WebTransportReceiveStream, WebTransportSendStream.
- F16: CONFIRMED. MDN Memory.grow, Examples > "Detachment upon growing": "Every call to `grow` will detach any references to the old `buffer`, even for `grow(0)`!" / "For a shared `Memory` instance, the initial `buffer` ... will not become detached, but rather its length will not be updated."
- F17b/c (must declare shared to import shared; shared needs max): CONFIRMED with note. threads Overview.md, "Shared Linear Memory": "It is a validation error to attempt to import shared linear memory if the module's memory import doesn't specify that it allows..." (truncated by tool) supports the claim; the doc's own quote ("import non-shared memory if the module's memory import specifies it as being shared") is the OTHER direction, so quote and claim are mismatched in the doc (the doc labels the right rule "inferred"). "Spec Changes": "Note that shared linear memory without an explicit maximum size is not permitted." confirmed.
- F18: CONFIRMED. wasm-bindgen guide raytrace.md: "Note that this requires a nightly Rust toolchain." / "export RUSTFLAGS='-C target-feature=+atomics'" + "-Z build-std=panic_abort,std" / "Currently it's required to use the `--target no-modules` or `--target web` flag with `wasm-bindgen` to run threaded code." / "The main thread in a browser cannot block." / "the standard library has no viable route to implement the `std::thread` module." / "This demo should work in the latest Firefox and Chrome versions at this time, and other browsers are likely to follow suit."
Next: F19, F20-F24.
- F19: CONFIRMED. rustc book wasm32-unknown-unknown: "...and `std::thread::spawn` will panic. There is no means by which this can be overridden." Default list: multivalue, mutable-globals, reference-types, sign-ext, "nontrapping-fptoint (Rust 1.87.0+, LLVM 20+)", "bulk-memory (Rust 1.87.0+, LLVM 20+)". "Tier: 2".
- F20: CONFIRMED. MDN COOP: "To use these features in a document, you will need to set the COOP header to `same-origin` and the `Cross-Origin-Embedder-Policy` header to `require-corp` (or `credentialless`)." MDN crossOriginIsolated: "`Performance.now()` offers better precision." / "`Performance.measureUserAgentSpecificMemory()` can be called."
- F21a/b: CONFIRMED. MDN COEP require-corp: "A document can only load resources requested in no-cors mode from the same origin, or resources that have explicitly set the Cross-Origin-Resource-Policy header to a value that allows it to be embedded." credentialless: "...requests are sent without credentials: cookies are omitted in the request, and ignored in the response." Frames: MDN COEP "The policy applies to loaded resources, and resources in <iframe>s and nested frames." (web.dev chain quote pending).
- F23a/b (same-origin severs popups; allow-popups for OAuth/payments): CONFIRMED. MDN COOP: "A document with `same-origin` will always open a document with `unsafe-none` in a new BCG." / "When opened in a new BCG, any references between the new document and its opener are severed, and the new document may be process-isolated from its opener." / same-origin-allow-popups "...For example, when using a cross-origin service for OAuth or payments."
- F23c ("the value meant for those popups does not give isolation"): UNSUPPORTED in cited source; MDN says the directive is for "integrations where a document needs the benefits of cross-origin isolation but also needs to open and retain a reference to trusted cross-origin documents" (reads the other way). Checking web.dev.
Next: web.dev coop-coep (F21c, F23c, F24), F22, F25-F28 compat data.
- F21c (chain incl. frames): CONFIRMED. web.dev coop-coep: "The entire chain of the documents, including parent frames and child frames, must be cross-origin isolated, too."
- F24a (report-only): CONFIRMED. web.dev: "You will receive reports without blocking embedded content."
- F24b (headers on worker scripts): CONFIRMED (doc said inferred). web.dev: "Recursively apply this to **all** documents including the top-level document, iframes and worker scripts."
- F23c: stays UNSUPPORTED; web.dev changelog (2021-04-16) reads the other way: "COOP same-origin-allow-popups to be a relaxed condition for cross-origin isolation". Neither cited source says it gives no isolation. Impact on REC5 small: "login outside the isolated page" is safe either way, but the stated reason is unsourced.
- F22 web.dev line out of date: CONFIRMED that web.dev says "Credentialless mode is available from Chrome 96 and in Edge, but is not yet supported by other browsers." (dated 2022-06-21).
Next: BCD JSONs F22, F25, F26; webassembly features F27; F28.
- F22: CONFIRMED. BCD http/headers/Cross-Origin-Embedder-Policy.json: header chrome "83", edge "mirror", firefox "79", safari "15.2"; credentialless chrome "96", edge "mirror", firefox "119", safari {"version_added": false}.
- F26: CONFIRMED. BCD SharedArrayBuffer.json __compat: chrome "68", edge "mirror", firefox "79", safari "15.2"; grow/growable/maxByteLength: chrome "111", firefox "128", safari "16.4". Atomics.json: wait & notify chrome "68", firefox "78", safari "15.2"; waitAsync chrome[0] "90" (87-90 partial), firefox "145", safari "16.4". (Baseline 2025 line pending, waitAsync MDN page.)
- F25 BCD Worker.json options_type_parameter: values chrome 80 / edge mirror / firefox 114 / safari 15 reported by tool as summary -> re-asking for verbatim.
- F25: CONFIRMED. BCD api/Worker.json options_type_parameter: "chrome": { "version_added": "80" }, "edge": "mirror", "firefox": { "version_added": "114" }, "safari": { "version_added": "15" }. wasm-bindgen guide wasm-in-web-worker.md: "At the time of this writing, only Chrome supports modules in web workers, e.g. Firefox does not." (out of date, as F25 says).
- F27: CONFIRMED (values). webassembly website features.json: chrome threads "74", bulkMemory "75", referenceTypes "96", saturatedFloatToInt "75", signExtensions "74", multiValue "85"; firefox "79"/"79"/"79"/"64"/"62"/"78"; safari threads "14.1", bulk "15", ref "15", sat "15", signExt "14.1", multiValue "13.1". No edge key (Edge=Chromium inferred, as doc says). Note: mutable-globals (in Rust's default list, F19) is not listed in F27 -> small gap in "the features a default Rust build uses".
- F28: CONFIRMED. guide browser-support.md: "Firefox, Chrome, Safari, and Edge browsers are all supported by `wasm-bindgen`."
Next: F29, F30, F33, waitAsync Baseline (F26 tail), then legacy facts + VISION/LOOP numbers.
- F29: CONFIRMED. MDN WebTransport: "This feature is available in Web Workers." / "Since March 2026, this feature works across the latest devices and browser versions." MDN WebSocket: "Note: This feature is available in Web Workers."
- F30: CONFIRMED. MDN Performance.now, Security requirements: "Resolution in isolated contexts: 5 microseconds" / "Resolution in non-isolated contexts: 100 microseconds"; intro: "...the time when navigation has started in window contexts, or the time when the worker is run in Worker and ServiceWorker contexts)." Note: "This feature is available in Web Workers." Supports REC2 mark 3's shared-clock method.
- F33a: CONFIRMED. vite.dev/guide/features (header "v8.3.4"), Web Workers > Import with Constructors: "A web worker script can be imported using new Worker() and new SharedWorker()." / "The worker detection will only work if the new URL() constructor is used directly inside the new Worker() declaration." WebAssembly > Manual Initialization: "When you need control over when and how the module is instantiated, import it with ?init."
Next: vite server.headers, waitAsync Baseline, then local legacy/VISION/LOOP.
- F33b: CONFIRMED. vite.dev/config/server-options, "server.headers": "Specify server response headers."
- F26 tail: CONFIRMED. MDN waitAsync: "Baseline 2025" / "Since November 2025, this feature works across the latest devices and browser versions."
- (extra, not in any BND rests-on) F13/F14: Atomics page "(Most browsers will not allow `wait()` on the browser's main thread.)", "Atomic operations make sure that predictable values are written and read..." CONFIRMED; waitAsync "It is non-blocking and, unlike Atomics.wait(), can be used on the main thread." CONFIRMED.
Web sources done. Next: legacy facts GP13 GP14 GC2 GA15 GP10 CS1 GP2 and VISION_V2/LOOP_V2 numbers.
- Legacy CS1: CONFIRMED. shamble-core.js:1479 `const FIXED_DT=1/60;` :1480 `const BTN={FIRE:1,ADS:2,RELOAD:4,MELEE:8,GRENADE:16,SHIELD:32,SPRINT:64};` :1481 makeInputCommand(seq,tick) {seq,tick,dt,moveX,moveZ,aimX,aimZ,aimY,buttons}; LEGACY_INVENTORY CS1 row "Nothing reads any of them", mark drop.
- Legacy GP13: CONFIRMED. shamble-game.js:3053 `const dt=Math.min(clock.getDelta(),0.05)`; update only if !hitStop && !paused && !shopOpen && !tutorialOpen. Inventory GP13 mark redesign.
- Legacy GP14: CONFIRMED. Inventory GP14 "HUD writes are interleaved throughout"; AUDIT_BUGS_PERF.md:10 "`update()` 0.49 ms/frame CPU".
- Legacy GP10: CONFIRMED. AUDIT_BUGS_PERF.md:27 P3 "~30 DOM writes per frame in `update()`"; :85 "0.01 real writes per" frame. Inventory GP10 port to TS, look brief pending.
- Legacy GC2: CONFIRMED. Inventory GC2 "12 fixed slots"; docs/reports/2026-10-16-three-teams-core.md:48 "48 soldiers (head): combat 0.80–1.00".
- Legacy GA15: CONFIRMED. Inventory GA15: struct fields aim yaw, reload progress, throw charge, grounded, ADS; "Aim pitch travels outside the struct: pitchWeapon writes the gun's rotation.x".
- Legacy GP2: CONFIRMED. Inventory GP2 lists event-time actions; v2 note "press and release edges ... neither lost nor doubled".
Next: VISION_V2 §3/§4/§5/§6, LOOP_V2 §11.
- VISION_V2 numbers: CONFIRMED. §3 "`[DEFAULT]` ... Bots fill empty slots."; §4 "Desktop browsers first: Chrome, Edge, Firefox and Safari." / "**Minimum:** a laptop with integrated graphics runs the largest mode at 30+ fps on Low." / "Matches feel fair up to about 100 ms of ping."; §5 "Guest play first; a simple login arrives with account progression"; §6.1 modes as data + rooms and private matches; §6.2 "aiming at height"; §6.3 data-driven abilities; §6.4 "big and multi-part"; §6.7 faction content as data; §6.8 moments; §6.9 spectacle budget "without dropping frames".
- LOOP_V2 §11 (line 153-155): "Then run the scale test on this map (N = 12, 24, 36, 48)." CONFIRMED.
- Pass marks: every REC's spike list is headed "Spike must prove (proposed)"; 16.7 ms = 1/60 s arithmetic. No unmarked unsourced number found.
Flags drafted (to rank in final): REC2 missing basis "Wasm memory buffer cannot be transferred, hence the one copy"; REC2 uses F30 inline but not in Rests-on; REC1 mark records hardwareConcurrency (F32) not in Rests-on; REC3 F15 copy-by-default unsupported; REC5 F23c unsupported, "hosting constraints" no finding, "isolation buys only" omits measureUserAgentSpecificMemory (F20); REC6 offline play traces to no §6 item (self-flagged OQ1), §6.1 trace weak; REC1 §6.2 parenthetical adds "off the render thread" not in §6.2; F27 omits mutable-globals; OQ7 "broke compatibility three times" leans on unsupported 0.2.122.
Next: optional F32 check, then final table + DONE.
- (extra, not in BND rests-on) F32: CONFIRMED. MDN hardwareConcurrency: "...returns the number of logical processors available to run threads on the user's computer." / "The browser may, however, choose to report a lower number of logical cores in order to represent more accurately the number of Workers that can run at once, so don't treat this as an absolute measurement..."
- REC2 missing-basis probe: WebAssembly JS API spec (webassembly.github.io/spec/js-api) §5.3 Memories: "Set buffer.[[ArrayBufferDetachKey]] to "WebAssembly.Memory"." -> the memory's own buffer has a detach key, so user code cannot transfer it (the throw itself is ECMA-262 DetachArrayBuffer, not on allow list, not read). This is why REC2 must copy once; R1 has no finding for it.

## Final table
| id | claim | source | verdict | location |
|---|---|---|---|---|
| F2a | CLI and lib must match since 0.2.128 | GH release 0.2.128 | confirmed | release notes, "Requires matching `wasm-bindgen` and CLI versions (schema bump)." |
| F2b | cadence; breaking in .122/.123/.128; year 2026 | GH releases list | partly: dates + .123 confirmed; .122 breaking and year unsupported | .123: "a breaking change for callers that relied on widening `&mut` references." |
| F3 | .d.ts on by default | guide cli.md | confirmed | --typescript |
| F4 | typescript_custom_section appends to .d.ts | guide attr page | confirmed | body |
| F5 | serde-wasm-bindgen many calls; JSON 2x-0.2x | guide serde page | confirmed | "An alternative approach - using JSON" |
| F7 | keep data in linear memory, overlay views | rustwasm book | confirmed | game-of-life/implementing |
| F8 | tsify desc, serde route, deprecation, tsify-next exists, status unclear | docs.rs tsify, tsify-next | confirmed | crate root pages |
| F9 | ts-rs test-time generation | docs.rs ts-rs 12.0.1 | confirmed ("no runtime part" inferred; MSRV not checked) | crate root |
| F10 | only repr(C) gives readable layout | Rust Reference type-layout | confirmed | "The Rust representation", "The C representation", "#[repr(C)] Structs" |
| F11 | typed array = view; offset multiple of element size | MDN TypedArray | confirmed | constructor; Exceptions RangeError |
| F12 | SAB needs secure + isolated; postMessage throws | MDN SharedArrayBuffer | confirmed | Security requirements / API availability |
| F15a | transfer = zero-copy move, sender loses it | MDN Transferable objects | confirmed | intro paragraphs |
| F15b | without transfer list data is copied | MDN Transferable objects | unsupported (no sentence; code example only) | - |
| F15c | typed arrays not transferable; WT streams listed | MDN Transferable objects | confirmed | list |
| F16 | grow detaches; shared keeps old length | MDN Memory.grow | confirmed | Examples > Detachment upon growing |
| F17a | shared Memory buffer is a SAB | MDN SharedArrayBuffer | confirmed | WebAssembly shared memory |
| F17b | module must declare shared to import shared | threads Overview.md | confirmed (doc's quote is the reverse rule) | Shared Linear Memory |
| F17c | shared memory needs a maximum | threads Overview.md | confirmed | Spec Changes |
| F18 | threads need nightly, +atomics, build-std, web/no-modules | guide raytrace.md | confirmed | body |
| F19 | thread::spawn panics; default features | rustc book wasm32-unknown-unknown | confirmed | body; Tier 2 |
| F20 | isolation = COOP same-origin + COEP; finer timers, memory API | MDN COOP, crossOriginIsolated | confirmed | COOP intro; crossOriginIsolated list |
| F21 | require-corp needs CORP; credentialless; frames | MDN COEP; web.dev coop-coep | confirmed | directives; web.dev body |
| F22 | Safari has no credentialless; versions | BCD COEP json; web.dev | confirmed | support objects |
| F23a | same-origin severs popups | MDN COOP | confirmed | unsafe-none / same-origin |
| F23b | allow-popups is for OAuth/payments | MDN COOP | confirmed | same-origin-allow-popups |
| F23c | allow-popups gives no isolation | MDN COOP; web.dev | unsupported (sources lean the other way) | MDN "needs the benefits of cross-origin isolation"; web.dev 2021 changelog "relaxed condition for cross-origin isolation" |
| F24a | report-only header | web.dev coop-coep | confirmed | "You will receive reports without blocking embedded content." |
| F24b | headers must cover worker scripts | web.dev coop-coep | confirmed (doc said inferred) | "...including the top-level document, iframes and worker scripts." |
| F25 | module workers in all four | BCD Worker.json; guide | confirmed | options_type_parameter 80/mirror/114/15 |
| F26 | SAB, Atomics in all four; waitAsync Baseline 2025 | BCD SAB, Atomics; MDN waitAsync | confirmed | support objects; badge |
| F27 | Wasm features + threads in three engines | webassembly features.json | confirmed (Edge inferred; mutable-globals missing from list) | chrome/firefox/safari keys |
| F28 | wasm-bindgen supports four browsers | guide browser-support.md | confirmed | first sentence |
| F29 | WebTransport, WebSocket in workers | MDN WebTransport, WebSocket | confirmed | notes; Baseline "Since March 2026" |
| F30 | 5 us vs 100 us; worker time origin | MDN Performance.now | confirmed | Security requirements; intro |
| F33 | Vite workers, ?init, server.headers | vite.dev features, server-options | confirmed | Web Workers; WebAssembly > Manual Initialization; server.headers |
| CS1 | FIXED_DT, BTN, makeInputCommand unwired | shamble-core.js:1479-1481; LEGACY_INVENTORY CS1 | confirmed | code |
| GP2 | event-time actions; edges | LEGACY_INVENTORY GP2 | confirmed | row |
| GP13 | rAF variable dt clamp 0.05, freezes | shamble-game.js:3053 | confirmed | loop() |
| GP14 | HUD writes interleaved; 0.49 ms | LEGACY_INVENTORY GP14; AUDIT_BUGS_PERF.md:10 | confirmed | row; runtime profile |
| GP10 | ~30 writes -> 0.01 real writes | AUDIT_BUGS_PERF.md:27, :85 | confirmed | P3; CS3 |
| GC2 | 12 slots; 0.80-1.00 ms at 48 | LEGACY_INVENTORY GC2; reports/2026-10-16-three-teams-core.md:48 | confirmed | row; table |
| GA15 | state struct; aim pitch side channel | LEGACY_INVENTORY GA15 | confirmed | row |
| V3 | bots fill empty slots | VISION_V2 §3 | confirmed | [DEFAULT] line |
| V4 | four browsers; laptop iGPU 30+ fps; ~100 ms ping | VISION_V2 §4 | confirmed | lines 1,2,4 |
| V5 | guest first, login later | VISION_V2 §5 | confirmed | line 1 |
| L11 | N = 12, 24, 36, 48 | LOOP_V2 §11 | confirmed | "Then run the scale test on this map (N = 12, 24, 36, 48)." |
Counts: 46 rows; confirmed 43, partly confirmed 1 (F2b), unsupported 2 (F15b, F23c), wrong 0.
DONE
