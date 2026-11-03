// Workflow script (verbatim, launched twice at about 13:50 Amsterdam, 2026-11-03: wf_30f1279c-530 with args {units: [e1, e2]}, wf_9313e5bc-dda with {units: [e3]}): verifier units and skeptics, effort high (condition 12), model: the session's (Opus 5.5)
export const meta = {
  name: 'v2-drafts-fixcheck-e',
  description: "fixcheck-e: the stop's answers as recorded, in three units (ADR 0001, 0002, 0004; ADR 0003, 0005, 0006 and GLOSSARY §8; the synthesis, the counts and every map row's disposition), each answer against the words and the advisor's rulings; a skeptic per wrong or unsupported verdict",
  phases: [
    { title: 'Verify', detail: 'three verifier units on the recorded answers' },
    { title: 'Skeptics', detail: 'a fresh verifier per wrong or unsupported verdict' },
  ],
}
const C = '/home/user/Shamble-Frontlines/.checkpoints/v2-decision-drafts'
const BASE = '4374a1c'
const SAVE = `## Save as you go (Mustafa, 2026-11-02: usage limits cut agents mid-task; lose as little as possible)
- This applies to every agent of this task, the ones running now and every later wave.
- Create your output file at the start, then add to it as you go: each FIX or NOTE block (or verdict row) goes into the
  file the moment you have composed it, never held back for the end. The session's background saver commits and pushes
  the checkpoint folder every 5 minutes, so whatever is in your files survives a cut.
- Keep the file valid as you go: finish each block (its END line) before you start the next one.
- After each finding or unit of work, add one line to your notes file: what is done, what comes next.
- Scratch files (copies of docs, diffs, scripts, fetched excerpts, applied previews) live in the notes folder, in
  ${C}/<your-unit>-scratch/, never in /tmp, a scratchpad or
  .gate/ (those are not saved). Keep scratch to what you need to resume: for a fetched page, save the excerpt you quote
  and its audit row, not the whole page.
- On a restart, read your notes, your output file and your scratch folder first, and continue after the last complete
  block. Never redo a finished block, and never start the file again from scratch.
(The session adds, for every unit from now on: no binaries and no file over 1 MB in your scratch folder; keep the source and the few lines you quote.)`
const FINDING = { type: 'object', properties: {
  id: { type: 'string' }, severity: { type: 'string', enum: ['high', 'medium', 'low'] },
  kind: { type: 'string', enum: ['wrong', 'unsupported', 'misquote', 'misattributed', 'over-read', 'unmarked reading', 'inconsistent', 'count', 'missing disposition', 'rule', 'other'] },
  doc_line: { type: 'string' }, author: { type: 'string' }, current_text: { type: 'string' },
  source_says: { type: 'string' }, smallest_fix: { type: 'string' } },
  required: ['id', 'severity', 'kind', 'doc_line', 'author', 'current_text', 'source_says', 'smallest_fix'] }
const UNIT_SCHEMA = { type: 'object', properties: {
  verdict: { type: 'string', enum: ['PASS', 'PASS WITH NOTES', 'FAIL'] },
  claims: { type: 'integer' }, confirmed: { type: 'integer' },
  checks: { type: 'array', items: { type: 'object', properties: { check: { type: 'string' }, result: { type: 'string' } }, required: ['check', 'result'] } },
  findings: { type: 'array', items: FINDING }, not_read: { type: 'array', items: { type: 'string' } }, file: { type: 'string' } },
  required: ['verdict', 'claims', 'confirmed', 'checks', 'findings', 'not_read', 'file'] }
const SKEPTIC_SCHEMA = { type: 'object', properties: {
  id: { type: 'string' }, outcome: { type: 'string', enum: ['upheld', 'partly', 'refuted'] },
  evidence: { type: 'string' }, corrected_fix: { type: 'string' }, file: { type: 'string' } },
  required: ['id', 'outcome', 'evidence', 'corrected_fix', 'file'] }

