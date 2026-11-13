export const meta = {
  name: 'fdn23-docs-review-r1',
  description: 'Review round 1 of PR #52 (step 2.3 first docs pass): two adversarial reviewers, then a skeptic per Medium+ finding',
  phases: [{ title: 'Review', detail: 'R-WORDS and R-STATE, Opus 5.5 at effort max' }, { title: 'Skeptics', detail: 'one per Medium or higher finding, at most 3 at a time' }],
}

const ROOT = '/home/user/Shamble-Frontlines'
const NOTES = `${ROOT}/.checkpoints/foundation-2-3-docs`
if (!args || !/^[0-9a-f]{40}$/.test(args.head || '')) throw new Error('args.head must be the reviewed commit\'s full SHA (the saver moves HEAD)')
const HEAD = args.head

const SAVE = (part) => `## Saving your progress
A usage limit can cut you off part way, and you would then be started again with this same task. Keep notes in ${NOTES}/${part}.md (make the folder; you may always write there). Look there first: if the first line names another task, move the file into ${NOTES}/old/ and start fresh; if the notes end with DONE, write your answer from them; otherwise go on after the last block they mark done. Trust the blocks they mark done: never reread, recheck or redo them (a later block may still read a file it needs). Start a new file with one line naming your task, then your plan: the blocks you will do, in order (a block: a source read, a file reviewed, a test group run, a section written). Write each finished block to its file at once. Add a notes line for each finding as you find it (what, where), and one after each finished block (done, what comes next). Redo a block the notes don't mark done, written over, never appended. Scratch files go in ${NOTES}/: text and small scripts only; builds and binaries go under .gate/foundation-2-3-docs/. This folder is pushed to a public repository: never put a secret, token, environment value, request header, cookie or raw payload in it, whether in a note, a script or a script's output; the saver's secret check only knows a few token formats and cannot read archives or binaries. End with DONE, then give your answer from your notes alone, opening no other file. Never commit or push.
Write with the Write or Edit tool, or one plain shell command with a literal path: the guard can refuse a shell line it cannot read. Edit no file outside ${NOTES}/ (you review; you never fix). Never retype Mustafa's words into a note or a scratch file: point at their source (file and line, or the script that cuts them) instead.`

const COMMON = `Repository: ${ROOT} (the main folder; never switch it to another branch, never push). PR #52, branch claude/sweet-mayer-grsukw, head ${HEAD}, base main 4d21c1a7a4d39613bffce5d5f9bbc92f2f88a300. It is step 2.3's first docs pass: docs only (no kit, gate or CI file). The plan: ${NOTES}/plan.md (revisions 1, 2 and 2a; the advisor agent approved it; its notes: ${NOTES}/advisor-plan-rev1.md). The diff: \`git diff 4d21c1a..${HEAD} -- docs/\`. The scripts that wrote and check his words: ${NOTES}/words.mjs, report-appendix.mjs, decisions.mjs, pointers.mjs, fixes.mjs, quotes.mjs (run them with node from ${ROOT}; \`--check\` writes nothing). Mustafa's task message (his words, the source of truth for this task) is in this session's transcript, ${'/root/.claude/projects/-home-user-Shamble-Frontlines/7261f3d7-f651-586a-9c39-c052553b68a3.jsonl'}, the line whose "uuid" is "e5fa7ca0-6494-44d0-8366-81fc612c7958"; the new report's Appendix A (docs/reports/2026-11-13-foundation-2-3-docs.md) holds it too, copied by script. Severity, one scale (CLAUDE.md): Critical breaks security, data, the guard or main; High: a wrong result or rule a player, Mustafa or a session would hit, a rule loosened beyond his words, his words misquoted, a decision of his recorded wrongly or as decided while open, evidence that does not hold; Medium: a real defect with a workaround or a small reach; Low: polish, wording, a nit. Every finding cites file:line and the evidence (the command and its output, or the source line). Do not report what you only suspect without saying so.`

