# R6: Infrastructure

## 1. Header

- **Topic:** R6, infrastructure (`docs/V2_PHASE0_BRIEF.md` §4, row R6).
- **Author agent:** infra-engineer (wave 2 of `.checkpoints/v2-research/plan.md`: one owner agent per topic).
- **Date:** 2026-10-28 (every source below was read that day).
- **Status:** spot-checked (19 of 40 key claims).
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
plus `docs/LOOP_V2.md` as a project input, not counted), 20 sources not read.

**How to read this file.**
- Wave 1's findings and recommendations are cited by id (R1-…, R2-…), never copied as R6 findings. Legacy ids (CD2,
  TL13, …) are rows of `docs/LEGACY_INVENTORY.md`; the inventory's marks are working recommendations (the Advisor's
  R7-MARKS (a)).
- Inside a question's section (and its pointers), a bare id (F12, REC3, OQ2) is that question's id: in R6-OBS's
  text, F8 means R6-OBS-F8.
- Assumptions are numbered A-1 to A-11 across the questions and listed under each question's options.
- Prices are research, not a commitment: no account was created and no service configured. EUR and USD stay apart (no
  exchange rate was read). Every price excludes VAT unless the source says otherwise.
- No spike was run and no code was written. Every pass mark and reference machine is proposed; nothing here is
  decided, and nothing here touches the look.

## 2. TL;DR
- Client and per-PR previews: Cloudflare (Workers static assets, or Pages) can send COOP and COEP from a `_headers` file (inferred: it takes any header), gives every branch its own link with no cap, and charges nothing for static traffic; GitHub Pages cannot send those headers.
- Game servers: Hetzner Cloud in one EU location, one room per process on a dedicated-vCPU CCX23 (proposed as R2's reference server), with the 9.6 times cheaper shared CX33 measured beside it; no load balancer on the UDP path; one wildcard certificate through ACME.
- Cost: about 18 EUR a month (lean) to 102 EUR + 5 USD at the start; about 240–360 EUR + about 20 USD plus Sentry at the goal; even 8 full 48-player rooms stay inside one VM's included 20 TB of traffic.
- Operations: `tracing` JSON logs, Prometheus-format metrics with bounded labels (tick overruns, fallback share), Sentry with Wasm debug files in its EU region with IP collection off, Reporting API crash reports; all data in the EU; the law itself was not read (outside the allow list).
- Telemetry: one record per match written at Ended (content hash, mode id, a 10 s points timeline, tick-stamped objective and kill events with source ids, unit time per slot, moments, no player ids) in EU object storage, analysed in batches per content hash. The security baseline is deferred (R6-SEC-OQ1).

## 3. Findings

Ids are global (R6-HOST-F1 and so on). Every source was read on 2026-10-28; prices are as published that day, excluding VAT unless said. "Confirmed in the source" means the quoted words were returned verbatim by the fetch; "inferred" means the point came from the fetch's own summary or from arithmetic.

### 1. Hosting for previews and EU game servers (R6-HOST)

#### Static client hosting and per-PR previews

- **R6-HOST-F1.** Cloudflare Pages sets custom response headers from a `_headers` file in the build output, at most 100
  rules, 2,000 characters per line. Source: https://developers.cloudflare.com/pages/configuration/headers/ ("Last updated
  Aug 25, 2026") · official · read 2026-10-28 · confirmed in the source: "creating a plain text file called `_headers`
  without a file extension, in the static asset directory of your project."; "You may define up to 100 header rules.";
  "Each line in the `_headers` file has a 2,000 character limit." That COOP and COEP can be set this way is inferred: the
  mechanism takes any header name; the page does not name COOP or COEP.
- **R6-HOST-F2.** On Cloudflare (Pages and Workers), `_headers` does not apply to responses generated by code (Pages
  Functions or a Worker script); those set their own headers. Sources: the Pages headers page (F1) and
  https://developers.cloudflare.com/workers/static-assets/headers/ ("Last updated Sep 22, 2026") · official · read
  2026-10-28 · confirmed in the source: "Custom headers defined in the `_headers` file are not applied to responses
  generated by Pages Functions."; "Custom headers defined in the `_headers` file are not applied to responses generated
  by your Worker code." The Workers page states the same 100-rule and 2,000-character limits.
- **R6-HOST-F3.** Cloudflare Pages limits: 20,000 files per site on the Free plan (100,000 on paid plans), 25 MiB per
  asset, unlimited active preview deployments; builds per month 500 (Free), 5,000 (Pro), 20,000 (Business). Source:
  https://developers.cloudflare.com/pages/platform/limits/ ("Last updated Sep 5, 2026") · official · read 2026-10-28 ·
  confirmed in the source: "Cloudflare Pages sites can contain up to 20,000 files on the Free plan."; "The maximum file
  size for a single Cloudflare Pages site asset is 25 MiB."; "You can have an unlimited number of preview deployments
  active on your project at a time."; "Paid plans (such as Pro, Business, and Enterprise plans) can have up to 100,000
  files per site." The build counts are confirmed as table cells ("Builds per month": 500 / 5,000 / 20,000, the
  verifier's re-read).
- **R6-HOST-F4.** Cloudflare Pages gives each branch a hash URL plus a branch alias that follows the branch's newest
  commit, and these URLs are public by default; an access policy (Cloudflare Access) can limit previews to the account's
  members. Source: https://developers.cloudflare.com/pages/configuration/preview-deployments/ ("Last updated Oct 12,
  2026") · official · read 2026-10-28 · confirmed in the source: "Each new branch you create will receive a new,
  randomly-generated hash in front of your `pages.dev` subdomain."; "the alias always maps to the latest commit of that
  branch."; "By default, these deployment URLs are public." The access policy (Settings > General; it protects preview
  deployments only, not the production `*.pages.dev` or custom domains) is inferred (summary).
- **R6-HOST-F5.** Cloudflare Workers serves static assets with free, unlimited requests and no egress or bandwidth
  charge; the Workers Paid plan has a minimum of 5 USD per month per account; the Free plan allows 100,000 Worker
  requests per day. Source: https://developers.cloudflare.com/workers/platform/pricing/ ("Last updated Oct 2, 2026") ·
  official · read 2026-10-28 · confirmed in the source: "Requests to static assets are free and unlimited."; "There are
  no additional charges for data transfer (egress) or throughput (bandwidth)."; "minimum charge of $5 USD per month for
  an account."; "100,000 per day".
- **R6-HOST-F6.** Workers also has preview URLs per branch, and Cloudflare describes Workers as having a broader
  feature set than Pages; the page does not say Pages is deprecated. Source:
  https://developers.cloudflare.com/workers/static-assets/migration-guides/migrate-from-pages/ ("Last updated Oct 12,
  2026") · official · read 2026-10-28 · confirmed in the source: "Unlike Pages, Workers has a distinctly broader set of
  features available to it."; "Workers serves each Preview at `<preview-name>.<your-domain>`, and the Preview name
  defaults to the branch name."; "Add a `previews` block and enable `workers.dev` Preview URLs in your Wrangler
  configuration file."
- **R6-HOST-F7.** Netlify sets headers from `_headers` or `netlify.toml`, only on files it serves from its store (not
  on functions or proxied content), and the header set is the same for every deploy context, including deploy
  previews, unless a per-context build copies in another file. Source: https://docs.netlify.com/routing/headers/ ("Last
  updated: Dec 1, 2025") · official · read 2026-10-28 · confirmed in the source: "Save a plain text file called
  `_headers` to the" publish directory; "Custom headers apply only to files Netlify serves from our own backing store.";
  "global for all builds and cannot be scoped for specific branches or deploy contexts." Netlify's prices were not read
  (netlify.com is outside the allow list).
- **R6-HOST-F8.** Vercel sets custom response headers per path pattern in `vercel.json`. Source:
  https://vercel.com/docs/project-configuration/vercel-json (last_updated 2026-08-14) · official · read 2026-10-28 ·
  confirmed in the source: `source`: "A pattern that matches each incoming pathname (excluding querystring)."; `headers`:
  "A non-empty array of key/value pairs representing each response header."
- **R6-HOST-F9.** Vercel's free Hobby plan is for non-commercial, personal use only, includes 100 GB of Fast Data
  Transfer and 100 deployments per day; Pro costs 20 USD per developer seat per month. Source:
  https://vercel.com/docs/plans/hobby (last_updated 2026-09-14) · official · read 2026-10-28 · confirmed in the source:
  "the Hobby plan restricts users to non-commercial, personal use only."; "First 100 GB"; "Developer seats cost **$20
  per user / month**" (USD, excluding VAT).
- **R6-HOST-F10.** Vercel's Standard Protection, on all plans, puts every deployment except production domains behind
  Vercel Authentication (Vercel users with access rights). Password protection costs 20 USD per month per project on
  Pro and does not exist on Hobby. Source: https://vercel.com/docs/deployment-protection (last_updated 2026-09-15) ·
  official · read 2026-10-28 · confirmed in the source: "Standard Protection is available on all plans"; "It protects
  all domains except [production domains]"; "Restricts access to only Vercel users with suitable access rights";
  "Available on Pro for $20 per month per protected project ... Not available on Hobby".
- **R6-HOST-F11.** GitHub Pages: 1 GB per site, a soft limit of 100 GB of bandwidth per month, and not allowed as free
  hosting for an online business or SaaS; the limits page names no custom headers. Source:
  https://docs.github.com/en/pages/getting-started-with-github-pages/github-pages-limits (no date) · official · read
  2026-10-28 · confirmed in the source: "Published GitHub Pages sites may be no larger than 1 GB."; "soft bandwidth limit
  of 100 GB per month"; "is not intended for or allowed to be used as a free web-hosting service".
