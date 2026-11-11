B11 notes: advisor-u2.md, advisor-u3q.md:180-186, .claude/rules/netcode.md (all at df8975b).

advisor-u3q.md:180-186 (the fresh-delta reading), word for word:
"- FINDING (the bite on L2): "decodes deltas again" read as "the first delta decoded after the link returns" can be met by STALE deltas: on WebSocket the 64 KB send budget holds the snapshots written during the burst (encoded against a baseline still valid then), and on quinn the datagram send buffer (1 MiB default, quinn-proto transport.rs:396) does the same. Both are decoded after the return even with the bite "a baseline that never falls back", so the bite would be Missed on L2. Reading needed (mine, inside the row being rewritten): a delta counts once it was encoded at or after the link's return. It also ties U6's datagram_send_buffer_size (verify-wt design problem 9) to -18 on WebTransport: a large buffer delays the first fresh datagram behind the stale ones."
-> For U4r on L1: each client's burst_heal = SimClock ns of the first decode of a delta whose encode instant is at or after the link's return, minus the return instant; the lab knows both instants (one process). L1 has no send buffer, so stale deltas are rare on L1, but the reading binds the record either way. U4j splits -18 into burst_heal and burst_return (plan :632): names at <base> (recheck).

advisor-u2.md (U2 close):
- :29 check.mjs 9 steps at a56d4be (now 10).
- :38-40 R6 finding -> C4 (:101-103): "a measured run, or a twin of one, whose build lists `bites` or names a bite, and a bite run whose build lacks it, are INVALID, with fixtures" (U4j builds C4: plan :627). So U4r's run script must run measured runs on the measured build, bite runs on the bites build.
- :45-47, :72-78 C1 (stream backoff, per-segment loss) "due before any L1 mark reads stream timing (PM-0001-18 first, U4)"; U4c's C1' (PTO doubling, L-c3) is its successor. -18's heal reads datagrams at U4r (a delta coded against defaults is a Snapshot datagram).
- :79-83 C2 "due before npt-room reads rtt_ns (U4)": sample only on newest packet; stream sample from own packet arrival (U4c's L-c7). The pre-check's per-client first-sample log reads it.
- :97-103 R6: bite twins from the bites build; records verdict.json + runs.txt committed, raw logs .gate/ (plan §10.6).
- :99-100 clock_res_ns = 1 for a simulated-clock run.
- :107-109 R8: a missing twin -> NOT-RUN, record faults INVALID.
- runs.txt model (c-u2 :93-104): "the judge reads the logs, never these lines".

.claude/rules/netcode.md (18 lines): paths server/, shared/, net/ (:2-5); validate every inbound message; seeded RNG; no Date.now/performance.now in shared code; deterministic iteration order; no unbounded buffers/maps/queues keyed by client data; interest management. Plan §7.1 item 8 (:876-877): every brief names it as binding on proto, room, client and server code. U4r edits none of those four crates (lane), but its replay clients and bots feed them.
- B11 done. All reading blocks done (B1-B11). Next: read §4.6 and §4.9 (:296-347) for -57's pairing and the RTT argument (sections U4r builds to), then W1.
