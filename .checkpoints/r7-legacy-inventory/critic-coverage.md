Task: R7 legacy harvest, coverage critic (top-level declarations of core/game/editor vs inventory Where fields)
Method: acorn top-level declarations (Program body: function/class/var-let-const, each declarator name) per file; scripts in scratchpad/critic (decls.js, match.js, rows.txt = all rows' Where incl. PG rows touching engine files).
Counts: core 138 decls (126 col-0 decl lines by grep), game 714, editor 353.
Check core: 138/138 covered by name in some Where (all verified by line too). DONE core.
Check game: 714 decls; 584 covered, 130 uncovered (106 no name/line hit; 21 line-only hits rejected as not owned by citing row; 3 weak prose-only name hits rejected: buildings, kills, enemies).
Uncovered game groups:
 A structures+barracks: buildings@492 makeBuilding@493 terminalPos,mapSpawn,termMesh,resupplyLabel@521 wallMat@524 bRoofMat@525 bFloorMat@526 barracksList@527 makeBarracks@528 (11) -> new row GW/GE game:491-550
 B editor transform helpers in game: _rotAround@2970 _scaleAround@2971 _scaleBuilding@2972 _scaleBarracks@2973 _rotateBarracksBy@2978 (5) -> ED game:2969-2982
 C anim scratch _aV,_aQ@231 _aE,_aY,_aDn,_aKnee@232 _aPoleR,_aPoleL,_aHold@233 (9) -> GA1
 D round state grenadeCount,kills,roundDeaths,roundAssists,gameOver@783 shopOpen@784 (6)
 E bots enemies@793; _eEyes,_ePc@1113 (3)
 F tech box mesh/HUD tboxMesh,tboxBar@888 tboxHudEl,tbNameEl,tbTimerEl,tbBarWrap,tbBarFill@900 _tbSec@901 tbRGB@902 (9)
 G aim raycast raycaster@1109 groundPlane,_v,_p1,_p2@1110 aimPoint,aimGround,aimHoverEnemy,aimPart,aimTarget,aimBlocked@1111 _ms,_adir,_aq,_aup@1135 (15) -> GC3
 H camera camPitchOverride,adsPitchK,adsPanK@1094 camRecoil,camShake@1413 (5) -> GP4
 I cursor HUD reticleEl,hitmarkEl,rotDotEl@1096 _rUp,_rDown,_rLeft,_rRight@1097 reloadFg@1098 RR_C@1099 _lastGap@2512 _curX,_curY@3044 (12)
 J HUD handles ammoEl,ammoVal,reserveVal,weaponNameEl@1854 hpBar,hpBarI,hpNum@1855 slotsEl,dmgNumbersEl@1856 showReadyFlash@1857 dryFire@1858 staminaBarEl,staminaFillEl@2342 eraUsEl,eraFillUsEl,eraNumUsEl,eraThemEl,eraHudEl@2418 _eraF,_eraTF,_eraH@2419 _abLabEl,_abValEl@2444 respawnCountEl@2546 (24)
 K career UI _svgU@1982 chainTimer,chainCount@2095 ccIconEl,ccLvlEl,ccPrestEl,ccXpEl,ccPopEl@2124 careerModalEl,mkBannerEl,rankBannerEl,rbIconEl@2125 _xpPopTO@2133 _mkTO@2134 _rbTO@2140 (15)
 L shot/throw scratch muzzleW,aimDir@1413 _hpV,_hnV,_dmgPt@1442 _thrV,_simP,_simV,_simPv@1706 (9)
 M jug scratch _jugV,_jugV2,_jugV3,_JUP@1191 (4)
 N movement _wish@1115 WORLD_FWD,WORLD_RIGHT@2603 (3)
Noted drift: CL12 says :47 KILL_WORDS but :47 is _gw/_comicCd/_comicCache (GUN_WORDS@45? KILL_WORDS@46 per GW3).
