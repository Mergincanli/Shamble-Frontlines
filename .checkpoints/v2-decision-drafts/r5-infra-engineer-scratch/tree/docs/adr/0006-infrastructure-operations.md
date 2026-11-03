# ADR-0006: Infrastructure and operations

**Status:** Proposed
**Date:** 2026-10-31
**Author:** infra-engineer (pass marks by test-engineer, plan step 4) ·
**Reviewed by:** (filled in at review) ·
**Signed off by Mustafa:** as the basis for the spikes (2026-11-03, SPIKE-ORDER (b)); not yet accepted

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
- **VISION_V2 §3 and §4** (counted like LOOP_V2, by the Advisor's answer to OQ-0001-14 (2026-11-03): "yes, a
  VISION_V2 §3 or §4 trace counts, marked as such"; decision map part j, J3): the start
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
5. Sentry's EU storage is chosen when the organization is created: "once selected, your data storage location can’t be
   changed", and its browser SDK sends
   the user's IP address by default (r6-infrastructure.md, Findings §3, R6-OBS-F9, F10, lines 318–331).

### What it takes and what it gives
- **Takes:** the room process, its lifecycle, join tokens, the guest identity and the lobby protocol (ADR 0001); the
  CI workflows, the preview build and deploy job, the generated
  [header sets](../research/GLOSSARY.md#the-production-header-set-and-the-preview-header-set), the Sentry upload job and how a
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
  (belongs with hosting, ADR-001)"). STATUS's old ADR-001 entry lists hosting among its subjects; the v2 drafts
  replace ADR-001..003, and hosting is decided here (decision map part j, J2: items ADR-001 and CDN-SRI).

## Decision

Serve the player build and every preview link from Cloudflare Pages, and run game servers on Hetzner Cloud in one EU
location, one room per process under a small supervisor per VM, on a dedicated-vCPU CCX23 named as the reference
server, with the shared CX33 measured beside it. Keep the budget flat and small at the start (about 18 to 102 EUR a
month) and state the goal budget at both room densities until the netcode proof test measures how many rooms fit one
VM, a figure that stays provisional until slice 6 measures the real sim (OQ-0006-19). Log, measure and report crashes
with small, EU-only tools that keep no personal data. Write one balance record per match inside the room, store it as
JSON lines in EU object storage, and analyse it in batches per content hash. List the security baseline as open, with
interim positions, until its own research before the first online playtest; the release model is open too
(OQ-0006-14). Every account, payment, secret and repository setting below is Mustafa's to create.

1. **The client host.** [The client host](../research/GLOSSARY.md#the-client-host) is Cloudflare Pages, for the player
   build and every per-PR [preview](../research/GLOSSARY.md#preview), with Workers static assets (the alternative), which
   takes the same `_headers` file. The build's generated `_headers` (ADR 0002) carries
   [the production header set](../research/GLOSSARY.md#the-production-header-set-and-the-preview-header-set) on
   production and the preview header set on previews; COOP and COEP appear only on a preview built with
   [the isolation flag](../research/GLOSSARY.md#preview-isolation-and-the-isolation-flag) on, and the preview Mustafa
   checks is built with it off (R5-XQ-OQ3, settled 2026-10-30: r5-toolchain-quality.md, Open questions, R5-XQ-OQ3,
   lines 1899–1910). The content bundle is served with an immutable cache header (R1-DATA-REC6, as R6-HOST-REC1's mark names it, r6 lines
   641–642; interface I-6). Every
   served file, authored models included (LOOK_V2 §8), stays under 25 MiB, Pages' per-asset limit (r6-infrastructure.md,
   Findings §1, R6-HOST-F3, lines 72–79). three.js is bundled from npm by Vite and served from the client host, so v2
   has no CDN script tag (the STATUS item "no SRI/CSP on the three.js CDN tag (belongs with hosting, ADR-001)"; inventory
   row PG19); the content security policy waits for the security baseline (part 13). Pages' Free plan allows 500 builds a
   month (R6-HOST-F3, r6 lines 72–79), and whether a direct upload counts against them is a spike item (part 14). If
   uploads count, only the newest push of each pull request deploys (ADR 0002's concurrency cancel) and the month's
   deploys are counted; a paid plan is Mustafa's after a price read (proposed, not researched).
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
   R5-PREV-REC5, lines 1642–1650). That is how this record reads the foundation's "private preview links": under every
   PREVIEW-WALL option, "previews that reach no game server and carry no unreleased art are public but unlisted"
   (GLOSSARY.md lines 623–624, the access wall). Every multiplayer preview, which talks to [the staging game server](../research/GLOSSARY.md#the-staging-game-server), is behind the
   access wall from the first one (PREVIEW-WALL (a)). The access wall is the client host's access policy, limited to
   Mustafa's Cloudflare account, or Cloudflare Access; Mustafa logs in once per Brave session (R5-PREV-REC5;
   R5-PREV-F12, F14, r5 lines 838–858). Pages' policy restricts "project deployments" as a whole (R5-PREV-F12, r5 lines
   838–844), so from the first multiplayer preview every preview of the project sits behind the wall (proposed interim
   position; whether to split them is OQ-0006-4). The policy covers previews, not the production domain
   (r6-infrastructure.md, Findings §1, R6-HOST-F4, lines 80–86, inferred there; Cloudflare's preview-deployments page
   states it, read by the verifier on 2026-11-01), so the game itself stays public. The wall hides a preview, not the
   files behind it: the repository is public (r5-toolchain-quality.md, Findings, Question 6, legacy facts, lines
   913–914), so a model file committed to it can be read by anyone once it is pushed, as R5-PREV-REC5 itself reasons:
   "A preview contains only what the public repository already contains (inferred)" (r5 line 1645). Where model files
   live is ADR 0005's OQ-0005-AP1; whether unreleased art must stay unseen is OQ-0006-18.
   *Why:* PREVIEW-WALL (a); a preview holds no secrets, so offline previews need only `noindex`
   (r6-infrastructure.md, R6-HOST-REC1, line 622). *Ids:* PREVIEW-WALL, R5-PREV-REC5, R5-PREV-OQ1, R6-HOST-OQ5.
   *Need:* GLOSSARY §8, the foundation ("private preview links") only; LOOK_V2 §8 (unreleased authored models).
3. **Preview cleanup.** 24 h after a pull request closes, its preview link no longer serves the build (HTTP 404, or the
   access wall), or the remaining case is written down with the reason; a weekly sweep leaves 0 previews of closed pull
   requests older than 7 days; build artifacts are kept 7 days, not the default 90 (r5-toolchain-quality.md,
   Recommendations, Question 6, R5-PREV-REC3, lines 1616–1624). CI failure artefacts (each failure's seed, recording
   and trace, ADR 0002 part 8) are kept 30 days, and each failure's issue carries its seed, so the run can be repeated
   after the artefacts expire (proposed: a nightly failure triaged a week later must still have its trace, and 30 days
   stays far under the default 90 that R5-PREV-REC3 cuts so a public repository keeps little lying around; R5 hands
   this retention here, r5-toolchain-quality.md, Pointers, line 2083). The policy is this record's; the close and sweep
   jobs
   are ADR 0002's workflows (seam S3). On Pages "The latest deployment for a branch cannot be deleted" (R5-PREV-F13,
   r5 lines 845–849), so whether a closed pull request's last link really disappears stays a spike item, PREV-DELETE, in
   the preview hosting check (part 14). The first workaround the check tries is a tombstone: a page with no build is
   deployed as the branch's last deployment, then every other deployment is deleted (proposed, not researched); if
   Pages fails PREV-DELETE with no workaround, Workers static assets is weighed again
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
   hash reach it is ADR 0002's workflow and ADR 0001's hash endpoint (critic rank 20).
   - *Who may join (interim position, BASELINE-WHEN (b); the security review, finding 2):* the wall guards the preview
     page, not this VM, whose UDP and TCP ports are public, and an Origin check stops browsers only, since a client
     outside a browser sends any Origin it likes. So staging rooms admit only join tokens signed with a staging key, and
     only an endpoint behind the access wall can obtain one; staging and production sign with separate keys (ADR 0001's
     key id), so a production room refuses every staging token whatever its Origin (proposed). Which endpoint mints
     staging tokens (Worker code on the client host behind the wall, which may bring Workers Paid's 5 USD a month, part
     1; or a staging lobby behind Cloudflare Access) is not researched and is read with OQ-0006-3.
   - *Its certificate hash (proposed, not researched):* a hash written into a static preview goes stale at the staging
     server's next certificate (at most 14 days later, or at any restart if each room process makes its own: R2 "notes
     per-process certificates if `serverCertificateHashes` is used", r2-netcode.md, Pointers between the R2 units, lines
     3441–3442), and R2-TRN-REC5 mark 2's "new connections succeed with the new hash within one reload" (r2 lines
     2720–2721) cannot hold when the reload fetches the same file. So on this VM the supervisor, not each room process,
     holds the certificate; it is rotated every 7 days (proposed: half the 14-day limit) and the next one is made a full
     period ahead, so a preview's file lists the current and the next hash (the option takes several, by its name; not
     quoted in R2) and keeps working for at least 7 days; an older preview is redeployed. Whether the staging game server
     instead presents, on both transports, the trusted certificate its `wss` endpoint needs anyway, so that previews
     carry no hash, is OQ-0006-5.
   - *What else runs on it:* Prometheus (part 9), which holds no secret. The batch analysis and its key to the record
     store do not: this VM runs pull-request-built native code, so it holds no secret beyond its rooms' certificate key
     (part 13, B10 and B12), and its rooms' records never enter the store (part 12).
   *Why:* a VM per pull request bills at least an hour each
   time and adds machines to run; previews only need a server that speaks the pull request's protocol version
   (r6 lines 676–677; R6-HOST-F17, r6 lines 164–168). *Ids:* R6-HOST-REC3, R5-PREV-OQ3. *Need:* VISION_V2 §6.1 (rooms, tested
   per pull request, r6 line 680).
