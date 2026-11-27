# Shamble Frontlines — Roadmap

> **Living plan + condensed history — kept deliberately lean.** The code — `shamble-core.js` → `shamble-game.js` → `shamble-editor.js`, loaded by `index.html` / `editor.html` — is the source of truth; this file is orientation only. Read **`START_HERE.md`** first, then `MODULE_MAP.md`; deep references: `GAME_DESIGN.md` (why), `TECHNICAL.md` (how), `ART_DIRECTION.md` (the locked *Living Comic* look), `UNITS.md` (planned roster).
>
> **Hygiene (please follow):** add **terse** entries (a few lines each) under "Recent work", newest first; keep ~8–12 there and fold the oldest into "Build history" when it overflows. Do **not** paste change-set detail, harness output or verify logs here — those belong in the chat.

---

## The game (30 seconds)
Browser-based **top-down team shooter** in **Three.js r128** (UMD global; **classic multi-script** — no ES modules / build step). A fusion of **Foxhole** (overhead mouse-aim cam), **C&C Renegade** (buy classes at a terminal, power-plant pricing, destroy-the-HQ win), **Red Alert 3** (bright toybox palette, rock-paper-scissors units, 3 factions), and **Gears of War** (weighty gun feel + a mechanic per gun). PvE vs 6 bots today; the deterministic headless SIM core is multiplayer groundwork.

## Design pillars
Readability first (overhead cam) · class identity over loadout freedom · rock-paper-scissors counters · gun feel = weight + a hook · base-assault economy with momentum swings · positive/vibrant RA3 energy · faction identity is *worn*, not just tinted.

## Economy (live values: `ECO` / `RULES_DEFAULT` / `T` — verified 2026-09-25)
- **Pay** (personal credits): kill +25 · refinery trickle 1/s (0.25 if your refinery is dead) · each held silo +1/s · harvester dock +600.
- **Tech** (shared team meter, era T1→T4): tech box held to expiry +10 · harvester dock +5 · kill +1 / headshot +2 · era costs [60, 90, 120].
- **Levers:** a downed power plant halves that team's tech and raises the player's prices (`priceMult`). Silos (3.5 s capture) and tech boxes (7 s channel, 120 s active) give mobile, comeback-friendly income.
- **Shop:** the special weapon (Torque Bow, 500 × `priceMult`), frag grenades, resupply (50). Class primaries are locked; the Officer class costs 1250, the Juggernaut 2500.

