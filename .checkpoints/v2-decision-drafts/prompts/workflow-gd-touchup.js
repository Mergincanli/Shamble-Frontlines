// Workflow script (verbatim, copied at launch, run wf_0c35d4e2-382, 2026-11-03 about 07:16 Amsterdam): game-designer agentType, effort xhigh; verifier agentType, effort xhigh; model: the session's (Opus 5.5)
export const meta = {
  name: 'v2-drafts-gd-touchup',
  description: "ADR 0005's ability section: the game-designer's last fixes (fixcheck-d2 F1b, F8, F9, F10 and two lost lines), each checked by a fresh verifier before the session applies them",
  phases: [
    { title: 'Fix', detail: 'game-designer writes exact FIX blocks for its own text' },
    { title: 'Check', detail: 'a fresh verifier dry-runs and judges every block before it is applied' },
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

const GD_SCHEMA = {
  type: 'object',
  properties: {
    blocks: { type: 'array', items: { type: 'object', properties: {
      id: { type: 'string' }, finding: { type: 'string' }, kind: { type: 'string', enum: ['FIX', 'NOTE'] },
      line_delta: { type: 'integer' } }, required: ['id', 'finding', 'kind', 'line_delta'] } },
    net_line_delta: { type: 'integer' },
    other_lost_lines: { type: 'string' },
    fixes_file: { type: 'string' },
  },
  required: ['blocks', 'net_line_delta', 'other_lost_lines', 'fixes_file'],
}
const CHECK_SCHEMA = {
  type: 'object',
  properties: {
    verdict: { type: 'string', enum: ['PASS', 'PASS WITH NOTES', 'FAIL'] },
    items: { type: 'array', items: { type: 'object', properties: {
      block: { type: 'string' }, holds: { type: 'boolean' }, problem: { type: 'string' } }, required: ['block', 'holds', 'problem'] } },
    lost_lines: { type: 'array', items: { type: 'string' } },
    net_line_delta: { type: 'integer' },
    pointers_off: { type: 'integer' },
    verdicts_file: { type: 'string' },
  },
  required: ['verdict', 'items', 'lost_lines', 'net_line_delta', 'pointers_off', 'verdicts_file'],
}

const gdPrompt = `You are the author of ADR 0005's section "The ability and effect system" in the v2 decision drafts of SHAMBLE FRONTLINES (branch claude/laughing-lovelace-ox61se; the docs as they are on disk).

Your task is written in ${C}/prompts/touchup2-game-designer.txt: read it and follow it exactly (its findings, its rules, its output file ${C}/touchup2-game-designer-fixes.md, its notes file and its scratch folder). A first run of this same task was cut off by a usage limit on 2026-11-02 at about 21:19 Amsterdam, after it had checked the findings; continue from its notes. Then read ${C}/prompts/touchup2-game-designer-resume.txt and follow it too: it was written for that first run, and it applies to you as its continuation (what changed on disk since, the line-count rule, the deadline: finish by 09:30 Amsterdam). Fetch nothing; edit no doc; never commit or push.

After your blocks, a fresh verifier dry-runs them on a copy and checks each one against its finding and its sources before the session applies anything, so make every BEFORE exact (copied by script from the file on disk, whole lines, once) and every AFTER complete.

${SAVE}

Return the structured result: each block (its id, the finding it answers, FIX or NOTE, its line change), the net line change of your section, any other lost line you found (or "none"), and your fixes file's path.`

const checkPrompt = (round, gd) => `You are a fresh verifier for the v2 decision drafts of SHAMBLE FRONTLINES (branch claude/laughing-lovelace-ox61se; the docs on disk; \`git -C /home/user/Shamble-Frontlines log -1 --format=%h\` names HEAD). Before the session applies them, you check the game-designer's FIX and NOTE blocks in ${C}/touchup2-game-designer-fixes.md (round ${round}; the author's own summary: ${JSON.stringify(gd)}). They answer fixcheck-d2's findings F1b, F8, F9 and F10 (${C}/verifier-fixcheck-d2-verdicts.md, section "Findings: ADR 0003, 0004, 0005"), and two lines the author found lost in its own text: AB-1's "Why" line (dropped by block RR-4 of ${C}/game-designer-fixes-final.md) and the dangling line "    names, with no generic-engine feature." after AB-1's pool items. The author's own text is the section of docs/adr/0005-content-abilities-assets.md from its heading "## The ability and effect system (game-designer)" to the line before "## The asset pipeline (engine-architect)". You edit no doc; you never commit, push or fetch.

Check, in this order:
1. **Dry-run.** Copy docs/adr/0005-content-abilities-assets.md into your scratch folder; run \`python3 ${C}/scripts/apply_fixes.py ${C}/touchup2-game-designer-fixes.md <copy> --dry-run\`, then without --dry-run on the copy. Every BEFORE must occur exactly once.
2. **Ownership.** Every block's BEFORE lies inside the author's section (bounds above). A block outside it is a finding.
3. **Each block against its finding and its sources.** Does the AFTER resolve the finding as fixcheck-d2 states it, and add nothing wrong? F1b restores the AB-3 heading exactly as it stood at 47eadd9 (\`git -C /home/user/Shamble-Frontlines show 47eadd9:docs/adr/0005-content-abilities-assets.md\`), between AB-2's Need line and AB-3's What line. AB-1's restored Why line: compare with 47eadd9's text; any change from it needs a stated reason. F8: ADR 0001 part 12 (grep "a steal is holding E" in docs/adr/0001-multiplayer-netcode.md) and LOOP_V2 §4 steps 2 to 4 (docs/LOOP_V2.md). F9: docs/LEGACY_TUNING.md, the WEAPONS table's \`pellets\` row (line about 180) and the Molotov's fire interval; recompute the example's arithmetic. F10: AB-5 keeps one Need line, and the one kept names everything the two said. For each NOTE: is its reason sound?
4. **Nothing else lost.** Compare the section's structural lines at 47eadd9 and on your applied copy: headings, the \`- **AB-n.\` starts, and each AB item's What, Why, Research, Need and "When a pool is full" lines. List every structural line present at 47eadd9 and absent from the copy that no block removes on purpose with a stated reason, and every structural line that appears twice in one item.
5. **The rules on the changed lines:** a number with no source; a GLOSSARY "Retire (exact)" phrase (the list is in docs/research/GLOSSARY.md; \`python3 ${C}/scripts/glossary_check_drafts.py\` reads docs/adr/ from the directory it runs in, so run it in a scratch tree laid out as the repository with your applied copy in place); a sentence directly under a table.
6. **Line count and pointers.** The section's net line change, and README's pointers: lay out a scratch tree (<scratch>/docs/adr/ with every docs/adr/*.md, 0005 replaced by your applied copy), run \`python3 ${C}/scripts/draft_pointers.py\` from <scratch>, and report how many are off.

Write your verdicts to ${C}/verifier-gdcheck-verdicts.md as you go (under a heading "Round ${round}"; keep earlier rounds), a table \`| block | finding | holds? | problem |\`, then the lost-line list, the counts and your verdict. Notes: ${C}/verifier-gdcheck-notes.md (look there first; if a round ${round} entry ends with DONE, answer from it). Scratch only in ${C}/verifier-gdcheck-scratch/.

${SAVE}

Return the structured result: PASS, PASS WITH NOTES or FAIL (FAIL if any block fails to apply, lies outside the section, does not resolve its finding, adds something wrong, or a structural line is still lost); one item per block (holds or not, and the problem, or "none"); the lost or doubled structural lines (empty if none); the net line change; the pointers off; your verdicts file's path.`

const repairPrompt = (round, check) => `You are the author of ADR 0005's section "The ability and effect system" (game-designer), continuing your touch-up (${C}/prompts/touchup2-game-designer.txt and ${C}/prompts/touchup2-game-designer-resume.txt, the same rules). A fresh verifier dry-ran your blocks in ${C}/touchup2-game-designer-fixes.md before they are applied and found problems (round ${round - 1}; its verdicts: ${C}/verifier-gdcheck-verdicts.md): ${JSON.stringify(check)}.

For each problem: check it first (you act as its skeptic). If it holds, correct your fixes file: replace the affected block in place (keep the file valid, every BEFORE exact against the doc on disk, which is still unchanged), or add the missing block. If it does not hold, add a NOTE with your evidence. Add a notes line for each. Fetch nothing; edit no doc; never commit or push.

${SAVE}

Return the structured result as before, for the whole file as it now stands.`

phase('Fix')
let gd = await agent(gdPrompt, { label: 'game-designer: touch-up', phase: 'Fix', agentType: 'game-designer', effort: 'xhigh', schema: GD_SCHEMA })
if (!gd) { log('game-designer returned nothing: stop, the session reads its notes'); return { gd: null } }
log(`game-designer: ${gd.blocks.length} blocks, net line change ${gd.net_line_delta}`)

const rounds = []
let check = null
for (let round = 1; round <= 3; round++) {
  phase('Check')
  check = await agent(checkPrompt(round, gd), { label: `verifier: pre-apply check, round ${round}`, phase: 'Check', agentType: 'verifier', effort: 'xhigh', schema: CHECK_SCHEMA })
  if (!check) { log(`round ${round}: the verifier returned nothing`); break }
  rounds.push({ round, gd, check })
  log(`round ${round}: ${check.verdict}; ${check.items.filter(i => !i.holds).length} blocks with a problem; lost lines ${check.lost_lines.length}; pointers off ${check.pointers_off}`)
  if (check.verdict !== 'FAIL' || round === 3) break
  phase('Fix')
  const fixed = await agent(repairPrompt(round + 1, check), { label: `game-designer: repair, round ${round + 1}`, phase: 'Fix', agentType: 'game-designer', effort: 'xhigh', schema: GD_SCHEMA })
  if (!fixed) { log('repair returned nothing'); break }
  gd = fixed
}
return { gd, check, rounds: rounds.length }
