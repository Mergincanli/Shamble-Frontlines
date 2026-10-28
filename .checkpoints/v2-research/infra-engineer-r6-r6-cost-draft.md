# R6-COST draft: monthly cost at the starting scale and at the goal

All sources read 2026-10-28. Prices as published that day, excluding VAT; EUR and USD are kept apart (no exchange
rate was read, so no conversion). Every assumption is marked **A-n** and listed at the end. This is research, not a
commitment: no account exists and none is created.

## Findings

- **R6-COST-F1.** Grafana Cloud Free: 10k active metric series, 50 GB of logs and 50 GB of traces ingested per month,
  14 days of retention, 3 active users, 50k frontend sessions per month. Source: https://grafana.com/pricing/ (no date,
  © 2026; the first fetch returned HTTP 502, the second worked) · official · read 2026-10-28 · confirmed in the source:
  "Limited to 10k active series per month"; "Limited to 50 GB ingested per month" (logs, and the same for traces);
  "Limited to 3 active users per month"; "Limited to 50k sessions per month". The 14-day retention is inferred (summary).
- **R6-COST-F2.** Grafana Cloud Pro: a 19 USD monthly platform fee plus usage: metrics from 6.50 USD per 1k series; logs
  and traces 0.05 USD per GB processed, 0.40 USD per GB written, 0.10 USD per GB retained. Source: as F1 · official ·
  read 2026-10-28 · confirmed in the source: "Platform fee of $19 per month"; "Starts at $6.50/ 1k series";
  "$0.050/GB Process," "$0.400/GB Write," "$0.100/GB Retain".
- **R6-COST-F3.** Sentry has a free Developer plan and paid Team, Business and Enterprise plans; paid plans include a
  monthly error volume (50k errors), and errors beyond it cost from 0.0003625 USD each (Team, pay as you go) or
  0.0011125 USD (Business). The base prices are on sentry.io, outside the allow list. Source:
  https://docs.sentry.io/pricing/ (no date) · official · read 2026-10-28 · confirmed in the source: "Sentry has one free
  Developer plan and three paid plans: Team, Business, and Enterprise."; "Each paid plan comes with the below pre-set
  monthly event volume, which is included in the price:"; "| >50k-100k | $0.0002900 | $0.0003625 | $0.0008900 |
  $0.0011125 |". The 50k figure is inferred (summary); the Developer plan's limits are not on the page.
- **R6-COST-F4.** Cloudflare R2 object storage: 0.015 USD per GB-month, 4.50 USD per million writes (Class A), 0.36 USD
  per million reads (Class B), free egress; free each month: 10 GB-month, 1 million Class A, 10 million Class B. Source:
  https://developers.cloudflare.com/r2/pricing/ ("Last updated Oct 26, 2026") · official · read 2026-10-28 · confirmed in
  the source: "$0.015 / GB-month"; "$4.50 / million requests"; "$0.36 / million requests"; "Free"; "10 GB-month /
  month"; "1 million requests / month"; "10 million requests / month". The currency is inferred from "$".
- **R6-COST-F5.** Hetzner Object Storage: 6.49 EUR per month base, including 1 TB stored and 1 TB of egress, in
  Falkenstein, Nuremberg and Helsinki. Source: https://www.hetzner.com/storage/object-storage (no date, © 2026) ·
  official · read 2026-10-28 · confirmed in the source: "We charge a base price of € 6.49 per month, excl. VAT per
  month."; "Storage: € 0.0087 per hour, excl. VAT per TB-hour." The included 1 TB and the locations are inferred
  (summary).

Prices from R6-HOST used here (not repeated as findings): Hetzner servers R6-HOST-F15 (CX33 8.99 EUR/mo, 0.0144/h;
CCX23 86.49 EUR/mo, 0.1386/h; 20 TB traffic included), extra traffic R6-HOST-F16 (1 EUR per TB, inferred reading),
hourly billing rounded up R6-HOST-F17, IPv4 R6-HOST-F19; Cloudflare static hosting R6-HOST-F5 (free static requests,
no egress charge; Workers Paid minimum 5 USD); Fly.io R6-HOST-F22 (0.02 USD per GB egress); GameLift R6-HOST-F23
(bandwidth free on gen 6+, c6a.large 0.099 USD/h on demand, Ohio).

