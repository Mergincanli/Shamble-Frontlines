# U3d plan rev 1: rust-reviewer's adversarial review (notes)

Started 2026-11-09 on branch claude/netcode-proof-test-plan-hcky5g, HEAD f6041cb (95dac9c + checkpoint-only commits;
`git diff --stat 95dac9c HEAD` touches .checkpoints only). Read-only: no code edits, builds, test runs or network.

## Plan of this review
1. Read u3d-plan.md (rev 1), u3d-plan-notes.md, u3d-scout-req.md, u3d-scout-r1.md.
2. Check the code claims at file:line: socket.rs (poll_recv, release path, fate log, header_numbers/texts,
   release_thread_id), thread.rs (slack/policy reads), stats.rs (fate log sizes, E), impair.rs, config.rs,
   socket_tests.rs (M13 run, pace, poller), delay_tests.rs (runtime, loadavg), Cargo.toml (tokio features, quinn
   dev-dep?), quinn/quinn-udp sources in ~/.cargo registry (BATCH_SIZE, drive_recv).
3. Focus: placement and shape (test file only? new pub symbols?), no unsafe/libc in npt-net, measurement points not
   perturbing (no lock/alloc on release path; fate log first-touch under lock), slack/steal reads safe, identical
   behaviour on targets that build it (cfg gates, Linux-only /proc), no engine feature beyond need.
4. Findings with severity, where, claim, evidence, fix. Verdict.

## Findings (one line each, as found)
