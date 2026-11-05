// Fix round 2 mutation rows (builder-fix2): SA-2's routine half, F-b, SA-5 (and the sed/perl targets), SA-6, SA-8.
// Each mutant in a scratch copy under .gate/kit-v3-3/fix2-mut/; the whole routine-guard test file runs on it except
// the in-suite mutation runs (which would only re-check the same rows); a mutant is killed when at least one test
// fails. Prints counts and the first failing test names only. Usage: node mut-fix2.mjs <worktree>
import { spawnSync } from 'node:child_process';
import { mkdirSync, readFileSync, writeFileSync, copyFileSync, rmSync } from 'node:fs';
import { join } from 'node:path';
const WT = process.argv[2];
const src = readFileSync(join(WT, '.claude/hooks/routine-guard.mjs'), 'utf8');
const ROWS = [
  // SA-2, one per site, the comparison, and the two refusals (with their parts)
  ['SA-2 site npmCheck', 'const shadow = originMainCheck(io.projectDir, io, role); // SA-2', 'const shadow = null;'],
  ['SA-2 site interpreter', 'const shadow = originMainCheck(root, io, role);', 'const shadow = null;'],
  ['SA-2 site protectedInBranch', "const shadow = originMainCheck(vcwd, io, 'night-shift'); // SA-2", 'const shadow = null;'],
  ["SA-2 site the PR Advisor's switch", "if (sub === 'switch' && role === 'pr-advisor') { const s = originMainCheck(vcwd, io, role); if (s) return deny(s); }", ''],
  ['SA-2 compared by commit', "return commitOf('origin/main', vcwd, io) === full ? null", 'return true ? null'],
  ['SA-2 the full ref must resolve', "if (full === null) return `routine (${role}): can't resolve", "if (false) return `routine (${role}): can't resolve"],
  ['SA-2 the full ref, not the short name', "const full = commitOf(ORIGIN_MAIN, vcwd, io);\n  if (full === null) return", "const full = commitOf('origin/main', vcwd, io);\n  if (full === null) return"],
  ['SA-2 refusal: fetch/pull refspecs', "if ((sub === 'fetch' || sub === 'pull') && args.some(", 'if (false && args.some('],
  ["SA-2 refusal: ':'", "w.includes(':') || w.startsWith('+')", "w.startsWith('+')"],
  ["SA-2 refusal: '+'", "w.includes(':') || w.startsWith('+')", "w.includes(':')"],
  ['SA-2 refusal: --refmap', ' || /^--refmap(=|$)/.test(w))) {', ')) {'],
  ['SA-2 refusal: new branch names', 'if (bad !== undefined) return deny(`routine (${role}): no new branch named', 'if (false) return deny(`routine (${role}): no new branch named'],
  ['SA-2 names: HEAD and main', "|| n === 'HEAD' || n === 'main');", ');'],
  ['SA-2 names: git branch', "if (sub === 'branch') return mode || !pos.length ? [] : [pos[0]];", "if (sub === 'branch') return [];"],
  ['SA-2 names: switch -c/--orphan', "if (sub === 'switch' && /^(-c|--create|--orphan)$/.test(t)) {", 'if (false) {'],
  ['SA-2 names: worktree add -b/-B', "if (sub === 'worktree' && /^-[bB]$/.test(t)) {", 'if (false) {'],
  ['SA-2 symbolic-ref reads only', "if (sub === 'symbolic-ref' && !(rest.length === 1", "if (false && !(rest.length === 1"],
  ['SA-2 item 4: git remote reads only', "if (sub === 'remote' && !(args.length === 0", "if (false && !(args.length === 0"],
  ['SA-2 item 4: branch --edit-description', "if (sub === 'branch' && args.includes('--edit-description')) return deny(", 'if (false) return deny('],
  // F-b
  ['F-b worktree add', "if (prog === 'git' && a[0] === 'worktree' && a[1] === 'add') {", 'if (false) {'],
  ['F-b worktree add: -b/-B/--reason values', "if (w === '-b' || w === '-B' || w === '--reason') { i++; continue; }", ''],
  ['F-b clone', "if (prog === 'git' && a[0] === 'clone') {", 'if (false) {'],
  ['F-b clone: option values', 'if (CLONE_VALUE.has(w)) { i++; continue; }', ''],
  ["F-b clone: the URL's last part", "const dir = pos.length > 1 ? pos[1] : pos.length ? cloneDir(pos[0]) : '';", "const dir = pos.length > 1 ? pos[1] : '';"],
  // SA-5 and the sed/perl targets
  ['SA-5 prefixes of --in-place', "(w.split('=')[0].length >= 3 && '--in-place'.startsWith(w.split('=')[0]))", '/^--in-place(=|$)/.test(w)'],
  ['sed/perl every positional word', 'return edits ? pos : [];', 'return edits ? pos.slice(1) : [];'],
  // SA-6
  ['SA-6 ancestor of the project folder', 'if (relIn(fold(abs, io), fold(realOf(io.projectDir, io), io)) !== null) return', 'if (false) return'],
  ['SA-6 ls-files kit path', 'return hit ? `${p} (it holds ${hit})` : null;', 'return null;'],
  ['SA-6 git failing asks', 'if (ls.code !== 0) return `${p} (the guard could not list its files)`;', 'if (ls.code !== 0) return null;'],
  // SA-8
  ['SA-8 the line check', 'if (!role && (GIT_PLACE_VARS.test(command) ||', 'if (!role && false && (GIT_PLACE_VARS.test(command) ||'],
  ['SA-8 raw text', 'if (!role && (GIT_PLACE_VARS.test(command) ||', 'if (!role && (false ||'],
  ['SA-8 words', 'cmds.some((c) => [...c.argv, ...c.redirects, ...c.reads].some((w) => GIT_PLACE_VARS.test(w))))) {', 'false)) {'],
  ['SA-8 the four names', 'const GIT_PLACE_VARS = /GIT_(DIR|WORK_TREE|COMMON_DIR|INDEX_FILE)/;', 'const GIT_PLACE_VARS = /GIT_(DIR|WORK_TREE)=/;'],
];
const base = join(WT, '.gate/kit-v3-3/fix2-mut');
rmSync(base, { recursive: true, force: true });
const runIn = (d) => {
  const r = spawnSync('node', ['--test', '--test-skip-pattern=mutation', 'routine-guard.test.mjs'], { cwd: d, encoding: 'utf8', timeout: 300000 });
  const fails = (r.stdout.match(/^not ok \d+ - .*$/gm) ?? []).map((l) => l.replace(/^not ok \d+ - /, ''));
  const pass = (r.stdout.match(/^# pass (\d+)/m) ?? [])[1];
  return { fails, pass };
};
const setup = (d, text) => {
  mkdirSync(d, { recursive: true });
  writeFileSync(join(d, 'routine-guard.mjs'), text);
  copyFileSync(join(WT, '.claude/hooks/routine-guard.test.mjs'), join(d, 'routine-guard.test.mjs'));
  copyFileSync(join(WT, '.claude/hooks/guard.mjs'), join(d, 'guard.mjs'));
};
let killed = 0;
for (const [i, [label, from, to]] of ROWS.entries()) {
  const n = src.split(from).length - 1;
  if (n !== 1) { console.log(`ROW ${i} ${label}: target occurs ${n} times (BAD)`); continue; }
  const d = join(base, `m${i}`);
  setup(d, src.replace(from, () => to));
  const { fails, pass } = runIn(d);
  if (fails.length) killed++;
  console.log(`${fails.length ? 'KILLED' : 'SURVIVED'} ${label}: ${fails.length} fail, ${pass} pass; e.g. ${fails.slice(0, 2).join(' | ')}`);
}
console.log(`killed ${killed}/${ROWS.length}`);
const c = join(base, 'control');
setup(c, src);
const ctl = runIn(c);
console.log(`control: ${ctl.pass} pass, ${ctl.fails.length} fail`);
