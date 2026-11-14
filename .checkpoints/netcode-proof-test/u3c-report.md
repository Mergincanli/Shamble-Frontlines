# U3c report: the WebSocket delay layer, fix 1's send gate, the kernel-numbers crate, and the kernel cross-check

Written 2026-11-08 (UTC); **U3c CLOSED on the closing head 768d00ba7a77529b524fc9f6df90d6494f87abcc**, the advisor's
U3C-CLOSE ruling's five conditions met (advisor-u3c-plan.md, "Side ruling: the condition-10 diagnostic, UP-STAMP and
U3c's close"): C2d's critic complete with no regression; check.mjs 9/9 and the gate re-run on that head (below); the
report fixes and the plan's U6 and U8 rows (941832e); the checkpoint pushed with the close; the critics' scratch copies
in Mustafa's cleanup ask. For the advisor first, then Mustafa.

## What happened
- U3c built the WebSocket path's delay layer (DelayStream) on the Linux-faithful TCP model from U3a, fix 1's send gate
  (send only when the connection can take a whole snapshot; skip, never queue), a small Linux-only crate that reads the
  kernel's own TCP numbers (npt-tcpinfo, the spike's one unsafe-code exception), tests that prove each mechanism, and a
  cross-check of the model against the real kernel.
- The model matches the kernel: the cross-check passed on its first and only run, and the root-only send-queue check
  passed.
- Condition 10's lateness trigger fired: the delay layer's own timing tail in this container (how late a record leaves
  against the model's delivery time). An A/B showed that the rise after the first run came with no code change. The
  per-release split then showed that **no single part holds the tail**: the release thread's own OS wake carries a tail
  of the same size as the whole, and up's p99 sits in up's work part (the pump's pass plus the reader's turn), to be
  split further at U6. The advisor ruled: no rebuild of the pumps off tokio before R1; the real test server (RS) decides
  at R1 with the bar already set, and a fallback order if it misses.
- Four reviewers checked the final diff; each finding was verified by a second agent (45 of 47 real). A fix pass, a
  re-review that caught one regression of our own (fixed and pinned), a last round, a final small round and a last tiny
  round closed every finding or named it as a known limit.

## What was built (commits on the spike branch)
| Commit | What |
|---|---|
| 2a93038 | Build B: npt-tcpinfo (getsockopt TCP_INFO, ioctl SIOCOUTQ; R20 (a)), check.mjs's unsafe-placement and libc checks, the root-run script |
| c5d7260 | Build A: DelayStream (TLS records, pump task per connection, release clock thread, size limits before buffering), the gate (`ws_gate`: snapshot_budget, may_write, TickWriter), mechanism tests, the kernel-check scripts |
| 292e526 | U3b's loopback P2 test made stable (pace impaired runs at the tick; FT1-FT4) |
| 81203da | The fix pass: the verified review findings |
| b5c9009 | The per-release lateness split (clock wake, pump start, the release's stamp; an identity test that the parts sum to the total) |
| b5e4043 | C2b: the compiler backstop (`unsafe_code = "forbid"` for the workspace; the two exceptions keep their own tables) and check.mjs's scope |
| f01d573 | C1b: regression R1 fixed; the pump's wake and panic gaps; split histograms pooled per process |
| f6cd1d7 | C1c and C2c, the final small round: the pump's last test gaps (the no-spin bound, add_burst's wake, the panic condition both ways), shutdown's error after a failed pump; check.mjs's reader gaps (lint sub-tables and name forms, the scrubbed git environment, RUSTC_BOOTSTRAP and manifest rustflags refused, the CARGO_HOME and stderr wiring tests, the BPF SAFETY line) |
| ecdeeb0 | The condition-10 diagnostic's output (evidence only) |
| 768d00b | C2d, the last tiny round (the closing head): a crate's lint inheritance read through the TOML mask (a `[lints]` hidden in a multi-line string no longer counts); a quoted `[["include"]]` config header refused (found in the builder's reader sweep: cargo 1.99.1 loads it); a member manifest's rustflags plant in the fixture; cargo's channel override refused beside RUSTC_BOOTSTRAP; a leading byte order mark read past; unacked_and_unsent's SAFETY comment for non-inet families that answer 0x5411 |

