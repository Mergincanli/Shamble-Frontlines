# Resolutions of the critic's 12 contradictions (step 3a, proposals for the advisor's review)

- **Task:** step 3a of `plan-amendments-2.md` (advisor conditions 1–4 applied). Author: engine-architect, 2026-10-30.
- **Status:** proposals. Nothing here is applied; the advisor reviews each item (step 3b, verdict to
  `advisor-resolutions-verdict.md`), and the owners apply only approved items (step 3c).
- **Inputs:** the six topic files as they stand at commit 8dee746 (line numbers below are of that state), the critic's
  list `engine-architect-consistency-result.json`, VISION_V2 §3, §4 and §6, and the advisor's table "Who decides each
  contradiction" (`advisor-amendments-2-verdict.md`). No page was fetched; no new source is used.
- **Outcomes:** **resolved** (one position holds, from findings the files already hold), **aligned** (one shared open
  question, the same words in every affected topic, one decider and a decision code) or **escalated** (a question for
  Mustafa or the Advisor, with options and a recommendation).
- **Numbers:** every changed number comes from a topic file (its id is given), is arithmetic on those numbers (shown
  under "Arithmetic"), or is a pass mark marked "proposed" with its reason.
- **Glossary:** terms such as "the stress scene" or "the server tick budget" are defined in `docs/research/GLOSSARY.md`.
  In the "after" texts, `(GLOSSARY)` marks a first use, which the owner turns into a link.
- **How the before/after texts work:** "before" quotes the file at the line given, exactly; "after" is the replacement
  for exactly that text. Text outside the quote stays. Where an item depends on an escalated decision, the "after" text
  holds under every option (advisor condition 2).
- **Order of work** (advisor condition 2): #10, #11 and #9 with their glossary terms, then #1; #19 before #21 and #22;
  #5 with #6; then #17, #23, #29. The items below are listed by rank.

## Summary

(Filled in when all 12 are written.)

## Where shared open questions go

An aligned or escalated item becomes one shared open question with the same words in every affected topic. Each topic
gets one new subsection for them, **"### Shared open questions (the same words in every topic)"**, at the end of its
open questions, placed before:
- R1: `### Pointers between R1's units` (r1:2872);
- R2: `### Pointers between the R2 units` (r2:3215);
- R3: `### Pointers to other topics` (r3:1398);
- R4: `### Wave-1 items R4 changes or answers (said openly, not silently)` (r4:1110);
- R5: `### Recommendations that hinge on a source not read` (r5:1889);
- R6: `### Recommendations that hinge on a source not read` (r6:915).

Each shared question is headed by its decision code in bold (for example **MODES-3 (Mustafa).**), so a search for the
code finds every copy (advisor condition 11). A topic's own earlier question on the same point stays and gains one
closing sentence, "Shared as <code> (below).". The critic's other 29 items go in a second new subsection after it,
`### Cross-topic open questions (consistency critic, 2026-10-29)`, with ids `R<n>-XT-OQ<k>` (see `cross-topic-oqs.md`).


| Rank | What clashes | Outcome | Decider and code |
|---|---|---|---|

## 1. What the tick budget covers, and its shares

