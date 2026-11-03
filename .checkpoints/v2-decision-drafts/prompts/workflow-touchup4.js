// Workflow script (verbatim, copied at launch, run wf_c38a2d32-525, 2026-11-03 about 08:46 Amsterdam; a first launch at effort high, wf_e4e34273-d92, was stopped within a minute and relaunched at xhigh, per CLAUDE.md's rule for ADR text): engine-architect, rust-reviewer, test-engineer, effort xhigh, model: the session's (Opus 5.5)
export const meta = {
  name: 'v2-drafts-touchup4',
  description: "The last touch-ups from fixcheck-d3: engine-architect (README seams S5, S9, S10; 111 open; review codes; one wrap in ADR 0002), rust-reviewer (ADR 0004 part 7), test-engineer (PM-0003-2; one wrap in ADR 0001's checklist), each writing exact FIX blocks for its own text",
  phases: [
    { title: 'Write', detail: 'three authors, each on its own text, exact FIX blocks the session applies' },
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
  blocks: { type: 'array', items: { type: 'object', properties: { id: { type: 'string' }, file: { type: 'string' }, kind: { type: 'string', enum: ['FIX', 'NOTE'] }, line_delta: { type: 'integer' }, summary: { type: 'string' } }, required: ['id', 'file', 'kind', 'line_delta', 'summary'] } },
  checks: { type: 'string' }, fixes_file: { type: 'string' } },
  required: ['blocks', 'checks', 'fixes_file'] }

const COMMON = (unit) => `This is notes only: you edit no doc, and you never commit, push or fetch. Check each finding first against the text and its source (you act as its owner and skeptic); if it holds, write one exact FIX block; if it does not, write a NOTE with your evidence. Keep the plan's rules: every number sourced; no GLOSSARY "Retire (exact)" phrase (\`python3 ${C}/scripts/glossary_check_drafts.py\` reads docs/ relative to where it runs: run it in a scratch tree laid out as the repository with your blocks applied); never a sentence directly under a table; no new line over 120 characters. Write the blocks into ${C}/${unit}-fixes.md, every BEFORE copied by script from the file on disk, whole lines, occurring exactly once:
### FIX <id>
FILE: <docs path>
BEFORE:
<the exact current text>
AFTER:
<the new text>
END
(or ### NOTE <id> … END). Check that the blocks apply: copy each doc into your scratch folder under its repository path (<scratch>/docs/…) and run \`python3 ${C}/scripts/apply_fixes.py ${C}/${unit}-fixes.md <scratch>/<docs path> --dry-run\` for each doc (the script applies a block only to a path that ends with its FILE). Notes: ${C}/${unit}-notes.md (look there first; if they end with DONE, answer from them); scratch only in ${C}/${unit}-scratch/. Finish by 11:00 Amsterdam.

${SAVE}

Return the structured result: each block (id, file, FIX or NOTE, line change, one-line summary), your checks (the dry runs, glossary, line lengths, and any pointer check named above), and your fixes file's path.`

