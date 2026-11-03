export const meta = {
  name: 'a1-docs-and-security-review',
  description: 'A1 (PR #30): docs-keeper updates three docs then a verifier checks them; security-auditor plus an adversarial lens review the GUARD-FILE fix, each finding checked by a skeptic',
  phases: [
    { title: 'Docs', detail: 'docs-keeper edits STATUS, ROADMAP, REPO_MAP in the A1 worktree' },
    { title: 'Verify docs', detail: 'independent check of the doc diff against the brief' },
    { title: 'Security review', detail: 'security-auditor and an adversarial bypass hunter on the fix' },
    { title: 'Check findings', detail: 'one skeptic per finding tries to reproduce or refute it' },
  ],
}

const WT = args.wt
const NOTES = WT + '/.checkpoints/kit-v3-1'
const SCRATCH = args.scratch
const save = (file) => `\n\n## Saving your progress\nA usage limit can cut you off part way, and you would then be started again with this same task. Keep notes in ${NOTES}/${file} (make the folder; you may always write there). Look there first: if the first line names another task, move the file into ${NOTES}/old/ and start fresh; if the notes end with DONE, write your answer from them; otherwise go on from where they stop. Start a new file with one line naming your task. After each unit you finish (a source read, a file reviewed, a test group run), add a line or two: what is done, what you found and where, what comes next. End with DONE, then give your answer. Never commit or push.`

const CONTEXT = `## Context (all checkable with git in the worktree)
- Change-set A1 (Mustafa, 2026-11-03): "Update PR #30 (kit-v3-1) onto main, resolve its conflicts, have it re-reviewed, then wait for my go." GUARD-FILE (a), decided by Mustafa: "fix it inside #30 as part of A1. When guard.mjs is missing, a day session is let through and only a routine is blocked. Add a test that removes the file. security-auditor reviews that change, then the advisor re-reviews the whole updated PR."
- Why: on 2026-11-03 a day session did a trial merge of PR #30 in its main folder, which loaded #30's .claude/settings.json (the guard hooks, "onFailure": "block"). Switching back removed .claude/hooks/guard.mjs, but the session kept the hook settings it had loaded; node then failed with "Cannot find module" on every call, every failure became a block, the ConfigChange hook blocked the reload of main's settings too, and the session froze.
- The A1 worktree: ${WT} (branch cs/kit-v3-1 = PR #30, https://github.com/Mergincanli/Shamble-Frontlines/pull/30, a draft while A1 runs, labelled needs-mustafa). Commits: af0c5dc merges main e4d8b22 into it (conflicts only in docs/STATUS.md, docs/ROADMAP.md, docs/REPO_MAP.md, resolved to main's side); 537c7a8 is the GUARD-FILE fix; later commits are checkpoints only. #30's head before A1 was 5be7d37.
- The plan: ${NOTES}/a1-plan.md (revision 2); the advisor's APPROVE with conditions C1-C9: ${NOTES}/advisor-plan-rev2.md; the checkpoint: ${WT}/.checkpoints/kit-v3-1.md.
- Never switch /home/user/Shamble-Frontlines (this session's main folder, on main) to another branch or commit; never commit or push; never run git checkout in any folder. Scratch experiments go under ${SCRATCH}/<your-own-folder>/ only.`

