Task: change-set 3 ("The loop closes"), the boundary and rule-test half of its plan (engine-architect, part "cs3").

## Plan (blocks, in order)
- Block 1. Read: the card + Step 2.5 table + Part 15 lanes; ADR 0004 parts 7 and 8 + PM-0004-27..29; ADR 0002 crate and
  package rules + PM-0002-53; code (crates/sim/src, crates/sim-wasm/src/lib.rs, crates/client-endpoint/src,
  packages/sim-worker/src + boundary/, packages/client/src/app + overlay/, packages/client/scripts/build.mjs +
  build-checks.mjs); cs2 report; STATUS_ARCHIVE PR #61 block "For change-set 3"; test-engineer-followups.md blocks 5-6;
  rust-reviewer-cs3-rng.md; browser-reports-reading.md (FPS-LINE, NEG-ZERO). Notes per source as read.
- Block 2. The boundaries after cs3 (EA-F9, EA-F10, WB-HEALTH (a), EA-F8, EA-N3, EA-N6/VF-L7, panic hook, follow-up table).
- Block 3. The page, alive (lanes, input path, readout + Copy report, FPS-LINE, NEG-ZERO, five plain lines).
- Block 4. The rule tests (a) float vectors, (b) RNG 10,000 outputs, (c) scan test, (d) PM-0002-53, (e) two-instance test.
- Block 5. Size and order (lines per file, build order with green steps, CI/gate edits named, what moves out first).
- Block 6. Where the build plan / drafts are wrong or out of date; questions for Mustafa.
- Then DONE, then the answer from these notes alone.

## Notes (one line per finding; one after each finished block)
- (start) No engine-architect memory exists under .claude/agent-memory-local/; notes file was absent: fresh start.
- cs3-context.md read (pointers only; card line 249, card 316-333, Step 2.5 463-476, scan ruling 949-953).
- Build plan read (card 316-333; table row 249; Step 2.5 463-476; scan ruling 949-953; Part 3.1 538-539; Part 3.2 548-558;
  cs5 card 365-367). Part 3.2 :551-553 "None is merged into the game ... kept two ways, as a patch file in its
  change-set's report ... and as a build with the bench page".
- FINDING B6-1 (two-instance test owner): Part 15 :105 (LANE-SPLIT (a), his yes, 2026-11-17 22:10) gives "the comparisons
  that stay (Part 3.2)" to LANE 2, listing "one Wasm instance or two (PM-0002-25): change-set 3, then 5" as needing cs3
  first; condition 4 (:78-80) "A comparison is never merged and never edits a game file on main. Lane 2 builds each one
  in a scratch copy after the change-set that gives it its subject is merged". The cs3 card (:327-331) predates Part 15
  and puts the test in cs3. Newer word = Part 15: the test is lane 2's, after cs3 merges, a patch + kept build, not in
  cs3's diff or size. cs3 must only leave the seam it needs (an instance-count-agnostic Worker entry).
- FINDING B6-2: the card's "the table of build options" (:330) is defined nowhere else in docs/ (grep: one hit). Ask
  the main session / plan to name it or drop it.
