// security-auditor r1: calls unwrap() only (no command is run); prints program and unreadable per form.
import { unwrap } from '/home/user/Shamble-Frontlines/.claude/hooks/routine-guard.mjs';
const rows = process.argv.slice(2).length ? process.argv.slice(2) : [
  'env -iu X git s', 'env -u -i git s', 'env -u -- git s', 'env --unset -- git s', 'env -- -i git s', 'env - - git s',
  'env -i -- X=1 git s', 'env X=1 -- git s', 'env -0 -v --debug git s', 'env -C', 'env -u', 'env',
  'nice -n -5 git s', 'nice --adjustment -5 git s', 'nice -+5 git s', 'nice -5x git s', 'nice -n5 git s', 'nice -n 5 -10 git s',
  'nice -- -5 git s', 'nice -- git s', 'stdbuf -o -- git s', 'stdbuf -i0 -o0 -e0 git s', 'stdbuf -- git s',
  'env nice stdbuf git s', 'env -i nice -n 1 git s', 'nice env -C /d git s', 'env -u X env -C /d git s',
  'env --unset=X=1 git s', 'env -uX=1 git s', 'env =x git s', 'env -i X git s', 'env x=1 y git s',
  'env --=x git s', 'env ---x git s', 'nice --adjustment', 'stdbuf --output', 'env -P /x git s', 'env -L0 git s',
];
for (const r of rows) { const u = unwrap(r.split(' ')); console.log(`${r.padEnd(32)} -> program: ${JSON.stringify(u.argv[0] ?? null)} · unreadable: ${u.unreadable}`); }
