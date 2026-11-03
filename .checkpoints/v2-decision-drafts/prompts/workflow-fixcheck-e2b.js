// Workflow script (verbatim, copied at launch, run wf_58154963-540, 2026-11-03 about 17:25 Amsterdam): verifier and skeptics, effort high, model: the session's (Opus 5.5)
export const meta = {
  name: 'v2-drafts-fixcheck-e2b',
  description: "fixcheck-e2's gap after the cut: ADR 0005's changed lines (OQ-0005-1's presented kinds, AB12, AB13's trigger, LOOK-TEST-INPUTS, SPIKE-ORDER, the header, line lengths, tables) against the words and the advisor's rulings; a skeptic per wrong verdict",
  phases: [ { title: 'Verify', detail: 'one verifier on ADR 0005' }, { title: 'Skeptics', detail: 'a fresh verifier per wrong or unsupported verdict' } ],
}
const C = '/home/user/Shamble-Frontlines/.checkpoints/v2-decision-drafts'
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
const FINDING = { type: 'object', properties: { id: { type: 'string' }, severity: { type: 'string', enum: ['high', 'medium', 'low'] }, kind: { type: 'string', enum: ['wrong', 'unsupported', 'misquote', 'misattributed', 'over-read', 'unmarked reading', 'inconsistent', 'count', 'rule', 'other'] }, doc_line: { type: 'string' }, author: { type: 'string' }, current_text: { type: 'string' }, source_says: { type: 'string' }, smallest_fix: { type: 'string' } }, required: ['id', 'severity', 'kind', 'doc_line', 'author', 'current_text', 'source_says', 'smallest_fix'] }
const UNIT_SCHEMA = { type: 'object', properties: { verdict: { type: 'string', enum: ['PASS', 'PASS WITH NOTES', 'FAIL'] }, claims: { type: 'integer' }, confirmed: { type: 'integer' }, checks: { type: 'array', items: { type: 'object', properties: { check: { type: 'string' }, result: { type: 'string' } }, required: ['check', 'result'] } }, findings: { type: 'array', items: FINDING }, not_read: { type: 'array', items: { type: 'string' } }, file: { type: 'string' } }, required: ['verdict', 'claims', 'confirmed', 'checks', 'findings', 'not_read', 'file'] }
const SKEPTIC_SCHEMA = { type: 'object', properties: { id: { type: 'string' }, outcome: { type: 'string', enum: ['upheld', 'partly', 'refuted'] }, evidence: { type: 'string' }, corrected_fix: { type: 'string' }, file: { type: 'string' } }, required: ['id', 'outcome', 'evidence', 'corrected_fix', 'file'] }
phase('Verify')
const u = await agent(`You are a fresh verifier, fixcheck-e unit e2b, for the v2 decision drafts of SHAMBLE FRONTLINES (branch claude/laughing-lovelace-ox61se; the docs on disk; \`git -C /home/user/Shamble-Frontlines log -1 --format=%h\` names HEAD). Mustafa and the Advisor answered the stop on 2026-11-03 (their words, verbatim: ${C}/inputs/STOP-ANSWERS-mustafa-2026-11-03.md); the advisor approved a plan to record them with conditions and rulings (${C}/advisor-verdict-r4.md); the authors' blocks are in ${C}/stop-*-fixes.md (for ADR 0005: test-engineer's, game-designer's, engine-architect-a's and graphics-engineer's; applied in 03ef222). Unit e2 (${C}/verifier-fixcheck-e2-verdicts.md and its notes; its diff ${C}/verifier-fixcheck-e2-scratch/0005.diff) was cut short by a usage limit: it rebuilt ADR 0005 byte for byte but checked only two quotes. You check the rest of ADR 0005's changed lines (\`git -C /home/user/Shamble-Frontlines diff -U3 4374a1c HEAD -- docs/adr/0005-content-abilities-assets.md\`). You edit no doc; never commit, push or fetch.

For every changed line: (1) every quote of an answer is word for word in the inputs file, attributed as the verdict's condition 2 marks it; (2) no over-reading: anything beyond the words is one of the verdict's rulings, marked "(the advisor's reading, 2026-11-03)" and faithful to it; (3) each ruling applied: OQ-0005-1 (CD-2's whole-set check fails when an id of a presented kind has no client entry; the presented kinds named, from AP-2, CD-13 and AB-3, with game-designer's confirmation of the ability kinds; the kinds with nothing to present listed by name; a bite on an existing CD-2 row, not a new row), OQ-0005-AB12 (concepts accepted; specific claims listed for a later check), OQ-0005-AB13 ("built from data" includes a new reviewed generic operation; "needed often" and "many fail" no longer triggers in the Rollback, "Revisit when", the frame's table and Appendix A; option (b) stays unscheduled), LOOK-TEST-INPUTS (Blender for the test models and the research part's export; LOOK_V2 not edited; ASSET-RESEARCH's export part before the look test), SPIKE-ORDER, the header (condition 4); (4) the rules: no GLOSSARY "Retire (exact)" phrase (\`python3 ${C}/scripts/glossary_check_drafts.py\`), no sentence directly under a table, PM rows of 7 cells, every table row its header's cell count, new prose lines at most 120 characters, options kept; (5) consistency inside ADR 0005 between its sections (the frame, content data, the ability section, the asset pipeline, the GPU rules, the pass marks).

Write ${C}/verifier-fixcheck-e2b-verdicts.md as you go (a claims table, "Findings" ranked with the exact current text, what the words or ruling say, the smallest fix and the author by section). Notes: ${C}/verifier-fixcheck-e2b-notes.md (look there first; if they end with DONE, answer from them). Scratch only in ${C}/verifier-fixcheck-e2b-scratch/. Take the time the work needs: finish by 20:30 Amsterdam.

${SAVE}

Return the structured result: PASS, PASS WITH NOTES or FAIL; claims judged and confirmed; one check line per task; every finding; anything not read; your verdicts file's path.`,
  { label: 'fixcheck-e2b: ADR 0005', phase: 'Verify', agentType: 'verifier', effort: 'high', schema: UNIT_SCHEMA })
