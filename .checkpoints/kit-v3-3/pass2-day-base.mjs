// fix pass 2, day part: rebuild the pre-day guard (the routine builder's rules) by switching each day change off in a
// flat copy, then run the new day rows of the test file against it, to show which rows move (null -> ask/deny).
// usage: node pass2-day-base.mjs <worktree> ; writes <worktree>/.gate/kit-v3-3/pass2/base/{routine-guard.mjs,routine-guard.test.mjs}
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { spawnSync } from 'node:child_process';
const wt = process.argv[2];
const out = join(wt, '.gate', 'kit-v3-3', 'pass2', 'base');
mkdirSync(out, { recursive: true });
let src = readFileSync(join(wt, '.claude', 'hooks', 'routine-guard.mjs'), 'utf8');
const off = [
  ["else if (line.gitBefore && pos[0] !== 'origin/main') {", 'else if (false) {'],
  ["if (t !== null && line.gitBefore && t !== 'origin/main') {", 'if (false) {'],
  ["if (c.sep === '&&' && !dirs.known) {", 'if (false) {'],
  ["const CD_REACH = new Set(['if', '{', 'time',", "const CD_REACH = new Set(['if', '{', '!', 'time',"],
  ["if (['function', 'trap', 'alias'].includes(prog)) return unknown();", ''],
  ["'--ref-format', '--revision']);", "'--ref-format']);"],
  ['return [...pos.slice(1), ...made, ...gitDir];',
    "const dir = pos.length > 1 ? pos[1] : pos.length ? cloneDir(pos[0]) : '';\n    return [...(dir ? [dir] : []), ...gitDir];"],
  ['if (wt !== undefined) return', 'if (false) return'],
  ["if (wts.code !== 0) return `${p} (the guard could not list this repository's worktrees)`;", ''],
];
for (const [from, to] of off) {
  if (src.split(from).length !== 2) throw new Error(`not once: ${from}`);
  src = src.replace(from, () => to);
}
writeFileSync(join(out, 'routine-guard.mjs'), src);
writeFileSync(join(out, 'routine-guard.test.mjs'), readFileSync(join(wt, '.claude', 'hooks', 'routine-guard.test.mjs'), 'utf8'));
const pat = String.raw`^day \(kit-v3-3\) .*(SD-[3-6] |V-1 |R-b a \./ cd after an unknown one, joined|SA-6 rm -rf / with no worktree)`;
for (const [label, dir] of [['base', out], ['head', join(wt, '.claude', 'hooks')]]) {
  const r = spawnSync('node', ['--test', `--test-name-pattern=${pat}`, join(dir, 'routine-guard.test.mjs')], { encoding: 'utf8' });
  const lines = r.stdout.split('\n');
  const ok = lines.filter((l) => /^ok \d+ - day/.test(l)).length;
  const bad = lines.filter((l) => /^not ok \d+ - day/.test(l)).map((l) => l.replace(/^not ok \d+ - /, ''));
  console.log(`${label}: ${ok} pass, ${bad.length} fail`);
  for (const b of bad) console.log(`  moves: ${b}`);
}
