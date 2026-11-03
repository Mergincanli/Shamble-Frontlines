# r6b infra-engineer fixes (ADR 0006 outside Pass marks)

### FIX R6B-IE-01
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
Serve the player build and every preview link from Cloudflare Pages, and run game servers on Hetzner Cloud in one EU
location, one room per process under a small supervisor per VM, on a dedicated-vCPU CCX23 named as the reference
server, with the shared CX33 measured beside it. Keep the budget flat and small at the start (about 18 to 102 EUR a
month) and state the goal budget at both room densities until the netcode proof test measures how many rooms fit one
AFTER:
Serve the player build and every preview link from Cloudflare Pages, and run game servers on Hetzner Cloud in one EU
location (Falkenstein, `fsn1`, to start: part 5), one room per process under a small supervisor per VM, on a
dedicated-vCPU CCX23 named as the reference server; a shared-vCPU fleet (K3) waits for a later measurement (part 5).
Keep the budget flat and small at the start (about 18 to 102 EUR a month; the lean end rests on the CX33's price,
part 8) and state the goal budget at both room densities until the netcode proof test measures how many rooms fit one
END

### FIX R6B-IE-02
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
4. **The staging game server.** Previews share one small VM,
   [the staging game server](../research/GLOSSARY.md#the-staging-game-server), proposed a CX33 (4 vCPU, 8 GB, 8.99 EUR
   a month: r6-infrastructure.md, Findings §1, R6-HOST-F15, lines 150–159), never one VM per pull request. Its
AFTER:
4. **The staging game server.** Previews share one small VM, never one VM per pull request:
   [the staging game server](../research/GLOSSARY.md#the-staging-game-server), proposed a CX33 (4 vCPU, 8 GB, 8.99 EUR
   a month: r6-infrastructure.md, Findings §1, R6-HOST-F15, lines 150–159), which cannot be ordered in Falkenstein
   (Mustafa, 2026-11-03), so its x86 plan is OQ-0006-22. Its
END

