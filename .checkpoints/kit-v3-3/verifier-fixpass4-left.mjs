// verifier: the C-PLAIN 'left out (R-PROG refuses it)' lines from a suite log: old (60ae124) plainForm, and in PASS4 rows?
import { readFileSync } from 'node:fs';
import { pathToFileURL } from 'node:url';
const [log, oldGuard, newGuard, testFile] = process.argv.slice(2);
const old = await import(pathToFileURL(oldGuard).href), neu = await import(pathToFileURL(newGuard).href);
const t = readFileSync(testFile, 'utf8');
const p4 = t.slice(t.indexOf('export const PASS4_ROUTINE_CASES = ['), t.indexOf('for (const [name, input, want, io, re] of PASS4_ROUTINE_CASES)'));
const rows = [...readFileSync(log, 'utf8').matchAll(/^# left out of the corpus \(R-PROG refuses it\): (".*"): (.*)$/gm)].map((m) => [JSON.parse(m[1].replace(/\\(\\|#)/g, "$1")), m[2]]); // TAP escapes \\ and #
let inP4 = 0, oldAcc = 0, oldAccNotP4 = 0;
for (const [l, why] of rows) {
  const a = old.plainForm(l) === null, inp = p4.includes(JSON.stringify(l).slice(1, -1)) || p4.includes(l.split(' ')[0] === 'undefined' ? '@@' : `'${l}'`);
  if (inp) inP4++; if (a) oldAcc++; if (a && !inp) oldAccNotP4++;
  console.log(`${a ? 'old-accepted' : 'old-refused '} ${inp ? 'P4' : '  '} ${JSON.stringify(l)}: ${why} | new: ${JSON.stringify(neu.plainForm(l))}`);
}
console.log(`total ${rows.length}; old plainForm accepted ${oldAcc}; text in PASS4 rows ${inP4}; old-accepted and not PASS4 ${oldAccNotP4}`);
