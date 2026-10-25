checker for area CS (R7 fixes2 ops check)
Ran apply-ops.py CS: 9 ops applied, only the 9 targeted fields differ from final-CS.json.
CS5: 3 ops match the corrections exactly (where, evidence, v2_note); core:1517-1518 comment, 1519 opens, 1524 closes rayStatic; AUDIT:110 is audit CS10. OK.
CS7: 3 ops match exactly (where, what, v2_note); game:1626-1628 enemyFire, 1512 team check in applyDamage, 2869-2873 bolts with no team check, building/harvester team checks at 1494-1495/1313/1132-1133; core:1497 loop over players, rayStatics linear. OK.
CS11: 3 ops match exactly (what, mark=redesign, reason); core:18-20 header lacks makeInputCommand; MODULE_MAP has no BTN/makeInputCommand; V2_PHASE0_BRIEF.md:7 Rust simulation core. OK.
Result: ok, no problems.
DONE
