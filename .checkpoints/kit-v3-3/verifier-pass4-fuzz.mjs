// Verifier pass 4: an independent "token soup" differential for R-PLAIN's claim. Random strings over the plain
// alphabet and operators, glued with or without blanks; keep those plainForm accepts and bash -n parses; run them in
// real bash with stub programs (argv logged NUL-separated) and compare with parseShell's commands and redirect targets.
// usage: node verifier-pass4-fuzz.mjs <a3 root> <scratch dir> <count> <seed>
import { spawnSync } from 'node:child_process';
import { mkdirSync, rmSync, writeFileSync, readFileSync, readdirSync, realpathSync } from 'node:fs';
import { join, dirname, relative } from 'node:path';
import { pathToFileURL } from 'node:url';
const [a3, scratch, countS = '4000', seedS = '7'] = process.argv.slice(2);
const { parseShell, plainForm } = await import(pathToFileURL(join(a3, '.claude/hooks/routine-guard.mjs')).href);
let seed = Number(seedS);
const r = () => { seed = (seed + 0x6D2B79F5) | 0; let x = Math.imul(seed ^ (seed >>> 15), 1 | seed); x = (x + Math.imul(x ^ (x >>> 7), 61 | x)) ^ x; return ((x ^ (x >>> 14)) >>> 0) / 4294967296; };
const pick = (a) => a[Math.floor(r() * a.length)];
const PROGS = ['git', 'gh', 'aa', 'bb'];
const WORDS = ['git', 'gh', 'aa', 'bb', 'x=1', 'Y=a:b', '-q', '--a=b', '2', '1', '10', 'a~b', 'a%b', 'a^b', 'u@v', 'a+b', 'a,b', 'a:b', 'a.b', 'd/e', 'k=v=w', '-', '--', 'aa2', 'f1', 'f2',
  "'q r'", "'#x'", "'a\nb'", '"s\'t"', '"<x>"', '"a b"', "''", '""', "'2'", "x='y z'", 'x="1 2"', "a'b'c", 'a"b"c'];
