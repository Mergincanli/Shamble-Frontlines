# U3c build B: npt-tcpinfo (R20 (a), C12 (a)-(g)), its deps checks and the root-run script

Started 2026-11-07. Worktree /home/user/Shamble-Frontlines/.claude/worktrees/wf_87550c07-bbd-2, branch u3c-build-b at
ebe82b20340cd5e2fbc4e0cd03374a1e9745df1f (the origin branch was not a fast-forward of the worktree's base, so the
switch -c route was taken). Nothing committed: the main session merges. Binding text: u3c-plan.md rev 2 §1.8 and
[C12]; advisor-u3c-plan.md C3, C9, C16.

## Plan (written before any code)
1. **Crate** spikes/netcode-proof/crates/tcpinfo, package npt-tcpinfo:
   - Cargo.toml: workspace package fields, `[lints] workspace = true`; libc (workspace pin =0.2.191) as the only
     normal dependency, under `cfg(target_os = "linux")`; dev-dependency socket2 =0.6.5 (already in Cargo.lock),
     features ["all"] for TCP_QUICKACK (features are not in Cargo.lock).
   - lib.rs: crate-level `#![deny(clippy::undocumented_unsafe_blocks, clippy::multiple_unsafe_ops_per_block)]`.
     Linux module: `srtt_us(fd: BorrowedFd<'_>) -> io::Result<Option<u32>>` (getsockopt TCP_INFO into a zeroed
     buffer of size_of::<libc::tcp_info>(); returned length checked against offset_of!(tcp_info, tcpi_rtt) + 4,
     else Unsupported; the field read safely with u32::from_ne_bytes at offset_of!; 0 -> None) and
     `unacked_and_unsent(fd: BorrowedFd<'_>) -> io::Result<u32>` (ioctl(libc::TIOCOUTQ) into a c_int; Linux's
     SIOCOUTQ is TIOCOUTQ). `#[allow(unsafe_code, reason = "...")]` on these two functions only; one unsafe op
     per block, each with a SAFETY comment stating its invariant. Pure helper rtt_from(buf, len) for the length rule.
   - Non-Linux stub: the same two names, generic over the handle (so it compiles on every target, Windows and
     wasm32 included), returning ErrorKind::Unsupported. Also compiled under cfg(test) on Linux and tested.
   - Only the two functions are pub (plan §4.7, M6); the request constant is a private const.
2. **Tests (C12 e)**, real loopback sockets, judged on identities:
   - the request constant is 0x5411 (= libc::TIOCOUTQ on x86-64/aarch64), not SIOCOUTQNSD 0x894B;
   - EINVAL on a listening socket (tcp.c:624-625);
   - non-reading peer with a small SO_RCVBUF: SIOCOUTQ grows; at a quiescent point it equals /proc/net/tcp's
     tx_queue (tcp_ipv4.c:2899, write_seq - snd_una) and bytes written - the peer's rx_queue;
   - extra (beyond C12 e, non-root): the receiver in delayed-ACK mode (TCP_QUICKACK 0): right after a 200 B write,
     SIOCOUTQ == 200 == tx_queue while the peer already holds the 200 B (rx_queue 200): sent, unacked, notsent 0,
     the state SIOCOUTQNSD would read as 0. Python prototype: 200 of 200 attempts caught it.
   - tcpi_rtt Some(> 0) after traffic; a fresh unconnected socket gives None (srtt_us 0, tcp.c:3842);
   - rtt_from's length rule (short -> Unsupported); the stub returns Unsupported.
