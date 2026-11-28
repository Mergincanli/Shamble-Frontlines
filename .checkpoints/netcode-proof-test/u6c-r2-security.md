# U6c round 2, security seat (security-auditor, xhigh). Read only. 2026-11-24.
Inputs, sha256 checked equal (all six): advisor-u6c-r1.md ad72c0b8…, u6c-review-netcode.md f29e1ac1…,
u6c-review-test.md 79197b42…, u6c-review-security.md c29540fa…, u6c-build.md a9d45dd4…, wip/u6c-fix.patch 0549389e….
Diff: `git diff aad2103… 6c3ff00…` narrowed to crates/host/src/serve.rs (+8/-2) and crates/server/src/probe.rs.
Paths below are under W = .claude/worktrees/agent-a3a3e3fdf559de095/spikes/netcode-proof. No Edit tool in this
session: this file is rewritten whole with Write after each block.

## 1. Plan
1. SEC-U6C-1 (serve.rs accept loop): read the changed lines and what they call (listener.accept, the tokio sleep);
   ask: bounded? which errors land there (per-connection vs process-wide)? can an attacker turn the 100 ms into a
   lock-out (accept-queue fill, EMFILE by holding connections, ECONNABORTED storms)?
2. SEC-U6C-3 (serve.rs cert-hash path): enumerate path forms against the three-part filter (len 2, all ASCII
   digits, parse u8 <= 31): sign, empty, overflow, long digit run, non-ASCII, percent-encoding, the `replace("NN")`
   step on the template.
3. SEC-U6C-4 (server probe.rs Intake::take): read `ProbeDatagram::decode(b, None)` (crates/net, at the lines called):
   what a stranger needs to send to take the pin; what a flood of undecodable datagrams costs before the real peer's
   first (memory per record, the records vector's growth, the decode cost, the leg's stall or wait).
