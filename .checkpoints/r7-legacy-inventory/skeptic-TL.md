Task: R7 skeptic for area TL (tools) rows TL1-TL17
Note: V2_PHASE0_BRIEF.md and VISION_V2.md are NOT in 48ffb51 (git ls-tree); verified against /home/user/Shamble-Frontlines working copy (6d278c4, which differs from 48ffb51 for tools/, docs/reports/, STATUS etc only by kit-v3-0 report + tools/tuning-dump.js). BRIEF:125 = R5 row (Cargo workspace, Vite, Vitest, Playwright, golden tests vs legacy); BRIEF:22 'CI gate v2'.
TL1 corrected: where noSmoke path is gate.mjs:373-374 (not in :186-223); evidence stale-main is gate-check-5.md:42 not :41; what: exit 2 = usage or setup error (die() :37, e.g. missing npm deps :64, bad base ref :74) incl. 5a without Playwright/Chromium when nothing else failed (:21, :213). Rest confirmed (STATUS:102, :125; gc5 :8,:43,:44-45).
TL2 confirmed (gate.mjs 104-127,131-140,144-157 incl outside-repo refusal :151, check1 :225-235, check2 :237-247; phase-0:26 (c) syntax->FAIL[1]; gc5:27).
TL3 corrected evidence only: phase-0.md:25-27 (refreshRect drop :25, wrType dup :26-27, --allow-drop & pure move :27), not :26 alone. Code refs confirmed (battery 70-104, 208-214; gate 249-275). repo-root:16 +0/-0 confirmed.
TL4: code/evidence confirmed (battery 106-205; gate 276-283; STATUS 125-126; phase-0:26,:39-40; EDITOR_V2:51-52). Mark: consider consistency w/ TL7 (rule carries into v2 per EDITOR_V2 §5) -> decide.
TL5 confirmed (battery 18-63; gc5:18,:38,:40-41; STATUS:99).
TL6 confirmed (gate 159-184, 285-336, 287, 291; gate-globals 14 browser + THREE; gc5 :35-37, :53-54; README 34, 38-40; STATUS:104).
TL7 confirmed (gate 338-347; gc5:38; BRIEF:125 gate v2/repo layout).
TL8 confirmed (gate 349-370; gc5:34, :38-39).
TL9 facts confirmed (smoke.mjs 32-39,40-114,118-193,147-157,159-165,170-173,196-208; gc5 :28,:31-32,:68,:44-45; README:20 '~1 s'). Mark corrected -> redesign (R5 gate v2 replaces today's gate incl. 5a; probes and network rule change per its own note; 'port to TypeScript' is for client code).
TL10 confirmed (hook 12-46, nodeCheck 16-19, regex 29-33 w/o comment handling, type allowlist :33; settings.json:97-101; phase-0:29-30; repo-root:28; STATUS:7-8).
TL11 confirmed (preview.js 13-17, 23-84, 86-89, 92-96, 98-109, 110-124, 125-132, 134-154, 156-170; STATUS:124; ANIM_PLAN:4; phase-0 :36,:63; ttc:30; map-dedupe:32).
TL13 evidence: three-teams-core.md:44 only (:45 is the fairness probe, not sim.js). Others ok: STATUS:122, bpw:27, tpf:87, ttc-1-6:42, phase-0:34.
TL12 evidence: editor-hotfix.md:44 does not name pageshot.js ('trusted Playwright input' only) -> drop it; editor-prefab-list:26-27 ok.
TL12 code confirmed (pageshot 33, 34-36, 38-44, 45-54; render-ab SKILL uses pageshot; phase-0:16,:35; ttc:51). Mark port-to-TS: keep (harness shape kept; defensible).
TL13 code confirmed (sim.js 5-60, 13-16, 28-38, 42-50, 53-59). LOOP_V2:153-155 ok, STATUS:111 ok, VISION_V2 §6 item 6 telemetry ok (main copy :76).
TL14 confirmed code (reach.js 14-81 parts); evidence ok, small fix: ttc:83-84 86.5 % is DM_Tri. EDITOR_V2:39, LOOP_V2:126-127, VISION §6 item 2 ok.
TL15 confirmed (perfbots 15-47, run 21-27, fairness 30-38, render 40-45, header 9-10; ttc :48,:49,:50,:54-55; STATUS 83-84,119).
TL16 confirmed (aabbs 2-15; village 15-31, 33-64, 57-64 values, 65-84, 86-115, 117-149, nudge <=12 m :129; phase-0:36,:44; bpw:37; TECHNICAL:231).
TL17 confirmed, evidence nit: npm audit is phase-0.md:42-43 (not :41). package.json pins ok; lock three :1998-2000, playwright :1459-1461; kit-v2-1 :20-21, :45; gc5:71; STATUS:7-9.
Mark decisions: TL4 -> redesign (consistency with TL7; EDITOR_V2:51-52 keeps the rule, plans editor as module). TL12 -> redesign (tool, not client code; R5 Playwright; renderer probe changes to WebGPU).
Missing systems: none; all 14 scoped files at 48ffb51 covered; gate.yml covered by KT8; tools/tuning-dump.js not at 48ffb51.
DONE
