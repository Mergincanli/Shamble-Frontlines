# u3-verify-fixes-model: skeptic of the WS fallback's three fixes (rust-reviewer)

Branch claude/netcode-proof-test-plan-hcky5g, head 689c81f (41bd8a0 + checkpoint). No memory found (no agent-memory dir).

## Plan
1. Read Mustafa's request (.checkpoints/netcode-proof-test.md, Decisions taken 2026-11-06 ~21:40) and advisor-u3q.md (1) S0-S14.
2. Read tcp_model.rs, fixes.rs, tests.rs.
3. Check each item against Linux v6.8 sources: (1) thin linear timeouts; (2) rto_min plumbing; (3) ssthresh/tcp_enter_loss;
   (4) handshake + reconnect harness; (5) adaptive budget + scaled trigger; (6) both explanations (instrument a worktree copy);
   (7) anything biasing numbers; would 611/738 move.
4. Run the pinned tests; worktree copy under .gate/ for diagnostics; remove when done.
5. Verdicts, bugs, must-correct.

## Findings (a line per finding)
