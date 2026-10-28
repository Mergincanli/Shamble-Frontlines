# R6-SEC draft: security baseline (deferred)

Not researched: the Advisor deferred it on 2026-10-27 ("In R6, cover hosting, costs, monitoring and telemetry; the
security baseline waits until later."). No findings, options or recommendations.

## Open questions
- **R6-SEC-OQ1. Security baseline (content security policy, dependency audits, anti-cheat basics): deferred by the
  Advisor, 2026-10-27. ADR 0006 lists it as open, or it is researched before 0006 is accepted.** The items wave 1 left
  to the baseline (from `docs/research/r2-netcode.md`, by id):
  - **Dependency audits:** the audit that keeps R2-TRN-REC1's version floors current (R2-TRN-REC1 mark 5, R2-TRN-F40:
    quinn-proto ≥ 0.11.18, rustls ≥ 0.23.46, ring ≥ 0.17.12, tokio ≥ 1.54.1); the rust-reviewer's cargo-audit and
    cargo-deny gates (R2's list of deferred reviewer findings, item 5: finding R-1).
  - **Content security policy:** the CSP that R2-ROOM-OQ11 weighs (where the guest secret lives: an HttpOnly cookie or
    local storage).
  - **Anti-cheat basics:** the cheat detection R2-SYNC-REC5's per-command view-time offset and aim log is kept for;
    requests without an Origin are tagged for detection (R2-TRN-REC1); the reader-client and byte-audit tests as
    anti-cheat evidence (R2-INT-REC2, REC5, listed in R2's security-auditor pointers).
  - **Key storage:** the storage of the lobby's token-signing keys (R2-ROOM-OQ2, R2-ROOM-OQ9).
  - Also listed for security by R1's pointers (not R2's): R1-DATA-REC7's allow list and fuzz corpus, and COOP as a guard
    against XS-Leaks (R1-BND-REC5's unit).
  Items R6 met on the way and left here: the DNS API key the wildcard certificate job needs (R6-HOST-REC4); the Sentry
  and Cloudflare tokens CI would hold (R6-OBS-REC3, R6-HOST-REC1); whether previews are private (R6-HOST-OQ5). Secrets
  are Mustafa's to add; agents never add them.