const ea = `You are engine-architect, author of the synthesis of the v2 decision drafts of SHAMBLE FRONTLINES (docs/ENGINE_OVERVIEW.md, docs/SPIKE_PLAN.md, docs/adr/README.md) and of ADR 0002 (docs/adr/0002-code-structure-build-toolchain.md); branch claude/laughing-lovelace-ox61se, the docs on disk. The last re-check, fixcheck-d3 (${C}/verifier-fixcheck-d3c-verdicts.md and ${C}/verifier-fixcheck-d3e-verdicts.md), found four things in your text:

1. **D3C-2 (medium, dangling reference).** ADR 0005:106–107 (its frame, your text) says "Its seams are the decision map's S5 (content, execution and drawing), S6 (moments and telemetry), S9 (the look and the asset rules) and S10 (the editor)." README's Seams table (under "Codes the drafts cite", the table after "**Seams**", about lines 325–333) holds only S1, S3, S4, S6, S11 and S12; S5, S9 and S10 live only in the decision map (${C}/engine-architect-decision-map.md, lines 207, 211, 212), which leaves the branch before the PR is marked ready. Smallest fix: add rows S5, S9 and S10 to README's Seams table, in its column form and in order, from the map's rows (shorten each cell to the table's style; keep what each side decides). README is pointed into by no other doc by line number, so its line count may change.
2. **D3C-4 (low, count).** README:23 says "114 numbered open questions, 112 of them open (OQ-0004-5 and -6 point to ADR 0005)", but OQ-0003-14 (ADR 0003, about line 885) is "Answered by its decider, the verifier (plan step 7, 2026-11-01)", so 111 are open. Fix README:23 ("111 of them open (OQ-0004-5 and -6 point to ADR 0005; OQ-0003-14 is answered)" or your words), and the 0003 row's last cell in the table above it ("20 (OQ-0003-1 to -20)") in the form the 0004 row uses ("12 numbered (OQ-0004-1 to -12), 10 open"). Check every other count statement in your three docs by script against the drafts.
3. **D3C-3 (low, dangling reference).** Review codes cited in the drafts and README (GD-REJOIN, GD-PICK-VIS, GD-CARRIER, RELEASE-WINDOW, LOOK-FLOOR, REDUCED-MOTION, COLOUR-TELL, HEAT-SOAK, POOL-FULL, EA-8, SA-10, RR-10; places: 0001:478, 513, 604; 0002:1253; 0005:1367; 0006:932–933; README:77, 79, 80, 81, 87) are defined only in the step-10 review files under .checkpoints/, which leave the branch before the PR is marked ready. Smallest fix: one sentence in README's "Codes the drafts cite" saying that review codes, and "the X review, finding n", name the step-10 reviews of 2026-11-02 (the report's appendix §7.2 summarizes each; the review files stay readable in the branch's history), as the section already does for "plan rule n" and "condition n". Keep it to that sentence.
4. **D3C-1, your half (low, wrap).** ADR 0002's Migration plan step 5 (about lines 1424–1426) breaks mid-sentence: "…G12 over both transport backends; the" / "   first multiplayer" / "   preview behind the access wall, …". Re-wrap that step at 120 characters so it reads as whole lines (it may lose a line; README's pointers into ADR 0002 all sit before line 1060, so they do not move: confirm with \`python3 ${C}/scripts/draft_pointers.py\` run in your scratch tree with all blocks applied, which must print "off 0").

${COMMON('touchup4-engine-architect')}`

const rr = `You are rust-reviewer, author of ADR 0004 (docs/adr/0004-simulation-core.md) in the v2 decision drafts of SHAMBLE FRONTLINES (branch claude/laughing-lovelace-ox61se; the docs on disk). The last re-check, fixcheck-d3, found two small things in part 7's sentence on \`rem_euclid\` and \`div_euclid\` (about lines 271–276, your fix 7cc6942):

1. **D3B-2 (low, inconsistent; ${C}/verifier-fixcheck-d3b-verdicts.md).** Part 7 says the two "may join only once fixed inputs, those cases and their sign mirrors included, equal an exact reference on every target (PM-0004-22, extended)". PM-0004-22 (the row "| PM-0004-22 |", test-engineer's) says "a correctly rounded Euclidean reference, the exact quotient and remainder by integer or rational arithmetic rounded once to the type": an exact remainder need not fit the type (rustc 1.97.0's (−2^-30f32).rem_euclid(1.0) = 1.0, the correct rounding of 1 − 2^-30), so a literally exact reference would fail a correct method. Smallest fix: "equal a rounded exact reference" (or "a correctly rounded reference", PM-0004-22's words).
2. **D3E-1 (low; ${C}/verifier-fixcheck-d3e-verdicts.md, fact 2).** The pin is Rust 1.99.1 (ADR 0002 part 4). fixcheck-d3e read the 1.99.1 source on 2026-11-03 (core/num/f32.rs and f64.rs: \`let q = trunc(x / rhs); if x % rhs < 0.0 { return if rhs > 0.0 { q - 1.0 } else { q + 1.0 }; } q\`, which std delegates to; its saved excerpts: ${C}/verifier-fixcheck-d3e-scratch/rust-1.99.1-src-core-f32.txt, -f64.txt, -src-std-f32.txt, -f64.txt), so 1.99.1 also returns 3 for the case. Smallest fix: say so in part 7, cited as the drafts cite the verifiers (by unit and date).

Keep part 7's paragraph at its line count (README points into ADR 0004 by line number, from line 855 on): re-wrap inside the paragraph if a line must absorb words, and run \`python3 ${C}/scripts/draft_pointers.py\` in a scratch tree with your blocks applied (it must print "off 0"). Change only your own text; PM-0004-22 is test-engineer's.

${COMMON('touchup4-rust-reviewer')}`

