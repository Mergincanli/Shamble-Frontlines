# Security guidance: SHAMBLE FRONTLINES

Context: a static browser client (Three.js, classic scripts) today; v2 plans an authoritative Rust game server over
WebTransport with a WebSocket fallback, sharing its simulation with the browser as WebAssembly. Assume every client is
hostile and modified.

- Server code must never accept client-reported state (position, hit, damage, health, ammo, cooldown, credits, XP,
  match results). It derives state from validated inputs only.
- Every inbound network message is schema-validated (type, range, length, enum), rate-limited and size-capped before use;
  unknown fields are rejected; sequence numbers must increase; client timestamps are clamped.
- No unbounded maps, queues or buffers keyed by client-provided data. Every room and connection has caps and timeouts.
- Player-provided strings (names, chat) reach the DOM only through textContent, never innerHTML or insertAdjacentHTML.
- No eval, new Function or dynamic script injection. No secrets or tokens in client code or committed files; .env stays local.
- Auth tokens are signed and short-lived; compare secrets with crypto.timingSafeEqual.
- Logs never contain tokens or raw client payloads.
- KVStore/localStorage is player-controlled: never trust it for anything competitive.
