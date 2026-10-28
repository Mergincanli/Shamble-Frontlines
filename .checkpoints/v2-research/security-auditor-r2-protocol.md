Task: v2-research wave 1 step 3 (Verify) -- security-auditor review of R2's recommended protocol (docs/research/r2-netcode.md) for security and anti-cheat; claim checks with exact quotes; gaps ranked; verdict.

## Progress log
- 2026-10-28 start: notes file created fresh (none existed). Next: read memory, r2-netcode.md.
- Read r2-netcode.md header (1-60), sync findings (66-410), all Recommendations (1788-2487). No agent memory dir exists (.claude/agent-memory/security-auditor absent); task rule 1 forbids writing outside notes, so durable decisions go in this file's "Threat model" section only.
- Security-relevant recs found: SYNC-REC2 (command validation, drop old, per-tick cap, forged-command test), REC5 (view time accept within 200 ms of estimate; cap 250 ms vs BUD-REC4 200 ms), REC8 (client aim point, server rebuilds from own origin), INT-REC2/REC5/REC7 (info hiding, field classes, spectator delay), TRN-REC2/3/5/6 (datagram cap, WS fallback, certs), ROOM-REC3/4/5/8 (join token, reconnect, room codes, guest secret).
- Next: read transport findings (823-1228), rooms findings (1228-1417), interest findings (636-823), OQs (2489-2925), sources.
