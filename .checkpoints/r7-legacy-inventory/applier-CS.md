applier-CS: R7 apply corrected checks of area CS (CS5, CS7, CS11) into fixes2-ops-CS.json
CS5: 3 replace ops (where rayStatic range 1519-1524 + comment 1517-1518; evidence audit row CS10 clarified; v2_note 'inline in rayStatics'). Code checked: core:1519-1524 holds rayStatic, 1517-1518 its comment.
CS7: 3 replace ops (where adds enemyFire consumer 1626-1628, bolts 2870-2873; what team re-check wording; v2_note linear loop wording). Code checked: game:1626 reads r.kind/r.id before applyDamage, applyDamage team check at game:1512, bolts 2870-2873 no team check.
CS11: what replace (core header also omits makeInputCommand; core:18-20 checked), mark set redesign, reason set.
Script applied cleanly: 9 ops, 9 fields, 3 rows -> after-CS.json.
DONE
