export const meta = {
  name: 'foundation-21-inventory',
  description: 'Classify every legacy-path reference for foundation step 2.1 (3 area agents, then a completeness critic)',
  phases: [
    { title: 'Classify', detail: 'three area agents: code+CI+hooks, kit instructions, docs' },
    { title: 'Critic', detail: 'one completeness critic over the three results' },
  ],
}

const NOTES = '.checkpoints/foundation'
const SAVE = (agent) => `## Saving your progress
A usage limit can cut you off part way, and you would then be started again with this same task. Keep notes in
${NOTES}/${agent}.md (make the folder; you may always write there). Look there first: if the first line names
another task, move the file into ${NOTES}/old/ and start fresh; if the notes end with DONE, write your answer from
them; otherwise go on after the last block they mark done. Trust the blocks they mark done: never reread, recheck
or redo them (a later block may still read a file it needs). Start a new file with one line naming your task, then
your plan: the blocks you will do, in order (a block: a source read, a file reviewed, a test group run, a section
written). Write each finished block to its file at once. Add a notes line for each finding as you find it (what,
where), and one after each finished block (done, what comes next). Redo a block the notes don't mark done, written
over, never appended. Scratch files go in ${NOTES}/: text and small scripts only; builds and binaries go under
.gate/foundation/. This folder is pushed to a public repository: never put a secret, token, environment value,
request header, cookie or raw payload in it, whether in a note, a script or a script's output; the saver's secret
check only knows a few token formats and cannot read archives or binaries. End with DONE, then give your answer
from your notes alone, opening no other file. Never commit or push.`

const COMMON = `You are one of three read-only inventory agents for SHAMBLE FRONTLINES foundation step 2.1, the restructure: the five
legacy files (shamble-core.js, shamble-game.js, shamble-editor.js, index.html, editor.html) move with git mv into legacy/.
This session PLANS ONLY: you edit no file except your own notes under ${NOTES}/, you move nothing, you run nothing that
writes outside ${NOTES}/ and .gate/foundation/. Read ${NOTES}/facts.md FIRST and trust it (layout assumed: OQ-0002-2 (a),
the legacy tools stay in tools/). Use its classes: BREAKS, COMMAND, STALE, NAME, CITATION, HISTORY, TESTDATA. The
PROTECTED column is already decided by the guard's protectedRel (listed in facts.md): copy it, do not judge it.
Everything you report is from reading (grep and read only, nothing was run after a move): say so where it matters.
Cite file:line for every claim. Be exhaustive within your area and brief in wording: plain words, short lines.`

const FILE_SCHEMA = {
  type: 'object',
  properties: {
    files: { type: 'array', items: { type: 'object', properties: {
      path: { type: 'string' }, protected: { type: 'boolean' }, hits: { type: 'integer' },
      classes: { type: 'array', items: { type: 'string' } },
      lines: { type: 'string', description: 'the line numbers or ranges of the hits that need a change, or "all NAME" etc.' },
      change: { type: 'string', enum: ['edit', 'none', 'optional'] },
      proposed: { type: 'string', description: 'the exact change, short (old -> new), or why none' },
      under_b: { type: 'string', description: 'what differs if the legacy tools moved into legacy/ (OQ-0002-2 (b)); empty if nothing' },
    }, required: ['path', 'protected', 'hits', 'classes', 'change', 'proposed'] } },
    indirect: { type: 'array', description: 'path dependencies a name grep cannot see (dynamic paths, folder arguments, globs, defaults)',
      items: { type: 'object', properties: { where: { type: 'string' }, what: { type: 'string' }, change: { type: 'string' } }, required: ['where', 'what', 'change'] } },
    findings: { type: 'array', items: { type: 'object', properties: { what: { type: 'string' }, where: { type: 'string' },
      matters_for: { type: 'string', description: '2.1, 2.2, PR #40, PR #46, Mustafa, or the advisor' } }, required: ['what', 'where', 'matters_for'] } },
    design: { type: 'string', description: 'area-specific design answer asked for in the prompt (markdown, at most ~60 lines)' },
  },
  required: ['files', 'indirect', 'findings', 'design'],
}

