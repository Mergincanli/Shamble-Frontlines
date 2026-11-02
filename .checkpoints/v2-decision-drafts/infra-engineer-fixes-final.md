# infra-engineer, final fix round (plan step 10's fix loop): ADR 0006, all but "### Pass marks"

Task: prompts/final-infra-engineer.txt (2026-11-02). Every BEFORE below was copied by script (whole lines, by line
range) from docs/adr/0006-infrastructure-operations.md at 47eadd9 (identical to the branch head for this file) and
checked to occur exactly once there; no two BEFOREs share a line, none touches "### Pass marks" (lines 797-964), and
applying the blocks top to bottom or in the order written gives the same file. Several findings that touch one place
share one block, named by all their ids. The coordinator's routed item of 2026-11-02 (GD-6's 0006 half and SA-15:
ADR 0004 at b815c85, parts 18 and 19, emits picks and pickups with their ticks; part 8 never stores, sends or logs the
seed) is in the part 9 and part 12 blocks. Line counts grow (new part 15, B12, OQ-0006-13 to -19); no sentence sits
directly under a table without a blank line.

New open questions: OQ-0006-13 (the preview token and production: the Advisor with security-auditor, Mustafa for an
account; SA-1, SA Q1), OQ-0006-14 (releases: the Advisor for the model, Mustafa for the window; IE-1, EA-5, EA Q2,
RELEASE-WINDOW), OQ-0006-15 (the domain: Mustafa after the Advisor's OQ-0006-5 read; IE-6, SA-3, DOMAIN), OQ-0006-16
(the port plan: infra-engineer with netcode-architect, the Advisor confirms; IE-7), OQ-0006-17 (VMs for spikes and
the bench: Mustafa with the Advisor; IE-14, SPIKE-VM-ACCESS), OQ-0006-18 (unreleased art in a public repository:
Mustafa; IE-12, UNRELEASED-ART), OQ-0006-19 (the goal budget provisional until slice 6: Mustafa on the Advisor's
advice; TE-2, TE Q2).

Sources new to the draft (read by reviewers, cited with their dates as the draft already does for the verifier's
reads): GitHub's runners reference (docs.github.com/en/actions/reference/runners/github-hosted-runners, the infra
review, 2026-11-02: macOS among the standard runners on a public repository); MDN Set-Cookie and Glossary "Site", and
RFC 4291 §2.5.1 (the security review, 2026-11-02). Everything else cites research lines at 9e7db28, checked in this
round: r5 770-774, 795-803, 823-830, 913-914, 1645, 673-678, 1809, 1835-1837; r6 164-168, 482-483, 516-522,
568-569, 840-842, 873-874; r2 1562, 2589-2592, 2661, 2700-2703, 2720-2721, 2881, 3439-3442.

### FIX TE-2, IE-1 (Decision intro: the goal budget provisional; the release model open; lines 118-121 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
VM. Log, measure and report crashes with small, EU-only tools that keep no personal data. Write one balance record per
match inside the room, store it as JSON lines in EU object storage, and analyse it in batches per content hash. List
the security baseline as open, with interim positions, until its own research before the first online playtest. Every
account, payment, secret and repository setting below is Mustafa's to create.
AFTER:
VM, a figure that stays provisional until slice 6 measures the real sim (OQ-0006-19). Log, measure and report crashes
with small, EU-only tools that keep no personal data. Write one balance record per match inside the room, store it as
JSON lines in EU object storage, and analyse it in batches per content hash. List the security baseline as open, with
interim positions, until its own research before the first online playtest; the release model is open too
(OQ-0006-14). Every account, payment, secret and repository setting below is Mustafa's to create.
END

### FIX IE-18 (part 1: what follows if direct uploads count against the 500 builds; lines 135-135 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
   row PG19); the content security policy waits for the security baseline (part 13).
AFTER:
   row PG19); the content security policy waits for the security baseline (part 13). Pages' Free plan allows 500 builds a
   month (R6-HOST-F3, r6 lines 72–79), and whether a direct upload counts against them is a spike item (part 14). If
   uploads count, only the newest push of each pull request deploys (ADR 0002's concurrency cancel) and the month's
   deploys are counted; a paid plan is Mustafa's after a price read (proposed, not researched).
END

### FIX IE-12 (part 2: the wall hides a preview, not files committed to a public repository; lines 156-156 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
   states it, read by the verifier on 2026-11-01), so the game itself stays public.
AFTER:
   states it, read by the verifier on 2026-11-01), so the game itself stays public. The wall hides a preview, not the
   files behind it: the repository is public (r5-toolchain-quality.md, Findings, Question 6, legacy facts, lines
   913–914), so a model file committed to it can be read by anyone once it is pushed, as R5-PREV-REC5 itself reasons:
   "A preview contains only what the public repository already contains (inferred)" (r5 line 1645). Where model files
   live is ADR 0005's OQ-0005-AP1; whether unreleased art must stay unseen is OQ-0006-18.
END

### FIX IE-20 (part 3: the first PREV-DELETE workaround the check tries; lines 166-166 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
   the preview hosting check (part 14); if Pages fails it with no workaround, Workers static assets is weighed again
AFTER:
   the preview hosting check (part 14). The first workaround the check tries is a tombstone: a page with no build is
   deployed as the branch's last deployment, then every other deployment is deleted (proposed, not researched); if
   Pages fails PREV-DELETE with no workaround, Workers static assets is weighed again
END

### FIX SA-2, IE-4, SA-4 (part 4: who may join a staging room, its certificate hash, what else runs on it; lines 173-179 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
   supervisor starts the room binary of the build a preview names; it presents development certificates (part 7);
   production servers refuse preview origins, and a preview never connects to a production server
   (r6-infrastructure.md, Recommendations §1, R6-HOST-REC3, lines 672–683). PREVIEW-WALL (a) ends the wait in "Until
   Mustafa answers PREVIEW-WALL (below), previews do not use the staging server" (r6 line 675): from the first multiplayer
   preview (slice 3, online), previews use it, behind the access wall (part 2). How a preview's binary and certificate
   hash reach it is ADR 0002's workflow and ADR 0001's hash endpoint (critic rank 20). At the start the same VM also
   runs Prometheus (part 9) and the batch analysis (part 12). *Why:* a VM per pull request bills at least an hour each
AFTER:
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
END

### FIX IE-5 (part 5: the firewall's rules per VM role; lines 185-190 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
   port; each VM has its own public IPv4 and IPv6 address. The firewall opens one UDP range for WebTransport and one
   TCP range for the WebSocket fallback (R2-TRN-REC3, as R6-HOST-REC2 says, r6 lines 648–649), plus the one rule part 9's metrics path may need
   (proposed: a change to R6-HOST-REC2's "plus nothing else", r6 line 649, because a Prometheus scrape needs an inbound
   port while a push needs none, R6-OBS-F4, r6 lines 279–284; spike 2 checks it), and nothing
   else; Hetzner's firewall is stateful, drops all inbound traffic no rule allows and costs nothing
   (r6-infrastructure.md, Findings §1, R6-HOST-F18, lines 169–174). VMs stay up and rooms start and end as processes
AFTER:
   port; each VM has its own public IPv4 and IPv6 address. Hetzner's firewall is stateful, drops all inbound traffic no
   rule allows and costs nothing (r6-infrastructure.md, Findings §1, R6-HOST-F18, lines 169–174). Its rules are listed
   per VM role (proposed: R6-HOST-REC2 opens the two game ranges "plus nothing else", r6 lines 648–649, but this
   record's own parts need more), and each role opens only its list:
   - *every game VM, production and staging:* one UDP range for WebTransport and one TCP range for the WebSocket
     fallback (R2-TRN-REC3, as R6-HOST-REC2 says), their ports set by the port plan (OQ-0006-16); the one rule part 9's
     metrics path may need, with the Prometheus host as its only source (a scrape needs an inbound port while a push
     needs none, R6-OBS-F4, r6 lines 279–284; spike 2 checks it); and SSH, by key only, from named source addresses only
     (proposed: without an admin path no one can install a room binary or read the logs part 9 keeps on the VM; who
     holds the key is OQ-0006-17);
   - *the staging game server, also:* the path ADR 0002's delivery job hands the room binary through (its form not
     researched);
   - *the lobby's host (OQ-0006-3):* TCP 443 for its HTTPS endpoint;
   - *ACME:* TCP 80 on each game VM, only if OQ-0006-5 picks http-01;
   - *nothing else:* the supervisor opens its link to the lobby from the VM (part 6), so that link needs no inbound rule.
   VMs stay up and rooms start and end as processes
END