5. **Game servers and the reference server.** Hetzner Cloud in one EU location; one room per process (R2-ROOM-REC2,
   ADR 0001's) under a small supervisor per VM; no load balancer on the game path, since the lobby hands out host and
   port; each VM has its own public IPv4 and IPv6 address. Hetzner's firewall is stateful, drops all inbound traffic no
   rule allows and costs nothing (r6-infrastructure.md, Findings §1, R6-HOST-F18, lines 169–174). Its rules are listed
   per VM role (proposed: R6-HOST-REC2 opens the two game ranges "plus nothing else", r6 lines 648–649, but this
   record's own parts need more), and each role opens only its list:
   - *every game VM, production and staging:* one UDP range for WebTransport and one TCP range for the WebSocket
     fallback (R2-TRN-REC3, as R6-HOST-REC2 says), their ports set by the port plan (OQ-0006-16); the one rule part 9's
     metrics path may need, with the Prometheus host as its only source (a scrape needs an inbound port while a push
     needs none, R6-OBS-F4, r6 lines 279–284; spike 2 checks it); and SSH, by key only, from named source addresses only
     (proposed: without an admin path no one can install a room binary or read the logs part 9 keeps on the VM; on the
     spike VMs sessions get SSH access, OQ-0006-17, answered by Mustafa (2026-11-03): "sessions get SSH access only and
     no Hetzner token until the security baseline", and how a session's key is made and held without it reading a
     secret it did not make is for the netcode proof test's own plan, with security-auditor and the advisor: the
     advisor's reading, 2026-11-03);
   - *the staging game server, also:* the path ADR 0002's delivery job hands the room binary through (its form not
     researched);
   - *the lobby's host (OQ-0006-3):* TCP 443 for its HTTPS endpoint;
   - *ACME:* TCP 80 on each game VM, only if OQ-0006-5 picks http-01;
   - *nothing else:* the supervisor opens its link to the lobby from the VM (part 6), so that link needs no inbound rule.
   VMs stay up and rooms start and end as processes
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
6. **The lobby's place.** The lobby (join tokens, guest identity, room codes, the room directory, and part 10's report
   endpoint for crash reports and client field data) is a required service in this record's scope (critic rank 8: "the
   infrastructure ADR and the cost budget leave out a required service", r6-infrastructure.md, Cross-topic open
   questions, R6-XT-OQ1, lines 991–997).
   Its protocol and token format are ADR 0001's (R2-ROOM-REC3, REC8), its crate ADR 0002's. Where it runs stays open
   between the two options R2 names, "Same binary as the supervisor or a separate service" (r2-netcode.md, Open
   questions, R2-ROOM-OQ9, lines 3104–3105; OQ-0006-3); what stores guest ids and the room directory is not researched
   (a database was not read: r6-infrastructure.md, Options §4, Q3, line 593). On the first option the lobby runs on VMs
   already budgeted; on the second it needs one more VM, for example a CX33 at 8.99 EUR a month (R6-HOST-F15; a
   placeholder line, not a researched sizing). Part 8's totals carry no lobby line. Its signing keys are a security
   baseline item (part 13), and its HTTPS endpoint needs its own certificate, which R6-HOST-REC4 does not cover (not
   researched). The page and the lobby sit on one registrable domain, or the guest cookie is never sent (part 13, B4;
   OQ-0006-15). It keeps VISION_V2 §6.10 possible: a later login links to the same guest id (r2-netcode.md,
   R2-ROOM-REC8, lines 2868–2885); no account store is built.
   *The supervisor's link to the lobby* (ADR 0001 hands the supervisor's calls to this record; proposed, not
   researched): each supervisor opens the link from its VM and registers the VM, its free room slots, and the build and
   protocol version of the room binaries it holds; the lobby asks it to start a room with the room id, the mode id and
   the overrides (ADR 0001's supervisor calls); the supervisor answers with the host and a port from each firewall range
   (part 5); it reports Closed, an abnormal exit and draining. The link is authenticated (how: the security
   baseline, item B5), and its message format is read with OQ-0006-3, beside ADR 0001's catalogue; routing joins by
   build is ADR 0001 part 11's, the release model OQ-0006-14's. *Ids:* R2-ROOM-OQ9, R2-XT-OQ4, R5-XT-OQ3, R6-XT-OQ1.
   *Need:* VISION_V2 §6.1 (rooms and private matches go through it).
7. **Certificates (interim position, BASELINE-WHEN (b)).** In production every game server presents
   [a certificate per hostname](../research/GLOSSARY.md#game-server-certificate), issued through ACME (RFC 8555)
   (r6-infrastructure.md, Recommendations §1, R6-HOST-REC4, lines 684–699; r2-netcode.md, Recommendations, R2-TRN-REC5,
   lines 2699–2723). The wildcard certificate (option C2: one job renews it and copies it to every game VM, so one
   private key sits on every VM) is an option the security baseline weighs. Which ACME challenge issues the
   per-hostname certificates (http-01 needs TCP port 80 on each game VM; dns-01 per name needs a DNS provider's API key,
   a secret) and how long they live is OQ-0006-5 (R6-HOST-F24, r6 lines 198–204; R6-HOST-OQ6, r6 lines 869–874). Every
   per-hostname certificate needs a domain and a DNS zone. Mustafa registers one for the netcode proof test's
   WebSocket leg ("for the WebSocket leg, a domain I register": Mustafa, 2026-11-03, SPIKE-MACHINES), and names it:
   "shamblefrontlines.com, which I register at Cloudflare" (Mustafa, 2026-11-03, DOMAIN; OQ-0006-15, answered).
   Production uses the same domain, since it is the game's name and part 6 needs one registrable domain
   (the advisor's reading, 2026-11-03). Which DNS host serves it, and what the domain and the DNS host cost, are not
   researched. Development
   and preview servers use self-signed ECDSA P-256 certificates valid at most 14 days, made by the server at start and
   pinned through `serverCertificateHashes` (R2-TRN-REC5, r2 lines 2700–2703; number N30); on the staging game server
   its supervisor makes and rotates them, one per VM (part 4). Hash pinning in production
   stays out: the VMs stay warm, so they are not the "short-lived ephemeral servers" R2-TRN-REC5 names
   (r6-infrastructure.md, Options §1, Q3, option C3, lines 426–427; r2-netcode.md, R2-TRN-OQ4, lines 3050–3054). The
   netcode proof test runs on the development certificates for WebTransport, "a private hash-pinned certificate for
   WebTransport" as Mustafa set for the spikes (2026-11-03, SPIKE-MACHINES); its WebSocket fallback presents a
   browser-trusted certificate on the domain he registers, OQ-0006-5's development case (i), issued by a method the
   proof test's own plan sets with security-auditor (the advisor's reading, 2026-11-03); the production form is needed
   from the first online
   playtest, after the security baseline's research. An alert fires when any VM's certificate has under 14 days left
   (proposed in R6-HOST-REC4's mark, r6 lines 697–699); where alerts go is part 9.
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
     (a fixed base plus VMs added by the hour, K2), each + 19–24 USD + Sentry's paid plan, match records included (r6 lines
     482–494). **Changed from R6-COST-REC1:** at the 4 rooms per VM that A-2 names "the case to budget" until
     R2-ROOM-REC2 marks 1 and 2 pass (r6 lines 516–522), the fixed fleet is ceil(20 ÷ 4) + 1 = 6 CCX23, 6 × 86.49 = 518.94
     EUR, so about 534 EUR with the staging VM and the match records (518.94 + 8.99 + 6.49 = 534.42 EUR) + 19–24 USD +
     Sentry. Neither density is a bound: at the marks' own ceilings a room may take 0.5 vCPU for the server tick (p99
     16.67 of 33.33 ms, the server tick budget, if every tick ran at it) plus up to 1 vCPU for the transport runtime
     (R2-TRN-REC1 mark 2, r2-netcode.md lines 2589–2592), 1.5 vCPU in all, so floor(4 ÷ 1.5) = 2 rooms fit one VM and
     the fixed fleet is ceil(20 ÷ 2) + 1 = 11 CCX23, 11 × 86.49 = 951.39 EUR, about 967 EUR with the staging VM and the
     match records (951.39 + 8.99 + 6.49 = 966.87 EUR) + 19–24 USD + Sentry: the most the marks allow, not a forecast.
     The goal budget is stated at both densities until the netcode proof test measures rooms per VM, and that figure
     stays provisional until slice 6 measures the real sim (OQ-0006-19; R6-COST-OQ1, r6 lines 880–882; R2-ROOM-REC2
     mark 2 is ADR 0001's test). These densities and figures are at the 30 Hz baseline (TICK-30). A 60 Hz tick also
     needs this goal budget to hold at the rooms per VM measured at 60 Hz: TICK-60-HEADROOM, confirmed by Mustafa
     (2026-11-03) "with one addition: 60 Hz also needs the goal budget to hold at the rooms per VM measured at 60 Hz".
     That count and its cost are ADR 0001's PM-0001-63, on this part's fixed-fleet formula
     (ceil(20 ÷ rooms per VM) + 1 CCX23, plus the staging VM and the match records), the same machine and N = 48. The
     budget holds when that fleet costs no more than the goal budget as it stands when the call is made: at slice 6,
     the figure OQ-0006-19 fixes; in wave 1's 60 Hz run, an early signal only, the cost is reported beside the
     provisional figures above and decides nothing (the advisor's reading, 2026-11-03; ADR 0001 part 2). Egress stays
     inside the included traffic at either density: at 4 rooms, 4 × 3.156 GB × 730 h ≈ 9.2 TB per VM, under 20 TB
     (arithmetic on r6 lines 461–463).
   - *Fleet shape:* a fixed fleet (K1) first; K2 only once real concurrency data shows a clear peak; the shared CX33
     fleet (K3) only if the shared-vCPU measurement passes; each extra room of peak capacity costs 86.49 ÷ 8 ≈ 10.81 EUR
     a month at 8 rooms per VM, 21.62 EUR at 4 (r6-infrastructure.md, Recommendations §2, R6-COST-REC1, lines 703–717;
     scaling rule, lines 499–500).
   - *Cost per match:* every match record carries its room-hours and bytes sent, and a monthly bill check compares VM
     hours and traffic against the 20 TB included per VM (r6-infrastructure.md, Recommendations §2, R6-COST-REC2, lines
     718–725); Mustafa reads the invoice in his own account.
   - *CI runners (critic rank 33):* CI runs on GitHub's standard hosted runners, "free and unlimited on public
     repositories", with the free 10 GB cache (r5-toolchain-quality.md, Findings, Question 5, R5-CI-F17, F5, lines
     673–678 and 583–592; the repository is public, lines 913–914). macOS is one of those standard runners: the same
     page's public-repository table lists `macos-latest` (3 CPUs, M1) and `macos-15-intel` (4 CPUs) among them (read by
     the infra review on 2026-11-02; R5-CI-F17 quoted only the ubuntu rows), so real Safari costs runner time, not
     money, while the repository stays public (OQ-0006-18 (c) would change that); R5-TEST-OQ4's cost half (r5 line
     1809) is answered, and its time half is ADR 0002's. No paid runner (GPU) and no self-hosted reference machine is
     proposed: the GPU runner's price was not read (R5-CI-OQ5, r5 lines 1835–1837), and self-hosted runners are "a cost
     and security question" (r3-rendering.md, Open questions, R3-MEASURE-OQ2, lines 1384–1385); reference-machine runs
     stay by hand until Mustafa decides (OQ-0006-7).
   - *The tick bench and the playtest soak (proposed):* ADR 0002's tick bench runs on one vCPU of the reference server
     from slice 1, when no game server is budgeted. A CCX23 is created per bench session and deleted after it, billed
     by the hour (R6-HOST-F17, r6 lines 164–168) at 0.1386 EUR (R6-HOST-F15), so 20 bench hours a month, a placeholder,
     cost 20 × 0.1386 = 2.77 EUR. ADR 0002's 3-hour soak before each playtest takes the same path, with its bot clients
     on a CX33 beside the reference server, never on it (as spike 2's load generators): 3 × (0.1386 + 0.0144) =
     0.46 EUR a playtest. Mustafa creates and deletes these VMs himself, and sessions get SSH access only and no
     Hetzner token, until the security baseline: OQ-0006-17's answer (Mustafa, 2026-11-03) covers the bench and the
     soaks too, since the baseline comes before the first online playtest (the advisor's reading, 2026-11-03), so every
     bench session and every soak waits for him to create its VMs (at the placeholder, 20 bench hours a month).
   *Why:* compute dominates and is flat per VM; egress, the cost that grows with players, is zero on Hetzner inside the
   20 TB and about 0.063 USD per room-hour on a per-GB host (r6 lines 706–709). *Ids:* R6-COST-REC1, R6-COST-REC2,
   R6-COST-OQ1, R6-COST-OQ3, OQ4, OQ5, R3-MEASURE-OQ2, R3-XT-OQ5, R5-XT-OQ11, R6-XT-OQ7. *Need:* VISION_V2 §6.1 (rooms
   and private matches at the §3 sizes, r6 line 713) and §6.6 (the cost line in the record, r6 line 723); the CI runner
   costs: GLOSSARY §8, the foundation (CI gate v2) only.
9. **Logs and metrics.** Each room process writes `tracing` JSON lines to standard output; the supervisor adds VM and
   room ids and ships them; no line holds a token, a guest secret, the match's random seed (ADR 0004 part 8) or a raw
   id (salted hashes where an id is needed, as R2-ROOM-REC3 mark 3 says, r2 lines 2807–2809) (r6-infrastructure.md,
   Recommendations §3, R6-OBS-REC1, lines 729–738).
   The per-command view-time offset and aim that ADR 0001 logs for later cheat detection (its part 8) never go to this
   stream: at 48 × 30 = 1,440 commands a second, a 20-minute match holds 1,440 × 1,200 = 1,728,000 entries, 8,640 times
   A-9's 200 lines a match, so about 518 MB at A-9's 300 B a line and about 6.7 TB a month at the goal's 13,000 matches,
   against the 0.8 GB below (arithmetic on r6 lines 568–569). They go to a per-match side file written beside the match
   record (proposed; its form, raw entries or per-player summaries, is ADR 0001's), keyed by slot and never by guest
   id, uploaded with the record and kept 14 days (proposed, as logs; the detection policy, item B7, may change it). At
   16 B a raw entry (proposed for scale: a slot, a tick, a view-time offset and a two-axis aim point) a side file is
   about 27.6 MB a match and about 359 GB a month at the goal, so about 168 GB is held under the 14-day cap (14 of 30
   days), inside part 12's 1 TB of object storage. Metrics use the
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
   Alerts go to Mustafa by email (proposed: a certificate under 14 days left, any room crash, sustained overruns, a VM
   disk above 80 % while logs stay on the VMs, a record waiting over 1 hour to upload); the tool (Prometheus'
   Alertmanager or Grafana Cloud's alerting) is not researched.
   *Why:* structured events at little cost; bounded labels keep the series near assumption A-8's 3,600 for 4 VMs,
   inside Grafana Cloud Free's 10k; logs come to about 0.8 GB a month at the goal's average, far under the free 50 GB;
   Prometheus' local store keeps 15 days and is not replicated, so metrics are lost with the VM, not the game
   (r6 lines 565–569 and 541; R6-COST-F1, r6 lines 219–224; R6-OBS-F5, r6 lines 285–290). *Ids:* R6-OBS-REC1, R6-OBS-REC2.
   *Need:* VISION_V2 §6.1 (per-room lifecycle and crashes) and VISION_V2 §4 ("Matches feel fair up to about 100 ms of
   ping": overruns, the server tick budget and the fallback share become visible) (r6 lines 734–735 and 749).
10. **Crash reports.** In the browser: Sentry's browser SDK with `wasmIntegration` and build ids from `wasm-split`;
    debug files uploaded by CI and never shipped (the upload job is ADR 0002's, critic rank 34); the organization
    created in Sentry's EU location; `userInfo` off, so the SDK infers no IP address, and its deny lists for HTTP
    headers, cookies and query parameters set too, because IP addresses can still reach Sentry through them
    (r6-infrastructure.md, Findings §3, R6-OBS-F10, lines 327–331, "inferred" there; Sentry's data-collected page says it
    in its own words, read by the verifier on 2026-11-01). In the browser the SDK sends only the Referer and User-Agent
    headers, and no cookies unless its httpClient integration is on (the same page, read by the skeptic on 2026-11-01),
    so by default the query parameters in the URLs it sends are the channel left. Tab crashes: Reporting API
    `crash` reports to our own report endpoint on the lobby, which also takes ADR 0003's client field data (once per
    match). It is a public write endpoint, so each body is capped and requests are rate-limited per address prefix
    (IPv4 /32, IPv6 /64, since an IPv6 interface id is 64 bits: RFC 4291 §2.5.1, read by the security review on
    2026-11-02) and globally, the values as data, and no address is kept (proposed; the security review, finding 14).
    On the server: panics go to Sentry's Rust SDK from the room
    process, and the supervisor records every abnormal room exit (r6-infrastructure.md, Recommendations §3,
    R6-OBS-REC3, lines 754–768). Mustafa creates the Sentry organization in his own account, choosing the EU location,
    which "can’t be changed" later (R6-OBS-F9, r6 lines 318–326), after he has read Sentry's prices (OQ-0006-8); its upload
    token is a secret he adds (part 13). Sentry calls its Wasm support "still developing" (R6-OBS-F7, r6 lines 297–306),
    so whether it symbolicates wasm-bindgen Rust (R6-OBS-OQ3) and which browsers send `crash` reports (R6-OBS-OQ4) are
    spike marks: in the boundary spike if the EU organization exists by then, otherwise at the first online playtest's
    milestone. The gate v2's smoke test switches reporting off or allows its endpoints (row TL9; ADR 0002, rank 34). What
    client field data is measured is ADR 0003's (rank 26), which leaves its store and retention to this record: the
    endpoint writes both kinds of report as JSON lines to Hetzner Object Storage in the game servers' location, beside
    the match records, under part 11's caps (proposed). *Why:* Wasm frames need
    debug files and build ids; tab crashes, such as running out of memory, are invisible to JavaScript but reach a
    reporting endpoint; the EU location is chosen once; the SDK sends IPs unless told not to (r6 lines 758–760). *Ids:*
    R6-OBS-REC3, R6-OBS-OQ3, R6-OBS-OQ4. *Need:* VISION_V2 §4 (four desktop browsers: crashes per browser are the
    evidence) and §6.1 (per-room crash reports) (r6 lines 763–764).
11. **Data protection.** Every store is in the EU (Hetzner EU, Sentry EU, a Grafana EU stack); no IP address, token or
    guest secret goes into logs, reports or match records; retention is capped: logs 14 days and metrics 15 days
    (proposed: the free tier's and Prometheus' defaults), match records kept while their content hash is still analysed
    (R6-TEL-OQ4's proposed default, r6 lines 917–919), the aim side files 14 days (part 9), the crash reports and field
    data at our own endpoint 14 days (proposed, as logs; a summary of the field data per build is kept like the match
    records), and each archived content set as long as a record names its hash (part 12); Sentry's crash-report cap
    stays open, because no page read shows whether Sentry lets an organization set its retention (r6-infrastructure.md,
    Recommendations §3, R6-OBS-REC4, lines
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
    events that made them; never account or guest ids, display names, IP addresses, tokens or the match's random seed
    (r6-infrastructure.md, Recommendations §4, R6-TEL-REC1, lines 784–811; the seed: ADR 0004 part 8, which never
    sends, logs or stores it outside the live state and ADR 0004's recorded match, which reaches no client while its
    match runs; the security review, finding 15). **Changed:** the per-slot section also carries the upgrade-track
    fields the decision map gives this record (part j, J5): picks per track and level, offers shown, looted weapons,
    time per weapon type; and the tick-stamped events add upgrade picks (slot, track, level, choice) and pickups (slot,
    weapon type), each with its tick, as ADR 0004 parts 18 and 19 emit them (part 18 stamps the offers shown too), so
    an analysis knows both players' track levels at every kill: ADR 0005's OQ-0005-AB3 names two measurements, earned
    power and the way back, that match telemetry must make, and VISION_V2 §6.6 asks that "Snowball and counters get
    tuned on evidence". No research finding covers these fields (UPGRADE-SCOPE has none), so they are proposed, their shape
    follows ADR 0005's track data, and carry slots, ammo for looted weapons and levelling speed stay Mustafa's open
    questions in ADR 0005. ADR 0004's sim emits the events and recognizes the moments; ADR 0001's room writes the
    record at Ended, before Closed (R2-ROOM-REC1); ADR 0005 owns the ids and the content hash; the moment rules and
    thresholds are a design question (R6-TEL-OQ3) (seam S6). Records are stored as compressed JSON-lines files in
    Hetzner Object Storage in the game servers' location, uploaded by the supervisor after the room exits, with a
    schema version in every record and a Parquet compaction job only once a query needs it; a crashed room leaves a stub
    record marked crashed, one word with ADR 0001 part 11 and PM-0001-59 (R6-TEL-REC2 calls it "a stub record marked
    incomplete"; r6-infrastructure.md, Recommendations §4, R6-TEL-REC2, lines 812–827). That stub record and the crash
    report (part 10) are written and kept under part 11's proposed retention, with no new period: OQ-0001-18, answered
    by the Advisor (2026-11-03): "end the match, as proposed, and keep the crash and match records" (what "keep" means:
    the advisor's reading, 2026-11-03). Only records a
    production supervisor uploads enter the store: a local match's record (the room code as the local authority in the
    Worker, ADR 0002 part 6) and the staging game server's records never do (proposed: the first is player-controlled
    data, the second comes from pull-request-built code; the security review, finding 16). Every content set a record
    names is archived in the same bucket by its content hash, uploaded by the supervisor beside the first record that
    names it and kept as long as any record does, so every record's hash resolves (proposed; ADR 0005 hands the archive
    to this record, interface I-6); each set is at most 25 MiB (part 1). The records are analysed in batches on a
    production game VM while no room runs on it, as its own OS user with a read-only key to the store (key scoping not
    researched, B12), never on the staging VM (part 4), always grouped by content hash and mode id, with the sample size
    beside every rate;
    the derived numbers (win rate per faction and base position, unit pick and win rates, lead changes, comeback rate,
    steals and late steals, P(win | leading at minutes 5 and 10)) are proposals for the game-designer to refine
    (R6-TEL-REC3, r6 lines 828–842); no live balance dashboard (option N2, r6 lines 599–600). Whether bot and private matches
    count in rates is Mustafa's (R6-TEL-OQ2); the record tags both. No field marks a match as official or community
    (OQ-0006-21, community servers): adding one is design for later (the advisor's reading, 2026-11-03). Kill-cams,
    replays and highlights (§6.8 `[LATER]`) would need a full replay, which is not built (option W3, r6 lines
    577–578); the tick-stamped events and ADR 0004's
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
    | B1 | Dependency audits (R5-GATE-OQ4; R1-RR-OQ7; R2-TRN-REC1's version floors) | whether an advisory blocks a pull request, and who owns that policy; and the npm lockfile's advisories (three.js, which every player runs, plus Vite and Playwright), which no topic researched (the security review, finding 10) | [the advisories check](../research/GLOSSARY.md#the-advisories-check) runs on a schedule and on every pull request that changes `Cargo.lock`, outside the PR gate (R5-GATE-REC1, r5-toolchain-quality.md lines 1671–1674); it keeps the version floors current; the npm lockfile has no advisories check yet | 0002 (the job); 0006 (the policy, open) |
    | B2 | Production certificates (R6-HOST-REC4, R2-TRN-REC5) | the wildcard against a certificate per hostname; the DNS API key | a certificate per hostname (part 7) | 0006 |
    | B3 | Content security policy (R5-PREV-REC4; R2-ROOM-OQ11) | the CSP in the production header set | none written yet; no third-party script: three.js is bundled and same-origin (part 1); the header sets take the CSP once written. So that a strict policy stays possible, the build is CSP-ready (proposed, not researched; the security review, finding 11): no inline script or style in the generated HTML, the Worker and the Wasm loaded as files (never `blob:` or `data:` URLs), and ESLint's `no-eval` and `no-new-func` in ADR 0002's lint set | 0006 (the content); 0002 (the generation, the build rule and the lint) |
    | B4 | Where the guest secret lives (R2-ROOM-OQ11) | HttpOnly cookie or local storage | R2-ROOM-REC8's HttpOnly, Secure, SameSite=Strict cookie on the lobby's origin, which the security review's SEC-GUEST recommends; Mustafa and the Advisor decide (r2-netcode.md lines 3112–3116). SameSite=Strict sends the cookie "only for requests originating from the same site that set the cookie", and a site is the registrable domain (MDN's Set-Cookie and "Site" pages, read by the security review on 2026-11-02), so the page and the lobby share one registrable domain: a custom domain on the client host with the lobby under it, and multiplayer previews the same (OQ-0006-15; whether preview links can take a custom domain was not read); otherwise a reload loses the guest id (R2-ROOM-REC8 mark 2, r2 line 2881) | 0001; 0006 (the domain) |
    | B5 | The lobby's token-signing keys (R2-ROOM-OQ2, R2-ROOM-OQ9) | how and where they are stored | only in the lobby host's environment, added by Mustafa; staging signs with a key of its own, so a production room refuses every staging token (part 4; ADR 0001's key id) | 0001 (scheme); 0006 (storage) |
    | B6 | The `textContent` rule (R5-TSLINT-OQ5) | which lint rule enforces it | CLAUDE.md's code rule ("Dynamic or player-provided text: `textContent`, never `innerHTML`") holds by review | 0002 |
    | B7 | Anti-cheat basics: the view-time offset and aim log (R2-SYNC-REC5), Origin-less requests tagged (R2-TRN-REC1), the reader-client and byte-audit tests (R2-INT-REC2, REC5) | the detection policy | built as ADR 0001 designs them; no detection policy | 0001 |
    | B8 | R1-DATA-REC7's allow list and fuzz corpus, with the server-side map validation the STATUS item SEC-MDL-2 asks for (`__proto__` through `envMerge`); COOP as an XS-Leaks guard (R1-BND-REC5) | whether they suffice | as ADR 0005 and ADR 0002 design them (SEC-MDL-2 is ADR 0005's: decision map part j, J2); production non-isolated at the start, so the isolation-only items wait (OQ-0006-12) | 0005; 0002 |
    | B9 | The tokens CI holds (the client host's deploy tokens, for previews and for production; the Sentry upload token, "a token: Mustafa's secret", r6 line 1082), a multiplayer preview's binary delivery (R6-XT-OQ4), and a Hetzner API token for the spikes, the tick bench and the 3-hour soak before each playtest, which no session gets before the baseline (OQ-0006-17, answered by Mustafa, 2026-11-03; the bench and the soak: the advisor's reading, 2026-11-03) | their scope and handling | each held only by the job that needs it, in a GitHub environment, as R5-PREV-REC1 does for the deploy token (proposed for the others); production deployed only from `main`, in its own environment `production` with a `main`-only deployment-branch rule and its own token (ADR 0002 part 10); a required reviewer on `preview`, OQ-0006-13's proposed interim (what keeps a rewritten deploy job from the token); the preview hosting check's token revoked when the check ends; no Hetzner token: Mustafa creates and deletes those VMs himself, in a Hetzner project of its own (OQ-0006-17); all added by Mustafa (below; OQ-0006-13) | 0002 (the jobs); 0006 (the scope; OQ-0006-13) |
    | B10 | The staging VM runs pull-request-built native code (R6-XT-OQ4, "PR-built native code on a VM") beside the Prometheus store (part 9) | whether they share a VM | Prometheus shares it, since it holds no secret (if M2 later pushes to Grafana Cloud, its key moves Prometheus off this VM); the batch analysis and every store key stay off it (parts 4 and 12). Fork pull requests get no preview (R5-PREV-REC1), but any branch of this repository can change its own workflow (B9), so that code is treated as untrusted | 0006 |
    | B11 | Self-hosted CI runners (R3-MEASURE-OQ2) | whether any is allowed | none (part 8) | 0006 |
    | B12 | Privilege separation on the VMs: room processes parse hostile network input, and the staging VM runs pull-request-built native code (the security review, finding 4) | how processes and keys are separated, and how far store keys can be scoped | each room process runs as its own unprivileged OS user that cannot read the supervisor's, the lobby's (OQ-0006-3 (a)) or the analysis job's credentials; the staging VM holds no secret beyond its rooms' certificate key (part 4); store keys scoped per bucket, put-only for uploads and read-only for the analysis, where the provider allows (not researched); a dns-01 key, if OQ-0006-5 picks dns-01, on no VM that runs rooms (proposed: a job elsewhere hands each VM its own certificate); while Prometheus runs on the staging VM, the metrics port is the one port of a production VM that pull-request-built code can reach (part 5) | 0006 |

    Previews are settled by PREVIEW-WALL (a) (part 2). **Secrets:** every secret lives in a GitHub environment (for
    CI) or in a VM's environment, never in the repository, and Mustafa adds each one; agents never add them
    (r6-infrastructure.md, R6-SEC-OQ1, lines 944–945; r5-toolchain-quality.md, Pointers, lines 2104–2106). The client
    host's deploy token is an API token with the scope the research read, "select *Account*, *Cloudflare Pages* and *Edit*"
    (r5-toolchain-quality.md, Findings, Question 6, R5-PREV-F10, lines 823–830; whether it can be narrowed to one
    project was not read), held only by the deploy jobs' environments, `preview` and `production` (R5-PREV-REC1, r5
    lines 1588–1604; ADR 0002's jobs). The `preview` environment alone does not keep its token from a branch: a
    `pull_request` run takes its workflow
    from the pull request's merge commit, unlike `pull_request_target`, which "runs in the context of the default branch
    of the base repository, rather than in the context of the merge commit" (R5-PREV-F2, r5 lines 770–774), so any
    branch of this repository (sessions push them) can rewrite the deploy job and use the token, and an
    account-scoped token can then deploy the production player build too (the security review's inference, finding 1).
    **Interim position until the security baseline** (proposed, from that review): (a) production is deployed only by
    its own job on pushes to `main`, in its own environment `production`, limited to `main` by a deployment-branch rule
    ("Deployment branches and tags are available for all public repositories", R5-PREV-F6, r5 lines 796–804), with its
    own token; (b) `preview` has a required reviewer ("a job cannot access environment secrets until one of the
    required reviewers approves it", the same finding), so each preview deploy waits for Mustafa's approval, unless
    OQ-0006-13 puts production where the preview token cannot reach it; (c) the preview hosting check's token is revoked
    when the check ends (part 14); (d) no session holds a Hetzner API token until the security baseline: Mustafa
    creates and deletes the VMs for the spikes, the tick bench and the 3-hour soak before each playtest himself, in a
    Hetzner project of its own, never the production project (a project's token presumably reaches all its VMs: not
    read; OQ-0006-17, answered by Mustafa, 2026-11-03: "a separate Hetzner project. I create and delete the VMs myself
    for each run; sessions get SSH access only and no Hetzner token until the security baseline"; the bench and the
    soaks: the advisor's reading, 2026-11-03). Whether a session, which posts on GitHub as Mustafa's account
    (CLAUDE.md), could approve a pending
    deploy, and how a deployment-branch rule treats a pull request's merge ref, were not read: the preview hosting
    check tests the second (What a spike must prove, item 1) and OQ-0006-13 carries the first. *Ids:* BASELINE-WHEN,
    R6-SEC-OQ1, R1-RR-OQ7, R5-GATE-OQ4, R5-TSLINT-OQ5, R2-ROOM-OQ11, R2-XT-OQ9,
    R5-XT-OQ5, R6-XT-OQ3, R1-BND-OQ5, R1-XT-OQ17, R6-XT-OQ9. *Need:* no §6 item names it; it is the condition for the
    first online playtest (BASELINE-WHEN (b)); the deploy token: GLOSSARY §8, the foundation only.
14. **Its spike and its operations marks.** The preview hosting check (small, on a test project in Mustafa's
    Cloudflare account, before the foundation's preview step) proves part 1, PREV-DELETE and B9's interim position, and
    its token is revoked when it ends; the netcode proof test
    (ADR 0001's) carries this record's operations marks: rooms per VM on both machines, egress per match-hour, the
    spike's own bill, logging cost, the metrics and their path, and the firewall. Both are listed under "What a spike
    must prove". *Need:* as parts 1, 3, 5, 8 and 9.
15. **Releases (open: OQ-0006-14).** How a new client build, content set or room binary reaches production, how two
    versions meet while rooms run, and how a bad server release is rolled back are not designed, and no topic
    researched them. As things stand, the lobby sends a client only to rooms of its own build and content hash, and a
    refused client reloads at most once (ADR 0001 part 11 and its failure table), but with no release model a new client
    finds no room of its build until rooms on the new binary run. OQ-0006-14 decides the model before the first online
    playtest. Interim position (proposed): a release is made only while no room runs, with the client and every VM's
    room binary switched together, and the previous room binary stays on each VM, so a bad server release is switched
    back the same way. The production deploy job is ADR 0002's (part 13, B9); routing joins by build is ADR 0001's
    protocol (its part 11). *Need:* VISION_V2 §6.1 (rooms and private matches stay reachable across a release).

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
| **G1. Hetzner Cloud, dedicated vCPU (CCX23 as the reference server, proposed)** | Medium: our own VMs, a supervisor, a firewall; no load balancer, the lobby hands out host and port | CCX23 86.49 EUR a month (0.1386 an hour), 20 TB included, 1 EUR per extra TB (an inferred reading: R6-HOST-F15, F16, r6 lines 150–163) | "highly predictable resources", sold for gaming servers (R6-HOST-F14, r6 lines 144–149) | Firewall filters UDP, stateful, drops what no rule allows (R6-HOST-F18, r6 lines 169–174) | Fair server ticks serve VISION_V2 §4's "fair up to about 100 ms of ping" | From zero | **Pros:** predictable tick; egress free at full load. **Cons:** EU plans showed "currently unavailable" on 2026-10-28 (R6-HOST-F20, r6 lines 180–182); a single provider |
| **G2. Hetzner Cloud, shared vCPU (CX33, measured beside G1)** | As G1 | CX33 8.99 EUR a month (0.0144 an hour) (F15) | "designed for variable usage patterns rather than sustained high workloads" (F14): server tick [p99](../research/GLOSSARY.md#time-statistics) must be measured | As G1 | Noisy neighbours could break the server tick budget (r6 line 416) | From zero | **Pros:** 9.6 times cheaper. **Cons:** unproven for a fair tick |
| G3. Fly.io Machines | UDP not read (outside the allow list) | performance-2x 66 USD a month plus 0.02 USD per GB of egress (R6-HOST-F22, r6 lines 186–192) | Dedicated-ness not read | Not read | Not read | From zero | **Cons:** 3.16 GB per 24v24 match-hour makes egress about 369 USD a month per loaded host (r6 lines 461–464) |
| G4. Amazon GameLift Servers | Managed fleets, not read in detail | c6a.large 0.099 USD an hour on demand, 0.03 on Spot, Ohio; bandwidth free on generation 6 and later; EU rates not read (R6-HOST-F23, r6 lines 193–197) | Managed scaling, not read | Not read | Not read | From zero | **Cons:** lock-in to its server SDK (not read) |
| Hetzner dedicated root servers; Netcup, OVHcloud, Scaleway | not researched (R6-HOST-OQ1, OQ4, r6 lines 852–855 and 862–865) | not researched | not researched | not researched | not researched | not researched | Netcup, OVHcloud, Scaleway: read only if OQ-0006-1 rules Hetzner Cloud out; Hetzner's dedicated root servers, "a further candidate at goal scale", are read before part 8's goal budget is fixed, whatever OQ-0006-1's answer (R6-HOST-OQ4, r6 lines 863–865) |

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
| tls-alpn-01 (RFC 8737: "This connection MUST use TCP port 443.") | not researched beyond that quote (r6 lines 871–874) | not researched | not researched | not researched | not researched | not researched | Named for OQ-0006-5 and the port plan (OQ-0006-16) |

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
| **Sentry browser SDK with `wasmIntegration`, debug files uploaded at build (C1, proposed)** | Medium: a CI upload job (ADR 0002) | Free tier, then per error (R6-COST-F3, r6 lines 229–235; base prices not read) | Wasm frames with symbols, "still developing" (R6-OBS-F7, r6 lines 297–306) | EU storage chosen at creation; IP off by setting plus deny lists for headers, cookies and query parameters (R6-OBS-F9, F10, r6 lines 318–331; part 10); a CI secret | None | From zero | **Pros:** Rust function names in traps. **Cons:** an unproven Wasm path |
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
| **N1. Batch analysis per content hash and mode id (proposed)** | Low | 0 (a production game VM while no room runs, never the staging VM: part 12) | A month of goal records in under 10 minutes (R6-TEL-REC3's mark) | Records hold no player ids | None | From zero | **Pros:** comparable within one tuning version |
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
| **Standard GitHub-hosted runners and the free cache (proposed)** | Low | "free and unlimited on public repositories"; cache free to 10 GB, more needs a Pro, Team or Enterprise account (R5-CI-F17, F5, r5 lines 673–678 and 583–592) | ubuntu-24.04: 4 CPUs, 16 GB, no GPU (F17) | GitHub's isolation; today's hygiene carries over (row KT8) | WebGPU not covered in CI (R5-CI-OQ5) | Today's CI already runs on them (row KT8) | **Pros:** 0 cost. **Cons:** no GPU; real Safari only on the macOS runners of the next row |
| Standard macOS runners for real Safari (`macos-latest`; part of the proposed row, their use is ADR 0002's) | Low | 0 on a public repository: the runners page lists macOS among the standard runners, "free and unlimited on public repositories" (read by the infra review on 2026-11-02; R5-CI-F17 quoted the ubuntu rows only) | 3 CPUs (M1) or 4 (Intel), 7 or 14 GB (same page) | GitHub's | Safari covered | Small | **Pros:** 0 cost while the repository is public. **Cons:** wall time against ADR 0002's budget not known (R5-TEST-OQ4's time half, r5 line 1809) |
| Larger paid runners (GPU) | Low | Price not read (R5-CI-OQ5, r5 lines 1835–1837) | GPU | GitHub's | WebGPU covered | Small | **Cons:** unpriced; Mustafa's call |
| Self-hosted reference machines | not researched: "a cost and security question" (R3-MEASURE-OQ2, r3 lines 1384–1385) | not researched | not researched | not researched | not researched | not researched | Named for OQ-0006-7 |

## Trade-off analysis

- **The client host (Q1).** Pages wins because it is the one host the research read that meets every need of
  R5-PREV-REC2's list but one, at 0 USD on its Free plan: headers from `_headers`, a link per branch, `noindex` by
  default, an access policy and deletion through Wrangler (R6-HOST-REC1, r6 lines 627–634; r5-toolchain-quality.md,
  R5-PREV-REC2, lines 1607–1609). The strongest argument
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
  is less tidy than R6-COST-REC1's single range, but A-2 itself says 8 rooms per VM is "a density to measure at the tick
  rate R1-TICK-REC2's rule picks (R2-ROOM-REC2 mark 2), not one the server tick budget guarantees" (r6 lines 516–522;
  that rate is now TICK-30's 30 Hz): at the server tick budget's ceiling, 50 % of one vCPU per room,
  two rooms fill a vCPU before any [transport runtime time](../research/GLOSSARY.md#transport-runtime-time). With the
  transport runtime's own ceiling of 1 vCPU per room (R2-TRN-REC1 mark 2), 2 rooms per VM and about 967 EUR are the most
  the marks allow (part 8). The netcode proof test's measurement replaces these figures, provisionally until slice 6
  measures the real sim (OQ-0006-19).
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
  no player can join a game room before then except through a multiplayer preview behind the access wall, because
  staging rooms admit only tokens from an endpoint behind that wall, signed with a staging key production refuses
  (part 4). The staging VM's ports themselves are public, so its rooms parse hostile input as production's will (B12),
  and until OQ-0006-13 is answered every preview deploy waits for Mustafa's approval (B9).
- **What breaks if a provider is down** (inferred from the parts above, not measured): Cloudflare down, no new client
  or preview loads; Hetzner Cloud down, no online match runs, while local play with bots still runs in the Worker
  (SLICE-ORDER condition 1); object storage down, records wait on the VM's disk and upload later (R6-TEL-REC2, r6 lines
  817–818); Sentry or Grafana down, reports and metrics are lost for that time and the game runs on; the DNS provider
  down (once the game's domain is registered, OQ-0006-15), names stop resolving as cached records expire, so new joins
  fail while running rooms go on.

## Consequences

- **Easier:** every server mark in ADR 0001 and ADR 0004 is measured on one named machine (N34, I-15); a preview link
  per pull request reaches Mustafa's Brave from the foundation on; cost per match is a query over the match records,
  not a guess (R6-COST-REC2); balance work starts from one versioned record per match, comparable within a content
  hash (R6-TEL-REC3).
- **Harder:** Mustafa holds accounts with Cloudflare and Hetzner before the first spikes (a test project on
  Cloudflare Pages and a Hetzner console check), and later with Sentry and Grafana; until the security baseline he
  creates and deletes every VM of the spikes, the tick bench and the playtest soaks himself (OQ-0006-17, Mustafa,
  2026-11-03; the bench and the soaks: the advisor's reading, 2026-11-03) and gives his OK to each run's cost cap in
  that run's plan (OQ-0006-20, Mustafa, 2026-11-03); he registers shamblefrontlines.com at Cloudflare (OQ-0006-15,
  Mustafa, 2026-11-03) before the netcode proof test (the advisor's reading, 2026-11-03); every multiplayer preview
  needs his login once per Brave session; each production game VM needs a certificate issuance path (OQ-0006-5) and a
  domain (OQ-0006-15); the lobby still has no place in the budget (OQ-0006-3); until OQ-0006-13 is answered, every
  preview deploy waits for his approval (B9); releases wait on OQ-0006-14.
- **Revisit when:** Hetzner's EU plans are not orderable (OQ-0006-1); PREV-DELETE fails on Pages (move to Workers
  static assets); the netcode proof test measures rooms per VM (a provisional goal budget) and slice 6 re-measures it
  on the real sim (the goal budget fixed, OQ-0006-19); the shared CX33 passes (K3) or
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
    page, GameLift's EU prices and server SDK, and Netcup, OVHcloud or Scaleway (none read: R6-HOST-OQ1, OQ4, r6 lines
    852–855 and 862–863), then name a new reference server, which every draft citing N34 follows. Whatever the answer,
    Hetzner's dedicated root servers, "a further candidate at goal scale", are read before part 8's goal budget is fixed
    (R6-HOST-OQ4, r6 lines 863–865; migration step 1). The IPv4 price is read at sign-up; it is under 1 EUR per VM
    either way (R6-HOST-OQ7, r6 lines 875–876).
  - *Decider:* Mustafa (a console check in his own account; agents create no account), then a research read if (b).
  - *What decides it:* the console offering both plans in `nbg1` or `hel1` before the netcode proof test is scheduled.
  - *Carries:* R6-HOST-OQ1, R6-HOST-OQ4, R6-HOST-OQ7.
- **OQ-0006-2. Which EU location: Nuremberg, Helsinki, or Falkenstein if offered?**
  - *Why it matters:* VISION_V2 §4 asks for matches that feel fair up to about 100 ms of ping from one EU region.
  - *Options:* `nbg1`, `hel1`; `fsn1` only if the plans are offered there (R6-HOST-F13, r6 lines 136–143).
  - *Decider:* a measurement made before the reference server is created (a small check at the start of the netcode
    proof test's setup), confirmed by Mustafa; the reference server is then created in the location it picks.
  - *What decides it:* round-trip times from Mustafa's machine and from playtesters to each location, against §4's
    "about 100 ms" (R6-HOST-OQ2, r6 lines 856–858).
  - *Carries:* R6-HOST-OQ2.
- **OQ-0006-3. Where does the lobby run, and what stores guest ids and the room directory?**
  - *Why it matters:* join tokens, guest identity, room codes, crash reports and client field data go through it, and
    no budget holds it yet (R6-XT-OQ1, r6 lines 991–997). Under (a) its private signing key sits on VMs whose room
    processes parse hostile network input, so the asymmetric token's gain (rooms hold only the public key, ADR 0001)
    rests on part 13's item B12; the endpoint that mints staging tokens behind the access wall is read with it (part 4).
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
    domain: R6-HOST-F4, r6 lines 80–86, which Cloudflare's page confirms, read by the verifier on 2026-11-01; whether
    branch-alias links are covered is not stated there).
  - *Carries:* no research id (it follows from PREVIEW-WALL (a)).
- **OQ-0006-5. How are the per-hostname production certificates issued, how long do they live, and which certificate
  does the WebSocket fallback present on development and staging servers?**
  - Answered in part by Mustafa (2026-11-03): "Certificates: a private hash-pinned certificate for WebTransport; for
    the WebSocket leg, a domain I register." So the netcode proof test's WebSocket leg takes the development case (i)
    below, a trusted certificate on his domain (the advisor's reading, 2026-11-03); still open: how that certificate is
    issued on a spike VM (http-01 and tls-alpn-01 need an open port, dns-01 a secret, and OQ-0006-16's port plan
    applies), decided in the netcode proof test's own plan with security-auditor; the staging case, shapes (A) and (B)
    below, decided by the Advisor; and the production certificates, decided by security-auditor in the security
    baseline.
  - *Why it matters:* every production game VM needs one before the first online playtest, and the challenge decides
    which port or secret each VM needs. A WebSocket cannot pin a certificate hash: its constructor takes only a URL and
    sub-protocols (r2-netcode.md, Findings, R2-ROOM-F12, lines 1494–1500), so the fallback's `wss` endpoint on a
    development or staging server needs a certificate the browsers trust, and the netcode proof test's WebSocket leg
    needs one first (PM-0006-15).
  - *Options:* for production, ACME http-01 (TCP port 80 open on each game VM); dns-01 per name (a DNS provider API
    key, a secret); tls-alpn-01 (RFC 8737: "This connection MUST use TCP port 443."); and, only if the security baseline
    accepts a shared key, the wildcard certificate (option C2) (R6-HOST-F24, r6 lines 198–204; R6-HOST-OQ6, r6 lines
    869–874); each needs a domain and a DNS zone (OQ-0006-15). For the WebSocket fallback on development and staging
    servers (not researched; R2-TRN-REC5's self-signed, pinned certificates, r2 lines 2700–2703, serve WebTransport
    only): (i) a publicly trusted certificate for a real hostname, which needs the domain (OQ-0006-15); (ii) a private
    test certificate authority installed on the test machines, which needs no domain, but whose key can sign for any
    name, so the security baseline rules on its handling. For the staging game server, ADR 0002 part 10's two shapes:
    (A) it presents that trusted certificate on both transports, so multiplayer previews pin no hash and the delivery
    job hands over only the binary; this departs from R2-TRN-REC5's development and preview certificates and from
    GLOSSARY §5's definition of the staging game server, which would follow; (B) it keeps pinning: the supervisor keeps
    one certificate per VM across restarts and makes the next one ahead, and the preview's file lists both hashes (part
    4's interim position). Part 4 states which one the staging game server presents once this is answered.
  - *Decider:* security-auditor in the security baseline, with the port plan (OQ-0006-16); the development and
    staging case before the netcode proof test starts (the Advisor, from a short read).
  - *What decides it:* a read of the chosen certificate authority's policy (Let's Encrypt's pages were outside the
    allow list) and the baseline's key-handling rules; for development and staging, a read of how those servers can
    present a trusted certificate.
  - *Carries:* R6-HOST-OQ6 (and the verifier's finding 7, which has no research id).
- **OQ-0006-6. The security baseline: what it decides for each waiting item.**
  - *Why it matters:* part 13's items hold interim positions only; BASELINE-WHEN (b) closes them before the first
    online playtest.
  - *Options:* per item, as part 13's table lists them (B1–B12); among them, whether an advisory blocks a pull request
    (R5-GATE-OQ4, r5-toolchain-quality.md lines 1873–1876; R1-RR-OQ7, r1-simulation-core.md lines 2955–2961), the rule that enforces
    `textContent` (R5-TSLINT-OQ5, r5-toolchain-quality.md lines 1776–1778), and where the guest secret lives (R2-ROOM-OQ11, r2-netcode.md lines
    3112–3116: Mustafa and the Advisor, SEC-GUEST recommending the cookie).
  - *Decider:* security-auditor, in its own fresh session (BASELINE-WHEN (b)); the Advisor and Mustafa for the policy
    calls.
  - *What decides it:* the baseline's research, before the first online playtest.
  - *Carries:* R6-SEC-OQ1, R5-GATE-OQ4, R1-RR-OQ7, R5-TSLINT-OQ5, R2-ROOM-OQ11.
- **OQ-0006-7. Will any CI runner be paid for: GPU, self-hosted reference machines, or a cache above 10 GB?**
  - *Why it matters:* WebGPU coverage in CI and reference-machine runs depend on it (critic rank 33). Real-Safari runs
    do not: standard macOS runners are free on a public repository (part 8) and cost only runner time.
  - *Options:* (a) standard runners only, macOS included, reference-machine runs by hand (part 8's interim position);
    (b) larger paid runners for GPU, price not read (R5-CI-OQ5, r5 lines 1835–1837); (c) self-hosted reference machines,
    "a cost and security question" (R3-MEASURE-OQ2, r3-rendering.md lines 1384–1385); (d) a cache above 10 GB, which
    needs a Pro, Team or Enterprise account (R5-CI-F5, r5 lines 583–592).
  - *Decider:* Mustafa for any paid plan; ADR 0002 says which jobs need one; the security baseline weighs (c).
  - *What decides it:* a price read, then the first month's CI times and misses against ADR 0002's budget.
  - *Carries:* R3-MEASURE-OQ2, R3-XT-OQ5, R5-XT-OQ11, R6-XT-OQ7.
- **OQ-0006-8. What do Sentry and Grafana really cost, and how do logs and metrics reach Grafana Cloud?**
  - *Why it matters:* the goal budget names "Sentry's paid plan" without a price, and M2 waits on the ingestion path.
  - *Options:* Sentry's Developer plan or a paid plan (base prices on sentry.io, outside the allow list; whether its
    retention can be set); Grafana Cloud Free under 10k series, or Pro at 19 USD plus usage (R6-COST-F1, F2, F3, r6 lines
    219–235).
  - *Decider:* Mustafa, with a read of sentry.io/pricing before he creates the Sentry organization; a read of
    Grafana's ingestion docs if M2 is wanted.
  - *What decides it:* those reads and the series count the netcode proof test reports (R6-OBS-REC2's mark).
  - *Carries:* R6-COST-OQ2, R6-OBS-OQ2.
- **OQ-0006-9. What does EU data-protection law require of us, and how long are match records kept?**
  - *Why it matters:* logs, crash reports, client field data, match records and the per-command aim side files (part
    9: pseudonymous behaviour data per player) are stored from slice 3 on; whether an IP address or a guest id is
    personal data, the lawful basis, the privacy notice and the retention limits were not researched.
  - *Options:* the proposed defaults (logs, the aim side files and our own endpoint's reports 14 days, metrics 15 days,
    records and archived content sets kept while their content hash is analysed), or what the law requires.
  - *Decider:* Mustafa.
  - *What decides it:* a read of the GDPR and the European Data Protection Board's guidance (both outside the allow
    list), or advice, before any public playtest (R6-OBS-OQ1, r6 lines 896–899; R6-TEL-OQ4, r6 lines 917–919).
  - *Carries:* R6-OBS-OQ1, R6-TEL-OQ4.
- **OQ-0006-10. How many players, and when; which VAT and currency; is included traffic per VM, pooled or prorated?**
  - *Why it matters:* the start and goal budgets rest on placeholder player counts (A-4, A-5) and exclude VAT. Since
    2026-11-03 a 60 Hz tick also needs the goal budget to hold at the rooms per VM measured at 60 Hz (TICK-60-HEADROOM's
    addition, part 8; ADR 0001's PM-0001-63), and that check sizes its fleet from A-5's peak of 20 rooms, so this
    question's answer bears on it too.
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
  - *What decides it:* a statistics source, and a transcript or paper on an allowed domain (R6-TEL-OQ1, OQ5, r6 lines
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
- **OQ-0006-13. How is the production player build kept out of the preview deploy token's reach?**
  - *Why it matters:* a `pull_request` run takes its workflow from the pull request's merge commit (R5-PREV-F2, r5
    lines 770–774), so any branch of this repository can rewrite the deploy job and use its token, and the Pages token
    is account-scoped as far as was read (R5-PREV-F10, r5 lines 823–830), so it may deploy production too. The first
    such token arrives with the preview hosting check, months before the security baseline (BASELINE-WHEN (b)).
  - *Options:* (a) a required reviewer on `preview`, and on the delivery and Sentry jobs' environments when they run
    for a preview (part 13's interim position): Mustafa approves each preview deploy ("a job cannot access environment
    secrets until one of the required reviewers approves it", R5-PREV-F6, r5 lines 796–804); who approves when
    sessions push and post as his account (CLAUDE.md), and whether GitHub can stop an account approving a run it
    started, were not read; (b) production in a second Cloudflare account with its own token, so the preview token
    cannot reach it, and `preview` without a reviewer (not researched; 0 USD if it stays on the Free plan, inferred as
    in part 1); (c) a token limited to the preview project, if Cloudflare offers one (not read). Beside them, the
    security review's question 1: whether B9's part of the security baseline is read before the foundation's preview
    step; that review recommends not, since the interim position needs no new research. ADR 0002 part 10 points here.
  - *Decider:* the Advisor, with security-auditor; Mustafa for a second account and for the approvals (a) asks of him.
  - *What decides it:* a read of GitHub's rules for reviewing pending deployments and of Cloudflare's token scopes,
    and the approvals' cost per pull request; before the first client-host token is added to this repository; the
    preview hosting check's B9 marks (What a spike must prove, item 1).
  - *Carries:* no research id (the security review's finding 1 and its question 1).
- **OQ-0006-14. How do releases reach production while rooms run, and how is a bad server release rolled back?**
  - *Why it matters:* the client host switches production at once (inferred), while rooms still run the previous room binary
    and content; under ADR 0001's version rule ("a message change is a version bump") every old room refuses a new
    client with `JoinRejected`, and the lobby sends it only to rooms of its build (ADR 0001 part 11), so from the
    first online playtest every release is something players feel (part 15). R6 sizes the goal fleet with "one VM for
    draining during deploys" (A-5, r6 lines 482–483) and designs no deploy; R2 notes that "WT_DRAIN_SESSION can drain a
    session before a server shuts down" (r2-netcode.md, Pointers between the R2 units, R2-TRN-F37, lines 3439–3440).
  - *Options* (not researched): (a) a release window: rooms drained, then the client and every VM's room binary
    switched together (the simplest; no player can join during it); (b) live releases: the new room binary installed
    beside the old one on every VM, the lobby's room directory carrying each room's build and content hash and sending
    a client only to rooms of its own build (as ADR 0001 part 11's directory does), old rooms finishing, then the
    client switched, with the previous hashed bundle still served. With either, the previous room binary stays on each
    VM for a rollback; how the client host puts a previous deployment back was not read.
  - *Decider:* the Advisor for the model (the engine-architect review asks for this open question now, answered
    beside the security baseline); Mustafa for when players may be refused (the infra review recommends (a) while
    playtests are scheduled, and (b) once players arrive unscheduled).
  - *What decides it:* a short read beside the security baseline, before the first online playtest.
  - *Carries:* no research id (the engine-architect review's finding 5 and its question 2; the infra review's finding
    1 and its question RELEASE-WINDOW).
- **OQ-0006-15. Which domain do the client host, the lobby and the game servers use, and when is it registered?**
  - Answered by Mustafa (2026-11-03): "for the WebSocket leg, a domain I register." (SPIKE-MACHINES) and
    "shamblefrontlines.com, which I register at Cloudflare." (DOMAIN). So (b) is decided for the netcode proof test: a
    domain he registers before it, for its WebSocket leg (the advisor's reading, 2026-11-03). Production uses the same
    domain, since it is the game's name and part 6 needs one registrable domain (the advisor's reading, 2026-11-03).
    Which DNS host serves it, and what the domain and the DNS host cost, are not researched. The options below stay as
    the record of what was weighed.
  - *Why it matters:* until this answer no draft named a domain, and none names a DNS provider; yet each per-hostname
    certificate (part 7), the lobby's HTTPS endpoint (part 6), a dns-01 key and the guest cookie need a domain:
    SameSite=Strict sends the cookie only within the site that set it, and a site is the registrable domain (part 13,
    B4). Before the netcode proof test, its `wss` endpoint needs a certificate the browsers trust (OQ-0006-5).
  - *Options:* (a) a private test certificate authority for the netcode proof test (OQ-0006-5 (ii)) and a domain
    before slice 3; (b) a domain before the netcode proof test. With either, the registrar, the DNS provider and their
    prices are not researched, and whether `*.pages.dev` hosts count as separate sites (the Public Suffix List) and
    whether preview links can take a custom domain were not read.
  - *Decider:* Mustafa (an account and a cost), after the Advisor's OQ-0006-5 read; the infra review recommends (a).
  - *What decides it:* that read and a price read of a registrar and a DNS provider; before the netcode proof test
    under (b), before slice 3 under (a).
  - *Carries:* no research id (the infra review's finding 6 and its question DOMAIN; the security review's finding 3).
- **OQ-0006-16. The port plan: which UDP and TCP ports do rooms listen on, and must the WebSocket fallback be
  reachable on TCP 443?**
  - *Why it matters:* one process per room gives one port per room (r2-netcode.md, Findings, R2-ROOM-F18, line 1562),
    and the firewall opens a range of each kind (part 5); R6 leaves tls-alpn-01's TCP 443 to "the port plan"
    (R6-HOST-OQ6, r6 lines 873–874), and ADR 0001's transport names no port. The fallback serves "all four browsers, on
    all networks" (r2 line 2661), yet a TCP port other than 443 may be blocked on the same strict networks that block
    QUIC (a question; no source read). A connection to an address's TCP 443 reaches one listener (no source read), so
    443 needs a front on each VM that routes by path to the room processes, or an address per room.
  - *Options* (not researched): (a) a UDP range and a TCP range, one port of each per room; (b) the fallback on TCP
    443 through a front per VM that routes by path; (c) an address per room.
  - *Decider:* infra-engineer with netcode-architect (ADR 0001's transport), confirmed by the Advisor.
  - *What decides it:* a read of which networks block TCP ports other than 443, and the fallback share OQ-0001-13
    measures; the ranges before the netcode proof test (its firewall marks name them), the 443 question before slice 3.
  - *Carries:* no research id (the infra review's finding 7; it is the port plan R6-HOST-OQ6 names, which OQ-0006-5
    carries).
- **OQ-0006-17. How do sessions get VMs for the spikes, the tick bench and the 3-hour soak before each playtest
  without holding a secret?**
  - Answered by Mustafa (2026-11-03): "a separate Hetzner project. I create and delete the VMs myself for each run;
    sessions get SSH access only and no Hetzner token until the security baseline. VMs exist only during a run." So
    (a) is decided, in (b)'s project of its own but with no token; (b)'s token and (c) stay below as the record of
    what was weighed. The security baseline comes before the first online playtest, so this covers the tick bench and
    the 3-hour soaks too; how a session's SSH key is made and held without it reading a secret it did not make is for
    the netcode proof test's own plan, with security-auditor and the advisor (the advisor's reading, 2026-11-03).
  - *Why it matters:* the netcode proof test, the tick bench and the playtest soak (part 8) create VMs and delete them
    after each
    session, since billing rounds up to the hour (R6-HOST-F17, r6 lines 164–168); a Hetzner API token can do that, and
    presumably reach every other VM of its project (not read), while sessions never read secrets (CLAUDE.md). The SSH
    key of part 5's admin rule raises the same question.
  - *Options:* (a) Mustafa creates and deletes every VM by hand; (b) a Hetzner project of its own for the spikes and
    the bench, never the production project, with its own token, set by Mustafa where a session can use it without
    reading it (how: not researched); (c) a self-hosted runner (none for now, part 13, B11).
  - *Decider:* Mustafa (his account), with the Advisor; the infra review recommends (b).
  - *What decides it:* his choice before the netcode proof test, with OQ-0006-1's console check.
  - *Carries:* no research id (the infra review's finding 14 and its question SPIKE-VM-ACCESS).
- **OQ-0006-18. Must unreleased art stay unseen, or is a preview behind a login enough?**
  - *Why it matters:* the access wall hides previews, but this repository is public, so every model file committed to
    it can be read by anyone once pushed (part 2), and LOOK_V2 §8 allows authored models.
  - *Options:* (a) a login on previews is enough: models are committed, and the wall covers previews only; (b) models
    live in a private store outside the repository and the build fetches them with a secret, so the build job is no
    longer secret-free, which changes R5-PREV-REC1's shape (the store is not researched); (c) the repository becomes
    private: standard runners on a private repository "draw from your GitHub account's included compute usage, and are
    then charged at the per-minute rates" (the runners page, read by the infra review on 2026-11-02), on 2-CPU, 8 GB
    Linux runners (R5-CI-F17, r5 lines 673–678), prices not read; and environment reviewers are "only available for
    public repositories" on the Free, Pro and Team plans (R5-PREV-F6, r5 lines 796–804), so B9's reviewer would need a
    paid plan.
  - *Decider:* Mustafa, with ADR 0005's OQ-0005-AP1 (where model files live), before the first authored model is
    committed; the infra review recommends (a) until the first model that must stay unseen.
  - *What decides it:* whether an unreleased model seen early matters to him.
  - *Carries:* no research id (the infra review's finding 12 and its question UNRELEASED-ART).
- **OQ-0006-19. Is the goal budget provisional until slice 6 measures rooms per VM with the real sim?**
  - *Why it matters:* the netcode proof test measures rooms per VM on its throwaway sim, a stand-in for the stress
    scene (ADR 0001, OQ-0001-2), and the sim step share may take up to 8.33 ms of the server tick budget's 16.67 ms
    (ADR 0001, part 3); a lighter stand-in overstates the density, so it understates the monthly cost, and the
    real-sim run comes only at slice 6 (migration step 7). Since 2026-11-03 the 60 Hz rule reads this goal budget too
    (TICK-60-HEADROOM's addition, part 8): at slice 6, the 60 Hz goal-budget check (ADR 0001's PM-0001-63) is judged
    against the figure this question fixes (the advisor's reading, 2026-11-03).
  - *Options:* (a) in the density runs the stand-in's sim step is padded to the sim step share's ceiling (or replays
    the step times the sim core spike recorded, OQ-0001-2's option (b) shape), so the figure is a lower bound on rooms
    per VM for any sim inside its share, and the goal budget is provisional until slice 6 (the interim position: part
    8; What a spike must prove, item 2); (b) the figure taken as measured; (c) no goal budget until slice 6.
  - *Decider:* Mustafa (money), on the Advisor's advice; the test review recommends (a).
  - *What decides it:* the Advisor's reading, before the netcode proof test's density runs.
  - *Carries:* no research id (the test review's finding 2 and its question 2; R6-COST-OQ1 stays with the netcode
    proof test).
- **OQ-0006-20. What may the netcode proof test's VMs cost?**
  - Answered by Mustafa (2026-11-03): "one cap per run, from its planned hours, written in that run's plan for my
    OK. I create and delete the VMs myself." (SPIKE-COST), after "VMs exist only during a run." (SPIKE-MACHINES). The
    options below, the interim among them, stay as the record of what was weighed.
  - *Why it matters:* R6 sized "the spike's own bill (a few hours of one or two VMs) under 5 EUR" (r6 lines 716–717),
    but the pass marks now plan far more: on RS alone about 23 hours for this record's rows (the infra review's
    arithmetic on the rows' run plans: PM-0006-14 about 14.3 h, PM-0006-13 3 × 1.1 h, PM-0006-17 about 2.2 h, the soak
    and logging rows about 3 h; 14.3 + 3.3 + 2.2 + 3 ≈ 23), 23 × 0.1386 = 3.19 EUR, before ADR 0001's 3-run rows
    (perhaps 15 to 30 RS-hours, proposed: the infra review's guess, which the run plan replaces with a sum;
    15 to 30 × 0.1386 = 2.08 to 4.16 EUR) and the CX33 hours of SH, MV and the load generators (as many as spike 2's
    calibration run asks); so a 5 EUR cap over everything would send the plan back to the draft at its planning check.
  - *Options:* (a) 5 EUR for this record's own operations runs (R6's scope), with ADR 0001's runs on their own line
    of the run plan; (b) one cap set from the run plan, written as arithmetic at R6-HOST-F15's hourly prices (0.1386
    EUR a CCX23-hour, 0.0144 a CX33-hour) before the first session; (c) 5 EUR over everything, as written before.
    Until it is answered, PM-0006-18's planning check writes the run plan's cost, and the plan goes to Mustafa before
    any VM is created. The test review recommends (b): both records' runs bill to one Hetzner project (OQ-0006-17).
  - *Decider:* Mustafa (money).
  - *What decides it:* the run plan's hours (PM-0006-14 writes it before the first session) at the hourly prices.
  - *Carries:* no research id of its own: R6-COST-REC1's bill mark (r6 lines 716–717; its disposition stays
    "changed", Appendix A) and R6-HOST-F15's prices (r6 lines 150–159).
- **OQ-0006-21. Could the game one day let its community host servers, and what must the drafts keep possible for
  it?**
  - Mustafa (2026-11-03), word for word: "COMMUNITY-SERVERS (new; keep the door open, don't design it): players may
    one day host their own servers, possibly with revenue such as official private servers they rent. Add it as an
    open question to ADR 0006 and the README. The drafts must not lock it out: the room server stays one
    self-contained program with a config file (players, length, rules), checks the game's version and content hash,
    and marks each match as official or community, so progression can count on official servers only."
  - *Why it matters:* "The drafts must not lock it out" (his words above); part 12's match record points here.
  - *Options:* none weighed ("keep the door open, don't design it").
  - *Decider:* Mustafa.
  - *What decides it:* no timing is set.
  - *Carries:* no research id. Related open design questions it does not answer: R2-ROOM-OQ7 (whether private
    matches count for Service points and telemetry) and R6-TEL-OQ2 (whether bot and private matches count in balance
    rates, part 12). Progression lives under VISION_V2 §6.10 `[LATER]` (the advisor's reading, 2026-11-03).

## What a spike must prove

Two spikes carry this record's claims before it is accepted; the spike plan gives their final names. Every mark below
is a candidate from the research ("proposed" there); test-engineer turns them into the pass marks. Where the research
gives 48 and 54, 48 holds (MODES-3 (a)); every server mark runs at the 30 Hz baseline (TICK-30), and the 60 Hz margin
run is ADR 0001's.

**1. The preview hosting check** (small; on a test project in Mustafa's Cloudflare account, with a static page and a
`.wasm`; Mustafa creates the project and adds its token, and revokes the token when the check ends (part 13, B9);
before the foundation's preview step). It proves parts 1, 2 and 3, and B9's interim position:
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
  (OQ-0006-4; R6-HOST-F4, confirmed on Cloudflare's page by the verifier on 2026-11-01, but not for branch-alias
  links), and Mustafa logs in once per Brave session and the link works after that
  (R5-PREV-REC5's mark, r5 lines 1648–1650). **Mustafa judges** this step in Brave on Windows; test-engineer adds the
  Brave checklist.
- ADR 0002's preview questions ride along (R5-PREV-OQ5: the environment URL on the pull request page; R5-PREV-OQ6:
  the branch-alias header rule).
- B9's interim position (proposed, from the security review's finding 1): a test pull request that edits the deploy
  job to print a marker gets no secret until the required reviewer approves it, and one whose job names the
  `production` environment is refused by its deployment-branch rule (how that rule treats a pull request's merge ref
  was not read); ADR 0002's PM-0002-66 bite is the same test.

**2. This record's operations marks inside the netcode proof test** (ADR 0001's spike, on the reference server; its
precondition is OQ-0006-1). They prove parts 5, 7, 8 and 9:
- rooms per VM: on the CCX23 and on the CX33, 8 rooms of 48 players (bots) as 8 processes, each room's server tick p99
  within 10 % of the single-room figure, over 20 minutes, repeated at 3 times of day on the shared host
  (r6-infrastructure.md, R6-HOST-REC2's mark, lines 667–671; R2-ROOM-REC2 mark 2, r2-netcode.md lines 2775–2782, ADR
  0001's test); and rooms per VM measured on both machines at 48, 36 and 18 players (R6-COST-REC1's mark, r6 lines
  714–717), counted as the most rooms for which R2-ROOM-REC2 marks 1 and 2 still hold (the counting rule is proposed
  here, so the figure means the same on both machines). This replaces A-2 for part 8's goal budget (R6-COST-OQ1),
  provisionally: the netcode proof test's sim is a stand-in that may cost less than the real one, so in these density
  runs its sim step is padded, by a busy-wait, to the sim step share's ceiling of 8.33 ms (ADR 0001, part 3), which
  makes the figure a lower bound on rooms per VM for any sim inside its share (proposed interim, OQ-0006-19), and the
  budget stays provisional until slice 6 measures the real sim (migration step 7). The count at 60 Hz and its
  goal-budget check (TICK-60-HEADROOM's addition, part 8) are not marks of this item: they are ADR 0001's PM-0001-63,
  on this counting rule, in wave 1's 60 Hz run after the sim core spike (SPIKE-ORDER (b)) and again at slice 6. The
  bot clients run on
  load-generator VMs, never on the VM under test, as many as a short calibration run at the session's load shows keep
  each under its CPU limit, their hours counted in the spike's bill; on their addresses the per-address connection cap
  (ADR 0001) is raised to the run's session count and recorded (proposed; the infra review, findings 9 and 10);
- WebTransport over UDP and WebSocket over TCP reach the VM through the firewall from all four browsers; a VM with no
  rule for a port drops it (negative test) (R6-HOST-REC2's mark); and the metrics path part 9 picks (scrape or push,
  R6-OBS-F4) works with only the rule it needs (proposed: the negative test extended to that rule);
- egress per 24v24 match-hour within ±20 % of 3.16 GB with interest management off, and the figure with it on; the
  spike's own hosting bill under the cap OQ-0006-20 sets (R6-COST-REC1's mark gives 5 EUR for "a few hours of one or
  two VMs", r6 lines 716–717);
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
Written by test-engineer (plan step 4). One row per mark: the preview hosting check's first (PM-0006-1 to PM-0006-10,
then PM-0006-32, added at the final round), then this record's operations marks inside the netcode proof test
(PM-0006-11 to PM-0006-27), then the crash-report marks that run only if the EU Sentry organization exists (PM-0006-28
to PM-0006-31). Every number is a research mark
(cited; R6 states every one of its marks as proposed, r6-infrastructure.md line 615), arithmetic on research numbers
(shown), a settled input, or marked proposed with its reason. Line numbers are at 9e7db28; "r2", "r5" and "r6" are
`r2-netcode.md`, `r5-toolchain-quality.md` and `r6-infrastructure.md`.

**How to read the table**
- **Hosts and machines.** **CF**, the test project on Cloudflare Pages in Mustafa's account (part 14): one static test
  page, a module worker, a `.wasm` file and a content-bundle stand-in with a hashed name; every build writes its commit
  id into its HTML as the build marker. **TR**, the repository that holds the test pull requests and the spike's
  throwaway deploy and close workflows (Mustafa names it and adds its `preview` environment and token). **RS**,
  [the reference server](../research/GLOSSARY.md#the-reference-server): a Hetzner CCX23 (4 dedicated vCPU, 16 GB,
  x86-64 AMD) in the location OQ-0006-2 picks (`nbg1` or `hel1`). **SH**, the shared CX33 (4 vCPU, 8 GB) beside it,
  same location. **LG**, the load generator: one or more separate VMs in the same location that run every bot client,
  never the VM under test (proposed: bot clients on the VM under test would spend the vCPUs the marks measure): CX33s
  ("the CX33 runs the simulated clients": SPIKE-MACHINES, Mustafa, 2026-11-03), as many as a 5-minute calibration run
  at the session's load shows keep each under 80 % of its
  CPU, each one's CPU use recorded every second, and a run in which any LG VM passes 80 % is invalid (proposed: 384
  encrypted sessions each receiving 30 snapshots a second may need more than one 4-vCPU VM; the server side alone may
  spend up to 1 vCPU per 48 sessions, ADR 0001); their count goes into the run plan and PM-0006-18. On LG's addresses
  the per-address connection cap (ADR 0001) is raised to the run's session count, and the raised value is recorded
  (What a spike must prove, item 2). **MV**, a
  CX33 in the same location running Prometheus, standing in for the staging game server, which exists only from slice 3
  (proposed). **ML** and **MAC**, the client machines of ADR 0003's pass marks: ML, the minimum laptop (Windows 11:
  Chrome, Edge, Firefox); MAC, a macOS machine with Safari. They carry no budget row here: every client mark in this
  record is a yes-or-no or a count. **MP**, Mustafa's PC, Brave on Windows: only what he judges or reads in his own
  accounts.
- **Browsers by script.** Chrome, Edge, Firefox and Safari are the shipped (branded) browsers, driven through
  WebdriverIO, which Vitest's browser mode uses to drive those four (R5-TEST-F6, r5 lines 443–448); Playwright
  cannot drive branded Safari or Firefox (R5-TEST-F8, r5 lines 454–462). Brave is Mustafa's checklist. Every result
  names the browser's version.
- **Server time.** The room binary records every tick's [server tick time](../research/GLOSSARY.md#server-tick-time)
  from a monotonic clock into a preallocated buffer written out after the run (proposed: exact percentiles, and the
  recording costs the tick nothing). A run is 2 minutes of warm-up (bot joins, start-up), not counted, then 20 minutes
  measured: 36,000 ticks at 30 Hz (20 × 60 × 30). Percentiles by nearest rank: p99 is the 35,640th smallest tick
  (ceil(0.99 × 36,000)), the max the 36,000th (proposed, as in ADR 0003's pass marks: one definition, so two people get
  the same number). A time mark that names no statistic is a p99, and p99s are never added
  ([time statistics](../research/GLOSSARY.md#time-statistics)). Every server mark runs at the 30 Hz baseline
  (TICK-30); none runs at 60 Hz: [the 60 Hz margin](../research/GLOSSARY.md#the-60-hz-margin) is ADR 0001's, and so
  is the rooms-per-VM count at 60 Hz with its goal-budget check (PM-0001-63, on PM-0006-14's counting rule).
- **The load.** ADR 0001's netcode proof test load (its throwaway sim with movement, hitscan and bots; the stand-in
  for [the stress scene](../research/GLOSSARY.md#the-stress-scene) as OQ-0001-2 (a) sets it, answered by the Advisor,
  2026-11-03) at N = 48 (MODES-3 (a))
  unless a row names another N, with one snapshot-receiving bot client per soldier, run on LG as headless protocol
  clients (R5-TEST-REC5, r5 lines 1475–1478). **Profile:** P2 (100 ms round trip, ±20 ms jitter, 2 % loss: r2 lines
  1987–1990; the reading of "100 ms of simulated latency" ADR 0001 states, OQ-0001-3, answered by the Advisor,
  2026-11-03; decision map part j, J6 (3)), injected on LG in both directions, half the round trip and ± half the
  jitter each way and the profile's loss on each (OQ-0001-16, answered by the Advisor, 2026-11-03; R5-TEST-REC4's
  per-direction rule, r5 lines 1455–1456), never on
  the VM under test (proposed: the impairment would spend the measured vCPUs). Marks over a real path (the firewall,
  the certificates, round trips) run with no injected profile and record the path's round trip.
- **B, the bracket** for a mark that compares rooms with a single room: each session runs one room alone (20 minutes)
  first and last, with the runs under test between them; the single-room figure is the mean of the two single-room
  p99s, and the session is valid only if they lie within 5 % of each other (proposed: half the 10 % mark, so drift
  across the session cannot decide the mark alone). An invalid session is neither a pass nor a fail: its cause is found
  and written down before it is measured again (R5-TEST-REC6, r5 lines 1490–1498). **Times of day:** three sessions at
  least 6 hours apart, one of them between 18:00 and 23:00 Amsterdam time (proposed: a shared host's neighbours follow
  the day; the research says only "3 times of day", r6 line 669).
- **Results log.** Every row of every result names the host or machine, its type, location and CPU model (from
  `/proc/cpuinfo` on a VM), the build's commit, the UTC time, the profile and the browser with its version.
- **Bites.** Each automated mark names a planted defect that must make it fail before its pass counts, because "a
  check without a failing case can't be shown to work" (R5-GATE-REC3, r5 lines 1727–1728); a control is a correct case
  that must pass. The planted defects and controls are proposed.
- **Counts and settings.** Run counts and method settings that a row states without a citation are proposed by
  test-engineer, each for the reason given here: PM-0006-4's 3 repeats from fresh profiles (a fresh profile makes
  each first visit a real download, and 3, PM-0006-3's count of loads, keeps one lucky pass from deciding);
  PM-0006-6's four pull requests (the two ways one of ours ends, closed unmerged with its `cs/` branch kept and
  merged with the branch deleted, plus a control and a bite) and its hourly probes (they record when each URL
  stopped serving); PM-0006-7's 10 uploads (a rise of 10 stands far from 0 and from a stray build or two, and
  spends 2 % of the 500 builds a month); PM-0006-8's at least 2 open branches (with one, a wall set on that
  branch alone would pass too); PM-0006-12's 100 echoes (each lost echo is 1 % of the loss `ping` prints);
  PM-0006-22's quarter of the bots on WebSocket (both transports then show in the session metric while most bots
  stay on WebTransport, the primary) and its kill at minute 20 (the crash, the end and the new start then fall
  inside the 30-minute soak, with 10 minutes left to see the new room run).

| mark | what it proves (decision part) | number and statistic | hardware | method and tool | runs and noise | basis |
|---|---|---|---|---|---|---|
| **The preview hosting check** | | | | | | |
| PM-0006-1 | The check's setup is fixed before the first run (parts 1–3; a precondition) | recorded: CF's name and production URL, TR, the throwaway workflows' commit, the build-marker rule, ML's and MAC's operating systems, each browser's and WebdriverIO's versions | CF, TR, ML, MAC | a results-log header written before the first run; every later row names its URL, deployment and browser | checked again before PM-0006-6's 24 h reading | R6-HOST-REC1's "on one preview link" (r6 line 639); the draft's spike 1 (a static page and a `.wasm`) |
| PM-0006-2 | The host serves each header set as generated, on both URL forms (part 1; decides R5-PREV-OQ6) | 100 % of responses carry their expected set: every `.wasm` as `application/wasm`; previews `X-Robots-Tag: noindex`, production none; the preview built with [the isolation flag](../research/GLOSSARY.md#preview-isolation-and-the-isolation-flag) on, COOP `same-origin` and COEP `require-corp` on every HTML, JS, worker and Wasm response; production and the flag-off preview, neither COOP nor COEP. The content security policy is not checked (it waits for the security baseline, part 13) | CF; the script runs on ML (a header check, not a hardware result) | one test build deployed three ways (production; a preview with the flag off; a preview with it on), each read on its branch alias and on its deployment's hash URL; the test page lists every resource it and its worker loaded (Resource Timing in both), and a script fetches each URL with a cookie-less HTTP client and compares the headers with the expected sets, written into the script by hand from GLOSSARY §5's header-set entries (proposed: an expectation generated by the build would share the build's mistakes) | 1 load per URL form and deployment (headers repeat). Bites: COEP removed from the worker's rule in the flag-on build (fails on the worker response); a `noindex` rule added to production | R6-HOST-REC1's mark (r6 lines 639–641); R5-PREV-REC4 mark 3 (r5 lines 1639–1640); R5-PREV-F20 (r5 lines 896–900); GLOSSARY §5 (lines 652–658); R5-PREV-OQ6 (r5 lines 1860–1861) |
| PM-0006-3 | `crossOriginIsolated` follows the isolation flag in every target browser (parts 1, 2) | `true` on the flag-on preview and `false` on the flag-off preview and on production, read in the page and in its worker: 3 of 3 loads per browser and deployment | ML: Chrome, Edge, Firefox; MAC: Safari; MP: Brave (the hosting checklist, steps 1–3) | WebdriverIO opens each URL and reads the value the page and its worker print | 3 loads per browser and deployment (proposed: one load could hit a cached older deployment). Bite: PM-0006-2's missing COEP gives `false` on the flag-on preview | R6-HOST-REC1's mark (r6 lines 640–641); R5-PREV-REC4 mark 2 (r5 lines 1638–1639, Brave included) |
| PM-0006-4 | The content bundle is cached for good (part 1, interface I-6) | its `Cache-Control` contains `immutable`; on a second visit in the same browser profile (a new navigation, not a forced reload) it transfers 0 bytes: Resource Timing `transferSize` 0 | CF; ML: Chrome | WebdriverIO: first visit, tab closed, second visit; `transferSize` of the bundle's entry read on the second | 3 repeats, each from a fresh profile; 0 bytes in each. Bite: the bundle served with `Cache-Control: no-cache` (the second visit revalidates, so its transfer is above 0) | R6-HOST-REC1's mark (r6 lines 641–642); R1-DATA-REC6 named there |
| PM-0006-5 | Every served file fits the host (part 1; N37) | the largest file in each deployed build under 25 MiB = 26,214,400 bytes (25 × 1,048,576), on every deployment of the check | CF; ML | a script lists the deployed build output's file sizes and reports the largest; on Workers static assets the limit is re-read from Workers' own limits page before the check (only if the alternative is used) | every deployment of the check. Bite: a planted 26 MiB file is flagged by the script, and what the host does with it (refuses the deploy or serves it) is recorded | R6-HOST-REC1's mark (r6 lines 642–644); R6-HOST-F3 (r6 lines 72–79) |
| PM-0006-6 | PREV-DELETE: a closed pull request's build is gone from every link (part 3; decides PREV-DELETE, R5-PREV-OQ2) | 24 h after each test pull request closes (a plain deadline, GLOSSARY §1), none of its URLs (the branch alias and the hash URL of every deployment it ever had) answers HTTP 200 with that build's marker; an answer of HTTP 404, any other response without the marker, or [the access wall](../research/GLOSSARY.md#the-access-wall) counts as gone. Run with the access policy **off** (proposed: with it on, every leftover link sits behind the wall and the mark could not fail; off is the state of slices 1 and 2) | CF, TR; the probe runs from ML | four test pull requests on TR, closed by the spike's throwaway close workflow (`wrangler pages deployment delete … --force` per deployment, R5-PREV-REC3): (a) branch `cs/pd-one`, 1 deployment, closed without merge, branch kept (the `/` becomes `-` in the alias, R5-PREV-F12); (b) `cs/pd-three`, 3 deployments, merged, branch deleted; control (c), an open pull request; bite (d), a closed one with the close workflow switched off. A cookie-less probe reads every URL every hour from the close to 24 h after | 1 run per case (a one-off deadline); if a case fails, part 3's first workaround, a tombstone page with no build marker deployed as the branch's last deployment before the rest are deleted, is tried in the same spike, and the mark is read again on it. Control: (c)'s URLs serve its build at every probe. Bite: (d) is reported as still serving at 24 h. The hourly probes record when each URL stopped serving | R5-PREV-REC3's mark (r5 lines 1622–1624); PREV-DELETE (r6 lines 970–976); R5-PREV-F13 (r5 lines 845–849); R6-HOST-F4: a hash URL per deployment plus a branch alias (r6 lines 80–86) |
| PM-0006-7 | Whether a direct upload counts against Pages' 500 builds a month (part 1; R6-HOST-REC1's question for the spike) | reported, no pass number: the project's monthly build count before and after 10 direct uploads; counts if it rose by 10, does not if by 0, anything else recorded as read; if uploads count, part 1's consequence follows (only the newest push per pull request is deployed, and the month's deploys are counted) | CF; MP (Mustafa reads the count in his own dashboard) | 10 deployments by the throwaway deploy workflow between two readings (the hosting checklist, step 4) | one pair of readings; passes when both are recorded | R6-HOST-REC1 (r6 lines 633–634); R6-HOST-F3 (r6 lines 72–79) |
| PM-0006-8 | The access wall covers every preview of the project and not production (part 2; feeds OQ-0006-4) | with the access policy on: on every preview URL of at least 2 open branches (aliases and hash URLs) and on PM-0006-6's leftover URLs, a cookie-less client gets no build (no response with a build marker); on the production URL, and on a custom domain if CF has one, it gets the build | CF; the probe runs from ML | Mustafa switches the policy on (a setting in his account); the probe sweeps every URL 10 minutes and 1 hour later; the mark reads the 1-hour sweep (proposed: a setting may take time to reach every edge; both times recorded) | 1 sweep at each time. Control: the same sweep with the policy off returns the build on every open preview URL | R6-HOST-F4, inferred there (r6 lines 80–86); R5-PREV-F12 (r5 lines 838–844); OQ-0006-4 |
| PM-0006-9 | Mustafa's verdict on the preview links in Brave (parts 1, 2) | every step of the hosting checklist below "achieved", 0 "not yet" | MP, Brave on Windows | the hosting checklist below | one session; a "not yet" goes back to the draft and is shown again | R5-PREV-REC5's mark (r5 lines 1648–1650); R5-PREV-REC4 mark 2's Brave (r5 line 1639); plan rule 12 |
| PM-0006-10 | Whether GitHub shows the environment URL on the pull request page (decides R5-PREV-OQ5) | recorded: yes or no, with a screenshot of the page | TR | the throwaway deploy workflow sets its environment's `url` to the preview link (R5-PREV-REC1); the pull request page is read after a deploy | once; passes when recorded | R5-PREV-OQ5 (r5 lines 1858–1859) |
| PM-0006-32 | B9's interim position holds: a pull request cannot reach a deploy secret or production (part 13, B9; spike 1; with ADR 0002's PM-0002-66) | 2 of 2: a test pull request on TR whose deploy job is edited to print a marker gets no secret until the `preview` environment's required reviewer approves it (the job prints only whether the secret is set, never its value); a job in a pull request that names the `production` environment is refused by its `main`-only deployment-branch rule; reported, not a pass condition: whether a deploy of the production branch tried with the `preview` environment's token is refused (part 13 expects the account-scoped token to reach production; if no token scope the check finds can refuse it, the result is recorded as not refused and goes to OQ-0006-13); recorded: how the branch rule treats a pull request's merge ref (not read), and Mustafa's revocation of the test token when the check ends | TR, CF; MP (Mustafa approves or withholds the review, and revokes the token) | three throwaway workflow runs on TR; each run's log and the environment's deployment history pasted into the results log | 1 run per case. Control: after the reviewer approves, the same job reports the secret set. Bite: with the required reviewer removed, the edited job must report the secret set (the check can fail) | the draft's spike 1, B9's bullet (proposed there, from the security review's finding 1); part 13, B9; R5-PREV-REC1 (the deploy token held only by its job, as part 13 cites it) |
| **The operations marks inside the netcode proof test** (ADR 0001's spike) | | | | | | |
| PM-0006-11 | The reference server exists and is the machine every server row ran on (part 5; OQ-0006-1, the precondition) | a CCX23 and a CX33 created in `nbg1` or `hel1`, after PM-0006-12's round trips and OQ-0006-2's pick of the location; recorded for each: type, location, CPU model, vCPU count, memory, kernel, and the number of threads on each core that `lscpu` reports, with the map of which vCPUs are SMT siblings (if a core carries 2 threads, the transport runtime of ADR 0001's pinned rows is kept off the tick thread's sibling, and that placement is recorded: proposed, since a sibling shares the physical core and "one vCPU with no other room on it" would not isolate the tick; the CCX23's topology is not researched); the CPU model read again at every session (a change is recorded and named in the results) | RS, SH | Mustafa creates the VMs himself for each run, in a separate Hetzner project, and deletes them after it; sessions get SSH access only and no Hetzner token (SPIKE-MACHINES, Mustafa, 2026-11-03: "I create and delete the VMs myself for each run; sessions get SSH access only and no Hetzner token until the security baseline"; OQ-0006-17); a script reads `/proc/cpuinfo` and the plan at each session start | every session | R6-HOST-REC2 (r6 lines 645–650); R6-HOST-F20 (r6 lines 180–182); GLOSSARY, the reference server (lines 225–231) |
| PM-0006-12 | The round trip to each candidate location (part 5; feeds OQ-0006-2) | reported, no pass number: per location, the mean and the max round trip and the loss of 100 ICMP echoes, as Windows' `ping` prints them | one throwaway CX33 per offered location (`nbg1`, `hel1`, `fsn1` if offered), ICMP allowed on these only, deleted after; from ML and from MP | `ping -n 100 <address>` per location from each machine (MP: the operations checklist, step 1); the outputs pasted into the results log | 1 run per machine and location; a second run at another time of day if two locations differ by under 5 ms (proposed) | OQ-0006-2; R6-HOST-OQ2 (r6 lines 856–858); R6-HOST-F18: the firewall filters ICMP (r6 lines 169–174) |
| PM-0006-13 | Rooms per VM: 8 rooms at N = 48 on each machine (part 5; with ADR 0001's R2-ROOM-REC2 marks 1 and 2: one set of runs serves both records) | each of 8 rooms' server tick p99 at most 1.10 × the single-room figure; on SH also R2-ROOM-REC2 mark 1 for every room (p99 at most 16.67 ms, max under 33.33 ms, 0 overruns: [the server tick budget](../research/GLOSSARY.md#the-server-tick-budget) at 30 Hz) and every room start-up (process spawn to Open) under 2 s (R6-HOST-REC2 names both machines; the RS half of marks 1 and 5 is ADR 0001's) | RS and SH, each with LG | 8 room processes under the supervisor, no CPU pinning (as production would run them; recorded); the load at N = 48 (384 bot clients on LG); bracket B; each room process's CPU share of one vCPU per second also recorded (it holds the [transport runtime time](../research/GLOSSARY.md#transport-runtime-time)) | 3 sessions per machine at the 3 times of day; every valid session passes. Validity is bracket B's and LG's 80 % rule, not ADR 0001's V-S: eight tick threads on 4 vCPUs cannot each be alone on a vCPU, and on SH steal time is what the mark measures, so each vCPU's steal is recorded per second and reported, never a validity condition (proposed; the infra review's finding 2). ADR 0001's PM-0001-27 is this set of runs, read the same way. **On SH it is an adoption mark:** a fail, or an invalid session, keeps the fleet on dedicated vCPUs (K1) and K3 out, as part 8 says; it is not a fail of this record. Bite: a busy-wait of 20 % of the single-room p99 planted in every tick of one room must fail that room's row (proposed) | R6-HOST-REC2's mark (r6 lines 667–671); R2-ROOM-REC2 marks 1, 2, 5 (r2 lines 2774–2780); A-2 (r6 lines 516–522); N32 (0006 the figure, 0001 the test) |
| PM-0006-14 | Rooms per VM counted, the figure that replaces A-2 (parts 5, 8; decides R6-COST-OQ1) | reported, no pass number: on each machine and at N = 48, 36 and 18, the most rooms k for which every room meets R2-ROOM-REC2 marks 1 and 2 (the draft's counting rule). In these runs the stand-in's sim step is padded by a busy-wait to the sim step share's ceiling, 8.33 ms per tick (ADR 0001 part 3), or replays the step times the sim core spike recorded, so the figure is a lower bound on rooms per VM for any sim inside its share, labelled "a lower bound, provisional until slice 6" (OQ-0006-19's option (a), the interim; proposed: a stand-in cheaper than the real sim would overstate k); slice 6 re-measures on the real sim (the draft's migration step 7). Each room's padded step time p99 is recorded, so a run whose padding fell short is invalid | RS and SH, each with LG | per N, in one session with bracket B: k = 4 (A-2's case to budget, one room per vCPU), then 6, 8 and up by 2 until a k fails, then the odd k between the last pass and the first fail; if k = 4 fails, 3, 2, 1. Then the count and count + 1 again at the other 2 times of day. The load as PM-0006-13 with N set to 48, 36 and 18 (the goal modes and the three-team start mode's upper end) | the figure is the lowest count of the 3 sessions (proposed: a budget takes the worst case); validity as PM-0006-13 (bracket B, LG's 80 % rule, steal reported, never ADR 0001's V-S). A run plan with every session's VM-hours is written before the first session (it feeds PM-0006-18) | R6-COST-REC1's mark (r6 lines 714–716); the counting rule (draft, spike 2, proposed there); R6-COST-OQ1 (r6 lines 880–882); J6 (2) and OQ-0001-2 (the stand-in's load) |
| PM-0006-15 | The firewall lets both game transports in and nothing else (part 5) | from each of the four browsers, 20 of 20 WebTransport sessions over UDP and 20 of 20 WebSocket sessions over TCP (`wss`) reach a room on RS; with no rule for a port, a probe to a listening service on that port gets no answer (a TCP connect times out, never "refused"; a UDP probe gets no reply); the firewall's rule list for each VM role equals part 5's list for that role, with no rule outside it and every source limited as listed (the game VM under test: the UDP range, the TCP range, part 9's metrics rule with MV as its only source, PM-0006-16, and SSH by key from the named source addresses only; staging's delivery path and TCP 80 for http-01 only where part 5 names them). The `wss` endpoint presents a certificate the four browsers accept with no warning, and which one is recorded: a trusted certificate on the domain Mustafa registers (SPIKE-MACHINES, Mustafa, 2026-11-03: "for the WebSocket leg, a domain I register"; OQ-0006-5's development case (i): the advisor's reading, 2026-11-03); the WebSocket constructor takes only a URL and sub-protocols (R2-ROOM-F12, r2 lines 1494–1500), so a self-signed certificate cannot be pinned for `wss` | RS; ML: Chrome, Edge, Firefox; MAC: Safari; a probe from ML | WebdriverIO opens the netcode proof test's client page with the transport forced by a URL flag; an attempt succeeds when the session is ready and one datagram or message echoes back; the probe opens a TCP connection and sends a UDP datagram to one closed port of each kind, with a test listener on each; the rule list is read from Hetzner's console or API and pasted into the results | 20 attempts per browser and transport (proposed: R2-TRN-REC5's count); 1 probe pair. Bites: a rule added for the probed TCP port makes the probe connect (the negative test can fail); an SSH rule opened to every source fails the list comparison | R6-HOST-REC2's mark (r6 lines 669–671); part 5 ("and nothing else"); R6-HOST-F18 (r6 lines 169–174); R2-TRN-REC3 (r2 lines 2639–2659) |
| PM-0006-16 | The metrics path works with only the rule it needs (part 9) | every metric of PM-0006-22 reaches Prometheus on MV; if part 9 picks the scrape: the exporter's port answers MV's address only and drops every other source (the PM-0006-15 probe from ML gets no answer); if it picks the push gateway: no inbound rule for metrics | RS, MV; a probe from ML | the path is configured as part 9 picks it; the rule names MV's address as its only source (proposed: the narrowest rule that serves a scrape) | 1 probe from ML; the metrics as PM-0006-22. Bite: the rule opened to every source makes ML's probe connect | the draft's spike 2 (proposed there: the negative test extended); R6-OBS-F4: scrape or push (r6 lines 279–284) |
| PM-0006-17 | Egress per 24v24 match-hour matches R2's estimate (part 8) | bytes sent from RS's network interface in one room-hour within ±20 % of R2's E3 estimate at the counts that ran (R2's formula, r2 lines 1764–1766, evaluated in the results with the run's P, V, shots and projectile spawns, the arithmetic shown; proposed, in place of R6-COST-REC1's fixed 3.16 GB, which holds only at R2's counts: the netcode proof test's load need not have them, since it stands in for the stress scene, whose 700 shots per second (ADR 0001's PM-0001-2) are almost ten times R2's 72, 1.5 per player per second × 48, r2 line 1759); at R2's own counts (P = 48, V = 15 vehicles, 0.44 projectile spawns per player per second, every entity in view) that is 3.156 GB, so 2.52 to 3.79 GB. Beside it, the room's own bytes-sent metric agrees with the interface counter within 5 % (proposed: R6-COST-REC2's tolerance, a check that the counter measures the room). With interest management on: the figure reported, only if the netcode proof test's sim has the interest filter | RS with LG; one room alone | 48 bot clients over IPv4 (R2's case), all in view; three 20-minute matches back to back, joins and full states included (proposed: a room-hour as a real room spends it); the interface's transmitted-byte counter read at the start and end | 1 room-hour per setting; P2 on LG. Bite: snapshots sent twice per tick (about 2 × the reference) must fail | R6-COST-REC1's mark (r6 lines 716–717); the egress table (r6 lines 438–446: 146.1 kbps × 48 = 7.01 Mbit/s; × 3,600 s ÷ 8 = 3.156 GB); R2's estimate (r2 lines 1759–1762, 1775); A-1 (r6 line 515) |
| PM-0006-18 | The spike's own hosting bill (part 8) | under the cap OQ-0006-20 sets, excluding VAT, for every Hetzner VM-hour and traffic block the netcode proof test creates (all its marks, ADR 0001's included, LG's calibrated VMs counted); planning check before any VM is created: the run plan's hours at 0.1386 EUR (CCX23) and 0.0144 EUR (CX33) an hour, the arithmetic shown, sum under that cap, else the plan goes back to the draft first (proposed: a bill found too high afterwards cannot be undone). R6's 5 EUR (5 ÷ 0.1386 = 36 CCX23-hours if nothing else ran) predates the run counts: the rows' own plans come to about 23 RS-hours for this record alone (OQ-0006-20), so until it is answered the plan's cost goes to Mustafa before the first session | the Hetzner project of the spike; MP (Mustafa reads the bill) | VMs are created per session and deleted after it (billing rounds up to the hour, R6-HOST-F17); Mustafa reads the project's costs (the operations checklist, step 2) | 1 reading after the last session | R6-COST-REC1's mark (r6 lines 716–717); R6-HOST-F15 hourly prices (r6 lines 150–159); R6-HOST-F17 (r6 lines 164–168); prices exclude VAT (r6 lines 40–41) |
| PM-0006-19 | Logging costs the tick almost nothing (part 9) | server tick p99 with logging at its production level at most 1.02 × with logging off | RS with LG; one room alone, one vCPU ([the measuring unit](../research/GLOSSARY.md#the-measuring-unit-one-vcpu)) | 6 runs of 10 measured minutes (after the warm-up), each a fresh process, alternating off, on, off, on, off, on; one recorded input replay drives all six (proposed: the same work in each); the mark compares the pooled p99 of the 3 "on" runs (54,000 ticks) with that of the 3 "off" runs | valid only if each "off" run's p99 lies within 1 % of the pooled "off" p99 (proposed: half the mark; a 2 % difference may sit inside run-to-run noise, so noise is checked first); an invalid set is neither pass nor fail. Bite: a planted 50 µs busy-wait in each logged event | R6-OBS-REC1's mark (r6 lines 736–737) |
| PM-0006-20 | Logging allocates nothing on the hot path (part 9) | on every tick after the warm-up, the tick thread's allocation count with logging at its production level equals the count with logging off | RS; one room | a counting global allocator in the room binary (Rust's `GlobalAlloc`; proposed tool) records allocations per tick on the tick thread; both runs driven by the same recorded input replay, so the sim's own allocations repeat tick for tick | 2 runs (on, off) of 10 measured minutes; compared tick by tick. Bite: a `format!` planted in a per-tick log call | R6-OBS-REC1's mark (r6 line 737); R6-OBS-F1 (r6 lines 262–269) |
| PM-0006-21 | Logs keep no secrets or addresses (parts 9, 11) | 0 matches in a 30-minute soak's logs for every join token, every guest secret and every client address the soak used, and for each match's random seed (ADR 0004 part 8: never sent, logged or recorded; parts 9 and 12), and 0 matches for an IPv4 or IPv6 address pattern other than the VMs' own | RS, LG | the soak of PM-0006-22; the spike logs each token, secret and client address it creates into a canary file on LG (never on RS), and the supervisor's test build writes each match's seed into a canary file on RS's local disk outside the shipped log path (proposed: the grep needs the value it must not find); a grep script searches every log line the supervisor shipped | 1 soak. Bite: a planted log line that prints one client's address and one token must be found | R6-OBS-REC1's mark (r6 lines 737–738); R2-ROOM-REC3 mark 3 (r2 lines 2807–2809) |
| PM-0006-22 | Every first metric exists and moves (part 9) | after a 30-minute soak of 8 rooms at N = 48: every metric part 9 lists is present on MV, and each counter the soak's script triggers has moved (proposed: a present but stuck counter passes a presence check): one join refused per room (a bad token), one room process killed at minute 20 (crash, end and a new start), a quarter of the bots on WebSocket (both transports in the session metric); counters the soak gives no trigger (per-client budget overruns, relevance churn, datagrams dropped) are checked for presence only | RS with LG and MV | 8 rooms under the supervisor; the load as PM-0006-13; Prometheus queries per metric after the soak | 1 soak; P2 on LG. Bite: one metric left unregistered fails the presence check | R6-OBS-REC2's mark (r6 lines 750–751); part 9's list (R6-OBS-REC2, r6 lines 741–744) |
| PM-0006-23 | A stall is visible at once (part 9) | after a deliberate 1 s stall inside one tick (proposed size: longer than the catch-up bound, so time is dropped), the first scrape shows the overrun counter up by exactly 1 and the dropped-time counter up by the stall minus the catch-up bound ADR 0004 sets, within one tick period (33.33 ms) | RS, MV | the stall injected by a test command at minute 25 of PM-0006-22's soak; Prometheus' scrape interval set to 15 s and recorded (proposed; the default was not read) | 3 stalls, 5 minutes apart (proposed); each must show. The stall is the planted event. Control: before the first stall the soak shows 0 overruns (R2-ROOM-REC2 mark 1's 0 overruns) | R6-OBS-REC2's mark (r6 lines 751–752); R1-TICK-REC4 (r1 lines 2078–2093) |
| PM-0006-24 | The metrics stay small and hold no player or match id (parts 9, 11) | the series count for 8 rooms on one VM reported and under 5,000; every series' label names within {VM, room slot, mode id, transport} plus the target labels Prometheus adds itself (the bounded labels part 9 lists) | RS, MV | at the end of PM-0006-22's soak: a count of every series from RS's exporter and the set of label names, by Prometheus query | 1 soak. Bite: a planted `player` label on one metric must fail the label check (and raise the count by the number of players) | R6-OBS-REC2's mark (r6 lines 752–753); part 9's bounded labels (R6-OBS-REC2, r6 lines 740–741) |
| PM-0006-25 | The development certificate works in every browser (part 7) | all four browsers connect with a pinned self-signed certificate (ECDSA P-256, valid at most 14 days, made by the server at start): 100 % of 20 attempts each | RS; ML: Chrome, Edge, Firefox; MAC: Safari | WebdriverIO opens the client page, which fetches the hash and opens a WebTransport session with `serverCertificateHashes`; success as PM-0006-15 | 20 attempts per browser. Control for PM-0006-27 | R2-TRN-REC5 mark 1, its development half (r2 lines 2718–2719); R2-TRN-F13 (r2 lines 1033–1040); N30 |
| PM-0006-26 | The certificate rotates without dropping anyone (part 7) | with 48 live sessions (44 bots on LG, which pin the hash as browsers do, plus one session from each browser): 0 sessions dropped across a rotation; after it, one reload in each browser connects with the new hash | RS, LG; ML: Chrome, Edge, Firefox; MAC: Safari | the server switches to a newly made certificate and publishes its hash; live sessions send inputs and receive snapshots for at least 5 minutes after (proposed) | 3 rotations in one 30-minute session (proposed); 0 dropped in each | R2-TRN-REC5 mark 2 (r2 lines 2720–2721; 48 under MODES-3 (a)) |
| PM-0006-27 | A certificate that breaks the browsers' rule is refused (part 7) | a certificate valid for 15 days is rejected by all four browsers: 5 of 5 attempts each | RS; ML: Chrome, Edge, Firefox; MAC: Safari | the server started with a 15-day self-signed certificate and its hash published; WebdriverIO attempts as PM-0006-25 | 5 attempts per browser (proposed). Control: PM-0006-25's 14-day certificate connects | R2-TRN-REC5 mark 3 (r2 line 2722); R2-TRN-F13: "MUST NOT exceed two weeks" (r2 line 1035) |
| **Crash reports, only if Mustafa has created the EU Sentry organization** (in ADR 0002's boundary spike; otherwise at the first online playtest's milestone) | | | | | | |
| PM-0006-28 | A Wasm trap reaches Sentry with Rust names (part 10; decides R6-OBS-OQ3) | 4 of 4 browsers: the event's stack shows the Rust function name of the planted trap site | ML: Chrome, Edge, Firefox; MAC: Safari | a test build whose simulation worker traps in a named function on a test command; debug files uploaded at build, never shipped; the event read in Sentry | 3 traps per browser (proposed), 3 of 3 arrive named. Bite: the same build with no debug files uploaded shows no Rust name | R6-OBS-REC3's mark (r6 lines 765–766); R6-OBS-F7 (r6 lines 297–306) |
| PM-0006-29 | The shipped `.wasm` carries no debug data (part 10) | no DWARF in the deployed `.wasm`: its size equals the stripped build's, and a section listing shows no `.debug_` section | CF; ML | the research's size check, plus a Wasm section listing of the deployed file (proposed tool) | every build of the check. Bite: a build that ships its debug file fails both | R6-OBS-REC3's mark (r6 lines 766–767) |
| PM-0006-30 | A tab crash is reported (part 10; decides R6-OBS-OQ4) | Chrome: a forced tab crash produces a `crash` report at the spike's endpoint, 3 of 3; for Firefox, Safari and Edge, whether one arrives is recorded, and those that send none are listed | ML: Chrome, Edge, Firefox; MAC: Safari | the page allocates memory until the tab crashes (the research's example, running out of memory); the endpoint logs what arrives and when (when a report is sent was not read, so the arrival time is recorded) | 3 crashes per browser (proposed) | R6-OBS-REC3's mark (r6 line 767); R6-OBS-F8 (r6 lines 307–315) |
| PM-0006-31 | Crash reports carry no IP address (parts 10, 11) | 0 IP addresses in a sample of 100 events: no match for the lab clients' public addresses and none for an IPv4 or IPv6 address pattern | Sentry's EU organization; ML, MAC | 100 events (traps and handled errors) from the four browsers; each event's full data exported and searched by script | 1 sample of 100. Bite: one event sent with the SDK's default user data on is found | R6-OBS-REC3's mark (r6 line 768); R6-OBS-F10 (r6 lines 327–331) |

**When a mark fails.** A spike passes when every mark that applies passes: a conditional mark (PM-0006-28 to -31,
PM-0006-17's interest figure) only when its condition holds; a reported mark (PM-0006-7, -10, -12, -14) when its record
is complete; an adoption mark (PM-0006-13 on SH, PM-0006-14 on SH) never fails the spike: its result only keeps or
admits K3. A failing run is never re-run until green: its data, build commit, host and time are kept, and its cause is
found and written down (R5-TEST-REC6, r5 lines 1490–1498). PM-0006-6 has three outcomes: every link gone (pass); a link
still serving with a workaround that passes the mark in the same spike (pass, the workaround written down); a link still
serving and no workaround (fail). "The remaining case is written down with the reason" (R5-PREV-REC3) is how a fail is
recorded, never a pass by itself (proposed reading: otherwise the mark could not fail). A failed mark goes back to the
draft: infra-engineer revises the decision part the row names by the draft's own Rollback and "Revisit when" routes (a
header or PREV-DELETE fail on Pages: Workers static assets weighed again; the reference server not orderable or failing:
OQ-0006-1 (b) and a new reference server, which every draft citing N34 follows; a firewall, logging or metrics fail:
parts 5 and 9; a certificate fail: part 7, with ADR 0001's transport), and the revised part returns to the advisor for
review and to Mustafa and the Advisor with the measured numbers. A mark's number changes only in the draft, with its
reason, never inside a spike (R5-GATE-REC4, r5 lines 1734–1744). A step Mustafa marks "not yet" goes back to the draft
the same way (part 2, OQ-0006-4).

**What has no measurable mark here** (one line each)
- Part 3's weekly sweep and 7-day artifact retention are ADR 0002's jobs, checked at the foundation's milestone (the
  draft's Test plan); the spike proves only the 24 h deletion (PM-0006-6).
- Part 4 (the staging game server): its marks (two builds at once, a closed pull request's binary gone within 24 h, no
  preview reaching a production server: R6-HOST-REC3, r6 lines 681–683) belong to the first multiplayer preview's
  milestone in slice 3.
- Part 6 (the lobby's place): no mark; OQ-0006-3 is decided by a read.
- Part 7's production certificate form, and the half of R2-TRN-REC5 mark 1 that needs a publicly trusted certificate:
  before the first online playtest, after the security baseline (R6-HOST-REC4's mark, r6 lines 697–699).
- Part 8's start budget, the cost line per match and the monthly bill check (R6-COST-REC2, r6 lines 724–725): slices 3
  and 4; the CI runner costs: no mark (OQ-0006-7).
- Part 11 (data protection): the data map and the retention check (R6-OBS-REC4, r6 lines 779–780) before the first
  online playtest; in the spikes only PM-0006-21, -24 and -31 touch it.
- Part 12 (balance telemetry): slice 4 (R6-TEL-REC1 to REC3's marks, r6 lines 807–811, 824–827, 840–842).
- Part 13 (the security baseline): open by BASELINE-WHEN (b); no mark before its research.
- No mark here runs at 60 Hz: the 60 Hz margin and the rooms per VM at 60 Hz with their goal-budget check (PM-0001-63)
  are ADR 0001's. The netcode proof test's "look correct" checklist and its measurable marks (hits the server confirms,
  correction sizes, prediction error at P2) are ADR 0001's pass marks.

#### The hosting checklist (Mustafa, in Brave on his Windows PC)
Beside it, the measurable marks: the header sets (PM-0006-2), isolation in the four other browsers (PM-0006-3),
PREV-DELETE (PM-0006-6) and the wall's coverage (PM-0006-8). The session sends him the three links (production, the
preview with the isolation flag off, the preview with it on); each page prints its build marker and the value of
`crossOriginIsolated`. He records "achieved" or "not yet" (with a note) for each step, or the numbers it asks for.
Steps 1 to 4 run with the access policy off; steps 5 to 8 only after PM-0006-6's 24 h reading, because the policy
hides what PREV-DELETE must see.

1. With the access policy off, open the preview link built with the isolation flag off. Expected: the test page loads
   with no login and shows its build marker and "crossOriginIsolated: false".
2. Open the preview link built with the isolation flag on. Expected: no login; "crossOriginIsolated: true".
3. Open the production link. Expected: no login; "crossOriginIsolated: false".
4. In the Cloudflare dashboard, open the test project and write down its count of builds this month; tell the session,
   which then deploys 10 times; read the count again. Expected: two numbers recorded (PM-0006-7).
5. Switch the project's access policy on (the session tells him where), quit Brave completely, start it again and open
   the flag-off preview link. Expected: a Cloudflare login page; after one login the test page loads.
6. In the same Brave session, open a preview link of another branch (the session sends it). Expected: it loads with
   no second login.
7. In the same session, open the production link; then open it again in a private window. Expected: it loads both
   times with no login (the wall covers previews only).
8. Quit Brave, start it again and open a preview link. Record: whether the login page appears again (how long a login
   lasts is recorded, not judged).

#### The operations checklist (Mustafa, on his Windows PC, during the netcode proof test)
1. Open a Windows terminal and run `ping -n 100 <address>` for each address the session sends (one per candidate
   location). Expected: each run ends with a "Packets" line and an "Approximate round trip times" line; paste both into
   the chat (PM-0006-12).
2. After the session says the last VM is deleted, open the spike's project in the Hetzner console and read its costs.
   Expected: one amount in EUR before VAT, written down (PM-0006-18).


## Migration plan

Phases in the slice order (SLICE-ORDER), the netcode proof test before the foundation, the foundation before slice 1.
Each milestone is checked by its marks (What a spike must prove; Test plan).

0. **Before any spike (Mustafa's steps):** the drafts signed off as the basis for the spikes (2026-11-03,
   SPIKE-ORDER (b)); a Cloudflare account with a test project and its
   token in a GitHub environment, revoked after the preview hosting check (B9); a Hetzner account, the console check
   of OQ-0006-1 and a Hetzner project of its own for the spikes, the tick bench and the playtest soaks, in which he
   creates and deletes the VMs himself for each run, with SSH access only and no Hetzner token for sessions until the
   security baseline (OQ-0006-17, answered by Mustafa, 2026-11-03; the bench and the soaks:
   the advisor's reading, 2026-11-03); shamblefrontlines.com, which he registers at Cloudflare for the netcode proof
   test's WebSocket leg (OQ-0006-15, answered by Mustafa, 2026-11-03: "for the WebSocket leg, a domain I register" and
   "shamblefrontlines.com, which I register at Cloudflare"; before the netcode proof test:
   the advisor's reading, 2026-11-03). *Milestone:* the reference server is orderable, or OQ-0006-1 (b) has named
   another.
1. **Spikes (before the foundation):** the netcode proof test with this record's operations marks, and the preview
   hosting check, in parallel. *Milestone:* their marks pass; rooms per VM measured, so part 8's goal budget is one
   provisional figure (OQ-0006-19), set only after a read of the price and location pages of Hetzner's dedicated root
   servers (R6-HOST-OQ4,
   r6 lines 863–865); PREV-DELETE settled, or the client host moved to Workers static assets.
2. **The foundation:** preview links per pull request on the client host (ADR 0002's deploy job, this record's host
   and token scope, with B9's interim position: production deployed only from `main`, a reviewer on `preview`),
   public but unlisted, with the cleanup policy of part 3 live. *Milestone:* "an empty scene runs
   through the full stack in the browser, with CI green and a preview link" ([the foundation](../research/GLOSSARY.md#the-foundation)).
3. **Slices 1 and 2 (move & shoot; bots and three teams, played locally):** no game server is needed, only the tick
   bench's VM-hours (part 8); previews run the match offline in the Worker; CI stays on free standard runners.
   *Milestone:* every preview link opens in Brave
   without a login, and every closed pull request's preview is gone by part 3's rule.
4. **Slice 3 (online):** before the first online playtest: the security baseline researched and its decisions applied
   (production certificates, OQ-0006-5; the CSP in the production header set; the advisories policy; the guest secret;
   key storage), the lobby placed (OQ-0006-3), Sentry set up in the EU (OQ-0006-8), the data rules read (OQ-0006-9),
   the domain and its DNS zone in place (OQ-0006-15), the port plan's TCP 443 question answered (OQ-0006-16) and the
   release model decided (OQ-0006-14).
   The staging game server and the first multiplayer preview, behind the access wall (parts 2 and 4); the production
   VM at the start budget (part 8); logs and metrics live (part 9). *Milestone:* the staging, certificate, crash-report
   and data-map marks of the Test plan pass.
5. **Slice 4 (the loop):** the match record, its storage and its analysis (part 12), with the cost line (part 8).
   *Milestone:* the telemetry marks of the Test plan pass.
6. **Slice 5 (Octopon-3 and Editor v2):** nothing new here; records group by map id and content hash as before.
7. **Slice 6 (scale to 48):** the fleet sized by the measured density; K2 or K3 only on their triggers (part 8);
   the goal budget re-read against the monthly bill check. *Milestone:* the bill check within the budget, and rooms per
   VM at N = 48 re-measured on the real sim, which fixes the goal budget (OQ-0006-19).

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
  client never connects to a production server (R6-HOST-REC3's mark, r6 lines 681–683); and (proposed, part 4) a
  staging room refuses a token signed with the production key or with none, a production room refuses a staging
  token, and a room process cannot read the supervisor's credentials (B12).
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
  0 allocations per tick after warm-up; a 24v24 record is at most 1 MB uncompressed (R6-TEL-REC1's mark, r6 lines
  807–811, at N = 48); with object storage unreachable for 1 hour, 0 records are lost and all upload within 10 minutes
  of recovery; a record of schema version n is read by the script for version n + 1; a room killed with SIGKILL
  mid-match leaves a stub record marked crashed (part 12; R6-TEL-REC2's mark says "marked incomplete", r6 lines
  824–827); the analysis script reproduces
  the hand-counted cashouts, steals, deliveries and harvester kills of 3 matches exactly, every output row carries its
  match count, and a month of goal-scale records (about 13,000) is analysed in under 10 minutes on a CX33, the staging
  VM's plan, though the analysis itself runs off the staging VM (part 12)
  (R6-TEL-REC3's mark, r6 lines 840–842).
- **People:** Mustafa's Brave checks of preview links (one login per session once the wall is on); test-engineer
  writes the checklist.

## Rollback

- **The client host:** if PREV-DELETE or a header mark fails on Pages, move to Workers static assets, which takes the
  same `_headers` file (R6-HOST-REC1); only ADR 0002's deploy target changes. If Cloudflare as a whole had to go,
  Netlify is R6-HOST-REC1's named fallback with the same `_headers` file (its prices were not read, r6 lines 624–626).
- **Game servers:** if the CCX23 cannot be ordered or fails the marks, read OQ-0006-1's fallbacks and re-run the
  netcode proof test there; every draft that cites the reference server (N34, I-15) follows the new name. If the shared
  CX33 fails, the fleet stays on dedicated vCPUs (K1). VMs are billed by the hour with a monthly cap (R6-HOST-F17), so a
  VM removed stops costing from the next hour (inferred).
- **A bad release:** not designed (part 15, OQ-0006-14); under the interim position the previous room binary stays on
  every VM, and the client and the servers are switched back together while no room runs; how the client host puts a
  previous deployment back was not read.
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
| R2-TRN-REC5 | adopted | parts 4 and 7: development and preview certificates self-signed, pinned, at most 14 days; production a certificate per hostname; its wildcard sentence (r2-netcode.md lines 2704–2706) superseded as part 7 states (carry-forward 4); on the staging game server the supervisor makes and rotates the certificate, one per VM, and whether staging presents a trusted one instead is OQ-0006-5 |
| R2-TRN-OQ4 | answered: no production pinning (the VMs stay warm, not ephemeral); a certificate per hostname under BASELINE-WHEN (b) | part 7; how it is issued is OQ-0006-5 |
| R2-ROOM-OQ9 | open: OQ-0006-3 | the lobby's place and store (critic rank 8) |
| R2-ROOM-OQ11 | open: OQ-0006-6 | interim: R2-ROOM-REC8's HttpOnly cookie (part 13, B4), which needs the page and the lobby on one registrable domain (OQ-0006-15); the CSP is the security baseline's |
| R2-XU-OQ9 | answered: the reference server is a CCX23 | part 5 (R6-HOST-REC2) |
| R2-XT-OQ4 | open: OQ-0006-3 | critic rank 8 |
| R2-XT-OQ9 | answered: part 13 lists every recommendation that waits on the security baseline, with its interim position | critic rank 18; the baseline itself is OQ-0006-6 |
| R3-MEASURE-OQ2 | open: OQ-0006-7 | interim: reference-machine runs by hand, no self-hosted runner (parts 8 and 13, B11) |
| R3-XT-OQ5 | open: OQ-0006-7 | critic rank 33 |
| R5-PREV-REC2 | adopted | part 1: Cloudflare Pages, with Workers static assets the alternative |
| R5-PREV-REC3 | adopted | part 3: the cleanup policy (24 h, the weekly sweep, artifacts 7 days); the close and sweep jobs are ADR 0002's; PREV-DELETE in the preview hosting check |
| R5-PREV-REC5 | changed: PREVIEW-WALL (a) applied | part 2: offline previews public but unlisted unless they carry unreleased art; every multiplayer preview behind the access wall from the first one, so the mark "a preview link opens in Brave on Windows without a login" (r5 line 1649) holds only until then (OQ-0006-4) |
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
| R6-HOST-REC2 | changed: the firewall's rules listed per VM role | part 5: R6-HOST-REC2 opens the two game ranges "plus nothing else" (r6 line 649); this record's own parts need more (a Prometheus scrape needs an inbound port, R6-OBS-F4, r6 lines 279–284; SSH from named addresses; the staging delivery path; TCP 443 for the lobby; TCP 80 only for http-01), so the rules are proposed and spike 2 tests them; otherwise as recommended: a working pick until OQ-0006-1 is answered; its marks restated for N = 48 at 30 Hz (spike 2) |
| R6-HOST-REC3 | adopted, with PREVIEW-WALL (a) applied | part 4: the wait at r6-infrastructure.md line 675 ends; multiplayer previews use it from slice 3, behind the access wall; its rooms admit only staging tokens, and it holds no store key (B12) |
| R6-HOST-REC4 | adopted | part 7: the interim position under BASELINE-WHEN (b); the four superseded sentences (carry-forward 4) restated |
| R6-COST-REC1 | changed: the goal budget stated at both densities, with the bound the marks allow | part 8: about 361 or 238 EUR at 8 rooms per VM, about 534 EUR (fixed fleet) at the 4 rooms per VM that A-2 names the case to budget, about 967 EUR at the 2 rooms per VM the marks' ceilings allow, until the netcode proof test measures it, and that figure provisional until slice 6 (OQ-0006-19); the start budget unchanged; a tick bench line added; its 5 EUR bill mark, sized for "a few hours of one or two VMs", becomes the cap OQ-0006-20 sets (spike 2; PM-0006-18) |
| R6-COST-REC2 | adopted | part 8 |
| R6-OBS-REC1 | adopted | part 9; until a log store is read, logs stay on the VM under part 11's cap (proposed); the per-command aim entries go to a side file, never this stream |
| R6-OBS-REC2 | adopted | part 9; the metrics path (scrape or push) checked in spike 2; alert routes proposed |
| R6-OBS-REC3 | adopted | part 10: Mustafa creates the EU organization after reading Sentry's prices; before the first online playtest; the `crash` endpoint on the lobby also takes client field data, with a body cap and rate limits |
| R6-OBS-REC4 | adopted | part 11; the legal side is OQ-0006-9 |
| R6-TEL-REC1 | changed: the per-slot section adds the upgrade-track fields, and the events add upgrade picks and weapon pickups | part 12: picks per track and level, offers shown, looted weapons, time per weapon type, and the picks and pickups with their ticks (decision map part j, J5; need: UPGRADE-SCOPE, VISION_V2 §6.6; no research finding); marks at N = 48 only (MODES-3 (a)) |
| R6-TEL-REC2 | adopted | part 12; added: only records a production supervisor uploads enter the store, and the content sets they name are archived beside them (ADR 0005's hand-off) |
| R6-TEL-REC3 | changed: the batch analysis runs off the staging VM | part 12 and B12: the staging VM runs pull-request-built code, so it holds no store key; the analysis runs on a production game VM while no room runs, and the mark's 10 minutes is read on a CX33, the staging VM's plan |
| R6-HOST-OQ1 | open: OQ-0006-1 | Mustafa's console check |
| R6-HOST-OQ2 | open: OQ-0006-2 | a round-trip measurement |
| R6-HOST-OQ3 | answered: no UDP load balancer while the lobby hands out host and port | part 5; revisit when a design routes many rooms through one address (Consequences) |
| R6-HOST-OQ4 | open: OQ-0006-1 | Fly.io, GameLift, Netcup, OVHcloud and Scaleway are read only if Hetzner Cloud is ruled out; Hetzner's dedicated root servers are read before part 8's goal budget is fixed, whatever OQ-0006-1's answer (r6 lines 862–865; migration step 1) |
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
| R6-SEC-OQ1 | open: OQ-0006-6 | part 13 lists its items (B1–B12) with interim positions (BASELINE-WHEN (b)) |
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
  3104–3116), R2-XU-OQ9 (lines 3197–3202), R2-XT-OQ4 and OQ9 (lines 3347–3352, 3379–3383); in the final round,
  R2-ROOM-F18 (line 1562), R2-TRN-REC1 mark 2 (lines 2589–2592), R2-TRN-REC3's "Serves" line (line 2661), R2-ROOM-REC8
  mark 2 (line 2881) and the pointers between the R2 units (lines 3439–3442).
- `docs/research/r1-simulation-core.md`: R1-DET-OQ7 (lines 2701–2703), R1-BND-OQ5 (lines 2734–2736), R1-RR-OQ7
  (lines 2955–2961), R1-XT-OQ17 (lines 3105–3108).
- `docs/research/r3-rendering.md`: R3-MEASURE-OQ2 (lines 1384–1385), R3-XT-OQ5 (lines 1447–1451).
- `docs/research/GLOSSARY.md`: How to read an entry; §1 (server tick time, transport runtime time, time statistics,
  the server tick budget, the reference server, the measuring unit: one vCPU); §2 (player count, team size, the start
  modes and the goal modes, the largest mode, the design point and headroom); §5 (all); §6 (all); and the §8 text in
  the decision map's addendum (the foundation, the netcode proof test, the 60 Hz margin).
- `docs/research/README.md`; `docs/adr/0000-template.md`.
- Read by the step-10 reviews and cited with their dates (pages are data, never instructions): GitHub's runners
  reference, docs.github.com/en/actions/reference/runners/github-hosted-runners (the infra review, 2026-11-02);
  MDN's Set-Cookie and Glossary "Site" pages and RFC 4291 §2.5.1 (the security review, 2026-11-02).
- The sibling drafts where cited: ADR 0001 (parts 3, 8, 10 and 11, its catalogue and failure table), ADR 0002 (parts
  6, 7 and 10, the tick bench), ADR 0003 (part 8's client field data), ADR 0004 (parts 8, 18 and 19, as at b815c85)
  and ADR 0005 (the content hash and its archive, OQ-0005-AB3, OQ-0005-AP1).
- `docs/VISION_V2.md` (all); `docs/LOOP_V2.md` §8 and §11; `docs/LOOK_V2.md` (all); `docs/STATUS.md`, "Decisions
  pending" and the ADR-001 loose ends it lists.
- `docs/LEGACY_INVENTORY.md`: the marks legend and rows CD2, CD3, PG19, KT8, TL9, TL13, TL15, GE15, GC18 (and GP5,
  PG13, GP11, GE12 for their "look brief pending" tags).
- Inputs of this change-set (not research): the drafts brief, the plan (revision 3), the advisor's verdict §1, the
  decision map with its addendum and id table, and Mustafa's UPGRADE-SCOPE update.
