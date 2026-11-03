// Workflow script (verbatim, copied at launch, run wf_6c0e9508-f3b, 2026-11-03 about 07:18 Amsterdam): verifier agentType, effort xhigh, model: the session's (Opus 5.5); no fetching
export const meta = {
  name: 'v2-drafts-fixcheck-d3a',
  description: 'fixcheck-d3 unit a: re-verify the touch-ups to ADR 0001, 0002 and 0006 and the four owner-checked verdicts; a fresh skeptic tries to refute every wrong or unsupported verdict',
  phases: [
    { title: 'Verify', detail: 'one verifier on the ADR 0001, 0002 and 0006 touch-ups' },
    { title: 'Skeptics', detail: 'a fresh verifier per wrong or unsupported verdict, trying to refute it' },
  ],
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
  block. Never redo a finished block, and never start the file again from scratch.`

const FINDING = { type: 'object', properties: {
  id: { type: 'string' }, severity: { type: 'string', enum: ['high', 'medium', 'low'] },
  kind: { type: 'string', enum: ['wrong', 'unsupported', 'citation off', 'inconsistent', 'lost or doubled line', 'rule', 'other'] },
  doc_line: { type: 'string' }, author: { type: 'string' }, current_text: { type: 'string' },
  source_says: { type: 'string' }, smallest_fix: { type: 'string' } },
  required: ['id', 'severity', 'kind', 'doc_line', 'author', 'current_text', 'source_says', 'smallest_fix'] }
const UNIT_SCHEMA = { type: 'object', properties: {
  verdict: { type: 'string', enum: ['PASS', 'PASS WITH NOTES', 'FAIL'] },
  claims: { type: 'integer' }, confirmed: { type: 'integer' },
  blocks_judged: { type: 'integer' }, blocks_with_problem: { type: 'integer' },
  owner_checks: { type: 'array', items: { type: 'object', properties: { id: { type: 'string' }, verdict_holds: { type: 'boolean' }, fix_right: { type: 'boolean' }, note: { type: 'string' } }, required: ['id', 'verdict_holds', 'fix_right', 'note'] } },
  findings: { type: 'array', items: FINDING },
  file: { type: 'string' } },
  required: ['verdict', 'claims', 'confirmed', 'blocks_judged', 'blocks_with_problem', 'owner_checks', 'findings', 'file'] }
const SKEPTIC_SCHEMA = { type: 'object', properties: {
  id: { type: 'string' }, outcome: { type: 'string', enum: ['upheld', 'partly', 'refuted'] },
  evidence: { type: 'string' }, corrected_fix: { type: 'string' }, file: { type: 'string' } },
  required: ['id', 'outcome', 'evidence', 'corrected_fix', 'file'] }

const unitPrompt = `You are a fresh verifier, fixcheck-d3 unit a, for the v2 decision drafts of SHAMBLE FRONTLINES (branch claude/laughing-lovelace-ox61se; the docs on disk; \`git -C /home/user/Shamble-Frontlines log -1 --format=%h\` names HEAD). You re-verify the touch-ups made to ADR 0001 (docs/adr/0001-multiplayer-netcode.md), ADR 0002 (docs/adr/0002-code-structure-build-toolchain.md) and ADR 0006 (docs/adr/0006-infrastructure-operations.md) after the final re-check fixcheck-d1, and you judge four "wrong" verdicts that their owners checked instead of a fresh skeptic, for time. You edit no source and no doc; you never commit or push; you fetch no web page (a claim that rests on a web page fixcheck-d0 did not read stays "unverified": say so).

## What to check
1. **The changed text.** \`git -C /home/user/Shamble-Frontlines diff -U3 449356c HEAD -- docs/adr/0001-multiplayer-netcode.md docs/adr/0002-code-structure-build-toolchain.md docs/adr/0006-infrastructure-operations.md\`. fixcheck-d1 checked these three at 604a497; confirm first that none changed between 604a497 and 449356c. The commits: cd1fc65 (ADR 0002, d1 F9), 7858762 (ADR 0001 and 0006: d1 F1, F2, F10, F11, F13, F14), 750d46d (ADR 0001 and 0006: d1 F6, F7, F12). For every added or changed line, judge each claim and number against what it cites (the research at 9e7db28 in docs/research/r1-…r6-*.md, the other drafts, docs/LOOP_V2.md, docs/VISION_V2.md, docs/LOOK_V2.md, docs/research/LEGACY_INVENTORY or the legacy docs): confirmed, wrong (quote the source), unsupported, or citation off (give the right one). Recompute every piece of arithmetic.
2. **Did each block do what its finding asked?** The blocks: ${C}/touchup-netcode-architect-fixes.md, ${C}/touchup-infra-engineer-fixes.md, ${C}/touchup-engine-architect-0002-fixes.md, and the ADR 0001 and 0006 blocks of ${C}/touchup-test-engineer-fixes.md (F7, F6, F12). The findings: ${C}/verifier-fixcheck-d1-verdicts.md §10. For each block: does its AFTER resolve its finding, and does it add anything new that is wrong? For each NOTE: is its reason sound? For d1's findings no block fixed: is each listed as deferred, with a reason, in docs/reports/2026-11-01-v2-decision-drafts-appendix.md §7.3?
3. **The four owner-checked verdicts.** fixcheck-d0 row 9 (${C}/verifier-fixcheck-d0-verdicts.md; ADR 0004's precision line), and fixcheck-d1 F1, F2 and F10. For each: read the original verdict, the owner's own check (the touch-up notes: ${C}/touchup-*-notes.md, and the owners' fixes files), and the source itself (for d0 row 9, the excerpt saved in the d0 scratch folder). Say whether the verdict holds and whether the fix that followed is right, as a skeptic would.
4. **The rules, on the changed text only** (the report's appendix §4, "Rules every author follows" 1–12): a number with no source; an option labelled "not researched" proposed as the decision; a GLOSSARY "Retire (exact)" phrase (\`python3 ${C}/scripts/glossary_check_drafts.py\`, run from the repository root); anything decided about the look beyond docs/LOOK_V2.md; a settled input reopened; a sentence directly under a table.

## Write
${C}/verifier-fixcheck-d3a-verdicts.md, as you go: the claims table (\`| # | doc:line | claim (short) | cited | verdict | where and the words | note |\`); the blocks table (\`| block | finding | resolved? | new problem |\`); the four owner-checked verdicts; "Findings", ranked, each with the exact current text, what the source says, the smallest fix and the text's author. Notes: ${C}/verifier-fixcheck-d3a-notes.md (look there first; if they end with DONE, answer from them; start the file with one line naming your task). Scratch only in ${C}/verifier-fixcheck-d3a-scratch/. Finish by 09:30 Amsterdam.

${SAVE}

## Return
The structured result: PASS, PASS WITH NOTES or FAIL; claims judged and confirmed; blocks judged and blocks with a problem; the four owner checks; every finding (wrong, unsupported, citation off, inconsistent, a rule) with its exact text, what the source says, the smallest fix and the author; your verdicts file's path.`

const skepticPrompt = (f, n) => `You are a fresh verifier acting as a skeptic for the v2 decision drafts of SHAMBLE FRONTLINES (branch claude/laughing-lovelace-ox61se; the docs on disk). The plan sends every "wrong" or "unsupported" verdict to a skeptic before its author acts on it. Your job: try to refute this verdict of fixcheck-d3 unit a. Read the draft text and the source yourself; do not trust the verdict's quotes. Decide: upheld (the text is wrong or unsupported as stated), partly (the verdict overstates, or its smallest fix is wrong: give the corrected fix), or refuted (the text holds: quote what supports it). You edit no doc, never commit or push, and fetch no web page.

The verdict: ${JSON.stringify(f)}

Write your result to ${C}/verifier-skeptic-6-d3a-${n}.md as you go (the verdict, the text you read with doc:line, the source's words with file:line, your outcome and why). Notes in the same file; if it ends with DONE, answer from it. Scratch only in ${C}/verifier-skeptic-6-scratch/.

${SAVE}

Return the structured result: the finding's id, upheld, partly or refuted, your evidence (doc:line and source:line with the words), the corrected smallest fix (or "as given" or "none"), and your file's path.`

phase('Verify')
const unit = await agent(unitPrompt, { label: 'fixcheck-d3a: ADR 0001, 0002, 0006', phase: 'Verify', agentType: 'verifier', effort: 'xhigh', schema: UNIT_SCHEMA })
if (!unit) { log('unit a returned nothing: the session reads its notes'); return { unit: null } }
const toCheck = unit.findings.filter(f => f.kind === 'wrong' || f.kind === 'unsupported')
log(`unit a: ${unit.verdict}; ${unit.confirmed}/${unit.claims} confirmed; ${unit.findings.length} findings, ${toCheck.length} to the skeptics`)
phase('Skeptics')
const skeptics = await parallel(toCheck.map((f, i) => () =>
  agent(skepticPrompt(f, i + 1), { label: `skeptic: ${f.id}`, phase: 'Skeptics', agentType: 'verifier', effort: 'xhigh', schema: SKEPTIC_SCHEMA })))
return { unit, skeptics: skeptics.filter(Boolean) }