### FIX R6B-IE-03
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
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
AFTER:
   inside them, because billing rounds up to the hour (R6-HOST-F17). The server CPU is x86-64 (the dedicated plans are
   AMD), which answers R1-DET-OQ7 for the first fleet; Arm waits until R1-DET-REC6's matrix includes it. **The
   reference server** is a CCX23 (4 dedicated vCPU, 16 GB, x86-64 AMD) (r6-infrastructure.md, Recommendations §1,
   R6-HOST-REC2, lines 645–671); this answers R2-XU-OQ9 (r2-netcode.md, Cross-unit open questions, R2-XU-OQ9, lines
   3197–3202), and every native server mark is measured on
   [one vCPU](../research/GLOSSARY.md#the-measuring-unit-one-vcpu) of it. Every EU plan showed "currently unavailable"
   on 2026-10-28 (R6-HOST-F20, r6 lines 180–182); Mustafa's console check answered OQ-0006-1 and his correction
   answered OQ-0006-2: the test servers are in Falkenstein (`fsn1`), "a CCX23 for the game server and a CAX21 for the
   48 simulated players. I created both once to try the console and deleted them right away. They'll be created fresh
   on run day, so their IP addresses will be new then." (Mustafa, 2026-11-03). So every server row runs on the CCX23,
   and the CAX21 (Arm64) carries only bot clients and the run's metrics (the pass marks' key, LG and MV); no address
   is fixed. The first production fleet starts in the test servers' location unless playtesters' round trips
   (R6-HOST-OQ2, open) say otherwise (the advisor's reading, 2026-11-03). The shared CX33 that R6-HOST-REC2 measures
   beside the reference server cannot be ordered in Falkenstein (Mustafa, 2026-11-03), so it is not measured: a shared
   fleet (K3) waits for a later measurement of a shared x86 plan that can be ordered in the fleet's location (part 8).
   *Why:* dedicated vCPUs are sold for "highly predictable resources" and name gaming servers, which the fair tick of
   VISION_V2 §4 and [the server tick budget](../research/GLOSSARY.md#the-server-tick-budget) need; 20 TB of egress per
   VM is included; the shared CX33 is 9.6 times cheaper (8.99 against 86.49 EUR), which is why a shared plan is
   measured before a fleet uses it, never assumed (r6 lines 656–659; R6-HOST-F14, F15, r6 lines 144–159). *Ids:*
   R6-HOST-REC2, R2-XU-OQ9, R1-DET-OQ7, R6-HOST-OQ3 (answered:
END

### FIX R6B-IE-04
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
   per-hostname certificate needs a domain and a DNS zone. Mustafa registers one for the netcode proof test's
   WebSocket leg ("for the WebSocket leg, a domain I register": Mustafa, 2026-11-03, SPIKE-MACHINES), and names it:
   "shamblefrontlines.com, which I register at Cloudflare" (Mustafa, 2026-11-03, DOMAIN; OQ-0006-15, answered).
AFTER:
   per-hostname certificate needs a domain and a DNS zone. The netcode proof test's WebSocket leg has one ("for the
   WebSocket leg, a domain I register": Mustafa, 2026-11-03, SPIKE-MACHINES), named
   "shamblefrontlines.com, which I register at Cloudflare" (Mustafa, 2026-11-03, DOMAIN; OQ-0006-15, answered), and
   registered (Mustafa, 2026-11-03).
END

### FIX R6B-IE-05
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
   browser-trusted certificate on the domain he registers, OQ-0006-5's development case (i), issued by a method the
AFTER:
   browser-trusted certificate on the registered domain, OQ-0006-5's development case (i), issued by a method the
END

### FIX R6B-IE-06
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
   - *Start* (MODES-3 (a); assumption A-4: up to 2 rooms at once, 18 humans each, playtests about 4 hours a day, servers
     on all month): about 18 EUR a month lean (production CX33 8.99 + staging CX33 8.99 = 17.98 EUR, 0 USD) to about
     102 EUR + 5 USD reference (production CCX23 86.49 + staging CX33 8.99 + match records on Hetzner Object Storage
     6.49 = 101.97 EUR; 5 USD only if Worker code is needed); egress about 137 GB, inside the 20 TB
     (r6-infrastructure.md, Options §2, start scenario, lines 466–480).
AFTER:
   - *Start* (MODES-3 (a); assumption A-4: up to 2 rooms at once, 18 humans each, playtests about 4 hours a day, servers
     on all month): about 18 EUR a month lean (production CX33 8.99 + staging CX33 8.99 = 17.98 EUR, 0 USD) to about
     102 EUR + 5 USD reference (production CCX23 86.49 + staging 8.99 + match records on Hetzner Object Storage
     6.49 = 101.97 EUR, where 8.99 is the staging VM's placeholder price (the CX33's, R6-HOST-F15); its plan is
     OQ-0006-22; 5 USD only if Worker code is needed); egress about 137 GB, inside the 20 TB
     (r6-infrastructure.md, Options §2, start scenario, lines 466–480). The lean case rests on the CX33's price for
     production (K3) and staging; the CX33 cannot be ordered in Falkenstein (Mustafa, 2026-11-03), so it holds only
     where a shared x86 plan can be ordered in the fleet's location and K3 passes (the advisor's reading, 2026-11-03).
END

### FIX R6B-IE-07
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
     (a fixed base plus VMs added by the hour, K2), each + 19–24 USD + Sentry's paid plan, match records included (r6 lines
     482–494). **Changed from R6-COST-REC1:** at the 4 rooms per VM that A-2 names "the case to budget" until
AFTER:
     (a fixed base plus VMs added by the hour, K2), each + 19–24 USD + Sentry's paid plan, match records and the staging
     VM at its placeholder 8.99 included here and below (r6 lines 482–494; OQ-0006-22). **Changed from R6-COST-REC1:**
     at the 4 rooms per VM that A-2 names "the case to budget" until
END

### FIX R6B-IE-08
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
   - *Fleet shape:* a fixed fleet (K1) first; K2 only once real concurrency data shows a clear peak; the shared CX33
     fleet (K3) only if the shared-vCPU measurement passes; each extra room of peak capacity costs 86.49 ÷ 8 ≈ 10.81 EUR
     a month at 8 rooms per VM, 21.62 EUR at 4 (r6-infrastructure.md, Recommendations §2, R6-COST-REC1, lines 703–717;
     scaling rule, lines 499–500).
AFTER:
   - *Fleet shape:* a fixed fleet (K1) first; K2 only once real concurrency data shows a clear peak; a shared-vCPU
     fleet (K3) only after a later measurement of a shared x86 plan that can be ordered in the fleet's location passes
     (the shared CX33 cannot be ordered in Falkenstein, Mustafa, 2026-11-03: part 5); each extra room of peak capacity
     costs 86.49 ÷ 8 ≈ 10.81 EUR a month at 8 rooms per VM, 21.62 EUR at 4 (r6-infrastructure.md, Recommendations §2,
     R6-COST-REC1, lines 703–717; scaling rule, lines 499–500).
