# U3c build A: completeness critic (test-engineer)

Started 2026-11-07. Worktree /home/user/Shamble-Frontlines/.claude/worktrees/wf_87550c07-bbd-1, base ebe82b2,
uncommitted. Plan: u3c-plan.md rev 2, scope 1.1-1.5, 1.7, 1.11, 1.6's script; C1-C11, C15, C16.

## Plan
1. Read diff + new files (ws_gate, delay*, ws_frame, ws_config, tls_test, tcp_model changes, fixes.rs, SERIES.md, results script).
2. Item by item: met / missing / wrong / unproven with file:line.
3. Run check.mjs with CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/u3v3-harness-check-target.
4. Bite checks: do the bites actually bite (try reasoning + maybe a scratch mutation in a COPY, never the worktree).
5. Verdict.

## Findings (as I go)
- (resume 2026-11-07 late) Prior session left only the plan block. Re-read plan rev 2, advisor-u3c-plan.md, build-a notes.
  Build's own open points: stall threshold half-gap (not plan's one gap); per-return row pub(crate); wrap bounds
  ws_max_message 1..=16,370; C10 trigger fired in 2/3 release runs; tests use sync tungstenite over a Bridge, not
  tokio-tungstenite (plan 1.5 says tokio-tungstenite).
- CHECK RUN 1 (23:35, machine up 3 min, load 0.3, nothing else running): `check: 1 step(s) failed`. test step FAIL:
  measured build npt-net lib "129 passed; 1 failed; 10 ignored", failing test
  l2::delay::tests::an_added_burst_loses_every_transmission_inside_it_both_ways (delay_tests.rs:857-889). check.mjs
  truncates the panic text. Other 8 steps PASS. Log: scratchpad check-run1.log. => the build's "all 9 steps pass" did
  NOT reproduce. The test passes alone 6/6 ([9, 10] inside). Full lib suite run once more: 130 passed. => a flake under
  parallel load: a defect to root-cause (agent rule), not retry into green. Looping the lib suite to catch the message.
- ws_gate.rs: snapshot_budget/may_write match fixes.rs:775-776 (outstanding + SNAP <= N x SNAP, owed 0, free >= SNAP).
  fixes.rs's room step still inlines the three conditions instead of calling may_write (only adaptive_limit goes
  through ws_gate): drift risk, rev 2 1.4 (e) only names adaptive_limit. Minor.
- ws_gate_tests (b): "writes happen only on a tick" is structural (only on_tick writes); "outstanding <= N x size after a
  write" is implied by the iff check with the same N (tautological once iff holds). Queueing bite + budget x2 bite bite
  (asserted error text). (a) table pinned with literals [6,3,4,6,8,15].
- tcp_model.rs: add_burst refuses start < now and len 0, prunes ended, MAX_BURSTS 64. bursts() doc says "not yet ended"
  but list is only pruned on add (doc drift, minor). with_loss_from cfg(test) zeroes the drawn loss only.
- delay.rs wrap converts profile bursts from DelayStream's clock to model axis (delay.rs:554-568). OK.
