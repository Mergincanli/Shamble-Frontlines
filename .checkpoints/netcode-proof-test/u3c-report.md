# U3c report: the WebSocket delay layer, fix 1's send gate, the kernel-numbers crate, and the kernel cross-check

DRAFT, written 2026-11-08 about 21:10 UTC while the final small fix round (workflow wf_5c9f078a-da1) runs. Parts marked
**TBD** wait for that round and for the condition-10 diagnostic. For the advisor first, then Mustafa.

## What happened
- U3c built the WebSocket path's delay layer (DelayStream) on the Linux-faithful TCP model from U3a, fix 1's send gate
  (send only when the connection can take a whole snapshot; skip, never queue), a small Linux-only crate that reads the
  kernel's own TCP numbers (npt-tcpinfo, the spike's one unsafe-code exception), tests that prove each mechanism, and a
  cross-check of the model against the real kernel.
- The model matches the kernel: the cross-check passed on its first run, and the root-only send-queue check passed.
- Condition 10's lateness trigger fired (the delay layer's own timing tail in this container). The advisor ruled to
  accept it here, with the bar moved to the real test server (RS) at R1. An A/B on unchanged code showed the tail comes
  from the container, not the code.
- Four reviewers checked the final diff; each finding was verified by a second agent (45 of 47 real). A fix pass, a
  re-review that caught one regression of our own (fixed and pinned), a last round and a final small round closed
  every finding or named it as a known limit.

