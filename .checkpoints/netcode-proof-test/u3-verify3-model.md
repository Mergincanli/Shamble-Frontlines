# u3-verify3-model: third skeptic round on TCP model v3 (rust-reviewer)

Branch claude/netcode-proof-test-plan-hcky5g, head 1e020a7 (+ checkpoint commits). Read-only on tracked files.

## Plan
1. Read prior notes (u3-verify-fixes-model, u3-verify2-model, u3-build-v3-model) and the merge message of 1e020a7.
2. Read tcp_model.rs in blocks; for each topic (1)-(6) open the matching Linux v6.8 function and compare line by line.
   Record a line per finding here at once.
3. Run the model's unit tests with my own target dir (.gate/rr3-target).
4. Worktree .gate/rr3-wt at HEAD: instrument a median recommended phase (P2, adaptive budget + one-endpoint
   reconnect) and the S13(e) seeds 1017 / 1475; trace each step, mark kernel / not kernel.
5. Check each new rule test + its bite test (tests.rs) tests what its name says.
6. Section (9): residual biases, would p99 1,051 / max 4,196 move.
7. Verdicts, bugs, must-correct. End with DONE.

## Findings (one line each, as found)
