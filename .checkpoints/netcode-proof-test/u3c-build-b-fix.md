# U3c build B fix: critic B's findings (2)-(9) on npt-tcpinfo and check.mjs's deps step

Started 2026-11-07. Worktree /home/user/Shamble-Frontlines/.claude/worktrees/agent-a3dfcb36e3c15ab56, branch u3c-b-fix at
539709a (the ff-only merge was refused: the worktree's branch had diverged, so `git switch -c u3c-b-fix 539709a`), then
build B's patch (.checkpoints/netcode-proof-test/wip/wf_87550c07-bbd-2.patch, `.checkpoints/*` excluded) applied clean.
Baseline: `check.mjs` 9/9 (measured 201, bites 214, self 15/15, deps 68 crate files). Nothing committed; the main session
merges. Binding: u3c-plan.md rev 2 §1.8 + [C12] (a)-(g); critic B (u3c-critic-b.md); the main session's brief.
Every cargo command: `cargo +1.99.1`, CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/u3v3-model-target. No
iptables, tc or other network change.

## Plan (written before any code)
1. **(2) Placement.** check.mjs: a small Rust lexer (line and nested block comments out, strings, raw strings and char
   literals kept whole, offsets kept) and an attribute scanner. In npt-tcpinfo every match of the relax pattern must sit
   inside an outer `#[allow(unsafe_code, reason = "...")]` (that exact form) in crates/tcpinfo/src/linux.rs, followed
   (other outer attributes aside) directly by `pub fn srtt_us` or `pub fn unacked_and_unsent`, each exactly once.
   Anything else (an inner attribute, cfg_attr, expect/warn, an allow on a mod, a helper or any other item, a match in
   a string, a third allow, a missing one) is a named problem with its line. TCPINFO_ALLOWS becomes
   TCPINFO_UNSAFE_FNS + TCPINFO_UNSAFE_FILE. Also (critic's bypass list, cheap): a `#[path]` or `include!` in any
   crate whose target leaves that crate's folder fails (it would compile npt-tcpinfo's file into another crate).
   Tests in check.test.mjs: the two critic mutations (module-level allow on `mod linux;` with one function's allow
   removed; the allow moved from srtt_us to a helper), plus cfg_attr/inner/expect forms in tcpinfo, a string match,
   the #[path]/include! bypass; the existing tests kept (messages updated where the wording changes).
2. **(4) Comments.** check.mjs's unsafeProblems docstring and header; workspace Cargo.toml's lint comment; tcpinfo's
   Cargo.toml and lib.rs doc; client-wasm's manifest comment ("the only crate allowed it" is stale). Each states only
   what the deps step enforces.
3. **(5) No libc in npt-net.** dependencyProblems: npt-net has no direct normal/build libc dependency (plan §4.2's C10
   reading: dev dependencies never ship). Test: a normal and a build edge fail, a dev edge does not. The deps step's
   summary line lists what it checks instead of "the dependency rules of plan §4.2 hold".
4. **(6)** tests.rs:4-5 and siocoutq_probe.rs:3-4: `siocoutq-root-check.sh` -> `.py`.
5. **(7) Any descriptor.** unacked_and_unsent: before the ioctl, bounded getsockopt(SOL_SOCKET, SO_DOMAIN / SO_TYPE /
   SO_PROTOCOL) into a c_int (optlen 4; sk_getsockopt copies at most min(len, 4)), one unsafe block in a closure inside
   the allowed function with its own SAFETY comment; anything but AF_INET/AF_INET6 + SOCK_STREAM + IPPROTO_TCP is an
   InvalidInput error; a non-socket is ENOTSOCK from the first getsockopt. srtt_us: getsockopt is socket-only (ENOTSOCK
   before any write on another fd) and every socket's getsockopt honours optlen, so its invariant holds on any
   descriptor: SAFETY comment says so, no extra check. Kernel cites (v6.8 net/socket.c, net/core/sock.c,
   net/ipv4/af_inet.c, fs/ioctl.c) fetched into the scratchpad. Tests: pipe (both fns ENOTSOCK), UDP socket (both
   error), a Unix stream socket, a listening TCP socket (still EINVAL). Lint bites: clippy clean both builds.
6. **(8)** tcp_kernel.rs: the srtt comparison retried until two readings agree (bounded) and counted; the SIOCOUTQ
   comparisons counted in the write loop, plus one at the quiescent point after WouldBlock; asserted minimums.
7. **(3) Root run.** Probe + script header: message k is written only while, just before it, the kernel's packets in
   flight (tcpi_unacked - tcpi_sacked - tcpi_lost + tcpi_retrans, sock_diag) are below tcpi_snd_cwnd (tcp_write_xmit's
   own cwnd test), at most 20; 200 B <= tcpi_snd_mss at GO (else VOID), so each write is one segment and the bytes
   stay <= cwnd x MSS; fewer than 4 writes is VOID. cwnd, mss and in-flight logged per sample. R1/R2 judged on the
   bytes actually written. Dry run kept (no iptables): must still fail R1 and R3.
8. **(9) R4.** No fixed 5 s: SIOCOUTQ = 0 (both readings), wqueue = 0 and every byte delivered at the release sample,
   which is taken once SIOCOUTQ reads 0 or 2 x tcpi_rto (read at RELEASED, the backed-off RTO) has passed. Why it
   cannot flake: the pending retransmission timer is due within one backed-off RTO of the last retransmission (before
   RELEASED); its DSACK ACK reaches S one loopback RTT later; the second RTO is scheduling slack. Header states it.
9. Final: check.mjs 9/9, clippy -D warnings with and without features (host + wasm32 lib), fmt --check; notes; DONE.

## Progress
- (6) done: tests.rs:4-5 and (pending block 7) siocoutq_probe.rs name `siocoutq-root-check.py`.
- (7) done in code: unacked_and_unsent reads SO_DOMAIN, SO_TYPE, SO_PROTOCOL through one closure (one unsafe block,
  its own SAFETY comment, optlen 4 checked on return) and makes the ioctl only on AF_INET/AF_INET6 + SOCK_STREAM +
  IPPROTO_TCP (InvalidInput otherwise); srtt_us unchanged in code, its SAFETY and doc now say why any descriptor is
  sound (getsockopt is socket-only: ENOTSOCK, net/socket.c:2397-2399 and :553-567; optlen bound, tcp.c:4067). Kernel
  sources fetched to scratchpad u3cbfix-linux-v6.8/ (net/socket.c, net/core/sock.c, af_inet.c, af_inet6.c, fs/ioctl.c,
  tcp*.c, tcp.h at the v6.8 tag). New test every_descriptor_that_is_not_a_tcp_socket_is_an_error: pipe (both ENOTSOCK),
  UDP, Unix stream, Unix datagram (SIOCOUTQ InvalidInput; TCP_INFO EOPNOTSUPP, errno 95, on all three), connected and
  unconnected TCP read. IPv6 is off here (socket(AF_INET6) gave EAFNOSUPPORT), so the AF_INET6 branch is untested.
  npt-tcpinfo: 8 of 8 pass; clippy -D warnings clean (all targets).
- (8) done: tcp_kernel.rs's test counts its comparisons: srtt (retried until two srtt_us readings agree, at most 100,
  asserted exactly 1), outstanding_bytes per write (asserted >= 1; this run 1,000 of 1,000) and one more at the
  quiescent point after the writes (retried up to 200 x 1 ms, asserted to have run, 0 < q <= written; this run
  1,000,000 of 1,000,000 B). Passes (0.14 s).
