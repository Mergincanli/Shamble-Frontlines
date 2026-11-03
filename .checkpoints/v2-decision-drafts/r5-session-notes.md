# The session's notes for plan revision 5 (v2-decision-drafts)

## COMMUNITY-SERVERS: ADR 0003 and 0004 (r5 condition 6, last bullet; the session's grep, no author)
- Grep `room process|match record|supervisor` over docs/adr/0003-rendering.md and docs/adr/0004-simulation-core.md
  (2026-11-03, about 17:55 Amsterdam): ADR 0003:1510 names the match record only as ADR 0006's; ADR 0004:123 (the sim
  runs natively in each room process), :309 (the seed never goes into the match record), :545-552 (the room writes the
  match record at Ended; ADR 0006 owns its shape), :568 ("so the supervisor records the exit and the VM's other rooms
  run on": the official deployment's supervisor watches a room that exits; the room needs none to start or to exit).
  Note: no change; nothing in either draft needs a supervisor, a lobby or an official host for a room to run, or fixes
  the record's fields (rust-reviewer's NOTE COMMUNITY-SERVERS on ADR 0004 reads :568-569 the same way).