3. **npt-net**: Cargo.toml gains `npt-tcpinfo = { path = "../tcpinfo" }` (the workspace's dependency table is not
   touched: only members, the libc pin and the lint comment change there); lib.rs gains one `mod tcp_kernel;` line
   (dead-code allow outside tests, as tcp_model's); new src/tcp_kernel.rs with pub(crate) `srtt_ns(&TcpStream)` and
   `outstanding_bytes(&TcpStream)`, doc comments citing v6.8 tcp.c:630 and :3842; a loopback test.
4. **Workspace Cargo.toml**: members + "crates/tcpinfo"; `libc = "=0.2.191"` in [workspace.dependencies]; the
   unsafe_code comment names npt-client-wasm and npt-tcpinfo under ADR 0002:264-265's rule.
5. **check.mjs deps step (C12 d)** + check.test.mjs tests with bites:
   - dependencyProblems: npt-tcpinfo is a member; libc is its only shipped dependency; only npt-net depends on it
     (any kind);
   - new unsafeProblems(files, read) over spikes/netcode-proof/crates: `allow|expect|warn (… unsafe_code` in a .rs
     file only under crates/client-wasm or crates/tcpinfo; no crate-wide `#![allow(unsafe_code` in tcpinfo; a crate
     manifest sets unsafe_code only in client-wasm; every other crate manifest keeps `[lints] workspace = true`.
6. **Root-run script (C12 e)**, written not run: spikes/netcode-proof/results/u3c-kernel-check/siocoutq-root-check.sh
   + the example crates/tcpinfo/examples/siocoutq_probe.rs. The ACK direction (client -> server, dport P) dropped
   about 1 s by one iptables INPUT rule on lo, removed in a trap, `iptables -S` compared. Writes 20 x 200 B in the
   first 100 ms (before the first RTO); pass rule in the header before any run: SIOCOUTQ == 200 k after write k and
   stays 4,000 through the hold, while tcpi_notsent_bytes (read independently through NETLINK_SOCK_DIAG, no unsafe)
   stays <= 200 and the peer has read every byte; back to 0 after release. A dry-run mode (no iptables call at all)
   checks the plumbing here, where the rule is expected to fail (no drop).
7. Lock update with a normal cargo build, then check.mjs 9/9, clippy -D warnings both builds, fmt; notes; DONE.

## Progress
- Block 1 done: workspace Cargo.toml (members, libc =0.2.191, lint comment); crates/tcpinfo (Cargo.toml, lib.rs,
  linux.rs, stub.rs, tests.rs). `cargo test -p npt-tcpinfo`: 7 passed in 2.21 s. Measured: tcpi_rtt after 20 round
  trips = 2 us (server and client); non-reading peer: 2,808 writes, 2,807,712 B written, SIOCOUTQ first 0 final
  2,803,712 = tx_queue, peer rx_queue 4,000 (2,803,712 + 4,000 = written); delayed-ACK state caught 50 of 50.
  Cargo.lock: +npt-tcpinfo (deps libc, socket2) only, so far.
- Block 2 done: npt-net wired (Cargo.toml path dep `npt-tcpinfo = { path = "../tcpinfo" }`, lib.rs mod line at the
  end, src/tcp_kernel.rs with pub(crate) srtt_ns / outstanding_bytes + one tokio loopback test: passes).
- Block 3 done: root-run probe crates/tcpinfo/examples/siocoutq_probe.rs (modes lockstep, zero-window) and the
  script results/u3c-kernel-check/siocoutq-root-check.py. DECIDED DIFFERENTLY: the script is Python (try/finally +
  SIGINT/SIGTERM/SIGHUP handlers as the trap), not bash, because this agent's guard refuses to run bash, and a script
  I can dry-run here beats one I cannot. Evidence (no network change): zero-window self-check PASS: SIOCOUTQ =
  tcpi_notsent_bytes (sock_diag) = wqueue = tx_queue = 2,803,712 of 2,807,712 written. Dry run (`--dry-run`, no
  iptables call): R0 pass (srtt 41 us, rto 204,000 us, cubic set), writes end at 95 ms, R1 fail 20/20 (SIOCOUTQ 0),
  R2 fail 9/9, R3 fail (total_retrans 0), R4 pass: the rule bites without the drop; exit 0. Log kept at
  .checkpoints/netcode-proof-test/u3c-build-b-dryrun.log (not in results/).
- Block 4 done: check.mjs deps step (npt-tcpinfo rules in dependencyProblems; new exported unsafeProblems +
  UNSAFE_EXCEPTIONS + TCPINFO_ALLOWS; crateFiles walks crates/), check.test.mjs +6 tests (15 of 15 pass). Mutation
  bites (u3c-build-b-bites.py, scratch copies): 11 of 11 mutations make a test fail. Real-tree bite: a planted
  `#[allow(unsafe_code)]` in crates/room/src/planted_bite.rs made `check.mjs --only deps` FAIL ("relaxes unsafe_code
  (only npt-client-wasm and npt-tcpinfo may)"), removed after. First real run found tcpinfo's own doc comments
  counted as attributes (4, not 2): fixed by counting tcpinfo's attributes in code only (comments stripped); outside
  the exceptions the scan stays a plain grep, comments included. Stub on wasm32-unknown-unknown: clippy -D warnings
  clean.
- Block 5 done: full check.mjs 9/9 (first pass: measured 201, bites 214 tests; self 15/15; deps 68 crate files).
  Lint bites (planted, then removed): an unsafe block without SAFETY -> clippy::undocumented_unsafe_blocks error; a
  block with two calls -> clippy::multiple_unsafe_ops_per_block error; an unsafe block in rtt_from (no #[allow]) ->
  rustc "usage of an `unsafe` block" (workspace deny). 20 repeated runs of the tcpinfo tests: 20/20 ok, 2.20 s each,
  delayed-ACK state 50/50 every run; but tcpi_rtt after plain ping-pong read 1 us once (2-4 us otherwise), close to
  the 0 floor, so the srtt tests (tcpinfo and npt-net) now add delayed-ACK RTT samples (srtt then ~21,000 us).
- Block 6 done: the root-run output is now .txt (the repository's .gitignore:7 ignores *.log, and the output is to be
  committed beside the script). R0 changed before any run: srtt_us must equal sock_diag's tcpi_rtt (0 as None), not
  be > 0 (loopback srtt read 5 and 8 us in the two dry runs; 1 us once in the unit tests). Dry run re-run: R0 pass,
  R1 fail 20/20, R2 fail 9/9, R3 fail, R4 pass, exit 0 (u3c-build-b-dryrun.txt, the latest dry run's full output).

## Summary for the main session
- Worktree /home/user/Shamble-Frontlines/.claude/worktrees/wf_87550c07-bbd-2, branch u3c-build-b, base
  ebe82b20340cd5e2fbc4e0cd03374a1e9745df1f; nothing committed.
- Files: spikes/netcode-proof/{Cargo.toml (members, libc pin, lint comment), Cargo.lock, check.mjs,
  check.test.mjs, crates/net/Cargo.toml, crates/net/src/lib.rs (one mod block at the end), crates/net/src/tcp_kernel.rs
  (new), crates/tcpinfo/** (new: Cargo.toml, src/lib.rs, linux.rs, stub.rs, tests.rs, examples/siocoutq_probe.rs),
  results/u3c-kernel-check/siocoutq-root-check.py (new)}.
- Cargo.lock: +[[package]] npt-tcpinfo 0.0.0 (dependencies libc, socket2); npt-net's dependencies gain npt-tcpinfo.
  No other package added or changed (libc 0.2.191 and socket2 0.6.5 were already locked). 9 insertions.
- Public surface: npt-tcpinfo exports srtt_us and unacked_and_unsent only; npt-net gains no pub item (tcp_kernel is a
  private module with pub(crate) srtt_ns and outstanding_bytes).
- Kernel cites (scratchpad linux-src-v6.8): tcp.c:623-630 (SIOCOUTQ; EINVAL on LISTEN :624-625; 0 in SYN states
  :627-628), :632-640 (SIOCOUTQNSD), :646 (`*karg = answ`), :3842 (tcpi_rtt = srtt_us >> 3), :3854
  (tcpi_notsent_bytes = write_seq - snd_nxt); tcp_ipv4.c:2899 (/proc tx_queue = write_seq - snd_una); tcp_input.c:2208
  (tcp_enter_loss sets high_seq = snd_nxt, never rewinds snd_nxt: the hold's notsent stays 0 after the RTO).
- libc 0.2.191 cites: gnu/mod.rs:248 tcp_info (tcpi_rtt :277, tcpi_notsent_bytes :296); arch/generic/mod.rs:35
  TIOCOUTQ = 0x5411 (mips :35 0x7472, powerpc :32 and sparc :35 0x40047473); no Default on libc structs (macros.rs:171
  s! derives Clone, Copy, Debug only), hence the byte buffer.

DONE