## Evidence
| What | Result | Where |
|---|---|---|
| Kernel cross-check (model vs Linux on loopback, 3 runs) | **PASS (final)**. RACK stall p50: kernel 33.344 ms (166 episodes), model 33.458 ms (5,672): within 2 ms. Probe-or-timeout share: kernel 6 of 172, model 246 of 5,918 (p 0.0416), inside the 99 % band 1-15. Noise guard held per run (60 ≤ 61, 54 ≤ 56, 58 ≤ 61 retransmissions). iptables identical before and after | results/u3c-kernel-check/out-20261108T004723Z/verdict.txt (1a9fd73) |
| Root SIOCOUTQ run (the kernel's send-queue count under a held loss) | **PASS** (rules R1-R4 pass; release after 435 ms; R4's bound 1.632 s) | results/u3c-kernel-check/siocoutq-root-check-20261108T005258Z.txt (989118e) |
| Lateness run 1 (quiet, P2, 30 Hz, 335 s, 10,051 samples each way) | down p50 0.112, p99 0.345, p99.9 1.908, max 4.200 ms, E 3.5 µs; up 0.161 / 0.509 / 2.111 / 2.839 ms, E 6.5 µs. **Condition 10's trigger fires** (p99.9 > 1 ms) | results/u3c-lateness/lateness-run1-20261108T0041Z.txt |
| Lateness run 2 (after the fix pass, which carried R1; the VM had restarted) | down 0.155 / 0.628 / 3.483 / 11.281 ms, E 16.4 µs; up 0.206 / 1.398 / 2.906 / 10.641 ms, E 25.9 µs | results/u3c-lateness/lateness-run2-20261108T1509Z-after-fixpass.txt |
| Interleaved A/B, unchanged pre-fix code against the fix pass | No consistent difference: pre E down 18.1 and 11.6 µs, up 28.2 and 26.7 µs; post down 16.2 and 22.1, up 27.9 and 38.2. The same pre-fix code gave E 3.5 / 6.5 µs in run 1: the rise after run 1 came with no code change | results/u3c-lateness/lateness-ab-{1..4}-*.txt (e317067) |
| Condition-10 diagnostic: each release split into its parts, interleaved with ImpairSocket's own lateness run, twice; reported, not judged | The parts add up on every release (0 mismatches). **No single part holds the tail**: in each of the 20 largest releases listed, one part is the largest, usually by far (work in 9, clock in 7, hop in 4). Per part, p99 / p99.9 / max (runs 1 and 2): clock (the release thread's own OS wake) down 0.16-0.26 / 1.6-2.3 / 3.1-3.6 ms, up 0.20-0.23 / 2.1-2.3 / 3.2-6.8 ms; hop (tokio's wake of the pump and its lock wait) down 0.24-0.30 / 1.5-2.0 / 2.9-3.2 ms, up 0.27-0.32 / 1.9-2.0 / 2.7-3.8 ms; work down (the pump's work and S's TLS write) 0.12-0.15 / 1.05-1.41 / 3.1-6.5 ms, up (the pump's pass after its opening step plus the reader's turn: the split cannot yet tell them apart) 0.83-1.01 / 2.5-2.6 / 8.4-22.2 ms, the largest part's E in up (16.6-18.6 µs; the parts' E do not add). Totals: down E 11.5 / 16.6 µs, p99 0.62 / 0.95 ms; up E 26.8 / 35.4 µs, p99 1.44 / 1.77 ms. ImpairSocket as a scale check (its whole path, send to receive, at 10,000 per second each way): p99.9 1.25-4.24 ms, max 3.1-8.2 ms. **The advisor's verdicts:** C10-B APPROVE (the pumps are not moved off tokio before R1); UP-STAMP APPROVE with changes (a stamp at the reader's wake, built in U6); U3C-CLOSE APPROVE on five conditions | results/u3c-lateness/c10-diagnostic-20261108T2152Z.txt (ecdeeb0) |
| check.mjs on the closing head 768d00b (clean tree, about 22:47 UTC) | **all 9 steps pass**: fmt PASS 0.3 s; clippy PASS 2.8 s (measured and bites builds linted); clippy-wasm32 PASS 0.4 s (both builds); test PASS 75.9 s (measured build 260 passed, 0 failed, 0 skips; bites build 274 passed, 0 failed, 0 skips); deps PASS 0.3 s (78 crate files and 1 cargo config read; the dependency rules, the forbid backstop and both exception tables, no cargo-features or rustflags key, no RUSTC_BOOTSTRAP or channel override); refs PASS (472 tracked files, none outside spikes/ references it); marks PASS (81 marks; drafts at 626df67; rows unchanged); self PASS (43 of 43); redact PASS (10 of 10 tests; 149 items, 0 findings) | this run (the same counts in the C2d critic's two runs in its copy) |
| The gate on the closing head 768d00b | **GATE PASS** with `--no-smoke`: checks 1-4 and 5b-5d pass (symbols +0 -0); 5a, the browser smoke run, is skipped because the container's Chromium does not match tools' Playwright, and CI's smoke job runs it | this run |

Memory per DelayStream connection: 309,583 B, plus 1,350,400 B once per process for the pooled split histograms.

## Reviews and fixes
| Round | Who | Outcome |
|---|---|---|
| Skeptic round on the final diff (wf_8f2da9fc-8fa) | rust-reviewer, netcode-architect, security-auditor, test-engineer; each finding verified by a second agent | 47 findings (6 medium, 22 low, 19 nits); 45 verified real, 2 refuted. Three APPROVE WITH FINDINGS, test-engineer REVISE |
| Fix pass (81203da) | two builders | every verified finding fixed or named as a known limit |
| Re-review of the fix pass (u3c-fix-review-result.txt) | two critics, each verified | gaps listed; **one regression of our own (R1)**: a down record released during the pump's up-loop step could wait up to a tick |
| Last round (b5e4043, f01d573) | two builders, two critics | R1 fixed and pinned by a test that fails when the fix is reverted; the compiler backstop landed; critics: code APPROVE, small test gaps |
| Final small round (wf_5c9f078a-da1; f6cd1d7) | two builders, two critics | C1c: COMPLETE (APPROVE), each of its 4 bites caught (the spin bite: 158,276 pump runs against a bound of 2,000; fixed code 112-141 even under 16 busy loops). C2c: no regression, 15 of 15 revert bites caught; five small gaps left (one low: a crate's `[lints] workspace = true` hidden in a multi-line string; four nits) |
| Last tiny round C2d (wf_b13b9984-6bc; 768d00b) | one builder, one critic | The critic: **complete, no gap, no regression**. 10 of 12 revert bites caught; the 2 missed are design details that open no way round (cargo itself refuses those forms), named below. The builder's sweep found and fixed one more gap (a quoted `[["include"]]` config header, which cargo loads); the critic's bite catches its revert |

## Known limits (stated, not fixed)
- check.mjs's dependency step guards against an accidental relaxation of the unsafe-code and determinism lints; review
  is the wall against a deliberate one. Out of its reach: a cargo config's [env] (CLIPPY_ARGS, or any variable for
  rustc), a build script's rustc-env output, a RUSTC_WRAPPER program (left allowed for sccache), a rust-toolchain.toml
  path override or nightly channel, the lexer's known misreads; a future wasm32 test runner would be refused by the
  runner refusal. From C2d's critic: the environment refusal covers RUSTC_BOOTSTRAP and cargo's channel override only
  (any other way to unlock unstable features on the pinned stable toolchain, a patched cargo or rustc included, is
  out of reach); two design details have no test of their own (the member manifests' unsafe_code name scan reads the
  comment-stripped text, not the mask; tomlHeaderHeads refuses a single-bracket `[include]`), and neither opens a way
  round, since cargo refuses those forms itself; three readers fail closed by design (a crate's lint inheritance read
  only in its bare spelling; an array line that only looks like an `[include]` header refused). N5's SAFETY line rests
  on AF_UNIX's and AF_PACKET's v6.8 sources only; the other families are assumed to follow the ioctl's `int *`
  convention, and the comment says so.
- DelayStream sits above TLS, so L2 never loses the TCP handshake or the TLS flights: it cannot measure the modelled
  reconnect (the harness skeptic's H17).
- This container is not evidence for a microsecond-level bar: lateness p99.9 up to about 3.7 ms and maxima up to about
  35 ms. **Since the VM restart, up's p99 here is 1.37-1.88 ms, over R1's p99 ≤ 1 ms bar** (run 2, the A/B and the
  diagnostic). RS decides at R1.
- ImpairSocket's own up E in this container (11.6 / 16.9 µs in the diagnostic) is over U3-Q1 (c)'s "about 10 µs" too:
  U3d reports it; RS judges it.
- The split's histograms are pooled per process and never reset: one measured run per process (carried to U6).
- npt-tcpinfo's srtt_us makes three extra getsockopt calls per call (the socket-kind checks): a cost for U6's per-tick
  gate to weigh.
- On a kernel without MPTCP the MPTCP half of npt-tcpinfo's test skips, visibly (check.mjs counts and prints skips).

## Carried forward
- **U5 (audit):** tokio-rustls =0.26.6; npt-tcpinfo's four unsafe blocks with their SAFETY comments and tests; the
  request and frame parser as a fuzz target (-34).
- **U6 (WsServer):** plan §12.1's U6 row: the shared gate and its inputs, the writer properties re-run, the plain-path
  SIOCOUTQ check, the ws-budget-x2 bite, gate_inputs() once per tick, add_burst's range check, the accept cap at about
  0.31 MB per connection, the split pool's one run per process, a shutdown error after a peer's unclean close read as
  a normal end, and **the reader stamp** (the advisor's UP-STAMP ruling: a stamp at the reader's wake splitting up's
  work into the pump's pass and the reader's turn, with an early-reader class, an identity test and a bite; U6's
  loopback self-test of WsServer reports the four parts).
- **U8 / R1's run plan:** DelayStream's self-test on RS (48 connections at P2 with the room running, at least 10,000
  samples per direction; judged E under about 10 µs and p99 ≤ 1 ms per direction; p99.9 and max reported; a max over
  10 ms goes to the advisor). The split built in U3c plus U6's reader stamp is reported, with steal read before and
  after each self-test run. A miss follows the advisor's fallback order: the split to the advisor first; a miss in
  up's pass part, first measure waking the reader right after the releasing step and yielding; moving the pumps off
  tokio only for a miss in the hop. Also the netem srtt read (C12 (f)).

## Decisions needed
- None for Mustafa. U3c's close was the advisor's (U3C-CLOSE: APPROVE on five conditions, all met).

## For Mustafa
1. The cleanup ask, once (his standing rule after each unit): the finished agent copies, the critics' scratch copies
   and the old build caches, about 15.5 GB in all (the list and sizes are in the checkpoint's Running work), by the
   method he approved: caches with `cargo clean --target-dir`, each dirty copy's leftovers committed to its own local
   branch first, then a plain `git worktree remove`.
