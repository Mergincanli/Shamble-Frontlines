Task: R7 coverage closer (assign unnamed top-level declarations from coverage-final.json to inventory rows)
Note: coverage-final.cjs bare-number regex has lookbehind (?<![\w.#x/:]) so ":848-851" style refs are never counted as ranges; many "unnamed" decls are inside cited ranges. Fix by naming them anyway.
Game (sed -n each line + grep -nw usages):
GE6: _hvDir :821 (used 870-871), damageHarvester :848, respawnHarvester :850 -> replace ":821 harvesters, ... :848-851 damage/destroy/respawn/reset"
GE10: siloRateEl :974 (written :990)
GC3: _v/_p1/_p2 :1110 shared scratch (also 1429 pitchWeapon, 1406/1760 throws, 1617 enemyFire, 2501 dmg numbers, 2543 blastReaches)
GC10: _devWheelT :1168 (wheel listener :3294, 160 ms)
GC5: _sRight/_sUpv/_sDir :1414 (also enemyFire :1618), _fireO :1419
GC12: _melTo :1669 (meleeHitCheck actually starts :1670)
GC13: _arcGeo/_arcDots/_arcRing :1702-1704 (+_arcHub :1705)
GE17: pips :1863; GE15: _progSaveTO :1955; GE16: ccIconEl/ccLvlEl/ccPrestEl/ccXpEl :2124
GE4: era*El :2418, _eraF/_eraTF/_eraH :2419; GW17: _occHit/_occRec/_occBox :2556, _occSX/_occSY/_occSZ :2557
Editor (sed -n each line + grep -nw usages):
ED16: _pv1/_pv2/_pvAim :59 (edPreviewUpdate 663-673, edMarqueePick 180)
ED2: append gizmo state :77/:78/:83, ring :115-121, _edStartResize :122, mirror :163-166 (24 names)
ED5: clUndoPush :298; ED8: append measure :476/:491, spawn :495/:503, symmetry :530/:531 (15 names)
ED4: _edOrthoH/_edOrthoA :646, edCam :648, edSyncOrtho :649, edToggleOrtho :652
ED9: _edRulesCtl :769, _rulesFmt :787; ED12: edInspBody/edInspKind :863, _edInspRef/_edInspHidden :864
ED1: _edLibCollapsed/_edLibMode :864 (edSetLibMode :856, :971, :1209)
Ops in scratchpad ops.json; validated: every find occurs once; after ops 0 of 36 game / 56 editor remain unnamed.
DONE