## What was built (commits on the spike branch)
| Commit | What |
|---|---|
| 2a93038 | Build B: npt-tcpinfo (getsockopt TCP_INFO, ioctl SIOCOUTQ; R20 (a)), check.mjs's unsafe-placement and libc checks, the root-run script |
| c5d7260 | Build A: DelayStream (TLS records, pump task per connection, release clock thread, size limits before buffering), the gate (`ws_gate`: snapshot_budget, may_write, TickWriter), mechanism tests, the kernel-check scripts |
| 292e526 | U3b's loopback P2 test made stable (pace impaired runs at the tick; FT1-FT4) |
| 81203da | The fix pass: the verified review findings |
| b5c9009 | The per-release lateness split (clock wake, pump start, S's write; an identity test that the parts sum to the total) |
| b5e4043 | C2b: the compiler backstop (`unsafe_code = "forbid"` for the workspace; the two exceptions keep their own tables) and check.mjs's scope |
| f01d573 | C1b: regression R1 fixed; the pump's wake and panic gaps; split histograms pooled per process |
| **TBD** | C1c and C2c: the final small round |

## Evidence
| What | Result | Where |
|---|---|---|
| Kernel cross-check (model vs Linux on loopback, 3 runs) | **PASS (final)**. RACK stall p50: kernel 33.344 ms (166 episodes), model 33.458 ms (5,672): within 2 ms. Probe-or-timeout share: kernel 6 of 172, model 246 of 5,918 (p 0.0416), inside the 99 % band 1-15. Noise guard held per run (60 ≤ 61, 54 ≤ 56, 58 ≤ 61 retransmissions). iptables identical before and after | results/u3c-kernel-check/out-20261108T004723Z/verdict.txt (1a9fd73) |
| Root SIOCOUTQ run (the kernel's send-queue count under a held loss) | **PASS** (rules R1-R4 pass; release after 435 ms; R4's bound 1.632 s) | results/u3c-kernel-check/siocoutq-root-check-20261108T005258Z.txt (989118e) |
| Lateness run 1 (quiet, P2, 30 Hz, 335 s, 10,051 samples each way) | down p50 0.112, p99 0.345, p99.9 1.908, max 4.200 ms, E 3.5 µs; up 0.161 / 0.509 / 2.111 / 2.839 ms, E 6.5 µs. **Condition 10's trigger fires** (p99.9 > 1 ms) | results/u3c-lateness/lateness-run1-20261108T0041Z.txt |
| Lateness run 2 (after the fix pass, which carried R1; the VM had restarted) | down 0.155 / 0.628 / 3.483 / 11.281 ms, E 16.4 µs; up 0.206 / 1.398 / 2.906 / 10.641 ms, E 25.9 µs | results/u3c-lateness/lateness-run2-20261108T1509Z-after-fixpass.txt |
| Interleaved A/B, unchanged pre-fix code against the fix pass | No consistent difference: pre E down 18.1 and 11.6 µs, up 28.2 and 26.7 µs; post down 16.2 and 22.1, up 27.9 and 38.2. The same pre-fix code gave E 3.5 / 6.5 µs in run 1: **the container got noisier, the code did not change** | results/u3c-lateness/lateness-ab-{1..4}-*.txt (e317067) |
| Condition-10 diagnostic (the split, interleaved with ImpairSocket's own lateness run) | **TBD** (reported, not judged) | **TBD** |
| check.mjs (fmt, clippy on both builds and wasm32, deps, tests, self, redact) | 9 of 9 at f01d573: 258 tests in the measured build, 272 in the bites build, self 35 of 35, redact 0. **Final: TBD** | the commit messages |
| The gate | PASS (`--no-smoke`: the container's Chromium does not match tools' Playwright) | the commit messages |

Memory per DelayStream connection: 309,583 B, plus 1,350,400 B once per process for the pooled split histograms.

## Reviews and fixes
| Round | Who | Outcome |
|---|---|---|
| Skeptic round on the final diff (wf_8f2da9fc-8fa) | rust-reviewer, netcode-architect, security-auditor, test-engineer; each finding verified by a second agent | 47 findings (6 medium, 22 low, 19 nits); 45 verified real, 2 refuted. Three APPROVE WITH FINDINGS, test-engineer REVISE |
| Fix pass (81203da) | two builders | every verified finding fixed or named as a known limit |
| Re-review of the fix pass (u3c-fix-review-result.txt) | two critics, each verified | gaps listed; **one regression of our own (R1)**: a down record released during the pump's up-loop step could wait up to a tick |
| Last round (b5e4043, f01d573) | two builders, two critics | R1 fixed and pinned by a test that fails when the fix is reverted; the compiler backstop landed; critics: code APPROVE, small test gaps |
| Final small round (wf_5c9f078a-da1) | two builders, two critics | **TBD** |

## Known limits (stated, not fixed)
- check.mjs's dependency step guards against an accidental relaxation of the unsafe-code and determinism lints; review
  is the wall against a deliberate one. Out of its reach: a cargo config's [env] (CLIPPY_ARGS), a build script's
  rustc-env output, a RUSTC_WRAPPER program (left allowed for sccache), a rust-toolchain.toml path override, the
  lexer's known misreads. **TBD: what C2c adds or names.**
- DelayStream sits above TLS, so L2 never loses the TCP handshake or the TLS flights: it cannot measure the modelled
  reconnect (the harness skeptic's H17).
- This container's timing tail (p99.9 up to about 3.5 ms, maxima up to about 35 ms) is not evidence for a
  microsecond-level bar: RS decides at R1.
- The split's histograms are pooled per process and never reset: one measured run per process (carried to U6).
- npt-tcpinfo's srtt_us makes three extra getsockopt calls per call (the socket-kind checks): a cost for U6's per-tick
  gate to weigh.
- On a kernel without MPTCP the MPTCP half of npt-tcpinfo's test skips, visibly (check.mjs counts and prints skips).

## Carried forward
- **U5 (audit):** tokio-rustls =0.26.6; npt-tcpinfo's four unsafe blocks with their SAFETY comments and tests; the
  request and frame parser as a fuzz target (-34).
- **U6 (WsServer):** the list in plan §12.1's U6 row (the shared gate, its inputs, the writer properties re-run, the
  plain-path SIOCOUTQ check, the ws-budget-x2 bite, gate_inputs() once per tick, add_burst's range check, the accept
  cap at about 0.31 MB per connection, the split pool's one run per process, ...).
- **U8 / R1's run plan:** DelayStream's self-test on RS (48 connections at P2 with the room running, at least 10,000
  samples per direction; judged E under about 10 µs and p99 ≤ 1 ms per direction; p99.9 and max reported; a max over
  10 ms goes to the advisor); the netem srtt read (C12 (f)).

## Decisions needed
- **U3C-CLOSE** (the advisor's): close U3c on this evidence? Recommendation: yes, once the final round and the
  diagnostic are in. **TBD.**

## For Mustafa
1. **TBD:** the cleanup ask, once (finished agent copies and build caches; sizes listed).
