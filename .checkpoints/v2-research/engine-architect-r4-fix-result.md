# R4 wave 2 fix step: the owner's report, verbatim

Workflow run wf_d3f13cf7-77e (prompt `prompts/engine-architect-r4-fix.txt`), the fix cut off by the spend limit on 2026-10-28, resumed and returned 2026-10-29 (about 21:21 UTC). Notes: `engine-architect-r4-fix.md`.

I applied the R4 fix pass to /home/user/Shamble-Frontlines/docs/research/r4-physics-navigation.md. No recommendation now rests on a wrong or unsupported claim.

**Changes by type:** recommendation 10 · number 3 · source 1 · nit 18. Line rewraps are formatting and are not counted.

The three numbers:
- **Tunnelling test speed:** the R4-PHYS-REC1 mark 1 speed goes from 9.0 to 13.3 m/s, the fastest soldier speed in the legacy movement data. I checked it in the legacy code (`shamble-core.js:128-137`, `shamble-game.js:2756` and `:2774`).
- **Vehicle count:** the P1 scene in R4-NAV-REC2 now uses 8 big vehicles instead of 4.
- **Quake III travel types:** R4-FLOORS-F5 now reads 18 ways to move plus `TRAVEL_INVALID`.

The source change: source 47, a parry3d page that only the verifier read, now backs R4-PHYS-F20's corrected quotes. The finding keeps its id, so it is not a re-source. I fetched no new page.

**Recommendations that became open questions:** none.

**Re-sourced claims:** none.

**Verifier findings not applied:** none. Item 5 (the missing Detour target option) went into option A's table cells rather than a new option column. No source read weighs the C-capable Wasm targets for the sim, and the fix rule allows no new claims.

**Header status:** "spot-checked (49 of 51 key claims)". The totals line now reads 47 sources read (37 official).

The session committed my in-progress edits as 11b2089 while I worked. I did not commit or push.

Files:
- /home/user/Shamble-Frontlines/docs/research/r4-physics-navigation.md
- /home/user/Shamble-Frontlines/.checkpoints/v2-research/engine-architect-r4-fix.md (fix log, ends with DONE)
- /home/user/Shamble-Frontlines/.checkpoints/v2-research/verifier-r4-result.md
