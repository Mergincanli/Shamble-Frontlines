// Verifier scratch (guard-env-fix): re-runs security-auditor's extra mutants (X r1, Y r2) against a snapshot's DAY_CASES.
// decide() with the test file's fake io only; no shell command. Usage: node verifier-extra.mjs <snapshotDir>
// (<snapshotDir> holds guard.mjs, routine-guard.mjs and routine-guard.test.mjs; copies are written inside it.)
import { readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';
const dir = process.argv[2];
const src = readFileSync(join(dir, 'routine-guard.mjs'), 'utf8');
let t = readFileSync(join(dir, 'routine-guard.test.mjs'), 'utf8');
t = t.replace("import { test } from 'node:test';", 'const test = () => {};') + '\nexport { dayIo as __dayIo, kindOf as __kindOf };\n';
writeFileSync(join(dir, 'vx-test.mjs'), t);
const { DAY_CASES, __dayIo: dayIo, __kindOf: kindOf } = await import(pathToFileURL(join(dir, 'vx-test.mjs')).href);
const extra = [
  ['X1', "const w = a[0], name = w.startsWith('--') ? w.split('=')[0] : w.slice(0, 2);", 'const w = a[0], name = w;'],
  ['X2', "while (a.length && a[0].startsWith('-') && a[0] !== '-') {", "while (a.length && a[0].startsWith('-')) {"],
  ['X3', 'if (o.value.test(name) || o.unread?.test(name)) {', 'if (o.value.test(name)) {'],
  ['X4', 'a = a.slice(name === w ? 2 : 1);', 'a = a.slice(2);'],
  ['X5', 'dayAsk ??= ask(`kit and gate files ask first in a day session: the guard does not follow', 'return ask(`kit and gate files ask first in a day session: the guard does not follow'],
  ['X6', "if (prog === 'env' && a[0] === '-') a = a.slice(1);", "if (a[0] === '-') a = a.slice(1);"],
  ['X7', "w.startsWith('--') ? w.split('=')[0] : w.slice(0, 2)", 'w.slice(0, 2)'],
  ['X8', 'flag: /^(-[i0v]|', 'flag: /^(-[iv]|'],
  ['X9', "if (w === '--') { a = a.slice(1); break; }", "if (w === '--') { a = a.slice(1); continue; }"],
  ['X10 (21615cb text)', "if (!o.flag?.test(w)) unreadable ??= `${prog} ${w}`;\n        a = a.slice(1);", "if (!o.flag?.test(w)) unreadable ??= `${prog} ${w}`;\n        a = a.slice(2);"],
  ['X10 (shownWord text)', "if (!o.flag?.test(w)) unreadable ??= `${prog} ${shownWord(w)}`;\n        a = a.slice(1);", "if (!o.flag?.test(w)) unreadable ??= `${prog} ${shownWord(w)}`;\n        a = a.slice(2);"],
  ['Y1', "w.slice(name.length + (name.startsWith('--') ? 1 : 0))", 'w.slice(name.length + 1)'],
  ['Y2', "(name.startsWith('--') ? 1 : 0)", '(0)'],
  ['Y3', '.split(/\\s+/).filter(Boolean), ...a]', '.split(/\\s+/), ...a]'],
  ['Y4', 'a = [...v.split(/\\s+/).filter(Boolean), ...a];', 'a = [...a, ...v.split(/\\s+/).filter(Boolean)];'],
  ['Y5', 'v.split(/\\s+/)', "v.split(' ')"],
  ['Y6 (reason)', '(w.length > 32 ?', '(w.length > 1e9 ?'],
  ['Y7 (reason)', '/[\\x00-\\x1f\\x7f-\\x9f]/g', '/[\\x00-\\x09\\x0b-\\x1f\\x7f-\\x9f]/g'],
  ['Y8', "const v = name === w ? a[1] ?? '' :", "const v = name === w ? '' :"],
  ['Y9 (reason)', '/[\\x00-\\x1f\\x7f-\\x9f]/g', '/[\\x00-\\x08\\x0a-\\x1f\\x7f-\\x9f]/g'],
];
let i = 0;
for (const [label, from, to] of extra) {
  const n = src.split(from).length - 1;
  if (n !== 1) { console.log(`${label}: target occurs ${n} times: not applicable`); continue; }
  const f = join(dir, `vx${i++}.mjs`);
  writeFileSync(f, src.replace(from, to));
  const m = await import(pathToFileURL(f).href);
  let kindFail = 0, reasonFail = 0;
  for (const [, input, want, opts, re] of DAY_CASES) {
    let d; try { d = await m.decide(input, dayIo(opts)); } catch { d = 'threw'; }
    const k = d === 'threw' ? 'threw' : kindOf(d);
    if (k !== want) kindFail++;
    else if (re && d && !re.test(d.ask ?? d.deny ?? '')) reasonFail++;
  }
  console.log(`${label}: kind-caught by ${kindFail} cases; reason-caught by ${reasonFail} more`);
}
