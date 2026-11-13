// docs-keeper scratch: replace STATUS's "From foundation 2.2" follow-ups bullet with the wrapped text from stdin.
import { readFileSync, writeFileSync } from 'node:fs';
const f = '/home/user/Shamble-Frontlines/docs/STATUS.md';
const lines = readFileSync(f, 'utf8').split('\n');
const s = lines.findIndex((l) => l.startsWith('- **From foundation 2.2 (PR #48), for 2.3:**'));
let e = s; while (!lines[e].endsWith('BRIEF-STALE.')) e++;
if (s < 0 || e - s !== 5) throw new Error(`block ${s}-${e}`);
const add = readFileSync(0, 'utf8').replace(/\n$/, '').split('\n');
lines.splice(s, e - s + 1, ...add);
writeFileSync(f, lines.join('\n'));
console.log(`replaced ${s + 1}-${e + 1} with ${add.length} lines`);
