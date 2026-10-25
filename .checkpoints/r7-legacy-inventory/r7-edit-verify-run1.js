export const meta = {
  name: 'r7-edit-verify',
  description: 'R7: per area, apply the skeptic corrections and coverage findings, then verify every new or widened row',
  phases: [
    { title: 'Edit', detail: 'one editor per area: corrections, missing systems, coverage suggestions' },
    { title: 'Verify', detail: 'a skeptic per area on the new and changed rows only' },
  ],
}
const REPO = '/home/user/Shamble-Frontlines'
const DIR = REPO + '/.checkpoints/r7-legacy-inventory'
const AREAS = ['CD', 'CL', 'CC', 'CM', 'CS', 'GW', 'GA', 'GC', 'GN', 'GE', 'GP', 'ED', 'PG', 'TL', 'KT']
const ROW = {
  type: 'object',
  properties: {
    id: { type: 'string' }, system: { type: 'string' }, what: { type: 'string' }, where: { type: 'string' },
    evidence: { type: 'string' }, state: { type: 'string', enum: ['C', 'M', 'D'] }, v2_note: { type: 'string' },
    mark: { type: 'string', enum: ['port to TypeScript', 'rewrite in Rust', 'redesign', 'drop'] },
    reason: { type: 'string' }, look_brief_pending: { type: 'boolean' },
  },
  required: ['id', 'system', 'what', 'where', 'evidence', 'state', 'v2_note', 'mark', 'reason', 'look_brief_pending'],
}
const EDIT = {
  type: 'object',
  properties: {
    rows: { type: 'array', items: ROW, description: 'the area\'s final rows, every one, in order' },
    changed: { type: 'array', items: { type: 'string' }, description: 'ids of rows that are new, or whose Where/what/mark changed for a missing-system or coverage reason (not just a skeptic correction applied as written)' },
    rejected: { type: 'array', items: { type: 'string' }, description: 'missing systems or coverage suggestions not taken, each with the reason' },
  },
  required: ['rows', 'changed', 'rejected'],
}
const CHECK = {
  type: 'object',
  properties: { checks: { type: 'array', items: { type: 'object', properties: {
    id: { type: 'string' }, verdict: { type: 'string', enum: ['confirmed', 'corrected', 'refuted'] },
    corrections: { type: 'string' }, evidence: { type: 'string' } }, required: ['id', 'verdict', 'corrections', 'evidence'] } } },
  required: ['checks'],
}
const MARKS = `Mark rules (recommendations for Mustafa): rewrite in Rust = a mechanic the authoritative v2 simulation decides that LOOP_V2 keeps (movement, collision, shots, line of sight, damage, team filtering, bot navigation and behaviour, vehicle physics; a rule LOOP_V2 marks [OPEN] says "rule pending LOOP_V2 [OPEN]"). port to TypeScript = client code worth keeping in shape (input, camera, procedural builders, cosmetic animation, synthesized audio, HUD and menu flows). redesign = something v2 already plans to change (where LOOP_V2 replaces a legacy rule, cite its section: eras and income economy, silos and tech boxes, win conditions, spawning and buying, progression §8; the map editor per EDITOR_V2; the look pipeline is "redesign (look brief pending)" with nothing more). drop = dead code and legacy-only workarounds. Every row whose output a player sees has look_brief_pending = true; its mark is about the code technique only, never the appearance.`
const saving = (file) => `## Saving your progress
A usage limit can cut you off part way, and you would then be started again with this same task. Keep notes in ${file} (you may write there and nowhere else). Look there first: if the first line names another task, move it into ${DIR}/old/ and start fresh; if the notes end with DONE, give your answer from them; otherwise go on from where they stop. After each row you finish, add a line (id, what you did, evidence). End with DONE, then answer. Never commit or push.`
const COMMON = `R7, the legacy harvest of SHAMBLE FRONTLINES (a legacy browser game being rebuilt as v2: Rust simulation core, TypeScript client on three.js WebGPU, own netcode). The repository at ${REPO} is read-only for you (main; the engine files shamble-core.js, shamble-game.js, shamble-editor.js and the pages are byte-identical to commit 48ffb51, so cite line numbers as they are now). Inputs: ${DIR}/rows-and-checks.json (per area: the reader's rows, the skeptic's checks with exact corrections and evidence, the skeptic's missing_systems, the reader's not_covered), ${DIR}/coverage.json (the coverage critic: uncovered declarations and suggested_rows naming areas by code), docs/LOOP_V2.md, docs/EDITOR_V2.md, docs/VISION_V2.md, docs/EDITOR_V1_INVENTORY.md, docs/reports/, docs/STATUS.md. Long lines: use grep -n and sed -n ranges, cut long lines. Never invent a number; "none recorded" is fine.`

const editPrompt = (a) => `${COMMON}

${MARKS}

## Your job: the editor of area ${a}
1. Start from the reader's rows for ${a}. Apply every skeptic correction for ${a} exactly as written (they carry evidence); where a correction is ambiguous, check the code and apply the version the code supports.
2. For each of the skeptic's missing_systems for ${a}: check it in the code. If it is a real system no row covers, add a row (next free id ${a}<n>); if it belongs to an existing row, widen that row's Where (and what) instead; if it is not real or already covered, list it under rejected with the reason.
3. Do the same for every coverage suggestion in coverage.json's suggested_rows that names ${a} (a suggestion naming several areas applies only to your rows; "FIX CL12" is a line-number fix).
4. Keep ids stable; never renumber. Every Where is file:line as the code is now; verify each line you add by grep.
Return all of ${a}'s final rows, the ids you changed for reasons 2–3, and what you rejected. Also write the final rows as JSON to ${DIR}/final-${a}.json when done.

${saving(`${DIR}/editor-${a}.md`)}`

const verifyPrompt = (a, e) => `${COMMON}

${MARKS}

## Your job: the second skeptic for area ${a}
The editor of ${a} added or widened these rows: ${JSON.stringify(e ? e.changed : [])}. Their current text is in the editor's result below. Try to refute each one: does every file:line hold what the row says (grep it)? Is it a real system, not a duplicate of another row in the area? Does the mark follow the rules? Default to "refuted" or "corrected" when the code doesn't show it; give exact replacement text per field and the evidence (command or file:line and what it showed).

Rows to check:
${JSON.stringify((e ? e.rows : []).filter((r) => e.changed.includes(r.id)), null, 1)}

${saving(`${DIR}/skeptic2-${a}.md`)}`

phase('Edit')
const results = await pipeline(
  AREAS,
  (a) => agent(editPrompt(a), { label: `edit:${a}`, phase: 'Edit', schema: EDIT, effort: 'high' }),
  (e, a) => (e && e.changed.length
    ? agent(verifyPrompt(a, e), { label: `verify2:${a}`, phase: 'Verify', schema: CHECK, effort: 'high' })
    : Promise.resolve({ checks: [] })).then((v) => ({ code: a, edit: e, verify: v })),
)
const done = results.filter((r) => r && r.edit && r.verify)
log(`areas back: ${done.length}/${AREAS.length}; missing: ${AREAS.filter((a) => !done.find((d) => d.code === a)).join(', ') || 'none'}`)
return { results: done }
