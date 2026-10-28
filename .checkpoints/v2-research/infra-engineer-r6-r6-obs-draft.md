# R6-OBS draft: logs, metrics and crash reports

All sources read 2026-10-28. Assumptions are marked **A-n** (numbering continues from R6-COST).

## Findings

### Rust server
- **R6-OBS-F1.** `tracing` 0.1.44 is a framework for structured, event-based diagnostics with spans (a period with a
  start and an end) and events (a moment); levels can be capped at compile time by features; a span or event no
  subscriber wants is not built, and a formatted message does not allocate by default. Source:
  https://docs.rs/tracing/latest/tracing/ (tracing 0.1.44) · official (crate docs) · read 2026-10-28 · confirmed in the
  source: "a framework for instrumenting Rust programs to collect structured, event-based diagnostic information.";
  "represents a period of time with a beginning and an end"; "represents a moment in time."; "A set of features
  controlling the static verbosity level."; "does not allocate by default." That unwanted spans are never built is
  inferred (the quote was cut: "For performance reasons, if no currently active subscribers express interest").
- **R6-OBS-F2.** `tracing-subscriber` 0.3.23 writes newline-delimited JSON logs (features `fmt`, `json`, `std`), one
  object per event with timestamp, level and fields. Source:
  https://docs.rs/tracing-subscriber/latest/tracing_subscriber/fmt/format/struct.Json.html (0.3.23) · official · read
  2026-10-28 · confirmed in the source: "the newline-delimited JSON log format should be used."; "Available on **crate
  features `fmt` and `json` and `std`** only."; `{"timestamp":"2022-02-15T18:47:10.821315Z","level":"INFO","fields":`.
- **R6-OBS-F3.** OpenTelemetry for Rust lists traces as Beta and metrics and logs as Stable; it publishes
  `opentelemetry`, `opentelemetry-sdk` and `opentelemetry-otlp` (no versions on the page). Source:
  https://opentelemetry.io/docs/languages/rust/ (no date) · official · read 2026-10-28 · confirmed in the source: the
  status table's "Beta" (traces), "Stable" (metrics), "Stable" (logs).
- **R6-OBS-F4.** `metrics-exporter-prometheus` 0.18.3 exposes the `metrics` facade's values on an HTTP endpoint for
  Prometheus to scrape, or pushes them to a push gateway; histograms go out as summaries or as histograms. Source:
  https://docs.rs/metrics-exporter-prometheus/latest/metrics_exporter_prometheus/ (0.18.3) · official · read 2026-10-28
  · confirmed in the source: "A metrics-compatible exporter for sending metrics to Prometheus."; "exposing an HTTP
  endpoint that can be scraped by Prometheus"; "push histograms as either aggregated summaries or aggregated
  histograms". The default between the two is not stated.
- **R6-OBS-F5.** Prometheus 3.15 keeps 15 days by default, stores 1–2 bytes per sample, sizes disk as retention ×
  samples per second × bytes per sample, and its local storage is neither clustered nor replicated. Source:
  https://prometheus.io/docs/prometheus/latest/storage/ (docs for 3.15) · official · read 2026-10-28 · confirmed in the
  source: "the retention time defaults to `15d`"; "Prometheus stores an average of only 1-2 bytes per sample.";
  "needed_disk_space = retention_time_seconds * ingested_samples_per_second * bytes_per_sample"; "Note that a
  limitation of local storage is that it is not clustered or replicated."
- **R6-OBS-F6.** The Sentry Rust SDK (`sentry` 0.49.5) captures panics after init and has a `tracing` integration.
  Source: https://docs.sentry.io/platforms/rust/ (no date) · official · read 2026-10-28 · confirmed in the source:
  `sentry = "0.49.5"`; "The quickest way to verify Sentry in your Rust application is to cause a panic."; "Sentry will
  capture this"; the integrations list names "tokio-rs/tracing".

