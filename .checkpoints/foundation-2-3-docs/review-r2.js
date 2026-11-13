export const meta = {
  name: 'fdn23-docs-review-r2',
  description: 'Review round 2 of PR #52 (step 2.3 first docs pass), on what changed since round 1: two reviewers, then a skeptic per Medium+ finding',
  phases: [{ title: 'Review', detail: 'R-WORDS and R-STATE, Opus 5.5 at effort max' }, { title: 'Skeptics', detail: 'one per Medium or higher finding, at most 3 at a time' }],
}

const ROOT = '/home/user/Shamble-Frontlines'
const NOTES = `${ROOT}/.checkpoints/foundation-2-3-docs`
if (!args || !/^[0-9a-f]{40}$/.test(args.head || '') || !/^[0-9a-f]{40}$/.test(args.base || '')) throw new Error('args.base and args.head must be full SHAs (the saver moves HEAD)')
const HEAD = args.head
const RBASE = args.base

const SAVE = (part) => `## Saving your progress
A usage limit can cut you off part way, and you would then be started again with this same task. Keep notes in ${NOTES}/${part}.md (make the folder; you may always write there). Look there first: if the first line names another task, move the file into ${NOTES}/old/ and start fresh; if the notes end with DONE, write your answer from them; otherwise go on after the last block they mark done. Trust the blocks they mark done: never reread, recheck or redo them (a later block may still read a file it needs). Start a new file with one line naming your task, then your plan: the blocks you will do, in order (a block: a source read, a file reviewed, a test group run, a section written). Write each finished block to its file at once. Add a notes line for each finding as you find it (what, where), and one after each finished block (done, what comes next). Redo a block the notes don't mark done, written over, never appended. Scratch files go in ${NOTES}/: text and small scripts only; builds and binaries go under .gate/foundation-2-3-docs/. This folder is pushed to a public repository: never put a secret, token, environment value, request header, cookie or raw payload in it, whether in a note, a script or a script's output; the saver's secret check only knows a few token formats and cannot read archives or binaries. End with DONE, then give your answer from your notes alone, opening no other file. Never commit or push.
Write with the Write or Edit tool, or one plain shell command with a literal path: the guard can refuse a shell line it cannot read. Edit no file outside ${NOTES}/ (you review; you never fix). Never retype Mustafa's words into a note or a scratch file: point at their source (file and line, or the script that cuts them) instead.`

const COMMON = `Repository: ${ROOT} (the main folder; never switch it to another branch, never push). PR #52, branch claude/sweet-mayer-grsukw, head ${HEAD}, base main 4d21c1a7a4d39613bffce5d5f9bbc92f2f88a300. It is step 2.3's first docs pass: docs only (no kit, gate or CI file). The plan: ${NOTES}/plan.md (revisions 1, 2, 2a and 2b; the advisor agent approved revision 1 and ruled 2a and 2b; its notes: ${NOTES}/advisor-plan-rev1.md, the 2a and 2b rulings in its addendum lines). THIS IS REVIEW ROUND 2, ON WHAT CHANGED ONLY: \`git diff ${RBASE}..${HEAD} -- docs/\` (round 1 reviewed ${RBASE}; round 1's fix pass is ${HEAD}). The whole PR's diff, for context only: \`git diff 4d21c1a..${HEAD} -- docs/\`. Round 1's 23 findings and their dispositions are in the report's "Reviews" table (docs/reports/2026-11-13-foundation-2-3-docs.md); confirm each fix landed and reads right, and look for anything the fix pass broke or left out. Settled by the advisor agent's revision 2c, not to be raised again: L-a's suggested text refused (it stated step 2.3's unapproved plan as fact); the 2.2 report's M1 note keeps the DECISIONS entry's title time; the register's OQ-0006-13 row left (the preview step, 2.6, still waits); P8 a quote after STATUS's lead-in; STATUS's kit bullet as pointers plus his kit words. The scripts that wrote and check his words: ${NOTES}/words.mjs, report-appendix.mjs, decisions.mjs, pointers.mjs, fixes.mjs, quotes.mjs (run them with node from ${ROOT}: words.mjs, quotes.mjs and scope.mjs only read; run report-appendix.mjs, decisions.mjs, pointers.mjs and fixes.mjs ONLY with \`--check\`, since without it they write; never run sync-fixes.mjs, step5-fixes.mjs or etask-header.mjs, which write). Mustafa's task message (his words, the source of truth for this task) is in this session's transcript, ${'/root/.claude/projects/-home-user-Shamble-Frontlines/7261f3d7-f651-586a-9c39-c052553b68a3.jsonl'}, the line whose "uuid" is "e5fa7ca0-6494-44d0-8366-81fc612c7958"; the new report's Appendix A (docs/reports/2026-11-13-foundation-2-3-docs.md) holds it too, copied by script. Severity, one scale (CLAUDE.md): Critical breaks security, data, the guard or main; High: a wrong result or rule a player, Mustafa or a session would hit, a rule loosened beyond his words, his words misquoted, a decision of his recorded wrongly or as decided while open, evidence that does not hold; Medium: a real defect with a workaround or a small reach; Low: polish, wording, a nit. Every finding cites file:line and the evidence (the command and its output, or the source line). Do not report what you only suspect without saying so. Pending by design, so not a finding in themselves (judge only whether they are honestly worded): #52's own state word in STATUS's row and ROADMAP's entry, #40's row (its second read), the report header's {{NOTES-SHA}}, the report's "Round 2 ... to come" line, the advisor tool's consultation list (more to come), and the fix pass's own CI runs (read after its push).`

