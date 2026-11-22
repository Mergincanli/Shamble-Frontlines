# U6c's brief, rev 1 (2026-11-22): the probe, the load, the page host and the certificate
**The unit:** the third piece of U6 as Mustafa's NPT-TRIM cut it (history.md:7561-7593; his answers of 2026-11-22,
history.md:7595-7607: RE-PLAN-OK yes, R1 on Thursday 3 December). Its row: replan-units.md:60 (P1, P2, B1, H1, H2:
1,950 estimated, 2,650-2,950 likely), first needed by R1 for the L2 probe, the load, the pages and the certificate.
Built on the one cargo lane after U6b's merge (replan-units.md §7: about Fri 27-Sat 28 Nov; this brief's day line:
section 8), on U6a and U6b. Short by TEST-FIRST: what is built, how it is tested, what is open. Reference, never
retyped: u6-brief.md rev 2.1, u6-facts.md, advisor-u6-brief.md, U6's row plan.md:1254 (replan-units.md:3 cites
:1253), u6a-brief.md and u6b-brief.md rev 1.1 (their interfaces and the build rules), u8-brief.md rev 1.1 section 0.1
(what U8 drives), security-r1-deps.md (the R1 path's dependency read, DONE).
**One advisor check** (NPT-TRIM), no re-check unless it finds a Critical or High. Section 9's eight questions are the
only asks (Q-U6C-5 is the read of new npt-log names that Q-U8-5 sends to this check); every other choice goes by its
lean, one line each (section 1).

## 0. Where things are
- **`<base>`** (filled at launch): U6b's merged head plus Block 0's commit (section 5); its spikes/netcode-proof tree,
  check.mjs counts (measured, bites) and `cargo check --workspace --all-targets --locked` there, 0 warnings.
- **U6a's and U6b's interfaces, re-read at `<base>`** (each with file:line in the builder's notes before Block 1):
  `StealReader` and the vCPU it reads (u6a-brief.md:144-148, pub(crate)); `npt_log::Machine::read_local` (:140-143);
  the settings rules (SEC-U6AD-2) and the clock item, `server/src/clock.rs`'s `now_ns` (:129-130); main.rs's
  subcommand and exit-code shape (:131-139); `Behaviour::host_step` (:163-165); the token file's path and format
  (:133-136); `WtClient::connect` and `WsClient::connect` with the pin, `Box<dyn Link + Send>` and `Recv::arrival_ns`
  (u6b-brief.md:128-171); the hash file's form and the settings field naming its path (:60-66, :207-214); the chain
  and key formats WsServer's loader reads from npt-cert's files (:210-212); ImpairSocket as U6b leaves it.
- **Today** (read at 7ae83dd; U6a and U6b change none of it): npt-host and npt-load are stubs (host/src/main.rs:1-8,
  bots/src/bin/npt-load.rs:1-6); npt-host depends on npt-log alone (host/Cargo.toml:16-18), and check.mjs keeps it off
  sim, room, proto and client (check.mjs:101-103); npt-bots already depends on npt-net and bans the clock crate-wide,
  "U6's runtime half adds its reviewed allows (§4.11)" (bots/clippy.toml). The L2 probe's home and design are written
  down: SERIES.md:497-501 (npt-server's binary in a probe mode, process `probe`, a free slot port with its own
  ImpairSocket, the pacer a dedicated thread with 1 ns slack, LG's side sending up and echoing) and
  lab/src/probe.rs:1-11 (U2's L1 probe: "L2's probe (U3, U6) keeps this design"); -3's seven `second.s.probe_*` rows
  name the L2 writer (SERIES.md:111-117). What the probe must compute is test code today: the poller
  (socket_tests.rs:676-713), the samples and E forms (:715-810), the pre-touch (:1384-1406, through the crate-private
  `fates` field) and the fate entries (stats.rs:426-505: `Fate` crate-private, `FateLog::entries` cfg(test); the
  canonical 56 B form :446-466). Timer slack is set through /proc, no unsafe code (thread.rs:1-60). C-U6-8's stamp
  sites: socket.rs:501-503 (up) and :724-726 (down).
- **What R1 asks of U6c** (replan-marks.md:106; u8-brief.md:389-402): -3's L2 parts P0-P4 and B1 (process `probe`,
  1,000,000 a direction per leg: marks.json PM-0001-3) with ImpairSocket's R1 self-test (E with L0 = 0 at most 10,000
  ns a direction over P1's measured records: u8-brief.md:356-358) and C-U8-2's, -4's and -6's reads; the load for the
  headers block (-1's LG part), calibration (lg-80 and V-S's inputs rule: marks/README.md:269-276) and DelayStream's
  loaded self-test (48 WebSocket clients at P2: C-U8-3); the page host for the four-browser smoke, PM-0006-25, -29 and
  -36; the WebSocket leg's browser-trusted certificate, staging then production (replan-units.md:117-118).

## 1. What U6c builds (blocks in order; each compiles with its tests passing before the next)
File and item names are leans; the builder may merge modules and reports the names.
- **Block 0, U6c's manifests and lock** (the main session's, on the lane before `<base>`): section 5.
- **Block 1, P1a the probe's shipped pieces** (crates/net, new l2/probe.rs; ~560 with tests).
  - **Promoted** from socket_tests.rs into shipped code that the tests then call, so T1, T2 and T5 re-run on the
    probe's own code (C-U6-1, C-U6-5): the poller (`start_poller`: a task on the caller's multi-thread runtime, never
    `block_on`, `poll_recv` with quinn's 32 buffers of 1,472 B until Pending, each datagram handed to the caller's
    closure); a pub `FateEntry` parsed from `drain_fate_log`'s canonical bytes (the tag checked, 56 B each); the
    samples and both E forms (`samples`, `nearest_rank`, `quantile`, `excess`, `forms`, as built).
  - **New:** `ImpairSocket::pretouch_fate_log()` (fills the log to its capacity and drains it, the self-test's
    pre-touch made pub; returns the count touched: C-U6-3); `ProbeDatagram` (leg, index, echo index, echo hold ns; one
    fixed length; another length or leg refused and counted), one source for both ends; `Pacer` (a dedicated std
    thread that sets its own slack to 1 ns through thread.rs, reads it back, and calls its closure at
    `start + i × interval` as lab/src/probe.rs:113 schedules; never tokio: SERIES.md:500); `probe_peer` (LG's side: a
    std UDP socket on an ephemeral port sending up datagrams to RS from a `Pacer`, each echoing the newest down
    datagram it has received with its hold on LG's clock, a reader thread, a stop; it writes no probe series:
    SERIES.md:501).
  - **The leg's series from its fates** (`leg_series`): for each fate whose t_in lies in the measured window,
    `probe_up_lost` or `probe_down_lost` 0 or 1 (inside B1's burst, `probe_burst_*`); `probe_*_delay_ns` = out − t_in
    of Delivered fates only (socket_tests.rs:735-750); `probe_rtt_ns` = the release of the up datagram carrying the
    echo (its Delivered up fate, joined in delivery order, since `poll_recv` hands datagrams over in release order)
    minus the echoed down datagram's t_in (its down fate, by flow index) minus LG's hold (SERIES.md:499-500), filed as
    SERIES.md:115 files L1's.
- **Block 2, P2 C-U6-8** (crates/net socket.rs, socket_tests.rs, marks/SERIES.md's down rows; ~170): each of TE2-TE4's
  plants is run first against the suite, and one that already fails a test by name drops out (the carry's last
  sentence); for each other, one test that fails on it: down's stamp taken at the pop (a test-only delay in the release
  path, so a stamp before `try_send` returns shows), up's clock read before the lock (the lock held between the two),
  `agree()` on its four input pairs. T7's doc (socket_tests.rs:1263-1265) and SERIES.md's down rows are reworded to
  claim only what a test then proves.
- **Block 3, P1b the probe mode** (crates/server new probe.rs and main.rs; crates/bots npt-load's `probe`; ~580 with
  tests): `npt-server probe <settings.json>` (SEC-U6AD-2's rules): the port (UDP 30001-30032, a free slot port:
  SERIES.md:498; R1.md names it, so U8's firewall lists stay as they are: u8-brief.md:203-204), LG's IPv4 (the one
  peer, pinned by `set_peer`; any other source dropped and counted), the legs from {P0, P1, P2, P3, P4, B1} in that
  order, the seed, packets a direction (default 1,000,000; B1 1,010,000 with its burst 50 s into the window, as
  lab/src/main.rs:394), the warm-up (lean 10 s), the log directory and the header's fields; one log per leg (process
  `probe`, layer `L2`, its profile).
  - **Base RTT** (ADR 0001:1590; config.rs:24-26): P0 (pass-through) runs first; the median of its first 10 s of round
    trips is every later leg's `base_rtt_ns`, written in each header; no echo within 30 s ends the run non-zero.
  - **Per leg** (C-U6-2): a new ImpairSocket bound on the port with the leg's L2Config (its fate log sized for
    warm-up, window and tail: about 2.6 million entries, about 146 MB at P1), `pretouch_fate_log`, the start-edge
    reads, the pacer (down) and the poller (up) on the server's multi-thread runtime, warm-up, the measured window, a
    tail (the profile's longest one-way delay plus 1 s), the poller stopped, the end-edge reads with the socket still
    bound, `run_summary` as the leg's `l2_summary` event with C-U6-5's fields added (section 4), one drain (its lock
    hold measured: C-U6-3), the series written; then the socket dropped. **Nothing is written to the log inside the
    window:** `take_second` once a second and the steal read once a second go to fixed-capacity buffers made before it.
  - **C-U6-4:** `second.v.steal_permille` each second through U6a's `StealReader` (the probe sits in npt-server's
    library, so it stays pub(crate): u6a-brief.md:52-53), over the vCPUs the probe process may use, read once at start
    from /proc/self/status and written as `params.steal_cpus` (no tick thread here; R1's `npt-ctl cpus` keeps the tick
    vCPU out of that set).
  - **C-U6-6:** LG's up stream is 10,000 fresh datagrams a second, each carrying one echo (the newest down datagram and
    its hold), so the probe's socket sees 10,000 a second each way and no echo datagram of its own; each leg records
    its window's datagrams per direction (the rate check is U8's probe block's, C-U8-4: u8-brief.md:363-364).
  - **The leg-edge reads** (Q-U8-5; plan.md:1256's C-U8-2 and C-U8-6): before and after every leg, outside the window
    and while its socket is bound: every `^cpu` line of /proc/stat; the socket's own row of /proc/net/udp, matched by
    its local port, with its `drops` column (a missing row is a failed read, written as such, never 0); /proc/net/snmp's
    Udp lines (and /proc/net/snmp6's Udp6 lines when present). Their names: section 4.
  - **LG's side:** `npt-load probe <settings.json>` runs Block 1's `probe_peer` until stdin closes (npt-load-ctl holds
    it: u8-brief.md:65-71), then prints one summary line.
- **Block 4, B1 npt-bots' npt-net glue, npt-load and calibration** (crates/bots lib and bin; ~700 with tests).
  - **The glue:** one client is one honest `Behaviour` with its endpoint, hosted through U6a's `Behaviour::host_step`
    (C-U6-18) over a `Box<dyn Link + Send>` from `WtClient::connect` or `WsClient::connect` with the pin (the room's
    hash file, copied to LG by npt-load-ctl), each received message fed with its `Recv::arrival_ns`; one tokio task a
    client on a multi-thread runtime; waits by `tokio::time::sleep` for the time left to the endpoint's next due time
    (`tokio::time::Instant` is banned: bots/clippy.toml); one clock item (`now_ns`) with the reviewed allow that file's
    comment asks for; every other time passed in. No rejoin: a client whose link closes is counted and stays out.
  - **npt-load's interface** (what U8's npt-load-ctl drives: C4): `npt-load run|calibrate|probe <settings.json>`, with
    the settings, token file and pin file at the fixed paths npt-load-ctl writes; settings by SEC-U6AD-2's rules (RS's
    IPv4; the slot NN, so UDP 30001 + NN and TCP 31001 + NN; transport `wt` or `ws`; clients 1-64, 48 at R1; the
    Behaviour from a fixed list; seed; duration; log directory; the header's and LG's machine fields); the token file
    read under a size cap, one token a slot, never logged (SEC-U6AD-3's reader side); `READY` on stdout once every
    client holds its `JoinAccepted` (else exit 1 after 30 s); the end at the duration or when stdin closes (npt-super's
    hold form: u6a-brief.md:37-40); one `DONE` line; exit 0, 2 (bad settings) or 1.
  - **Its log** (process `load`, machine role `LG` from `Machine::read_local`, so -1's LG part has its header):
    `second.s.inputs_count` each second, one value a client in slot order (SERIES.md:12, :432), and
    `second.v.cpu_permille`, LG's whole-machine CPU from /proc/stat (SERIES.md:433).
  - **Calibration** (plan.md:473, E13): `calibrate` is `run` for 5 minutes with one summary line (the largest
    `cpu_permille`, the seconds at or over 800, each client's inputs a second, smallest and largest); the verdict is
    lg-80's and V-S's own rule (marks/README.md:269-276), read by R1.md's block, so npt-load holds no second copy of
    the judge.
- **Block 5, H1 the page host, trimmed** (crates/host; ~440 with tests): `npt-host serve <settings.json>` (SEC-U6AD-2's
  rules): the bind address (TCP 31000), npt-cert's chain and key (read once at start under size caps), the dist
  directory, the hash files' path template (NN) and the host-key file's path; TLS 1.3 only, ALPN `http/1.1`, ring by
  `builder_with_provider` as ws_config.rs:27-38 does (no process default).
  - **Files:** a compiled-in allow-list (plan.md:1118) of U7's dist names (the index at `/`, the scripts, the Worker,
    the Wasm as `application/wasm`), each read into memory once at start (a missing one is exit 2), never a path built
    from the request; `/npt/cert-hash-NN.json` for NN 00-31 and `/npt/cert-hash.json` as room 00's (Q-U6C-3), each
    read per request from that room's hash file under a 1 KiB cap and served verbatim with `Cache-Control: no-store`
    (U6b's condition 4; an absent file 404, an oversized one 500, never cut short); `/npt/ssh-host-keys.txt` the same
    way under 4 KiB (plan.md:289; Q-U6C-6).
  - **HTTP:** GET and HEAD only (405 otherwise); the request target matched exactly (anything else 404,
    `/../Cargo.toml`, `%2e%2e` and `/npt/` among them); one request a connection, then close; httparse over one fixed
    buffer a connection, a request that does not fit closed (Q-U6C-4); on every answer the CSP of plan.md:1123-1126
    word for word, `X-Content-Type-Options: nosniff` and `Referrer-Policy: no-referrer`. §10.4's limits, its bite,
    `/iso/`, robots and noindex are cut (replan-units.md:13-14, :26-27).
  - **Logs:** no request log (no client string anywhere: E22); one stderr line at start and on a fatal error. It runs
    until killed; its start verb is U8's (Q-U6C-7).
- **Block 6, H2 the DNS-01 issue mode** (crates/host; ~380 with tests): `npt-host issue <staging|production> <dir>`
  (the word from a fixed list; `<dir>` absolute, existing, owned by the caller and writable by no one else; nothing else
  accepted); the directory URL is instant-acme's `LetsEncrypt` constant for the word (SEC-R1D-4); the name
  `rs.nt.shamblefrontlines.com` compiled in (the spare `rs-b` name joins only if a run plan needs it).
  - **One process, two phases** (Q-U8-4). Phase 1: a fresh account by `Account::builder()` (its key in memory only,
    never on disk); the directory's profile list printed (plan.md:970-971's record); production orders only if
    `shortlived` is listed (else exit 3, before any order) and with `NewOrder::profile("shortlived")`; staging takes
    `shortlived` if listed and the default otherwise, and says which (plan.md:966-971); one order, its one
    authorization's DNS-01 challenge; prints `_acme-challenge.<name>` and its `dns_value`; waits for one line `go` on
    stdin (read on a std thread under a byte bound; anything else is exit 2), inside one 30-minute deadline from the
    start that covers both phases. Phase 2: `set_ready`; `poll_ready` under a bounded `RetryPolicy` (Invalid or a
    timeout is exit 4, never retried: plan.md:972-974); a fresh ECDSA P-256 key and its CSR by rcgen, then
    `finalize_csr`; `poll_certificate`; the key (PKCS#8, mode 0640 set at creation) and the chain as the server sent
    it (mode 0644) written by `create_new` (an existing path is exit 2), each `write_all` then `sync_all`, in the
    formats WsServer's loader reads (section 0); the leaf's serial and DER SHA-256 printed for the ledger
    (plan.md:977-982); `DONE`. Every ACME call under `tokio::time::timeout` (SEC-R1D-4; lean 60 s each).
  - **Not built:** revocation (cut: replan-units.md:32, :118-119), so U6-Q18's "confirms instant-acme 0.8.5's
    revocation call by name" lapses with it; the HTTP-01 responder and port window (cut).
- **Choices by lean (no ask):** the probe's shared pieces in npt-net, its leg runner in npt-server (Q-U6C-1); P0 first
  for the base RTT; steal over the probe process's allowed vCPUs; no log write inside the window; the probe's port from
  settings (R1.md's pick; lean 30032, slot 31's); LG's peer in npt-net, called by npt-load, so npt-server's tests run
  both ends; npt-load's end-to-end tests with npt-server as npt-bots' dev-dependency (section 5); no rejoin in
  npt-load; npt-load reports and the judge's rule decides calibration; the host's files in memory from the start; one
  request a connection; no request log; rcgen's key and CSR with `finalize_csr` (instant-acme's `finalize()` is behind
  its `rcgen` feature, which the cleared set need not turn on); `Account::builder()`'s default client (its root store
  is security's Block 0 read: U6-Q18); 60 s a call; the files named `<word>.key.pem` and `<word>.chain.pem`.
- **New pub items** (plan §4.7's whole-surface rule; the netcode seat reads them): npt-net `start_poller`,
  `FateEntry`, `samples`, `forms`, `excess`, `nearest_rank`, `quantile` (with `Samples` and `Forms`),
  `ImpairSocket::pretouch_fate_log`, `ProbeDatagram`, `Pacer`, `probe_peer`, `leg_series`; npt-server `probe`
  (`ProbeSettings`, `run`); npt-bots the glue (`Hosted`) and `load` (`LoadSettings`, `run`). npt-host is a binary only.
  Each named in the report with its reason.

## 2. What U6c does not build
- **U6a, U6b (merged first):** TickLoop, the binary, `StealReader`, `Machine::read_local`, `host_step`, C-U6-17; the
  transports, their clients and the pin, `ConnCap`, the dev certificate and its hash file (U6b condition 4).
- **U6d:** C-U6-13, the rest of -14, -16 (a), -19 with `JoinAccepted`, the interest writers, C-U6-15, rate adaptation.
  **U6e:** ControlSocket, metrics and logs, the per-second merges, npt-super's density harness, -30's runner (it reuses
  Block 4's glue). **U6j:** the judge, C-U6-9 to -11.
- **U7:** the page, its Worker and dist (the names Block 5's allow-list takes from U7's brief), its fetch of the hash.
- **U8:** `npt-cert issue` (the egress window, `dig +norecurse` against every authoritative server, the "go"),
  `npt-load-ctl` and RS's key on LG (C4), r1.sh's `probe`, `calibrate` and `ds-selftest` blocks, the probe's port in
  R1.md, the hash file's removal before a room restart, the key file's group, the host's start verb (not in u8-brief.md
  rev 1.1: Q-U6C-7).
- **MOVED's brief:** the TCP model's burst knob (C10: "not the knob"); P2 too, if lever (1) is pulled (Q-U6C-8).
- **Cut or elsewhere:** the host's limits, 404 bite, `/iso/`, robots and noindex (replan-units.md:13-14, :26-27);
  HTTP-01's responder, npt-cert's port window and `revoke` (:32); E14's floods (:11-12). **No bite:** -3's bites are
  npt-l1's, built at U2 (l1/src/lib.rs:780-782), and `not-net-of-base` is MP's part, moved with R7 (replan-marks.md:20).

## 3. How it is tested
- **Unit and loopback tests** (127.0.0.1 only; no network at all; injected clock wherever a time is asserted;
  real-clock tests at most 5 s each, no timing bar in the container):
  - Block 1 (10): `FateEntry` from canonical bytes equal to the entries, a bad tag or length refused; two legs on two
    sockets give two summaries that never mix (C-U6-2); the pre-touch fills and empties to capacity and the drain's
    hold is measured (C-U6-3); C-U6-5's fixed fate entries (a late datagram in the warm-up stays out of E; one inside
    the window enters E with exactly its excess over n in both forms; T5's samples give 10,000 and 10,100 ns through
    the shipped forms); `leg_series` (one loss flag a fate, delays of Delivered fates only, B1's burst apart, the RTT
    join with the hold taken off); `ProbeDatagram` round trip and refusals; `Pacer` reads back 1 ns; `probe_peer`
    echoes the newest down with its hold. T1, T2, T5 and T7 then run through the shipped code (existing tests).
  - Block 2 (0-3): C-U6-8's, as many as the drop-out rule leaves.
  - Block 3 (8): settings accepted and refused; on fixed texts, /proc/net/udp's row by local port (present; missing as
    a failed read), /proc/net/snmp's Udp lines (with and without the newer columns), /proc/stat's `^cpu` lines; one
    end-to-end probe on loopback with `probe_peer` (P0 then P1, 2 s each at 2,000 a second): both edges' events
    present while the socket is bound, the steal series each second, P1's header carrying P0's measured base RTT, the
    window's counts per direction, C-U6-5's fields in P1's `l2_summary`, no log write inside the window (a test-local
    counting writer); C-U6-7 by test (lean: the window's loop reaches the socket only through a narrow wrapper the
    test counts: `take_second` once a second, nothing else).
  - Block 4 (10): the glue hosts 4 honest Behaviours over WtClient and over WsClient against an in-process npt-server
    room on U6b's transports at P2: all join, snapshots arrive, no kind-10 refusal (C-U6-17's form), inputs counted per
    client a second; settings accepted and refused; the token file read and in no log; `cpu_permille` on fixed
    /proc/stat texts; `inputs_count` in slot order; stdin closing ends `run` with exit 0 and its `DONE` line;
    calibration's summary on fixed records; LG's header with role `LG`.
  - Block 5 (10): every allowed path over TLS with its exact bytes and headers (the CSP string, nosniff,
    Referrer-Policy, `application/wasm`); HEAD; the 404 targets; 405; the hash file verbatim with `no-store`, per NN
    and as room 00's at the plain name; a replaced file served anew (nothing cached); absent 404, oversized 500; the
    host-key file; a request past the buffer closed; settings refused; TLS 1.3 and the one ALPN.
  - Block 6 (9), all against an in-process fake directory (a test `HttpClient` given to `Account::builder_with_http`:
    U6-Q18's term; no network): the whole order printing the TXT name and value, then on `go` the key 0640 and the
    chain 0644 by `create_new` and the serial and hash printed; the fake asserts the production order's `profile` is
    `shortlived`; production with no `shortlived` listed exits 3 before any order and writes nothing; staging without
    it orders with the default and says so; an Invalid order exits 4 after one poll sequence, nothing written; no `go`
    by a test deadline exits non-zero, nothing written; a call that never answers ends by its timeout; an existing
    output path exits 2; a word outside the list exits 2.
  - **At least +47 measured** (+47 to +50 with Block 2's).
- **Bites:** none (section 2). The bites build runs the same tests: **at least +47 bites.**
- **Plants** (scratch edits, one `-x -k` runner call each, restored and sha256-checked; each must fail the named
  test): P1 the poller driven by `block_on` on a thread of its own (T1's re-run); P2 one socket kept across two legs
  with `set_default` (the never-mix test); P3 the pre-touch skipped (the pre-touch test); P4 the window taking
  warm-up fates (C-U6-5's warm-up test); P5 the judged field holding the L0 = p50 form (C-U6-5's fixed-entries test);
  P6 Oversize fates in E's sample set (the same test); P7 the RTT without the hold taken off (`leg_series`' RTT test);
  P8 the end-edge read after the socket's drop (the end-to-end test's bound check); P9 a missing /proc/net/udp row
  read as 0 drops (the parser test); P10 P0's base RTT not given to P1 (the end-to-end header check); P11
  `inputs_count` in connection order, not slot order (the record test); P12 the hash file cached at start (the
  replaced-file test); P13 `Cache-Control` dropped from the hash answer (the hash test); P14 a target matched by
  prefix (the 404 test); P15 the production order sent without its profile (the fake's profile assert); P16 an Invalid
  order polled again (the Invalid test); P17 the key file made 0644 (the files test). TE2-TE4 are Block 2's own.
- **check.mjs's 11 steps** (fmt, clippy, clippy-wasm32, doc, test, deps, wasm, refs, marks, self, redact) PASS on both
  builds; counts as deltas on `<base>`'s (U6b's merged head plus Block 0, which adds no test): at least +47 measured
  and +47 bites. The test step's time against `<base>`'s; past 1.5 times it stops (u4f-brief.md section 1 item 5).
- **Block 0's evidence:** section 5's binding check, each item recorded.
- **Container readings, reported, never judged,** before Stop B's read (u8-brief.md:466): the probe on loopback with
  `probe_peer` (release build, an example under crates/server/examples/, no manifest change), P1 at 10,000 a second
  each way for 100 s: both E forms per direction, the over count, p50 to max, the drain's hold, the window's counts,
  steal before and after; npt-load with 48 clients for 60 s on each transport against a room at P2: inputs a client a
  second (smallest, largest) and this container's CPU (not LG's); the host serving U7's dist if U7's TypeScript half
  has built one, else fixture files.
