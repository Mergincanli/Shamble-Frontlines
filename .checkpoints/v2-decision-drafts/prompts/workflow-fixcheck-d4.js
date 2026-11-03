// Workflow script (verbatim, copied at launch, run wf_7d619266-6c2, 2026-11-03 about 09:08 Amsterdam): verifier fixcheck-d4 and skeptics, effort xhigh; docs-keeper, effort high; model: the session's (Opus 5.5)
export const meta = {
  name: 'v2-drafts-fixcheck-d4',
  description: "The last check: fixcheck-d4 on touch-up 4's ten blocks (ac252f8, bd68d59, fd8c6f8) and on every number and claim in the report and its appendix written this morning; a skeptic per wrong or unsupported verdict; then docs-keeper's last STATUS and ROADMAP touch",
  phases: [
    { title: 'Verify', detail: 'fixcheck-d4: the newest blocks, and the report against the verdict files and git' },
    { title: 'Skeptics', detail: 'a fresh verifier per wrong or unsupported verdict' },
    { title: 'Docs', detail: "docs-keeper's last touch to STATUS and ROADMAP" },
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
  block. Never redo a finished block, and never start the file again from scratch.
(The session adds, for every unit from now on: no binaries and no file over 1 MB in your scratch folder; keep the source and the few lines you quote.)`

const FINDING = { type: 'object', properties: {
  id: { type: 'string' }, severity: { type: 'string', enum: ['high', 'medium', 'low'] },
  kind: { type: 'string', enum: ['wrong', 'unsupported', 'citation off', 'inconsistent', 'count', 'rule', 'other'] },
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
const DK_SCHEMA = { type: 'object', properties: {
  changes: { type: 'array', items: { type: 'string' } }, repo_map_check: { type: 'string' } },
  required: ['changes', 'repo_map_check'] }

const d4 = `You are a fresh verifier, fixcheck-d4, the last check of the v2 decision drafts of SHAMBLE FRONTLINES (branch claude/laughing-lovelace-ox61se; the docs on disk; \`git -C /home/user/Shamble-Frontlines log -1 --format=%h\` names HEAD). You edit no source and no doc; you never commit or push; you fetch nothing (every source here is local).

## What to check
1. **Touch-up 4's ten blocks.** \`git -C /home/user/Shamble-Frontlines diff -U3 882cfa6 fd8c6f8 -- docs/adr docs/SPIKE_PLAN.md docs/ENGINE_OVERVIEW.md\` (commits ac252f8: README and ADR 0002, engine-architect; bd68d59: ADR 0004 part 7, rust-reviewer; fd8c6f8: PM-0003-2 and ADR 0001's netcode checklist, test-engineer). The blocks: ${C}/touchup4-engine-architect-fixes.md, ${C}/touchup4-rust-reviewer-fixes.md, ${C}/touchup4-test-engineer-fixes.md; the findings they answer: ${C}/verifier-fixcheck-d3b-verdicts.md, ${C}/verifier-fixcheck-d3c-verdicts.md, ${C}/verifier-fixcheck-d3e-verdicts.md (D3B-2, D3C-1 to D3C-4, D3E-1 to D3E-3). For every changed line: each claim and number against what it cites (README's new seam rows S5, S9, S10 against the decision map, ${C}/engine-architect-decision-map.md lines 207, 211, 212, and against the drafts as they now stand; the WebGPU wording against fixcheck-d3e's saved excerpts, ${C}/verifier-fixcheck-d3e-scratch/webgpu-excerpt*.txt; the 1.99.1 \`div_euclid\` claim against ${C}/verifier-fixcheck-d3e-scratch/rust-1.99.1-src-*.txt); confirmed, wrong (quote the source), unsupported, or citation off. Does each block resolve its finding and add nothing wrong? Are the re-wraps word for word (a word diff)? Rules: no GLOSSARY "Retire (exact)" phrase (\`python3 ${C}/scripts/glossary_check_drafts.py\`, from the repository root); no sentence directly under a table; PM rows of 7 cells; \`python3 ${C}/scripts/draft_pointers.py\` prints "off 0".
2. **The report and its appendix, every number and claim written on 2026-11-03.** docs/reports/2026-11-01-v2-decision-drafts.md (all of "Evidence" and "Decisions and deviations", and "What changed"'s counts) and docs/reports/2026-11-01-v2-decision-drafts-appendix.md (§2's 2026-11-03 entries, §5's touch-up row and the cuts line, §6 row 17, §7.1's rows from fixcheck-d0 on and the skeptics paragraph, §7.3's touch-up and deferred bullets): \`git -C /home/user/Shamble-Frontlines diff 449356c HEAD -- docs/reports/\` shows what changed. Check each against the verdict files (${C}/verifier-*-verdicts.md, ${C}/verifier-skeptic-*.md, ${C}/verifier-fixcheck-d3-critic.md), the fix files (${C}/*fixes*.md), git log, and the drafts at HEAD: block counts (901 in five rounds: 56, 167, 60, 53, 565; 63 in the touch-ups, by author), verdicts and their counts, the skeptic tally, the cut times, commit ids, and every "fixed in" or "deferred" claim. The placeholders {D4}, {CI} and {ENDCOMMITS} are the session's, filled after you: skip them.

## Write
${C}/verifier-fixcheck-d4-verdicts.md, as you go: the claims table (\`| # | doc:line | claim (short) | cited | verdict | where and the words | note |\`), the blocks table, "Findings" ranked (exact current text, what the source says, the smallest fix, the author: the session wrote the report and appendix). Notes: ${C}/verifier-fixcheck-d4-notes.md (look there first; if they end with DONE, answer from them). Scratch only in ${C}/verifier-fixcheck-d4-scratch/. Finish by 11:00 Amsterdam.

${SAVE}

## Return
The structured result: PASS, PASS WITH NOTES or FAIL; claims judged and confirmed; one check line per task; every finding; anything not read; your verdicts file's path.`

const skepticPrompt = (f, n) => `You are a fresh verifier acting as a skeptic for the v2 decision drafts of SHAMBLE FRONTLINES (branch claude/laughing-lovelace-ox61se; the docs on disk). The plan sends every "wrong" or "unsupported" verdict to a skeptic before its author acts on it. Try to refute this verdict of fixcheck-d4. Read the text and the source yourself; do not trust the verdict's quotes. Decide: upheld, partly (the verdict overstates, or its smallest fix is wrong: give the corrected fix), or refuted (quote what supports the text). You edit no doc, never commit or push, and fetch nothing.

The verdict: ${JSON.stringify(f)}

Write your result to ${C}/verifier-skeptic-7-d4-${n}.md as you go (notes in the same file; if it ends with DONE, answer from it). Scratch only in ${C}/verifier-skeptic-7-scratch/.

${SAVE}

Return the structured result: the finding's id, upheld, partly or refuted, your evidence, the corrected smallest fix (or "as given" or "none"), and your file's path.`

phase('Verify')
const unit = await agent(d4, { label: 'fixcheck-d4: the newest blocks and the report', phase: 'Verify', agentType: 'verifier', effort: 'xhigh', schema: UNIT_SCHEMA })
let skeptics = []
if (unit) {
  const toCheck = unit.findings.filter(f => f.kind === 'wrong' || f.kind === 'unsupported')
  log(`fixcheck-d4: ${unit.verdict}; ${unit.confirmed}/${unit.claims}; ${unit.findings.length} findings, ${toCheck.length} to the skeptics`)
  phase('Skeptics')
  skeptics = (await parallel(toCheck.map((f, i) => () => agent(skepticPrompt(f, i + 1),
    { label: `skeptic d4: ${f.id}`, phase: 'Skeptics', agentType: 'verifier', effort: 'xhigh', schema: SKEPTIC_SCHEMA })))).filter(Boolean)
} else { log('fixcheck-d4 returned nothing: the session reads its notes') }

phase('Docs')
const dk = await agent(`Read ${C}/prompts/docs-keeper-3.txt and follow it exactly: it is your task (docs-keeper, plan step 11, the last touch to docs/STATUS.md and docs/ROADMAP.md at the stop of v2-decision-drafts). Two facts it was written before: the re-checks are fixcheck-d3 (units a to e) and fixcheck-d4 (${unit ? 'verdict ' + unit.verdict : 'its verdict file: ' + C + '/verifier-fixcheck-d4-verdicts.md'}); the drafts' numbered open questions are 114, of which 111 are open (OQ-0003-14 is answered). Name the PR, not a SHA. Finish by 11:00 Amsterdam.`,
  { label: 'docs-keeper: STATUS, ROADMAP', phase: 'Docs', agentType: 'docs-keeper', effort: 'high', schema: DK_SCHEMA })
return { unit, skeptics, dk }
