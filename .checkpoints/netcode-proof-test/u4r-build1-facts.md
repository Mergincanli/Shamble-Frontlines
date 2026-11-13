# U4r build copy 1: fact check for the advisor (read-only, adversarial)

Sources: questions u4r-build1-questions.md; notes u4r-build.md; code .gate/u4r-snap1 (2fbec77 + wip/u4r.patch);
logs .gate/u4r/; brief u4r-brief.md; carries u4s-carries.md; plan.md.

## Plan
1. Read notes (282 lines), questions (done).
2. Point 1: logs regen-all.log, regen-precheck.log; check.mjs wasmStep; hash.rs legs/regenerate_all/--only;
   33.1/61.6 base source; WASM_CALL_LIMIT_MS location and scope (carry 84); arithmetic for (a)-(d).
3. Point 2: npt-room RADIUS_CM, AreaFlood visibility; interest set build; mirror.rs checks.
4. Point 3: run file sizes; lab.jsonl -57 per-tick; SERIES.md:66-73; brief 7.1 table; df; du shared-target.
5. Point 4: client refusal causes; L1 P0 fates; logs.rs refused_as_lost; carry 62 (h).
6. Point 5: HISTORY_MS, ROOM_HISTORY_MS, span_ms writer.
7. Point 6: World dump events section; shot counting.
8. Point 7: Behaviour::step view.
9. Point 8: d13.log vs notes table; U4R-Q14 arithmetic.
10. Point 9: --lead, --every-step, --logs 0 refusal/limits.

## Items (a line per item, marked done)
