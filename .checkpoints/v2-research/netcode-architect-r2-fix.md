Task: R2 netcode wave 1 step 4 (Fix): apply verifier, security-auditor and rust-reviewer results to docs/research/r2-netcode.md

## Progress
- 2026-10-28: read plan, amendments, the three reviewer reports, topic file parts 3-6 (recs, options, OQs). Fix list drafted in my head; writing it down next.

## Fix list (planned)
A. Unsupported basis: SYNC-F34 (SYNC-REC5 reason + mark 2); ROOM-F17 "owner alone" (ROOM-REC5 "sole write rights", options 3.4).
B. Verifier other: BUD-REC4/REC5 getStats (TRN-F9) -> Safari only, XU-OQ10 resolved by its own rule; BUD-REC1/REC4 one-way vs RTT -> correct text, XU-OQ2 update; SYNC-REC5 cap applies to final view time; XU-OQ18 add INT-REC7, INT-REC4 (6.9 renderer); TRN-REC3 HTTP/2 claim; ROOM-REC6 TF2 fill opt-in; BUD-REC4 input buffer 1-3 ticks proposed; TRN-REC4 Rests on + F34; label upgrades (item 10); TRN-F12 location §6.2.
C. Security: H1 SYNC-REC5; H2 SYNC-OQ11 + SYNC-F20 quote + option 3.2C/GC5 row; H3 limits (TRN-REC2/REC1 marks); H4 WS fallback (TRN-REC3); M1 Origin (TRN-F36 into a REC); M2 SYNC-REC2 validation table + SYNC-F19 g_active.c; M3 SYNC-OQ3 + REC2 mark; M4 BUD-REC2 event queue bound; M5 SYNC-REC6/INT-REC6 origins + INT-REC2 test; M6 ROOM-REC8 storage (OWASP); M7 ROOM-REC5 brute force; M8 ROOM-OQ2; M9 fuzz; M10 INT-REC7 follow rule; L1 SYNC-REC8 aim help; L2 INT OQ; L3 ROOM-REC3 logs; L4 display name; C28 TRN-F10 wording.
D. Rust: TRN-F28 "mature" + RustSec (new finding); TRN-F17 quinn min datagram size; ROOM-F7 label; TRN-OQ5 answer; TRN-REC5 rcgen dev-dep; axum ^0.29 vs 0.30; panic/unwind vs process; dangerous-configuration off.
E. Part 7 merged table; header; totals.
