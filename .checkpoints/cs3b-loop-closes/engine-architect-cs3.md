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
- cs2 plan's folder table (git show 929b146:.checkpoints/cs2-first-page/plan.md :95-123): `src/input/` = "input capture
  into the newest input", lane 1, created in cs3; `src/overlay/` = HTML readout and counters (lane 1); `src/dev/` = cs5.
  => key capture in packages/client/src/input/, key diagram + counters in src/overlay/ (dev builds only).
- generator read (layout_ts.rs: one table -> packages/types/src/generated/block/layout.ts; types index.ts exports
  BLOCK_LAYOUT, LAYOUT_HASH, checkLayoutHash). A second generated table would need the generator parameterised.
- B6-2 update: "the table of build options" is most likely ADR 0002 part 4's Wasm size readings across build options
  (opt-level "s" vs "z", wasm-opt on/off: :270-274); rust-reviewer's note says "beside the build-option table". wasm-opt
  is a new tool (not installed; TOOL-SOURCE applies) -> its column cannot be read without his word.
- Block 1 DONE (all named sources read). Next: Block 2 (boundaries), written below as its own section.

## Block 2. The boundaries after change-set 3 (DONE)

### 2.1 Who owns each piece of state (EA-F9)
| State | Today | After cs3 | File |
|---|---|---|---|
| the World (tick, seed; streams when a phase first draws) | `sim_wasm::Sim.world` | the authority: `room::Room` (netcode-architect designs Room) | crates/room/src/lib.rs (+ room.rs) |
| block writer, local schedule, the newest post (held bits, 16 press totals), endpoint counters, join state | `Sim.writer`, `.schedule`, `.newest_input` | `client_endpoint::ClientEndpoint` (new) | crates/client-endpoint/src/endpoint.rs |
| the loopback's two byte queues | none | `protocol` (its loopback backend; netcode) | crates/protocol/src/ |
| the panic text buffer (one per module instance) | none | `sim-wasm` (Wasm build) | crates/sim-wasm/src/panic.rs |
| the composition: three fields and the step in ADR 0002 part 6's order (:402-407) | logic + state in `Sim` | `Sim` = wrapper: `{ endpoint, room, link }`, about 25 lines; the order itself is netcode's | crates/sim-wasm/src/lib.rs |
| host-input checks (seed length, mode, pad cap) + `CreateError` | `Sim::create` | stay in sim-wasm as the wrapper's boundary checks (pad cap stays in `BlockWriter::new`) | crates/sim-wasm/src/lib.rs |
Composition home, options: (A, recommended) the wrapper stays in sim-wasm as a pub plain-Rust `LocalMatch<T: Transport>`
(no bindgen attributes; `Sim` wraps `LocalMatch<Loopback>`); devtools uses `LocalMatch<L1>` natively through sim-wasm's
rlib (crate-type already ["cdylib","rlib"]), so L1's tests and the recording run the Worker's own step order, not a
copy. Edge devtools -> sim-wasm is allowed (ADR 0002 :194 "anything"); devtools needs wasm-bindgen anyway for its test
module (Block 4). (B) a new crate `local-match` (deps client-endpoint, room, protocol, sim): cleaner wording, but a
crate ADR 0002 part 3 does not list (his go). (C) the composition in room or client-endpoint: refused, they may not
depend on each other (ADR 0002 :189-190). Two-instance seam: the loopback carries ENCODED bytes (the real messages), so
lane 2's two-instance patch splits LocalMatch into an authority type and an endpoint type and moves the order into the
host (ADR 0002 :407 "with two instances ... the host makes its calls in the same order") without touching room or
client-endpoint.

