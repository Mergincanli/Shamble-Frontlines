export const meta = {
  name: 'preview-check-review-round',
  description: 'Review round of the preview hosting check build (setup branch of shamble-preview-check): infra-engineer, security-auditor, test-engineer',
  phases: [{ title: 'Review', detail: 'three reviewers, one round, on the built head 1b37fae' }],
}

const COMMON = `You are one of three reviewers in the single review round of a built change-set (CLAUDE.md, the change-set protocol, step 5). Review only; edit nothing outside your notes file; never commit or push; create or change nothing on GitHub or Cloudflare; run no workflow.

What was built: the preview hosting check's test repository content, on branch "setup" of Mergincanli/shamble-preview-check, head 1b37fae6fbed1591ba88e109c74e05c79e0f5fc8, cloned at /home/user/shamble-preview-check (read it there; "git log --oneline -4" shows U1 fe8116d, U2 ba3946e, U3 1b37fae on top of the owner's README commit 4a67b83). Nothing has run yet: no pull request is open, and no workflow listens to a push.
- site/: the test page (index.html, page.js, worker.js, page.css, 404.html).
- tools/: build.mjs (seven modes and their _headers), lib.mjs, check-headers.mjs (PM-0006-2), check-sizes.mjs (PM-0006-5), probe.mjs (PM-0006-6 and -8), sets.mjs + sets.json, links.json, wdio/run.mjs + package.json + package-lock.json (PM-0006-3 and -4; WebdriverIO 10.1.0).
- .github/workflows/: deploy.yml, close.yml, probe.yml, browsers.yml. These four texts were approved by the owner byte for byte (sha256 prefixes 333e5a592a05, 98b42b8e59cf, f24ad6e81daf, 6791076916ae). Any change you propose to them must be stated as an exact before/after diff; it goes back to the owner for a new yes.
The plan the build follows (revision 2, approved by the advisor agent with conditions, the owner's go on it): /home/user/Shamble-Frontlines/.checkpoints/preview-hosting-check/plan.md (Part 1 and 1.1 the marks and departures, Part 2 what is built, Part 3 the owner's steps, Part 4 security, Part 7 the workflow texts). The owner's go and conditions, byte for byte: /home/user/Shamble-Frontlines/.checkpoints/preview-hosting-check/mustafa-2026-11-15-go.md. The pass marks: /home/user/Shamble-Frontlines/docs/adr/0006-infrastructure-operations.md lines 1366-1376 and spike item 1 at 1198-1222. The doc reads behind the plan: /home/user/Shamble-Frontlines/.checkpoints/preview-hosting-check/infra-engineer-docs.md and security-auditor-docs.md. The session's local proofs (they prove the scripts, not Cloudflare): /home/user/Shamble-Frontlines/.checkpoints/preview-hosting-check/local-proof/ (proof.mjs: 20 of 20; browser-proof.mjs: 5 of 5 in Chromium 153).
Facts already established: the owner's Claude GitHub App has read-only "administration" and no "deployments" permission (his screenshot); the token sits only in the "preview" environment (required reviewer, self-review off, admin bypass off); "production" admits branch main only and holds a harmless CANARY secret.

Rate every finding on CLAUDE.md's scale (Critical, High, Medium, Low), with the file and line, a concrete failure scenario, and the smallest fix. Separate what you verified (file:line, command output) from what you infer. If you find nothing in an area, say what you checked.

## Saving your progress
A usage limit can cut you off part way, and you would then be started again with this same task. Keep notes in /home/user/Shamble-Frontlines/.checkpoints/preview-hosting-check/<agent>-review.md (make the folder; you may always write there). Look there first: if the first line names another task, move the file into /home/user/Shamble-Frontlines/.checkpoints/preview-hosting-check/old/ and start fresh; if the notes end with DONE, write your answer from them; otherwise go on after the last block they mark done. Trust the blocks they mark done: never reread, recheck or redo them (a later block may still read a file it needs). Start a new file with one line naming your task, then your plan: the blocks you will do, in order (a block: a source read, a file reviewed, a test group run, a section written). Write each finished block to its file at once. Add a notes line for each finding as you find it (what, where), and one after each finished block (done, what comes next). Redo a block the notes don't mark done, written over, never appended. Scratch files go in /home/user/Shamble-Frontlines/.checkpoints/preview-hosting-check/: text and small scripts only; builds and binaries go under /home/user/Shamble-Frontlines/.gate/preview-hosting-check/. This folder is pushed to a public repository: never put a secret, token, environment value, request header, cookie or raw payload in it, whether in a note, a script or a script's output; the saver's secret check only knows a few token formats and cannot read archives or binaries. End with DONE, then give your answer from your notes alone, opening no other file. Never commit or push.`

