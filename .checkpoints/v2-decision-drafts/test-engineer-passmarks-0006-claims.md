# Claims ledger: ADR-0006's pass marks (test-engineer), 2026-11-01

A notes file beside `test-engineer-passmarks-0006.md`, in the plan's ledger shape (rule 9). It joins
`infra-engineer-adr0006-claims.md` [C15]; rows are numbered TE1, TE2 … so they never clash with that ledger's 1–355.
One row per claim or number in the pass-mark subsection and the spike entries. Lines are at 9e7db28 (the research
files are unchanged at the head). Short names: r1 = `docs/research/r1-simulation-core.md`, r2 = `r2-netcode.md`, r5 =
`r5-toolchain-quality.md`, r6 = `r6-infrastructure.md`, GL = `docs/research/GLOSSARY.md`, draft =
`infra-engineer-adr0006-draft.md`, d0001 = `netcode-architect-adr0001-draft.md`, map =
`engine-architect-decision-map.md`, TE-0003 = `test-engineer-passmarks-0003.md`.

Tiers (the plan's): `research verified` = an R1 or R2 recommendation claim, or a finding its topic's verifier checked
(`inputs/findings-checked.tsv` "yes", or r6 line 1170's second pass for R6-HOST-F8 and F20) · `research, source to
check` = any other finding, R3–R6 recommendation text, assumption or open question · `arithmetic` (shown) · `settled`
(a STATUS code, GLOSSARY, the plan, the map's assignment, or a draft's own text read as such) · `design doc` ·
`proposed` (a research mark carried as written: "proposed (R6's)", "(R5's)"; or the test-engineer's own, "proposed
(test-engineer)", with its reason). R6 states every one of its pass marks as proposed (r6 line 615).

Mark codes: PRE the "How to read the table" bullets · PM-n the table row · FAIL the failure rule · NOM the "no
measurable mark" lines · HC the hosting checklist (step n) · OC the operations checklist · SE the spike entries.

## How to read the table

| # | the claim or number (short) | where | citation (file, id, lines) | tier |
|---|---|---|---|---|
| TE1 | R6 states every pass mark and reference machine as proposed | PRE | r6, Recommendations intro, line 615 | research, source to check |
| TE2 | CF: a test project on Cloudflare Pages in Mustafa's account, a static page and a `.wasm` | PRE, SE | draft, What a spike must prove, spike 1, lines 717–719; map part g, row 947 | settled (the draft's and the map's) |
| TE3 | CF also holds a module worker and a content-bundle stand-in with a hashed name | PRE, SE | r6 R6-HOST-REC1 mark, lines 639–642 (worker responses; the content bundle) | proposed (test-engineer: the research mark reads worker responses and the bundle, so the test page needs both) |
| TE4 | Every build writes its commit id into its HTML as the build marker | PRE, PM-6 | none | proposed (test-engineer: "no longer serves the build" needs a way to tell this build from any other response) |
| TE5 | TR: the repository for the test pull requests; Mustafa names it and adds its `preview` environment and token | PRE, SE | draft header, lines 22–23 (accounts, secrets, repository settings are Mustafa's); r5 R5-PREV-REC1, lines 1586–1590 (environment `preview`) | settled (the draft's); proposed (R5's) |
| TE6 | RS: a CCX23, 4 dedicated vCPU, 16 GB, x86-64 AMD, in `nbg1` or `hel1` | PRE | r6 R6-HOST-REC2, lines 645–650; GL the reference server, 225–231; map N34; draft part 5, lines 186–193 | settled (map N34, GLOSSARY) |
| TE7 | SH: the shared CX33, 4 vCPU, 8 GB, beside it | PRE | r6 R6-HOST-REC2, 645–650; R6-HOST-F15, 150–159 | research verified (F15) |
| TE8 | LG: bot clients on a separate VM, never on the VM under test | PRE | none | proposed (test-engineer: bot clients on the VM under test spend the vCPUs the marks measure) |
| TE9 | LG is a CX33; a run is invalid if its CPU use passes 80 % | PRE | none | proposed (test-engineer: a saturated generator, not the server, would set the result) |
| TE10 | MV: a CX33 running Prometheus, standing in for the staging game server, which exists only from slice 3 | PRE | draft part 9 (Prometheus on the staging VM), lines 280–282; draft part 4 (from slice 3), lines 171–172; r6 R6-OBS-REC2, 739–741 | proposed (test-engineer: no staging VM before slice 3) |
| TE11 | ML and MAC are the client machines of ADR 0003's pass marks (ML the minimum laptop, Windows 11; MAC a macOS machine with Safari) | PRE | TE-0003, How to read the table; r3 R3-BUDGET-REC1, 1096–1102 | proposed (R3's machines; reused by test-engineer) |
| TE12 | No client budget row here: every client mark is a yes-or-no or a count | PRE | draft, What a spike must prove, lines 710–768 (no client budget mark) | settled (read of the draft) |
| TE13 | MP: Mustafa's PC, Brave on Windows, only what he judges or reads in his own accounts | PRE | CLAUDE.md (plays in Brave on Windows); plan rule 12, lines 142–147; draft header, lines 22–23 | design doc; settled |
| TE14 | WebdriverIO drives Chrome, Edge, Firefox and Safari (Vitest's browser mode lists them for its WebdriverIO provider) | PRE | r5 R5-TEST-F6, 443–448 | research verified |
| TE15 | Playwright cannot drive branded Safari or Firefox | PRE | r5 R5-TEST-F8, 454–462 | research verified |
| TE16 | WebdriverIO is the tool for the four branded browsers | PRE | r5 R5-TEST-F6 | proposed (test-engineer: the one researched driver of all four) |
| TE17 | Every tick's server tick time recorded from a monotonic clock into a preallocated buffer, written after the run | PRE | GL server tick time, 113–118 | proposed (test-engineer: exact percentiles; the recording costs the tick nothing) |
| TE18 | 20 measured minutes | PRE, PM-13 | r2 R2-ROOM-REC2 mark 1, 2775–2776 ("over 20 minutes") | research verified (R2 rec) |
| TE19 | 2 minutes of warm-up, not counted | PRE | none | proposed (test-engineer: bot joins and start-up are not steady state) |
| TE20 | 36,000 ticks = 20 × 60 × 30 | PRE | map N18 | arithmetic |
| TE21 | p99 = the 35,640th smallest tick (ceil(0.99 × 36,000)); max = the 36,000th; nearest rank | PRE | ceil(35,640.0) = 35,640; TE-0003's TE16 (the same definition) | arithmetic; the nearest-rank choice proposed (test-engineer: one definition, so results repeat) |
| TE22 | A time mark naming no statistic is a p99; p99s are never added | PRE | GL time statistics, 141–147 | settled (GLOSSARY) |
| TE23 | Every server mark at 30 Hz; none at 60 Hz; the 60 Hz margin is ADR 0001's run | PRE, NOM | TICK-30 (STATUS); plan, TICK-60-HEADROOM, lines 83–91; map J1, N3 owner 0001 | settled |
| TE24 | The load is ADR 0001's proof-test load; its stand-in for the stress scene as OQ-0001-2 sets it | PRE | d0001, What a spike must prove, lines 926–928, item 3 (OQ-0001-2), lines 965–967; map J6 (2), lines 1215–1219 | settled (the map's ruling; ADR 0001's text) |
| TE25 | N = 48 | PRE | MODES-3 (a); GL player count and the largest mode | settled |
| TE26 | One snapshot-receiving bot client per soldier | PRE | GL the stress scene, 425–428 (a snapshot-receiving client on every soldier for a server mark with the server-only share); d0001 budgets setup, 521–523 | settled (GLOSSARY) |
| TE27 | Bots as headless protocol clients | PRE | r5 R5-TEST-REC5, 1475–1478 | proposed (R5's) |
| TE28 | P2 = 100 ms round trip, ±20 ms jitter, 2 % loss; latency is round-trip time | PRE | r2, sync recommendations preface, 1987–1990 | proposed (R2's) |
| TE29 | The reading of "100 ms of simulated latency" is P2, stated by ADR 0001 | PRE | map J6 (3), 1220–1221; map N27, N54 | settled (the map's ruling) |
| TE30 | P2 injected per direction, half the round trip each way | PRE | r5 R5-TEST-REC4, 1455–1456 | proposed (R5's) |
| TE31 | The impairment runs on LG, never on the VM under test | PRE | none | proposed (test-engineer: it would spend the measured vCPUs) |
| TE32 | Real-path marks run with no injected profile; the path's round trip recorded | PRE | none | proposed (test-engineer: the firewall and certificates are tested on the path players use) |
| TE33 | Bracket B: one room alone first and last; the mean of the two p99s; valid within 5 % | PRE | none | proposed (test-engineer: half the 10 % mark, so drift alone cannot decide it) |
| TE34 | An invalid session is neither pass nor fail; its cause is found first | PRE, FAIL | r5 R5-TEST-REC6, 1490–1498 | proposed (R5's rule, applied to measurement) |
| TE35 | "3 times of day" on the shared host | PRE, PM-13 | r6 R6-HOST-REC2 mark, 668–669 | proposed (R6's) |
| TE36 | Three sessions at least 6 hours apart, one between 18:00 and 23:00 Amsterdam time | PRE | none | proposed (test-engineer: neighbours' load follows the day; the research gives no spacing) |
| TE37 | Results-log fields (host, type, location, CPU model, commit, UTC time, profile, browser version) | PRE | none | proposed (test-engineer) |
| TE38 | "a check without a failing case can't be shown to work" | PRE | r5 R5-GATE-REC3, 1727–1728 | proposed (R5's reason) |
| TE39 | Every planted defect and control is proposed | PRE | none | proposed (test-engineer) |

## The preview hosting check (PM-0006-1 to PM-0006-10)

| # | the claim or number (short) | where | citation (file, id, lines) | tier |
|---|---|---|---|---|
| TE40 | The setup recorded before the first run; checked again before PM-0006-6's 24 h reading | PM-1 | r6 R6-HOST-REC1 mark, 639 ("on one preview link"); draft spike 1, 717–719 | proposed (test-engineer) |
| TE41 | 100 % of HTML, JS, worker and Wasm responses in a full page load carry the configured headers, checked by a script | PM-2 | r6 R6-HOST-REC1 mark, 639–640 | proposed (R6's) |
| TE42 | `.wasm` served as `application/wasm` | PM-2 | r5 R5-PREV-F20, 896–900; GL header sets, 652–658 | research verified (F20) |
| TE43 | Previews carry `X-Robots-Tag: noindex`; production does not | PM-2 | GL header sets, 652–658 (the preview set is production's plus `noindex`); r5 R5-PREV-F12, 838–844 | settled (GLOSSARY); research verified (F12) |
| TE44 | Flag on: COOP `same-origin`, COEP `require-corp` | PM-2 | GL the isolation flag, 636–642 | settled (GLOSSARY) |
| TE45 | Production and the flag-off preview carry no COOP or COEP | PM-2 | r5 R5-PREV-REC4 mark 3, 1639–1640 | proposed (R5's) |
| TE46 | The CSP is not checked: it waits for the security baseline | PM-2 | draft part 13, item B3, line 366; BASELINE-WHEN (b) | settled |
| TE47 | Each deployment read on its branch alias and its hash URL | PM-2 | r6 R6-HOST-F4, 80–86 (a hash URL per deployment and a branch alias) | research, source to check (F4 unchecked); the method proposed (test-engineer) |
| TE48 | Decides R5-PREV-OQ6 ("Decided by REC4's header check on a real preview") | PM-2, SE | r5 R5-PREV-OQ6, 1860–1861; map part g, row 947 | research, source to check; settled (map) |
| TE49 | The test page lists every resource it and its worker loaded (Resource Timing); a cookie-less client fetches each | PM-2 | none (Resource Timing not researched) | proposed (test-engineer: sees worker requests without a driver's network hooks) |
| TE50 | Expected sets written by hand from GLOSSARY §5 | PM-2 | none | proposed (test-engineer: an expectation generated by the build shares its mistakes) |
| TE51 | 1 load per URL form and deployment (headers repeat) | PM-2 | none | proposed (test-engineer) |
| TE52 | Bites: COEP removed from the worker rule; `noindex` added to production | PM-2 | r5 R5-PREV-REC4 mark 1, 1637 (a removed header must fail the check) | proposed (test-engineer, after R5's mutation) |
| TE53 | `crossOriginIsolated` true with the flag on, false with it off, in Chrome, Edge, Firefox and Safari | PM-3 | r6 R6-HOST-REC1 mark, 640–641 | proposed (R6's) |
| TE54 | Brave too | PM-3, HC1–3 | r5 R5-PREV-REC4 mark 2, 1638–1639 | proposed (R5's) |
| TE55 | False on production (production is non-isolated) | PM-3 | GL the isolation flag, 636–642 | settled (GLOSSARY) |
| TE56 | Read in the page and in its worker | PM-3 | none | proposed (test-engineer: the worker's script is its own response with its own headers) |
| TE57 | 3 of 3 loads per browser and deployment | PM-3 | none | proposed (test-engineer: one load could hit a cached older deployment) |
| TE58 | The content bundle served with an immutable cache header; a repeat visit fetches 0 bytes of it | PM-4 | r6 R6-HOST-REC1 mark, 641–642 | proposed (R6's) |
| TE59 | Interface I-6 | PM-4 | draft part 1, lines 128–129 | settled |
| TE60 | `transferSize` 0 on a new navigation in the same profile (not a forced reload), Chrome, 3 repeats from fresh profiles | PM-4 | none | proposed (test-engineer) |
| TE61 | Bite: `Cache-Control: no-cache` makes the second visit revalidate, so its transfer is above 0 | PM-4 | none | proposed (test-engineer) |
| TE62 | Pages' per-asset limit: 25 MiB | PM-5 | r6 R6-HOST-F3, 72–79 | research verified |
| TE63 | 25 MiB = 26,214,400 bytes | PM-5 | 25 × 1,048,576 | arithmetic |
| TE64 | On Workers static assets, the mark is rechecked against Workers' own limits page | PM-5 | r6 R6-HOST-REC1 mark, 642–644 | proposed (R6's) |
| TE65 | N37 | PM-5 | map c3, N37 | settled |
| TE66 | Bite: a planted 26 MiB file is flagged; the host's reaction recorded | PM-5 | none | proposed (test-engineer) |
| TE67 | 24 h after a PR closes its preview URL no longer serves the build (HTTP 404, or the access wall), or the remaining case is written down with the reason | PM-6, FAIL | r5 R5-PREV-REC3 mark, 1622–1624; r6 PREV-DELETE, 970–976 | proposed (R5's) |
| TE68 | 24 h is a plain deadline, not a statistic | PM-6 | GL time statistics, 146–147 | settled (GLOSSARY) |
| TE69 | "The latest deployment for a branch cannot be deleted" | PM-6 | r5 R5-PREV-F13, 845–849 | research, source to check |
| TE70 | Every URL is probed: the branch alias and every deployment's hash URL | PM-6 | r6 R6-HOST-F4, 80–86; R5-PREV-F13 | proposed (test-engineer: the mark's "preview URL" is one of several, and the latest deployment's own URL may outlive the alias) |
| TE71 | Gone = no HTTP 200 with the build's marker | PM-6 | none | proposed (test-engineer) |
| TE72 | Run with the access policy off | PM-6, HC | none | proposed (test-engineer: with it on, every leftover link sits behind the wall and the mark could not fail) |
| TE73 | Off is the state of slices 1 and 2 | PM-6 | draft Migration plan step 3, 787–789 | settled (the draft's) |
| TE74 | The close job deletes each deployment with `wrangler pages deployment delete … --force` | PM-6 | r5 R5-PREV-REC3, 1616–1618; R5-PREV-F13 | proposed (R5's) |
| TE75 | Branch aliases are lowercased and `/` becomes `-` (`fix/api` → `fix-api`) | PM-6 | r5 R5-PREV-F12, 838–844 | research verified |
| TE76 | Cases (a) `cs/pd-one`, 1 deployment, closed unmerged, branch kept; (b) `cs/pd-three`, 3 deployments, merged, branch deleted; control (c) open; bite (d) close job off | PM-6 | CLAUDE.md (branches `cs/<slug>`) | proposed (test-engineer: the two ways a pull request ends, on our branch-name form) |
| TE77 | Hourly probes from the close to 24 h | PM-6 | none | proposed (test-engineer: records when each URL stopped serving) |
| TE78 | 1 run per case | PM-6 | GL time statistics (a one-off deadline) | proposed (test-engineer) |
| TE79 | Pages' Free plan: 500 builds a month | PM-7 | r6 R6-HOST-F3, 72–79 | research verified |
| TE80 | Whether a direct upload counts against them is for the spike | PM-7 | r6 R6-HOST-REC1, 633–634 | research, source to check |
| TE81 | The count read before and after 10 direct uploads, by Mustafa; counts if +10, not if +0 | PM-7, HC4 | none | proposed (test-engineer: the dashboard is Mustafa's account) |
| TE82 | The access policy protects previews only, not production `*.pages.dev` or custom domains | PM-8, HC7 | r6 R6-HOST-F4, 84–86 | research, source to check (inferred there) |
| TE83 | "Enabling the access policy will restrict viewing project deployments to your Cloudflare account." | PM-8 | r5 R5-PREV-F12, 838–844 | research verified |
| TE84 | At least 2 open branches and PM-0006-6's leftover URLs; cookie-less client; sweeps at 10 minutes and 1 hour; the mark reads the 1-hour sweep | PM-8 | none | proposed (test-engineer: a setting may take time to reach every edge) |
| TE85 | Control: with the policy off, the build on every open preview URL | PM-8 | none | proposed (test-engineer) |
| TE86 | Feeds OQ-0006-4 | PM-8, SE | draft OQ-0006-4, 626–636 | settled (the draft's) |
| TE87 | A preview link opens in Brave on Windows without a login; with the access wall, one login per session and the link works after | PM-9, HC1–3, HC5–6 | r5 R5-PREV-REC5 mark, 1648–1650 | proposed (R5's) |
| TE88 | People judge: Mustafa's checklist beside the measurable marks | PM-9 | plan rule 12, 142–147; draft spike 1, 733–734 | settled |
| TE89 | Whether GitHub shows the environment URL on the PR page is decided by the spike | PM-10 | r5 R5-PREV-OQ5, 1858–1859 | research, source to check |
| TE90 | The deploy job sets its environment's `url` to the preview link | PM-10 | r5 R5-PREV-REC1, 1589–1591 | proposed (R5's) |

## The operations marks inside the netcode proof test (PM-0006-11 to PM-0006-27)

| # | the claim or number (short) | where | citation (file, id, lines) | tier |
|---|---|---|---|---|
| TE91 | Hetzner's EU locations include `nbg1` and `hel1`; the plans' pages list "NBG1 HEL1" | PM-11 | r6 R6-HOST-F13, 136–143 | research verified |
| TE92 | On 2026-10-28 every EU plan showed "currently unavailable" | PM-11 | r6 R6-HOST-F20, 180–182 | research verified (r6 line 1170) |
| TE93 | OQ-0006-1 is the precondition | PM-11, SE | draft spike 2, line 739; map part g, row 941 ("the reference server orderable (R6-HOST-OQ1)") | settled |
| TE94 | Type, location, CPU model, vCPU count, memory and kernel recorded; the CPU model read at every session | PM-11 | none | proposed (test-engineer: no topic read whether a plan's CPU model stays fixed; a change would move every server number) |
| TE95 | Mustafa creates the VMs or the API token | PM-11 | draft header, 22–23 | settled |
| TE96 | Round trips from Mustafa's machine and from playtesters to `nbg1`, `hel1` (and `fsn1` if offered) decide the location | PM-12 | r6 R6-HOST-OQ2, 856–858; draft OQ-0006-2, 608–614 | research, source to check; settled (the draft's decider) |
| TE97 | Hetzner's firewall filters ICMP | PM-12 | r6 R6-HOST-F18, 169–174 | research, source to check |
| TE98 | `ping -n 100` per location; the mean, max and loss as Windows prints them | PM-12, OC1 | none | proposed (test-engineer: a tool on every Windows PC, no page and no certificate needed) |
| TE99 | One throwaway CX33 per location, ICMP allowed on these only, deleted after | PM-12 | none | proposed (test-engineer: keeps part 5's rule on the game VMs) |
| TE100 | A second run if two locations differ by under 5 ms | PM-12 | none | proposed (test-engineer) |
| TE101 | 8 rooms of 48 (bots) as 8 processes, each room's server tick p99 within 10 % of the single-room figure, over 20 minutes, 3 times of day, on CCX23 and CX33 | PM-13 | r6 R6-HOST-REC2 mark, 667–669; r2 R2-ROOM-REC2 mark 2, 2777 | proposed (R6's); research verified (R2 rec) |
| TE102 | 1.10 × the single-room figure | PM-13 | 1 + 10 % | arithmetic |
| TE103 | R2-ROOM-REC2 mark 1: p99 within the server tick budget over 20 minutes | PM-13 | r2 R2-ROOM-REC2, 2775–2776 | research verified (R2 rec) |
| TE104 | At 30 Hz: p99 at most 16.7 ms, max under 33.3 ms, 0 overruns | PM-13 | GL the server tick budget, 167–172; d0001 budgets table, line 527 | settled (GLOSSARY; TICK-30) |
| TE105 | 1000 / 30 = 33.3 ms; 50 % of it = 16.7 ms | PM-13 | arithmetic | arithmetic |
| TE106 | Every room start-up under 2 s on both machines (R6-HOST-REC2's mark; the draft's restatement omits it) | PM-13 | r6 R6-HOST-REC2 mark, 670; r2 R2-ROOM-REC2 mark 5, 2780 | proposed (R6's); research verified (R2 rec) |
| TE107 | The RS half of marks 1 and 5 is ADR 0001's | PM-13 | map part g, row 941 ("R2-ROOM-REC2 (marks 1, 3, 5, 6; marks 2 and 4 for rooms per VM)") | settled (map) |
| TE108 | One set of runs serves ADR 0001's R2-ROOM-REC2 marks 1 and 2 and this row | PM-13, SE | map N32 (0006 the figure, 0001 the test); seam S4 | settled |
| TE109 | No CPU pinning, recorded | PM-13 | none | proposed (test-engineer: as production would run them) |
| TE110 | 384 bot clients | PM-13 | 8 × 48 | arithmetic |
| TE111 | Each room process's CPU share of one vCPU recorded (it holds the transport runtime time) | PM-13 | GL transport runtime time, 130–135 | settled (GLOSSARY) |
| TE112 | 3 sessions on RS too, at the same times of day | PM-13 | none | proposed (test-engineer: the same schedule makes the two machines comparable) |
| TE113 | On SH an adoption mark: a fail or an invalid session keeps K1 and K3 out, as part 8 says | PM-13, FAIL | draft part 8, lines 260–261 ("K3 only if the shared-vCPU measurement passes"); r6 R6-COST-REC1, 703–706 | settled (the draft's) |
| TE114 | Bite: a busy-wait of 20 % of the single-room p99 in every tick of one room | PM-13 | none | proposed (test-engineer) |
| TE115 | A-2: "a density to measure"; 4 rooms per VM the case to budget | PM-13, PM-14 | r6 A-2, 516–522 | research, source to check (assumption) |
| TE116 | Rooms per VM measured on CCX23 and CX33 at 48, 36 and 18 players; the number replaces A-2 | PM-14 | r6 R6-COST-REC1 mark, 714–716 | proposed (R6's) |
| TE117 | The counting rule: the most rooms for which R2-ROOM-REC2 marks 1 and 2 hold | PM-14 | draft spike 2, 743–745 | proposed (the draft's) |
| TE118 | 48, 36, 18: the goal modes and the three-team start mode's upper end | PM-14 | r6, 714–715; GL the start modes and the goal modes, 315–320 | settled (MODES-3 (a)) |
| TE119 | The figure is the budget's density only if each room's sim step p99 is at least the sim core spike's figure; else "on the stand-in", both densities kept until slice 6 | PM-14, SE | map J6 (2), 1215–1219 (the 30 Hz marks may use a stand-in); draft Migration plan step 7, 799–801 (re-measured on the real sim) | proposed (test-engineer: a stand-in cheaper than the real sim overstates the count) |
| TE120 | The search: k = 4, 6, 8 … up by 2, the odd k between, down to 1 if 4 fails | PM-14 | r6 A-2, 516–522 (4 per VM, one per vCPU) | proposed (test-engineer) |
| TE121 | The figure is the lowest count of 3 sessions | PM-14 | none | proposed (test-engineer: a budget takes the worst case) |
| TE122 | Decides R6-COST-OQ1 | PM-14, SE | r6 R6-COST-OQ1, 880–882; draft Appendix A, line 914 | settled (the draft's disposition) |
| TE123 | WebTransport over UDP and WebSocket over TCP reach the VM through the firewall from all four browsers; a VM with no rule for a port drops it | PM-15 | r6 R6-HOST-REC2 mark, 669–671 | proposed (R6's) |
| TE124 | The firewall opens the two game ranges "plus nothing else"; the draft adds the metrics rule | PM-15 | r6 R6-HOST-REC2, 648–649; draft part 5, 180–182 | research, source to check; settled (the draft's) |
| TE125 | Hetzner's firewall is stateful and drops all inbound traffic no rule allows | PM-15 | r6 R6-HOST-F18, 169–174 | research, source to check |
| TE126 | A dropped TCP connect times out (never "refused"); a dropped UDP datagram gets no reply; a listener behind each probed port | PM-15 | none | proposed (test-engineer: a drop sends no answer; the listener lets the probe fail) |
| TE127 | 20 attempts per browser and transport | PM-15 | r2 R2-TRN-REC5 mark 1, 2718–2719 (20 attempts) | proposed (test-engineer, borrowing R2's count) |
| TE128 | The WebSocket fallback is `wss` only | PM-15 | r2 R2-TRN-REC3, 2655 | research verified (R2 rec) |
| TE129 | The WebSocket constructor takes only a URL and sub-protocols, so a self-signed certificate cannot be pinned for `wss` | PM-15 | r2 R2-ROOM-F12, 1494–1500 (signatures confirmed, r2 line 3707); R2-TRN-F14, 1047–1053 (pinning is the WebTransport constructor's option) | research verified (the signatures); the consequence inferred (test-engineer) |
| TE130 | The draft names no development certificate for `wss` | PM-15 | draft part 7, 220–222 (self-signed, pinned through `serverCertificateHashes`) | settled (read of the draft) |
| TE131 | Bite: a rule for the probed TCP port makes the probe connect | PM-15 | none | proposed (test-engineer) |
| TE132 | The metrics path works with only the rule it needs (the negative test extended) | PM-16 | draft spike 2, 748–749 | proposed (the draft's) |
| TE133 | The exporter can be scraped (an inbound port) or push to a gateway | PM-16 | r6 R6-OBS-F4, 279–284 | research, source to check |
| TE134 | The scrape rule names MV's address as its only source | PM-16 | none | proposed (test-engineer: the narrowest rule that serves a scrape) |
| TE135 | Egress per 24v24 match-hour within ±20 % of 3.16 GB with interest management off, and the figure with it on | PM-17 | r6 R6-COST-REC1 mark, 716–717 | proposed (R6's) |
| TE136 | 146.1 kbps × 48 = 7.01 Mbit/s; × 3,600 s ÷ 8 = 3.156 GB | PM-17 | r6 egress table, 438–446; r2 results row, 1775 | arithmetic (R6's, on R2's estimate) |
| TE137 | ±20 %: 2.52 to 3.79 GB | PM-17 | 3.156 × 0.8 = 2.525; 3.156 × 1.2 = 3.787 | arithmetic |
| TE138 | R2's E3 formula at 30 Hz | PM-17 | r2 worked bandwidth estimate, 1764–1766 | research, source to check (an estimate on assumptions) |
| TE139 | R2's counts: V = 15 moving vehicles, 32 projectiles alive, 0.44 spawns per player per second, every entity in view | PM-17 | r2, 1759–1762, 1765–1766 | research, source to check (assumptions) |
| TE140 | The reference is R2's formula at the counts that ran, the arithmetic shown in the results | PM-17 | r2, 1764–1766 | proposed (test-engineer: the proof test's load need not have R2's counts; with V = 0 the formula gives 112.3 kbps against 146.1, a 23 % gap, beyond the band) |
| TE141 | The room's bytes-sent metric agrees with the interface counter within 5 % | PM-17 | r6 R6-COST-REC2 mark, 724–725 (5 %) | proposed (test-engineer, borrowing R6's tolerance) |
| TE142 | The interest-on figure only if the proof test's sim has the interest filter | PM-17, FAIL | map part g, row 941 (R2-INT-REC1, REC4 "only if the bandwidth mark uses the interest filter") | settled (map) |
| TE143 | 48 bot clients over IPv4, all in view; bots stand in for humans | PM-17 | r6 A-1, 515; r6, 438–439 (IPv4, humans only) | research, source to check (assumption) |
| TE144 | A room-hour as three 20-minute matches, joins and full states included | PM-17 | r2 R2-ROOM-REC1, 2751 (LOOP_V2 §8: about 20 min) | proposed (test-engineer: a room-hour as a real room spends it) |
| TE145 | Hetzner bills only outgoing traffic, so the interface's sent bytes are the billed direction | PM-17 | r6 R6-HOST-F17, 164–168 | research verified |
| TE146 | Bite: snapshots sent twice per tick | PM-17 | none | proposed (test-engineer) |
| TE147 | The spike's own bill under 5 EUR | PM-18 | r6 R6-COST-REC1 mark, 717 | proposed (R6's) |
| TE148 | The bill covers every VM-hour and traffic block of the whole netcode proof test | PM-18 | r6, 717 ("the spike's own bill (a few hours of one or two VMs)") | proposed (test-engineer's reading) |
| TE149 | CCX23 0.1386 EUR an hour, CX33 0.0144 EUR an hour, excluding VAT | PM-18 | r6 R6-HOST-F15, 150–159 | research verified |
| TE150 | 5 ÷ 0.1386 = 36 CCX23-hours | PM-18 | 36.07 | arithmetic |
| TE151 | A planning check before any VM; a plan over 5 EUR goes back to the draft first | PM-18, SE | none | proposed (test-engineer: a bill found too high afterwards cannot be undone) |
| TE152 | Billing rounds up to the hour | PM-18 | r6 R6-HOST-F17, 164–168 | research verified |
| TE153 | Prices exclude VAT | PM-18 | r6 Header, 40–41 | research, source to check |
| TE154 | Mustafa reads the costs in his account | PM-18, OC2 | draft header, 22–23 | settled |
| TE155 | Logging at its production level keeps server tick p99 within 2 % of logging off | PM-19 | r6 R6-OBS-REC1 mark, 736–737 | proposed (R6's) |
| TE156 | 1.02 × | PM-19 | 1 + 2 % | arithmetic |
| TE157 | One room on one vCPU, no other room | PM-19 | GL the measuring unit, 244–249 | settled (GLOSSARY) |
| TE158 | 6 runs of 10 minutes alternating off and on, one recorded input replay; pooled p99s over 54,000 ticks | PM-19 | 3 × 10 × 60 × 30 = 54,000 | proposed (test-engineer: alternation cancels slow drift); arithmetic |
| TE159 | Valid only if each off-run p99 lies within 1 % of the pooled off p99 | PM-19 | none | proposed (test-engineer: half the mark; a 2 % difference may sit inside run-to-run noise) |
| TE160 | Bite: a 50 µs busy-wait in each logged event | PM-19 | none | proposed (test-engineer) |
| TE161 | 0 allocations per tick from logging on the hot path, counted | PM-20 | r6 R6-OBS-REC1 mark, 737 | proposed (R6's) |
| TE162 | `tracing`: a formatted message does not allocate by default | PM-20 | r6 R6-OBS-F1, 262–269 | research verified |
| TE163 | A counting `GlobalAlloc`; the same replay; equal allocation counts tick by tick | PM-20 | none | proposed (test-engineer: the sim's own allocations repeat, so any difference is logging's) |
| TE164 | Bite: a `format!` in a per-tick log call | PM-20 | none | proposed (test-engineer) |
| TE165 | A grep of a 30-minute soak's logs finds 0 tokens, secrets or raw IPs | PM-21 | r6 R6-OBS-REC1 mark, 737–738 | proposed (R6's) |
| TE166 | No token, guest secret or raw id in any log; salted hashes where an id is needed | PM-21 | r2 R2-ROOM-REC3 mark 3, 2807–2809 | research verified (R2 rec) |
| TE167 | Canary file on LG; an address pattern other than the VMs' own; bite: a planted line with an address and a token | PM-21 | none | proposed (test-engineer: the grep knows every secret the soak made, and a planted leak proves it finds one) |
| TE168 | Every metric listed is present after a 30-minute soak at N = 48 | PM-22 | r6 R6-OBS-REC2 mark, 750–751 | proposed (R6's; 48 under MODES-3 (a)) |
| TE169 | The first metrics: server tick time histogram, overruns, dropped time, per-phase time, sessions by transport, bytes and datagrams sent and dropped, per-client budget overruns, relevance churn, joins refused, room starts, ends and crashes, memory | PM-22 | r6 R6-OBS-REC2, 741–744; draft part 9, 283–286 | research, source to check |
| TE170 | The soak is 8 rooms; a bad token per room, a room killed at minute 20, a quarter of the bots on WebSocket; untriggered counters checked for presence only | PM-22 | none | proposed (test-engineer: a present but stuck counter passes a presence check) |
| TE171 | Bite: one metric left unregistered | PM-22 | none | proposed (test-engineer) |
| TE172 | A deliberately stalled tick shows as one overrun and its dropped time within one scrape | PM-23 | r6 R6-OBS-REC2 mark, 751–752 | proposed (R6's) |
| TE173 | Catch-up per wake is bounded; a longer stall drops time and records it; overruns and dropped intervals are metrics | PM-23 | r1 R1-TICK-REC4, 2078–2086 | research verified (R1 rec) |
| TE174 | A 1 s stall, longer than the bound; dropped time = the stall minus the bound ADR 0004 sets, within one tick period (33.3 ms) | PM-23 | r1 R1-TICK-REC4 (proposed bound 0.25 s, 2082–2083); map N16 (0004 states it) | arithmetic; the stall size proposed (test-engineer) |
| TE175 | Scrape interval 15 s, recorded; the default was not read | PM-23 | none | proposed (test-engineer) |
| TE176 | 3 stalls 5 minutes apart; control: 0 overruns before the first | PM-23 | GL the server tick budget, 167–172 (0 overruns) | proposed (test-engineer) |
| TE177 | The series count reported and under 5,000 for 8 rooms on one VM | PM-24 | r6 R6-OBS-REC2 mark, 752–753 | proposed (R6's) |
| TE178 | Labels bounded: VM, room slot 0–7, mode id, transport; never a player or match id | PM-24 | r6 R6-OBS-REC2, 740–741 | research, source to check |
| TE179 | Label names checked against that set (plus Prometheus' own target labels); bite: a planted `player` label | PM-24 | none | proposed (test-engineer: an unbounded label can stay under 5,000 series with 384 players) |
| TE180 | All four browsers connect with a pinned self-signed certificate: 100 % of 20 attempts each | PM-25 | r2 R2-TRN-REC5 mark 1, 2718–2719 | research verified (R2 rec) |
| TE181 | ECDSA P-256, valid at most 14 days, made by the server at start, pinned through `serverCertificateHashes` | PM-25 | r2 R2-TRN-REC5, 2700–2703; R2-TRN-F13, 1033–1040 | research verified |
| TE182 | N30, owner 0006 | PM-25 | map c2, N30 | settled |
| TE183 | Rotation with 48 live sessions: 0 dropped; new connections succeed with the new hash within one reload | PM-26 | r2 R2-TRN-REC5 mark 2, 2720–2721 | research verified (R2 rec; 48 under MODES-3 (a)) |
| TE184 | 44 bots that pin the hash plus one session per browser; 5 minutes after each rotation; 3 rotations | PM-26 | none | proposed (test-engineer: the browsers' behaviour stays in the test; a one-in-a-few failure has a chance to show) |
| TE185 | A certificate valid for 15 days is rejected by all four browsers | PM-27 | r2 R2-TRN-REC5 mark 3, 2722 | research verified (R2 rec) |
| TE186 | "the total length of the validity period MUST NOT exceed two weeks" | PM-27 | r2 R2-TRN-F13, 1035 | research verified |
| TE187 | 5 attempts per browser; control PM-0006-25 | PM-27 | none | proposed (test-engineer) |

## Crash reports (PM-0006-28 to PM-0006-31)

| # | the claim or number (short) | where | citation (file, id, lines) | tier |
|---|---|---|---|---|
| TE188 | Only if the EU Sentry organization exists, in the boundary spike; otherwise at the first online playtest's milestone | PM-28–31, SE | draft, What a spike must prove, 763–768; map part g, row 942 | settled |
| TE189 | A deliberate Wasm trap arrives in Sentry with Rust function names in Chrome, Edge, Firefox and Safari (4 of 4) | PM-28 | r6 R6-OBS-REC3 mark, 765–766 | proposed (R6's) |
| TE190 | Sentry calls its Wasm support "still developing" | PM-28 | r6 R6-OBS-F7, 297–306 | research verified |
| TE191 | 3 traps per browser; bite: no debug files uploaded | PM-28 | none | proposed (test-engineer: the check must read names that come from symbolication) |
| TE192 | The shipped `.wasm` holds no DWARF (size checked against the stripped build) | PM-29 | r6 R6-OBS-REC3 mark, 766–767 | proposed (R6's) |
| TE193 | A section listing shows no `.debug_` section; bite: a build that ships its debug file | PM-29 | none | proposed (test-engineer) |
| TE194 | A forced tab crash in Chrome produces a `crash` report; browsers that send none are listed | PM-30 | r6 R6-OBS-REC3 mark, 767 | proposed (R6's) |
| TE195 | Tab crashes such as running out of memory reach a reporting endpoint; browser support varies | PM-30 | r6 R6-OBS-F8, 307–315 | research, source to check |
| TE196 | 3 crashes per browser, 3 of 3 in Chrome; arrival time recorded (when a report is sent was not read) | PM-30 | none | proposed (test-engineer) |
| TE197 | 0 IP addresses in a sample of 100 events | PM-31 | r6 R6-OBS-REC3 mark, 768 | proposed (R6's) |
| TE198 | The SDK sends the IP by default; `userInfo: false` turns it off | PM-31 | r6 R6-OBS-F10, 327–331 | research, source to check |
| TE199 | Search by script for the lab clients' addresses and any address pattern; bite: an event with default user data on | PM-31 | none | proposed (test-engineer) |

## Failure rule, no-mark lines, checklists, spike entries

| # | the claim or number (short) | where | citation (file, id, lines) | tier |
|---|---|---|---|---|
| TE200 | Conditional, reported and adoption marks: how each counts | FAIL | TE-0003, failure rule (same form) | proposed (test-engineer) |
| TE201 | A failing run is never re-run until green; data kept; cause found | FAIL | r5 R5-TEST-REC6, 1490–1498 | proposed (R5's) |
| TE202 | PM-0006-6's three outcomes; "written down" records a fail, never a pass by itself | FAIL | r5 R5-PREV-REC3 mark, 1622–1624; draft part 3, 159–162 (a failure with no workaround sends the host to Workers static assets) | proposed (test-engineer's reading: otherwise the mark could not fail) |
| TE203 | A failed mark goes back to the draft by its Rollback and "Revisit when" routes, then to the advisor and to Mustafa and the Advisor | FAIL | draft Rollback, 840–857; Consequences "Revisit when", 566–571; plan step 2 and rule 12 | settled |
| TE204 | A new reference server is followed by every draft citing N34 | FAIL | draft Rollback, 845–846 | settled (the draft's) |
| TE205 | A mark's number changes only in the draft, with its reason | FAIL | r5 R5-GATE-REC4, 1734–1744 | proposed (R5's) |
| TE206 | The sweep and artifact retention are ADR 0002's jobs, checked at the foundation | NOM | draft part 3, 158–159; Test plan, 813–815; seam S3 | settled |
| TE207 | The staging game server's marks belong to the first multiplayer preview | NOM | r6 R6-HOST-REC3 mark, 681–683; map part g, later milestones, 957 | settled (map) |
| TE208 | The lobby's place is decided by a read | NOM | draft OQ-0006-3, 615–625 | settled |
| TE209 | The production certificate form waits for the security baseline, before the first online playtest | NOM | r6 R6-HOST-REC4 mark, 697–699; BASELINE-WHEN (b); map, 957 | settled |
| TE210 | The cost line, the bill check and the start budget: slices 3 and 4; CI runner costs: OQ-0006-7 | NOM | r6 R6-COST-REC2 mark, 724–725; map, 958; draft OQ-0006-7, 658–667 | settled |
| TE211 | The data map and retention check before the first online playtest | NOM | r6 R6-OBS-REC4 mark, 779–780; map, 957 | settled |
| TE212 | Telemetry marks at slice 4 | NOM | r6, 807–811, 824–827, 840–842; map, 958 | settled |
| TE213 | The security baseline has no mark before its research | NOM | BASELINE-WHEN (b) | settled |
| TE214 | The netcode proof test's "look correct" checklist and its measurable marks are ADR 0001's | NOM | d0001, What a spike must prove, item 4, 968–984; plan rule 12 | settled |
| TE215 | Steps 1 to 4 with the policy off; 5 to 8 after PM-0006-6's 24 h reading | HC | TE72 | proposed (test-engineer) |
| TE216 | Step 8: how long a login lasts is recorded, not judged | HC8 | none | proposed (test-engineer: the research marks one login per session, not its length) |
| TE217 | The operations checklist: `ping` outputs and the bill reading | OC1–2 | TE98, TE154 | proposed (test-engineer) |
| TE218 | Spike names, what each proves, owners | SE | map part g, rows 941, 942, 947 | settled (the map's assignment) |
| TE219 | The hosting check runs in parallel with the netcode proof test and ends before the foundation's preview step | SE | map part g, row 947 and order, 962–963 | settled (the map's proposal, for SPIKE_PLAN) |
| TE220 | Sizes: S (hosting check, calendar span at least two days), M added to ADR 0001's spike, S added to the boundary spike | SE | none | proposed (test-engineer: the work listed in each entry) |
| TE221 | Subsumed and decided ids per spike | SE | map part g, rows 941, 942, 947; draft Appendix A, 885, 889, 914, 921–922 | settled (the map's and the draft's dispositions) |
| TE222 | The map places R5-PREV-REC5's access wall at the first multiplayer preview; the draft runs its check in the hosting check | SE | map, 957; draft spike 1, 731–734 | settled (both texts, read 2026-11-01) |
| TE223 | R5-PREV-REC4 mark 1 (the deploy job's check) is ADR 0002's, at the foundation | SE | draft Test plan, 813–814; map, 952 | settled |
| TE224 | ADR 0001's spike text names R2-TRN-REC5 marks 1 and 3 as ADR 0006's | SE | d0001, 946–947 | settled (read 2026-11-01) |
| TE225 | The boundary spike's dependency: the Sentry organization created in the EU after Sentry's prices are read; its upload token added by Mustafa | SE | draft part 10, 303–305; OQ-0006-8, 668–676 | settled |
| TE226 | The sim core spike gives the sim step time in the stress scene on the reference server | SE, PM-14 | map part g, row 943 | settled (the map's) |

## Counts
- Rows: 226 (TE1–TE226), counted by script on this file.
- By tier (first tier of each row; script on this file): research verified 26 · research, source to check 21 ·
  arithmetic 11 · settled 54 · design doc 1 · proposed 113 (R6's 21, R5's 12, R2's 1, R3's machines reused 1, the
  draft's 2, test-engineer's own 76, its two readings included).
- Rows naming a test-engineer proposal anywhere in the tier (method choices, readings, planted defects and controls,
  each with its reason): 79.
