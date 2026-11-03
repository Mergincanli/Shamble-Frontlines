# stop-infra-engineer fixes (ADR 0006, stop answers 2026-11-03)

infra-engineer, ADR 0006 only, outside its "Pass marks" section and the two checklists (test-engineer's, not touched).
Docs at 4374a1c (unchanged on disk; base main 9e7db28). Every BEFORE was copied by script from the file on disk, whole
lines, each occurring once. "-2" ids are second places for a row that the map did not list: M8-2 (part 7's second
sentence: the proof test's two certificates), M4-2 (part 13's table, row B9: the Hetzner token), M5-2 (Consequences,
"Harder": Mustafa's VMs and his domain), H16-2 (OQ-0006-10 stays open, with the pointer condition 1 asks for), RC7-2
(the Test plan's stub-record mark, the same one word). FIX M5 also carries migration step 0's "the drafts approved"
under SPIKE-ORDER (b). PM-0001-63 is cited as test-engineer's new
row in ADR 0001's 60 Hz group (rooms per VM at 60 Hz with the goal-budget check).

### FIX O15
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
**Author:** infra-engineer (pass marks by test-engineer, plan step 4) · **Reviewed by:** (filled in at review) · **Signed off by Mustafa:** no
AFTER:
**Author:** infra-engineer (pass marks by test-engineer, plan step 4) ·
**Reviewed by:** (filled in at review) ·
**Signed off by Mustafa:** as the basis for the spikes (2026-11-03, SPIKE-ORDER (b)); not yet accepted
END

### FIX X3
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
- **VISION_V2 §3 and §4** (counted like LOOP_V2 under the session's trace reading, decision map part j, J3): the start
AFTER:
- **VISION_V2 §3 and §4** (counted like LOOP_V2, by the Advisor's answer to OQ-0001-14 (2026-11-03): "yes, a
  VISION_V2 §3 or §4 trace counts, marked as such"; decision map part j, J3): the start
END

### FIX M2
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
     needs none, R6-OBS-F4, r6 lines 279–284; spike 2 checks it); and SSH, by key only, from named source addresses only
     (proposed: without an admin path no one can install a room binary or read the logs part 9 keeps on the VM; who
     holds the key is OQ-0006-17);
AFTER:
     needs none, R6-OBS-F4, r6 lines 279–284; spike 2 checks it); and SSH, by key only, from named source addresses only
     (proposed: without an admin path no one can install a room binary or read the logs part 9 keeps on the VM; on the
     spike VMs sessions get SSH access, OQ-0006-17, answered by Mustafa (2026-11-03): "sessions get SSH access only and
     no Hetzner token until the security baseline", and how a session's key is made and held without it reading a
     secret it did not make is for the netcode proof test's own plan, with security-auditor and the advisor: the
     advisor's reading, 2026-11-03);
END

### FIX M8
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
   per-hostname certificate needs a domain and a DNS zone, and no draft names one yet (OQ-0006-15; the registrar, the
   DNS provider and their prices are not researched). Development
AFTER:
   per-hostname certificate needs a domain and a DNS zone, and no draft names one yet: Mustafa registers one for the
   netcode proof test's WebSocket leg ("for the WebSocket leg, a domain I register": Mustafa, 2026-11-03,
   SPIKE-MACHINES), and which domain, registrar and DNS host, and whether production uses it later, stay open
   (OQ-0006-15, answered in part; the registrar, the DNS provider and their prices are not researched). Development
END

### FIX M8-2
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
   netcode proof test runs on the development certificates for WebTransport (which certificate its WebSocket fallback
   presents is OQ-0006-5); the production form is needed from the first online
AFTER:
   netcode proof test runs on the development certificates for WebTransport, "a private hash-pinned certificate for
   WebTransport" as Mustafa set for the spikes (2026-11-03, SPIKE-MACHINES); its WebSocket fallback presents a
   browser-trusted certificate on the domain he registers, OQ-0006-5's development case (i), issued by a method the
   proof test's own plan sets with security-auditor (the advisor's reading, 2026-11-03); the production form is needed
   from the first online
END

### FIX H12
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
     The goal budget is stated at both densities until the netcode proof test measures rooms per VM, and that figure
     stays provisional until slice 6 measures the real sim (OQ-0006-19; R6-COST-OQ1, r6 lines 880–882; R2-ROOM-REC2
     mark 2 is ADR 0001's test). Egress stays inside the included traffic at
     either density: at 4 rooms, 4 × 3.156 GB × 730 h ≈ 9.2 TB per VM, under 20 TB (arithmetic on r6 lines 461–463).
AFTER:
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
END

### FIX M3
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
     0.46 EUR a playtest. Who creates and deletes these VMs is OQ-0006-17.
AFTER:
     0.46 EUR a playtest. Mustafa creates and deletes these VMs himself, and sessions get SSH access only and no
     Hetzner token, until the security baseline: OQ-0006-17's answer (Mustafa, 2026-11-03) covers the bench and the
     soaks too, since the baseline comes before the first online playtest (the advisor's reading, 2026-11-03), so every
     bench session and every soak waits for him to create its VMs (at the placeholder, 20 bench hours a month).
END

### FIX RC7
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
    schema version in every record and a Parquet compaction job only once a query needs it; a crashed room leaves a stub
    record marked incomplete (r6-infrastructure.md, Recommendations §4, R6-TEL-REC2, lines 812–827). Only records a
AFTER:
    schema version in every record and a Parquet compaction job only once a query needs it; a crashed room leaves a stub
    record marked crashed, one word with ADR 0001 part 11 and PM-0001-59 (R6-TEL-REC2 calls it "a stub record marked
    incomplete"; r6-infrastructure.md, Recommendations §4, R6-TEL-REC2, lines 812–827). That stub record and the crash
    report (part 10) are written and kept under part 11's proposed retention, with no new period: OQ-0001-18, answered
    by the Advisor (2026-11-03): "end the match, as proposed, and keep the crash and match records" (what "keep" means:
    the advisor's reading, 2026-11-03). Only records a
END

### FIX M4-2
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
    | B9 | The tokens CI holds (the client host's deploy tokens, for previews and for production; the Sentry upload token, "a token: Mustafa's secret", r6 line 1082), a multiplayer preview's binary delivery (R6-XT-OQ4), and the Hetzner API token the spikes, the tick bench and the 3-hour soak before each playtest need (OQ-0006-17) | their scope and handling | each held only by the job that needs it, in a GitHub environment, as R5-PREV-REC1 does for the deploy token (proposed for the others); production deployed only from `main`, in its own environment `production` with a `main`-only deployment-branch rule and its own token (ADR 0002 part 10); a required reviewer on `preview`, OQ-0006-13's proposed interim (what keeps a rewritten deploy job from the token); the preview hosting check's token revoked when the check ends; the Hetzner token in a project of its own; all added by Mustafa (below; OQ-0006-13) | 0002 (the jobs); 0006 (the scope; OQ-0006-13) |
AFTER:
    | B9 | The tokens CI holds (the client host's deploy tokens, for previews and for production; the Sentry upload token, "a token: Mustafa's secret", r6 line 1082), a multiplayer preview's binary delivery (R6-XT-OQ4), and a Hetzner API token for the spikes, the tick bench and the 3-hour soak before each playtest, which no session gets before the baseline (OQ-0006-17, answered by Mustafa, 2026-11-03) | their scope and handling | each held only by the job that needs it, in a GitHub environment, as R5-PREV-REC1 does for the deploy token (proposed for the others); production deployed only from `main`, in its own environment `production` with a `main`-only deployment-branch rule and its own token (ADR 0002 part 10); a required reviewer on `preview`, OQ-0006-13's proposed interim (what keeps a rewritten deploy job from the token); the preview hosting check's token revoked when the check ends; no Hetzner token: Mustafa creates and deletes those VMs himself, in a Hetzner project of its own (OQ-0006-17); all added by Mustafa (below; OQ-0006-13) | 0002 (the jobs); 0006 (the scope; OQ-0006-13) |
END

### FIX M4
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
    when the check ends (part 14); (d) the Hetzner API token for the spikes, the tick bench and the 3-hour soak before
    each playtest belongs to a Hetzner project of its own, never the production project (a project's token presumably
    reaches all its VMs: not read;
    OQ-0006-17). Whether a session, which posts on GitHub as Mustafa's account (CLAUDE.md), could approve a pending
AFTER:
    when the check ends (part 14); (d) no session holds a Hetzner API token until the security baseline: Mustafa
    creates and deletes the VMs for the spikes, the tick bench and the 3-hour soak before each playtest himself, in a
    Hetzner project of its own, never the production project (a project's token presumably reaches all its VMs: not
    read; OQ-0006-17, answered by Mustafa, 2026-11-03: "a separate Hetzner project. I create and delete the VMs myself
    for each run; sessions get SSH access only and no Hetzner token until the security baseline"; the bench and the
    soaks: the advisor's reading, 2026-11-03). Whether a session, which posts on GitHub as Mustafa's account
    (CLAUDE.md), could approve a pending
END

### FIX M5-2
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
- **Harder:** Mustafa holds accounts with Cloudflare and Hetzner before the first spikes (a test project on Cloudflare Pages
  and a Hetzner console check), and later with Sentry and Grafana; every multiplayer preview needs his login once per
AFTER:
- **Harder:** Mustafa holds accounts with Cloudflare and Hetzner before the first spikes (a test project on
  Cloudflare Pages and a Hetzner console check), and later with Sentry and Grafana; until the security baseline he
  creates and deletes every VM of the spikes, the tick bench and the playtest soaks himself (OQ-0006-17), and he
  registers a domain before the netcode proof test (OQ-0006-15; both 2026-11-03); every multiplayer preview needs his
  login once per
END

### FIX M7
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
- **OQ-0006-5. How are the per-hostname production certificates issued, how long do they live, and which certificate
  does the WebSocket fallback present on development and staging servers?**
AFTER:
- **OQ-0006-5. How are the per-hostname production certificates issued, how long do they live, and which certificate
  does the WebSocket fallback present on development and staging servers?**
  - Answered in part by Mustafa (2026-11-03): "Certificates: a private hash-pinned certificate for WebTransport; for
    the WebSocket leg, a domain I register." So the netcode proof test's WebSocket leg takes the development case (i)
    below, a trusted certificate on his domain (the advisor's reading, 2026-11-03); still open: how that certificate is
    issued on a spike VM (http-01 and tls-alpn-01 need an open port, dns-01 a secret, and OQ-0006-16's port plan
    applies), decided in the netcode proof test's own plan with security-auditor; the staging case, shapes (A) and (B)
    below, decided by the Advisor; and the production certificates, decided by security-auditor in the security
    baseline.
END

### FIX H16-2
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
  - *Why it matters:* the start and goal budgets rest on placeholder player counts (A-4, A-5) and exclude VAT.
AFTER:
  - *Why it matters:* the start and goal budgets rest on placeholder player counts (A-4, A-5) and exclude VAT. Since
    2026-11-03 a 60 Hz tick also needs the goal budget to hold at the rooms per VM measured at 60 Hz (TICK-60-HEADROOM's
    addition, part 8; ADR 0001's PM-0001-63), and that check sizes its fleet from A-5's peak of 20 rooms, so this
    question's answer bears on it too.
END

### FIX M6
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
- **OQ-0006-15. Which domain do the client host, the lobby and the game servers use, and when is it registered?**
AFTER:
- **OQ-0006-15. Which domain do the client host, the lobby and the game servers use, and when is it registered?**
  - Answered in part by Mustafa (2026-11-03): "for the WebSocket leg, a domain I register." So (b) is decided for the
    netcode proof test: a domain he registers before it, for its WebSocket leg (the advisor's reading, 2026-11-03);
    still open: which domain, which registrar and which DNS host, and whether production uses it later, decided by
    Mustafa. The options below stay as the record of what was weighed.
END

### FIX M1
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
- **OQ-0006-17. How do sessions get VMs for the spikes, the tick bench and the 3-hour soak before each playtest
  without holding a secret?**
AFTER:
- **OQ-0006-17. How do sessions get VMs for the spikes, the tick bench and the 3-hour soak before each playtest
  without holding a secret?**
  - Answered by Mustafa (2026-11-03): "a separate Hetzner project. I create and delete the VMs myself for each run;
    sessions get SSH access only and no Hetzner token until the security baseline. VMs exist only during a run." So
    (a) is decided, in (b)'s project of its own but with no token; (b)'s token and (c) stay below as the record of
    what was weighed. The security baseline comes before the first online playtest, so this covers the tick bench and
    the 3-hour soaks too; how a session's SSH key is made and held without it reading a secret it did not make is for
    the netcode proof test's own plan, with security-auditor and the advisor (the advisor's reading, 2026-11-03).
END

### FIX H16
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
    (ADR 0001, part 3); a lighter stand-in overstates the density, so it understates the monthly cost, and the
    real-sim run comes only at slice 6 (migration step 7).
AFTER:
    (ADR 0001, part 3); a lighter stand-in overstates the density, so it understates the monthly cost, and the
    real-sim run comes only at slice 6 (migration step 7). Since 2026-11-03 the 60 Hz rule reads this goal budget too
    (TICK-60-HEADROOM's addition, part 8): at slice 6, the 60 Hz goal-budget check (ADR 0001's PM-0001-63) is judged
    against the figure this question fixes (the advisor's reading, 2026-11-03).
END

### FIX M9
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
- **OQ-0006-20. What may the netcode proof test's VMs cost?**
AFTER:
- **OQ-0006-20. What may the netcode proof test's VMs cost?**
  - Answered in part by Mustafa (2026-11-03): "VMs exist only during a run." It sets no cap (the advisor's reading,
    2026-11-03); still open: the cost cap, decided by Mustafa (money). He creates each run's VMs himself (OQ-0006-17),
    so under the interim below each run plan's cost reaches him before its VMs exist.
END

### FIX H13
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
  budget stays provisional until slice 6 measures the real sim (migration step 7). The bot clients run on
AFTER:
  budget stays provisional until slice 6 measures the real sim (migration step 7). The count at 60 Hz and its
  goal-budget check (TICK-60-HEADROOM's addition, part 8) are not marks of this item: they are ADR 0001's PM-0001-63,
  on this counting rule, in wave 1's 60 Hz run after the sim core spike (SPIKE-ORDER (b)) and again at slice 6. The
  bot clients run on
END

### FIX M5
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
0. **Before any spike (Mustafa's steps):** the drafts approved; a Cloudflare account with a test project and its
   token in a GitHub environment, revoked after the preview hosting check (B9); a Hetzner account, the console check
   of OQ-0006-1 and a Hetzner project of its own for the spikes, the tick bench and the playtest soaks (OQ-0006-17); a
   domain, only if
   OQ-0006-15 picks one before the netcode proof test. *Milestone:* the reference server is orderable, or OQ-0006-1
   (b) has named another.
AFTER:
0. **Before any spike (Mustafa's steps):** the drafts signed off as the basis for the spikes (2026-11-03,
   SPIKE-ORDER (b)); a Cloudflare account with a test project and its
   token in a GitHub environment, revoked after the preview hosting check (B9); a Hetzner account, the console check
   of OQ-0006-1 and a Hetzner project of its own for the spikes, the tick bench and the playtest soaks, in which he
   creates and deletes the VMs himself for each run, with SSH access only and no Hetzner token for sessions until the
   security baseline (OQ-0006-17, answered by Mustafa, 2026-11-03); a domain he registers for the netcode proof
   test's WebSocket leg ("for the WebSocket leg, a domain I register": OQ-0006-15, answered in part). *Milestone:* the
   reference server is orderable, or OQ-0006-1 (b) has named another.
END

### FIX RC7-2
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
  of recovery; a record of schema version n is read by the script for version n + 1; a room killed with SIGKILL
  mid-match leaves a stub record marked incomplete (R6-TEL-REC2's mark, r6 lines 824–827); the analysis script reproduces
AFTER:
  of recovery; a record of schema version n is read by the script for version n + 1; a room killed with SIGKILL
  mid-match leaves a stub record marked crashed (part 12; R6-TEL-REC2's mark says "marked incomplete", r6 lines
  824–827); the analysis script reproduces
END

### NOTE RC8
no change: part 11 (ADR 0006:419-425) already caps every store the answer keeps (crash reports at our endpoint 14
days, proposed; match records while their content hash is still analysed; Sentry's crash-report cap open). By the
advisor's ruling on ROOM-CRASH, "keep" means written and kept under part 11's proposed retention, with no new period;
FIX RC7 says so at part 12's stub record, so part 11 itself needs nothing.
END

### NOTE H23
no change: ADR 0006:50-51 (TICK-30, a settled input: "60 Hz only with the 60 Hz margin of at least 25 % (ADR 0001
owns the rule and its marks)") links GLOSSARY §8 and states a necessary condition; by the advisor's ruling on
TICK-60-HEADROOM such restatements may stay once §8 is amended (condition 3, engine-architect's entry). ADR 0001 owns
the rule and its marks, PM-0001-63 included, and part 8 (FIX H12) states the addition in this record.
END

### NOTE S15
no change: OQ-0006-19 option (a)'s "OQ-0001-2's option (b) shape" (ADR 0006:996) names a padding method for the
30 Hz density runs, which stays a weighed option of OQ-0006-19 (still open). OQ-0001-2's answer, (a) as an early
signal only, is about the 60 Hz margin run's sim load and is recorded in ADR 0001 (netcode-architect); it does not
change what this option describes. The key's "as OQ-0001-2 sets it" (S16) is test-engineer's.
END

### NOTE O21
no change: migration step 1 (ADR 0006:1280-1281: "the netcode proof test with this record's operations marks, and the
preview hosting check, in parallel") already matches Mustafa's SPIKE-ORDER (b): "The preview hosting check may run
alongside any of them." This record runs no other heavy spike; wave 1's order is SPIKE_PLAN's (engine-architect) and
ADR 0001's step 1 (netcode-architect).
END

### NOTE M10
no change: OQ-0006-1 (ADR 0006:762-773) stays open. No answer gives his console check, and "a separate Hetzner
project" does not say whether a CCX23 and a CX33 can be ordered in an EU location; it stays in "For Mustafa" (the
verdict's item 1).
END

### NOTE M13
no change at my place: spike item 2's "the development certificates: all four browsers connect with a pinned
self-signed certificate" (ADR 0006:1081) is Mustafa's "a private hash-pinned certificate for WebTransport"; the
WebSocket leg's certificate is stated in part 7 (FIX M8-2). PM-0006-25 and -27 are test-engineer's (its NOTE M13).
END

### NOTE X5
no change: ADR 0006 has no "VISION_V2 §4 only" need line (grep: none); its §4 traces sit beside §6 traces (parts 5, 7,
9 and 10), and "What it serves" marks §3 and §4 as such, now under the Advisor's answer to OQ-0001-14 (FIX X3).
END

### NOTE LA3
no change: by the ruling on LATER, the drafts' own entries for the twelve LATER questions are not edited. Mine are
OQ-0006-13 (ADR 0006:896), OQ-0006-14 (:915) and OQ-0006-18 (:975); they keep their deciders and timings as written,
and README carries the design-session marker (engine-architect). For the report: the ruling times OQ-0006-13 "before
the foundation's client-host token", while the entry says "before the first client-host token is added to this
repository", which is the preview hosting check's token (spike item 1). The ruling's later timing governs, with B9's
interim (each preview deploy waits for Mustafa's approval) covering the check until then; by LA3 the entry's wording
stays, so the report should name the difference.
END