const FINDINGS = {
  type: 'object',
  properties: {
    verdict: { type: 'string', enum: ['no findings', 'findings'] },
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
          changes_approved_text: { type: 'boolean' },
          verified: { type: 'boolean' },
        },
        required: ['id', 'severity', 'file', 'summary', 'failure_scenario', 'fix', 'changes_approved_text', 'verified'],
      },
    },
    rulings: { type: 'string' },
    checked: { type: 'string' },
    notes_file: { type: 'string' },
  },
  required: ['verdict', 'findings', 'checked', 'notes_file'],
}

const REVIEWERS = [
  {
    agentType: 'infra-engineer',
    label: 'review:infra-engineer',
    focus: `Your area (notes file: infra-engineer-review.md): CI and hosting. Do the four workflows and the scripts work together on GitHub's runners and on Cloudflare Pages as the plan describes? Check each command line a workflow runs against the script's options and file names; the matrix built from tools/sets.mjs's output; the artifact names and download patterns (deploy writes records/$RUN-$LEG-$i.json flat, verify downloads deployment-record-<run>-* merged, probe downloads every deployment-record* artifact one folder deeper); Wrangler 4.156.0 installed in a step without the token and run from $RUNNER_TEMP/wrangler/node_modules/.bin/wrangler (does that path exist after "npm install --prefix"?); the WRANGLER_OUTPUT_FILE_PATH entry fields the deploy step reads; the production branch lookup; close.yml's listing (env=preview, per_page 25 paging) and forced deletions and the tombstone path; probe.yml's gh api calls; browsers.yml on windows-2025 and macos-26 (bash, safaridriver, npm ci --ignore-scripts, the preinstalled drivers through CHROMEWEBDRIVER, EDGEWEBDRIVER, GECKOWEBDRIVER); what happens when a download pattern matches no artifact (the "revoked" run). Say for each workflow whether a mark could become unmeasurable.`,
  },
  {
    agentType: 'security-auditor',
    label: 'review:security-auditor',
    focus: `Your area (notes file: security-auditor-review.md): the token and CI security, the B9 tests (PM-0006-32), and the owner's condition 2, word for word: "In the build's review, have the security-auditor rule on the deploy job's "deployments: write": say what needs it, or remove it and show me the changed line." Note for that ruling: no GitHub page read so far says whether the deployment record and the environment URL need "deployments: write" (your own G6 notes: "not stated"); the setup pull request's rehearsal run could show it. Rule: keep it (and say what needs it), or remove it (give the exact before/after line of deploy.yml). Also check: every value from a pull request reaches a shell only through env; the token appears only in the deploy and tombstone steps and is never printed or written anywhere but the 0600 header file; no token-holding job checks out or runs repository code; the install-without-token step (the advisor agent's A1) and its residual risk; the "refuse server code" step; the tombstone and cleanup steps cannot delete production or another project; probe.yml and browsers.yml hold no secret; the B9 variants in the plan's Part 7.5 test what PM-0006-32 needs; the scripts never print headers, cookies or bodies that could hold secrets.`,
  },
  {
    agentType: 'test-engineer',
    label: 'review:test-engineer',
    focus: `Your area (notes file: test-engineer-review.md): is every mark proven, and does every bite bite? For PM-0006-1 to -10 and -32, map what each script or workflow measures to the mark's number, statistic, counts, controls and bites as the draft writes them (ADR 0006 lines 1366-1376) and as Part 1.1 plans them; find any mark that cannot pass, cannot fail, or could be read as a pass when it was really "no reading" (a transport error, a 5xx, the 1010 block, a stale cache, a skipped job). Check the expected header sets in check-headers.mjs against GLOSSARY section 5 (/home/user/Shamble-Frontlines/docs/research/GLOSSARY.md lines 636-665); the probe's classification and its expectations per case (a), (b), (c) control, (d) bite, (a') and (b') tombstone, and the wall purpose; wdio/run.mjs's 3 loads per browser and link, the bite-coep reading, and the cache test (first visit, tab closed, second visit, transferSize); the local proofs' coverage and what they do not prove.`,
  },
]

phase('Review')
const results = await parallel(REVIEWERS.map((r) => () =>
  agent(`${COMMON}\n\n${r.focus}`, { label: r.label, phase: 'Review', agentType: r.agentType, effort: 'high', schema: FINDINGS })
    .then((res) => ({ reviewer: r.agentType, ...(res || { verdict: 'missing', findings: [], checked: '', notes_file: '' }) }))
))
const returned = results.filter(Boolean)
log(`reviewers returned: ${returned.length} of ${REVIEWERS.length}`)
return returned
