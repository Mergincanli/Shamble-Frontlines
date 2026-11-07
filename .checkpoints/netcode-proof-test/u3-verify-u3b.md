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
- (1) quinn contract CONFIRMED: runtime.rs:42-89 (trait); endpoint.rs:794-850 loops poll_recv until Pending (Ok(0)
  would spin: msgs=0 then loop), so socket.rs:470-477 (Pending when n==0, waker stored) is right; connection.rs:1020-1061
  max_datagrams = max_transmit_segments().min(..) = 1, WouldBlock -> buffered + retry; ImpairSocket never WouldBlocks
  (socket.rs:430 Ok always; AlwaysWritable socket.rs:391-394). Multi-segment split socket.rs:407-413 chunks(seg) as
  quinn-udp's own apple path does (unix.rs:390-397). quinn recv_buf chunk = min(max_udp_payload,64K)*1*32/32 = 65,527 B
  (endpoint.rs:754-758,785) so the Oversize branch at socket.rs:452 never fires in practice.
- (1) may_fragment = UdpSocketState's (quinn-udp unix.rs:125-150, DF via IP_MTU_DISCOVER); allow_mtud = !may_fragment
  (quinn endpoint.rs:140). Nonblocking set by UdpSocketState::new (unix.rs:109), so tokio::UdpSocket::from_std and the
  release thread's try_send never block.
- (2) try_send = raw send() (quinn-udp unix.rs:223-225); send() swallows all but WouldBlock (unix.rs:207-219):
  M5 CONFIRMED. send_fate socket.rs:571-579 maps WouldBlock->Kind::WouldBlock, else SendError with errno.
- (3) Reorder step vs L1 (l1/src/lib.rs:617-645): same condition (now < held.due && succ_due + 1 > held.due), same
  release-on-successor, hold drawn only for delivered, word places 0/1/2 (lib.rs:61-64 vs impair.rs:34-37), same
  below() and delay() formula (lib.rs:418-440 vs impair.rs:107-111). L1's poll-time release (lib.rs:716-723) ==
  L2's token (impair.rs:559-584). CONFIRMED equivalent.
- (3) Shaper arithmetic CONFIRMED (units bit-ns/s consistent; shaper.rs:55-63); -23 5,920 (config.rs:117-119), -24
  max(R/40, 5,920) (config.rs:122-124); before the delay (impair.rs:479-489). Doc nit: shaper.rs:9-10 says "takes it
  1 ns after the first has left, not before"; the test (shaper.rs:119-120) shows it is taken AT the departure ns.
- (6) Tests: 45 pass, 1 ignored (release, all features, 4.87 s). M13 ignored run reproduced: 100,000/dir in 10.000 s,
  delivered up 97,926 / down 98,017 (identical to the build notes: fates deterministic for one peer); up lateness p50
  28.5 us p99 153.4 p99.9 4,374 max 9,136 E 13.5 us; down p50 32.5 p99 105.6 p99.9 2,956 max 8,928 E 7.8 us; release
  CPU 0.393 core; overtaken 507/485, reordered 910/909 permille; slack 1. (Container shared with my own work.)
