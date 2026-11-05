// kit-v3-3 step 6 mutation rows: each row edits a scratch copy of checkpoint.mjs under .gate/kit-v3-3/step6/ (never the
// real file) and runs the new secret test against it through CHECKPOINT_SCRIPT. Every row must FAIL the test.
//   node mut-step6.mjs <worktree>
import { readFileSync, writeFileSync, mkdirSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
const W = process.argv[2], SRC = `${W}/.claude/scripts/checkpoint.mjs`, OUT = `${W}/.gate/kit-v3-3/step6`;
const src = readFileSync(SRC, 'utf8');
const rows = [
  ['API key without ESC', 'sk-(proj-', (s) => s.replace('[^A-Za-z0-9-]|${ESC})sk-', '[^A-Za-z0-9-]|%3[Dd])sk-')],
  ['AWS key without ESC', 'AKIA', (s) => s.replace('[^A-Za-z0-9_]|${ESC})(AKIA', '[^A-Za-z0-9_])(AKIA')],
  ['Slack without ESC', 'xox', (s) => s.replace('[^A-Za-z0-9_]|${ESC})xox', '[^A-Za-z0-9_])xox')],
  ['no quoted-printable =XX', '=XX', (s) => s.replace('|=[0-9A-F]{2}|', '|')],
  ['no %25 chain', '%25', (s) => s.replace('%(?:25)*(?:', '%(?:')],
  ['no %uXXXX', '%u', (s) => s.replace('(?:[0-9A-Fa-f]{2}|u[0-9A-Fa-f]{4})|=', '[0-9A-Fa-f]{2}|=')],
  ['no backslash escapes', '\\', (s) => s.replace(String.raw`|\\(?:[bfnrtv0]|x[0-9A-Fa-f]{2}|u[0-9A-Fa-f]{4})`, '')],
  ['JSON \\b \\f missing', '\\b\\f', (s) => s.replace('[bfnrtv0]', '[nrtv0]')],
  ['ESC too wide (any letter)', 'wide', (s) => s.replace('const ESC = String.raw`', 'const ESC = String.raw`[A-Za-z]|')],
];
mkdirSync(OUT, { recursive: true });
let bad = 0;
for (const [i, [name, , mut]] of rows.entries()) {
  const m = mut(src);
  if (m === src) { console.log(`row ${i} ${name}: MUTATION DID NOT APPLY`); bad++; continue; }
  const f = `${OUT}/mut-${i}.mjs`; writeFileSync(f, m);
  const r = spawnSync(process.execPath, ['--test', '--test-name-pattern=keys after a %XX', `${W}/.claude/scripts/checkpoint.test.mjs`],
    { encoding: 'utf8', env: { ...process.env, CHECKPOINT_SCRIPT: f } });
  const fail = /# fail [1-9]/.test(r.stdout), pass = (r.stdout.match(/# pass (\d+)/) || [])[1];
  console.log(`row ${i} ${name}: ${fail ? 'test FAILS (killed)' : 'test passes (SURVIVED)'} pass=${pass}`);
  if (!fail) bad++;
}
console.log(bad ? `${bad} row(s) survived or did not apply` : 'all rows killed');