- (2) done: check.mjs gains lexRust (comments blanked; strings, raw strings, char literals kept; offsets kept),
  rustAttributes, itemAfter, tcpinfoProblems (every relaxing in npt-tcpinfo must be an outer
  `#[allow(unsafe_code, reason = "...")]` in crates/tcpinfo/src/linux.rs directly on `pub fn srtt_us` or
  `pub fn unacked_and_unsent`, one each; problems name file:line and the item) and pathProblems (every #[path] and
  include! names a .rs file inside its own crate; the critic's #[path]/include! bypass). TCPINFO_ALLOWS replaced by
  TCPINFO_UNSAFE_FILE + TCPINFO_UNSAFE_FNS. check.test.mjs: 15 -> 21 tests (the critic's two mutations, the exact form,
  inner/cfg_attr/expect/no-reason/extra-lint/raw-reason forms, stacked allows, the stub file, lexer cases, path cases).
  Bites: scratchpad u3cbfix/bites_check.py, 15 of 15 mutations of check.mjs make a test fail. The critic's bypass
  script on the real tree: 7 of 9 caught; the 2 missed are npt-client-wasm's (an exception by manifest, not scanned,
  stated in the docstring). Real-tree bite: mutation 1 planted -> `check.mjs --only deps` FAIL with both problems;
  files restored (sha256 equal to the saved copies), deps + self PASS.
