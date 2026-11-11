// e1-e2.mjs (foundation-restructure, step 2.1b): evidence E1 and E2 (plan revision 3, 1.5).
// E1, the bytes did not change: `git diff --find-renames=100% --name-status A <move>` lists exactly five R100 rows and
//   nothing else, and each moved blob equals A's and OLD's.
// E2, OLD's untouched gate passes on the moved bytes: in the throwaway worktree .gate/fr-e2 at <move>, OLD's five gate
//   files (git show 44446a5:tools/<x>) are written into legacy/tools/, node_modules linked; that gate takes its root as
//   the folder above its own (legacy/), so it reads the moved bytes as its head and OLD's root paths as its base:
//   `node .gate/fr-e2/legacy/tools/gate.mjs 44446a5` must PASS, symbols +0 -0, smoke on both pages. The scratch files
//   are removed one by one afterwards (no recursive delete), so the worktree is clean again.
// Run from the repo root: node .gate/foundation-restructure/e1-e2.mjs <move-commit>
import { writeFileSync, mkdirSync, symlinkSync, unlinkSync, rmdirSync, existsSync } from 'node:fs';
import { join } from 'node:path';
import { spawnSync, execFileSync } from 'node:child_process';

const REPO = '/home/user/Shamble-Frontlines';
const MOVE = process.argv[2];
if (!MOVE) { console.error('usage: e1-e2.mjs <move-commit>'); process.exit(2); }
const A = '76b4f7a', OLD = '44446a5';
const FIVE = ['shamble-core.js', 'shamble-game.js', 'shamble-editor.js', 'index.html', 'editor.html'];
const git = (cwd, ...a) => execFileSync('git', a, { cwd, encoding: 'utf8', maxBuffer: 64 << 20 });
const out = { move: git(REPO, 'rev-parse', MOVE).trim(), e1: {}, e2: {} };
let ok = true;
const check = (k, name, pass, detail = '') => { (out[k].checks ||= []).push({ name, pass, detail }); if (!pass) ok = false; console.log(`${k.toUpperCase()} ${pass ? 'PASS' : 'FAIL'} ${name}${detail ? ' :: ' + detail : ''}`); };

// ---------- E1 ----------
const ns = git(REPO, 'diff', '--find-renames=100%', '--name-status', A, MOVE).trim().split('\n');
out.e1.nameStatus = ns;
check('e1', 'exactly five rows', ns.length === 5, `${ns.length} rows`);
for (const f of FIVE) check('e1', `R100 ${f} -> legacy/${f}`, ns.includes(`R100\t${f}\tlegacy/${f}`));
const stat = git(REPO, 'show', '--format=', '--name-status', '--find-renames=100%', MOVE).trim().split('\n');
out.e1.commitOnly = stat;
check('e1', 'the move commit holds the five renames and nothing else', stat.length === 5 && stat.every(l => l.startsWith('R100\t')));
out.e1.blobs = {};
for (const f of FIVE) {
  const b = { A: git(REPO, 'rev-parse', `${A}:${f}`).trim(), OLD: git(REPO, 'rev-parse', `${OLD}:${f}`).trim(), move: git(REPO, 'rev-parse', `${MOVE}:legacy/${f}`).trim() };
  out.e1.blobs[f] = b;
  check('e1', `${f}: blob equal at A, OLD and legacy/ in the move`, b.A === b.OLD && b.A === b.move, b.move.slice(0, 7));
}

// ---------- E2 ----------
const WT = join(REPO, '.gate/fr-e2');
check('e2', 'worktree .gate/fr-e2 is at the move commit', git(WT, 'rev-parse', 'HEAD').trim() === out.move);
check('e2', 'worktree .gate/fr-e2 is clean before', git(WT, 'status', '--porcelain', '--untracked-files=all').trim() === '');
const T = join(WT, 'legacy/tools');
const GATE = ['gate.mjs', 'smoke.mjs', 'battery.js', 'gate-globals.json', 'package.json'];
mkdirSync(T);
for (const x of GATE) writeFileSync(join(T, x), git(REPO, 'show', `${OLD}:tools/${x}`));
symlinkSync(join(REPO, 'tools/node_modules'), join(T, 'node_modules'));
out.e2.oldGateBlobs = Object.fromEntries(GATE.map(x => [x, git(REPO, 'rev-parse', `${OLD}:tools/${x}`).trim()]));
const p = spawnSync(process.execPath, [join(T, 'gate.mjs'), OLD], { cwd: join(WT, 'legacy'), encoding: 'utf8', maxBuffer: 64 << 20 });
out.e2.exit = p.status; out.e2.stdout = p.stdout; out.e2.stderr = p.stderr;
console.log(p.stdout.trim()); if (p.stderr.trim()) console.log('stderr: ' + p.stderr.trim());
check('e2', 'OLD gate exit 0', p.status === 0, `exit ${p.status}`);
check('e2', 'GATE PASS', /^GATE PASS$/m.test(p.stdout));
check('e2', 'symbols +0 -0', /\| \+0 -0 \|/.test(p.stdout));
check('e2', 'smoke index 8/8 and editor 5/5', /index\.html 8\/8 steps, 0 errors/.test(p.stdout) && /editor\.html 5\/5 steps, 0 errors/.test(p.stdout));
// remove the scratch files one by one, then the empty folder
unlinkSync(join(T, 'node_modules'));
for (const x of GATE) unlinkSync(join(T, x));
rmdirSync(T);
check('e2', 'worktree .gate/fr-e2 is clean after', git(WT, 'status', '--porcelain', '--untracked-files=all').trim() === '' && !existsSync(T));

out.verdict = ok ? 'PASS' : 'FAIL';
console.log('E1 and E2: ' + out.verdict);
writeFileSync(join(REPO, '.checkpoints/foundation-restructure/e1-e2-result.json'), JSON.stringify(out, null, 1) + '\n');
process.exit(ok ? 0 : 1);
