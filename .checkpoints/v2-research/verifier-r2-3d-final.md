# verifier-r2-3d-final: step 3d final check of R2 (netcode), only what the fix round changed (diff f67245e..2382ea2 -- docs/research/r2-netcode.md). Task: prompts/verifier-r2-3d-final.txt
Started 2026-10-31. Notes saved as I go.

## Progress
- [ ] read inputs (first-pass result, skeptic, fix messages, owner replies, diff, hits tsv)
- [x] inputs read: verifier-r2-3d-result.md, skeptic result, prompts/r2-3d-fixes.txt + -2.txt, owner replies (netcode-architect-r2-consistency-result.md addenda), diff f67245e..2382ea2 (59 changed lines at head; +59 -35). Working tree docs == 2382ea2 == HEAD.
- [x] glossary script re-run on 2382ea2 (.gate/r2-3d-final/): R2 exact 0, in context 201 (was 198), header links GLOSSARY; changed_line_hits.py -> 32 hits, IDENTICAL to scripts/glossary-hits-3d-final-r2.tsv.
- [x] item 1 sub-check r2:2338->2339 (BUD-REC2 spike, "the design point, N = 48"): 160 kbps mean rests on the 48-player estimate: CONFIRMED. r2:2332-2333 reason "E3 brings the all-in-view 48-player case ... to about 146 kbps"; worked estimate row r2:1775 "24v24 or 16v16v16 (goal; design point 48) | 48 | 15 | 584 (609 with events) | 30 Hz | **146.1 kbps**". Recomputed: 65+16+47*7.5+15*9.4+9 = 583.5 B; +15.8*48/30 = 609.3 B; *30*8 = 146.2 kbps (+28 B/datagram -> 152.9 kbps vs 160). Label right. Under (b) the mark is a design-point mark, not a 54 mark: by the same formula 54 players (V~16-17) give ~161 kbps payload, ~168 with 28 B, over 160 -> no R2 bandwidth mark covers N = 54; note for synthesis, not a fix-round defect (fix message allowed this option).
- [x] BUD-REC6 reason/spike wording (r2:2402-2405, 2414) checked vs GLOSSARY §1: 50 % of period = 8.3/16.7; 25 % share = 4.2/8.3 (glossary :186-188); "25 % of the tick period for the server tick ... not at 60 Hz on any snapshot tick" = glossary 6.44 (5.24) vs 4.17; non-snapshot 60 Hz tick 4.04 fits. "fit ... at 60 Hz on every tick but one that both encodes and re-floods": holds at 48 via the server-only share (4.41 > 4.17; total 8.28 fits) and at 54 via both (total 8.99 > 8.33); needs "budget" read with its shares (its definition). OK. "p99 server tick time at half the period can double": arithmetic, OK. Spike "its 25 % share of the tick period": = glossary; fixes the 2.1 ms misreading. CLOSED.
- [x] credits: CLAUDE.md:8 "Design for 48+ players; bots fill empty slots." CONFIRMED; VISION_V2.md has 0 "48" (grep), §3 (VISION_V2.md:42-48) holds the modes ("two teams with 2–8 players, and three teams with 6–18 players"; "12v12v12 or 24v24"). All six Serves lines (r2:2289, 2356, 2410, 2628, 2738) + part 7 r2:3898 now credit CLAUDE.md. Remaining "48+" in R2 head: scope/brief/table/Q lines (15, 21, 81, 446, 466(quote), 1382, 1582, 1610, 1617, 1619, 1776, 1902, 1981, 2276, 2495, 2899, 2915, 2918) - none credits VISION_V2 §3. 
- NIT (side effect of fix-2): R2-XU-OQ18 r2:3259-3261 still quotes, present tense, R2-TRN-REC6 serving "the §4 fairness target (about 100 ms) at the §3 scale (48+ players)" and R2-BUD-REC3 "§3 (48+ players), §4 (browsers first)"; at base (r2:2737) TRN-REC6's quote matched exactly; after r2:2738's change it no longer does. Fix: XU-OQ9-style "before 2026-10-31" framing.
- [x] XU-OQ9 past tense (r2:3196-3200): quotes verified at ae492a6 (pre-3c): r2:2385 "one vCPU of the EU instance type R6 recommends", r2:2732 "one 4-vCPU EU cloud VM, R6 to name it", r2:2558 "reference server hardware R6 names", r2:2016/2174/2278 "the proposed reference server". "Before 2026-10-30" right (3c commit 2684735, 2026-10-30). CLOSED.
- [x] part-7 counts: 3d recs 13+2+1+1 = 17; 3d nits 1+3+2+1+2+4 = 13; 94+3+13 = 110; 20+17 = 37; 16 bare 48s = 15 largest-mode + 1 design point. All CONFIRMED. (Owner's reply said "Fourteen marks ... 16 recommendation changes"; the doc's 13+2 is the right one - reply-only slip, not in R2.)