const HEAD = (u) => `You are a fresh verifier, fixcheck-e unit ${u}, for the v2 decision drafts of SHAMBLE FRONTLINES (branch claude/laughing-lovelace-ox61se; the docs on disk; \`git -C /home/user/Shamble-Frontlines log -1 --format=%h\` names HEAD). Mustafa and the Advisor answered the stop on 2026-11-03 (their words, verbatim: ${C}/inputs/STOP-ANSWERS-mustafa-2026-11-03.md). The advisor approved a plan to record them with conditions and rulings (${C}/advisor-verdict-r4.md); its landing map (${C}/stop-answers-map.md, 205 rows) is the work list; the authors' blocks are in ${C}/stop-*-fixes.md (applied in 03ef222 and, for ADR 0002's pass marks, the commit after it). The docs before this round: ${BASE}. You edit no doc; never commit, push or fetch.

For every changed line in your files (\`git -C /home/user/Shamble-Frontlines diff -U3 ${BASE} HEAD -- <your files>\`), check:
1. **The words.** Every quote of an answer is word for word in the inputs file; attribution follows the verdict's condition 2 (Mustafa's: SPIKE-ORDER, TICK-60-HEADROOM, REWIND-CAP, SPIKE-MACHINES, LOOK-TEST-INPUTS, TOY-BOX, LATER; the Advisor's: TICK-60-SETUP, NET-PROFILES, OQ-0002-8, OQ-0005-1, OQ-0001-14, ROOM-CRASH, OQ-0003-20, OQ-0005-AB12, AB13; OQ-0003-17 the Advisor's, confirmed by Mustafa).
2. **No over-reading.** What a line records as decided is what the words say; anything more is either one of the verdict's rulings, marked "(the advisor's reading, 2026-11-03)" and faithful to that ruling, or a finding (kind "over-read" or "unmarked reading"). A partly answered question says "answered in part", what stays open and who decides (OQ-0001-5, OQ-0003-1, OQ-0006-1, -5, -15, -20 among them).
3. **The rulings applied.** Each place follows the verdict's ruling for its answer (for example: the 60 Hz rule with the goal budget and PM-0001-63; the wave-1 run as an early signal, slice 6 deciding; P2's values and the half-each-way split, 2 % loss each direction; the rewind cap's rule, p99, a whole tick, steps and weighings that go; STATUS-FIELD's header wording, condition 4; the presented kinds for OQ-0005-1; AB13's trigger; ROOM-CRASH's "marked crashed").
4. **Nothing else changed.** Rebuild your files from ${BASE} by applying the authors' blocks for them in the session's order (for each doc: test-engineer's, then the section owner's, then, for ADR 0005, game-designer's, engine-architect's and graphics-engineer's; README, SPIKE_PLAN and ENGINE_OVERVIEW: engine-architect-b's; ADR 0002's pass marks: stop-test-engineer-0002-fixes.md last) with \`python3 ${C}/scripts/apply_fixes.py\` on scratch copies, and compare with HEAD: they must be byte for byte the same.
5. **The rules.** No GLOSSARY "Retire (exact)" phrase (\`python3 ${C}/scripts/glossary_check_drafts.py\`, from the repository root); no sentence directly under a table; PM rows of 7 cells; new prose lines at most 120 characters; options kept.`