## Legacy facts used
- **TL15 (perfbots):** 48 soldiers at 0.80–1.00 ms per legacy JavaScript step. It is the only CPU figure for a
  48-player room; R2-ROOM-REC2's "8 rooms on a 4-vCPU VM" is a proposal built on it, not a measurement.
- **CD2 (KVStore):** legacy stores nothing on a server, so today's hosting cost is zero; every line below is new.

## Arithmetic

### Egress per match-hour (from R2's worked estimate)
R2's per-client downstream at E3, 30 Hz, IPv4, every entity in view (R2's worked bandwidth estimate, R2-BUD-REC2),
times the number of humans (bots cost no traffic, R2). 1 kbps for one hour = 0.45 MB.

| Mode (VISION_V2 §3) | Humans (A-1) | Per client | Room downstream | Egress per match-hour |
|---|---|---|---|---|
| 4v4 (start, two teams, upper end) | 8 | 44.2 kbps | 354 kbps | 0.16 GB |
| 6v6v6 (start, three teams, upper end) | 18 | 70.3 kbps | 1.27 Mbit/s | 0.57 GB |
| 12v12v12 (goal) | 36 | 116.2 kbps | 4.18 Mbit/s | 1.88 GB |
| 24v24 (goal, design point 48) | 48 | 146.1 kbps | 7.01 Mbit/s | 3.16 GB (R2: about 3.2) |

