# Claims ledger: ADR-0006 (infrastructure and operations), infra-engineer, 2026-10-31

One row per claim or number in `infra-engineer-adr0006-draft.md`. Lines are at commit 9e7db28. Tiers:
`research verified` (an R1 or R2 recommendation claim, or a finding the topic's own verifier section lists as checked:
r6 lines 1103–1123 and the second pass at r6 line 1170, which checked R6-HOST-F8 and F20; r5's spot-check rows, guide
`inputs/findings-checked.tsv`) · `research, source to check` (any other research finding, R3–R6 recommendation text,
assumption or open-question text) · `arithmetic` (shown) · `settled` (a STATUS code, GLOSSARY §8, a GLOSSARY entry the
advisor decided, or the plan's decision map, which binds the authors) · `design doc` (VISION_V2, LOOP_V2, LOOK_V2,
CLAUDE.md) · `proposed mark` (with its reason; "research's" means the research proposed it and the draft carries it).

Note for the verifier: `findings-checked.tsv` lists R6-HOST-F8 and R6-HOST-F20 as "no", but r6 line 1170 (the second
pass) lists both among its 19 confirmed; the topic's own verifier section is the authority (plan step 7), so they are
`research verified` here.

## Header and summary paragraph
| # | Claim (short) | Section | Citation | Tier |
|---|---|---|---|---|
| 1 | Player build and every preview served by Cloudflare Pages | summary | r6-infrastructure.md, Recommendations §1, R6-HOST-REC1, lines 619–644 | research, source to check |
| 2 | Previews that reach a game server sit behind a login from the first one | summary | PREVIEW-WALL (a), STATUS "Decisions pending" | settled |
| 3 | Game servers on Hetzner Cloud, one EU location, one room per process | summary | r6, R6-HOST-REC2, lines 645–671; r2-netcode.md, R2-ROOM-REC2, lines 2762–2782 | research, source to check (R6 rec); research verified (R2 rec) |
| 4 | The security baseline stays open with interim positions until its research before the first online playtest | summary | BASELINE-WHEN (b), STATUS "Decisions pending"; r6 lines 968–969 | settled |
| 5 | The netcode proof test, the foundation's previews and every server mark need a machine, a host and a budget | summary | GLOSSARY §8 (the foundation, the netcode proof test; decision map part j, J1); N20/N34 (map part c) | settled |
| 6 | Prices as read 2026-10-28, excluding VAT; EUR and USD kept apart, no exchange rate | header note | r6, Header, lines 40–41; Findings intro, line 54 | research, source to check |
| 7 | No account created, no service configured in the research | header note | r6, Header, lines 40–41 | research, source to check |
| 8 | Accounts, payments, secrets and repository settings are Mustafa's | header note | r6, R6-SEC-OQ1, lines 944–945; r5-toolchain-quality.md, Pointers, lines 2104–2106; CLAUDE.md "Never" | design doc |

## Context
| # | Claim (short) | Section | Citation | Tier |
|---|---|---|---|---|
| 9 | §6.6 quote "every match records the numbers that matter: who led, the swings, steals, and unit pick and win rates" | Context, serves | docs/VISION_V2.md §6.6 | design doc |
| 10 | §6.8 kill-cams, replays and highlights are [LATER] | Context, serves | VISION_V2 §6.8 | design doc |
| 11 | §6.10 accounts [LATER] | Context, serves | VISION_V2 §6.10 | design doc |
| 12 | §3/§4 count like LOOP_V2 under the session's trace reading | Context, serves | decision map part j, J3 | settled |
| 13 | §4 quotes: "Desktop browsers first: Chrome, Edge, Firefox and Safari"; "Matches feel fair up to about 100 ms of ping. Servers start in one EU region." | Context, serves | VISION_V2 §4 | design doc |
| 14 | §5 "Guest play first" | Context, serves | VISION_V2 §5 | design doc |
| 15 | LOOP_V2 §8 points sources; prototype records Service points through KVStore | Context, serves | docs/LOOP_V2.md §8 | design doc |
| 16 | LOOP_V2 §11 counters "cashouts, steals, deliveries, harvester kills" | Context, serves | LOOP_V2 §11 | design doc |
| 17 | LOOK_V2 §8 "v2 may use authored 3D models" | Context, serves | docs/LOOK_V2.md §8 | design doc |
| 18 | A preview with unreleased art is behind the access wall | Context, serves | r5, Recommendations, Question 6, R5-PREV-REC5, lines 1642–1650; GLOSSARY, the access wall (GLOSSARY.md:618–635) | research, source to check |
| 19 | The foundation: "CI gate v2, and private preview links"; "ends when an empty scene runs through the full stack in the browser, with CI green and a preview link" | Context, serves | GLOSSARY §8, the foundation (brief §2) | settled |
| 20 | MODES-3 (a): totals; largest mode 24v24, N = 48; 48 holds where a mark gives 48 and 54 | Context, settled | STATUS "Decisions pending"; carry-forward 3 | settled |
| 21 | TICK-30: 30 Hz baseline; 60 Hz only with a 60 Hz margin of at least 25 % | Context, settled | STATUS (TICK-30); GLOSSARY §8, the 60 Hz margin | settled |
| 22 | SLICE-ORDER and its three conditions | Context, settled | STATUS "Decisions pending" | settled |
| 23 | PREVIEW-WALL (a) quote "private from the first preview that reaches the test game server" | Context, settled | STATUS "Decisions pending" | settled |
| 24 | BASELINE-WHEN (b) quote | Context, settled | STATUS "Decisions pending" | settled |
| 25 | UPGRADE-SCOPE: tracks per unit and weapon type owned by the player | Context, settled | inputs/UPGRADE-SCOPE-mustafa-2026-10-31.md (STATUS once docs-keeper records it) | settled |
| 26 | SPIKE-SCOPE (a); R7-MARKS (a) | Context, settled | STATUS:187–188, :159 | settled |
| 27 | Pages: `_headers` sets headers | Context, finding 1 | r6, Findings §1, R6-HOST-F1, lines 60–65 | research verified |
| 28 | Pages: a link per branch, no cap on previews | Context, finding 1 | r6, R6-HOST-F3, lines 72–79 | research verified |
| 29 | Pages sends `noindex` on previews; access policy limits them | Context, finding 1 | r5, Findings, Question 6, R5-PREV-F12, lines 838–844 | research verified |
| 30 | "The latest deployment for a branch cannot be deleted" | Context, finding 1 | r5, R5-PREV-F13, lines 845–849 | research, source to check |
| 31 | Hetzner dedicated plans for "highly predictable resources", naming gaming servers | Context, finding 2 | r6, R6-HOST-F14, lines 144–149 | research verified |
| 32 | CCX23 4 dedicated vCPU, 16 GB, 86.49 EUR/month, 20 TB included | Context, finding 2 | r6, R6-HOST-F15, lines 150–159 | research verified |
| 33 | 2026-10-28: every EU plan "currently unavailable. Please check back later." | Context, finding 2 | r6, R6-HOST-F20, lines 180–182 (second pass, r6 line 1170) | research verified |
| 34 | 24v24 match-hour about 3.16 GB | Context, finding 3 | r6, Options §2, egress table, lines 441–446 | arithmetic (r6's, on R2's estimate) |
| 35 | 8 full rooms on one VM all month stay inside 20 TB (8 × 3.156 GB × 730 h ≈ 18.4 TB) | Context, finding 3 | r6, Options §2, lines 461–464 | arithmetic |
| 36 | A-2 "a density to measure"; "the 4-rooms-per-VM row (one per vCPU) is the case to budget unless R2-ROOM-REC2 marks 1 and 2 pass on it" | Context, finding 4 | r6, Options §2, assumption A-2, lines 516–522 | research, source to check (assumption) |
| 37 | Sentry EU chosen at creation "and can't be changed" | Context, finding 5 | r6, Findings §3, R6-OBS-F9, lines 318–326 | research verified |
| 38 | Sentry's browser SDK sends the IP by default | Context, finding 5 | r6, R6-OBS-F10, lines 327–331 | research, source to check |
| 39 | Takes/gives per draft (room process, lobby protocol 0001; CI workflows, deploy job, header sets, Sentry upload job, multiplayer preview delivery 0002; field data 0003; events and moments 0004; content hash, ids, bundle 0005; gives N34/I-15, N37, S4, S6/I-10, S12) | Context, takes and gives | decision map part a (ADR 0006 scope; seams S3, S4, S6, S12), part c (N34, N37), part d (I-10, I-15); critic ranks 20, 26, 34 (map part b XT table) | settled |
| 40 | Legacy has no server, host, domain or certificate; "v2's hosting starts from zero" | Context, legacy | r6, Findings §1, legacy facts, lines 207–208; LEGACY_INVENTORY.md row CD2 (:83) | research, source to check |
| 41 | Legacy 0.80–1.00 ms per JavaScript step at 48 soldiers on DM_Tri (rows TL15, CM4) | Context, legacy | r6, legacy facts, lines 211–215 | research, source to check (R7 row) |
| 42 | R2's 8 rooms per VM "not a measurement" | Context, legacy | r6, Costs legacy facts, lines 255–256 | research, source to check |
| 43 | Today's CI holds no secret, token read-only; preview deploy is the first job with a secret | Context, legacy | r5, Findings, Question 6, legacy facts KT8, lines 904–907 | research, source to check |
| 44 | Smoke test fails on any network request; previews and crash reporting need origins allowed or reporting off | Context, legacy | r6, legacy facts, lines 209–210 and 353–354 | research, source to check |
| 45 | TL13: stall % 13.0 / 5.7 / 7.3 against 3.4 / 16.1 / 6.5, the same mean | Context, legacy | r6, Findings §4, legacy facts, lines 380–383 | research, source to check (R7 row) |
| 46 | GE15: a grenade kill counts for the held gun; GC18: assists never counted | Context, legacy | r6, lines 384–388 | research, source to check (R7 rows) |
| 47 | The three.js CDN tag has no integrity hash and no CSP (row PG19; STATUS quote) | Context, legacy | LEGACY_INVENTORY.md row PG19 (:377); STATUS "Deferred from gate-check-5 reviews" item | research, source to check (R7 row) |
| 48 | Old ADR-001 lists hosting; drafts replace ADR-001..003; hosting decided here | Context, legacy | STATUS:198–199; decision map part j, J2 (ADR-001, CDN-SRI) | settled |

