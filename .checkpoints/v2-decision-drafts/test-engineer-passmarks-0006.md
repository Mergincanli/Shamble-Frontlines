# Pass marks for ADR 0006 (infrastructure and operations), test-engineer, 2026-11-01

Plan step 4. Two parts: the `### Pass marks` subsection, which the session swaps in verbatim for the draft's
placeholder (`### Pass marks` and its one line, `infra-engineer-adr0006-draft.md` lines 770–771), and the spike entries
for `docs/SPIKE_PLAN.md`, which stay in this notes file for the synthesis (plan step 6). The claims ledger for every
row is `test-engineer-passmarks-0006-claims.md`.

---8<--- the subsection starts on the next line ---

### Pass marks
Written by test-engineer (plan step 4). One row per mark: the preview hosting check's first (PM-0006-1 to PM-0006-10),
then this record's operations marks inside the netcode proof test (PM-0006-11 to PM-0006-27), then the crash-report
marks that run only if the EU Sentry organization exists (PM-0006-28 to PM-0006-31). Every number is a research mark
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
  same location. **LG**, the load generator: a separate VM in the same location that runs every bot client, never the
  VM under test (proposed: bot clients on the VM under test would spend the vCPUs the marks measure); a CX33 unless
  its own CPU use (recorded every second) passes 80 % in a run, which makes that run invalid (proposed). **MV**, a
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
  (TICK-30); none runs at 60 Hz: [the 60 Hz margin](../research/GLOSSARY.md#the-60-hz-margin) is ADR 0001's run.
- **The load.** ADR 0001's netcode proof test load (its throwaway sim with movement, hitscan and bots; the stand-in
  for [the stress scene](../research/GLOSSARY.md#the-stress-scene) as OQ-0001-2 sets it) at N = 48 (MODES-3 (a))
  unless a row names another N, with one snapshot-receiving bot client per soldier, run on LG as headless protocol
  clients (R5-TEST-REC5, r5 lines 1475–1478). **Profile:** P2 (100 ms round trip, ±20 ms jitter, 2 % loss: r2 lines
  1987–1990; the reading of "100 ms of simulated latency" ADR 0001 states, decision map part j, J6 (3)), injected on LG
  in both directions, half the round trip each way (R5-TEST-REC4's per-direction rule, r5 lines 1455–1456), never on
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

| mark | what it proves (decision part) | number and statistic | hardware | method and tool | runs and noise | basis |
|---|---|---|---|---|---|---|
| **The preview hosting check** | | | | | | |
| PM-0006-1 | The check's setup is fixed before the first run (parts 1–3; a precondition) | recorded: CF's name and production URL, TR, the throwaway workflows' commit, the build-marker rule, ML's and MAC's operating systems, each browser's and WebdriverIO's versions | CF, TR, ML, MAC | a results-log header written before the first run; every later row names its URL, deployment and browser | checked again before PM-0006-6's 24 h reading | R6-HOST-REC1's "on one preview link" (r6 line 639); the draft's spike 1 (a static page and a `.wasm`) |
| PM-0006-2 | The host serves each header set as generated, on both URL forms (part 1; decides R5-PREV-OQ6) | 100 % of responses carry their expected set: every `.wasm` as `application/wasm`; previews `X-Robots-Tag: noindex`, production none; the preview built with [the isolation flag](../research/GLOSSARY.md#preview-isolation-and-the-isolation-flag) on, COOP `same-origin` and COEP `require-corp` on every HTML, JS, worker and Wasm response; production and the flag-off preview, neither COOP nor COEP. The content security policy is not checked (it waits for the security baseline, part 13) | CF; the script runs on ML (a header check, not a hardware result) | one test build deployed three ways (production; a preview with the flag off; a preview with it on), each read on its branch alias and on its deployment's hash URL; the test page lists every resource it and its worker loaded (Resource Timing in both), and a script fetches each URL with a cookie-less HTTP client and compares the headers with the expected sets, written into the script by hand from GLOSSARY §5's header-set entries (proposed: an expectation generated by the build would share the build's mistakes) | 1 load per URL form and deployment (headers repeat). Bites: COEP removed from the worker's rule in the flag-on build (fails on the worker response); a `noindex` rule added to production | R6-HOST-REC1's mark (r6 lines 639–641); R5-PREV-REC4 mark 3 (r5 lines 1639–1640); R5-PREV-F20 (r5 lines 896–900); GLOSSARY §5 (lines 652–658); R5-PREV-OQ6 (r5 lines 1860–1861) |
| PM-0006-3 | `crossOriginIsolated` follows the isolation flag in every target browser (parts 1, 2) | `true` on the flag-on preview and `false` on the flag-off preview and on production, read in the page and in its worker: 3 of 3 loads per browser and deployment | ML: Chrome, Edge, Firefox; MAC: Safari; MP: Brave (the hosting checklist, steps 1–3) | WebdriverIO opens each URL and reads the value the page and its worker print | 3 loads per browser and deployment (proposed: one load could hit a cached older deployment). Bite: PM-0006-2's missing COEP gives `false` on the flag-on preview | R6-HOST-REC1's mark (r6 lines 640–641); R5-PREV-REC4 mark 2 (r5 lines 1638–1639, Brave included) |
| PM-0006-4 | The content bundle is cached for good (part 1, interface I-6) | its `Cache-Control` contains `immutable`; on a second visit in the same browser profile (a new navigation, not a forced reload) it transfers 0 bytes: Resource Timing `transferSize` 0 | CF; ML: Chrome | WebdriverIO: first visit, tab closed, second visit; `transferSize` of the bundle's entry read on the second | 3 repeats, each from a fresh profile; 0 bytes in each. Bite: the bundle served with `Cache-Control: no-cache` (the second visit revalidates, so its transfer is above 0) | R6-HOST-REC1's mark (r6 lines 641–642); R1-DATA-REC6 named there |
| PM-0006-5 | Every served file fits the host (part 1; N37) | the largest file in each deployed build under 25 MiB = 26,214,400 bytes (25 × 1,048,576), on every deployment of the check | CF; ML | a script lists the deployed build output's file sizes and reports the largest; on Workers static assets the limit is re-read from Workers' own limits page before the check (only if the alternative is used) | every deployment of the check. Bite: a planted 26 MiB file is flagged by the script, and what the host does with it (refuses the deploy or serves it) is recorded | R6-HOST-REC1's mark (r6 lines 642–644); R6-HOST-F3 (r6 lines 72–79) |
| PM-0006-6 | PREV-DELETE: a closed pull request's build is gone from every link (part 3; decides PREV-DELETE, R5-PREV-OQ2) | 24 h after each test pull request closes (a plain deadline, GLOSSARY §1), none of its URLs (the branch alias and the hash URL of every deployment it ever had) answers HTTP 200 with that build's marker; an answer of HTTP 404, any other response without the marker, or [the access wall](../research/GLOSSARY.md#the-access-wall) counts as gone. Run with the access policy **off** (proposed: with it on, every leftover link sits behind the wall and the mark could not fail; off is the state of slices 1 and 2) | CF, TR; the probe runs from ML | four test pull requests on TR, closed by the spike's throwaway close workflow (`wrangler pages deployment delete … --force` per deployment, R5-PREV-REC3): (a) branch `cs/pd-one`, 1 deployment, closed without merge, branch kept (the `/` becomes `-` in the alias, R5-PREV-F12); (b) `cs/pd-three`, 3 deployments, merged, branch deleted; control (c), an open pull request; bite (d), a closed one with the close workflow switched off. A cookie-less probe reads every URL every hour from the close to 24 h after | 1 run per case (a one-off deadline). Control: (c)'s URLs serve its build at every probe. Bite: (d) is reported as still serving at 24 h. The hourly probes record when each URL stopped serving | R5-PREV-REC3's mark (r5 lines 1622–1624); PREV-DELETE (r6 lines 970–976); R5-PREV-F13 (r5 lines 845–849); R6-HOST-F4: a hash URL per deployment plus a branch alias (r6 lines 80–86) |
| PM-0006-7 | Whether a direct upload counts against Pages' 500 builds a month (part 1; R6-HOST-REC1's question for the spike) | reported, no pass number: the project's monthly build count before and after 10 direct uploads; counts if it rose by 10, does not if by 0, anything else recorded as read | CF; MP (Mustafa reads the count in his own dashboard) | 10 deployments by the throwaway deploy workflow between two readings (the hosting checklist, step 4) | one pair of readings; passes when both are recorded | R6-HOST-REC1 (r6 lines 633–634); R6-HOST-F3 (r6 lines 72–79) |
| PM-0006-8 | The access wall covers every preview of the project and not production (part 2; feeds OQ-0006-4) | with the access policy on: on every preview URL of at least 2 open branches (aliases and hash URLs) and on PM-0006-6's leftover URLs, a cookie-less client gets no build (no response with a build marker); on the production URL, and on a custom domain if CF has one, it gets the build | CF; the probe runs from ML | Mustafa switches the policy on (a setting in his account); the probe sweeps every URL 10 minutes and 1 hour later; the mark reads the 1-hour sweep (proposed: a setting may take time to reach every edge; both times recorded) | 1 sweep at each time. Control: the same sweep with the policy off returns the build on every open preview URL | R6-HOST-F4, inferred there (r6 lines 80–86); R5-PREV-F12 (r5 lines 838–844); OQ-0006-4 |
| PM-0006-9 | Mustafa's verdict on the preview links in Brave (parts 1, 2) | every step of the hosting checklist below "achieved", 0 "not yet" | MP, Brave on Windows | the hosting checklist below | one session; a "not yet" goes back to the draft and is shown again | R5-PREV-REC5's mark (r5 lines 1648–1650); R5-PREV-REC4 mark 2's Brave (r5 line 1639); plan rule 12 |
| PM-0006-10 | Whether GitHub shows the environment URL on the pull request page (decides R5-PREV-OQ5) | recorded: yes or no, with a screenshot of the page | TR | the throwaway deploy workflow sets its environment's `url` to the preview link (R5-PREV-REC1); the pull request page is read after a deploy | once; passes when recorded | R5-PREV-OQ5 (r5 lines 1858–1859) |
| **The operations marks inside the netcode proof test** (ADR 0001's spike) | | | | | | |
| PM-0006-11 | The reference server exists and is the machine every server row ran on (part 5; OQ-0006-1, the precondition) | a CCX23 and a CX33 created in `nbg1` or `hel1`; recorded for each: type, location, CPU model, vCPU count, memory, kernel; the CPU model read again at every session (a change is recorded and named in the results) | RS, SH | Mustafa creates the VMs or the API token that creates them (his account); a script reads `/proc/cpuinfo` and the plan at each session start | every session | R6-HOST-REC2 (r6 lines 645–650); R6-HOST-F20 (r6 lines 180–182); GLOSSARY, the reference server (lines 225–231) |
| PM-0006-12 | The round trip to each candidate location (part 5; feeds OQ-0006-2) | reported, no pass number: per location, the mean and the max round trip and the loss of 100 ICMP echoes, as Windows' `ping` prints them | one throwaway CX33 per offered location (`nbg1`, `hel1`, `fsn1` if offered), ICMP allowed on these only, deleted after; from ML and from MP | `ping -n 100 <address>` per location from each machine (MP: the operations checklist, step 1); the outputs pasted into the results log | 1 run per machine and location; a second run at another time of day if two locations differ by under 5 ms (proposed) | OQ-0006-2; R6-HOST-OQ2 (r6 lines 856–858); R6-HOST-F18: the firewall filters ICMP (r6 lines 169–174) |
| PM-0006-13 | Rooms per VM: 8 rooms of 48 on each machine (part 5; with ADR 0001's R2-ROOM-REC2 marks 1 and 2: one set of runs serves both records) | each of 8 rooms' server tick p99 at most 1.10 × the single-room figure; on SH also R2-ROOM-REC2 mark 1 for every room (p99 at most 16.7 ms, max under 33.3 ms, 0 overruns: [the server tick budget](../research/GLOSSARY.md#the-server-tick-budget) at 30 Hz) and every room start-up (process spawn to Open) under 2 s (R6-HOST-REC2 names both machines; the RS half of marks 1 and 5 is ADR 0001's) | RS and SH, each with LG | 8 room processes under the supervisor, no CPU pinning (as production would run them; recorded); the load at N = 48 (384 bot clients on LG); bracket B; each room process's CPU share of one vCPU per second also recorded (it holds the [transport runtime time](../research/GLOSSARY.md#transport-runtime-time)) | 3 sessions per machine at the 3 times of day; every valid session passes. **On SH it is an adoption mark:** a fail, or an invalid session, keeps the fleet on dedicated vCPUs (K1) and K3 out, as part 8 says; it is not a fail of this record. Bite: a busy-wait of 20 % of the single-room p99 planted in every tick of one room must fail that room's row (proposed) | R6-HOST-REC2's mark (r6 lines 667–671); R2-ROOM-REC2 marks 1, 2, 5 (r2 lines 2774–2780); A-2 (r6 lines 516–522); N32 (0006 the figure, 0001 the test) |
| PM-0006-14 | Rooms per VM counted, the figure that replaces A-2 (parts 5, 8; decides R6-COST-OQ1) | reported, no pass number: on each machine and at N = 48, 36 and 18, the most rooms k for which every room meets R2-ROOM-REC2 marks 1 and 2 (the draft's counting rule). The figure counts as the goal budget's density only if each room's sim step time p99 in the run is at least the sim core spike's figure for the stress scene at that N (proposed: a stand-in cheaper than the real sim would overstate k); otherwise it is labelled "on the stand-in" and part 8 keeps both densities until slice 6 re-measures (the draft's migration step 7) | RS and SH, each with LG | per N, in one session with bracket B: k = 4 (A-2's case to budget, one room per vCPU), then 6, 8 and up by 2 until a k fails, then the odd k between the last pass and the first fail; if k = 4 fails, 3, 2, 1. Then the count and count + 1 again at the other 2 times of day. The load as PM-0006-13 with N set to 48, 36 and 18 (the goal modes and the three-team start mode's upper end) | the figure is the lowest count of the 3 sessions (proposed: a budget takes the worst case). A run plan with every session's VM-hours is written before the first session (it feeds PM-0006-18) | R6-COST-REC1's mark (r6 lines 714–716); the counting rule (draft, spike 2, proposed there); R6-COST-OQ1 (r6 lines 880–882); J6 (2) and OQ-0001-2 (the stand-in's load) |
| PM-0006-15 | The firewall lets both game transports in and nothing else (part 5) | from each of the four browsers, 20 of 20 WebTransport sessions over UDP and 20 of 20 WebSocket sessions over TCP (`wss`) reach a room on RS; with no rule for a port, a probe to a listening service on that port gets no answer (a TCP connect times out, never "refused"; a UDP probe gets no reply); the firewall's rule list holds the UDP range, the TCP range and part 9's metrics rule (PM-0006-16), and 0 other rules. The `wss` endpoint presents a certificate the four browsers accept with no warning, and which one is recorded: the draft names none for development, and the WebSocket constructor takes only a URL and sub-protocols (R2-ROOM-F12, r2 lines 1494–1500), so a self-signed certificate cannot be pinned for `wss` | RS; ML: Chrome, Edge, Firefox; MAC: Safari; a probe from ML | WebdriverIO opens the proof test's client page with the transport forced by a URL flag; an attempt succeeds when the session is ready and one datagram or message echoes back; the probe opens a TCP connection and sends a UDP datagram to one closed port of each kind, with a test listener on each; the rule list is read from Hetzner's console or API and pasted into the results | 20 attempts per browser and transport (proposed: R2-TRN-REC5's count); 1 probe pair. Bite: a rule added for the probed TCP port makes the probe connect (the negative test can fail) | R6-HOST-REC2's mark (r6 lines 669–671); part 5 ("and nothing else"); R6-HOST-F18 (r6 lines 169–174); R2-TRN-REC3 (r2 lines 2639–2659) |
| PM-0006-16 | The metrics path works with only the rule it needs (part 9) | every metric of PM-0006-22 reaches Prometheus on MV; if part 9 picks the scrape: the exporter's port answers MV's address only and drops every other source (the PM-0006-15 probe from ML gets no answer); if it picks the push gateway: no inbound rule for metrics | RS, MV; a probe from ML | the path is configured as part 9 picks it; the rule names MV's address as its only source (proposed: the narrowest rule that serves a scrape) | 1 probe from ML; the metrics as PM-0006-22. Bite: the rule opened to every source makes ML's probe connect | the draft's spike 2 (proposed there: the negative test extended); R6-OBS-F4: scrape or push (r6 lines 279–284) |
| PM-0006-17 | Egress per 24v24 match-hour matches R2's estimate (part 8) | bytes sent from RS's network interface in one room-hour within ±20 % of R2's E3 estimate at the counts that ran (R2's formula, r2 lines 1764–1766, evaluated in the results with the run's P, V, shots and projectile spawns, the arithmetic shown); at R2's own counts (P = 48, V = 15 moving vehicles, 0.44 projectile spawns per player per second, every entity in view) that is 3.156 GB, so 2.52 to 3.79 GB. Beside it, the room's own bytes-sent metric agrees with the interface counter within 5 % (proposed: R6-COST-REC2's tolerance, a check that the counter measures the room). With interest management on: the figure reported, only if the proof test's sim has the interest filter | RS with LG; one room alone | 48 bot clients over IPv4 (R2's case), all in view; three 20-minute matches back to back, joins and full states included (proposed: a room-hour as a real room spends it); the interface's transmitted-byte counter read at the start and end | 1 room-hour per setting; P2 on LG. Bite: snapshots sent twice per tick (about 2 × the reference) must fail | R6-COST-REC1's mark (r6 lines 716–717); the egress table (r6 lines 438–446: 146.1 kbps × 48 = 7.01 Mbit/s; × 3,600 s ÷ 8 = 3.156 GB); R2's estimate (r2 lines 1759–1762, 1775); A-1 (r6 line 515) |
| PM-0006-18 | The spike's own hosting bill (part 8) | under 5 EUR excluding VAT for every Hetzner VM-hour and traffic block the netcode proof test creates (all its marks, ADR 0001's included); planning check before any VM is created: the run plan's hours at 0.1386 EUR (CCX23) and 0.0144 EUR (CX33) an hour sum under 5 EUR (5 ÷ 0.1386 = 36 CCX23-hours if nothing else ran), else the plan goes back to the draft first (proposed: a bill found too high afterwards cannot be undone) | the Hetzner project of the spike; MP (Mustafa reads the bill) | VMs are created per session and deleted after it (billing rounds up to the hour, R6-HOST-F17); Mustafa reads the project's costs (the operations checklist, step 2) | 1 reading after the last session | R6-COST-REC1's mark (r6 lines 716–717); R6-HOST-F15 hourly prices (r6 lines 150–159); R6-HOST-F17 (r6 lines 164–168); prices exclude VAT (r6 lines 40–41) |
| PM-0006-19 | Logging costs the tick almost nothing (part 9) | server tick p99 with logging at its production level at most 1.02 × with logging off | RS with LG; one room alone, one vCPU ([the measuring unit](../research/GLOSSARY.md#the-measuring-unit-one-vcpu)) | 6 runs of 10 measured minutes (after the warm-up), each a fresh process, alternating off, on, off, on, off, on; one recorded input replay drives all six (proposed: the same work in each); the mark compares the pooled p99 of the 3 "on" runs (54,000 ticks) with that of the 3 "off" runs | valid only if each "off" run's p99 lies within 1 % of the pooled "off" p99 (proposed: half the mark; a 2 % difference may sit inside run-to-run noise, so noise is checked first); an invalid set is neither pass nor fail. Bite: a planted 50 µs busy-wait in each logged event | R6-OBS-REC1's mark (r6 lines 736–737) |
| PM-0006-20 | Logging allocates nothing on the hot path (part 9) | on every tick after the warm-up, the tick thread's allocation count with logging at its production level equals the count with logging off | RS; one room | a counting global allocator in the room binary (Rust's `GlobalAlloc`; proposed tool) records allocations per tick on the tick thread; both runs driven by the same recorded input replay, so the sim's own allocations repeat tick for tick | 2 runs (on, off) of 10 measured minutes; compared tick by tick. Bite: a `format!` planted in a per-tick log call | R6-OBS-REC1's mark (r6 line 737); R6-OBS-F1 (r6 lines 262–269) |
| PM-0006-21 | Logs keep no secrets or addresses (parts 9, 11) | 0 matches in a 30-minute soak's logs for every join token, every guest secret and every client address the soak used, and 0 matches for an IPv4 or IPv6 address pattern other than the VMs' own | RS, LG | the soak of PM-0006-22; the spike logs each token, secret and client address it creates into a canary file on LG (never on RS); a grep script searches every log line the supervisor shipped | 1 soak. Bite: a planted log line that prints one client's address and one token must be found | R6-OBS-REC1's mark (r6 lines 737–738); R2-ROOM-REC3 mark 3 (r2 lines 2807–2809) |
| PM-0006-22 | Every first metric exists and moves (part 9) | after a 30-minute soak of 8 rooms at N = 48: every metric part 9 lists is present on MV, and each counter the soak's script triggers has moved (proposed: a present but stuck counter passes a presence check): one join refused per room (a bad token), one room process killed at minute 20 (crash, end and a new start), a quarter of the bots on WebSocket (both transports in the session metric); counters the soak gives no trigger (per-client budget overruns, relevance churn, datagrams dropped) are checked for presence only | RS with LG and MV | 8 rooms under the supervisor; the load as PM-0006-13; Prometheus queries per metric after the soak | 1 soak; P2 on LG. Bite: one metric left unregistered fails the presence check | R6-OBS-REC2's mark (r6 lines 750–751); part 9's list (R6-OBS-REC2, r6 lines 741–744) |
| PM-0006-23 | A stall is visible at once (part 9) | after a deliberate 1 s stall inside one tick (proposed size: longer than the catch-up bound, so time is dropped), the first scrape shows the overrun counter up by exactly 1 and the dropped-time counter up by the stall minus the catch-up bound ADR 0004 sets, within one tick period (33.3 ms) | RS, MV | the stall injected by a test command at minute 25 of PM-0006-22's soak; Prometheus' scrape interval set to 15 s and recorded (proposed; the default was not read) | 3 stalls, 5 minutes apart (proposed); each must show. The stall is the planted event. Control: before the first stall the soak shows 0 overruns (R2-ROOM-REC2 mark 1's 0 overruns) | R6-OBS-REC2's mark (r6 lines 751–752); R1-TICK-REC4 (r1 lines 2078–2093) |
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
- No mark here runs at 60 Hz: the 60 Hz margin is ADR 0001's run. The netcode proof test's "look correct" checklist and
  its measurable marks (hits the server confirms, correction sizes, prediction error at P2) are ADR 0001's pass marks.

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

---8<--- the subsection ends on the line above ---

### Spike entries (for docs/SPIKE_PLAN.md)
Names from the decision map's part g. Sizes: S about a day, M 2 to 4 days, L a week or more of session work.

**The preview hosting check** (ADR 0006)
- **What it proves:** the client host serves the production header set and the preview header set as generated, on
  branch aliases and hash URLs; `crossOriginIsolated` follows the isolation flag in Chrome, Edge, Firefox, Safari and
  Brave; the content bundle is cached immutably; every file fits the 25 MiB limit; a closed pull request's build is
  gone from every link within 24 h (PREV-DELETE); what the access wall covers, and Mustafa's one login per Brave
  session; whether direct uploads count against Pages' build allowance; whether GitHub shows the environment URL.
- **Its marks:** PM-0006-1 to PM-0006-10 (the hosting checklist inside PM-0006-9).
- **Dependencies:** the drafts approved; Mustafa's Cloudflare account with the test project, its deploy token in a
  `preview` environment of the repository he names for the test pull requests (secrets are his to add); a static test
  page with a module worker, a `.wasm` and a hashed bundle stand-in; throwaway deploy and close workflows; ML and MAC
  with the four browsers and WebdriverIO; Mustafa's time in Brave and in his dashboard. It shares nothing with the
  netcode proof test, so it runs in parallel with it; it must end before the foundation's preview step.
- **Size:** S of session work, with a calendar span of at least two days: PM-0006-6 waits 24 h after each close, and
  PM-0006-8 needs the policy switched after PREV-DELETE's run.
- **Research spikes it subsumes:** R6-HOST-REC1 (its mark); R5-PREV-REC2 (its mark); R5-PREV-REC3 (the 24 h mark; the
  sweep is the foundation's); R5-PREV-REC4 marks 2 and 3 (mark 1, the deploy job's check, is ADR 0002's at the
  foundation); R5-PREV-REC5's mark (the decision map places it at the first multiplayer preview; the draft runs it
  here, on the test project). **Decides:** PREV-DELETE (R5-PREV-OQ2); R5-PREV-OQ5; R5-PREV-OQ6; it gives Mustafa what he
  needs for OQ-0006-4.

**The netcode proof test** (ADR 0001's spike), this record's operations marks
- **What they prove:** the reference server and its shared neighbour named and recorded; rooms per VM on both
  machines (8 rooms of 48, and the count at 48, 36 and 18), which fixes part 8's goal budget; both game transports
  through the firewall from four browsers and nothing else; the metrics path; egress per match-hour against R2's
  estimate; the spike's own bill; logging cost, allocations and secrets; every first metric present and moving, a
  stall visible at once, bounded series; the development certificates in four browsers, their rotation and the
  15-day refusal; the round trip to each candidate location.
- **Their marks:** PM-0006-11 to PM-0006-27 (the operations checklist inside PM-0006-12 and PM-0006-18). PM-0006-13
  shares its runs with ADR 0001's R2-ROOM-REC2 marks 1 and 2: one set of runs, and the two tables must read the same.
  ADR 0001's spike text names R2-TRN-REC5 marks 1 and 3 as this record's: they are PM-0006-25 and PM-0006-27.
- **Dependencies:** OQ-0006-1 answered (a), in Mustafa's Hetzner account; ADR 0001's throwaway sim and bots as protocol
  clients, native, with its recorded input replays; the sim core spike's sim step time in the stress scene for
  PM-0006-14's validity (else its figure is labelled "on the stand-in"); LG and MV in the same location; ML and MAC
  with the four browsers; the run plan of PM-0006-18 written and under 5 EUR before the first VM; Mustafa's two
  checklist steps.
- **Size:** M added to ADR 0001's spike: three sessions on each machine at different times of day, the count search
  at three sizes, a 30-minute soak and the certificate runs.
- **Research spikes it subsumes:** R6-HOST-REC2 (its mark, with R2-ROOM-REC2 marks 1, 2 and 5 on the CX33); R6-COST-REC1
  (its mark); R6-OBS-REC1 (its mark); R6-OBS-REC2 (its mark); R2-TRN-REC5 marks 1 (the development half), 2 and 3.
  **Decides:** R6-COST-OQ1; feeds OQ-0006-2.

**The Rust ↔ TypeScript boundary spike** (ADR 0002's), only if the EU Sentry organization exists
- **What it proves for this record:** a Wasm trap reaches Sentry with Rust names in four browsers; no debug data
  shipped; tab crashes reported (and which browsers send none); no IP address in crash reports.
- **Its marks:** PM-0006-28 to PM-0006-31; otherwise they move to the first online playtest's milestone.
- **Dependencies:** Mustafa's Sentry organization created in the EU location after he reads Sentry's prices
  (OQ-0006-8), and its upload token added by him; a crash-report endpoint for the spike (the lobby's does not exist
  yet).
- **Size:** S added to the boundary spike.
- **Research spikes it subsumes:** R6-OBS-REC3 (its mark). **Decides:** R6-OBS-OQ3, R6-OBS-OQ4.
