# R6-TEL draft: balance telemetry (VISION_V2 §6.6)

All sources read 2026-10-28. Assumptions continue the R6 numbering (A-10 onward). Few outside sources exist for this
question inside the allow list: the shipped-game talks found are video only or outside it (Sources not read). Most of
what follows is design built on wave 1's recommendations and on VISION_V2 and LOOP_V2, marked as such.

## Findings
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

## Legacy facts used
- **TL13 (sim.js):** the headless match already reports a kill matrix, deliveries, trucks and buildings lost, tech and
  silo captures, era per team and frags per slot over 90–300 s: the prototype of v2's match record, and proof that
  "x3 runs, never one" (CLAUDE.md) applies to balance numbers too (STATUS figures: stall means 13.0 / 5.7 / 7.3 across
  three runs of one build).
- **GE15 (career):** per-weapon kills credit the weapon in hand when the kill is processed, so a grenade kill counts
  for the held gun. v2's damage events must carry their source id (R1-ABL-REC5, R1-ECS-REC3) or unit and weapon rates
  are wrong from day one.
- **GC18 (combat leftovers):** last-attacker fields are written but never read, and assists are never counted. Assists
  and "who contributed to the steal" need damage contribution kept per victim, which legacy never did.
- **GP11 and GE12:** the scoreboard shows only kills and deaths and the win rule is interim (HQ elimination); v2's
  record follows LOOP_V2 §8's points race instead.
- **CD2 (KVStore):** career stats live only in the browser; nothing is collected today.

## Options

### Q1. What a match record holds
- **W1. Results only** (winner, final points, duration): cheap, but answers none of "who led", "swings" or "steals".
- **W2. Results + a points timeline + objective and kill events + per-slot unit use (proposed).** Every §6.6 number is
  derivable; size estimated below (A-10).
- **W3. A full replay (every input or snapshot):** answers everything, including later kill-cams (§6.8 [LATER]), but
  is orders of magnitude larger and is a §6.8 [LATER] feature, so out of scope now.

### Q2. How it is recorded on the server
- **R-a. The room's sim emits typed gameplay events (already needed for clients, R2-BUD-REC2); a recorder in the room
  process appends them to a bounded buffer and samples team state on a tick cadence (R1-TICK-REC3); at Ended it writes
  the record (R2-ROOM-REC1).** No new path through the sim.
- **R-b. A separate telemetry stream sent live to a collector during the match:** more moving parts, and a collector
  outage then loses data mid-match.

### Q3. Storage and format
| Option | Format | Where | Cost at the goal (A-11) | Fit |
|---|---|---|---|---|
| S1. One compressed JSON-lines file per match in object storage | JSON lines | Hetzner Object Storage (EU, F5 of R6-COST) | 6.49 EUR/mo flat (1 TB included) | Simple; any tool reads it |
| S2. As S1, plus a periodic job that compacts into Parquet files for analysis | JSON lines + Parquet (F1) | same | same | Column queries over many matches; adds one job |
| S3. Cloudflare R2 instead of Hetzner | as S1/S2 | R2 (region and jurisdiction not read) | about 1 USD/mo after the free 10 GB | Cheap; EU location unverified |
| S4. A database (SQL) | rows | not read (postgresql.org, sqlite.org outside the allow list) | not computed | Needs a read first |
| S5. A hosted analytics warehouse (for example BigQuery) | tables | BigQuery's price page came back truncated | not read | Generic; not needed at this volume |

### Q4. Analysis
- **N1. A batch script over the stored records** (weekly or on demand), grouped by content hash and mode id
  (R1-DATA-REC6), on the staging VM: computes the derived numbers in REC3.
- **N2. Live dashboards** of balance numbers: no §6 item asks for live balance data; the metrics in R6-OBS cover live
  health.

## Recommendations
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
  available in Rust when column queries are needed (F1). The supervisor uploads after exit, so a crashed room's partial
  buffer can be flushed by the supervisor from a file the room appends to (proposed), and a storage outage only delays
  uploads (records wait on the VM's disk).
  *Rests on:* F1; R6-COST-F4, R6-COST-F5; R6-HOST-REC2 (the supervisor); A-11.
  *Serves:* §6.6.
  *Spike must prove (proposed):* with object storage unreachable for 1 hour, 0 records are lost and all upload within
  10 minutes of recovery; a record of schema version n is read by the analysis script for version n+1 (one added
  field, old records still parse); a room killed with SIGKILL mid-match leaves a partial record marked incomplete.
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

## Open questions
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

### Recommendations that hinge on a source not read
| Recommendation | Source | Reason not read |
|---|---|---|
| R6-TEL-REC3 (what rates mean) | https://arxiv.org/pdf/2502.01250v2 | outside the allow list |
| R6-TEL-REC3 (shipped practice) | https://gdcvault.com/play/1024237/ | GDC Vault redirects to a maintenance page; video only |
| R6-TEL-REC2 (S3 against S1) | Cloudflare R2 data location docs | not fetched in this run |

## Assumptions
- **A-10.** A 20-minute 48-player match: about 10 gameplay events per second (R2's worked estimate uses the same rate)
  → 12,000 events × about 80 B as JSON ≈ 1 MB uncompressed; the timeline adds about 360 samples; compression is not
  estimated (no source).
- **A-11.** At the goal's average of 6 rooms (A-5) with 20-minute matches: about 13,000 matches a month (6 × 3 × 730),
  about 13 GB uncompressed a month at A-10's upper bound.

## Sources read
1. https://docs.rs/parquet/latest/parquet/ · parquet 60.1.0 · 28 October 2026 · official
2. docs/LOOP_V2.md §4, §8, §11 · project design doc (input, not a research source)

## Sources not read
- https://cloud.google.com/bigquery/pricing · content truncated to the title
- https://gdcvault.com/play/1024237/ · GDC Vault maintenance redirect; video only
- https://arxiv.org/pdf/2502.01250v2 · outside the allow list
- https://us.forums.blizzard.com/en/heroes/t/internal-data-used-for-balance-patches/33949 · outside the allow list
- postgresql.org, sqlite.org, duckdb.org · outside the allow list

## Cross-topic notes
- **R1:** damage and effect events must carry the source ability or weapon id (R1's pointer to R6) and the killer's
  unit id; the record's content hash is R1-DATA-REC6's.
- **R2:** the recorder reads the same tick-stamped events the clients get (R2-BUD-REC2); writing at Ended is
  R2-ROOM-REC1's mark (3).
- **game-designer (from the loop slice on):** owns the derived definitions in REC3 and the moment thresholds (OQ3).
- **test-engineer:** the statistics question (OQ1).