- Part 15 lanes: lane 1 owns crates/**, packages/types, sim-worker, net, e2e, client's app shell, input, HTML overlay
  and counters, dev panels, interface file, first drawing code (until cs5 hand-over), build script + every build file,
  gate.yml/toolchain.yml/gate files (15.2 :90-95). So key diagram + counters = lane 1 (overlay/counters).
- ADR 0004 part 7 read (:257-308). Allow-list, exact: `+ - * /` and negation; comparisons and `total_cmp`; `sqrt`,
  `floor`, `ceil`, `round`, `trunc`, `fract`, explicit `mul_add`; `abs`, `copysign`; `as` casts; "the exact
  non-arithmetic methods the rules themselves use, such as `is_finite`, `is_nan`, `to_bits` and `from_bits`" (OPEN-ENDED:
  "such as" -> the scan test needs a closed list; B6 item). Banned by name: powi, hypot, cbrt, to_degrees, to_radians,
  min, max, clamp, rem_euclid, div_euclid, float `%` ("review, not the lint, catches the operator"), std sin/cos/atan2/
  powf/exp, algebraic_*, fast-math, relaxed-simd; simd128 off. Three own helpers (min/max/clamp as plain compares; clamp
  with reversed bounds returns a bound); degrees->radians by one multiply with a pinned constant. Transcendentals only via
  pinned `libm` crate (not in cs3: no transcendental needed yet). "debug and test builds assert finite state each tick".
- ADR 0004 part 8 read (:309-337): "The sim draws only from `rand_chacha`'s `ChaCha8Rng`, pinned" -> CONFLICTS with
  rust-reviewer's own-ChaCha8 recommendation (B6 item: part 8 needs an amendment if he picks own code; decision docs need
  his go). Seed from host (Worker crypto.getRandomValues), never sent/logged; own sampling functions + pinned vectors;
  streams' state (seed, stream, position) is in the state hash; streams: match setup, weapon spread, bot decisions,
  ability procs, upgrade offers -- all authority-only today.
- PM-0004-27 (:1148): "the first 10,000 outputs of each stream, and of the sim's own sampling functions on them, equal
  pinned test vectors on every target of PM-0004-19"; 20 identical verdicts; Bites: "a stream seeded from the clock".
  PM-0004-28 (:1149) presentation never reaches sim (RD Chrome; not cs3's). PM-0004-29 (:1150) authority-only streams off
  the client: CI Wasm in Node, needs the prediction step (client-mode instance) -> not before cs7 (no prediction in cs3).
- PM-0004-22 (:1143): >=10,000 fixed inputs per libm fn and `sqrt`, bit-identical native/Wasm-Node/browsers; `sqrt` also
  equals a correctly rounded reference; min/max/clamp helpers same bits for (+0,-0),(-0,+0), reversed clamp returns a
  bound, no panic; Bites: one output moved by one ulp on one target. PM-0004-23 (:1144): lints = gate v2's G2 (2.3's).
- ADR 0002 read: part 3 crate table (:182-251): `protocol` = messages, codec, decoders never panic, Rust side of thin
  transport + loopback backend, deps `sim`/`content` types only; `room` = no I/O/async/clock, slot table, per-client
  command queues, snapshot encoding via protocol; `client-endpoint` = no bindings/transport/async/clock, snapshot
  decoding, block writer + layout table, built in sim-wasm AND natively for headless clients; `sim-wasm` = "the
  wasm-bindgen bindings the Worker loads, and nothing else"; `devtools` = replay runner, hash comparer, tick bench,
  headless clients, deps "anything". Rules (G5/G8): room + client-endpoint build wasm32 + native, no wasm-bindgen, no
  clock; sim-worker never imports rendering/DOM. Part 4 (:252-275): overflow-checks on for `sim` and `protocol` in every
  profile; counters use explicit wrapping_/checked_; Wasm keeps -Cpanic=abort; "A trap carries no panic message, so the
  Wasm build's panic hook (ADR 0004 part 20, S20) writes the message and location into a fixed buffer in linear memory,
  and when the instance traps the Worker's host reads that buffer, through a fresh view of the memory (part 5), and
  attaches it to the trap's report." (=> NOT console_error_panic_hook; a fixed buffer, read by the host.)
- ADR 0002 part 5 (:284-346): control API "(create, mode flag, load the content bundle, one input call per tick, and the
  step) is wasm-bindgen's own"; per tick ONE input call + ONE step call; input call carries newest input + datagrams with
  arrival times + host time, "all as plain values"; step leaves outgoing datagrams and block in linear memory, returns
  where they lie and when next tick is due (local: plain accumulator); fresh view of memory.buffer after every call;
  calls per tick within R1-BND-REC1 mark 2's four (glue calls included). Input post (:318-329): sequence number, move
  input, held bitmask, press/release edge counters as running totals (ADR 0001's fields), cursor ray + view time.
- PM-0002-53 (:1327): "a planted `u16` counter that overflows with `+` panics in the dev and the release test builds,
  native and Wasm (4 of 4); the same counter with `wrapping_add` gives the same value in all four; the checks' cost in sim
  step time reported with the tick bench's method"; "a committed test in `sim` run in the four builds". NOTE: Wasm panic =
  abort/trap; a test harness cannot catch_unwind under panic=abort in wasm32 -> the Wasm half needs the trap observed from
  outside (Node runs the built Wasm and expects a WebAssembly.RuntimeError + the hook's message). B4(d) design.
- Code read (Rust): sim/world.rs (World{tick u32, seed [u8;32]}, authority_step checked_add, write_state, state_hash
  fnv1a_64); sim/lib.rs has `overflow_checks_are_on` (u8 + 1, should_panic, native only) and protocol/lib.rs the same.
  NO random stream exists yet (world holds only the seed). sim-wasm/lib.rs 259: `Sim{world, writer, schedule,
  newest_input}`; plain-Rust `Sim::create` + `CreateError`; bindgen surface = `tick_hz()`, `Sim::new(seed,mode,pad,now)`,
  `block_ptr`, `block_len`, `input(sequence)`, `newest_input()` (EA-F10: test-only getter on the surface), `step()->f64`,
  `rebase(now)->f64`; tests mix the endpoint's checks into the bindings crate (EA-F9). client-endpoint: block.rs
  (BlockWriter, header only, MAX_PAD_BYTES 1 MiB), layout.rs (BLOCK_LAYOUT = header{tick u32 @0, layout_hash u32 @4};
  ElementType U8/U32 only; LAYOUT_HASH FNV-1a 32), schedule.rs (LocalSchedule f64 anchor; float maths here is fine:
  part 7 is "inside the sim"). protocol/messages.rs: Join, JoinAccepted, Input{acks, commands}, SnapshotAck, Command{tick,
  sequence u16, move_x/y i8, buttons u16, presses [u8;16], aim, view_time}, AimPoint, ViewTime, Snapshot header. room
  and devtools are stubs. Cargo: overflow-checks=true in release for all members; wasm-bindgen =0.2.130; ts-rs dev-dep.
- Code read (sim-worker): worker.ts 57 (init(), crypto seed, `new Sim(seed, MODE_LOCAL, pad, now)`, SimHost; errors from
  start() posted as {type:"error"}); host.ts 159 (HostSim = input(sequence)/step()/rebase()/block_ptr()/block_len();
  pacer step = `sim.input(newestSequence); sim.step()`; #wake catches a throw (trap) -> #failed + error post; afterTick
  copies block into pooled buffer, posts {type:"block", buffer, doneAt}); messages.ts 91 (PageToWorker start/input{sequence,
  returned}/visibility/report-request + `isPageToWorker`; WorkerToPage ready/block/error/report with NO parser on the
  page side = EA-F8); pacing.ts 127 (MAX_TICKS_PER_WAKE 7); read-block.ts (fresh view per copy); block-pool.ts;
  index.ts exports types + stats only. boundary/sim-wasm.test.mjs 138: built Wasm in Node via Vitest `initSync` (the
  Wasm-side route for vectors), includes a glue test "each per-tick call reaches exactly one export" and uses
  `newest_input()` twice (EA-F10's test-only member). package.json test = `vitest run --dir src` (boundary/ excluded).
- CI after PR #62 (ci-edit.diff): toolchain job builds the page (--production, dev) with the pinned wasm-bindgen
  program (ONLY `wasm-bindgen` is extracted from the archive; `wasm-bindgen-test-runner` is in the archive but refused by
  the "more than the one program came out" check), runs `vitest run --root packages/sim-worker --dir
  packages/sim-worker/boundary` (=> any new *.test.mjs in boundary/ runs in CI with NO CI edit, if what it loads is
  built by a command CI already runs), tsc on worker/page/specs, Playwright in Chrome (/opt/google/chrome/chrome).
- DESIGN NOTE (Wasm route for rule tests): ADR 0004 PM-0004-22 names "unit vectors in `cargo test` and
  wasm-bindgen-test (R5-TEST-REC1)". wasm-bindgen-test = NEW dependency crates + a CI edit to extract the runner + a
  runner env var (no root .cargo/config.toml allowed while the netcode proof rules hold). Cheaper route W2: one
  test-only cdylib crate using only the already-pinned `wasm-bindgen`, built by build.mjs into a gitignored folder,
  loaded by boundary/*.test.mjs in Node (and by one Playwright spec in Chromium). Test-only exports never touch the
  shipped `sim-wasm` (WB-HEALTH (a), EA-F10). Needs: a new workspace member (manifest -> security-auditor's answer on
  the exact content; no new third-party crate), a departure from ADR 0002 part 3's crate list (B6), and either one CI
  line or the dev build building it.
- CI today (toolchain.yml :84-91): `cargo test --workspace --locked` (=> scan test, native vectors, L1 tests, native
  PM-0002-53 dev half run with NO CI edit), `cargo test --release -p sim -p protocol --locked` (=> PM-0002-53 native
  release half, if the test is in sim), `cargo build --target wasm32 -p sim -p protocol -p room -p client-endpoint -p
  sim-wasm`, `npm test --workspaces`. A NEW crate is in `--workspace` automatically but in neither the release-test
  nor the wasm32 build line (each would be a CI edit if wanted).
- Code read (client): app/boot.ts 128 (boot order; WorkerLink; `stop(problem)` terminates the Worker + notice; frame loop
  `sequence=(sequence+1)>>>0; link.frame(sequence); picture.draw(...)`; devtools-only overlay + `__shambleReport`);
  app/worker-link.ts 144 (`onMessage(data: unknown)` = an ad-hoc parser of WorkerToPage inline, casting `stats` with
  `as WorkerStats` unchecked = EA-F8's target; `failure` string = "match over" state on the page side; `frame(sequence)`
  posts {type:"input", sequence, returned}); error-hook.ts (bounded one-line `note`); flags.ts (devtools-only pad/run);
  overlay/overlay.ts 46 (pre + Copy report button, textContent); overlay/report.ts 103 (ReportData, readoutLines,
  formatReport; "fps" toFixed; frames line `frames: ${fps.toFixed(1)} a second · frame time ...`); picture.ts 68 = the
  interface file (PictureFrame{newest, previous, timeMs}; "Events join when the first change-set has some").
- Build files read: build.mjs 141 (buildWasm: cargo build --release wasm32 -p sim-wasm into root target/, checked
  wasm-bindgen --target web into packages/sim-worker/pkg; refusedBuildSettings; buildPage via Vite; flags --wasm-only,
  --production, --out-dir); build-checks.mjs 180 (bundleProblems: `page = scripts.filter(name !~ /(^|\/)worker-[^/]*\.js$/)`
  = EA-N6/VF-L7's name prefix; DEV_MARKERS list; cspProblems). vite.config.mjs: worker.format "es", no manifest, no
  worker entryFileNames (Vite's default asset naming gives `assets/worker-<hash>.js` from the file name `worker.ts`).
- STATUS_ARCHIVE :1084-1121 read (the cs3 list, verbatim ids). EA-N6 :1101 "finds the Worker's chunk by a name prefix";
  VF-L7 :1102-1104 "scan every script but the Worker's entry chunk, named by the build itself". EA-F10 :1094-1096 also
  names TS test-only members: `Ring.clear`, `BlockPool.free`, the `percentile` re-export. EA-F6/EA-F12 (:1133-1135) are
  for the drafts' next revision, NOT cs3. VF-L5 is in the context's list though the brief omits it (with RR-N4).
- My cs2 findings re-read (git show 929b146:.checkpoints/cs2-first-page/engine-architect-review1.md :71-114 and
  review2.md :18-31). F8: worker-link.ts parses WorkerToPage ad hoc, `stats as WorkerStats` unchecked -> fix
  `isWorkerToPage` beside `isPageToWorker` + a switch on the narrowed type. F9: Sim composes World+BlockWriter+
  LocalSchedule+newest_input with checks in Sim::create; "the composition belongs in client-endpoint (writer, schedule,
  newest input) and room (the authority's World), with sim-wasm a wrapper". F10: newest_input test-only; TS dead code
  Ring.clear (stats.ts), BlockPool.free, `percentile` re-export (index.ts:4). F13 four nits: (1) WorkerLink's state public
  + writable (ready, blockBytes, tickHz, blocks, tick, posts, failure, stats, lastPause, pauses); (2) the schedule's
  anchor has two writers (Sim::new(now_ms) then Pacer ctor `rebase(now())`); (3) folder cycle by types app/boot.ts ->
  overlay/report.ts -> app/worker-link.ts (PauseRecord); (4) sim-worker declares @shamble/types used only by the
  boundary test. N3: "match over" has two owners (boot.ts stop() vs WorkerLink.failure); the Worker `error` event path
  (e.g. a trap in `rebase` on resume, outside host.ts's try) never sets failure, so frame()/requestReport() keep
  posting. Fix: one owner (`WorkerLink.end(reason)` called by stop(), or the link owns stop with terminate passed in).
  N5: `webgpuOffered` probed in boot.ts -> a fallback reason in PictureInfo when render/ changes hands (cs5). N6: name
  the Worker's chunk from the build (the page's `new URL("worker-...js")` reference or a Vite manifest entry).
  F14 (rebase counted) is already fixed in the boundary test (calls expect rebase: 1).
- NOTE: host.ts's try/catch wraps only `#pacer.wake`; `resume()` (visibility) calls `sim.rebase` OUTSIDE it (host.ts:97
  -> pacing.ts:123): a trap there is an uncaught Worker error -> boot.ts's `error` listener -> stop(); the link is not
  told (N3). The panic hook's message must reach the page on BOTH paths (B2 design).
- test-engineer-followups.md block 5 (SA-TREE, :207-244): three pure functions in build-checks.mjs (`runtimeCrates`,
  `overriddenCrates`, `missingOverflowOverrides`) fed by `cargo tree --locked -p sim-wasm --target wasm32-unknown-unknown
  -e normal,no-proc-macro --prefix none`, called by buildWasm() before cargo build; unit test with fixture + plants;
  about +52 lines; "Recommendation: with change-set 3. It adds the random-number crate and so its override in the same
  change" (premise: a library; with our own ChaCha8 no runtime crate joins -> the plant uses fixtures only). Block 6
  (RR-F5, :249-268): one builder path in the Wasm (wasm-bindgen's externref.rs panic location under <HOME>/.cargo/
  registry/src/...); fix = "a remap from the repository's own settings" (env RUSTFLAGS refused), rust-reviewer's fix;
  "Change-set 3's size table must name the machine it was read on (or wait for the remap)". SA-DIST: `dist-*/` line.
- rust-reviewer-cs3-rng.md read whole (97 lines, DONE). Recommends (c) own ChaCha8 in `sim` (~100-150 lines + ~200
  test lines; 16-word buffer gives the same word sequence; next_u64 = two consecutive words low first). Vectors (its
  block 5): a committed generator SCRIPT (Python or Node, independent of both crates), self-checked against the 8
  published 8-round/256-bit vectors of draft-strombergson-chacha-test-vectors-01 (+12/20-round free), writes per stream:
  10,000 next_u32 from word 0; 10,000 next_u64 from word 1; one save/restore at a mid-block word (next 1,000 words);
  one window across the 2^32-block carry (96 words from block 2^32-1); fixed public test seed (draft TC8 key), each
  stream id. Format raw LE binary, 40,000 B per stream u32 view (+80,000 u64 view); 5 streams x 120,000 B ~ 600 KB, or
  ~200 KB u32-only; text manifest with SHA-256 per section. Its Wasm route: "one exported call in sim-wasm (seed,
  stream id, start word, count -> the words)" + a boundary test. CONFLICT with EA-F10 / WB-HEALTH (a): a test-only
  export on the shipped surface (I design a test-only Wasm module instead; B2/B4). If a library is picked: (b)
  chacha20 =0.10.2 features ["rng"], set_stream before any draw and before set_word_pos, never get_block_pos, native
  vectors also with --cfg chacha20_backend="soft" and "sse2" (needs RUSTFLAGS -> refused by build.mjs's env rule:
  a test-only exception or cargo --config), 3 overflow overrides, security-auditor first; (a) 4 crates, last choice.
- rr-chacha-ref.py exists in this folder (independent ChaCha, matches all 48 draft vectors): the seed of the generator
  script (moves to a committed tools path in cs3; it is Python: CI runs Python? -> the vector FILE is committed, the
  script is run by hand/by the test only as a regeneration check; B4).
- netcode-architect-cs3.md read (B1 done at 78 lines; B2 design next). Seams with mine: (1) its F-ECHO: authority holds
  the slot's applied input (buttons + press totals) in hashed state, snapshot carries it in a minimal owner section ->
  the block's echo section (layout table + writer = client-endpoint, my boundary); (2) its F-PRESS: the page counts
  presses per button cumulatively (keydown, not repeat) and posts running totals; the endpoint paces at most one edge
  per button per tick (ADR 0001 :235) -> the page's post shape is mine (B3), the pacing rule is its; (3) the step order
  (ADR 0002 part 6 :402-407, its) vs where the composition lives (mine, B2); (4) NA-N1 step gets host time, NA-N7 Pacer
  non-finite due time (sim-worker files I also touch for EA-F8/EA-N3: coordinate one edit per file); (5) its F-P0/F-B1
  on PM-0002-49 (not mine). It reads the lints at ADR 0002 :462-474 (protocol decoders deny unwrap/expect/indexing).
- browser-reports-reading.md read. FPS-LINE (:68-70): readout fps = 1000 / median frame gap (harness/frame-times.ts:25-28
  `fps()`); Firefox showed 239.2 where the run's average was 179.9 -> "The line should be a count over time". Fix:
  fps = held deltas' count / their sum x 1000 (frames over the time they took, the ring's window; a `Ring.sum()` or
  `mean()`), ~6 lines + test. NEG-ZERO (:71-72): "deliveryP50: -0.00 ms" from report.ts `ms()` = toFixed(2) of a tiny
  negative (two rounded clocks). Fix: a value that rounds to zero prints "0.00" (no sign); ~2 lines + test. Both in
  files cs3 edits anyway (lane 1: overlay + CPU harness stay lane 1's; only the GPU half of the harness moves at cs5)
  -> IN. BR-GAP: his words: README asks window in front during first run + one step covered on purpose (README text, not
  code). FF-WAKES/BR-LATE: readings for the pacing marks (no code). ADAPTER/TQ-FF/NOT-SHOWN: not code for cs3.
- ADR 0004 part 20 read (:562-590): Wasm "installs a small panic hook of its own (`std::panic::set_hook`, no extra
  crate) that writes the message and location into a fixed buffer in linear memory before the abort, and the Worker's
  host reads it after the trap and attaches it to the report"; "the Wasm build is S20's to check"; local-play recovery
  after a trap "is not researched (OQ-0004-7)". => cs3: hook installed by `#[wasm_bindgen(start)]` (no host call), a
  static fixed buffer (u32 length + N bytes) written by a non-allocating truncating fmt::Write; ONE new free export
  `panic_buffer() -> *const u8` read once at start; after a trap the host NEVER calls an export again (a lock or state
  left mid-panic), it reads the bytes through a fresh view. No `static mut` (unsafe_code deny): a `Mutex<[u8;N]>`-style
  static (no_threads Mutex on wasm32) or atomics; rust-reviewer to pick the unsafe-free form.
- No rust-src in the pinned toolchain (sysroot has no lib/rustlib/src): the scan's complete list of std f32/f64
  methods must be read from doc.rust-lang.org/1.99.1 (std's f32 and f64 pages) by the build session and pinned in the
  test with the URL; I state no count.
- DESIGN NOTE (vectors home): inputs read at RUN TIME (std::fs in native tests, passed in from JS in Wasm), never
  include_bytes!, so the compiler cannot fold the operations at build time with its own float code.
