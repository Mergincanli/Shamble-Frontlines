# STATUS: the one-screen dashboard
_Kept current by docs-keeper after every change-set. Mustafa and the Advisor read this first._
_Last updated: 2026-10-31; v2 phase 0 in progress. Since 2026-10-17: kit-v3-0 (PR #26), docs-after-kit-v2-2 (PR #25), R7, the legacy inventory (PR #27), docs-after-r7 (PR #29), the R1–R6 research (PR #28, b6effde, 2026-10-31) and docs-after-v2-research (PR #31, 2f5b52e, 2026-10-31) merged. `main` is protected: merging needs a pull request with the `gate` and `smoke` checks green (Mustafa, 2026-10-17; rules read with the GitHub API). Approved unattended work: [Night queue, issue #16](https://github.com/Mergincanli/Shamble-Frontlines/issues/16)._

## Phase
**v2 phase 0 (V2_PHASE0_BRIEF):** the v2 rebuild, a game-specific engine (`docs/VISION_V2.md` and
`docs/V2_PHASE0_BRIEF.md`, added 2026-10-24). Done: kit-v3-0, the process upgrades (PR #26, f2ccc13, 2026-10-25);
docs-after-kit-v2-2 (PR #25, 4c24d07, 2026-10-27; docs only); R7, the legacy inventory (PR #27, 07ab240, 2026-10-27);
docs-after-r7 (PR #29, e9f11be, 2026-10-27; docs only); R1–R6 research (PR #28, b6effde, 2026-10-31; docs only);
docs-after-v2-research (PR #31, 2f5b52e, 2026-10-31; docs only).
Next: the decision drafts (`docs/ENGINE_OVERVIEW.md`, `docs/adr/0001`–`0006`), planned in a fresh session that Mustafa
starts with the Advisor's drafts brief (DRAFTS-SESSION (a)), starting from the 2026-10-31 decisions
under "Decisions pending"; then a stop for Mustafa and the Advisor; then phase 1 and the playable slices in the new
order (SLICE-ORDER, Mustafa and the Advisor: 1 move & shoot → 2 bots and three teams (played locally) → 3 online → 4
the loop), with, in their words (Mustafa and the Advisor): "The netcode proof test (48
simulated players over WebTransport) still runs before the foundation." The v2 day session
(session_0173vVo5q59ameR6cnN9BENg) handed over and stopped on 2026-10-31. The legacy game is frozen (critical fixes
only).
**Legacy phase 0 (environment): environment + green baseline**: merged (PR #2, 2d30e4a, 2026-10-14, Advisor APPROVE; CS-1 / PR #1 approved
after the fact). Status stays **PARTIAL** until Mustafa's Windows run ("Check a new machine" in tools/README.md: npm ci,
gate, headed RTX renders, headless-gl, `/permissions`, syntax-hook test); Windows-only fixes become a small follow-up
change-set. Gate check 5 is now on main, so that machine needs a fresh `npm ci` in tools/ (eslint-scope) and Node 22.13+.
**Merged 2026-10-14:** editor-prefab-list (PR #4, c458fe7), gate-check-5 (PR #5, 8675367), kit-v1-2 (PR #6, ef019e0),
bot-poplar-wedge (PR #7, 159bec4).
**Merged 2026-10-15/16:** editor-undo-ground (PR #8, b5f4ecf), docs-loop-v2 (PR #9, 67cf55b), map-dedupe-on-load
(PR #10), team-prep-fixes (PR #11, 513f2fc), docs-editor-v2 (PR #12, fec5e67), editor-hotfix (PR #13, 8f4c3b6), map-dedupe-on-load merged as 7ca2cc4.
**Merged 2026-10-17** (each with the Advisor's APPROVE; Mustafa's go where the rules need it): kit-v1-3 (PR #14, 9614ecf; reporting rule),
kit-v1-4 (PR #17, bd27eeb; CLAUDE.md "Night shift" section), kit-v1-5 (PR #18, 2303073; `.claude/NIGHT_SHIFT.md`),
kit-v1-6 (PR #19, cbd79e5; cut-off run pick-up, report kept current), **3a three-teams-core** (PR #15, 5ffe679),
docs-after-3a (PR #20, df1e6d3; docs only), kit-v2-0 (PR #21, 04c086c; the `advisor` agent), kit-v2-1 (PR #22, 211ddcb; automated merging), docs-after-kit-v2-1 (PR #24, e727ce4; docs only, merged by the PR Advisor), kit-v2-2 (PR #23, 48ffb51; the merge go comes after the APPROVE).
_Superseded by v2 (paused in issue #16); the legacy lines below, as of 2026-10-17, are kept for the record:_
**In flight:** none; next game work is N1 from the Night queue.
Editor v2 comes after the playable greybox loop (Advisor's queue; LOOP_V2 §11 defines the loop but has no editor step).
**Greybox phase (LOOP_V2 §11; superseded by v2):** the new core loop (three teams, harvest money, uplink tech, points race) is built as a
greybox prototype with bots, one change-set at a time; each plan says how it meets the [DECIDED] and [DEFAULT] items and
flags any [OPEN] item it needs decided; all numbers live in one tunable data table; [LATER] items are not built.
**Queue (Advisor, 2026-10-15; one change-set at a time; superseded by v2, paused in issue #16):** greybox phase: 1. three-teams, split by the approved plan into
0. team-prep-fixes (merged) → 3a three-teams-core (merged, PR #15) → **N1** (next; editor Play never rebuilds the nav
grid, a one-line change-set; Night queue item 1, APPROVED) → R19 (navBuild building-cell fix; Night queue item 2, NEEDS PLAN;
Advisor 2026-10-17: right after N1, before 3b) → 3b three-teams-hud → 3c spawn-and-buy → "ramps as slopes in SIM" (lets `_botFireWorld` go) →
2. octopon-3-greybox (NC-7 blast reach by line of sight comes before its playtests) →
3. harvest-economy → 4. bot-replan-storm (the navSteer replan storm, bot-poplar-wedge PERF-2) → 5. uplinks →
6. win-and-scoring → then the scale test on the Octopon-3 greybox (N = 12, 24, 36, 48). **Paused until the greybox loop
is playable:** tutorial-field-manual, ui-step-2, ui-step-3, build-spike and the ADR run (Prompt 3). house-textures-seeded
stays low priority and unslotted (seeded house textures also make before/after renders exact). The old order (client
visual/UI work first, gameplay after the foundation refactor) is paused by LOOP_V2 §12.

## In flight
| Work | Branch | State | Waiting on |
|---|---|---|---|
| docs-after-31 (this change-set; docs only: STATUS and ROADMAP after #31's merge, and the v2 day session's handover) | `cs/docs-after-31`, PR #32 | plan approved by the advisor with 5 conditions | the advisor's review of its final head, then Mustafa's go |
| **docs-after-v2-research** (docs only, symbols +0 −0: STATUS, ROADMAP, the research folder and VISION_V2 §3 catch up with PR #28's merge and the 2026-10-31 decisions; the research files carry the answers, 39 exact edits in af2560c; VISION_V2 §3 gets the totals reading, VISION-LINE (a)) | merged: PR #31, 2f5b52e, 2026-10-31, on Mustafa's "merge #31" after the advisor's APPROVE of 1c687d1 | done; the report is PR #31's body | none |
| **R1–R6 research** (six topic files in `docs/research/`, docs only, symbols +0 −0. Labels: R1 and R2 "verified (every recommendation claim)"; R3 "spot-checked (45 of 77 key claims)", R4 "spot-checked (50 of 52 key claims)", R5 "spot-checked (61 of 115 key claims)", R6 "spot-checked (21 of 42 key claims)". Shared: `GLOSSARY.md` (34 terms, starting with the tick budgets) and `UNREAD_SOURCES.md` (180 sources not read, with what each was needed for). The consistency critic's 12 contradictions: 7 resolved, 1 aligned into one shared question, 4 escalated (see Decisions pending). Report `docs/reports/2026-10-31-v2-research.md` and its appendix) | merged: PR #28, b6effde, 2026-10-31, on Mustafa's "merge #28" after the advisor's APPROVE of 854d42f | done; report `docs/reports/2026-10-31-v2-research.md` | none |
| docs-after-r7 (docs only: STATUS and ROADMAP catch up with v2 phase 0 and the Advisor's 2026-10-27 decisions) | merged: PR #29, e9f11be, 2026-10-27, on Mustafa's go after the advisor's APPROVE of d973b3f | done | none |
| kit-v3-1 (write guards for kit and gate files, a builder agent, a default effort, the secret-pattern fix) | `cs/kit-v3-1`, PR #30 (open since 2026-10-27; not a draft; head 5be7d37) | from the R7 session (session_017agAXp7koecrVYw8LUufy5) on Opus 5.5 with the reduced scope (KIT31-STATE (b), the Advisor, 2026-10-27); "shelved" only meant "not in the v2 day session", which keeps away from its branch and files | PR #30 conflicts with main (GitHub: mergeable state "dirty", base e9f11be, read 2026-10-31); it needs an update for main's changes, a day-session advisor's APPROVE of its head, then Mustafa's go (its own merge path) |
| **R7 legacy inventory** (`docs/LEGACY_INVENTORY.md`: 247 systems, each with a recommended mark: rewrite in Rust 45 · port to TypeScript 74 · redesign 96 · drop 32; 118 rows "look brief pending"; `docs/LEGACY_TUNING.md` and `docs/legacy-tuning.json`: 60 tuning tables; read-only `tools/tuning-dump.js`; symbols +0 −0) | merged: PR #27, 07ab240, 2026-10-27, by the R7 session on Mustafa's go after the advisor's APPROVE of head 63be6f6 | done; report `docs/reports/2026-10-27-r7-legacy-inventory.md` | none |
| docs-after-kit-v2-2 (docs only: kit-v2-2's and #24's merges and main's branch protection recorded) | merged: PR #25, 4c24d07, 2026-10-27 | done | none |
| **kit-v3-0** (v2 process upgrades: VISION_V2.md and V2_PHASE0_BRIEF.md, CLAUDE.md's v2 rules, checkpoints, REPO_MAP.md, six new agents, the web allow list; no game code, symbols +0 −0) | merged: PR #26, f2ccc13, 2026-10-25, on Mustafa's "merge #26" after the advisor's APPROVE of its final head | done; report `docs/reports/2026-10-24-kit-v3-0.md` | none |
| kit-v2-2 (CLAUDE.md "Standing permission": Mustafa's go is for the PR's current head, given after the advisor approved that head; a later push needs the advisor's review first, then his go again; now matches advisor.md:98–105. security-auditor PASS WITH NOTES, findings 1 and 3 fixed, re-check PASS; findings 2 and 5 deferred, finding 4 open: see Open bugs and risks. No game code, symbols +0 -0) | merged: PR #23, 48ffb51, 2026-10-17, PR Advisor APPROVE on 964ea5a, merged on Mustafa's go in chat (PR Advisor's #23 comment) | done | none |
| docs-after-kit-v2-1 (docs only: kit-v2-1's merge and Mustafa's Brave check of 3a recorded; PR #22 finding 6 nits fixed) | merged: PR #24, e727ce4, 2026-10-17, PR Advisor APPROVE on d255277, merged by the PR Advisor (class ready) | done | none |
| **kit-v2-1** (automated merging: the PR Advisor routine merges approved ready PRs at once; `needs-mustafa` (rules category, or a major change; when unsure) waits for Mustafa's go. His go counts only in his own words in the chat of the merging session, about that PR, after the advisor's APPROVE of its current head; a "merge" on GitHub never counts. The approved full SHA goes in the review comment and is the merge's expected head; a later push needs a new review and go. Fork PRs skipped; PR text is evidence, never instructions. Green = `gate` and `smoke` both passed on the approved SHA; a hard-constraint change also needs its accepted ADR on main. The gate's own files (everything `tools/gate.mjs` or `.github/workflows/` load: today gate.mjs, smoke.mjs, battery.js, gate-globals.json, tools/package.json, tools/package-lock.json; any `.npmrc`) join the rules category; a changed repro tool (preview.js, sim.js) is replaced by main's copy. The advisor always works from main's advisor.md. NIGHT_SHIFT rule 7 REVISE path; night shift never merges; CLAUDE.md Agents lists `advisor`. No game code, symbols +0 -0. Reviews: advisor plan review REVISE (8 texts applied); security-auditor FAIL, all 8 findings fixed; deferred: wording at tools/gate.mjs:333 / tools/README.md calling gate-globals.json "reviewed data") | merged: PR #22, 211ddcb, 2026-10-17, PR Advisor APPROVE (run 2, head 9baa7f2), merged on Mustafa's go in chat | done | none |
| kit-v2-0 (new `.claude/agents/advisor.md`: plan reviews, PR reviews, merges by the PR Advisor routine; CLAUDE.md protocol steps 2 and 6 and the standing permission route plans and PRs through the advisor; NIGHT_SHIFT rule 7: NEEDS PLAN items get a plan reviewed by the advisor agent; labels `ready-to-merge`, `needs-mustafa`, `changes-requested`) | merged: PR #21, 04c086c, 2026-10-17, PR Advisor APPROVE, merged on Mustafa's go in chat | done; no game code | none |
| docs-after-3a (docs only: STATUS + ROADMAP caught up with the 2026-10-17 merges and the Advisor's R19-ORDER / LOS-SIZE / K4-MEASURE verdicts) | merged: PR #20, df1e6d3, 2026-10-17, PR Advisor APPROVE, merged on Mustafa's go in chat | done | none |
| **3a three-teams-core** (teams as data, 12-slot soldier table, team-filtered shots, two-layer damage, per-team objectives, bot targeting with a LOS budget, DM_Tri arena at `?map=tri`, interim end rule; symbols +22 -0 vs main) | merged: [PR #15](https://github.com/Mergincanli/Shamble-Frontlines/pull/15), 5ffe679, 2026-10-17, Advisor APPROVE, merged on Mustafa's go | done; gate PASS on the merged main (symbols +22 -0, smoke 8/8 + 5/5). Mustafa's first Brave check ran the main build by mistake (Tab showed You alone + Hostile 01-06); renders showed the PR #15 build lists You + Ally 01-03 vs Hostile 01-04. Mustafa checked main (three teams, 3a) in Brave on 2026-10-17: "it looks good" (his words, in the implementer session's chat) | none (3b is next in the greybox queue, after N1 and R19) |
| kit-v1-3 (reporting rule: "Reporting to Mustafa" in CLAUDE.md + advisor-report template) | merged: PR #14, 9614ecf, 2026-10-17, Advisor APPROVE | done | none |
| kit-v1-4 (CLAUDE.md "Night shift": approved unattended work goes into the Night queue, issue #16, the day it is approved; anything needing Mustafa never does) | merged: PR #17, bd27eeb, 2026-10-17, Advisor APPROVE | done | none |
| kit-v1-5 (`.claude/NIGHT_SHIFT.md`: the night shift's rules; the three routines read it from main) | merged: PR #18, 2303073, 2026-10-17, Advisor APPROVE | done | none |
| kit-v1-6 (NIGHT_SHIFT.md: an earlier night's cut-off run is picked up; report comment updated after every finished item) | merged: PR #19, cbd79e5, 2026-10-17, Advisor APPROVE | done | none |
| UI theme transplant, step 1 (menu → dossier, pause → clipboard, field manual) | main (baseline) | delivered, battery PASS | Mustafa's Brave check incl. DEV-panel FPS on the menu |
| kit-v1-1 (settings + 19 deny rules) | merged: PR #3, 5858864, 2026-10-14, Advisor APPROVE | done | Mustafa runs `/permissions` locally (Windows check) |
| kit-v1-2 (narrowed the playwright allow rule) | merged: PR #6, ef019e0, 2026-10-14, Advisor APPROVE | done | none |
| bot-poplar-wedge (`shamble-game.js` only: navBuild clearance 0.7 m + half a cell, navPath start from nearest open cell, 3-call warm-up at end of bootGame; symbols +0 -0) | merged: [PR #7](https://github.com/Mergincanli/Shamble-Frontlines/pull/7), 159bec4, 2026-10-14, Advisor APPROVE | done; report `docs/reports/2026-10-14-bot-poplar-wedge.md` | none |
| editor-undo-ground (`shamble-game.js` only: `addBox`, `addPath`, `makeBuilding`, `makeBarracks` and the `buildMap` terminal now call `stylizeOne()`; `clearMap` now clears `trees[]` and `props[]`; symbols +0 -0) | merged: [PR #8](https://github.com/Mergincanli/Shamble-Frontlines/pull/8), b5f4ecf, 2026-10-15, Advisor APPROVE | done: Mustafa's Brave check passed; report `docs/reports/2026-10-14-editor-undo-ground.md` | none |
| team-prep-fixes (B1 bots hit through the SIM hit world; B7 allocation-light LOS and shot checks via `rayStatics` (103 B / 648-750 B per call); SIM tick in headless runs; B2 bar width; B3 enemy armory; B5 scoreboard churn; B6 map names; B-labels are the three-teams plan's, not AUDIT_BUGS_PERF B1-B3; symbols +5 -0) | merged: [PR #11](https://github.com/Mergincanli/Shamble-Frontlines/pull/11), 513f2fc, 2026-10-16, Advisor APPROVE | done: Mustafa's Brave feel check passed; report `docs/reports/2026-10-15-team-prep-fixes.md` | none |
| docs-editor-v2 (docs only: EDITOR_V2.md + map-editor feature inventory with recommended keep / drop / improve marks) | merged: [PR #12](https://github.com/Mergincanli/Shamble-Frontlines/pull/12), fec5e67, 2026-10-16, Advisor APPROVE | done | Mustafa reviews the inventory marks |
| docs-loop-v2 (docs only: docs/LOOP_V2.md added; pointers in GAME_DESIGN §5–6, DESIGN_NOTES (death tax → Recruit, Spy target → harvester building, Momentum largely answered, three-team question partly answered) and START_HERE's doc list; STATUS / ROADMAP queue) | merged: [PR #9](https://github.com/Mergincanli/Shamble-Frontlines/pull/9), 67cf55b, 2026-10-15, Advisor APPROVE | done | none |
| map-dedupe-on-load (`dedupeMapDef` in game + `edFlashHint` in editor, symbols +2; drop exact duplicate trees / props when a saved map loads; status-bar note; no auto-save) | merged: [PR #10](https://github.com/Mergincanli/Shamble-Frontlines/pull/10), 2026-10-16, Advisor APPROVE | done: Mustafa's Brave check passed; report `docs/reports/2026-10-14-map-dedupe-on-load.md` | Mustafa loads and re-saves each old map once |
| Diagnosis (read-only): poplar wedge + editor Ctrl+Z re-rolls ground textures | n/a | done, both plans approved (wedge: bot-poplar-wedge; Ctrl+Z: editor-undo-ground) | none |
| editor-prefab-list | merged: PR #4, c458fe7, 2026-10-14 | done: Mustafa's Brave check of editor.html passed (no problems), Advisor APPROVE | none |
| gate-check-5 (5a smoke, 5b names, 5c layering, 5d script order; CI jobs gate + smoke) | merged: PR #5, 8675367, 2026-10-14 | done: Advisor APPROVE after his own full-gate run (PASS, 26 s) and mutations that failed correctly | none |
| Phase 0 setup | merged (2d30e4a) | PARTIAL | Mustafa's Windows run |

## Targets
- **v2 (VISION_V2 §3, read with MODES-3 (a): the starting sizes are totals; Mustafa, 2026-10-31):** start with two teams
  with 2–8 players in total (1v1 to 4v4) and three teams with 6–18 in total (2v2v2 to 6v6v6); the goal 12v12v12 or
  24v24; the largest mode 24v24, 48 players. One faction per team, squads of 4, bots fill empty slots; design for 48+
  players (CLAUDE.md).
- **Multiplayer (before VISION_V2, 2026-10-24; LOOP_V2 §2 and DESIGN_NOTES still carry it: DOCS-ALIGN):** three teams,
  one per faction, of 12–16 players each in squads of 4 (12v12v12 to 16v16v16, possibly larger): design for 48+
  players. Bots fill empty slots. Milestones: 2–8 players → 16 → full scale, with load tests at
  each step.

## Decisions pending
- **Taken by Mustafa and the Advisor 2026-10-31 (relayed by Mustafa in the v2 day session's chat):** SLICE-ORDER, the
  order of the v2 playable slices: "1 move & shoot → 2 bots and three teams (played locally) → 3 online → 4 the loop".
  Their conditions, for the decision drafts:
  1. "From slice 1, the game runs as a client plus a local server (the Rust simulation in a Web Worker), talking
     through the same messages the network will carry. No offline-only shortcuts."
  2. "Player movement and shooting are driven by inputs and can be replayed from day one, so prediction slots in later
     without rework."
  3. "The netcode proof test (48 simulated players over WebTransport) still runs before the foundation."
- **Taken by Mustafa 2026-10-31 (in the v2 day session's chat (session_0173vVo5q59ameR6cnN9BENg), with his "merge
  #28"):**
  - MODES-3 (a) (do VISION_V2 §3's starting sizes count players in total or per team): totals; "the biggest match is
    24v24 = 48".
  - PREVIEW-WALL (a) (do the per-PR preview links sit behind a Cloudflare login): "private from the first preview that
    reaches the test game server".
  - SEC-SPREAD (a) (who knows the weapon spread): "server-only spread": the spread stays known only to the server;
    tracers use a similar-looking spread that the hit corrects.
  - ABL-PREDICT (does the client predict the player's own abilities): "later, after the Feel Lab test" (Waiting, below).
- **Taken by the Advisor 2026-10-31 (relayed by Mustafa):**
  - BASELINE-WHEN (b) (when the postponed security baseline is researched): "List it in ADR 0006 as open, with
    temporary positions. Research the security baseline before the first online playtest, in its own fresh session."
  - VISION-LINE (a) (a bracket in VISION_V2 §3 saying the starting sizes are totals): "I wrote that line; the
    clarification is right. Add it as proposed." Added in this change-set (af2560c).
  - DRAFTS-SESSION (a) (should the decision drafts be planned in this session or a fresh one): "Mustafa starts a fresh
    session with the Advisor's drafts brief."
  - TICK-30 (the server tick rate): "The 60 Hz margin (8.28 of 8.33 ms) is effectively zero. For the decision drafts: a
    30 Hz server tick is the baseline for 48 players; 60 Hz only if the netcode proof test shows at least 25 % headroom
    on the target hardware."
- **Waiting (from R1–R6; the full wording is in each topic's "Shared open questions" and the v2-research report):**
  - ABL-PREDICT (Mustafa, after a Feel Lab test that compares both ways on the Molotov and the Shield): does the client
    predict the player's own abilities?
- **Next: the decision drafts** (`docs/ENGINE_OVERVIEW.md`, `docs/adr/0001`–`0006`), starting from the decisions above.
  They carry forward:
  1. The synthesis verifies every ADR claim and number, R3–R6's included (research plan condition 9; the advisor's #28
     review, finding 4).
  2. R2-BUD-REC6's reason (r2-netcode.md:2402-2403) says the marks fit the 50 % server tick budget at 60 Hz on every
     tick but one that both encodes a snapshot and re-floods; that holds at N = 54 (8.99 of 8.33 ms), but at N = 48
     that tick fits by 0.05 ms (8.28 of 8.33 ms) while its server-only share is 0.24 ms over its own 25 % (4.41 of
     4.17 ms) (GLOSSARY.md:62-75, lines at b6effde). Under TICK-30 the drafts restate these marks for the 30 Hz baseline and say what the
     25 % headroom for 60 Hz is measured against (the advisor's #28 review, finding 2).
  3. Where a research mark gives values for 48 and 54, the drafts use 48; where a research sentence says "until
     Mustafa answers …", the answer applies.
  4. R6-HOST-REC4 now recommends a certificate per hostname (C23.b1–b3); four sentences still describe the wildcard as
     R6's proposal (r2-netcode.md:2703-2705, r6-infrastructure.md:47, 938-940 and 1056, lines at b6effde): the drafts
     follow REC4.
  5. No doc defines SLICE-ORDER's "netcode proof test" or "the foundation" yet: the drafts' plan names the research
     spike the proof test is and what the foundation is, for the Advisor to confirm.
- **Open, KIT-ALIGN** (rules files the 2026-10-31 answers left stale; a kit change-set after PR #30 aligns them, with
  Mustafa's go): `CLAUDE.md:7-8` and `.claude/agents/netcode-architect.md:17-19` (the per-team reading of VISION_V2
  §3); `CLAUDE.md:192`, `.claude/agents/security-auditor.md:14` and `.claude/agents/infra-engineer.md:29-30` (the
  security baseline as part of R6); `.claude/agents/infra-engineer.md:23-26` and `.claude/agents/security-auditor.md:46`
  (preview privacy, costs at the starting scale); optional: `.claude/agents/netcode-architect.md:37` and
  `.claude/agents/security-auditor.md:30` (server-only spread as a rule).
- **Open, DOCS-ALIGN** (the Advisor's documents; the session does not edit them): `docs/V2_PHASE0_BRIEF.md` §1 ("Then:
  playable slices, as in VISION_V2"; VISION_V2 has no slice list) and line 126 (R6's row still holds the security
  baseline); `docs/LOOP_V2.md:90-91` (tracers and the Feel Lab test); `docs/LOOP_V2.md:19-20` and
  `docs/DESIGN_NOTES.md:34-35` (team sizes written before VISION_V2: "12v12v12 to 16v16v16, possibly more" in LOOP_V2, "possibly larger" in DESIGN_NOTES).
- Taken by the Advisor 2026-10-30 (relayed by Mustafa): VERIFY-FETCH (a), research may read allow-listed pages with
  curl by day (read-only, the host audit in each report); the night shift stays on WebFetch until kit-v3-1 is merged.
  The unread sources go in `docs/research/UNREAD_SOURCES.md`; the critic's 12 contradictions are resolved before the
  drafts, with a shared glossary (`docs/research/GLOSSARY.md`). NS-2 (c): the night shift starts a fresh session per
  run (carried out 2026-10-31: see Routines).
- Taken by the Advisor 2026-10-28 (relayed by Mustafa): WINDOW-0730 (a), the night shift ends at 07:30 (the R7 session
  writes it into kit-v3-1; the routine's cron already runs 00:10–07:10 (the old routine, switched off 2026-10-31; the new routine runs hourly once Mustafa sets its schedule: see Routines)); NIGHT-RESEARCH (a), research stays daytime
  only; fewer agents at once (4–5) for the same work, so units finish before a spend-limit cut.
- Taken by the Advisor 2026-10-27:
  - R7-MARKS (a): the inventory's marks are working recommendations, changed when their ADR draft comes up; rows
    marked "look brief pending" stay pending until the look brief exists.
  - MANAGED (b): "Mustafa is not an admin of the university's Claude plan", a kit-v3-1 question for the kit-v3-1 PR
    (its question is on the kit-v3-1 branch, which the v2 day session (session_0173vVo5q59ameR6cnN9BENg) may
    not open, so nothing more is written).
  - KIT31-STATE (b): the R7 session finishes kit-v3-1 on Opus 5.5 (not 4.8), with the reduced scope, and its PR
    comes to Mustafa.
  - kit-v3-1 "shelved" for the v2 day session only (KIT31-STATE (b)): it keeps away from the branch and its files.
  - SRC-ACCESS (a): some sources the research needs could not be read: Valve's developer wiki (HTTP 403), GDC Vault (a
    maintenance page) and Unreal's docs (outside the allow list). A recommendation that needs one became an open
    question; the full list is `docs/research/UNREAD_SOURCES.md` (67 hinge rows); for any recommendation that hinges
    on a missing source the Advisor looks for another copy; official-docs domains such as Epic's can be added in the
    next kit PR.
  - Research in the daytime only until kit-v3-1 is merged.
  - The night shift takes only items that need no web access, and lists any permission-prompt stop in its morning
    report.
  - The PR Advisor routine off for now; the advisor agent reviews PRs in the v2 day session
    (session_0173vVo5q59ameR6cnN9BENg).
  - R6 covers hosting, costs, monitoring and telemetry; the security baseline waits.
- Taken by the Advisor 2026-10-27 (R7 session):
  - BRANCH-2 (a): research on `cs/v2-research`, kit-v3-1 on its own branch, kit-v3-1 lands first if they collide.
  - SPEND-1 (b): full checks for R1 and R2 and for anything that becomes a decision, spot checks for R3–R6, review
    rounds stop at nits.
- Taken by the Advisor 2026-10-25 ([Night queue comment](https://github.com/Mergincanli/Shamble-Frontlines/issues/16#issuecomment-6396928581); options in `docs/reports/2026-10-24-kit-v3-0.md`, "Decisions needed"):
  - KV3-MERGE (a): the session merged kit-v3-0 on Mustafa's own 'merge #26', given after the advisor approved its
    final head, not on his earlier advance go.
  - ROUTINE-EFFORT (b): night code changes go through a high-effort builder agent; the night's main session only
    orchestrates.
  - SPIKE-SCOPE (a): once the drafts are approved, the legacy constraints apply to `legacy/` only and `spikes/` is
    throwaway.
  - WRITE-GUARD (a): write limits for kit and gate files, enforced in settings (rules that ask first, or deny rules if
    routines let ask-first rules pass silently), plus the secret-pattern fix; kit changes by day.
  - DOMAINS-ACCEPT (a): six documentation hosts where anyone can publish (github.com, raw.githubusercontent.com,
    docs.rs, crates.io, developer.valvesoftware.com, datatracker.ietf.org) stay on the list of sites fetched without
    a prompt, with the risk recorded. This holds only once WRITE-GUARD is merged; until then research runs in the
    daytime only.
- Taken by the Advisor 2026-10-14: gl advisories accepted for now (drop gl for Playwright-only renders if it won't
  install natively on Windows); hook path (`$CLAUDE_PROJECT_DIR`) waits for the Windows run; check 5 is its own
  change-set after the prefab fix; standing permissions granted (see CLAUDE.md)
- ADR-001..003 below: the v2 drafts `docs/adr/0001`–`0006` replace them (CLAUDE.md, "Decisions pending").
- **ADR-001 Multiplayer architecture** (authority, tick, transport, hosting, offline play): not started. Inputs logged so far: the 11 wall-clock reads (performance.now / Date.now) that make same-seed matches differ; no SRI/CSP on the three.js CDN tag; server-side map-def validation (SEC-MDL-2: `__proto__` key through `envMerge` / `rulesMerge`). Added by 3a: one hit world for every shooter is a multiplayer invariant (K1: bots still use `_botFireWorld`, a view of SIM without ramp slots, until ramps become slopes in SIM); `update()` must become phased (today's real order is in TECHNICAL §4a, K4); the server must repeat the team allow-list (`teamOf`); boot-time `Math.random` draws per slot (`bootGame` soldier loop, `enemyFire` spread, strafe and respawn type); three.js UUIDs draw from the shared `Math.random`; the bot-shot FX cull makes the shared `Math.random` sequence camera-dependent (cosmetic effects need their own RNG stream); `applyDamage` runs `addTech`, whose HUD writes and era toasts should become an event; humans take over bot slots; the LOS budget is sized for 12 soldiers (480 checks/s caps demand from about 24 soldiers in combat; fair since the review fix); map data needs a full schema check server-side (`placeObjectives` checks only the objective keys; `version:1` is written but never read) [verified in code: no `.version` read in shamble-*.js]
- **ADR-002 Code structure & build** (modules, bundler, TypeScript, double-click build): under discussion with Mustafa;
  evidence first via the build spike
- **ADR-003 Three.js version** (stay on r128 vs upgrade): deferred until after ADR-002
- **Design: progression:** LOOP_V2 §8 now settles the shape: team tech tiers in the match; Service points plus
  WARDOGS-style three-gate unlocks on the account; the unlock catalogue is [LATER]. Affects what the server must persist
- **Three-teams plan: APPROVED** with D1-D16 plus three tweaks: D6 (the 200 HP Empire elite leaves the bot roster, done in 3a [verified: `spawnEnemyMesh(_fac,false)`, `elite:false`]; its kit code
  stays for a possible Tier 3 or hero unit), D11 (your own HQ down = you lose; the win and loss rules stay placeholders until
  win-and-scoring) and DM_Tri (`?map=tri` follows LOOP_V2 §9's 6 m gap rule and gets the bot reachability check)
- **LOOP_V2 [OPEN] items:** Spy cost and rules (tech or money, disguise break, hack time, steal amount, warning);
  aiming at height with ledges (decide in octopon-3-greybox); Octopon-3 map size (start near Cabana, adjust with the scale test)

## Routines
_NS-2 (c) (the night shift starts a fresh session on every run) carried out 2026-10-31. These facts are the v2 day
session's read-backs with the routines API at 13:54 UTC and again at about 14:06 UTC; docs-keeper could not check them [assumed]._
- **Night shift: "Shamble Frontline Night shift"** (trig_01QpV6y6UaG1Na2q3jumRDxh), created by Mustafa on the Routines
  page at 13:50 UTC: a fresh session on every run (no persistent session), the repository attached, Opus 5.5, the same
  one-line prompt. Its schedule reads `CRON_TZ=Europe/Amsterdam 10 0 * * *` (once a night, 00:10) until Mustafa sets
  `CRON_TZ=Europe/Amsterdam 10 0-7 * * *` on its page (the API refuses agents' edits to routines they did not
  create). Its test run at 13:51 UTC read `.claude/NIGHT_SHIFT.md` from main and stopped silently, outside the window;
  it has not yet posted on issue #16 (tomorrow morning's comment is the first proof it can post). Until kit-v3-1 is
  merged, it takes only items that need no web access, and a stop at a permission prompt is listed in its morning
  report. Research runs in the daytime only.
- **Old "Night shift"** (trig_019LtRqmAdzddyk6K3e3kDJz, one persistent session): switched off by the v2 day session
  (session_0173vVo5q59ameR6cnN9BENg) at 13:54 UTC, when the night shift moved to a fresh session on every run (NS-2 (c)). Mustafa deletes it after one good night of the new
  routine (deleting it also deletes its session).
- **PR Advisor** (trig_01YPyWAsQmsaQoydhMGG3CkJ; fires on pull request events): off since 2026-10-27 (Mustafa
  switched it off, after an agent's switch-off was refused; the routine reads enabled false at 21:35 UTC). The advisor
  agent reviewed PRs in the v2 day session (session_0173vVo5q59ameR6cnN9BENg) until its handover on 2026-10-31; while
  the routine is off, the session that opens a PR runs the advisor agent on it (CLAUDE.md protocol steps 2 and 6), and
  nothing merges without Mustafa's "merge #<n>".

## Open bugs and risks
- **R1–R6 research** (its report, "Risks"):
  1. Closed 2026-10-31 by MODES-3 (a): the largest mode is 48 players, so the three marks that failed only at 54 are no
     longer at risk. At 48 and 60 Hz the re-flood tick still has almost no margin (carry-forward 2), and the Advisor
     made 30 Hz the baseline (TICK-30).
  2. R3–R6 are spot-checked only: the drafts must verify every ADR claim and number, R3–R6's included.
  3. Unread sources: 67 rows where a recommendation hinges on one wait for other copies (`docs/research/UNREAD_SOURCES.md`).
  4. The org spend limit cut the research five times; notes-first kept every cut recoverable.
- **R7 legacy inventory** (its report, "Risks"):
  1. R1–R6 use the marks before Mustafa confirms them: answered by R7-MARKS (a), the marks are working recommendations.
  2. Some `ANIMATION_PLAN.md` §6 numbers come from headless snippets that are not in `tools/`, so they can't be re-run
     as is.
  3. The inventory's line numbers go stale once phase 1 moves the legacy files to `legacy/`.
- **3a, ranked (all from the change-set's evidence; not re-measured by docs-keeper):**
  1. Cabana: buildings are not NAV cells, so 27.5 % of objective routes and 25 % of siege legs cross a building roof bots can't climb (`tools/reach.js`; R19: the `navBuild` building-cell fix with bot replanning; queued as Night queue item 2, NEEDS PLAN, right after N1 and before 3b)
  2. Allocation grows with team fights (garbage-included inspector profile, MB per game second inside update(), seeded): default Cabana natural match head 4.6-4.8 vs main 2.4-3.1 (head's ally and Soviet bots fight, main's bots don't), Cabana ring combat head 1.72-2.09 vs main 1.94-1.98; DM_Tri 12 soldiers 2.9-3.0 and 48 soldiers 7.2-8.2 after the review fixes (main Cabana at 48: 6.2-6.3). Driver: `updateSoldierAnim` hit/death states (the CS11 anim split); `syncSimWorld` runs twice per step since K4 (0.39-0.43 after the house-array fix, was 0.56-0.65; main 0.16 for its one call)
  3. Draw calls: about 120 per on-screen soldier (pre-existing). Merge per-material parts, prune shadow casters, add LOD before team size rises
  4. DM_Tri: Soviet and Empire enterable barracks doors face south (rot 0 for the nav grid); the HUD shows two teams, two silo dots and "Cabana Republic" (3b)
  5. Colour (3b review, Brave decides): ally teal bars and tracers have little value contrast on grass; unit-type accents tint bot faces red / blue on any team; the 'SUPPLY CACHE' world label clips (pre-existing)
  6. Truck lanes are map data, so moving a refinery in the editor leaves its dock (editor change-set)
  7. `teamSpawnPoint` at team size 16 can put a soldier inside a building, and the spawn line runs along world x (3c)
- First-deploy tutorial lists controls that no longer exist (V melee, B shield, 1–5 switch): every new player sees it (fix paused with tutorial-field-manual until the greybox loop is playable)
- team-prep-fixes follow-ups (low): NC-7 the grenade-arc preview still loops `rayStatic` per static (about 77.7 KB per call while aiming; small perf change-set); PERF-1 `resolveShot` about 112 B per live player (5.4 KB per shot at 48) and PERF-2 the 7 boxed `rayStatics` arguments (the 103 B left per LOS call), both deferred to multiplayer prep; player shots and LOS still see ramp boxes (needs a sloped-ramp primitive in SIM: queued as "ramps as slopes in SIM"); NC-1 (bot sight gate flat at 1.45 m): fixed in 3a, `acquireTarget` now uses each soldier's own height [verified: shamble-game.js:1588]. Label note: "NC-7" here is the grenade-arc preview; the 3a queue's NC-7 is blast reach by line of sight (ROADMAP)
- Editor: Ctrl+Z (undo) changes the ground textures (old, reported by Mustafa): fixed and merged (PR #8, b5f4ecf; Mustafa confirmed Ctrl+Z in Brave). Root cause: `stylizeScene` runs only at boot, so rebuilt roads / crates / buildings / barracks / terminal lost toon + ink; also `clearMap` never cleared trees / props, so every undo / redo / reset / load stacked another copy of all 70 trees and 36 props, also into snapshots and saved maps
- Maps saved on older builds after an undo may contain duplicate trees and props: map-dedupe-on-load drops exact duplicates on Load (PR #10, merged, Brave check passed). Limit: on an old stacked save, Cabana trees and props that the map had deleted or moved come back once. They are single entries, so they stay; Mustafa removes them by hand. Measured on the edited fixture: 69 / 35 intended, 71 / 37 after dedupe
- Deferred (map-dedupe-on-load): prefabs saved by marquee over a stacked area can stamp exact stacks again. Exposure is small: prefabs could only be saved since dbe24f8 (2026-10-14). The next map Save and Load drops them. Follow-up: dedupe on prefab load or stamp with the same key
- Deferred, info, older (SEC-MDL-2): `envMerge` / `rulesMerge` use `Object.assign`, so a stored `__proto__` key swaps the prototype of that local env/rules sub-object (`Object.prototype` stays clean). Belongs with the server-side map-def validation (ADR-001)
- Editor bugs reported by Mustafa (2026-10-15, editor.html on main): placed trees can't be deleted; rotation does nothing when the slider changes; the editor lags after a few placements. Fixed by editor-hotfix (PR #13, 8f4c3b6, merged after Mustafa's Brave check): delete, rotation, inspector selection capture; the lag needed no code fix, PR #8 and PR #10 removed its causes
- N1 (found in 3a): the editor's Play never rebuilds the nav grid; a one-line change-set, Night queue item 1 (APPROVED), next after the 3a merge [verified: `editorPlay` sets mode='game' itself and never calls `deploy()`, where `navBuild()` runs]
- Deferred, low (editor-undo-ground PERF-2): `_disposeObj3D` disposes the shared cached BK textures (houses / covers already did on base; trees / props add the well roof `tile`). r128 re-uploads them next frame, nothing breaks. Follow-up: skip `BK._tex` textures in `_disposeObj3D`
- Pre-existing, low (art review): `serializeMap` rounds to 0.1 m, so the resupply terminal snaps about 2 cm on a rebuild; the editor playtest camera clips into the barracks roof (no roof cutaway in playtest)
- Deferred from gate-check-5 reviews: no SRI/CSP on the three.js CDN tag (belongs with hosting, ADR-001)
- GitHub branch protection on `main` is on (Mustafa, 2026-10-17, in the implementer session's chat: "PROTECT-MAIN is done, main is protected"). Rules, read with the GitHub REST API on 2026-10-17 (`gh api repos/Mergincanli/Shamble-Frontlines/rules/branches/main`): "Shamble Ruleset 1" (active, targets the default branch `main`) blocks deleting and force-pushing `main`, requires a pull request (0 approvals) and requires the `gate` and `smoke` checks; a branch behind `main` may still merge (not strict); no classic protection rule. The `.claude/settings.json` deny rules stay a second layer (fragile by design). Decision PROTECT-MAIN: done (recommended option (a), plus a required pull request)
- Fixed (PR #22 review, finding 5): CLAUDE.md's merge sentence now says the go comes after the APPROVE of the PR's current head and a later push needs a new review and go, as advisor.md:98–105 does (kit-v2-2, PR #23, 48ffb51)
- Deferred, low (kit-v2-1): `tools/gate.mjs:333` and `tools/README.md:34` still call `gate-globals.json` "reviewed data, not a gate tool", which contradicts kit-v2-1's category 3 (gate files need Mustafa's go). It errs on the safe side; fix the wording when gate.mjs is next touched
- Open, for a later security review (kit-v2-1): night-shift rules 3-4 (`.claude/NIGHT_SHIFT.md`) trust any "Night shift running" comment on the public issue #16, whoever wrote it, so an outside comment could make a night run stop silently or take over a "cut-off" item
- Open, medium (security review of kit-v2-2, finding 4; existed before kit-v2-2): CLAUDE.md "Standing permission" lets a session accept an advisor approval read from a PR comment by the `Mergincanli` account, but every session posts as that account (`advisor.md`:103–104), so a confused or injected session could post "APPROVE <SHA>" on its own head. Mustafa's own go in chat is still required. The fix is its own change-set and still needs a plan; that plan picks the option (accept only the merging session's own advisor verdict, or a check run sessions can't write) as a decision for Mustafa
- Deferred, low (security review of kit-v2-2, findings 2 and 5): "the advisor's review" can be read as any verdict, not only an APPROVE, and smaller wording gaps remain between CLAUDE.md and advisor.md (listed in reports/2026-10-17-kit-v2-2.md, Reviews); one later rules PR edits both files together (needs Mustafa's go)
- The 3D scene renders every frame behind the opaque main menu (wasted GPU)
- Bot nav, editor-made layouts (R1, tracked per the Advisor): rotated colliders at odd angles keep up to 0.34 m of solid inside free nav cells, and wall gaps of about 3-5.4 m can close for bots depending on grid alignment (built-in maps unaffected). Recommendation: an editor-side reachability check; new maps keep walkable gaps at least 6 m wide
- Bot nav, navSteer "replan storm" (old; bot-poplar-wedge PERF-2): a bot whose straight line flickers clear/blocked replans every few frames; the wedge fix hits it more on Romania (budget-limited frames 0.9% -> 1.8%). Advisor: its own change-set in the greybox phase (bot-replan-storm)
- Bot nav, A* gives up after 8000 nodes (old): a few far-corner routes fail and fall back to a straight line
- The game reads performance.now()/Date.now() in 11 places, so matches with the same seed differ run to run. Tracked as an input for ADR-001 (deterministic shared simulation)

## Health (latest measurements)
| Metric | Value | When | Source |
|---|---|---|---|
| Combat allocation (GC churn) | ~4.9 MB/s (audit harness, scripted combat; not comparable to the 2026-10-15 headless match) | 2026-09-26 | audit CS11 |
| Perf baselines, team-prep-fixes head (headless): `hasLOS` | 103 B per call (was 20-26 KB) | 2026-10-15 | reports/2026-10-15-team-prep-fixes.md |
| `resolveShot` / `enemyFire` / `syncSimWorld` | `resolveShot` 648-750 B at 6 players, about 5.4 KB at 48 (was 18.1 KB) · `enemyFire` 0.88-1.0 KB per call (was 3.7-7.0 KB) · `syncSimWorld` 561 B per call (pre-existing, the houses fallback array) | 2026-10-15 | reports/2026-10-15-team-prep-fixes.md |
| Allocation method (the audit row is stated to use it; the team-prep-fixes rows do not name their method [assumed the same]) | Inspector sampling profile with `includeObjectsCollectedByMajorGC` / `MinorGC` (garbage included). `node --heap-prof` counts only objects alive at exit and is NOT used for rates. 3a rows are in Open bugs and risks (item 2); they come from the 3a commit messages, not re-run here | 2026-10-16 | tools/perfbots.js header |
| Combat garbage per game second | close-combat ring 6 / 12 bots 2.5-3.2 / 2.0-2.5 MB (was 15.5-16.1 / 28.3-28.5) · natural 90 s match 1.79-1.97 MB (was 2.80-3.05) | 2026-10-15 | reports/2026-10-15-team-prep-fixes.md |
| Village draw calls | 202 (was 398) | 2026-09-26 | audit CS12 |
| Headless match stall % (tools/sim.js, 40 unseeded runs each) | base mean 8.82 / median 7.5 · bot-poplar-wedge mean 7.73 / median 6.5 | 2026-10-14 | sim.js |
| Seeded match pairs (240) | tree-face stall windows 1885 -> 0; mean stall 9.12% -> 8.50% | 2026-10-14 | scratch harness (report) |
| Editor rebuild (Cabana): full-map draw calls boot / 1 / 2 rebuilds | base 1992 / 2406 / 3502 · head 1992 / 1992 / 1992; heap after GC boot / 10 rebuilds: base 42.9 / 119.5 MB · head 42.7 / 44.5 MB | 2026-10-14 | preview.js harness + perf-auditor |
| Gate | on main 8675367: full gate PASS in 26 s (Advisor's independent run), all 8 checks (1-4, 5a-5d); core 134 / game 691 / editor 349, 44 editor refs guarded (re-measured on main + kit-v1-2), smoke 8/8 + 5/5 steps | 2026-10-14 | gate.mjs |
| Gate (official battery, older) | 0 flagged, 44 editor refs all guarded | 2026-10-10 | battery.js |
