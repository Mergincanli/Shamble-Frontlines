// e1-final.mjs (foundation-restructure, step 2.1b; the advisor agent's C3, second half): E1 on the final head.
// `git diff --find-renames=100% --name-status 76b4f7a <head> -- ':!.checkpoints'` must list exactly: the five legacy files
// as R100 renames into legacy/ (blobs equal to 76b4f7a's), the step 7 files (M), the two kit files (M), the docs of the
// docs pass (M) and the new 2.1b report (A); nothing else (no gate file, no CI, no settings or hooks, no spikes/).
// Run from the repo root: node .gate/foundation-restructure/e1-final.mjs <head>
import { writeFileSync } from 'node:fs';
import { execFileSync } from 'node:child_process';

const REPO = '/home/user/Shamble-Frontlines';
const HEAD = process.argv[2] || 'HEAD';
const git = (...a) => execFileSync('git', a, { cwd: REPO, encoding: 'utf8', maxBuffer: 64 << 20 });
const sha = git('rev-parse', HEAD).trim();
const rows = git('diff', '--find-renames=100%', '--name-status', '76b4f7a', sha, '--', '.', ':!.checkpoints').trim().split('\n').filter(Boolean);
const FIVE = ['shamble-core.js', 'shamble-game.js', 'shamble-editor.js', 'index.html', 'editor.html'];
const STEP7 = ['tools/README.md', 'tools/preview.js', 'tools/pageshot.js', 'tools/sim.js', 'tools/reach.js', 'tools/perfbots.js',
  'tools/tuning-dump.js', 'tools/aabbs.js', 'tools/village.py', 'README.md'];
const KIT = ['CLAUDE.md', '.claude/agents/verifier.md'];
const DOCS = ['docs/DECISIONS.md', 'docs/REPO_MAP.md', 'docs/START_HERE.md', 'docs/MODULE_MAP.md', 'docs/TECHNICAL.md', 'docs/STATUS.md',
  'docs/STATUS_ARCHIVE.md', 'docs/ROADMAP.md', 'docs/adr/0002-code-structure-build-toolchain.md', 'docs/adr/README.md'];
const res = { head: sha, rows, checks: [] };
let ok = true;
const check = (name, pass, detail = '') => { res.checks.push({ name, pass, detail }); if (!pass) ok = false; console.log(`${pass ? 'PASS' : 'FAIL'} ${name}${detail ? ' :: ' + detail : ''}`); };
for (const f of FIVE) {
  check(`R100 ${f} -> legacy/${f}`, rows.includes(`R100\t${f}\tlegacy/${f}`));
  check(`legacy/${f} blob equals 76b4f7a's ${f}`, git('rev-parse', `${sha}:legacy/${f}`).trim() === git('rev-parse', `76b4f7a:${f}`).trim());
}
const known = new Set([...FIVE.map(f => `R100\t${f}\tlegacy/${f}`), ...[...STEP7, ...KIT, ...DOCS].map(f => `M\t${f}`), 'A\tdocs/reports/2026-11-11-foundation-2-1b.md']);
const extra = rows.filter(r => !known.has(r));
check('nothing outside the plan: the five renames, step 7, the two kit files, the docs, the report', extra.length === 0, extra.join(' | ') || `${rows.length} rows`);
const missing = [...known].filter(k => !rows.includes(k) && !k.startsWith('M\tdocs/'));
check('every planned code, tool and kit file is in the diff', missing.length === 0, missing.join(' | '));
check('no .checkpoints/ file left on the head', git('ls-tree', '-r', '--name-only', sha, '.checkpoints').trim() === '');
res.verdict = ok ? 'PASS' : 'FAIL';
console.log(`E1 final head ${sha.slice(0, 7)}: ${res.verdict}`);
writeFileSync(`${REPO}/.gate/foundation-restructure/e1-final-result.json`, JSON.stringify(res, null, 1) + '\n');
process.exit(ok ? 0 : 1);