- **R6-HOST-F12.** GitHub Pages cannot set COOP and COEP: the community request has been open since 2022, and a reply
  in 2023 gave "No ETA". Source: https://github.com/orgs/community/discussions/13309 (opened Mar 23, 2022; still open,
  last comment Oct 25, 2026) · other (community forum; the replier's staff role is not shown) · read 2026-10-28 ·
  confirmed in the source: "the inability to set these headers in Github Pages is becoming a bigger and bigger
  problem"; "This is a scenario we would support with custom headers. No ETA at the moment as to when this feature will
  be available."

#### EU game servers

- **R6-HOST-F13.** Hetzner Cloud's EU locations are Falkenstein (`fsn1`) and Nuremberg (`nbg1`) in Germany and Helsinki
  (`hel1`) in Finland, one network zone (eu-central); the shared Arm (Ampere) and shared Intel/AMD types exist only
  there. Sources: https://docs.hetzner.com/cloud/general/locations (Last change 2021-11-03) and
  https://www.hetzner.com/cloud (no date, © 2026) · official · read 2026-10-28 · confirmed in the source: "DE
  Falkenstein `fsn1`", "DE Nuremberg `nbg1`", "FI Helsinki `hel1`"; "available to you in Germany, Finland, Singapore,
  and the USA." Which type runs where is inferred (the availability grid is flattened in the fetch). The verifier's
  re-read of the product pages found the location column "NBG1 HEL1" (no FSN1) for CX23, CX33, CX43, CAX21, CCX13,
  CCX23 and CCX33, so these plans may not be offered in Falkenstein (unverified for the same reason).
- **R6-HOST-F14.** Hetzner sells shared vCPU plans as suited to development and variable load, and dedicated vCPU plans
  for "highly predictable resources", naming gaming servers. Source: https://www.hetzner.com/cloud · official · read
  2026-10-28 · confirmed in the source: "Shared cloud plans are perfect for development and testing environments.";
  "They are designed for variable usage patterns rather than sustained high workloads."; "hardware resources are shared
  with "neighbors"."; "Dedicated cloud servers assign dedicated vCPUs to your project, ensuring unshared performance and
  highly predictable resources"; dedicated plans suit "gaming servers".
- **R6-HOST-F15.** Hetzner Cloud prices in the EU (EUR per month, excluding VAT, hourly rate in brackets, 20 TB of
  traffic included unless said): shared Intel/AMD CX23 2 vCPU / 4 GB 5.99 (0.0096), CX33 4 vCPU / 8 GB 8.99 (0.0144),
  CX43 8 vCPU / 16 GB 16.49 (0.0264); shared Arm CAX21 4 vCPU / 8 GB 10.99 (0.0176); shared AMD CPX32 4 vCPU / 8 GB
  35.99 (0.0577); dedicated AMD CCX13 2 vCPU / 8 GB 43.49 (0.0697), CCX23 4 vCPU / 16 GB 86.49 (0.1386), CCX33 8 vCPU /
  32 GB 138.99 (0.2227; 30 TB). The general-purpose tables sit under the heading "Price incl. IPv4". Sources:
  https://www.hetzner.com/cloud/cost-optimized, https://www.hetzner.com/cloud/regular-performance,
  https://www.hetzner.com/cloud/general-purpose (no dates, © 2026) · official · read 2026-10-28 · confirmed in the
  source as table cells (WebFetch rendered the tables; the verifier re-read them): CX33 "€ 8.99 per month, excl. VAT",
  "€ 0.0144 per hour, excl. VAT", "20 TB"; CCX23 "€ 86.49 per month, excl. VAT", "€ 0.1386 per hour, excl. VAT";
  CCX13 "€ 43.49 per month, excl. VAT"; CCX33 "30 TB", "€ 138.99"; CAX21 "€ 10.99"; CX23 "€ 5.99"; CX43 "€ 16.49".
- **R6-HOST-F16.** Hetzner charges for extra EU traffic per TB at 1.00 EUR, excluding VAT; the page's own sentence reads
  "per hour", which looks like a rendering error. Source: the three product pages of F15 · official · read 2026-10-28 ·
  confirmed in the source (the words): "For each additional TB at EU-Central locations we charge € 1.00 per hour, excl.
  VAT." The reading "1.00 EUR per extra TB" is inferred.
- **R6-HOST-F17.** Hetzner bills servers by the hour, rounded up, with a monthly cap, and bills only outgoing traffic,
  in 100 MB blocks. Source: https://docs.hetzner.com/cloud/billing/faq (Last change 2020-07-07) · official · read
  2026-10-28 · confirmed in the source: "Servers have both a monthly price cap and a price per hour."; "We always round
  up the hourly usage of a server."; "We only bill for outgoing traffic. Incoming and internal traffic is free."; "in
  blocks of 100MB".
- **R6-HOST-F18.** Hetzner Cloud Firewalls filter UDP as well as TCP, are stateful, drop all inbound traffic that no
  rule allows, cost nothing, and take up to 500 rules. Sources: https://docs.hetzner.com/cloud/firewalls/faq (Last change
  2020-03-08) and https://docs.hetzner.com/cloud/firewalls/overview (Last change 2021-12-23) · official · read
  2026-10-28 · confirmed in the source: "You can filter TCP, UDP, ICMP, ESP and GRE traffic."; "If you don't define any
  rules here, all inbound traffic will be dropped."; "Yes, our Cloud Firewalls are stateful"; "We do not charge for our
  Cloud Firewalls."; "Up to 500 (effective) rules per Firewall."
- **R6-HOST-F19.** A Hetzner server gets its public addresses as Primary IPs; IPv6 is free; a 2022 page prices IPv4 at
  0.50 EUR per month. Source: https://docs.hetzner.com/cloud/servers/primary-ips/overview (2022-06-28) · official · read
  2026-10-28 · confirmed in the source: "We charge € 0.50 / $ 0.60 per month (excl. VAT) for our Primary IPv4s."; "We do
  not charge for our Primary IPv6s." Whether 0.50 EUR is current is unverified (the 2026 product tables say "Price incl.
  IPv4").
- **R6-HOST-F20.** On 2026-10-28 every EU plan on Hetzner's three product pages showed "This product is currently
  unavailable. Please check back later." Source: the three product pages of F15 · official · read 2026-10-28 · confirmed
  in the source (the words); whether it means a real shortage or a page that fills availability by script is unverified.
- **R6-HOST-F21.** Hetzner's load balancer overview names no protocols, so UDP support behind a Hetzner load balancer is
  unverified. Source: https://docs.hetzner.com/cloud/load-balancers/overview (Last change 2021-12-23) · official · read
  2026-10-28 · inferred (absence of a statement).
- **R6-HOST-F22.** Fly.io prices (USD per month, Ashburn; hourly rate in brackets): performance-1x 2 GB 33.00 (0.0458),
  performance-2x 4 GB 66.00 (0.0917), performance-4x 8 GB 132.01 (0.1833); egress in North America and Europe 0.02 USD per GB; a dedicated IPv4 2.00 USD per month.
  Source: https://fly.io/pricing/ (no date) · official · read 2026-10-28 · confirmed in the source: "Egress, North
  America and Europe | $0.02 / GB"; "Dedicated IPv4 | $2.00 / mo"; "| performance-1x · 2GB | $0.0458 | $33.00 |";
  "| performance-2x · 4GB | $0.0917 | $66.00 |"; "| performance-4x · 8GB | $0.1833 | $132.01 |" (hourly rates from
  the verifier's re-read). Fly.io's UDP page was
  not read (it redirects to docs.fly.io, outside the allow list).
- **R6-HOST-F23.** Amazon GameLift Servers charges by instance hour only: network bandwidth is free on instance
  generation 6 and later. Its example rate for a c6a.large (2 vCPU, 4 GB) is 0.099 USD per hour on demand and 0.03 USD
  on Spot, in US East (Ohio); EU rates are not on the page. Source: https://aws.amazon.com/gamelift/servers/pricing/ (no
  date, © 2026) · official · read 2026-10-28 · confirmed in the source: "Network bandwidth is free for all instance
  types generation 6 and later."; "c6a.large (1 core, 2 vCPU, 4 GB) @ $.099 OD /$.03 Spot".
- **R6-HOST-F24.** ACME (RFC 8555) automates certificate issuance; an order may name a wildcard domain; the dns-01
  challenge proves control with a TXT record at `_acme-challenge.<name>`, and http-01 needs TCP port 80 on the host.
  Source: https://www.rfc-editor.org/rfc/rfc8555.html (RFC 8555, March 2019), abstract, §7.1.3, §8.3, §8.4 · official
  (IETF standard) · read 2026-10-28 · confirmed in the source: "describes a protocol that a CA and an applicant can use
  to automate the process of verification"; "Any identifier of type "dns" in a newOrder request MAY have a wildcard
  domain name as its value."; "prepending the label "_acme-challenge" to the domain name being validated"; "to TCP port
  80 on the HTTP server."

#### Legacy facts used
- **CD2 (KVStore):** legacy keeps everything in the browser (local storage or memory); there is no server, so v2's
  hosting starts from zero: no existing host, domain or certificate to migrate.
- **TL9 (smoke test):** today's gate fails on any network request or WebSocket. A v2 preview that talks to a game server
  needs the gate v2 (R5) to allow the preview's own origins; the hosting choice must give a stable origin list.
- **TL15 (perfbots):** 48 soldiers cost 0.80–1.00 ms per legacy JavaScript step: the only CPU number for sizing a
  48-player room before the spike. R2-BUD-REC6 uses the same 0.80–1.00 ms figure but cites it as "legacy CM4"
  (r2-netcode.md:2382), which in `docs/LEGACY_INVENTORY.md` is the DM_Tri map row; the measurement is TL15's, so the id
  slip is R2's.

### 2. Costs (R6-COST)

- **R6-COST-F1.** Grafana Cloud Free: 10k active metric series, 50 GB of logs and 50 GB of traces ingested per month,
  14 days of retention, 3 active users, 50k frontend sessions per month. Source: https://grafana.com/pricing/ (no date,
  © 2026; the first fetch returned HTTP 502, the second worked) · official · read 2026-10-28 · confirmed in the source:
  "Limited to 10k active series per month"; "Limited to 50 GB ingested per month" (logs, and the same for traces);
  "Limited to 3 active users per month"; "Limited to 50k sessions per month"; "14 days retention" (metrics), "14 day
  retention" (logs, traces), the last two from the verifier's re-read.
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
  month."; "Storage: € 0.0087 per hour, excl. VAT per TB-hour."; "This base price includes 1 TB of storage (up to 744
  TB-hours) and 1 TB of egress traffic."; "Falkenstein, DE (FSN1), Helsinki, FI (HEL1), and Nuremberg, DE (NBG1)." (the
  last two from the verifier's re-read).

Prices from R6-HOST used here (not repeated as findings): Hetzner servers R6-HOST-F15 (CX33 8.99 EUR/mo, 0.0144/h;
CCX23 86.49 EUR/mo, 0.1386/h; 20 TB traffic included), extra traffic R6-HOST-F16 (1 EUR per TB, inferred reading),
hourly billing rounded up R6-HOST-F17, IPv4 R6-HOST-F19; Cloudflare static hosting R6-HOST-F5 (free static requests,
no egress charge; Workers Paid minimum 5 USD); Fly.io R6-HOST-F22 (0.02 USD per GB egress); GameLift R6-HOST-F23
(bandwidth free on gen 6+, c6a.large 0.099 USD/h on demand, Ohio).

#### Legacy facts used
- **TL15 (perfbots):** 48 soldiers at 0.80–1.00 ms per legacy JavaScript step. It is the only CPU figure for a
  48-player room; R2-ROOM-REC2's "8 rooms on a 4-vCPU VM" is a proposal built on it, not a measurement.
- **CD2 (KVStore):** legacy stores nothing on a server, so today's hosting cost is zero; every line below is new.

### 3. Logs, metrics and crash reports (R6-OBS)

#### Rust server
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

