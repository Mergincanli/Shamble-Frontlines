// Workflow script (verbatim, copied at launch, run wf_d9bbfb18-41c, 2026-11-03 about 13:04 Amsterdam): engine-architect (two units), infra-engineer, graphics-engineer; effort xhigh; model: the session's (Opus 5.5)
export const meta = {
  name: 'v2-drafts-stop-record-b',
  description: "Record the stop's answers, part B: engine-architect (ADR 0002, ADR 0005's frame, content and asset sections, GLOSSARY §8's 60 Hz entry; and README, SPIKE_PLAN, ENGINE_OVERVIEW with the pointers last), infra-engineer (ADR 0006), graphics-engineer (ADR 0003, ADR 0005's GPU rules)",
  phases: [ { title: 'Write', detail: 'four units, each on its own text, from the landing map and the advisor\'s rulings' } ],
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
- The header lines (condition 4, your documents only): "**Status:** Proposed" stays; "**Signed off by Mustafa:**" reads "as the basis for the spikes (2026-11-03, SPIKE-ORDER (b)); not yet accepted"; "**Reviewed by:**" may name the step-10 reviews (2026-11-02) if it says "(filled in at review)".
- The one new pass mark is PM-0001-63 (test-engineer writes it in ADR 0001's 60 Hz group: rooms per VM at 60 Hz with the goal-budget check); cite it by that id. Counts after the answers (condition 6 and the COUNTS ruling): 114 numbered open questions, 98 open (a partly answered one stays open, marked "answered in part"; OQ-0001-5 stays open); pass marks 63, 78, 47, 75, 46 and 32 (341).
- Edit only your own text; a needed change in another author's text is a NOTE naming that author. No GLOSSARY "Retire (exact)" phrase (check with \`python3 ${C}/scripts/glossary_check_drafts.py\` in a scratch tree laid out as the repository, with your blocks applied); no sentence directly under a table; table rows keep their cell count; new or rewrapped prose lines at most 120 characters.

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
  { unit: 'stop-engine-architect-a', agentType: 'engine-architect', who: 'engine-architect, author of ADR 0002 (docs/adr/0002-code-structure-build-toolchain.md, except its "Pass marks" section), of ADR 0005\'s frame and its sections "Content data (engine-architect)" and "The asset pipeline (engine-architect)" (docs/adr/0005-content-abilities-assets.md), and of GLOSSARY §8 (docs/research/GLOSSARY.md from line 744 on, appended by this PR)', files: 'docs/adr/0002-code-structure-build-toolchain.md, docs/adr/0005-content-abilities-assets.md (your frame and sections, incl. its header lines), docs/research/GLOSSARY.md §8 (lines 744 on only: condition 3 amends "The 60 Hz margin" entry, lines 782–797, and lines 1–743 keep their bytes; check with `bash ' + C + '/scripts/write_guard.sh` after applying in your scratch tree if it can be pointed there, else compare the first 743 lines byte for byte)',
    rows: 'H23 (your 0002 place) H24 and S23 (GLOSSARY §8\'s 60 Hz entry: condition 3 replaces the NOTE-only rule) N8 O15 (the 0002 and 0005 headers) O18 O22 M22 L7 L9 E1 E2 E3 E4 E5 C1 C2 C3 C4 C5 C6 C7 A4 B3 B4 (your frame place) B5 X5 (your need lines). By the OQ-0005-1 ruling: list in ADR 0005 the presented kinds that need a client entry (from AP-2, CD-13 and AB-3; game-designer confirms the ability kinds) and the kinds with nothing to present, by name. M24 (GLOSSARY lines 707–712) stays a NOTE for the report' },
  { unit: 'stop-engine-architect-b', agentType: 'engine-architect', who: 'engine-architect, author of the synthesis (docs/adr/README.md, docs/SPIKE_PLAN.md, docs/ENGINE_OVERVIEW.md)', files: 'docs/adr/README.md, docs/SPIKE_PLAN.md, docs/ENGINE_OVERVIEW.md (incl. their header lines and README\'s Status column and lines 24–25, condition 4)',
    rows: 'RC9 RC10 H18 H19 H20 H21 H22 S17 S18 S19 S20 S21 S22 N14 N15 N16 RW14 RW15 RW16 O1 O2 O3 O4 O5 O6 O7 O8 O9 O10 O11 O12 O13 O14 O15 (ENGINE_OVERVIEW, SPIKE_PLAN and README headers) M17 M18 M19 M20 M23 L1 L2 L3 L4 L5 L6 T1 E6 C10 C11 X4 F5 F6 F7 Q5 Q6 A5 B6 B7 LA1 LA2 (README\'s design questions join the LATER session\'s agenda, by the LATER ruling). Also condition 1\'s corrections: TOY-BOX has no README row (a NOTE); ENGINE_OVERVIEW changes at :3–4, :15 and :117; README:47 names all three crates (room, client-endpoint, navbake). Also: README\'s sentence on review codes says the review files stay readable "in PR #33\'s commits" (the checkpoint folder leaves the branch before the PR is marked ready and the merge is a squash), not "the branch\'s history". LAST, condition 5 (POINTERS (b)): README\'s 95 pointers lose their line numbers ("(line 162 at 604a497)" included); each keeps its file link and names its OQ id, part or section; README:10\'s sentence about line numbers changes to say so. Write these pointer blocks last in your fixes file, each BEFORE taken from a scratch copy of README with your earlier README blocks already applied (blocks apply in file order); a script may generate them' },
  { unit: 'stop-infra-engineer', agentType: 'infra-engineer', who: 'infra-engineer, author of ADR 0006 (docs/adr/0006-infrastructure-operations.md) except its "Pass marks" section', files: 'docs/adr/0006-infrastructure-operations.md (your text)',
    rows: 'RC7 RC8 H12 H13 H16 H23 (your 0006 places) S15 O15 (your header) O21 M1 M2 M3 M4 M5 M6 M7 M8 M9 M10 M13 (your place) X3 X5 (your need lines) LA3 (your entries: not edited, by the ruling). Also condition 1: places for TICK-60-HEADROOM (part 8 and spike item 2 point to PM-0001-63; the "none runs at 60 Hz" lines stay; OQ-0006-19 and -10 stay open with a pointer) and for OQ-0006-5\'s development case (i) for the WebSocket leg, the issuance method left to the proof test\'s plan; ROOM-CRASH: part 12\'s stub record is "marked crashed" (one word with ADR 0001), kept under part 11\'s proposed retention, no new period' },
  { unit: 'stop-graphics-engineer', agentType: 'graphics-engineer', who: 'graphics-engineer, author of ADR 0003 (docs/adr/0003-rendering.md) except its "Pass marks" section and look checklist, and of ADR 0005\'s section "The asset pipeline: GPU rules (graphics-engineer)"', files: 'docs/adr/0003-rendering.md (your text) and your section of docs/adr/0005-content-abilities-assets.md',
    rows: 'O15 (the 0003 header) O19 M21 (your place: OQ-0003-1 answered only in part; "Test PC: my own" is MP and stands in for no ML, RD or MAC, by the SPIKE-MACHINES ruling; ML, RD and MAC still to be named, Mustafa\'s, escalated) L8 L10 L11 L13 (LOOK-TEST-INPUTS: Blender for the test models and the research part\'s export route; part 11\'s list of objects stays, each may be a blockout at Mustafa\'s bar; no checklist edit; LOOK_V2 not edited) T3 (your place) F1 F2 Q1 Q2 X5 (your need lines) LA3 (your entries: not edited, by the ruling)' },
]

phase('Write')
const out = await parallel(UNITS.map(u => () => agent(`${common(u.unit, u.who, u.files)}

Your map rows: ${u.rows}.`, { label: u.unit, phase: 'Write', agentType: u.agentType, effort: 'xhigh', schema: SCHEMA })))
return { eaA: out[0], eaB: out[1], infra: out[2], graphics: out[3] }
