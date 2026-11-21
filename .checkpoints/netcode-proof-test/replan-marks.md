# Re-plan mark table under NPT-TRIM (test-engineer, 2026-11-21; read only, no build)
Plan (block 0): list the 81 marks (marks.json, node one-liner); place each by plan.md §6.2/§7.2/§12.1 and the U4-U6
briefs; read judged results from the set reports; then blocks 1-6. Status: blocks 1 done; next block 2.
Rule: NPT-TRIM, history.md:7561-7593 (KEEP :7569-7574; MOVE :7576-7581; the table and MOVED :7585).

## 1. The table (81 marks, marks.json order)
Key. Rev 3.3 column: unit; run (plan.md §6.2 line). U4 rows: U4r u4-build-plan.md:781-834, U4f :836-853, U4f2
:855-879, U4g :881-901, U4h :903-918; U5-U8 plan.md:1252-1255. Destinations: **CS7** Slice 1 change-set 7 Lag
(docs/reports/2026-11-17-build-plan-slice-1.md:386-397), **CS8** Shoot (:399-413), **CS9** Throw (:415-419), **S3**
Slice 3 (transports, crate choice, lag compensation stay out until it: :895, :899), **S6** slice 6 on the real sim
(SPIKE_PLAN.md:633-635), **SB** the security baseline before the first online playtest (SPIKE_PLAN.md:631), **RP** the
real protocol and its dependency checks (history.md:7578). "Safari later" = BROWSER-SET (a) (:7581). ML parts are
judged only on the named laptop (R8; plan.md:717, :760-762); ML\* is rough, never judged. L1 = this container.