#### Browser client
- **R6-OBS-F7.** Sentry symbolicates WebAssembly crashes from DWARF debug files that carry a `build_id` custom section
  (added by its `wasm-split` tool), uploaded to Sentry or served from a symbol server; the browser side uses
  `@sentry/wasm`'s `wasmIntegration` with `@sentry/browser`; Sentry calls the feature "still developing". Source:
  https://docs.sentry.io/platforms/native/guides/wasm/ (no date) · official · read 2026-10-28 · confirmed in the source:
  "we require debug information files to be augmented by a `build_id` custom section."; "upload your WebAssembly debug
  information files to Sentry or use a symbol server."; "We provide a tool called wasm-split that can automatically add
  this section if missing."; "use the `@sentry/wasm` package and apply its `wasmIntegration`"; "This feature is still
  developing, so may be rough around the edges." (The JavaScript Wasm integration page returned 404.) The word "DWARF"
  is a paraphrase: it sits in a link the fetch dropped ("if your compiler can output" [link]).
- **R6-OBS-F8.** The Reporting API (Baseline 2026, newly available since March 2026) sends reports to endpoints named
  in a `Reporting-Endpoints` header; its types include `crash`, `coep`, `coop` and `csp-violation`; crash reports
  cannot be observed in JavaScript because the page has crashed, but they reach an endpoint. Source:
  https://developer.mozilla.org/en-US/docs/Web/API/Reporting_API (last modified Apr 21, 2026) · official · read
  2026-10-28 · confirmed in the source: "The Reporting API provides a generic reporting mechanism for web applications
  to use"; "The Reporting-Endpoints HTTP header is used to specify the name and URL for different endpoints"; "Since
  March 2026, this feature works across the latest devices and browser versions."; "Some parts of this feature may have
  varying levels of support."; "except for `crash` reports, which can't be observed in JavaScript (because the
  observing page has crashed)". The type list is inferred (table extracted by the fetch).

#### Data protection and where data lives
- **R6-OBS-F9.** Sentry stores event data in the US or the EU (Frankfurt), chosen when the organization is created and
  never changeable; some data (user accounts, organization settings) may be stored in the US whatever the choice.
  Source: https://docs.sentry.io/organization/data-storage-location/ (no date) · official · read 2026-10-28 · confirmed
  in the source: "Sentry supports data storage in both the US and the EU."; "Frankfurt, Germany."; "once selected, your
  data storage location can’t be changed."; "Here’s a list of the types of data that may be stored in the US,
  regardless of your selected Data Storage Location."; the list, from the verifier's re-read: "User accounts,
  notification settings, and 2FA authenticators"; "Organization settings, configurations, and teams"; "Access tokens
  for users and organizations"; "DSN keys"; "Cron check-ins"; "Detailed usage data".
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

#### Legacy facts used
- **GP5 and PG13 (dev panel perf overlay):** legacy measures frame time, draw calls and heap only on the player's own
  screen and sends nothing anywhere; v2 has no field data to start from.
- **TL15 (perfbots):** p50, p95 and max per step, the same shape v2's tick histograms take (R1-TICK-REC4, R2-BUD-REC6).
- **TL9 (smoke test):** fails on any network request; once v2 reports errors, the gate v2 (R5) must allow the report
  endpoints in the test build or switch reporting off there.

### 4. Balance telemetry, VISION_V2 §6.6 (R6-TEL)

- **R6-TEL-F1.** The `parquet` crate 60.1.0 is the Apache Parquet implementation in Rust; most users write through
  `ArrowWriter`; compression codecs come as optional features. Source: https://docs.rs/parquet/latest/parquet/ (parquet
  60.1.0, the page shows "28 October 2026") · official (crate docs) · read 2026-10-28 · confirmed in the source:
  "Apache Parquet implementation in Rust"; "Most users will use" `ArrowWriter`. The codec features (brotli, flate2,
  lz4_flex, snap, zstd) are inferred from the dependency list.
- **R6-TEL-F2 (design input, not an outside source).** What §6.6 and §6.8 ask, in LOOP_V2's rules: points come from
  uplink cashouts (the most), harvester deliveries, kills and structure kills; an HQ kill is "a big points swing"; first
  to the target wins, else the leader at about 20 minutes (LOOP_V2 §8); an uplink is stolen by holding E, and "The
  countdown keeps running, now for the new owner" (LOOP_V2 §4); v0's evidence already names the counters "cashouts,
  steals, deliveries, harvester kills" (LOOP_V2 §11). Source: docs/LOOP_V2.md §4, §8, §11 · project design doc · read
  2026-10-28 · confirmed in the source (quotes as given).

Storage prices used: R6-COST-F4 (Cloudflare R2: 0.015 USD per GB-month, 10 GB free) and R6-COST-F5 (Hetzner Object
Storage: 6.49 EUR per month with 1 TB, EU locations). Log hygiene: R6-OBS-F12 (OWASP).

Wave 1 this builds on (cited, not repeated): R1-DATA-REC6 (every match record stores the content hash and the mode
id), R1-ABL-REC5 (pick and win rates keyed by validated unit, weapon and ability ids), R1-ECS-REC3 (stable entity ids,
so events name who did what, for §6.6 and §6.8), R1-TICK-REC3 (telemetry sampling on a cadence in whole ticks),
R2-ROOM-REC1 (results and telemetry written at Ended, before Closed, in 100 % of normal ends), R2-BUD-REC2 (gameplay
events stamped with ticks, sent until acked), R2-ROOM-OQ7 (whether private matches count: a tag on the record).

#### Legacy facts used
- **TL13 (sim.js):** the headless match already reports a kill matrix, deliveries, trucks and buildings lost, tech and
  silo captures, era per team and frags per slot over 90–300 s: the prototype of v2's match record, and proof that
  "x3 runs, never one" (CLAUDE.md) applies to balance numbers too (TL13's evidence: bot stall % of 13.0 / 5.7 / 7.3 in three
  runs of one build against 3.4 / 16.1 / 6.5 of the other, the same mean of 8.67: noise).
- **GE15 (career):** per-weapon kills credit the weapon in hand when the kill is processed, so a grenade kill counts
  for the held gun. v2's damage events must carry their source id (R1-ABL-REC5, R1-ECS-REC3) or unit and weapon rates
  are wrong from day one.
- **GC18 (combat leftovers):** last-attacker fields are written but never read, and assists are never counted. Assists
  and "who contributed to the steal" need damage contribution kept per victim, which legacy never did.
- **GP11 and GE12:** the scoreboard shows only kills and deaths and the win rule is interim (HQ elimination); v2's
  record follows LOOP_V2 §8's points race instead.
- **CD2 (KVStore):** career stats live only in the browser; nothing is collected today.

### 5. Security baseline, deferred (R6-SEC)

Not researched: deferred by the Advisor on 2026-10-27. No findings (see R6-SEC-OQ1).

## 4. Options

### 1. Hosting for previews and EU game servers (R6-HOST)

#### Q1. Static client and per-PR previews

| Option | COOP/COEP | Previews | Privacy of previews | Cost and limits | Risk |
|---|---|---|---|---|---|
| A. Cloudflare Workers static assets (or Pages) | Yes, `_headers` (F1, F2; COOP/COEP by name inferred) | Per branch (F4, F6), unlimited (F3) | Public by default; Cloudflare Access can limit them (F4, inferred) | Static requests free, no egress charge (F5); 25 MiB per file, 20,000 files (F3, Pages figures) | Code-generated responses need headers set in code (F2) |
| B. Netlify | Yes, `_headers` (F7; COOP/COEP by name inferred) | Deploy previews (inferred from F7) | Not read | Prices not read (outside allow list) | Headers cannot differ per context without a build trick (F7): G3 (isolated previews, not production) needs that trick |
| C. Vercel | Yes, `vercel.json` (F8; COOP/COEP by name inferred) | Yes; protected by default on all plans (F10) | Vercel Authentication: viewers need a Vercel account (F10) | Hobby is non-commercial only (F9); Pro 20 USD per seat per month | Commercial use forces Pro |
| D. GitHub Pages | No (F11, F12) | Not per PR natively (not read) | Public | 1 GB, 100 GB per month soft (F11) | Cannot meet R1-BND-REC5's G3 (isolation in previews) |
| E. Serve the client from the game VMs (our own web server) | Yes (our server) | Our own work | Our own work | VM traffic (F15, F16) | Every preview, cache and certificate is ours to run: generic work no §6 item asks for |

#### Q2. EU game servers (one room per process, R2-ROOM-REC2)

| Option | UDP and ports | Fair tick (§4) | Cost basis | Start-up and scaling | Risk |
|---|---|---|---|---|---|
| G1. Hetzner Cloud, dedicated vCPU (CCX) | Own public IPv4/IPv6 per VM; firewall filters UDP (F18, F19); no load balancer needed, since the lobby hands out host:port (R2-ROOM-REC2) | "highly predictable resources" (F14) | CCX23 4 vCPU 86.49 EUR/mo, 20 TB included, 1 EUR per extra TB (F15, F16) | Hourly billing, rounded up (F17): keep VMs warm, rooms are processes | EU availability on 2026-10-28 unclear (F20); single provider |
| G2. Hetzner Cloud, shared vCPU (CX/CAX) | Same as G1 | Shared with neighbours (F14): tick p99 must be measured | CX33 4 vCPU 8.99 EUR/mo (F15) | Same as G1 | Noisy neighbours could break the tick budget |
| G3. Fly.io Machines | UDP not read (docs outside allow list) | performance types (F22), dedicated-ness not read | performance-2x 66 USD/mo + 0.02 USD/GB egress (F22) | Not read | 3.2 GB per 48-player match-hour (R2) at 0.02 USD/GB adds up (R6-COST) |
| G4. Amazon GameLift Servers | Managed fleets (not read in detail) | Instance types | 0.099 USD/h c6a.large on demand, Ohio; bandwidth free on gen 6+ (F23); EU rates not read | Managed scaling (not read) | Vendor lock-in to its server SDK (not read) |

#### Q3. Production certificates for game servers (R2-TRN-REC5, R2-TRN-OQ4)
- **C1. One certificate per host via ACME http-01:** needs TCP 80 open on each game VM (F24).
- **C2. One wildcard certificate per region name (for example `*.eu.<domain>`) via ACME dns-01:** one renewal job, the
  same certificate and private key copied to every VM (F24); the DNS provider's API key must live with that job (a
  secret: Mustafa adds it; key handling belongs to the deferred baseline). R2-TRN-REC5 asks for a certificate "per game
  server hostname": C2 departs from that (R6-HOST-REC4 says so).
- **C3. Self-signed, hash-pinned, short-lived servers:** R2-TRN-REC5's development and preview path; for production
  only "if R6 picks short-lived ephemeral servers". G1/G2 keep VMs warm (F17), so they are not ephemeral.

#### Q4. The starting region
- Nuremberg (Germany) or Helsinki (Finland), in Hetzner's one EU zone (F13); Falkenstein (Germany) only if the plans
  of R6-HOST-REC2 are offered there (their product pages list "NBG1 HEL1", F13). No latency source was read for any of
  them: the choice is a measurement.

