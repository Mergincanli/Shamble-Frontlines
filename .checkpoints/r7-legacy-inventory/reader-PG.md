Task: R7 legacy harvest, area PG (pages: index.html, editor.html, ui_theme_mockup) at 48ffb51
## checks
- checkout HEAD bed4978; git diff 48ffb51..HEAD touches no index.html/editor.html/docs (only tools/tuning-dump + checkpoints) -> line numbers valid
- pages: index.html 1266 lines, editor.html 1269; diff = header comment 3-9, editor adds mEditor button (1046), editor script tag (1259), bootEditor() (1262). Everything else identical (editor DOM + charLab DOM ship in index.html too)
- VISION_V2.md and V2_PHASE0_BRIEF.md NOT in checkout
- CSS 16-734; legacy tokens :root 17-24; wr- tokens 225-232; ed tokens 328; CS14 families 718-733
- z stack: halftone 8, chips/era/tbox 9, hud 10, dmgNumbers 11, reticle/rotDot/hitmark/reloadRing/label 12, concussion 13, levelToast/skull 14, death/xpEarn 15, mk 16, rank 17, end/editor 18, pause 19, title/killStrip/edPanels 20, manual 21, edMarquee 22, edPvBanner 25, scoreboard/shop/edModals 30, career 31, tutorial 33, dev 40, charLab 42, edTip 60
- dead CSS: #topbar 90-93, #hint 129-131 (no DOM, no JS); #edGenHead/#edGenNote 537-538 (DOM uses classes 1211-1212). #halftone CSS 30 element made by core updateHalftone 149 (default off)
- dvCharLab button (878) only wired by editor.js 1030 -> dead in index.html; mEditor wired editor.js 1023
- tutorial 938-958: lists V melee, B shield, 1-5 (no KeyV/KeyB game handlers; game keys 3264-3270: R,F,G,E,1,3,2); STATUS:89; shown on first deploy via maybeShowTutorial in deploy 1107
- three control lists: tutorial 941-953, pause wr-keys 989-993, shop note game 1897
- HUD bases 748-768 hardwired f-/h- two teams + 2 silo dots; STATUS:85 risk 4; scoreboard 997-1028 two columns, header 'Cabana Republic · Base Assault' hardcoded 1000
- evidence: AUDIT CS3 (83-86) 0.01 real writes/frame ~99.7% skipped; CS7 (107) dmg numbers pool 40; CS1/B1 (22, 62-68) ~43 ms/shot CPU removed; CS14 (114) two families; team-prep B5 38->0 DOM changes/frame (report l.68, PERF-5 +16 B); STATUS:45 UI transplant step1 delivered battery PASS awaiting Brave; STATUS:99 + gate-check-5 l.56 + phase-0 l.47-48 no SRI/CSP; STATUS:107 scene renders behind opaque menu; gate-check-5 l.27 5d exact order, 22/22 mutation proofs
## rows (final)
PG1 two entry pages + duplication: C, drop, look F
PG2 tokens + two UI families: C/M? -> C (CS14 is D), redesign (look brief pending), look T
PG3 main menu dossier: C, port TS, look T
PG4 pause clipboard + field manual + options: C, port TS, look T
PG5 first-deploy tutorial: C, redesign (STATUS:89, ROADMAP:71, LOOP_V2 §12), look T
PG6 combat HUD + cursor layer: M, port TS, look T
PG7 objective/economy HUD: C, redesign LOOP_V2 §3,§4,§8,§11.1, look T
PG8 feedback banners + kill strip: M, port TS, look T
PG9 armory shop modal: C, redesign LOOP_V2 §2,§3,§8, look T
PG10 career modal + chip: C, redesign LOOP_V2 §8, look T
PG11 scoreboard: M, redesign LOOP_V2 §8 §11.1, look T
PG12 death + end screens: C, redesign LOOP_V2 §2 §8 §11.6, look T
PG13 dev panel DOM: C, redesign (Feel Lab §7, server admin), look T
PG14 Character Lab DOM: C, port TS, look T
PG15 map editor chrome: C, redesign EDITOR_V2, look F
PG16 overlay z-order/modal states: C, port TS (layer table), look T
PG17 script tags + inline boot + menu tail: M (gate 5d), drop, look F
PG18 three r128 CDN tag: C/D, drop, look F
PG19 dead CSS/markup: C, drop, look F
PG20 ui_theme_mockup.html: C, redesign (look brief pending), look T
## final reading order renumbered (supersedes list above)
PG1 shell+two pages drop | PG2 tokens/families redesign-look | PG3 combat HUD+cursor port | PG4 objective HUD redesign §3/4/8/11.1 | PG5 scoreboard redesign | PG6 death/end redesign | PG7 menu dossier port | PG8 pause/manual port | PG9 editor chrome redesign EDITOR_V2 (look F) | PG10 feedback banners+kill strip port | PG11 shop redesign | PG12 career redesign | PG13 dev panel redesign | PG14 Character Lab DOM port | PG15 tutorial redesign | PG16 z-order layer table port | PG17 dead CSS/markup drop | PG18 script tags+inline boot drop | PG19 r128 CDN drop | PG20 mockup redesign-look
driver lines verified: setPause 1104, deploy 1107, placeCursor 3045, renderShop 1894, openShop 1869, showLevelToast 1866 (innerHTML), renderCareer 2144, popSkull 2244, triggerVignette 2384, sbBuildRosters 2467, sbShow 2490, updateBaseHUD 2491, _sd/_so/_st 2606-2608, setMenuCam 2967, backToMenu 2983, WR 2997, sfxStampThud 3007, sfxTypeTick 3012, wrDeploy 3024, openManual 3035, wrBoot 3039, loop 3053, bootGame 3055, bootEditor editor 1022, DEV 786, devToggle 2161, TutStore 2335
DONE
