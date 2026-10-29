# R5 draft, question 4: tests (R5-TEST)

Brief question (quoted): "tests (cargo test, property tests, Vitest, Playwright, golden tests against legacy)". Task
scope: cargo test, property tests (crates and versions), Vitest, Playwright, golden tests against legacy behaviour
(which behaviour and how: the toolkit's sim, battery and tuning data), determinism tests, and network tests with
injected latency, jitter and loss. All sources read 2026-10-28.

## Findings

- **R5-TEST-F1. `cargo test` runs a crate's tests on several threads of one process; doc tests run too.** Source:
  https://doc.rust-lang.org/cargo/commands/cargo-test.html · official · read 2026-10-28 · confirmed in the source: "The
  executable automatically runs all functions annotated with the `#[test]` attribute in multiple threads."; `--jobs`
  "does not affect how many threads are used when running the tests."; "Documentation tests are also run by default,
  which is handled by `rustdoc`."; `--no-fail-fast`: "Run all tests regardless of failure." Consequence (inferred): a test
  that reads a clock or shares global state can pass alone and fail beside others.
- **R5-TEST-F2. proptest 1.12.0 runs 256 cases by default, saves failing cases to `proptest-regressions` files, and
  takes its seed from an environment variable.** Sources: https://docs.rs/proptest/latest/proptest/ and
  https://docs.rs/proptest/latest/proptest/test_runner/struct.Config.html (1.12.0) · official (crate docs) · read
  2026-10-28 · confirmed in the source: "Hypothesis-like property-based testing and shrinking."; `cases` "The number of
  successful test cases that must execute for the test as a whole to pass." (default 256, "can be overridden by setting
  the `PROPTEST_CASES` environment variable."); `failure_persistence` "Indicates whether and how to persist failed test
  results." (default `FileFailurePersistence::SourceParallel("proptest-regressions")`); the seed is "Set by using the
  PROPTEST_RNG_SEED environment variable." That an unset variable means a fresh random seed per run is from the fetch
  summary (inferred).
- **R5-TEST-F3. insta 1.49.0 keeps snapshots as `.snap` files beside the tests, never writes them in CI, and can round
  floats.** Source: https://docs.rs/insta/latest/insta/ · official · read 2026-10-28 · confirmed in the source: snapshot
  tests "assert values against a reference value (the snapshot)"; stored "Right next to your test in a folder called
  `snapshots` as individual" `.snap` files; the default update mode is "`no` for CI environments or `new` otherwise", and
  `no` "does not write to snapshot files at all; just runs tests"; `rounded_redaction` "Creates a redaction that rounds
  floating point numbers to a given number of decimal places."
- **R5-TEST-F4. cargo-fuzz drives libFuzzer, needs a nightly compiler, and runs only on Unix-like x86-64 and AArch64.**
  Source: https://github.com/rust-fuzz/cargo-fuzz (README; no version shown) · official (the project) · read 2026-10-28 ·
  confirmed in the source: "A `cargo` subcommand for fuzzing with `libFuzzer`! Easy to use!"; it "needs a nightly
  compiler since it uses some unstable command-line flags."; libFuzzer "only works on x86-64 and Aarch64, and only on
  Unix-like operating systems (not Windows)."
- **R5-TEST-F5. wasm-bindgen-test runs Rust tests compiled to Wasm under `cargo test`, in Node.js by default or in
  Chrome, Firefox or Safari through their WebDriver.** Sources:
  https://rustwasm.github.io/docs/wasm-bindgen/wasm-bindgen-test/index.html and .../browsers.html (no version shown; the
  wasm-bindgen project has since moved organisation, R1-BND-F2, so this guide may be older than the crate) · official ·
  read 2026-10-28 · confirmed in the source: "an experimental test harness for Rust programs compiled to Wasm"; "the
  usual `cargo test` command but with an explicit wasm target."; "By default tests run on Node.js."; for
  `wasm_bindgen_test_configure!(run_in_browser);`, "Note that this will ignore any environment variable set."; drivers
  through `GECKODRIVER`, `CHROMEDRIVER` and `SAFARIDRIVER`.
