# R6: Infrastructure

## 1. Header

- **Topic:** R6, infrastructure (`docs/V2_PHASE0_BRIEF.md` §4, row R6).
- **Author agent:** infra-engineer (wave 2 of `.checkpoints/v2-research/plan.md`: one owner agent per topic).
- **Date:** 2026-10-28 (every source below was read that day).
- **Status:** draft: spot-check pending.
- **The brief's questions for this topic (quoted, `docs/V2_PHASE0_BRIEF.md` §4, row R6):** "Hosting for previews and
  EU game servers; costs; logs, metrics and crash reports; balance telemetry (VISION_V2 §6.6); security baseline
  (content security policy, dependency audits, anti-cheat basics)"
- **Scope, as the Advisor amended it on 2026-10-27:** "In R6, cover hosting, costs, monitoring and telemetry; the
  security baseline waits until later." The CI side of preview links (how a pull request gets its link) is R5's; R6
  covers what the hosts can do, their limits and prices. Cross-origin isolation stays R1's question; R6 only states,
  per host, whether it can send the headers.

| # | Question | Ids | Draft and notes (`.checkpoints/v2-research/`) |
|---|---|---|---|
| 1 | Hosting for previews and EU game servers | R6-HOST-F1–F24, REC1–REC4, OQ1–OQ7 | `infra-engineer-r6-r6-host-draft.md` |
| 2 | Costs | R6-COST-F1–F5, REC1–REC2, OQ1–OQ5 | `infra-engineer-r6-r6-cost-draft.md` |
| 3 | Logs, metrics and crash reports | R6-OBS-F1–F12, REC1–REC4, OQ1–OQ5 | `infra-engineer-r6-r6-obs-draft.md` |
| 4 | Balance telemetry (VISION_V2 §6.6) | R6-TEL-F1–F2, REC1–REC3, OQ1–OQ5 | `infra-engineer-r6-r6-tel-draft.md` |
| 5 | Security baseline (deferred) | R6-SEC-OQ1 | `infra-engineer-r6-r6-sec-draft.md` |

Notes file: `infra-engineer-r6.md` (one summary per source, saved as read).

Totals: 43 findings, 13 recommendations, 23 open questions, 42 sources read (41 official, 0 shipped game, 1 other;
plus `docs/LOOP_V2.md` as a project input, not counted), 17 sources not read.

**How to read this file.**
- Wave 1's findings and recommendations are cited by id (R1-…, R2-…), never copied as R6 findings. Legacy ids (CD2,
  TL13, …) are rows of `docs/LEGACY_INVENTORY.md`; the inventory's marks are working recommendations (the Advisor's
  R7-MARKS (a)).
- Assumptions are numbered A-1 to A-11 across the questions and listed under each question's options.
- Prices are research, not a commitment: no account was created and no service configured. EUR and USD stay apart (no
  exchange rate was read). Every price excludes VAT unless the source says otherwise.
- No spike was run and no code was written. Every pass mark and reference machine is proposed; nothing here is
  decided, and nothing here touches the look.

## 2. TL;DR
- Client and per-PR previews: Cloudflare (Workers static assets, or Pages) sends COOP and COEP from a `_headers` file, gives every branch its own link with no cap, and charges nothing for static traffic; GitHub Pages cannot send those headers.
- Game servers: Hetzner Cloud in one EU location, one room per process on a dedicated-vCPU CCX23 (proposed as R2's reference server), with the 9.6 times cheaper shared CX33 measured beside it; no load balancer on the UDP path; one wildcard certificate through ACME.
- Cost: about 18 EUR a month (lean) to 102 EUR + 5 USD at the start; about 240–360 EUR + about 20 USD plus Sentry at the goal; even 8 full 48-player rooms stay inside one VM's included 20 TB of traffic.
- Operations: `tracing` JSON logs, Prometheus-format metrics with bounded labels (tick overruns, fallback share), Sentry with Wasm debug files in its EU region with IP collection off, Reporting API crash reports; all data in the EU; the law itself was not read (outside the allow list).
- Telemetry: one record per match written at Ended (content hash, mode id, a 10 s points timeline, tick-stamped objective and kill events with source ids, unit time per slot, moments, no player ids) in EU object storage, analysed in batches per content hash. The security baseline is deferred (R6-SEC-OQ1).