const AREAS = [
  { key: 'inv-code', title: 'code, CI, hooks and root files', prompt: `${COMMON}

YOUR AREA: ${NOTES}/hits-a-code.txt (101 hits), plus a full read of each of these for references a name grep misses:
every file in tools/ (gate.mjs, smoke.mjs, battery.js, preview.js, pageshot.js, sim.js, reach.js, perfbots.js,
tuning-dump.js, aabbs.js, village.py, README.md, package.json, gate-globals.json), .github/workflows/gate.yml,
.claude/hooks/*.mjs, .claude/scripts/*.mjs, .claude/settings.json, .gitignore, .gitattributes, README.md, FIRST_PROMPTS.md,
docs/ui_theme_mockup.html, and the five legacy files themselves (links between the pages, location/href, any local fetch
or file reference, the three.js script URL).
Give one files[] row per file that has hits or an indirect dependency (files with neither: list them in design as "checked, nothing").
In design, answer:
1. The gate change (tools/gate.mjs, smoke.mjs; battery.js only if needed): which constants and functions change, the head
   folder (legacy/), and the base-file fallback chain so a base ref from before the move (files at the root) and one from
   CS-1's era (shamble-frontlines-repo/) still load; what classicParts, check 5d and the smoke resolve against. Symbols
   added or removed (the gate's own top-level names), so the plan can state them.
2. Evidence that does not lean on the edited gate (advisor.md's tier checklists say CI's green does not stand when a gate
   file changes: read .claude/agents/advisor.md around :85-:100 and :150-:160). Propose exact commands for: (a) the five
   files byte-identical after the move (e.g. git diff -M100% name-status showing R100, or blob hashes); (b) main's
   untouched gate passing on those bytes; (c) the edited gate still failing the four Phase 0 mutations and check 5's 22
   mutations (docs/reports/2026-10-14-gate-check-5.md:30-41) with their controls, in throwaway worktrees under .gate/;
   (d) a dry-run move in a throwaway worktree as the build's first step.
3. The saves: KVStore uses localStorage on file:// (shamble-core.js:31-41). Design a headless Chromium test (Playwright
   persistent context, tools/node_modules) that tells whether a key written by a page at <dir>/index.html is read by a page
   at <dir>/legacy/index.html, and an export step for Mustafa's maps before his Brave check (does the editor have a map
   export? cite it). Mark the answer UNKNOWN until run.
4. PM-0002-36: the exact command pair (base '.' and head 'legacy', both maps, --pre=tools/tuning-dump.js) and how R7's
   builder (.checkpoints/r7-legacy-inventory/build-tuning.py at a715be2, read it with git show) can be run unchanged
   except its input and output paths, into .gate/foundation/, and cmp'd with docs/legacy-tuning.json and LEGACY_TUNING.md.
   Note any hard-coded path in the builder.

${SAVE('inv-code')}` },
  { key: 'inv-kit', title: 'kit instructions', prompt: `${COMMON}

YOUR AREA: ${NOTES}/hits-b-kit.txt (24 hits), plus a full read of CLAUDE.md, every file in .claude/agents/, .claude/rules/,
.claude/skills/*/SKILL.md, .claude/CHECKPOINTS.md, .claude/NIGHT_SHIFT.md, .claude/claude-security-guidance.md,
.claude/security-patterns.json, for references a name grep misses (commands with '.' as the build folder, globs, "the root",
"double-click", load-order text, the gate's file list).
Every one of these files is PROTECTED (each edit asks Mustafa), so for each say whether the edit is REQUIRED for
something to work or stay true after the move, or OPTIONAL. Aim for the fewest kit edits that leave nothing broken or false.
Give exact replacement text (old -> new) for each required edit, one line each.
In design, answer:
1. The minimal kit edit list for 2.1, with why each is required; and the list you leave alone, with why (NAME still true).
2. Rule text that 2.1 or 2.2 runs into and that only Mustafa can settle (quote each with file:line), for example CLAUDE.md's
   hard-constraint line "the game runs by double-clicking index.html (file://)", "Phase 0 writes no v2 code", "It moves to
   legacy/ in phase 1", the model rule, and STATUS's SPIKE-SCOPE (a) "once the drafts are approved" (docs/STATUS.md:745)
   against "signed off as the basis for the spikes, not yet accepted" (docs/STATUS.md:14-15). State each as a question with
   the options, never as a reading.
3. Overlap with PR #46 (files in facts.md; its list is not final): which of your required edits touch a file #46 names.
4. What advisor.md requires of a PR that changes gate files and of one that moves game files (tier, needs-mustafa, the
   Brave check), quoting its lines, so the plan states the PR's tier and merge route correctly.

${SAVE('inv-kit')}` },
  { key: 'inv-docs', title: 'docs', prompt: `${COMMON}

YOUR AREA: ${NOTES}/hits-c-docs.txt (603 hits in docs/), plus a read of each hit's context as needed.
Group rows by file. For big citation files (docs/LEGACY_INVENTORY.md 238 hits, docs/LEGACY_TUNING.md, docs/legacy-tuning.json,
docs/EDITOR_V1_INVENTORY.md, docs/research/*.md) one row per file with counts per class is enough, but list every hit that
is COMMAND or STALE individually by line.
Rules: docs/legacy-tuning.json and docs/LEGACY_TUNING.md must not change (facts.md). ADRs, the register (docs/adr/README.md),
design and look docs keep Mustafa's go (risk tier 1); reports and ROADMAP history never change.
In design, answer:
1. The docs edit list for 2.1 (which current-truth docs change and how: REPO_MAP, START_HERE, MODULE_MAP, TECHNICAL, README.md
   if it is in your list, STATUS), and the policy for citations (a one-line note, not a mass rewrite?), with reasons.
2. OQ-0002-2's docs part (LEGACY_INVENTORY row KT21: where the legacy docs go): read docs/adr/0002-code-structure-build-toolchain.md
   :998-1007 and KT21 (docs/LEGACY_INVENTORY.md:431). Options: legacy docs stay in docs/ (with a marker) or move to
   legacy/docs/. Count the path changes each option costs (links into and out of those docs, REPO_MAP rows, CLAUDE.md lines)
   and recommend one.
3. Read .claude/scripts/repo-map-check.mjs: what REPO_MAP must list after legacy/ appears (and after 2.2's crates/, packages/),
   so the check passes.
4. PR #40: its marks.json hashes PM rows of ADR 0001, 0004 and 0006 by their line text. For each hit in those three ADRs, say
   whether it sits inside a '| PM-' row; 2.1 must leave those rows byte-identical.
5. Which docs name the hard constraints or the double-click rule (they must say legacy/index.html after the move).

${SAVE('inv-docs')}` },
]

