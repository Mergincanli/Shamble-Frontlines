Task: R7 legacy harvest, editor of area ED (apply skeptic corrections, missing systems, coverage suggestions; write final-ED.json)
Inputs read: rows-and-checks.json ED (15 rows, 15 checks, 8 missing, 8 not_covered); coverage.json 7 suggestions naming ED.
Code verified at HEAD a54a652 (git diff vs 48ffb51 empty for engine files/pages). editor.js has 1252 lines; bootEditor :1022-1252.
Line fixes found vs skeptic missing list: Export/Reset/Copy wiring is :1100 edExport, :1101 edReset, :1103 edCloseModal, :1104 edCopy (not 1104-1108); inspector empty-state IIFEs :1246-1247 (not 1244-1245); orphan comment :1249-1251 (not 1248-1250).
Plan: corrections ED4 evidence, ED5 evidence, ED15 where+evidence. Missing: snap/rot -> new ED16 (with coverage NEW core state); export/reset -> widen ED13; ghost footprint :61,:235,:687 -> widen ED1; pick plumbing :67-70 -> widen ED2; overlay infra -> widen ED8; drag tool pointer logic -> widen ED1 (:1066,:1080-1083,:1090-1093) and ED2 (:1056-1065,:1075-1079,:1089); inspector column -> widen ED12; orphan comments: :1249-1251 widen ED14, :46-48 reject (comment; markers GW10/GP15).
Coverage: game:2969-2982 helpers -> widen ED2 (not NEW: they are ED2's transform plumbing; buildMap also calls them, GW20); ED4 camera 65-66,646,+pan :1050-1052; ED9 shells; ED12/ED13/ED5/ED6/ED7 adds; ED3 edClipboard :218.
ED1: widened (ghost footprint :61-64, :235, :687-688; drag tools :1066, :1068-1071, :1080-1083, :1090-1093) - skeptic missing 3+6; inventory B7.
ED2: widened (pick plumbing :67-72, :76-84, :95, :99, :106-114, :125, :144, :147, :162; pointer :1056-1065, :1075-1079, :1087-1089; game:2969-2982) - skeptic missing 4+6, coverage WIDEN ED2 + NEW transform helpers (folded: ED2's plumbing; also used by buildMap :628/:636, GW20).
ED3: added :218 edClipboard (coverage).
ED4: skeptic evidence correction applied; widened camera state :65-66, :646, pan :1050-1052 (coverage).
ED5: skeptic evidence correction applied; added :279, :391-392, :447-450, buttons :1029-1031 (coverage).
ED6: added :300 _clSyncPoseUI, game:479 (coverage).
ED7: added :334 _alSubset (coverage).
ED8: widened overlay framework :460-473, :476, :495, :524, :526-527, :530-542, :552, :554, :558-559, :1049, :1106 (skeptic missing 5 + coverage).
ED9: added panel shells :714, :746-747, :768-769, :800 (coverage).
ED10, ED11: unchanged (confirmed).
ED12: widened :863-864, :867, :869, :873, :971-972, IIFEs :1246-1247 (skeptic missing 7 + coverage; A5, G4).
ED13: widened :701, :703, export/reset/copy :1100/:1101/:1103/:1104, maps modal :1109-1110 (skeptic missing 2 with lines corrected by code; coverage; O2, O4).
ED14: widened with orphan comment :1249-1251 (skeptic missing 8 part).
ED15: skeptic where+evidence correction applied; editor.html:311 in correction replaced by :1046 (grep id="mEditor"); what widened to match.
ED16: NEW core state + grid snap + rotation step :49-59, :458, :582-584, :1095 (skeptic missing 1 + coverage NEW); redesign EDITOR_V2 §3, §6 3.3; E4/E5.
Rejected: orphan comment :46-48 (comment, not a system; enemy spawn markers owned by GW10/GP15); edFootprint into ED2 (put in ED1, the placement ghost that uses it at :687).
final-ED.json written (16 rows).
DONE