### FIX IE-11, SA-3, EA-10 (part 6: the report endpoint, one registrable domain, the supervisor's link to the lobby; lines 208-220 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
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
AFTER:
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
   the overrides (ADR 0001's supervisor calls); the supervisor answers with the host and a port from the firewall's
   range (part 5); it reports Closed, an abnormal exit and draining. The link is authenticated (how: the security
   baseline, item B5), and its message format is read with OQ-0006-3, beside ADR 0001's catalogue; routing joins by
   build is OQ-0006-14's. *Ids:* R2-ROOM-OQ9, R2-XT-OQ4, R5-XT-OQ3, R6-XT-OQ1.
   *Need:* VISION_V2 §6.1 (rooms and private matches go through it).
END

### FIX IE-6, IE-4, IE-15 (part 7: the domain, the staging supervisor's certificate, where alerts go; lines 227-235 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
   a secret) and how long they live is OQ-0006-5 (R6-HOST-F24, r6 lines 198–204; R6-HOST-OQ6, r6 lines 869–874). Development
   and preview servers use self-signed ECDSA P-256 certificates valid at most 14 days, made by the server at start and
   pinned through `serverCertificateHashes` (R2-TRN-REC5, r2 lines 2700–2703; number N30). Hash pinning in production
   stays out: the VMs stay warm, so they are not the "short-lived ephemeral servers" R2-TRN-REC5 names
   (r6-infrastructure.md, Options §1, Q3, option C3, lines 426–427; r2-netcode.md, R2-TRN-OQ4, lines 3050–3054). The
   netcode proof test runs on the development certificates for WebTransport (which certificate its WebSocket fallback
   presents is OQ-0006-5); the production form is needed from the first online
   playtest, after the security baseline's research. An alert fires when any VM's certificate has under 14 days left
   (proposed in R6-HOST-REC4's mark, r6 lines 697–699).
AFTER:
   a secret) and how long they live is OQ-0006-5 (R6-HOST-F24, r6 lines 198–204; R6-HOST-OQ6, r6 lines 869–874). Every
   per-hostname certificate needs a domain and a DNS zone, and no draft names one yet (OQ-0006-15; the registrar, the
   DNS provider and their prices are not researched). Development
   and preview servers use self-signed ECDSA P-256 certificates valid at most 14 days, made by the server at start and
   pinned through `serverCertificateHashes` (R2-TRN-REC5, r2 lines 2700–2703; number N30); on the staging game server
   its supervisor makes and rotates them, one per VM (part 4). Hash pinning in production
   stays out: the VMs stay warm, so they are not the "short-lived ephemeral servers" R2-TRN-REC5 names
   (r6-infrastructure.md, Options §1, Q3, option C3, lines 426–427; r2-netcode.md, R2-TRN-OQ4, lines 3050–3054). The
   netcode proof test runs on the development certificates for WebTransport (which certificate its WebSocket fallback
   presents is OQ-0006-5); the production form is needed from the first online
   playtest, after the security baseline's research. An alert fires when any VM's certificate has under 14 days left
   (proposed in R6-HOST-REC4's mark, r6 lines 697–699); where alerts go is part 9.
END

### FIX IE-8, TE-2 (part 8, goal: the density the marks' ceilings allow; the measured figure provisional; lines 265-266 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
     Sentry. The goal budget is stated at both densities until the netcode proof test measures rooms per VM
     (R6-COST-OQ1, r6 lines 880–882; R2-ROOM-REC2 mark 2 is ADR 0001's test). Egress stays inside the included traffic at
AFTER:
     Sentry. Neither density is a bound: at the marks' own ceilings a room may take 0.5 vCPU for the server tick (p99
     16.67 of 33.33 ms, the server tick budget, if every tick ran at it) plus up to 1 vCPU for the transport runtime
     (R2-TRN-REC1 mark 2, r2-netcode.md lines 2589–2592), 1.5 vCPU in all, so floor(4 ÷ 1.5) = 2 rooms fit one VM and
     the fixed fleet is ceil(20 ÷ 2) + 1 = 11 CCX23, 11 × 86.49 = 951.39 EUR, about 967 EUR with the staging VM and the
     match records (951.39 + 8.99 + 6.49 = 966.87 EUR) + 19–24 USD + Sentry: the most the marks allow, not a forecast.
     The goal budget is stated at both densities until the netcode proof test measures rooms per VM, and that figure
     stays provisional until slice 6 measures the real sim (OQ-0006-19; R6-COST-OQ1, r6 lines 880–882; R2-ROOM-REC2
     mark 2 is ADR 0001's test). Egress stays inside the included traffic at
END

### FIX IE-3, IE-14 (part 8: macOS runners are standard and free on a public repository; the tick bench's line; lines 275-280 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
   - *CI runners (critic rank 33):* CI runs on GitHub's standard hosted runners, "free and unlimited on public
     repositories", with the free 10 GB cache (r5-toolchain-quality.md, Findings, Question 5, R5-CI-F17, F5, lines
     673–678 and 583–592; the repository is public, lines 913–914). No paid runner (GPU or macOS) and no self-hosted
     reference machine is proposed: their prices were not read (R5-CI-OQ5, R5-TEST-OQ4, r5 lines 1835–1837 and 1809), and
     self-hosted runners are "a cost and security question" (r3-rendering.md, Open questions, R3-MEASURE-OQ2, lines
     1384–1385); reference-machine runs stay by hand until Mustafa decides (OQ-0006-7).
AFTER:
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
   - *The tick bench (proposed):* ADR 0002's tick bench runs on one vCPU of the reference server from slice 1, when no
     game server is budgeted. A CCX23 is created per bench session and deleted after it, billed by the hour (R6-HOST-F17,
     r6 lines 164–168) at 0.1386 EUR (R6-HOST-F15), so 20 bench hours a month, a placeholder, cost 20 × 0.1386 =
     2.77 EUR; who creates and deletes it is OQ-0006-17.
END

### FIX SA-5, SA-15 (part 9: the per-command aim entries go to a side file, never the log stream; no line holds the seed; lines 287-288 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
   room ids and ships them; no line holds a token, a guest secret or a raw id (salted hashes where an id is needed, as
   R2-ROOM-REC3 mark 3 says, r2 lines 2807–2809) (r6-infrastructure.md, Recommendations §3, R6-OBS-REC1, lines 729–738). Metrics use the
AFTER:
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
END

### FIX IE-15 (part 9: where alerts go; lines 299-299 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
   R6-OBS-F4, lines 279–284); the netcode proof test picks the path and checks the firewall rule it needs (part 14).
AFTER:
   R6-OBS-F4, lines 279–284); the netcode proof test picks the path and checks the firewall rule it needs (part 14).
   Alerts go to Mustafa by email (proposed: a certificate under 14 days left, any room crash, sustained overruns, a VM
   disk above 80 % while logs stay on the VMs, a record waiting over 1 hour to upload); the tool (Prometheus'
   Alertmanager or Grafana Cloud's alerting) is not researched.
END

### FIX EA-10, SA-14 (part 10: one report endpoint on the lobby for crash reports and field data; its limits; lines 313-314 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
    so by default the query parameters in the URLs it sends are the channel left. Tab crashes: Reporting API
    `crash` reports to our own endpoint on the lobby. On the server: panics go to Sentry's Rust SDK from the room
AFTER:
    so by default the query parameters in the URLs it sends are the channel left. Tab crashes: Reporting API
    `crash` reports to our own report endpoint on the lobby, which also takes ADR 0003's client field data (once per
    match). It is a public write endpoint, so each body is capped and requests are rate-limited per address prefix
    (IPv4 /32, IPv6 /64, since an IPv6 interface id is 64 bits: RFC 4291 §2.5.1, read by the security review on
    2026-11-02) and globally, the values as data, and no address is kept (proposed; the security review, finding 14).
    On the server: panics go to Sentry's Rust SDK from the room
END

### FIX EA-10 (part 10: where client field data is stored; lines 321-322 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
    milestone. The gate v2's smoke test switches reporting off or allows its endpoints (row TL9; ADR 0002, rank 34). What
    client field data is measured is ADR 0003's (rank 26); it is stored under part 11's rules. *Why:* Wasm frames need
AFTER:
    milestone. The gate v2's smoke test switches reporting off or allows its endpoints (row TL9; ADR 0002, rank 34). What
    client field data is measured is ADR 0003's (rank 26), which leaves its store and retention to this record: the
    endpoint writes both kinds of report as JSON lines to Hetzner Object Storage in the game servers' location, beside
    the match records, under part 11's caps (proposed). *Why:* Wasm frames need
END