// ---------------- Track A: docs ----------------
const DK_PROMPT = `You are updating the docs for change-set A1 (PR #30, kit-v3-1) in its worktree. Edit ONLY these three files, by absolute path: ${WT}/docs/STATUS.md, ${WT}/docs/ROADMAP.md, ${WT}/docs/REPO_MAP.md. Never edit anything under /home/user/Shamble-Frontlines; the session commits your edits.

${CONTEXT}

## Facts for the docs
- 537c7a8, the fix: the three guard handlers in .claude/settings.json (PreToolUse, PermissionRequest, ConfigChange) now run \`node -e <bootstrap> \${CLAUDE_PROJECT_DIR}/.claude/hooks/guard.mjs\`. The one-line bootstrap imports guard.mjs; if guard.mjs is missing or can't load, a day session goes on (exit 0, no decision) and a routine is blocked (exit 1, which "onFailure": "block" turns into a block). When guard.mjs loads, it runs as before. Tests: .claude/hooks/settings.test.mjs 98/98 (16 new, including the hook command run with guard.mjs removed, broken and present; the same file fails the day-session cases on 5be7d37's settings); routine-guard.test.mjs 192/192 on Linux, unchanged; 183/192 on Windows (9 pre-existing failures, path separators: the "Q3" follow-up).
- After A1: security-auditor reviews the fix, the advisor reviews the whole PR at its final head, CI gate and smoke must be green, then Mustafa's go ("merge #30"). The PR stays needs-mustafa.
- PR #33 merged on 2026-11-03 at 21:57 UTC (squash, e4d8b22). Main's STATUS/ROADMAP still call it "ready for review, not merged": leave those lines exactly as they are (a separate docs-after-33 pass will fix them; the advisor's ruling Q2), and never call #33 "not merged" in anything you write (condition C5).

## Edits
1. STATUS, "In flight" table: the kit-v3-1 row (about line 69) still shows the original scope ("write guards for kit and gate files, a builder agent, a default effort, the secret-pattern fix") and the state of 2026-10-31. Rewrite that one row. Work: #30's real scope (source: #30's own row on its branch, \`git -C ${WT} show 5be7d37:docs/STATUS.md\`, the row starting "| **kit-v3-1** (routine guard for the two cloud routines.") plus the GUARD-FILE fix. Branch: \`cs/kit-v3-1\`, PR #30 (a draft while A1 runs). State: updated onto main e4d8b22 (af0c5dc, 2026-11-04) and the GUARD-FILE (a) fix (537c7a8); re-review: security-auditor on the fix, then the advisor on the whole PR. Waiting on: the advisor's APPROVE of the final head with gate and smoke green, then Mustafa's go; after the merge, his setup steps (item 3). One table row, four columns.
2. STATUS: record the docs-after-33 follow-up in one short line or row where it fits (for example a row "docs-after-33 (docs only: STATUS and ROADMAP record PR #33's merge, e4d8b22, 2026-11-03)" with state "not started"). Don't edit #33's own lines.
3. STATUS: #30's post-merge steps are on no dashboard yet. From #30's report (${WT}/docs/reports/2026-10-27-kit-v3-1.md, "For Mustafa" items 2-5): two routine-only environments that set SHAMBLE_ROLE=night-shift and SHAMBLE_ROLE=pr-advisor, pinning Claude Code >= v2.1.295, with the two routines pointed at them (the night-shift cron \`CRON_TZ=Europe/Amsterdam 10 0-7 * * *\`); the PR-HOME setup script (reviewed before use); the TEST-RUN (a routine refuses a disallowed action and allows the legitimate ones); then the PR Advisor routine back on. Put them where Mustafa sees them as "after #30 merges".
4. STATUS: the Q3 follow-up (advisor condition C6), one line where open risks or follow-ups live: routine-guard.test.mjs fails 9 of 192 on Windows (pre-existing: the credential checks match forward-slash paths only); CI does not run the hook tests (.github/workflows runs only tools/gate.mjs and smoke); the follow-up: separator-agnostic path handling in routine-guard.mjs, and the hook tests in CI.
5. STATUS: sweep every other line that mentions kit-v3-1 or PR #30 (about lines 125, 186, 274, 284, 292, 297, 302-313, 320, 362). Dated decision records ("Taken by ...") stay as written. A line that states the current state and is no longer true gets fixed (for example one that still says #30 conflicts with main or names head 5be7d37 as current). Report every line you checked and what you did.
6. ROADMAP: put #30's "Kit v3.1 — routine guard" Recent-work entry back at the top of "Recent work" (its text: \`git -C ${WT} show 5be7d37:docs/ROADMAP.md\`), updated: PR #30, updated onto main e4d8b22 and the GUARD-FILE fix on 2026-11-04, not merged yet. A few lines, in the file's style.
7. ROADMAP: the merge left a doubled Build-history entry. The line starting "- **Docs: loop v2 (2026-10-15, merged):**" (about line 49, #30's copy) duplicates main's line starting "- **Docs loop v2, team prep fixes, map dedupe (2026-10-15/16):**" (about line 53). Delete #30's line; keep main's untouched.
8. REPO_MAP: "Last checked: 2026-11-04". The \`.claude/settings.json\` row says the guard's three handlers start it through a one-line \`node -e\` bootstrap. The \`.claude/hooks/guard.mjs\` row: the launcher; when it is missing or can't load, the settings.json bootstrap lets a day session through and blocks a routine. The \`.claude/hooks/settings.test.mjs\` row also names the GUARD-FILE tests (the hook command run with guard.mjs removed, broken and present). Then run \`node .claude/scripts/repo-map-check.mjs\` with ${WT} as the working directory, and report its output line.
Style: plain words, short, as the files read now; decimals with periods; table rows stay on one line; claim no line number you didn't check.
${save('docs-keeper-a1.md')}

## Answer
Each edit (file:line, one line each); the sweep for item 5 (line, what it says, kept or fixed); the repo-map check's line; anything you didn't do, and why.`

