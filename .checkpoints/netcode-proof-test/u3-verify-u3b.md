# U3b skeptic (rust-reviewer): ImpairSocket and L2Config at 5a5aba9

Scope: spikes/netcode-proof/crates/net/src/l2.rs and l2/*.rs; build notes u3-build-u3b.md; rules M1-M5, M9-M15, M19
(advisor-u3q.md 230-290) and u3-build-plan.md's U3b bullet (line 29-39). Default verdict "unclear"; refute from
primary sources (quinn 0.11.12 / quinn-udp source in ~/.cargo/registry, crates/l1).

## Plan
1. Read l2.rs, config.rs, impair.rs, shaper.rs, socket.rs, thread.rs, stats.rs, names.rs, wt.rs; then tests.
2. quinn 0.11.12 AsyncUdpSocket trait + its use in endpoint/runtime (poll_recv meta contract, poll_writable,
   try_send semantics, max_transmit_segments/max_receive_segments, may_fragment). Check ImpairSocket.
3. Release thread: heap, try_send error mapping, slack via /proc, Drop, races.
4. Fates vs crates/l1 (ChaCha8 per stream, 16 words, ReorderStep), shaper arithmetic, B1, UDP block.
5. M10 bounds; M1 lateness points and the E[...] computation.
6. Run the tests (and the ignored M13 run), plant defects in a throwaway worktree under .gate/.
7. wt.rs: TLS (M7), accept loops (M8), migration (M9).
8. SERIES.md rows vs names.rs.
9. Build notes' open points.

## Findings (one line each, as found)
