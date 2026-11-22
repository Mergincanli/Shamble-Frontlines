# Advisor: U6b's build brief, rev 1 (the one check NPT-TRIM allows)

Instructions: main's `.claude/agents/advisor.md` (git show origin/main:.claude/agents/advisor.md, 335 lines), read 2026-11-22.
Rules in force: NPT-TRIM (history.md:7561-7593), his answers of 2026-11-22 (history.md:7595-7607; R1 Thursday 3 December),
TEST-FIRST (history.md:2234-2282).

## Plan
1. Read the brief (u6b-brief.md rev 1) and the drafter's notes (netcode-u6b-brief.md).
2. Read the carries: replan.md, replan-units.md §1 §2 §7, u6-brief.md rev 2.1, u6-facts.md, advisor-u6-brief.md,
   plan.md:1254, u6a-brief.md rev 1.1, advisor-u6a-brief.md (conditions 2, 3, 6), security-r1-deps.md (:249-253),
   u8-brief.md (DelayStream self-test, run plan), advisor-u8-brief.md if it exists.
3. Check the code facts the brief leans on (spikes/ crates, Cargo manifests, lock) by grep.
4. Rule: Q1-Q6; the seven contradictions; size and R1's date; Block 0's manifests and security's read; the time trap;
   anything else.
5. Verdict with paste-ready conditions; summary.

## Notes (one line per finding, as I go)
- Steps 1-2 DONE: read u6b-brief.md (411 lines), netcode-u6b-brief.md, replan.md, replan-units.md whole, u6-brief.md
  rev 2.1, advisor-u6-brief.md, u6a-brief.md rev 1.1, advisor-u6a-brief.md, security-r1-deps.md (:1-60, :180-283),
  u8-brief.md (:185-310, :340-368), plan.md:1254 (split on <br>) and :1256, u6a-block0.md:30-60, the card (:150-166).
  advisor-u8-brief.md exists but holds only its plan (no rulings yet, 15 lines): Q3 is ruled so it holds either way.
- Facts that hold (step 3): wt.rs:4-5 and :176-179 put web-transport-quinn on aws-lc-rs (so replan-units.md:77-78's
  "as l2/wt.rs does" is wrong); wt_tests.rs:586-592; ws_config.rs:1-6; the lock's aws-lc-rs dependents are exactly
  quinn-proto, rustls, rustls-webpki; quinn 0.11.12's default is rustls-ring; wtransport 0.7.2's default is
  ["self-signed","ring"] with rcgen default-features off; wtq 0.13.6 default ["aws-lc-rs"], `ring` exists, its
  `crypto::default_provider()` reads the process default first; rcgen 0.14.10 exports `date_time_ymd`
  (lib.rs:44-47; certificate.rs:1040-1047, which `expect`s a valid date) and `not_before`/`not_after` are
  `time::OffsetDateTime` (certificate.rs:57-58); delay.rs: POOL :185-205, split_rows :1119, add_burst :1136,
  gate_inputs :1237, header_numbers :1349, take_second :1424, run_summary :1466, closing wake_all :1907; stats.rs:52-61
  `Summary.excess` is E[(L − p50 − 0.25 ms)+] (the median form only); link/src/lib.rs:64-88 `Link`, and :97-119 the
  blanket `impl<L: Link + ?Sized> Link for Box<L>` that forwards each method by hand; pump.rs:13-14; socket.rs:176,
  :312; ws_gate.rs:25, :36, :58-69, :74; ws_config.rs:63; tcp_kernel.rs:22, :28; tls_test.rs:92-97 (`Pinned` holds the
  whole certificate, not a hash); proto/src/control.rs:362-388 (SetPeerProfile, SetPeerThrottle, SetPeerBottleneck,
  PeerBurst {peers, ms} with no start field, BlockPeerUdp); check.mjs:2017, :2057; SERIES.md:241, :326-330, :589-616.
- F1 (Medium): Q1 (a)'s provided `begin_tick` would not reach WsLink through `Box<dyn Link + Send>` unless the blanket
  Box impl (link/src/lib.rs:97-119) forwards it: a provided method compiles without the forward, so TickLoop's calls
  would hit the default and fix 1's gate would never run in the binary while WsLink's own tests pass.
- F2 (Medium): Q1 (a) needs TickLoop's I/O send site to call `begin_tick`, but section 6 allows TickLoop's seam "only
  for the accept stamp" (u6b-brief.md:335-336): as written that part stops the build.
