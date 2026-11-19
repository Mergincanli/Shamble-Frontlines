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
