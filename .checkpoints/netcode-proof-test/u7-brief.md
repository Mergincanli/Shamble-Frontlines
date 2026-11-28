# U7's brief, rev 1.1 (2026-11-23): a plain browser client (the advisor's C1-C14 bind: section 0.1)
**The unit:** U7 as Mustafa's NPT-TRIM keeps it (history.md:7561-7593), item 3 word for word: "U7, cut to a plain
browser client: it connects over WebTransport and WebSocket, sends presses, receives snapshots and shows its counters.
No sim in Wasm, no prediction, no drawing, no hash check: the real game runs the sim as Wasm since change-set 2 and
checks its determinism in change-set 3." His answers of 2026-11-22 (history.md:7595-7607): RE-PLAN-OK yes (reading
(3), replan.md:101-102: the bots' connection code as WebAssembly, no world step), R1 on Thursday 3 December. Its row:
replan.md:18 and replan-units.md §4 (:123-149: Rust 360 on the cargo lane, TypeScript 1,080 off it, about 1,450,
likely 1,950-2,200); the order: replan-units.md §7 and the advisor's lane of advisor-u6c-brief.md section 3 (:267-277).
Short by TEST-FIRST (history.md:2267): what is built, how it is tested, what is open. Reference, never retyped: plan.md
E7 (:485-506, one line later than replan-units.md cites past :379: advisor-u6c-brief.md:18-21), U7's row plan.md:1255,
u6a-, u6b- and u6c-brief.md (interfaces and build rules), advisor-u6c-brief.md (C1 (e), C9), U8's snapshot
wip/u8-build.patch with u8-build.md F2, F14, F15 (its proposed page interfaces), security-r1-deps.md B4, B5, B7.
**One advisor check** (NPT-TRIM), no re-check unless it finds a Critical or High. Section 9's four questions are the
only asks; every other choice goes by its lean, one line each.

## 0.1 Rev 1.1: the advisor's conditions (read first)
The advisor's one check: APPROVE WITH CONDITIONS, advisor-u7-brief.md (352 lines, sha256
65e109aa7732307310bbdee71053d72a96613ac65e3cc277581b4a6506044df3): 0 Critical, 0 High, 6 Medium, 4 Low. Its conditions
C1-C14 (advisor-u7-brief.md:110-247) apply word for word, each replacing or adding to the sections it names; they are
not retyped here (NPT-TRIM: no restated rulings). Its section 3 answers Q-U7-1 to -4, section 5 is U8's fix-pass item
9, and section 6 holds R1's date (about a day of slack, on six assumptions; the trigger: not merged by the end of
Monday 30). Where this brief and a condition differ, the condition rules.

## 0. Where things are
- **Two halves, two bases.** The TypeScript half (T-blocks) builds off the cargo lane once this brief passes, on
  `<base-ts>` = the branch head at launch plus Block 0a (the npm files); it runs no cargo. The Rust half (R-blocks)
  rides the lane after U6c's merge (Sun 29, advisor-u6c-brief.md:272), on `<base>` = U6c's merged head plus Block 0b;
  `cargo check --workspace --all-targets --locked` there, 0 warnings, and check.mjs's counts are filled at launch.
- **Today** (the working tree, 2026-11-22): crates/client-wasm is a stub (src/lib.rs: two doc lines; Cargo.toml:
  npt-client only, `unsafe_code = "allow"`, "wasm-bindgen joins at U7"); wasm-bindgen 0.2.130 and js-sys 0.3.107 are
  already in Cargo.lock (Cargo.lock:663, :1879), so Block 0b adds no package. `ClientEndpoint` (client/src/lib.rs) has
  `input(command, received, now_ns)` (:533-559, no send stamp: plan.md:488 had one), `step() -> StepOut` (:683-755,
  `next_due_ns` from a step count), `with_clock(max_lead)` (:363); the bots pass 9 (bots/src/lib.rs:1305), the lab
  `CONTROLLER_MAX_LEAD` 9 under a compile-time assert against `CMD_CATCHUP + 1` and the room's 400 ms bound
  (lab/src/scenario.rs:60-66). Its `Ping` carries the step's own time (lib.rs:740-743). ClockSync: the offset mapped
  to `i64::MAX` on overflow (clock.rs:163, :209), the next ping by a loop (:122-124), `period_ns` 0 for a rate above
  10^9 (:65). The endpoint never sends `Resync` (no trap recovery here: moved). No `web/` folder exists.
- **What U6a-U6c and U8 give U7, re-read at `<base>`** (file:line in the builder's notes before Block R3): U6a's token
  line `{slot} nptj_<base64url of 16 B>` (wip/u6a-build.patch:753-790) and its private `PROTOCOL_VERSION` 1 and
  `CONTENT_HASH` (:548-551: "U7's page and U6b's clients take the same"); U6b's hash file (u6b-brief.md:60-66), its
  ports UDP 30001 + NN and TCP 31001 + NN (plan.md:909-910), WtLink's framing (control: the client's first
  bidirectional stream, 2 B length, at most 512 B; each FullState on its own server stream with a 4 B length) and
  WsLink's 1 B channel tag (catalogue :73-76, whose values no library holds yet: Q-U7-2); U6c's page host (C1 (e)
  below); U8's URL contract and reads (wip/u8-build.patch:111-132, config.mjs `pageUrl` and `PAGE`; :374-404, lib.mjs
  `read`, `events` and `save`).