4. FP-2 `edge_events` new signature and FP-1 `run_leg`'s new arguments (server probe.rs changed lines): panic,
   unbounded allocation, unchecked arithmetic on peer-controlled input (`measured_end`'s `k + 1`, `max`,
   `leg_series`' call with `s.packets` and `interval`), reading `leg_series` at the lines run_leg calls.
5. Notes line after each finding; then verdict, table, per-item status, outside the diff, not checked, DONE.
Runs: none planned (the builder's logs are the evidence); none made.

## 2. Notes (one line per finding)
- N0 (fact): the diff's serve.rs lines are W serve.rs:202-206 (the NN filter) and :257-261 (the accept error); server
  probe.rs's are :19-20 (module doc), :176-179 (edge_events), :282-283 (the pin), :465, :553 (edge calls), :561-570
  (leg_series' call), :604-609 (measured_end), :640 (probe_window_ns). No run made; Bash used only for sha256sum, git
  diff and ls.
- N1 SEC-U6C-1 (closed): serve.rs:257-261 sleeps 100 ms on every accept error, then retries: at most 10 failed accepts a
  second, no spin, nothing allocated per turn; both runtimes that run `serve` have the timer (serve.rs:287-290
  `enable_all`; serve_tests.rs:53-55 too), so the sleep cannot panic. Successful accepts never sleep, so the loop catches
  up after a stall. Which errors reach the branch (sources, each read through a fetch tool's excerpts; each claim
  attributed only to the fetch that returned it):
  (a) accept(2), https://man7.org/linux/man-pages/man2/accept.2.html, RETURN VALUE, "Error handling": "Linux accept()
  (and accept4()) passes already-pending network errors on the new socket as an error code from accept()", and it
  advises treating ENETDOWN, EPROTO, ENOPROTOOPT, EHOSTDOWN, ENONET, EHOSTUNREACH, EOPNOTSUPP and ENETUNREACH "like
  EAGAIN by retrying"; its ERRORS entry: "EMFILE The per-process limit on the number of open file descriptors has been
  reached."
  (b) net/socket.c at v6.1 (https://raw.githubusercontent.com/torvalds/linux/v6.1/net/socket.c) and master
  (https://raw.githubusercontent.com/torvalds/linux/master/net/socket.c): `do_accept` calls `getname(newsock, …, 2)` and
  maps a failure to ECONNABORTED (both versions). v6.1 only: `__sys_accept4_file` reserves the fd
  (`get_unused_fd_flags`) before `do_accept` takes the connection from the queue, so an fd failure (EMFILE, per (a))
  leaves the connection queued for the next try (the master fetch did not cover this ordering).
  (c) net/ipv4/af_inet.c at v6.1 (https://raw.githubusercontent.com/torvalds/linux/v6.1/net/ipv4/af_inet.c) and master
  (https://raw.githubusercontent.com/torvalds/linux/master/net/ipv4/af_inet.c): `inet_getname` refuses a TCPF_CLOSE
  socket only when `peer == 1` (both versions), so accept's 2 returns the address of a client that reset while queued
  (no error). Master only: `inet_accept` returns only the listener-side error from `sk_prot->accept`.
  (d) net/ipv4/inet_connection_sock.c, master only
  (https://raw.githubusercontent.com/torvalds/linux/master/net/ipv4/inet_connection_sock.c): `inet_csk_accept` fails only
  with EINVAL (not listening), EAGAIN (tokio's Pending) or a signal, and never reads the child's sk_err.
  Reconciled: (a) is the general contract; for TCP I found no path at the versions read that hands a child's network
  error to accept, so the branch sees local errors (EMFILE, ENFILE, ENOMEM, ENOBUFS). Confidence: medium (excerpts; (b)'s
  ordering at v6.1 only, (c)'s inet_accept and (d) at master only; the deployed kernel's version not read).
  Lock-out, either way: no new one. If such per-connection errors were reachable, stalling the queue would take a
  sustained stream of more than 10 erroring connections a second (each costs the loop 100 ms); round 1's cut items (no
  connection cap, no handshake or read deadline, systemd's soft 1,024 fds with no LimitNOFILE in U8's unit: Stop B's)
  already let about 1,020 idle TLS connections, opened once and held, lock the host out, and that stays the cheaper
  route; under it, EMFILE keeps real clients queued and the next try (<= 100 ms after an fd frees) takes them.
  Prior art (e) hyper v0.14.27 (https://raw.githubusercontent.com/hyperium/hyper/v0.14.27/src/server/tcp.rs):
  `is_connection_error` (ConnectionRefused, ConnectionAborted, ConnectionReset) retried at once, every other error
  sleeps 1 s (its doc names EMFILE, ENFILE). Our blanket sleep departs from (a)'s and (e)'s split only for errors not
  seen on Linux TCP: a reuse note for SEC-U6C-2's STATUS follow-up (any reuse of npt-host, at the security baseline),
  not a finding.
- N2 SEC-U6C-3 (closed): serve.rs:202-206 accepts exactly the 32 names 00-31: `len() == 2` (bytes, checked first, so a
  long digit run is refused in O(1) and never parsed), every byte ASCII 0-9 (refuses `+5`, `-5`, ` 5`, an Arabic-Indic
  digit (2 UTF-8 bytes), `%3` and any percent form, none decoded), then the parse (cannot fail on two ASCII digits; 32-99
  refused). Empty: `/npt/cert-hash-.json` gives "" (len 0), 404. Overflow: impossible at two digits. Other forms fail the
  exact prefix or suffix (absolute-form target, `//npt/…`, `.JSON`, a `#` fragment), so 404. `replace("NN", n)` (:209)
  puts two ASCII digits into the operator's template: no `/`, `.` or traversal.
- N3 SEC-U6C-4 (closed as ruled; its residual is R2-1): server probe.rs:283 pins only when `ProbeDatagram::decode(b,
  None)` is Some, i.e. length 40, magic `NPT2`, leg byte <= 5 (net probe.rs:295-302); evaluated only while unpinned and
  only for LG's IPv4 (`ours` first, short-circuit), O(1) (the length test first). Junk, a port scan or another program on
  LG no longer pins. Flood before LG's first: no growth (records preallocated :473 and pushed only below capacity
  :301-305; the fate log fixed at 2 x records, :459, entries past it counted as `fate_log_full`, impair.rs:338-343; the
  512-slot peer table fixed: config.rs:53, :110; impair.rs:4-10). Each flood datagram, from any source, takes one OTHER
  record and one fate entry, so a flood larger than the slack (rate x 2 s + 10,000, :458) pushes LG's last echo records
  out (`probe_records_dropped_count`, :645) and can fill the fate log, which costs measured records in both directions
  (whether `fate_log_full` reaches the leg's summary I did not read); the same before the fix. No stall: wait_peer
  (:417-439) waits for the first probe datagram; only a volumetric flood that drops every datagram of LG's for
  `peer_wait_ns` stops it, and that ends Err. `decode(b, None)`, not `Some(leg)`, is right: LG still sends the last
  leg's number until the new leg's first down (round 1 N10).
- N4 SEC-U6C-R2-1 (Low; not a new defect: round 1's SEC-U6C-4 exploit as written, "a spoofed datagram", left by the
  ruled fix): the decode gate stops junk, not a forger. The format is public (net probe.rs:283-310) and decode ignores
  bytes 5-7 and 32-39, so one crafted 40-byte datagram from LG's spoofed IPv4 with another port, landing between the
  leg's bind and LG's first datagram, still takes the pin. Reach unchanged: blind UDP spoofing of an allow-listed source
  (RS's firewall admits the game UDP ports from his home IPv4 and LG's only: U8's ops/firewall/rs.txt:10-11, from my U6b
  review's notes, not re-read this round). Loud on what I read: with the pin on the stranger, leg_series keeps only the
  stranger's ordinal's ups (net probe.rs:701-710, :718), so the up count collapses and the 1,000,000-record mark fails,
  and P0 ends "no echo within 30 s" or "no round trip in its window's first 10 s" (server probe.rs:525-526, :401-405).
  Only a forger who keeps sending a full stream at LG's rate with guessed echoes and holds for the whole leg could fill
  the count (the down index is predictable). Fix (after R1): a per-run tag from both settings in bytes 32-39, checked by
  decode (a blind spoofer cannot know it); or LG binds a fixed source port named in RS's settings and the pin takes only
  that address. Test: the e2e stranger sends a well-formed datagram without the tag; the pin stays LG's.
- N5 SEC-U6C-R2-2 (Low, new): the words round the pin still give the old rule: module doc probe.rs:8-9 ("the first
  datagram from the peer's IPv4"), Intake's doc :263 ("the first source with the peer's IPv4"), wait_peer's doc :416 and
  its error :431-432 ("no datagram from the peer {} within {} s"). After :283 a peer whose datagrams arrive but do not
  decode (LG and RS from two builds: another length, magic or leg range) ends the leg with "no datagram from the peer",
  and the leg writes no log (F-NO-PEER's new test pins that), so the `probe_other_source_count` that would show it never
  reaches a file: R1's operator would look at the firewall, not the binaries. Fix: "no probe datagram from the peer {}
  within {} s" and the three docs to "the first probe datagram". Test: F-NO-PEER's test with a peer that sends 8 B of
  junk asserts the message names "probe datagram".
- N6 (cleared) FP-2: edge_events(_bound: &ImpairSocket, …) (:179) takes a borrow it never reads; no new input, no
  panic, no allocation; the two calls (:465, :553) pass `&sock` before `drop(sock)` (:571). The /proc reads are unchanged.
- N7 (cleared) FP-1 in run_leg: the new arguments are settings or the socket's clock, never wire data: `zero + warm`
  (warm <= 600 s), `s.packets` (1-10,000,000), `interval` (SEC / rate.max(1)); `measured_end`'s `k + 1` (:609) is a
  second index. In leg_series at the lines run_leg reaches, the peer's wire values are the echo index (through
  `down_t_in.get(i)`, then `echoed[i]`, whose length equals down_t_in's: :782-785) and the hold (checked_sub: :788), as
  before; the new code adds `kept` and `order` (each <= fates.len(): :712-720), the rows from the start's second to the
  last kept t_in's (:733-737; a kept t_in is >= start and <= that last, so `seconds[(sec(t) - first) as usize]` at :775 is
  in range), and `span`'s `t - start_ns` with t >= start (:805). All bounded by the fate log's capacity, which the
  settings size (:459); a peer can fill the log, never grow it. No new panic, no unchecked arithmetic on peer input.
- N8 (considered, not a defect at R1's rates): between the Intake's pin (poller) and `set_peer` (:484, after wait_peer's
  1 ms poll) LG's table slot is unpinned. Eviction takes the least recently seen unpinned slot (impair.rs:353-362), and
  every datagram that passes the host checks refreshes its slot's `last_seen_ns` (impair.rs:513-516), so evicting LG's
  slot takes 512 new spoofed addresses inside one gap between LG's datagrams (about 100 µs at 10,000 a second):
  volumetric. If it happened, LG's later fates would take a new ordinal and the up count would fall short (loud). That
  LG sends steadily before the pin is round 1's reading (N10), not re-read here.

## 3. Verdict: APPROVE WITH FINDINGS
Two Lows, no Critical, High or Medium. SEC-U6C-1 and SEC-U6C-3 are closed; SEC-U6C-4 is closed as ruled, and its
residual (R2-1) is round 1's own exploit at the same Low, not a new defect; R2-2 is new wording. FP-1's and FP-2's
changed lines add no panic, no unbounded allocation and no unchecked arithmetic on peer input. Under the review budget
both Lows become STATUS follow-ups and trigger no push on their own; while open they keep the PR needs-mustafa
(advisor.md, PR review step 4) until taken as follow-ups. Round 1's host read stands as Stop B's: the host's changed
lines (serve.rs:202-206, :257-261) are read here and change nothing in it.

## 4. Findings
| Id | Severity | What | Evidence (W) | Exploit sketch | Fix | Test |
|---|---|---|---|---|---|---|
| SEC-U6C-R2-1 | Low (round 1's SEC-U6C-4 residual, not new) | The decode gate stops junk, not a forged probe datagram | crates/server/src/probe.rs:283; crates/net/src/l2/probe.rs:295-310 | Spoof LG's IPv4 from another port with one 40 B `NPT2` datagram before LG's first: the pin and the down pacer go to that port; the leg fails loudly (up count collapses, :718) | Per-run tag from both settings in bytes 32-39, checked by decode; or a fixed LG source port in RS's settings, pinned exactly | e2e stranger sends a well-formed datagram without the tag; the pin stays LG's |
| SEC-U6C-R2-2 | Low (new) | The pin's docs and wait_peer's error still say "first datagram" / "no datagram from the peer" | crates/server/src/probe.rs:8-9, :263, :416, :431-432 | None (diagnostics): a binary skew between LG and RS reads as "no datagram", with no leg log to show the other-source count | "no probe datagram from the peer …"; the three docs to "the first probe datagram" | F-NO-PEER's test with an 8 B junk peer asserts "probe datagram" in the message |

Anti-cheat coverage (the surfaces this diff touches; no game input path changes in U6c):
| Action | Where the server validates it | Gap |
|---|---|---|
| Cert-hash fetch `/npt/cert-hash-NN.json` | serve.rs:199-213 (exact prefix and suffix, two ASCII digits, <= 31, capped file) | none |
| Accept of a TCP connection | serve.rs:256-268 (accept, 100 ms back-off on error, TLS on its own task) | round 1's cut items (no connection cap, no deadlines): Stop B's |
| Probe peer's first datagram (the pin) | server probe.rs:277-285 (LG's IPv4 and a decodable probe datagram) | SEC-U6C-R2-1 (a forger), SEC-U6C-R2-2 (words) |
| Probe datagrams after the pin | server probe.rs:286-305 (exact address, decode with the leg, counted, bounded records) | none new |
| Echo index and hold from the wire | net probe.rs:782-798 (`get`, checked_sub; repeats and unmatched counted) | none new |

## 5. Fix-pass items in my scope, by round-1 id
- SEC-U6C-1 (accept-error spin): **closed.** The loop backs off 100 ms per error, at most 10 tries a second; no new
  lock-out (N1). Reuse note (the man page's and hyper's split of connection errors) folded into SEC-U6C-2's follow-up.
- SEC-U6C-3 (`+` in NN): **closed** for serve.rs (N2). load.rs:265's mirror is the netcode seat's; the `+5` 404 entry is
  the test seat's.
- SEC-U6C-4 (pin before decode): **closed as ruled** (junk, scans and other programs on LG no longer pin; a flood grows
  nothing and stalls nothing: N3). Its residual, a forged datagram, is SEC-U6C-R2-1 at the same Low.
- FP-2 (F-P8 = NC-U6C-11), the security side of `edge_events`' new signature: **clear** (N6).
- FP-1 (NC-U6C-1), the security side of `run_leg`'s new arguments and the `leg_series` lines they reach: **clear** (N7).

## 6. Outside the diff
- None found. (N8 was considered and is not a defect at R1's rates.)

## 7. What I did not check
- I ran nothing: no cargo, test, plant or `--list`. I opened no log under .gate/u6c; the fix pass's results (fix-check.log
  11 steps PASS, plant-p18b.log) are as u6c-build.md:265-293 reports them.
- crates/bots/src/load.rs:265's digit check (the netcode seat's) and the `+5` 404 test entry in serve_tests.rs (the test
  seat's).
- The deployed kernel's version on RS; the kernel was read through a fetch tool's excerpts (the fd-before-dequeue order
  at v6.1 only; `inet_accept` and `inet_csk_accept` at master only); the IPv6 accept path (`inet6_getname`) not read; no
  accept error reproduced.
- Whether the judge reads `probe_other_source_count` (R2-1's "a forger with a full stream" case); whether
  `fate_log_full` reaches the leg's summary (N3); LG's probe peer's send pattern before the pin (N8 leans on round 1's
  N10); RS's firewall rule (from my U6b notes, not re-read).
- FP-1's count rule as a measurement (the netcode seat's): read here only for panics, allocation and arithmetic on peer
  input.
- My agent memory not updated: the task names one output file.

DONE
