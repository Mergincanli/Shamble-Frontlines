# U3 scout (netcode-architect): R5-TEST-OQ2, WebTransport servers over a custom quinn socket

Branch claude/netcode-proof-test-plan-hcky5g, head ad527f3. Research only: no source, manifest or doc edits.
Registry root R = /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f

## Plan
1. Cargo.lock: which quinn / quinn-proto / quinn-udp each WT crate links (Q4).
2. quinn 0.11.12: AsyncUdpSocket, UdpPoller, Transmit, RecvMeta, Endpoint::new_with_abstract_socket, Runtime (Q3).
3. quinn-proto: how GSO/GRO segment counts, may_fragment, local_addr, WouldBlock are used; RTT/loss/pacing timing (Q3).
4. wtransport 0.7.2: every server constructor, the call chain to quinn::Endpoint (Q1).
5. web-transport-quinn 0.13.6: server accept from a caller-built quinn::Endpoint (Q2).
6. ADR 0001 PM-0001-23/-24 and R-17 queue depths (Q5). PM-0001-3 L2 probe (Q6).
7. Optional probe build under .gate/netcode-proof-test/ if a claim needs compile proof.
8. Recommended construction per crate; DONE.

## Findings (one line each, as found)
