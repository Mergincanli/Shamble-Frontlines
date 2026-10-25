Task: R7 legacy inventory skeptic for area PG (pages), rows PG1-PG20
Checkout: HEAD dac3948 = 48ffb51 + checkpoint/tools files only (git diff 48ffb51 -- pages/js/docs empty). VISION_V2.md absent.
Facts: diff index/editor = 4 hunks (header 3-9, 1046 mEditor, 1259 editor tag, 1262 bootEditor). gate-check-5 22/22 at l.8, 5d exact order l.27. TECHNICAL:32-33 ok.
All cited game fn lines hold (2413,2414,2606-8,3045,2506,923,990,2420,2491,2467,2490,2997,3000,3007,3012,3015,3024,3039,1104,3035-7,1866,2135,2142,2244,2329,2384,1869,1894,2126,2144,2157,786,2161,2339,2341,2215,3053,3055,2967,1101,1107). TutStore const at 2336 (2335=tutorialEl/TUT_KEY).
AUDIT 22 B1, 62-68 CS1 ~43ms, 83-86 CS3 0.01/99.7%, 107 CS7 40 nodes/60 spawns, 114 CS14 ok. STATUS 45,55,85,89,95,99,107 ok.
Reports ok: 3teams l.27,l.101; team-prep l.67 B3, l.68 B5 38->0, l.103 PERF-5, l.172; phase-0 l.35, l.47-48; repo-root l.31-33.
PG9 evidence wrong: editor-hotfix l.40-42 are feature rows E2/E4/E5, not Brave; Brave passes = STATUS:55 (prefab) and STATUS:95 (hotfix PR #13).
z-index lines all match PG16 list; values missed by summary: 9 (eraHud,tboxHud,careerChip,staminaBar), 11 dmgNumbers, 13 concussion, 15 deathScreen/xpEarn, 20 killStrip & edWorld/Rules/Maker(437), 21 pause.wr-manual, 22 edMarquee, 25 edPvBanner. keydown gates actually 3253-3262 (tutorialOpen 3258, shopOpen 3260, paused 3262). Many overlays toggle by .show class not display.
Keys bound 3264-3271: R,F,G,E,1,3,2,[ ; Tab 3254, Esc/P 3261, Backquote 3253. No KeyV/KeyB. Shop note game:1897 ok.
PG8: aim/audio settings NOT persisted (adsMode='hold' game:32; 3299-3300 save nothing) -> v2_note KVStore claim wrong.
PG2: --ed-* tokens at :325-330 on #editor (rule :324), not :328 alone. mockup :8-17 tokens = wr- values minus --wr-faded. Four sets total (legacy, wr, ed, mockup).
PG14: enterCharLab at editor:432 not 438.
PG17 extra dead: .cl-hint CSS :635 (no element/JS); #pipWrap/.pip2 created by game 1862-1863 but always display:none (updateRank 1865).
CSS ranges of PG3/4/5/6/7/8/10/11/12/13/14 verified ok.
PG3: #reloadBar dead: CSS :87 display:none, JS only sets opacity (game 1162,1379,1387,1412...). #credits readout :795 not mentioned in PG3/PG4.
PG4: pips ok (f-harv labelled SUP); era uses firstEnemy (2421), base pips firstEnemy (2491). ok.
PG5 ok (:1000 header, :1001 'Round 01', :1026 foot 'destroy the enemy HQ').
PG6 ok (:967-968, again text 'Click or press R'; R->resetRound 3264).
PG7 DOM ok; aria = aria-current/aria-pressed/aria-hidden/nav aria-label (no per-button labels).
PG9 ok; also load-time IIFEs editor:1133 (Maker docked into generators) and :1246 (World/Rules into #edIEmpty).
PG10 DOM ok.
PG11: shop lists Squad classes (1899), Vehicles (1914), Special Weapons (1927), frag (1940); rows' info built by innerHTML string concat at 1903,1918,1931 -> 'textContent' claim wrong.