## Check 1: the two fix messages, item by item
- fix 1.1 (16 bare 48s): APPLIED. 15 read "the largest mode's N (48, or 54 under MODES-3 (b))" (r2:2017 linked #the-largest-mode-and-its-n = first use in R2 per --first-uses; anchors on changed lines 3/3 resolve); r2:2292 keeps "48 clients" (quoted at r2:2412) + "(the largest mode's N, or 54 clients under MODES-3 (b)...)"; r2:2339 "the design point, N = 48" (basis confirmed above). Part-7 statement corrected r2:3896-3898. CLOSED.
- fix 1.2 (four shares): APPLIED as worded in the message: r2:2402 "the server tick budget's 50 % of the tick period"; r2:2403 "25 % of the tick period for the server tick"; r2:2405 "a p99 server tick time at half the period"; r2:2414 "its 25 % share of the tick period" (= GLOSSARY:186-188, 4.2 ms at 60 Hz, 8.3 at 30 Hz). Logged 3 nits + 1 rec. CLOSED.
- fix 1.3 (credit): APPLIED r2:2289, r2:3898. CLOSED.
- fix 1.4 (nits): totals r2:3914-3917 right; XU-OQ9 r2:3196-3199 past tense "Before 2026-10-30"; ROOM-REC2 mark 5 "every room start-up" (rec). CLOSED.
- fix 2 (four more credits): APPLIED r2:2356, 2410, 2628, 2738; 4 nits logged r2:3938; totals 110. CLOSED.
- New nit from fix 2: XU-OQ18 r2:3259-3261 (see above).
- GLOSSARY (outside R2, owner engine-architect, nit): "Used today" now stale for R2: GLOSSARY:131 "R2-TRN-REC1 mark 2 caps it for one process with 48 sessions"; GLOSSARY:277 "R2-INT-REC1 (48 clients), R2-TRN-REC1 mark 2 (48 sessions), R2-ROOM-REC2 (a 48-player room)"; largest-mode / design-point Used-today lists lack the 15 new R2 places and R2-BUD-REC2.

## Check 2: the 17 recommendation changes (value holds? number's basis? (a) 48 / (b) 54?)
New numbers in the diff: only 48 / 54 (MODES-3, GLOSSARY:331-337, r2:3294-3301; 2x24 = 48, 3x18 = 54), 50 % / 25 % of the tick period (resolution 1, GLOSSARY:164-188) and part-7 counts. No other number added; Rests-on lines untouched.
1. SYNC-REC1 m1 r2:2017: hash-exact decode, no N-valued limit. Holds (a)(b).
2. SYNC-REC1 m4 r2:2022: N clients' snapshot work within server-only share (25 % of period, N-independent). At 54 GLOSSARY §1 check: server-only 3.85 ms < 4.17 at 60 Hz. Holds.
3. SYNC-REC3 m2 r2:2103: 5 cm / 1 % proposed, N-independent. Holds.
4. SYNC-REC5 m4 r2:2186-2187: 20 µs p99 per shot holds at 54 (proposed). BUT "at 700 shots per second" is the 48-player full-auto bound (r2:2280 "at most about 700 at 48"; r2:459 "about 670 ... at 48 players"; r2:3175 XU-OQ7 "the upper bound 'at 48 players all on full auto'"). Under (b) it pairs 54 candidates with 48's shot rate; GLOSSARY §1's N = 54 check scales per-shot marks x1.125 (~790/s). Needs a scaling it does not state. NIT (wording; the 20 µs value holds; the budget check already scales).
5. SYNC-REC9 m1 r2:2292-2293: within server-only share at p99; at 54 fits per §1 check (3.85 < 4.17). Holds; reads right under both ("48 clients (the largest mode's N, or 54 clients under (b)").
6. BUD-REC1 spike r2:2320: rewind cap, missing-input 0.1 %, upstream 48 kbps per client, fire rate: all per-client/per-weapon, N-independent. Holds.
7. BUD-REC3 spike r2:2359: 0 oversize datagrams (split path handles any N), join 300 ms proposed. By R2's own full-record formula (r2:2353) 54 projectile-free ~1.1 kB < 1171; holds.
8. INT-REC1 (c) r2:2440: 0.2 ms p99 "proposed" (no 48 derivation in R2). At 54 an unscaled, 12.5 % stricter per-client ceiling; §1's 54 check scales interest to 0.79 ms "for this check only" and still fits. Holds.
9. INT-REC4 r2:2500: 0.5 ms p99, same as 8. Holds.
10. TRN-REC1 m2 r2:2589: 1 vCPU transport cap, 1 ms p99 per send call, per-session rates: proposed, none derived from 48. Holds at 54 as a cap (its vCPU share enters rooms-per-VM, measured).
11. TRN-REC5 m2 r2:2719: 0 sessions dropped. Holds.
12. ROOM-REC2 m1 r2:2774: p99 within 50 % of period (N-independent). 54 at 60 Hz: §1 check 7.99 ms fits, 8.99 on re-flood ticks over (already stated in GLOSSARY §1, spike to measure). Holds as a measured mark.
13. ROOM-REC2 m3 r2:2778: 64 MB "proposed ceiling", no derivation from 48. Holds (stricter per player at 54).
14-15. SYNC-OQ13 r2:2953, BUD-OQ9 r2:2994: questions, no values. Hold.
16. BUD-REC6 spike r2:2414: "its 25 % share of the tick period" = GLOSSARY definition; N-independent; at 54 60 Hz sim step share 4.14 (0.03 left) per §1. Holds.
17. ROOM-REC2 m5 r2:2779 "every room start-up (process spawn to Open) under 2 s": "Open" = ROOM-REC1 state (r2:2748). The spike as written starts 1 room (m1) + 8 rooms (m2), ~9 start-ups, no restart after m4's SIGKILL. "Every" = a plain limit, the GLOSSARY:143-145 one-off-deadline rule ("a single event ... is a plain limit, not a statistic"); with ~9 samples a p99 is the max anyway; matches its use (r2:2763: option B "only if the spike shows ... start-up cost is too high"). 2 s unchanged proposed. HOLDS.
(design-point) BUD-REC2 r2:2339 (nit, not a rec change): holds at N = 48; under (b) no R2 bandwidth mark covers 54 (R2's formula r2:1764-1766 at P=54, V=15-18: ~158-165 kbps payload, ~164-171 with 28 B/datagram, over 160): a note for the synthesis/MODES-3, not a fix-round defect.
BUD-REC6 reason nits r2:2402-2405: hold (see above).

## Check 3: no new claim, number or source
- Added lines: 0 URLs, 0 finding ids, 0 new r-line refs. Numbers added beyond those removed: 48/54 (MODES-3), 25/50 (resolution 1), 8.3 (part-7 record of the 2.1-vs-4.2 misreading, GLOSSARY value), 160 (part-7 record, existing mark), counts 13/15/16/17/37/110, dates, section/resolution refs. No new value.
- New claims are part-7 records, each confirmed: 160 kbps rests on 48 estimate (r2:1775, 2332-2333); "Design for 48+ players" is CLAUDE.md's (CLAUDE.md:8); VISION_V2 §3 holds the modes (VISION_V2.md:42-48); BUD-REC6 quotes "48 clients" (r2:2412); "a spike of a few rooms has no meaningful p99" (reasoning; ~9 start-ups); "it read as 25 % of the 8.3 ms budget" (skeptic's finding). CLOSED.

## Check 4: the 32 "in context" hits on changed lines (scripts/glossary-hits-3d-final-r2.tsv; my re-run identical). Retire (exact) R2 = 0 (re-run on 2382ea2).
2017 largest-mode 48: right (beside "the largest mode's N" + 54)
2022 largest-mode 48: right (same)
2103 team-size players: right (a match's player count, "the largest mode's N players")
2103 largest-mode 48: right (beside marking words + 54)
2186 largest-mode 48: right (same)
2289 team-size players: right (CLAUDE.md's "design for 48+ players", a match count)
2292 largest-mode 48: right ("48 clients (the largest mode's N, or 54 clients under MODES-3 (b)")
2320 team-size players: right (player count)
2320 largest-mode 48: right (beside marking words + 54)
2339 largest-mode 48: right (means the design point, beside "the design point, N = 48" + its link)
2356 team-size players: right (CLAUDE.md credit, match count)
2359 largest-mode 48: right
2402 server-tick-budget 50 %: right ("the server tick budget's 50 % of the tick period")
2403 server-tick-budget 25 %: right ("25 % of the tick period for the server tick": whole named)
2405 server-tick-budget 50 %: right (same sentence names "the server tick budget's 50 % of the tick period"; parenthesis "half the period")
2405 tick-period "the period": right (sentence names "the tick period" twice; "server tick time ... overruns" cannot mean the snapshot interval)
2410 team-size players: right (CLAUDE.md credit)
2414 authority server-only: right (the server-only share's work, not offline-held state)
2414 server-tick-budget 25 %: right ("its 25 % share of the tick period")
2440 largest-mode 48: right
2440 time-stats "at most 0.2 ms per tick": right ("at p99" follows in the sentence)
2500 largest-mode 48: right
2500 time-stats "at most 0.5 ms per tick": right ("at p99" follows)
2589 largest-mode 48: right
2628 team-size players: right (CLAUDE.md credit)
2719 largest-mode 48: right
2738 team-size players: right (CLAUDE.md credit)
2774 team-size players: right (player count of one room)
2774 largest-mode 48: right
2778 largest-mode 48: right ("the largest mode's N: 48, or 54")
2953 largest-mode 48: right
2994 largest-mode 48: right
Totals: 32 right, 0 wrong. Skipped on changed lines: r2:2412 (quoted "48 clients"), r2:3196-3198 (quoted old wordings, exact phrases inside quotes, as before), part 7 r2:3896-3938 (dated record). CLOSED.

## Check 5: the label
- No Rests-on line changed; no new finding or source. Every changed basis claim is internal and confirmed above (CLAUDE.md:8; VISION_V2.md:43-44 for 48 = 24v24 and 54 = 3 x 18; r2:1775 and 2332-2333; r2:2412; GLOSSARY:138-145, 164-188, 331-337, 350-357). No external fetch needed.
- Label: R2 keeps "verified (every recommendation claim)".

## Host audit
WebFetch 0, curl 0. No host contacted.

## Remaining (all nits)
- SYNC-REC5 mark 4 r2:2187: "at 700 shots per second" unscaled at 54.
- XU-OQ18 r2:3259-3261: present-tense quotes of Serves wordings replaced at r2:2356/2738.
- GLOSSARY:131, 277 and the largest-mode/design-point Used-today lists stale for R2 (outside R2).
- Note (not a defect): under (b) no R2 bandwidth mark at N = 54 (BUD-REC2 is the design point).
DONE
