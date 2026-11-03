// Workflow script (verbatim, copied at launch, run wf_7663bb31-6ca, 2026-11-03 about 12:34 Amsterdam): engine-architect mapper and the advisor, effort xhigh, model: the session's (Opus 5.5)
export const meta = {
  name: 'v2-drafts-stop-plan',
  description: "Plan revision 4 (record the stop's answers, close the change-set): a mapper lists every place in the nine documents each answer touches; then the advisor reviews the plan with the map",
  phases: [
    { title: 'Map', detail: 'engine-architect, read-only: every place each answer lands, by author' },
    { title: 'Review', detail: 'the advisor: APPROVE, REVISE or ESCALATE' },
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

const MAP_SCHEMA = { type: 'object', properties: {
  places: { type: 'array', items: { type: 'object', properties: {
    answer: { type: 'string' }, doc_line: { type: 'string' }, author: { type: 'string' },
    current: { type: 'string' }, change: { type: 'string' }, in_plan_map: { type: 'boolean' } },
    required: ['answer', 'doc_line', 'author', 'current', 'change', 'in_plan_map'] } },
  partial_answers: { type: 'array', items: { type: 'string' } },
  questions_for_the_advisor: { type: 'array', items: { type: 'string' } },
  file: { type: 'string' } },
  required: ['places', 'partial_answers', 'questions_for_the_advisor', 'file'] }
const VERDICT_SCHEMA = { type: 'object', properties: {
  verdict: { type: 'string', enum: ['APPROVE', 'APPROVE WITH CONDITIONS', 'REVISE', 'ESCALATE'] },
  conditions: { type: 'array', items: { type: 'string' } },
  rulings: { type: 'array', items: { type: 'object', properties: { code: { type: 'string' }, ruling: { type: 'string' } }, required: ['code', 'ruling'] } },
  escalations: { type: 'array', items: { type: 'string' } },
  file: { type: 'string' } },
  required: ['verdict', 'conditions', 'rulings', 'escalations', 'file'] }

phase('Map')
const map = await agent(`You are engine-architect, the synthesis author of the v2 decision drafts of SHAMBLE FRONTLINES (branch claude/laughing-lovelace-ox61se; the docs on disk; HEAD 4374a1c). Mustafa and the Advisor have answered the stop: their words, verbatim, are in ${C}/inputs/STOP-ANSWERS-mustafa-2026-11-03.md. The session's plan to record them is ${C}/plan-r4-stop-answers.md; its table "Where each answer lands" is a first guess. This is a read-only mapping unit: you edit no doc, never commit, push or fetch.

Find every place in the nine documents (docs/adr/0001-multiplayer-netcode.md, 0002-code-structure-build-toolchain.md, 0003-rendering.md, 0004-simulation-core.md, 0005-content-abilities-assets.md, 0006-infrastructure-operations.md, docs/adr/README.md, docs/SPIKE_PLAN.md, docs/ENGINE_OVERVIEW.md) that each answer touches: the open question it answers (by its bold id), every part, table row, pass mark (PM row), checklist step, README row and SPIKE_PLAN line that states the question as open, a "proposed" value the answer now fixes, a value the answer changes (for example P2's jitter and loss, the rewind cap, the 60 Hz rule, wave 1's order, the reference or test machines, the certificates and the domain, the Spline route, the content check), or a "to confirm" the answer confirms. Search by id, by code (SPIKE-ORDER, TICK-60-HEADROOM, TICK-60-SETUP, NET-PROFILES, REWIND-CAP, SPIKE-MACHINES, LOOK-TEST-INPUTS, TOY-BOX, ROOM-CRASH, HEAT-SOAK, DOMAIN, SPIKE-VM-ACCESS and the rest) and by content (the numbers and phrases each answer is about). For each place give doc:line, its author (the drafts' "Written by" lines and section headings say who wrote what: the "Pass marks" sections and the Brave checklists are test-engineer's; ADR 0005's sections: ability = game-designer, GPU rules = graphics-engineer, the rest = engine-architect), the current text (short), the change the answer asks for, and whether the plan's table already names it. Name each answer that settles only part of its question (what stays open, who decides). Note anything the plan should ask the advisor (for example a §8 glossary entry an answer contradicts, a settled input, LOOK_V2 §8's Spline flow against Mustafa's Blender route).

Write ${C}/stop-answers-map.md as you go (a table per answer). Notes: ${C}/stop-answers-map-notes.md (look there first; if they end with DONE, answer from them). Scratch only in ${C}/stop-answers-map-scratch/. Finish by 14:30 Amsterdam.

${SAVE}

Return the structured result: every place, the partial answers, your questions for the advisor, and your file's path.`,
  { label: 'engine-architect: the landing map', phase: 'Map', agentType: 'engine-architect', effort: 'xhigh', schema: MAP_SCHEMA })
if (!map) { log('the mapper returned nothing: the session reads its notes'); return { map: null } }
log(`map: ${map.places.length} places (${map.places.filter(p => !p.in_plan_map).length} not in the plan's table); ${map.partial_answers.length} partial answers`)

phase('Review')
const review = await agent(`You are the project's Advisor agent. Review a plan before any edit, as your agent file says (main's .claude/agents/advisor.md; this branch does not change it). The change-set is v2-decision-drafts (SHAMBLE FRONTLINES; branch claude/laughing-lovelace-ox61se standing in for cs/v2-decision-drafts; draft PR #33; base main 9e7db28; HEAD 4374a1c, CI green). It reached its stop: the six decision drafts, ENGINE_OVERVIEW, SPIKE_PLAN and the drafts' README, with the report docs/reports/2026-11-01-v2-decision-drafts.md and its appendix. Mustafa and the Advisor (Claude in a claude.ai Project) answered the stop today; Mustafa's message, verbatim: ${C}/inputs/STOP-ANSWERS-mustafa-2026-11-03.md. He asked: "record these, mark PR #33 ready, and once it's approved, squash-merge it so the 31 MB of scratch build output never reaches main's history. Then the netcode proof test as its own change-set, plan first."

The plan to review: ${C}/plan-r4-stop-answers.md (revision 4 of the change-set's plan; revisions 1–3 and your verdict of 2026-10-31 are in the report's appendix §3–§4). The landing map from engine-architect (every place each answer touches): ${C}/stop-answers-map.md; its summary: ${JSON.stringify({ places: map.places.length, not_in_plan: map.places.filter(p => !p.in_plan_map).map(p => p.answer + ' @ ' + p.doc_line), partial_answers: map.partial_answers, questions: map.questions_for_the_advisor })}.

Judge: does the plan record the answers faithfully and completely (with the map), keep each author to its own text, keep the merge rules of CLAUDE.md (a session merges only on Mustafa's own go in its chat for the PR's current head, given after the advisor approved that head; the squash he asks for), close the change-set by .claude/CHECKPOINTS.md §9, and avoid scope creep? Rule on its decisions (STATUS-FIELD, POINTERS) or escalate what is Mustafa's; answer the mapper's questions where they are yours. Verdict: APPROVE, APPROVE WITH CONDITIONS (list them), REVISE (what to fix) or ESCALATE (what Mustafa must decide). Write your verdict, verbatim as you would give it, to ${C}/advisor-verdict-r4.md (notes in the same file; if it ends with DONE, answer from it). You edit no doc and never commit or push.

${SAVE}

Return the structured result: the verdict, the conditions, each ruling (code and ruling), any escalation, and your file's path.`,
  { label: 'advisor: plan revision 4', phase: 'Review', agentType: 'advisor', effort: 'xhigh', schema: VERDICT_SCHEMA })
return { map, review }
