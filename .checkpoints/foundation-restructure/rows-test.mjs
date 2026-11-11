// rows-test.mjs (foundation-restructure, 2.1a): the build-folder rule as a table, on scratch copies of the edited tools.
// Each row is a plain folder .gate/fdn-rows/<row> (inside the repo, so git finds the repo for the base) holding a copy of
// .gate/fdn-scratch/tools (the edited gate) and the five files laid out as the row says. Run from the repo root:
//   node .checkpoints/foundation-restructure/rows-test.mjs
// Prints one line per row: exit code, the gate's build/prefix (from --json) or its first error line, and PASS/FAIL of the
// row against its expectation.
import { cpSync, mkdirSync, symlinkSync, existsSync, copyFileSync } from 'node:fs';
import { join } from 'node:path';
import { spawnSync } from 'node:child_process';

const REPO = '/home/user/Shamble-Frontlines';
const S = join(REPO, '.gate/fdn-scratch'), OUT = join(REPO, '.gate/fdn-rows');
const FIVE = ['shamble-core.js', 'shamble-game.js', 'shamble-editor.js', 'index.html', 'editor.html'];
const env = { ...process.env, PLAYWRIGHT_BROWSERS_PATH: join(REPO, '.gate/pw-browsers') };
const rows = [
  { id: 'a_root_only', root: FIVE, legacy: [], smoke: true, want: { exit: 0, build: '.' } },
  { id: 'b_legacy_only', root: [], legacy: FIVE, smoke: true, want: { exit: 0, build: 'legacy/' } },
  { id: 'c_both_complete', root: FIVE, legacy: FIVE, want: { exit: 2, msg: /two places/ } },
  { id: 'd_neither', root: [], legacy: [], want: { exit: 2, msg: /^gate: shamble-core\.js is missing from the working tree/ } },
  { id: 'e_partial_legacy', root: [], legacy: ['shamble-core.js', 'shamble-game.js', 'index.html', 'editor.html'],
    want: { exit: 2, msg: /^gate: legacy\/shamble-editor\.js is missing from the working tree/ } },
  { id: 'f_root_plus_stray_legacy_js', root: FIVE, legacy: ['shamble-game.js'], want: { exit: 2, msg: /two places/ } },
  { id: 'g_partial_root', root: ['shamble-core.js', 'shamble-game.js', 'index.html', 'editor.html'], legacy: [],
    want: { exit: 2, msg: /^gate: shamble-editor\.js is missing from the working tree/ } },
  { id: 'h_legacy_plus_stray_root_page', root: ['index.html'], legacy: FIVE, smoke: false, want: { exit: 0, build: 'legacy/' } },
];
let bad = 0;
for (const r of rows) {
  const dir = join(OUT, r.id);
  if (existsSync(dir)) { console.log(`${r.id}: folder exists, skipped (use a fresh OUT)`); bad++; continue; }
  mkdirSync(join(dir, 'legacy'), { recursive: true });
  cpSync(join(S, 'tools'), join(dir, 'tools'), { recursive: true, filter: src => !src.endsWith('/node_modules') });
  symlinkSync(join(REPO, 'tools/node_modules'), join(dir, 'tools/node_modules'));
  for (const f of r.root) copyFileSync(join(S, f), join(dir, f));
  for (const f of r.legacy) copyFileSync(join(S, f), join(dir, 'legacy', f));
  const args = ['tools/gate.mjs', '44446a5', '--json', ...(r.smoke ? [] : ['--no-smoke'])];
  const p = spawnSync(process.execPath, args, { cwd: dir, env, encoding: 'utf8', maxBuffer: 64 << 20 });
  let build = null, prefix = null, pass = null;
  try { const j = JSON.parse(p.stdout); build = j.build; prefix = j.base && j.base.prefix; pass = j.pass; } catch { /* died before the result */ }
  const err = (p.stderr || '').split('\n')[0];
  let ok = p.status === r.want.exit;
  if (r.want.build !== undefined) ok = ok && build === r.want.build;
  if (r.want.msg) ok = ok && r.want.msg.test(err);
  if (!ok) bad++;
  console.log(`${ok ? 'PASS' : 'FAIL'} ${r.id}: exit ${p.status}; build ${build}; base prefix ${JSON.stringify(prefix)}; gate pass ${pass}; stderr: ${err || '(none)'}`);
}
// the smoke CLI follows the same rule
for (const [id, want] of [['b_legacy_only', 0], ['c_both_complete', 2]]) {
  const p = spawnSync(process.execPath, ['tools/smoke.mjs', '--json'], { cwd: join(OUT, id), env, encoding: 'utf8', maxBuffer: 64 << 20 });
  let summary = ''; try { summary = JSON.parse(p.stdout).summary; } catch { summary = (p.stderr || '').split('\n')[0]; }
  const ok = p.status === want; if (!ok) bad++;
  console.log(`${ok ? 'PASS' : 'FAIL'} smoke CLI ${id}: exit ${p.status}; ${summary.slice(0, 160)}`);
}
console.log(bad ? `ROWS: ${bad} FAILED` : 'ROWS: ALL PASS');
process.exit(bad ? 1 : 0);
