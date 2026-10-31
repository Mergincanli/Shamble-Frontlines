# Step 3d, R2: the final check's report (verbatim)

Workflow run wf_cbb704b7-47f (prompt `prompts/verifier-r2-3d-final.txt`, xhigh), launched 2026-10-31 10:22 UTC, returned about 10:31 UTC. Notes: `verifier-r2-3d-final.md`.

Check 1, the two fix messages: closed. Every item was applied as asked.
- The four shares now name the tick period (r2:2402, 2403, 2405, 2414).
- The r2:2339 mark is labelled "the design point, N = 48", which is right: its 160 kbps mean rests on the 48-player estimate.
- All six "design for 48+" credits and part 7 now name CLAUDE.md (r2:2289, 2356, 2410, 2628, 2738, 3898).
- The part 7 totals add up: 37 recommendation changes and 110 nits.
- One nit was introduced by the second fix: R2-XU-OQ18 (r2:3259-3261) still quotes, in the present tense, the old Serves wording that r2:2738 replaced, and r2:2356's was replaced too.

Check 2, the 17 recommendation changes: closed apart from one nit. Every new number is 48 or 54 (MODES-3) or 25 % / 50 % of the tick period (resolution 1). Each value holds under both readings.
- Nit: r2:2187 keeps "700 shots per second", the full-auto rate at 48 players (r2:2280, r2:3175). Under reading (b) it pairs 54 candidates with that rate. GLOSSARY §1 scales it ×1.125 at 54, but the mark does not say so.
- R2-ROOM-REC2 mark 5 holds. The spike starts about 9 rooms, and "every" makes it a plain limit, which is the glossary's rule for one-off events (GLOSSARY:143-145).
- Note, not a fix item: under reading (b), no R2 bandwidth mark covers 54 players. R2's own formula gives about 164–171 kbps there, above the 160 kbps mean.

Check 3, no new claim, number or source: closed.

Check 4, glossary hits: 32 hits judged, all 32 right, 0 wrong. "Retire (exact)" is still 0; I re-ran the search on 2382ea2 and got a hit list identical to the session's.

Check 5, the label: closed. No "Rests on" line changed, and no finding or source was added.

Claims checked at a source:
- "Design for 48+" belongs to CLAUDE.md · CLAUDE.md:8 · confirmed · "Design for 48+ players; bots fill empty slots."
- VISION_V2 §3 holds the modes · VISION_V2.md:43-44 · confirmed · "three teams with 6–18 players"; "12v12v12 or 24v24" (no "48" anywhere in the file).
- The 160 kbps mark rests on the 48-player estimate · r2:1775 · confirmed · "design point 48 … **146.1 kbps**".
- The R2-XU-OQ9 quotes · r2-netcode.md at ae492a6, r2:2385, 2558, 2732 · confirmed.

**Label:** R2 keeps "verified (every recommendation claim)".

**Only nits remain:** yes. The two above, plus stale "Used today" lists in GLOSSARY.md (lines 131 and 277; the glossary's owner).

**Host audit:** nothing fetched (WebFetch 0, curl 0).

Notes, ending with DONE: /home/user/Shamble-Frontlines/.checkpoints/v2-research/verifier-r2-3d-final.md
