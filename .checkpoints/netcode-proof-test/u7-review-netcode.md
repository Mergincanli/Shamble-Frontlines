# U7 review round: the netcode seat (netcode-architect, 2026-11-26)
Packet: u7-round-packet.md. Diff: wip/u7-build-rs.patch (42 files, base 9177fb5). Brief u7-brief.md rev 1.1 + C1-C14
(advisor-u7-brief.md:110-247; C6 as reworded in advisor-main-66-head.md section 1). Read only, except this file.

## Plan
1. Builder notes (rs 0030, t 2050). 2. crates/client clock.rs, lib.rs, tests (R1, R2, C3). 3. client-wasm lib.rs,
engine.rs, script.rs (R3, C2, C5, C6). 4. web/src lib.ts, worker.ts, main.ts, hooks.ts (URL, token, framing, chooser,
C9, records C1). 5. e2e t5ii.mjs, room.mjs, serve.mjs, t5i.mjs (transport legs, known-open). 6. Logs. 7. Findings.

## Notes (one line per finding, as I go)