- **R5-TEST-F6. Vitest 5.0.5 runs tests in a real browser through Playwright (Chromium, Firefox, WebKit) or WebdriverIO
  (Chrome, Edge, Firefox, Safari).** Source: https://vitest.dev/guide/browser/ (v5.0.5) · official · read 2026-10-28 ·
  confirmed in the source: Browser Mode "allows you to run your tests in the browser natively"; "Vitest always requires a
  provider to be defined"; "`playwright` supports these browsers:" Firefox, WebKit, Chromium; "`webdriverio` supports
  these browsers:" Firefox, Chrome, Edge, Safari; "However, to run tests in CI you need to install either `playwright` or
  `webdriverio`." Stable or experimental: not stated (confirmed by absence).
- **R5-TEST-F7. Vitest retries nothing by default, and its fake timers replace `setTimeout` and `setInterval`.**
  Sources: https://vitest.dev/config/retry and https://vitest.dev/guide/mocking/timers · official · read 2026-10-28 ·
  confirmed in the source: `retry` "Retry the test specific number of times if it fails." (Default `0`); fake timers
  "mock calls to `setTimeout` and `setInterval`". Whether they fake `performance.now` is not on the page (confirmed by
  absence).
- **R5-TEST-F8. Playwright's WebKit is not Safari; for the closest-to-Safari result it points to macOS.** Source:
  https://playwright.dev/docs/browsers (no version shown) · official · read 2026-10-28 · confirmed in the source:
  "Playwright can run tests on Chromium, WebKit and Firefox browsers."; "Playwright's WebKit is derived from the latest
  WebKit main branch sources, often before these updates are incorporated"; "Playwright doesn't work with the branded
  version of Safari since it relies on patches."; "While running WebKit on Linux CI is usually the most affordable
  option, for the closest-to-Safari experience you should" (the rest of the sentence, run on macOS, is inferred from the
  fetch summary); likewise "Playwright doesn't work with the branded version of Firefox since it relies on patches."
- **R5-TEST-F9. Playwright retries nothing by default, labels a test that passes only on retry "flaky", and can fail
  the run on any flaky test.** Sources: https://playwright.dev/docs/test-retries and
  https://playwright.dev/docs/api/class-testconfig · official · read 2026-10-28 · confirmed in the source: "By default
  failing tests are not retried."; flaky means "tests that failed on the first run, but passed when retried.";
  `failOnFlakyTests` (added in v1.52) "Whether to exit with an error if any tests are marked as flaky. Useful on CI.";
  `forbidOnly` "Whether to exit with an error if any tests or groups are marked as test.only() or test.describe.only().
  Useful on CI."; `repeatEach` "The number of times to repeat each test, useful for debugging flaky tests."; `webServer`
  "Launch a development web server (or multiple) during the tests."
- **R5-TEST-F10. Playwright's clock controls a page's time sources.** Source: https://playwright.dev/docs/clock ·
  official · read 2026-10-28 · confirmed in the source: it "overrides native global classes and functions related to time
  allowing them to be manually controlled."; `runFor` "Runs the time for a specific duration."; `fastForward` "Fast
  forwards the time." The covered list (Date, timers, `requestAnimationFrame`, `performance`) is from the fetch summary
  (inferred). Workers are not mentioned (confirmed by absence).
- **R5-TEST-F11. turmoil 0.7.2 simulates hosts and a network in one thread, with seeded latency and message failure,
  for Tokio code.** Sources: https://github.com/tokio-rs/turmoil (README) and https://docs.rs/turmoil/latest/turmoil/,
  .../struct.Builder.html (0.7.2) · official · read 2026-10-28 · confirmed in the source: "Turmoil is a family of crates
  for deterministic simulation testing of distributed systems."; it "injects 'hardship' — latency, drops, partitions,
  crashes, torn writes — into the simulated network and filesystem." "under manual control or a seeded RNG."; turmoil-net
  is a "Drop-in replacement for `tokio::net`"; "This module contains the simulated TCP/UDP networking types.";
  `min_message_latency` / `max_message_latency` "The minimum [maximum] latency that a message will take to transfer over
  a link on the network."; `fail_rate` "The failure rate of messages on the network."; `rng_seed` "Seed for random numbers
  generated by turmoil." QUIC is not mentioned (confirmed by absence).