const OPS = [' ', ' ', ' ', '\t', '  ', '>', '>>', '>|', '<', '&>', '&>>', '>&1', '>&2', '2>&1', '1>&2', '>&-', '2>', '2>>', '&', '&&', '|', '|&', ';', '\n', ' > ', ' < ', ' | ', ' ; ', ' && '];
const lines = [];
const tries = Number(countS) * 6;
for (let n = 0; n < tries && lines.length < Number(countS); n++) {
  let s = pick(PROGS);
  for (let k = 3 + Math.floor(r() * 12); k > 0; k--) s += r() < 0.5 ? pick(OPS) : (r() < 0.5 ? ' ' : '') + pick(WORDS);
  if (s.includes('||')) continue;
  if (plainForm(s) !== null) continue;
  const p = parseShell(s);
  const NAME = /^[A-Za-z_][A-Za-z0-9_]*=/;
  const progOk = p.cmds.every((c) => { let k = 0; while (k < c.argv.length && NAME.test(c.argv[k])) k++; return k === c.argv.length || PROGS.includes(c.argv[k]); });
  if (!progOk) continue;
  if ([...p.cmds.flatMap((c) => [...c.redirects, ...c.reads])].some((f) => f.startsWith('/') || f.split('/').includes('..') || f === '' || f === '-')) continue;
  if (spawnSync('bash', ['--norc', '--noprofile', '-n', '-c', s]).status !== 0) continue;
  lines.push(s);
}
const root = realpathSync((rmSync(scratch, { recursive: true, force: true }), mkdirSync(scratch, { recursive: true }), scratch));
const bin = join(root, 'bin'), log = join(root, 'log');
mkdirSync(bin);
const stub = `#!/bin/sh\nprintf '%s\\0' "\${0##*/}" "$@" '\x01' >> '${log}'\n`;
for (const n of [...PROGS, '__mark']) { writeFileSync(join(bin, n), stub); spawnSync('chmod', ['755', join(bin, n)]); }
const walk = (d, base = d) => readdirSync(d, { withFileTypes: true }).flatMap((e) => (e.isDirectory() ? walk(join(d, e.name), base) : [relative(base, join(d, e.name))]));
const dirOf = (k) => join(root, 'w', `L${k}`);
const before = [];
lines.forEach((line, k) => {
  mkdirSync(dirOf(k), { recursive: true });
  for (const c of parseShell(line).cmds) {
    for (const f of [...c.redirects, ...c.reads]) mkdirSync(join(dirOf(k), dirname(f)), { recursive: true });
    for (const f of c.reads) writeFileSync(join(dirOf(k), f), '');
  }
  before.push(new Set(walk(dirOf(k))));
});
const BASH = spawnSync('bash', ['-c', 'echo $BASH'], { encoding: 'utf8' }).stdout.trim();
for (let s = 0; s < lines.length; s += 300) {
  const script = lines.slice(s, s + 300).map((line, j) => `cd -- '${dirOf(s + j)}' || exit 97\n__mark ${s + j}\n${line}\nwait\n`).join('');
  writeFileSync(join(root, `b${s}.sh`), script);
  spawnSync(BASH, ['--norc', '--noprofile', join(root, `b${s}.sh`)], { cwd: root, env: { PATH: bin, HOME: root, LC_ALL: 'C' }, encoding: 'utf8', maxBuffer: 1 << 26 });
}
const ran = new Map(); let cur = null;
for (const rec of readFileSync(log, 'utf8').split('\x01\0').filter(Boolean)) {
  const argv = rec.split('\0').slice(0, -1);
  if (argv[0] === '__mark') { cur = Number(argv[1]); ran.set(cur, []); } else ran.get(cur).push(argv);
}
const NAME = /^[A-Za-z_][A-Za-z0-9_]*=/;
const program = (argv) => { let k = 0; while (k < argv.length && NAME.test(argv[k])) k++; return argv.slice(k); };
const bad = [], unsafe = [];
lines.forEach((line, k) => {
  const p = parseShell(line);
  const want = p.cmds.map((c) => program(c.argv)).filter((a) => a.length).map((a) => JSON.stringify(a));
  const got = (ran.get(k) ?? []).map((a) => JSON.stringify(a));
  const ws = p.forks ? [...want].sort() : want, gs = p.forks ? [...got].sort() : got;
  const files = [...new Set(p.cmds.flatMap((c) => c.redirects).filter((f) => !/^\/dev\//.test(f)).map((f) => join(f)))].sort();
  const made = walk(dirOf(k)).filter((f) => !before[k].has(f)).sort();
  if (JSON.stringify(ws) !== JSON.stringify(gs) || JSON.stringify(files) !== JSON.stringify(made)) bad.push({ line, want: ws, got: gs, files, made });
  // unsafe: bash ran a command parseShell does not list (as a multiset), or made a file it does not list
  const left = [...want];
  const extra = got.filter((a) => { const i = left.indexOf(a); if (i < 0) return true; left.splice(i, 1); return false; });
  const extraFiles = made.filter((f) => !files.includes(f));
  // in order (no fork): bash's commands must be parseShell's list with some left out, order kept
  let j = 0; for (const a of gs) { while (j < ws.length && ws[j] !== a) j++; if (j >= ws.length) { j = -1; break; } j++; }
  if (extra.length || extraFiles.length || (!p.forks && j < 0)) unsafe.push({ line, want: ws, got: gs, files, made });
});
console.log(`UNSAFE ${unsafe.length}`); for (const b of unsafe.slice(0, 15)) console.log('UNSAFE', JSON.stringify(b));
console.log(`lines ${lines.length} (seed ${seedS}), commands ${[...ran.values()].reduce((n, g) => n + g.length, 0)}, differ ${bad.length}`);
const odd = bad.filter((b) => !/>&(\d\d+|[3-9])/.test(b.line)); console.log(`differ without a >& to an unopened descriptor: ${odd.length}`); for (const b of odd.slice(0, 10)) console.log(JSON.stringify(b));
rmSync(root, { recursive: true, force: true });
