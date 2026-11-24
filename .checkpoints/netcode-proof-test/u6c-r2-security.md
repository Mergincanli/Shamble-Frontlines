# U6c round 2, security seat (security-auditor, xhigh). Read only. 2026-11-24.
Inputs, sha256 checked equal (all six): advisor-u6c-r1.md ad72c0b8…, u6c-review-netcode.md f29e1ac1…,
u6c-review-test.md 79197b42…, u6c-review-security.md c29540fa…, u6c-build.md a9d45dd4…, wip/u6c-fix.patch 0549389e….
Diff: `git diff aad2103… 6c3ff00…` narrowed to crates/host/src/serve.rs (+8/-2) and crates/server/src/probe.rs
(+29/-?). Paths below are under W = .claude/worktrees/agent-a3a3e3fdf559de095/spikes/netcode-proof.

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
