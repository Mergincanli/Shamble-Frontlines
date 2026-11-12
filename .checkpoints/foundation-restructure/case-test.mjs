// case-test.mjs (foundation-restructure, step 2.1b, step 7): what the real findBuild (tools/smoke.mjs) does with letter
// case on this machine, for the tools/README.md paragraph's Linux claim ("a `Legacy/` folder or a
// `legacy/SHAMBLE-core.js` file is another name, neither counted nor read"). Scratch roots under
// .gate/foundation-restructure/case/ (findBuild checks names only, so the files are empty):
//   r1_capital_folder: the five files at the root, `Legacy/` holding the three engine names -> the root, no error
//   r2_capital_files:  the five files at the root, `legacy/SHAMBLE-*.js` (three files)      -> the root, no error
//   r3_control_two:    the five files at the root, `legacy/shamble-game.js`                 -> "two places" (exit 2 in the tools)
//   r4_control_legacy: nothing at the root, the five files in `legacy/`                     -> legacy/
// The move check's letter case is proven by 2.1a's rows-test-f1 row j_letter_case (the real gate: exit 2 on Index.html).
// Run from the repo root: node .gate/foundation-restructure/case-test.mjs
import { mkdirSync, writeFileSync, existsSync, readdirSync } from 'node:fs';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';

const REPO = '/home/user/Shamble-Frontlines';
const W = join(REPO, '.gate/foundation-restructure/case');
if (existsSync(W) && readdirSync(W).length) { console.error('use a fresh .gate/foundation-restructure/case'); process.exit(2); }
const { findBuild } = await import(pathToFileURL(join(REPO, 'tools/smoke.mjs')).href);
const FIVE = ['shamble-core.js', 'shamble-game.js', 'shamble-editor.js', 'index.html', 'editor.html'];
const JS = FIVE.slice(0, 3);
const rows = [
  { id: 'r1_capital_folder', root: FIVE, sub: 'Legacy', files: JS, want: b => !b.error && b.prefix === '' },
  { id: 'r2_capital_files', root: FIVE, sub: 'legacy', files: JS.map(f => f.replace('shamble', 'SHAMBLE')), want: b => !b.error && b.prefix === '' },
  { id: 'r3_control_two', root: FIVE, sub: 'legacy', files: ['shamble-game.js'], want: b => /two places/.test(b.error || '') },
  { id: 'r4_control_legacy', root: [], sub: 'legacy', files: FIVE, want: b => !b.error && b.prefix === 'legacy/' },
];
// is this file system case-sensitive? (Linux ext4/overlay: yes)
mkdirSync(join(W, 'probe'), { recursive: true });
writeFileSync(join(W, 'probe', 'ABC.txt'), '');
const res = { platform: process.platform, node: process.version, caseSensitive: !existsSync(join(W, 'probe', 'abc.txt')), rows: [] };
console.log(`platform ${res.platform}, node ${res.node}, case-sensitive file system: ${res.caseSensitive}`);
for (const r of rows) {
  const root = join(W, r.id);
  mkdirSync(join(root, r.sub), { recursive: true });
  for (const f of r.root) writeFileSync(join(root, f), '');
  for (const f of r.files) writeFileSync(join(root, r.sub, f), '');
  const b = findBuild(root);
  const out = { id: r.id, root: r.root, [r.sub + '/']: r.files, result: b.error ? { error: b.error } : { prefix: b.prefix || '(the root)' } };
  out.pass = r.want(b);
  res.rows.push(out);
  console.log(`${out.pass ? 'PASS' : 'FAIL'} ${r.id} :: ${b.error ? 'error: ' + b.error : 'build folder: ' + (b.prefix || 'the repo root')}`);
}
res.verdict = res.rows.every(r => r.pass) ? 'PASS' : 'FAIL';
console.log('letter case (findBuild): ' + res.verdict);
writeFileSync(join(REPO, '.checkpoints/foundation-restructure/case-test-result.json'), JSON.stringify(res, null, 1) + '\n');
process.exit(res.verdict === 'PASS' ? 0 : 1);