### 2. Costs (R6-COST)

#### Cost arithmetic
##### Egress per match-hour (from R2's worked estimate)
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

##### Cost per room-hour (48 humans)
| Host | Compute per room-hour | Egress per room-hour | Basis |
|---|---|---|---|
| Hetzner CCX23, 8 rooms per VM (A-2) | 0.0173 EUR | 0 inside the 20 TB; 0.0032 EUR beyond | 0.1386 EUR/h ÷ 8; 3.16 GB × 1 EUR/TB |
| Hetzner CCX23, 4 rooms per VM (one per vCPU, R1-ECS-OQ4) | 0.0347 EUR | as above | 0.1386 ÷ 4 |
| Hetzner CX33, 8 rooms per VM (A-2, if the spike passes on shared vCPU) | 0.0018 EUR | as above | 0.0144 ÷ 8 |
| Fly.io performance-2x, 4 rooms (A-3) | 0.0229 USD | 0.063 USD | 0.0917 USD/h ÷ 4 (R6-HOST-F22); 3.16 GB × 0.02 USD |
| GameLift c6a.large, 4 rooms (A-3), Ohio rate | 0.025 USD on demand, 0.0075 USD Spot | 0 (gen 6+) | 0.099 or 0.03 USD/h ÷ 4 |

A CCX23 running 8 rooms of 48 humans all month sends 8 × 3.156 GB × 730 h ≈ 18.4 TB, under its 20 TB (R6-HOST-F15;
3.16 in the table is 3.156 rounded, and the GB products here and below use 3.156):
on Hetzner, egress costs nothing extra even at full load. The same traffic on Fly.io would cost about 369 USD per
month per such host (18,431 GB × 0.02 USD).

##### Monthly scenarios
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

