# PM-0004-20 and PM-0004-35: the old and the new wording, side by side (4c-2's draft, his DEPART-4C (a))

His words (2026-11-28, item 12 a, saved in `.checkpoints/cs4-walk/mustafa-2026-11-28-answers.md`): the report shows
the old and the new wording of the two marks side by side, and the new wording asks no less than the old about what
the players' file does. ADR 0004's rows are not edited in 4c-2 (a decision doc; the advisor agent's F2): the new
wording goes into 4c-2's report and a dated pointer in STATUS, as DEPART-4 did in 4b.

## Old, copied by script (`pm-old-rows.mjs`) from `docs/adr/0004-simulation-core.md`, lines 1141 and 1156

| mark | what it proves (decision part) | number and statistic | hardware | method and tool | runs and noise | basis |
|---|---|---|---|---|---|---|
| PM-0004-20 | parry3d's determinism flag holds across targets, and its cost is known (S7; parts 7, 10; decides R1-DET-OQ1) | with `enhanced-determinism` on: 0 mismatches between native and Wasm over SS for 18,000 ticks; reported: parry3d's query time per tick with the flag on and off, native and in a browser, mean and p99 | CI (hashes); RS (native); ML: Chrome (Wasm) | SS at N = 48; the hashes as PM-0004-19; the times from the phase entry | 1 run for the hashes; T, V for the times | R1-DET-REC3 (lines 2178–2182, applied to parry3d by the draft's part 10); R1-DET-OQ1 (line 2680) |
| PM-0004-35 | parry3d supports every shape pair the sim needs (S9; part 10; checks OQ-0004-3's answer) | 0 `Err(Unsupported)` results over PM-0004-30's and -31's runs; the pairs listed (the soldier's capsule or cylinder and the grenade's ball against oriented boxes, ramp wedges, other convex shapes and the ground shape; vehicle hull compounds against the same and against capsules) | CI | a counter on every query's result | 1 run. Bites: an unsupported pair placed in the scene is counted | R4-PHYS-REC1 mark 6 (r4 lines 712–717); R4-PHYS-OQ4 (r4 line 1090) |

## New (DRAFT, the main session's, 2026-11-28; for rust-reviewer's and netcode-architect's review at step 1, then the advisor agent; his reading of the result at 4c-2's report)
| mark | what it proves (decision part) | number and statistic | hardware | method and tool | runs and noise | basis |
|---|---|---|---|---|---|---|
| PM-0004-20 | The players' file's movement queries (our own sweeps, and parry3d's tree with its determinism flag) hold across targets, and their cost is known (S7; parts 7, 10; decides R1-DET-OQ1) | with `enhanced-determinism` on: 0 mismatches between native and Wasm over SS for 18,000 ticks, on the players' build (the file players get); reported: the query time per tick of everything the players' file queries (our sweeps and parry3d's tree, each and together), with the flag on and off, native and in a browser, mean and p99 | CI (hashes); RS (native); ML: Chrome (Wasm) | SS at N = 48; the hashes as PM-0004-19; the times from the phase entry | 1 run for the hashes; T, V for the times | as the old row; his DEPART-4C (a), 2026-11-28, item 12 |
| PM-0004-35 | Every shape pair the sim queries is handled, by our own sweeps or by parry3d (S9; part 10; checks OQ-0004-3's answer) | 0 unhandled results over PM-0004-30's and -31's runs: for our sweeps, every form the map format allows (box, ramp) against the soldier's body, each result counted; for parry3d, 0 `Err(Unsupported)` results for every query the players' file still makes; and parry3d checks our sweeps for every form pair, the ramp included, before the movement step counts as done (hit times within 1e-5, and misses alike); the pairs listed (the soldier's body and the grenade's ball against oriented boxes, ramp wedges and the ground; vehicle hull compounds against the same and against the soldier's body, when vehicles exist) | CI | a counter on every query's result, ours and parry3d's; the check of our sweeps against parry3d's casts (step 1's test 21) | 1 run. Bites: a form placed in the scene that no sweep handles is counted; an unsupported parry3d pair placed in the scene is counted; a ramp's side planted at its high end's height is caught by the check | as the old row; his DEPART-4C (a), 2026-11-28, item 12 |

## Does the new ask no less of the players' file? (his test; the advisor agent's F2 three keeps)
| The old asks | The new asks |
|---|---|
| native against Wasm, flag on: 0 mismatches over SS, 18,000 ticks | the same, named on the players' build (the advisor agent's first keep) |
| parry3d's query time per tick, flag on and off, native and in a browser, mean and p99 | the query time per tick of all the file's queries, our sweeps added to parry3d's tree, each and together, flag on and off, native and in a browser, mean and p99 (the second keep; more than the old: our sweeps are timed too) |
| 0 `Err(Unsupported)` over PM-0004-30's and -31's runs; a counter on every query's result; bites: an unsupported pair placed in the scene | 0 unhandled results over the same runs, for our sweeps and for parry3d's queries; a counter on every result, ours and the library's; bites: both kinds (the third keep); and, new, parry3d's check of our sweeps for every form pair, the ramp included (his DEPART-4C (a) (1)) |

## What 4c-2 can run, and what stays open in STATUS by name
- **In 4c-2 (to be confirmed against the plan at step 1's review):** PM-0004-20's hash half on `walk` (one soldier) on
  the players' build, native against Wasm in Node and Chromium, the recordings' way (PM-0004-19's); PM-0004-35's
  counters over PM-0004-30's runs, and test 21.
- **Open in STATUS by name** (his item 12 c, and the advisor agent's F2): PM-0004-20's 48-soldier run over SS (not
  met by the one-soldier recording; named with the first build that can show it); the times with the determinism flag
  off (no build without the flag exists in 4c); PM-0004-31's 48-soldier runs, which PM-0004-35 counts over.
