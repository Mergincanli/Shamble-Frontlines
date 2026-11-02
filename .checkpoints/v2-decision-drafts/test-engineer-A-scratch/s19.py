SUBS = [
("on every tick | RS (any machine for the hash) | 3 recordings of 5 minutes (proposed), each replayed over WebTransport and over WebSocket | 3 recordings; 0 mismatching ticks. Bites: a wall-clock read on one transport's command path |",
 "on every tick; a run counts only if every command ran at the tick stamped on it and the server's late-command counter reads 0 at its end, whatever OQ-0001-6 picks for play (a late command makes the run invalid, not failed: the draft's item 1) | RS (any machine for the hash) | 3 recordings of 5 minutes (proposed), each replayed over WebTransport and over WebSocket at P0, the input buffer set deep enough for P0 on both transports (the draft's item 1) and recorded | 3 recordings; 0 mismatching ticks. Bites: a wall-clock read on one transport's command path; the validity check itself: one command of a replay delayed past its tick must make that run invalid (counter above 0), never a pass |"),
("| r2 R2-TRN-REC3 mark 2 (lines 2666–2667) |",
 "| r2 R2-TRN-REC3 mark 2 (lines 2666–2667); the draft, \"What a spike must prove\" item 1 and the test plan (the late-command counter at 0); the test review of 2026-11-02, finding 3 |"),
]
