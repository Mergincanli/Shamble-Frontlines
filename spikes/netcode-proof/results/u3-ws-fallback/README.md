# The WebSocket fallback's recovery, measured on the Linux 6.8 TCP model (U3a, 2026-11-07)

Outputs kept so the numbers behind PM-0001-18's WebSocket bar and the reconnect requirement (ADR 0001's
OQ-0001-19 and `docs/design/ws-reconnect.md`, from the docs PR) can be read and reproduced after the change-set's
checkpoints are gone. The model is `spikes/netcode-proof/crates/net/src/tcp_model.rs` ("linux-6.8-tcp v3", checked
against Linux v6.8 by three skeptic rounds); the harness is `tcp_model/fixes.rs`. Each run is a B1 phase at P2 (100 ms,
±20 ms, 2 % loss) with one 1 s burst of total loss; recovery is the time from the link's return to the client's first
fresh delta.

| File | What it is | Made at | Reproduce (from `spikes/netcode-proof`) |
|---|---|---|---|
| `tables-1e020a7.txt` | Every table: the 60,000-phase rows (today, fix 1, fix 3, the reconnect policies, browsers, TLS, timestamps, Windows-like clients), multi-burst lineages, side effects, and the 600,000-phase tail with bars and pass chances | 1e020a7 | `cargo +1.99.1 test --release -p npt-net --lib fixes -- --ignored --nocapture --test-threads 1` |
| `bar-fix1-bf9d248.txt` | The model's bound for a correct fix-1 build (`ws_bar_fix1`), with the broken builds and model-error rows | bf9d248 | `cargo +1.99.1 test --release -p npt-net --lib fixes::ws_bar_fix1 -- --ignored --nocapture` |
| `reconnect-policies-1e020a7.patch` | The third skeptic round's diagnostic patch to `fixes.rs` (applies to 1e020a7): the scaled abort at max(500 ms, 4 x RTT), join-or-close, and other outage lengths (0.6, 1.5, 2.5 s), behind the reconnect record's policy tables. Throwaway code, kept for reproduction, not built into the spike | 1e020a7 | `git apply` it in a worktree at 1e020a7, then run its tests as the patch names them |

Key rows (600,000 phases): today p50 1,121, p99 2,686, max 12,068 ms; fix 1 alone (the newest-snapshot-only budget)
654 / 2,164 / max 11,384; fix 1 plus the browser-runnable reconnect 654 / 1,051 / max 4,196 (the reconnect
requirement's target). The model's bound for a correct fix-1 build at 480 client-bursts (single bursts): "half within
0.67 s, at most 5 over 2.5 s, none over 6.9 s" (correct 0.968; today and a budget twice too large 0.000).
