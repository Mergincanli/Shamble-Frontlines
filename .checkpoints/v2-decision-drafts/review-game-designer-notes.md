Task: game-designer review of the v2 decision drafts (plan step 10, wave 1), prompt prompts/review-game-designer.txt, scope ADR 0001 (server decides: hits, picks, offers, drops, pickups, UPGRADE-SCOPE authority), ADR 0004 (sim rules), ADR 0005 ability pass-mark rows (condition 15). Reviewing at commit f149c94 (>= 9a03999).

## Progress
- 2026-11-02: started. No agent memory found. Notes file created.
- Read: brief, UPGRADE-SCOPE input, plan settled inputs + rules, advisor conditions (15 = author reviews pass-mark domain fit), LOOP_V2, VISION_V2, LEGACY_TUNING weapons/units.
- ADR 0001 read (parts 1-14, state table, messages, OQs, spike items). Candidate findings:
  F-A (high) 0001:193-194 + PM-0001-21 (0001:1140): aim point "within the weapon's range" is a command check; forged set includes "beyond weapon range" (r2:2048, 2062). Legacy ranges: katana 3.6 m, bagpipe 34 m, molotov 0 (LEGACY_TUNING:182-188). Top-down cursor routinely beyond -> Banzai swings, Molotov throws rejected/counted forged. Fix: client clamps point along fire ray to range, or server clamps not rejects.
  F-B (medium) 0001:383-384 track state = "levels" only; 0001:174 own state unspecified; 0005:930-931 (AB-8) asks ADR 0001 to give the predicting sim the owner's pick effects. Picks/ability modifiers on movement/weapon -> corrections each pick. Fix: own-state section carries the local soldier's effect rows/attributes.
  F-C (low) 0001:383 refusal "counted like a forged command" -> honest races (double click; offer closed while pick in flight) flagged.
  F-D (low/question) track state owner-only (0001:463): who sees picks is design (pillar 3 readability).
  F-E (low/question) active reload tap quantised to tick (0005:881 "judges the tap from the command's tick"); OQ-0001-7 covers shots only.
  F-F (question) carried cache public or spatial? (0001:462 "cache location" public).
- ADR 0004 read (parts 1-21, OQs, S-table, PM-0004-62..66). Candidate findings:
  F-G (medium) 0004:425 "presence counting for steals (GE8)" and 0004:750 GE9 row "steal by GE8's presence count": LOOP_V2 §4.4 says "holding E for a few seconds"; INV GE8 (LEGACY_INVENTORY:294) says contest rules not decided by LOOP_V2. No OQ on contested/3-team steals.
  F-H (low/med) LOOP_V2 §2 [DEFAULT] spawn protection 3 s + enemies cannot enter barracks: no part places them (0004 part 17 lists §3,§4,§8 only; part 13 flags movement/shots/sight, not per team); 0005:1799 says spawning inside barracks is ADR 0004's.
  F-I (low) 0004:433 "the pending offer of three" (one per track) + 0004:444-445 "drawn ... when a track levels" pre-empt AB8 queue option (0005:1194).
  F-J (low) money unit + rebuild split remainder (LOOP_V2 §3) not stated; f32 throughout (0004:232).
  F-K (low/question) bots taking pickups not in any OQ (AB10 = tracks only; OQ-0004-1 = structure).
- ADR 0005 pass marks PM-0005-18..32, 45, 46 checked; arithmetic verified (0.864 s, 121>=120, 63/19 hits, 12th hit). Candidate:
  F-L (medium) PM-0005-31 (0005 table row) "distinct candidates" decides AB8's open "are the three candidates always different".
  F-M (low/med) PM-0005-25 covers 2 of 3 triangle legs; Conscript beats Peacekeeper (Bagpipe 34 m range; blast skips Shield, CD8/GC7) has no mark.
  F-N (low) PM-0005-24 no mark for the minute-5 floor (LOOP_V2 §4 [DECIDED]); 5 min = 9,000 ticks at 30 Hz.
  F-O (low) PM-0005-29 u4 "the player drops W" assumes a voluntary drop (AB1/AB9 open); rephrase via death + fixture pickup.
- Next: write review file; check ENGINE_OVERVIEW/README lines for my domain; line numbers of exact texts.
- More found: F-P (medium) no command field or message names a purchase (0001:484 Input contents; catalogue 0001:482-498; checks 0001:191-197) while 0005:868-869, 920, 922 rely on "the purchase names a unit" checked by the authority. F-Q (medium, seam, outside named scope) 0006:342-350 record keeps picks only per slot (no tick) though 0004:446/454-458 emits them tick-stamped -> AB3's two measurements impossible. F-R (low) Feel Lab step 3 (0005:2166-2167) accepts a predicted start-then-undo the client could refuse (team tier is team-class, 0001:461). F-S (low) record keyed by slot (0004:431, 0005:925): a late rejoiner loses tracks; AB10 covers only human-takes-bot. F-T (low) pickup pool capacity/full rule + stress scene has no pickups (GL:397-418). F-U (low) pickup on "existing button edges" (0001:386) vs E for steals (LOOP_V2:44).
- Camera evidence for F-A: cursor up to ~28 m from look point, 44 m with ADS pan (r2:847-851, R2-INT-F18).
- Next: write review-game-designer.md (verdicts: 0001 REVISE, 0004 REVISE, 0005 ability PM rows REVISE (one row) ), then DONE.
- Review file written: review-game-designer.md (21 findings, 5 questions; verdicts 0001 REVISE, 0004 REVISE small, 0005 ability rows REVISE one row). Citations spot-checked.
DONE
