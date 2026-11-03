# r5 infra-engineer fixes (ADR 0006)

### FIX OQ15-part7
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
   per-hostname certificate needs a domain and a DNS zone, and no draft names one yet: Mustafa registers one for the
   netcode proof test's WebSocket leg ("for the WebSocket leg, a domain I register": Mustafa, 2026-11-03,
   SPIKE-MACHINES), and which domain, registrar and DNS host, and whether production uses it later, stay open
   (OQ-0006-15, answered in part; the registrar, the DNS provider and their prices are not researched). Development
AFTER:
   per-hostname certificate needs a domain and a DNS zone. Mustafa registers one for the netcode proof test's
   WebSocket leg ("for the WebSocket leg, a domain I register": Mustafa, 2026-11-03, SPIKE-MACHINES), and names it:
   "shamblefrontlines.com, which I register at Cloudflare" (Mustafa, 2026-11-03, DOMAIN; OQ-0006-15, answered).
   Production uses the same domain, since it is the game's name and part 6 needs one registrable domain
   (the advisor's reading, 2026-11-03). Which DNS host serves it, and what the domain and the DNS host cost, are not
   researched. Development
END

### FIX OQ21-part12
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
    count in rates is Mustafa's (R6-TEL-OQ2); the record tags both. Kill-cams, replays and highlights (§6.8 `[LATER]`)
    would need a full replay, which is not built (option W3, r6 lines 577–578); the tick-stamped events and ADR 0004's
AFTER:
    count in rates is Mustafa's (R6-TEL-OQ2); the record tags both. No field marks a match as official or community
    (OQ-0006-21, community servers): adding one is design for later (the advisor's reading, 2026-11-03). Kill-cams,
    replays and highlights (§6.8 `[LATER]`) would need a full replay, which is not built (option W3, r6 lines
    577–578); the tick-stamped events and ADR 0004's
END

### FIX F3-B9
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
    | B9 | The tokens CI holds (the client host's deploy tokens, for previews and for production; the Sentry upload token, "a token: Mustafa's secret", r6 line 1082), a multiplayer preview's binary delivery (R6-XT-OQ4), and a Hetzner API token for the spikes, the tick bench and the 3-hour soak before each playtest, which no session gets before the baseline (OQ-0006-17, answered by Mustafa, 2026-11-03) | their scope and handling | each held only by the job that needs it, in a GitHub environment, as R5-PREV-REC1 does for the deploy token (proposed for the others); production deployed only from `main`, in its own environment `production` with a `main`-only deployment-branch rule and its own token (ADR 0002 part 10); a required reviewer on `preview`, OQ-0006-13's proposed interim (what keeps a rewritten deploy job from the token); the preview hosting check's token revoked when the check ends; no Hetzner token: Mustafa creates and deletes those VMs himself, in a Hetzner project of its own (OQ-0006-17); all added by Mustafa (below; OQ-0006-13) | 0002 (the jobs); 0006 (the scope; OQ-0006-13) |
AFTER:
    | B9 | The tokens CI holds (the client host's deploy tokens, for previews and for production; the Sentry upload token, "a token: Mustafa's secret", r6 line 1082), a multiplayer preview's binary delivery (R6-XT-OQ4), and a Hetzner API token for the spikes, the tick bench and the 3-hour soak before each playtest, which no session gets before the baseline (OQ-0006-17, answered by Mustafa, 2026-11-03; the bench and the soak: the advisor's reading, 2026-11-03) | their scope and handling | each held only by the job that needs it, in a GitHub environment, as R5-PREV-REC1 does for the deploy token (proposed for the others); production deployed only from `main`, in its own environment `production` with a `main`-only deployment-branch rule and its own token (ADR 0002 part 10); a required reviewer on `preview`, OQ-0006-13's proposed interim (what keeps a rewritten deploy job from the token); the preview hosting check's token revoked when the check ends; no Hetzner token: Mustafa creates and deletes those VMs himself, in a Hetzner project of its own (OQ-0006-17); all added by Mustafa (below; OQ-0006-13) | 0002 (the jobs); 0006 (the scope; OQ-0006-13) |
END

### FIX OQ15-provider-down
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
  down (once OQ-0006-15 gives the game a domain), names stop resolving as cached records expire, so new joins fail
  while running rooms go on.
AFTER:
  down (once the game's domain is registered, OQ-0006-15), names stop resolving as cached records expire, so new joins
  fail while running rooms go on.
END

### FIX F3-OQ15-OQ20-harder
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
  creates and deletes every VM of the spikes, the tick bench and the playtest soaks himself (OQ-0006-17), and he
  registers a domain before the netcode proof test (OQ-0006-15; both 2026-11-03); every multiplayer preview needs his
  login once per
  Brave session; each production game VM needs a certificate issuance path (OQ-0006-5) and a domain (OQ-0006-15); the
  lobby still has no place in the budget (OQ-0006-3); until OQ-0006-13 is answered, every preview deploy waits for his
  approval (B9); releases wait on OQ-0006-14.
AFTER:
  creates and deletes every VM of the spikes, the tick bench and the playtest soaks himself (OQ-0006-17, Mustafa,
  2026-11-03; the bench and the soaks: the advisor's reading, 2026-11-03) and gives his OK to each run's cost cap in
  that run's plan (OQ-0006-20, Mustafa, 2026-11-03); he registers shamblefrontlines.com at Cloudflare (OQ-0006-15,
  Mustafa, 2026-11-03) before the netcode proof test (the advisor's reading, 2026-11-03); every multiplayer preview
  needs his login once per Brave session; each production game VM needs a certificate issuance path (OQ-0006-5) and a
  domain (OQ-0006-15); the lobby still has no place in the budget (OQ-0006-3); until OQ-0006-13 is answered, every
  preview deploy waits for his approval (B9); releases wait on OQ-0006-14.
END

### FIX OQ15-entry
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
  - Answered in part by Mustafa (2026-11-03): "for the WebSocket leg, a domain I register." So (b) is decided for the
    netcode proof test: a domain he registers before it, for its WebSocket leg (the advisor's reading, 2026-11-03);
    still open: which domain, which registrar and which DNS host, and whether production uses it later, decided by
    Mustafa. The options below stay as the record of what was weighed.
  - *Why it matters:* no draft names a domain or a DNS provider, yet each per-hostname certificate (part 7), the
    lobby's HTTPS endpoint (part 6), a dns-01 key and the guest cookie need one: SameSite=Strict sends the cookie only
    within the site that set it, and a site is the registrable domain (part 13, B4). Before the netcode proof test, its
    `wss` endpoint needs a certificate the browsers trust (OQ-0006-5).
AFTER:
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
END

### FIX OQ20-entry
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
  - Answered in part by Mustafa (2026-11-03): "VMs exist only during a run." It sets no cap (the advisor's reading,
    2026-11-03); still open: the cost cap, decided by Mustafa (money). He creates each run's VMs himself (OQ-0006-17),
    so under the interim below each run plan's cost reaches him before its VMs exist.
AFTER:
  - Answered by Mustafa (2026-11-03): "one cap per run, from its planned hours, written in that run's plan for my
    OK. I create and delete the VMs myself." (SPIKE-COST), after "VMs exist only during a run." (SPIKE-MACHINES). The
    options below, the interim among them, stay as the record of what was weighed.
END

### FIX OQ21-entry
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
  - *Carries:* no research id of its own: R6-COST-REC1's bill mark (r6 lines 716–717; its disposition stays
    "changed", Appendix A) and R6-HOST-F15's prices (r6 lines 150–159).
AFTER:
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
END

### FIX F3-OQ15-migration0
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
   security baseline (OQ-0006-17, answered by Mustafa, 2026-11-03); a domain he registers for the netcode proof
   test's WebSocket leg ("for the WebSocket leg, a domain I register": OQ-0006-15, answered in part). *Milestone:* the
   reference server is orderable, or OQ-0006-1 (b) has named another.
AFTER:
   security baseline (OQ-0006-17, answered by Mustafa, 2026-11-03; the bench and the soaks:
   the advisor's reading, 2026-11-03); shamblefrontlines.com, which he registers at Cloudflare for the netcode proof
   test's WebSocket leg (OQ-0006-15, answered by Mustafa, 2026-11-03: "for the WebSocket leg, a domain I register" and
   "shamblefrontlines.com, which I register at Cloudflare"; before the netcode proof test:
   the advisor's reading, 2026-11-03). *Milestone:* the reference server is orderable, or OQ-0006-1 (b) has named
   another.
END

### NOTE OQ21-part4
no change: (docs/adr/0006-infrastructure-operations.md, part 4, 0006:188-220) part 4 describes the official staging
game server for previews, not what a room needs to run: its supervisor starts the room binary a preview names
(0006:190-191) and, on that VM only, holds the rooms' certificate (0006:208-209); staging rooms admit only join tokens
signed with a staging key, and a production room refuses every staging token (0006:199-201), the per-environment keys
the advisor's ruling on COMMUNITY-SERVERS (verdict r5) finds fit "progression … on official servers only"; the staging
VM's records never enter the store (0006:216). Nothing there makes a room need a supervisor, a lobby or an official
host to start, fixes how a room is configured, or bypasses the version and content-hash check, so no sentence reads as
closing the door on his three constraints and no pointer goes in (condition 6: the one pointer is part 12's, FIX
OQ21-part12).
END

### NOTE OQ21-part5
no change: (docs/adr/0006-infrastructure-operations.md, part 5, 0006:221-258) "one room per process (R2-ROOM-REC2,
ADR 0001's)" (0006:221-222) keeps each room one program. "under a small supervisor per VM", no load balancer "since the
lobby hands out host and port" (0006:222-223), the firewall rules per VM role (0006:225-240) and "VMs stay up and rooms
start and end as processes inside them" (0006:241-242) describe the official fleet on Hetzner, not a condition for a
room to start: ADR 0002's `server` crate needs no supervisor or lobby (engine-architect a's NOTE
COMMUNITY-SERVERS-0002), and ADR 0001's room starts from values passed in (netcode-architect's NOTE
COMMUNITY-SERVERS-0001). The version and content-hash check is ADR 0001's `Join`, which part 5 leaves as it is. The
reference server is the machine the marks are measured on (0006:243-250), which says nothing about who may run a room.
So no sentence reads as closing the door, and no pointer goes in.
END

### NOTE OQ21-part6
no change: (docs/adr/0006-infrastructure-operations.md, part 6, 0006:259-280) the lobby is "a required service in this
record's scope" (0006:259-262), the scope being the service this record hosts; "Its protocol and token format are ADR
0001's" (0006:263). The supervisor's link registers "the build and protocol version of the room binaries it holds", and
the lobby starts a room "with the room id, the mode id and the overrides" (0006:274-276): his version check and his
config values (players, length, rules come with the mode and its overrides, ADR 0005's CD-6 and CD-9, engine-architect
a's NOTE COMMUNITY-SERVERS-0005) pass through it rather than being built into the room. The guest identity, and "It
keeps VISION_V2 §6.10 possible" (0006:271-272), sit on the lobby this record hosts, which fits "progression can count
on official servers only". Who signs a community room's join tokens is not designed here: the room holds only the
lobby's public key, one of its start values (netcode-architect's NOTE COMMUNITY-SERVERS-0001), and the rest is
OQ-0006-21's ("keep the door open, don't design it"). So no sentence reads as closing the door, and no pointer goes in.
END

### NOTE OQ21-part12-rest
no change: (docs/adr/0006-infrastructure-operations.md, part 12, 0006:455-503, outside FIX OQ21-part12) the header
already carries "content hash and mode id; server build id" (0006:456), so a record says what its room ran; "Only
records a production supervisor uploads enter the store", and a local match's and the staging server's records never
do (0006:484-488), which fits "progression can count on official servers only" (the advisor's ruling on
COMMUNITY-SERVERS, verdict r5: official-only storage keys fit); the private flag (0006:457) and "the record tags both"
(0006:497) belong to R6-TEL-OQ2, which OQ-0006-21 names as related and does not answer. No field, part, pass mark,
crate or "Needs outside §6" entry is added; the one pointer is FIX OQ21-part12 (the advisor's ruling: the record has no
official-or-community field, and adding one is design for later).
END

### NOTE PM-0006-18
no change: (docs/adr/0006-infrastructure-operations.md, PM-0006-18, 0006:1253) test-engineer's text, unchanged as
condition 5 says ("PM-0006-18 is unchanged"). Its "under the cap OQ-0006-20 sets" stays true with his answer (one cap
per run, in that run's plan). Its last clause, "so until it is answered the plan's cost goes to Mustafa before the
first session", now describes the time before his answer of 2026-11-03; any rewording is test-engineer's, and this
round leaves it as the record. The same holds for the interim sentence in OQ-0006-20's options (0006:1071-1072), which
FIX OQ20-entry keeps as the record ("the interim among them").
END

### NOTE OQ15-OQ20-rest
no change: the other places in my text that name OQ-0006-15 or OQ-0006-20 stay true with the answers and carry no
"answered in part" label: part 6 (0006:270-271) and B4 (0006:515) (the page and the lobby on one registrable domain);
OQ-0006-5 (0006:841-860: "a trusted certificate on his domain", "which needs the domain (OQ-0006-15)"); the Harder
bullet's "a domain (OQ-0006-15)" (kept word for word in FIX F3-OQ15-OQ20-harder); "the spike's own hosting bill under
the cap OQ-0006-20 sets" (0006:1133); migration step 4's "the domain and its DNS zone in place (OQ-0006-15)"
(0006:1364); Appendix A's R2-ROOM-OQ11 (0006:1454) and R6-COST-REC1 ("becomes the cap OQ-0006-20 sets", 0006:1478)
rows. PM-0006-15 (0006:1250, "the domain Mustafa registers") is test-engineer's and stays true. README and SPIKE_PLAN
labels are engine-architect b's (condition 5).
END