- **The report gives:** counts and provenance; each plant's failing test and restore hash; the SERIES.md rows written
  (section 4's new names; the seven `probe_*` rows, `params.base_rtt_ns`, `probe_rate_pps`, `probe_sent_count`, the
  `l2_*` header, `second.v.l2_*` and `l2_summary` rows for process `probe`, `second.v.steal_permille`,
  `second.s.inputs_count`, `second.v.cpu_permille`, LG's header) and every npt-server or npt-bots row left, named for a
  later unit (advisor-u6-brief.md condition 1); the new pub items; added lines per block against section 8; what is not
  verified (no RS, LG, browser or Let's Encrypt here: the first real order is R1's staging run; a loopback E is not
  RS's).

## 4. The carries, word for word, and the advisor's conditions
**C-U6-1 to -8, U3d's carries** (plan.md:1254, kept by replan-units.md:38-39):
- "**C-U6-1 the poller:** the -3 probe's inbound poller is a task spawned on npt-server's multi-thread runtime (never
  `block_on`), calling `poll_recv` with quinn's 32 buffers of 1,472 B until Pending, so -3's L2 up series and the
  leg's E include the hop quinn pays; U3d's blocked-worker test (T1) re-run on the probe's own poller; the pacer stays
  a dedicated thread;" Blocks 1 and 3.
- "**C-U6-2 one socket per leg:** one ImpairSocket per profile leg, so each `l2_summary` is one leg's; no reset method
  (plan §4.7 lists none) and `set_default` never carries a socket across legs; a test shows two legs' summaries never
  mix;" Blocks 1 and 3.
- "**C-U6-3 the fate log:** U6 chooses the drain cadence (per second with capacity for one to two seconds, its pages
  touched in the warm-up; or one log per leg pre-faulted before the measured phase); the drain's closure only copies
  into a buffer the probe owns, no I/O under the socket's lock; the lock hold per drain measured and reported;"
  Blocks 1 and 3; the cadence is Q-U6C-2.
