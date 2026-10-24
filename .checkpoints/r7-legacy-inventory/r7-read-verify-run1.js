export const meta = {
  name: 'r7-read-verify',
  description: 'R7 legacy inventory: one reader per area, a skeptic per area, then a coverage critic',
  phases: [
    { title: 'Read', detail: 'one reader per area writes inventory rows with file:line and evidence' },
    { title: 'Verify', detail: 'a skeptic per area tries to refute every row' },
    { title: 'Coverage', detail: 'a critic lists top-level declarations no row covers' },
  ],
}
const WT = '/tmp/claude-0/-home-user-Shamble-Frontlines/dbb5e494-b49f-5970-85a2-528a1e15b24b/scratchpad/r7'
const NOTES = WT + '/.checkpoints/r7-legacy-inventory'
const SHA = '48ffb51'

const AREAS = [
  { code: 'CD', name: 'core data and persistence', scope: 'shamble-core.js lines 1-232: the manifest, KVStore, the tuning object T, TEAMS, TUNE, WEAPONS, UNIT_TYPES, VEHICLE_TYPES, ENV_DEFAULT and the environment helpers, the render globals (scene, camera, renderer, lights, ground, sea, beach).' },
  { code: 'CL', name: 'core look pipeline', scope: 'shamble-core.js lines 233-559: the NPR pipeline (toonify, cel ramps, makeOutlineMat, addHull, ensureOutlineNormals, stylizeScene, stylizeOne, INK and outline constants), comic SFX sprites, mergeStaticGroup and _matSig, procedural textures.' },
  { code: 'CC', name: 'core characters, factions and guns', scope: 'shamble-core.js lines 560-845: the character builder (buildEntity, buildCharacter, assembleChar, CPARTS, CHAR_* constants), FACTION_PALETTES, applyFaction, applyVillain, the gameplay soldier kit (SOLDIER_KIT, ELITE_KIT, buildSoldier if here), weapon-prop makers, GUN_KIT, buildGunModel and sockets, the 2-bone arm IK, poseArms.' },
  { code: 'CM', name: 'core maps and building kit', scope: 'shamble-core.js lines 846-1473: cabanaRepublicMap, parseGridToMapDef, DM_Cabana and its village pass, DM_Romania, DM_Tri, the building kit BK (footprints, roofs, skins, dormers, BK.roll), the house makers (timber house, thatch cottage, brick-and-flint farmhouse, Norman manor), _bkWorldBoxes, cover pieces, the building-kit game glue.' },
  { code: 'CS', name: 'core headless SIM math', scope: 'shamble-core.js lines 1474-1543: the fenced SIM block (FIXED_DT, BTN, BODY, PART, raySphere, rayCapsule, rayAabb, rayStatic, rayStatics, resolveShot, SIM, hasLOS, makeInputCommand if present, hyp2/hyp3 wherever defined).' },
  { code: 'GW', name: 'game world, map building and movement physics', scope: 'shamble-game.js: lines 1-223 (manifest, PLAYER_FACTION, RULES_DEFAULT and applyRules, the environment model, platforms and ramps), 574-683 (architecture, cover glue, vegetation, orchard), 2527-2608 (vertical surface query, cliff collision, gravity and jump, unified occlusion), 3099-3219 (farmyard props); plus buildMap, clearMap, serializeMap, dedupeMapDef, resolveObstacles, syncSimWorld, curMap wherever they are (grep).' },
  { code: 'GA', name: 'game animation (ANIM v2)', scope: 'shamble-game.js lines 224-573 (ANIM, _animInit, updateSoldierAnim, IK holds, planted feet, stride, greatcoat panels, reloads, flinch, deaths, draw, throws, bayonet; the cosmetic action hooks) and the player melee and shield held models near 684. ANIMATION_PLAN.md has the metrics.' },
  { code: 'GC', name: 'game combat, teams and vehicles', scope: 'shamble-game.js: player weapons and firing (fireOnce, castShot, playerFireOrigin, setHeldGun, reloads, ADS), damage and teams (PLAYER_TEAM, teamOf, sameTeam, teamFilter, TEAM_INDEX, SOLDIERS, activateRoster, applyDamage, onDamageEvent, damageEnemy, damagePlayer, playerDie, respawn), the unit-type system near 1155, the Juggernaut near 1185-1433, melee and shield near 1654, charged throws and grenades near 1694, the Torque Bow near 1784; grep for each.' },
  { code: 'GN', name: 'game bots and navigation', scope: 'shamble-game.js: enemies and bot AI near 1434-1653 (setEnemyType, AI_STATE, acquireTarget, the LOS budget, enemyFire, _botFireWorld, verticality helpers, nearestEnemyStructure, teamSpawnPoint, spawnEnemyMesh) and NAV near 2609-2965 (navBuild, navPath, navSteer, A*, the replan logic); grep for each.' },
  { code: 'GE', name: 'game economy, objectives and progression', scope: 'shamble-game.js: ECO near 689, eras and tech (addTech, priceMult, the power plant), silos near 967, the armory shop near 1867, harvesters and the supply convoy near 3220, tech boxes near 3227, placeObjectives, HQ and win rules (checkEnd), progression Prog near 3312, the career screen; grep for each.' },
  { code: 'GP', name: 'game presentation, input and dispatch', scope: 'shamble-game.js: the FX pool near 1011 (FXPool, spawnDamageNumber, _dnPool), the perf overlay and camera presets near 2168, WebAudio SFX near 2218-2325, the headshot skull, the XP banner, the onboarding overlay near 2334, the kill vignette near 2344 (vigShot), the main menu camera near 2966, the war-room UI near 2993 (wr*, openManual, closeManual), the HUD and per-frame DOM writes (_sd, _so, _st), input listeners, the dev panel near 3320, feel systems near 3357, and the dispatch: loop, update, deploy, backToMenu, bootGame, the let warm-up block at the end; grep for each.' },
  { code: 'ED', name: 'editor', scope: 'shamble-editor.js (all 1252 lines): the map editor (summarise it in a few rows that point to docs/EDITOR_V1_INVENTORY.md, which already lists its features row by row; do not repeat that inventory), the Character Lab (clRebuild, clTick), the Anim Lab (alTick, AL_ACTIONS, alEnsureUI), prefabs and save/load, bootEditor, and editor-only hooks the game calls behind typeof guards.' },
  { code: 'PG', name: 'pages', scope: 'index.html and editor.html: the head (CSS, the wr- war-room theme, tokens, keyframes), the DOM (menus, HUD, overlays and their z-order, the field manual, career, shop, scoreboard, tutorial, dev panel), the script tags and the inline script (bootGame, bootEditor, the menu tail), the three.js r128 CDN tag; the differences between the two pages; docs/ui_theme_mockup.html as the UI source of truth.' },
  { code: 'TL', name: 'tools (the verification toolkit)', scope: 'tools/: gate.mjs (checks 1-4, 5a-5d), smoke.mjs, battery.js, gate-globals.json, preview.js, pageshot.js, sim.js, reach.js, perfbots.js, aabbs.js, village.py, package.json and the lockfile, README.md; .claude/hooks/syntax-gate.mjs.' },
  { code: 'KT', name: 'kit (process)', scope: 'CLAUDE.md, .claude/ (agents, skills, rules, hooks, settings.json, NIGHT_SHIFT.md, claude-security-guidance.md, security-patterns.json), .github/workflows/gate.yml, docs/reports/ and the docs process (STATUS, ROADMAP, ADR template), FIRST_PROMPTS.md, the routines. Routines (facts from the session, 2026-10-24; list only these SHAMBLE FRONTLINES routines): "PR Advisor (Shamble Frontlines)" trig_01YPyWAsQmsaQoydhMGG3CkJ, enabled, fired by GitHub events (no cron), model claude-opus-5-5, last run 2026-10-17; "Night shift" trig_01T8Gjr1xk1gG1Jf3beYV1xq, disabled, cron 5 22 * * * (UTC); "Night Shift 2" trig_01KEaCB725bRjEaHD9fn2QKq, disabled, cron 10 3 * * *, model claude-opus-5-5; "Night Shift 3 (Morning)" trig_012NWfZ7skzEapaohx12Zteu, disabled, cron 30 5 * * *. Kit rows describe main at 48ffb51; add the note "changed by kit-v3-0" to the v2 note where a row is about the agents, the model policy, checkpoints, the night shift or settings.json (kit-v3-0 is an open rules PR that changes those). No other project, repository or routine is read, cited or listed.' },
]