END

### FIX R6B-IE-09
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
     cost 20 × 0.1386 = 2.77 EUR. ADR 0002's 3-hour soak before each playtest takes the same path, with its bot clients
     on a CX33 beside the reference server, never on it (as spike 2's load generators): 3 × (0.1386 + 0.0144) =
     0.46 EUR a playtest. Mustafa creates and deletes these VMs himself, and sessions get SSH access only and no
AFTER:
     cost 20 × 0.1386 = 2.77 EUR. ADR 0002's 3-hour soak before each playtest takes the same path, with its bot clients
     on a client VM beside the reference server, never on it (as spike 2's load generators): 3 × (0.1386 + 0.0176) =
     0.47 EUR a playtest (0.0176 a CAX21-hour, R6-HOST-F15). Mustafa creates and deletes these VMs himself, and
     sessions get SSH access only and no
END

### FIX R6B-IE-10
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
    (ADR 0001's) carries this record's operations marks: rooms per VM on both machines, egress per match-hour, the
    spike's own bill, logging cost, the metrics and their path, and the firewall. Both are listed under "What a spike
    must prove". *Need:* as parts 1, 3, 5, 8 and 9.
AFTER:
    (ADR 0001's) carries this record's operations marks: rooms per VM on the reference server, egress per match-hour,
    the spike's own bill, logging cost, the metrics and their path, and the firewall. Both are listed under "What a
    spike must prove". *Need:* as parts 1, 3, 5, 8 and 9.
END

### FIX R6B-IE-11
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
| **G2. Hetzner Cloud, shared vCPU (CX33, measured beside G1)** | As G1 | CX33 8.99 EUR a month (0.0144 an hour) (F15) | "designed for variable usage patterns rather than sustained high workloads" (F14): server tick [p99](../research/GLOSSARY.md#time-statistics) must be measured | As G1 | Noisy neighbours could break the server tick budget (r6 line 416) | From zero | **Pros:** 9.6 times cheaper. **Cons:** unproven for a fair tick |
AFTER:
| **G2. Hetzner Cloud, shared vCPU (CX33, measured beside G1)** | As G1 | CX33 8.99 EUR a month (0.0144 an hour) (F15) | "designed for variable usage patterns rather than sustained high workloads" (F14): server tick [p99](../research/GLOSSARY.md#time-statistics) must be measured | As G1 | Noisy neighbours could break the server tick budget (r6 line 416) | From zero | **Pros:** 9.6 times cheaper. **Cons:** unproven for a fair tick; not measured after all: the CX33 cannot be ordered in Falkenstein (Mustafa, 2026-11-03), so K3 waits (part 5) |
END

### FIX R6B-IE-12
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
### Q6. The starting location (r6-infrastructure.md, Options §1, Q4, lines 429–432)
Nuremberg (`nbg1`) or Helsinki (`hel1`), in Hetzner's one EU zone; Falkenstein (`fsn1`) only if the plans are offered
there (their product pages list "NBG1 HEL1", R6-HOST-F13, r6 lines 136–143). No latency source was read for any of them,
so the table's columns are equal but for latency, which a measurement decides (OQ-0006-2): complexity, cost and effort
are the same in all three; security the same; the look and feel differ only through round-trip time.
AFTER:
### Q6. The starting location (r6-infrastructure.md, Options §1, Q4, lines 429–432)
R6 weighed three locations in Hetzner's one EU zone (OQ-0006-2's options, kept there as the record); the product pages
it read listed two of them and not Falkenstein (R6-HOST-F13, r6 lines 136–143), and no latency source was read for any
of them, so the columns are equal but for latency: complexity, cost and effort are the same in all three; security the
same; the look and feel differ only through round-trip time. Mustafa answered it for the test servers: "both test
servers will be in Falkenstein, not Nuremberg: a CCX23 for the game server and a CAX21 for the 48 simulated players"
(Mustafa, 2026-11-03; OQ-0006-2); playtesters' round trips to `fsn1` (R6-HOST-OQ2) check it for production (part 5).
END

### FIX R6B-IE-13
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
- **Game servers (Q3, Q4).** Dedicated vCPUs win because the fair tick needs "highly predictable resources", and
  Hetzner's included 20 TB makes egress, the cost that grows with players, zero even at full load (R6-HOST-F14, F15;
  r6 lines 461–464). The shared CX33 is measured beside it because it is 9.6 times cheaper. The strongest argument against
  Hetzner is that its EU plans showed "currently unavailable" on 2026-10-28 (R6-HOST-F20) and that it is a single
  provider: OQ-0006-1 makes that a check before the netcode proof test, not a surprise during it. A fixed fleet (K1)
AFTER:
- **Game servers (Q3, Q4).** Dedicated vCPUs win because the fair tick needs "highly predictable resources", and
  Hetzner's included 20 TB makes egress, the cost that grows with players, zero even at full load (R6-HOST-F14, F15;
  r6 lines 461–464). A shared plan, 9.6 times cheaper in the CX33's case, waits for a later measurement (K3; the CX33
  cannot be ordered in Falkenstein, Mustafa, 2026-11-03). The strongest argument against
  Hetzner is that its EU plans showed "currently unavailable" on 2026-10-28 (R6-HOST-F20) and that it is a single
  provider: OQ-0006-1's console check answered the first before the netcode proof test, and since its VMs are created
  fresh on each run day, capacity on the day stays ADR 0001's failure row, with OQ-0006-1 (b) as the fallback (the
  advisor's reading, 2026-11-03). A fixed fleet (K1)
END

### FIX R6B-IE-14
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
  down (once the game's domain is registered, OQ-0006-15), names stop resolving as cached records expire, so new joins
AFTER:
  down (the game's domain is registered, OQ-0006-15), names stop resolving as cached records expire, so new joins
END

### FIX R6B-IE-15
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
- **Harder:** Mustafa holds accounts with Cloudflare and Hetzner before the first spikes (a test project on
  Cloudflare Pages and a Hetzner console check), and later with Sentry and Grafana; until the security baseline he
  creates and deletes every VM of the spikes, the tick bench and the playtest soaks himself (OQ-0006-17, Mustafa,
  2026-11-03; the bench and the soaks: the advisor's reading, 2026-11-03) and gives his OK to each run's cost cap in
  that run's plan (OQ-0006-20, Mustafa, 2026-11-03); he registers shamblefrontlines.com at Cloudflare (OQ-0006-15,
  Mustafa, 2026-11-03) before the netcode proof test (the advisor's reading, 2026-11-03); every multiplayer preview
AFTER:
- **Harder:** Mustafa holds accounts with Cloudflare and Hetzner before the first spikes (a test project on
  Cloudflare Pages and a Hetzner project; his console check is done, OQ-0006-1), and later with Sentry and Grafana;
  until the security baseline he
  creates and deletes every VM of the spikes, the tick bench and the playtest soaks himself (OQ-0006-17, Mustafa,
  2026-11-03; the bench and the soaks: the advisor's reading, 2026-11-03) and gives his OK to each run's cost cap in
  that run's plan (OQ-0006-20, Mustafa, 2026-11-03); he holds shamblefrontlines.com at Cloudflare (OQ-0006-15),
  registered (Mustafa, 2026-11-03); every multiplayer preview
END

### FIX R6B-IE-16
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
- **Revisit when:** Hetzner's EU plans are not orderable (OQ-0006-1); PREV-DELETE fails on Pages (move to Workers
  static assets); the netcode proof test measures rooms per VM (a provisional goal budget) and slice 6 re-measures it
  on the real sim (the goal budget fixed, OQ-0006-19); the shared CX33 passes (K3) or
AFTER:
- **Revisit when:** Hetzner's plans cannot be ordered in `fsn1` on a run day (OQ-0006-1 (b)); PREV-DELETE fails on
  Pages (move to Workers
  static assets); the netcode proof test measures rooms per VM (a provisional goal budget) and slice 6 re-measures it
  on the real sim (the goal budget fixed, OQ-0006-19); a shared x86 plan that can be ordered in the fleet's location
  passes a later measurement (K3) or
END

### FIX R6B-IE-17
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
- **OQ-0006-1. Can Mustafa order Hetzner's CCX23 and CX33 in an EU location?**
  - *Why it matters:* every native server mark and the netcode proof test run on the reference server; on 2026-10-28
AFTER:
- **OQ-0006-1. Can Mustafa order Hetzner's CCX23 and CX33 in an EU location?**
  - Answered by Mustafa (2026-11-03): "in Falkenstein the cost-optimized x86 line (CX33) can't be ordered; only its
    Arm64 version can." (his Hetzner check) and "a CCX23 for the game server and a CAX21 for the 48 simulated players.
    I created both once to try the console and deleted them right away. They'll be created fresh on run day, so their
    IP addresses will be new then." (his correction). So (a) holds: the reference server stands, a CCX23 in `fsn1`
    (part 5); the CX33 is not measured (K3 waits, part 8), and the staging VM's plan is OQ-0006-22. VMs created fresh
    on each run day mean capacity on the day is not guaranteed, so ADR 0001's failure row on Hetzner capacity and
    (b)'s route stay the fallback (the advisor's reading, 2026-11-03). R6-HOST-OQ4 and R6-HOST-OQ7, which this entry
    carries, stay open, each with its own trigger as written under *Options*. The options below stay as the record of
    what was weighed.
  - *Why it matters:* every native server mark and the netcode proof test run on the reference server; on 2026-10-28
END

### FIX R6B-IE-18
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
  - *What decides it:* the console offering both plans in `nbg1` or `hel1` before the netcode proof test is scheduled.
AFTER:
  - *What decides it:* the console offering both plans in an EU location before the netcode proof test is scheduled.
END

### FIX R6B-IE-19
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
- **OQ-0006-2. Which EU location: Nuremberg, Helsinki, or Falkenstein if offered?**
AFTER:
- **OQ-0006-2. Which EU location: Nuremberg, Helsinki, or Falkenstein if offered?**
  - Answered by Mustafa (2026-11-03): "both test servers will be in Falkenstein, not Nuremberg: a CCX23 for the game
    server and a CAX21 for the 48 simulated players" (his correction). So both test servers are created in `fsn1` on
    each run day, and the first production fleet starts there unless playtesters' round trips (R6-HOST-OQ2, still
    open) say otherwise (the advisor's reading, 2026-11-03). The options below stay as the record of what was weighed.
END

### FIX R6B-IE-20
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
  - Answered in part by Mustafa (2026-11-03): "Certificates: a private hash-pinned certificate for WebTransport; for
    the WebSocket leg, a domain I register." So the netcode proof test's WebSocket leg takes the development case (i)
AFTER:
  - Answered in part by Mustafa (2026-11-03): "Certificates: a private hash-pinned certificate for WebTransport; for
    the WebSocket leg, a domain I register." (the domain: registered (Mustafa, 2026-11-03), OQ-0006-15). So the netcode
    proof test's WebSocket leg takes the development case (i)
END

### FIX R6B-IE-21
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
  - Answered by Mustafa (2026-11-03): "for the WebSocket leg, a domain I register." (SPIKE-MACHINES) and
    "shamblefrontlines.com, which I register at Cloudflare." (DOMAIN). So (b) is decided for the netcode proof test: a
    domain he registers before it, for its WebSocket leg (the advisor's reading, 2026-11-03). Production uses the same
AFTER:
  - Answered by Mustafa (2026-11-03): "for the WebSocket leg, a domain I register." (SPIKE-MACHINES) and
    "shamblefrontlines.com, which I register at Cloudflare." (DOMAIN); registered (Mustafa, 2026-11-03): "DOMAIN: done.
    shamblefrontlines.com is registered at Cloudflare." So (b) is decided for the netcode proof test: the registered
    domain serves its WebSocket leg (the advisor's reading, 2026-11-03). Production uses the same
END

### FIX R6B-IE-22
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
    15 to 30 × 0.1386 = 2.08 to 4.16 EUR) and the CX33 hours of SH, MV and the load generators (as many as spike 2's
    calibration run asks); so a 5 EUR cap over everything would send the plan back to the draft at its planning check.
AFTER:
    15 to 30 × 0.1386 = 2.08 to 4.16 EUR) and the CX33 hours of SH, MV and the load generators (as many as spike 2's
    calibration run asks; the rig is now the CCX23 and CAX21s, part 5); so a 5 EUR cap over everything would send the
    plan back to the draft at its planning check.
END

### FIX R6B-IE-23
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
    rates, part 12). Progression lives under VISION_V2 §6.10 `[LATER]` (the advisor's reading, 2026-11-03).
AFTER:
    rates, part 12). Progression lives under VISION_V2 §6.10 `[LATER]` (the advisor's reading, 2026-11-03).
- **OQ-0006-22. Which x86 plan runs the staging game server, now that the proposed CX33 cannot be ordered in
  Falkenstein?**
  - *Why it matters:* the staging VM runs the room server (part 4), and an Arm plan is an Arm host (ADR 0004's
    PM-0004-26 first); part 8's lean case rests on the CX33's price, and Mustafa's check (2026-11-03) found "in
    Falkenstein the cost-optimized x86 line (CX33) can't be ordered; only its Arm64 version can."
  - *Options:* (a) a CX33 (8.99 EUR a month) only where it can be ordered; (b) a CCX13 (43.49, R6-HOST-F15) if it can
    be ordered in the fleet's location; (c) a shared x86 "Regular Performance" plan (CPX32 35.99, R6-HOST-F15), whose
    locations are not researched. Prices excluding VAT, read 2026-10-28 (r6-infrastructure.md, Findings §1, lines
    150–159). Until it is answered, part 8 and PM-0001-63 carry 8.99 as the staging VM's placeholder price.
  - *Decider:* Mustafa (money), with the Advisor.
  - *What decides it:* his choice before slice 3 (online), when the staging game server is first created (part 4).
  - *Carries:* no research id (the advisor's ruling HZ-STAGING, 2026-11-03, on his Hetzner check).
END

### FIX R6B-IE-24
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
**2. This record's operations marks inside the netcode proof test** (ADR 0001's spike, on the reference server; its
precondition is OQ-0006-1). They prove parts 5, 7, 8 and 9:
- rooms per VM: on the CCX23 and on the CX33, 8 rooms of 48 players (bots) as 8 processes, each room's server tick p99
  within 10 % of the single-room figure, over 20 minutes, repeated at 3 times of day on the shared host
  (r6-infrastructure.md, R6-HOST-REC2's mark, lines 667–671; R2-ROOM-REC2 mark 2, r2-netcode.md lines 2775–2782, ADR
  0001's test); and rooms per VM measured on both machines at 48, 36 and 18 players (R6-COST-REC1's mark, r6 lines
  714–717), counted as the most rooms for which R2-ROOM-REC2 marks 1 and 2 still hold (the counting rule is proposed
  here, so the figure means the same on both machines). This replaces A-2 for part 8's goal budget (R6-COST-OQ1),
AFTER:
**2. This record's operations marks inside the netcode proof test** (ADR 0001's spike, on the reference server, a
CCX23 in `fsn1`; its precondition, OQ-0006-1, is answered). They prove parts 5, 7, 8 and 9:
- rooms per VM: on the CCX23, 8 rooms of 48 players (bots) as 8 processes, each room's server tick p99
  within 10 % of the single-room figure, over 20 minutes
  (r6-infrastructure.md, R6-HOST-REC2's mark, lines 667–671; R2-ROOM-REC2 mark 2, r2-netcode.md lines 2775–2782, ADR
  0001's test); and rooms per VM measured at 48, 36 and 18 players (R6-COST-REC1's mark, r6 lines
  714–717), counted as the most rooms for which R2-ROOM-REC2 marks 1 and 2 still hold (the counting rule is proposed
  here); the shared host's half of R6-HOST-REC2's mark (3 times of day) waits with K3 (part 8). This replaces A-2 for
  part 8's goal budget (R6-COST-OQ1),
END

### FIX R6B-IE-25
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
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
AFTER:
0. **Before any spike (Mustafa's steps):** the drafts signed off as the basis for the spikes (2026-11-03,
   SPIKE-ORDER (b)); a Cloudflare account with a test project and its
   token in a GitHub environment, revoked after the preview hosting check (B9); a Hetzner account (its console check
   done: OQ-0006-1, answered by Mustafa, 2026-11-03) and a Hetzner project of its own for the spikes, the tick bench
   and the playtest soaks, in which he creates the VMs fresh for each run and deletes them himself, with SSH access
   only and no Hetzner token for sessions until the security baseline (OQ-0006-17, answered by Mustafa, 2026-11-03;
   the bench and the soaks: the advisor's reading, 2026-11-03); shamblefrontlines.com for the netcode proof test's
   WebSocket leg (OQ-0006-15, answered by Mustafa, 2026-11-03: "for the WebSocket leg, a domain I register" and
   "shamblefrontlines.com, which I register at Cloudflare"), registered (Mustafa, 2026-11-03). *Milestone:* the
   reference server is orderable (OQ-0006-1, answered), or OQ-0006-1 (b) has named another.
END

### FIX R6B-IE-26
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
  match count, and a month of goal-scale records (about 13,000) is analysed in under 10 minutes on a CX33, the staging
  VM's plan, though the analysis itself runs off the staging VM (part 12)
AFTER:
  match count, and a month of goal-scale records (about 13,000) is analysed in under 10 minutes on the staging VM's
  plan (OQ-0006-22), though the analysis itself runs off the staging VM (part 12)
END

### FIX R6B-IE-27
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
- **Game servers:** if the CCX23 cannot be ordered or fails the marks, read OQ-0006-1's fallbacks and re-run the
  netcode proof test there; every draft that cites the reference server (N34, I-15) follows the new name. If the shared
  CX33 fails, the fleet stays on dedicated vCPUs (K1). VMs are billed by the hour with a monthly cap (R6-HOST-F17), so a
AFTER:
- **Game servers:** if the CCX23 cannot be ordered on a run day or fails the marks, read OQ-0006-1's fallbacks and
  re-run the netcode proof test there; every draft that cites the reference server (N34, I-15) follows the new name.
  A shared-vCPU fleet (K3) comes only after a later measurement of a shared x86 plan that can be ordered in the
  fleet's location passes; until then, or if it fails, the fleet stays on dedicated vCPUs (K1). VMs are billed by the
  hour with a monthly cap (R6-HOST-F17), so a
END

### FIX R6B-IE-28
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
| R6-HOST-REC2 | changed: the firewall's rules listed per VM role | part 5: R6-HOST-REC2 opens the two game ranges "plus nothing else" (r6 line 649); this record's own parts need more (a Prometheus scrape needs an inbound port, R6-OBS-F4, r6 lines 279–284; SSH from named addresses; the staging delivery path; TCP 443 for the lobby; TCP 80 only for http-01), so the rules are proposed and spike 2 tests them; otherwise as recommended: a working pick until OQ-0006-1 is answered; its marks restated for N = 48 at 30 Hz (spike 2) |
AFTER:
| R6-HOST-REC2 | changed: the firewall's rules listed per VM role; the shared CX33 not measured | part 5: R6-HOST-REC2 opens the two game ranges "plus nothing else" (r6 line 649); this record's own parts need more (a Prometheus scrape needs an inbound port, R6-OBS-F4, r6 lines 279–284; SSH from named addresses; the staging delivery path; TCP 443 for the lobby; TCP 80 only for http-01), so the rules are proposed and spike 2 tests them; the CX33 it measures beside the reference server cannot be ordered in Falkenstein (Mustafa, 2026-11-03), so K3 waits for a later measurement of a shared x86 plan (part 8); otherwise as recommended: the CCX23 stands as the reference server, in `fsn1` (OQ-0006-1 and OQ-0006-2, answered by Mustafa, 2026-11-03); its marks restated for N = 48 at 30 Hz (spike 2) |
END

### FIX R6B-IE-29
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
| R6-TEL-REC3 | changed: the batch analysis runs off the staging VM | part 12 and B12: the staging VM runs pull-request-built code, so it holds no store key; the analysis runs on a production game VM while no room runs, and the mark's 10 minutes is read on a CX33, the staging VM's plan |
| R6-HOST-OQ1 | open: OQ-0006-1 | Mustafa's console check |
| R6-HOST-OQ2 | open: OQ-0006-2 | a round-trip measurement |
AFTER:
| R6-TEL-REC3 | changed: the batch analysis runs off the staging VM | part 12 and B12: the staging VM runs pull-request-built code, so it holds no store key; the analysis runs on a production game VM while no room runs, and the mark's 10 minutes is read on the staging VM's plan (OQ-0006-22) |
| R6-HOST-OQ1 | answered: OQ-0006-1 (Mustafa, 2026-11-03) | his console check: the CCX23 and the CAX21 can be ordered in `fsn1`, the CX33 cannot (part 5) |
| R6-HOST-OQ2 | open: carried by OQ-0006-2, which Mustafa answered for the test servers (2026-11-03) | playtesters' round trips to `fsn1` against VISION_V2 §4's "about 100 ms"; the first production fleet starts there unless they say otherwise (part 5) |
END

### FIX R6B-IE-30
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
| R6-HOST-OQ4 | open: OQ-0006-1 | Fly.io, GameLift, Netcup, OVHcloud and Scaleway are read only if Hetzner Cloud is ruled out; Hetzner's dedicated root servers are read before part 8's goal budget is fixed, whatever OQ-0006-1's answer (r6 lines 862–865; migration step 1) |
AFTER:
| R6-HOST-OQ4 | open: carried by OQ-0006-1, whose console check is answered; this id keeps its own trigger | Fly.io, GameLift, Netcup, OVHcloud and Scaleway are read only if Hetzner Cloud is ruled out; Hetzner's dedicated root servers are read before part 8's goal budget is fixed, whatever OQ-0006-1's answer (r6 lines 862–865; migration step 1) |
END

### FIX R6B-IE-31
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
| R6-HOST-OQ7 | open: OQ-0006-1 | read at sign-up; under 1 EUR per VM either way |
AFTER:
| R6-HOST-OQ7 | open: carried by OQ-0006-1, whose console check is answered; this id keeps its own trigger | read at sign-up; under 1 EUR per VM either way |
END

### NOTE R6B-IE-N1
no change: 91 (Gives: the reference server every server mark is measured on), 246 (kept word for word inside R6B-IE-03), 753 (every server mark measured on one named machine) and 1103 (every server mark at 30 Hz): true again, since every server row runs on the CCX23 (AMENDMENT, DS dropped; condition 2).
END

### NOTE R6B-IE-N2
no change: 267 (part 6) and 825 (OQ-0006-3 (b)): the lobby's "for example a CX33 at 8.99 EUR a month" is a labelled placeholder example, which HZ-STAGING keeps as an example.
END

### NOTE R6B-IE-N3
no change: 606 (G1's dated finding "currently unavailable" on 2026-10-28, a record), 610 (the root servers, Netcup, OVHcloud, Scaleway row: read only if OQ-0006-1 rules Hetzner Cloud out, consistent with its answer (a)) and 618 (K3's option row, the record; its trigger now reads in part 8, R6B-IE-08): option tables keep what was weighed.
END

### NOTE R6B-IE-N4
no change: 801-808 and 810 (OQ-0006-1's Why it matters, Options with R6-HOST-OQ1 at 803, Decider and Carries): kept as the record of what was weighed; the answer line (R6B-IE-17) says (a) holds and that R6-HOST-OQ4 and OQ7 stay open with their own triggers; 809 is R6B-IE-18.
END

### NOTE R6B-IE-N5
no change: 812-818 (OQ-0006-2's Why it matters, Options with `nbg1`, `hel1`, `fsn1` at 813, Decider, What decides it with R6-HOST-OQ2 at 817, Carries at 818): the record HZ-LOCATION keeps, under the answer line of R6B-IE-19; its title at 811 is the question asked, kept.
END

### NOTE R6B-IE-N6
no change: 981 (OQ-0006-15's title, "when is it registered?", the question asked) and 988-999 (its Why it matters, Options, Decider, What decides it, Carries, with "a domain before the netcode proof test" at 993 and 997): the record the answer line keeps ("The options below stay as the record of what was weighed"); the answer is R6B-IE-21.
END

### NOTE R6B-IE-N7
no change: 1032 (OQ-0006-17's "What decides it", with OQ-0006-1's console check): a field of an answered entry, kept as its record; the check itself is now answered (R6B-IE-17).
END

### NOTE R6B-IE-N8
no change: 1075-1077 (OQ-0006-20's options (a) to (c) with "0.0144 a CX33-hour" and the interim): the record keeps its words, as AMENDMENT condition 9 rules; its one pointer is at 1074 (R6B-IE-22).
END

### NOTE R6B-IE-N9
no change: 1146-1149 (spike item 2: bot clients on load-generator VMs, never the VM under test, as many as a short calibration run shows; the connection cap raised on their addresses and recorded): already LG-COUNT's rule (the option he chose) and address-neutral; LG's CAX21, the day's addresses and the host keys are the pass marks' key (test-engineer, AMENDMENT condition 11).
END

### NOTE R6B-IE-N10
no change: 1300, and 1189-1199 and 1267-1280 (the advisor's list): inside the Pass marks section, test-engineer's text (AMENDMENT condition 13). 1300's route (the reference server not orderable or failing: OQ-0006-1 (b) and a new reference server) stays valid as the fallback for a run day without capacity (the advisor's reading, 2026-11-03, HZ-OQ1).
END

### NOTE R6B-IE-N11
no change: 360-368 (part 8's CI runners: macOS is a standard runner, free while the repository is public): it already reads the runner's price, as AMENDMENT 2 rules (no change, or at most one pointer); the runner standing in as MAC is ADR 0001's key (test-engineer), to which ADR 0006's key points.
END

### NOTE R6B-IE-N12
no change: 214, 396, 643, 731 and 1455 (Prometheus on the staging VM, M1): the production design; in the proof test MV, the metrics role on LG's CAX21, stands in for it (the pass marks' key, test-engineer).
END

### NOTE R6B-IE-N13
no change: 1513, 1515 and 1516 (Appendix A rows R6-HOST-OQ3, OQ5, OQ6, inside the advisor's 1510-1517): none rests on the CX33, the location or OQ-0006-1; 1511, 1512, 1514 and 1517 are R6B-IE-29 to -31.
END

