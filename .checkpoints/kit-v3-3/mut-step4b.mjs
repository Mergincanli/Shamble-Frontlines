// kit-v3-3 step 4b mutation runs: each mutant in a scratch copy under .gate/kit-v3-3/, never the real files; prints
// the failing test names. Usage (from the worktree root): node .checkpoints/kit-v3-3/mut-step4b.mjs
import { spawnSync } from 'node:child_process';
import { copyFileSync, mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
const H = '.claude/hooks';
const rg = readFileSync(join(H, 'routine-guard.mjs'), 'utf8');
const st = readFileSync('.claude/settings.json', 'utf8');
const base = (f) => spawnSync('git', ['show', `HEAD:${f}`], { encoding: 'utf8' }).stdout;
const m = (from, to, s = rg) => (s.split(from).length !== 2 ? null : s.replace(from, to));
const m2 = (pairs) => pairs.reduce((s, [a, b]) => (s === null ? null : m(a, b, s)), rg);
const runs = [
  ['unmutated copy', {}],
  ['base guard (HEAD = f22c4df, before step 4b) + new tests', { rg: base(`${H}/routine-guard.mjs`), st: base('.claude/settings.json') }],
  ['R1 day ask off (file tools)', { rg: m('return ask(`kit and gate files ask first in a day session: ${rel}`);', 'return null;') }],
  ['R1 dayEdit not wired', { rg: m('if (!role) return dayEdit(tool, ti, cwd, io, input);', 'if (!role) return null;') }],
  ['R1+shell day ask off (both asks)', { rg: m2([['return ask(`kit and gate files ask first in a day session: ${rel}`);', 'return null;'], ['first ??= ask(`kit and gate files ask first in a day session: ${rel} (${what})`);', '']]) }],
  ['R2 plain merge unguarded', { rg: m('} else if (args.length === 1) {', '} else if (false) {') }],
  ['R3 rev-list allow removed', { st: m('      "Bash(git rev-list *)",\n', '', st) }],
  ['R4 agent ask on an unresolvable target', { rg: m('if (agent) return deny(`${KIT_AGENT}; an agent spells', 'if (false) return deny(`${KIT_AGENT}; an agent spells') }],
  ['S-1 target word instead of SHA', { rg: m("const h = sha ? protectedHit([ORIGIN_MAIN, sha, '--'], vcwd, io) : { failed: true };", "const h = protectedHit([ORIGIN_MAIN, pos[0], '--'], vcwd, io);") }],
  ['S-1 commitOf passes a -word to git', { rg: m("if (!ref || ref.startsWith('-')) return null;", 'if (!ref) return null;') }],
  ['S-1 commitOf: no - check and no --end-of-options', { rg: m2([["if (!ref || ref.startsWith('-')) return null;", 'if (!ref) return null;'], ["['rev-parse', '--verify', '--end-of-options', `${ref}^{commit}`]", "['rev-parse', '--verify', `${ref}^{commit}`]"]]) }],
  ['S-2 merge by name', { rg: m('if (t === null || !isOriginMain(t, vcwd, io)) {', "if (t !== 'origin/main') {") }],
  ['S-2 switch -c start by name', { rg: m('if (pos[0] && !isOriginMain(pos[0], vcwd, io)) {', "if (pos[0] && pos[0] !== 'origin/main') {") }],
  ['S-2 origin/main resolved by short name', { rg: m('const o = commitOf(ORIGIN_MAIN, vcwd, io);', "const o = commitOf('origin/main', vcwd, io);") }],
  ['S-3 darwin does not fold', { rg: m("['win32', 'darwin'].includes(io.platform ?? process.platform)", "['win32'].includes(io.platform ?? process.platform)") }],
  ['S-4 pushd/popd not followed', { rg: m("if (!role && (prog === 'cd' || prog === 'pushd' || prog === 'popd')) {", "if (!role && prog === 'cd') {") }],
  ['S-4 forked line not lost', { rg: m('if (forked) return lost();', '') }],
  ['S-4 untracked target not lost', { rg: m('if (a.length > 1 || /^[-+]/.test(t) || /[$`*?[{]/.test(t) || /^~[^/]/.test(t)) return lost();', '') }],
];
for (const [i, [label, files]] of runs.entries()) {
  if (Object.values(files).some((v) => v === null || v === undefined || v === '')) { console.log(`${label}: TARGET NOT FOUND`); continue; }
  const dir = join('.gate', 'kit-v3-3', `mut-step4b-${i}`);
  rmSync(dir, { recursive: true, force: true });
  const hd = join(dir, '.claude', 'hooks');
  mkdirSync(hd, { recursive: true });
  writeFileSync(join(hd, 'routine-guard.mjs'), files.rg ?? rg);
  copyFileSync(join(H, 'guard.mjs'), join(hd, 'guard.mjs'));
  for (const f of ['routine-guard.test.mjs', 'settings.test.mjs']) copyFileSync(join(H, f), join(hd, f));
  writeFileSync(join(dir, '.claude', 'settings.json'), files.st ?? st);
  const out = [];
  for (const t of ['routine-guard.test.mjs', 'settings.test.mjs']) {
    const r = spawnSync(process.execPath, ['--test', join(hd, t)], { encoding: 'utf8', maxBuffer: 64 * 1024 * 1024 });
    const fails = r.stdout.split('\n').filter((l) => /^not ok/.test(l)).map((l) => l.replace(/^not ok \d+ - /, ''));
    const count = (k) => (r.stdout.match(new RegExp(`^# ${k} (\\d+)`, 'm')) || [])[1];
    out.push(`${t} ${count('pass')}/${count('tests')}${fails.length ? ' failing: ' + fails.join(' | ') : ''}`);
  }
  console.log(`[${i}] ${label}: ${out.join(' ;; ')}`);
}