### FIX SA-5, SA-14, EA-10, EA-6 (part 11: retention of the aim side files, our endpoint's reports and the content archive; lines 328-331 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
    guest secret goes into logs, reports or match records; retention is capped: logs 14 days and metrics 15 days
    (proposed: the free tier's and Prometheus' defaults), match records kept while their content hash is still analysed
    (R6-TEL-OQ4's proposed default, r6 lines 917–919); the crash reports' cap stays open, because no page read shows
    whether Sentry lets an organization set its retention (r6-infrastructure.md, Recommendations §3, R6-OBS-REC4, lines
AFTER:
    guest secret goes into logs, reports or match records; retention is capped: logs 14 days and metrics 15 days
    (proposed: the free tier's and Prometheus' defaults), match records kept while their content hash is still analysed
    (R6-TEL-OQ4's proposed default, r6 lines 917–919), the aim side files 14 days (part 9), the crash reports and field
    data at our own endpoint 14 days (proposed, as logs; a summary of the field data per build is kept like the match
    records), and each archived content set as long as a record names its hash (part 12); Sentry's crash-report cap
    stays open, because no page read shows whether Sentry lets an organization set its retention (r6-infrastructure.md,
    Recommendations §3, R6-OBS-REC4, lines
END

### FIX GD-6, SA-15 (part 12: upgrade picks and pickups as tick-stamped events; the seed never in the record; lines 347-350 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
    events that made them; never account or guest ids, display names, IP addresses or tokens (r6-infrastructure.md,
    Recommendations §4, R6-TEL-REC1, lines 784–811). **Changed:** the per-slot section also carries the upgrade-track
    fields the decision map gives this record (part j, J5): picks per track and level, offers shown, looted weapons,
    time per weapon type. No research finding covers them (UPGRADE-SCOPE has none), so they are proposed, their shape
AFTER:
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
END

### FIX SA-16, EA-6, SA-4 (part 12: which records enter the store; the content archive; the analysis off the staging VM; lines 354-358 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
    thresholds are a design question (R6-TEL-OQ3) (seam S6). Records are stored as compressed JSON-lines files in
    Hetzner Object Storage in the game servers' location, uploaded by the supervisor after the room exits, with a
    schema version in every record and a Parquet compaction job only once a query needs it; a crashed room leaves a stub
    record marked incomplete (r6-infrastructure.md, Recommendations §4, R6-TEL-REC2, lines 812–827). They are analysed
    in batches on the staging VM, always grouped by content hash and mode id, with the sample size beside every rate;
AFTER:
    thresholds are a design question (R6-TEL-OQ3) (seam S6). Records are stored as compressed JSON-lines files in
    Hetzner Object Storage in the game servers' location, uploaded by the supervisor after the room exits, with a
    schema version in every record and a Parquet compaction job only once a query needs it; a crashed room leaves a stub
    record marked incomplete (r6-infrastructure.md, Recommendations §4, R6-TEL-REC2, lines 812–827). Only records a
    production supervisor uploads enter the store: a local match's record (the room code as the local authority in the
    Worker, ADR 0002 part 6) and the staging game server's records never do (proposed: the first is player-controlled
    data, the second comes from pull-request-built code; the security review, finding 16). Every content set a record
    names is archived in the same bucket by its content hash, uploaded by the supervisor beside the first record that
    names it and kept as long as any record does, so every record's hash resolves (proposed; ADR 0005 hands the archive
    to this record, interface I-6); each set is at most 25 MiB (part 1). The records are analysed in batches on a
    production game VM while no room runs on it, as its own OS user with a read-only key to the store (key scoping not
    researched, B12), never on the staging VM (part 4), always grouped by content hash and mode id, with the sample size
    beside every rate;
END

### FIX SA-10 (part 13, B1: the npm lockfile's advisories; lines 377-377 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
    | B1 | Dependency audits (R5-GATE-OQ4; R1-RR-OQ7; R2-TRN-REC1's version floors) | whether an advisory blocks a pull request, and who owns that policy | [the advisories check](../research/GLOSSARY.md#the-advisories-check) runs on a schedule and on every pull request that changes `Cargo.lock`, outside the PR gate (R5-GATE-REC1, r5-toolchain-quality.md lines 1671–1674); it keeps the version floors current | 0002 (the job); 0006 (the policy, open) |
AFTER:
    | B1 | Dependency audits (R5-GATE-OQ4; R1-RR-OQ7; R2-TRN-REC1's version floors) | whether an advisory blocks a pull request, and who owns that policy; and the npm lockfile's advisories (three.js, which every player runs, plus Vite and Playwright), which no topic researched (the security review, finding 10) | [the advisories check](../research/GLOSSARY.md#the-advisories-check) runs on a schedule and on every pull request that changes `Cargo.lock`, outside the PR gate (R5-GATE-REC1, r5-toolchain-quality.md lines 1671–1674); it keeps the version floors current; the npm lockfile has no advisories check yet | 0002 (the job); 0006 (the policy, open) |
END

### FIX SA-11, SA-3, SA-2 (part 13, B3 (a CSP-ready build), B4 (one registrable domain), B5 (a separate staging key); lines 379-381 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
    | B3 | Content security policy (R5-PREV-REC4; R2-ROOM-OQ11) | the CSP in the production header set | none written yet; no third-party script: three.js is bundled and same-origin (part 1); the header sets take the CSP once written | 0006 (the content); 0002 (the generation) |
    | B4 | Where the guest secret lives (R2-ROOM-OQ11) | HttpOnly cookie or local storage | R2-ROOM-REC8's HttpOnly, Secure, SameSite=Strict cookie on the lobby's origin, which the security review's SEC-GUEST recommends; Mustafa and the Advisor decide (r2-netcode.md lines 3112–3116) | 0001 |
    | B5 | The lobby's token-signing keys (R2-ROOM-OQ2, R2-ROOM-OQ9) | how and where they are stored | only in the lobby host's environment, added by Mustafa | 0001 (scheme); 0006 (storage) |
AFTER:
    | B3 | Content security policy (R5-PREV-REC4; R2-ROOM-OQ11) | the CSP in the production header set | none written yet; no third-party script: three.js is bundled and same-origin (part 1); the header sets take the CSP once written. So that a strict policy stays possible, the build is CSP-ready (proposed, not researched; the security review, finding 11): no inline script or style in the generated HTML, the Worker and the Wasm loaded as files (never `blob:` or `data:` URLs), and ESLint's `no-eval` and `no-new-func` in ADR 0002's lint set | 0006 (the content); 0002 (the generation, the build rule and the lint) |
    | B4 | Where the guest secret lives (R2-ROOM-OQ11) | HttpOnly cookie or local storage | R2-ROOM-REC8's HttpOnly, Secure, SameSite=Strict cookie on the lobby's origin, which the security review's SEC-GUEST recommends; Mustafa and the Advisor decide (r2-netcode.md lines 3112–3116). SameSite=Strict sends the cookie "only for requests originating from the same site that set the cookie", and a site is the registrable domain (MDN's Set-Cookie and "Site" pages, read by the security review on 2026-11-02), so the page and the lobby share one registrable domain: a custom domain on the client host with the lobby under it, and multiplayer previews the same (OQ-0006-15; whether preview links can take a custom domain was not read); otherwise a reload loses the guest id (R2-ROOM-REC8 mark 2, r2 line 2881) | 0001; 0006 (the domain) |
    | B5 | The lobby's token-signing keys (R2-ROOM-OQ2, R2-ROOM-OQ9) | how and where they are stored | only in the lobby host's environment, added by Mustafa; staging signs with a key of its own, so a production room refuses every staging token (part 4; ADR 0001's key id) | 0001 (scheme); 0006 (storage) |
END

### FIX SA-1, IE-14, SA-4 (part 13, B9 (production deploys, the reviewer, the Hetzner token), B10, B12 (new); lines 385-387 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
    | B9 | The tokens CI holds (the client host's deploy token; the Sentry upload token, "a token: Mustafa's secret", r6 line 1082) and a multiplayer preview's binary delivery (R6-XT-OQ4) | their scope and handling | each held only by the job that needs it, in a GitHub environment, as R5-PREV-REC1 does for the deploy token (proposed for the others); added by Mustafa (below) | 0002 (the jobs); 0006 (the scope) |
    | B10 | The staging VM runs pull-request-built native code (R6-XT-OQ4, "PR-built native code on a VM") beside the Prometheus store (part 9) | whether they share a VM | they share it: fork pull requests get no preview (R5-PREV-REC1), so only this repository's pull requests run there | 0006 |
    | B11 | Self-hosted CI runners (R3-MEASURE-OQ2) | whether any is allowed | none (part 8) | 0006 |
AFTER:
    | B9 | The tokens CI holds (the client host's deploy tokens, for previews and for production; the Sentry upload token, "a token: Mustafa's secret", r6 line 1082), a multiplayer preview's binary delivery (R6-XT-OQ4), and the Hetzner API token the spikes and the tick bench need (OQ-0006-17) | their scope and handling | each held only by the job that needs it, in a GitHub environment, as R5-PREV-REC1 does for the deploy token (proposed for the others); production deployed only from `main`, in its own environment with its own token; a required reviewer on `preview`; the preview hosting check's token revoked when the check ends; the Hetzner token in a project of its own; all added by Mustafa (below; OQ-0006-13) | 0002 (the jobs); 0006 (the scope) |
    | B10 | The staging VM runs pull-request-built native code (R6-XT-OQ4, "PR-built native code on a VM") beside the Prometheus store (part 9) | whether they share a VM | Prometheus shares it, since it holds no secret (if M2 later pushes to Grafana Cloud, its key moves Prometheus off this VM); the batch analysis and every store key stay off it (parts 4 and 12). Fork pull requests get no preview (R5-PREV-REC1), but any branch of this repository can change its own workflow (B9), so that code is treated as untrusted | 0006 |
    | B11 | Self-hosted CI runners (R3-MEASURE-OQ2) | whether any is allowed | none (part 8) | 0006 |
    | B12 | Privilege separation on the VMs: room processes parse hostile network input, and the staging VM runs pull-request-built native code (the security review, finding 4) | how processes and keys are separated, and how far store keys can be scoped | each room process runs as its own unprivileged OS user that cannot read the supervisor's, the lobby's (OQ-0006-3 (a)) or the analysis job's credentials; the staging VM holds no secret beyond its rooms' certificate key (part 4); store keys scoped per bucket, put-only for uploads and read-only for the analysis, where the provider allows (not researched); a dns-01 key, if OQ-0006-5 picks dns-01, on no VM that runs rooms (proposed: a job elsewhere hands each VM its own certificate); while Prometheus runs on the staging VM, the metrics port is the one port of a production VM that pull-request-built code can reach (part 5) | 0006 |
END

### FIX SA-1, IE-14 (part 13, Secrets: why the environment alone does not hold, and the interim position; lines 395-395 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
    0002's job). *Ids:* BASELINE-WHEN, R6-SEC-OQ1, R1-RR-OQ7, R5-GATE-OQ4, R5-TSLINT-OQ5, R2-ROOM-OQ11, R2-XT-OQ9,
AFTER:
    0002's job). That environment alone does not keep the token from a branch: a `pull_request` run takes its workflow
    from the pull request's merge commit, unlike `pull_request_target`, which "runs in the context of the default branch
    of the base repository, rather than in the context of the merge commit" (R5-PREV-F2, r5 lines 770–774), so any
    branch of this repository (sessions push them) can rewrite the deploy job and use the token, and an
    account-scoped token can then deploy the production player build too (the security review's inference, finding 1).
    **Interim position until the security baseline** (proposed, from that review): (a) production is deployed only by
    its own job on pushes to `main`, in its own environment `production`, limited to `main` by a deployment-branch rule
    ("Deployment branches and tags are available for all public repositories", R5-PREV-F6, r5 lines 795–803), with its
    own token; (b) `preview` has a required reviewer ("a job cannot access environment secrets until one of the
    required reviewers approves it", the same finding), so each preview deploy waits for Mustafa's approval, unless
    OQ-0006-13 puts production where the preview token cannot reach it; (c) the preview hosting check's token is revoked
    when the check ends (part 14); (d) the Hetzner API token for the spikes and the tick bench belongs to a Hetzner
    project of its own, never the production project (a project's token presumably reaches all its VMs: not read;
    OQ-0006-17). Whether a session, which posts on GitHub as Mustafa's account (CLAUDE.md), could approve a pending
    deploy, and how a deployment-branch rule treats a pull request's merge ref, were not read: the preview hosting
    check tests the second (What a spike must prove, item 1) and OQ-0006-13 carries the first. *Ids:* BASELINE-WHEN,
    R6-SEC-OQ1, R1-RR-OQ7, R5-GATE-OQ4, R5-TSLINT-OQ5, R2-ROOM-OQ11, R2-XT-OQ9,
END

### FIX SA-1, IE-1 (part 14 (the spike's token revoked) and a new part 15, releases; lines 398-402 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
14. **Its spike and its operations marks.** The preview hosting check (small, on a test project in Mustafa's
    Cloudflare account, before the foundation's preview step) proves part 1 and PREV-DELETE; the netcode proof test
    (ADR 0001's) carries this record's operations marks: rooms per VM on both machines, egress per match-hour, the
    spike's own bill, logging cost, the metrics and their path, and the firewall. Both are listed under "What a spike
    must prove". *Need:* as parts 1, 3, 5, 8 and 9.
AFTER:
14. **Its spike and its operations marks.** The preview hosting check (small, on a test project in Mustafa's
    Cloudflare account, before the foundation's preview step) proves part 1, PREV-DELETE and B9's interim position, and
    its token is revoked when it ends; the netcode proof test
    (ADR 0001's) carries this record's operations marks: rooms per VM on both machines, egress per match-hour, the
    spike's own bill, logging cost, the metrics and their path, and the firewall. Both are listed under "What a spike
    must prove". *Need:* as parts 1, 3, 5, 8 and 9.
15. **Releases (open: OQ-0006-14).** How a new client build, content set or room binary reaches production, how two
    versions meet while rooms run, and how a bad server release is rolled back are not designed, and no topic
    researched them. As things stand, every room still on the old binary refuses a new client (ADR 0001's `JoinRejected`
    on a version or content mismatch), and the client's reload fetches the same new bundle, so it cannot recover.
    OQ-0006-14 decides the model before the first online playtest. Interim position (proposed): a release is made only
    while no room runs, with the client and every VM's room binary switched together, and the previous room binary stays
    on each VM, so a bad server release is switched back the same way. The production deploy job is ADR 0002's (part
    13, B9); routing joins by build is ADR 0001's protocol (part 6). *Need:* VISION_V2 §6.1 (rooms and private matches
    stay reachable across a release).
END

### FIX IE-7 (Options Q5: tls-alpn-01 named for the port plan; lines 458-458 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
| tls-alpn-01 (RFC 8737: "This connection MUST use TCP port 443.") | not researched beyond that quote (r6 lines 871–874) | not researched | not researched | not researched | not researched | not researched | Named for OQ-0006-5's port plan |
AFTER:
| tls-alpn-01 (RFC 8737: "This connection MUST use TCP port 443.") | not researched beyond that quote (r6 lines 871–874) | not researched | not researched | not researched | not researched | not researched | Named for OQ-0006-5 and the port plan (OQ-0006-16) |
END

### FIX SA-4 (Options Q10, N1: where the analysis runs; lines 507-507 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
| **N1. Batch analysis per content hash and mode id (proposed)** | Low | 0 (staging VM) | A month of goal records in under 10 minutes (R6-TEL-REC3's mark) | Records hold no player ids | None | From zero | **Pros:** comparable within one tuning version |
AFTER:
| **N1. Batch analysis per content hash and mode id (proposed)** | Low | 0 (a production game VM while no room runs, never the staging VM: part 12) | A month of goal records in under 10 minutes (R6-TEL-REC3's mark) | Records hold no player ids | None | From zero | **Pros:** comparable within one tuning version |
END

### FIX IE-3 (Options Q13: standard macOS runners are free on a public repository; GPU stays unpriced; lines 522-523 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
| **Standard GitHub-hosted runners and the free cache (proposed)** | Low | "free and unlimited on public repositories"; cache free to 10 GB, more needs a Pro, Team or Enterprise account (R5-CI-F17, F5, r5 lines 673–678 and 583–592) | ubuntu-24.04: 4 CPUs, 16 GB, no GPU (F17) | GitHub's isolation; today's hygiene carries over (row KT8) | WebGPU not covered in CI (R5-CI-OQ5) | Today's CI already runs on them (row KT8) | **Pros:** 0 cost. **Cons:** no GPU, no real Safari |
| Larger paid runners (GPU; macOS for real Safari) | Low | Price not read (R5-CI-OQ5, r5 lines 1835–1837; R5-TEST-OQ4, r5 line 1809) | GPU or macOS | GitHub's | WebGPU or Safari covered | Small | **Cons:** unpriced; Mustafa's call |
AFTER:
| **Standard GitHub-hosted runners and the free cache (proposed)** | Low | "free and unlimited on public repositories"; cache free to 10 GB, more needs a Pro, Team or Enterprise account (R5-CI-F17, F5, r5 lines 673–678 and 583–592) | ubuntu-24.04: 4 CPUs, 16 GB, no GPU (F17) | GitHub's isolation; today's hygiene carries over (row KT8) | WebGPU not covered in CI (R5-CI-OQ5) | Today's CI already runs on them (row KT8) | **Pros:** 0 cost. **Cons:** no GPU; real Safari only on the macOS runners of the next row |
| Standard macOS runners for real Safari (`macos-latest`; part of the proposed row, their use is ADR 0002's) | Low | 0 on a public repository: the runners page lists macOS among the standard runners, "free and unlimited on public repositories" (read by the infra review on 2026-11-02; R5-CI-F17 quoted the ubuntu rows only) | 3 CPUs (M1) or 4 (Intel), 7 or 14 GB (same page) | GitHub's | Safari covered | Small | **Pros:** 0 cost while the repository is public. **Cons:** wall time against ADR 0002's budget not known (R5-TEST-OQ4's time half, r5 line 1809) |
| Larger paid runners (GPU) | Low | Price not read (R5-CI-OQ5, r5 lines 1835–1837) | GPU | GitHub's | WebGPU covered | Small | **Cons:** unpriced; Mustafa's call |
END

### FIX IE-8, TE-2 (Trade-off: the bound at both ceilings; the measurement provisional; lines 550-550 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
  two rooms fill a vCPU before any [transport runtime time](../research/GLOSSARY.md#transport-runtime-time). The netcode proof test's measurement replaces both figures.
AFTER:
  two rooms fill a vCPU before any [transport runtime time](../research/GLOSSARY.md#transport-runtime-time). With the
  transport runtime's own ceiling of 1 vCPU per room (R2-TRN-REC1 mark 2), 2 rooms per VM and about 967 EUR are the most
  the marks allow (part 8). The netcode proof test's measurement replaces these figures, provisionally until slice 6
  measures the real sim (OQ-0006-19).
END

### FIX SA-2, SA-1 (Trade-off: what the access wall covers, and the staging VM's public ports; lines 565-565 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
  no player reaches a game server before then except through a multiplayer preview behind the access wall (part 2).
AFTER:
  no player can join a game room before then except through a multiplayer preview behind the access wall, because
  staging rooms admit only tokens from an endpoint behind that wall, signed with a staging key production refuses
  (part 4). The staging VM's ports themselves are public, so its rooms parse hostile input as production's will (B12),
  and until OQ-0006-13 is answered every preview deploy waits for Mustafa's approval (B9).
END

### FIX IE-6 (What breaks if a provider is down: the DNS provider, once a domain exists; lines 569-569 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
  817–818); Sentry or Grafana down, reports and metrics are lost for that time and the game runs on.
AFTER:
  817–818); Sentry or Grafana down, reports and metrics are lost for that time and the game runs on; the DNS provider
  down (once OQ-0006-15 gives the game a domain), names stop resolving as cached records expire, so new joins fail
  while running rooms go on.
END

### FIX IE-6, SA-1, IE-1, TE-2 (Consequences: harder (a domain, the approvals, releases) and revisit (the provisional budget); lines 579-582 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
  Brave session; each production game VM needs a certificate issuance path (OQ-0006-5); the lobby still has no place
  in the budget (OQ-0006-3).
- **Revisit when:** Hetzner's EU plans are not orderable (OQ-0006-1); PREV-DELETE fails on Pages (move to Workers
  static assets); the netcode proof test measures rooms per VM (fix the goal budget); the shared CX33 passes (K3) or
AFTER:
  Brave session; each production game VM needs a certificate issuance path (OQ-0006-5) and a domain (OQ-0006-15); the
  lobby still has no place in the budget (OQ-0006-3); until OQ-0006-13 is answered, every preview deploy waits for his
  approval (B9); releases wait on OQ-0006-14.
- **Revisit when:** Hetzner's EU plans are not orderable (OQ-0006-1); PREV-DELETE fails on Pages (move to Workers
  static assets); the netcode proof test measures rooms per VM (a provisional goal budget) and slice 6 re-measures it
  on the real sim (the goal budget fixed, OQ-0006-19); the shared CX33 passes (K3) or
END

### FIX IE-17 (OQ-0006-2: the location is picked before the reference server is created; lines 628-628 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
  - *Decider:* a measurement (the netcode proof test's setup or a small check before it), confirmed by Mustafa.
AFTER:
  - *Decider:* a measurement made before the reference server is created (a small check at the start of the netcode
    proof test's setup), confirmed by Mustafa; the reference server is then created in the location it picks.
END

### FIX SA-4, EA-10 (OQ-0006-3: field data, and the signing key's place under option (a); lines 633-634 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
  - *Why it matters:* join tokens, guest identity, room codes and crash reports go through it, and no budget holds it
    yet (R6-XT-OQ1, r6 lines 991–997).
AFTER:
  - *Why it matters:* join tokens, guest identity, room codes, crash reports and client field data go through it, and
    no budget holds it yet (R6-XT-OQ1, r6 lines 991–997). Under (a) its private signing key sits on VMs whose room
    processes parse hostile network input, so the asymmetric token's gain (rooms hold only the public key, ADR 0001)
    rests on part 13's item B12; the endpoint that mints staging tokens behind the access wall is read with it (part 4).
END

### FIX IE-4, IE-6, IE-7 (OQ-0006-5: the domain, the development and staging options, the port plan; lines 665-668 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
    869–874). For the WebSocket fallback on development and staging servers: not researched (R2-TRN-REC5's
    self-signed, pinned certificates, r2 lines 2700–2703, serve WebTransport only).
  - *Decider:* security-auditor in the security baseline, with the port plan of ADR 0001's transport; the development
    and staging case before the netcode proof test starts (the Advisor, from a short read).
AFTER:
    869–874); each needs a domain and a DNS zone (OQ-0006-15). For the WebSocket fallback on development and staging
    servers (not researched; R2-TRN-REC5's self-signed, pinned certificates, r2 lines 2700–2703, serve WebTransport
    only): (i) a publicly trusted certificate for a real hostname, which needs the domain (OQ-0006-15); (ii) a private
    test certificate authority installed on the test machines, which needs no domain, but whose key can sign for any
    name, so the security baseline rules on its handling. For the staging game server, also: whether it presents that
    trusted certificate on WebTransport too, so multiplayer previews carry no hash and no rotation (part 4); that would
    depart from R2-TRN-REC5's development and preview certificates and from GLOSSARY §5's definition of the staging
    game server, which would follow.
  - *Decider:* security-auditor in the security baseline, with the port plan (OQ-0006-16); the development and
    staging case before the netcode proof test starts (the Advisor, from a short read).
END

### FIX IE-3 (OQ-0006-7: macOS is not a paid runner on a public repository; lines 684-688 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
- **OQ-0006-7. Will any CI runner be paid for: GPU, macOS, self-hosted reference machines, or a cache above 10 GB?**
  - *Why it matters:* WebGPU coverage in CI, real-Safari runs and reference-machine runs depend on it (critic rank
    33).
  - *Options:* (a) standard runners only, reference-machine runs by hand (part 8's interim position); (b) larger paid
    runners, price not read (R5-CI-OQ5, r5 lines 1835–1837; R5-TEST-OQ4, r5 line 1809); (c) self-hosted reference machines,
AFTER:
- **OQ-0006-7. Will any CI runner be paid for: GPU, self-hosted reference machines, or a cache above 10 GB?**
  - *Why it matters:* WebGPU coverage in CI and reference-machine runs depend on it (critic rank 33). Real-Safari runs
    do not: standard macOS runners are free on a public repository (part 8) and cost only runner time.
  - *Options:* (a) standard runners only, macOS included, reference-machine runs by hand (part 8's interim position);
    (b) larger paid runners for GPU, price not read (R5-CI-OQ5, r5 lines 1835–1837); (c) self-hosted reference machines,
END

### FIX SA-4 (OQ-0006-6: the table now runs to B12; lines 676-676 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
  - *Options:* per item, as part 13's table lists them (B1–B11); among them, whether an advisory blocks a pull request
AFTER:
  - *Options:* per item, as part 13's table lists them (B1–B12); among them, whether an advisory blocks a pull request
END

### FIX SA-5 (OQ-0006-9: the aim side files and our own endpoint's reports; lines 704-707 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
  - *Why it matters:* logs, crash reports and match records are stored from slice 3 on; whether an IP address or a
    guest id is personal data, the lawful basis, the privacy notice and the retention limits were not researched.
  - *Options:* the proposed defaults (logs 14 days, metrics 15 days, records kept while their content hash is
    analysed), or what the law requires.
AFTER:
  - *Why it matters:* logs, crash reports, client field data, match records and the per-command aim side files (part
    9: pseudonymous behaviour data per player) are stored from slice 3 on; whether an IP address or a guest id is
    personal data, the lawful basis, the privacy notice and the retention limits were not researched.
  - *Options:* the proposed defaults (logs, the aim side files and our own endpoint's reports 14 days, metrics 15 days,
    records and archived content sets kept while their content hash is analysed), or what the law requires.
END

### FIX SA-1, SA-Q1, IE-1, EA-5, EA-Q2, IE-6, SA-3, IE-7, IE-14, IE-12, TE-2 (Open questions: OQ-0006-13 to OQ-0006-19, added after OQ-0006-12; lines 734-734 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
  - *Carries:* R1-BND-OQ5, R1-XT-OQ17, R6-XT-OQ9.
AFTER:
  - *Carries:* R1-BND-OQ5, R1-XT-OQ17, R6-XT-OQ9.
- **OQ-0006-13. How is the production player build kept out of the preview deploy token's reach?**
  - *Why it matters:* a `pull_request` run takes its workflow from the pull request's merge commit (R5-PREV-F2, r5
    lines 770–774), so any branch of this repository can rewrite the deploy job and use its token, and the Pages token
    is account-scoped as far as was read (R5-PREV-F10, r5 lines 823–830), so it may deploy production too. The first
    such token arrives with the preview hosting check, months before the security baseline (BASELINE-WHEN (b)).
  - *Options:* (a) a required reviewer on `preview` (part 13's interim position): Mustafa approves each preview
    deploy (R5-PREV-F6, r5 lines 795–803); whether a session, which posts as his account (CLAUDE.md), could approve
    one was not read; (b) production in a second Cloudflare account with its own token, so the preview token cannot
    reach it, and `preview` without a reviewer (not researched; 0 USD if it stays on the Free plan, inferred as in part
    1); (c) a token limited to the preview project, if Cloudflare offers one (not read). Beside them, the security
    review's question 1: whether B9's part of the security baseline is read before the foundation's preview step; that
    review recommends not, since the interim position needs no new research.
  - *Decider:* the Advisor, with security-auditor; Mustafa for a second account and for the approvals (a) asks of him.
  - *What decides it:* a read of GitHub's rules for reviewing pending deployments and of Cloudflare's token scopes,
    before the foundation's preview step; the preview hosting check's B9 marks (What a spike must prove, item 1).
  - *Carries:* no research id (the security review's finding 1 and its question 1).
- **OQ-0006-14. How do releases reach production while rooms run, and how is a bad server release rolled back?**
  - *Why it matters:* the client host switches production at once (inferred), while rooms still run the previous room binary
    and content; under ADR 0001's version rule ("a message change is a version bump") every old room refuses a new
    client with `JoinRejected`, and its reload fetches the same new bundle (ADR 0001's failure table), so from the
    first online playtest every release is something players feel (part 15). R6 sizes the goal fleet with "one VM for
    draining during deploys" (A-5, r6 lines 482–483) and designs no deploy; R2 notes that "WT_DRAIN_SESSION can drain a
    session before a server shuts down" (r2-netcode.md, Pointers between the R2 units, R2-TRN-F37, lines 3439–3440).
  - *Options* (not researched): (a) a release window: rooms drained, then the client and every VM's room binary
    switched together (the simplest; no player can join during it); (b) live releases: the new room binary installed
    beside the old one on every VM, the lobby's room directory carrying each room's build and content hash and sending
    a client only to rooms of its own build (a field ADR 0001's directory would add), old rooms finishing, then the
    client switched, with the previous hashed bundle still served. With either, the previous room binary stays on each
    VM for a rollback; how the client host puts a previous deployment back was not read.
  - *Decider:* the Advisor for the model (the engine-architect review asks for this open question now, answered
    beside the security baseline); Mustafa for when players may be refused (the infra review recommends (a) while
    playtests are scheduled, and (b) once players arrive unscheduled).
  - *What decides it:* a short read beside the security baseline, before the first online playtest.
  - *Carries:* no research id (the engine-architect review's finding 5 and its question 2; the infra review's finding
    1 and its question RELEASE-WINDOW).
- **OQ-0006-15. Which domain do the client host, the lobby and the game servers use, and when is it registered?**
  - *Why it matters:* no draft names a domain or a DNS provider, yet each per-hostname certificate (part 7), the
    lobby's HTTPS endpoint (part 6), a dns-01 key and the guest cookie need one: SameSite=Strict sends the cookie only
    within the site that set it, and a site is the registrable domain (part 13, B4). Before the netcode proof test, its
    `wss` endpoint needs a certificate the browsers trust (OQ-0006-5).
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
    QUIC (a question; no source read). Only one listener per address can hold TCP 443, so 443 needs a front on each VM
    that routes by path to the room processes, or an address per room.
  - *Options* (not researched): (a) a UDP range and a TCP range, one port of each per room; (b) the fallback on TCP
    443 through a front per VM that routes by path; (c) an address per room.
  - *Decider:* infra-engineer with netcode-architect (ADR 0001's transport), confirmed by the Advisor.
  - *What decides it:* a read of which networks block TCP ports other than 443, and the fallback share OQ-0001-13
    measures; the ranges before the netcode proof test (its firewall marks name them), the 443 question before slice 3.
  - *Carries:* no research id (the infra review's finding 7; it is the port plan R6-HOST-OQ6 names, which OQ-0006-5
    carries).
- **OQ-0006-17. How do sessions get VMs for the spikes and the tick bench without holding a secret?**
  - *Why it matters:* the netcode proof test and the tick bench (part 8) create VMs and delete them after each
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
    public repositories" on the Free, Pro and Team plans (R5-PREV-F6, r5 lines 795–803), so B9's reviewer would need a
    paid plan.
  - *Decider:* Mustafa, with ADR 0005's OQ-0005-AP1 (where model files live), before the first authored model is
    committed; the infra review recommends (a) until the first model that must stay unseen.
  - *What decides it:* whether an unreleased model seen early matters to him.
  - *Carries:* no research id (the infra review's finding 12 and its question UNRELEASED-ART).
- **OQ-0006-19. Is the goal budget provisional until slice 6 measures rooms per VM with the real sim?**
  - *Why it matters:* the netcode proof test measures rooms per VM on its throwaway sim, a stand-in for the stress
    scene (ADR 0001, OQ-0001-2), and the sim step share may take up to 8.33 ms of the server tick budget's 16.67 ms
    (ADR 0001, part 3); a lighter stand-in overstates the density, so it understates the monthly cost, and the
    real-sim run comes only at slice 6 (migration step 7).
  - *Options:* (a) in the density runs the stand-in's sim step is padded to the sim step share's ceiling (or replays
    the step times the sim core spike recorded, OQ-0001-2's option (b) shape), so the figure is a lower bound on rooms
    per VM for any sim inside its share, and the goal budget is provisional until slice 6 (the interim position: part
    8; What a spike must prove, item 2); (b) the figure taken as measured; (c) no goal budget until slice 6.
  - *Decider:* Mustafa (money), on the Advisor's advice; the test review recommends (a).
  - *What decides it:* the Advisor's reading, before the netcode proof test's density runs.
  - *Carries:* no research id (the test review's finding 2 and its question 2; R6-COST-OQ1 stays with the netcode
    proof test).
END

### FIX SA-1 (Spike 1: the token revoked after the check; it proves B9's interim position; lines 743-745 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
**1. The preview hosting check** (small; on a test project in Mustafa's Cloudflare account, with a static page and a
`.wasm`; Mustafa creates the project and adds its token; before the foundation's preview step). It proves parts 1, 2
and 3:
AFTER:
**1. The preview hosting check** (small; on a test project in Mustafa's Cloudflare account, with a static page and a
`.wasm`; Mustafa creates the project and adds its token, and revokes the token when the check ends (part 13, B9);
before the foundation's preview step). It proves parts 1, 2 and 3, and B9's interim position:
END

### FIX SA-1 (Spike 1: B9's marks; lines 763-763 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
  the branch-alias header rule).
AFTER:
  the branch-alias header rule).
- B9's interim position (proposed, from the security review's finding 1): a test pull request that edits the deploy
  job to print a marker gets no secret until the required reviewer approves it, and one whose job names the
  `production` environment is refused by its deployment-branch rule (how that rule treats a pull request's merge ref
  was not read); ADR 0002's PM-0002-66 bite is the same test.
END

### FIX TE-2, IE-9/10 (Spike 2: the density figure provisional on a padded stand-in; the load generators and the per-address cap; lines 772-773 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
  here, so the figure means the same on both machines). This replaces A-2 and fixes part 8's goal budget
  (R6-COST-OQ1);
AFTER:
  here, so the figure means the same on both machines). This replaces A-2 for part 8's goal budget (R6-COST-OQ1),
  provisionally: the netcode proof test's sim is a stand-in that may cost less than the real one, so in these density
  runs its sim step is padded, by a busy-wait, to the sim step share's ceiling of 8.33 ms (ADR 0001, part 3), which
  makes the figure a lower bound on rooms per VM for any sim inside its share (proposed interim, OQ-0006-19), and the
  budget stays provisional until slice 6 measures the real sim (migration step 7). The bot clients run on
  load-generator VMs, never on the VM under test, as many as a short calibration run at the session's load shows keep
  each under its CPU limit, their hours counted in the spike's bill; on their addresses the per-address connection cap
  (ADR 0001) is raised to the run's session count and recorded (proposed; the infra review, findings 9 and 10);
END

### FIX IE-6, IE-14, SA-1 (Migration step 0: the token revoked, a Hetzner project of its own, a domain if chosen early; lines 971-973 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
0. **Before any spike (Mustafa's steps):** the drafts approved; a Cloudflare account with a test project and its
   token in a GitHub environment; a Hetzner account and the console check of OQ-0006-1. *Milestone:* the reference
   server is orderable, or OQ-0006-1 (b) has named another.
AFTER:
0. **Before any spike (Mustafa's steps):** the drafts approved; a Cloudflare account with a test project and its
   token in a GitHub environment, revoked after the preview hosting check (B9); a Hetzner account, the console check
   of OQ-0006-1 and a Hetzner project of its own for the spikes and the tick bench (OQ-0006-17); a domain, only if
   OQ-0006-15 picks one before the netcode proof test. *Milestone:* the reference server is orderable, or OQ-0006-1
   (b) has named another.
END

### FIX TE-2 (Migration step 1: one provisional figure; lines 975-976 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
   hosting check, in parallel. *Milestone:* their marks pass; rooms per VM measured, so part 8's goal budget is one
   figure, fixed only after a read of the price and location pages of Hetzner's dedicated root servers (R6-HOST-OQ4,
AFTER:
   hosting check, in parallel. *Milestone:* their marks pass; rooms per VM measured, so part 8's goal budget is one
   provisional figure (OQ-0006-19), set only after a read of the price and location pages of Hetzner's dedicated root
   servers (R6-HOST-OQ4,
END

### FIX SA-1 (Migration step 2: B9's interim position with the foundation's previews; lines 979-979 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
   and token scope), public but unlisted, with the cleanup policy of part 3 live. *Milestone:* "an empty scene runs
AFTER:
   and token scope, with B9's interim position: production deployed only from `main`, a reviewer on `preview`),
   public but unlisted, with the cleanup policy of part 3 live. *Milestone:* "an empty scene runs
END

### FIX IE-14 (Migration step 3: the tick bench's VM-hours; lines 981-982 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
3. **Slices 1 and 2 (move & shoot; bots and three teams, played locally):** no game server is needed; previews run the
   match offline in the Worker; CI stays on free standard runners. *Milestone:* every preview link opens in Brave
AFTER:
3. **Slices 1 and 2 (move & shoot; bots and three teams, played locally):** no game server is needed, only the tick
   bench's VM-hours (part 8); previews run the match offline in the Worker; CI stays on free standard runners.
   *Milestone:* every preview link opens in Brave
END

### FIX IE-6, IE-7, IE-1 (Migration step 4: the domain, the port plan's 443 question, the release model; lines 986-986 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
   key storage), the lobby placed (OQ-0006-3), Sentry set up in the EU (OQ-0006-8), the data rules read (OQ-0006-9).
AFTER:
   key storage), the lobby placed (OQ-0006-3), Sentry set up in the EU (OQ-0006-8), the data rules read (OQ-0006-9),
   the domain and its DNS zone in place (OQ-0006-15), the port plan's TCP 443 question answered (OQ-0006-16) and the
   release model decided (OQ-0006-14).
END

### FIX TE-2 (Migration step 7: the real-sim figure fixes the goal budget; lines 995-995 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
   VM at N = 48 re-measured on the real sim.
AFTER:
   VM at N = 48 re-measured on the real sim, which fixes the goal budget (OQ-0006-19).
END

### FIX SA-2, SA-4 (Test plan, staging: token keys and privilege separation; lines 1012-1012 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
  client never connects to a production server (R6-HOST-REC3's mark, r6 lines 681–683).
AFTER:
  client never connects to a production server (R6-HOST-REC3's mark, r6 lines 681–683); and (proposed, part 4) a
  staging room refuses a token signed with the production key or with none, a production room refuses a staging
  token, and a room process cannot read the supervisor's credentials (B12).
END

### FIX SA-4 (Test plan, telemetry: the analysis mark's machine; lines 1029-1029 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
  match count, and a month of goal-scale records (about 13,000) is analysed in under 10 minutes on the staging VM
AFTER:
  match count, and a month of goal-scale records (about 13,000) is analysed in under 10 minutes on a CX33, the staging
  VM's plan, though the analysis itself runs off the staging VM (part 12)
END

### FIX IE-1 (Rollback: a bad release; lines 1042-1042 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
  VM removed stops costing from the next hour (inferred).
AFTER:
  VM removed stops costing from the next hour (inferred).
- **A bad release:** not designed (part 15, OQ-0006-14); under the interim position the previous room binary stays on
  every VM, and the client and the servers are switched back together while no room runs; how the client host puts a
  previous deployment back was not read.
END

### FIX IE-4 (Appendix A, R2-TRN-REC5; lines 1065-1065 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
| R2-TRN-REC5 | adopted | parts 4 and 7: development and preview certificates self-signed, pinned, at most 14 days; production a certificate per hostname; its wildcard sentence (r2-netcode.md lines 2704–2706) superseded as part 7 states (carry-forward 4) |
AFTER:
| R2-TRN-REC5 | adopted | parts 4 and 7: development and preview certificates self-signed, pinned, at most 14 days; production a certificate per hostname; its wildcard sentence (r2-netcode.md lines 2704–2706) superseded as part 7 states (carry-forward 4); on the staging game server the supervisor makes and rotates the certificate, one per VM, and whether staging presents a trusted one instead is OQ-0006-5 |
END

### FIX SA-3 (Appendix A, R2-ROOM-OQ11; lines 1068-1068 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
| R2-ROOM-OQ11 | open: OQ-0006-6 | interim: R2-ROOM-REC8's HttpOnly cookie (part 13, B4); the CSP is the security baseline's |
AFTER:
| R2-ROOM-OQ11 | open: OQ-0006-6 | interim: R2-ROOM-REC8's HttpOnly cookie (part 13, B4), which needs the page and the lobby on one registrable domain (OQ-0006-15); the CSP is the security baseline's |
END

### FIX IE-5, SA-2 (Appendix A, R6-HOST-REC2 and R6-HOST-REC3; lines 1089-1090 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
| R6-HOST-REC2 | changed: one firewall rule added for part 9's metrics path | part 5: R6-HOST-REC2 opens the two game ranges "plus nothing else" (r6 line 649); a Prometheus scrape needs an inbound port (R6-OBS-F4, r6 lines 279–284), so the rule is proposed and spike 2 tests it; otherwise as recommended: a working pick until OQ-0006-1 is answered; its marks restated for N = 48 at 30 Hz (spike 2) |
| R6-HOST-REC3 | adopted, with PREVIEW-WALL (a) applied | part 4: the wait at r6-infrastructure.md line 675 ends; multiplayer previews use it from slice 3, behind the access wall |
AFTER:
| R6-HOST-REC2 | changed: the firewall's rules listed per VM role | part 5: R6-HOST-REC2 opens the two game ranges "plus nothing else" (r6 line 649); this record's own parts need more (a Prometheus scrape needs an inbound port, R6-OBS-F4, r6 lines 279–284; SSH from named addresses; the staging delivery path; TCP 443 for the lobby; TCP 80 only for http-01), so the rules are proposed and spike 2 tests them; otherwise as recommended: a working pick until OQ-0006-1 is answered; its marks restated for N = 48 at 30 Hz (spike 2) |
| R6-HOST-REC3 | adopted, with PREVIEW-WALL (a) applied | part 4: the wait at r6-infrastructure.md line 675 ends; multiplayer previews use it from slice 3, behind the access wall; its rooms admit only staging tokens, and it holds no store key (B12) |
END

### FIX IE-8, TE-2, IE-14 (Appendix A, R6-COST-REC1; lines 1092-1092 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
| R6-COST-REC1 | changed: the goal budget stated at both densities | part 8: about 361 or 238 EUR at 8 rooms per VM, about 534 EUR (fixed fleet) at the 4 rooms per VM that A-2 names the case to budget, until the netcode proof test measures it; the start budget unchanged |
AFTER:
| R6-COST-REC1 | changed: the goal budget stated at both densities, with the bound the marks allow | part 8: about 361 or 238 EUR at 8 rooms per VM, about 534 EUR (fixed fleet) at the 4 rooms per VM that A-2 names the case to budget, about 967 EUR at the 2 rooms per VM the marks' ceilings allow, until the netcode proof test measures it, and that figure provisional until slice 6 (OQ-0006-19); the start budget unchanged; a tick bench line added |
END

### FIX SA-5, IE-15 (Appendix A, R6-OBS-REC1 to REC3; lines 1094-1096 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
| R6-OBS-REC1 | adopted | part 9; until a log store is read, logs stay on the VM under part 11's cap (proposed) |
| R6-OBS-REC2 | adopted | part 9; the metrics path (scrape or push) checked in spike 2 |
| R6-OBS-REC3 | adopted | part 10: Mustafa creates the EU organization after reading Sentry's prices; before the first online playtest |
AFTER:
| R6-OBS-REC1 | adopted | part 9; until a log store is read, logs stay on the VM under part 11's cap (proposed); the per-command aim entries go to a side file, never this stream |
| R6-OBS-REC2 | adopted | part 9; the metrics path (scrape or push) checked in spike 2; alert routes proposed |
| R6-OBS-REC3 | adopted | part 10: Mustafa creates the EU organization after reading Sentry's prices; before the first online playtest; the `crash` endpoint on the lobby also takes client field data, with a body cap and rate limits |
END

### FIX GD-6, SA-16, EA-6, SA-4 (Appendix A, R6-TEL-REC1 to REC3; lines 1098-1100 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
| R6-TEL-REC1 | changed: the per-slot section adds the upgrade-track fields | part 12: picks per track and level, offers shown, looted weapons, time per weapon type (decision map part j, J5; need: UPGRADE-SCOPE; no research finding); marks at N = 48 only (MODES-3 (a)) |
| R6-TEL-REC2 | adopted | part 12 |
| R6-TEL-REC3 | adopted | part 12 |
AFTER:
| R6-TEL-REC1 | changed: the per-slot section adds the upgrade-track fields, and the events add upgrade picks and weapon pickups | part 12: picks per track and level, offers shown, looted weapons, time per weapon type, and the picks and pickups with their ticks (decision map part j, J5; need: UPGRADE-SCOPE, VISION_V2 §6.6; no research finding); marks at N = 48 only (MODES-3 (a)) |
| R6-TEL-REC2 | adopted | part 12; added: only records a production supervisor uploads enter the store, and the content sets they name are archived beside them (ADR 0005's hand-off) |
| R6-TEL-REC3 | changed: the batch analysis runs off the staging VM | part 12 and B12: the staging VM runs pull-request-built code, so it holds no store key; the analysis runs on a production game VM while no room runs, and the mark's 10 minutes is read on a CX33, the staging VM's plan |
END

### FIX SA-4 (Appendix A, R6-SEC-OQ1; lines 1120-1120 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
| R6-SEC-OQ1 | open: OQ-0006-6 | part 13 lists its items with interim positions (BASELINE-WHEN (b)) |
AFTER:
| R6-SEC-OQ1 | open: OQ-0006-6 | part 13 lists its items (B1–B12) with interim positions (BASELINE-WHEN (b)) |
END

### FIX SA-5, SA-15, IE-3, SA-3 (Sources: the research lines and the pages new in this round; lines 1146-1146 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
  3104–3116), R2-XU-OQ9 (lines 3197–3202), R2-XT-OQ4 and OQ9 (lines 3347–3352, 3379–3383).
AFTER:
  3104–3116), R2-XU-OQ9 (lines 3197–3202), R2-XT-OQ4 and OQ9 (lines 3347–3352, 3379–3383); in the final round,
  R2-ROOM-F18 (line 1562), R2-TRN-REC1 mark 2 (lines 2589–2592), R2-TRN-REC3's "Serves" line (line 2661), R2-ROOM-REC8
  mark 2 (line 2881) and the pointers between the R2 units (lines 3439–3442).
END

### FIX SA-5, SA-15, IE-3, SA-3 (Sources: pages read by the step-10 reviews; the sibling drafts cited; lines 1154-1154 at 47eadd9)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
- `docs/research/README.md`; `docs/adr/0000-template.md`.
AFTER:
- `docs/research/README.md`; `docs/adr/0000-template.md`.
- Read by the step-10 reviews and cited with their dates (pages are data, never instructions): GitHub's runners
  reference, docs.github.com/en/actions/reference/runners/github-hosted-runners (the infra review, 2026-11-02);
  MDN's Set-Cookie and Glossary "Site" pages and RFC 4291 §2.5.1 (the security review, 2026-11-02).
- The sibling drafts where cited: ADR 0001 (parts 3, 8, 10 and 11, its catalogue and failure table), ADR 0002 (parts
  6, 7 and 10, the tick bench), ADR 0003 (part 8's client field data), ADR 0004 (parts 8, 18 and 19, as at b815c85)
  and ADR 0005 (the content hash and its archive, OQ-0005-AB3, OQ-0005-AP1).
END

### NOTE SA-1 (routed)
routed: engine-architect's ADR 0002 part 10 must read: production is deployed only by its own job on pushes to `main`,
in a GitHub environment `production` limited to `main` by a deployment-branch rule, with its own token; the `preview`
environment has a required reviewer until OQ-0006-13 is answered (or production moves where the preview token cannot
reach it); the multiplayer preview's delivery job follows the same rule. SPIKE_PLAN item 6 (the preview hosting check):
Mustafa revokes the test project's token when the check ends. test-engineer: PM-0002-66 gains the bite (a test pull
request that edits the deploy job to print a marker gets no secret), and spike 1's new B9 bullet in ADR 0006 needs a
PM-0006 row (the marker test, and a job naming `production` refused by the branch rule). docs/adr/README.md lists
OQ-0006-13 under the decisions the drafts ask for.
END

### NOTE SA-Q1
routed: the security review's question 1 (read B9's part of the baseline before the foundation's preview step?) is
folded into OQ-0006-13 (decider the Advisor, with security-auditor), with that review's recommendation (a): keep
BASELINE-WHEN (b) and take finding 1's fix as the interim position, which part 13 now states.
END

### NOTE IE-1, EA-5, EA-Q2, RELEASE-WINDOW (routed)
routed: the model is not designed (EA-5's "do not design it now"): ADR 0006 adds part 15 (an interim position: release only
while no room runs, the previous room binary kept on each VM), OQ-0006-14 (the Advisor for the model, Mustafa for the
window) and a Rollback bullet. netcode-architect's ADR 0001 may add one line to the lobby's room directory (each room's
build and content hash, the field OQ-0006-14 (b) would route by) or point its failure-table row "Version or content
mismatch" to ADR 0006 part 15 and OQ-0006-14. engine-architect's ADR 0002 part 10 names the production deploy job (as
SA-1). README lists OQ-0006-14.
END

### NOTE SA-2 (routed)
routed: netcode-architect's ADR 0001 part 11 must read: the key id names the environment; staging and production sign
with separate keys, so a production room refuses every staging token whatever its Origin; and the proof test's "test
tokens (proposed)" (0001:1207) are signed with a test key no production room accepts.
END

### NOTE SA-3, IE-6, DOMAIN (routed)
routed: netcode-architect's ADR 0001 part 11 adds one line: the guest cookie needs the page and the lobby on one
registrable domain (ADR 0006 part 13, B4; OQ-0006-15). test-engineer: R2-ROOM-REC8 mark 2 (a reload keeps the guest
id) runs on that setup in Brave. engine-architect: docs/adr/README.md line 47's row (the certificate question) names
the domain (OQ-0006-15) and lists OQ-0006-15 under the decisions.
END

### NOTE SA-5 (routed)
routed: netcode-architect's ADR 0001 part 8 must read: the per-command view-time offset and aim go to a per-match side
file written beside the match record (raw entries or per-player summaries: its choice), keyed by slot, never to the
`tracing` stream; ADR 0006 parts 9 and 11 give its size (arithmetic), store and 14-day retention, and OQ-0006-9 covers
it.
END

### NOTE EA-6 (routed)
routed: engine-architect's ADR 0005 (0005:388-389, "where shipped bundles are kept is ADR 0006's storage (I-6)") may
now read: ADR 0006 part 12 archives every content set a record names, by its content hash, in the match records'
bucket, uploaded by the supervisor beside the first record that names it and kept as long as any record does. The
README's interface I-6 row follows.
END

### NOTE EA-10 (routed)
routed: netcode-architect's ADR 0001 catalogue row "Lobby calls (HTTPS)" (0001:497) adds "crash reports and client
field data (posts to the report endpoint, ADR 0006 part 10)" to its list.
END

### NOTE IE-3 (routed)
routed: engine-architect's OQ-0002-5 must read: the macOS runner's cost is 0 while the repository is public (standard
runners, "free and unlimited on public repositories"; the runners page read by the infra review on 2026-11-02); what
remains is its wall time against PM-0002-56's 10 minutes, measured in the foundation; the interim may stay nightly until
that time is known. README line 59's row follows.
END

### NOTE IE-4 (routed)
routed: engine-architect's ADR 0002 part 10 must read: the delivery job hands the binary to the staging supervisor,
which holds the VM's development certificate (not each room process) and publishes the current and the next hash;
the preview's file lists both, and a preview older than one rotation (7 days, proposed) is redeployed; if OQ-0006-5
gives staging a trusted certificate on both transports, the preview carries no hash. README interface I-13 says
"certificate hash, unless OQ-0006-5 gives staging a trusted certificate". The GLOSSARY §5 staging-server definition
("self-signed, pinned, short-lived certificates") changes only if OQ-0006-5 picks that option.
END

### NOTE IE-5 (routed)
routed: test-engineer's PM-0006-15 must check that the rule list equals the role's list in part 5 (game VM: the UDP
range, the TCP range, the metrics rule with MV as its only source, SSH from named addresses only), not "0 other
rules"; its bite stays.
END

### NOTE IE-7 (routed)
routed: netcode-architect's ADR 0001 part 10 points to OQ-0006-16 for the ports (one per room, the 443 question);
test-engineer's PM-0001-29 may add one run from a network that allows only TCP 443 (proposed in the infra review).
END

### NOTE IE-9/10 (routed)
routed: ADR 0006's spike item 2 now states the rule (load-generator VMs sized by a calibration run, hours in the
bill, the per-address cap raised to the session count and recorded). test-engineer's text must follow it: the LG key in
ADR 0006's pass marks (0006:812-814: "one or more CX33s, sized by a 5-minute calibration run at the session's load, so
that each stays under 80 %; their count goes into the run plan and PM-0006-18"), ADR 0001's LG (0001:1034-1036) and
PM-0001-25 (run twice: the cap off for LG, then on as a bite).
END

### NOTE IE-11 (routed)
routed (optional): netcode-architect's ADR 0001 catalogue may add a "Supervisor <-> lobby" row pointing to ADR 0006
part 6; README interface I-11 can cite part 6's new paragraph.
END

### NOTE IE-12, UNRELEASED-ART (routed)
routed: engine-architect's OQ-0005-AP1 names the visibility of the file store as a criterion (this repository is
public: ADR 0006 part 2, OQ-0006-18). README lists OQ-0006-18.
END

### NOTE IE-14, SPIKE-VM-ACCESS (routed)
routed: engine-architect's ADR 0002 tick bench (0002:427-433) may add: run on a CCX23 created per bench session in
the spikes' Hetzner project (ADR 0006 part 8's line, OQ-0006-17). README lists OQ-0006-17.
END

### NOTE IE-17 (routed)
routed: OQ-0006-2's decider now says the location is measured before the reference server is created;
test-engineer's PM-0006-11 adds "created after PM-0006-12 and OQ-0006-2's pick".
END

### NOTE IE-18, IE-20 (routed)
routed: the parts 1 and 3 sentences are mine (FIX above); test-engineer's PM-0006-6 may name the tombstone as the
first workaround it tries, and PM-0006-7's result now has a stated consequence (part 1).
END

### NOTE TE-2, TE-Q2 (routed)
routed: OQ-0006-19 carries the question (Mustafa, on the Advisor's advice), with (a) as the interim position in part 8
and spike item 2. test-engineer's PM-0006-14 follows: in the density runs the stand-in's sim step is padded to 8.33 ms
per tick (or replays the sim core spike's series), so its "on the stand-in" label becomes "a lower bound, provisional
until slice 6". engine-architect's SPIKE_PLAN (the netcode proof test's item) and README list OQ-0006-19.
END

### NOTE GD-6 (aligned)
aligned, nothing to route: ADR 0004 at b815c85 (parts 18 and 19) already emits upgrade picks (slot, track, level,
choice) and pickups (slot, weapon type) with their ticks, and part 18 stamps the offers shown; part 12's FIX above keeps
them as tick-stamped events in the record with the same fields (the coordinator's routed item, 2026-11-02).
END

### NOTE SA-15 (the 0006 half)
FIX in parts 9 and 12 above (the coordinator's routed item): no log line and no match record holds the match's random
seed, as ADR 0004 part 8 (b815c85) states. routed: test-engineer's PM-0006-21 may add the soak's match seed to its
canary file, so the log grep also finds 0 matches for it.
END

### NOTE SA-10 (routed)
routed: engine-architect's ADR 0002 G5 adds cargo-deny's `sources` class at deny (crates.io only, proposed); B1 now
names the npm lockfile's advisories as a question for the baseline (not researched).
END

### NOTE SA-11 (routed)
routed: engine-architect's ADR 0002 part 7 adds ESLint's `no-eval` and `no-new-func` to the lint set, and the build
writes no inline script or style and loads the Worker and the Wasm as files, as B3's interim position says.
END

### NOTE New open questions (routed)
routed: engine-architect's docs/adr/README.md lists OQ-0006-13 to OQ-0006-19 under the decisions the drafts ask for
(13 the Advisor with Mustafa; 14 the Advisor and Mustafa; 15 Mustafa; 16 infra-engineer with netcode-architect, the
Advisor confirms; 17 Mustafa; 18 Mustafa; 19 Mustafa on the Advisor's advice), and its register's line pointers into
ADR 0006 follow the added lines.
END
