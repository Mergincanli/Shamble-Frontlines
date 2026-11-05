// Fix round R-a..R-e mutation rows (builder-fix1): each mutant in a scratch copy under .gate/kit-v3-3/fix1-mut/,
// the day cases and the real-folder test run on it; a mutant is killed when at least one fails. Prints counts and
// the first failing case names only. Usage: node mut-fix1.mjs <worktree>
import { spawnSync } from 'node:child_process';
import { mkdirSync, readFileSync, writeFileSync, copyFileSync, rmSync } from 'node:fs';
import { join } from 'node:path';
const WT = process.argv[2];
const src = readFileSync(join(WT, '.claude/hooks/routine-guard.mjs'), 'utf8');
const ROWS = [
  ['R-a relative write while unknown asks', 'if (vcwd === null && !isAbsolute(t)) {', 'if (false) {'],
  ['R-a unknown state reaches the write check', 'const w = dayShellWrites(c, argv, dirs.known ? vcwd : null, io, input);', 'const w = dayShellWrites(c, argv, vcwd, io, input);'],
  ['R-a main-folder check reads unknown as project folder', 'if (!dirs.known) vcwd = io.projectDir;', ''],
  ['R-a agent refused', 'if (agent) return deny(`${KIT_AGENT}; after a cd', 'if (false) return deny(`${KIT_AGENT}; after a cd'],
  ['R-b only absolute, ./, ../ targets', '(isAbsolute(t) || /^\\.\\.?(\\/|$)/.test(t))', 'true'],
  ['R-b no variable or glob', "&& !/[$`*?[{]/.test(t);", ';'],
  ['R-b one target', 'const fixed = a.length === 1 &&', 'const fixed = a.length >= 1 &&'],
  ['R-b folder exists and can be entered', '!(io.isDir ? io.isDir(to) : io.exists(to))', 'false'],
  ['R-b no option but -L', "if (prog === 'cd') while (/^-L+$/.test(a[0] ?? '')) a = a.slice(1);", "if (prog === 'cd') while (/^-[LPe@]+$/.test(a[0] ?? '')) a = a.slice(1);"],
  ['R-b relative target needs a known folder', '(here !== null || isAbsolute(t))', 'true'],
  ['R-b old behaviour: bare names followed (whole rule)', "const fixed = a.length === 1 && (isAbsolute(t) || /^\\.\\.?(\\/|$)/.test(t)) && !/[$`*?[{]/.test(t);", 'const fixed = a.length === 1 && !/[$`*?[{]/.test(t) && !/^[-~]/.test(t);'],
  ['R-c leading run', "if (!dirs.lead || forked || c.sep === '||' ||", "if (forked || c.sep === '||' ||"],
  ['R-c cd after ||', "forked || c.sep === '||' ||", 'forked ||'],
  ['R-c || separator recorded', "endCmd('||')", 'endCmd()'],
  ['R-c bare word, not a path', 'argv[0] !== prog || !prefix', '!prefix'],
  ['R-c only listed words reach a cd', '!prefix.every((w) => CD_REACH.has(w))', 'false'],
  ['R-c command -v does not run the cd', "'command', '-p', '--']);", "'command', '-p', '-v', '--']);"],
  ['R-c loop keyword does not reach a cd', "new Set(['if', '{',", "new Set(['while', 'if', '{',"],
  ["R-c unwrap strips command's options", "while (/^-[pvV]+$|^--$/.test(a[0] ?? '')) a = a.slice(1);", ''],
  ['R-c bare keyword runs nothing', 'prefix.every((w) => WRAPPERS.has(w))) return vcwd;', 'false) return vcwd;'],
  ['R-c pushd to unknown target: stack unknown', "if (prog === 'pushd') dirs.stack = null;", ''],
  ['R-c popd with an empty stack', 'if (a.length || !dirs.stack?.length) {', 'if (a.length || !dirs.stack) {'],
  ['R-c popd to an unknown entry', 'if (top === null) return unknown();', 'if (top === null) return vcwd;'],
  ['R-d forked line: every cd unknown', 'if (!dirs.lead || forked ||', 'if (!dirs.lead ||'],
  ['R-d forks flag', "(c === '&' && s[i + 1] !== '>')) forks = true;", "(c === '&' && s[i + 1] !== '>')) forks = false;"],
  ['R-e exec', "if (prefix.includes('exec') ||", 'if ('],
  ['R-e source, . and eval', "['source', '.', 'eval'].includes(prog) ||", ''],
  ['R-e program word starting with - or holding $', "|| /^-|[$`]/.test(argv[0] ?? '')) return unknown();", ') return unknown();'],
];
const base = join(WT, '.gate/kit-v3-3/fix1-mut');
rmSync(base, { recursive: true, force: true });
let killed = 0;
for (const [i, [label, from, to]] of ROWS.entries()) {
  const n = src.split(from).length - 1;
  if (n !== 1) { console.log(`ROW ${i} ${label}: target occurs ${n} times (BAD)`); continue; }
  const d = join(base, `m${i}`);
  mkdirSync(d, { recursive: true });
  writeFileSync(join(d, 'routine-guard.mjs'), src.replace(from, () => to));
  copyFileSync(join(WT, '.claude/hooks/routine-guard.test.mjs'), join(d, 'routine-guard.test.mjs'));
  copyFileSync(join(WT, '.claude/hooks/guard.mjs'), join(d, 'guard.mjs'));
  const r = spawnSync('node', ['--test', '--test-name-pattern=^(day \\(kit-v3-3\\)|real folders)', 'routine-guard.test.mjs'], { cwd: d, encoding: 'utf8', timeout: 300000 });
  const fails = (r.stdout.match(/^not ok \d+ - .*$/gm) ?? []).map((l) => l.replace(/^not ok \d+ - /, ''));
  const pass = (r.stdout.match(/^# pass (\d+)/m) ?? [])[1];
  if (fails.length) killed++;
  console.log(`${fails.length ? 'KILLED' : 'SURVIVED'} ${label}: ${fails.length} fail, ${pass} pass; e.g. ${fails.slice(0, 2).join(' | ')}`);
}
console.log(`killed ${killed}/${ROWS.length}`);
// control: the unmutated copy
const c = join(base, 'control');
mkdirSync(c, { recursive: true });
writeFileSync(join(c, 'routine-guard.mjs'), src);
copyFileSync(join(WT, '.claude/hooks/routine-guard.test.mjs'), join(c, 'routine-guard.test.mjs'));
copyFileSync(join(WT, '.claude/hooks/guard.mjs'), join(c, 'guard.mjs'));
const rc = spawnSync('node', ['--test', '--test-name-pattern=^(day \\(kit-v3-3\\)|real folders)', 'routine-guard.test.mjs'], { cwd: c, encoding: 'utf8' });
console.log(`control: ${(rc.stdout.match(/^# pass (\d+)/m) ?? [])[1]} pass, ${(rc.stdout.match(/^# fail (\d+)/m) ?? [])[1]} fail`);