- F3 (Medium): WsClient "with the same limits" (u6b-brief.md:87-88) would take the server's 513 B frame and message
  limits (catalogue docs/design/netcode-proof-messages.md:73-76) and refuse the room's FullState (65,536 + 5 B plus the
  tag) as too long.
- F4 (Medium): Q3 against U8's brief: U8 sizes the self-test "5 min measured after 1 min warm-up" (u8-brief.md:206)
  while the pool never resets (delay.rs:185-188), and C-U8-5's form may be L0 = 0 (u8-brief.md:352-355) while
  `Summary.excess` gives only the median form (stats.rs:58-59).
- F5 (Low): the blocks sum to 2,850 (150 + 500 + 200 + 300 + 130 + 760 + 560 + 250), section 8's T-sums to 2,700.
- F6 (Low): `ConnCap` crosses into npt-server (both binds take it) but is not in the new pub items (u6b-brief.md:141-144).
- F7 (Low): Block 0's expected lock diff names the removed build crates loosely; by the lock they are exactly cmake,
  dunce, fs_extra and pkg-config besides aws-lc-rs 1.19.0 and aws-lc-sys 0.47.0 (cc stays for ring and generator,
  zeroize for rustls).
- F8 (Low): RC-12's "before the run package is built" cannot hold literally: U8 builds beside U6a now
  (replan-units.md:197-198); its purpose (advisor-u3c-plan.md:187-188) is met if the readings reach R1.md before Stop
  B's read.
- F9 (Low): the 20-iteration kick tests at P2 with loss against the 5 s real-clock cap; the hash file's order against
  OPEN and a room restart; SERIES.md:616's second clause (what the plain path writes) unplaced; `begin_tick`'s default
  written "Ok" while `SendResult` has no Ok (link/src/lib.rs:33-42).
- Steps 1-3 DONE. Next: rulings.

