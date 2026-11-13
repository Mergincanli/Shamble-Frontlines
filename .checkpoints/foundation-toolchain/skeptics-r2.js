export const meta = {
  name: 'foundation-2-2-review-round-2-skeptics',
  description: 'Foundation 2.2 review round 2: one adversarial skeptic per deduped finding, a pool of args.pool at a time (Opus 5.5, max)',
  phases: [{ title: 'Verify', detail: 'one skeptic per deduped finding, a small pool' }],
}

const WT = '/home/user/Shamble-Frontlines/.gate/foundation-toolchain-wt'
const NOTES = WT + '/.checkpoints/foundation-toolchain'
const HEAD = args.head

const BLOCK = (d) => `You are a skeptic for foundation step 2.2 of Shamble Frontlines (the toolchain, PR #48, risk tier 2), review round 2. Your one job: test whether one deduped review finding holds, and whether its proposed fix is right. Default to "does not hold" when the evidence is not there.

Where: the branch worktree ${WT} (branch claude/laughing-pasteur-fwya24). The content under review is ${HEAD}; later commits touch only .checkpoints/. Main is origin/main (2aa5e9c). Never work in the folder /home/user/Shamble-Frontlines itself (it holds main). Run each command from ${WT}; the guard refuses \`git -C\`, so run \`cd\` as a command of its own. Change no file in ${WT} except your notes file: to re-run or build anything, copy what you need (\`git archive\` or \`git worktree add\`) into your own folder /home/user/Shamble-Frontlines/.gate/skeptic-r2-${d.id}/ and work there. Some deletes are refused: leave leftovers and name them. Here: rustup, cargo and rustc 1.99.1, the wasm32-unknown-unknown target, Node 22. CI logs: \`gh api repos/Mergincanli/Shamble-Frontlines/actions/jobs/<job id>/logs\`.

The finding, ${d.id} (${d.severity}; it merges ${d.merges.join(', ')}). The session's one-line reading: ${d.gist}
Read the full texts first: ${NOTES}/review-r2-findings.md (the deduped table and test-engineer's full texts), then ${d.sources}.
${d.extra || ''}

Do:
1. Re-run or re-read what the finding rests on (commands, probes, files, logs, sources). Prefer your own minimal reproduction to the reviewer's outputs; read the cited source lines yourself.
2. Judge whether it holds as stated, and at which severity on the one scale (Critical: breaks security, data, the guard or main; High: a wrong result or rule a player, Mustafa or a session would hit, a rule loosened beyond Mustafa's words, evidence that does not hold; Medium: a real defect with a workaround or a small reach; Low: polish, wording, a nit; None if it does not hold).
3. Judge the proposed fix: right and complete? What would it break? Give the exact replacement wording you would use where the fix is text (keep it short and accurate; the report and its decision entries go to Mustafa, a game-design student, so plain words first).
4. If you notice something else of Medium or higher in passing, put it in also_noticed with its evidence; otherwise leave it empty. Do not go looking for new findings.
Write your notes as you go to ${NOTES}/skeptic-r2-${d.id}.md (a plan line first, then a line per check). Return the verdict per the schema.`

const VERDICT = {
  type: 'object',
  properties: {
    id: { type: 'string' },
    holds: { type: 'boolean' },
    severity: { type: 'string', enum: ['Critical', 'High', 'Medium', 'Low', 'None'] },
    reason: { type: 'string', description: 'what you re-ran or re-read, and what it showed' },
    fix_ok: { type: 'string', description: 'is the proposed fix right and complete; what it would break; the exact wording you would use' },
    also_noticed: { type: 'string', description: 'anything of Medium or higher noticed in passing, with evidence; empty if none' },
  },
  required: ['id', 'holds', 'severity', 'reason', 'fix_ok', 'also_noticed'],
}

phase('Verify')
const queue = args.findings.slice()
const out = []
const worker = async () => {
  while (queue.length) {
    const d = queue.shift()
    const v = await agent(BLOCK(d), { label: `skeptic:${d.id}`, phase: 'Verify', agentType: 'general-purpose', model: 'opus', effort: 'max', schema: VERDICT })
    out.push(v ? { ...v, id: d.id, status: 'ok' } : { id: d.id, status: 'failed' })
    log(`${out.length} of ${args.findings.length} findings checked (${d.id}: ${v ? (v.holds ? 'holds, ' + v.severity : 'does not hold') : 'failed'})`)
  }
}
await parallel(Array.from({ length: args.pool }, () => () => worker()))
return { verdicts: out, failed: out.filter((v) => v.status !== 'ok').map((v) => v.id) }