| Mark | In plain words | Rev 3.3 | Judged so far | NPT-TRIM | FLAG |
|---|---|---|---|---|---|
| -1 | Rig named, measuring unit holds | U1, U6; every run (:722) | not run | KEEP: U6, U8; R1, R2, R3, R5. Parts RD (with -43) and MAC MOVED: S3, Safari later | |
| -2 | The load is the stress scene | U4, U6; every run (:722) | L1 load only: SS PASS on BRAWL-1 and U4f sets (advisor-brawl1-sets-p0.md:25; u4f-report.md:599); ES FAIL 50.05 vs 10.5 events/s (u4r-report.md:308) | KEEP: all kept runs; ES needs carry 93 (F10) | F10 |
| -3 | Network profiles are what they say | U2, U6; C, R1, every L2 block, R7's MP (:723) | L1 P1-P4, B1 PASS (results/c-u2-pm-0001-3/81c6d01e371b/verdict.json); L2, MP NOT-RUN | KEEP: R1, R2, R3 probes. Part "MP at P2" MOVED with R7: S3 | |
| -4 | Server tick budget holds at 30 Hz | U6; R2 (:724) | not run | KEEP: U6; R2 (STAND-IN until S6, marks.json notes) | |
| -5 | Its two shares hold, each alone | U6; R2 (:724) | not run | KEEP: U6; R2 (server share holds the rewind queries) | |
| -6 | Snapshot encoding fits | U6; R2 (:724) | not run | KEEP: U6; R2 | |
| -7 | Lag compensation stays cheap and small | U4h span, U6; R2 (:724) | not run | KEEP: U6 + block 4's piece; R2 | F9 |
| -8 | Interest picks under the cap, in time | U6; R2 (:724) | not run | KEEP: U6; R2 | |
| -9 | Enclosure test fits, re-flood included | U6; R2 (:724) | not run | KEEP: U6; R2 | |
| -10 | Sync cost grows with what clients see | U6; R2 N=24, 36 (:725) | not run | KEEP: U6; R2 (its N=36 is R2's scale step, not R5's search) | |
| -11 | Transport runtime fits beside the tick | U6; R2 (:724) | not run | KEEP: U6; R2 (one crate: -56 item 1 narrows, F2) | |
| -12 | Downstream fits per client | U6; R2 ES, SS (:724) | not run | KEEP: U6; R2. ES part needs carry 93 (F10); -54's E3b may re-open it (F7) | F10 |
| -13 | Upstream fits per client | U6; R2 P2, P4 (:724) | not run | KEEP: U6; R2 | |
| -14 | No datagram is ever too big | U6, U7; R1, R2, R4, R7 (:726) | not run | KEEP: R2 (S30), R1 browser sessions. MAC part MOVED: Safari later | |
| -15 | A room's memory stays bounded, flat | U6; R2 (:724) | not run | KEEP: U6; R2 | |
| -16 | A room starts fast | U6; every run, first R1 (:727) | not run | KEEP: R1, R2, R3, R5 | |
| -17 | Every decoded snapshot is the server's | U4r, U6; C + R3 (:728) | L1 P0, P2, P3 PASS (advisor-brawl1-sets-p0-full.md:226, :233; history.md:3266); L1 B1 INVALID, ruled (u4r-report.md:545) | KEEP: L1 B1 by U4's B1 row (block 3); R2 (every S30); R3 L2 | |
| -57 | Decoded values keep protocol precision | U4r; C (:729) | PASS, L1 P0/P2/P3/B1 (u4r-report.md:543) | KEEP: done (L1) | |
| -58 | Every counter survives its wrap | U4g; C (:729; u4-build-plan.md:891-894) | not run | lean KEEP: L1 by U4's -58 row (block 3) | F1 |
| -18 | A loss burst heals | U4r→U4g, U6; C + R3 (:728) | L1 B1 INVALID, ruled (u4r-report.md:545) | KEEP: L1 B1 by U4's B1 row; R3 L2 B1 wt, ws | |
| -19 | Mid-match join or resync gets world fast | U6; R3 LG + R7 ML (:730) | not run | KEEP: R3 LG joins (after C-U6-11/-13/-16/-19). ML Chrome part MOVED with R7: S3 | |
| -20 | Commands arrive in time, count once | U4f, U6; C + R3 (:728) | L1 A and B PASS 3/3, 46ca143 (u4f-report.md:654-655); L2 NOT-RUN | KEEP: R3 L2 P3, both options | F13 |
| -21 | Forged commands, lag switches change nothing | U4f2; C (:729) | not run (U4f2 building) | KEEP: U4f2; L1 (-21's set) | |
| -22 | A never-acker cannot grow the server | U4f; C (:729) | PASS 3/3 (u4f-report.md:685-689) | KEEP: done (L1) | |
| -23 | A throttled link degrades by priority | U6; R3 (:731) | not run | KEEP: U6; R3 | |
| -24 | A bottleneck never stacks stale snapshots | U6; R3 (:731) | not run | KEEP: U6; R3 | |
| -25 | Hostile input contained on both transports | U6, U7; R4 (:733) | not run | MOVED: SB (R4, history.md:7579) | |
| -26 | A connection's panic ends only it | U6; R3 (:732) | not run | KEEP: U6; R3 | |
| -59 | A tick-thread panic ends match, process | U6; R3 (:732) | not run | KEEP: U6; R3 | |
| -27 | Rooms share a VM fairly | U6; R5 D1-D3 (:734) | not run | KEEP: U6; R5 D1, D2, D3 | |
| -28 | Each WebTransport crate works in every browser | U6, U7; ML\* loopback, R4 MAC-r (:735) | not run | lean KEEP: carried crate, ML loopback after U7; other crate MOVED: S3 (:895); Safari later | F2 |
| -29 | Fallback takes over when QUIC blocked | U6, U7; R4 (:736) | not run | lean KEEP: R1 (+0.25 h); Safari later | F3 |
| -30 | Both transports give the same server | U6; R3 + C loopback (:732) | pre-check recorded, not judged; NOT-RUN until U6 (u4-build-plan.md:827) | KEEP: L1 by U6d B3 + U6j J1 (u6-brief.md:35-36, :57-58); R3 | |
| -31 | Fallback's stalls stay short | U6; R3 LG + R7 ML (:731, :737) | not run | KEEP: R3 LG. ML Chrome part MOVED with R7: S3 | |
| -32 | One build runs four browsers, both backends | U7; R7 (:737) | not run | MOVED: CS6 measures both backends (:381-382), browsers online S3; Safari later. R7's; U7 draws nothing | |
| -33 | Client's RTT agrees with the server's | U7; R7 (:737) | not run | MOVED: S3 (needs WebTransport's rtt(); R7's) | |
| -34 | No decoder panics or over-allocates | U5; C (:738) | not run (U5 stopped, built kept) | MOVED: RP (:7578) | |
| -35 | Transport stays swappable and current | U5, U6; C (:738) | not run | MOVED: RP (:7578) | |
| -36 | Datagram size measured per browser | U7; laptop before R1, R1 (:739) | not run | KEEP: ML loopback legs after U7; R1 EU path. MAC-b part MOVED: Safari later | |
| -37 | Datagrams arrive in time at P2 | U6; R3 (:731) | not run | KEEP: R3 | |
| -38 | One-shots survive loss; bulk spares control | U6; R3 (:731) | not run | KEEP: R3 | |
| -39 | The 60 Hz margin | outside (:747) | outside | MOVED (already outside): S6's 60 Hz run | |
| -40 | The research's other 60 Hz marks | outside (:747) | outside | MOVED (already outside): S6 | |
| -63 | Goal budget at 60 Hz rooms per VM | outside (:747) | outside | MOVED (already outside): S6 | |
| -41 | Prediction exact when commands arrive in time | U4g; C + R7 (:740) | not run | MOVED: CS7 (:395) | |
| -42 | Corrections stay small in a crowd | U4g; C + R7 (:740) | not run | MOVED: CS7 (48 scripted clients: :392) | |
| -60 | Server change to own state predicted next snapshot | U4g; C + R7 (:740) | not run | MOVED: CS7 (movement), CS8 (fire timing: :404) | |
| -43 | Own moves, shots feel same at any latency | U7; R7 ML, RD (:741) | not run | MOVED: CS7 (:253), CS8 for shots; RD part when RD is named | |
| -44 | The replay fits the minimum laptop | U7; R7 (:741) | not run | MOVED: CS7 (the replay's cost in the Worker: :395-396) | |
| -45 | Remote soldiers drawn without guessing | U4g2; C + R7 (:740) | not run | MOVED: CS7 (interpolation: :388) | |
| -46 | Delay adapts; respawns never slide | U7; R7 (:742) | not run | MOVED: CS7 (delay), CS8 (respawns: :403) | |
| -61 | Remote events play with their poses | U4g2; C + R7 (:740) | not run | MOVED: CS8 (events: :403) | |
| -47 | The client's clock is right | U4f; C (:729) | PASS 3/3, a4cac39 (u4f-report.md:564-573) | MOVED: CS7 (clock estimate: :388), his list (:7577); L1 PASS kept as record | |
| -48 | Hits register as the shooter saw them | U4h; C + R3 test cap (:743) | not run | MOVED: S3 (:899) | |
| -49 | Rewind cap holds; clipping is known | U4h; C + R3 + R7 (:743) | not run | MOVED: S3 (:899) | |
| -50 | A cheat cannot buy rewind | U4h; C (:729) | not run | MOVED: S3 (:899) | |
| -51 | Aiming at height hits the clicked point | U4h; C + R7 (:742) | not run | MOVED: CS8 (hitscan: :402), lagged parts S3 | |
| -52 | Hit feedback waits for the server | U7; R1, R4, R7 (:742) | not run | MOVED: CS8 (hit marker on confirmation: :405), browsers S3. R7's; U7 draws nothing | |
| -62 | Own shot cues play once on replay | U4g; C + R7 (:740) | not run | MOVED: CS8 (own weapon state predicted: :404) | |
| -53 | Mustafa's verdict at P2 | R7 (:744) | not run | MOVED: S3 (R7, :7577) | |
| -54 | Client projectile flight holds, or E3b | U4h2; C + R3 (:745) | not run | lean MOVED: CS9 (shared trajectory: :416) | F7 |
| -55 | Soldier beside moving vehicle barely corrected | U4g; C (:745) | not run | lean MOVED: where vehicles enter (SPIKE_PLAN.md:629) | F8 |
| -56 | The proof test's records are complete | all; every run (:746) | not run | KEEP: every kept run; items 5-10 and item 1's pick move | F11 |
| PM-0004-67 | The server loop keeps time | U4b, U6; R2 (:748) | not run | KEEP: R2 | |
| PM-0006-11 | Reference server exists, ran every row | U6, U8; every run (:749) | not run | KEEP: R1, R2, R3, R5 | |
| PM-0006-12 | Round trip to the test servers | U8; R1 ping (:750) | not run (person) | KEEP: R1 (MP and ML pings) | |
| PM-0006-13 | 8 rooms at N = 48 on RS | U6; R5 D1-D3 (:751) | not run | KEEP: R5 | |
| PM-0006-14 | Rooms per VM counted, replaces A-2 | U6; R5, R6 (:752) | not run (reported) | KEEP: R5 N=48. Parts N=36, N=18 MOVED: S6 (:7580) | |
| PM-0006-15 | Firewall admits both transports, nothing else | U8; R4 (:753) | not run | lean MOVED: SB with R4 | F6 |
| PM-0006-16 | Metrics path works with one rule | U6; R5 D1 (:754) | not run | KEEP: R5 (probe from ML: named laptop) | |
| PM-0006-17 | Egress per match-hour matches R2 | U6; R2 (:755) | not run | KEEP: R2 | |
| PM-0006-18 | The spike's own hosting bill | after the last run (:756) | not run (person) | KEEP: after R5 | |
| PM-0006-19 | Logging costs the tick almost nothing | U6; R2 (:755) | not run | KEEP: R2 | |
| PM-0006-20 | Logging allocates nothing on hot path | U6; R2 (:755) | not run | KEEP: R2 | |
| PM-0006-21 | Logs keep no secrets or addresses | U6; R5 soak (:754) | not run | KEEP: R5 D1 soak | |
| PM-0006-22 | Every first metric exists and moves | U6; R5 soak (:754) | not run | KEEP: R5 D1 soak | |
| PM-0006-23 | A stall is visible at once | U6; R5 soak (:754) | not run | KEEP: R5 D1 soak | |
| PM-0006-24 | Metrics small, no player or match id | U6; R5 soak (:754) | not run | KEEP: R5 D1 soak | |
| PM-0006-25 | Dev certificate works in every browser | U6, U7; R4 (:757) | not run | lean KEEP: R1 (+0.2 h); Safari later | F4 |
| PM-0006-26 | Certificate rotates without dropping anyone | U6; R4 (:757) | not run | lean MOVED: SB with R4 | F5 |
| PM-0006-27 | Out-of-rule certificate is refused | U6, U7; R4 (:757) | not run | lean MOVED: SB with R4 | F5 |