- **R5-TEST-F12. A shipping game transport injects loss, lag, jitter, reordering, duplication and rate limits below the
  protocol, per direction.** Sources:
  https://raw.githubusercontent.com/ValveSoftware/GameNetworkingSockets/master/include/steam/steamnetworkingtypes.h and
  https://github.com/ValveSoftware/GameNetworkingSockets (README: "a basic transport layer for games." that "has shipped
  on consoles, mobile platforms, and non-Steam stores") · shipped game (a shipping vendor's own library, the tier rule of
  R2) · read 2026-10-28 · confirmed in the source: "[global float, 0--100] Randomly discard N pct of packets instead of
  sending/recv"; "This is a global option only, since it is applied at a low level"; "[global int32].  Delay all
  outbound/inbound packets by N ms"; "Simulated jitter/clumping."; "[global float 0--100] Percentage of packets we will
  add additional" [delay to, for reordering]; "[global float 0--100] Globally duplicate some percentage of packets.";
  "[global int32] Global UDP token bucket rate limits."; "[global int32] Trace every UDP packet, similar to Wireshark or
  tcpdump."
- **R5-TEST-F13. quinn 0.11.12 can run on any socket that implements its `AsyncUdpSocket` trait.** Sources:
  https://docs.rs/quinn/latest/quinn/trait.AsyncUdpSocket.html and .../struct.Endpoint.html (0.11.12) · official · read
  2026-10-28 · confirmed in the source: "Abstract implementation of a UDP socket for runtime independence";
  `Endpoint::new_with_abstract_socket(config, server_config, socket: Arc<dyn AsyncUdpSocket>, runtime)` "Construct an
  endpoint with arbitrary configuration and pre-constructed abstract socket". Whether wtransport or web-transport-quinn
  (R2-TRN-REC1) expose this constructor: not read (unverified).
- **R5-TEST-F14. A game-networking practitioner tests against extreme simulated networks and soak-tests overnight.**
  Source: https://gafferongames.com/post/packet_fragmentation_and_reassembly/ (September 6, 2016) · other (R2 tiers
  Glenn Fiedler's articles "other", R2-XU-OQ16) · read 2026-10-28 · confirmed in the source: "Implement network
  simulators and make sure code handles the worst possible network conditions imaginable."; "99% packet loss, 10 seconds
  of latency and +/- several seconds of jitter."; soak with "a huge amount of packet loss, out of order and duplicates
  through a packet simulator."; "Your soak test passes when it runs overnight and doesn't hang or assert."

### Legacy facts used
- **Evidence habits** (START_HERE §5): targeted harness tests on the real code: animation metrics (hands on sockets
  < 1 cm, planted-foot drift, boots ≥ 0 cm above ground, IK stretch ≤ 0.98), a fuzz of every class and ability with 0
  errors, a 3-minute soak with a flat heap after GC, `sim.js` stall % over at least 3 runs per build (random AI).
- **TL9** (redesign): the smoke shape carries: wait on game state with generous timeouts, check that each step did its
  job, console errors fail, report the renderer string, fresh storage per page; probes move to a v2 test-hook API; the
  network rule flips to allow the local game server's WebTransport and WebSocket only.
- **TL11** (drop, kept as the legacy oracle until legacy retires): `preview.js eval` runs harness snippets on the real
  legacy build; the natural runner for golden values.
- **TL13** (redesign): keep sim.js's metrics (stall %, engaged against free stalls, kill matrix, friendly fire,
  objective counters, slot bias); v2 runs seeded, deterministic bot matches on the native server. Legacy same-seed runs
  differ (11 wall-clock reads, STATUS:111).
