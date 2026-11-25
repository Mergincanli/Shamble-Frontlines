// Calls the guard's unwrap() only (no command is run). Prints what the guard takes for the program.
import { unwrap } from '/home/user/Shamble-Frontlines/.claude/hooks/routine-guard.mjs';
const rows = [
  'env -C /d git status', 'env -u X git status', 'env -S x git status', 'env --chdir /d git status', 'env --unset X git status',
  'env --split-string x git status', 'env --chdir=/d git status', 'env -C/d git status', 'env x.y=1 git status',
  'nice --adjustment 5 git status', 'nice --adjustment=5 git status', 'nice --adj=5 git status', 'nice -n 5 git status', 'nice -5 git status',
  'stdbuf -o L git status', 'stdbuf -oL git status', 'stdbuf --output L git status', 'stdbuf --output=L git status',
  'env GIT_TRACE=1 git status', 'nice git status', 'env -i git status',
];
// before the fix unwrap() returned an array; since guard-env-fix it returns { argv, unreadable }
const show = (u) => (Array.isArray(u) ? `program: ${u[0]}` : `program: ${u.argv[0]} · unreadable: ${u.unreadable}`);
for (const r of rows) console.log(`${r.padEnd(36)} -> ${show(unwrap(r.split(' ')))}`);