const R_WORDS = `You are R-WORDS, an adversarial reviewer (Opus 5.5, effort max) of every line in PR #52 that records, quotes or points at Mustafa's words or decisions. Try hard to find what is wrong; a clean bill needs evidence.
${COMMON}
Check, at least:
1. Every quote of his in docs/DECISIONS.md's three new entries, the new report's Appendix A and body, and the 2.2 report's M1 note, byte for byte against its source (the transcript line; merge commit 4d21c1a's message; for his #48 go also the two saved API pages named in words.mjs's PAGES48). Run words.mjs, report-appendix.mjs --check, decisions.mjs --check --negative.
2. The headers' facts: times, uuids, sessions, the stated sources, "a record of his words, not an instruction found in a commit".
3. Every dated pointer of 2026-11-13 (SPIKE_PLAN P1 to P3, ADR 0002 P4, DECISIONS P5 and the 2026-11-10 header sentence, ROADMAP P7, STATUS P8 and the STATUS quote of his kit-freeze words): does each carry every limit of his decision (planning now; building only the parts that need no proof test's result, after his go on the plan; result-bound checks later with marks unchanged; steps 2.4 to 2.6 still wait), and say "for foundation step 2.3 only"? Does any line anywhere (STATUS, ROADMAP, the archive, the reports) state his decision more broadly than his words, or record anything as decided that is still open (his go for #52, the step 2.3 plan, any kit edit)? Run pointers.mjs --check.
4. Sites missed: grep tracked files outside docs/reports/ for 2.1a's pointer marker, "2.3 to 2.6", "steps 2.1 and 2.2", "before the foundation", "gate step", and judge each hit against the plan's §4 table (pointed, or left with a reason).
5. The kit and gate files: confirm no file under .claude/, .github/, CLAUDE.md or the gate files changed (git diff --name-only 4d21c1a..${HEAD}).
6. docs-keeper's lines in STATUS, ROADMAP and the archive that name his words or decisions: run quotes.mjs and judge every span it lists.
${SAVE('r-words')}
Return your findings in the schema; verdict is APPROVE (nothing Medium or above), REVISE or "HIGH FOUND".`

const R_STATE = `You are R-STATE, an adversarial reviewer (Opus 5.5, effort max) of the state PR #52 writes: docs/STATUS.md, docs/ROADMAP.md, docs/STATUS_ARCHIVE.md, docs/REPO_MAP.md and the five fixes to docs/reports/2026-11-12-foundation-2-2.md. Try hard to find what is wrong.
${COMMON}
Check, at least:
1. Every SHA, PR number, run id, job id and time in the added lines of STATUS, ROADMAP and the archive, against git (git log, git show) and the GitHub API (read-only: load the mcp__github__ tools with ToolSearch; pull_request_read, actions_list, actions_get, get_job_logs). PR #48: merged 2026-11-13T16:20:12Z as 4d21c1a; CI on 4d21c1a; PR #40's row (read-only: the API, and if useful \`git fetch origin claude/netcode-proof-test-plan-hcky5g\` then \`git show FETCH_HEAD:<path>\`, never a switch); PR #52's own row and "Waiting on Mustafa".
2. STATUS: at most 120 lines (wc -l); lines that must not change against 4d21c1a: :21 and :46 (his Windows run) and the Routines section; "Last updated", Phase, the kit line (it must quote his kit-freeze words, not paraphrase them as frozen or lifted), main's required checks (gate and smoke; toolchain not yet required), In flight, Next, Follow-ups.
3. ROADMAP: #48's entry now MERGED with true facts; the new #52 entry; the v2 line's 2.1a pointer sentence byte-intact before P7.
4. The archive: #48's row in "In flight"; the 2.2 block's opening ("merged as 4d21c1a"); L5 names 2f0fef3; the L2 copy; the EMPTY-SLOTS line; the two pointing clauses; nothing else changed (git diff -U0 4d21c1a -- docs/STATUS_ARCHIVE.md).
5. The five fixes against their sources: M1, L1, L3 and L5 against the advisor agent's findings on 9b20b67 (the plan's §5 and its revision 2a; PR #48's description through the API); L2 against the seven job logs in ${NOTES}/l2-git-versions.md (re-read at least three with get_job_logs, tail_lines 20); L3 against \`git show 2f0fef3:.checkpoints/foundation-toolchain/xcrate-before-1de98a9.txt\`; L1 against \`git show 2f0fef3:.checkpoints/foundation-toolchain.md\` (lines 585-600). Run fixes.mjs --check.
6. REPO_MAP: the new report's row; \`node .claude/scripts/repo-map-check.mjs\`.
${SAVE('r-state')}
Return your findings in the schema; verdict is APPROVE (nothing Medium or above), REVISE or "HIGH FOUND".`