**Outcome: resolved** (one server tick budget, split into two shares; every existing figure placed inside one of them).
Decider: the advisor (the advisor's table: "Me (engineering), after #9/#10/#11. The 60-or-30 Hz rule stays as it is").
It builds on resolution 9 (the stress scene), 10 (the largest mode's N) and 11 (one vCPU of the reference server).

**Both sides.**
- The sim alone: r1:2014-2018 (R1-TICK-REC2 P1) "At 60 Hz: p99 tick time at N = 48 at most 4.2 ms (25 % of 16.7 ms)
  … The 25 % leaves room for snapshot encoding, interest management and several matches per core (R2, R6)"; r1:2025
  "Decision rule: 60 Hz if P1–P3 pass at 60 Hz"; r1:1908-1909 (R1-ECS-REC1 mark 1) "a mean tick ≤ 2 ms and a p99 tick
  ≤ 4 ms native"; r1:2394-2396 (R1-ABL-REC6) "at most 1 ms per tick"; r4:691-692 (R4-PHYS-REC1 mark 2) "at most half of
  that mark's mean tick (≤ 1 ms native mean)"; r4:841 (R4-NAV-REC2) "≤ 10 % of the tick's p99 budget".
- The whole or other parts: r2:2377-2379 (R2-BUD-REC6) "for the whole tick, not just the simulation step: at 48
  players, p99 ≤ 25 % of the tick interval (≤ 4 ms at 60 Hz, ≤ 8 ms at 30 Hz)"; r2:2284 (R2-BUD-REC1) "60 Hz if R1's
  tick spike meets its CPU mark"; r2:2733 (R2-ROOM-REC2) "p99 tick time at most 50 % of the tick period"; r2:2276-2278
  (R2-SYNC-REC9) sync work "at most 25 % of the tick budget"; r2:2447-2448 (R2-INT-REC3) "at most 10 % of the server
  tick budget"; r2:2558 (R2-TRN-REC1 mark 2) "at most 1 CPU core for the transport layer"; r6:512 (R6 A-2) "8 rooms of
  48 players per 4-vCPU VM".
- Existing questions: R2-XU-OQ8 (R2's shares only) and R1-XU-OQ1 (R1's units only).

**The proposal (GLOSSARY: the server tick budget; the sim step share and the server-only share).**
- One total: in the stress scene, on one vCPU of the reference server, the p99 server tick time is at most **50 % of
  the tick period** (8.3 ms at 60 Hz, 16.7 ms at 30 Hz), the max under one tick period, zero overruns. This is
  R2-ROOM-REC2's figure.
- Two shares of 25 % of the tick period each (4.2 ms at 60 Hz, 8.3 ms at 30 Hz): the **sim step share**
  (R1-TICK-REC2 P1's figure, unchanged) and the **server-only share** (R2-SYNC-REC9's figure, read against the tick
  period: command handling, lag-compensated hit tests, interest, snapshot encoding, the tick's send calls).
- Inside each share, every topic keeps its own mark, now stated against the defined whole and reported separately:
  movement and physics ≤ 1 ms mean (R4-PHYS-REC1), abilities ≤ 1 ms (R1-ABL-REC6), navigation ≤ 10 % of the budget =
  5 % of the period (R4-NAV-REC2), the visibility pass ≤ 10 % of the budget = 5 % of the period (R2-INT-REC3) in the
  sim step share; encoding ≤ 50 µs per client (R2-BUD-REC6), a rewound shot ≤ 20 µs (R2-SYNC-REC5), the region query
  ≤ 0.2 ms (R2-INT-REC1), ranking and packing ≤ 0.5 ms (R2-INT-REC4) in the server-only share.
- The network runtime's own CPU (encryption, sockets) runs beside the tick, not in it (R2-ROOM-REC2: "network I/O on an
  async runtime"); it is the transport runtime time (GLOSSARY), capped by R2-TRN-REC1 mark 2 and counted in the
  rooms-per-VM figure, not in the tick budget.
- The 60-or-30 Hz rule stays word for word (R1-TICK-REC2: "60 Hz if P1–P3 pass at 60 Hz; otherwise 30 Hz if they
  pass at 30 Hz …"; R2-BUD-REC1: "60 Hz if R1's tick spike meets its CPU mark, else 30 Hz"). P1's CPU mark is the sim
  step share, as today; R2-BUD-REC6 checks the whole at the rate the rule picks.
- A time mark that names no statistic is a p99 (GLOSSARY: time statistics).

**Arithmetic: do the parts fit?** Each part at its own ceiling; sums of p99s are an upper bound. Snapshots run at 30 Hz
(R2-BUD-REC1), so at a 60 Hz tick every second tick encodes, at 30 Hz every tick. Tick period 16.67 ms (60 Hz), 33.33 ms
(30 Hz); 25 % = 4.17 or 8.33 ms; 5 % = 0.83 or 1.67 ms.

| Part (N = 48) | Mark and id | 60 Hz, ms | 30 Hz, ms |
|---|---|---|---|
| movement and physics | ≤ 1 ms mean, R4-PHYS-REC1 mark 2 | 1.00 | 1.00 |
| abilities, effects, weapons | ≤ 1 ms, R1-ABL-REC6 | 1.00 | 1.00 |
| navigation | 5 % of the period, R4-NAV-REC2 | 0.83 | 1.67 |
| visibility pass | 5 % of the period, R2-INT-REC3 | 0.83 | 1.67 |
| **sim step share, sum (limit)** | | **3.67 (4.17)** | **5.33 (8.33)** |
| snapshot encoding | 48 × 50 µs, R2-BUD-REC6 | 2.40 | 2.40 |
| lag compensation | 700 shots/s × 20 µs = 14 ms/s, R2-SYNC-REC5 mark 4 | 0.23 | 0.47 |
| interest | 0.2 + 0.5 ms, R2-INT-REC1, R2-INT-REC4 | 0.70 | 0.70 |
| **server-only share, sum (limit)** | | **3.33 (4.17)** | **3.57 (8.33)** |
| **server tick, sum (limit 50 %)** | | **7.00 (8.33)** | **8.90 (16.67)** |

What is left: at 60 Hz, 0.50 ms in the sim step share for the rest of the sim (AI decisions, projectiles, economy,
objectives, moments) and 0.83 ms in the server-only share for command handling and the send calls; at 30 Hz, 3.00 ms
and 4.77 ms. At N = 54 (MODES-3 (b)), scaling the per-soldier, per-client and per-shot marks by 54 / 48 = 1.125 (an
assumption for this check only; the marks are set at 48): sim step 1.125 + 1.125 + 0.83 + 0.83 = 3.92 ms at 60 Hz
(5.58 at 30 Hz); server-only 2.70 + 0.26 + 0.79 = 3.75 ms at 60 Hz (2.70 + 0.53 + 0.79 = 4.02 at 30 Hz): both still
inside 4.17 ms (8.33 ms), so the proposal holds under either reading of MODES-3.

The tightest part is the visibility pass: R2-INT-F20's legacy JavaScript figure for a three-point pass of 768 pairs is
6–13 ms (unindexed, per pass), against 0.83 ms per tick at 60 Hz here; the pass is amortized across ticks
(R2-INT-REC3), and a Rust cast with a spatial index has not been measured (R2-INT-F20). The spike measures it; the
proposal does not hide it.

**The option the advisor may prefer instead: 25 % for the whole tick (R2-BUD-REC6's figure).** The whole server tick
≤ 4.17 ms at 60 Hz, 8.33 ms at 30 Hz, with R2-INT-REC3's and R4-NAV-REC2's "10 %" then 2.5 % of the period (0.42 or
0.83 ms). Arithmetic at N = 48: at 60 Hz the sim parts are 1 + 1 + 0.42 + 0.42 = 2.83 ms and the server-only parts on a
snapshot tick 3.33 ms, 6.17 ms in all, over 4.17 ms; even with encoding spread over two ticks (proposed in no topic)
2.83 + 0.23 + 0.70 + 1.20 = 4.97 ms, still over. At 30 Hz: 3.67 + 3.57 = 7.23 ms, inside 8.33 ms. So under 25 % the
topics' own marks cannot meet the budget at 60 Hz, and the 60-or-30 rule would land on 30 Hz by arithmetic before
P1–P3 are measured. Its gain: two rooms per vCPU at the ceiling use half of it, leaving the other half for the
transport runtimes and the system, so R6's A-2 would follow from the budget.

**Why 50 % (the recommendation).**
1. The marks the topics already set add up inside it at 60 Hz and 30 Hz, at N = 48 and N = 54 (table above); under
   25 % they do not at 60 Hz, which would decide the tick rate before the spike the rule waits for.
2. It changes the fewest marks. R1-TICK-REC2 P1's 25 %, R2-SYNC-REC9's 25 % and R2-ROOM-REC2's 50 % keep their
   numbers. R2-BUD-REC6's whole moves from 25 % to 50 %, R1-ECS-REC1 mark 1's p99 from 4 ms to 4.2 ms (R1-TICK-REC2's
   arithmetic for 25 % of 16.7 ms; R1-XU-OQ1 already lists the two) with its 2 ms mean now reported, and the nested
   "10 %" marks get their value (5 % of the period). R4-PHYS-REC1 keeps its 1 ms.
3. It keeps room for bursts: a p99 tick at half the period can double before it overruns (arithmetic), and the max
   stays under one period with zero overruns (R1-TICK-REC2 P1).

**What it costs, said openly.** The budget no longer promises more than one room per vCPU: at the ceiling two rooms
fill a vCPU at p99 before the transport runtime (arithmetic). R6's A-2 (2 rooms per vCPU) becomes a density to measure
(R2-ROOM-REC2 mark 2, R6-COST-OQ1), not one the budget guarantees; if it fails, the compute per room-hour doubles
(R6's own rows: 0.0173 against 0.0347 EUR, r6:452-453). The density is measured at the tick rate the rule picks,
because 30 Hz "roughly halves the CPU per simulated second" (R1-TICK-REC2, inferred). R1-TICK-REC2's "several matches
per core" and R2-BUD-REC6's "more than one room per core" become "measured, not budgeted". R2-TRN-REC1 mark 2's
"1 vCPU for the transport layer" for one process cannot sit beside 2 rooms per vCPU either; its measured share per room
enters the same density figure.

*Changes per file (before → after).* Line numbers are of commit 8dee746; resolutions 9 and 11 change other parts of
some of the same sentences (their substrings do not overlap with these).

R1:
- R1-R1-1, r1:1908-1909 (R1-ECS-REC1 mark 1):
  - before: `The pass mark: a mean tick ≤ 2 ms and
     a p99 tick ≤ 4 ms native;`
  - after: `The pass mark: a p99 sim step time within the sim step share (GLOSSARY: 25 % of the tick period, 4.2 ms at
     60 Hz and 8.3 ms at 30 Hz, the mark of R1-TICK-REC2 P1), the mean reported, native;`
- R1-R1-2, r1:2015 (R1-TICK-REC2 P1):
  - before: `4.2 ms (25 % of 16.7 ms)`
  - after: `4.2 ms (the sim step share, GLOSSARY: 25 % of 16.7 ms)`
- R1-R1-3, r1:2017-2018 (R1-TICK-REC2 P1):
  - before: `The 25 % leaves room for snapshot encoding, interest management and several matches per core (R2, R6); headroom
  itself is Fiedler's advice (F3).`
  - after: `The other 25 % of the server tick budget (GLOSSARY: 50 % of the tick period) is the server-only share, for
  snapshot encoding, interest management, lag compensation and commands (R2-SYNC-REC9); how many matches share a vCPU
  is measured, not budgeted (R2-ROOM-REC2 mark 2); headroom
  itself is Fiedler's advice (F3).`
- R1-R1-4, r1:2136 (R1-DET-REC3, spike):
  - before: `it fits the per-tick sim budget the tick-rate unit sets.`
  - after: `it fits inside the sim step share (GLOSSARY).`
- R1-R1-5, r1:2396 (R1-ABL-REC6):
  - before: `costs at most 1 ms per tick`
  - after: `costs at most 1 ms per tick at p99, inside the sim step share (GLOSSARY),`
- R1-R1-6, r1:2546-2547 (R1-ECS-OQ4):
  - before: `Decided by: REC1 mark 1's
  native tick time against the tick period the tick unit picks, and R6's hosting cost per match.`
  - after: `Decided by: REC1 mark 1's
  native sim step time against the sim step share (GLOSSARY) at the tick rate the tick unit picks, and R6's hosting
  cost per match.`
- R1-XU-OQ1: settled by R9-8's closing sentence (one stress scene and one server tick budget).

R2:
- R1-R2-1, r2:2016-2017 (R2-SYNC-REC1 mark 4):
  - before: `stays within the share of the tick budget the tick-rate unit sets.`
  - after: `stays within the server-only share (GLOSSARY: 25 % of the tick period, shared with lag compensation,
     interest, commands and the tick's sends).`
- R1-R2-2, r2:2276-2278 (R2-SYNC-REC9 mark 1):
  - before: `Snapshot
     encoding, lag compensation and command handling together use at most 25 % of the tick budget at the 99th
     percentile on the proposed reference server.`
  - after: `Snapshot
     encoding, lag compensation, command handling, interest (the region query, ranking and packing) and the tick's
     send calls together stay within the server-only share (GLOSSARY: 25 % of the tick period) at the 99th
     percentile on one vCPU of the reference server.`
- R1-R2-3, r2:2377-2381 (R2-BUD-REC6, heading and reason):
  - before: `A server CPU budget per tick for the whole tick, not just the simulation step: at 48 players, p99 ≤ 25
  % of the tick interval (≤ 4 ms at 60 Hz, ≤ 8 ms at 30 Hz) including the simulation (R1), lag-compensated hit tests
  and snapshot encoding for every client, measured separately.** Reason: no source read gives a tick budget; 25 %
  leaves room for bursts and for more than one room per core; per-client encode cost is the unknown that grows with
  player count.`
  - after: `A server CPU budget per tick for the whole tick, not just the simulation step: at the largest mode's N,
  p99 server tick time within the server tick budget (GLOSSARY: ≤ 50 % of the tick period, ≤ 8.3 ms at 60 Hz,
  ≤ 16.7 ms at 30 Hz), split into the sim step share (R1) and the server-only share (lag-compensated hit tests,
  interest, snapshot encoding for every client, commands and the tick's sends), 25 % each, measured separately.**
  Reason: no source read gives a tick budget; the topics' own part marks add up inside 50 % at both tick rates, while
  25 % for the whole tick would not hold them at 60 Hz (the arithmetic is in GLOSSARY's tick-budget section); 50 % leaves
  room for bursts (a p99 tick at half the period can double before it overruns); how many rooms share a vCPU is
  measured (R2-ROOM-REC2 mark 2), not budgeted; per-client encode cost is the unknown that grows with player count.`
- R1-R2-4, r2:2385-2387 (R2-BUD-REC6, spike):
  - before: `48
  players, 30-minute soak: p99 whole tick ≤ 4 ms at 60 Hz (or ≤ 8 ms at 30 Hz), 0 missed ticks,`
  - after: `the
  stress scene (GLOSSARY), 30-minute soak: p99 server tick time ≤ 8.3 ms at 60 Hz (or ≤ 16.7 ms at 30 Hz), the sim
  step and the server-only work each within their 25 % share, 0 missed ticks,`
- R1-R2-5, r2:2447-2449 (R2-INT-REC3, spike):
  - before: `takes at most 10 % of the server tick budget on the reference server (the tick rate comes
    from the tick unit)`
  - after: `takes at most 10 % of the server tick budget (GLOSSARY: 5 % of the tick period, 0.8 ms at 60 Hz and 1.7 ms
    at 30 Hz, inside the sim step share) on one vCPU of the reference server (the tick rate comes
    from the tick unit)`
- R1-R2-6, r2:2733 (R2-ROOM-REC2, spike):
  - before: `p99 tick time at most 50 % of the tick period chosen by the tick-rate unit,`
  - after: `p99 server tick time within the server tick budget (GLOSSARY: 50 % of the tick period chosen by the tick-rate
  unit),`
- R1-R2-7, r2:2559 (R2-TRN-REC1 mark 2), a sentence added after the mark's last sentence, before `3. Swap cost:`:
  - after: `The transport runtime (GLOSSARY) runs beside the tick, not in it; its measured share of a vCPU per room enters
   the rooms-per-VM figure (R2-ROOM-REC2 mark 2, R6-COST-OQ1).`
- R1-R2-8, r2:3140-3141 (R2-XU-OQ8), at the end of the entry:
  - before: `Decided by:
  the synthesis, which sets one tick budget and divides it; then the spikes.`
  - after: `Decided by:
  the synthesis, which sets one tick budget and divides it; then the spikes. Settled by the shared glossary
  (2026-10-30): one server tick budget, 50 % of the tick period, split into a sim step share and a server-only share
  of 25 % each, every figure above placed inside one of them (GLOSSARY).`

R4:
- R1-R4-1, r4:691-692 (R4-PHYS-REC1 mark 2):
  - before: `take at most half of that mark's
     mean tick (≤ 1 ms native mean),`
  - after: `take at most 1 ms native mean, with the
     p99 reported, inside the sim step share (GLOSSARY),`
- R1-R4-2, r4:841 (R4-NAV-REC2, spike):
  - before: `takes ≤ 10 % of the tick's p99 budget`
  - after: `takes ≤ 10 % of the server tick budget at p99 (GLOSSARY: 5 % of the tick period, inside the sim step
  share)`
- R1-R4-3, r4:954 (R4-LOS-REC1 mark 4):
  - before: `inside its 10 % budget.`
  - after: `inside its share (GLOSSARY: 5 % of the tick period).`
- R1-R4-4, r4:970 (R4-LOS-REC2, spike):
  - before: `within R2-INT-REC3's 10 % budget`
  - after: `within R2-INT-REC3's share (GLOSSARY: 5 % of the tick period)`

R5:
- R1-R5-1, r5:1472-1473 (R5-TEST-REC5):
  - before: `Measured: server tick p50,
  p99 and max (against R1-TICK's budget),`
  - after: `Measured: server tick time p50,
  p99 and max (against the server tick budget, GLOSSARY),`
- R1-R5-2, r5:1481 (R5-TEST-REC5, spike):
  - before: `and tick p99 inside the budget ADR-0004 sets;`
  - after: `and server tick time p99 inside the server tick budget (GLOSSARY; ADR-0004 and ADR-0001 adopt it);`

R6:
- R1-R6-1, r6:512 (A-2):
  - before: `- **A-2.** 8 rooms of 48 players per 4-vCPU VM (R2-ROOM-REC2's proposal); smaller rooms fit at least as many.`
  - after: `- **A-2.** 8 rooms of 48 players per 4-vCPU VM (R2-ROOM-REC2's proposal); smaller rooms fit at least as many.
  It is a density to measure at the tick rate R1-TICK-REC2's rule picks (R2-ROOM-REC2 mark 2), not one the server tick
  budget guarantees: at the budget's ceiling (GLOSSARY: 50 % of one vCPU per room at p99) two rooms fill a vCPU before
  the transport runtime, so the 4-rooms-per-VM row (one per vCPU) is the case to budget if the measured ticks do not
  sit well below the ceiling (R6-COST-OQ1). At 30 Hz the CPU per simulated second roughly halves (R1-TICK-REC2,
  inferred).`
- R1-R6-2, r6:850-851 (R6-COST-OQ1):
  - before: `Decided by: R2-ROOM-REC2's spike on the reference server (R6-HOST-REC2).`
  - after: `Decided by: R2-ROOM-REC2's spike on the reference server (R6-HOST-REC2), at the tick rate R1-TICK-REC2's rule
  picks, with each room's transport runtime counted (GLOSSARY).`

*Combined view of R1-TICK-REC2 P1 with resolutions 1, 9 and 11 and the glossary's "sim step time" applied* (for the
advisor; the owner applies the parts one by one):

> P1 server, native: the stress scene (GLOSSARY) at N = 12 / 24 / 36 / 48 / 64 soldiers and at the largest mode's N
> where it is not among them (MODES-3) (bots with full AI, worst case: all in one combat cluster), with its 15 big
> vehicles, 300 live projectiles and up to 8 active effects per soldier, on a DM_Tri-sized greybox, 36,000 ticks per
> run, 3 runs. At 60 Hz: p99 sim step time at the largest mode's N at most 4.2 ms (the sim step share, GLOSSARY: 25 % of
> 16.7 ms) on one vCPU of the reference server, max at most 16.7 ms, zero overruns. At 30 Hz: p99 at most 8.3 ms (25 %
> of 33.3 ms). Report mean, p50, p95, p99 and max per N. The other 25 % of the server tick budget (GLOSSARY: 50 % of the
> tick period) is the server-only share, for snapshot encoding, interest management, lag compensation and commands
> (R2-SYNC-REC9); how many matches share a vCPU is measured, not budgeted (R2-ROOM-REC2 mark 2); headroom itself is
> Fiedler's advice (F3).

**Basis ids:** R1-TICK-REC2, R1-ECS-REC1, R1-ABL-REC6, R1-DET-REC3, R1-ECS-OQ4, R1-XU-OQ1, R2-BUD-REC1, R2-BUD-REC6,
R2-SYNC-REC1, R2-SYNC-REC5, R2-SYNC-REC9, R2-INT-REC1, R2-INT-REC3, R2-INT-REC4, R2-INT-F20, R2-TRN-REC1, R2-TRN-REC6,
R2-ROOM-REC2, R2-XU-OQ8, R4-PHYS-REC1, R4-NAV-REC2, R4-NAV-REC3, R4-LOS-REC1, R4-LOS-REC2, R5-TEST-REC5, R6 A-2,
R6-COST-OQ1, the cost rows r6:452-453. **New numbers:** 8.3 and 16.7 ms (50 % of the period), 4.2 and 8.3 ms (25 %),
0.8 and 1.7 ms (5 %), 2.4 ms, 0.23 and 0.47 ms, and every sum in the table are arithmetic (shown); the 50 % total and
the 25 % / 25 % split are figures from R2-ROOM-REC2, R1-TICK-REC2 and R2-SYNC-REC9, combined as a proposal; the 1.125
scaling is an assumption for the check only.


## 5. Whether the client predicts the player's own abilities

## 6. The spread random stream: in the sim state or server-only

## 9. The stress scene

**Outcome: resolved** (one proposed worst-case scene; the game-designer's caps replace its counts later, R2-BUD-OQ5).
Decider: the advisor (the advisor's table: "Me: one proposed worst-case scene. Caps later (R2-BUD-OQ5)").

**Both sides.**
- r1:1907-1908 (R1-ECS-REC1 mark 1): "48 soldiers (bots), 8 vehicles, 300 live projectiles and 200 active effects";
  r4:765-767 (R4-PHYS-REC4): "keeps 8 big vehicles (R1-ECS-REC1's number), harvesters included"; r4:839-840
  (R4-NAV-REC2): "with 8 big vehicles instead of P1's 4"; r4:1000 (R4-OBST-REC1): "48 bots, 8 big vehicles including
  harvesters".
- r1:2013-2014 (R1-TICK-REC2 P1): "plus 4 big vehicles and 200 live projectiles"; r3:1110-1111 (R3-BUDGET-REC2): "48
  soldiers on screen in one fight, 4 large units, 200 live projectiles and their effects (R1-TICK's scene)"; r3:1079
  (R3-CHAR-REC4) and r3:1187 (R3-MEASURE-REC2) the same.
- r2:1757-1758 (worked estimate): projectiles "32 at 48 players", vehicles "one per squad of 4 (LOOP_V2 §2) + one
  harvester per team (GE6)" (15 at 48, r2:1771); r2:2173 (R2-SYNC-REC5 mark 3): "48 soldiers plus 12 vehicles and
  harvesters".
- r1:2395 (R1-ABL-REC6): "48 soldiers … each with up to 8 active effects"; r1:2133-2134 (R1-DET-REC3): "48 characters
  plus the vehicles VISION_V2 §6.4 implies, count set by R4".
- Existing questions: R1-XU-OQ1 (R1's units only) and R2-BUD-OQ5 (the caps).

**The scene (GLOSSARY: the stress scene), each count with its basis.**

| Part | Count | Basis | Why this one |
|---|---|---|---|
| Soldiers | the largest mode's N (48 or 54, MODES-3), bots with full AI, in three teams, all in one combat cluster; scale steps N = 12, 24, 36, 48 and the N = 64 headroom run where a mark asks | R1-TICK-REC2 P1 (cluster, scale steps); resolution 10 | three teams give more enemy pairs at the same N (arithmetic below), and R2-INT-REC3 bounds the visibility cost by the enemy-pair count (R2-INT-F20); R2's vehicle count is the three-team count |
| Big vehicles | 15 in play, harvesters included | R2's worked estimate at 48 (r2:1758, r2:1771) | the largest count any topic uses at 48, so no mark runs on fewer vehicles than another topic assumes; it comes from a stated rule (squads of 4, LOOP_V2 §2; one harvester per team, GE6), where 4 and 8 come with none (R1-ECS-REC1: "proposed, not a design target") |
| of which very large units | 4, each at least 20 parts | R1-TICK-REC2 P1, R3-CHAR-REC4; VISION_V2 §6.9 ("a few very large units") | R3's draw-call arithmetic rests on 4 (r3:1134-1136); the part counts of the other 11 are in no file |
| Live projectiles | 300 | R1-ECS-REC1 | the largest stress count (against 200 in R1-TICK-REC2 and R3); R2's 32 is its bandwidth assumption, not a stress count |
| Active effects | up to 8 per soldier: N × 8 | R1-ABL-REC6 | the effect unit's own load, and the larger one (384 at 48 against R1-ECS-REC1's 200) |
| Where | soldiers, the 4 very large units and the live projectiles in the one fight (on screen for R3's marks); the other 11 big vehicles move elsewhere on the same map | VISION_V2 §6.9; R3-BUDGET-REC2 | keeps every render budget's arithmetic valid while the sim still carries all 15 |
| Map | one 3D greybox shared by every mark, R1-TICK-REC2's "DM_Tri-sized greybox" | R1-TICK-REC2 | one map so results compare |

*Arithmetic.* 48 players in 3 teams with squads of 4: 48 / 4 = 12 squads, plus 3 harvesters = 15 (R2's row at
r2:1771). Enemy pairs at N = 48: 768 at 16v16v16 against 576 at 24v24 (R2-INT-F20, r2:863, gives both). At
N = 54 (18v18v18): 3 × 18 × 18 = 972. Active effects: 48 × 8 = 384; 54 × 8 = 432. Under MODES-3 (b), R2's rule gives no whole number
(18 / 4 = 4.5 squads per team), so 15 big vehicles is kept (proposed) under both readings until the caps.

*Holds under every option of MODES-3:* only N and its three-team arrangement change (16v16v16 or 18v18v18); the other
counts do not.

*Alternatives weighed (for the advisor).* (i) All 15 big vehicles in the fight on screen: a harder render case, but
R3's draw-call arithmetic would need part counts that no file gives. (ii) R1-ECS-REC1's 8 vehicles: fewer than R2's
estimate assumes, so R2's history mark (R2-SYNC-REC5 mark 3) would run on a larger scene than the CPU marks.

*Consequences.* R3-BUDGET-REC2 moves from 200 to 300 projectiles (a recommendation change in R3, logged by its owner);
R2-SYNC-REC5 mark 3's history now holds 15 big vehicles plus N soldiers (its 1 MB mark is unchanged; R2's own estimate is
"about 0.1 MB at 60 Hz for 60 movers", inferred, against 63 movers at N = 48). Under MODES-3 (b), R3's Low draw-call
arithmetic exceeds its 800 cap (R9-13 below), which R3-BUDGET-OQ4 already leaves open.

*Changes per file (before → after).*

R1:
- R9-1, r1:1907-1908 (R1-ECS-REC1 mark 1):
  - before: `Stress load (proposed, not a design target; the game-designer may change it): 48 soldiers (bots),
     8 vehicles, 300 live projectiles and 200 active effects, on a 3D greybox.`
  - after: `The stress scene (GLOSSARY; proposed, not a design target; the game-designer's caps replace its counts,
     R2-BUD-OQ5): the largest mode's N soldiers (bots), 15 big vehicles, 300 live projectiles and up to 8 active
     effects per soldier, on a 3D greybox.`
- R9-2, r1:1914-1915 (R1-ECS-REC1 mark 2):
  - before: `at the
     stress load.`
  - after: `in the
     stress scene.`
- R9-3, r1:2013-2014 (R1-TICK-REC2 P1):
  - before: `N = 12 / 24 / 36 / 48 / 64 soldiers (bots with full AI, worst case: all in one combat cluster),
  plus 4 big vehicles and 200 live projectiles, on a DM_Tri-sized greybox`
  - after: `the stress scene (GLOSSARY) at N = 12 / 24 / 36 / 48 / 64 soldiers and at the largest mode's N where it is
  not among them (MODES-3) (bots with full AI, worst case: all in one combat cluster), with its 15 big vehicles, 300
  live projectiles and up to 8 active effects per soldier, on a DM_Tri-sized greybox`
- R9-4, r1:2015 (R1-TICK-REC2 P1):
  - before: `at N = 48 at most`
  - after: `at the largest mode's N at most`
- R9-5, r1:2036 (R1-TICK-REC3, spike):
  - before: `at N = 48 in P1's scene`
  - after: `at the largest mode's N in the stress scene (GLOSSARY)`
- R9-6, r1:2133-2134 (R1-DET-REC3, spike):
  - before: `a physics scene of the largest mode's size (48 characters plus the
  vehicles VISION_V2 §6.4 implies, count set by R4)`
  - after: `the stress scene (GLOSSARY: the largest mode's N soldiers and 15 big vehicles, the count R4-PHYS-REC4
  takes up)`
- R9-7, r1:2395-2396 (R1-ABL-REC6):
  - before: `for 48 soldiers (VISION_V2 §3, 24v24), each with up to 8 active effects (a
  proposed load)`
  - after: `in the stress scene (GLOSSARY: the largest mode's N soldiers, VISION_V2 §3, each with up to 8 active
  effects, a proposed load)`
- R9-8, r1:2759-2760 (R1-XU-OQ1), at the end of the entry:
  - before: `What decides it: the synthesis picks one stress scene and one split of the tick budget; until then every figure
  stays as each unit proposed it.`
  - after: `What decides it: the synthesis picks one stress scene and one split of the tick budget; until then every figure
  stays as each unit proposed it. Settled by the shared glossary (2026-10-30): one stress scene and one server tick
  budget with its shares (GLOSSARY), which the marks above now use.`

R2:
- R9-9, r2:2173 (R2-SYNC-REC5 mark 3):
  - before: `The history for 48 soldiers plus 12 vehicles and harvesters stays at or under 1 MB.`
  - after: `The history for the stress scene's soldiers and big vehicles (GLOSSARY: the largest mode's N and 15,
  harvesters included) stays at or under 1 MB.`
- R9-10, r2:2464 (R2-INT-REC4, spike):
  - before: `in the worst case (48 players plus vehicles and projectiles in the centre field),`
  - after: `in the stress scene (GLOSSARY), with its fight in the centre field,`
- R9-11, r2:2934-2935 (R2-BUD-OQ5), at the end of the entry:
  - before: `Decides: game-designer's caps; the spike then tests the cap as the worst
  case.`
  - after: `Decides: game-designer's caps; the spike then tests the cap as the worst
  case. Until then every topic's stress marks use one proposed scene (GLOSSARY: the stress scene; 15 big vehicles,
  300 live projectiles); the estimate's 32 projectiles stay its own bandwidth assumption.`

R3:
- R9-12, r3:1079-1080 (R3-CHAR-REC4, spike):
  - before: `four large units (each at least 20 parts; the count of R3-BUDGET-REC2's and R1-TICK's
scene) on screen with 48 soldiers`
  - after: `the stress scene's four very large units (GLOSSARY; each at least 20 parts) on screen with its soldiers`
- R9-13, r3:1136 (R3-BUDGET-REC2, the arithmetic):
  - before: `four large units up to 200 (R3-CHAR-REC4): about 786, inside 800.`
  - after: `four very large units up to 200 (R3-CHAR-REC4): about 786, inside 800, at N = 48 (at N = 54, MODES-3 (b),
  the soldiers alone take 54 × 4 × 2 = 432 draws and the sum is about 834, over 800: R3-BUDGET-OQ4).`
- R9-14, r3:1110-1111 (R3-BUDGET-REC2, heading):
  - before: `measured in the worst case of the largest mode: 48 soldiers on screen
in one fight, 4 large units, 200 live projectiles and their effects (R1-TICK's scene), on the reference machines.`
  - after: `measured in the stress scene's fight on screen (GLOSSARY): the largest mode's N soldiers
in one fight, its 4 very large units and its 300 live projectiles with their visual effects, on the reference machines.`
- R9-15, r3:1187 (R3-MEASURE-REC2, heading):
  - before: `largest mode's worst case (R3-BUDGET-REC2: 48 soldiers in one fight, 4 large units, 200 projectiles)`
  - after: `stress scene (GLOSSARY; R3-BUDGET-REC2's fight on screen)`
- R9-16, r3:1189-1190 (R3-MEASURE-REC2, reason):
  - before: `R1-TICK's scale
scene give the load`
  - after: `the stress scene
(GLOSSARY) give the load`

R4:
- R9-17, r4:691 (R4-PHYS-REC1 mark 2):
  - before: `At R1-ECS-REC1 mark 1's stress load,`
  - after: `In the stress scene (GLOSSARY),`
- R9-18, r4:765-767 (R4-PHYS-REC4, spike):
  - before: `the stress load for R1-DET-REC3 and
  R1-ECS-REC1 mark 1 keeps 8 big vehicles (R1-ECS-REC1's number), harvesters included (LOOP_V2 §3; how many per team
  is the game-designer's), at 30 and 60 Hz.`
  - after: `the stress scene (GLOSSARY), which R1-DET-REC3 and
  R1-ECS-REC1 mark 1 use, holds 15 big vehicles, harvesters included (LOOP_V2 §3; how many per team
  is the game-designer's, R2-BUD-OQ5), at 30 and 60 Hz.`
- R9-19, r4:839-840 (R4-NAV-REC2, spike):
  - before: `in R1-TICK-REC2's P1 scene at N = 48 and N = 64 bots, with 8 big vehicles instead of
  P1's 4 (the count R4-PHYS-REC4 proposes; the one stress scene is R1-XU-OQ1's, for the synthesis),`
  - after: `in the stress scene (GLOSSARY) at the largest mode's N and in its N = 64 headroom run,`
- R9-20, r4:1000 (R4-OBST-REC1, spike):
  - before: `R4-PHYS-REC4's stress scene (48 bots, 8 big vehicles including harvesters)`
  - after: `the stress scene (GLOSSARY: the largest mode's N bots, 15 big vehicles including harvesters)`
- R9-21, r4:1115-1116 (wave-1 list):
  - before: `its spike scene stands, with the
  vehicle count it asked R4 for: 8 big vehicles, harvesters included (R4-PHYS-REC4).`
  - after: `its spike scene is now
  the stress scene (GLOSSARY), whose 15 big vehicles include harvesters (R4-PHYS-REC4).`
- R9-22, r4:1117-1118 (wave-1 list):
  - before: `- R1-TICK-REC2's P1 scene has 4 big vehicles; R4 proposes 8 there too (R4-PHYS-REC4), so R4-NAV-REC2's mark runs P1
  with 8. The one stress scene stays R1-XU-OQ1's, for the synthesis.`
  - after: `- R1-TICK-REC2's P1 scene had 4 big vehicles and R4 proposed 8 (R4-PHYS-REC4); R1, R3 and R4 now share one
  stress scene with 15 (GLOSSARY), which settles the scene part of R1-XU-OQ1.`

R5, R6: no change (no stress counts).

**Basis ids:** R1-ECS-REC1, R1-TICK-REC2, R1-ABL-REC6, R1-DET-REC3, R1-XU-OQ1, R2 worked estimate (r2:1757-1758,
r2:1771), R2-SYNC-REC5, R2-INT-REC3, R2-INT-F20, R2-INT-REC4, R2-BUD-OQ5, R3-BUDGET-REC2, R3-CHAR-REC4,
R3-MEASURE-REC2, R4-PHYS-REC1, R4-PHYS-REC4, R4-NAV-REC2, R4-OBST-REC1; VISION_V2 §6.9. **New numbers:** 15, 300,
8 per soldier and 4 are taken from the files (ids above); 384, 432, 768, 576, 972, 432 draws and 834 are arithmetic
(shown); the scene as a whole is proposed.


## 10. Mode sizes: totals or per team

**Outcome: escalated. Decision code MODES-3; decider: Mustafa** (the advisor's table: "Mustafa now (MODES-3 below)").

**Both sides.**
- Totals: r1:2587-2588 (R1-TICK-OQ7) "the largest mode (48 soldiers, 47 of them bots)"; r1:2098-2099 (R1-DET-REC1) "a
  bots-only 24v24 match (48 players) and a 12v12v12 match (36 players)"; r5:1471 (R5-TEST-REC5) "24v24 = 48 and
  12v12v12 = 36"; r6:442 "6v6v6 (start, three teams, upper end) | 18"; r2:1768-1769 (R2's worked estimate) "Two teams,
  4v4 (VISION_V2 §3 start, upper end)" and "Three teams, 6v6v6 (start, upper end)".
- Per team: r2:2820 (R2-ROOM-REC7) "mode files for 2×2, 2×8, 3×6, 3×18, 12v12v12 and 24v24" (3×18 = 54); r2:2809
  (R2-ROOM-REC6) "3 teams × 16 slots".
- Three-team test sizes: r2:2447 (R2-INT-REC3), r4:954 (R4-LOS-REC1 mark 4) and r4:969 (R4-LOS-REC2) "24v24 and
  16v16v16".
- Existing questions: R2-BUD-OQ6 ("totals or per team? … Decides: Mustafa"), R2-XU-OQ13, R3-BUDGET-OQ4.

**Why it is escalated.** The topics' findings cannot settle how VISION_V2 §3 is read; only its owner can (R2-BUD-OQ6
already names Mustafa). It sets the largest mode's N, and with it the stress scene (resolution 9), the offline scale
test (R1-TICK-OQ7), the soak runs (R5-TEST-REC5) and the cost rows (R6).

**The question for Mustafa (MODES-3).** VISION_V2 §3 says "two teams with 2–8 players, and three teams with 6–18
players" to start. Are these totals or sizes per team?
- (a) Totals: 1v1 to 4v4, and 2v2v2 to 6v6v6. The largest mode is then the goal 24v24, N = 48; the three-team goal
  12v12v12 has 36.
- (b) Per team: 2v2 to 8v8, and 6v6v6 to 18v18v18. The largest mode is then 18v18v18, N = 54.
- **Recommendation: (a).** Under (b) the "start small" modes would be larger (54) than both goals (48 and 36), and 54
  is "above the 48 design point" (R2-BUD-OQ6). A run above 48 stays possible as a labelled headroom run (the design
  point and headroom, GLOSSARY), as R1-TICK-REC2 P1 and R4-NAV-REC2 already run N = 64. This is also the advisor's
  recommendation.

**What changes now, under every option** (basis: VISION_V2 §3; R2-BUD-OQ6; LOOP_V2 §2 as quoted at r1:311 and r2:466,
"12v12v12 to 16v16v16, possibly more"). The topics stop fixing N at 48 where they mean the largest mode, name the
test-mode list (GLOSSARY) instead of their own lists, and carry one shared question.

*Arithmetic for the lists:* under (a), "2–8 players" in two teams is 1 to 4 per team and "6–18 players" in three teams
is 2 to 6 per team; under (b), 2 to 8 and 6 to 18 per team; 3 × 18 = 54; 16v16v16 = 3 × 16 = 48.

*Shared question, to insert in R1, R2, R3, R4, R5 and R6 (the "Shared open questions" subsection), word for word:*

> **MODES-3 (Mustafa). How are VISION_V2 §3's starting sizes read?** VISION_V2 §3 says "two teams with 2–8 players,
> and three teams with 6–18 players" to start, and "The goal: 12v12v12 or 24v24". (a) Totals: 1v1 to 4v4, and 2v2v2
> to 6v6v6; the largest mode is then the goal 24v24, N = 48. (b) Per team: 2v2 to 8v8, and 6v6v6 to 18v18v18; the
> largest mode is then 18v18v18, N = 54. The answer sets the largest mode's N (GLOSSARY), which the stress scene, the
> offline scale test (R1-TICK-OQ7), the soak runs (R5-TEST-REC5) and the cost rows (R6-COST) use, and items 3 and 4 of
> the test-mode list (GLOSSARY). Until it is answered, a mark that needs the largest mode's N gives both values. Also
> asked as R2-BUD-OQ6, R2-XU-OQ13 and R3-BUDGET-OQ4. Decided by: Mustafa.

*Changes per file (before → after; text outside the quote unchanged).*

R1:
- R10-1, r1:2098-2099 (R1-DET-REC1, spike):
  - before: `(a) a bots-only 24v24 match (48 players) and a 12v12v12 match (36 players), the two
  goals of VISION_V2 §3, each run 10 minutes`
  - after: `(a) a bots-only 24v24 match (48 players) and a 12v12v12 match (36 players), the two
  goals of VISION_V2 §3, plus the largest mode where it is neither (GLOSSARY; MODES-3: 18v18v18 under reading (b)),
  each run 10 minutes`
- R10-2, r1:2588 (R1-TICK-OQ7):
  - before: `does not say whether the largest mode (48 soldiers, 47 of them bots) must run offline on the minimum laptop.`
  - after: `does not say whether the largest mode (GLOSSARY; 48 soldiers under MODES-3 (a), 54 under (b), all but the
  player bots) must run offline on the minimum laptop.`
- R10-3: insert the shared question MODES-3.

R2:
- R10-4, r2:1768 (worked estimate, results table):
  - before: `| Two teams, 4v4 (VISION_V2 §3 start, upper end) |`
  - after: `| Two teams, 4v4 (VISION_V2 §3 start, upper end, as totals: MODES-3 (a)) |`
- R10-5, r2:1769:
  - before: `| Three teams, 6v6v6 (start, upper end) |`
  - after: `| Three teams, 6v6v6 (start, upper end, as totals: MODES-3 (a)) |`
- R10-6, r2:2447 (R2-INT-REC3, spike):
  - before: `at 24v24 and 16v16v16, all players in one fight`
  - after: `at 24v24 and 16v16v16 (the test-mode list, GLOSSARY) and, under MODES-3 (b), 18v18v18, all players in one
  fight`
- R10-7, r2:2809 (R2-ROOM-REC6, spike):
  - before: `(1) 3 teams × 16 slots, 30 humans`
  - after: `(1) the largest mode's N in three teams (GLOSSARY; 3 teams × 16 slots under MODES-3 (a), 3 × 18 under (b)),
  30 humans`
- R10-8, r2:2820 (R2-ROOM-REC7, spike):
  - before: `(1) mode files for 2×2, 2×8, 3×6, 3×18, 12v12v12 and 24v24 load`
  - after: `(1) mode files for every mode of the test-mode list (GLOSSARY; under MODES-3 (a): 1v1, 4v4, 2v2v2, 6v6v6,
  12v12v12, 16v16v16 and 24v24; under (b): 2v2, 8v8, 6v6v6, 18v18v18, 12v12v12, 16v16v16 and 24v24; plus the
  N = 64 headroom mode) load`
- R10-9, r2:2938 (R2-BUD-OQ6), at the end of the entry:
  - before: `point but inside the estimate's 64-player row. Decides: Mustafa.`
  - after: `point but inside the estimate's 64-player row. Decides: Mustafa. Shared as MODES-3 (below).`
- R10-10, r2:3173 (R2-XU-OQ13), at the end of the entry:
  - before: `(R2-ROOM-REC6). Decided by: Mustafa (R2-BUD-OQ6).`
  - after: `(R2-ROOM-REC6). Decided by: Mustafa (R2-BUD-OQ6). Shared as MODES-3 (below).`
- R10-11: insert the shared question MODES-3.

R3:
- R10-12, r3:1360-1361 (R3-BUDGET-OQ4), at the end of the entry:
  - before: `OQ7 decides the N of the offline scale test. The draw-call and triangle rows scale with N.`
  - after: `OQ7 decides the N of the offline scale test. The draw-call and triangle rows scale with N. The largest mode's
  N (GLOSSARY) is 48 or 54 by the answer to MODES-3 (below).`
- R10-13: insert the shared question MODES-3.

R4:
- R10-14, r4:954 (R4-LOS-REC1 mark 4):
  - before: `R2-INT-REC3's full pass (24v24 and 16v16v16, three points)`
  - after: `R2-INT-REC3's full pass (24v24 and 16v16v16 from the test-mode list, GLOSSARY, and 18v18v18 under MODES-3
  (b); three points)`
- R10-15, r4:969 (R4-LOS-REC2, spike):
  - before: `at 24v24 and 16v16v16 with everyone in one fight`
  - after: `at 24v24 and 16v16v16 (the test-mode list, GLOSSARY) and, under MODES-3 (b), 18v18v18, with everyone in one
  fight`
- R10-16: insert the shared question MODES-3.

R5:
- R10-17, r5:1471 (R5-TEST-REC5):
  - before: `(24v24 = 48 and 12v12v12 = 36, plus LOOP_V2 §11's N = 12, 24, 36, 48 scale steps)`
  - after: `(24v24 = 48 and 12v12v12 = 36, plus the largest mode where it is neither (GLOSSARY; MODES-3: 18v18v18 = 54
  under reading (b)), plus LOOP_V2 §11's N = 12, 24, 36, 48 scale steps)`
- R10-18: insert the shared question MODES-3.

R6:
- R10-19, r6:441:
  - before: `| 4v4 (start, two teams, upper end) |`
  - after: `| 4v4 (start, two teams, upper end, as totals: MODES-3 (a)) |`
- R10-20, r6:442:
  - before: `| 6v6v6 (start, three teams, upper end) |`
  - after: `| 6v6v6 (start, three teams, upper end, as totals: MODES-3 (a)) |`
- R10-21, r6:447, after the table's closing paragraph:
  - before: `client (R2). Ingress is free on Hetzner (R6-HOST-F17).`
  - after: `client (R2). Ingress is free on Hetzner (R6-HOST-F17). Under MODES-3 (b) the start modes' upper ends are 8v8
  (16 players) and 18v18v18 (54); their rows follow from R2's formula (r2:1760-1762) once Mustafa answers.`
- R10-22, r6:464 (Start scenario):
  - before: `**Start (VISION_V2 §3: two teams of 2–8, three teams of 6–18).**`
  - after: `**Start (VISION_V2 §3: two teams of 2–8, three teams of 6–18, read as totals: MODES-3 (a); under (b) a start
  room holds up to 54).**`
- R10-23: insert the shared question MODES-3.

**Basis ids:** R2-BUD-OQ6, R2-XU-OQ13, R3-BUDGET-OQ4, R1-TICK-OQ7, R2-ROOM-REC6, R2-ROOM-REC7, R2-INT-REC3,
R4-LOS-REC1, R4-LOS-REC2, R1-DET-REC1, R5-TEST-REC5, R6 egress table; VISION_V2 §3; LOOP_V2 §2 as quoted at r1:311 and
r2:466. **New numbers:** 54 (3 × 18, arithmetic), per-team ranges under (a) (arithmetic above); no other.
**Changes that depend on this item elsewhere:** resolution 9 (the stress scene's N) and resolution 1 (P1's N).


## 11. One core or one vCPU as the measuring unit

**Outcome: resolved.** The reference server is the CCX23 that R6 names, and every server time mark is measured on one
vCPU of it (GLOSSARY: the reference server; the measuring unit). Decider: the advisor (the advisor's table: "Me").

**Both sides.**
- One core: r1:1905-1906 (R1-ECS-REC1) "reference hardware proposed: one server core of the R6 host type"; r1:2015
  (R1-TICK-REC2) "on one core of the proposed reference server"; r1:2396 (R1-ABL-REC6) "on the proposed reference
  server core"; r4:658 "one core of the R6 server type"; r1:2545 (R1-ECS-OQ4) "one match per core".
- One vCPU: r2:2385 (R2-BUD-REC6) "one vCPU of the EU instance type R6 recommends"; r2:2732 (R2-ROOM-REC2) "one 4-vCPU
  EU cloud VM"; r6:628-629 (R6-HOST-REC2) "the dedicated CCX23 (4 vCPU, 16 GB, x86-64 AMD)"; r6:942-943 "'one vCPU of
  the EU instance type' in R2-BUD-REC6 is one vCPU of CCX23".
- Existing questions: R2-XU-OQ9 (answered by R6 for R2 only) and R1-TICK-OQ6.

**Which holds and why: one vCPU of the CCX23.**
1. R6, the topic that owns hosting, names the machine (R6-HOST-REC2) and already states R2's unit in it (r6:942-943).
2. The CCX23 is sold, billed and sized in vCPUs (R6-HOST-F15), and R6's density and costs are per vCPU (R6-COST A-2;
   the per-room-hour rows at r6:452-454, one of which already reads R1-ECS-OQ4's "one per core" as "one per vCPU",
   r6:453). A mark in vCPUs turns straight into rooms per VM and cost per room-hour.
3. "One core" is defined for the reference server nowhere in the six files, and the two units can differ: AWS lists a
   "c6a.large (1 core, 2 vCPU, 4 GB)" (R6-HOST-F23). Measuring R1 and R4 on a "core" and R2 and R6 on a "vCPU" could
   differ by that factor.
4. It holds whatever MODES-3 decides: the unit does not depend on N.

R6-HOST-REC2 stays conditional on R6-HOST-OQ1 (Hetzner's EU capacity), and Mustafa's approval of the reference hardware
(R1-TICK-OQ6) stays as it is. Nothing here changes a number: only the unit's name.

*Changes per file (before → after).*

R1:
- R11-1, r1:1905-1906 (R1-ECS-REC1):
  - before: `reference hardware proposed: one server core of the R6 host type, and the
  VISION_V2 §4 minimum laptop for Wasm`
  - after: `reference hardware proposed: one vCPU of the reference server (GLOSSARY: the CCX23 of R6-HOST-REC2), and the
  VISION_V2 §4 minimum laptop for Wasm`
- R11-2, r1:2012 (R1-TICK-REC2):
  - before: `reference hardware proposed, to be named with R3, R5 and R6`
  - after: `reference hardware proposed: the reference server (GLOSSARY: R6-HOST-REC2's CCX23) and R3-BUDGET-REC1's
  minimum laptop`
- R11-3, r1:2015 (R1-TICK-REC2 P1):
  - before: `on one core of the proposed reference server`
  - after: `on one vCPU of the reference server`
- R11-4, r1:2396 (R1-ABL-REC6):
  - before: `on the proposed reference server core`
  - after: `on one vCPU of the reference server (GLOSSARY)`
- R11-5, r1:2545 (R1-ECS-OQ4, heading):
  - before: `or is one match per core enough?`
  - after: `or is one match per vCPU of the reference server enough?`
- R11-6, r1:2584-2585 (R1-TICK-OQ6):
  - before: `P2 and P3 need a named minimum laptop CPU and P1 a named server core. Decided by: R3, R5 and R6
proposals, approved by Mustafa.`
  - after: `P2 and P3 need a named minimum laptop CPU and P1 a named server. R6 proposes the server, one vCPU of a CCX23
(R6-HOST-REC2; GLOSSARY: the reference server), and R3 the laptop class (R3-BUDGET-REC1). Decided by: those proposals,
approved by Mustafa.`
- R11-7, r1:2955-2956 (pointers to R6):
  - before: `cost per match if one match runs per core (R1-ECS-OQ4)`
  - after: `cost per match if one match runs per vCPU (R1-ECS-OQ4)`
  - before: `and matches per core at 30 Hz against 60 Hz`
  - after: `and matches per vCPU at 30 Hz against 60 Hz`

R2:
- R11-8, r2:2385 (R2-BUD-REC6):
  - before: `on the reference server (proposed: one vCPU of the EU instance type R6 recommends)`
  - after: `on one vCPU of the reference server (GLOSSARY: the CCX23 of R6-HOST-REC2)`
- R11-9, r2:2558-2559 (R2-TRN-REC1 mark 2):
  - before: `most 1 CPU core for the transport layer, and p99 server send-call time ≤ 1 ms, on reference server hardware R6 names
   (proposed).`
  - after: `most 1 vCPU for the transport layer, and p99 server send-call time ≤ 1 ms, on the reference server (GLOSSARY:
   R6-HOST-REC2's CCX23) (proposed).`
- R11-10, r2:2732 (R2-ROOM-REC2):
  - before: `reference host proposed: one 4-vCPU EU cloud VM, R6 to name it`
  - after: `reference host: the reference server (GLOSSARY: the 4-vCPU CCX23 of R6-HOST-REC2)`
- R11-11, r2:3145-3146 (R2-XU-OQ9), at the end of the entry:
  - before: `Decided by: R6's instance choice; the synthesis
  names one reference machine.`
  - after: `Decided by: R6's instance choice; the synthesis
  names one reference machine. Answered by R6-HOST-REC2: the reference server is a CCX23, and every server mark is
  measured on one of its vCPUs (GLOSSARY).`
- R11-12, the words "per core" in the budgets options table E (r2:1721, "several rooms per core") and the pointers
  between units (r2:3244, "rooms per core"): "core" → "vCPU" in each, nothing else. (R2-BUD-REC6's reason, r2:2380,
  "more than one room per core", is rewritten whole by resolution 1, change R1-R2-3; if resolution 1 is not approved,
  "core" → "vCPU" there too.)

R4:
- R11-13, r4:658:
  - before: `(one core of the R6 server type; the
VISION_V2 §4 minimum laptop for Wasm)`
  - after: `(one vCPU of the reference server, GLOSSARY: R6-HOST-REC2's CCX23; the
VISION_V2 §4 minimum laptop for Wasm)`

R6: no change; R6 already uses the term. R6-HOST-REC2 gains the glossary link at its first use.

**Basis ids:** R6-HOST-REC2, R6-HOST-F15, R6-HOST-F23, R6-COST A-2 and the cost rows (r6:452-454), the pointer
r6:942-943, R2-XU-OQ9, R1-TICK-OQ6, R6-HOST-OQ1. **New numbers:** none.


## 17. Who owns the dependency audit in CI

## 19. Previews on Cloudflare Workers or Pages

## 21. A login wall on previews

## 22. Isolation headers on every preview, or behind a flag

## 23. One wildcard certificate on every game server

## 29. HashMap in the sim: lookups allowed, or the type banned
