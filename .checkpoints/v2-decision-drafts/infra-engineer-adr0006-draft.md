# ADR-0006: Infrastructure and operations

**Status:** Proposed
**Date:** 2026-10-31
**Author:** infra-engineer (pass marks by test-engineer, plan step 4) · **Reviewed by:** (filled in at review) · **Signed off by Mustafa:** no

This record decides where SHAMBLE FRONTLINES v2 runs and how it is watched. The player build and every pull request's
[preview link](../research/GLOSSARY.md#preview) are served by [Cloudflare Pages](../research/GLOSSARY.md#the-client-host); previews that reach a game server sit behind a login from the first one.
Game servers run on Hetzner Cloud in one EU location, one room per process, on a named [reference server](../research/GLOSSARY.md#the-reference-server) that every
server pass mark is measured on. It sets the [start and goal](../research/GLOSSARY.md#the-start-modes-and-the-goal-modes) budgets (with the dates of the prices), the logs, metrics
and crash reports, where the balance-telemetry match records are stored and how they are analysed, and the data rules
for all of it. [The security baseline](../research/GLOSSARY.md#the-security-baseline) stays open, with interim positions, until its own research before the first
online playtest. It decides this now because [the netcode proof test](../research/GLOSSARY.md#the-netcode-proof-test), [the foundation](../research/GLOSSARY.md#the-foundation)'s preview links and every server
mark in ADR 0001 and ADR 0004 need a named machine, a host and a budget before they can start.

Citations give the research file, its part, the id and the lines at commit 9e7db28 (the topic files do not change in
this PR). Inside a parenthesis, "r1", "r2", "r3", "r5" and "r6" stand for `r1-simulation-core.md`, `r2-netcode.md`,
`r3-rendering.md`, `r5-toolchain-quality.md` and `r6-infrastructure.md`; an id's part is the one its kind names (an F
id under Findings, a REC under Recommendations, an OQ under Open questions, an option or an A-n assumption under
Options); the claims ledger beside this draft gives every citation in full. Prices are as the research read them on 2026-10-28, in EUR or USD as published, excluding VAT
(r6-infrastructure.md, Header and Findings intro, lines 40–41 and 54); no exchange rate was read, so EUR and USD stay
apart. No account was created and no service configured in this research (r6 lines 40–41): every account, payment,
secret and repository setting named below is Mustafa's to create, never a session's.

## Context

### What it serves
- **VISION_V2 §6.1, modes as data, rooms and private matches:** rooms need machines, a supervisor, a lobby and a
  client host that reaches them.
- **VISION_V2 §6.6, balance telemetry:** "every match records the numbers that matter: who led, the swings, steals,
  and unit pick and win rates".
- **VISION_V2 §6.8, moments:** the server recognizes them; this record stores them with the match. Kill-cams, replays
  and highlights are `[LATER]`: nothing is built for them here (part 12 says what keeps them possible).
- **VISION_V2 §6.10, accounts,** `[LATER]`: no account store is built; the lobby's guest identity keeps it possible
  (part 6).
- **VISION_V2 §3 and §4** (counted like LOOP_V2 under the session's trace reading, decision map part j, J3): the start
  modes and the goal modes ([the start modes and the goal modes](../research/GLOSSARY.md#the-start-modes-and-the-goal-modes)),
  "Desktop browsers first: Chrome, Edge, Firefox and Safari", "Matches feel fair up to about 100 ms of ping. Servers
  start in one EU region." VISION_V2 §5: "Guest play first".
- **LOOP_V2 §8** (points from cashouts, deliveries, kills and structure kills; the prototype records Service points
  through KVStore) and **§11** (the objective counters "cashouts, steals, deliveries, harvester kills").
- **LOOK_V2 §8:** "v2 may use authored 3D models": the client host serves them as files (part 1), and a preview that
  carries unreleased art is behind [the access wall](../research/GLOSSARY.md#the-access-wall) (part 2).
- **[The foundation](../research/GLOSSARY.md#the-foundation)** (GLOSSARY §8, brief §2): "CI gate v2, and private preview
  links"; it "ends when an empty scene runs through the full stack in the browser, with CI green and a preview link".

### Settled inputs this record applies (never reopened)
- **MODES-3 (a):** totals; [the largest mode](../research/GLOSSARY.md#the-largest-mode-and-its-n) is [24v24](../research/GLOSSARY.md#team-size-and-how-modes-are-written), [N](../research/GLOSSARY.md#player-count-n) = 48. Where an R6 mark gives 48 and 54, 48 holds
  (carry-forward 3).
- **TICK-30:** a 30 Hz server tick is the baseline; 60 Hz only with
  [the 60 Hz margin](../research/GLOSSARY.md#the-60-hz-margin) of at least 25 % (ADR 0001 owns the rule and its marks).
- **SLICE-ORDER** and its three conditions: the game runs as a client plus a local server from slice 1; inputs drive
  movement and shooting; [the netcode proof test](../research/GLOSSARY.md#the-netcode-proof-test) runs before the
  foundation.
- **PREVIEW-WALL (a):** "private from the first preview that reaches the test game server" (Mustafa, 2026-10-31).
- **BASELINE-WHEN (b):** "List it in ADR 0006 as open, with temporary positions. Research the security baseline
  before the first online playtest, in its own fresh session." (the Advisor, 2026-10-31).
- **UPGRADE-SCOPE** (Mustafa, 2026-10-31): roguelike upgrade tracks per unit and per weapon type, owned by the player;
  this record only adds their balance fields to the match record (part 12).
- **SPIKE-SCOPE (a)** (spikes are throwaway, in `spikes/`) and **R7-MARKS (a)** (the inventory's marks are working
  recommendations).

### The findings the decision turns on
1. Cloudflare Pages sets response headers from a `_headers` file, gives every branch its own link with no cap on
   previews, sends `noindex` on previews, and can put previews behind an access policy; but "The latest deployment for
   a branch cannot be deleted" (r6-infrastructure.md, Findings §1, R6-HOST-F1, F3, lines 60–65 and 72–79;
   r5-toolchain-quality.md, Findings, Question 6, R5-PREV-F12, F13, lines 838–849).
2. Hetzner sells dedicated vCPU plans for "highly predictable resources", naming gaming servers; a CCX23 (4 dedicated
   vCPU, 16 GB) costs 86.49 EUR a month with 20 TB of traffic included; and on 2026-10-28 every EU plan showed "This
   product is currently unavailable. Please check back later." (r6-infrastructure.md, Findings §1, R6-HOST-F14, F15,
   F20, lines 144–159 and 180–182).
3. A 24v24 match-hour sends about 3.16 GB, so even 8 full rooms on one VM all month stay inside its 20 TB
   (r6-infrastructure.md, Options §2, cost arithmetic, lines 441–446 and 461–464).
4. The research's 8 rooms per 4-vCPU VM is "a density to measure", and "the 4-rooms-per-VM row (one per vCPU) is the
   case to budget unless R2-ROOM-REC2 marks 1 and 2 pass on it" (r6-infrastructure.md, Options §2, assumption A-2,
   lines 516–522).
5. Sentry's EU storage is chosen when the organization is created "and can't be changed", and its browser SDK sends
   the user's IP address by default (r6-infrastructure.md, Findings §3, R6-OBS-F9, F10, lines 318–331).

### What it takes and what it gives
- **Takes:** the room process, its lifecycle, join tokens, the guest identity and the lobby protocol (ADR 0001); the
  CI workflows, the preview build and deploy job, the generated header sets, the Sentry upload job and how a
  multiplayer preview gets its server binary and certificate hash (ADR 0002; seam S3, critic ranks 20 and 34); what
  client field data is measured (ADR 0003, rank 26); the tick-stamped events and recognized moments (ADR 0004); the
  content hash, the ids and the content bundle (ADR 0005).
- **Gives:** [the reference server](../research/GLOSSARY.md#the-reference-server) every server mark is measured on
  (interface I-15, number N34); the client host and its 25 MiB per-file limit (N37) to ADR 0002; the VMs, the
  supervisor, the firewall, the lobby's hosting and the production certificate form to ADR 0001 (seam S4); the match
  record's shape, storage and analysis to ADR 0004, ADR 0001 and ADR 0005 (seam S6, interface I-10); the security
  baseline's list of waiting items and their interim positions to ADR 0001, ADR 0002 and ADR 0005 (seam S12).

### What the legacy game proves
- Legacy has no server, host, domain or certificate: its storage is the browser's (KVStore, inventory row CD2), so
  "v2's hosting starts from zero" (r6-infrastructure.md, Findings §1, legacy facts, lines 207–208).
- The only CPU figure for a 48-soldier room is legacy's 0.80–1.00 ms per JavaScript step on DM_Tri (inventory rows
  TL15, CM4; r6-infrastructure.md, legacy facts, lines 211–215); R2's 8 rooms per VM is a proposal built on it, "not a
  measurement" (r6 lines 255–256).
- Today's CI holds no secret and its token is read-only, so a preview deploy is the first job with a secret (row KT8;
  r5-toolchain-quality.md, Findings, Question 6, legacy facts, lines 904–907).
- The smoke test fails on any network request (row TL9), so a preview that talks to a game server, and crash
  reporting, need the gate v2 to allow their origins or switch reporting off (r6-infrastructure.md, legacy facts,
  lines 209–210 and 353–354).
- Balance numbers need several runs (row TL13: bot stall % of 13.0 / 5.7 / 7.3 against 3.4 / 16.1 / 6.5, the same
  mean), kill sources (row GE15: a grenade kill counts for the held gun) and damage contribution (row GC18: assists
  never counted) (r6-infrastructure.md, Findings §4, legacy facts, lines 380–388).
- The three.js CDN tag has no integrity hash and no CSP (row PG19; STATUS "no SRI/CSP on the three.js CDN tag
  (belongs with hosting, ADR-001)").

## Decision

Serve the player build and every preview link from Cloudflare Pages, and run game servers on Hetzner Cloud in one EU
location, one room per process under a small supervisor per VM, on a dedicated-vCPU CCX23 named as the reference
server, with the shared CX33 measured beside it. Keep the budget flat and small at the start (about 18 to 102 EUR a
month) and state the goal budget at both room densities until the netcode proof test measures how many rooms fit one
VM. Log, measure and report crashes with small, EU-only tools that keep no personal data. Write one balance record per
match inside the room, store it as JSON lines in EU object storage, and analyse it in batches per content hash. List
the security baseline as open, with interim positions, until its own research before the first online playtest. Every
account, payment, secret and repository setting below is Mustafa's to create.

1. **The client host.** [The client host](../research/GLOSSARY.md#the-client-host) is Cloudflare Pages, for the player
   build and every per-PR [preview](../research/GLOSSARY.md#preview), with Workers static assets (the alternative), which
   takes the same `_headers` file. The build's generated `_headers` (ADR 0002) carries
   [the production header set](../research/GLOSSARY.md#the-production-header-set-and-the-preview-header-set) on
   production and the preview header set on previews; COOP and COEP appear only on a preview built with
   [the isolation flag](../research/GLOSSARY.md#preview-isolation-and-the-isolation-flag) on, and the preview Mustafa
   checks is built with it off (R5-XQ-OQ3, settled 2026-10-30: r5-toolchain-quality.md, Open questions, R5-XQ-OQ3,
   lines 1899–1910). The content bundle is served with an immutable cache header (R1-DATA-REC6; interface I-6). Every
   served file, authored models included (LOOK_V2 §8), stays under 25 MiB, Pages' per-asset limit (r6-infrastructure.md,
   Findings §1, R6-HOST-F3, lines 72–79). three.js is bundled from npm by Vite and served from the client host, so v2
   has no CDN script tag (the STATUS item "no SRI/CSP on the three.js CDN tag (belongs with hosting, ADR-001)"; inventory
   row PG19); the content security policy waits for the security baseline (part 13).
   *Why:* its `_headers` file sets custom response headers (COOP and COEP by name inferred), every branch gets its own
   link with no cap on previews, and GitHub Pages cannot send those headers; Pages meets every need of R5-PREV-REC2's
   list on the findings read but the deletion of a branch's latest deployment (r6-infrastructure.md,
   Recommendations §1, R6-HOST-REC1, lines 619–644; r5-toolchain-quality.md, Recommendations, Question 6,
   R5-PREV-REC2, lines 1605–1615). *Cost:* 0 USD on Pages' Free plan (the price is inferred from the plan's name),
   5 USD a month for Workers Paid only if Worker code is ever needed (r6-infrastructure.md, Options §2, start scenario,
   line 476; R6-HOST-F5, lines 87–92). *Ids:* R6-HOST-REC1, R5-PREV-REC2, R5-PREV-OQ4. *Need:* VISION_V2 §6.1 (rooms and
   private matches are reached from this client) and VISION_V2 §4 (desktop browsers first).
2. **Preview privacy.** A preview that runs the match offline in the Worker with bots is public but unlisted
   (`noindex` by default, branch-name links), unless it carries unreleased art, which puts it behind
   [the access wall](../research/GLOSSARY.md#the-access-wall) (r5-toolchain-quality.md, Recommendations, Question 6,
   R5-PREV-REC5, lines 1642–1650). Every multiplayer preview, which talks to [the staging game server](../research/GLOSSARY.md#the-staging-game-server), is behind the
   access wall from the first one (PREVIEW-WALL (a)). The access wall is the client host's access policy, limited to
   Mustafa's Cloudflare account, or Cloudflare Access; Mustafa logs in once per Brave session (R5-PREV-REC5;
   R5-PREV-F12, F14, r5 lines 838–858). Pages' policy restricts "project deployments" as a whole (R5-PREV-F12, lines
   838–844), so from the first multiplayer preview every preview of the project sits behind the wall (proposed interim
   position; whether to split them is OQ-0006-4). The policy covers previews, not the production domain
   (r6-infrastructure.md, Findings §1, R6-HOST-F4, lines 80–86, inferred), so the game itself stays public.
   *Why:* PREVIEW-WALL (a); a preview holds no secrets, so offline previews need only `noindex`
   (r6-infrastructure.md, R6-HOST-REC1, line 622). *Ids:* PREVIEW-WALL, R5-PREV-REC5, R5-PREV-OQ1, R6-HOST-OQ5.
   *Need:* GLOSSARY §8, the foundation ("private preview links") only; LOOK_V2 §8 (unreleased authored models).
3. **Preview cleanup.** 24 h after a pull request closes, its preview link no longer serves the build (HTTP 404, or the
   access wall), or the remaining case is written down with the reason; a weekly sweep leaves 0 previews of closed pull
   requests older than 7 days; build artifacts are kept 7 days, not the default 90 (r5-toolchain-quality.md,
   Recommendations, Question 6, R5-PREV-REC3, lines 1616–1624). The policy is this record's; the close and sweep jobs
   are ADR 0002's workflows (seam S3). On Pages "The latest deployment for a branch cannot be deleted" (R5-PREV-F13,
   r5 lines 845–849), so whether a closed pull request's last link really disappears stays a spike item, PREV-DELETE, in
   the preview hosting check (part 14); if Pages fails it with no workaround, Workers static assets is weighed again
   (r6-infrastructure.md, Shared open questions, PREV-DELETE, lines 970–976). A closed pull request's room binary on the
   staging game server is gone within 24 h (R6-HOST-REC3's mark, r6 lines 681–683). *Ids:* R5-PREV-REC3, R5-PREV-OQ2,
   PREV-DELETE. *Need:* GLOSSARY §8, the foundation only (R5-PREV-REC3 serves no §6 item, r5 line 1622).
4. **The staging game server.** Previews share one small VM,
   [the staging game server](../research/GLOSSARY.md#the-staging-game-server), proposed a CX33 (4 vCPU, 8 GB, 8.99 EUR
   a month: r6-infrastructure.md, Findings §1, R6-HOST-F15, lines 150–159), never one VM per pull request. Its
   supervisor starts the room binary of the build a preview names; it presents development certificates (part 7);
   production servers refuse preview origins, and a preview never connects to a production server
   (r6-infrastructure.md, Recommendations §1, R6-HOST-REC3, lines 672–683). PREVIEW-WALL (a) ends the wait in "Until
   Mustafa answers PREVIEW-WALL (below), previews do not use the staging server" (r6 line 675): from the first multiplayer
   preview (slice 3, online), previews use it, behind the access wall (part 2). How a preview's binary and certificate
   hash reach it is ADR 0002's workflow and ADR 0001's hash endpoint (critic rank 20). At the start the same VM also
   runs Prometheus (part 9) and the batch analysis (part 12). *Why:* a VM per pull request bills at least an hour each
   time and adds machines to run; previews only need a server that speaks the pull request's protocol version
   (r6 lines 676–677; R6-HOST-F17, r6 lines 164–168). *Ids:* R6-HOST-REC3, R5-PREV-OQ3. *Need:* VISION_V2 §6.1 (rooms, tested
   per pull request, r6 line 680).
5. **Game servers and the reference server.** Hetzner Cloud in one EU location; one room per process (R2-ROOM-REC2,
   ADR 0001's) under a small supervisor per VM; no load balancer on the game path, since the lobby hands out host and
   port; each VM has its own public IPv4 and IPv6 address. The firewall opens one UDP range for WebTransport and one
   TCP range for the WebSocket fallback (R2-TRN-REC3), plus the one rule part 9's metrics path may need, and nothing
   else; Hetzner's firewall is stateful, drops all inbound traffic no rule allows and costs nothing
   (r6-infrastructure.md, Findings §1, R6-HOST-F18, lines 169–174). VMs stay up and rooms start and end as processes
   inside them, because billing rounds up to the hour (R6-HOST-F17). The server CPU is x86-64 (the dedicated plans are
   AMD), which answers R1-DET-OQ7 for the first fleet; Arm waits until R1-DET-REC6's matrix includes it. **The
   reference server** is a CCX23 (4 dedicated vCPU, 16 GB, x86-64 AMD), and the shared CX33 (4 vCPU, 8 GB) is measured
   beside it (r6-infrastructure.md, Recommendations §1, R6-HOST-REC2, lines 645–671); this answers R2-XU-OQ9
   (r2-netcode.md, Cross-unit open questions, R2-XU-OQ9, lines 3197–3202), and every native server mark is measured on
   [one vCPU](../research/GLOSSARY.md#the-measuring-unit-one-vcpu) of it. It is a working pick, not a settled one:
   every EU plan showed "currently unavailable" on 2026-10-28 (R6-HOST-F20, r6 lines 180–182), so Mustafa checks capacity
   in the Hetzner console with his own account before the netcode proof test is scheduled (OQ-0006-1). The location is
   Nuremberg (`nbg1`) or Helsinki (`hel1`), Falkenstein (`fsn1`) only if the plans are offered there, picked by a
   round-trip measurement (r6-infrastructure.md, Options §1, Q4, lines 429–432; OQ-0006-2).
   *Why:* dedicated vCPUs are sold for "highly predictable resources" and name gaming servers, which the fair tick of
   VISION_V2 §4 and [the server tick budget](../research/GLOSSARY.md#the-server-tick-budget) need; 20 TB of egress per
   VM is included; the shared CX33 is 9.6 times cheaper (8.99 against 86.49 EUR), so it is measured, not assumed
   (r6 lines 656–659; R6-HOST-F14, F15, r6 lines 144–159). *Ids:* R6-HOST-REC2, R2-XU-OQ9, R1-DET-OQ7, R6-HOST-OQ3 (answered:
   no UDP load balancer while the lobby hands out host and port, r6 lines 859–861). *Need:* VISION_V2 §6.1, §6.5, §6.6 and
   §6.8 (rooms, economy, records and moments on the server) and VISION_V2 §4 (one EU region, fair to about 100 ms)
   (r6 lines 665–666).
6. **The lobby's place.** The lobby (join tokens, guest identity, room codes, the room directory, and part 10's crash
   report endpoint) is a required service in this record's scope (critic rank 8: "the infrastructure ADR and the cost
   budget leave out a required service", r6-infrastructure.md, Cross-topic open questions, R6-XT-OQ1, lines 991–997).
   Its protocol and token format are ADR 0001's (R2-ROOM-REC3, REC8), its crate ADR 0002's. Where it runs stays open
   between the two options R2 names, "Same binary as the supervisor or a separate service" (r2-netcode.md, Open
   questions, R2-ROOM-OQ9, lines 3104–3105; OQ-0006-3); what stores guest ids and the room directory is not researched
   (a database was not read: r6-infrastructure.md, Options §4, Q3, line 593). On the first option the lobby runs on VMs
   already budgeted; on the second it needs one more VM, for example a CX33 at 8.99 EUR a month (R6-HOST-F15; a
   placeholder line, not a researched sizing). Part 8's totals carry no lobby line. Its signing keys are a security
   baseline item (part 13), and its HTTPS endpoint needs its own certificate, which R6-HOST-REC4 does not cover (not
   researched). It keeps VISION_V2 §6.10 possible: a later login links to the same guest id (r2-netcode.md,
   R2-ROOM-REC8, lines 2868–2885); no account store is built. *Ids:* R2-ROOM-OQ9, R2-XT-OQ4, R5-XT-OQ3, R6-XT-OQ1.
   *Need:* VISION_V2 §6.1 (rooms and private matches go through it).
7. **Certificates (interim position, BASELINE-WHEN (b)).** In production every game server presents
   [a certificate per hostname](../research/GLOSSARY.md#game-server-certificate), issued through ACME (RFC 8555)
   (r6-infrastructure.md, Recommendations §1, R6-HOST-REC4, lines 684–699; r2-netcode.md, Recommendations, R2-TRN-REC5,
   lines 2699–2723). The wildcard certificate (option C2: one job renews it and copies it to every game VM, so one
   private key sits on every VM) is an option the security baseline weighs. Which ACME challenge issues the
   per-hostname certificates (http-01 needs TCP port 80 on each game VM; dns-01 per name needs a DNS provider's API key,
   a secret) and how long they live is OQ-0006-5 (R6-HOST-F24, r6 lines 198–204; R6-HOST-OQ6, r6 lines 869–874). Development
   and preview servers use self-signed ECDSA P-256 certificates valid at most 14 days, made by the server at start and
   pinned through `serverCertificateHashes` (R2-TRN-REC5, r2 lines 2700–2703; number N30). Hash pinning in production
   stays out: the VMs stay warm, so they are not the "short-lived ephemeral servers" R2-TRN-REC5 names
   (r6-infrastructure.md, Options §1, Q3, option C3, lines 426–427; r2-netcode.md, R2-TRN-OQ4, lines 3050–3054). The
   netcode proof test runs on the development certificates; the production form is needed from the first online
   playtest, after the security baseline's research. An alert fires when any VM's certificate has under 14 days left
   (proposed in R6-HOST-REC4's mark, r6 lines 697–699).
   *Carry-forward 4, four superseded sentences* (each still calls the wildcard R6's proposal; R6-HOST-REC4 holds):
   - r2-netcode.md lines 2704–2706, "R6-HOST-REC4 proposes one wildcard certificate instead, which puts one private key
     on every game VM; whether the security baseline accepts that is BASELINE-WHEN (below).", reads: "R6-HOST-REC4
     recommends a certificate per hostname too; one wildcard certificate, with one private key on every game VM, is an
     option the security baseline weighs (BASELINE-WHEN (b))."
   - r6-infrastructure.md line 47, "one wildcard certificate through ACME", reads: "a certificate per hostname through
     ACME; the wildcard certificate is an option the security baseline weighs."
   - r6-infrastructure.md lines 941–943, "the DNS API key the wildcard certificate job needs (R6-HOST-REC4); the
     wildcard certificate's private key, copied to every game VM", reads: "the DNS API key a dns-01 job would need
     (OQ-0006-5); the wildcard certificate's shared private key, only if the security baseline accepts option C2."
   - r6-infrastructure.md lines 1065–1066, "R6-HOST-REC4's one wildcard certificate departs from R2-TRN-REC5's ...",
     reads: "R6-HOST-REC4 recommends R2-TRN-REC5's certificate per game server hostname; the wildcard certificate,
     which would depart from it, is an option the security baseline weighs."
   *Why:* a certificate per hostname is R2-TRN-REC5's form and puts no shared private key on the game VMs; the
   wildcard's gain (adding a VM is a copy, not an issuance) is the baseline's to weigh (r6 lines 692–694). *Ids:*
   R6-HOST-REC4, R2-TRN-REC5, R2-TRN-OQ4, R6-HOST-OQ6. *Need:* VISION_V2 §6.1 (private matches on real servers) and
   VISION_V2 §4 (the four browsers need a trusted certificate per hostname in production) (r6 line 696).
8. **Costs** (prices read 2026-10-28, excluding VAT; [the start modes and the goal modes](../research/GLOSSARY.md#the-start-modes-and-the-goal-modes)).
   - *Start* (MODES-3 (a); assumption A-4: up to 2 rooms at once, 18 humans each, playtests about 4 hours a day, servers
     on all month): about 18 EUR a month lean (production CX33 8.99 + staging CX33 8.99 = 17.98 EUR, 0 USD) to about
     102 EUR + 5 USD reference (production CCX23 86.49 + staging CX33 8.99 + match records on Hetzner Object Storage
     6.49 = 101.97 EUR; 5 USD only if Worker code is needed); egress about 137 GB, inside the 20 TB
     (r6-infrastructure.md, Options §2, start scenario, lines 466–480).
   - *Goal* (12v12v12 or 24v24; A-5: a peak of 20 rooms of 48 humans, an average of 6 rooms, the fleet sized to the peak
     plus one VM for draining): at R6's 8 rooms per VM, about 361 EUR (a fixed CCX23 fleet of 4 VMs) or about 238 EUR
     (a fixed base plus VMs added by the hour, K2), each + 19–24 USD + Sentry's paid plan, match records included (lines
     482–494). **Changed from R6-COST-REC1:** at the 4 rooms per VM that A-2 names "the case to budget" until
     R2-ROOM-REC2 marks 1 and 2 pass (r6 lines 516–522), the fixed fleet is ceil(20 ÷ 4) + 1 = 6 CCX23, 6 × 86.49 = 518.94
     EUR, so about 534 EUR with the staging VM and the match records (518.94 + 8.99 + 6.49 = 534.42 EUR) + 19–24 USD +
     Sentry. The goal budget is stated at both densities until the netcode proof test measures rooms per VM
     (R6-COST-OQ1, r6 lines 880–882; R2-ROOM-REC2 mark 2 is ADR 0001's test). Egress stays inside the included traffic at
     either density: at 4 rooms, 4 × 3.156 GB × 730 h ≈ 9.2 TB per VM, under 20 TB (arithmetic on r6 lines 461–463).
   - *Fleet shape:* a fixed fleet (K1) first; K2 only once real concurrency data shows a clear peak; the shared CX33
     fleet (K3) only if the shared-vCPU measurement passes; each extra room of peak capacity costs 86.49 ÷ 8 ≈ 10.81 EUR
     a month at 8 rooms per VM, 21.62 EUR at 4 (r6-infrastructure.md, Recommendations §2, R6-COST-REC1, lines 703–717;
     scaling rule, lines 499–500).
   - *Cost per match:* every match record carries its room-hours and bytes sent, and a monthly bill check compares VM
     hours and traffic against the 20 TB included per VM (r6-infrastructure.md, Recommendations §2, R6-COST-REC2, lines
     718–725); Mustafa reads the invoice in his own account.
   - *CI runners (critic rank 33):* CI runs on GitHub's standard hosted runners, "free and unlimited on public
     repositories", with the free 10 GB cache (r5-toolchain-quality.md, Findings, Question 5, R5-CI-F17, F5, lines
     673–678 and 583–592; the repository is public, lines 913–914). No paid runner (GPU or macOS) and no self-hosted
     reference machine is proposed: their prices were not read (R5-CI-OQ5, R5-TEST-OQ4, r5 lines 1835–1837 and 1809), and
     self-hosted runners are "a cost and security question" (r3-rendering.md, Open questions, R3-MEASURE-OQ2, lines
     1384–1385); reference-machine runs stay by hand until Mustafa decides (OQ-0006-7).
   *Why:* compute dominates and is flat per VM; egress, the cost that grows with players, is zero on Hetzner inside the
   20 TB and about 0.063 USD per room-hour on a per-GB host (r6 lines 706–709). *Ids:* R6-COST-REC1, R6-COST-REC2,
   R6-COST-OQ1, R6-COST-OQ3, OQ4, OQ5, R3-MEASURE-OQ2, R3-XT-OQ5, R5-XT-OQ11, R6-XT-OQ7. *Need:* VISION_V2 §6.1 (rooms
   and private matches at the §3 sizes, r6 line 713) and §6.6 (the cost line in the record, r6 line 723); the CI runner
   costs: GLOSSARY §8, the foundation (CI gate v2) only.
9. **Logs and metrics.** Each room process writes `tracing` JSON lines to standard output; the supervisor adds VM and
   room ids and ships them; no line holds a token, a guest secret or a raw id (salted hashes where an id is needed, as
   R2-ROOM-REC3 mark 3 says) (r6-infrastructure.md, Recommendations §3, R6-OBS-REC1, lines 729–738). Metrics use the
   `metrics` facade with `metrics-exporter-prometheus`, scraped by a Prometheus on the staging VM to start, Grafana
   Cloud's EU stack once its ingestion path is read (OQ-0006-8); labels are bounded (VM, room slot 0–7, mode id,
   transport), never a player or match id; the first metrics are the [server tick time](../research/GLOSSARY.md#server-tick-time) histogram, overruns and dropped
   time (R1-TICK-REC4), per-phase time (R2-BUD-REC6), sessions by transport (the fallback share, R2-TRN-OQ7), bytes and
   datagrams sent and dropped, per-client budget overruns, relevance churn, joins refused by reason, room starts, ends
   and crashes, and process memory (R6-OBS-REC2, r6 lines 739–753; interface I-12). Logs and metrics share one log and
   metrics store once it is read; until then logs stay on the VM under part 11's 14-day cap (proposed). Tick phases
   are timed locally and exported as histograms (option T1); there is no distributed tracing (T2: Beta in Rust, and no
   §6 item asks for it) (r6-infrastructure.md, Options §3, Q3, lines 545–549). A Prometheus scrape needs an inbound
   port on every game VM, while the exporter can also push to a push gateway (r6-infrastructure.md, Findings §3,
   R6-OBS-F4, lines 279–284); the netcode proof test picks the path and checks the firewall rule it needs (part 14).
   *Why:* structured events at little cost; bounded labels keep the series near assumption A-8's 3,600 for 4 VMs,
   inside Grafana Cloud Free's 10k; logs come to about 0.8 GB a month at the goal's average, far under the free 50 GB;
   Prometheus' local store keeps 15 days and is not replicated, so metrics are lost with the VM, not the game
   (r6 lines 565–569 and 541; R6-COST-F1, r6 lines 219–224; R6-OBS-F5, r6 lines 285–290). *Ids:* R6-OBS-REC1, R6-OBS-REC2.
   *Need:* VISION_V2 §6.1 (per-room lifecycle and crashes) and VISION_V2 §4 ("Matches feel fair up to about 100 ms of
   ping": overruns, the server tick budget and the fallback share become visible) (r6 lines 734–735 and 749).
10. **Crash reports.** In the browser: Sentry's browser SDK with `wasmIntegration` and build ids from `wasm-split`;
    debug files uploaded by CI and never shipped (the upload job is ADR 0002's, critic rank 34); the organization
    created in Sentry's EU location; `userInfo` off, so the SDK does not send IP addresses. Tab crashes: Reporting API
    `crash` reports to our own endpoint on the lobby. On the server: panics go to Sentry's Rust SDK from the room
    process, and the supervisor records every abnormal room exit (r6-infrastructure.md, Recommendations §3,
    R6-OBS-REC3, lines 754–768). Mustafa creates the Sentry organization in his own account, choosing the EU location,
    which "can't be changed" later (R6-OBS-F9, r6 lines 318–326), after he has read Sentry's prices (OQ-0006-8); its upload
    token is a secret he adds (part 13). Sentry calls its Wasm support "still developing" (R6-OBS-F7, r6 lines 297–306),
    so whether it symbolicates wasm-bindgen Rust (R6-OBS-OQ3) and which browsers send `crash` reports (R6-OBS-OQ4) are
    spike marks: in the boundary spike if the EU organization exists by then, otherwise at the first online playtest's
    milestone. The gate v2's smoke test switches reporting off or allows its endpoints (row TL9; ADR 0002, rank 34). What
    client field data is measured is ADR 0003's (rank 26); it is stored under part 11's rules. *Why:* Wasm frames need
    debug files and build ids; tab crashes, such as running out of memory, are invisible to JavaScript but reach a
    reporting endpoint; the EU location is chosen once; the SDK sends IPs unless told not to (r6 lines 758–760). *Ids:*
    R6-OBS-REC3, R6-OBS-OQ3, R6-OBS-OQ4. *Need:* VISION_V2 §4 (four desktop browsers: crashes per browser are the
    evidence) and §6.1 (per-room crash reports) (r6 lines 763–764).
11. **Data protection.** Every store is in the EU (Hetzner EU, Sentry EU, a Grafana EU stack); no IP address, token or
    guest secret goes into logs, reports or match records; retention is capped: logs 14 days and metrics 15 days
    (proposed: the free tier's and Prometheus' defaults), match records kept while their content hash is still analysed
    (R6-TEL-OQ4's proposed default, r6 lines 917–919); the crash reports' cap stays open, because no page read shows
    whether Sentry lets an organization set its retention (r6-infrastructure.md, Recommendations §3, R6-OBS-REC4, lines
    769–780). Some Sentry data, such as user accounts, organization settings and access tokens, may be stored in the US
    whatever the choice (R6-OBS-F9, r6 lines 318–326). The legal side (lawful basis, privacy notice, what the law allows)
    is not researched: Mustafa decides it before any public playtest (OQ-0006-9). *Why:* the EU choice is one-way on
    Sentry and on Grafana; OWASP asks for bounded retention and no personal data in logs (r6 lines 773–774). *Ids:*
    R6-OBS-REC4, R6-OBS-OQ1, R6-TEL-OQ4. *Need:* no §6 item asks for it as a feature; it is the condition under which
    the stores of §6.1 and §6.6 may run (r6 lines 777–778).
12. **Balance telemetry.** Every match writes one record inside the room (options W2 and R-a), shaped as R6-TEL-REC1
    lists it: a header (random match id; record schema version; content hash and mode id; server build id; map id; team
    count, sizes and faction per team; private flag; start time in UTC to the minute; duration; end reason; winner and
    final points; humans and bots per team; room-hours and bytes sent); a timeline (points, money and tech tier per team
    every 10 s of game time, 120 samples per team in a 20-minute match); tick-stamped events (uplink caches picked up,
    placed, stolen and cashed out, with from team, to team and countdown left; harvester deliveries and kills;
    structure and HQ kills; tech tiers reached; kills with killer and victim slot, killer and victim unit id, source
    weapon or ability id and assisting slots; unit and vehicle purchases); a per-slot section (human or bot, team,
    faction, time spent as each unit id, kills, deaths, assists); and moments stored as tagged events that point at the
    events that made them; never account or guest ids, display names, IP addresses or tokens (r6-infrastructure.md,
    Recommendations §4, R6-TEL-REC1, lines 784–811). **Changed:** the per-slot section also carries the upgrade-track
    fields the decision map gives this record (part j, J5): picks per track and level, offers shown, looted weapons,
    time per weapon type. No research finding covers them (UPGRADE-SCOPE has none), so they are proposed, their shape
    follows ADR 0005's track data, and carry slots, ammo for looted weapons and levelling speed stay Mustafa's open
    questions in ADR 0005. ADR 0004's sim emits the events and recognizes the moments; ADR 0001's room writes the
    record at Ended, before Closed (R2-ROOM-REC1); ADR 0005 owns the ids and the content hash; the moment rules and
    thresholds are a design question (R6-TEL-OQ3) (seam S6). Records are stored as compressed JSON-lines files in
    Hetzner Object Storage in the game servers' location, uploaded by the supervisor after the room exits, with a
    schema version in every record and a Parquet compaction job only once a query needs it; a crashed room leaves a stub
    record marked incomplete (r6-infrastructure.md, Recommendations §4, R6-TEL-REC2, lines 812–827). They are analysed
    in batches on the staging VM, always grouped by content hash and mode id, with the sample size beside every rate;
    the derived numbers (win rate per faction and base position, unit pick and win rates, lead changes, comeback rate,
    steals and late steals, P(win | leading at minutes 5 and 10)) are proposals for the game-designer to refine
    (R6-TEL-REC3, r6 lines 828–842); no live balance dashboard (option N2, r6 lines 599–600). Whether bot and private matches
    count in rates is Mustafa's (R6-TEL-OQ2); the record tags both. Kill-cams, replays and highlights (§6.8 `[LATER]`)
    would need a full replay, which is not built (option W3, r6 lines 577–578); the tick-stamped events and ADR 0004's
    recorded matches keep them possible. *Why:* each §6.6 number and each §6.8 moment is a function of the timeline
    and the events; legacy shows the trap of untagged kill sources (row GE15) and missing assists (row GC18); writing
    at Ended reuses R2-ROOM-REC1's guarantee; at the goal the volume is about 13 GB a month uncompressed, inside 1 TB
    for 6.49 EUR (r6 lines 800–802 and 815–816). *Ids:* R6-TEL-REC1, R6-TEL-REC2, R6-TEL-REC3, R6-TEL-OQ1, R6-TEL-OQ5.
    *Need:* VISION_V2 §6.6 and §6.8, with §6.5 and §6.1 (r6 line 806); the upgrade-track fields: UPGRADE-SCOPE only.
13. **The security baseline: open, with interim positions (BASELINE-WHEN (b)).**
    [The security baseline](../research/GLOSSARY.md#the-security-baseline) ("content security policy, dependency
    audits, anti-cheat basics") is researched by security-auditor in its own fresh session before the first online
    playtest. Until then each item that waits on it (r6-infrastructure.md, Open questions §5, R6-SEC-OQ1, lines
    926–945; critic rank 18) holds this interim position:

    | # | Item that waits | What the baseline decides | Interim position | Owner draft |
    |---|---|---|---|---|
    | B1 | Dependency audits (R5-GATE-OQ4; R1-RR-OQ7; R2-TRN-REC1's version floors) | whether an advisory blocks a pull request, and who owns that policy | [the advisories check](../research/GLOSSARY.md#the-advisories-check) runs on a schedule and on every pull request that changes `Cargo.lock`, outside the PR gate (R5-GATE-REC1, r5-toolchain-quality.md lines 1671–1674); it keeps the version floors current | 0002 (the job); 0006 (the policy, open) |
    | B2 | Production certificates (R6-HOST-REC4, R2-TRN-REC5) | the wildcard against a certificate per hostname; the DNS API key | a certificate per hostname (part 7) | 0006 |
    | B3 | Content security policy (R5-PREV-REC4; R2-ROOM-OQ11) | the CSP in the production header set | none written yet; no third-party script: three.js is bundled and same-origin (part 1); the header sets take the CSP once written | 0006 (the content); 0002 (the generation) |
    | B4 | Where the guest secret lives (R2-ROOM-OQ11) | HttpOnly cookie or local storage | R2-ROOM-REC8's HttpOnly, Secure, SameSite=Strict cookie on the lobby's origin, which the security review's SEC-GUEST recommends; Mustafa and the Advisor decide (r2-netcode.md lines 3112–3116) | 0001 |
    | B5 | The lobby's token-signing keys (R2-ROOM-OQ2, R2-ROOM-OQ9) | how and where they are stored | only in the lobby host's environment, added by Mustafa | 0001 (scheme); 0006 (storage) |
    | B6 | The `textContent` rule (R5-TSLINT-OQ5) | which lint rule enforces it | CLAUDE.md's code rule ("Dynamic or player-provided text: `textContent`, never `innerHTML`") holds by review | 0002 |
    | B7 | Anti-cheat basics: the view-time offset and aim log (R2-SYNC-REC5), Origin-less requests tagged (R2-TRN-REC1), the reader-client and byte-audit tests (R2-INT-REC2, REC5) | the detection policy | built as ADR 0001 designs them; no detection policy | 0001 |
    | B8 | R1-DATA-REC7's allow list and fuzz corpus; COOP as an XS-Leaks guard (R1-BND-REC5) | whether they suffice | as ADR 0005 and ADR 0002 design them; production non-isolated at the start | 0005; 0002 |
    | B9 | The tokens CI holds (the client host's deploy token, the Sentry upload token) and a multiplayer preview's binary delivery (R6-XT-OQ4) | their scope and handling | each in its own GitHub environment, added by Mustafa (below) | 0002 (the jobs); 0006 (the scope) |
    | B10 | The staging VM runs pull-request-built native code (R6-XT-OQ4, "PR-built native code on a VM") beside the Prometheus store (part 9) | whether they share a VM | they share it: fork pull requests get no preview (R5-PREV-REC1), so only this repository's pull requests run there | 0006 |
    | B11 | Self-hosted CI runners (R3-MEASURE-OQ2) | whether any is allowed | none (part 8) | 0006 |

    Previews are settled by PREVIEW-WALL (a) (part 2). **Secrets:** every secret lives in a GitHub environment (for
    CI) or in a VM's environment, never in the repository, and Mustafa adds each one; agents never add them
    (r6-infrastructure.md, R6-SEC-OQ1, lines 944–945; r5-toolchain-quality.md, Pointers, lines 2104–2106). The client
    host's deploy token is an account-scoped "Cloudflare Pages, Edit" API token, the scope the research read
    (r5-toolchain-quality.md, Findings, Question 6, R5-PREV-F10, lines 823–830; whether it can be narrowed to one
    project was not read), held only by the deploy job's environment `preview` (R5-PREV-REC1, r5 lines 1588–1604; ADR
    0002's job). *Ids:* BASELINE-WHEN, R6-SEC-OQ1, R1-RR-OQ7, R5-GATE-OQ4, R5-TSLINT-OQ5, R2-ROOM-OQ11, R2-XT-OQ9,
    R5-XT-OQ5, R6-XT-OQ3, R1-BND-OQ5, R1-XT-OQ17, R6-XT-OQ9. *Need:* no §6 item names it; it is the condition for the
    first online playtest (BASELINE-WHEN (b)); the deploy token: GLOSSARY §8, the foundation only.
14. **Its spike and its operations marks.** The preview hosting check (small, on a test project in Mustafa's
    Cloudflare account, before the foundation's preview step) proves part 1 and PREV-DELETE; the netcode proof test
    (ADR 0001's) carries this record's operations marks: rooms per VM on both machines, egress per match-hour, the
    spike's own bill, logging cost, the metrics and their path, and the firewall. Both are listed under "What a spike
    must prove". *Need:* as parts 1, 3, 5, 8 and 9.

## Options considered

Every option below comes from the research's Options sections (cited per question). "From zero" in the effort column
means legacy has no host, server, domain or certificate to migrate (inventory row CD2; r6-infrastructure.md, Findings
§1, legacy facts, lines 207–208), so the option is new work of its own size. "None" in the look and feel column means
the option serves the same build and does not touch what a player sees.

### Q1. Where the client and the previews are served (r6-infrastructure.md, Options §1, Q1, lines 401–409; r5-toolchain-quality.md, Options, Question 6, hosts table, lines 1120–1128)

| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Effect on the look and feel | Effort from the current code | Pros and cons |
|---|---|---|---|---|---|---|---|
| **A. Cloudflare Pages, Workers static assets the alternative (proposed)** | Low: a prebuilt directory deployed from Actions by wrangler-action with an account-scoped Pages Edit token (R5-PREV-F10, r5 lines 823–830) | 0 USD on the Free plan (inferred from its name, r6 line 476); Workers Paid 5 USD a month minimum, only if Worker code is needed (R6-HOST-F5, r6 lines 87–92) | Unlimited previews; 20,000 files per site (Free) and 25 MiB per file; 500 builds a month on Free (R6-HOST-F3, r6 lines 72–79) | Previews public by default, an access policy limits them to the account; `noindex` by default (R5-PREV-F12, r5 lines 838–844); a branch's latest deployment cannot be deleted (R5-PREV-F13, r5 lines 845–849) | None; Mustafa's Brave check needs one login per session once the wall is on | From zero | **Pros:** headers from `_headers` (R6-HOST-F1, F2, r6 lines 60–71); a link per branch; meets R5-PREV-REC2's list but deletion. **Cons:** PREV-DELETE unresolved; `_headers` does not apply to code-generated responses (F2) |
| B. Netlify | Low on our side, but its natural path builds on Netlify's servers (path P5, r5 line 1118) | Prices not read (outside the allow list, r6 lines 105–106) | Not read | Password option, `noindex` (R5-PREV-F15, r5 lines 859–866); sensitive variables kept from fork builds (R5-PREV-F16, r5 lines 867–872) | None | From zero | **Pros:** `_headers` like Pages (R6-HOST-F7, r6 lines 100–106). **Cons:** one header set for every deploy context, so isolated previews need a build trick (F7); P5 "breaks 'the preview is the tested build'" (r5 line 1118) |
| C. Vercel | Low; `vercel deploy --prebuilt` with three secrets (R5-PREV-F17, r5 lines 873–878) | Hobby is "non-commercial, personal use only"; Pro 20 USD per seat per month (R6-HOST-F9, r6 lines 111–115) | 100 GB of transfer on Hobby (F9) | Standard Protection on every deployment, viewers need a Vercel account; password protection 20 USD a month per project on Pro (R6-HOST-F10, r6 lines 116–121) | None | From zero | **Pros:** headers per path in `vercel.json` (R6-HOST-F8, r6 lines 107–110). **Cons:** commercial use forces Pro |
| D. GitHub Pages | Low | 1 GB per site, 100 GB a month soft limit; not allowed as free hosting for an online business (R6-HOST-F11, r6 lines 122–126) | No per-PR site natively (inferred, r5 line 1128) | Public only | None | From zero | **Cons:** cannot set COOP and COEP (R6-HOST-F12, r6 lines 127–132), so no isolated preview |
| E. Our own web server on the game VMs | High: "Every preview, cache and certificate is ours to run" (r6 line 409) | VM traffic (R6-HOST-F15, F16) | Ours to build | Ours to secure | None | From zero | **Cons:** generic work no §6 item asks for (r6 line 409) |

### Q2. Who may open a preview, and what the access wall covers
- Public previews that reach the staging game server (PREVIEW-WALL options (b) and (c)): settled by PREVIEW-WALL (a).
- Once the wall is on, what it covers (not weighed by the research; part 2's interim position):

| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Effect on the look and feel | Effort from the current code | Pros and cons |
|---|---|---|---|---|---|---|---|
| **(i) Every preview of the project behind the wall from the first multiplayer preview (proposed interim)** | Low: one project setting (R5-PREV-F12's "restrict viewing project deployments to your Cloudflare account", r5 lines 838–844) | 0 (no price read for the policy) | Same as Q1 A | Every preview behind a login | Mustafa logs in once per Brave session (R5-PREV-REC5) | From zero | **Pros:** meets PREVIEW-WALL (a) with one switch. **Cons:** offline previews lose their open link too |
| (ii) Multiplayer previews in a second Pages project | not researched | not researched | not researched | not researched | not researched | not researched | Named so OQ-0006-4 can weigh it |
| (iii) Cloudflare Access per hostname | not researched for Pages (read for Workers previews only, R5-PREV-F14, r5 lines 850–858) | not researched | not researched | not researched | not researched | not researched | Named so OQ-0006-4 can weigh it |

### Q3. EU game servers (r6-infrastructure.md, Options §1, Q2, lines 411–418)

| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Effect on the look and feel | Effort from the current code | Pros and cons |
|---|---|---|---|---|---|---|---|
| **G1. Hetzner Cloud, dedicated vCPU (CCX23 as the reference server, proposed)** | Medium: our own VMs, a supervisor, a firewall; no load balancer, the lobby hands out host and port | CCX23 86.49 EUR a month (0.1386 an hour), 20 TB included, 1 EUR per extra TB (R6-HOST-F15, F16, r6 lines 150–163) | "highly predictable resources", sold for gaming servers (R6-HOST-F14, r6 lines 144–149) | Firewall filters UDP, stateful, drops what no rule allows (R6-HOST-F18, r6 lines 169–174) | Fair server ticks serve VISION_V2 §4's "fair up to about 100 ms of ping" | From zero | **Pros:** predictable tick; egress free at full load. **Cons:** EU plans showed "currently unavailable" on 2026-10-28 (R6-HOST-F20, r6 lines 180–182); a single provider |
| **G2. Hetzner Cloud, shared vCPU (CX33, measured beside G1)** | As G1 | CX33 8.99 EUR a month (0.0144 an hour) (F15) | "designed for variable usage patterns rather than sustained high workloads" (F14): server tick [p99](../research/GLOSSARY.md#time-statistics) must be measured | As G1 | Noisy neighbours could break the server tick budget (r6 line 416) | From zero | **Pros:** 9.6 times cheaper. **Cons:** unproven for a fair tick |
| G3. Fly.io Machines | UDP not read (outside the allow list) | performance-2x 66 USD a month plus 0.02 USD per GB of egress (R6-HOST-F22, r6 lines 186–192) | Dedicated-ness not read | Not read | Not read | From zero | **Cons:** 3.16 GB per 24v24 match-hour makes egress about 369 USD a month per loaded host (r6 lines 461–464) |
| G4. Amazon GameLift Servers | Managed fleets, not read in detail | c6a.large 0.099 USD an hour on demand, 0.03 on Spot, Ohio; bandwidth free on generation 6 and later; EU rates not read (R6-HOST-F23, r6 lines 193–197) | Managed scaling, not read | Not read | Not read | From zero | **Cons:** lock-in to its server SDK (not read) |
| Hetzner dedicated root servers; Netcup, OVHcloud, Scaleway | not researched (R6-HOST-OQ1, OQ4, r6 lines 852–855 and 862–865) | not researched | not researched | not researched | not researched | not researched | Read only if OQ-0006-1 rules Hetzner Cloud out |

### Q4. The fleet's shape (r6-infrastructure.md, Options §2, K1–K4, lines 502–512)

| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Effect on the look and feel | Effort from the current code | Pros and cons |
|---|---|---|---|---|---|---|---|
| **K1. A fixed fleet sized to the peak (proposed first)** | Low: "nothing to automate" | Goal: 4 × 86.49 = 345.96 EUR of game VMs at 8 rooms per VM; 6 × 86.49 = 518.94 EUR at 4 (part 8) | Pays for idle hours | Fewest moving parts | None | From zero | **Pros:** simplest. **Cons:** idle hours |
| K2. A fixed base plus VMs added by the hour | Medium: the provider's API, a draining supervisor, an unmeasured VM start-up time | 222.88 EUR of game VMs at the goal, about 36 % less (r6 lines 487 and 503) | Hourly round-up makes VMs per match wasteful (R6-HOST-F17) | API credentials (a secret; inferred) | None | From zero | **Cons:** needs real concurrency data first |
| K3. Shared-vCPU VMs | As K1 | About 9.6 times cheaper per VM | Only if server tick p99 on shared hosts passes R2-ROOM-REC2's marks at several times of day | As K1 | Risk to the fair tick | From zero | **Cons:** unproven |
| K4. Per-GB egress hosts or managed fleets | Varies | Fly's egress alone about 276 USD a month at the goal's average (r6 line 496) | As G3, G4 | Not read | Not read | From zero | **Cons:** egress costs more than a Hetzner host per loaded host |

### Q5. Production certificates (r6-infrastructure.md, Options §1, Q3, lines 420–427; R6-HOST-OQ6, lines 869–874)

| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Effect on the look and feel | Effort from the current code | Pros and cons |
|---|---|---|---|---|---|---|---|
| **C1. A certificate per hostname through ACME http-01 (one of part 7's two issuance paths)** | Low per VM | No price read | One issuance per new VM | No shared key; TCP port 80 open on every game VM (R6-HOST-F24, r6 lines 198–204) | None | From zero | **Pros:** R2-TRN-REC5's form. **Cons:** a port beyond the two game ranges |
| **A certificate per hostname through ACME dns-01 (the other path, R6-HOST-REC4 r6 line 685)** | Medium: a DNS provider API | No price read | One issuance per new VM | No shared key; a DNS API key (a secret) | None | From zero | **Pros:** no TCP 80. **Cons:** a DNS secret to guard |
| C2. One wildcard certificate per region name through dns-01 | One renewal job, copied to every VM | No price read | Adding a VM is a copy | One private key on every game VM: weighed by the security baseline (BASELINE-WHEN (b)), not here | None | From zero | **Pros:** simple scaling. **Cons:** departs from R2-TRN-REC5 |
| C3. Self-signed, hash-pinned, short-lived | Low | 0 | Development and previews only | Pinning rules are strict (R2-TRN-REC5) | None | From zero | Production only "if R6 picks short-lived ephemeral servers"; G1 and G2 keep VMs warm, so not (r6 lines 426–427) |
| tls-alpn-01 (RFC 8737: "This connection MUST use TCP port 443.") | not researched beyond that quote (r6 lines 871–874) | not researched | not researched | not researched | not researched | not researched | Named for OQ-0006-5's port plan |

### Q6. The starting location (r6-infrastructure.md, Options §1, Q4, lines 429–432)
Nuremberg (`nbg1`) or Helsinki (`hel1`), in Hetzner's one EU zone; Falkenstein (`fsn1`) only if the plans are offered
there (their product pages list "NBG1 HEL1", R6-HOST-F13, r6 lines 136–143). No latency source was read for any of them,
so the table's columns are equal but for latency, which a measurement decides (OQ-0006-2): complexity, cost and effort
are the same in all three; security the same; the look and feel differ only through round-trip time.

### Q7. Logs, metrics and traces (r6-infrastructure.md, Options §3, Q1–Q3, lines 532–549)

| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Effect on the look and feel | Effort from the current code | Pros and cons |
|---|---|---|---|---|---|---|---|
| **L1. `tracing` JSON lines to standard output, shipped by the supervisor (proposed)** | Low | About 0.8 GB of logs a month at the goal's average (A-9, r6 lines 568–569) | A formatted message does not allocate by default (R6-OBS-F1, r6 lines 262–269) | Exclusions per OWASP (R6-OBS-F12, r6 lines 338–344) | None | From zero | **Pros:** any log store reads JSON lines. **Cons:** a store still to choose |
| L2. OpenTelemetry logs over OTLP | More crates | As L1 | Stable (R6-OBS-F3, r6 lines 275–278) | As L1 | None | From zero | **Cons:** nothing in §6 needs its cross-service view |
| **M1. `metrics` + Prometheus exporter, scraped by a Prometheus on the staging VM (proposed to start)** | Low | 0 extra; about 0.6 GB of disk at A-8 (r6 line 541; A-8 r6 lines 565–567) | 15 days by default, 1–2 bytes per sample (R6-OBS-F5, r6 lines 285–290) | Not replicated: metrics lost with the VM, not the game; a scrape needs an inbound rule (R6-OBS-F4) | None | From zero | **Pros:** read and small. **Cons:** one VM holds it |
| **M2. The same exporter into Grafana Cloud's EU stack (once its path is read)** | Low | Free under 10k series (R6-COST-F1, r6 lines 219–224); Pro 19 USD plus usage (R6-COST-F2, r6 lines 225–228) | Grafana's service | Region fixed at creation (R6-OBS-F11, r6 lines 332–337) | None | From zero | **Cons:** how data reaches it was not read (R6-OBS-OQ2) |
| M3. OpenTelemetry metrics over OTLP | More dependencies in the room binary | As M2 | Stable (F3) | As M2 | None | From zero | **Cons:** more crates for the same series |
| **T1. Tick phases timed locally, exported as histograms (proposed)** | Low | 0 | Serves R2-BUD-REC6's separate measurements | None | None | From zero | **Pros:** what the budgets need |
| T2. Distributed tracing with OpenTelemetry | Medium | — | Beta in Rust (F3) | — | None | From zero | **Cons:** one room process has no cross-service path to follow: a generic feature no §6 item asks for (r6 lines 548–549) |

### Q8. Client errors and crashes (r6-infrastructure.md, Options §3, Q4, lines 551–556)

| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Effect on the look and feel | Effort from the current code | Pros and cons |
|---|---|---|---|---|---|---|---|
| **Sentry browser SDK with `wasmIntegration`, debug files uploaded at build (C1, proposed)** | Medium: a CI upload job (ADR 0002) | Free tier, then per error (R6-COST-F3, r6 lines 229–235; base prices not read) | Wasm frames with symbols, "still developing" (R6-OBS-F7, r6 lines 297–306) | EU storage chosen at creation; IP off by setting (R6-OBS-F9, F10, r6 lines 318–331); a CI secret | None | From zero | **Pros:** Rust function names in traps. **Cons:** an unproven Wasm path |
| **Reporting API `crash` to our own endpoint (C2, proposed beside C1)** | Low | 0 (our server) | Tab crashes JavaScript cannot see (R6-OBS-F8, r6 lines 307–315) | Our own EU store | None | From zero | **Pros:** catches out-of-memory crashes. **Cons:** no stack; browser support varies (F8) |
| C3. Our own `error` and `unhandledrejection` handler | Medium: symbolication is ours | Our work | Wasm traps as raw offsets | Our own | None | From zero | **Cons:** rebuilds what C1 gives |

### Q9. Data protection (r6-infrastructure.md, Options §3, Q5, lines 558–562)
- **P1. EU-only storage for every store, no IPs, tokens or guest secrets in logs or reports, bounded retention
  (proposed):** low complexity (settings at creation); no extra cost read; one-way choices on Sentry and Grafana; none
  for the look and feel; from zero. **Pros:** the condition for §6.1's and §6.6's stores. **Cons:** the choice cannot
  be undone later.
- P2. A provider's default region and settings: US storage for Sentry unless chosen at creation (R6-OBS-F9), the IP
  sent by default (R6-OBS-F10). **Cons:** fails P1's rules, and the Sentry region cannot be changed afterwards.

### Q10. The match record: what it holds, how it is recorded, stored and analysed (r6-infrastructure.md, Options §4, Q1–Q4, lines 573–600)

| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Effect on the look and feel | Effort from the current code | Pros and cons |
|---|---|---|---|---|---|---|---|
| W1. Results only | Low | Smallest | — | — | None | From zero; TL13 holds the prototype counters | **Cons:** answers none of "who led", "swings" or "steals" |
| **W2. Results, a points timeline, objective and kill events, per-slot unit use (proposed)** | Medium | About 1 MB uncompressed per 24v24 match (A-10, r6 lines 603–605) | Under 1 % of server tick p99 (R6-TEL-REC1's mark) | No player ids (R6-TEL-REC1) | None | From zero (TL13's metrics carry over) | **Pros:** every §6.6 number derivable. **Cons:** a schema to keep versioned |
| W3. A full replay | High | "orders of magnitude larger" | — | — | None | From zero | A §6.8 `[LATER]` feature: out of scope (r6 lines 577–578) |
| **R-a. A recorder inside the room (proposed)** | Low: no new path through the sim | 0 | A bounded buffer | A crash loses the buffer (stub record, R6-TEL-REC2) | None | From zero | **Pros:** reuses R2-ROOM-REC1's write at Ended |
| R-b. A live telemetry stream to a collector | Higher | A collector | — | A collector outage loses data mid-match | None | From zero | **Cons:** more moving parts |
| **S1. Compressed JSON lines in Hetzner Object Storage (proposed), S2 Parquet compaction only when needed** | Low | 6.49 EUR a month with 1 TB (R6-COST-F5, r6 lines 241–246) | About 13 GB a month at the goal (A-11, r6 lines 606–607) | EU, next to the servers | None | From zero | **Pros:** any tool reads it |
| S3. Cloudflare R2 instead | Low | About 1 USD a month after the free 10 GB (R6-COST-F4, r6 lines 236–240) | As S1 | Region and jurisdiction not read | None | From zero | **Cons:** EU location unverified |
| S4. A SQL database | not researched (r6 line 593) | not researched | not researched | not researched | not researched | not researched | Needs a read first |
| S5. A hosted analytics warehouse | not researched (BigQuery's page truncated, r6 line 594) | not researched | not researched | not researched | not researched | not researched | "Generic; not needed at this volume" |
| **N1. Batch analysis per content hash and mode id (proposed)** | Low | 0 (staging VM) | A month of goal records in under 10 minutes (R6-TEL-REC3's mark) | Records hold no player ids | None | From zero | **Pros:** comparable within one tuning version |
| N2. Live balance dashboards | Medium | — | — | — | None | From zero | **Cons:** no §6 item asks for live balance data |

### Q11. Where the lobby runs (r2-netcode.md, Open questions, R2-ROOM-OQ9, lines 3104–3105)
R2 names two forms, "Same binary as the supervisor or a separate service", and hands the choice and the store to R6;
R6 neither hosts nor costs it (R6-XT-OQ1, r6 lines 991–997). Neither form was weighed by any topic, so every column is
"not researched" for both, and neither is the proposed decision (part 6, OQ-0006-3).

### Q12. When the security baseline is researched
Researching it before this record, or leaving it as one shared question: settled by BASELINE-WHEN (b).

### Q13. CI runners (critic rank 33; r5-toolchain-quality.md, R5-CI-F17, R5-CI-OQ5, R5-TEST-OQ4; r3-rendering.md, R3-MEASURE-OQ2)

| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Effect on the look and feel | Effort from the current code | Pros and cons |
|---|---|---|---|---|---|---|---|
| **Standard GitHub-hosted runners and the free cache (proposed)** | Low | "free and unlimited on public repositories"; cache free to 10 GB, more needs a Pro, Team or Enterprise account (R5-CI-F17, F5, r5 lines 673–678 and 583–592) | ubuntu-24.04: 4 CPUs, 16 GB, no GPU (F17) | GitHub's isolation; today's hygiene carries over (row KT8) | WebGPU not covered in CI (R5-CI-OQ5) | Today's CI already runs on them (row KT8) | **Pros:** 0 cost. **Cons:** no GPU, no real Safari |
| Larger paid runners (GPU; macOS for real Safari) | Low | Price not read (R5-CI-OQ5, r5 lines 1835–1837; R5-TEST-OQ4, r5 line 1809) | GPU or macOS | GitHub's | WebGPU or Safari covered | Small | **Cons:** unpriced; Mustafa's call |
| Self-hosted reference machines | not researched: "a cost and security question" (R3-MEASURE-OQ2, r3 lines 1384–1385) | not researched | not researched | not researched | not researched | not researched | Named for OQ-0006-7 |

## Trade-off analysis

- **The client host (Q1).** Pages wins because it is the one host the research read that meets every need of
  R5-PREV-REC2's list but one, at 0 USD on its Free plan: headers from `_headers`, a link per branch, `noindex` by
  default, an access policy and deletion through Wrangler (R6-HOST-REC1, r6 lines 627–634). The strongest argument
  against it is that very exception: "The latest deployment for a branch cannot be deleted" (R5-PREV-F13), so a
  closed pull request's last link may stay reachable. Under PREVIEW-WALL (a) that link sits behind the wall if the
  preview reached the staging server, and the preview hosting check either settles it or sends the host to Workers
  static assets with the same `_headers` file.
- **Preview privacy (Q2).** Putting every preview of the project behind the wall from the first multiplayer preview
  meets PREVIEW-WALL (a) with one setting. Against it: offline previews lose their open link too, and Mustafa logs in
  once per Brave session for every check (R5-PREV-REC5). OQ-0006-4 lets him weigh a split.
- **Game servers (Q3, Q4).** Dedicated vCPUs win because the fair tick needs "highly predictable resources", and
  Hetzner's included 20 TB makes egress, the cost that grows with players, zero even at full load (R6-HOST-F14, F15;
  r6 lines 461–464). The shared CX33 is measured beside it because it is 9.6 times cheaper. The strongest argument against
  Hetzner is that its EU plans showed "currently unavailable" on 2026-10-28 (R6-HOST-F20) and that it is a single
  provider: OQ-0006-1 makes that a check before the netcode proof test, not a surprise during it. A fixed fleet (K1)
  wins on simplicity; against it, K2 would save about 36 % of the goal's game-VM cost (r6 line 503), which is worth
  automating only once real concurrency data shows a peak.
- **The goal budget at two densities (part 8).** Stating both 361 EUR (8 rooms per VM) and 534 EUR (4 rooms per VM)
  is less tidy than R6-COST-REC1's single range, but A-2 itself says 8 rooms per VM is "a density to measure, not one
  the server tick budget guarantees" (r6 lines 516–522): at the server tick budget's ceiling, 50 % of one vCPU per room,
  two rooms fill a vCPU before any [transport runtime time](../research/GLOSSARY.md#transport-runtime-time). The proof test's measurement replaces both figures.
- **Certificates (Q5).** A certificate per hostname wins as the interim position because it is R2-TRN-REC5's form and
  puts no shared private key on the game VMs; against it, each new VM needs an issuance and either TCP port 80 open or
  a DNS API key (R6-HOST-F24). The wildcard's gain, "adding a VM a copy, not an issuance" (r6 line 694), is the security
  baseline's to weigh.
- **Logs, metrics and crash reports (Q7, Q8).** Small, read tools win (`tracing`, the Prometheus exporter, Sentry):
  each is free or cheap at the start and keeps data in the EU. Against them: Prometheus on the staging VM is one
  unreplicated store (metrics are lost with the VM, not the game: r6 line 541), and Sentry's Wasm support is "still
  developing" with its base prices unread, while its EU choice is permanent (R6-OBS-F7, F9; R6-COST-F3).
- **Telemetry (Q10).** W2 recorded inside the room (R-a) wins because every §6.6 number is a function of the
  timeline and the events, with no new path through the sim (r6 lines 575–583). Against it: a crashed room loses its
  in-memory buffer, so crashed matches leave only a stub record (R6-TEL-REC2), and the upgrade-track fields rest on no
  research finding.
- **The security baseline left open (Q12).** Settled by BASELINE-WHEN (b). Its cost is that the CSP, the guest
  secret's home, key storage and the advisories policy have only interim positions until the first online playtest;
  no player reaches a game server before then except through a multiplayer preview behind the access wall (part 2).
- **What breaks if a provider is down** (inferred from the parts above, not measured): Cloudflare down, no new client
  or preview loads; Hetzner Cloud down, no online match runs, while local play with bots still runs in the Worker
  (SLICE-ORDER condition 1); object storage down, records wait on the VM's disk and upload later (R6-TEL-REC2, lines
  817–818); Sentry or Grafana down, reports and metrics are lost for that time and the game runs on.

## Consequences

- **Easier:** every server mark in ADR 0001 and ADR 0004 is measured on one named machine (N34, I-15); a preview link
  per pull request reaches Mustafa's Brave from the foundation on; cost per match is a query over the match records,
  not a guess (R6-COST-REC2); balance work starts from one versioned record per match, comparable within a content
  hash (R6-TEL-REC3).
- **Harder:** Mustafa holds accounts with Cloudflare and Hetzner before the first spikes (a test project on Cloudflare Pages
  and a Hetzner console check), and later with Sentry and Grafana; every multiplayer preview needs his login once per
  Brave session; each production game VM needs a certificate issuance path (OQ-0006-5); the lobby still has no place
  in the budget (OQ-0006-3).
- **Revisit when:** Hetzner's EU plans are not orderable (OQ-0006-1); PREV-DELETE fails on Pages (move to Workers
  static assets); the netcode proof test measures rooms per VM (fix the goal budget); the shared CX33 passes (K3) or
  real concurrency data shows a peak (K2); the security baseline lands (certificates, CSP, the advisories policy, the
  guest secret, key storage); a later design routes many rooms through one address (a UDP load balancer, R6-HOST-OQ3,
  r6 lines 859–861); production isolation is adopted (OQ-0006-12); Mustafa's data-protection read is done (OQ-0006-9);
  Grafana's ingestion path or Sentry's prices are read (OQ-0006-8).
- **Hard constraints in CLAUDE.md that change** (for v2 code only, once this record is accepted; spikes are exempt now
  under SPIKE-SCOPE (a); nothing in CLAUDE.md or `.claude/` changes in this PR):
  - "the game runs by double-clicking index.html (file://)": the v2 player build is served over HTTPS from the client
    host (part 1); ADR 0002 records the end of the double-click player build (decision map part j, J2).
  - "Persistence only through `KVStore`": v2 adds server-side stores outside the client (match records in EU object
    storage, part 12; the lobby's store, part 6), under part 11's data rules.
  - "No external assets": ends with the legacy engine under LOOK_V2 §8; the client host serves authored model files,
    each under its 25 MiB per-file limit (part 1).
  - "Three.js r128 UMD": v2 has no CDN script tag (part 1); which three.js it bundles is ADR 0003's.
- **Legacy inventory rows** (the inventory file is not edited; this record changes no mark, and none of its rows is
  "look brief pending": GP5, PG13, GP11 and GE12, which R6 cites as legacy facts, are ADR 0003's presentation rows):

  | Row id | Mark now | Proposed mark | Why |
  |---|---|---|---|
  | CD2 (KVStore, LEGACY_INVENTORY.md:83) | port to TypeScript | port to TypeScript (no change) | client storage keeps settings and a guest cache; server-side stores are new (parts 6, 12) |
  | CD3 (window.storage backend, :84) | drop | drop (no change) | v2 is a hosted client (part 1) |
  | PG19 (three.js r128 CDN tag, :377) | drop | drop (no change) | three.js bundled and served same-origin from the client host; the CSP waits for the security baseline (parts 1, 13) |
  | KT8 (gate.yml, :417) | redesign | redesign (no change) | the CI hygiene carries over; preview links get this record's host and token scope; the jobs are ADR 0002's |
  | TL9 (smoke test, :394) | redesign | redesign (no change) | its network rule must allow the preview's own origins and the report endpoints, or switch reporting off (parts 1, 10) |
  | TL13 (sim.js, :398) | redesign | redesign (no change) | its metrics carry into the match record (part 12) |
  | TL15 (perfbots.js, :400) | redesign | redesign (no change) | the Rust tick bench runs on the reference server (part 5) |
  | GE15 (career, :301) | redesign | redesign (no change) | kill events carry their source id (part 12); account progression stays `[LATER]` |
  | GC18 (combat leftovers, :257) | drop | drop (no change) | assists come from the record's assisting slots, not from legacy's stubs (part 12) |

## Open questions

- **OQ-0006-1. Can Mustafa order Hetzner's CCX23 and CX33 in an EU location?**
  - *Why it matters:* every native server mark and the netcode proof test run on the reference server; on 2026-10-28
    every EU plan showed "This product is currently unavailable. Please check back later." (R6-HOST-F20, r6 lines 180–182).
  - *Options:* (a) orderable: the reference server stands; (b) a real shortage: read the fallbacks first, Fly.io's UDP
    page, GameLift's EU prices and server SDK, Hetzner's dedicated root servers, and Netcup, OVHcloud or Scaleway (none
    read: R6-HOST-OQ1, OQ4, r6 lines 852–855 and 862–865), then name a new reference server, which every draft citing N34
    follows. The IPv4 price is read at sign-up; it is under 1 EUR per VM either way (R6-HOST-OQ7, r6 lines 875–876).
  - *Decider:* Mustafa (a console check in his own account; agents create no account), then a research read if (b).
  - *What decides it:* the console offering both plans in `nbg1` or `hel1` before the netcode proof test is scheduled.
  - *Carries:* R6-HOST-OQ1, R6-HOST-OQ4, R6-HOST-OQ7.
- **OQ-0006-2. Which EU location: Nuremberg, Helsinki, or Falkenstein if offered?**
  - *Why it matters:* VISION_V2 §4 asks for matches that feel fair up to about 100 ms of ping from one EU region.
  - *Options:* `nbg1`, `hel1`; `fsn1` only if the plans are offered there (R6-HOST-F13, r6 lines 136–143).
  - *Decider:* a measurement (the netcode proof test's setup or a small check before it), confirmed by Mustafa.
  - *What decides it:* round-trip times from Mustafa's machine and from playtesters to each location, against §4's
    "about 100 ms" (R6-HOST-OQ2, r6 lines 856–858).
  - *Carries:* R6-HOST-OQ2.
- **OQ-0006-3. Where does the lobby run, and what stores guest ids and the room directory?**
  - *Why it matters:* join tokens, guest identity, room codes and crash reports go through it, and no budget holds it
    yet (R6-XT-OQ1, r6 lines 991–997).
  - *Options:* (a) the same binary as the supervisor, on the game VMs (no new VM line); (b) a separate service (one
    more VM, for example a CX33 at 8.99 EUR a month as a placeholder) (r2-netcode.md, R2-ROOM-OQ9, lines 3104–3105);
    the store: not researched (a SQL database was not read, r6 line 593). Its HTTPS endpoint's certificate: not
    researched.
  - *Decider:* the Advisor, from a short lobby read done with the security baseline (its key storage is item B5);
    Mustafa for any paid service. ADR 0001 keeps the protocol, ADR 0002 the crate.
  - *What decides it:* that read, before slice 3 (online).
  - *Carries:* R2-ROOM-OQ9, R2-XT-OQ4, R5-XT-OQ3, R6-XT-OQ1.
- **OQ-0006-4. Once the access wall is on, does it cover every preview, or only multiplayer previews and previews
  with unreleased art?**
  - *Why it matters:* Pages' access policy restricts "project deployments" as a whole (R5-PREV-F12, r5 lines 838–844),
    so PREVIEW-WALL (a) may hide offline previews too, and every Brave check then needs a login.
  - *Options:* (i) every preview of the project behind the wall from the first multiplayer preview (part 2's interim
    position); (ii) multiplayer previews in a second Pages project (not researched); (iii) Cloudflare Access per
    hostname (not researched for Pages; read for Workers previews only, R5-PREV-F14, r5 lines 850–858).
  - *Decider:* Mustafa (convenience against one more project to run).
  - *What decides it:* the preview hosting check shows what the policy covers (every preview, and not the production
    domain: R6-HOST-F4, r6 lines 80–86, inferred).
  - *Carries:* no research id (it follows from PREVIEW-WALL (a)).
- **OQ-0006-5. How are the per-hostname production certificates issued, and how long do they live?**
  - *Why it matters:* every production game VM needs one before the first online playtest, and the challenge decides
    which port or secret each VM needs.
  - *Options:* ACME http-01 (TCP port 80 open on each game VM); dns-01 per name (a DNS provider API key, a secret);
    tls-alpn-01 (RFC 8737: "This connection MUST use TCP port 443."); and, only if the security baseline accepts a
    shared key, the wildcard certificate (option C2) (R6-HOST-F24, r6 lines 198–204; R6-HOST-OQ6, r6 lines 869–874).
  - *Decider:* security-auditor in the security baseline, with the port plan of ADR 0001's transport.
  - *What decides it:* a read of the chosen certificate authority's policy (Let's Encrypt's pages were outside the
    allow list) and the baseline's key-handling rules.
  - *Carries:* R6-HOST-OQ6.
- **OQ-0006-6. The security baseline: what it decides for each waiting item.**
  - *Why it matters:* part 13's items hold interim positions only; BASELINE-WHEN (b) closes them before the first
    online playtest.
  - *Options:* per item, as part 13's table lists them (B1–B11); among them, whether an advisory blocks a pull request
    (R5-GATE-OQ4, r5-toolchain-quality.md lines 1873–1876; R1-RR-OQ7, r1-simulation-core.md lines 2955–2961), the rule that enforces
    `textContent` (R5-TSLINT-OQ5, r5-toolchain-quality.md lines 1776–1778), and where the guest secret lives (R2-ROOM-OQ11, r2-netcode.md lines
    3112–3116: Mustafa and the Advisor, SEC-GUEST recommending the cookie).
  - *Decider:* security-auditor, in its own fresh session (BASELINE-WHEN (b)); the Advisor and Mustafa for the policy
    calls.
  - *What decides it:* the baseline's research, before the first online playtest.
  - *Carries:* R6-SEC-OQ1, R5-GATE-OQ4, R1-RR-OQ7, R5-TSLINT-OQ5, R2-ROOM-OQ11.
- **OQ-0006-7. Will any CI runner be paid for: GPU, macOS, self-hosted reference machines, or a cache above 10 GB?**
  - *Why it matters:* WebGPU coverage in CI, real-Safari runs and reference-machine runs depend on it (critic rank
    33).
  - *Options:* (a) standard runners only, reference-machine runs by hand (part 8's interim position); (b) larger paid
    runners, price not read (R5-CI-OQ5, r5 lines 1835–1837; R5-TEST-OQ4, r5 line 1809); (c) self-hosted reference machines,
    "a cost and security question" (R3-MEASURE-OQ2, r3-rendering.md lines 1384–1385); (d) a cache above 10 GB, which
    needs a Pro, Team or Enterprise account (R5-CI-F5, r5 lines 583–592).
  - *Decider:* Mustafa for any paid plan; ADR 0002 says which jobs need one; the security baseline weighs (c).
  - *What decides it:* a price read, then the first month's CI times and misses against ADR 0002's budget.
  - *Carries:* R3-MEASURE-OQ2, R3-XT-OQ5, R5-XT-OQ11, R6-XT-OQ7.
- **OQ-0006-8. What do Sentry and Grafana really cost, and how do logs and metrics reach Grafana Cloud?**
  - *Why it matters:* the goal budget names "Sentry's paid plan" without a price, and M2 waits on the ingestion path.
  - *Options:* Sentry's Developer plan or a paid plan (base prices on sentry.io, outside the allow list; whether its
    retention can be set); Grafana Cloud Free under 10k series, or Pro at 19 USD plus usage (R6-COST-F1, F2, F3, lines
    219–235).
  - *Decider:* Mustafa, with a read of sentry.io/pricing before he creates the Sentry organization; a read of
    Grafana's ingestion docs if M2 is wanted.
  - *What decides it:* those reads and the series count the netcode proof test reports (R6-OBS-REC2's mark).
  - *Carries:* R6-COST-OQ2, R6-OBS-OQ2.
- **OQ-0006-9. What does EU data-protection law require of us, and how long are match records kept?**
  - *Why it matters:* logs, crash reports and match records are stored from slice 3 on; whether an IP address or a
    guest id is personal data, the lawful basis, the privacy notice and the retention limits were not researched.
  - *Options:* the proposed defaults (logs 14 days, metrics 15 days, records kept while their content hash is
    analysed), or what the law requires.
  - *Decider:* Mustafa.
  - *What decides it:* a read of the GDPR and the European Data Protection Board's guidance (both outside the allow
    list), or advice, before any public playtest (R6-OBS-OQ1, r6 lines 896–899; R6-TEL-OQ4, r6 lines 917–919).
  - *Carries:* R6-OBS-OQ1, R6-TEL-OQ4.
- **OQ-0006-10. How many players, and when; which VAT and currency; is included traffic per VM, pooled or prorated?**
  - *Why it matters:* the start and goal budgets rest on placeholder player counts (A-4, A-5) and exclude VAT.
  - *Options:* the placeholders until playtest data replaces them.
  - *Decider:* Mustafa (his plans, his billing address, the first invoice).
  - *What decides it:* playtest data in the match records (R6-COST-REC2) and the provider's invoice and terms
    (R6-COST-OQ3, OQ4, OQ5, r6 lines 887–892).
  - *Carries:* R6-COST-OQ3, R6-COST-OQ4, R6-COST-OQ5.
- **OQ-0006-11. How many matches before a rate means something, and how do shipped games do it?**
  - *Why it matters:* single samples mislead (row TL13), and part 12's derived rates need a minimum count.
  - *Options:* confidence intervals per cell and a minimum match count; a shipped game's published method, if one is
    found.
  - *Decider:* test-engineer and the game-designer, before the first tuning pass.
  - *What decides it:* a statistics source, and a transcript or paper on an allowed domain (R6-TEL-OQ1, OQ5, lines
    910–912 and 920–922).
  - *Carries:* R6-TEL-OQ1, R6-TEL-OQ5.
- **OQ-0006-12. If production isolation is ever adopted: embeds, cross-origin popups, CORP and the login flow.**
  - *Why it matters:* each would need CORP under isolation, or a separate page (r1-simulation-core.md, R1-BND-OQ5,
    lines 2734–2736); this record proposes no embed.
  - *Options:* none needed while production ships without isolation (the isolation flag is off by default); otherwise
    CORP per resource or a separate page.
  - *Decider:* Mustafa with this record, only if production isolation is adopted (R1-BND-REC2 mark 5's test).
  - *What decides it:* that adoption (critic rank 38, r1-simulation-core.md lines 3105–3108).
  - *Carries:* R1-BND-OQ5, R1-XT-OQ17, R6-XT-OQ9.

## What a spike must prove

Two spikes carry this record's claims before it is accepted; the spike plan gives their final names. Every mark below
is a candidate from the research ("proposed" there); test-engineer turns them into the pass marks. Where the research
gives 48 and 54, 48 holds (MODES-3 (a)); every server mark runs at the 30 Hz baseline (TICK-30), and the 60 Hz margin
run is ADR 0001's.

**1. The preview hosting check** (small; on a test project in Mustafa's Cloudflare account, with a static page and a
`.wasm`; Mustafa creates the project and adds its token; before the foundation's preview step). It proves parts 1, 2
and 3:
- every HTML, JS, worker and Wasm response carries the configured headers, on 100 % of responses in a full page load,
  checked by a script (r6-infrastructure.md, R6-HOST-REC1's mark, lines 639–644);
- `crossOriginIsolated` is true in Chrome, Edge, Firefox and Safari with the isolation flag on and false with it off
  (same mark), and in Brave too (r5-toolchain-quality.md, R5-PREV-REC4 mark 2, lines 1636–1640);
- a hashed bundle file is served with an immutable cache header and a repeat visit fetches 0 bytes of it; the largest
  file stays under 25 MiB, rechecked against Workers' own limits page if the alternative is used (R6-HOST-REC1's mark);
- PREV-DELETE: 24 h after a test pull request closes, its preview link no longer serves the build (HTTP 404, or the
  access wall), or the remaining case is written down with the reason (R5-PREV-REC3's mark, lines 1622–1624;
  r6-infrastructure.md, PREV-DELETE, lines 970–976); a failure with no workaround sends the host to Workers static
  assets;
- whether a direct upload counts against Pages' 500 builds a month (R6-HOST-REC1, r6 lines 633–634);
- with the access policy on: it covers every preview of the project and not the production domain
  (OQ-0006-4; R6-HOST-F4, inferred), and Mustafa logs in once per Brave session and the link works after that
  (R5-PREV-REC5's mark, r5 lines 1648–1650). **Mustafa judges** this step in Brave on Windows; test-engineer adds the
  Brave checklist.
- ADR 0002's preview questions ride along (R5-PREV-OQ5: the environment URL on the pull request page; R5-PREV-OQ6:
  the branch-alias header rule).

**2. This record's operations marks inside the netcode proof test** (ADR 0001's spike, on the reference server; its
precondition is OQ-0006-1). They prove parts 5, 7, 8 and 9:
- rooms per VM: on the CCX23 and on the CX33, the number of rooms of 48, 36 and 18 players that run as separate
  processes with each room's server tick p99 within 10 % of the single-room figure, over 20 minutes, repeated at 3
  times of day on the shared host (r6-infrastructure.md, R6-HOST-REC2's mark, lines 667–671; R6-COST-REC1's mark, lines
  714–717; R2-ROOM-REC2 mark 2, r2-netcode.md lines 2775–2782, ADR 0001's test). This replaces A-2 and fixes part 8's
  goal budget (R6-COST-OQ1);
- WebTransport over UDP and WebSocket over TCP reach the VM through the firewall from all four browsers; a VM with no
  rule for a port drops it (negative test) (R6-HOST-REC2's mark); and the metrics path part 9 picks (scrape or push,
  R6-OBS-F4) works with only the rule it needs (proposed: the negative test extended to that rule);
- egress per 24v24 match-hour within ±20 % of 3.16 GB with interest management off, and the figure with it on; the
  spike's own hosting bill under 5 EUR (R6-COST-REC1's mark);
- logging at its production level keeps server tick p99 within 2 % of a run with logging off, with 0 allocations per
  tick from logging on the hot path (counted), and a grep of a 30-minute soak's logs finds 0 tokens, secrets or raw IPs
  (R6-OBS-REC1's mark, r6 lines 736–738);
- every metric of part 9 is present after a 30-minute soak at N = 48; a deliberately stalled tick shows as one overrun
  and its dropped time within one scrape; the series count is reported and is under 5,000 for 8 rooms on one VM
  (R6-OBS-REC2's mark, r6 lines 750–753);
- the development certificates: all four browsers connect with a pinned self-signed certificate in 100 % of 20
  attempts each; a rotation with 48 live sessions drops 0 of them and new connections succeed with the new hash within
  one reload; a certificate valid for 15 days is rejected by all four browsers (r2-netcode.md, R2-TRN-REC5 marks 1–3,
  lines 2717–2722; the half of mark 1 that needs a production certificate per hostname waits for the Test plan).

**Optional, in the boundary spike (ADR 0002's), only if Mustafa has created the EU Sentry organization by then:** a
deliberate Wasm trap in the simulation worker arrives in Sentry with Rust function names in Chrome, Edge, Firefox and
Safari (4 of 4); the shipped `.wasm` holds no DWARF; a forced tab crash in Chrome produces a `crash` report at our
endpoint, and browsers that send none are listed; 0 IP addresses in a sample of 100 events (r6-infrastructure.md,
R6-OBS-REC3's mark, lines 765–768; it answers R6-OBS-OQ3 and OQ4). Otherwise these marks move to the first online
playtest's milestone.

### Pass marks
Written by test-engineer (plan step 4).

## Migration plan

Phases in the slice order (SLICE-ORDER), the netcode proof test before the foundation, the foundation before slice 1.
Each milestone is checked by its marks (What a spike must prove; Test plan).

0. **Before any spike (Mustafa's steps):** the drafts approved; a Cloudflare account with a test project and its
   token in a GitHub environment; a Hetzner account and the console check of OQ-0006-1. *Milestone:* the reference
   server is orderable, or OQ-0006-1 (b) has named another.
1. **Spikes (before the foundation):** the netcode proof test with this record's operations marks, and the preview
   hosting check, in parallel. *Milestone:* their marks pass; rooms per VM measured, so part 8's goal budget is one
   figure; PREV-DELETE settled, or the client host moved to Workers static assets.
2. **The foundation:** preview links per pull request on the client host (ADR 0002's deploy job, this record's host
   and token scope), public but unlisted, with the cleanup policy of part 3 live. *Milestone:* "an empty scene runs
   through the full stack in the browser, with CI green and a preview link" ([the foundation](../research/GLOSSARY.md#the-foundation)).
3. **Slices 1 and 2 (move & shoot; bots and three teams, played locally):** no game server is needed; previews run the
   match offline in the Worker; CI stays on free standard runners. *Milestone:* every preview link opens in Brave
   without a login, and every closed pull request's preview is gone by part 3's rule.
4. **Slice 3 (online):** before the first online playtest: the security baseline researched and its decisions applied
   (production certificates, OQ-0006-5; the CSP in the production header set; the advisories policy; the guest secret;
   key storage), the lobby placed (OQ-0006-3), Sentry set up in the EU (OQ-0006-8), the data rules read (OQ-0006-9).
   The staging game server and the first multiplayer preview, behind the access wall (parts 2 and 4); the production
   VM at the start budget (part 8); logs and metrics live (part 9). *Milestone:* the staging, certificate, crash-report
   and data-map marks of the Test plan pass.
5. **Slice 4 (the loop):** the match record, its storage and its analysis (part 12), with the cost line (part 8).
   *Milestone:* the telemetry marks of the Test plan pass.
6. **Slice 5 (Octopon-3 and Editor v2):** nothing new here; records group by map id and content hash as before.
7. **Slice 6 (scale to 48):** the fleet sized by the measured density; K2 or K3 only on their triggers (part 8);
   the goal budget re-read against the monthly bill check. *Milestone:* the bill check within the budget, and rooms per
   VM at N = 48 re-measured on the real sim.

## Test plan

- **Network and transport:** both transports reach the VMs through the firewall from all four browsers, and a port
  with no rule is dropped (R6-HOST-REC2's mark); network simulation with latency, jitter and loss is ADR 0001's
  profiles (P0–P4, B1), run against the staging game server like any other.
- **Soak and load:** 30-minute soaks at N = 48 for logs and metrics, 20-minute runs at 3 times of day for rooms per VM
  on the shared host (What a spike must prove, spike 2).
- **Cost:** 100 % of spike match records carry room-hours and bytes sent, and their sum over a test day matches the
  server's own interface counters within 5 % (r6-infrastructure.md, R6-COST-REC2's mark, lines 724–725); the monthly
  bill check against the budget from slice 3 on.
- **Previews:** the deploy job's header check passes and fails when a header is removed (R5-PREV-REC4 mark 1, ADR
  0002's); 24 h after a pull request closes its link no longer serves the build, and the weekly sweep leaves 0 previews
  of closed pull requests older than 7 days (R5-PREV-REC3's mark).
- **The staging game server** (at the first multiplayer preview): two previews of different builds play at the same
  time on one staging VM, each against its own binary; a closed pull request's binary is gone within 24 h; a preview
  client never connects to a production server (R6-HOST-REC3's mark, r6 lines 681–683).
- **Production certificates** (before the first online playtest): R2-TRN-REC5's marks with the per-hostname
  certificates (all four browsers, 20 of 20 connections); a forced renewal swaps the certificate on a VM with 48 live
  sessions and drops 0 of them; an alert fires when any VM's certificate has under 14 days left (R6-HOST-REC4's mark,
  r6 lines 697–699).
- **Crash reports** (before the first online playtest, if not done in the boundary spike): R6-OBS-REC3's marks (What
  a spike must prove).
- **Data protection:** a data map lists every store, its region and its retention, with 0 stores outside the EU for
  event data; data older than its cap is gone on a check one day after the cap (R6-OBS-REC4's mark, r6 lines 779–780).
- **Telemetry** (slice 4): in 30 bot matches across 3 modes (2 teams, 3 teams, and 24v24), 100 % write a record before
  Closed; every kill event names a source id that resolves in the content set of the record's hash; the points total
  recomputed from events equals the final points in 100 % of records; recording adds under 1 % to server tick p99 and
  0 allocations per tick after warm-up; a 24v24 record is at most 1 MB uncompressed (R6-TEL-REC1's mark, lines
  807–811, at N = 48); with object storage unreachable for 1 hour, 0 records are lost and all upload within 10 minutes
  of recovery; a record of schema version n is read by the script for version n + 1; a room killed with SIGKILL
  mid-match leaves a stub record marked incomplete (R6-TEL-REC2's mark, r6 lines 824–827); the analysis script reproduces
  the hand-counted cashouts, steals, deliveries and harvester kills of 3 matches exactly, every output row carries its
  match count, and a month of goal-scale records (about 13,000) is analysed in under 10 minutes on the staging VM
  (R6-TEL-REC3's mark, r6 lines 840–842).
- **People:** Mustafa's Brave checks of preview links (one login per session once the wall is on); test-engineer
  writes the checklist.

## Rollback

- **The client host:** if PREV-DELETE or a header mark fails on Pages, move to Workers static assets, which takes the
  same `_headers` file (R6-HOST-REC1); only ADR 0002's deploy target changes. If Cloudflare as a whole had to go,
  Netlify is R6-HOST-REC1's named fallback with the same `_headers` file (its prices were not read, r6 lines 624–626).
- **Game servers:** if the CCX23 cannot be ordered or fails the marks, read OQ-0006-1's fallbacks and re-run the
  proof test there; every draft that cites the reference server (N34, I-15) follows the new name. If the shared CX33
  fails, the fleet stays on dedicated vCPUs (K1). VMs are billed by the hour with a monthly cap (R6-HOST-F17), so a
  VM removed stops costing from the next hour (inferred).
- **Certificates:** the production form can move between a certificate per hostname and the wildcard by the security
  baseline's decision; the development certificates are not affected.
- **Observability:** the same exporter feeds Prometheus on the staging VM or Grafana Cloud (M1 or M2), so moving
  between them changes no room code. Removing Sentry is a client and server change; its EU storage choice cannot be
  undone (R6-OBS-F9), so the organization is created once, in the EU.
- **Telemetry:** records are versioned files; a schema change adds a version and old records still parse
  (R6-TEL-REC2's mark); deleting the bucket removes them.
- **The whole record:** if it is rejected, no spike has a reference server or a preview host, so the netcode proof
  test waits until another machine is named; nothing else in the repository depends on it yet.

## Appendix A: research dispositions

One row per id the decision map gives this record as owner (65 ids; `engine-architect-decision-map-ids.tsv`, owner
0006), in the map's order.

| id | disposition | where or why |
|---|---|---|
| R1-DET-OQ7 | answered: x86-64 to start; Arm only after R1-DET-REC6's matrix includes it | part 5 (R6-HOST-REC2) |
| R1-BND-OQ5 | open: OQ-0006-12 | only if production isolation is adopted |
| R1-RR-OQ7 | open: OQ-0006-6 | the mechanism is answered (part 13, B1: the advisories check on a schedule and on `Cargo.lock` changes); whether an advisory blocks a pull request is the security baseline's |
| BASELINE-WHEN | answered: (b) | part 13: listed open with interim positions; researched before the first online playtest |
| R1-XT-OQ17 | open: OQ-0006-12 | critic rank 38, with R1-BND-OQ5 |
| R2-TRN-REC5 | adopted | parts 4 and 7: development and preview certificates self-signed, pinned, at most 14 days; production a certificate per hostname; its wildcard sentence (r2-netcode.md lines 2704–2706) superseded as part 7 states (carry-forward 4) |
| R2-TRN-OQ4 | answered: no production pinning (the VMs stay warm, not ephemeral); a certificate per hostname under BASELINE-WHEN (b) | part 7; how it is issued is OQ-0006-5 |
| R2-ROOM-OQ9 | open: OQ-0006-3 | the lobby's place and store (critic rank 8) |
| R2-ROOM-OQ11 | open: OQ-0006-6 | interim: R2-ROOM-REC8's HttpOnly cookie (part 13, B4); the CSP is the security baseline's |
| R2-XU-OQ9 | answered: the reference server is a CCX23 | part 5 (R6-HOST-REC2) |
| R2-XT-OQ4 | open: OQ-0006-3 | critic rank 8 |
| R2-XT-OQ9 | answered: part 13 lists every recommendation that waits on the security baseline, with its interim position | critic rank 18; the baseline itself is OQ-0006-6 |
| R3-MEASURE-OQ2 | open: OQ-0006-7 | interim: reference-machine runs by hand, no self-hosted runner (parts 8 and 13, B11) |
| R3-XT-OQ5 | open: OQ-0006-7 | critic rank 33 |
| R5-PREV-REC2 | adopted | part 1: Cloudflare Pages, with Workers static assets the alternative |
| R5-PREV-REC3 | adopted | part 3: the cleanup policy (24 h, the weekly sweep, artifacts 7 days); the close and sweep jobs are ADR 0002's; PREV-DELETE in the preview hosting check |
| R5-PREV-REC5 | changed: PREVIEW-WALL (a) applied | part 2: offline previews public but unlisted unless they carry unreleased art; every multiplayer preview behind the access wall from the first one, so the mark "opens in Brave without a login" holds only until then (OQ-0006-4) |
| R5-TSLINT-OQ5 | open: OQ-0006-6 | interim: CLAUDE.md's `textContent` rule by review (part 13, B6) |
| R5-PREV-OQ1 | answered: PREVIEW-WALL (a) | part 2; what the wall covers is OQ-0006-4 |
| R5-PREV-OQ2 | spike: the preview hosting check (PREV-DELETE) | part 3; What a spike must prove, spike 1 |
| R5-PREV-OQ3 | answered: offline previews need no game server; multiplayer previews use the staging game server, behind the access wall | parts 2 and 4; delivery of the binary and the certificate hash is ADR 0002's (critic rank 20) |
| R5-PREV-OQ4 | answered by R6-HOST-REC1: Cloudflare Pages, 0 USD on its Free plan (price inferred) | parts 1 and 8 |
| R5-GATE-OQ4 | open: OQ-0006-6 | interim: scheduled and on `Cargo.lock` changes (part 13, B1) |
| PREV-DELETE | spike: the preview hosting check | part 3; a failure sends the client host to Workers static assets |
| PREVIEW-WALL | answered: (a) | part 2 |
| R5-XT-OQ3 | open: OQ-0006-3 | critic rank 8 |
| R5-XT-OQ5 | answered: part 13 lists the waiting items with interim positions | critic rank 18; the baseline itself is OQ-0006-6 |
| R5-XT-OQ11 | open: OQ-0006-7 | critic rank 33 |
| R6-HOST-REC1 | adopted | part 1 |
| R6-HOST-REC2 | adopted | part 5: a working pick until OQ-0006-1 is answered; its marks restated for N = 48 at 30 Hz (spike 2) |
| R6-HOST-REC3 | adopted, with PREVIEW-WALL (a) applied | part 4: the wait at r6-infrastructure.md line 675 ends; multiplayer previews use it from slice 3, behind the access wall |
| R6-HOST-REC4 | adopted | part 7: the interim position under BASELINE-WHEN (b); the four superseded sentences (carry-forward 4) restated |
| R6-COST-REC1 | changed: the goal budget stated at both densities | part 8: about 361 or 238 EUR at 8 rooms per VM, about 534 EUR (fixed fleet) at the 4 rooms per VM that A-2 names the case to budget, until the netcode proof test measures it; the start budget unchanged |
| R6-COST-REC2 | adopted | part 8 |
| R6-OBS-REC1 | adopted | part 9; until a log store is read, logs stay on the VM under part 11's cap (proposed) |
| R6-OBS-REC2 | adopted | part 9; the metrics path (scrape or push) checked in spike 2 |
| R6-OBS-REC3 | adopted | part 10: Mustafa creates the EU organization after reading Sentry's prices; before the first online playtest |
| R6-OBS-REC4 | adopted | part 11; the legal side is OQ-0006-9 |
| R6-TEL-REC1 | changed: the per-slot section adds the upgrade-track fields | part 12: picks per track and level, offers shown, looted weapons, time per weapon type (decision map part j, J5; need: UPGRADE-SCOPE; no research finding); marks at N = 48 only (MODES-3 (a)) |
| R6-TEL-REC2 | adopted | part 12 |
| R6-TEL-REC3 | adopted | part 12 |
| R6-HOST-OQ1 | open: OQ-0006-1 | Mustafa's console check |
| R6-HOST-OQ2 | open: OQ-0006-2 | a round-trip measurement |
| R6-HOST-OQ3 | answered: no UDP load balancer while the lobby hands out host and port | part 5; revisit when a design routes many rooms through one address (Consequences) |
| R6-HOST-OQ4 | open: OQ-0006-1 | the fallbacks are read only if Hetzner is ruled out |
| R6-HOST-OQ5 | answered: PREVIEW-WALL (a) | part 2 |
| R6-HOST-OQ6 | open: OQ-0006-5 | the challenge and lifetimes of the per-hostname certificates; the wildcard only if the baseline weighs it in |
| R6-HOST-OQ7 | open: OQ-0006-1 | read at sign-up; under 1 EUR per VM either way |
| R6-COST-OQ1 | spike: the netcode proof test (this record's operations marks) | parts 8 and 14; spike 2 |
| R6-COST-OQ2 | open: OQ-0006-8 | Sentry's prices and retention, Grafana's usage |
| R6-COST-OQ3 | open: OQ-0006-10 | player numbers |
| R6-COST-OQ4 | open: OQ-0006-10 | included traffic per VM, pooled or prorated (matters only for K2) |
| R6-COST-OQ5 | open: OQ-0006-10 | VAT and currency |
| R6-OBS-OQ1 | open: OQ-0006-9 | Mustafa's data-protection read |
| R6-OBS-OQ2 | open: OQ-0006-8 | Grafana's ingestion path |
| R6-OBS-OQ3 | spike: R6-OBS-REC3's marks (in the boundary spike if the EU Sentry organization exists, otherwise at the first online playtest's milestone) | part 10 |
| R6-OBS-OQ4 | spike: R6-OBS-REC3's marks, as R6-OBS-OQ3 | part 10 |
| R6-TEL-OQ1 | open: OQ-0006-11 | test-engineer and the game-designer |
| R6-TEL-OQ4 | open: OQ-0006-9 | proposed default: kept while the content hash is analysed (part 11) |
| R6-TEL-OQ5 | open: OQ-0006-11 | a shipped game's method on an allowed domain |
| R6-SEC-OQ1 | open: OQ-0006-6 | part 13 lists its items with interim positions (BASELINE-WHEN (b)) |
| R6-XT-OQ1 | open: OQ-0006-3 | critic rank 8 |
| R6-XT-OQ3 | answered: part 13 lists the waiting items with interim positions | critic rank 18; the baseline itself is OQ-0006-6 |
| R6-XT-OQ7 | open: OQ-0006-7 | critic rank 33 |
| R6-XT-OQ9 | open: OQ-0006-12 | critic rank 38 |

**Cited, owned elsewhere** (no disposition here): R6-OBS-OQ5 and R6-XT-OQ5 (ADR 0003: what client field data is
measured; stored under part 11), R6-XT-OQ4 (ADR 0002: a multiplayer preview's binary and certificate hash), R6-XT-OQ6
(ADR 0004: recovery from a Wasm trap), R6-XT-OQ8 (ADR 0002: the Sentry upload job and the smoke test's network rule),
R6-TEL-OQ2, R6-TEL-OQ3 and R6-XT-OQ2 (design questions for Mustafa), R5-PREV-REC1 and R5-PREV-REC4 (ADR 0002: the
deploy job and the generated header sets, with this record's host, token scope and header content), R2-ROOM-REC2
(ADR 0001: the room process; this record's VMs and supervisor), MODES-3 (ADR 0001).

## Sources

All research lines at commit 9e7db28.
- `docs/research/r6-infrastructure.md`: read in full (Header, TL;DR, Findings §1–§5, Options §1–§5, Recommendations
  §1–§5, Open questions §1–§5, the shared and cross-topic open questions, the hinge table, the pointers, the verifier
  spot-check and its second pass, Sources).
- `docs/research/r5-toolchain-quality.md`: Findings, Question 5 (R5-CI-F5, F17) and Question 6 (R5-PREV-F1–F20, legacy
  facts, lines 756–915); Options, Questions 5 and 6 (lines 1078–1128); Recommendations, Question 5 (R5-CI-REC1–REC5)
  and Question 6 (R5-PREV-REC1–REC5) (lines 1507–1650); R5-GATE-REC1 and R5-GATE-F3; Open questions, Questions 5–7
  and R5-XQ (lines 1822–1911), R5-TSLINT-OQ5, R5-TEST-OQ4; the shared and cross-topic open questions it shares with R6
  (lines 1918–1930, 1939–1956, 1978–1983, 1991–1995, 2021–2025); Pointers from R5-CI and R5-PREV (lines 2091–2106).
- `docs/research/r2-netcode.md`: R2-TRN-REC3 (lines 2639–2650, its first part), R2-TRN-REC5 (lines 2699–2723), R2-ROOM-REC1–REC3
  (lines 2748–2811), R2-ROOM-REC8 (lines 2868–2885), R2-TRN-OQ4 (lines 3050–3054), R2-ROOM-OQ9 and OQ11 (lines
  3104–3116), R2-XU-OQ9 (lines 3197–3202), R2-XT-OQ4 and OQ9 (lines 3347–3352, 3379–3383).
- `docs/research/r1-simulation-core.md`: R1-DET-OQ7 (lines 2701–2703), R1-BND-OQ5 (lines 2734–2736), R1-RR-OQ7
  (lines 2955–2961), R1-XT-OQ17 (lines 3105–3108).
- `docs/research/r3-rendering.md`: R3-MEASURE-OQ2 (lines 1384–1385), R3-XT-OQ5 (lines 1447–1451).
- `docs/research/GLOSSARY.md`: How to read an entry; §1 (server tick time, transport runtime time, time statistics,
  the server tick budget, the reference server, the measuring unit: one vCPU); §2 (player count, team size, the start
  modes and the goal modes, the largest mode, the design point and headroom); §5 (all); §6 (all); and the §8 text in
  the decision map's addendum (the foundation, the netcode proof test, the 60 Hz margin).
- `docs/research/README.md`; `docs/adr/0000-template.md`.
- `docs/VISION_V2.md` (all); `docs/LOOP_V2.md` §8 and §11; `docs/LOOK_V2.md` (all); `docs/STATUS.md`, "Decisions
  pending" and the ADR-001 loose ends it lists.
- `docs/LEGACY_INVENTORY.md`: the marks legend and rows CD2, CD3, PG19, KT8, TL9, TL13, TL15, GE15, GC18 (and GP5,
  PG13, GP11, GE12 for their "look brief pending" tags).
- Inputs of this change-set (not research): the drafts brief, the plan (revision 3), the advisor's verdict §1, the
  decision map with its addendum and id table, and Mustafa's UPGRADE-SCOPE update.