const units = [
  { key: 'e1', label: 'fixcheck-e1: ADR 0001, 0002, 0004', files: 'docs/adr/0001-multiplayer-netcode.md docs/adr/0002-code-structure-build-toolchain.md docs/adr/0004-simulation-core.md', extra: '' },
  { key: 'e2', label: 'fixcheck-e2: ADR 0003, 0005, 0006, GLOSSARY §8', files: 'docs/adr/0003-rendering.md docs/adr/0005-content-abilities-assets.md docs/adr/0006-infrastructure-operations.md docs/research/GLOSSARY.md', extra: ' For GLOSSARY.md: lines 1–743 keep their bytes (`bash ' + C + '/scripts/write_guard.sh` passes) and only §8\'s 60 Hz entry changed, per the verdict\'s condition 3.' },
  { key: 'e3', label: 'fixcheck-e3: the synthesis, the counts, every map row', files: 'docs/adr/README.md docs/SPIKE_PLAN.md docs/ENGINE_OVERVIEW.md', extra: ' Also, across all nine documents: (a) every one of the map\'s 205 rows has a disposition (a FIX block that applied, or a NOTE with a reason) in some ' + C + '/stop-*-fixes.md file, by its row id, and each NOTE\'s reason is sound; list any row without one; (b) the counts by script: 114 numbered open questions, 98 open (12, 7, 17, 10, 33, 19), pass marks 63, 78, 47, 75, 46, 32 (341), each id once, numbered without gaps, and README and SPIKE_PLAN state the same; (c) consistency: P2\'s values, the 60 Hz rule, the rewind cap\'s rule, wave 1\'s order, the header wording, the reference machines\' status and PM-0001-63 read the same in every document; (d) README keeps no draft line number in a pointer, and every pointer names an OQ id, part or section that exists; (e) `python3 ' + C + '/scripts/dispositions.py ' + C + '/inputs/research-ids.tsv ' + C + '/engine-architect-decision-map-ids.tsv docs/adr/000[1-6]-*.md docs/adr/README.md docs/SPIKE_PLAN.md` gives 521 of 521, each once.' },
]

const skepticPrompt = (u, f, n) => `You are a fresh verifier acting as a skeptic for the v2 decision drafts of SHAMBLE FRONTLINES (branch claude/laughing-lovelace-ox61se; the docs on disk). Try to refute this verdict of fixcheck-e unit ${u}. Read the text, Mustafa's words (${C}/inputs/STOP-ANSWERS-mustafa-2026-11-03.md) and the advisor's verdict (${C}/advisor-verdict-r4.md) yourself; do not trust the verdict's quotes. Decide: upheld, partly (it overstates, or its smallest fix is wrong: give the corrected fix), or refuted (quote what supports the text). You edit no doc, never commit or push, and fetch nothing.

The verdict: ${JSON.stringify(f)}

Write your result to ${C}/verifier-skeptic-8-${u}-${n}.md as you go (notes in the same file; if it ends with DONE, answer from it). Scratch only in ${C}/verifier-skeptic-8-scratch/.

${SAVE}

Return the structured result: the finding's id, upheld, partly or refuted, your evidence, the corrected smallest fix (or "as given" or "none"), and your file's path.`

const chosen = units.filter(u => (args && args.units ? args.units : ['e1', 'e2', 'e3']).includes(u.key))
const results = await pipeline(chosen,
  u => agent(`${HEAD(u.key)}

Your files: ${u.files}.${u.extra}

Write ${C}/verifier-fixcheck-${u.key}-verdicts.md as you go: a claims table (\`| # | doc:line | claim (short) | cited | verdict | where and the words | note |\`), the rebuild's result, "Findings" ranked (exact current text, what the words or the ruling say, the smallest fix, the author). Notes: ${C}/verifier-fixcheck-${u.key}-notes.md (look there first; if they end with DONE, answer from them). Scratch only in ${C}/verifier-fixcheck-${u.key}-scratch/. Finish by 17:30 Amsterdam.

${SAVE}

Return the structured result: PASS, PASS WITH NOTES or FAIL; claims judged and confirmed; one check line per task; every finding; anything not read; your verdicts file's path.`,
    { label: u.label, phase: 'Verify', agentType: 'verifier', effort: 'high', schema: UNIT_SCHEMA }),
  (r, u) => {
    if (!r) { log(`unit ${u.key} returned nothing`); return { key: u.key, unit: null, skeptics: [] } }
    const toCheck = r.findings.filter(f => ['wrong', 'unsupported', 'misquote', 'misattributed', 'over-read'].includes(f.kind))
    log(`${u.key}: ${r.verdict}; ${r.confirmed}/${r.claims}; ${r.findings.length} findings, ${toCheck.length} to the skeptics`)
    return parallel(toCheck.map((f, i) => () => agent(skepticPrompt(u.key, f, i + 1),
      { label: `skeptic ${u.key}: ${f.id}`, phase: 'Skeptics', agentType: 'verifier', effort: 'high', schema: SKEPTIC_SCHEMA })))
      .then(s => ({ key: u.key, unit: r, skeptics: s.filter(Boolean) }))
  })
return { results }
