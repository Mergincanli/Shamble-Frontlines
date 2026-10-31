# ADR-0006: Infrastructure and operations

**Status:** Proposed
**Date:** 2026-10-31
**Author:** infra-engineer (pass marks by test-engineer, plan step 4) · **Reviewed by:** (filled in at review) · **Signed off by Mustafa:** no

This record decides where SHAMBLE FRONTLINES v2 runs and how it is watched. The player build and every pull request's
preview link are served by Cloudflare Pages; previews that reach a game server sit behind a login from the first one.
Game servers run on Hetzner Cloud in one EU location, one room per process, on a named reference server that every
server pass mark is measured on. It sets the start and goal budgets (with the dates of the prices), the logs, metrics
and crash reports, where the balance-telemetry match records are stored and how they are analysed, and the data rules
for all of it. The security baseline stays open, with interim positions, until its own research before the first
online playtest. It decides this now because the netcode proof test, the foundation's preview links and every server
mark in ADR 0001 and ADR 0004 need a named machine, a host and a budget before they can start.

Citations give the research file, its part, the id and the lines at commit 9e7db28 (the topic files do not change in
this PR). Prices are as the research read them on 2026-10-28, in EUR or USD as published, excluding VAT
(r6-infrastructure.md, Header and Findings intro, lines 40–41 and 54); no exchange rate was read, so EUR and USD stay
apart. No account was created and no service configured in this research (lines 40–41): every account, payment,
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
  carries unreleased art is behind the access wall (part 2).
- **[The foundation](../research/GLOSSARY.md#the-foundation)** (GLOSSARY §8, brief §2): "CI gate v2, and private preview
  links"; it "ends when an empty scene runs through the full stack in the browser, with CI green and a preview link".

### Settled inputs this record applies (never reopened)
- **MODES-3 (a):** totals; the largest mode is 24v24, N = 48. Where an R6 mark gives 48 and 54, 48 holds
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
  measurement" (lines 255–256).
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
