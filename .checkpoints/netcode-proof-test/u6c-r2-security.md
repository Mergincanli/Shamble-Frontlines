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
Runs: none planned (the builder's logs are the evidence); a run only if a claim truly needs one, named here.

## 2. Notes (one line per finding)
- N0 (fact): the diff's serve.rs lines are W serve.rs:202-206 (the NN filter) and :257-261 (the accept error); server
  probe.rs's are :19-20 (module doc), :176-179 (edge_events), :282-283 (the pin), :465, :553 (edge calls), :561-570
  (leg_series' call), :604-609 (measured_end), :640 (probe_window_ns). No run made; no Bash beyond sha256sum and git diff.
- N1 SEC-U6C-1 (closed): serve.rs:257-261 sleeps 100 ms on every accept error, then retries: at most 10 failed accepts a
  second, no spin, nothing allocated per turn; the runtime has the timer (serve.rs:287-290 `enable_all`; the test's
  serve_tests.rs:53-55 too), so the sleep cannot panic. Lock-out: no. On Linux the errors that reach this branch are local
  ones only. Sources read: (a) net/socket.c, v6.1 and master: `__sys_accept4_file` takes the fd (`get_unused_fd_flags`)
  before `do_accept` dequeues, so EMFILE leaves the connection queued for the next try; `do_accept` calls
  `getname(newsock, .., 2)` and maps its failure to ECONNABORTED; (b) net/ipv4/af_inet.c, v6.1 and master: `inet_getname`
  returns ENOTCONN for TCPF_CLOSE only when `peer == 1`, so accept's 2 returns the address of a client that reset in the
  queue (no error); (c) net/ipv4/inet_connection_sock.c master: `inet_csk_accept` fails only with EINVAL (not listening),
  EAGAIN (tokio's Pending) or a signal, and never reads the child's sk_err. So a remote client cannot make one accept fail
  on its own; its only route here is fd or memory exhaustion (round 1's scenario), and then the queue holds real clients
  until an fd frees and the next try (<= 100 ms) takes them. Successful accepts do not sleep, so the loop catches up after
  a stall. The lock-out that remains is the exhaustion itself (round 1's cut items: no connection cap, no handshake or
  read deadline; LimitNOFILE in U8's unit, Stop B's), not the sleep. Prior art (d) hyper v0.14.27 src/server/tcp.rs:
  `is_connection_error` (ConnectionRefused, Aborted, Reset) retried at once, every other error sleeps 1 s (EMFILE, ENFILE
  in its doc). Our blanket sleep would matter only on an OS whose accept returns per-connection aborts (the BSDs); the
  host runs on Linux only, so this is a reuse note for SEC-U6C-2's STATUS follow-up, not a finding. Confidence: high
  (kernel source read at two versions).
- N2 SEC-U6C-3 (closed): serve.rs:202-206 accepts exactly the 32 names 00-31: `len() == 2` (bytes, checked first, so a
  long digit run is refused in O(1) and never parsed), every byte ASCII 0-9 (refuses `+5`, `-5`, ` 5`, an Arabic-Indic
  digit (2 UTF-8 bytes), `%3` and any percent form, none decoded), then the parse (cannot fail on two ASCII digits; 32-99
  refused). Empty: `/npt/cert-hash-.json` gives "" (len 0), 404. Overflow: impossible at two digits. Other forms fail the
  exact prefix or suffix (absolute-form target, `//npt/…`, `.JSON`, a `#` fragment), so 404. `replace("NN", n)` (:209)
  puts two ASCII digits into the operator's template: no `/`, `.` or traversal. The `+5` 404 entry is the test seat's;
  load.rs:265's mirror is the netcode seat's (not read).
- N3 SEC-U6C-4 (closed as ruled; the residual is R2-1): server probe.rs:283 pins only when `ProbeDatagram::decode(b,
  None)` is Some, i.e. length 40, magic `NPT2`, leg byte <= 5 (net probe.rs:295-302); evaluated only while unpinned and
  only for LG's IPv4 (`ours` first, short-circuit), O(1) (the length test first). Junk, a port scan or another program on
  LG no longer pins. Flood before LG's first: no growth (records preallocated :473, pushed only below capacity :301-305;
  the fate log and the 512-slot peer table fixed: config.rs:53, :110; impair.rs:4-10); each flood datagram, any source,
  takes one OTHER record, so a flood larger than the slack (rate x 2 s + 10,000, :458) pushes LG's last records out
  (`probe_records_dropped_count`, :645): loud, the same as before the fix. No stall: wait_peer (:417-439) waits for the
  first probe datagram and LG keeps sending, so only a volumetric flood that drops all of LG's datagrams for
  `peer_wait_ns` stops it, and that ends Err (loud). Not decode(b, Some(leg)): LG still sends the last leg's number
  until the new leg's first down (round 1 N10), so None is right.
- N4 SEC-U6C-R2-1 (Low): the decode gate stops junk, not a forger. The format is public (net probe.rs:283-310) and
  decode ignores bytes 5-7 and 32-39, so one crafted 40-byte datagram with LG's spoofed IPv4 and another port, landing
  between the leg's bind and LG's first datagram, still takes the pin (round 1's exploit as written: "a spoofed
  datagram"). Reach unchanged: blind spoofing past the firewall, loud (`probe_other_source_count` near LG's whole send
  count, LG's log with no downs); a false pass needs a forged 10,000 pps stream with guessed echoes and holds (the down
  index is predictable), and I did not check whether the judge reads `probe_other_source_count`. Fix (after R1): a
  per-run tag from both settings in bytes 32-39, checked by decode (a blind spoofer cannot know it); or LG binds a fixed
  source port named in RS's settings and the pin takes only that address. Test: the e2e stranger sends a well-formed
  datagram without the tag; the pin stays LG's.
- N5 SEC-U6C-R2-2 (Low): the words round the pin still give the old rule: module doc probe.rs:8-9 ("the first datagram
  from the peer's IPv4"), Intake's doc :263 ("the first source with the peer's IPv4"), wait_peer's doc :416 and its error
  :431-432 ("no datagram from the peer {} within {} s"). After :283 a peer whose datagrams arrive but do not decode (LG
  and RS from two builds: another length, magic or leg range) ends the leg with "no datagram from the peer", and the leg
  writes no log (F-NO-PEER's test pins that), so the `probe_other_source_count` that would show it never reaches a file:
  R1's operator would look at the firewall, not the binaries. Fix: "no probe datagram from the peer {} within {} s" and
  the three docs to "the first probe datagram". Test: F-NO-PEER's test with a peer sending 8 B junk asserts the message
  names "probe datagram".
- N6 (cleared) FP-2: edge_events(_bound: &ImpairSocket, ..) (:179) takes a borrow it never reads; no new input, no
  panic, no allocation; the two calls (:465, :553) pass `&sock` before `drop(sock)` (:571). The /proc reads are unchanged.
- N7 (cleared) FP-1 in run_leg: the new arguments are settings or the socket's clock, never wire data: `zero + warm`
  (warm <= 600 s), `s.packets` (1-10,000,000), `interval` (SEC / rate.max(1)); `measured_end`'s `k + 1` (:609) is a
  second index. In leg_series at the lines run_leg reaches, the peer's wire values are the echo index (through
  `down_t_in.get(i)`, then `echoed[i]` with echoed's length equal: :782-785) and the hold (checked_sub: :788), as before;
  the new code adds `kept` and `order` (each <= fates.len(), :712-720), the rows from the start's second to the last kept
  t_in's (:733-737; a kept t_in is >= start and <= that last, so `seconds[(sec(t) - first) as usize]` at :775 is in
  range), and `span`'s `t - start_ns` with t >= start (:805). All bounded by the fate log's capacity, which the settings
  size (:459); a peer can fill the log, never grow it. No new panic, no unchecked arithmetic on peer input.
- N8 (outside the diff, Low): between the Intake's pin (poller) and `set_peer` (:484, after wait_peer's 1 ms poll), LG's
  slot is unpinned; 512 or more new spoofed addresses passing the host checks in that window evict it (impair.rs:346-367),
  LG's later fates take a new ordinal and leg_series keeps the first one's only (net probe.rs:701-710, :718): the up count
  falls short, the mark fails loudly. Same window before the fix. Taker: after R1 (U6e or R2's run plan).