const te = `You are test-engineer, author of the pass-mark tables and the Brave checklists of the v2 decision drafts of SHAMBLE FRONTLINES (branch claude/laughing-lovelace-ox61se; the docs on disk). The last re-check, fixcheck-d3, found three small things in your text:

1. **D3E-2 (low; ${C}/verifier-fixcheck-d3e-verdicts.md, fact 1).** PM-0003-2 (docs/adr/0003-rendering.md, the row "| PM-0003-2 |") bites with "a planted bind group on the renderer's device that leaves out one entry of its layout, which must count as a device error whatever the console shows". The WebGPU specification (W3C Candidate Recommendation Draft of 15 September 2026, §22.3 and §22.4; fixcheck-d3e's excerpts in ${C}/verifier-fixcheck-d3e-scratch/webgpu-excerpt*.txt) says an error made inside an open error scope goes to that scope and fires no \`uncapturederror\` event, and that such events "won't necessarily be dispatched for all uncaptured errors". Smallest fix (optional in d3e's words, worth doing): the planted call sits outside any error scope of the renderer's, and if no \`uncapturederror\` arrives, the hook's own \`pushErrorScope('validation')\` around the planted call decides whether the browser or the hook missed it (WebGPU §22.3).
2. **D3E-3 (low; same file).** PM-0003-2 counts "its \`lost\` promise resolving" as a device error; \`GPUDevice.destroy()\` also resolves \`lost\`, with reason "destroyed" (\`GPUDeviceLostReason\` is "unknown" or "destroyed"), so a teardown would count. Smallest fix: count \`lost\` only with reason "unknown" (or close the count before any teardown).
3. **D3C-1, your half (low, wrap; ${C}/verifier-fixcheck-d3c-verdicts.md).** ADR 0001's netcode checklist (under "#### The netcode checklist", step 7, about lines 1517–1520) has a 156-character line. Re-wrap step 7 at 120 characters (it may gain a line: README's pointers into ADR 0001 all sit before line 1160, so they do not move; confirm with \`python3 ${C}/scripts/draft_pointers.py\` in a scratch tree with your blocks applied, which must print "off 0").

PM-0003-2 stays one table line with exactly 7 cells.

${COMMON('touchup4-test-engineer')}`

phase('Write')
const out = await parallel([
  () => agent(ea, { label: 'engine-architect: README, ADR 0002', phase: 'Write', agentType: 'engine-architect', effort: 'xhigh', schema: FIX_SCHEMA }),
  () => agent(rr, { label: 'rust-reviewer: ADR 0004 part 7', phase: 'Write', agentType: 'rust-reviewer', effort: 'xhigh', schema: FIX_SCHEMA }),
  () => agent(te, { label: 'test-engineer: PM-0003-2, ADR 0001 step 7', phase: 'Write', agentType: 'test-engineer', effort: 'xhigh', schema: FIX_SCHEMA }),
])
return { ea: out[0], rr: out[1], te: out[2] }
