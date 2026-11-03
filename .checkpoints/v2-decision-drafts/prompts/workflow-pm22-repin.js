// Workflow script (verbatim, copied at launch, run wf_9d6ea220-1f3, 2026-11-03 about 07:48 Amsterdam): test-engineer xhigh, engine-architect high, model: the session's (Opus 5.5)
export const meta = {
  name: 'v2-drafts-pm22-repin',
  description: "Two small touch-ups in parallel: test-engineer extends PM-0004-22 as ADR 0004 part 7 now names it, and the synthesis author re-pins README's 23 pointers into ADR 0005",
  phases: [
    { title: 'Write', detail: 'test-engineer (PM-0004-22) and engine-architect (README pointers), each writing exact FIX blocks' },
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

const FIX_SCHEMA = { type: 'object', properties: {
  blocks: { type: 'array', items: { type: 'object', properties: { id: { type: 'string' }, kind: { type: 'string', enum: ['FIX', 'NOTE'] }, line_delta: { type: 'integer' }, summary: { type: 'string' } }, required: ['id', 'kind', 'line_delta', 'summary'] } },
  net_line_delta: { type: 'integer' }, checks: { type: 'string' }, file: { type: 'string' } },
  required: ['blocks', 'net_line_delta', 'checks', 'file'] }

const BLOCK_FORM = (file, fixes) => `Write the blocks into ${fixes}, every BEFORE copied by script from the file on disk, whole lines, occurring exactly once:
### FIX <id>
FILE: ${file}
BEFORE:
<the exact current text>
AFTER:
<the new text>
END
or, for something you decline, ### NOTE <id> … END with your evidence. Check that your blocks apply: copy the doc into your scratch folder under the same relative path (<scratch>/${file}) and run \`python3 ${C}/scripts/apply_fixes.py ${fixes} <scratch>/${file} --dry-run\` (the script applies a block only to a path that ends with its FILE).`

const te = `You are test-engineer, the author of the pass-mark tables of the v2 decision drafts of SHAMBLE FRONTLINES (branch claude/laughing-lovelace-ox61se; the docs on disk). ADR 0004's row PM-0004-22 (docs/adr/0004-simulation-core.md, the line starting "| PM-0004-22 |") is yours. rust-reviewer's fix to ADR 0004 part 7 (commit 7cc6942, after the re-check fixcheck-d3a's finding D3A-1 and a skeptic) now says \`rem_euclid\` and \`div_euclid\` may join the allow-list "only once fixed inputs, those cases and their sign mirrors included, equal an exact reference on every target (PM-0004-22, extended)", because rustc 1.97.0's \`div_euclid\` returns 3 where the exact Euclidean quotient is 2, the same bits on every target, which a bit-identity check passes. rust-reviewer's NOTE D3A-1-PM22 (${C}/rust-reviewer-touchup-d3a-fixes.md) proposes the extension's words for your row: a clause after the \`sqrt\` clause, and an addition to the row's "Bites" cell.

Check it first: read part 7 as it now stands, the NOTE, the skeptic's file (${C}/verifier-skeptic-6-d3a-1.md) and rust-reviewer's run (${C}/rust-reviewer-touchup-d3a-scratch/rr_euclid.out); recompute the expected quotients and remainders yourself (exact integer arithmetic on the inputs scaled by 2^23 or 2^52). If it holds, write one exact FIX block for the PM-0004-22 row: it stays one table line with exactly 7 cells, and keeps every number sourced (the cases and their expected values from those runs, named by unit and date as the drafts name the verifiers). The clause applies only if part 7's two methods are to join the list; it must not make today's run of PM-0004-22 depend on them. Keep the plan's rules: no GLOSSARY "Retire (exact)" phrase (\`python3 ${C}/scripts/glossary_check_drafts.py\` on a scratch tree laid out as the repository); never a sentence directly under a table. Fetch nothing; edit no doc; never commit or push.

${BLOCK_FORM('docs/adr/0004-simulation-core.md', C + '/test-engineer-touchup-pm22-fixes.md')}

Notes: ${C}/test-engineer-touchup-pm22-notes.md (look there first; if they end with DONE, answer from them). Scratch only in ${C}/test-engineer-touchup-pm22-scratch/. Finish by 09:30 Amsterdam.

${SAVE}

Return the structured result: each block (id, FIX or NOTE, line change, one-line summary), the net line change, your checks (dry run, the 7 cells, glossary), and your file's path.`

const ea = `You are engine-architect, the author of the synthesis of the v2 decision drafts of SHAMBLE FRONTLINES (docs/ENGINE_OVERVIEW.md, docs/SPIKE_PLAN.md, docs/adr/README.md; branch claude/laughing-lovelace-ox61se; the docs on disk). The game-designer's last touch-up of ADR 0005's ability section (commit 361b61d: fixcheck-d2 F1b, F8, F9, F10 and AB-1's lost Why line) added 2 lines to ADR 0005, so README's 23 "[ADR 0005](…) line L" pointers at line 1186 or later no longer land on their open question (each is now 2 lines further down). ADR 0004's last touch-up (7cc6942) kept its line count. This is notes only: you edit no doc, and you never commit, push or fetch.

1. Run \`python3 ${C}/scripts/draft_pointers.py\` from the repository root (/home/user/Shamble-Frontlines). It lists each pointer that is off, with the line where its OQ id now sits.
2. For each pointer that is off, write one exact FIX block that changes only its line number to the line the script gives. Change nothing else; keep every line's length and wrapping where you can (a number one digit longer may re-wrap its line: then keep the paragraph's line count). Where one README line holds several pointers, one block for that line is fine.
3. Check by applying your blocks to a scratch tree: copy docs/adr/*.md into <scratch>/docs/adr/, apply your blocks to <scratch>/docs/adr/README.md, and run the pointer script from <scratch> (it reads docs/adr/ relative to where it runs): it must print "off 0". \`python3 ${C}/scripts/glossary_check_drafts.py\` run the same way must still give exact 0.

${BLOCK_FORM('docs/adr/README.md', C + '/touchup3-engine-architect-repin-fixes.md')}

Notes: ${C}/touchup3-engine-architect-repin-notes.md (look there first; if they end with DONE, answer from them). Scratch only in ${C}/touchup3-engine-architect-repin-scratch/. Finish by 09:30 Amsterdam.

${SAVE}

Return the structured result: each block (id, FIX, line change, one-line summary), the net line change (0 expected), your checks (the script's output on your scratch tree, glossary), and your file's path.`

phase('Write')
const [pm22, repin] = await parallel([
  () => agent(te, { label: 'test-engineer: PM-0004-22', phase: 'Write', agentType: 'test-engineer', effort: 'xhigh', schema: FIX_SCHEMA }),
  () => agent(ea, { label: 'engine-architect: re-pin README', phase: 'Write', agentType: 'engine-architect', effort: 'high', schema: FIX_SCHEMA }),
])
return { pm22, repin }