- (5) done: dependencyProblems: "npt-net depends on libc directly" for a normal or build edge (dev does not ship, C10),
  "npt-net is not a workspace member"; test (normal, build fail; dev, through tokio pass). The deps summary line is
  DEPS_CHECKED, which lists each rule it checks instead of "the dependency rules of plan §4.2 hold".
- (4) done: comments now state only what is enforced: check.mjs header, UNSAFE_EXCEPTIONS and unsafeProblems
  docstrings (what is caught, and that npt-client-wasm's files are not scanned), the npt-tcpinfo deps comment ("two
  functions reached through npt-net alone", not "npt-net's one module"); workspace Cargo.toml's lint note (the exact
  placement and every failure the deps step raises); tcpinfo's Cargo.toml (placement; libc the only normal dependency,
  socket2 a dev one) and lib.rs doc; client-wasm's manifest ("the only crate allowed it" was stale).
- (3) done (written, dry-run checked; the root run is the main session's): the probe's write phase follows THE CWND
  RULE, stated in the script's header before any root run: message k is written only if, read through sock_diag just
  before it, packets in flight (tcpi_unacked - tcpi_sacked - tcpi_lost + tcpi_retrans) + 1 < tcpi_snd_cwnd (one packet
  of room for a tail loss probe); the first that fails ends the phase; N = writes made (expected 9 at cwnd 10), VOID if
  N < 4 or tcpi_snd_mss < 200. R1/R2 judged on 200 k / 200 N. Every SAMPLE line now logs in_flight, snd_cwnd, snd_mss.
  Kernel cites: tcp_output.c:2073-2094 (tcp_cwnd_test), :2743-2749, :2829-2875 and :2900 (loss probe);
  include/net/tcp.h:241 (TCP_INIT_CWND), :1253-1256; tcp.c:3785, 3826, 3829-3833 (tcp_info fields).
- (9) done: R4 is an identity with an RTO bound: SIOCOUTQ = 0 (both readings), wqueue = 0 and every byte delivered at
  the release sample, taken once SIOCOUTQ reads 0 or 2 x tcpi_rto (read at RELEASED) has passed; the header states why
  it cannot flake (the pending retransmission timer is due within one backed-off RTO, tcp_timer.c:654-658,
  tcp_input.c:3200-3225; the duplicate segment is ACKed at once with a D-SACK, tcp_input.c:5228-5236; the second RTO
  is slack). The script's wait for the verdict after RELEASED is 30 s (the rule already out), above 2 x tcpi_rto.
- Dry run (no iptables call): R0 pass (srtt 7 us = sock_diag's, cubic set, snd_cwnd 10, snd_mss 32768), 20 writes
  (ACKs flow, so the cwnd rule never stops it), R1 fail 20/20, R2 fail 9/9, R3 fail, R4 pass (bound 408 ms, 0 ms),
  exit 0: the rule bites without the drop. Log: .checkpoints/netcode-proof-test/u3c-build-b-fix-dryrun.txt (moved out of
  results/). The cwnd rule's stop path cannot be exercised without dropping ACKs: not verified here.
  Probe zero-window self-check: PASS (2,803,712 B in all five readings).
- Rust bites (scratchpad u3cbfix/bites_rust.py; each mutation on the real file, restored by sha256): 7 of 7 caught:
  the TCP check's verdict ignored, no getsockopt at all (the pre-fix code), only SO_PROTOCOL checked (all three fail
  every_descriptor_that_is_not_a_tcp_socket_is_an_error); the getsockopt block without SAFETY
  (undocumented_unsafe_blocks) and with two unsafe ops (multiple_unsafe_ops_per_block) fail clippy; npt-net's
  outstanding and srtt comparisons forced never to run fail the tcp_kernel test.
- Final: `check.mjs` all 9 steps pass (measured 202 tests, bites 215; deps 68 crate files; self 21 of 21; redact 10 of
  10). `cargo +1.99.1 fmt --all --check` clean; clippy -D warnings in both builds (check.mjs) and npt-tcpinfo's lib on
  wasm32-unknown-unknown (the stub) clean. Cargo.lock unchanged by this fix (build B's 9 lines only).
- Not done here, by design: the root run (iptables; the main session's); the cwnd rule's stop path and the AF_INET6
  branch of the TCP check are untested in this container (no ACK drop without iptables; IPv6 off). Not in this brief:
  critic B's (f) note (R1's run plan reads srtt against netem's 100 ms; no tc here).

DONE
