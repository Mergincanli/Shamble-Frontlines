---
paths:
  - "**/server/**"
  - "**/shared/**"
  - "**/net/**"
---
# Rules for multiplayer code (active once these folders exist; see docs/adr/)
- The server is authoritative. Clients send INPUTS (sequenced commands), never state. Never accept client-reported
  position, hit, damage, health, ammo, cooldown, currency or progression.
- Validate every inbound message before use: schema, types, ranges, lengths, sequence monotonicity, timestamp clamps;
  reject unknown fields; per-connection rate and size limits; disconnect abusers.
- Shared simulation code runs identically in the browser and in Node: no DOM, no THREE scene, no `Math.random` (seeded
  RNG), no `Date.now` / `performance.now` (use the tick counter), deterministic iteration order.
- Fixed tick (`FIXED_DT`). Lag-compensation rewind is bounded by the ADR value. No unbounded buffers, maps or queues
  keyed by client data.
- Send each client only what it may know (interest management). No secrets in client bundles; secrets via environment only.
- Every protocol change updates the message catalogue in docs/design/ and its tests, including latency, jitter and loss cases.
