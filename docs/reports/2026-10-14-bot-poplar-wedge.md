# Advisor report: bot-poplar-wedge (2026-10-14, branch cs/bot-poplar-wedge, base main ef019e0)

## Goal
Bots no longer freeze for 30 to 90 s against a poplar, apple tree or other narrow collider on their way to the refinery or a
Tech Box.

## Outcome
DONE as planned, plus both of the Advisor's additions and one perf fix from the review. Waiting on the Advisor's APPROVE.

## What changed (shamble-game.js only; symbols +0 −0, matches the plan)
- `navBuild`: each 2 m cell centre is tested at the bot radius plus half a cell (0.7 + 1.0) instead of 0.75. A cell is
  blocked as soon as any part of its square is solid for a bot.
- `navPath`: a start in a blocked or boxed-in cell (no open side) begins A* from the nearest free cell, and that cell is
  the first waypoint.
- End of `bootGame()`: three warm-up `navPath` calls (perf review PERF-1, see below).

## Evidence (base ef019e0 → head 4d8acfd; harnesses run through `tools/preview.js eval` on the real code)
| Measure | Base | Head |
|---|---|---|
| Starts pinned > 5 s at the poplar (1,787 starts, 40 s walk to the refinery) | 39 (worst 39.2 s) | 0 |
| Sweep, 4 goals × 2,003 spawn starts: starts wedged ≥ 3 s / ≥ 30 s | 112 / 43 | 0 / 0 |
| Sweep arrivals within 75 s (refinery, techN, techW, techE) | 1949, 2003, 1987, 2000 | 1957, 2003, 2003, 2003 |
| Refinery arrivals within 150 s | 1994 of 2003 | 2003 of 2003 |
| Seeded 90 s matches, 240 pairs: runs with tree-face stalls / stall windows | 39 / 1,885 | 0 / 0 |
| Same: runs with poplar stalls / runs ≥ 18% stall | 4 / 11 | 0 / 4 (all 4 equally bad on base, not on trees) |
| Same: stall % mean / median (paired head − base: median 0, max +1.7) | 9.12 / 8.4 | 8.50 / 7.7 |
| tools/sim.js unseeded, 40 runs each: stall % mean / median | 8.82 / 7.5 | 7.73 / 6.5 |
- **Advisor addition 1 (exactness):** this is the exact square-to-box distance check, on both metrics (Euclidean and the square one the collision uses).
  - Free cells within 0.7 m of an axis-aligned collider: Cabana 763 → 0, Romania 433 → 0. The worst case is exactly 0.7 m.
  - Rotated colliders: the maps only have 90° houses, and their overlap went from 32 cells (0.5 m) to 0.
  - Synthetic 3 × 3 m crate at 64 sub-cell offsets: worst leftover 0.22 / 0.34 / 0.26 / 0.34 m at 15 / 30 / 45 / 60°, against 1.17 / 1.30 / 1.32 / 1.30 m on base.
- **Advisor addition 2 (reachability):** from every spawn sample to every bot goal (buildings, Tech Boxes, silos, resupply, player spawn), the way bots do it.
  - Romania: 5,150 of 5,150 pairs. Cabana: 7,589 of 7,590 (base: 7,587).
  - The one failure, like base's three, is a far-corner route to a *hostile* building, which bots don't target. It hits A*'s existing 8,000-node cap; with the cap lifted the route is 8 waypoints.
  - Each map stays one connected component (Cabana 22,400 → 21,861 free cells).
  - Saved maps live in the browser, so only the two built-in maps were loadable here.
- Gate on 4d8acfd: PASS, all 8 checks. CI (gate + smoke): PASS on 688d3d9 and 4d8acfd. village.py gives byte-identical output on base and head: it doesn't read the nav grid.

## Reviews (workflow: perf-auditor and an adversarial regression hunter, each verified)
- **perf-auditor: PASS WITH NOTES.**
  - navBuild time is neutral: median 42.8 vs 43.5 ms on Cabana, 15.5 vs 15.0 ms on Romania. My earlier "46 → 43" was noise.
  - On a fixed query set, A* does 2–4% fewer expansions. No allocation is added. Nav costs about 0.02 ms per frame.
  - **Fixed: PERF-1.** The new branch had no type feedback at boot, so navPath kept deoptimising. The warm-up cut deopts per match from 134 to 1 (Cabana seed 4) and from 6 to 2 (Romania seed 6); others stay at 1–3. A heavier warm-up reaching 0 would cost about 15 ms at boot for about 0.01 ms per frame; not taken.
  - **Deferred: PERF-2.** An old "replan storm" in navSteer that head hits more often on Romania (budget-limited frames 0.9% → 1.8%).
- **Regression hunter: PASS WITH NOTES.**
  - 150 paired walks per scenario (player chase, wall-hugging chase and starts, Tech Boxes, building sieges, random goals) on both maps: stalls ≥ 3 s went from 1–16 per set to 0. Direction reversals fell from 226–1,048 to 0–22. Route times are unchanged (median ratio 1.00).
  - **Deferred: R1.** On editor-made layouts, wall gaps of about 3.0–5.4 m can close for bots, depending on grid alignment. It can't happen on either built-in map.

## Decisions and deviations
- The start snap also makes the snapped cell the first waypoint (the plan said only "start A* there").
- No `NAV.botR` property: a commented local `r = 0.7 + cs/2` instead.
- The warm-up was added from the review, not from the plan.
- I ran 240 seeded pairs and 43 unseeded sims per side (the plan said ≥ 60 and ≥ 3), because the event is rare. Three unseeded runs alone pointed the wrong way.
- Found on the way: the game reads `performance.now()` / `Date.now()` in 11 places. Matches with the same seed and build differ between runs, which matters for the deterministic shared sim (ADR-001).

## Risks and open questions
1. Editor maps: rotated colliders at odd angles leave up to 0.34 m, and gaps of 3–5.4 m may close (R1). Recommendation: an editor-side reachability check (fits map-dedupe-on-load), plus an exact test for rotated colliders if odd angles get used.
2. PERF-2 replan storm: recommendation, its own small change-set with sims.
3. Not verified: Brave, a real GPU, and maps saved in Mustafa's browser.

## For Mustafa in Brave
Nothing visual changed. Optional gameplay check: deploy on Cabana and watch the poplar row (x −80 to −90, z −81 to −84)
and the apple orchard for the first 15 s. Bots should walk round the trees. Also check that bots still pass the village
gaps and the manor corners.

## Proposed next step
editor-undo-ground: Ctrl+Z keeps the toon look and no longer duplicates trees and props.
