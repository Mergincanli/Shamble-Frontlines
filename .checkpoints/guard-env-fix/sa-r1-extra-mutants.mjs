// security-auditor r1: extra mutants of unwrap()/bash() against the DAY_CASES, on copies under .gate/guard-env-fix/.
import { readFileSync, writeFileSync, mkdirSync } from 'node:fs';
const R = '/home/user/Shamble-Frontlines/.claude/hooks/', OUT = '/home/user/Shamble-Frontlines/.gate/guard-env-fix/';
mkdirSync(OUT, { recursive: true });
writeFileSync(OUT + 'routine-guard.mjs', readFileSync(R + 'routine-guard.mjs', 'utf8'));
const extra = [
  ['X1 name is the whole word', "const w = a[0], name = w.startsWith('--') ? w.split('=')[0] : w.slice(0, 2);", 'const w = a[0], name = w;'],
  ['X2 lone - read as an option', "while (a.length && a[0].startsWith('-') && a[0] !== '-') {", "while (a.length && a[0].startsWith('-')) {"],
  ['X3 unread forms take only their word', 'if (o.value.test(name) || o.unread?.test(name)) {', 'if (o.value.test(name)) {'],
  ['X4 a value always takes the next word', 'a = a.slice(name === w ? 2 : 1);', 'a = a.slice(2);'],
  ['X5 the ask returns at once', 'dayAsk ??= ask(`kit and gate files ask first in a day session: the guard does not follow', 'return ask(`kit and gate files ask first in a day session: the guard does not follow'],
  ['X6 lone - for every wrapper', "if (prog === 'env' && a[0] === '-') a = a.slice(1);", "if (a[0] === '-') a = a.slice(1);"],
  ['X7 short names from the whole word for long', "w.startsWith('--') ? w.split('=')[0] : w.slice(0, 2)", 'w.slice(0, 2)'],
  ['X8 env flags lose -0', 'flag: /^(-[i0v]|', 'flag: /^(-[iv]|'],
  ['X9 -- does not break', "if (w === '--') { a = a.slice(1); break; }", "if (w === '--') { a = a.slice(1); continue; }"],
  ['X10 unknown word drops two', "if (!o.flag?.test(w)) unreadable ??= `${prog} ${w}`;\n        a = a.slice(1);", "if (!o.flag?.test(w)) unreadable ??= `${prog} ${w}`;\n        a = a.slice(2);"],
];
let t = readFileSync(R + 'routine-guard.test.mjs', 'utf8');
const start = t.indexOf('const DAY_MUTANTS = ['), end = t.indexOf('\n];\n', start);
t = t.slice(0, start) + 'const DAY_MUTANTS = ' + JSON.stringify(extra, null, 1) + t.slice(end + 3);
t = t.replace("assert.deepEqual(survivors, [], `mutants no day case catches", "console.log('SURVIVORS: ' + JSON.stringify(survivors)); assert.deepEqual(survivors, [], `mutants no day case catches");
writeFileSync(OUT + 'routine-guard.test.mjs', t);
