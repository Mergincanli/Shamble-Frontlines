export const meta = {
  name: 'v2-drafts-r6-round-b',
  description: "Plan revision 6's round, part B: engine-architect b (README, SPIKE_PLAN, ENGINE_OVERVIEW), rust-reviewer (ADR 0004 part 7), engine-architect a (ADR 0002 and 0005), each writing exact FIX or NOTE blocks for its own text",
  phases: [ { title: 'Write', detail: 'three authors on their own text, by the advisor\'s conditions on plan revision 6' } ],
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
  blocks: { type: 'array', items: { type: 'object', properties: { id: { type: 'string' }, file: { type: 'string' }, kind: { type: 'string', enum: ['FIX', 'NOTE'] }, summary: { type: 'string' } }, required: ['id', 'file', 'kind', 'summary'] } },
  checks: { type: 'string' }, fixes_file: { type: 'string' } },
  required: ['blocks', 'checks', 'fixes_file'] }
const common = (unit, who) => `You are ${who} in the v2 decision drafts of SHAMBLE FRONTLINES (branch claude/laughing-lovelace-ox61se; the docs on disk; base main 9e7db28). This is the round of plan revision 6 (${C}/plan-r6-hetzner-check.md, with its addendum): Mustafa's Hetzner console check, his correction and his answers to the session's two questions. The advisor approved it with conditions: ${C}/advisor-verdict-r6.md (read its Rulings and Conditions first and follow them exactly). Mustafa's words, verbatim: ${C}/inputs/HETZNER-CHECK-mustafa-2026-11-03.md; ${C}/inputs/HETZNER-CORRECTION-mustafa-2026-11-03.md (its screenshot description is the session's, never his words; his first sentence there is not quoted in the drafts, condition 2); ${C}/inputs/HETZNER-ANSWERS-mustafa-2026-11-03.md (the option texts are the session's wording, which he chose: label them so, never as his words); and his earlier ones in the same folder: STOP-ANSWERS-mustafa-2026-11-03.md, DESIGN-ANSWERS-mustafa-2026-11-03.md, ANSWERS-2-mustafa-2026-11-03.md. The work list by script (condition 1): ${C}/r6-worklist.md; every line in your section needs a disposition from you: a FIX block, or a NOTE "no change: <reason>" (one NOTE may cover several lines if it names each). After this round there is no re-check round (Mustafa: "No further re-check rounds; anything left goes into the PR as notes"), so make every block exact and complete the first time.

Rules: quotes word for word from the six inputs files, attributed as their marks say; a reading from the advisor's verdicts written "(the advisor's reading, 2026-11-03)"; prices only R6-HOST-F15's, excluding VAT (CCX23 86.49 EUR a month, 0.1386 an hour; CCX13 43.49, 0.0697; CAX21 10.99, 0.0176; CX33 8.99, 0.0144), his "about €0.020/h incl. VAT" only inside his quote, no VAT rate, never the screenshot's figures; "Falkenstein" or \`fsn1\` only in his words about what his check found there and in OQ-0006-2's options as the record; no live "nbg1 or hel1" and no "picked by a round-trip measurement"; the CCX13's code is DS, never "the development server" alone (ADR 0005 and ADR 0002 use that name for the local server); edit only your own text (a change in another author's text is a NOTE naming that author); keep options; no GLOSSARY "Retire (exact)" phrase (check with \`python3 ${C}/scripts/glossary_check_drafts.py\` in a scratch tree laid out as the repository with your blocks applied); no sentence directly under a table; table rows keep their cell count; prose lines at most 120 characters; no pass-mark row added or removed (the total stays 341); no new open question (115 numbered stays).

Write the blocks into ${C}/${unit}-fixes.md as you go, every BEFORE copied by script from the file on disk, whole lines, occurring exactly once:
### FIX <id>
FILE: <docs path>
BEFORE:
<the exact current text>
AFTER:
<the new text>
END
or ### NOTE <id> / no change: <reason> / END. Dry-run them: copy each doc into your scratch folder under its repository path and run \`python3 ${C}/scripts/apply_fixes.py ${C}/${unit}-fixes.md <scratch>/<docs path> --dry-run\`. Notes: ${C}/${unit}-notes.md (look there first; if they end with DONE, answer from them). Scratch only in ${C}/${unit}-scratch/. Fetch nothing; edit no doc; never commit or push. Finish by 20:15 Amsterdam (\`TZ=Europe/Amsterdam date\`).

${SAVE}

Return the structured result: each block (id, file, FIX or NOTE, one-line summary), your checks, and your fixes file's path.`
const EAB = `Your text: docs/adr/README.md, docs/SPIKE_PLAN.md and docs/ENGINE_OVERVIEW.md. Your items (every line of the work list's engine-architect-b section; conditions 2 to 4 and 10):
- README :21 (ADR 0006's row: 21 numbered, 17 open: -1 answered in part, -2 answered by Mustafa (2026-11-03), the others as they stand) and :23-25 (the total: 115 numbered, 96 open; OQ-0006-2 answered and OQ-0006-1 answered in part by his Hetzner check later that day).
- README :133 (OQ-0006-1's row: answered in part by his check, as ADR 0006's entry words it; still open: whether the client VM, SH's CX33 and MV can be ordered in Nuremberg, checked in the proof test's plan) and :134 (OQ-0006-2: "Answered by Mustafa (2026-11-03): Nuremberg (\`nbg1\`), the option he chose", no quotation marks around the option).
- SPIKE_PLAN's machines paragraph (:31-33): RS with his words quoted ("The CCX23 (dedicated) is used only for the measurement runs: tick timing, the 60 Hz margin and rooms per VM. Each run's plan says which server it needs."); DS, the CCX13 (dedicated x86, 2 vCPU, 8 GB; 43.49 EUR a month, 0.0697 an hour, excluding VAT, R6-HOST-F15), for development and "does it work" runs (his answer to the session's question 1, the option he chose, in the session's wording); LG with his words quoted ("Use a CAX21 (Arm64, 4 vCPU, 8 GB, about €0.020/h incl. VAT) for the simulated clients instead of the CX33, so their build targets aarch64."), whether it "(or another client VM)" can be ordered in Nuremberg checked in the proof test's plan; SH x86 only (a CX33 if orderable in Nuremberg, else not measured); MV a small VM in Nuremberg running Prometheus only, never RS, named in the proof test's plan after its order check; his earlier "the CX33 runs the simulated clients" kept only as the stop's record, if at all.
- Item 1's "Depends on" (:129-130): OQ-0006-1 answered in part, the location \`nbg1\` (OQ-0006-2, answered), the client VM's order check in the proof test's own plan; :176 ("it feeds OQ-0006-2": PM-0006-12 now decides nothing); :281 (item 4's "RS orderable (OQ-0006-1)"); :513 (the open list: the server location answered, OQ-0006-1 answered in part).
- ENGINE_OVERVIEW only if a line becomes untrue (otherwise a NOTE).
- Every other line of your section: a FIX or a NOTE.`
const RUST = `Your text: ADR 0004 (docs/adr/0004-simulation-core.md) outside its "Pass marks" section. Your item (condition 6): an Arm load generator is not "an ARM host" in part 7's sense: it serves no player and runs no room (the advisor's reading, 2026-11-03). S7, PM-0004-26 and the A64 key stay as they are. At most one sentence in part 7 (about lines 286-290), pointing to ADR 0001's rule for rows that compare an LG client's result with the server's, which netcode-architect writes in this round: no pass mark's verdict rests on native aarch64 agreeing with x86-64 or Wasm. If your text needs nothing, a NOTE with the reason. The CX33 key and PM-0004-1 and -4 are test-engineer's (it adds DS and makes the CX33 part conditional on the order check).`
const EAA = `Your text: ADR 0002 (docs/adr/0002-code-structure-build-toolchain.md) and ADR 0005 outside its ability section and its GPU rules. Your items (condition 7):
- "No arm64 leg" stays: RS, DS and SH are all x86-64. At most one sentence, where ADR 0002 speaks of the arm64 leg (about 626-627) or of the playtest soak (about 519-528): the bot clients' aarch64 build is the netcode proof test's own; the playtest soak's bot VM is named when the first soak is planned; if that VM is Arm, \`devtools\` gains an aarch64 build target in that change-set (a build, not a determinism leg), and the soak's desyncs follow ADR 0001's rule (netcode-architect writes it in this round: no pass mark's verdict rests on native aarch64 agreeing with x86-64 or Wasm).
- Check that nothing in ADR 0002 or ADR 0005 calls the CCX13 "the development server" (that name is the local server's; the CCX13 is DS in the keys), and that no sentence of yours becomes untrue under his answers (the reference server as the only game server, the location, the load generators' machines): a FIX or a NOTE for each place you find, and a NOTE if none.`
phase('Write')
const out = await parallel([
  () => agent(`${common('r6-engine-architect-b', 'engine-architect (b), author of docs/adr/README.md, docs/SPIKE_PLAN.md and docs/ENGINE_OVERVIEW.md')}\n\n${EAB}`, { label: 'r6-engine-architect-b', phase: 'Write', agentType: 'engine-architect', effort: 'xhigh', schema: SCHEMA }),
  () => agent(`${common('r6-rust-reviewer', 'rust-reviewer, author of ADR 0004 (docs/adr/0004-simulation-core.md) except its "Pass marks" section')}\n\n${RUST}`, { label: 'r6-rust-reviewer', phase: 'Write', agentType: 'rust-reviewer', effort: 'xhigh', schema: SCHEMA }),
  () => agent(`${common('r6-engine-architect-a', 'engine-architect (a), author of ADR 0002 (docs/adr/0002-code-structure-build-toolchain.md) and of ADR 0005 outside its ability section and its GPU rules')}\n\n${EAA}`, { label: 'r6-engine-architect-a', phase: 'Write', agentType: 'engine-architect', effort: 'xhigh', schema: SCHEMA }),
])
return { eaB: out[0], rust: out[1], eaA: out[2] }