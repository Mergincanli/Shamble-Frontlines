// Workflow script (verbatim, copied after launch, run wf_fba530ed-2f3, 2026-11-03 about 17:25 Amsterdam): verifier and skeptics, effort high, model: the session's (Opus 5.5)
export const meta = {
  name: 'v2-drafts-fixcheck-e3b',
  description: "fixcheck-e3's gaps after the cut: SPIKE_PLAN's changed lines from item 5 on, the cross-document consistency over all nine documents, and SPIKE_PLAN's count statement; a skeptic per wrong verdict",
  phases: [ { title: 'Verify', detail: 'one verifier on what e3 could not read' }, { title: 'Skeptics', detail: 'a fresh verifier per wrong or unsupported verdict' } ],
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
const u = await agent(`You are a fresh verifier, fixcheck-e unit e3b, for the v2 decision drafts of SHAMBLE FRONTLINES (branch claude/laughing-lovelace-ox61se; the docs on disk; \`git -C /home/user/Shamble-Frontlines log -1 --format=%h\` names HEAD). Mustafa and the Advisor answered the stop on 2026-11-03 (their words, verbatim: ${C}/inputs/STOP-ANSWERS-mustafa-2026-11-03.md); the advisor approved a plan to record them with conditions and rulings (${C}/advisor-verdict-r4.md); the authors' blocks are in ${C}/stop-*-fixes.md (applied in 03ef222 and 59d41e0). The docs before this round: 4374a1c. Unit e3 (${C}/verifier-fixcheck-e3-verdicts.md and its notes) was cut short by a usage limit and left three things unread; you close exactly those. You edit no doc; never commit, push or fetch.

1. **SPIKE_PLAN from item 5 to its end** (\`git -C /home/user/Shamble-Frontlines diff -U3 4374a1c HEAD -- docs/SPIKE_PLAN.md\`, the hunks from item 5's heading on): every quote of an answer word for word in the inputs file; attribution as the verdict's condition 2 marks it (Mustafa's: SPIKE-ORDER, TICK-60-HEADROOM, REWIND-CAP, SPIKE-MACHINES, LOOK-TEST-INPUTS, TOY-BOX, LATER; the Advisor's: TICK-60-SETUP, NET-PROFILES, OQ-0002-8, OQ-0005-1, OQ-0001-14, ROOM-CRASH, OQ-0003-20, OQ-0005-AB12, AB13; OQ-0003-17 the Advisor's, confirmed by Mustafa); no over-reading (anything beyond the words is one of the verdict's rulings, marked "(the advisor's reading, 2026-11-03)" and faithful to it); each ruling applied (for example: the asset research's export part between two wave-1 spikes, before Mustafa makes the models; the Feel Lab weighing of the cap gone; OQ-0003-17's and OQ-0003-20's readings; OQ-SPIKE-1).
2. **Consistency across all nine documents** (the six ADRs, README, SPIKE_PLAN, ENGINE_OVERVIEW): each of these reads the same everywhere it appears: P2's values (100 ms round trip, ±20 ms jitter, 2 % loss), the half-each-way split and 2 % loss each direction; the 60 Hz rule (the margin and the goal budget at the rooms per VM measured at 60 Hz, PM-0001-63; the wave-1 run an early signal; slice 6 deciding); the rewind cap's rule (p99 at P2, rounded up to a whole tick; no value yet); wave 1's order; the header wording (condition 4); the reference machines (OQ-0003-1 open in part, MP stands in for none); ROOM-CRASH's "marked crashed"; OQ-0005-1's presented kinds. List every place that disagrees, with both texts.
3. **SPIKE_PLAN's own count statements** (open questions, pass marks, spikes) against the script counts: 114 numbered, 98 open (12, 7, 17, 10, 33, 19), pass marks 63, 78, 47, 75, 46, 32 (341).

Write ${C}/verifier-fixcheck-e3b-verdicts.md as you go (a claims table, the consistency table, the counts, "Findings" ranked with the exact current text, what the words or ruling say, the smallest fix and the author). Notes: ${C}/verifier-fixcheck-e3b-notes.md (look there first; if they end with DONE, answer from them). Scratch only in ${C}/verifier-fixcheck-e3b-scratch/. Take the time the work needs: finish by 20:30 Amsterdam.

${SAVE}

Return the structured result: PASS, PASS WITH NOTES or FAIL; claims judged and confirmed; one check line per task; every finding; anything not read; your verdicts file's path.`,
  { label: 'fixcheck-e3b: SPIKE_PLAN item 5 on, consistency, counts', phase: 'Verify', agentType: 'verifier', effort: 'high', schema: UNIT_SCHEMA })
let skeptics = []
if (u) {
  const toCheck = u.findings.filter(f => ['wrong', 'unsupported', 'misquote', 'misattributed', 'over-read'].includes(f.kind))
  log(`e3b: ${u.verdict}; ${u.confirmed}/${u.claims}; ${u.findings.length} findings, ${toCheck.length} to the skeptics`)
  phase('Skeptics')
  skeptics = (await parallel(toCheck.map((f, i) => () => agent(`You are a fresh verifier acting as a skeptic for the v2 decision drafts of SHAMBLE FRONTLINES (branch claude/laughing-lovelace-ox61se; the docs on disk). Try to refute this verdict of fixcheck-e unit e3b. Read the text, Mustafa's words (${C}/inputs/STOP-ANSWERS-mustafa-2026-11-03.md) and the advisor's verdict (${C}/advisor-verdict-r4.md) yourself. Decide: upheld, partly (give the corrected fix) or refuted (quote what supports the text). You edit no doc, never commit or push, and fetch nothing.

The verdict: ${JSON.stringify(f)}

Write your result to ${C}/verifier-skeptic-8-e3b-${i + 1}.md as you go (notes in the same file; if it ends with DONE, answer from it). Scratch only in ${C}/verifier-skeptic-8-scratch/.

${SAVE}

Return the structured result: the finding's id, upheld, partly or refuted, your evidence, the corrected smallest fix, and your file's path.`, { label: `skeptic e3b: ${f.id}`, phase: 'Skeptics', agentType: 'verifier', effort: 'high', schema: SKEPTIC_SCHEMA })))).filter(Boolean)
}
return { unit: u, skeptics }