### Browser client
- **R6-OBS-F7.** Sentry symbolicates WebAssembly crashes from DWARF debug files that carry a `build_id` custom section
  (added by its `wasm-split` tool), uploaded to Sentry or served from a symbol server; the browser side uses
  `@sentry/wasm`'s `wasmIntegration` with `@sentry/browser`; Sentry calls the feature "still developing". Source:
  https://docs.sentry.io/platforms/native/guides/wasm/ (no date) · official · read 2026-10-28 · confirmed in the source:
  "we require debug information files to be augmented by a `build_id` custom section."; "upload your WebAssembly debug
  information files to Sentry or use a symbol server."; "We provide a tool called wasm-split that can automatically add
  this section if missing."; "use the `@sentry/wasm` package and apply its `wasmIntegration`"; "This feature is still
  developing, so may be rough around the edges." (The JavaScript Wasm integration page returned 404.)
- **R6-OBS-F8.** The Reporting API (Baseline 2026, newly available since March 2026) sends reports to endpoints named
  in a `Reporting-Endpoints` header; its types include `crash`, `coep`, `coop` and `csp-violation`; crash reports
  cannot be observed in JavaScript because the page has crashed, but they reach an endpoint. Source:
  https://developer.mozilla.org/en-US/docs/Web/API/Reporting_API (last modified Apr 21, 2026) · official · read
  2026-10-28 · confirmed in the source: "The Reporting API provides a generic reporting mechanism for web applications
  to use"; "The Reporting-Endpoints HTTP header is used to specify the name and URL for different endpoints"; "Since
  March 2026, this feature works across the latest devices and browser versions."; "Some parts of this feature may have
  varying levels of support."; "except for `crash` reports, which can't be observed in JavaScript (because the
  observing page has crashed)". The type list is inferred (table extracted by the fetch).

### Data protection and where data lives
- **R6-OBS-F9.** Sentry stores event data in the US or the EU (Frankfurt), chosen when the organization is created and
  never changeable; some data (user accounts, organization settings) may be stored in the US whatever the choice.
  Source: https://docs.sentry.io/organization/data-storage-location/ (no date) · official · read 2026-10-28 · confirmed
  in the source: "Sentry supports data storage in both the US and the EU."; "Frankfurt, Germany."; "once selected, your
  data storage location can’t be changed."; "Here’s a list of the types of data that may be stored in the US,
  regardless of your selected Data Storage Location." Which types are on the list is inferred (summary).
