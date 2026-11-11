// e6-commands.mjs (foundation-restructure, step 2.1b): evidence E6, every command 2.1b changes, run once as written
// (tools/README.md, the tool header comments, tools/village.py's doc line, START_HERE, verifier.md:28), from the repo
// root on the head's working tree. Linux needs the documented `xvfb-run -a` prefix for preview.js and pageshot --headed.
// Records each command's exit code, time and last output lines, and checks the files it should write.
// Run from the repo root: node .gate/foundation-restructure/e6-commands.mjs
import { writeFileSync, mkdirSync, existsSync, readFileSync, statSync } from 'node:fs';
import { join } from 'node:path';
import { spawnSync } from 'node:child_process';

const REPO = '/home/user/Shamble-Frontlines';
const E6 = '.gate/foundation-restructure/e6';
mkdirSync(join(REPO, E6), { recursive: true });
writeFileSync(join(REPO, E6, 'pre-teams4.js'), 'TUNE.teams.size = 4;\n');
const X = ['xvfb-run', '-a'];
const N = process.execPath;
const CMDS = [
  { id: 'battery', where: 'tools/README.md:22, START_HERE :117', argv: [N, 'tools/battery.js', 'legacy'], ok: r => r.exit === 0 && /"core"/.test(r.out) },
  { id: 'preview-menu', where: 'tools/README.md:23, preview.js:2', argv: [...X, N, 'tools/preview.js', 'legacy', 'menu', `${E6}/menu.png`], files: [`${E6}/menu.png`] },
  { id: 'preview-shot', where: 'tools/README.md:23 (shot <out.png> <x> <z>)', argv: [...X, N, 'tools/preview.js', 'legacy', 'shot', `${E6}/shot.png`, '0', '0'], files: [`${E6}/shot.png`] },
  { id: 'sim', where: 'tools/README.md:24, sim.js:1, verifier.md:28, START_HERE :117', argv: [...X, N, 'tools/preview.js', 'legacy', 'eval', 'tools/sim.js'] },
  { id: 'reach', where: 'tools/README.md:25, reach.js:1', argv: [...X, N, 'tools/preview.js', 'legacy', 'eval', 'tools/reach.js'] },
  { id: 'perfbots', where: 'tools/README.md:26, perfbots.js:2', argv: [...X, N, '--expose-gc', 'tools/preview.js', 'legacy', 'eval', 'tools/perfbots.js', `--pre=${E6}/pre-teams4.js`] },
  { id: 'aabbs', where: 'tools/README.md:27, aabbs.js:1, village.py:2', argv: [...X, N, 'tools/preview.js', 'legacy', 'eval', 'tools/aabbs.js'], stdoutTo: '.gate/aabbs.json', ok: r => r.exit === 0 && (() => { try { JSON.parse(readFileSync(join(REPO, '.gate/aabbs.json'), 'utf8')); return true; } catch { return false; } })() },
  { id: 'village', where: 'tools/README.md:27 (python tools/village.py, no arguments)', argv: ['python', 'tools/village.py'], files: ['.gate/village.json'] },
  { id: 'pageshot', where: 'tools/README.md:28', argv: [N, 'tools/pageshot.js', 'legacy', 'index.html', `${E6}/s`, 'wait:1500', 'shot:menu'], files: [`${E6}/s_menu.png`] },
  { id: 'pageshot-tri', where: 'tools/README.md:28 (index.html?map=tri)', argv: [N, 'tools/pageshot.js', 'legacy', 'index.html?map=tri', `${E6}/tri`, 'wait:1500', 'shot:menu'], files: [`${E6}/tri_menu.png`] },
  { id: 'pageshot-header', where: 'pageshot.js:8', argv: [N, 'tools/pageshot.js', 'legacy', 'index.html', '.gate/s', 'wait:1500', 'shot:menu', 'eval:deploy()', 'wait:900', 'eval:(closeTutorial(),setPause(true))', 'shot:pause'], files: ['.gate/s_menu.png', '.gate/s_pause.png'] },
  { id: 'pageshot-headed', where: 'tools/README.md:56 (Check a new machine)', argv: [...X, N, 'tools/pageshot.js', 'legacy', 'index.html', '.gate/gpu', '--headed', 'size:1600x900', 'wait:2500', 'shot:menu', 'eval:(deploy(),0)', 'wait:1500', 'eval:(closeTutorial(),setPause(true),0)', 'wait:600', 'shot:pause'], files: ['.gate/gpu_menu.png', '.gate/gpu_pause.png'] },
  { id: 'preview-menu-readme58', where: 'tools/README.md:58', argv: [...X, N, 'tools/preview.js', 'legacy', 'menu', '.gate/preview_menu.png'], files: ['.gate/preview_menu.png'] },
  { id: 'tuning-dump', where: 'tuning-dump.js:3', argv: [...X, N, 'tools/preview.js', 'legacy', 'eval', 'tools/tuning-dump.js', '--pre=tools/tuning-dump.js'], stdoutTo: `${E6}/tuning-cabana.json` },
];
const only = process.argv.slice(2);
const results = [];
for (const c of CMDS) {
  if (only.length && !only.includes(c.id)) continue;
  for (const f of c.files || []) if (existsSync(join(REPO, f))) writeFileSync(join(REPO, f), ''); // a stale file must not pass
  const t0 = process.hrtime.bigint();
  const p = spawnSync(c.argv[0], c.argv.slice(1), { cwd: REPO, encoding: 'utf8', maxBuffer: 256 << 20 });
  const secs = Number(process.hrtime.bigint() - t0) / 1e9;
  if (c.stdoutTo) writeFileSync(join(REPO, c.stdoutTo), p.stdout || '');
  const r = { id: c.id, where: c.where, command: c.argv.map(a => a === N ? 'node' : a).join(' ') + (c.stdoutTo ? ` > ${c.stdoutTo}` : ''), exit: p.status, secs: +secs.toFixed(1),
    out: (p.stdout || '').trim().split('\n').slice(-3).join(' | ').slice(0, 400), err: (p.stderr || '').trim().split('\n').slice(-2).join(' | ').slice(0, 300) };
  const filesOk = (c.files || []).every(f => existsSync(join(REPO, f)) && statSync(join(REPO, f)).size > 0);
  r.pass = (c.ok ? c.ok({ ...r, out: p.stdout || '' }) : r.exit === 0) && filesOk;
  if (c.files) r.files = c.files.map(f => `${f} ${existsSync(join(REPO, f)) ? statSync(join(REPO, f)).size : 'missing'} bytes`);
  results.push(r);
  console.log(`${r.pass ? 'PASS' : 'FAIL'} ${c.id} exit ${r.exit} ${r.secs}s :: ${r.out.slice(0, 160)}`);
}
const file = join(REPO, '.checkpoints/foundation-restructure/e6-commands-result.json');
const prev = existsSync(file) ? JSON.parse(readFileSync(file, 'utf8')) : { results: [] };
const merged = [...prev.results.filter(r => !results.some(x => x.id === r.id)), ...results];
writeFileSync(file, JSON.stringify({ results: merged, verdict: merged.every(r => r.pass) ? 'PASS' : 'FAIL' }, null, 1) + '\n');
console.log(`E6 commands: ${merged.filter(r => r.pass).length} of ${merged.length} PASS`);
