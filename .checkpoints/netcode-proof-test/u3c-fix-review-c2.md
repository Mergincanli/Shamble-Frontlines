# U3c fix-pass re-review, part C2 (security-auditor): check.mjs and npt-tcpinfo

Scope: `git diff 95597ef..81203da -- spikes/`, the C2 half only (check.mjs, check.test.mjs, Cargo.toml lint comment,
crates/tcpinfo/src/{lib,linux,tests}.rs, results/u3c-kernel-check/siocoutq-root-check.py).
Findings assigned: S1, F-D, F-E, F-N, F8, F-J, S5 (part c).
Inputs read: u3c-review-result.txt (S1 :399, S5 :431, F-D :527, F-N :599, F-E :607, F-J :639, F8 :76 and verdicts),
u3c-fix-c2.md (fix notes), u3c-plan.md rev 2, advisor-u3c-plan.md.
Rules: no source edits; bypass attempts in a scratch copy only (scratchpad), CARGO_TARGET_DIR=.gate/u3c-review-target;
no iptables, no #[ignore]d evidence tests. Memory: none stored (no .claude/agent-memory for this agent).

## Plan
1. Read the C2 diff (check.mjs, check.test.mjs, tcpinfo, root script, Cargo.toml).
2. Per finding: fixed / partly / not, file:line, the test that fails when the fix is reverted (revert in a scratch copy).
3. Bypass attempts against the new rules in a scratch copy of spikes/netcode-proof (outside the main tree).
4. False positives: run check.mjs (deps, self) on the real tree; judge refusals of legitimate setups (RUSTC_WRAPPER,
   CARGO_HOME config, sccache, CI env).
5. F8: each of the 4 unsafe blocks: SAFETY invariant, lengths, placement.
6. Regressions.

## Progress
