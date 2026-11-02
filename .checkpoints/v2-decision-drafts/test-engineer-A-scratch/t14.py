SUBS = [
("| PM-0002-33 | A Wasm trap reaches error reporting (part 9; only if Mustafa has created the EU Sentry organisation) | a deliberate Wasm trap in the Worker arrives in Sentry with Rust function names in Chrome, Edge, Firefox and Safari (4 of 4) |",
 "| PM-0002-33 | A Wasm trap reaches error reporting with its panic message (parts 4 and 9; the Sentry half only if Mustafa has created the EU Sentry organisation) | a deliberate panic in the Worker's sim, with a known message: the trap's report carries the panic hook's message and location (ADR 0004 part 20's fixed buffer, read through a fresh view: part 4), in the page's own error log in Chrome, Edge, Firefox and Safari (4 of 4), always; and the report arrives in Sentry with Rust function names and that message (4 of 4), when Sentry exists |"),
("| 1 trap per browser; conditional |",
 "| 1 trap per browser; the Sentry half conditional. Bites: a host that reads the hook's buffer through a view taken before the instance's memory grew (it reads zero bytes) |"),
("| R6-OBS-REC3's mark (r6, 765–766) |",
 "| R6-OBS-REC3's mark (r6, 765–766); the draft, part 4 (the hook's buffer read after a trap); the Rust review of 2026-11-02, finding 9 |"),
]