- **K1. A fixed fleet sized to the peak:** simplest; pays for idle hours; nothing to automate.
- **K2. A fixed base plus VMs added by the hour for the evening peak:** cheaper (game VMs about 36 % less in the goal case: 222.88 against 345.96 EUR; the total about 34 % less);
  needs automation through the provider's API, a supervisor that drains, and a VM start-up time nobody has measured
  (R6-HOST-OQ2's spike can time it); hourly round-up (R6-HOST-F17) makes VMs per match wasteful.
- **K3. Shared-vCPU VMs:** about 9.6 times cheaper per VM; only if the tick p99 on shared hosts passes R2-ROOM-REC2's
  marks at several times of day (R6-HOST-REC2's spike).
- **K4. Per-GB egress hosts (Fly.io) or managed fleets (GameLift):** per-GB pricing at 3.16 GB per 48-player room-hour
  costs more in egress per fully loaded host than the host itself costs on Hetzner (about 369 USD a month against 86.49
  EUR for a CCX23, above); at the goal's average load, Fly's egress alone (about 276 USD) is of the same order as the
  whole Hetzner fleet (222.88–345.96 EUR); no exchange rate was read, so the two are not compared further. GameLift
  waives bandwidth but its EU rates and SDK were not read.

#### Assumptions
- **A-1.** Every player is human and all are in view (R2's upper bound); bots cost no traffic.
- **A-2.** 8 rooms of 48 players per 4-vCPU VM (R2-ROOM-REC2's proposal); smaller rooms fit at least as many.
- **A-3.** 4 rooms of 48 per 2-vCPU or 4 GB instance on Fly.io or GameLift (the same 2 rooms per vCPU as A-2).
- **A-4.** Start: up to 2 rooms at once, 18 humans each, about 4 hours a day.
- **A-5.** Goal: peak 20 rooms of 48 humans; average 6 rooms over 730 hours.
- **A-6.** K2: 2 VMs always on, 2 more for 6 hours a day, 30 days.
- **A-7.** Monitoring moves to Grafana Pro at the goal; usage on top is not estimated until R6-OBS counts series and log
  volume.

### 3. Logs, metrics and crash reports (R6-OBS)

#### Q1. Server logs
- **L1. `tracing` with JSON lines to standard output per room process** (F1, F2); the supervisor tags each line with
  VM and room and forwards it. Low effort; any log store reads JSON lines.
- **L2. OpenTelemetry logs** (Stable, F3) over OTLP: one protocol for logs and metrics; more crates, and nothing in §6
  needs the cross-service view it adds.

#### Q2. Server metrics
| Option | What | Cost | Risk |
|---|---|---|---|
| M1. `metrics` + Prometheus exporter, scraped by a self-hosted Prometheus on the staging VM | F4, F5 | 0 extra (staging VM, R6-HOST-REC3); disk about 0.6 GB at A-8 | Not replicated (F5): metrics lost with the VM, not the game |
| M2. Same exporter, stored in Grafana Cloud (EU stack) | F4, F11, R6-COST-F1 | Free under 10k series | How scraped data reaches Grafana Cloud was not read (R6-OBS-OQ2) |
| M3. OpenTelemetry metrics (Stable, F3) over OTLP | F3 | as M2 | More dependencies in the room binary |

#### Q3. Traces
- **T1. Spans stay local:** tick phases (simulation, lag compensation, encoding) are timed with `tracing` spans or
  plain timers and exported as histograms (M1/M2). Serves R2-BUD-REC6's separate measurements.
- **T2. Distributed tracing with OpenTelemetry:** Beta in Rust (F3), and one room process has no cross-service request
  path to follow: a generic feature no §6 item asks for.

#### Q4. Client errors and crashes
| Option | Catches | Wasm frames | Privacy | Cost |
|---|---|---|---|---|
| C1. Sentry browser SDK + `wasmIntegration`, debug files uploaded at build | JS errors, Wasm traps with symbols | Yes, "still developing" (F7) | EU storage at creation (F9); IP off by setting (F10) | Free tier, then per error (R6-COST-F3) |
| C2. Reporting API `crash` (and later `coep`, `coop`, `csp-violation`) to our own endpoint | Tab crashes JS cannot see (F8) | No stack | Our own store, EU | 0 (our server) |
| C3. Our own `error` and `unhandledrejection` handler posting to our endpoint | JS errors; Wasm traps as raw offsets | Offsets only, symbolicated by us | Our own | Our work |

#### Q5. Data protection
- **P1. EU-only storage for every store** (Hetzner EU, Sentry EU, a Grafana EU stack; F9, F11); no IP addresses,
  tokens or guest secrets in logs or reports (F10, F12; R2-ROOM-REC3 mark 3); bounded retention.
- **P2. A provider's default region and settings:** US storage for Sentry unless chosen at creation (F9), IP sent by
  default (F10).

#### Assumptions
- **A-8.** Series per VM: about 50 per room slot (3 histograms of about 14 series each, plus about 8 counters and
  gauges) × 8 slots, plus about 500 host series (assumption) ≈ 900; 4 VMs ≈ 3,600. At a 15 s scrape, 240 samples per
  second × 15 days × 2 B ≈ 0.6 GB on disk (F5's formula).
- **A-9.** Logs: about 300 B per line, about 200 lines per match (lifecycle, joins, leaves, warnings); at the goal's
  average of 6 rooms with 20-minute matches, about 13,000 matches and 0.8 GB per month, far under 50 GB.

### 4. Balance telemetry, VISION_V2 §6.6 (R6-TEL)

#### Q1. What a match record holds
- **W1. Results only** (winner, final points, duration): cheap, but answers none of "who led", "swings" or "steals".
- **W2. Results + a points timeline + objective and kill events + per-slot unit use (proposed).** Every §6.6 number is
  derivable; size estimated below (A-10).
- **W3. A full replay (every input or snapshot):** answers everything, including later kill-cams (§6.8 [LATER]), but
  is orders of magnitude larger and is a §6.8 [LATER] feature, so out of scope now.

#### Q2. How it is recorded on the server
- **R-a. The room's sim emits typed gameplay events (already needed for clients, R2-BUD-REC2); a recorder in the room
  process appends them to a bounded buffer and samples team state on a tick cadence (R1-TICK-REC3); at Ended it writes
  the record (R2-ROOM-REC1).** No new path through the sim.
- **R-b. A separate telemetry stream sent live to a collector during the match:** more moving parts, and a collector
  outage then loses data mid-match.

#### Q3. Storage and format
| Option | Format | Where | Cost at the goal (A-11) | Fit |
|---|---|---|---|---|
| S1. One compressed JSON-lines file per match in object storage | JSON lines | Hetzner Object Storage (EU, F5 of R6-COST) | 6.49 EUR/mo flat (1 TB included) | Simple; any tool reads it |
| S2. As S1, plus a periodic job that compacts into Parquet files for analysis | JSON lines + Parquet (F1) | same | same | Column queries over many matches; adds one job |
| S3. Cloudflare R2 instead of Hetzner | as S1/S2 | R2 (region and jurisdiction not read) | about 1 USD/mo after the free 10 GB | Cheap; EU location unverified |
| S4. A database (SQL) | rows | not read (postgresql.org, sqlite.org outside the allow list) | not computed | Needs a read first |
| S5. A hosted analytics warehouse (for example BigQuery) | tables | BigQuery's price page came back truncated | not read | Generic; not needed at this volume |

#### Q4. Analysis
- **N1. A batch script over the stored records** (weekly or on demand), grouped by content hash and mode id
  (R1-DATA-REC6), on the staging VM: computes the derived numbers in REC3.
- **N2. Live dashboards** of balance numbers: no §6 item asks for live balance data; the metrics in R6-OBS cover live
  health.

#### Assumptions
- **A-10.** A 20-minute 48-player match: about 10 gameplay events per second (R2's worked estimate uses the same rate)
  → 12,000 events × about 80 B as JSON ≈ 1 MB uncompressed; the timeline adds about 360 samples; compression is not
  estimated (no source).
- **A-11.** At the goal's average of 6 rooms (A-5) with 20-minute matches: about 13,000 matches a month (6 × 3 × 730),
  about 13 GB uncompressed a month at A-10's upper bound.

### 5. Security baseline, deferred (R6-SEC)

None (deferred).

## 5. Recommendations

Every pass mark and every reference machine below is proposed. "Serves" names the VISION_V2 §6 item (and the §3 or §4 target) each recommendation serves.

### 1. Hosting for previews and EU game servers (R6-HOST)

- **R6-HOST-REC1. Host the client and the per-PR previews on Cloudflare (Workers static assets; Pages as the
  same-headers alternative), and send COOP and COEP from `_headers` on previews when R1-BND-REC5's option G3 is on.**
  Previews stay harmless rather than secret: a preview holds no secrets, talks only to the staging game server (REC3),
  and is limited to the account with Cloudflare Access if Mustafa wants it private (R6-HOST-OQ5). Netlify (B) is the
  fallback with the same `_headers` file; Vercel (C, headers from `vercel.json`, F8) only on Pro, because Hobby is
  non-commercial (F9).
  *Reason:* its `_headers` file sets custom response headers, so it can send the isolation headers (F1, F2; COOP and
  COEP by name inferred), gives every branch its own link with no cap on previews (F3,
  F4, F6), charges nothing for static requests or egress (F5), and GitHub Pages cannot send the headers (F11, F12).
  *Rests on:* F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12; R1-BND-REC5 (option G3, wave 1, cited).
  *Serves:* §6.1 (rooms and private matches are reached from this client); §4 (desktop browsers first). The isolation
  headers serve R1-BND-REC5.
  *Spike must prove (proposed):* on one preview link: every HTML, JS, worker and Wasm response carries the configured
  headers (100 % of responses in a full page load, checked by a script); `crossOriginIsolated` is true in Chrome,
  Edge, Firefox and Safari when G3 is on and false with it off; the content bundle of R1-DATA-REC6 is served with an
  immutable cache header and a repeat visit fetches 0 bytes of it; the largest build file stays under 25 MiB (F3: the
  Pages per-asset limit; the Workers static-assets limit was not read, so the mark is rechecked against Workers' own
  limits page before the spike).
- **R6-HOST-REC2. Run game servers on Hetzner Cloud in one EU location, one room per process under a small supervisor
  per VM, with no load balancer on the game path. Name the reference server for R2 (R2-XU-OQ9): the dedicated CCX23
  (4 vCPU, 16 GB, x86-64 AMD), and measure the shared CX33 (4 vCPU, 8 GB) beside it.**
  Each VM has its own public IPv4 and IPv6; the lobby hands out host and port (R2-ROOM-REC2); the firewall opens one
  UDP range for WebTransport and one TCP range for the WebSocket fallback (R2-TRN-REC3), plus nothing else. VMs stay up
  (billing rounds up to the hour, F17), and rooms start and end as processes inside them. The server CPU is x86-64 to
  start (dedicated plans are AMD, F15), which answers R1-DET-OQ7 for the first fleet; Arm (CAX) stays an option only
  after R1-DET-REC6's matrix includes it. The pick is conditional on R6-HOST-OQ1: on 2026-10-28 every EU plan on
  Hetzner's product pages showed "This product is currently unavailable. Please check back later." (F20); until a
  console check shows capacity, Hetzner is the working pick, not a settled one, and OQ1 and OQ4 name what is read next
  if the shortage is real.
  *Reason:* dedicated vCPUs are sold for predictable load and gaming servers (F14), which the fair tick of §4 and
  R2-BUD-REC6 need; Hetzner includes 20 TB of egress per VM and charges 1 EUR per extra TB (F15, F16), far below
  per-GB pricing (F22), and its firewall handles UDP (F18). Shared CX33 is 9.6 times cheaper than CCX23 (8.99 against
  86.49 EUR, F15), so it is worth measuring, not assuming.
  *Rests on:* F13, F14, F15, F16, F17, F18, F19, F20 (the availability risk), F22; R2-BUD-REC6 (the per-tick budget the
  reason names), R2-ROOM-REC2 (host and port from the lobby; the spike's marks: 8 rooms, within 10 %, under 2 s,
  r2-netcode.md:2733–2738), R2-TRN-REC3 (the UDP and TCP ports), R1-DET-OQ7 and R1-DET-REC6 (the server CPU) (wave 1,
  cited); legacy TL15.
  *Serves:* §6.1 (rooms and private matches), §6.5 (economy server-side), §6.6 and §6.8 (recorded and recognized on
  the server); §4 (one EU region, fair to about 100 ms).
  *Spike must prove (proposed):* R2-ROOM-REC2's marks on both CCX23 and CX33: 8 rooms of 48 players (bots) as 8
  processes, each room's tick p99 within 10 % of the single-room figure, over 20 minutes, repeated at 3 times of day
  (shared hosts vary with neighbours); WebTransport over UDP and WebSocket over TCP reach the VM through the firewall
  from all four browsers; a room process starts in under 2 s (R2's mark); a VM with no rule for a port drops it
  (negative test).
- **R6-HOST-REC3. Previews get one shared staging game server, not one per PR.** One small VM (proposed CX33) runs a
  supervisor that starts the room binary of the build a preview names, with self-signed, pinned certificates as
  R2-TRN-REC5 says for previews; offline previews run the match in the worker with bots and need no server
  (R1-BND-REC6). Old binaries are deleted when their PR closes (R5's CI side).
  *Reason:* a VM per PR bills at least an hour each time (F17) and adds machines to run; previews only need a server
  that speaks the PR's protocol version.
  *Rests on:* F15, F17; R2-TRN-REC5, R1-BND-REC6 (wave 1, cited).
  *Serves:* §6.1 (rooms, tested per PR).
  *Spike must prove (proposed):* two previews of different builds play at the same time on one staging VM, each against
  its own binary; a closed PR's binary is gone within 24 h; a preview client never connects to a production server
  (its allowed origins and server list differ).
- **R6-HOST-REC4. Production certificates: one wildcard certificate for the game-server names (option C2), from ACME
  with dns-01, renewed by one job and copied to every game VM.** This departs from R2-TRN-REC5's wording, "a publicly
  trusted certificate per game server hostname" (r2-netcode.md:2667): every hostname is still covered by a publicly
  trusted certificate, but by one shared certificate, so one private key sits on every game VM (a key-handling item for
  the deferred baseline, R6-SEC-OQ1). Per-hostname certificates (C1, or dns-01 per name) stay the alternative if the
  baseline rejects a shared key. It needs no TCP 80 on game VMs.
  *Reason:* RFC 8555 allows wildcard orders and proves control through DNS (F24); one certificate for all VMs makes
  adding a VM a copy, not an issuance.
  *Rests on:* F24; R2-TRN-REC5 (the departure is stated above).
  *Serves:* §6.1; §4 (four browsers need a publicly trusted certificate in production, R2-TRN-REC5).
  *Spike must prove (proposed):* R2-TRN-REC5's marks (all four browsers, 20 of 20 connections) with the wildcard
  certificate; a forced renewal swaps the certificate on a VM with 48 live sessions and drops 0 of them; an alert fires
  when any VM's certificate has under 14 days left (proposed).

### 2. Costs (R6-COST)

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

### 3. Logs, metrics and crash reports (R6-OBS)

- **R6-OBS-REC1. Server logs: `tracing` with JSON lines on standard output per room process; the supervisor adds VM and
  room ids and ships them; no tokens, guest secrets or raw ids in any line (salted hashes where an id is needed, as
  R2-ROOM-REC3 mark 3 says).**
  *Reason:* structured events at little cost (F1, F2); OWASP's exclusions (F12) match R2's log rule.
  *Rests on:* F1, F2, F12; R2-ROOM-REC3 (cited).
  *Serves:* §4 ("Matches feel fair up to about 100 ms of ping": per-room logs show the tick overruns and room events
  behind an unfair match); §6.1 (per-room crash reports and lifecycle, R2-ROOM-REC1, R2-ROOM-OQ9).
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
  and retention capped: logs 14 days, metrics 15 days (proposed); the cap for crash reports is open, because no page
  read shows whether Sentry lets an organization set its retention (sentry.io is outside the allow list; R6-COST-OQ2's
  read decides).** The legal side (lawful basis, privacy notice, how long the law allows) is not researched here: R6-OBS-OQ1.
  *Reason:* the EU choice is one-way on both Sentry and Grafana (F9, F11); OWASP asks for bounded retention and no
  personal data in logs (F12); 14 and 15 days are the free tier's and Prometheus's defaults (R6-COST-F1, F5).
  *Rests on:* F5, F9, F10, F11, F12; R6-COST-F1.
  *Serves:* no §6 item asks for data protection as a feature; it is the condition under which the stores that §6.1
  (rooms) and §6.6 (match records) create may run, under rules Mustafa sets (R6-OBS-OQ1).
  *Spike must prove (proposed):* a data map lists every store, its region and its retention, with 0 stores outside the
  EU for event data; data older than the cap is gone on a check one day after the cap.

### 4. Balance telemetry, VISION_V2 §6.6 (R6-TEL)

- **R6-TEL-REC1. Every match writes one record (W2), recorded inside the room (R-a), with this shape (proposed):**
  - *Header:* random match id; record schema version; content hash and mode id (R1-DATA-REC6); server build id; map
    id; team count, sizes and faction per team; private flag (R2-ROOM-OQ7); start time (UTC, to the minute); duration;
    end reason (target or timer); winner and final points; humans and bots per team; room-hours and bytes sent
    (R6-COST-REC2).
  - *Timeline:* points, money and tech tier per team every 10 s of game time (R1-TICK-REC3's cadence; 120 samples per
    team in a 20-minute match).
  - *Events (tick-stamped):* uplink cache picked up, placed, stolen (from team, to team, countdown left), cashed out;
    harvester delivery and kill; structure and HQ kills; tech tier reached; kills (killer and victim slot, killer and
    victim unit id, source weapon or ability id, assisting slots); unit and vehicle purchases.
  - *Per slot:* human or bot, team, faction, time spent as each unit id (for pick rates), kills, deaths, assists.
  - *Moments (§6.8):* the server's recognized moments (for example a steal with under 10 s of countdown left, a
    comeback) stored as tagged events pointing at the events that made them, so the moment rules and the telemetry
    read the same stream.
  - *Never in it:* account or guest ids, display names, IP addresses, tokens (R6-OBS-F12; R2-ROOM-REC3 mark 3).
    Balance needs slots, not people.
  *Reason:* each §6.6 number (who led, swings, steals, unit pick and win rates) and each §6.8 moment is a function of
  the timeline and events; W1 cannot answer them and W3 is [LATER]; legacy shows the trap of untagged kill sources
  (GE15) and missing assists (GC18); recording inside the room at Ended reuses R2-ROOM-REC1's guarantee.
  *Rests on:* F2; R6-OBS-F12; R1-DATA-REC6, R1-ABL-REC5, R1-ECS-REC3, R1-TICK-REC3, R2-ROOM-REC1, R2-BUD-REC2 (cited);
  legacy TL13, GE15, GC18; A-10.
  *Serves:* §6.6 (balance telemetry), §6.8 (moments), §6.5 (economy numbers on evidence), §6.1 (mode id, private flag).
  *Spike must prove (proposed):* in 30 bot matches across 3 modes (2 teams, 3 teams, 48 players), 100 % write a record
  before Closed; every kill event names a source id that resolves in the content set of the record's hash; the points
  total recomputed from events equals the final points in 100 % of records; recording adds under 1 % to tick p99 and
  0 allocations per tick after warm-up (bounded buffer, preallocated); a record is at most 1 MB uncompressed (A-10).
- **R6-TEL-REC2. Store records as compressed JSON-lines files in EU object storage (S1, Hetzner Object Storage in the
  game servers' location), uploaded by the supervisor after the room exits, with a schema version in every record and
  a Parquet compaction job (S2, `parquet` 60.1.0) only once a query needs it.**
  *Reason:* at the goal the volume is about 13 GB a month uncompressed (A-11), well inside 1 TB for 6.49 EUR (R6-COST-F5);
  Hetzner keeps it in the EU next to the servers, while Cloudflare R2's location was not read (S3); Parquet is
  available in Rust when column queries are needed (F1). The supervisor uploads after exit, so a storage outage only
  delays uploads (records wait on the VM's disk). A room that crashes loses its in-memory buffer (R6-TEL-REC1 writes at
  Ended); the supervisor then writes a stub record marked incomplete with the header fields it holds (match id, build,
  mode, start time, exit reason), so crashed matches are counted, never silently missing (proposed).
  *Rests on:* F1; R6-COST-F4, R6-COST-F5; R6-HOST-REC2 (the supervisor); R6-TEL-REC1 (the buffer written at Ended);
  A-11.
  *Serves:* §6.6.
  *Spike must prove (proposed):* with object storage unreachable for 1 hour, 0 records are lost and all upload within
  10 minutes of recovery; a record of schema version n is read by the analysis script for version n+1 (one added
  field, old records still parse); a room killed with SIGKILL mid-match leaves a stub record marked incomplete, with
  its exit reason.
- **R6-TEL-REC3. Analyse in batches (N1), always grouped by content hash and mode id, with sample sizes shown next to
  every rate.** Derived numbers (proposed definitions, for the game-designer to refine): win rate per faction and per
  base position; unit pick rate (share of human player-time per unit id) and unit win rate (win rate of teams by
  player-time on that unit); lead changes per match (the leader in the 10 s points timeline changes); comeback rate
  (the winner trailed by at least a set share of the target at some point); steals per match and late steals (under
  10 s left); P(win | leading at minute 5 and at minute 10) as the snowball measure the healthy-snowball session
  (VISION_V2 §7) needs.
  *Reason:* pick and win rates are only comparable within one tuning version (R1-DATA-REC6); legacy's three-run spread
  (TL13) shows single samples mislead; VISION_V2 asks for "Snowball and counters get tuned on evidence" (§6.6), which
  these derived numbers make concrete.
  *Rests on:* F2; R1-DATA-REC6 (cited); legacy TL13; VISION_V2 §6.6, §7.
  *Serves:* §6.6.
  *Spike must prove (proposed):* the script reproduces, from 30 bot-match records, the hand-counted cashouts, steals,
  deliveries and harvester kills of 3 matches exactly (LOOP_V2 §11's counters); every output row carries its match
  count; a run over a month of goal-scale records (about 13,000, A-11) finishes in under 10 minutes on the staging VM.

### 5. Security baseline, deferred (R6-SEC)

None (deferred).

## 6. Open questions

### 1. Hosting for previews and EU game servers (R6-HOST)

- **R6-HOST-OQ1. Is Hetzner's EU capacity actually available?** Every EU plan showed "currently unavailable" on
  2026-10-28 (F20). Decided by: a check in the Hetzner console when Mustafa has an account (not created by agents); if
  it is a real shortage, Netcup, OVHcloud or Scaleway (all outside the allow list, not read) or GameLift (G4) need a
  read.
- **R6-HOST-OQ2. Which EU location gives the best ping across the player base?** No latency source was read. Decided
  by: a spike measuring RTT from Mustafa's machine and from playtesters to `nbg1` and `hel1` (and `fsn1` only if the
  chosen plans are offered there: their product pages list "NBG1 HEL1", F13), against §4's "about 100 ms".
- **R6-HOST-OQ3. Does a later fleet need a UDP load balancer?** Not with host:port from the lobby (R2-ROOM-REC2);
  Hetzner's load balancer page names no protocols (F21). Decided by: whether a later design routes many rooms through
  one address (R2-TRN-F39's connection IDs).
- **R6-HOST-OQ4. Fly.io and GameLift as fallbacks.** Fly's UDP page is outside the allow list; GameLift's EU prices and
  server SDK were not read (F23). Decided by: a read of those pages if OQ1 rules Hetzner out. Hetzner's dedicated root
  servers (www.hetzner.com, not read in this run) are a further candidate at goal scale; decided by a read of their price
  and location pages before the goal budget (R6-COST-REC1) is fixed.
- **R6-HOST-OQ5. Private or only harmless previews?** Cloudflare Access can limit previews to the account (F4,
  inferred); Mustafa's Brave would then need a Cloudflare login. Decided by: Mustafa (convenience against privacy),
  after the security baseline.
- **R6-HOST-OQ6. Does the wildcard certificate need dns-01, and how long will certificates live?** RFC 8555 leaves both
  to the CA (F24); Let's Encrypt's and the CA/Browser Forum's pages are outside the allow list. Decided by: a read of
  the chosen CA's policy. The verifier names a further option, ACME's tls-alpn-01 challenge (RFC 8737, not read in this
  run), as needing only TCP 443, which the WebSocket fallback may open anyway; decided by a read of RFC 8737 if
  per-hostname certificates (R6-HOST-REC4's alternative) are wanted.
- **R6-HOST-OQ7. What does a Hetzner IPv4 address cost today?** The 2022 page says 0.50 EUR per month (F19); the 2026
  tables say "Price incl. IPv4" (F15). Decided by: the price list at sign-up. Under 1 EUR per VM either way.

### 2. Costs (R6-COST)

- **R6-COST-OQ1. How many rooms fit one VM?** A-2 (8 rooms of 48 per 4-vCPU VM) is R2's proposal over a JavaScript
  figure (TL15). Decided by: R2-ROOM-REC2's spike on the reference server (R6-HOST-REC2).
- **R6-COST-OQ2. Sentry's and Grafana's real bills.** Sentry's base prices and the Developer plan's limits are on
  sentry.io (outside the allow list); Grafana's usage depends on the series count R6-OBS sets. Decided by: reading
  sentry.io/pricing with Mustafa (including whether Sentry's retention can be set, R6-OBS-REC4), and R6-OBS's series
  estimate.
- **R6-COST-OQ3. How many players, and when?** A-4 and A-5 are placeholders; VISION_V2 gives sizes, not player counts.
  Decided by: playtest data (R6-COST-REC2) and Mustafa's plans.
- **R6-COST-OQ4. Is included traffic per VM, pooled, or prorated for VMs that run part of a month?** Not stated on the
  pages read (R6-HOST-F15, F17). Matters only for K2. Decided by: Hetzner's terms or the first invoice.
- **R6-COST-OQ5. VAT and currency.** All prices exclude VAT; the rate Mustafa pays and any exchange rate were not read.
  Decided by: Mustafa's billing address and the provider's invoice.

### 3. Logs, metrics and crash reports (R6-OBS)

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

### 4. Balance telemetry, VISION_V2 §6.6 (R6-TEL)

- **R6-TEL-OQ1. How many matches before a rate means something?** No statistics source was read; the arXiv paper found
  in search argues win and pick rates are weak alone in team games (outside the allow list). Decided by: test-engineer
  and game-designer (confidence intervals per cell, and a minimum count) before the first tuning pass.
- **R6-TEL-OQ2. Do bot matches and private matches count?** Bots fill slots (VISION_V2 §3), so most early matches are
  part-bot; private matches are R2-ROOM-OQ7. The record tags both; whether rates include them is Mustafa's call.
- **R6-TEL-OQ3. Which moments does the server recognize, and with what thresholds?** "Last-second" (10 s here) and
  "comeback" need numbers. Decided by: Mustafa and the game-designer (§6.8).
- **R6-TEL-OQ4. How long are match records kept?** Without player identifiers they may not be personal data; that is
  the legal question in R6-OBS-OQ1. Decided by: Mustafa after that read; proposed default: kept while their content
  hash is still analysed.
- **R6-TEL-OQ5. How shipped games do it.** Riot's GDC talk on League of Legends balance (GDC Vault, video only) and
  public talks on telemetry pipelines were not read. Decided by: a transcript or paper on an allowed domain, if one
  turns up.

### 5. Security baseline, deferred (R6-SEC)

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
  Items R6 met on the way and left here: the DNS API key the wildcard certificate job needs (R6-HOST-REC4); the wildcard
  certificate's private key, copied to every game VM (R6-HOST-REC4, a departure from R2-TRN-REC5's per-hostname
  certificate); the Sentry
  and Cloudflare tokens CI would hold (R6-OBS-REC3, R6-HOST-REC1); whether previews are private (R6-HOST-OQ5). Secrets
  are Mustafa's to add; agents never add them.

### Recommendations that hinge on a source not read

| Recommendation | Source | Reason not read |
|---|---|---|
| R6-HOST-REC1 (Netlify as fallback) | https://www.netlify.com/pricing/ | outside the allow list |
| R6-HOST-REC1 (the 25 MiB mark on Workers static assets) | Cloudflare Workers static-assets limits page (developers.cloudflare.com) | not fetched in this run (the mark uses the Pages limit, F3) |
| R6-HOST-REC2 (Hetzner against alternatives) | https://docs.fly.io/networking/udp-and-tcp | outside the allow list (redirect from fly.io) |
| R6-HOST-REC2 (Hetzner against alternatives) | https://cloud.google.com/vpc/network-pricing | fetched, but only the title came back (page truncated) |
| R6-HOST-REC4 | https://letsencrypt.org/docs/ (challenge types, rate limits, lifetimes) | outside the allow list |
| R6-COST-REC1 (Sentry line) | https://sentry.io/pricing/ | outside the allow list |
| R6-COST-REC1 (per-GB comparison with a large cloud) | https://cloud.google.com/vpc/network-pricing | content truncated to the title |
| R6-OBS-REC3 | https://docs.sentry.io/platforms/javascript/configuration/integrations/wasm/ | 404 on 2026-10-28 |
| R6-OBS-REC4 | https://eur-lex.europa.eu/eli/reg/2016/679/oj (GDPR) | outside the allow list |
| R6-OBS-REC2 (M2 path) | Grafana Cloud ingestion docs | not fetched in this run (R6-OBS-OQ2) |
| R6-TEL-REC3 (what rates mean) | https://arxiv.org/pdf/2502.01250v2 | outside the allow list |
| R6-TEL-REC3 (shipped practice) | https://gdcvault.com/play/1024237/ | GDC Vault redirects to a maintenance page; video only |
| R6-TEL-REC2 (S3 against S1) | Cloudflare R2 data location docs | not fetched in this run |

### Pointers to other topics

From Hosting for previews and EU game servers:

- **R1:** per host, can it send COOP and COEP? Cloudflare Pages and Workers: yes, from `_headers`, static files only
  (F1, F2); Netlify: yes, but the same for every deploy context (F7); Vercel: yes (F8); GitHub Pages: no (F11, F12).
  For the first three, the pages show a mechanism for any custom header; that it carries COOP and COEP is inferred (no
  page names them). Code-generated responses (a Worker, a function) set their own.
- **R2:** reference server for R2-XU-OQ9: Hetzner CCX23 (4 dedicated vCPU, AMD), with CX33 measured beside it; "one
  vCPU of the EU instance type" in R2-BUD-REC6 is one vCPU of CCX23. No load balancer on the UDP path. IPv6 is free on
  Hetzner (F19), so R2-BUD-OQ11's IPv6 share depends on players' networks.
- **R5:** the preview link's CI part: a deploy step to Cloudflare from Actions needs an API token as a repository
  secret (Mustafa adds it); the staging server (REC3) receives the PR's server binary from CI.

From Costs:

- **R2:** at the goal, egress is not a cost driver on Hetzner (inside 20 TB per VM at full load); it is on per-GB hosts.
  Interest management's savings (R2-BUD-REC2) matter for players' links, not for this bill.
- **R1:** R1-ECS-OQ4's "one match per core" costs 0.0347 EUR per room-hour on CCX23, against 0.0173 at R2's 2 rooms per
  vCPU.

From Logs, metrics and crash reports:

- **R5:** CI uploads Wasm debug files to Sentry (a token: Mustafa's secret) and strips them from the shipped build; the
  gate v2's smoke test needs reporting off or the endpoints allowed (legacy TL9).
- **R1:** the `coep` and `coop` report types (F8) are the report-only rollout R1-BND-REC5 asks for.
- **R2:** the metric list in REC2 is R2's pointers (fallback share, relevance churn, budget overruns) plus R1-TICK-REC4.

From Balance telemetry, VISION_V2 §6.6:

- **R1:** damage and effect events must carry the source ability or weapon id (R1's pointer to R6) and the killer's
  unit id; the record's content hash is R1-DATA-REC6's.
- **R2:** the recorder reads the same tick-stamped events the clients get (R2-BUD-REC2); writing at Ended is
  R2-ROOM-REC1's mark (3).
- **game-designer (from the loop slice on):** owns the derived definitions in REC3 and the moment thresholds (OQ3).
- **test-engineer:** the statistics question (OQ1).

## 7. Verifier spot-check

The wave 2 verifier spot-checked 19 of the 40 key claims (the R6 findings named in a "Rests on:" list) on 2026-10-28/29,
verdict PASS WITH NOTES; its report is `.checkpoints/v2-research/verifier-r6-result.md` and its exact quotes are in
`.checkpoints/v2-research/verifier-r6.md`. The owner applied it on 2026-10-29 (log:
`.checkpoints/v2-research/infra-engineer-r6-fix.md`). No source was re-read and no claim re-sourced in the fix.

| Claim id | Claim | Source | Verdict | Location and exact words | Final state |
|---|---|---|---|---|---|
| R6-HOST-F15 | Hetzner CX/CAX/CCX prices in EUR, 20 TB included | hetzner.com/cloud/cost-optimized, /general-purpose | confirmed | table cells: CX33 "€ 8.99 per month, excl. VAT" "€ 0.0144 per hour"; CCX23 "€ 86.49 per month, excl. VAT" "€ 0.1386 per hour"; "20 TB"; CCX33 "30 TB" "€ 138.99"; CAX21 "€ 10.99" | fixed: the cell quotes added to the finding |
| R6-HOST-F16 | extra traffic 1.00 EUR per TB | hetzner.com/cloud/general-purpose | confirmed (the words; the per-TB reading stays inferred, as the file says) | "For each additional TB at EU-Central locations we charge € 1.00 per hour, excl. VAT." | unchanged |
| R6-HOST-F17 | billed by the hour, rounded up; only outgoing traffic | docs.hetzner.com/cloud/billing/faq (2020-07-07) | confirmed | "Servers have both a monthly price cap and a price per hour."; "We always round up the hourly usage of a server."; "We only bill for outgoing traffic." | unchanged |
| R6-HOST-F22 | Fly.io prices | fly.io/pricing | confirmed | "performance-2x · 4GB \| $0.0917 \| $66.00"; "Egress, North America and Europe \| $0.02 / GB"; "Dedicated IPv4 \| $2.00 / mo" | fixed: hourly rates added (verifier's quotes); the room-hour table now gives 0.0229 USD |
| R6-HOST-F3 | Pages: 20,000 files, 25 MiB per file, unlimited previews | developers.cloudflare.com/pages/platform/limits (Sep 5, 2026) | confirmed | "up to 20,000 files on the Free plan."; "25 MiB."; "unlimited number of preview deployments"; builds 500 / 5,000 / 20,000 (table cells) | fixed: build counts upgraded to confirmed (table cells) |
| R6-HOST-F5 | static requests free, no egress charge, 5 USD minimum | developers.cloudflare.com/workers/platform/pricing (Oct 2, 2026) | confirmed | "Requests to static assets are free and unlimited."; "There are no additional charges for data transfer (egress)"; "a minimum charge of $5 USD per month for an account" | unchanged |
| R6-HOST-F1 | `_headers` file, 100 rules, 2,000 characters | developers.cloudflare.com/pages/configuration/headers (Aug 25, 2026) | confirmed | "You may define up to 100 header rules."; "Each line in the `_headers` file has a 2,000 character limit." | unchanged (COOP and COEP by name stay inferred; TL;DR and pointers now say so) |
| R6-HOST-F12 | GitHub Pages: COOP/COEP request open, "No ETA" | github.com/orgs/community/discussions/13309 | confirmed (community thread only) | "No ETA at the moment as to when this feature will be available." (Jul 10, 2023) | unchanged |
| R6-HOST-F13 | EU locations and countries | hetzner.com/cloud | confirmed (which plan runs where stays inferred, as the file says) | "available to you in Germany, Finland, Singapore, and the USA."; "Falkenstein, Nuremberg, and Helsinki" | fixed: the product pages' "NBG1 HEL1" location column noted |
| R6-HOST-F14 | shared vCPU for variable load; dedicated for gaming | hetzner.com/cloud | confirmed | "They are designed for variable usage patterns rather than sustained high workloads."; "highly predictable resources."; "gaming servers" | unchanged |
| R6-HOST-F24 | ACME wildcard orders, dns-01 TXT record, http-01 on port 80 | RFC 8555 §7.1.3, §8.3, §8.4 | confirmed | "MAY have a wildcard domain name as its value."; "prepending the label "_acme-challenge""; "MUST be sent to TCP port 80" | unchanged |
| R6-COST-F1 | Grafana Free: 10k series, 50 GB, 14 days, 3 users | grafana.com/pricing | confirmed (the 14 days too) | "Limited to 10k active series per month"; "Limited to 50 GB ingested per month"; "14 days retention"; "Limited to 3 active users per month" | fixed: the 14 days upgraded to confirmed |
| R6-COST-F2 | Grafana Pro: 19 USD plus usage | grafana.com/pricing | confirmed | "Platform fee of $19 per month includes:"; "$6.50/ 1k series"; "$0.050/GB Process" "$0.400/GBWrite" "$0.100/GBRetain" | unchanged |
| R6-COST-F5 | Hetzner Object Storage 6.49 EUR with 1 TB, EU locations | hetzner.com/storage/object-storage | confirmed (the 1 TB and the locations too) | "base price of € 6.49 per month, excl. VAT"; "includes 1 TB of storage (up to 744 TB-hours) and 1 TB of egress traffic."; "(FSN1) … (HEL1), and … (NBG1)" | fixed: the 1 TB and the locations upgraded to confirmed |
| R6-OBS-F5 | Prometheus: 15 days default, 1–2 bytes per sample, not replicated | prometheus.io/docs/prometheus/latest/storage | confirmed | "the retention time defaults to `15d`."; "an average of only 1-2 bytes per sample."; "not clustered or replicated." | unchanged |
| R6-OBS-F1 | `tracing` 0.1.44: spans and events; no allocation by default | docs.rs/tracing/latest | confirmed ("not built" stays inferred, as the file says) | "a span represents a _period of time_ with a beginning and an end."; "does not allocate by default." | unchanged |
| R6-OBS-F7 | Sentry Wasm: build_id, wasm-split, wasmIntegration, "still developing" | docs.sentry.io/platforms/native/guides/wasm | confirmed (the word "DWARF" is paraphrase only: the fetch dropped the link text) | "augmented by a build_id custom section."; "We provide a tool called wasm-split"; "apply its wasmIntegration"; "This feature is still developing, so may be rough around the edges." | fixed: "DWARF" marked as paraphrase |
| R6-OBS-F9 | Sentry: US or EU (Frankfurt), cannot be changed, some data stays in the US | docs.sentry.io/organization/data-storage-location | confirmed (the US list too) | "Sentry supports data storage in both the US and the EU."; "Frankfurt, Germany."; "once selected, your data storage location can’t be changed." | fixed: the US list quoted and upgraded to confirmed |
| R6-TEL-F2 | LOOP_V2 points race, steals, counters | docs/LOOP_V2.md §4 l.44, §8 l.98–100, §11 l.153–154 | confirmed | "The countdown keeps running, now for the new owner."; "Destroying an HQ is a big points swing"; "(cashouts, steals, deliveries, harvester kills)" | unchanged |

**The verifier's critical read, and what was done with each**
1. (medium) The wildcard certificate departs from R2-TRN-REC5's "per game server hostname". Fixed: R6-HOST-REC4 and
   option C2 state the departure, quote R2 in full and name per-hostname certificates as the alternative; the shared
   private key is added to R6-SEC-OQ1's items met on the way.
2. (medium) Option K4's "costs more than the whole Hetzner fleet" did not follow. Fixed: K4 now compares per fully
   loaded host (about 369 USD against 86.49 EUR) and calls the goal-scale figures (about 276 USD against 222.88–345.96
   EUR) the same order, with no exchange rate read.
3. (medium) Missing "Rests on:" entries. Fixed: R6-HOST-REC2 adds F20, R2-BUD-REC6, R2-ROOM-REC2 (with its marks'
   lines), R2-TRN-REC3, R1-DET-OQ7 and R1-DET-REC6; R6-HOST-REC1 adds F8 and R1-BND-REC5.
4. (medium) R6-HOST-REC2 picked Hetzner without F20. Fixed: its text now quotes F20 and makes the pick conditional on
   R6-HOST-OQ1 (a working pick until a console check shows capacity).
5. (low–medium) fsn1 may not offer the recommended plans. Fixed: F13 notes the "NBG1 HEL1" location column; Q4 and
   R6-HOST-OQ2 offer Falkenstein only if the plans are offered there.
6. (low–medium) Two expected options (Hetzner dedicated root servers; ACME tls-alpn-01, RFC 8737). Partly applied: not
   added as options, because the fix rule adds a new source only where a recommendation cannot stand without it, and
   neither R6-HOST-REC2 nor R6-HOST-REC4 needs one; both are named as unread candidates in R6-HOST-OQ4 and R6-HOST-OQ6
   and listed under "Sources not read".
7. (low) The 25 MiB mark is a Pages limit. Fixed: R6-HOST-REC1's mark says so and is rechecked against the Workers
   static-assets limits page before the spike; that page is listed as not read.
8. (low) Fly.io's hourly price was on the page. Fixed: F22 carries the hourly rates; the room-hour table gives 0.0229
   USD (0.0917 ÷ 4).
9. (low) F15 confirmed without a quote. Fixed: the verifier's cell quotes are in F15.
10. (low) COOP and COEP stated as fact. Fixed: the TL;DR, the Q1 table (Netlify, Vercel), R6-HOST-REC1's reason and the
    R1 pointer now mark them inferred.
11. (low) R6-OBS-REC4's Sentry retention had no source; loose "Serves" traces. Fixed: the Sentry cap is open, decided by
    R6-COST-OQ2's read; R6-OBS-REC1 now serves §4's fair tick first; R6-OBS-REC4's "Serves" says no §6 item asks for it
    (it is a condition for the stores §6.1 and §6.6 create).
12. (low) R6-TEL-REC2's append file conflicted with R6-TEL-REC1's in-memory buffer. Fixed: a crashed room's buffer is
    lost and the supervisor writes a stub record marked incomplete; the pass mark follows.
13. (low) R2 cites TL15's figure as "legacy CM4". Fixed: R6's TL15 line says the id slip is R2's.
- Arithmetic nit (3.156 GB, not 3.16, behind 18,431 and 13,823 GB). Fixed: the full-host sentence says so.

**Counts:** 19 confirmed, 0 wrong, 0 unsupported (19 of 40 key claims checked). No recommendation rests on a wrong or
unsupported claim; none became an open question; no claim was re-sourced. Not checked at their source (21): HOST-F2
(Workers half), F4, F6, F7, F9, F10, F11, F18, F19, F23; COST-F3, F4; OBS-F2, F3, F4, F6, F8, F10, F11, F12; TEL-F1.

## 8. Sources

### Sources read (all read 2026-10-28)

#### Static hosting and previews (R6-HOST)
| # | URL | Title | Version or page date | Tier |
|---|---|---|---|---|
| 1 | https://developers.cloudflare.com/pages/configuration/headers/ | Cloudflare Pages: Headers | Last updated Aug 25, 2026 | official |
| 2 | https://developers.cloudflare.com/pages/platform/limits/ | Cloudflare Pages: Limits | Last updated Sep 5, 2026 | official |
| 3 | https://developers.cloudflare.com/pages/configuration/preview-deployments/ | Cloudflare Pages: Preview deployments | Last updated Oct 12, 2026 | official |
| 4 | https://developers.cloudflare.com/workers/static-assets/headers/ | Workers static assets: Headers | Last updated Sep 22, 2026 | official |
| 5 | https://developers.cloudflare.com/workers/platform/pricing/ | Cloudflare Workers pricing | Last updated Oct 2, 2026 | official |
| 6 | https://developers.cloudflare.com/workers/static-assets/migration-guides/migrate-from-pages/ | Migrate from Pages to Workers | Last updated Oct 12, 2026 | official |
| 7 | https://docs.netlify.com/routing/headers/ | Netlify: Custom headers | Last updated Dec 1, 2025 | official |
| 8 | https://vercel.com/docs/project-configuration/vercel-json | Static configuration with vercel.json | 2026-08-14 | official |
| 9 | https://vercel.com/docs/plans/hobby | Vercel Hobby plan | 2026-09-14 | official |
| 10 | https://vercel.com/docs/deployment-protection | Vercel Deployment Protection | 2026-09-15 | official |
| 11 | https://docs.github.com/en/pages/getting-started-with-github-pages/github-pages-limits | GitHub Pages limits | no date | official |
| 12 | https://github.com/orgs/community/discussions/13309 | Allow setting COOP and COEP headers in Github Pages | opened 2022-03-23, still open | other (community forum) |

#### Game servers and certificates (R6-HOST)
| # | URL | Title | Version or page date | Tier |
|---|---|---|---|---|
| 13 | https://www.hetzner.com/cloud | Hetzner Cloud | no date (© 2026) | official |
| 14 | https://www.hetzner.com/cloud/cost-optimized | Hetzner Cloud Cost-Optimized plans | no date (© 2026) | official |
| 15 | https://www.hetzner.com/cloud/regular-performance | Hetzner Cloud Regular Performance plans | no date (© 2026) | official |
| 16 | https://www.hetzner.com/cloud/general-purpose | Hetzner Cloud General Purpose (dedicated) plans | no date (© 2026) | official |
| 17 | https://docs.hetzner.com/cloud/billing/faq | Hetzner Cloud billing FAQ | Last change 2020-07-07 | official |
| 18 | https://docs.hetzner.com/cloud/firewalls/overview | Hetzner Cloud Firewalls overview | Last change 2021-12-23 | official |
| 19 | https://docs.hetzner.com/cloud/firewalls/faq | Hetzner Cloud Firewalls FAQ | Last change 2020-03-08 | official |
| 20 | https://docs.hetzner.com/cloud/servers/primary-ips/overview | Hetzner Primary IPs | 2022-06-28 | official |
| 21 | https://docs.hetzner.com/cloud/load-balancers/overview | Hetzner Load Balancers overview | Last change 2021-12-23 | official |
| 22 | https://docs.hetzner.com/cloud/general/locations | Hetzner Cloud locations | Last change 2021-11-03 | official |
| 23 | https://fly.io/pricing/ | Fly.io pricing | no date | official |
| 24 | https://aws.amazon.com/gamelift/servers/pricing/ | Amazon GameLift Servers pricing | no date (© 2026) | official |
| 25 | https://www.rfc-editor.org/rfc/rfc8555.html | RFC 8555, Automatic Certificate Management Environment (ACME) | March 2019 | official (IETF standard) |

#### Costs (R6-COST)
| # | URL | Title | Version or page date | Tier |
|---|---|---|---|---|
| 26 | https://grafana.com/pricing/ | Grafana Cloud pricing | no date (© 2026); first fetch HTTP 502, second read | official |
| 27 | https://docs.sentry.io/pricing/ | Sentry pricing (docs) | no date | official |
| 28 | https://developers.cloudflare.com/r2/pricing/ | Cloudflare R2 pricing | Last updated Oct 26, 2026 | official |
| 29 | https://www.hetzner.com/storage/object-storage | Hetzner Object Storage | no date (© 2026) | official |

#### Logs, metrics and crash reports (R6-OBS)
| # | URL | Title | Version or page date | Tier |
|---|---|---|---|---|
| 30 | https://docs.rs/tracing/latest/tracing/ | tracing | 0.1.44 | official (crate docs) |
| 31 | https://docs.rs/tracing-subscriber/latest/tracing_subscriber/fmt/format/struct.Json.html | tracing-subscriber, Json formatter | 0.3.23 | official (crate docs) |
| 32 | https://opentelemetry.io/docs/languages/rust/ | OpenTelemetry Rust | no date | official |
| 33 | https://docs.rs/metrics-exporter-prometheus/latest/metrics_exporter_prometheus/ | metrics-exporter-prometheus | 0.18.3 | official (crate docs) |
| 34 | https://prometheus.io/docs/prometheus/latest/storage/ | Prometheus storage | docs for 3.15 | official |
| 35 | https://docs.sentry.io/platforms/rust/ | Sentry for Rust | `sentry` 0.49.5; no date | official |
| 36 | https://docs.sentry.io/platforms/native/guides/wasm/ | Sentry for WebAssembly | no date | official |
| 37 | https://developer.mozilla.org/en-US/docs/Web/API/Reporting_API | MDN: Reporting API | last modified Apr 21, 2026 | official |
| 38 | https://docs.sentry.io/organization/data-storage-location/ | Sentry data storage location | no date | official |
| 39 | https://docs.sentry.io/platforms/javascript/data-management/data-collected/ | Sentry JavaScript: data collected | no date | official |
| 40 | https://grafana.com/docs/grafana-cloud/security-and-account-management/regional-availability/ | Grafana Cloud regional availability | no date | official |
| 41 | https://cheatsheetseries.owasp.org/cheatsheets/Logging_Cheat_Sheet.html | OWASP Logging Cheat Sheet | no date | official (OWASP, tiered as R2 does) |

#### Balance telemetry (R6-TEL)
| # | URL | Title | Version or page date | Tier |
|---|---|---|---|---|
| 42 | https://docs.rs/parquet/latest/parquet/ | parquet | 60.1.0 (page shows 28 October 2026) | official (crate docs) |

Project input read (not counted as a research source): `docs/LOOP_V2.md` §1, §4, §8, §11.

Counts by tier: official 41, shipped game 0, other 1; total 42.

### Sources not read
| # | URL | Reason |
|---|---|---|
| 1 | https://docs.fly.io/networking/udp-and-tcp | outside the allow list (fly.io/docs redirects there) |
| 2 | https://cloud.google.com/vpc/network-pricing | fetched; only the title came back (content truncated), no prices visible |
| 3 | https://www.netlify.com/pricing/ | outside the allow list |
| 4 | https://letsencrypt.org/docs/ (challenge types, rate limits, certificate lifetimes) | outside the allow list |
| 5 | https://cabforum.org/ (certificate lifetime rules) | outside the allow list |
| 6 | OVHcloud, Scaleway and Netcup price pages | outside the allow list |
| 7 | https://sentry.io/pricing/ | outside the allow list |
| 8 | https://docs.sentry.io/platforms/javascript/configuration/integrations/wasm/ | the page answered "Page Not Found" (404) on 2026-10-28 |
| 9 | Grafana Cloud ingestion docs (agent or remote write) | not fetched in this run (R6-OBS-OQ2) |
| 10 | https://eur-lex.europa.eu/eli/reg/2016/679/oj (GDPR) | outside the allow list |
| 11 | European Data Protection Board guidance (edpb.europa.eu) | outside the allow list |
| 12 | https://cloud.google.com/bigquery/pricing | fetched; only the title came back (content truncated) |
| 13 | Cloudflare R2 data location (jurisdiction) docs | not fetched in this run (R6-TEL-REC2's option S3) |
| 14 | https://gdcvault.com/play/1024237/ (Riot, League of Legends balance) | GDC Vault redirects to a maintenance page outside the allow list; video only |
| 15 | https://arxiv.org/pdf/2502.01250v2 (win and pick rates in team games) | outside the allow list |
| 16 | https://us.forums.blizzard.com/en/heroes/t/internal-data-used-for-balance-patches/33949 | outside the allow list |
| 17 | postgresql.org, sqlite.org, duckdb.org | outside the allow list |
| 18 | https://www.rfc-editor.org/rfc/rfc8737.html (ACME tls-alpn-01) | not fetched: named by the verifier; the fix step adds a source only where a recommendation cannot stand without it (R6-HOST-OQ6) |
| 19 | Hetzner dedicated root server pages (www.hetzner.com) | not fetched, same reason (R6-HOST-OQ4) |
| 20 | Cloudflare Workers static-assets limits page (developers.cloudflare.com) | not fetched, same reason (R6-HOST-REC1's 25 MiB mark) |