- **TL14** (redesign): reachability becomes a 3D navigation check in the Rust sim, callable from the editor and CI.
- **TL15** (redesign): warm-up then measured steps, percentiles, reps, the fairness metric; in v2 a Rust tick bench at N
  players plus a client render bench.
- **LOOP_V2 §11**: evidence per greybox step is the gate, `sim.js` with three teams and at least 3 runs per build,
  objective counters (cashouts, steals, deliveries, harvester kills); then the scale test at N = 12, 24, 36, 48.
- **Golden candidates with recorded numbers**, all marked "rewrite in Rust": CS4 (a ray that starts inside the torso
  cylinder returns -1, inside the head sphere 0), CS5 (a face shot on a 45° crate measures 4.586, was 3.586), CS6 (the
  B7 batch ray gave the same results as the old code on 20,000 of 20,000 random rays, 1,640 rotated hits), GW14 and
  GN10 (the jump reach is 3.07 m by the formula, about 2.98 m as integrated at 60 Hz: frame-rate dependent), CD8
  (locational damage multipliers), CD12 (the weapons table; LOOP_V2 §7 changes the AMK), GE8 (presence count). Marked
  "port to TypeScript": GA5 (planted-foot drift 0.0 on walk, strafe and backpedal; sprint mean 0.14) and GA8 (hand on
  grip 0.0 cm, target < 1 cm). `docs/legacy-tuning.json`: every tuning number, produced by `tools/tuning-dump.js` on the
  real build; two runs per map gave byte-identical output; 17 values checked against their source literal.
- **Gate history** (docs/reports/2026-10-14-gate-check-5.md): 22 of 22 planted defects failed exactly the expected
  checks, and controls (valid code that looks suspicious) passed: the model for proving each test can fail.
- Wave 1 cited, not restated: R1-DET-REC6 (recorded matches, per-tick state hashes; native twice in separate processes,
  Wasm in Node, Wasm in the browsers the runner offers; "R5 picks the tool"), R1-ECS-REC2 mark 1 (two processes, same
  hashes over a 10-minute bot match), R2-XU-OQ12 and r2-netcode.md's sync profiles (P0 = 0 ms; P1 = 50 ms ±10 ms 1 %;
  P2 = 100 ms ±20 ms 2 %; P3 = 150 ms ±30 ms 5 %; P4 = 200 ms ±40 ms 5 %; B1 = P2 plus a 1 s burst of 100 % loss; round
  trip; reordering in every spike), R2-TRN-REC4 marks 1, 2 and 4 (all four browsers on both backends; app RTT within
  ±5 ms of quinn's at 100 ms; 10 million fuzz inputs per message type with 0 panics), R2-TRN-OQ1 (a recurring run
  against browser beta channels), R4's notes to R5 (a counting test for the static BVH build, a parity fuzz, the
  reachability check in the sim crate), R3-MEASURE-REC3 (CI gates counts, not timings).

## Options

### Property tests and fuzzing
| Option | Read? | Fit | Cost |
|---|---|---|---|
| proptest | yes (F2) | shrinking, committed regression files, seedable | a dev-dependency |
| quickcheck, bolero | no (not fetched in this run) | unknown | – |
| cargo-fuzz for decoders | yes (F4) | coverage-guided, for the bytes clients send | nightly; Linux CI only |

### Where browser tests run
| Option | Browsers | Fit | Risk |
|---|---|---|---|
| Playwright Test | Chromium, Firefox, WebKit (F8) | end-to-end client plus local server (`webServer`, F9); TL9's shape | WebKit is not Safari (F8) |
| Vitest Browser Mode | via Playwright or WebdriverIO (F6) | component and boundary tests inside a page | maturity not stated (F6) |
| wasm-bindgen-test | Node by default; Chrome, Firefox, Safari by WebDriver (F5) | Rust unit tests compiled to Wasm, unchanged | the guide read may be older than the crate |
| Real Safari on macOS (safaridriver) | Safari | the only way to test Safari itself (F5, F8) | a macOS runner (cost: R5-CI, R6) |