- "**C-U6-4:** the probe process writes `second.v.steal_permille` as npt-server does;" Block 3 (U6a's half is built).
- "**C-U6-5 the judged E's producer:** at the P1 leg's end the probe computes per direction, from its fate log and
  over exactly -3's measured records: n, E with L0 = 0 (judged), E with L0 = the window's nearest-rank p50, the over
  count, p50, p99, p99.9 and max, on the histogram's sample set (finished fates but Oversize; -3's delays use
  Delivered only), in the leg's summary event under names U6 adds to SERIES.md; tested on fixed fate entries (a late
  datagram in the warm-up stays out of E; inside the window it enters E with exactly its excess over n in both forms;
  T5's samples give 10,000 and 10,100 ns) and live (U3d's T2 identity re-run);" Blocks 1 and 3; names below.
- "**C-U6-6 rate and composition:** U6 states the up stream's composition (LG's fresh sends against its echoes) so the
  probe's socket sees 10,000 per second each way; another rate makes the leg's self-test INVALID and is recorded, but
  never by design: if -3's sends plus echoes cannot give 10,000 per direction through the probe's socket (ADR
  0001:1590), U6's plan brings it to the advisor before R1's run plan is built;" Block 3: the composition is stated
  there, 10,000 a second each way by design, so nothing comes to the advisor unless the loopback reading misses.
