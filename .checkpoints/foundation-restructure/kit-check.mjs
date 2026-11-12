// kit-check.mjs (foundation-restructure, step 2.1b, step 8): the kit commit holds exactly the four lines Mustafa said yes
// to (KIT-2.1B (a), 2026-11-11), compared by script, never by eye:
//   - the approved Before/After lines come from the advisor agent's notes (advisor-plan-check.md, "### Approved message
//     text": the text sent to him, each line indented by four spaces);
//   - the commit's removed and added lines (git show <kit>) must equal them, file by file, in order, and nothing else;
//   - each Before line must equal main 76b4f7a's line (CLAUDE.md :19, :43, :44; verifier.md :28), and each After line
//     must sit at the same line number in the kit commit.
// Run from the repo root: node .gate/foundation-restructure/kit-check.mjs <kit-commit>
import { readFileSync, writeFileSync } from 'node:fs';
import { execFileSync } from 'node:child_process';

const REPO = '/home/user/Shamble-Frontlines';
const KIT = process.argv[2];
if (!KIT) { console.error('usage: kit-check.mjs <kit-commit>'); process.exit(2); }
const git = (...a) => execFileSync('git', a, { cwd: REPO, encoding: 'utf8', maxBuffer: 64 << 20 });
const notes = readFileSync(`${REPO}/.checkpoints/foundation-restructure/advisor-plan-check.md`, 'utf8').split('\n');
const start = notes.indexOf('### Approved message text');
if (start < 0) { console.error('no "### Approved message text" in the notes'); process.exit(2); }
const pairs = [];
for (let i = start; i < notes.length && pairs.length < 4; i++) {
  const m = notes[i].match(/^(\d)\. (CLAUDE\.md|\.claude\/agents\/verifier\.md), line (\d+)\. Before:$/);
  if (!m) continue;
  if (notes[i + 2].trim() !== 'After:' || !notes[i + 1].startsWith('    ') || !notes[i + 3].startsWith('    ')) { console.error('unexpected layout at notes line ' + (i + 1)); process.exit(2); }
  pairs.push({ n: +m[1], file: m[2], line: +m[3], before: notes[i + 1].slice(4), after: notes[i + 3].slice(4) });
}
const res = { kit: git('rev-parse', KIT).trim(), notesLine: start + 1, pairs, checks: [] };
let ok = pairs.length === 4;
const check = (name, pass, detail = '') => { res.checks.push({ name, pass, detail }); if (!pass) ok = false; console.log(`${pass ? 'PASS' : 'FAIL'} ${name}${detail ? ' :: ' + detail : ''}`); };
check('four approved pairs read from the notes', pairs.length === 4, `${pairs.length} pairs`);
// the commit's files and its removed/added lines
const files = git('show', '--format=', '--name-only', KIT).trim().split('\n');
check('the kit commit touches exactly CLAUDE.md and .claude/agents/verifier.md', JSON.stringify(files.sort()) === JSON.stringify(['.claude/agents/verifier.md', 'CLAUDE.md']), files.join(', '));
for (const file of ['CLAUDE.md', '.claude/agents/verifier.md']) {
  const diff = git('show', '--format=', '-U0', KIT, '--', file).split('\n');
  const removed = diff.filter(l => l.startsWith('-') && !l.startsWith('---')).map(l => l.slice(1));
  const added = diff.filter(l => l.startsWith('+') && !l.startsWith('+++')).map(l => l.slice(1));
  const want = pairs.filter(p => p.file === file);
  check(`${file}: removed lines equal the approved Before lines`, JSON.stringify(removed) === JSON.stringify(want.map(p => p.before)), `${removed.length} removed`);
  check(`${file}: added lines equal the approved After lines`, JSON.stringify(added) === JSON.stringify(want.map(p => p.after)), `${added.length} added`);
}
for (const p of pairs) {
  const onMain = git('show', `76b4f7a:${p.file}`).split('\n')[p.line - 1];
  const inKit = git('show', `${KIT}:${p.file}`).split('\n')[p.line - 1];
  check(`kit line ${p.n}: Before equals main 76b4f7a's ${p.file}:${p.line}`, onMain === p.before);
  check(`kit line ${p.n}: After sits at ${p.file}:${p.line} in the kit commit`, inKit === p.after);
}
res.verdict = ok ? 'PASS' : 'FAIL';
console.log('kit lines: ' + res.verdict);
writeFileSync(`${REPO}/.checkpoints/foundation-restructure/kit-check-result.json`, JSON.stringify(res, null, 1) + '\n');
process.exit(ok ? 0 : 1);
