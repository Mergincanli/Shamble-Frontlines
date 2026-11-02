SUBS = [
("over an honest client at the same RTT, under P0 and P1 | L1 | 1,000 shots per profile and client |",
 "over an honest client at the same RTT, under P0 and P1; a cheat client that delays every pong by 150 ms: with the transport's own RTT (quinn's `rtt()`, L1's link RTT standing in) it gains no more than the tolerance, and its logged gap between that RTT and its ping RTT is at least 145 ms in every one-second sample (150 ms less PM-0001-33's 5 ms honest bound; proposed); on the WebSocket path, where the RTT is the lowest recent ping sample, its extra rewind is recorded (part 8's accepted risk, held to the cap by PM-0001-49) | L1 | 1,000 shots per profile and client |"),
("| r2 R2-SYNC-REC5 mark 5 (lines 2188–2190); the draft, part 8 (the view-time tolerance) |",
 "| r2 R2-SYNC-REC5 mark 5 (lines 2188–2190); the draft, part 8 (the view-time tolerance, the server's own RTT, quinn's `rtt()` logged beside the ping RTT); the security review of 2026-11-02, finding 6 |"),
]
