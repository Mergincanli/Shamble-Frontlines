Task: R7 verifier spot-check, group tlkt (areas TL, KT): M rows in TL/KT + sampled KT2, KT12
Rows to check: TL1-TL17 (all M), KT7, KT8, KT9 (M), KT2, KT12 (sampled C).
TL1: gate-check-5.md:8,42,43,44-45; STATUS:102,125 (sed -n); gate.mjs:15-100,71,35,186-223,213,220,372-375. All confirmed.
TL2: phase-0.md:26 (c) syntax error FAIL[1]; gate-check-5:27; gate.mjs 104-127,131-140,144-157 (151 outside repo),225-235,237-247. Confirmed.
TL3: phase-0.md:25-28 (sha 20d1ff0c), STATUS:125 (134/691/349), repo-root.md:16 (+0/-0); battery.js 70-104, 208-214; gate.mjs 249-275. Confirmed.
TL4: STATUS:125-126 (44 guarded, 0 flagged); phase-0:26 FAIL[4] FLAGGED; phase-0:39-40 607b04b; battery 106-205; gate 276-283. Confirmed. EDITOR_V2 quote pending check.
TL5: gate-check-5:18,38-41; battery 18-19,23-38,40-41,43-44,46-47,49-63. Confirmed. STATUS:99 SRI/CSP ok. "syntax hook does not use scanner" pending (TL10).
TL6: gate-check-5:33-38 (actual mutations at :35-37), :53-54; README:34,38-40; STATUS:104; gate.mjs 159-184, 285-336, :287, :291, :333; gate-globals 14 browser + THREE. Confirmed.
TL4 extra: EDITOR_V2:51 actual text "The player build (index.html) must never load editor code." Row quote drops "(index.html)" inside quote marks -> nit.
TL7: gate-check-5:38 '5c: core calls game'; gate.mjs 338-347. Confirmed.
TL8: gate-check-5:34 (5a+5d swapped tags, defer), :38-39 (unquoted src = 5b+5d, text/ecmascript = 5d); gate.mjs 349-370. Confirmed (unquoted src also fails 5b; fine).
TL9: smoke.mjs 30-39,40-114,118-193,147-157,159-165,170-173,196-208 all match; gate-check-5:28-29 (16.7-19.3 s, 0.9-1.6 m),:31-32,:68 (9.4 s, 30 s),:44-45 (57/59 s, apt ~20 s); README:20 '~1 s after last step'. Confirmed.
TL10: hook :12-46,:16,:29-33 confirmed; divergence claims true (regex ignores comments; type filter skips text/ecmascript). phase-0:29-30, repo-root:28, STATUS:6-8 confirmed. BLOCKING: '.claude/settings.json:101' is a WebFetch allow line; the hook wiring is settings.json:162-169 (command at :167) (grep -n syntax-gate .claude/settings.json).
TL11: preview.js all Where ranges match (13-17,23-84,86-89,92-96,98-109,110-124,125-132,134-154,156-170); STATUS:124 (1992/2406/3502 vs 1992; 119.5 vs 44.5 MB); ANIMATION_PLAN:4; phase-0:36,:63; three-teams-core:30; map-dedupe:32 (29 assertions). Confirmed.
TL12: pageshot.js 15-57, :33, :34-36, :38-44, :45-54 match; phase-0:16,:35; three-teams-core:51; editor-prefab-list:26-27; STATUS:6-8; render-ab SKILL uses pageshot. Confirmed. (pageshot does print console errors and warnings, does not block network: true.)
TL13: sim.js 5-60, 13-16, 28-38, 42-50, 53-59 ok; STATUS:122 (8.82/7.5 vs 7.73/6.5), STATUS:111 (11 places); poplar-wedge:27; team-prep:87 (13.0/5.7/7.3 vs 3.4/16.1/6.5, mean 8.67 both, arithmetic ok); three-teams-core:43-44; steps-1-6:42; phase-0:34; LOOP_V2:153-155. Confirmed.
TL14: reach.js 14-81, 18-23, 29-36, 39-62, 64-69, 70-75 ok; three-teams-core:41 (180 routes 100/100/100; Cabana 100/72.5/75), :83-84 (86.5 %); STATUS:82 (27.5/25, R19); EDITOR_V2:39; LOOP_V2:126-127. Confirmed.
TL15: perfbots 15-47, 21-27, 31-38, 40-45, header 1-14 ok; three-teams-core:48 (0.14-0.21; 0.80-1.00, p95 0.97-1.54), :49 (2.55/2.54/2.47), :50 (1,480 / 5,843), :54-55 (~20x); STATUS:83-84, :119. Confirmed.
TL16: aabbs 2-15; village.py 15-31,33-64,65-84,86-115,117-149 ok; RULES 57-64 values (4/8/20/15/8) ok; nudge <=12 m ok; phase-0:36,:44-45; poplar-wedge:37; TECHNICAL:231. SHOULD: v2_note cites EDITOR_V2 ':37-39' for 'symmetry tool ... 120 degrees'; symmetry is EDITOR_V2:35-36, validation panel :39-40 (37-38 = objectives as data).
TL17: package.json 1-21 pins all match; lock three :1998-2000, playwright :1459-1461 ok; README 6-14,16-27,29-45,51-61,66-69 ok; phase-0:42-43 (0; 1 critical + 4 high; node-gyp 10.3.1 -> tar 6.2.1); kit-v2-1:20-21,:45; STATUS:6-9; gate-check-5:71. Confirmed.