const MARKS = `## The marks (recommendations; Mustafa confirms or changes each)
- **rewrite in Rust**: a mechanic the authoritative v2 simulation decides that LOOP_V2 keeps: movement, collision, shots, line of sight, damage, team filtering, bot navigation and behaviour, vehicle physics. A rule LOOP_V2 marks [OPEN] gets "rule pending LOOP_V2 [OPEN]" in the reason.
- **port to TypeScript**: client-side code worth keeping in shape for the new client: input mapping, camera, procedural builders, cosmetic animation, synthesized audio, HUD and menu flows.
- **redesign**: a system v2 already plans to change. Where LOOP_V2 replaces a legacy rule, the mark is redesign and the reason cites the LOOP_V2 section: this covers the eras and income economy, the silo and tech-box objectives, win conditions, spawning and buying, and progression (§8). The map editor follows EDITOR_V2. The look pipeline itself is "redesign (look brief pending)" with no further recommendation.
- **drop**: dead code and legacy-only workarounds (classic-script globals and the checks that exist only because of them, three.js r128 limits, the artifact-runtime storage path).
- **The look is not decided** (Mustafa and the Advisor are writing the look brief). Every row whose output a player sees carries look_brief_pending = true, whatever its mark: procedural builders, materials, characters, FX, HUD and war-room UI, the look pipeline. For those rows the mark is about the code technique only, never the appearance.
- v2 in one line: a Rust simulation core compiled to Wasm (browser, Web Worker) and native (server), a TypeScript client on the latest three.js WebGPURenderer with a WebGL 2 fallback, our own netcode (server authority, WebTransport with a WebSocket fallback). Read docs/VISION_V2.md §6 and docs/LOOP_V2.md before marking.`

