// verifier: apply PLAIN_BEHIND's 'S3 old order' mutant to the head guard; print the bash() diff against 60ae124's bash()
import { readFileSync } from 'node:fs';
const [headF, oldF] = process.argv.slice(2);
const S = readFileSync(headF, 'utf8'), O = readFileSync(oldF, 'utf8');
const at = S.indexOf('    if (role) { // fix pass 4 (P4-S3');
const L = S.slice(at, S.indexOf('      if (rawProg !== prog)', at));
const OLD = ((lines) => [...lines.slice(4), ...lines.slice(1, 3)].join(''))(L.split(/(?<=\n)/));
const M = S.replace(L, () => OLD);
const fn = (s) => s.slice(s.indexOf('async function bash('), s.indexOf('\n}\n', s.indexOf('async function bash(')));
console.log(fn(M) === fn(O) ? 'bash() of the mutant === bash() at 60ae124' : 'DIFFERENT');
