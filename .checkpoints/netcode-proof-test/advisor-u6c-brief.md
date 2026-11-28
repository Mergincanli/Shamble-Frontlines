# The advisor's check of U6c's brief, rev 1 (2026-11-22)
Brief: u6c-brief.md rev 1, 505 lines, sha256 13b0fe58777585525188adbd9d61cde4bc3a1d70df2306ae78cc88c6ba26ed5b (checked
equal at the start of this check). Drafter: netcode-architect (netcode-u6c-brief.md). Working by main's
`.claude/agents/advisor.md` (read with `git show origin/main:.claude/agents/advisor.md`). Rules read word for word:
NPT-TRIM history.md:7561-7593; his answers of 2026-11-22 history.md:7595-7607; TEST-FIRST history.md:2234-2282.

## Plan
1. Read the rules, the card, the brief and the drafter's notes. DONE (brief hash checked).
2. Read U8's brief rev 1.1 (C10, C1-C4, Q-U8-4/-5, :55-71, :200-240, :383, :416-423, :466, :505-521) and its check.
3. Read U6a's and U6b's briefs rev 1.1 and their checks (conditions the brief cites; Block 0 shape u6b-brief.md:393-400).
4. Read security-r1-deps.md (instant-acme, the 17, SEC-R1D-4, :249-253) and security-u6b-deps.md.
5. Grep the spike's code for each fact the brief leans on (stubs, socket_tests.rs lines, stats.rs, thread.rs, SERIES.md
   rows, check.mjs, clippy.toml, ws_config.rs, config.rs, lab probe, lock entries).
6. Rule the six contradictions, Q-U6C-1 to -8, size and R1's date, Block 0, and the rest; note each finding here.
7. Verdict, paste-ready conditions, answers. Hand back.

