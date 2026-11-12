export const meta = {
  name: 'foundation-2-2-review-round-1-skeptics',
  description: 'Foundation 2.2 review round 1, run 2: one adversarial skeptic per deduped finding, 3 at a time (Opus 5.5, max)',
  phases: [{ title: 'Verify', detail: 'one skeptic per finding, in batches of 3' }],
}

const WT = '/home/user/Shamble-Frontlines/.gate/foundation-toolchain-wt'
const NOTES = WT + '/.checkpoints/foundation-toolchain'
const HEAD = args.head

const BLOCK = (d) => `You are a skeptic for foundation step 2.2 of Shamble Frontlines (the toolchain, PR #48, risk tier 2). Your one job: test whether a review finding holds. Default to "does not hold" when the evidence is not there; do not invent new findings.

Where: the branch worktree ${WT} (branch claude/laughing-pasteur-fwya24; the head under review is ${HEAD}; main is origin/main, 2aa5e9c). Never the folder /home/user/Shamble-Frontlines itself (it holds main). Run each command from ${WT} with cd in the same command (the guard refuses git -C and complex shell forms; you cannot be asked). Change no file in ${WT}: to re-run or build anything, copy what you need into your own folder /home/user/Shamble-Frontlines/.gate/skeptic-${d.id}/ and work there. ${WT}/node_modules is a root npm install left by a local test (the same as CI's own \`npm ci\`). CI logs: \`curl -sSL https://api.github.com/repos/Mergincanli/Shamble-Frontlines/actions/jobs/<job id>/logs\`; GitHub's REST API works without a token for public reads.

The finding, ${d.id} (${d.severity}; it merges the reviewers' ${d.merges.join(', ')}; the session's one-line reading: ${d.gist}). Read each merged finding's full text (problem, evidence, proposed fix) from ${NOTES}/review-r1-findings.json: a JSON list of the five reviewers' returns; each return's value.findings holds findings by id (${d.merges.join(', ')}).

Do: re-run or re-read what the finding rests on (commands, files, logs, sources); judge whether it holds as stated, and at which severity on CLAUDE.md's one scale (Critical, High, Medium, Low; None if it does not hold); judge whether the proposed fix is right and complete, and name anything it would break. Write your notes as you go to ${NOTES}/skeptic-r1-${d.id}.md (a plan line first, then a line per check). Return the verdict per the schema.`

const VERDICT = {
  type: 'object',
  properties: {
    id: { type: 'string' },
    holds: { type: 'boolean' },
    severity: { type: 'string', enum: ['Critical', 'High', 'Medium', 'Low', 'None'] },
    reason: { type: 'string', description: 'what you re-ran or re-read, and what it showed' },
    fix_ok: { type: 'string', description: 'is the proposed fix right and complete; what it would break; a better fix if any' },
  },
  required: ['id', 'holds', 'severity', 'reason', 'fix_ok'],
}

phase('Verify')
const out = []
const list = args.findings
for (let i = 0; i < list.length; i += 3) {
  const batch = list.slice(i, i + 3)
  const vs = await parallel(batch.map((d) => () => agent(BLOCK(d),
    { label: `skeptic:${d.id}`, phase: 'Verify', agentType: 'general-purpose', model: 'opus', effort: 'max', schema: VERDICT })))
  batch.forEach((d, k) => out.push(vs[k] ? { ...vs[k], id: d.id, status: 'ok' } : { id: d.id, status: 'failed' }))
  log(`${out.length} of ${list.length} findings checked`)
}
return { verdicts: out, failed: out.filter((v) => v.status !== 'ok').map((v) => v.id) }