const DOCS_CHECK = {
  type: 'object',
  properties: {
    pass: { type: 'boolean' },
    checks: { type: 'array', items: { type: 'object', properties: { n: { type: 'integer' }, pass: { type: 'boolean' }, evidence: { type: 'string' } }, required: ['n', 'pass', 'evidence'] } },
    problems: { type: 'array', items: { type: 'string' } },
  },
  required: ['pass', 'checks', 'problems'],
}
const VERIFY_DOCS_PROMPT = (dkAnswer) => `Independently check docs-keeper's uncommitted edits for change-set A1 in the worktree ${WT}. Don't trust its summary; read the diff yourself: \`git -C ${WT} diff -- docs/\` and \`git -C ${WT} status --short\`. Edit nothing.

${CONTEXT}

docs-keeper's own summary (evidence, not truth):
<<<
${String(dkAnswer || '(none returned)').slice(0, 6000)}
>>>

Checks (one result each, with evidence: the diff hunk, a command and its output):
1. Only docs/STATUS.md, docs/ROADMAP.md and docs/REPO_MAP.md are modified (nothing else modified or untracked outside .checkpoints/).
2. Nothing docs-keeper added calls PR #33 "not merged" or "ready for review"; #33's existing lines are unchanged; a docs-after-33 follow-up is recorded.
3. STATUS's In-flight table has exactly one kit-v3-1 row; it describes the routine guard (not "write guards for kit and gate files, a builder agent ..."), names af0c5dc and 537c7a8 correctly (check with git log), names PR #30 and Mustafa's go; every row of that table keeps four columns (count the unescaped pipes).
4. #30's post-merge steps (two routine-only environments with SHAMBLE_ROLE, Claude Code >= v2.1.295, PR-HOME, the TEST-RUN, the PR Advisor routine back on) appear in STATUS.
5. The Q3 follow-up appears (9 of 192 on Windows; CI doesn't run the hook tests; separator-agnostic paths and the hook tests in CI).
6. ROADMAP: a "Kit v3.1 — routine guard" entry is at the top of "Recent work" and is accurate; the doubled "Docs: loop v2 (2026-10-15, merged)" Build-history line is gone, and main's "Docs loop v2, team prep fixes, map dedupe" line is unchanged.
7. REPO_MAP: "Last checked: 2026-11-04"; the settings.json, guard.mjs and settings.test.mjs rows describe the bootstrap and the GUARD-FILE tests accurately (compare with ${WT}/.claude/settings.json and the test file); \`node .claude/scripts/repo-map-check.mjs\` run in ${WT} passes.
8. Every fact docs-keeper added is true against the repository (SHAs, the 98/98 and 192/192 counts, dates, file names): list any that isn't.
9. No other line of main's docs changed: every hunk is one of the edits above or a sweep fix; list any other hunk.
${save('verifier-a1-docs.md')}

Return pass=true only if every check passes.`

