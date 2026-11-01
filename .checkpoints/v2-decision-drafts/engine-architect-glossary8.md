# GLOSSARY §8, as the session appends it (engine-architect, synthesis, 2026-11-01)

Source: the decision map's part J1 (`engine-architect-decision-map.md`, lines 1066–1118, the text inside its
markdown fence), checked against the drafts brief §2 (`inputs/DRAFTS_BRIEF.md`, lines 52–59 and 60–66).

## Corrections and checks (read before appending)
1. **Placement (correction).** Condition 3 says "after line 742". `docs/research/GLOSSARY.md` at 9e7db28 has 742
   newline characters and one more, unterminated line: line 743, `  R5-RUSTLINT-OQ5 (settled 2026-10-30). R2, R3, R4,
   R6: none.` (`wc -l` reports 742; 64,088 bytes). Appending after line 742 would split the hash-map ban entry's "Used
   today" field. Append after the file's last line instead: add the missing newline, one blank line, then the text
   below the rule line of this file, ending with a newline. Lines 1–743 keep their bytes and numbers, so every GLOSSARY
   line cited at 9e7db28 still holds, and the base file stays a byte prefix of the new one (the write guard's check;
   the checkpoint's open finding on condition 1's `cmp`).
2. **The two brief quotes: no correction.** Each `>` line, with its `> ` prefix removed, was compared by script with the
   brief's lines 52–59 (the foundation) and 60–66 (the netcode proof test): identical, word for word, including the
   blank quoted line before "It ends when…".
3. **Line 85: no correction.** The closing note's "(line 85)" holds: GLOSSARY:85 reads "(60 Hz if P1–P3 pass at 60 Hz,
   else 30 Hz), which does not change."
4. **Anchors:** `#8-phase-1-terms-the-advisor-2026-10-31`, `#the-foundation`, `#the-netcode-proof-test`,
   `#the-60-hz-margin`. The drafts, ENGINE_OVERVIEW and SPIKE_PLAN link these. The entry's own links ("time
   statistics", "the design point and headroom") name existing entries (GLOSSARY:141, :355).
5. **Not corrected, for the consistency pass:** GLOSSARY's "How to read an entry" says a "Depends on" field "states
   every reading with its decision code and picks none" (GLOSSARY:30–31). The 60 Hz margin's "Depends on" states one
   reading, the advisor's, which condition 3 fixes as the definition; ADR 0001's OQ-0001-1 lists the other, option (b),
   "another reading of TICK-30's '25 % headroom'". Condition 3's wording is the Advisor's, so it stands as written.

---
## 8. Phase 1 terms (the Advisor, 2026-10-31)

### The foundation
- **Definition:** the Advisor's drafts brief, §2, 2026-10-31:

> - **The foundation.** Phase 1's work before slice 1's gameplay:
>   - the repository restructure, moving the old game to `legacy/`;
>   - the toolchain: a Cargo workspace plus TypeScript with Vite;
>   - CI gate v2, and private preview links;
>   - TypeScript types generated from the Rust code;
>   - the client + local server skeleton: the simulation in a Web Worker, behind the same message protocol the network uses.
>
>   It ends when an empty scene runs through the full stack in the browser, with CI green and a preview link.

- **Marked by:** "the foundation".
- **Retire (exact):** none.
- **Retire (in context):** none.
- **Used today:** none in R1–R6 (a phase 1 term); the decision drafts, `docs/ENGINE_OVERVIEW.md` and
  `docs/SPIKE_PLAN.md` use it.

### The netcode proof test
- **Definition:** the Advisor's drafts brief, §2, 2026-10-31:

> - **The netcode proof test.** A throwaway spike that proves the online path:
>   - the Rust simulation runs natively as a server, and as Wasm in the client;
>   - they connect over WebTransport, with the WebSocket fallback;
>   - 48 simulated clients run at a 30 Hz tick, with tick time, bandwidth per client and memory all within the budgets that
>     ADR 0001 sets;
>   - one human client at 100 ms of simulated latency sees prediction, reconciliation and lag-compensated hits that look
>     correct.

- **Marked by:** "the netcode proof test".
- **Retire (exact):** none.
- **Retire (in context):** none.
- **Used today:** none in R1–R6 (a phase 1 term); the decision drafts, `docs/ENGINE_OVERVIEW.md` and
  `docs/SPIKE_PLAN.md` use it.

### The 60 Hz margin
- **Definition:** 1 − (p99 server tick time at a 60 Hz tick ÷ the server tick budget at 60 Hz, 8.33 ms). It is measured
  in the netcode proof test, in one run (p99s do not add: see time statistics): on one vCPU of the reference server, in
  the stress scene at N = 48 with a snapshot-receiving client on every soldier, over R2-BUD-REC6's 30-minute soak,
  re-flood ticks included. A 60 Hz tick needs a 60 Hz margin of at least 25 % (a p99 server tick time of at most
  6.25 ms) and every 60 Hz mark the research sets, as written (R2-BUD-REC6's two shares, R1-TICK-REC2 P1–P3);
  otherwise the tick stays at 30 Hz.
- **Depends on:** TICK-60-HEADROOM (the advisor's reading of 2026-10-31: TICK-30's 25 % is measured against the
  server tick budget at 60 Hz, 8.33 ms), confirmed by Mustafa and the Advisor at the stop.
- **Marked by:** "the 60 Hz margin".
- **Retire (exact):** none.
- **Retire (in context):** "headroom" where TICK-30's 25 % is meant. TICK-30's own word "headroom" appears only in
  quotes of TICK-30; headroom keeps its meaning of a run above the largest mode's N (the design point and headroom).
- **Used today:** none in R1–R6 (a phase 1 term); ADR 0001 owns it, and ADR 0004 and `docs/SPIKE_PLAN.md` cite it.

TICK-30 (the Advisor, 2026-10-31) replaces, for the drafts, the tick-rate rule the Tick period entry calls unchanged (line 85): 30 Hz is the baseline, 60 Hz needs a 60 Hz margin of at least 25 %.
