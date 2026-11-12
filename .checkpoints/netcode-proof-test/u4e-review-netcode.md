# U4e round 1: netcode-architect review notes

Reviewer: netcode-architect (read only; log prefix nc). Started 2026-11-12.
Read copy: /home/user/Shamble-Frontlines/.gate/u4e-r1 (97bb9d3 + U4e snapshot patch). Builder's copy (uncommitted):
/home/user/Shamble-Frontlines/.claude/worktrees/wf_0fa6595a-3e2-1. Base: /home/user/Shamble-Frontlines/.gate/u4e-prefix (1e8c188).

## Plan
1. Read the packet's "judged against" half (§5 U4e, §4.1, §4.2, §4.5-4.7, §4.9-4.11, §4.14, W1-*, advisor-u4e-brief,
   B2.1, B5, Mustafa's words, carries 21-27, 39, 40, 50, 51, 53-57, 61, 62, 66, 68, the left-out ids).
2. Read the source in the read copy (not only the packet diff): room (tick, InputBuffer, EventQueue, AckRing, Encoder,
   InterestGrid, AreaFlood, Priority, rewind_at, Roster), server lib (Decoder::pump, ConnLimits, Counters), client
   (ClientEndpoint), sim Scene::effect_rows.
3. Focus checks, one block each, a line per finding:
   a. room tick: InputBuffer option B + catch-up bound; carry 21 order; rewind_at integer half (carry 22, U4E-Q5);
      events stamped once into one list (23, 26); EventQueue cap as data, overflow resync + kick, age bound (U4E-Q6;
      50, 57); one Resync in flight.
   b. baselines/window (§4.5): AckRing ages, newest fully acked tick, partly acked never a baseline, window remainder
      after pump bytes at 30/60 Hz, groups and the split, encoder refusal skips that client's snapshot (54).
   c. own-state section (W1-S1; 51, 53, 55, 56, 68): own rows only via Scene::effect_rows; 56's measurement; 68's
      condition on ES byte figures.
   d. client (§4.6; 61; B2.1): complete-tick rule, sentinel causes, when lost, stale group decode/drop (B5 reading),
      acks + group mask, -58 rebuilds across wraps (W1-N2), the 1,150 B refusal.
   e. pump (§4.5 RTT; §4.9): pings, Pongs hold capped at one tick, RTT reported, bytes counted.
   f. -18 end to end (cond. 6): baseline held or not under the bite; gap alignments; 500 ms window (U4E-Q14).
   g. e2 forced split, e7 mirror over Loopback at P0 for 1,000 ticks: test what plan/brief ask, every clause.
   h. later judged runs (U4r, U4f) INVALID risks / marks passing too easily; provisional values named.
   i. scope: files/symbols outside the plan and rulings.
4. Verdict + ranked findings (severity, file:line, scenario, cheapest proof, before-merge).

## Blocks