const ROW = {
  type: 'object',
  properties: {
    id: { type: 'string', description: 'area code + number, e.g. CD3' },
    system: { type: 'string' },
    what: { type: 'string', description: 'what it does, one or two plain sentences' },
    where: { type: 'string', description: 'file:line at 48ffb51 and the main functions or constants' },
    evidence: { type: 'string', description: 'numbers and proof we have: report file + figure, AUDIT_BUGS_PERF section, STATUS health row; "none recorded" if none' },
    state: { type: 'string', enum: ['C', 'M', 'D'], description: 'C code read at 48ffb51 · M measured in a report or harness · D a doc claim not re-checked' },
    v2_note: { type: 'string', description: 'what v2 should keep, learn or avoid; known bugs; LOOP_V2 or EDITOR_V2 section where it changes' },
    mark: { type: 'string', enum: ['port to TypeScript', 'rewrite in Rust', 'redesign', 'drop'] },
    reason: { type: 'string', description: 'one line' },
    look_brief_pending: { type: 'boolean' },
  },
  required: ['id', 'system', 'what', 'where', 'evidence', 'state', 'v2_note', 'mark', 'reason', 'look_brief_pending'],
}
const READ = { type: 'object', properties: { rows: { type: 'array', items: ROW }, not_covered: { type: 'array', items: { type: 'string' }, description: 'things in your scope you saw but did not turn into a row, and why' } }, required: ['rows', 'not_covered'] }
const VERIFY = {
  type: 'object',
  properties: {
    checks: { type: 'array', items: { type: 'object', properties: {
      id: { type: 'string' }, verdict: { type: 'string', enum: ['confirmed', 'corrected', 'refuted'] },
      corrections: { type: 'string', description: 'exact replacement text per field, as field: new text; empty if confirmed' },
      evidence: { type: 'string', description: 'the command run or file:line read, and what it showed' } },
      required: ['id', 'verdict', 'corrections', 'evidence'] } },
    missing_systems: { type: 'array', items: { type: 'string' }, description: 'systems in this area that no row covers, with file:line' },
  },
  required: ['checks', 'missing_systems'],
}

const saving = (file) => `## Saving your progress
A usage limit can cut you off part way, and you would then be started again with this same task. Keep notes in ${file} (make the folder; you may always write there and nowhere else). Look there first: if the first line names another task, move the file into ${NOTES}/old/ and start fresh; if the notes end with DONE, give your answer from them; otherwise go on from where they stop. Start a new file with one line naming your task. After each row or check you finish, add it to the file (the row's fields in a few lines, or the check's verdict and evidence). End with DONE, then give your answer. Never commit or push.`

const COMMON = `SHAMBLE FRONTLINES is a browser team shooter (legacy build: Three.js r128, classic scripts sharing globals, PvE against bots). It is being rebuilt as v2; this is R7, the legacy harvest: an inventory of every legacy system for the v2 team. The repository at main ${SHA} is checked out read-only for you at ${WT}: read files there (line numbers must be those of that checkout), and never edit, create or delete anything except your own notes file. The engine files have very long lines: use grep -n and read line ranges (sed -n 'a,bp'), and cut long lines when you print them. Evidence and numbers come from docs/reports/*.md, docs/AUDIT_BUGS_PERF.md, docs/STATUS.md (health table and risks), docs/ANIMATION_PLAN.md and docs/TECHNICAL.md: cite the file and the figure; never invent a number; "none recorded" is a valid answer. Background reading as needed: docs/MODULE_MAP.md, docs/START_HERE.md §6 (grep targets), docs/LOOP_V2.md, docs/EDITOR_V2.md, docs/VISION_V2.md.`

