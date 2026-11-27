// test-engineer scratch: mutants of the build copy's routine-guard.mjs that its review-tracks tests may not catch.
import { cpSync, readFileSync, writeFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
const R = '/home/user/Shamble-Frontlines', B = `${R}/.gate/review-tracks/build/.claude/hooks`, M = `${R}/.gate/te-track-mut/.claude/hooks`;
cpSync(B, M, { recursive: true });
const src = readFileSync(`${B}/routine-guard.mjs`, 'utf8');
const MUT = [
  ["allow read rethrows (a throw refuses instead of the prompt)", "catch { /* silence: the prompt */ }", "catch (e) { throw e; }"],
  ['comments read without --paginate', "io.gh(['api', '--paginate', `repos/${repo}/issues/${n}/comments", "io.gh(['api', `repos/${repo}/issues/${n}/comments"],
  ['jq: m reads created_at (edited PASS accepted)', 'm: .updated_at', 'm: .created_at'],
  ['jq: t reads a constant User', 't: .user.type', 't: "User"'],
  ['latest gate run -> earliest', 'const after = Math.max(...done)', 'const after = Math.min(...done)'],
  ['no-op mutant (sanity: must survive)', "const MARK = 'advisormergecheck:pass';", "const MARK = 'advisormergecheck:pass'; void 0;"],
];
for (const [label, from, to] of MUT) {
  const n = src.split(from).length - 1;
  if (n !== 1) { console.log(`BADTARGET (${n}) ${label}`); continue; }
  writeFileSync(`${M}/routine-guard.mjs`, src.replace(from, to));
  const r = spawnSync(process.execPath, ['--test', '--test-name-pattern=review tracks', `${M}/routine-guard.test.mjs`], { cwd: R, encoding: 'utf8' });
  const pass = /# pass (\d+)/.exec(r.stdout)?.[1], fail = /# fail (\d+)/.exec(r.stdout)?.[1];
  console.log(`${r.status === 0 ? 'SURVIVED' : 'KILLED  '} ${label} (pass ${pass}, fail ${fail})`);
}
writeFileSync(`${M}/routine-guard.mjs`, src);
