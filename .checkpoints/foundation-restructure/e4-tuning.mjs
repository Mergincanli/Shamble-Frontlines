// e4-tuning.mjs (foundation-restructure, step 2.1b): evidence E4, the legacy tuning oracle after the move
// (PM-0002-36, read as the advisor agent's Q-A1). The dump (tools/tuning-dump.js as --pre, both maps) at A with the
// folder '.' and at the head with the folder 'legacy', twice each, A's copies of preview.js and tuning-dump.js for all
// runs (2.1b edits only their header comments): every head run must be byte-identical to A's. Then R7's builder
// (build-tuning.py at a715be2), run unchanged except its input and output paths, on the head dumps: its two files must
// be cmp-equal to docs/legacy-tuning.json and docs/LEGACY_TUNING.md.
//   A    = .gate/fr-a at 76b4f7a (files at the root; tools/node_modules linked)
//   head = the main folder's legacy/ (checked equal to the move commit's blobs first)
// Run from the repo root: node .gate/foundation-restructure/e4-tuning.mjs <move-commit>
import { readFileSync, writeFileSync, mkdirSync, existsSync } from 'node:fs';
import { join } from 'node:path';
import { spawnSync, execFileSync } from 'node:child_process';

const REPO = '/home/user/Shamble-Frontlines';
const MOVE = process.argv[2];
if (!MOVE) { console.error('usage: e4-tuning.mjs <move-commit>'); process.exit(2); }
const AWT = join(REPO, '.gate/fr-a');
const OUT = join(REPO, '.gate/foundation-restructure/e4');
mkdirSync(join(OUT, 'out', 'docs'), { recursive: true });
const git = (...a) => execFileSync('git', a, { cwd: REPO, encoding: 'utf8', maxBuffer: 64 << 20 });
const FIVE = ['shamble-core.js', 'shamble-game.js', 'shamble-editor.js', 'index.html', 'editor.html'];
const report = { move: MOVE, runs: [], checks: [] };
const check = (name, pass, detail = '') => { report.checks.push({ name, pass, detail }); console.log(`${pass ? 'PASS' : 'FAIL'} ${name}${detail ? ' :: ' + detail : ''}`); };

// the inputs are what they claim to be
check('A worktree is 76b4f7a', execFileSync('git', ['rev-parse', 'HEAD'], { cwd: AWT, encoding: 'utf8' }).trim().startsWith('76b4f7a'));
for (const f of FIVE) {
  const disk = execFileSync('git', ['hash-object', join(REPO, 'legacy', f)], { encoding: 'utf8' }).trim();
  check(`head legacy/${f} equals the move commit's blob`, disk === git('rev-parse', `${MOVE}:legacy/${f}`).trim());
}

function dump(folder, map, n) {
  const args = ['-a', process.execPath, 'tools/preview.js', folder, 'eval', 'tools/tuning-dump.js', '--pre=tools/tuning-dump.js'];
  if (map) args.push('--map=' + map);
  const p = spawnSync('xvfb-run', args, { cwd: AWT, encoding: 'utf8', maxBuffer: 256 << 20 });
  const name = `${folder === '.' ? 'a' : 'head'}-${map || 'cabana'}-${n}.json`;
  writeFileSync(join(OUT, name), p.stdout);
  report.runs.push({ name, folder, map: map || 'default', exit: p.status, bytes: p.stdout.length, stderr: (p.stderr || '').split('\n').filter(Boolean).slice(-2) });
  console.log(`${name}: exit ${p.status}, ${p.stdout.length} bytes`);
  return p.stdout;
}
const res = {};
for (const map of ['', 'tri']) for (const n of [1, 2]) {
  res[`a-${map || 'cabana'}-${n}`] = dump('.', map, n);
  res[`head-${map || 'cabana'}-${n}`] = dump(join(REPO, 'legacy'), map, n);
}
for (const map of ['cabana', 'tri']) {
  const a1 = res[`a-${map}-1`];
  check(`${map}: A's two runs identical`, a1 === res[`a-${map}-2`]);
  check(`${map}: head run 1 byte-identical to A's`, res[`head-${map}-1`] === a1);
  check(`${map}: head run 2 byte-identical to A's`, res[`head-${map}-2`] === a1);
  check(`${map}: the dump is not empty JSON`, (() => { try { return Object.keys(JSON.parse(a1).defaults || {}).length > 0; } catch { return false; } })());
}

// R7's builder at a715be2, unchanged except its input and output paths
const orig = git('show', 'a715be2:.checkpoints/r7-legacy-inventory/build-tuning.py');
const rel = p => p.replace(REPO + '/', '');
const pairs = [
  ["'.checkpoints/r7-legacy-inventory/tuning/cabana-1.json'", `'${rel(join(OUT, 'head-cabana-1.json'))}'`],
  ["'.checkpoints/r7-legacy-inventory/tuning/tri-1.json'", `'${rel(join(OUT, 'head-tri-1.json'))}'`],
  ["'docs/legacy-tuning.json'", `'${rel(join(OUT, 'out/docs/legacy-tuning.json'))}'`],
  ["'docs/LEGACY_TUNING.md'", `'${rel(join(OUT, 'out/docs/LEGACY_TUNING.md'))}'`],
];
let copy = orig;
for (const [from, to] of pairs) { const n = copy.split(from).length - 1; check(`builder path ${from} found`, n >= 1, `${n} times`); copy = copy.split(from).join(to); }
writeFileSync(join(OUT, 'build-tuning.py'), copy);
const b = spawnSync('python3', ['-I', join(OUT, 'build-tuning.py')], { cwd: REPO, encoding: 'utf8' });
check('the builder ran', b.status === 0, (b.stderr || '').split('\n').filter(Boolean).slice(-1).join(''));
for (const f of ['legacy-tuning.json', 'LEGACY_TUNING.md']) {
  const built = existsSync(join(OUT, 'out/docs', f)) ? readFileSync(join(OUT, 'out/docs', f)) : Buffer.alloc(0);
  check(`docs/${f} reproduced byte for byte`, built.equals(readFileSync(join(REPO, 'docs', f))), `${built.length} bytes`);
}
report.verdict = report.checks.every(c => c.pass) ? 'PASS' : 'FAIL';
console.log('E4: ' + report.verdict);
writeFileSync(join(REPO, '.checkpoints/foundation-restructure/e4-result.json'), JSON.stringify(report, null, 1) + '\n');