## Notes (one line per finding, as found)
- N0 plan.md gained one line at :379 on 2026-11-22 08:50 (commit 4d71571, C-U4F2-8's text), after replan-units.md
  (0d9ccac, 2026-11-21 23:17) was written: every plan.md cite past :379 in replan-units.md and replan.md is one low
  today (U6's row is :1254, U8's :1256; `/npt/cert-hash.json` is :995-996). This explains contradiction (f)'s row
  number; u6b-brief.md's "plan.md:994-995" and this brief's copies of it carry the same off-by-one.
- N1 (f) checked: socket_tests.rs last changed 1ccff06 (2026-11-09), so the carry's :1263-1264 still points at T7's
  doc; the doc block is :1263-1271 today, its stamp claims at :1263-1265 and again in the bite sentence :1270-1271.
  Stamp sites socket.rs:501-503 and :724-726 hold. Carries stay word for word (no document change); Block 2 rewords
  the whole block :1263-1271. Low.
- N2 instant-acme 0.8.5 API (docs.rs, read 2026-11-22): `Account::builder()` and `builder_with_root(pem_path)` are
  hyper-rustls-gated; `builder_with_http(Box<dyn HttpClient>)` and `Account::profiles()` exist; `Order::finalize()` is
  rcgen-gated, `finalize_csr(&[u8])`, `poll_ready(&RetryPolicy) -> OrderStatus`, `poll_certificate` exist;
  `HttpClient::request(Request<BodyWrapper<Bytes>>) -> BytesResponse`; `LetsEncrypt::{Production, Staging}.url()`.
  Block 6's calls all exist. But instant-acme's `ring = [.., "rcgen/ring"]` is non-weak (security-r1-deps.md:70-72),
  and Cargo turns on a same-named feature when a non-weak `dep/feat` names an optional dep, so `rcgen` (and
  `finalize()`) is very likely on: binding check (3)'s "instant-acme with `ring` and `hyper-rustls` only" would bounce
  the save to security for nothing. Low (fails safe); condition.
- N3 tokio-rustls's workspace entry is default-features off, "no TLS 1.2" (Cargo.toml); hyper-rustls brings rustls's
  `tls12`; ws_config.rs:31-32 names TLS 1.3 explicitly, so WsServer holds; Block 5 must name it the same way (brief
  says so). QUIC is 1.3 by protocol. OK.
- N4 Medium: the probe's peer port. `ImpairSocket::set_peer` takes a full SocketAddr (socket.rs:244); the brief pins
  "LG's IPv4" and gives `probe_peer` an ephemeral port, and never says the down pacer waits for LG's first up datagram.
  On the VMs `fw-lg` admits only TCP 22 from RS and ICMP (u8-brief.md:200-201): down datagrams RS sends before LG's
  flow exists are dropped at Hetzner's firewall; loopback tests cannot see it, so it would surface as an INVALID -3 at
  R1. Fix: LG sends first; RS takes the port from the first datagram from LG's IPv4, pins it, then starts the down
  pacer; LG keeps one socket for all legs and tolerates ECONNREFUSED between legs.
- N5 Medium: the echo rule. L1's probe echoes "the newest up datagram the server has received and not yet echoed"
  (lab/src/probe.rs:7) and L2 "keeps this design" (:10-11); SERIES.md:115 files "one per probe round trip". The
  brief's LG echoes "the newest down datagram it has received" in every up datagram (Block 1, C-U6-6), so a down
  datagram is echoed again whenever the next one is late or lost: duplicate samples in a judged series (PM-0001-3's
  L2 P1 RTT mean, marks.json:1009-1030). Fix: not yet echoed, else none.
- N6 (main session's additions, ~20:05 UTC): U8 built (wip/u8-build.patch e9fd605c…, 4,176 added, builder done);
  read u8-build.md whole and the snapshot's npt-cert, npt-load-ctl, r1.sh (room_settings, load_settings, probe,
  probe_presence) and config.mjs. U8's interfaces are proposals (u8-build.md:28-30, :57-62).
- N7 HIGH (ADV-U6C-1): the brief and U8's built package disagree where R1 would fail: (1) U8 calls `npt-host issue
  <word>` with no directory and parses strict stdout lines PROFILE/TXT_NAME/TXT_VALUE/go/SERIAL/SHA256/DONE
  (snapshot :2532-2534, :2585, :2598); the brief takes `<dir>` and prints a profile list on stdout; (2) U8's rooms
  read `/var/lib/npt/tls/chain.pem` and `key.pem` (:1630) while staging then production both write by `create_new`:
  one name means the production order exits 2 on the existing path, in front of him, after his TXT edit; (3) npt-super
  starts the room binary as `--settings <file>` (U6a's worktree super/src/main.rs:212-214, server/src/main.rs:401-405)
  and U8 starts the probe as room 02 with `mode: probe` (:1610, :1796-1798), so `npt-server probe <file>` is never
  started; (4) npt-load-ctl starts `npt-load <settings>` with stdin /dev/null (:3105): the brief's "ends when stdin
  closes" ends it at once; U8's load settings lack header, machine, behaviour, seed and pin fields (:1670); (5) U8's
  `probe_presence` reads one room-02.log with `leg_edge` fields `leg`, `edge`, `cpu_lines`, `udp_drops` (:1810-1835),
  which break marks/README.md:41-43, while the brief writes one log per leg with other names; (6) U8's page URL is
  `/?smoke=1&suite=…&room=NN#t=…` (config.mjs, :115-122) and the brief matches the target exactly: every page 404s.
  None is caught by loopback tests; U6c's Rust fixed after its merge would need a lane slot before R1. Closed by C1
  and the U8 fix-pass message (section 5).
- N8 Medium (ADV-U6C-5): no starter for the page host (U8 F22 = contradiction (b) = Q-U6C-7). Ruled: U8's systemd
  path and service pair (no sudoers line), triggered by production.chain.pem (U8's tls dir 2750 npt-host:npt-room,
  snapshot :857); U6c writes the chain after the key's sync.
- N9 Medium (ADV-U6C-4): the growth factor. U4f2 ~2,643 against 1,700-2,250 (1.2-1.55), U8 4,176 against 2,150
  (1.94), U6a 5,460 against 2,345 (2.33, u6a-build.md:147-150). U6c at 1.9-2.3 x 2,800 is 5,300-6,450: near or past
  SIZE-CHECK's 6,000. P2 (Block 2) sits second, so it cannot be the release valve. Fix: P2 last, a size trigger.
- N10 Low: ImpairSocket's Drop signals its threads and does not join them (socket.rs:440-446), so rebinding the same
  port for the next leg can meet EADDRINUSE; a bounded wait-until-bind. Fate log: fixed capacity, keeps the first
  entries, counts the rest (stats.rs:468-484), and the reader never stalls on an empty pool (socket.rs:131-150), so
  no kernel drop comes from the poller's stop: the edge order as written holds.
- N11 F26 checked: redact-check.mjs:135 HOSTNAME + :229-236 read `crc-32-1.2.2.tgz` as quads 32.1.2.2 / 2.2.1.32;
  check.mjs:2643-2646 runs the tests and the scan. A false positive on a registry tarball name; the fix is narrow.
- N12 Block 0: tokio-rustls's workspace entry says "no TLS 1.2" (Cargo.toml) and hyper-rustls turns rustls's `tls12`
  on workspace-wide; WsServer names TLS 1.3 (ws_config.rs:31-32). instant-acme's DefaultClient platform verifier reads
  the OS store, and rustls-native-certs honours SSL_CERT_FILE / SSL_CERT_DIR: U8's npt-cert runs setpriv
  `--reset-env` (snapshot :2584-2585). `builder_with_root(pem_path)` exists as the other root-store choice.
- N13 U8 F12 = contradiction (a) = Q-U6C-6: U8 built the console fingerprint first and the HTTPS line as the
  cross-check; accepted; it needs the host (N8).
- N14 Outside U6c, for U8's fix pass: U8's `npt-ctl rooms start` runs `npt-super hold <settings>` (snapshot :2803)
  and puts a `super` object inside the room's settings (:1633); U6a's build has `npt-super start|time-starts <flags>`
  (--room, --settings, --log, --run, …: its worktree super/src/main.rs:50-90, :337-342) and npt-server refuses unknown
  settings fields (SEC-U6AD-2). Aligns after U6a merges.
- Notes DONE; the ruling follows.

---

# Ruling

## 1. Verdict: APPROVE WITH CONDITIONS (C1-C9 below)
One High (ADV-U6C-1: the brief's interfaces and U8's built package disagree in six places where R1 itself would fail,
and loopback tests see none of them). C1 closes it as written, with section 5's message for U8's fix pass; applied word
for word, no re-check is needed (the precedent: advisor-u8-brief.md:41-43, advisor-replan.md's High). If the main
session changes C1's substance, C1 alone comes back. Mediums ADV-U6C-2 to -5 were read against their sources in full.
Nothing here changes a pass mark, the judge or marks.json: the new names are SERIES.md rows that U8's script reads.
Risk tier of the work U6c builds: 1 (spike code), with security's seat because the page host and the ACME client face
the internet in every run (replan.md:93-94).

| Id | Sev | Finding | Where | Fix |
|---|---|---|---|---|
| ADV-U6C-1 | High | Six interface mismatches with U8's built package (N7): the issue mode's arguments and stdout lines; one TLS file name for staging and production under `create_new` (the production order exits 2 in front of him); the probe never started by npt-super; npt-load ended at once by a /dev/null stdin; leg-edge names and log layout; the page URL's query 404s | u6c-brief.md:99-100, :108-114, :122-136, :139-154, :287-301; snapshot :1610-1670, :1810-1835, :2532-2598, :3105; config.mjs `pageUrl` | C1, section 5 |
| ADV-U6C-2 | Medium | The probe's peer port: `set_peer` needs a full address (socket.rs:244), LG binds an ephemeral port, and nothing makes LG speak first; `fw-lg` drops RS's unsolicited UDP (u8-brief.md:200-201), so a leg can be lost at R1 only | u6c-brief.md:56-59, :74-76 | C2 |
| ADV-U6C-3 | Medium | The echo: L1 echoes "the newest … received and not yet echoed" (lab/src/probe.rs:7) and L2 "keeps this design" (:10-11); the brief echoes the newest down in every up datagram, so a late or lost down makes a duplicate RTT sample in a judged series (marks.json:1009-1030; SERIES.md:115 "one per probe round trip") | u6c-brief.md:56-59, :92-94 | C3 |
| ADV-U6C-4 | Medium | Size: the last three units grew 1.2-2.3 times their estimates (N9), so U6c's 2,800 is about 5,300-6,450 against SIZE-CHECK's 6,000; Block 2, R1's one piece it can spare, is built second, so it cannot be the valve | u6c-brief.md:66-71, :439-456 | C4 |
| ADV-U6C-5 | Medium | Nothing starts the page host (U8 F22; contradiction (b); Q-U6C-7): no browser leg, no HTTPS cross-check | u8-build.md:148-154; u8-brief.md:205-240 | C1 (f), section 5 item 1 |
| ADV-U6C-6 | Low | Binding check (3) expects instant-acme "with `ring` and `hyper-rustls` only", but `ring` names `rcgen/ring` non-weakly (security-r1-deps.md:70-72), which very likely turns on the `rcgen` feature too: the save would bounce to security for nothing; section 1's "`finalize()` … need not turn on" is likely wrong | u6c-brief.md:162-163, :383-384 | C5 |
| ADV-U6C-7 | Low | Cites: Q-U6C-3's plan.md:994-995 is :995-996 today, :966-971 is :967-972, :970-971 is :971-972 (N0); Block 2 rewords only :1263-1265 while the bite sentence :1270-1271 makes the same stamp claims | u6c-brief.md:70, :128, :145-146, :470 | C8 |
| ADV-U6C-8 | Low | ImpairSocket's Drop does not join its threads (socket.rs:440-446): rebinding the slot's port for the next leg can meet EADDRINUSE | u6c-brief.md:81-86 | C2 |

Count for U6c's brief: 1 High, 4 Medium, 3 Low. F26 (a Medium already on the branch, not U6c's) is ruled apart in
section 6.

## 2. Conditions (paste-ready; the main session pastes them word for word into the brief's section 0.1 as rev 1.1)
**C1 (ADV-U6C-1, High), sections 1 (Blocks 3-6), 2, 4 and 9.** "U8's build finished on 2026-11-22 (snapshot
wip/u8-build.patch, sha256 e9fd605c…; its interfaces were proposals: u8-build.md:28-30, :57-62). U6c builds these
forms, and U8's fix pass (advisor-u6c-brief.md section 5) changes U8's side to match:
(a) The issue mode is `npt-host issue staging|production`, exactly two arguments, writing into the compiled-in
directory `/var/lib/npt/tls` (the library function takes the directory, so tests pass a temporary one; the binary takes
no path); the directory check stays (owned by the calling user, writable by no one else; U8 makes it 2750
`npt-host:npt-room`). Its files are `staging.key.pem`, `staging.chain.pem`, `production.key.pem` and
`production.chain.pem`; the chain is created only after the key's `sync_all` returns, so a watcher of the chain sees
both whole. Stdout carries exactly these lines, in this order, one each: `PROFILE <name>` (the order's profile,
`default` when the order names none), `TXT_NAME _acme-challenge.rs.nt.shamblefrontlines.com`, `TXT_VALUE <dns_value>`;
then it reads `go`; then `SERIAL <lowercase hex>`, `SHA256 <64 lowercase hex digits of the leaf's DER>`, `DONE`.
Nothing else goes to stdout; the directory's profile list goes to stderr as one line `PROFILES <name>,<name>…`
(plan.md:971's record; U8's certificate block keeps stderr in its results).
(b) The probe starts as every RS process does: npt-server's settings gain `mode` (`room` or `probe`; SEC-U6AD-2's
rules), and npt-super's hold form starts `npt-server --settings <file>` through `npt-ctl rooms start NN` (U8 uses room
02: the probe binds the settings' `wt_bind`, UDP 30003, which replaces this brief's lean of 30032). Probe mode adds
`probe_peer_ipv4`, `probe_legs` (default all six), `probe_packets` (default 1,000,000) and `probe_warmup_s` (default
10); it parses and range-checks the room fields the run's settings carry and uses none of them; it mints no token file,
writes no hash file and binds no TCP port; it prints `OPEN` once P0's socket is bound, exits 0 after its last leg, and
on stdin's close ends at once, non-zero, every finished leg's log kept. One log per leg, named from `log_path` with the
leg before `.log` (`room-02.log` gives `room-02-P0.log` to `room-02-B1.log`).
(c) LG's side is `npt-load <settings path>`, one argument, as U8's npt-load-ctl starts it; the settings' `mode` is
`run`, `calibrate` or `probe`. The settings are U8's fields (`room`, `server`, `wt_port`, `ws_port`, `mode`,
`transport`, `clients`, `duration_s`, `token_path`, `log_path`) plus the header's (`run`, `set`, `marks`, `date`,
`commit`), `machine` (LG's role, name, plan, location and address), `behaviour` (from a fixed list), `seed` and
`pin_hex` (64 lowercase hex digits: the leaf the clients pin); SEC-U6AD-2's rules; probe mode reads no token. npt-load
ends at `duration_s` with `DONE` and exit 0. Its stdin is /dev/null under npt-load-ctl, so stdin's close is no end for
npt-load: section 4's sentence "npt-load's `run` and `probe` end the same way, when stdin closes" is struck. TERM
(npt-load-ctl's `stop`) ends it at once, with no `DONE`, recorded as stopped. `READY` and `DONE` go to stdout, which
npt-load-ctl writes to the load's log file.
(d) The probe's records keep Q-U6C-5's names (section 4); U8's proposed `leg_edge` fields `leg`, `edge`, `cpu_lines`
and `udp_drops` break marks/README.md:41-43's naming rule and are not built. In each leg's log, `leg_edge` and
`cpu_stat` are written at `ph` `warmup` (after the bind, before the warm-up) and at `ph` `after` (after the tail, the
socket still bound).
(e) The page host matches the request target's path (up to the first `?`) exactly and ignores any query string, which
it never reads or logs (U8's page URL is `/?smoke=1&suite=…&room=NN…`: config.mjs `pageUrl`). It serves
`/npt/cert-hash-NN.json` for NN 00-31 from the settings' path template (U8: `/var/lib/npt/hash/room-NN.json`) and no
`/npt/cert-hash.json` (Q-U6C-3 as ruled); its TLS chain and key are `production.chain.pem` and `production.key.pem`.
(f) The host's start (Q-U6C-7 as ruled): `npt-host serve <settings.json>` runs until it is stopped; U8's systemd path
and service pair in rs.yaml starts it as `npt-host` once `/var/lib/npt/tls/production.chain.pem` exists, with no
sudoers line.
Section 3 adds a test for each of (a) (the exact stdout lines; a staging then a production order in one directory both
succeed), (b) (probe-mode settings accepted and refused; the per-leg log names in the end-to-end test), (c) (each mode's
settings accepted and refused; the end at `duration_s` with `DONE`) and (e) (`/?a=b` serves the index; `/x?a=b` is a
404), and plants P20 (the query kept in the match: (e)'s test) and P21 (the profile list printed on stdout: (a)'s
test)."

**C2 (ADV-U6C-2, -8, Medium), Blocks 1 and 3, section 3.** "LG speaks first. `probe_peer` keeps one UDP socket for the
whole probe (one flow through both Hetzner firewalls; `fw-lg` admits no inbound UDP from RS: u8-brief.md:200-201),
sends from its start to its `duration_s` without a pause, and counts, never fails on, send and receive errors while
RS's socket is between legs (ICMP unreachable, ECONNREFUSED). In every leg RS's down pacer starts only after the first
datagram whose source IPv4 equals `probe_peer_ipv4` has reached that leg's socket; that address and port are then
pinned by `set_peer` (socket.rs:244 takes a full address), and every other source is dropped and counted; a leg with
no such datagram within its warm-up ends the run non-zero (P0's 30 s rule extends to every leg). Rebinding the slot's
port for the next leg retries for at most 2 s, since ImpairSocket's Drop signals its threads without joining them
(socket.rs:440-446). Section 3 adds a Block 3 test (no down datagram before the peer's first; another source dropped
and counted; P1 binds the port P0 used) and plant P18 (the pacer started before the peer is known), which must fail
it."

**C3 (ADV-U6C-3, Medium), Block 1, C-U6-6's line in Block 3, section 3.** "The echo keeps L1's design
(lab/src/probe.rs:7, :10-11): each up datagram carries the newest down datagram LG has received and not yet echoed,
with its hold, or none; a down datagram is echoed at most once, so `probe_rtt_ns` has one sample per round trip
(SERIES.md:115). Block 3's C-U6-6 line reads "each carrying the newest down datagram not yet echoed and its hold, or
none". On RS a repeated echo index is counted and only its first sample kept. Section 3's `probe_peer` test adds an up
datagram sent with no new down (it carries none), and plant P19 (LG echoes the newest down whether echoed or not) must
fail it."

**C4 (ADV-U6C-4, Medium), section 1's order, sections 8 and 9.** "Block 2 (P2, C-U6-8) moves after Block 6 and is
built last, as the release valve. After Block 6 the builder writes its added lines in its notes; if they pass 5,400,
Block 2 is not built and the main session pulls lever (1): P2 rides in MOVED's brief, before R1's report, named in that
day's message. Past 6,000 SIZE-CHECK applies as written (history.md:2048-2051): the build stops and Mustafa gets three
lines; no split or move is made to get under it. Section 8's size line reads: "About 2,800 estimated; at the 1.9-2.3
times the last units grew (U8 4,176 against 2,150; U6a 5,460 against 2,345) about 5,300-6,450, against the 6,000
stop; Block 2 is built last as the valve." Section 8's lane line reads: "R1 on Thursday 3 December holds with about
one day of slack (advisor-u6c-brief.md section 3), on four assumptions: each lane unit closes in one round within about
a day and a half of its Block 0; the disk is freed before each build to the 4 GiB floor plus about 4 GiB (U6a's end
checks stopped on the floor: u6a-build.md:139-146); no SIZE-CHECK stop; no spend-limit cut longer than half a day.
Three lines go to Mustafa the day U6b is not merged by the end of Thu 26, or U6c by the end of Sun 29, or a lane unit
hits SIZE-CHECK." Lever (2) is taken now: security-auditor's read of the host and the issue mode in U6c's round stands
as Stop B's read of "U6c's host" (u8-brief.md:466-469), so Stop B's round reads U8's package and wrappers, and may start
once U8's fix pass is in. Q-U6C-8 reads "Ruled: (a), with Block 2 last as the valve and lever (2) taken now"."

**C5 (ADV-U6C-6, Low), sections 1 and 5.** "Section 5's binding check item (3) reads "instant-acme with `ring`,
`hyper-rustls` and `rcgen` (its `ring` feature names `rcgen/ring` without `?`: security-r1-deps.md:70-72), never
`aws-lc-rs`, `fips`, `time` or `x509-parser`". Section 1's lean "(instant-acme's `finalize()` is behind its `rcgen`
feature, which the cleared set need not turn on)" reads "(`finalize_csr`, so the key's creation, mode and write stay in
our code and tests; `finalize()` is likely compiled in by instant-acme's `ring` feature and is never called)".
security-auditor's short read covers the eight items of advisor-u6c-brief.md section 4 before Block 0 runs."

**C6 (Q-U6C-1 to -8), section 9.** "Each question reads "Ruled", with the terms in advisor-u6c-brief.md section 3:
Q-U6C-1 (a); Q-U6C-2 (a); Q-U6C-3 (a), per room only, no plain name; Q-U6C-4 (a), one 8 KiB buffer and 32 header
slots per connection, a request that does not fit closed; Q-U6C-5 the names as listed, read by U8's probe block per
C1 (d); Q-U6C-6 U8's form (the console fingerprint first, the HTTPS line as the cross-check); Q-U6C-7 U8's systemd
path and service pair (C1 (f)); Q-U6C-8 per C4."

**C7 (counts), section 3.** "With C1's four tests and C2's, at least +52 measured and +52 bites (Block 2's 0-3 on top,
when built); plants P1-P21 with TE2-TE4 (P18 and P19 from C2 and C3, P20 and P21 from C1)."

**C8 (ADV-U6C-7, Low), sections 1, 4 and 9.** "Block 2 rewords T7's doc block, socket_tests.rs:1263-1271 (its stamp
claims at :1263-1265 and :1270-1271; the carry's :1263-1264 is word for word and stays). This brief's own cites read
plan.md:995-996 (the hash file), :967-972 (staging's profile) and :971-972 (the profile list's record): plan.md gained a
line at :379 on 2026-11-22 (4d71571). `/npt/ssh-host-keys.txt` is served with `Cache-Control: no-store` as the hash
files are (plan.md:1128: "`no-store` on the two files")."

**C9 (section 10).** "Before launch: F26's fix on the lane (advisor-u6c-brief.md section 6); U8's fix pass off the lane
after this rev 1.1 and U6a's merge (section 5's message); U7's drafter takes C1 (e): the page fetches
`/npt/cert-hash-<room>.json` with the `room` of its own query, and the host ignores queries. At U6c's merge the main
session checks U8's side against U6c's merged code, one line per interface (u8-build.md:62)."

## 3. Q-U6C-1 to -8, the size and R1's date
- **Q-U6C-1 (the probe's home): (a), the lean.** SERIES.md:497-501 already names npt-server's binary in a probe mode,
  C-U6-1 names npt-server's runtime, and `StealReader` stays pub(crate) (u6a-brief.md condition 5). The entry is a
  settings `mode`, not a subcommand (C1 (b)): U6a's binary takes only `--settings <file>` (its worktree
  server/src/main.rs:401-405) and npt-super starts it that way (super/src/main.rs:212-214), so the probe starts through
  `npt-ctl rooms start` like every RS process, with U8's CPU arrangement. The netcode seat reads the new pub items in
  place of engine-architect (Q-U6A-7, as U6b followed).
- **Q-U6C-2 (C-U6-3's cadence): (a), the lean.** One log per leg, pre-touched, drained once at the leg's end: C-U6-3
  names this form; no copy under the socket's lock inside the window. Checked: the fate log keeps its first entries
  and counts the rest (stats.rs:468-484), and the reader never stalls on an empty pool (socket.rs:131-150), so the
  edge order as written cannot make a kernel drop at the end edge.
- **Q-U6C-3 (the hash URL): (a), per room only, no plain name.** U8 runs the browser rooms one at a time, NN 10-19
  (u8-build.md:153-154; config.mjs's rooms), and its page URL carries `room` (config.mjs `pageUrl`). A plain name as
  room 00's alias would let a page pin a room it did not join: WebTransport then fails and the page falls back to
  WebSocket, which a -29 attempt would read as the fallback working. Not a marks.json word (no mark names the URL).
  plan.md:995-996 is not edited (paper rule); U8's R1.md lists it beside :287.
- **Q-U6C-4 (the request buffer): (a), the lean,** with the constants named: one 8 KiB buffer and 32 httparse header
  slots per connection; a request that does not parse complete inside them is closed. It is the parser's buffer, not
  the cut cap (no counter, no 431, no timeout: replan-units.md:13-14 stays cut).
- **Q-U6C-5 (the new names): approved as listed** (each ends in its unit or is a flag: marks/README.md:41-43; the
  `probe_` prefix keeps them apart from `l2_summary`'s whole-life fields: SERIES.md:490-491). U8's probe block reads
  them per C1 (d) and section 5 item 4. They are SERIES.md rows read by a run script, not by the judge: no marks.json
  change.
- **Q-U6C-6 (the host-key file and the first SSH): U8's built form.** §1.6's own fallback (plan.md:297-298: the
  Hetzner console fingerprint, him present) checks RS's first SSH; the HTTPS fetch of the key line is the cross-check
  once the page host runs after the production certificate, before any browser room (u8-build.md:47-50). It is not
  looser: the fallback is the plan's, over his two-factor console. U6c serves the file under 4 KiB with `no-store`.
  security-auditor confirms the order at Stop B.
- **Q-U6C-7 (the host's start): U8's option (a), a systemd path and service pair** (u8-build.md:150-152), ruled with
  F22 as one: no sudoers line (the sudo surface Stop B reads stays as built), the start tied to the production chain's
  existence, which C1 (a) makes the last file written; R2, R3 and R5 skip the public order (replan.md:70-71), so their
  hosts never start. Not the drafter's `npt-ctl host start|stop`: a verb is one more sudoers line and one more step to
  forget in the chain.
- **Q-U6C-8 (the size and the levers): (a), changed by C4.** Lever (2) taken now (it costs nothing: security reads the
  host in this round anyway). Lever (1) becomes the valve: Block 2 last, dropped to MOVED only by C4's count. P2 checks
  the stamps R1's self-test reads, so it is built before R1 when the size allows; R1's run does not read it.
- **The size.** The re-plan's 1.35-1.5 factor no longer holds: U4f2 grew about 1.2-1.55 times, U8 1.94 (4,176 against
  2,150: u8-build.md:139-143), U6a 2.33 (5,460 against 2,345: u6a-build.md:147-150). At 1.9-2.3, U6c's 2,800 is about
  5,300-6,450. The stop stays his and stays as written; C4 orders the blocks so that the one piece R1's run can spare
  is the last built. For the main session, not this brief: U6b's 2,650 at the same factor is about 5,000-6,100, so its
  launch prompt could name its own last-built valve.
- **R1 on Thursday 3 December: it holds, with about one day of slack.** The lines-a-day reckoning (1,175 merged a day)
  no longer predicts the calendar: U6a built 5,460 lines in about 85 minutes, a day before its planned start, so the
  building is not what takes the days. What takes them is each unit's Block 0 and DEP-HOLD read, end checks, plants,
  round, fixes and merge, plus the disk floor (U6a's end checks stopped on it: u6a-build.md:139-146) and spend-limit
  cuts. With U6a's day gained, the lane reads: U6a's round, fixes and merge Mon 23-Tue 24; U6b Block 0 and build Tue
  24, round and merge Wed 25; U6c Block 0 and build Thu 26, round Fri 27, fixes and merge Sat 28; U7's Rust half Sun 29;
  Stop B's round off the lane (lever (2): U8's package only) from about Sat 28, his Stop B steps Tue 1; Wed 2 spare.
  **Assumptions** (C4's line): one round per lane unit, closing within about a day and a half of its Block 0; disk
  freed before each build; no SIZE-CHECK stop; no cut longer than half a day. **Triggers for the three lines** (to him
  that day, in advisor-u6b-brief.md section 3's shape with today's numbers): U6b not merged by the end of Thu 26, U6c not
  merged by the end of Sun 29, or any lane unit hitting SIZE-CHECK. Not a decision today: one line in the day's message.
- **Levers to take now:** lever (2) (above); Block 2 last (C4); security-auditor's Block 0 read now, beside this rev
  1.1, so Block 0 runs the hour U6b merges (section 4); the disk freed before U6c's launch by u4f-brief.md section 4's
  rules (U6c adds hyper, h2 and a release example build to the shared target); U8's fix pass off the lane now, so Stop
  B's round reads an aligned package and U6c's Rust needs no lane slot after its merge.

## 4. Block 0: what security's reads already cover, and what its short read must cover
**Covered already** (security-r1-deps.md, read whole): instant-acme `=0.8.5` with default features off and `["ring",
"hyper-rustls"]`, CLEAR WITH CONDITIONS (DefaultClient `https_only` with the platform verifier or `with_roots`, no
custom plain-HTTP client, SEC-R1D-4: :225-227, :212-214); its 16 crates at exact versions with their checksums
(:228-230, :258-273), among them http-body-util 0.1.5; http 1.5.0, httparse 1.10.1 and bytes 1.12.1 already locked
(:33; Cargo.lock:119-122, :459-472); pem 4.0.0 and base64 0.23.1 entering with instant-acme (:61-62, :70-72); OSV
clean at every pinned version (:99-101, :183-188). U6b's read (security-u6b-deps.md) covers the provider and
web-transport-quinn, not U6c.

**The short read must cover, before Block 0 runs (one answer, then dependency-check.md and the save):**
1. **Exactly the content:** section 5's (a) workspace lines (the instant-acme line as written; httparse, http, bytes and
   http-body-util as exact pins) and nothing else; (b) crates/host's [dependencies] and [dev-dependencies] as listed;
   (c) crates/bots' tokio, serde, serde_json and the npt-server dev-dependency; no `features`, `default-features`,
   `optional` or version key beyond those written.
2. **The direct entries:** httparse becomes our own parse of client bytes on TCP 31000 (one 8 KiB buffer, 32 header
   slots: Q-U6C-4); http, bytes and http-body-util are dev-only (the fake directory) and reach no shipped binary.
3. **rcgen's `pem` in crates/host:** a feature beyond its pin's condition (`default-features = false, features =
   ["ring"]`: :224). instant-acme's `ring` turns it on anyway, so it is in every workspace build, the room binary
   included (U8 builds the workspace: u8-brief.md:194-195), as dead code; the yes or no on that.
4. **npt-bots:** tokio from the workspace entry (no `signal`, `process` or `fs`: SEC-U6AD-1's set), so npt-load stops
   by its duration or by TERM's default action; npt-server as a dev-dependency only, never in npt-load's release
   binary.
5. **The root store** (U6-Q18's term): `Account::builder()`'s platform verifier (rustls-platform-verifier 0.7.1, the
   OS store, which rustls-native-certs lets SSL_CERT_FILE and SSL_CERT_DIR move: U8's npt-cert clears the environment,
   setpriv `--reset-env`, snapshot :2584-2585) or `Account::builder_with_root(pem_path)` (a hyper-rustls-gated choice
   on docs.rs: roots from a root-owned file, staging's and production's apart). The drafter's lean is the platform
   verifier; webpki-roots is in no lock.
6. **The feature tree** (binding check item (3) as C5 words it): hyper with client, http1 and http2, no server;
   hyper-rustls with ring, tls12, native-tokio and the platform verifier; rustls's `tls12` now on across the workspace
   (tokio-rustls's workspace comment says "no TLS 1.2"): every server config names TLS 1.3 (ws_config.rs:31-32; Block 5;
   QUIC is 1.3 by protocol); the workspace comment's change, if any, is Block 0's.
7. **The lock diff:** exactly the 17 at those versions and checksums; the arrays naming base64 or pem gain versions;
   two duplicate warnings (base64, pem) under `multiple-versions = "warn"`, not errors.
8. **The tools:** its yes to cargo-deny 0.22.0 and cargo-audit 0.23.0 again (cleared per unit so far).

## 5. U8's fix pass (paste-ready message for a builder on the snapshot)
**When and how:** now, off the lane (no cargo), one general-purpose builder at xhigh in its own worktree on
wip/u8-build.patch (e9fd605c…), after U6c's rev 1.1, F26's commit (section 6) and U6a's merge (item 8). Not a review
round: it finishes U8's build against the interfaces now fixed, so Stop B's round (U8's one round) reads them aligned.
Its result is a new snapshot, its notes in u8-build.md. The message, word for word:

"U8's fix pass (the advisor's U6c brief check, advisor-u6c-brief.md section 5). Apply wip/u8-build.patch in your
worktree, then make exactly these changes, each with its test, and nothing else:
1. The page host's start (F22; Q-U6C-7 ruled): rs.yaml gains `npt-host.path` (`PathExists=/var/lib/npt/tls/
   production.chain.pem`) and `npt-host.service` (`User=npt-host`, `Group=npt-host`, `NoNewPrivileges=yes`,
   `ExecStart=/opt/npt/bin/npt-host serve /etc/npt/host.json`, `Restart=no`), installed by root from the root-owned
   clone, and `/etc/npt/host.json`, root-owned 0644: bind `0.0.0.0:31000`, chain and key
   `/var/lib/npt/tls/production.chain.pem` and `production.key.pem`, dist `/var/lib/npt/dist`, hash template
   `/var/lib/npt/hash/room-NN.json`, host-key file `/var/lib/npt/dist/ssh-host-keys.txt`. No sudoers line names
   npt-host or systemctl. r1.sh's browser block and the HTTPS host-key cross-check first check `systemctl is-active
   npt-host` and stop if it is not active. Test 1 asserts the pair and the settings; test 3 asserts no such sudoers
   line. The 32 KiB check still holds.
2. The TLS names: every room's `ws_cert_chain` and `ws_cert_key` are `/var/lib/npt/tls/production.chain.pem` and
   `production.key.pem` (staging writes `staging.*` beside them; one name would make the production order exit 2 on the
   existing path). npt-cert's header names the four files. npt-cert's call `npt-host issue "$PROFILE"` and its line
   protocol stay; the issue mode prints its profile list on stderr as `PROFILES …`, which the cert block's results keep.
3. The hash URL: R1.md and the comments name `/npt/cert-hash-NN.json` (the page derives NN from its `room` query);
   plan.md:995-996 joins the superseded lines beside :287.
4. The probe: `room_settings 02 probe` adds `probe_peer_ipv4` (LG's IPv4), `probe_legs`, `probe_packets` and
   `probe_warmup_s`; the probe mints no token, so the probe block stores an empty token for room 02 (`printf '' | lg
   "token 02"`) in place of `rooms tokens 02`; LG's `duration_s` is the probe's planned total plus 60 s (six legs at
   warm-up + window + tail + about 10 s of set-up each: about 690 s), and the block waits for the probe's own exit in
   npt-super's log (up to that total plus 120 s), not for LG's; `probe_presence` reads the six per-leg logs
   `room-02-<leg>.log` (P0, P1, P2, P3, P4, B1): in each, `leg_edge` at `ph` `warmup` and `after` with
   `udp_row_found` 1 at both and `udp_drops_count` at both, equal at both on P1 (C-U8-6), and `cpu_stat` at both with
   `cpu_user_count`; a missing read makes that leg INVALID; `keep` copies the six. A test on fixture logs (present,
   a missing edge, a P1 drop). R1.md's probe block hours are re-checked against U6c's container reading.
5. npt-load: `load_settings` adds `run`, `set`, `marks`, `date`, `commit`, `machine` (role `LG`, name `LG1`, plan
   `CAX21`, location `fsn1`, address LG's IPv4), `behaviour`, `seed` and `pin_hex` (on `wt` the room's hash file's
   `value_hex`; on `ws` the leaf hash of the chain that room's WsServer presents: the production order's `SHA256` line
   when the room names the production chain). npt-load ends at `duration_s` (stdin stays /dev/null); `stop` stays the
   abort path; where a block needs LG's clients up (the DelayStream self-test's window mark: u6b-brief.md condition 3),
   r1.sh waits for `READY` in `lg "log NN"`.
6. F26: test 6 (ops/tests/redact.test.mjs) asserts no finding over the spike folder; the pre-push hook's EXPECTED and
   the operator test take redact-check.mjs's new sha256 from F26's commit.
7. The stale knob lines: none to edit (the brief is not rewritten); U8's report names u8-brief.md:383's parenthesis,
   :505-507 and :521 as superseded by C10 in section 0.1.
8. npt-super and the room's settings (after U6a merges): `npt-ctl rooms start NN` calls npt-super as U6a's merged code
   takes it (U6a's build today: `npt-super start` with `--room`, `--settings`, `--log`, `--run` … flags:
   super/src/main.rs:50-90, :337-342; not `hold <path>`), and `room_settings` writes only the fields U6a's, U6b's and
   U6c's merged settings types accept (they refuse unknown fields: SEC-U6AD-2), so the `super` object moves to
   npt-super's flags.
Report: each change with file:line, the tests and plants with their counts, and added lines; never commit."

## 6. F26: check.mjs's redact step on the branch (its own ruling, apart from U6c's verdict)
- **The defect** (checked): redact-check.mjs:135's HOSTNAME pattern with :229-236's rDNS reading takes
  `crc-32-1.2.2.tgz` in `"resolved": "https://registry.npmjs.org/crc-32/-/crc-32-1.2.2.tgz"` as the labels crc, 32, 1,
  2, 2, tgz, whose quad 32.1.2.2 (reversed 2.2.1.32) is no allowed address; check.mjs:2643-2646 runs the scan over the
  spike, so the step fails on the branch head and on every later run (U6a's merge check first). Nothing leaked.
  Severity Medium: a real defect with a workaround (it blocks every check.mjs run until fixed, and CI does not run it).
- **The fix: (i), narrowed as far as it goes.** In `scanText`, a HOSTNAME match lying wholly inside the path of a URL
  whose host is exactly `registry.npmjs.org` and whose path has npm's tarball form (`/<name>/-/<file>.tgz` or
  `/@<scope>/<name>/-/<file>.tgz`) is not given the host-name reading; every other rule (dotted, IPv6, IPv4-mapped,
  URL-decimal, tokens, canaries) still scans that text. Not (ii): skipping package-lock.json would leave a `resolved`
  URL to any other host unscanned (U7's lock will be the next), and loosens the check beyond the defect. Not an allow
  in check.mjs's step: the pre-push hook runs the same file on the laptop.
- **Tests** in redact-check.test.mjs: the line-606 form passes; the same tarball file name under another host's path
  still fails (`https://example.org/crc-32/-/crc-32-1.2.2.tgz`); a dotted address inside a registry path still fails;
  an address-shaped host name as a URL's host still fails. **Plant:** the host condition dropped (the skip for any
  host): the example.org test must fail; restored and sha256-checked.
- **Who:** the main session, as its own commit on the lane before U6a's merge check (as it proposed): the file is a
  spike tool, not a gate or kit file (advisor.md's list), inside the plan, undoable, and touches no pass mark, money or
  main (TEST-FIRST rule 2), so it goes by this ruling, named in the day's message. **Reader:** security-auditor at xhigh,
  one short read of the diff, tests and plant before U6a merges (it is the leak scanner every later check and the
  laptop's pre-push hook rely on). **U8's test 6 changes with it:** to "no finding" (section 5 item 6), with the hook's
  EXPECTED hash.

## 7. The six contradictions
- **(a) Host keys before the first SSH (plan.md:286-290 against replan-units.md:110-111, u8-brief.md:416-423).** Real:
  under DNS-01 by hand the order runs over SSH. Ruled with U8's F12 and Q-U6C-6: U8's built form stands (the console
  fingerprint first, the HTTPS line after the production certificate and the host's start); no plan.md edit (U8's
  R1.md lists :287); U6c's brief: C6's Q-U6C-6 words. U8: no fix pass item (built); security-auditor confirms at Stop B.
- **(b) No starter for the page host (u8-brief.md:205-240).** Real (U8's F22). Ruled with Q-U6C-7: U8's systemd path
  and service pair. U6c: C1 (a) and (f) (chain written last; `serve` runs until stopped); section 2's "the host's start
  verb (not in u8-brief.md rev 1.1: Q-U6C-7)" reads "the host's start: rs.yaml's npt-host.path and npt-host.service
  (U8's fix pass)". U8: fix pass item 1, now.
- **(c) One `/npt/cert-hash.json` against a room per mark and browser (plan.md:995-996, u6b-brief.md:64-66 against
  u8-brief.md:55-63, u6b-brief.md:212-214).** Real. Ruled Q-U6C-3: per room only. U6c: C1 (e). u6b-brief.md is not
  edited (its condition 4 fixes the file's content and the serving rule, not the URL). U8: fix pass item 3, now. U7's
  drafter: C9.
- **(d) The knob still given to U6c (u8-brief.md:383, :505-507, :521 against C10, :97-101).** Real but harmless: section
  0.1 opens with "each replaces or adds to the section it names", and C10 names :505-507's and :521's sections; :383's
  parenthesis is not named. U8 built no Rust, so nothing was built from it. No edit; U8's report names the three as
  superseded (fix pass item 7). Waits for U8's report.
- **(e) U6b's "rcgen without `pem`" (u6b-brief.md:396-397) after U6c's Block 0.** Real and expected: it holds at U6b's
  Block 0, before instant-acme enters; instant-acme's `ring` turns `pem` on (security-r1-deps.md:70-72). No edit to
  u6b-brief.md; U6c's binding check item (3) already says so (u6c-brief.md:384-385); security's short read answers
  section 4 item 3. Nothing for U8.
- **(f) C-U6-8's cite and replan-units.md:3's row.** Checked: socket_tests.rs is unchanged since 1ccff06 (2026-11-09),
  so the carry's :1263-1264 still points at T7's doc (its stamp clause); replan-units.md:3's :1253 went stale when
  4d71571 inserted plan.md:379 (2026-11-22 08:50), which shifted every later plan.md line by one. No document changes:
  carries stay word for word, and older papers are not rewritten (advisor-u6b-brief.md:143-145). U6c cites live lines
  (C8). Nothing for U8.

## 8. The rest
- **Carries met by number:** C-U6-1 to -8 (Blocks 1-3; C3 refines C-U6-6's composition; C4 moves C-U6-8's block
  last, its words unchanged); u6a-brief.md conditions 2 (U6c's line stands), 3 (no longer applied to npt-load: C1 (c))
  and 5; u6b-brief.md condition 4 (C1 (e): per request, 1 KiB, verbatim, `no-store`); U8's C10 items (Q-U8-4's
  interface with C1 (a)'s lines; Q-U8-5's reads with C1 (d); the probe's port, now room 02's; npt-load's interface for
  C4 with C1 (c)); advisor-u6-brief.md conditions 1, 6, 7 and 8 (U6-Q2, U6-Q17 term (3), U6-Q18 with its revocation term
  lapsed by the re-plan's cut, replan-units.md:32); Q11 and Q12; plan.md:970's 90-day fallback not taken (production
  fails closed, exit 3); security-r1-deps.md's instant-acme conditions and SEC-R1D-4.
- **Tests, bites, plants:** no bite is right (-3's bites are npt-l1's: l1/src/lib.rs:780-782); the plants are each tied
  to a named test; C1-C3 add P18-P21. The fake directory needs the JWS payload decoded to assert the order's `profile`
  (P15): a base64url decoder in test code, since base64 is no direct dependency and none is added.
- **Bans:** the brief's list stands (.claude/rules/netcode.md for crates/server and crates/net; the host's cut limits
  stay cut, so its connections are bounded only by the firewall, as the re-plan decided). npt-net has no clippy.toml,
  so `Pacer` and `probe_peer` read the clock there with no allow; npt-load's one reviewed allow stays.
- **Files:** as section 6 lists, plus crates/server's settings type (C1 (b)'s `mode`); every manifest Block 0's.
- **Review seats:** netcode-architect, test-engineer and security-auditor at xhigh, one round (replan.md:93-94; the
  engine-architect seat replaced as Q-U6A-7 ruled). Security's read of the host and the issue mode stands as Stop B's.
- **Pass marks, the judge, marks.json:** untouched. The probe computes the judged E form already ruled (U3D-L0,
  U3D-BAR); the new names are SERIES.md rows; the hash URL is in no mark.
- **Not verified here:** instant-acme's source (its API read on docs.rs today, not its code: the security read streamed
  the crate); Cargo turning on the same-named `rcgen` feature (reasoned from Cargo's feature rules, not run); whether
  Hetzner's firewalls track UDP flows (every LG client relies on it already; C2 makes the probe rely on it the same
  way); U6a's npt-super and npt-server read in its unmerged worktree, which its round may change; no RS, LG, browser or
  Let's Encrypt.

## Decisions needed
None. Every ruling here is inside the plan, undoable, and touches no pass mark, money or main. The three lines on R1's
date go to him only on a trigger in section 3.

## For Mustafa
Nothing from this check. The day's message names: U6c's brief rev 1.1 (one High closed by its conditions); R1 on
Thursday 3 December holds with about a day of slack, and the triggers that would bring him three lines; F26's fix to
the leak scanner (a false alarm on a package name), read by security-auditor.

DONE
