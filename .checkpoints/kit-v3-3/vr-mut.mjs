// verifier: independent mutation rows on the head's guard; copies under .gate/kit-v3-3/vr-mut-<i>/. Prints failing names.
// Usage: node vr-mut.mjs <worktree>
import { spawnSync } from 'node:child_process';
import { copyFileSync, mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
const WT = process.argv[2];
const H = join(WT, '.claude/hooks');
const rg = readFileSync(join(H, 'routine-guard.mjs'), 'utf8');
const gd = readFileSync(join(H, 'guard.mjs'), 'utf8');
const m = (s, from, to) => (s.split(from).length === 2 ? s.replace(from, to) : null);
const chain = (s, ...pairs) => pairs.reduce((a, [f, t]) => (a === null ? null : m(a, f, t)), s);
const runs = [
  ['control (unmutated)', {}],
  ['day shell ask off', { rg: m(rg, 'first ??= ask(`kit and gate files ask first in a day session: ${rel} (${what})`);', '') }],
  ['dayEdit ask off', { rg: m(rg, '  return ask(`kit and gate files ask first in a day session: ${rel}`);\n}', '  return null;\n}') }],
  ['agent deny keyed on agent_type', { rg: chain(rg, ['const agent = !!input.agent_id;', 'const agent = !!input.agent_type;'], ['if (input.agent_id) return deny(`${KIT_AGENT}: ${rel}`);', 'if (input.agent_type) return deny(`${KIT_AGENT}: ${rel}`);']) }],
  ['Actions method check off', { rg: m(rg, 'if (ok.includes(ti.method)) return null;', 'return null;') }],
  ['Actions owner/repo check off', { rg: m(rg, "if (String(ti.owner ?? '').toLowerCase() !== owner || String(ti.repo ?? '').toLowerCase() !== repo) {", 'if (false) {') }],
  ['--output rev-list off', { rg: m(rg, "  'rev-list': ['--output'],\n", '') }],
  ['--output diff off', { rg: m(rg, "  diff: ['--output'],\n", '') }],
  ['--output log off', { rg: m(rg, "  log: ['--output'],\n", '') }],
  ['--output show off', { rg: m(rg, "  show: ['--output'],\n", '') }],
  ['kitPath projectRel first', { rg: m(rg, '  const g = gitPlace(abs, io);\n  if (!projectCommon', '  { const q = relIn(fold(realOf(io.projectDir, io), io), fold(abs, io)); if (q !== null) return protectedFold(q, io) ? q : null; }\n  const g = gitPlace(abs, io);\n  if (!projectCommon') }],
  ['stale main: name exemption', { rg: m(rg, "const h = sha ? protectedHit([ORIGIN_MAIN, sha, '--'], vcwd, io) : { failed: true };", "const h = pos[0] === 'main' ? null : sha ? protectedHit([ORIGIN_MAIN, sha, '--'], vcwd, io) : { failed: true };") }],
  ['guard.mjs fail-closed off', { gd: m(gd, "print(d.ask, false, event === 'PreToolUse' && !role)", "print(d.ask, false, event === 'PreToolUse')") }],
];
for (const [i, [label, f]] of runs.entries()) {
  if (Object.values(f).some((v) => v === null)) { console.log(`[${i}] ${label}: TARGET NOT FOUND`); continue; }
  const dir = join(WT, '.gate', 'kit-v3-3', `vr-mut-${i}`, '.claude', 'hooks');
  rmSync(join(WT, '.gate', 'kit-v3-3', `vr-mut-${i}`), { recursive: true, force: true }); mkdirSync(dir, { recursive: true });
  writeFileSync(join(dir, 'routine-guard.mjs'), f.rg ?? rg);
  writeFileSync(join(dir, 'guard.mjs'), f.gd ?? gd);
  copyFileSync(join(H, 'routine-guard.test.mjs'), join(dir, 'routine-guard.test.mjs'));
  const r = spawnSync(process.execPath, ['--test', join(dir, 'routine-guard.test.mjs')], { encoding: 'utf8' });
  const pass = /# pass (\d+)/.exec(r.stdout)?.[1], tot = /# tests (\d+)/.exec(r.stdout)?.[1];
  const fails = [...r.stdout.matchAll(/^not ok \d+ - (.*)$/gm)].map((x) => x[1].slice(0, 70));
  console.log(`[${i}] ${label}: ${pass}/${tot}; fails: ${fails.join(' | ') || '-'}`);
}