const R_WORDS = `You are R-WORDS, an adversarial reviewer (Opus 5.5, effort max) of every line in PR #52 that records, quotes or points at Mustafa's words or decisions. Try hard to find what is wrong; a clean bill needs evidence.
${COMMON}
Check, at least, on what changed:
1. E-TASK's header (docs/DECISIONS.md, the 2026-11-13 18:47:55 entry): its new wording ("his words of 2026-11-13 are the rule there too, and step 2.3's plan is to list their edits") equals decisions.mjs's head, reads as his decision says, and no quote below it changed (decisions.mjs --check --negative; git diff ${RBASE}..${HEAD} -- docs/DECISIONS.md).
2. STATUS's kit quote: byte-equal to his paragraph from "The kit's freeze (KIT-FREEZE-2) ends with this step." to its last sentence "No such edit happens in the docs pass or while planning." in A2 (the transcript line, uuid e5fa7ca0-6494-44d0-8366-81fc612c7958), and no limit of his dropped (pointers.mjs --check; your own byte check).
3. The report's new text that states or rests on his rules: its Outcome's advisor-tool gap and its "The advisor tool" section against his words of 2026-11-11 on the advisor tool (docs/DECISIONS.md; his five moments), and nothing in the report or the docs recording any decision of his wrongly or as given while open (his go for #52, the step 2.3 plan, any kit edit).
4. Your round-1 findings RW-F1 to RW-F6: each fixed as the report's "Reviews" table says, or rightly left.
5. Run words.mjs, quotes.mjs, report-appendix.mjs --check, decisions.mjs --check --negative, pointers.mjs --check and fixes.mjs --check on the tree; the notes commit just before ${HEAD} holds the scripts as used.
${SAVE('r2-words')}
Return your findings in the schema; verdict is APPROVE (nothing Medium or above), REVISE or "HIGH FOUND".`

const R_STATE = `You are R-STATE, an adversarial reviewer (Opus 5.5, effort max) of the state PR #52 writes: docs/STATUS.md, docs/ROADMAP.md, docs/STATUS_ARCHIVE.md, the five fixes to docs/reports/2026-11-12-foundation-2-2.md, and the new report's body (docs/reports/2026-11-13-foundation-2-3-docs.md above its Appendix A). docs/REPO_MAP.md must be unchanged against the base (revision 2b: no row). Try hard to find what is wrong.
${COMMON}
Check, at least, on what changed:
1. The advisor-tool timeline in the report ("Outcome" and "The advisor tool"): re-derive it yourself. The calls are the session transcript's assistant entries holding a server_tool_use or tool_use named "advisor" (their "timestamp"); the "step N done" saves are \`git log --format='%h %cd %s' --date=format-local:'%H:%M:%S' 4d21c1a..${HEAD}\` (TZ=UTC). Is every gap named, and is any claim of the report about it wrong?
2. The refusals in the report ("Decisions and deviations"): re-derive them from every transcript: the session's (the path in COMMON) and its agents' (/root/.claude/projects/-home-user-Shamble-Frontlines/7261f3d7-f651-586a-9c39-c052553b68a3/subagents/agent-*.jsonl and subagents/workflows/*/agent-*.jsonl): tool_result entries with is_error set whose text is a hook's or the guard's refusal. Is any missing, or any route wrong?
3. The archive's changed lines (git diff --word-diff=porcelain ${RBASE}..${HEAD} -- docs/STATUS_ARCHIVE.md) against their sources: the line numbers (grep the tree), the run ids (merge commit 4d21c1a's message), the time stamps (the merge commit; the saved page named in words.mjs's PAGES48).
4. STATUS: at most 120 lines; the main line's new wording; #52's row; base lines 21 and 46 and the Routines section unchanged against 4d21c1a.
5. Your round-1 findings RS-F1 to RS-L-k: each fixed as the report's "Reviews" table says, or rightly left; the report's other new text (E4, E7, E8, C8, "What changed").
6. ROADMAP's #52 entry; the 2.2 report's L1 sentence; fixes.mjs --check (its whole-file check of the 2.2 report).
${SAVE('r2-state')}
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
  () => agent(R_WORDS, { label: 'R2-WORDS', phase: 'Review', schema: FINDINGS, effort: 'max', model: 'opus', agentType: 'general-purpose' }),
  () => agent(R_STATE, { label: 'R2-STATE', phase: 'Review', schema: FINDINGS, effort: 'max', model: 'opus', agentType: 'general-purpose' }),
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
    `You are a skeptic (Opus 5.5, effort max). Try to REFUTE one review finding on PR #52; default to holds=false only with evidence that it is wrong, and say if its severity should change.\n${COMMON}\nThe finding (${f.by}, ${f.id}, ${f.severity}): ${f.claim}\nFile: ${f.file} ${f.line || ''}\nIts evidence: ${f.evidence}\nIts proposed fix: ${f.fix}\nRe-check it yourself from the sources (run the commands again; read the lines). Answer in the schema.\n${SAVE(`r2-skeptic-${f.by}-${f.id.replace(/[^A-Za-z0-9-]/g, '')}`)}`,
    { label: `r2-skeptic:${f.by}:${f.id}`, phase: 'Skeptics', schema: VERDICT, effort: 'max', model: 'opus', agentType: 'general-purpose' })))
  got.forEach((v, k) => verdicts.push({ finding: batch[k], verdict: v }))
}
log(`skeptics: ${verdicts.length} sent, ${verdicts.filter((v) => v.verdict).length} returned`)
return { words, state, verdicts, counts: { sentReviewers: 2, returnedReviewers: [words, state].filter(Boolean).length, sentSkeptics: toTest.length, returnedSkeptics: verdicts.filter((v) => v.verdict).length } }
