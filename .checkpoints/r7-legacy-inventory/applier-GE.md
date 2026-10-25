applier-GE: R7 fixes2 applier for area GE (corrected checks GE14, GE15, GE16, GE17)
GE14: what set (whole new text); where extended with the new cites via replace on the tail ":2112 and :2429)" (so the separator reads "; " not " ; "). All cites checked in code (pickClass :1181-1184, :1896, :1933, :1934, :1947, :1363, :3254, :3260).
GE15: where replace "DEV :2212-2213" -> ":2212-2214 (devSetLevel, devPrestige, devReset)"; what append (per-weapon kills credit weapon in hand, :1536, :1775). Checked in code.
GE16: what replace of the opening clause (US-Army insignia, 18 RANK_TIERS :1959-1978, 10 PRESTIGE :2050-2061) + append scoreboard sentence; where extended via replace on tail ":2327-2328". Checked: 18 tiers, 10 medals, :2468/:2477/:2484.
GE17: system set, what set, where set, v2_note set; evidence: roundAssists clause removed by replace, grep sentence appended with the regex reworded in words (rule 3: no pipes), mark and reason untouched. Lookups checked by grep.
ops file: fixes2-ops-GE.json, 13 ops, apply-ops.py GE applied cleanly -> after-GE.json
DONE
