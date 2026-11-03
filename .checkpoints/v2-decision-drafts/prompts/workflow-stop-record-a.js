// Workflow script (verbatim, copied at launch, run wf_c88e880c-a17, 2026-11-03 about 13:04 Amsterdam): netcode-architect, test-engineer, rust-reviewer, game-designer; effort xhigh; model: the session's (Opus 5.5)
export const meta = {
  name: 'v2-drafts-stop-record-a',
  description: "Record the stop's answers, part A: netcode-architect (ADR 0001), test-engineer (every pass mark and Brave checklist), rust-reviewer (ADR 0004), game-designer (ADR 0005's ability section), each writing exact FIX or NOTE blocks for its own map rows",
  phases: [ { title: 'Write', detail: 'four authors, each on its own text, from the landing map and the advisor\'s rulings' } ],
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
  rows: { type: 'array', items: { type: 'object', properties: { id: { type: 'string' }, disposition: { type: 'string', enum: ['FIX', 'NO CHANGE'] }, blocks: { type: 'string' }, note: { type: 'string' } }, required: ['id', 'disposition', 'blocks', 'note'] } },
  fix_blocks: { type: 'integer' }, checks: { type: 'string' }, fixes_file: { type: 'string' } },
  required: ['rows', 'fix_blocks', 'checks', 'fixes_file'] }

const common = (unit, who, files) => `You are ${who} in the v2 decision drafts of SHAMBLE FRONTLINES (branch claude/laughing-lovelace-ox61se; the docs on disk; base main 9e7db28). Mustafa and the Advisor answered the stop on 2026-11-03. Record their answers in your own text.

Read first: their words, verbatim: ${C}/inputs/STOP-ANSWERS-mustafa-2026-11-03.md; the advisor's verdict on the plan, whose conditions and rulings you follow exactly: ${C}/advisor-verdict-r4.md; the landing map, whose rows are your work list: ${C}/stop-answers-map.md (each row: id, place, author, current text, the change asked); the plan: ${C}/plan-r4-stop-answers.md.

Your files: ${files}. Every map row below gets a disposition from you: a FIX block for the places in your own text, or a NOTE "no change: <reason>" (the advisor's ruling, or why the place needs nothing). Name each block by its map row id ("### FIX H5", "### FIX H5-2" for a second place, "### NOTE X5"), so fixcheck-e can check every row. A row that names another author too: handle only your own place in it.

Rules (the verdict's conditions 1, 2, 4 and the plan's rules):
- Record only what the words say. Quotes are word for word from the inputs file. Attribution follows the message's marks (condition 2): Mustafa's answers SPIKE-ORDER, TICK-60-HEADROOM, REWIND-CAP, SPIKE-MACHINES, LOOK-TEST-INPUTS, TOY-BOX, LATER; the Advisor's TICK-60-SETUP, NET-PROFILES, OQ-0002-8, OQ-0005-1, OQ-0001-14, ROOM-CRASH, OQ-0003-20, OQ-0005-AB12 and AB13; OQ-0003-17 the Advisor's, confirmed by Mustafa. A fully answered open question reads "Answered by Mustafa (2026-11-03): …" or "Answered by the Advisor (2026-11-03): …"; a partly answered one "Answered in part by … (2026-11-03): …; still open: …, decided by …". A reading taken from the advisor's verdict, not from the words, is written "(the advisor's reading, 2026-11-03)".
- Keep each question's options as they were; "proposed" becomes "decided" only where the answer settles it.
- The header lines (condition 4, your draft only): "**Status:** Proposed" stays; "**Signed off by Mustafa:**" reads "as the basis for the spikes (2026-11-03, SPIKE-ORDER (b)); not yet accepted"; "**Reviewed by:**" may name the step-10 reviews (2026-11-02) if it says "(filled in at review)".
- The one new pass mark is PM-0001-63 (test-engineer writes it in ADR 0001's 60 Hz group: rooms per VM at 60 Hz with the goal-budget check); cite it by that id.
- Edit only your own text; a needed change in another author's text is a NOTE naming that author. No GLOSSARY "Retire (exact)" phrase (check with \`python3 ${C}/scripts/glossary_check_drafts.py\` in a scratch tree laid out as the repository, with your blocks applied); no sentence directly under a table; table rows keep their cell count (PM rows 7 cells); new or rewrapped prose lines at most 120 characters. README's line pointers are being dropped (POINTERS (b)), so your line count may change.

Write the blocks into ${C}/${unit}-fixes.md as you go, every BEFORE copied by script from the file on disk, whole lines, occurring exactly once:
### FIX <row id>
FILE: <docs path>
BEFORE:
<the exact current text>
AFTER:
<the new text>
END
### NOTE <row id>
no change: <reason>
END
Check that the blocks apply: copy each doc into your scratch folder under its repository path and run \`python3 ${C}/scripts/apply_fixes.py ${C}/${unit}-fixes.md <scratch>/<docs path> --dry-run\` for each (the script applies a block only to a path that ends with its FILE; blocks apply in file order). Notes: ${C}/${unit}-notes.md (look there first; if they end with DONE, answer from them). Scratch only in ${C}/${unit}-scratch/. Fetch nothing; edit no doc; never commit or push. Finish by 16:30 Amsterdam.

${SAVE}

Return the structured result: one entry per map row of yours (id, FIX or NO CHANGE, the block ids, a one-line note), the number of FIX blocks, your checks (dry runs, glossary, cells, line lengths), and your fixes file's path.`