## 1. Q-U6B-1 to -6 (each inside the plan, undoable, no pass mark, no money, nothing on main: mine, TEST-FIRST rule 2,
history.md:2261)
- **Q-U6B-1, W18: Ruled (a), with five terms.** (b) delays every WebSocket snapshot a tick (a measured path changed
  for an interface's sake); (c) couples npt-server to npt-net's types. (a) adds one provided method to a std-only leaf
  trait (npt-link stays wasm32-clean) and keeps the room's own BufferFull path (plan §4.4). Terms: (1) the signature
  `fn begin_tick(&mut self, lens: &[usize], now_ns: u64) -> SendResult`, default `SendResult::Sent` (the brief's "Ok"
  is no variant: link/src/lib.rs:33-42); (2) the blanket `impl<L: Link + ?Sized> Link for Box<L>`
  (link/src/lib.rs:97-119) forwards it, with a test that calls it through `Box<dyn Link + Send>` and sees WsLink's
  refusal, and a plant P16 (the forward removed) failing that test (F1); (3) TickLoop's I/O side calls it once per
  session per tick, before that tick's datagram sends, with each datagram's length; that call site (and a per-tick
  batch boundary, if U6a's I/O side has none) is U6b's second TickLoop change, added to section 6's lane and named in
  the report (F2); (4) WsLink answers every datagram of a refused tick BufferFull and counts it in
  `tick.v.ws_skipped_snaps_count`; a datagram send that no current `begin_tick` announced, or one past its count, is
  answered BufferFull and counted, never written ungated; control and FullState sends stay ungated; (5) the trait's
  doc text states the caller's duty, and the netcode seat reads it against §4.7 (Q-U6A-7's pattern: no
  engine-architect seat).
- **Q-U6B-2, the provider: Ruled (a), ring.** One provider for every TLS configuration in npt-net (wtransport's,
  web-transport-quinn's, tokio-rustls's), no aws-lc-sys C build in every measured build and on RS once wt-wtq is
  default, a lock that only shrinks, and the same ring floor the workspace already pins (Cargo.toml:59-60, :69-71;
  plan.md:965-966 chose ring for instant-acme for the same reason). No pass mark reads the provider; U3's loopback
  equality of the two crates was measured, never judged. Terms: no `CryptoProvider::install_default` anywhere (wtq's
  `crypto::default_provider()` reads a process default first), each configuration's provider chosen by
  `builder_with_provider` as M7 and M16 have it; wt.rs's module text (:4-6) and wt_tests.rs:586-592 say ring; `<base>`'s
  check.mjs test step PASSES with the wt module's tests now in the measured build on ring (not only counted);
  security-auditor's short read (section 4) before the save.
- **Q-U6B-3, the series R1's DelayStream self-test reads: Ruled (a), with four terms; it agrees with U8's brief once
  U8's rev takes term (3) (F4).** Pooled L per direction in the split's existing process pool costs about 60 lines
  and two histograms; the per-second merge across connections serves only R3's -31 and -18 rows and stays U6e's
  (SERIES.md:589-591, :616). Terms: (1) L is recorded per release into one more `Pooled` per direction in `Pool`
  (delay.rs:189-205), the same L the identity sums (woken: clock + hop + pass + reader; other: wait + pass + reader),
  and the fixed heap it adds is in the header's split bytes; (2) `ws_split` writes per direction n, p50, p99, p99.9,
  max and E in **both** forms with their over counts: L0 = 0 (E[(L − 0.25 ms)+]) and the median form (E[(L − p50 −
  0.25 ms)+], `Summary.excess` today: stats.rs:58-59), by giving `summarize` its threshold as a parameter, tested
  against a brute-force sum as stats.rs's own tests do; so whichever form the U8 check rules for C-U8-5
  (u8-brief.md:352-355; advisor-u8-brief.md has no ruling yet) is read from the log with no code change, and if that
  check rules a third form, the main session brings it to U6b before Block 5 closes; (3) the window is the room
  process's whole life from its first wrap (RC-10: one measured run per process; no window mark is built), so U8's
  rev 1.1 states the self-test's judged window as its room process's life, the warm-up inside it (stricter, never
  more lenient), replacing "5 min measured after 1 min warm-up" (u8-brief.md:206), and names `event.ws_split`'s
  pooled-L fields as the series it reads; (4) the per-second merge is U6e's (S9), named in U6e's brief before R3, and
  nothing in R1 reads it.
- **Q-U6B-4, the key and the hash file: Ruled as leaned, with terms.** The key in memory only meets U6-Q17 term (1)'s
  purpose (npt-host's user never reads the key) more strictly than the cut 0700 folder did. The format
  `{"algorithm":"sha-256","value_hex":"<64 lowercase hex digits>","validity_ns":<u64>}` stands, with no "next" field
  (PM-0006-26's rotation moved). Terms: the file is written by serde_json from a struct of exactly those fields, by
  `create_new` with mode 0644 set at creation (public data; never a chmod), one `write_all` then `sync_all`, after the
  certificate is made and before OPEN, so anything that reads it after OPEN reads the current hash; `validity_ns` equals
  `header.params.cert_validity_ns`; an existing path is refused (exit 2), so U8's R1.md removes the previous room's
  file before any room restart between legs (C-U6-19's gate allows restarts); U6c's host serves it verbatim with
  `Cache-Control: no-store` (plan.md:994-995), read per request or after each room start, under a size cap; U7's page
  turns `value_hex` into the bytes of `serverCertificateHashes`. U6c's and U7's briefs take this form.
- **Q-U6B-5, SetProfile and the B1 range check: Ruled, they move to U6e with their callers** (Q-U6B-6 (b)). The
  per-session client controls were cut (replan-units.md:24-25); what remains are the operator's per-peer controls in
  `RoomControl` (proto/src/control.rs:362-388), all U6e's, and they span both transports (SetPeerProfile,
  SetPeerThrottle, SetPeerBottleneck, PeerBurst, BlockPeerUdp). Built now, U6b would fix `peer_burst(ip, start_ns,
  len_ns)` before its caller exists, while `PeerBurst` carries no start (control.rs:379-383) and `burst_return`'s
  offset is drawn from the run's seed (SERIES.md:326): U6e would refit it. No R1 mark reads any of them; Q-U6A-4's
  principle (no item without its reader) applies. SP-6 and RC-2 travel to U6e's brief by number; they stay U6's.
- **Q-U6B-6, the size: Ruled (b), apply the lever now** (section 3).

## 2. The seven contradictions (each inside the plan; none changes a pass mark, a run day or money; none is his)
1. **aws-lc-rs vs ring** (wt.rs:4-5, :176-179 vs replan-units.md:77-78 and advisor-u6-brief.md:248-250): ring
   (Q-U6B-2). replan-units.md:77-78's "as l2/wt.rs does" is a wrong fact; it is noted in U6b's report, not rewritten.
2. **SetProfile and TriggerBurst cut (replan-units.md:24-25), kept in T5 (:59):** the per-session client controls are
   cut; the DelayStream method and the per-peer hooks serve U6e's per-peer controls and move there (Q-U6B-5). R3 still
   has them before it runs, so no mark moves.
3. **The 0700 key folder cut** (U6-Q17 term (1), advisor-u6-brief.md:208-209; replan-units.md:28-30): the key in
   memory only replaces it (Q-U6B-4), stricter than the term.
4. **The binding check written for U6 whole** (security-r1-deps.md:249-253): its own words send "aws-lc-*" back to
   security before the save (:251), so U6b's removal needs security's short read, as the drafter leans (section 4).
5. **The merge across connections in no piece** (SERIES.md:589-591, :616): U6e's S9, named in U6e's brief, before R3;
   R1 reads pooled L instead (Q-U6B-3).
6. **Stale cites:** U6's row is plan.md:1254 (replan-units.md:3 says :1253); the reader stamp's site is
   delay.rs:1907 (the row says :1884); replan-units.md:77-78 (item 1). The brief cites the live lines; older papers are
   not rewritten (NPT-TRIM's paper rule, history.md:7592).
7. **S8b's 75 lines not covering the wiring and OPEN's move:** u6a-brief.md:175-177 gives U6b the binary's transport
   wiring and settings, and advisor-u6a-brief.md condition 3 moves OPEN; the +175 is a sizing gap the re-plan left,
   not new scope. Accepted.
None touches a pass mark or money. Only the size reaches a run day, through R1's date: section 3 rules it holds.

## 3. Size, R1's date and the lever
- **The arithmetic:** section 8's T-sums give 2,700 (500 + 330 + 300 + 760 + 560 + 250); section 1's blocks give 2,850
  (150 + 500 + 200 + 300 + 130 + 760 + 560 + 250): Block 1's 150 is counted in the blocks and only partly in T1 and T2
  (F5, Low). Until rev 1.1 recounts, the honest figure is the blocks': about 2,850, likely 3,850-4,275 (re-plan 2,205,
  likely 2,950-3,300). Each addition is R1's or a carry's: the closing tasks for `Kick(replaced)` (advisor-u6a-brief.md
  condition 6), the binary's wiring and OPEN's move (condition 3), pooled L (Q-U6B-3), the pin promoted.
- **The lane before R1 as built so far:** U6a 3,150-3,800 + U6b 3,850-4,275 + U6c 2,650-2,950 + U7's Rust 490-540 =
  about 10,140-11,565 likely lines (the brief's own figure: 9,950-11,350; my U6a ruling: 9,250-10,600 in 8 days, about
  1,150-1,325 merged a day, "no slack": advisor-u6a-brief.md:136-145). U6a's start on Sunday afternoon (the card :157-158)
  adds about 0.4 of a day: about 1,205-1,375 a day needed, a few per cent past the rate I called "no slack", against
  about 1,175 merged a day last week under heavy paper (replan-units.md:208-209).
- **The lever: Ruled, take it now.** Out of U6b into U6e: `DelayStream::set_profile` and its test, the WsServer
  per-peer profile hook, the B1 range check, `burst_return`'s four server fields (SERIES.md:326-329) and their test,
  plants P12 and P13: about 250 estimated, 340-375 likely. Why now: no R1 mark reads them (section 0's R1 list,
  u6b-brief.md:35-40); their caller is U6e's and would refit them (Q-U6B-5); and "only if the build runs long" fires
  when half of them are built. With it U6b is about 2,600 (likely 3,510-3,900), the lane before R1 about 9,800-11,190,
  about 1,165-1,330 a day: where my U6a ruling put R1. The cost: U6e grows by the same lines after R1, which takes
  about a third of the one spare day (Tue 15) in U8's close calendar at the middle (u8-brief.md:286-306); U8's own
  lever (R2 and R3 as soon as U6e merges) still keeps Friday 18 December at the top. R1 has him present and a planned
  day; R2-R5 run unattended. Counts become at least +42 measured and +44 bites (Block 4's `burst_return` test and
  Block 5's three hook tests leave), 13 plants plus P16 (Q-U6B-1 term 2).
- **R1 on Thursday 3 December: it still holds, with no slack, with the lever taken now.** Without the lever it sits a
  few per cent past that line; with it, at the line. One lost day (a spend-limit cut of most of a day, a second round
  on any lane unit, a Block 0 surprise) and it slips; then he gets three lines at once. Not a decision today: one line
  in the day's message. The three lines, ready if a day is lost:
  1. What grew: the server pieces before R1 came to about 9,800-11,200 likely lines against the re-plan's 8,350-9,250
     (U6a's room log, start timer, tokens and pinning; U6b's closing tasks, binary wiring and OPEN's move).
  2. Why: R1's marks need them and no piece held them; the build rate is about 1,175 lines a day merged.
  3. The cheaper option: split R1, its server blocks first and its browser blocks when U7 lands (one more bring-up,
     about EUR 0.15: replan.md:45), or more usage headroom this week.

## 4. Block 0's manifest changes: what security-auditor's short read must cover
Ruled: Block 0 as section 5 writes it, the main session's, on the lane after U6a merges, held by DEP-HOLD. The read:
1. **Exactly the content:** the workspace's `web-transport-quinn = { version = "=0.13.6", default-features = false }`
   (edition 2024 honours a member's `default-features = false` only through the workspace entry) and
   `rcgen = { version = "=0.14.10", default-features = false, features = ["ring"] }`; npt-net's `default = ["wt-wtq"]`,
   `bites = []`, `web-transport-quinn = { workspace = true, optional = true, features = ["ring"] }` and futures-util
   moved to [dependencies]; npt-server's `rcgen.workspace = true` and `bites` gaining `"npt-net/bites"`; no other key,
   feature or version, and no dependent naming a `wt-*` feature or `default-features = false` on npt-net (U6-Q13 term
   (1)).
2. **The four provider lines:** wt.rs:176-179 (ring, with its doc line and the module text :4-6), wt_tests.rs:586-592,
   ws_config.rs:4; ring chosen per configuration by `builder_with_provider`, never `CryptoProvider::install_default`,
   so web-transport-quinn's `crypto::default_provider()` (which reads a process default first) resolves to ring.
3. **The lock diff, named exactly:** removed aws-lc-rs 1.19.0, aws-lc-sys 0.47.0, cmake, dunce, fs_extra and
   pkg-config (by the lock, each reached only from aws-lc-sys; cc stays for ring and generator, zeroize for rustls);
   `"aws-lc-rs"` leaves the dependency arrays of quinn-proto, rustls and rustls-webpki; npt-server's array gains
   `"rcgen"`; nothing added; no version, checksum or source line changed. Anything else back to security before the
   save. This is the case security-r1-deps.md:251 sends back ("aws-lc-*").
4. **The feature tree:** `cargo tree --locked --offline -e features` shows rustls, quinn-proto and rustls-webpki without
   aws-lc-rs; web-transport-quinn with `ring` only; rcgen with `ring` and `crypto` from npt-server (and, in
   `--all-features` builds, what wtransport's `self-signed` adds, named), never `aws_lc_rs`, `fips`, `pem` or
   `x509-parser` from npt-server; `time` only transitive (item 5's ruling needs no direct `time`).
5. **What `default = ["wt-wtq"]` puts in the room binary:** web-transport-quinn, web-transport-proto and url 2.5.9 now
   in every default build of npt-net's dependents (npt-server, npt-bots): the transport's HTTP/3 parsing is the
   intended surface; no `Url::origin()` on client input (SEC-R1D-1).
6. **The bites gate:** `bites` is never in any `default`; the RS build in U8's package uses default features only (no
   `--all-features`, no `--features bites`), so `Bite::BudgetX2` is never in a run binary.
7. **The TLS change's reach:** ring's key exchange groups (X25519, P-256, P-384; no X25519MLKEM768 hybrid, which
   rustls offers by default only with aws-lc-rs) and ECDSA P-256 signing for the dev certificate; every R1 browser
   offers X25519, so a note, not a blocker, unless security finds otherwise.
8. **The tools and the audit:** its yes again to cargo-deny 0.22.0 and cargo-audit 0.23.0 (cleared for U6a's Block 0
   only: security-u6a-deps.md section 6); `cargo deny --locked check advisories bans sources` and `cargo audit` on the
   final lock, clean as u6a-block0.md:36-55 defines it, with the database's date; dependency-check.md, one line per
   changed file.

## 5. The build trap: rcgen's validity dates and `time`
**Ruled: the lean, with five terms.** rcgen 0.14.10 exports `date_time_ymd` (lib.rs:44-47) and `time` is already in
the lock; the type need never be named. Terms: (1) the civil date from one reviewed `SystemTime::now` read in cert.rs
(the one allow in server/clippy.toml, with its reason: calendar validity), by a days-to-civil routine tested on fixed
dates (1970-01-01, 2000-02-29, 2026-11-22, 2100-03-01, a year end); (2) `not_before` = `date_time_ymd` of today (00:00
UTC) and `not_after` = `not_before + std::time::Duration::from_secs(13 × 86,400)` (time 0.3's `Add<std::time::Duration>`
names no `time` item) or `date_time_ymd` of the civil date 13 days on, the same in both cases; (3) a clock before
1970 or past year 9999 is exit 2, never a panic (rcgen's `date_time_ymd` `expect`s a valid date:
certificate.rs:1040-1047); (4) `cert_validity_ns` = 13 × 86,400 × 10^9 exactly, from the same two values, and Block
6's test reads the DER's validity back; (5) this read never feeds any timing series (U6a's clock item stays the only
monotonic source). If `Add<std::time::Duration>` will not compile without naming `time`, the second form holds; only
if both fail does the build stop for a manifest change.

## 6. Anything else (findings on the one scale)
- **ADV-U6BB-1, Medium (F1):** `begin_tick` must be forwarded by the blanket Box impl. Q-U6B-1 term (2); condition 1.
- **ADV-U6BB-2, Medium (F2):** TickLoop's I/O send site joins U6b's lane. Q-U6B-1 term (3); condition 1.
- **ADV-U6BB-3, Medium (F3):** WsClient's limits are the client's own: tungstenite's frame and message limits at least
  the largest room message plus its 1 B tag (a FullState of 65,536 + 5 B: catalogue :73-76), not the server's 513 B;
  its write side keeps the catalogue's client limits. Block 6's join test catches it late; condition 3 fixes it now.
- **ADV-U6BB-4, Medium (F4):** Q3's window and form against U8's brief. Q-U6B-3 terms (2) and (3); condition 2.
- **ADV-U6BB-5, Low (F5):** one size figure (section 3); condition 4.
- **ADV-U6BB-6, Low (F6):** `ConnCap` (or whatever form both binds share) among the new pub items, the netcode seat
  reading it; condition 5.
- **ADV-U6BB-7, Low (F7):** the lock diff's removed set named exactly (section 4 item 3); condition 4.
- **ADV-U6BB-8, Low (F8):** RC-12's order reads "the four parts reach R1.md before Stop B's read" (U8 builds beside
  U6: replan-units.md:197-198; UP-STAMP item 3's purpose, advisor-u3c-plan.md:187-188); condition 5.
- **ADV-U6BB-9, Low (F9):** the two 20-iteration kick tests fit the 5 s real-clock cap (run concurrently, or fewer
  iterations with the reason stated); SERIES.md:616's second clause (which names the plain path writes) goes to U6e's
  brief with the per-second merge, U6b writing only `ws_wrapper` 0 and fix 1's inputs on that path; condition 5.
- **Right as written:** the blocks' order and the one cargo lane; the cap at the first QUIC packet and the TCP accept,
  before TLS, `Incoming::refuse()` and an immediate drop; the deadlines over QUIC, HTTP/3, TLS and the upgrade, from
  the accept stamp carried into `Decoder::new` (plan.md:516; pump.rs:13-14); arrival stamps at the reader task; the
  closing tasks before the permit returns; the pin keeping the signature check; the key never written; E22 and no
  Origin check; the bites per instance; the plants, each failing by name; C-U6-19's line for U6b; carries SP-1 to SP-8
  and RC-1 to RC-12 quoted word for word (I checked SP-5 to SP-8 and RC-12 against plan.md:1254's split lines 5-9 and
  22); one builder, the ff-only start, agents never commit, Write and Edit only; one round with netcode-architect,
  test-engineer and security-auditor at xhigh (security's seat by NPT-TRIM, history.md:7590, and replan.md:93); the
  SIZE-CHECK stop past 6,000.
