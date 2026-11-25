# test-engineer, review round 1 of change-set 3b (head bbbf09e4, base 28254a73)

My copy: /home/user/Shamble-Frontlines/.gate/ci-wasm-wt/.gate/test-engineer-cs3b (detached at bbbf09e4).
Question: is it proven? For each claim, a test that fails if the claim is false; does the named evidence hold?

## Plan
1. Read: the resume card (C1..C8), plan s5 "Evidence" and s9 pass marks, ADR 0001 row PM-0001-34, ADR 0002 rows
   PM-0002-12/13/64, my own followups file (3A-FROM-F64, TE-NT7, PA-N2).
2. Claim 1 (decoder): read decoder_inputs.rs, count inputs from code; run release --nocapture and debug; list paths
   the inputs never reach (cap+1 with bytes present etc.) and check codec.rs unit tests cover each.
3. Claim 2 (PM-0002-12): presses.spec.ts vs the row word by word; press-tally.ts: are releases measured or derived;
   the bite-last-post result (stalePosts line).
4. Claim 3 (PM-0002-64 + milestones' browser half): loop.spec.ts; local_match.rs every_tick_runs_the_newest_post;
   host.test.ts.
5. Claim 4 (PM-0002-13 reading): worker-link.ts measurement vs the mark's quantity; what the report leaves out.
6. Claim 5 (allocation): step_alloc.rs; allocation.spec.ts limits 170/680 vs readings 551..602: flake risk.
7. Claim 6: every added/changed test file: assertions that cannot fail, names promising more, untested behaviour;
   re-run a sample of plants-3b.py; plant >= 5 own defects.
8. Claim 7 (3A-FROM-F64): make_f32_vectors.py narrowing(); native + wasm runs.
9. Claim 8: numbers in the card vs the logs under .gate/cs3b/.
10. Browser: build + run presses/loop/allocation specs in my copy (port 4173 check first, one at a time).

## Findings (one line each, as found)