### Network tests
| Option | What it simulates | Deterministic | Covers the real transport | Cost |
|---|---|---|---|---|
| L1. Our own link simulator behind the transport interface (R2-TRN-REC4) | per-direction latency, jitter, loss, reordering, duplication, rate limit (F12's set) on simulated time | yes, seeded | no (no QUIC, no browser) | small, our code |
| turmoil (F11) | Tokio TCP and UDP with latency and message failure | yes, seeded | QUIC on it not stated | depends on quinn running over it (unverified) |
| L2. An impairment socket under the real quinn endpoint (F13), GNS-style (F12) | the same set, on the server's UDP | no (real time, real OS) | yes, for WebTransport from real browsers | small; WebSocket (TCP) needs a separate proxy |
| OS-level impairment (Linux `tc netem`) | kernel-level delay, loss | no | yes | its docs are outside the allow list (not read) |

## Recommendations

- **R5-TEST-REC1. The layers, the tool for each, and where each runs.**
  | Layer | Tool | What it proves | Runs |
  |---|---|---|---|
  | Rust unit and property | `cargo test`, proptest 1.x | sim rules, maths, decoders, content validation | gate: every PR |
  | Rust in Wasm | wasm-bindgen-test in Node | the same unit tests pass when compiled to Wasm | gate |
  | Rust ↔ TypeScript boundary | Vitest (Node) loading the built `sim-wasm` | the layout hash of R1-BND-REC4, every message type round-trips between Rust encoders and TypeScript decoders | gate |
  | TypeScript unit | Vitest (Node) | client logic that is not rendering: input mapping, interpolation buffers, HUD state | gate |
  | Browser end-to-end | Playwright Test (Chromium, Firefox, WebKit) with the built client and a local server via `webServer` | TL9's smoke shape on v2, both transport backends (R2-TRN-REC4 mark 1), counts of R3-MEASURE-REC3 | gate |
  | Safari | the e2e suite on macOS with real Safari | what WebKit on Linux cannot show (F8) | for PRs touching `client`, `net`, `sim-worker` or `sim`, else nightly (cost: R5-CI) |
  | Legacy golden | `cargo test` reading committed fixtures (REC3) | the behaviour v2 keeps equals legacy within stated tolerances | gate |
  | Determinism | REC2's suite | identical per-tick hashes on every target | gate (short), nightly (long) |
  | Network | REC4's two layers | R2's pass marks under named profiles | gate (L1 short), nightly (L1 long, L2) |
  | Soak and load | REC5 | 48+ players with bots, hours | nightly and before each playtest |
  | Fuzz | cargo-fuzz on decoders and the content loader | 0 panics, 0 over-limit allocations | nightly, Linux |
  Reason: each tool is the read option for its layer (F1, F2, F4–F9); WebKit stands in for Safari only partly (F8).
  Rests on: R5-TEST-F1, F2, F4, F5, F6, F8, F9; legacy TL9, TL15; R1-BND-REC4, R2-TRN-REC4, R3-MEASURE-REC3 (cited).
  Serves: §6.1–§6.9 through their tests; §4 (four desktop browsers).
  A spike must prove (proposed): one planted defect per layer fails that layer and is caught by the gate or nightly job
  the table names (for example: a byte-order swap in one encoder fails the boundary layer only; a page error in deploy
  fails e2e only); every layer's clean run passes 20 times in a row on one commit.
- **R5-TEST-REC2. The determinism suite's tools (R1-DET-REC6 asks R5 to pick them).** A `devtools` replay runner (native,
  release, built as the server builds the sim, R5-LAYOUT-REC4) plays a recorded match and writes one hash per tick; the
  gate runs it twice in two processes and once as Wasm in Node (the same runner compiled to Wasm, or wasm-bindgen-test);
  Playwright opens a replay page that runs the `sim-wasm` Worker in Chromium, Firefox and WebKit and returns the hash
  stream; real Safari joins on macOS (REC1's rule). A comparer reports the first mismatching tick and a field-level diff.
  Gate set: three short recordings (proposed: 2 minutes each at the tick rate R1-TICK picks, one per mode: two teams,
  three teams, bots only); nightly: R1-ECS-REC2 mark 1's 10-minute bot match and every recording added since.
  Reason: native and Wasm in Node are cheap and catch most divergence; the browsers must be in the matrix because their
  engines differ (R1-DET-REC6's reason); Playwright drives three engines (F8) and Safari needs macOS (F5, F8).
  Rests on: R5-TEST-F5, F8; R5-LAYOUT-F4; R1-DET-REC6, R1-ECS-REC2 (cited).
  Serves: §6.2, §6.6, §6.8 (re-runnable matches); §3 (the 48-player scale).
  A spike must prove (proposed): R1-DET-REC6's planted divergences (one std `sin`, one `HashMap` iteration, one
  presentation RNG draw inside the sim) are each reported at the first tick they change; the gate's determinism step
  takes under 3 minutes on the CI runner; the nightly comparison of a 10-minute 48-bot match reports 0 mismatches across
  native ×2, Node and three browser engines.
- **R5-TEST-REC3. Golden tests against legacy: captured once from the legacy oracle, committed, read by v2 tests, one
  per kept behaviour.** How: a harness snippet per behaviour runs on the real legacy build through `node tools/preview.js
  legacy eval <snippet>` (TL11) and writes a JSON fixture with the legacy commit SHA, the inventory row id, the inputs
  and the outputs; v2's Rust (or TypeScript, for "port" rows) test reads the fixture and compares within a tolerance the
  fixture states (legacy computes in JavaScript doubles; v2's sim may use `f32`, R1-DET-OQ3). What: only rows the
  inventory marks "rewrite in Rust" or "port to TypeScript" whose rule LOOP_V2 keeps: the ray maths (CS4, CS5, and the
  20,000 seeded random rays of CS6), shot resolution and locational damage (CS7, CD8), the tuning tables imported into
  the content crate (`docs/legacy-tuning.json`, exact for every value v2 keeps: CD4, CD6, CD10–CD14), presence counting
  (GE8), and for the TypeScript animation port the metrics of GA5 and GA8 (drift, hand-on-grip) as thresholds. Not
  golden: anything LOOP_V2 replaces (eras, silos, the tech box), random-AI match statistics (sim.js numbers are
  distributions: compared as ≥ 3 runs per build, TL13), and legacy artefacts such as the frame-rate-dependent jump (GW14:
  OQ3). v2's own outputs (not legacy's) use insta snapshots (F3) for content-validation errors and replay summaries,
  which CI never rewrites.
  Reason: the oracle already runs the real code and produces reproducible dumps (TL11; the tuning dump was byte-identical
  over two runs); a committed fixture survives legacy's retirement; a golden must pin a behaviour we keep, so each names
  its row and its LOOP_V2 status; insta never writes in CI, so a changed snapshot fails (F3).
  Rests on: R5-TEST-F3; legacy TL11, TL13, CS4, CS5, CS6, CS7, CD8, CD12, GE8, GA5, GA8, GW14, `docs/legacy-tuning.json`.
  Serves: §6.3 (abilities and tuning as data start from legacy's numbers), §6.2 (hit and sight maths in 3D start from
  the 2D-plus-height maths we keep).
  A spike must prove (proposed): each golden fails when the v2 value is moved by more than its tolerance (planted); the
  CS6 fixture of 20,000 rays reproduces 20,000 of 20,000 within tolerance; regenerating a fixture from legacy on the same
  SHA gives a byte-identical file; a list of goldens per inventory row is printed by the suite.
- **R5-TEST-REC4. The network simulator in two layers, with R2's profiles as the named set.** L1: a link simulator of
  our own behind R2-TRN-REC4's transport interface, seeded, on simulated time, per direction (half the profile's round
  trip each way, proposed), with latency, jitter, loss, reordering, duplication and a rate limit (F12's set) plus a
  packet-fate log; it runs the real server sim and headless bot clients in one process. L2: an impairment wrapper around
  the server's UDP socket under the real quinn endpoint (F13), with the same profiles, for real browsers over
  WebTransport; WebSocket gets a TCP proxy with delay and stalls (loss on TCP shows as delay). Profiles: R2's P0–P4 and
  B1 by name; every network pass mark of R2 names its profile. Fuzz targets (cargo-fuzz) cover every decoder (R2-TRN-REC4
  mark 4) and the content loader (R1-RR-OQ3).
  Reason: L1 is deterministic and fast, so it can run in the gate and replay a failure from its seed; L2 is the only
  layer that tests QUIC, congestion control (R2-TRN-REC6, R2-TRN-OQ8) and the browsers; a shipping game library puts the
  impairment below the protocol in exactly this set (F12), and quinn takes a custom socket (F13); turmoil fits Tokio
  sockets but QUIC on it is unread (F11); one named set settles R2-XU-OQ12; Fiedler's advice is to test past the
  expected worst case (F14).
  Rests on: R5-TEST-F4, F11, F12, F13, F14; R2-TRN-REC4, R2-TRN-REC6, R2-XU-OQ12, R1-RR-OQ3 (cited).
  Serves: §4 (fair to about 100 ms of ping, P2), §6.1 (rooms survive bad networks).
  A spike must prove (proposed): in L1, the same seed and profile give an identical packet-fate log and identical
  per-tick hashes on two runs; measured loss, mean delay and jitter over 100,000 packets fall within 5 % of the profile
  (relative); in L2, app-level RTT agrees with the profile within ±5 ms at P2 (R2-TRN-REC4 mark 2's form); a 1-minute
  L1 run per profile P0–P4 and B1 fits in the gate's network step, under 2 minutes in total.
- **R5-TEST-REC5. Soak and load at 48 and more, with bots as protocol clients.** Bots are headless Rust clients that
  speak the real protocol (inputs in, snapshots out) so a full room needs no browsers; one nightly run per mode at the
  targets of VISION_V2 §3 (24v24 = 48 and 12v12v12 = 36, plus LOOP_V2 §11's N = 12, 24, 36, 48 scale steps) over L1 at
  P2, 30 minutes each (proposed), and one L2 run with 4 real browser clients among the bots. Measured: server tick p50,
  p99 and max (against R1-TICK's budget), bytes per client per second (against R2-BUD's budget), resident memory slope,
  queue lengths, desyncs (from REC2's hashes on sampled clients), panics and hangs; plus TL13's match metrics (stall %,
  kill matrix, objective counters) for game-designer. Before each playtest, one 3-hour soak (proposed).
  Reason: v2 is designed for 48+ (CLAUDE.md, VISION_V2 §3); legacy's soak and scale habits carry (START_HERE §5, TL15,
  LOOP_V2 §11); an overnight soak passes when nothing hangs or asserts (F14), and we add numbers.
  Rests on: R5-TEST-F14; legacy TL13, TL15, LOOP_V2 §11.
  Serves: §3 (modes and scale), §6.1, §6.6 (the telemetry fields are exercised at full size).
  A spike must prove (proposed): a 30-minute 48-bot room at P2 runs with 0 panics, 0 hangs, 0 desyncs, a memory slope
  within 1 MB per 10 minutes after warm-up, and tick p99 inside the budget ADR-0004 sets; a planted leak (one buffer kept
  per snapshot) is flagged by the slope check within the run.
- **R5-TEST-REC6. Flakes: no retries, fixed seeds in the gate, no wall-clock waits; a failure is a defect.**
  Vitest `retry: 0` and Playwright `retries: 0` (both defaults, F7, F9) are written out; Playwright `failOnFlakyTests`
  and `forbidOnly` on in CI (F9). The gate sets `PROPTEST_RNG_SEED` per run from the commit (proposed: a fixed seed per
  test file) so a red result reproduces; a nightly run uses fresh seeds and any failure it finds is committed to
  `proptest-regressions` with its fix (F2). Sim and network tests run on simulated time only, never `sleep`; browser
  tests wait on state through the test-hook API (TL9), and page time is driven by Playwright's clock where a test needs
  time to pass (F10). Rust tests hold no shared global state, because tests in a crate run on parallel threads (F1).
  Every failure keeps its seed, recording and trace as CI artefacts, gets an issue, and is reproduced with `repeatEach`
  or the seed before it is fixed; a test is never skipped or retried into green.
  Reason: a retry hides the defect (F9's own label for it is "flaky"); a random seed makes a red run unreproducible
  (F2); parallel threads expose shared state (F1); legacy's smoke already waits on state with generous timeouts (TL9).
  Rests on: R5-TEST-F1, F2, F7, F9, F10; legacy TL9.
  Serves: none directly (it keeps every other test's verdict trustworthy).
  A spike must prove (proposed): the gate's whole suite run 20 times on one commit gives 20 identical verdicts; a planted
  timing test (a 50 ms `setTimeout` raced against a fixed wait) is caught by the 20-run check; a planted test that
  passes only on retry fails the Playwright run.

## Open questions
- **R5-TEST-OQ1. Does Playwright's clock reach the sim's Worker?** F10 does not mention workers. The sim's own time is
  the tick counter (R1-TICK), so this matters only for client-side timers inside the Worker. Decides it: the spike.
- **R5-TEST-OQ2. Can wtransport or web-transport-quinn be built on a custom quinn endpoint, so L2 sits under them?**
  F13 is quinn's; the two candidates (R2-TRN-REC1) were not read for it. Decides it: R2's interop spike.
- **R5-TEST-OQ3. A golden for intended behaviour or for legacy's behaviour?** The jump reach is 3.07 m by the formula
  and about 2.98 m as integrated at 60 Hz (GW14, GN10). With a fixed tick v2 gets one value; which one bots and maps
  are tuned for is a rule question. Decides it: game-designer (from the loop slice on), with Mustafa.
- **R5-TEST-OQ4. How long a real-Safari run takes and costs on macOS runners.** Decides it: infra-engineer (R5-CI) and R6.
- **R5-TEST-OQ5. Is turmoil worth adding for the server's own async code (rooms, timers) besides L1?** QUIC over it is
  unread (F11). Decides it: the rooms spike (R2-ROOM), if L1 cannot express its cases.
- **R5-TEST-OQ6. OS-level impairment as a third layer.** Linux `tc netem`'s documentation is outside the allow list;
  L2 covers the same need for UDP. Decides it: only if L2 cannot reproduce a bug seen in a playtest.
- **R5-TEST-OQ7. quickcheck or bolero instead of proptest.** Not read. Decides it: no need unless proptest fails a
  spike mark; proptest is the read option.

| Recommendation that hinges on a source not read | Source | Reason not read |
|---|---|---|
| REC2, REC4 (L2 under the WebTransport crate) | wtransport and web-transport-quinn docs on endpoint construction | not fetched in this run; OQ2 |
| REC4 (OS-level layer left out) | Linux netem documentation (man7.org) | outside the allow list |
| REC1 (proptest over alternatives) | docs.rs/quickcheck, docs.rs/bolero | not fetched in this run |

## Sources read (R5-TEST)
S41, S42, S43, S44, S45, S46, S47, S48, S49, S50, S51, S52, S53, S54, S55, S56, S57, S58 (notes file); repo:
docs/reports/2026-10-14-gate-check-5.md, docs/LEGACY_INVENTORY.md rows, docs/legacy-tuning.json, docs/LOOP_V2.md §11.
Not read: man7.org (netem), docs of quickcheck, bolero, wtransport, web-transport-quinn (not fetched); mutants.rs and
nexte.st (outside the allow list).

## Cross-topic notes
- netcode-architect: REC4 settles R2-XU-OQ12 with R2's own P0–P4 and B1; the per-direction split is proposed; OQ2.
- engine-architect: REC2's replay runner and the recording format (R1-DET-REC6 (1)).
- game-designer: OQ3; REC5's match metrics.
- infra-engineer: Safari on macOS (OQ4), nightly job budget, artefact retention for failures (REC6).
