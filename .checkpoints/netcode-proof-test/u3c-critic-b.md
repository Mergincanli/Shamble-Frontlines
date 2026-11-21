# U3c build B completeness critic (test-engineer), 2026-11-07

Worktree: /home/user/Shamble-Frontlines/.claude/worktrees/wf_87550c07-bbd-2 (uncommitted on ebe82b2).
Plan: u3c-plan.md rev 2, scope 1.8 [C12] (a)-(g) + check.mjs deps checks. Read-only on the worktree.

## Plan
1. git diff + new files: Cargo.toml, Cargo.lock, net/Cargo.toml, net/lib.rs, tcp_kernel.rs, tcpinfo/*, check.mjs,
   check.test.mjs, root script.
2. Per item (a)-(g), (d) deps checks: met / missing / wrong / unproven with file:line.
3. Run check.mjs with CARGO_TARGET_DIR=.gate/u3v3-model-target; record result.
4. Adversarial: tests that cannot fail, wall-clock thresholds, public items beyond rev 2, grep bypasses.

## Findings (as I go)
- check.mjs reproduced (CARGO_TARGET_DIR=.gate/u3v3-model-target): "check: all 9 steps pass", exit 0; test 201/214,
  deps "... npt-client-wasm and npt-tcpinfo (68 crate files read)", self 15/15, redact 0 findings. Matches the claim.
- (a) met: tcpinfo/Cargo.toml:11-12 [lints] workspace; linux.rs:27-31,62-66 allow on the two fns; SAFETY :37-40,
  :69-72; lib.rs:26-29 deny the two clippy lints. One unsafe op per block (:41-49, :73).
- (b) met + tested: linux.rs:83-97 rtt_from; tests.rs:173-198 (68 pinned, at+3 Unsupported, at+4 Some, 0 None).
- (c) met in code; lib-only clippy wasm32 re-run by me: exit 0. --all-targets on wasm32 fails (socket2 dev-dep), not
  required. npt-net tcp_kernel.rs:39-42 (not(unix) handle) never compiled anywhere: unproven. check.mjs clippy-wasm32
  is LEAF only (check.mjs:21,274): the stub's off-Linux build is not re-checked by check.mjs.
- (d) libc pin met (Cargo.toml [workspace.dependencies]). deps checks met with tests (check.test.mjs +6).
  BYPASS (scratch bypass.mjs, real tree + mutation): `#[allow(unsafe_code)] mod linux;` in lib.rs with one fn allow
  removed -> MISSED (count stays 2, not `#!`). Allow moved to a helper -> MISSED. `#[path]`/include! of tcpinfo's
  linux.rs from crates/room -> MISSED (grep by folder). check.mjs:143-147 docstring claims "must sit on its two
  functions only": not enforced, count only (check.mjs:170-175).
  §4.2 rev 3.3 (plan.md:432-434) says npt-net stays free of libc: not checked, yet the deps line says "the dependency
  rules of plan §4.2 hold".
- (e) tests: request const tests.rs:103-114; EINVAL :116-125; srtt >0 :127-157; non-reading peer :214-258;
  delayed-ACK sent-unacked :265-310 (beyond plan; bites: my Python nsd.py: SIOCOUTQ 200 vs SIOCOUTQNSD 0, 50/50, so a
  NSD mutant fails assert_eq(q1, tx)).
- ROOT RUN DEFECT (probe :49 WRITES=20, :57 DELIVERY_WAIT 100 ms, :423-426, :450-455): measured cwnd after the probe's
  warm-up (cwnd.py, offset 80): 10 in 9 of 10 runs, 16 once (ssthresh hit). With every ACK dropped, tcp_write_xmit's
  cwnd test lets at most cwnd (+1 TLP new-data probe) of the 20 segments out; writes 11/12..20 stay unsent, so the
  peer cannot read them (R1 "delivered < written") and tcpi_notsent_bytes > 200; each such sample waits the full
  100 ms, so the last write lands ~950 ms after GO > rto 204 ms -> VOID (:450). A rerun repeats it. The dry run cannot
  show this (ACKs flow). Fix: WRITES <= 8, or read tcpi_snd_cwnd at GO and VOID/limit; log snd_cwnd per sample.
- (f) missing: "R1's run plan reads srtt against netem's 100 ms; no tc here" is in no notes, carry or plan text
  (grep plan.md, checkpoint, u3c-build-b.md).
- (g) met: Cargo.toml lint comment (B); plan text §4.1/4.2/4.7/4.8 in b90fb7e (main session), base ebe82b2 is its
  ancestor.
- Stale refs: tests.rs:4-5 and siocoutq_probe.rs:4 name `siocoutq-root-check.sh`; the file is .py.
- Soundness note: unacked_and_unsent is a safe pub fn on any BorrowedFd; SAFETY (linux.rs:69-72) covers TCP sockets
  and ttys only; a device fd whose driver gives 0x5411 another meaning is outside the invariant (doc lib.rs:19-21 is a
  precondition a safe fn cannot impose). For rust-reviewer/security-auditor.
- Kernel cites for the root-run defect (scratchpad linux-src-v6.8): tcp_output.c:2742-2749 (cwnd_quota 0 -> break,
  except push_one == 2, the TLP's one forced packet), :2073-2086 tcp_cwnd_test (in_flight >= cwnd -> 0);
  include/net/tcp.h:241 TCP_INIT_CWND 10, :1253-1256 tcp_packets_in_flight. No ACK -> no RACK mark, so in_flight
  stays packets_out until the RTO; after it tcp_enter_loss leaves cwnd 1 with in_flight 1: no new data either.
- Reproduced: tcpinfo tests 3 x (7 passed, 2.2 s each); probe zero-window SELFCHECK PASS (2,803,712 x5).
- tcp_kernel.rs:93-95 and :115-117: conditional asserts with no count of how many ran (tcpinfo's tests count them).
- R4 (root-check.py:34; probe :443, :494-498) is a 5 s wall-clock pass criterion; margin wide (next RTO retransmit
  ~1.43 s after GO), low.
- Cargo.lock: npt-tcpinfo lists socket2 (dev) beside libc; plan §5 wording says libc only; stated in the report.
- Worktree unchanged by my runs (git status identical before and after).

## Verdict
gaps: one blocking-for-the-evidence-run defect (root run cannot pass: cwnd 10 vs 20 writes), (f) missing, the
placement claim of the unsafe check not enforced (module-level allow, #[path] bypass), smaller items above.
DONE
