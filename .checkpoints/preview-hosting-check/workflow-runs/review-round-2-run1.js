export const meta = {
  name: 'preview-check-review-round-2',
  description: 'Review round 2 of the preview hosting check build, on what changed (1b37fae..2df0112): infra-engineer, test-engineer',
  phases: [{ title: 'Review 2', detail: 'two reviewers, on the fix pass only' }],
}

const COMMON = `You are a reviewer in round 2 of the review budget (CLAUDE.md, the change-set protocol, step 5): round 2 reviews only what changed since round 1. Review only; edit nothing outside your notes file; never commit or push; create or change nothing on GitHub or Cloudflare; run no workflow.

The test repository Mergincanli/shamble-preview-check is cloned at /home/user/shamble-preview-check, branch "setup". Round 1 reviewed head 1b37fae; the fix pass is commit 2df0112 (read it with "git show 2df0112" or "git diff 1b37fae 2df0112" run from inside that folder; the guard refuses "git -C"). The four workflow files did not change (they stay byte for byte as the owner approved them). Round 1's findings and the fix pass's claims: /home/user/Shamble-Frontlines/.checkpoints/preview-hosting-check/infra-engineer-review.md, security-auditor-review.md, test-engineer-review.md, and the checkpoint /home/user/Shamble-Frontlines/.checkpoints/preview-hosting-check.md (step 15). The session's local proofs, which prove the scripts and not Cloudflare: /home/user/Shamble-Frontlines/.checkpoints/preview-hosting-check/local-proof/ (proof.mjs now 29 of 29 with a server that redirects .html paths as Pages does; browser-proof.mjs 5 of 5); you may run them (node proof.mjs /home/user/shamble-preview-check /home/user/Shamble-Frontlines/.gate/preview-hosting-check takes about 4 minutes).

Check: is each round-1 finding the fix pass claims to fix really fixed (F1/TE-1 above all), and did the fix pass break anything else in check-headers.mjs, lib.mjs, probe.mjs, wdio/run.mjs, site/page.js or site/index.html, including their command lines and file names as the unchanged workflows use them? Rate every finding on CLAUDE.md's scale (Critical, High, Medium, Low) with file, line, a failure scenario and the smallest fix; separate verified from inferred.

## Saving your progress
A usage limit can cut you off part way, and you would then be started again with this same task. Keep notes in /home/user/Shamble-Frontlines/.checkpoints/preview-hosting-check/<agent>-review-2.md (make the folder; you may always write there). Look there first: if the first line names another task, move the file into /home/user/Shamble-Frontlines/.checkpoints/preview-hosting-check/old/ and start fresh; if the notes end with DONE, write your answer from them; otherwise go on after the last block they mark done. Trust the blocks they mark done: never reread, recheck or redo them (a later block may still read a file it needs). Start a new file with one line naming your task, then your plan: the blocks you will do, in order (a block: a source read, a file reviewed, a test group run, a section written). Write each finished block to its file at once. Add a notes line for each finding as you find it (what, where), and one after each finished block (done, what comes next). Redo a block the notes don't mark done, written over, never appended. Scratch files go in /home/user/Shamble-Frontlines/.checkpoints/preview-hosting-check/: text and small scripts only; builds and binaries go under /home/user/Shamble-Frontlines/.gate/preview-hosting-check/. This folder is pushed to a public repository: never put a secret, token, environment value, request header, cookie or raw payload in it, whether in a note, a script or a script's output; the saver's secret check only knows a few token formats and cannot read archives or binaries. End with DONE, then give your answer from your notes alone, opening no other file. Never commit or push.`

const FINDINGS = {
  type: 'object',
  properties: {
    verdict: { type: 'string', enum: ['no findings', 'findings'] },
    fixed_confirmed: { type: 'array', items: { type: 'string' } },
    findings: {
      type: 'array',
      items: {
        type: 'object',
        properties: {
          id: { type: 'string' },
          severity: { type: 'string', enum: ['Critical', 'High', 'Medium', 'Low'] },
          file: { type: 'string' },
          line: { type: 'string' },
          summary: { type: 'string' },
          failure_scenario: { type: 'string' },
          fix: { type: 'string' },
          verified: { type: 'boolean' },
        },
        required: ['id', 'severity', 'file', 'summary', 'failure_scenario', 'fix', 'verified'],
      },
    },
    checked: { type: 'string' },
    notes_file: { type: 'string' },
  },
  required: ['verdict', 'fixed_confirmed', 'findings', 'checked', 'notes_file'],
}

const REVIEWERS = [
  { agentType: 'infra-engineer', label: 'review2:infra-engineer', focus: 'Your notes file: infra-engineer-review-2.md. Your round-1 findings: F1 (High) and F4 (and F2, which is a workflow-text change waiting for the owner, not part of this fix pass).' },
  { agentType: 'test-engineer', label: 'review2:test-engineer', focus: 'Your notes file: test-engineer-review-2.md. Your round-1 findings: TE-1 (High) and TE-2 to TE-6, TE-8 to TE-10 (the fix pass claims these); TE-7, TE-11, TE-12, TE-13 became results-log procedures, not code.' },
]

phase('Review 2')
const results = await parallel(REVIEWERS.map((r) => () =>
  agent(`${COMMON}\n\n${r.focus}`, { label: r.label, phase: 'Review 2', agentType: r.agentType, effort: 'high', schema: FINDINGS })
    .then((res) => ({ reviewer: r.agentType, ...(res || { verdict: 'missing', fixed_confirmed: [], findings: [], checked: '', notes_file: '' }) }))
))
log(`round 2 reviewers returned: ${results.filter(Boolean).length} of ${REVIEWERS.length}`)
return results.filter(Boolean)
