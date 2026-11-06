# u3-verify-fixes-adr: skeptic of the ADR 0001 conflict analysis for the WebSocket fallback's three fixes

Branch claude/netcode-proof-test-plan-hcky5g, head 689c81f (41bd8a0 + checkpoint). No tracked file edited.

## Plan
1. Read Mustafa's request (checkpoint "Decisions taken", 2026-11-06 ~21:40) and the decisions the main session sent.
2. Read ADR 0001 parts 10 (admission, tokens, Q22), 11/12 (transport, line 422), 13 (key, 64 KB), line 424, 450, 507;
   ADR 0006 (kernel settings, socket options).
3. Read plan.md §10.3 (join tokens, roster).
4. Read fixes.rs for what fix 1/2/3 actually do (reconnect period, attempts in parallel, buffer sizes).
5. Verdict per claim (1)-(5); conflicts missed; exact lines a docs PR changes for WS-FIX (a) + RESUME-CRED (a).
6. DONE.

## Findings (a line each, as found)
