Task: split every GLOSSARY.md 'Retire' field into Retire (exact) / Retire (in context) / Keeps (format only), before step 3c of plan-amendments-2.md; advisor ruling 2026-10-30.
Method: hits from scripts/glossary-hits-before-split.tsv (viewer: scratchpad hits.py); before-copy of GLOSSARY.md in scratchpad glossary-before.md.
Rules used: exact = no context clause (where/for/as/without/with no/in/once) and no correct hit today; the advisor's core rulings; clause -> in context; an example quote inside a clause inherits it; a clause-less phrase with a correct hit -> in context with added context words.
Found: quoted phrases split by a line break never match (glossary side: 7; topic side, e.g. r4:691-692, r6:609-610). Unbreaking glossary-side ones (format); retire_split_compare.py compares raw strings, so it will flag those as new/dropped.
Entries (entry: exact / in context / keeps):
How to read an entry: labels described (three bullets: exact, in context, Keeps)
Tick period: exact none / in context 'the tick interval' (added: for our tick period; R1-TICK-F13 is Source's own term, a correct use), 'the period' / keeps none
Sim step time: exact 'the per-tick sim budget' (1 hit, R1-DET-REC3, wrong) / in context tick time, mean tick, p99 tick, tick cost / keeps Whole ticks
Server tick time: exact 'whole tick' (4 hits: r2:1721-1722 table, R2-BUD-REC6, R2-XU-OQ8, all the whole server tick), 'p99 whole tick' (1) / in context a room's tick, tick p99, tick time / keeps none
Transport runtime time: exact '1 CPU core for the transport' (1, R2-TRN-REC1) / in context 'the transport layer' / keeps none
Time statistics: exact none / in context the 4 marks (clause: a time mark with no statistic; a mark with p99 named is right; 0.2 ms and 0.1 us phrases unbroken onto one line) / keeps none
The server tick budget: exact 'the tick budget' (5 hits, all the server tick budget unmarked), 'the tick's p99 budget' (1), 'the share of the tick budget the tick-rate unit sets' (1) / in context 'the server tick budget' w/o number, 'CPU budget per tick', '25 %', '50 %' / keeps none
Sim step share and server-only share: exact 'the share of the tick budget' (0, absorbed by the longer phrase), '25 % of the tick budget' (1), 'half of that mark's mean tick' (0: r4:691-692 line break hides the one wrong use) / in context '10 % of the server tick budget' (unbroken), '10 % budget' / keeps none
The client's sim share: exact none / in context 'tick time' / keeps none
The reference server: exact the 7 id-anchored phrases (1 hit each, all wrong; 'one vCPU of the EU instance type R6 recommends' had 0 because the glossary broke it across a line: unbroken now) + 'reference host' (2, R2-ROOM-REC2, wrong) / in context 'to be named with R3, R5 and R6' / keeps none
The measuring unit: exact one core (1: R2-INT-F20, cost of the server LOS pass, but measured on the unrecorded legacy machine: borderline), per core (7, all server: matches/rooms per core), server core (1), 1 CPU core (1) / in context none / keeps Core, at least 4 cores and 8 threads
Player count (N): exact none / in context characters, 48 characters (example inside the 'for soldiers' clause; could be right for character models), 48 / keeps the largest mode's N
Team size: exact none / in context 2x2, players / keeps none
Start and goal modes: exact none / in context 'the goal', '(start, upper end)' / keeps none
The largest mode: exact none / in context 'the largest mode (48 soldiers, 47 of them bots)', 'Largest N' (clause: where they fix N at 48), '48' / keeps none
Design point: exact none / in context 'the largest mode' / keeps none
Test-mode list: exact none / in context 'every mode we test' (the meaning, left quoted: a text using it carries the term unmarked), the R2-ROOM-REC7 list (unbroken), '24v24 = 48 and 12v12v12 = 36' (right where it names the goal modes) / keeps none
The stress scene: exact P1's scene (1, R1-TICK-REC3), the worst case of the largest mode (1), R4-PHYS-REC4's stress scene (1), a physics scene of the largest mode's size (0: glossary line break; unbroken), the worst case (48 players ... …) (0: the ellipsis never matches) / in context stress load (r4:761 Rests-on cites R1-ECS-REC1's own load), R1-TICK-REC2's P1 scene (r4:1119 wave-1 list), R1-TICK's scene (r3:1151 Rests-on): added context words; ids of the shared pair split by the hits / keeps none
Big vehicle: exact none / in context vehicles, 8 vehicles (R1-XU-OQ1 restates R1-ECS-REC1), large units, vehicles and harvesters / keeps big
Live projectile: exact none / in context 200 projectiles, projectiles alive / keeps none
Active effect: exact none / in context effects, 200 active effects / keeps none
The authority: exact none / in context server-only / keeps authority-only
The prediction scope: exact none / in context predict only..., partial prediction / keeps none
Shared and authority-only stream: exact server-only random stream (1, R2-SYNC-REC3, wrong) / in context the simulation's seeded RNG / keeps server-only keyed random stream
State hash: exact none / in context its hash / keeps none
Preview: exact none / in context deploy preview (3 hits, all Netlify's product: right), preview deployment / keeps none
The client host: exact 'Cloudflare (Workers static assets; Pages as the same-headers alternative)' (0: r6:609-610 line break hides the one wrong use) / in context Cloudflare / keeps none
The staging game server: exact none / in context a real server, an EU server / keeps none
The access wall: exact none / in context the access policy, Cloudflare Access, private previews, login wall (added: as a name for the access wall; its 1 hit r1:3696 is a site's login wall, right) / keeps none
Preview isolation: exact none / in context COOP and COEP..., when R1-BND-REC5's option G3 is on (unbroken), always / keeps none
Header sets: exact none / in context a preview-only header set / keeps none
The security baseline: exact none (all 9 hits wrong, but the 'as different names' clause keeps them in context per the advisor's rule) / in context R6's deferred baseline, the deferred baseline, R6's security baseline / keeps the security baseline
The advisories check: exact audit gate (1, r2:2577 R2-TRN-REC1 mark 5, wrong) / in context cargo-audit and cargo-deny as CI gates, dependency audits / keeps none
Game-server certificate: exact none / in context a publicly trusted certificate / keeps none
Hash-map ban: exact none / in context hash maps serve lookups only, zero uses ... outcome code (unbroken; the ellipsis never matches) / keeps none
Verification (2026-10-30): 0 unsplit Retire fields; 34 entries each with Retire (exact) and Retire (in context); 7 Keeps clauses (+1 in How to read). retire_split_compare.py: 34 entries, 0 problems (8 phrases rejoined across a line break). Quoted occurrences before 111 = after 103 + 8 kept. glossary_check.py --skip-records after: exact hits 40 (all judged wrong in the hit review), in context 683.
Counts: exact 30 phrases; in context 73 quoted occurrences (70 distinct: tick time x3, 48 x2); keeps 8.
Open for the session: the check matches literal spaces, so a phrase split by a line break in a topic file is missed (r4:691-692 half of that mark's mean tick; r6:609-610 the Cloudflare old order): match \s+ between words.
DONE
