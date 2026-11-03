// Workflow script (verbatim, copied at launch, run wf_edc68814-282, 2026-11-03 about 13:39 Amsterdam): test-engineer, effort xhigh, model: the session's (Opus 5.5)
export const meta = {
  name: 'v2-drafts-stop-te-0002',
  description: "test-engineer's follow-up: ADR 0002's pass marks after the stop's answers (the 60 Hz line, the boundary spike's stand-in clauses), from engine-architect's NOTEs H23-2 and O18-5",
  phases: [ { title: 'Write', detail: 'test-engineer, exact FIX or NOTE blocks for ADR 0002\'s pass marks' } ],
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
const SCHEMA = { type: 'object', properties: {
  blocks: { type: 'array', items: { type: 'object', properties: { id: { type: 'string' }, kind: { type: 'string', enum: ['FIX', 'NOTE'] }, summary: { type: 'string' } }, required: ['id', 'kind', 'summary'] } },
  checks: { type: 'string' }, fixes_file: { type: 'string' } },
  required: ['blocks', 'checks', 'fixes_file'] }
phase('Write')
const r = await agent(`You are test-engineer, author of the "Pass marks" sections of the v2 decision drafts of SHAMBLE FRONTLINES (branch claude/laughing-lovelace-ox61se; the docs on disk; HEAD has the stop's answers recorded, commit 03ef222). Your earlier unit this afternoon (${C}/stop-test-engineer-fixes.md, applied) covered ADR 0001, 0003, 0004, 0005 and 0006; ADR 0002's "Pass marks" section (docs/adr/0002-code-structure-build-toolchain.md, from "### Pass marks" to "## Migration plan") was left out of its file list. engine-architect's NOTEs found two places there (${C}/stop-engine-architect-a-fixes.md, NOTE H23-2 and NOTE O18-5):
- H23-2: "No mark here is set at 60 Hz: TICK-30's baseline is 30 Hz, and the 60 Hz margin is ADR 0001's, measured in one run of the netcode proof test." By TICK-60-SETUP (the Advisor, 2026-11-03: "(a), as an early signal only. The final 60 Hz call is measured again with the real sim at slice 6.") and the advisor's ruling, the wave-1 run is an early signal and slice 6's run decides.
- O18-5: PM-0002-28 ("the sim core spike's sim in this Worker once it exists", "with the stand-in sim: reported only") and the BX-P key: by SPIKE-ORDER (Mustafa, 2026-11-03: wave 1 runs one heavy spike at a time, the netcode proof test, then the sim core spike, then the 60 Hz run, then the boundary spike) and the advisor's ruling ("The boundary spike uses the sim core spike's sim"), the boundary spike now runs after the sim core spike, so its sim exists. engine-architect marks the clauses as optional to drop.

Read: Mustafa's words ${C}/inputs/STOP-ANSWERS-mustafa-2026-11-03.md; the advisor's verdict ${C}/advisor-verdict-r4.md (its conditions 2 and 6 and the rulings TICK-60-SETUP and SPIKE-ORDER); ADR 0002 as it now stands (part 12 and the migration steps engine-architect just changed). For each place: if the answer changes it, write one exact FIX block (quote the words exactly; attribute them as condition 2 says; mark a reading from the verdict "(the advisor's reading, 2026-11-03)"); keep pass-mark changes minimal (condition 6: no new row); a PM row keeps its 7 cells. If it needs nothing, write a NOTE with the reason. Also scan the rest of ADR 0002's pass marks for any other line the stop's answers make wrong (the 60 Hz call, the spike order, P2's values, the machines) and handle it the same way.

Write the blocks into ${C}/stop-test-engineer-0002-fixes.md as you go, every BEFORE copied by script from the file on disk, whole lines, occurring exactly once:
### FIX <id>
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
<the exact current text>
AFTER:
<the new text>
END
(or ### NOTE <id> … END). Check: \`python3 ${C}/scripts/apply_fixes.py ${C}/stop-test-engineer-0002-fixes.md <scratch>/docs/adr/0002-code-structure-build-toolchain.md --dry-run\` on a copy; no GLOSSARY "Retire (exact)" phrase; prose lines at most 120 characters. Notes: ${C}/stop-test-engineer-0002-notes.md (look there first; if they end with DONE, answer from them). Scratch only in ${C}/stop-test-engineer-0002-scratch/. Fetch nothing; edit no doc; never commit or push. Finish by 16:30 Amsterdam.

${SAVE}

Return the structured result: each block (id, FIX or NOTE, summary), your checks, and your fixes file's path.`,
  { label: 'test-engineer: ADR 0002 pass marks', phase: 'Write', agentType: 'test-engineer', effort: 'xhigh', schema: SCHEMA })
return r
