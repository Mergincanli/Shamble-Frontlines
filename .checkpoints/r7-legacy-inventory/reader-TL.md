R7 legacy inventory — reader TL (tools, verification toolkit)
## Checks
- HEAD a45c9fc = 48ffb51 + .checkpoints + tools/tuning-dump.js (R7's own, added after 48ffb51: out of scope). tools/ and hook otherwise identical to 48ffb51 (git diff --stat).
- Files read in full: gate.mjs 375, smoke.mjs 208, battery.js 214, preview.js 170, pageshot.js 57, sim.js 60, reach.js 81, perfbots.js 47, aabbs.js 15, village.py 149, gate-globals.json 22, package.json 21, README 69, syntax-gate.mjs 46. Hook wired at .claude/settings.json:101. CI .github/workflows/gate.yml (out of scope).
- Evidence: gate-check-5 report :8,:27-45,:68-71; STATUS :5-9,:82,:90,:99,:102,:104,:119,:122,:125-126; three-teams-core :30,:41,:48-50,:54-56,:99; phase-0 :26-46,:63; bot-poplar-wedge :27,:37; team-prep-fixes :87; kit-v2-1 :19-21,:45,:59-60; TECHNICAL :213,:227-241; ANIMATION_PLAN :4; V2_PHASE0_BRIEF :125 (R5 gate v2); EDITOR_V2 :39 validation panel; LOOP_V2 :126-127, :153-155.
- Known code-read divergence: syntax-gate.mjs:29-33 regex HTML scan checks commented-out scripts and skips text/ecmascript, unlike battery.scanHtml/scriptKind (battery.js:23-44).
## Rows
TL1 gate driver gate.mjs:15-100,186-223 (resolveBase, finish, exit 0/1/2, --json, --allow-drop) M redesign
TL2 checks 1+2 syntax/concatenation gate.mjs:104-157,225-247 (nodeCheck, pageScripts, classicParts) M drop
TL3 check 3 symbols battery.js:70-104 + gate.mjs:249-275 M drop
TL4 check 4 player build battery.js:106-205 + gate.mjs:276-283 M drop
TL5 HTML scanner battery.js:18-63 C drop
TL6 check 5b names gate.mjs:285-336 + gate-globals.json M drop
TL7 check 5c layering gate.mjs:338-347 M redesign
TL8 check 5d script order gate.mjs:349-370 M drop
TL9 smoke 5a smoke.mjs:30-193 M port TS
TL10 syntax hook .claude/hooks/syntax-gate.mjs:12-46 M redesign
TL11 preview.js host+renders preview.js:13-170 M drop
TL12 pageshot.js M port TS
TL13 sim.js M redesign
TL14 reach.js M redesign
TL15 perfbots.js M redesign
TL16 aabbs.js+village.py M redesign
TL17 package.json+lock+README M redesign
DONE
