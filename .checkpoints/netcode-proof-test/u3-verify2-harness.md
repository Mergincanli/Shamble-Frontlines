# u3-verify2-harness: second skeptic round, the reconnect harness (fixes.rs run() and policy)

Agent: netcode-architect (skeptic). Branch claude/netcode-proof-test-plan-hcky5g, head cc72f6f (+0dae8b7 checkpoint).
Scope: fixes.rs run() and its policy vs browser behaviour (Chromium, Firefox sources) and ADR 0001; items (1)-(9) of the task.
Rules: no tracked edits, no commits; diagnostics only in a throwaway worktree under .gate/.

## Plan
1. Read first-round notes (u3-verify-fixes-model.md, u3-verify-fixes-adr.md), the fix plan u3a-fix-plan.md, the commit message.
2. Read fixes.rs (run(), policy, tables, ws_limit_tail), tests.rs S13, relevant parts of tcp_model.rs.
3. Item (1): Chromium websocket_endpoint_lock_manager.cc, transport_connect_sub_job.cc, websocket_transport_client_socket_pool;
   Firefox WebSocketChannel.cpp (FailDelayManager, nsWSAdmissionManager); what ws.close() on CONNECTING frees; timeouts.
4. Item (2)-(5): harness policy vs ADR 0001 text (tick, Join, FullState, "decodes deltas again").
5. Item (6): flights sizes; Cargo.toml TLS stack (rustls? aws-lc-rs? ring?) and its default key share.
6. Item (7): side-effects table; cost of a false takeover vs FullState.
7. Item (8): ADR conflicts compared with round 1's list; token claims "bound to the session".
8. Item (9): anything biasing the numbers; run the pinned test in a worktree; maybe a probe or two.
9. Verdicts, bugs, must-correct; DONE.

## Findings (one line each, appended as found)
