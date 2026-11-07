# Advisor notes: U3c plan review (u3c-plan.md)

Started 2026-11-07. Branch claude/netcode-proof-test-plan-hcky5g, head 2ec1533; code as of 7321cc0.

## Plan
1. Read u3c-plan.md (done), then sources: advisor-ws-docs-r2.md:370-420, verify-ws-docs-r2.md:47,
   advisor-u3q.md (M14, M16, M17, S0-S14), u3-build-plan.md, plan.md §4.1-4.8, §12.1, catalogue :61-78.
2. Code: tcp_model.rs :2058-2205, fixes.rs :163-169 / :770-787, l2/socket.rs :646-737, l2/config.rs, link/src/lib.rs,
   Cargo.toml :56-58, Cargo.lock (libc, rustls, socket2).
3. Facts: crate sources in scratchpad (socket2/rustix/nix), tokio-rustls 0.26.6 deps.
4. Rule on 1-6.

## Findings (as I go)
