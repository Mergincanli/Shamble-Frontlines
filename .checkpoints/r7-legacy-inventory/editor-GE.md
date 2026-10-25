# editor-GE: R7 legacy harvest, editor of area GE
started; inputs loaded
Lines verified at current main (git diff 48ffb51 HEAD on engine files and pages empty).
Skeptic corrections to apply: GE1 where; GE3 evidence; GE4 what; GE5 where; GE7 v2_note; GE8 v2_note; GE10 where; GE11 where; GE14 evidence; GE15 v2_note+reason; GE16 evidence; GE17 what/where/evidence. GE2/6/9/12/13 confirmed.
Plan for missing/coverage: GE9 widen (tboxMesh :888/:897, HUD handles :900, _tbSec :901, tbRGB :902 used :925/:932/:990); GE12 widen (gameOver/roundDeaths :783, :1837/:1840/:1846, guards :858/:942/:994/:1507, scoreboard :2477/:2489, onDeath :2104); GE14 widen (nearPanel/shopOpen :784, :3053, :1718, :3284, :2112, :2429); GE15 widen (chainTimer :2095, :2099-2100, decay :2734, onKill caller :1536 player only, onDeath caller :1826); GE16 widen (_svgU :1982/:1991/:2064, :2124-2125, _mkTO :2134, _rbTO :2140, xpEarn :2327-2328); GE17 widen (_xpPopTO :2133 only warm-up, ccPopEl :2124 never used, roundAssists :783 never incremented).
Rejects: terminal object + panelPos (GW20), updateBaseHUD (PG4, GP10), NEW team structures row (GW20), 783 kills/grenadeCount/meleeCd (GC7/GC14/GC18), credits (GE3), 793 enemies (GN1).
GE1,GE3,GE4,GE5,GE7,GE8,GE10,GE11,GE14(evidence),GE15(v2_note,reason),GE16(evidence),GE17(what/where/evidence): skeptic corrections applied as written; lines re-checked by sed.
GE9: widened with tboxMesh/tboxBar :888/:897, HUD handles :900, _tbSec :901, tbRGB :902 (coverage + missing 5).
GE12: widened with gameOver/roundDeaths :783 and guards (coverage).
GE14: widened with nearPanel/shopOpen :784 (coverage).
GE15: widened with kill chain :2095/:2099-2100/:2734, callers :1536/:1826 (missing 4).
GE16: widened with _svgU :1982, handles :2124-2125, timers :2134/:2140, :2327-2328 (coverage).
GE17: widened with roundAssists (never incremented), ccPopEl :2124, _xpPopTO :2133 (coverage; grep counts).
final-GE.json written (17 rows).
DONE