### 2.2 The wasm-bindgen surface after cs3 (WB-HEALTH (a): "one type, plain numbers on every tick")
| Export | Today | After cs3 | Called |
|---|---|---|---|
| class `Sim` | yes | yes: still the one type | |
| `Sim::new(seed, mode, pad_bytes, now_ms)` | yes | `new(seed, mode, pad_bytes)`: `now_ms` dropped (EA-F13: the anchor's two writers; the Pacer's first `rebase(now)` is the one writer) | once |
| `block_ptr()`, `block_len()` | yes | unchanged | once |
| `input(sequence: u32)` | yes | `input(post, held, presses_0_3, presses_4_7, presses_8_11, presses_12_15)`, all u32: post number, held bitmask (16 bits used), 16 press totals (u8, wrapping) packed four per word, counter k in byte k%4 of word k/4 (LE) | per tick |
| `newest_input()` | tests only | DROPPED (EA-F10); tests read the echo in the block | - |
| `step() -> f64` | yes | `step(now_ms) -> f64` if netcode adopts NA-N1 (ADR 0002 :333-335 host's time) | per tick |
| `rebase(now_ms) -> f64` | yes | unchanged | start, pause, drop |
| `tick_hz()` | yes | unchanged | once |
| `panic_buffer() -> *const u8` | no | NEW free fn (the hook is per module instance, not per Sim) | once at start |
| `__wbindgen_start` via `#[wasm_bindgen(start)]` | no | NEW: installs the hook at instantiation; the glue calls it inside init(), the host never does | at init |
Per tick: 2 Wasm calls (input, step), 0 glue allocations (no string/array/object crosses); R1-BND-REC1 mark 2's four
and PM-0002-3's count hold. Why six plain numbers and not an input area in linear memory: 18 bytes of post now; no new
export, no second layout table, no second generated file, the glue's simplest form (WB-HEALTH (a)). The input area
(the block's mirror, generated layout + hash) is the right form when variable-length data crosses: online datagrams
with arrival times (ADR 0002 :331-334), Slice 3. Not built now. Packing slip guard: a boundary test presses button
k = 0..15 once each and reads k's echo.

### 2.3 Worker <-> page messages (EA-F8)
- packages/sim-worker/src/messages.ts: `PageToWorker.input` = `{type:"input", post, held, p0, p1, p2, p3, returned}`
  (flat numbers; the page packs its 16 counters, no inner array per frame); `isPageToWorker` checks post/p0..p3 as
  counts and held <= 0xFFFF. NEW `isWorkerToPage(value): value is WorkerToPage` and `isWorkerStats` beside it (every
  WorkerStats field a finite number or boolean), exported through index.ts; `WorkerLink.onMessage` switches on the
  narrowed type with an exhaustive `never` default. WorkerToPage keeps four kinds; the trap's text rides `error.message`.

### 2.4 "The match is over": one owner (EA-N3)
- Worker: SimHost guards EVERY Wasm call after construction through one private `#call(fn)` (today only `#pacer.wake`
  is guarded; `resume()` -> `rebase` at host.ts:97 / pacing.ts:123 is not): on a throw it sets `#failed`, clears the
  timer, reads the panic text through a fresh view (never another export call), posts ONE error, and makes no Wasm call
  again. worker.ts `start()` (init + constructor) uses the same reader. New file packages/sim-worker/src/panic-message.ts
  (`readPanicMessage(memory, ptr): string`, about 20 lines).
- Page: `WorkerLink` owns the end: constructor takes `terminate: () => void`; `end(reason): boolean` sets `failure`
  once, terminates once, returns true on the first call; onMessage's failures call it; boot.ts's Worker `error`
  listener calls `link.end(...)` and shows the notice only when it returns true; frame()/requestReport() already stop on
  `failure`. boot.ts keeps the notice and the error count; the decision lives in the link.
- What the page shows in every build: the notice "The match stopped: the simulation stopped: panicked at
  crates/<file>:<line>:<col>: <message>" (textContent); dev readout: errors 1 (last: the same, one line, 300 chars).

### 2.5 The glue check by the build's own chunk (EA-N6, VF-L7)
- build.mjs `buildPage`: an inline Vite worker plugin (`worker.plugins`, merged with vite.config.mjs) records, in
  `generateBundle`, the file name of the one entry chunk whose `facadeModuleId` is packages/sim-worker/src/worker.ts;
  the build refuses when it finds none or more than one.
- build-checks.mjs `bundleProblems(files, production, workerEntry)`: page = every .js but exactly `workerEntry`; the
  glue and the name WebAssembly are refused everywhere else, a lazily loaded page module named worker-... included
  (VF-L7's plant becomes a test case in build-checks.test.mjs).
- Fallback if the pinned Vite does not merge `worker.plugins` as read: disjoint names set in vite.config.mjs (worker
  `assets/sim-worker-[hash].js`, page chunks `assets/page-[name]-[hash].js`), the check reading the same exported
  constant. Not verified which works.

### 2.6 The panic hook
- crates/sim-wasm/src/panic.rs (new): `#[wasm_bindgen(start)]` -> `std::panic::set_hook` (no extra crate, ADR 0004
  :578-579). One static buffer of 1,024 bytes: bytes 0-3 the text's length (u32 LE, 0 = no panic), 4-7 the text's
  capacity (1,016), 8.. the UTF-8 text "panicked at <file>:<line>:<col>:\n<message>", cut at a char boundary, written
  by a `core::fmt::Write` that never allocates (a panic may come from a failed allocation). No `unsafe`
  (unsafe_code = "deny"): rust-reviewer picks the form (a lock-guarded static array or atomics). Self-describing
  header, so TypeScript copies no capacity constant (my cs2 F5's lesson).
- Read: `panic_buffer()` once at start; after a trap, a fresh `Uint8Array(memory.buffer, ptr, 8 + len)`, `TextDecoder`
  non-fatal; no export call after a trap (a lock or a half-written state may be left).
- Wasm bytes: unknown; measured in cs3 from the build's own size table (raw, gzip, brotli) with and without the hook, on
  one named machine (RR-F5).
- Tests: native unit tests of the writer (cut, char boundary, length word) in `cargo test -p sim-wasm`; in Wasm through
  the devtools test module (Block 4: a planted overflow traps, the buffer holds "attempt to add with overflow" and a
  location); host.test.ts: a stand-in throwing a RuntimeError with a fake memory holding a text -> one error post.

### 2.7 The block after cs3 (seam with netcode's F-ECHO)
- client-endpoint layout.rs: two sections join the header (tick @0, layout_hash @4 never move): `echo` (1 row: held
  u32, presses_0_3..presses_12_15 u32: the authority's applied input for the local slot, decoded from the snapshot,
  same packing as the post) and `link` (1 row: sent u32, received u32, refused u32: the endpoint's message counters).
  Field names final with netcode's snapshot owner section. LAYOUT_HASH changes; layout.ts regenerated; the page's
  startup check covers drift. Embryo of ADR 0002 :290-292's owner-only section.

### 2.8 Follow-ups: in or out of cs3
| Id | In/out | Lines (est.) | Why |
|---|---|---|---|
| EA-F8 | in | +95 (45 code, 50 test) | cs3 changes the post and the readout's inputs; a parser before more kinds come |
| EA-F9 | in | move ~60 out of sim-wasm; ~40 into client-endpoint (rest is netcode's endpoint) | the card's endpoint and room land here |
| EA-F10 | in | -25 | the input reaches a command (my cs2 condition); Ring.clear, BlockPool.free, the percentile re-export go |
| EA-F13 | in, all 4 | +30 | (1) WorkerLink fields -> getters; (2) one writer of the anchor; (3) report.ts states its own pause shape (no app import); (4) @shamble/types -> devDependencies of sim-worker (rides cs3's manifest hold) |
| EA-N3 | in | +50 (20 code, 30 test) | cs3 adds the trap path; its text must reach the link on every path |
| EA-N5 | out -> cs5 | 0 | render/ fact; my cs2 words: "when render/ changes hands" |
| EA-N6 + VF-L7 | in | +50 (20 build, 30 test) | cs3 builds a second Wasm module; the check must be exact first |
| VF-L4 | in | +10 test | serve.test.mjs is edited for FAVICON |
| GE-N1, GE-N4, GE-N5, GE-N6 | out -> cs5 | 0 | render/ items; lane 1 closes them before render/ changes hands (cs5), with EA-N5 |
| TE-NT3 | in | +25 test | trap texts carry newlines: the cleaning now matters |
| TE-NT4 | in | +15 test | host.ts is rewritten |
| TE-NT5 | in | +5 | the loop spec reads the same reports |
| TE-NT7 + PA-N2 | in | +20 | cs3 adds per-frame code (post with counters, key diagram): the allocation test must cover the listener and start after warm-up |
| PA-L2 | out -> cs5 | 0 | blocks grow at 12/48 soldiers; the pool is sized with the bench |
| TE-L | in | +50 test | its items sit in files cs3 edits (serve tests, build-check plants, one-Worker check, scratch-buffer test) |
| SA-ENV | in | +6 | new specs may read env |
| FAVICON | in | +3 code, +8 test | serve.mjs answers /favicon.ico with 204 (no icon, no look decision); a console 404 would trip G12 later |
| SA-TREE | in | +52 (test-engineer's estimate) | lands with or without an RNG library; with own ChaCha8 its plants use fixtures only |
| RR-F5 | in, if rust-reviewer confirms the mechanism | +10, +10 test | the size table must not depend on the builder's home; candidate: `cargo build --config build.rustflags=[--remap-path-prefix=<cargo home>/registry/src=/cargo/registry]` from build.mjs (argv, not env); Not verified |
| __pycache__/ in .gitignore | in | +1 | his words, 2026-11-19 14:28:25 UTC |
| FPS-LINE, NEG-ZERO | in | +25 | Block 3 |
| BR-GAP README steps | in (zip README text) | docs | his words, 16:08:04 UTC |
| VF-L5, RR-N4, TE-NT6, E2E-TYPES, SA-DIST | out (taken by #62) | 0 | CI-WASM's own group |
| NA-N1..N7 | netcode's part | - | |
- B2 DONE. Next: Block 3 (the page, alive).

## Block 3. The page, alive (DONE)

### 3.1 Owners and folders (Part 15.2 :90-95; cs2 plan's folder table)
All lane 1: `packages/client/src/input/` (new: key capture into the newest post), `src/overlay/` (the key diagram and
the counters, development builds only, behind `__SHAMBLE_DEVTOOLS__` like the readout), `src/harness/frame-times.ts`
(FPS-LINE; the CPU half stays lane 1's after cs5), `src/app/` (wiring), `packages/e2e/tests/skeleton/` (the loop spec).
`src/picture.ts` (the interface file) does NOT change: the diagram is HTML, so lane 2's picture never learns of it.

### 3.2 The input path, key to command (nothing else changes on the page side: one post per drawn frame)
1. `src/input/keyboard.ts` (new, `KeyboardInput`): keydown/keyup on window by `KeyboardEvent.code` (layout-free
   physical keys): KeyW 0, KeyA 1, KeyS 2, KeyD 3, Space 4, ShiftLeft/ShiftRight 5 (the six keys change-set 4 walks
   with; the other 10 of the 16 bits enter with their systems, as messages.rs:83 says). keydown, not `repeat`, bit not
   yet held: set the held bit and add 1 (wrapping at 256) to that button's press total; keyup clears the bit; window
   blur and a hidden tab clear every held bit (NA-N2's page half); press totals never go back. Space's default (scroll)
   is prevented. State: one 16-bit mask and a 16-byte array viewed as four LE words; no allocation per key or frame
   beyond the browser's own event objects.
2. `app/boot.ts` frame loop: `link.frame(keyboard)` once per drawn frame; WorkerLink posts `{type:"input", post, held,
   p0, p1, p2, p3, returned}`; `post` is the page's per-frame post number (NA-N5's rename of `sequence`).
3. Worker host: keeps the newest post's six numbers (a superseded post loses no press: running totals, ADR 0002
   :321-322); per tick `sim.input(post, held, p0, p1, p2, p3)` then `sim.step(...)`.
4. Rust (netcode's order): the endpoint builds T's Command (held -> buttons; totals -> presses, at most one edge per
   button per tick, its F-PRESS), the loopback, the room runs T and records the applied input in hashed state (its
   F-ECHO), the snapshot's owner part carries it back, the endpoint decodes it and writes the block's `echo` section
   and its `link` section (sent, received, refused, and `post_used`: the post number tick T's command was built from,
   so the page can read PM-0002-64's "every tick on the newest post" in Chromium).
5. Page: the diagram reads the newest block's `echo` at most once per frame, only when a new block arrived (30 Hz),
   toggling a class on six elements made once (no per-frame allocation; textContent only).
Behaviour of the diagram (plain, development-only; not the look): a key is lit while the ECHOED held bit is set, and
for 150 ms after each echoed press, so a tap shorter than a frame still shows. Its pure part (echo -> six booleans)
is a function with a Vitest test; the DOM wiring is covered by the loop spec in Chromium (no DOM library is added).

### 3.3 What the readout and "Copy report" gain
- Readout lines: `messages: sent N · received N · refused 0` (block `link`); `presses: N made · N echoed · 0 lost`,
  where made = the page's own count per button (u32), echoed = the sum of wrap-aware deltas of the echoed u8 totals
  per block, and lost = made - echoed counted only once the newest press is 0.5 s old (until then "(counting)").
- Report rows: messages.sent, messages.received, messages.refused, presses.made, presses.echoed, presses.lost,
  keys.bound ("W A S D Space Shift"), postUsed, and `fps` redefined (below). The tally is a small pure module
  `src/app/press-tally.ts` with its own test. DEV_MARKERS (build-checks.mjs) gains one marker of the diagram, so the
  production scan proves the diagram is cut out.
- FPS-LINE (in): `FrameTimes.fps()` = frames over the time they took: held deltas' count / their sum x 1000, through a
  new `Ring.mean()` (computed at readout time, 4 times a second); test: deltas 4, 4, 4, 4, 20 ms give 138.9, where the
  median rule gives 250.
- NEG-ZERO (in): one helper prints a value that rounds to zero without a sign ("0.00"), used by `ms()` and the report's
  `toFixed(3)` path; test: -0.001 prints "0.00 ms".
- Both belong here: cs3 edits these files for its own counters, and his cs3 check reads these lines.

### 3.4 What he sees in his browser (five plain lines)
1. The same grid page, now with a small box of six keys (W, A, S, D, Space, Shift) beside the readout.
2. Pressing a key lights its box a moment later: the light comes back from the local server through the real messages.
3. A quick tap flashes its box; holding keeps it lit; letting go darkens it.
4. The readout counts ticks (30 a second), messages sent and received (climbing with the ticks) and presses made,
   echoed and lost (lost stays 0).
5. The frame-rate line now equals frames counted over time (Firefox included), and no reading shows "-0.00".
(The zip's README keeps the Brave and Firefox steps and adds his BR-GAP steps: the window in front and uncovered for
the first run, then one step with the window covered on purpose.)
- B3 DONE. Next: Block 4 (the rule tests).

## Block 4. The rule tests (DONE)

### 4.0 One Wasm route for every rule test: the devtools test module (never shipped)
- `crates/devtools`: crate-type ["cdylib","rlib"]; deps sim, room, protocol, client-endpoint, sim-wasm (for
  `LocalMatch` and the real panic hook), wasm-bindgen (workspace; already in Cargo.lock): no third-party crate joins.
  `src/rules/` = pure runners (no I/O, Wasm-buildable); `src/wasm.rs` (about 60 lines) = the exports:
  `f32_vectors(inputs: &[u8]) -> String`, `chacha8_words(seed: &[u8], stream: u64, start_word: u64, count: u32) ->
  Vec<u32>`, `plant_u16_add(a: u16, b: u16) -> u16`, `plant_u16_wrapping_add(a, b) -> u16`, and netcode's
  `replay(recording: &[u8]) -> Vec<u64>` (per-tick hashes); `panic_buffer()` comes linked from sim-wasm.
- Built by `node packages/client/scripts/build.mjs --devtools-wasm` (new mode): cargo for wasm32 `-p devtools` in release
  AND dev profile, the same checks as the player's build (the tool's checksum and version, environment overrides
  refused), wasm-bindgen --target web into `packages/sim-worker/pkg-devtools/{release,dev}/` (gitignored, +1 line);
  never imported by the page, never in dist.
- Run in Node by `packages/sim-worker/boundary/rules.test.mjs` (CI's boundary step after #62 picks it up by folder), and
  in Chromium by `packages/e2e/tests/rules/wasm-rules.spec.ts`: Playwright routes a fake origin (http://rules.invalid/)
  to the module's files and the vector files and runs the three checks in the page's engine (CI's Playwright step
  picks it up). Both need the module built first: CI edit E2 (Block 5).
- Why not exports in sim-wasm (rust-reviewer's suggested route): WB-HEALTH (a) and EA-F10 keep test members out of the
  player's Wasm. Why not wasm-bindgen-test (the drafts' named tool, R5-TEST-REC1, PM-0004-22): new crates, a CI edit
  to unpack `wasm-bindgen-test-runner` (today's step refuses every member but `wasm-bindgen`), and a runner variable
  (no root .cargo/config.toml while the netcode proof test's rules hold, build plan :955-957). Question WASM-TEST-ROUTE.
- Reading to state in the plan: the Wasm halves run the same crates compiled with the player's target, profile and
  flags, inside another module; the player's file itself exposes no hash, vector or plant by design (state_hash.rs:2-3:
  the hash covers the seed and never leaves the Worker).

### 4.1 (a) Every allow-listed float operation and sqrt, bit for bit
- Ops (f32 only: part 7 :261 "f32 for every real-valued quantity"; f64 is in no sim code): + - * /, negation; < <= > >=
  == != and total_cmp; sqrt, floor, ceil, round, trunc, fract; mul_add; abs, copysign; `as` casts f32 -> i8 u8 i16 u16
  i32 u32 i64 u64, i32 u32 i64 u64 -> f32, f32 <-> f64; is_finite, is_nan (to_bits/from_bits are the harness's own
  I/O). About 37 operations.
- Inputs: `crates/devtools/vectors/f32-inputs.bin`, 10,000 cases of (a, b, c) bit patterns, LE = 120,000 bytes: an edge
  table first (all ordered pairs of about 26 values: +-0, +-smallest and largest subnormal, +-smallest normal, +-0.5,
  +-1, +-1.5, +-2.5, values just below 0.5, around 2^23, 2^24, 2^31, 2^32 for the casts, +-max, +-inf), then seeded
  values (uniform over non-NaN bit patterns, and uniform in game magnitudes). >= 10,000 per op meets PM-0004-22's count
  for sqrt. NaN never as an input (part 7 :285); a NaN result is normalised to 0x7FC00000 before comparison (Wasm's NaN
  payload and sign are not deterministic; what must agree is which cases give NaN). Integer-to-float casts read a's
  (or a,b's) bits as the integer.
- Expected: a correctly rounded reference from an independent generator, `crates/devtools/vectors/make_f32_vectors.py`
  (Python 3 standard library: struct, fractions.Fraction, math.isqrt; exact rational result rounded once to f32
  nearest-even; round = half away from zero; Rust's saturating casts; Rust's total_cmp order), self-checked on
  hand-known answers. So "sqrt equals a correctly rounded reference" (PM-0004-22) holds by construction, and native
  and Wasm must each equal the reference, hence each other. Why it matters: on baseline x86-64 floor/ceil/trunc/round
  and mul_add are library calls and in Wasm instructions or Rust's own libm port (my reading of how LLVM lowers them;
  to be confirmed by rust-reviewer), so two implementations meet here.
- Committed form (recommended): `f32-expected.txt`, one line per op (name, count, width, NaN count, FNV-1a 64 of the
  normalised outputs): about 40 lines, about 3 KB; the generator's `--full` mode writes every output for a diagnosis
  (the first differing index). FNV-1a 64 changes on any single changed byte (each step is a bijection of the state).
  The alternative, every output committed: about 1,230,000 bytes (sum of the widths over 10,000 cases).
- Runner: `crates/devtools/src/rules/f32_ops.rs`; inputs read at run time (std::fs natively; passed from JS in Wasm),
  never include_bytes!, so no compiler folds the ops with its own float code.
- Native: `crates/devtools/tests/f32_vectors.rs` (dev in today's `cargo test --workspace`; release needs E1).

### 4.2 (b) The first 10,000 outputs of each stream (recommended: own ChaCha8 in sim)
- Streams that exist now: none (World = tick + seed, world.rs:16-19). cs3 adds the generator, not the purposes:
  `crates/sim/src/rng.rs` (`ChaCha8`: 32-byte key, 64-bit stream id, 64-bit block counter, 16-word buffer; next_u32;
  next_u64 = two consecutive words, low first; position get/set; the stream state's canonical bytes at fixed width, no
  usize). No stream joins World in cs3, so the state bytes, the hash and the recording format are untouched. The five
  purposes of part 8 (:323-325) wait for their first draw: weapon spread (cs8 Shoot), match setup (the first phase that
  draws), bot decisions (Slice 2), ability procs and upgrade offers (later slices); each one's id and its 10,000 words
  join the file in its own change-set, with its sampling functions' vectors (PM-0004-27 "and of the sim's own sampling
  functions on them").
- Pinned now: a public test key (the Strombergson draft's TC8 key), stream ids 0, 1, 2^32 and 2^64-1 (both halves of
  the id's two state words); per id 10,000 next_u32 from word 0; next_u64 checked as pairs of the same words; one
  save-and-restore at word 1,001 (next 1,000 words equal); one window of 96 words across the 2^32-block carry (one id).
  File `crates/devtools/vectors/chacha8-words.bin` = 4 x 40,000 + 384 = 160,384 bytes, with `chacha8.manifest.txt`
  (key hex, ids, counts, starts, SHA-256 per section, generator name). Generator `make_chacha8_vectors.py` grown from
  rr-chacha-ref.py (which matches all 48 draft vectors), self-checked on the 48 before it writes.
- In sim itself: the draft's 8 vectors at 8 rounds and 256-bit keys (2 blocks each, 1,024 bytes) as hex in rng.rs's
  unit tests: native dev and release through today's lines.
- Native cross-target file test: `crates/devtools/tests/stream_vectors.rs` (first differing index); Wasm:
  `chacha8_words` in the test module, Node and Chromium as 4.0.
- Not in cs3: ChaCha8's cost per tick at N = 48 (PM-0004-27's reading): no draw per tick until spread (cs8).
- If he picks a library: the same files, generator and tests; rng.rs becomes a thin `Stream` over the library's
  ChaCha8Rng with the order from_seed -> set_stream -> set_word_pos and no get_block_pos (rust-reviewer's conditions);
  for chacha20 0.10.2 two more native runs with `--cfg chacha20_backend="soft"` and `"sse2"` (rustflags: a CI edit E4,
  since build.mjs refuses RUSTFLAGS from the environment for the shipped build); overflow overrides (chacha20, rand_core,
  cpufeatures; rand_chacha: four), SA-TREE's check proving itself on that addition; security-auditor's answer on the
  exact manifest first; ADR 0004 part 8's crate name changes. With own code, part 8's "only from rand_chacha's
  ChaCha8Rng" (:309) changes to our own ChaCha8 (a decision-draft line: his go).

### 4.3 (c) The scan test (interim, under the advisor's three conditions, build plan :949-953)
- File `crates/sim/tests/rules_scan.rs`; scans exactly `crates/sim/src/**/*.rs` (from CARGO_MANIFEST_DIR, sorted).
  Runs in today's CI test job (`cargo test --workspace`, and `cargo test --release -p sim`): condition 2, no CI edit.
- Condition 1: two lists in the test: ALLOWED = part 7's methods word for word (total_cmp, sqrt, floor, ceil, round,
  trunc, fract, mul_add, abs, copysign, is_finite, is_nan, to_bits, from_bits; operators, comparisons and `as` are
  syntax); STD = every inherent method of std 1.99.1's f32 and f64 pages (read by the build session from
  doc.rust-lang.org, URL and date in the file; no rust-src in the pinned toolchain). BANNED = STD - ALLOWED. The test
  asserts ALLOWED is inside STD and BANNED holds part 7's named ones (powi, hypot, cbrt, to_degrees, to_radians, min,
  max, clamp, rem_euclid, div_euclid, sin, cos, atan2, powf, exp, algebraic_*).
- Fails on: (1) a BANNED name as a method call `.name(` or as `f32::name(`/`f64::name(` (a text scan cannot see types,
  so integer code in sim uses the path form on an integer type, `Ord::min(a, b)` or `u32::min(a, b)`; the failure
  message says so); (2) HashMap, HashSet, hash_map, hash_set, RandomState, DefaultHasher, unless the item carries
  part 9's escape (`#[allow(clippy::disallowed_types, reason = "...")]`, ADR 0004 :340-342); (3) Instant, SystemTime,
  UNIX_EPOCH and any `std::time` path (the tick is the only clock, part 4). Comments and string/char literals are
  stripped first (a small lexer: nested block comments, raw strings, char literals against lifetimes). Not caught, as
  part 7 says (:282-283): the float `%` operator.
- Bites in the same file (PM-0004-23's list :1144): a HashMap field, x.sin(), deg.to_radians(), a.min(b),
  f32::powi(x, 2), Instant::now(), std::time::Duration; controls: x.abs(), x.fract(), x.sqrt(), the escape on a
  lookup-only map, cmp::min(a, b), a comment and a string that name HashMap and .sin(.
- Out of its scope: room and client-endpoint (their clock ban is G5's, ADR 0002 :209-210); extending the clock pattern
  to them is beyond the ruling (only with the advisor's word).
- The report says "interim" and names the ruling it replaces (the 2.3 plan's: the float ban merges before any float
  arithmetic enters sim). Note: with own ChaCha8, cs3 brings no float arithmetic into sim; the interim matters from cs4.
- Open in the draft: part 7's "such as is_finite, is_nan, to_bits and from_bits" (:267-268) is not a closed list; the
  scan takes exactly those four (B6).

### 4.4 (d) PM-0002-53, the planted overflow (4 of 4)
- Native dev + release: sim/src/lib.rs's `overflow_checks_are_on` becomes the mark's u16 counter (`black_box(u16::MAX) +
  1` should_panic "attempt to add with overflow") plus `wrapping_add` giving 0; today's `cargo test --workspace` and
  `cargo test --release -p sim` run both: no CI edit.
- Wasm dev + release: the test module's two builds; `plant_u16_add(65535, 1)` throws WebAssembly.RuntimeError and the
  real hook's buffer holds "attempt to add with overflow" with its location; `plant_u16_wrapping_add(65535, 1)` is 0 in
  both, as natively. A fresh instance per case (a trapped one is never called again). This also proves the panic hook
  in Wasm (ADR 0004 S20's check).
- Reading (B6): the mark's "a committed test in sim" holds natively; in Wasm the planted counter is devtools' code
  under the same workspace profile (`[profile.release] overflow-checks = true` for every member, root Cargo.toml).
  Literal "in sim" in Wasm needs wasm-bindgen-test or a test item in sim's shipped code. Its cost clause ("with the
  tick bench's method") waits for the tick bench (devtools; not built in cs3).

### 4.5 (e) The two-instance test
- Owner: lane 2, after cs3 merges, never merged (Part 15 :78-80, :105; LANE-SPLIT (a)); the card :327-331 is older (B6).
- Design: the same Worker instantiates the same compiled module twice (two linear memories): instance A the authority
  (room + sim), instance B the client endpoint; the host copies the encoded datagrams between the two memories and calls
  them in ADR 0002 part 6's order (:407). The patch adds two exported types (authority-only, endpoint-only) and a
  test-only trigger that makes the endpoint panic.
- What it shows: after the planted trap in B, A's tick keeps rising for 10 s and its per-tick hashes equal a run with no
  trap, and B restarts and joins again; in the game's one-instance layout the same trap ends the match (the authority
  lived in the trapped instance). Informs OQ-0002-3 and OQ-0004-7. Counts, time and memory at 12 and 48: cs5.
- Cost: Part 3.2 "half a day, twice" (:558); my estimate of the patch about 300 lines (two constructors and the split
  about 80, the host's two-instance driver about 120, the test about 80, a build variant about 20). Kept as a patch file
  in its report and a build (Part 3.2 :551-553). First to move out if cs3 nears twice its size (the card): with Part
  15 it is already outside; cs3 keeps only the seam (2.1: the loopback carries bytes).
- B4 DONE. Next: Block 5 (size and order).

## Block 5. Size and order (DONE)

### 5.1 Lines added, my half (estimates; no code written; hand-written lines as the plan counts them)
Rust (about 1,200): sim-wasm lib.rs +90 (rewritten; about 120 removed), local.rs +25 (struct and seam; the order is
netcode's), panic.rs +75; client-endpoint endpoint.rs +40 (the moved state; the rest netcode's), layout.rs +12,
block.rs +60; sim rng.rs +270 (150 code, 120 tests), lib.rs +15, tests/rules_scan.rs +280; devtools lib.rs and
rules/mod.rs +10, rules/f32_ops.rs +130, plants +10, wasm.rs +60, tests/f32_vectors.rs +50, tests/stream_vectors.rs +60;
Cargo.toml files +14.
Python generators (about 250): make_f32_vectors.py +150, make_chacha8_vectors.py +100.
TypeScript, JS, CSS, config (about 1,360): sim-worker messages.ts +55 / test +50, host.ts +30 / test +60,
panic-message.ts +20 / test +30, worker.ts +8, index.ts +1, stats.ts +8 / test +8, block-pool test +6; client
input/keyboard.ts +70 / test +70, overlay/keys.ts +50 / test +40, overlay/report.ts +25 / test +30, app/press-tally.ts
+40 / test +50, app/worker-link.ts +40 / test +45, app/boot.ts +20, app/error-hook.test.ts +25, harness/frame-times.ts
+6 / test +10, styles.css +15; scripts build.mjs +53, build-checks.mjs +40 / test +62, serve.mjs +3 / test +48;
boundary/sim-wasm.test.mjs +45, boundary/rules.test.mjs +90; e2e tests/skeleton/loop.spec.ts +90,
tests/rules/wasm-rules.spec.ts +80, allocation.spec.ts +20, skeleton/isolation specs +11; .gitignore +2, package.json 1.
My half: about 2,800 lines. Vector files (fixtures, not counted): f32-inputs.bin 120,000 bytes; f32-expected.txt about
3 KB (or every output: about 1,230,000 bytes); chacha8-words.bin 160,384 bytes; two manifests, about 2 KB.
=> The card's 2,700 (:249) is about my half alone; with netcode's half (codec, room, endpoint logic, step, replay, L1)
the change-set nears or passes twice its plan (5,400). The plan must total both halves before the advisor's review.

### 5.2 Build order of the whole change-set (each step ends green: cargo dev+release, npm, boundary, e2e, gate)
0. #62 merged and taken in; his answers RNG-CRATE and WASM-TEST-ROUTE; CI edits asked with exact text.
1. Follow-ups needing nothing new: EA-F8, EA-F10 (TS part), EA-F13 (1)(3), EA-N3 (page half), EA-N6+VF-L7, FAVICON,
   VF-L4, TE-L, SA-ENV, TE-NT3/4/5, FPS-LINE, NEG-ZERO, `__pycache__/` in .gitignore.
2. The scan test and PM-0002-53's native half (sim only; no manifest change).
3. ONE manifest batch (the hold: security-auditor answers on exactly this content; one release record): devtools
   (crate-type, deps), sim-wasm, room, client-endpoint, protocol path deps, sim-worker's @shamble/types to dev; with a
   library (RNG-CRATE (a)/(b)) its lines and overflow overrides here too.
4. ChaCha8 in sim with the published vectors; the stream generator and file; devtools' native stream test. (Needs
   RNG-CRATE.)
5. Float vectors: generator, inputs, expected digests, devtools' runner and native test.
6. The panic hook (sim-wasm panic.rs, panic_buffer), the Worker's guard and panic-message.ts (EA-N3 Worker half).
7. The devtools test module and `build.mjs --devtools-wasm`; boundary/rules.test.mjs (vectors, plants dev+release,
   the hook's text). (Needs WASM-TEST-ROUTE; runs in CI after E2.)
8. Netcode's core in its order (codec + loopback, room, ClientEndpoint with EA-F9's move, the step), the block's echo
   and link sections, sim-wasm's new surface (input with six numbers; newest_input and now_ms dropped), the host and
   page messages, boundary/sim-wasm.test.mjs updated.
9. The page alive: input/, press tally, key diagram, readout and report rows, loop.spec.ts, the allocation spec
   (TE-NT7, PA-N2).
10. Netcode's replay runner, the first recording, native twice, `replay` in the test module (Node, and Chromium through
    wasm-rules.spec.ts); L1 and its statistics.
11. SA-TREE's check; RR-F5's remap (rust-reviewer's mechanism); the size table (with and without the hook; the build
    options table) on one named machine. Then reviews, verifier, report, the zip with Brave, Firefox and BR-GAP steps.
CI edits (each asked of him with its exact text; recommended in a small CI pull request right after cs3, as CI-WASM, so
cs3 stays risk tier 1 (.github/ is tier 2); the scan test and every new native test already run in today's
`cargo test --workspace`, the L1 tests too):
- E1, toolchain.yml :88: `cargo test --release -p sim -p protocol --locked` -> `cargo test --release -p sim -p protocol
  -p devtools --locked` (vectors, replay and L1 in release; netcode may add -p room).
- E2, inside the cut before the boundary step: `node packages/client/scripts/build.mjs --devtools-wasm` (the Wasm-side
  vector run, the plants and the Chromium rules spec then run in the existing boundary and Playwright steps).
- E4, only with chacha20: the native vector tests again with `--config 'build.rustflags=["--cfg",
  "chacha20_backend=\"soft\""]'` and with "sse2".
- No gate edit (`tools/gate.mjs` is legacy-only and untouched).

### 5.3 What moves out first if it grows (after the card's two: the two-instance test, already lane 2's, and the build
options table)
1. Hygiene not needed by cs3's own changes: TE-L, VF-L4, FAVICON, SA-ENV, TE-NT5 (about 85) -> a small follow-up.
2. SA-TREE and RR-F5 (about 95) -> the CI pull request after cs3.
3. TE-NT7 + PA-N2 (about 20) -> cs5's bench.
4. The vectors' Chromium run (about 40; the replay's Chromium run stays: the Step 2.5 table :470 asks it here).
5. The scan's literal stripping (about 40): strip comments only; a string literal naming a banned call then fails.
Never cut: the scan test, the vectors natively and in Node, PM-0002-53, the panic hook, EA-F8/F9/F10/N3, the page alive.
- B5 DONE. Next: Block 6.

## Block 6. Where the build plan or the drafts are wrong or out of date for cs3; questions for him (DONE)

### 6.1 Out of date or wrong (file:line)
1. Two-instance test owner: build plan :327-331 puts it in cs3; Part 15 :78-80 and :105 (his LANE-SPLIT (a), newer)
   give "the comparisons that stay" to lane 2, built after cs3 merges, never merged. The plan follows Part 15 and says
   so (a stated reading; a question only if the advisor wants one).
2. "The table of build options" (:330) is defined nowhere else; most likely ADR 0002 part 4's Wasm size readings
   (:270-274: opt-level "s" against "z", with and without wasm-opt). wasm-opt is a new tool (TOOL-SOURCE applies): its
   column cannot be read without his word.
3. ADR 0004 part 8 :309 "The sim draws only from rand_chacha's ChaCha8Rng, pinned": rand's own guide now says
   rand_chacha "is likely to be discontinued" (rust-reviewer F2.1); the line changes with any RNG-CRATE answer
   (decision-draft text: his go).
4. ADR 0004 PM-0004-22 :1143 (and PM-0004-27's method) name "unit vectors in cargo test and wasm-bindgen-test
   (R5-TEST-REC1)"; with WASM-TEST-ROUTE (a) the Wasm halves run through devtools' test module in Node and Chromium.
5. ADR 0002 PM-0002-53 :1327 "a committed test in sim run in the four builds": the Wasm halves run a planted counter in
   devtools' module under the same workspace profile; literal "in sim" in Wasm needs wasm-bindgen-test. Its cost clause
   needs the tick bench (not built in cs3).
6. ADR 0004 part 7 :267-268 "the exact non-arithmetic methods the rules themselves use, such as is_finite, is_nan,
   to_bits and from_bits": an open list; the interim scan takes exactly those four (closing it is a draft line).
7. ADR 0002 part 3 :191 (sim-wasm: "the wasm-bindgen bindings ... and nothing else"): with option (A) it also holds
   the 25-line composition `LocalMatch` and devtools depends on it and becomes a cdylib using wasm-bindgen (:194 allows
   "anything", but no row says so). A note for the drafts' next revision.
8. ADR 0002 part 5 :330-332 (the control API list): cs3's input call takes six plain numbers, `now_ms` leaves create,
   `panic_buffer()` and the start function join, `load the content bundle` waits for cs4. Same next revision (with
   EA-F6 and EA-F12, STATUS_ARCHIVE :1133-1135).
9. Build plan card :324-326 "the first 10,000 outputs of each random stream": no stream exists (world.rs:16-19) and
   none is drawn in cs3; the plan reads it as the generator at four edge ids, each purpose's stream joining with its
   first draw (spread first, cs8).
10. Build plan :446-447 "CI files are the 2.3 chat's" and every "2.3 after 3" in the Step 2.5 table (:466-475): out of
    date by Part 15 :174 (lane 1 owns toolchain.yml while the 2.3 chat is stopped); "2.3 after 3" becomes lane 1's CI
    pull request after cs3.
11. Build plan :939 ("the unit vectors in Wasm | 2"): they did not land in cs2; they land in cs3, in devtools' module.
12. ADR 0002 part 6 :397-399 ("whether a second instance is worth its cost ... is measured in the boundary spike"): now
    lane 2's comparison after cs3 (Part 15).
13. test-engineer-followups.md :239-241 (SA-TREE "adds the random-number crate"): true only for a library; with own
    ChaCha8 no runtime crate joins, its plants use fixtures only.
14. The card's size (:249, 2,700 lines) against my half's estimate of about 2,800 (5.1).

### 6.2 Questions for Mustafa (self-contained, plain words)
- RNG-CRATE (his own question; first): Where do the game's random numbers come from? Why it matters: bot choices,
  weapon spread and upgrade offers must come out the same on the server and in every browser for years. Options: (a)
  rand_chacha 0.10.0, the library the engine draft names (four outside packages; its makers expect to discontinue it);
  (b) chacha20 0.10.2, the library the rand project moved to (two outside packages, three on the server; the server
  picks one of three fast paths by its processor; two earlier releases were withdrawn in August for undefined
  behaviour on old processors, with no report of wrong numbers); (c) our own small ChaCha8 in the simulation (about
  270 lines with its tests, checked against published test numbers; nothing from outside enters). Recommendation: (c),
  with rust-reviewer: the only option where server and browsers run the very same code.
- WASM-TEST-ROUTE: How do the rule tests run inside WebAssembly? Why it matters: it decides whether new outside code
  enters, and how literally two pass marks are met. Options: (a) a test-only build of our own tools crate, made with
  the bindings tool you already approved, run in the test steps that exist (no new outside code; the marks' wording
  "wasm-bindgen-test" and "a test in sim" change in the drafts); (b) the drafts' named tool, wasm-bindgen-test (new
  outside packages, and CI must unpack a second program from the archive you approved). Recommendation: (a).
- CI-LOOP: Where do change-set 3's two CI edits go (E1: the release test line also runs our tools crate; E2: one line
  that builds the test-only WebAssembly before the tests)? Why it matters: a CI edit makes a pull request the higher
  risk tier. Options: (a) a small CI pull request right after change-set 3, as with CI-WASM; (b) inside change-set
  3's pull request. Recommendation: (a); the exact text goes to you with the plan.
- SIZE-3 (only if the two halves together pass about 4,000 lines): The card plans 2,700 lines; our estimates are higher.
  Why it matters: past twice the plan the work stops and asks you. Options: (a) one pull request with a new planned
  size and stop line; (b) two pull requests: 3a, the rule tests and boundaries (nothing new to see), then 3b, the loop
  with your browser check; (c) keep 2,700 and move the listed items (5.3) out. Recommendation: (b): the rules settle
  before the loop rests on them, which is the card's own order.
- B6 DONE. Next: DONE, then the answer.

## Not verified (for the answer)
- Every line count (no code written); the panic hook's Wasm bytes; the std 1.99.1 f32/f64 method list (to be read).
- That Vite merges an inline `worker.plugins` with vite.config.mjs (EA-N6's route; the fallback is named).
- How LLVM lowers floor/ceil/trunc/round/mul_add on baseline x86-64 and wasm32 (my reading; rust-reviewer to confirm).
- That a Playwright-routed page (http://rules.invalid/) loads the module's ES glue and Wasm; that devtools linking
  sim-wasm's rlib brings its exports and start function into the test module without a clash; that six u32 params
  pass with no glue allocation (as today's u32 param does).
- RR-F5's remap through `cargo --config build.rustflags` (rust-reviewer's call).
- Nothing was built, run or opened in a browser (the brief's rules).
DONE