## Phases
- **0 — Economy spine + eras** — DONE
- **1 — Tier-gated arsenal + player level** — DONE (data-driven era/minLevel gates; content still placeholder)
- **2 — Harvester / supply convoy** — DONE
- **3 — Tech boxes** — DONE
- **4 — Economy-as-weapon (power plant + silos)** — DONE
- **5 — Unit-type system** — IN PROGRESS: 5.1 first-tier infantry done · 5.2 ground vehicles started (Juggernaut built + drivable) · aircraft pending
- **6 — Multiplayer** — NOT STARTED (SIM core is deterministic groundwork) (legacy plan; v2 replaces it)
- **v2 phase 0 (V2_PHASE0_BRIEF)** — IN PROGRESS: kit-v3-0, R7, R1–R6, the decision drafts (Proposed, signed off as the basis for the spikes) and kit-v3-1 done (PRs #26, #27, #28, #33, #30); the routines set up and tested (TEST-RUN, PR #34, 2026-11-04); Mustafa's design walk B3 recorded (PR #36, 2026-11-04); next: the netcode proof test
- **v2 playable slices, in order (Mustafa and the Advisor, 2026-10-31):** 1 move & shoot → 2 bots and three teams (played locally) → 3 online → 4 the loop

## Build history (condensed, by system)
- **World / economy:** two-pool economy + era state machine; power-plant pricing; 2 capturable silos; rotating tech boxes; per-team AI harvester convoy; HUD + hold-TAB team scoreboard with real kill attribution; persistent rank/prestige (`Prog`).
- **Map:** data-driven ASCII grid "Cabana Republic" on a rectangular island (water / beach / grass), now populated with a symmetric Normandy village (see Recent work).
- **Arsenal / units:** 4 classes — Conscript, Peacekeeper, Imperial Warrior, Officer (tier 2) — each a locked primary + one ability (molotov / riot shield / banzai katana / bayonet charge); R/P/S triangle; shop extras: frag grenade + Torque Bow special; enemies are the same unit types plus the Empire elite.
- **Vehicles:** Juggernaut — the first vehicle: shop-bought at era T2 (2500), pilot-driven walker with an independent turret and triple artillery (130 dmg, 130 m, hits buildings); idles when unpiloted (`buildJuggernaut` / `enterJug` / `tickPilot` / `exitJug`).
- **Character system:** one data-driven builder `buildEntity`; Character Lab (male/female bodies, live faction recolour, wardrobe slots, 8 hairstyles, 5 facial-hair styles, proportion sliders, JSON export/import); material-**role** faction system; gameplay soldiers faction-dressed (`buildSoldier` + `SOLDIER_KIT`); procedural animation (locomotion → carry/ADS aim layer → 2-bone arm IK).
- **Map editor (sandbox):** grid/angle snap, gizmos + resize handles, path decals, marquee + group ops, Tab playtest, dockable World panel, undo/redo, numeric inspector, mirror/array/prefab stamps, analysis overlays, verticality; **dark-token UI overhaul**; **Building Maker** (`BK` kit-of-parts: footprint/roof/skins/dormers + seeded `BK.roll`; one region preset so far: `normandy`).
- **Architecture:** timber house, thatched cottage, brick-and-flint farmhouse, Norman manor (round tower + conical roof); trees (apple/poplar/oak), props (8 kinds), covers (hedge/drystone/fence).
- **Art direction:** "Lush Comic" → relocked as **"Living Comic"** (2026-07-08, ART_DIRECTION §9 steps 1–5: 2-band ramps, differential dark ink, Living Comic env, brightened primaries, Empire elite villain, halftone default-off).
- **Storage (2026-09-25):** `KVStore` adapter (window.storage → localStorage → memory); stores no longer silently in-memory under `file://`; the tutorial no longer re-opens on every deploy.
- **Village / colour / ink passes (2026-09-25):** `DM_Cabana` populated with a mirrored Normandy village, validated against roads, colliders, objectives, spawns and routes (`village.py`); `LinearEncoding` output with option-C lights (sun 0.8, hemi 0.32, amb 0.12) and `ENV.nature.gain` 1.56; contact-line ink via `ensureOutlineNormals` (smoothed + mitred hulls, ground clamp).
- **Doc pass (2026-09-25):** every doc re-verified against the live build; START_HERE and PROJECT_INSTRUCTIONS rewritten for the current workflow; drift fixed across all docs.
- **Structure:** file split (2026-07-12) — single ~4,680-line HTML → core/game/editor + two pages; player menu is Play + Career only.
- **Headless toolkit (2026-09-25):** `preview.js` renders the real build (WebGL1); `battery.js` implements checks 3–4 on an AST (mutation-tested against the `placeCursor` incident); `sim.js` runs a headless 90 s match.
- **Animation overhaul (2026-09-26):** ANIM v2 — per-gun models + sockets (`GUN_KIT`), gun-first IK holds with both hands, planted feet with procedural ankles, hips within a twist budget, four-panel greatcoat skirt, per-gun reloads; player shots fire from `playerFireOrigin`, not the animated muzzle. Confirmed in Brave.
- **Bug & optimization run (2026-09-26):** full audit (`AUDIT_BUGS_PERF.md`); kill-vignette hitch fixed (confirmed in Brave); GC churn −20 %; const-tracking-let deopts removed via a warm-up; Juggernaut in the faction livery (`PLAYER_FACTION`).
- **Agentic environment (2026-10-13/14):** Phase 0 (portable toolkit with lockfile, `tools/gate.mjs` checks 1-4, CI, syntax hook, `pageshot.js --headed`; PARTIAL until Mustafa's Windows run) and CS-1 (repo moved to the root so Claude Code loads CLAUDE.md and `.claude/`; 52 pure renames).
- **Kit v1.1 + editor-prefab-list (2026-10-14):** kit-v1-1 (PR #3, 5858864: `.claude/settings.json` with 19 deny rules + standing-permission paragraph in CLAUDE.md; Mustafa still runs `/permissions` locally); editor-prefab-list (PR #4, c458fe7: `refreshPrefabList`, `refreshMapList`, `edSavePrefab` hoisted out of `bootEditor()`; the three editor ReferenceErrors gone; Mustafa's Brave check passed). Branch protection on `main`: turned on by Mustafa, 2026-10-17.
- **Gate check 5, kit v1.2, bot poplar wedge (2026-10-14):** gate-check-5 (PR #5, 8675367: check 5 in `tools/gate.mjs`, 5a smoke, 5b names, 5c layering, 5d script order; CI jobs gate + smoke; fresh `npm ci` and Node 22.13+ needed; reports/2026-10-14-gate-check-5.md) · kit-v1-2 (PR #6, ef019e0: playwright allow rule narrowed to `install chromium`) · bot-poplar-wedge (PR #7, 159bec4: `navBuild` clearance 0.7 m + half a cell, `navPath` starts from the nearest open cell, 3-call warm-up; poplar pins >5 s 39 -> 0, mean stall 8.82% -> 7.73%; reports/2026-10-14-bot-poplar-wedge.md; its risks are in STATUS).
- **Editor undo ground (2026-10-14, merged 2026-10-15):** editor-undo-ground (PR #8, b5f4ecf: `addBox`, `addPath`, `makeBuilding`, `makeBarracks` and the `buildMap` terminal call `stylizeOne()`; `clearMap` clears `trees[]` and `props[]`, so editor rebuilds no longer lose toon + ink or stack trees / props; draw calls stay 1992 over two rebuilds; Mustafa's Ctrl+Z check in Brave passed; reports/2026-10-14-editor-undo-ground.md).
- **Docs loop v2, team prep fixes, map dedupe (2026-10-15/16):** docs-loop-v2 (PR #9, 67cf55b: `docs/LOOP_V2.md`) · team-prep-fixes (PR #11, 513f2fc: bots hit through the SIM hit world; allocation-light LOS and shot checks via `rayStatics`; reports/2026-10-15-team-prep-fixes.md) · map-dedupe-on-load (PR #10: exact duplicate trees / props dropped on Load; reports/2026-10-14-map-dedupe-on-load.md). Brave checks passed.
- **Three teams core, 3a (2026-10-17):** PR #15, 5ffe679: teams as data, a 12-slot soldier table, team-filtered shots, two-layer damage, bot targeting under a LOS budget, the DM_Tri arena (`?map=tri`); Mustafa's Brave check passed; follow-ups in STATUS.
- **Docs after 3a (2026-10-17):** docs-after-3a (PR #20, df1e6d3; docs only): STATUS and ROADMAP caught up with the 2026-10-17 merges and the Advisor's R19-ORDER / LOS-SIZE / K4-MEASURE verdicts.
- **Kit v1.3 (2026-10-17):** PR #14, 9614ecf: the "Reporting to Mustafa" rule in CLAUDE.md and the advisor-report template.
- **Kit v1.4–v1.6, the night shift (2026-10-17):** kit-v1-4 (PR #17, bd27eeb: CLAUDE.md "Night shift": approved unattended work goes into the Night queue, issue #16) · kit-v1-5 (PR #18, 2303073: `.claude/NIGHT_SHIFT.md`, the routines read it from main) · kit-v1-6 (PR #19, cbd79e5: a cut-off run is picked up; the report comment updated per item). No game code.
- **Kit v2.0–v2.1, the advisor and automated merging (2026-10-17):** kit-v2-0 (PR #21, 04c086c: `.claude/agents/advisor.md`, plan and PR reviews, labels `ready-to-merge` / `needs-mustafa` / `changes-requested`) · kit-v2-1 (PR #22, 211ddcb: the PR Advisor routine merges approved ready PRs; `needs-mustafa` PRs wait for Mustafa's own go in the merging session's chat; the approved full SHA is the merge's expected head; the gate's own files join the rules category). No game code.
- **Kit v2.2 + docs after kit v2.1 (2026-10-17):** kit-v2-2 (PR #23, 48ffb51: Mustafa's go is for the PR's current head, given after the advisor approved it; a later push needs a new review, then his go again) · docs-after-kit-v2-1 (PR #24, e727ce4; docs only). Branch protection on `main`: a pull request with `gate` and `smoke` green (rules in STATUS). No game code.
- **The v2 pivot (2026-10-24):** Mustafa's `docs/VISION_V2.md` (north star and targets) and `docs/V2_PHASE0_BRIEF.md` (phase 0: process upgrades, research R1–R7, decision drafts) added with kit-v3-0: v2 is a game-specific engine (Rust simulation core, TypeScript client on the latest three.js, our own netcode). The legacy game is frozen.
- **Kit v3.0 + docs after kit v2.2 (2026-10-25/27):** kit-v3-0 (PR #26, f2ccc13: v2 phase 0 step 1, CLAUDE.md's v2 rules, Opus with a stated effort everywhere, `.claude/CHECKPOINTS.md`, `docs/REPO_MAP.md`, six new agents, the web allow list; reports/2026-10-24-kit-v3-0.md) · docs-after-kit-v2-2 (PR #25, 4c24d07; docs only: kit-v2-2's and #24's merges and main's branch protection).
- **R7 legacy inventory + docs after R7 (2026-10-27):** PR #27, 07ab240 (docs and one read-only tool): `docs/LEGACY_INVENTORY.md` (247 systems, each marked rewrite in Rust, port to TypeScript, redesign or drop; the marks are working recommendations, R7-MARKS (a)), `docs/LEGACY_TUNING.md` and `docs/legacy-tuning.json` (60 tuning tables), `tools/tuning-dump.js`; reports/2026-10-27-r7-legacy-inventory.md · docs-after-r7 (PR #29, e9f11be; docs only: STATUS and ROADMAP caught up with v2 phase 0 and the Advisor's 2026-10-27 decisions).
- **R1–R6 research (2026-10-27 to 10-31):** PR #28, b6effde, on Mustafa's "merge #28" (docs only): six topic files in `docs/research/` (R1 simulation core … R6 infrastructure), `GLOSSARY.md` and `UNREAD_SOURCES.md`; R1 and R2 verified, R3–R6 spot-checked; reports/2026-10-31-v2-research.md.
- **Docs after v2 research (2026-10-31):** PR #31, 2f5b52e, on Mustafa's "merge #31" (docs only): STATUS, ROADMAP, the
  research folder and VISION_V2 §3 caught up with PR #28's merge and the 2026-10-31 decisions; the decision drafts
  planned in a fresh session (DRAFTS-SESSION (a)).
- **v2 decision drafts (2026-10-31 to 11-03):** PR #33, e4d8b22, by squash (docs only): the six decision drafts
  `docs/adr/0001`–`0006` (Proposed), `docs/ENGINE_OVERVIEW.md`, `docs/SPIKE_PLAN.md`, the register `docs/adr/README.md`,
  GLOSSARY §8 and `docs/LOOK_V2.md`; signed off by Mustafa and the Advisor on 2026-11-03 as the basis for the spikes,
  not yet accepted; reports/2026-11-01-v2-decision-drafts.md.
- **Kit v3.1, the routine guard (2026-10-27 to 11-04):** PR #30, e2b5799, on Mustafa's "merge #30" (rules; no game
  code): `.claude/hooks/` (`routine-guard.mjs`, `guard.mjs` and their tests) make the night shift and the PR Advisor
  safe to run unattended (a limited shell, no web, no push to main, no `.claude/` edits; day sessions unchanged);
  GUARD-FILE (a); reports/2026-10-27-kit-v3-1.md.
- **TEST-RUN part B + docs after kit v3.1 (2026-11-04):** part-b-test (PR #34, 868e165; docs only: the kit-v3-1
  report's corrections): the PR Advisor routine, started by "Run now", reviewed it, labelled it `ready-to-merge` and
  merged it with `expectedHeadSha` a432692…: TEST-RUN B2; Mustafa accepted parts A and B; the routines are set up (the
  PR Advisor on a schedule trigger only) · docs-after-kit-v3-1 (PR #35, 4893ab6, on Mustafa's go after the advisor's
  APPROVE of 76c35bf; rules and docs): CLAUDE.md's routine lines and advisor.md's PR review match the routine setup;
  STATUS, ROADMAP and REPO_MAP record PRs #33, #30 and #34, the setup and TEST-RUN; PORTAL-EMBED as open questions
  OQ-0002-9 and OQ-0006-23; report `docs/reports/2026-11-04-docs-after-kit-v3-1.md`.
- **Mustafa's design walk B3 (2026-11-04):** b3-design-walk (PR #36, 8bcc65e, by merge commit, on Mustafa's go after
  the advisor's APPROVE of a32e4c0; docs only): his 2026-11-04 decisions and ideas recorded word for word, tagged
  `[DECIDED]`, `[IDEA]` (new tag) or `[OPEN]`, in LOOP_V2 (new §13) and UNITS.md §0, with DESIGN_NOTES pointers and
  "bears on" clauses in the ADR register (no ADR edited); report `docs/reports/2026-11-04-b3-design-walk.md`.
- **Kit v3.2, checkpoint rules for every session [2026-11-04, MERGED]** — kit-v3-2 (PR #37, 12d7961, 23:06 UTC, by merge commit, on Mustafa's go after the advisor's APPROVE of 342ff49; A2 on Mustafa's list; rules, no code, symbols +0 −0): `.claude/CHECKPOINTS.md` takes his four anti-waste rules of 2026-11-03 for every session: at most 3 agents at a time, workflow agents included; each agent writes its plan first and saves each block as it goes, a restart continuing after the last complete block and trusting the blocks its notes mark done (E9-1: E9's dry run of the paste block found restarted agents re-checking them); scratch text and small scripts in the notes folder, builds and binaries under `.gate/<task-slug>/`; the background saver checked and restarted at every checkpoint update. CLAUDE.md, NIGHT_SHIFT.md, the `changeset` skill, four agent files and tools/README.md restate them. TOY-BOX and LOOK-LINES (B): CLAUDE.md, art-director.md and graphics-engineer.md point at `docs/LOOK_V2.md`. Mustafa's four answers: "recommendations” in V3." (AGENT-CAP A, SCRATCH-HOME A, SAVER-CHECK A, LOOK-LINES B). Plan revisions 1 and 2, the advisor's APPROVE with 11 conditions. Also takes #36's final-review follow-ups (STATUS, ROADMAP). Report: `docs/reports/2026-11-04-kit-v3-2.md`.
- **Kit v3.3, the guard's follow-ups [2026-11-05, MERGED]** — kit-v3-3 (PR #38, af8393e, 21:25 UTC, by merge commit, on Mustafa's go ("merge #38") after the advisor's APPROVE of 8b7cfb3; CI green on main; A3 on Mustafa's list; rules and kit code, symbols +0 −0; built: WT-NIGHT (a) in ed3ea8c, fix round 2acfbbc, fix pass 2 (c8cbb1a), fix pass 3 (9589bfb), fix pass 4 (0c481e1); plan revision 2 with the advisor's conditions C1-C12 and its E10 ruling): day sessions ask before kit and gate edits (file tools and shell writes) and refuse an agent's; switching the main folder to anything but reviewed main asks; `--output` refused for `git rev-list`, `diff`, `log` and `show`, `rev-list` for routines; read-only Actions methods for pr-advisor on this repository; the saver catches `sk-`, AWS and Slack keys behind `%XX` and other escapes; the rule texts as the docs quote them (f22c4df, 2e82216; step 4c, two security-review fixes, bbcb96b). STATUS's Follow-ups and Mustafa's queued words (93b9a07). E11a: the ask stops a kit-file Write in default, acceptEdits and auto modes. Reviews of 442fbf7: security-auditor FAIL on SA-1 (a relative write after a cd the guard could not follow), the verifier PASS WITH NOTES; skeptics upheld five findings, refuted one. The fix round (2acfbbc; the advisor's ruling: PROCEED): the folder is known only where the line's text fixes it (R-a to R-e), routines can no longer forge origin/main (SA-2), plus F-b, SA-5, SA-6, SA-8; tests 716/716, every new rule's mutant killed, gate PASS. Step 9 (ed3ea8c), WT-NIGHT (a) as Mustafa answered it: the night shift's `git switch` is refused unless the branch's rules, CI and gate files equal main's. Its delta review failed (security-auditor SD-1 and SD-2, the verifier V-2 and V-3: the switch check could be outrun by a fetch on the same line or answered by a look-alike tag, and renamed or non-ASCII kit paths were missed); skeptics upheld every finding; two more automated reviews (findings 9 to 13; 13b refuted). Fix pass 2 (c8cbb1a; the advisor's ruling: PROCEED): a night switch, commit or push alone on its line, exact ref lookups, rename-proof name lists, five day fixes; tests 864/864, every new mutant killed, gate PASS. Pass 3 failed: security-auditor P3-1 to P3-3 (a comment, a here-document delimiter and ANSI-C quoting hid whole commands from the guard's reader on routine paths) and P3-5 (paths named on a night commit), all on main 12d7961 too; the verifier PASS WITH NOTES; skeptics upheld all four. The advisor's ruling: PROCEED inside A3, plus its P3-9 (`>&file`). Mustafa's scope stop (2026-11-05): no new shell parsing; D-ASK instead; one final review; then the kit is frozen until the netcode proof test is under way. Fix pass 3 (9589bfb): R-PLAIN (routines write plain shell or are refused), the night commit allowlist, git never in the background, D-ASK (a day line the guard can't fully read asks, an agent's is refused), the clone fix; tests 1084/1084, C-PLAIN against real bash PASS, no routine line looser, every new mutant killed, gate with smoke PASS. Pass 4, the one final review: security-auditor FAIL on P4-S4 (Critical: a worktree under `.gate/` let a routine carry a rules or CI change into a commit and a push) and P4-S1, P4-S2, P4-S3, P4-S5 (High: wrappers or keywords given by a path, a quoted program after a keyword, a command with no program, xvfb-run's file options), four of them on main 12d7961 too; the verifier PASS WITH NOTES; the automated review of 9589bfb, its finding 14 (Medium, day). The advisor's ruling: PROCEED inside A3, plus its own A4-1 (High: a failed cd followed by `;`); everything below High becomes a follow-up (FU-P4a to FU-P4c). Fix pass 4 (0c481e1): R-PROG (every routine command starts with git, gh, npm, node, python, python3 or date by name) and the worktree path rule; tests 1188/1188, no routine line looser (1348 role decisions: 0 looser), every new mutant killed, gate with smoke PASS. The verifier's check of fix pass 4: PASS WITH NOTES (its Low is follow-up FU-P4d). Report: `docs/reports/2026-11-05-kit-v3-3.md` (its corrections after the merge: B.1). The advisor's APPROVE of the final head 8b7cfb3 (PR comment 6669362242); merged on his go (`needs-mustafa`; no Brave check). Limits: STATUS, Open bugs and risks.
- **EDITOR_V2's plan view and Mustafa's editor decisions [2026-11-06, MERGED]** — b1-plan-view (PR #39, ee736ec, 17:09 UTC, by merge commit, on Mustafa's go ("merge #39") after the advisor's APPROVE of 4f5976b; CI green on main; B.1 on Mustafa's list; docs only, symbols +0 −0; built 6ddbdd9, fix pass 1d79b57; plan revision 1 with the advisor's APPROVE and 14 conditions): his "Plan view: draw to build" (2026-11-03) and the eight lines of his B.1 addition (2026-11-05) go word for word into a dated block in EDITOR_V2 §3, "Decided 2026-11-05", in his order, tagged `[DECIDED]` with his marks kept ("Principle", "lower priority"; no new tag); the old lines stay as they are, with pointers in brackets; a "bears on it" clause in the register's OQ-0005-CD4 row (no ADR draft edited); the A3 report's corrections after the merge. E1 (his words, each piece exactly once), E2 (old bullets unchanged but one pointer) and E3 (tag counts) PASS; gate `--no-smoke` PASS. Follow-ups for the drafts' next revision: his "1 unit = 1 metre" (ADR 0004, 0005, 0003) and "keep editing while the match runs" against OQ-0005-CD4. Reviewed: the verifier and game-designer PASS WITH NOTES, nothing Critical or High; the fix pass 1d79b57. Report: `docs/reports/2026-11-06-b1-plan-view.md` (its correction after the merge: B.2). The advisor's APPROVE of the final head 4f5976b (PR comment 6682184695), with two Low notes: STATUS's "a draft" (taken by B.2) and one point of plan condition 8 left out of the report's risk 3 (it stands in the plan); merged on his go (`needs-mustafa`; no Brave check).
- **VISION_V2's community-servers note and B3's hand-over [2026-11-06, MERGED]** — b2-community-servers (PR #42, 4c6db5b, 23:57 UTC, by merge commit, on Mustafa's go ("merge #42") after the advisor's APPROVE WITH NOTES of e223ecc; CI green on main; B.2 on Mustafa's list; docs only, symbols +0 −0; built 67978c2, fix pass 8d1540f; plan revision 1 with the advisor's APPROVE and 9 conditions): his "Community servers (later)" note of 2026-11-03 goes word for word to the end of VISION_V2 §5, tagged `[LATER]`: the why, the trust rule (`[DECIDED]`: progression counts on official servers only), the precedents, and a pointer to its open question in the drafts (ADR 0006, OQ-0006-21); B3's hand-over: pillar 5's `[OPEN]` gives way to his Factions line of 2026-11-04 (`[DECIDED]` at the level of principle; which units are shared or per faction still waits on his unit list), with pointers on §5's account-progression line and §7's faction question; a dated header line; one clause in the register's OQ-0006-21 row (no ADR draft edited); the B.1 report's correction after the merge; STATUS and ROADMAP after PRs #41 and #39. E1-E4 PASS (his words byte for byte, nothing else changed in VISION_V2, tags `[DECIDED]` +2, `[OPEN]` −1, `[LATER]` +1, section citations resolve); gate `--no-smoke` PASS; CI green on 67978c2. Reviewed once: the verifier, game-designer and infra-engineer, each PASS WITH NOTES (one Medium and nine Low findings, nothing Critical or High), then one fix pass. Follow-ups (STATUS, "From B.2"): at the drafts' next revision, ADR 0006's OQ-0006-21 entry may point at the note and ADR 0005's five VISION_V2 line ranges move; whether rented official private servers count is not decided; "the war around the matches" for a later VISION_V2 pass; a stale VISION_V2 line citation in R1's dated research record, left unedited. Report: `docs/reports/2026-11-06-b2-community-servers.md` (its corrections after the merge: vfx-xp-notes). The advisor's APPROVE WITH NOTES of the final head e223ecc (PR comment 6698888516), with four Low notes, taken by vfx-xp-notes; merged on his go (`needs-mustafa`; no Brave check). One pull_request CI run stalled installing Chromium's system libraries and passed on a re-run (STATUS, Open bugs and risks).

**The VFX editor and private-match XP [2026-11-07, MERGED]** — vfx-xp-notes (PR #43, 69ea88e, 11:28 UTC, by merge commit, on Mustafa's go ("merge #43") after the advisor's APPROVE WITH NOTES of 4e03bb5; CI green on main; Mustafa's two items of 2026-11-06, both "For the next docs pass", next by his VFX-ORDER (a); docs only, symbols +0 −0; plan revision 1 with the advisor's APPROVE and 10 conditions): his VFX editor item goes word for word into EDITOR_V2 as §7, a future tool after Editor v2's map features (the tool `[LATER]`, each of his ten principles `[DECIDED]`), with a dated header line; his private-match XP rule goes into VISION_V2's community-servers note after the trust rule (`[DECIDED]`: 25 % of normal XP with bots in the match, 100 % humans only; the Advisor's two suggestions `[OPEN]`, as he marked them); "bears on it" clauses in five register rows (OQ-0003-15, OQ-0003-16, OQ-0005-CD7, OQ-0006-21, R2-ROOM-OQ7; no ADR draft or LOOK_V2 edited); STATUS and ROADMAP after PR #42; the B.2 report's corrections after the merge. E1-E3 PASS (his words byte for byte; EDITOR_V2 `[DECIDED]` +10 and `[LATER]` +1, VISION_V2 `[DECIDED]` +1 and `[OPEN]` +1); gate `--no-smoke` PASS. Follow-ups (STATUS, "From vfx-xp-notes"): which "XP" his rule means (XP-TERM: the docs' other XPs listed, none mapped); his "Comic flavour" against LOOK_V2 §7's "used sparingly" (LOOK), for the look brief's owners; the `[LATER]` tool gives the drafts no engine feature now (decision map J3); his VFX style guide and its reference study, a follow-up only, not queued. Reviewed once: the verifier, game-designer and art-director, each PASS WITH NOTES (two Medium, the rest Low; nothing Critical or High), then one fix pass (a fifth register clause, OQ-0003-15; follow-ups). Report: `docs/reports/2026-11-07-vfx-xp-notes.md`. His XP-TERM (a) and LOOK (a) of 2026-11-07 and the advisor's four Low notes went to kit-v3-4.

**Kit v3.4, the review policy [2026-11-07 to 11-09, MERGED]** — kit-v3-4 (PR #44, 44446a5, 2026-11-09 18:21 UTC, by
merge commit, on Mustafa's go ("merge #44") after the advisor's APPROVE WITH NOTES of 7c36d5a; CI green on main after
one re-run; A4 on Mustafa's list and A4's addition, both 2026-11-04; rules and docs, symbols +0 −0; plan revision 1 with
the advisor's APPROVE WITH CONDITIONS (13); his TIER0-SCOPE (a), REVERT-EXCEPTION (a) and DISK-WHEN (a) of 2026-11-08):
CLAUDE.md's protocol takes risk tiers 0-3 (a PR takes the highest tier any changed file or effect reaches; a tier lifts
no `needs-mustafa` trigger; docs that record his words or decisions keep his go), a review budget (at most two rounds, a
third only for an open Critical or High; the gate and the verifier never cut), one severity scale and a rollback rule (a
day session reverts the merge that broke main first, no plan step, merged by today's classes); advisor.md takes the
tiers mapped onto `ready` and `needs-mustafa`, per-tier checklists and the revert steps (21d2b56);
`.claude/CHECKPOINTS.md` §10, disk hygiene (b85b2cf). The docs (d8147cd): his foundation notes in SPIKE_PLAN and ADR
0006, his XP-TERM (a) and LOOK (a), #43's Low notes. The five kit edits in his kit sitting of 2026-11-08; the kit tests
pass with main's counts; gate PASS. The review round (infra-engineer, the verifier, security-auditor; two Highs fixed)
and one fix pass; the verifier's H1 (the revert check ignored whitespace) fixed at 3d39527; follow-ups F1-F5 in STATUS;
the report `docs/reports/2026-11-08-kit-v3-4.md`. His GUARD-RENAME (c): the guard fix (F1, agent memory included) is the
next change-set, kit-v3-5.

**Kit v3.5, the guard's rename gap [2026-11-09 to 11-10, MERGED]** — kit-v3-5 (F1 from kit-v3-4, agent
memory included: Mustafa's GUARD-RENAME (c); rules and kit code, symbols +0 −0; plan revisions 1-3 with the advisor's
REVISE, then APPROVE with D1-D6, then APPROVE with E1-E5; his ORDER (b), SA5-RIDE (a) and allow-list direction of
2026-11-10): the PR Advisor merges only from an allow-list (`mergeAllowed`: plain names in `docs/`, `tools/`,
`README.md` and `FIRST_PROMPTS.md`, no rules, gate or decision-doc name in any letter case), read on both names of a
renamed file with whole lists; `CLAUDE.local.md` and `AGENTS.md` protected anywhere; `tools/npm-shrinkwrap.json` a gate
file (9bacf47). Tests red on main's guard (47 of 1086 fail), then green (1086/1086, settings 127, checkpoint 27); gate
PASS. Docs: #44 merged, his queue word for word (KIT-FREEZE-2, STATUS-DIET, LANE-RULES, the unit walk, the Slice 1
note), the known limits, the kit-v3-4 report's correction. The review round (three PASS WITH NOTES, nothing Critical or
High) and its fix pass (08423ce: routine-guard 1107/1107); the verifier's re-check (PASS WITH NOTES); the report
`docs/reports/2026-11-10-kit-v3-5.md`. Merged: PR #46, 2d22b69, 2026-11-10 16:05 UTC, by merge commit, on Mustafa's
go after the advisor's APPROVE WITH NOTES of 0573db3; the PR Advisor routine re-enabled at 16:07 UTC.

**STATUS-DIET with LANE-RULES [2026-11-10 to 11-11, MERGED]** — status-diet (Mustafa's queue of 2026-11-10, items 2
and 3, with ACCOUNTS and his Models addition): STATUS (1233 lines) split by a line-range script into STATUS (109 lines:
the phase, in flight, next, waiting on Mustafa, the routines, pointers), `docs/DECISIONS.md` (his decisions word for
word, newest first, and the queue of his words) and `docs/STATUS_ARCHIVE.md` (the rest; Follow-ups and Open bugs and
risks stay live); every moved line byte for byte, no decision id lost. CLAUDE.md: the read list, "Lanes and accounts"
and the Fable 5.2 advisor-tool exception, his words quoted; docs-keeper.md follows the split; #46 recorded as merged.
Report `docs/reports/2026-11-10-status-diet.md`. Merged: PR #49, bb2d817, 2026-11-11 09:03 UTC, by merge commit, on
Mustafa's go after the advisor's APPROVE of 2e6e5d2; CI green on main (run 42991918886).

**The foundation, 2.1a: the legacy gate reads both layouts [2026-11-10 to 11-11, MERGED]** —
foundation-restructure, the first of step 2.1's two pull requests (Mustafa's decision of 2026-11-10: the second
account plans and builds steps 2.1 and 2.2 now; his SPLIT (a)); gate files and tools only, no game file, no file
moved, symbols +0 −0 (dd12aeb, 3f29cb8). `tools/smoke.mjs` finds the build folder (`legacy/` when it holds a
`shamble-*.js` file, else the root; engine files in both is a setup error, exit 2); `tools/gate.mjs` reads the head there
and each base from `legacy/`, the root or `shamble-frontlines-repo/`, and (3f29cb8, the security review's F1, on his yes
of 2026-11-11) when the folder differs between base and head, the base's folder must keep no page or engine file in any
letter case (exit 2); `tools/village.py` defaults to `legacy/` when present; `gate-globals.json` is called a gate file.
Proof: the mutation proof, 31 of 31 PASS on the edited gate and again on the fixed gate, identical in every column (every
planted fault fails the same checks on the old gate, the edited gate at the root and the edited gate on `legacy/`; every
control passes on all three); the F1 rows red on the unfixed gate and green on the fixed one; the decoy stopped; both
move directions and their bases recorded raw; gate PASS. PR #50 replaces #47 (his BRANCH-47 (a)); its docs pass records
#49's merge and his words of 2026-11-10 and 2026-11-11 (`docs/DECISIONS.md`). Round 2: three APPROVE; the verifier PASS
WITH NOTES; report `docs/reports/2026-11-11-foundation-2-1a.md`. Merged:
PR #50, 76b4f7a, 2026-11-11 18:38 UTC, by merge commit, on Mustafa's go after the advisor's APPROVE of f6becf5; CI
green on main (run 43079707687).

**The foundation, 2.1b: the move [2026-11-11 to 11-12, MERGED]** — foundation-restructure, the second of step 2.1's two
pull requests, after 2.1a (PR #50, merged as 76b4f7a on 2026-11-11, by merge commit, on Mustafa's go after the
advisor's APPROVE of f6becf5). The five legacy files move into `legacy/` with `git mv`, bytes unchanged, in a commit of
their own (c090843); the commands in text name the `legacy` folder (`tools/README.md` with the build-folder rule, the
tool headers, `tools/village.py`, the root README: 1e6f84e); the four kit lines on his yes, KIT-2.1B (a) (CLAUDE.md
:19, :43, :44 and `.claude/agents/verifier.md:28`: b6b3b81); the docs (his words of 2026-11-11 in `docs/DECISIONS.md`,
his SCOPE (a) in ADR 0002 and the register, OQ-0002-2 answered in part, the maps, STATUS). Evidence E1-E6 and E8 PASS;
CI's gate and smoke green on c090843, 1e6f84e and b6b3b81. Review round 1 and the verifier passed; the report:
`docs/reports/2026-11-11-foundation-2-1b.md`. Merged: PR #51, 2aa5e9c, 2026-11-12 13:22 UTC, by merge commit, on
Mustafa's go after the advisor's APPROVE of 5d50bf2; CI green on main (run 43226692719).

**The foundation, 2.2: the toolchain [2026-11-10 to 11-13, MERGED]** — foundation-toolchain, ADR 0002's
migration plan step 2.2 (Mustafa's decision of 2026-11-10: the second account plans and builds steps 2.1 and 2.2 now).
The pins: Rust 1.99.1 (`rust-toolchain.toml`); a virtual Cargo workspace and its lock with six empty crates in `crates/`
(unsafe code denied, overflow checks in every profile for every workspace crate); npm workspaces with exact versions and
one lock, five empty packages in `packages/` (TypeScript 6.0.3, Vitest 5.1.0, @playwright/test 1.65.0, Node 22). The CI
job `toolchain` (`.github/workflows/toolchain.yml`; `gate.yml` unchanged) installs the pinned toolchain, checks rustc
against the pin, then builds and tests with the network cut off (a TCP probe must fail inside); it is green on the
build's heads, and eight planted failures each went red at the expected step and were reverted. On his yes: the four kit
lines (the hard constraints bind `legacy/` only; his 2026-11-10 decision is the exception in the three lines that forbid
v2 code) and the CI job's release-test line; main merged in (c901187); the overflow tests expect their panic's message.
Review round 1 (five reviewers, 17 findings, each checked by a skeptic): the overflow setting widened to every workspace
crate on the advisor agent's ruling (RR-1, High; his go asked, then given with the merge, OVERFLOW-ALL). Review round 2
(four reviewers, nothing Critical or High): on the advisor agent's ruling D1-R2, `"*"` kept and the panic side handed to
the drafts' owners. The Fable advisor's whole read before ready changed no file. The verifier passed (bd44a68). Merged:
PR #48, 4d21c1a, 2026-11-13 16:20 UTC, by merge commit, on Mustafa's go after the advisor agent's APPROVE of 9b20b67; CI
green on main (gate run 43461710700, toolchain run 43461710673).

**The foundation, 2.3: the first docs pass [2026-11-13 to 11-14, MERGED]** — foundation-2-3-docs (PR #52, 4460ef5,
2026-11-14 09:06:30 UTC by `merged_at`, by merge commit, on Mustafa's go of 09:02:29 UTC, quoted whole in its message,
after the advisor agent's APPROVE of bfd719b; CI green on main), by
Mustafa's words of 2026-11-13 (docs only, no kit, gate or CI file): his go for #48 and his decision on step 2.3 go into
`docs/DECISIONS.md` word for word, by script; dated pointers of 2026-11-13 in SPIKE_PLAN, ADR 0002, DECISIONS
(SLICE-ORDER condition 3) and ROADMAP, his decision quoted in STATUS; the 2.2 report's fixes M1, L1, L2 (worded as the
CI logs read), L3 and L5, the archive's lines on the same claims, on #48's merge and on his go for it, and two pointing
clauses; STATUS, ROADMAP and the archive made true for main 4d21c1a. Report
`docs/reports/2026-11-13-foundation-2-3-docs.md`. Step 2.3's plan followed on its branch and waits for his go.

**Kit: the saver in stints while a night run waits on agents [2026-11-16 to 11-17, MERGED]** — night-shift-stints (PR
#54, 82c605e, 2026-11-17 07:03:03 UTC by `merged_at`, by merge commit, on his go after the advisor's and the
security-auditor's APPROVE WITH FINDINGS and the PR Advisor routine's APPROVE of 20f9d59; CI green on main): three lines
at the end of `.claude/NIGHT_SHIFT.md` step 6, by his NS-STATUS (a) and his yes to the exact lines (NS-LINES (a)), the
exception to KIT-FREEZE-2 for that one change; seven Lows deferred in writing (the archive's "Follow-ups"), two of them
his NS-TRAILER and NS-FIRST, his (b): the kit's next change after the freeze.

**Main's docs after #52, #53 and #54 [2026-11-17, MERGED]** — docs-after-54, by Mustafa's DOCS-54 (a) of
2026-11-17 (docs only, no kit, gate or CI file; from the netcode proof test's session, its plan approved by the advisor
agent with conditions C1-C9): his words in that session since #52 go into `docs/DECISIONS.md` by script, copied from
its transcript and checked byte for byte against its history.md, only his typed text inside the quotes, the netcode
test's own words left with PR #40 and each cut named (the foundation chat's own words are that lane's to record); his go
for #52 from merge commit 4460ef5's message; two pointers in "Queued"; STATUS, ROADMAP, the archive and REPO_MAP made
true for main 82c605e. One review round (the verifier PASS; the advisor agent APPROVE WITH FINDINGS, Lows only) and one
fix pass, then the advisor agent's APPROVE WITH FINDINGS of aea6f5b and the verifier's PASS. Merged: PR #55, c94c81b,
2026-11-17 17:09:33 UTC by `merged_at`, by merge commit, on his go.

**Change-set 0 of the build plan [2026-11-17, MERGED]** — cs0-model-rules, on his go of 2026-11-17 (docs only):
his five messages of 2026-11-14 and 2026-11-17 into `docs/DECISIONS.md` by script; the plan and its appendix carried
as reports; the model-file rules and the soldier's joint list, on paper (PM-0005-47 to -56); his answers in the drafts
(open questions 98 to 96). Report `docs/reports/2026-11-17-cs0-model-rules.md`. Two review rounds (the verifier: PASS).
Merged: PR #56, d7dad97, 2026-11-17 23:00:12 UTC, by merge commit, on his go after the advisor agent's APPROVE of
cd61b28; CI green on main (gate run 856 with smoke, toolchain run 155). With the merge he accepted that PM-0005-54
stands as written (a fail with today's rifle prototype, not waived) and the checkpoint commits in main's history.

**Part 15 of the build plan, the second lane, to main [2026-11-17 to 11-18, MERGED]** — part-15-second-lane, by
his go of 2026-11-17 for #56 (docs only): Part 15 as he read it, with his six answers on top (all (a), with additions),
in `docs/reports/2026-11-17-build-plan-slice-1-part-15.md`; his two messages of 2026-11-17 in `docs/DECISIONS.md` by
script; dated pointers in ADR 0002 (a seventh package, VFX-ROUTE) and in the build plan report; the rules' line on gun
files (GUN-FILES); STATUS, ROADMAP, the archive and REPO_MAP after #56's merge. The verifier: PASS. Report
`docs/reports/2026-11-17-part-15-second-lane.md`. Merged: PR #57, 22c6afa, 2026-11-18 11:09:31 UTC, by merge commit,
on his go after the advisor agent's APPROVE of f26ac5b; CI green on main (gate run 866 with smoke, toolchain run 165).
With the merge he answered OLD-LINES, who may fold old lines in the shared record files (`docs/DECISIONS.md`,
2026-11-18, 11:08:25 UTC).

**Change-set 1 of the build plan, Types [2026-11-18, MERGED]** — cs1-types (foundation step
2.4), by lane 1, on his words of 2026-11-18 with his go on #57: the first eight message types in `crates/protocol`
(plain types, no codec), the output block's layout table and its hash in `crates/client-endpoint`, their TypeScript
generated into `packages/types` (ts-rs 12.0.1 as a test-only dependency, and our own small generator), the page's
layout-hash check. Two review rounds; report `docs/reports/2026-11-18-cs1-types.md`, which tells two misses first (CI
built the new dependency before its security check; an evidence record withdrawn and redone). The verifier: PASS (15:03
UTC); the advisor agent: APPROVE (its word on the final head is in #58's description). Merged: PR #58, 9b4b518,
2026-11-18 15:39:28 UTC, by merge commit, on his go after that APPROVE; CI green on main (gate run 884 with smoke,
toolchain run 182). With the merge he accepted the two misses as written and answered its five questions: the kit
hold and the two CI edits (PR #60), the "Lane requests" issue (#59), no watch (`docs/DECISIONS.md`, 15:38:13 UTC).

**The kit hold and two CI edits [2026-11-18, MERGED]** — kit-hold-ci, by lane 1, on his words of
2026-11-18 with his go on #58 (risk tier 2, kit and CI): the checkpoint script holds a push that changes a manifest or
a lockfile until its security check is recorded (his answer DEP-HOLD (a): `.claude/scripts/checkpoint.mjs`, six new
tests, one rule line in `.claude/CHECKPOINTS.md` §3), written on his yes to its exact text, once with one change of
his and once more for the fix of a hole in the changed lines; the `toolchain` job runs with no cache, writes the
generated types again and compares them, and type-checks `packages/types` (his CI-CUT (a), part 1, and CI-TYPES (a)),
shown on real runs; 293 added lines, accepted by his word. One review round, nothing Critical or High; report
`docs/reports/2026-11-18-kit-hold-ci.md`. Merged: PR #60, b763ba3, 2026-11-18 20:30:46 UTC, by merge commit, on his go
after the advisor agent's APPROVE of e5ca735; CI green on main (gate run 900 with smoke, toolchain run 198). With the
merge he accepted the four things that went wrong and the five Mediums as follow-ups, and answered WRITE-MODE (a) and
REC-OLD (a), with one limit (`docs/DECISIONS.md`, 2026-11-18, 20:20:09 UTC).

**Change-set 2 of the build plan, First page [2026-11-18 to 11-19, MERGED]** — cs2-first-page (foundation
step 2.5, first half), by lane 1: the Rust simulation (a tick counter and the match seed), built as WebAssembly with
wasm-bindgen 0.2.130, runs in one module Web Worker at 30 ticks a second and hands the page one block per tick; the
page makes no Wasm call, draws an empty lit ground with a 1 m grid on three.js 0.187.0's WebGPU renderer (WebGL 2 as
the fallback), names its backend and has a development readout; a small server for a built page on this computer
only; the build checks the wasm-bindgen tool's file before every run; browser tests in `packages/e2e` (not in CI
yet). Two review rounds, no Critical or High open; report `docs/reports/2026-11-19-cs2-first-page.md`. His browser
check in Brave and Firefox: errors 0, WebGPU started in both (`docs/reports/2026-11-19-cs2-browser-check.md`). Merged:
PR #61, f5030a3, 2026-11-19 15:10:54 UTC, by merge commit, on his go of 14:28:25 UTC for head 4cf30ce; CI green on
main (gate run 934 with smoke, toolchain run 232).

**CI builds the WebAssembly and runs the tests that need it [2026-11-19 to 11-24, MERGED]** — ci-wasm
(risk tier 2; planned by the first builder chat, carried by a thread of his project, built by the chat he calls
Builder V3): before the network cut the `toolchain` job fetches the wasm-bindgen tool's archive by his TOOL-SOURCE
values, runs GitHub's own check on it and installs its one program root-owned; inside the cut it builds the page
twice, then runs the boundary tests, five type checks and the 12 browser tests in the runner image's Chrome (CI does
not pin the test counts). Two CI writes on his yes (the first stopped at its install check on `/opt`), three planted
faults each red at its own step; with it his browser check of #61 and his words of 2026-11-19 and 2026-11-23. Two
review rounds, no Critical or High; report `docs/reports/2026-11-19-ci-wasm.md`. Merged: PR #64, 1ec6f6c, 2026-11-24
10:15:31 UTC, by merge commit, by his main Advisor on his tap after his go for head 8aadfb0; CI green on main (gate
run 45373057216 with smoke, toolchain run 45373056953). #63 and #62 closed with it.

**Change-set 3a of the build plan, Rules and boundaries [2026-11-24, MERGED]** — cs3a-rules-boundaries (the first
half of change-set 3, his SIZE-3 (b)), by lane 1, built by the chat he calls Builder V3: the simulation's rules under
tests. A scan of `crates/sim/src` refuses a float method outside ADR 0004 part 7's list, a hash map, a clock and
per-target code; our own ChaCha8 (his RNG-SOURCE (c)) matches the 8 published vectors and a 160,384-byte stream file;
38 float operations on 10,048 inputs are exact natively and in WebAssembly, but for 16 `mul_add` results below the
smallest normal float in WebAssembly, pinned as a toolchain defect, with `mul_add` refused in `sim` (his MUL-ADD (a));
the panic hook's text reaches the page; a test-only WebAssembly module, built outside every package; the build refuses
a runtime crate without its overflow override and a feature set that differs between builds, and the runner's Wasm is
the container's byte for byte. A player sees nothing new beyond two lines of the development readout. 3,562 added
lines (stop line 4,100), 96 planted defects caught; CI green on 8619d539 and on the corrected head 8d277cf2; three
review rounds, the third on one High, now closed; the verifier PASS and the advisor agent APPROVE on 8d277cf2; his
browser check: errors 0 in Brave and Firefox (`docs/reports/2026-11-24-cs3a-browser-check.md`); report
`docs/reports/2026-11-24-cs3a-rules-boundaries.md`. Merged: PR #65, 28254a7, 2026-11-24 22:40:39 UTC, by merge
commit, by his main Advisor on his tap of 22:40:00 UTC (his go: `docs/DECISIONS.md`, 22:44:50 UTC), with the expected
head 1624f084 (the last commit, after the advisor agent's read); gate, smoke and toolchain green on main (read 22:45
UTC).

- **Kit: KEPT-LOGS (2026-11-15):** kept-logs-rule (PR #53, 53272a1, on his KL-MERGE (a)): `.claude/CHECKPOINTS.md` §11 and `.claude/scripts/kept-logs.mjs`: a day session may pack and remove a closed run set's raw logs and the one old build cache, nothing else; SA-KL-1 to -4 and KLT-1 to -8 left open by his word.
- **UI theme step 1 (2026-10-02):** menu = dossier, pause = clipboard, Field manual; `wr-*` CSS in both pages; awaiting Brave. (Audit phase 2, 2026-09-26, is the "Bug & optimization run" above.)
- **Performance / hygiene:** pooled FX, GPU resources disposed on map rebuild, player-following shadow frustum; kill vignette captured on the GPU (`vigShot` corner render + `drawImage`); hot paths use `hyp2`/`hyp3` and change-only DOM writes (`_sd`/`_so`/`_st`); ~4.9 MB/s residual GC churn (AUDIT_BUGS_PERF CS11; measured — `AUDIT_BUGS_PERF.md`); an early kill-vignette attempt (readbacks queued via `_vigQueue`/`vigStep()`) did not remove the hitch and was superseded by the GPU capture.

## Current state (verified against the build, 2026-09-25)
- **Look:** Living Comic with **colour-true output** (authored hex = on-screen), option-C lighting on buildings/characters, a **nature gain** keeping grass/foliage/hedges/sea lush, and **contact ink** (every object inked where it meets the ground). Confirmed by Mustafa in Brave.
- **Map:** Cabana Republic carries the village (11 houses, 70 trees, 36 props, 10 hedges/fences); the centre stays open as the main combat space.
- **Persistence:** `KVStore` — rank, tutorial flag, maps, prefabs survive reloads under `file://`. Confirmed in Brave.
- **Tooling:** headless previewer + battery + match simulator + `gate.mjs`, all in `tools/`, with GitHub Actions CI (see START_HERE §5, `tools/README.md`).
- **Animation:** ANIM v2 — weapon-first IK holds, planted feet with ankles, per-gun reloads (see Recent work). Katana + shield moved onto the IK system (awaiting Brave check).
- **Performance:** kill-vignette hitch fixed (GPU capture); GC churn ~4.9 MB/s after the audit (AUDIT_BUGS_PERF CS11); details in `AUDIT_BUGS_PERF.md`.

## Next / open
- **v2, next:** the netcode proof test, its own change-set branched from main, plan first (PR #40); the foundation: 2.1a, 2.1b and 2.2 merged (PRs #50, #51, #48), step 2.3's first docs pass merged (PR #52) and its plan waits for his go; wave 1 runs one heavy spike at a time, in `docs/SPIKE_PLAN.md`'s order (the netcode proof test, the sim core spike and then the 60 Hz run, the boundary spike, the look proof test; the preview hosting check alongside any; the netcode proof test before the foundation; pointer, 2026-11-11: for foundation steps 2.1 and 2.2, overtaken by Mustafa's decision of 2026-11-10, `docs/DECISIONS.md`; pointer, 2026-11-13: for foundation step 2.3 only, overtaken in part by his decision of 2026-11-13, `docs/DECISIONS.md`: "Step 2.3 may be planned now. The parts of step 2.3 that need no proof test's result may be built before the proof tests report, after my go on the plan. Every gate check that needs a result joins later, when that result is in, with its pass marks unchanged. Steps 2.4 to 2.6 still wait for their results."); then phase 1 and the slices in the new order; the security baseline is researched before the first online playtest, in its own fresh session. Mustafa's change-set queue (kit-v3-2 (A2), merged as PR #37; the guard's follow-ups (A3), merged as PR #38; the EDITOR_V2 plan-view note with his editor decisions of 2026-11-05 (B.1), merged as PR #39; the VISION_V2 community-servers note, plus B3's hand-over: VISION_V2's pillar 5 `[OPEN]` (which quoted LOOP_V2's old "infantry shared, tanks per faction"), account progression and faction asymmetry, against LOOP_V2 §5 and §8 of 2026-11-04 (B.2), merged as PR #42; the VFX editor note in EDITOR_V2, with his private-match XP rule in VISION_V2 (his two items of 2026-11-06; his VFX-ORDER (a): next after B.2, its own docs change-set), merged as PR #43; the review policy (A4), merged as PR #44, after the kit freeze lifted (Mustafa's UNDER-WAY (a) of 2026-11-06: the netcode plan's approval plus its first build step committed, both done by the netcode session's record in its PR #40, U0, f6886bf); after A4 the queue holds no item of his still to carry out): STATUS, Phase.
- **Pointer, 2026-11-17 (change-set 0; not his words):** for foundation steps 2.4 and 2.5 and Slice 1, his decision of
  2026-11-14 changes "wave 1 runs one heavy spike at a time" in the line above; they follow the build plan's order
  (`docs/reports/2026-11-17-build-plan-slice-1.md`). His go of 2026-11-17: "The 2.3 chat stays stopped for now: I pass
  it nothing until its plan has been checked." Both: `docs/DECISIONS.md`.
- **Pointer, 2026-11-24 (change-set 3a's docs pass; not his words):** of the build plan, change-sets 0, 1 and 2 are
  merged (PRs #56, #58, #61), CI builds the WebAssembly and runs the tests that need it (PR #64), and change-set 3a is
  in flight; next, in change-set 3's approved plan's order, with Firefox joining by his FIREFOX-TESTS (b): the small CI
  pull request, then 3b (`docs/STATUS.md`, "Next").
- **Pointer, 2026-11-25 (change-set 3b's docs pass; not his words):** change-set 3a is merged (PR #65); 3b is in
  review, the small CI pull request is an Opus 5.5 chat's beside it; then change-set 3c (the link simulator and the
  first recording, moved out of 3b by the size ladder), then change-set 4. Slice 3 picks how commands are timed, no
  longer the netcode proof test (his decision of 2026-11-24, `docs/DECISIONS.md`) (`docs/STATUS.md`, "Next").
- **Pointer, 2026-11-25 (change-set 4a's docs pass; not his words):** change-set 3b is merged (PR #68) and the small CI
  pull request too (PR #71). There is no change-set 3c: his ORDER-3C (c) of 2026-11-25, 16:59:52 UTC
  (`docs/DECISIONS.md`) splits it; the recording is change-set 4a (PR #70, in flight), and the link simulator follows
  Walk as its own small change-set, merged before change-set 7 starts. Walk is four change-sets by his WALK-CUT (a):
  4a the recording, 4b content and the world, 4c the first soldier (he plays), 4d sights (`docs/STATUS.md`, "Next").
- **Pointer, 2026-11-26 (change-set 4b-1's docs pass; not his words):** change-set 4a is merged (PR #70); 4b is two
  pull requests by his SIZE-4B (b) (`docs/DECISIONS.md`, 2026-11-26, 14:09:13 UTC): 4b-1, Content, is reviewed; next
  is 4b-2 (the collision library and the static collision world, the built-file check, his Export's values into the
  content and the simulation, the page drawing the scene from the map), after a short re-read by the advisor agent;
  then 4c, 4d and the link simulator (`docs/STATUS.md`, "Next").
- **Superseded by v2** (paused in issue #16; where things stand: `docs/STATUS.md` and `docs/V2_PHASE0_BRIEF.md`). **Queue (Advisor, 2026-10-15; updated 2026-10-16 after 3a):** greybox phase: three-teams (0. team-prep-fixes, merged → 3a three-teams-core, merged 2026-10-17 → **N1** (next; editor Play never rebuilds the nav grid; a one-line change-set) → **R19** (navBuild building-cell fix; right after N1, before 3b; Night queue item 2, NEEDS PLAN) → 3b three-teams-hud → 3c spawn-and-buy; editor v2 comes after the playable greybox loop, per the Advisor's queue) → **"ramps as slopes in SIM"** (its own change-set, before octopon-3-greybox; lets `_botFireWorld` go) → **NC-7 blast reach by line of sight** (to the target's nearest surface, ignoring the target's own box; before the octopon-3 playtests) → octopon-3-greybox → harvest-economy → bot-replan-storm → uplinks → win-and-scoring → scale test on the Octopon-3 greybox (N = 12, 24, 36, 48). Paused until the greybox loop is playable: tutorial-field-manual, ui-step-2, ui-step-3, build-spike, ADR run. house-textures-seeded: low priority, unslotted.
- **From the 3a report** (approved unattended work is listed in the [Night queue, issue #16](https://github.com/Mergincanli/Shamble-Frontlines/issues/16)): the `navBuild` building-cell fix with bot replanning (R19: 27.5 % of Cabana objective routes cross a roof bots can't climb) is now approved into the queue as item 2, NEEDS PLAN (Advisor 2026-10-17, R19-ORDER). Advisor LOS-SIZE: the bots' sight-check budget for 48 soldiers is sized in the 48-player performance change-set, with measured cost, together with the `updateSoldierAnim` split (CS11) and the soldier draw-call reduction (about 120 per on-screen soldier); that work is still proposed, not approved, and comes before `TUNE.teams.size` rises. Advisor K4-MEASURE: bot fairness between teams is judged by target searches per bot per second and the share of time with a target (`tools/perfbots.js` fairness), plus which team's bot fires first in mutual sightings (not in the tools yet; R19's plan adds it). After N1 and R19: 3b three-teams-hud, 3c spawn-and-buy, as planned.
- **Core loop v2 (Oct 2026):** see LOOP_V2.md; greybox prototype first (§11).
- **Graphics — rest of Upgrade A:** comic cast shadows (fold the sun's shadow into the 2-band ramp) → painted ground (macro colour patches, baked contact shadows, dirt roads instead of grey tiles). Then **Upgrade B "Comic Impact"**: bright hit flash, hit-stop on every kill, weighted shake, cel explosions, comic muzzle stars, impact frames.
- **Constraints review with Mustafa:** no-addons and no-post-processing date from the artifact era — decide keep / amend.
- **Architecture:** real base buildings (HQ / barracks / refinery / power plant are still placeholder boxes); fix the candy-stripe brick farmhouse; more region presets.
- **AI:** bots route around obstacles (grid A*, `navSteer`); richer behaviours (shield / molotov / officer) still to design.
- **Performance (open):** static kit pieces merged by material (village 398 → 202 draw calls); FX pooling with Comic Impact; pre-declare runtime-added object fields; skip the 3D render while the opaque menu covers the screen (it renders every frame behind it today). See `AUDIT_BUGS_PERF.md` §6.
- **Editor bug (open):** Ctrl+Z (undo) changed the ground textures (old, reported by Mustafa); fixed and merged (PR #8). (The three-ReferenceError editor bug is done: PR #4.)
- **Animation:** actions (flinch, deaths, weapon draw, throws, bayonet thrust), katana + shield on IK, and the Anim Lab (Character Lab tab) — built, awaiting Brave check. Reference + next steps: `ANIMATION_PLAN.md`. DEV: after Unlock All, mouse wheel / 2 cycle every gun.
- **UI theme (approved mockup):** "war-room dossier" drawn in the Living Comic style — inked sheets with halftone cel shadows, stencil headings, rubber-stamp juice, ease-in-out pen marks (4 variants); paper for menu screens, dark field-kit crate for in-game overlays. Source of truth: `ui_theme_mockup.html` (keep in Project Knowledge). Menu + pause done (awaiting Brave); next: service record + end of round → armory + scoreboard, each gated on Mustafa's screenshots; then the first-deploy tutorial — it lists dead controls (V / B / 1–5), so make it the clipboard's field manual (one controls list).
- **Character rework (later):** move from the blocky look to Foxhole-like human proportions; the service-record portrait in the mockup is the style target.
- **Gameplay (design first):** progression (Identity / Momentum / Mastery — design only), infantry gun tiers T1→T4 (held for progression), Spy unit, richer enemy behaviours (shield / molotov / officer), aircraft, multiplayer.

---

## Recent work
*(Newest first, a few lines each. Older items fold into "Build history" above.)*

**The review tracks [2026-11-27, applied; pull request next]** — review-tracks (risk tier 2; the Guard fix chat; his
brief of 2026-11-25 with SOURCE-GO (a), his PLAN-GO (a), MERGE-PROMPT (d), WAKE-END (a), ADDITIONS-OK (a) and SIZE-LINE
(a)): main's `.claude/scripts/track.mjs` gives a change-set its light or full track, its reviewers, its go floors and
the copied-words check on DECISIONS; a pull request merges by the chat that built it on his Advisor's PASS, or on his
go in a chat; the guard skips the merge prompt only on (d)'s conditions, and refuses the PASS line in GitHub tool
writes and every subagent merge; advisor.md's "His go keeps". 1029 added lines (stop line about 980). The advisor agent
and security-auditor APPROVE; the verifier PASS WITH NOTES. Open for him: PASS-FORGE, then his go.

**CI-TIME [2026-11-27, reviewed]** — #78, by the chat he calls CI Builder: the `toolchain` job's time limit 15 to 25
minutes (`.github/workflows/toolchain.yml:38`, his exact text approved in advance; nothing else; the 5-minute browser
stops stay), since Builder V4's newest head took 13 minutes. The advisor agent, security-auditor and infra-engineer
approved; it merges on his go. Report: `docs/reports/2026-11-27-ci-time.md`.
_Pointer (2026-11-27, review-tracks' docs pass): merged as PR #78, merge commit b8f02b01 (13:55:47 UTC, read with `git log`)._

**Change-set 4b-1 of the build plan, Content [2026-11-26, reviewed]** — cs4b-content, the first of 4b's two pull
requests (his SIZE-4B (b)), by lane 1, the chat he calls Builder V4: the content crate with no dependency (one reader
for a cut grammar of JSON that refuses everything else, the canonical writer, the bundle with the movement and map
kinds, the map's exact rules and the aim's reach, the time unit of 3,000 a second), held against a browser engine's
answers for 26,653 numbers and 400 trees, by seven limit cases and by a million-input fuzz run in ten stages,
natively, under Node and in the browsers; the source scan reads the content crate; the links check (his LINKS-CHECK
(c)). 7,164 written lines against the stop line 7,450; no browser check. Two review rounds, no Critical or High; the
planted defects 35 of 35 caught. Report `docs/reports/2026-11-26-cs4b1-content.md`. Next: 4b-2.
_Pointer (2026-11-27, review-tracks' docs pass): merged as PR #75, merge commit b9aad019 (2026-11-26 18:53:13 UTC, read with `git log`)._

**Main's wall [2026-11-26, the wall on; docs in flight]** — main-wall (MAIN-WALL, risk tier 2: repository settings by his
clicks, no code; the Guard fix chat): a second ruleset on main, "main-wall (only the owner updates main)", lets only his
account update main; "Shamble Ruleset 1" is unchanged, so a pull request with gate and smoke green stays required for
every merge, his included. Measured first on a public throwaway repository (deleted by his click after 17:27 UTC [assumed: his "done" of 17:31:41 UTC]); the advisor agent approved
the plan (security-auditor: REVISE, text only, all applied) and both approved the results; rollback is one settings switch. Open: T10 (a day-session and a PR
Advisor merge under the wall). Then, by his SPEED-ORDER (b), the review tracks, then the guard change-set. Report
`docs/reports/2026-11-26-main-wall.md`; TECHNICAL §8c.
_Pointer (2026-11-27, review-tracks' docs pass): its docs merged as PR #76 (2671595f, 18:00:25 UTC, the day-session half of T10); the PR Advisor merged #77 (47bb79e9, 18:52:17 UTC by the merge commit's date; who merged it [assumed: the chat's card]), T10's other half, so T10 passed._

**The preview hand-over step [2026-11-26, in flight]** — ci-preview-handover, by the chat he calls CI Builder, on his
block of 2026-11-26 (risk tier 2; CI and the test configuration only): at the end of the `toolchain` job, on pull
requests from this repository only, a check that the tested player build holds only plain files and folders with no
dot names, then its upload as `preview-site` (7 days) for lane 2's preview workflow, by `actions/upload-artifact`
v7.0.2 pinned by commit (his UPLOAD-SOURCE (a), GUARD-STEP (a), HIDDEN-FILES (b)); TOKEN-WORDS; the recording's replay
test joins the rule tests in Firefox (FIREFOX-REPLAY: 3 passed in the container's Firefox 161). CUT-ESCAPE stays
accepted, its fix a follow-up (his words). Written at d12e3a8 on his CI-WRITE-3 (a); CI: the push run on d12e3a8 green (toolchain 45836151624, gate and smoke 45836152061): the set-up passed with the upload action named, the two new steps skipped on a push, the runner's Firefox 160 "3 passed (12.5s)" with the replay test "first-loop.rec: 1800 steps, every shared hash as in the file", the job 5 min 45 s; the pull request run, the first that runs the two new steps, after the draft is opened. Report
`docs/reports/2026-11-26-ci-preview-handover.md`; waits for the advisor agent's review of the final head and his go.
_Pointer (2026-11-26, 4b-1's docs pass): merged as PR #74, merge commit 3b949c1 (its own time 15:24:22 UTC, read with `git log`)._

**The second guard fix: nothing in front of git or gh unread [2026-11-25 to 11-26, MERGED (#73, 4a62873e)]** —
guard-wrappers (risk tier 2; his SAME-GAP (a); the chat he calls the Guard fix chat): the day guard reads the options of
every wrapper and keyword it strips from one table; an unknown form, or a line at the turn limit, asks in a session and
refuses a subagent; git or gh behind a program it does not strip asks; xargs asks unless a read-only program follows;
the direct git helper check. Guard tests 1551/1551; 166 added lines of code and tests (stop line 200). Four
security-auditor rounds, the fourth by his leave; it left R4-1 open (Critical, one more spelling, recorded by class
only), and by his R4-END (1) what is approved merges. Next, in his order: MAIN-WALL, then R4-1's fix with GUARD-KIT,
then the review tracks. Report `docs/reports/2026-11-26-guard-wrappers.md`. _Corrected 2026-11-26 (MAIN-WALL's docs
pass): merged as PR #73, 4a62873e; his SPEED-ORDER (b) has since put the review tracks before R4-1's fix._

**Change-set 4a of the build plan, The recording [2026-11-25 to 11-26, MERGED]** — cs4a-recording (the first of change-set 4's four
parts, his WALK-CUT (a); the recording came here from the old change-set 3c by his ORDER-3C (c)), by lane 1, built by
the chat he calls Builder V4: a recorded match replays to the same hashes, tick by tick, natively (one process; two
more processes with different hash-map seeds, with a 36,000-step run; two matches in turn) and in WebAssembly on the
file players get (in Node, and in Chromium in a Worker). The first recording, 1,800 steps on today's state, is
committed with its hashes, a word-size vector and a manifest, checked by `make_recordings --check` and every test run.
The one new number on the players' file is the shared state's hash (`Sim::shared_hash`), read only by a replay and fed
straight from the state's one visitor, with no buffer; the hash report (PM-0004-21) keeps FNV-1a 64. On the fixed head
15 planted defect checks were caught at their targets and 4 controls held. A player sees nothing new: no browser
check. 3,300 written lines against the stop line of 3,400 (his STOP-4A (a); the designs counted 1,365); the size
ladder moved 3B-HOSTILE-ALLOC to the link simulator's change-set. Five reviewers in two rounds, all APPROVE in round 2.
Report `docs/reports/2026-11-25-cs4a-recording.md`. Merged: PR #70, 2483e9a, 2026-11-26 12:22:01 UTC by `merged_at`.
_Pointer (2026-11-26, ci-preview-handover's docs pass): merged as PR #70, 2483e9a, 2026-11-26 12:22:01 UTC
(`merged_at`), by merge commit, with the head 420b16b; gate, smoke and toolchain green on main (runs 45818982040 and
45818982052)._

**The small CI pull request after change-set 3a [2026-11-24 to 11-25, MERGED]** — ci-after-3a, by the
chat he calls CI Builder (risk tier 2; CI and the test configuration only): the `toolchain` job runs every crate's
release tests (E1), checks the committed vector files against their two generators (3A-GEN-CHECK), builds `devtools`
for wasm32 on a line of its own (3A-RULES-STEP's CI half) and runs the rule tests in the runner image's Mozilla
Firefox 160.0 (FIREFOX-TESTS (b), his FIREFOX-SOURCE (c)). The first GitHub run was red as he had foreseen (the
runner's Firefox gives the page no WebGL), so by his rule decided in advance Firefox runs the rule tests only and its
drawing tests are a follow-up (FF-DRAW). Green on code head 832268e (Chromium 16 passed, Firefox 2). Report
`docs/reports/2026-11-25-ci-after-3a.md`. Merged: PR #71, 07d673b, 2026-11-25 20:49:09 UTC by `merged_at`.
_Pointer (2026-11-26, ci-preview-handover's docs pass): merged as PR #71, 07d673b, 2026-11-25 20:49:09 UTC
(`merged_at`), by merge commit, by his tap with the head c9460ca pinned (his words of 2026-11-25, 21:50 Amsterdam,
`docs/DECISIONS.md`)._

**Guard fix: a wrapper's option values [2026-11-25, MERGED]** — guard-env-fix (PR #69, risk tier 2; SEC-U7B-1): the day guard reads env, nice and stdbuf options with their values, so git and gh behind them no longer skip its check;
a form it cannot read asks the main session and refuses an agent; routines unchanged. Guard tests 1247/1247; security-auditor APPROVE in round 2; the verifier PASS WITH NOTES. Merged: PR #69, 311c0484, 2026-11-25 16:34:48 UTC by `merged_at`, by merge commit, on his go for head 08bb20af.

**ADR 0006's small revision after the preview hosting check [2026-11-25, MERGED]** — adr0006-header-row, by
lane 2 (the chat he calls Builder V2), on his ONE-OFF-HEADER (a) and STEP4-COUNT (b) (docs only; an ADR draft, so his
go): the header mark (PM-0006-2) is judged on production's own addresses, with the reason in the row; production's
one-off links have the host's header recorded, not judged. Part 1 weighs Workers static assets in a short paragraph
from Cloudflare's docs read 2026-11-25: nothing was measured on Workers, and in the draft Pages stays. The build-count
question is said plainly to be not answered; the pass words of PM-0006-7 and checklist step 4 are re-stated as the
advisor agent's reading of his STEP4-COUNT (b), never as his decision; part 1's "no cap on previews" is dated; his
message of 2026-11-25, 11:07:07 UTC in `docs/DECISIONS.md`. Report `docs/reports/2026-11-25-adr0006-header-row.md`.
PR #67. His answers of 2026-11-25 (14:55 Amsterdam): PASS-WORDS (a), with PM-0006-7 and checklist step 4 reported as
"recorded, the question is not answered"; step 4 achieved (recorded, the question is not answered); WORKERS-NEXT
(a), keep Pages. Main's #68 taken in by one
merge commit. Merged: PR #67, c1da1bd, 2026-11-25 17:13:00 UTC by `merged_at`.

**Change-set 3b of the build plan, The loop closes [2026-11-24 to 11-25, MERGED]** — cs3b-loop-closes (the second
half of change-set 3, his SIZE-3 (b)), by lane 1, built by the chat he calls Builder V3: the page's keys go to the
Worker as posts, and a local server there answers through the real messages (five messages' codec, the transport
interface and the loopback, the room, the client endpoint, local play's one step). A player sees a key diagram (W, A,
S, D, Space, Shift) lit from the local server's echo and two new readout lines (messages; presses). The decoder test
runs at its row's count: the mark has started, never passed. The link simulator and the first recording moved to a
change-set 3c by the plan's size ladder (he was told first); since his ORDER-3C (c) there is no 3c: the recording is
change-set 4a, the link simulator its own change-set after Walk. Final code 6abf3d2a: 6,909 added lines against about
3,800 planned (stop line 7,600); the players' Wasm 53,813 bytes; 114 Rust, 139 Node, 18 boundary and 16 browser tests;
53 of 53 planted defects caught; CI green on it. Six reviewers passed two rounds; the verifier PASS; the advisor
agent approved the code as it stands; his browser check clean in Brave and Firefox (2026-11-25); RELEASE-COUNT
decided, (a) with two conditions. Report `docs/reports/2026-11-25-cs3b-the-loop-closes.md`. Merged: PR #68, 3ab9eae,
2026-11-25 13:57:03 UTC (`merged_at`), by merge commit, from his main Advisor thread on his tap of 13:56:26 UTC, with
the expected head 5e02ee73 (the last commit, which Builder V3 reported its advisor agent had approved); gate, smoke and
toolchain green on it before the merge (the merge commit's message).

**The preview hosting check, ADR 0006's spike 1 [2026-11-15 to 11-25, MERGED]** — preview-hosting-check
(SPIKE_PLAN entry 6), by lane 2, run by the chat he calls Builder V2 in his throwaway test repository
Mergincanli/shamble-preview-check on a test Cloudflare Pages project: 56 runs over the rehearsal and two days of
sittings. A closed pull request's links were gone within 24 hours, through the cleanup job (without it the host kept
them); the login wall covered every preview link and left production's own address open; a pull request reached no
token without his approval and no production; the deleted token was refused. Not passed by the draft's rule:
production's one-off links carry the host's `X-Robots-Tag: noindex` (the header mark, back to the draft), and the
build count (no count shown anywhere) and his verdict on checklist step 4 are open; five marks met with departures
that change no reading. Two review rounds on the build, the verifier PASS WITH NOTES on the evidence; report
`docs/reports/2026-11-24-preview-hosting-check.md`. His answers of 2026-11-25 are in DECISIONS (a normal merge,
by his HISTORY-ID (a)); waits for the advisor agent's review of its final head and his go.
_Pointer (2026-11-25): merged as PR #66, 09faefa, 2026-11-25 11:10:11 UTC (`merged_at`), by merge commit, a normal
merge on his go of 11:07:07 UTC (his HISTORY-ID (a)), with the expected head d7937de._