const readPrompt = (a) => `${COMMON}

${MARKS}

## Your area: ${a.code}, ${a.name}
Scope: ${a.scope}
Write one row per system (a system is a coherent piece a v2 engineer would port, rewrite, redesign or drop as a unit: usually 4-15 rows for an area). Row ids are ${a.code}1, ${a.code}2, … in reading order. For each row: what it does in plain words; Where as file:line at ${SHA} plus the main functions or constants (verify every line number by grep); the evidence and numbers we have (report file and figure); state C, M or D; a v2 note (what to keep, learn or avoid; known bugs from STATUS or the reports; the LOOP_V2 or EDITOR_V2 section where it changes); the recommended mark with a one-line reason; look_brief_pending.
List under not_covered anything in your scope you left out, with the reason.

${saving(`${NOTES}/reader-${a.code}.md`)}`

const verifyPrompt = (a, r) => `${COMMON}

${MARKS}

## Your job: the skeptic for area ${a.code} (${a.name})
Another agent wrote the rows below. Try to refute every row: re-open each Where (does the file:line at ${SHA} hold that function or constant, and does it do what "what" says?), each number (is it in the cited report, with that value?), the state tag, and the mark (does it follow the mark rules above, including the LOOP_V2 and look rules?). Default to "refuted" or "corrected" when you cannot confirm a claim from the files; give the exact replacement text for each corrected field, and the evidence (command or file:line, and what it showed) for every verdict. Then list systems in this area's scope (${a.scope}) that no row covers, with file:line.

Rows:
${JSON.stringify(r ? r.rows : [], null, 1)}

${saving(`${NOTES}/skeptic-${a.code}.md`)}`

const COVER = {
  type: 'object',
  properties: {
    per_file: { type: 'array', items: { type: 'object', properties: {
      file: { type: 'string' }, top_level_total: { type: 'number' }, covered: { type: 'number' },
      uncovered: { type: 'array', items: { type: 'string' }, description: 'name @ line, grouped where they form one system' } },
      required: ['file', 'top_level_total', 'covered', 'uncovered'] } },
    suggested_rows: { type: 'array', items: { type: 'string' }, description: 'systems that deserve a new row: name, file:line range, which area' },
    method: { type: 'string' },
  },
  required: ['per_file', 'suggested_rows', 'method'],
}

phase('Read')
const results = await pipeline(
  AREAS,
  (a) => agent(readPrompt(a), { label: `read:${a.code}`, phase: 'Read', schema: READ, effort: 'high' }),
  (r, a) => agent(verifyPrompt(a, r), { label: `verify:${a.code}`, phase: 'Verify', schema: VERIFY, effort: 'high' }).then((v) => ({ code: a.code, read: r, verify: v })),
)
const done = results.filter(Boolean)
const failed = AREAS.map((a, i) => (results[i] && results[i].read && results[i].verify) ? null : a.code).filter(Boolean)
log(`areas back: ${done.length}/${AREAS.length}; incomplete: ${failed.join(', ') || 'none'}`)

phase('Coverage')
const allRows = done.flatMap((d) => (d.read ? d.read.rows : []).map((x) => ({ id: x.id, system: x.system, where: x.where })))
const coverage = await agent(`${COMMON}

## Your job: the coverage critic
Below are the Where fields of every inventory row (id, system, where). For each of shamble-core.js, shamble-game.js and shamble-editor.js at ${WT}, list every top-level declaration (function, const, let, class at column 0, e.g. with grep -nE '^(async )?function |^(const|let|var|class) '), and count how many of them some row's Where names or clearly includes (a row that names a line range covers the declarations inside it only when its "system" plausibly owns them; be strict). List the uncovered ones as name @ line, grouped into systems where they belong together, and suggest the rows that are missing (name, file:line range, which area code: CD CL CC CM CS for core, GW GA GC GN GE GP for game, ED for editor). Say exactly how you counted.

Rows:
${JSON.stringify(allRows, null, 1)}

${saving(`${NOTES}/critic-coverage.md`)}`, { label: 'critic:coverage', phase: 'Coverage', schema: COVER, effort: 'high' })

return { results: done, failed, coverage }