- "**C-U6-7 observer calls:** inside the measured window the probe calls only `try_send`, `poll_recv`, `take_second`
  once per second and the drain; `header_numbers`, `header_texts` and `run_summary` run at the leg's edges;" Block 3
  (under Q-U6C-2's lean the drain is the end edge's).
- "**C-U6-8 the stamps' tests (the U3d skeptic's TE2-TE4; the advisor's U4d close ruling, 2026-11-11):** a test fails
  if down's send stamp is taken at the pop instead of after `try_send` (TE2's plant, socket.rs:726); one fails if up's
  clock read moves before the lock (TE3's swap, socket.rs:501 and :503); one fails if `agree()` reads `p999_pass ||
  would == "MISS"` (TE4's rewrite). Each plant is run once and fails by name. T7's doc (socket_tests.rs:1263-1264) and
  SERIES.md's down rows then claim only what a test proves. A plant the day session's run already shows failing a test
  drops out" Block 2.

**Q-U8-5's new npt-log names** (for this check: "its new npt-log names come to me in U6c's brief check",
u8-brief.md:152; Q-U6C-5). Names end in their unit or are flags (marks/README.md:41-43); each gets a SERIES.md row:
- **C-U6-5's, added to P1's `l2_summary` event** (prefixed so they never read as the socket's whole-life fields:
  SERIES.md:490-491): `probe_up_count` (n), `probe_up_excess_ns` (L0 = 0, judged), `probe_up_excess_p50_ns` (L0 = the
  window's nearest-rank p50), `probe_up_over_count` (L above 0.25 ms), `probe_up_p50_ns`, `probe_up_p99_ns`,
  `probe_up_p999_ns`, `probe_up_max_ns`; the same eight for `probe_down_`; `probe_window_ns`; `probe_up_window_count`
  and `probe_down_window_count` (C-U6-6's datagrams in the window); `probe_drain_hold_ns` (C-U6-3).
- **C-U8-6's, event `leg_edge`** (twice a leg, `ph` `warmup` before the window and `after` behind it):
  `udp_row_found` (flag), `udp_drops_count` (the socket's own row), and per Udp field present
  `snmp_udp_<field>_count` and `snmp6_udp_<field>_count` (in_datagrams, no_ports, in_errors, out_datagrams,
  rcvbuf_errors, sndbuf_errors, in_csum_errors, ignored_multi, mem_errors).
- **C-U8-2's, event `cpu_stat`** (the same two edges): `cpu_<field>_count` for the summed `cpu` line and
  `cpu<N>_<field>_count` for each vCPU N, fields user, nice, system, idle, iowait, irq, softirq, steal, guest and
  guest_nice, in the kernel's clock ticks as read.
- **Header:** `params.steal_cpus` (the vCPU list C-U6-4's steal covers).

**C-U6-19's gate line** (u6a-brief.md condition 2, :30-34): "Until U6d merges, U6a's, U6b's, U6c's, U7's and U8's
briefs and R1's run plan each state in one line that their tests or runs hold no throwing or rejoining hostile client
and no client restarted mid-run, or bring C-U6-19 first. In R1 each browser leg (each browser on each transport, the
QUIC-blocked fallback, -36's three) joins its own slot once, or the room restarts between legs; the token file holds a
slot for every leg. SA-F1-1 stays open until U6d merges." U6c's line: **U6c's tests hold no throwing or rejoining
hostile client and no client restarted mid-run: npt-load hosts honest Behaviours only, each joining its own slot once,
never restarted or rejoined; the probe has no game client.**

**From U6a's and U6b's briefs:**
- u6a-brief.md condition 5 (:52-53): "`StealReader` stays pub(crate); U6c's brief makes it pub if its probe lands
  outside npt-server." It stays pub(crate) under Q-U6C-1's lean.
- u6a-brief.md condition 3 (:37-40), the hold form: "npt-super starts a run's room, times it to OPEN, writes the event,
  keeps the room until its own end (stdin held open, closed at npt-super's end) and records its exit". npt-load's
  `run` and `probe` end the same way, when stdin closes.
- u6b-brief.md condition 4 (:64-65): "U6c's host serves it verbatim with `Cache-Control: no-store` (plan.md:994-995),
  read per request or after each room start, under a size cap". Block 5: per request, 1 KiB, per room (Q-U6C-3).

**From U8's brief** (u8-brief.md section 0.1):
- C10 (:100-101): "U6c's brief then takes Q-U8-4's interface, Q-U8-5's reads, the probe's ports and npt-load's
  control interface for C4; not the knob." Blocks 6, 3, 3 and 4; the knob stays out (section 2).
- Q-U8-4 (:144-149): "U6c's issue mode is **one process** across both phases (it prints name and value, waits for
  "go" on stdin, then finishes), so no ACME account key or order state is written to disk between phases;" and "one
  30-minute bound covers both phases; `dig` takes no caller argument. U6c's brief takes this interface." Block 6.
- Q-U8-5 (:150-153): "U6c's brief carries it, and its new npt-log names come to me in U6c's brief check; U8's `probe`
  block checks presence, a missing read makes the leg INVALID." Block 3 and the names above.
- C4 (:65-71): "starts, feeds and stops U6c's npt-load, copies token files and runs C5's check. Its verbs follow the
  npt-load interface U6c's brief names." Block 4's interface (`run`, `calibrate`, `probe`; stdin's close as the stop).

**advisor-u6-brief.md, on the blocks it gave the uncut U6d** (now U6c's):
- Condition 1 (:292-295): "each piece's brief names the SERIES.md rows it writes, and U6's close report shows every
  npt-server row written or named for a later unit". Section 3's report line.
- Condition 6 and ADV-U6B-7 (:251): "U6d (instant-acme's stack, npt-bots on npt-net)". Section 5's Block 0.
- Condition 7 and ADV-U6B-8: every reviewer at xhigh; its engine-architect seat, "U6d if the probe lands in npt-server
  (Q12)" (:258), is replaced by NPT-TRIM's default as Q-U6A-7 ruled and U6b followed: the netcode seat reads the pub
  items (section 7).
- Condition 8, U6-Q2 (:179-183): npt-bots' glue and npt-load "after C-U6-18 (U6a's S3: one entry point for a
  Behaviour's host order "before npt-bots and npt-load host Behaviours", row:43)", with the clock bans' reviewed allows:
  Block 4; its E14 floods are cut.
- Condition 8, U6-Q17 term (3) (:212-213): "(3) npt-host's `/npt/cert-hash.json` (plan.md:994-995) is U6d's". Block 5.
- Condition 8, U6-Q18 (:214-219): "Terms: the root store for hyper-rustls is chosen at U6d's Block 0 by
  security-auditor's read; U6d confirms instant-acme 0.8.5's revocation call by name (plan.md:967); its tests run
  against an in-process fake directory (no network to Let's Encrypt here), the first real order being R1's staging dry
  run." The root store: section 5 (lean `Account::builder()`'s platform verifier over the OS store; webpki-roots is in
  no lock); the revocation term lapses with `revoke`'s cut; the fake directory: Block 6's tests. No 90-day fallback.
- The deferrals (:230-232): "Q11: if -3's sends and echoes cannot give 10,000 a direction, the advisor before R1's run
  plan (C-U6-6)." Met by C-U6-6's composition. "Q12: §4.7 lists npt-lab's `probe` (plan.md:612) and no probe item in
  npt-server (:600-601); a home in npt-server adds a public item, so engine-architect reads it." Q-U6C-1, read by the
  netcode seat as above.

**security-r1-deps.md:** the pin (:225-227): "instant-acme =0.8.5 (U6, npt-host only): CLEAR WITH CONDITIONS:
default-features = false, features = ["ring", "hyper-rustls"]; never aws-lc-rs / fips; DefaultClient (https_only) with
the platform verifier or with_roots, no custom plain-HTTP client; SEC-R1D-4." The fake `HttpClient` lives in tests
only; no path of the shipped binary takes a client from outside. SEC-R1D-4 (:212-214): "Fix: each ACME call under
`tokio::time::timeout` inside the wrapper's 30-minute bound; the directory URL a constant per staging|production (as
npt-cert's no-free-argument plan)." Block 6.

## 5. Dependencies: Block 0 (named up front; DEP-HOLD)
- **The main session makes Block 0 on the lane before `<base>`** (builders run offline; a further manifest change stops
  the build; `<base>` must compile), after U6b's merge, under DEP-HOLD.
  - **(a) spikes/netcode-proof/Cargo.toml [workspace.dependencies]:** new `instant-acme = { version = "=0.8.5",
    default-features = false, features = ["ring", "hyper-rustls"] }` (the 17 packages enter the lock); new direct
    entries for packages already locked or among the 17: `httparse = "=1.10.1"` (the page host's request parse),
    `http = "=1.5.0"`, `bytes = "=1.12.1"` and `http-body-util = "=0.1.5"` (the fake directory, dev only).
  - **(b) crates/host/Cargo.toml:** [dependencies] `instant-acme.workspace = true`, `rcgen = { workspace = true,
    features = ["pem"] }` (U6b's entry; pem 4.0.0 and base64 0.23.1 are 2 of the 17), `tokio.workspace = true`,
    `tokio-rustls.workspace = true`, `httparse.workspace = true`, `serde.workspace = true`,
    `serde_json.workspace = true`, `sha2.workspace = true`; [dev-dependencies] `http`, `bytes`, `http-body-util`, each
    `.workspace = true`.
  - **(c) crates/bots/Cargo.toml:** [dependencies] `tokio.workspace = true`, `serde.workspace = true`,
    `serde_json.workspace = true`; [dev-dependencies] `npt-server.workspace = true` (Block 4's end-to-end tests).
  - **(d)** crates/server and crates/net: no change expected (both already have tokio, serde and npt-net's pieces).
- **The expected lock diff:** exactly the 17 packages of security-r1-deps.md:228-230 at those versions, crates.io
  source, checksums equal to :258-273's; npt-host's and npt-bots' dependency arrays gain their new names; rcgen's array
  gains pem 4.0.0; the arrays that name base64 or pem today gain the version Cargo adds when a second version enters
  (base64 0.22.1, pem 3.0.6); nothing else (url and form_urlencoded were U6a's; no aws-lc-* returns after U6b removed
  it). A newer patch of any of the 16 is pinned back by `cargo update -p <name> --precise <cleared>`, or comes back to
  security-auditor.
- **The binding check** (security-r1-deps.md:249-253 for the 17, in U6b's Block 0 shape, u6b-brief.md:393-400): (1) the
  manifest edits, then the lock refreshed (network on, the main session only; never `cargo update` without `-p`), then
  `cargo fetch --locked`; (2) the lock diff exactly as above; (3) `cargo tree --locked --offline -e features`:
  instant-acme with `ring` and `hyper-rustls` only (no `aws-lc-rs`, no `fips`), hyper with client, http1 and http2
  (no server), rcgen now with `pem` (expected: instant-acme's `ring` turns it on, security-r1-deps.md:70-72, so
  u6b-brief.md:396-397's "without `pem`" holds only until here), rustls's features as unified (if hyper-rustls turns on
  `tls12`, every server config still names TLS 1.3 only: ws_config.rs:27-38 and Block 5); (4) `cargo check --workspace
  --all-targets --locked` 0 warnings, check.mjs's test and deps steps PASS at `<base>` with counts equal before and
  after Block 0; (5) `cargo deny --locked check advisories bans sources` and `cargo audit` clean as U6a's Block 0
  defined it (u6a-block0.md:36-55), the database's date recorded (two new duplicate warnings expected, base64 and pem;
  deny.toml has `multiple-versions = "warn"`). Anything else returns to security-auditor before the save.
- **DEP-HOLD** (main's .claude/CHECKPOINTS.md §3): the save waits for security-auditor's answer on exactly this content
  and dependency-check.md (one line per changed file). security-r1-deps.md clears instant-acme and its 16 (:225-230);
  its binding check is written for U6 whole (the url bump, done in U6a). **A short read is needed for the rest:** the
  four direct entries in (a), rcgen's `pem` in (b), npt-bots' moves and its npt-server dev-dependency in (c), the root
  store behind `Account::builder()` (U6-Q18's term), and its yes to cargo-deny 0.22.0 and cargo-audit 0.23.0 again.

## 6. The rules that stand (as u6a-brief.md section 6 gives them)
- **Start:** one builder (general-purpose, opus, xhigh), `isolation: worktree`, which starts at main. First step, in
  its worktree: `git merge --ff-only <base>` (if it cannot fast-forward, `git switch -c u6c-build <base>`), then
  `git log -1 --format=%H` shows `<base>`; both in its notes first (u4f-brief.md:292-295). `<base>` is left for the
  main session to fill, after Block 0's save.
- **Notes:** `.checkpoints/netcode-proof-test/u6c-build.md` in the worktree: the task, the blocks, each finished block
  at once, a notes line per finding, `git diff --stat <base>`'s added lines after each block.
- **The shared target** `/home/user/Shamble-Frontlines/.gate/shared-target` and `CARGO_NET_OFFLINE=true`, every cargo
  call through a U6c runner (the main session's names-only copy of u6b-run.sh, logs under .gate/u6c/), the lane lock
  and the disk floors of u4f-brief.md section 4 rules 1-9.
- **Files:** U6c's lane is crates/net (l2/probe.rs; socket.rs's `pretouch_fate_log` and Block 2's test-only delay;
  stats.rs's `FateEntry`; socket_tests.rs; lib.rs's exports), crates/server (probe.rs, main.rs's `probe`, tests,
  examples), crates/bots (its lib modules, the npt-load binary, tests), crates/host (all of it), and marks/SERIES.md
  (new rows; Block 2's down rows' wording). Not U6c's: every Cargo.toml and Cargo.lock (Block 0's; a further change
  stops the build), proto, room, client, sim, link, l1, log, lab, judge, super, client-wasm, marks.json,
  marks/README.md, any other existing SERIES.md row, check.mjs, ops/. A part that needs one stops, is reported, and
  the rest is built.
- **Bans:** `.claude/rules/netcode.md` (crates/server and crates/net): nothing unbounded keyed by client data (every
  queue bounded, every wait under a deadline); server/clippy.toml's and bots/clippy.toml's clock and hash-map bans,
  with one reviewed allow for npt-load's clock item (the probe uses U6a's); npt-host reads no wall clock (its deadlines
  on tokio's timer); no client string or token in any log (E22); SEC-U6AD-2's and -3's rules for every file U6c reads
  or writes; no `CryptoProvider::install_default` in our code (u6b-brief.md condition 2); no Origin check and no
  `Url::origin()` on client input; tests on 127.0.0.1 only and nothing to the network (the fake directory); LEAD-BOUND
  frozen at 400 ms.
- **SIZE-CHECK** (history.md:2048-2051): past 6,000 added lines the build stops, the end checks run on the tree as it
  stands, and the main session gives Mustafa three lines (what grew, why, the cheaper option).
- **Agents never commit;** the main session snapshots, merges and commits. The builder's brief carries, word for word:
  "Write files with the Write and Edit tools only (no sed, no shell redirect, no python writes; never `cargo fmt`
  writing files: apply rustfmt's diff by Edit); an agent with no Edit tool rewrites its notes file whole with Write."

## 7. Review
One round (NPT-TRIM), netcode-architect, test-engineer and security-auditor, each at xhigh; a second only for an open
Critical or High. Security joins because the page host and the ACME client face the internet in every run
(replan.md:93). Its reads: the page host (the allow-list, exact matching, the verbatim files and their caps, TLS, the
headers, no request log), the issue mode (the fixed words, the constant directory URLs, the account key in memory, the
profile failing closed, no retry, the per-call timeouts, `create_new` and 0640, the fake client reachable from tests
only, the root store), npt-load's token and pin reads, the probe's settings and its one pinned peer. Netcode: the probe
against C-U6-1 to -7 and SERIES.md:497-501, the E fields and the leg-edge names, the new pub items against §4.7 (the
engine-architect seat replaced by NPT-TRIM's default, as Q-U6A-7 ruled), the glue's host order and clock. Test: the
plants, the counts, the fake directory's coverage. Under Q-U6C-8 (2), security's read of the host and the issue mode
here stands as Stop B's read of "U6c's host" (u8-brief.md:466-469).

## 8. Size
- **The re-plan's figure:** 1,950 estimated, 2,650-2,950 likely (replan-units.md:60).
- **This brief's:** about 2,800 (2,550-3,050). P1 1,140 (Blocks 1 and 3, +390: the E machinery moved from test code
  into shipped code, since C-U6-5 has the probe compute it; the base RTT, ADR 0001:1590; C-U8-2's and C-U8-6's
  leg-edge reads, given to U6c after the re-plan by Q-U8-5; LG's peer); P2 170 (+20); B1 700 (+100: C4's interface,
  LG's header, the summary); H1 440 (+140: TLS and HTTP on tokio-rustls and httparse, since hyper has no server
  feature here; the per-room hash files; the host-key file); H2 380 (+230: U6-Q18's in-process fake directory,
  SEC-R1D-4's timeouts, Q-U8-4's one-process form). Block 0: about 20 manifest lines. **Likely 3,800-4,200** at the
  units' 1.35-1.5. **The stop: past 6,000** (SIZE-CHECK; replan.md:25: "each unit under the 6,000 stop at 1.5x").
- **The lane before R1:** U6a 3,150-3,800 + U6b 3,600-4,000 + U6c 3,800-4,200 + U7's Rust 490-540 = about
  11,050-12,550 likely lines (the card's 9,900-11,300 held U6c at 2,650-2,950). **Day line:** build from U6b's merge
  (about Fri 27), about two days; round, fixes and merge about Mon 30; then U7's Rust half on the lane (Mon 30-Tue 1)
  beside Stop B's read, and his Stop B steps Tue 1. **R1 on Thursday 3 December holds with no slack at the middle of
  these numbers; at the top it slips a day,** and he gets three lines at once (U6a's condition 8).
- **Levers** (Q-U6C-8): (1) P2 rides in MOVED's brief, before R1's report (R1's run does not read it; its report does,
  as the burst knob's ruling reasoned: advisor-u8-brief.md:106-107), about 230-255 likely lines off the lane before
  R1; (2) security's read of U6c's host in this round stands as Stop B's (replan-units.md:214-215), so Stop B's round
  reads U8's package and wrappers only; (3) the split R1 (replan-units.md:215-216) is his.

## 9. Open questions (each with my lean; the advisor's check answers them)
- **Q-U6C-1, the probe's home** (U6-Q12; §4.7 lists npt-lab's `probe` and no probe item in npt-server: plan.md:612,
  :600-601). (a) npt-server's library (`npt_server::probe`, run as `npt-server probe`), its shared pieces in npt-net;
  (b) npt-lab's `probe` with an L2 layer; (c) a new crate. **Lean (a):** SERIES.md:497-501 already names "npt-server's
  binary in a probe mode", C-U6-1 names npt-server's runtime, and `StealReader` stays pub(crate); the netcode seat
  reads the new pub items (Q-U6A-7's pattern).
- **Q-U6C-2, C-U6-3's drain cadence.** (a) One log per leg, pre-touched before the window and drained once at the
  leg's end (the self-test's own form, socket_tests.rs:1384-1406; about 146 MB at P1, RS has 16 GB); (b) a drain each
  second with one to two seconds' capacity (about 2.2 MB; some 20,000 entries copied under the socket's lock each
  second, inside the window). **Lean (a):** no drain under the lock inside the measured window at all; the one drain's
  hold is still measured and reported.
- **Q-U6C-3, the hash file's URL with several rooms.** U8's C3 runs a room per mark and browser (u8-brief.md:55-63),
  and each room makes its own certificate (u6b-brief.md:212-214), so one `/npt/cert-hash.json` (plan.md:994-995) is
  ambiguous. (a) `/npt/cert-hash-NN.json` for NN 00-31 from a path template in the host's settings, the plain name as
  room 00's; (b) one name, one room with a browser at a time. **Lean (a);** U7's brief takes the form, and its dist
  names become Block 5's compiled-in allow-list.
- **Q-U6C-4, the page host's request buffer.** The cut removed "the page host's 5 s header timeout, 8 KB cap,
  per-address and total caps" (replan-units.md:13-14), but parsing still needs a buffer. (a) One fixed buffer a
  connection; a request that does not fit is closed (no counter, no 431, no timeout); (b) a growing buffer (against
  .claude/rules/netcode.md: nothing unbounded keyed by client data). **Lean (a):** it is the parser's own buffer, not
  the cut cap with its tests and counters.
- **Q-U6C-5, the new npt-log names** (section 4; Q-U8-5 sends them here). **Lean:** as listed; the builder adds each
  SERIES.md row, and U8's `probe` block reads `leg_edge.udp_row_found`, `udp_drops_count` and the `cpu_stat` lines.
- **Q-U6C-6, the host-key file and the first SSH.** plan.md:286-290 has the laptop fetch `/npt/ssh-host-keys.txt`
  over the production certificate before the first SSH; under DNS-01 by hand the order is started from the laptop
  over SSH (replan-units.md:110-111; u8-brief.md:416-423 puts the certificate after bring-up), so that check cannot
  come first. **Lean:** U6c serves the file (a few lines, plan.md:1119); the order of the first SSH goes to U8's next
  edit and Stop B's security read (my lean there: §1.6's console fallback for the first SSH, and the HTTPS check after
  issuance, before any room starts).
- **Q-U6C-7, the host's start.** No wrapper in u8-brief.md rev 1.1 starts `npt-host serve` as `npt-host`
  (u8-brief.md:205-240), the user that reads the 0640 key. **Lean:** U8's next edit adds `npt-ctl host start|stop`
  (fixed settings path, a transient scope as `npt-host`), read at Stop B; U6c's interface is `npt-host serve
  <settings.json>`, running until killed.
- **Q-U6C-8, the size and the levers:** about 2,800 against the re-plan's 1,950 (section 8). (a) Build as placed;
  lever (2) taken now; lever (1) ruled now, for the main session to pull if the build runs past Sat 28, named in that
  day's message; (b) both levers now. **Lean (a):** P2 checks the very stamps R1's self-test reads, so it is cheaper
  built before R1 if the days allow; lever (2) costs nothing, since security reads the host in this round anyway.

## 10. For the main session
- **Now:** this brief to the advisor (one check); security-auditor's short read of section 5's content beside it.
- **Before launch (on the lane, after U6b merges):** Block 0 (a)-(c), the binding check, cargo-deny and cargo-audit
  with the database date, dependency-check.md, the save; fill `<base>` and its counts; the runner copy u6c-run.sh; the
  size line (section 8) on the card before the first snapshot; the advisor's answers applied as rev 1.1 (no re-check
  unless it finds a Critical or High); U7's drafter told Q-U6C-3's URL form; U8's next edit told Q-U6C-5's names,
  Q-U6C-6, Q-U6C-7, npt-load's interface (Block 4) and the probe's port (Block 3).
- **Close:** the builder's notes and return list (u4f-brief.md section 7's shape); snapshot; check.mjs in a merge
  worktree; the round; fixes; merge; U6c's report (counts, plants, rows, size, carries by number, Block 0's lock diff
  and audit results, the container readings).