phase('Classify')
const results = await parallel(AREAS.map(a => () =>
  agent(a.prompt, { label: a.key, phase: 'Classify', schema: FILE_SCHEMA, model: 'opus', effort: 'high' })
    .then(r => r ? { key: a.key, ...r } : null)))
const got = results.filter(Boolean)
log(`classifiers returned ${got.length} of ${AREAS.length}: ${got.map(g => g.key).join(', ')}`)
const missing = AREAS.filter(a => !got.find(g => g.key === a.key)).map(a => a.key)
if (missing.length) log(`MISSING results (rerun on their own): ${missing.join(', ')}`)

phase('Critic')
const summary = got.map(g => `### ${g.key}\nfiles:\n${g.files.map(f => `- ${f.path} | ${f.protected ? 'PROTECTED' : '-'} | ${f.hits} | ${f.classes.join('/')} | ${f.change} | ${f.proposed}`).join('\n')}\nindirect:\n${g.indirect.map(i => `- ${i.where}: ${i.what} -> ${i.change}`).join('\n')}\nfindings:\n${g.findings.map(x => `- ${x.what} (${x.where}; ${x.matters_for})`).join('\n')}`).join('\n\n')

const critic = await agent(`${COMMON}

You are the COMPLETENESS CRITIC. Three area agents classified every legacy-path reference; their results are below (their
notes are in ${NOTES}/inv-code.md, inv-kit.md, inv-docs.md). Your job: find what they MISSED or got WRONG.
1. Sweep the whole tracked tree (git ls-files, excluding .checkpoints/ and spikes/) for references a name grep cannot see:
   relative paths ('./', '../'), names built at run time ('shamble-' + x, template strings, path.join(ROOT, ...)), readdirSync
   or glob over the root, '*.js' or '*.html' at the root, case or backslash variants, the gate's own lists (does check 1
   cover "every file" by a glob?), .gitattributes and .gitignore patterns, CI steps, settings rules, the night shift's and the
   PR Advisor's written commands (.claude/NIGHT_SHIFT.md, .claude/agents/advisor.md), hooks that act on a path (syntax-gate).
2. Spot-check at least 12 rows across the three results (at least 3 per agent), reading the cited lines; report each check as
   upheld or wrong, with the line.
3. Check the 'design' answers for a claim that does not hold against the code.
Return only what is missing or wrong, plus the spot-check results. If nothing is missing, say so plainly.

RESULTS:
${summary}

DESIGN ANSWERS:
${got.map(g => `### ${g.key}\n${g.design}`).join('\n\n')}

${SAVE('inv-critic')}`, {
  label: 'inv-critic', phase: 'Critic', model: 'opus', effort: 'high',
  schema: { type: 'object', properties: {
    missing: { type: 'array', items: { type: 'object', properties: { where: { type: 'string' }, what: { type: 'string' }, change: { type: 'string' } }, required: ['where', 'what', 'change'] } },
    wrong: { type: 'array', items: { type: 'object', properties: { row: { type: 'string' }, why: { type: 'string' }, correct: { type: 'string' } }, required: ['row', 'why', 'correct'] } },
    spot_checks: { type: 'array', items: { type: 'object', properties: { row: { type: 'string' }, verdict: { type: 'string', enum: ['upheld', 'wrong'] }, evidence: { type: 'string' } }, required: ['row', 'verdict', 'evidence'] } },
    design_issues: { type: 'array', items: { type: 'string' } },
  }, required: ['missing', 'wrong', 'spot_checks', 'design_issues'] },
})

return { results: got, missing, critic }