// ---------------- Track B: security ----------------
const FINDINGS = {
  type: 'object',
  properties: {
    verdict: { type: 'string', enum: ['PASS', 'PASS WITH NOTES', 'FAIL'] },
    summary: { type: 'string' },
    findings: { type: 'array', items: { type: 'object', properties: {
      id: { type: 'string' },
      severity: { type: 'string', enum: ['Critical', 'High', 'Medium', 'Low', 'Note'] },
      title: { type: 'string' },
      where: { type: 'string' },
      evidence: { type: 'string' },
      introducedByFix: { type: 'boolean' },
      recommendation: { type: 'string' },
    }, required: ['id', 'severity', 'title', 'where', 'evidence', 'introducedByFix', 'recommendation'] } },
    onF5: { type: 'string' },
    onF6: { type: 'string' },
    onQ3: { type: 'string' },
    verifiedYourself: { type: 'array', items: { type: 'string' } },
    couldNotVerify: { type: 'array', items: { type: 'string' } },
  },
  required: ['verdict', 'summary', 'findings', 'onF5', 'onF6', 'onQ3', 'verifiedYourself', 'couldNotVerify'],
}
const ITEMS = `## Put in front of you (weigh each; answer onF5, onF6 and onQ3 explicitly)
- F5 (honesty): with guard.mjs missing or broken, a day session now runs with no guard checks at all: the git/gh option-form checks and the fail-closed merge check are off too. Before the fix, a missing guard froze the day session, so the merge check was effectively tamper-evident; after it, a day session that deletes or breaks guard.mjs (for example under prompt injection) goes on with no merge check. Weigh it against R3c (a broken routine-guard.mjs already lets a day session through with no checks, by design: "a broken rules file never locks a day session out of fixing it"), against the static deny list (\`gh pr merge\` is denied; the GitHub MCP merge tool is not statically denied, only the guard's mergeCheck covers it), and against Mustafa's decision, which says a day session is let through.
- F6 (pre-existing, not introduced, as far as the session knows): an empty guard.mjs (it loads and decides nothing) lets a routine through in the old and the new form alike; \`node -e\` under NODE_OPTIONS module-type flags behaves the same in old and new; whether Claude Code minds a hook that exits before reading a large stdin (EPIPE) is host behaviour, unverifiable here. Any change to the bootstrap needs the advisor, because condition C3 pins its exact text.
- Q3 (accepted by the advisor as a follow-up, condition C6): routine-guard.test.mjs fails 9 of 192 on Windows (pre-existing; the credential checks match forward-slash paths only); CI does not run the hook tests; routine-guard.mjs is untouched in A1.
- The routine side rests on documented "onFailure": "block" behaviour (Claude Code v2.1.295 or later; already a condition of #30). The official hooks reference is quoted verbatim in \`git -C ${WT} show 2a75345:.checkpoints/kit-v3-1/hook-docs.md\` (fetched 2026-10-27). Do not fetch the web: it is the night window, and code.claude.com is not on the allow list, so a fetch would be refused. Say what you could not check live.`

const SA_PROMPT = `Security review of one change: the GUARD-FILE fix in PR #30, commit 537c7a8. Review the diff \`git -C ${WT} diff 31e8871 537c7a8\` (.claude/settings.json: the three guard handlers' args; .claude/hooks/settings.test.mjs: the new tests; .claude/hooks/guard.mjs: the header comment only), in the context of the guard it launches (${WT}/.claude/hooks/guard.mjs and routine-guard.mjs; their behaviour is unchanged since 5be7d37 and was reviewed in #30's original change-set).

${CONTEXT}

${ITEMS}

## What to establish
- Can the new hook command ever let a routine (SHAMBLE_ROLE set, any value) through when guard.mjs is missing, broken or unloadable? Can it freeze or block a day session in a way the fix claims to prevent? Run the hook command yourself as Claude Code's exec form would (args passed as-is, \${CLAUDE_PROJECT_DIR} substituted as a plain string, no shell), in scratch copies under ${SCRATCH}/security-auditor/.
- Is the bootstrap's text safe against placeholder substitution and shell interpretation on every platform the project uses (Linux in the cloud, Windows on Mustafa's PC)?
- Do the new tests faithfully prove the security claim (that they would catch a regression of the freeze, and of the routine side)? Run \`node --test .claude/hooks/settings.test.mjs\` and \`node --test .claude/hooks/routine-guard.test.mjs\` with ${WT} as the working directory.
- Anything else in the fix a reviewer of hook code should catch.
${save('security-auditor-a1.md')}

## Answer
A verdict (PASS, PASS WITH NOTES or FAIL); findings ranked by severity, each with where (file:line), evidence (a command and its output, or a quote), whether the fix introduced it, and a concrete recommendation; your explicit position on F5, F6 and Q3; what you verified yourself; what you could not verify and why.`