- **R6-OBS-F10.** Sentry's JavaScript SDK sends the user's IP address by default; `dataCollection: { userInfo: false }`
  turns it off, and IPs can still arrive in headers, cookies or query strings unless denied. Source:
  https://docs.sentry.io/platforms/javascript/data-management/data-collected/ (no date) · official · read 2026-10-28 ·
  confirmed in the source: "By default, the SDK sends the user's IP address."; "To disable it, set `dataCollection: {
  userInfo: false }`."; "the IP address won't be inferred." The headers, cookies and query point is inferred (summary).
- **R6-OBS-F11.** Grafana Cloud runs stacks in EU regions, including Germany (AWS eu-central-1), and a stack's region
  cannot be changed later. Source:
  https://grafana.com/docs/grafana-cloud/security-and-account-management/regional-availability/ (no date) · official ·
  read 2026-10-28 · confirmed in the source: "Grafana does not support changing the Grafana Cloud region of an existing
  stack." The region list (Germany, Ireland, Sweden direct; Belgium, Netherlands through marketplaces) is inferred
  (table extracted by the fetch).
- **R6-OBS-F12.** OWASP: session ids, access tokens and sensitive personal data should not go into logs directly, but
  removed, masked, hashed or encrypted; logs must be kept no longer than the required retention period. Source:
  https://cheatsheetseries.owasp.org/cheatsheets/Logging_Cheat_Sheet.html (no date) · official (OWASP, tiered as R2
  does) · read 2026-10-28 · confirmed in the source: "Session identification values"; "Access tokens"; "Sensitive
  personal data and some forms of personally identifiable information (PII)"; "removed, masked, sanitized, hashed, or
  encrypted."; "must not be destroyed before the duration of the required data retention period, and must not be kept
  beyond this time."; "Legal, regulatory and contractual obligations may impact on these periods."

Free tiers and prices used here are R6-COST-F1 to F3 (Grafana Cloud Free: 10k series, 50 GB logs, 14 days; Sentry
plans and per-error rates).

## Legacy facts used
- **GP5 and PG13 (dev panel perf overlay):** legacy measures frame time, draw calls and heap only on the player's own
  screen and sends nothing anywhere; v2 has no field data to start from.
- **TL15 (perfbots):** p50, p95 and max per step, the same shape v2's tick histograms take (R1-TICK-REC4, R2-BUD-REC6).
- **TL9 (smoke test):** fails on any network request; once v2 reports errors, the gate v2 (R5) must allow the report
  endpoints in the test build or switch reporting off there.

## Options

### Q1. Server logs
- **L1. `tracing` with JSON lines to standard output per room process** (F1, F2); the supervisor tags each line with
  VM and room and forwards it. Low effort; any log store reads JSON lines.
- **L2. OpenTelemetry logs** (Stable, F3) over OTLP: one protocol for logs and metrics; more crates, and nothing in §6
  needs the cross-service view it adds.

### Q2. Server metrics
| Option | What | Cost | Risk |
|---|---|---|---|
| M1. `metrics` + Prometheus exporter, scraped by a self-hosted Prometheus on the staging VM | F4, F5 | 0 extra (staging VM, R6-HOST-REC3); disk about 0.6 GB at A-8 | Not replicated (F5): metrics lost with the VM, not the game |
| M2. Same exporter, stored in Grafana Cloud (EU stack) | F4, F11, R6-COST-F1 | Free under 10k series | How scraped data reaches Grafana Cloud was not read (R6-OBS-OQ2) |
| M3. OpenTelemetry metrics (Stable, F3) over OTLP | F3 | as M2 | More dependencies in the room binary |

### Q3. Traces
- **T1. Spans stay local:** tick phases (simulation, lag compensation, encoding) are timed with `tracing` spans or
  plain timers and exported as histograms (M1/M2). Serves R2-BUD-REC6's separate measurements.
- **T2. Distributed tracing with OpenTelemetry:** Beta in Rust (F3), and one room process has no cross-service request
  path to follow: a generic feature no §6 item asks for.

### Q4. Client errors and crashes
| Option | Catches | Wasm frames | Privacy | Cost |
|---|---|---|---|---|
| C1. Sentry browser SDK + `wasmIntegration`, debug files uploaded at build | JS errors, Wasm traps with symbols | Yes, "still developing" (F7) | EU storage at creation (F9); IP off by setting (F10) | Free tier, then per error (R6-COST-F3) |
| C2. Reporting API `crash` (and later `coep`, `coop`, `csp-violation`) to our own endpoint | Tab crashes JS cannot see (F8) | No stack | Our own store, EU | 0 (our server) |
| C3. Our own `error` and `unhandledrejection` handler posting to our endpoint | JS errors; Wasm traps as raw offsets | Offsets only, symbolicated by us | Our own | Our work |

### Q5. Data protection
- **P1. EU-only storage for every store** (Hetzner EU, Sentry EU, a Grafana EU stack; F9, F11); no IP addresses,
  tokens or guest secrets in logs or reports (F10, F12; R2-ROOM-REC3 mark 3); bounded retention.
- **P2. A provider's default region and settings:** US storage for Sentry unless chosen at creation (F9), IP sent by
  default (F10).

## Recommendations
- **R6-OBS-REC1. Server logs: `tracing` with JSON lines on standard output per room process; the supervisor adds VM and
  room ids and ships them; no tokens, guest secrets or raw ids in any line (salted hashes where an id is needed, as
  R2-ROOM-REC3 mark 3 says).**
  *Reason:* structured events at little cost (F1, F2); OWASP's exclusions (F12) match R2's log rule.
  *Rests on:* F1, F2, F12; R2-ROOM-REC3 (cited).
  *Serves:* §6.1 (per-room crash reports and lifecycle, R2-ROOM-REC1, R2-ROOM-OQ9).
  *Spike must prove (proposed):* with logging at its production level, tick p99 within 2 % of a run with logging off;
  0 allocations per tick from logging on the hot path (counted); a grep of a 30-minute soak's logs finds 0 tokens,
  secrets or raw IPs.
- **R6-OBS-REC2. Server metrics: the `metrics` facade with `metrics-exporter-prometheus`, scraped by a Prometheus on the
  staging VM to start (M1); Grafana Cloud EU (M2) once its ingestion path is read. Labels are bounded (VM, room slot
  0–7, mode id, transport), never a player or match id. The first metrics are the ones wave 1 asked for:** tick time
  histogram and overruns and dropped time (R1-TICK-REC4), per-phase time (R2-BUD-REC6), sessions by transport
  (WebTransport or WebSocket: the fallback share, R2-TRN-OQ7), bytes and datagrams sent and dropped, per-client budget
  overruns and relevance churn (R2's pointers), joins refused by reason, room starts, ends and crashes, process memory.
  *Reason:* the exporter and Prometheus are read and small (F4, F5); bounded labels keep the series count near A-8's
  3,600, inside Grafana's free 10k (R6-COST-F1) if M2 follows; distributed tracing (T2) is Beta (F3) and serves no §6
  item.
  *Rests on:* F3, F4, F5, F11; R6-COST-F1; R1-TICK-REC4, R2-BUD-REC6, R2-TRN-OQ7 (cited); A-8.
  *Serves:* §6.1 (rooms); §4 (fair to about 100 ms: the tick budget and the fallback share are what make it visible).
  *Spike must prove (proposed):* every metric listed is present after a 30-minute 48-player soak; a deliberately
  stalled tick shows as one overrun and its dropped time within one scrape; the series count is reported and is under
  5,000 for 8 rooms on one VM.
- **R6-OBS-REC3. Client errors and crashes: Sentry's browser SDK with `wasmIntegration` and build ids from `wasm-split`
  (C1), debug files uploaded by CI and never shipped, the organization created in the EU location, `userInfo` off; plus
  Reporting API `crash` reports to our own lobby endpoint (C2). Server panics go to Sentry's Rust SDK from the room
  process, and the supervisor records every abnormal room exit.**
  *Reason:* Wasm frames need debug files and build ids (F7); tab crashes, such as running out of memory, are invisible
  to JavaScript but reach a reporting endpoint (F8); the EU location must be chosen at creation and cannot change
  (F9); the SDK sends IPs unless told not to (F10); Rust panics are captured (F6).
  *Rests on:* F6, F7, F8, F9, F10; R6-COST-F3.
  *Serves:* §4 (four desktop browsers: crashes per browser are the evidence); §6.1 (per-room crash reports,
  R2-ROOM-OQ9).
  *Spike must prove (proposed):* a deliberate Wasm trap in the simulation worker arrives in Sentry with Rust function
  names in Chrome, Edge, Firefox and Safari (4 of 4); the shipped `.wasm` holds no DWARF (size checked against the
  stripped build); a forced tab crash in Chrome produces a `crash` report at our endpoint (browsers that send none are
  listed); 0 IP addresses in a sample of 100 events.
- **R6-OBS-REC4. Data protection defaults: every store in the EU (P1), no IP addresses or secrets in logs and reports,
  and retention capped: logs 14 days, metrics 15 days, crash reports at Sentry's shortest setting (proposed).** The
  legal side (lawful basis, privacy notice, how long the law allows) is not researched here: R6-OBS-OQ1.
  *Reason:* the EU choice is one-way on both Sentry and Grafana (F9, F11); OWASP asks for bounded retention and no
  personal data in logs (F12); 14 and 15 days are the free tier's and Prometheus's defaults (R6-COST-F1, F5).
  *Rests on:* F5, F9, F10, F11, F12; R6-COST-F1.
  *Serves:* §6.1 and §6.6 (the data they produce stays within the rules Mustafa sets).
  *Spike must prove (proposed):* a data map lists every store, its region and its retention, with 0 stores outside the
  EU for event data; data older than the cap is gone on a check one day after the cap.

## Open questions
- **R6-OBS-OQ1. What does EU data-protection law require of us?** Whether an IP address or a guest id is personal data,
  the lawful basis, the privacy notice and the retention limits: the GDPR's text (eur-lex.europa.eu) and the European
  Data Protection Board's guidance are outside the allow list. Decided by: Mustafa, with a read of those sources (or
  advice) before any public playtest.
- **R6-OBS-OQ2. How do metrics and logs reach Grafana Cloud?** The agent or remote-write path was not read. Decided by:
  a read of Grafana's ingestion docs if M2 is wanted.
- **R6-OBS-OQ3. Does Sentry's Wasm symbolication work for Rust built by wasm-bindgen?** Sentry's page names C, C++ or
  Rust only in passing and calls the feature still developing (F7). Decided by: REC3's spike.
- **R6-OBS-OQ4. Which browsers send `crash` reports?** MDN warns of varying support (F8). Decided by: REC3's spike.
- **R6-OBS-OQ5. Client performance in the field** (frame time per match against VISION_V2 §4's 30 and 60 fps): worth a
  per-match summary in the match record? R3 owns how it is measured. Decided by: R3 and the synthesis.

### Recommendations that hinge on a source not read
| Recommendation | Source | Reason not read |
|---|---|---|
| R6-OBS-REC3 | https://docs.sentry.io/platforms/javascript/configuration/integrations/wasm/ | 404 on 2026-10-28 |
| R6-OBS-REC4 | https://eur-lex.europa.eu/eli/reg/2016/679/oj (GDPR) | outside the allow list |
| R6-OBS-REC2 (M2 path) | Grafana Cloud ingestion docs | not fetched in this run (R6-OBS-OQ2) |

## Assumptions
- **A-8.** Series per VM: about 50 per room slot (3 histograms of about 14 series each, plus about 8 counters and
  gauges) × 8 slots, plus about 500 host series (assumption) ≈ 900; 4 VMs ≈ 3,600. At a 15 s scrape, 240 samples per
  second × 15 days × 2 B ≈ 0.6 GB on disk (F5's formula).
- **A-9.** Logs: about 300 B per line, about 200 lines per match (lifecycle, joins, leaves, warnings); at the goal's
  average of 6 rooms with 20-minute matches, about 13,000 matches and 0.8 GB per month, far under 50 GB.

## Sources read
1. https://docs.rs/tracing/latest/tracing/ · tracing 0.1.44 · official
2. https://docs.rs/tracing-subscriber/latest/tracing_subscriber/fmt/format/struct.Json.html · tracing-subscriber 0.3.23 · official
3. https://opentelemetry.io/docs/languages/rust/ · OpenTelemetry Rust · no date · official
4. https://docs.rs/metrics-exporter-prometheus/latest/metrics_exporter_prometheus/ · 0.18.3 · official
5. https://prometheus.io/docs/prometheus/latest/storage/ · Prometheus 3.15 storage · official
6. https://docs.sentry.io/platforms/rust/ · Sentry Rust SDK (sentry 0.49.5) · no date · official
7. https://docs.sentry.io/platforms/native/guides/wasm/ · Sentry WebAssembly · no date · official
8. https://developer.mozilla.org/en-US/docs/Web/API/Reporting_API · Reporting API · Apr 21, 2026 · official
9. https://docs.sentry.io/organization/data-storage-location/ · Sentry data storage location · no date · official
10. https://docs.sentry.io/platforms/javascript/data-management/data-collected/ · Sentry JS data collected · no date · official
11. https://grafana.com/docs/grafana-cloud/security-and-account-management/regional-availability/ · Grafana Cloud regions · no date · official
12. https://cheatsheetseries.owasp.org/cheatsheets/Logging_Cheat_Sheet.html · OWASP Logging Cheat Sheet · no date · official (OWASP)

## Sources not read
- https://docs.sentry.io/platforms/javascript/configuration/integrations/wasm/ · 404 ("Page Not Found")
- https://eur-lex.europa.eu/eli/reg/2016/679/oj · outside the allow list

## Cross-topic notes
- **R5:** CI uploads Wasm debug files to Sentry (a token: Mustafa's secret) and strips them from the shipped build; the
  gate v2's smoke test needs reporting off or the endpoints allowed (legacy TL9).
- **R1:** the `coep` and `coop` report types (F8) are the report-only rollout R1-BND-REC5 asks for.
- **R2:** the metric list in REC2 is R2's pointers (fallback share, relevance churn, budget overruns) plus R1-TICK-REC4.