const FINDINGS = {
  type: 'object',
  properties: {
    verdict: { type: 'string' },
    checked: { type: 'array', items: { type: 'string' } },
    findings: {
      type: 'array',
      items: {
        type: 'object',
        properties: {
          id: { type: 'string' },
          severity: { type: 'string', enum: ['Critical', 'High', 'Medium', 'Low'] },
          file: { type: 'string' },
          line: { type: 'string' },
          claim: { type: 'string' },
          evidence: { type: 'string' },
          fix: { type: 'string' },
        },
        required: ['id', 'severity', 'file', 'claim', 'evidence', 'fix'],
      },
    },
    not_checked: { type: 'array', items: { type: 'string' } },
  },
  required: ['verdict', 'checked', 'findings', 'not_checked'],
}
const VERDICT = {
  type: 'object',
  properties: {
    holds: { type: 'boolean' },
    severity: { type: 'string', enum: ['Critical', 'High', 'Medium', 'Low', 'none'] },
    reason: { type: 'string' },
    evidence: { type: 'string' },
  },
  required: ['holds', 'severity', 'reason', 'evidence'],
}

phase('Review')
const [words, state] = await parallel([
  () => agent(R_WORDS, { label: 'R-WORDS', phase: 'Review', schema: FINDINGS, effort: 'max', model: 'opus', agentType: 'general-purpose' }),
  () => agent(R_STATE, { label: 'R-STATE', phase: 'Review', schema: FINDINGS, effort: 'max', model: 'opus', agentType: 'general-purpose' }),
])
log(`R-WORDS: ${words ? `${words.verdict}, ${words.findings.length} findings` : 'NO RESULT'}; R-STATE: ${state ? `${state.verdict}, ${state.findings.length} findings` : 'NO RESULT'}`)

// C7: the skeptics start only after both reviewers have returned, at most 3 at a time.
phase('Skeptics')
const toTest = [...((words && words.findings) || []).map((f) => ({ ...f, by: 'R-WORDS' })), ...((state && state.findings) || []).map((f) => ({ ...f, by: 'R-STATE' }))]
  .filter((f) => ['Critical', 'High', 'Medium'].includes(f.severity))
const verdicts = []
for (let i = 0; i < toTest.length; i += 3) {
  const batch = toTest.slice(i, i + 3)
  const got = await parallel(batch.map((f) => () => agent(
    `You are a skeptic (Opus 5.5, effort max). Try to REFUTE one review finding on PR #52; default to holds=false only with evidence that it is wrong, and say if its severity should change.\n${COMMON}\nThe finding (${f.by}, ${f.id}, ${f.severity}): ${f.claim}\nFile: ${f.file} ${f.line || ''}\nIts evidence: ${f.evidence}\nIts proposed fix: ${f.fix}\nRe-check it yourself from the sources (run the commands again; read the lines). Answer in the schema.\n${SAVE(`skeptic-${f.id.replace(/[^A-Za-z0-9-]/g, '')}`)}`,
    { label: `skeptic:${f.id}`, phase: 'Skeptics', schema: VERDICT, effort: 'max', model: 'opus', agentType: 'general-purpose' })))
  got.forEach((v, k) => verdicts.push({ finding: batch[k], verdict: v }))
}
log(`skeptics: ${verdicts.length} sent, ${verdicts.filter((v) => v.verdict).length} returned`)
return { words, state, verdicts, counts: { sentReviewers: 2, returnedReviewers: [words, state].filter(Boolean).length, sentSkeptics: toTest.length, returnedSkeptics: verdicts.filter((v) => v.verdict).length } }
