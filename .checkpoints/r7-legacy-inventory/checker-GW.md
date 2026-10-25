# checker-GW (R7 fixes2 checker, area GW)
- Ran apply-ops.py GW: 3 ops applied, only GW20 where/what/v2_note differ from final-GW.json (21 rows compared).
- GW20 where: correction's "so it reads" wording applied exactly; shamble-editor.js:621 barracks, :622 spawn (mapSpawn.set), :623 resupply (terminalPos.set) confirmed.
- GW20 what: applied verbatim; shamble-game.js:2793 sameTeam && <2.6, :2795 nearTerm distanceTo(terminalPos)<6, E at :3267 -> openShop or tryResupply confirmed.
- GW20 v2_note: applied verbatim; LOOP_V2 §9 :116-117 "structures players can enter [LATER]", §11 :156-157 "Not in v0 ... enterable structures" confirmed. mark/look_brief_pending untouched.
- GW21 was confirmed (no work).
DONE: ok=true, no problems.
