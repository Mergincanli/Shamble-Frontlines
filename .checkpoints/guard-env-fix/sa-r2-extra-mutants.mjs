// security-auditor r2: extra mutants of the round-2 lines against the DAY_CASES, on copies under .gate/guard-env-fix/.
import { readFileSync, writeFileSync, mkdirSync } from 'node:fs';
const R = '/home/user/Shamble-Frontlines/.claude/hooks/', OUT = '/home/user/Shamble-Frontlines/.gate/guard-env-fix/';
mkdirSync(OUT, { recursive: true });
writeFileSync(OUT + 'routine-guard.mjs', readFileSync(R + 'routine-guard.mjs', 'utf8'));
const extra = [
  ['Y1 short attached value off by one', "w.slice(name.length + (name.startsWith('--') ? 1 : 0))", 'w.slice(name.length + 1)'],
  ['Y2 long value keeps its =', "(name.startsWith('--') ? 1 : 0)", '(0)'],
  ['Y3 empty words kept', '.split(/\\s+/).filter(Boolean), ...a]', '.split(/\\s+/), ...a]'],
  ['Y4 words put after the rest', 'a = [...v.split(/\\s+/).filter(Boolean), ...a];', 'a = [...a, ...v.split(/\\s+/).filter(Boolean)];'],
  ['Y5 split on one space only', 'v.split(/\\s+/)', "v.split(' ')"],
  ['Y6 no cut', '(w.length > 32 ?', '(w.length > 1e9 ?'],
  ['Y7 newline not replaced', '/[\\x00-\\x1f\\x7f-\\x9f]/g', '/[\\x00-\\x09\\x0b-\\x1f\\x7f-\\x9f]/g'],
  ['Y8 next-word value lost', "const v = name === w ? a[1] ?? '' :", "const v = name === w ? '' :"],
  ['Y9 tab not replaced', '/[\\x00-\\x1f\\x7f-\\x9f]/g', '/[\\x00-\\x08\\x0a-\\x1f\\x7f-\\x9f]/g'],
];
let t = readFileSync(R + 'routine-guard.test.mjs', 'utf8');
const start = t.indexOf('const DAY_MUTANTS = ['), end = t.indexOf('\n];\n', start);
t = t.slice(0, start) + 'const DAY_MUTANTS = ' + JSON.stringify(extra, null, 1) + t.slice(end + 3);
t = t.replace("assert.deepEqual(survivors, [], `mutants no day case catches", "console.log('SURVIVORS: ' + JSON.stringify(survivors)); assert.deepEqual(survivors, [], `mutants no day case catches");
writeFileSync(OUT + 'routine-guard.test.mjs', t);