const UNITS = [
  { unit: 'stop-netcode-architect', who: 'netcode-architect, author of ADR 0001 (docs/adr/0001-multiplayer-netcode.md) except its "Pass marks" section and the netcode checklist', files: 'docs/adr/0001-multiplayer-netcode.md (your text)',
    rows: 'RC1 RC2 RC3 RC5 H1 H2 H3 H4 H5 H6 H10 H11 S1 S2 S3 S4 S5 S8 S9 N1 N2 N3 RW1 RW2 RW3 RW4 RW5 RW6 RW7 RW8 RW9 RW13 O15 (your header) O16 O17 M14 M15 X1 X2 X5 (your need lines)' },
  { unit: 'stop-test-engineer', who: 'test-engineer, author of every "Pass marks" section and Brave checklist in ADR 0001 to 0006 (the netcode checklist, the look checklist, the Feel Lab checklist and the pass-mark keys)', files: 'the pass-mark sections and checklists of docs/adr/0001-multiplayer-netcode.md, 0003-rendering.md, 0004-simulation-core.md, 0005-content-abilities-assets.md and 0006-infrastructure-operations.md',
    rows: 'RC4 H7 H8 H9 H14 H15 H17 S6 S7 S13 S16 N4 N5 N6 N9 N10 N11 N12 N13 RW10 RW11 RW12 M11 M12 M13 (your place) M16 M21 (your place) L12 T3 (your place) C8 F3 F4 Q3 Q4 B4 (your place). Also, by the verdict: the new row PM-0001-63 (condition 6 and the TICK-60-HEADROOM ruling: the goal-mode fleet at the rooms per VM measured at 60 Hz, by ADR 0006 part 8\'s formula, same machine, N = 48, costs no more than the goal budget as it stands at the call; in the wave-1 early signal the cost is reported beside part 8\'s provisional figures and decides nothing; at slice 6 the figure OQ-0006-19 fixes), with every list or count of the 60 Hz group that names its rows; OQ-0005-1\'s rule as a bite on an existing CD-2 whole-set row (PM-0005-4 or -23), the presented kinds as engine-architect lists them in ADR 0005 (name the kinds from AP-2, CD-13 and AB-3 yourself if its text is not applied yet, and say so in a NOTE)' },
  { unit: 'stop-rust-reviewer', who: 'rust-reviewer, author of ADR 0004 (docs/adr/0004-simulation-core.md) except its "Pass marks" section', files: 'docs/adr/0004-simulation-core.md (your text)',
    rows: 'RC6 H23 (your 0004 places) S10 S11 S12 S14 O15 (your header) O20 LA3 (your 0004 entries: the drafts\' own entries for the LATER questions are not edited, by the ruling) X5 (your need lines, if any)' },
  { unit: 'stop-game-designer', who: 'game-designer, author of ADR 0005\'s section "The ability and effect system" (docs/adr/0005-content-abilities-assets.md, from its heading to the line before "## The asset pipeline (engine-architect)")', files: 'your section of docs/adr/0005-content-abilities-assets.md',
    rows: 'C9 A1 A2 A3 B1 B2 LA3 (your section\'s entries: not edited, by the ruling). By the OQ-0005-1 ruling, also confirm (a NOTE) which ability kinds have something to present and so need a client entry' },
]

phase('Write')
const out = await parallel(UNITS.map(u => () => agent(`${common(u.unit, u.who, u.files)}

Your map rows: ${u.rows}.`, { label: u.unit, phase: 'Write', agentType: u.unit.replace('stop-', ''), effort: 'xhigh', schema: SCHEMA })))
return { netcode: out[0], test: out[1], rust: out[2], game: out[3] }