## Decision, parts 1–7
| # | Claim (short) | Section | Citation | Tier |
|---|---|---|---|---|
| 49 | The client host is Cloudflare Pages with Workers static assets the alternative taking the same `_headers` | Decision 1 | r6, R6-HOST-REC1, lines 619–621; GLOSSARY, the client host (decided by the advisor 2026-10-30, resolution 19), GLOSSARY.md:587–603 | settled |
| 50 | The build's `_headers` carries the production header set on production and the preview header set on previews | Decision 1 | GLOSSARY, the production header set and the preview header set, GLOSSARY.md:652–666; r5, R5-PREV-REC4, lines 1625–1640 (ADR 0002's) | research, source to check |
| 51 | COOP/COEP only on a preview built with the isolation flag on; the preview Mustafa checks built with it off (R5-XQ-OQ3 settled 2026-10-30) | Decision 1 | r5, Open questions, R5-XQ-OQ3, lines 1899–1910; GLOSSARY.md:636–651 | settled (glossary entry, 2026-10-30) |
| 52 | Content bundle served with an immutable cache header | Decision 1 | r6, R6-HOST-REC1's mark, lines 641–642 (R1-DATA-REC6) | proposed mark (the research's) |
| 53 | Every served file under 25 MiB, Pages' per-asset limit | Decision 1 | r6, R6-HOST-F3, lines 72–79 | research verified |
| 54 | Authored models included in that limit | Decision 1 | LOOK_V2 §8 with r6 R6-HOST-F3 | design doc |
| 55 | three.js bundled from npm by Vite and served from the client host: no CDN script tag; CSP waits for the baseline | Decision 1 | decision map part j, J2 (CDN-SRI); STATUS item quoted; row PG19 | settled |
| 56 | `_headers` sets custom headers (COOP and COEP by name inferred) | Decision 1, why | r6, R6-HOST-F1, F2, lines 60–71; R6-HOST-REC1 reason, lines 627–629 | research verified (F1); research, source to check (F2) |
| 57 | GitHub Pages cannot send those headers | Decision 1, why | r6, R6-HOST-F11, F12, lines 122–132 | research verified (F12); research, source to check (F11) |
| 58 | Pages meets every need of R5-PREV-REC2's list but deletion of a branch's latest deployment | Decision 1, why | r6, R6-HOST-REC1 reason, lines 630–633; r5, R5-PREV-REC2, lines 1605–1615 | research, source to check |
| 59 | 0 USD on Pages' Free plan (inferred from the plan's name) | Decision 1, cost | r6, Options §2, start scenario, line 476 | research, source to check |
| 60 | Workers Paid minimum 5 USD a month, only if Worker code is needed | Decision 1, cost | r6, R6-HOST-F5, lines 87–92; line 476 | research verified |
| 61 | Need: §6.1 (rooms reached from this client), §4 (desktop browsers first) | Decision 1 | r6, R6-HOST-REC1 "Serves", lines 637–638; VISION_V2 §6.1, §4 | design doc |
| 62 | Offline previews public but unlisted (`noindex`, branch-name links) unless they carry unreleased art | Decision 2 | r5, R5-PREV-REC5, lines 1642–1650 | research, source to check |
| 63 | Every multiplayer preview behind the access wall from the first one | Decision 2 | PREVIEW-WALL (a) | settled |
| 64 | The access wall: the host's access policy limited to Mustafa's Cloudflare account, or Cloudflare Access | Decision 2 | GLOSSARY, the access wall, GLOSSARY.md:618–635; r5, R5-PREV-F12, F14, lines 838–858 | research verified (F12, F14) |
| 65 | Mustafa logs in once per Brave session | Decision 2 | r5, R5-PREV-REC5, lines 1643–1650 | research, source to check |
| 66 | Pages' policy restricts "project deployments" as a whole | Decision 2 | r5, R5-PREV-F12, lines 838–844 ("Enabling the access policy will restrict viewing project deployments to your Cloudflare account.") | research verified |
| 67 | So from the first multiplayer preview every preview of the project sits behind the wall (interim) | Decision 2 | follows from #66; OQ-0006-4 | proposed mark (interim position: the one setting the research read meets PREVIEW-WALL (a)) |
| 68 | The policy covers previews, not the production domain (inferred) | Decision 2 | r6, R6-HOST-F4, lines 80–86 (inferred from the summary) | research, source to check |
| 69 | A preview holds no secrets | Decision 2, why | r6, R6-HOST-REC1, line 622 | research, source to check |
| 70 | Need: GLOSSARY §8 the foundation ("private preview links") only; LOOK_V2 §8 | Decision 2 | GLOSSARY §8; LOOK_V2 §8; map part j, J3 (0006's features for README) | settled |
| 71 | 24 h after close, link no longer serves (404 or the wall) or the case written down; weekly sweep 0 previews older than 7 days; artifacts 7 days, not 90 | Decision 3 | r5, R5-PREV-REC3, lines 1616–1624; R5-PREV-F9 (default 90 days), lines 815–819 | proposed mark (the research's); research, source to check (F9) |
| 72 | Policy is 0006's, jobs are 0002's (seam S3) | Decision 3 | decision map part a, seam S3 | settled |
| 73 | "The latest deployment for a branch cannot be deleted" | Decision 3 | r5, R5-PREV-F13, lines 845–849 | research, source to check |
| 74 | PREV-DELETE in the preview hosting check; a failure sends the host back to Workers | Decision 3 | r6, Shared open questions, PREV-DELETE, lines 970–976 | settled (PREV-DELETE is a spike item, GLOSSARY decision codes) |
| 75 | A closed PR's room binary gone within 24 h | Decision 3 | r6, R6-HOST-REC3's mark, lines 681–683 | proposed mark (the research's) |
| 76 | R5-PREV-REC3 serves no §6 item | Decision 3 | r5, line 1622 | research, source to check |
| 77 | Staging: one small shared VM, proposed CX33 (4 vCPU, 8 GB, 8.99 EUR/month), not one per PR | Decision 4 | r6, R6-HOST-REC3, lines 672–673; R6-HOST-F15, lines 150–159 | research verified (price); research, source to check (rec) |
| 78 | Its supervisor starts the room binary a preview names; production refuses preview origins; a preview never connects to production | Decision 4 | r6, R6-HOST-REC3 and its mark, lines 672–683 | research, source to check |
| 79 | PREVIEW-WALL (a) ends the wait at r6 line 675; previews use it from slice 3, behind the wall | Decision 4 | r6 line 675 (superseded by PREVIEW-WALL (a), carry-forward 3); SLICE-ORDER (slice 3 online) | settled |
| 80 | Binary and hash delivery is 0002's with 0001's hash endpoint (critic rank 20) | Decision 4 | decision map part b, XT rank 20 (owner 0002); r6, R6-XT-OQ4, lines 1009–1014 | settled |
| 81 | The same VM runs Prometheus and the batch analysis at the start | Decision 4 | r6, R6-OBS-REC2, lines 739–741; R6-TEL-REC3 / option N1, lines 597–598 | research, source to check |
| 82 | A VM per PR bills at least an hour each time; previews need only a server speaking the PR's protocol version | Decision 4, why | r6, R6-HOST-REC3 reason, lines 676–677; R6-HOST-F17, lines 164–168 | research verified (F17) |
| 83 | Need: §6.1 (rooms, tested per PR) | Decision 4 | r6, line 680 | design doc |
| 84 | One room per process (R2-ROOM-REC2, 0001's) under a supervisor per VM; no load balancer; the lobby hands out host and port; own IPv4 and IPv6 per VM | Decision 5 | r6, R6-HOST-REC2, lines 645–649; r2, R2-ROOM-REC2, lines 2762–2764 | research verified (R2 rec); research, source to check (R6 rec) |
| 85 | Firewall: one UDP range for WebTransport, one TCP range for the WebSocket fallback, nothing else | Decision 5 | r6, R6-HOST-REC2, lines 648–649 (R2-TRN-REC3) | research, source to check |
| 86 | Plus the one rule the metrics path may need | Decision 5 | r6, R6-OBS-F4, lines 279–284 (scrape or push) | proposed mark (a scrape needs an inbound rule; checked in spike 2) |
| 87 | Hetzner's firewall is stateful, drops what no rule allows, costs nothing | Decision 5 | r6, R6-HOST-F18, lines 169–174 | research, source to check |
| 88 | VMs stay up; billing rounds up to the hour | Decision 5 | r6, R6-HOST-REC2, lines 649–650; R6-HOST-F17, lines 164–168 | research verified (F17) |
| 89 | x86-64 (dedicated plans are AMD) answers R1-DET-OQ7; Arm after R1-DET-REC6's matrix includes it | Decision 5 | r6, R6-HOST-REC2, lines 650–652; r1-simulation-core.md, R1-DET-OQ7, lines 2701–2703 | research, source to check |
| 90 | The reference server: CCX23 (4 dedicated vCPU, 16 GB, x86-64 AMD), CX33 (4 vCPU, 8 GB) measured beside it | Decision 5 | r6, R6-HOST-REC2, lines 645–647; GLOSSARY, the reference server, GLOSSARY.md:225–243 | research, source to check (rec); research verified (specs, R6-HOST-F15) |
| 91 | It answers R2-XU-OQ9 | Decision 5 | r2-netcode.md, R2-XU-OQ9, lines 3197–3202 | research, source to check (OQ text) |
| 92 | Every native server mark measured on one vCPU of it | Decision 5 | GLOSSARY, the measuring unit: one vCPU, GLOSSARY.md:244–264; map N20 | settled (glossary) |
| 93 | A working pick, not settled: "currently unavailable" on 2026-10-28 | Decision 5 | r6, R6-HOST-REC2, lines 652–655; R6-HOST-F20, lines 180–182 | research verified (F20) |
| 94 | Mustafa checks capacity in the console with his own account before the proof test | Decision 5 | r6, R6-HOST-OQ1, lines 852–855 ("when Mustafa has an account (not created by agents)") | research, source to check |
| 95 | Location nbg1 or hel1; fsn1 only if offered; picked by round-trip measurement | Decision 5 | r6, Options §1, Q4, lines 429–432; R6-HOST-F13, lines 136–143 | research verified (F13); research, source to check (Q4) |
| 96 | Dedicated vCPUs sold for predictable load; fair tick needs it; 20 TB included | Decision 5, why | r6, R6-HOST-REC2 reason, lines 656–658 | research, source to check |
| 97 | Shared CX33 is 9.6 times cheaper (8.99 against 86.49 EUR) | Decision 5, why | r6, lines 658–659 (86.49 ÷ 8.99 = 9.62) | arithmetic |
| 98 | R6-HOST-OQ3 answered: no UDP load balancer while the lobby hands out host and port | Decision 5 | r6, R6-HOST-OQ3, lines 859–861 | research, source to check |
| 99 | Need: §6.1, §6.5, §6.6, §6.8 and §4 | Decision 5 | r6, R6-HOST-REC2 "Serves", lines 665–666 | design doc |
| 100 | The lobby handles join tokens, guest identity, room codes, the room directory, and the crash report endpoint | Decision 6 | r6, R6-XT-OQ1, lines 991–993; R6-OBS-REC3 (C2 to "our own lobby endpoint"), lines 756–757; r2, R2-ROOM-OQ9, lines 3104–3105 | research, source to check |
| 101 | "the infrastructure ADR and the cost budget leave out a required service" | Decision 6 | r6, R6-XT-OQ1, lines 991–997 | research, source to check |
| 102 | Lobby protocol and token format 0001's (R2-ROOM-REC3, REC8); crate 0002's | Decision 6 | decision map part a (0001 item 8), seam S4, interface I-11 | settled |
| 103 | R2's two forms: "Same binary as the supervisor or a separate service" | Decision 6 | r2, R2-ROOM-OQ9, lines 3104–3105 | research, source to check (OQ text) |
| 104 | The lobby's store is not researched (a SQL database not read) | Decision 6 | r6, Options §4, Q3, S4, line 593 | research, source to check |
| 105 | Second option needs one more VM, for example a CX33 at 8.99 EUR (placeholder) | Decision 6 | r6, R6-HOST-F15, lines 150–159 | proposed mark (placeholder line so the budget names the cost; not a researched sizing) |
| 106 | Part 8's totals carry no lobby line | Decision 6 | r6, start and goal tables, lines 471–494 (no lobby row) | research, source to check |
| 107 | Signing keys are a baseline item; the lobby's HTTPS endpoint needs its own certificate, not covered by R6-HOST-REC4 (not researched) | Decision 6 | r6, R6-SEC-OQ1, line 938; R6-HOST-REC4, line 684 ("per game-server hostname") | research, source to check |
| 108 | A later login links to the same guest id (keeps §6.10 possible) | Decision 6 | r2, R2-ROOM-REC8, lines 2868–2875 | research verified (R2 rec) |
| 109 | Production: a certificate per hostname, through ACME (RFC 8555) | Decision 7 | r6, R6-HOST-REC4, lines 684–691; R6-HOST-F24, lines 198–204; r2, R2-TRN-REC5, lines 2704–2705 | research verified (R2 rec; F24) |
| 110 | The wildcard (C2): one job renews it and copies it to every VM, one private key on every VM; an option the baseline weighs | Decision 7 | r6, R6-HOST-REC4, lines 685–689; Options Q3 C2, lines 422–425; BASELINE-WHEN (b) | settled (BASELINE-WHEN (b)) |
| 111 | http-01 needs TCP 80 on each VM; dns-01 per name needs a DNS API key (a secret) | Decision 7 | r6, R6-HOST-F24, lines 198–204; Options Q3, lines 421–424 | research verified (F24) |
| 112 | The challenge and the lifetimes are OQ-0006-5 | Decision 7 | r6, R6-HOST-OQ6, lines 869–874 | research, source to check |
| 113 | Dev and preview: self-signed ECDSA P-256, at most 14 days, made at start, pinned via `serverCertificateHashes` | Decision 7 | r2, R2-TRN-REC5, lines 2700–2703; map N30 | research verified (R2 rec) |
| 114 | No production pinning: VMs stay warm, not "short-lived ephemeral servers" | Decision 7 | r6, Options Q3 C3, lines 426–427; r2, R2-TRN-OQ4, lines 3050–3054 | research, source to check |
| 115 | The proof test runs on dev certificates; the production form is needed from the first online playtest | Decision 7 | decision map part g (proof test subsumes REC5's development certificates; R6-HOST-REC4 "before the first online playtest, with the security baseline") | settled |
| 116 | Expiry alert under 14 days | Decision 7 | r6, R6-HOST-REC4's mark, lines 697–699 | proposed mark (the research's) |
| 117 | CF-4: four superseded sentences r2:2704–2706, r6:47, r6:941–943, r6:1065–1066 and their corrected text | Decision 7 | STATUS carry-forward 4; decision map part f, CF-4; r6 lines 684–691 (REC4 holds) | settled |
| 118 | A certificate per hostname puts no shared private key on the VMs; the wildcard's gain "a copy, not an issuance" | Decision 7, why | r6, R6-HOST-REC4 reason, lines 692–694 | research, source to check |
| 119 | Need: §6.1 (private matches on real servers), §4 (four browsers need a trusted certificate per hostname) | Decision 7 | r6, R6-HOST-REC4 "Serves", line 696 | design doc |

## Decision, parts 8–14
| # | Claim (short) | Section | Citation | Tier |
|---|---|---|---|---|
| 120 | Prices read 2026-10-28, excluding VAT | Decision 8 | r6, Findings intro, line 54; Header, lines 40–41 | research, source to check |
| 121 | A-4: up to 2 rooms at once, 18 humans each, about 4 hours a day, servers on all month | Decision 8, start | r6, Options §2, start scenario, lines 467–469; A-4, line 524 | research, source to check (assumption) |
| 122 | Lean start: CX33 8.99 + CX33 8.99 = 17.98 EUR, about 18 EUR, 0 USD | Decision 8, start | r6, lines 471–480 (CX33 price R6-HOST-F15, lines 150–159) | arithmetic |
| 123 | Reference start: CCX23 86.49 + CX33 8.99 + Object Storage 6.49 = 101.97 EUR, about 102 EUR + 5 USD (Workers Paid only if code) | Decision 8, start | r6, lines 471–480; R6-HOST-F15; R6-COST-F5, lines 241–246; R6-HOST-F5, lines 87–92 | arithmetic |
| 124 | Start egress about 137 GB (2 × 0.57 GB × 4 h × 30), inside 20 TB | Decision 8, start | r6, line 475 | arithmetic |
| 125 | A-5: peak 20 rooms of 48 humans, average 6 rooms; fleet = peak plus one VM for draining | Decision 8, goal | r6, lines 482–483; A-5, line 525 | research, source to check (assumption) |
| 126 | Goal at 8 rooms per VM: about 361 EUR (fixed CCX23 fleet of 4 VMs: 345.96 + 8.99 + 6.49 = 361.44) | Decision 8, goal | r6, goal table, lines 485–494 | arithmetic |
| 127 | Goal K2: about 238 EUR (222.88 + 8.99 + 6.49 = 238.36) | Decision 8, goal | r6, lines 487 and 494 | arithmetic |
| 128 | Each + 19–24 USD (Grafana Pro 19 USD + client hosting 0–5 USD) + Sentry's paid plan | Decision 8, goal | r6, lines 490–494; R6-COST-F2, lines 225–228 | arithmetic |
| 129 | At 4 rooms per VM, "the case to budget" until R2-ROOM-REC2 marks 1 and 2 pass | Decision 8, goal | r6, A-2, lines 516–522 | research, source to check (assumption) |
| 130 | Fixed fleet at 4 rooms per VM: ceil(20 ÷ 4) + 1 = 6 CCX23; 6 × 86.49 = 518.94 EUR | Decision 8, goal | arithmetic on A-5 (r6 line 525, the sizing rule at lines 482–483) and R6-HOST-F15 (86.49) | arithmetic |
| 131 | With staging and records: 518.94 + 8.99 + 6.49 = 534.42 EUR, about 534 EUR + 19–24 USD + Sentry | Decision 8, goal | arithmetic on #130, R6-HOST-F15, R6-COST-F5 | arithmetic |
| 132 | R6-COST-REC1 changed: goal stated at both densities until the proof test measures rooms per VM | Decision 8, goal | r6, R6-COST-REC1, lines 703–705; R6-COST-OQ1, lines 880–882 | proposed mark (reason: A-2's own caveat, r6 lines 516–522) |
| 133 | R2-ROOM-REC2 mark 2 is 0001's test | Decision 8, goal | decision map part c, N32; seam S4 | settled |
| 134 | Egress at 4 rooms: 4 × 3.156 GB × 730 h ≈ 9.2 TB per VM, under 20 TB | Decision 8, goal | arithmetic on r6 lines 461–463 (3.156 GB per room-hour; 730 h) | arithmetic |
| 135 | K1 first; K2 only once concurrency data shows a peak; K3 only if the shared measurement passes | Decision 8, fleet | r6, R6-COST-REC1, lines 703–705 | research, source to check |
| 136 | Each extra room of peak capacity: 86.49 ÷ 8 ≈ 10.81 EUR a month at 8 per VM; 21.62 EUR at 4 | Decision 8, fleet | r6, scaling rule, lines 499–500 | arithmetic |
| 137 | Every record carries room-hours and bytes sent; monthly bill check against 20 TB per VM | Decision 8, cost per match | r6, R6-COST-REC2, lines 718–725 | research, source to check |
| 138 | Mustafa reads the invoice in his own account | Decision 8, cost per match | r6, R6-COST-OQ4, OQ5, lines 889–892 (invoice decides); account rule as #8 | design doc |
| 139 | Standard GitHub-hosted runners "free and unlimited on public repositories" | Decision 8, CI | r5, Findings, Question 5, R5-CI-F17, lines 673–678 | research verified |
| 140 | Free 10 GB cache | Decision 8, CI | r5, R5-CI-F5, lines 583–592 | research verified |
| 141 | The repository is public | Decision 8, CI | r5, Question 6 legacy facts, lines 913–914 | research, source to check |
| 142 | GPU and macOS runner prices not read | Decision 8, CI | r5, R5-CI-OQ5, lines 1835–1837; R5-TEST-OQ4, line 1809 | research, source to check |
| 143 | Self-hosted runners "a cost and security question" | Decision 8, CI | r3-rendering.md, R3-MEASURE-OQ2, lines 1384–1385 | research, source to check |
| 144 | Reference-machine runs by hand until Mustafa decides | Decision 8, CI | R3-MEASURE-OQ2's "or are run by hand" option, r3 lines 1384–1385; OQ-0006-7 | proposed mark (interim: the unpriced options wait for Mustafa) |
| 145 | Compute dominates and is flat per VM; egress zero on Hetzner inside 20 TB, about 0.063 USD per room-hour on a per-GB host | Decision 8, why | r6, R6-COST-REC1 reason, lines 706–709 | research, source to check |
| 146 | Need: §6.1 (§3 sizes) and §6.6 (cost line); CI runner costs: the foundation only | Decision 8 | r6 lines 713 and 723; map part j, J3 | design doc |
| 147 | `tracing` JSON lines to stdout per room process; supervisor adds VM and room ids and ships them; no tokens, guest secrets or raw ids | Decision 9 | r6, R6-OBS-REC1, lines 729–731 | research, source to check |
| 148 | Salted hashes where an id is needed (R2-ROOM-REC3 mark 3) | Decision 9 | r2, R2-ROOM-REC3 mark 3, lines 2807–2809 | proposed mark (R2's, verified as an R2 recommendation) |
| 149 | `metrics` + `metrics-exporter-prometheus`, scraped by Prometheus on the staging VM; Grafana Cloud EU once its path is read | Decision 9 | r6, R6-OBS-REC2, lines 739–741 | research, source to check |
| 150 | Labels bounded (VM, room slot 0–7, mode id, transport), never a player or match id | Decision 9 | r6, R6-OBS-REC2, lines 740–741 | research, source to check |
| 151 | The first metrics list (server tick time histogram, overruns, dropped time; per-phase time; sessions by transport; bytes and datagrams; budget overruns; relevance churn; joins refused; room starts, ends, crashes; memory) | Decision 9 | r6, R6-OBS-REC2, lines 741–744; interface I-12 (map part d) | research, source to check |
| 152 | Until a store is read, logs stay on the VM under the 14-day cap | Decision 9 | none in the research | proposed mark (reason: R6-OBS-REC1 names no store; part 11's cap applies) |
| 153 | T1 local timing; no distributed tracing (T2 Beta in Rust, no §6 item) | Decision 9 | r6, Options §3, Q3, lines 545–549; R6-OBS-F3, lines 275–278 | research, source to check |
| 154 | A scrape needs an inbound port; the exporter can push to a push gateway | Decision 9 | r6, R6-OBS-F4, lines 279–284 | research, source to check (F4; "inbound port" inferred from "exposing an HTTP endpoint that can be scraped") |
| 155 | Series near A-8's 3,600 for 4 VMs, inside Grafana Free's 10k | Decision 9, why | r6, A-8, lines 565–567; R6-COST-F1, lines 219–224 | arithmetic (r6's A-8); research verified (F1) |
| 156 | Logs about 0.8 GB a month at the goal's average, under the free 50 GB | Decision 9, why | r6, A-9, lines 568–569; R6-COST-F1 | arithmetic (r6's A-9) |
| 157 | Prometheus keeps 15 days, not replicated: metrics lost with the VM, not the game | Decision 9, why | r6, R6-OBS-F5, lines 285–290; M1 row, line 541 | research verified |
| 158 | Need: §6.1 and §4 ("Matches feel fair up to about 100 ms of ping") | Decision 9 | r6 lines 734–735 and 749 | design doc |
| 159 | Sentry browser SDK + `wasmIntegration`, build ids from `wasm-split`, debug files uploaded by CI and never shipped, EU organization, `userInfo` off | Decision 10 | r6, R6-OBS-REC3, lines 754–756 | research, source to check |
| 160 | The upload job is 0002's (rank 34) | Decision 10 | decision map part b, XT rank 34 (owner 0002) | settled |
| 161 | Reporting API `crash` reports to our own endpoint on the lobby | Decision 10 | r6, R6-OBS-REC3, lines 756–757; R6-OBS-F8, lines 307–315 | research, source to check |
| 162 | Server panics to Sentry's Rust SDK; supervisor records every abnormal exit | Decision 10 | r6, R6-OBS-REC3, lines 756–757; R6-OBS-F6, lines 291–294 | research, source to check |
| 163 | EU location "can't be changed" | Decision 10 | r6, R6-OBS-F9, lines 318–326 | research verified |
| 164 | Mustafa creates the organization after reading Sentry's prices; upload token is his secret | Decision 10 | r6, R6-COST-OQ2, lines 883–886; pointers, line 1082 | research, source to check |
| 165 | Sentry calls its Wasm support "still developing" | Decision 10 | r6, R6-OBS-F7, lines 297–306 | research verified |
| 166 | Symbolication for wasm-bindgen Rust (R6-OBS-OQ3) and which browsers send `crash` reports (R6-OBS-OQ4) are spike marks | Decision 10 | r6, R6-OBS-OQ3, OQ4, lines 902–904 | research, source to check |
| 167 | In the boundary spike if the EU org exists, else at the first online playtest's milestone | Decision 10 | decision map part g (boundary spike: "R6-OBS-REC3 (a Wasm trap reaching Sentry, if Sentry is set up)"; later milestones list R6-OBS-REC3 before the first online playtest) | settled |
| 168 | The gate v2's smoke test switches reporting off or allows its endpoints | Decision 10 | r6, legacy facts TL9, lines 353–354; pointers, lines 1082–1083 | research, source to check |
| 169 | Field data: what is measured is 0003's (rank 26); stored under part 11's rules | Decision 10 | decision map part b (R6-OBS-OQ5, R6-XT-OQ5 owner 0003); interface I-18 | settled |
| 170 | Why: Wasm frames need debug files; tab crashes invisible to JS but reach an endpoint; EU chosen once; SDK sends IPs unless told not to | Decision 10, why | r6, R6-OBS-REC3 reason, lines 758–760 | research, source to check |
| 171 | Need: §4 (four browsers) and §6.1 (per-room crash reports) | Decision 10 | r6 lines 763–764 | design doc |
| 172 | Every store in the EU; no IP, token or guest secret in logs, reports or records | Decision 11 | r6, R6-OBS-REC4, lines 769–770; option P1, lines 559–561; R6-TEL-REC1 "Never in it", line 798 | research, source to check |
| 173 | Logs 14 days, metrics 15 days (proposed: the free tier's and Prometheus' defaults) | Decision 11 | r6, R6-OBS-REC4, lines 770 and 773–774; R6-COST-F1 (14 days), R6-OBS-F5 (15d) | proposed mark (the research's) |
| 174 | Records kept while their content hash is analysed (proposed default) | Decision 11 | r6, R6-TEL-OQ4, lines 917–919 | proposed mark (the research's) |
| 175 | Crash-report cap open: no page read shows whether Sentry lets an organization set retention | Decision 11 | r6, R6-OBS-REC4, lines 770–772 | research, source to check |
| 176 | Some Sentry data (user accounts, organization settings, access tokens) may be stored in the US whatever the choice | Decision 11 | r6, R6-OBS-F9, lines 322–326 | research verified |
| 177 | The legal side not researched; Mustafa decides before any public playtest | Decision 11 | r6, R6-OBS-REC4, line 772; R6-OBS-OQ1, lines 896–899 | research, source to check |
| 178 | EU choice one-way on Sentry and Grafana; OWASP: bounded retention, no personal data in logs | Decision 11, why | r6, R6-OBS-REC4 reason, lines 773–774; R6-OBS-F11, F12 | research verified (F9); research, source to check (F11, F12) |
| 179 | Need: no §6 item as a feature; condition for §6.1's and §6.6's stores | Decision 11 | r6 lines 777–778 | design doc |
| 180 | One record per match, inside the room (W2, R-a), with R6-TEL-REC1's header, timeline, events, per-slot section, moments, "never in it" | Decision 12 | r6, R6-TEL-REC1, lines 784–799 | research, source to check |
| 181 | Timeline every 10 s, 120 samples per team in a 20-minute match | Decision 12 | r6, R6-TEL-REC1, lines 789–790 (20 min × 60 ÷ 10 = 120) | arithmetic |
| 182 | Changed: per slot adds picks per track and level, offers shown, looted weapons, time per weapon type | Decision 12 | decision map part j, J5 (0006's row); no research finding (map part h, G13) | proposed mark (need: UPGRADE-SCOPE; the map assigns the fields here) |
| 183 | Their shape follows 0005's track data; carry slots, ammo for looted weapons and levelling speed stay Mustafa's open questions in 0005 | Decision 12 | inputs/UPGRADE-SCOPE-mustafa-2026-10-31.md ("Still open"); map part j, J5 | settled |
| 184 | 0004 emits events and recognizes moments; 0001's room writes at Ended before Closed; 0005 owns ids and content hash; thresholds are design (R6-TEL-OQ3) (seam S6) | Decision 12 | decision map part a, seam S6; r2, R2-ROOM-REC1, lines 2748–2761; r6, R6-TEL-OQ3, lines 915–916 | settled |
| 185 | Storage: compressed JSON lines in Hetzner Object Storage in the servers' location, uploaded by the supervisor after exit, schema version per record, Parquet only when needed, stub record for a crashed room | Decision 12 | r6, R6-TEL-REC2, lines 812–820 | research, source to check |
| 186 | Analysis in batches on the staging VM, grouped by content hash and mode id, sample size beside every rate; the derived numbers | Decision 12 | r6, R6-TEL-REC3, lines 828–834 | research, source to check |
| 187 | No live balance dashboard (N2) | Decision 12 | r6, option N2, lines 599–600 | research, source to check |
| 188 | Whether bot and private matches count is Mustafa's (R6-TEL-OQ2); the record tags both | Decision 12 | r6, R6-TEL-OQ2, lines 913–914; map part b (DESIGN) | research, source to check |
| 189 | Kill-cams, replays, highlights would need a full replay (W3), not built; events and recorded matches keep them possible | Decision 12 | r6, option W3, lines 577–578; VISION_V2 §6.8 [LATER]; map part e1 (§6.8 row) | research, source to check |
| 190 | Why: each §6.6 number and §6.8 moment is a function of timeline and events; GE15, GC18; writing at Ended reuses R2-ROOM-REC1 | Decision 12, why | r6, R6-TEL-REC1 reason, lines 800–802 | research, source to check |
| 191 | About 13 GB a month uncompressed at the goal, inside 1 TB for 6.49 EUR | Decision 12, why | r6, A-11, lines 606–607; R6-TEL-REC2 reason, lines 815–816; R6-COST-F5 | arithmetic (r6's A-11); research verified (F5) |
| 192 | Need: §6.6 and §6.8, with §6.5 and §6.1; the track fields: UPGRADE-SCOPE only | Decision 12 | r6 line 806; map part j, J3 | design doc |
| 193 | The security baseline's subject "content security policy, dependency audits, anti-cheat basics"; researched by security-auditor before the first online playtest in its own fresh session | Decision 13 | r6, R6-SEC-OQ1, line 926; BASELINE-WHEN (b), r6 lines 968–969 | settled |
| 194 | The items waiting on it (R6-SEC-OQ1's list; critic rank 18) | Decision 13 | r6, R6-SEC-OQ1, lines 926–945; R6-XT-OQ3, lines 1004–1008 | research, source to check |
| 195 | B1: the advisories check runs on a schedule and on every PR that changes `Cargo.lock`, outside the PR gate; keeps R2-TRN-REC1's floors current | Decision 13 | r5, R5-GATE-REC1, lines 1671–1674; r6, R6-SEC-OQ1, lines 929–932; GLOSSARY, the advisories check, GLOSSARY.md:691–706 | settled (BASELINE-WHEN (b) interim position) |
| 196 | B1: whether an advisory blocks a PR and who owns the policy wait | Decision 13 | r5, R5-GATE-OQ4, lines 1873–1876; r1, R1-RR-OQ7, lines 2955–2961 | research, source to check |
| 197 | B2: certificate per hostname (interim) | Decision 13 | BASELINE-WHEN (b); r6, R6-HOST-REC4 | settled |
| 198 | B3: no CSP written yet; no third-party script; the header sets take the CSP once written | Decision 13 | GLOSSARY, the production header set (GLOSSARY.md:652–657: "R6's content security policy once the security baseline writes it"); map part j, J2 (CDN-SRI) | settled |
| 199 | B4: R2-ROOM-REC8's HttpOnly, Secure, SameSite=Strict cookie on the lobby's origin; SEC-GUEST recommends it; Mustafa and the Advisor decide | Decision 13 | r2, R2-ROOM-OQ11, lines 3112–3116; R2-ROOM-REC8, lines 2868–2871 | research verified (R2 rec) |
| 200 | B5: signing keys only in the lobby host's environment, added by Mustafa | Decision 13 | r6, R6-SEC-OQ1, lines 938 and 944–945 | proposed mark (interim: secrets in the environment only, the research's rule at r6 lines 944–945) |
| 201 | B6: CLAUDE.md's rule ("Dynamic or player-provided text: `textContent`, never `innerHTML`") holds by review | Decision 13 | CLAUDE.md "Code rules"; r5, R5-TSLINT-OQ5, lines 1776–1778 | design doc |
| 202 | B7: anti-cheat basics listed (view-time offset and aim log; Origin-less requests tagged; reader-client and byte-audit tests) | Decision 13 | r6, R6-SEC-OQ1, lines 935–937 | research, source to check |
| 203 | B8: R1-DATA-REC7's allow list and fuzz corpus; COOP as an XS-Leaks guard | Decision 13 | r6, R6-SEC-OQ1, lines 939–940 | research, source to check |
| 204 | B8: SEC-MDL-2 is 0005's | Decision 13 | STATUS:257; decision map part j, J2 (SEC-MDL-2 → 0005, 0006 cites) | settled |
| 205 | B8: production non-isolated at the start | Decision 13 | GLOSSARY, preview isolation and the isolation flag, GLOSSARY.md:636–651 | settled (glossary entry) |
| 206 | B9: the Sentry upload token "a token: Mustafa's secret" | Decision 13 | r6, pointers, line 1082 | research, source to check |
| 207 | B9: each token held only by the job that needs it, in a GitHub environment, as R5-PREV-REC1 does for the deploy token | Decision 13 | r5, R5-PREV-REC1, lines 1588–1604 | proposed mark (for the Sentry and staging tokens: the same rule R5-PREV-REC1 sets for the host token) |
| 208 | B10: the staging VM runs PR-built native code (R6-XT-OQ4's words) beside Prometheus; they share it because forks get no preview | Decision 13 | r6, R6-XT-OQ4, lines 1009–1011 ("PR-built native code on a VM"); r5, R5-PREV-REC1, lines 1588–1591 | proposed mark (interim: listed for the baseline; R6 puts both on the staging VM) |
| 209 | B11: no self-hosted runner | Decision 13 | r3, R3-MEASURE-OQ2, lines 1384–1385; part 8 | proposed mark (interim, until OQ-0006-7) |
| 210 | Previews settled by PREVIEW-WALL (a) | Decision 13 | PREVIEW-WALL (a) | settled |
| 211 | Secrets only in GitHub environments or the VM's environment, never in the repository; Mustafa adds each; agents never add them | Decision 13 | r6, R6-SEC-OQ1, lines 944–945; r5, Pointers, lines 2104–2106; CLAUDE.md "Never" (commit secrets) | design doc |
| 212 | The deploy token is an account-scoped "Cloudflare Pages, Edit" API token; narrowing to one project not read | Decision 13 | r5, R5-PREV-F10, lines 823–830 | research, source to check |
| 213 | Held only by the deploy job's environment `preview` | Decision 13 | r5, R5-PREV-REC1, lines 1588–1593 | research, source to check |
| 214 | Need: no §6 item; the condition for the first online playtest; the deploy token: the foundation only | Decision 13 | BASELINE-WHEN (b); map part j, J3 | settled |
| 215 | The preview hosting check on a test project in Mustafa's Cloudflare account, before the foundation's preview step | Decision 14 | decision map part g (the preview hosting check: depends on "Mustafa's Cloudflare account and a test project"; place "before the foundation's preview step") | settled |
| 216 | The proof test carries this record's operations marks (rooms per VM, egress, the bill, logging cost, metrics, firewall) | Decision 14 | decision map part a (ADR 0006 item 7), part g (the proof test subsumes R6-HOST-REC2, R6-COST-REC1, R6-OBS-REC1, REC2) | settled |

## Options considered
| # | Claim (short) | Section | Citation | Tier |
|---|---|---|---|---|
| 217 | Every option comes from the research's Options sections; "from zero": legacy has nothing to migrate | Options, intro | r6, legacy facts, lines 207–208; row CD2 | research, source to check |
| 218 | Q1 A: wrangler-action deploy with an account-scoped Pages Edit token | Options Q1 | r5, R5-PREV-F10, lines 823–830 | research, source to check |
| 219 | Q1 A: 0 USD Free (inferred); Workers Paid 5 USD minimum | Options Q1 | as #59, #60 | as #59, #60 |
| 220 | Q1 A: unlimited previews; 20,000 files per site (Free); 25 MiB per file; 500 builds a month on Free | Options Q1 | r6, R6-HOST-F3, lines 72–79 | research verified |
| 221 | Q1 A: previews public by default, access policy, `noindex` by default; latest deployment cannot be deleted | Options Q1 | r5, R5-PREV-F12, F13, lines 838–849 | research verified (F12); research, source to check (F13) |
| 222 | Q1 A: `_headers` does not apply to code-generated responses | Options Q1 | r6, R6-HOST-F2, lines 66–71 | research, source to check |
| 223 | Q1 B Netlify: path P5 builds on Netlify's servers and "breaks 'the preview is the tested build'" | Options Q1 | r5, Options, Question 6, P5, line 1118 | research, source to check |
| 224 | Q1 B: Netlify prices not read (outside the allow list) | Options Q1 | r6, R6-HOST-F7, lines 105–106 | research, source to check |
| 225 | Q1 B: password option, `noindex` (Deploy Previews); sensitive variables kept from fork builds | Options Q1 | r5, R5-PREV-F15, F16, lines 859–872 | research, source to check |
| 226 | Q1 B: one header set for every deploy context | Options Q1 | r6, R6-HOST-F7, lines 100–106 | research, source to check |
| 227 | Q1 C Vercel: `vercel deploy --prebuilt` with three secrets | Options Q1 | r5, R5-PREV-F17, lines 873–878 | research, source to check |
| 228 | Q1 C: Hobby "non-commercial, personal use only", 100 GB transfer; Pro 20 USD per seat per month | Options Q1 | r6, R6-HOST-F9, lines 111–115 | research, source to check |
| 229 | Q1 C: Standard Protection needs a Vercel account; password protection 20 USD a month per project on Pro | Options Q1 | r6, R6-HOST-F10, lines 116–121 | research, source to check |
| 230 | Q1 C: headers per path in `vercel.json` | Options Q1 | r6, R6-HOST-F8, lines 107–110 | research verified (second pass, r6 line 1170) |
| 231 | Q1 D GitHub Pages: 1 GB per site, 100 GB a month soft, not for an online business; no per-PR site (inferred); cannot set COOP/COEP | Options Q1 | r6, R6-HOST-F11, F12, lines 122–132; r5, hosts table, line 1128 | research verified (F12); research, source to check (F11, r5 inferred) |
| 232 | Q1 E own web server: "Every preview, cache and certificate is ours to run"; generic work no §6 item asks for | Options Q1 | r6, Options Q1 row E, line 409 | research, source to check |
| 233 | Q2: public previews to staging settled by PREVIEW-WALL (a) | Options Q2 | PREVIEW-WALL (a) | settled |
| 234 | Q2 (i): one project setting; (ii), (iii) not researched; Access read for Workers previews only | Options Q2 | r5, R5-PREV-F12, F14, lines 838–858 | research verified (F12, F14) |
| 235 | Q3 G1: CCX23 86.49 EUR a month (0.1386 an hour), 20 TB, 1 EUR per extra TB | Options Q3 | r6, R6-HOST-F15, F16, lines 150–163 | research verified |
| 236 | Q3 G1: "highly predictable resources"; firewall filters UDP, stateful, default drop | Options Q3 | r6, R6-HOST-F14, F18, lines 144–149 and 169–174 | research verified (F14); research, source to check (F18) |
| 237 | Q3 G1: EU plans "currently unavailable" 2026-10-28; single provider | Options Q3 | r6, R6-HOST-F20, lines 180–182; Q2 table, line 415 | research verified |
| 238 | Q3 G2: CX33 8.99 EUR a month (0.0144 an hour); "designed for variable usage patterns rather than sustained high workloads" | Options Q3 | r6, R6-HOST-F15, F14 | research verified |
| 239 | Q3 G2: noisy neighbours could break the server tick budget | Options Q3 | r6, Q2 table, line 416 | research, source to check |
| 240 | Q3 G3 Fly.io: performance-2x 66 USD a month + 0.02 USD per GB; UDP not read | Options Q3 | r6, R6-HOST-F22, lines 186–192 | research verified |
| 241 | Q3 G3: egress about 369 USD a month per loaded host (18,431 GB × 0.02 USD) | Options Q3 | r6, lines 461–464 | arithmetic |
| 242 | Q3 G4 GameLift: c6a.large 0.099 USD an hour on demand, 0.03 Spot, Ohio; bandwidth free on gen 6+; EU rates not read; SDK lock-in not read | Options Q3 | r6, R6-HOST-F23, lines 193–197; Q2 table, line 418 | research, source to check |
| 243 | Q3: Hetzner root servers, Netcup, OVHcloud, Scaleway not researched | Options Q3 | r6, R6-HOST-OQ1, OQ4, lines 852–855 and 862–865 | research, source to check |
| 244 | Q4 K1: "nothing to automate"; pays for idle hours | Options Q4 | r6, K1, line 502 | research, source to check |
| 245 | Q4 K1 cost: 4 × 86.49 = 345.96 EUR at 8 per VM; 6 × 86.49 = 518.94 EUR at 4 | Options Q4 | r6, line 487; #130 | arithmetic |
| 246 | Q4 K2: 222.88 EUR of game VMs, about 36 % less; needs the API, a draining supervisor, an unmeasured start-up time; hourly round-up | Options Q4 | r6, lines 487 and 503–505 | arithmetic (r6's); research, source to check |
| 247 | Q4 K2: API credentials (a secret; inferred) | Options Q4 | inferred from "automation through the provider's API", r6 line 504 | proposed mark (inference, labelled in the draft) |
| 248 | Q4 K3: about 9.6 times cheaper; only if shared hosts pass R2-ROOM-REC2's marks at several times of day | Options Q4 | r6, K3, lines 506–507 | research, source to check |
| 249 | Q4 K4: Fly's egress alone about 276 USD a month at the goal's average | Options Q4 | r6, line 496 (13,823 GB × 0.02 USD) | arithmetic |
| 250 | Q5 C1: http-01, TCP 80 on every game VM | Options Q5 | r6, C1, line 421; R6-HOST-F24 | research verified (F24) |
| 251 | Q5 dns-01 per name named in R6-HOST-REC4 | Options Q5 | r6, R6-HOST-REC4, line 685 | research, source to check |
| 252 | Q5 C2: one renewal job, copied to every VM; one key on every VM; departs from R2-TRN-REC5; weighed by the baseline | Options Q5 | r6, C2, lines 422–425; BASELINE-WHEN (b) | settled |
| 253 | Q5 C3: production only "if R6 picks short-lived ephemeral servers"; VMs warm, so not | Options Q5 | r6, C3, lines 426–427 | research, source to check |
| 254 | Q5 tls-alpn-01: RFC 8737 "This connection MUST use TCP port 443." (not researched further) | Options Q5 | r6, R6-HOST-OQ6, lines 871–874 | research, source to check |
| 255 | Q6: nbg1, hel1; fsn1 only if offered ("NBG1 HEL1"); no latency source read | Options Q6 | r6, Q4, lines 429–432; R6-HOST-F13, lines 136–143 | research verified (F13) |
| 256 | Q7 L1: about 0.8 GB of logs a month; a formatted message does not allocate by default | Options Q7 | r6, A-9, lines 568–569; R6-OBS-F1, lines 262–269 | research verified (F1); arithmetic (A-9) |
| 257 | Q7 L1: OWASP exclusions | Options Q7 | r6, R6-OBS-F12, lines 338–344 | research, source to check |
| 258 | Q7 L2: OpenTelemetry logs Stable; nothing in §6 needs its cross-service view | Options Q7 | r6, L2, lines 535–536; R6-OBS-F3, lines 275–278 | research, source to check |
| 259 | Q7 M1: 0 extra; about 0.6 GB of disk at A-8; 15 days default, 1–2 bytes per sample; not replicated | Options Q7 | r6, M1, line 541; A-8, lines 565–567; R6-OBS-F5, lines 285–290 | research verified (F5); arithmetic (A-8) |
| 260 | Q7 M2: free under 10k series; Pro 19 USD plus usage; region fixed at creation; ingestion path not read | Options Q7 | r6, R6-COST-F1, F2, lines 219–228; R6-OBS-F11, lines 332–337; R6-OBS-OQ2, lines 900–901 | research verified (F1, F2); research, source to check (F11) |
| 261 | Q7 M3: more dependencies in the room binary | Options Q7 | r6, M3, line 543 | research, source to check |
| 262 | Q7 T1 serves R2-BUD-REC6's separate measurements; T2 Beta, "a generic feature no §6 item asks for" | Options Q7 | r6, T1, T2, lines 546–549 | research, source to check |
| 263 | Q8 Sentry (C1): free tier then per error; base prices not read; Wasm "still developing"; EU at creation; IP off by setting | Options Q8 | r6, R6-COST-F3, lines 229–235; R6-OBS-F7, F9, F10, lines 297–331 | research verified (F7, F9); research, source to check (COST-F3, F10) |
| 264 | Q8 Reporting API (C2): tab crashes JS cannot see; support varies | Options Q8 | r6, R6-OBS-F8, lines 307–315 | research, source to check |
| 265 | Q8 own handler (C3): Wasm traps as raw offsets, symbolication ours | Options Q8 | r6, C3, line 556 | research, source to check |
| 266 | Q9 P1 and P2 as the research gives them | Options Q9 | r6, Q5, lines 558–562 | research, source to check |
| 267 | Q10 W1 answers none of "who led", "swings", "steals" | Options Q10 | r6, W1, line 574 | research, source to check |
| 268 | Q10 W2: about 1 MB uncompressed per 24v24 match (A-10) | Options Q10 | r6, A-10, lines 603–605 | arithmetic (r6's: 12,000 events × about 80 B) |
| 269 | Q10 W2: under 1 % of server tick p99 | Options Q10 | r6, R6-TEL-REC1's mark, lines 809–810 | proposed mark (the research's) |
| 270 | Q10 W3: "orders of magnitude larger", a §6.8 [LATER] feature | Options Q10 | r6, W3, lines 577–578 | research, source to check |
| 271 | Q10 R-a: no new path through the sim; R-b: a collector outage loses data mid-match | Options Q10 | r6, Q2, lines 581–585 | research, source to check |
| 272 | Q10 S1: 6.49 EUR a month with 1 TB; about 13 GB a month at the goal | Options Q10 | r6, R6-COST-F5, lines 241–246; A-11, lines 606–607 | research verified (F5); arithmetic (A-11) |
| 273 | Q10 S3: about 1 USD a month after the free 10 GB; region not read | Options Q10 | r6, S3, line 592; R6-COST-F4, lines 236–240 | research, source to check |
| 274 | Q10 S4 not read; S5 BigQuery truncated, "Generic; not needed at this volume" | Options Q10 | r6, lines 593–594 | research, source to check |
| 275 | Q10 N1: a month of goal records in under 10 minutes | Options Q10 | r6, R6-TEL-REC3's mark, lines 840–842 | proposed mark (the research's) |
| 276 | Q10 N2: no §6 item asks for live balance data | Options Q10 | r6, N2, lines 599–600 | research, source to check |
| 277 | Q11: R2 names two forms; R6 neither hosts nor costs the lobby; neither weighed | Options Q11 | r2, R2-ROOM-OQ9, lines 3104–3105; r6, R6-XT-OQ1, lines 991–997 | research, source to check |
| 278 | Q12: settled by BASELINE-WHEN (b) | Options Q12 | STATUS | settled |
| 279 | Q13 standard runners: free and unlimited; cache free to 10 GB, more needs Pro, Team or Enterprise; ubuntu-24.04 4 CPUs, 16 GB, no GPU | Options Q13 | r5, R5-CI-F17, F5, lines 673–678 and 583–592 | research verified |
| 280 | Q13: today's CI already runs on them (row KT8) | Options Q13 | LEGACY_INVENTORY.md row KT8 (:417) | research, source to check (R7 row) |
| 281 | Q13: WebGPU not covered in CI; GPU and macOS prices not read | Options Q13 | r5, R5-CI-OQ5, lines 1835–1837; R5-TEST-OQ4, line 1809 | research, source to check |
| 282 | Q13: self-hosted reference machines "a cost and security question" | Options Q13 | r3, R3-MEASURE-OQ2, lines 1384–1385 | research, source to check |

## Trade-off analysis and Consequences
| # | Claim (short) | Section | Citation | Tier |
|---|---|---|---|---|
| 283 | Pages: the one host read meeting every need but one, at 0 USD (as #58, #59) | Trade-off | r6, R6-HOST-REC1, lines 627–634 | research, source to check |
| 284 | Against: the latest deployment cannot be deleted; the check settles it or sends the host to Workers | Trade-off | r5, R5-PREV-F13; r6, PREV-DELETE, lines 970–976 | research, source to check |
| 285 | Wall on every preview: one setting; against, offline previews lose their open link; one login per session | Trade-off | as #66, #67, #65 | as #66, #67, #65 |
| 286 | Dedicated vCPUs and included 20 TB make egress zero even at full load; shared CX33 9.6 times cheaper | Trade-off | as #31, #32, #35, #97 | as cited |
| 287 | Against Hetzner: "currently unavailable"; single provider | Trade-off | as #33; r6 Q2 table, line 415 | research verified |
| 288 | K2 would save about 36 % of the goal's game-VM cost | Trade-off | r6, line 503 | arithmetic (r6's) |
| 289 | Two densities: at the server tick budget's ceiling, 50 % of one vCPU per room, two rooms fill a vCPU before transport runtime time | Trade-off | r6, A-2, lines 518–521; GLOSSARY, the server tick budget, GLOSSARY.md:167–188 | settled (glossary, resolution 1); research, source to check (A-2) |
| 290 | Certificates: per hostname puts no shared key; each new VM needs an issuance and TCP 80 or a DNS key; wildcard's gain is the baseline's | Trade-off | as #109–#111, #118 | as cited |
| 291 | Tools free or cheap at the start; Prometheus one unreplicated store; Sentry still developing, prices unread, EU permanent | Trade-off | as #157, #163, #165; r6 R6-COST-F3 | as cited |
| 292 | Telemetry: every §6.6 number a function of timeline and events, no new path through the sim; crashed room leaves only a stub; track fields rest on no research | Trade-off | r6, lines 575–583; R6-TEL-REC2, lines 818–820; #182 | research, source to check |
| 293 | No player reaches a game server before the first online playtest except through a multiplayer preview behind the wall | Trade-off | SLICE-ORDER (online in slice 3); PREVIEW-WALL (a); BASELINE-WHEN (b) | settled |
| 294 | Provider down: Cloudflare, no new client or preview loads; Hetzner, no online match, local play with bots still runs; object storage, records wait on disk; Sentry or Grafana, reports and metrics lost for that time | Trade-off | inferred from parts 1, 5, 9, 10, 12; SLICE-ORDER condition 1; r6, R6-TEL-REC2, lines 817–818 | proposed mark (inference from the decision's parts, labelled "inferred, not measured" in the draft) |
| 295 | Easier: one named machine; previews from the foundation; cost per match a query; one versioned record per match | Consequences | as #90, #19, #137, #185–#186 | as cited |
| 296 | Harder: Mustafa's accounts with Cloudflare and Hetzner before the spikes, later Sentry and Grafana; logins; certificate issuance path; the lobby unbudgeted | Consequences | as #8, #65, #112, #106 | as cited |
| 297 | Revisit-when list (OQ-0006-1, PREV-DELETE, rooms per VM, K2/K3 triggers, the baseline, UDP load balancer, isolation, data protection, Grafana and Sentry reads) | Consequences | as the parts and OQs cited; r6, R6-HOST-OQ3, lines 859–861 | as cited |
| 298 | Hard constraint "file://": the v2 player build served over HTTPS from the client host; 0002 records the end of the double-click build | Consequences | CLAUDE.md "Hard constraints"; decision map part j, J2 (ADR-002 row) | settled |
| 299 | Hard constraint "Persistence only through KVStore": server-side stores added | Consequences | CLAUDE.md; parts 6, 12 | settled (this record, on acceptance) |
| 300 | Hard constraint "No external assets" ends under LOOK_V2 §8; files under 25 MiB | Consequences | LOOK_V2 §8; R6-HOST-F3 | design doc |
| 301 | Hard constraint "Three.js r128 UMD": no CDN tag; which three.js is 0003's | Consequences | map part j, J2 (ADR-003, CDN-SRI rows) | settled |
| 302 | Spikes exempt now under SPIKE-SCOPE (a); nothing in CLAUDE.md or `.claude/` changes in this PR | Consequences | STATUS:187–188; plan rule 11 | settled |
| 303 | Inventory: no mark changes; GP5, PG13, GP11, GE12 are look-brief-pending presentation rows of 0003 | Consequences | LEGACY_INVENTORY.md rows GP5 (:315), PG13 (:371), GP11 (:321), GE12 (:298) | research, source to check (R7 rows) |
| 304 | Inventory rows and their marks now: CD2 port to TypeScript (:83); CD3 drop (:84); PG19 drop (:377); KT8 redesign (:417); TL9 redesign (:394); TL13 redesign (:398); TL15 redesign (:400); GE15 redesign (:301); GC18 drop (:257) | Consequences | LEGACY_INVENTORY.md at those lines | research, source to check (R7 rows) |

## Open questions
| # | Claim (short) | Section | Citation | Tier |
|---|---|---|---|---|
| 305 | OQ-0006-1: fallbacks (Fly.io UDP page, GameLift EU prices and SDK, Hetzner root servers, Netcup, OVHcloud, Scaleway) unread; IPv4 under 1 EUR per VM either way | OQ-0006-1 | r6, R6-HOST-OQ1, OQ4, OQ7, lines 852–855, 862–865, 875–876 | research, source to check |
| 306 | OQ-0006-2: round-trip times against §4's "about 100 ms" | OQ-0006-2 | r6, R6-HOST-OQ2, lines 856–858; VISION_V2 §4 | research, source to check |
| 307 | OQ-0006-3: options from R2-ROOM-OQ9; store not researched; placeholder CX33 8.99 EUR; decider the Advisor with a lobby read, Mustafa for paid service | OQ-0006-3 | as #103–#105; r6, R6-XT-OQ1 | research, source to check; proposed mark (the decider and the read) |
| 308 | OQ-0006-4: options (i)–(iii); Mustafa decides after the check | OQ-0006-4 | as #66–#68, #234 | as cited |
| 309 | OQ-0006-5: http-01, dns-01, tls-alpn-01 (TCP 443), wildcard only if the baseline accepts; CA policy unread (Let's Encrypt outside the allow list) | OQ-0006-5 | r6, R6-HOST-F24, lines 198–204; R6-HOST-OQ6, lines 869–874; sources not read, line 1306 | research, source to check |
| 310 | OQ-0006-6: the baseline per item; guest secret decided by Mustafa and the Advisor, SEC-GUEST recommending the cookie | OQ-0006-6 | r6, R6-SEC-OQ1; r5, R5-GATE-OQ4, lines 1873–1876; R5-TSLINT-OQ5, lines 1776–1778; r1, R1-RR-OQ7, lines 2955–2961; r2, R2-ROOM-OQ11, lines 3112–3116 | research, source to check |
| 311 | OQ-0006-7: options (a)–(d); a cache above 10 GB needs a Pro, Team or Enterprise account | OQ-0006-7 | r5, R5-CI-F5, lines 583–592; R5-CI-OQ5; R5-TEST-OQ4; r3, R3-MEASURE-OQ2 | research verified (F5); research, source to check |
| 312 | OQ-0006-8: Sentry base prices on sentry.io (outside the allow list); Grafana Free under 10k series or Pro 19 USD plus usage | OQ-0006-8 | r6, R6-COST-F1, F2, F3, lines 219–235; R6-COST-OQ2, lines 883–886; R6-OBS-OQ2, lines 900–901 | research verified (F1, F2); research, source to check (F3, OQs) |
| 313 | OQ-0006-9: GDPR and EDPB outside the allow list; before any public playtest | OQ-0006-9 | r6, R6-OBS-OQ1, lines 896–899; R6-TEL-OQ4, lines 917–919 | research, source to check |
| 314 | OQ-0006-10: A-4 and A-5 are placeholders; VAT and invoice; traffic per VM | OQ-0006-10 | r6, R6-COST-OQ3, OQ4, OQ5, lines 887–892 | research, source to check |
| 315 | OQ-0006-11: confidence intervals per cell and a minimum count; test-engineer and game-designer | OQ-0006-11 | r6, R6-TEL-OQ1, OQ5, lines 910–912 and 920–922 | research, source to check |
| 316 | OQ-0006-12: CORP or a separate page under isolation; only if production isolation is adopted (R1-BND-REC2 mark 5) | OQ-0006-12 | r1, R1-BND-OQ5, lines 2734–2736; R1-XT-OQ17, lines 3105–3108 | research, source to check |

## What a spike must prove, Migration plan, Test plan, Rollback
| # | Claim (short) | Section | Citation | Tier |
|---|---|---|---|---|
| 317 | Every server mark at the 30 Hz baseline; the 60 Hz margin run is 0001's | Spike intro | TICK-30; map part c, N2–N3 (owner 0001) | settled |
| 318 | Spike 1 needs Mustafa's Cloudflare account, a test project, a static page with a `.wasm`; before the foundation's preview step | Spike 1 | decision map part g (the preview hosting check) | settled |
| 319 | Headers on 100 % of responses in a full page load, checked by a script | Spike 1 | r6, R6-HOST-REC1's mark, lines 639–641 | proposed mark (the research's) |
| 320 | `crossOriginIsolated` true with the flag on, false off, in Chrome, Edge, Firefox, Safari; and Brave | Spike 1 | r6, lines 640–641; r5, R5-PREV-REC4 mark 2, lines 1637–1639 | proposed mark (the research's) |
| 321 | Hashed bundle immutable, repeat visit fetches 0 bytes; largest file under 25 MiB, rechecked on Workers | Spike 1 | r6, lines 641–644 | proposed mark (the research's) |
| 322 | PREV-DELETE: 24 h after close, 404 or the wall, or the case written down; a failure sends the host to Workers | Spike 1 | r5, R5-PREV-REC3's mark, lines 1622–1624; r6, PREV-DELETE, lines 970–976 | proposed mark (the research's) |
| 323 | Whether a direct upload counts against 500 builds a month | Spike 1 | r6, lines 633–634 | research, source to check |
| 324 | Policy covers every preview and not production; one login per session, the link works after | Spike 1 | r6, R6-HOST-F4 (inferred); r5, R5-PREV-REC5's mark, lines 1648–1650 | proposed mark (the research's, plus the coverage check of OQ-0006-4) |
| 325 | Mustafa judges in Brave on Windows; test-engineer adds the checklist | Spike 1 | plan rule 12 | settled |
| 326 | R5-PREV-OQ5 and OQ6 ride along (0002's) | Spike 1 | decision map part g ("Decides: ... R5-PREV-OQ5, OQ6") | settled |
| 327 | Spike 2's precondition is OQ-0006-1 | Spike 2 | decision map part g (the proof test depends on "the reference server orderable (R6-HOST-OQ1)") | settled |
| 328 | 8 rooms of 48 (bots) as 8 processes, each p99 within 10 % of one room's, over 20 minutes, at 3 times of day on the shared host, on CCX23 and CX33 | Spike 2 | r6, R6-HOST-REC2's mark, lines 667–671; r2, R2-ROOM-REC2 mark 2, lines 2775–2782 | proposed mark (the research's; 48 under MODES-3 (a)) |
| 329 | Rooms per VM at 48, 36 and 18 players on both machines | Spike 2 | r6, R6-COST-REC1's mark, lines 714–717 | proposed mark (the research's; MODES-3 (a)'s sizes) |
| 330 | Counted as the most rooms for which R2-ROOM-REC2 marks 1 and 2 hold | Spike 2 | none in the research | proposed mark (reason: the research names the count but no rule; this makes the figure mean the same on both machines) |
| 331 | Both transports through the firewall from four browsers; a port with no rule dropped | Spike 2 | r6, R6-HOST-REC2's mark, lines 669–671 | proposed mark (the research's) |
| 332 | The metrics path works with only the rule it needs (negative test extended) | Spike 2 | R6-OBS-F4, r6 lines 279–284 | proposed mark (reason: part 5's firewall rule; the research's negative test extended) |
| 333 | Egress per 24v24 match-hour within ±20 % of 3.16 GB, interest off, and the figure with it on; the bill under 5 EUR | Spike 2 | r6, R6-COST-REC1's mark, lines 716–717 | proposed mark (the research's) |
| 334 | Logging: p99 within 2 % of logging off; 0 allocations per tick; 0 tokens, secrets, raw IPs in a 30-minute soak | Spike 2 | r6, R6-OBS-REC1's mark, lines 736–738 | proposed mark (the research's) |
| 335 | Metrics present after a 30-minute soak at N = 48; a stalled tick shows as one overrun within one scrape; series under 5,000 for 8 rooms | Spike 2 | r6, R6-OBS-REC2's mark, lines 750–753 | proposed mark (the research's; 48 under MODES-3 (a)) |
| 336 | Dev certificates: 100 % of 20 per browser pinned; rotation with 48 live sessions drops 0, new hash within one reload; 15-day certificate rejected | Spike 2 | r2, R2-TRN-REC5 marks 1–3, lines 2717–2722 | research verified (R2 rec); proposed mark (48 under MODES-3 (a)) |
| 337 | Optional Sentry marks: trap with Rust names in 4 of 4 browsers; no DWARF shipped; Chrome tab crash report; 0 IPs in 100 events | Spike, optional | r6, R6-OBS-REC3's mark, lines 765–768 | proposed mark (the research's) |
| 338 | Migration 0: Mustafa's accounts and console check before any spike | Migration | as #8, #94 | as cited |
| 339 | Migration 1: spikes before the foundation; the proof test before the foundation | Migration | SLICE-ORDER condition 3 | settled |
| 340 | Migration 2: the foundation's milestone quote | Migration | GLOSSARY §8, the foundation | settled |
| 341 | Migration 3: slices 1–2 need no game server; previews offline | Migration | SLICE-ORDER (slice 2 "played locally"); GLOSSARY, preview (GLOSSARY.md:562–571) | settled |
| 342 | Migration 4: slice 3's pre-playtest steps (baseline, lobby, Sentry, data rules); staging and first multiplayer preview; production VM; logs and metrics | Migration | BASELINE-WHEN (b); parts 2, 4, 6, 8–11; decision map part g, later milestones | settled |
| 343 | Migration 5: the match record in slice 4 | Migration | decision map part g, later milestones (R6-TEL-REC1–REC3, R6-COST-REC2 at slice 4) | settled |
| 344 | Migration 7: fleet sized by measured density; bill check | Migration | part 8; R6-COST-REC1, REC2 | research, source to check |
| 345 | Test: profiles P0–P4, B1 are 0001's | Test plan | decision map part c, N27 | settled |
| 346 | Test: cost mark, the sum within 5 % of interface counters | Test plan | r6, R6-COST-REC2's mark, lines 724–725 | proposed mark (the research's) |
| 347 | Test: deploy job's header check passes and fails when a header is removed (0002's) | Test plan | r5, R5-PREV-REC4 mark 1, lines 1636–1637 | proposed mark (the research's) |
| 348 | Test: staging marks (two builds at once; binary gone within 24 h; never to production) | Test plan | r6, R6-HOST-REC3's mark, lines 681–683 | proposed mark (the research's) |
| 349 | Test: production certificates (four browsers 20 of 20; renewal with 48 live sessions drops 0; alert under 14 days) | Test plan | r6, R6-HOST-REC4's mark, lines 697–699 | proposed mark (the research's) |
| 350 | Test: data map, 0 stores outside the EU; data past its cap gone a day after | Test plan | r6, R6-OBS-REC4's mark, lines 779–780 | proposed mark (the research's) |
| 351 | Test: telemetry marks (30 bot matches across 3 modes; 100 % before Closed; source ids resolve; points recomputed; under 1 %, 0 allocations; at most 1 MB; storage outage 1 h; schema n read by n + 1; SIGKILL stub; 3 matches reproduced; 13,000 records in under 10 minutes) | Test plan | r6, R6-TEL-REC1's mark, lines 807–811; R6-TEL-REC2's mark, lines 824–827; R6-TEL-REC3's mark, lines 840–842 | proposed mark (the research's; the largest mode at N = 48) |
| 352 | Rollback: Workers takes the same `_headers`; Netlify the named fallback, prices unread | Rollback | r6, R6-HOST-REC1, lines 619–626 | research, source to check |
| 353 | Rollback: VMs billed by the hour with a monthly cap; a removed VM stops costing from the next hour (inferred) | Rollback | r6, R6-HOST-F17, lines 164–168 | research verified (F17); the stop is an inference, labelled in the draft |
| 354 | Rollback: the same exporter feeds M1 or M2; Sentry's EU choice cannot be undone | Rollback | r6, M1, M2, lines 541–542; R6-OBS-F9 | research verified (F9); research, source to check (M1/M2) |
| 355 | Rollback: versioned records, old records still parse | Rollback | r6, R6-TEL-REC2's mark, lines 824–826 | proposed mark (the research's) |

## Counts
Computed by script over the tier column (see the notes file for the run).