let skeptics = []
if (u) {
  const toCheck = u.findings.filter(f => ['wrong', 'unsupported', 'misquote', 'misattributed', 'over-read'].includes(f.kind))
  log(`e2b: ${u.verdict}; ${u.confirmed}/${u.claims}; ${u.findings.length} findings, ${toCheck.length} to the skeptics`)
  phase('Skeptics')
  skeptics = (await parallel(toCheck.map((f, i) => () => agent(`You are a fresh verifier acting as a skeptic for the v2 decision drafts of SHAMBLE FRONTLINES (branch claude/laughing-lovelace-ox61se; the docs on disk). Try to refute this verdict of fixcheck-e unit e2b. Read the text, Mustafa's words (${C}/inputs/STOP-ANSWERS-mustafa-2026-11-03.md) and the advisor's verdict (${C}/advisor-verdict-r4.md) yourself. Decide: upheld, partly (give the corrected fix) or refuted (quote what supports the text). You edit no doc, never commit or push, and fetch nothing.

The verdict: ${JSON.stringify(f)}

Write your result to ${C}/verifier-skeptic-8-e2b-${i + 1}.md as you go (notes in the same file; if it ends with DONE, answer from it). Scratch only in ${C}/verifier-skeptic-8-scratch/.

${SAVE}

Return the structured result: the finding's id, upheld, partly or refuted, your evidence, the corrected smallest fix, and your file's path.`, { label: `skeptic e2b: ${f.id}`, phase: 'Skeptics', agentType: 'verifier', effort: 'high', schema: SKEPTIC_SCHEMA })))).filter(Boolean)
}
return { unit: u, skeptics }