Interest management lowers these (R2-BUD-REC2's mark: mean ≤ 100 kbps with the filter); IPv6 adds 4.8 kbps per
client (R2). Ingress is free on Hetzner (R6-HOST-F17).

### Cost per room-hour (48 humans)
| Host | Compute per room-hour | Egress per room-hour | Basis |
|---|---|---|---|
| Hetzner CCX23, 8 rooms per VM (A-2) | 0.0173 EUR | 0 inside the 20 TB; 0.0032 EUR beyond | 0.1386 EUR/h ÷ 8; 3.16 GB × 1 EUR/TB |
| Hetzner CCX23, 4 rooms per VM (one per vCPU, R1-ECS-OQ4) | 0.0347 EUR | as above | 0.1386 ÷ 4 |
| Hetzner CX33, 8 rooms per VM (A-2, if the spike passes on shared vCPU) | 0.0018 EUR | as above | 0.0144 ÷ 8 |
| Fly.io performance-2x, 4 rooms (A-3) | not computed per hour (monthly price only, R6-HOST-F22) | 0.063 USD | 3.16 GB × 0.02 USD |
| GameLift c6a.large, 4 rooms (A-3), Ohio rate | 0.025 USD on demand, 0.0075 USD Spot | 0 (gen 6+) | 0.099 or 0.03 USD/h ÷ 4 |

A CCX23 running 8 rooms of 48 humans all month sends 8 × 3.16 GB × 730 h ≈ 18.4 TB, under its 20 TB (R6-HOST-F15):
on Hetzner, egress costs nothing extra even at full load. The same traffic on Fly.io would cost about 369 USD per
month per such host (18,431 GB × 0.02 USD).

### Monthly scenarios
**Start (VISION_V2 §3: two teams of 2–8, three teams of 6–18).** A-4: up to 2 rooms at once, 18 humans each,
playtests about 4 hours a day; servers stay on all month (R6-HOST-REC2).

| Line | Lean | Reference |
|---|---|---|
| Production game VM | CX33 8.99 EUR | CCX23 86.49 EUR |
| Staging VM for previews (R6-HOST-REC3) | CX33 8.99 EUR | CX33 8.99 EUR |
| Egress (2 × 0.57 GB × 4 h × 30 ≈ 137 GB) | 0 (inside 20 TB) | 0 |
| Client and previews on Cloudflare | 0 USD (free static) | 5 USD (Workers Paid, only if Worker code is needed) |
| Metrics, logs (Grafana Cloud Free) | 0 | 0 |
| Crash reports (Sentry Developer, limits not read) | 0 | 0 |
| Match records (R6-TEL; Cloudflare R2 free 10 GB, or Hetzner Object Storage) | 0 USD | 6.49 EUR |
| **Total** | **about 18 EUR + 0 USD** | **about 102 EUR + 5 USD** |

**Goal (12v12v12 or 24v24).** A-5: peak 20 rooms of 48 humans at once (960 players), average 6 rooms over the month;
the fleet is sized to the peak plus one VM for draining during deploys.

| Line | Fixed fleet (CCX23) | Fixed base + hourly peak VMs (A-6) | Shared CX33 fleet (if the spike passes) |
|---|---|---|---|
| Game VMs | 4 × 86.49 = 345.96 EUR | 2 full months (172.98) + 2 VMs × 6 h × 30 days × 0.1386 (49.90) = 222.88 EUR | 4 × 8.99 = 35.96 EUR |
| Egress (6 × 3.16 GB × 730 h ≈ 13.8 TB) | 0 (inside 4 × 20 TB) | 0 (inside 2 × 20 TB plus the peak VMs' share, if the quota is per VM: not read) | 0 |
| Staging VM | 8.99 EUR | 8.99 EUR | 8.99 EUR |
| Client hosting | 0–5 USD | 0–5 USD | 0–5 USD |
| Monitoring (Grafana Pro, A-7) | from 19 USD + usage | same | same |
| Crash reports (Sentry paid plan) | base price not read (R6-COST-OQ2) + 0.0003625 USD per error above the quota | same | same |
| Match records (R6-TEL) | under 1 USD (R2, F4) or 6.49 EUR (F5) | same | same |
| **Total (match records on Hetzner Object Storage, 6.49 EUR, included)** | **about 361 EUR + 19–24 USD + Sentry** | **about 238 EUR + 19–24 USD + Sentry** | **about 51 EUR + 19–24 USD + Sentry** |

The same goal on Fly.io would add about 276 USD of egress alone (13,823 GB × 0.02 USD); on GameLift the egress is 0 but
EU compute rates were not read.

**Scaling rule (arithmetic):** on CCX23 with 8 rooms per VM, each extra room of peak capacity costs 86.49 ÷ 8 ≈ 10.81
EUR per month; at 4 rooms per VM, 21.62 EUR.

## Options
- **K1. A fixed fleet sized to the peak:** simplest; pays for idle hours; nothing to automate.
- **K2. A fixed base plus VMs added by the hour for the evening peak:** cheaper (game VMs about 36 % less in the goal case: 222.88 against 345.96 EUR; the total about 34 % less);
  needs automation through the provider's API, a supervisor that drains, and a VM start-up time nobody has measured
  (R6-HOST-OQ2's spike can time it); hourly round-up (R6-HOST-F17) makes VMs per match wasteful.
- **K3. Shared-vCPU VMs:** about 9.6 times cheaper per VM; only if the tick p99 on shared hosts passes R2-ROOM-REC2's
  marks at several times of day (R6-HOST-REC2's spike).
- **K4. Per-GB egress hosts (Fly.io) or managed fleets (GameLift):** per-GB pricing at 3.16 GB per 48-player room-hour
  costs more than the whole Hetzner fleet (above); GameLift waives bandwidth but its EU rates and SDK were not read.

## Recommendations
- **R6-COST-REC1. Budget the start at about 18 EUR per month (lean) to about 102 EUR + 5 USD (reference), and the goal
  at about 240–360 EUR + about 20 USD per month plus Sentry's paid plan, on Hetzner with a fixed fleet (K1) first; move
  to K2 only once real concurrency data shows a clear peak, and to K3 only if the shared-vCPU spike passes.**
  *Reason:* compute dominates and is flat per VM (R6-HOST-F15); a full CCX23 stays inside its included 20 TB even at 8
  rooms of 48 all month (arithmetic over R2's 3.16 GB per room-hour and R6-HOST-F15), so egress, the cost that grows
  with players, is zero on Hetzner and 0.063 USD per room-hour on a per-GB host (R6-HOST-F22); observability and
  storage start free (F1, F4).
  *Rests on:* F1, F2, F3, F4, F5; R6-HOST-F5, F15, F16, F17, F22, F23; R2's worked bandwidth estimate and R2-ROOM-REC2
  (wave 1, cited); assumptions A-1 to A-7.
  *Serves:* §6.1 (rooms and private matches at the §3 sizes).
  *Spike must prove (proposed):* rooms per VM measured on CCX23 and CX33 at 48, 36 and 18 players (the number that
  replaces A-2); measured egress per 48-player match-hour within ±20 % of 3.16 GB with interest management off, and
  the figure with it on; the spike's own bill (a few hours of one or two VMs) under 5 EUR.
- **R6-COST-REC2. Put a cost line into every match record and a monthly bill check into operations.** Each match
  record (R6-TEL) carries its room-hours and bytes sent, so cost per match is a query, not a guess; a monthly check
  compares VM hours and traffic against the included 20 TB per VM.
  *Reason:* every number above rests on A-1 to A-7; the first real playtests replace them.
  *Rests on:* R6-HOST-F15, F17; R1-DATA-REC6 (match records, cited).
  *Serves:* §6.6 (the match record), §6.1.
  *Spike must prove (proposed):* 100 % of spike match records carry room-hours and bytes sent; the sum over a test
  day matches the server's own interface counters within 5 %.

## Open questions
- **R6-COST-OQ1. How many rooms fit one VM?** A-2 (8 rooms of 48 per 4-vCPU VM) is R2's proposal over a JavaScript
  figure (TL15). Decided by: R2-ROOM-REC2's spike on the reference server (R6-HOST-REC2).
- **R6-COST-OQ2. Sentry's and Grafana's real bills.** Sentry's base prices and the Developer plan's limits are on
  sentry.io (outside the allow list); Grafana's usage depends on the series count R6-OBS sets. Decided by: reading
  sentry.io/pricing with Mustafa, and R6-OBS's series estimate.
- **R6-COST-OQ3. How many players, and when?** A-4 and A-5 are placeholders; VISION_V2 gives sizes, not player counts.
  Decided by: playtest data (R6-COST-REC2) and Mustafa's plans.
- **R6-COST-OQ4. Is included traffic per VM, pooled, or prorated for VMs that run part of a month?** Not stated on the
  pages read (R6-HOST-F15, F17). Matters only for K2. Decided by: Hetzner's terms or the first invoice.
- **R6-COST-OQ5. VAT and currency.** All prices exclude VAT; the rate Mustafa pays and any exchange rate were not read.
  Decided by: Mustafa's billing address and the provider's invoice.

### Recommendations that hinge on a source not read
| Recommendation | Source | Reason not read |
|---|---|---|
| R6-COST-REC1 (Sentry line) | https://sentry.io/pricing/ | outside the allow list |
| R6-COST-REC1 (per-GB comparison with a large cloud) | https://cloud.google.com/vpc/network-pricing | content truncated to the title |

## Assumptions
- **A-1.** Every player is human and all are in view (R2's upper bound); bots cost no traffic.
- **A-2.** 8 rooms of 48 players per 4-vCPU VM (R2-ROOM-REC2's proposal); smaller rooms fit at least as many.
- **A-3.** 4 rooms of 48 per 2-vCPU or 4 GB instance on Fly.io or GameLift (the same 2 rooms per vCPU as A-2).
- **A-4.** Start: up to 2 rooms at once, 18 humans each, about 4 hours a day.
- **A-5.** Goal: peak 20 rooms of 48 humans; average 6 rooms over 730 hours.
- **A-6.** K2: 2 VMs always on, 2 more for 6 hours a day, 30 days.
- **A-7.** Monitoring moves to Grafana Pro at the goal; usage on top is not estimated until R6-OBS counts series and log
  volume.

## Sources read
1. https://grafana.com/pricing/ · Grafana Cloud pricing · no date (© 2026) · official
2. https://docs.sentry.io/pricing/ · Sentry pricing docs · no date · official
3. https://developers.cloudflare.com/r2/pricing/ · Cloudflare R2 pricing · Last updated Oct 26, 2026 · official
4. https://www.hetzner.com/storage/object-storage · Hetzner Object Storage · no date (© 2026) · official
(plus the R6-HOST sources cited by id)

## Sources not read
- https://sentry.io/pricing/ · outside the allow list
- https://cloud.google.com/vpc/network-pricing · content truncated (see R6-HOST)

## Cross-topic notes
- **R2:** at the goal, egress is not a cost driver on Hetzner (inside 20 TB per VM at full load); it is on per-GB hosts.
  Interest management's savings (R2-BUD-REC2) matter for players' links, not for this bill.
- **R1:** R1-ECS-OQ4's "one match per core" costs 0.0347 EUR per room-hour on CCX23, against 0.0173 at R2's 2 rooms per
  vCPU.
