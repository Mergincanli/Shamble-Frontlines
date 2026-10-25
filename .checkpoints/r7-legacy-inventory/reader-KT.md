Task: R7 legacy harvest, area KT (kit/process) reader notes
Checks so far (2026-10-25): checkout HEAD a45c9fc; kit files identical to 48ffb51 (diff 48ffb51..HEAD only .checkpoints + tools/tuning-dump.js).
Read: CLAUDE.md(125 lines), settings.json(109), hooks/syntax-gate.mjs(46), NIGHT_SHIFT.md(45), security guidance(15)+patterns(21), 7 agents, 6 rules, 4 skills, gate.yml(92), STATUS, ADR template, reports/README.
Findings: engine-editor.md:7 "known bug refreshPrefabList" is stale (now top-level shamble-editor.js:1010, fixed PR #4). docs-keeper.md:5 + verifier.md:5 model: sonnet (contradicts Opus policy; kit-v3-0). NIGHT_SHIFT.md:6 window 00:00-07:15 vs Night Shift 3 cron 30 5 (UTC?) = 07:30 CEST -> outside window; Night shift 5 22 UTC = 00:05 CEST but 23:05 CET after DST (2026-10-25).
More checks: settings.json 33 allow / 16 ask / 36 deny (=85, matches kit-v1-2 report). docs/reports: 22 reports + README. VISION_V2/V2_PHASE0_BRIEF/REPO_MAP absent at 48ffb51 (read VISION_V2 §6 from live repo as background only).
Rows (all state C unless noted; marks):
KT1 CLAUDE.md instructions+hard constraints CLAUDE.md:1-36,104-125 | redesign | v2 constraints come from ADR 0001-0006 | lbp false
KT2 change-set protocol + changeset skill CLAUDE.md:38-62; skills/changeset/SKILL.md:9-26 | redesign | kit-v3-0 | false
KT3 merge authority: advisor agent + standing permission + PR Advisor routine CLAUDE.md:64-73; agents/advisor.md:25-121 | redesign | STATUS:100-102 PROTECT-MAIN, :106 Mergincanli comment risk, :103 | false
KT4 reviewer agents perf/security/netcode | redesign | changed by kit-v3-0 | false
KT5 art-director + render-ab skill | redesign (look brief pending) | true
KT6 verifier + docs-keeper (model sonnet) | redesign | false
KT7 gate skill (checks 1-4,5b-5d) | drop (checks exist for globals); keep smoke+mutation proofs | false
KT8 CI gate.yml | redesign (CI gate v2, R5) | false
KT9 syntax hook | redesign | false
KT10 permissions settings.json:3-95 | redesign | false
KT11 security guidance+patterns+plugin | redesign | false
KT12 legacy path rules engine-core/game/editor/pages/tools | drop (harvest lessons) | stale engine-editor.md:7 | false
KT13 netcode rule | redesign | false
KT14 night shift NIGHT_SHIFT.md + routines | redesign | window/cron mismatch; STATUS:105 | false
KT15 reporting: advisor-report skill + docs/reports + CLAUDE.md:75-84 | redesign(keep shape) | false
KT16 STATUS+ROADMAP dashboard | redesign | false
KT17 ADR template | redesign | false
KT18 FIRST_PROMPTS.md | drop | false
All rows verified; composing answer. DONE
