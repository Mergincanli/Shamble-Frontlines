# Task: R7 legacy inventory, game-designer review of rules rows (GE all + rule rows of GC GN GW CD GP) against LOOP_V2
Context: LOOP_V2 [OPEN] tags only at §5 (Spy cost + spy details), §9 (aiming at height, map size). No [OPEN] on economy/eras/win/spawn.
Rows in scope: GE1-17; GC1,2,4?,9,10,11,17,19; GN2,3,9; GW1,2,10,20; CD4,7,10,11,12,13,14; GP11,(GP3,GP14).
GE1: sed game.js:40-44; RULES_DEFAULT matches tuning json. OK.
GE2: sed game.js:688-704, core:81; ECO numbers match json. OK.
GE6: sed 845-876: refinery-dead self-destruct, respawn 12 s, 600 cr only to player, 5 tech. OK.
GE3,GE5,GE13: sed 781-783,1523,1809-1813,2432-2435,2456,1942,2693-2695,1872-1893,1175-1184,1357-1363. Numbers OK. GE13 omits grenade 50 (hard-coded) from 'only costs'. unitOwned persists through death until round reset (:28,:1846) -- check whether any row says so (LOOP_V2 §5 death loses unit).
GC1,GC2: rows read; GC1 Rust OK (team filter kept). GC9: sed 1825-1827, 2939, core:77-78: 2.0/4.0/1.4x OK. GC10 says owned for round (matches :28/:1846). GC11: sed 2755-2774: 1.3x, 2.4 m OK; abilities hard-coded literals (1.3, 2.4) not in tuning json.
GC4,GC17,GC19 read; GC17 numbers match JUG/JUG_PILOT/JUG_CANNON. GC19 Rust while LOOP_V2 silent: row flags it for Mustafa, OK. GN2,GN3,GN9 read: marks redesign citing §2/§5/§10 correct; GN9 siege 11 dmg, 1.6x cadence (:2923,:2927) OK.
GW1,GW2,GW10,GW20 read. GC2 v2_note says 'LOOP_V2 §2 has three teams always playing' -> conflicts with VISION_V2 §3 [DECIDED] two-team modes; activateRoster :762-779 already supports 2-of-3 teams (Cabana). FINDING should. GE12 sed 1832-1850 OK. GE15 sed 1951-1957,2095-2116 numbers OK; total XP to L100 = 2,175,855 (~43.5k kills at 50 XP).
CD4,CD6,CD7,CD10,CD11,CD12,CD13,CD14 read; numbers match json. Bot fire model (:2913, :1627): bots use UNIT_TYPES enemyDmg/burst/fireInterval + 4.5 s pause = 1.2-6.2 DPS vs player weapons 9.7 (AMK) / 51 (Bagpipe) / 65 (Shabzon) sustained. Counter triangle unmeasured; PK>Warrior at Tech 0 needs >=11/16 pellets. FINDING should on CD13.
GE4 sed 2420-2440 OK. Tier off-by-one: legacy era starts 1, unit tier t buyable at era t (:1178) = after t-1 era-ups; LOOP_V2 §4 T2 units at Tech 2 (2 tier-ups). CD13 'Officer at tier 2 (already so)', GN2 'where legacy already has it' misleading; GC17 too. FINDING should. GE5 Rust covers tech halving not in LOOP_V2: should. GE1/GW2 duplicate row (both game.js:40-44). GP3 arenaX 60 vs live 186x126 (core:225, json). Tech pace DM_Tri ~57-78 tech/team/300 s -> era 2 at ~4.6-5.3 min.
GP11: two-column board; v2 note omits two-team modes (nit). GP14, GP3 read. GC3 carries 'rule pending LOOP_V2 [OPEN]' for aim at height: OK.
Hard-coded rule literals not in legacy-tuning.json: XP 50/150, chain 5 s (:2099-2100), curve 500/110/5 (:1957), cap 100 (:2106), priceMult 2 (:782), tech cut 0.5 (:2434), grenade 50 (:2456,:1942), refinery respawn 1.4x (:2939), siege 1.6x+0.4 (:2923,:2927), bayonet 1.3x/2.4 m (:2756,:2761), banzai 1.3x (:2774), terminal 6 m (:1811), panel 2.6 m (:2793). FINDING should (GE15 + LEGACY_TUNING).
Find strings verified once each (CD13, GN2, GC2, GP3, GE13, GE5).
Findings: CD13 tier off-by-one (should); GN2 same (should); GE4 append mapping+pace (should); GC2 two-team modes (should); GE5 tech cut outside Rust mark (should); CD13 counter triangle/bot model unmeasured (should); GW2 duplicate of GE1 (should); GP3 arena 60 vs 186x126 (should); GE15 hard-coded literals (should); GP11 two-team (nit); GE13 grenade price (nit).
Verdict: APPROVE with fixes (no blocking; marks follow condition 3).
DONE