const HUNTER_PROMPT = `Adversarial check of one change: the GUARD-FILE fix in PR #30 (commit 537c7a8, \`git -C ${WT} diff 31e8871 537c7a8\`). Your only job is to break it. Find a concrete file state, environment or hook input under which EITHER (1) a routine (SHAMBLE_ROLE set) gets past the three guard hooks when it should be blocked, OR (2) a day session (no SHAMBLE_ROLE) gets blocked or frozen by them because guard.mjs is missing, broken or replaced. Run every idea as an experiment: the handler's args exactly as in ${WT}/.claude/settings.json, \${CLAUDE_PROJECT_DIR} replaced by a scratch project folder, started with node directly and no shell, input on stdin as Claude Code sends it (PreToolUse, PermissionRequest and ConfigChange shapes; see \`git -C ${WT} show 2a75345:.checkpoints/kit-v3-1/hook-docs.md\`). Exit 0 with empty output is "no decision" (the call goes on); exit 1 or any other non-0, non-2 exit is a hook failure that "onFailure": "block" turns into a block; a JSON deny or block on stdout with exit 0 is a refusal. Work only under ${SCRATCH}/bypass-hunter/.

${CONTEXT}

Tell apart what the fix introduced from what was already true at 5be7d37 (run the old handler args too: \`git -C ${WT} show 5be7d37:.claude/settings.json\`). Only report what you reproduced; for each, the exact commands and outputs.
${save('bypass-hunter-a1.md')}`

const VOTE = {
  type: 'object',
  properties: {
    real: { type: 'boolean' },
    reproduced: { type: 'boolean' },
    introducedByFix: { type: 'boolean' },
    severity: { type: 'string', enum: ['Critical', 'High', 'Medium', 'Low', 'Note', 'None'] },
    inScopeForA1: { type: 'boolean' },
    reasoning: { type: 'string' },
    repro: { type: 'string' },
  },
  required: ['real', 'reproduced', 'introducedByFix', 'severity', 'inScopeForA1', 'reasoning', 'repro'],
}
const SKEPTIC_PROMPT = (f, lens) => `A reviewer (${lens}) reported this finding about the GUARD-FILE fix in PR #30 (commit 537c7a8, worktree ${WT}). Try to REFUTE it. Reproduce it yourself from scratch under ${SCRATCH}/skeptic-${f.id}/ (never trust the reviewer's output), and decide: is it real; does it reproduce; did the fix introduce it, or was it already true at 5be7d37 (check the old handler args, \`git -C ${WT} show 5be7d37:.claude/settings.json\`); its real severity; and whether A1 must act on it (in scope: the fix and its tests; out of scope: reviewed guard code A1 must not touch under condition C6, or a decision Mustafa already took). If you cannot reproduce it, say real=false unless the code plainly shows it.

${CONTEXT}

The finding:
<<<
${JSON.stringify(f, null, 2).slice(0, 5000)}
>>>
${save('skeptic-a1-' + f.id + '.md')}`

const SEC_LENSES = [
  { key: 'security-auditor', prompt: SA_PROMPT, agentType: 'security-auditor', schema: FINDINGS },
  { key: 'bypass-hunter', prompt: HUNTER_PROMPT, agentType: 'general-purpose', schema: FINDINGS },
]

const [docs, security] = await parallel([
  async () => {
    const dk = await agent(DK_PROMPT, { label: 'docs-keeper', phase: 'Docs', agentType: 'docs-keeper', effort: 'high' })
    const check = await agent(VERIFY_DOCS_PROMPT(dk), { label: 'verify-docs', phase: 'Verify docs', agentType: 'verifier', effort: 'high', schema: DOCS_CHECK })
    return { dk, check }
  },
  async () => pipeline(
    SEC_LENSES,
    (lens) => agent(lens.prompt, { label: lens.key, phase: 'Security review', agentType: lens.agentType, effort: 'high', schema: lens.schema }),
    (review, lens) => {
      const fs = (review && review.findings) || []
      log(`${lens.key}: ${review ? review.verdict : 'no result'}, ${fs.length} findings`)
      return parallel(fs.map((f) => () =>
        agent(SKEPTIC_PROMPT({ ...f, id: lens.key + '-' + f.id }, lens.key), { label: `skeptic:${lens.key}:${f.id}`, phase: 'Check findings', agentType: 'general-purpose', effort: 'high', schema: VOTE })
          .then((v) => ({ finding: f, vote: v }))
      )).then((votes) => ({ lens: lens.key, review, votes }))
    },
  ),
])
return { docs, security }
