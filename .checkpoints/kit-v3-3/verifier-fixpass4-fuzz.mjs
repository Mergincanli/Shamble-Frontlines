// Verifier fix pass 4 (R-PROG form; from verifier-pass4-fuzz.mjs): programs from the seven, settings from ROLE_ENV_OK,
// plus non-routine words (aa, bb, cd, x=1) that R-PROG must refuse; extra checks: R-PROG agrees with parseShell on the
// program word, and bash runs no program outside the seven (stubs aa/bb log too; "not found" counted).
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
const PROGS = ['git', 'gh', 'npm', 'node', 'python', 'python3', 'date'];
const OTHER = ['aa', 'bb', 'cd', 'if', 'x=1'];
const WORDS = ['git', 'gh', 'npm', 'node', 'python3', 'date', 'aa', 'bb', 'cd', 'x=1', 'TZ=a:b', 'LC_ALL=C', 'LANG="a b"', 'NO_COLOR=1', "LANGUAGE='x'", '-q', '--a=b', '2', '1', '10', 'a~b', 'a%b', 'a^b', 'u@v', 'a+b', 'a,b', 'a:b', 'a.b', 'd/e', 'k=v=w', '-', '--', 'aa2', 'f1', 'f2',
  "'q r'", "'#x'", "'a\nb'", '"s\'t"', '"<x>"', '"a b"', "''", '""', "'2'", "x='y z'", 'x="1 2"', "a'b'c", 'a"b"c'];
const OPS = [' ', ' ', ' ', '\t', '  ', '>', '>>', '>|', '<', '&>', '&>>', '>&1', '>&2', '2>&1', '1>&2', '>&-', '2>', '2>>', '&', '&&', '|', '|&', ';', '\n', ' > ', ' < ', ' | ', ' ; ', ' && '];
const lines = []; let generated = 0, refused = 0, synErr = 0; const progMismatch = [];
const tries = Number(countS) * 6;
for (let n = 0; n < tries && lines.length < Number(countS); n++) {
  let s = r() < 0.15 ? pick(OTHER) + ' ' : '';
  if (r() < 0.3) s += pick(['TZ=x ', 'LC_ALL=C ', 'NO_COLOR=1 ', 'TZ="a b" ']);
  s += pick(PROGS);
  for (let k = 3 + Math.floor(r() * 12); k > 0; k--) s += r() < 0.5 ? pick(OPS) : (r() < 0.5 ? ' ' : '') + pick(WORDS);
  if (s.includes('||')) continue;
  generated++;
  if (plainForm(s) !== null) { refused++; continue; }
  if (spawnSync('bash', ['--norc', '--noprofile', '-n', '-c', s]).status !== 0) { synErr++; continue; } // bash -n first: the gap count is over lines bash parses
  const p = parseShell(s);
  const NAME = /^[A-Za-z_][A-Za-z0-9_]*=/;
  const progOk = p.cmds.every((c) => { let k = 0; while (k < c.argv.length && NAME.test(c.argv[k])) k++; return k < c.argv.length && PROGS.includes(c.argv[k]) && c.argv.slice(0, k).every((a) => ['TZ', 'LC_ALL', 'LANG', 'LANGUAGE', 'NO_COLOR'].includes(a.split('=')[0])); });
  if (!progOk) { progMismatch.push(s); continue; }
  if ([...p.cmds.flatMap((c) => [...c.redirects, ...c.reads])].some((f) => f.startsWith('/') || f.split('/').includes('..') || f === '' || f === '-')) continue;
  lines.push(s);
}
const root = realpathSync((rmSync(scratch, { recursive: true, force: true }), mkdirSync(scratch, { recursive: true }), scratch));
const bin = join(root, 'bin'), log = join(root, 'log');
mkdirSync(bin);
const stub = `#!/bin/sh\nprintf '%s\\0' "\${0##*/}" "$@" '\x01' >> '${log}'\n`;
for (const n of [...PROGS, 'aa', 'bb', '__mark']) { writeFileSync(join(bin, n), stub); spawnSync('chmod', ['755', join(bin, n)]); }
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
let notFound = 0;
const BASH = spawnSync('bash', ['-c', 'echo $BASH'], { encoding: 'utf8' }).stdout.trim();
for (let s = 0; s < lines.length; s += 300) {
  const script = lines.slice(s, s + 300).map((line, j) => `cd -- '${dirOf(s + j)}' || exit 97\n__mark ${s + j}\n${line}\nwait\n`).join('');
  writeFileSync(join(root, `b${s}.sh`), script);
  const res = spawnSync(BASH, ['--norc', '--noprofile', join(root, `b${s}.sh`)], { cwd: root, env: { PATH: bin, HOME: root, LC_ALL: 'C' }, encoding: 'utf8', maxBuffer: 1 << 26 });
  notFound += (res.stderr.match(/not found/g) ?? []).length;
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
const outside = [...ran.values()].flat().filter((a) => !PROGS.includes(a[0]));
console.log(`generated ${generated}, R-PLAIN/R-PROG refused ${refused}, accepted but bash -n fails ${synErr}, accepted but parseShell program not one of the seven (R-PROG gap) ${progMismatch.length}`); for (const m of progMismatch.slice(0, 10)) console.log('GAP', JSON.stringify(m));
console.log(`bash ran a program outside the seven: ${outside.length}; "not found" in stderr: ${notFound}`);
console.log(`UNSAFE ${unsafe.length}`); for (const b of unsafe.slice(0, 15)) console.log('UNSAFE', JSON.stringify(b));
console.log(`lines ${lines.length} (seed ${seedS}), commands ${[...ran.values()].reduce((n, g) => n + g.length, 0)}, differ ${bad.length}`);
const odd = bad.filter((b) => !/>&(\d\d+|[3-9])/.test(b.line)); console.log(`differ without a >& to an unopened descriptor: ${odd.length}`); for (const b of odd.slice(0, 10)) console.log(JSON.stringify(b));
rmSync(root, { recursive: true, force: true });