- **What R1 asks of U7** (replan-units.md:124-126; replan-marks.md:31, :49, :56, :96): the four-browser smoke,
  "WebTransport · Playing" then "WebSocket · Playing" (plan.md:1214-1223); -36's `event.mds_read.size_bytes` from
  process `browser`, at least 61 reads in 10 min, three browsers at once; -29's `event.fallback.playing_ns` (at most
  3 s from the page starting to connect, 20 attempts); PM-0006-25's `event.attempt.ok` (success as PM-0006-15's,
  "session ready and one echo": marks.json:11463); -14's browser side `second.v.recv_over_max_count`
  (marks.json:3583-3597). Under LAPTOP (b) all are rough readings at R1; R8 judges them. Every name exists in
  SERIES.md (:90, :132, :154, :171): U7 adds none.

## 1. What U7 builds (blocks in order; each compiles with its tests passing before the next)
File and item names are leans; the builder may merge modules and reports the names.
- **Block 0a and 0b, the dependency changes** (the main session's, under DEP-HOLD): section 5.
- **Block T1, `web/` and its build** (spikes/netcode-proof/web; ~170): `package.json` (Block 0a), `tsconfig.json`
  (strict, ES2022 modules, lib dom and webworker), `index.html` (no inline script or style: the CSP of plan.md:1123-1126
  forbids both), `build.mjs` (U8's rs.yaml runs it as `npm_config_ignore_scripts=true CARGO_NET_OFFLINE=true node
  build.mjs` after `npm ci --ignore-scripts`: u8-build.md F14, taken as proposed): `cargo build --locked --offline
  --release --target wasm32-unknown-unknown -p npt-client-wasm` in its own target folder, with RUSTFLAGS remapping
  CARGO_HOME, so the bytes match on RS and here (R1.md's expected dist SHA-256s, filled at Stop B; a test scans the
  Wasm for absolute home paths); `wasm-bindgen --target web --no-typescript` (its version read first, exit 2 unless
  0.2.130); `tsc -p .`; then exactly seven files into a fresh `web/dist/`, each SHA-256 printed. No wasm-opt
  (C-U7-1 went with the hash check).
- **The dist names** (U6c's compiled-in allow-list takes them: u6c-brief.md:126-128): `/` (index.html, `text/html;
  charset=utf-8`), `/main.js`, `/worker.js`, `/lib.js`, `/hooks.js`, `/npt_client_wasm.js` (`text/javascript;
  charset=utf-8`), `/npt_client_wasm_bg.wasm` (`application/wasm`). Plus `/npt/cert-hash-NN.json` (U6c's).
- **Block T2, the page's logic, pure and Node-testable** (web/src/lib.ts; ~420):
  - **The URL contract, U8's as built** (wip/u8-build.patch:111-123): `/?smoke=1&suite=<s>&room=<NN>&transport=<wt|ws>
    &attempt=<k>#t=<token>`; suites `smoke`, `cert-14d`, `fallback`, `mds`, and `wt-attempts` with an optional
    `hash=wrong` control (Q-U7-4); NN 00-31; the token `nptj_` plus 22 base64url characters. A malformed link starts
    nothing and says why on the status line, never echoing the token.
  - **The hash file:** fetched with `cache: "no-store"` from `/npt/cert-hash-<room>.json` (C9); exactly `algorithm`
    `sha-256`, `value_hex` 64 lowercase hex digits (u6b-brief.md condition 4), turned into the 32 bytes of
    `serverCertificateHashes`; anything else refused before any connect.
  - **Framing for the transports:** the control stream's 2 B length and the full-state stream's 4 B length reassembled
    across any chunk split, each capped by Rust's `limits()` (a frame over its cap closes the session, counted);
    WebSocket's 1 B channel tag added and stripped from the same table. Whole framed messages go to the Wasm.
  - **The transport chooser** (plan.md:496-497): `wt` tries WebTransport with a 1.5 s timeout from `hooks.js`, then
    WebSocket; `ws` goes straight to WebSocket; `cert-14d` and `wt-attempts` never fall back (a failed session writes
    `attempt.ok` 0 at once). Status text exactly `<WebTransport|WebSocket> · Playing` (U+00B7, as U8's `PAGE` reads
    it: wip/u8-build.patch:126-132), Playing meaning joined and the client's own soldier decoded from a complete tick
    (`own_soldier()` is some).
  - **The suites' records**, npt-log lines (crates/log/src/lib.rs's `event` and `second` forms, `ph` `measured`, `ns`
    from the Worker's clock) in a page array `window.nptRecords`, at most 4,096 lines (more dropped, counted on the
    page): `attempt.ok` once per attempt (1 at JoinAccepted plus the first ClockSync sample, else 0 at a 10 s
    deadline); `fallback.playing_ns` once, when Playing over WebSocket, timed from the start of the connect sequence
    (the hash fetch's start: never smaller than "starting to connect"); `mds_read.size_bytes` at session ready and
    every 10 s (`hooks.js`); per second `second.v.recv_over_max_count` (datagrams received above the newest
    `maxDatagramSize`). No header and no footer: Q-U7-1. No token, URL or bind value in any record or status.
- **Block T3, the Worker and the page** (web/src/worker.ts, main.ts; ~330): the page reads its URL, starts a module
  Worker (`type: "module"`), posts the link's fields, and draws the status, `#version` (the browser's brand versions,
  else its user-agent string) and a counters table once a second, all by `textContent`. The Worker loads the Wasm,
  makes a 16 B bind by `crypto.getRandomValues` (catalogue :112-115), opens WebTransport (`serverCertificateHashes`,
  `allowPooling` false, `requireUnreliable` true), its datagram writer by `createWritable()` where present else
  `writable` (section 10's facts), its first bidirectional stream as Control and every incoming unidirectional stream
  as a FullState; or `wss://<host>:<31001 + NN>/` with `binaryType` arraybuffer. The game host is the page's own
  `location.hostname`, so one rule serves RS and the loopback test. Every received message is stamped by
  `performance.now()` in its own read (as integer ns), queued (at most 512, more dropped and counted) and handed over
  at the next wake; the timer is set from `next_due_ns`; after each wake's sends the Worker stamps the send time it
  passes to the next step (C-U7-3). `maxDatagramSize` is read as the suites need. A trap ends the session ("Ended"),
  no restart.
- **Block T4, `hooks.js` and the bites dist** (~40): the measured `hooks.js` holds only the 1,500 ms timeout and the
  10,000 ms read period; `node build.mjs --bites` writes `web/dist-bites/` with a `hooks.js` that arms one id from a
  `bite` query: PM-0001-36/read-once (one read, at start) and PM-0001-29/no-wt-timeout (no timeout: the attempt hangs).
  The measured dist holds no bite id (a test scans it). R1 runs no page bite (rough readings); R8's judge needs them.
- **Block R1, C-U7-4** (crates/client clock.rs; ~110 with tests): the offset clamped at `i64::MIN..=i64::MAX` in both
  places; the next ping time in closed form; `period_ns` from `.max(1)`; the public surface trimmed as the carry lists
  (the endpoint and the lab read the rest through `ClientEndpoint::clock()`; each change named in the report).
- **Block R2, C-U7-3 and the cap** (crates/client lib.rs, clock.rs; one line each in lab and bots; ~110): a new
  `ClientEndpoint::sent(at_ns)`: the host's send time of the last step's datagrams; ClockSync keeps each ping's wire
  stamp for matching and takes that send time as its t0. No caller that does not call it changes, so every existing
  test, pinned hash and recording passes unchanged (a moved pin stops the build: U6a's condition 5 rule). The cap:
  Q-U7-3's form.
- **Block R3, npt-client-wasm** (crates/client-wasm; ~380 with tests; plain Rust tested natively, thin wasm-bindgen
  wrappers): `Client::new(token: &str, bind: &[u8]) -> Result<Client, JsError>` (the `nptj_` token decoded to its 16 B;
  the `Join` with the shared version and content hash: Q-U7-2; `with_clock` at the cap); `step(now_ns: u64,
  prev_send_ns: u64, inbox: &[u8]) -> Vec<u8>` (the inbox packed as channel, arrival ns, length, bytes, at most 512
  messages, a malformed record refused and counted; `sent`, `input` with the press script's command, then every due
  step up to `npt_sim::LOOP_CATCHUP` a call, the rest left due; the outbox packed as channel, length, bytes);
  `next_due_ns()`; `counters() -> Vec<u32>` with `counter_names()` as their one home; `limits() -> Vec<u32>` (the
  stream caps from `Limits`, the WebSocket tags). The press script (script.rs): a slow circle, one fire press every
  2 s, one reload press every 20 s, no throw, no melee, from the step count only. A `clippy.toml` as client's (no
  clock, no hash map). u64 crosses as BigInt and bytes as `Uint8Array` with no js-sys.
- **Block R4, check.mjs** (~60): npt-client-wasm joins LEAF (:39) and a deps rule (it ships npt-client and
  wasm-bindgen only); a new `web` step: `tsc --noEmit -p web` and `node --test 'web/test/*.test.mjs'` (U8 F11's glob
  form). If U6b or U6c has not moved the shared constants (Q-U7-2), R4 moves them (two lines each side).
- **Block T5, the browser runs** (web/e2e/; ~220; container readings, reported, never judged): (i) **early contact**,
  the day U6b merges, with U6b's merged release binary (no cargo): a page opens a WebTransport session with the room's
  hash and its control stream, and a `wss` connection, recorded and closed (the room's 2 s Join deadline then closes
  them); (ii) **the whole client**, after R3: section 3's browser list.
- **Choices by lean (no ask; one line each in the day's message):** the Wasm's Rust owns the press script, the
  catch-up and the counters' names, TypeScript owns sockets, framing and records; flat dist names; no Vite, no
  three.js, no @types/node (tests are .mjs over tsc's output); `ph` `measured` on every record; the game host from
  `location.hostname`; no wasm-opt; Playing as `own_soldier()`; the page's two bites now as a constants module.
- **New pub items** (plan §4.7; the netcode seat reads them): npt-client `ClientEndpoint::sent`, the cap constant
  (Q-U7-3); npt-client-wasm `Client` and its five methods, `counter_names`, `limits`; under Q-U7-2 the moved
  constants and tag functions; ClockSync's items made crate-private (fewer, not more).

## 2. What U7 does not build
- **Goes** (replan-units.md:138-139, :140-144): render.ts and three.js, block.ts, the form and record.ts, AimCast,
  BlockWriter, `/iso/`, the Worker restart and rejoin after a trap (so no `Resync` path: C-U6-16 (a)'s browser sentence
  moves with PM-0004-61's online half), the hash check with C-U7-1 and C-U7-2. The `ws://` and wrong-Origin pages
  given to "U7's web harness" (u4s-carries.md:831) are cut with the Origin checks (replan-units.md:19).
- **-28's 100-attempt script** (replan.md:63-64: "a scripted page on the test laptop, after U7"): not U7's and not on
  R1's path; U7's page serves it through `wt-attempts` (Q-U7-4). **-52's R1 parts:** moved whole (replan-marks.md:75:
  "R7's; U7 draws nothing"), so replan-units.md:145-146's flag is settled. **-36's loopback legs:** not run before R1
  (u8-brief.md:296-298).
- **-56 item 2's "negotiated draft"** (marks.json -56's notes): the page cannot read it; the server side would record
  it, and no brief names that yet (for the main session, section 10).
- **U6b, U6c, U8:** the transports, the page host and its allow-list, the WebDriver scripts and the npt-log header
  around the page's records (Q-U7-1).

## 3. How it is tested
- **Rust** (measured and bites builds; no clock, no network): R1 (5): the clamp at `server_ns = u64::MAX` and at a
  negative overflow; the closed-form next ping equal to the old loop on fixed cases, a step 1 s late scheduling the
  first interval past now; `period_ns` 1 at a rate of `u32::MAX` with `next_tick_at` not panicking; f2, f3 and f31
  unchanged. R2 (4): a host whose send lags its step by 5 ms gives an unbiased offset with `sent` and a 2.5 ms bias
  without it; no `sent` call leaves every stamp as today; trace.rs's honest-stamp tests unchanged; the cap's binding
  (Q-U7-3). R3 (9): token decode (good; no `nptj_`; 21 or 23 characters; a non-alphabet byte); a `Client` against
  npt-room's own `Room` in-process (as npt-server's tests host it; Block 0b's dev-dependencies) joins, decodes a
  FullState, files ticks and becomes Playing; a 20-tick wake runs `LOOP_CATCHUP` steps and leaves the rest due; inbox
  and outbox packing round trip; a truncated inbox record refused and counted; `counter_names` and `counters` the same
  length; `limits` equal to `Limits`; the press script's commands draw no refusal but the sim's own (cooldown) over
  3,000 steps. **At least +18 measured and +18 bites.**
- **Node** (the `web` step; a fake transport and a fake core with the Wasm class's shape; fake timers): the URL
  contract (each field good and bad; the token never in the status); the hash file (good; wrong algorithm; upper-case
  or short hex) and its bytes; framing split at every byte offset of a control, a full-state and a two-message chunk;
  a frame over its cap closes; the WebSocket tag both ways; the chooser (wt ready: WebTransport; wt failing:
  WebSocket at once; wt silent: WebSocket at 1.5 s; `ws`: never WebTransport; `cert-14d`: no fallback, `ok` 0); the
  status strings byte for byte; `attempt.ok` (1 at join plus one sample; 0 at the deadline); one `fallback` event;
  `mds_read` at start and every 10 s; `recv_over_max_count` per second; the record cap; every record parses as
  npt-log's `event` or `second` line and holds no token; index.html has no inline script or style; the dist is the
  seven names; the measured dist holds no `PM-0001-` string and no `innerHTML`; the bites dist arms each bite by
  query (2). **About +30 tests**, a new count line.
- **Plants** (scratch edits, one run each, restored and sha256-checked; each must fail its named test): P1 the offset
  mapped to `i64::MAX` again (the clamp test); P2 the next ping one interval after the due time (the late-step test);
  P3 `period_ns` without `.max(1)` (the `u32::MAX` test); P4 `sent` ignored (the lagged-send test); P5 the token
  decoded without stripping `nptj_` (the token test); P6 `LOOP_CATCHUP + 1` steps a wake (the catch-up test); P7 an
  inbox length unchecked (the truncated-record test); P8 the hash bytes reversed (the hash test; T5's smoke); P9 the
  1.5 s timer removed (the silent-wt test); P10 a 1 B control length (the split test); P11 the full-state cap without
  its 4 B (the cap test); P12 one `mds_read` only (the read test); P13 a fallback in `cert-14d` (the no-fallback test);
  P14 the counters by `innerHTML` (the dist scan); P15 a bite id in the measured `hooks.js` (the dist scan); P16 the
  token in the status on a bad link (the URL test).
- **Browser runs** (Block T5 (ii); this container: Chromium 1243 at /opt/pw-browsers/chromium-1243/chrome-linux64,
  driven headless from tools/'s playwright-core 1.56.1 by `executablePath`, a pairing Playwright's docs call "at your
  own risk", read 2026-11-22; if it will not launch, a 100-line CDP driver over 127.0.0.1, else the first contact is
  R1's smoke and the report says so). npt-server from `<base>`'s release build on 127.0.0.1 (settings as U6b's tests
  write them, the room's own certificate for `wss`); the dist from a test server on `http://127.0.0.1` (a secure
  context, as U0 used: u0-probes.md:13) that matches the path up to `?`, serves the hash per room and sends
  plan.md:1123-1126's CSP word for word with `https://127.0.0.1:*` and `wss://127.0.0.1:*` added. Pass marks: (1)
  `smoke` on `wt` shows "WebTransport · Playing" within 3 s, (2) on `ws` "WebSocket · Playing" (the room's own
  certificate admitted only by the test browser's context, `ignoreHTTPSErrors`; never a flag on the laptop:
  plan.md:1222-1223); (3) three `cert-14d` attempts, `ok` 1 each; (4) `wt-attempts` with `hash=wrong`, `ok` 0; (5)
  `fallback` with a silent UDP socket on the room's WebTransport port: Playing over WebSocket, `playing_ns` from 1.5
  to 3 s; (6) `mds`: at least 2 reads in 12 s, each at least 1,000 B (U0 read 1,024 B here); (7) every record valid,
  no token; no CSP violation in the console; (8) the bites dist: read-once gives 1 read in 12 s, no-wt-timeout no
  fallback event in 10 s; (9) the room's log: the page's slot has no kind-10 refusal and nothing counted as forged.
  - **Restated on 26 November 2026 (his U7-MARK8B (a) and U7-MARK9 (a), history.md:8068-8088; the text above is kept
    as it was):** (8)'s no-wt-timeout half reads "a fallback later than 3 s, or none within 10 s". The 3 s is twice the
    page's own 1.5 s timer, not a reading. Restated after measuring Chrome (Chrome for Testing 153.0.8010.12,
    headless; each run's summary.json, m5_fallback and m8_no_wt_timeout_playing_ns): run 3 (25 Nov, the tree before
    the fix pass) fell back at 1.60 s normal (mark 5) and 4.06 s planted; run 4 (26 Nov, the fixed tree, 80b6748 plus
    wip/u7-build-rs.patch 6535c181…) at 1.56 s and 4.07 s. So the planted
    bug is still caught: removing the page's timer moves the fallback past 3 s. (9) is reported NOT-RUN, never PASS,
    with the in-process tests standing in (F3's legs and R3's press-script test). The real mark is read in U6d, the
    unit that adds the room's per-slot refusal list; U6d comes after R1 (replan.md:22, :92), so R1 runs with mark 9
    not run.
- **check.mjs's 11 steps** (fmt, clippy, clippy-wasm32, doc, test, deps, wasm, refs, marks, self, redact) PASS on both
  builds, plus the new `web` step; counts as deltas on `<base>`'s: at least +18 measured, +18 bites, about +30 web.
  The test step's time against `<base>`'s; past 1.5 times it stops (u4f-brief.md section 1 item 5).
- **The report gives:** counts and provenance; each plant's failing test and restore hash; the browser runs' marks
  with the Chromium build; the seven dist SHA-256s; the new pub items; added lines per block against section 8; what is
  not verified (Firefox, Edge, Brave and Safari, the CSP outside Chromium, RS's network, the build's byte equality on
  RS, which Stop B compares).

## 4. The carries, word for word, and the conditions that bind U7
- **C-U7-3** (plan.md:1255; advisor-u4f-r1.md:146-148): "the endpoint takes the previous step's send time, and
  ClockSync stamps each `Ping` with it (NC-U4F-4). The controller's cap gets a source the browser client can reach. My
  lean: a `JoinAccepted` field, since the bound is room data (U4F-Q8 (i) (b)); a wire change goes through the
  catalogue first." Block R2. The cap's source is ruled for no wire change (advisor-u6a-brief.md:93-99): "U7's brief
  puts it where both sides reach it (npt-client does not depend on npt-room) with a test that the client's cap equals
  the room's bound; if the bound turns out to depend on `cmd_timing`, the page's pinned config names the option and
  the header records it. U7's brief check confirms the form." Its form: Q-U7-3.
- **C-U7-4** (plan.md:1255; advisor-u4f-r1.md:149-154): "ClockSync's hardening when it becomes the Worker's: the
  offset clamped, not mapped to i64::MAX (RR-U4F-2), with a unit case at server_ns = u64::MAX; the next ping time in
  closed form (RR-U4F-5 (3)); `period_ns.max(1)` or a clamped tick rate (RR-U4F-5 (4)); its public surface trimmed:
  `ClockSync::new`, `period_ns`, `unknown_pongs`, `next_tick_at` and `one_way_hi_ns` each made crate-private, removed
  or given a consumer, and `ping_every_ns` removed (EA-U4F-1)." Block R1.
- **C-U7-1 and C-U7-2 GO** (replan.md:78-79): "The hash checks (C-U7-1, -2) go with the sim." C-U7-2's tool question
  survives only for the browser runs: Chromium 1243 and playwright-core 1.56.1 are the container's, recorded in the
  report (no CI job runs spikes/).
- **C-U6-19's gate line** (advisor-u6a-brief.md:233-237): "Until U6d merges, U6a's, U6b's, U6c's, U7's and U8's
  briefs and R1's run plan each state in one line that their tests or runs hold no throwing or rejoining hostile
  client and no client restarted mid-run, or bring C-U6-19 first. In R1 each browser leg (each browser on each
  transport, the QUIC-blocked fallback, -36's three) joins its own slot once, or the room restarts between legs; the
  token file holds a slot for every leg. SA-F1-1 stays open until U6d merges." U7's line: **U7's tests and pages hold
  no throwing or rejoining hostile client and no client restarted mid-run: a page joins its own slot once per load,
  never rejoins (the Worker restart is cut), and its press script fires and reloads honestly and never throws.**
- **The hash file** (u6b-brief.md:65-66, condition 4): "U7's page turns `value_hex` into `serverCertificateHashes`'
  bytes." **Its URL** (advisor-u6c-brief.md:222-223, C9): "U7's drafter takes C1 (e): the page fetches
  `/npt/cert-hash-<room>.json` with the `room` of its own query, and the host ignores queries." C1 (e) (:151-152):
  "The page host matches the request target's path (up to the first `?`) exactly and ignores any query string, which
  it never reads or logs". Q-U6C-3 (:237): "**Q-U6C-3 (the hash URL): (a), per room only, no plain name.**" So the
  page never fetches `/npt/cert-hash.json`; U8's fix pass renames its comments (advisor-u6c-brief.md:339-340).
- **npm** (advisor-u8-brief.md:185-189, C2): "No `.npmrc` anywhere under spikes/" and "U7's brief takes the same rule
  for `web/`." This replaces security-r1-deps.md:245's `web/.npmrc` (security confirms the substitute at Stop B).

## 5. Dependencies: Block 0 (named up front; DEP-HOLD)
- **Block 0a (off the lane, the main session, before the T-blocks):** `web/package.json` with `"type": "module"` and
  `typescript` exactly `6.0.3` as its only dependency; `web/package-lock.json` made with npm's `--before` set 7 days
  back (SEC-R1D-6) and `npm_config_ignore_scripts=true ... --ignore-scripts`, no `.npmrc`; every `resolved` on
  https://registry.npmjs.org/ with an sha512 `integrity`; `npm audit`'s output recorded.
- **Block 0b (on the lane, the main session, the hour U6c merges):** (a) spikes/netcode-proof/Cargo.toml
  `wasm-bindgen = "=0.2.130"` (default features only); (b) crates/client-wasm/Cargo.toml `wasm-bindgen.workspace =
  true` in place of its comment line, and `[dev-dependencies]` `npt-room.workspace = true` and `npt-sim.workspace =
  true` (R3's in-process room test; never shipped). **Expected lock diff:** npt-client-wasm's dependency array gains
  ` "npt-room",`, ` "npt-sim",` and ` "wasm-bindgen",` and nothing else (no package, version, checksum or source
  line). **The tool:** `cargo install wasm-bindgen-cli --version 0.2.130 --locked`, default features, CARGO_TARGET_DIR
  and `--root` under .gate/.
- **Cleared already** (security-r1-deps.md:232-238): "wasm-bindgen =0.2.130 (U7, npt-client-wasm only): CLEAR WITH
  CONDITIONS: default features (std) only, no serde-serialize; no web-sys or wasm-bindgen-futures without another
  read; js-sys =0.3.107 allowed (locked)."; wasm-bindgen-cli 0.2.130 by `cargo install --locked`, the `wasm-bindgen`
  binary only; "typescript 6.0.3 exact (U7): CLEAR." Not used: js-sys, web-sys, wasm-bindgen-futures, Vite,
  @types/node, three.js, wasm-opt.
- **The binding check** (security-r1-deps.md:254-255): "U7: the lock gains no package from wasm-bindgen;
  web/package-lock.json holds typescript 6.0.3 (+ @types/node, vite's 19 if kept) and nothing more." Here: typescript
  alone. Plus `cargo tree --locked --offline -e features -p npt-client-wasm --target wasm32-unknown-unknown` showing
  wasm-bindgen's default features only; `cargo check --workspace --all-targets --locked` 0 warnings with check.mjs's
  counts unchanged by 0b; `cargo deny --locked check advisories bans sources` and `cargo audit` clean as U6a's Block 0
  defined it, the database's date recorded; the redact step clean on the new lock (F26's fix: advisor-u6c-brief.md
  section 6, "U7's lock will be the next").
- **DEP-HOLD** (main's .claude/CHECKPOINTS.md:60-62 holds package.json and package-lock.json too): each save waits for
  security-auditor's answer on exactly that content and dependency-check.md. **A short read is needed** for: the two
  npm files as made (0a); the manifest lines, the two dev-dependencies and the three-line lock diff (0b); the CLI
  installed in this container (its clearance names RS's install: :234); the `.npmrc` substitute for `web/`; its yes to
  cargo-deny 0.22.0 and cargo-audit 0.23.0 again.

## 6. The rules that stand (as u6a-brief.md section 6 gives them)
- **Start:** two builders over the unit, never at once on one file: the T-builder (general-purpose, opus, xhigh,
  `isolation: worktree`) from `<base-ts>`; the R-builder the same from `<base>`. First step in each worktree: `git
  merge --ff-only <base…>` (else `git switch -c u7-<half> <base…>`), then `git log -1 --format=%H`; both in its notes
  first. The R-builder merges the T-half's merged head when it starts (T5 (ii) needs both).
- **Notes:** `.checkpoints/netcode-proof-test/u7-build-ts.md` and `u7-build-rs.md` in the worktrees: the task, the
  blocks, each finished block at once, a notes line per finding, added lines after each block.
- **The shared target** `/home/user/Shamble-Frontlines/.gate/shared-target` and `CARGO_NET_OFFLINE=true` for every
  cargo call, through a U7 runner (the main session's names-only copy of u6c-run.sh, logs under .gate/u7/), the lane
  lock and u4f-brief.md section 4's disk floors; build.mjs's wasm32 build in its own target under .gate/. The T-builder
  runs no cargo; it runs U6b's merged release binary copied under .gate/u7/ for T5 (i).
- **Files:** the T-half's lane is spikes/netcode-proof/web/ (but its two npm files, Block 0a's). The R-half's lane is
  crates/client-wasm (all of it but Cargo.toml), crates/client (lib.rs, clock.rs, tests), the cap's one line in
  crates/lab/src/scenario.rs and in crates/bots/src/lib.rs, check.mjs (LEAF, the deps rule, the `web` step), and
  under Q-U7-2 only, crates/proto, crates/link, crates/net and crates/server/src/main.rs (the moved constants and
  their uses). Not U7's: every Cargo.toml and Cargo.lock (Block 0b's), marks.json, marks/README.md, SERIES.md, ops/.
  A part that needs one stops, is reported, and the rest is built.
- **Bans:** `.claude/rules/netcode.md`; no clock and no hash map in Rust (client's clippy.toml); no `innerHTML`,
  `eval`, `Function` or inline script or style; nothing fetched or connected but the page's own origin and the room's
  two ports; no token, URL or bind value in any record, status, console line or log; every queue in the Worker bounded;
  `Date.now` in nothing recorded (one Worker clock, `performance.now`); tests on 127.0.0.1 only; LEAD-BOUND frozen at
  400 ms.
- **SIZE-CHECK** (history.md:2048-2051): past 6,000 added lines the build stops, the end checks run on the tree as it
  stands, and the main session gives Mustafa three lines (what grew, why, the cheaper option).
- **Agents never commit;** the main session snapshots, merges and commits. Each builder's brief carries, word for
  word: "Write files with the Write and Edit tools only (no sed, no shell redirect, no python writes; never `cargo fmt`
  writing files: apply rustfmt's diff by Edit); an agent with no Edit tool rewrites its notes file whole with Write."

## 7. Review
One round (NPT-TRIM) over both halves, after R4 and T5 (ii): netcode-architect and test-engineer, each at xhigh; a
second only for an open Critical or High. **Security does not sit (lean):** no U7 code listens or faces the internet in
a run (replan.md:93-94 seats security on U6b, U6c and U6e); its parts are Block 0's short reads, and Stop B's read of
what the host serves. The netcode seat reads the token and bind handling, the CSP fit, the pub items, the framing caps
and C-U7-3/-4; the test seat the plants, the fakes' fidelity and the browser runs.

## 8. Size, and R1's date
- **The re-plan's figure:** about 1,450 (Rust 360, TypeScript 1,080), likely 1,950-2,200 (replan-units.md:147-149).
- **This brief's:** about 2,200. Rust about 660 (R1 110, R2 110, R3 380, R4 60) and TypeScript and JS about 1,550 (T1
  170, T2 420, T3 330, T4 40, Node tests 370, T5 220). Over the re-plan by -28's suite, the page's two bites, the
  reproducible build, the catch-up and packing tests, check.mjs's `web` step and the browser runs, none of which the
  re-plan counted. **Likely 2,950-3,300** at the re-plan's 1.35-1.5, **4,200-5,050** at the 1.9-2.3 the last units
  grew (advisor-u6c-brief.md:262-264); the stop past 6,000. On the lane: about 660, likely 900-1,500.
- **R1 on Thursday 3 December holds on the advisor's lane** (advisor-u6c-brief.md:267-277): building is fast; the days
  go to Block 0, end checks, rounds and merges. U7's needs, in order: (1) the dist names now, so U6c's allow-list is
  built with them (Thu 26); (2) the T-half Mon 23-Thu 26 and T5 (i) the day U6b merges; (3) Q-U7-1 ruled before U8's
  fix pass ends, so Stop B reads an aligned `save`; (4) Block 0b the hour U6c merges (Sat 28), the R-half Sun 29, the
  round and merge by the end of Mon 30, so R1.md's expected dist SHA-256s come from the merged commit before his Stop
  B steps Tue 1. **Trigger:** U7 not merged by the end of Mon 30 sends him three lines that day (the split R1 of
  replan.md:45-46, browser blocks later, is the closer option). **Not earlier on the lane:** each slot before U6c moves
  U6c's merge, from which the Stop B schedule runs, and three reviewers fill the agent seats in U6b's and U6c's rounds;
  T5 (i) gets the browser's first contact early without a lane slot.

## 9. Open questions (each with my lean; the advisor's check answers them)
- **Q-U7-1, who writes the browser logs' header.** U8's `save()` writes the page's lines as they are
  (wip/u8-build.patch:398-404), but an npt-log file must open with a header (crates/log/src/lib.rs:585) whose
  `machine` has 13 required fields the page cannot know, and -36's parts select `params.browser`, `params.path` and
  `machine.name` (marks.json:7337-7356). (a) U8's `save()` writes the header (process `browser`; `machine` from the
  laptop with name `ML*`; run, set, date, `params` suite, browser, path, transport, room, attempt and the page's
  version) and the footer around the page's lines, one item added to U8's fix pass (advisor-u6c-brief.md section 5
  says "nothing else"); (b) the page writes a header with placeholders that `save()` rewrites. **Lean (a):** the
  laptop alone knows the machine and the run; the page keeps to what it measured.
- **Q-U7-2, the shared constants' one home.** The `Join`'s version and content hash are private to U6a's binary, and
  WsLink's channel tags will be private to npt-net, which wasm32 cannot reach; U6b's clients, U6c's npt-load and the
  page must send the same values. **Lean:** `npt_proto::PROTOCOL_VERSION` and `CONTENT_HASH`, and
  `npt_link::Channel::ws_tag` with `from_ws_tag`, made by the first unit that needs them in a library (the main session
  names it in U6b's launch prompt); else R4 moves them. No value changes, so no wire change.
- **Q-U7-3, C-U7-3's cap form** (the ruling asks this check to confirm it). The controller's cap is not the room's
  bound but the bound less a margin, at most 9 (client/src/lib.rs:354-359): 9 ticks against the room's 12 (400 ms at
  30 Hz). **Lean:** `npt_client::HONEST_MAX_LEAD = 9`, read by the Wasm, the bots (bots/src/lib.rs:1305) and the lab,
  whose compile-time assert (scenario.rs:60-66: at most `CMD_CATCHUP + 1` and below the room's bound in ticks) then
  binds the client's cap to the room's bound; I read the ruling's "equals" as that binding. It holds under both
  `cmd_timing` options (the lab passes 9 under both: lib.rs:358-359), so the page names no option.
- **Q-U7-4, -28's carried-crate parts.** replan-units.md:138 lists connect.ts as gone with -28, while replan.md:63-64
  and replan-marks.md:48 keep -28's carried-crate parts as "a scripted page on the test laptop, after U7". **Lean:**
  the page takes suite `wt-attempts` (attempt rules as `cert-14d`, which already cover "datagrams + one bidirectional
  stream", SERIES.md:132) and the `hash=wrong` control, about 30 lines; the 100-attempt laptop script waits for R8's
  plan and stays off R1's path.

## 10. For the main session
- **Now:** this brief to the advisor (one check); security-auditor's short read of Block 0a and 0b beside it; Block 0a
  and its save, then the T-builder; U6c's builder told the seven dist names and their types (section 1); U6b's launch
  prompt names Q-U7-2's homes once ruled; U8's fix pass takes Q-U7-1 once ruled.
- **Browser facts this brief rests on** (read 2026-11-22): the W3C WebTransport draft: "[Exposed=(Window,Worker),
  SecureContext]", certificate hashes need X.509v3, ECDSA P-256 and no RSA, and "the total length of the validity
  period MUST NOT exceed two weeks" (U6b's 13 days passes); MDN's WebTransport() page: hashes need `allowPooling`
  false, "This feature is available in Web Workers"; MDN's browser-compat data: WebTransport Chrome 97, Firefox 114;
  `serverCertificateHashes` Chrome and Edge 100, Firefox 125; `requireUnreliable` Firefox 114 and not Chrome (passing
  it is harmless: our server speaks HTTP/3 only); `maxDatagramSize` Chrome 97, Firefox 114; `datagrams.writable`
  deprecated, `createWritable` Chrome 159, Firefox 155 (hence the feature check); module Workers Chrome 80, Firefox
  114; MDN's Using Web Workers: a Worker's CSP is its own script's response header (the host sends the CSP on every
  answer, so the Worker gets it); MDN's connect-src page lists `fetch()` and `WebSocket`, not WebTransport, and warns
  that `'self'` misses websocket schemes in some browsers (the plan's CSP names `wss://` explicitly).
- **Before the R-half (the hour U6c merges):** Block 0b, the tool, the binding check, dependency-check.md, the save;
  fill `<base>` and its counts; the runner copy u7-run.sh; the size line on the card.
- **Close:** both builders' notes; snapshots; check.mjs in a merge worktree; the round; fixes; merge by the end of Mon
  30; U7's report (counts, plants, the browser runs, the dist SHA-256s for R1.md, size, carries by number, Block 0's
  diffs and audits). Flag for a later brief: who records -56 item 2's negotiated draft per browser.
